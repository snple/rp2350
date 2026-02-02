//! MSP - Main Stack Pointer
//!
//! The Main Stack Pointer is the default stack pointer. It is used:
//! - In Handler mode (exception/interrupt handlers)
//! - In Thread mode when CONTROL.SPSEL = 0

use core::arch::asm;

/// Read the Main Stack Pointer
#[inline(always)]
pub fn read() -> u32 {
    let msp: u32;
    unsafe {
        asm!("mrs {0}, MSP", out(reg) msp, options(nomem, nostack, preserves_flags));
    }
    msp
}

/// Write the Main Stack Pointer
///
/// # Safety
///
/// Changing the stack pointer is extremely dangerous. The caller must
/// ensure the new value points to valid, properly aligned stack memory.
/// The stack must be 8-byte aligned per ARM ABI requirements.
#[inline(always)]
pub unsafe fn write(msp: u32) {
    unsafe {
        asm!("msr MSP, {0}", in(reg) msp, options(nomem, nostack, preserves_flags));
    }
}
