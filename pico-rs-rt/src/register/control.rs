//! CONTROL Register
//!
//! The Control Register controls the stack used and the privilege level
//! for software execution when the processor is in Thread mode.
//!
//! Fields:
//! - `nPRIV` (bit 0): Thread mode privilege level
//!   - 0 = Privileged
//!   - 1 = Unprivileged
//! - `SPSEL` (bit 1): Stack pointer selection
//!   - 0 = Use MSP (Main Stack Pointer)
//!   - 1 = Use PSP (Process Stack Pointer)
//! - `FPCA` (bit 2): FP extension active (if FPU present)

use core::arch::asm;

/// Thread mode privilege level
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Npriv {
    /// Privileged mode
    Privileged = 0,
    /// Unprivileged mode
    Unprivileged = 1,
}

/// Stack pointer selection
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Spsel {
    /// Main Stack Pointer
    Msp = 0,
    /// Process Stack Pointer
    Psp = 1,
}

/// CONTROL register value
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Control {
    bits: u32,
}

impl Control {
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

    /// Get privilege level
    #[inline(always)]
    pub const fn npriv(self) -> Npriv {
        if (self.bits & (1 << 0)) != 0 {
            Npriv::Unprivileged
        } else {
            Npriv::Privileged
        }
    }

    /// Get stack pointer selection
    #[inline(always)]
    pub const fn spsel(self) -> Spsel {
        if (self.bits & (1 << 1)) != 0 {
            Spsel::Psp
        } else {
            Spsel::Msp
        }
    }

    /// Check if FP extension is active
    #[inline(always)]
    pub const fn fpca(self) -> bool {
        (self.bits & (1 << 2)) != 0
    }

    /// Create with specified privilege level
    #[inline(always)]
    pub const fn with_npriv(self, npriv: Npriv) -> Self {
        let bits = (self.bits & !(1 << 0)) | (npriv as u32);
        Self { bits }
    }

    /// Create with specified stack pointer
    #[inline(always)]
    pub const fn with_spsel(self, spsel: Spsel) -> Self {
        let bits = (self.bits & !(1 << 1)) | ((spsel as u32) << 1);
        Self { bits }
    }
}

/// Read the CONTROL register
#[inline(always)]
pub fn read() -> Control {
    let bits: u32;
    unsafe {
        asm!("mrs {0}, CONTROL", out(reg) bits, options(nomem, nostack, preserves_flags));
    }
    Control::from_bits(bits)
}

/// Write the CONTROL register
///
/// # Safety
///
/// Modifying the CONTROL register can:
/// - Switch between privileged and unprivileged mode
/// - Switch the active stack pointer
///   Both operations require careful handling to avoid undefined behavior.
///
/// An ISB instruction should be used after modifying CONTROL to ensure
/// the effects are visible to subsequent instructions.
#[inline(always)]
pub unsafe fn write(control: Control) {
    unsafe {
        asm!("msr CONTROL, {0}", in(reg) control.bits, options(nomem, nostack, preserves_flags));
    }
}
