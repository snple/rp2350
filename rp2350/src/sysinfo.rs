//! SYSINFO (System Information) HAL for RP2350
//!
//! This module provides read-only access to chip identification and platform
//! information for the RP2350.
//!
//! ## Features
//!
//! - Chip ID (JEDEC JEP-106 compliant)
//! - Package selection
//! - Platform identification (ASIC, FPGA, simulation)
//! - Chip git reference
//!
//! ## Example
//!
//! ```ignore
//! use rp2350::sysinfo::{Platform, Sysinfo};
//! use rp2350_pac::SYSINFO;
//!
//! let sysinfo_pac = unsafe { SYSINFO::steal() };
//! let sysinfo = Sysinfo::init(sysinfo_pac);
//!
//! // Get chip ID information
//! let chip_id = sysinfo.chip_id();
//! let part = sysinfo.part();
//! let revision = sysinfo.revision();
//!
//! // Check platform
//! if sysinfo.platform() == Some(Platform::Asic) {
//!     // Running on real hardware
//! }
//! ```

use crate::Steal;

/// The package variant of this chip.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Package {
    /// RP2350A (QFN-60)
    Rp2350a,
    /// RP2350B (QFN-80)
    Rp2350b,
}

/// The environment the code is running on.
///
/// Post-production chips always report [`Platform::Asic`]; the simulation
/// variants are only meaningful during pre-production development.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Platform {
    /// Real silicon.
    Asic,
    /// FPGA build.
    Fpga,
    /// HDL (RTL) simulation.
    HdlSim,
    /// Batch simulation.
    BatchSim,
    /// Gate-level simulation.
    GateSim,
}

/// SYSINFO peripheral wrapper
///
/// Provides read-only access to system information registers.
/// Owns the PAC SYSINFO peripheral to enforce proper ownership semantics.
pub struct Sysinfo {
    sysinfo: rp2350_pac::SYSINFO,
}

impl Sysinfo {
    /// Take ownership of the SYSINFO peripheral.
    ///
    /// SYSINFO is a read-only peripheral: its registers are hard-wired and
    /// there is no meaningful reset to perform, so this only transfers
    /// ownership of the [`rp2350_pac::SYSINFO`] register block.
    ///
    /// # Arguments
    /// * `sysinfo` - The SYSINFO peripheral from PAC
    #[inline]
    pub fn init(sysinfo: rp2350_pac::SYSINFO) -> Self {
        Self { sysinfo }
    }

    /// Obtain a handle to the SYSINFO peripheral without initialization.
    ///
    /// # Safety
    ///
    /// The caller must ensure the SYSINFO peripheral has already been
    /// initialized (e.g. via [`Sysinfo::init`]) before use.
    #[inline]
    pub unsafe fn steal() -> Steal<Self> {
        Steal(Self {
            sysinfo: unsafe { rp2350_pac::SYSINFO::steal() },
        })
    }

    /// Get reference to the underlying register block
    ///
    /// # Safety
    /// Direct register access can bypass the HAL's safety guarantees.
    /// The caller must ensure operations do not conflict with the HAL state.
    #[inline]
    pub unsafe fn regs(&self) -> &rp2350_pac::sysinfo::RegisterBlock {
        &self.sysinfo
    }

    /// Release the SYSINFO peripheral
    #[inline]
    pub fn release(self) -> rp2350_pac::SYSINFO {
        self.sysinfo
    }

    // ========================================================================
    // Chip ID
    // ========================================================================

    /// Get the raw chip ID register value
    #[inline]
    pub fn chip_id(&self) -> u32 {
        self.sysinfo.chip_id.read_bits()
    }

    /// Get the chip revision (bits 31:28)
    #[inline]
    pub fn revision(&self) -> u8 {
        self.sysinfo.chip_id.revision() as u8
    }

    /// Get the chip part number (bits 27:12)
    #[inline]
    pub fn part(&self) -> u16 {
        self.sysinfo.chip_id.part() as u16
    }

    /// Get the manufacturer ID (bits 11:1)
    #[inline]
    pub fn manufacturer(&self) -> u16 {
        self.sysinfo.chip_id.manufacturer() as u16
    }

    // ========================================================================
    // Package Selection
    // ========================================================================

    /// Get the package variant of this chip.
    ///
    /// `PACKAGE_SEL[0]` is `0` for RP2350A (QFN-60) and `1` for
    /// RP2350B (QFN-80).
    #[inline]
    pub fn package(&self) -> Package {
        if self.sysinfo.package_sel.package_sel() {
            Package::Rp2350b
        } else {
            Package::Rp2350a
        }
    }

    // ========================================================================
    // Platform
    // ========================================================================

    /// Get the environment this code is running on.
    ///
    /// Returns `None` if the `PLATFORM` register holds no known value (all
    /// five platform bits clear); use [`Sysinfo::platform_raw`] to inspect
    /// the raw bits in that case.
    #[inline]
    pub fn platform(&self) -> Option<Platform> {
        let p = &self.sysinfo.platform;
        if p.asic() {
            Some(Platform::Asic)
        } else if p.fpga() {
            Some(Platform::Fpga)
        } else if p.hdlsim() {
            Some(Platform::HdlSim)
        } else if p.batchsim() {
            Some(Platform::BatchSim)
        } else if p.gatesim() {
            Some(Platform::GateSim)
        } else {
            None
        }
    }

    /// Get the raw platform register value.
    #[inline]
    pub fn platform_raw(&self) -> u32 {
        self.sysinfo.platform.read_bits()
    }

    // ========================================================================
    // Git Reference
    // ========================================================================

    /// Get the chip source git hash
    ///
    /// This is used to identify the chip version.
    #[inline]
    pub fn gitref(&self) -> u32 {
        self.sysinfo.gitref_rp2350.read_bits()
    }
}
