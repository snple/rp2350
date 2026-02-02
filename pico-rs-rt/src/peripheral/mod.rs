//! Cortex-M33 Core Peripherals
//!
//! This module provides access to the core peripherals of the Cortex-M33:
//!
//! - **NVIC** - Nested Vectored Interrupt Controller
//! - **SCB** - System Control Block
//! - **SYST** - SysTick Timer
//! - **MPU** - Memory Protection Unit (ARMv8-M enhanced)
//! - **FPU** - Floating Point Unit (FPv5 with FP16 support)
//! - **SAU** - Security Attribution Unit (ARMv8-M TrustZone)
//!
//! # ARMv8-M Specific Features
//!
//! The RP2350's Cortex-M33 includes several ARMv8-M specific features:
//!
//! - TrustZone security extensions (SAU)
//! - Stack limit registers (MSPLIM, PSPLIM)
//! - Enhanced MPU with flexible region sizes
//! - FPv5 with half-precision (FP16) support

use core::sync::atomic::{AtomicBool, Ordering};

pub mod fpu;
pub mod mpu;
pub mod nvic;
pub mod sau;
pub mod scb;
pub mod syst;

// Re-export peripheral types
pub use fpu::FPU;
pub use mpu::MPU;
pub use nvic::NVIC;
pub use sau::SAU;
pub use scb::SCB;
pub use syst::SYST;

/// Guard to ensure peripherals are taken only once
static TAKEN: AtomicBool = AtomicBool::new(false);

/// Core Peripherals container
///
/// This struct provides singleton access to all core peripherals.
/// Use `Peripherals::take()` to get ownership.
#[allow(non_snake_case)]
pub struct Peripherals {
    /// Nested Vectored Interrupt Controller
    pub NVIC: NVIC,
    /// System Control Block
    pub SCB: SCB,
    /// SysTick Timer
    pub SYST: SYST,
    /// Memory Protection Unit
    pub MPU: MPU,
    /// Floating Point Unit
    pub FPU: FPU,
    /// Security Attribution Unit (TrustZone)
    pub SAU: SAU,
}

impl Peripherals {
    /// Take the core peripherals singleton
    ///
    /// Returns `Some(Peripherals)` on the first call, `None` on subsequent calls.
    /// This ensures that only one instance of the peripherals exists.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let peripherals = Peripherals::take().unwrap();
    /// peripherals.SYST.set_reload(1000);
    /// ```
    #[inline]
    pub fn take() -> Option<Self> {
        if TAKEN.swap(true, Ordering::AcqRel) {
            None
        } else {
            Some(unsafe { Self::steal() })
        }
    }

    /// Unsafely create an instance of the core peripherals
    ///
    /// # Safety
    ///
    /// This bypasses the singleton check. The caller must ensure that
    /// no other code has access to the peripherals to prevent data races.
    #[inline]
    pub unsafe fn steal() -> Self {
        Self {
            NVIC: NVIC::new(),
            SCB: SCB::new(),
            SYST: SYST::new(),
            MPU: MPU::new(),
            FPU: FPU::new(),
            SAU: SAU::new(),
        }
    }

    /// Release the peripherals back to the pool
    ///
    /// After calling this, `take()` will succeed again.
    #[inline]
    pub fn release(self) {
        TAKEN.store(false, Ordering::Release);
    }
}

// =============================================================================
// Peripheral Base Addresses (from ARM Cortex-M33 TRM)
// =============================================================================

/// NVIC base address
pub const NVIC_BASE: u32 = 0xE000_E100;

/// SCB base address
pub const SCB_BASE: u32 = 0xE000_ED00;

/// SysTick base address
pub const SYST_BASE: u32 = 0xE000_E010;

/// DCB base address
pub const DCB_BASE: u32 = 0xE000_EDF0;

/// DWT base address
pub const DWT_BASE: u32 = 0xE000_1000;

/// MPU base address
pub const MPU_BASE: u32 = 0xE000_ED90;

/// FPU base address (Coprocessor Access Control)
pub const CPACR_BASE: u32 = 0xE000_ED88;

/// SAU base address (Security Attribution Unit)
pub const SAU_BASE: u32 = 0xE000_EDD0;

// =============================================================================
// Peripheral Wrapper Macro
// =============================================================================

/// Helper macro to create peripheral wrappers
macro_rules! peripheral {
    ($name:ident, $reg:ty, $base:expr) => {
        impl $name {
            /// Pointer to the peripheral's register block
            pub const PTR: *const $reg = $base as *const $reg;

            /// Create a new instance (internal use only)
            #[inline(always)]
            pub(crate) const fn new() -> Self {
                Self {
                    _marker: core::marker::PhantomData,
                }
            }

            /// Returns a raw pointer to the register block
            #[inline(always)]
            pub const fn ptr() -> *const $reg {
                Self::PTR
            }
        }

        impl core::ops::Deref for $name {
            type Target = $reg;

            #[inline(always)]
            fn deref(&self) -> &Self::Target {
                unsafe { &*Self::PTR }
            }
        }
    };
}

pub(crate) use peripheral;

// =============================================================================
// Common Register Types
// =============================================================================

/// 32-bit read/write register
#[repr(transparent)]
pub struct Reg32 {
    value: core::cell::UnsafeCell<u32>,
}

impl Reg32 {
    /// Read the register
    #[inline(always)]
    pub fn read(&self) -> u32 {
        unsafe { core::ptr::read_volatile(self.value.get()) }
    }

    /// Write the register
    #[inline(always)]
    pub fn write(&self, value: u32) {
        unsafe { core::ptr::write_volatile(self.value.get(), value) }
    }

    /// Modify the register with a closure
    #[inline(always)]
    pub fn modify<F>(&self, f: F)
    where
        F: FnOnce(u32) -> u32,
    {
        let old = self.read();
        self.write(f(old));
    }
}

/// 8-bit read/write register
#[repr(transparent)]
pub struct Reg8 {
    value: core::cell::UnsafeCell<u8>,
}

impl Reg8 {
    /// Read the register
    #[inline(always)]
    pub fn read(&self) -> u8 {
        unsafe { core::ptr::read_volatile(self.value.get()) }
    }

    /// Write the register
    #[inline(always)]
    pub fn write(&self, value: u8) {
        unsafe { core::ptr::write_volatile(self.value.get(), value) }
    }
}
