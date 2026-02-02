//! PSP - Process Stack Pointer
//!
//! The Process Stack Pointer is an alternative stack pointer that can be
//! used in Thread mode when CONTROL.SPSEL = 1. This is commonly used
//! in RTOS implementations where each task has its own stack.

use core::arch::asm;

/// Read the Process Stack Pointer
#[inline(always)]
pub fn read() -> u32 {
    let psp: u32;
    unsafe {
        asm!("mrs {0}, PSP", out(reg) psp, options(nomem, nostack, preserves_flags));
    }
    psp
}

/// Write the Process Stack Pointer
///
/// # Safety
///
/// Changing the stack pointer is extremely dangerous. The caller must
/// ensure the new value points to valid, properly aligned stack memory.
/// The stack must be 8-byte aligned per ARM ABI requirements.
#[inline(always)]
pub unsafe fn write(psp: u32) {
    unsafe {
        asm!("msr PSP, {0}", in(reg) psp, options(nomem, nostack, preserves_flags));
    }
}
