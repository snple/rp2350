//! Register access primitives
//!
//! Provides safe, ergonomic access to memory-mapped registers with support
//! for RP2350's atomic SET/CLR/XOR aliases.

use core::ptr::{read_volatile, write_volatile};

/// A register at a fixed address
#[derive(Clone, Copy)]
pub struct Reg {
    addr: u32,
}

impl Reg {
    /// Create a new register reference
    #[inline(always)]
    pub const fn new(addr: u32) -> Self {
        Self { addr }
    }

    /// Read the register value
    #[inline(always)]
    pub fn read(&self) -> u32 {
        unsafe { read_volatile(self.addr as *const u32) }
    }

    /// Write a value to the register
    #[inline(always)]
    pub fn write(&self, value: u32) {
        unsafe { write_volatile(self.addr as *mut u32, value) }
    }

    /// Modify the register using a closure
    #[inline(always)]
    pub fn modify<F: FnOnce(u32) -> u32>(&self, f: F) {
        self.write(f(self.read()));
    }

    /// Set bits using atomic SET alias (addr + 0x2000)
    #[inline(always)]
    pub fn set_bits(&self, bits: u32) {
        unsafe { write_volatile((self.addr + 0x2000) as *mut u32, bits) }
    }

    /// Clear bits using atomic CLR alias (addr + 0x3000)
    #[inline(always)]
    pub fn clear_bits(&self, bits: u32) {
        unsafe { write_volatile((self.addr + 0x3000) as *mut u32, bits) }
    }

    /// XOR bits using atomic XOR alias (addr + 0x1000)
    #[inline(always)]
    pub fn xor_bits(&self, bits: u32) {
        unsafe { write_volatile((self.addr + 0x1000) as *mut u32, bits) }
    }

    /// Check if specific bits are set
    #[inline(always)]
    pub fn is_set(&self, bits: u32) -> bool {
        (self.read() & bits) == bits
    }

    /// Check if any of the specific bits are set
    #[inline(always)]
    pub fn any_set(&self, bits: u32) -> bool {
        (self.read() & bits) != 0
    }

    /// Wait until specific bits are set
    #[inline(always)]
    pub fn wait_set(&self, bits: u32) {
        while !self.is_set(bits) {
            cortex_m::asm::nop();
        }
    }

    /// Wait until specific bits are cleared
    #[inline(always)]
    pub fn wait_clear(&self, bits: u32) {
        while self.any_set(bits) {
            cortex_m::asm::nop();
        }
    }
}

/// A read-only register
#[derive(Clone, Copy)]
pub struct RegRO {
    addr: u32,
}

impl RegRO {
    /// Create a new read-only register reference
    #[inline(always)]
    pub const fn new(addr: u32) -> Self {
        Self { addr }
    }

    /// Read the register value
    #[inline(always)]
    pub fn read(&self) -> u32 {
        unsafe { read_volatile(self.addr as *const u32) }
    }

    /// Check if specific bits are set
    #[inline(always)]
    pub fn is_set(&self, bits: u32) -> bool {
        (self.read() & bits) == bits
    }
}

/// A write-only register
#[derive(Clone, Copy)]
pub struct RegWO {
    addr: u32,
}

impl RegWO {
    /// Create a new write-only register reference
    #[inline(always)]
    pub const fn new(addr: u32) -> Self {
        Self { addr }
    }

    /// Write a value to the register
    #[inline(always)]
    pub fn write(&self, value: u32) {
        unsafe { write_volatile(self.addr as *mut u32, value) }
    }
}

/// Helper macro to define a register block
#[macro_export]
macro_rules! reg_block {
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident = $base:expr;
        $(
            $(#[$field_meta:meta])*
            $field_vis:vis $field:ident : $reg_type:ident = $offset:expr;
        )*
    ) => {
        $(#[$meta])*
        #[allow(non_snake_case)]
        $vis struct $name;

        impl $name {
            /// Base address of this peripheral
            pub const BASE: u32 = $base;

            $(
                $(#[$field_meta])*
                $field_vis const $field: $crate::regs::$reg_type = $crate::regs::$reg_type::new($base + $offset);
            )*
        }
    };
}
