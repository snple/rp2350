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

/// Per-core guard: index 0 = Core0, index 1 = Core1.
///
/// The SIO CPUID register (offset 0, at SIO base 0xD000_0000) returns the
/// number of the core it is read from (0 or 1), so it can be used directly
/// as an index.
static TAKEN: [AtomicBool; 2] = [AtomicBool::new(false), AtomicBool::new(false)];

/// Read the number of the currently executing core (0 = Core0, 1 = Core1).
///
/// Reads the SIO CPUID register (address 0xD000_0000, read-only, no side
/// effects). The address is identical in both Secure and Non-Secure mappings
/// on RP2350.
#[inline(always)]
fn core_id() -> usize {
    // SIO base = 0xD000_0000, CPUID is at offset 0, returns 0 or 1.
    // SAFETY: this address is always valid on RP2350 and the access is read-only.
    unsafe { core::ptr::read_volatile(0xD000_0000usize as *const u32) as usize }
}

/// Core Peripherals container
///
/// Each core has its own PPB (Private Peripheral Bus), so this struct's
/// singleton is tracked **per core**: Core0 and Core1 can each call
/// [`Peripherals::take()`] once, independently of each other.
///
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
    /// Take the core peripherals singleton for the **calling core**.
    ///
    /// Since each RP2350 Cortex-M33 core has its own PPB, Core0 and Core1
    /// can each call this method successfully once. A second call from the
    /// same core returns `None`.
    ///
    /// Returns `Some(Peripherals)` on the first call per core, `None` on subsequent calls.
    ///
    /// # Example
    ///
    /// ```ignore
    /// // Both Core0 and Core1 can do this independently:
    /// let peripherals = Peripherals::take().unwrap();
    /// peripherals.SYST.set_reload(1000);
    /// ```
    #[inline]
    pub fn take() -> Option<Self> {
        if TAKEN[core_id()].swap(true, Ordering::AcqRel) {
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
            NVIC: unsafe { NVIC::steal() },
            SCB: unsafe { SCB::steal() },
            SYST: unsafe { SYST::steal() },
            MPU: unsafe { MPU::steal() },
            FPU: unsafe { FPU::steal() },
            SAU: unsafe { SAU::steal() },
        }
    }

    /// Release the peripherals back to the pool for the **calling core**.
    ///
    /// After calling this, `take()` will succeed again for this core.
    #[inline]
    pub fn release(self) {
        TAKEN[core_id()].store(false, Ordering::Release);
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

            /// Steal an instance of this peripheral
            ///
            /// # Safety
            ///
            /// Ensure that the new instance of the peripheral cannot be used in a way
            /// that may race with any existing instances.
            #[inline]
            pub unsafe fn steal() -> Self {
                Self {
                    _marker: PhantomData,
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

        impl core::fmt::Debug for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct(stringify!($name)).finish()
            }
        }
    };
}

pub(crate) use peripheral;

// =============================================================================
// Common Register Types
// =============================================================================

/// A volatile cell for memory-mapped register access.
///
/// Provides volatile read/write access to a memory location with proper
/// semantics for hardware registers.
#[repr(transparent)]
pub struct VolatileCell<T> {
    value: core::cell::UnsafeCell<T>,
}

impl<T> VolatileCell<T> {
    /// Create a new VolatileCell
    #[inline(always)]
    pub const fn new(value: T) -> Self {
        Self {
            value: core::cell::UnsafeCell::new(value),
        }
    }

    /// Get a raw pointer to the underlying value
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut T {
        self.value.get()
    }
}

impl<T: Copy> VolatileCell<T> {
    /// Read the value with volatile semantics
    #[inline(always)]
    pub fn read_bits(&self) -> T {
        unsafe { core::ptr::read_volatile(self.value.get()) }
    }

    /// Write the value with volatile semantics
    #[inline(always)]
    pub fn write_bits(&self, value: T) {
        unsafe { core::ptr::write_volatile(self.value.get(), value) }
    }

    /// Modify the value using a closure
    #[inline(always)]
    pub fn modify_bits<F: FnOnce(T) -> T>(&self, f: F) {
        self.write_bits(f(self.read_bits()));
    }
}
