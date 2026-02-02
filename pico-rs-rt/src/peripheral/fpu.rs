//! Floating Point Unit (FPU)
//!
//! The Cortex-M33 on RP2350 includes an FPv5 floating-point unit supporting:
//!
//! - Single-precision (float32) operations
//! - Half-precision (float16) conversion instructions (ARMv8-M)
//! - Hardware square root and division
//! - IEEE 754 compliant rounding modes
//!
//! # Lazy Context Saving
//!
//! By default, the FPU uses lazy context saving to improve interrupt latency.
//! FPU registers are only saved/restored when actually needed.
//!
//! # FPU Registers
//!
//! - S0-S31: Single-precision registers (also accessible as D0-D15 double)
//! - FPSCR: Floating-point status and control register
//! - FPCCR: Floating-point context control register
//! - FPCAR: Floating-point context address register
//! - FPDSCR: Floating-point default status control register

use core::marker::PhantomData;

use super::{Reg32, peripheral};

/// FPU base address (in System Control Space)
const FPU_BASE: u32 = 0xE000_EF34;

/// CPACR address (Coprocessor Access Control Register)
const CPACR: *mut u32 = 0xE000_ED88 as *mut u32;

/// FPU Register Block
#[repr(C)]
pub struct RegisterBlock {
    /// Floating-point Context Control Register
    pub fpccr: Reg32,
    /// Floating-point Context Address Register
    pub fpcar: Reg32,
    /// Floating-point Default Status Control Register
    pub fpdscr: Reg32,
    /// Media and VFP Feature Register 0 (read-only)
    pub mvfr0: Reg32,
    /// Media and VFP Feature Register 1 (read-only)
    pub mvfr1: Reg32,
    /// Media and VFP Feature Register 2 (read-only)
    pub mvfr2: Reg32,
}

/// FPCCR (Floating-point Context Control Register) bits
pub mod fpccr {
    /// Lazy state preservation active
    pub const LSPACT: u32 = 1 << 0;
    /// User privilege level was active when lazy state preservation was activated
    pub const USER: u32 = 1 << 1;
    /// Thread mode was active when lazy state preservation was activated
    pub const THREAD: u32 = 1 << 3;
    /// HardFault handler
    pub const HFRDY: u32 = 1 << 4;
    /// MemManage fault handler
    pub const MMRDY: u32 = 1 << 5;
    /// BusFault handler
    pub const BFRDY: u32 = 1 << 6;
    /// Debug monitor pending
    pub const MONRDY: u32 = 1 << 8;
    /// Secure exception was active when lazy state preservation was activated
    pub const SFRDY: u32 = 1 << 7;
    /// Lazy state preservation for secure context
    pub const SPLIMVIOL: u32 = 1 << 9;
    /// Uses Thread stack for context save
    pub const UFRDY: u32 = 1 << 10;
    /// Enable automatic lazy state preservation for FP context
    pub const ASPEN: u32 = 1 << 30;
    /// Enable lazy context save
    pub const LSPEN: u32 = 1 << 31;
}

/// FPSCR (Floating-point Status and Control Register) bits
pub mod fpscr {
    /// Invalid Operation cumulative exception flag
    pub const IOC: u32 = 1 << 0;
    /// Division by Zero cumulative exception flag
    pub const DZC: u32 = 1 << 1;
    /// Overflow cumulative exception flag
    pub const OFC: u32 = 1 << 2;
    /// Underflow cumulative exception flag
    pub const UFC: u32 = 1 << 3;
    /// Inexact cumulative exception flag
    pub const IXC: u32 = 1 << 4;
    /// Input Denormal cumulative exception flag
    pub const IDC: u32 = 1 << 7;

    /// Rounding mode mask
    pub const RMODE_MASK: u32 = 0b11 << 22;
    /// Round to Nearest (RN) mode
    pub const RMODE_RN: u32 = 0b00 << 22;
    /// Round towards Plus Infinity (RP) mode
    pub const RMODE_RP: u32 = 0b01 << 22;
    /// Round towards Minus Infinity (RM) mode
    pub const RMODE_RM: u32 = 0b10 << 22;
    /// Round towards Zero (RZ) mode
    pub const RMODE_RZ: u32 = 0b11 << 22;

    /// Flush-to-Zero mode enable
    pub const FZ: u32 = 1 << 24;
    /// Default NaN mode enable
    pub const DN: u32 = 1 << 25;
    /// Alternative Half-Precision control
    pub const AHP: u32 = 1 << 26;

    /// Negative condition flag
    pub const N: u32 = 1 << 31;
    /// Zero condition flag
    pub const Z: u32 = 1 << 30;
    /// Carry condition flag
    pub const C: u32 = 1 << 29;
    /// Overflow condition flag
    pub const V: u32 = 1 << 28;
}

/// FPU Rounding Mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum RoundingMode {
    /// Round to Nearest (default)
    Nearest = fpscr::RMODE_RN,
    /// Round towards Plus Infinity
    PlusInfinity = fpscr::RMODE_RP,
    /// Round towards Minus Infinity
    MinusInfinity = fpscr::RMODE_RM,
    /// Round towards Zero
    Zero = fpscr::RMODE_RZ,
}

/// FPU Exception Flags
#[derive(Debug, Clone, Copy, Default)]
pub struct ExceptionFlags {
    /// Invalid operation occurred
    pub invalid_operation: bool,
    /// Division by zero occurred
    pub division_by_zero: bool,
    /// Overflow occurred
    pub overflow: bool,
    /// Underflow occurred
    pub underflow: bool,
    /// Inexact result
    pub inexact: bool,
    /// Input denormal
    pub input_denormal: bool,
}

/// FPU peripheral
pub struct FPU {
    _marker: PhantomData<*const ()>,
}

peripheral!(FPU, RegisterBlock, FPU_BASE);

impl FPU {
    /// Enable the FPU
    ///
    /// Sets CP10 and CP11 to full access in CPACR.
    #[inline]
    pub fn enable() {
        unsafe {
            let cpacr = core::ptr::read_volatile(CPACR);
            // CP10 and CP11 = 0b11 (full access)
            core::ptr::write_volatile(CPACR, cpacr | (0b11 << 20) | (0b11 << 22));
        }
        crate::asm::dsb();
        crate::asm::isb();
    }

    /// Disable the FPU
    ///
    /// Clears CP10 and CP11 in CPACR.
    #[inline]
    pub fn disable() {
        unsafe {
            let cpacr = core::ptr::read_volatile(CPACR);
            core::ptr::write_volatile(CPACR, cpacr & !((0b11 << 20) | (0b11 << 22)));
        }
        crate::asm::dsb();
        crate::asm::isb();
    }

    /// Check if FPU is enabled
    #[inline]
    pub fn is_enabled() -> bool {
        let cpacr = unsafe { core::ptr::read_volatile(CPACR) };
        // Check if both CP10 and CP11 have full access
        (cpacr & ((0b11 << 20) | (0b11 << 22))) == ((0b11 << 20) | (0b11 << 22))
    }

    /// Enable lazy context saving
    ///
    /// When enabled, FPU registers are only saved during context switch
    /// if they were actually used.
    #[inline]
    pub fn enable_lazy_stacking(&mut self) {
        unsafe {
            (*Self::PTR)
                .fpccr
                .modify(|v| v | fpccr::LSPEN | fpccr::ASPEN);
        }
    }

    /// Disable lazy context saving
    ///
    /// FPU registers will always be saved during context switch.
    #[inline]
    pub fn disable_lazy_stacking(&mut self) {
        unsafe {
            (*Self::PTR)
                .fpccr
                .modify(|v| v & !(fpccr::LSPEN | fpccr::ASPEN));
        }
    }

    /// Check if lazy stacking is enabled
    #[inline]
    pub fn is_lazy_stacking_enabled(&self) -> bool {
        unsafe { (*Self::PTR).fpccr.read() & (fpccr::LSPEN | fpccr::ASPEN) != 0 }
    }

    /// Read the FPSCR (Floating-point Status and Control Register)
    #[inline]
    pub fn fpscr() -> u32 {
        let result: u32;
        unsafe {
            core::arch::asm!(
                "vmrs {0}, fpscr",
                out(reg) result,
                options(nomem, nostack),
            )
        }
        result
    }

    /// Write the FPSCR
    ///
    /// # Safety
    ///
    /// Changing FPSCR affects FPU behavior for all floating-point operations.
    #[inline]
    pub unsafe fn set_fpscr(value: u32) {
        unsafe {
            core::arch::asm!(
                "vmsr fpscr, {0}",
                in(reg) value,
                options(nomem, nostack),
            )
        }
    }

    /// Get current rounding mode
    #[inline]
    pub fn rounding_mode() -> RoundingMode {
        let fpscr = Self::fpscr();
        match fpscr & fpscr::RMODE_MASK {
            fpscr::RMODE_RN => RoundingMode::Nearest,
            fpscr::RMODE_RP => RoundingMode::PlusInfinity,
            fpscr::RMODE_RM => RoundingMode::MinusInfinity,
            fpscr::RMODE_RZ => RoundingMode::Zero,
            _ => RoundingMode::Nearest,
        }
    }

    /// Set rounding mode
    ///
    /// # Safety
    ///
    /// Changing the rounding mode affects all floating-point operations.
    #[inline]
    pub unsafe fn set_rounding_mode(mode: RoundingMode) {
        let fpscr = Self::fpscr();
        unsafe { Self::set_fpscr((fpscr & !fpscr::RMODE_MASK) | mode as u32) };
    }

    /// Get exception flags
    #[inline]
    pub fn exception_flags() -> ExceptionFlags {
        let fpscr = Self::fpscr();
        ExceptionFlags {
            invalid_operation: fpscr & fpscr::IOC != 0,
            division_by_zero: fpscr & fpscr::DZC != 0,
            overflow: fpscr & fpscr::OFC != 0,
            underflow: fpscr & fpscr::UFC != 0,
            inexact: fpscr & fpscr::IXC != 0,
            input_denormal: fpscr & fpscr::IDC != 0,
        }
    }

    /// Clear all exception flags
    #[inline]
    pub fn clear_exception_flags() {
        let fpscr = Self::fpscr();
        let mask = fpscr::IOC | fpscr::DZC | fpscr::OFC | fpscr::UFC | fpscr::IXC | fpscr::IDC;
        unsafe { Self::set_fpscr(fpscr & !mask) };
    }

    /// Enable flush-to-zero mode
    ///
    /// Denormal numbers are flushed to zero, improving performance.
    #[inline]
    pub fn enable_flush_to_zero() {
        let fpscr = Self::fpscr();
        unsafe { Self::set_fpscr(fpscr | fpscr::FZ) };
    }

    /// Disable flush-to-zero mode
    #[inline]
    pub fn disable_flush_to_zero() {
        let fpscr = Self::fpscr();
        unsafe { Self::set_fpscr(fpscr & !fpscr::FZ) };
    }

    /// Enable default NaN mode
    ///
    /// Any operation involving NaN returns the default NaN.
    #[inline]
    pub fn enable_default_nan() {
        let fpscr = Self::fpscr();
        unsafe { Self::set_fpscr(fpscr | fpscr::DN) };
    }

    /// Disable default NaN mode
    #[inline]
    pub fn disable_default_nan() {
        let fpscr = Self::fpscr();
        unsafe { Self::set_fpscr(fpscr & !fpscr::DN) };
    }

    /// Get MVFR0 (Media and VFP Feature Register 0)
    ///
    /// Describes the features of the FPU.
    #[inline]
    pub fn mvfr0(&self) -> u32 {
        unsafe { (*Self::PTR).mvfr0.read() }
    }

    /// Get MVFR1 (Media and VFP Feature Register 1)
    #[inline]
    pub fn mvfr1(&self) -> u32 {
        unsafe { (*Self::PTR).mvfr1.read() }
    }

    /// Get MVFR2 (Media and VFP Feature Register 2)
    ///
    /// ARMv8-M: Contains information about half-precision support.
    #[inline]
    pub fn mvfr2(&self) -> u32 {
        unsafe { (*Self::PTR).mvfr2.read() }
    }

    /// Check if half-precision (FP16) is supported
    #[inline]
    pub fn supports_half_precision(&self) -> bool {
        // MVFR1 bits [27:24] (FPHP field) indicate HP support
        (self.mvfr1() >> 24) & 0xF != 0
    }

    /// Check if single-precision is supported
    #[inline]
    pub fn supports_single_precision(&self) -> bool {
        // MVFR0 bits [7:4] (Single Precision field)
        (self.mvfr0() >> 4) & 0xF != 0
    }

    /// Check if hardware divide is supported
    #[inline]
    pub fn supports_divide(&self) -> bool {
        // MVFR0 bits [19:16] (Divide field)
        (self.mvfr0() >> 16) & 0xF != 0
    }

    /// Check if hardware square root is supported
    #[inline]
    pub fn supports_sqrt(&self) -> bool {
        // MVFR0 bits [23:20] (Square Root field)
        (self.mvfr0() >> 20) & 0xF != 0
    }
}

// Safety: FPU is a single peripheral
unsafe impl Send for FPU {}

// =============================================================================
// Half-precision conversion functions (ARMv8-M FPv5)
// =============================================================================

/// Convert single-precision float to half-precision
///
/// Uses the VCVTB instruction (ARMv8-M FPv5).
/// Requires FPU hardware support.
#[cfg(target_feature = "vfp2")]
#[inline(always)]
pub fn f32_to_f16(value: f32) -> u16 {
    let result: u32;
    unsafe {
        core::arch::asm!(
            "vcvtb.f16.f32 {out}, {inp}",
            out = out(sreg) result,
            inp = in(sreg) value,
            options(nomem, nostack),
        )
    }
    result as u16
}

/// Convert half-precision to single-precision float
///
/// Uses the VCVTB instruction (ARMv8-M FPv5).
/// Requires FPU hardware support.
#[cfg(target_feature = "vfp2")]
#[inline(always)]
pub fn f16_to_f32(value: u16) -> f32 {
    let result: f32;
    unsafe {
        core::arch::asm!(
            "vcvtb.f32.f16 {out}, {inp}",
            out = out(sreg) result,
            inp = in(sreg) value as u32,
            options(nomem, nostack),
        )
    }
    result
}

/// Convert single-precision float to half-precision (software fallback)
///
/// Used when FPU target feature is not available at compile time.
#[cfg(not(target_feature = "vfp2"))]
#[inline(always)]
pub fn f32_to_f16(value: f32) -> u16 {
    // Software implementation
    let bits = value.to_bits();
    let sign = (bits >> 16) & 0x8000;
    let exp = ((bits >> 23) & 0xFF) as i32;
    let mantissa = bits & 0x007F_FFFF;

    if exp == 255 {
        // Infinity or NaN
        if mantissa != 0 {
            // NaN
            (sign | 0x7E00) as u16
        } else {
            // Infinity
            (sign | 0x7C00) as u16
        }
    } else if exp > 142 {
        // Overflow to infinity
        (sign | 0x7C00) as u16
    } else if exp < 113 {
        // Underflow to zero (or subnormal)
        if exp < 103 {
            sign as u16
        } else {
            // Subnormal
            let shift = 126 - exp;
            let m = (mantissa | 0x0080_0000) >> shift;
            (sign | (m >> 13)) as u16
        }
    } else {
        // Normal number
        let new_exp = ((exp - 127 + 15) as u32) << 10;
        let new_mantissa = mantissa >> 13;
        (sign | new_exp | new_mantissa) as u16
    }
}

/// Convert half-precision to single-precision float (software fallback)
///
/// Used when FPU target feature is not available at compile time.
#[cfg(not(target_feature = "vfp2"))]
#[inline(always)]
pub fn f16_to_f32(value: u16) -> f32 {
    let sign = ((value as u32) & 0x8000) << 16;
    let exp = ((value >> 10) & 0x1F) as i32;
    let mantissa = (value & 0x03FF) as u32;

    let bits = if exp == 31 {
        // Infinity or NaN
        if mantissa != 0 {
            // NaN
            sign | 0x7FC0_0000
        } else {
            // Infinity
            sign | 0x7F80_0000
        }
    } else if exp == 0 {
        if mantissa == 0 {
            // Zero
            sign
        } else {
            // Subnormal - normalize it
            let mut m = mantissa;
            let mut e = -14i32;
            while (m & 0x0400) == 0 {
                m <<= 1;
                e -= 1;
            }
            m &= 0x03FF;
            let new_exp = ((e + 127) as u32) << 23;
            sign | new_exp | (m << 13)
        }
    } else {
        // Normal number
        let new_exp = ((exp - 15 + 127) as u32) << 23;
        sign | new_exp | (mantissa << 13)
    };

    f32::from_bits(bits)
}
