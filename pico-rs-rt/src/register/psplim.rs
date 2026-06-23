//! PSPLIM - Process Stack Pointer Limit (ARMv8-M)
//!
//! The Process Stack Pointer Limit register defines the lower limit for PSP.
//! If the stack pointer goes below this limit, a fault is generated.
//! This helps catch stack overflow conditions for task stacks.
//!
//! This register is only available on ARMv8-M (Cortex-M33, M23, etc.)

use core::arch::asm;

/// Read the Process Stack Pointer Limit
#[inline(always)]
pub fn read() -> u32 {
    let psplim: u32;
    unsafe {
        asm!("mrs {0}, PSPLIM", out(reg) psplim, options(nomem, nostack, preserves_flags));
    }
    psplim
}

/// Write the Process Stack Pointer Limit
///
/// Sets the lower bound for valid PSP values. Must be word-aligned.
///
/// # Safety
///
/// Setting an incorrect limit could cause spurious faults or fail to
/// detect actual stack overflows.
#[inline(always)]
pub unsafe fn write(psplim: u32) {
    unsafe {
        asm!("msr PSPLIM, {0}", in(reg) psplim, options(nomem, nostack, preserves_flags));
    }
}
