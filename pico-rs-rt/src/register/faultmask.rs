//! FAULTMASK Register
//!
//! The Fault Mask Register prevents activation of all exceptions except NMI.
//! When FAULTMASK is set, all exceptions including HardFault are masked.
//!
//! This register is only available on ARMv7-M and ARMv8-M Mainline (including M33).

use core::arch::asm;

/// FAULTMASK register value
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Faultmask {
    bits: u32,
}

impl Faultmask {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get raw bits
    #[inline(always)]
    pub const fn bits(self) -> u32 {
        self.bits
    }

    /// Check if faults are enabled (FAULTMASK.FM = 0)
    #[inline(always)]
    pub const fn is_active(self) -> bool {
        (self.bits & 1) == 0
    }

    /// Check if faults are masked (FAULTMASK.FM = 1)
    #[inline(always)]
    pub const fn is_inactive(self) -> bool {
        (self.bits & 1) != 0
    }
}

/// Read the FAULTMASK register
#[inline(always)]
pub fn read() -> Faultmask {
    let bits: u32;
    unsafe {
        asm!("mrs {0}, FAULTMASK", out(reg) bits, options(nomem, nostack, preserves_flags));
    }
    Faultmask::from_bits(bits)
}

/// Enable all configurable faults (clear FAULTMASK)
///
/// # Safety
///
/// Enabling faults in contexts where they were previously masked may
/// cause unexpected behavior.
#[inline(always)]
pub unsafe fn enable_faults() {
    unsafe {
        asm!("cpsie f", options(nomem, nostack, preserves_flags));
    }
}

/// Disable all configurable faults (set FAULTMASK)
///
/// When set, only NMI can be taken. Use sparingly and for very short
/// durations as this prevents fault handling.
#[inline(always)]
pub fn disable_faults() {
    unsafe {
        asm!("cpsid f", options(nomem, nostack, preserves_flags));
    }
}
