//! RP2350 bootrom (mask ROM) API access
//!
//! The bootrom publishes a table of callable functions that stay resident for
//! the lifetime of the chip. This module resolves entries in that table and
//! exposes the ones the HAL needs today; [`reboot`] is the watchdog-based reset
//! that backs [`crate::reboot`].
//!
//! ## Features
//!
//! - [`rom_version_number`] — version byte of the resident bootrom
//! - [`rom_table_lookup`] — resolve a two-character tag (`b"RB"` = reboot,
//!   `b"GS"` = `get_sys_info`, …) to a function address
//! - [`rom_data_lookup`] — resolve a data tag (`b"GR"` = git revision,
//!   `b"PT"` = resident partition table) to a data address
//! - [`get_sys_info`] — raw access to the bootrom's `get_sys_info` call
//! - [`flash_op`] — raw access to the bootrom's checked flash read, erase and
//!   program
//! - [`reboot`] — reset into the normal, BOOTSEL, RAM image, flash update or
//!   PC/SP boot path
//!
//! The typed half lives in sibling modules: [`crate::reboot`] wraps
//! [`reboot`], [`crate::sysinfo_rom`] decodes [`get_sys_info`], and
//! [`crate::flash`] wraps [`flash_op`].
//!
//! ## Example
//!
//! ```ignore
//! // ROM-level: reboot into BOOTSEL 500 ms from now, both USB interfaces
//! // enabled. `crate::reboot` is the typed, safe wrapper around this.
//! let rc = unsafe { rp2350::bootrom::reboot(0x0002, 500, 0, 0) };
//! assert!(rc >= 0, "bootrom refused to reboot: {}", rc);
//! ```
//!
//! ## Notes
//!
//! - The well-known addresses below are bootrom entry points, not peripheral
//!   registers, so they cannot come from the PAC (RP2350 datasheet §5.5.1).
//! - Bootrom calls that report an `i32` use the negative error codes of
//!   RP2350 datasheet §5.4.3; [`BootRomError`] names the ones this HAL knows.
//! - [`flash_op`] puts QMI into direct mode while it runs, so neither core may
//!   fetch instructions or data from XIP during the call; [`crate::flash`]
//!   spells out what that means in practice.
//! - Secure and Non-secure ARM callers see separate function tables. The lookup
//!   call itself works in both security states, so [`reboot`] and
//!   [`get_sys_info`] ask for the Secure entry first and fall back to the
//!   Non-secure one instead of having to determine the current state via `TT`.
//! - `reset_usb_boot` (`b"UB"`) is RP2040-only. On RP2350 the same effect comes
//!   from [`reboot`] with boot type `BOOTSEL` (`0x2`), which is what
//!   [`crate::reboot::RebootKind::BootSel`] uses.
//! - This module targets the ARM (Cortex-M33) bootrom only: the HAL does not
//!   support the RISC-V cores, so their well-known addresses and function flags
//!   are not implemented.

/// A bootrom table tag, e.g. `b"RB"` for the reboot function.
pub type RomFnTableCode = [u8; 2];

/// Flags selecting an entry type in [`rom_table_lookup`] / [`rom_data_lookup`]
/// (RP2350 datasheet §5.5.1).
pub mod rt_flags {
    /// Secure ARM function entry.
    pub const FUNC_ARM_SEC: u32 = 0x0004;
    /// Non-secure ARM function entry.
    pub const FUNC_ARM_NONSEC: u32 = 0x0010;
    /// Data table entry, for use with [`super::rom_data_lookup`].
    pub const DATA: u32 = 0x0040;
}

/// Type of the bootrom's table lookup function.
type RomTableLookupFn = unsafe extern "C" fn(code: u32, mask: u32) -> usize;

/// Type of the bootrom's `reboot` function.
type RomRebootFn = unsafe extern "C" fn(flags: u32, delay_ms: u32, p0: u32, p1: u32) -> i32;

/// Type of the bootrom's `get_sys_info` function.
type RomGetSysInfoFn = unsafe extern "C" fn(out: *mut u32, out_words: usize, flags: u32) -> i32;

/// Type of the bootrom's `flash_op` function.
type RomFlashOpFn =
    unsafe extern "C" fn(flags: u32, addr: u32, size_bytes: u32, buffer: *mut u8) -> i32;

/// Address of the bootrom version byte.
const ROM_VERSION_NUMBER: *const u8 = 0x0000_0013 as _;

/// Well-known pointer to the table lookup function, ROM version 2 (16-bit value).
const ROM_TABLE_LOOKUP_V2: *const u16 = 0x0000_0016 as _;

/// Well-known pointer to the table lookup function, ROM version 1 (32-bit value).
const ROM_TABLE_LOOKUP_V1: *const u32 = 0x0000_0018 as _;

/// Tag of the bootrom `reboot` function.
const TAG_REBOOT: RomFnTableCode = *b"RB";

/// Tag of the bootrom `get_sys_info` function.
const TAG_GET_SYS_INFO: RomFnTableCode = *b"GS";

/// Tag of the bootrom `flash_op` function.
const TAG_FLASH_OP: RomFnTableCode = *b"FO";

/// Tag of the bootrom git revision data entry.
const TAG_GIT_REVISION: RomFnTableCode = *b"GR";

/// Tag of the resident partition table data entry.
const TAG_PARTITION_TABLE: RomFnTableCode = *b"PT";

/// Version number of the resident bootrom (the byte at `0x13`).
#[inline]
pub fn rom_version_number() -> u8 {
    // SAFETY: 0x13 is a bootrom constant, readable in every security state.
    unsafe { ROM_VERSION_NUMBER.read() }
}

/// Resolve a bootrom function table entry by tag.
///
/// `tag` is a two-character code (`b"RB"` reboot, `b"GS"` `get_sys_info`, …)
/// and `mask` is one of [`rt_flags`]. Returns `0` when the tag is not present
/// in the table.
pub fn rom_table_lookup(tag: RomFnTableCode, mask: u32) -> usize {
    let code = u16::from_le_bytes(tag) as u32;

    // SAFETY: `table_lookup_ptr` yields the bootrom's own lookup function,
    // which takes plain values and is callable from any security state.
    let lookup: RomTableLookupFn = unsafe { core::mem::transmute(table_lookup_ptr()) };

    // SAFETY: `lookup` is the bootrom table lookup function.
    unsafe { lookup(code, mask) }
}

/// Resolve a bootrom data table entry by tag.
///
/// The data table holds pointers rather than code, so pass [`rt_flags::DATA`]
/// as `mask`: `b"GR"` is the bootrom git revision, `b"PT"` the resident
/// partition table, and so on. Returns `0` when the tag is not present.
///
/// On ARM the bootrom uses a single lookup function for both tables, so this
/// differs from [`rom_table_lookup`] only in the `mask` callers pass.
pub fn rom_data_lookup(tag: RomFnTableCode, mask: u32) -> usize {
    rom_table_lookup(tag, mask)
}

/// Error codes returned by bootrom calls that report an `i32`
/// (RP2350 datasheet §5.4.3).
///
/// `0` means success and is not represented here. A negative value maps to the
/// matching variant; anything the datasheet does not define maps to
/// [`BootRomError::Unknown`].
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BootRomError {
    /// The operation was disallowed by a security constraint.
    NotPermitted = -4,
    /// A parameter is outside the range of supported values.
    InvalidArg = -5,
    /// An address argument was out of bounds, or the caller may not access it.
    InvalidAddress = -10,
    /// An address argument was not correctly aligned.
    BadAlignment = -11,
    /// The request cannot be serviced in the current state.
    InvalidState = -12,
    /// A caller-supplied buffer was too small for the result.
    BufferTooSmall = -13,
    /// Another bootrom function must be called first.
    PreconditionNotMet = -14,
    /// Cached data was inconsistent with the data it was derived from.
    ModifiedData = -15,
    /// A data structure failed to validate.
    InvalidData = -16,
    /// A lookup or search found nothing.
    NotFound = -17,
    /// The change is impossible given the current state, e.g. clearing an OTP
    /// bit.
    UnsupportedModification = -18,
    /// A required bootrom lock is not owned.
    LockRequired = -19,
    /// An error this HAL does not name.
    Unknown = -1,
}

impl From<i32> for BootRomError {
    fn from(value: i32) -> Self {
        match value {
            -4 => Self::NotPermitted,
            -5 => Self::InvalidArg,
            -10 => Self::InvalidAddress,
            -11 => Self::BadAlignment,
            -12 => Self::InvalidState,
            -13 => Self::BufferTooSmall,
            -14 => Self::PreconditionNotMet,
            -15 => Self::ModifiedData,
            -16 => Self::InvalidData,
            -17 => Self::NotFound,
            -18 => Self::UnsupportedModification,
            -19 => Self::LockRequired,
            _ => Self::Unknown,
        }
    }
}

/// Read system information into `out`, selected by `flags`.
///
/// This is the bootrom's `get_sys_info` call (tag `GS`) without any decoding:
/// `out` must have room for the whole block, and the return value is the number
/// of words the bootrom wrote — the first of which reports which requested
/// flags it satisfied — or a negative [`BootRomError`] code. See
/// [`crate::sysinfo_rom`] for typed access.
///
/// The flag values and block layouts are in RP2350 datasheet §5.5.11.1.
pub fn get_sys_info(out: &mut [u32], flags: u32) -> i32 {
    let func = get_sys_info_ptr();

    // SAFETY: `out` is a valid slice and its length is passed alongside it.
    unsafe { func(out.as_mut_ptr(), out.len(), flags) }
}

/// Resolve the `get_sys_info` entry, preferring the Secure table.
fn get_sys_info_ptr() -> RomGetSysInfoFn {
    let ptr = rom_table_lookup(TAG_GET_SYS_INFO, rt_flags::FUNC_ARM_SEC);
    let ptr = if ptr == 0 {
        rom_table_lookup(TAG_GET_SYS_INFO, rt_flags::FUNC_ARM_NONSEC)
    } else {
        ptr
    };
    assert!(ptr != 0, "bootrom has no 'GS' (get_sys_info) entry");

    // SAFETY: the bootrom only ever stores a `rom_get_sys_info_fn` under the
    // 'GS' tag, and `usize`/function pointers have the same size here.
    unsafe { core::mem::transmute(ptr) }
}

/// Perform a checked flash read, erase or program (tag `FO`).
///
/// `flags` is the bootrom's `CFLASH` word: an operation, an address space and
/// an effective security level (RP2350 datasheet §5.5.8.2). The bootrom checks
/// the range against the devices described by `FLASH_DEVINFO` and, when a
/// partition table is in effect, against the partition permissions, then runs
/// the operation. Erase needs 4 KiB alignment, program 256-byte alignment, and
/// read has no constraint.
///
/// Returns `0` on success or a negative [`BootRomError`] code. See
/// [`crate::flash`] for the typed, high-level wrapper.
///
/// # Safety
///
/// `buffer` must point at `size_bytes` readable bytes for a read, or
/// `size_bytes` writable bytes for a program. An erase does not touch it, so it
/// may be null with any `size_bytes`. The bootrom validates the flash range,
/// not the buffer.
///
/// QMI is in direct mode while the call runs, so neither core may fetch
/// instructions or data from XIP: run the caller from RAM.
pub unsafe fn flash_op(flags: u32, addr: u32, size_bytes: u32, buffer: *mut u8) -> i32 {
    let func = flash_op_ptr();

    // SAFETY: the caller upholds the buffer contract described above.
    unsafe { func(flags, addr, size_bytes, buffer) }
}

/// Resolve the `flash_op` entry, preferring the Secure table.
fn flash_op_ptr() -> RomFlashOpFn {
    let ptr = rom_table_lookup(TAG_FLASH_OP, rt_flags::FUNC_ARM_SEC);
    let ptr = if ptr == 0 {
        rom_table_lookup(TAG_FLASH_OP, rt_flags::FUNC_ARM_NONSEC)
    } else {
        ptr
    };
    assert!(ptr != 0, "bootrom has no 'FO' (flash_op) entry");

    // SAFETY: the bootrom only ever stores a `rom_flash_op_fn` under the 'FO'
    // tag, and `usize`/function pointers have the same size here.
    unsafe { core::mem::transmute(ptr) }
}

/// Ask the bootrom to reset the chip through the watchdog.
///
/// `flags` selects the boot path; see [`crate::reboot`] for the typed API and
/// the meaning of `p0`/`p1` (RP2350 datasheet §5.5.10.1). The reset is
/// asynchronous: it fires `delay_ms` after this call returns.
///
/// `p0`/`p1` are per boot type: `BOOTSEL` takes the flag word (interface
/// disables, activity LED) in `p0` and the activity GPIO number in `p1`,
/// whereas `PC_SP` takes PC then SP. The `bootrom.h` and
/// `picoboot_constants.h` comments have `BOOTSEL` the other way round, but the
/// SDK's own RP2350 implementation of `reset_usb_boot()` passes the flags in
/// `p0` (`pico_bootrom/bootrom.c`, the `#elif defined(ROM_FUNC_REBOOT)`
/// branch), and that is the ordering mirrored here.
///
/// Returns the bootrom result: `0` (`BOOTROM_OK`) when the reboot was
/// initiated, or a negative error code when it was refused.
///
/// # Safety
///
/// The bootrom performs no validation of `p0`/`p1`. With a `PC_SP` boot the
/// chip jumps to `p0` with stack pointer `p1`, and `RAM_IMAGE` / `FLASH_UPDATE`
/// interpret `p0` as an address, so the `(flags, p0, p1)` triple must be
/// self-consistent. Prefer [`crate::reboot::reboot`].
pub unsafe fn reboot(flags: u32, delay_ms: u32, p0: u32, p1: u32) -> i32 {
    let func = reboot_ptr();
    // SAFETY: the caller upholds the contract described above.
    unsafe { func(flags, delay_ms, p0, p1) }
}

/// Read the well-known pointer to the bootrom's table lookup function.
fn table_lookup_ptr() -> usize {
    // SAFETY: both addresses hold the bootrom's well-known pointer. ROM
    // version 1 stores it as a 32-bit value at 0x18; later versions store a
    // 16-bit value at 0x16, which is already a full address because the ROM is
    // mapped at address 0.
    unsafe {
        if rom_version_number() == 1 {
            ROM_TABLE_LOOKUP_V1.read() as usize
        } else {
            ROM_TABLE_LOOKUP_V2.read() as usize
        }
    }
}

/// Resolve the `reboot` entry, preferring the Secure table.
fn reboot_ptr() -> RomRebootFn {
    let ptr = rom_table_lookup(TAG_REBOOT, rt_flags::FUNC_ARM_SEC);
    let ptr = if ptr == 0 {
        rom_table_lookup(TAG_REBOOT, rt_flags::FUNC_ARM_NONSEC)
    } else {
        ptr
    };
    assert!(ptr != 0, "bootrom has no 'RB' (reboot) entry");

    // SAFETY: the bootrom only ever stores a `rom_reboot_fn` under the 'RB'
    // tag, and `usize`/function pointers have the same size on this target.
    unsafe { core::mem::transmute(ptr) }
}

/// The eight most significant hex digits of the bootrom's git revision.
///
/// Useful when a workaround depends on which bootrom build is resident.
pub fn git_revision() -> u32 {
    let ptr = rom_data_lookup(TAG_GIT_REVISION, rt_flags::DATA) as *const u32;
    assert!(!ptr.is_null(), "bootrom has no 'GR' (git revision) entry");

    // SAFETY: the 'GR' entry points at a u32 in the bootrom, which is readable
    // in every security state.
    unsafe { ptr.read() }
}

/// Pointer to the resident partition table.
///
/// The bootrom keeps the loaded partition table in RAM and consults it for
/// flash permissions and ranges. The `PT` data entry holds a pointer to a
/// pointer to that structure, whose layout the bootrom treats as internal, so
/// this returns a raw word-aligned pointer to it.
pub fn partition_table_pointer() -> *const u32 {
    let entry = rom_data_lookup(TAG_PARTITION_TABLE, rt_flags::DATA) as *const *const u32;
    assert!(
        !entry.is_null(),
        "bootrom has no 'PT' (partition table) entry"
    );

    // SAFETY: the 'PT' entry points at a pointer to the resident table.
    unsafe { entry.read() }
}
