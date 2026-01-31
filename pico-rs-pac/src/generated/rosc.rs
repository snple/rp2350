//! ROSC

#![allow(dead_code)]

pub const BASE: u32 = 0x400e8000;

/// CTRL Register
///
/// Ring Oscillator control
///
/// Address offset: 0x0000
/// Reset value: 0x00000aa0
///
/// ## Fields
///
/// - **ENABLE** `[23:12]` (RW)
///
///   On power-up this field is initialised to ENABLE
///   The system clock must be switched to another source before setting this field to DISABLE otherwise the chip will lock up
///   The 12-bit code is intended to give some protection against accidental writes. An invalid setting will enable the oscillator.
///
/// - **FREQ_RANGE** `[11:0]` (RW)
///
///   Controls the number of delay stages in the ROSC ring
///   LOW uses stages 0 to 7
///   MEDIUM uses stages 0 to 5
///   HIGH uses stages 0 to 3
///   TOOHIGH uses stages 0 to 1 and should not be used because its frequency exceeds design specifications
///   The clock output will not glitch when changing the range up one step at a time
///   The clock output will glitch when changing the range down
///   Note: the values here are gray coded which is why HIGH comes before TOOHIGH
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

    /// **ENABLE**
    ///
    /// On power-up this field is initialised to ENABLE
    /// The system clock must be switched to another source before setting this field to DISABLE otherwise the chip will lock up
    /// The 12-bit code is intended to give some protection against accidental writes. An invalid setting will enable the oscillator.
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
    /// Controls the number of delay stages in the ROSC ring
    /// LOW uses stages 0 to 7
    /// MEDIUM uses stages 0 to 5
    /// HIGH uses stages 0 to 3
    /// TOOHIGH uses stages 0 to 1 and should not be used because its frequency exceeds design specifications
    /// The clock output will not glitch when changing the range up one step at a time
    /// The clock output will glitch when changing the range down
    /// Note: the values here are gray coded which is why HIGH comes before TOOHIGH
    ///
    /// Bits `[11:0]`, mask `0x00000fff`
    pub const FREQ_RANGE_OFFSET: u32 = 0;
    pub const FREQ_RANGE_MASK: u32 = 0x00000fff;
    pub const FREQ_RANGE_LOW: u32 = 4004 << 0;
    pub const FREQ_RANGE_LOW_VAL: u32 = 4004;
    pub const FREQ_RANGE_MEDIUM: u32 = 4005 << 0;
    pub const FREQ_RANGE_MEDIUM_VAL: u32 = 4005;
    pub const FREQ_RANGE_HIGH: u32 = 4007 << 0;
    pub const FREQ_RANGE_HIGH_VAL: u32 = 4007;
    pub const FREQ_RANGE_TOOHIGH: u32 = 4006 << 0;
    pub const FREQ_RANGE_TOOHIGH_VAL: u32 = 4006;

    // ======== Field Accessors ========

    /// **ENABLE** `[23:12]` (RW)
    ///
    /// On power-up this field is initialised to ENABLE
    /// The system clock must be switched to another source before setting this field to DISABLE otherwise the chip will lock up
    /// The 12-bit code is intended to give some protection against accidental writes. An invalid setting will enable the oscillator.
    #[inline(always)]
    pub fn enable(&self) -> u32 {
        (self.value.read_bits() & Self::ENABLE_MASK) >> Self::ENABLE_OFFSET
    }

    /// Write **ENABLE** `[23:12]` (RW)
    ///
    /// On power-up this field is initialised to ENABLE
    /// The system clock must be switched to another source before setting this field to DISABLE otherwise the chip will lock up
    /// The 12-bit code is intended to give some protection against accidental writes. An invalid setting will enable the oscillator.
    #[inline(always)]
    pub fn write_enable(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::ENABLE_MASK) | ((value << Self::ENABLE_OFFSET) & Self::ENABLE_MASK));
    }

    /// **FREQ_RANGE** `[11:0]` (RW)
    ///
    /// Controls the number of delay stages in the ROSC ring
    /// LOW uses stages 0 to 7
    /// MEDIUM uses stages 0 to 5
    /// HIGH uses stages 0 to 3
    /// TOOHIGH uses stages 0 to 1 and should not be used because its frequency exceeds design specifications
    /// The clock output will not glitch when changing the range up one step at a time
    /// The clock output will glitch when changing the range down
    /// Note: the values here are gray coded which is why HIGH comes before TOOHIGH
    #[inline(always)]
    pub fn freq_range(&self) -> u32 {
        (self.value.read_bits() & Self::FREQ_RANGE_MASK) >> Self::FREQ_RANGE_OFFSET
    }

    /// Write **FREQ_RANGE** `[11:0]` (RW)
    ///
    /// Controls the number of delay stages in the ROSC ring
    /// LOW uses stages 0 to 7
    /// MEDIUM uses stages 0 to 5
    /// HIGH uses stages 0 to 3
    /// TOOHIGH uses stages 0 to 1 and should not be used because its frequency exceeds design specifications
    /// The clock output will not glitch when changing the range up one step at a time
    /// The clock output will glitch when changing the range down
    /// Note: the values here are gray coded which is why HIGH comes before TOOHIGH
    #[inline(always)]
    pub fn write_freq_range(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::FREQ_RANGE_MASK) | ((value << Self::FREQ_RANGE_OFFSET) & Self::FREQ_RANGE_MASK));
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
        Self { bits: 0x00000aa0 }
    }

    /// Read **ENABLE** `[23:12]`
    ///
    /// On power-up this field is initialised to ENABLE
    /// The system clock must be switched to another source before setting this field to DISABLE otherwise the chip will lock up
    /// The 12-bit code is intended to give some protection against accidental writes. An invalid setting will enable the oscillator.
    #[inline(always)]
    pub const fn enable(&self) -> u32 {
        (self.bits & Ctrl::ENABLE_MASK) >> Ctrl::ENABLE_OFFSET
    }

    /// Read **FREQ_RANGE** `[11:0]`
    ///
    /// Controls the number of delay stages in the ROSC ring
    /// LOW uses stages 0 to 7
    /// MEDIUM uses stages 0 to 5
    /// HIGH uses stages 0 to 3
    /// TOOHIGH uses stages 0 to 1 and should not be used because its frequency exceeds design specifications
    /// The clock output will not glitch when changing the range up one step at a time
    /// The clock output will glitch when changing the range down
    /// Note: the values here are gray coded which is why HIGH comes before TOOHIGH
    #[inline(always)]
    pub const fn freq_range(&self) -> u32 {
        (self.bits & Ctrl::FREQ_RANGE_MASK) >> Ctrl::FREQ_RANGE_OFFSET
    }

    /// Set **ENABLE** `[23:12]`
    ///
    /// On power-up this field is initialised to ENABLE
    /// The system clock must be switched to another source before setting this field to DISABLE otherwise the chip will lock up
    /// The 12-bit code is intended to give some protection against accidental writes. An invalid setting will enable the oscillator.
    #[inline(always)]
    pub const fn with_enable(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ctrl::ENABLE_MASK) | ((value << Ctrl::ENABLE_OFFSET) & Ctrl::ENABLE_MASK)
        }
    }

    /// Set **FREQ_RANGE** `[11:0]`
    ///
    /// Controls the number of delay stages in the ROSC ring
    /// LOW uses stages 0 to 7
    /// MEDIUM uses stages 0 to 5
    /// HIGH uses stages 0 to 3
    /// TOOHIGH uses stages 0 to 1 and should not be used because its frequency exceeds design specifications
    /// The clock output will not glitch when changing the range up one step at a time
    /// The clock output will glitch when changing the range down
    /// Note: the values here are gray coded which is why HIGH comes before TOOHIGH
    #[inline(always)]
    pub const fn with_freq_range(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ctrl::FREQ_RANGE_MASK) | ((value << Ctrl::FREQ_RANGE_OFFSET) & Ctrl::FREQ_RANGE_MASK)
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

/// FREQA Register
///
/// The FREQA & FREQB registers control the frequency by controlling the drive strength of each stage
/// The drive strength has 4 levels determined by the number of bits set
/// Increasing the number of bits set increases the drive strength and increases the oscillation frequency
/// 0 bits set is the default drive strength
/// 1 bit set doubles the drive strength
/// 2 bits set triples drive strength
/// 3 bits set quadruples drive strength
/// For frequency randomisation set both DS0_RANDOM=1 & DS1_RANDOM=1
///
/// Address offset: 0x0004
/// Reset value: 0x00000088
///
/// ## Fields
///
/// - **PASSWD** `[31:16]` (RW)
///
///   Set to 0x9696 to apply the settings
///   Any other value in this field will set all drive strengths to 0
///
/// - **DS3** `[14:12]` (RW)
///
///   Stage 3 drive strength
///
/// - **DS2** `[10:8]` (RW)
///
///   Stage 2 drive strength
///
/// - **DS1_RANDOM** `[7]` (RW)
///
///   Randomises the stage 1 drive strength
///
/// - **DS1** `[6:4]` (RW)
///
///   Stage 1 drive strength
///
/// - **DS0_RANDOM** `[3]` (RW)
///
///   Randomises the stage 0 drive strength
///
/// - **DS0** `[2:0]` (RW)
///
///   Stage 0 drive strength
#[repr(transparent)]
pub struct Freqa {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Freqa {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Freqa {
    // ======== Field Constants ========

    /// **PASSWD**
    ///
    /// Set to 0x9696 to apply the settings
    /// Any other value in this field will set all drive strengths to 0
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const PASSWD_OFFSET: u32 = 16;
    pub const PASSWD_MASK: u32 = 0xffff0000;
    pub const PASSWD_PASS: u32 = 38550 << 16;
    pub const PASSWD_PASS_VAL: u32 = 38550;

    /// **DS3**
    ///
    /// Stage 3 drive strength
    ///
    /// Bits `[14:12]`, mask `0x00007000`
    pub const DS3_OFFSET: u32 = 12;
    pub const DS3_MASK: u32 = 0x00007000;

    /// **DS2**
    ///
    /// Stage 2 drive strength
    ///
    /// Bits `[10:8]`, mask `0x00000700`
    pub const DS2_OFFSET: u32 = 8;
    pub const DS2_MASK: u32 = 0x00000700;

    /// **DS1_RANDOM**
    ///
    /// Randomises the stage 1 drive strength
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const DS1_RANDOM_OFFSET: u32 = 7;
    pub const DS1_RANDOM_MASK: u32 = 0x00000080;
    pub const DS1_RANDOM: u32 = 1 << 7;

    /// **DS1**
    ///
    /// Stage 1 drive strength
    ///
    /// Bits `[6:4]`, mask `0x00000070`
    pub const DS1_OFFSET: u32 = 4;
    pub const DS1_MASK: u32 = 0x00000070;

    /// **DS0_RANDOM**
    ///
    /// Randomises the stage 0 drive strength
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const DS0_RANDOM_OFFSET: u32 = 3;
    pub const DS0_RANDOM_MASK: u32 = 0x00000008;
    pub const DS0_RANDOM: u32 = 1 << 3;

    /// **DS0**
    ///
    /// Stage 0 drive strength
    ///
    /// Bits `[2:0]`, mask `0x00000007`
    pub const DS0_OFFSET: u32 = 0;
    pub const DS0_MASK: u32 = 0x00000007;

    // ======== Field Accessors ========

    /// **PASSWD** `[31:16]` (RW)
    ///
    /// Set to 0x9696 to apply the settings
    /// Any other value in this field will set all drive strengths to 0
    #[inline(always)]
    pub fn passwd(&self) -> u32 {
        (self.value.read_bits() & Self::PASSWD_MASK) >> Self::PASSWD_OFFSET
    }

    /// Write **PASSWD** `[31:16]` (RW)
    ///
    /// Set to 0x9696 to apply the settings
    /// Any other value in this field will set all drive strengths to 0
    #[inline(always)]
    pub fn write_passwd(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::PASSWD_MASK) | ((value << Self::PASSWD_OFFSET) & Self::PASSWD_MASK));
    }

    /// **DS3** `[14:12]` (RW)
    ///
    /// Stage 3 drive strength
    #[inline(always)]
    pub fn ds3(&self) -> u32 {
        (self.value.read_bits() & Self::DS3_MASK) >> Self::DS3_OFFSET
    }

    /// Write **DS3** `[14:12]` (RW)
    ///
    /// Stage 3 drive strength
    #[inline(always)]
    pub fn write_ds3(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DS3_MASK) | ((value << Self::DS3_OFFSET) & Self::DS3_MASK));
    }

    /// **DS2** `[10:8]` (RW)
    ///
    /// Stage 2 drive strength
    #[inline(always)]
    pub fn ds2(&self) -> u32 {
        (self.value.read_bits() & Self::DS2_MASK) >> Self::DS2_OFFSET
    }

    /// Write **DS2** `[10:8]` (RW)
    ///
    /// Stage 2 drive strength
    #[inline(always)]
    pub fn write_ds2(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DS2_MASK) | ((value << Self::DS2_OFFSET) & Self::DS2_MASK));
    }

    /// **DS1_RANDOM** `[7]` (RW)
    ///
    /// Randomises the stage 1 drive strength
    #[inline(always)]
    pub fn ds1_random(&self) -> bool {
        (self.value.read_bits() & Self::DS1_RANDOM_MASK) != 0
    }

    /// Set DS1_RANDOM bit atomically (RW)
    #[inline(always)]
    pub fn set_ds1_random(&self) {
        self.value.set_bits(Self::DS1_RANDOM_MASK);
    }

    /// Clear DS1_RANDOM bit atomically (RW)
    #[inline(always)]
    pub fn clr_ds1_random(&self) {
        self.value.clr_bits(Self::DS1_RANDOM_MASK);
    }

    /// **DS1** `[6:4]` (RW)
    ///
    /// Stage 1 drive strength
    #[inline(always)]
    pub fn ds1(&self) -> u32 {
        (self.value.read_bits() & Self::DS1_MASK) >> Self::DS1_OFFSET
    }

    /// Write **DS1** `[6:4]` (RW)
    ///
    /// Stage 1 drive strength
    #[inline(always)]
    pub fn write_ds1(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DS1_MASK) | ((value << Self::DS1_OFFSET) & Self::DS1_MASK));
    }

    /// **DS0_RANDOM** `[3]` (RW)
    ///
    /// Randomises the stage 0 drive strength
    #[inline(always)]
    pub fn ds0_random(&self) -> bool {
        (self.value.read_bits() & Self::DS0_RANDOM_MASK) != 0
    }

    /// Set DS0_RANDOM bit atomically (RW)
    #[inline(always)]
    pub fn set_ds0_random(&self) {
        self.value.set_bits(Self::DS0_RANDOM_MASK);
    }

    /// Clear DS0_RANDOM bit atomically (RW)
    #[inline(always)]
    pub fn clr_ds0_random(&self) {
        self.value.clr_bits(Self::DS0_RANDOM_MASK);
    }

    /// **DS0** `[2:0]` (RW)
    ///
    /// Stage 0 drive strength
    #[inline(always)]
    pub fn ds0(&self) -> u32 {
        (self.value.read_bits() & Self::DS0_MASK) >> Self::DS0_OFFSET
    }

    /// Write **DS0** `[2:0]` (RW)
    ///
    /// Stage 0 drive strength
    #[inline(always)]
    pub fn write_ds0(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DS0_MASK) | ((value << Self::DS0_OFFSET) & Self::DS0_MASK));
    }

}

/// FREQA value type for batch read/modify/write operations
///
/// Use `freqa.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `freqa.write_val(val)`.
#[derive(Clone, Copy)]
pub struct FreqaVal {
    bits: u32,
}

impl FreqaVal {
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
        Self { bits: 0x00000088 }
    }

    /// Read **PASSWD** `[31:16]`
    ///
    /// Set to 0x9696 to apply the settings
    /// Any other value in this field will set all drive strengths to 0
    #[inline(always)]
    pub const fn passwd(&self) -> u32 {
        (self.bits & Freqa::PASSWD_MASK) >> Freqa::PASSWD_OFFSET
    }

    /// Read **DS3** `[14:12]`
    ///
    /// Stage 3 drive strength
    #[inline(always)]
    pub const fn ds3(&self) -> u32 {
        (self.bits & Freqa::DS3_MASK) >> Freqa::DS3_OFFSET
    }

    /// Read **DS2** `[10:8]`
    ///
    /// Stage 2 drive strength
    #[inline(always)]
    pub const fn ds2(&self) -> u32 {
        (self.bits & Freqa::DS2_MASK) >> Freqa::DS2_OFFSET
    }

    /// Read **DS1_RANDOM** `[7:7]`
    ///
    /// Randomises the stage 1 drive strength
    #[inline(always)]
    pub const fn ds1_random(&self) -> bool {
        (self.bits & Freqa::DS1_RANDOM_MASK) != 0
    }

    /// Read **DS1** `[6:4]`
    ///
    /// Stage 1 drive strength
    #[inline(always)]
    pub const fn ds1(&self) -> u32 {
        (self.bits & Freqa::DS1_MASK) >> Freqa::DS1_OFFSET
    }

    /// Read **DS0_RANDOM** `[3:3]`
    ///
    /// Randomises the stage 0 drive strength
    #[inline(always)]
    pub const fn ds0_random(&self) -> bool {
        (self.bits & Freqa::DS0_RANDOM_MASK) != 0
    }

    /// Read **DS0** `[2:0]`
    ///
    /// Stage 0 drive strength
    #[inline(always)]
    pub const fn ds0(&self) -> u32 {
        (self.bits & Freqa::DS0_MASK) >> Freqa::DS0_OFFSET
    }

    /// Set **PASSWD** `[31:16]`
    ///
    /// Set to 0x9696 to apply the settings
    /// Any other value in this field will set all drive strengths to 0
    #[inline(always)]
    pub const fn with_passwd(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Freqa::PASSWD_MASK) | ((value << Freqa::PASSWD_OFFSET) & Freqa::PASSWD_MASK)
        }
    }

    /// Set **DS3** `[14:12]`
    ///
    /// Stage 3 drive strength
    #[inline(always)]
    pub const fn with_ds3(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Freqa::DS3_MASK) | ((value << Freqa::DS3_OFFSET) & Freqa::DS3_MASK)
        }
    }

    /// Set **DS2** `[10:8]`
    ///
    /// Stage 2 drive strength
    #[inline(always)]
    pub const fn with_ds2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Freqa::DS2_MASK) | ((value << Freqa::DS2_OFFSET) & Freqa::DS2_MASK)
        }
    }

    /// Set **DS1_RANDOM** `[7:7]`
    ///
    /// Randomises the stage 1 drive strength
    #[inline(always)]
    pub const fn with_ds1_random(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Freqa::DS1_RANDOM_MASK
            } else {
                self.bits & !Freqa::DS1_RANDOM_MASK
            }
        }
    }

    /// Set **DS1** `[6:4]`
    ///
    /// Stage 1 drive strength
    #[inline(always)]
    pub const fn with_ds1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Freqa::DS1_MASK) | ((value << Freqa::DS1_OFFSET) & Freqa::DS1_MASK)
        }
    }

    /// Set **DS0_RANDOM** `[3:3]`
    ///
    /// Randomises the stage 0 drive strength
    #[inline(always)]
    pub const fn with_ds0_random(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Freqa::DS0_RANDOM_MASK
            } else {
                self.bits & !Freqa::DS0_RANDOM_MASK
            }
        }
    }

    /// Set **DS0** `[2:0]`
    ///
    /// Stage 0 drive strength
    #[inline(always)]
    pub const fn with_ds0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Freqa::DS0_MASK) | ((value << Freqa::DS0_OFFSET) & Freqa::DS0_MASK)
        }
    }

}

impl Default for FreqaVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Freqa {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> FreqaVal {
        FreqaVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: FreqaVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(FreqaVal) -> FreqaVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// FREQB Register
///
/// For a detailed description see freqa register
///
/// Address offset: 0x0008
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **PASSWD** `[31:16]` (RW)
///
///   Set to 0x9696 to apply the settings
///   Any other value in this field will set all drive strengths to 0
///
/// - **DS7** `[14:12]` (RW)
///
///   Stage 7 drive strength
///
/// - **DS6** `[10:8]` (RW)
///
///   Stage 6 drive strength
///
/// - **DS5** `[6:4]` (RW)
///
///   Stage 5 drive strength
///
/// - **DS4** `[2:0]` (RW)
///
///   Stage 4 drive strength
#[repr(transparent)]
pub struct Freqb {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Freqb {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Freqb {
    // ======== Field Constants ========

    /// **PASSWD**
    ///
    /// Set to 0x9696 to apply the settings
    /// Any other value in this field will set all drive strengths to 0
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const PASSWD_OFFSET: u32 = 16;
    pub const PASSWD_MASK: u32 = 0xffff0000;
    pub const PASSWD_PASS: u32 = 38550 << 16;
    pub const PASSWD_PASS_VAL: u32 = 38550;

    /// **DS7**
    ///
    /// Stage 7 drive strength
    ///
    /// Bits `[14:12]`, mask `0x00007000`
    pub const DS7_OFFSET: u32 = 12;
    pub const DS7_MASK: u32 = 0x00007000;

    /// **DS6**
    ///
    /// Stage 6 drive strength
    ///
    /// Bits `[10:8]`, mask `0x00000700`
    pub const DS6_OFFSET: u32 = 8;
    pub const DS6_MASK: u32 = 0x00000700;

    /// **DS5**
    ///
    /// Stage 5 drive strength
    ///
    /// Bits `[6:4]`, mask `0x00000070`
    pub const DS5_OFFSET: u32 = 4;
    pub const DS5_MASK: u32 = 0x00000070;

    /// **DS4**
    ///
    /// Stage 4 drive strength
    ///
    /// Bits `[2:0]`, mask `0x00000007`
    pub const DS4_OFFSET: u32 = 0;
    pub const DS4_MASK: u32 = 0x00000007;

    // ======== Field Accessors ========

    /// **PASSWD** `[31:16]` (RW)
    ///
    /// Set to 0x9696 to apply the settings
    /// Any other value in this field will set all drive strengths to 0
    #[inline(always)]
    pub fn passwd(&self) -> u32 {
        (self.value.read_bits() & Self::PASSWD_MASK) >> Self::PASSWD_OFFSET
    }

    /// Write **PASSWD** `[31:16]` (RW)
    ///
    /// Set to 0x9696 to apply the settings
    /// Any other value in this field will set all drive strengths to 0
    #[inline(always)]
    pub fn write_passwd(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::PASSWD_MASK) | ((value << Self::PASSWD_OFFSET) & Self::PASSWD_MASK));
    }

    /// **DS7** `[14:12]` (RW)
    ///
    /// Stage 7 drive strength
    #[inline(always)]
    pub fn ds7(&self) -> u32 {
        (self.value.read_bits() & Self::DS7_MASK) >> Self::DS7_OFFSET
    }

    /// Write **DS7** `[14:12]` (RW)
    ///
    /// Stage 7 drive strength
    #[inline(always)]
    pub fn write_ds7(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DS7_MASK) | ((value << Self::DS7_OFFSET) & Self::DS7_MASK));
    }

    /// **DS6** `[10:8]` (RW)
    ///
    /// Stage 6 drive strength
    #[inline(always)]
    pub fn ds6(&self) -> u32 {
        (self.value.read_bits() & Self::DS6_MASK) >> Self::DS6_OFFSET
    }

    /// Write **DS6** `[10:8]` (RW)
    ///
    /// Stage 6 drive strength
    #[inline(always)]
    pub fn write_ds6(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DS6_MASK) | ((value << Self::DS6_OFFSET) & Self::DS6_MASK));
    }

    /// **DS5** `[6:4]` (RW)
    ///
    /// Stage 5 drive strength
    #[inline(always)]
    pub fn ds5(&self) -> u32 {
        (self.value.read_bits() & Self::DS5_MASK) >> Self::DS5_OFFSET
    }

    /// Write **DS5** `[6:4]` (RW)
    ///
    /// Stage 5 drive strength
    #[inline(always)]
    pub fn write_ds5(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DS5_MASK) | ((value << Self::DS5_OFFSET) & Self::DS5_MASK));
    }

    /// **DS4** `[2:0]` (RW)
    ///
    /// Stage 4 drive strength
    #[inline(always)]
    pub fn ds4(&self) -> u32 {
        (self.value.read_bits() & Self::DS4_MASK) >> Self::DS4_OFFSET
    }

    /// Write **DS4** `[2:0]` (RW)
    ///
    /// Stage 4 drive strength
    #[inline(always)]
    pub fn write_ds4(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DS4_MASK) | ((value << Self::DS4_OFFSET) & Self::DS4_MASK));
    }

}

/// FREQB value type for batch read/modify/write operations
///
/// Use `freqb.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `freqb.write_val(val)`.
#[derive(Clone, Copy)]
pub struct FreqbVal {
    bits: u32,
}

impl FreqbVal {
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

    /// Read **PASSWD** `[31:16]`
    ///
    /// Set to 0x9696 to apply the settings
    /// Any other value in this field will set all drive strengths to 0
    #[inline(always)]
    pub const fn passwd(&self) -> u32 {
        (self.bits & Freqb::PASSWD_MASK) >> Freqb::PASSWD_OFFSET
    }

    /// Read **DS7** `[14:12]`
    ///
    /// Stage 7 drive strength
    #[inline(always)]
    pub const fn ds7(&self) -> u32 {
        (self.bits & Freqb::DS7_MASK) >> Freqb::DS7_OFFSET
    }

    /// Read **DS6** `[10:8]`
    ///
    /// Stage 6 drive strength
    #[inline(always)]
    pub const fn ds6(&self) -> u32 {
        (self.bits & Freqb::DS6_MASK) >> Freqb::DS6_OFFSET
    }

    /// Read **DS5** `[6:4]`
    ///
    /// Stage 5 drive strength
    #[inline(always)]
    pub const fn ds5(&self) -> u32 {
        (self.bits & Freqb::DS5_MASK) >> Freqb::DS5_OFFSET
    }

    /// Read **DS4** `[2:0]`
    ///
    /// Stage 4 drive strength
    #[inline(always)]
    pub const fn ds4(&self) -> u32 {
        (self.bits & Freqb::DS4_MASK) >> Freqb::DS4_OFFSET
    }

    /// Set **PASSWD** `[31:16]`
    ///
    /// Set to 0x9696 to apply the settings
    /// Any other value in this field will set all drive strengths to 0
    #[inline(always)]
    pub const fn with_passwd(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Freqb::PASSWD_MASK) | ((value << Freqb::PASSWD_OFFSET) & Freqb::PASSWD_MASK)
        }
    }

    /// Set **DS7** `[14:12]`
    ///
    /// Stage 7 drive strength
    #[inline(always)]
    pub const fn with_ds7(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Freqb::DS7_MASK) | ((value << Freqb::DS7_OFFSET) & Freqb::DS7_MASK)
        }
    }

    /// Set **DS6** `[10:8]`
    ///
    /// Stage 6 drive strength
    #[inline(always)]
    pub const fn with_ds6(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Freqb::DS6_MASK) | ((value << Freqb::DS6_OFFSET) & Freqb::DS6_MASK)
        }
    }

    /// Set **DS5** `[6:4]`
    ///
    /// Stage 5 drive strength
    #[inline(always)]
    pub const fn with_ds5(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Freqb::DS5_MASK) | ((value << Freqb::DS5_OFFSET) & Freqb::DS5_MASK)
        }
    }

    /// Set **DS4** `[2:0]`
    ///
    /// Stage 4 drive strength
    #[inline(always)]
    pub const fn with_ds4(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Freqb::DS4_MASK) | ((value << Freqb::DS4_OFFSET) & Freqb::DS4_MASK)
        }
    }

}

impl Default for FreqbVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Freqb {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> FreqbVal {
        FreqbVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: FreqbVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(FreqbVal) -> FreqbVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// RANDOM Register
///
/// Loads a value to the LFSR randomiser
///
/// Address offset: 0x000c
/// Reset value: 0x3f04b16d
///
/// ## Fields
///
/// - **SEED** `[31:0]` (RW)
#[repr(transparent)]
pub struct Random {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Random {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Random {
    // ======== Field Constants ========

    /// **SEED**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SEED_OFFSET: u32 = 0;
    pub const SEED_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SEED** `[31:0]` (RW)
    #[inline(always)]
    pub fn seed(&self) -> u32 {
        (self.value.read_bits() & Self::SEED_MASK) >> Self::SEED_OFFSET
    }

    /// Write **SEED** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_seed(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SEED_MASK) | ((value << Self::SEED_OFFSET) & Self::SEED_MASK));
    }

}

/// RANDOM value type for batch read/modify/write operations
///
/// Use `random.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `random.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RandomVal {
    bits: u32,
}

impl RandomVal {
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
        Self { bits: 0x3f04b16d }
    }

    /// Read **SEED** `[31:0]`
    #[inline(always)]
    pub const fn seed(&self) -> u32 {
        (self.bits & Random::SEED_MASK) >> Random::SEED_OFFSET
    }

    /// Set **SEED** `[31:0]`
    #[inline(always)]
    pub const fn with_seed(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Random::SEED_MASK) | ((value << Random::SEED_OFFSET) & Random::SEED_MASK)
        }
    }

}

impl Default for RandomVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Random {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RandomVal {
        RandomVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RandomVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RandomVal) -> RandomVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// DORMANT Register
///
/// Ring Oscillator pause control
///
/// Address offset: 0x0010
///
/// ## Fields
///
/// - **DORMANT** `[31:0]` (RW)
///
///   This is used to save power by pausing the ROSC
///   On power-up this field is initialised to WAKE
///   An invalid write will also select WAKE
///   Warning: setup the irq before selecting dormant mode
#[repr(transparent)]
pub struct Dormant {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Dormant {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Dormant {
    // ======== Field Constants ========

    /// **DORMANT**
    ///
    /// This is used to save power by pausing the ROSC
    /// On power-up this field is initialised to WAKE
    /// An invalid write will also select WAKE
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
    /// This is used to save power by pausing the ROSC
    /// On power-up this field is initialised to WAKE
    /// An invalid write will also select WAKE
    /// Warning: setup the irq before selecting dormant mode
    #[inline(always)]
    pub fn dormant(&self) -> u32 {
        (self.value.read_bits() & Self::DORMANT_MASK) >> Self::DORMANT_OFFSET
    }

    /// Write **DORMANT** `[31:0]` (RW)
    ///
    /// This is used to save power by pausing the ROSC
    /// On power-up this field is initialised to WAKE
    /// An invalid write will also select WAKE
    /// Warning: setup the irq before selecting dormant mode
    #[inline(always)]
    pub fn write_dormant(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DORMANT_MASK) | ((value << Self::DORMANT_OFFSET) & Self::DORMANT_MASK));
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
    /// This is used to save power by pausing the ROSC
    /// On power-up this field is initialised to WAKE
    /// An invalid write will also select WAKE
    /// Warning: setup the irq before selecting dormant mode
    #[inline(always)]
    pub const fn dormant(&self) -> u32 {
        (self.bits & Dormant::DORMANT_MASK) >> Dormant::DORMANT_OFFSET
    }

    /// Set **DORMANT** `[31:0]`
    ///
    /// This is used to save power by pausing the ROSC
    /// On power-up this field is initialised to WAKE
    /// An invalid write will also select WAKE
    /// Warning: setup the irq before selecting dormant mode
    #[inline(always)]
    pub const fn with_dormant(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Dormant::DORMANT_MASK) | ((value << Dormant::DORMANT_OFFSET) & Dormant::DORMANT_MASK)
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

/// DIV Register
///
/// Controls the output divider
///
/// Address offset: 0x0014
///
/// ## Fields
///
/// - **DIV** `[15:0]` (RW)
///
///   set to 0xaa00 + div where
///   div = 0 divides by 128
///   div = 1-127 divides by div
///   any other value sets div=128
///   this register resets to div=32
#[repr(transparent)]
pub struct Div {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Div {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Div {
    // ======== Field Constants ========

    /// **DIV**
    ///
    /// set to 0xaa00 + div where
    /// div = 0 divides by 128
    /// div = 1-127 divides by div
    /// any other value sets div=128
    /// this register resets to div=32
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const DIV_OFFSET: u32 = 0;
    pub const DIV_MASK: u32 = 0x0000ffff;
    pub const DIV_PASS: u32 = 43520 << 0;
    pub const DIV_PASS_VAL: u32 = 43520;

    // ======== Field Accessors ========

    /// **DIV** `[15:0]` (RW)
    ///
    /// set to 0xaa00 + div where
    /// div = 0 divides by 128
    /// div = 1-127 divides by div
    /// any other value sets div=128
    /// this register resets to div=32
    #[inline(always)]
    pub fn div(&self) -> u32 {
        (self.value.read_bits() & Self::DIV_MASK) >> Self::DIV_OFFSET
    }

    /// Write **DIV** `[15:0]` (RW)
    ///
    /// set to 0xaa00 + div where
    /// div = 0 divides by 128
    /// div = 1-127 divides by div
    /// any other value sets div=128
    /// this register resets to div=32
    #[inline(always)]
    pub fn write_div(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DIV_MASK) | ((value << Self::DIV_OFFSET) & Self::DIV_MASK));
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

    /// Read **DIV** `[15:0]`
    ///
    /// set to 0xaa00 + div where
    /// div = 0 divides by 128
    /// div = 1-127 divides by div
    /// any other value sets div=128
    /// this register resets to div=32
    #[inline(always)]
    pub const fn div(&self) -> u32 {
        (self.bits & Div::DIV_MASK) >> Div::DIV_OFFSET
    }

    /// Set **DIV** `[15:0]`
    ///
    /// set to 0xaa00 + div where
    /// div = 0 divides by 128
    /// div = 1-127 divides by div
    /// any other value sets div=128
    /// this register resets to div=32
    #[inline(always)]
    pub const fn with_div(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Div::DIV_MASK) | ((value << Div::DIV_OFFSET) & Div::DIV_MASK)
        }
    }

}

impl Default for DivVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Div {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DivVal {
        DivVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DivVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DivVal) -> DivVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// PHASE Register
///
/// Controls the phase shifted output
///
/// Address offset: 0x0018
/// Reset value: 0x00000008
///
/// ## Fields
///
/// - **PASSWD** `[11:4]` (RW)
///
///   set to 0xaa
///   any other value enables the output with shift=0
///
/// - **ENABLE** `[3]` (RW)
///
///   enable the phase-shifted output
///   this can be changed on-the-fly
///
/// - **FLIP** `[2]` (RW)
///
///   invert the phase-shifted output
///   this is ignored when div=1
///
/// - **SHIFT** `[1:0]` (RW)
///
///   phase shift the phase-shifted output by SHIFT input clocks
///   this can be changed on-the-fly
///   must be set to 0 before setting div=1
#[repr(transparent)]
pub struct Phase {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Phase {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Phase {
    // ======== Field Constants ========

    /// **PASSWD**
    ///
    /// set to 0xaa
    /// any other value enables the output with shift=0
    ///
    /// Bits `[11:4]`, mask `0x00000ff0`
    pub const PASSWD_OFFSET: u32 = 4;
    pub const PASSWD_MASK: u32 = 0x00000ff0;

    /// **ENABLE**
    ///
    /// enable the phase-shifted output
    /// this can be changed on-the-fly
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const ENABLE_OFFSET: u32 = 3;
    pub const ENABLE_MASK: u32 = 0x00000008;
    pub const ENABLE: u32 = 1 << 3;

    /// **FLIP**
    ///
    /// invert the phase-shifted output
    /// this is ignored when div=1
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const FLIP_OFFSET: u32 = 2;
    pub const FLIP_MASK: u32 = 0x00000004;
    pub const FLIP: u32 = 1 << 2;

    /// **SHIFT**
    ///
    /// phase shift the phase-shifted output by SHIFT input clocks
    /// this can be changed on-the-fly
    /// must be set to 0 before setting div=1
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const SHIFT_OFFSET: u32 = 0;
    pub const SHIFT_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **PASSWD** `[11:4]` (RW)
    ///
    /// set to 0xaa
    /// any other value enables the output with shift=0
    #[inline(always)]
    pub fn passwd(&self) -> u32 {
        (self.value.read_bits() & Self::PASSWD_MASK) >> Self::PASSWD_OFFSET
    }

    /// Write **PASSWD** `[11:4]` (RW)
    ///
    /// set to 0xaa
    /// any other value enables the output with shift=0
    #[inline(always)]
    pub fn write_passwd(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::PASSWD_MASK) | ((value << Self::PASSWD_OFFSET) & Self::PASSWD_MASK));
    }

    /// **ENABLE** `[3]` (RW)
    ///
    /// enable the phase-shifted output
    /// this can be changed on-the-fly
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

    /// **FLIP** `[2]` (RW)
    ///
    /// invert the phase-shifted output
    /// this is ignored when div=1
    #[inline(always)]
    pub fn flip(&self) -> bool {
        (self.value.read_bits() & Self::FLIP_MASK) != 0
    }

    /// Set FLIP bit atomically (RW)
    #[inline(always)]
    pub fn set_flip(&self) {
        self.value.set_bits(Self::FLIP_MASK);
    }

    /// Clear FLIP bit atomically (RW)
    #[inline(always)]
    pub fn clr_flip(&self) {
        self.value.clr_bits(Self::FLIP_MASK);
    }

    /// **SHIFT** `[1:0]` (RW)
    ///
    /// phase shift the phase-shifted output by SHIFT input clocks
    /// this can be changed on-the-fly
    /// must be set to 0 before setting div=1
    #[inline(always)]
    pub fn shift(&self) -> u32 {
        (self.value.read_bits() & Self::SHIFT_MASK) >> Self::SHIFT_OFFSET
    }

    /// Write **SHIFT** `[1:0]` (RW)
    ///
    /// phase shift the phase-shifted output by SHIFT input clocks
    /// this can be changed on-the-fly
    /// must be set to 0 before setting div=1
    #[inline(always)]
    pub fn write_shift(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SHIFT_MASK) | ((value << Self::SHIFT_OFFSET) & Self::SHIFT_MASK));
    }

}

/// PHASE value type for batch read/modify/write operations
///
/// Use `phase.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `phase.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PhaseVal {
    bits: u32,
}

impl PhaseVal {
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
        Self { bits: 0x00000008 }
    }

    /// Read **PASSWD** `[11:4]`
    ///
    /// set to 0xaa
    /// any other value enables the output with shift=0
    #[inline(always)]
    pub const fn passwd(&self) -> u32 {
        (self.bits & Phase::PASSWD_MASK) >> Phase::PASSWD_OFFSET
    }

    /// Read **ENABLE** `[3:3]`
    ///
    /// enable the phase-shifted output
    /// this can be changed on-the-fly
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        (self.bits & Phase::ENABLE_MASK) != 0
    }

    /// Read **FLIP** `[2:2]`
    ///
    /// invert the phase-shifted output
    /// this is ignored when div=1
    #[inline(always)]
    pub const fn flip(&self) -> bool {
        (self.bits & Phase::FLIP_MASK) != 0
    }

    /// Read **SHIFT** `[1:0]`
    ///
    /// phase shift the phase-shifted output by SHIFT input clocks
    /// this can be changed on-the-fly
    /// must be set to 0 before setting div=1
    #[inline(always)]
    pub const fn shift(&self) -> u32 {
        (self.bits & Phase::SHIFT_MASK) >> Phase::SHIFT_OFFSET
    }

    /// Set **PASSWD** `[11:4]`
    ///
    /// set to 0xaa
    /// any other value enables the output with shift=0
    #[inline(always)]
    pub const fn with_passwd(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Phase::PASSWD_MASK) | ((value << Phase::PASSWD_OFFSET) & Phase::PASSWD_MASK)
        }
    }

    /// Set **ENABLE** `[3:3]`
    ///
    /// enable the phase-shifted output
    /// this can be changed on-the-fly
    #[inline(always)]
    pub const fn with_enable(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Phase::ENABLE_MASK
            } else {
                self.bits & !Phase::ENABLE_MASK
            }
        }
    }

    /// Set **FLIP** `[2:2]`
    ///
    /// invert the phase-shifted output
    /// this is ignored when div=1
    #[inline(always)]
    pub const fn with_flip(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Phase::FLIP_MASK
            } else {
                self.bits & !Phase::FLIP_MASK
            }
        }
    }

    /// Set **SHIFT** `[1:0]`
    ///
    /// phase shift the phase-shifted output by SHIFT input clocks
    /// this can be changed on-the-fly
    /// must be set to 0 before setting div=1
    #[inline(always)]
    pub const fn with_shift(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Phase::SHIFT_MASK) | ((value << Phase::SHIFT_OFFSET) & Phase::SHIFT_MASK)
        }
    }

}

impl Default for PhaseVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Phase {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PhaseVal {
        PhaseVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: PhaseVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(PhaseVal) -> PhaseVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// STATUS Register
///
/// Ring Oscillator Status
///
/// Address offset: 0x001c
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
///   An invalid value has been written to CTRL_ENABLE or CTRL_FREQ_RANGE or FREQA or FREQB or DIV or PHASE or DORMANT
///
/// - **DIV_RUNNING** `[16]` (RO)
///
///   post-divider is running
///   this resets to 0 but transitions to 1 during chip startup
///
/// - **ENABLED** `[12]` (RO)
///
///   Oscillator is enabled but not necessarily running and stable
///   this resets to 0 but transitions to 1 during chip startup
#[repr(transparent)]
pub struct Status {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Status {
    type Target = crate::common::VolatileCell<u32>;
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
    /// An invalid value has been written to CTRL_ENABLE or CTRL_FREQ_RANGE or FREQA or FREQB or DIV or PHASE or DORMANT
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const BADWRITE_OFFSET: u32 = 24;
    pub const BADWRITE_MASK: u32 = 0x01000000;
    pub const BADWRITE: u32 = 1 << 24;

    /// **DIV_RUNNING**
    ///
    /// post-divider is running
    /// this resets to 0 but transitions to 1 during chip startup
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const DIV_RUNNING_OFFSET: u32 = 16;
    pub const DIV_RUNNING_MASK: u32 = 0x00010000;
    pub const DIV_RUNNING: u32 = 1 << 16;

    /// **ENABLED**
    ///
    /// Oscillator is enabled but not necessarily running and stable
    /// this resets to 0 but transitions to 1 during chip startup
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const ENABLED_OFFSET: u32 = 12;
    pub const ENABLED_MASK: u32 = 0x00001000;
    pub const ENABLED: u32 = 1 << 12;

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
    /// An invalid value has been written to CTRL_ENABLE or CTRL_FREQ_RANGE or FREQA or FREQB or DIV or PHASE or DORMANT
    #[inline(always)]
    pub fn badwrite(&self) -> bool {
        (self.value.read_bits() & Self::BADWRITE_MASK) != 0
    }

    /// Set BADWRITE bit atomically (RW)
    #[inline(always)]
    pub fn set_badwrite(&self) {
        self.value.set_bits(Self::BADWRITE_MASK);
    }

    /// Clear BADWRITE bit atomically (RW)
    #[inline(always)]
    pub fn clr_badwrite(&self) {
        self.value.clr_bits(Self::BADWRITE_MASK);
    }

    /// **DIV_RUNNING** `[16]` (RO)
    ///
    /// post-divider is running
    /// this resets to 0 but transitions to 1 during chip startup
    #[inline(always)]
    pub fn div_running(&self) -> bool {
        (self.value.read_bits() & Self::DIV_RUNNING_MASK) != 0
    }

    /// **ENABLED** `[12]` (RO)
    ///
    /// Oscillator is enabled but not necessarily running and stable
    /// this resets to 0 but transitions to 1 during chip startup
    #[inline(always)]
    pub fn enabled(&self) -> bool {
        (self.value.read_bits() & Self::ENABLED_MASK) != 0
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
    /// An invalid value has been written to CTRL_ENABLE or CTRL_FREQ_RANGE or FREQA or FREQB or DIV or PHASE or DORMANT
    #[inline(always)]
    pub const fn badwrite(&self) -> bool {
        (self.bits & Status::BADWRITE_MASK) != 0
    }

    /// Read **DIV_RUNNING** `[16:16]`
    ///
    /// post-divider is running
    /// this resets to 0 but transitions to 1 during chip startup
    #[inline(always)]
    pub const fn div_running(&self) -> bool {
        (self.bits & Status::DIV_RUNNING_MASK) != 0
    }

    /// Read **ENABLED** `[12:12]`
    ///
    /// Oscillator is enabled but not necessarily running and stable
    /// this resets to 0 but transitions to 1 during chip startup
    #[inline(always)]
    pub const fn enabled(&self) -> bool {
        (self.bits & Status::ENABLED_MASK) != 0
    }

    /// Set **BADWRITE** `[24:24]`
    ///
    /// An invalid value has been written to CTRL_ENABLE or CTRL_FREQ_RANGE or FREQA or FREQB or DIV or PHASE or DORMANT
    #[inline(always)]
    pub const fn with_badwrite(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Status::BADWRITE_MASK
            } else {
                self.bits & !Status::BADWRITE_MASK
            }
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

/// RANDOMBIT Register
///
/// This just reads the state of the oscillator output so randomness is compromised if the ring oscillator is stopped or run at a harmonic of the bus frequency
///
/// Address offset: 0x0020
/// Reset value: 0x00000001
///
/// ## Fields
///
/// - **RANDOMBIT** `[0]` (RO)
#[repr(transparent)]
pub struct Randombit {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Randombit {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Randombit {
    // ======== Field Constants ========

    /// **RANDOMBIT**
    /// Bits `[0:0]`, mask `0x00000001`
    pub const RANDOMBIT_OFFSET: u32 = 0;
    pub const RANDOMBIT_MASK: u32 = 0x00000001;
    pub const RANDOMBIT: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **RANDOMBIT** `[0]` (RO)
    #[inline(always)]
    pub fn randombit(&self) -> bool {
        (self.value.read_bits() & Self::RANDOMBIT_MASK) != 0
    }

}

/// RANDOMBIT value type for batch read/modify/write operations
///
/// Use `randombit.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `randombit.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RandombitVal {
    bits: u32,
}

impl RandombitVal {
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
        Self { bits: 0x00000001 }
    }

    /// Read **RANDOMBIT** `[0:0]`
    #[inline(always)]
    pub const fn randombit(&self) -> bool {
        (self.bits & Randombit::RANDOMBIT_MASK) != 0
    }

}

impl Default for RandombitVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Randombit {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RandombitVal {
        RandombitVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RandombitVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RandombitVal) -> RandombitVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// COUNT Register
///
/// A down counter running at the ROSC frequency which counts to zero and stops.
/// To start the counter write a non-zero value.
/// Can be used for short software pauses when setting up time sensitive hardware.
///
/// Address offset: 0x0024
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **COUNT** `[15:0]` (RW)
#[repr(transparent)]
pub struct Count {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Count {
    type Target = crate::common::VolatileCell<u32>;
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
        self.value.modify_bits(|r| (r & !Self::COUNT_MASK) | ((value << Self::COUNT_OFFSET) & Self::COUNT_MASK));
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
            bits: (self.bits & !Count::COUNT_MASK) | ((value << Count::COUNT_OFFSET) & Count::COUNT_MASK)
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
    /// Ring Oscillator control
    pub ctrl: Ctrl,
    /// 0x0004
    /// The FREQA & FREQB registers control the frequency by controlling the drive strength of each stage
    /// The drive strength has 4 levels determined by the number of bits set
    /// Increasing the number of bits set increases the drive strength and increases the oscillation frequency
    /// 0 bits set is the default drive strength
    /// 1 bit set doubles the drive strength
    /// 2 bits set triples drive strength
    /// 3 bits set quadruples drive strength
    /// For frequency randomisation set both DS0_RANDOM=1 & DS1_RANDOM=1
    pub freqa: Freqa,
    /// 0x0008
    /// For a detailed description see freqa register
    pub freqb: Freqb,
    /// 0x000c
    /// Loads a value to the LFSR randomiser
    pub random: Random,
    /// 0x0010
    /// Ring Oscillator pause control
    pub dormant: Dormant,
    /// 0x0014
    /// Controls the output divider
    pub div: Div,
    /// 0x0018
    /// Controls the phase shifted output
    pub phase: Phase,
    /// 0x001c
    /// Ring Oscillator Status
    pub status: Status,
    /// 0x0020
    /// This just reads the state of the oscillator output so randomness is compromised if the ring oscillator is stopped or run at a harmonic of the bus frequency
    pub randombit: Randombit,
    /// 0x0024
    /// A down counter running at the ROSC frequency which counts to zero and stops.
    /// To start the counter write a non-zero value.
    /// Can be used for short software pauses when setting up time sensitive hardware.
    pub count: Count,
}

