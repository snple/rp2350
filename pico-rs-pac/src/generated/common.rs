//! Common types and utilities for register access

#![allow(dead_code)]

use core::cell::UnsafeCell;
use core::ptr::{read_volatile, write_volatile};

/// A volatile cell for memory-mapped register access.
///
/// Similar to `vcell::VolatileCell`, this provides volatile read/write access
/// to a memory location with proper semantics for hardware registers.
#[repr(transparent)]
pub struct VolatileCell<T> {
    value: UnsafeCell<T>,
}

unsafe impl<T: Send> Send for VolatileCell<T> {}
unsafe impl<T> Sync for VolatileCell<T> {}

impl<T> VolatileCell<T> {
    /// Create a new VolatileCell
    #[inline(always)]
    pub const fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
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
        unsafe { read_volatile(self.value.get()) }
    }

    /// Write the value with volatile semantics
    #[inline(always)]
    pub fn write_bits(&self, value: T) {
        unsafe { write_volatile(self.value.get(), value) }
    }

    /// Modify the value using a closure
    #[inline(always)]
    pub fn modify_bits<F: FnOnce(T) -> T>(&self, f: F) {
        self.write_bits(f(self.read_bits()));
    }
}

/// A volatile cell with atomic SET/CLR/XOR support.
///
/// This wrapper provides atomic bit manipulation via RP2350 hardware aliases.
/// Use this for registers in APB/AHB peripherals (0x40000000-0x50ffffff).
///
/// Atomic operations use hardware address aliases:
/// - SET: base + 0x2000 - sets bits that are 1
/// - CLR: base + 0x3000 - clears bits that are 1
/// - XOR: base + 0x1000 - toggles bits that are 1
#[repr(transparent)]
pub struct AtomicCell<T> {
    value: UnsafeCell<T>,
}

unsafe impl<T: Send> Send for AtomicCell<T> {}
unsafe impl<T> Sync for AtomicCell<T> {}

impl<T> AtomicCell<T> {
    /// Create a new AtomicCell
    #[inline(always)]
    pub const fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
        }
    }

    /// Get a raw pointer to the underlying value
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut T {
        self.value.get()
    }
}

impl<T: Copy> AtomicCell<T> {
    /// Read the value with volatile semantics
    #[inline(always)]
    pub fn read_bits(&self) -> T {
        unsafe { read_volatile(self.value.get()) }
    }

    /// Write the value with volatile semantics
    #[inline(always)]
    pub fn write_bits(&self, value: T) {
        unsafe { write_volatile(self.value.get(), value) }
    }

    /// Modify the value using a closure
    #[inline(always)]
    pub fn modify_bits<F: FnOnce(T) -> T>(&self, f: F) {
        self.write_bits(f(self.read_bits()));
    }
}

impl AtomicCell<u32> {
    /// Atomic SET - sets bits that are 1 (RP2350 hardware alias at +0x2000)
    ///
    /// This is a single-cycle atomic operation that only affects bits set to 1.
    #[inline(always)]
    pub fn set_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x2000) as *mut u32;
            write_volatile(addr, bits);
        }
    }

    /// Atomic CLR - clears bits that are 1 (RP2350 hardware alias at +0x3000)
    ///
    /// This is a single-cycle atomic operation that only affects bits set to 1.
    #[inline(always)]
    pub fn clr_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x3000) as *mut u32;
            write_volatile(addr, bits);
        }
    }

    /// Atomic XOR - toggles bits that are 1 (RP2350 hardware alias at +0x1000)
    ///
    /// This is a single-cycle atomic operation that only affects bits set to 1.
    #[inline(always)]
    pub fn xor_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x1000) as *mut u32;
            write_volatile(addr, bits);
        }
    }
}

/// Spin until condition is true
#[inline(always)]
pub fn wait_until<F: Fn() -> bool>(cond: F) {
    while !cond() {
        core::hint::spin_loop();
    }
}

/// Spin while condition is true
#[inline(always)]
pub fn wait_while<F: Fn() -> bool>(cond: F) {
    while cond() {
        core::hint::spin_loop();
    }
}

/// Hardware spinlock (SIO has 32 spinlocks)
#[derive(Clone, Copy)]
pub struct Spinlock(u8);

impl Spinlock {
    pub const fn new(index: u8) -> Self {
        Self(index)
    }

    #[inline(always)]
    pub fn try_lock(&self) -> bool {
        let addr = 0xd0000100 + (self.0 as u32) * 4;
        unsafe { read_volatile(addr as *const u32) != 0 }
    }

    #[inline(always)]
    pub fn unlock(&self) {
        let addr = 0xd0000100 + (self.0 as u32) * 4;
        unsafe { write_volatile(addr as *mut u32, 1) }
    }

    #[inline(always)]
    pub fn lock(&self) {
        while !self.try_lock() {
            core::hint::spin_loop();
        }
    }
}
