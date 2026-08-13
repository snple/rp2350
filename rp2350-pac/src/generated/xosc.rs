//! XOSC
//! Controls the crystal oscillator
//!
//! This peripheral supports atomic SET/CLR/XOR register aliases.

#![allow(dead_code)]

pub const BASE: u32 = 0x40048000;

/// CTRL Register
///
/// Crystal Oscillator Control
///
/// Address offset: 0x0000
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **ENABLE** `[23:12]` (RW)
///
///   On power-up this field is initialised to DISABLE and the chip runs from the ROSC.
///   If the chip has subsequently been programmed to run from the XOSC then setting this field to DISABLE may lock-up the chip. If this is a concern then run the clk_ref from the ROSC and enable the clk_sys RESUS feature.
///   The 12-bit code is intended to give some protection against accidental writes. An invalid setting will retain the previous value. The actual value being used can be read from STATUS_ENABLED
///
/// - **FREQ_RANGE** `[11:0]` (RW)
///
///   The 12-bit code is intended to give some protection against accidental writes. An invalid setting will retain the previous value. The actual value being used can be read from STATUS_FREQ_RANGE
#[repr(transparent)]
pub struct Ctrl {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Ctrl {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ctrl {
    // ======== Field Constants ========

    /// **ENABLE**
    ///
    /// On power-up this field is initialised to DISABLE and the chip runs from the ROSC.
    /// If the chip has subsequently been programmed to run from the XOSC then setting this field to DISABLE may lock-up the chip. If this is a concern then run the clk_ref from the ROSC and enable the clk_sys RESUS feature.
    /// The 12-bit code is intended to give some protection against accidental writes. An invalid setting will retain the previous value. The actual value being used can be read from STATUS_ENABLED
    ///
    /// Bits `[23:12]`, mask `0x00fff000`
    pub const ENABLE_OFFSET: u32 = 12;
    pub const ENABLE_MASK: u32 = 0x00fff000;
    pub const ENABLE_DISABLE: u32 = 3358 << 12;
    pub const ENABLE_DISABLE_VAL: u32 = 3358;
    pub const ENABLE_ENABLE: u32 = 4011 << 12;
    pub const ENABLE_ENABLE_VAL: u32 = 4011;

    /// **FREQ_RANGE**
    ///
    /// The 12-bit code is intended to give some protection against accidental writes. An invalid setting will retain the previous value. The actual value being used can be read from STATUS_FREQ_RANGE
    ///
    /// Bits `[11:0]`, mask `0x00000fff`
    pub const FREQ_RANGE_OFFSET: u32 = 0;
    pub const FREQ_RANGE_MASK: u32 = 0x00000fff;
    pub const FREQ_RANGE_1_15MHZ: u32 = 2720 << 0;
    pub const FREQ_RANGE_1_15MHZ_VAL: u32 = 2720;
    pub const FREQ_RANGE_10_30MHZ: u32 = 2721 << 0;
    pub const FREQ_RANGE_10_30MHZ_VAL: u32 = 2721;
    pub const FREQ_RANGE_25_60MHZ: u32 = 2722 << 0;
    pub const FREQ_RANGE_25_60MHZ_VAL: u32 = 2722;
    pub const FREQ_RANGE_40_100MHZ: u32 = 2723 << 0;
    pub const FREQ_RANGE_40_100MHZ_VAL: u32 = 2723;

    // ======== Field Accessors ========

    /// **ENABLE** `[23:12]` (RW)
    ///
    /// On power-up this field is initialised to DISABLE and the chip runs from the ROSC.
    /// If the chip has subsequently been programmed to run from the XOSC then setting this field to DISABLE may lock-up the chip. If this is a concern then run the clk_ref from the ROSC and enable the clk_sys RESUS feature.
    /// The 12-bit code is intended to give some protection against accidental writes. An invalid setting will retain the previous value. The actual value being used can be read from STATUS_ENABLED
    #[inline(always)]
    pub fn enable(&self) -> u32 {
        (self.value.read_bits() & Self::ENABLE_MASK) >> Self::ENABLE_OFFSET
    }

    /// Write **ENABLE** `[23:12]` (RW)
    ///
    /// On power-up this field is initialised to DISABLE and the chip runs from the ROSC.
    /// If the chip has subsequently been programmed to run from the XOSC then setting this field to DISABLE may lock-up the chip. If this is a concern then run the clk_ref from the ROSC and enable the clk_sys RESUS feature.
    /// The 12-bit code is intended to give some protection against accidental writes. An invalid setting will retain the previous value. The actual value being used can be read from STATUS_ENABLED
    #[inline(always)]
    pub fn write_enable(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ENABLE_MASK) | ((value << Self::ENABLE_OFFSET) & Self::ENABLE_MASK)
        });
    }

    /// **FREQ_RANGE** `[11:0]` (RW)
    ///
    /// The 12-bit code is intended to give some protection against accidental writes. An invalid setting will retain the previous value. The actual value being used can be read from STATUS_FREQ_RANGE
    #[inline(always)]
    pub fn freq_range(&self) -> u32 {
        (self.value.read_bits() & Self::FREQ_RANGE_MASK) >> Self::FREQ_RANGE_OFFSET
    }

    /// Write **FREQ_RANGE** `[11:0]` (RW)
    ///
    /// The 12-bit code is intended to give some protection against accidental writes. An invalid setting will retain the previous value. The actual value being used can be read from STATUS_FREQ_RANGE
    #[inline(always)]
    pub fn write_freq_range(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::FREQ_RANGE_MASK)
                | ((value << Self::FREQ_RANGE_OFFSET) & Self::FREQ_RANGE_MASK)
        });
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
        Self { bits: 0x00000000 }
    }

    /// Read **ENABLE** `[23:12]`
    ///
    /// On power-up this field is initialised to DISABLE and the chip runs from the ROSC.
    /// If the chip has subsequently been programmed to run from the XOSC then setting this field to DISABLE may lock-up the chip. If this is a concern then run the clk_ref from the ROSC and enable the clk_sys RESUS feature.
    /// The 12-bit code is intended to give some protection against accidental writes. An invalid setting will retain the previous value. The actual value being used can be read from STATUS_ENABLED
    #[inline(always)]
    pub const fn enable(&self) -> u32 {
        (self.bits & Ctrl::ENABLE_MASK) >> Ctrl::ENABLE_OFFSET
    }

    /// Read **FREQ_RANGE** `[11:0]`
    ///
    /// The 12-bit code is intended to give some protection against accidental writes. An invalid setting will retain the previous value. The actual value being used can be read from STATUS_FREQ_RANGE
    #[inline(always)]
    pub const fn freq_range(&self) -> u32 {
        (self.bits & Ctrl::FREQ_RANGE_MASK) >> Ctrl::FREQ_RANGE_OFFSET
    }

    /// Set **ENABLE** `[23:12]`
    ///
    /// On power-up this field is initialised to DISABLE and the chip runs from the ROSC.
    /// If the chip has subsequently been programmed to run from the XOSC then setting this field to DISABLE may lock-up the chip. If this is a concern then run the clk_ref from the ROSC and enable the clk_sys RESUS feature.
    /// The 12-bit code is intended to give some protection against accidental writes. An invalid setting will retain the previous value. The actual value being used can be read from STATUS_ENABLED
    #[inline(always)]
    pub const fn with_enable(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ctrl::ENABLE_MASK)
                | ((value << Ctrl::ENABLE_OFFSET) & Ctrl::ENABLE_MASK),
        }
    }

    /// Set **FREQ_RANGE** `[11:0]`
    ///
    /// The 12-bit code is intended to give some protection against accidental writes. An invalid setting will retain the previous value. The actual value being used can be read from STATUS_FREQ_RANGE
    #[inline(always)]
    pub const fn with_freq_range(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ctrl::FREQ_RANGE_MASK)
                | ((value << Ctrl::FREQ_RANGE_OFFSET) & Ctrl::FREQ_RANGE_MASK),
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

/// STATUS Register
///
/// Crystal Oscillator Status
///
/// Address offset: 0x0004
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **STABLE** `[31]` (RO)
///
///   Oscillator is running and stable
///
/// - **BADWRITE** `[24]` (RW)
///
///   An invalid value has been written to CTRL_ENABLE or CTRL_FREQ_RANGE or DORMANT
///
/// - **ENABLED** `[12]` (RO)
///
///   Oscillator is enabled but not necessarily running and stable, resets to 0
///
/// - **FREQ_RANGE** `[1:0]` (RO)
///
///   The current frequency range setting
#[repr(transparent)]
pub struct Status {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Status {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Status {
    // ======== Field Constants ========

    /// **STABLE**
    ///
    /// Oscillator is running and stable
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const STABLE_OFFSET: u32 = 31;
    pub const STABLE_MASK: u32 = 0x80000000;
    pub const STABLE: u32 = 1 << 31;

    /// **BADWRITE**
    ///
    /// An invalid value has been written to CTRL_ENABLE or CTRL_FREQ_RANGE or DORMANT
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const BADWRITE_OFFSET: u32 = 24;
    pub const BADWRITE_MASK: u32 = 0x01000000;
    pub const BADWRITE: u32 = 1 << 24;

    /// **ENABLED**
    ///
    /// Oscillator is enabled but not necessarily running and stable, resets to 0
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const ENABLED_OFFSET: u32 = 12;
    pub const ENABLED_MASK: u32 = 0x00001000;
    pub const ENABLED: u32 = 1 << 12;

    /// **FREQ_RANGE**
    ///
    /// The current frequency range setting
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const FREQ_RANGE_OFFSET: u32 = 0;
    pub const FREQ_RANGE_MASK: u32 = 0x00000003;
    pub const FREQ_RANGE_1_15MHZ: u32 = 0 << 0;
    pub const FREQ_RANGE_1_15MHZ_VAL: u32 = 0;
    pub const FREQ_RANGE_10_30MHZ: u32 = 1 << 0;
    pub const FREQ_RANGE_10_30MHZ_VAL: u32 = 1;
    pub const FREQ_RANGE_25_60MHZ: u32 = 2 << 0;
    pub const FREQ_RANGE_25_60MHZ_VAL: u32 = 2;
    pub const FREQ_RANGE_40_100MHZ: u32 = 3 << 0;
    pub const FREQ_RANGE_40_100MHZ_VAL: u32 = 3;

    // ======== Field Accessors ========

    /// **STABLE** `[31]` (RO)
    ///
    /// Oscillator is running and stable
    #[inline(always)]
    pub fn stable(&self) -> bool {
        (self.value.read_bits() & Self::STABLE_MASK) != 0
    }

    /// **BADWRITE** `[24]` (RW)
    ///
    /// An invalid value has been written to CTRL_ENABLE or CTRL_FREQ_RANGE or DORMANT
    #[inline(always)]
    pub fn badwrite(&self) -> bool {
        (self.value.read_bits() & Self::BADWRITE_MASK) != 0
    }

    /// Set BADWRITE bit atomically (RW)
    #[inline(always)]
    pub fn set_badwrite(&self) {
        self.set_bits(Self::BADWRITE_MASK);
    }

    /// Clear BADWRITE bit atomically (RW)
    #[inline(always)]
    pub fn clr_badwrite(&self) {
        self.clr_bits(Self::BADWRITE_MASK);
    }

    /// **ENABLED** `[12]` (RO)
    ///
    /// Oscillator is enabled but not necessarily running and stable, resets to 0
    #[inline(always)]
    pub fn enabled(&self) -> bool {
        (self.value.read_bits() & Self::ENABLED_MASK) != 0
    }

    /// **FREQ_RANGE** `[1:0]` (RO)
    ///
    /// The current frequency range setting
    #[inline(always)]
    pub fn freq_range(&self) -> u32 {
        (self.value.read_bits() & Self::FREQ_RANGE_MASK) >> Self::FREQ_RANGE_OFFSET
    }
}

/// STATUS value type for batch read/modify/write operations
///
/// Use `status.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `status.write_val(val)`.
#[derive(Clone, Copy)]
pub struct StatusVal {
    bits: u32,
}

impl StatusVal {
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

    /// Read **STABLE** `[31:31]`
    ///
    /// Oscillator is running and stable
    #[inline(always)]
    pub const fn stable(&self) -> bool {
        (self.bits & Status::STABLE_MASK) != 0
    }

    /// Read **BADWRITE** `[24:24]`
    ///
    /// An invalid value has been written to CTRL_ENABLE or CTRL_FREQ_RANGE or DORMANT
    #[inline(always)]
    pub const fn badwrite(&self) -> bool {
        (self.bits & Status::BADWRITE_MASK) != 0
    }

    /// Read **ENABLED** `[12:12]`
    ///
    /// Oscillator is enabled but not necessarily running and stable, resets to 0
    #[inline(always)]
    pub const fn enabled(&self) -> bool {
        (self.bits & Status::ENABLED_MASK) != 0
    }

    /// Read **FREQ_RANGE** `[1:0]`
    ///
    /// The current frequency range setting
    #[inline(always)]
    pub const fn freq_range(&self) -> u32 {
        (self.bits & Status::FREQ_RANGE_MASK) >> Status::FREQ_RANGE_OFFSET
    }

    /// Set **BADWRITE** `[24:24]`
    ///
    /// An invalid value has been written to CTRL_ENABLE or CTRL_FREQ_RANGE or DORMANT
    #[inline(always)]
    pub const fn with_badwrite(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Status::BADWRITE_MASK
            } else {
                self.bits & !Status::BADWRITE_MASK
            },
        }
    }
}

impl Default for StatusVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Status {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> StatusVal {
        StatusVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: StatusVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(StatusVal) -> StatusVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DORMANT Register
///
/// Crystal Oscillator pause control
///
/// Address offset: 0x0008
///
/// ## Fields
///
/// - **DORMANT** `[31:0]` (RW)
///
///   This is used to save power by pausing the XOSC
///   On power-up this field is initialised to WAKE
///   An invalid write will also select WAKE
///   Warning: stop the PLLs before selecting dormant mode
///   Warning: setup the irq before selecting dormant mode
#[repr(transparent)]
pub struct Dormant {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Dormant {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Dormant {
    // ======== Field Constants ========

    /// **DORMANT**
    ///
    /// This is used to save power by pausing the XOSC
    /// On power-up this field is initialised to WAKE
    /// An invalid write will also select WAKE
    /// Warning: stop the PLLs before selecting dormant mode
    /// Warning: setup the irq before selecting dormant mode
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const DORMANT_OFFSET: u32 = 0;
    pub const DORMANT_MASK: u32 = 0xffffffff;
    pub const DORMANT_DORMANT: u32 = 1668246881 << 0;
    pub const DORMANT_DORMANT_VAL: u32 = 1668246881;
    pub const DORMANT_WAKE: u32 = 2002873189 << 0;
    pub const DORMANT_WAKE_VAL: u32 = 2002873189;

    // ======== Field Accessors ========

    /// **DORMANT** `[31:0]` (RW)
    ///
    /// This is used to save power by pausing the XOSC
    /// On power-up this field is initialised to WAKE
    /// An invalid write will also select WAKE
    /// Warning: stop the PLLs before selecting dormant mode
    /// Warning: setup the irq before selecting dormant mode
    #[inline(always)]
    pub fn dormant(&self) -> u32 {
        (self.value.read_bits() & Self::DORMANT_MASK) >> Self::DORMANT_OFFSET
    }

    /// Write **DORMANT** `[31:0]` (RW)
    ///
    /// This is used to save power by pausing the XOSC
    /// On power-up this field is initialised to WAKE
    /// An invalid write will also select WAKE
    /// Warning: stop the PLLs before selecting dormant mode
    /// Warning: setup the irq before selecting dormant mode
    #[inline(always)]
    pub fn write_dormant(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DORMANT_MASK) | ((value << Self::DORMANT_OFFSET) & Self::DORMANT_MASK)
        });
    }
}

/// DORMANT value type for batch read/modify/write operations
///
/// Use `dormant.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dormant.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DormantVal {
    bits: u32,
}

impl DormantVal {
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

    /// Read **DORMANT** `[31:0]`
    ///
    /// This is used to save power by pausing the XOSC
    /// On power-up this field is initialised to WAKE
    /// An invalid write will also select WAKE
    /// Warning: stop the PLLs before selecting dormant mode
    /// Warning: setup the irq before selecting dormant mode
    #[inline(always)]
    pub const fn dormant(&self) -> u32 {
        (self.bits & Dormant::DORMANT_MASK) >> Dormant::DORMANT_OFFSET
    }

    /// Set **DORMANT** `[31:0]`
    ///
    /// This is used to save power by pausing the XOSC
    /// On power-up this field is initialised to WAKE
    /// An invalid write will also select WAKE
    /// Warning: stop the PLLs before selecting dormant mode
    /// Warning: setup the irq before selecting dormant mode
    #[inline(always)]
    pub const fn with_dormant(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Dormant::DORMANT_MASK)
                | ((value << Dormant::DORMANT_OFFSET) & Dormant::DORMANT_MASK),
        }
    }
}

impl Default for DormantVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Dormant {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DormantVal {
        DormantVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DormantVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DormantVal) -> DormantVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// STARTUP Register
///
/// Controls the startup delay
///
/// Address offset: 0x000c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **X4** `[20]` (RW)
///
///   Multiplies the startup_delay by 4, just in case. The reset value is controlled by a mask-programmable tiecell and is provided in case we are booting from XOSC and the default startup delay is insufficient. The reset value is 0x0.
///
/// - **DELAY** `[13:0]` (RW)
///
///   in multiples of 256*xtal_period. The reset value of 0xc4 corresponds to approx 50 000 cycles.
#[repr(transparent)]
pub struct Startup {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Startup {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Startup {
    // ======== Field Constants ========

    /// **X4**
    ///
    /// Multiplies the startup_delay by 4, just in case. The reset value is controlled by a mask-programmable tiecell and is provided in case we are booting from XOSC and the default startup delay is insufficient. The reset value is 0x0.
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const X4_OFFSET: u32 = 20;
    pub const X4_MASK: u32 = 0x00100000;
    pub const X4: u32 = 1 << 20;

    /// **DELAY**
    ///
    /// in multiples of 256*xtal_period. The reset value of 0xc4 corresponds to approx 50 000 cycles.
    ///
    /// Bits `[13:0]`, mask `0x00003fff`
    pub const DELAY_OFFSET: u32 = 0;
    pub const DELAY_MASK: u32 = 0x00003fff;

    // ======== Field Accessors ========

    /// **X4** `[20]` (RW)
    ///
    /// Multiplies the startup_delay by 4, just in case. The reset value is controlled by a mask-programmable tiecell and is provided in case we are booting from XOSC and the default startup delay is insufficient. The reset value is 0x0.
    #[inline(always)]
    pub fn x4(&self) -> bool {
        (self.value.read_bits() & Self::X4_MASK) != 0
    }

    /// Set X4 bit atomically (RW)
    #[inline(always)]
    pub fn set_x4(&self) {
        self.set_bits(Self::X4_MASK);
    }

    /// Clear X4 bit atomically (RW)
    #[inline(always)]
    pub fn clr_x4(&self) {
        self.clr_bits(Self::X4_MASK);
    }

    /// **DELAY** `[13:0]` (RW)
    ///
    /// in multiples of 256*xtal_period. The reset value of 0xc4 corresponds to approx 50 000 cycles.
    #[inline(always)]
    pub fn delay(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_MASK) >> Self::DELAY_OFFSET
    }

    /// Write **DELAY** `[13:0]` (RW)
    ///
    /// in multiples of 256*xtal_period. The reset value of 0xc4 corresponds to approx 50 000 cycles.
    #[inline(always)]
    pub fn write_delay(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DELAY_MASK) | ((value << Self::DELAY_OFFSET) & Self::DELAY_MASK)
        });
    }
}

/// STARTUP value type for batch read/modify/write operations
///
/// Use `startup.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `startup.write_val(val)`.
#[derive(Clone, Copy)]
pub struct StartupVal {
    bits: u32,
}

impl StartupVal {
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

    /// Read **X4** `[20:20]`
    ///
    /// Multiplies the startup_delay by 4, just in case. The reset value is controlled by a mask-programmable tiecell and is provided in case we are booting from XOSC and the default startup delay is insufficient. The reset value is 0x0.
    #[inline(always)]
    pub const fn x4(&self) -> bool {
        (self.bits & Startup::X4_MASK) != 0
    }

    /// Read **DELAY** `[13:0]`
    ///
    /// in multiples of 256*xtal_period. The reset value of 0xc4 corresponds to approx 50 000 cycles.
    #[inline(always)]
    pub const fn delay(&self) -> u32 {
        (self.bits & Startup::DELAY_MASK) >> Startup::DELAY_OFFSET
    }

    /// Set **X4** `[20:20]`
    ///
    /// Multiplies the startup_delay by 4, just in case. The reset value is controlled by a mask-programmable tiecell and is provided in case we are booting from XOSC and the default startup delay is insufficient. The reset value is 0x0.
    #[inline(always)]
    pub const fn with_x4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Startup::X4_MASK
            } else {
                self.bits & !Startup::X4_MASK
            },
        }
    }

    /// Set **DELAY** `[13:0]`
    ///
    /// in multiples of 256*xtal_period. The reset value of 0xc4 corresponds to approx 50 000 cycles.
    #[inline(always)]
    pub const fn with_delay(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Startup::DELAY_MASK)
                | ((value << Startup::DELAY_OFFSET) & Startup::DELAY_MASK),
        }
    }
}

impl Default for StartupVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Startup {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> StartupVal {
        StartupVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: StartupVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(StartupVal) -> StartupVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// COUNT Register
///
/// A down counter running at the xosc frequency which counts to zero and stops.
/// Can be used for short software pauses when setting up time sensitive hardware.
/// To start the counter, write a non-zero value. Reads will return 1 while the count is running and 0 when it has finished.
/// Minimum count value is 4. Count values <4 will be treated as count value =4.
/// Note that synchronisation to the register clock domain costs 2 register clock cycles and the counter cannot compensate for that.
///
/// Address offset: 0x0010
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **COUNT** `[15:0]` (RW)
#[repr(transparent)]
pub struct Count {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Count {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Count {
    // ======== Field Constants ========

    /// **COUNT**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const COUNT_OFFSET: u32 = 0;
    pub const COUNT_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **COUNT** `[15:0]` (RW)
    #[inline(always)]
    pub fn count(&self) -> u32 {
        (self.value.read_bits() & Self::COUNT_MASK) >> Self::COUNT_OFFSET
    }

    /// Write **COUNT** `[15:0]` (RW)
    #[inline(always)]
    pub fn write_count(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::COUNT_MASK) | ((value << Self::COUNT_OFFSET) & Self::COUNT_MASK)
        });
    }
}

/// COUNT value type for batch read/modify/write operations
///
/// Use `count.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `count.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CountVal {
    bits: u32,
}

impl CountVal {
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

    /// Read **COUNT** `[15:0]`
    #[inline(always)]
    pub const fn count(&self) -> u32 {
        (self.bits & Count::COUNT_MASK) >> Count::COUNT_OFFSET
    }

    /// Set **COUNT** `[15:0]`
    #[inline(always)]
    pub const fn with_count(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Count::COUNT_MASK)
                | ((value << Count::COUNT_OFFSET) & Count::COUNT_MASK),
        }
    }
}

impl Default for CountVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Count {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CountVal {
        CountVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CountVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CountVal) -> CountVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// Crystal Oscillator Control
    pub ctrl: Ctrl,
    /// 0x0004
    /// Crystal Oscillator Status
    pub status: Status,
    /// 0x0008
    /// Crystal Oscillator pause control
    pub dormant: Dormant,
    /// 0x000c
    /// Controls the startup delay
    pub startup: Startup,
    /// 0x0010
    /// A down counter running at the xosc frequency which counts to zero and stops.
    /// Can be used for short software pauses when setting up time sensitive hardware.
    /// To start the counter, write a non-zero value. Reads will return 1 while the count is running and 0 when it has finished.
    /// Minimum count value is 4. Count values <4 will be treated as count value =4.
    /// Note that synchronisation to the register clock domain costs 2 register clock cycles and the counter cannot compensate for that.
    pub count: Count,
}
