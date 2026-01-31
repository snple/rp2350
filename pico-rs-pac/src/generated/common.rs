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

impl VolatileCell<u32> {
    /// Read the value with volatile semantics
    #[inline(always)]
    pub fn read_bits(&self) -> u32 {
        unsafe { read_volatile(self.value.get()) }
    }

    /// Write the value with volatile semantics
    #[inline(always)]
    pub fn write_bits(&self, value: u32) {
        unsafe { write_volatile(self.value.get(), value) }
    }

    /// Modify the value using a closure
    #[inline(always)]
    pub fn modify_bits<F: FnOnce(u32) -> u32>(&self, f: F) {
        self.write_bits(f(self.read_bits()));
    }

    /// Atomic SET - sets bits that are 1 (RP2350 hardware alias at +0x2000)
    #[inline(always)]
    pub fn set_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x2000) as *mut u32;
            write_volatile(addr, bits);
        }
    }

    /// Atomic CLR - clears bits that are 1 (RP2350 hardware alias at +0x3000)
    #[inline(always)]
    pub fn clr_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x3000) as *mut u32;
            write_volatile(addr, bits);
        }
    }

    /// Atomic XOR - toggles bits that are 1 (RP2350 hardware alias at +0x1000)
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
