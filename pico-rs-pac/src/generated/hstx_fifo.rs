//! HSTX_FIFO
//! FIFO status and write access for HSTX
//!
//! This peripheral supports atomic SET/CLR/XOR register aliases.

#![allow(dead_code)]

pub const BASE: u32 = 0x50600000;

/// STAT Register
///
/// FIFO status
///
/// Address offset: 0x0000
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **WOF** `[10]` (RW)
///
///   FIFO was written when full. Write 1 to clear.
///
/// - **EMPTY** `[9]` (RO)
///
/// - **FULL** `[8]` (RO)
///
/// - **LEVEL** `[7:0]` (RO)
#[repr(transparent)]
pub struct Stat {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Stat {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Stat {
    // ======== Field Constants ========

    /// **WOF**
    ///
    /// FIFO was written when full. Write 1 to clear.
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const WOF_OFFSET: u32 = 10;
    pub const WOF_MASK: u32 = 0x00000400;
    pub const WOF: u32 = 1 << 10;

    /// **EMPTY**
    /// Bits `[9:9]`, mask `0x00000200`
    pub const EMPTY_OFFSET: u32 = 9;
    pub const EMPTY_MASK: u32 = 0x00000200;
    pub const EMPTY: u32 = 1 << 9;

    /// **FULL**
    /// Bits `[8:8]`, mask `0x00000100`
    pub const FULL_OFFSET: u32 = 8;
    pub const FULL_MASK: u32 = 0x00000100;
    pub const FULL: u32 = 1 << 8;

    /// **LEVEL**
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const LEVEL_OFFSET: u32 = 0;
    pub const LEVEL_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **WOF** `[10]` (RW)
    ///
    /// FIFO was written when full. Write 1 to clear.
    #[inline(always)]
    pub fn wof(&self) -> bool {
        (self.value.read_bits() & Self::WOF_MASK) != 0
    }

    /// Set WOF bit atomically (RW)
    #[inline(always)]
    pub fn set_wof(&self) {
        self.set_bits(Self::WOF_MASK);
    }

    /// Clear WOF bit atomically (RW)
    #[inline(always)]
    pub fn clr_wof(&self) {
        self.clr_bits(Self::WOF_MASK);
    }

    /// **EMPTY** `[9]` (RO)
    #[inline(always)]
    pub fn empty(&self) -> bool {
        (self.value.read_bits() & Self::EMPTY_MASK) != 0
    }

    /// **FULL** `[8]` (RO)
    #[inline(always)]
    pub fn full(&self) -> bool {
        (self.value.read_bits() & Self::FULL_MASK) != 0
    }

    /// **LEVEL** `[7:0]` (RO)
    #[inline(always)]
    pub fn level(&self) -> u32 {
        (self.value.read_bits() & Self::LEVEL_MASK) >> Self::LEVEL_OFFSET
    }

}

/// STAT value type for batch read/modify/write operations
///
/// Use `stat.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `stat.write_val(val)`.
#[derive(Clone, Copy)]
pub struct StatVal {
    bits: u32,
}

impl StatVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **WOF** `[10:10]`
    ///
    /// FIFO was written when full. Write 1 to clear.
    #[inline(always)]
    pub const fn wof(&self) -> bool {
        (self.bits & Stat::WOF_MASK) != 0
    }

    /// Read **EMPTY** `[9:9]`
    #[inline(always)]
    pub const fn empty(&self) -> bool {
        (self.bits & Stat::EMPTY_MASK) != 0
    }

    /// Read **FULL** `[8:8]`
    #[inline(always)]
    pub const fn full(&self) -> bool {
        (self.bits & Stat::FULL_MASK) != 0
    }

    /// Read **LEVEL** `[7:0]`
    #[inline(always)]
    pub const fn level(&self) -> u32 {
        (self.bits & Stat::LEVEL_MASK) >> Stat::LEVEL_OFFSET
    }

    /// Set **WOF** `[10:10]`
    ///
    /// FIFO was written when full. Write 1 to clear.
    #[inline(always)]
    pub const fn with_wof(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Stat::WOF_MASK
            } else {
                self.bits & !Stat::WOF_MASK
            }
        }
    }

}

impl Default for StatVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Stat {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> StatVal {
        StatVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: StatVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(StatVal) -> StatVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// FIFO Register
///
/// Write access to FIFO
///
/// Address offset: 0x0004
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **FIFO** `[31:0]` (WO)
#[repr(transparent)]
pub struct Fifo {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Fifo {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Fifo {
    // ======== Field Constants ========

    /// **FIFO**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const FIFO_OFFSET: u32 = 0;
    pub const FIFO_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// Write **FIFO** `[31:0]` (WO)
    #[inline(always)]
    pub fn write_fifo(&self, value: u32) {
        self.value.write_bits((value << Self::FIFO_OFFSET) & Self::FIFO_MASK);
    }

}

/// FIFO value type for batch read/modify/write operations
///
/// Use `fifo.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `fifo.write_val(val)`.
#[derive(Clone, Copy)]
pub struct FifoVal {
    bits: u32,
}

impl FifoVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Set **FIFO** `[31:0]`
    #[inline(always)]
    pub const fn with_fifo(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Fifo::FIFO_MASK) | ((value << Fifo::FIFO_OFFSET) & Fifo::FIFO_MASK)
        }
    }

}

impl Default for FifoVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Fifo {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> FifoVal {
        FifoVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: FifoVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(FifoVal) -> FifoVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// FIFO status
    pub stat: Stat,
    /// 0x0004
    /// Write access to FIFO
    pub fifo: Fifo,
}

