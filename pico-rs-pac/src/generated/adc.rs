//! ADC - Control and data interface to SAR ADC

#![allow(dead_code)]

pub const BASE: u32 = 0x400a0000;

/// CS Register
///
/// ADC Control and Status
///
/// Address offset: 0x0000
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **RROBIN** [24:16] (RW): Round-robin sampling. 1 bit per channel. Set all bits to 0 to disable. Othe...
/// - **AINSEL** [15:12] (RW): Select analog mux input. Updated automatically in round-robin mode. This is...
/// - **ERR_STICKY** [10] (RW): Some past ADC conversion encountered an error. Write 1 to clear.
/// - **ERR** [9] (RO): The most recent ADC conversion encountered an error; result is undefined or...
/// - **READY** [8] (RO): 1 if the ADC is ready to start a new conversion. Implies any previous conve...
/// - **START_MANY** [3] (RW): Continuously perform conversions whilst this bit is 1. A new conversion wil...
/// - **START_ONCE** [2] (WO): Start a single conversion. Self-clearing. Ignored if start_many is asserted.
/// - **TS_EN** [1] (RW): Power on temperature sensor. 1 - enabled. 0 - disabled.
/// - **EN** [0] (RW): Power on ADC and enable its clock. 1 - enabled. 0 - disabled.
#[repr(transparent)]
pub struct Cs {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for Cs {}
unsafe impl Sync for Cs {}

impl Cs {
    // ======== Field Constants ========

    /// RROBIN - Round-robin sampling. 1 bit per channel. Set all bits to 0 to disable. Othe...
    /// Bits [24:16], mask 0x01ff0000
    pub const RROBIN_OFFSET: u32 = 16;
    pub const RROBIN_MASK: u32 = 0x01ff0000;

    /// AINSEL - Select analog mux input. Updated automatically in round-robin mode. This is...
    /// Bits [15:12], mask 0x0000f000
    pub const AINSEL_OFFSET: u32 = 12;
    pub const AINSEL_MASK: u32 = 0x0000f000;

    /// ERR_STICKY - Some past ADC conversion encountered an error. Write 1 to clear.
    /// Bits [10:10], mask 0x00000400
    pub const ERR_STICKY_OFFSET: u32 = 10;
    pub const ERR_STICKY_MASK: u32 = 0x00000400;
    pub const ERR_STICKY: u32 = 1 << 10;

    /// ERR - The most recent ADC conversion encountered an error; result is undefined or...
    /// Bits [9:9], mask 0x00000200
    pub const ERR_OFFSET: u32 = 9;
    pub const ERR_MASK: u32 = 0x00000200;
    pub const ERR: u32 = 1 << 9;

    /// READY - 1 if the ADC is ready to start a new conversion. Implies any previous conve...
    /// Bits [8:8], mask 0x00000100
    pub const READY_OFFSET: u32 = 8;
    pub const READY_MASK: u32 = 0x00000100;
    pub const READY: u32 = 1 << 8;

    /// START_MANY - Continuously perform conversions whilst this bit is 1. A new conversion wil...
    /// Bits [3:3], mask 0x00000008
    pub const START_MANY_OFFSET: u32 = 3;
    pub const START_MANY_MASK: u32 = 0x00000008;
    pub const START_MANY: u32 = 1 << 3;

    /// START_ONCE - Start a single conversion. Self-clearing. Ignored if start_many is asserted.
    /// Bits [2:2], mask 0x00000004
    pub const START_ONCE_OFFSET: u32 = 2;
    pub const START_ONCE_MASK: u32 = 0x00000004;
    pub const START_ONCE: u32 = 1 << 2;

    /// TS_EN - Power on temperature sensor. 1 - enabled. 0 - disabled.
    /// Bits [1:1], mask 0x00000002
    pub const TS_EN_OFFSET: u32 = 1;
    pub const TS_EN_MASK: u32 = 0x00000002;
    pub const TS_EN: u32 = 1 << 1;

    /// EN - Power on ADC and enable its clock. 1 - enabled. 0 - disabled.
    /// Bits [0:0], mask 0x00000001
    pub const EN_OFFSET: u32 = 0;
    pub const EN_MASK: u32 = 0x00000001;
    pub const EN: u32 = 1 << 0;

    // ======== Basic Access Methods ========

    /// Read the raw register value as u32
    #[inline(always)]
    pub fn read_bits(&self) -> u32 {
        unsafe { core::ptr::read_volatile(self.value.get()) }
    }

    /// Write a raw u32 value to the register
    #[inline(always)]
    pub fn write_bits(&self, value: u32) {
        unsafe { core::ptr::write_volatile(self.value.get(), value) }
    }

    /// Modify the raw register value
    #[inline(always)]
    pub fn modify_bits<F: FnOnce(u32) -> u32>(&self, f: F) {
        self.write_bits(f(self.read_bits()));
    }

    /// Atomic SET - sets bits that are 1 (hardware alias at +0x2000)
    #[inline(always)]
    pub fn set_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x2000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic CLR - clears bits that are 1 (hardware alias at +0x3000)
    #[inline(always)]
    pub fn clr_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x3000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic XOR - toggles bits that are 1 (hardware alias at +0x1000)
    #[inline(always)]
    pub fn xor_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x1000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    // ======== Field Accessors ========

    /// **RROBIN** [24:16]: Round-robin sampling. 1 bit per channel. Set all bits to 0 to disable. Othe...
    #[inline(always)]
    pub fn rrobin(&self) -> u32 {
        (self.read_bits() & Self::RROBIN_MASK) >> Self::RROBIN_OFFSET
    }

    /// Write RROBIN field
    #[inline(always)]
    pub fn write_rrobin(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::RROBIN_MASK) | ((value << Self::RROBIN_OFFSET) & Self::RROBIN_MASK));
    }

    /// **AINSEL** [15:12]: Select analog mux input. Updated automatically in round-robin mode. This is...
    #[inline(always)]
    pub fn ainsel(&self) -> u32 {
        (self.read_bits() & Self::AINSEL_MASK) >> Self::AINSEL_OFFSET
    }

    /// Write AINSEL field
    #[inline(always)]
    pub fn write_ainsel(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::AINSEL_MASK) | ((value << Self::AINSEL_OFFSET) & Self::AINSEL_MASK));
    }

    /// **ERR_STICKY** [10:10]: Some past ADC conversion encountered an error. Write 1 to clear.
    #[inline(always)]
    pub fn err_sticky(&self) -> bool {
        (self.read_bits() & Self::ERR_STICKY_MASK) != 0
    }

    /// Set ERR_STICKY bit
    #[inline(always)]
    pub fn set_err_sticky(&self) {
        self.set_bits(Self::ERR_STICKY_MASK);
    }

    /// Clear ERR_STICKY bit
    #[inline(always)]
    pub fn clr_err_sticky(&self) {
        self.clr_bits(Self::ERR_STICKY_MASK);
    }

    /// **ERR** [9:9]: The most recent ADC conversion encountered an error; result is undefined or...
    #[inline(always)]
    pub fn err(&self) -> bool {
        (self.read_bits() & Self::ERR_MASK) != 0
    }

    /// **READY** [8:8]: 1 if the ADC is ready to start a new conversion. Implies any previous conve...
    #[inline(always)]
    pub fn ready(&self) -> bool {
        (self.read_bits() & Self::READY_MASK) != 0
    }

    /// **START_MANY** [3:3]: Continuously perform conversions whilst this bit is 1. A new conversion wil...
    #[inline(always)]
    pub fn start_many(&self) -> bool {
        (self.read_bits() & Self::START_MANY_MASK) != 0
    }

    /// Set START_MANY bit
    #[inline(always)]
    pub fn set_start_many(&self) {
        self.set_bits(Self::START_MANY_MASK);
    }

    /// Clear START_MANY bit
    #[inline(always)]
    pub fn clr_start_many(&self) {
        self.clr_bits(Self::START_MANY_MASK);
    }

    /// **START_ONCE** [2:2]: Start a single conversion. Self-clearing. Ignored if start_many is asserted.
    /// Trigger START_ONCE (write-only)
    #[inline(always)]
    pub fn trigger_start_once(&self) {
        self.write_bits(Self::START_ONCE_MASK);
    }

    /// **TS_EN** [1:1]: Power on temperature sensor. 1 - enabled. 0 - disabled.
    #[inline(always)]
    pub fn ts_en(&self) -> bool {
        (self.read_bits() & Self::TS_EN_MASK) != 0
    }

    /// Set TS_EN bit
    #[inline(always)]
    pub fn set_ts_en(&self) {
        self.set_bits(Self::TS_EN_MASK);
    }

    /// Clear TS_EN bit
    #[inline(always)]
    pub fn clr_ts_en(&self) {
        self.clr_bits(Self::TS_EN_MASK);
    }

    /// **EN** [0:0]: Power on ADC and enable its clock. 1 - enabled. 0 - disabled.
    #[inline(always)]
    pub fn en(&self) -> bool {
        (self.read_bits() & Self::EN_MASK) != 0
    }

    /// Set EN bit
    #[inline(always)]
    pub fn set_en(&self) {
        self.set_bits(Self::EN_MASK);
    }

    /// Clear EN bit
    #[inline(always)]
    pub fn clr_en(&self) {
        self.clr_bits(Self::EN_MASK);
    }

}

/// CS value type for batch read/modify/write operations
///
/// Use `cs.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `cs.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CsVal {
    bits: u32,
}

impl CsVal {
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

    /// Read RROBIN [24:16]: Round-robin sampling. 1 bit per channel. Set all bits to 0 to disable. Othe...
    #[inline(always)]
    pub const fn rrobin(&self) -> u32 {
        (self.bits & Cs::RROBIN_MASK) >> Cs::RROBIN_OFFSET
    }

    /// Read AINSEL [15:12]: Select analog mux input. Updated automatically in round-robin mode. This is...
    #[inline(always)]
    pub const fn ainsel(&self) -> u32 {
        (self.bits & Cs::AINSEL_MASK) >> Cs::AINSEL_OFFSET
    }

    /// Read ERR_STICKY [10:10]: Some past ADC conversion encountered an error. Write 1 to clear.
    #[inline(always)]
    pub const fn err_sticky(&self) -> bool {
        (self.bits & Cs::ERR_STICKY_MASK) != 0
    }

    /// Read ERR [9:9]: The most recent ADC conversion encountered an error; result is undefined or...
    #[inline(always)]
    pub const fn err(&self) -> bool {
        (self.bits & Cs::ERR_MASK) != 0
    }

    /// Read READY [8:8]: 1 if the ADC is ready to start a new conversion. Implies any previous conve...
    #[inline(always)]
    pub const fn ready(&self) -> bool {
        (self.bits & Cs::READY_MASK) != 0
    }

    /// Read START_MANY [3:3]: Continuously perform conversions whilst this bit is 1. A new conversion wil...
    #[inline(always)]
    pub const fn start_many(&self) -> bool {
        (self.bits & Cs::START_MANY_MASK) != 0
    }

    /// Read TS_EN [1:1]: Power on temperature sensor. 1 - enabled. 0 - disabled.
    #[inline(always)]
    pub const fn ts_en(&self) -> bool {
        (self.bits & Cs::TS_EN_MASK) != 0
    }

    /// Read EN [0:0]: Power on ADC and enable its clock. 1 - enabled. 0 - disabled.
    #[inline(always)]
    pub const fn en(&self) -> bool {
        (self.bits & Cs::EN_MASK) != 0
    }

    /// Set RROBIN [24:16]: Round-robin sampling. 1 bit per channel. Set all bits to 0 to disable. Othe...
    #[inline(always)]
    pub const fn with_rrobin(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cs::RROBIN_MASK) | ((value << Cs::RROBIN_OFFSET) & Cs::RROBIN_MASK)
        }
    }

    /// Set AINSEL [15:12]: Select analog mux input. Updated automatically in round-robin mode. This is...
    #[inline(always)]
    pub const fn with_ainsel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cs::AINSEL_MASK) | ((value << Cs::AINSEL_OFFSET) & Cs::AINSEL_MASK)
        }
    }

    /// Set ERR_STICKY [10:10]: Some past ADC conversion encountered an error. Write 1 to clear.
    #[inline(always)]
    pub const fn with_err_sticky(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cs::ERR_STICKY_MASK
            } else {
                self.bits & !Cs::ERR_STICKY_MASK
            }
        }
    }

    /// Set START_MANY [3:3]: Continuously perform conversions whilst this bit is 1. A new conversion wil...
    #[inline(always)]
    pub const fn with_start_many(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cs::START_MANY_MASK
            } else {
                self.bits & !Cs::START_MANY_MASK
            }
        }
    }

    /// Set START_ONCE [2:2]: Start a single conversion. Self-clearing. Ignored if start_many is asserted.
    #[inline(always)]
    pub const fn with_start_once(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cs::START_ONCE_MASK
            } else {
                self.bits & !Cs::START_ONCE_MASK
            }
        }
    }

    /// Set TS_EN [1:1]: Power on temperature sensor. 1 - enabled. 0 - disabled.
    #[inline(always)]
    pub const fn with_ts_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cs::TS_EN_MASK
            } else {
                self.bits & !Cs::TS_EN_MASK
            }
        }
    }

    /// Set EN [0:0]: Power on ADC and enable its clock. 1 - enabled. 0 - disabled.
    #[inline(always)]
    pub const fn with_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cs::EN_MASK
            } else {
                self.bits & !Cs::EN_MASK
            }
        }
    }

}

impl Default for CsVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Cs {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> CsVal {
        CsVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: CsVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(CsVal) -> CsVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// RESULT Register
///
/// Result of most recent ADC conversion
///
/// Address offset: 0x0004
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **RESULT** [11:0] (RO): 
#[repr(transparent)]
pub struct Result {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for Result {}
unsafe impl Sync for Result {}

impl Result {
    // ======== Field Constants ========

    /// Bits [11:0], mask 0x00000fff
    pub const RESULT_OFFSET: u32 = 0;
    pub const RESULT_MASK: u32 = 0x00000fff;

    // ======== Basic Access Methods ========

    /// Read the raw register value as u32
    #[inline(always)]
    pub fn read_bits(&self) -> u32 {
        unsafe { core::ptr::read_volatile(self.value.get()) }
    }

    /// Write a raw u32 value to the register
    #[inline(always)]
    pub fn write_bits(&self, value: u32) {
        unsafe { core::ptr::write_volatile(self.value.get(), value) }
    }

    /// Modify the raw register value
    #[inline(always)]
    pub fn modify_bits<F: FnOnce(u32) -> u32>(&self, f: F) {
        self.write_bits(f(self.read_bits()));
    }

    /// Atomic SET - sets bits that are 1 (hardware alias at +0x2000)
    #[inline(always)]
    pub fn set_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x2000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic CLR - clears bits that are 1 (hardware alias at +0x3000)
    #[inline(always)]
    pub fn clr_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x3000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic XOR - toggles bits that are 1 (hardware alias at +0x1000)
    #[inline(always)]
    pub fn xor_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x1000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    // ======== Field Accessors ========

    /// **RESULT** [11:0]
    #[inline(always)]
    pub fn result(&self) -> u32 {
        (self.read_bits() & Self::RESULT_MASK) >> Self::RESULT_OFFSET
    }

}

/// RESULT value type for batch read/modify/write operations
///
/// Use `result.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `result.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ResultVal {
    bits: u32,
}

impl ResultVal {
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

    /// Read RESULT [11:0]
    #[inline(always)]
    pub const fn result(&self) -> u32 {
        (self.bits & Result::RESULT_MASK) >> Result::RESULT_OFFSET
    }

}

impl Default for ResultVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Result {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> ResultVal {
        ResultVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: ResultVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(ResultVal) -> ResultVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// FCS Register
///
/// FIFO control and status
///
/// Address offset: 0x0008
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **THRESH** [27:24] (RW): DREQ/IRQ asserted when level >= threshold
/// - **LEVEL** [19:16] (RO): The number of conversion results currently waiting in the FIFO
/// - **OVER** [11] (RW): 1 if the FIFO has been overflowed. Write 1 to clear.
/// - **UNDER** [10] (RW): 1 if the FIFO has been underflowed. Write 1 to clear.
/// - **FULL** [9] (RO): 
/// - **EMPTY** [8] (RO): 
/// - **DREQ_EN** [3] (RW): If 1: assert DMA requests when FIFO contains data
/// - **ERR** [2] (RW): If 1: conversion error bit appears in the FIFO alongside the result
/// - **SHIFT** [1] (RW): If 1: FIFO results are right-shifted to be one byte in size. Enables DMA to...
/// - **EN** [0] (RW): If 1: write result to the FIFO after each conversion.
#[repr(transparent)]
pub struct Fcs {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for Fcs {}
unsafe impl Sync for Fcs {}

impl Fcs {
    // ======== Field Constants ========

    /// THRESH - DREQ/IRQ asserted when level >= threshold
    /// Bits [27:24], mask 0x0f000000
    pub const THRESH_OFFSET: u32 = 24;
    pub const THRESH_MASK: u32 = 0x0f000000;

    /// LEVEL - The number of conversion results currently waiting in the FIFO
    /// Bits [19:16], mask 0x000f0000
    pub const LEVEL_OFFSET: u32 = 16;
    pub const LEVEL_MASK: u32 = 0x000f0000;

    /// OVER - 1 if the FIFO has been overflowed. Write 1 to clear.
    /// Bits [11:11], mask 0x00000800
    pub const OVER_OFFSET: u32 = 11;
    pub const OVER_MASK: u32 = 0x00000800;
    pub const OVER: u32 = 1 << 11;

    /// UNDER - 1 if the FIFO has been underflowed. Write 1 to clear.
    /// Bits [10:10], mask 0x00000400
    pub const UNDER_OFFSET: u32 = 10;
    pub const UNDER_MASK: u32 = 0x00000400;
    pub const UNDER: u32 = 1 << 10;

    /// Bits [9:9], mask 0x00000200
    pub const FULL_OFFSET: u32 = 9;
    pub const FULL_MASK: u32 = 0x00000200;
    pub const FULL: u32 = 1 << 9;

    /// Bits [8:8], mask 0x00000100
    pub const EMPTY_OFFSET: u32 = 8;
    pub const EMPTY_MASK: u32 = 0x00000100;
    pub const EMPTY: u32 = 1 << 8;

    /// DREQ_EN - If 1: assert DMA requests when FIFO contains data
    /// Bits [3:3], mask 0x00000008
    pub const DREQ_EN_OFFSET: u32 = 3;
    pub const DREQ_EN_MASK: u32 = 0x00000008;
    pub const DREQ_EN: u32 = 1 << 3;

    /// ERR - If 1: conversion error bit appears in the FIFO alongside the result
    /// Bits [2:2], mask 0x00000004
    pub const ERR_OFFSET: u32 = 2;
    pub const ERR_MASK: u32 = 0x00000004;
    pub const ERR: u32 = 1 << 2;

    /// SHIFT - If 1: FIFO results are right-shifted to be one byte in size. Enables DMA to...
    /// Bits [1:1], mask 0x00000002
    pub const SHIFT_OFFSET: u32 = 1;
    pub const SHIFT_MASK: u32 = 0x00000002;
    pub const SHIFT: u32 = 1 << 1;

    /// EN - If 1: write result to the FIFO after each conversion.
    /// Bits [0:0], mask 0x00000001
    pub const EN_OFFSET: u32 = 0;
    pub const EN_MASK: u32 = 0x00000001;
    pub const EN: u32 = 1 << 0;

    // ======== Basic Access Methods ========

    /// Read the raw register value as u32
    #[inline(always)]
    pub fn read_bits(&self) -> u32 {
        unsafe { core::ptr::read_volatile(self.value.get()) }
    }

    /// Write a raw u32 value to the register
    #[inline(always)]
    pub fn write_bits(&self, value: u32) {
        unsafe { core::ptr::write_volatile(self.value.get(), value) }
    }

    /// Modify the raw register value
    #[inline(always)]
    pub fn modify_bits<F: FnOnce(u32) -> u32>(&self, f: F) {
        self.write_bits(f(self.read_bits()));
    }

    /// Atomic SET - sets bits that are 1 (hardware alias at +0x2000)
    #[inline(always)]
    pub fn set_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x2000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic CLR - clears bits that are 1 (hardware alias at +0x3000)
    #[inline(always)]
    pub fn clr_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x3000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic XOR - toggles bits that are 1 (hardware alias at +0x1000)
    #[inline(always)]
    pub fn xor_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x1000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    // ======== Field Accessors ========

    /// **THRESH** [27:24]: DREQ/IRQ asserted when level >= threshold
    #[inline(always)]
    pub fn thresh(&self) -> u32 {
        (self.read_bits() & Self::THRESH_MASK) >> Self::THRESH_OFFSET
    }

    /// Write THRESH field
    #[inline(always)]
    pub fn write_thresh(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::THRESH_MASK) | ((value << Self::THRESH_OFFSET) & Self::THRESH_MASK));
    }

    /// **LEVEL** [19:16]: The number of conversion results currently waiting in the FIFO
    #[inline(always)]
    pub fn level(&self) -> u32 {
        (self.read_bits() & Self::LEVEL_MASK) >> Self::LEVEL_OFFSET
    }

    /// **OVER** [11:11]: 1 if the FIFO has been overflowed. Write 1 to clear.
    #[inline(always)]
    pub fn over(&self) -> bool {
        (self.read_bits() & Self::OVER_MASK) != 0
    }

    /// Set OVER bit
    #[inline(always)]
    pub fn set_over(&self) {
        self.set_bits(Self::OVER_MASK);
    }

    /// Clear OVER bit
    #[inline(always)]
    pub fn clr_over(&self) {
        self.clr_bits(Self::OVER_MASK);
    }

    /// **UNDER** [10:10]: 1 if the FIFO has been underflowed. Write 1 to clear.
    #[inline(always)]
    pub fn under(&self) -> bool {
        (self.read_bits() & Self::UNDER_MASK) != 0
    }

    /// Set UNDER bit
    #[inline(always)]
    pub fn set_under(&self) {
        self.set_bits(Self::UNDER_MASK);
    }

    /// Clear UNDER bit
    #[inline(always)]
    pub fn clr_under(&self) {
        self.clr_bits(Self::UNDER_MASK);
    }

    /// **FULL** [9:9]
    #[inline(always)]
    pub fn full(&self) -> bool {
        (self.read_bits() & Self::FULL_MASK) != 0
    }

    /// **EMPTY** [8:8]
    #[inline(always)]
    pub fn empty(&self) -> bool {
        (self.read_bits() & Self::EMPTY_MASK) != 0
    }

    /// **DREQ_EN** [3:3]: If 1: assert DMA requests when FIFO contains data
    #[inline(always)]
    pub fn dreq_en(&self) -> bool {
        (self.read_bits() & Self::DREQ_EN_MASK) != 0
    }

    /// Set DREQ_EN bit
    #[inline(always)]
    pub fn set_dreq_en(&self) {
        self.set_bits(Self::DREQ_EN_MASK);
    }

    /// Clear DREQ_EN bit
    #[inline(always)]
    pub fn clr_dreq_en(&self) {
        self.clr_bits(Self::DREQ_EN_MASK);
    }

    /// **ERR** [2:2]: If 1: conversion error bit appears in the FIFO alongside the result
    #[inline(always)]
    pub fn err(&self) -> bool {
        (self.read_bits() & Self::ERR_MASK) != 0
    }

    /// Set ERR bit
    #[inline(always)]
    pub fn set_err(&self) {
        self.set_bits(Self::ERR_MASK);
    }

    /// Clear ERR bit
    #[inline(always)]
    pub fn clr_err(&self) {
        self.clr_bits(Self::ERR_MASK);
    }

    /// **SHIFT** [1:1]: If 1: FIFO results are right-shifted to be one byte in size. Enables DMA to...
    #[inline(always)]
    pub fn shift(&self) -> bool {
        (self.read_bits() & Self::SHIFT_MASK) != 0
    }

    /// Set SHIFT bit
    #[inline(always)]
    pub fn set_shift(&self) {
        self.set_bits(Self::SHIFT_MASK);
    }

    /// Clear SHIFT bit
    #[inline(always)]
    pub fn clr_shift(&self) {
        self.clr_bits(Self::SHIFT_MASK);
    }

    /// **EN** [0:0]: If 1: write result to the FIFO after each conversion.
    #[inline(always)]
    pub fn en(&self) -> bool {
        (self.read_bits() & Self::EN_MASK) != 0
    }

    /// Set EN bit
    #[inline(always)]
    pub fn set_en(&self) {
        self.set_bits(Self::EN_MASK);
    }

    /// Clear EN bit
    #[inline(always)]
    pub fn clr_en(&self) {
        self.clr_bits(Self::EN_MASK);
    }

}

/// FCS value type for batch read/modify/write operations
///
/// Use `fcs.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `fcs.write_val(val)`.
#[derive(Clone, Copy)]
pub struct FcsVal {
    bits: u32,
}

impl FcsVal {
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

    /// Read THRESH [27:24]: DREQ/IRQ asserted when level >= threshold
    #[inline(always)]
    pub const fn thresh(&self) -> u32 {
        (self.bits & Fcs::THRESH_MASK) >> Fcs::THRESH_OFFSET
    }

    /// Read LEVEL [19:16]: The number of conversion results currently waiting in the FIFO
    #[inline(always)]
    pub const fn level(&self) -> u32 {
        (self.bits & Fcs::LEVEL_MASK) >> Fcs::LEVEL_OFFSET
    }

    /// Read OVER [11:11]: 1 if the FIFO has been overflowed. Write 1 to clear.
    #[inline(always)]
    pub const fn over(&self) -> bool {
        (self.bits & Fcs::OVER_MASK) != 0
    }

    /// Read UNDER [10:10]: 1 if the FIFO has been underflowed. Write 1 to clear.
    #[inline(always)]
    pub const fn under(&self) -> bool {
        (self.bits & Fcs::UNDER_MASK) != 0
    }

    /// Read FULL [9:9]
    #[inline(always)]
    pub const fn full(&self) -> bool {
        (self.bits & Fcs::FULL_MASK) != 0
    }

    /// Read EMPTY [8:8]
    #[inline(always)]
    pub const fn empty(&self) -> bool {
        (self.bits & Fcs::EMPTY_MASK) != 0
    }

    /// Read DREQ_EN [3:3]: If 1: assert DMA requests when FIFO contains data
    #[inline(always)]
    pub const fn dreq_en(&self) -> bool {
        (self.bits & Fcs::DREQ_EN_MASK) != 0
    }

    /// Read ERR [2:2]: If 1: conversion error bit appears in the FIFO alongside the result
    #[inline(always)]
    pub const fn err(&self) -> bool {
        (self.bits & Fcs::ERR_MASK) != 0
    }

    /// Read SHIFT [1:1]: If 1: FIFO results are right-shifted to be one byte in size. Enables DMA to...
    #[inline(always)]
    pub const fn shift(&self) -> bool {
        (self.bits & Fcs::SHIFT_MASK) != 0
    }

    /// Read EN [0:0]: If 1: write result to the FIFO after each conversion.
    #[inline(always)]
    pub const fn en(&self) -> bool {
        (self.bits & Fcs::EN_MASK) != 0
    }

    /// Set THRESH [27:24]: DREQ/IRQ asserted when level >= threshold
    #[inline(always)]
    pub const fn with_thresh(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Fcs::THRESH_MASK) | ((value << Fcs::THRESH_OFFSET) & Fcs::THRESH_MASK)
        }
    }

    /// Set OVER [11:11]: 1 if the FIFO has been overflowed. Write 1 to clear.
    #[inline(always)]
    pub const fn with_over(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Fcs::OVER_MASK
            } else {
                self.bits & !Fcs::OVER_MASK
            }
        }
    }

    /// Set UNDER [10:10]: 1 if the FIFO has been underflowed. Write 1 to clear.
    #[inline(always)]
    pub const fn with_under(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Fcs::UNDER_MASK
            } else {
                self.bits & !Fcs::UNDER_MASK
            }
        }
    }

    /// Set DREQ_EN [3:3]: If 1: assert DMA requests when FIFO contains data
    #[inline(always)]
    pub const fn with_dreq_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Fcs::DREQ_EN_MASK
            } else {
                self.bits & !Fcs::DREQ_EN_MASK
            }
        }
    }

    /// Set ERR [2:2]: If 1: conversion error bit appears in the FIFO alongside the result
    #[inline(always)]
    pub const fn with_err(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Fcs::ERR_MASK
            } else {
                self.bits & !Fcs::ERR_MASK
            }
        }
    }

    /// Set SHIFT [1:1]: If 1: FIFO results are right-shifted to be one byte in size. Enables DMA to...
    #[inline(always)]
    pub const fn with_shift(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Fcs::SHIFT_MASK
            } else {
                self.bits & !Fcs::SHIFT_MASK
            }
        }
    }

    /// Set EN [0:0]: If 1: write result to the FIFO after each conversion.
    #[inline(always)]
    pub const fn with_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Fcs::EN_MASK
            } else {
                self.bits & !Fcs::EN_MASK
            }
        }
    }

}

impl Default for FcsVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Fcs {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> FcsVal {
        FcsVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: FcsVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(FcsVal) -> FcsVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// FIFO Register
///
/// Conversion result FIFO
///
/// Address offset: 0x000c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **ERR** [15] (RO): 1 if this particular sample experienced a conversion error. Remains in the ...
/// - **VAL** [11:0] (RO): 
#[repr(transparent)]
pub struct Fifo {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for Fifo {}
unsafe impl Sync for Fifo {}

impl Fifo {
    // ======== Field Constants ========

    /// ERR - 1 if this particular sample experienced a conversion error. Remains in the ...
    /// Bits [15:15], mask 0x00008000
    pub const ERR_OFFSET: u32 = 15;
    pub const ERR_MASK: u32 = 0x00008000;
    pub const ERR: u32 = 1 << 15;

    /// Bits [11:0], mask 0x00000fff
    pub const VAL_OFFSET: u32 = 0;
    pub const VAL_MASK: u32 = 0x00000fff;

    // ======== Basic Access Methods ========

    /// Read the raw register value as u32
    #[inline(always)]
    pub fn read_bits(&self) -> u32 {
        unsafe { core::ptr::read_volatile(self.value.get()) }
    }

    /// Write a raw u32 value to the register
    #[inline(always)]
    pub fn write_bits(&self, value: u32) {
        unsafe { core::ptr::write_volatile(self.value.get(), value) }
    }

    /// Modify the raw register value
    #[inline(always)]
    pub fn modify_bits<F: FnOnce(u32) -> u32>(&self, f: F) {
        self.write_bits(f(self.read_bits()));
    }

    /// Atomic SET - sets bits that are 1 (hardware alias at +0x2000)
    #[inline(always)]
    pub fn set_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x2000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic CLR - clears bits that are 1 (hardware alias at +0x3000)
    #[inline(always)]
    pub fn clr_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x3000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic XOR - toggles bits that are 1 (hardware alias at +0x1000)
    #[inline(always)]
    pub fn xor_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x1000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    // ======== Field Accessors ========

    /// **ERR** [15:15]: 1 if this particular sample experienced a conversion error. Remains in the ...
    #[inline(always)]
    pub fn err(&self) -> bool {
        (self.read_bits() & Self::ERR_MASK) != 0
    }

    /// **VAL** [11:0]
    #[inline(always)]
    pub fn val(&self) -> u32 {
        (self.read_bits() & Self::VAL_MASK) >> Self::VAL_OFFSET
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

    /// Read ERR [15:15]: 1 if this particular sample experienced a conversion error. Remains in the ...
    #[inline(always)]
    pub const fn err(&self) -> bool {
        (self.bits & Fifo::ERR_MASK) != 0
    }

    /// Read VAL [11:0]
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        (self.bits & Fifo::VAL_MASK) >> Fifo::VAL_OFFSET
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
    pub fn read(&self) -> FifoVal {
        FifoVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: FifoVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(FifoVal) -> FifoVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// DIV Register
///
/// Clock divider. If non-zero, CS_START_MANY will start conversions at regular
/// intervals rather than back-to-back. The divider is reset when either of
/// these fields are written. Total period is 1 + INT + FRAC / 256
///
/// Address offset: 0x0010
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **INT** [23:8] (RW): Integer part of clock divisor.
/// - **FRAC** [7:0] (RW): Fractional part of clock divisor. First-order delta-sigma.
#[repr(transparent)]
pub struct Div {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for Div {}
unsafe impl Sync for Div {}

impl Div {
    // ======== Field Constants ========

    /// INT - Integer part of clock divisor.
    /// Bits [23:8], mask 0x00ffff00
    pub const INT_OFFSET: u32 = 8;
    pub const INT_MASK: u32 = 0x00ffff00;

    /// FRAC - Fractional part of clock divisor. First-order delta-sigma.
    /// Bits [7:0], mask 0x000000ff
    pub const FRAC_OFFSET: u32 = 0;
    pub const FRAC_MASK: u32 = 0x000000ff;

    // ======== Basic Access Methods ========

    /// Read the raw register value as u32
    #[inline(always)]
    pub fn read_bits(&self) -> u32 {
        unsafe { core::ptr::read_volatile(self.value.get()) }
    }

    /// Write a raw u32 value to the register
    #[inline(always)]
    pub fn write_bits(&self, value: u32) {
        unsafe { core::ptr::write_volatile(self.value.get(), value) }
    }

    /// Modify the raw register value
    #[inline(always)]
    pub fn modify_bits<F: FnOnce(u32) -> u32>(&self, f: F) {
        self.write_bits(f(self.read_bits()));
    }

    /// Atomic SET - sets bits that are 1 (hardware alias at +0x2000)
    #[inline(always)]
    pub fn set_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x2000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic CLR - clears bits that are 1 (hardware alias at +0x3000)
    #[inline(always)]
    pub fn clr_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x3000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic XOR - toggles bits that are 1 (hardware alias at +0x1000)
    #[inline(always)]
    pub fn xor_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x1000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    // ======== Field Accessors ========

    /// **INT** [23:8]: Integer part of clock divisor.
    #[inline(always)]
    pub fn int(&self) -> u32 {
        (self.read_bits() & Self::INT_MASK) >> Self::INT_OFFSET
    }

    /// Write INT field
    #[inline(always)]
    pub fn write_int(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::INT_MASK) | ((value << Self::INT_OFFSET) & Self::INT_MASK));
    }

    /// **FRAC** [7:0]: Fractional part of clock divisor. First-order delta-sigma.
    #[inline(always)]
    pub fn frac(&self) -> u32 {
        (self.read_bits() & Self::FRAC_MASK) >> Self::FRAC_OFFSET
    }

    /// Write FRAC field
    #[inline(always)]
    pub fn write_frac(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::FRAC_MASK) | ((value << Self::FRAC_OFFSET) & Self::FRAC_MASK));
    }

}

/// DIV value type for batch read/modify/write operations
///
/// Use `div.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `div.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DivVal {
    bits: u32,
}

impl DivVal {
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

    /// Read INT [23:8]: Integer part of clock divisor.
    #[inline(always)]
    pub const fn int(&self) -> u32 {
        (self.bits & Div::INT_MASK) >> Div::INT_OFFSET
    }

    /// Read FRAC [7:0]: Fractional part of clock divisor. First-order delta-sigma.
    #[inline(always)]
    pub const fn frac(&self) -> u32 {
        (self.bits & Div::FRAC_MASK) >> Div::FRAC_OFFSET
    }

    /// Set INT [23:8]: Integer part of clock divisor.
    #[inline(always)]
    pub const fn with_int(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Div::INT_MASK) | ((value << Div::INT_OFFSET) & Div::INT_MASK)
        }
    }

    /// Set FRAC [7:0]: Fractional part of clock divisor. First-order delta-sigma.
    #[inline(always)]
    pub const fn with_frac(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Div::FRAC_MASK) | ((value << Div::FRAC_OFFSET) & Div::FRAC_MASK)
        }
    }

}

impl Default for DivVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Div {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> DivVal {
        DivVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: DivVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(DivVal) -> DivVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// INTR Register
///
/// Raw Interrupts
///
/// Address offset: 0x0014
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **FIFO** [0] (RO): Triggered when the sample FIFO reaches a certain level. This level can be p...
#[repr(transparent)]
pub struct Intr {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for Intr {}
unsafe impl Sync for Intr {}

impl Intr {
    // ======== Field Constants ========

    /// FIFO - Triggered when the sample FIFO reaches a certain level. This level can be p...
    /// Bits [0:0], mask 0x00000001
    pub const FIFO_OFFSET: u32 = 0;
    pub const FIFO_MASK: u32 = 0x00000001;
    pub const FIFO: u32 = 1 << 0;

    // ======== Basic Access Methods ========

    /// Read the raw register value as u32
    #[inline(always)]
    pub fn read_bits(&self) -> u32 {
        unsafe { core::ptr::read_volatile(self.value.get()) }
    }

    /// Write a raw u32 value to the register
    #[inline(always)]
    pub fn write_bits(&self, value: u32) {
        unsafe { core::ptr::write_volatile(self.value.get(), value) }
    }

    /// Modify the raw register value
    #[inline(always)]
    pub fn modify_bits<F: FnOnce(u32) -> u32>(&self, f: F) {
        self.write_bits(f(self.read_bits()));
    }

    /// Atomic SET - sets bits that are 1 (hardware alias at +0x2000)
    #[inline(always)]
    pub fn set_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x2000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic CLR - clears bits that are 1 (hardware alias at +0x3000)
    #[inline(always)]
    pub fn clr_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x3000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic XOR - toggles bits that are 1 (hardware alias at +0x1000)
    #[inline(always)]
    pub fn xor_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x1000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    // ======== Field Accessors ========

    /// **FIFO** [0:0]: Triggered when the sample FIFO reaches a certain level. This level can be p...
    #[inline(always)]
    pub fn fifo(&self) -> bool {
        (self.read_bits() & Self::FIFO_MASK) != 0
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

    /// Read FIFO [0:0]: Triggered when the sample FIFO reaches a certain level. This level can be p...
    #[inline(always)]
    pub const fn fifo(&self) -> bool {
        (self.bits & Intr::FIFO_MASK) != 0
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
    pub fn read(&self) -> IntrVal {
        IntrVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: IntrVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(IntrVal) -> IntrVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// INTE Register
///
/// Interrupt Enable
///
/// Address offset: 0x0018
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **FIFO** [0] (RW): Triggered when the sample FIFO reaches a certain level. This level can be p...
#[repr(transparent)]
pub struct Inte {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for Inte {}
unsafe impl Sync for Inte {}

impl Inte {
    // ======== Field Constants ========

    /// FIFO - Triggered when the sample FIFO reaches a certain level. This level can be p...
    /// Bits [0:0], mask 0x00000001
    pub const FIFO_OFFSET: u32 = 0;
    pub const FIFO_MASK: u32 = 0x00000001;
    pub const FIFO: u32 = 1 << 0;

    // ======== Basic Access Methods ========

    /// Read the raw register value as u32
    #[inline(always)]
    pub fn read_bits(&self) -> u32 {
        unsafe { core::ptr::read_volatile(self.value.get()) }
    }

    /// Write a raw u32 value to the register
    #[inline(always)]
    pub fn write_bits(&self, value: u32) {
        unsafe { core::ptr::write_volatile(self.value.get(), value) }
    }

    /// Modify the raw register value
    #[inline(always)]
    pub fn modify_bits<F: FnOnce(u32) -> u32>(&self, f: F) {
        self.write_bits(f(self.read_bits()));
    }

    /// Atomic SET - sets bits that are 1 (hardware alias at +0x2000)
    #[inline(always)]
    pub fn set_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x2000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic CLR - clears bits that are 1 (hardware alias at +0x3000)
    #[inline(always)]
    pub fn clr_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x3000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic XOR - toggles bits that are 1 (hardware alias at +0x1000)
    #[inline(always)]
    pub fn xor_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x1000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    // ======== Field Accessors ========

    /// **FIFO** [0:0]: Triggered when the sample FIFO reaches a certain level. This level can be p...
    #[inline(always)]
    pub fn fifo(&self) -> bool {
        (self.read_bits() & Self::FIFO_MASK) != 0
    }

    /// Set FIFO bit
    #[inline(always)]
    pub fn set_fifo(&self) {
        self.set_bits(Self::FIFO_MASK);
    }

    /// Clear FIFO bit
    #[inline(always)]
    pub fn clr_fifo(&self) {
        self.clr_bits(Self::FIFO_MASK);
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

    /// Read FIFO [0:0]: Triggered when the sample FIFO reaches a certain level. This level can be p...
    #[inline(always)]
    pub const fn fifo(&self) -> bool {
        (self.bits & Inte::FIFO_MASK) != 0
    }

    /// Set FIFO [0:0]: Triggered when the sample FIFO reaches a certain level. This level can be p...
    #[inline(always)]
    pub const fn with_fifo(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Inte::FIFO_MASK
            } else {
                self.bits & !Inte::FIFO_MASK
            }
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
    pub fn read(&self) -> InteVal {
        InteVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: InteVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(InteVal) -> InteVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// INTF Register
///
/// Interrupt Force
///
/// Address offset: 0x001c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **FIFO** [0] (RW): Triggered when the sample FIFO reaches a certain level. This level can be p...
#[repr(transparent)]
pub struct Intf {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for Intf {}
unsafe impl Sync for Intf {}

impl Intf {
    // ======== Field Constants ========

    /// FIFO - Triggered when the sample FIFO reaches a certain level. This level can be p...
    /// Bits [0:0], mask 0x00000001
    pub const FIFO_OFFSET: u32 = 0;
    pub const FIFO_MASK: u32 = 0x00000001;
    pub const FIFO: u32 = 1 << 0;

    // ======== Basic Access Methods ========

    /// Read the raw register value as u32
    #[inline(always)]
    pub fn read_bits(&self) -> u32 {
        unsafe { core::ptr::read_volatile(self.value.get()) }
    }

    /// Write a raw u32 value to the register
    #[inline(always)]
    pub fn write_bits(&self, value: u32) {
        unsafe { core::ptr::write_volatile(self.value.get(), value) }
    }

    /// Modify the raw register value
    #[inline(always)]
    pub fn modify_bits<F: FnOnce(u32) -> u32>(&self, f: F) {
        self.write_bits(f(self.read_bits()));
    }

    /// Atomic SET - sets bits that are 1 (hardware alias at +0x2000)
    #[inline(always)]
    pub fn set_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x2000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic CLR - clears bits that are 1 (hardware alias at +0x3000)
    #[inline(always)]
    pub fn clr_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x3000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic XOR - toggles bits that are 1 (hardware alias at +0x1000)
    #[inline(always)]
    pub fn xor_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x1000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    // ======== Field Accessors ========

    /// **FIFO** [0:0]: Triggered when the sample FIFO reaches a certain level. This level can be p...
    #[inline(always)]
    pub fn fifo(&self) -> bool {
        (self.read_bits() & Self::FIFO_MASK) != 0
    }

    /// Set FIFO bit
    #[inline(always)]
    pub fn set_fifo(&self) {
        self.set_bits(Self::FIFO_MASK);
    }

    /// Clear FIFO bit
    #[inline(always)]
    pub fn clr_fifo(&self) {
        self.clr_bits(Self::FIFO_MASK);
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

    /// Read FIFO [0:0]: Triggered when the sample FIFO reaches a certain level. This level can be p...
    #[inline(always)]
    pub const fn fifo(&self) -> bool {
        (self.bits & Intf::FIFO_MASK) != 0
    }

    /// Set FIFO [0:0]: Triggered when the sample FIFO reaches a certain level. This level can be p...
    #[inline(always)]
    pub const fn with_fifo(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intf::FIFO_MASK
            } else {
                self.bits & !Intf::FIFO_MASK
            }
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
    pub fn read(&self) -> IntfVal {
        IntfVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: IntfVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(IntfVal) -> IntfVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// INTS Register
///
/// Interrupt status after masking & forcing
///
/// Address offset: 0x0020
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **FIFO** [0] (RO): Triggered when the sample FIFO reaches a certain level. This level can be p...
#[repr(transparent)]
pub struct Ints {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for Ints {}
unsafe impl Sync for Ints {}

impl Ints {
    // ======== Field Constants ========

    /// FIFO - Triggered when the sample FIFO reaches a certain level. This level can be p...
    /// Bits [0:0], mask 0x00000001
    pub const FIFO_OFFSET: u32 = 0;
    pub const FIFO_MASK: u32 = 0x00000001;
    pub const FIFO: u32 = 1 << 0;

    // ======== Basic Access Methods ========

    /// Read the raw register value as u32
    #[inline(always)]
    pub fn read_bits(&self) -> u32 {
        unsafe { core::ptr::read_volatile(self.value.get()) }
    }

    /// Write a raw u32 value to the register
    #[inline(always)]
    pub fn write_bits(&self, value: u32) {
        unsafe { core::ptr::write_volatile(self.value.get(), value) }
    }

    /// Modify the raw register value
    #[inline(always)]
    pub fn modify_bits<F: FnOnce(u32) -> u32>(&self, f: F) {
        self.write_bits(f(self.read_bits()));
    }

    /// Atomic SET - sets bits that are 1 (hardware alias at +0x2000)
    #[inline(always)]
    pub fn set_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x2000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic CLR - clears bits that are 1 (hardware alias at +0x3000)
    #[inline(always)]
    pub fn clr_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x3000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic XOR - toggles bits that are 1 (hardware alias at +0x1000)
    #[inline(always)]
    pub fn xor_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x1000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    // ======== Field Accessors ========

    /// **FIFO** [0:0]: Triggered when the sample FIFO reaches a certain level. This level can be p...
    #[inline(always)]
    pub fn fifo(&self) -> bool {
        (self.read_bits() & Self::FIFO_MASK) != 0
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

    /// Read FIFO [0:0]: Triggered when the sample FIFO reaches a certain level. This level can be p...
    #[inline(always)]
    pub const fn fifo(&self) -> bool {
        (self.bits & Ints::FIFO_MASK) != 0
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
    pub fn read(&self) -> IntsVal {
        IntsVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: IntsVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(IntsVal) -> IntsVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000 - ADC Control and Status
    pub cs: Cs,
    /// 0x0004 - Result of most recent ADC conversion
    pub result: Result,
    /// 0x0008 - FIFO control and status
    pub fcs: Fcs,
    /// 0x000c - Conversion result FIFO
    pub fifo: Fifo,
    /// 0x0010 - Clock divider. If non-zero, CS_START_MANY will start conversions at regular...
    pub div: Div,
    /// 0x0014 - Raw Interrupts
    pub intr: Intr,
    /// 0x0018 - Interrupt Enable
    pub inte: Inte,
    /// 0x001c - Interrupt Force
    pub intf: Intf,
    /// 0x0020 - Interrupt status after masking & forcing
    pub ints: Ints,
}

