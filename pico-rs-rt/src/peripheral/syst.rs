//! SysTick Timer
//!
//! The SysTick timer is a 24-bit down-counting timer built into all Cortex-M
//! processors. It's typically used for OS tick generation or simple delays.
//!
//! # Features
//!
//! - 24-bit reload value (max 0x00FFFFFF = 16,777,215)
//! - Optional interrupt on reaching zero
//! - Choice of processor clock or external reference clock
//!
//! # Example
//!
//! ```ignore
//! use pico_rs_rt::peripheral::SYST;
//!
//! let mut syst = SYST::take();
//!
//! // Configure for 1ms tick at 150 MHz
//! syst.set_reload(150_000 - 1);
//! syst.clear_current();
//! syst.enable_interrupt();
//! syst.enable_counter();
//! ```

use core::marker::PhantomData;

use super::{Reg32, SYST_BASE, peripheral};

/// SysTick Register Block
#[repr(C)]
pub struct RegisterBlock {
    /// Control and Status Register
    pub csr: Reg32,
    /// Reload Value Register
    pub rvr: Reg32,
    /// Current Value Register
    pub cvr: Reg32,
    /// Calibration Value Register (read-only)
    pub calib: Reg32,
}

/// SysTick Timer peripheral
pub struct SYST {
    _marker: PhantomData<*const ()>,
}

peripheral!(SYST, RegisterBlock, SYST_BASE);

unsafe impl Send for SYST {}

// =============================================================================
// Constants
// =============================================================================

/// Maximum reload value (24-bit)
pub const MAX_RELOAD: u32 = 0x00FF_FFFF;

/// CSR bit: Enable counter
const CSR_ENABLE: u32 = 1 << 0;
/// CSR bit: Enable interrupt
const CSR_TICKINT: u32 = 1 << 1;
/// CSR bit: Clock source (1 = processor, 0 = external)
const CSR_CLKSOURCE: u32 = 1 << 2;
/// CSR bit: Count flag (set when counter reaches 0)
const CSR_COUNTFLAG: u32 = 1 << 16;

/// Calibration bit: SKEW (calibration value is not exactly 10ms)
const CALIB_SKEW: u32 = 1 << 30;
/// Calibration bit: NOREF (no external reference clock)
const CALIB_NOREF: u32 = 1 << 31;

// =============================================================================
// Clock Source
// =============================================================================

/// SysTick clock source
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SystClkSource {
    /// Processor clock (typically fastest)
    Core,
    /// External reference clock (device-specific)
    External,
}

// =============================================================================
// Implementation
// =============================================================================

impl SYST {
    /// Enable the counter
    #[inline]
    pub fn enable_counter(&mut self) {
        unsafe {
            (*Self::PTR).csr.modify(|v| v | CSR_ENABLE);
        }
    }

    /// Disable the counter
    #[inline]
    pub fn disable_counter(&mut self) {
        unsafe {
            (*Self::PTR).csr.modify(|v| v & !CSR_ENABLE);
        }
    }

    /// Check if the counter is enabled
    #[inline]
    pub fn is_counter_enabled(&self) -> bool {
        unsafe { (*Self::PTR).csr.read() & CSR_ENABLE != 0 }
    }

    /// Enable the SysTick interrupt
    #[inline]
    pub fn enable_interrupt(&mut self) {
        unsafe {
            (*Self::PTR).csr.modify(|v| v | CSR_TICKINT);
        }
    }

    /// Disable the SysTick interrupt
    #[inline]
    pub fn disable_interrupt(&mut self) {
        unsafe {
            (*Self::PTR).csr.modify(|v| v & !CSR_TICKINT);
        }
    }

    /// Check if the interrupt is enabled
    #[inline]
    pub fn is_interrupt_enabled(&self) -> bool {
        unsafe { (*Self::PTR).csr.read() & CSR_TICKINT != 0 }
    }

    /// Set the clock source
    #[inline]
    pub fn set_clock_source(&mut self, source: SystClkSource) {
        unsafe {
            match source {
                SystClkSource::Core => (*Self::PTR).csr.modify(|v| v | CSR_CLKSOURCE),
                SystClkSource::External => (*Self::PTR).csr.modify(|v| v & !CSR_CLKSOURCE),
            }
        }
    }

    /// Get the current clock source
    #[inline]
    pub fn clock_source(&self) -> SystClkSource {
        if unsafe { (*Self::PTR).csr.read() & CSR_CLKSOURCE != 0 } {
            SystClkSource::Core
        } else {
            SystClkSource::External
        }
    }

    /// Set the reload value
    ///
    /// The counter will reload to this value when it reaches zero.
    /// Maximum value is 0x00FFFFFF (24-bit).
    #[inline]
    pub fn set_reload(&mut self, value: u32) {
        unsafe { (*Self::PTR).rvr.write(value.min(MAX_RELOAD)) };
    }

    /// Get the reload value
    #[inline]
    pub fn get_reload() -> u32 {
        unsafe { (*Self::PTR).rvr.read() & MAX_RELOAD }
    }

    /// Clear the current value
    ///
    /// Writing any value to CVR clears it to zero and clears the COUNTFLAG.
    #[inline]
    pub fn clear_current(&mut self) {
        unsafe { (*Self::PTR).cvr.write(0) };
    }

    /// Get the current counter value
    #[inline]
    pub fn get_current() -> u32 {
        unsafe { (*Self::PTR).cvr.read() & MAX_RELOAD }
    }

    /// Check if the counter has wrapped (reached zero) since last read
    ///
    /// Reading this flag clears it.
    #[inline]
    pub fn has_wrapped(&mut self) -> bool {
        unsafe { (*Self::PTR).csr.read() & CSR_COUNTFLAG != 0 }
    }

    /// Get the calibration value (ticks per 10ms, if available)
    ///
    /// Returns `None` if the calibration value is not available or not accurate.
    #[inline]
    pub fn get_ticks_per_10ms() -> Option<u32> {
        let calib = unsafe { (*Self::PTR).calib.read() };
        let tenms = calib & 0x00FF_FFFF;

        if tenms == 0 || calib & CALIB_NOREF != 0 {
            None
        } else {
            Some(tenms)
        }
    }

    /// Check if calibration value is exactly 10ms
    ///
    /// If `false`, the calibration value might be slightly off.
    #[inline]
    pub fn is_calibration_precise() -> bool {
        unsafe { (*Self::PTR).calib.read() & CALIB_SKEW == 0 }
    }

    /// Check if external reference clock is available
    #[inline]
    pub fn has_reference_clock() -> bool {
        unsafe { (*Self::PTR).calib.read() & CALIB_NOREF == 0 }
    }
}
