//! PRIMASK Register
//!
//! The Priority Mask Register prevents activation of all exceptions with
//! configurable priority. Setting PRIMASK effectively disables all
//! interrupts except NMI and HardFault.

use core::arch::asm;

/// PRIMASK register value
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Primask {
    bits: u32,
}

impl Primask {
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

    /// Check if interrupts are enabled (PRIMASK.PM = 0)
    #[inline(always)]
    pub const fn interrupts_enabled(self) -> bool {
        (self.bits & 1) == 0
    }

    /// Check if interrupts are disabled (PRIMASK.PM = 1)
    #[inline(always)]
    pub const fn interrupts_disabled(self) -> bool {
        (self.bits & 1) != 0
    }
}

/// Read the PRIMASK register
#[inline(always)]
pub fn read() -> Primask {
    let bits: u32;
    unsafe {
        asm!("mrs {0}, PRIMASK", out(reg) bits, options(nomem, nostack, preserves_flags));
    }
    Primask::from_bits(bits)
}

/// Read raw PRIMASK value
#[inline(always)]
pub fn read_raw() -> u32 {
    let bits: u32;
    unsafe {
        asm!("mrs {0}, PRIMASK", out(reg) bits, options(nomem, nostack, preserves_flags));
    }
    bits
}

/// Write raw PRIMASK value
///
/// # Safety
///
/// Writing to PRIMASK changes interrupt masking state which can affect
/// system behavior. Ensure proper nesting with read_raw() for restore.
#[inline(always)]
pub unsafe fn write_raw(bits: u32) {
    unsafe {
        asm!("msr PRIMASK, {0}", in(reg) bits, options(nomem, nostack, preserves_flags));
    }
}
