//! System reboot through the bootrom
//!
//! [`reboot`] asks the bootrom to reset the chip via the watchdog, choosing the
//! boot path: normal flash boot, BOOTSEL (`usb boot`), an image in RAM, a flash
//! update, or a direct PC/SP jump. This is the mechanism the bootrom's own USB
//! boot interfaces use, and it is a superset of
//! [`watchdog::reboot`](crate::watchdog::reboot), which always takes the normal
//! path.
//!
//! ## Features
//!
//! - Normal / BOOTSEL / RAM image / flash update / PC+SP boot targets
//! - BOOTSEL with the picoboot and/or mass-storage USB interface suppressed
//!
//! ## Example
//!
//! ```ignore
//! use rp2350::reboot::{self, RebootKind};
//!
//! // Equivalent of the SDK's `reset_usb_boot(0, 0)`: reboot into BOOTSEL with
//! // both USB interfaces enabled and no activity LED.
//! reboot::reboot(RebootKind::BootSel {
//!     picoboot_disabled: false,
//!     msd_disabled: false,
//! });
//! ```
//!
//! ## Notes
//!
//! The reset is asynchronous: the bootrom arms the watchdog and returns, and
//! the chip resets [`REBOOT_DELAY_MS`] later. [`reboot`] never returns — it
//! spins until the reset happens, and panics if the bootrom refuses the request
//! (for example [`RebootKind::PcSp`] from a Non-secure ARM caller).
//!
//! This HAL is ARM-only (Cortex-M33), so no architecture-switch flags are set:
//! both cores stay on the architecture the chip booted with.

/// Delay between arming the watchdog and the reset, in milliseconds.
///
/// The bootrom needs a few hundred microseconds to shut down USB and the other
/// boot-time state; this matches what `picotool` and `rp235x-hal` use.
pub const REBOOT_DELAY_MS: u32 = 500;

// Boot types (`flags[3:0]`), RP2350 datasheet §5.5.10.1.
/// Reboot through the normal boot path.
const REBOOT_TYPE_NORMAL: u32 = 0x0000;
/// Reboot like BOOTSEL being held during power-up.
const REBOOT_TYPE_BOOTSEL: u32 = 0x0002;
/// Search RAM (including XIP RAM) for an image and boot it.
const REBOOT_TYPE_RAM_IMAGE: u32 = 0x0003;
/// Normal boot, but preferring the partition that was just updated.
const REBOOT_TYPE_FLASH_UPDATE: u32 = 0x0004;
/// Jump straight to a PC/SP pair (secure ARM callers only).
const REBOOT_TYPE_PC_SP: u32 = 0x000d;

// `p0` carries these flags; `p1` is the optional activity-indicator GPIO
// number, which this API does not expose.
/// BOOTSEL flag: do not expose the picoboot interface (`p0` bit 1).
const BOOTSEL_DISABLE_PICOBOOT: u32 = 0x02;
/// BOOTSEL flag: do not expose the mass-storage interface (`p0` bit 0).
const BOOTSEL_DISABLE_MSD: u32 = 0x01;

/// Which boot path the next reset should take.
///
/// Mirrors the bootrom's `REBOOT_TYPE_*` values; see RP2350 datasheet §5.5.10.1.
#[allow(unpredictable_function_pointer_comparisons)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RebootKind {
    /// Reboot through the normal boot path (flash boot, partition table rules
    /// apply).
    Normal,

    /// Reboot like BOOTSEL is held down during power-up: the chip exposes the
    /// USB boot interfaces (picoboot and/or mass storage) and waits for a host.
    ///
    /// Leaving both flags `false` is the same as the SDK's
    /// `reset_usb_boot(0, 0)`; the activity LED is not used.
    BootSel {
        /// Do not expose the picoboot (vendor) USB interface.
        picoboot_disabled: bool,
        /// Do not expose the USB mass-storage interface.
        msd_disabled: bool,
    },

    /// Search a region of RAM (including XIP RAM) for an image and boot it.
    ///
    /// This is the reboot the bootrom performs for a RAM UF2 dragged onto the
    /// BOOTSEL mass-storage drive. The region must be word-aligned.
    Ram {
        /// Start of the region to search.
        start_addr: *const u32,
        /// Size of the region in bytes.
        size: usize,
    },

    /// Normal boot, but prefer the flash partition/slot containing
    /// `start_addr` when the boot paths disagree — the reboot after a flash
    /// update (TBYB, version downgrades).
    FlashUpdate {
        /// Start address of the flash region that was updated.
        start_addr: *const u32,
    },

    /// Reboot directly into `pc` with stack pointer `sp`.
    ///
    /// Arm only: the bootrom rejects this from a Non-secure ARM caller.
    PcSp {
        /// Initial program counter.
        pc: fn() -> !,
        /// Initial stack pointer.
        sp: *const u32,
    },
}

/// Reboot the chip. Never returns.
///
/// # Panics
///
/// Panics if the bootrom refuses the reboot, e.g. [`RebootKind::PcSp`] from a
/// Non-secure ARM caller.
pub fn reboot(kind: RebootKind) -> ! {
    let (boot_type, p0, p1) = match kind {
        RebootKind::Normal => (REBOOT_TYPE_NORMAL, 0, 0),
        RebootKind::BootSel {
            picoboot_disabled,
            msd_disabled,
        } => {
            let mut flags = 0;
            if picoboot_disabled {
                flags |= BOOTSEL_DISABLE_PICOBOOT;
            }
            if msd_disabled {
                flags |= BOOTSEL_DISABLE_MSD;
            }
            // p1 would be the activity LED GPIO number; with the
            // GPIO_PIN_SPECIFIED flag clear the bootrom ignores it.
            (REBOOT_TYPE_BOOTSEL, flags, 0)
        }
        RebootKind::Ram { start_addr, size } => {
            (REBOOT_TYPE_RAM_IMAGE, start_addr as u32, size as u32)
        }
        RebootKind::FlashUpdate { start_addr } => (REBOOT_TYPE_FLASH_UPDATE, start_addr as u32, 0),
        RebootKind::PcSp { pc, sp } => (REBOOT_TYPE_PC_SP, pc as usize as u32, sp as u32),
    };

    // SAFETY: `p0`/`p1` are built from the matching `RebootKind` variant above,
    // and no architecture-switch bits are set (this HAL is ARM-only).
    let rc = unsafe { crate::bootrom::reboot(boot_type, REBOOT_DELAY_MS, p0, p1) };
    if rc < 0 {
        panic!("bootrom refused reboot (error {})", rc);
    }

    // The watchdog reset is asynchronous: spin until it fires.
    loop {
        core::hint::spin_loop();
    }
}
