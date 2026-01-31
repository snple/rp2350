//! PWM
//! Simple PWM

#![allow(dead_code)]

pub const BASE: u32 = 0x400a8000;

/// CH0_CSR Register
///
/// Control and status register
///
/// Address offset: 0x0000
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **PH_ADV** `[7]` (WO)
///
///   Advance the phase of the counter by 1 count, while it is running.
///   Self-clearing. Write a 1, and poll until low. Counter must be running
///   at less than full speed (div_int + div_frac / 16 > 1)
///
/// - **PH_RET** `[6]` (WO)
///
///   Retard the phase of the counter by 1 count, while it is running.
///   Self-clearing. Write a 1, and poll until low. Counter must be running.
///
/// - **DIVMODE** `[5:4]` (RW)
///
/// - **B_INV** `[3]` (RW)
///
///   Invert output B
///
/// - **A_INV** `[2]` (RW)
///
///   Invert output A
///
/// - **PH_CORRECT** `[1]` (RW)
///
///   1: Enable phase-correct modulation. 0: Trailing-edge
///
/// - **EN** `[0]` (RW)
///
///   Enable the PWM channel.
#[repr(transparent)]
pub struct Ch0Csr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch0Csr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch0Csr {
    // ======== Field Constants ========

    /// **PH_ADV**
    ///
    /// Advance the phase of the counter by 1 count, while it is running.
    /// Self-clearing. Write a 1, and poll until low. Counter must be running
    /// at less than full speed (div_int + div_frac / 16 > 1)
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const PH_ADV_OFFSET: u32 = 7;
    pub const PH_ADV_MASK: u32 = 0x00000080;
    pub const PH_ADV: u32 = 1 << 7;

    /// **PH_RET**
    ///
    /// Retard the phase of the counter by 1 count, while it is running.
    /// Self-clearing. Write a 1, and poll until low. Counter must be running.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const PH_RET_OFFSET: u32 = 6;
    pub const PH_RET_MASK: u32 = 0x00000040;
    pub const PH_RET: u32 = 1 << 6;

    /// **DIVMODE**
    /// Bits `[5:4]`, mask `0x00000030`
    pub const DIVMODE_OFFSET: u32 = 4;
    pub const DIVMODE_MASK: u32 = 0x00000030;
    /// div = 0
    /// Free-running counting at rate dictated by fractional divider
    pub const DIVMODE_DIV: u32 = 0 << 4;
    pub const DIVMODE_DIV_VAL: u32 = 0;
    /// level = 1
    /// Fractional divider operation is gated by the PWM B pin.
    pub const DIVMODE_LEVEL: u32 = 1 << 4;
    pub const DIVMODE_LEVEL_VAL: u32 = 1;
    /// rise = 2
    /// Counter advances with each rising edge of the PWM B pin.
    pub const DIVMODE_RISE: u32 = 2 << 4;
    pub const DIVMODE_RISE_VAL: u32 = 2;
    /// fall = 3
    /// Counter advances with each falling edge of the PWM B pin.
    pub const DIVMODE_FALL: u32 = 3 << 4;
    pub const DIVMODE_FALL_VAL: u32 = 3;

    /// **B_INV**
    ///
    /// Invert output B
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const B_INV_OFFSET: u32 = 3;
    pub const B_INV_MASK: u32 = 0x00000008;
    pub const B_INV: u32 = 1 << 3;

    /// **A_INV**
    ///
    /// Invert output A
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const A_INV_OFFSET: u32 = 2;
    pub const A_INV_MASK: u32 = 0x00000004;
    pub const A_INV: u32 = 1 << 2;

    /// **PH_CORRECT**
    ///
    /// 1: Enable phase-correct modulation. 0: Trailing-edge
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const PH_CORRECT_OFFSET: u32 = 1;
    pub const PH_CORRECT_MASK: u32 = 0x00000002;
    pub const PH_CORRECT: u32 = 1 << 1;

    /// **EN**
    ///
    /// Enable the PWM channel.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const EN_OFFSET: u32 = 0;
    pub const EN_MASK: u32 = 0x00000001;
    pub const EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **PH_ADV** (WO)
    ///
    /// Advance the phase of the counter by 1 count, while it is running.
    /// Self-clearing. Write a 1, and poll until low. Counter must be running
    /// at less than full speed (div_int + div_frac / 16 > 1)
    #[inline(always)]
    pub fn trigger_ph_adv(&self) {
        self.value.write_bits(Self::PH_ADV_MASK);
    }

    /// Trigger **PH_RET** (WO)
    ///
    /// Retard the phase of the counter by 1 count, while it is running.
    /// Self-clearing. Write a 1, and poll until low. Counter must be running.
    #[inline(always)]
    pub fn trigger_ph_ret(&self) {
        self.value.write_bits(Self::PH_RET_MASK);
    }

    /// **DIVMODE** `[5:4]` (RW)
    #[inline(always)]
    pub fn divmode(&self) -> u32 {
        (self.value.read_bits() & Self::DIVMODE_MASK) >> Self::DIVMODE_OFFSET
    }

    /// Write **DIVMODE** `[5:4]` (RW)
    #[inline(always)]
    pub fn write_divmode(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DIVMODE_MASK) | ((value << Self::DIVMODE_OFFSET) & Self::DIVMODE_MASK));
    }

    /// **B_INV** `[3]` (RW)
    ///
    /// Invert output B
    #[inline(always)]
    pub fn b_inv(&self) -> bool {
        (self.value.read_bits() & Self::B_INV_MASK) != 0
    }

    /// Set B_INV bit atomically (RW)
    #[inline(always)]
    pub fn set_b_inv(&self) {
        self.value.set_bits(Self::B_INV_MASK);
    }

    /// Clear B_INV bit atomically (RW)
    #[inline(always)]
    pub fn clr_b_inv(&self) {
        self.value.clr_bits(Self::B_INV_MASK);
    }

    /// **A_INV** `[2]` (RW)
    ///
    /// Invert output A
    #[inline(always)]
    pub fn a_inv(&self) -> bool {
        (self.value.read_bits() & Self::A_INV_MASK) != 0
    }

    /// Set A_INV bit atomically (RW)
    #[inline(always)]
    pub fn set_a_inv(&self) {
        self.value.set_bits(Self::A_INV_MASK);
    }

    /// Clear A_INV bit atomically (RW)
    #[inline(always)]
    pub fn clr_a_inv(&self) {
        self.value.clr_bits(Self::A_INV_MASK);
    }

    /// **PH_CORRECT** `[1]` (RW)
    ///
    /// 1: Enable phase-correct modulation. 0: Trailing-edge
    #[inline(always)]
    pub fn ph_correct(&self) -> bool {
        (self.value.read_bits() & Self::PH_CORRECT_MASK) != 0
    }

    /// Set PH_CORRECT bit atomically (RW)
    #[inline(always)]
    pub fn set_ph_correct(&self) {
        self.value.set_bits(Self::PH_CORRECT_MASK);
    }

    /// Clear PH_CORRECT bit atomically (RW)
    #[inline(always)]
    pub fn clr_ph_correct(&self) {
        self.value.clr_bits(Self::PH_CORRECT_MASK);
    }

    /// **EN** `[0]` (RW)
    ///
    /// Enable the PWM channel.
    #[inline(always)]
    pub fn en(&self) -> bool {
        (self.value.read_bits() & Self::EN_MASK) != 0
    }

    /// Set EN bit atomically (RW)
    #[inline(always)]
    pub fn set_en(&self) {
        self.value.set_bits(Self::EN_MASK);
    }

    /// Clear EN bit atomically (RW)
    #[inline(always)]
    pub fn clr_en(&self) {
        self.value.clr_bits(Self::EN_MASK);
    }

}

/// CH0_CSR value type for batch read/modify/write operations
///
/// Use `ch0csr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch0csr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch0CsrVal {
    bits: u32,
}

impl Ch0CsrVal {
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

    /// Read **DIVMODE** `[5:4]`
    #[inline(always)]
    pub const fn divmode(&self) -> u32 {
        (self.bits & Ch0Csr::DIVMODE_MASK) >> Ch0Csr::DIVMODE_OFFSET
    }

    /// Read **B_INV** `[3:3]`
    ///
    /// Invert output B
    #[inline(always)]
    pub const fn b_inv(&self) -> bool {
        (self.bits & Ch0Csr::B_INV_MASK) != 0
    }

    /// Read **A_INV** `[2:2]`
    ///
    /// Invert output A
    #[inline(always)]
    pub const fn a_inv(&self) -> bool {
        (self.bits & Ch0Csr::A_INV_MASK) != 0
    }

    /// Read **PH_CORRECT** `[1:1]`
    ///
    /// 1: Enable phase-correct modulation. 0: Trailing-edge
    #[inline(always)]
    pub const fn ph_correct(&self) -> bool {
        (self.bits & Ch0Csr::PH_CORRECT_MASK) != 0
    }

    /// Read **EN** `[0:0]`
    ///
    /// Enable the PWM channel.
    #[inline(always)]
    pub const fn en(&self) -> bool {
        (self.bits & Ch0Csr::EN_MASK) != 0
    }

    /// Set **PH_ADV** `[7:7]`
    ///
    /// Advance the phase of the counter by 1 count, while it is running.
    /// Self-clearing. Write a 1, and poll until low. Counter must be running
    /// at less than full speed (div_int + div_frac / 16 > 1)
    #[inline(always)]
    pub const fn with_ph_adv(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch0Csr::PH_ADV_MASK
            } else {
                self.bits & !Ch0Csr::PH_ADV_MASK
            }
        }
    }

    /// Set **PH_RET** `[6:6]`
    ///
    /// Retard the phase of the counter by 1 count, while it is running.
    /// Self-clearing. Write a 1, and poll until low. Counter must be running.
    #[inline(always)]
    pub const fn with_ph_ret(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch0Csr::PH_RET_MASK
            } else {
                self.bits & !Ch0Csr::PH_RET_MASK
            }
        }
    }

    /// Set **DIVMODE** `[5:4]`
    #[inline(always)]
    pub const fn with_divmode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch0Csr::DIVMODE_MASK) | ((value << Ch0Csr::DIVMODE_OFFSET) & Ch0Csr::DIVMODE_MASK)
        }
    }

    /// Set **B_INV** `[3:3]`
    ///
    /// Invert output B
    #[inline(always)]
    pub const fn with_b_inv(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch0Csr::B_INV_MASK
            } else {
                self.bits & !Ch0Csr::B_INV_MASK
            }
        }
    }

    /// Set **A_INV** `[2:2]`
    ///
    /// Invert output A
    #[inline(always)]
    pub const fn with_a_inv(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch0Csr::A_INV_MASK
            } else {
                self.bits & !Ch0Csr::A_INV_MASK
            }
        }
    }

    /// Set **PH_CORRECT** `[1:1]`
    ///
    /// 1: Enable phase-correct modulation. 0: Trailing-edge
    #[inline(always)]
    pub const fn with_ph_correct(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch0Csr::PH_CORRECT_MASK
            } else {
                self.bits & !Ch0Csr::PH_CORRECT_MASK
            }
        }
    }

    /// Set **EN** `[0:0]`
    ///
    /// Enable the PWM channel.
    #[inline(always)]
    pub const fn with_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch0Csr::EN_MASK
            } else {
                self.bits & !Ch0Csr::EN_MASK
            }
        }
    }

}

impl Default for Ch0CsrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ch0Csr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch0CsrVal {
        Ch0CsrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch0CsrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch0CsrVal) -> Ch0CsrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// CH0_DIV Register
///
/// INT and FRAC form a fixed-point fractional number.
/// Counting rate is system clock frequency divided by this number.
/// Fractional division uses simple 1st-order sigma-delta.
///
/// Address offset: 0x0004
/// Reset value: 0x00000010
///
/// ## Fields
///
/// - **INT** `[11:4]` (RW)
///
/// - **FRAC** `[3:0]` (RW)
#[repr(transparent)]
pub struct Ch0Div {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch0Div {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch0Div {
    // ======== Field Constants ========

    /// **INT**
    /// Bits `[11:4]`, mask `0x00000ff0`
    pub const INT_OFFSET: u32 = 4;
    pub const INT_MASK: u32 = 0x00000ff0;

    /// **FRAC**
    /// Bits `[3:0]`, mask `0x0000000f`
    pub const FRAC_OFFSET: u32 = 0;
    pub const FRAC_MASK: u32 = 0x0000000f;

    // ======== Field Accessors ========

    /// **INT** `[11:4]` (RW)
    #[inline(always)]
    pub fn int(&self) -> u32 {
        (self.value.read_bits() & Self::INT_MASK) >> Self::INT_OFFSET
    }

    /// Write **INT** `[11:4]` (RW)
    #[inline(always)]
    pub fn write_int(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::INT_MASK) | ((value << Self::INT_OFFSET) & Self::INT_MASK));
    }

    /// **FRAC** `[3:0]` (RW)
    #[inline(always)]
    pub fn frac(&self) -> u32 {
        (self.value.read_bits() & Self::FRAC_MASK) >> Self::FRAC_OFFSET
    }

    /// Write **FRAC** `[3:0]` (RW)
    #[inline(always)]
    pub fn write_frac(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::FRAC_MASK) | ((value << Self::FRAC_OFFSET) & Self::FRAC_MASK));
    }

}

/// CH0_DIV value type for batch read/modify/write operations
///
/// Use `ch0div.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch0div.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch0DivVal {
    bits: u32,
}

impl Ch0DivVal {
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
        Self { bits: 0x00000010 }
    }

    /// Read **INT** `[11:4]`
    #[inline(always)]
    pub const fn int(&self) -> u32 {
        (self.bits & Ch0Div::INT_MASK) >> Ch0Div::INT_OFFSET
    }

    /// Read **FRAC** `[3:0]`
    #[inline(always)]
    pub const fn frac(&self) -> u32 {
        (self.bits & Ch0Div::FRAC_MASK) >> Ch0Div::FRAC_OFFSET
    }

    /// Set **INT** `[11:4]`
    #[inline(always)]
    pub const fn with_int(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch0Div::INT_MASK) | ((value << Ch0Div::INT_OFFSET) & Ch0Div::INT_MASK)
        }
    }

    /// Set **FRAC** `[3:0]`
    #[inline(always)]
    pub const fn with_frac(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch0Div::FRAC_MASK) | ((value << Ch0Div::FRAC_OFFSET) & Ch0Div::FRAC_MASK)
        }
    }

}

impl Default for Ch0DivVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ch0Div {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch0DivVal {
        Ch0DivVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch0DivVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch0DivVal) -> Ch0DivVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// CH0_CTR Register
///
/// Direct access to the PWM counter
///
/// Address offset: 0x0008
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CH0_CTR** `[15:0]` (RW)
#[repr(transparent)]
pub struct Ch0Ctr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch0Ctr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch0Ctr {
    // ======== Field Constants ========

    /// **CH0_CTR**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const CH0_CTR_OFFSET: u32 = 0;
    pub const CH0_CTR_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **CH0_CTR** `[15:0]` (RW)
    #[inline(always)]
    pub fn ch0_ctr(&self) -> u32 {
        (self.value.read_bits() & Self::CH0_CTR_MASK) >> Self::CH0_CTR_OFFSET
    }

    /// Write **CH0_CTR** `[15:0]` (RW)
    #[inline(always)]
    pub fn write_ch0_ctr(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::CH0_CTR_MASK) | ((value << Self::CH0_CTR_OFFSET) & Self::CH0_CTR_MASK));
    }

}

/// CH0_CTR value type for batch read/modify/write operations
///
/// Use `ch0ctr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch0ctr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch0CtrVal {
    bits: u32,
}

impl Ch0CtrVal {
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

    /// Read **CH0_CTR** `[15:0]`
    #[inline(always)]
    pub const fn ch0_ctr(&self) -> u32 {
        (self.bits & Ch0Ctr::CH0_CTR_MASK) >> Ch0Ctr::CH0_CTR_OFFSET
    }

    /// Set **CH0_CTR** `[15:0]`
    #[inline(always)]
    pub const fn with_ch0_ctr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch0Ctr::CH0_CTR_MASK) | ((value << Ch0Ctr::CH0_CTR_OFFSET) & Ch0Ctr::CH0_CTR_MASK)
        }
    }

}

impl Default for Ch0CtrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ch0Ctr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch0CtrVal {
        Ch0CtrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch0CtrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch0CtrVal) -> Ch0CtrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// CH0_CC Register
///
/// Counter compare values
///
/// Address offset: 0x000c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **B** `[31:16]` (RW)
///
/// - **A** `[15:0]` (RW)
#[repr(transparent)]
pub struct Ch0Cc {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch0Cc {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch0Cc {
    // ======== Field Constants ========

    /// **B**
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const B_OFFSET: u32 = 16;
    pub const B_MASK: u32 = 0xffff0000;

    /// **A**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const A_OFFSET: u32 = 0;
    pub const A_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **B** `[31:16]` (RW)
    #[inline(always)]
    pub fn b(&self) -> u32 {
        (self.value.read_bits() & Self::B_MASK) >> Self::B_OFFSET
    }

    /// Write **B** `[31:16]` (RW)
    #[inline(always)]
    pub fn write_b(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::B_MASK) | ((value << Self::B_OFFSET) & Self::B_MASK));
    }

    /// **A** `[15:0]` (RW)
    #[inline(always)]
    pub fn a(&self) -> u32 {
        (self.value.read_bits() & Self::A_MASK) >> Self::A_OFFSET
    }

    /// Write **A** `[15:0]` (RW)
    #[inline(always)]
    pub fn write_a(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::A_MASK) | ((value << Self::A_OFFSET) & Self::A_MASK));
    }

}

/// CH0_CC value type for batch read/modify/write operations
///
/// Use `ch0cc.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch0cc.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch0CcVal {
    bits: u32,
}

impl Ch0CcVal {
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

    /// Read **B** `[31:16]`
    #[inline(always)]
    pub const fn b(&self) -> u32 {
        (self.bits & Ch0Cc::B_MASK) >> Ch0Cc::B_OFFSET
    }

    /// Read **A** `[15:0]`
    #[inline(always)]
    pub const fn a(&self) -> u32 {
        (self.bits & Ch0Cc::A_MASK) >> Ch0Cc::A_OFFSET
    }

    /// Set **B** `[31:16]`
    #[inline(always)]
    pub const fn with_b(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch0Cc::B_MASK) | ((value << Ch0Cc::B_OFFSET) & Ch0Cc::B_MASK)
        }
    }

    /// Set **A** `[15:0]`
    #[inline(always)]
    pub const fn with_a(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch0Cc::A_MASK) | ((value << Ch0Cc::A_OFFSET) & Ch0Cc::A_MASK)
        }
    }

}

impl Default for Ch0CcVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ch0Cc {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch0CcVal {
        Ch0CcVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch0CcVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch0CcVal) -> Ch0CcVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// CH0_TOP Register
///
/// Counter wrap value
///
/// Address offset: 0x0010
/// Reset value: 0x0000ffff
///
/// ## Fields
///
/// - **CH0_TOP** `[15:0]` (RW)
#[repr(transparent)]
pub struct Ch0Top {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch0Top {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch0Top {
    // ======== Field Constants ========

    /// **CH0_TOP**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const CH0_TOP_OFFSET: u32 = 0;
    pub const CH0_TOP_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **CH0_TOP** `[15:0]` (RW)
    #[inline(always)]
    pub fn ch0_top(&self) -> u32 {
        (self.value.read_bits() & Self::CH0_TOP_MASK) >> Self::CH0_TOP_OFFSET
    }

    /// Write **CH0_TOP** `[15:0]` (RW)
    #[inline(always)]
    pub fn write_ch0_top(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::CH0_TOP_MASK) | ((value << Self::CH0_TOP_OFFSET) & Self::CH0_TOP_MASK));
    }

}

/// CH0_TOP value type for batch read/modify/write operations
///
/// Use `ch0top.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch0top.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch0TopVal {
    bits: u32,
}

impl Ch0TopVal {
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
        Self { bits: 0x0000ffff }
    }

    /// Read **CH0_TOP** `[15:0]`
    #[inline(always)]
    pub const fn ch0_top(&self) -> u32 {
        (self.bits & Ch0Top::CH0_TOP_MASK) >> Ch0Top::CH0_TOP_OFFSET
    }

    /// Set **CH0_TOP** `[15:0]`
    #[inline(always)]
    pub const fn with_ch0_top(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch0Top::CH0_TOP_MASK) | ((value << Ch0Top::CH0_TOP_OFFSET) & Ch0Top::CH0_TOP_MASK)
        }
    }

}

impl Default for Ch0TopVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ch0Top {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch0TopVal {
        Ch0TopVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch0TopVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch0TopVal) -> Ch0TopVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// CH1_CSR Register
///
/// Control and status register
///
/// Address offset: 0x0014
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **PH_ADV** `[7]` (WO)
///
///   Advance the phase of the counter by 1 count, while it is running.
///   Self-clearing. Write a 1, and poll until low. Counter must be running
///   at less than full speed (div_int + div_frac / 16 > 1)
///
/// - **PH_RET** `[6]` (WO)
///
///   Retard the phase of the counter by 1 count, while it is running.
///   Self-clearing. Write a 1, and poll until low. Counter must be running.
///
/// - **DIVMODE** `[5:4]` (RW)
///
/// - **B_INV** `[3]` (RW)
///
///   Invert output B
///
/// - **A_INV** `[2]` (RW)
///
///   Invert output A
///
/// - **PH_CORRECT** `[1]` (RW)
///
///   1: Enable phase-correct modulation. 0: Trailing-edge
///
/// - **EN** `[0]` (RW)
///
///   Enable the PWM channel.
#[repr(transparent)]
pub struct Ch1Csr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch1Csr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch1Csr {
    // ======== Field Constants ========

    /// **PH_ADV**
    ///
    /// Advance the phase of the counter by 1 count, while it is running.
    /// Self-clearing. Write a 1, and poll until low. Counter must be running
    /// at less than full speed (div_int + div_frac / 16 > 1)
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const PH_ADV_OFFSET: u32 = 7;
    pub const PH_ADV_MASK: u32 = 0x00000080;
    pub const PH_ADV: u32 = 1 << 7;

    /// **PH_RET**
    ///
    /// Retard the phase of the counter by 1 count, while it is running.
    /// Self-clearing. Write a 1, and poll until low. Counter must be running.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const PH_RET_OFFSET: u32 = 6;
    pub const PH_RET_MASK: u32 = 0x00000040;
    pub const PH_RET: u32 = 1 << 6;

    /// **DIVMODE**
    /// Bits `[5:4]`, mask `0x00000030`
    pub const DIVMODE_OFFSET: u32 = 4;
    pub const DIVMODE_MASK: u32 = 0x00000030;
    /// div = 0
    /// Free-running counting at rate dictated by fractional divider
    pub const DIVMODE_DIV: u32 = 0 << 4;
    pub const DIVMODE_DIV_VAL: u32 = 0;
    /// level = 1
    /// Fractional divider operation is gated by the PWM B pin.
    pub const DIVMODE_LEVEL: u32 = 1 << 4;
    pub const DIVMODE_LEVEL_VAL: u32 = 1;
    /// rise = 2
    /// Counter advances with each rising edge of the PWM B pin.
    pub const DIVMODE_RISE: u32 = 2 << 4;
    pub const DIVMODE_RISE_VAL: u32 = 2;
    /// fall = 3
    /// Counter advances with each falling edge of the PWM B pin.
    pub const DIVMODE_FALL: u32 = 3 << 4;
    pub const DIVMODE_FALL_VAL: u32 = 3;

    /// **B_INV**
    ///
    /// Invert output B
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const B_INV_OFFSET: u32 = 3;
    pub const B_INV_MASK: u32 = 0x00000008;
    pub const B_INV: u32 = 1 << 3;

    /// **A_INV**
    ///
    /// Invert output A
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const A_INV_OFFSET: u32 = 2;
    pub const A_INV_MASK: u32 = 0x00000004;
    pub const A_INV: u32 = 1 << 2;

    /// **PH_CORRECT**
    ///
    /// 1: Enable phase-correct modulation. 0: Trailing-edge
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const PH_CORRECT_OFFSET: u32 = 1;
    pub const PH_CORRECT_MASK: u32 = 0x00000002;
    pub const PH_CORRECT: u32 = 1 << 1;

    /// **EN**
    ///
    /// Enable the PWM channel.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const EN_OFFSET: u32 = 0;
    pub const EN_MASK: u32 = 0x00000001;
    pub const EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **PH_ADV** (WO)
    ///
    /// Advance the phase of the counter by 1 count, while it is running.
    /// Self-clearing. Write a 1, and poll until low. Counter must be running
    /// at less than full speed (div_int + div_frac / 16 > 1)
    #[inline(always)]
    pub fn trigger_ph_adv(&self) {
        self.value.write_bits(Self::PH_ADV_MASK);
    }

    /// Trigger **PH_RET** (WO)
    ///
    /// Retard the phase of the counter by 1 count, while it is running.
    /// Self-clearing. Write a 1, and poll until low. Counter must be running.
    #[inline(always)]
    pub fn trigger_ph_ret(&self) {
        self.value.write_bits(Self::PH_RET_MASK);
    }

    /// **DIVMODE** `[5:4]` (RW)
    #[inline(always)]
    pub fn divmode(&self) -> u32 {
        (self.value.read_bits() & Self::DIVMODE_MASK) >> Self::DIVMODE_OFFSET
    }

    /// Write **DIVMODE** `[5:4]` (RW)
    #[inline(always)]
    pub fn write_divmode(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DIVMODE_MASK) | ((value << Self::DIVMODE_OFFSET) & Self::DIVMODE_MASK));
    }

    /// **B_INV** `[3]` (RW)
    ///
    /// Invert output B
    #[inline(always)]
    pub fn b_inv(&self) -> bool {
        (self.value.read_bits() & Self::B_INV_MASK) != 0
    }

    /// Set B_INV bit atomically (RW)
    #[inline(always)]
    pub fn set_b_inv(&self) {
        self.value.set_bits(Self::B_INV_MASK);
    }

    /// Clear B_INV bit atomically (RW)
    #[inline(always)]
    pub fn clr_b_inv(&self) {
        self.value.clr_bits(Self::B_INV_MASK);
    }

    /// **A_INV** `[2]` (RW)
    ///
    /// Invert output A
    #[inline(always)]
    pub fn a_inv(&self) -> bool {
        (self.value.read_bits() & Self::A_INV_MASK) != 0
    }

    /// Set A_INV bit atomically (RW)
    #[inline(always)]
    pub fn set_a_inv(&self) {
        self.value.set_bits(Self::A_INV_MASK);
    }

    /// Clear A_INV bit atomically (RW)
    #[inline(always)]
    pub fn clr_a_inv(&self) {
        self.value.clr_bits(Self::A_INV_MASK);
    }

    /// **PH_CORRECT** `[1]` (RW)
    ///
    /// 1: Enable phase-correct modulation. 0: Trailing-edge
    #[inline(always)]
    pub fn ph_correct(&self) -> bool {
        (self.value.read_bits() & Self::PH_CORRECT_MASK) != 0
    }

    /// Set PH_CORRECT bit atomically (RW)
    #[inline(always)]
    pub fn set_ph_correct(&self) {
        self.value.set_bits(Self::PH_CORRECT_MASK);
    }

    /// Clear PH_CORRECT bit atomically (RW)
    #[inline(always)]
    pub fn clr_ph_correct(&self) {
        self.value.clr_bits(Self::PH_CORRECT_MASK);
    }

    /// **EN** `[0]` (RW)
    ///
    /// Enable the PWM channel.
    #[inline(always)]
    pub fn en(&self) -> bool {
        (self.value.read_bits() & Self::EN_MASK) != 0
    }

    /// Set EN bit atomically (RW)
    #[inline(always)]
    pub fn set_en(&self) {
        self.value.set_bits(Self::EN_MASK);
    }

    /// Clear EN bit atomically (RW)
    #[inline(always)]
    pub fn clr_en(&self) {
        self.value.clr_bits(Self::EN_MASK);
    }

}

/// CH1_CSR value type for batch read/modify/write operations
///
/// Use `ch1csr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch1csr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch1CsrVal {
    bits: u32,
}

impl Ch1CsrVal {
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

    /// Read **DIVMODE** `[5:4]`
    #[inline(always)]
    pub const fn divmode(&self) -> u32 {
        (self.bits & Ch1Csr::DIVMODE_MASK) >> Ch1Csr::DIVMODE_OFFSET
    }

    /// Read **B_INV** `[3:3]`
    ///
    /// Invert output B
    #[inline(always)]
    pub const fn b_inv(&self) -> bool {
        (self.bits & Ch1Csr::B_INV_MASK) != 0
    }

    /// Read **A_INV** `[2:2]`
    ///
    /// Invert output A
    #[inline(always)]
    pub const fn a_inv(&self) -> bool {
        (self.bits & Ch1Csr::A_INV_MASK) != 0
    }

    /// Read **PH_CORRECT** `[1:1]`
    ///
    /// 1: Enable phase-correct modulation. 0: Trailing-edge
    #[inline(always)]
    pub const fn ph_correct(&self) -> bool {
        (self.bits & Ch1Csr::PH_CORRECT_MASK) != 0
    }

    /// Read **EN** `[0:0]`
    ///
    /// Enable the PWM channel.
    #[inline(always)]
    pub const fn en(&self) -> bool {
        (self.bits & Ch1Csr::EN_MASK) != 0
    }

    /// Set **PH_ADV** `[7:7]`
    ///
    /// Advance the phase of the counter by 1 count, while it is running.
    /// Self-clearing. Write a 1, and poll until low. Counter must be running
    /// at less than full speed (div_int + div_frac / 16 > 1)
    #[inline(always)]
    pub const fn with_ph_adv(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch1Csr::PH_ADV_MASK
            } else {
                self.bits & !Ch1Csr::PH_ADV_MASK
            }
        }
    }

    /// Set **PH_RET** `[6:6]`
    ///
    /// Retard the phase of the counter by 1 count, while it is running.
    /// Self-clearing. Write a 1, and poll until low. Counter must be running.
    #[inline(always)]
    pub const fn with_ph_ret(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch1Csr::PH_RET_MASK
            } else {
                self.bits & !Ch1Csr::PH_RET_MASK
            }
        }
    }

    /// Set **DIVMODE** `[5:4]`
    #[inline(always)]
    pub const fn with_divmode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch1Csr::DIVMODE_MASK) | ((value << Ch1Csr::DIVMODE_OFFSET) & Ch1Csr::DIVMODE_MASK)
        }
    }

    /// Set **B_INV** `[3:3]`
    ///
    /// Invert output B
    #[inline(always)]
    pub const fn with_b_inv(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch1Csr::B_INV_MASK
            } else {
                self.bits & !Ch1Csr::B_INV_MASK
            }
        }
    }

    /// Set **A_INV** `[2:2]`
    ///
    /// Invert output A
    #[inline(always)]
    pub const fn with_a_inv(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch1Csr::A_INV_MASK
            } else {
                self.bits & !Ch1Csr::A_INV_MASK
            }
        }
    }

    /// Set **PH_CORRECT** `[1:1]`
    ///
    /// 1: Enable phase-correct modulation. 0: Trailing-edge
    #[inline(always)]
    pub const fn with_ph_correct(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch1Csr::PH_CORRECT_MASK
            } else {
                self.bits & !Ch1Csr::PH_CORRECT_MASK
            }
        }
    }

    /// Set **EN** `[0:0]`
    ///
    /// Enable the PWM channel.
    #[inline(always)]
    pub const fn with_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch1Csr::EN_MASK
            } else {
                self.bits & !Ch1Csr::EN_MASK
            }
        }
    }

}

impl Default for Ch1CsrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ch1Csr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch1CsrVal {
        Ch1CsrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch1CsrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch1CsrVal) -> Ch1CsrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// CH1_DIV Register
///
/// INT and FRAC form a fixed-point fractional number.
/// Counting rate is system clock frequency divided by this number.
/// Fractional division uses simple 1st-order sigma-delta.
///
/// Address offset: 0x0018
/// Reset value: 0x00000010
///
/// ## Fields
///
/// - **INT** `[11:4]` (RW)
///
/// - **FRAC** `[3:0]` (RW)
#[repr(transparent)]
pub struct Ch1Div {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch1Div {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch1Div {
    // ======== Field Constants ========

    /// **INT**
    /// Bits `[11:4]`, mask `0x00000ff0`
    pub const INT_OFFSET: u32 = 4;
    pub const INT_MASK: u32 = 0x00000ff0;

    /// **FRAC**
    /// Bits `[3:0]`, mask `0x0000000f`
    pub const FRAC_OFFSET: u32 = 0;
    pub const FRAC_MASK: u32 = 0x0000000f;

    // ======== Field Accessors ========

    /// **INT** `[11:4]` (RW)
    #[inline(always)]
    pub fn int(&self) -> u32 {
        (self.value.read_bits() & Self::INT_MASK) >> Self::INT_OFFSET
    }

    /// Write **INT** `[11:4]` (RW)
    #[inline(always)]
    pub fn write_int(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::INT_MASK) | ((value << Self::INT_OFFSET) & Self::INT_MASK));
    }

    /// **FRAC** `[3:0]` (RW)
    #[inline(always)]
    pub fn frac(&self) -> u32 {
        (self.value.read_bits() & Self::FRAC_MASK) >> Self::FRAC_OFFSET
    }

    /// Write **FRAC** `[3:0]` (RW)
    #[inline(always)]
    pub fn write_frac(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::FRAC_MASK) | ((value << Self::FRAC_OFFSET) & Self::FRAC_MASK));
    }

}

/// CH1_DIV value type for batch read/modify/write operations
///
/// Use `ch1div.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch1div.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch1DivVal {
    bits: u32,
}

impl Ch1DivVal {
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
        Self { bits: 0x00000010 }
    }

    /// Read **INT** `[11:4]`
    #[inline(always)]
    pub const fn int(&self) -> u32 {
        (self.bits & Ch1Div::INT_MASK) >> Ch1Div::INT_OFFSET
    }

    /// Read **FRAC** `[3:0]`
    #[inline(always)]
    pub const fn frac(&self) -> u32 {
        (self.bits & Ch1Div::FRAC_MASK) >> Ch1Div::FRAC_OFFSET
    }

    /// Set **INT** `[11:4]`
    #[inline(always)]
    pub const fn with_int(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch1Div::INT_MASK) | ((value << Ch1Div::INT_OFFSET) & Ch1Div::INT_MASK)
        }
    }

    /// Set **FRAC** `[3:0]`
    #[inline(always)]
    pub const fn with_frac(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch1Div::FRAC_MASK) | ((value << Ch1Div::FRAC_OFFSET) & Ch1Div::FRAC_MASK)
        }
    }

}

impl Default for Ch1DivVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ch1Div {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch1DivVal {
        Ch1DivVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch1DivVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch1DivVal) -> Ch1DivVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// CH1_CTR Register
///
/// Direct access to the PWM counter
///
/// Address offset: 0x001c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CH1_CTR** `[15:0]` (RW)
#[repr(transparent)]
pub struct Ch1Ctr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch1Ctr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch1Ctr {
    // ======== Field Constants ========

    /// **CH1_CTR**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const CH1_CTR_OFFSET: u32 = 0;
    pub const CH1_CTR_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **CH1_CTR** `[15:0]` (RW)
    #[inline(always)]
    pub fn ch1_ctr(&self) -> u32 {
        (self.value.read_bits() & Self::CH1_CTR_MASK) >> Self::CH1_CTR_OFFSET
    }

    /// Write **CH1_CTR** `[15:0]` (RW)
    #[inline(always)]
    pub fn write_ch1_ctr(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::CH1_CTR_MASK) | ((value << Self::CH1_CTR_OFFSET) & Self::CH1_CTR_MASK));
    }

}

/// CH1_CTR value type for batch read/modify/write operations
///
/// Use `ch1ctr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch1ctr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch1CtrVal {
    bits: u32,
}

impl Ch1CtrVal {
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

    /// Read **CH1_CTR** `[15:0]`
    #[inline(always)]
    pub const fn ch1_ctr(&self) -> u32 {
        (self.bits & Ch1Ctr::CH1_CTR_MASK) >> Ch1Ctr::CH1_CTR_OFFSET
    }

    /// Set **CH1_CTR** `[15:0]`
    #[inline(always)]
    pub const fn with_ch1_ctr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch1Ctr::CH1_CTR_MASK) | ((value << Ch1Ctr::CH1_CTR_OFFSET) & Ch1Ctr::CH1_CTR_MASK)
        }
    }

}

impl Default for Ch1CtrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ch1Ctr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch1CtrVal {
        Ch1CtrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch1CtrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch1CtrVal) -> Ch1CtrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// CH1_CC Register
///
/// Counter compare values
///
/// Address offset: 0x0020
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **B** `[31:16]` (RW)
///
/// - **A** `[15:0]` (RW)
#[repr(transparent)]
pub struct Ch1Cc {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch1Cc {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch1Cc {
    // ======== Field Constants ========

    /// **B**
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const B_OFFSET: u32 = 16;
    pub const B_MASK: u32 = 0xffff0000;

    /// **A**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const A_OFFSET: u32 = 0;
    pub const A_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **B** `[31:16]` (RW)
    #[inline(always)]
    pub fn b(&self) -> u32 {
        (self.value.read_bits() & Self::B_MASK) >> Self::B_OFFSET
    }

    /// Write **B** `[31:16]` (RW)
    #[inline(always)]
    pub fn write_b(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::B_MASK) | ((value << Self::B_OFFSET) & Self::B_MASK));
    }

    /// **A** `[15:0]` (RW)
    #[inline(always)]
    pub fn a(&self) -> u32 {
        (self.value.read_bits() & Self::A_MASK) >> Self::A_OFFSET
    }

    /// Write **A** `[15:0]` (RW)
    #[inline(always)]
    pub fn write_a(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::A_MASK) | ((value << Self::A_OFFSET) & Self::A_MASK));
    }

}

/// CH1_CC value type for batch read/modify/write operations
///
/// Use `ch1cc.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch1cc.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch1CcVal {
    bits: u32,
}

impl Ch1CcVal {
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

    /// Read **B** `[31:16]`
    #[inline(always)]
    pub const fn b(&self) -> u32 {
        (self.bits & Ch1Cc::B_MASK) >> Ch1Cc::B_OFFSET
    }

    /// Read **A** `[15:0]`
    #[inline(always)]
    pub const fn a(&self) -> u32 {
        (self.bits & Ch1Cc::A_MASK) >> Ch1Cc::A_OFFSET
    }

    /// Set **B** `[31:16]`
    #[inline(always)]
    pub const fn with_b(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch1Cc::B_MASK) | ((value << Ch1Cc::B_OFFSET) & Ch1Cc::B_MASK)
        }
    }

    /// Set **A** `[15:0]`
    #[inline(always)]
    pub const fn with_a(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch1Cc::A_MASK) | ((value << Ch1Cc::A_OFFSET) & Ch1Cc::A_MASK)
        }
    }

}

impl Default for Ch1CcVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ch1Cc {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch1CcVal {
        Ch1CcVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch1CcVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch1CcVal) -> Ch1CcVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// CH1_TOP Register
///
/// Counter wrap value
///
/// Address offset: 0x0024
/// Reset value: 0x0000ffff
///
/// ## Fields
///
/// - **CH1_TOP** `[15:0]` (RW)
#[repr(transparent)]
pub struct Ch1Top {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch1Top {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch1Top {
    // ======== Field Constants ========

    /// **CH1_TOP**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const CH1_TOP_OFFSET: u32 = 0;
    pub const CH1_TOP_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **CH1_TOP** `[15:0]` (RW)
    #[inline(always)]
    pub fn ch1_top(&self) -> u32 {
        (self.value.read_bits() & Self::CH1_TOP_MASK) >> Self::CH1_TOP_OFFSET
    }

    /// Write **CH1_TOP** `[15:0]` (RW)
    #[inline(always)]
    pub fn write_ch1_top(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::CH1_TOP_MASK) | ((value << Self::CH1_TOP_OFFSET) & Self::CH1_TOP_MASK));
    }

}

/// CH1_TOP value type for batch read/modify/write operations
///
/// Use `ch1top.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch1top.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch1TopVal {
    bits: u32,
}

impl Ch1TopVal {
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
        Self { bits: 0x0000ffff }
    }

    /// Read **CH1_TOP** `[15:0]`
    #[inline(always)]
    pub const fn ch1_top(&self) -> u32 {
        (self.bits & Ch1Top::CH1_TOP_MASK) >> Ch1Top::CH1_TOP_OFFSET
    }

    /// Set **CH1_TOP** `[15:0]`
    #[inline(always)]
    pub const fn with_ch1_top(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch1Top::CH1_TOP_MASK) | ((value << Ch1Top::CH1_TOP_OFFSET) & Ch1Top::CH1_TOP_MASK)
        }
    }

}

impl Default for Ch1TopVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ch1Top {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch1TopVal {
        Ch1TopVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch1TopVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch1TopVal) -> Ch1TopVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// CH2_CSR Register
///
/// Control and status register
///
/// Address offset: 0x0028
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **PH_ADV** `[7]` (WO)
///
///   Advance the phase of the counter by 1 count, while it is running.
///   Self-clearing. Write a 1, and poll until low. Counter must be running
///   at less than full speed (div_int + div_frac / 16 > 1)
///
/// - **PH_RET** `[6]` (WO)
///
///   Retard the phase of the counter by 1 count, while it is running.
///   Self-clearing. Write a 1, and poll until low. Counter must be running.
///
/// - **DIVMODE** `[5:4]` (RW)
///
/// - **B_INV** `[3]` (RW)
///
///   Invert output B
///
/// - **A_INV** `[2]` (RW)
///
///   Invert output A
///
/// - **PH_CORRECT** `[1]` (RW)
///
///   1: Enable phase-correct modulation. 0: Trailing-edge
///
/// - **EN** `[0]` (RW)
///
///   Enable the PWM channel.
#[repr(transparent)]
pub struct Ch2Csr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch2Csr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch2Csr {
    // ======== Field Constants ========

    /// **PH_ADV**
    ///
    /// Advance the phase of the counter by 1 count, while it is running.
    /// Self-clearing. Write a 1, and poll until low. Counter must be running
    /// at less than full speed (div_int + div_frac / 16 > 1)
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const PH_ADV_OFFSET: u32 = 7;
    pub const PH_ADV_MASK: u32 = 0x00000080;
    pub const PH_ADV: u32 = 1 << 7;

    /// **PH_RET**
    ///
    /// Retard the phase of the counter by 1 count, while it is running.
    /// Self-clearing. Write a 1, and poll until low. Counter must be running.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const PH_RET_OFFSET: u32 = 6;
    pub const PH_RET_MASK: u32 = 0x00000040;
    pub const PH_RET: u32 = 1 << 6;

    /// **DIVMODE**
    /// Bits `[5:4]`, mask `0x00000030`
    pub const DIVMODE_OFFSET: u32 = 4;
    pub const DIVMODE_MASK: u32 = 0x00000030;
    /// div = 0
    /// Free-running counting at rate dictated by fractional divider
    pub const DIVMODE_DIV: u32 = 0 << 4;
    pub const DIVMODE_DIV_VAL: u32 = 0;
    /// level = 1
    /// Fractional divider operation is gated by the PWM B pin.
    pub const DIVMODE_LEVEL: u32 = 1 << 4;
    pub const DIVMODE_LEVEL_VAL: u32 = 1;
    /// rise = 2
    /// Counter advances with each rising edge of the PWM B pin.
    pub const DIVMODE_RISE: u32 = 2 << 4;
    pub const DIVMODE_RISE_VAL: u32 = 2;
    /// fall = 3
    /// Counter advances with each falling edge of the PWM B pin.
    pub const DIVMODE_FALL: u32 = 3 << 4;
    pub const DIVMODE_FALL_VAL: u32 = 3;

    /// **B_INV**
    ///
    /// Invert output B
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const B_INV_OFFSET: u32 = 3;
    pub const B_INV_MASK: u32 = 0x00000008;
    pub const B_INV: u32 = 1 << 3;

    /// **A_INV**
    ///
    /// Invert output A
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const A_INV_OFFSET: u32 = 2;
    pub const A_INV_MASK: u32 = 0x00000004;
    pub const A_INV: u32 = 1 << 2;

    /// **PH_CORRECT**
    ///
    /// 1: Enable phase-correct modulation. 0: Trailing-edge
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const PH_CORRECT_OFFSET: u32 = 1;
    pub const PH_CORRECT_MASK: u32 = 0x00000002;
    pub const PH_CORRECT: u32 = 1 << 1;

    /// **EN**
    ///
    /// Enable the PWM channel.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const EN_OFFSET: u32 = 0;
    pub const EN_MASK: u32 = 0x00000001;
    pub const EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **PH_ADV** (WO)
    ///
    /// Advance the phase of the counter by 1 count, while it is running.
    /// Self-clearing. Write a 1, and poll until low. Counter must be running
    /// at less than full speed (div_int + div_frac / 16 > 1)
    #[inline(always)]
    pub fn trigger_ph_adv(&self) {
        self.value.write_bits(Self::PH_ADV_MASK);
    }

    /// Trigger **PH_RET** (WO)
    ///
    /// Retard the phase of the counter by 1 count, while it is running.
    /// Self-clearing. Write a 1, and poll until low. Counter must be running.
    #[inline(always)]
    pub fn trigger_ph_ret(&self) {
        self.value.write_bits(Self::PH_RET_MASK);
    }

    /// **DIVMODE** `[5:4]` (RW)
    #[inline(always)]
    pub fn divmode(&self) -> u32 {
        (self.value.read_bits() & Self::DIVMODE_MASK) >> Self::DIVMODE_OFFSET
    }

    /// Write **DIVMODE** `[5:4]` (RW)
    #[inline(always)]
    pub fn write_divmode(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DIVMODE_MASK) | ((value << Self::DIVMODE_OFFSET) & Self::DIVMODE_MASK));
    }

    /// **B_INV** `[3]` (RW)
    ///
    /// Invert output B
    #[inline(always)]
    pub fn b_inv(&self) -> bool {
        (self.value.read_bits() & Self::B_INV_MASK) != 0
    }

    /// Set B_INV bit atomically (RW)
    #[inline(always)]
    pub fn set_b_inv(&self) {
        self.value.set_bits(Self::B_INV_MASK);
    }

    /// Clear B_INV bit atomically (RW)
    #[inline(always)]
    pub fn clr_b_inv(&self) {
        self.value.clr_bits(Self::B_INV_MASK);
    }

    /// **A_INV** `[2]` (RW)
    ///
    /// Invert output A
    #[inline(always)]
    pub fn a_inv(&self) -> bool {
        (self.value.read_bits() & Self::A_INV_MASK) != 0
    }

    /// Set A_INV bit atomically (RW)
    #[inline(always)]
    pub fn set_a_inv(&self) {
        self.value.set_bits(Self::A_INV_MASK);
    }

    /// Clear A_INV bit atomically (RW)
    #[inline(always)]
    pub fn clr_a_inv(&self) {
        self.value.clr_bits(Self::A_INV_MASK);
    }

    /// **PH_CORRECT** `[1]` (RW)
    ///
    /// 1: Enable phase-correct modulation. 0: Trailing-edge
    #[inline(always)]
    pub fn ph_correct(&self) -> bool {
        (self.value.read_bits() & Self::PH_CORRECT_MASK) != 0
    }

    /// Set PH_CORRECT bit atomically (RW)
    #[inline(always)]
    pub fn set_ph_correct(&self) {
        self.value.set_bits(Self::PH_CORRECT_MASK);
    }

    /// Clear PH_CORRECT bit atomically (RW)
    #[inline(always)]
    pub fn clr_ph_correct(&self) {
        self.value.clr_bits(Self::PH_CORRECT_MASK);
    }

    /// **EN** `[0]` (RW)
    ///
    /// Enable the PWM channel.
    #[inline(always)]
    pub fn en(&self) -> bool {
        (self.value.read_bits() & Self::EN_MASK) != 0
    }

    /// Set EN bit atomically (RW)
    #[inline(always)]
    pub fn set_en(&self) {
        self.value.set_bits(Self::EN_MASK);
    }

    /// Clear EN bit atomically (RW)
    #[inline(always)]
    pub fn clr_en(&self) {
        self.value.clr_bits(Self::EN_MASK);
    }

}

/// CH2_CSR value type for batch read/modify/write operations
///
/// Use `ch2csr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch2csr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch2CsrVal {
    bits: u32,
}

impl Ch2CsrVal {
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

    /// Read **DIVMODE** `[5:4]`
    #[inline(always)]
    pub const fn divmode(&self) -> u32 {
        (self.bits & Ch2Csr::DIVMODE_MASK) >> Ch2Csr::DIVMODE_OFFSET
    }

    /// Read **B_INV** `[3:3]`
    ///
    /// Invert output B
    #[inline(always)]
    pub const fn b_inv(&self) -> bool {
        (self.bits & Ch2Csr::B_INV_MASK) != 0
    }

    /// Read **A_INV** `[2:2]`
    ///
    /// Invert output A
    #[inline(always)]
    pub const fn a_inv(&self) -> bool {
        (self.bits & Ch2Csr::A_INV_MASK) != 0
    }

    /// Read **PH_CORRECT** `[1:1]`
    ///
    /// 1: Enable phase-correct modulation. 0: Trailing-edge
    #[inline(always)]
    pub const fn ph_correct(&self) -> bool {
        (self.bits & Ch2Csr::PH_CORRECT_MASK) != 0
    }

    /// Read **EN** `[0:0]`
    ///
    /// Enable the PWM channel.
    #[inline(always)]
    pub const fn en(&self) -> bool {
        (self.bits & Ch2Csr::EN_MASK) != 0
    }

    /// Set **PH_ADV** `[7:7]`
    ///
    /// Advance the phase of the counter by 1 count, while it is running.
    /// Self-clearing. Write a 1, and poll until low. Counter must be running
    /// at less than full speed (div_int + div_frac / 16 > 1)
    #[inline(always)]
    pub const fn with_ph_adv(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch2Csr::PH_ADV_MASK
            } else {
                self.bits & !Ch2Csr::PH_ADV_MASK
            }
        }
    }

    /// Set **PH_RET** `[6:6]`
    ///
    /// Retard the phase of the counter by 1 count, while it is running.
    /// Self-clearing. Write a 1, and poll until low. Counter must be running.
    #[inline(always)]
    pub const fn with_ph_ret(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch2Csr::PH_RET_MASK
            } else {
                self.bits & !Ch2Csr::PH_RET_MASK
            }
        }
    }

    /// Set **DIVMODE** `[5:4]`
    #[inline(always)]
    pub const fn with_divmode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch2Csr::DIVMODE_MASK) | ((value << Ch2Csr::DIVMODE_OFFSET) & Ch2Csr::DIVMODE_MASK)
        }
    }

    /// Set **B_INV** `[3:3]`
    ///
    /// Invert output B
    #[inline(always)]
    pub const fn with_b_inv(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch2Csr::B_INV_MASK
            } else {
                self.bits & !Ch2Csr::B_INV_MASK
            }
        }
    }

    /// Set **A_INV** `[2:2]`
    ///
    /// Invert output A
    #[inline(always)]
    pub const fn with_a_inv(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch2Csr::A_INV_MASK
            } else {
                self.bits & !Ch2Csr::A_INV_MASK
            }
        }
    }

    /// Set **PH_CORRECT** `[1:1]`
    ///
    /// 1: Enable phase-correct modulation. 0: Trailing-edge
    #[inline(always)]
    pub const fn with_ph_correct(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch2Csr::PH_CORRECT_MASK
            } else {
                self.bits & !Ch2Csr::PH_CORRECT_MASK
            }
        }
    }

    /// Set **EN** `[0:0]`
    ///
    /// Enable the PWM channel.
    #[inline(always)]
    pub const fn with_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch2Csr::EN_MASK
            } else {
                self.bits & !Ch2Csr::EN_MASK
            }
        }
    }

}

impl Default for Ch2CsrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ch2Csr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch2CsrVal {
        Ch2CsrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch2CsrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch2CsrVal) -> Ch2CsrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// CH2_DIV Register
///
/// INT and FRAC form a fixed-point fractional number.
/// Counting rate is system clock frequency divided by this number.
/// Fractional division uses simple 1st-order sigma-delta.
///
/// Address offset: 0x002c
/// Reset value: 0x00000010
///
/// ## Fields
///
/// - **INT** `[11:4]` (RW)
///
/// - **FRAC** `[3:0]` (RW)
#[repr(transparent)]
pub struct Ch2Div {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch2Div {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch2Div {
    // ======== Field Constants ========

    /// **INT**
    /// Bits `[11:4]`, mask `0x00000ff0`
    pub const INT_OFFSET: u32 = 4;
    pub const INT_MASK: u32 = 0x00000ff0;

    /// **FRAC**
    /// Bits `[3:0]`, mask `0x0000000f`
    pub const FRAC_OFFSET: u32 = 0;
    pub const FRAC_MASK: u32 = 0x0000000f;

    // ======== Field Accessors ========

    /// **INT** `[11:4]` (RW)
    #[inline(always)]
    pub fn int(&self) -> u32 {
        (self.value.read_bits() & Self::INT_MASK) >> Self::INT_OFFSET
    }

    /// Write **INT** `[11:4]` (RW)
    #[inline(always)]
    pub fn write_int(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::INT_MASK) | ((value << Self::INT_OFFSET) & Self::INT_MASK));
    }

    /// **FRAC** `[3:0]` (RW)
    #[inline(always)]
    pub fn frac(&self) -> u32 {
        (self.value.read_bits() & Self::FRAC_MASK) >> Self::FRAC_OFFSET
    }

    /// Write **FRAC** `[3:0]` (RW)
    #[inline(always)]
    pub fn write_frac(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::FRAC_MASK) | ((value << Self::FRAC_OFFSET) & Self::FRAC_MASK));
    }

}

/// CH2_DIV value type for batch read/modify/write operations
///
/// Use `ch2div.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch2div.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch2DivVal {
    bits: u32,
}

impl Ch2DivVal {
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
        Self { bits: 0x00000010 }
    }

    /// Read **INT** `[11:4]`
    #[inline(always)]
    pub const fn int(&self) -> u32 {
        (self.bits & Ch2Div::INT_MASK) >> Ch2Div::INT_OFFSET
    }

    /// Read **FRAC** `[3:0]`
    #[inline(always)]
    pub const fn frac(&self) -> u32 {
        (self.bits & Ch2Div::FRAC_MASK) >> Ch2Div::FRAC_OFFSET
    }

    /// Set **INT** `[11:4]`
    #[inline(always)]
    pub const fn with_int(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch2Div::INT_MASK) | ((value << Ch2Div::INT_OFFSET) & Ch2Div::INT_MASK)
        }
    }

    /// Set **FRAC** `[3:0]`
    #[inline(always)]
    pub const fn with_frac(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch2Div::FRAC_MASK) | ((value << Ch2Div::FRAC_OFFSET) & Ch2Div::FRAC_MASK)
        }
    }

}

impl Default for Ch2DivVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ch2Div {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch2DivVal {
        Ch2DivVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch2DivVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch2DivVal) -> Ch2DivVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// CH2_CTR Register
///
/// Direct access to the PWM counter
///
/// Address offset: 0x0030
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CH2_CTR** `[15:0]` (RW)
#[repr(transparent)]
pub struct Ch2Ctr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch2Ctr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch2Ctr {
    // ======== Field Constants ========

    /// **CH2_CTR**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const CH2_CTR_OFFSET: u32 = 0;
    pub const CH2_CTR_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **CH2_CTR** `[15:0]` (RW)
    #[inline(always)]
    pub fn ch2_ctr(&self) -> u32 {
        (self.value.read_bits() & Self::CH2_CTR_MASK) >> Self::CH2_CTR_OFFSET
    }

    /// Write **CH2_CTR** `[15:0]` (RW)
    #[inline(always)]
    pub fn write_ch2_ctr(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::CH2_CTR_MASK) | ((value << Self::CH2_CTR_OFFSET) & Self::CH2_CTR_MASK));
    }

}

/// CH2_CTR value type for batch read/modify/write operations
///
/// Use `ch2ctr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch2ctr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch2CtrVal {
    bits: u32,
}

impl Ch2CtrVal {
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

    /// Read **CH2_CTR** `[15:0]`
    #[inline(always)]
    pub const fn ch2_ctr(&self) -> u32 {
        (self.bits & Ch2Ctr::CH2_CTR_MASK) >> Ch2Ctr::CH2_CTR_OFFSET
    }

    /// Set **CH2_CTR** `[15:0]`
    #[inline(always)]
    pub const fn with_ch2_ctr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch2Ctr::CH2_CTR_MASK) | ((value << Ch2Ctr::CH2_CTR_OFFSET) & Ch2Ctr::CH2_CTR_MASK)
        }
    }

}

impl Default for Ch2CtrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ch2Ctr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch2CtrVal {
        Ch2CtrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch2CtrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch2CtrVal) -> Ch2CtrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// CH2_CC Register
///
/// Counter compare values
///
/// Address offset: 0x0034
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **B** `[31:16]` (RW)
///
/// - **A** `[15:0]` (RW)
#[repr(transparent)]
pub struct Ch2Cc {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch2Cc {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch2Cc {
    // ======== Field Constants ========

    /// **B**
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const B_OFFSET: u32 = 16;
    pub const B_MASK: u32 = 0xffff0000;

    /// **A**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const A_OFFSET: u32 = 0;
    pub const A_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **B** `[31:16]` (RW)
    #[inline(always)]
    pub fn b(&self) -> u32 {
        (self.value.read_bits() & Self::B_MASK) >> Self::B_OFFSET
    }

    /// Write **B** `[31:16]` (RW)
    #[inline(always)]
    pub fn write_b(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::B_MASK) | ((value << Self::B_OFFSET) & Self::B_MASK));
    }

    /// **A** `[15:0]` (RW)
    #[inline(always)]
    pub fn a(&self) -> u32 {
        (self.value.read_bits() & Self::A_MASK) >> Self::A_OFFSET
    }

    /// Write **A** `[15:0]` (RW)
    #[inline(always)]
    pub fn write_a(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::A_MASK) | ((value << Self::A_OFFSET) & Self::A_MASK));
    }

}

/// CH2_CC value type for batch read/modify/write operations
///
/// Use `ch2cc.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch2cc.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch2CcVal {
    bits: u32,
}

impl Ch2CcVal {
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

    /// Read **B** `[31:16]`
    #[inline(always)]
    pub const fn b(&self) -> u32 {
        (self.bits & Ch2Cc::B_MASK) >> Ch2Cc::B_OFFSET
    }

    /// Read **A** `[15:0]`
    #[inline(always)]
    pub const fn a(&self) -> u32 {
        (self.bits & Ch2Cc::A_MASK) >> Ch2Cc::A_OFFSET
    }

    /// Set **B** `[31:16]`
    #[inline(always)]
    pub const fn with_b(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch2Cc::B_MASK) | ((value << Ch2Cc::B_OFFSET) & Ch2Cc::B_MASK)
        }
    }

    /// Set **A** `[15:0]`
    #[inline(always)]
    pub const fn with_a(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch2Cc::A_MASK) | ((value << Ch2Cc::A_OFFSET) & Ch2Cc::A_MASK)
        }
    }

}

impl Default for Ch2CcVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ch2Cc {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch2CcVal {
        Ch2CcVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch2CcVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch2CcVal) -> Ch2CcVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// CH2_TOP Register
///
/// Counter wrap value
///
/// Address offset: 0x0038
/// Reset value: 0x0000ffff
///
/// ## Fields
///
/// - **CH2_TOP** `[15:0]` (RW)
#[repr(transparent)]
pub struct Ch2Top {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch2Top {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch2Top {
    // ======== Field Constants ========

    /// **CH2_TOP**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const CH2_TOP_OFFSET: u32 = 0;
    pub const CH2_TOP_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **CH2_TOP** `[15:0]` (RW)
    #[inline(always)]
    pub fn ch2_top(&self) -> u32 {
        (self.value.read_bits() & Self::CH2_TOP_MASK) >> Self::CH2_TOP_OFFSET
    }

    /// Write **CH2_TOP** `[15:0]` (RW)
    #[inline(always)]
    pub fn write_ch2_top(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::CH2_TOP_MASK) | ((value << Self::CH2_TOP_OFFSET) & Self::CH2_TOP_MASK));
    }

}

/// CH2_TOP value type for batch read/modify/write operations
///
/// Use `ch2top.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch2top.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch2TopVal {
    bits: u32,
}

impl Ch2TopVal {
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
        Self { bits: 0x0000ffff }
    }

    /// Read **CH2_TOP** `[15:0]`
    #[inline(always)]
    pub const fn ch2_top(&self) -> u32 {
        (self.bits & Ch2Top::CH2_TOP_MASK) >> Ch2Top::CH2_TOP_OFFSET
    }

    /// Set **CH2_TOP** `[15:0]`
    #[inline(always)]
    pub const fn with_ch2_top(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch2Top::CH2_TOP_MASK) | ((value << Ch2Top::CH2_TOP_OFFSET) & Ch2Top::CH2_TOP_MASK)
        }
    }

}

impl Default for Ch2TopVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ch2Top {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch2TopVal {
        Ch2TopVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch2TopVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch2TopVal) -> Ch2TopVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// CH3_CSR Register
///
/// Control and status register
///
/// Address offset: 0x003c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **PH_ADV** `[7]` (WO)
///
///   Advance the phase of the counter by 1 count, while it is running.
///   Self-clearing. Write a 1, and poll until low. Counter must be running
///   at less than full speed (div_int + div_frac / 16 > 1)
///
/// - **PH_RET** `[6]` (WO)
///
///   Retard the phase of the counter by 1 count, while it is running.
///   Self-clearing. Write a 1, and poll until low. Counter must be running.
///
/// - **DIVMODE** `[5:4]` (RW)
///
/// - **B_INV** `[3]` (RW)
///
///   Invert output B
///
/// - **A_INV** `[2]` (RW)
///
///   Invert output A
///
/// - **PH_CORRECT** `[1]` (RW)
///
///   1: Enable phase-correct modulation. 0: Trailing-edge
///
/// - **EN** `[0]` (RW)
///
///   Enable the PWM channel.
#[repr(transparent)]
pub struct Ch3Csr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch3Csr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch3Csr {
    // ======== Field Constants ========

    /// **PH_ADV**
    ///
    /// Advance the phase of the counter by 1 count, while it is running.
    /// Self-clearing. Write a 1, and poll until low. Counter must be running
    /// at less than full speed (div_int + div_frac / 16 > 1)
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const PH_ADV_OFFSET: u32 = 7;
    pub const PH_ADV_MASK: u32 = 0x00000080;
    pub const PH_ADV: u32 = 1 << 7;

    /// **PH_RET**
    ///
    /// Retard the phase of the counter by 1 count, while it is running.
    /// Self-clearing. Write a 1, and poll until low. Counter must be running.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const PH_RET_OFFSET: u32 = 6;
    pub const PH_RET_MASK: u32 = 0x00000040;
    pub const PH_RET: u32 = 1 << 6;

    /// **DIVMODE**
    /// Bits `[5:4]`, mask `0x00000030`
    pub const DIVMODE_OFFSET: u32 = 4;
    pub const DIVMODE_MASK: u32 = 0x00000030;
    /// div = 0
    /// Free-running counting at rate dictated by fractional divider
    pub const DIVMODE_DIV: u32 = 0 << 4;
    pub const DIVMODE_DIV_VAL: u32 = 0;
    /// level = 1
    /// Fractional divider operation is gated by the PWM B pin.
    pub const DIVMODE_LEVEL: u32 = 1 << 4;
    pub const DIVMODE_LEVEL_VAL: u32 = 1;
    /// rise = 2
    /// Counter advances with each rising edge of the PWM B pin.
    pub const DIVMODE_RISE: u32 = 2 << 4;
    pub const DIVMODE_RISE_VAL: u32 = 2;
    /// fall = 3
    /// Counter advances with each falling edge of the PWM B pin.
    pub const DIVMODE_FALL: u32 = 3 << 4;
    pub const DIVMODE_FALL_VAL: u32 = 3;

    /// **B_INV**
    ///
    /// Invert output B
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const B_INV_OFFSET: u32 = 3;
    pub const B_INV_MASK: u32 = 0x00000008;
    pub const B_INV: u32 = 1 << 3;

    /// **A_INV**
    ///
    /// Invert output A
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const A_INV_OFFSET: u32 = 2;
    pub const A_INV_MASK: u32 = 0x00000004;
    pub const A_INV: u32 = 1 << 2;

    /// **PH_CORRECT**
    ///
    /// 1: Enable phase-correct modulation. 0: Trailing-edge
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const PH_CORRECT_OFFSET: u32 = 1;
    pub const PH_CORRECT_MASK: u32 = 0x00000002;
    pub const PH_CORRECT: u32 = 1 << 1;

    /// **EN**
    ///
    /// Enable the PWM channel.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const EN_OFFSET: u32 = 0;
    pub const EN_MASK: u32 = 0x00000001;
    pub const EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **PH_ADV** (WO)
    ///
    /// Advance the phase of the counter by 1 count, while it is running.
    /// Self-clearing. Write a 1, and poll until low. Counter must be running
    /// at less than full speed (div_int + div_frac / 16 > 1)
    #[inline(always)]
    pub fn trigger_ph_adv(&self) {
        self.value.write_bits(Self::PH_ADV_MASK);
    }

    /// Trigger **PH_RET** (WO)
    ///
    /// Retard the phase of the counter by 1 count, while it is running.
    /// Self-clearing. Write a 1, and poll until low. Counter must be running.
    #[inline(always)]
    pub fn trigger_ph_ret(&self) {
        self.value.write_bits(Self::PH_RET_MASK);
    }

    /// **DIVMODE** `[5:4]` (RW)
    #[inline(always)]
    pub fn divmode(&self) -> u32 {
        (self.value.read_bits() & Self::DIVMODE_MASK) >> Self::DIVMODE_OFFSET
    }

    /// Write **DIVMODE** `[5:4]` (RW)
    #[inline(always)]
    pub fn write_divmode(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DIVMODE_MASK) | ((value << Self::DIVMODE_OFFSET) & Self::DIVMODE_MASK));
    }

    /// **B_INV** `[3]` (RW)
    ///
    /// Invert output B
    #[inline(always)]
    pub fn b_inv(&self) -> bool {
        (self.value.read_bits() & Self::B_INV_MASK) != 0
    }

    /// Set B_INV bit atomically (RW)
    #[inline(always)]
    pub fn set_b_inv(&self) {
        self.value.set_bits(Self::B_INV_MASK);
    }

    /// Clear B_INV bit atomically (RW)
    #[inline(always)]
    pub fn clr_b_inv(&self) {
        self.value.clr_bits(Self::B_INV_MASK);
    }

    /// **A_INV** `[2]` (RW)
    ///
    /// Invert output A
    #[inline(always)]
    pub fn a_inv(&self) -> bool {
        (self.value.read_bits() & Self::A_INV_MASK) != 0
    }

    /// Set A_INV bit atomically (RW)
    #[inline(always)]
    pub fn set_a_inv(&self) {
        self.value.set_bits(Self::A_INV_MASK);
    }

    /// Clear A_INV bit atomically (RW)
    #[inline(always)]
    pub fn clr_a_inv(&self) {
        self.value.clr_bits(Self::A_INV_MASK);
    }

    /// **PH_CORRECT** `[1]` (RW)
    ///
    /// 1: Enable phase-correct modulation. 0: Trailing-edge
    #[inline(always)]
    pub fn ph_correct(&self) -> bool {
        (self.value.read_bits() & Self::PH_CORRECT_MASK) != 0
    }

    /// Set PH_CORRECT bit atomically (RW)
    #[inline(always)]
    pub fn set_ph_correct(&self) {
        self.value.set_bits(Self::PH_CORRECT_MASK);
    }

    /// Clear PH_CORRECT bit atomically (RW)
    #[inline(always)]
    pub fn clr_ph_correct(&self) {
        self.value.clr_bits(Self::PH_CORRECT_MASK);
    }

    /// **EN** `[0]` (RW)
    ///
    /// Enable the PWM channel.
    #[inline(always)]
    pub fn en(&self) -> bool {
        (self.value.read_bits() & Self::EN_MASK) != 0
    }

    /// Set EN bit atomically (RW)
    #[inline(always)]
    pub fn set_en(&self) {
        self.value.set_bits(Self::EN_MASK);
    }

    /// Clear EN bit atomically (RW)
    #[inline(always)]
    pub fn clr_en(&self) {
        self.value.clr_bits(Self::EN_MASK);
    }

}

/// CH3_CSR value type for batch read/modify/write operations
///
/// Use `ch3csr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch3csr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch3CsrVal {
    bits: u32,
}

impl Ch3CsrVal {
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

    /// Read **DIVMODE** `[5:4]`
    #[inline(always)]
    pub const fn divmode(&self) -> u32 {
        (self.bits & Ch3Csr::DIVMODE_MASK) >> Ch3Csr::DIVMODE_OFFSET
    }

    /// Read **B_INV** `[3:3]`
    ///
    /// Invert output B
    #[inline(always)]
    pub const fn b_inv(&self) -> bool {
        (self.bits & Ch3Csr::B_INV_MASK) != 0
    }

    /// Read **A_INV** `[2:2]`
    ///
    /// Invert output A
    #[inline(always)]
    pub const fn a_inv(&self) -> bool {
        (self.bits & Ch3Csr::A_INV_MASK) != 0
    }

    /// Read **PH_CORRECT** `[1:1]`
    ///
    /// 1: Enable phase-correct modulation. 0: Trailing-edge
    #[inline(always)]
    pub const fn ph_correct(&self) -> bool {
        (self.bits & Ch3Csr::PH_CORRECT_MASK) != 0
    }

    /// Read **EN** `[0:0]`
    ///
    /// Enable the PWM channel.
    #[inline(always)]
    pub const fn en(&self) -> bool {
        (self.bits & Ch3Csr::EN_MASK) != 0
    }

    /// Set **PH_ADV** `[7:7]`
    ///
    /// Advance the phase of the counter by 1 count, while it is running.
    /// Self-clearing. Write a 1, and poll until low. Counter must be running
    /// at less than full speed (div_int + div_frac / 16 > 1)
    #[inline(always)]
    pub const fn with_ph_adv(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch3Csr::PH_ADV_MASK
            } else {
                self.bits & !Ch3Csr::PH_ADV_MASK
            }
        }
    }

    /// Set **PH_RET** `[6:6]`
    ///
    /// Retard the phase of the counter by 1 count, while it is running.
    /// Self-clearing. Write a 1, and poll until low. Counter must be running.
    #[inline(always)]
    pub const fn with_ph_ret(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch3Csr::PH_RET_MASK
            } else {
                self.bits & !Ch3Csr::PH_RET_MASK
            }
        }
    }

    /// Set **DIVMODE** `[5:4]`
    #[inline(always)]
    pub const fn with_divmode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch3Csr::DIVMODE_MASK) | ((value << Ch3Csr::DIVMODE_OFFSET) & Ch3Csr::DIVMODE_MASK)
        }
    }

    /// Set **B_INV** `[3:3]`
    ///
    /// Invert output B
    #[inline(always)]
    pub const fn with_b_inv(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch3Csr::B_INV_MASK
            } else {
                self.bits & !Ch3Csr::B_INV_MASK
            }
        }
    }

    /// Set **A_INV** `[2:2]`
    ///
    /// Invert output A
    #[inline(always)]
    pub const fn with_a_inv(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch3Csr::A_INV_MASK
            } else {
                self.bits & !Ch3Csr::A_INV_MASK
            }
        }
    }

    /// Set **PH_CORRECT** `[1:1]`
    ///
    /// 1: Enable phase-correct modulation. 0: Trailing-edge
    #[inline(always)]
    pub const fn with_ph_correct(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch3Csr::PH_CORRECT_MASK
            } else {
                self.bits & !Ch3Csr::PH_CORRECT_MASK
            }
        }
    }

    /// Set **EN** `[0:0]`
    ///
    /// Enable the PWM channel.
    #[inline(always)]
    pub const fn with_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch3Csr::EN_MASK
            } else {
                self.bits & !Ch3Csr::EN_MASK
            }
        }
    }

}

impl Default for Ch3CsrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ch3Csr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch3CsrVal {
        Ch3CsrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch3CsrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch3CsrVal) -> Ch3CsrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// CH3_DIV Register
///
/// INT and FRAC form a fixed-point fractional number.
/// Counting rate is system clock frequency divided by this number.
/// Fractional division uses simple 1st-order sigma-delta.
///
/// Address offset: 0x0040
/// Reset value: 0x00000010
///
/// ## Fields
///
/// - **INT** `[11:4]` (RW)
///
/// - **FRAC** `[3:0]` (RW)
#[repr(transparent)]
pub struct Ch3Div {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch3Div {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch3Div {
    // ======== Field Constants ========

    /// **INT**
    /// Bits `[11:4]`, mask `0x00000ff0`
    pub const INT_OFFSET: u32 = 4;
    pub const INT_MASK: u32 = 0x00000ff0;

    /// **FRAC**
    /// Bits `[3:0]`, mask `0x0000000f`
    pub const FRAC_OFFSET: u32 = 0;
    pub const FRAC_MASK: u32 = 0x0000000f;

    // ======== Field Accessors ========

    /// **INT** `[11:4]` (RW)
    #[inline(always)]
    pub fn int(&self) -> u32 {
        (self.value.read_bits() & Self::INT_MASK) >> Self::INT_OFFSET
    }

    /// Write **INT** `[11:4]` (RW)
    #[inline(always)]
    pub fn write_int(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::INT_MASK) | ((value << Self::INT_OFFSET) & Self::INT_MASK));
    }

    /// **FRAC** `[3:0]` (RW)
    #[inline(always)]
    pub fn frac(&self) -> u32 {
        (self.value.read_bits() & Self::FRAC_MASK) >> Self::FRAC_OFFSET
    }

    /// Write **FRAC** `[3:0]` (RW)
    #[inline(always)]
    pub fn write_frac(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::FRAC_MASK) | ((value << Self::FRAC_OFFSET) & Self::FRAC_MASK));
    }

}

/// CH3_DIV value type for batch read/modify/write operations
///
/// Use `ch3div.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch3div.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch3DivVal {
    bits: u32,
}

impl Ch3DivVal {
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
        Self { bits: 0x00000010 }
    }

    /// Read **INT** `[11:4]`
    #[inline(always)]
    pub const fn int(&self) -> u32 {
        (self.bits & Ch3Div::INT_MASK) >> Ch3Div::INT_OFFSET
    }

    /// Read **FRAC** `[3:0]`
    #[inline(always)]
    pub const fn frac(&self) -> u32 {
        (self.bits & Ch3Div::FRAC_MASK) >> Ch3Div::FRAC_OFFSET
    }

    /// Set **INT** `[11:4]`
    #[inline(always)]
    pub const fn with_int(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch3Div::INT_MASK) | ((value << Ch3Div::INT_OFFSET) & Ch3Div::INT_MASK)
        }
    }

    /// Set **FRAC** `[3:0]`
    #[inline(always)]
    pub const fn with_frac(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch3Div::FRAC_MASK) | ((value << Ch3Div::FRAC_OFFSET) & Ch3Div::FRAC_MASK)
        }
    }

}

impl Default for Ch3DivVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ch3Div {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch3DivVal {
        Ch3DivVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch3DivVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch3DivVal) -> Ch3DivVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// CH3_CTR Register
///
/// Direct access to the PWM counter
///
/// Address offset: 0x0044
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CH3_CTR** `[15:0]` (RW)
#[repr(transparent)]
pub struct Ch3Ctr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch3Ctr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch3Ctr {
    // ======== Field Constants ========

    /// **CH3_CTR**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const CH3_CTR_OFFSET: u32 = 0;
    pub const CH3_CTR_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **CH3_CTR** `[15:0]` (RW)
    #[inline(always)]
    pub fn ch3_ctr(&self) -> u32 {
        (self.value.read_bits() & Self::CH3_CTR_MASK) >> Self::CH3_CTR_OFFSET
    }

    /// Write **CH3_CTR** `[15:0]` (RW)
    #[inline(always)]
    pub fn write_ch3_ctr(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::CH3_CTR_MASK) | ((value << Self::CH3_CTR_OFFSET) & Self::CH3_CTR_MASK));
    }

}

/// CH3_CTR value type for batch read/modify/write operations
///
/// Use `ch3ctr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch3ctr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch3CtrVal {
    bits: u32,
}

impl Ch3CtrVal {
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

    /// Read **CH3_CTR** `[15:0]`
    #[inline(always)]
    pub const fn ch3_ctr(&self) -> u32 {
        (self.bits & Ch3Ctr::CH3_CTR_MASK) >> Ch3Ctr::CH3_CTR_OFFSET
    }

    /// Set **CH3_CTR** `[15:0]`
    #[inline(always)]
    pub const fn with_ch3_ctr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch3Ctr::CH3_CTR_MASK) | ((value << Ch3Ctr::CH3_CTR_OFFSET) & Ch3Ctr::CH3_CTR_MASK)
        }
    }

}

impl Default for Ch3CtrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ch3Ctr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch3CtrVal {
        Ch3CtrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch3CtrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch3CtrVal) -> Ch3CtrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// CH3_CC Register
///
/// Counter compare values
///
/// Address offset: 0x0048
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **B** `[31:16]` (RW)
///
/// - **A** `[15:0]` (RW)
#[repr(transparent)]
pub struct Ch3Cc {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch3Cc {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch3Cc {
    // ======== Field Constants ========

    /// **B**
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const B_OFFSET: u32 = 16;
    pub const B_MASK: u32 = 0xffff0000;

    /// **A**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const A_OFFSET: u32 = 0;
    pub const A_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **B** `[31:16]` (RW)
    #[inline(always)]
    pub fn b(&self) -> u32 {
        (self.value.read_bits() & Self::B_MASK) >> Self::B_OFFSET
    }

    /// Write **B** `[31:16]` (RW)
    #[inline(always)]
    pub fn write_b(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::B_MASK) | ((value << Self::B_OFFSET) & Self::B_MASK));
    }

    /// **A** `[15:0]` (RW)
    #[inline(always)]
    pub fn a(&self) -> u32 {
        (self.value.read_bits() & Self::A_MASK) >> Self::A_OFFSET
    }

    /// Write **A** `[15:0]` (RW)
    #[inline(always)]
    pub fn write_a(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::A_MASK) | ((value << Self::A_OFFSET) & Self::A_MASK));
    }

}

/// CH3_CC value type for batch read/modify/write operations
///
/// Use `ch3cc.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch3cc.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch3CcVal {
    bits: u32,
}

impl Ch3CcVal {
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

    /// Read **B** `[31:16]`
    #[inline(always)]
    pub const fn b(&self) -> u32 {
        (self.bits & Ch3Cc::B_MASK) >> Ch3Cc::B_OFFSET
    }

    /// Read **A** `[15:0]`
    #[inline(always)]
    pub const fn a(&self) -> u32 {
        (self.bits & Ch3Cc::A_MASK) >> Ch3Cc::A_OFFSET
    }

    /// Set **B** `[31:16]`
    #[inline(always)]
    pub const fn with_b(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch3Cc::B_MASK) | ((value << Ch3Cc::B_OFFSET) & Ch3Cc::B_MASK)
        }
    }

    /// Set **A** `[15:0]`
    #[inline(always)]
    pub const fn with_a(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch3Cc::A_MASK) | ((value << Ch3Cc::A_OFFSET) & Ch3Cc::A_MASK)
        }
    }

}

impl Default for Ch3CcVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ch3Cc {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch3CcVal {
        Ch3CcVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch3CcVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch3CcVal) -> Ch3CcVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// CH3_TOP Register
///
/// Counter wrap value
///
/// Address offset: 0x004c
/// Reset value: 0x0000ffff
///
/// ## Fields
///
/// - **CH3_TOP** `[15:0]` (RW)
#[repr(transparent)]
pub struct Ch3Top {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch3Top {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch3Top {
    // ======== Field Constants ========

    /// **CH3_TOP**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const CH3_TOP_OFFSET: u32 = 0;
    pub const CH3_TOP_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **CH3_TOP** `[15:0]` (RW)
    #[inline(always)]
    pub fn ch3_top(&self) -> u32 {
        (self.value.read_bits() & Self::CH3_TOP_MASK) >> Self::CH3_TOP_OFFSET
    }

    /// Write **CH3_TOP** `[15:0]` (RW)
    #[inline(always)]
    pub fn write_ch3_top(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::CH3_TOP_MASK) | ((value << Self::CH3_TOP_OFFSET) & Self::CH3_TOP_MASK));
    }

}

/// CH3_TOP value type for batch read/modify/write operations
///
/// Use `ch3top.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch3top.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch3TopVal {
    bits: u32,
}

impl Ch3TopVal {
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
        Self { bits: 0x0000ffff }
    }

    /// Read **CH3_TOP** `[15:0]`
    #[inline(always)]
    pub const fn ch3_top(&self) -> u32 {
        (self.bits & Ch3Top::CH3_TOP_MASK) >> Ch3Top::CH3_TOP_OFFSET
    }

    /// Set **CH3_TOP** `[15:0]`
    #[inline(always)]
    pub const fn with_ch3_top(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch3Top::CH3_TOP_MASK) | ((value << Ch3Top::CH3_TOP_OFFSET) & Ch3Top::CH3_TOP_MASK)
        }
    }

}

impl Default for Ch3TopVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ch3Top {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch3TopVal {
        Ch3TopVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch3TopVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch3TopVal) -> Ch3TopVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// CH4_CSR Register
///
/// Control and status register
///
/// Address offset: 0x0050
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **PH_ADV** `[7]` (WO)
///
///   Advance the phase of the counter by 1 count, while it is running.
///   Self-clearing. Write a 1, and poll until low. Counter must be running
///   at less than full speed (div_int + div_frac / 16 > 1)
///
/// - **PH_RET** `[6]` (WO)
///
///   Retard the phase of the counter by 1 count, while it is running.
///   Self-clearing. Write a 1, and poll until low. Counter must be running.
///
/// - **DIVMODE** `[5:4]` (RW)
///
/// - **B_INV** `[3]` (RW)
///
///   Invert output B
///
/// - **A_INV** `[2]` (RW)
///
///   Invert output A
///
/// - **PH_CORRECT** `[1]` (RW)
///
///   1: Enable phase-correct modulation. 0: Trailing-edge
///
/// - **EN** `[0]` (RW)
///
///   Enable the PWM channel.
#[repr(transparent)]
pub struct Ch4Csr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch4Csr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch4Csr {
    // ======== Field Constants ========

    /// **PH_ADV**
    ///
    /// Advance the phase of the counter by 1 count, while it is running.
    /// Self-clearing. Write a 1, and poll until low. Counter must be running
    /// at less than full speed (div_int + div_frac / 16 > 1)
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const PH_ADV_OFFSET: u32 = 7;
    pub const PH_ADV_MASK: u32 = 0x00000080;
    pub const PH_ADV: u32 = 1 << 7;

    /// **PH_RET**
    ///
    /// Retard the phase of the counter by 1 count, while it is running.
    /// Self-clearing. Write a 1, and poll until low. Counter must be running.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const PH_RET_OFFSET: u32 = 6;
    pub const PH_RET_MASK: u32 = 0x00000040;
    pub const PH_RET: u32 = 1 << 6;

    /// **DIVMODE**
    /// Bits `[5:4]`, mask `0x00000030`
    pub const DIVMODE_OFFSET: u32 = 4;
    pub const DIVMODE_MASK: u32 = 0x00000030;
    /// div = 0
    /// Free-running counting at rate dictated by fractional divider
    pub const DIVMODE_DIV: u32 = 0 << 4;
    pub const DIVMODE_DIV_VAL: u32 = 0;
    /// level = 1
    /// Fractional divider operation is gated by the PWM B pin.
    pub const DIVMODE_LEVEL: u32 = 1 << 4;
    pub const DIVMODE_LEVEL_VAL: u32 = 1;
    /// rise = 2
    /// Counter advances with each rising edge of the PWM B pin.
    pub const DIVMODE_RISE: u32 = 2 << 4;
    pub const DIVMODE_RISE_VAL: u32 = 2;
    /// fall = 3
    /// Counter advances with each falling edge of the PWM B pin.
    pub const DIVMODE_FALL: u32 = 3 << 4;
    pub const DIVMODE_FALL_VAL: u32 = 3;

    /// **B_INV**
    ///
    /// Invert output B
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const B_INV_OFFSET: u32 = 3;
    pub const B_INV_MASK: u32 = 0x00000008;
    pub const B_INV: u32 = 1 << 3;

    /// **A_INV**
    ///
    /// Invert output A
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const A_INV_OFFSET: u32 = 2;
    pub const A_INV_MASK: u32 = 0x00000004;
    pub const A_INV: u32 = 1 << 2;

    /// **PH_CORRECT**
    ///
    /// 1: Enable phase-correct modulation. 0: Trailing-edge
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const PH_CORRECT_OFFSET: u32 = 1;
    pub const PH_CORRECT_MASK: u32 = 0x00000002;
    pub const PH_CORRECT: u32 = 1 << 1;

    /// **EN**
    ///
    /// Enable the PWM channel.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const EN_OFFSET: u32 = 0;
    pub const EN_MASK: u32 = 0x00000001;
    pub const EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **PH_ADV** (WO)
    ///
    /// Advance the phase of the counter by 1 count, while it is running.
    /// Self-clearing. Write a 1, and poll until low. Counter must be running
    /// at less than full speed (div_int + div_frac / 16 > 1)
    #[inline(always)]
    pub fn trigger_ph_adv(&self) {
        self.value.write_bits(Self::PH_ADV_MASK);
    }

    /// Trigger **PH_RET** (WO)
    ///
    /// Retard the phase of the counter by 1 count, while it is running.
    /// Self-clearing. Write a 1, and poll until low. Counter must be running.
    #[inline(always)]
    pub fn trigger_ph_ret(&self) {
        self.value.write_bits(Self::PH_RET_MASK);
    }

    /// **DIVMODE** `[5:4]` (RW)
    #[inline(always)]
    pub fn divmode(&self) -> u32 {
        (self.value.read_bits() & Self::DIVMODE_MASK) >> Self::DIVMODE_OFFSET
    }

    /// Write **DIVMODE** `[5:4]` (RW)
    #[inline(always)]
    pub fn write_divmode(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DIVMODE_MASK) | ((value << Self::DIVMODE_OFFSET) & Self::DIVMODE_MASK));
    }

    /// **B_INV** `[3]` (RW)
    ///
    /// Invert output B
    #[inline(always)]
    pub fn b_inv(&self) -> bool {
        (self.value.read_bits() & Self::B_INV_MASK) != 0
    }

    /// Set B_INV bit atomically (RW)
    #[inline(always)]
    pub fn set_b_inv(&self) {
        self.value.set_bits(Self::B_INV_MASK);
    }

    /// Clear B_INV bit atomically (RW)
    #[inline(always)]
    pub fn clr_b_inv(&self) {
        self.value.clr_bits(Self::B_INV_MASK);
    }

    /// **A_INV** `[2]` (RW)
    ///
    /// Invert output A
    #[inline(always)]
    pub fn a_inv(&self) -> bool {
        (self.value.read_bits() & Self::A_INV_MASK) != 0
    }

    /// Set A_INV bit atomically (RW)
    #[inline(always)]
    pub fn set_a_inv(&self) {
        self.value.set_bits(Self::A_INV_MASK);
    }

    /// Clear A_INV bit atomically (RW)
    #[inline(always)]
    pub fn clr_a_inv(&self) {
        self.value.clr_bits(Self::A_INV_MASK);
    }

    /// **PH_CORRECT** `[1]` (RW)
    ///
    /// 1: Enable phase-correct modulation. 0: Trailing-edge
    #[inline(always)]
    pub fn ph_correct(&self) -> bool {
        (self.value.read_bits() & Self::PH_CORRECT_MASK) != 0
    }

    /// Set PH_CORRECT bit atomically (RW)
    #[inline(always)]
    pub fn set_ph_correct(&self) {
        self.value.set_bits(Self::PH_CORRECT_MASK);
    }

    /// Clear PH_CORRECT bit atomically (RW)
    #[inline(always)]
    pub fn clr_ph_correct(&self) {
        self.value.clr_bits(Self::PH_CORRECT_MASK);
    }

    /// **EN** `[0]` (RW)
    ///
    /// Enable the PWM channel.
    #[inline(always)]
    pub fn en(&self) -> bool {
        (self.value.read_bits() & Self::EN_MASK) != 0
    }

    /// Set EN bit atomically (RW)
    #[inline(always)]
    pub fn set_en(&self) {
        self.value.set_bits(Self::EN_MASK);
    }

    /// Clear EN bit atomically (RW)
    #[inline(always)]
    pub fn clr_en(&self) {
        self.value.clr_bits(Self::EN_MASK);
    }

}

/// CH4_CSR value type for batch read/modify/write operations
///
/// Use `ch4csr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch4csr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch4CsrVal {
    bits: u32,
}

impl Ch4CsrVal {
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

    /// Read **DIVMODE** `[5:4]`
    #[inline(always)]
    pub const fn divmode(&self) -> u32 {
        (self.bits & Ch4Csr::DIVMODE_MASK) >> Ch4Csr::DIVMODE_OFFSET
    }

    /// Read **B_INV** `[3:3]`
    ///
    /// Invert output B
    #[inline(always)]
    pub const fn b_inv(&self) -> bool {
        (self.bits & Ch4Csr::B_INV_MASK) != 0
    }

    /// Read **A_INV** `[2:2]`
    ///
    /// Invert output A
    #[inline(always)]
    pub const fn a_inv(&self) -> bool {
        (self.bits & Ch4Csr::A_INV_MASK) != 0
    }

    /// Read **PH_CORRECT** `[1:1]`
    ///
    /// 1: Enable phase-correct modulation. 0: Trailing-edge
    #[inline(always)]
    pub const fn ph_correct(&self) -> bool {
        (self.bits & Ch4Csr::PH_CORRECT_MASK) != 0
    }

    /// Read **EN** `[0:0]`
    ///
    /// Enable the PWM channel.
    #[inline(always)]
    pub const fn en(&self) -> bool {
        (self.bits & Ch4Csr::EN_MASK) != 0
    }

    /// Set **PH_ADV** `[7:7]`
    ///
    /// Advance the phase of the counter by 1 count, while it is running.
    /// Self-clearing. Write a 1, and poll until low. Counter must be running
    /// at less than full speed (div_int + div_frac / 16 > 1)
    #[inline(always)]
    pub const fn with_ph_adv(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch4Csr::PH_ADV_MASK
            } else {
                self.bits & !Ch4Csr::PH_ADV_MASK
            }
        }
    }

    /// Set **PH_RET** `[6:6]`
    ///
    /// Retard the phase of the counter by 1 count, while it is running.
    /// Self-clearing. Write a 1, and poll until low. Counter must be running.
    #[inline(always)]
    pub const fn with_ph_ret(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch4Csr::PH_RET_MASK
            } else {
                self.bits & !Ch4Csr::PH_RET_MASK
            }
        }
    }

    /// Set **DIVMODE** `[5:4]`
    #[inline(always)]
    pub const fn with_divmode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch4Csr::DIVMODE_MASK) | ((value << Ch4Csr::DIVMODE_OFFSET) & Ch4Csr::DIVMODE_MASK)
        }
    }

    /// Set **B_INV** `[3:3]`
    ///
    /// Invert output B
    #[inline(always)]
    pub const fn with_b_inv(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch4Csr::B_INV_MASK
            } else {
                self.bits & !Ch4Csr::B_INV_MASK
            }
        }
    }

    /// Set **A_INV** `[2:2]`
    ///
    /// Invert output A
    #[inline(always)]
    pub const fn with_a_inv(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch4Csr::A_INV_MASK
            } else {
                self.bits & !Ch4Csr::A_INV_MASK
            }
        }
    }

    /// Set **PH_CORRECT** `[1:1]`
    ///
    /// 1: Enable phase-correct modulation. 0: Trailing-edge
    #[inline(always)]
    pub const fn with_ph_correct(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch4Csr::PH_CORRECT_MASK
            } else {
                self.bits & !Ch4Csr::PH_CORRECT_MASK
            }
        }
    }

    /// Set **EN** `[0:0]`
    ///
    /// Enable the PWM channel.
    #[inline(always)]
    pub const fn with_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch4Csr::EN_MASK
            } else {
                self.bits & !Ch4Csr::EN_MASK
            }
        }
    }

}

impl Default for Ch4CsrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ch4Csr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch4CsrVal {
        Ch4CsrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch4CsrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch4CsrVal) -> Ch4CsrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// CH4_DIV Register
///
/// INT and FRAC form a fixed-point fractional number.
/// Counting rate is system clock frequency divided by this number.
/// Fractional division uses simple 1st-order sigma-delta.
///
/// Address offset: 0x0054
/// Reset value: 0x00000010
///
/// ## Fields
///
/// - **INT** `[11:4]` (RW)
///
/// - **FRAC** `[3:0]` (RW)
#[repr(transparent)]
pub struct Ch4Div {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch4Div {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch4Div {
    // ======== Field Constants ========

    /// **INT**
    /// Bits `[11:4]`, mask `0x00000ff0`
    pub const INT_OFFSET: u32 = 4;
    pub const INT_MASK: u32 = 0x00000ff0;

    /// **FRAC**
    /// Bits `[3:0]`, mask `0x0000000f`
    pub const FRAC_OFFSET: u32 = 0;
    pub const FRAC_MASK: u32 = 0x0000000f;

    // ======== Field Accessors ========

    /// **INT** `[11:4]` (RW)
    #[inline(always)]
    pub fn int(&self) -> u32 {
        (self.value.read_bits() & Self::INT_MASK) >> Self::INT_OFFSET
    }

    /// Write **INT** `[11:4]` (RW)
    #[inline(always)]
    pub fn write_int(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::INT_MASK) | ((value << Self::INT_OFFSET) & Self::INT_MASK));
    }

    /// **FRAC** `[3:0]` (RW)
    #[inline(always)]
    pub fn frac(&self) -> u32 {
        (self.value.read_bits() & Self::FRAC_MASK) >> Self::FRAC_OFFSET
    }

    /// Write **FRAC** `[3:0]` (RW)
    #[inline(always)]
    pub fn write_frac(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::FRAC_MASK) | ((value << Self::FRAC_OFFSET) & Self::FRAC_MASK));
    }

}

/// CH4_DIV value type for batch read/modify/write operations
///
/// Use `ch4div.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch4div.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch4DivVal {
    bits: u32,
}

impl Ch4DivVal {
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
        Self { bits: 0x00000010 }
    }

    /// Read **INT** `[11:4]`
    #[inline(always)]
    pub const fn int(&self) -> u32 {
        (self.bits & Ch4Div::INT_MASK) >> Ch4Div::INT_OFFSET
    }

    /// Read **FRAC** `[3:0]`
    #[inline(always)]
    pub const fn frac(&self) -> u32 {
        (self.bits & Ch4Div::FRAC_MASK) >> Ch4Div::FRAC_OFFSET
    }

    /// Set **INT** `[11:4]`
    #[inline(always)]
    pub const fn with_int(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch4Div::INT_MASK) | ((value << Ch4Div::INT_OFFSET) & Ch4Div::INT_MASK)
        }
    }

    /// Set **FRAC** `[3:0]`
    #[inline(always)]
    pub const fn with_frac(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch4Div::FRAC_MASK) | ((value << Ch4Div::FRAC_OFFSET) & Ch4Div::FRAC_MASK)
        }
    }

}

impl Default for Ch4DivVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ch4Div {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch4DivVal {
        Ch4DivVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch4DivVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch4DivVal) -> Ch4DivVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// CH4_CTR Register
///
/// Direct access to the PWM counter
///
/// Address offset: 0x0058
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CH4_CTR** `[15:0]` (RW)
#[repr(transparent)]
pub struct Ch4Ctr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch4Ctr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch4Ctr {
    // ======== Field Constants ========

    /// **CH4_CTR**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const CH4_CTR_OFFSET: u32 = 0;
    pub const CH4_CTR_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **CH4_CTR** `[15:0]` (RW)
    #[inline(always)]
    pub fn ch4_ctr(&self) -> u32 {
        (self.value.read_bits() & Self::CH4_CTR_MASK) >> Self::CH4_CTR_OFFSET
    }

    /// Write **CH4_CTR** `[15:0]` (RW)
    #[inline(always)]
    pub fn write_ch4_ctr(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::CH4_CTR_MASK) | ((value << Self::CH4_CTR_OFFSET) & Self::CH4_CTR_MASK));
    }

}

/// CH4_CTR value type for batch read/modify/write operations
///
/// Use `ch4ctr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch4ctr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch4CtrVal {
    bits: u32,
}

impl Ch4CtrVal {
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

    /// Read **CH4_CTR** `[15:0]`
    #[inline(always)]
    pub const fn ch4_ctr(&self) -> u32 {
        (self.bits & Ch4Ctr::CH4_CTR_MASK) >> Ch4Ctr::CH4_CTR_OFFSET
    }

    /// Set **CH4_CTR** `[15:0]`
    #[inline(always)]
    pub const fn with_ch4_ctr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch4Ctr::CH4_CTR_MASK) | ((value << Ch4Ctr::CH4_CTR_OFFSET) & Ch4Ctr::CH4_CTR_MASK)
        }
    }

}

impl Default for Ch4CtrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ch4Ctr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch4CtrVal {
        Ch4CtrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch4CtrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch4CtrVal) -> Ch4CtrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// CH4_CC Register
///
/// Counter compare values
///
/// Address offset: 0x005c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **B** `[31:16]` (RW)
///
/// - **A** `[15:0]` (RW)
#[repr(transparent)]
pub struct Ch4Cc {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch4Cc {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch4Cc {
    // ======== Field Constants ========

    /// **B**
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const B_OFFSET: u32 = 16;
    pub const B_MASK: u32 = 0xffff0000;

    /// **A**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const A_OFFSET: u32 = 0;
    pub const A_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **B** `[31:16]` (RW)
    #[inline(always)]
    pub fn b(&self) -> u32 {
        (self.value.read_bits() & Self::B_MASK) >> Self::B_OFFSET
    }

    /// Write **B** `[31:16]` (RW)
    #[inline(always)]
    pub fn write_b(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::B_MASK) | ((value << Self::B_OFFSET) & Self::B_MASK));
    }

    /// **A** `[15:0]` (RW)
    #[inline(always)]
    pub fn a(&self) -> u32 {
        (self.value.read_bits() & Self::A_MASK) >> Self::A_OFFSET
    }

    /// Write **A** `[15:0]` (RW)
    #[inline(always)]
    pub fn write_a(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::A_MASK) | ((value << Self::A_OFFSET) & Self::A_MASK));
    }

}

/// CH4_CC value type for batch read/modify/write operations
///
/// Use `ch4cc.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch4cc.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch4CcVal {
    bits: u32,
}

impl Ch4CcVal {
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

    /// Read **B** `[31:16]`
    #[inline(always)]
    pub const fn b(&self) -> u32 {
        (self.bits & Ch4Cc::B_MASK) >> Ch4Cc::B_OFFSET
    }

    /// Read **A** `[15:0]`
    #[inline(always)]
    pub const fn a(&self) -> u32 {
        (self.bits & Ch4Cc::A_MASK) >> Ch4Cc::A_OFFSET
    }

    /// Set **B** `[31:16]`
    #[inline(always)]
    pub const fn with_b(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch4Cc::B_MASK) | ((value << Ch4Cc::B_OFFSET) & Ch4Cc::B_MASK)
        }
    }

    /// Set **A** `[15:0]`
    #[inline(always)]
    pub const fn with_a(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch4Cc::A_MASK) | ((value << Ch4Cc::A_OFFSET) & Ch4Cc::A_MASK)
        }
    }

}

impl Default for Ch4CcVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ch4Cc {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch4CcVal {
        Ch4CcVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch4CcVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch4CcVal) -> Ch4CcVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// CH4_TOP Register
///
/// Counter wrap value
///
/// Address offset: 0x0060
/// Reset value: 0x0000ffff
///
/// ## Fields
///
/// - **CH4_TOP** `[15:0]` (RW)
#[repr(transparent)]
pub struct Ch4Top {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch4Top {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch4Top {
    // ======== Field Constants ========

    /// **CH4_TOP**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const CH4_TOP_OFFSET: u32 = 0;
    pub const CH4_TOP_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **CH4_TOP** `[15:0]` (RW)
    #[inline(always)]
    pub fn ch4_top(&self) -> u32 {
        (self.value.read_bits() & Self::CH4_TOP_MASK) >> Self::CH4_TOP_OFFSET
    }

    /// Write **CH4_TOP** `[15:0]` (RW)
    #[inline(always)]
    pub fn write_ch4_top(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::CH4_TOP_MASK) | ((value << Self::CH4_TOP_OFFSET) & Self::CH4_TOP_MASK));
    }

}

/// CH4_TOP value type for batch read/modify/write operations
///
/// Use `ch4top.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch4top.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch4TopVal {
    bits: u32,
}

impl Ch4TopVal {
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
        Self { bits: 0x0000ffff }
    }

    /// Read **CH4_TOP** `[15:0]`
    #[inline(always)]
    pub const fn ch4_top(&self) -> u32 {
        (self.bits & Ch4Top::CH4_TOP_MASK) >> Ch4Top::CH4_TOP_OFFSET
    }

    /// Set **CH4_TOP** `[15:0]`
    #[inline(always)]
    pub const fn with_ch4_top(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch4Top::CH4_TOP_MASK) | ((value << Ch4Top::CH4_TOP_OFFSET) & Ch4Top::CH4_TOP_MASK)
        }
    }

}

impl Default for Ch4TopVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ch4Top {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch4TopVal {
        Ch4TopVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch4TopVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch4TopVal) -> Ch4TopVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// CH5_CSR Register
///
/// Control and status register
///
/// Address offset: 0x0064
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **PH_ADV** `[7]` (WO)
///
///   Advance the phase of the counter by 1 count, while it is running.
///   Self-clearing. Write a 1, and poll until low. Counter must be running
///   at less than full speed (div_int + div_frac / 16 > 1)
///
/// - **PH_RET** `[6]` (WO)
///
///   Retard the phase of the counter by 1 count, while it is running.
///   Self-clearing. Write a 1, and poll until low. Counter must be running.
///
/// - **DIVMODE** `[5:4]` (RW)
///
/// - **B_INV** `[3]` (RW)
///
///   Invert output B
///
/// - **A_INV** `[2]` (RW)
///
///   Invert output A
///
/// - **PH_CORRECT** `[1]` (RW)
///
///   1: Enable phase-correct modulation. 0: Trailing-edge
///
/// - **EN** `[0]` (RW)
///
///   Enable the PWM channel.
#[repr(transparent)]
pub struct Ch5Csr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch5Csr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch5Csr {
    // ======== Field Constants ========

    /// **PH_ADV**
    ///
    /// Advance the phase of the counter by 1 count, while it is running.
    /// Self-clearing. Write a 1, and poll until low. Counter must be running
    /// at less than full speed (div_int + div_frac / 16 > 1)
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const PH_ADV_OFFSET: u32 = 7;
    pub const PH_ADV_MASK: u32 = 0x00000080;
    pub const PH_ADV: u32 = 1 << 7;

    /// **PH_RET**
    ///
    /// Retard the phase of the counter by 1 count, while it is running.
    /// Self-clearing. Write a 1, and poll until low. Counter must be running.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const PH_RET_OFFSET: u32 = 6;
    pub const PH_RET_MASK: u32 = 0x00000040;
    pub const PH_RET: u32 = 1 << 6;

    /// **DIVMODE**
    /// Bits `[5:4]`, mask `0x00000030`
    pub const DIVMODE_OFFSET: u32 = 4;
    pub const DIVMODE_MASK: u32 = 0x00000030;
    /// div = 0
    /// Free-running counting at rate dictated by fractional divider
    pub const DIVMODE_DIV: u32 = 0 << 4;
    pub const DIVMODE_DIV_VAL: u32 = 0;
    /// level = 1
    /// Fractional divider operation is gated by the PWM B pin.
    pub const DIVMODE_LEVEL: u32 = 1 << 4;
    pub const DIVMODE_LEVEL_VAL: u32 = 1;
    /// rise = 2
    /// Counter advances with each rising edge of the PWM B pin.
    pub const DIVMODE_RISE: u32 = 2 << 4;
    pub const DIVMODE_RISE_VAL: u32 = 2;
    /// fall = 3
    /// Counter advances with each falling edge of the PWM B pin.
    pub const DIVMODE_FALL: u32 = 3 << 4;
    pub const DIVMODE_FALL_VAL: u32 = 3;

    /// **B_INV**
    ///
    /// Invert output B
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const B_INV_OFFSET: u32 = 3;
    pub const B_INV_MASK: u32 = 0x00000008;
    pub const B_INV: u32 = 1 << 3;

    /// **A_INV**
    ///
    /// Invert output A
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const A_INV_OFFSET: u32 = 2;
    pub const A_INV_MASK: u32 = 0x00000004;
    pub const A_INV: u32 = 1 << 2;

    /// **PH_CORRECT**
    ///
    /// 1: Enable phase-correct modulation. 0: Trailing-edge
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const PH_CORRECT_OFFSET: u32 = 1;
    pub const PH_CORRECT_MASK: u32 = 0x00000002;
    pub const PH_CORRECT: u32 = 1 << 1;

    /// **EN**
    ///
    /// Enable the PWM channel.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const EN_OFFSET: u32 = 0;
    pub const EN_MASK: u32 = 0x00000001;
    pub const EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **PH_ADV** (WO)
    ///
    /// Advance the phase of the counter by 1 count, while it is running.
    /// Self-clearing. Write a 1, and poll until low. Counter must be running
    /// at less than full speed (div_int + div_frac / 16 > 1)
    #[inline(always)]
    pub fn trigger_ph_adv(&self) {
        self.value.write_bits(Self::PH_ADV_MASK);
    }

    /// Trigger **PH_RET** (WO)
    ///
    /// Retard the phase of the counter by 1 count, while it is running.
    /// Self-clearing. Write a 1, and poll until low. Counter must be running.
    #[inline(always)]
    pub fn trigger_ph_ret(&self) {
        self.value.write_bits(Self::PH_RET_MASK);
    }

    /// **DIVMODE** `[5:4]` (RW)
    #[inline(always)]
    pub fn divmode(&self) -> u32 {
        (self.value.read_bits() & Self::DIVMODE_MASK) >> Self::DIVMODE_OFFSET
    }

    /// Write **DIVMODE** `[5:4]` (RW)
    #[inline(always)]
    pub fn write_divmode(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DIVMODE_MASK) | ((value << Self::DIVMODE_OFFSET) & Self::DIVMODE_MASK));
    }

    /// **B_INV** `[3]` (RW)
    ///
    /// Invert output B
    #[inline(always)]
    pub fn b_inv(&self) -> bool {
        (self.value.read_bits() & Self::B_INV_MASK) != 0
    }

    /// Set B_INV bit atomically (RW)
    #[inline(always)]
    pub fn set_b_inv(&self) {
        self.value.set_bits(Self::B_INV_MASK);
    }

    /// Clear B_INV bit atomically (RW)
    #[inline(always)]
    pub fn clr_b_inv(&self) {
        self.value.clr_bits(Self::B_INV_MASK);
    }

    /// **A_INV** `[2]` (RW)
    ///
    /// Invert output A
    #[inline(always)]
    pub fn a_inv(&self) -> bool {
        (self.value.read_bits() & Self::A_INV_MASK) != 0
    }

    /// Set A_INV bit atomically (RW)
    #[inline(always)]
    pub fn set_a_inv(&self) {
        self.value.set_bits(Self::A_INV_MASK);
    }

    /// Clear A_INV bit atomically (RW)
    #[inline(always)]
    pub fn clr_a_inv(&self) {
        self.value.clr_bits(Self::A_INV_MASK);
    }

    /// **PH_CORRECT** `[1]` (RW)
    ///
    /// 1: Enable phase-correct modulation. 0: Trailing-edge
    #[inline(always)]
    pub fn ph_correct(&self) -> bool {
        (self.value.read_bits() & Self::PH_CORRECT_MASK) != 0
    }

    /// Set PH_CORRECT bit atomically (RW)
    #[inline(always)]
    pub fn set_ph_correct(&self) {
        self.value.set_bits(Self::PH_CORRECT_MASK);
    }

    /// Clear PH_CORRECT bit atomically (RW)
    #[inline(always)]
    pub fn clr_ph_correct(&self) {
        self.value.clr_bits(Self::PH_CORRECT_MASK);
    }

    /// **EN** `[0]` (RW)
    ///
    /// Enable the PWM channel.
    #[inline(always)]
    pub fn en(&self) -> bool {
        (self.value.read_bits() & Self::EN_MASK) != 0
    }

    /// Set EN bit atomically (RW)
    #[inline(always)]
    pub fn set_en(&self) {
        self.value.set_bits(Self::EN_MASK);
    }

    /// Clear EN bit atomically (RW)
    #[inline(always)]
    pub fn clr_en(&self) {
        self.value.clr_bits(Self::EN_MASK);
    }

}

/// CH5_CSR value type for batch read/modify/write operations
///
/// Use `ch5csr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch5csr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch5CsrVal {
    bits: u32,
}

impl Ch5CsrVal {
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

    /// Read **DIVMODE** `[5:4]`
    #[inline(always)]
    pub const fn divmode(&self) -> u32 {
        (self.bits & Ch5Csr::DIVMODE_MASK) >> Ch5Csr::DIVMODE_OFFSET
    }

    /// Read **B_INV** `[3:3]`
    ///
    /// Invert output B
    #[inline(always)]
    pub const fn b_inv(&self) -> bool {
        (self.bits & Ch5Csr::B_INV_MASK) != 0
    }

    /// Read **A_INV** `[2:2]`
    ///
    /// Invert output A
    #[inline(always)]
    pub const fn a_inv(&self) -> bool {
        (self.bits & Ch5Csr::A_INV_MASK) != 0
    }

    /// Read **PH_CORRECT** `[1:1]`
    ///
    /// 1: Enable phase-correct modulation. 0: Trailing-edge
    #[inline(always)]
    pub const fn ph_correct(&self) -> bool {
        (self.bits & Ch5Csr::PH_CORRECT_MASK) != 0
    }

    /// Read **EN** `[0:0]`
    ///
    /// Enable the PWM channel.
    #[inline(always)]
    pub const fn en(&self) -> bool {
        (self.bits & Ch5Csr::EN_MASK) != 0
    }

    /// Set **PH_ADV** `[7:7]`
    ///
    /// Advance the phase of the counter by 1 count, while it is running.
    /// Self-clearing. Write a 1, and poll until low. Counter must be running
    /// at less than full speed (div_int + div_frac / 16 > 1)
    #[inline(always)]
    pub const fn with_ph_adv(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch5Csr::PH_ADV_MASK
            } else {
                self.bits & !Ch5Csr::PH_ADV_MASK
            }
        }
    }

    /// Set **PH_RET** `[6:6]`
    ///
    /// Retard the phase of the counter by 1 count, while it is running.
    /// Self-clearing. Write a 1, and poll until low. Counter must be running.
    #[inline(always)]
    pub const fn with_ph_ret(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch5Csr::PH_RET_MASK
            } else {
                self.bits & !Ch5Csr::PH_RET_MASK
            }
        }
    }

    /// Set **DIVMODE** `[5:4]`
    #[inline(always)]
    pub const fn with_divmode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch5Csr::DIVMODE_MASK) | ((value << Ch5Csr::DIVMODE_OFFSET) & Ch5Csr::DIVMODE_MASK)
        }
    }

    /// Set **B_INV** `[3:3]`
    ///
    /// Invert output B
    #[inline(always)]
    pub const fn with_b_inv(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch5Csr::B_INV_MASK
            } else {
                self.bits & !Ch5Csr::B_INV_MASK
            }
        }
    }

    /// Set **A_INV** `[2:2]`
    ///
    /// Invert output A
    #[inline(always)]
    pub const fn with_a_inv(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch5Csr::A_INV_MASK
            } else {
                self.bits & !Ch5Csr::A_INV_MASK
            }
        }
    }

    /// Set **PH_CORRECT** `[1:1]`
    ///
    /// 1: Enable phase-correct modulation. 0: Trailing-edge
    #[inline(always)]
    pub const fn with_ph_correct(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch5Csr::PH_CORRECT_MASK
            } else {
                self.bits & !Ch5Csr::PH_CORRECT_MASK
            }
        }
    }

    /// Set **EN** `[0:0]`
    ///
    /// Enable the PWM channel.
    #[inline(always)]
    pub const fn with_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch5Csr::EN_MASK
            } else {
                self.bits & !Ch5Csr::EN_MASK
            }
        }
    }

}

impl Default for Ch5CsrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ch5Csr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch5CsrVal {
        Ch5CsrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch5CsrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch5CsrVal) -> Ch5CsrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// CH5_DIV Register
///
/// INT and FRAC form a fixed-point fractional number.
/// Counting rate is system clock frequency divided by this number.
/// Fractional division uses simple 1st-order sigma-delta.
///
/// Address offset: 0x0068
/// Reset value: 0x00000010
///
/// ## Fields
///
/// - **INT** `[11:4]` (RW)
///
/// - **FRAC** `[3:0]` (RW)
#[repr(transparent)]
pub struct Ch5Div {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch5Div {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch5Div {
    // ======== Field Constants ========

    /// **INT**
    /// Bits `[11:4]`, mask `0x00000ff0`
    pub const INT_OFFSET: u32 = 4;
    pub const INT_MASK: u32 = 0x00000ff0;

    /// **FRAC**
    /// Bits `[3:0]`, mask `0x0000000f`
    pub const FRAC_OFFSET: u32 = 0;
    pub const FRAC_MASK: u32 = 0x0000000f;

    // ======== Field Accessors ========

    /// **INT** `[11:4]` (RW)
    #[inline(always)]
    pub fn int(&self) -> u32 {
        (self.value.read_bits() & Self::INT_MASK) >> Self::INT_OFFSET
    }

    /// Write **INT** `[11:4]` (RW)
    #[inline(always)]
    pub fn write_int(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::INT_MASK) | ((value << Self::INT_OFFSET) & Self::INT_MASK));
    }

    /// **FRAC** `[3:0]` (RW)
    #[inline(always)]
    pub fn frac(&self) -> u32 {
        (self.value.read_bits() & Self::FRAC_MASK) >> Self::FRAC_OFFSET
    }

    /// Write **FRAC** `[3:0]` (RW)
    #[inline(always)]
    pub fn write_frac(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::FRAC_MASK) | ((value << Self::FRAC_OFFSET) & Self::FRAC_MASK));
    }

}

/// CH5_DIV value type for batch read/modify/write operations
///
/// Use `ch5div.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch5div.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch5DivVal {
    bits: u32,
}

impl Ch5DivVal {
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
        Self { bits: 0x00000010 }
    }

    /// Read **INT** `[11:4]`
    #[inline(always)]
    pub const fn int(&self) -> u32 {
        (self.bits & Ch5Div::INT_MASK) >> Ch5Div::INT_OFFSET
    }

    /// Read **FRAC** `[3:0]`
    #[inline(always)]
    pub const fn frac(&self) -> u32 {
        (self.bits & Ch5Div::FRAC_MASK) >> Ch5Div::FRAC_OFFSET
    }

    /// Set **INT** `[11:4]`
    #[inline(always)]
    pub const fn with_int(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch5Div::INT_MASK) | ((value << Ch5Div::INT_OFFSET) & Ch5Div::INT_MASK)
        }
    }

    /// Set **FRAC** `[3:0]`
    #[inline(always)]
    pub const fn with_frac(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch5Div::FRAC_MASK) | ((value << Ch5Div::FRAC_OFFSET) & Ch5Div::FRAC_MASK)
        }
    }

}

impl Default for Ch5DivVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ch5Div {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch5DivVal {
        Ch5DivVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch5DivVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch5DivVal) -> Ch5DivVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// CH5_CTR Register
///
/// Direct access to the PWM counter
///
/// Address offset: 0x006c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CH5_CTR** `[15:0]` (RW)
#[repr(transparent)]
pub struct Ch5Ctr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch5Ctr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch5Ctr {
    // ======== Field Constants ========

    /// **CH5_CTR**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const CH5_CTR_OFFSET: u32 = 0;
    pub const CH5_CTR_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **CH5_CTR** `[15:0]` (RW)
    #[inline(always)]
    pub fn ch5_ctr(&self) -> u32 {
        (self.value.read_bits() & Self::CH5_CTR_MASK) >> Self::CH5_CTR_OFFSET
    }

    /// Write **CH5_CTR** `[15:0]` (RW)
    #[inline(always)]
    pub fn write_ch5_ctr(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::CH5_CTR_MASK) | ((value << Self::CH5_CTR_OFFSET) & Self::CH5_CTR_MASK));
    }

}

/// CH5_CTR value type for batch read/modify/write operations
///
/// Use `ch5ctr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch5ctr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch5CtrVal {
    bits: u32,
}

impl Ch5CtrVal {
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

    /// Read **CH5_CTR** `[15:0]`
    #[inline(always)]
    pub const fn ch5_ctr(&self) -> u32 {
        (self.bits & Ch5Ctr::CH5_CTR_MASK) >> Ch5Ctr::CH5_CTR_OFFSET
    }

    /// Set **CH5_CTR** `[15:0]`
    #[inline(always)]
    pub const fn with_ch5_ctr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch5Ctr::CH5_CTR_MASK) | ((value << Ch5Ctr::CH5_CTR_OFFSET) & Ch5Ctr::CH5_CTR_MASK)
        }
    }

}

impl Default for Ch5CtrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ch5Ctr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch5CtrVal {
        Ch5CtrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch5CtrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch5CtrVal) -> Ch5CtrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// CH5_CC Register
///
/// Counter compare values
///
/// Address offset: 0x0070
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **B** `[31:16]` (RW)
///
/// - **A** `[15:0]` (RW)
#[repr(transparent)]
pub struct Ch5Cc {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch5Cc {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch5Cc {
    // ======== Field Constants ========

    /// **B**
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const B_OFFSET: u32 = 16;
    pub const B_MASK: u32 = 0xffff0000;

    /// **A**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const A_OFFSET: u32 = 0;
    pub const A_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **B** `[31:16]` (RW)
    #[inline(always)]
    pub fn b(&self) -> u32 {
        (self.value.read_bits() & Self::B_MASK) >> Self::B_OFFSET
    }

    /// Write **B** `[31:16]` (RW)
    #[inline(always)]
    pub fn write_b(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::B_MASK) | ((value << Self::B_OFFSET) & Self::B_MASK));
    }

    /// **A** `[15:0]` (RW)
    #[inline(always)]
    pub fn a(&self) -> u32 {
        (self.value.read_bits() & Self::A_MASK) >> Self::A_OFFSET
    }

    /// Write **A** `[15:0]` (RW)
    #[inline(always)]
    pub fn write_a(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::A_MASK) | ((value << Self::A_OFFSET) & Self::A_MASK));
    }

}

/// CH5_CC value type for batch read/modify/write operations
///
/// Use `ch5cc.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch5cc.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch5CcVal {
    bits: u32,
}

impl Ch5CcVal {
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

    /// Read **B** `[31:16]`
    #[inline(always)]
    pub const fn b(&self) -> u32 {
        (self.bits & Ch5Cc::B_MASK) >> Ch5Cc::B_OFFSET
    }

    /// Read **A** `[15:0]`
    #[inline(always)]
    pub const fn a(&self) -> u32 {
        (self.bits & Ch5Cc::A_MASK) >> Ch5Cc::A_OFFSET
    }

    /// Set **B** `[31:16]`
    #[inline(always)]
    pub const fn with_b(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch5Cc::B_MASK) | ((value << Ch5Cc::B_OFFSET) & Ch5Cc::B_MASK)
        }
    }

    /// Set **A** `[15:0]`
    #[inline(always)]
    pub const fn with_a(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch5Cc::A_MASK) | ((value << Ch5Cc::A_OFFSET) & Ch5Cc::A_MASK)
        }
    }

}

impl Default for Ch5CcVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ch5Cc {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch5CcVal {
        Ch5CcVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch5CcVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch5CcVal) -> Ch5CcVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// CH5_TOP Register
///
/// Counter wrap value
///
/// Address offset: 0x0074
/// Reset value: 0x0000ffff
///
/// ## Fields
///
/// - **CH5_TOP** `[15:0]` (RW)
#[repr(transparent)]
pub struct Ch5Top {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch5Top {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch5Top {
    // ======== Field Constants ========

    /// **CH5_TOP**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const CH5_TOP_OFFSET: u32 = 0;
    pub const CH5_TOP_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **CH5_TOP** `[15:0]` (RW)
    #[inline(always)]
    pub fn ch5_top(&self) -> u32 {
        (self.value.read_bits() & Self::CH5_TOP_MASK) >> Self::CH5_TOP_OFFSET
    }

    /// Write **CH5_TOP** `[15:0]` (RW)
    #[inline(always)]
    pub fn write_ch5_top(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::CH5_TOP_MASK) | ((value << Self::CH5_TOP_OFFSET) & Self::CH5_TOP_MASK));
    }

}

/// CH5_TOP value type for batch read/modify/write operations
///
/// Use `ch5top.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch5top.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch5TopVal {
    bits: u32,
}

impl Ch5TopVal {
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
        Self { bits: 0x0000ffff }
    }

    /// Read **CH5_TOP** `[15:0]`
    #[inline(always)]
    pub const fn ch5_top(&self) -> u32 {
        (self.bits & Ch5Top::CH5_TOP_MASK) >> Ch5Top::CH5_TOP_OFFSET
    }

    /// Set **CH5_TOP** `[15:0]`
    #[inline(always)]
    pub const fn with_ch5_top(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch5Top::CH5_TOP_MASK) | ((value << Ch5Top::CH5_TOP_OFFSET) & Ch5Top::CH5_TOP_MASK)
        }
    }

}

impl Default for Ch5TopVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ch5Top {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch5TopVal {
        Ch5TopVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch5TopVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch5TopVal) -> Ch5TopVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// CH6_CSR Register
///
/// Control and status register
///
/// Address offset: 0x0078
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **PH_ADV** `[7]` (WO)
///
///   Advance the phase of the counter by 1 count, while it is running.
///   Self-clearing. Write a 1, and poll until low. Counter must be running
///   at less than full speed (div_int + div_frac / 16 > 1)
///
/// - **PH_RET** `[6]` (WO)
///
///   Retard the phase of the counter by 1 count, while it is running.
///   Self-clearing. Write a 1, and poll until low. Counter must be running.
///
/// - **DIVMODE** `[5:4]` (RW)
///
/// - **B_INV** `[3]` (RW)
///
///   Invert output B
///
/// - **A_INV** `[2]` (RW)
///
///   Invert output A
///
/// - **PH_CORRECT** `[1]` (RW)
///
///   1: Enable phase-correct modulation. 0: Trailing-edge
///
/// - **EN** `[0]` (RW)
///
///   Enable the PWM channel.
#[repr(transparent)]
pub struct Ch6Csr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch6Csr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch6Csr {
    // ======== Field Constants ========

    /// **PH_ADV**
    ///
    /// Advance the phase of the counter by 1 count, while it is running.
    /// Self-clearing. Write a 1, and poll until low. Counter must be running
    /// at less than full speed (div_int + div_frac / 16 > 1)
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const PH_ADV_OFFSET: u32 = 7;
    pub const PH_ADV_MASK: u32 = 0x00000080;
    pub const PH_ADV: u32 = 1 << 7;

    /// **PH_RET**
    ///
    /// Retard the phase of the counter by 1 count, while it is running.
    /// Self-clearing. Write a 1, and poll until low. Counter must be running.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const PH_RET_OFFSET: u32 = 6;
    pub const PH_RET_MASK: u32 = 0x00000040;
    pub const PH_RET: u32 = 1 << 6;

    /// **DIVMODE**
    /// Bits `[5:4]`, mask `0x00000030`
    pub const DIVMODE_OFFSET: u32 = 4;
    pub const DIVMODE_MASK: u32 = 0x00000030;
    /// div = 0
    /// Free-running counting at rate dictated by fractional divider
    pub const DIVMODE_DIV: u32 = 0 << 4;
    pub const DIVMODE_DIV_VAL: u32 = 0;
    /// level = 1
    /// Fractional divider operation is gated by the PWM B pin.
    pub const DIVMODE_LEVEL: u32 = 1 << 4;
    pub const DIVMODE_LEVEL_VAL: u32 = 1;
    /// rise = 2
    /// Counter advances with each rising edge of the PWM B pin.
    pub const DIVMODE_RISE: u32 = 2 << 4;
    pub const DIVMODE_RISE_VAL: u32 = 2;
    /// fall = 3
    /// Counter advances with each falling edge of the PWM B pin.
    pub const DIVMODE_FALL: u32 = 3 << 4;
    pub const DIVMODE_FALL_VAL: u32 = 3;

    /// **B_INV**
    ///
    /// Invert output B
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const B_INV_OFFSET: u32 = 3;
    pub const B_INV_MASK: u32 = 0x00000008;
    pub const B_INV: u32 = 1 << 3;

    /// **A_INV**
    ///
    /// Invert output A
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const A_INV_OFFSET: u32 = 2;
    pub const A_INV_MASK: u32 = 0x00000004;
    pub const A_INV: u32 = 1 << 2;

    /// **PH_CORRECT**
    ///
    /// 1: Enable phase-correct modulation. 0: Trailing-edge
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const PH_CORRECT_OFFSET: u32 = 1;
    pub const PH_CORRECT_MASK: u32 = 0x00000002;
    pub const PH_CORRECT: u32 = 1 << 1;

    /// **EN**
    ///
    /// Enable the PWM channel.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const EN_OFFSET: u32 = 0;
    pub const EN_MASK: u32 = 0x00000001;
    pub const EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **PH_ADV** (WO)
    ///
    /// Advance the phase of the counter by 1 count, while it is running.
    /// Self-clearing. Write a 1, and poll until low. Counter must be running
    /// at less than full speed (div_int + div_frac / 16 > 1)
    #[inline(always)]
    pub fn trigger_ph_adv(&self) {
        self.value.write_bits(Self::PH_ADV_MASK);
    }

    /// Trigger **PH_RET** (WO)
    ///
    /// Retard the phase of the counter by 1 count, while it is running.
    /// Self-clearing. Write a 1, and poll until low. Counter must be running.
    #[inline(always)]
    pub fn trigger_ph_ret(&self) {
        self.value.write_bits(Self::PH_RET_MASK);
    }

    /// **DIVMODE** `[5:4]` (RW)
    #[inline(always)]
    pub fn divmode(&self) -> u32 {
        (self.value.read_bits() & Self::DIVMODE_MASK) >> Self::DIVMODE_OFFSET
    }

    /// Write **DIVMODE** `[5:4]` (RW)
    #[inline(always)]
    pub fn write_divmode(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DIVMODE_MASK) | ((value << Self::DIVMODE_OFFSET) & Self::DIVMODE_MASK));
    }

    /// **B_INV** `[3]` (RW)
    ///
    /// Invert output B
    #[inline(always)]
    pub fn b_inv(&self) -> bool {
        (self.value.read_bits() & Self::B_INV_MASK) != 0
    }

    /// Set B_INV bit atomically (RW)
    #[inline(always)]
    pub fn set_b_inv(&self) {
        self.value.set_bits(Self::B_INV_MASK);
    }

    /// Clear B_INV bit atomically (RW)
    #[inline(always)]
    pub fn clr_b_inv(&self) {
        self.value.clr_bits(Self::B_INV_MASK);
    }

    /// **A_INV** `[2]` (RW)
    ///
    /// Invert output A
    #[inline(always)]
    pub fn a_inv(&self) -> bool {
        (self.value.read_bits() & Self::A_INV_MASK) != 0
    }

    /// Set A_INV bit atomically (RW)
    #[inline(always)]
    pub fn set_a_inv(&self) {
        self.value.set_bits(Self::A_INV_MASK);
    }

    /// Clear A_INV bit atomically (RW)
    #[inline(always)]
    pub fn clr_a_inv(&self) {
        self.value.clr_bits(Self::A_INV_MASK);
    }

    /// **PH_CORRECT** `[1]` (RW)
    ///
    /// 1: Enable phase-correct modulation. 0: Trailing-edge
    #[inline(always)]
    pub fn ph_correct(&self) -> bool {
        (self.value.read_bits() & Self::PH_CORRECT_MASK) != 0
    }

    /// Set PH_CORRECT bit atomically (RW)
    #[inline(always)]
    pub fn set_ph_correct(&self) {
        self.value.set_bits(Self::PH_CORRECT_MASK);
    }

    /// Clear PH_CORRECT bit atomically (RW)
    #[inline(always)]
    pub fn clr_ph_correct(&self) {
        self.value.clr_bits(Self::PH_CORRECT_MASK);
    }

    /// **EN** `[0]` (RW)
    ///
    /// Enable the PWM channel.
    #[inline(always)]
    pub fn en(&self) -> bool {
        (self.value.read_bits() & Self::EN_MASK) != 0
    }

    /// Set EN bit atomically (RW)
    #[inline(always)]
    pub fn set_en(&self) {
        self.value.set_bits(Self::EN_MASK);
    }

    /// Clear EN bit atomically (RW)
    #[inline(always)]
    pub fn clr_en(&self) {
        self.value.clr_bits(Self::EN_MASK);
    }

}

/// CH6_CSR value type for batch read/modify/write operations
///
/// Use `ch6csr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch6csr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch6CsrVal {
    bits: u32,
}

impl Ch6CsrVal {
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

    /// Read **DIVMODE** `[5:4]`
    #[inline(always)]
    pub const fn divmode(&self) -> u32 {
        (self.bits & Ch6Csr::DIVMODE_MASK) >> Ch6Csr::DIVMODE_OFFSET
    }

    /// Read **B_INV** `[3:3]`
    ///
    /// Invert output B
    #[inline(always)]
    pub const fn b_inv(&self) -> bool {
        (self.bits & Ch6Csr::B_INV_MASK) != 0
    }

    /// Read **A_INV** `[2:2]`
    ///
    /// Invert output A
    #[inline(always)]
    pub const fn a_inv(&self) -> bool {
        (self.bits & Ch6Csr::A_INV_MASK) != 0
    }

    /// Read **PH_CORRECT** `[1:1]`
    ///
    /// 1: Enable phase-correct modulation. 0: Trailing-edge
    #[inline(always)]
    pub const fn ph_correct(&self) -> bool {
        (self.bits & Ch6Csr::PH_CORRECT_MASK) != 0
    }

    /// Read **EN** `[0:0]`
    ///
    /// Enable the PWM channel.
    #[inline(always)]
    pub const fn en(&self) -> bool {
        (self.bits & Ch6Csr::EN_MASK) != 0
    }

    /// Set **PH_ADV** `[7:7]`
    ///
    /// Advance the phase of the counter by 1 count, while it is running.
    /// Self-clearing. Write a 1, and poll until low. Counter must be running
    /// at less than full speed (div_int + div_frac / 16 > 1)
    #[inline(always)]
    pub const fn with_ph_adv(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch6Csr::PH_ADV_MASK
            } else {
                self.bits & !Ch6Csr::PH_ADV_MASK
            }
        }
    }

    /// Set **PH_RET** `[6:6]`
    ///
    /// Retard the phase of the counter by 1 count, while it is running.
    /// Self-clearing. Write a 1, and poll until low. Counter must be running.
    #[inline(always)]
    pub const fn with_ph_ret(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch6Csr::PH_RET_MASK
            } else {
                self.bits & !Ch6Csr::PH_RET_MASK
            }
        }
    }

    /// Set **DIVMODE** `[5:4]`
    #[inline(always)]
    pub const fn with_divmode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch6Csr::DIVMODE_MASK) | ((value << Ch6Csr::DIVMODE_OFFSET) & Ch6Csr::DIVMODE_MASK)
        }
    }

    /// Set **B_INV** `[3:3]`
    ///
    /// Invert output B
    #[inline(always)]
    pub const fn with_b_inv(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch6Csr::B_INV_MASK
            } else {
                self.bits & !Ch6Csr::B_INV_MASK
            }
        }
    }

    /// Set **A_INV** `[2:2]`
    ///
    /// Invert output A
    #[inline(always)]
    pub const fn with_a_inv(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch6Csr::A_INV_MASK
            } else {
                self.bits & !Ch6Csr::A_INV_MASK
            }
        }
    }

    /// Set **PH_CORRECT** `[1:1]`
    ///
    /// 1: Enable phase-correct modulation. 0: Trailing-edge
    #[inline(always)]
    pub const fn with_ph_correct(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch6Csr::PH_CORRECT_MASK
            } else {
                self.bits & !Ch6Csr::PH_CORRECT_MASK
            }
        }
    }

    /// Set **EN** `[0:0]`
    ///
    /// Enable the PWM channel.
    #[inline(always)]
    pub const fn with_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch6Csr::EN_MASK
            } else {
                self.bits & !Ch6Csr::EN_MASK
            }
        }
    }

}

impl Default for Ch6CsrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ch6Csr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch6CsrVal {
        Ch6CsrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch6CsrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch6CsrVal) -> Ch6CsrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// CH6_DIV Register
///
/// INT and FRAC form a fixed-point fractional number.
/// Counting rate is system clock frequency divided by this number.
/// Fractional division uses simple 1st-order sigma-delta.
///
/// Address offset: 0x007c
/// Reset value: 0x00000010
///
/// ## Fields
///
/// - **INT** `[11:4]` (RW)
///
/// - **FRAC** `[3:0]` (RW)
#[repr(transparent)]
pub struct Ch6Div {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch6Div {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch6Div {
    // ======== Field Constants ========

    /// **INT**
    /// Bits `[11:4]`, mask `0x00000ff0`
    pub const INT_OFFSET: u32 = 4;
    pub const INT_MASK: u32 = 0x00000ff0;

    /// **FRAC**
    /// Bits `[3:0]`, mask `0x0000000f`
    pub const FRAC_OFFSET: u32 = 0;
    pub const FRAC_MASK: u32 = 0x0000000f;

    // ======== Field Accessors ========

    /// **INT** `[11:4]` (RW)
    #[inline(always)]
    pub fn int(&self) -> u32 {
        (self.value.read_bits() & Self::INT_MASK) >> Self::INT_OFFSET
    }

    /// Write **INT** `[11:4]` (RW)
    #[inline(always)]
    pub fn write_int(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::INT_MASK) | ((value << Self::INT_OFFSET) & Self::INT_MASK));
    }

    /// **FRAC** `[3:0]` (RW)
    #[inline(always)]
    pub fn frac(&self) -> u32 {
        (self.value.read_bits() & Self::FRAC_MASK) >> Self::FRAC_OFFSET
    }

    /// Write **FRAC** `[3:0]` (RW)
    #[inline(always)]
    pub fn write_frac(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::FRAC_MASK) | ((value << Self::FRAC_OFFSET) & Self::FRAC_MASK));
    }

}

/// CH6_DIV value type for batch read/modify/write operations
///
/// Use `ch6div.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch6div.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch6DivVal {
    bits: u32,
}

impl Ch6DivVal {
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
        Self { bits: 0x00000010 }
    }

    /// Read **INT** `[11:4]`
    #[inline(always)]
    pub const fn int(&self) -> u32 {
        (self.bits & Ch6Div::INT_MASK) >> Ch6Div::INT_OFFSET
    }

    /// Read **FRAC** `[3:0]`
    #[inline(always)]
    pub const fn frac(&self) -> u32 {
        (self.bits & Ch6Div::FRAC_MASK) >> Ch6Div::FRAC_OFFSET
    }

    /// Set **INT** `[11:4]`
    #[inline(always)]
    pub const fn with_int(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch6Div::INT_MASK) | ((value << Ch6Div::INT_OFFSET) & Ch6Div::INT_MASK)
        }
    }

    /// Set **FRAC** `[3:0]`
    #[inline(always)]
    pub const fn with_frac(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch6Div::FRAC_MASK) | ((value << Ch6Div::FRAC_OFFSET) & Ch6Div::FRAC_MASK)
        }
    }

}

impl Default for Ch6DivVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ch6Div {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch6DivVal {
        Ch6DivVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch6DivVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch6DivVal) -> Ch6DivVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// CH6_CTR Register
///
/// Direct access to the PWM counter
///
/// Address offset: 0x0080
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CH6_CTR** `[15:0]` (RW)
#[repr(transparent)]
pub struct Ch6Ctr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch6Ctr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch6Ctr {
    // ======== Field Constants ========

    /// **CH6_CTR**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const CH6_CTR_OFFSET: u32 = 0;
    pub const CH6_CTR_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **CH6_CTR** `[15:0]` (RW)
    #[inline(always)]
    pub fn ch6_ctr(&self) -> u32 {
        (self.value.read_bits() & Self::CH6_CTR_MASK) >> Self::CH6_CTR_OFFSET
    }

    /// Write **CH6_CTR** `[15:0]` (RW)
    #[inline(always)]
    pub fn write_ch6_ctr(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::CH6_CTR_MASK) | ((value << Self::CH6_CTR_OFFSET) & Self::CH6_CTR_MASK));
    }

}

/// CH6_CTR value type for batch read/modify/write operations
///
/// Use `ch6ctr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch6ctr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch6CtrVal {
    bits: u32,
}

impl Ch6CtrVal {
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

    /// Read **CH6_CTR** `[15:0]`
    #[inline(always)]
    pub const fn ch6_ctr(&self) -> u32 {
        (self.bits & Ch6Ctr::CH6_CTR_MASK) >> Ch6Ctr::CH6_CTR_OFFSET
    }

    /// Set **CH6_CTR** `[15:0]`
    #[inline(always)]
    pub const fn with_ch6_ctr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch6Ctr::CH6_CTR_MASK) | ((value << Ch6Ctr::CH6_CTR_OFFSET) & Ch6Ctr::CH6_CTR_MASK)
        }
    }

}

impl Default for Ch6CtrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ch6Ctr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch6CtrVal {
        Ch6CtrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch6CtrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch6CtrVal) -> Ch6CtrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// CH6_CC Register
///
/// Counter compare values
///
/// Address offset: 0x0084
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **B** `[31:16]` (RW)
///
/// - **A** `[15:0]` (RW)
#[repr(transparent)]
pub struct Ch6Cc {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch6Cc {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch6Cc {
    // ======== Field Constants ========

    /// **B**
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const B_OFFSET: u32 = 16;
    pub const B_MASK: u32 = 0xffff0000;

    /// **A**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const A_OFFSET: u32 = 0;
    pub const A_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **B** `[31:16]` (RW)
    #[inline(always)]
    pub fn b(&self) -> u32 {
        (self.value.read_bits() & Self::B_MASK) >> Self::B_OFFSET
    }

    /// Write **B** `[31:16]` (RW)
    #[inline(always)]
    pub fn write_b(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::B_MASK) | ((value << Self::B_OFFSET) & Self::B_MASK));
    }

    /// **A** `[15:0]` (RW)
    #[inline(always)]
    pub fn a(&self) -> u32 {
        (self.value.read_bits() & Self::A_MASK) >> Self::A_OFFSET
    }

    /// Write **A** `[15:0]` (RW)
    #[inline(always)]
    pub fn write_a(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::A_MASK) | ((value << Self::A_OFFSET) & Self::A_MASK));
    }

}

/// CH6_CC value type for batch read/modify/write operations
///
/// Use `ch6cc.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch6cc.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch6CcVal {
    bits: u32,
}

impl Ch6CcVal {
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

    /// Read **B** `[31:16]`
    #[inline(always)]
    pub const fn b(&self) -> u32 {
        (self.bits & Ch6Cc::B_MASK) >> Ch6Cc::B_OFFSET
    }

    /// Read **A** `[15:0]`
    #[inline(always)]
    pub const fn a(&self) -> u32 {
        (self.bits & Ch6Cc::A_MASK) >> Ch6Cc::A_OFFSET
    }

    /// Set **B** `[31:16]`
    #[inline(always)]
    pub const fn with_b(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch6Cc::B_MASK) | ((value << Ch6Cc::B_OFFSET) & Ch6Cc::B_MASK)
        }
    }

    /// Set **A** `[15:0]`
    #[inline(always)]
    pub const fn with_a(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch6Cc::A_MASK) | ((value << Ch6Cc::A_OFFSET) & Ch6Cc::A_MASK)
        }
    }

}

impl Default for Ch6CcVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ch6Cc {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch6CcVal {
        Ch6CcVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch6CcVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch6CcVal) -> Ch6CcVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// CH6_TOP Register
///
/// Counter wrap value
///
/// Address offset: 0x0088
/// Reset value: 0x0000ffff
///
/// ## Fields
///
/// - **CH6_TOP** `[15:0]` (RW)
#[repr(transparent)]
pub struct Ch6Top {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch6Top {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch6Top {
    // ======== Field Constants ========

    /// **CH6_TOP**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const CH6_TOP_OFFSET: u32 = 0;
    pub const CH6_TOP_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **CH6_TOP** `[15:0]` (RW)
    #[inline(always)]
    pub fn ch6_top(&self) -> u32 {
        (self.value.read_bits() & Self::CH6_TOP_MASK) >> Self::CH6_TOP_OFFSET
    }

    /// Write **CH6_TOP** `[15:0]` (RW)
    #[inline(always)]
    pub fn write_ch6_top(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::CH6_TOP_MASK) | ((value << Self::CH6_TOP_OFFSET) & Self::CH6_TOP_MASK));
    }

}

/// CH6_TOP value type for batch read/modify/write operations
///
/// Use `ch6top.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch6top.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch6TopVal {
    bits: u32,
}

impl Ch6TopVal {
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
        Self { bits: 0x0000ffff }
    }

    /// Read **CH6_TOP** `[15:0]`
    #[inline(always)]
    pub const fn ch6_top(&self) -> u32 {
        (self.bits & Ch6Top::CH6_TOP_MASK) >> Ch6Top::CH6_TOP_OFFSET
    }

    /// Set **CH6_TOP** `[15:0]`
    #[inline(always)]
    pub const fn with_ch6_top(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch6Top::CH6_TOP_MASK) | ((value << Ch6Top::CH6_TOP_OFFSET) & Ch6Top::CH6_TOP_MASK)
        }
    }

}

impl Default for Ch6TopVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ch6Top {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch6TopVal {
        Ch6TopVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch6TopVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch6TopVal) -> Ch6TopVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// CH7_CSR Register
///
/// Control and status register
///
/// Address offset: 0x008c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **PH_ADV** `[7]` (WO)
///
///   Advance the phase of the counter by 1 count, while it is running.
///   Self-clearing. Write a 1, and poll until low. Counter must be running
///   at less than full speed (div_int + div_frac / 16 > 1)
///
/// - **PH_RET** `[6]` (WO)
///
///   Retard the phase of the counter by 1 count, while it is running.
///   Self-clearing. Write a 1, and poll until low. Counter must be running.
///
/// - **DIVMODE** `[5:4]` (RW)
///
/// - **B_INV** `[3]` (RW)
///
///   Invert output B
///
/// - **A_INV** `[2]` (RW)
///
///   Invert output A
///
/// - **PH_CORRECT** `[1]` (RW)
///
///   1: Enable phase-correct modulation. 0: Trailing-edge
///
/// - **EN** `[0]` (RW)
///
///   Enable the PWM channel.
#[repr(transparent)]
pub struct Ch7Csr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch7Csr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch7Csr {
    // ======== Field Constants ========

    /// **PH_ADV**
    ///
    /// Advance the phase of the counter by 1 count, while it is running.
    /// Self-clearing. Write a 1, and poll until low. Counter must be running
    /// at less than full speed (div_int + div_frac / 16 > 1)
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const PH_ADV_OFFSET: u32 = 7;
    pub const PH_ADV_MASK: u32 = 0x00000080;
    pub const PH_ADV: u32 = 1 << 7;

    /// **PH_RET**
    ///
    /// Retard the phase of the counter by 1 count, while it is running.
    /// Self-clearing. Write a 1, and poll until low. Counter must be running.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const PH_RET_OFFSET: u32 = 6;
    pub const PH_RET_MASK: u32 = 0x00000040;
    pub const PH_RET: u32 = 1 << 6;

    /// **DIVMODE**
    /// Bits `[5:4]`, mask `0x00000030`
    pub const DIVMODE_OFFSET: u32 = 4;
    pub const DIVMODE_MASK: u32 = 0x00000030;
    /// div = 0
    /// Free-running counting at rate dictated by fractional divider
    pub const DIVMODE_DIV: u32 = 0 << 4;
    pub const DIVMODE_DIV_VAL: u32 = 0;
    /// level = 1
    /// Fractional divider operation is gated by the PWM B pin.
    pub const DIVMODE_LEVEL: u32 = 1 << 4;
    pub const DIVMODE_LEVEL_VAL: u32 = 1;
    /// rise = 2
    /// Counter advances with each rising edge of the PWM B pin.
    pub const DIVMODE_RISE: u32 = 2 << 4;
    pub const DIVMODE_RISE_VAL: u32 = 2;
    /// fall = 3
    /// Counter advances with each falling edge of the PWM B pin.
    pub const DIVMODE_FALL: u32 = 3 << 4;
    pub const DIVMODE_FALL_VAL: u32 = 3;

    /// **B_INV**
    ///
    /// Invert output B
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const B_INV_OFFSET: u32 = 3;
    pub const B_INV_MASK: u32 = 0x00000008;
    pub const B_INV: u32 = 1 << 3;

    /// **A_INV**
    ///
    /// Invert output A
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const A_INV_OFFSET: u32 = 2;
    pub const A_INV_MASK: u32 = 0x00000004;
    pub const A_INV: u32 = 1 << 2;

    /// **PH_CORRECT**
    ///
    /// 1: Enable phase-correct modulation. 0: Trailing-edge
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const PH_CORRECT_OFFSET: u32 = 1;
    pub const PH_CORRECT_MASK: u32 = 0x00000002;
    pub const PH_CORRECT: u32 = 1 << 1;

    /// **EN**
    ///
    /// Enable the PWM channel.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const EN_OFFSET: u32 = 0;
    pub const EN_MASK: u32 = 0x00000001;
    pub const EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **PH_ADV** (WO)
    ///
    /// Advance the phase of the counter by 1 count, while it is running.
    /// Self-clearing. Write a 1, and poll until low. Counter must be running
    /// at less than full speed (div_int + div_frac / 16 > 1)
    #[inline(always)]
    pub fn trigger_ph_adv(&self) {
        self.value.write_bits(Self::PH_ADV_MASK);
    }

    /// Trigger **PH_RET** (WO)
    ///
    /// Retard the phase of the counter by 1 count, while it is running.
    /// Self-clearing. Write a 1, and poll until low. Counter must be running.
    #[inline(always)]
    pub fn trigger_ph_ret(&self) {
        self.value.write_bits(Self::PH_RET_MASK);
    }

    /// **DIVMODE** `[5:4]` (RW)
    #[inline(always)]
    pub fn divmode(&self) -> u32 {
        (self.value.read_bits() & Self::DIVMODE_MASK) >> Self::DIVMODE_OFFSET
    }

    /// Write **DIVMODE** `[5:4]` (RW)
    #[inline(always)]
    pub fn write_divmode(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DIVMODE_MASK) | ((value << Self::DIVMODE_OFFSET) & Self::DIVMODE_MASK));
    }

    /// **B_INV** `[3]` (RW)
    ///
    /// Invert output B
    #[inline(always)]
    pub fn b_inv(&self) -> bool {
        (self.value.read_bits() & Self::B_INV_MASK) != 0
    }

    /// Set B_INV bit atomically (RW)
    #[inline(always)]
    pub fn set_b_inv(&self) {
        self.value.set_bits(Self::B_INV_MASK);
    }

    /// Clear B_INV bit atomically (RW)
    #[inline(always)]
    pub fn clr_b_inv(&self) {
        self.value.clr_bits(Self::B_INV_MASK);
    }

    /// **A_INV** `[2]` (RW)
    ///
    /// Invert output A
    #[inline(always)]
    pub fn a_inv(&self) -> bool {
        (self.value.read_bits() & Self::A_INV_MASK) != 0
    }

    /// Set A_INV bit atomically (RW)
    #[inline(always)]
    pub fn set_a_inv(&self) {
        self.value.set_bits(Self::A_INV_MASK);
    }

    /// Clear A_INV bit atomically (RW)
    #[inline(always)]
    pub fn clr_a_inv(&self) {
        self.value.clr_bits(Self::A_INV_MASK);
    }

    /// **PH_CORRECT** `[1]` (RW)
    ///
    /// 1: Enable phase-correct modulation. 0: Trailing-edge
    #[inline(always)]
    pub fn ph_correct(&self) -> bool {
        (self.value.read_bits() & Self::PH_CORRECT_MASK) != 0
    }

    /// Set PH_CORRECT bit atomically (RW)
    #[inline(always)]
    pub fn set_ph_correct(&self) {
        self.value.set_bits(Self::PH_CORRECT_MASK);
    }

    /// Clear PH_CORRECT bit atomically (RW)
    #[inline(always)]
    pub fn clr_ph_correct(&self) {
        self.value.clr_bits(Self::PH_CORRECT_MASK);
    }

    /// **EN** `[0]` (RW)
    ///
    /// Enable the PWM channel.
    #[inline(always)]
    pub fn en(&self) -> bool {
        (self.value.read_bits() & Self::EN_MASK) != 0
    }

    /// Set EN bit atomically (RW)
    #[inline(always)]
    pub fn set_en(&self) {
        self.value.set_bits(Self::EN_MASK);
    }

    /// Clear EN bit atomically (RW)
    #[inline(always)]
    pub fn clr_en(&self) {
        self.value.clr_bits(Self::EN_MASK);
    }

}

/// CH7_CSR value type for batch read/modify/write operations
///
/// Use `ch7csr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch7csr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch7CsrVal {
    bits: u32,
}

impl Ch7CsrVal {
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

    /// Read **DIVMODE** `[5:4]`
    #[inline(always)]
    pub const fn divmode(&self) -> u32 {
        (self.bits & Ch7Csr::DIVMODE_MASK) >> Ch7Csr::DIVMODE_OFFSET
    }

    /// Read **B_INV** `[3:3]`
    ///
    /// Invert output B
    #[inline(always)]
    pub const fn b_inv(&self) -> bool {
        (self.bits & Ch7Csr::B_INV_MASK) != 0
    }

    /// Read **A_INV** `[2:2]`
    ///
    /// Invert output A
    #[inline(always)]
    pub const fn a_inv(&self) -> bool {
        (self.bits & Ch7Csr::A_INV_MASK) != 0
    }

    /// Read **PH_CORRECT** `[1:1]`
    ///
    /// 1: Enable phase-correct modulation. 0: Trailing-edge
    #[inline(always)]
    pub const fn ph_correct(&self) -> bool {
        (self.bits & Ch7Csr::PH_CORRECT_MASK) != 0
    }

    /// Read **EN** `[0:0]`
    ///
    /// Enable the PWM channel.
    #[inline(always)]
    pub const fn en(&self) -> bool {
        (self.bits & Ch7Csr::EN_MASK) != 0
    }

    /// Set **PH_ADV** `[7:7]`
    ///
    /// Advance the phase of the counter by 1 count, while it is running.
    /// Self-clearing. Write a 1, and poll until low. Counter must be running
    /// at less than full speed (div_int + div_frac / 16 > 1)
    #[inline(always)]
    pub const fn with_ph_adv(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch7Csr::PH_ADV_MASK
            } else {
                self.bits & !Ch7Csr::PH_ADV_MASK
            }
        }
    }

    /// Set **PH_RET** `[6:6]`
    ///
    /// Retard the phase of the counter by 1 count, while it is running.
    /// Self-clearing. Write a 1, and poll until low. Counter must be running.
    #[inline(always)]
    pub const fn with_ph_ret(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch7Csr::PH_RET_MASK
            } else {
                self.bits & !Ch7Csr::PH_RET_MASK
            }
        }
    }

    /// Set **DIVMODE** `[5:4]`
    #[inline(always)]
    pub const fn with_divmode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch7Csr::DIVMODE_MASK) | ((value << Ch7Csr::DIVMODE_OFFSET) & Ch7Csr::DIVMODE_MASK)
        }
    }

    /// Set **B_INV** `[3:3]`
    ///
    /// Invert output B
    #[inline(always)]
    pub const fn with_b_inv(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch7Csr::B_INV_MASK
            } else {
                self.bits & !Ch7Csr::B_INV_MASK
            }
        }
    }

    /// Set **A_INV** `[2:2]`
    ///
    /// Invert output A
    #[inline(always)]
    pub const fn with_a_inv(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch7Csr::A_INV_MASK
            } else {
                self.bits & !Ch7Csr::A_INV_MASK
            }
        }
    }

    /// Set **PH_CORRECT** `[1:1]`
    ///
    /// 1: Enable phase-correct modulation. 0: Trailing-edge
    #[inline(always)]
    pub const fn with_ph_correct(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch7Csr::PH_CORRECT_MASK
            } else {
                self.bits & !Ch7Csr::PH_CORRECT_MASK
            }
        }
    }

    /// Set **EN** `[0:0]`
    ///
    /// Enable the PWM channel.
    #[inline(always)]
    pub const fn with_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch7Csr::EN_MASK
            } else {
                self.bits & !Ch7Csr::EN_MASK
            }
        }
    }

}

impl Default for Ch7CsrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ch7Csr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch7CsrVal {
        Ch7CsrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch7CsrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch7CsrVal) -> Ch7CsrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// CH7_DIV Register
///
/// INT and FRAC form a fixed-point fractional number.
/// Counting rate is system clock frequency divided by this number.
/// Fractional division uses simple 1st-order sigma-delta.
///
/// Address offset: 0x0090
/// Reset value: 0x00000010
///
/// ## Fields
///
/// - **INT** `[11:4]` (RW)
///
/// - **FRAC** `[3:0]` (RW)
#[repr(transparent)]
pub struct Ch7Div {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch7Div {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch7Div {
    // ======== Field Constants ========

    /// **INT**
    /// Bits `[11:4]`, mask `0x00000ff0`
    pub const INT_OFFSET: u32 = 4;
    pub const INT_MASK: u32 = 0x00000ff0;

    /// **FRAC**
    /// Bits `[3:0]`, mask `0x0000000f`
    pub const FRAC_OFFSET: u32 = 0;
    pub const FRAC_MASK: u32 = 0x0000000f;

    // ======== Field Accessors ========

    /// **INT** `[11:4]` (RW)
    #[inline(always)]
    pub fn int(&self) -> u32 {
        (self.value.read_bits() & Self::INT_MASK) >> Self::INT_OFFSET
    }

    /// Write **INT** `[11:4]` (RW)
    #[inline(always)]
    pub fn write_int(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::INT_MASK) | ((value << Self::INT_OFFSET) & Self::INT_MASK));
    }

    /// **FRAC** `[3:0]` (RW)
    #[inline(always)]
    pub fn frac(&self) -> u32 {
        (self.value.read_bits() & Self::FRAC_MASK) >> Self::FRAC_OFFSET
    }

    /// Write **FRAC** `[3:0]` (RW)
    #[inline(always)]
    pub fn write_frac(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::FRAC_MASK) | ((value << Self::FRAC_OFFSET) & Self::FRAC_MASK));
    }

}

/// CH7_DIV value type for batch read/modify/write operations
///
/// Use `ch7div.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch7div.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch7DivVal {
    bits: u32,
}

impl Ch7DivVal {
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
        Self { bits: 0x00000010 }
    }

    /// Read **INT** `[11:4]`
    #[inline(always)]
    pub const fn int(&self) -> u32 {
        (self.bits & Ch7Div::INT_MASK) >> Ch7Div::INT_OFFSET
    }

    /// Read **FRAC** `[3:0]`
    #[inline(always)]
    pub const fn frac(&self) -> u32 {
        (self.bits & Ch7Div::FRAC_MASK) >> Ch7Div::FRAC_OFFSET
    }

    /// Set **INT** `[11:4]`
    #[inline(always)]
    pub const fn with_int(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch7Div::INT_MASK) | ((value << Ch7Div::INT_OFFSET) & Ch7Div::INT_MASK)
        }
    }

    /// Set **FRAC** `[3:0]`
    #[inline(always)]
    pub const fn with_frac(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch7Div::FRAC_MASK) | ((value << Ch7Div::FRAC_OFFSET) & Ch7Div::FRAC_MASK)
        }
    }

}

impl Default for Ch7DivVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ch7Div {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch7DivVal {
        Ch7DivVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch7DivVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch7DivVal) -> Ch7DivVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// CH7_CTR Register
///
/// Direct access to the PWM counter
///
/// Address offset: 0x0094
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CH7_CTR** `[15:0]` (RW)
#[repr(transparent)]
pub struct Ch7Ctr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch7Ctr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch7Ctr {
    // ======== Field Constants ========

    /// **CH7_CTR**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const CH7_CTR_OFFSET: u32 = 0;
    pub const CH7_CTR_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **CH7_CTR** `[15:0]` (RW)
    #[inline(always)]
    pub fn ch7_ctr(&self) -> u32 {
        (self.value.read_bits() & Self::CH7_CTR_MASK) >> Self::CH7_CTR_OFFSET
    }

    /// Write **CH7_CTR** `[15:0]` (RW)
    #[inline(always)]
    pub fn write_ch7_ctr(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::CH7_CTR_MASK) | ((value << Self::CH7_CTR_OFFSET) & Self::CH7_CTR_MASK));
    }

}

/// CH7_CTR value type for batch read/modify/write operations
///
/// Use `ch7ctr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch7ctr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch7CtrVal {
    bits: u32,
}

impl Ch7CtrVal {
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

    /// Read **CH7_CTR** `[15:0]`
    #[inline(always)]
    pub const fn ch7_ctr(&self) -> u32 {
        (self.bits & Ch7Ctr::CH7_CTR_MASK) >> Ch7Ctr::CH7_CTR_OFFSET
    }

    /// Set **CH7_CTR** `[15:0]`
    #[inline(always)]
    pub const fn with_ch7_ctr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch7Ctr::CH7_CTR_MASK) | ((value << Ch7Ctr::CH7_CTR_OFFSET) & Ch7Ctr::CH7_CTR_MASK)
        }
    }

}

impl Default for Ch7CtrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ch7Ctr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch7CtrVal {
        Ch7CtrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch7CtrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch7CtrVal) -> Ch7CtrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// CH7_CC Register
///
/// Counter compare values
///
/// Address offset: 0x0098
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **B** `[31:16]` (RW)
///
/// - **A** `[15:0]` (RW)
#[repr(transparent)]
pub struct Ch7Cc {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch7Cc {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch7Cc {
    // ======== Field Constants ========

    /// **B**
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const B_OFFSET: u32 = 16;
    pub const B_MASK: u32 = 0xffff0000;

    /// **A**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const A_OFFSET: u32 = 0;
    pub const A_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **B** `[31:16]` (RW)
    #[inline(always)]
    pub fn b(&self) -> u32 {
        (self.value.read_bits() & Self::B_MASK) >> Self::B_OFFSET
    }

    /// Write **B** `[31:16]` (RW)
    #[inline(always)]
    pub fn write_b(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::B_MASK) | ((value << Self::B_OFFSET) & Self::B_MASK));
    }

    /// **A** `[15:0]` (RW)
    #[inline(always)]
    pub fn a(&self) -> u32 {
        (self.value.read_bits() & Self::A_MASK) >> Self::A_OFFSET
    }

    /// Write **A** `[15:0]` (RW)
    #[inline(always)]
    pub fn write_a(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::A_MASK) | ((value << Self::A_OFFSET) & Self::A_MASK));
    }

}

/// CH7_CC value type for batch read/modify/write operations
///
/// Use `ch7cc.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch7cc.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch7CcVal {
    bits: u32,
}

impl Ch7CcVal {
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

    /// Read **B** `[31:16]`
    #[inline(always)]
    pub const fn b(&self) -> u32 {
        (self.bits & Ch7Cc::B_MASK) >> Ch7Cc::B_OFFSET
    }

    /// Read **A** `[15:0]`
    #[inline(always)]
    pub const fn a(&self) -> u32 {
        (self.bits & Ch7Cc::A_MASK) >> Ch7Cc::A_OFFSET
    }

    /// Set **B** `[31:16]`
    #[inline(always)]
    pub const fn with_b(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch7Cc::B_MASK) | ((value << Ch7Cc::B_OFFSET) & Ch7Cc::B_MASK)
        }
    }

    /// Set **A** `[15:0]`
    #[inline(always)]
    pub const fn with_a(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch7Cc::A_MASK) | ((value << Ch7Cc::A_OFFSET) & Ch7Cc::A_MASK)
        }
    }

}

impl Default for Ch7CcVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ch7Cc {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch7CcVal {
        Ch7CcVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch7CcVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch7CcVal) -> Ch7CcVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// CH7_TOP Register
///
/// Counter wrap value
///
/// Address offset: 0x009c
/// Reset value: 0x0000ffff
///
/// ## Fields
///
/// - **CH7_TOP** `[15:0]` (RW)
#[repr(transparent)]
pub struct Ch7Top {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch7Top {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch7Top {
    // ======== Field Constants ========

    /// **CH7_TOP**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const CH7_TOP_OFFSET: u32 = 0;
    pub const CH7_TOP_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **CH7_TOP** `[15:0]` (RW)
    #[inline(always)]
    pub fn ch7_top(&self) -> u32 {
        (self.value.read_bits() & Self::CH7_TOP_MASK) >> Self::CH7_TOP_OFFSET
    }

    /// Write **CH7_TOP** `[15:0]` (RW)
    #[inline(always)]
    pub fn write_ch7_top(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::CH7_TOP_MASK) | ((value << Self::CH7_TOP_OFFSET) & Self::CH7_TOP_MASK));
    }

}

/// CH7_TOP value type for batch read/modify/write operations
///
/// Use `ch7top.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch7top.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch7TopVal {
    bits: u32,
}

impl Ch7TopVal {
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
        Self { bits: 0x0000ffff }
    }

    /// Read **CH7_TOP** `[15:0]`
    #[inline(always)]
    pub const fn ch7_top(&self) -> u32 {
        (self.bits & Ch7Top::CH7_TOP_MASK) >> Ch7Top::CH7_TOP_OFFSET
    }

    /// Set **CH7_TOP** `[15:0]`
    #[inline(always)]
    pub const fn with_ch7_top(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch7Top::CH7_TOP_MASK) | ((value << Ch7Top::CH7_TOP_OFFSET) & Ch7Top::CH7_TOP_MASK)
        }
    }

}

impl Default for Ch7TopVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ch7Top {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch7TopVal {
        Ch7TopVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch7TopVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch7TopVal) -> Ch7TopVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// CH8_CSR Register
///
/// Control and status register
///
/// Address offset: 0x00a0
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **PH_ADV** `[7]` (WO)
///
///   Advance the phase of the counter by 1 count, while it is running.
///   Self-clearing. Write a 1, and poll until low. Counter must be running
///   at less than full speed (div_int + div_frac / 16 > 1)
///
/// - **PH_RET** `[6]` (WO)
///
///   Retard the phase of the counter by 1 count, while it is running.
///   Self-clearing. Write a 1, and poll until low. Counter must be running.
///
/// - **DIVMODE** `[5:4]` (RW)
///
/// - **B_INV** `[3]` (RW)
///
///   Invert output B
///
/// - **A_INV** `[2]` (RW)
///
///   Invert output A
///
/// - **PH_CORRECT** `[1]` (RW)
///
///   1: Enable phase-correct modulation. 0: Trailing-edge
///
/// - **EN** `[0]` (RW)
///
///   Enable the PWM channel.
#[repr(transparent)]
pub struct Ch8Csr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch8Csr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch8Csr {
    // ======== Field Constants ========

    /// **PH_ADV**
    ///
    /// Advance the phase of the counter by 1 count, while it is running.
    /// Self-clearing. Write a 1, and poll until low. Counter must be running
    /// at less than full speed (div_int + div_frac / 16 > 1)
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const PH_ADV_OFFSET: u32 = 7;
    pub const PH_ADV_MASK: u32 = 0x00000080;
    pub const PH_ADV: u32 = 1 << 7;

    /// **PH_RET**
    ///
    /// Retard the phase of the counter by 1 count, while it is running.
    /// Self-clearing. Write a 1, and poll until low. Counter must be running.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const PH_RET_OFFSET: u32 = 6;
    pub const PH_RET_MASK: u32 = 0x00000040;
    pub const PH_RET: u32 = 1 << 6;

    /// **DIVMODE**
    /// Bits `[5:4]`, mask `0x00000030`
    pub const DIVMODE_OFFSET: u32 = 4;
    pub const DIVMODE_MASK: u32 = 0x00000030;
    /// div = 0
    /// Free-running counting at rate dictated by fractional divider
    pub const DIVMODE_DIV: u32 = 0 << 4;
    pub const DIVMODE_DIV_VAL: u32 = 0;
    /// level = 1
    /// Fractional divider operation is gated by the PWM B pin.
    pub const DIVMODE_LEVEL: u32 = 1 << 4;
    pub const DIVMODE_LEVEL_VAL: u32 = 1;
    /// rise = 2
    /// Counter advances with each rising edge of the PWM B pin.
    pub const DIVMODE_RISE: u32 = 2 << 4;
    pub const DIVMODE_RISE_VAL: u32 = 2;
    /// fall = 3
    /// Counter advances with each falling edge of the PWM B pin.
    pub const DIVMODE_FALL: u32 = 3 << 4;
    pub const DIVMODE_FALL_VAL: u32 = 3;

    /// **B_INV**
    ///
    /// Invert output B
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const B_INV_OFFSET: u32 = 3;
    pub const B_INV_MASK: u32 = 0x00000008;
    pub const B_INV: u32 = 1 << 3;

    /// **A_INV**
    ///
    /// Invert output A
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const A_INV_OFFSET: u32 = 2;
    pub const A_INV_MASK: u32 = 0x00000004;
    pub const A_INV: u32 = 1 << 2;

    /// **PH_CORRECT**
    ///
    /// 1: Enable phase-correct modulation. 0: Trailing-edge
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const PH_CORRECT_OFFSET: u32 = 1;
    pub const PH_CORRECT_MASK: u32 = 0x00000002;
    pub const PH_CORRECT: u32 = 1 << 1;

    /// **EN**
    ///
    /// Enable the PWM channel.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const EN_OFFSET: u32 = 0;
    pub const EN_MASK: u32 = 0x00000001;
    pub const EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **PH_ADV** (WO)
    ///
    /// Advance the phase of the counter by 1 count, while it is running.
    /// Self-clearing. Write a 1, and poll until low. Counter must be running
    /// at less than full speed (div_int + div_frac / 16 > 1)
    #[inline(always)]
    pub fn trigger_ph_adv(&self) {
        self.value.write_bits(Self::PH_ADV_MASK);
    }

    /// Trigger **PH_RET** (WO)
    ///
    /// Retard the phase of the counter by 1 count, while it is running.
    /// Self-clearing. Write a 1, and poll until low. Counter must be running.
    #[inline(always)]
    pub fn trigger_ph_ret(&self) {
        self.value.write_bits(Self::PH_RET_MASK);
    }

    /// **DIVMODE** `[5:4]` (RW)
    #[inline(always)]
    pub fn divmode(&self) -> u32 {
        (self.value.read_bits() & Self::DIVMODE_MASK) >> Self::DIVMODE_OFFSET
    }

    /// Write **DIVMODE** `[5:4]` (RW)
    #[inline(always)]
    pub fn write_divmode(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DIVMODE_MASK) | ((value << Self::DIVMODE_OFFSET) & Self::DIVMODE_MASK));
    }

    /// **B_INV** `[3]` (RW)
    ///
    /// Invert output B
    #[inline(always)]
    pub fn b_inv(&self) -> bool {
        (self.value.read_bits() & Self::B_INV_MASK) != 0
    }

    /// Set B_INV bit atomically (RW)
    #[inline(always)]
    pub fn set_b_inv(&self) {
        self.value.set_bits(Self::B_INV_MASK);
    }

    /// Clear B_INV bit atomically (RW)
    #[inline(always)]
    pub fn clr_b_inv(&self) {
        self.value.clr_bits(Self::B_INV_MASK);
    }

    /// **A_INV** `[2]` (RW)
    ///
    /// Invert output A
    #[inline(always)]
    pub fn a_inv(&self) -> bool {
        (self.value.read_bits() & Self::A_INV_MASK) != 0
    }

    /// Set A_INV bit atomically (RW)
    #[inline(always)]
    pub fn set_a_inv(&self) {
        self.value.set_bits(Self::A_INV_MASK);
    }

    /// Clear A_INV bit atomically (RW)
    #[inline(always)]
    pub fn clr_a_inv(&self) {
        self.value.clr_bits(Self::A_INV_MASK);
    }

    /// **PH_CORRECT** `[1]` (RW)
    ///
    /// 1: Enable phase-correct modulation. 0: Trailing-edge
    #[inline(always)]
    pub fn ph_correct(&self) -> bool {
        (self.value.read_bits() & Self::PH_CORRECT_MASK) != 0
    }

    /// Set PH_CORRECT bit atomically (RW)
    #[inline(always)]
    pub fn set_ph_correct(&self) {
        self.value.set_bits(Self::PH_CORRECT_MASK);
    }

    /// Clear PH_CORRECT bit atomically (RW)
    #[inline(always)]
    pub fn clr_ph_correct(&self) {
        self.value.clr_bits(Self::PH_CORRECT_MASK);
    }

    /// **EN** `[0]` (RW)
    ///
    /// Enable the PWM channel.
    #[inline(always)]
    pub fn en(&self) -> bool {
        (self.value.read_bits() & Self::EN_MASK) != 0
    }

    /// Set EN bit atomically (RW)
    #[inline(always)]
    pub fn set_en(&self) {
        self.value.set_bits(Self::EN_MASK);
    }

    /// Clear EN bit atomically (RW)
    #[inline(always)]
    pub fn clr_en(&self) {
        self.value.clr_bits(Self::EN_MASK);
    }

}

/// CH8_CSR value type for batch read/modify/write operations
///
/// Use `ch8csr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch8csr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch8CsrVal {
    bits: u32,
}

impl Ch8CsrVal {
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

    /// Read **DIVMODE** `[5:4]`
    #[inline(always)]
    pub const fn divmode(&self) -> u32 {
        (self.bits & Ch8Csr::DIVMODE_MASK) >> Ch8Csr::DIVMODE_OFFSET
    }

    /// Read **B_INV** `[3:3]`
    ///
    /// Invert output B
    #[inline(always)]
    pub const fn b_inv(&self) -> bool {
        (self.bits & Ch8Csr::B_INV_MASK) != 0
    }

    /// Read **A_INV** `[2:2]`
    ///
    /// Invert output A
    #[inline(always)]
    pub const fn a_inv(&self) -> bool {
        (self.bits & Ch8Csr::A_INV_MASK) != 0
    }

    /// Read **PH_CORRECT** `[1:1]`
    ///
    /// 1: Enable phase-correct modulation. 0: Trailing-edge
    #[inline(always)]
    pub const fn ph_correct(&self) -> bool {
        (self.bits & Ch8Csr::PH_CORRECT_MASK) != 0
    }

    /// Read **EN** `[0:0]`
    ///
    /// Enable the PWM channel.
    #[inline(always)]
    pub const fn en(&self) -> bool {
        (self.bits & Ch8Csr::EN_MASK) != 0
    }

    /// Set **PH_ADV** `[7:7]`
    ///
    /// Advance the phase of the counter by 1 count, while it is running.
    /// Self-clearing. Write a 1, and poll until low. Counter must be running
    /// at less than full speed (div_int + div_frac / 16 > 1)
    #[inline(always)]
    pub const fn with_ph_adv(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch8Csr::PH_ADV_MASK
            } else {
                self.bits & !Ch8Csr::PH_ADV_MASK
            }
        }
    }

    /// Set **PH_RET** `[6:6]`
    ///
    /// Retard the phase of the counter by 1 count, while it is running.
    /// Self-clearing. Write a 1, and poll until low. Counter must be running.
    #[inline(always)]
    pub const fn with_ph_ret(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch8Csr::PH_RET_MASK
            } else {
                self.bits & !Ch8Csr::PH_RET_MASK
            }
        }
    }

    /// Set **DIVMODE** `[5:4]`
    #[inline(always)]
    pub const fn with_divmode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch8Csr::DIVMODE_MASK) | ((value << Ch8Csr::DIVMODE_OFFSET) & Ch8Csr::DIVMODE_MASK)
        }
    }

    /// Set **B_INV** `[3:3]`
    ///
    /// Invert output B
    #[inline(always)]
    pub const fn with_b_inv(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch8Csr::B_INV_MASK
            } else {
                self.bits & !Ch8Csr::B_INV_MASK
            }
        }
    }

    /// Set **A_INV** `[2:2]`
    ///
    /// Invert output A
    #[inline(always)]
    pub const fn with_a_inv(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch8Csr::A_INV_MASK
            } else {
                self.bits & !Ch8Csr::A_INV_MASK
            }
        }
    }

    /// Set **PH_CORRECT** `[1:1]`
    ///
    /// 1: Enable phase-correct modulation. 0: Trailing-edge
    #[inline(always)]
    pub const fn with_ph_correct(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch8Csr::PH_CORRECT_MASK
            } else {
                self.bits & !Ch8Csr::PH_CORRECT_MASK
            }
        }
    }

    /// Set **EN** `[0:0]`
    ///
    /// Enable the PWM channel.
    #[inline(always)]
    pub const fn with_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch8Csr::EN_MASK
            } else {
                self.bits & !Ch8Csr::EN_MASK
            }
        }
    }

}

impl Default for Ch8CsrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ch8Csr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch8CsrVal {
        Ch8CsrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch8CsrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch8CsrVal) -> Ch8CsrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// CH8_DIV Register
///
/// INT and FRAC form a fixed-point fractional number.
/// Counting rate is system clock frequency divided by this number.
/// Fractional division uses simple 1st-order sigma-delta.
///
/// Address offset: 0x00a4
/// Reset value: 0x00000010
///
/// ## Fields
///
/// - **INT** `[11:4]` (RW)
///
/// - **FRAC** `[3:0]` (RW)
#[repr(transparent)]
pub struct Ch8Div {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch8Div {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch8Div {
    // ======== Field Constants ========

    /// **INT**
    /// Bits `[11:4]`, mask `0x00000ff0`
    pub const INT_OFFSET: u32 = 4;
    pub const INT_MASK: u32 = 0x00000ff0;

    /// **FRAC**
    /// Bits `[3:0]`, mask `0x0000000f`
    pub const FRAC_OFFSET: u32 = 0;
    pub const FRAC_MASK: u32 = 0x0000000f;

    // ======== Field Accessors ========

    /// **INT** `[11:4]` (RW)
    #[inline(always)]
    pub fn int(&self) -> u32 {
        (self.value.read_bits() & Self::INT_MASK) >> Self::INT_OFFSET
    }

    /// Write **INT** `[11:4]` (RW)
    #[inline(always)]
    pub fn write_int(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::INT_MASK) | ((value << Self::INT_OFFSET) & Self::INT_MASK));
    }

    /// **FRAC** `[3:0]` (RW)
    #[inline(always)]
    pub fn frac(&self) -> u32 {
        (self.value.read_bits() & Self::FRAC_MASK) >> Self::FRAC_OFFSET
    }

    /// Write **FRAC** `[3:0]` (RW)
    #[inline(always)]
    pub fn write_frac(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::FRAC_MASK) | ((value << Self::FRAC_OFFSET) & Self::FRAC_MASK));
    }

}

/// CH8_DIV value type for batch read/modify/write operations
///
/// Use `ch8div.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch8div.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch8DivVal {
    bits: u32,
}

impl Ch8DivVal {
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
        Self { bits: 0x00000010 }
    }

    /// Read **INT** `[11:4]`
    #[inline(always)]
    pub const fn int(&self) -> u32 {
        (self.bits & Ch8Div::INT_MASK) >> Ch8Div::INT_OFFSET
    }

    /// Read **FRAC** `[3:0]`
    #[inline(always)]
    pub const fn frac(&self) -> u32 {
        (self.bits & Ch8Div::FRAC_MASK) >> Ch8Div::FRAC_OFFSET
    }

    /// Set **INT** `[11:4]`
    #[inline(always)]
    pub const fn with_int(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch8Div::INT_MASK) | ((value << Ch8Div::INT_OFFSET) & Ch8Div::INT_MASK)
        }
    }

    /// Set **FRAC** `[3:0]`
    #[inline(always)]
    pub const fn with_frac(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch8Div::FRAC_MASK) | ((value << Ch8Div::FRAC_OFFSET) & Ch8Div::FRAC_MASK)
        }
    }

}

impl Default for Ch8DivVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ch8Div {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch8DivVal {
        Ch8DivVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch8DivVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch8DivVal) -> Ch8DivVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// CH8_CTR Register
///
/// Direct access to the PWM counter
///
/// Address offset: 0x00a8
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CH8_CTR** `[15:0]` (RW)
#[repr(transparent)]
pub struct Ch8Ctr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch8Ctr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch8Ctr {
    // ======== Field Constants ========

    /// **CH8_CTR**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const CH8_CTR_OFFSET: u32 = 0;
    pub const CH8_CTR_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **CH8_CTR** `[15:0]` (RW)
    #[inline(always)]
    pub fn ch8_ctr(&self) -> u32 {
        (self.value.read_bits() & Self::CH8_CTR_MASK) >> Self::CH8_CTR_OFFSET
    }

    /// Write **CH8_CTR** `[15:0]` (RW)
    #[inline(always)]
    pub fn write_ch8_ctr(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::CH8_CTR_MASK) | ((value << Self::CH8_CTR_OFFSET) & Self::CH8_CTR_MASK));
    }

}

/// CH8_CTR value type for batch read/modify/write operations
///
/// Use `ch8ctr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch8ctr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch8CtrVal {
    bits: u32,
}

impl Ch8CtrVal {
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

    /// Read **CH8_CTR** `[15:0]`
    #[inline(always)]
    pub const fn ch8_ctr(&self) -> u32 {
        (self.bits & Ch8Ctr::CH8_CTR_MASK) >> Ch8Ctr::CH8_CTR_OFFSET
    }

    /// Set **CH8_CTR** `[15:0]`
    #[inline(always)]
    pub const fn with_ch8_ctr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch8Ctr::CH8_CTR_MASK) | ((value << Ch8Ctr::CH8_CTR_OFFSET) & Ch8Ctr::CH8_CTR_MASK)
        }
    }

}

impl Default for Ch8CtrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ch8Ctr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch8CtrVal {
        Ch8CtrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch8CtrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch8CtrVal) -> Ch8CtrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// CH8_CC Register
///
/// Counter compare values
///
/// Address offset: 0x00ac
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **B** `[31:16]` (RW)
///
/// - **A** `[15:0]` (RW)
#[repr(transparent)]
pub struct Ch8Cc {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch8Cc {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch8Cc {
    // ======== Field Constants ========

    /// **B**
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const B_OFFSET: u32 = 16;
    pub const B_MASK: u32 = 0xffff0000;

    /// **A**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const A_OFFSET: u32 = 0;
    pub const A_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **B** `[31:16]` (RW)
    #[inline(always)]
    pub fn b(&self) -> u32 {
        (self.value.read_bits() & Self::B_MASK) >> Self::B_OFFSET
    }

    /// Write **B** `[31:16]` (RW)
    #[inline(always)]
    pub fn write_b(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::B_MASK) | ((value << Self::B_OFFSET) & Self::B_MASK));
    }

    /// **A** `[15:0]` (RW)
    #[inline(always)]
    pub fn a(&self) -> u32 {
        (self.value.read_bits() & Self::A_MASK) >> Self::A_OFFSET
    }

    /// Write **A** `[15:0]` (RW)
    #[inline(always)]
    pub fn write_a(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::A_MASK) | ((value << Self::A_OFFSET) & Self::A_MASK));
    }

}

/// CH8_CC value type for batch read/modify/write operations
///
/// Use `ch8cc.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch8cc.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch8CcVal {
    bits: u32,
}

impl Ch8CcVal {
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

    /// Read **B** `[31:16]`
    #[inline(always)]
    pub const fn b(&self) -> u32 {
        (self.bits & Ch8Cc::B_MASK) >> Ch8Cc::B_OFFSET
    }

    /// Read **A** `[15:0]`
    #[inline(always)]
    pub const fn a(&self) -> u32 {
        (self.bits & Ch8Cc::A_MASK) >> Ch8Cc::A_OFFSET
    }

    /// Set **B** `[31:16]`
    #[inline(always)]
    pub const fn with_b(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch8Cc::B_MASK) | ((value << Ch8Cc::B_OFFSET) & Ch8Cc::B_MASK)
        }
    }

    /// Set **A** `[15:0]`
    #[inline(always)]
    pub const fn with_a(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch8Cc::A_MASK) | ((value << Ch8Cc::A_OFFSET) & Ch8Cc::A_MASK)
        }
    }

}

impl Default for Ch8CcVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ch8Cc {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch8CcVal {
        Ch8CcVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch8CcVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch8CcVal) -> Ch8CcVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// CH8_TOP Register
///
/// Counter wrap value
///
/// Address offset: 0x00b0
/// Reset value: 0x0000ffff
///
/// ## Fields
///
/// - **CH8_TOP** `[15:0]` (RW)
#[repr(transparent)]
pub struct Ch8Top {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch8Top {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch8Top {
    // ======== Field Constants ========

    /// **CH8_TOP**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const CH8_TOP_OFFSET: u32 = 0;
    pub const CH8_TOP_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **CH8_TOP** `[15:0]` (RW)
    #[inline(always)]
    pub fn ch8_top(&self) -> u32 {
        (self.value.read_bits() & Self::CH8_TOP_MASK) >> Self::CH8_TOP_OFFSET
    }

    /// Write **CH8_TOP** `[15:0]` (RW)
    #[inline(always)]
    pub fn write_ch8_top(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::CH8_TOP_MASK) | ((value << Self::CH8_TOP_OFFSET) & Self::CH8_TOP_MASK));
    }

}

/// CH8_TOP value type for batch read/modify/write operations
///
/// Use `ch8top.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch8top.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch8TopVal {
    bits: u32,
}

impl Ch8TopVal {
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
        Self { bits: 0x0000ffff }
    }

    /// Read **CH8_TOP** `[15:0]`
    #[inline(always)]
    pub const fn ch8_top(&self) -> u32 {
        (self.bits & Ch8Top::CH8_TOP_MASK) >> Ch8Top::CH8_TOP_OFFSET
    }

    /// Set **CH8_TOP** `[15:0]`
    #[inline(always)]
    pub const fn with_ch8_top(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch8Top::CH8_TOP_MASK) | ((value << Ch8Top::CH8_TOP_OFFSET) & Ch8Top::CH8_TOP_MASK)
        }
    }

}

impl Default for Ch8TopVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ch8Top {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch8TopVal {
        Ch8TopVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch8TopVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch8TopVal) -> Ch8TopVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// CH9_CSR Register
///
/// Control and status register
///
/// Address offset: 0x00b4
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **PH_ADV** `[7]` (WO)
///
///   Advance the phase of the counter by 1 count, while it is running.
///   Self-clearing. Write a 1, and poll until low. Counter must be running
///   at less than full speed (div_int + div_frac / 16 > 1)
///
/// - **PH_RET** `[6]` (WO)
///
///   Retard the phase of the counter by 1 count, while it is running.
///   Self-clearing. Write a 1, and poll until low. Counter must be running.
///
/// - **DIVMODE** `[5:4]` (RW)
///
/// - **B_INV** `[3]` (RW)
///
///   Invert output B
///
/// - **A_INV** `[2]` (RW)
///
///   Invert output A
///
/// - **PH_CORRECT** `[1]` (RW)
///
///   1: Enable phase-correct modulation. 0: Trailing-edge
///
/// - **EN** `[0]` (RW)
///
///   Enable the PWM channel.
#[repr(transparent)]
pub struct Ch9Csr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch9Csr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch9Csr {
    // ======== Field Constants ========

    /// **PH_ADV**
    ///
    /// Advance the phase of the counter by 1 count, while it is running.
    /// Self-clearing. Write a 1, and poll until low. Counter must be running
    /// at less than full speed (div_int + div_frac / 16 > 1)
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const PH_ADV_OFFSET: u32 = 7;
    pub const PH_ADV_MASK: u32 = 0x00000080;
    pub const PH_ADV: u32 = 1 << 7;

    /// **PH_RET**
    ///
    /// Retard the phase of the counter by 1 count, while it is running.
    /// Self-clearing. Write a 1, and poll until low. Counter must be running.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const PH_RET_OFFSET: u32 = 6;
    pub const PH_RET_MASK: u32 = 0x00000040;
    pub const PH_RET: u32 = 1 << 6;

    /// **DIVMODE**
    /// Bits `[5:4]`, mask `0x00000030`
    pub const DIVMODE_OFFSET: u32 = 4;
    pub const DIVMODE_MASK: u32 = 0x00000030;
    /// div = 0
    /// Free-running counting at rate dictated by fractional divider
    pub const DIVMODE_DIV: u32 = 0 << 4;
    pub const DIVMODE_DIV_VAL: u32 = 0;
    /// level = 1
    /// Fractional divider operation is gated by the PWM B pin.
    pub const DIVMODE_LEVEL: u32 = 1 << 4;
    pub const DIVMODE_LEVEL_VAL: u32 = 1;
    /// rise = 2
    /// Counter advances with each rising edge of the PWM B pin.
    pub const DIVMODE_RISE: u32 = 2 << 4;
    pub const DIVMODE_RISE_VAL: u32 = 2;
    /// fall = 3
    /// Counter advances with each falling edge of the PWM B pin.
    pub const DIVMODE_FALL: u32 = 3 << 4;
    pub const DIVMODE_FALL_VAL: u32 = 3;

    /// **B_INV**
    ///
    /// Invert output B
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const B_INV_OFFSET: u32 = 3;
    pub const B_INV_MASK: u32 = 0x00000008;
    pub const B_INV: u32 = 1 << 3;

    /// **A_INV**
    ///
    /// Invert output A
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const A_INV_OFFSET: u32 = 2;
    pub const A_INV_MASK: u32 = 0x00000004;
    pub const A_INV: u32 = 1 << 2;

    /// **PH_CORRECT**
    ///
    /// 1: Enable phase-correct modulation. 0: Trailing-edge
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const PH_CORRECT_OFFSET: u32 = 1;
    pub const PH_CORRECT_MASK: u32 = 0x00000002;
    pub const PH_CORRECT: u32 = 1 << 1;

    /// **EN**
    ///
    /// Enable the PWM channel.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const EN_OFFSET: u32 = 0;
    pub const EN_MASK: u32 = 0x00000001;
    pub const EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **PH_ADV** (WO)
    ///
    /// Advance the phase of the counter by 1 count, while it is running.
    /// Self-clearing. Write a 1, and poll until low. Counter must be running
    /// at less than full speed (div_int + div_frac / 16 > 1)
    #[inline(always)]
    pub fn trigger_ph_adv(&self) {
        self.value.write_bits(Self::PH_ADV_MASK);
    }

    /// Trigger **PH_RET** (WO)
    ///
    /// Retard the phase of the counter by 1 count, while it is running.
    /// Self-clearing. Write a 1, and poll until low. Counter must be running.
    #[inline(always)]
    pub fn trigger_ph_ret(&self) {
        self.value.write_bits(Self::PH_RET_MASK);
    }

    /// **DIVMODE** `[5:4]` (RW)
    #[inline(always)]
    pub fn divmode(&self) -> u32 {
        (self.value.read_bits() & Self::DIVMODE_MASK) >> Self::DIVMODE_OFFSET
    }

    /// Write **DIVMODE** `[5:4]` (RW)
    #[inline(always)]
    pub fn write_divmode(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DIVMODE_MASK) | ((value << Self::DIVMODE_OFFSET) & Self::DIVMODE_MASK));
    }

    /// **B_INV** `[3]` (RW)
    ///
    /// Invert output B
    #[inline(always)]
    pub fn b_inv(&self) -> bool {
        (self.value.read_bits() & Self::B_INV_MASK) != 0
    }

    /// Set B_INV bit atomically (RW)
    #[inline(always)]
    pub fn set_b_inv(&self) {
        self.value.set_bits(Self::B_INV_MASK);
    }

    /// Clear B_INV bit atomically (RW)
    #[inline(always)]
    pub fn clr_b_inv(&self) {
        self.value.clr_bits(Self::B_INV_MASK);
    }

    /// **A_INV** `[2]` (RW)
    ///
    /// Invert output A
    #[inline(always)]
    pub fn a_inv(&self) -> bool {
        (self.value.read_bits() & Self::A_INV_MASK) != 0
    }

    /// Set A_INV bit atomically (RW)
    #[inline(always)]
    pub fn set_a_inv(&self) {
        self.value.set_bits(Self::A_INV_MASK);
    }

    /// Clear A_INV bit atomically (RW)
    #[inline(always)]
    pub fn clr_a_inv(&self) {
        self.value.clr_bits(Self::A_INV_MASK);
    }

    /// **PH_CORRECT** `[1]` (RW)
    ///
    /// 1: Enable phase-correct modulation. 0: Trailing-edge
    #[inline(always)]
    pub fn ph_correct(&self) -> bool {
        (self.value.read_bits() & Self::PH_CORRECT_MASK) != 0
    }

    /// Set PH_CORRECT bit atomically (RW)
    #[inline(always)]
    pub fn set_ph_correct(&self) {
        self.value.set_bits(Self::PH_CORRECT_MASK);
    }

    /// Clear PH_CORRECT bit atomically (RW)
    #[inline(always)]
    pub fn clr_ph_correct(&self) {
        self.value.clr_bits(Self::PH_CORRECT_MASK);
    }

    /// **EN** `[0]` (RW)
    ///
    /// Enable the PWM channel.
    #[inline(always)]
    pub fn en(&self) -> bool {
        (self.value.read_bits() & Self::EN_MASK) != 0
    }

    /// Set EN bit atomically (RW)
    #[inline(always)]
    pub fn set_en(&self) {
        self.value.set_bits(Self::EN_MASK);
    }

    /// Clear EN bit atomically (RW)
    #[inline(always)]
    pub fn clr_en(&self) {
        self.value.clr_bits(Self::EN_MASK);
    }

}

/// CH9_CSR value type for batch read/modify/write operations
///
/// Use `ch9csr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch9csr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch9CsrVal {
    bits: u32,
}

impl Ch9CsrVal {
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

    /// Read **DIVMODE** `[5:4]`
    #[inline(always)]
    pub const fn divmode(&self) -> u32 {
        (self.bits & Ch9Csr::DIVMODE_MASK) >> Ch9Csr::DIVMODE_OFFSET
    }

    /// Read **B_INV** `[3:3]`
    ///
    /// Invert output B
    #[inline(always)]
    pub const fn b_inv(&self) -> bool {
        (self.bits & Ch9Csr::B_INV_MASK) != 0
    }

    /// Read **A_INV** `[2:2]`
    ///
    /// Invert output A
    #[inline(always)]
    pub const fn a_inv(&self) -> bool {
        (self.bits & Ch9Csr::A_INV_MASK) != 0
    }

    /// Read **PH_CORRECT** `[1:1]`
    ///
    /// 1: Enable phase-correct modulation. 0: Trailing-edge
    #[inline(always)]
    pub const fn ph_correct(&self) -> bool {
        (self.bits & Ch9Csr::PH_CORRECT_MASK) != 0
    }

    /// Read **EN** `[0:0]`
    ///
    /// Enable the PWM channel.
    #[inline(always)]
    pub const fn en(&self) -> bool {
        (self.bits & Ch9Csr::EN_MASK) != 0
    }

    /// Set **PH_ADV** `[7:7]`
    ///
    /// Advance the phase of the counter by 1 count, while it is running.
    /// Self-clearing. Write a 1, and poll until low. Counter must be running
    /// at less than full speed (div_int + div_frac / 16 > 1)
    #[inline(always)]
    pub const fn with_ph_adv(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch9Csr::PH_ADV_MASK
            } else {
                self.bits & !Ch9Csr::PH_ADV_MASK
            }
        }
    }

    /// Set **PH_RET** `[6:6]`
    ///
    /// Retard the phase of the counter by 1 count, while it is running.
    /// Self-clearing. Write a 1, and poll until low. Counter must be running.
    #[inline(always)]
    pub const fn with_ph_ret(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch9Csr::PH_RET_MASK
            } else {
                self.bits & !Ch9Csr::PH_RET_MASK
            }
        }
    }

    /// Set **DIVMODE** `[5:4]`
    #[inline(always)]
    pub const fn with_divmode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch9Csr::DIVMODE_MASK) | ((value << Ch9Csr::DIVMODE_OFFSET) & Ch9Csr::DIVMODE_MASK)
        }
    }

    /// Set **B_INV** `[3:3]`
    ///
    /// Invert output B
    #[inline(always)]
    pub const fn with_b_inv(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch9Csr::B_INV_MASK
            } else {
                self.bits & !Ch9Csr::B_INV_MASK
            }
        }
    }

    /// Set **A_INV** `[2:2]`
    ///
    /// Invert output A
    #[inline(always)]
    pub const fn with_a_inv(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch9Csr::A_INV_MASK
            } else {
                self.bits & !Ch9Csr::A_INV_MASK
            }
        }
    }

    /// Set **PH_CORRECT** `[1:1]`
    ///
    /// 1: Enable phase-correct modulation. 0: Trailing-edge
    #[inline(always)]
    pub const fn with_ph_correct(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch9Csr::PH_CORRECT_MASK
            } else {
                self.bits & !Ch9Csr::PH_CORRECT_MASK
            }
        }
    }

    /// Set **EN** `[0:0]`
    ///
    /// Enable the PWM channel.
    #[inline(always)]
    pub const fn with_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch9Csr::EN_MASK
            } else {
                self.bits & !Ch9Csr::EN_MASK
            }
        }
    }

}

impl Default for Ch9CsrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ch9Csr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch9CsrVal {
        Ch9CsrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch9CsrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch9CsrVal) -> Ch9CsrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// CH9_DIV Register
///
/// INT and FRAC form a fixed-point fractional number.
/// Counting rate is system clock frequency divided by this number.
/// Fractional division uses simple 1st-order sigma-delta.
///
/// Address offset: 0x00b8
/// Reset value: 0x00000010
///
/// ## Fields
///
/// - **INT** `[11:4]` (RW)
///
/// - **FRAC** `[3:0]` (RW)
#[repr(transparent)]
pub struct Ch9Div {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch9Div {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch9Div {
    // ======== Field Constants ========

    /// **INT**
    /// Bits `[11:4]`, mask `0x00000ff0`
    pub const INT_OFFSET: u32 = 4;
    pub const INT_MASK: u32 = 0x00000ff0;

    /// **FRAC**
    /// Bits `[3:0]`, mask `0x0000000f`
    pub const FRAC_OFFSET: u32 = 0;
    pub const FRAC_MASK: u32 = 0x0000000f;

    // ======== Field Accessors ========

    /// **INT** `[11:4]` (RW)
    #[inline(always)]
    pub fn int(&self) -> u32 {
        (self.value.read_bits() & Self::INT_MASK) >> Self::INT_OFFSET
    }

    /// Write **INT** `[11:4]` (RW)
    #[inline(always)]
    pub fn write_int(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::INT_MASK) | ((value << Self::INT_OFFSET) & Self::INT_MASK));
    }

    /// **FRAC** `[3:0]` (RW)
    #[inline(always)]
    pub fn frac(&self) -> u32 {
        (self.value.read_bits() & Self::FRAC_MASK) >> Self::FRAC_OFFSET
    }

    /// Write **FRAC** `[3:0]` (RW)
    #[inline(always)]
    pub fn write_frac(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::FRAC_MASK) | ((value << Self::FRAC_OFFSET) & Self::FRAC_MASK));
    }

}

/// CH9_DIV value type for batch read/modify/write operations
///
/// Use `ch9div.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch9div.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch9DivVal {
    bits: u32,
}

impl Ch9DivVal {
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
        Self { bits: 0x00000010 }
    }

    /// Read **INT** `[11:4]`
    #[inline(always)]
    pub const fn int(&self) -> u32 {
        (self.bits & Ch9Div::INT_MASK) >> Ch9Div::INT_OFFSET
    }

    /// Read **FRAC** `[3:0]`
    #[inline(always)]
    pub const fn frac(&self) -> u32 {
        (self.bits & Ch9Div::FRAC_MASK) >> Ch9Div::FRAC_OFFSET
    }

    /// Set **INT** `[11:4]`
    #[inline(always)]
    pub const fn with_int(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch9Div::INT_MASK) | ((value << Ch9Div::INT_OFFSET) & Ch9Div::INT_MASK)
        }
    }

    /// Set **FRAC** `[3:0]`
    #[inline(always)]
    pub const fn with_frac(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch9Div::FRAC_MASK) | ((value << Ch9Div::FRAC_OFFSET) & Ch9Div::FRAC_MASK)
        }
    }

}

impl Default for Ch9DivVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ch9Div {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch9DivVal {
        Ch9DivVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch9DivVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch9DivVal) -> Ch9DivVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// CH9_CTR Register
///
/// Direct access to the PWM counter
///
/// Address offset: 0x00bc
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CH9_CTR** `[15:0]` (RW)
#[repr(transparent)]
pub struct Ch9Ctr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch9Ctr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch9Ctr {
    // ======== Field Constants ========

    /// **CH9_CTR**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const CH9_CTR_OFFSET: u32 = 0;
    pub const CH9_CTR_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **CH9_CTR** `[15:0]` (RW)
    #[inline(always)]
    pub fn ch9_ctr(&self) -> u32 {
        (self.value.read_bits() & Self::CH9_CTR_MASK) >> Self::CH9_CTR_OFFSET
    }

    /// Write **CH9_CTR** `[15:0]` (RW)
    #[inline(always)]
    pub fn write_ch9_ctr(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::CH9_CTR_MASK) | ((value << Self::CH9_CTR_OFFSET) & Self::CH9_CTR_MASK));
    }

}

/// CH9_CTR value type for batch read/modify/write operations
///
/// Use `ch9ctr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch9ctr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch9CtrVal {
    bits: u32,
}

impl Ch9CtrVal {
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

    /// Read **CH9_CTR** `[15:0]`
    #[inline(always)]
    pub const fn ch9_ctr(&self) -> u32 {
        (self.bits & Ch9Ctr::CH9_CTR_MASK) >> Ch9Ctr::CH9_CTR_OFFSET
    }

    /// Set **CH9_CTR** `[15:0]`
    #[inline(always)]
    pub const fn with_ch9_ctr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch9Ctr::CH9_CTR_MASK) | ((value << Ch9Ctr::CH9_CTR_OFFSET) & Ch9Ctr::CH9_CTR_MASK)
        }
    }

}

impl Default for Ch9CtrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ch9Ctr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch9CtrVal {
        Ch9CtrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch9CtrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch9CtrVal) -> Ch9CtrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// CH9_CC Register
///
/// Counter compare values
///
/// Address offset: 0x00c0
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **B** `[31:16]` (RW)
///
/// - **A** `[15:0]` (RW)
#[repr(transparent)]
pub struct Ch9Cc {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch9Cc {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch9Cc {
    // ======== Field Constants ========

    /// **B**
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const B_OFFSET: u32 = 16;
    pub const B_MASK: u32 = 0xffff0000;

    /// **A**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const A_OFFSET: u32 = 0;
    pub const A_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **B** `[31:16]` (RW)
    #[inline(always)]
    pub fn b(&self) -> u32 {
        (self.value.read_bits() & Self::B_MASK) >> Self::B_OFFSET
    }

    /// Write **B** `[31:16]` (RW)
    #[inline(always)]
    pub fn write_b(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::B_MASK) | ((value << Self::B_OFFSET) & Self::B_MASK));
    }

    /// **A** `[15:0]` (RW)
    #[inline(always)]
    pub fn a(&self) -> u32 {
        (self.value.read_bits() & Self::A_MASK) >> Self::A_OFFSET
    }

    /// Write **A** `[15:0]` (RW)
    #[inline(always)]
    pub fn write_a(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::A_MASK) | ((value << Self::A_OFFSET) & Self::A_MASK));
    }

}

/// CH9_CC value type for batch read/modify/write operations
///
/// Use `ch9cc.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch9cc.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch9CcVal {
    bits: u32,
}

impl Ch9CcVal {
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

    /// Read **B** `[31:16]`
    #[inline(always)]
    pub const fn b(&self) -> u32 {
        (self.bits & Ch9Cc::B_MASK) >> Ch9Cc::B_OFFSET
    }

    /// Read **A** `[15:0]`
    #[inline(always)]
    pub const fn a(&self) -> u32 {
        (self.bits & Ch9Cc::A_MASK) >> Ch9Cc::A_OFFSET
    }

    /// Set **B** `[31:16]`
    #[inline(always)]
    pub const fn with_b(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch9Cc::B_MASK) | ((value << Ch9Cc::B_OFFSET) & Ch9Cc::B_MASK)
        }
    }

    /// Set **A** `[15:0]`
    #[inline(always)]
    pub const fn with_a(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch9Cc::A_MASK) | ((value << Ch9Cc::A_OFFSET) & Ch9Cc::A_MASK)
        }
    }

}

impl Default for Ch9CcVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ch9Cc {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch9CcVal {
        Ch9CcVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch9CcVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch9CcVal) -> Ch9CcVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// CH9_TOP Register
///
/// Counter wrap value
///
/// Address offset: 0x00c4
/// Reset value: 0x0000ffff
///
/// ## Fields
///
/// - **CH9_TOP** `[15:0]` (RW)
#[repr(transparent)]
pub struct Ch9Top {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch9Top {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch9Top {
    // ======== Field Constants ========

    /// **CH9_TOP**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const CH9_TOP_OFFSET: u32 = 0;
    pub const CH9_TOP_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **CH9_TOP** `[15:0]` (RW)
    #[inline(always)]
    pub fn ch9_top(&self) -> u32 {
        (self.value.read_bits() & Self::CH9_TOP_MASK) >> Self::CH9_TOP_OFFSET
    }

    /// Write **CH9_TOP** `[15:0]` (RW)
    #[inline(always)]
    pub fn write_ch9_top(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::CH9_TOP_MASK) | ((value << Self::CH9_TOP_OFFSET) & Self::CH9_TOP_MASK));
    }

}

/// CH9_TOP value type for batch read/modify/write operations
///
/// Use `ch9top.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch9top.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch9TopVal {
    bits: u32,
}

impl Ch9TopVal {
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
        Self { bits: 0x0000ffff }
    }

    /// Read **CH9_TOP** `[15:0]`
    #[inline(always)]
    pub const fn ch9_top(&self) -> u32 {
        (self.bits & Ch9Top::CH9_TOP_MASK) >> Ch9Top::CH9_TOP_OFFSET
    }

    /// Set **CH9_TOP** `[15:0]`
    #[inline(always)]
    pub const fn with_ch9_top(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch9Top::CH9_TOP_MASK) | ((value << Ch9Top::CH9_TOP_OFFSET) & Ch9Top::CH9_TOP_MASK)
        }
    }

}

impl Default for Ch9TopVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ch9Top {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch9TopVal {
        Ch9TopVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch9TopVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch9TopVal) -> Ch9TopVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// CH10_CSR Register
///
/// Control and status register
///
/// Address offset: 0x00c8
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **PH_ADV** `[7]` (WO)
///
///   Advance the phase of the counter by 1 count, while it is running.
///   Self-clearing. Write a 1, and poll until low. Counter must be running
///   at less than full speed (div_int + div_frac / 16 > 1)
///
/// - **PH_RET** `[6]` (WO)
///
///   Retard the phase of the counter by 1 count, while it is running.
///   Self-clearing. Write a 1, and poll until low. Counter must be running.
///
/// - **DIVMODE** `[5:4]` (RW)
///
/// - **B_INV** `[3]` (RW)
///
///   Invert output B
///
/// - **A_INV** `[2]` (RW)
///
///   Invert output A
///
/// - **PH_CORRECT** `[1]` (RW)
///
///   1: Enable phase-correct modulation. 0: Trailing-edge
///
/// - **EN** `[0]` (RW)
///
///   Enable the PWM channel.
#[repr(transparent)]
pub struct Ch10Csr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch10Csr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch10Csr {
    // ======== Field Constants ========

    /// **PH_ADV**
    ///
    /// Advance the phase of the counter by 1 count, while it is running.
    /// Self-clearing. Write a 1, and poll until low. Counter must be running
    /// at less than full speed (div_int + div_frac / 16 > 1)
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const PH_ADV_OFFSET: u32 = 7;
    pub const PH_ADV_MASK: u32 = 0x00000080;
    pub const PH_ADV: u32 = 1 << 7;

    /// **PH_RET**
    ///
    /// Retard the phase of the counter by 1 count, while it is running.
    /// Self-clearing. Write a 1, and poll until low. Counter must be running.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const PH_RET_OFFSET: u32 = 6;
    pub const PH_RET_MASK: u32 = 0x00000040;
    pub const PH_RET: u32 = 1 << 6;

    /// **DIVMODE**
    /// Bits `[5:4]`, mask `0x00000030`
    pub const DIVMODE_OFFSET: u32 = 4;
    pub const DIVMODE_MASK: u32 = 0x00000030;
    /// div = 0
    /// Free-running counting at rate dictated by fractional divider
    pub const DIVMODE_DIV: u32 = 0 << 4;
    pub const DIVMODE_DIV_VAL: u32 = 0;
    /// level = 1
    /// Fractional divider operation is gated by the PWM B pin.
    pub const DIVMODE_LEVEL: u32 = 1 << 4;
    pub const DIVMODE_LEVEL_VAL: u32 = 1;
    /// rise = 2
    /// Counter advances with each rising edge of the PWM B pin.
    pub const DIVMODE_RISE: u32 = 2 << 4;
    pub const DIVMODE_RISE_VAL: u32 = 2;
    /// fall = 3
    /// Counter advances with each falling edge of the PWM B pin.
    pub const DIVMODE_FALL: u32 = 3 << 4;
    pub const DIVMODE_FALL_VAL: u32 = 3;

    /// **B_INV**
    ///
    /// Invert output B
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const B_INV_OFFSET: u32 = 3;
    pub const B_INV_MASK: u32 = 0x00000008;
    pub const B_INV: u32 = 1 << 3;

    /// **A_INV**
    ///
    /// Invert output A
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const A_INV_OFFSET: u32 = 2;
    pub const A_INV_MASK: u32 = 0x00000004;
    pub const A_INV: u32 = 1 << 2;

    /// **PH_CORRECT**
    ///
    /// 1: Enable phase-correct modulation. 0: Trailing-edge
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const PH_CORRECT_OFFSET: u32 = 1;
    pub const PH_CORRECT_MASK: u32 = 0x00000002;
    pub const PH_CORRECT: u32 = 1 << 1;

    /// **EN**
    ///
    /// Enable the PWM channel.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const EN_OFFSET: u32 = 0;
    pub const EN_MASK: u32 = 0x00000001;
    pub const EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **PH_ADV** (WO)
    ///
    /// Advance the phase of the counter by 1 count, while it is running.
    /// Self-clearing. Write a 1, and poll until low. Counter must be running
    /// at less than full speed (div_int + div_frac / 16 > 1)
    #[inline(always)]
    pub fn trigger_ph_adv(&self) {
        self.value.write_bits(Self::PH_ADV_MASK);
    }

    /// Trigger **PH_RET** (WO)
    ///
    /// Retard the phase of the counter by 1 count, while it is running.
    /// Self-clearing. Write a 1, and poll until low. Counter must be running.
    #[inline(always)]
    pub fn trigger_ph_ret(&self) {
        self.value.write_bits(Self::PH_RET_MASK);
    }

    /// **DIVMODE** `[5:4]` (RW)
    #[inline(always)]
    pub fn divmode(&self) -> u32 {
        (self.value.read_bits() & Self::DIVMODE_MASK) >> Self::DIVMODE_OFFSET
    }

    /// Write **DIVMODE** `[5:4]` (RW)
    #[inline(always)]
    pub fn write_divmode(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DIVMODE_MASK) | ((value << Self::DIVMODE_OFFSET) & Self::DIVMODE_MASK));
    }

    /// **B_INV** `[3]` (RW)
    ///
    /// Invert output B
    #[inline(always)]
    pub fn b_inv(&self) -> bool {
        (self.value.read_bits() & Self::B_INV_MASK) != 0
    }

    /// Set B_INV bit atomically (RW)
    #[inline(always)]
    pub fn set_b_inv(&self) {
        self.value.set_bits(Self::B_INV_MASK);
    }

    /// Clear B_INV bit atomically (RW)
    #[inline(always)]
    pub fn clr_b_inv(&self) {
        self.value.clr_bits(Self::B_INV_MASK);
    }

    /// **A_INV** `[2]` (RW)
    ///
    /// Invert output A
    #[inline(always)]
    pub fn a_inv(&self) -> bool {
        (self.value.read_bits() & Self::A_INV_MASK) != 0
    }

    /// Set A_INV bit atomically (RW)
    #[inline(always)]
    pub fn set_a_inv(&self) {
        self.value.set_bits(Self::A_INV_MASK);
    }

    /// Clear A_INV bit atomically (RW)
    #[inline(always)]
    pub fn clr_a_inv(&self) {
        self.value.clr_bits(Self::A_INV_MASK);
    }

    /// **PH_CORRECT** `[1]` (RW)
    ///
    /// 1: Enable phase-correct modulation. 0: Trailing-edge
    #[inline(always)]
    pub fn ph_correct(&self) -> bool {
        (self.value.read_bits() & Self::PH_CORRECT_MASK) != 0
    }

    /// Set PH_CORRECT bit atomically (RW)
    #[inline(always)]
    pub fn set_ph_correct(&self) {
        self.value.set_bits(Self::PH_CORRECT_MASK);
    }

    /// Clear PH_CORRECT bit atomically (RW)
    #[inline(always)]
    pub fn clr_ph_correct(&self) {
        self.value.clr_bits(Self::PH_CORRECT_MASK);
    }

    /// **EN** `[0]` (RW)
    ///
    /// Enable the PWM channel.
    #[inline(always)]
    pub fn en(&self) -> bool {
        (self.value.read_bits() & Self::EN_MASK) != 0
    }

    /// Set EN bit atomically (RW)
    #[inline(always)]
    pub fn set_en(&self) {
        self.value.set_bits(Self::EN_MASK);
    }

    /// Clear EN bit atomically (RW)
    #[inline(always)]
    pub fn clr_en(&self) {
        self.value.clr_bits(Self::EN_MASK);
    }

}

/// CH10_CSR value type for batch read/modify/write operations
///
/// Use `ch10csr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch10csr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch10CsrVal {
    bits: u32,
}

impl Ch10CsrVal {
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

    /// Read **DIVMODE** `[5:4]`
    #[inline(always)]
    pub const fn divmode(&self) -> u32 {
        (self.bits & Ch10Csr::DIVMODE_MASK) >> Ch10Csr::DIVMODE_OFFSET
    }

    /// Read **B_INV** `[3:3]`
    ///
    /// Invert output B
    #[inline(always)]
    pub const fn b_inv(&self) -> bool {
        (self.bits & Ch10Csr::B_INV_MASK) != 0
    }

    /// Read **A_INV** `[2:2]`
    ///
    /// Invert output A
    #[inline(always)]
    pub const fn a_inv(&self) -> bool {
        (self.bits & Ch10Csr::A_INV_MASK) != 0
    }

    /// Read **PH_CORRECT** `[1:1]`
    ///
    /// 1: Enable phase-correct modulation. 0: Trailing-edge
    #[inline(always)]
    pub const fn ph_correct(&self) -> bool {
        (self.bits & Ch10Csr::PH_CORRECT_MASK) != 0
    }

    /// Read **EN** `[0:0]`
    ///
    /// Enable the PWM channel.
    #[inline(always)]
    pub const fn en(&self) -> bool {
        (self.bits & Ch10Csr::EN_MASK) != 0
    }

    /// Set **PH_ADV** `[7:7]`
    ///
    /// Advance the phase of the counter by 1 count, while it is running.
    /// Self-clearing. Write a 1, and poll until low. Counter must be running
    /// at less than full speed (div_int + div_frac / 16 > 1)
    #[inline(always)]
    pub const fn with_ph_adv(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch10Csr::PH_ADV_MASK
            } else {
                self.bits & !Ch10Csr::PH_ADV_MASK
            }
        }
    }

    /// Set **PH_RET** `[6:6]`
    ///
    /// Retard the phase of the counter by 1 count, while it is running.
    /// Self-clearing. Write a 1, and poll until low. Counter must be running.
    #[inline(always)]
    pub const fn with_ph_ret(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch10Csr::PH_RET_MASK
            } else {
                self.bits & !Ch10Csr::PH_RET_MASK
            }
        }
    }

    /// Set **DIVMODE** `[5:4]`
    #[inline(always)]
    pub const fn with_divmode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch10Csr::DIVMODE_MASK) | ((value << Ch10Csr::DIVMODE_OFFSET) & Ch10Csr::DIVMODE_MASK)
        }
    }

    /// Set **B_INV** `[3:3]`
    ///
    /// Invert output B
    #[inline(always)]
    pub const fn with_b_inv(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch10Csr::B_INV_MASK
            } else {
                self.bits & !Ch10Csr::B_INV_MASK
            }
        }
    }

    /// Set **A_INV** `[2:2]`
    ///
    /// Invert output A
    #[inline(always)]
    pub const fn with_a_inv(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch10Csr::A_INV_MASK
            } else {
                self.bits & !Ch10Csr::A_INV_MASK
            }
        }
    }

    /// Set **PH_CORRECT** `[1:1]`
    ///
    /// 1: Enable phase-correct modulation. 0: Trailing-edge
    #[inline(always)]
    pub const fn with_ph_correct(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch10Csr::PH_CORRECT_MASK
            } else {
                self.bits & !Ch10Csr::PH_CORRECT_MASK
            }
        }
    }

    /// Set **EN** `[0:0]`
    ///
    /// Enable the PWM channel.
    #[inline(always)]
    pub const fn with_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch10Csr::EN_MASK
            } else {
                self.bits & !Ch10Csr::EN_MASK
            }
        }
    }

}

impl Default for Ch10CsrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ch10Csr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch10CsrVal {
        Ch10CsrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch10CsrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch10CsrVal) -> Ch10CsrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// CH10_DIV Register
///
/// INT and FRAC form a fixed-point fractional number.
/// Counting rate is system clock frequency divided by this number.
/// Fractional division uses simple 1st-order sigma-delta.
///
/// Address offset: 0x00cc
/// Reset value: 0x00000010
///
/// ## Fields
///
/// - **INT** `[11:4]` (RW)
///
/// - **FRAC** `[3:0]` (RW)
#[repr(transparent)]
pub struct Ch10Div {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch10Div {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch10Div {
    // ======== Field Constants ========

    /// **INT**
    /// Bits `[11:4]`, mask `0x00000ff0`
    pub const INT_OFFSET: u32 = 4;
    pub const INT_MASK: u32 = 0x00000ff0;

    /// **FRAC**
    /// Bits `[3:0]`, mask `0x0000000f`
    pub const FRAC_OFFSET: u32 = 0;
    pub const FRAC_MASK: u32 = 0x0000000f;

    // ======== Field Accessors ========

    /// **INT** `[11:4]` (RW)
    #[inline(always)]
    pub fn int(&self) -> u32 {
        (self.value.read_bits() & Self::INT_MASK) >> Self::INT_OFFSET
    }

    /// Write **INT** `[11:4]` (RW)
    #[inline(always)]
    pub fn write_int(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::INT_MASK) | ((value << Self::INT_OFFSET) & Self::INT_MASK));
    }

    /// **FRAC** `[3:0]` (RW)
    #[inline(always)]
    pub fn frac(&self) -> u32 {
        (self.value.read_bits() & Self::FRAC_MASK) >> Self::FRAC_OFFSET
    }

    /// Write **FRAC** `[3:0]` (RW)
    #[inline(always)]
    pub fn write_frac(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::FRAC_MASK) | ((value << Self::FRAC_OFFSET) & Self::FRAC_MASK));
    }

}

/// CH10_DIV value type for batch read/modify/write operations
///
/// Use `ch10div.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch10div.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch10DivVal {
    bits: u32,
}

impl Ch10DivVal {
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
        Self { bits: 0x00000010 }
    }

    /// Read **INT** `[11:4]`
    #[inline(always)]
    pub const fn int(&self) -> u32 {
        (self.bits & Ch10Div::INT_MASK) >> Ch10Div::INT_OFFSET
    }

    /// Read **FRAC** `[3:0]`
    #[inline(always)]
    pub const fn frac(&self) -> u32 {
        (self.bits & Ch10Div::FRAC_MASK) >> Ch10Div::FRAC_OFFSET
    }

    /// Set **INT** `[11:4]`
    #[inline(always)]
    pub const fn with_int(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch10Div::INT_MASK) | ((value << Ch10Div::INT_OFFSET) & Ch10Div::INT_MASK)
        }
    }

    /// Set **FRAC** `[3:0]`
    #[inline(always)]
    pub const fn with_frac(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch10Div::FRAC_MASK) | ((value << Ch10Div::FRAC_OFFSET) & Ch10Div::FRAC_MASK)
        }
    }

}

impl Default for Ch10DivVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ch10Div {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch10DivVal {
        Ch10DivVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch10DivVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch10DivVal) -> Ch10DivVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// CH10_CTR Register
///
/// Direct access to the PWM counter
///
/// Address offset: 0x00d0
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CH10_CTR** `[15:0]` (RW)
#[repr(transparent)]
pub struct Ch10Ctr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch10Ctr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch10Ctr {
    // ======== Field Constants ========

    /// **CH10_CTR**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const CH10_CTR_OFFSET: u32 = 0;
    pub const CH10_CTR_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **CH10_CTR** `[15:0]` (RW)
    #[inline(always)]
    pub fn ch10_ctr(&self) -> u32 {
        (self.value.read_bits() & Self::CH10_CTR_MASK) >> Self::CH10_CTR_OFFSET
    }

    /// Write **CH10_CTR** `[15:0]` (RW)
    #[inline(always)]
    pub fn write_ch10_ctr(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::CH10_CTR_MASK) | ((value << Self::CH10_CTR_OFFSET) & Self::CH10_CTR_MASK));
    }

}

/// CH10_CTR value type for batch read/modify/write operations
///
/// Use `ch10ctr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch10ctr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch10CtrVal {
    bits: u32,
}

impl Ch10CtrVal {
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

    /// Read **CH10_CTR** `[15:0]`
    #[inline(always)]
    pub const fn ch10_ctr(&self) -> u32 {
        (self.bits & Ch10Ctr::CH10_CTR_MASK) >> Ch10Ctr::CH10_CTR_OFFSET
    }

    /// Set **CH10_CTR** `[15:0]`
    #[inline(always)]
    pub const fn with_ch10_ctr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch10Ctr::CH10_CTR_MASK) | ((value << Ch10Ctr::CH10_CTR_OFFSET) & Ch10Ctr::CH10_CTR_MASK)
        }
    }

}

impl Default for Ch10CtrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ch10Ctr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch10CtrVal {
        Ch10CtrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch10CtrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch10CtrVal) -> Ch10CtrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// CH10_CC Register
///
/// Counter compare values
///
/// Address offset: 0x00d4
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **B** `[31:16]` (RW)
///
/// - **A** `[15:0]` (RW)
#[repr(transparent)]
pub struct Ch10Cc {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch10Cc {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch10Cc {
    // ======== Field Constants ========

    /// **B**
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const B_OFFSET: u32 = 16;
    pub const B_MASK: u32 = 0xffff0000;

    /// **A**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const A_OFFSET: u32 = 0;
    pub const A_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **B** `[31:16]` (RW)
    #[inline(always)]
    pub fn b(&self) -> u32 {
        (self.value.read_bits() & Self::B_MASK) >> Self::B_OFFSET
    }

    /// Write **B** `[31:16]` (RW)
    #[inline(always)]
    pub fn write_b(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::B_MASK) | ((value << Self::B_OFFSET) & Self::B_MASK));
    }

    /// **A** `[15:0]` (RW)
    #[inline(always)]
    pub fn a(&self) -> u32 {
        (self.value.read_bits() & Self::A_MASK) >> Self::A_OFFSET
    }

    /// Write **A** `[15:0]` (RW)
    #[inline(always)]
    pub fn write_a(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::A_MASK) | ((value << Self::A_OFFSET) & Self::A_MASK));
    }

}

/// CH10_CC value type for batch read/modify/write operations
///
/// Use `ch10cc.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch10cc.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch10CcVal {
    bits: u32,
}

impl Ch10CcVal {
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

    /// Read **B** `[31:16]`
    #[inline(always)]
    pub const fn b(&self) -> u32 {
        (self.bits & Ch10Cc::B_MASK) >> Ch10Cc::B_OFFSET
    }

    /// Read **A** `[15:0]`
    #[inline(always)]
    pub const fn a(&self) -> u32 {
        (self.bits & Ch10Cc::A_MASK) >> Ch10Cc::A_OFFSET
    }

    /// Set **B** `[31:16]`
    #[inline(always)]
    pub const fn with_b(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch10Cc::B_MASK) | ((value << Ch10Cc::B_OFFSET) & Ch10Cc::B_MASK)
        }
    }

    /// Set **A** `[15:0]`
    #[inline(always)]
    pub const fn with_a(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch10Cc::A_MASK) | ((value << Ch10Cc::A_OFFSET) & Ch10Cc::A_MASK)
        }
    }

}

impl Default for Ch10CcVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ch10Cc {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch10CcVal {
        Ch10CcVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch10CcVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch10CcVal) -> Ch10CcVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// CH10_TOP Register
///
/// Counter wrap value
///
/// Address offset: 0x00d8
/// Reset value: 0x0000ffff
///
/// ## Fields
///
/// - **CH10_TOP** `[15:0]` (RW)
#[repr(transparent)]
pub struct Ch10Top {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch10Top {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch10Top {
    // ======== Field Constants ========

    /// **CH10_TOP**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const CH10_TOP_OFFSET: u32 = 0;
    pub const CH10_TOP_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **CH10_TOP** `[15:0]` (RW)
    #[inline(always)]
    pub fn ch10_top(&self) -> u32 {
        (self.value.read_bits() & Self::CH10_TOP_MASK) >> Self::CH10_TOP_OFFSET
    }

    /// Write **CH10_TOP** `[15:0]` (RW)
    #[inline(always)]
    pub fn write_ch10_top(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::CH10_TOP_MASK) | ((value << Self::CH10_TOP_OFFSET) & Self::CH10_TOP_MASK));
    }

}

/// CH10_TOP value type for batch read/modify/write operations
///
/// Use `ch10top.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch10top.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch10TopVal {
    bits: u32,
}

impl Ch10TopVal {
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
        Self { bits: 0x0000ffff }
    }

    /// Read **CH10_TOP** `[15:0]`
    #[inline(always)]
    pub const fn ch10_top(&self) -> u32 {
        (self.bits & Ch10Top::CH10_TOP_MASK) >> Ch10Top::CH10_TOP_OFFSET
    }

    /// Set **CH10_TOP** `[15:0]`
    #[inline(always)]
    pub const fn with_ch10_top(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch10Top::CH10_TOP_MASK) | ((value << Ch10Top::CH10_TOP_OFFSET) & Ch10Top::CH10_TOP_MASK)
        }
    }

}

impl Default for Ch10TopVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ch10Top {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch10TopVal {
        Ch10TopVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch10TopVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch10TopVal) -> Ch10TopVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// CH11_CSR Register
///
/// Control and status register
///
/// Address offset: 0x00dc
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **PH_ADV** `[7]` (WO)
///
///   Advance the phase of the counter by 1 count, while it is running.
///   Self-clearing. Write a 1, and poll until low. Counter must be running
///   at less than full speed (div_int + div_frac / 16 > 1)
///
/// - **PH_RET** `[6]` (WO)
///
///   Retard the phase of the counter by 1 count, while it is running.
///   Self-clearing. Write a 1, and poll until low. Counter must be running.
///
/// - **DIVMODE** `[5:4]` (RW)
///
/// - **B_INV** `[3]` (RW)
///
///   Invert output B
///
/// - **A_INV** `[2]` (RW)
///
///   Invert output A
///
/// - **PH_CORRECT** `[1]` (RW)
///
///   1: Enable phase-correct modulation. 0: Trailing-edge
///
/// - **EN** `[0]` (RW)
///
///   Enable the PWM channel.
#[repr(transparent)]
pub struct Ch11Csr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch11Csr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch11Csr {
    // ======== Field Constants ========

    /// **PH_ADV**
    ///
    /// Advance the phase of the counter by 1 count, while it is running.
    /// Self-clearing. Write a 1, and poll until low. Counter must be running
    /// at less than full speed (div_int + div_frac / 16 > 1)
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const PH_ADV_OFFSET: u32 = 7;
    pub const PH_ADV_MASK: u32 = 0x00000080;
    pub const PH_ADV: u32 = 1 << 7;

    /// **PH_RET**
    ///
    /// Retard the phase of the counter by 1 count, while it is running.
    /// Self-clearing. Write a 1, and poll until low. Counter must be running.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const PH_RET_OFFSET: u32 = 6;
    pub const PH_RET_MASK: u32 = 0x00000040;
    pub const PH_RET: u32 = 1 << 6;

    /// **DIVMODE**
    /// Bits `[5:4]`, mask `0x00000030`
    pub const DIVMODE_OFFSET: u32 = 4;
    pub const DIVMODE_MASK: u32 = 0x00000030;
    /// div = 0
    /// Free-running counting at rate dictated by fractional divider
    pub const DIVMODE_DIV: u32 = 0 << 4;
    pub const DIVMODE_DIV_VAL: u32 = 0;
    /// level = 1
    /// Fractional divider operation is gated by the PWM B pin.
    pub const DIVMODE_LEVEL: u32 = 1 << 4;
    pub const DIVMODE_LEVEL_VAL: u32 = 1;
    /// rise = 2
    /// Counter advances with each rising edge of the PWM B pin.
    pub const DIVMODE_RISE: u32 = 2 << 4;
    pub const DIVMODE_RISE_VAL: u32 = 2;
    /// fall = 3
    /// Counter advances with each falling edge of the PWM B pin.
    pub const DIVMODE_FALL: u32 = 3 << 4;
    pub const DIVMODE_FALL_VAL: u32 = 3;

    /// **B_INV**
    ///
    /// Invert output B
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const B_INV_OFFSET: u32 = 3;
    pub const B_INV_MASK: u32 = 0x00000008;
    pub const B_INV: u32 = 1 << 3;

    /// **A_INV**
    ///
    /// Invert output A
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const A_INV_OFFSET: u32 = 2;
    pub const A_INV_MASK: u32 = 0x00000004;
    pub const A_INV: u32 = 1 << 2;

    /// **PH_CORRECT**
    ///
    /// 1: Enable phase-correct modulation. 0: Trailing-edge
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const PH_CORRECT_OFFSET: u32 = 1;
    pub const PH_CORRECT_MASK: u32 = 0x00000002;
    pub const PH_CORRECT: u32 = 1 << 1;

    /// **EN**
    ///
    /// Enable the PWM channel.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const EN_OFFSET: u32 = 0;
    pub const EN_MASK: u32 = 0x00000001;
    pub const EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **PH_ADV** (WO)
    ///
    /// Advance the phase of the counter by 1 count, while it is running.
    /// Self-clearing. Write a 1, and poll until low. Counter must be running
    /// at less than full speed (div_int + div_frac / 16 > 1)
    #[inline(always)]
    pub fn trigger_ph_adv(&self) {
        self.value.write_bits(Self::PH_ADV_MASK);
    }

    /// Trigger **PH_RET** (WO)
    ///
    /// Retard the phase of the counter by 1 count, while it is running.
    /// Self-clearing. Write a 1, and poll until low. Counter must be running.
    #[inline(always)]
    pub fn trigger_ph_ret(&self) {
        self.value.write_bits(Self::PH_RET_MASK);
    }

    /// **DIVMODE** `[5:4]` (RW)
    #[inline(always)]
    pub fn divmode(&self) -> u32 {
        (self.value.read_bits() & Self::DIVMODE_MASK) >> Self::DIVMODE_OFFSET
    }

    /// Write **DIVMODE** `[5:4]` (RW)
    #[inline(always)]
    pub fn write_divmode(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DIVMODE_MASK) | ((value << Self::DIVMODE_OFFSET) & Self::DIVMODE_MASK));
    }

    /// **B_INV** `[3]` (RW)
    ///
    /// Invert output B
    #[inline(always)]
    pub fn b_inv(&self) -> bool {
        (self.value.read_bits() & Self::B_INV_MASK) != 0
    }

    /// Set B_INV bit atomically (RW)
    #[inline(always)]
    pub fn set_b_inv(&self) {
        self.value.set_bits(Self::B_INV_MASK);
    }

    /// Clear B_INV bit atomically (RW)
    #[inline(always)]
    pub fn clr_b_inv(&self) {
        self.value.clr_bits(Self::B_INV_MASK);
    }

    /// **A_INV** `[2]` (RW)
    ///
    /// Invert output A
    #[inline(always)]
    pub fn a_inv(&self) -> bool {
        (self.value.read_bits() & Self::A_INV_MASK) != 0
    }

    /// Set A_INV bit atomically (RW)
    #[inline(always)]
    pub fn set_a_inv(&self) {
        self.value.set_bits(Self::A_INV_MASK);
    }

    /// Clear A_INV bit atomically (RW)
    #[inline(always)]
    pub fn clr_a_inv(&self) {
        self.value.clr_bits(Self::A_INV_MASK);
    }

    /// **PH_CORRECT** `[1]` (RW)
    ///
    /// 1: Enable phase-correct modulation. 0: Trailing-edge
    #[inline(always)]
    pub fn ph_correct(&self) -> bool {
        (self.value.read_bits() & Self::PH_CORRECT_MASK) != 0
    }

    /// Set PH_CORRECT bit atomically (RW)
    #[inline(always)]
    pub fn set_ph_correct(&self) {
        self.value.set_bits(Self::PH_CORRECT_MASK);
    }

    /// Clear PH_CORRECT bit atomically (RW)
    #[inline(always)]
    pub fn clr_ph_correct(&self) {
        self.value.clr_bits(Self::PH_CORRECT_MASK);
    }

    /// **EN** `[0]` (RW)
    ///
    /// Enable the PWM channel.
    #[inline(always)]
    pub fn en(&self) -> bool {
        (self.value.read_bits() & Self::EN_MASK) != 0
    }

    /// Set EN bit atomically (RW)
    #[inline(always)]
    pub fn set_en(&self) {
        self.value.set_bits(Self::EN_MASK);
    }

    /// Clear EN bit atomically (RW)
    #[inline(always)]
    pub fn clr_en(&self) {
        self.value.clr_bits(Self::EN_MASK);
    }

}

/// CH11_CSR value type for batch read/modify/write operations
///
/// Use `ch11csr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch11csr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch11CsrVal {
    bits: u32,
}

impl Ch11CsrVal {
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

    /// Read **DIVMODE** `[5:4]`
    #[inline(always)]
    pub const fn divmode(&self) -> u32 {
        (self.bits & Ch11Csr::DIVMODE_MASK) >> Ch11Csr::DIVMODE_OFFSET
    }

    /// Read **B_INV** `[3:3]`
    ///
    /// Invert output B
    #[inline(always)]
    pub const fn b_inv(&self) -> bool {
        (self.bits & Ch11Csr::B_INV_MASK) != 0
    }

    /// Read **A_INV** `[2:2]`
    ///
    /// Invert output A
    #[inline(always)]
    pub const fn a_inv(&self) -> bool {
        (self.bits & Ch11Csr::A_INV_MASK) != 0
    }

    /// Read **PH_CORRECT** `[1:1]`
    ///
    /// 1: Enable phase-correct modulation. 0: Trailing-edge
    #[inline(always)]
    pub const fn ph_correct(&self) -> bool {
        (self.bits & Ch11Csr::PH_CORRECT_MASK) != 0
    }

    /// Read **EN** `[0:0]`
    ///
    /// Enable the PWM channel.
    #[inline(always)]
    pub const fn en(&self) -> bool {
        (self.bits & Ch11Csr::EN_MASK) != 0
    }

    /// Set **PH_ADV** `[7:7]`
    ///
    /// Advance the phase of the counter by 1 count, while it is running.
    /// Self-clearing. Write a 1, and poll until low. Counter must be running
    /// at less than full speed (div_int + div_frac / 16 > 1)
    #[inline(always)]
    pub const fn with_ph_adv(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch11Csr::PH_ADV_MASK
            } else {
                self.bits & !Ch11Csr::PH_ADV_MASK
            }
        }
    }

    /// Set **PH_RET** `[6:6]`
    ///
    /// Retard the phase of the counter by 1 count, while it is running.
    /// Self-clearing. Write a 1, and poll until low. Counter must be running.
    #[inline(always)]
    pub const fn with_ph_ret(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch11Csr::PH_RET_MASK
            } else {
                self.bits & !Ch11Csr::PH_RET_MASK
            }
        }
    }

    /// Set **DIVMODE** `[5:4]`
    #[inline(always)]
    pub const fn with_divmode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch11Csr::DIVMODE_MASK) | ((value << Ch11Csr::DIVMODE_OFFSET) & Ch11Csr::DIVMODE_MASK)
        }
    }

    /// Set **B_INV** `[3:3]`
    ///
    /// Invert output B
    #[inline(always)]
    pub const fn with_b_inv(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch11Csr::B_INV_MASK
            } else {
                self.bits & !Ch11Csr::B_INV_MASK
            }
        }
    }

    /// Set **A_INV** `[2:2]`
    ///
    /// Invert output A
    #[inline(always)]
    pub const fn with_a_inv(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch11Csr::A_INV_MASK
            } else {
                self.bits & !Ch11Csr::A_INV_MASK
            }
        }
    }

    /// Set **PH_CORRECT** `[1:1]`
    ///
    /// 1: Enable phase-correct modulation. 0: Trailing-edge
    #[inline(always)]
    pub const fn with_ph_correct(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch11Csr::PH_CORRECT_MASK
            } else {
                self.bits & !Ch11Csr::PH_CORRECT_MASK
            }
        }
    }

    /// Set **EN** `[0:0]`
    ///
    /// Enable the PWM channel.
    #[inline(always)]
    pub const fn with_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ch11Csr::EN_MASK
            } else {
                self.bits & !Ch11Csr::EN_MASK
            }
        }
    }

}

impl Default for Ch11CsrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ch11Csr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch11CsrVal {
        Ch11CsrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch11CsrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch11CsrVal) -> Ch11CsrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// CH11_DIV Register
///
/// INT and FRAC form a fixed-point fractional number.
/// Counting rate is system clock frequency divided by this number.
/// Fractional division uses simple 1st-order sigma-delta.
///
/// Address offset: 0x00e0
/// Reset value: 0x00000010
///
/// ## Fields
///
/// - **INT** `[11:4]` (RW)
///
/// - **FRAC** `[3:0]` (RW)
#[repr(transparent)]
pub struct Ch11Div {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch11Div {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch11Div {
    // ======== Field Constants ========

    /// **INT**
    /// Bits `[11:4]`, mask `0x00000ff0`
    pub const INT_OFFSET: u32 = 4;
    pub const INT_MASK: u32 = 0x00000ff0;

    /// **FRAC**
    /// Bits `[3:0]`, mask `0x0000000f`
    pub const FRAC_OFFSET: u32 = 0;
    pub const FRAC_MASK: u32 = 0x0000000f;

    // ======== Field Accessors ========

    /// **INT** `[11:4]` (RW)
    #[inline(always)]
    pub fn int(&self) -> u32 {
        (self.value.read_bits() & Self::INT_MASK) >> Self::INT_OFFSET
    }

    /// Write **INT** `[11:4]` (RW)
    #[inline(always)]
    pub fn write_int(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::INT_MASK) | ((value << Self::INT_OFFSET) & Self::INT_MASK));
    }

    /// **FRAC** `[3:0]` (RW)
    #[inline(always)]
    pub fn frac(&self) -> u32 {
        (self.value.read_bits() & Self::FRAC_MASK) >> Self::FRAC_OFFSET
    }

    /// Write **FRAC** `[3:0]` (RW)
    #[inline(always)]
    pub fn write_frac(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::FRAC_MASK) | ((value << Self::FRAC_OFFSET) & Self::FRAC_MASK));
    }

}

/// CH11_DIV value type for batch read/modify/write operations
///
/// Use `ch11div.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch11div.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch11DivVal {
    bits: u32,
}

impl Ch11DivVal {
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
        Self { bits: 0x00000010 }
    }

    /// Read **INT** `[11:4]`
    #[inline(always)]
    pub const fn int(&self) -> u32 {
        (self.bits & Ch11Div::INT_MASK) >> Ch11Div::INT_OFFSET
    }

    /// Read **FRAC** `[3:0]`
    #[inline(always)]
    pub const fn frac(&self) -> u32 {
        (self.bits & Ch11Div::FRAC_MASK) >> Ch11Div::FRAC_OFFSET
    }

    /// Set **INT** `[11:4]`
    #[inline(always)]
    pub const fn with_int(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch11Div::INT_MASK) | ((value << Ch11Div::INT_OFFSET) & Ch11Div::INT_MASK)
        }
    }

    /// Set **FRAC** `[3:0]`
    #[inline(always)]
    pub const fn with_frac(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch11Div::FRAC_MASK) | ((value << Ch11Div::FRAC_OFFSET) & Ch11Div::FRAC_MASK)
        }
    }

}

impl Default for Ch11DivVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ch11Div {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch11DivVal {
        Ch11DivVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch11DivVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch11DivVal) -> Ch11DivVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// CH11_CTR Register
///
/// Direct access to the PWM counter
///
/// Address offset: 0x00e4
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CH11_CTR** `[15:0]` (RW)
#[repr(transparent)]
pub struct Ch11Ctr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch11Ctr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch11Ctr {
    // ======== Field Constants ========

    /// **CH11_CTR**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const CH11_CTR_OFFSET: u32 = 0;
    pub const CH11_CTR_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **CH11_CTR** `[15:0]` (RW)
    #[inline(always)]
    pub fn ch11_ctr(&self) -> u32 {
        (self.value.read_bits() & Self::CH11_CTR_MASK) >> Self::CH11_CTR_OFFSET
    }

    /// Write **CH11_CTR** `[15:0]` (RW)
    #[inline(always)]
    pub fn write_ch11_ctr(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::CH11_CTR_MASK) | ((value << Self::CH11_CTR_OFFSET) & Self::CH11_CTR_MASK));
    }

}

/// CH11_CTR value type for batch read/modify/write operations
///
/// Use `ch11ctr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch11ctr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch11CtrVal {
    bits: u32,
}

impl Ch11CtrVal {
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

    /// Read **CH11_CTR** `[15:0]`
    #[inline(always)]
    pub const fn ch11_ctr(&self) -> u32 {
        (self.bits & Ch11Ctr::CH11_CTR_MASK) >> Ch11Ctr::CH11_CTR_OFFSET
    }

    /// Set **CH11_CTR** `[15:0]`
    #[inline(always)]
    pub const fn with_ch11_ctr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch11Ctr::CH11_CTR_MASK) | ((value << Ch11Ctr::CH11_CTR_OFFSET) & Ch11Ctr::CH11_CTR_MASK)
        }
    }

}

impl Default for Ch11CtrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ch11Ctr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch11CtrVal {
        Ch11CtrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch11CtrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch11CtrVal) -> Ch11CtrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// CH11_CC Register
///
/// Counter compare values
///
/// Address offset: 0x00e8
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **B** `[31:16]` (RW)
///
/// - **A** `[15:0]` (RW)
#[repr(transparent)]
pub struct Ch11Cc {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch11Cc {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch11Cc {
    // ======== Field Constants ========

    /// **B**
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const B_OFFSET: u32 = 16;
    pub const B_MASK: u32 = 0xffff0000;

    /// **A**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const A_OFFSET: u32 = 0;
    pub const A_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **B** `[31:16]` (RW)
    #[inline(always)]
    pub fn b(&self) -> u32 {
        (self.value.read_bits() & Self::B_MASK) >> Self::B_OFFSET
    }

    /// Write **B** `[31:16]` (RW)
    #[inline(always)]
    pub fn write_b(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::B_MASK) | ((value << Self::B_OFFSET) & Self::B_MASK));
    }

    /// **A** `[15:0]` (RW)
    #[inline(always)]
    pub fn a(&self) -> u32 {
        (self.value.read_bits() & Self::A_MASK) >> Self::A_OFFSET
    }

    /// Write **A** `[15:0]` (RW)
    #[inline(always)]
    pub fn write_a(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::A_MASK) | ((value << Self::A_OFFSET) & Self::A_MASK));
    }

}

/// CH11_CC value type for batch read/modify/write operations
///
/// Use `ch11cc.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch11cc.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch11CcVal {
    bits: u32,
}

impl Ch11CcVal {
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

    /// Read **B** `[31:16]`
    #[inline(always)]
    pub const fn b(&self) -> u32 {
        (self.bits & Ch11Cc::B_MASK) >> Ch11Cc::B_OFFSET
    }

    /// Read **A** `[15:0]`
    #[inline(always)]
    pub const fn a(&self) -> u32 {
        (self.bits & Ch11Cc::A_MASK) >> Ch11Cc::A_OFFSET
    }

    /// Set **B** `[31:16]`
    #[inline(always)]
    pub const fn with_b(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch11Cc::B_MASK) | ((value << Ch11Cc::B_OFFSET) & Ch11Cc::B_MASK)
        }
    }

    /// Set **A** `[15:0]`
    #[inline(always)]
    pub const fn with_a(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch11Cc::A_MASK) | ((value << Ch11Cc::A_OFFSET) & Ch11Cc::A_MASK)
        }
    }

}

impl Default for Ch11CcVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ch11Cc {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch11CcVal {
        Ch11CcVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch11CcVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch11CcVal) -> Ch11CcVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// CH11_TOP Register
///
/// Counter wrap value
///
/// Address offset: 0x00ec
/// Reset value: 0x0000ffff
///
/// ## Fields
///
/// - **CH11_TOP** `[15:0]` (RW)
#[repr(transparent)]
pub struct Ch11Top {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ch11Top {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ch11Top {
    // ======== Field Constants ========

    /// **CH11_TOP**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const CH11_TOP_OFFSET: u32 = 0;
    pub const CH11_TOP_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **CH11_TOP** `[15:0]` (RW)
    #[inline(always)]
    pub fn ch11_top(&self) -> u32 {
        (self.value.read_bits() & Self::CH11_TOP_MASK) >> Self::CH11_TOP_OFFSET
    }

    /// Write **CH11_TOP** `[15:0]` (RW)
    #[inline(always)]
    pub fn write_ch11_top(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::CH11_TOP_MASK) | ((value << Self::CH11_TOP_OFFSET) & Self::CH11_TOP_MASK));
    }

}

/// CH11_TOP value type for batch read/modify/write operations
///
/// Use `ch11top.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ch11top.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ch11TopVal {
    bits: u32,
}

impl Ch11TopVal {
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
        Self { bits: 0x0000ffff }
    }

    /// Read **CH11_TOP** `[15:0]`
    #[inline(always)]
    pub const fn ch11_top(&self) -> u32 {
        (self.bits & Ch11Top::CH11_TOP_MASK) >> Ch11Top::CH11_TOP_OFFSET
    }

    /// Set **CH11_TOP** `[15:0]`
    #[inline(always)]
    pub const fn with_ch11_top(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ch11Top::CH11_TOP_MASK) | ((value << Ch11Top::CH11_TOP_OFFSET) & Ch11Top::CH11_TOP_MASK)
        }
    }

}

impl Default for Ch11TopVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ch11Top {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ch11TopVal {
        Ch11TopVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ch11TopVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ch11TopVal) -> Ch11TopVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// EN Register
///
/// This register aliases the CSR_EN bits for all channels.
/// Writing to this register allows multiple channels to be enabled
/// or disabled simultaneously, so they can run in perfect sync.
/// For each channel, there is only one physical EN register bit,
/// which can be accessed through here or CHx_CSR.
///
/// Address offset: 0x00f0
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CH11** `[11]` (RW)
///
/// - **CH10** `[10]` (RW)
///
/// - **CH9** `[9]` (RW)
///
/// - **CH8** `[8]` (RW)
///
/// - **CH7** `[7]` (RW)
///
/// - **CH6** `[6]` (RW)
///
/// - **CH5** `[5]` (RW)
///
/// - **CH4** `[4]` (RW)
///
/// - **CH3** `[3]` (RW)
///
/// - **CH2** `[2]` (RW)
///
/// - **CH1** `[1]` (RW)
///
/// - **CH0** `[0]` (RW)
#[repr(transparent)]
pub struct En {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for En {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl En {
    // ======== Field Constants ========

    /// **CH11**
    /// Bits `[11:11]`, mask `0x00000800`
    pub const CH11_OFFSET: u32 = 11;
    pub const CH11_MASK: u32 = 0x00000800;
    pub const CH11: u32 = 1 << 11;

    /// **CH10**
    /// Bits `[10:10]`, mask `0x00000400`
    pub const CH10_OFFSET: u32 = 10;
    pub const CH10_MASK: u32 = 0x00000400;
    pub const CH10: u32 = 1 << 10;

    /// **CH9**
    /// Bits `[9:9]`, mask `0x00000200`
    pub const CH9_OFFSET: u32 = 9;
    pub const CH9_MASK: u32 = 0x00000200;
    pub const CH9: u32 = 1 << 9;

    /// **CH8**
    /// Bits `[8:8]`, mask `0x00000100`
    pub const CH8_OFFSET: u32 = 8;
    pub const CH8_MASK: u32 = 0x00000100;
    pub const CH8: u32 = 1 << 8;

    /// **CH7**
    /// Bits `[7:7]`, mask `0x00000080`
    pub const CH7_OFFSET: u32 = 7;
    pub const CH7_MASK: u32 = 0x00000080;
    pub const CH7: u32 = 1 << 7;

    /// **CH6**
    /// Bits `[6:6]`, mask `0x00000040`
    pub const CH6_OFFSET: u32 = 6;
    pub const CH6_MASK: u32 = 0x00000040;
    pub const CH6: u32 = 1 << 6;

    /// **CH5**
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CH5_OFFSET: u32 = 5;
    pub const CH5_MASK: u32 = 0x00000020;
    pub const CH5: u32 = 1 << 5;

    /// **CH4**
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CH4_OFFSET: u32 = 4;
    pub const CH4_MASK: u32 = 0x00000010;
    pub const CH4: u32 = 1 << 4;

    /// **CH3**
    /// Bits `[3:3]`, mask `0x00000008`
    pub const CH3_OFFSET: u32 = 3;
    pub const CH3_MASK: u32 = 0x00000008;
    pub const CH3: u32 = 1 << 3;

    /// **CH2**
    /// Bits `[2:2]`, mask `0x00000004`
    pub const CH2_OFFSET: u32 = 2;
    pub const CH2_MASK: u32 = 0x00000004;
    pub const CH2: u32 = 1 << 2;

    /// **CH1**
    /// Bits `[1:1]`, mask `0x00000002`
    pub const CH1_OFFSET: u32 = 1;
    pub const CH1_MASK: u32 = 0x00000002;
    pub const CH1: u32 = 1 << 1;

    /// **CH0**
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CH0_OFFSET: u32 = 0;
    pub const CH0_MASK: u32 = 0x00000001;
    pub const CH0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CH11** `[11]` (RW)
    #[inline(always)]
    pub fn ch11(&self) -> bool {
        (self.value.read_bits() & Self::CH11_MASK) != 0
    }

    /// Set CH11 bit atomically (RW)
    #[inline(always)]
    pub fn set_ch11(&self) {
        self.value.set_bits(Self::CH11_MASK);
    }

    /// Clear CH11 bit atomically (RW)
    #[inline(always)]
    pub fn clr_ch11(&self) {
        self.value.clr_bits(Self::CH11_MASK);
    }

    /// **CH10** `[10]` (RW)
    #[inline(always)]
    pub fn ch10(&self) -> bool {
        (self.value.read_bits() & Self::CH10_MASK) != 0
    }

    /// Set CH10 bit atomically (RW)
    #[inline(always)]
    pub fn set_ch10(&self) {
        self.value.set_bits(Self::CH10_MASK);
    }

    /// Clear CH10 bit atomically (RW)
    #[inline(always)]
    pub fn clr_ch10(&self) {
        self.value.clr_bits(Self::CH10_MASK);
    }

    /// **CH9** `[9]` (RW)
    #[inline(always)]
    pub fn ch9(&self) -> bool {
        (self.value.read_bits() & Self::CH9_MASK) != 0
    }

    /// Set CH9 bit atomically (RW)
    #[inline(always)]
    pub fn set_ch9(&self) {
        self.value.set_bits(Self::CH9_MASK);
    }

    /// Clear CH9 bit atomically (RW)
    #[inline(always)]
    pub fn clr_ch9(&self) {
        self.value.clr_bits(Self::CH9_MASK);
    }

    /// **CH8** `[8]` (RW)
    #[inline(always)]
    pub fn ch8(&self) -> bool {
        (self.value.read_bits() & Self::CH8_MASK) != 0
    }

    /// Set CH8 bit atomically (RW)
    #[inline(always)]
    pub fn set_ch8(&self) {
        self.value.set_bits(Self::CH8_MASK);
    }

    /// Clear CH8 bit atomically (RW)
    #[inline(always)]
    pub fn clr_ch8(&self) {
        self.value.clr_bits(Self::CH8_MASK);
    }

    /// **CH7** `[7]` (RW)
    #[inline(always)]
    pub fn ch7(&self) -> bool {
        (self.value.read_bits() & Self::CH7_MASK) != 0
    }

    /// Set CH7 bit atomically (RW)
    #[inline(always)]
    pub fn set_ch7(&self) {
        self.value.set_bits(Self::CH7_MASK);
    }

    /// Clear CH7 bit atomically (RW)
    #[inline(always)]
    pub fn clr_ch7(&self) {
        self.value.clr_bits(Self::CH7_MASK);
    }

    /// **CH6** `[6]` (RW)
    #[inline(always)]
    pub fn ch6(&self) -> bool {
        (self.value.read_bits() & Self::CH6_MASK) != 0
    }

    /// Set CH6 bit atomically (RW)
    #[inline(always)]
    pub fn set_ch6(&self) {
        self.value.set_bits(Self::CH6_MASK);
    }

    /// Clear CH6 bit atomically (RW)
    #[inline(always)]
    pub fn clr_ch6(&self) {
        self.value.clr_bits(Self::CH6_MASK);
    }

    /// **CH5** `[5]` (RW)
    #[inline(always)]
    pub fn ch5(&self) -> bool {
        (self.value.read_bits() & Self::CH5_MASK) != 0
    }

    /// Set CH5 bit atomically (RW)
    #[inline(always)]
    pub fn set_ch5(&self) {
        self.value.set_bits(Self::CH5_MASK);
    }

    /// Clear CH5 bit atomically (RW)
    #[inline(always)]
    pub fn clr_ch5(&self) {
        self.value.clr_bits(Self::CH5_MASK);
    }

    /// **CH4** `[4]` (RW)
    #[inline(always)]
    pub fn ch4(&self) -> bool {
        (self.value.read_bits() & Self::CH4_MASK) != 0
    }

    /// Set CH4 bit atomically (RW)
    #[inline(always)]
    pub fn set_ch4(&self) {
        self.value.set_bits(Self::CH4_MASK);
    }

    /// Clear CH4 bit atomically (RW)
    #[inline(always)]
    pub fn clr_ch4(&self) {
        self.value.clr_bits(Self::CH4_MASK);
    }

    /// **CH3** `[3]` (RW)
    #[inline(always)]
    pub fn ch3(&self) -> bool {
        (self.value.read_bits() & Self::CH3_MASK) != 0
    }

    /// Set CH3 bit atomically (RW)
    #[inline(always)]
    pub fn set_ch3(&self) {
        self.value.set_bits(Self::CH3_MASK);
    }

    /// Clear CH3 bit atomically (RW)
    #[inline(always)]
    pub fn clr_ch3(&self) {
        self.value.clr_bits(Self::CH3_MASK);
    }

    /// **CH2** `[2]` (RW)
    #[inline(always)]
    pub fn ch2(&self) -> bool {
        (self.value.read_bits() & Self::CH2_MASK) != 0
    }

    /// Set CH2 bit atomically (RW)
    #[inline(always)]
    pub fn set_ch2(&self) {
        self.value.set_bits(Self::CH2_MASK);
    }

    /// Clear CH2 bit atomically (RW)
    #[inline(always)]
    pub fn clr_ch2(&self) {
        self.value.clr_bits(Self::CH2_MASK);
    }

    /// **CH1** `[1]` (RW)
    #[inline(always)]
    pub fn ch1(&self) -> bool {
        (self.value.read_bits() & Self::CH1_MASK) != 0
    }

    /// Set CH1 bit atomically (RW)
    #[inline(always)]
    pub fn set_ch1(&self) {
        self.value.set_bits(Self::CH1_MASK);
    }

    /// Clear CH1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_ch1(&self) {
        self.value.clr_bits(Self::CH1_MASK);
    }

    /// **CH0** `[0]` (RW)
    #[inline(always)]
    pub fn ch0(&self) -> bool {
        (self.value.read_bits() & Self::CH0_MASK) != 0
    }

    /// Set CH0 bit atomically (RW)
    #[inline(always)]
    pub fn set_ch0(&self) {
        self.value.set_bits(Self::CH0_MASK);
    }

    /// Clear CH0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_ch0(&self) {
        self.value.clr_bits(Self::CH0_MASK);
    }

}

/// EN value type for batch read/modify/write operations
///
/// Use `en.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `en.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EnVal {
    bits: u32,
}

impl EnVal {
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

    /// Read **CH11** `[11:11]`
    #[inline(always)]
    pub const fn ch11(&self) -> bool {
        (self.bits & En::CH11_MASK) != 0
    }

    /// Read **CH10** `[10:10]`
    #[inline(always)]
    pub const fn ch10(&self) -> bool {
        (self.bits & En::CH10_MASK) != 0
    }

    /// Read **CH9** `[9:9]`
    #[inline(always)]
    pub const fn ch9(&self) -> bool {
        (self.bits & En::CH9_MASK) != 0
    }

    /// Read **CH8** `[8:8]`
    #[inline(always)]
    pub const fn ch8(&self) -> bool {
        (self.bits & En::CH8_MASK) != 0
    }

    /// Read **CH7** `[7:7]`
    #[inline(always)]
    pub const fn ch7(&self) -> bool {
        (self.bits & En::CH7_MASK) != 0
    }

    /// Read **CH6** `[6:6]`
    #[inline(always)]
    pub const fn ch6(&self) -> bool {
        (self.bits & En::CH6_MASK) != 0
    }

    /// Read **CH5** `[5:5]`
    #[inline(always)]
    pub const fn ch5(&self) -> bool {
        (self.bits & En::CH5_MASK) != 0
    }

    /// Read **CH4** `[4:4]`
    #[inline(always)]
    pub const fn ch4(&self) -> bool {
        (self.bits & En::CH4_MASK) != 0
    }

    /// Read **CH3** `[3:3]`
    #[inline(always)]
    pub const fn ch3(&self) -> bool {
        (self.bits & En::CH3_MASK) != 0
    }

    /// Read **CH2** `[2:2]`
    #[inline(always)]
    pub const fn ch2(&self) -> bool {
        (self.bits & En::CH2_MASK) != 0
    }

    /// Read **CH1** `[1:1]`
    #[inline(always)]
    pub const fn ch1(&self) -> bool {
        (self.bits & En::CH1_MASK) != 0
    }

    /// Read **CH0** `[0:0]`
    #[inline(always)]
    pub const fn ch0(&self) -> bool {
        (self.bits & En::CH0_MASK) != 0
    }

    /// Set **CH11** `[11:11]`
    #[inline(always)]
    pub const fn with_ch11(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | En::CH11_MASK
            } else {
                self.bits & !En::CH11_MASK
            }
        }
    }

    /// Set **CH10** `[10:10]`
    #[inline(always)]
    pub const fn with_ch10(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | En::CH10_MASK
            } else {
                self.bits & !En::CH10_MASK
            }
        }
    }

    /// Set **CH9** `[9:9]`
    #[inline(always)]
    pub const fn with_ch9(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | En::CH9_MASK
            } else {
                self.bits & !En::CH9_MASK
            }
        }
    }

    /// Set **CH8** `[8:8]`
    #[inline(always)]
    pub const fn with_ch8(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | En::CH8_MASK
            } else {
                self.bits & !En::CH8_MASK
            }
        }
    }

    /// Set **CH7** `[7:7]`
    #[inline(always)]
    pub const fn with_ch7(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | En::CH7_MASK
            } else {
                self.bits & !En::CH7_MASK
            }
        }
    }

    /// Set **CH6** `[6:6]`
    #[inline(always)]
    pub const fn with_ch6(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | En::CH6_MASK
            } else {
                self.bits & !En::CH6_MASK
            }
        }
    }

    /// Set **CH5** `[5:5]`
    #[inline(always)]
    pub const fn with_ch5(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | En::CH5_MASK
            } else {
                self.bits & !En::CH5_MASK
            }
        }
    }

    /// Set **CH4** `[4:4]`
    #[inline(always)]
    pub const fn with_ch4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | En::CH4_MASK
            } else {
                self.bits & !En::CH4_MASK
            }
        }
    }

    /// Set **CH3** `[3:3]`
    #[inline(always)]
    pub const fn with_ch3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | En::CH3_MASK
            } else {
                self.bits & !En::CH3_MASK
            }
        }
    }

    /// Set **CH2** `[2:2]`
    #[inline(always)]
    pub const fn with_ch2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | En::CH2_MASK
            } else {
                self.bits & !En::CH2_MASK
            }
        }
    }

    /// Set **CH1** `[1:1]`
    #[inline(always)]
    pub const fn with_ch1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | En::CH1_MASK
            } else {
                self.bits & !En::CH1_MASK
            }
        }
    }

    /// Set **CH0** `[0:0]`
    #[inline(always)]
    pub const fn with_ch0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | En::CH0_MASK
            } else {
                self.bits & !En::CH0_MASK
            }
        }
    }

}

impl Default for EnVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl En {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EnVal {
        EnVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EnVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EnVal) -> EnVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// INTR Register
///
/// Raw Interrupts
///
/// Address offset: 0x00f4
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CH11** `[11]` (RW)
///
/// - **CH10** `[10]` (RW)
///
/// - **CH9** `[9]` (RW)
///
/// - **CH8** `[8]` (RW)
///
/// - **CH7** `[7]` (RW)
///
/// - **CH6** `[6]` (RW)
///
/// - **CH5** `[5]` (RW)
///
/// - **CH4** `[4]` (RW)
///
/// - **CH3** `[3]` (RW)
///
/// - **CH2** `[2]` (RW)
///
/// - **CH1** `[1]` (RW)
///
/// - **CH0** `[0]` (RW)
#[repr(transparent)]
pub struct Intr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Intr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Intr {
    // ======== Field Constants ========

    /// **CH11**
    /// Bits `[11:11]`, mask `0x00000800`
    pub const CH11_OFFSET: u32 = 11;
    pub const CH11_MASK: u32 = 0x00000800;
    pub const CH11: u32 = 1 << 11;

    /// **CH10**
    /// Bits `[10:10]`, mask `0x00000400`
    pub const CH10_OFFSET: u32 = 10;
    pub const CH10_MASK: u32 = 0x00000400;
    pub const CH10: u32 = 1 << 10;

    /// **CH9**
    /// Bits `[9:9]`, mask `0x00000200`
    pub const CH9_OFFSET: u32 = 9;
    pub const CH9_MASK: u32 = 0x00000200;
    pub const CH9: u32 = 1 << 9;

    /// **CH8**
    /// Bits `[8:8]`, mask `0x00000100`
    pub const CH8_OFFSET: u32 = 8;
    pub const CH8_MASK: u32 = 0x00000100;
    pub const CH8: u32 = 1 << 8;

    /// **CH7**
    /// Bits `[7:7]`, mask `0x00000080`
    pub const CH7_OFFSET: u32 = 7;
    pub const CH7_MASK: u32 = 0x00000080;
    pub const CH7: u32 = 1 << 7;

    /// **CH6**
    /// Bits `[6:6]`, mask `0x00000040`
    pub const CH6_OFFSET: u32 = 6;
    pub const CH6_MASK: u32 = 0x00000040;
    pub const CH6: u32 = 1 << 6;

    /// **CH5**
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CH5_OFFSET: u32 = 5;
    pub const CH5_MASK: u32 = 0x00000020;
    pub const CH5: u32 = 1 << 5;

    /// **CH4**
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CH4_OFFSET: u32 = 4;
    pub const CH4_MASK: u32 = 0x00000010;
    pub const CH4: u32 = 1 << 4;

    /// **CH3**
    /// Bits `[3:3]`, mask `0x00000008`
    pub const CH3_OFFSET: u32 = 3;
    pub const CH3_MASK: u32 = 0x00000008;
    pub const CH3: u32 = 1 << 3;

    /// **CH2**
    /// Bits `[2:2]`, mask `0x00000004`
    pub const CH2_OFFSET: u32 = 2;
    pub const CH2_MASK: u32 = 0x00000004;
    pub const CH2: u32 = 1 << 2;

    /// **CH1**
    /// Bits `[1:1]`, mask `0x00000002`
    pub const CH1_OFFSET: u32 = 1;
    pub const CH1_MASK: u32 = 0x00000002;
    pub const CH1: u32 = 1 << 1;

    /// **CH0**
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CH0_OFFSET: u32 = 0;
    pub const CH0_MASK: u32 = 0x00000001;
    pub const CH0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CH11** `[11]` (RW)
    #[inline(always)]
    pub fn ch11(&self) -> bool {
        (self.value.read_bits() & Self::CH11_MASK) != 0
    }

    /// Set CH11 bit atomically (RW)
    #[inline(always)]
    pub fn set_ch11(&self) {
        self.value.set_bits(Self::CH11_MASK);
    }

    /// Clear CH11 bit atomically (RW)
    #[inline(always)]
    pub fn clr_ch11(&self) {
        self.value.clr_bits(Self::CH11_MASK);
    }

    /// **CH10** `[10]` (RW)
    #[inline(always)]
    pub fn ch10(&self) -> bool {
        (self.value.read_bits() & Self::CH10_MASK) != 0
    }

    /// Set CH10 bit atomically (RW)
    #[inline(always)]
    pub fn set_ch10(&self) {
        self.value.set_bits(Self::CH10_MASK);
    }

    /// Clear CH10 bit atomically (RW)
    #[inline(always)]
    pub fn clr_ch10(&self) {
        self.value.clr_bits(Self::CH10_MASK);
    }

    /// **CH9** `[9]` (RW)
    #[inline(always)]
    pub fn ch9(&self) -> bool {
        (self.value.read_bits() & Self::CH9_MASK) != 0
    }

    /// Set CH9 bit atomically (RW)
    #[inline(always)]
    pub fn set_ch9(&self) {
        self.value.set_bits(Self::CH9_MASK);
    }

    /// Clear CH9 bit atomically (RW)
    #[inline(always)]
    pub fn clr_ch9(&self) {
        self.value.clr_bits(Self::CH9_MASK);
    }

    /// **CH8** `[8]` (RW)
    #[inline(always)]
    pub fn ch8(&self) -> bool {
        (self.value.read_bits() & Self::CH8_MASK) != 0
    }

    /// Set CH8 bit atomically (RW)
    #[inline(always)]
    pub fn set_ch8(&self) {
        self.value.set_bits(Self::CH8_MASK);
    }

    /// Clear CH8 bit atomically (RW)
    #[inline(always)]
    pub fn clr_ch8(&self) {
        self.value.clr_bits(Self::CH8_MASK);
    }

    /// **CH7** `[7]` (RW)
    #[inline(always)]
    pub fn ch7(&self) -> bool {
        (self.value.read_bits() & Self::CH7_MASK) != 0
    }

    /// Set CH7 bit atomically (RW)
    #[inline(always)]
    pub fn set_ch7(&self) {
        self.value.set_bits(Self::CH7_MASK);
    }

    /// Clear CH7 bit atomically (RW)
    #[inline(always)]
    pub fn clr_ch7(&self) {
        self.value.clr_bits(Self::CH7_MASK);
    }

    /// **CH6** `[6]` (RW)
    #[inline(always)]
    pub fn ch6(&self) -> bool {
        (self.value.read_bits() & Self::CH6_MASK) != 0
    }

    /// Set CH6 bit atomically (RW)
    #[inline(always)]
    pub fn set_ch6(&self) {
        self.value.set_bits(Self::CH6_MASK);
    }

    /// Clear CH6 bit atomically (RW)
    #[inline(always)]
    pub fn clr_ch6(&self) {
        self.value.clr_bits(Self::CH6_MASK);
    }

    /// **CH5** `[5]` (RW)
    #[inline(always)]
    pub fn ch5(&self) -> bool {
        (self.value.read_bits() & Self::CH5_MASK) != 0
    }

    /// Set CH5 bit atomically (RW)
    #[inline(always)]
    pub fn set_ch5(&self) {
        self.value.set_bits(Self::CH5_MASK);
    }

    /// Clear CH5 bit atomically (RW)
    #[inline(always)]
    pub fn clr_ch5(&self) {
        self.value.clr_bits(Self::CH5_MASK);
    }

    /// **CH4** `[4]` (RW)
    #[inline(always)]
    pub fn ch4(&self) -> bool {
        (self.value.read_bits() & Self::CH4_MASK) != 0
    }

    /// Set CH4 bit atomically (RW)
    #[inline(always)]
    pub fn set_ch4(&self) {
        self.value.set_bits(Self::CH4_MASK);
    }

    /// Clear CH4 bit atomically (RW)
    #[inline(always)]
    pub fn clr_ch4(&self) {
        self.value.clr_bits(Self::CH4_MASK);
    }

    /// **CH3** `[3]` (RW)
    #[inline(always)]
    pub fn ch3(&self) -> bool {
        (self.value.read_bits() & Self::CH3_MASK) != 0
    }

    /// Set CH3 bit atomically (RW)
    #[inline(always)]
    pub fn set_ch3(&self) {
        self.value.set_bits(Self::CH3_MASK);
    }

    /// Clear CH3 bit atomically (RW)
    #[inline(always)]
    pub fn clr_ch3(&self) {
        self.value.clr_bits(Self::CH3_MASK);
    }

    /// **CH2** `[2]` (RW)
    #[inline(always)]
    pub fn ch2(&self) -> bool {
        (self.value.read_bits() & Self::CH2_MASK) != 0
    }

    /// Set CH2 bit atomically (RW)
    #[inline(always)]
    pub fn set_ch2(&self) {
        self.value.set_bits(Self::CH2_MASK);
    }

    /// Clear CH2 bit atomically (RW)
    #[inline(always)]
    pub fn clr_ch2(&self) {
        self.value.clr_bits(Self::CH2_MASK);
    }

    /// **CH1** `[1]` (RW)
    #[inline(always)]
    pub fn ch1(&self) -> bool {
        (self.value.read_bits() & Self::CH1_MASK) != 0
    }

    /// Set CH1 bit atomically (RW)
    #[inline(always)]
    pub fn set_ch1(&self) {
        self.value.set_bits(Self::CH1_MASK);
    }

    /// Clear CH1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_ch1(&self) {
        self.value.clr_bits(Self::CH1_MASK);
    }

    /// **CH0** `[0]` (RW)
    #[inline(always)]
    pub fn ch0(&self) -> bool {
        (self.value.read_bits() & Self::CH0_MASK) != 0
    }

    /// Set CH0 bit atomically (RW)
    #[inline(always)]
    pub fn set_ch0(&self) {
        self.value.set_bits(Self::CH0_MASK);
    }

    /// Clear CH0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_ch0(&self) {
        self.value.clr_bits(Self::CH0_MASK);
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

    /// Read **CH11** `[11:11]`
    #[inline(always)]
    pub const fn ch11(&self) -> bool {
        (self.bits & Intr::CH11_MASK) != 0
    }

    /// Read **CH10** `[10:10]`
    #[inline(always)]
    pub const fn ch10(&self) -> bool {
        (self.bits & Intr::CH10_MASK) != 0
    }

    /// Read **CH9** `[9:9]`
    #[inline(always)]
    pub const fn ch9(&self) -> bool {
        (self.bits & Intr::CH9_MASK) != 0
    }

    /// Read **CH8** `[8:8]`
    #[inline(always)]
    pub const fn ch8(&self) -> bool {
        (self.bits & Intr::CH8_MASK) != 0
    }

    /// Read **CH7** `[7:7]`
    #[inline(always)]
    pub const fn ch7(&self) -> bool {
        (self.bits & Intr::CH7_MASK) != 0
    }

    /// Read **CH6** `[6:6]`
    #[inline(always)]
    pub const fn ch6(&self) -> bool {
        (self.bits & Intr::CH6_MASK) != 0
    }

    /// Read **CH5** `[5:5]`
    #[inline(always)]
    pub const fn ch5(&self) -> bool {
        (self.bits & Intr::CH5_MASK) != 0
    }

    /// Read **CH4** `[4:4]`
    #[inline(always)]
    pub const fn ch4(&self) -> bool {
        (self.bits & Intr::CH4_MASK) != 0
    }

    /// Read **CH3** `[3:3]`
    #[inline(always)]
    pub const fn ch3(&self) -> bool {
        (self.bits & Intr::CH3_MASK) != 0
    }

    /// Read **CH2** `[2:2]`
    #[inline(always)]
    pub const fn ch2(&self) -> bool {
        (self.bits & Intr::CH2_MASK) != 0
    }

    /// Read **CH1** `[1:1]`
    #[inline(always)]
    pub const fn ch1(&self) -> bool {
        (self.bits & Intr::CH1_MASK) != 0
    }

    /// Read **CH0** `[0:0]`
    #[inline(always)]
    pub const fn ch0(&self) -> bool {
        (self.bits & Intr::CH0_MASK) != 0
    }

    /// Set **CH11** `[11:11]`
    #[inline(always)]
    pub const fn with_ch11(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intr::CH11_MASK
            } else {
                self.bits & !Intr::CH11_MASK
            }
        }
    }

    /// Set **CH10** `[10:10]`
    #[inline(always)]
    pub const fn with_ch10(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intr::CH10_MASK
            } else {
                self.bits & !Intr::CH10_MASK
            }
        }
    }

    /// Set **CH9** `[9:9]`
    #[inline(always)]
    pub const fn with_ch9(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intr::CH9_MASK
            } else {
                self.bits & !Intr::CH9_MASK
            }
        }
    }

    /// Set **CH8** `[8:8]`
    #[inline(always)]
    pub const fn with_ch8(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intr::CH8_MASK
            } else {
                self.bits & !Intr::CH8_MASK
            }
        }
    }

    /// Set **CH7** `[7:7]`
    #[inline(always)]
    pub const fn with_ch7(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intr::CH7_MASK
            } else {
                self.bits & !Intr::CH7_MASK
            }
        }
    }

    /// Set **CH6** `[6:6]`
    #[inline(always)]
    pub const fn with_ch6(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intr::CH6_MASK
            } else {
                self.bits & !Intr::CH6_MASK
            }
        }
    }

    /// Set **CH5** `[5:5]`
    #[inline(always)]
    pub const fn with_ch5(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intr::CH5_MASK
            } else {
                self.bits & !Intr::CH5_MASK
            }
        }
    }

    /// Set **CH4** `[4:4]`
    #[inline(always)]
    pub const fn with_ch4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intr::CH4_MASK
            } else {
                self.bits & !Intr::CH4_MASK
            }
        }
    }

    /// Set **CH3** `[3:3]`
    #[inline(always)]
    pub const fn with_ch3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intr::CH3_MASK
            } else {
                self.bits & !Intr::CH3_MASK
            }
        }
    }

    /// Set **CH2** `[2:2]`
    #[inline(always)]
    pub const fn with_ch2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intr::CH2_MASK
            } else {
                self.bits & !Intr::CH2_MASK
            }
        }
    }

    /// Set **CH1** `[1:1]`
    #[inline(always)]
    pub const fn with_ch1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intr::CH1_MASK
            } else {
                self.bits & !Intr::CH1_MASK
            }
        }
    }

    /// Set **CH0** `[0:0]`
    #[inline(always)]
    pub const fn with_ch0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intr::CH0_MASK
            } else {
                self.bits & !Intr::CH0_MASK
            }
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

/// IRQ0_INTE Register
///
/// Interrupt Enable for irq0
///
/// Address offset: 0x00f8
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CH11** `[11]` (RW)
///
/// - **CH10** `[10]` (RW)
///
/// - **CH9** `[9]` (RW)
///
/// - **CH8** `[8]` (RW)
///
/// - **CH7** `[7]` (RW)
///
/// - **CH6** `[6]` (RW)
///
/// - **CH5** `[5]` (RW)
///
/// - **CH4** `[4]` (RW)
///
/// - **CH3** `[3]` (RW)
///
/// - **CH2** `[2]` (RW)
///
/// - **CH1** `[1]` (RW)
///
/// - **CH0** `[0]` (RW)
#[repr(transparent)]
pub struct Irq0Inte {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Irq0Inte {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Irq0Inte {
    // ======== Field Constants ========

    /// **CH11**
    /// Bits `[11:11]`, mask `0x00000800`
    pub const CH11_OFFSET: u32 = 11;
    pub const CH11_MASK: u32 = 0x00000800;
    pub const CH11: u32 = 1 << 11;

    /// **CH10**
    /// Bits `[10:10]`, mask `0x00000400`
    pub const CH10_OFFSET: u32 = 10;
    pub const CH10_MASK: u32 = 0x00000400;
    pub const CH10: u32 = 1 << 10;

    /// **CH9**
    /// Bits `[9:9]`, mask `0x00000200`
    pub const CH9_OFFSET: u32 = 9;
    pub const CH9_MASK: u32 = 0x00000200;
    pub const CH9: u32 = 1 << 9;

    /// **CH8**
    /// Bits `[8:8]`, mask `0x00000100`
    pub const CH8_OFFSET: u32 = 8;
    pub const CH8_MASK: u32 = 0x00000100;
    pub const CH8: u32 = 1 << 8;

    /// **CH7**
    /// Bits `[7:7]`, mask `0x00000080`
    pub const CH7_OFFSET: u32 = 7;
    pub const CH7_MASK: u32 = 0x00000080;
    pub const CH7: u32 = 1 << 7;

    /// **CH6**
    /// Bits `[6:6]`, mask `0x00000040`
    pub const CH6_OFFSET: u32 = 6;
    pub const CH6_MASK: u32 = 0x00000040;
    pub const CH6: u32 = 1 << 6;

    /// **CH5**
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CH5_OFFSET: u32 = 5;
    pub const CH5_MASK: u32 = 0x00000020;
    pub const CH5: u32 = 1 << 5;

    /// **CH4**
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CH4_OFFSET: u32 = 4;
    pub const CH4_MASK: u32 = 0x00000010;
    pub const CH4: u32 = 1 << 4;

    /// **CH3**
    /// Bits `[3:3]`, mask `0x00000008`
    pub const CH3_OFFSET: u32 = 3;
    pub const CH3_MASK: u32 = 0x00000008;
    pub const CH3: u32 = 1 << 3;

    /// **CH2**
    /// Bits `[2:2]`, mask `0x00000004`
    pub const CH2_OFFSET: u32 = 2;
    pub const CH2_MASK: u32 = 0x00000004;
    pub const CH2: u32 = 1 << 2;

    /// **CH1**
    /// Bits `[1:1]`, mask `0x00000002`
    pub const CH1_OFFSET: u32 = 1;
    pub const CH1_MASK: u32 = 0x00000002;
    pub const CH1: u32 = 1 << 1;

    /// **CH0**
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CH0_OFFSET: u32 = 0;
    pub const CH0_MASK: u32 = 0x00000001;
    pub const CH0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CH11** `[11]` (RW)
    #[inline(always)]
    pub fn ch11(&self) -> bool {
        (self.value.read_bits() & Self::CH11_MASK) != 0
    }

    /// Set CH11 bit atomically (RW)
    #[inline(always)]
    pub fn set_ch11(&self) {
        self.value.set_bits(Self::CH11_MASK);
    }

    /// Clear CH11 bit atomically (RW)
    #[inline(always)]
    pub fn clr_ch11(&self) {
        self.value.clr_bits(Self::CH11_MASK);
    }

    /// **CH10** `[10]` (RW)
    #[inline(always)]
    pub fn ch10(&self) -> bool {
        (self.value.read_bits() & Self::CH10_MASK) != 0
    }

    /// Set CH10 bit atomically (RW)
    #[inline(always)]
    pub fn set_ch10(&self) {
        self.value.set_bits(Self::CH10_MASK);
    }

    /// Clear CH10 bit atomically (RW)
    #[inline(always)]
    pub fn clr_ch10(&self) {
        self.value.clr_bits(Self::CH10_MASK);
    }

    /// **CH9** `[9]` (RW)
    #[inline(always)]
    pub fn ch9(&self) -> bool {
        (self.value.read_bits() & Self::CH9_MASK) != 0
    }

    /// Set CH9 bit atomically (RW)
    #[inline(always)]
    pub fn set_ch9(&self) {
        self.value.set_bits(Self::CH9_MASK);
    }

    /// Clear CH9 bit atomically (RW)
    #[inline(always)]
    pub fn clr_ch9(&self) {
        self.value.clr_bits(Self::CH9_MASK);
    }

    /// **CH8** `[8]` (RW)
    #[inline(always)]
    pub fn ch8(&self) -> bool {
        (self.value.read_bits() & Self::CH8_MASK) != 0
    }

    /// Set CH8 bit atomically (RW)
    #[inline(always)]
    pub fn set_ch8(&self) {
        self.value.set_bits(Self::CH8_MASK);
    }

    /// Clear CH8 bit atomically (RW)
    #[inline(always)]
    pub fn clr_ch8(&self) {
        self.value.clr_bits(Self::CH8_MASK);
    }

    /// **CH7** `[7]` (RW)
    #[inline(always)]
    pub fn ch7(&self) -> bool {
        (self.value.read_bits() & Self::CH7_MASK) != 0
    }

    /// Set CH7 bit atomically (RW)
    #[inline(always)]
    pub fn set_ch7(&self) {
        self.value.set_bits(Self::CH7_MASK);
    }

    /// Clear CH7 bit atomically (RW)
    #[inline(always)]
    pub fn clr_ch7(&self) {
        self.value.clr_bits(Self::CH7_MASK);
    }

    /// **CH6** `[6]` (RW)
    #[inline(always)]
    pub fn ch6(&self) -> bool {
        (self.value.read_bits() & Self::CH6_MASK) != 0
    }

    /// Set CH6 bit atomically (RW)
    #[inline(always)]
    pub fn set_ch6(&self) {
        self.value.set_bits(Self::CH6_MASK);
    }

    /// Clear CH6 bit atomically (RW)
    #[inline(always)]
    pub fn clr_ch6(&self) {
        self.value.clr_bits(Self::CH6_MASK);
    }

    /// **CH5** `[5]` (RW)
    #[inline(always)]
    pub fn ch5(&self) -> bool {
        (self.value.read_bits() & Self::CH5_MASK) != 0
    }

    /// Set CH5 bit atomically (RW)
    #[inline(always)]
    pub fn set_ch5(&self) {
        self.value.set_bits(Self::CH5_MASK);
    }

    /// Clear CH5 bit atomically (RW)
    #[inline(always)]
    pub fn clr_ch5(&self) {
        self.value.clr_bits(Self::CH5_MASK);
    }

    /// **CH4** `[4]` (RW)
    #[inline(always)]
    pub fn ch4(&self) -> bool {
        (self.value.read_bits() & Self::CH4_MASK) != 0
    }

    /// Set CH4 bit atomically (RW)
    #[inline(always)]
    pub fn set_ch4(&self) {
        self.value.set_bits(Self::CH4_MASK);
    }

    /// Clear CH4 bit atomically (RW)
    #[inline(always)]
    pub fn clr_ch4(&self) {
        self.value.clr_bits(Self::CH4_MASK);
    }

    /// **CH3** `[3]` (RW)
    #[inline(always)]
    pub fn ch3(&self) -> bool {
        (self.value.read_bits() & Self::CH3_MASK) != 0
    }

    /// Set CH3 bit atomically (RW)
    #[inline(always)]
    pub fn set_ch3(&self) {
        self.value.set_bits(Self::CH3_MASK);
    }

    /// Clear CH3 bit atomically (RW)
    #[inline(always)]
    pub fn clr_ch3(&self) {
        self.value.clr_bits(Self::CH3_MASK);
    }

    /// **CH2** `[2]` (RW)
    #[inline(always)]
    pub fn ch2(&self) -> bool {
        (self.value.read_bits() & Self::CH2_MASK) != 0
    }

    /// Set CH2 bit atomically (RW)
    #[inline(always)]
    pub fn set_ch2(&self) {
        self.value.set_bits(Self::CH2_MASK);
    }

    /// Clear CH2 bit atomically (RW)
    #[inline(always)]
    pub fn clr_ch2(&self) {
        self.value.clr_bits(Self::CH2_MASK);
    }

    /// **CH1** `[1]` (RW)
    #[inline(always)]
    pub fn ch1(&self) -> bool {
        (self.value.read_bits() & Self::CH1_MASK) != 0
    }

    /// Set CH1 bit atomically (RW)
    #[inline(always)]
    pub fn set_ch1(&self) {
        self.value.set_bits(Self::CH1_MASK);
    }

    /// Clear CH1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_ch1(&self) {
        self.value.clr_bits(Self::CH1_MASK);
    }

    /// **CH0** `[0]` (RW)
    #[inline(always)]
    pub fn ch0(&self) -> bool {
        (self.value.read_bits() & Self::CH0_MASK) != 0
    }

    /// Set CH0 bit atomically (RW)
    #[inline(always)]
    pub fn set_ch0(&self) {
        self.value.set_bits(Self::CH0_MASK);
    }

    /// Clear CH0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_ch0(&self) {
        self.value.clr_bits(Self::CH0_MASK);
    }

}

/// IRQ0_INTE value type for batch read/modify/write operations
///
/// Use `irq0inte.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `irq0inte.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Irq0InteVal {
    bits: u32,
}

impl Irq0InteVal {
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

    /// Read **CH11** `[11:11]`
    #[inline(always)]
    pub const fn ch11(&self) -> bool {
        (self.bits & Irq0Inte::CH11_MASK) != 0
    }

    /// Read **CH10** `[10:10]`
    #[inline(always)]
    pub const fn ch10(&self) -> bool {
        (self.bits & Irq0Inte::CH10_MASK) != 0
    }

    /// Read **CH9** `[9:9]`
    #[inline(always)]
    pub const fn ch9(&self) -> bool {
        (self.bits & Irq0Inte::CH9_MASK) != 0
    }

    /// Read **CH8** `[8:8]`
    #[inline(always)]
    pub const fn ch8(&self) -> bool {
        (self.bits & Irq0Inte::CH8_MASK) != 0
    }

    /// Read **CH7** `[7:7]`
    #[inline(always)]
    pub const fn ch7(&self) -> bool {
        (self.bits & Irq0Inte::CH7_MASK) != 0
    }

    /// Read **CH6** `[6:6]`
    #[inline(always)]
    pub const fn ch6(&self) -> bool {
        (self.bits & Irq0Inte::CH6_MASK) != 0
    }

    /// Read **CH5** `[5:5]`
    #[inline(always)]
    pub const fn ch5(&self) -> bool {
        (self.bits & Irq0Inte::CH5_MASK) != 0
    }

    /// Read **CH4** `[4:4]`
    #[inline(always)]
    pub const fn ch4(&self) -> bool {
        (self.bits & Irq0Inte::CH4_MASK) != 0
    }

    /// Read **CH3** `[3:3]`
    #[inline(always)]
    pub const fn ch3(&self) -> bool {
        (self.bits & Irq0Inte::CH3_MASK) != 0
    }

    /// Read **CH2** `[2:2]`
    #[inline(always)]
    pub const fn ch2(&self) -> bool {
        (self.bits & Irq0Inte::CH2_MASK) != 0
    }

    /// Read **CH1** `[1:1]`
    #[inline(always)]
    pub const fn ch1(&self) -> bool {
        (self.bits & Irq0Inte::CH1_MASK) != 0
    }

    /// Read **CH0** `[0:0]`
    #[inline(always)]
    pub const fn ch0(&self) -> bool {
        (self.bits & Irq0Inte::CH0_MASK) != 0
    }

    /// Set **CH11** `[11:11]`
    #[inline(always)]
    pub const fn with_ch11(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq0Inte::CH11_MASK
            } else {
                self.bits & !Irq0Inte::CH11_MASK
            }
        }
    }

    /// Set **CH10** `[10:10]`
    #[inline(always)]
    pub const fn with_ch10(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq0Inte::CH10_MASK
            } else {
                self.bits & !Irq0Inte::CH10_MASK
            }
        }
    }

    /// Set **CH9** `[9:9]`
    #[inline(always)]
    pub const fn with_ch9(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq0Inte::CH9_MASK
            } else {
                self.bits & !Irq0Inte::CH9_MASK
            }
        }
    }

    /// Set **CH8** `[8:8]`
    #[inline(always)]
    pub const fn with_ch8(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq0Inte::CH8_MASK
            } else {
                self.bits & !Irq0Inte::CH8_MASK
            }
        }
    }

    /// Set **CH7** `[7:7]`
    #[inline(always)]
    pub const fn with_ch7(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq0Inte::CH7_MASK
            } else {
                self.bits & !Irq0Inte::CH7_MASK
            }
        }
    }

    /// Set **CH6** `[6:6]`
    #[inline(always)]
    pub const fn with_ch6(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq0Inte::CH6_MASK
            } else {
                self.bits & !Irq0Inte::CH6_MASK
            }
        }
    }

    /// Set **CH5** `[5:5]`
    #[inline(always)]
    pub const fn with_ch5(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq0Inte::CH5_MASK
            } else {
                self.bits & !Irq0Inte::CH5_MASK
            }
        }
    }

    /// Set **CH4** `[4:4]`
    #[inline(always)]
    pub const fn with_ch4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq0Inte::CH4_MASK
            } else {
                self.bits & !Irq0Inte::CH4_MASK
            }
        }
    }

    /// Set **CH3** `[3:3]`
    #[inline(always)]
    pub const fn with_ch3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq0Inte::CH3_MASK
            } else {
                self.bits & !Irq0Inte::CH3_MASK
            }
        }
    }

    /// Set **CH2** `[2:2]`
    #[inline(always)]
    pub const fn with_ch2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq0Inte::CH2_MASK
            } else {
                self.bits & !Irq0Inte::CH2_MASK
            }
        }
    }

    /// Set **CH1** `[1:1]`
    #[inline(always)]
    pub const fn with_ch1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq0Inte::CH1_MASK
            } else {
                self.bits & !Irq0Inte::CH1_MASK
            }
        }
    }

    /// Set **CH0** `[0:0]`
    #[inline(always)]
    pub const fn with_ch0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq0Inte::CH0_MASK
            } else {
                self.bits & !Irq0Inte::CH0_MASK
            }
        }
    }

}

impl Default for Irq0InteVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Irq0Inte {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Irq0InteVal {
        Irq0InteVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Irq0InteVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Irq0InteVal) -> Irq0InteVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// IRQ0_INTF Register
///
/// Interrupt Force for irq0
///
/// Address offset: 0x00fc
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CH11** `[11]` (RW)
///
/// - **CH10** `[10]` (RW)
///
/// - **CH9** `[9]` (RW)
///
/// - **CH8** `[8]` (RW)
///
/// - **CH7** `[7]` (RW)
///
/// - **CH6** `[6]` (RW)
///
/// - **CH5** `[5]` (RW)
///
/// - **CH4** `[4]` (RW)
///
/// - **CH3** `[3]` (RW)
///
/// - **CH2** `[2]` (RW)
///
/// - **CH1** `[1]` (RW)
///
/// - **CH0** `[0]` (RW)
#[repr(transparent)]
pub struct Irq0Intf {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Irq0Intf {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Irq0Intf {
    // ======== Field Constants ========

    /// **CH11**
    /// Bits `[11:11]`, mask `0x00000800`
    pub const CH11_OFFSET: u32 = 11;
    pub const CH11_MASK: u32 = 0x00000800;
    pub const CH11: u32 = 1 << 11;

    /// **CH10**
    /// Bits `[10:10]`, mask `0x00000400`
    pub const CH10_OFFSET: u32 = 10;
    pub const CH10_MASK: u32 = 0x00000400;
    pub const CH10: u32 = 1 << 10;

    /// **CH9**
    /// Bits `[9:9]`, mask `0x00000200`
    pub const CH9_OFFSET: u32 = 9;
    pub const CH9_MASK: u32 = 0x00000200;
    pub const CH9: u32 = 1 << 9;

    /// **CH8**
    /// Bits `[8:8]`, mask `0x00000100`
    pub const CH8_OFFSET: u32 = 8;
    pub const CH8_MASK: u32 = 0x00000100;
    pub const CH8: u32 = 1 << 8;

    /// **CH7**
    /// Bits `[7:7]`, mask `0x00000080`
    pub const CH7_OFFSET: u32 = 7;
    pub const CH7_MASK: u32 = 0x00000080;
    pub const CH7: u32 = 1 << 7;

    /// **CH6**
    /// Bits `[6:6]`, mask `0x00000040`
    pub const CH6_OFFSET: u32 = 6;
    pub const CH6_MASK: u32 = 0x00000040;
    pub const CH6: u32 = 1 << 6;

    /// **CH5**
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CH5_OFFSET: u32 = 5;
    pub const CH5_MASK: u32 = 0x00000020;
    pub const CH5: u32 = 1 << 5;

    /// **CH4**
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CH4_OFFSET: u32 = 4;
    pub const CH4_MASK: u32 = 0x00000010;
    pub const CH4: u32 = 1 << 4;

    /// **CH3**
    /// Bits `[3:3]`, mask `0x00000008`
    pub const CH3_OFFSET: u32 = 3;
    pub const CH3_MASK: u32 = 0x00000008;
    pub const CH3: u32 = 1 << 3;

    /// **CH2**
    /// Bits `[2:2]`, mask `0x00000004`
    pub const CH2_OFFSET: u32 = 2;
    pub const CH2_MASK: u32 = 0x00000004;
    pub const CH2: u32 = 1 << 2;

    /// **CH1**
    /// Bits `[1:1]`, mask `0x00000002`
    pub const CH1_OFFSET: u32 = 1;
    pub const CH1_MASK: u32 = 0x00000002;
    pub const CH1: u32 = 1 << 1;

    /// **CH0**
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CH0_OFFSET: u32 = 0;
    pub const CH0_MASK: u32 = 0x00000001;
    pub const CH0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CH11** `[11]` (RW)
    #[inline(always)]
    pub fn ch11(&self) -> bool {
        (self.value.read_bits() & Self::CH11_MASK) != 0
    }

    /// Set CH11 bit atomically (RW)
    #[inline(always)]
    pub fn set_ch11(&self) {
        self.value.set_bits(Self::CH11_MASK);
    }

    /// Clear CH11 bit atomically (RW)
    #[inline(always)]
    pub fn clr_ch11(&self) {
        self.value.clr_bits(Self::CH11_MASK);
    }

    /// **CH10** `[10]` (RW)
    #[inline(always)]
    pub fn ch10(&self) -> bool {
        (self.value.read_bits() & Self::CH10_MASK) != 0
    }

    /// Set CH10 bit atomically (RW)
    #[inline(always)]
    pub fn set_ch10(&self) {
        self.value.set_bits(Self::CH10_MASK);
    }

    /// Clear CH10 bit atomically (RW)
    #[inline(always)]
    pub fn clr_ch10(&self) {
        self.value.clr_bits(Self::CH10_MASK);
    }

    /// **CH9** `[9]` (RW)
    #[inline(always)]
    pub fn ch9(&self) -> bool {
        (self.value.read_bits() & Self::CH9_MASK) != 0
    }

    /// Set CH9 bit atomically (RW)
    #[inline(always)]
    pub fn set_ch9(&self) {
        self.value.set_bits(Self::CH9_MASK);
    }

    /// Clear CH9 bit atomically (RW)
    #[inline(always)]
    pub fn clr_ch9(&self) {
        self.value.clr_bits(Self::CH9_MASK);
    }

    /// **CH8** `[8]` (RW)
    #[inline(always)]
    pub fn ch8(&self) -> bool {
        (self.value.read_bits() & Self::CH8_MASK) != 0
    }

    /// Set CH8 bit atomically (RW)
    #[inline(always)]
    pub fn set_ch8(&self) {
        self.value.set_bits(Self::CH8_MASK);
    }

    /// Clear CH8 bit atomically (RW)
    #[inline(always)]
    pub fn clr_ch8(&self) {
        self.value.clr_bits(Self::CH8_MASK);
    }

    /// **CH7** `[7]` (RW)
    #[inline(always)]
    pub fn ch7(&self) -> bool {
        (self.value.read_bits() & Self::CH7_MASK) != 0
    }

    /// Set CH7 bit atomically (RW)
    #[inline(always)]
    pub fn set_ch7(&self) {
        self.value.set_bits(Self::CH7_MASK);
    }

    /// Clear CH7 bit atomically (RW)
    #[inline(always)]
    pub fn clr_ch7(&self) {
        self.value.clr_bits(Self::CH7_MASK);
    }

    /// **CH6** `[6]` (RW)
    #[inline(always)]
    pub fn ch6(&self) -> bool {
        (self.value.read_bits() & Self::CH6_MASK) != 0
    }

    /// Set CH6 bit atomically (RW)
    #[inline(always)]
    pub fn set_ch6(&self) {
        self.value.set_bits(Self::CH6_MASK);
    }

    /// Clear CH6 bit atomically (RW)
    #[inline(always)]
    pub fn clr_ch6(&self) {
        self.value.clr_bits(Self::CH6_MASK);
    }

    /// **CH5** `[5]` (RW)
    #[inline(always)]
    pub fn ch5(&self) -> bool {
        (self.value.read_bits() & Self::CH5_MASK) != 0
    }

    /// Set CH5 bit atomically (RW)
    #[inline(always)]
    pub fn set_ch5(&self) {
        self.value.set_bits(Self::CH5_MASK);
    }

    /// Clear CH5 bit atomically (RW)
    #[inline(always)]
    pub fn clr_ch5(&self) {
        self.value.clr_bits(Self::CH5_MASK);
    }

    /// **CH4** `[4]` (RW)
    #[inline(always)]
    pub fn ch4(&self) -> bool {
        (self.value.read_bits() & Self::CH4_MASK) != 0
    }

    /// Set CH4 bit atomically (RW)
    #[inline(always)]
    pub fn set_ch4(&self) {
        self.value.set_bits(Self::CH4_MASK);
    }

    /// Clear CH4 bit atomically (RW)
    #[inline(always)]
    pub fn clr_ch4(&self) {
        self.value.clr_bits(Self::CH4_MASK);
    }

    /// **CH3** `[3]` (RW)
    #[inline(always)]
    pub fn ch3(&self) -> bool {
        (self.value.read_bits() & Self::CH3_MASK) != 0
    }

    /// Set CH3 bit atomically (RW)
    #[inline(always)]
    pub fn set_ch3(&self) {
        self.value.set_bits(Self::CH3_MASK);
    }

    /// Clear CH3 bit atomically (RW)
    #[inline(always)]
    pub fn clr_ch3(&self) {
        self.value.clr_bits(Self::CH3_MASK);
    }

    /// **CH2** `[2]` (RW)
    #[inline(always)]
    pub fn ch2(&self) -> bool {
        (self.value.read_bits() & Self::CH2_MASK) != 0
    }

    /// Set CH2 bit atomically (RW)
    #[inline(always)]
    pub fn set_ch2(&self) {
        self.value.set_bits(Self::CH2_MASK);
    }

    /// Clear CH2 bit atomically (RW)
    #[inline(always)]
    pub fn clr_ch2(&self) {
        self.value.clr_bits(Self::CH2_MASK);
    }

    /// **CH1** `[1]` (RW)
    #[inline(always)]
    pub fn ch1(&self) -> bool {
        (self.value.read_bits() & Self::CH1_MASK) != 0
    }

    /// Set CH1 bit atomically (RW)
    #[inline(always)]
    pub fn set_ch1(&self) {
        self.value.set_bits(Self::CH1_MASK);
    }

    /// Clear CH1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_ch1(&self) {
        self.value.clr_bits(Self::CH1_MASK);
    }

    /// **CH0** `[0]` (RW)
    #[inline(always)]
    pub fn ch0(&self) -> bool {
        (self.value.read_bits() & Self::CH0_MASK) != 0
    }

    /// Set CH0 bit atomically (RW)
    #[inline(always)]
    pub fn set_ch0(&self) {
        self.value.set_bits(Self::CH0_MASK);
    }

    /// Clear CH0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_ch0(&self) {
        self.value.clr_bits(Self::CH0_MASK);
    }

}

/// IRQ0_INTF value type for batch read/modify/write operations
///
/// Use `irq0intf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `irq0intf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Irq0IntfVal {
    bits: u32,
}

impl Irq0IntfVal {
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

    /// Read **CH11** `[11:11]`
    #[inline(always)]
    pub const fn ch11(&self) -> bool {
        (self.bits & Irq0Intf::CH11_MASK) != 0
    }

    /// Read **CH10** `[10:10]`
    #[inline(always)]
    pub const fn ch10(&self) -> bool {
        (self.bits & Irq0Intf::CH10_MASK) != 0
    }

    /// Read **CH9** `[9:9]`
    #[inline(always)]
    pub const fn ch9(&self) -> bool {
        (self.bits & Irq0Intf::CH9_MASK) != 0
    }

    /// Read **CH8** `[8:8]`
    #[inline(always)]
    pub const fn ch8(&self) -> bool {
        (self.bits & Irq0Intf::CH8_MASK) != 0
    }

    /// Read **CH7** `[7:7]`
    #[inline(always)]
    pub const fn ch7(&self) -> bool {
        (self.bits & Irq0Intf::CH7_MASK) != 0
    }

    /// Read **CH6** `[6:6]`
    #[inline(always)]
    pub const fn ch6(&self) -> bool {
        (self.bits & Irq0Intf::CH6_MASK) != 0
    }

    /// Read **CH5** `[5:5]`
    #[inline(always)]
    pub const fn ch5(&self) -> bool {
        (self.bits & Irq0Intf::CH5_MASK) != 0
    }

    /// Read **CH4** `[4:4]`
    #[inline(always)]
    pub const fn ch4(&self) -> bool {
        (self.bits & Irq0Intf::CH4_MASK) != 0
    }

    /// Read **CH3** `[3:3]`
    #[inline(always)]
    pub const fn ch3(&self) -> bool {
        (self.bits & Irq0Intf::CH3_MASK) != 0
    }

    /// Read **CH2** `[2:2]`
    #[inline(always)]
    pub const fn ch2(&self) -> bool {
        (self.bits & Irq0Intf::CH2_MASK) != 0
    }

    /// Read **CH1** `[1:1]`
    #[inline(always)]
    pub const fn ch1(&self) -> bool {
        (self.bits & Irq0Intf::CH1_MASK) != 0
    }

    /// Read **CH0** `[0:0]`
    #[inline(always)]
    pub const fn ch0(&self) -> bool {
        (self.bits & Irq0Intf::CH0_MASK) != 0
    }

    /// Set **CH11** `[11:11]`
    #[inline(always)]
    pub const fn with_ch11(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq0Intf::CH11_MASK
            } else {
                self.bits & !Irq0Intf::CH11_MASK
            }
        }
    }

    /// Set **CH10** `[10:10]`
    #[inline(always)]
    pub const fn with_ch10(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq0Intf::CH10_MASK
            } else {
                self.bits & !Irq0Intf::CH10_MASK
            }
        }
    }

    /// Set **CH9** `[9:9]`
    #[inline(always)]
    pub const fn with_ch9(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq0Intf::CH9_MASK
            } else {
                self.bits & !Irq0Intf::CH9_MASK
            }
        }
    }

    /// Set **CH8** `[8:8]`
    #[inline(always)]
    pub const fn with_ch8(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq0Intf::CH8_MASK
            } else {
                self.bits & !Irq0Intf::CH8_MASK
            }
        }
    }

    /// Set **CH7** `[7:7]`
    #[inline(always)]
    pub const fn with_ch7(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq0Intf::CH7_MASK
            } else {
                self.bits & !Irq0Intf::CH7_MASK
            }
        }
    }

    /// Set **CH6** `[6:6]`
    #[inline(always)]
    pub const fn with_ch6(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq0Intf::CH6_MASK
            } else {
                self.bits & !Irq0Intf::CH6_MASK
            }
        }
    }

    /// Set **CH5** `[5:5]`
    #[inline(always)]
    pub const fn with_ch5(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq0Intf::CH5_MASK
            } else {
                self.bits & !Irq0Intf::CH5_MASK
            }
        }
    }

    /// Set **CH4** `[4:4]`
    #[inline(always)]
    pub const fn with_ch4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq0Intf::CH4_MASK
            } else {
                self.bits & !Irq0Intf::CH4_MASK
            }
        }
    }

    /// Set **CH3** `[3:3]`
    #[inline(always)]
    pub const fn with_ch3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq0Intf::CH3_MASK
            } else {
                self.bits & !Irq0Intf::CH3_MASK
            }
        }
    }

    /// Set **CH2** `[2:2]`
    #[inline(always)]
    pub const fn with_ch2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq0Intf::CH2_MASK
            } else {
                self.bits & !Irq0Intf::CH2_MASK
            }
        }
    }

    /// Set **CH1** `[1:1]`
    #[inline(always)]
    pub const fn with_ch1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq0Intf::CH1_MASK
            } else {
                self.bits & !Irq0Intf::CH1_MASK
            }
        }
    }

    /// Set **CH0** `[0:0]`
    #[inline(always)]
    pub const fn with_ch0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq0Intf::CH0_MASK
            } else {
                self.bits & !Irq0Intf::CH0_MASK
            }
        }
    }

}

impl Default for Irq0IntfVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Irq0Intf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Irq0IntfVal {
        Irq0IntfVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Irq0IntfVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Irq0IntfVal) -> Irq0IntfVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// IRQ0_INTS Register
///
/// Interrupt status after masking & forcing for irq0
///
/// Address offset: 0x0100
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CH11** `[11]` (RO)
///
/// - **CH10** `[10]` (RO)
///
/// - **CH9** `[9]` (RO)
///
/// - **CH8** `[8]` (RO)
///
/// - **CH7** `[7]` (RO)
///
/// - **CH6** `[6]` (RO)
///
/// - **CH5** `[5]` (RO)
///
/// - **CH4** `[4]` (RO)
///
/// - **CH3** `[3]` (RO)
///
/// - **CH2** `[2]` (RO)
///
/// - **CH1** `[1]` (RO)
///
/// - **CH0** `[0]` (RO)
#[repr(transparent)]
pub struct Irq0Ints {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Irq0Ints {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Irq0Ints {
    // ======== Field Constants ========

    /// **CH11**
    /// Bits `[11:11]`, mask `0x00000800`
    pub const CH11_OFFSET: u32 = 11;
    pub const CH11_MASK: u32 = 0x00000800;
    pub const CH11: u32 = 1 << 11;

    /// **CH10**
    /// Bits `[10:10]`, mask `0x00000400`
    pub const CH10_OFFSET: u32 = 10;
    pub const CH10_MASK: u32 = 0x00000400;
    pub const CH10: u32 = 1 << 10;

    /// **CH9**
    /// Bits `[9:9]`, mask `0x00000200`
    pub const CH9_OFFSET: u32 = 9;
    pub const CH9_MASK: u32 = 0x00000200;
    pub const CH9: u32 = 1 << 9;

    /// **CH8**
    /// Bits `[8:8]`, mask `0x00000100`
    pub const CH8_OFFSET: u32 = 8;
    pub const CH8_MASK: u32 = 0x00000100;
    pub const CH8: u32 = 1 << 8;

    /// **CH7**
    /// Bits `[7:7]`, mask `0x00000080`
    pub const CH7_OFFSET: u32 = 7;
    pub const CH7_MASK: u32 = 0x00000080;
    pub const CH7: u32 = 1 << 7;

    /// **CH6**
    /// Bits `[6:6]`, mask `0x00000040`
    pub const CH6_OFFSET: u32 = 6;
    pub const CH6_MASK: u32 = 0x00000040;
    pub const CH6: u32 = 1 << 6;

    /// **CH5**
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CH5_OFFSET: u32 = 5;
    pub const CH5_MASK: u32 = 0x00000020;
    pub const CH5: u32 = 1 << 5;

    /// **CH4**
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CH4_OFFSET: u32 = 4;
    pub const CH4_MASK: u32 = 0x00000010;
    pub const CH4: u32 = 1 << 4;

    /// **CH3**
    /// Bits `[3:3]`, mask `0x00000008`
    pub const CH3_OFFSET: u32 = 3;
    pub const CH3_MASK: u32 = 0x00000008;
    pub const CH3: u32 = 1 << 3;

    /// **CH2**
    /// Bits `[2:2]`, mask `0x00000004`
    pub const CH2_OFFSET: u32 = 2;
    pub const CH2_MASK: u32 = 0x00000004;
    pub const CH2: u32 = 1 << 2;

    /// **CH1**
    /// Bits `[1:1]`, mask `0x00000002`
    pub const CH1_OFFSET: u32 = 1;
    pub const CH1_MASK: u32 = 0x00000002;
    pub const CH1: u32 = 1 << 1;

    /// **CH0**
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CH0_OFFSET: u32 = 0;
    pub const CH0_MASK: u32 = 0x00000001;
    pub const CH0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CH11** `[11]` (RO)
    #[inline(always)]
    pub fn ch11(&self) -> bool {
        (self.value.read_bits() & Self::CH11_MASK) != 0
    }

    /// **CH10** `[10]` (RO)
    #[inline(always)]
    pub fn ch10(&self) -> bool {
        (self.value.read_bits() & Self::CH10_MASK) != 0
    }

    /// **CH9** `[9]` (RO)
    #[inline(always)]
    pub fn ch9(&self) -> bool {
        (self.value.read_bits() & Self::CH9_MASK) != 0
    }

    /// **CH8** `[8]` (RO)
    #[inline(always)]
    pub fn ch8(&self) -> bool {
        (self.value.read_bits() & Self::CH8_MASK) != 0
    }

    /// **CH7** `[7]` (RO)
    #[inline(always)]
    pub fn ch7(&self) -> bool {
        (self.value.read_bits() & Self::CH7_MASK) != 0
    }

    /// **CH6** `[6]` (RO)
    #[inline(always)]
    pub fn ch6(&self) -> bool {
        (self.value.read_bits() & Self::CH6_MASK) != 0
    }

    /// **CH5** `[5]` (RO)
    #[inline(always)]
    pub fn ch5(&self) -> bool {
        (self.value.read_bits() & Self::CH5_MASK) != 0
    }

    /// **CH4** `[4]` (RO)
    #[inline(always)]
    pub fn ch4(&self) -> bool {
        (self.value.read_bits() & Self::CH4_MASK) != 0
    }

    /// **CH3** `[3]` (RO)
    #[inline(always)]
    pub fn ch3(&self) -> bool {
        (self.value.read_bits() & Self::CH3_MASK) != 0
    }

    /// **CH2** `[2]` (RO)
    #[inline(always)]
    pub fn ch2(&self) -> bool {
        (self.value.read_bits() & Self::CH2_MASK) != 0
    }

    /// **CH1** `[1]` (RO)
    #[inline(always)]
    pub fn ch1(&self) -> bool {
        (self.value.read_bits() & Self::CH1_MASK) != 0
    }

    /// **CH0** `[0]` (RO)
    #[inline(always)]
    pub fn ch0(&self) -> bool {
        (self.value.read_bits() & Self::CH0_MASK) != 0
    }

}

/// IRQ0_INTS value type for batch read/modify/write operations
///
/// Use `irq0ints.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `irq0ints.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Irq0IntsVal {
    bits: u32,
}

impl Irq0IntsVal {
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

    /// Read **CH11** `[11:11]`
    #[inline(always)]
    pub const fn ch11(&self) -> bool {
        (self.bits & Irq0Ints::CH11_MASK) != 0
    }

    /// Read **CH10** `[10:10]`
    #[inline(always)]
    pub const fn ch10(&self) -> bool {
        (self.bits & Irq0Ints::CH10_MASK) != 0
    }

    /// Read **CH9** `[9:9]`
    #[inline(always)]
    pub const fn ch9(&self) -> bool {
        (self.bits & Irq0Ints::CH9_MASK) != 0
    }

    /// Read **CH8** `[8:8]`
    #[inline(always)]
    pub const fn ch8(&self) -> bool {
        (self.bits & Irq0Ints::CH8_MASK) != 0
    }

    /// Read **CH7** `[7:7]`
    #[inline(always)]
    pub const fn ch7(&self) -> bool {
        (self.bits & Irq0Ints::CH7_MASK) != 0
    }

    /// Read **CH6** `[6:6]`
    #[inline(always)]
    pub const fn ch6(&self) -> bool {
        (self.bits & Irq0Ints::CH6_MASK) != 0
    }

    /// Read **CH5** `[5:5]`
    #[inline(always)]
    pub const fn ch5(&self) -> bool {
        (self.bits & Irq0Ints::CH5_MASK) != 0
    }

    /// Read **CH4** `[4:4]`
    #[inline(always)]
    pub const fn ch4(&self) -> bool {
        (self.bits & Irq0Ints::CH4_MASK) != 0
    }

    /// Read **CH3** `[3:3]`
    #[inline(always)]
    pub const fn ch3(&self) -> bool {
        (self.bits & Irq0Ints::CH3_MASK) != 0
    }

    /// Read **CH2** `[2:2]`
    #[inline(always)]
    pub const fn ch2(&self) -> bool {
        (self.bits & Irq0Ints::CH2_MASK) != 0
    }

    /// Read **CH1** `[1:1]`
    #[inline(always)]
    pub const fn ch1(&self) -> bool {
        (self.bits & Irq0Ints::CH1_MASK) != 0
    }

    /// Read **CH0** `[0:0]`
    #[inline(always)]
    pub const fn ch0(&self) -> bool {
        (self.bits & Irq0Ints::CH0_MASK) != 0
    }

}

impl Default for Irq0IntsVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Irq0Ints {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Irq0IntsVal {
        Irq0IntsVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Irq0IntsVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Irq0IntsVal) -> Irq0IntsVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// IRQ1_INTE Register
///
/// Interrupt Enable for irq1
///
/// Address offset: 0x0104
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CH11** `[11]` (RW)
///
/// - **CH10** `[10]` (RW)
///
/// - **CH9** `[9]` (RW)
///
/// - **CH8** `[8]` (RW)
///
/// - **CH7** `[7]` (RW)
///
/// - **CH6** `[6]` (RW)
///
/// - **CH5** `[5]` (RW)
///
/// - **CH4** `[4]` (RW)
///
/// - **CH3** `[3]` (RW)
///
/// - **CH2** `[2]` (RW)
///
/// - **CH1** `[1]` (RW)
///
/// - **CH0** `[0]` (RW)
#[repr(transparent)]
pub struct Irq1Inte {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Irq1Inte {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Irq1Inte {
    // ======== Field Constants ========

    /// **CH11**
    /// Bits `[11:11]`, mask `0x00000800`
    pub const CH11_OFFSET: u32 = 11;
    pub const CH11_MASK: u32 = 0x00000800;
    pub const CH11: u32 = 1 << 11;

    /// **CH10**
    /// Bits `[10:10]`, mask `0x00000400`
    pub const CH10_OFFSET: u32 = 10;
    pub const CH10_MASK: u32 = 0x00000400;
    pub const CH10: u32 = 1 << 10;

    /// **CH9**
    /// Bits `[9:9]`, mask `0x00000200`
    pub const CH9_OFFSET: u32 = 9;
    pub const CH9_MASK: u32 = 0x00000200;
    pub const CH9: u32 = 1 << 9;

    /// **CH8**
    /// Bits `[8:8]`, mask `0x00000100`
    pub const CH8_OFFSET: u32 = 8;
    pub const CH8_MASK: u32 = 0x00000100;
    pub const CH8: u32 = 1 << 8;

    /// **CH7**
    /// Bits `[7:7]`, mask `0x00000080`
    pub const CH7_OFFSET: u32 = 7;
    pub const CH7_MASK: u32 = 0x00000080;
    pub const CH7: u32 = 1 << 7;

    /// **CH6**
    /// Bits `[6:6]`, mask `0x00000040`
    pub const CH6_OFFSET: u32 = 6;
    pub const CH6_MASK: u32 = 0x00000040;
    pub const CH6: u32 = 1 << 6;

    /// **CH5**
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CH5_OFFSET: u32 = 5;
    pub const CH5_MASK: u32 = 0x00000020;
    pub const CH5: u32 = 1 << 5;

    /// **CH4**
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CH4_OFFSET: u32 = 4;
    pub const CH4_MASK: u32 = 0x00000010;
    pub const CH4: u32 = 1 << 4;

    /// **CH3**
    /// Bits `[3:3]`, mask `0x00000008`
    pub const CH3_OFFSET: u32 = 3;
    pub const CH3_MASK: u32 = 0x00000008;
    pub const CH3: u32 = 1 << 3;

    /// **CH2**
    /// Bits `[2:2]`, mask `0x00000004`
    pub const CH2_OFFSET: u32 = 2;
    pub const CH2_MASK: u32 = 0x00000004;
    pub const CH2: u32 = 1 << 2;

    /// **CH1**
    /// Bits `[1:1]`, mask `0x00000002`
    pub const CH1_OFFSET: u32 = 1;
    pub const CH1_MASK: u32 = 0x00000002;
    pub const CH1: u32 = 1 << 1;

    /// **CH0**
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CH0_OFFSET: u32 = 0;
    pub const CH0_MASK: u32 = 0x00000001;
    pub const CH0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CH11** `[11]` (RW)
    #[inline(always)]
    pub fn ch11(&self) -> bool {
        (self.value.read_bits() & Self::CH11_MASK) != 0
    }

    /// Set CH11 bit atomically (RW)
    #[inline(always)]
    pub fn set_ch11(&self) {
        self.value.set_bits(Self::CH11_MASK);
    }

    /// Clear CH11 bit atomically (RW)
    #[inline(always)]
    pub fn clr_ch11(&self) {
        self.value.clr_bits(Self::CH11_MASK);
    }

    /// **CH10** `[10]` (RW)
    #[inline(always)]
    pub fn ch10(&self) -> bool {
        (self.value.read_bits() & Self::CH10_MASK) != 0
    }

    /// Set CH10 bit atomically (RW)
    #[inline(always)]
    pub fn set_ch10(&self) {
        self.value.set_bits(Self::CH10_MASK);
    }

    /// Clear CH10 bit atomically (RW)
    #[inline(always)]
    pub fn clr_ch10(&self) {
        self.value.clr_bits(Self::CH10_MASK);
    }

    /// **CH9** `[9]` (RW)
    #[inline(always)]
    pub fn ch9(&self) -> bool {
        (self.value.read_bits() & Self::CH9_MASK) != 0
    }

    /// Set CH9 bit atomically (RW)
    #[inline(always)]
    pub fn set_ch9(&self) {
        self.value.set_bits(Self::CH9_MASK);
    }

    /// Clear CH9 bit atomically (RW)
    #[inline(always)]
    pub fn clr_ch9(&self) {
        self.value.clr_bits(Self::CH9_MASK);
    }

    /// **CH8** `[8]` (RW)
    #[inline(always)]
    pub fn ch8(&self) -> bool {
        (self.value.read_bits() & Self::CH8_MASK) != 0
    }

    /// Set CH8 bit atomically (RW)
    #[inline(always)]
    pub fn set_ch8(&self) {
        self.value.set_bits(Self::CH8_MASK);
    }

    /// Clear CH8 bit atomically (RW)
    #[inline(always)]
    pub fn clr_ch8(&self) {
        self.value.clr_bits(Self::CH8_MASK);
    }

    /// **CH7** `[7]` (RW)
    #[inline(always)]
    pub fn ch7(&self) -> bool {
        (self.value.read_bits() & Self::CH7_MASK) != 0
    }

    /// Set CH7 bit atomically (RW)
    #[inline(always)]
    pub fn set_ch7(&self) {
        self.value.set_bits(Self::CH7_MASK);
    }

    /// Clear CH7 bit atomically (RW)
    #[inline(always)]
    pub fn clr_ch7(&self) {
        self.value.clr_bits(Self::CH7_MASK);
    }

    /// **CH6** `[6]` (RW)
    #[inline(always)]
    pub fn ch6(&self) -> bool {
        (self.value.read_bits() & Self::CH6_MASK) != 0
    }

    /// Set CH6 bit atomically (RW)
    #[inline(always)]
    pub fn set_ch6(&self) {
        self.value.set_bits(Self::CH6_MASK);
    }

    /// Clear CH6 bit atomically (RW)
    #[inline(always)]
    pub fn clr_ch6(&self) {
        self.value.clr_bits(Self::CH6_MASK);
    }

    /// **CH5** `[5]` (RW)
    #[inline(always)]
    pub fn ch5(&self) -> bool {
        (self.value.read_bits() & Self::CH5_MASK) != 0
    }

    /// Set CH5 bit atomically (RW)
    #[inline(always)]
    pub fn set_ch5(&self) {
        self.value.set_bits(Self::CH5_MASK);
    }

    /// Clear CH5 bit atomically (RW)
    #[inline(always)]
    pub fn clr_ch5(&self) {
        self.value.clr_bits(Self::CH5_MASK);
    }

    /// **CH4** `[4]` (RW)
    #[inline(always)]
    pub fn ch4(&self) -> bool {
        (self.value.read_bits() & Self::CH4_MASK) != 0
    }

    /// Set CH4 bit atomically (RW)
    #[inline(always)]
    pub fn set_ch4(&self) {
        self.value.set_bits(Self::CH4_MASK);
    }

    /// Clear CH4 bit atomically (RW)
    #[inline(always)]
    pub fn clr_ch4(&self) {
        self.value.clr_bits(Self::CH4_MASK);
    }

    /// **CH3** `[3]` (RW)
    #[inline(always)]
    pub fn ch3(&self) -> bool {
        (self.value.read_bits() & Self::CH3_MASK) != 0
    }

    /// Set CH3 bit atomically (RW)
    #[inline(always)]
    pub fn set_ch3(&self) {
        self.value.set_bits(Self::CH3_MASK);
    }

    /// Clear CH3 bit atomically (RW)
    #[inline(always)]
    pub fn clr_ch3(&self) {
        self.value.clr_bits(Self::CH3_MASK);
    }

    /// **CH2** `[2]` (RW)
    #[inline(always)]
    pub fn ch2(&self) -> bool {
        (self.value.read_bits() & Self::CH2_MASK) != 0
    }

    /// Set CH2 bit atomically (RW)
    #[inline(always)]
    pub fn set_ch2(&self) {
        self.value.set_bits(Self::CH2_MASK);
    }

    /// Clear CH2 bit atomically (RW)
    #[inline(always)]
    pub fn clr_ch2(&self) {
        self.value.clr_bits(Self::CH2_MASK);
    }

    /// **CH1** `[1]` (RW)
    #[inline(always)]
    pub fn ch1(&self) -> bool {
        (self.value.read_bits() & Self::CH1_MASK) != 0
    }

    /// Set CH1 bit atomically (RW)
    #[inline(always)]
    pub fn set_ch1(&self) {
        self.value.set_bits(Self::CH1_MASK);
    }

    /// Clear CH1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_ch1(&self) {
        self.value.clr_bits(Self::CH1_MASK);
    }

    /// **CH0** `[0]` (RW)
    #[inline(always)]
    pub fn ch0(&self) -> bool {
        (self.value.read_bits() & Self::CH0_MASK) != 0
    }

    /// Set CH0 bit atomically (RW)
    #[inline(always)]
    pub fn set_ch0(&self) {
        self.value.set_bits(Self::CH0_MASK);
    }

    /// Clear CH0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_ch0(&self) {
        self.value.clr_bits(Self::CH0_MASK);
    }

}

/// IRQ1_INTE value type for batch read/modify/write operations
///
/// Use `irq1inte.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `irq1inte.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Irq1InteVal {
    bits: u32,
}

impl Irq1InteVal {
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

    /// Read **CH11** `[11:11]`
    #[inline(always)]
    pub const fn ch11(&self) -> bool {
        (self.bits & Irq1Inte::CH11_MASK) != 0
    }

    /// Read **CH10** `[10:10]`
    #[inline(always)]
    pub const fn ch10(&self) -> bool {
        (self.bits & Irq1Inte::CH10_MASK) != 0
    }

    /// Read **CH9** `[9:9]`
    #[inline(always)]
    pub const fn ch9(&self) -> bool {
        (self.bits & Irq1Inte::CH9_MASK) != 0
    }

    /// Read **CH8** `[8:8]`
    #[inline(always)]
    pub const fn ch8(&self) -> bool {
        (self.bits & Irq1Inte::CH8_MASK) != 0
    }

    /// Read **CH7** `[7:7]`
    #[inline(always)]
    pub const fn ch7(&self) -> bool {
        (self.bits & Irq1Inte::CH7_MASK) != 0
    }

    /// Read **CH6** `[6:6]`
    #[inline(always)]
    pub const fn ch6(&self) -> bool {
        (self.bits & Irq1Inte::CH6_MASK) != 0
    }

    /// Read **CH5** `[5:5]`
    #[inline(always)]
    pub const fn ch5(&self) -> bool {
        (self.bits & Irq1Inte::CH5_MASK) != 0
    }

    /// Read **CH4** `[4:4]`
    #[inline(always)]
    pub const fn ch4(&self) -> bool {
        (self.bits & Irq1Inte::CH4_MASK) != 0
    }

    /// Read **CH3** `[3:3]`
    #[inline(always)]
    pub const fn ch3(&self) -> bool {
        (self.bits & Irq1Inte::CH3_MASK) != 0
    }

    /// Read **CH2** `[2:2]`
    #[inline(always)]
    pub const fn ch2(&self) -> bool {
        (self.bits & Irq1Inte::CH2_MASK) != 0
    }

    /// Read **CH1** `[1:1]`
    #[inline(always)]
    pub const fn ch1(&self) -> bool {
        (self.bits & Irq1Inte::CH1_MASK) != 0
    }

    /// Read **CH0** `[0:0]`
    #[inline(always)]
    pub const fn ch0(&self) -> bool {
        (self.bits & Irq1Inte::CH0_MASK) != 0
    }

    /// Set **CH11** `[11:11]`
    #[inline(always)]
    pub const fn with_ch11(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq1Inte::CH11_MASK
            } else {
                self.bits & !Irq1Inte::CH11_MASK
            }
        }
    }

    /// Set **CH10** `[10:10]`
    #[inline(always)]
    pub const fn with_ch10(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq1Inte::CH10_MASK
            } else {
                self.bits & !Irq1Inte::CH10_MASK
            }
        }
    }

    /// Set **CH9** `[9:9]`
    #[inline(always)]
    pub const fn with_ch9(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq1Inte::CH9_MASK
            } else {
                self.bits & !Irq1Inte::CH9_MASK
            }
        }
    }

    /// Set **CH8** `[8:8]`
    #[inline(always)]
    pub const fn with_ch8(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq1Inte::CH8_MASK
            } else {
                self.bits & !Irq1Inte::CH8_MASK
            }
        }
    }

    /// Set **CH7** `[7:7]`
    #[inline(always)]
    pub const fn with_ch7(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq1Inte::CH7_MASK
            } else {
                self.bits & !Irq1Inte::CH7_MASK
            }
        }
    }

    /// Set **CH6** `[6:6]`
    #[inline(always)]
    pub const fn with_ch6(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq1Inte::CH6_MASK
            } else {
                self.bits & !Irq1Inte::CH6_MASK
            }
        }
    }

    /// Set **CH5** `[5:5]`
    #[inline(always)]
    pub const fn with_ch5(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq1Inte::CH5_MASK
            } else {
                self.bits & !Irq1Inte::CH5_MASK
            }
        }
    }

    /// Set **CH4** `[4:4]`
    #[inline(always)]
    pub const fn with_ch4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq1Inte::CH4_MASK
            } else {
                self.bits & !Irq1Inte::CH4_MASK
            }
        }
    }

    /// Set **CH3** `[3:3]`
    #[inline(always)]
    pub const fn with_ch3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq1Inte::CH3_MASK
            } else {
                self.bits & !Irq1Inte::CH3_MASK
            }
        }
    }

    /// Set **CH2** `[2:2]`
    #[inline(always)]
    pub const fn with_ch2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq1Inte::CH2_MASK
            } else {
                self.bits & !Irq1Inte::CH2_MASK
            }
        }
    }

    /// Set **CH1** `[1:1]`
    #[inline(always)]
    pub const fn with_ch1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq1Inte::CH1_MASK
            } else {
                self.bits & !Irq1Inte::CH1_MASK
            }
        }
    }

    /// Set **CH0** `[0:0]`
    #[inline(always)]
    pub const fn with_ch0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq1Inte::CH0_MASK
            } else {
                self.bits & !Irq1Inte::CH0_MASK
            }
        }
    }

}

impl Default for Irq1InteVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Irq1Inte {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Irq1InteVal {
        Irq1InteVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Irq1InteVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Irq1InteVal) -> Irq1InteVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// IRQ1_INTF Register
///
/// Interrupt Force for irq1
///
/// Address offset: 0x0108
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CH11** `[11]` (RW)
///
/// - **CH10** `[10]` (RW)
///
/// - **CH9** `[9]` (RW)
///
/// - **CH8** `[8]` (RW)
///
/// - **CH7** `[7]` (RW)
///
/// - **CH6** `[6]` (RW)
///
/// - **CH5** `[5]` (RW)
///
/// - **CH4** `[4]` (RW)
///
/// - **CH3** `[3]` (RW)
///
/// - **CH2** `[2]` (RW)
///
/// - **CH1** `[1]` (RW)
///
/// - **CH0** `[0]` (RW)
#[repr(transparent)]
pub struct Irq1Intf {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Irq1Intf {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Irq1Intf {
    // ======== Field Constants ========

    /// **CH11**
    /// Bits `[11:11]`, mask `0x00000800`
    pub const CH11_OFFSET: u32 = 11;
    pub const CH11_MASK: u32 = 0x00000800;
    pub const CH11: u32 = 1 << 11;

    /// **CH10**
    /// Bits `[10:10]`, mask `0x00000400`
    pub const CH10_OFFSET: u32 = 10;
    pub const CH10_MASK: u32 = 0x00000400;
    pub const CH10: u32 = 1 << 10;

    /// **CH9**
    /// Bits `[9:9]`, mask `0x00000200`
    pub const CH9_OFFSET: u32 = 9;
    pub const CH9_MASK: u32 = 0x00000200;
    pub const CH9: u32 = 1 << 9;

    /// **CH8**
    /// Bits `[8:8]`, mask `0x00000100`
    pub const CH8_OFFSET: u32 = 8;
    pub const CH8_MASK: u32 = 0x00000100;
    pub const CH8: u32 = 1 << 8;

    /// **CH7**
    /// Bits `[7:7]`, mask `0x00000080`
    pub const CH7_OFFSET: u32 = 7;
    pub const CH7_MASK: u32 = 0x00000080;
    pub const CH7: u32 = 1 << 7;

    /// **CH6**
    /// Bits `[6:6]`, mask `0x00000040`
    pub const CH6_OFFSET: u32 = 6;
    pub const CH6_MASK: u32 = 0x00000040;
    pub const CH6: u32 = 1 << 6;

    /// **CH5**
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CH5_OFFSET: u32 = 5;
    pub const CH5_MASK: u32 = 0x00000020;
    pub const CH5: u32 = 1 << 5;

    /// **CH4**
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CH4_OFFSET: u32 = 4;
    pub const CH4_MASK: u32 = 0x00000010;
    pub const CH4: u32 = 1 << 4;

    /// **CH3**
    /// Bits `[3:3]`, mask `0x00000008`
    pub const CH3_OFFSET: u32 = 3;
    pub const CH3_MASK: u32 = 0x00000008;
    pub const CH3: u32 = 1 << 3;

    /// **CH2**
    /// Bits `[2:2]`, mask `0x00000004`
    pub const CH2_OFFSET: u32 = 2;
    pub const CH2_MASK: u32 = 0x00000004;
    pub const CH2: u32 = 1 << 2;

    /// **CH1**
    /// Bits `[1:1]`, mask `0x00000002`
    pub const CH1_OFFSET: u32 = 1;
    pub const CH1_MASK: u32 = 0x00000002;
    pub const CH1: u32 = 1 << 1;

    /// **CH0**
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CH0_OFFSET: u32 = 0;
    pub const CH0_MASK: u32 = 0x00000001;
    pub const CH0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CH11** `[11]` (RW)
    #[inline(always)]
    pub fn ch11(&self) -> bool {
        (self.value.read_bits() & Self::CH11_MASK) != 0
    }

    /// Set CH11 bit atomically (RW)
    #[inline(always)]
    pub fn set_ch11(&self) {
        self.value.set_bits(Self::CH11_MASK);
    }

    /// Clear CH11 bit atomically (RW)
    #[inline(always)]
    pub fn clr_ch11(&self) {
        self.value.clr_bits(Self::CH11_MASK);
    }

    /// **CH10** `[10]` (RW)
    #[inline(always)]
    pub fn ch10(&self) -> bool {
        (self.value.read_bits() & Self::CH10_MASK) != 0
    }

    /// Set CH10 bit atomically (RW)
    #[inline(always)]
    pub fn set_ch10(&self) {
        self.value.set_bits(Self::CH10_MASK);
    }

    /// Clear CH10 bit atomically (RW)
    #[inline(always)]
    pub fn clr_ch10(&self) {
        self.value.clr_bits(Self::CH10_MASK);
    }

    /// **CH9** `[9]` (RW)
    #[inline(always)]
    pub fn ch9(&self) -> bool {
        (self.value.read_bits() & Self::CH9_MASK) != 0
    }

    /// Set CH9 bit atomically (RW)
    #[inline(always)]
    pub fn set_ch9(&self) {
        self.value.set_bits(Self::CH9_MASK);
    }

    /// Clear CH9 bit atomically (RW)
    #[inline(always)]
    pub fn clr_ch9(&self) {
        self.value.clr_bits(Self::CH9_MASK);
    }

    /// **CH8** `[8]` (RW)
    #[inline(always)]
    pub fn ch8(&self) -> bool {
        (self.value.read_bits() & Self::CH8_MASK) != 0
    }

    /// Set CH8 bit atomically (RW)
    #[inline(always)]
    pub fn set_ch8(&self) {
        self.value.set_bits(Self::CH8_MASK);
    }

    /// Clear CH8 bit atomically (RW)
    #[inline(always)]
    pub fn clr_ch8(&self) {
        self.value.clr_bits(Self::CH8_MASK);
    }

    /// **CH7** `[7]` (RW)
    #[inline(always)]
    pub fn ch7(&self) -> bool {
        (self.value.read_bits() & Self::CH7_MASK) != 0
    }

    /// Set CH7 bit atomically (RW)
    #[inline(always)]
    pub fn set_ch7(&self) {
        self.value.set_bits(Self::CH7_MASK);
    }

    /// Clear CH7 bit atomically (RW)
    #[inline(always)]
    pub fn clr_ch7(&self) {
        self.value.clr_bits(Self::CH7_MASK);
    }

    /// **CH6** `[6]` (RW)
    #[inline(always)]
    pub fn ch6(&self) -> bool {
        (self.value.read_bits() & Self::CH6_MASK) != 0
    }

    /// Set CH6 bit atomically (RW)
    #[inline(always)]
    pub fn set_ch6(&self) {
        self.value.set_bits(Self::CH6_MASK);
    }

    /// Clear CH6 bit atomically (RW)
    #[inline(always)]
    pub fn clr_ch6(&self) {
        self.value.clr_bits(Self::CH6_MASK);
    }

    /// **CH5** `[5]` (RW)
    #[inline(always)]
    pub fn ch5(&self) -> bool {
        (self.value.read_bits() & Self::CH5_MASK) != 0
    }

    /// Set CH5 bit atomically (RW)
    #[inline(always)]
    pub fn set_ch5(&self) {
        self.value.set_bits(Self::CH5_MASK);
    }

    /// Clear CH5 bit atomically (RW)
    #[inline(always)]
    pub fn clr_ch5(&self) {
        self.value.clr_bits(Self::CH5_MASK);
    }

    /// **CH4** `[4]` (RW)
    #[inline(always)]
    pub fn ch4(&self) -> bool {
        (self.value.read_bits() & Self::CH4_MASK) != 0
    }

    /// Set CH4 bit atomically (RW)
    #[inline(always)]
    pub fn set_ch4(&self) {
        self.value.set_bits(Self::CH4_MASK);
    }

    /// Clear CH4 bit atomically (RW)
    #[inline(always)]
    pub fn clr_ch4(&self) {
        self.value.clr_bits(Self::CH4_MASK);
    }

    /// **CH3** `[3]` (RW)
    #[inline(always)]
    pub fn ch3(&self) -> bool {
        (self.value.read_bits() & Self::CH3_MASK) != 0
    }

    /// Set CH3 bit atomically (RW)
    #[inline(always)]
    pub fn set_ch3(&self) {
        self.value.set_bits(Self::CH3_MASK);
    }

    /// Clear CH3 bit atomically (RW)
    #[inline(always)]
    pub fn clr_ch3(&self) {
        self.value.clr_bits(Self::CH3_MASK);
    }

    /// **CH2** `[2]` (RW)
    #[inline(always)]
    pub fn ch2(&self) -> bool {
        (self.value.read_bits() & Self::CH2_MASK) != 0
    }

    /// Set CH2 bit atomically (RW)
    #[inline(always)]
    pub fn set_ch2(&self) {
        self.value.set_bits(Self::CH2_MASK);
    }

    /// Clear CH2 bit atomically (RW)
    #[inline(always)]
    pub fn clr_ch2(&self) {
        self.value.clr_bits(Self::CH2_MASK);
    }

    /// **CH1** `[1]` (RW)
    #[inline(always)]
    pub fn ch1(&self) -> bool {
        (self.value.read_bits() & Self::CH1_MASK) != 0
    }

    /// Set CH1 bit atomically (RW)
    #[inline(always)]
    pub fn set_ch1(&self) {
        self.value.set_bits(Self::CH1_MASK);
    }

    /// Clear CH1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_ch1(&self) {
        self.value.clr_bits(Self::CH1_MASK);
    }

    /// **CH0** `[0]` (RW)
    #[inline(always)]
    pub fn ch0(&self) -> bool {
        (self.value.read_bits() & Self::CH0_MASK) != 0
    }

    /// Set CH0 bit atomically (RW)
    #[inline(always)]
    pub fn set_ch0(&self) {
        self.value.set_bits(Self::CH0_MASK);
    }

    /// Clear CH0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_ch0(&self) {
        self.value.clr_bits(Self::CH0_MASK);
    }

}

/// IRQ1_INTF value type for batch read/modify/write operations
///
/// Use `irq1intf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `irq1intf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Irq1IntfVal {
    bits: u32,
}

impl Irq1IntfVal {
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

    /// Read **CH11** `[11:11]`
    #[inline(always)]
    pub const fn ch11(&self) -> bool {
        (self.bits & Irq1Intf::CH11_MASK) != 0
    }

    /// Read **CH10** `[10:10]`
    #[inline(always)]
    pub const fn ch10(&self) -> bool {
        (self.bits & Irq1Intf::CH10_MASK) != 0
    }

    /// Read **CH9** `[9:9]`
    #[inline(always)]
    pub const fn ch9(&self) -> bool {
        (self.bits & Irq1Intf::CH9_MASK) != 0
    }

    /// Read **CH8** `[8:8]`
    #[inline(always)]
    pub const fn ch8(&self) -> bool {
        (self.bits & Irq1Intf::CH8_MASK) != 0
    }

    /// Read **CH7** `[7:7]`
    #[inline(always)]
    pub const fn ch7(&self) -> bool {
        (self.bits & Irq1Intf::CH7_MASK) != 0
    }

    /// Read **CH6** `[6:6]`
    #[inline(always)]
    pub const fn ch6(&self) -> bool {
        (self.bits & Irq1Intf::CH6_MASK) != 0
    }

    /// Read **CH5** `[5:5]`
    #[inline(always)]
    pub const fn ch5(&self) -> bool {
        (self.bits & Irq1Intf::CH5_MASK) != 0
    }

    /// Read **CH4** `[4:4]`
    #[inline(always)]
    pub const fn ch4(&self) -> bool {
        (self.bits & Irq1Intf::CH4_MASK) != 0
    }

    /// Read **CH3** `[3:3]`
    #[inline(always)]
    pub const fn ch3(&self) -> bool {
        (self.bits & Irq1Intf::CH3_MASK) != 0
    }

    /// Read **CH2** `[2:2]`
    #[inline(always)]
    pub const fn ch2(&self) -> bool {
        (self.bits & Irq1Intf::CH2_MASK) != 0
    }

    /// Read **CH1** `[1:1]`
    #[inline(always)]
    pub const fn ch1(&self) -> bool {
        (self.bits & Irq1Intf::CH1_MASK) != 0
    }

    /// Read **CH0** `[0:0]`
    #[inline(always)]
    pub const fn ch0(&self) -> bool {
        (self.bits & Irq1Intf::CH0_MASK) != 0
    }

    /// Set **CH11** `[11:11]`
    #[inline(always)]
    pub const fn with_ch11(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq1Intf::CH11_MASK
            } else {
                self.bits & !Irq1Intf::CH11_MASK
            }
        }
    }

    /// Set **CH10** `[10:10]`
    #[inline(always)]
    pub const fn with_ch10(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq1Intf::CH10_MASK
            } else {
                self.bits & !Irq1Intf::CH10_MASK
            }
        }
    }

    /// Set **CH9** `[9:9]`
    #[inline(always)]
    pub const fn with_ch9(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq1Intf::CH9_MASK
            } else {
                self.bits & !Irq1Intf::CH9_MASK
            }
        }
    }

    /// Set **CH8** `[8:8]`
    #[inline(always)]
    pub const fn with_ch8(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq1Intf::CH8_MASK
            } else {
                self.bits & !Irq1Intf::CH8_MASK
            }
        }
    }

    /// Set **CH7** `[7:7]`
    #[inline(always)]
    pub const fn with_ch7(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq1Intf::CH7_MASK
            } else {
                self.bits & !Irq1Intf::CH7_MASK
            }
        }
    }

    /// Set **CH6** `[6:6]`
    #[inline(always)]
    pub const fn with_ch6(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq1Intf::CH6_MASK
            } else {
                self.bits & !Irq1Intf::CH6_MASK
            }
        }
    }

    /// Set **CH5** `[5:5]`
    #[inline(always)]
    pub const fn with_ch5(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq1Intf::CH5_MASK
            } else {
                self.bits & !Irq1Intf::CH5_MASK
            }
        }
    }

    /// Set **CH4** `[4:4]`
    #[inline(always)]
    pub const fn with_ch4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq1Intf::CH4_MASK
            } else {
                self.bits & !Irq1Intf::CH4_MASK
            }
        }
    }

    /// Set **CH3** `[3:3]`
    #[inline(always)]
    pub const fn with_ch3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq1Intf::CH3_MASK
            } else {
                self.bits & !Irq1Intf::CH3_MASK
            }
        }
    }

    /// Set **CH2** `[2:2]`
    #[inline(always)]
    pub const fn with_ch2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq1Intf::CH2_MASK
            } else {
                self.bits & !Irq1Intf::CH2_MASK
            }
        }
    }

    /// Set **CH1** `[1:1]`
    #[inline(always)]
    pub const fn with_ch1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq1Intf::CH1_MASK
            } else {
                self.bits & !Irq1Intf::CH1_MASK
            }
        }
    }

    /// Set **CH0** `[0:0]`
    #[inline(always)]
    pub const fn with_ch0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq1Intf::CH0_MASK
            } else {
                self.bits & !Irq1Intf::CH0_MASK
            }
        }
    }

}

impl Default for Irq1IntfVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Irq1Intf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Irq1IntfVal {
        Irq1IntfVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Irq1IntfVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Irq1IntfVal) -> Irq1IntfVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// IRQ1_INTS Register
///
/// Interrupt status after masking & forcing for irq1
///
/// Address offset: 0x010c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CH11** `[11]` (RO)
///
/// - **CH10** `[10]` (RO)
///
/// - **CH9** `[9]` (RO)
///
/// - **CH8** `[8]` (RO)
///
/// - **CH7** `[7]` (RO)
///
/// - **CH6** `[6]` (RO)
///
/// - **CH5** `[5]` (RO)
///
/// - **CH4** `[4]` (RO)
///
/// - **CH3** `[3]` (RO)
///
/// - **CH2** `[2]` (RO)
///
/// - **CH1** `[1]` (RO)
///
/// - **CH0** `[0]` (RO)
#[repr(transparent)]
pub struct Irq1Ints {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Irq1Ints {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Irq1Ints {
    // ======== Field Constants ========

    /// **CH11**
    /// Bits `[11:11]`, mask `0x00000800`
    pub const CH11_OFFSET: u32 = 11;
    pub const CH11_MASK: u32 = 0x00000800;
    pub const CH11: u32 = 1 << 11;

    /// **CH10**
    /// Bits `[10:10]`, mask `0x00000400`
    pub const CH10_OFFSET: u32 = 10;
    pub const CH10_MASK: u32 = 0x00000400;
    pub const CH10: u32 = 1 << 10;

    /// **CH9**
    /// Bits `[9:9]`, mask `0x00000200`
    pub const CH9_OFFSET: u32 = 9;
    pub const CH9_MASK: u32 = 0x00000200;
    pub const CH9: u32 = 1 << 9;

    /// **CH8**
    /// Bits `[8:8]`, mask `0x00000100`
    pub const CH8_OFFSET: u32 = 8;
    pub const CH8_MASK: u32 = 0x00000100;
    pub const CH8: u32 = 1 << 8;

    /// **CH7**
    /// Bits `[7:7]`, mask `0x00000080`
    pub const CH7_OFFSET: u32 = 7;
    pub const CH7_MASK: u32 = 0x00000080;
    pub const CH7: u32 = 1 << 7;

    /// **CH6**
    /// Bits `[6:6]`, mask `0x00000040`
    pub const CH6_OFFSET: u32 = 6;
    pub const CH6_MASK: u32 = 0x00000040;
    pub const CH6: u32 = 1 << 6;

    /// **CH5**
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CH5_OFFSET: u32 = 5;
    pub const CH5_MASK: u32 = 0x00000020;
    pub const CH5: u32 = 1 << 5;

    /// **CH4**
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CH4_OFFSET: u32 = 4;
    pub const CH4_MASK: u32 = 0x00000010;
    pub const CH4: u32 = 1 << 4;

    /// **CH3**
    /// Bits `[3:3]`, mask `0x00000008`
    pub const CH3_OFFSET: u32 = 3;
    pub const CH3_MASK: u32 = 0x00000008;
    pub const CH3: u32 = 1 << 3;

    /// **CH2**
    /// Bits `[2:2]`, mask `0x00000004`
    pub const CH2_OFFSET: u32 = 2;
    pub const CH2_MASK: u32 = 0x00000004;
    pub const CH2: u32 = 1 << 2;

    /// **CH1**
    /// Bits `[1:1]`, mask `0x00000002`
    pub const CH1_OFFSET: u32 = 1;
    pub const CH1_MASK: u32 = 0x00000002;
    pub const CH1: u32 = 1 << 1;

    /// **CH0**
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CH0_OFFSET: u32 = 0;
    pub const CH0_MASK: u32 = 0x00000001;
    pub const CH0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CH11** `[11]` (RO)
    #[inline(always)]
    pub fn ch11(&self) -> bool {
        (self.value.read_bits() & Self::CH11_MASK) != 0
    }

    /// **CH10** `[10]` (RO)
    #[inline(always)]
    pub fn ch10(&self) -> bool {
        (self.value.read_bits() & Self::CH10_MASK) != 0
    }

    /// **CH9** `[9]` (RO)
    #[inline(always)]
    pub fn ch9(&self) -> bool {
        (self.value.read_bits() & Self::CH9_MASK) != 0
    }

    /// **CH8** `[8]` (RO)
    #[inline(always)]
    pub fn ch8(&self) -> bool {
        (self.value.read_bits() & Self::CH8_MASK) != 0
    }

    /// **CH7** `[7]` (RO)
    #[inline(always)]
    pub fn ch7(&self) -> bool {
        (self.value.read_bits() & Self::CH7_MASK) != 0
    }

    /// **CH6** `[6]` (RO)
    #[inline(always)]
    pub fn ch6(&self) -> bool {
        (self.value.read_bits() & Self::CH6_MASK) != 0
    }

    /// **CH5** `[5]` (RO)
    #[inline(always)]
    pub fn ch5(&self) -> bool {
        (self.value.read_bits() & Self::CH5_MASK) != 0
    }

    /// **CH4** `[4]` (RO)
    #[inline(always)]
    pub fn ch4(&self) -> bool {
        (self.value.read_bits() & Self::CH4_MASK) != 0
    }

    /// **CH3** `[3]` (RO)
    #[inline(always)]
    pub fn ch3(&self) -> bool {
        (self.value.read_bits() & Self::CH3_MASK) != 0
    }

    /// **CH2** `[2]` (RO)
    #[inline(always)]
    pub fn ch2(&self) -> bool {
        (self.value.read_bits() & Self::CH2_MASK) != 0
    }

    /// **CH1** `[1]` (RO)
    #[inline(always)]
    pub fn ch1(&self) -> bool {
        (self.value.read_bits() & Self::CH1_MASK) != 0
    }

    /// **CH0** `[0]` (RO)
    #[inline(always)]
    pub fn ch0(&self) -> bool {
        (self.value.read_bits() & Self::CH0_MASK) != 0
    }

}

/// IRQ1_INTS value type for batch read/modify/write operations
///
/// Use `irq1ints.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `irq1ints.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Irq1IntsVal {
    bits: u32,
}

impl Irq1IntsVal {
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

    /// Read **CH11** `[11:11]`
    #[inline(always)]
    pub const fn ch11(&self) -> bool {
        (self.bits & Irq1Ints::CH11_MASK) != 0
    }

    /// Read **CH10** `[10:10]`
    #[inline(always)]
    pub const fn ch10(&self) -> bool {
        (self.bits & Irq1Ints::CH10_MASK) != 0
    }

    /// Read **CH9** `[9:9]`
    #[inline(always)]
    pub const fn ch9(&self) -> bool {
        (self.bits & Irq1Ints::CH9_MASK) != 0
    }

    /// Read **CH8** `[8:8]`
    #[inline(always)]
    pub const fn ch8(&self) -> bool {
        (self.bits & Irq1Ints::CH8_MASK) != 0
    }

    /// Read **CH7** `[7:7]`
    #[inline(always)]
    pub const fn ch7(&self) -> bool {
        (self.bits & Irq1Ints::CH7_MASK) != 0
    }

    /// Read **CH6** `[6:6]`
    #[inline(always)]
    pub const fn ch6(&self) -> bool {
        (self.bits & Irq1Ints::CH6_MASK) != 0
    }

    /// Read **CH5** `[5:5]`
    #[inline(always)]
    pub const fn ch5(&self) -> bool {
        (self.bits & Irq1Ints::CH5_MASK) != 0
    }

    /// Read **CH4** `[4:4]`
    #[inline(always)]
    pub const fn ch4(&self) -> bool {
        (self.bits & Irq1Ints::CH4_MASK) != 0
    }

    /// Read **CH3** `[3:3]`
    #[inline(always)]
    pub const fn ch3(&self) -> bool {
        (self.bits & Irq1Ints::CH3_MASK) != 0
    }

    /// Read **CH2** `[2:2]`
    #[inline(always)]
    pub const fn ch2(&self) -> bool {
        (self.bits & Irq1Ints::CH2_MASK) != 0
    }

    /// Read **CH1** `[1:1]`
    #[inline(always)]
    pub const fn ch1(&self) -> bool {
        (self.bits & Irq1Ints::CH1_MASK) != 0
    }

    /// Read **CH0** `[0:0]`
    #[inline(always)]
    pub const fn ch0(&self) -> bool {
        (self.bits & Irq1Ints::CH0_MASK) != 0
    }

}

impl Default for Irq1IntsVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Irq1Ints {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Irq1IntsVal {
        Irq1IntsVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Irq1IntsVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Irq1IntsVal) -> Irq1IntsVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// Control and status register
    pub ch0_csr: Ch0Csr,
    /// 0x0004
    /// INT and FRAC form a fixed-point fractional number.
    /// Counting rate is system clock frequency divided by this number.
    /// Fractional division uses simple 1st-order sigma-delta.
    pub ch0_div: Ch0Div,
    /// 0x0008
    /// Direct access to the PWM counter
    pub ch0_ctr: Ch0Ctr,
    /// 0x000c
    /// Counter compare values
    pub ch0_cc: Ch0Cc,
    /// 0x0010
    /// Counter wrap value
    pub ch0_top: Ch0Top,
    /// 0x0014
    /// Control and status register
    pub ch1_csr: Ch1Csr,
    /// 0x0018
    /// INT and FRAC form a fixed-point fractional number.
    /// Counting rate is system clock frequency divided by this number.
    /// Fractional division uses simple 1st-order sigma-delta.
    pub ch1_div: Ch1Div,
    /// 0x001c
    /// Direct access to the PWM counter
    pub ch1_ctr: Ch1Ctr,
    /// 0x0020
    /// Counter compare values
    pub ch1_cc: Ch1Cc,
    /// 0x0024
    /// Counter wrap value
    pub ch1_top: Ch1Top,
    /// 0x0028
    /// Control and status register
    pub ch2_csr: Ch2Csr,
    /// 0x002c
    /// INT and FRAC form a fixed-point fractional number.
    /// Counting rate is system clock frequency divided by this number.
    /// Fractional division uses simple 1st-order sigma-delta.
    pub ch2_div: Ch2Div,
    /// 0x0030
    /// Direct access to the PWM counter
    pub ch2_ctr: Ch2Ctr,
    /// 0x0034
    /// Counter compare values
    pub ch2_cc: Ch2Cc,
    /// 0x0038
    /// Counter wrap value
    pub ch2_top: Ch2Top,
    /// 0x003c
    /// Control and status register
    pub ch3_csr: Ch3Csr,
    /// 0x0040
    /// INT and FRAC form a fixed-point fractional number.
    /// Counting rate is system clock frequency divided by this number.
    /// Fractional division uses simple 1st-order sigma-delta.
    pub ch3_div: Ch3Div,
    /// 0x0044
    /// Direct access to the PWM counter
    pub ch3_ctr: Ch3Ctr,
    /// 0x0048
    /// Counter compare values
    pub ch3_cc: Ch3Cc,
    /// 0x004c
    /// Counter wrap value
    pub ch3_top: Ch3Top,
    /// 0x0050
    /// Control and status register
    pub ch4_csr: Ch4Csr,
    /// 0x0054
    /// INT and FRAC form a fixed-point fractional number.
    /// Counting rate is system clock frequency divided by this number.
    /// Fractional division uses simple 1st-order sigma-delta.
    pub ch4_div: Ch4Div,
    /// 0x0058
    /// Direct access to the PWM counter
    pub ch4_ctr: Ch4Ctr,
    /// 0x005c
    /// Counter compare values
    pub ch4_cc: Ch4Cc,
    /// 0x0060
    /// Counter wrap value
    pub ch4_top: Ch4Top,
    /// 0x0064
    /// Control and status register
    pub ch5_csr: Ch5Csr,
    /// 0x0068
    /// INT and FRAC form a fixed-point fractional number.
    /// Counting rate is system clock frequency divided by this number.
    /// Fractional division uses simple 1st-order sigma-delta.
    pub ch5_div: Ch5Div,
    /// 0x006c
    /// Direct access to the PWM counter
    pub ch5_ctr: Ch5Ctr,
    /// 0x0070
    /// Counter compare values
    pub ch5_cc: Ch5Cc,
    /// 0x0074
    /// Counter wrap value
    pub ch5_top: Ch5Top,
    /// 0x0078
    /// Control and status register
    pub ch6_csr: Ch6Csr,
    /// 0x007c
    /// INT and FRAC form a fixed-point fractional number.
    /// Counting rate is system clock frequency divided by this number.
    /// Fractional division uses simple 1st-order sigma-delta.
    pub ch6_div: Ch6Div,
    /// 0x0080
    /// Direct access to the PWM counter
    pub ch6_ctr: Ch6Ctr,
    /// 0x0084
    /// Counter compare values
    pub ch6_cc: Ch6Cc,
    /// 0x0088
    /// Counter wrap value
    pub ch6_top: Ch6Top,
    /// 0x008c
    /// Control and status register
    pub ch7_csr: Ch7Csr,
    /// 0x0090
    /// INT and FRAC form a fixed-point fractional number.
    /// Counting rate is system clock frequency divided by this number.
    /// Fractional division uses simple 1st-order sigma-delta.
    pub ch7_div: Ch7Div,
    /// 0x0094
    /// Direct access to the PWM counter
    pub ch7_ctr: Ch7Ctr,
    /// 0x0098
    /// Counter compare values
    pub ch7_cc: Ch7Cc,
    /// 0x009c
    /// Counter wrap value
    pub ch7_top: Ch7Top,
    /// 0x00a0
    /// Control and status register
    pub ch8_csr: Ch8Csr,
    /// 0x00a4
    /// INT and FRAC form a fixed-point fractional number.
    /// Counting rate is system clock frequency divided by this number.
    /// Fractional division uses simple 1st-order sigma-delta.
    pub ch8_div: Ch8Div,
    /// 0x00a8
    /// Direct access to the PWM counter
    pub ch8_ctr: Ch8Ctr,
    /// 0x00ac
    /// Counter compare values
    pub ch8_cc: Ch8Cc,
    /// 0x00b0
    /// Counter wrap value
    pub ch8_top: Ch8Top,
    /// 0x00b4
    /// Control and status register
    pub ch9_csr: Ch9Csr,
    /// 0x00b8
    /// INT and FRAC form a fixed-point fractional number.
    /// Counting rate is system clock frequency divided by this number.
    /// Fractional division uses simple 1st-order sigma-delta.
    pub ch9_div: Ch9Div,
    /// 0x00bc
    /// Direct access to the PWM counter
    pub ch9_ctr: Ch9Ctr,
    /// 0x00c0
    /// Counter compare values
    pub ch9_cc: Ch9Cc,
    /// 0x00c4
    /// Counter wrap value
    pub ch9_top: Ch9Top,
    /// 0x00c8
    /// Control and status register
    pub ch10_csr: Ch10Csr,
    /// 0x00cc
    /// INT and FRAC form a fixed-point fractional number.
    /// Counting rate is system clock frequency divided by this number.
    /// Fractional division uses simple 1st-order sigma-delta.
    pub ch10_div: Ch10Div,
    /// 0x00d0
    /// Direct access to the PWM counter
    pub ch10_ctr: Ch10Ctr,
    /// 0x00d4
    /// Counter compare values
    pub ch10_cc: Ch10Cc,
    /// 0x00d8
    /// Counter wrap value
    pub ch10_top: Ch10Top,
    /// 0x00dc
    /// Control and status register
    pub ch11_csr: Ch11Csr,
    /// 0x00e0
    /// INT and FRAC form a fixed-point fractional number.
    /// Counting rate is system clock frequency divided by this number.
    /// Fractional division uses simple 1st-order sigma-delta.
    pub ch11_div: Ch11Div,
    /// 0x00e4
    /// Direct access to the PWM counter
    pub ch11_ctr: Ch11Ctr,
    /// 0x00e8
    /// Counter compare values
    pub ch11_cc: Ch11Cc,
    /// 0x00ec
    /// Counter wrap value
    pub ch11_top: Ch11Top,
    /// 0x00f0
    /// This register aliases the CSR_EN bits for all channels.
    /// Writing to this register allows multiple channels to be enabled
    /// or disabled simultaneously, so they can run in perfect sync.
    /// For each channel, there is only one physical EN register bit,
    /// which can be accessed through here or CHx_CSR.
    pub en: En,
    /// 0x00f4
    /// Raw Interrupts
    pub intr: Intr,
    /// 0x00f8
    /// Interrupt Enable for irq0
    pub irq0_inte: Irq0Inte,
    /// 0x00fc
    /// Interrupt Force for irq0
    pub irq0_intf: Irq0Intf,
    /// 0x0100
    /// Interrupt status after masking & forcing for irq0
    pub irq0_ints: Irq0Ints,
    /// 0x0104
    /// Interrupt Enable for irq1
    pub irq1_inte: Irq1Inte,
    /// 0x0108
    /// Interrupt Force for irq1
    pub irq1_intf: Irq1Intf,
    /// 0x010c
    /// Interrupt status after masking & forcing for irq1
    pub irq1_ints: Irq1Ints,
}

