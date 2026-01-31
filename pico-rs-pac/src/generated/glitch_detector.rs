//! GLITCH_DETECTOR
//! Glitch detector controls

#![allow(dead_code)]

pub const BASE: u32 = 0x40158000;

/// ARM Register
///
/// Forcibly arm the glitch detectors, if they are not already armed by OTP. When armed, any individual detector trigger will cause a restart of the switched core power domain's power-on reset state machine.
/// Glitch detector triggers are recorded accumulatively in TRIG_STATUS. If the system is reset by a glitch detector trigger, this is recorded in POWMAN_CHIP_RESET.
/// This register is Secure read/write only.
///
/// Address offset: 0x0000
/// Reset value: 0x00005bad
///
/// ## Fields
///
/// - **ARM** `[15:0]` (RW)
#[repr(transparent)]
pub struct Arm {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Arm {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Arm {
    // ======== Field Constants ========

    /// **ARM**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const ARM_OFFSET: u32 = 0;
    pub const ARM_MASK: u32 = 0x0000ffff;
    /// no = 23469
    /// Do not force the glitch detectors to be armed
    pub const ARM_NO: u32 = 23469 << 0;
    pub const ARM_NO_VAL: u32 = 23469;
    /// yes = 0
    /// Force the glitch detectors to be armed. (Any value other than ARM_NO counts as YES)
    pub const ARM_YES: u32 = 0 << 0;
    pub const ARM_YES_VAL: u32 = 0;

    // ======== Field Accessors ========

    /// **ARM** `[15:0]` (RW)
    #[inline(always)]
    pub fn arm(&self) -> u32 {
        (self.value.read_bits() & Self::ARM_MASK) >> Self::ARM_OFFSET
    }

    /// Write **ARM** `[15:0]` (RW)
    #[inline(always)]
    pub fn write_arm(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::ARM_MASK) | ((value << Self::ARM_OFFSET) & Self::ARM_MASK));
    }

}

/// ARM value type for batch read/modify/write operations
///
/// Use `arm.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `arm.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ArmVal {
    bits: u32,
}

impl ArmVal {
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
        Self { bits: 0x00005bad }
    }

    /// Read **ARM** `[15:0]`
    #[inline(always)]
    pub const fn arm(&self) -> u32 {
        (self.bits & Arm::ARM_MASK) >> Arm::ARM_OFFSET
    }

    /// Set **ARM** `[15:0]`
    #[inline(always)]
    pub const fn with_arm(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Arm::ARM_MASK) | ((value << Arm::ARM_OFFSET) & Arm::ARM_MASK)
        }
    }

}

impl Default for ArmVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Arm {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ArmVal {
        ArmVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ArmVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ArmVal) -> ArmVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// DISARM Register
///
/// Address offset: 0x0004
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **DISARM** `[15:0]` (RW)
///   Forcibly disarm the glitch detectors, if they are armed by OTP. Ignored if ARM is YES.
///   This register is Secure read/write only.
#[repr(transparent)]
pub struct Disarm {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Disarm {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Disarm {
    // ======== Field Constants ========

    /// **DISARM**
    /// Forcibly disarm the glitch detectors, if they are armed by OTP. Ignored if ARM is YES.
    /// This register is Secure read/write only.
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const DISARM_OFFSET: u32 = 0;
    pub const DISARM_MASK: u32 = 0x0000ffff;
    /// no = 0
    /// Do not disarm the glitch detectors. (Any value other than DISARM_YES counts as NO)
    pub const DISARM_NO: u32 = 0 << 0;
    pub const DISARM_NO_VAL: u32 = 0;
    /// yes = 56495
    /// Disarm the glitch detectors
    pub const DISARM_YES: u32 = 56495 << 0;
    pub const DISARM_YES_VAL: u32 = 56495;

    // ======== Field Accessors ========

    /// **DISARM** `[15:0]` (RW)
    /// Forcibly disarm the glitch detectors, if they are armed by OTP. Ignored if ARM is YES.
    /// This register is Secure read/write only.
    #[inline(always)]
    pub fn disarm(&self) -> u32 {
        (self.value.read_bits() & Self::DISARM_MASK) >> Self::DISARM_OFFSET
    }

    /// Write **DISARM** `[15:0]` (RW)
    /// Forcibly disarm the glitch detectors, if they are armed by OTP. Ignored if ARM is YES.
    /// This register is Secure read/write only.
    #[inline(always)]
    pub fn write_disarm(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DISARM_MASK) | ((value << Self::DISARM_OFFSET) & Self::DISARM_MASK));
    }

}

/// DISARM value type for batch read/modify/write operations
///
/// Use `disarm.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `disarm.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DisarmVal {
    bits: u32,
}

impl DisarmVal {
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

    /// Read **DISARM** `[15:0]`
    /// Forcibly disarm the glitch detectors, if they are armed by OTP. Ignored if ARM is YES.
    /// This register is Secure read/write only.
    #[inline(always)]
    pub const fn disarm(&self) -> u32 {
        (self.bits & Disarm::DISARM_MASK) >> Disarm::DISARM_OFFSET
    }

    /// Set **DISARM** `[15:0]`
    /// Forcibly disarm the glitch detectors, if they are armed by OTP. Ignored if ARM is YES.
    /// This register is Secure read/write only.
    #[inline(always)]
    pub const fn with_disarm(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Disarm::DISARM_MASK) | ((value << Disarm::DISARM_OFFSET) & Disarm::DISARM_MASK)
        }
    }

}

impl Default for DisarmVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Disarm {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DisarmVal {
        DisarmVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DisarmVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DisarmVal) -> DisarmVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SENSITIVITY Register
///
/// Adjust the sensitivity of glitch detectors to values other than their OTP-provided defaults.
/// This register is Secure read/write only.
///
/// Address offset: 0x0008
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **DEFAULT** `[31:24]` (RW)
/// - **DET3_INV** `[15:14]` (RW)
///   Must be the inverse of DET3, else the default value is used.
/// - **DET2_INV** `[13:12]` (RW)
///   Must be the inverse of DET2, else the default value is used.
/// - **DET1_INV** `[11:10]` (RW)
///   Must be the inverse of DET1, else the default value is used.
/// - **DET0_INV** `[9:8]` (RW)
///   Must be the inverse of DET0, else the default value is used.
/// - **DET3** `[7:6]` (RW)
///   Set sensitivity for detector 3. Higher values are more sensitive.
/// - **DET2** `[5:4]` (RW)
///   Set sensitivity for detector 2. Higher values are more sensitive.
/// - **DET1** `[3:2]` (RW)
///   Set sensitivity for detector 1. Higher values are more sensitive.
/// - **DET0** `[1:0]` (RW)
///   Set sensitivity for detector 0. Higher values are more sensitive.
#[repr(transparent)]
pub struct Sensitivity {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Sensitivity {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sensitivity {
    // ======== Field Constants ========

    /// **DEFAULT**
    /// Bits `[31:24]`, mask `0xff000000`
    pub const DEFAULT_OFFSET: u32 = 24;
    pub const DEFAULT_MASK: u32 = 0xff000000;
    /// yes = 0
    /// Use the default sensitivity configured in OTP for all detectors. (Any value other than DEFAULT_NO counts as YES)
    pub const DEFAULT_YES: u32 = 0 << 24;
    pub const DEFAULT_YES_VAL: u32 = 0;
    /// no = 222
    /// Do not use the default sensitivity configured in OTP. Instead use the value from this register.
    pub const DEFAULT_NO: u32 = 222 << 24;
    pub const DEFAULT_NO_VAL: u32 = 222;

    /// **DET3_INV**
    /// Must be the inverse of DET3, else the default value is used.
    /// Bits `[15:14]`, mask `0x0000c000`
    pub const DET3_INV_OFFSET: u32 = 14;
    pub const DET3_INV_MASK: u32 = 0x0000c000;

    /// **DET2_INV**
    /// Must be the inverse of DET2, else the default value is used.
    /// Bits `[13:12]`, mask `0x00003000`
    pub const DET2_INV_OFFSET: u32 = 12;
    pub const DET2_INV_MASK: u32 = 0x00003000;

    /// **DET1_INV**
    /// Must be the inverse of DET1, else the default value is used.
    /// Bits `[11:10]`, mask `0x00000c00`
    pub const DET1_INV_OFFSET: u32 = 10;
    pub const DET1_INV_MASK: u32 = 0x00000c00;

    /// **DET0_INV**
    /// Must be the inverse of DET0, else the default value is used.
    /// Bits `[9:8]`, mask `0x00000300`
    pub const DET0_INV_OFFSET: u32 = 8;
    pub const DET0_INV_MASK: u32 = 0x00000300;

    /// **DET3**
    /// Set sensitivity for detector 3. Higher values are more sensitive.
    /// Bits `[7:6]`, mask `0x000000c0`
    pub const DET3_OFFSET: u32 = 6;
    pub const DET3_MASK: u32 = 0x000000c0;

    /// **DET2**
    /// Set sensitivity for detector 2. Higher values are more sensitive.
    /// Bits `[5:4]`, mask `0x00000030`
    pub const DET2_OFFSET: u32 = 4;
    pub const DET2_MASK: u32 = 0x00000030;

    /// **DET1**
    /// Set sensitivity for detector 1. Higher values are more sensitive.
    /// Bits `[3:2]`, mask `0x0000000c`
    pub const DET1_OFFSET: u32 = 2;
    pub const DET1_MASK: u32 = 0x0000000c;

    /// **DET0**
    /// Set sensitivity for detector 0. Higher values are more sensitive.
    /// Bits `[1:0]`, mask `0x00000003`
    pub const DET0_OFFSET: u32 = 0;
    pub const DET0_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **DEFAULT** `[31:24]` (RW)
    #[inline(always)]
    pub fn default(&self) -> u32 {
        (self.value.read_bits() & Self::DEFAULT_MASK) >> Self::DEFAULT_OFFSET
    }

    /// Write **DEFAULT** `[31:24]` (RW)
    #[inline(always)]
    pub fn write_default(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DEFAULT_MASK) | ((value << Self::DEFAULT_OFFSET) & Self::DEFAULT_MASK));
    }

    /// **DET3_INV** `[15:14]` (RW)
    /// Must be the inverse of DET3, else the default value is used.
    #[inline(always)]
    pub fn det3_inv(&self) -> u32 {
        (self.value.read_bits() & Self::DET3_INV_MASK) >> Self::DET3_INV_OFFSET
    }

    /// Write **DET3_INV** `[15:14]` (RW)
    /// Must be the inverse of DET3, else the default value is used.
    #[inline(always)]
    pub fn write_det3_inv(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DET3_INV_MASK) | ((value << Self::DET3_INV_OFFSET) & Self::DET3_INV_MASK));
    }

    /// **DET2_INV** `[13:12]` (RW)
    /// Must be the inverse of DET2, else the default value is used.
    #[inline(always)]
    pub fn det2_inv(&self) -> u32 {
        (self.value.read_bits() & Self::DET2_INV_MASK) >> Self::DET2_INV_OFFSET
    }

    /// Write **DET2_INV** `[13:12]` (RW)
    /// Must be the inverse of DET2, else the default value is used.
    #[inline(always)]
    pub fn write_det2_inv(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DET2_INV_MASK) | ((value << Self::DET2_INV_OFFSET) & Self::DET2_INV_MASK));
    }

    /// **DET1_INV** `[11:10]` (RW)
    /// Must be the inverse of DET1, else the default value is used.
    #[inline(always)]
    pub fn det1_inv(&self) -> u32 {
        (self.value.read_bits() & Self::DET1_INV_MASK) >> Self::DET1_INV_OFFSET
    }

    /// Write **DET1_INV** `[11:10]` (RW)
    /// Must be the inverse of DET1, else the default value is used.
    #[inline(always)]
    pub fn write_det1_inv(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DET1_INV_MASK) | ((value << Self::DET1_INV_OFFSET) & Self::DET1_INV_MASK));
    }

    /// **DET0_INV** `[9:8]` (RW)
    /// Must be the inverse of DET0, else the default value is used.
    #[inline(always)]
    pub fn det0_inv(&self) -> u32 {
        (self.value.read_bits() & Self::DET0_INV_MASK) >> Self::DET0_INV_OFFSET
    }

    /// Write **DET0_INV** `[9:8]` (RW)
    /// Must be the inverse of DET0, else the default value is used.
    #[inline(always)]
    pub fn write_det0_inv(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DET0_INV_MASK) | ((value << Self::DET0_INV_OFFSET) & Self::DET0_INV_MASK));
    }

    /// **DET3** `[7:6]` (RW)
    /// Set sensitivity for detector 3. Higher values are more sensitive.
    #[inline(always)]
    pub fn det3(&self) -> u32 {
        (self.value.read_bits() & Self::DET3_MASK) >> Self::DET3_OFFSET
    }

    /// Write **DET3** `[7:6]` (RW)
    /// Set sensitivity for detector 3. Higher values are more sensitive.
    #[inline(always)]
    pub fn write_det3(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DET3_MASK) | ((value << Self::DET3_OFFSET) & Self::DET3_MASK));
    }

    /// **DET2** `[5:4]` (RW)
    /// Set sensitivity for detector 2. Higher values are more sensitive.
    #[inline(always)]
    pub fn det2(&self) -> u32 {
        (self.value.read_bits() & Self::DET2_MASK) >> Self::DET2_OFFSET
    }

    /// Write **DET2** `[5:4]` (RW)
    /// Set sensitivity for detector 2. Higher values are more sensitive.
    #[inline(always)]
    pub fn write_det2(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DET2_MASK) | ((value << Self::DET2_OFFSET) & Self::DET2_MASK));
    }

    /// **DET1** `[3:2]` (RW)
    /// Set sensitivity for detector 1. Higher values are more sensitive.
    #[inline(always)]
    pub fn det1(&self) -> u32 {
        (self.value.read_bits() & Self::DET1_MASK) >> Self::DET1_OFFSET
    }

    /// Write **DET1** `[3:2]` (RW)
    /// Set sensitivity for detector 1. Higher values are more sensitive.
    #[inline(always)]
    pub fn write_det1(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DET1_MASK) | ((value << Self::DET1_OFFSET) & Self::DET1_MASK));
    }

    /// **DET0** `[1:0]` (RW)
    /// Set sensitivity for detector 0. Higher values are more sensitive.
    #[inline(always)]
    pub fn det0(&self) -> u32 {
        (self.value.read_bits() & Self::DET0_MASK) >> Self::DET0_OFFSET
    }

    /// Write **DET0** `[1:0]` (RW)
    /// Set sensitivity for detector 0. Higher values are more sensitive.
    #[inline(always)]
    pub fn write_det0(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DET0_MASK) | ((value << Self::DET0_OFFSET) & Self::DET0_MASK));
    }

}

/// SENSITIVITY value type for batch read/modify/write operations
///
/// Use `sensitivity.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sensitivity.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SensitivityVal {
    bits: u32,
}

impl SensitivityVal {
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

    /// Read **DEFAULT** `[31:24]`
    #[inline(always)]
    pub const fn default(&self) -> u32 {
        (self.bits & Sensitivity::DEFAULT_MASK) >> Sensitivity::DEFAULT_OFFSET
    }

    /// Read **DET3_INV** `[15:14]`
    /// Must be the inverse of DET3, else the default value is used.
    #[inline(always)]
    pub const fn det3_inv(&self) -> u32 {
        (self.bits & Sensitivity::DET3_INV_MASK) >> Sensitivity::DET3_INV_OFFSET
    }

    /// Read **DET2_INV** `[13:12]`
    /// Must be the inverse of DET2, else the default value is used.
    #[inline(always)]
    pub const fn det2_inv(&self) -> u32 {
        (self.bits & Sensitivity::DET2_INV_MASK) >> Sensitivity::DET2_INV_OFFSET
    }

    /// Read **DET1_INV** `[11:10]`
    /// Must be the inverse of DET1, else the default value is used.
    #[inline(always)]
    pub const fn det1_inv(&self) -> u32 {
        (self.bits & Sensitivity::DET1_INV_MASK) >> Sensitivity::DET1_INV_OFFSET
    }

    /// Read **DET0_INV** `[9:8]`
    /// Must be the inverse of DET0, else the default value is used.
    #[inline(always)]
    pub const fn det0_inv(&self) -> u32 {
        (self.bits & Sensitivity::DET0_INV_MASK) >> Sensitivity::DET0_INV_OFFSET
    }

    /// Read **DET3** `[7:6]`
    /// Set sensitivity for detector 3. Higher values are more sensitive.
    #[inline(always)]
    pub const fn det3(&self) -> u32 {
        (self.bits & Sensitivity::DET3_MASK) >> Sensitivity::DET3_OFFSET
    }

    /// Read **DET2** `[5:4]`
    /// Set sensitivity for detector 2. Higher values are more sensitive.
    #[inline(always)]
    pub const fn det2(&self) -> u32 {
        (self.bits & Sensitivity::DET2_MASK) >> Sensitivity::DET2_OFFSET
    }

    /// Read **DET1** `[3:2]`
    /// Set sensitivity for detector 1. Higher values are more sensitive.
    #[inline(always)]
    pub const fn det1(&self) -> u32 {
        (self.bits & Sensitivity::DET1_MASK) >> Sensitivity::DET1_OFFSET
    }

    /// Read **DET0** `[1:0]`
    /// Set sensitivity for detector 0. Higher values are more sensitive.
    #[inline(always)]
    pub const fn det0(&self) -> u32 {
        (self.bits & Sensitivity::DET0_MASK) >> Sensitivity::DET0_OFFSET
    }

    /// Set **DEFAULT** `[31:24]`
    #[inline(always)]
    pub const fn with_default(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sensitivity::DEFAULT_MASK) | ((value << Sensitivity::DEFAULT_OFFSET) & Sensitivity::DEFAULT_MASK)
        }
    }

    /// Set **DET3_INV** `[15:14]`
    /// Must be the inverse of DET3, else the default value is used.
    #[inline(always)]
    pub const fn with_det3_inv(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sensitivity::DET3_INV_MASK) | ((value << Sensitivity::DET3_INV_OFFSET) & Sensitivity::DET3_INV_MASK)
        }
    }

    /// Set **DET2_INV** `[13:12]`
    /// Must be the inverse of DET2, else the default value is used.
    #[inline(always)]
    pub const fn with_det2_inv(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sensitivity::DET2_INV_MASK) | ((value << Sensitivity::DET2_INV_OFFSET) & Sensitivity::DET2_INV_MASK)
        }
    }

    /// Set **DET1_INV** `[11:10]`
    /// Must be the inverse of DET1, else the default value is used.
    #[inline(always)]
    pub const fn with_det1_inv(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sensitivity::DET1_INV_MASK) | ((value << Sensitivity::DET1_INV_OFFSET) & Sensitivity::DET1_INV_MASK)
        }
    }

    /// Set **DET0_INV** `[9:8]`
    /// Must be the inverse of DET0, else the default value is used.
    #[inline(always)]
    pub const fn with_det0_inv(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sensitivity::DET0_INV_MASK) | ((value << Sensitivity::DET0_INV_OFFSET) & Sensitivity::DET0_INV_MASK)
        }
    }

    /// Set **DET3** `[7:6]`
    /// Set sensitivity for detector 3. Higher values are more sensitive.
    #[inline(always)]
    pub const fn with_det3(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sensitivity::DET3_MASK) | ((value << Sensitivity::DET3_OFFSET) & Sensitivity::DET3_MASK)
        }
    }

    /// Set **DET2** `[5:4]`
    /// Set sensitivity for detector 2. Higher values are more sensitive.
    #[inline(always)]
    pub const fn with_det2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sensitivity::DET2_MASK) | ((value << Sensitivity::DET2_OFFSET) & Sensitivity::DET2_MASK)
        }
    }

    /// Set **DET1** `[3:2]`
    /// Set sensitivity for detector 1. Higher values are more sensitive.
    #[inline(always)]
    pub const fn with_det1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sensitivity::DET1_MASK) | ((value << Sensitivity::DET1_OFFSET) & Sensitivity::DET1_MASK)
        }
    }

    /// Set **DET0** `[1:0]`
    /// Set sensitivity for detector 0. Higher values are more sensitive.
    #[inline(always)]
    pub const fn with_det0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sensitivity::DET0_MASK) | ((value << Sensitivity::DET0_OFFSET) & Sensitivity::DET0_MASK)
        }
    }

}

impl Default for SensitivityVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Sensitivity {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SensitivityVal {
        SensitivityVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SensitivityVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SensitivityVal) -> SensitivityVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// LOCK Register
///
/// Address offset: 0x000c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **LOCK** `[7:0]` (RW)
///   Write any nonzero value to disable writes to ARM, DISARM, SENSITIVITY and LOCK. This register is Secure read/write only.
#[repr(transparent)]
pub struct Lock {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Lock {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Lock {
    // ======== Field Constants ========

    /// **LOCK**
    /// Write any nonzero value to disable writes to ARM, DISARM, SENSITIVITY and LOCK. This register is Secure read/write only.
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const LOCK_OFFSET: u32 = 0;
    pub const LOCK_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **LOCK** `[7:0]` (RW)
    /// Write any nonzero value to disable writes to ARM, DISARM, SENSITIVITY and LOCK. This register is Secure read/write only.
    #[inline(always)]
    pub fn lock(&self) -> u32 {
        (self.value.read_bits() & Self::LOCK_MASK) >> Self::LOCK_OFFSET
    }

    /// Write **LOCK** `[7:0]` (RW)
    /// Write any nonzero value to disable writes to ARM, DISARM, SENSITIVITY and LOCK. This register is Secure read/write only.
    #[inline(always)]
    pub fn write_lock(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::LOCK_MASK) | ((value << Self::LOCK_OFFSET) & Self::LOCK_MASK));
    }

}

/// LOCK value type for batch read/modify/write operations
///
/// Use `lock.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lock.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LockVal {
    bits: u32,
}

impl LockVal {
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

    /// Read **LOCK** `[7:0]`
    /// Write any nonzero value to disable writes to ARM, DISARM, SENSITIVITY and LOCK. This register is Secure read/write only.
    #[inline(always)]
    pub const fn lock(&self) -> u32 {
        (self.bits & Lock::LOCK_MASK) >> Lock::LOCK_OFFSET
    }

    /// Set **LOCK** `[7:0]`
    /// Write any nonzero value to disable writes to ARM, DISARM, SENSITIVITY and LOCK. This register is Secure read/write only.
    #[inline(always)]
    pub const fn with_lock(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Lock::LOCK_MASK) | ((value << Lock::LOCK_OFFSET) & Lock::LOCK_MASK)
        }
    }

}

impl Default for LockVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Lock {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LockVal {
        LockVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LockVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LockVal) -> LockVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// TRIG_STATUS Register
///
/// Set when a detector output triggers. Write-1-clear.
/// (May immediately return high if the detector remains in a failed state. Detectors can only be cleared by a full reset of the switched core power domain.)
/// This register is Secure read/write only.
///
/// Address offset: 0x0010
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **DET3** `[3]` (RW)
/// - **DET2** `[2]` (RW)
/// - **DET1** `[1]` (RW)
/// - **DET0** `[0]` (RW)
#[repr(transparent)]
pub struct TrigStatus {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TrigStatus {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TrigStatus {
    // ======== Field Constants ========

    /// **DET3**
    /// Bits `[3:3]`, mask `0x00000008`
    pub const DET3_OFFSET: u32 = 3;
    pub const DET3_MASK: u32 = 0x00000008;
    pub const DET3: u32 = 1 << 3;

    /// **DET2**
    /// Bits `[2:2]`, mask `0x00000004`
    pub const DET2_OFFSET: u32 = 2;
    pub const DET2_MASK: u32 = 0x00000004;
    pub const DET2: u32 = 1 << 2;

    /// **DET1**
    /// Bits `[1:1]`, mask `0x00000002`
    pub const DET1_OFFSET: u32 = 1;
    pub const DET1_MASK: u32 = 0x00000002;
    pub const DET1: u32 = 1 << 1;

    /// **DET0**
    /// Bits `[0:0]`, mask `0x00000001`
    pub const DET0_OFFSET: u32 = 0;
    pub const DET0_MASK: u32 = 0x00000001;
    pub const DET0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DET3** `[3]` (RW)
    #[inline(always)]
    pub fn det3(&self) -> bool {
        (self.value.read_bits() & Self::DET3_MASK) != 0
    }

    /// Set DET3 bit atomically (RW)
    #[inline(always)]
    pub fn set_det3(&self) {
        self.value.set_bits(Self::DET3_MASK);
    }

    /// Clear DET3 bit atomically (RW)
    #[inline(always)]
    pub fn clr_det3(&self) {
        self.value.clr_bits(Self::DET3_MASK);
    }

    /// **DET2** `[2]` (RW)
    #[inline(always)]
    pub fn det2(&self) -> bool {
        (self.value.read_bits() & Self::DET2_MASK) != 0
    }

    /// Set DET2 bit atomically (RW)
    #[inline(always)]
    pub fn set_det2(&self) {
        self.value.set_bits(Self::DET2_MASK);
    }

    /// Clear DET2 bit atomically (RW)
    #[inline(always)]
    pub fn clr_det2(&self) {
        self.value.clr_bits(Self::DET2_MASK);
    }

    /// **DET1** `[1]` (RW)
    #[inline(always)]
    pub fn det1(&self) -> bool {
        (self.value.read_bits() & Self::DET1_MASK) != 0
    }

    /// Set DET1 bit atomically (RW)
    #[inline(always)]
    pub fn set_det1(&self) {
        self.value.set_bits(Self::DET1_MASK);
    }

    /// Clear DET1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_det1(&self) {
        self.value.clr_bits(Self::DET1_MASK);
    }

    /// **DET0** `[0]` (RW)
    #[inline(always)]
    pub fn det0(&self) -> bool {
        (self.value.read_bits() & Self::DET0_MASK) != 0
    }

    /// Set DET0 bit atomically (RW)
    #[inline(always)]
    pub fn set_det0(&self) {
        self.value.set_bits(Self::DET0_MASK);
    }

    /// Clear DET0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_det0(&self) {
        self.value.clr_bits(Self::DET0_MASK);
    }

}

/// TRIG_STATUS value type for batch read/modify/write operations
///
/// Use `trigstatus.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `trigstatus.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TrigStatusVal {
    bits: u32,
}

impl TrigStatusVal {
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

    /// Read **DET3** `[3:3]`
    #[inline(always)]
    pub const fn det3(&self) -> bool {
        (self.bits & TrigStatus::DET3_MASK) != 0
    }

    /// Read **DET2** `[2:2]`
    #[inline(always)]
    pub const fn det2(&self) -> bool {
        (self.bits & TrigStatus::DET2_MASK) != 0
    }

    /// Read **DET1** `[1:1]`
    #[inline(always)]
    pub const fn det1(&self) -> bool {
        (self.bits & TrigStatus::DET1_MASK) != 0
    }

    /// Read **DET0** `[0:0]`
    #[inline(always)]
    pub const fn det0(&self) -> bool {
        (self.bits & TrigStatus::DET0_MASK) != 0
    }

    /// Set **DET3** `[3:3]`
    #[inline(always)]
    pub const fn with_det3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TrigStatus::DET3_MASK
            } else {
                self.bits & !TrigStatus::DET3_MASK
            }
        }
    }

    /// Set **DET2** `[2:2]`
    #[inline(always)]
    pub const fn with_det2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TrigStatus::DET2_MASK
            } else {
                self.bits & !TrigStatus::DET2_MASK
            }
        }
    }

    /// Set **DET1** `[1:1]`
    #[inline(always)]
    pub const fn with_det1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TrigStatus::DET1_MASK
            } else {
                self.bits & !TrigStatus::DET1_MASK
            }
        }
    }

    /// Set **DET0** `[0:0]`
    #[inline(always)]
    pub const fn with_det0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TrigStatus::DET0_MASK
            } else {
                self.bits & !TrigStatus::DET0_MASK
            }
        }
    }

}

impl Default for TrigStatusVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl TrigStatus {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TrigStatusVal {
        TrigStatusVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TrigStatusVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TrigStatusVal) -> TrigStatusVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// TRIG_FORCE Register
///
/// Simulate the firing of one or more detectors. Writing ones to this register will set the matching bits in STATUS_TRIG.
/// If the glitch detectors are currently armed, writing ones will also immediately reset the switched core power domain, and set the reset reason latches in POWMAN_CHIP_RESET to indicate a glitch detector resets.
/// This register is Secure read/write only.
///
/// Address offset: 0x0014
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **TRIG_FORCE** `[3:0]` (WO)
#[repr(transparent)]
pub struct TrigForce {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TrigForce {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TrigForce {
    // ======== Field Constants ========

    /// **TRIG_FORCE**
    /// Bits `[3:0]`, mask `0x0000000f`
    pub const TRIG_FORCE_OFFSET: u32 = 0;
    pub const TRIG_FORCE_MASK: u32 = 0x0000000f;

    // ======== Field Accessors ========

    /// **TRIG_FORCE** `[3:0]` (WO)
    /// Write **TRIG_FORCE** `[3:0]` (WO)
    #[inline(always)]
    pub fn write_trig_force(&self, value: u32) {
        self.value.write_bits((value << Self::TRIG_FORCE_OFFSET) & Self::TRIG_FORCE_MASK);
    }

}

/// TRIG_FORCE value type for batch read/modify/write operations
///
/// Use `trigforce.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `trigforce.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TrigForceVal {
    bits: u32,
}

impl TrigForceVal {
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

    /// Set **TRIG_FORCE** `[3:0]`
    #[inline(always)]
    pub const fn with_trig_force(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TrigForce::TRIG_FORCE_MASK) | ((value << TrigForce::TRIG_FORCE_OFFSET) & TrigForce::TRIG_FORCE_MASK)
        }
    }

}

impl Default for TrigForceVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl TrigForce {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TrigForceVal {
        TrigForceVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TrigForceVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TrigForceVal) -> TrigForceVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// Forcibly arm the glitch detectors, if they are not already armed by OTP. When armed, any individual detector trigger will cause a restart of the switched core power domain's power-on reset state machine.
    /// Glitch detector triggers are recorded accumulatively in TRIG_STATUS. If the system is reset by a glitch detector trigger, this is recorded in POWMAN_CHIP_RESET.
    /// This register is Secure read/write only.
    pub arm: Arm,
    /// 0x0004
    pub disarm: Disarm,
    /// 0x0008
    /// Adjust the sensitivity of glitch detectors to values other than their OTP-provided defaults.
    /// This register is Secure read/write only.
    pub sensitivity: Sensitivity,
    /// 0x000c
    pub lock: Lock,
    /// 0x0010
    /// Set when a detector output triggers. Write-1-clear.
    /// (May immediately return high if the detector remains in a failed state. Detectors can only be cleared by a full reset of the switched core power domain.)
    /// This register is Secure read/write only.
    pub trig_status: TrigStatus,
    /// 0x0014
    /// Simulate the firing of one or more detectors. Writing ones to this register will set the matching bits in STATUS_TRIG.
    /// If the glitch detectors are currently armed, writing ones will also immediately reset the switched core power domain, and set the reset reason latches in POWMAN_CHIP_RESET to indicate a glitch detector resets.
    /// This register is Secure read/write only.
    pub trig_force: TrigForce,
}

