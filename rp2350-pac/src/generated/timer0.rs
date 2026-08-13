//! TIMER0
//! Controls time and alarms
//! time is a 64 bit value indicating the time since power-on
//! timeh is the top 32 bits of time & timel is the bottom 32 bits to change time write to timelw before timehw to read time read from timelr before timehr
//!
//! This peripheral supports atomic SET/CLR/XOR register aliases.

#![allow(dead_code)]

pub const BASE: u32 = 0x400b0000;

/// TIMEHW Register
///
/// Write to bits 63:32 of time always write timelw before timehw
///
/// Address offset: 0x0000
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **TIMEHW** `[31:0]` (WO)
#[repr(transparent)]
pub struct Timehw {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Timehw {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Timehw {
    // ======== Field Constants ========

    /// **TIMEHW**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const TIMEHW_OFFSET: u32 = 0;
    pub const TIMEHW_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// Write **TIMEHW** `[31:0]` (WO)
    #[inline(always)]
    pub fn write_timehw(&self, value: u32) {
        self.value
            .write_bits((value << Self::TIMEHW_OFFSET) & Self::TIMEHW_MASK);
    }
}

/// TIMEHW value type for batch read/modify/write operations
///
/// Use `timehw.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `timehw.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TimehwVal {
    bits: u32,
}

impl TimehwVal {
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

    /// Set **TIMEHW** `[31:0]`
    #[inline(always)]
    pub const fn with_timehw(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Timehw::TIMEHW_MASK)
                | ((value << Timehw::TIMEHW_OFFSET) & Timehw::TIMEHW_MASK),
        }
    }
}

impl Default for TimehwVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Timehw {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TimehwVal) {
        self.value.write_bits(val.bits())
    }
}

/// TIMELW Register
///
/// Write to bits 31:0 of time writes do not get copied to time until timehw is written
///
/// Address offset: 0x0004
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **TIMELW** `[31:0]` (WO)
#[repr(transparent)]
pub struct Timelw {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Timelw {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Timelw {
    // ======== Field Constants ========

    /// **TIMELW**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const TIMELW_OFFSET: u32 = 0;
    pub const TIMELW_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// Write **TIMELW** `[31:0]` (WO)
    #[inline(always)]
    pub fn write_timelw(&self, value: u32) {
        self.value
            .write_bits((value << Self::TIMELW_OFFSET) & Self::TIMELW_MASK);
    }
}

/// TIMELW value type for batch read/modify/write operations
///
/// Use `timelw.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `timelw.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TimelwVal {
    bits: u32,
}

impl TimelwVal {
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

    /// Set **TIMELW** `[31:0]`
    #[inline(always)]
    pub const fn with_timelw(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Timelw::TIMELW_MASK)
                | ((value << Timelw::TIMELW_OFFSET) & Timelw::TIMELW_MASK),
        }
    }
}

impl Default for TimelwVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Timelw {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TimelwVal) {
        self.value.write_bits(val.bits())
    }
}

/// TIMEHR Register
///
/// Read from bits 63:32 of time always read timelr before timehr
///
/// Address offset: 0x0008
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **TIMEHR** `[31:0]` (RO)
#[repr(transparent)]
pub struct Timehr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Timehr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Timehr {
    // ======== Field Constants ========

    /// **TIMEHR**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const TIMEHR_OFFSET: u32 = 0;
    pub const TIMEHR_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **TIMEHR** `[31:0]` (RO)
    #[inline(always)]
    pub fn timehr(&self) -> u32 {
        (self.value.read_bits() & Self::TIMEHR_MASK) >> Self::TIMEHR_OFFSET
    }
}

/// TIMEHR value type for batch read/modify/write operations
///
/// Use `timehr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `timehr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TimehrVal {
    bits: u32,
}

impl TimehrVal {
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

    /// Read **TIMEHR** `[31:0]`
    #[inline(always)]
    pub const fn timehr(&self) -> u32 {
        (self.bits & Timehr::TIMEHR_MASK) >> Timehr::TIMEHR_OFFSET
    }
}

impl Default for TimehrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Timehr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TimehrVal {
        TimehrVal::from_bits(self.value.read_bits())
    }
}

/// TIMELR Register
///
/// Read from bits 31:0 of time
///
/// Address offset: 0x000c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **TIMELR** `[31:0]` (RO)
#[repr(transparent)]
pub struct Timelr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Timelr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Timelr {
    // ======== Field Constants ========

    /// **TIMELR**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const TIMELR_OFFSET: u32 = 0;
    pub const TIMELR_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **TIMELR** `[31:0]` (RO)
    #[inline(always)]
    pub fn timelr(&self) -> u32 {
        (self.value.read_bits() & Self::TIMELR_MASK) >> Self::TIMELR_OFFSET
    }
}

/// TIMELR value type for batch read/modify/write operations
///
/// Use `timelr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `timelr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TimelrVal {
    bits: u32,
}

impl TimelrVal {
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

    /// Read **TIMELR** `[31:0]`
    #[inline(always)]
    pub const fn timelr(&self) -> u32 {
        (self.bits & Timelr::TIMELR_MASK) >> Timelr::TIMELR_OFFSET
    }
}

impl Default for TimelrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Timelr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TimelrVal {
        TimelrVal::from_bits(self.value.read_bits())
    }
}

/// ALARM0 Register
///
/// Arm alarm 0, and configure the time it will fire. Once armed, the alarm fires when TIMER_ALARM0 == TIMELR. The alarm will disarm itself once it fires, and can be disarmed early using the ARMED status register.
///
/// Address offset: 0x0010
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **ALARM0** `[31:0]` (RW)
#[repr(transparent)]
pub struct Alarm0 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Alarm0 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Alarm0 {
    // ======== Field Constants ========

    /// **ALARM0**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const ALARM0_OFFSET: u32 = 0;
    pub const ALARM0_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **ALARM0** `[31:0]` (RW)
    #[inline(always)]
    pub fn alarm0(&self) -> u32 {
        (self.value.read_bits() & Self::ALARM0_MASK) >> Self::ALARM0_OFFSET
    }

    /// Write **ALARM0** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_alarm0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ALARM0_MASK) | ((value << Self::ALARM0_OFFSET) & Self::ALARM0_MASK)
        });
    }
}

/// ALARM0 value type for batch read/modify/write operations
///
/// Use `alarm0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `alarm0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Alarm0Val {
    bits: u32,
}

impl Alarm0Val {
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

    /// Read **ALARM0** `[31:0]`
    #[inline(always)]
    pub const fn alarm0(&self) -> u32 {
        (self.bits & Alarm0::ALARM0_MASK) >> Alarm0::ALARM0_OFFSET
    }

    /// Set **ALARM0** `[31:0]`
    #[inline(always)]
    pub const fn with_alarm0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Alarm0::ALARM0_MASK)
                | ((value << Alarm0::ALARM0_OFFSET) & Alarm0::ALARM0_MASK),
        }
    }
}

impl Default for Alarm0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Alarm0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Alarm0Val {
        Alarm0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Alarm0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Alarm0Val) -> Alarm0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ALARM1 Register
///
/// Arm alarm 1, and configure the time it will fire. Once armed, the alarm fires when TIMER_ALARM1 == TIMELR. The alarm will disarm itself once it fires, and can be disarmed early using the ARMED status register.
///
/// Address offset: 0x0014
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **ALARM1** `[31:0]` (RW)
#[repr(transparent)]
pub struct Alarm1 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Alarm1 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Alarm1 {
    // ======== Field Constants ========

    /// **ALARM1**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const ALARM1_OFFSET: u32 = 0;
    pub const ALARM1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **ALARM1** `[31:0]` (RW)
    #[inline(always)]
    pub fn alarm1(&self) -> u32 {
        (self.value.read_bits() & Self::ALARM1_MASK) >> Self::ALARM1_OFFSET
    }

    /// Write **ALARM1** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_alarm1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ALARM1_MASK) | ((value << Self::ALARM1_OFFSET) & Self::ALARM1_MASK)
        });
    }
}

/// ALARM1 value type for batch read/modify/write operations
///
/// Use `alarm1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `alarm1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Alarm1Val {
    bits: u32,
}

impl Alarm1Val {
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

    /// Read **ALARM1** `[31:0]`
    #[inline(always)]
    pub const fn alarm1(&self) -> u32 {
        (self.bits & Alarm1::ALARM1_MASK) >> Alarm1::ALARM1_OFFSET
    }

    /// Set **ALARM1** `[31:0]`
    #[inline(always)]
    pub const fn with_alarm1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Alarm1::ALARM1_MASK)
                | ((value << Alarm1::ALARM1_OFFSET) & Alarm1::ALARM1_MASK),
        }
    }
}

impl Default for Alarm1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Alarm1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Alarm1Val {
        Alarm1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Alarm1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Alarm1Val) -> Alarm1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ALARM2 Register
///
/// Arm alarm 2, and configure the time it will fire. Once armed, the alarm fires when TIMER_ALARM2 == TIMELR. The alarm will disarm itself once it fires, and can be disarmed early using the ARMED status register.
///
/// Address offset: 0x0018
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **ALARM2** `[31:0]` (RW)
#[repr(transparent)]
pub struct Alarm2 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Alarm2 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Alarm2 {
    // ======== Field Constants ========

    /// **ALARM2**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const ALARM2_OFFSET: u32 = 0;
    pub const ALARM2_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **ALARM2** `[31:0]` (RW)
    #[inline(always)]
    pub fn alarm2(&self) -> u32 {
        (self.value.read_bits() & Self::ALARM2_MASK) >> Self::ALARM2_OFFSET
    }

    /// Write **ALARM2** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_alarm2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ALARM2_MASK) | ((value << Self::ALARM2_OFFSET) & Self::ALARM2_MASK)
        });
    }
}

/// ALARM2 value type for batch read/modify/write operations
///
/// Use `alarm2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `alarm2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Alarm2Val {
    bits: u32,
}

impl Alarm2Val {
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

    /// Read **ALARM2** `[31:0]`
    #[inline(always)]
    pub const fn alarm2(&self) -> u32 {
        (self.bits & Alarm2::ALARM2_MASK) >> Alarm2::ALARM2_OFFSET
    }

    /// Set **ALARM2** `[31:0]`
    #[inline(always)]
    pub const fn with_alarm2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Alarm2::ALARM2_MASK)
                | ((value << Alarm2::ALARM2_OFFSET) & Alarm2::ALARM2_MASK),
        }
    }
}

impl Default for Alarm2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Alarm2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Alarm2Val {
        Alarm2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Alarm2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Alarm2Val) -> Alarm2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ALARM3 Register
///
/// Arm alarm 3, and configure the time it will fire. Once armed, the alarm fires when TIMER_ALARM3 == TIMELR. The alarm will disarm itself once it fires, and can be disarmed early using the ARMED status register.
///
/// Address offset: 0x001c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **ALARM3** `[31:0]` (RW)
#[repr(transparent)]
pub struct Alarm3 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Alarm3 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Alarm3 {
    // ======== Field Constants ========

    /// **ALARM3**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const ALARM3_OFFSET: u32 = 0;
    pub const ALARM3_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **ALARM3** `[31:0]` (RW)
    #[inline(always)]
    pub fn alarm3(&self) -> u32 {
        (self.value.read_bits() & Self::ALARM3_MASK) >> Self::ALARM3_OFFSET
    }

    /// Write **ALARM3** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_alarm3(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ALARM3_MASK) | ((value << Self::ALARM3_OFFSET) & Self::ALARM3_MASK)
        });
    }
}

/// ALARM3 value type for batch read/modify/write operations
///
/// Use `alarm3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `alarm3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Alarm3Val {
    bits: u32,
}

impl Alarm3Val {
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

    /// Read **ALARM3** `[31:0]`
    #[inline(always)]
    pub const fn alarm3(&self) -> u32 {
        (self.bits & Alarm3::ALARM3_MASK) >> Alarm3::ALARM3_OFFSET
    }

    /// Set **ALARM3** `[31:0]`
    #[inline(always)]
    pub const fn with_alarm3(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Alarm3::ALARM3_MASK)
                | ((value << Alarm3::ALARM3_OFFSET) & Alarm3::ALARM3_MASK),
        }
    }
}

impl Default for Alarm3Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Alarm3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Alarm3Val {
        Alarm3Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Alarm3Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Alarm3Val) -> Alarm3Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ARMED Register
///
/// Indicates the armed/disarmed status of each alarm. A write to the corresponding ALARMx register arms the alarm. Alarms automatically disarm upon firing, but writing ones here will disarm immediately without waiting to fire.
///
/// Address offset: 0x0020
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **ARMED** `[3:0]` (RW)
#[repr(transparent)]
pub struct Armed {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Armed {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Armed {
    // ======== Field Constants ========

    /// **ARMED**
    /// Bits `[3:0]`, mask `0x0000000f`
    pub const ARMED_OFFSET: u32 = 0;
    pub const ARMED_MASK: u32 = 0x0000000f;

    // ======== Field Accessors ========

    /// **ARMED** `[3:0]` (RW)
    #[inline(always)]
    pub fn armed(&self) -> u32 {
        (self.value.read_bits() & Self::ARMED_MASK) >> Self::ARMED_OFFSET
    }

    /// Write **ARMED** `[3:0]` (RW)
    #[inline(always)]
    pub fn write_armed(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ARMED_MASK) | ((value << Self::ARMED_OFFSET) & Self::ARMED_MASK)
        });
    }
}

/// ARMED value type for batch read/modify/write operations
///
/// Use `armed.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `armed.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ArmedVal {
    bits: u32,
}

impl ArmedVal {
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

    /// Read **ARMED** `[3:0]`
    #[inline(always)]
    pub const fn armed(&self) -> u32 {
        (self.bits & Armed::ARMED_MASK) >> Armed::ARMED_OFFSET
    }

    /// Set **ARMED** `[3:0]`
    #[inline(always)]
    pub const fn with_armed(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Armed::ARMED_MASK)
                | ((value << Armed::ARMED_OFFSET) & Armed::ARMED_MASK),
        }
    }
}

impl Default for ArmedVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Armed {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ArmedVal {
        ArmedVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ArmedVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ArmedVal) -> ArmedVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TIMERAWH Register
///
/// Raw read from bits 63:32 of time (no side effects)
///
/// Address offset: 0x0024
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **TIMERAWH** `[31:0]` (RO)
#[repr(transparent)]
pub struct Timerawh {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Timerawh {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Timerawh {
    // ======== Field Constants ========

    /// **TIMERAWH**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const TIMERAWH_OFFSET: u32 = 0;
    pub const TIMERAWH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **TIMERAWH** `[31:0]` (RO)
    #[inline(always)]
    pub fn timerawh(&self) -> u32 {
        (self.value.read_bits() & Self::TIMERAWH_MASK) >> Self::TIMERAWH_OFFSET
    }
}

/// TIMERAWH value type for batch read/modify/write operations
///
/// Use `timerawh.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `timerawh.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TimerawhVal {
    bits: u32,
}

impl TimerawhVal {
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

    /// Read **TIMERAWH** `[31:0]`
    #[inline(always)]
    pub const fn timerawh(&self) -> u32 {
        (self.bits & Timerawh::TIMERAWH_MASK) >> Timerawh::TIMERAWH_OFFSET
    }
}

impl Default for TimerawhVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Timerawh {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TimerawhVal {
        TimerawhVal::from_bits(self.value.read_bits())
    }
}

/// TIMERAWL Register
///
/// Raw read from bits 31:0 of time (no side effects)
///
/// Address offset: 0x0028
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **TIMERAWL** `[31:0]` (RO)
#[repr(transparent)]
pub struct Timerawl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Timerawl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Timerawl {
    // ======== Field Constants ========

    /// **TIMERAWL**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const TIMERAWL_OFFSET: u32 = 0;
    pub const TIMERAWL_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **TIMERAWL** `[31:0]` (RO)
    #[inline(always)]
    pub fn timerawl(&self) -> u32 {
        (self.value.read_bits() & Self::TIMERAWL_MASK) >> Self::TIMERAWL_OFFSET
    }
}

/// TIMERAWL value type for batch read/modify/write operations
///
/// Use `timerawl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `timerawl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TimerawlVal {
    bits: u32,
}

impl TimerawlVal {
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

    /// Read **TIMERAWL** `[31:0]`
    #[inline(always)]
    pub const fn timerawl(&self) -> u32 {
        (self.bits & Timerawl::TIMERAWL_MASK) >> Timerawl::TIMERAWL_OFFSET
    }
}

impl Default for TimerawlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Timerawl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TimerawlVal {
        TimerawlVal::from_bits(self.value.read_bits())
    }
}

/// DBGPAUSE Register
///
/// Set bits high to enable pause when the corresponding debug ports are active
///
/// Address offset: 0x002c
/// Reset value: 0x00000007
///
/// ## Fields
///
/// - **DBG1** `[2]` (RW)
///
///   Pause when processor 1 is in debug mode
///
/// - **DBG0** `[1]` (RW)
///
///   Pause when processor 0 is in debug mode
#[repr(transparent)]
pub struct Dbgpause {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Dbgpause {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Dbgpause {
    // ======== Field Constants ========

    /// **DBG1**
    ///
    /// Pause when processor 1 is in debug mode
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const DBG1_OFFSET: u32 = 2;
    pub const DBG1_MASK: u32 = 0x00000004;
    pub const DBG1: u32 = 1 << 2;

    /// **DBG0**
    ///
    /// Pause when processor 0 is in debug mode
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const DBG0_OFFSET: u32 = 1;
    pub const DBG0_MASK: u32 = 0x00000002;
    pub const DBG0: u32 = 1 << 1;

    // ======== Field Accessors ========

    /// **DBG1** `[2]` (RW)
    ///
    /// Pause when processor 1 is in debug mode
    #[inline(always)]
    pub fn dbg1(&self) -> bool {
        (self.value.read_bits() & Self::DBG1_MASK) != 0
    }

    /// Set DBG1 bit atomically (RW)
    #[inline(always)]
    pub fn set_dbg1(&self) {
        self.set_bits(Self::DBG1_MASK);
    }

    /// Clear DBG1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_dbg1(&self) {
        self.clr_bits(Self::DBG1_MASK);
    }

    /// **DBG0** `[1]` (RW)
    ///
    /// Pause when processor 0 is in debug mode
    #[inline(always)]
    pub fn dbg0(&self) -> bool {
        (self.value.read_bits() & Self::DBG0_MASK) != 0
    }

    /// Set DBG0 bit atomically (RW)
    #[inline(always)]
    pub fn set_dbg0(&self) {
        self.set_bits(Self::DBG0_MASK);
    }

    /// Clear DBG0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_dbg0(&self) {
        self.clr_bits(Self::DBG0_MASK);
    }
}

/// DBGPAUSE value type for batch read/modify/write operations
///
/// Use `dbgpause.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dbgpause.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DbgpauseVal {
    bits: u32,
}

impl DbgpauseVal {
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
        Self { bits: 0x00000007 }
    }

    /// Read **DBG1** `[2:2]`
    ///
    /// Pause when processor 1 is in debug mode
    #[inline(always)]
    pub const fn dbg1(&self) -> bool {
        (self.bits & Dbgpause::DBG1_MASK) != 0
    }

    /// Read **DBG0** `[1:1]`
    ///
    /// Pause when processor 0 is in debug mode
    #[inline(always)]
    pub const fn dbg0(&self) -> bool {
        (self.bits & Dbgpause::DBG0_MASK) != 0
    }

    /// Set **DBG1** `[2:2]`
    ///
    /// Pause when processor 1 is in debug mode
    #[inline(always)]
    pub const fn with_dbg1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Dbgpause::DBG1_MASK
            } else {
                self.bits & !Dbgpause::DBG1_MASK
            },
        }
    }

    /// Set **DBG0** `[1:1]`
    ///
    /// Pause when processor 0 is in debug mode
    #[inline(always)]
    pub const fn with_dbg0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Dbgpause::DBG0_MASK
            } else {
                self.bits & !Dbgpause::DBG0_MASK
            },
        }
    }
}

impl Default for DbgpauseVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Dbgpause {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DbgpauseVal {
        DbgpauseVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DbgpauseVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DbgpauseVal) -> DbgpauseVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PAUSE Register
///
/// Set high to pause the timer
///
/// Address offset: 0x0030
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **PAUSE** `[0]` (RW)
#[repr(transparent)]
pub struct Pause {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Pause {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Pause {
    // ======== Field Constants ========

    /// **PAUSE**
    /// Bits `[0:0]`, mask `0x00000001`
    pub const PAUSE_OFFSET: u32 = 0;
    pub const PAUSE_MASK: u32 = 0x00000001;
    pub const PAUSE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **PAUSE** `[0]` (RW)
    #[inline(always)]
    pub fn pause(&self) -> bool {
        (self.value.read_bits() & Self::PAUSE_MASK) != 0
    }

    /// Set PAUSE bit atomically (RW)
    #[inline(always)]
    pub fn set_pause(&self) {
        self.set_bits(Self::PAUSE_MASK);
    }

    /// Clear PAUSE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pause(&self) {
        self.clr_bits(Self::PAUSE_MASK);
    }
}

/// PAUSE value type for batch read/modify/write operations
///
/// Use `pause.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pause.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PauseVal {
    bits: u32,
}

impl PauseVal {
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

    /// Read **PAUSE** `[0:0]`
    #[inline(always)]
    pub const fn pause(&self) -> bool {
        (self.bits & Pause::PAUSE_MASK) != 0
    }

    /// Set **PAUSE** `[0:0]`
    #[inline(always)]
    pub const fn with_pause(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pause::PAUSE_MASK
            } else {
                self.bits & !Pause::PAUSE_MASK
            },
        }
    }
}

impl Default for PauseVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Pause {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PauseVal {
        PauseVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: PauseVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(PauseVal) -> PauseVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LOCKED Register
///
/// Set locked bit to disable write access to timer Once set, cannot be cleared (without a reset)
///
/// Address offset: 0x0034
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **LOCKED** `[0]` (RW)
#[repr(transparent)]
pub struct Locked {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Locked {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Locked {
    // ======== Field Constants ========

    /// **LOCKED**
    /// Bits `[0:0]`, mask `0x00000001`
    pub const LOCKED_OFFSET: u32 = 0;
    pub const LOCKED_MASK: u32 = 0x00000001;
    pub const LOCKED: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **LOCKED** `[0]` (RW)
    #[inline(always)]
    pub fn locked(&self) -> bool {
        (self.value.read_bits() & Self::LOCKED_MASK) != 0
    }

    /// Set LOCKED bit atomically (RW)
    #[inline(always)]
    pub fn set_locked(&self) {
        self.set_bits(Self::LOCKED_MASK);
    }

    /// Clear LOCKED bit atomically (RW)
    #[inline(always)]
    pub fn clr_locked(&self) {
        self.clr_bits(Self::LOCKED_MASK);
    }
}

/// LOCKED value type for batch read/modify/write operations
///
/// Use `locked.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `locked.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LockedVal {
    bits: u32,
}

impl LockedVal {
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

    /// Read **LOCKED** `[0:0]`
    #[inline(always)]
    pub const fn locked(&self) -> bool {
        (self.bits & Locked::LOCKED_MASK) != 0
    }

    /// Set **LOCKED** `[0:0]`
    #[inline(always)]
    pub const fn with_locked(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Locked::LOCKED_MASK
            } else {
                self.bits & !Locked::LOCKED_MASK
            },
        }
    }
}

impl Default for LockedVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Locked {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LockedVal {
        LockedVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LockedVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LockedVal) -> LockedVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SOURCE Register
///
/// Selects the source for the timer. Defaults to the normal tick configured in the ticks block (typically configured to 1 microsecond). Writing to 1 will ignore the tick and count clk_sys cycles instead.
///
/// Address offset: 0x0038
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CLK_SYS** `[0]` (RW)
#[repr(transparent)]
pub struct Source {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Source {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Source {
    // ======== Field Constants ========

    /// **CLK_SYS**
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CLK_SYS_OFFSET: u32 = 0;
    pub const CLK_SYS_MASK: u32 = 0x00000001;
    pub const CLK_SYS: u32 = 1 << 0;
    pub const CLK_SYS_TICK: u32 = 0 << 0;
    pub const CLK_SYS_TICK_VAL: u32 = 0;
    pub const CLK_SYS_CLK_SYS: u32 = 1 << 0;
    pub const CLK_SYS_CLK_SYS_VAL: u32 = 1;

    // ======== Field Accessors ========

    /// **CLK_SYS** `[0]` (RW)
    #[inline(always)]
    pub fn clk_sys(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_MASK) != 0
    }

    /// Set CLK_SYS bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys(&self) {
        self.set_bits(Self::CLK_SYS_MASK);
    }

    /// Clear CLK_SYS bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys(&self) {
        self.clr_bits(Self::CLK_SYS_MASK);
    }
}

/// SOURCE value type for batch read/modify/write operations
///
/// Use `source.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `source.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SourceVal {
    bits: u32,
}

impl SourceVal {
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

    /// Read **CLK_SYS** `[0:0]`
    #[inline(always)]
    pub const fn clk_sys(&self) -> bool {
        (self.bits & Source::CLK_SYS_MASK) != 0
    }

    /// Set **CLK_SYS** `[0:0]`
    #[inline(always)]
    pub const fn with_clk_sys(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Source::CLK_SYS_MASK
            } else {
                self.bits & !Source::CLK_SYS_MASK
            },
        }
    }
}

impl Default for SourceVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Source {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SourceVal {
        SourceVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SourceVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SourceVal) -> SourceVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// INTR Register
///
/// Raw Interrupts
///
/// Address offset: 0x003c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **ALARM_3** `[3]` (RW)
///
/// - **ALARM_2** `[2]` (RW)
///
/// - **ALARM_1** `[1]` (RW)
///
/// - **ALARM_0** `[0]` (RW)
#[repr(transparent)]
pub struct Intr {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Intr {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Intr {
    // ======== Field Constants ========

    /// **ALARM_3**
    /// Bits `[3:3]`, mask `0x00000008`
    pub const ALARM_3_OFFSET: u32 = 3;
    pub const ALARM_3_MASK: u32 = 0x00000008;
    pub const ALARM_3: u32 = 1 << 3;

    /// **ALARM_2**
    /// Bits `[2:2]`, mask `0x00000004`
    pub const ALARM_2_OFFSET: u32 = 2;
    pub const ALARM_2_MASK: u32 = 0x00000004;
    pub const ALARM_2: u32 = 1 << 2;

    /// **ALARM_1**
    /// Bits `[1:1]`, mask `0x00000002`
    pub const ALARM_1_OFFSET: u32 = 1;
    pub const ALARM_1_MASK: u32 = 0x00000002;
    pub const ALARM_1: u32 = 1 << 1;

    /// **ALARM_0**
    /// Bits `[0:0]`, mask `0x00000001`
    pub const ALARM_0_OFFSET: u32 = 0;
    pub const ALARM_0_MASK: u32 = 0x00000001;
    pub const ALARM_0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ALARM_3** `[3]` (RW)
    #[inline(always)]
    pub fn alarm_3(&self) -> bool {
        (self.value.read_bits() & Self::ALARM_3_MASK) != 0
    }

    /// Set ALARM_3 bit atomically (RW)
    #[inline(always)]
    pub fn set_alarm_3(&self) {
        self.set_bits(Self::ALARM_3_MASK);
    }

    /// Clear ALARM_3 bit atomically (RW)
    #[inline(always)]
    pub fn clr_alarm_3(&self) {
        self.clr_bits(Self::ALARM_3_MASK);
    }

    /// **ALARM_2** `[2]` (RW)
    #[inline(always)]
    pub fn alarm_2(&self) -> bool {
        (self.value.read_bits() & Self::ALARM_2_MASK) != 0
    }

    /// Set ALARM_2 bit atomically (RW)
    #[inline(always)]
    pub fn set_alarm_2(&self) {
        self.set_bits(Self::ALARM_2_MASK);
    }

    /// Clear ALARM_2 bit atomically (RW)
    #[inline(always)]
    pub fn clr_alarm_2(&self) {
        self.clr_bits(Self::ALARM_2_MASK);
    }

    /// **ALARM_1** `[1]` (RW)
    #[inline(always)]
    pub fn alarm_1(&self) -> bool {
        (self.value.read_bits() & Self::ALARM_1_MASK) != 0
    }

    /// Set ALARM_1 bit atomically (RW)
    #[inline(always)]
    pub fn set_alarm_1(&self) {
        self.set_bits(Self::ALARM_1_MASK);
    }

    /// Clear ALARM_1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_alarm_1(&self) {
        self.clr_bits(Self::ALARM_1_MASK);
    }

    /// **ALARM_0** `[0]` (RW)
    #[inline(always)]
    pub fn alarm_0(&self) -> bool {
        (self.value.read_bits() & Self::ALARM_0_MASK) != 0
    }

    /// Set ALARM_0 bit atomically (RW)
    #[inline(always)]
    pub fn set_alarm_0(&self) {
        self.set_bits(Self::ALARM_0_MASK);
    }

    /// Clear ALARM_0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_alarm_0(&self) {
        self.clr_bits(Self::ALARM_0_MASK);
    }
}

/// INTR value type for batch read/modify/write operations
///
/// Use `intr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `intr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntrVal {
    bits: u32,
}

impl IntrVal {
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

    /// Read **ALARM_3** `[3:3]`
    #[inline(always)]
    pub const fn alarm_3(&self) -> bool {
        (self.bits & Intr::ALARM_3_MASK) != 0
    }

    /// Read **ALARM_2** `[2:2]`
    #[inline(always)]
    pub const fn alarm_2(&self) -> bool {
        (self.bits & Intr::ALARM_2_MASK) != 0
    }

    /// Read **ALARM_1** `[1:1]`
    #[inline(always)]
    pub const fn alarm_1(&self) -> bool {
        (self.bits & Intr::ALARM_1_MASK) != 0
    }

    /// Read **ALARM_0** `[0:0]`
    #[inline(always)]
    pub const fn alarm_0(&self) -> bool {
        (self.bits & Intr::ALARM_0_MASK) != 0
    }

    /// Set **ALARM_3** `[3:3]`
    #[inline(always)]
    pub const fn with_alarm_3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intr::ALARM_3_MASK
            } else {
                self.bits & !Intr::ALARM_3_MASK
            },
        }
    }

    /// Set **ALARM_2** `[2:2]`
    #[inline(always)]
    pub const fn with_alarm_2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intr::ALARM_2_MASK
            } else {
                self.bits & !Intr::ALARM_2_MASK
            },
        }
    }

    /// Set **ALARM_1** `[1:1]`
    #[inline(always)]
    pub const fn with_alarm_1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intr::ALARM_1_MASK
            } else {
                self.bits & !Intr::ALARM_1_MASK
            },
        }
    }

    /// Set **ALARM_0** `[0:0]`
    #[inline(always)]
    pub const fn with_alarm_0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intr::ALARM_0_MASK
            } else {
                self.bits & !Intr::ALARM_0_MASK
            },
        }
    }
}

impl Default for IntrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Intr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IntrVal {
        IntrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IntrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IntrVal) -> IntrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// INTE Register
///
/// Interrupt Enable
///
/// Address offset: 0x0040
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **ALARM_3** `[3]` (RW)
///
/// - **ALARM_2** `[2]` (RW)
///
/// - **ALARM_1** `[1]` (RW)
///
/// - **ALARM_0** `[0]` (RW)
#[repr(transparent)]
pub struct Inte {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Inte {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Inte {
    // ======== Field Constants ========

    /// **ALARM_3**
    /// Bits `[3:3]`, mask `0x00000008`
    pub const ALARM_3_OFFSET: u32 = 3;
    pub const ALARM_3_MASK: u32 = 0x00000008;
    pub const ALARM_3: u32 = 1 << 3;

    /// **ALARM_2**
    /// Bits `[2:2]`, mask `0x00000004`
    pub const ALARM_2_OFFSET: u32 = 2;
    pub const ALARM_2_MASK: u32 = 0x00000004;
    pub const ALARM_2: u32 = 1 << 2;

    /// **ALARM_1**
    /// Bits `[1:1]`, mask `0x00000002`
    pub const ALARM_1_OFFSET: u32 = 1;
    pub const ALARM_1_MASK: u32 = 0x00000002;
    pub const ALARM_1: u32 = 1 << 1;

    /// **ALARM_0**
    /// Bits `[0:0]`, mask `0x00000001`
    pub const ALARM_0_OFFSET: u32 = 0;
    pub const ALARM_0_MASK: u32 = 0x00000001;
    pub const ALARM_0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ALARM_3** `[3]` (RW)
    #[inline(always)]
    pub fn alarm_3(&self) -> bool {
        (self.value.read_bits() & Self::ALARM_3_MASK) != 0
    }

    /// Set ALARM_3 bit atomically (RW)
    #[inline(always)]
    pub fn set_alarm_3(&self) {
        self.set_bits(Self::ALARM_3_MASK);
    }

    /// Clear ALARM_3 bit atomically (RW)
    #[inline(always)]
    pub fn clr_alarm_3(&self) {
        self.clr_bits(Self::ALARM_3_MASK);
    }

    /// **ALARM_2** `[2]` (RW)
    #[inline(always)]
    pub fn alarm_2(&self) -> bool {
        (self.value.read_bits() & Self::ALARM_2_MASK) != 0
    }

    /// Set ALARM_2 bit atomically (RW)
    #[inline(always)]
    pub fn set_alarm_2(&self) {
        self.set_bits(Self::ALARM_2_MASK);
    }

    /// Clear ALARM_2 bit atomically (RW)
    #[inline(always)]
    pub fn clr_alarm_2(&self) {
        self.clr_bits(Self::ALARM_2_MASK);
    }

    /// **ALARM_1** `[1]` (RW)
    #[inline(always)]
    pub fn alarm_1(&self) -> bool {
        (self.value.read_bits() & Self::ALARM_1_MASK) != 0
    }

    /// Set ALARM_1 bit atomically (RW)
    #[inline(always)]
    pub fn set_alarm_1(&self) {
        self.set_bits(Self::ALARM_1_MASK);
    }

    /// Clear ALARM_1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_alarm_1(&self) {
        self.clr_bits(Self::ALARM_1_MASK);
    }

    /// **ALARM_0** `[0]` (RW)
    #[inline(always)]
    pub fn alarm_0(&self) -> bool {
        (self.value.read_bits() & Self::ALARM_0_MASK) != 0
    }

    /// Set ALARM_0 bit atomically (RW)
    #[inline(always)]
    pub fn set_alarm_0(&self) {
        self.set_bits(Self::ALARM_0_MASK);
    }

    /// Clear ALARM_0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_alarm_0(&self) {
        self.clr_bits(Self::ALARM_0_MASK);
    }
}

/// INTE value type for batch read/modify/write operations
///
/// Use `inte.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `inte.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InteVal {
    bits: u32,
}

impl InteVal {
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

    /// Read **ALARM_3** `[3:3]`
    #[inline(always)]
    pub const fn alarm_3(&self) -> bool {
        (self.bits & Inte::ALARM_3_MASK) != 0
    }

    /// Read **ALARM_2** `[2:2]`
    #[inline(always)]
    pub const fn alarm_2(&self) -> bool {
        (self.bits & Inte::ALARM_2_MASK) != 0
    }

    /// Read **ALARM_1** `[1:1]`
    #[inline(always)]
    pub const fn alarm_1(&self) -> bool {
        (self.bits & Inte::ALARM_1_MASK) != 0
    }

    /// Read **ALARM_0** `[0:0]`
    #[inline(always)]
    pub const fn alarm_0(&self) -> bool {
        (self.bits & Inte::ALARM_0_MASK) != 0
    }

    /// Set **ALARM_3** `[3:3]`
    #[inline(always)]
    pub const fn with_alarm_3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Inte::ALARM_3_MASK
            } else {
                self.bits & !Inte::ALARM_3_MASK
            },
        }
    }

    /// Set **ALARM_2** `[2:2]`
    #[inline(always)]
    pub const fn with_alarm_2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Inte::ALARM_2_MASK
            } else {
                self.bits & !Inte::ALARM_2_MASK
            },
        }
    }

    /// Set **ALARM_1** `[1:1]`
    #[inline(always)]
    pub const fn with_alarm_1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Inte::ALARM_1_MASK
            } else {
                self.bits & !Inte::ALARM_1_MASK
            },
        }
    }

    /// Set **ALARM_0** `[0:0]`
    #[inline(always)]
    pub const fn with_alarm_0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Inte::ALARM_0_MASK
            } else {
                self.bits & !Inte::ALARM_0_MASK
            },
        }
    }
}

impl Default for InteVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Inte {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InteVal {
        InteVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: InteVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(InteVal) -> InteVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// INTF Register
///
/// Interrupt Force
///
/// Address offset: 0x0044
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **ALARM_3** `[3]` (RW)
///
/// - **ALARM_2** `[2]` (RW)
///
/// - **ALARM_1** `[1]` (RW)
///
/// - **ALARM_0** `[0]` (RW)
#[repr(transparent)]
pub struct Intf {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Intf {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Intf {
    // ======== Field Constants ========

    /// **ALARM_3**
    /// Bits `[3:3]`, mask `0x00000008`
    pub const ALARM_3_OFFSET: u32 = 3;
    pub const ALARM_3_MASK: u32 = 0x00000008;
    pub const ALARM_3: u32 = 1 << 3;

    /// **ALARM_2**
    /// Bits `[2:2]`, mask `0x00000004`
    pub const ALARM_2_OFFSET: u32 = 2;
    pub const ALARM_2_MASK: u32 = 0x00000004;
    pub const ALARM_2: u32 = 1 << 2;

    /// **ALARM_1**
    /// Bits `[1:1]`, mask `0x00000002`
    pub const ALARM_1_OFFSET: u32 = 1;
    pub const ALARM_1_MASK: u32 = 0x00000002;
    pub const ALARM_1: u32 = 1 << 1;

    /// **ALARM_0**
    /// Bits `[0:0]`, mask `0x00000001`
    pub const ALARM_0_OFFSET: u32 = 0;
    pub const ALARM_0_MASK: u32 = 0x00000001;
    pub const ALARM_0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ALARM_3** `[3]` (RW)
    #[inline(always)]
    pub fn alarm_3(&self) -> bool {
        (self.value.read_bits() & Self::ALARM_3_MASK) != 0
    }

    /// Set ALARM_3 bit atomically (RW)
    #[inline(always)]
    pub fn set_alarm_3(&self) {
        self.set_bits(Self::ALARM_3_MASK);
    }

    /// Clear ALARM_3 bit atomically (RW)
    #[inline(always)]
    pub fn clr_alarm_3(&self) {
        self.clr_bits(Self::ALARM_3_MASK);
    }

    /// **ALARM_2** `[2]` (RW)
    #[inline(always)]
    pub fn alarm_2(&self) -> bool {
        (self.value.read_bits() & Self::ALARM_2_MASK) != 0
    }

    /// Set ALARM_2 bit atomically (RW)
    #[inline(always)]
    pub fn set_alarm_2(&self) {
        self.set_bits(Self::ALARM_2_MASK);
    }

    /// Clear ALARM_2 bit atomically (RW)
    #[inline(always)]
    pub fn clr_alarm_2(&self) {
        self.clr_bits(Self::ALARM_2_MASK);
    }

    /// **ALARM_1** `[1]` (RW)
    #[inline(always)]
    pub fn alarm_1(&self) -> bool {
        (self.value.read_bits() & Self::ALARM_1_MASK) != 0
    }

    /// Set ALARM_1 bit atomically (RW)
    #[inline(always)]
    pub fn set_alarm_1(&self) {
        self.set_bits(Self::ALARM_1_MASK);
    }

    /// Clear ALARM_1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_alarm_1(&self) {
        self.clr_bits(Self::ALARM_1_MASK);
    }

    /// **ALARM_0** `[0]` (RW)
    #[inline(always)]
    pub fn alarm_0(&self) -> bool {
        (self.value.read_bits() & Self::ALARM_0_MASK) != 0
    }

    /// Set ALARM_0 bit atomically (RW)
    #[inline(always)]
    pub fn set_alarm_0(&self) {
        self.set_bits(Self::ALARM_0_MASK);
    }

    /// Clear ALARM_0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_alarm_0(&self) {
        self.clr_bits(Self::ALARM_0_MASK);
    }
}

/// INTF value type for batch read/modify/write operations
///
/// Use `intf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `intf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntfVal {
    bits: u32,
}

impl IntfVal {
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

    /// Read **ALARM_3** `[3:3]`
    #[inline(always)]
    pub const fn alarm_3(&self) -> bool {
        (self.bits & Intf::ALARM_3_MASK) != 0
    }

    /// Read **ALARM_2** `[2:2]`
    #[inline(always)]
    pub const fn alarm_2(&self) -> bool {
        (self.bits & Intf::ALARM_2_MASK) != 0
    }

    /// Read **ALARM_1** `[1:1]`
    #[inline(always)]
    pub const fn alarm_1(&self) -> bool {
        (self.bits & Intf::ALARM_1_MASK) != 0
    }

    /// Read **ALARM_0** `[0:0]`
    #[inline(always)]
    pub const fn alarm_0(&self) -> bool {
        (self.bits & Intf::ALARM_0_MASK) != 0
    }

    /// Set **ALARM_3** `[3:3]`
    #[inline(always)]
    pub const fn with_alarm_3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intf::ALARM_3_MASK
            } else {
                self.bits & !Intf::ALARM_3_MASK
            },
        }
    }

    /// Set **ALARM_2** `[2:2]`
    #[inline(always)]
    pub const fn with_alarm_2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intf::ALARM_2_MASK
            } else {
                self.bits & !Intf::ALARM_2_MASK
            },
        }
    }

    /// Set **ALARM_1** `[1:1]`
    #[inline(always)]
    pub const fn with_alarm_1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intf::ALARM_1_MASK
            } else {
                self.bits & !Intf::ALARM_1_MASK
            },
        }
    }

    /// Set **ALARM_0** `[0:0]`
    #[inline(always)]
    pub const fn with_alarm_0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intf::ALARM_0_MASK
            } else {
                self.bits & !Intf::ALARM_0_MASK
            },
        }
    }
}

impl Default for IntfVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Intf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IntfVal {
        IntfVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IntfVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IntfVal) -> IntfVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// INTS Register
///
/// Interrupt status after masking & forcing
///
/// Address offset: 0x0048
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **ALARM_3** `[3]` (RO)
///
/// - **ALARM_2** `[2]` (RO)
///
/// - **ALARM_1** `[1]` (RO)
///
/// - **ALARM_0** `[0]` (RO)
#[repr(transparent)]
pub struct Ints {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ints {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ints {
    // ======== Field Constants ========

    /// **ALARM_3**
    /// Bits `[3:3]`, mask `0x00000008`
    pub const ALARM_3_OFFSET: u32 = 3;
    pub const ALARM_3_MASK: u32 = 0x00000008;
    pub const ALARM_3: u32 = 1 << 3;

    /// **ALARM_2**
    /// Bits `[2:2]`, mask `0x00000004`
    pub const ALARM_2_OFFSET: u32 = 2;
    pub const ALARM_2_MASK: u32 = 0x00000004;
    pub const ALARM_2: u32 = 1 << 2;

    /// **ALARM_1**
    /// Bits `[1:1]`, mask `0x00000002`
    pub const ALARM_1_OFFSET: u32 = 1;
    pub const ALARM_1_MASK: u32 = 0x00000002;
    pub const ALARM_1: u32 = 1 << 1;

    /// **ALARM_0**
    /// Bits `[0:0]`, mask `0x00000001`
    pub const ALARM_0_OFFSET: u32 = 0;
    pub const ALARM_0_MASK: u32 = 0x00000001;
    pub const ALARM_0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ALARM_3** `[3]` (RO)
    #[inline(always)]
    pub fn alarm_3(&self) -> bool {
        (self.value.read_bits() & Self::ALARM_3_MASK) != 0
    }

    /// **ALARM_2** `[2]` (RO)
    #[inline(always)]
    pub fn alarm_2(&self) -> bool {
        (self.value.read_bits() & Self::ALARM_2_MASK) != 0
    }

    /// **ALARM_1** `[1]` (RO)
    #[inline(always)]
    pub fn alarm_1(&self) -> bool {
        (self.value.read_bits() & Self::ALARM_1_MASK) != 0
    }

    /// **ALARM_0** `[0]` (RO)
    #[inline(always)]
    pub fn alarm_0(&self) -> bool {
        (self.value.read_bits() & Self::ALARM_0_MASK) != 0
    }
}

/// INTS value type for batch read/modify/write operations
///
/// Use `ints.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ints.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntsVal {
    bits: u32,
}

impl IntsVal {
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

    /// Read **ALARM_3** `[3:3]`
    #[inline(always)]
    pub const fn alarm_3(&self) -> bool {
        (self.bits & Ints::ALARM_3_MASK) != 0
    }

    /// Read **ALARM_2** `[2:2]`
    #[inline(always)]
    pub const fn alarm_2(&self) -> bool {
        (self.bits & Ints::ALARM_2_MASK) != 0
    }

    /// Read **ALARM_1** `[1:1]`
    #[inline(always)]
    pub const fn alarm_1(&self) -> bool {
        (self.bits & Ints::ALARM_1_MASK) != 0
    }

    /// Read **ALARM_0** `[0:0]`
    #[inline(always)]
    pub const fn alarm_0(&self) -> bool {
        (self.bits & Ints::ALARM_0_MASK) != 0
    }
}

impl Default for IntsVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ints {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IntsVal {
        IntsVal::from_bits(self.value.read_bits())
    }
}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// Write to bits 63:32 of time always write timelw before timehw
    pub timehw: Timehw,
    /// 0x0004
    /// Write to bits 31:0 of time writes do not get copied to time until timehw is written
    pub timelw: Timelw,
    /// 0x0008
    /// Read from bits 63:32 of time always read timelr before timehr
    pub timehr: Timehr,
    /// 0x000c
    /// Read from bits 31:0 of time
    pub timelr: Timelr,
    /// 0x0010
    /// Arm alarm 0, and configure the time it will fire. Once armed, the alarm fires when TIMER_ALARM0 == TIMELR. The alarm will disarm itself once it fires, and can be disarmed early using the ARMED status register.
    pub alarm0: Alarm0,
    /// 0x0014
    /// Arm alarm 1, and configure the time it will fire. Once armed, the alarm fires when TIMER_ALARM1 == TIMELR. The alarm will disarm itself once it fires, and can be disarmed early using the ARMED status register.
    pub alarm1: Alarm1,
    /// 0x0018
    /// Arm alarm 2, and configure the time it will fire. Once armed, the alarm fires when TIMER_ALARM2 == TIMELR. The alarm will disarm itself once it fires, and can be disarmed early using the ARMED status register.
    pub alarm2: Alarm2,
    /// 0x001c
    /// Arm alarm 3, and configure the time it will fire. Once armed, the alarm fires when TIMER_ALARM3 == TIMELR. The alarm will disarm itself once it fires, and can be disarmed early using the ARMED status register.
    pub alarm3: Alarm3,
    /// 0x0020
    /// Indicates the armed/disarmed status of each alarm. A write to the corresponding ALARMx register arms the alarm. Alarms automatically disarm upon firing, but writing ones here will disarm immediately without waiting to fire.
    pub armed: Armed,
    /// 0x0024
    /// Raw read from bits 63:32 of time (no side effects)
    pub timerawh: Timerawh,
    /// 0x0028
    /// Raw read from bits 31:0 of time (no side effects)
    pub timerawl: Timerawl,
    /// 0x002c
    /// Set bits high to enable pause when the corresponding debug ports are active
    pub dbgpause: Dbgpause,
    /// 0x0030
    /// Set high to pause the timer
    pub pause: Pause,
    /// 0x0034
    /// Set locked bit to disable write access to timer Once set, cannot be cleared (without a reset)
    pub locked: Locked,
    /// 0x0038
    /// Selects the source for the timer. Defaults to the normal tick configured in the ticks block (typically configured to 1 microsecond). Writing to 1 will ignore the tick and count clk_sys cycles instead.
    pub source: Source,
    /// 0x003c
    /// Raw Interrupts
    pub intr: Intr,
    /// 0x0040
    /// Interrupt Enable
    pub inte: Inte,
    /// 0x0044
    /// Interrupt Force
    pub intf: Intf,
    /// 0x0048
    /// Interrupt status after masking & forcing
    pub ints: Ints,
}
