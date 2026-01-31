//! WATCHDOG

#![allow(dead_code)]

pub const BASE: u32 = 0x400d8000;

/// CTRL Register
///
/// Watchdog control
/// The rst_wdsel register determines which subsystems are reset when the watchdog is triggered.
/// The watchdog can be triggered in software.
///
/// Address offset: 0x0000
/// Reset value: 0x07000000
///
/// ## Fields
///
/// - **TRIGGER** `[31]` (WO)
///   Trigger a watchdog reset
/// - **ENABLE** `[30]` (RW)
///   When not enabled the watchdog timer is paused
/// - **PAUSE_DBG1** `[26]` (RW)
///   Pause the watchdog timer when processor 1 is in debug mode
/// - **PAUSE_DBG0** `[25]` (RW)
///   Pause the watchdog timer when processor 0 is in debug mode
/// - **PAUSE_JTAG** `[24]` (RW)
///   Pause the watchdog timer when JTAG is accessing the bus fabric
/// - **TIME** `[23:0]` (RO)
///   Indicates the time in usec before a watchdog reset will be triggered
#[repr(transparent)]
pub struct Ctrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ctrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ctrl {
    // ======== Field Constants ========

    /// **TRIGGER**
    /// Trigger a watchdog reset
    /// Bits `[31:31]`, mask `0x80000000`
    pub const TRIGGER_OFFSET: u32 = 31;
    pub const TRIGGER_MASK: u32 = 0x80000000;
    pub const TRIGGER: u32 = 1 << 31;

    /// **ENABLE**
    /// When not enabled the watchdog timer is paused
    /// Bits `[30:30]`, mask `0x40000000`
    pub const ENABLE_OFFSET: u32 = 30;
    pub const ENABLE_MASK: u32 = 0x40000000;
    pub const ENABLE: u32 = 1 << 30;

    /// **PAUSE_DBG1**
    /// Pause the watchdog timer when processor 1 is in debug mode
    /// Bits `[26:26]`, mask `0x04000000`
    pub const PAUSE_DBG1_OFFSET: u32 = 26;
    pub const PAUSE_DBG1_MASK: u32 = 0x04000000;
    pub const PAUSE_DBG1: u32 = 1 << 26;

    /// **PAUSE_DBG0**
    /// Pause the watchdog timer when processor 0 is in debug mode
    /// Bits `[25:25]`, mask `0x02000000`
    pub const PAUSE_DBG0_OFFSET: u32 = 25;
    pub const PAUSE_DBG0_MASK: u32 = 0x02000000;
    pub const PAUSE_DBG0: u32 = 1 << 25;

    /// **PAUSE_JTAG**
    /// Pause the watchdog timer when JTAG is accessing the bus fabric
    /// Bits `[24:24]`, mask `0x01000000`
    pub const PAUSE_JTAG_OFFSET: u32 = 24;
    pub const PAUSE_JTAG_MASK: u32 = 0x01000000;
    pub const PAUSE_JTAG: u32 = 1 << 24;

    /// **TIME**
    /// Indicates the time in usec before a watchdog reset will be triggered
    /// Bits `[23:0]`, mask `0x00ffffff`
    pub const TIME_OFFSET: u32 = 0;
    pub const TIME_MASK: u32 = 0x00ffffff;

    // ======== Field Accessors ========

    /// **TRIGGER** `[31]` (WO)
    /// Trigger a watchdog reset
    /// Trigger **TRIGGER** (WO)
    /// Trigger a watchdog reset
    #[inline(always)]
    pub fn trigger_trigger(&self) {
        self.value.write_bits(Self::TRIGGER_MASK);
    }

    /// **ENABLE** `[30]` (RW)
    /// When not enabled the watchdog timer is paused
    #[inline(always)]
    pub fn enable(&self) -> bool {
        (self.value.read_bits() & Self::ENABLE_MASK) != 0
    }

    /// Set ENABLE bit atomically (RW)
    #[inline(always)]
    pub fn set_enable(&self) {
        self.value.set_bits(Self::ENABLE_MASK);
    }

    /// Clear ENABLE bit atomically (RW)
    #[inline(always)]
    pub fn clr_enable(&self) {
        self.value.clr_bits(Self::ENABLE_MASK);
    }

    /// **PAUSE_DBG1** `[26]` (RW)
    /// Pause the watchdog timer when processor 1 is in debug mode
    #[inline(always)]
    pub fn pause_dbg1(&self) -> bool {
        (self.value.read_bits() & Self::PAUSE_DBG1_MASK) != 0
    }

    /// Set PAUSE_DBG1 bit atomically (RW)
    #[inline(always)]
    pub fn set_pause_dbg1(&self) {
        self.value.set_bits(Self::PAUSE_DBG1_MASK);
    }

    /// Clear PAUSE_DBG1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_pause_dbg1(&self) {
        self.value.clr_bits(Self::PAUSE_DBG1_MASK);
    }

    /// **PAUSE_DBG0** `[25]` (RW)
    /// Pause the watchdog timer when processor 0 is in debug mode
    #[inline(always)]
    pub fn pause_dbg0(&self) -> bool {
        (self.value.read_bits() & Self::PAUSE_DBG0_MASK) != 0
    }

    /// Set PAUSE_DBG0 bit atomically (RW)
    #[inline(always)]
    pub fn set_pause_dbg0(&self) {
        self.value.set_bits(Self::PAUSE_DBG0_MASK);
    }

    /// Clear PAUSE_DBG0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_pause_dbg0(&self) {
        self.value.clr_bits(Self::PAUSE_DBG0_MASK);
    }

    /// **PAUSE_JTAG** `[24]` (RW)
    /// Pause the watchdog timer when JTAG is accessing the bus fabric
    #[inline(always)]
    pub fn pause_jtag(&self) -> bool {
        (self.value.read_bits() & Self::PAUSE_JTAG_MASK) != 0
    }

    /// Set PAUSE_JTAG bit atomically (RW)
    #[inline(always)]
    pub fn set_pause_jtag(&self) {
        self.value.set_bits(Self::PAUSE_JTAG_MASK);
    }

    /// Clear PAUSE_JTAG bit atomically (RW)
    #[inline(always)]
    pub fn clr_pause_jtag(&self) {
        self.value.clr_bits(Self::PAUSE_JTAG_MASK);
    }

    /// **TIME** `[23:0]` (RO)
    /// Indicates the time in usec before a watchdog reset will be triggered
    #[inline(always)]
    pub fn time(&self) -> u32 {
        (self.value.read_bits() & Self::TIME_MASK) >> Self::TIME_OFFSET
    }

}

/// CTRL value type for batch read/modify/write operations
///
/// Use `ctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CtrlVal {
    bits: u32,
}

impl CtrlVal {
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
        Self { bits: 0x07000000 }
    }

    /// Read **ENABLE** `[30:30]`
    /// When not enabled the watchdog timer is paused
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        (self.bits & Ctrl::ENABLE_MASK) != 0
    }

    /// Read **PAUSE_DBG1** `[26:26]`
    /// Pause the watchdog timer when processor 1 is in debug mode
    #[inline(always)]
    pub const fn pause_dbg1(&self) -> bool {
        (self.bits & Ctrl::PAUSE_DBG1_MASK) != 0
    }

    /// Read **PAUSE_DBG0** `[25:25]`
    /// Pause the watchdog timer when processor 0 is in debug mode
    #[inline(always)]
    pub const fn pause_dbg0(&self) -> bool {
        (self.bits & Ctrl::PAUSE_DBG0_MASK) != 0
    }

    /// Read **PAUSE_JTAG** `[24:24]`
    /// Pause the watchdog timer when JTAG is accessing the bus fabric
    #[inline(always)]
    pub const fn pause_jtag(&self) -> bool {
        (self.bits & Ctrl::PAUSE_JTAG_MASK) != 0
    }

    /// Read **TIME** `[23:0]`
    /// Indicates the time in usec before a watchdog reset will be triggered
    #[inline(always)]
    pub const fn time(&self) -> u32 {
        (self.bits & Ctrl::TIME_MASK) >> Ctrl::TIME_OFFSET
    }

    /// Set **TRIGGER** `[31:31]`
    /// Trigger a watchdog reset
    #[inline(always)]
    pub const fn with_trigger(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctrl::TRIGGER_MASK
            } else {
                self.bits & !Ctrl::TRIGGER_MASK
            }
        }
    }

    /// Set **ENABLE** `[30:30]`
    /// When not enabled the watchdog timer is paused
    #[inline(always)]
    pub const fn with_enable(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctrl::ENABLE_MASK
            } else {
                self.bits & !Ctrl::ENABLE_MASK
            }
        }
    }

    /// Set **PAUSE_DBG1** `[26:26]`
    /// Pause the watchdog timer when processor 1 is in debug mode
    #[inline(always)]
    pub const fn with_pause_dbg1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctrl::PAUSE_DBG1_MASK
            } else {
                self.bits & !Ctrl::PAUSE_DBG1_MASK
            }
        }
    }

    /// Set **PAUSE_DBG0** `[25:25]`
    /// Pause the watchdog timer when processor 0 is in debug mode
    #[inline(always)]
    pub const fn with_pause_dbg0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctrl::PAUSE_DBG0_MASK
            } else {
                self.bits & !Ctrl::PAUSE_DBG0_MASK
            }
        }
    }

    /// Set **PAUSE_JTAG** `[24:24]`
    /// Pause the watchdog timer when JTAG is accessing the bus fabric
    #[inline(always)]
    pub const fn with_pause_jtag(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctrl::PAUSE_JTAG_MASK
            } else {
                self.bits & !Ctrl::PAUSE_JTAG_MASK
            }
        }
    }

}

impl Default for CtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ctrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CtrlVal {
        CtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CtrlVal) -> CtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// LOAD Register
///
/// Load the watchdog timer. The maximum setting is 0xffffff which corresponds to approximately 16 seconds.
///
/// Address offset: 0x0004
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **LOAD** `[23:0]` (WO)
#[repr(transparent)]
pub struct Load {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Load {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Load {
    // ======== Field Constants ========

    /// **LOAD**
    /// Bits `[23:0]`, mask `0x00ffffff`
    pub const LOAD_OFFSET: u32 = 0;
    pub const LOAD_MASK: u32 = 0x00ffffff;

    // ======== Field Accessors ========

    /// **LOAD** `[23:0]` (WO)
    /// Write **LOAD** `[23:0]` (WO)
    #[inline(always)]
    pub fn write_load(&self, value: u32) {
        self.value.write_bits((value << Self::LOAD_OFFSET) & Self::LOAD_MASK);
    }

}

/// LOAD value type for batch read/modify/write operations
///
/// Use `load.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `load.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LoadVal {
    bits: u32,
}

impl LoadVal {
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

    /// Set **LOAD** `[23:0]`
    #[inline(always)]
    pub const fn with_load(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Load::LOAD_MASK) | ((value << Load::LOAD_OFFSET) & Load::LOAD_MASK)
        }
    }

}

impl Default for LoadVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Load {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LoadVal {
        LoadVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LoadVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LoadVal) -> LoadVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// REASON Register
///
/// Logs the reason for the last reset. Both bits are zero for the case of a hardware reset.
/// Additionally, as of RP2350, a debugger warm reset of either core (SYSRESETREQ or hartreset) will also clear the watchdog reason register, so that software loaded under the debugger following a watchdog timeout will not continue to see the timeout condition.
///
/// Address offset: 0x0008
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **FORCE** `[1]` (RO)
/// - **TIMER** `[0]` (RO)
#[repr(transparent)]
pub struct Reason {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Reason {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Reason {
    // ======== Field Constants ========

    /// **FORCE**
    /// Bits `[1:1]`, mask `0x00000002`
    pub const FORCE_OFFSET: u32 = 1;
    pub const FORCE_MASK: u32 = 0x00000002;
    pub const FORCE: u32 = 1 << 1;

    /// **TIMER**
    /// Bits `[0:0]`, mask `0x00000001`
    pub const TIMER_OFFSET: u32 = 0;
    pub const TIMER_MASK: u32 = 0x00000001;
    pub const TIMER: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **FORCE** `[1]` (RO)
    #[inline(always)]
    pub fn force(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_MASK) != 0
    }

    /// **TIMER** `[0]` (RO)
    #[inline(always)]
    pub fn timer(&self) -> bool {
        (self.value.read_bits() & Self::TIMER_MASK) != 0
    }

}

/// REASON value type for batch read/modify/write operations
///
/// Use `reason.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `reason.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ReasonVal {
    bits: u32,
}

impl ReasonVal {
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

    /// Read **FORCE** `[1:1]`
    #[inline(always)]
    pub const fn force(&self) -> bool {
        (self.bits & Reason::FORCE_MASK) != 0
    }

    /// Read **TIMER** `[0:0]`
    #[inline(always)]
    pub const fn timer(&self) -> bool {
        (self.bits & Reason::TIMER_MASK) != 0
    }

}

impl Default for ReasonVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Reason {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ReasonVal {
        ReasonVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ReasonVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ReasonVal) -> ReasonVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SCRATCH0 Register
///
/// Scratch register. Information persists through soft reset of the chip.
///
/// Address offset: 0x000c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SCRATCH0** `[31:0]` (RW)
#[repr(transparent)]
pub struct Scratch0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Scratch0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Scratch0 {
    // ======== Field Constants ========

    /// **SCRATCH0**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SCRATCH0_OFFSET: u32 = 0;
    pub const SCRATCH0_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SCRATCH0** `[31:0]` (RW)
    #[inline(always)]
    pub fn scratch0(&self) -> u32 {
        (self.value.read_bits() & Self::SCRATCH0_MASK) >> Self::SCRATCH0_OFFSET
    }

    /// Write **SCRATCH0** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_scratch0(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SCRATCH0_MASK) | ((value << Self::SCRATCH0_OFFSET) & Self::SCRATCH0_MASK));
    }

}

/// SCRATCH0 value type for batch read/modify/write operations
///
/// Use `scratch0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `scratch0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Scratch0Val {
    bits: u32,
}

impl Scratch0Val {
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

    /// Read **SCRATCH0** `[31:0]`
    #[inline(always)]
    pub const fn scratch0(&self) -> u32 {
        (self.bits & Scratch0::SCRATCH0_MASK) >> Scratch0::SCRATCH0_OFFSET
    }

    /// Set **SCRATCH0** `[31:0]`
    #[inline(always)]
    pub const fn with_scratch0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Scratch0::SCRATCH0_MASK) | ((value << Scratch0::SCRATCH0_OFFSET) & Scratch0::SCRATCH0_MASK)
        }
    }

}

impl Default for Scratch0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Scratch0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Scratch0Val {
        Scratch0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Scratch0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Scratch0Val) -> Scratch0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SCRATCH1 Register
///
/// Scratch register. Information persists through soft reset of the chip.
///
/// Address offset: 0x0010
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SCRATCH1** `[31:0]` (RW)
#[repr(transparent)]
pub struct Scratch1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Scratch1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Scratch1 {
    // ======== Field Constants ========

    /// **SCRATCH1**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SCRATCH1_OFFSET: u32 = 0;
    pub const SCRATCH1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SCRATCH1** `[31:0]` (RW)
    #[inline(always)]
    pub fn scratch1(&self) -> u32 {
        (self.value.read_bits() & Self::SCRATCH1_MASK) >> Self::SCRATCH1_OFFSET
    }

    /// Write **SCRATCH1** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_scratch1(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SCRATCH1_MASK) | ((value << Self::SCRATCH1_OFFSET) & Self::SCRATCH1_MASK));
    }

}

/// SCRATCH1 value type for batch read/modify/write operations
///
/// Use `scratch1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `scratch1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Scratch1Val {
    bits: u32,
}

impl Scratch1Val {
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

    /// Read **SCRATCH1** `[31:0]`
    #[inline(always)]
    pub const fn scratch1(&self) -> u32 {
        (self.bits & Scratch1::SCRATCH1_MASK) >> Scratch1::SCRATCH1_OFFSET
    }

    /// Set **SCRATCH1** `[31:0]`
    #[inline(always)]
    pub const fn with_scratch1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Scratch1::SCRATCH1_MASK) | ((value << Scratch1::SCRATCH1_OFFSET) & Scratch1::SCRATCH1_MASK)
        }
    }

}

impl Default for Scratch1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Scratch1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Scratch1Val {
        Scratch1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Scratch1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Scratch1Val) -> Scratch1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SCRATCH2 Register
///
/// Scratch register. Information persists through soft reset of the chip.
///
/// Address offset: 0x0014
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SCRATCH2** `[31:0]` (RW)
#[repr(transparent)]
pub struct Scratch2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Scratch2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Scratch2 {
    // ======== Field Constants ========

    /// **SCRATCH2**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SCRATCH2_OFFSET: u32 = 0;
    pub const SCRATCH2_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SCRATCH2** `[31:0]` (RW)
    #[inline(always)]
    pub fn scratch2(&self) -> u32 {
        (self.value.read_bits() & Self::SCRATCH2_MASK) >> Self::SCRATCH2_OFFSET
    }

    /// Write **SCRATCH2** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_scratch2(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SCRATCH2_MASK) | ((value << Self::SCRATCH2_OFFSET) & Self::SCRATCH2_MASK));
    }

}

/// SCRATCH2 value type for batch read/modify/write operations
///
/// Use `scratch2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `scratch2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Scratch2Val {
    bits: u32,
}

impl Scratch2Val {
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

    /// Read **SCRATCH2** `[31:0]`
    #[inline(always)]
    pub const fn scratch2(&self) -> u32 {
        (self.bits & Scratch2::SCRATCH2_MASK) >> Scratch2::SCRATCH2_OFFSET
    }

    /// Set **SCRATCH2** `[31:0]`
    #[inline(always)]
    pub const fn with_scratch2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Scratch2::SCRATCH2_MASK) | ((value << Scratch2::SCRATCH2_OFFSET) & Scratch2::SCRATCH2_MASK)
        }
    }

}

impl Default for Scratch2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Scratch2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Scratch2Val {
        Scratch2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Scratch2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Scratch2Val) -> Scratch2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SCRATCH3 Register
///
/// Scratch register. Information persists through soft reset of the chip.
///
/// Address offset: 0x0018
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SCRATCH3** `[31:0]` (RW)
#[repr(transparent)]
pub struct Scratch3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Scratch3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Scratch3 {
    // ======== Field Constants ========

    /// **SCRATCH3**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SCRATCH3_OFFSET: u32 = 0;
    pub const SCRATCH3_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SCRATCH3** `[31:0]` (RW)
    #[inline(always)]
    pub fn scratch3(&self) -> u32 {
        (self.value.read_bits() & Self::SCRATCH3_MASK) >> Self::SCRATCH3_OFFSET
    }

    /// Write **SCRATCH3** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_scratch3(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SCRATCH3_MASK) | ((value << Self::SCRATCH3_OFFSET) & Self::SCRATCH3_MASK));
    }

}

/// SCRATCH3 value type for batch read/modify/write operations
///
/// Use `scratch3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `scratch3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Scratch3Val {
    bits: u32,
}

impl Scratch3Val {
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

    /// Read **SCRATCH3** `[31:0]`
    #[inline(always)]
    pub const fn scratch3(&self) -> u32 {
        (self.bits & Scratch3::SCRATCH3_MASK) >> Scratch3::SCRATCH3_OFFSET
    }

    /// Set **SCRATCH3** `[31:0]`
    #[inline(always)]
    pub const fn with_scratch3(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Scratch3::SCRATCH3_MASK) | ((value << Scratch3::SCRATCH3_OFFSET) & Scratch3::SCRATCH3_MASK)
        }
    }

}

impl Default for Scratch3Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Scratch3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Scratch3Val {
        Scratch3Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Scratch3Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Scratch3Val) -> Scratch3Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SCRATCH4 Register
///
/// Scratch register. Information persists through soft reset of the chip.
///
/// Address offset: 0x001c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SCRATCH4** `[31:0]` (RW)
#[repr(transparent)]
pub struct Scratch4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Scratch4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Scratch4 {
    // ======== Field Constants ========

    /// **SCRATCH4**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SCRATCH4_OFFSET: u32 = 0;
    pub const SCRATCH4_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SCRATCH4** `[31:0]` (RW)
    #[inline(always)]
    pub fn scratch4(&self) -> u32 {
        (self.value.read_bits() & Self::SCRATCH4_MASK) >> Self::SCRATCH4_OFFSET
    }

    /// Write **SCRATCH4** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_scratch4(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SCRATCH4_MASK) | ((value << Self::SCRATCH4_OFFSET) & Self::SCRATCH4_MASK));
    }

}

/// SCRATCH4 value type for batch read/modify/write operations
///
/// Use `scratch4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `scratch4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Scratch4Val {
    bits: u32,
}

impl Scratch4Val {
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

    /// Read **SCRATCH4** `[31:0]`
    #[inline(always)]
    pub const fn scratch4(&self) -> u32 {
        (self.bits & Scratch4::SCRATCH4_MASK) >> Scratch4::SCRATCH4_OFFSET
    }

    /// Set **SCRATCH4** `[31:0]`
    #[inline(always)]
    pub const fn with_scratch4(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Scratch4::SCRATCH4_MASK) | ((value << Scratch4::SCRATCH4_OFFSET) & Scratch4::SCRATCH4_MASK)
        }
    }

}

impl Default for Scratch4Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Scratch4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Scratch4Val {
        Scratch4Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Scratch4Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Scratch4Val) -> Scratch4Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SCRATCH5 Register
///
/// Scratch register. Information persists through soft reset of the chip.
///
/// Address offset: 0x0020
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SCRATCH5** `[31:0]` (RW)
#[repr(transparent)]
pub struct Scratch5 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Scratch5 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Scratch5 {
    // ======== Field Constants ========

    /// **SCRATCH5**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SCRATCH5_OFFSET: u32 = 0;
    pub const SCRATCH5_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SCRATCH5** `[31:0]` (RW)
    #[inline(always)]
    pub fn scratch5(&self) -> u32 {
        (self.value.read_bits() & Self::SCRATCH5_MASK) >> Self::SCRATCH5_OFFSET
    }

    /// Write **SCRATCH5** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_scratch5(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SCRATCH5_MASK) | ((value << Self::SCRATCH5_OFFSET) & Self::SCRATCH5_MASK));
    }

}

/// SCRATCH5 value type for batch read/modify/write operations
///
/// Use `scratch5.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `scratch5.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Scratch5Val {
    bits: u32,
}

impl Scratch5Val {
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

    /// Read **SCRATCH5** `[31:0]`
    #[inline(always)]
    pub const fn scratch5(&self) -> u32 {
        (self.bits & Scratch5::SCRATCH5_MASK) >> Scratch5::SCRATCH5_OFFSET
    }

    /// Set **SCRATCH5** `[31:0]`
    #[inline(always)]
    pub const fn with_scratch5(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Scratch5::SCRATCH5_MASK) | ((value << Scratch5::SCRATCH5_OFFSET) & Scratch5::SCRATCH5_MASK)
        }
    }

}

impl Default for Scratch5Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Scratch5 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Scratch5Val {
        Scratch5Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Scratch5Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Scratch5Val) -> Scratch5Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SCRATCH6 Register
///
/// Scratch register. Information persists through soft reset of the chip.
///
/// Address offset: 0x0024
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SCRATCH6** `[31:0]` (RW)
#[repr(transparent)]
pub struct Scratch6 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Scratch6 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Scratch6 {
    // ======== Field Constants ========

    /// **SCRATCH6**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SCRATCH6_OFFSET: u32 = 0;
    pub const SCRATCH6_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SCRATCH6** `[31:0]` (RW)
    #[inline(always)]
    pub fn scratch6(&self) -> u32 {
        (self.value.read_bits() & Self::SCRATCH6_MASK) >> Self::SCRATCH6_OFFSET
    }

    /// Write **SCRATCH6** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_scratch6(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SCRATCH6_MASK) | ((value << Self::SCRATCH6_OFFSET) & Self::SCRATCH6_MASK));
    }

}

/// SCRATCH6 value type for batch read/modify/write operations
///
/// Use `scratch6.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `scratch6.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Scratch6Val {
    bits: u32,
}

impl Scratch6Val {
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

    /// Read **SCRATCH6** `[31:0]`
    #[inline(always)]
    pub const fn scratch6(&self) -> u32 {
        (self.bits & Scratch6::SCRATCH6_MASK) >> Scratch6::SCRATCH6_OFFSET
    }

    /// Set **SCRATCH6** `[31:0]`
    #[inline(always)]
    pub const fn with_scratch6(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Scratch6::SCRATCH6_MASK) | ((value << Scratch6::SCRATCH6_OFFSET) & Scratch6::SCRATCH6_MASK)
        }
    }

}

impl Default for Scratch6Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Scratch6 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Scratch6Val {
        Scratch6Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Scratch6Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Scratch6Val) -> Scratch6Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SCRATCH7 Register
///
/// Scratch register. Information persists through soft reset of the chip.
///
/// Address offset: 0x0028
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SCRATCH7** `[31:0]` (RW)
#[repr(transparent)]
pub struct Scratch7 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Scratch7 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Scratch7 {
    // ======== Field Constants ========

    /// **SCRATCH7**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SCRATCH7_OFFSET: u32 = 0;
    pub const SCRATCH7_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SCRATCH7** `[31:0]` (RW)
    #[inline(always)]
    pub fn scratch7(&self) -> u32 {
        (self.value.read_bits() & Self::SCRATCH7_MASK) >> Self::SCRATCH7_OFFSET
    }

    /// Write **SCRATCH7** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_scratch7(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SCRATCH7_MASK) | ((value << Self::SCRATCH7_OFFSET) & Self::SCRATCH7_MASK));
    }

}

/// SCRATCH7 value type for batch read/modify/write operations
///
/// Use `scratch7.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `scratch7.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Scratch7Val {
    bits: u32,
}

impl Scratch7Val {
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

    /// Read **SCRATCH7** `[31:0]`
    #[inline(always)]
    pub const fn scratch7(&self) -> u32 {
        (self.bits & Scratch7::SCRATCH7_MASK) >> Scratch7::SCRATCH7_OFFSET
    }

    /// Set **SCRATCH7** `[31:0]`
    #[inline(always)]
    pub const fn with_scratch7(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Scratch7::SCRATCH7_MASK) | ((value << Scratch7::SCRATCH7_OFFSET) & Scratch7::SCRATCH7_MASK)
        }
    }

}

impl Default for Scratch7Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Scratch7 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Scratch7Val {
        Scratch7Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Scratch7Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Scratch7Val) -> Scratch7Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// Watchdog control
    /// The rst_wdsel register determines which subsystems are reset when the watchdog is triggered.
    /// The watchdog can be triggered in software.
    pub ctrl: Ctrl,
    /// 0x0004
    /// Load the watchdog timer. The maximum setting is 0xffffff which corresponds to approximately 16 seconds.
    pub load: Load,
    /// 0x0008
    /// Logs the reason for the last reset. Both bits are zero for the case of a hardware reset.
    /// Additionally, as of RP2350, a debugger warm reset of either core (SYSRESETREQ or hartreset) will also clear the watchdog reason register, so that software loaded under the debugger following a watchdog timeout will not continue to see the timeout condition.
    pub reason: Reason,
    /// 0x000c
    /// Scratch register. Information persists through soft reset of the chip.
    pub scratch0: Scratch0,
    /// 0x0010
    /// Scratch register. Information persists through soft reset of the chip.
    pub scratch1: Scratch1,
    /// 0x0014
    /// Scratch register. Information persists through soft reset of the chip.
    pub scratch2: Scratch2,
    /// 0x0018
    /// Scratch register. Information persists through soft reset of the chip.
    pub scratch3: Scratch3,
    /// 0x001c
    /// Scratch register. Information persists through soft reset of the chip.
    pub scratch4: Scratch4,
    /// 0x0020
    /// Scratch register. Information persists through soft reset of the chip.
    pub scratch5: Scratch5,
    /// 0x0024
    /// Scratch register. Information persists through soft reset of the chip.
    pub scratch6: Scratch6,
    /// 0x0028
    /// Scratch register. Information persists through soft reset of the chip.
    pub scratch7: Scratch7,
}

