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

/// Per-core guard: index 0 = Core0，index 1 = Core1。
///
/// SIO CPUID 寄存器（偏移 0，位于 SIO 基址 0xD000_0000）在哪个核心
/// 上读取就返回该核心的编号（0 或 1），因此可直接用它做索引。
static TAKEN: [AtomicBool; 2] = [AtomicBool::new(false), AtomicBool::new(false)];

/// 读取当前执行核心的编号（0 = Core0，1 = Core1）。
///
/// 直接读取 SIO CPUID 寄存器（地址 0xD000_0000，只读，无副作用）。
/// 该寄存器在 RP2350 Secure/Non-Secure 两种映射下地址相同。
#[inline(always)]
fn core_id() -> usize {
    // SIO base = 0xD000_0000，CPUID 在偏移 0 处，返回值为 0 或 1。
    // SAFETY: 该地址在 RP2350 上始终有效，且此为只读寄存器。
    unsafe { core::ptr::read_volatile(0xD000_0000usize as *const u32) as usize }
}

/// Core Peripherals container
///
/// 每个核心各持有独立的 PPB（Private Peripheral Bus），因此本结构体
/// 的 singleton 是**按核心**独立跟踪的：Core0 和 Core1 可以各自
/// 调用 [`Peripherals::take()`] 一次，互不干扰。
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
    /// 由于 RP2350 每个 Cortex-M33 核心拥有独立的 PPB，Core0 和 Core1
    /// 可以各自成功调用本方法一次。同一核心的第二次调用将返回 `None`。
    ///
    /// Returns `Some(Peripherals)` on the first call per core, `None` on subsequent calls.
    ///
    /// # Example
    ///
    /// ```ignore
    /// // Core0 和 Core1 均可独立执行：
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
