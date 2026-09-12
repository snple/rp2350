//! Flash read, erase and program through the bootrom
//!
//! The bootrom's `flash_op` call (tag `FO`) is a checked flash operation: it
//! validates the range against the flash devices described by `FLASH_DEVINFO`
//! and, when a resident partition table is in effect, against the partition
//! permissions and the caller's security level. It uses the fastest erase
//! command the attached devices allow, and leaves the QSPI device and QMI in a
//! usable state when it returns.
//!
//! This module is the high-level wrapper; the raw call is
//! [`crate::bootrom::flash_op`]. [`crate::qspi`] is the other end of the
//! spectrum, driving QMI direct-mode commands yourself, and [`crate::xip`]
//! controls the cache on top of it.
//!
//! ## Example
//!
//! ```ignore
//! use rp2350::flash;
//!
//! const SLOT: u32 = 0x1000_0000 + 0x20_0000;
//!
//! flash::erase(SLOT, 4096)?;
//! flash::program(SLOT, &image[..256])?;
//! ```
//!
//! ## Notes
//!
//! QMI is in direct mode for the duration of the call, so neither core may
//! fetch instructions or data from flash while it runs: an XIP access faults.
//! Run the calling code from RAM (see `#[ramfunc]`) and make sure the other
//! core is parked or also running from RAM. This module does no cross-core
//! coordination of its own.
//!
//! Addresses are always XIP-window addresses. What [`Options`] changes is
//! whether QMI translates them through `ATRANS0..7` first, and which security
//! level the partition check uses. Geometry is the bootrom's business: an erase
//! that is not 4 KiB aligned comes back as [`Error::BadAlignment`] instead of
//! damaging anything, and an access the partition table forbids as
//! [`Error::NotPermitted`].

use crate::bootrom::{self, BootRomError};

/// Errors from a flash operation: the bootrom's own codes.
///
/// [`BootRomError::BadAlignment`] and [`BootRomError::InvalidAddress`] cover
/// bad geometry, [`BootRomError::NotPermitted`] a blocked access, and
/// [`BootRomError::InvalidState`] a request that cannot be serviced right now.
pub type Error = BootRomError;

/// The effective security level a flash operation is checked against.
///
/// Only the Secure entry takes this into account; the Non-secure entry this
/// HAL falls back to when running Non-secure is always checked against
/// Non-secure permissions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecurityLevel {
    /// The Secure world.
    Secure,
    /// The Non-secure world.
    NonSecure,
    /// A bootloader acting on behalf of the boot process.
    Bootloader,
}

impl SecurityLevel {
    const fn bits(self) -> u32 {
        match self {
            Self::Secure => 1,
            Self::NonSecure => 2,
            Self::Bootloader => 3,
        }
    }
}

/// Which of the two flash address spaces an operation works in.
///
/// Both use XIP-window addresses; the difference is whether QMI translates
/// them through `ATRANS0..7` first.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
enum AddressSpace {
    /// Physical flash addresses, identity mapped.
    Storage,
    /// Runtime addresses, translated by the QMI address translation registers.
    Runtime,
}

impl AddressSpace {
    const fn bits(self) -> u32 {
        match self {
            Self::Storage => 0,
            Self::Runtime => 1,
        }
    }
}

/// What an operation does with the addressed range.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
enum Op {
    /// Erase whole 4 KiB sectors.
    Erase,
    /// Program whole 256-byte pages.
    Program,
    /// Read an arbitrary range.
    Read,
}

impl Op {
    const fn bits(self) -> u32 {
        match self {
            Self::Erase => 0,
            Self::Program => 1,
            Self::Read => 2,
        }
    }
}

// `CFLASH` flag word layout (RP2350 datasheet §5.5.8.2).
const ASPACE_LSB: u32 = 0;
const SECLEVEL_LSB: u32 = 8;
const OP_LSB: u32 = 16;

/// Assemble the `CFLASH` flag word for the bootrom.
const fn cflash(op: Op, space: AddressSpace, level: SecurityLevel) -> u32 {
    (op.bits() << OP_LSB) | (space.bits() << ASPACE_LSB) | (level.bits() << SECLEVEL_LSB)
}

/// A flash operation with the address space and security level under the
/// caller's control.
///
/// [`erase`], [`program`] and [`read`] are the same three operations at the
/// defaults: runtime addresses at the Secure level. Reach for this type when a
/// bootloader has to write through the address translation windows, or when a
/// partition table makes the effective security level matter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Options {
    space: AddressSpace,
    level: SecurityLevel,
}

impl Options {
    /// Runtime addresses at the Secure level.
    pub const fn new() -> Self {
        Self {
            space: AddressSpace::Runtime,
            level: SecurityLevel::Secure,
        }
    }

    /// Treat addresses as storage addresses instead of runtime ones.
    pub const fn storage(self) -> Self {
        Self {
            space: AddressSpace::Storage,
            level: self.level,
        }
    }

    /// Check the access against a different security level.
    pub const fn security_level(self, level: SecurityLevel) -> Self {
        Self {
            space: self.space,
            level,
        }
    }

    /// Erase `len` bytes starting at `addr`.
    ///
    /// `addr` must be 4 KiB aligned and `len` a multiple of 4 KiB.
    pub fn erase(self, addr: u32, len: u32) -> Result<(), Error> {
        self.run(Op::Erase, addr, len, core::ptr::null_mut())
    }

    /// Program `data` at `addr`.
    ///
    /// `addr` must be 256-byte aligned and `data.len()` a multiple of 256.
    /// Flash bits only go from 1 to 0, so erase the range first.
    pub fn program(self, addr: u32, data: &[u8]) -> Result<(), Error> {
        // The bootrom only reads the buffer for a program operation.
        self.run(
            Op::Program,
            addr,
            data.len() as u32,
            data.as_ptr().cast_mut(),
        )
    }

    /// Read `data.len()` bytes starting at `addr`.
    pub fn read(self, addr: u32, data: &mut [u8]) -> Result<(), Error> {
        self.run(Op::Read, addr, data.len() as u32, data.as_mut_ptr())
    }

    fn run(self, op: Op, addr: u32, len: u32, buffer: *mut u8) -> Result<(), Error> {
        let flags = cflash(op, self.space, self.level);

        // SAFETY: `erase` passes a null buffer because the bootrom does not
        // touch it; `program` and `read` pass a slice pointer together with its
        // length, and the bootrom accesses it only in the direction the
        // operation implies.
        let rc = unsafe { bootrom::flash_op(flags, addr, len, buffer) };
        if rc < 0 {
            return Err(BootRomError::from(rc));
        }

        Ok(())
    }
}

impl Default for Options {
    fn default() -> Self {
        Self::new()
    }
}

/// Erase `len` bytes of flash starting at the runtime address `addr`.
///
/// `addr` must be 4 KiB aligned and `len` a multiple of 4 KiB. See [`Options`]
/// for storage addresses and other security levels.
pub fn erase(addr: u32, len: u32) -> Result<(), Error> {
    Options::new().erase(addr, len)
}

/// Program `data` at the runtime address `addr`.
///
/// `addr` must be 256-byte aligned and `data.len()` a multiple of 256. Flash
/// bits only go from 1 to 0, so erase the range first.
pub fn program(addr: u32, data: &[u8]) -> Result<(), Error> {
    Options::new().program(addr, data)
}

/// Read `data.len()` bytes from the runtime address `addr`.
///
/// Unlike erase and program there is no alignment or size restriction.
pub fn read(addr: u32, data: &mut [u8]) -> Result<(), Error> {
    Options::new().read(addr, data)
}
