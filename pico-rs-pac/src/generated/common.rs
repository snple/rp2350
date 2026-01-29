//! Common types and utilities for register access

#![allow(dead_code)]

use core::ptr::{read_volatile, write_volatile};

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
