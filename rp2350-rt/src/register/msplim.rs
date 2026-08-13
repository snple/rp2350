//! MSPLIM - Main Stack Pointer Limit (ARMv8-M)
//!
//! The Main Stack Pointer Limit register defines the lower limit for MSP.
//! If the stack pointer goes below this limit, a fault is generated.
//! This helps catch stack overflow conditions.
//!
//! This register is only available on ARMv8-M (Cortex-M33, M23, etc.)

use core::arch::asm;

/// Read the Main Stack Pointer Limit
#[inline(always)]
pub fn read() -> u32 {
    let msplim: u32;
    unsafe {
        asm!("mrs {0}, MSPLIM", out(reg) msplim, options(nomem, nostack, preserves_flags));
    }
    msplim
}

/// Write the Main Stack Pointer Limit
///
/// Sets the lower bound for valid MSP values. Must be word-aligned.
///
/// # Safety
///
/// Setting an incorrect limit could cause spurious faults or fail to
/// detect actual stack overflows.
#[inline(always)]
pub unsafe fn write(msplim: u32) {
    unsafe {
        asm!("msr MSPLIM, {0}", in(reg) msplim, options(nomem, nostack, preserves_flags));
    }
}
