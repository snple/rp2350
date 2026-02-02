//! Non-Secure Access Control Register (NSACR)
//!
//! The NSACR controls Non-Secure access to coprocessors and system features.
//! This register is only accessible from Secure state.

use core::arch::asm;

/// NSACR Register bits
pub mod bits {
    /// CP0 Non-Secure access enable
    pub const CP0: u32 = 1 << 0;
    /// CP1 Non-Secure access enable
    pub const CP1: u32 = 1 << 1;
    /// CP2 Non-Secure access enable
    pub const CP2: u32 = 1 << 2;
    /// CP3 Non-Secure access enable
    pub const CP3: u32 = 1 << 3;
    /// CP4 Non-Secure access enable
    pub const CP4: u32 = 1 << 4;
    /// CP5 Non-Secure access enable
    pub const CP5: u32 = 1 << 5;
    /// CP6 Non-Secure access enable
    pub const CP6: u32 = 1 << 6;
    /// CP7 Non-Secure access enable
    pub const CP7: u32 = 1 << 7;
    /// CP10 (FPU) Non-Secure access enable
    pub const CP10: u32 = 1 << 10;
    /// CP11 (FPU) Non-Secure access enable
    pub const CP11: u32 = 1 << 11;
}

/// Read the NSACR register
///
/// # Safety
///
/// This is only valid when executed from Secure state.
#[inline(always)]
pub unsafe fn read() -> u32 {
    let result: u32;
    unsafe {
        asm!(
            "mrs {0}, NSACR",
            out(reg) result,
            options(nomem, nostack, preserves_flags),
        )
    }
    result
}

/// Write the NSACR register
///
/// # Safety
///
/// This is only valid when executed from Secure state.
/// Changing NSACR affects Non-Secure code's access to coprocessors.
#[inline(always)]
pub unsafe fn write(value: u32) {
    unsafe {
        asm!(
            "msr NSACR, {0}",
            in(reg) value,
            options(nomem, nostack, preserves_flags),
        )
    }
}

/// Allow Non-Secure access to the FPU (CP10/CP11)
///
/// # Safety
///
/// This is only valid when executed from Secure state.
#[inline(always)]
pub unsafe fn allow_fpu_nonsecure() {
    unsafe {
        let nsacr = read();
        write(nsacr | bits::CP10 | bits::CP11);
    }
}

/// Deny Non-Secure access to the FPU (CP10/CP11)
///
/// # Safety
///
/// This is only valid when executed from Secure state.
#[inline(always)]
pub unsafe fn deny_fpu_nonsecure() {
    unsafe {
        let nsacr = read();
        write(nsacr & !(bits::CP10 | bits::CP11));
    }
}
