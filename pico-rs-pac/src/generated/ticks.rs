//! TICKS
//!
//! This peripheral supports atomic SET/CLR/XOR register aliases.

#![allow(dead_code)]

pub const BASE: u32 = 0x40108000;

/// PROC0_CTRL Register
///
/// Controls the tick generator
///
/// Address offset: 0x0000
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **RUNNING** `[1]` (RO)
///
///   Is the tick generator running?
///
/// - **ENABLE** `[0]` (RW)
///
///   start / stop tick generation
#[repr(transparent)]
pub struct Proc0Ctrl {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Proc0Ctrl {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Proc0Ctrl {
    // ======== Field Constants ========

    /// **RUNNING**
    ///
    /// Is the tick generator running?
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const RUNNING_OFFSET: u32 = 1;
    pub const RUNNING_MASK: u32 = 0x00000002;
    pub const RUNNING: u32 = 1 << 1;

    /// **ENABLE**
    ///
    /// start / stop tick generation
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const ENABLE_OFFSET: u32 = 0;
    pub const ENABLE_MASK: u32 = 0x00000001;
    pub const ENABLE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **RUNNING** `[1]` (RO)
    ///
    /// Is the tick generator running?
    #[inline(always)]
    pub fn running(&self) -> bool {
        (self.value.read_bits() & Self::RUNNING_MASK) != 0
    }

    /// **ENABLE** `[0]` (RW)
    ///
    /// start / stop tick generation
    #[inline(always)]
    pub fn enable(&self) -> bool {
        (self.value.read_bits() & Self::ENABLE_MASK) != 0
    }

    /// Set ENABLE bit atomically (RW)
    #[inline(always)]
    pub fn set_enable(&self) {
        self.set_bits(Self::ENABLE_MASK);
    }

    /// Clear ENABLE bit atomically (RW)
    #[inline(always)]
    pub fn clr_enable(&self) {
        self.clr_bits(Self::ENABLE_MASK);
    }
}

/// PROC0_CTRL value type for batch read/modify/write operations
///
/// Use `proc0ctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `proc0ctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Proc0CtrlVal {
    bits: u32,
}

impl Proc0CtrlVal {
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

    /// Read **RUNNING** `[1:1]`
    ///
    /// Is the tick generator running?
    #[inline(always)]
    pub const fn running(&self) -> bool {
        (self.bits & Proc0Ctrl::RUNNING_MASK) != 0
    }

    /// Read **ENABLE** `[0:0]`
    ///
    /// start / stop tick generation
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        (self.bits & Proc0Ctrl::ENABLE_MASK) != 0
    }

    /// Set **ENABLE** `[0:0]`
    ///
    /// start / stop tick generation
    #[inline(always)]
    pub const fn with_enable(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc0Ctrl::ENABLE_MASK
            } else {
                self.bits & !Proc0Ctrl::ENABLE_MASK
            },
        }
    }
}

impl Default for Proc0CtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Proc0Ctrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Proc0CtrlVal {
        Proc0CtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Proc0CtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Proc0CtrlVal) -> Proc0CtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PROC0_CYCLES Register
///
/// Address offset: 0x0004
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **PROC0_CYCLES** `[8:0]` (RW)
///
///   Total number of clk_tick cycles before the next tick.
#[repr(transparent)]
pub struct Proc0Cycles {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Proc0Cycles {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Proc0Cycles {
    // ======== Field Constants ========

    /// **PROC0_CYCLES**
    ///
    /// Total number of clk_tick cycles before the next tick.
    ///
    /// Bits `[8:0]`, mask `0x000001ff`
    pub const PROC0_CYCLES_OFFSET: u32 = 0;
    pub const PROC0_CYCLES_MASK: u32 = 0x000001ff;

    // ======== Field Accessors ========

    /// **PROC0_CYCLES** `[8:0]` (RW)
    ///
    /// Total number of clk_tick cycles before the next tick.
    #[inline(always)]
    pub fn proc0_cycles(&self) -> u32 {
        (self.value.read_bits() & Self::PROC0_CYCLES_MASK) >> Self::PROC0_CYCLES_OFFSET
    }

    /// Write **PROC0_CYCLES** `[8:0]` (RW)
    ///
    /// Total number of clk_tick cycles before the next tick.
    #[inline(always)]
    pub fn write_proc0_cycles(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PROC0_CYCLES_MASK)
                | ((value << Self::PROC0_CYCLES_OFFSET) & Self::PROC0_CYCLES_MASK)
        });
    }
}

/// PROC0_CYCLES value type for batch read/modify/write operations
///
/// Use `proc0cycles.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `proc0cycles.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Proc0CyclesVal {
    bits: u32,
}

impl Proc0CyclesVal {
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

    /// Read **PROC0_CYCLES** `[8:0]`
    ///
    /// Total number of clk_tick cycles before the next tick.
    #[inline(always)]
    pub const fn proc0_cycles(&self) -> u32 {
        (self.bits & Proc0Cycles::PROC0_CYCLES_MASK) >> Proc0Cycles::PROC0_CYCLES_OFFSET
    }

    /// Set **PROC0_CYCLES** `[8:0]`
    ///
    /// Total number of clk_tick cycles before the next tick.
    #[inline(always)]
    pub const fn with_proc0_cycles(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Proc0Cycles::PROC0_CYCLES_MASK)
                | ((value << Proc0Cycles::PROC0_CYCLES_OFFSET) & Proc0Cycles::PROC0_CYCLES_MASK),
        }
    }
}

impl Default for Proc0CyclesVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Proc0Cycles {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Proc0CyclesVal {
        Proc0CyclesVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Proc0CyclesVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Proc0CyclesVal) -> Proc0CyclesVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PROC0_COUNT Register
///
/// Address offset: 0x0008
///
/// ## Fields
///
/// - **PROC0_COUNT** `[8:0]` (RO)
///
///   Count down timer: the remaining number clk_tick cycles before the next tick is generated.
#[repr(transparent)]
pub struct Proc0Count {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Proc0Count {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Proc0Count {
    // ======== Field Constants ========

    /// **PROC0_COUNT**
    ///
    /// Count down timer: the remaining number clk_tick cycles before the next tick is generated.
    ///
    /// Bits `[8:0]`, mask `0x000001ff`
    pub const PROC0_COUNT_OFFSET: u32 = 0;
    pub const PROC0_COUNT_MASK: u32 = 0x000001ff;

    // ======== Field Accessors ========

    /// **PROC0_COUNT** `[8:0]` (RO)
    ///
    /// Count down timer: the remaining number clk_tick cycles before the next tick is generated.
    #[inline(always)]
    pub fn proc0_count(&self) -> u32 {
        (self.value.read_bits() & Self::PROC0_COUNT_MASK) >> Self::PROC0_COUNT_OFFSET
    }
}

/// PROC0_COUNT value type for batch read/modify/write operations
///
/// Use `proc0count.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `proc0count.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Proc0CountVal {
    bits: u32,
}

impl Proc0CountVal {
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

    /// Read **PROC0_COUNT** `[8:0]`
    ///
    /// Count down timer: the remaining number clk_tick cycles before the next tick is generated.
    #[inline(always)]
    pub const fn proc0_count(&self) -> u32 {
        (self.bits & Proc0Count::PROC0_COUNT_MASK) >> Proc0Count::PROC0_COUNT_OFFSET
    }
}

impl Default for Proc0CountVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Proc0Count {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Proc0CountVal {
        Proc0CountVal::from_bits(self.value.read_bits())
    }
}

/// PROC1_CTRL Register
///
/// Controls the tick generator
///
/// Address offset: 0x000c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **RUNNING** `[1]` (RO)
///
///   Is the tick generator running?
///
/// - **ENABLE** `[0]` (RW)
///
///   start / stop tick generation
#[repr(transparent)]
pub struct Proc1Ctrl {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Proc1Ctrl {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Proc1Ctrl {
    // ======== Field Constants ========

    /// **RUNNING**
    ///
    /// Is the tick generator running?
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const RUNNING_OFFSET: u32 = 1;
    pub const RUNNING_MASK: u32 = 0x00000002;
    pub const RUNNING: u32 = 1 << 1;

    /// **ENABLE**
    ///
    /// start / stop tick generation
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const ENABLE_OFFSET: u32 = 0;
    pub const ENABLE_MASK: u32 = 0x00000001;
    pub const ENABLE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **RUNNING** `[1]` (RO)
    ///
    /// Is the tick generator running?
    #[inline(always)]
    pub fn running(&self) -> bool {
        (self.value.read_bits() & Self::RUNNING_MASK) != 0
    }

    /// **ENABLE** `[0]` (RW)
    ///
    /// start / stop tick generation
    #[inline(always)]
    pub fn enable(&self) -> bool {
        (self.value.read_bits() & Self::ENABLE_MASK) != 0
    }

    /// Set ENABLE bit atomically (RW)
    #[inline(always)]
    pub fn set_enable(&self) {
        self.set_bits(Self::ENABLE_MASK);
    }

    /// Clear ENABLE bit atomically (RW)
    #[inline(always)]
    pub fn clr_enable(&self) {
        self.clr_bits(Self::ENABLE_MASK);
    }
}

/// PROC1_CTRL value type for batch read/modify/write operations
///
/// Use `proc1ctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `proc1ctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Proc1CtrlVal {
    bits: u32,
}

impl Proc1CtrlVal {
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

    /// Read **RUNNING** `[1:1]`
    ///
    /// Is the tick generator running?
    #[inline(always)]
    pub const fn running(&self) -> bool {
        (self.bits & Proc1Ctrl::RUNNING_MASK) != 0
    }

    /// Read **ENABLE** `[0:0]`
    ///
    /// start / stop tick generation
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        (self.bits & Proc1Ctrl::ENABLE_MASK) != 0
    }

    /// Set **ENABLE** `[0:0]`
    ///
    /// start / stop tick generation
    #[inline(always)]
    pub const fn with_enable(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc1Ctrl::ENABLE_MASK
            } else {
                self.bits & !Proc1Ctrl::ENABLE_MASK
            },
        }
    }
}

impl Default for Proc1CtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Proc1Ctrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Proc1CtrlVal {
        Proc1CtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Proc1CtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Proc1CtrlVal) -> Proc1CtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PROC1_CYCLES Register
///
/// Address offset: 0x0010
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **PROC1_CYCLES** `[8:0]` (RW)
///
///   Total number of clk_tick cycles before the next tick.
#[repr(transparent)]
pub struct Proc1Cycles {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Proc1Cycles {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Proc1Cycles {
    // ======== Field Constants ========

    /// **PROC1_CYCLES**
    ///
    /// Total number of clk_tick cycles before the next tick.
    ///
    /// Bits `[8:0]`, mask `0x000001ff`
    pub const PROC1_CYCLES_OFFSET: u32 = 0;
    pub const PROC1_CYCLES_MASK: u32 = 0x000001ff;

    // ======== Field Accessors ========

    /// **PROC1_CYCLES** `[8:0]` (RW)
    ///
    /// Total number of clk_tick cycles before the next tick.
    #[inline(always)]
    pub fn proc1_cycles(&self) -> u32 {
        (self.value.read_bits() & Self::PROC1_CYCLES_MASK) >> Self::PROC1_CYCLES_OFFSET
    }

    /// Write **PROC1_CYCLES** `[8:0]` (RW)
    ///
    /// Total number of clk_tick cycles before the next tick.
    #[inline(always)]
    pub fn write_proc1_cycles(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PROC1_CYCLES_MASK)
                | ((value << Self::PROC1_CYCLES_OFFSET) & Self::PROC1_CYCLES_MASK)
        });
    }
}

/// PROC1_CYCLES value type for batch read/modify/write operations
///
/// Use `proc1cycles.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `proc1cycles.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Proc1CyclesVal {
    bits: u32,
}

impl Proc1CyclesVal {
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

    /// Read **PROC1_CYCLES** `[8:0]`
    ///
    /// Total number of clk_tick cycles before the next tick.
    #[inline(always)]
    pub const fn proc1_cycles(&self) -> u32 {
        (self.bits & Proc1Cycles::PROC1_CYCLES_MASK) >> Proc1Cycles::PROC1_CYCLES_OFFSET
    }

    /// Set **PROC1_CYCLES** `[8:0]`
    ///
    /// Total number of clk_tick cycles before the next tick.
    #[inline(always)]
    pub const fn with_proc1_cycles(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Proc1Cycles::PROC1_CYCLES_MASK)
                | ((value << Proc1Cycles::PROC1_CYCLES_OFFSET) & Proc1Cycles::PROC1_CYCLES_MASK),
        }
    }
}

impl Default for Proc1CyclesVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Proc1Cycles {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Proc1CyclesVal {
        Proc1CyclesVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Proc1CyclesVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Proc1CyclesVal) -> Proc1CyclesVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PROC1_COUNT Register
///
/// Address offset: 0x0014
///
/// ## Fields
///
/// - **PROC1_COUNT** `[8:0]` (RO)
///
///   Count down timer: the remaining number clk_tick cycles before the next tick is generated.
#[repr(transparent)]
pub struct Proc1Count {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Proc1Count {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Proc1Count {
    // ======== Field Constants ========

    /// **PROC1_COUNT**
    ///
    /// Count down timer: the remaining number clk_tick cycles before the next tick is generated.
    ///
    /// Bits `[8:0]`, mask `0x000001ff`
    pub const PROC1_COUNT_OFFSET: u32 = 0;
    pub const PROC1_COUNT_MASK: u32 = 0x000001ff;

    // ======== Field Accessors ========

    /// **PROC1_COUNT** `[8:0]` (RO)
    ///
    /// Count down timer: the remaining number clk_tick cycles before the next tick is generated.
    #[inline(always)]
    pub fn proc1_count(&self) -> u32 {
        (self.value.read_bits() & Self::PROC1_COUNT_MASK) >> Self::PROC1_COUNT_OFFSET
    }
}

/// PROC1_COUNT value type for batch read/modify/write operations
///
/// Use `proc1count.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `proc1count.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Proc1CountVal {
    bits: u32,
}

impl Proc1CountVal {
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

    /// Read **PROC1_COUNT** `[8:0]`
    ///
    /// Count down timer: the remaining number clk_tick cycles before the next tick is generated.
    #[inline(always)]
    pub const fn proc1_count(&self) -> u32 {
        (self.bits & Proc1Count::PROC1_COUNT_MASK) >> Proc1Count::PROC1_COUNT_OFFSET
    }
}

impl Default for Proc1CountVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Proc1Count {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Proc1CountVal {
        Proc1CountVal::from_bits(self.value.read_bits())
    }
}

/// TIMER0_CTRL Register
///
/// Controls the tick generator
///
/// Address offset: 0x0018
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **RUNNING** `[1]` (RO)
///
///   Is the tick generator running?
///
/// - **ENABLE** `[0]` (RW)
///
///   start / stop tick generation
#[repr(transparent)]
pub struct Timer0Ctrl {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Timer0Ctrl {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Timer0Ctrl {
    // ======== Field Constants ========

    /// **RUNNING**
    ///
    /// Is the tick generator running?
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const RUNNING_OFFSET: u32 = 1;
    pub const RUNNING_MASK: u32 = 0x00000002;
    pub const RUNNING: u32 = 1 << 1;

    /// **ENABLE**
    ///
    /// start / stop tick generation
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const ENABLE_OFFSET: u32 = 0;
    pub const ENABLE_MASK: u32 = 0x00000001;
    pub const ENABLE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **RUNNING** `[1]` (RO)
    ///
    /// Is the tick generator running?
    #[inline(always)]
    pub fn running(&self) -> bool {
        (self.value.read_bits() & Self::RUNNING_MASK) != 0
    }

    /// **ENABLE** `[0]` (RW)
    ///
    /// start / stop tick generation
    #[inline(always)]
    pub fn enable(&self) -> bool {
        (self.value.read_bits() & Self::ENABLE_MASK) != 0
    }

    /// Set ENABLE bit atomically (RW)
    #[inline(always)]
    pub fn set_enable(&self) {
        self.set_bits(Self::ENABLE_MASK);
    }

    /// Clear ENABLE bit atomically (RW)
    #[inline(always)]
    pub fn clr_enable(&self) {
        self.clr_bits(Self::ENABLE_MASK);
    }
}

/// TIMER0_CTRL value type for batch read/modify/write operations
///
/// Use `timer0ctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `timer0ctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Timer0CtrlVal {
    bits: u32,
}

impl Timer0CtrlVal {
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

    /// Read **RUNNING** `[1:1]`
    ///
    /// Is the tick generator running?
    #[inline(always)]
    pub const fn running(&self) -> bool {
        (self.bits & Timer0Ctrl::RUNNING_MASK) != 0
    }

    /// Read **ENABLE** `[0:0]`
    ///
    /// start / stop tick generation
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        (self.bits & Timer0Ctrl::ENABLE_MASK) != 0
    }

    /// Set **ENABLE** `[0:0]`
    ///
    /// start / stop tick generation
    #[inline(always)]
    pub const fn with_enable(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Timer0Ctrl::ENABLE_MASK
            } else {
                self.bits & !Timer0Ctrl::ENABLE_MASK
            },
        }
    }
}

impl Default for Timer0CtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Timer0Ctrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Timer0CtrlVal {
        Timer0CtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Timer0CtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Timer0CtrlVal) -> Timer0CtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TIMER0_CYCLES Register
///
/// Address offset: 0x001c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **TIMER0_CYCLES** `[8:0]` (RW)
///
///   Total number of clk_tick cycles before the next tick.
#[repr(transparent)]
pub struct Timer0Cycles {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Timer0Cycles {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Timer0Cycles {
    // ======== Field Constants ========

    /// **TIMER0_CYCLES**
    ///
    /// Total number of clk_tick cycles before the next tick.
    ///
    /// Bits `[8:0]`, mask `0x000001ff`
    pub const TIMER0_CYCLES_OFFSET: u32 = 0;
    pub const TIMER0_CYCLES_MASK: u32 = 0x000001ff;

    // ======== Field Accessors ========

    /// **TIMER0_CYCLES** `[8:0]` (RW)
    ///
    /// Total number of clk_tick cycles before the next tick.
    #[inline(always)]
    pub fn timer0_cycles(&self) -> u32 {
        (self.value.read_bits() & Self::TIMER0_CYCLES_MASK) >> Self::TIMER0_CYCLES_OFFSET
    }

    /// Write **TIMER0_CYCLES** `[8:0]` (RW)
    ///
    /// Total number of clk_tick cycles before the next tick.
    #[inline(always)]
    pub fn write_timer0_cycles(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TIMER0_CYCLES_MASK)
                | ((value << Self::TIMER0_CYCLES_OFFSET) & Self::TIMER0_CYCLES_MASK)
        });
    }
}

/// TIMER0_CYCLES value type for batch read/modify/write operations
///
/// Use `timer0cycles.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `timer0cycles.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Timer0CyclesVal {
    bits: u32,
}

impl Timer0CyclesVal {
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

    /// Read **TIMER0_CYCLES** `[8:0]`
    ///
    /// Total number of clk_tick cycles before the next tick.
    #[inline(always)]
    pub const fn timer0_cycles(&self) -> u32 {
        (self.bits & Timer0Cycles::TIMER0_CYCLES_MASK) >> Timer0Cycles::TIMER0_CYCLES_OFFSET
    }

    /// Set **TIMER0_CYCLES** `[8:0]`
    ///
    /// Total number of clk_tick cycles before the next tick.
    #[inline(always)]
    pub const fn with_timer0_cycles(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Timer0Cycles::TIMER0_CYCLES_MASK)
                | ((value << Timer0Cycles::TIMER0_CYCLES_OFFSET)
                    & Timer0Cycles::TIMER0_CYCLES_MASK),
        }
    }
}

impl Default for Timer0CyclesVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Timer0Cycles {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Timer0CyclesVal {
        Timer0CyclesVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Timer0CyclesVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Timer0CyclesVal) -> Timer0CyclesVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TIMER0_COUNT Register
///
/// Address offset: 0x0020
///
/// ## Fields
///
/// - **TIMER0_COUNT** `[8:0]` (RO)
///
///   Count down timer: the remaining number clk_tick cycles before the next tick is generated.
#[repr(transparent)]
pub struct Timer0Count {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Timer0Count {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Timer0Count {
    // ======== Field Constants ========

    /// **TIMER0_COUNT**
    ///
    /// Count down timer: the remaining number clk_tick cycles before the next tick is generated.
    ///
    /// Bits `[8:0]`, mask `0x000001ff`
    pub const TIMER0_COUNT_OFFSET: u32 = 0;
    pub const TIMER0_COUNT_MASK: u32 = 0x000001ff;

    // ======== Field Accessors ========

    /// **TIMER0_COUNT** `[8:0]` (RO)
    ///
    /// Count down timer: the remaining number clk_tick cycles before the next tick is generated.
    #[inline(always)]
    pub fn timer0_count(&self) -> u32 {
        (self.value.read_bits() & Self::TIMER0_COUNT_MASK) >> Self::TIMER0_COUNT_OFFSET
    }
}

/// TIMER0_COUNT value type for batch read/modify/write operations
///
/// Use `timer0count.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `timer0count.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Timer0CountVal {
    bits: u32,
}

impl Timer0CountVal {
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

    /// Read **TIMER0_COUNT** `[8:0]`
    ///
    /// Count down timer: the remaining number clk_tick cycles before the next tick is generated.
    #[inline(always)]
    pub const fn timer0_count(&self) -> u32 {
        (self.bits & Timer0Count::TIMER0_COUNT_MASK) >> Timer0Count::TIMER0_COUNT_OFFSET
    }
}

impl Default for Timer0CountVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Timer0Count {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Timer0CountVal {
        Timer0CountVal::from_bits(self.value.read_bits())
    }
}

/// TIMER1_CTRL Register
///
/// Controls the tick generator
///
/// Address offset: 0x0024
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **RUNNING** `[1]` (RO)
///
///   Is the tick generator running?
///
/// - **ENABLE** `[0]` (RW)
///
///   start / stop tick generation
#[repr(transparent)]
pub struct Timer1Ctrl {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Timer1Ctrl {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Timer1Ctrl {
    // ======== Field Constants ========

    /// **RUNNING**
    ///
    /// Is the tick generator running?
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const RUNNING_OFFSET: u32 = 1;
    pub const RUNNING_MASK: u32 = 0x00000002;
    pub const RUNNING: u32 = 1 << 1;

    /// **ENABLE**
    ///
    /// start / stop tick generation
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const ENABLE_OFFSET: u32 = 0;
    pub const ENABLE_MASK: u32 = 0x00000001;
    pub const ENABLE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **RUNNING** `[1]` (RO)
    ///
    /// Is the tick generator running?
    #[inline(always)]
    pub fn running(&self) -> bool {
        (self.value.read_bits() & Self::RUNNING_MASK) != 0
    }

    /// **ENABLE** `[0]` (RW)
    ///
    /// start / stop tick generation
    #[inline(always)]
    pub fn enable(&self) -> bool {
        (self.value.read_bits() & Self::ENABLE_MASK) != 0
    }

    /// Set ENABLE bit atomically (RW)
    #[inline(always)]
    pub fn set_enable(&self) {
        self.set_bits(Self::ENABLE_MASK);
    }

    /// Clear ENABLE bit atomically (RW)
    #[inline(always)]
    pub fn clr_enable(&self) {
        self.clr_bits(Self::ENABLE_MASK);
    }
}

/// TIMER1_CTRL value type for batch read/modify/write operations
///
/// Use `timer1ctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `timer1ctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Timer1CtrlVal {
    bits: u32,
}

impl Timer1CtrlVal {
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

    /// Read **RUNNING** `[1:1]`
    ///
    /// Is the tick generator running?
    #[inline(always)]
    pub const fn running(&self) -> bool {
        (self.bits & Timer1Ctrl::RUNNING_MASK) != 0
    }

    /// Read **ENABLE** `[0:0]`
    ///
    /// start / stop tick generation
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        (self.bits & Timer1Ctrl::ENABLE_MASK) != 0
    }

    /// Set **ENABLE** `[0:0]`
    ///
    /// start / stop tick generation
    #[inline(always)]
    pub const fn with_enable(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Timer1Ctrl::ENABLE_MASK
            } else {
                self.bits & !Timer1Ctrl::ENABLE_MASK
            },
        }
    }
}

impl Default for Timer1CtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Timer1Ctrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Timer1CtrlVal {
        Timer1CtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Timer1CtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Timer1CtrlVal) -> Timer1CtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TIMER1_CYCLES Register
///
/// Address offset: 0x0028
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **TIMER1_CYCLES** `[8:0]` (RW)
///
///   Total number of clk_tick cycles before the next tick.
#[repr(transparent)]
pub struct Timer1Cycles {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Timer1Cycles {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Timer1Cycles {
    // ======== Field Constants ========

    /// **TIMER1_CYCLES**
    ///
    /// Total number of clk_tick cycles before the next tick.
    ///
    /// Bits `[8:0]`, mask `0x000001ff`
    pub const TIMER1_CYCLES_OFFSET: u32 = 0;
    pub const TIMER1_CYCLES_MASK: u32 = 0x000001ff;

    // ======== Field Accessors ========

    /// **TIMER1_CYCLES** `[8:0]` (RW)
    ///
    /// Total number of clk_tick cycles before the next tick.
    #[inline(always)]
    pub fn timer1_cycles(&self) -> u32 {
        (self.value.read_bits() & Self::TIMER1_CYCLES_MASK) >> Self::TIMER1_CYCLES_OFFSET
    }

    /// Write **TIMER1_CYCLES** `[8:0]` (RW)
    ///
    /// Total number of clk_tick cycles before the next tick.
    #[inline(always)]
    pub fn write_timer1_cycles(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TIMER1_CYCLES_MASK)
                | ((value << Self::TIMER1_CYCLES_OFFSET) & Self::TIMER1_CYCLES_MASK)
        });
    }
}

/// TIMER1_CYCLES value type for batch read/modify/write operations
///
/// Use `timer1cycles.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `timer1cycles.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Timer1CyclesVal {
    bits: u32,
}

impl Timer1CyclesVal {
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

    /// Read **TIMER1_CYCLES** `[8:0]`
    ///
    /// Total number of clk_tick cycles before the next tick.
    #[inline(always)]
    pub const fn timer1_cycles(&self) -> u32 {
        (self.bits & Timer1Cycles::TIMER1_CYCLES_MASK) >> Timer1Cycles::TIMER1_CYCLES_OFFSET
    }

    /// Set **TIMER1_CYCLES** `[8:0]`
    ///
    /// Total number of clk_tick cycles before the next tick.
    #[inline(always)]
    pub const fn with_timer1_cycles(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Timer1Cycles::TIMER1_CYCLES_MASK)
                | ((value << Timer1Cycles::TIMER1_CYCLES_OFFSET)
                    & Timer1Cycles::TIMER1_CYCLES_MASK),
        }
    }
}

impl Default for Timer1CyclesVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Timer1Cycles {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Timer1CyclesVal {
        Timer1CyclesVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Timer1CyclesVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Timer1CyclesVal) -> Timer1CyclesVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TIMER1_COUNT Register
///
/// Address offset: 0x002c
///
/// ## Fields
///
/// - **TIMER1_COUNT** `[8:0]` (RO)
///
///   Count down timer: the remaining number clk_tick cycles before the next tick is generated.
#[repr(transparent)]
pub struct Timer1Count {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Timer1Count {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Timer1Count {
    // ======== Field Constants ========

    /// **TIMER1_COUNT**
    ///
    /// Count down timer: the remaining number clk_tick cycles before the next tick is generated.
    ///
    /// Bits `[8:0]`, mask `0x000001ff`
    pub const TIMER1_COUNT_OFFSET: u32 = 0;
    pub const TIMER1_COUNT_MASK: u32 = 0x000001ff;

    // ======== Field Accessors ========

    /// **TIMER1_COUNT** `[8:0]` (RO)
    ///
    /// Count down timer: the remaining number clk_tick cycles before the next tick is generated.
    #[inline(always)]
    pub fn timer1_count(&self) -> u32 {
        (self.value.read_bits() & Self::TIMER1_COUNT_MASK) >> Self::TIMER1_COUNT_OFFSET
    }
}

/// TIMER1_COUNT value type for batch read/modify/write operations
///
/// Use `timer1count.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `timer1count.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Timer1CountVal {
    bits: u32,
}

impl Timer1CountVal {
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

    /// Read **TIMER1_COUNT** `[8:0]`
    ///
    /// Count down timer: the remaining number clk_tick cycles before the next tick is generated.
    #[inline(always)]
    pub const fn timer1_count(&self) -> u32 {
        (self.bits & Timer1Count::TIMER1_COUNT_MASK) >> Timer1Count::TIMER1_COUNT_OFFSET
    }
}

impl Default for Timer1CountVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Timer1Count {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Timer1CountVal {
        Timer1CountVal::from_bits(self.value.read_bits())
    }
}

/// WATCHDOG_CTRL Register
///
/// Controls the tick generator
///
/// Address offset: 0x0030
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **RUNNING** `[1]` (RO)
///
///   Is the tick generator running?
///
/// - **ENABLE** `[0]` (RW)
///
///   start / stop tick generation
#[repr(transparent)]
pub struct WatchdogCtrl {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for WatchdogCtrl {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl WatchdogCtrl {
    // ======== Field Constants ========

    /// **RUNNING**
    ///
    /// Is the tick generator running?
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const RUNNING_OFFSET: u32 = 1;
    pub const RUNNING_MASK: u32 = 0x00000002;
    pub const RUNNING: u32 = 1 << 1;

    /// **ENABLE**
    ///
    /// start / stop tick generation
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const ENABLE_OFFSET: u32 = 0;
    pub const ENABLE_MASK: u32 = 0x00000001;
    pub const ENABLE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **RUNNING** `[1]` (RO)
    ///
    /// Is the tick generator running?
    #[inline(always)]
    pub fn running(&self) -> bool {
        (self.value.read_bits() & Self::RUNNING_MASK) != 0
    }

    /// **ENABLE** `[0]` (RW)
    ///
    /// start / stop tick generation
    #[inline(always)]
    pub fn enable(&self) -> bool {
        (self.value.read_bits() & Self::ENABLE_MASK) != 0
    }

    /// Set ENABLE bit atomically (RW)
    #[inline(always)]
    pub fn set_enable(&self) {
        self.set_bits(Self::ENABLE_MASK);
    }

    /// Clear ENABLE bit atomically (RW)
    #[inline(always)]
    pub fn clr_enable(&self) {
        self.clr_bits(Self::ENABLE_MASK);
    }
}

/// WATCHDOG_CTRL value type for batch read/modify/write operations
///
/// Use `watchdogctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `watchdogctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct WatchdogCtrlVal {
    bits: u32,
}

impl WatchdogCtrlVal {
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

    /// Read **RUNNING** `[1:1]`
    ///
    /// Is the tick generator running?
    #[inline(always)]
    pub const fn running(&self) -> bool {
        (self.bits & WatchdogCtrl::RUNNING_MASK) != 0
    }

    /// Read **ENABLE** `[0:0]`
    ///
    /// start / stop tick generation
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        (self.bits & WatchdogCtrl::ENABLE_MASK) != 0
    }

    /// Set **ENABLE** `[0:0]`
    ///
    /// start / stop tick generation
    #[inline(always)]
    pub const fn with_enable(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | WatchdogCtrl::ENABLE_MASK
            } else {
                self.bits & !WatchdogCtrl::ENABLE_MASK
            },
        }
    }
}

impl Default for WatchdogCtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl WatchdogCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> WatchdogCtrlVal {
        WatchdogCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: WatchdogCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(WatchdogCtrlVal) -> WatchdogCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// WATCHDOG_CYCLES Register
///
/// Address offset: 0x0034
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **WATCHDOG_CYCLES** `[8:0]` (RW)
///
///   Total number of clk_tick cycles before the next tick.
#[repr(transparent)]
pub struct WatchdogCycles {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for WatchdogCycles {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl WatchdogCycles {
    // ======== Field Constants ========

    /// **WATCHDOG_CYCLES**
    ///
    /// Total number of clk_tick cycles before the next tick.
    ///
    /// Bits `[8:0]`, mask `0x000001ff`
    pub const WATCHDOG_CYCLES_OFFSET: u32 = 0;
    pub const WATCHDOG_CYCLES_MASK: u32 = 0x000001ff;

    // ======== Field Accessors ========

    /// **WATCHDOG_CYCLES** `[8:0]` (RW)
    ///
    /// Total number of clk_tick cycles before the next tick.
    #[inline(always)]
    pub fn watchdog_cycles(&self) -> u32 {
        (self.value.read_bits() & Self::WATCHDOG_CYCLES_MASK) >> Self::WATCHDOG_CYCLES_OFFSET
    }

    /// Write **WATCHDOG_CYCLES** `[8:0]` (RW)
    ///
    /// Total number of clk_tick cycles before the next tick.
    #[inline(always)]
    pub fn write_watchdog_cycles(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::WATCHDOG_CYCLES_MASK)
                | ((value << Self::WATCHDOG_CYCLES_OFFSET) & Self::WATCHDOG_CYCLES_MASK)
        });
    }
}

/// WATCHDOG_CYCLES value type for batch read/modify/write operations
///
/// Use `watchdogcycles.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `watchdogcycles.write_val(val)`.
#[derive(Clone, Copy)]
pub struct WatchdogCyclesVal {
    bits: u32,
}

impl WatchdogCyclesVal {
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

    /// Read **WATCHDOG_CYCLES** `[8:0]`
    ///
    /// Total number of clk_tick cycles before the next tick.
    #[inline(always)]
    pub const fn watchdog_cycles(&self) -> u32 {
        (self.bits & WatchdogCycles::WATCHDOG_CYCLES_MASK) >> WatchdogCycles::WATCHDOG_CYCLES_OFFSET
    }

    /// Set **WATCHDOG_CYCLES** `[8:0]`
    ///
    /// Total number of clk_tick cycles before the next tick.
    #[inline(always)]
    pub const fn with_watchdog_cycles(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !WatchdogCycles::WATCHDOG_CYCLES_MASK)
                | ((value << WatchdogCycles::WATCHDOG_CYCLES_OFFSET)
                    & WatchdogCycles::WATCHDOG_CYCLES_MASK),
        }
    }
}

impl Default for WatchdogCyclesVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl WatchdogCycles {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> WatchdogCyclesVal {
        WatchdogCyclesVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: WatchdogCyclesVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(WatchdogCyclesVal) -> WatchdogCyclesVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// WATCHDOG_COUNT Register
///
/// Address offset: 0x0038
///
/// ## Fields
///
/// - **WATCHDOG_COUNT** `[8:0]` (RO)
///
///   Count down timer: the remaining number clk_tick cycles before the next tick is generated.
#[repr(transparent)]
pub struct WatchdogCount {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for WatchdogCount {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl WatchdogCount {
    // ======== Field Constants ========

    /// **WATCHDOG_COUNT**
    ///
    /// Count down timer: the remaining number clk_tick cycles before the next tick is generated.
    ///
    /// Bits `[8:0]`, mask `0x000001ff`
    pub const WATCHDOG_COUNT_OFFSET: u32 = 0;
    pub const WATCHDOG_COUNT_MASK: u32 = 0x000001ff;

    // ======== Field Accessors ========

    /// **WATCHDOG_COUNT** `[8:0]` (RO)
    ///
    /// Count down timer: the remaining number clk_tick cycles before the next tick is generated.
    #[inline(always)]
    pub fn watchdog_count(&self) -> u32 {
        (self.value.read_bits() & Self::WATCHDOG_COUNT_MASK) >> Self::WATCHDOG_COUNT_OFFSET
    }
}

/// WATCHDOG_COUNT value type for batch read/modify/write operations
///
/// Use `watchdogcount.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `watchdogcount.write_val(val)`.
#[derive(Clone, Copy)]
pub struct WatchdogCountVal {
    bits: u32,
}

impl WatchdogCountVal {
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

    /// Read **WATCHDOG_COUNT** `[8:0]`
    ///
    /// Count down timer: the remaining number clk_tick cycles before the next tick is generated.
    #[inline(always)]
    pub const fn watchdog_count(&self) -> u32 {
        (self.bits & WatchdogCount::WATCHDOG_COUNT_MASK) >> WatchdogCount::WATCHDOG_COUNT_OFFSET
    }
}

impl Default for WatchdogCountVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl WatchdogCount {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> WatchdogCountVal {
        WatchdogCountVal::from_bits(self.value.read_bits())
    }
}

/// RISCV_CTRL Register
///
/// Controls the tick generator
///
/// Address offset: 0x003c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **RUNNING** `[1]` (RO)
///
///   Is the tick generator running?
///
/// - **ENABLE** `[0]` (RW)
///
///   start / stop tick generation
#[repr(transparent)]
pub struct RiscvCtrl {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for RiscvCtrl {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RiscvCtrl {
    // ======== Field Constants ========

    /// **RUNNING**
    ///
    /// Is the tick generator running?
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const RUNNING_OFFSET: u32 = 1;
    pub const RUNNING_MASK: u32 = 0x00000002;
    pub const RUNNING: u32 = 1 << 1;

    /// **ENABLE**
    ///
    /// start / stop tick generation
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const ENABLE_OFFSET: u32 = 0;
    pub const ENABLE_MASK: u32 = 0x00000001;
    pub const ENABLE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **RUNNING** `[1]` (RO)
    ///
    /// Is the tick generator running?
    #[inline(always)]
    pub fn running(&self) -> bool {
        (self.value.read_bits() & Self::RUNNING_MASK) != 0
    }

    /// **ENABLE** `[0]` (RW)
    ///
    /// start / stop tick generation
    #[inline(always)]
    pub fn enable(&self) -> bool {
        (self.value.read_bits() & Self::ENABLE_MASK) != 0
    }

    /// Set ENABLE bit atomically (RW)
    #[inline(always)]
    pub fn set_enable(&self) {
        self.set_bits(Self::ENABLE_MASK);
    }

    /// Clear ENABLE bit atomically (RW)
    #[inline(always)]
    pub fn clr_enable(&self) {
        self.clr_bits(Self::ENABLE_MASK);
    }
}

/// RISCV_CTRL value type for batch read/modify/write operations
///
/// Use `riscvctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `riscvctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RiscvCtrlVal {
    bits: u32,
}

impl RiscvCtrlVal {
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

    /// Read **RUNNING** `[1:1]`
    ///
    /// Is the tick generator running?
    #[inline(always)]
    pub const fn running(&self) -> bool {
        (self.bits & RiscvCtrl::RUNNING_MASK) != 0
    }

    /// Read **ENABLE** `[0:0]`
    ///
    /// start / stop tick generation
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        (self.bits & RiscvCtrl::ENABLE_MASK) != 0
    }

    /// Set **ENABLE** `[0:0]`
    ///
    /// start / stop tick generation
    #[inline(always)]
    pub const fn with_enable(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RiscvCtrl::ENABLE_MASK
            } else {
                self.bits & !RiscvCtrl::ENABLE_MASK
            },
        }
    }
}

impl Default for RiscvCtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl RiscvCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RiscvCtrlVal {
        RiscvCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RiscvCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RiscvCtrlVal) -> RiscvCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RISCV_CYCLES Register
///
/// Address offset: 0x0040
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **RISCV_CYCLES** `[8:0]` (RW)
///
///   Total number of clk_tick cycles before the next tick.
#[repr(transparent)]
pub struct RiscvCycles {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for RiscvCycles {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RiscvCycles {
    // ======== Field Constants ========

    /// **RISCV_CYCLES**
    ///
    /// Total number of clk_tick cycles before the next tick.
    ///
    /// Bits `[8:0]`, mask `0x000001ff`
    pub const RISCV_CYCLES_OFFSET: u32 = 0;
    pub const RISCV_CYCLES_MASK: u32 = 0x000001ff;

    // ======== Field Accessors ========

    /// **RISCV_CYCLES** `[8:0]` (RW)
    ///
    /// Total number of clk_tick cycles before the next tick.
    #[inline(always)]
    pub fn riscv_cycles(&self) -> u32 {
        (self.value.read_bits() & Self::RISCV_CYCLES_MASK) >> Self::RISCV_CYCLES_OFFSET
    }

    /// Write **RISCV_CYCLES** `[8:0]` (RW)
    ///
    /// Total number of clk_tick cycles before the next tick.
    #[inline(always)]
    pub fn write_riscv_cycles(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RISCV_CYCLES_MASK)
                | ((value << Self::RISCV_CYCLES_OFFSET) & Self::RISCV_CYCLES_MASK)
        });
    }
}

/// RISCV_CYCLES value type for batch read/modify/write operations
///
/// Use `riscvcycles.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `riscvcycles.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RiscvCyclesVal {
    bits: u32,
}

impl RiscvCyclesVal {
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

    /// Read **RISCV_CYCLES** `[8:0]`
    ///
    /// Total number of clk_tick cycles before the next tick.
    #[inline(always)]
    pub const fn riscv_cycles(&self) -> u32 {
        (self.bits & RiscvCycles::RISCV_CYCLES_MASK) >> RiscvCycles::RISCV_CYCLES_OFFSET
    }

    /// Set **RISCV_CYCLES** `[8:0]`
    ///
    /// Total number of clk_tick cycles before the next tick.
    #[inline(always)]
    pub const fn with_riscv_cycles(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RiscvCycles::RISCV_CYCLES_MASK)
                | ((value << RiscvCycles::RISCV_CYCLES_OFFSET) & RiscvCycles::RISCV_CYCLES_MASK),
        }
    }
}

impl Default for RiscvCyclesVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl RiscvCycles {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RiscvCyclesVal {
        RiscvCyclesVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RiscvCyclesVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RiscvCyclesVal) -> RiscvCyclesVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RISCV_COUNT Register
///
/// Address offset: 0x0044
///
/// ## Fields
///
/// - **RISCV_COUNT** `[8:0]` (RO)
///
///   Count down timer: the remaining number clk_tick cycles before the next tick is generated.
#[repr(transparent)]
pub struct RiscvCount {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for RiscvCount {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RiscvCount {
    // ======== Field Constants ========

    /// **RISCV_COUNT**
    ///
    /// Count down timer: the remaining number clk_tick cycles before the next tick is generated.
    ///
    /// Bits `[8:0]`, mask `0x000001ff`
    pub const RISCV_COUNT_OFFSET: u32 = 0;
    pub const RISCV_COUNT_MASK: u32 = 0x000001ff;

    // ======== Field Accessors ========

    /// **RISCV_COUNT** `[8:0]` (RO)
    ///
    /// Count down timer: the remaining number clk_tick cycles before the next tick is generated.
    #[inline(always)]
    pub fn riscv_count(&self) -> u32 {
        (self.value.read_bits() & Self::RISCV_COUNT_MASK) >> Self::RISCV_COUNT_OFFSET
    }
}

/// RISCV_COUNT value type for batch read/modify/write operations
///
/// Use `riscvcount.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `riscvcount.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RiscvCountVal {
    bits: u32,
}

impl RiscvCountVal {
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

    /// Read **RISCV_COUNT** `[8:0]`
    ///
    /// Count down timer: the remaining number clk_tick cycles before the next tick is generated.
    #[inline(always)]
    pub const fn riscv_count(&self) -> u32 {
        (self.bits & RiscvCount::RISCV_COUNT_MASK) >> RiscvCount::RISCV_COUNT_OFFSET
    }
}

impl Default for RiscvCountVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RiscvCount {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RiscvCountVal {
        RiscvCountVal::from_bits(self.value.read_bits())
    }
}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// Controls the tick generator
    pub proc0_ctrl: Proc0Ctrl,
    /// 0x0004
    pub proc0_cycles: Proc0Cycles,
    /// 0x0008
    pub proc0_count: Proc0Count,
    /// 0x000c
    /// Controls the tick generator
    pub proc1_ctrl: Proc1Ctrl,
    /// 0x0010
    pub proc1_cycles: Proc1Cycles,
    /// 0x0014
    pub proc1_count: Proc1Count,
    /// 0x0018
    /// Controls the tick generator
    pub timer0_ctrl: Timer0Ctrl,
    /// 0x001c
    pub timer0_cycles: Timer0Cycles,
    /// 0x0020
    pub timer0_count: Timer0Count,
    /// 0x0024
    /// Controls the tick generator
    pub timer1_ctrl: Timer1Ctrl,
    /// 0x0028
    pub timer1_cycles: Timer1Cycles,
    /// 0x002c
    pub timer1_count: Timer1Count,
    /// 0x0030
    /// Controls the tick generator
    pub watchdog_ctrl: WatchdogCtrl,
    /// 0x0034
    pub watchdog_cycles: WatchdogCycles,
    /// 0x0038
    pub watchdog_count: WatchdogCount,
    /// 0x003c
    /// Controls the tick generator
    pub riscv_ctrl: RiscvCtrl,
    /// 0x0040
    pub riscv_cycles: RiscvCycles,
    /// 0x0044
    pub riscv_count: RiscvCount,
}
