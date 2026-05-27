//! BASEPRI Register
//!
//! The Base Priority register defines the minimum priority for exception
//! processing. When set to a non-zero value, it prevents the activation
//! of all exceptions with the same or lower priority level.
//!
//! This register is only available on ARMv7-M and ARMv8-M Mainline (including M33).

use core::arch::asm;

/// BASEPRI register value
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Basepri {
    bits: u8,
}

impl Basepri {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u8) -> Self {
        Self { bits }
    }

    /// Get raw bits
    #[inline(always)]
    pub const fn bits(self) -> u8 {
        self.bits
    }

    /// Check if base priority is active (non-zero)
    ///
    /// When active, exceptions with priority equal to or lower than this
    /// value are masked.
    #[inline(always)]
    pub const fn is_active(self) -> bool {
        self.bits != 0
    }
}

/// Read the BASEPRI register
#[inline(always)]
pub fn read() -> Basepri {
    let bits: u32;
    unsafe {
        asm!("mrs {0}, BASEPRI", out(reg) bits, options(nomem, nostack, preserves_flags));
    }
    Basepri::from_bits(bits as u8)
}

/// Write the BASEPRI register
///
/// Sets the minimum priority for exception processing. Setting to 0
/// disables the effect of BASEPRI.
///
/// On Cortex-M33, only the upper bits are implemented (typically 4 bits),
/// so valid values are 0, 16, 32, 48, ..., 240.
///
/// # Safety
///
/// Modifying BASEPRI changes which exceptions can preempt the current
/// execution context, which can affect system behavior.
#[inline(always)]
pub unsafe fn write(basepri: Basepri) {
    unsafe {
        asm!(
            "msr BASEPRI, {0}",
            in(reg) basepri.bits as u32,
            options(nomem, nostack, preserves_flags)
        );
    }
}

/// Write BASEPRI_MAX (only raises priority, never lowers)
///
/// This variant only updates BASEPRI if the new value is higher priority
/// (lower numerical value) than the current value. This is useful for
/// implementing nested critical sections.
///
/// # Safety
///
/// Same safety considerations as `write()`.
#[inline(always)]
pub unsafe fn write_max(basepri: Basepri) {
    unsafe {
        asm!(
            "msr BASEPRI_MAX, {0}",
            in(reg) basepri.bits as u32,
            options(nomem, nostack, preserves_flags)
        );
    }
}
