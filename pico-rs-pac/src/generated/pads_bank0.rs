//! PADS_BANK0
//!
//! This peripheral supports atomic SET/CLR/XOR register aliases.

#![allow(dead_code)]

pub const BASE: u32 = 0x40038000;

/// VOLTAGE_SELECT Register
///
/// Voltage select. Per bank control
///
/// Address offset: 0x0000
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **VOLTAGE_SELECT** `[0]` (RW)
#[repr(transparent)]
pub struct VoltageSelect {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for VoltageSelect {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl VoltageSelect {
    // ======== Field Constants ========

    /// **VOLTAGE_SELECT**
    /// Bits `[0:0]`, mask `0x00000001`
    pub const VOLTAGE_SELECT_OFFSET: u32 = 0;
    pub const VOLTAGE_SELECT_MASK: u32 = 0x00000001;
    pub const VOLTAGE_SELECT: u32 = 1 << 0;
    /// 3v3 = 0
    /// Set voltage to 3.3V (DVDD >= 2V5)
    pub const VOLTAGE_SELECT_3V3: u32 = 0 << 0;
    pub const VOLTAGE_SELECT_3V3_VAL: u32 = 0;
    /// 1v8 = 1
    /// Set voltage to 1.8V (DVDD <= 1V8)
    pub const VOLTAGE_SELECT_1V8: u32 = 1 << 0;
    pub const VOLTAGE_SELECT_1V8_VAL: u32 = 1;

    // ======== Field Accessors ========

    /// **VOLTAGE_SELECT** `[0]` (RW)
    #[inline(always)]
    pub fn voltage_select(&self) -> bool {
        (self.value.read_bits() & Self::VOLTAGE_SELECT_MASK) != 0
    }

    /// Set VOLTAGE_SELECT bit atomically (RW)
    #[inline(always)]
    pub fn set_voltage_select(&self) {
        self.set_bits(Self::VOLTAGE_SELECT_MASK);
    }

    /// Clear VOLTAGE_SELECT bit atomically (RW)
    #[inline(always)]
    pub fn clr_voltage_select(&self) {
        self.clr_bits(Self::VOLTAGE_SELECT_MASK);
    }

}

/// VOLTAGE_SELECT value type for batch read/modify/write operations
///
/// Use `voltageselect.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `voltageselect.write_val(val)`.
#[derive(Clone, Copy)]
pub struct VoltageSelectVal {
    bits: u32,
}

impl VoltageSelectVal {
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

    /// Read **VOLTAGE_SELECT** `[0:0]`
    #[inline(always)]
    pub const fn voltage_select(&self) -> bool {
        (self.bits & VoltageSelect::VOLTAGE_SELECT_MASK) != 0
    }

    /// Set **VOLTAGE_SELECT** `[0:0]`
    #[inline(always)]
    pub const fn with_voltage_select(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | VoltageSelect::VOLTAGE_SELECT_MASK
            } else {
                self.bits & !VoltageSelect::VOLTAGE_SELECT_MASK
            }
        }
    }

}

impl Default for VoltageSelectVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl VoltageSelect {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> VoltageSelectVal {
        VoltageSelectVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: VoltageSelectVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(VoltageSelectVal) -> VoltageSelectVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// GPIO0 Register
///
/// Address offset: 0x0004
/// Reset value: 0x00000116
///
/// ## Fields
///
/// - **ISO** `[8]` (RW)
///
///   Pad isolation control. Remove this once the pad is configured by software.
///
/// - **OD** `[7]` (RW)
///
///   Output disable. Has priority over output enable from peripherals
///
/// - **IE** `[6]` (RW)
///
///   Input enable
///
/// - **DRIVE** `[5:4]` (RW)
///
///   Drive strength.
///
/// - **PUE** `[3]` (RW)
///
///   Pull up enable
///
/// - **PDE** `[2]` (RW)
///
///   Pull down enable
///
/// - **SCHMITT** `[1]` (RW)
///
///   Enable schmitt trigger
///
/// - **SLEWFAST** `[0]` (RW)
///
///   Slew rate control. 1 = Fast, 0 = Slow
#[repr(transparent)]
pub struct Gpio0 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Gpio0 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Gpio0 {
    // ======== Field Constants ========

    /// **ISO**
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ISO_OFFSET: u32 = 8;
    pub const ISO_MASK: u32 = 0x00000100;
    pub const ISO: u32 = 1 << 8;

    /// **OD**
    ///
    /// Output disable. Has priority over output enable from peripherals
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const OD_OFFSET: u32 = 7;
    pub const OD_MASK: u32 = 0x00000080;
    pub const OD: u32 = 1 << 7;

    /// **IE**
    ///
    /// Input enable
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const IE_OFFSET: u32 = 6;
    pub const IE_MASK: u32 = 0x00000040;
    pub const IE: u32 = 1 << 6;

    /// **DRIVE**
    ///
    /// Drive strength.
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const DRIVE_OFFSET: u32 = 4;
    pub const DRIVE_MASK: u32 = 0x00000030;
    pub const DRIVE_2M_A: u32 = 0 << 4;
    pub const DRIVE_2M_A_VAL: u32 = 0;
    pub const DRIVE_4M_A: u32 = 1 << 4;
    pub const DRIVE_4M_A_VAL: u32 = 1;
    pub const DRIVE_8M_A: u32 = 2 << 4;
    pub const DRIVE_8M_A_VAL: u32 = 2;
    pub const DRIVE_12M_A: u32 = 3 << 4;
    pub const DRIVE_12M_A_VAL: u32 = 3;

    /// **PUE**
    ///
    /// Pull up enable
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const PUE_OFFSET: u32 = 3;
    pub const PUE_MASK: u32 = 0x00000008;
    pub const PUE: u32 = 1 << 3;

    /// **PDE**
    ///
    /// Pull down enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const PDE_OFFSET: u32 = 2;
    pub const PDE_MASK: u32 = 0x00000004;
    pub const PDE: u32 = 1 << 2;

    /// **SCHMITT**
    ///
    /// Enable schmitt trigger
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SCHMITT_OFFSET: u32 = 1;
    pub const SCHMITT_MASK: u32 = 0x00000002;
    pub const SCHMITT: u32 = 1 << 1;

    /// **SLEWFAST**
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SLEWFAST_OFFSET: u32 = 0;
    pub const SLEWFAST_MASK: u32 = 0x00000001;
    pub const SLEWFAST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ISO** `[8]` (RW)
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub fn iso(&self) -> bool {
        (self.value.read_bits() & Self::ISO_MASK) != 0
    }

    /// Set ISO bit atomically (RW)
    #[inline(always)]
    pub fn set_iso(&self) {
        self.set_bits(Self::ISO_MASK);
    }

    /// Clear ISO bit atomically (RW)
    #[inline(always)]
    pub fn clr_iso(&self) {
        self.clr_bits(Self::ISO_MASK);
    }

    /// **OD** `[7]` (RW)
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub fn od(&self) -> bool {
        (self.value.read_bits() & Self::OD_MASK) != 0
    }

    /// Set OD bit atomically (RW)
    #[inline(always)]
    pub fn set_od(&self) {
        self.set_bits(Self::OD_MASK);
    }

    /// Clear OD bit atomically (RW)
    #[inline(always)]
    pub fn clr_od(&self) {
        self.clr_bits(Self::OD_MASK);
    }

    /// **IE** `[6]` (RW)
    ///
    /// Input enable
    #[inline(always)]
    pub fn ie(&self) -> bool {
        (self.value.read_bits() & Self::IE_MASK) != 0
    }

    /// Set IE bit atomically (RW)
    #[inline(always)]
    pub fn set_ie(&self) {
        self.set_bits(Self::IE_MASK);
    }

    /// Clear IE bit atomically (RW)
    #[inline(always)]
    pub fn clr_ie(&self) {
        self.clr_bits(Self::IE_MASK);
    }

    /// **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn drive(&self) -> u32 {
        (self.value.read_bits() & Self::DRIVE_MASK) >> Self::DRIVE_OFFSET
    }

    /// Write **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn write_drive(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DRIVE_MASK) | ((value << Self::DRIVE_OFFSET) & Self::DRIVE_MASK));
    }

    /// **PUE** `[3]` (RW)
    ///
    /// Pull up enable
    #[inline(always)]
    pub fn pue(&self) -> bool {
        (self.value.read_bits() & Self::PUE_MASK) != 0
    }

    /// Set PUE bit atomically (RW)
    #[inline(always)]
    pub fn set_pue(&self) {
        self.set_bits(Self::PUE_MASK);
    }

    /// Clear PUE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pue(&self) {
        self.clr_bits(Self::PUE_MASK);
    }

    /// **PDE** `[2]` (RW)
    ///
    /// Pull down enable
    #[inline(always)]
    pub fn pde(&self) -> bool {
        (self.value.read_bits() & Self::PDE_MASK) != 0
    }

    /// Set PDE bit atomically (RW)
    #[inline(always)]
    pub fn set_pde(&self) {
        self.set_bits(Self::PDE_MASK);
    }

    /// Clear PDE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pde(&self) {
        self.clr_bits(Self::PDE_MASK);
    }

    /// **SCHMITT** `[1]` (RW)
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub fn schmitt(&self) -> bool {
        (self.value.read_bits() & Self::SCHMITT_MASK) != 0
    }

    /// Set SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn set_schmitt(&self) {
        self.set_bits(Self::SCHMITT_MASK);
    }

    /// Clear SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn clr_schmitt(&self) {
        self.clr_bits(Self::SCHMITT_MASK);
    }

    /// **SLEWFAST** `[0]` (RW)
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub fn slewfast(&self) -> bool {
        (self.value.read_bits() & Self::SLEWFAST_MASK) != 0
    }

    /// Set SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn set_slewfast(&self) {
        self.set_bits(Self::SLEWFAST_MASK);
    }

    /// Clear SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn clr_slewfast(&self) {
        self.clr_bits(Self::SLEWFAST_MASK);
    }

}

/// GPIO0 value type for batch read/modify/write operations
///
/// Use `gpio0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpio0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Gpio0Val {
    bits: u32,
}

impl Gpio0Val {
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
        Self { bits: 0x00000116 }
    }

    /// Read **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn iso(&self) -> bool {
        (self.bits & Gpio0::ISO_MASK) != 0
    }

    /// Read **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn od(&self) -> bool {
        (self.bits & Gpio0::OD_MASK) != 0
    }

    /// Read **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        (self.bits & Gpio0::IE_MASK) != 0
    }

    /// Read **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn drive(&self) -> u32 {
        (self.bits & Gpio0::DRIVE_MASK) >> Gpio0::DRIVE_OFFSET
    }

    /// Read **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn pue(&self) -> bool {
        (self.bits & Gpio0::PUE_MASK) != 0
    }

    /// Read **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn pde(&self) -> bool {
        (self.bits & Gpio0::PDE_MASK) != 0
    }

    /// Read **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn schmitt(&self) -> bool {
        (self.bits & Gpio0::SCHMITT_MASK) != 0
    }

    /// Read **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn slewfast(&self) -> bool {
        (self.bits & Gpio0::SLEWFAST_MASK) != 0
    }

    /// Set **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn with_iso(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio0::ISO_MASK
            } else {
                self.bits & !Gpio0::ISO_MASK
            }
        }
    }

    /// Set **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn with_od(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio0::OD_MASK
            } else {
                self.bits & !Gpio0::OD_MASK
            }
        }
    }

    /// Set **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn with_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio0::IE_MASK
            } else {
                self.bits & !Gpio0::IE_MASK
            }
        }
    }

    /// Set **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn with_drive(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Gpio0::DRIVE_MASK) | ((value << Gpio0::DRIVE_OFFSET) & Gpio0::DRIVE_MASK)
        }
    }

    /// Set **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn with_pue(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio0::PUE_MASK
            } else {
                self.bits & !Gpio0::PUE_MASK
            }
        }
    }

    /// Set **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn with_pde(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio0::PDE_MASK
            } else {
                self.bits & !Gpio0::PDE_MASK
            }
        }
    }

    /// Set **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn with_schmitt(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio0::SCHMITT_MASK
            } else {
                self.bits & !Gpio0::SCHMITT_MASK
            }
        }
    }

    /// Set **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn with_slewfast(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio0::SLEWFAST_MASK
            } else {
                self.bits & !Gpio0::SLEWFAST_MASK
            }
        }
    }

}

impl Default for Gpio0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Gpio0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Gpio0Val {
        Gpio0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Gpio0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Gpio0Val) -> Gpio0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// GPIO1 Register
///
/// Address offset: 0x0008
/// Reset value: 0x00000116
///
/// ## Fields
///
/// - **ISO** `[8]` (RW)
///
///   Pad isolation control. Remove this once the pad is configured by software.
///
/// - **OD** `[7]` (RW)
///
///   Output disable. Has priority over output enable from peripherals
///
/// - **IE** `[6]` (RW)
///
///   Input enable
///
/// - **DRIVE** `[5:4]` (RW)
///
///   Drive strength.
///
/// - **PUE** `[3]` (RW)
///
///   Pull up enable
///
/// - **PDE** `[2]` (RW)
///
///   Pull down enable
///
/// - **SCHMITT** `[1]` (RW)
///
///   Enable schmitt trigger
///
/// - **SLEWFAST** `[0]` (RW)
///
///   Slew rate control. 1 = Fast, 0 = Slow
#[repr(transparent)]
pub struct Gpio1 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Gpio1 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Gpio1 {
    // ======== Field Constants ========

    /// **ISO**
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ISO_OFFSET: u32 = 8;
    pub const ISO_MASK: u32 = 0x00000100;
    pub const ISO: u32 = 1 << 8;

    /// **OD**
    ///
    /// Output disable. Has priority over output enable from peripherals
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const OD_OFFSET: u32 = 7;
    pub const OD_MASK: u32 = 0x00000080;
    pub const OD: u32 = 1 << 7;

    /// **IE**
    ///
    /// Input enable
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const IE_OFFSET: u32 = 6;
    pub const IE_MASK: u32 = 0x00000040;
    pub const IE: u32 = 1 << 6;

    /// **DRIVE**
    ///
    /// Drive strength.
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const DRIVE_OFFSET: u32 = 4;
    pub const DRIVE_MASK: u32 = 0x00000030;
    pub const DRIVE_2M_A: u32 = 0 << 4;
    pub const DRIVE_2M_A_VAL: u32 = 0;
    pub const DRIVE_4M_A: u32 = 1 << 4;
    pub const DRIVE_4M_A_VAL: u32 = 1;
    pub const DRIVE_8M_A: u32 = 2 << 4;
    pub const DRIVE_8M_A_VAL: u32 = 2;
    pub const DRIVE_12M_A: u32 = 3 << 4;
    pub const DRIVE_12M_A_VAL: u32 = 3;

    /// **PUE**
    ///
    /// Pull up enable
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const PUE_OFFSET: u32 = 3;
    pub const PUE_MASK: u32 = 0x00000008;
    pub const PUE: u32 = 1 << 3;

    /// **PDE**
    ///
    /// Pull down enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const PDE_OFFSET: u32 = 2;
    pub const PDE_MASK: u32 = 0x00000004;
    pub const PDE: u32 = 1 << 2;

    /// **SCHMITT**
    ///
    /// Enable schmitt trigger
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SCHMITT_OFFSET: u32 = 1;
    pub const SCHMITT_MASK: u32 = 0x00000002;
    pub const SCHMITT: u32 = 1 << 1;

    /// **SLEWFAST**
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SLEWFAST_OFFSET: u32 = 0;
    pub const SLEWFAST_MASK: u32 = 0x00000001;
    pub const SLEWFAST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ISO** `[8]` (RW)
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub fn iso(&self) -> bool {
        (self.value.read_bits() & Self::ISO_MASK) != 0
    }

    /// Set ISO bit atomically (RW)
    #[inline(always)]
    pub fn set_iso(&self) {
        self.set_bits(Self::ISO_MASK);
    }

    /// Clear ISO bit atomically (RW)
    #[inline(always)]
    pub fn clr_iso(&self) {
        self.clr_bits(Self::ISO_MASK);
    }

    /// **OD** `[7]` (RW)
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub fn od(&self) -> bool {
        (self.value.read_bits() & Self::OD_MASK) != 0
    }

    /// Set OD bit atomically (RW)
    #[inline(always)]
    pub fn set_od(&self) {
        self.set_bits(Self::OD_MASK);
    }

    /// Clear OD bit atomically (RW)
    #[inline(always)]
    pub fn clr_od(&self) {
        self.clr_bits(Self::OD_MASK);
    }

    /// **IE** `[6]` (RW)
    ///
    /// Input enable
    #[inline(always)]
    pub fn ie(&self) -> bool {
        (self.value.read_bits() & Self::IE_MASK) != 0
    }

    /// Set IE bit atomically (RW)
    #[inline(always)]
    pub fn set_ie(&self) {
        self.set_bits(Self::IE_MASK);
    }

    /// Clear IE bit atomically (RW)
    #[inline(always)]
    pub fn clr_ie(&self) {
        self.clr_bits(Self::IE_MASK);
    }

    /// **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn drive(&self) -> u32 {
        (self.value.read_bits() & Self::DRIVE_MASK) >> Self::DRIVE_OFFSET
    }

    /// Write **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn write_drive(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DRIVE_MASK) | ((value << Self::DRIVE_OFFSET) & Self::DRIVE_MASK));
    }

    /// **PUE** `[3]` (RW)
    ///
    /// Pull up enable
    #[inline(always)]
    pub fn pue(&self) -> bool {
        (self.value.read_bits() & Self::PUE_MASK) != 0
    }

    /// Set PUE bit atomically (RW)
    #[inline(always)]
    pub fn set_pue(&self) {
        self.set_bits(Self::PUE_MASK);
    }

    /// Clear PUE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pue(&self) {
        self.clr_bits(Self::PUE_MASK);
    }

    /// **PDE** `[2]` (RW)
    ///
    /// Pull down enable
    #[inline(always)]
    pub fn pde(&self) -> bool {
        (self.value.read_bits() & Self::PDE_MASK) != 0
    }

    /// Set PDE bit atomically (RW)
    #[inline(always)]
    pub fn set_pde(&self) {
        self.set_bits(Self::PDE_MASK);
    }

    /// Clear PDE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pde(&self) {
        self.clr_bits(Self::PDE_MASK);
    }

    /// **SCHMITT** `[1]` (RW)
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub fn schmitt(&self) -> bool {
        (self.value.read_bits() & Self::SCHMITT_MASK) != 0
    }

    /// Set SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn set_schmitt(&self) {
        self.set_bits(Self::SCHMITT_MASK);
    }

    /// Clear SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn clr_schmitt(&self) {
        self.clr_bits(Self::SCHMITT_MASK);
    }

    /// **SLEWFAST** `[0]` (RW)
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub fn slewfast(&self) -> bool {
        (self.value.read_bits() & Self::SLEWFAST_MASK) != 0
    }

    /// Set SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn set_slewfast(&self) {
        self.set_bits(Self::SLEWFAST_MASK);
    }

    /// Clear SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn clr_slewfast(&self) {
        self.clr_bits(Self::SLEWFAST_MASK);
    }

}

/// GPIO1 value type for batch read/modify/write operations
///
/// Use `gpio1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpio1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Gpio1Val {
    bits: u32,
}

impl Gpio1Val {
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
        Self { bits: 0x00000116 }
    }

    /// Read **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn iso(&self) -> bool {
        (self.bits & Gpio1::ISO_MASK) != 0
    }

    /// Read **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn od(&self) -> bool {
        (self.bits & Gpio1::OD_MASK) != 0
    }

    /// Read **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        (self.bits & Gpio1::IE_MASK) != 0
    }

    /// Read **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn drive(&self) -> u32 {
        (self.bits & Gpio1::DRIVE_MASK) >> Gpio1::DRIVE_OFFSET
    }

    /// Read **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn pue(&self) -> bool {
        (self.bits & Gpio1::PUE_MASK) != 0
    }

    /// Read **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn pde(&self) -> bool {
        (self.bits & Gpio1::PDE_MASK) != 0
    }

    /// Read **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn schmitt(&self) -> bool {
        (self.bits & Gpio1::SCHMITT_MASK) != 0
    }

    /// Read **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn slewfast(&self) -> bool {
        (self.bits & Gpio1::SLEWFAST_MASK) != 0
    }

    /// Set **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn with_iso(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio1::ISO_MASK
            } else {
                self.bits & !Gpio1::ISO_MASK
            }
        }
    }

    /// Set **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn with_od(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio1::OD_MASK
            } else {
                self.bits & !Gpio1::OD_MASK
            }
        }
    }

    /// Set **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn with_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio1::IE_MASK
            } else {
                self.bits & !Gpio1::IE_MASK
            }
        }
    }

    /// Set **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn with_drive(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Gpio1::DRIVE_MASK) | ((value << Gpio1::DRIVE_OFFSET) & Gpio1::DRIVE_MASK)
        }
    }

    /// Set **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn with_pue(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio1::PUE_MASK
            } else {
                self.bits & !Gpio1::PUE_MASK
            }
        }
    }

    /// Set **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn with_pde(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio1::PDE_MASK
            } else {
                self.bits & !Gpio1::PDE_MASK
            }
        }
    }

    /// Set **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn with_schmitt(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio1::SCHMITT_MASK
            } else {
                self.bits & !Gpio1::SCHMITT_MASK
            }
        }
    }

    /// Set **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn with_slewfast(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio1::SLEWFAST_MASK
            } else {
                self.bits & !Gpio1::SLEWFAST_MASK
            }
        }
    }

}

impl Default for Gpio1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Gpio1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Gpio1Val {
        Gpio1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Gpio1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Gpio1Val) -> Gpio1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// GPIO2 Register
///
/// Address offset: 0x000c
/// Reset value: 0x00000116
///
/// ## Fields
///
/// - **ISO** `[8]` (RW)
///
///   Pad isolation control. Remove this once the pad is configured by software.
///
/// - **OD** `[7]` (RW)
///
///   Output disable. Has priority over output enable from peripherals
///
/// - **IE** `[6]` (RW)
///
///   Input enable
///
/// - **DRIVE** `[5:4]` (RW)
///
///   Drive strength.
///
/// - **PUE** `[3]` (RW)
///
///   Pull up enable
///
/// - **PDE** `[2]` (RW)
///
///   Pull down enable
///
/// - **SCHMITT** `[1]` (RW)
///
///   Enable schmitt trigger
///
/// - **SLEWFAST** `[0]` (RW)
///
///   Slew rate control. 1 = Fast, 0 = Slow
#[repr(transparent)]
pub struct Gpio2 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Gpio2 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Gpio2 {
    // ======== Field Constants ========

    /// **ISO**
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ISO_OFFSET: u32 = 8;
    pub const ISO_MASK: u32 = 0x00000100;
    pub const ISO: u32 = 1 << 8;

    /// **OD**
    ///
    /// Output disable. Has priority over output enable from peripherals
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const OD_OFFSET: u32 = 7;
    pub const OD_MASK: u32 = 0x00000080;
    pub const OD: u32 = 1 << 7;

    /// **IE**
    ///
    /// Input enable
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const IE_OFFSET: u32 = 6;
    pub const IE_MASK: u32 = 0x00000040;
    pub const IE: u32 = 1 << 6;

    /// **DRIVE**
    ///
    /// Drive strength.
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const DRIVE_OFFSET: u32 = 4;
    pub const DRIVE_MASK: u32 = 0x00000030;
    pub const DRIVE_2M_A: u32 = 0 << 4;
    pub const DRIVE_2M_A_VAL: u32 = 0;
    pub const DRIVE_4M_A: u32 = 1 << 4;
    pub const DRIVE_4M_A_VAL: u32 = 1;
    pub const DRIVE_8M_A: u32 = 2 << 4;
    pub const DRIVE_8M_A_VAL: u32 = 2;
    pub const DRIVE_12M_A: u32 = 3 << 4;
    pub const DRIVE_12M_A_VAL: u32 = 3;

    /// **PUE**
    ///
    /// Pull up enable
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const PUE_OFFSET: u32 = 3;
    pub const PUE_MASK: u32 = 0x00000008;
    pub const PUE: u32 = 1 << 3;

    /// **PDE**
    ///
    /// Pull down enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const PDE_OFFSET: u32 = 2;
    pub const PDE_MASK: u32 = 0x00000004;
    pub const PDE: u32 = 1 << 2;

    /// **SCHMITT**
    ///
    /// Enable schmitt trigger
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SCHMITT_OFFSET: u32 = 1;
    pub const SCHMITT_MASK: u32 = 0x00000002;
    pub const SCHMITT: u32 = 1 << 1;

    /// **SLEWFAST**
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SLEWFAST_OFFSET: u32 = 0;
    pub const SLEWFAST_MASK: u32 = 0x00000001;
    pub const SLEWFAST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ISO** `[8]` (RW)
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub fn iso(&self) -> bool {
        (self.value.read_bits() & Self::ISO_MASK) != 0
    }

    /// Set ISO bit atomically (RW)
    #[inline(always)]
    pub fn set_iso(&self) {
        self.set_bits(Self::ISO_MASK);
    }

    /// Clear ISO bit atomically (RW)
    #[inline(always)]
    pub fn clr_iso(&self) {
        self.clr_bits(Self::ISO_MASK);
    }

    /// **OD** `[7]` (RW)
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub fn od(&self) -> bool {
        (self.value.read_bits() & Self::OD_MASK) != 0
    }

    /// Set OD bit atomically (RW)
    #[inline(always)]
    pub fn set_od(&self) {
        self.set_bits(Self::OD_MASK);
    }

    /// Clear OD bit atomically (RW)
    #[inline(always)]
    pub fn clr_od(&self) {
        self.clr_bits(Self::OD_MASK);
    }

    /// **IE** `[6]` (RW)
    ///
    /// Input enable
    #[inline(always)]
    pub fn ie(&self) -> bool {
        (self.value.read_bits() & Self::IE_MASK) != 0
    }

    /// Set IE bit atomically (RW)
    #[inline(always)]
    pub fn set_ie(&self) {
        self.set_bits(Self::IE_MASK);
    }

    /// Clear IE bit atomically (RW)
    #[inline(always)]
    pub fn clr_ie(&self) {
        self.clr_bits(Self::IE_MASK);
    }

    /// **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn drive(&self) -> u32 {
        (self.value.read_bits() & Self::DRIVE_MASK) >> Self::DRIVE_OFFSET
    }

    /// Write **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn write_drive(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DRIVE_MASK) | ((value << Self::DRIVE_OFFSET) & Self::DRIVE_MASK));
    }

    /// **PUE** `[3]` (RW)
    ///
    /// Pull up enable
    #[inline(always)]
    pub fn pue(&self) -> bool {
        (self.value.read_bits() & Self::PUE_MASK) != 0
    }

    /// Set PUE bit atomically (RW)
    #[inline(always)]
    pub fn set_pue(&self) {
        self.set_bits(Self::PUE_MASK);
    }

    /// Clear PUE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pue(&self) {
        self.clr_bits(Self::PUE_MASK);
    }

    /// **PDE** `[2]` (RW)
    ///
    /// Pull down enable
    #[inline(always)]
    pub fn pde(&self) -> bool {
        (self.value.read_bits() & Self::PDE_MASK) != 0
    }

    /// Set PDE bit atomically (RW)
    #[inline(always)]
    pub fn set_pde(&self) {
        self.set_bits(Self::PDE_MASK);
    }

    /// Clear PDE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pde(&self) {
        self.clr_bits(Self::PDE_MASK);
    }

    /// **SCHMITT** `[1]` (RW)
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub fn schmitt(&self) -> bool {
        (self.value.read_bits() & Self::SCHMITT_MASK) != 0
    }

    /// Set SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn set_schmitt(&self) {
        self.set_bits(Self::SCHMITT_MASK);
    }

    /// Clear SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn clr_schmitt(&self) {
        self.clr_bits(Self::SCHMITT_MASK);
    }

    /// **SLEWFAST** `[0]` (RW)
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub fn slewfast(&self) -> bool {
        (self.value.read_bits() & Self::SLEWFAST_MASK) != 0
    }

    /// Set SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn set_slewfast(&self) {
        self.set_bits(Self::SLEWFAST_MASK);
    }

    /// Clear SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn clr_slewfast(&self) {
        self.clr_bits(Self::SLEWFAST_MASK);
    }

}

/// GPIO2 value type for batch read/modify/write operations
///
/// Use `gpio2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpio2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Gpio2Val {
    bits: u32,
}

impl Gpio2Val {
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
        Self { bits: 0x00000116 }
    }

    /// Read **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn iso(&self) -> bool {
        (self.bits & Gpio2::ISO_MASK) != 0
    }

    /// Read **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn od(&self) -> bool {
        (self.bits & Gpio2::OD_MASK) != 0
    }

    /// Read **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        (self.bits & Gpio2::IE_MASK) != 0
    }

    /// Read **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn drive(&self) -> u32 {
        (self.bits & Gpio2::DRIVE_MASK) >> Gpio2::DRIVE_OFFSET
    }

    /// Read **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn pue(&self) -> bool {
        (self.bits & Gpio2::PUE_MASK) != 0
    }

    /// Read **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn pde(&self) -> bool {
        (self.bits & Gpio2::PDE_MASK) != 0
    }

    /// Read **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn schmitt(&self) -> bool {
        (self.bits & Gpio2::SCHMITT_MASK) != 0
    }

    /// Read **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn slewfast(&self) -> bool {
        (self.bits & Gpio2::SLEWFAST_MASK) != 0
    }

    /// Set **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn with_iso(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio2::ISO_MASK
            } else {
                self.bits & !Gpio2::ISO_MASK
            }
        }
    }

    /// Set **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn with_od(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio2::OD_MASK
            } else {
                self.bits & !Gpio2::OD_MASK
            }
        }
    }

    /// Set **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn with_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio2::IE_MASK
            } else {
                self.bits & !Gpio2::IE_MASK
            }
        }
    }

    /// Set **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn with_drive(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Gpio2::DRIVE_MASK) | ((value << Gpio2::DRIVE_OFFSET) & Gpio2::DRIVE_MASK)
        }
    }

    /// Set **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn with_pue(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio2::PUE_MASK
            } else {
                self.bits & !Gpio2::PUE_MASK
            }
        }
    }

    /// Set **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn with_pde(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio2::PDE_MASK
            } else {
                self.bits & !Gpio2::PDE_MASK
            }
        }
    }

    /// Set **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn with_schmitt(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio2::SCHMITT_MASK
            } else {
                self.bits & !Gpio2::SCHMITT_MASK
            }
        }
    }

    /// Set **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn with_slewfast(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio2::SLEWFAST_MASK
            } else {
                self.bits & !Gpio2::SLEWFAST_MASK
            }
        }
    }

}

impl Default for Gpio2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Gpio2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Gpio2Val {
        Gpio2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Gpio2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Gpio2Val) -> Gpio2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// GPIO3 Register
///
/// Address offset: 0x0010
/// Reset value: 0x00000116
///
/// ## Fields
///
/// - **ISO** `[8]` (RW)
///
///   Pad isolation control. Remove this once the pad is configured by software.
///
/// - **OD** `[7]` (RW)
///
///   Output disable. Has priority over output enable from peripherals
///
/// - **IE** `[6]` (RW)
///
///   Input enable
///
/// - **DRIVE** `[5:4]` (RW)
///
///   Drive strength.
///
/// - **PUE** `[3]` (RW)
///
///   Pull up enable
///
/// - **PDE** `[2]` (RW)
///
///   Pull down enable
///
/// - **SCHMITT** `[1]` (RW)
///
///   Enable schmitt trigger
///
/// - **SLEWFAST** `[0]` (RW)
///
///   Slew rate control. 1 = Fast, 0 = Slow
#[repr(transparent)]
pub struct Gpio3 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Gpio3 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Gpio3 {
    // ======== Field Constants ========

    /// **ISO**
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ISO_OFFSET: u32 = 8;
    pub const ISO_MASK: u32 = 0x00000100;
    pub const ISO: u32 = 1 << 8;

    /// **OD**
    ///
    /// Output disable. Has priority over output enable from peripherals
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const OD_OFFSET: u32 = 7;
    pub const OD_MASK: u32 = 0x00000080;
    pub const OD: u32 = 1 << 7;

    /// **IE**
    ///
    /// Input enable
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const IE_OFFSET: u32 = 6;
    pub const IE_MASK: u32 = 0x00000040;
    pub const IE: u32 = 1 << 6;

    /// **DRIVE**
    ///
    /// Drive strength.
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const DRIVE_OFFSET: u32 = 4;
    pub const DRIVE_MASK: u32 = 0x00000030;
    pub const DRIVE_2M_A: u32 = 0 << 4;
    pub const DRIVE_2M_A_VAL: u32 = 0;
    pub const DRIVE_4M_A: u32 = 1 << 4;
    pub const DRIVE_4M_A_VAL: u32 = 1;
    pub const DRIVE_8M_A: u32 = 2 << 4;
    pub const DRIVE_8M_A_VAL: u32 = 2;
    pub const DRIVE_12M_A: u32 = 3 << 4;
    pub const DRIVE_12M_A_VAL: u32 = 3;

    /// **PUE**
    ///
    /// Pull up enable
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const PUE_OFFSET: u32 = 3;
    pub const PUE_MASK: u32 = 0x00000008;
    pub const PUE: u32 = 1 << 3;

    /// **PDE**
    ///
    /// Pull down enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const PDE_OFFSET: u32 = 2;
    pub const PDE_MASK: u32 = 0x00000004;
    pub const PDE: u32 = 1 << 2;

    /// **SCHMITT**
    ///
    /// Enable schmitt trigger
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SCHMITT_OFFSET: u32 = 1;
    pub const SCHMITT_MASK: u32 = 0x00000002;
    pub const SCHMITT: u32 = 1 << 1;

    /// **SLEWFAST**
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SLEWFAST_OFFSET: u32 = 0;
    pub const SLEWFAST_MASK: u32 = 0x00000001;
    pub const SLEWFAST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ISO** `[8]` (RW)
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub fn iso(&self) -> bool {
        (self.value.read_bits() & Self::ISO_MASK) != 0
    }

    /// Set ISO bit atomically (RW)
    #[inline(always)]
    pub fn set_iso(&self) {
        self.set_bits(Self::ISO_MASK);
    }

    /// Clear ISO bit atomically (RW)
    #[inline(always)]
    pub fn clr_iso(&self) {
        self.clr_bits(Self::ISO_MASK);
    }

    /// **OD** `[7]` (RW)
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub fn od(&self) -> bool {
        (self.value.read_bits() & Self::OD_MASK) != 0
    }

    /// Set OD bit atomically (RW)
    #[inline(always)]
    pub fn set_od(&self) {
        self.set_bits(Self::OD_MASK);
    }

    /// Clear OD bit atomically (RW)
    #[inline(always)]
    pub fn clr_od(&self) {
        self.clr_bits(Self::OD_MASK);
    }

    /// **IE** `[6]` (RW)
    ///
    /// Input enable
    #[inline(always)]
    pub fn ie(&self) -> bool {
        (self.value.read_bits() & Self::IE_MASK) != 0
    }

    /// Set IE bit atomically (RW)
    #[inline(always)]
    pub fn set_ie(&self) {
        self.set_bits(Self::IE_MASK);
    }

    /// Clear IE bit atomically (RW)
    #[inline(always)]
    pub fn clr_ie(&self) {
        self.clr_bits(Self::IE_MASK);
    }

    /// **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn drive(&self) -> u32 {
        (self.value.read_bits() & Self::DRIVE_MASK) >> Self::DRIVE_OFFSET
    }

    /// Write **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn write_drive(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DRIVE_MASK) | ((value << Self::DRIVE_OFFSET) & Self::DRIVE_MASK));
    }

    /// **PUE** `[3]` (RW)
    ///
    /// Pull up enable
    #[inline(always)]
    pub fn pue(&self) -> bool {
        (self.value.read_bits() & Self::PUE_MASK) != 0
    }

    /// Set PUE bit atomically (RW)
    #[inline(always)]
    pub fn set_pue(&self) {
        self.set_bits(Self::PUE_MASK);
    }

    /// Clear PUE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pue(&self) {
        self.clr_bits(Self::PUE_MASK);
    }

    /// **PDE** `[2]` (RW)
    ///
    /// Pull down enable
    #[inline(always)]
    pub fn pde(&self) -> bool {
        (self.value.read_bits() & Self::PDE_MASK) != 0
    }

    /// Set PDE bit atomically (RW)
    #[inline(always)]
    pub fn set_pde(&self) {
        self.set_bits(Self::PDE_MASK);
    }

    /// Clear PDE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pde(&self) {
        self.clr_bits(Self::PDE_MASK);
    }

    /// **SCHMITT** `[1]` (RW)
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub fn schmitt(&self) -> bool {
        (self.value.read_bits() & Self::SCHMITT_MASK) != 0
    }

    /// Set SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn set_schmitt(&self) {
        self.set_bits(Self::SCHMITT_MASK);
    }

    /// Clear SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn clr_schmitt(&self) {
        self.clr_bits(Self::SCHMITT_MASK);
    }

    /// **SLEWFAST** `[0]` (RW)
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub fn slewfast(&self) -> bool {
        (self.value.read_bits() & Self::SLEWFAST_MASK) != 0
    }

    /// Set SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn set_slewfast(&self) {
        self.set_bits(Self::SLEWFAST_MASK);
    }

    /// Clear SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn clr_slewfast(&self) {
        self.clr_bits(Self::SLEWFAST_MASK);
    }

}

/// GPIO3 value type for batch read/modify/write operations
///
/// Use `gpio3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpio3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Gpio3Val {
    bits: u32,
}

impl Gpio3Val {
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
        Self { bits: 0x00000116 }
    }

    /// Read **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn iso(&self) -> bool {
        (self.bits & Gpio3::ISO_MASK) != 0
    }

    /// Read **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn od(&self) -> bool {
        (self.bits & Gpio3::OD_MASK) != 0
    }

    /// Read **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        (self.bits & Gpio3::IE_MASK) != 0
    }

    /// Read **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn drive(&self) -> u32 {
        (self.bits & Gpio3::DRIVE_MASK) >> Gpio3::DRIVE_OFFSET
    }

    /// Read **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn pue(&self) -> bool {
        (self.bits & Gpio3::PUE_MASK) != 0
    }

    /// Read **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn pde(&self) -> bool {
        (self.bits & Gpio3::PDE_MASK) != 0
    }

    /// Read **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn schmitt(&self) -> bool {
        (self.bits & Gpio3::SCHMITT_MASK) != 0
    }

    /// Read **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn slewfast(&self) -> bool {
        (self.bits & Gpio3::SLEWFAST_MASK) != 0
    }

    /// Set **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn with_iso(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio3::ISO_MASK
            } else {
                self.bits & !Gpio3::ISO_MASK
            }
        }
    }

    /// Set **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn with_od(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio3::OD_MASK
            } else {
                self.bits & !Gpio3::OD_MASK
            }
        }
    }

    /// Set **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn with_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio3::IE_MASK
            } else {
                self.bits & !Gpio3::IE_MASK
            }
        }
    }

    /// Set **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn with_drive(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Gpio3::DRIVE_MASK) | ((value << Gpio3::DRIVE_OFFSET) & Gpio3::DRIVE_MASK)
        }
    }

    /// Set **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn with_pue(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio3::PUE_MASK
            } else {
                self.bits & !Gpio3::PUE_MASK
            }
        }
    }

    /// Set **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn with_pde(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio3::PDE_MASK
            } else {
                self.bits & !Gpio3::PDE_MASK
            }
        }
    }

    /// Set **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn with_schmitt(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio3::SCHMITT_MASK
            } else {
                self.bits & !Gpio3::SCHMITT_MASK
            }
        }
    }

    /// Set **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn with_slewfast(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio3::SLEWFAST_MASK
            } else {
                self.bits & !Gpio3::SLEWFAST_MASK
            }
        }
    }

}

impl Default for Gpio3Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Gpio3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Gpio3Val {
        Gpio3Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Gpio3Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Gpio3Val) -> Gpio3Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// GPIO4 Register
///
/// Address offset: 0x0014
/// Reset value: 0x00000116
///
/// ## Fields
///
/// - **ISO** `[8]` (RW)
///
///   Pad isolation control. Remove this once the pad is configured by software.
///
/// - **OD** `[7]` (RW)
///
///   Output disable. Has priority over output enable from peripherals
///
/// - **IE** `[6]` (RW)
///
///   Input enable
///
/// - **DRIVE** `[5:4]` (RW)
///
///   Drive strength.
///
/// - **PUE** `[3]` (RW)
///
///   Pull up enable
///
/// - **PDE** `[2]` (RW)
///
///   Pull down enable
///
/// - **SCHMITT** `[1]` (RW)
///
///   Enable schmitt trigger
///
/// - **SLEWFAST** `[0]` (RW)
///
///   Slew rate control. 1 = Fast, 0 = Slow
#[repr(transparent)]
pub struct Gpio4 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Gpio4 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Gpio4 {
    // ======== Field Constants ========

    /// **ISO**
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ISO_OFFSET: u32 = 8;
    pub const ISO_MASK: u32 = 0x00000100;
    pub const ISO: u32 = 1 << 8;

    /// **OD**
    ///
    /// Output disable. Has priority over output enable from peripherals
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const OD_OFFSET: u32 = 7;
    pub const OD_MASK: u32 = 0x00000080;
    pub const OD: u32 = 1 << 7;

    /// **IE**
    ///
    /// Input enable
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const IE_OFFSET: u32 = 6;
    pub const IE_MASK: u32 = 0x00000040;
    pub const IE: u32 = 1 << 6;

    /// **DRIVE**
    ///
    /// Drive strength.
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const DRIVE_OFFSET: u32 = 4;
    pub const DRIVE_MASK: u32 = 0x00000030;
    pub const DRIVE_2M_A: u32 = 0 << 4;
    pub const DRIVE_2M_A_VAL: u32 = 0;
    pub const DRIVE_4M_A: u32 = 1 << 4;
    pub const DRIVE_4M_A_VAL: u32 = 1;
    pub const DRIVE_8M_A: u32 = 2 << 4;
    pub const DRIVE_8M_A_VAL: u32 = 2;
    pub const DRIVE_12M_A: u32 = 3 << 4;
    pub const DRIVE_12M_A_VAL: u32 = 3;

    /// **PUE**
    ///
    /// Pull up enable
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const PUE_OFFSET: u32 = 3;
    pub const PUE_MASK: u32 = 0x00000008;
    pub const PUE: u32 = 1 << 3;

    /// **PDE**
    ///
    /// Pull down enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const PDE_OFFSET: u32 = 2;
    pub const PDE_MASK: u32 = 0x00000004;
    pub const PDE: u32 = 1 << 2;

    /// **SCHMITT**
    ///
    /// Enable schmitt trigger
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SCHMITT_OFFSET: u32 = 1;
    pub const SCHMITT_MASK: u32 = 0x00000002;
    pub const SCHMITT: u32 = 1 << 1;

    /// **SLEWFAST**
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SLEWFAST_OFFSET: u32 = 0;
    pub const SLEWFAST_MASK: u32 = 0x00000001;
    pub const SLEWFAST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ISO** `[8]` (RW)
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub fn iso(&self) -> bool {
        (self.value.read_bits() & Self::ISO_MASK) != 0
    }

    /// Set ISO bit atomically (RW)
    #[inline(always)]
    pub fn set_iso(&self) {
        self.set_bits(Self::ISO_MASK);
    }

    /// Clear ISO bit atomically (RW)
    #[inline(always)]
    pub fn clr_iso(&self) {
        self.clr_bits(Self::ISO_MASK);
    }

    /// **OD** `[7]` (RW)
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub fn od(&self) -> bool {
        (self.value.read_bits() & Self::OD_MASK) != 0
    }

    /// Set OD bit atomically (RW)
    #[inline(always)]
    pub fn set_od(&self) {
        self.set_bits(Self::OD_MASK);
    }

    /// Clear OD bit atomically (RW)
    #[inline(always)]
    pub fn clr_od(&self) {
        self.clr_bits(Self::OD_MASK);
    }

    /// **IE** `[6]` (RW)
    ///
    /// Input enable
    #[inline(always)]
    pub fn ie(&self) -> bool {
        (self.value.read_bits() & Self::IE_MASK) != 0
    }

    /// Set IE bit atomically (RW)
    #[inline(always)]
    pub fn set_ie(&self) {
        self.set_bits(Self::IE_MASK);
    }

    /// Clear IE bit atomically (RW)
    #[inline(always)]
    pub fn clr_ie(&self) {
        self.clr_bits(Self::IE_MASK);
    }

    /// **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn drive(&self) -> u32 {
        (self.value.read_bits() & Self::DRIVE_MASK) >> Self::DRIVE_OFFSET
    }

    /// Write **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn write_drive(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DRIVE_MASK) | ((value << Self::DRIVE_OFFSET) & Self::DRIVE_MASK));
    }

    /// **PUE** `[3]` (RW)
    ///
    /// Pull up enable
    #[inline(always)]
    pub fn pue(&self) -> bool {
        (self.value.read_bits() & Self::PUE_MASK) != 0
    }

    /// Set PUE bit atomically (RW)
    #[inline(always)]
    pub fn set_pue(&self) {
        self.set_bits(Self::PUE_MASK);
    }

    /// Clear PUE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pue(&self) {
        self.clr_bits(Self::PUE_MASK);
    }

    /// **PDE** `[2]` (RW)
    ///
    /// Pull down enable
    #[inline(always)]
    pub fn pde(&self) -> bool {
        (self.value.read_bits() & Self::PDE_MASK) != 0
    }

    /// Set PDE bit atomically (RW)
    #[inline(always)]
    pub fn set_pde(&self) {
        self.set_bits(Self::PDE_MASK);
    }

    /// Clear PDE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pde(&self) {
        self.clr_bits(Self::PDE_MASK);
    }

    /// **SCHMITT** `[1]` (RW)
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub fn schmitt(&self) -> bool {
        (self.value.read_bits() & Self::SCHMITT_MASK) != 0
    }

    /// Set SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn set_schmitt(&self) {
        self.set_bits(Self::SCHMITT_MASK);
    }

    /// Clear SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn clr_schmitt(&self) {
        self.clr_bits(Self::SCHMITT_MASK);
    }

    /// **SLEWFAST** `[0]` (RW)
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub fn slewfast(&self) -> bool {
        (self.value.read_bits() & Self::SLEWFAST_MASK) != 0
    }

    /// Set SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn set_slewfast(&self) {
        self.set_bits(Self::SLEWFAST_MASK);
    }

    /// Clear SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn clr_slewfast(&self) {
        self.clr_bits(Self::SLEWFAST_MASK);
    }

}

/// GPIO4 value type for batch read/modify/write operations
///
/// Use `gpio4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpio4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Gpio4Val {
    bits: u32,
}

impl Gpio4Val {
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
        Self { bits: 0x00000116 }
    }

    /// Read **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn iso(&self) -> bool {
        (self.bits & Gpio4::ISO_MASK) != 0
    }

    /// Read **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn od(&self) -> bool {
        (self.bits & Gpio4::OD_MASK) != 0
    }

    /// Read **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        (self.bits & Gpio4::IE_MASK) != 0
    }

    /// Read **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn drive(&self) -> u32 {
        (self.bits & Gpio4::DRIVE_MASK) >> Gpio4::DRIVE_OFFSET
    }

    /// Read **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn pue(&self) -> bool {
        (self.bits & Gpio4::PUE_MASK) != 0
    }

    /// Read **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn pde(&self) -> bool {
        (self.bits & Gpio4::PDE_MASK) != 0
    }

    /// Read **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn schmitt(&self) -> bool {
        (self.bits & Gpio4::SCHMITT_MASK) != 0
    }

    /// Read **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn slewfast(&self) -> bool {
        (self.bits & Gpio4::SLEWFAST_MASK) != 0
    }

    /// Set **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn with_iso(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio4::ISO_MASK
            } else {
                self.bits & !Gpio4::ISO_MASK
            }
        }
    }

    /// Set **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn with_od(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio4::OD_MASK
            } else {
                self.bits & !Gpio4::OD_MASK
            }
        }
    }

    /// Set **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn with_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio4::IE_MASK
            } else {
                self.bits & !Gpio4::IE_MASK
            }
        }
    }

    /// Set **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn with_drive(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Gpio4::DRIVE_MASK) | ((value << Gpio4::DRIVE_OFFSET) & Gpio4::DRIVE_MASK)
        }
    }

    /// Set **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn with_pue(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio4::PUE_MASK
            } else {
                self.bits & !Gpio4::PUE_MASK
            }
        }
    }

    /// Set **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn with_pde(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio4::PDE_MASK
            } else {
                self.bits & !Gpio4::PDE_MASK
            }
        }
    }

    /// Set **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn with_schmitt(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio4::SCHMITT_MASK
            } else {
                self.bits & !Gpio4::SCHMITT_MASK
            }
        }
    }

    /// Set **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn with_slewfast(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio4::SLEWFAST_MASK
            } else {
                self.bits & !Gpio4::SLEWFAST_MASK
            }
        }
    }

}

impl Default for Gpio4Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Gpio4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Gpio4Val {
        Gpio4Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Gpio4Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Gpio4Val) -> Gpio4Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// GPIO5 Register
///
/// Address offset: 0x0018
/// Reset value: 0x00000116
///
/// ## Fields
///
/// - **ISO** `[8]` (RW)
///
///   Pad isolation control. Remove this once the pad is configured by software.
///
/// - **OD** `[7]` (RW)
///
///   Output disable. Has priority over output enable from peripherals
///
/// - **IE** `[6]` (RW)
///
///   Input enable
///
/// - **DRIVE** `[5:4]` (RW)
///
///   Drive strength.
///
/// - **PUE** `[3]` (RW)
///
///   Pull up enable
///
/// - **PDE** `[2]` (RW)
///
///   Pull down enable
///
/// - **SCHMITT** `[1]` (RW)
///
///   Enable schmitt trigger
///
/// - **SLEWFAST** `[0]` (RW)
///
///   Slew rate control. 1 = Fast, 0 = Slow
#[repr(transparent)]
pub struct Gpio5 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Gpio5 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Gpio5 {
    // ======== Field Constants ========

    /// **ISO**
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ISO_OFFSET: u32 = 8;
    pub const ISO_MASK: u32 = 0x00000100;
    pub const ISO: u32 = 1 << 8;

    /// **OD**
    ///
    /// Output disable. Has priority over output enable from peripherals
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const OD_OFFSET: u32 = 7;
    pub const OD_MASK: u32 = 0x00000080;
    pub const OD: u32 = 1 << 7;

    /// **IE**
    ///
    /// Input enable
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const IE_OFFSET: u32 = 6;
    pub const IE_MASK: u32 = 0x00000040;
    pub const IE: u32 = 1 << 6;

    /// **DRIVE**
    ///
    /// Drive strength.
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const DRIVE_OFFSET: u32 = 4;
    pub const DRIVE_MASK: u32 = 0x00000030;
    pub const DRIVE_2M_A: u32 = 0 << 4;
    pub const DRIVE_2M_A_VAL: u32 = 0;
    pub const DRIVE_4M_A: u32 = 1 << 4;
    pub const DRIVE_4M_A_VAL: u32 = 1;
    pub const DRIVE_8M_A: u32 = 2 << 4;
    pub const DRIVE_8M_A_VAL: u32 = 2;
    pub const DRIVE_12M_A: u32 = 3 << 4;
    pub const DRIVE_12M_A_VAL: u32 = 3;

    /// **PUE**
    ///
    /// Pull up enable
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const PUE_OFFSET: u32 = 3;
    pub const PUE_MASK: u32 = 0x00000008;
    pub const PUE: u32 = 1 << 3;

    /// **PDE**
    ///
    /// Pull down enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const PDE_OFFSET: u32 = 2;
    pub const PDE_MASK: u32 = 0x00000004;
    pub const PDE: u32 = 1 << 2;

    /// **SCHMITT**
    ///
    /// Enable schmitt trigger
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SCHMITT_OFFSET: u32 = 1;
    pub const SCHMITT_MASK: u32 = 0x00000002;
    pub const SCHMITT: u32 = 1 << 1;

    /// **SLEWFAST**
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SLEWFAST_OFFSET: u32 = 0;
    pub const SLEWFAST_MASK: u32 = 0x00000001;
    pub const SLEWFAST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ISO** `[8]` (RW)
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub fn iso(&self) -> bool {
        (self.value.read_bits() & Self::ISO_MASK) != 0
    }

    /// Set ISO bit atomically (RW)
    #[inline(always)]
    pub fn set_iso(&self) {
        self.set_bits(Self::ISO_MASK);
    }

    /// Clear ISO bit atomically (RW)
    #[inline(always)]
    pub fn clr_iso(&self) {
        self.clr_bits(Self::ISO_MASK);
    }

    /// **OD** `[7]` (RW)
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub fn od(&self) -> bool {
        (self.value.read_bits() & Self::OD_MASK) != 0
    }

    /// Set OD bit atomically (RW)
    #[inline(always)]
    pub fn set_od(&self) {
        self.set_bits(Self::OD_MASK);
    }

    /// Clear OD bit atomically (RW)
    #[inline(always)]
    pub fn clr_od(&self) {
        self.clr_bits(Self::OD_MASK);
    }

    /// **IE** `[6]` (RW)
    ///
    /// Input enable
    #[inline(always)]
    pub fn ie(&self) -> bool {
        (self.value.read_bits() & Self::IE_MASK) != 0
    }

    /// Set IE bit atomically (RW)
    #[inline(always)]
    pub fn set_ie(&self) {
        self.set_bits(Self::IE_MASK);
    }

    /// Clear IE bit atomically (RW)
    #[inline(always)]
    pub fn clr_ie(&self) {
        self.clr_bits(Self::IE_MASK);
    }

    /// **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn drive(&self) -> u32 {
        (self.value.read_bits() & Self::DRIVE_MASK) >> Self::DRIVE_OFFSET
    }

    /// Write **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn write_drive(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DRIVE_MASK) | ((value << Self::DRIVE_OFFSET) & Self::DRIVE_MASK));
    }

    /// **PUE** `[3]` (RW)
    ///
    /// Pull up enable
    #[inline(always)]
    pub fn pue(&self) -> bool {
        (self.value.read_bits() & Self::PUE_MASK) != 0
    }

    /// Set PUE bit atomically (RW)
    #[inline(always)]
    pub fn set_pue(&self) {
        self.set_bits(Self::PUE_MASK);
    }

    /// Clear PUE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pue(&self) {
        self.clr_bits(Self::PUE_MASK);
    }

    /// **PDE** `[2]` (RW)
    ///
    /// Pull down enable
    #[inline(always)]
    pub fn pde(&self) -> bool {
        (self.value.read_bits() & Self::PDE_MASK) != 0
    }

    /// Set PDE bit atomically (RW)
    #[inline(always)]
    pub fn set_pde(&self) {
        self.set_bits(Self::PDE_MASK);
    }

    /// Clear PDE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pde(&self) {
        self.clr_bits(Self::PDE_MASK);
    }

    /// **SCHMITT** `[1]` (RW)
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub fn schmitt(&self) -> bool {
        (self.value.read_bits() & Self::SCHMITT_MASK) != 0
    }

    /// Set SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn set_schmitt(&self) {
        self.set_bits(Self::SCHMITT_MASK);
    }

    /// Clear SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn clr_schmitt(&self) {
        self.clr_bits(Self::SCHMITT_MASK);
    }

    /// **SLEWFAST** `[0]` (RW)
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub fn slewfast(&self) -> bool {
        (self.value.read_bits() & Self::SLEWFAST_MASK) != 0
    }

    /// Set SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn set_slewfast(&self) {
        self.set_bits(Self::SLEWFAST_MASK);
    }

    /// Clear SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn clr_slewfast(&self) {
        self.clr_bits(Self::SLEWFAST_MASK);
    }

}

/// GPIO5 value type for batch read/modify/write operations
///
/// Use `gpio5.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpio5.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Gpio5Val {
    bits: u32,
}

impl Gpio5Val {
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
        Self { bits: 0x00000116 }
    }

    /// Read **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn iso(&self) -> bool {
        (self.bits & Gpio5::ISO_MASK) != 0
    }

    /// Read **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn od(&self) -> bool {
        (self.bits & Gpio5::OD_MASK) != 0
    }

    /// Read **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        (self.bits & Gpio5::IE_MASK) != 0
    }

    /// Read **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn drive(&self) -> u32 {
        (self.bits & Gpio5::DRIVE_MASK) >> Gpio5::DRIVE_OFFSET
    }

    /// Read **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn pue(&self) -> bool {
        (self.bits & Gpio5::PUE_MASK) != 0
    }

    /// Read **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn pde(&self) -> bool {
        (self.bits & Gpio5::PDE_MASK) != 0
    }

    /// Read **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn schmitt(&self) -> bool {
        (self.bits & Gpio5::SCHMITT_MASK) != 0
    }

    /// Read **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn slewfast(&self) -> bool {
        (self.bits & Gpio5::SLEWFAST_MASK) != 0
    }

    /// Set **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn with_iso(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio5::ISO_MASK
            } else {
                self.bits & !Gpio5::ISO_MASK
            }
        }
    }

    /// Set **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn with_od(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio5::OD_MASK
            } else {
                self.bits & !Gpio5::OD_MASK
            }
        }
    }

    /// Set **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn with_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio5::IE_MASK
            } else {
                self.bits & !Gpio5::IE_MASK
            }
        }
    }

    /// Set **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn with_drive(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Gpio5::DRIVE_MASK) | ((value << Gpio5::DRIVE_OFFSET) & Gpio5::DRIVE_MASK)
        }
    }

    /// Set **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn with_pue(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio5::PUE_MASK
            } else {
                self.bits & !Gpio5::PUE_MASK
            }
        }
    }

    /// Set **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn with_pde(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio5::PDE_MASK
            } else {
                self.bits & !Gpio5::PDE_MASK
            }
        }
    }

    /// Set **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn with_schmitt(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio5::SCHMITT_MASK
            } else {
                self.bits & !Gpio5::SCHMITT_MASK
            }
        }
    }

    /// Set **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn with_slewfast(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio5::SLEWFAST_MASK
            } else {
                self.bits & !Gpio5::SLEWFAST_MASK
            }
        }
    }

}

impl Default for Gpio5Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Gpio5 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Gpio5Val {
        Gpio5Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Gpio5Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Gpio5Val) -> Gpio5Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// GPIO6 Register
///
/// Address offset: 0x001c
/// Reset value: 0x00000116
///
/// ## Fields
///
/// - **ISO** `[8]` (RW)
///
///   Pad isolation control. Remove this once the pad is configured by software.
///
/// - **OD** `[7]` (RW)
///
///   Output disable. Has priority over output enable from peripherals
///
/// - **IE** `[6]` (RW)
///
///   Input enable
///
/// - **DRIVE** `[5:4]` (RW)
///
///   Drive strength.
///
/// - **PUE** `[3]` (RW)
///
///   Pull up enable
///
/// - **PDE** `[2]` (RW)
///
///   Pull down enable
///
/// - **SCHMITT** `[1]` (RW)
///
///   Enable schmitt trigger
///
/// - **SLEWFAST** `[0]` (RW)
///
///   Slew rate control. 1 = Fast, 0 = Slow
#[repr(transparent)]
pub struct Gpio6 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Gpio6 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Gpio6 {
    // ======== Field Constants ========

    /// **ISO**
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ISO_OFFSET: u32 = 8;
    pub const ISO_MASK: u32 = 0x00000100;
    pub const ISO: u32 = 1 << 8;

    /// **OD**
    ///
    /// Output disable. Has priority over output enable from peripherals
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const OD_OFFSET: u32 = 7;
    pub const OD_MASK: u32 = 0x00000080;
    pub const OD: u32 = 1 << 7;

    /// **IE**
    ///
    /// Input enable
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const IE_OFFSET: u32 = 6;
    pub const IE_MASK: u32 = 0x00000040;
    pub const IE: u32 = 1 << 6;

    /// **DRIVE**
    ///
    /// Drive strength.
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const DRIVE_OFFSET: u32 = 4;
    pub const DRIVE_MASK: u32 = 0x00000030;
    pub const DRIVE_2M_A: u32 = 0 << 4;
    pub const DRIVE_2M_A_VAL: u32 = 0;
    pub const DRIVE_4M_A: u32 = 1 << 4;
    pub const DRIVE_4M_A_VAL: u32 = 1;
    pub const DRIVE_8M_A: u32 = 2 << 4;
    pub const DRIVE_8M_A_VAL: u32 = 2;
    pub const DRIVE_12M_A: u32 = 3 << 4;
    pub const DRIVE_12M_A_VAL: u32 = 3;

    /// **PUE**
    ///
    /// Pull up enable
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const PUE_OFFSET: u32 = 3;
    pub const PUE_MASK: u32 = 0x00000008;
    pub const PUE: u32 = 1 << 3;

    /// **PDE**
    ///
    /// Pull down enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const PDE_OFFSET: u32 = 2;
    pub const PDE_MASK: u32 = 0x00000004;
    pub const PDE: u32 = 1 << 2;

    /// **SCHMITT**
    ///
    /// Enable schmitt trigger
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SCHMITT_OFFSET: u32 = 1;
    pub const SCHMITT_MASK: u32 = 0x00000002;
    pub const SCHMITT: u32 = 1 << 1;

    /// **SLEWFAST**
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SLEWFAST_OFFSET: u32 = 0;
    pub const SLEWFAST_MASK: u32 = 0x00000001;
    pub const SLEWFAST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ISO** `[8]` (RW)
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub fn iso(&self) -> bool {
        (self.value.read_bits() & Self::ISO_MASK) != 0
    }

    /// Set ISO bit atomically (RW)
    #[inline(always)]
    pub fn set_iso(&self) {
        self.set_bits(Self::ISO_MASK);
    }

    /// Clear ISO bit atomically (RW)
    #[inline(always)]
    pub fn clr_iso(&self) {
        self.clr_bits(Self::ISO_MASK);
    }

    /// **OD** `[7]` (RW)
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub fn od(&self) -> bool {
        (self.value.read_bits() & Self::OD_MASK) != 0
    }

    /// Set OD bit atomically (RW)
    #[inline(always)]
    pub fn set_od(&self) {
        self.set_bits(Self::OD_MASK);
    }

    /// Clear OD bit atomically (RW)
    #[inline(always)]
    pub fn clr_od(&self) {
        self.clr_bits(Self::OD_MASK);
    }

    /// **IE** `[6]` (RW)
    ///
    /// Input enable
    #[inline(always)]
    pub fn ie(&self) -> bool {
        (self.value.read_bits() & Self::IE_MASK) != 0
    }

    /// Set IE bit atomically (RW)
    #[inline(always)]
    pub fn set_ie(&self) {
        self.set_bits(Self::IE_MASK);
    }

    /// Clear IE bit atomically (RW)
    #[inline(always)]
    pub fn clr_ie(&self) {
        self.clr_bits(Self::IE_MASK);
    }

    /// **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn drive(&self) -> u32 {
        (self.value.read_bits() & Self::DRIVE_MASK) >> Self::DRIVE_OFFSET
    }

    /// Write **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn write_drive(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DRIVE_MASK) | ((value << Self::DRIVE_OFFSET) & Self::DRIVE_MASK));
    }

    /// **PUE** `[3]` (RW)
    ///
    /// Pull up enable
    #[inline(always)]
    pub fn pue(&self) -> bool {
        (self.value.read_bits() & Self::PUE_MASK) != 0
    }

    /// Set PUE bit atomically (RW)
    #[inline(always)]
    pub fn set_pue(&self) {
        self.set_bits(Self::PUE_MASK);
    }

    /// Clear PUE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pue(&self) {
        self.clr_bits(Self::PUE_MASK);
    }

    /// **PDE** `[2]` (RW)
    ///
    /// Pull down enable
    #[inline(always)]
    pub fn pde(&self) -> bool {
        (self.value.read_bits() & Self::PDE_MASK) != 0
    }

    /// Set PDE bit atomically (RW)
    #[inline(always)]
    pub fn set_pde(&self) {
        self.set_bits(Self::PDE_MASK);
    }

    /// Clear PDE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pde(&self) {
        self.clr_bits(Self::PDE_MASK);
    }

    /// **SCHMITT** `[1]` (RW)
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub fn schmitt(&self) -> bool {
        (self.value.read_bits() & Self::SCHMITT_MASK) != 0
    }

    /// Set SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn set_schmitt(&self) {
        self.set_bits(Self::SCHMITT_MASK);
    }

    /// Clear SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn clr_schmitt(&self) {
        self.clr_bits(Self::SCHMITT_MASK);
    }

    /// **SLEWFAST** `[0]` (RW)
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub fn slewfast(&self) -> bool {
        (self.value.read_bits() & Self::SLEWFAST_MASK) != 0
    }

    /// Set SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn set_slewfast(&self) {
        self.set_bits(Self::SLEWFAST_MASK);
    }

    /// Clear SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn clr_slewfast(&self) {
        self.clr_bits(Self::SLEWFAST_MASK);
    }

}

/// GPIO6 value type for batch read/modify/write operations
///
/// Use `gpio6.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpio6.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Gpio6Val {
    bits: u32,
}

impl Gpio6Val {
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
        Self { bits: 0x00000116 }
    }

    /// Read **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn iso(&self) -> bool {
        (self.bits & Gpio6::ISO_MASK) != 0
    }

    /// Read **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn od(&self) -> bool {
        (self.bits & Gpio6::OD_MASK) != 0
    }

    /// Read **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        (self.bits & Gpio6::IE_MASK) != 0
    }

    /// Read **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn drive(&self) -> u32 {
        (self.bits & Gpio6::DRIVE_MASK) >> Gpio6::DRIVE_OFFSET
    }

    /// Read **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn pue(&self) -> bool {
        (self.bits & Gpio6::PUE_MASK) != 0
    }

    /// Read **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn pde(&self) -> bool {
        (self.bits & Gpio6::PDE_MASK) != 0
    }

    /// Read **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn schmitt(&self) -> bool {
        (self.bits & Gpio6::SCHMITT_MASK) != 0
    }

    /// Read **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn slewfast(&self) -> bool {
        (self.bits & Gpio6::SLEWFAST_MASK) != 0
    }

    /// Set **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn with_iso(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio6::ISO_MASK
            } else {
                self.bits & !Gpio6::ISO_MASK
            }
        }
    }

    /// Set **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn with_od(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio6::OD_MASK
            } else {
                self.bits & !Gpio6::OD_MASK
            }
        }
    }

    /// Set **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn with_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio6::IE_MASK
            } else {
                self.bits & !Gpio6::IE_MASK
            }
        }
    }

    /// Set **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn with_drive(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Gpio6::DRIVE_MASK) | ((value << Gpio6::DRIVE_OFFSET) & Gpio6::DRIVE_MASK)
        }
    }

    /// Set **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn with_pue(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio6::PUE_MASK
            } else {
                self.bits & !Gpio6::PUE_MASK
            }
        }
    }

    /// Set **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn with_pde(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio6::PDE_MASK
            } else {
                self.bits & !Gpio6::PDE_MASK
            }
        }
    }

    /// Set **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn with_schmitt(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio6::SCHMITT_MASK
            } else {
                self.bits & !Gpio6::SCHMITT_MASK
            }
        }
    }

    /// Set **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn with_slewfast(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio6::SLEWFAST_MASK
            } else {
                self.bits & !Gpio6::SLEWFAST_MASK
            }
        }
    }

}

impl Default for Gpio6Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Gpio6 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Gpio6Val {
        Gpio6Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Gpio6Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Gpio6Val) -> Gpio6Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// GPIO7 Register
///
/// Address offset: 0x0020
/// Reset value: 0x00000116
///
/// ## Fields
///
/// - **ISO** `[8]` (RW)
///
///   Pad isolation control. Remove this once the pad is configured by software.
///
/// - **OD** `[7]` (RW)
///
///   Output disable. Has priority over output enable from peripherals
///
/// - **IE** `[6]` (RW)
///
///   Input enable
///
/// - **DRIVE** `[5:4]` (RW)
///
///   Drive strength.
///
/// - **PUE** `[3]` (RW)
///
///   Pull up enable
///
/// - **PDE** `[2]` (RW)
///
///   Pull down enable
///
/// - **SCHMITT** `[1]` (RW)
///
///   Enable schmitt trigger
///
/// - **SLEWFAST** `[0]` (RW)
///
///   Slew rate control. 1 = Fast, 0 = Slow
#[repr(transparent)]
pub struct Gpio7 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Gpio7 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Gpio7 {
    // ======== Field Constants ========

    /// **ISO**
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ISO_OFFSET: u32 = 8;
    pub const ISO_MASK: u32 = 0x00000100;
    pub const ISO: u32 = 1 << 8;

    /// **OD**
    ///
    /// Output disable. Has priority over output enable from peripherals
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const OD_OFFSET: u32 = 7;
    pub const OD_MASK: u32 = 0x00000080;
    pub const OD: u32 = 1 << 7;

    /// **IE**
    ///
    /// Input enable
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const IE_OFFSET: u32 = 6;
    pub const IE_MASK: u32 = 0x00000040;
    pub const IE: u32 = 1 << 6;

    /// **DRIVE**
    ///
    /// Drive strength.
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const DRIVE_OFFSET: u32 = 4;
    pub const DRIVE_MASK: u32 = 0x00000030;
    pub const DRIVE_2M_A: u32 = 0 << 4;
    pub const DRIVE_2M_A_VAL: u32 = 0;
    pub const DRIVE_4M_A: u32 = 1 << 4;
    pub const DRIVE_4M_A_VAL: u32 = 1;
    pub const DRIVE_8M_A: u32 = 2 << 4;
    pub const DRIVE_8M_A_VAL: u32 = 2;
    pub const DRIVE_12M_A: u32 = 3 << 4;
    pub const DRIVE_12M_A_VAL: u32 = 3;

    /// **PUE**
    ///
    /// Pull up enable
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const PUE_OFFSET: u32 = 3;
    pub const PUE_MASK: u32 = 0x00000008;
    pub const PUE: u32 = 1 << 3;

    /// **PDE**
    ///
    /// Pull down enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const PDE_OFFSET: u32 = 2;
    pub const PDE_MASK: u32 = 0x00000004;
    pub const PDE: u32 = 1 << 2;

    /// **SCHMITT**
    ///
    /// Enable schmitt trigger
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SCHMITT_OFFSET: u32 = 1;
    pub const SCHMITT_MASK: u32 = 0x00000002;
    pub const SCHMITT: u32 = 1 << 1;

    /// **SLEWFAST**
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SLEWFAST_OFFSET: u32 = 0;
    pub const SLEWFAST_MASK: u32 = 0x00000001;
    pub const SLEWFAST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ISO** `[8]` (RW)
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub fn iso(&self) -> bool {
        (self.value.read_bits() & Self::ISO_MASK) != 0
    }

    /// Set ISO bit atomically (RW)
    #[inline(always)]
    pub fn set_iso(&self) {
        self.set_bits(Self::ISO_MASK);
    }

    /// Clear ISO bit atomically (RW)
    #[inline(always)]
    pub fn clr_iso(&self) {
        self.clr_bits(Self::ISO_MASK);
    }

    /// **OD** `[7]` (RW)
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub fn od(&self) -> bool {
        (self.value.read_bits() & Self::OD_MASK) != 0
    }

    /// Set OD bit atomically (RW)
    #[inline(always)]
    pub fn set_od(&self) {
        self.set_bits(Self::OD_MASK);
    }

    /// Clear OD bit atomically (RW)
    #[inline(always)]
    pub fn clr_od(&self) {
        self.clr_bits(Self::OD_MASK);
    }

    /// **IE** `[6]` (RW)
    ///
    /// Input enable
    #[inline(always)]
    pub fn ie(&self) -> bool {
        (self.value.read_bits() & Self::IE_MASK) != 0
    }

    /// Set IE bit atomically (RW)
    #[inline(always)]
    pub fn set_ie(&self) {
        self.set_bits(Self::IE_MASK);
    }

    /// Clear IE bit atomically (RW)
    #[inline(always)]
    pub fn clr_ie(&self) {
        self.clr_bits(Self::IE_MASK);
    }

    /// **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn drive(&self) -> u32 {
        (self.value.read_bits() & Self::DRIVE_MASK) >> Self::DRIVE_OFFSET
    }

    /// Write **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn write_drive(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DRIVE_MASK) | ((value << Self::DRIVE_OFFSET) & Self::DRIVE_MASK));
    }

    /// **PUE** `[3]` (RW)
    ///
    /// Pull up enable
    #[inline(always)]
    pub fn pue(&self) -> bool {
        (self.value.read_bits() & Self::PUE_MASK) != 0
    }

    /// Set PUE bit atomically (RW)
    #[inline(always)]
    pub fn set_pue(&self) {
        self.set_bits(Self::PUE_MASK);
    }

    /// Clear PUE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pue(&self) {
        self.clr_bits(Self::PUE_MASK);
    }

    /// **PDE** `[2]` (RW)
    ///
    /// Pull down enable
    #[inline(always)]
    pub fn pde(&self) -> bool {
        (self.value.read_bits() & Self::PDE_MASK) != 0
    }

    /// Set PDE bit atomically (RW)
    #[inline(always)]
    pub fn set_pde(&self) {
        self.set_bits(Self::PDE_MASK);
    }

    /// Clear PDE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pde(&self) {
        self.clr_bits(Self::PDE_MASK);
    }

    /// **SCHMITT** `[1]` (RW)
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub fn schmitt(&self) -> bool {
        (self.value.read_bits() & Self::SCHMITT_MASK) != 0
    }

    /// Set SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn set_schmitt(&self) {
        self.set_bits(Self::SCHMITT_MASK);
    }

    /// Clear SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn clr_schmitt(&self) {
        self.clr_bits(Self::SCHMITT_MASK);
    }

    /// **SLEWFAST** `[0]` (RW)
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub fn slewfast(&self) -> bool {
        (self.value.read_bits() & Self::SLEWFAST_MASK) != 0
    }

    /// Set SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn set_slewfast(&self) {
        self.set_bits(Self::SLEWFAST_MASK);
    }

    /// Clear SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn clr_slewfast(&self) {
        self.clr_bits(Self::SLEWFAST_MASK);
    }

}

/// GPIO7 value type for batch read/modify/write operations
///
/// Use `gpio7.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpio7.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Gpio7Val {
    bits: u32,
}

impl Gpio7Val {
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
        Self { bits: 0x00000116 }
    }

    /// Read **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn iso(&self) -> bool {
        (self.bits & Gpio7::ISO_MASK) != 0
    }

    /// Read **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn od(&self) -> bool {
        (self.bits & Gpio7::OD_MASK) != 0
    }

    /// Read **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        (self.bits & Gpio7::IE_MASK) != 0
    }

    /// Read **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn drive(&self) -> u32 {
        (self.bits & Gpio7::DRIVE_MASK) >> Gpio7::DRIVE_OFFSET
    }

    /// Read **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn pue(&self) -> bool {
        (self.bits & Gpio7::PUE_MASK) != 0
    }

    /// Read **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn pde(&self) -> bool {
        (self.bits & Gpio7::PDE_MASK) != 0
    }

    /// Read **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn schmitt(&self) -> bool {
        (self.bits & Gpio7::SCHMITT_MASK) != 0
    }

    /// Read **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn slewfast(&self) -> bool {
        (self.bits & Gpio7::SLEWFAST_MASK) != 0
    }

    /// Set **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn with_iso(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio7::ISO_MASK
            } else {
                self.bits & !Gpio7::ISO_MASK
            }
        }
    }

    /// Set **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn with_od(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio7::OD_MASK
            } else {
                self.bits & !Gpio7::OD_MASK
            }
        }
    }

    /// Set **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn with_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio7::IE_MASK
            } else {
                self.bits & !Gpio7::IE_MASK
            }
        }
    }

    /// Set **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn with_drive(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Gpio7::DRIVE_MASK) | ((value << Gpio7::DRIVE_OFFSET) & Gpio7::DRIVE_MASK)
        }
    }

    /// Set **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn with_pue(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio7::PUE_MASK
            } else {
                self.bits & !Gpio7::PUE_MASK
            }
        }
    }

    /// Set **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn with_pde(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio7::PDE_MASK
            } else {
                self.bits & !Gpio7::PDE_MASK
            }
        }
    }

    /// Set **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn with_schmitt(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio7::SCHMITT_MASK
            } else {
                self.bits & !Gpio7::SCHMITT_MASK
            }
        }
    }

    /// Set **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn with_slewfast(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio7::SLEWFAST_MASK
            } else {
                self.bits & !Gpio7::SLEWFAST_MASK
            }
        }
    }

}

impl Default for Gpio7Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Gpio7 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Gpio7Val {
        Gpio7Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Gpio7Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Gpio7Val) -> Gpio7Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// GPIO8 Register
///
/// Address offset: 0x0024
/// Reset value: 0x00000116
///
/// ## Fields
///
/// - **ISO** `[8]` (RW)
///
///   Pad isolation control. Remove this once the pad is configured by software.
///
/// - **OD** `[7]` (RW)
///
///   Output disable. Has priority over output enable from peripherals
///
/// - **IE** `[6]` (RW)
///
///   Input enable
///
/// - **DRIVE** `[5:4]` (RW)
///
///   Drive strength.
///
/// - **PUE** `[3]` (RW)
///
///   Pull up enable
///
/// - **PDE** `[2]` (RW)
///
///   Pull down enable
///
/// - **SCHMITT** `[1]` (RW)
///
///   Enable schmitt trigger
///
/// - **SLEWFAST** `[0]` (RW)
///
///   Slew rate control. 1 = Fast, 0 = Slow
#[repr(transparent)]
pub struct Gpio8 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Gpio8 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Gpio8 {
    // ======== Field Constants ========

    /// **ISO**
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ISO_OFFSET: u32 = 8;
    pub const ISO_MASK: u32 = 0x00000100;
    pub const ISO: u32 = 1 << 8;

    /// **OD**
    ///
    /// Output disable. Has priority over output enable from peripherals
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const OD_OFFSET: u32 = 7;
    pub const OD_MASK: u32 = 0x00000080;
    pub const OD: u32 = 1 << 7;

    /// **IE**
    ///
    /// Input enable
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const IE_OFFSET: u32 = 6;
    pub const IE_MASK: u32 = 0x00000040;
    pub const IE: u32 = 1 << 6;

    /// **DRIVE**
    ///
    /// Drive strength.
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const DRIVE_OFFSET: u32 = 4;
    pub const DRIVE_MASK: u32 = 0x00000030;
    pub const DRIVE_2M_A: u32 = 0 << 4;
    pub const DRIVE_2M_A_VAL: u32 = 0;
    pub const DRIVE_4M_A: u32 = 1 << 4;
    pub const DRIVE_4M_A_VAL: u32 = 1;
    pub const DRIVE_8M_A: u32 = 2 << 4;
    pub const DRIVE_8M_A_VAL: u32 = 2;
    pub const DRIVE_12M_A: u32 = 3 << 4;
    pub const DRIVE_12M_A_VAL: u32 = 3;

    /// **PUE**
    ///
    /// Pull up enable
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const PUE_OFFSET: u32 = 3;
    pub const PUE_MASK: u32 = 0x00000008;
    pub const PUE: u32 = 1 << 3;

    /// **PDE**
    ///
    /// Pull down enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const PDE_OFFSET: u32 = 2;
    pub const PDE_MASK: u32 = 0x00000004;
    pub const PDE: u32 = 1 << 2;

    /// **SCHMITT**
    ///
    /// Enable schmitt trigger
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SCHMITT_OFFSET: u32 = 1;
    pub const SCHMITT_MASK: u32 = 0x00000002;
    pub const SCHMITT: u32 = 1 << 1;

    /// **SLEWFAST**
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SLEWFAST_OFFSET: u32 = 0;
    pub const SLEWFAST_MASK: u32 = 0x00000001;
    pub const SLEWFAST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ISO** `[8]` (RW)
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub fn iso(&self) -> bool {
        (self.value.read_bits() & Self::ISO_MASK) != 0
    }

    /// Set ISO bit atomically (RW)
    #[inline(always)]
    pub fn set_iso(&self) {
        self.set_bits(Self::ISO_MASK);
    }

    /// Clear ISO bit atomically (RW)
    #[inline(always)]
    pub fn clr_iso(&self) {
        self.clr_bits(Self::ISO_MASK);
    }

    /// **OD** `[7]` (RW)
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub fn od(&self) -> bool {
        (self.value.read_bits() & Self::OD_MASK) != 0
    }

    /// Set OD bit atomically (RW)
    #[inline(always)]
    pub fn set_od(&self) {
        self.set_bits(Self::OD_MASK);
    }

    /// Clear OD bit atomically (RW)
    #[inline(always)]
    pub fn clr_od(&self) {
        self.clr_bits(Self::OD_MASK);
    }

    /// **IE** `[6]` (RW)
    ///
    /// Input enable
    #[inline(always)]
    pub fn ie(&self) -> bool {
        (self.value.read_bits() & Self::IE_MASK) != 0
    }

    /// Set IE bit atomically (RW)
    #[inline(always)]
    pub fn set_ie(&self) {
        self.set_bits(Self::IE_MASK);
    }

    /// Clear IE bit atomically (RW)
    #[inline(always)]
    pub fn clr_ie(&self) {
        self.clr_bits(Self::IE_MASK);
    }

    /// **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn drive(&self) -> u32 {
        (self.value.read_bits() & Self::DRIVE_MASK) >> Self::DRIVE_OFFSET
    }

    /// Write **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn write_drive(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DRIVE_MASK) | ((value << Self::DRIVE_OFFSET) & Self::DRIVE_MASK));
    }

    /// **PUE** `[3]` (RW)
    ///
    /// Pull up enable
    #[inline(always)]
    pub fn pue(&self) -> bool {
        (self.value.read_bits() & Self::PUE_MASK) != 0
    }

    /// Set PUE bit atomically (RW)
    #[inline(always)]
    pub fn set_pue(&self) {
        self.set_bits(Self::PUE_MASK);
    }

    /// Clear PUE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pue(&self) {
        self.clr_bits(Self::PUE_MASK);
    }

    /// **PDE** `[2]` (RW)
    ///
    /// Pull down enable
    #[inline(always)]
    pub fn pde(&self) -> bool {
        (self.value.read_bits() & Self::PDE_MASK) != 0
    }

    /// Set PDE bit atomically (RW)
    #[inline(always)]
    pub fn set_pde(&self) {
        self.set_bits(Self::PDE_MASK);
    }

    /// Clear PDE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pde(&self) {
        self.clr_bits(Self::PDE_MASK);
    }

    /// **SCHMITT** `[1]` (RW)
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub fn schmitt(&self) -> bool {
        (self.value.read_bits() & Self::SCHMITT_MASK) != 0
    }

    /// Set SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn set_schmitt(&self) {
        self.set_bits(Self::SCHMITT_MASK);
    }

    /// Clear SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn clr_schmitt(&self) {
        self.clr_bits(Self::SCHMITT_MASK);
    }

    /// **SLEWFAST** `[0]` (RW)
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub fn slewfast(&self) -> bool {
        (self.value.read_bits() & Self::SLEWFAST_MASK) != 0
    }

    /// Set SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn set_slewfast(&self) {
        self.set_bits(Self::SLEWFAST_MASK);
    }

    /// Clear SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn clr_slewfast(&self) {
        self.clr_bits(Self::SLEWFAST_MASK);
    }

}

/// GPIO8 value type for batch read/modify/write operations
///
/// Use `gpio8.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpio8.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Gpio8Val {
    bits: u32,
}

impl Gpio8Val {
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
        Self { bits: 0x00000116 }
    }

    /// Read **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn iso(&self) -> bool {
        (self.bits & Gpio8::ISO_MASK) != 0
    }

    /// Read **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn od(&self) -> bool {
        (self.bits & Gpio8::OD_MASK) != 0
    }

    /// Read **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        (self.bits & Gpio8::IE_MASK) != 0
    }

    /// Read **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn drive(&self) -> u32 {
        (self.bits & Gpio8::DRIVE_MASK) >> Gpio8::DRIVE_OFFSET
    }

    /// Read **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn pue(&self) -> bool {
        (self.bits & Gpio8::PUE_MASK) != 0
    }

    /// Read **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn pde(&self) -> bool {
        (self.bits & Gpio8::PDE_MASK) != 0
    }

    /// Read **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn schmitt(&self) -> bool {
        (self.bits & Gpio8::SCHMITT_MASK) != 0
    }

    /// Read **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn slewfast(&self) -> bool {
        (self.bits & Gpio8::SLEWFAST_MASK) != 0
    }

    /// Set **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn with_iso(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio8::ISO_MASK
            } else {
                self.bits & !Gpio8::ISO_MASK
            }
        }
    }

    /// Set **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn with_od(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio8::OD_MASK
            } else {
                self.bits & !Gpio8::OD_MASK
            }
        }
    }

    /// Set **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn with_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio8::IE_MASK
            } else {
                self.bits & !Gpio8::IE_MASK
            }
        }
    }

    /// Set **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn with_drive(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Gpio8::DRIVE_MASK) | ((value << Gpio8::DRIVE_OFFSET) & Gpio8::DRIVE_MASK)
        }
    }

    /// Set **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn with_pue(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio8::PUE_MASK
            } else {
                self.bits & !Gpio8::PUE_MASK
            }
        }
    }

    /// Set **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn with_pde(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio8::PDE_MASK
            } else {
                self.bits & !Gpio8::PDE_MASK
            }
        }
    }

    /// Set **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn with_schmitt(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio8::SCHMITT_MASK
            } else {
                self.bits & !Gpio8::SCHMITT_MASK
            }
        }
    }

    /// Set **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn with_slewfast(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio8::SLEWFAST_MASK
            } else {
                self.bits & !Gpio8::SLEWFAST_MASK
            }
        }
    }

}

impl Default for Gpio8Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Gpio8 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Gpio8Val {
        Gpio8Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Gpio8Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Gpio8Val) -> Gpio8Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// GPIO9 Register
///
/// Address offset: 0x0028
/// Reset value: 0x00000116
///
/// ## Fields
///
/// - **ISO** `[8]` (RW)
///
///   Pad isolation control. Remove this once the pad is configured by software.
///
/// - **OD** `[7]` (RW)
///
///   Output disable. Has priority over output enable from peripherals
///
/// - **IE** `[6]` (RW)
///
///   Input enable
///
/// - **DRIVE** `[5:4]` (RW)
///
///   Drive strength.
///
/// - **PUE** `[3]` (RW)
///
///   Pull up enable
///
/// - **PDE** `[2]` (RW)
///
///   Pull down enable
///
/// - **SCHMITT** `[1]` (RW)
///
///   Enable schmitt trigger
///
/// - **SLEWFAST** `[0]` (RW)
///
///   Slew rate control. 1 = Fast, 0 = Slow
#[repr(transparent)]
pub struct Gpio9 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Gpio9 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Gpio9 {
    // ======== Field Constants ========

    /// **ISO**
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ISO_OFFSET: u32 = 8;
    pub const ISO_MASK: u32 = 0x00000100;
    pub const ISO: u32 = 1 << 8;

    /// **OD**
    ///
    /// Output disable. Has priority over output enable from peripherals
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const OD_OFFSET: u32 = 7;
    pub const OD_MASK: u32 = 0x00000080;
    pub const OD: u32 = 1 << 7;

    /// **IE**
    ///
    /// Input enable
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const IE_OFFSET: u32 = 6;
    pub const IE_MASK: u32 = 0x00000040;
    pub const IE: u32 = 1 << 6;

    /// **DRIVE**
    ///
    /// Drive strength.
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const DRIVE_OFFSET: u32 = 4;
    pub const DRIVE_MASK: u32 = 0x00000030;
    pub const DRIVE_2M_A: u32 = 0 << 4;
    pub const DRIVE_2M_A_VAL: u32 = 0;
    pub const DRIVE_4M_A: u32 = 1 << 4;
    pub const DRIVE_4M_A_VAL: u32 = 1;
    pub const DRIVE_8M_A: u32 = 2 << 4;
    pub const DRIVE_8M_A_VAL: u32 = 2;
    pub const DRIVE_12M_A: u32 = 3 << 4;
    pub const DRIVE_12M_A_VAL: u32 = 3;

    /// **PUE**
    ///
    /// Pull up enable
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const PUE_OFFSET: u32 = 3;
    pub const PUE_MASK: u32 = 0x00000008;
    pub const PUE: u32 = 1 << 3;

    /// **PDE**
    ///
    /// Pull down enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const PDE_OFFSET: u32 = 2;
    pub const PDE_MASK: u32 = 0x00000004;
    pub const PDE: u32 = 1 << 2;

    /// **SCHMITT**
    ///
    /// Enable schmitt trigger
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SCHMITT_OFFSET: u32 = 1;
    pub const SCHMITT_MASK: u32 = 0x00000002;
    pub const SCHMITT: u32 = 1 << 1;

    /// **SLEWFAST**
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SLEWFAST_OFFSET: u32 = 0;
    pub const SLEWFAST_MASK: u32 = 0x00000001;
    pub const SLEWFAST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ISO** `[8]` (RW)
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub fn iso(&self) -> bool {
        (self.value.read_bits() & Self::ISO_MASK) != 0
    }

    /// Set ISO bit atomically (RW)
    #[inline(always)]
    pub fn set_iso(&self) {
        self.set_bits(Self::ISO_MASK);
    }

    /// Clear ISO bit atomically (RW)
    #[inline(always)]
    pub fn clr_iso(&self) {
        self.clr_bits(Self::ISO_MASK);
    }

    /// **OD** `[7]` (RW)
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub fn od(&self) -> bool {
        (self.value.read_bits() & Self::OD_MASK) != 0
    }

    /// Set OD bit atomically (RW)
    #[inline(always)]
    pub fn set_od(&self) {
        self.set_bits(Self::OD_MASK);
    }

    /// Clear OD bit atomically (RW)
    #[inline(always)]
    pub fn clr_od(&self) {
        self.clr_bits(Self::OD_MASK);
    }

    /// **IE** `[6]` (RW)
    ///
    /// Input enable
    #[inline(always)]
    pub fn ie(&self) -> bool {
        (self.value.read_bits() & Self::IE_MASK) != 0
    }

    /// Set IE bit atomically (RW)
    #[inline(always)]
    pub fn set_ie(&self) {
        self.set_bits(Self::IE_MASK);
    }

    /// Clear IE bit atomically (RW)
    #[inline(always)]
    pub fn clr_ie(&self) {
        self.clr_bits(Self::IE_MASK);
    }

    /// **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn drive(&self) -> u32 {
        (self.value.read_bits() & Self::DRIVE_MASK) >> Self::DRIVE_OFFSET
    }

    /// Write **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn write_drive(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DRIVE_MASK) | ((value << Self::DRIVE_OFFSET) & Self::DRIVE_MASK));
    }

    /// **PUE** `[3]` (RW)
    ///
    /// Pull up enable
    #[inline(always)]
    pub fn pue(&self) -> bool {
        (self.value.read_bits() & Self::PUE_MASK) != 0
    }

    /// Set PUE bit atomically (RW)
    #[inline(always)]
    pub fn set_pue(&self) {
        self.set_bits(Self::PUE_MASK);
    }

    /// Clear PUE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pue(&self) {
        self.clr_bits(Self::PUE_MASK);
    }

    /// **PDE** `[2]` (RW)
    ///
    /// Pull down enable
    #[inline(always)]
    pub fn pde(&self) -> bool {
        (self.value.read_bits() & Self::PDE_MASK) != 0
    }

    /// Set PDE bit atomically (RW)
    #[inline(always)]
    pub fn set_pde(&self) {
        self.set_bits(Self::PDE_MASK);
    }

    /// Clear PDE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pde(&self) {
        self.clr_bits(Self::PDE_MASK);
    }

    /// **SCHMITT** `[1]` (RW)
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub fn schmitt(&self) -> bool {
        (self.value.read_bits() & Self::SCHMITT_MASK) != 0
    }

    /// Set SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn set_schmitt(&self) {
        self.set_bits(Self::SCHMITT_MASK);
    }

    /// Clear SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn clr_schmitt(&self) {
        self.clr_bits(Self::SCHMITT_MASK);
    }

    /// **SLEWFAST** `[0]` (RW)
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub fn slewfast(&self) -> bool {
        (self.value.read_bits() & Self::SLEWFAST_MASK) != 0
    }

    /// Set SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn set_slewfast(&self) {
        self.set_bits(Self::SLEWFAST_MASK);
    }

    /// Clear SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn clr_slewfast(&self) {
        self.clr_bits(Self::SLEWFAST_MASK);
    }

}

/// GPIO9 value type for batch read/modify/write operations
///
/// Use `gpio9.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpio9.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Gpio9Val {
    bits: u32,
}

impl Gpio9Val {
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
        Self { bits: 0x00000116 }
    }

    /// Read **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn iso(&self) -> bool {
        (self.bits & Gpio9::ISO_MASK) != 0
    }

    /// Read **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn od(&self) -> bool {
        (self.bits & Gpio9::OD_MASK) != 0
    }

    /// Read **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        (self.bits & Gpio9::IE_MASK) != 0
    }

    /// Read **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn drive(&self) -> u32 {
        (self.bits & Gpio9::DRIVE_MASK) >> Gpio9::DRIVE_OFFSET
    }

    /// Read **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn pue(&self) -> bool {
        (self.bits & Gpio9::PUE_MASK) != 0
    }

    /// Read **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn pde(&self) -> bool {
        (self.bits & Gpio9::PDE_MASK) != 0
    }

    /// Read **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn schmitt(&self) -> bool {
        (self.bits & Gpio9::SCHMITT_MASK) != 0
    }

    /// Read **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn slewfast(&self) -> bool {
        (self.bits & Gpio9::SLEWFAST_MASK) != 0
    }

    /// Set **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn with_iso(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio9::ISO_MASK
            } else {
                self.bits & !Gpio9::ISO_MASK
            }
        }
    }

    /// Set **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn with_od(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio9::OD_MASK
            } else {
                self.bits & !Gpio9::OD_MASK
            }
        }
    }

    /// Set **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn with_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio9::IE_MASK
            } else {
                self.bits & !Gpio9::IE_MASK
            }
        }
    }

    /// Set **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn with_drive(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Gpio9::DRIVE_MASK) | ((value << Gpio9::DRIVE_OFFSET) & Gpio9::DRIVE_MASK)
        }
    }

    /// Set **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn with_pue(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio9::PUE_MASK
            } else {
                self.bits & !Gpio9::PUE_MASK
            }
        }
    }

    /// Set **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn with_pde(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio9::PDE_MASK
            } else {
                self.bits & !Gpio9::PDE_MASK
            }
        }
    }

    /// Set **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn with_schmitt(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio9::SCHMITT_MASK
            } else {
                self.bits & !Gpio9::SCHMITT_MASK
            }
        }
    }

    /// Set **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn with_slewfast(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio9::SLEWFAST_MASK
            } else {
                self.bits & !Gpio9::SLEWFAST_MASK
            }
        }
    }

}

impl Default for Gpio9Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Gpio9 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Gpio9Val {
        Gpio9Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Gpio9Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Gpio9Val) -> Gpio9Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// GPIO10 Register
///
/// Address offset: 0x002c
/// Reset value: 0x00000116
///
/// ## Fields
///
/// - **ISO** `[8]` (RW)
///
///   Pad isolation control. Remove this once the pad is configured by software.
///
/// - **OD** `[7]` (RW)
///
///   Output disable. Has priority over output enable from peripherals
///
/// - **IE** `[6]` (RW)
///
///   Input enable
///
/// - **DRIVE** `[5:4]` (RW)
///
///   Drive strength.
///
/// - **PUE** `[3]` (RW)
///
///   Pull up enable
///
/// - **PDE** `[2]` (RW)
///
///   Pull down enable
///
/// - **SCHMITT** `[1]` (RW)
///
///   Enable schmitt trigger
///
/// - **SLEWFAST** `[0]` (RW)
///
///   Slew rate control. 1 = Fast, 0 = Slow
#[repr(transparent)]
pub struct Gpio10 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Gpio10 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Gpio10 {
    // ======== Field Constants ========

    /// **ISO**
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ISO_OFFSET: u32 = 8;
    pub const ISO_MASK: u32 = 0x00000100;
    pub const ISO: u32 = 1 << 8;

    /// **OD**
    ///
    /// Output disable. Has priority over output enable from peripherals
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const OD_OFFSET: u32 = 7;
    pub const OD_MASK: u32 = 0x00000080;
    pub const OD: u32 = 1 << 7;

    /// **IE**
    ///
    /// Input enable
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const IE_OFFSET: u32 = 6;
    pub const IE_MASK: u32 = 0x00000040;
    pub const IE: u32 = 1 << 6;

    /// **DRIVE**
    ///
    /// Drive strength.
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const DRIVE_OFFSET: u32 = 4;
    pub const DRIVE_MASK: u32 = 0x00000030;
    pub const DRIVE_2M_A: u32 = 0 << 4;
    pub const DRIVE_2M_A_VAL: u32 = 0;
    pub const DRIVE_4M_A: u32 = 1 << 4;
    pub const DRIVE_4M_A_VAL: u32 = 1;
    pub const DRIVE_8M_A: u32 = 2 << 4;
    pub const DRIVE_8M_A_VAL: u32 = 2;
    pub const DRIVE_12M_A: u32 = 3 << 4;
    pub const DRIVE_12M_A_VAL: u32 = 3;

    /// **PUE**
    ///
    /// Pull up enable
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const PUE_OFFSET: u32 = 3;
    pub const PUE_MASK: u32 = 0x00000008;
    pub const PUE: u32 = 1 << 3;

    /// **PDE**
    ///
    /// Pull down enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const PDE_OFFSET: u32 = 2;
    pub const PDE_MASK: u32 = 0x00000004;
    pub const PDE: u32 = 1 << 2;

    /// **SCHMITT**
    ///
    /// Enable schmitt trigger
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SCHMITT_OFFSET: u32 = 1;
    pub const SCHMITT_MASK: u32 = 0x00000002;
    pub const SCHMITT: u32 = 1 << 1;

    /// **SLEWFAST**
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SLEWFAST_OFFSET: u32 = 0;
    pub const SLEWFAST_MASK: u32 = 0x00000001;
    pub const SLEWFAST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ISO** `[8]` (RW)
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub fn iso(&self) -> bool {
        (self.value.read_bits() & Self::ISO_MASK) != 0
    }

    /// Set ISO bit atomically (RW)
    #[inline(always)]
    pub fn set_iso(&self) {
        self.set_bits(Self::ISO_MASK);
    }

    /// Clear ISO bit atomically (RW)
    #[inline(always)]
    pub fn clr_iso(&self) {
        self.clr_bits(Self::ISO_MASK);
    }

    /// **OD** `[7]` (RW)
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub fn od(&self) -> bool {
        (self.value.read_bits() & Self::OD_MASK) != 0
    }

    /// Set OD bit atomically (RW)
    #[inline(always)]
    pub fn set_od(&self) {
        self.set_bits(Self::OD_MASK);
    }

    /// Clear OD bit atomically (RW)
    #[inline(always)]
    pub fn clr_od(&self) {
        self.clr_bits(Self::OD_MASK);
    }

    /// **IE** `[6]` (RW)
    ///
    /// Input enable
    #[inline(always)]
    pub fn ie(&self) -> bool {
        (self.value.read_bits() & Self::IE_MASK) != 0
    }

    /// Set IE bit atomically (RW)
    #[inline(always)]
    pub fn set_ie(&self) {
        self.set_bits(Self::IE_MASK);
    }

    /// Clear IE bit atomically (RW)
    #[inline(always)]
    pub fn clr_ie(&self) {
        self.clr_bits(Self::IE_MASK);
    }

    /// **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn drive(&self) -> u32 {
        (self.value.read_bits() & Self::DRIVE_MASK) >> Self::DRIVE_OFFSET
    }

    /// Write **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn write_drive(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DRIVE_MASK) | ((value << Self::DRIVE_OFFSET) & Self::DRIVE_MASK));
    }

    /// **PUE** `[3]` (RW)
    ///
    /// Pull up enable
    #[inline(always)]
    pub fn pue(&self) -> bool {
        (self.value.read_bits() & Self::PUE_MASK) != 0
    }

    /// Set PUE bit atomically (RW)
    #[inline(always)]
    pub fn set_pue(&self) {
        self.set_bits(Self::PUE_MASK);
    }

    /// Clear PUE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pue(&self) {
        self.clr_bits(Self::PUE_MASK);
    }

    /// **PDE** `[2]` (RW)
    ///
    /// Pull down enable
    #[inline(always)]
    pub fn pde(&self) -> bool {
        (self.value.read_bits() & Self::PDE_MASK) != 0
    }

    /// Set PDE bit atomically (RW)
    #[inline(always)]
    pub fn set_pde(&self) {
        self.set_bits(Self::PDE_MASK);
    }

    /// Clear PDE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pde(&self) {
        self.clr_bits(Self::PDE_MASK);
    }

    /// **SCHMITT** `[1]` (RW)
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub fn schmitt(&self) -> bool {
        (self.value.read_bits() & Self::SCHMITT_MASK) != 0
    }

    /// Set SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn set_schmitt(&self) {
        self.set_bits(Self::SCHMITT_MASK);
    }

    /// Clear SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn clr_schmitt(&self) {
        self.clr_bits(Self::SCHMITT_MASK);
    }

    /// **SLEWFAST** `[0]` (RW)
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub fn slewfast(&self) -> bool {
        (self.value.read_bits() & Self::SLEWFAST_MASK) != 0
    }

    /// Set SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn set_slewfast(&self) {
        self.set_bits(Self::SLEWFAST_MASK);
    }

    /// Clear SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn clr_slewfast(&self) {
        self.clr_bits(Self::SLEWFAST_MASK);
    }

}

/// GPIO10 value type for batch read/modify/write operations
///
/// Use `gpio10.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpio10.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Gpio10Val {
    bits: u32,
}

impl Gpio10Val {
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
        Self { bits: 0x00000116 }
    }

    /// Read **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn iso(&self) -> bool {
        (self.bits & Gpio10::ISO_MASK) != 0
    }

    /// Read **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn od(&self) -> bool {
        (self.bits & Gpio10::OD_MASK) != 0
    }

    /// Read **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        (self.bits & Gpio10::IE_MASK) != 0
    }

    /// Read **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn drive(&self) -> u32 {
        (self.bits & Gpio10::DRIVE_MASK) >> Gpio10::DRIVE_OFFSET
    }

    /// Read **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn pue(&self) -> bool {
        (self.bits & Gpio10::PUE_MASK) != 0
    }

    /// Read **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn pde(&self) -> bool {
        (self.bits & Gpio10::PDE_MASK) != 0
    }

    /// Read **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn schmitt(&self) -> bool {
        (self.bits & Gpio10::SCHMITT_MASK) != 0
    }

    /// Read **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn slewfast(&self) -> bool {
        (self.bits & Gpio10::SLEWFAST_MASK) != 0
    }

    /// Set **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn with_iso(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio10::ISO_MASK
            } else {
                self.bits & !Gpio10::ISO_MASK
            }
        }
    }

    /// Set **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn with_od(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio10::OD_MASK
            } else {
                self.bits & !Gpio10::OD_MASK
            }
        }
    }

    /// Set **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn with_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio10::IE_MASK
            } else {
                self.bits & !Gpio10::IE_MASK
            }
        }
    }

    /// Set **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn with_drive(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Gpio10::DRIVE_MASK) | ((value << Gpio10::DRIVE_OFFSET) & Gpio10::DRIVE_MASK)
        }
    }

    /// Set **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn with_pue(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio10::PUE_MASK
            } else {
                self.bits & !Gpio10::PUE_MASK
            }
        }
    }

    /// Set **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn with_pde(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio10::PDE_MASK
            } else {
                self.bits & !Gpio10::PDE_MASK
            }
        }
    }

    /// Set **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn with_schmitt(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio10::SCHMITT_MASK
            } else {
                self.bits & !Gpio10::SCHMITT_MASK
            }
        }
    }

    /// Set **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn with_slewfast(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio10::SLEWFAST_MASK
            } else {
                self.bits & !Gpio10::SLEWFAST_MASK
            }
        }
    }

}

impl Default for Gpio10Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Gpio10 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Gpio10Val {
        Gpio10Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Gpio10Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Gpio10Val) -> Gpio10Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// GPIO11 Register
///
/// Address offset: 0x0030
/// Reset value: 0x00000116
///
/// ## Fields
///
/// - **ISO** `[8]` (RW)
///
///   Pad isolation control. Remove this once the pad is configured by software.
///
/// - **OD** `[7]` (RW)
///
///   Output disable. Has priority over output enable from peripherals
///
/// - **IE** `[6]` (RW)
///
///   Input enable
///
/// - **DRIVE** `[5:4]` (RW)
///
///   Drive strength.
///
/// - **PUE** `[3]` (RW)
///
///   Pull up enable
///
/// - **PDE** `[2]` (RW)
///
///   Pull down enable
///
/// - **SCHMITT** `[1]` (RW)
///
///   Enable schmitt trigger
///
/// - **SLEWFAST** `[0]` (RW)
///
///   Slew rate control. 1 = Fast, 0 = Slow
#[repr(transparent)]
pub struct Gpio11 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Gpio11 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Gpio11 {
    // ======== Field Constants ========

    /// **ISO**
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ISO_OFFSET: u32 = 8;
    pub const ISO_MASK: u32 = 0x00000100;
    pub const ISO: u32 = 1 << 8;

    /// **OD**
    ///
    /// Output disable. Has priority over output enable from peripherals
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const OD_OFFSET: u32 = 7;
    pub const OD_MASK: u32 = 0x00000080;
    pub const OD: u32 = 1 << 7;

    /// **IE**
    ///
    /// Input enable
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const IE_OFFSET: u32 = 6;
    pub const IE_MASK: u32 = 0x00000040;
    pub const IE: u32 = 1 << 6;

    /// **DRIVE**
    ///
    /// Drive strength.
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const DRIVE_OFFSET: u32 = 4;
    pub const DRIVE_MASK: u32 = 0x00000030;
    pub const DRIVE_2M_A: u32 = 0 << 4;
    pub const DRIVE_2M_A_VAL: u32 = 0;
    pub const DRIVE_4M_A: u32 = 1 << 4;
    pub const DRIVE_4M_A_VAL: u32 = 1;
    pub const DRIVE_8M_A: u32 = 2 << 4;
    pub const DRIVE_8M_A_VAL: u32 = 2;
    pub const DRIVE_12M_A: u32 = 3 << 4;
    pub const DRIVE_12M_A_VAL: u32 = 3;

    /// **PUE**
    ///
    /// Pull up enable
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const PUE_OFFSET: u32 = 3;
    pub const PUE_MASK: u32 = 0x00000008;
    pub const PUE: u32 = 1 << 3;

    /// **PDE**
    ///
    /// Pull down enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const PDE_OFFSET: u32 = 2;
    pub const PDE_MASK: u32 = 0x00000004;
    pub const PDE: u32 = 1 << 2;

    /// **SCHMITT**
    ///
    /// Enable schmitt trigger
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SCHMITT_OFFSET: u32 = 1;
    pub const SCHMITT_MASK: u32 = 0x00000002;
    pub const SCHMITT: u32 = 1 << 1;

    /// **SLEWFAST**
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SLEWFAST_OFFSET: u32 = 0;
    pub const SLEWFAST_MASK: u32 = 0x00000001;
    pub const SLEWFAST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ISO** `[8]` (RW)
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub fn iso(&self) -> bool {
        (self.value.read_bits() & Self::ISO_MASK) != 0
    }

    /// Set ISO bit atomically (RW)
    #[inline(always)]
    pub fn set_iso(&self) {
        self.set_bits(Self::ISO_MASK);
    }

    /// Clear ISO bit atomically (RW)
    #[inline(always)]
    pub fn clr_iso(&self) {
        self.clr_bits(Self::ISO_MASK);
    }

    /// **OD** `[7]` (RW)
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub fn od(&self) -> bool {
        (self.value.read_bits() & Self::OD_MASK) != 0
    }

    /// Set OD bit atomically (RW)
    #[inline(always)]
    pub fn set_od(&self) {
        self.set_bits(Self::OD_MASK);
    }

    /// Clear OD bit atomically (RW)
    #[inline(always)]
    pub fn clr_od(&self) {
        self.clr_bits(Self::OD_MASK);
    }

    /// **IE** `[6]` (RW)
    ///
    /// Input enable
    #[inline(always)]
    pub fn ie(&self) -> bool {
        (self.value.read_bits() & Self::IE_MASK) != 0
    }

    /// Set IE bit atomically (RW)
    #[inline(always)]
    pub fn set_ie(&self) {
        self.set_bits(Self::IE_MASK);
    }

    /// Clear IE bit atomically (RW)
    #[inline(always)]
    pub fn clr_ie(&self) {
        self.clr_bits(Self::IE_MASK);
    }

    /// **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn drive(&self) -> u32 {
        (self.value.read_bits() & Self::DRIVE_MASK) >> Self::DRIVE_OFFSET
    }

    /// Write **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn write_drive(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DRIVE_MASK) | ((value << Self::DRIVE_OFFSET) & Self::DRIVE_MASK));
    }

    /// **PUE** `[3]` (RW)
    ///
    /// Pull up enable
    #[inline(always)]
    pub fn pue(&self) -> bool {
        (self.value.read_bits() & Self::PUE_MASK) != 0
    }

    /// Set PUE bit atomically (RW)
    #[inline(always)]
    pub fn set_pue(&self) {
        self.set_bits(Self::PUE_MASK);
    }

    /// Clear PUE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pue(&self) {
        self.clr_bits(Self::PUE_MASK);
    }

    /// **PDE** `[2]` (RW)
    ///
    /// Pull down enable
    #[inline(always)]
    pub fn pde(&self) -> bool {
        (self.value.read_bits() & Self::PDE_MASK) != 0
    }

    /// Set PDE bit atomically (RW)
    #[inline(always)]
    pub fn set_pde(&self) {
        self.set_bits(Self::PDE_MASK);
    }

    /// Clear PDE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pde(&self) {
        self.clr_bits(Self::PDE_MASK);
    }

    /// **SCHMITT** `[1]` (RW)
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub fn schmitt(&self) -> bool {
        (self.value.read_bits() & Self::SCHMITT_MASK) != 0
    }

    /// Set SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn set_schmitt(&self) {
        self.set_bits(Self::SCHMITT_MASK);
    }

    /// Clear SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn clr_schmitt(&self) {
        self.clr_bits(Self::SCHMITT_MASK);
    }

    /// **SLEWFAST** `[0]` (RW)
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub fn slewfast(&self) -> bool {
        (self.value.read_bits() & Self::SLEWFAST_MASK) != 0
    }

    /// Set SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn set_slewfast(&self) {
        self.set_bits(Self::SLEWFAST_MASK);
    }

    /// Clear SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn clr_slewfast(&self) {
        self.clr_bits(Self::SLEWFAST_MASK);
    }

}

/// GPIO11 value type for batch read/modify/write operations
///
/// Use `gpio11.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpio11.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Gpio11Val {
    bits: u32,
}

impl Gpio11Val {
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
        Self { bits: 0x00000116 }
    }

    /// Read **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn iso(&self) -> bool {
        (self.bits & Gpio11::ISO_MASK) != 0
    }

    /// Read **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn od(&self) -> bool {
        (self.bits & Gpio11::OD_MASK) != 0
    }

    /// Read **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        (self.bits & Gpio11::IE_MASK) != 0
    }

    /// Read **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn drive(&self) -> u32 {
        (self.bits & Gpio11::DRIVE_MASK) >> Gpio11::DRIVE_OFFSET
    }

    /// Read **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn pue(&self) -> bool {
        (self.bits & Gpio11::PUE_MASK) != 0
    }

    /// Read **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn pde(&self) -> bool {
        (self.bits & Gpio11::PDE_MASK) != 0
    }

    /// Read **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn schmitt(&self) -> bool {
        (self.bits & Gpio11::SCHMITT_MASK) != 0
    }

    /// Read **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn slewfast(&self) -> bool {
        (self.bits & Gpio11::SLEWFAST_MASK) != 0
    }

    /// Set **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn with_iso(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio11::ISO_MASK
            } else {
                self.bits & !Gpio11::ISO_MASK
            }
        }
    }

    /// Set **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn with_od(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio11::OD_MASK
            } else {
                self.bits & !Gpio11::OD_MASK
            }
        }
    }

    /// Set **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn with_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio11::IE_MASK
            } else {
                self.bits & !Gpio11::IE_MASK
            }
        }
    }

    /// Set **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn with_drive(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Gpio11::DRIVE_MASK) | ((value << Gpio11::DRIVE_OFFSET) & Gpio11::DRIVE_MASK)
        }
    }

    /// Set **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn with_pue(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio11::PUE_MASK
            } else {
                self.bits & !Gpio11::PUE_MASK
            }
        }
    }

    /// Set **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn with_pde(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio11::PDE_MASK
            } else {
                self.bits & !Gpio11::PDE_MASK
            }
        }
    }

    /// Set **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn with_schmitt(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio11::SCHMITT_MASK
            } else {
                self.bits & !Gpio11::SCHMITT_MASK
            }
        }
    }

    /// Set **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn with_slewfast(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio11::SLEWFAST_MASK
            } else {
                self.bits & !Gpio11::SLEWFAST_MASK
            }
        }
    }

}

impl Default for Gpio11Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Gpio11 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Gpio11Val {
        Gpio11Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Gpio11Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Gpio11Val) -> Gpio11Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// GPIO12 Register
///
/// Address offset: 0x0034
/// Reset value: 0x00000116
///
/// ## Fields
///
/// - **ISO** `[8]` (RW)
///
///   Pad isolation control. Remove this once the pad is configured by software.
///
/// - **OD** `[7]` (RW)
///
///   Output disable. Has priority over output enable from peripherals
///
/// - **IE** `[6]` (RW)
///
///   Input enable
///
/// - **DRIVE** `[5:4]` (RW)
///
///   Drive strength.
///
/// - **PUE** `[3]` (RW)
///
///   Pull up enable
///
/// - **PDE** `[2]` (RW)
///
///   Pull down enable
///
/// - **SCHMITT** `[1]` (RW)
///
///   Enable schmitt trigger
///
/// - **SLEWFAST** `[0]` (RW)
///
///   Slew rate control. 1 = Fast, 0 = Slow
#[repr(transparent)]
pub struct Gpio12 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Gpio12 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Gpio12 {
    // ======== Field Constants ========

    /// **ISO**
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ISO_OFFSET: u32 = 8;
    pub const ISO_MASK: u32 = 0x00000100;
    pub const ISO: u32 = 1 << 8;

    /// **OD**
    ///
    /// Output disable. Has priority over output enable from peripherals
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const OD_OFFSET: u32 = 7;
    pub const OD_MASK: u32 = 0x00000080;
    pub const OD: u32 = 1 << 7;

    /// **IE**
    ///
    /// Input enable
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const IE_OFFSET: u32 = 6;
    pub const IE_MASK: u32 = 0x00000040;
    pub const IE: u32 = 1 << 6;

    /// **DRIVE**
    ///
    /// Drive strength.
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const DRIVE_OFFSET: u32 = 4;
    pub const DRIVE_MASK: u32 = 0x00000030;
    pub const DRIVE_2M_A: u32 = 0 << 4;
    pub const DRIVE_2M_A_VAL: u32 = 0;
    pub const DRIVE_4M_A: u32 = 1 << 4;
    pub const DRIVE_4M_A_VAL: u32 = 1;
    pub const DRIVE_8M_A: u32 = 2 << 4;
    pub const DRIVE_8M_A_VAL: u32 = 2;
    pub const DRIVE_12M_A: u32 = 3 << 4;
    pub const DRIVE_12M_A_VAL: u32 = 3;

    /// **PUE**
    ///
    /// Pull up enable
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const PUE_OFFSET: u32 = 3;
    pub const PUE_MASK: u32 = 0x00000008;
    pub const PUE: u32 = 1 << 3;

    /// **PDE**
    ///
    /// Pull down enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const PDE_OFFSET: u32 = 2;
    pub const PDE_MASK: u32 = 0x00000004;
    pub const PDE: u32 = 1 << 2;

    /// **SCHMITT**
    ///
    /// Enable schmitt trigger
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SCHMITT_OFFSET: u32 = 1;
    pub const SCHMITT_MASK: u32 = 0x00000002;
    pub const SCHMITT: u32 = 1 << 1;

    /// **SLEWFAST**
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SLEWFAST_OFFSET: u32 = 0;
    pub const SLEWFAST_MASK: u32 = 0x00000001;
    pub const SLEWFAST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ISO** `[8]` (RW)
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub fn iso(&self) -> bool {
        (self.value.read_bits() & Self::ISO_MASK) != 0
    }

    /// Set ISO bit atomically (RW)
    #[inline(always)]
    pub fn set_iso(&self) {
        self.set_bits(Self::ISO_MASK);
    }

    /// Clear ISO bit atomically (RW)
    #[inline(always)]
    pub fn clr_iso(&self) {
        self.clr_bits(Self::ISO_MASK);
    }

    /// **OD** `[7]` (RW)
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub fn od(&self) -> bool {
        (self.value.read_bits() & Self::OD_MASK) != 0
    }

    /// Set OD bit atomically (RW)
    #[inline(always)]
    pub fn set_od(&self) {
        self.set_bits(Self::OD_MASK);
    }

    /// Clear OD bit atomically (RW)
    #[inline(always)]
    pub fn clr_od(&self) {
        self.clr_bits(Self::OD_MASK);
    }

    /// **IE** `[6]` (RW)
    ///
    /// Input enable
    #[inline(always)]
    pub fn ie(&self) -> bool {
        (self.value.read_bits() & Self::IE_MASK) != 0
    }

    /// Set IE bit atomically (RW)
    #[inline(always)]
    pub fn set_ie(&self) {
        self.set_bits(Self::IE_MASK);
    }

    /// Clear IE bit atomically (RW)
    #[inline(always)]
    pub fn clr_ie(&self) {
        self.clr_bits(Self::IE_MASK);
    }

    /// **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn drive(&self) -> u32 {
        (self.value.read_bits() & Self::DRIVE_MASK) >> Self::DRIVE_OFFSET
    }

    /// Write **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn write_drive(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DRIVE_MASK) | ((value << Self::DRIVE_OFFSET) & Self::DRIVE_MASK));
    }

    /// **PUE** `[3]` (RW)
    ///
    /// Pull up enable
    #[inline(always)]
    pub fn pue(&self) -> bool {
        (self.value.read_bits() & Self::PUE_MASK) != 0
    }

    /// Set PUE bit atomically (RW)
    #[inline(always)]
    pub fn set_pue(&self) {
        self.set_bits(Self::PUE_MASK);
    }

    /// Clear PUE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pue(&self) {
        self.clr_bits(Self::PUE_MASK);
    }

    /// **PDE** `[2]` (RW)
    ///
    /// Pull down enable
    #[inline(always)]
    pub fn pde(&self) -> bool {
        (self.value.read_bits() & Self::PDE_MASK) != 0
    }

    /// Set PDE bit atomically (RW)
    #[inline(always)]
    pub fn set_pde(&self) {
        self.set_bits(Self::PDE_MASK);
    }

    /// Clear PDE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pde(&self) {
        self.clr_bits(Self::PDE_MASK);
    }

    /// **SCHMITT** `[1]` (RW)
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub fn schmitt(&self) -> bool {
        (self.value.read_bits() & Self::SCHMITT_MASK) != 0
    }

    /// Set SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn set_schmitt(&self) {
        self.set_bits(Self::SCHMITT_MASK);
    }

    /// Clear SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn clr_schmitt(&self) {
        self.clr_bits(Self::SCHMITT_MASK);
    }

    /// **SLEWFAST** `[0]` (RW)
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub fn slewfast(&self) -> bool {
        (self.value.read_bits() & Self::SLEWFAST_MASK) != 0
    }

    /// Set SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn set_slewfast(&self) {
        self.set_bits(Self::SLEWFAST_MASK);
    }

    /// Clear SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn clr_slewfast(&self) {
        self.clr_bits(Self::SLEWFAST_MASK);
    }

}

/// GPIO12 value type for batch read/modify/write operations
///
/// Use `gpio12.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpio12.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Gpio12Val {
    bits: u32,
}

impl Gpio12Val {
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
        Self { bits: 0x00000116 }
    }

    /// Read **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn iso(&self) -> bool {
        (self.bits & Gpio12::ISO_MASK) != 0
    }

    /// Read **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn od(&self) -> bool {
        (self.bits & Gpio12::OD_MASK) != 0
    }

    /// Read **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        (self.bits & Gpio12::IE_MASK) != 0
    }

    /// Read **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn drive(&self) -> u32 {
        (self.bits & Gpio12::DRIVE_MASK) >> Gpio12::DRIVE_OFFSET
    }

    /// Read **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn pue(&self) -> bool {
        (self.bits & Gpio12::PUE_MASK) != 0
    }

    /// Read **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn pde(&self) -> bool {
        (self.bits & Gpio12::PDE_MASK) != 0
    }

    /// Read **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn schmitt(&self) -> bool {
        (self.bits & Gpio12::SCHMITT_MASK) != 0
    }

    /// Read **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn slewfast(&self) -> bool {
        (self.bits & Gpio12::SLEWFAST_MASK) != 0
    }

    /// Set **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn with_iso(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio12::ISO_MASK
            } else {
                self.bits & !Gpio12::ISO_MASK
            }
        }
    }

    /// Set **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn with_od(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio12::OD_MASK
            } else {
                self.bits & !Gpio12::OD_MASK
            }
        }
    }

    /// Set **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn with_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio12::IE_MASK
            } else {
                self.bits & !Gpio12::IE_MASK
            }
        }
    }

    /// Set **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn with_drive(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Gpio12::DRIVE_MASK) | ((value << Gpio12::DRIVE_OFFSET) & Gpio12::DRIVE_MASK)
        }
    }

    /// Set **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn with_pue(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio12::PUE_MASK
            } else {
                self.bits & !Gpio12::PUE_MASK
            }
        }
    }

    /// Set **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn with_pde(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio12::PDE_MASK
            } else {
                self.bits & !Gpio12::PDE_MASK
            }
        }
    }

    /// Set **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn with_schmitt(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio12::SCHMITT_MASK
            } else {
                self.bits & !Gpio12::SCHMITT_MASK
            }
        }
    }

    /// Set **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn with_slewfast(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio12::SLEWFAST_MASK
            } else {
                self.bits & !Gpio12::SLEWFAST_MASK
            }
        }
    }

}

impl Default for Gpio12Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Gpio12 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Gpio12Val {
        Gpio12Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Gpio12Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Gpio12Val) -> Gpio12Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// GPIO13 Register
///
/// Address offset: 0x0038
/// Reset value: 0x00000116
///
/// ## Fields
///
/// - **ISO** `[8]` (RW)
///
///   Pad isolation control. Remove this once the pad is configured by software.
///
/// - **OD** `[7]` (RW)
///
///   Output disable. Has priority over output enable from peripherals
///
/// - **IE** `[6]` (RW)
///
///   Input enable
///
/// - **DRIVE** `[5:4]` (RW)
///
///   Drive strength.
///
/// - **PUE** `[3]` (RW)
///
///   Pull up enable
///
/// - **PDE** `[2]` (RW)
///
///   Pull down enable
///
/// - **SCHMITT** `[1]` (RW)
///
///   Enable schmitt trigger
///
/// - **SLEWFAST** `[0]` (RW)
///
///   Slew rate control. 1 = Fast, 0 = Slow
#[repr(transparent)]
pub struct Gpio13 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Gpio13 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Gpio13 {
    // ======== Field Constants ========

    /// **ISO**
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ISO_OFFSET: u32 = 8;
    pub const ISO_MASK: u32 = 0x00000100;
    pub const ISO: u32 = 1 << 8;

    /// **OD**
    ///
    /// Output disable. Has priority over output enable from peripherals
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const OD_OFFSET: u32 = 7;
    pub const OD_MASK: u32 = 0x00000080;
    pub const OD: u32 = 1 << 7;

    /// **IE**
    ///
    /// Input enable
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const IE_OFFSET: u32 = 6;
    pub const IE_MASK: u32 = 0x00000040;
    pub const IE: u32 = 1 << 6;

    /// **DRIVE**
    ///
    /// Drive strength.
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const DRIVE_OFFSET: u32 = 4;
    pub const DRIVE_MASK: u32 = 0x00000030;
    pub const DRIVE_2M_A: u32 = 0 << 4;
    pub const DRIVE_2M_A_VAL: u32 = 0;
    pub const DRIVE_4M_A: u32 = 1 << 4;
    pub const DRIVE_4M_A_VAL: u32 = 1;
    pub const DRIVE_8M_A: u32 = 2 << 4;
    pub const DRIVE_8M_A_VAL: u32 = 2;
    pub const DRIVE_12M_A: u32 = 3 << 4;
    pub const DRIVE_12M_A_VAL: u32 = 3;

    /// **PUE**
    ///
    /// Pull up enable
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const PUE_OFFSET: u32 = 3;
    pub const PUE_MASK: u32 = 0x00000008;
    pub const PUE: u32 = 1 << 3;

    /// **PDE**
    ///
    /// Pull down enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const PDE_OFFSET: u32 = 2;
    pub const PDE_MASK: u32 = 0x00000004;
    pub const PDE: u32 = 1 << 2;

    /// **SCHMITT**
    ///
    /// Enable schmitt trigger
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SCHMITT_OFFSET: u32 = 1;
    pub const SCHMITT_MASK: u32 = 0x00000002;
    pub const SCHMITT: u32 = 1 << 1;

    /// **SLEWFAST**
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SLEWFAST_OFFSET: u32 = 0;
    pub const SLEWFAST_MASK: u32 = 0x00000001;
    pub const SLEWFAST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ISO** `[8]` (RW)
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub fn iso(&self) -> bool {
        (self.value.read_bits() & Self::ISO_MASK) != 0
    }

    /// Set ISO bit atomically (RW)
    #[inline(always)]
    pub fn set_iso(&self) {
        self.set_bits(Self::ISO_MASK);
    }

    /// Clear ISO bit atomically (RW)
    #[inline(always)]
    pub fn clr_iso(&self) {
        self.clr_bits(Self::ISO_MASK);
    }

    /// **OD** `[7]` (RW)
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub fn od(&self) -> bool {
        (self.value.read_bits() & Self::OD_MASK) != 0
    }

    /// Set OD bit atomically (RW)
    #[inline(always)]
    pub fn set_od(&self) {
        self.set_bits(Self::OD_MASK);
    }

    /// Clear OD bit atomically (RW)
    #[inline(always)]
    pub fn clr_od(&self) {
        self.clr_bits(Self::OD_MASK);
    }

    /// **IE** `[6]` (RW)
    ///
    /// Input enable
    #[inline(always)]
    pub fn ie(&self) -> bool {
        (self.value.read_bits() & Self::IE_MASK) != 0
    }

    /// Set IE bit atomically (RW)
    #[inline(always)]
    pub fn set_ie(&self) {
        self.set_bits(Self::IE_MASK);
    }

    /// Clear IE bit atomically (RW)
    #[inline(always)]
    pub fn clr_ie(&self) {
        self.clr_bits(Self::IE_MASK);
    }

    /// **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn drive(&self) -> u32 {
        (self.value.read_bits() & Self::DRIVE_MASK) >> Self::DRIVE_OFFSET
    }

    /// Write **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn write_drive(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DRIVE_MASK) | ((value << Self::DRIVE_OFFSET) & Self::DRIVE_MASK));
    }

    /// **PUE** `[3]` (RW)
    ///
    /// Pull up enable
    #[inline(always)]
    pub fn pue(&self) -> bool {
        (self.value.read_bits() & Self::PUE_MASK) != 0
    }

    /// Set PUE bit atomically (RW)
    #[inline(always)]
    pub fn set_pue(&self) {
        self.set_bits(Self::PUE_MASK);
    }

    /// Clear PUE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pue(&self) {
        self.clr_bits(Self::PUE_MASK);
    }

    /// **PDE** `[2]` (RW)
    ///
    /// Pull down enable
    #[inline(always)]
    pub fn pde(&self) -> bool {
        (self.value.read_bits() & Self::PDE_MASK) != 0
    }

    /// Set PDE bit atomically (RW)
    #[inline(always)]
    pub fn set_pde(&self) {
        self.set_bits(Self::PDE_MASK);
    }

    /// Clear PDE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pde(&self) {
        self.clr_bits(Self::PDE_MASK);
    }

    /// **SCHMITT** `[1]` (RW)
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub fn schmitt(&self) -> bool {
        (self.value.read_bits() & Self::SCHMITT_MASK) != 0
    }

    /// Set SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn set_schmitt(&self) {
        self.set_bits(Self::SCHMITT_MASK);
    }

    /// Clear SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn clr_schmitt(&self) {
        self.clr_bits(Self::SCHMITT_MASK);
    }

    /// **SLEWFAST** `[0]` (RW)
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub fn slewfast(&self) -> bool {
        (self.value.read_bits() & Self::SLEWFAST_MASK) != 0
    }

    /// Set SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn set_slewfast(&self) {
        self.set_bits(Self::SLEWFAST_MASK);
    }

    /// Clear SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn clr_slewfast(&self) {
        self.clr_bits(Self::SLEWFAST_MASK);
    }

}

/// GPIO13 value type for batch read/modify/write operations
///
/// Use `gpio13.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpio13.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Gpio13Val {
    bits: u32,
}

impl Gpio13Val {
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
        Self { bits: 0x00000116 }
    }

    /// Read **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn iso(&self) -> bool {
        (self.bits & Gpio13::ISO_MASK) != 0
    }

    /// Read **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn od(&self) -> bool {
        (self.bits & Gpio13::OD_MASK) != 0
    }

    /// Read **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        (self.bits & Gpio13::IE_MASK) != 0
    }

    /// Read **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn drive(&self) -> u32 {
        (self.bits & Gpio13::DRIVE_MASK) >> Gpio13::DRIVE_OFFSET
    }

    /// Read **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn pue(&self) -> bool {
        (self.bits & Gpio13::PUE_MASK) != 0
    }

    /// Read **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn pde(&self) -> bool {
        (self.bits & Gpio13::PDE_MASK) != 0
    }

    /// Read **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn schmitt(&self) -> bool {
        (self.bits & Gpio13::SCHMITT_MASK) != 0
    }

    /// Read **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn slewfast(&self) -> bool {
        (self.bits & Gpio13::SLEWFAST_MASK) != 0
    }

    /// Set **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn with_iso(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio13::ISO_MASK
            } else {
                self.bits & !Gpio13::ISO_MASK
            }
        }
    }

    /// Set **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn with_od(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio13::OD_MASK
            } else {
                self.bits & !Gpio13::OD_MASK
            }
        }
    }

    /// Set **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn with_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio13::IE_MASK
            } else {
                self.bits & !Gpio13::IE_MASK
            }
        }
    }

    /// Set **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn with_drive(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Gpio13::DRIVE_MASK) | ((value << Gpio13::DRIVE_OFFSET) & Gpio13::DRIVE_MASK)
        }
    }

    /// Set **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn with_pue(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio13::PUE_MASK
            } else {
                self.bits & !Gpio13::PUE_MASK
            }
        }
    }

    /// Set **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn with_pde(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio13::PDE_MASK
            } else {
                self.bits & !Gpio13::PDE_MASK
            }
        }
    }

    /// Set **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn with_schmitt(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio13::SCHMITT_MASK
            } else {
                self.bits & !Gpio13::SCHMITT_MASK
            }
        }
    }

    /// Set **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn with_slewfast(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio13::SLEWFAST_MASK
            } else {
                self.bits & !Gpio13::SLEWFAST_MASK
            }
        }
    }

}

impl Default for Gpio13Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Gpio13 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Gpio13Val {
        Gpio13Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Gpio13Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Gpio13Val) -> Gpio13Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// GPIO14 Register
///
/// Address offset: 0x003c
/// Reset value: 0x00000116
///
/// ## Fields
///
/// - **ISO** `[8]` (RW)
///
///   Pad isolation control. Remove this once the pad is configured by software.
///
/// - **OD** `[7]` (RW)
///
///   Output disable. Has priority over output enable from peripherals
///
/// - **IE** `[6]` (RW)
///
///   Input enable
///
/// - **DRIVE** `[5:4]` (RW)
///
///   Drive strength.
///
/// - **PUE** `[3]` (RW)
///
///   Pull up enable
///
/// - **PDE** `[2]` (RW)
///
///   Pull down enable
///
/// - **SCHMITT** `[1]` (RW)
///
///   Enable schmitt trigger
///
/// - **SLEWFAST** `[0]` (RW)
///
///   Slew rate control. 1 = Fast, 0 = Slow
#[repr(transparent)]
pub struct Gpio14 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Gpio14 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Gpio14 {
    // ======== Field Constants ========

    /// **ISO**
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ISO_OFFSET: u32 = 8;
    pub const ISO_MASK: u32 = 0x00000100;
    pub const ISO: u32 = 1 << 8;

    /// **OD**
    ///
    /// Output disable. Has priority over output enable from peripherals
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const OD_OFFSET: u32 = 7;
    pub const OD_MASK: u32 = 0x00000080;
    pub const OD: u32 = 1 << 7;

    /// **IE**
    ///
    /// Input enable
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const IE_OFFSET: u32 = 6;
    pub const IE_MASK: u32 = 0x00000040;
    pub const IE: u32 = 1 << 6;

    /// **DRIVE**
    ///
    /// Drive strength.
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const DRIVE_OFFSET: u32 = 4;
    pub const DRIVE_MASK: u32 = 0x00000030;
    pub const DRIVE_2M_A: u32 = 0 << 4;
    pub const DRIVE_2M_A_VAL: u32 = 0;
    pub const DRIVE_4M_A: u32 = 1 << 4;
    pub const DRIVE_4M_A_VAL: u32 = 1;
    pub const DRIVE_8M_A: u32 = 2 << 4;
    pub const DRIVE_8M_A_VAL: u32 = 2;
    pub const DRIVE_12M_A: u32 = 3 << 4;
    pub const DRIVE_12M_A_VAL: u32 = 3;

    /// **PUE**
    ///
    /// Pull up enable
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const PUE_OFFSET: u32 = 3;
    pub const PUE_MASK: u32 = 0x00000008;
    pub const PUE: u32 = 1 << 3;

    /// **PDE**
    ///
    /// Pull down enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const PDE_OFFSET: u32 = 2;
    pub const PDE_MASK: u32 = 0x00000004;
    pub const PDE: u32 = 1 << 2;

    /// **SCHMITT**
    ///
    /// Enable schmitt trigger
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SCHMITT_OFFSET: u32 = 1;
    pub const SCHMITT_MASK: u32 = 0x00000002;
    pub const SCHMITT: u32 = 1 << 1;

    /// **SLEWFAST**
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SLEWFAST_OFFSET: u32 = 0;
    pub const SLEWFAST_MASK: u32 = 0x00000001;
    pub const SLEWFAST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ISO** `[8]` (RW)
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub fn iso(&self) -> bool {
        (self.value.read_bits() & Self::ISO_MASK) != 0
    }

    /// Set ISO bit atomically (RW)
    #[inline(always)]
    pub fn set_iso(&self) {
        self.set_bits(Self::ISO_MASK);
    }

    /// Clear ISO bit atomically (RW)
    #[inline(always)]
    pub fn clr_iso(&self) {
        self.clr_bits(Self::ISO_MASK);
    }

    /// **OD** `[7]` (RW)
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub fn od(&self) -> bool {
        (self.value.read_bits() & Self::OD_MASK) != 0
    }

    /// Set OD bit atomically (RW)
    #[inline(always)]
    pub fn set_od(&self) {
        self.set_bits(Self::OD_MASK);
    }

    /// Clear OD bit atomically (RW)
    #[inline(always)]
    pub fn clr_od(&self) {
        self.clr_bits(Self::OD_MASK);
    }

    /// **IE** `[6]` (RW)
    ///
    /// Input enable
    #[inline(always)]
    pub fn ie(&self) -> bool {
        (self.value.read_bits() & Self::IE_MASK) != 0
    }

    /// Set IE bit atomically (RW)
    #[inline(always)]
    pub fn set_ie(&self) {
        self.set_bits(Self::IE_MASK);
    }

    /// Clear IE bit atomically (RW)
    #[inline(always)]
    pub fn clr_ie(&self) {
        self.clr_bits(Self::IE_MASK);
    }

    /// **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn drive(&self) -> u32 {
        (self.value.read_bits() & Self::DRIVE_MASK) >> Self::DRIVE_OFFSET
    }

    /// Write **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn write_drive(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DRIVE_MASK) | ((value << Self::DRIVE_OFFSET) & Self::DRIVE_MASK));
    }

    /// **PUE** `[3]` (RW)
    ///
    /// Pull up enable
    #[inline(always)]
    pub fn pue(&self) -> bool {
        (self.value.read_bits() & Self::PUE_MASK) != 0
    }

    /// Set PUE bit atomically (RW)
    #[inline(always)]
    pub fn set_pue(&self) {
        self.set_bits(Self::PUE_MASK);
    }

    /// Clear PUE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pue(&self) {
        self.clr_bits(Self::PUE_MASK);
    }

    /// **PDE** `[2]` (RW)
    ///
    /// Pull down enable
    #[inline(always)]
    pub fn pde(&self) -> bool {
        (self.value.read_bits() & Self::PDE_MASK) != 0
    }

    /// Set PDE bit atomically (RW)
    #[inline(always)]
    pub fn set_pde(&self) {
        self.set_bits(Self::PDE_MASK);
    }

    /// Clear PDE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pde(&self) {
        self.clr_bits(Self::PDE_MASK);
    }

    /// **SCHMITT** `[1]` (RW)
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub fn schmitt(&self) -> bool {
        (self.value.read_bits() & Self::SCHMITT_MASK) != 0
    }

    /// Set SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn set_schmitt(&self) {
        self.set_bits(Self::SCHMITT_MASK);
    }

    /// Clear SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn clr_schmitt(&self) {
        self.clr_bits(Self::SCHMITT_MASK);
    }

    /// **SLEWFAST** `[0]` (RW)
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub fn slewfast(&self) -> bool {
        (self.value.read_bits() & Self::SLEWFAST_MASK) != 0
    }

    /// Set SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn set_slewfast(&self) {
        self.set_bits(Self::SLEWFAST_MASK);
    }

    /// Clear SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn clr_slewfast(&self) {
        self.clr_bits(Self::SLEWFAST_MASK);
    }

}

/// GPIO14 value type for batch read/modify/write operations
///
/// Use `gpio14.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpio14.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Gpio14Val {
    bits: u32,
}

impl Gpio14Val {
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
        Self { bits: 0x00000116 }
    }

    /// Read **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn iso(&self) -> bool {
        (self.bits & Gpio14::ISO_MASK) != 0
    }

    /// Read **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn od(&self) -> bool {
        (self.bits & Gpio14::OD_MASK) != 0
    }

    /// Read **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        (self.bits & Gpio14::IE_MASK) != 0
    }

    /// Read **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn drive(&self) -> u32 {
        (self.bits & Gpio14::DRIVE_MASK) >> Gpio14::DRIVE_OFFSET
    }

    /// Read **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn pue(&self) -> bool {
        (self.bits & Gpio14::PUE_MASK) != 0
    }

    /// Read **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn pde(&self) -> bool {
        (self.bits & Gpio14::PDE_MASK) != 0
    }

    /// Read **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn schmitt(&self) -> bool {
        (self.bits & Gpio14::SCHMITT_MASK) != 0
    }

    /// Read **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn slewfast(&self) -> bool {
        (self.bits & Gpio14::SLEWFAST_MASK) != 0
    }

    /// Set **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn with_iso(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio14::ISO_MASK
            } else {
                self.bits & !Gpio14::ISO_MASK
            }
        }
    }

    /// Set **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn with_od(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio14::OD_MASK
            } else {
                self.bits & !Gpio14::OD_MASK
            }
        }
    }

    /// Set **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn with_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio14::IE_MASK
            } else {
                self.bits & !Gpio14::IE_MASK
            }
        }
    }

    /// Set **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn with_drive(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Gpio14::DRIVE_MASK) | ((value << Gpio14::DRIVE_OFFSET) & Gpio14::DRIVE_MASK)
        }
    }

    /// Set **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn with_pue(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio14::PUE_MASK
            } else {
                self.bits & !Gpio14::PUE_MASK
            }
        }
    }

    /// Set **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn with_pde(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio14::PDE_MASK
            } else {
                self.bits & !Gpio14::PDE_MASK
            }
        }
    }

    /// Set **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn with_schmitt(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio14::SCHMITT_MASK
            } else {
                self.bits & !Gpio14::SCHMITT_MASK
            }
        }
    }

    /// Set **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn with_slewfast(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio14::SLEWFAST_MASK
            } else {
                self.bits & !Gpio14::SLEWFAST_MASK
            }
        }
    }

}

impl Default for Gpio14Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Gpio14 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Gpio14Val {
        Gpio14Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Gpio14Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Gpio14Val) -> Gpio14Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// GPIO15 Register
///
/// Address offset: 0x0040
/// Reset value: 0x00000116
///
/// ## Fields
///
/// - **ISO** `[8]` (RW)
///
///   Pad isolation control. Remove this once the pad is configured by software.
///
/// - **OD** `[7]` (RW)
///
///   Output disable. Has priority over output enable from peripherals
///
/// - **IE** `[6]` (RW)
///
///   Input enable
///
/// - **DRIVE** `[5:4]` (RW)
///
///   Drive strength.
///
/// - **PUE** `[3]` (RW)
///
///   Pull up enable
///
/// - **PDE** `[2]` (RW)
///
///   Pull down enable
///
/// - **SCHMITT** `[1]` (RW)
///
///   Enable schmitt trigger
///
/// - **SLEWFAST** `[0]` (RW)
///
///   Slew rate control. 1 = Fast, 0 = Slow
#[repr(transparent)]
pub struct Gpio15 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Gpio15 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Gpio15 {
    // ======== Field Constants ========

    /// **ISO**
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ISO_OFFSET: u32 = 8;
    pub const ISO_MASK: u32 = 0x00000100;
    pub const ISO: u32 = 1 << 8;

    /// **OD**
    ///
    /// Output disable. Has priority over output enable from peripherals
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const OD_OFFSET: u32 = 7;
    pub const OD_MASK: u32 = 0x00000080;
    pub const OD: u32 = 1 << 7;

    /// **IE**
    ///
    /// Input enable
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const IE_OFFSET: u32 = 6;
    pub const IE_MASK: u32 = 0x00000040;
    pub const IE: u32 = 1 << 6;

    /// **DRIVE**
    ///
    /// Drive strength.
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const DRIVE_OFFSET: u32 = 4;
    pub const DRIVE_MASK: u32 = 0x00000030;
    pub const DRIVE_2M_A: u32 = 0 << 4;
    pub const DRIVE_2M_A_VAL: u32 = 0;
    pub const DRIVE_4M_A: u32 = 1 << 4;
    pub const DRIVE_4M_A_VAL: u32 = 1;
    pub const DRIVE_8M_A: u32 = 2 << 4;
    pub const DRIVE_8M_A_VAL: u32 = 2;
    pub const DRIVE_12M_A: u32 = 3 << 4;
    pub const DRIVE_12M_A_VAL: u32 = 3;

    /// **PUE**
    ///
    /// Pull up enable
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const PUE_OFFSET: u32 = 3;
    pub const PUE_MASK: u32 = 0x00000008;
    pub const PUE: u32 = 1 << 3;

    /// **PDE**
    ///
    /// Pull down enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const PDE_OFFSET: u32 = 2;
    pub const PDE_MASK: u32 = 0x00000004;
    pub const PDE: u32 = 1 << 2;

    /// **SCHMITT**
    ///
    /// Enable schmitt trigger
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SCHMITT_OFFSET: u32 = 1;
    pub const SCHMITT_MASK: u32 = 0x00000002;
    pub const SCHMITT: u32 = 1 << 1;

    /// **SLEWFAST**
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SLEWFAST_OFFSET: u32 = 0;
    pub const SLEWFAST_MASK: u32 = 0x00000001;
    pub const SLEWFAST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ISO** `[8]` (RW)
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub fn iso(&self) -> bool {
        (self.value.read_bits() & Self::ISO_MASK) != 0
    }

    /// Set ISO bit atomically (RW)
    #[inline(always)]
    pub fn set_iso(&self) {
        self.set_bits(Self::ISO_MASK);
    }

    /// Clear ISO bit atomically (RW)
    #[inline(always)]
    pub fn clr_iso(&self) {
        self.clr_bits(Self::ISO_MASK);
    }

    /// **OD** `[7]` (RW)
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub fn od(&self) -> bool {
        (self.value.read_bits() & Self::OD_MASK) != 0
    }

    /// Set OD bit atomically (RW)
    #[inline(always)]
    pub fn set_od(&self) {
        self.set_bits(Self::OD_MASK);
    }

    /// Clear OD bit atomically (RW)
    #[inline(always)]
    pub fn clr_od(&self) {
        self.clr_bits(Self::OD_MASK);
    }

    /// **IE** `[6]` (RW)
    ///
    /// Input enable
    #[inline(always)]
    pub fn ie(&self) -> bool {
        (self.value.read_bits() & Self::IE_MASK) != 0
    }

    /// Set IE bit atomically (RW)
    #[inline(always)]
    pub fn set_ie(&self) {
        self.set_bits(Self::IE_MASK);
    }

    /// Clear IE bit atomically (RW)
    #[inline(always)]
    pub fn clr_ie(&self) {
        self.clr_bits(Self::IE_MASK);
    }

    /// **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn drive(&self) -> u32 {
        (self.value.read_bits() & Self::DRIVE_MASK) >> Self::DRIVE_OFFSET
    }

    /// Write **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn write_drive(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DRIVE_MASK) | ((value << Self::DRIVE_OFFSET) & Self::DRIVE_MASK));
    }

    /// **PUE** `[3]` (RW)
    ///
    /// Pull up enable
    #[inline(always)]
    pub fn pue(&self) -> bool {
        (self.value.read_bits() & Self::PUE_MASK) != 0
    }

    /// Set PUE bit atomically (RW)
    #[inline(always)]
    pub fn set_pue(&self) {
        self.set_bits(Self::PUE_MASK);
    }

    /// Clear PUE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pue(&self) {
        self.clr_bits(Self::PUE_MASK);
    }

    /// **PDE** `[2]` (RW)
    ///
    /// Pull down enable
    #[inline(always)]
    pub fn pde(&self) -> bool {
        (self.value.read_bits() & Self::PDE_MASK) != 0
    }

    /// Set PDE bit atomically (RW)
    #[inline(always)]
    pub fn set_pde(&self) {
        self.set_bits(Self::PDE_MASK);
    }

    /// Clear PDE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pde(&self) {
        self.clr_bits(Self::PDE_MASK);
    }

    /// **SCHMITT** `[1]` (RW)
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub fn schmitt(&self) -> bool {
        (self.value.read_bits() & Self::SCHMITT_MASK) != 0
    }

    /// Set SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn set_schmitt(&self) {
        self.set_bits(Self::SCHMITT_MASK);
    }

    /// Clear SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn clr_schmitt(&self) {
        self.clr_bits(Self::SCHMITT_MASK);
    }

    /// **SLEWFAST** `[0]` (RW)
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub fn slewfast(&self) -> bool {
        (self.value.read_bits() & Self::SLEWFAST_MASK) != 0
    }

    /// Set SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn set_slewfast(&self) {
        self.set_bits(Self::SLEWFAST_MASK);
    }

    /// Clear SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn clr_slewfast(&self) {
        self.clr_bits(Self::SLEWFAST_MASK);
    }

}

/// GPIO15 value type for batch read/modify/write operations
///
/// Use `gpio15.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpio15.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Gpio15Val {
    bits: u32,
}

impl Gpio15Val {
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
        Self { bits: 0x00000116 }
    }

    /// Read **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn iso(&self) -> bool {
        (self.bits & Gpio15::ISO_MASK) != 0
    }

    /// Read **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn od(&self) -> bool {
        (self.bits & Gpio15::OD_MASK) != 0
    }

    /// Read **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        (self.bits & Gpio15::IE_MASK) != 0
    }

    /// Read **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn drive(&self) -> u32 {
        (self.bits & Gpio15::DRIVE_MASK) >> Gpio15::DRIVE_OFFSET
    }

    /// Read **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn pue(&self) -> bool {
        (self.bits & Gpio15::PUE_MASK) != 0
    }

    /// Read **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn pde(&self) -> bool {
        (self.bits & Gpio15::PDE_MASK) != 0
    }

    /// Read **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn schmitt(&self) -> bool {
        (self.bits & Gpio15::SCHMITT_MASK) != 0
    }

    /// Read **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn slewfast(&self) -> bool {
        (self.bits & Gpio15::SLEWFAST_MASK) != 0
    }

    /// Set **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn with_iso(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio15::ISO_MASK
            } else {
                self.bits & !Gpio15::ISO_MASK
            }
        }
    }

    /// Set **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn with_od(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio15::OD_MASK
            } else {
                self.bits & !Gpio15::OD_MASK
            }
        }
    }

    /// Set **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn with_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio15::IE_MASK
            } else {
                self.bits & !Gpio15::IE_MASK
            }
        }
    }

    /// Set **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn with_drive(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Gpio15::DRIVE_MASK) | ((value << Gpio15::DRIVE_OFFSET) & Gpio15::DRIVE_MASK)
        }
    }

    /// Set **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn with_pue(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio15::PUE_MASK
            } else {
                self.bits & !Gpio15::PUE_MASK
            }
        }
    }

    /// Set **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn with_pde(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio15::PDE_MASK
            } else {
                self.bits & !Gpio15::PDE_MASK
            }
        }
    }

    /// Set **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn with_schmitt(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio15::SCHMITT_MASK
            } else {
                self.bits & !Gpio15::SCHMITT_MASK
            }
        }
    }

    /// Set **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn with_slewfast(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio15::SLEWFAST_MASK
            } else {
                self.bits & !Gpio15::SLEWFAST_MASK
            }
        }
    }

}

impl Default for Gpio15Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Gpio15 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Gpio15Val {
        Gpio15Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Gpio15Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Gpio15Val) -> Gpio15Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// GPIO16 Register
///
/// Address offset: 0x0044
/// Reset value: 0x00000116
///
/// ## Fields
///
/// - **ISO** `[8]` (RW)
///
///   Pad isolation control. Remove this once the pad is configured by software.
///
/// - **OD** `[7]` (RW)
///
///   Output disable. Has priority over output enable from peripherals
///
/// - **IE** `[6]` (RW)
///
///   Input enable
///
/// - **DRIVE** `[5:4]` (RW)
///
///   Drive strength.
///
/// - **PUE** `[3]` (RW)
///
///   Pull up enable
///
/// - **PDE** `[2]` (RW)
///
///   Pull down enable
///
/// - **SCHMITT** `[1]` (RW)
///
///   Enable schmitt trigger
///
/// - **SLEWFAST** `[0]` (RW)
///
///   Slew rate control. 1 = Fast, 0 = Slow
#[repr(transparent)]
pub struct Gpio16 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Gpio16 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Gpio16 {
    // ======== Field Constants ========

    /// **ISO**
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ISO_OFFSET: u32 = 8;
    pub const ISO_MASK: u32 = 0x00000100;
    pub const ISO: u32 = 1 << 8;

    /// **OD**
    ///
    /// Output disable. Has priority over output enable from peripherals
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const OD_OFFSET: u32 = 7;
    pub const OD_MASK: u32 = 0x00000080;
    pub const OD: u32 = 1 << 7;

    /// **IE**
    ///
    /// Input enable
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const IE_OFFSET: u32 = 6;
    pub const IE_MASK: u32 = 0x00000040;
    pub const IE: u32 = 1 << 6;

    /// **DRIVE**
    ///
    /// Drive strength.
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const DRIVE_OFFSET: u32 = 4;
    pub const DRIVE_MASK: u32 = 0x00000030;
    pub const DRIVE_2M_A: u32 = 0 << 4;
    pub const DRIVE_2M_A_VAL: u32 = 0;
    pub const DRIVE_4M_A: u32 = 1 << 4;
    pub const DRIVE_4M_A_VAL: u32 = 1;
    pub const DRIVE_8M_A: u32 = 2 << 4;
    pub const DRIVE_8M_A_VAL: u32 = 2;
    pub const DRIVE_12M_A: u32 = 3 << 4;
    pub const DRIVE_12M_A_VAL: u32 = 3;

    /// **PUE**
    ///
    /// Pull up enable
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const PUE_OFFSET: u32 = 3;
    pub const PUE_MASK: u32 = 0x00000008;
    pub const PUE: u32 = 1 << 3;

    /// **PDE**
    ///
    /// Pull down enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const PDE_OFFSET: u32 = 2;
    pub const PDE_MASK: u32 = 0x00000004;
    pub const PDE: u32 = 1 << 2;

    /// **SCHMITT**
    ///
    /// Enable schmitt trigger
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SCHMITT_OFFSET: u32 = 1;
    pub const SCHMITT_MASK: u32 = 0x00000002;
    pub const SCHMITT: u32 = 1 << 1;

    /// **SLEWFAST**
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SLEWFAST_OFFSET: u32 = 0;
    pub const SLEWFAST_MASK: u32 = 0x00000001;
    pub const SLEWFAST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ISO** `[8]` (RW)
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub fn iso(&self) -> bool {
        (self.value.read_bits() & Self::ISO_MASK) != 0
    }

    /// Set ISO bit atomically (RW)
    #[inline(always)]
    pub fn set_iso(&self) {
        self.set_bits(Self::ISO_MASK);
    }

    /// Clear ISO bit atomically (RW)
    #[inline(always)]
    pub fn clr_iso(&self) {
        self.clr_bits(Self::ISO_MASK);
    }

    /// **OD** `[7]` (RW)
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub fn od(&self) -> bool {
        (self.value.read_bits() & Self::OD_MASK) != 0
    }

    /// Set OD bit atomically (RW)
    #[inline(always)]
    pub fn set_od(&self) {
        self.set_bits(Self::OD_MASK);
    }

    /// Clear OD bit atomically (RW)
    #[inline(always)]
    pub fn clr_od(&self) {
        self.clr_bits(Self::OD_MASK);
    }

    /// **IE** `[6]` (RW)
    ///
    /// Input enable
    #[inline(always)]
    pub fn ie(&self) -> bool {
        (self.value.read_bits() & Self::IE_MASK) != 0
    }

    /// Set IE bit atomically (RW)
    #[inline(always)]
    pub fn set_ie(&self) {
        self.set_bits(Self::IE_MASK);
    }

    /// Clear IE bit atomically (RW)
    #[inline(always)]
    pub fn clr_ie(&self) {
        self.clr_bits(Self::IE_MASK);
    }

    /// **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn drive(&self) -> u32 {
        (self.value.read_bits() & Self::DRIVE_MASK) >> Self::DRIVE_OFFSET
    }

    /// Write **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn write_drive(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DRIVE_MASK) | ((value << Self::DRIVE_OFFSET) & Self::DRIVE_MASK));
    }

    /// **PUE** `[3]` (RW)
    ///
    /// Pull up enable
    #[inline(always)]
    pub fn pue(&self) -> bool {
        (self.value.read_bits() & Self::PUE_MASK) != 0
    }

    /// Set PUE bit atomically (RW)
    #[inline(always)]
    pub fn set_pue(&self) {
        self.set_bits(Self::PUE_MASK);
    }

    /// Clear PUE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pue(&self) {
        self.clr_bits(Self::PUE_MASK);
    }

    /// **PDE** `[2]` (RW)
    ///
    /// Pull down enable
    #[inline(always)]
    pub fn pde(&self) -> bool {
        (self.value.read_bits() & Self::PDE_MASK) != 0
    }

    /// Set PDE bit atomically (RW)
    #[inline(always)]
    pub fn set_pde(&self) {
        self.set_bits(Self::PDE_MASK);
    }

    /// Clear PDE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pde(&self) {
        self.clr_bits(Self::PDE_MASK);
    }

    /// **SCHMITT** `[1]` (RW)
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub fn schmitt(&self) -> bool {
        (self.value.read_bits() & Self::SCHMITT_MASK) != 0
    }

    /// Set SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn set_schmitt(&self) {
        self.set_bits(Self::SCHMITT_MASK);
    }

    /// Clear SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn clr_schmitt(&self) {
        self.clr_bits(Self::SCHMITT_MASK);
    }

    /// **SLEWFAST** `[0]` (RW)
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub fn slewfast(&self) -> bool {
        (self.value.read_bits() & Self::SLEWFAST_MASK) != 0
    }

    /// Set SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn set_slewfast(&self) {
        self.set_bits(Self::SLEWFAST_MASK);
    }

    /// Clear SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn clr_slewfast(&self) {
        self.clr_bits(Self::SLEWFAST_MASK);
    }

}

/// GPIO16 value type for batch read/modify/write operations
///
/// Use `gpio16.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpio16.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Gpio16Val {
    bits: u32,
}

impl Gpio16Val {
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
        Self { bits: 0x00000116 }
    }

    /// Read **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn iso(&self) -> bool {
        (self.bits & Gpio16::ISO_MASK) != 0
    }

    /// Read **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn od(&self) -> bool {
        (self.bits & Gpio16::OD_MASK) != 0
    }

    /// Read **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        (self.bits & Gpio16::IE_MASK) != 0
    }

    /// Read **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn drive(&self) -> u32 {
        (self.bits & Gpio16::DRIVE_MASK) >> Gpio16::DRIVE_OFFSET
    }

    /// Read **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn pue(&self) -> bool {
        (self.bits & Gpio16::PUE_MASK) != 0
    }

    /// Read **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn pde(&self) -> bool {
        (self.bits & Gpio16::PDE_MASK) != 0
    }

    /// Read **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn schmitt(&self) -> bool {
        (self.bits & Gpio16::SCHMITT_MASK) != 0
    }

    /// Read **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn slewfast(&self) -> bool {
        (self.bits & Gpio16::SLEWFAST_MASK) != 0
    }

    /// Set **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn with_iso(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio16::ISO_MASK
            } else {
                self.bits & !Gpio16::ISO_MASK
            }
        }
    }

    /// Set **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn with_od(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio16::OD_MASK
            } else {
                self.bits & !Gpio16::OD_MASK
            }
        }
    }

    /// Set **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn with_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio16::IE_MASK
            } else {
                self.bits & !Gpio16::IE_MASK
            }
        }
    }

    /// Set **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn with_drive(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Gpio16::DRIVE_MASK) | ((value << Gpio16::DRIVE_OFFSET) & Gpio16::DRIVE_MASK)
        }
    }

    /// Set **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn with_pue(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio16::PUE_MASK
            } else {
                self.bits & !Gpio16::PUE_MASK
            }
        }
    }

    /// Set **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn with_pde(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio16::PDE_MASK
            } else {
                self.bits & !Gpio16::PDE_MASK
            }
        }
    }

    /// Set **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn with_schmitt(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio16::SCHMITT_MASK
            } else {
                self.bits & !Gpio16::SCHMITT_MASK
            }
        }
    }

    /// Set **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn with_slewfast(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio16::SLEWFAST_MASK
            } else {
                self.bits & !Gpio16::SLEWFAST_MASK
            }
        }
    }

}

impl Default for Gpio16Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Gpio16 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Gpio16Val {
        Gpio16Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Gpio16Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Gpio16Val) -> Gpio16Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// GPIO17 Register
///
/// Address offset: 0x0048
/// Reset value: 0x00000116
///
/// ## Fields
///
/// - **ISO** `[8]` (RW)
///
///   Pad isolation control. Remove this once the pad is configured by software.
///
/// - **OD** `[7]` (RW)
///
///   Output disable. Has priority over output enable from peripherals
///
/// - **IE** `[6]` (RW)
///
///   Input enable
///
/// - **DRIVE** `[5:4]` (RW)
///
///   Drive strength.
///
/// - **PUE** `[3]` (RW)
///
///   Pull up enable
///
/// - **PDE** `[2]` (RW)
///
///   Pull down enable
///
/// - **SCHMITT** `[1]` (RW)
///
///   Enable schmitt trigger
///
/// - **SLEWFAST** `[0]` (RW)
///
///   Slew rate control. 1 = Fast, 0 = Slow
#[repr(transparent)]
pub struct Gpio17 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Gpio17 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Gpio17 {
    // ======== Field Constants ========

    /// **ISO**
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ISO_OFFSET: u32 = 8;
    pub const ISO_MASK: u32 = 0x00000100;
    pub const ISO: u32 = 1 << 8;

    /// **OD**
    ///
    /// Output disable. Has priority over output enable from peripherals
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const OD_OFFSET: u32 = 7;
    pub const OD_MASK: u32 = 0x00000080;
    pub const OD: u32 = 1 << 7;

    /// **IE**
    ///
    /// Input enable
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const IE_OFFSET: u32 = 6;
    pub const IE_MASK: u32 = 0x00000040;
    pub const IE: u32 = 1 << 6;

    /// **DRIVE**
    ///
    /// Drive strength.
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const DRIVE_OFFSET: u32 = 4;
    pub const DRIVE_MASK: u32 = 0x00000030;
    pub const DRIVE_2M_A: u32 = 0 << 4;
    pub const DRIVE_2M_A_VAL: u32 = 0;
    pub const DRIVE_4M_A: u32 = 1 << 4;
    pub const DRIVE_4M_A_VAL: u32 = 1;
    pub const DRIVE_8M_A: u32 = 2 << 4;
    pub const DRIVE_8M_A_VAL: u32 = 2;
    pub const DRIVE_12M_A: u32 = 3 << 4;
    pub const DRIVE_12M_A_VAL: u32 = 3;

    /// **PUE**
    ///
    /// Pull up enable
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const PUE_OFFSET: u32 = 3;
    pub const PUE_MASK: u32 = 0x00000008;
    pub const PUE: u32 = 1 << 3;

    /// **PDE**
    ///
    /// Pull down enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const PDE_OFFSET: u32 = 2;
    pub const PDE_MASK: u32 = 0x00000004;
    pub const PDE: u32 = 1 << 2;

    /// **SCHMITT**
    ///
    /// Enable schmitt trigger
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SCHMITT_OFFSET: u32 = 1;
    pub const SCHMITT_MASK: u32 = 0x00000002;
    pub const SCHMITT: u32 = 1 << 1;

    /// **SLEWFAST**
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SLEWFAST_OFFSET: u32 = 0;
    pub const SLEWFAST_MASK: u32 = 0x00000001;
    pub const SLEWFAST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ISO** `[8]` (RW)
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub fn iso(&self) -> bool {
        (self.value.read_bits() & Self::ISO_MASK) != 0
    }

    /// Set ISO bit atomically (RW)
    #[inline(always)]
    pub fn set_iso(&self) {
        self.set_bits(Self::ISO_MASK);
    }

    /// Clear ISO bit atomically (RW)
    #[inline(always)]
    pub fn clr_iso(&self) {
        self.clr_bits(Self::ISO_MASK);
    }

    /// **OD** `[7]` (RW)
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub fn od(&self) -> bool {
        (self.value.read_bits() & Self::OD_MASK) != 0
    }

    /// Set OD bit atomically (RW)
    #[inline(always)]
    pub fn set_od(&self) {
        self.set_bits(Self::OD_MASK);
    }

    /// Clear OD bit atomically (RW)
    #[inline(always)]
    pub fn clr_od(&self) {
        self.clr_bits(Self::OD_MASK);
    }

    /// **IE** `[6]` (RW)
    ///
    /// Input enable
    #[inline(always)]
    pub fn ie(&self) -> bool {
        (self.value.read_bits() & Self::IE_MASK) != 0
    }

    /// Set IE bit atomically (RW)
    #[inline(always)]
    pub fn set_ie(&self) {
        self.set_bits(Self::IE_MASK);
    }

    /// Clear IE bit atomically (RW)
    #[inline(always)]
    pub fn clr_ie(&self) {
        self.clr_bits(Self::IE_MASK);
    }

    /// **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn drive(&self) -> u32 {
        (self.value.read_bits() & Self::DRIVE_MASK) >> Self::DRIVE_OFFSET
    }

    /// Write **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn write_drive(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DRIVE_MASK) | ((value << Self::DRIVE_OFFSET) & Self::DRIVE_MASK));
    }

    /// **PUE** `[3]` (RW)
    ///
    /// Pull up enable
    #[inline(always)]
    pub fn pue(&self) -> bool {
        (self.value.read_bits() & Self::PUE_MASK) != 0
    }

    /// Set PUE bit atomically (RW)
    #[inline(always)]
    pub fn set_pue(&self) {
        self.set_bits(Self::PUE_MASK);
    }

    /// Clear PUE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pue(&self) {
        self.clr_bits(Self::PUE_MASK);
    }

    /// **PDE** `[2]` (RW)
    ///
    /// Pull down enable
    #[inline(always)]
    pub fn pde(&self) -> bool {
        (self.value.read_bits() & Self::PDE_MASK) != 0
    }

    /// Set PDE bit atomically (RW)
    #[inline(always)]
    pub fn set_pde(&self) {
        self.set_bits(Self::PDE_MASK);
    }

    /// Clear PDE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pde(&self) {
        self.clr_bits(Self::PDE_MASK);
    }

    /// **SCHMITT** `[1]` (RW)
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub fn schmitt(&self) -> bool {
        (self.value.read_bits() & Self::SCHMITT_MASK) != 0
    }

    /// Set SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn set_schmitt(&self) {
        self.set_bits(Self::SCHMITT_MASK);
    }

    /// Clear SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn clr_schmitt(&self) {
        self.clr_bits(Self::SCHMITT_MASK);
    }

    /// **SLEWFAST** `[0]` (RW)
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub fn slewfast(&self) -> bool {
        (self.value.read_bits() & Self::SLEWFAST_MASK) != 0
    }

    /// Set SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn set_slewfast(&self) {
        self.set_bits(Self::SLEWFAST_MASK);
    }

    /// Clear SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn clr_slewfast(&self) {
        self.clr_bits(Self::SLEWFAST_MASK);
    }

}

/// GPIO17 value type for batch read/modify/write operations
///
/// Use `gpio17.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpio17.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Gpio17Val {
    bits: u32,
}

impl Gpio17Val {
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
        Self { bits: 0x00000116 }
    }

    /// Read **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn iso(&self) -> bool {
        (self.bits & Gpio17::ISO_MASK) != 0
    }

    /// Read **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn od(&self) -> bool {
        (self.bits & Gpio17::OD_MASK) != 0
    }

    /// Read **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        (self.bits & Gpio17::IE_MASK) != 0
    }

    /// Read **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn drive(&self) -> u32 {
        (self.bits & Gpio17::DRIVE_MASK) >> Gpio17::DRIVE_OFFSET
    }

    /// Read **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn pue(&self) -> bool {
        (self.bits & Gpio17::PUE_MASK) != 0
    }

    /// Read **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn pde(&self) -> bool {
        (self.bits & Gpio17::PDE_MASK) != 0
    }

    /// Read **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn schmitt(&self) -> bool {
        (self.bits & Gpio17::SCHMITT_MASK) != 0
    }

    /// Read **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn slewfast(&self) -> bool {
        (self.bits & Gpio17::SLEWFAST_MASK) != 0
    }

    /// Set **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn with_iso(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio17::ISO_MASK
            } else {
                self.bits & !Gpio17::ISO_MASK
            }
        }
    }

    /// Set **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn with_od(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio17::OD_MASK
            } else {
                self.bits & !Gpio17::OD_MASK
            }
        }
    }

    /// Set **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn with_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio17::IE_MASK
            } else {
                self.bits & !Gpio17::IE_MASK
            }
        }
    }

    /// Set **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn with_drive(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Gpio17::DRIVE_MASK) | ((value << Gpio17::DRIVE_OFFSET) & Gpio17::DRIVE_MASK)
        }
    }

    /// Set **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn with_pue(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio17::PUE_MASK
            } else {
                self.bits & !Gpio17::PUE_MASK
            }
        }
    }

    /// Set **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn with_pde(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio17::PDE_MASK
            } else {
                self.bits & !Gpio17::PDE_MASK
            }
        }
    }

    /// Set **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn with_schmitt(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio17::SCHMITT_MASK
            } else {
                self.bits & !Gpio17::SCHMITT_MASK
            }
        }
    }

    /// Set **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn with_slewfast(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio17::SLEWFAST_MASK
            } else {
                self.bits & !Gpio17::SLEWFAST_MASK
            }
        }
    }

}

impl Default for Gpio17Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Gpio17 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Gpio17Val {
        Gpio17Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Gpio17Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Gpio17Val) -> Gpio17Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// GPIO18 Register
///
/// Address offset: 0x004c
/// Reset value: 0x00000116
///
/// ## Fields
///
/// - **ISO** `[8]` (RW)
///
///   Pad isolation control. Remove this once the pad is configured by software.
///
/// - **OD** `[7]` (RW)
///
///   Output disable. Has priority over output enable from peripherals
///
/// - **IE** `[6]` (RW)
///
///   Input enable
///
/// - **DRIVE** `[5:4]` (RW)
///
///   Drive strength.
///
/// - **PUE** `[3]` (RW)
///
///   Pull up enable
///
/// - **PDE** `[2]` (RW)
///
///   Pull down enable
///
/// - **SCHMITT** `[1]` (RW)
///
///   Enable schmitt trigger
///
/// - **SLEWFAST** `[0]` (RW)
///
///   Slew rate control. 1 = Fast, 0 = Slow
#[repr(transparent)]
pub struct Gpio18 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Gpio18 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Gpio18 {
    // ======== Field Constants ========

    /// **ISO**
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ISO_OFFSET: u32 = 8;
    pub const ISO_MASK: u32 = 0x00000100;
    pub const ISO: u32 = 1 << 8;

    /// **OD**
    ///
    /// Output disable. Has priority over output enable from peripherals
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const OD_OFFSET: u32 = 7;
    pub const OD_MASK: u32 = 0x00000080;
    pub const OD: u32 = 1 << 7;

    /// **IE**
    ///
    /// Input enable
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const IE_OFFSET: u32 = 6;
    pub const IE_MASK: u32 = 0x00000040;
    pub const IE: u32 = 1 << 6;

    /// **DRIVE**
    ///
    /// Drive strength.
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const DRIVE_OFFSET: u32 = 4;
    pub const DRIVE_MASK: u32 = 0x00000030;
    pub const DRIVE_2M_A: u32 = 0 << 4;
    pub const DRIVE_2M_A_VAL: u32 = 0;
    pub const DRIVE_4M_A: u32 = 1 << 4;
    pub const DRIVE_4M_A_VAL: u32 = 1;
    pub const DRIVE_8M_A: u32 = 2 << 4;
    pub const DRIVE_8M_A_VAL: u32 = 2;
    pub const DRIVE_12M_A: u32 = 3 << 4;
    pub const DRIVE_12M_A_VAL: u32 = 3;

    /// **PUE**
    ///
    /// Pull up enable
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const PUE_OFFSET: u32 = 3;
    pub const PUE_MASK: u32 = 0x00000008;
    pub const PUE: u32 = 1 << 3;

    /// **PDE**
    ///
    /// Pull down enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const PDE_OFFSET: u32 = 2;
    pub const PDE_MASK: u32 = 0x00000004;
    pub const PDE: u32 = 1 << 2;

    /// **SCHMITT**
    ///
    /// Enable schmitt trigger
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SCHMITT_OFFSET: u32 = 1;
    pub const SCHMITT_MASK: u32 = 0x00000002;
    pub const SCHMITT: u32 = 1 << 1;

    /// **SLEWFAST**
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SLEWFAST_OFFSET: u32 = 0;
    pub const SLEWFAST_MASK: u32 = 0x00000001;
    pub const SLEWFAST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ISO** `[8]` (RW)
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub fn iso(&self) -> bool {
        (self.value.read_bits() & Self::ISO_MASK) != 0
    }

    /// Set ISO bit atomically (RW)
    #[inline(always)]
    pub fn set_iso(&self) {
        self.set_bits(Self::ISO_MASK);
    }

    /// Clear ISO bit atomically (RW)
    #[inline(always)]
    pub fn clr_iso(&self) {
        self.clr_bits(Self::ISO_MASK);
    }

    /// **OD** `[7]` (RW)
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub fn od(&self) -> bool {
        (self.value.read_bits() & Self::OD_MASK) != 0
    }

    /// Set OD bit atomically (RW)
    #[inline(always)]
    pub fn set_od(&self) {
        self.set_bits(Self::OD_MASK);
    }

    /// Clear OD bit atomically (RW)
    #[inline(always)]
    pub fn clr_od(&self) {
        self.clr_bits(Self::OD_MASK);
    }

    /// **IE** `[6]` (RW)
    ///
    /// Input enable
    #[inline(always)]
    pub fn ie(&self) -> bool {
        (self.value.read_bits() & Self::IE_MASK) != 0
    }

    /// Set IE bit atomically (RW)
    #[inline(always)]
    pub fn set_ie(&self) {
        self.set_bits(Self::IE_MASK);
    }

    /// Clear IE bit atomically (RW)
    #[inline(always)]
    pub fn clr_ie(&self) {
        self.clr_bits(Self::IE_MASK);
    }

    /// **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn drive(&self) -> u32 {
        (self.value.read_bits() & Self::DRIVE_MASK) >> Self::DRIVE_OFFSET
    }

    /// Write **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn write_drive(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DRIVE_MASK) | ((value << Self::DRIVE_OFFSET) & Self::DRIVE_MASK));
    }

    /// **PUE** `[3]` (RW)
    ///
    /// Pull up enable
    #[inline(always)]
    pub fn pue(&self) -> bool {
        (self.value.read_bits() & Self::PUE_MASK) != 0
    }

    /// Set PUE bit atomically (RW)
    #[inline(always)]
    pub fn set_pue(&self) {
        self.set_bits(Self::PUE_MASK);
    }

    /// Clear PUE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pue(&self) {
        self.clr_bits(Self::PUE_MASK);
    }

    /// **PDE** `[2]` (RW)
    ///
    /// Pull down enable
    #[inline(always)]
    pub fn pde(&self) -> bool {
        (self.value.read_bits() & Self::PDE_MASK) != 0
    }

    /// Set PDE bit atomically (RW)
    #[inline(always)]
    pub fn set_pde(&self) {
        self.set_bits(Self::PDE_MASK);
    }

    /// Clear PDE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pde(&self) {
        self.clr_bits(Self::PDE_MASK);
    }

    /// **SCHMITT** `[1]` (RW)
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub fn schmitt(&self) -> bool {
        (self.value.read_bits() & Self::SCHMITT_MASK) != 0
    }

    /// Set SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn set_schmitt(&self) {
        self.set_bits(Self::SCHMITT_MASK);
    }

    /// Clear SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn clr_schmitt(&self) {
        self.clr_bits(Self::SCHMITT_MASK);
    }

    /// **SLEWFAST** `[0]` (RW)
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub fn slewfast(&self) -> bool {
        (self.value.read_bits() & Self::SLEWFAST_MASK) != 0
    }

    /// Set SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn set_slewfast(&self) {
        self.set_bits(Self::SLEWFAST_MASK);
    }

    /// Clear SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn clr_slewfast(&self) {
        self.clr_bits(Self::SLEWFAST_MASK);
    }

}

/// GPIO18 value type for batch read/modify/write operations
///
/// Use `gpio18.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpio18.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Gpio18Val {
    bits: u32,
}

impl Gpio18Val {
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
        Self { bits: 0x00000116 }
    }

    /// Read **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn iso(&self) -> bool {
        (self.bits & Gpio18::ISO_MASK) != 0
    }

    /// Read **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn od(&self) -> bool {
        (self.bits & Gpio18::OD_MASK) != 0
    }

    /// Read **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        (self.bits & Gpio18::IE_MASK) != 0
    }

    /// Read **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn drive(&self) -> u32 {
        (self.bits & Gpio18::DRIVE_MASK) >> Gpio18::DRIVE_OFFSET
    }

    /// Read **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn pue(&self) -> bool {
        (self.bits & Gpio18::PUE_MASK) != 0
    }

    /// Read **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn pde(&self) -> bool {
        (self.bits & Gpio18::PDE_MASK) != 0
    }

    /// Read **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn schmitt(&self) -> bool {
        (self.bits & Gpio18::SCHMITT_MASK) != 0
    }

    /// Read **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn slewfast(&self) -> bool {
        (self.bits & Gpio18::SLEWFAST_MASK) != 0
    }

    /// Set **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn with_iso(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio18::ISO_MASK
            } else {
                self.bits & !Gpio18::ISO_MASK
            }
        }
    }

    /// Set **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn with_od(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio18::OD_MASK
            } else {
                self.bits & !Gpio18::OD_MASK
            }
        }
    }

    /// Set **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn with_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio18::IE_MASK
            } else {
                self.bits & !Gpio18::IE_MASK
            }
        }
    }

    /// Set **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn with_drive(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Gpio18::DRIVE_MASK) | ((value << Gpio18::DRIVE_OFFSET) & Gpio18::DRIVE_MASK)
        }
    }

    /// Set **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn with_pue(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio18::PUE_MASK
            } else {
                self.bits & !Gpio18::PUE_MASK
            }
        }
    }

    /// Set **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn with_pde(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio18::PDE_MASK
            } else {
                self.bits & !Gpio18::PDE_MASK
            }
        }
    }

    /// Set **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn with_schmitt(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio18::SCHMITT_MASK
            } else {
                self.bits & !Gpio18::SCHMITT_MASK
            }
        }
    }

    /// Set **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn with_slewfast(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio18::SLEWFAST_MASK
            } else {
                self.bits & !Gpio18::SLEWFAST_MASK
            }
        }
    }

}

impl Default for Gpio18Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Gpio18 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Gpio18Val {
        Gpio18Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Gpio18Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Gpio18Val) -> Gpio18Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// GPIO19 Register
///
/// Address offset: 0x0050
/// Reset value: 0x00000116
///
/// ## Fields
///
/// - **ISO** `[8]` (RW)
///
///   Pad isolation control. Remove this once the pad is configured by software.
///
/// - **OD** `[7]` (RW)
///
///   Output disable. Has priority over output enable from peripherals
///
/// - **IE** `[6]` (RW)
///
///   Input enable
///
/// - **DRIVE** `[5:4]` (RW)
///
///   Drive strength.
///
/// - **PUE** `[3]` (RW)
///
///   Pull up enable
///
/// - **PDE** `[2]` (RW)
///
///   Pull down enable
///
/// - **SCHMITT** `[1]` (RW)
///
///   Enable schmitt trigger
///
/// - **SLEWFAST** `[0]` (RW)
///
///   Slew rate control. 1 = Fast, 0 = Slow
#[repr(transparent)]
pub struct Gpio19 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Gpio19 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Gpio19 {
    // ======== Field Constants ========

    /// **ISO**
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ISO_OFFSET: u32 = 8;
    pub const ISO_MASK: u32 = 0x00000100;
    pub const ISO: u32 = 1 << 8;

    /// **OD**
    ///
    /// Output disable. Has priority over output enable from peripherals
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const OD_OFFSET: u32 = 7;
    pub const OD_MASK: u32 = 0x00000080;
    pub const OD: u32 = 1 << 7;

    /// **IE**
    ///
    /// Input enable
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const IE_OFFSET: u32 = 6;
    pub const IE_MASK: u32 = 0x00000040;
    pub const IE: u32 = 1 << 6;

    /// **DRIVE**
    ///
    /// Drive strength.
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const DRIVE_OFFSET: u32 = 4;
    pub const DRIVE_MASK: u32 = 0x00000030;
    pub const DRIVE_2M_A: u32 = 0 << 4;
    pub const DRIVE_2M_A_VAL: u32 = 0;
    pub const DRIVE_4M_A: u32 = 1 << 4;
    pub const DRIVE_4M_A_VAL: u32 = 1;
    pub const DRIVE_8M_A: u32 = 2 << 4;
    pub const DRIVE_8M_A_VAL: u32 = 2;
    pub const DRIVE_12M_A: u32 = 3 << 4;
    pub const DRIVE_12M_A_VAL: u32 = 3;

    /// **PUE**
    ///
    /// Pull up enable
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const PUE_OFFSET: u32 = 3;
    pub const PUE_MASK: u32 = 0x00000008;
    pub const PUE: u32 = 1 << 3;

    /// **PDE**
    ///
    /// Pull down enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const PDE_OFFSET: u32 = 2;
    pub const PDE_MASK: u32 = 0x00000004;
    pub const PDE: u32 = 1 << 2;

    /// **SCHMITT**
    ///
    /// Enable schmitt trigger
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SCHMITT_OFFSET: u32 = 1;
    pub const SCHMITT_MASK: u32 = 0x00000002;
    pub const SCHMITT: u32 = 1 << 1;

    /// **SLEWFAST**
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SLEWFAST_OFFSET: u32 = 0;
    pub const SLEWFAST_MASK: u32 = 0x00000001;
    pub const SLEWFAST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ISO** `[8]` (RW)
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub fn iso(&self) -> bool {
        (self.value.read_bits() & Self::ISO_MASK) != 0
    }

    /// Set ISO bit atomically (RW)
    #[inline(always)]
    pub fn set_iso(&self) {
        self.set_bits(Self::ISO_MASK);
    }

    /// Clear ISO bit atomically (RW)
    #[inline(always)]
    pub fn clr_iso(&self) {
        self.clr_bits(Self::ISO_MASK);
    }

    /// **OD** `[7]` (RW)
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub fn od(&self) -> bool {
        (self.value.read_bits() & Self::OD_MASK) != 0
    }

    /// Set OD bit atomically (RW)
    #[inline(always)]
    pub fn set_od(&self) {
        self.set_bits(Self::OD_MASK);
    }

    /// Clear OD bit atomically (RW)
    #[inline(always)]
    pub fn clr_od(&self) {
        self.clr_bits(Self::OD_MASK);
    }

    /// **IE** `[6]` (RW)
    ///
    /// Input enable
    #[inline(always)]
    pub fn ie(&self) -> bool {
        (self.value.read_bits() & Self::IE_MASK) != 0
    }

    /// Set IE bit atomically (RW)
    #[inline(always)]
    pub fn set_ie(&self) {
        self.set_bits(Self::IE_MASK);
    }

    /// Clear IE bit atomically (RW)
    #[inline(always)]
    pub fn clr_ie(&self) {
        self.clr_bits(Self::IE_MASK);
    }

    /// **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn drive(&self) -> u32 {
        (self.value.read_bits() & Self::DRIVE_MASK) >> Self::DRIVE_OFFSET
    }

    /// Write **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn write_drive(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DRIVE_MASK) | ((value << Self::DRIVE_OFFSET) & Self::DRIVE_MASK));
    }

    /// **PUE** `[3]` (RW)
    ///
    /// Pull up enable
    #[inline(always)]
    pub fn pue(&self) -> bool {
        (self.value.read_bits() & Self::PUE_MASK) != 0
    }

    /// Set PUE bit atomically (RW)
    #[inline(always)]
    pub fn set_pue(&self) {
        self.set_bits(Self::PUE_MASK);
    }

    /// Clear PUE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pue(&self) {
        self.clr_bits(Self::PUE_MASK);
    }

    /// **PDE** `[2]` (RW)
    ///
    /// Pull down enable
    #[inline(always)]
    pub fn pde(&self) -> bool {
        (self.value.read_bits() & Self::PDE_MASK) != 0
    }

    /// Set PDE bit atomically (RW)
    #[inline(always)]
    pub fn set_pde(&self) {
        self.set_bits(Self::PDE_MASK);
    }

    /// Clear PDE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pde(&self) {
        self.clr_bits(Self::PDE_MASK);
    }

    /// **SCHMITT** `[1]` (RW)
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub fn schmitt(&self) -> bool {
        (self.value.read_bits() & Self::SCHMITT_MASK) != 0
    }

    /// Set SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn set_schmitt(&self) {
        self.set_bits(Self::SCHMITT_MASK);
    }

    /// Clear SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn clr_schmitt(&self) {
        self.clr_bits(Self::SCHMITT_MASK);
    }

    /// **SLEWFAST** `[0]` (RW)
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub fn slewfast(&self) -> bool {
        (self.value.read_bits() & Self::SLEWFAST_MASK) != 0
    }

    /// Set SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn set_slewfast(&self) {
        self.set_bits(Self::SLEWFAST_MASK);
    }

    /// Clear SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn clr_slewfast(&self) {
        self.clr_bits(Self::SLEWFAST_MASK);
    }

}

/// GPIO19 value type for batch read/modify/write operations
///
/// Use `gpio19.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpio19.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Gpio19Val {
    bits: u32,
}

impl Gpio19Val {
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
        Self { bits: 0x00000116 }
    }

    /// Read **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn iso(&self) -> bool {
        (self.bits & Gpio19::ISO_MASK) != 0
    }

    /// Read **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn od(&self) -> bool {
        (self.bits & Gpio19::OD_MASK) != 0
    }

    /// Read **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        (self.bits & Gpio19::IE_MASK) != 0
    }

    /// Read **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn drive(&self) -> u32 {
        (self.bits & Gpio19::DRIVE_MASK) >> Gpio19::DRIVE_OFFSET
    }

    /// Read **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn pue(&self) -> bool {
        (self.bits & Gpio19::PUE_MASK) != 0
    }

    /// Read **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn pde(&self) -> bool {
        (self.bits & Gpio19::PDE_MASK) != 0
    }

    /// Read **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn schmitt(&self) -> bool {
        (self.bits & Gpio19::SCHMITT_MASK) != 0
    }

    /// Read **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn slewfast(&self) -> bool {
        (self.bits & Gpio19::SLEWFAST_MASK) != 0
    }

    /// Set **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn with_iso(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio19::ISO_MASK
            } else {
                self.bits & !Gpio19::ISO_MASK
            }
        }
    }

    /// Set **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn with_od(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio19::OD_MASK
            } else {
                self.bits & !Gpio19::OD_MASK
            }
        }
    }

    /// Set **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn with_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio19::IE_MASK
            } else {
                self.bits & !Gpio19::IE_MASK
            }
        }
    }

    /// Set **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn with_drive(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Gpio19::DRIVE_MASK) | ((value << Gpio19::DRIVE_OFFSET) & Gpio19::DRIVE_MASK)
        }
    }

    /// Set **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn with_pue(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio19::PUE_MASK
            } else {
                self.bits & !Gpio19::PUE_MASK
            }
        }
    }

    /// Set **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn with_pde(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio19::PDE_MASK
            } else {
                self.bits & !Gpio19::PDE_MASK
            }
        }
    }

    /// Set **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn with_schmitt(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio19::SCHMITT_MASK
            } else {
                self.bits & !Gpio19::SCHMITT_MASK
            }
        }
    }

    /// Set **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn with_slewfast(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio19::SLEWFAST_MASK
            } else {
                self.bits & !Gpio19::SLEWFAST_MASK
            }
        }
    }

}

impl Default for Gpio19Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Gpio19 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Gpio19Val {
        Gpio19Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Gpio19Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Gpio19Val) -> Gpio19Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// GPIO20 Register
///
/// Address offset: 0x0054
/// Reset value: 0x00000116
///
/// ## Fields
///
/// - **ISO** `[8]` (RW)
///
///   Pad isolation control. Remove this once the pad is configured by software.
///
/// - **OD** `[7]` (RW)
///
///   Output disable. Has priority over output enable from peripherals
///
/// - **IE** `[6]` (RW)
///
///   Input enable
///
/// - **DRIVE** `[5:4]` (RW)
///
///   Drive strength.
///
/// - **PUE** `[3]` (RW)
///
///   Pull up enable
///
/// - **PDE** `[2]` (RW)
///
///   Pull down enable
///
/// - **SCHMITT** `[1]` (RW)
///
///   Enable schmitt trigger
///
/// - **SLEWFAST** `[0]` (RW)
///
///   Slew rate control. 1 = Fast, 0 = Slow
#[repr(transparent)]
pub struct Gpio20 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Gpio20 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Gpio20 {
    // ======== Field Constants ========

    /// **ISO**
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ISO_OFFSET: u32 = 8;
    pub const ISO_MASK: u32 = 0x00000100;
    pub const ISO: u32 = 1 << 8;

    /// **OD**
    ///
    /// Output disable. Has priority over output enable from peripherals
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const OD_OFFSET: u32 = 7;
    pub const OD_MASK: u32 = 0x00000080;
    pub const OD: u32 = 1 << 7;

    /// **IE**
    ///
    /// Input enable
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const IE_OFFSET: u32 = 6;
    pub const IE_MASK: u32 = 0x00000040;
    pub const IE: u32 = 1 << 6;

    /// **DRIVE**
    ///
    /// Drive strength.
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const DRIVE_OFFSET: u32 = 4;
    pub const DRIVE_MASK: u32 = 0x00000030;
    pub const DRIVE_2M_A: u32 = 0 << 4;
    pub const DRIVE_2M_A_VAL: u32 = 0;
    pub const DRIVE_4M_A: u32 = 1 << 4;
    pub const DRIVE_4M_A_VAL: u32 = 1;
    pub const DRIVE_8M_A: u32 = 2 << 4;
    pub const DRIVE_8M_A_VAL: u32 = 2;
    pub const DRIVE_12M_A: u32 = 3 << 4;
    pub const DRIVE_12M_A_VAL: u32 = 3;

    /// **PUE**
    ///
    /// Pull up enable
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const PUE_OFFSET: u32 = 3;
    pub const PUE_MASK: u32 = 0x00000008;
    pub const PUE: u32 = 1 << 3;

    /// **PDE**
    ///
    /// Pull down enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const PDE_OFFSET: u32 = 2;
    pub const PDE_MASK: u32 = 0x00000004;
    pub const PDE: u32 = 1 << 2;

    /// **SCHMITT**
    ///
    /// Enable schmitt trigger
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SCHMITT_OFFSET: u32 = 1;
    pub const SCHMITT_MASK: u32 = 0x00000002;
    pub const SCHMITT: u32 = 1 << 1;

    /// **SLEWFAST**
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SLEWFAST_OFFSET: u32 = 0;
    pub const SLEWFAST_MASK: u32 = 0x00000001;
    pub const SLEWFAST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ISO** `[8]` (RW)
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub fn iso(&self) -> bool {
        (self.value.read_bits() & Self::ISO_MASK) != 0
    }

    /// Set ISO bit atomically (RW)
    #[inline(always)]
    pub fn set_iso(&self) {
        self.set_bits(Self::ISO_MASK);
    }

    /// Clear ISO bit atomically (RW)
    #[inline(always)]
    pub fn clr_iso(&self) {
        self.clr_bits(Self::ISO_MASK);
    }

    /// **OD** `[7]` (RW)
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub fn od(&self) -> bool {
        (self.value.read_bits() & Self::OD_MASK) != 0
    }

    /// Set OD bit atomically (RW)
    #[inline(always)]
    pub fn set_od(&self) {
        self.set_bits(Self::OD_MASK);
    }

    /// Clear OD bit atomically (RW)
    #[inline(always)]
    pub fn clr_od(&self) {
        self.clr_bits(Self::OD_MASK);
    }

    /// **IE** `[6]` (RW)
    ///
    /// Input enable
    #[inline(always)]
    pub fn ie(&self) -> bool {
        (self.value.read_bits() & Self::IE_MASK) != 0
    }

    /// Set IE bit atomically (RW)
    #[inline(always)]
    pub fn set_ie(&self) {
        self.set_bits(Self::IE_MASK);
    }

    /// Clear IE bit atomically (RW)
    #[inline(always)]
    pub fn clr_ie(&self) {
        self.clr_bits(Self::IE_MASK);
    }

    /// **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn drive(&self) -> u32 {
        (self.value.read_bits() & Self::DRIVE_MASK) >> Self::DRIVE_OFFSET
    }

    /// Write **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn write_drive(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DRIVE_MASK) | ((value << Self::DRIVE_OFFSET) & Self::DRIVE_MASK));
    }

    /// **PUE** `[3]` (RW)
    ///
    /// Pull up enable
    #[inline(always)]
    pub fn pue(&self) -> bool {
        (self.value.read_bits() & Self::PUE_MASK) != 0
    }

    /// Set PUE bit atomically (RW)
    #[inline(always)]
    pub fn set_pue(&self) {
        self.set_bits(Self::PUE_MASK);
    }

    /// Clear PUE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pue(&self) {
        self.clr_bits(Self::PUE_MASK);
    }

    /// **PDE** `[2]` (RW)
    ///
    /// Pull down enable
    #[inline(always)]
    pub fn pde(&self) -> bool {
        (self.value.read_bits() & Self::PDE_MASK) != 0
    }

    /// Set PDE bit atomically (RW)
    #[inline(always)]
    pub fn set_pde(&self) {
        self.set_bits(Self::PDE_MASK);
    }

    /// Clear PDE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pde(&self) {
        self.clr_bits(Self::PDE_MASK);
    }

    /// **SCHMITT** `[1]` (RW)
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub fn schmitt(&self) -> bool {
        (self.value.read_bits() & Self::SCHMITT_MASK) != 0
    }

    /// Set SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn set_schmitt(&self) {
        self.set_bits(Self::SCHMITT_MASK);
    }

    /// Clear SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn clr_schmitt(&self) {
        self.clr_bits(Self::SCHMITT_MASK);
    }

    /// **SLEWFAST** `[0]` (RW)
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub fn slewfast(&self) -> bool {
        (self.value.read_bits() & Self::SLEWFAST_MASK) != 0
    }

    /// Set SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn set_slewfast(&self) {
        self.set_bits(Self::SLEWFAST_MASK);
    }

    /// Clear SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn clr_slewfast(&self) {
        self.clr_bits(Self::SLEWFAST_MASK);
    }

}

/// GPIO20 value type for batch read/modify/write operations
///
/// Use `gpio20.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpio20.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Gpio20Val {
    bits: u32,
}

impl Gpio20Val {
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
        Self { bits: 0x00000116 }
    }

    /// Read **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn iso(&self) -> bool {
        (self.bits & Gpio20::ISO_MASK) != 0
    }

    /// Read **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn od(&self) -> bool {
        (self.bits & Gpio20::OD_MASK) != 0
    }

    /// Read **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        (self.bits & Gpio20::IE_MASK) != 0
    }

    /// Read **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn drive(&self) -> u32 {
        (self.bits & Gpio20::DRIVE_MASK) >> Gpio20::DRIVE_OFFSET
    }

    /// Read **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn pue(&self) -> bool {
        (self.bits & Gpio20::PUE_MASK) != 0
    }

    /// Read **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn pde(&self) -> bool {
        (self.bits & Gpio20::PDE_MASK) != 0
    }

    /// Read **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn schmitt(&self) -> bool {
        (self.bits & Gpio20::SCHMITT_MASK) != 0
    }

    /// Read **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn slewfast(&self) -> bool {
        (self.bits & Gpio20::SLEWFAST_MASK) != 0
    }

    /// Set **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn with_iso(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio20::ISO_MASK
            } else {
                self.bits & !Gpio20::ISO_MASK
            }
        }
    }

    /// Set **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn with_od(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio20::OD_MASK
            } else {
                self.bits & !Gpio20::OD_MASK
            }
        }
    }

    /// Set **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn with_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio20::IE_MASK
            } else {
                self.bits & !Gpio20::IE_MASK
            }
        }
    }

    /// Set **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn with_drive(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Gpio20::DRIVE_MASK) | ((value << Gpio20::DRIVE_OFFSET) & Gpio20::DRIVE_MASK)
        }
    }

    /// Set **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn with_pue(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio20::PUE_MASK
            } else {
                self.bits & !Gpio20::PUE_MASK
            }
        }
    }

    /// Set **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn with_pde(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio20::PDE_MASK
            } else {
                self.bits & !Gpio20::PDE_MASK
            }
        }
    }

    /// Set **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn with_schmitt(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio20::SCHMITT_MASK
            } else {
                self.bits & !Gpio20::SCHMITT_MASK
            }
        }
    }

    /// Set **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn with_slewfast(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio20::SLEWFAST_MASK
            } else {
                self.bits & !Gpio20::SLEWFAST_MASK
            }
        }
    }

}

impl Default for Gpio20Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Gpio20 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Gpio20Val {
        Gpio20Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Gpio20Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Gpio20Val) -> Gpio20Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// GPIO21 Register
///
/// Address offset: 0x0058
/// Reset value: 0x00000116
///
/// ## Fields
///
/// - **ISO** `[8]` (RW)
///
///   Pad isolation control. Remove this once the pad is configured by software.
///
/// - **OD** `[7]` (RW)
///
///   Output disable. Has priority over output enable from peripherals
///
/// - **IE** `[6]` (RW)
///
///   Input enable
///
/// - **DRIVE** `[5:4]` (RW)
///
///   Drive strength.
///
/// - **PUE** `[3]` (RW)
///
///   Pull up enable
///
/// - **PDE** `[2]` (RW)
///
///   Pull down enable
///
/// - **SCHMITT** `[1]` (RW)
///
///   Enable schmitt trigger
///
/// - **SLEWFAST** `[0]` (RW)
///
///   Slew rate control. 1 = Fast, 0 = Slow
#[repr(transparent)]
pub struct Gpio21 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Gpio21 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Gpio21 {
    // ======== Field Constants ========

    /// **ISO**
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ISO_OFFSET: u32 = 8;
    pub const ISO_MASK: u32 = 0x00000100;
    pub const ISO: u32 = 1 << 8;

    /// **OD**
    ///
    /// Output disable. Has priority over output enable from peripherals
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const OD_OFFSET: u32 = 7;
    pub const OD_MASK: u32 = 0x00000080;
    pub const OD: u32 = 1 << 7;

    /// **IE**
    ///
    /// Input enable
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const IE_OFFSET: u32 = 6;
    pub const IE_MASK: u32 = 0x00000040;
    pub const IE: u32 = 1 << 6;

    /// **DRIVE**
    ///
    /// Drive strength.
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const DRIVE_OFFSET: u32 = 4;
    pub const DRIVE_MASK: u32 = 0x00000030;
    pub const DRIVE_2M_A: u32 = 0 << 4;
    pub const DRIVE_2M_A_VAL: u32 = 0;
    pub const DRIVE_4M_A: u32 = 1 << 4;
    pub const DRIVE_4M_A_VAL: u32 = 1;
    pub const DRIVE_8M_A: u32 = 2 << 4;
    pub const DRIVE_8M_A_VAL: u32 = 2;
    pub const DRIVE_12M_A: u32 = 3 << 4;
    pub const DRIVE_12M_A_VAL: u32 = 3;

    /// **PUE**
    ///
    /// Pull up enable
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const PUE_OFFSET: u32 = 3;
    pub const PUE_MASK: u32 = 0x00000008;
    pub const PUE: u32 = 1 << 3;

    /// **PDE**
    ///
    /// Pull down enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const PDE_OFFSET: u32 = 2;
    pub const PDE_MASK: u32 = 0x00000004;
    pub const PDE: u32 = 1 << 2;

    /// **SCHMITT**
    ///
    /// Enable schmitt trigger
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SCHMITT_OFFSET: u32 = 1;
    pub const SCHMITT_MASK: u32 = 0x00000002;
    pub const SCHMITT: u32 = 1 << 1;

    /// **SLEWFAST**
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SLEWFAST_OFFSET: u32 = 0;
    pub const SLEWFAST_MASK: u32 = 0x00000001;
    pub const SLEWFAST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ISO** `[8]` (RW)
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub fn iso(&self) -> bool {
        (self.value.read_bits() & Self::ISO_MASK) != 0
    }

    /// Set ISO bit atomically (RW)
    #[inline(always)]
    pub fn set_iso(&self) {
        self.set_bits(Self::ISO_MASK);
    }

    /// Clear ISO bit atomically (RW)
    #[inline(always)]
    pub fn clr_iso(&self) {
        self.clr_bits(Self::ISO_MASK);
    }

    /// **OD** `[7]` (RW)
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub fn od(&self) -> bool {
        (self.value.read_bits() & Self::OD_MASK) != 0
    }

    /// Set OD bit atomically (RW)
    #[inline(always)]
    pub fn set_od(&self) {
        self.set_bits(Self::OD_MASK);
    }

    /// Clear OD bit atomically (RW)
    #[inline(always)]
    pub fn clr_od(&self) {
        self.clr_bits(Self::OD_MASK);
    }

    /// **IE** `[6]` (RW)
    ///
    /// Input enable
    #[inline(always)]
    pub fn ie(&self) -> bool {
        (self.value.read_bits() & Self::IE_MASK) != 0
    }

    /// Set IE bit atomically (RW)
    #[inline(always)]
    pub fn set_ie(&self) {
        self.set_bits(Self::IE_MASK);
    }

    /// Clear IE bit atomically (RW)
    #[inline(always)]
    pub fn clr_ie(&self) {
        self.clr_bits(Self::IE_MASK);
    }

    /// **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn drive(&self) -> u32 {
        (self.value.read_bits() & Self::DRIVE_MASK) >> Self::DRIVE_OFFSET
    }

    /// Write **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn write_drive(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DRIVE_MASK) | ((value << Self::DRIVE_OFFSET) & Self::DRIVE_MASK));
    }

    /// **PUE** `[3]` (RW)
    ///
    /// Pull up enable
    #[inline(always)]
    pub fn pue(&self) -> bool {
        (self.value.read_bits() & Self::PUE_MASK) != 0
    }

    /// Set PUE bit atomically (RW)
    #[inline(always)]
    pub fn set_pue(&self) {
        self.set_bits(Self::PUE_MASK);
    }

    /// Clear PUE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pue(&self) {
        self.clr_bits(Self::PUE_MASK);
    }

    /// **PDE** `[2]` (RW)
    ///
    /// Pull down enable
    #[inline(always)]
    pub fn pde(&self) -> bool {
        (self.value.read_bits() & Self::PDE_MASK) != 0
    }

    /// Set PDE bit atomically (RW)
    #[inline(always)]
    pub fn set_pde(&self) {
        self.set_bits(Self::PDE_MASK);
    }

    /// Clear PDE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pde(&self) {
        self.clr_bits(Self::PDE_MASK);
    }

    /// **SCHMITT** `[1]` (RW)
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub fn schmitt(&self) -> bool {
        (self.value.read_bits() & Self::SCHMITT_MASK) != 0
    }

    /// Set SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn set_schmitt(&self) {
        self.set_bits(Self::SCHMITT_MASK);
    }

    /// Clear SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn clr_schmitt(&self) {
        self.clr_bits(Self::SCHMITT_MASK);
    }

    /// **SLEWFAST** `[0]` (RW)
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub fn slewfast(&self) -> bool {
        (self.value.read_bits() & Self::SLEWFAST_MASK) != 0
    }

    /// Set SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn set_slewfast(&self) {
        self.set_bits(Self::SLEWFAST_MASK);
    }

    /// Clear SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn clr_slewfast(&self) {
        self.clr_bits(Self::SLEWFAST_MASK);
    }

}

/// GPIO21 value type for batch read/modify/write operations
///
/// Use `gpio21.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpio21.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Gpio21Val {
    bits: u32,
}

impl Gpio21Val {
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
        Self { bits: 0x00000116 }
    }

    /// Read **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn iso(&self) -> bool {
        (self.bits & Gpio21::ISO_MASK) != 0
    }

    /// Read **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn od(&self) -> bool {
        (self.bits & Gpio21::OD_MASK) != 0
    }

    /// Read **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        (self.bits & Gpio21::IE_MASK) != 0
    }

    /// Read **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn drive(&self) -> u32 {
        (self.bits & Gpio21::DRIVE_MASK) >> Gpio21::DRIVE_OFFSET
    }

    /// Read **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn pue(&self) -> bool {
        (self.bits & Gpio21::PUE_MASK) != 0
    }

    /// Read **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn pde(&self) -> bool {
        (self.bits & Gpio21::PDE_MASK) != 0
    }

    /// Read **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn schmitt(&self) -> bool {
        (self.bits & Gpio21::SCHMITT_MASK) != 0
    }

    /// Read **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn slewfast(&self) -> bool {
        (self.bits & Gpio21::SLEWFAST_MASK) != 0
    }

    /// Set **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn with_iso(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio21::ISO_MASK
            } else {
                self.bits & !Gpio21::ISO_MASK
            }
        }
    }

    /// Set **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn with_od(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio21::OD_MASK
            } else {
                self.bits & !Gpio21::OD_MASK
            }
        }
    }

    /// Set **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn with_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio21::IE_MASK
            } else {
                self.bits & !Gpio21::IE_MASK
            }
        }
    }

    /// Set **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn with_drive(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Gpio21::DRIVE_MASK) | ((value << Gpio21::DRIVE_OFFSET) & Gpio21::DRIVE_MASK)
        }
    }

    /// Set **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn with_pue(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio21::PUE_MASK
            } else {
                self.bits & !Gpio21::PUE_MASK
            }
        }
    }

    /// Set **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn with_pde(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio21::PDE_MASK
            } else {
                self.bits & !Gpio21::PDE_MASK
            }
        }
    }

    /// Set **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn with_schmitt(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio21::SCHMITT_MASK
            } else {
                self.bits & !Gpio21::SCHMITT_MASK
            }
        }
    }

    /// Set **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn with_slewfast(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio21::SLEWFAST_MASK
            } else {
                self.bits & !Gpio21::SLEWFAST_MASK
            }
        }
    }

}

impl Default for Gpio21Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Gpio21 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Gpio21Val {
        Gpio21Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Gpio21Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Gpio21Val) -> Gpio21Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// GPIO22 Register
///
/// Address offset: 0x005c
/// Reset value: 0x00000116
///
/// ## Fields
///
/// - **ISO** `[8]` (RW)
///
///   Pad isolation control. Remove this once the pad is configured by software.
///
/// - **OD** `[7]` (RW)
///
///   Output disable. Has priority over output enable from peripherals
///
/// - **IE** `[6]` (RW)
///
///   Input enable
///
/// - **DRIVE** `[5:4]` (RW)
///
///   Drive strength.
///
/// - **PUE** `[3]` (RW)
///
///   Pull up enable
///
/// - **PDE** `[2]` (RW)
///
///   Pull down enable
///
/// - **SCHMITT** `[1]` (RW)
///
///   Enable schmitt trigger
///
/// - **SLEWFAST** `[0]` (RW)
///
///   Slew rate control. 1 = Fast, 0 = Slow
#[repr(transparent)]
pub struct Gpio22 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Gpio22 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Gpio22 {
    // ======== Field Constants ========

    /// **ISO**
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ISO_OFFSET: u32 = 8;
    pub const ISO_MASK: u32 = 0x00000100;
    pub const ISO: u32 = 1 << 8;

    /// **OD**
    ///
    /// Output disable. Has priority over output enable from peripherals
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const OD_OFFSET: u32 = 7;
    pub const OD_MASK: u32 = 0x00000080;
    pub const OD: u32 = 1 << 7;

    /// **IE**
    ///
    /// Input enable
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const IE_OFFSET: u32 = 6;
    pub const IE_MASK: u32 = 0x00000040;
    pub const IE: u32 = 1 << 6;

    /// **DRIVE**
    ///
    /// Drive strength.
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const DRIVE_OFFSET: u32 = 4;
    pub const DRIVE_MASK: u32 = 0x00000030;
    pub const DRIVE_2M_A: u32 = 0 << 4;
    pub const DRIVE_2M_A_VAL: u32 = 0;
    pub const DRIVE_4M_A: u32 = 1 << 4;
    pub const DRIVE_4M_A_VAL: u32 = 1;
    pub const DRIVE_8M_A: u32 = 2 << 4;
    pub const DRIVE_8M_A_VAL: u32 = 2;
    pub const DRIVE_12M_A: u32 = 3 << 4;
    pub const DRIVE_12M_A_VAL: u32 = 3;

    /// **PUE**
    ///
    /// Pull up enable
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const PUE_OFFSET: u32 = 3;
    pub const PUE_MASK: u32 = 0x00000008;
    pub const PUE: u32 = 1 << 3;

    /// **PDE**
    ///
    /// Pull down enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const PDE_OFFSET: u32 = 2;
    pub const PDE_MASK: u32 = 0x00000004;
    pub const PDE: u32 = 1 << 2;

    /// **SCHMITT**
    ///
    /// Enable schmitt trigger
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SCHMITT_OFFSET: u32 = 1;
    pub const SCHMITT_MASK: u32 = 0x00000002;
    pub const SCHMITT: u32 = 1 << 1;

    /// **SLEWFAST**
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SLEWFAST_OFFSET: u32 = 0;
    pub const SLEWFAST_MASK: u32 = 0x00000001;
    pub const SLEWFAST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ISO** `[8]` (RW)
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub fn iso(&self) -> bool {
        (self.value.read_bits() & Self::ISO_MASK) != 0
    }

    /// Set ISO bit atomically (RW)
    #[inline(always)]
    pub fn set_iso(&self) {
        self.set_bits(Self::ISO_MASK);
    }

    /// Clear ISO bit atomically (RW)
    #[inline(always)]
    pub fn clr_iso(&self) {
        self.clr_bits(Self::ISO_MASK);
    }

    /// **OD** `[7]` (RW)
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub fn od(&self) -> bool {
        (self.value.read_bits() & Self::OD_MASK) != 0
    }

    /// Set OD bit atomically (RW)
    #[inline(always)]
    pub fn set_od(&self) {
        self.set_bits(Self::OD_MASK);
    }

    /// Clear OD bit atomically (RW)
    #[inline(always)]
    pub fn clr_od(&self) {
        self.clr_bits(Self::OD_MASK);
    }

    /// **IE** `[6]` (RW)
    ///
    /// Input enable
    #[inline(always)]
    pub fn ie(&self) -> bool {
        (self.value.read_bits() & Self::IE_MASK) != 0
    }

    /// Set IE bit atomically (RW)
    #[inline(always)]
    pub fn set_ie(&self) {
        self.set_bits(Self::IE_MASK);
    }

    /// Clear IE bit atomically (RW)
    #[inline(always)]
    pub fn clr_ie(&self) {
        self.clr_bits(Self::IE_MASK);
    }

    /// **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn drive(&self) -> u32 {
        (self.value.read_bits() & Self::DRIVE_MASK) >> Self::DRIVE_OFFSET
    }

    /// Write **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn write_drive(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DRIVE_MASK) | ((value << Self::DRIVE_OFFSET) & Self::DRIVE_MASK));
    }

    /// **PUE** `[3]` (RW)
    ///
    /// Pull up enable
    #[inline(always)]
    pub fn pue(&self) -> bool {
        (self.value.read_bits() & Self::PUE_MASK) != 0
    }

    /// Set PUE bit atomically (RW)
    #[inline(always)]
    pub fn set_pue(&self) {
        self.set_bits(Self::PUE_MASK);
    }

    /// Clear PUE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pue(&self) {
        self.clr_bits(Self::PUE_MASK);
    }

    /// **PDE** `[2]` (RW)
    ///
    /// Pull down enable
    #[inline(always)]
    pub fn pde(&self) -> bool {
        (self.value.read_bits() & Self::PDE_MASK) != 0
    }

    /// Set PDE bit atomically (RW)
    #[inline(always)]
    pub fn set_pde(&self) {
        self.set_bits(Self::PDE_MASK);
    }

    /// Clear PDE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pde(&self) {
        self.clr_bits(Self::PDE_MASK);
    }

    /// **SCHMITT** `[1]` (RW)
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub fn schmitt(&self) -> bool {
        (self.value.read_bits() & Self::SCHMITT_MASK) != 0
    }

    /// Set SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn set_schmitt(&self) {
        self.set_bits(Self::SCHMITT_MASK);
    }

    /// Clear SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn clr_schmitt(&self) {
        self.clr_bits(Self::SCHMITT_MASK);
    }

    /// **SLEWFAST** `[0]` (RW)
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub fn slewfast(&self) -> bool {
        (self.value.read_bits() & Self::SLEWFAST_MASK) != 0
    }

    /// Set SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn set_slewfast(&self) {
        self.set_bits(Self::SLEWFAST_MASK);
    }

    /// Clear SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn clr_slewfast(&self) {
        self.clr_bits(Self::SLEWFAST_MASK);
    }

}

/// GPIO22 value type for batch read/modify/write operations
///
/// Use `gpio22.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpio22.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Gpio22Val {
    bits: u32,
}

impl Gpio22Val {
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
        Self { bits: 0x00000116 }
    }

    /// Read **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn iso(&self) -> bool {
        (self.bits & Gpio22::ISO_MASK) != 0
    }

    /// Read **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn od(&self) -> bool {
        (self.bits & Gpio22::OD_MASK) != 0
    }

    /// Read **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        (self.bits & Gpio22::IE_MASK) != 0
    }

    /// Read **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn drive(&self) -> u32 {
        (self.bits & Gpio22::DRIVE_MASK) >> Gpio22::DRIVE_OFFSET
    }

    /// Read **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn pue(&self) -> bool {
        (self.bits & Gpio22::PUE_MASK) != 0
    }

    /// Read **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn pde(&self) -> bool {
        (self.bits & Gpio22::PDE_MASK) != 0
    }

    /// Read **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn schmitt(&self) -> bool {
        (self.bits & Gpio22::SCHMITT_MASK) != 0
    }

    /// Read **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn slewfast(&self) -> bool {
        (self.bits & Gpio22::SLEWFAST_MASK) != 0
    }

    /// Set **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn with_iso(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio22::ISO_MASK
            } else {
                self.bits & !Gpio22::ISO_MASK
            }
        }
    }

    /// Set **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn with_od(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio22::OD_MASK
            } else {
                self.bits & !Gpio22::OD_MASK
            }
        }
    }

    /// Set **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn with_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio22::IE_MASK
            } else {
                self.bits & !Gpio22::IE_MASK
            }
        }
    }

    /// Set **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn with_drive(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Gpio22::DRIVE_MASK) | ((value << Gpio22::DRIVE_OFFSET) & Gpio22::DRIVE_MASK)
        }
    }

    /// Set **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn with_pue(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio22::PUE_MASK
            } else {
                self.bits & !Gpio22::PUE_MASK
            }
        }
    }

    /// Set **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn with_pde(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio22::PDE_MASK
            } else {
                self.bits & !Gpio22::PDE_MASK
            }
        }
    }

    /// Set **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn with_schmitt(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio22::SCHMITT_MASK
            } else {
                self.bits & !Gpio22::SCHMITT_MASK
            }
        }
    }

    /// Set **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn with_slewfast(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio22::SLEWFAST_MASK
            } else {
                self.bits & !Gpio22::SLEWFAST_MASK
            }
        }
    }

}

impl Default for Gpio22Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Gpio22 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Gpio22Val {
        Gpio22Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Gpio22Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Gpio22Val) -> Gpio22Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// GPIO23 Register
///
/// Address offset: 0x0060
/// Reset value: 0x00000116
///
/// ## Fields
///
/// - **ISO** `[8]` (RW)
///
///   Pad isolation control. Remove this once the pad is configured by software.
///
/// - **OD** `[7]` (RW)
///
///   Output disable. Has priority over output enable from peripherals
///
/// - **IE** `[6]` (RW)
///
///   Input enable
///
/// - **DRIVE** `[5:4]` (RW)
///
///   Drive strength.
///
/// - **PUE** `[3]` (RW)
///
///   Pull up enable
///
/// - **PDE** `[2]` (RW)
///
///   Pull down enable
///
/// - **SCHMITT** `[1]` (RW)
///
///   Enable schmitt trigger
///
/// - **SLEWFAST** `[0]` (RW)
///
///   Slew rate control. 1 = Fast, 0 = Slow
#[repr(transparent)]
pub struct Gpio23 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Gpio23 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Gpio23 {
    // ======== Field Constants ========

    /// **ISO**
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ISO_OFFSET: u32 = 8;
    pub const ISO_MASK: u32 = 0x00000100;
    pub const ISO: u32 = 1 << 8;

    /// **OD**
    ///
    /// Output disable. Has priority over output enable from peripherals
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const OD_OFFSET: u32 = 7;
    pub const OD_MASK: u32 = 0x00000080;
    pub const OD: u32 = 1 << 7;

    /// **IE**
    ///
    /// Input enable
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const IE_OFFSET: u32 = 6;
    pub const IE_MASK: u32 = 0x00000040;
    pub const IE: u32 = 1 << 6;

    /// **DRIVE**
    ///
    /// Drive strength.
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const DRIVE_OFFSET: u32 = 4;
    pub const DRIVE_MASK: u32 = 0x00000030;
    pub const DRIVE_2M_A: u32 = 0 << 4;
    pub const DRIVE_2M_A_VAL: u32 = 0;
    pub const DRIVE_4M_A: u32 = 1 << 4;
    pub const DRIVE_4M_A_VAL: u32 = 1;
    pub const DRIVE_8M_A: u32 = 2 << 4;
    pub const DRIVE_8M_A_VAL: u32 = 2;
    pub const DRIVE_12M_A: u32 = 3 << 4;
    pub const DRIVE_12M_A_VAL: u32 = 3;

    /// **PUE**
    ///
    /// Pull up enable
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const PUE_OFFSET: u32 = 3;
    pub const PUE_MASK: u32 = 0x00000008;
    pub const PUE: u32 = 1 << 3;

    /// **PDE**
    ///
    /// Pull down enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const PDE_OFFSET: u32 = 2;
    pub const PDE_MASK: u32 = 0x00000004;
    pub const PDE: u32 = 1 << 2;

    /// **SCHMITT**
    ///
    /// Enable schmitt trigger
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SCHMITT_OFFSET: u32 = 1;
    pub const SCHMITT_MASK: u32 = 0x00000002;
    pub const SCHMITT: u32 = 1 << 1;

    /// **SLEWFAST**
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SLEWFAST_OFFSET: u32 = 0;
    pub const SLEWFAST_MASK: u32 = 0x00000001;
    pub const SLEWFAST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ISO** `[8]` (RW)
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub fn iso(&self) -> bool {
        (self.value.read_bits() & Self::ISO_MASK) != 0
    }

    /// Set ISO bit atomically (RW)
    #[inline(always)]
    pub fn set_iso(&self) {
        self.set_bits(Self::ISO_MASK);
    }

    /// Clear ISO bit atomically (RW)
    #[inline(always)]
    pub fn clr_iso(&self) {
        self.clr_bits(Self::ISO_MASK);
    }

    /// **OD** `[7]` (RW)
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub fn od(&self) -> bool {
        (self.value.read_bits() & Self::OD_MASK) != 0
    }

    /// Set OD bit atomically (RW)
    #[inline(always)]
    pub fn set_od(&self) {
        self.set_bits(Self::OD_MASK);
    }

    /// Clear OD bit atomically (RW)
    #[inline(always)]
    pub fn clr_od(&self) {
        self.clr_bits(Self::OD_MASK);
    }

    /// **IE** `[6]` (RW)
    ///
    /// Input enable
    #[inline(always)]
    pub fn ie(&self) -> bool {
        (self.value.read_bits() & Self::IE_MASK) != 0
    }

    /// Set IE bit atomically (RW)
    #[inline(always)]
    pub fn set_ie(&self) {
        self.set_bits(Self::IE_MASK);
    }

    /// Clear IE bit atomically (RW)
    #[inline(always)]
    pub fn clr_ie(&self) {
        self.clr_bits(Self::IE_MASK);
    }

    /// **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn drive(&self) -> u32 {
        (self.value.read_bits() & Self::DRIVE_MASK) >> Self::DRIVE_OFFSET
    }

    /// Write **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn write_drive(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DRIVE_MASK) | ((value << Self::DRIVE_OFFSET) & Self::DRIVE_MASK));
    }

    /// **PUE** `[3]` (RW)
    ///
    /// Pull up enable
    #[inline(always)]
    pub fn pue(&self) -> bool {
        (self.value.read_bits() & Self::PUE_MASK) != 0
    }

    /// Set PUE bit atomically (RW)
    #[inline(always)]
    pub fn set_pue(&self) {
        self.set_bits(Self::PUE_MASK);
    }

    /// Clear PUE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pue(&self) {
        self.clr_bits(Self::PUE_MASK);
    }

    /// **PDE** `[2]` (RW)
    ///
    /// Pull down enable
    #[inline(always)]
    pub fn pde(&self) -> bool {
        (self.value.read_bits() & Self::PDE_MASK) != 0
    }

    /// Set PDE bit atomically (RW)
    #[inline(always)]
    pub fn set_pde(&self) {
        self.set_bits(Self::PDE_MASK);
    }

    /// Clear PDE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pde(&self) {
        self.clr_bits(Self::PDE_MASK);
    }

    /// **SCHMITT** `[1]` (RW)
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub fn schmitt(&self) -> bool {
        (self.value.read_bits() & Self::SCHMITT_MASK) != 0
    }

    /// Set SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn set_schmitt(&self) {
        self.set_bits(Self::SCHMITT_MASK);
    }

    /// Clear SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn clr_schmitt(&self) {
        self.clr_bits(Self::SCHMITT_MASK);
    }

    /// **SLEWFAST** `[0]` (RW)
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub fn slewfast(&self) -> bool {
        (self.value.read_bits() & Self::SLEWFAST_MASK) != 0
    }

    /// Set SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn set_slewfast(&self) {
        self.set_bits(Self::SLEWFAST_MASK);
    }

    /// Clear SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn clr_slewfast(&self) {
        self.clr_bits(Self::SLEWFAST_MASK);
    }

}

/// GPIO23 value type for batch read/modify/write operations
///
/// Use `gpio23.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpio23.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Gpio23Val {
    bits: u32,
}

impl Gpio23Val {
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
        Self { bits: 0x00000116 }
    }

    /// Read **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn iso(&self) -> bool {
        (self.bits & Gpio23::ISO_MASK) != 0
    }

    /// Read **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn od(&self) -> bool {
        (self.bits & Gpio23::OD_MASK) != 0
    }

    /// Read **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        (self.bits & Gpio23::IE_MASK) != 0
    }

    /// Read **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn drive(&self) -> u32 {
        (self.bits & Gpio23::DRIVE_MASK) >> Gpio23::DRIVE_OFFSET
    }

    /// Read **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn pue(&self) -> bool {
        (self.bits & Gpio23::PUE_MASK) != 0
    }

    /// Read **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn pde(&self) -> bool {
        (self.bits & Gpio23::PDE_MASK) != 0
    }

    /// Read **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn schmitt(&self) -> bool {
        (self.bits & Gpio23::SCHMITT_MASK) != 0
    }

    /// Read **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn slewfast(&self) -> bool {
        (self.bits & Gpio23::SLEWFAST_MASK) != 0
    }

    /// Set **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn with_iso(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio23::ISO_MASK
            } else {
                self.bits & !Gpio23::ISO_MASK
            }
        }
    }

    /// Set **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn with_od(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio23::OD_MASK
            } else {
                self.bits & !Gpio23::OD_MASK
            }
        }
    }

    /// Set **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn with_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio23::IE_MASK
            } else {
                self.bits & !Gpio23::IE_MASK
            }
        }
    }

    /// Set **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn with_drive(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Gpio23::DRIVE_MASK) | ((value << Gpio23::DRIVE_OFFSET) & Gpio23::DRIVE_MASK)
        }
    }

    /// Set **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn with_pue(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio23::PUE_MASK
            } else {
                self.bits & !Gpio23::PUE_MASK
            }
        }
    }

    /// Set **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn with_pde(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio23::PDE_MASK
            } else {
                self.bits & !Gpio23::PDE_MASK
            }
        }
    }

    /// Set **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn with_schmitt(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio23::SCHMITT_MASK
            } else {
                self.bits & !Gpio23::SCHMITT_MASK
            }
        }
    }

    /// Set **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn with_slewfast(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio23::SLEWFAST_MASK
            } else {
                self.bits & !Gpio23::SLEWFAST_MASK
            }
        }
    }

}

impl Default for Gpio23Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Gpio23 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Gpio23Val {
        Gpio23Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Gpio23Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Gpio23Val) -> Gpio23Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// GPIO24 Register
///
/// Address offset: 0x0064
/// Reset value: 0x00000116
///
/// ## Fields
///
/// - **ISO** `[8]` (RW)
///
///   Pad isolation control. Remove this once the pad is configured by software.
///
/// - **OD** `[7]` (RW)
///
///   Output disable. Has priority over output enable from peripherals
///
/// - **IE** `[6]` (RW)
///
///   Input enable
///
/// - **DRIVE** `[5:4]` (RW)
///
///   Drive strength.
///
/// - **PUE** `[3]` (RW)
///
///   Pull up enable
///
/// - **PDE** `[2]` (RW)
///
///   Pull down enable
///
/// - **SCHMITT** `[1]` (RW)
///
///   Enable schmitt trigger
///
/// - **SLEWFAST** `[0]` (RW)
///
///   Slew rate control. 1 = Fast, 0 = Slow
#[repr(transparent)]
pub struct Gpio24 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Gpio24 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Gpio24 {
    // ======== Field Constants ========

    /// **ISO**
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ISO_OFFSET: u32 = 8;
    pub const ISO_MASK: u32 = 0x00000100;
    pub const ISO: u32 = 1 << 8;

    /// **OD**
    ///
    /// Output disable. Has priority over output enable from peripherals
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const OD_OFFSET: u32 = 7;
    pub const OD_MASK: u32 = 0x00000080;
    pub const OD: u32 = 1 << 7;

    /// **IE**
    ///
    /// Input enable
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const IE_OFFSET: u32 = 6;
    pub const IE_MASK: u32 = 0x00000040;
    pub const IE: u32 = 1 << 6;

    /// **DRIVE**
    ///
    /// Drive strength.
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const DRIVE_OFFSET: u32 = 4;
    pub const DRIVE_MASK: u32 = 0x00000030;
    pub const DRIVE_2M_A: u32 = 0 << 4;
    pub const DRIVE_2M_A_VAL: u32 = 0;
    pub const DRIVE_4M_A: u32 = 1 << 4;
    pub const DRIVE_4M_A_VAL: u32 = 1;
    pub const DRIVE_8M_A: u32 = 2 << 4;
    pub const DRIVE_8M_A_VAL: u32 = 2;
    pub const DRIVE_12M_A: u32 = 3 << 4;
    pub const DRIVE_12M_A_VAL: u32 = 3;

    /// **PUE**
    ///
    /// Pull up enable
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const PUE_OFFSET: u32 = 3;
    pub const PUE_MASK: u32 = 0x00000008;
    pub const PUE: u32 = 1 << 3;

    /// **PDE**
    ///
    /// Pull down enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const PDE_OFFSET: u32 = 2;
    pub const PDE_MASK: u32 = 0x00000004;
    pub const PDE: u32 = 1 << 2;

    /// **SCHMITT**
    ///
    /// Enable schmitt trigger
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SCHMITT_OFFSET: u32 = 1;
    pub const SCHMITT_MASK: u32 = 0x00000002;
    pub const SCHMITT: u32 = 1 << 1;

    /// **SLEWFAST**
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SLEWFAST_OFFSET: u32 = 0;
    pub const SLEWFAST_MASK: u32 = 0x00000001;
    pub const SLEWFAST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ISO** `[8]` (RW)
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub fn iso(&self) -> bool {
        (self.value.read_bits() & Self::ISO_MASK) != 0
    }

    /// Set ISO bit atomically (RW)
    #[inline(always)]
    pub fn set_iso(&self) {
        self.set_bits(Self::ISO_MASK);
    }

    /// Clear ISO bit atomically (RW)
    #[inline(always)]
    pub fn clr_iso(&self) {
        self.clr_bits(Self::ISO_MASK);
    }

    /// **OD** `[7]` (RW)
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub fn od(&self) -> bool {
        (self.value.read_bits() & Self::OD_MASK) != 0
    }

    /// Set OD bit atomically (RW)
    #[inline(always)]
    pub fn set_od(&self) {
        self.set_bits(Self::OD_MASK);
    }

    /// Clear OD bit atomically (RW)
    #[inline(always)]
    pub fn clr_od(&self) {
        self.clr_bits(Self::OD_MASK);
    }

    /// **IE** `[6]` (RW)
    ///
    /// Input enable
    #[inline(always)]
    pub fn ie(&self) -> bool {
        (self.value.read_bits() & Self::IE_MASK) != 0
    }

    /// Set IE bit atomically (RW)
    #[inline(always)]
    pub fn set_ie(&self) {
        self.set_bits(Self::IE_MASK);
    }

    /// Clear IE bit atomically (RW)
    #[inline(always)]
    pub fn clr_ie(&self) {
        self.clr_bits(Self::IE_MASK);
    }

    /// **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn drive(&self) -> u32 {
        (self.value.read_bits() & Self::DRIVE_MASK) >> Self::DRIVE_OFFSET
    }

    /// Write **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn write_drive(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DRIVE_MASK) | ((value << Self::DRIVE_OFFSET) & Self::DRIVE_MASK));
    }

    /// **PUE** `[3]` (RW)
    ///
    /// Pull up enable
    #[inline(always)]
    pub fn pue(&self) -> bool {
        (self.value.read_bits() & Self::PUE_MASK) != 0
    }

    /// Set PUE bit atomically (RW)
    #[inline(always)]
    pub fn set_pue(&self) {
        self.set_bits(Self::PUE_MASK);
    }

    /// Clear PUE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pue(&self) {
        self.clr_bits(Self::PUE_MASK);
    }

    /// **PDE** `[2]` (RW)
    ///
    /// Pull down enable
    #[inline(always)]
    pub fn pde(&self) -> bool {
        (self.value.read_bits() & Self::PDE_MASK) != 0
    }

    /// Set PDE bit atomically (RW)
    #[inline(always)]
    pub fn set_pde(&self) {
        self.set_bits(Self::PDE_MASK);
    }

    /// Clear PDE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pde(&self) {
        self.clr_bits(Self::PDE_MASK);
    }

    /// **SCHMITT** `[1]` (RW)
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub fn schmitt(&self) -> bool {
        (self.value.read_bits() & Self::SCHMITT_MASK) != 0
    }

    /// Set SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn set_schmitt(&self) {
        self.set_bits(Self::SCHMITT_MASK);
    }

    /// Clear SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn clr_schmitt(&self) {
        self.clr_bits(Self::SCHMITT_MASK);
    }

    /// **SLEWFAST** `[0]` (RW)
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub fn slewfast(&self) -> bool {
        (self.value.read_bits() & Self::SLEWFAST_MASK) != 0
    }

    /// Set SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn set_slewfast(&self) {
        self.set_bits(Self::SLEWFAST_MASK);
    }

    /// Clear SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn clr_slewfast(&self) {
        self.clr_bits(Self::SLEWFAST_MASK);
    }

}

/// GPIO24 value type for batch read/modify/write operations
///
/// Use `gpio24.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpio24.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Gpio24Val {
    bits: u32,
}

impl Gpio24Val {
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
        Self { bits: 0x00000116 }
    }

    /// Read **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn iso(&self) -> bool {
        (self.bits & Gpio24::ISO_MASK) != 0
    }

    /// Read **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn od(&self) -> bool {
        (self.bits & Gpio24::OD_MASK) != 0
    }

    /// Read **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        (self.bits & Gpio24::IE_MASK) != 0
    }

    /// Read **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn drive(&self) -> u32 {
        (self.bits & Gpio24::DRIVE_MASK) >> Gpio24::DRIVE_OFFSET
    }

    /// Read **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn pue(&self) -> bool {
        (self.bits & Gpio24::PUE_MASK) != 0
    }

    /// Read **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn pde(&self) -> bool {
        (self.bits & Gpio24::PDE_MASK) != 0
    }

    /// Read **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn schmitt(&self) -> bool {
        (self.bits & Gpio24::SCHMITT_MASK) != 0
    }

    /// Read **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn slewfast(&self) -> bool {
        (self.bits & Gpio24::SLEWFAST_MASK) != 0
    }

    /// Set **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn with_iso(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio24::ISO_MASK
            } else {
                self.bits & !Gpio24::ISO_MASK
            }
        }
    }

    /// Set **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn with_od(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio24::OD_MASK
            } else {
                self.bits & !Gpio24::OD_MASK
            }
        }
    }

    /// Set **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn with_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio24::IE_MASK
            } else {
                self.bits & !Gpio24::IE_MASK
            }
        }
    }

    /// Set **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn with_drive(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Gpio24::DRIVE_MASK) | ((value << Gpio24::DRIVE_OFFSET) & Gpio24::DRIVE_MASK)
        }
    }

    /// Set **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn with_pue(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio24::PUE_MASK
            } else {
                self.bits & !Gpio24::PUE_MASK
            }
        }
    }

    /// Set **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn with_pde(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio24::PDE_MASK
            } else {
                self.bits & !Gpio24::PDE_MASK
            }
        }
    }

    /// Set **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn with_schmitt(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio24::SCHMITT_MASK
            } else {
                self.bits & !Gpio24::SCHMITT_MASK
            }
        }
    }

    /// Set **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn with_slewfast(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio24::SLEWFAST_MASK
            } else {
                self.bits & !Gpio24::SLEWFAST_MASK
            }
        }
    }

}

impl Default for Gpio24Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Gpio24 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Gpio24Val {
        Gpio24Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Gpio24Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Gpio24Val) -> Gpio24Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// GPIO25 Register
///
/// Address offset: 0x0068
/// Reset value: 0x00000116
///
/// ## Fields
///
/// - **ISO** `[8]` (RW)
///
///   Pad isolation control. Remove this once the pad is configured by software.
///
/// - **OD** `[7]` (RW)
///
///   Output disable. Has priority over output enable from peripherals
///
/// - **IE** `[6]` (RW)
///
///   Input enable
///
/// - **DRIVE** `[5:4]` (RW)
///
///   Drive strength.
///
/// - **PUE** `[3]` (RW)
///
///   Pull up enable
///
/// - **PDE** `[2]` (RW)
///
///   Pull down enable
///
/// - **SCHMITT** `[1]` (RW)
///
///   Enable schmitt trigger
///
/// - **SLEWFAST** `[0]` (RW)
///
///   Slew rate control. 1 = Fast, 0 = Slow
#[repr(transparent)]
pub struct Gpio25 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Gpio25 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Gpio25 {
    // ======== Field Constants ========

    /// **ISO**
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ISO_OFFSET: u32 = 8;
    pub const ISO_MASK: u32 = 0x00000100;
    pub const ISO: u32 = 1 << 8;

    /// **OD**
    ///
    /// Output disable. Has priority over output enable from peripherals
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const OD_OFFSET: u32 = 7;
    pub const OD_MASK: u32 = 0x00000080;
    pub const OD: u32 = 1 << 7;

    /// **IE**
    ///
    /// Input enable
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const IE_OFFSET: u32 = 6;
    pub const IE_MASK: u32 = 0x00000040;
    pub const IE: u32 = 1 << 6;

    /// **DRIVE**
    ///
    /// Drive strength.
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const DRIVE_OFFSET: u32 = 4;
    pub const DRIVE_MASK: u32 = 0x00000030;
    pub const DRIVE_2M_A: u32 = 0 << 4;
    pub const DRIVE_2M_A_VAL: u32 = 0;
    pub const DRIVE_4M_A: u32 = 1 << 4;
    pub const DRIVE_4M_A_VAL: u32 = 1;
    pub const DRIVE_8M_A: u32 = 2 << 4;
    pub const DRIVE_8M_A_VAL: u32 = 2;
    pub const DRIVE_12M_A: u32 = 3 << 4;
    pub const DRIVE_12M_A_VAL: u32 = 3;

    /// **PUE**
    ///
    /// Pull up enable
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const PUE_OFFSET: u32 = 3;
    pub const PUE_MASK: u32 = 0x00000008;
    pub const PUE: u32 = 1 << 3;

    /// **PDE**
    ///
    /// Pull down enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const PDE_OFFSET: u32 = 2;
    pub const PDE_MASK: u32 = 0x00000004;
    pub const PDE: u32 = 1 << 2;

    /// **SCHMITT**
    ///
    /// Enable schmitt trigger
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SCHMITT_OFFSET: u32 = 1;
    pub const SCHMITT_MASK: u32 = 0x00000002;
    pub const SCHMITT: u32 = 1 << 1;

    /// **SLEWFAST**
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SLEWFAST_OFFSET: u32 = 0;
    pub const SLEWFAST_MASK: u32 = 0x00000001;
    pub const SLEWFAST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ISO** `[8]` (RW)
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub fn iso(&self) -> bool {
        (self.value.read_bits() & Self::ISO_MASK) != 0
    }

    /// Set ISO bit atomically (RW)
    #[inline(always)]
    pub fn set_iso(&self) {
        self.set_bits(Self::ISO_MASK);
    }

    /// Clear ISO bit atomically (RW)
    #[inline(always)]
    pub fn clr_iso(&self) {
        self.clr_bits(Self::ISO_MASK);
    }

    /// **OD** `[7]` (RW)
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub fn od(&self) -> bool {
        (self.value.read_bits() & Self::OD_MASK) != 0
    }

    /// Set OD bit atomically (RW)
    #[inline(always)]
    pub fn set_od(&self) {
        self.set_bits(Self::OD_MASK);
    }

    /// Clear OD bit atomically (RW)
    #[inline(always)]
    pub fn clr_od(&self) {
        self.clr_bits(Self::OD_MASK);
    }

    /// **IE** `[6]` (RW)
    ///
    /// Input enable
    #[inline(always)]
    pub fn ie(&self) -> bool {
        (self.value.read_bits() & Self::IE_MASK) != 0
    }

    /// Set IE bit atomically (RW)
    #[inline(always)]
    pub fn set_ie(&self) {
        self.set_bits(Self::IE_MASK);
    }

    /// Clear IE bit atomically (RW)
    #[inline(always)]
    pub fn clr_ie(&self) {
        self.clr_bits(Self::IE_MASK);
    }

    /// **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn drive(&self) -> u32 {
        (self.value.read_bits() & Self::DRIVE_MASK) >> Self::DRIVE_OFFSET
    }

    /// Write **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn write_drive(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DRIVE_MASK) | ((value << Self::DRIVE_OFFSET) & Self::DRIVE_MASK));
    }

    /// **PUE** `[3]` (RW)
    ///
    /// Pull up enable
    #[inline(always)]
    pub fn pue(&self) -> bool {
        (self.value.read_bits() & Self::PUE_MASK) != 0
    }

    /// Set PUE bit atomically (RW)
    #[inline(always)]
    pub fn set_pue(&self) {
        self.set_bits(Self::PUE_MASK);
    }

    /// Clear PUE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pue(&self) {
        self.clr_bits(Self::PUE_MASK);
    }

    /// **PDE** `[2]` (RW)
    ///
    /// Pull down enable
    #[inline(always)]
    pub fn pde(&self) -> bool {
        (self.value.read_bits() & Self::PDE_MASK) != 0
    }

    /// Set PDE bit atomically (RW)
    #[inline(always)]
    pub fn set_pde(&self) {
        self.set_bits(Self::PDE_MASK);
    }

    /// Clear PDE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pde(&self) {
        self.clr_bits(Self::PDE_MASK);
    }

    /// **SCHMITT** `[1]` (RW)
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub fn schmitt(&self) -> bool {
        (self.value.read_bits() & Self::SCHMITT_MASK) != 0
    }

    /// Set SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn set_schmitt(&self) {
        self.set_bits(Self::SCHMITT_MASK);
    }

    /// Clear SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn clr_schmitt(&self) {
        self.clr_bits(Self::SCHMITT_MASK);
    }

    /// **SLEWFAST** `[0]` (RW)
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub fn slewfast(&self) -> bool {
        (self.value.read_bits() & Self::SLEWFAST_MASK) != 0
    }

    /// Set SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn set_slewfast(&self) {
        self.set_bits(Self::SLEWFAST_MASK);
    }

    /// Clear SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn clr_slewfast(&self) {
        self.clr_bits(Self::SLEWFAST_MASK);
    }

}

/// GPIO25 value type for batch read/modify/write operations
///
/// Use `gpio25.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpio25.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Gpio25Val {
    bits: u32,
}

impl Gpio25Val {
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
        Self { bits: 0x00000116 }
    }

    /// Read **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn iso(&self) -> bool {
        (self.bits & Gpio25::ISO_MASK) != 0
    }

    /// Read **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn od(&self) -> bool {
        (self.bits & Gpio25::OD_MASK) != 0
    }

    /// Read **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        (self.bits & Gpio25::IE_MASK) != 0
    }

    /// Read **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn drive(&self) -> u32 {
        (self.bits & Gpio25::DRIVE_MASK) >> Gpio25::DRIVE_OFFSET
    }

    /// Read **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn pue(&self) -> bool {
        (self.bits & Gpio25::PUE_MASK) != 0
    }

    /// Read **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn pde(&self) -> bool {
        (self.bits & Gpio25::PDE_MASK) != 0
    }

    /// Read **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn schmitt(&self) -> bool {
        (self.bits & Gpio25::SCHMITT_MASK) != 0
    }

    /// Read **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn slewfast(&self) -> bool {
        (self.bits & Gpio25::SLEWFAST_MASK) != 0
    }

    /// Set **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn with_iso(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio25::ISO_MASK
            } else {
                self.bits & !Gpio25::ISO_MASK
            }
        }
    }

    /// Set **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn with_od(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio25::OD_MASK
            } else {
                self.bits & !Gpio25::OD_MASK
            }
        }
    }

    /// Set **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn with_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio25::IE_MASK
            } else {
                self.bits & !Gpio25::IE_MASK
            }
        }
    }

    /// Set **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn with_drive(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Gpio25::DRIVE_MASK) | ((value << Gpio25::DRIVE_OFFSET) & Gpio25::DRIVE_MASK)
        }
    }

    /// Set **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn with_pue(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio25::PUE_MASK
            } else {
                self.bits & !Gpio25::PUE_MASK
            }
        }
    }

    /// Set **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn with_pde(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio25::PDE_MASK
            } else {
                self.bits & !Gpio25::PDE_MASK
            }
        }
    }

    /// Set **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn with_schmitt(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio25::SCHMITT_MASK
            } else {
                self.bits & !Gpio25::SCHMITT_MASK
            }
        }
    }

    /// Set **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn with_slewfast(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio25::SLEWFAST_MASK
            } else {
                self.bits & !Gpio25::SLEWFAST_MASK
            }
        }
    }

}

impl Default for Gpio25Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Gpio25 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Gpio25Val {
        Gpio25Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Gpio25Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Gpio25Val) -> Gpio25Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// GPIO26 Register
///
/// Address offset: 0x006c
/// Reset value: 0x00000116
///
/// ## Fields
///
/// - **ISO** `[8]` (RW)
///
///   Pad isolation control. Remove this once the pad is configured by software.
///
/// - **OD** `[7]` (RW)
///
///   Output disable. Has priority over output enable from peripherals
///
/// - **IE** `[6]` (RW)
///
///   Input enable
///
/// - **DRIVE** `[5:4]` (RW)
///
///   Drive strength.
///
/// - **PUE** `[3]` (RW)
///
///   Pull up enable
///
/// - **PDE** `[2]` (RW)
///
///   Pull down enable
///
/// - **SCHMITT** `[1]` (RW)
///
///   Enable schmitt trigger
///
/// - **SLEWFAST** `[0]` (RW)
///
///   Slew rate control. 1 = Fast, 0 = Slow
#[repr(transparent)]
pub struct Gpio26 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Gpio26 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Gpio26 {
    // ======== Field Constants ========

    /// **ISO**
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ISO_OFFSET: u32 = 8;
    pub const ISO_MASK: u32 = 0x00000100;
    pub const ISO: u32 = 1 << 8;

    /// **OD**
    ///
    /// Output disable. Has priority over output enable from peripherals
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const OD_OFFSET: u32 = 7;
    pub const OD_MASK: u32 = 0x00000080;
    pub const OD: u32 = 1 << 7;

    /// **IE**
    ///
    /// Input enable
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const IE_OFFSET: u32 = 6;
    pub const IE_MASK: u32 = 0x00000040;
    pub const IE: u32 = 1 << 6;

    /// **DRIVE**
    ///
    /// Drive strength.
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const DRIVE_OFFSET: u32 = 4;
    pub const DRIVE_MASK: u32 = 0x00000030;
    pub const DRIVE_2M_A: u32 = 0 << 4;
    pub const DRIVE_2M_A_VAL: u32 = 0;
    pub const DRIVE_4M_A: u32 = 1 << 4;
    pub const DRIVE_4M_A_VAL: u32 = 1;
    pub const DRIVE_8M_A: u32 = 2 << 4;
    pub const DRIVE_8M_A_VAL: u32 = 2;
    pub const DRIVE_12M_A: u32 = 3 << 4;
    pub const DRIVE_12M_A_VAL: u32 = 3;

    /// **PUE**
    ///
    /// Pull up enable
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const PUE_OFFSET: u32 = 3;
    pub const PUE_MASK: u32 = 0x00000008;
    pub const PUE: u32 = 1 << 3;

    /// **PDE**
    ///
    /// Pull down enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const PDE_OFFSET: u32 = 2;
    pub const PDE_MASK: u32 = 0x00000004;
    pub const PDE: u32 = 1 << 2;

    /// **SCHMITT**
    ///
    /// Enable schmitt trigger
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SCHMITT_OFFSET: u32 = 1;
    pub const SCHMITT_MASK: u32 = 0x00000002;
    pub const SCHMITT: u32 = 1 << 1;

    /// **SLEWFAST**
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SLEWFAST_OFFSET: u32 = 0;
    pub const SLEWFAST_MASK: u32 = 0x00000001;
    pub const SLEWFAST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ISO** `[8]` (RW)
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub fn iso(&self) -> bool {
        (self.value.read_bits() & Self::ISO_MASK) != 0
    }

    /// Set ISO bit atomically (RW)
    #[inline(always)]
    pub fn set_iso(&self) {
        self.set_bits(Self::ISO_MASK);
    }

    /// Clear ISO bit atomically (RW)
    #[inline(always)]
    pub fn clr_iso(&self) {
        self.clr_bits(Self::ISO_MASK);
    }

    /// **OD** `[7]` (RW)
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub fn od(&self) -> bool {
        (self.value.read_bits() & Self::OD_MASK) != 0
    }

    /// Set OD bit atomically (RW)
    #[inline(always)]
    pub fn set_od(&self) {
        self.set_bits(Self::OD_MASK);
    }

    /// Clear OD bit atomically (RW)
    #[inline(always)]
    pub fn clr_od(&self) {
        self.clr_bits(Self::OD_MASK);
    }

    /// **IE** `[6]` (RW)
    ///
    /// Input enable
    #[inline(always)]
    pub fn ie(&self) -> bool {
        (self.value.read_bits() & Self::IE_MASK) != 0
    }

    /// Set IE bit atomically (RW)
    #[inline(always)]
    pub fn set_ie(&self) {
        self.set_bits(Self::IE_MASK);
    }

    /// Clear IE bit atomically (RW)
    #[inline(always)]
    pub fn clr_ie(&self) {
        self.clr_bits(Self::IE_MASK);
    }

    /// **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn drive(&self) -> u32 {
        (self.value.read_bits() & Self::DRIVE_MASK) >> Self::DRIVE_OFFSET
    }

    /// Write **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn write_drive(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DRIVE_MASK) | ((value << Self::DRIVE_OFFSET) & Self::DRIVE_MASK));
    }

    /// **PUE** `[3]` (RW)
    ///
    /// Pull up enable
    #[inline(always)]
    pub fn pue(&self) -> bool {
        (self.value.read_bits() & Self::PUE_MASK) != 0
    }

    /// Set PUE bit atomically (RW)
    #[inline(always)]
    pub fn set_pue(&self) {
        self.set_bits(Self::PUE_MASK);
    }

    /// Clear PUE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pue(&self) {
        self.clr_bits(Self::PUE_MASK);
    }

    /// **PDE** `[2]` (RW)
    ///
    /// Pull down enable
    #[inline(always)]
    pub fn pde(&self) -> bool {
        (self.value.read_bits() & Self::PDE_MASK) != 0
    }

    /// Set PDE bit atomically (RW)
    #[inline(always)]
    pub fn set_pde(&self) {
        self.set_bits(Self::PDE_MASK);
    }

    /// Clear PDE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pde(&self) {
        self.clr_bits(Self::PDE_MASK);
    }

    /// **SCHMITT** `[1]` (RW)
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub fn schmitt(&self) -> bool {
        (self.value.read_bits() & Self::SCHMITT_MASK) != 0
    }

    /// Set SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn set_schmitt(&self) {
        self.set_bits(Self::SCHMITT_MASK);
    }

    /// Clear SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn clr_schmitt(&self) {
        self.clr_bits(Self::SCHMITT_MASK);
    }

    /// **SLEWFAST** `[0]` (RW)
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub fn slewfast(&self) -> bool {
        (self.value.read_bits() & Self::SLEWFAST_MASK) != 0
    }

    /// Set SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn set_slewfast(&self) {
        self.set_bits(Self::SLEWFAST_MASK);
    }

    /// Clear SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn clr_slewfast(&self) {
        self.clr_bits(Self::SLEWFAST_MASK);
    }

}

/// GPIO26 value type for batch read/modify/write operations
///
/// Use `gpio26.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpio26.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Gpio26Val {
    bits: u32,
}

impl Gpio26Val {
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
        Self { bits: 0x00000116 }
    }

    /// Read **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn iso(&self) -> bool {
        (self.bits & Gpio26::ISO_MASK) != 0
    }

    /// Read **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn od(&self) -> bool {
        (self.bits & Gpio26::OD_MASK) != 0
    }

    /// Read **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        (self.bits & Gpio26::IE_MASK) != 0
    }

    /// Read **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn drive(&self) -> u32 {
        (self.bits & Gpio26::DRIVE_MASK) >> Gpio26::DRIVE_OFFSET
    }

    /// Read **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn pue(&self) -> bool {
        (self.bits & Gpio26::PUE_MASK) != 0
    }

    /// Read **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn pde(&self) -> bool {
        (self.bits & Gpio26::PDE_MASK) != 0
    }

    /// Read **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn schmitt(&self) -> bool {
        (self.bits & Gpio26::SCHMITT_MASK) != 0
    }

    /// Read **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn slewfast(&self) -> bool {
        (self.bits & Gpio26::SLEWFAST_MASK) != 0
    }

    /// Set **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn with_iso(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio26::ISO_MASK
            } else {
                self.bits & !Gpio26::ISO_MASK
            }
        }
    }

    /// Set **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn with_od(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio26::OD_MASK
            } else {
                self.bits & !Gpio26::OD_MASK
            }
        }
    }

    /// Set **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn with_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio26::IE_MASK
            } else {
                self.bits & !Gpio26::IE_MASK
            }
        }
    }

    /// Set **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn with_drive(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Gpio26::DRIVE_MASK) | ((value << Gpio26::DRIVE_OFFSET) & Gpio26::DRIVE_MASK)
        }
    }

    /// Set **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn with_pue(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio26::PUE_MASK
            } else {
                self.bits & !Gpio26::PUE_MASK
            }
        }
    }

    /// Set **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn with_pde(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio26::PDE_MASK
            } else {
                self.bits & !Gpio26::PDE_MASK
            }
        }
    }

    /// Set **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn with_schmitt(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio26::SCHMITT_MASK
            } else {
                self.bits & !Gpio26::SCHMITT_MASK
            }
        }
    }

    /// Set **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn with_slewfast(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio26::SLEWFAST_MASK
            } else {
                self.bits & !Gpio26::SLEWFAST_MASK
            }
        }
    }

}

impl Default for Gpio26Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Gpio26 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Gpio26Val {
        Gpio26Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Gpio26Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Gpio26Val) -> Gpio26Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// GPIO27 Register
///
/// Address offset: 0x0070
/// Reset value: 0x00000116
///
/// ## Fields
///
/// - **ISO** `[8]` (RW)
///
///   Pad isolation control. Remove this once the pad is configured by software.
///
/// - **OD** `[7]` (RW)
///
///   Output disable. Has priority over output enable from peripherals
///
/// - **IE** `[6]` (RW)
///
///   Input enable
///
/// - **DRIVE** `[5:4]` (RW)
///
///   Drive strength.
///
/// - **PUE** `[3]` (RW)
///
///   Pull up enable
///
/// - **PDE** `[2]` (RW)
///
///   Pull down enable
///
/// - **SCHMITT** `[1]` (RW)
///
///   Enable schmitt trigger
///
/// - **SLEWFAST** `[0]` (RW)
///
///   Slew rate control. 1 = Fast, 0 = Slow
#[repr(transparent)]
pub struct Gpio27 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Gpio27 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Gpio27 {
    // ======== Field Constants ========

    /// **ISO**
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ISO_OFFSET: u32 = 8;
    pub const ISO_MASK: u32 = 0x00000100;
    pub const ISO: u32 = 1 << 8;

    /// **OD**
    ///
    /// Output disable. Has priority over output enable from peripherals
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const OD_OFFSET: u32 = 7;
    pub const OD_MASK: u32 = 0x00000080;
    pub const OD: u32 = 1 << 7;

    /// **IE**
    ///
    /// Input enable
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const IE_OFFSET: u32 = 6;
    pub const IE_MASK: u32 = 0x00000040;
    pub const IE: u32 = 1 << 6;

    /// **DRIVE**
    ///
    /// Drive strength.
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const DRIVE_OFFSET: u32 = 4;
    pub const DRIVE_MASK: u32 = 0x00000030;
    pub const DRIVE_2M_A: u32 = 0 << 4;
    pub const DRIVE_2M_A_VAL: u32 = 0;
    pub const DRIVE_4M_A: u32 = 1 << 4;
    pub const DRIVE_4M_A_VAL: u32 = 1;
    pub const DRIVE_8M_A: u32 = 2 << 4;
    pub const DRIVE_8M_A_VAL: u32 = 2;
    pub const DRIVE_12M_A: u32 = 3 << 4;
    pub const DRIVE_12M_A_VAL: u32 = 3;

    /// **PUE**
    ///
    /// Pull up enable
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const PUE_OFFSET: u32 = 3;
    pub const PUE_MASK: u32 = 0x00000008;
    pub const PUE: u32 = 1 << 3;

    /// **PDE**
    ///
    /// Pull down enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const PDE_OFFSET: u32 = 2;
    pub const PDE_MASK: u32 = 0x00000004;
    pub const PDE: u32 = 1 << 2;

    /// **SCHMITT**
    ///
    /// Enable schmitt trigger
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SCHMITT_OFFSET: u32 = 1;
    pub const SCHMITT_MASK: u32 = 0x00000002;
    pub const SCHMITT: u32 = 1 << 1;

    /// **SLEWFAST**
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SLEWFAST_OFFSET: u32 = 0;
    pub const SLEWFAST_MASK: u32 = 0x00000001;
    pub const SLEWFAST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ISO** `[8]` (RW)
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub fn iso(&self) -> bool {
        (self.value.read_bits() & Self::ISO_MASK) != 0
    }

    /// Set ISO bit atomically (RW)
    #[inline(always)]
    pub fn set_iso(&self) {
        self.set_bits(Self::ISO_MASK);
    }

    /// Clear ISO bit atomically (RW)
    #[inline(always)]
    pub fn clr_iso(&self) {
        self.clr_bits(Self::ISO_MASK);
    }

    /// **OD** `[7]` (RW)
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub fn od(&self) -> bool {
        (self.value.read_bits() & Self::OD_MASK) != 0
    }

    /// Set OD bit atomically (RW)
    #[inline(always)]
    pub fn set_od(&self) {
        self.set_bits(Self::OD_MASK);
    }

    /// Clear OD bit atomically (RW)
    #[inline(always)]
    pub fn clr_od(&self) {
        self.clr_bits(Self::OD_MASK);
    }

    /// **IE** `[6]` (RW)
    ///
    /// Input enable
    #[inline(always)]
    pub fn ie(&self) -> bool {
        (self.value.read_bits() & Self::IE_MASK) != 0
    }

    /// Set IE bit atomically (RW)
    #[inline(always)]
    pub fn set_ie(&self) {
        self.set_bits(Self::IE_MASK);
    }

    /// Clear IE bit atomically (RW)
    #[inline(always)]
    pub fn clr_ie(&self) {
        self.clr_bits(Self::IE_MASK);
    }

    /// **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn drive(&self) -> u32 {
        (self.value.read_bits() & Self::DRIVE_MASK) >> Self::DRIVE_OFFSET
    }

    /// Write **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn write_drive(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DRIVE_MASK) | ((value << Self::DRIVE_OFFSET) & Self::DRIVE_MASK));
    }

    /// **PUE** `[3]` (RW)
    ///
    /// Pull up enable
    #[inline(always)]
    pub fn pue(&self) -> bool {
        (self.value.read_bits() & Self::PUE_MASK) != 0
    }

    /// Set PUE bit atomically (RW)
    #[inline(always)]
    pub fn set_pue(&self) {
        self.set_bits(Self::PUE_MASK);
    }

    /// Clear PUE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pue(&self) {
        self.clr_bits(Self::PUE_MASK);
    }

    /// **PDE** `[2]` (RW)
    ///
    /// Pull down enable
    #[inline(always)]
    pub fn pde(&self) -> bool {
        (self.value.read_bits() & Self::PDE_MASK) != 0
    }

    /// Set PDE bit atomically (RW)
    #[inline(always)]
    pub fn set_pde(&self) {
        self.set_bits(Self::PDE_MASK);
    }

    /// Clear PDE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pde(&self) {
        self.clr_bits(Self::PDE_MASK);
    }

    /// **SCHMITT** `[1]` (RW)
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub fn schmitt(&self) -> bool {
        (self.value.read_bits() & Self::SCHMITT_MASK) != 0
    }

    /// Set SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn set_schmitt(&self) {
        self.set_bits(Self::SCHMITT_MASK);
    }

    /// Clear SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn clr_schmitt(&self) {
        self.clr_bits(Self::SCHMITT_MASK);
    }

    /// **SLEWFAST** `[0]` (RW)
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub fn slewfast(&self) -> bool {
        (self.value.read_bits() & Self::SLEWFAST_MASK) != 0
    }

    /// Set SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn set_slewfast(&self) {
        self.set_bits(Self::SLEWFAST_MASK);
    }

    /// Clear SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn clr_slewfast(&self) {
        self.clr_bits(Self::SLEWFAST_MASK);
    }

}

/// GPIO27 value type for batch read/modify/write operations
///
/// Use `gpio27.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpio27.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Gpio27Val {
    bits: u32,
}

impl Gpio27Val {
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
        Self { bits: 0x00000116 }
    }

    /// Read **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn iso(&self) -> bool {
        (self.bits & Gpio27::ISO_MASK) != 0
    }

    /// Read **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn od(&self) -> bool {
        (self.bits & Gpio27::OD_MASK) != 0
    }

    /// Read **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        (self.bits & Gpio27::IE_MASK) != 0
    }

    /// Read **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn drive(&self) -> u32 {
        (self.bits & Gpio27::DRIVE_MASK) >> Gpio27::DRIVE_OFFSET
    }

    /// Read **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn pue(&self) -> bool {
        (self.bits & Gpio27::PUE_MASK) != 0
    }

    /// Read **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn pde(&self) -> bool {
        (self.bits & Gpio27::PDE_MASK) != 0
    }

    /// Read **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn schmitt(&self) -> bool {
        (self.bits & Gpio27::SCHMITT_MASK) != 0
    }

    /// Read **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn slewfast(&self) -> bool {
        (self.bits & Gpio27::SLEWFAST_MASK) != 0
    }

    /// Set **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn with_iso(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio27::ISO_MASK
            } else {
                self.bits & !Gpio27::ISO_MASK
            }
        }
    }

    /// Set **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn with_od(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio27::OD_MASK
            } else {
                self.bits & !Gpio27::OD_MASK
            }
        }
    }

    /// Set **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn with_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio27::IE_MASK
            } else {
                self.bits & !Gpio27::IE_MASK
            }
        }
    }

    /// Set **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn with_drive(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Gpio27::DRIVE_MASK) | ((value << Gpio27::DRIVE_OFFSET) & Gpio27::DRIVE_MASK)
        }
    }

    /// Set **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn with_pue(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio27::PUE_MASK
            } else {
                self.bits & !Gpio27::PUE_MASK
            }
        }
    }

    /// Set **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn with_pde(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio27::PDE_MASK
            } else {
                self.bits & !Gpio27::PDE_MASK
            }
        }
    }

    /// Set **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn with_schmitt(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio27::SCHMITT_MASK
            } else {
                self.bits & !Gpio27::SCHMITT_MASK
            }
        }
    }

    /// Set **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn with_slewfast(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio27::SLEWFAST_MASK
            } else {
                self.bits & !Gpio27::SLEWFAST_MASK
            }
        }
    }

}

impl Default for Gpio27Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Gpio27 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Gpio27Val {
        Gpio27Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Gpio27Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Gpio27Val) -> Gpio27Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// GPIO28 Register
///
/// Address offset: 0x0074
/// Reset value: 0x00000116
///
/// ## Fields
///
/// - **ISO** `[8]` (RW)
///
///   Pad isolation control. Remove this once the pad is configured by software.
///
/// - **OD** `[7]` (RW)
///
///   Output disable. Has priority over output enable from peripherals
///
/// - **IE** `[6]` (RW)
///
///   Input enable
///
/// - **DRIVE** `[5:4]` (RW)
///
///   Drive strength.
///
/// - **PUE** `[3]` (RW)
///
///   Pull up enable
///
/// - **PDE** `[2]` (RW)
///
///   Pull down enable
///
/// - **SCHMITT** `[1]` (RW)
///
///   Enable schmitt trigger
///
/// - **SLEWFAST** `[0]` (RW)
///
///   Slew rate control. 1 = Fast, 0 = Slow
#[repr(transparent)]
pub struct Gpio28 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Gpio28 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Gpio28 {
    // ======== Field Constants ========

    /// **ISO**
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ISO_OFFSET: u32 = 8;
    pub const ISO_MASK: u32 = 0x00000100;
    pub const ISO: u32 = 1 << 8;

    /// **OD**
    ///
    /// Output disable. Has priority over output enable from peripherals
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const OD_OFFSET: u32 = 7;
    pub const OD_MASK: u32 = 0x00000080;
    pub const OD: u32 = 1 << 7;

    /// **IE**
    ///
    /// Input enable
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const IE_OFFSET: u32 = 6;
    pub const IE_MASK: u32 = 0x00000040;
    pub const IE: u32 = 1 << 6;

    /// **DRIVE**
    ///
    /// Drive strength.
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const DRIVE_OFFSET: u32 = 4;
    pub const DRIVE_MASK: u32 = 0x00000030;
    pub const DRIVE_2M_A: u32 = 0 << 4;
    pub const DRIVE_2M_A_VAL: u32 = 0;
    pub const DRIVE_4M_A: u32 = 1 << 4;
    pub const DRIVE_4M_A_VAL: u32 = 1;
    pub const DRIVE_8M_A: u32 = 2 << 4;
    pub const DRIVE_8M_A_VAL: u32 = 2;
    pub const DRIVE_12M_A: u32 = 3 << 4;
    pub const DRIVE_12M_A_VAL: u32 = 3;

    /// **PUE**
    ///
    /// Pull up enable
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const PUE_OFFSET: u32 = 3;
    pub const PUE_MASK: u32 = 0x00000008;
    pub const PUE: u32 = 1 << 3;

    /// **PDE**
    ///
    /// Pull down enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const PDE_OFFSET: u32 = 2;
    pub const PDE_MASK: u32 = 0x00000004;
    pub const PDE: u32 = 1 << 2;

    /// **SCHMITT**
    ///
    /// Enable schmitt trigger
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SCHMITT_OFFSET: u32 = 1;
    pub const SCHMITT_MASK: u32 = 0x00000002;
    pub const SCHMITT: u32 = 1 << 1;

    /// **SLEWFAST**
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SLEWFAST_OFFSET: u32 = 0;
    pub const SLEWFAST_MASK: u32 = 0x00000001;
    pub const SLEWFAST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ISO** `[8]` (RW)
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub fn iso(&self) -> bool {
        (self.value.read_bits() & Self::ISO_MASK) != 0
    }

    /// Set ISO bit atomically (RW)
    #[inline(always)]
    pub fn set_iso(&self) {
        self.set_bits(Self::ISO_MASK);
    }

    /// Clear ISO bit atomically (RW)
    #[inline(always)]
    pub fn clr_iso(&self) {
        self.clr_bits(Self::ISO_MASK);
    }

    /// **OD** `[7]` (RW)
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub fn od(&self) -> bool {
        (self.value.read_bits() & Self::OD_MASK) != 0
    }

    /// Set OD bit atomically (RW)
    #[inline(always)]
    pub fn set_od(&self) {
        self.set_bits(Self::OD_MASK);
    }

    /// Clear OD bit atomically (RW)
    #[inline(always)]
    pub fn clr_od(&self) {
        self.clr_bits(Self::OD_MASK);
    }

    /// **IE** `[6]` (RW)
    ///
    /// Input enable
    #[inline(always)]
    pub fn ie(&self) -> bool {
        (self.value.read_bits() & Self::IE_MASK) != 0
    }

    /// Set IE bit atomically (RW)
    #[inline(always)]
    pub fn set_ie(&self) {
        self.set_bits(Self::IE_MASK);
    }

    /// Clear IE bit atomically (RW)
    #[inline(always)]
    pub fn clr_ie(&self) {
        self.clr_bits(Self::IE_MASK);
    }

    /// **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn drive(&self) -> u32 {
        (self.value.read_bits() & Self::DRIVE_MASK) >> Self::DRIVE_OFFSET
    }

    /// Write **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn write_drive(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DRIVE_MASK) | ((value << Self::DRIVE_OFFSET) & Self::DRIVE_MASK));
    }

    /// **PUE** `[3]` (RW)
    ///
    /// Pull up enable
    #[inline(always)]
    pub fn pue(&self) -> bool {
        (self.value.read_bits() & Self::PUE_MASK) != 0
    }

    /// Set PUE bit atomically (RW)
    #[inline(always)]
    pub fn set_pue(&self) {
        self.set_bits(Self::PUE_MASK);
    }

    /// Clear PUE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pue(&self) {
        self.clr_bits(Self::PUE_MASK);
    }

    /// **PDE** `[2]` (RW)
    ///
    /// Pull down enable
    #[inline(always)]
    pub fn pde(&self) -> bool {
        (self.value.read_bits() & Self::PDE_MASK) != 0
    }

    /// Set PDE bit atomically (RW)
    #[inline(always)]
    pub fn set_pde(&self) {
        self.set_bits(Self::PDE_MASK);
    }

    /// Clear PDE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pde(&self) {
        self.clr_bits(Self::PDE_MASK);
    }

    /// **SCHMITT** `[1]` (RW)
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub fn schmitt(&self) -> bool {
        (self.value.read_bits() & Self::SCHMITT_MASK) != 0
    }

    /// Set SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn set_schmitt(&self) {
        self.set_bits(Self::SCHMITT_MASK);
    }

    /// Clear SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn clr_schmitt(&self) {
        self.clr_bits(Self::SCHMITT_MASK);
    }

    /// **SLEWFAST** `[0]` (RW)
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub fn slewfast(&self) -> bool {
        (self.value.read_bits() & Self::SLEWFAST_MASK) != 0
    }

    /// Set SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn set_slewfast(&self) {
        self.set_bits(Self::SLEWFAST_MASK);
    }

    /// Clear SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn clr_slewfast(&self) {
        self.clr_bits(Self::SLEWFAST_MASK);
    }

}

/// GPIO28 value type for batch read/modify/write operations
///
/// Use `gpio28.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpio28.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Gpio28Val {
    bits: u32,
}

impl Gpio28Val {
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
        Self { bits: 0x00000116 }
    }

    /// Read **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn iso(&self) -> bool {
        (self.bits & Gpio28::ISO_MASK) != 0
    }

    /// Read **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn od(&self) -> bool {
        (self.bits & Gpio28::OD_MASK) != 0
    }

    /// Read **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        (self.bits & Gpio28::IE_MASK) != 0
    }

    /// Read **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn drive(&self) -> u32 {
        (self.bits & Gpio28::DRIVE_MASK) >> Gpio28::DRIVE_OFFSET
    }

    /// Read **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn pue(&self) -> bool {
        (self.bits & Gpio28::PUE_MASK) != 0
    }

    /// Read **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn pde(&self) -> bool {
        (self.bits & Gpio28::PDE_MASK) != 0
    }

    /// Read **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn schmitt(&self) -> bool {
        (self.bits & Gpio28::SCHMITT_MASK) != 0
    }

    /// Read **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn slewfast(&self) -> bool {
        (self.bits & Gpio28::SLEWFAST_MASK) != 0
    }

    /// Set **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn with_iso(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio28::ISO_MASK
            } else {
                self.bits & !Gpio28::ISO_MASK
            }
        }
    }

    /// Set **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn with_od(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio28::OD_MASK
            } else {
                self.bits & !Gpio28::OD_MASK
            }
        }
    }

    /// Set **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn with_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio28::IE_MASK
            } else {
                self.bits & !Gpio28::IE_MASK
            }
        }
    }

    /// Set **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn with_drive(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Gpio28::DRIVE_MASK) | ((value << Gpio28::DRIVE_OFFSET) & Gpio28::DRIVE_MASK)
        }
    }

    /// Set **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn with_pue(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio28::PUE_MASK
            } else {
                self.bits & !Gpio28::PUE_MASK
            }
        }
    }

    /// Set **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn with_pde(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio28::PDE_MASK
            } else {
                self.bits & !Gpio28::PDE_MASK
            }
        }
    }

    /// Set **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn with_schmitt(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio28::SCHMITT_MASK
            } else {
                self.bits & !Gpio28::SCHMITT_MASK
            }
        }
    }

    /// Set **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn with_slewfast(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio28::SLEWFAST_MASK
            } else {
                self.bits & !Gpio28::SLEWFAST_MASK
            }
        }
    }

}

impl Default for Gpio28Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Gpio28 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Gpio28Val {
        Gpio28Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Gpio28Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Gpio28Val) -> Gpio28Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// GPIO29 Register
///
/// Address offset: 0x0078
/// Reset value: 0x00000116
///
/// ## Fields
///
/// - **ISO** `[8]` (RW)
///
///   Pad isolation control. Remove this once the pad is configured by software.
///
/// - **OD** `[7]` (RW)
///
///   Output disable. Has priority over output enable from peripherals
///
/// - **IE** `[6]` (RW)
///
///   Input enable
///
/// - **DRIVE** `[5:4]` (RW)
///
///   Drive strength.
///
/// - **PUE** `[3]` (RW)
///
///   Pull up enable
///
/// - **PDE** `[2]` (RW)
///
///   Pull down enable
///
/// - **SCHMITT** `[1]` (RW)
///
///   Enable schmitt trigger
///
/// - **SLEWFAST** `[0]` (RW)
///
///   Slew rate control. 1 = Fast, 0 = Slow
#[repr(transparent)]
pub struct Gpio29 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Gpio29 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Gpio29 {
    // ======== Field Constants ========

    /// **ISO**
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ISO_OFFSET: u32 = 8;
    pub const ISO_MASK: u32 = 0x00000100;
    pub const ISO: u32 = 1 << 8;

    /// **OD**
    ///
    /// Output disable. Has priority over output enable from peripherals
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const OD_OFFSET: u32 = 7;
    pub const OD_MASK: u32 = 0x00000080;
    pub const OD: u32 = 1 << 7;

    /// **IE**
    ///
    /// Input enable
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const IE_OFFSET: u32 = 6;
    pub const IE_MASK: u32 = 0x00000040;
    pub const IE: u32 = 1 << 6;

    /// **DRIVE**
    ///
    /// Drive strength.
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const DRIVE_OFFSET: u32 = 4;
    pub const DRIVE_MASK: u32 = 0x00000030;
    pub const DRIVE_2M_A: u32 = 0 << 4;
    pub const DRIVE_2M_A_VAL: u32 = 0;
    pub const DRIVE_4M_A: u32 = 1 << 4;
    pub const DRIVE_4M_A_VAL: u32 = 1;
    pub const DRIVE_8M_A: u32 = 2 << 4;
    pub const DRIVE_8M_A_VAL: u32 = 2;
    pub const DRIVE_12M_A: u32 = 3 << 4;
    pub const DRIVE_12M_A_VAL: u32 = 3;

    /// **PUE**
    ///
    /// Pull up enable
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const PUE_OFFSET: u32 = 3;
    pub const PUE_MASK: u32 = 0x00000008;
    pub const PUE: u32 = 1 << 3;

    /// **PDE**
    ///
    /// Pull down enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const PDE_OFFSET: u32 = 2;
    pub const PDE_MASK: u32 = 0x00000004;
    pub const PDE: u32 = 1 << 2;

    /// **SCHMITT**
    ///
    /// Enable schmitt trigger
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SCHMITT_OFFSET: u32 = 1;
    pub const SCHMITT_MASK: u32 = 0x00000002;
    pub const SCHMITT: u32 = 1 << 1;

    /// **SLEWFAST**
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SLEWFAST_OFFSET: u32 = 0;
    pub const SLEWFAST_MASK: u32 = 0x00000001;
    pub const SLEWFAST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ISO** `[8]` (RW)
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub fn iso(&self) -> bool {
        (self.value.read_bits() & Self::ISO_MASK) != 0
    }

    /// Set ISO bit atomically (RW)
    #[inline(always)]
    pub fn set_iso(&self) {
        self.set_bits(Self::ISO_MASK);
    }

    /// Clear ISO bit atomically (RW)
    #[inline(always)]
    pub fn clr_iso(&self) {
        self.clr_bits(Self::ISO_MASK);
    }

    /// **OD** `[7]` (RW)
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub fn od(&self) -> bool {
        (self.value.read_bits() & Self::OD_MASK) != 0
    }

    /// Set OD bit atomically (RW)
    #[inline(always)]
    pub fn set_od(&self) {
        self.set_bits(Self::OD_MASK);
    }

    /// Clear OD bit atomically (RW)
    #[inline(always)]
    pub fn clr_od(&self) {
        self.clr_bits(Self::OD_MASK);
    }

    /// **IE** `[6]` (RW)
    ///
    /// Input enable
    #[inline(always)]
    pub fn ie(&self) -> bool {
        (self.value.read_bits() & Self::IE_MASK) != 0
    }

    /// Set IE bit atomically (RW)
    #[inline(always)]
    pub fn set_ie(&self) {
        self.set_bits(Self::IE_MASK);
    }

    /// Clear IE bit atomically (RW)
    #[inline(always)]
    pub fn clr_ie(&self) {
        self.clr_bits(Self::IE_MASK);
    }

    /// **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn drive(&self) -> u32 {
        (self.value.read_bits() & Self::DRIVE_MASK) >> Self::DRIVE_OFFSET
    }

    /// Write **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn write_drive(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DRIVE_MASK) | ((value << Self::DRIVE_OFFSET) & Self::DRIVE_MASK));
    }

    /// **PUE** `[3]` (RW)
    ///
    /// Pull up enable
    #[inline(always)]
    pub fn pue(&self) -> bool {
        (self.value.read_bits() & Self::PUE_MASK) != 0
    }

    /// Set PUE bit atomically (RW)
    #[inline(always)]
    pub fn set_pue(&self) {
        self.set_bits(Self::PUE_MASK);
    }

    /// Clear PUE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pue(&self) {
        self.clr_bits(Self::PUE_MASK);
    }

    /// **PDE** `[2]` (RW)
    ///
    /// Pull down enable
    #[inline(always)]
    pub fn pde(&self) -> bool {
        (self.value.read_bits() & Self::PDE_MASK) != 0
    }

    /// Set PDE bit atomically (RW)
    #[inline(always)]
    pub fn set_pde(&self) {
        self.set_bits(Self::PDE_MASK);
    }

    /// Clear PDE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pde(&self) {
        self.clr_bits(Self::PDE_MASK);
    }

    /// **SCHMITT** `[1]` (RW)
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub fn schmitt(&self) -> bool {
        (self.value.read_bits() & Self::SCHMITT_MASK) != 0
    }

    /// Set SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn set_schmitt(&self) {
        self.set_bits(Self::SCHMITT_MASK);
    }

    /// Clear SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn clr_schmitt(&self) {
        self.clr_bits(Self::SCHMITT_MASK);
    }

    /// **SLEWFAST** `[0]` (RW)
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub fn slewfast(&self) -> bool {
        (self.value.read_bits() & Self::SLEWFAST_MASK) != 0
    }

    /// Set SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn set_slewfast(&self) {
        self.set_bits(Self::SLEWFAST_MASK);
    }

    /// Clear SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn clr_slewfast(&self) {
        self.clr_bits(Self::SLEWFAST_MASK);
    }

}

/// GPIO29 value type for batch read/modify/write operations
///
/// Use `gpio29.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpio29.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Gpio29Val {
    bits: u32,
}

impl Gpio29Val {
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
        Self { bits: 0x00000116 }
    }

    /// Read **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn iso(&self) -> bool {
        (self.bits & Gpio29::ISO_MASK) != 0
    }

    /// Read **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn od(&self) -> bool {
        (self.bits & Gpio29::OD_MASK) != 0
    }

    /// Read **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        (self.bits & Gpio29::IE_MASK) != 0
    }

    /// Read **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn drive(&self) -> u32 {
        (self.bits & Gpio29::DRIVE_MASK) >> Gpio29::DRIVE_OFFSET
    }

    /// Read **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn pue(&self) -> bool {
        (self.bits & Gpio29::PUE_MASK) != 0
    }

    /// Read **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn pde(&self) -> bool {
        (self.bits & Gpio29::PDE_MASK) != 0
    }

    /// Read **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn schmitt(&self) -> bool {
        (self.bits & Gpio29::SCHMITT_MASK) != 0
    }

    /// Read **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn slewfast(&self) -> bool {
        (self.bits & Gpio29::SLEWFAST_MASK) != 0
    }

    /// Set **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn with_iso(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio29::ISO_MASK
            } else {
                self.bits & !Gpio29::ISO_MASK
            }
        }
    }

    /// Set **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn with_od(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio29::OD_MASK
            } else {
                self.bits & !Gpio29::OD_MASK
            }
        }
    }

    /// Set **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn with_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio29::IE_MASK
            } else {
                self.bits & !Gpio29::IE_MASK
            }
        }
    }

    /// Set **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn with_drive(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Gpio29::DRIVE_MASK) | ((value << Gpio29::DRIVE_OFFSET) & Gpio29::DRIVE_MASK)
        }
    }

    /// Set **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn with_pue(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio29::PUE_MASK
            } else {
                self.bits & !Gpio29::PUE_MASK
            }
        }
    }

    /// Set **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn with_pde(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio29::PDE_MASK
            } else {
                self.bits & !Gpio29::PDE_MASK
            }
        }
    }

    /// Set **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn with_schmitt(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio29::SCHMITT_MASK
            } else {
                self.bits & !Gpio29::SCHMITT_MASK
            }
        }
    }

    /// Set **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn with_slewfast(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio29::SLEWFAST_MASK
            } else {
                self.bits & !Gpio29::SLEWFAST_MASK
            }
        }
    }

}

impl Default for Gpio29Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Gpio29 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Gpio29Val {
        Gpio29Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Gpio29Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Gpio29Val) -> Gpio29Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// GPIO30 Register
///
/// Address offset: 0x007c
/// Reset value: 0x00000116
///
/// ## Fields
///
/// - **ISO** `[8]` (RW)
///
///   Pad isolation control. Remove this once the pad is configured by software.
///
/// - **OD** `[7]` (RW)
///
///   Output disable. Has priority over output enable from peripherals
///
/// - **IE** `[6]` (RW)
///
///   Input enable
///
/// - **DRIVE** `[5:4]` (RW)
///
///   Drive strength.
///
/// - **PUE** `[3]` (RW)
///
///   Pull up enable
///
/// - **PDE** `[2]` (RW)
///
///   Pull down enable
///
/// - **SCHMITT** `[1]` (RW)
///
///   Enable schmitt trigger
///
/// - **SLEWFAST** `[0]` (RW)
///
///   Slew rate control. 1 = Fast, 0 = Slow
#[repr(transparent)]
pub struct Gpio30 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Gpio30 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Gpio30 {
    // ======== Field Constants ========

    /// **ISO**
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ISO_OFFSET: u32 = 8;
    pub const ISO_MASK: u32 = 0x00000100;
    pub const ISO: u32 = 1 << 8;

    /// **OD**
    ///
    /// Output disable. Has priority over output enable from peripherals
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const OD_OFFSET: u32 = 7;
    pub const OD_MASK: u32 = 0x00000080;
    pub const OD: u32 = 1 << 7;

    /// **IE**
    ///
    /// Input enable
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const IE_OFFSET: u32 = 6;
    pub const IE_MASK: u32 = 0x00000040;
    pub const IE: u32 = 1 << 6;

    /// **DRIVE**
    ///
    /// Drive strength.
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const DRIVE_OFFSET: u32 = 4;
    pub const DRIVE_MASK: u32 = 0x00000030;
    pub const DRIVE_2M_A: u32 = 0 << 4;
    pub const DRIVE_2M_A_VAL: u32 = 0;
    pub const DRIVE_4M_A: u32 = 1 << 4;
    pub const DRIVE_4M_A_VAL: u32 = 1;
    pub const DRIVE_8M_A: u32 = 2 << 4;
    pub const DRIVE_8M_A_VAL: u32 = 2;
    pub const DRIVE_12M_A: u32 = 3 << 4;
    pub const DRIVE_12M_A_VAL: u32 = 3;

    /// **PUE**
    ///
    /// Pull up enable
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const PUE_OFFSET: u32 = 3;
    pub const PUE_MASK: u32 = 0x00000008;
    pub const PUE: u32 = 1 << 3;

    /// **PDE**
    ///
    /// Pull down enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const PDE_OFFSET: u32 = 2;
    pub const PDE_MASK: u32 = 0x00000004;
    pub const PDE: u32 = 1 << 2;

    /// **SCHMITT**
    ///
    /// Enable schmitt trigger
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SCHMITT_OFFSET: u32 = 1;
    pub const SCHMITT_MASK: u32 = 0x00000002;
    pub const SCHMITT: u32 = 1 << 1;

    /// **SLEWFAST**
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SLEWFAST_OFFSET: u32 = 0;
    pub const SLEWFAST_MASK: u32 = 0x00000001;
    pub const SLEWFAST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ISO** `[8]` (RW)
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub fn iso(&self) -> bool {
        (self.value.read_bits() & Self::ISO_MASK) != 0
    }

    /// Set ISO bit atomically (RW)
    #[inline(always)]
    pub fn set_iso(&self) {
        self.set_bits(Self::ISO_MASK);
    }

    /// Clear ISO bit atomically (RW)
    #[inline(always)]
    pub fn clr_iso(&self) {
        self.clr_bits(Self::ISO_MASK);
    }

    /// **OD** `[7]` (RW)
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub fn od(&self) -> bool {
        (self.value.read_bits() & Self::OD_MASK) != 0
    }

    /// Set OD bit atomically (RW)
    #[inline(always)]
    pub fn set_od(&self) {
        self.set_bits(Self::OD_MASK);
    }

    /// Clear OD bit atomically (RW)
    #[inline(always)]
    pub fn clr_od(&self) {
        self.clr_bits(Self::OD_MASK);
    }

    /// **IE** `[6]` (RW)
    ///
    /// Input enable
    #[inline(always)]
    pub fn ie(&self) -> bool {
        (self.value.read_bits() & Self::IE_MASK) != 0
    }

    /// Set IE bit atomically (RW)
    #[inline(always)]
    pub fn set_ie(&self) {
        self.set_bits(Self::IE_MASK);
    }

    /// Clear IE bit atomically (RW)
    #[inline(always)]
    pub fn clr_ie(&self) {
        self.clr_bits(Self::IE_MASK);
    }

    /// **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn drive(&self) -> u32 {
        (self.value.read_bits() & Self::DRIVE_MASK) >> Self::DRIVE_OFFSET
    }

    /// Write **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn write_drive(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DRIVE_MASK) | ((value << Self::DRIVE_OFFSET) & Self::DRIVE_MASK));
    }

    /// **PUE** `[3]` (RW)
    ///
    /// Pull up enable
    #[inline(always)]
    pub fn pue(&self) -> bool {
        (self.value.read_bits() & Self::PUE_MASK) != 0
    }

    /// Set PUE bit atomically (RW)
    #[inline(always)]
    pub fn set_pue(&self) {
        self.set_bits(Self::PUE_MASK);
    }

    /// Clear PUE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pue(&self) {
        self.clr_bits(Self::PUE_MASK);
    }

    /// **PDE** `[2]` (RW)
    ///
    /// Pull down enable
    #[inline(always)]
    pub fn pde(&self) -> bool {
        (self.value.read_bits() & Self::PDE_MASK) != 0
    }

    /// Set PDE bit atomically (RW)
    #[inline(always)]
    pub fn set_pde(&self) {
        self.set_bits(Self::PDE_MASK);
    }

    /// Clear PDE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pde(&self) {
        self.clr_bits(Self::PDE_MASK);
    }

    /// **SCHMITT** `[1]` (RW)
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub fn schmitt(&self) -> bool {
        (self.value.read_bits() & Self::SCHMITT_MASK) != 0
    }

    /// Set SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn set_schmitt(&self) {
        self.set_bits(Self::SCHMITT_MASK);
    }

    /// Clear SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn clr_schmitt(&self) {
        self.clr_bits(Self::SCHMITT_MASK);
    }

    /// **SLEWFAST** `[0]` (RW)
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub fn slewfast(&self) -> bool {
        (self.value.read_bits() & Self::SLEWFAST_MASK) != 0
    }

    /// Set SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn set_slewfast(&self) {
        self.set_bits(Self::SLEWFAST_MASK);
    }

    /// Clear SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn clr_slewfast(&self) {
        self.clr_bits(Self::SLEWFAST_MASK);
    }

}

/// GPIO30 value type for batch read/modify/write operations
///
/// Use `gpio30.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpio30.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Gpio30Val {
    bits: u32,
}

impl Gpio30Val {
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
        Self { bits: 0x00000116 }
    }

    /// Read **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn iso(&self) -> bool {
        (self.bits & Gpio30::ISO_MASK) != 0
    }

    /// Read **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn od(&self) -> bool {
        (self.bits & Gpio30::OD_MASK) != 0
    }

    /// Read **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        (self.bits & Gpio30::IE_MASK) != 0
    }

    /// Read **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn drive(&self) -> u32 {
        (self.bits & Gpio30::DRIVE_MASK) >> Gpio30::DRIVE_OFFSET
    }

    /// Read **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn pue(&self) -> bool {
        (self.bits & Gpio30::PUE_MASK) != 0
    }

    /// Read **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn pde(&self) -> bool {
        (self.bits & Gpio30::PDE_MASK) != 0
    }

    /// Read **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn schmitt(&self) -> bool {
        (self.bits & Gpio30::SCHMITT_MASK) != 0
    }

    /// Read **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn slewfast(&self) -> bool {
        (self.bits & Gpio30::SLEWFAST_MASK) != 0
    }

    /// Set **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn with_iso(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio30::ISO_MASK
            } else {
                self.bits & !Gpio30::ISO_MASK
            }
        }
    }

    /// Set **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn with_od(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio30::OD_MASK
            } else {
                self.bits & !Gpio30::OD_MASK
            }
        }
    }

    /// Set **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn with_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio30::IE_MASK
            } else {
                self.bits & !Gpio30::IE_MASK
            }
        }
    }

    /// Set **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn with_drive(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Gpio30::DRIVE_MASK) | ((value << Gpio30::DRIVE_OFFSET) & Gpio30::DRIVE_MASK)
        }
    }

    /// Set **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn with_pue(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio30::PUE_MASK
            } else {
                self.bits & !Gpio30::PUE_MASK
            }
        }
    }

    /// Set **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn with_pde(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio30::PDE_MASK
            } else {
                self.bits & !Gpio30::PDE_MASK
            }
        }
    }

    /// Set **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn with_schmitt(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio30::SCHMITT_MASK
            } else {
                self.bits & !Gpio30::SCHMITT_MASK
            }
        }
    }

    /// Set **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn with_slewfast(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio30::SLEWFAST_MASK
            } else {
                self.bits & !Gpio30::SLEWFAST_MASK
            }
        }
    }

}

impl Default for Gpio30Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Gpio30 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Gpio30Val {
        Gpio30Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Gpio30Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Gpio30Val) -> Gpio30Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// GPIO31 Register
///
/// Address offset: 0x0080
/// Reset value: 0x00000116
///
/// ## Fields
///
/// - **ISO** `[8]` (RW)
///
///   Pad isolation control. Remove this once the pad is configured by software.
///
/// - **OD** `[7]` (RW)
///
///   Output disable. Has priority over output enable from peripherals
///
/// - **IE** `[6]` (RW)
///
///   Input enable
///
/// - **DRIVE** `[5:4]` (RW)
///
///   Drive strength.
///
/// - **PUE** `[3]` (RW)
///
///   Pull up enable
///
/// - **PDE** `[2]` (RW)
///
///   Pull down enable
///
/// - **SCHMITT** `[1]` (RW)
///
///   Enable schmitt trigger
///
/// - **SLEWFAST** `[0]` (RW)
///
///   Slew rate control. 1 = Fast, 0 = Slow
#[repr(transparent)]
pub struct Gpio31 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Gpio31 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Gpio31 {
    // ======== Field Constants ========

    /// **ISO**
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ISO_OFFSET: u32 = 8;
    pub const ISO_MASK: u32 = 0x00000100;
    pub const ISO: u32 = 1 << 8;

    /// **OD**
    ///
    /// Output disable. Has priority over output enable from peripherals
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const OD_OFFSET: u32 = 7;
    pub const OD_MASK: u32 = 0x00000080;
    pub const OD: u32 = 1 << 7;

    /// **IE**
    ///
    /// Input enable
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const IE_OFFSET: u32 = 6;
    pub const IE_MASK: u32 = 0x00000040;
    pub const IE: u32 = 1 << 6;

    /// **DRIVE**
    ///
    /// Drive strength.
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const DRIVE_OFFSET: u32 = 4;
    pub const DRIVE_MASK: u32 = 0x00000030;
    pub const DRIVE_2M_A: u32 = 0 << 4;
    pub const DRIVE_2M_A_VAL: u32 = 0;
    pub const DRIVE_4M_A: u32 = 1 << 4;
    pub const DRIVE_4M_A_VAL: u32 = 1;
    pub const DRIVE_8M_A: u32 = 2 << 4;
    pub const DRIVE_8M_A_VAL: u32 = 2;
    pub const DRIVE_12M_A: u32 = 3 << 4;
    pub const DRIVE_12M_A_VAL: u32 = 3;

    /// **PUE**
    ///
    /// Pull up enable
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const PUE_OFFSET: u32 = 3;
    pub const PUE_MASK: u32 = 0x00000008;
    pub const PUE: u32 = 1 << 3;

    /// **PDE**
    ///
    /// Pull down enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const PDE_OFFSET: u32 = 2;
    pub const PDE_MASK: u32 = 0x00000004;
    pub const PDE: u32 = 1 << 2;

    /// **SCHMITT**
    ///
    /// Enable schmitt trigger
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SCHMITT_OFFSET: u32 = 1;
    pub const SCHMITT_MASK: u32 = 0x00000002;
    pub const SCHMITT: u32 = 1 << 1;

    /// **SLEWFAST**
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SLEWFAST_OFFSET: u32 = 0;
    pub const SLEWFAST_MASK: u32 = 0x00000001;
    pub const SLEWFAST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ISO** `[8]` (RW)
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub fn iso(&self) -> bool {
        (self.value.read_bits() & Self::ISO_MASK) != 0
    }

    /// Set ISO bit atomically (RW)
    #[inline(always)]
    pub fn set_iso(&self) {
        self.set_bits(Self::ISO_MASK);
    }

    /// Clear ISO bit atomically (RW)
    #[inline(always)]
    pub fn clr_iso(&self) {
        self.clr_bits(Self::ISO_MASK);
    }

    /// **OD** `[7]` (RW)
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub fn od(&self) -> bool {
        (self.value.read_bits() & Self::OD_MASK) != 0
    }

    /// Set OD bit atomically (RW)
    #[inline(always)]
    pub fn set_od(&self) {
        self.set_bits(Self::OD_MASK);
    }

    /// Clear OD bit atomically (RW)
    #[inline(always)]
    pub fn clr_od(&self) {
        self.clr_bits(Self::OD_MASK);
    }

    /// **IE** `[6]` (RW)
    ///
    /// Input enable
    #[inline(always)]
    pub fn ie(&self) -> bool {
        (self.value.read_bits() & Self::IE_MASK) != 0
    }

    /// Set IE bit atomically (RW)
    #[inline(always)]
    pub fn set_ie(&self) {
        self.set_bits(Self::IE_MASK);
    }

    /// Clear IE bit atomically (RW)
    #[inline(always)]
    pub fn clr_ie(&self) {
        self.clr_bits(Self::IE_MASK);
    }

    /// **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn drive(&self) -> u32 {
        (self.value.read_bits() & Self::DRIVE_MASK) >> Self::DRIVE_OFFSET
    }

    /// Write **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn write_drive(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DRIVE_MASK) | ((value << Self::DRIVE_OFFSET) & Self::DRIVE_MASK));
    }

    /// **PUE** `[3]` (RW)
    ///
    /// Pull up enable
    #[inline(always)]
    pub fn pue(&self) -> bool {
        (self.value.read_bits() & Self::PUE_MASK) != 0
    }

    /// Set PUE bit atomically (RW)
    #[inline(always)]
    pub fn set_pue(&self) {
        self.set_bits(Self::PUE_MASK);
    }

    /// Clear PUE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pue(&self) {
        self.clr_bits(Self::PUE_MASK);
    }

    /// **PDE** `[2]` (RW)
    ///
    /// Pull down enable
    #[inline(always)]
    pub fn pde(&self) -> bool {
        (self.value.read_bits() & Self::PDE_MASK) != 0
    }

    /// Set PDE bit atomically (RW)
    #[inline(always)]
    pub fn set_pde(&self) {
        self.set_bits(Self::PDE_MASK);
    }

    /// Clear PDE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pde(&self) {
        self.clr_bits(Self::PDE_MASK);
    }

    /// **SCHMITT** `[1]` (RW)
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub fn schmitt(&self) -> bool {
        (self.value.read_bits() & Self::SCHMITT_MASK) != 0
    }

    /// Set SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn set_schmitt(&self) {
        self.set_bits(Self::SCHMITT_MASK);
    }

    /// Clear SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn clr_schmitt(&self) {
        self.clr_bits(Self::SCHMITT_MASK);
    }

    /// **SLEWFAST** `[0]` (RW)
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub fn slewfast(&self) -> bool {
        (self.value.read_bits() & Self::SLEWFAST_MASK) != 0
    }

    /// Set SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn set_slewfast(&self) {
        self.set_bits(Self::SLEWFAST_MASK);
    }

    /// Clear SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn clr_slewfast(&self) {
        self.clr_bits(Self::SLEWFAST_MASK);
    }

}

/// GPIO31 value type for batch read/modify/write operations
///
/// Use `gpio31.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpio31.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Gpio31Val {
    bits: u32,
}

impl Gpio31Val {
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
        Self { bits: 0x00000116 }
    }

    /// Read **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn iso(&self) -> bool {
        (self.bits & Gpio31::ISO_MASK) != 0
    }

    /// Read **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn od(&self) -> bool {
        (self.bits & Gpio31::OD_MASK) != 0
    }

    /// Read **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        (self.bits & Gpio31::IE_MASK) != 0
    }

    /// Read **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn drive(&self) -> u32 {
        (self.bits & Gpio31::DRIVE_MASK) >> Gpio31::DRIVE_OFFSET
    }

    /// Read **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn pue(&self) -> bool {
        (self.bits & Gpio31::PUE_MASK) != 0
    }

    /// Read **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn pde(&self) -> bool {
        (self.bits & Gpio31::PDE_MASK) != 0
    }

    /// Read **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn schmitt(&self) -> bool {
        (self.bits & Gpio31::SCHMITT_MASK) != 0
    }

    /// Read **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn slewfast(&self) -> bool {
        (self.bits & Gpio31::SLEWFAST_MASK) != 0
    }

    /// Set **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn with_iso(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio31::ISO_MASK
            } else {
                self.bits & !Gpio31::ISO_MASK
            }
        }
    }

    /// Set **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn with_od(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio31::OD_MASK
            } else {
                self.bits & !Gpio31::OD_MASK
            }
        }
    }

    /// Set **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn with_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio31::IE_MASK
            } else {
                self.bits & !Gpio31::IE_MASK
            }
        }
    }

    /// Set **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn with_drive(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Gpio31::DRIVE_MASK) | ((value << Gpio31::DRIVE_OFFSET) & Gpio31::DRIVE_MASK)
        }
    }

    /// Set **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn with_pue(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio31::PUE_MASK
            } else {
                self.bits & !Gpio31::PUE_MASK
            }
        }
    }

    /// Set **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn with_pde(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio31::PDE_MASK
            } else {
                self.bits & !Gpio31::PDE_MASK
            }
        }
    }

    /// Set **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn with_schmitt(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio31::SCHMITT_MASK
            } else {
                self.bits & !Gpio31::SCHMITT_MASK
            }
        }
    }

    /// Set **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn with_slewfast(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio31::SLEWFAST_MASK
            } else {
                self.bits & !Gpio31::SLEWFAST_MASK
            }
        }
    }

}

impl Default for Gpio31Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Gpio31 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Gpio31Val {
        Gpio31Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Gpio31Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Gpio31Val) -> Gpio31Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// GPIO32 Register
///
/// Address offset: 0x0084
/// Reset value: 0x00000116
///
/// ## Fields
///
/// - **ISO** `[8]` (RW)
///
///   Pad isolation control. Remove this once the pad is configured by software.
///
/// - **OD** `[7]` (RW)
///
///   Output disable. Has priority over output enable from peripherals
///
/// - **IE** `[6]` (RW)
///
///   Input enable
///
/// - **DRIVE** `[5:4]` (RW)
///
///   Drive strength.
///
/// - **PUE** `[3]` (RW)
///
///   Pull up enable
///
/// - **PDE** `[2]` (RW)
///
///   Pull down enable
///
/// - **SCHMITT** `[1]` (RW)
///
///   Enable schmitt trigger
///
/// - **SLEWFAST** `[0]` (RW)
///
///   Slew rate control. 1 = Fast, 0 = Slow
#[repr(transparent)]
pub struct Gpio32 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Gpio32 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Gpio32 {
    // ======== Field Constants ========

    /// **ISO**
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ISO_OFFSET: u32 = 8;
    pub const ISO_MASK: u32 = 0x00000100;
    pub const ISO: u32 = 1 << 8;

    /// **OD**
    ///
    /// Output disable. Has priority over output enable from peripherals
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const OD_OFFSET: u32 = 7;
    pub const OD_MASK: u32 = 0x00000080;
    pub const OD: u32 = 1 << 7;

    /// **IE**
    ///
    /// Input enable
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const IE_OFFSET: u32 = 6;
    pub const IE_MASK: u32 = 0x00000040;
    pub const IE: u32 = 1 << 6;

    /// **DRIVE**
    ///
    /// Drive strength.
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const DRIVE_OFFSET: u32 = 4;
    pub const DRIVE_MASK: u32 = 0x00000030;
    pub const DRIVE_2M_A: u32 = 0 << 4;
    pub const DRIVE_2M_A_VAL: u32 = 0;
    pub const DRIVE_4M_A: u32 = 1 << 4;
    pub const DRIVE_4M_A_VAL: u32 = 1;
    pub const DRIVE_8M_A: u32 = 2 << 4;
    pub const DRIVE_8M_A_VAL: u32 = 2;
    pub const DRIVE_12M_A: u32 = 3 << 4;
    pub const DRIVE_12M_A_VAL: u32 = 3;

    /// **PUE**
    ///
    /// Pull up enable
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const PUE_OFFSET: u32 = 3;
    pub const PUE_MASK: u32 = 0x00000008;
    pub const PUE: u32 = 1 << 3;

    /// **PDE**
    ///
    /// Pull down enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const PDE_OFFSET: u32 = 2;
    pub const PDE_MASK: u32 = 0x00000004;
    pub const PDE: u32 = 1 << 2;

    /// **SCHMITT**
    ///
    /// Enable schmitt trigger
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SCHMITT_OFFSET: u32 = 1;
    pub const SCHMITT_MASK: u32 = 0x00000002;
    pub const SCHMITT: u32 = 1 << 1;

    /// **SLEWFAST**
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SLEWFAST_OFFSET: u32 = 0;
    pub const SLEWFAST_MASK: u32 = 0x00000001;
    pub const SLEWFAST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ISO** `[8]` (RW)
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub fn iso(&self) -> bool {
        (self.value.read_bits() & Self::ISO_MASK) != 0
    }

    /// Set ISO bit atomically (RW)
    #[inline(always)]
    pub fn set_iso(&self) {
        self.set_bits(Self::ISO_MASK);
    }

    /// Clear ISO bit atomically (RW)
    #[inline(always)]
    pub fn clr_iso(&self) {
        self.clr_bits(Self::ISO_MASK);
    }

    /// **OD** `[7]` (RW)
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub fn od(&self) -> bool {
        (self.value.read_bits() & Self::OD_MASK) != 0
    }

    /// Set OD bit atomically (RW)
    #[inline(always)]
    pub fn set_od(&self) {
        self.set_bits(Self::OD_MASK);
    }

    /// Clear OD bit atomically (RW)
    #[inline(always)]
    pub fn clr_od(&self) {
        self.clr_bits(Self::OD_MASK);
    }

    /// **IE** `[6]` (RW)
    ///
    /// Input enable
    #[inline(always)]
    pub fn ie(&self) -> bool {
        (self.value.read_bits() & Self::IE_MASK) != 0
    }

    /// Set IE bit atomically (RW)
    #[inline(always)]
    pub fn set_ie(&self) {
        self.set_bits(Self::IE_MASK);
    }

    /// Clear IE bit atomically (RW)
    #[inline(always)]
    pub fn clr_ie(&self) {
        self.clr_bits(Self::IE_MASK);
    }

    /// **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn drive(&self) -> u32 {
        (self.value.read_bits() & Self::DRIVE_MASK) >> Self::DRIVE_OFFSET
    }

    /// Write **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn write_drive(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DRIVE_MASK) | ((value << Self::DRIVE_OFFSET) & Self::DRIVE_MASK));
    }

    /// **PUE** `[3]` (RW)
    ///
    /// Pull up enable
    #[inline(always)]
    pub fn pue(&self) -> bool {
        (self.value.read_bits() & Self::PUE_MASK) != 0
    }

    /// Set PUE bit atomically (RW)
    #[inline(always)]
    pub fn set_pue(&self) {
        self.set_bits(Self::PUE_MASK);
    }

    /// Clear PUE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pue(&self) {
        self.clr_bits(Self::PUE_MASK);
    }

    /// **PDE** `[2]` (RW)
    ///
    /// Pull down enable
    #[inline(always)]
    pub fn pde(&self) -> bool {
        (self.value.read_bits() & Self::PDE_MASK) != 0
    }

    /// Set PDE bit atomically (RW)
    #[inline(always)]
    pub fn set_pde(&self) {
        self.set_bits(Self::PDE_MASK);
    }

    /// Clear PDE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pde(&self) {
        self.clr_bits(Self::PDE_MASK);
    }

    /// **SCHMITT** `[1]` (RW)
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub fn schmitt(&self) -> bool {
        (self.value.read_bits() & Self::SCHMITT_MASK) != 0
    }

    /// Set SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn set_schmitt(&self) {
        self.set_bits(Self::SCHMITT_MASK);
    }

    /// Clear SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn clr_schmitt(&self) {
        self.clr_bits(Self::SCHMITT_MASK);
    }

    /// **SLEWFAST** `[0]` (RW)
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub fn slewfast(&self) -> bool {
        (self.value.read_bits() & Self::SLEWFAST_MASK) != 0
    }

    /// Set SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn set_slewfast(&self) {
        self.set_bits(Self::SLEWFAST_MASK);
    }

    /// Clear SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn clr_slewfast(&self) {
        self.clr_bits(Self::SLEWFAST_MASK);
    }

}

/// GPIO32 value type for batch read/modify/write operations
///
/// Use `gpio32.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpio32.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Gpio32Val {
    bits: u32,
}

impl Gpio32Val {
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
        Self { bits: 0x00000116 }
    }

    /// Read **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn iso(&self) -> bool {
        (self.bits & Gpio32::ISO_MASK) != 0
    }

    /// Read **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn od(&self) -> bool {
        (self.bits & Gpio32::OD_MASK) != 0
    }

    /// Read **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        (self.bits & Gpio32::IE_MASK) != 0
    }

    /// Read **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn drive(&self) -> u32 {
        (self.bits & Gpio32::DRIVE_MASK) >> Gpio32::DRIVE_OFFSET
    }

    /// Read **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn pue(&self) -> bool {
        (self.bits & Gpio32::PUE_MASK) != 0
    }

    /// Read **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn pde(&self) -> bool {
        (self.bits & Gpio32::PDE_MASK) != 0
    }

    /// Read **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn schmitt(&self) -> bool {
        (self.bits & Gpio32::SCHMITT_MASK) != 0
    }

    /// Read **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn slewfast(&self) -> bool {
        (self.bits & Gpio32::SLEWFAST_MASK) != 0
    }

    /// Set **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn with_iso(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio32::ISO_MASK
            } else {
                self.bits & !Gpio32::ISO_MASK
            }
        }
    }

    /// Set **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn with_od(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio32::OD_MASK
            } else {
                self.bits & !Gpio32::OD_MASK
            }
        }
    }

    /// Set **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn with_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio32::IE_MASK
            } else {
                self.bits & !Gpio32::IE_MASK
            }
        }
    }

    /// Set **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn with_drive(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Gpio32::DRIVE_MASK) | ((value << Gpio32::DRIVE_OFFSET) & Gpio32::DRIVE_MASK)
        }
    }

    /// Set **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn with_pue(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio32::PUE_MASK
            } else {
                self.bits & !Gpio32::PUE_MASK
            }
        }
    }

    /// Set **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn with_pde(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio32::PDE_MASK
            } else {
                self.bits & !Gpio32::PDE_MASK
            }
        }
    }

    /// Set **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn with_schmitt(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio32::SCHMITT_MASK
            } else {
                self.bits & !Gpio32::SCHMITT_MASK
            }
        }
    }

    /// Set **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn with_slewfast(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio32::SLEWFAST_MASK
            } else {
                self.bits & !Gpio32::SLEWFAST_MASK
            }
        }
    }

}

impl Default for Gpio32Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Gpio32 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Gpio32Val {
        Gpio32Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Gpio32Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Gpio32Val) -> Gpio32Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// GPIO33 Register
///
/// Address offset: 0x0088
/// Reset value: 0x00000116
///
/// ## Fields
///
/// - **ISO** `[8]` (RW)
///
///   Pad isolation control. Remove this once the pad is configured by software.
///
/// - **OD** `[7]` (RW)
///
///   Output disable. Has priority over output enable from peripherals
///
/// - **IE** `[6]` (RW)
///
///   Input enable
///
/// - **DRIVE** `[5:4]` (RW)
///
///   Drive strength.
///
/// - **PUE** `[3]` (RW)
///
///   Pull up enable
///
/// - **PDE** `[2]` (RW)
///
///   Pull down enable
///
/// - **SCHMITT** `[1]` (RW)
///
///   Enable schmitt trigger
///
/// - **SLEWFAST** `[0]` (RW)
///
///   Slew rate control. 1 = Fast, 0 = Slow
#[repr(transparent)]
pub struct Gpio33 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Gpio33 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Gpio33 {
    // ======== Field Constants ========

    /// **ISO**
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ISO_OFFSET: u32 = 8;
    pub const ISO_MASK: u32 = 0x00000100;
    pub const ISO: u32 = 1 << 8;

    /// **OD**
    ///
    /// Output disable. Has priority over output enable from peripherals
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const OD_OFFSET: u32 = 7;
    pub const OD_MASK: u32 = 0x00000080;
    pub const OD: u32 = 1 << 7;

    /// **IE**
    ///
    /// Input enable
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const IE_OFFSET: u32 = 6;
    pub const IE_MASK: u32 = 0x00000040;
    pub const IE: u32 = 1 << 6;

    /// **DRIVE**
    ///
    /// Drive strength.
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const DRIVE_OFFSET: u32 = 4;
    pub const DRIVE_MASK: u32 = 0x00000030;
    pub const DRIVE_2M_A: u32 = 0 << 4;
    pub const DRIVE_2M_A_VAL: u32 = 0;
    pub const DRIVE_4M_A: u32 = 1 << 4;
    pub const DRIVE_4M_A_VAL: u32 = 1;
    pub const DRIVE_8M_A: u32 = 2 << 4;
    pub const DRIVE_8M_A_VAL: u32 = 2;
    pub const DRIVE_12M_A: u32 = 3 << 4;
    pub const DRIVE_12M_A_VAL: u32 = 3;

    /// **PUE**
    ///
    /// Pull up enable
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const PUE_OFFSET: u32 = 3;
    pub const PUE_MASK: u32 = 0x00000008;
    pub const PUE: u32 = 1 << 3;

    /// **PDE**
    ///
    /// Pull down enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const PDE_OFFSET: u32 = 2;
    pub const PDE_MASK: u32 = 0x00000004;
    pub const PDE: u32 = 1 << 2;

    /// **SCHMITT**
    ///
    /// Enable schmitt trigger
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SCHMITT_OFFSET: u32 = 1;
    pub const SCHMITT_MASK: u32 = 0x00000002;
    pub const SCHMITT: u32 = 1 << 1;

    /// **SLEWFAST**
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SLEWFAST_OFFSET: u32 = 0;
    pub const SLEWFAST_MASK: u32 = 0x00000001;
    pub const SLEWFAST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ISO** `[8]` (RW)
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub fn iso(&self) -> bool {
        (self.value.read_bits() & Self::ISO_MASK) != 0
    }

    /// Set ISO bit atomically (RW)
    #[inline(always)]
    pub fn set_iso(&self) {
        self.set_bits(Self::ISO_MASK);
    }

    /// Clear ISO bit atomically (RW)
    #[inline(always)]
    pub fn clr_iso(&self) {
        self.clr_bits(Self::ISO_MASK);
    }

    /// **OD** `[7]` (RW)
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub fn od(&self) -> bool {
        (self.value.read_bits() & Self::OD_MASK) != 0
    }

    /// Set OD bit atomically (RW)
    #[inline(always)]
    pub fn set_od(&self) {
        self.set_bits(Self::OD_MASK);
    }

    /// Clear OD bit atomically (RW)
    #[inline(always)]
    pub fn clr_od(&self) {
        self.clr_bits(Self::OD_MASK);
    }

    /// **IE** `[6]` (RW)
    ///
    /// Input enable
    #[inline(always)]
    pub fn ie(&self) -> bool {
        (self.value.read_bits() & Self::IE_MASK) != 0
    }

    /// Set IE bit atomically (RW)
    #[inline(always)]
    pub fn set_ie(&self) {
        self.set_bits(Self::IE_MASK);
    }

    /// Clear IE bit atomically (RW)
    #[inline(always)]
    pub fn clr_ie(&self) {
        self.clr_bits(Self::IE_MASK);
    }

    /// **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn drive(&self) -> u32 {
        (self.value.read_bits() & Self::DRIVE_MASK) >> Self::DRIVE_OFFSET
    }

    /// Write **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn write_drive(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DRIVE_MASK) | ((value << Self::DRIVE_OFFSET) & Self::DRIVE_MASK));
    }

    /// **PUE** `[3]` (RW)
    ///
    /// Pull up enable
    #[inline(always)]
    pub fn pue(&self) -> bool {
        (self.value.read_bits() & Self::PUE_MASK) != 0
    }

    /// Set PUE bit atomically (RW)
    #[inline(always)]
    pub fn set_pue(&self) {
        self.set_bits(Self::PUE_MASK);
    }

    /// Clear PUE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pue(&self) {
        self.clr_bits(Self::PUE_MASK);
    }

    /// **PDE** `[2]` (RW)
    ///
    /// Pull down enable
    #[inline(always)]
    pub fn pde(&self) -> bool {
        (self.value.read_bits() & Self::PDE_MASK) != 0
    }

    /// Set PDE bit atomically (RW)
    #[inline(always)]
    pub fn set_pde(&self) {
        self.set_bits(Self::PDE_MASK);
    }

    /// Clear PDE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pde(&self) {
        self.clr_bits(Self::PDE_MASK);
    }

    /// **SCHMITT** `[1]` (RW)
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub fn schmitt(&self) -> bool {
        (self.value.read_bits() & Self::SCHMITT_MASK) != 0
    }

    /// Set SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn set_schmitt(&self) {
        self.set_bits(Self::SCHMITT_MASK);
    }

    /// Clear SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn clr_schmitt(&self) {
        self.clr_bits(Self::SCHMITT_MASK);
    }

    /// **SLEWFAST** `[0]` (RW)
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub fn slewfast(&self) -> bool {
        (self.value.read_bits() & Self::SLEWFAST_MASK) != 0
    }

    /// Set SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn set_slewfast(&self) {
        self.set_bits(Self::SLEWFAST_MASK);
    }

    /// Clear SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn clr_slewfast(&self) {
        self.clr_bits(Self::SLEWFAST_MASK);
    }

}

/// GPIO33 value type for batch read/modify/write operations
///
/// Use `gpio33.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpio33.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Gpio33Val {
    bits: u32,
}

impl Gpio33Val {
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
        Self { bits: 0x00000116 }
    }

    /// Read **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn iso(&self) -> bool {
        (self.bits & Gpio33::ISO_MASK) != 0
    }

    /// Read **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn od(&self) -> bool {
        (self.bits & Gpio33::OD_MASK) != 0
    }

    /// Read **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        (self.bits & Gpio33::IE_MASK) != 0
    }

    /// Read **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn drive(&self) -> u32 {
        (self.bits & Gpio33::DRIVE_MASK) >> Gpio33::DRIVE_OFFSET
    }

    /// Read **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn pue(&self) -> bool {
        (self.bits & Gpio33::PUE_MASK) != 0
    }

    /// Read **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn pde(&self) -> bool {
        (self.bits & Gpio33::PDE_MASK) != 0
    }

    /// Read **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn schmitt(&self) -> bool {
        (self.bits & Gpio33::SCHMITT_MASK) != 0
    }

    /// Read **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn slewfast(&self) -> bool {
        (self.bits & Gpio33::SLEWFAST_MASK) != 0
    }

    /// Set **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn with_iso(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio33::ISO_MASK
            } else {
                self.bits & !Gpio33::ISO_MASK
            }
        }
    }

    /// Set **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn with_od(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio33::OD_MASK
            } else {
                self.bits & !Gpio33::OD_MASK
            }
        }
    }

    /// Set **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn with_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio33::IE_MASK
            } else {
                self.bits & !Gpio33::IE_MASK
            }
        }
    }

    /// Set **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn with_drive(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Gpio33::DRIVE_MASK) | ((value << Gpio33::DRIVE_OFFSET) & Gpio33::DRIVE_MASK)
        }
    }

    /// Set **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn with_pue(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio33::PUE_MASK
            } else {
                self.bits & !Gpio33::PUE_MASK
            }
        }
    }

    /// Set **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn with_pde(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio33::PDE_MASK
            } else {
                self.bits & !Gpio33::PDE_MASK
            }
        }
    }

    /// Set **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn with_schmitt(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio33::SCHMITT_MASK
            } else {
                self.bits & !Gpio33::SCHMITT_MASK
            }
        }
    }

    /// Set **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn with_slewfast(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio33::SLEWFAST_MASK
            } else {
                self.bits & !Gpio33::SLEWFAST_MASK
            }
        }
    }

}

impl Default for Gpio33Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Gpio33 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Gpio33Val {
        Gpio33Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Gpio33Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Gpio33Val) -> Gpio33Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// GPIO34 Register
///
/// Address offset: 0x008c
/// Reset value: 0x00000116
///
/// ## Fields
///
/// - **ISO** `[8]` (RW)
///
///   Pad isolation control. Remove this once the pad is configured by software.
///
/// - **OD** `[7]` (RW)
///
///   Output disable. Has priority over output enable from peripherals
///
/// - **IE** `[6]` (RW)
///
///   Input enable
///
/// - **DRIVE** `[5:4]` (RW)
///
///   Drive strength.
///
/// - **PUE** `[3]` (RW)
///
///   Pull up enable
///
/// - **PDE** `[2]` (RW)
///
///   Pull down enable
///
/// - **SCHMITT** `[1]` (RW)
///
///   Enable schmitt trigger
///
/// - **SLEWFAST** `[0]` (RW)
///
///   Slew rate control. 1 = Fast, 0 = Slow
#[repr(transparent)]
pub struct Gpio34 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Gpio34 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Gpio34 {
    // ======== Field Constants ========

    /// **ISO**
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ISO_OFFSET: u32 = 8;
    pub const ISO_MASK: u32 = 0x00000100;
    pub const ISO: u32 = 1 << 8;

    /// **OD**
    ///
    /// Output disable. Has priority over output enable from peripherals
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const OD_OFFSET: u32 = 7;
    pub const OD_MASK: u32 = 0x00000080;
    pub const OD: u32 = 1 << 7;

    /// **IE**
    ///
    /// Input enable
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const IE_OFFSET: u32 = 6;
    pub const IE_MASK: u32 = 0x00000040;
    pub const IE: u32 = 1 << 6;

    /// **DRIVE**
    ///
    /// Drive strength.
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const DRIVE_OFFSET: u32 = 4;
    pub const DRIVE_MASK: u32 = 0x00000030;
    pub const DRIVE_2M_A: u32 = 0 << 4;
    pub const DRIVE_2M_A_VAL: u32 = 0;
    pub const DRIVE_4M_A: u32 = 1 << 4;
    pub const DRIVE_4M_A_VAL: u32 = 1;
    pub const DRIVE_8M_A: u32 = 2 << 4;
    pub const DRIVE_8M_A_VAL: u32 = 2;
    pub const DRIVE_12M_A: u32 = 3 << 4;
    pub const DRIVE_12M_A_VAL: u32 = 3;

    /// **PUE**
    ///
    /// Pull up enable
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const PUE_OFFSET: u32 = 3;
    pub const PUE_MASK: u32 = 0x00000008;
    pub const PUE: u32 = 1 << 3;

    /// **PDE**
    ///
    /// Pull down enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const PDE_OFFSET: u32 = 2;
    pub const PDE_MASK: u32 = 0x00000004;
    pub const PDE: u32 = 1 << 2;

    /// **SCHMITT**
    ///
    /// Enable schmitt trigger
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SCHMITT_OFFSET: u32 = 1;
    pub const SCHMITT_MASK: u32 = 0x00000002;
    pub const SCHMITT: u32 = 1 << 1;

    /// **SLEWFAST**
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SLEWFAST_OFFSET: u32 = 0;
    pub const SLEWFAST_MASK: u32 = 0x00000001;
    pub const SLEWFAST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ISO** `[8]` (RW)
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub fn iso(&self) -> bool {
        (self.value.read_bits() & Self::ISO_MASK) != 0
    }

    /// Set ISO bit atomically (RW)
    #[inline(always)]
    pub fn set_iso(&self) {
        self.set_bits(Self::ISO_MASK);
    }

    /// Clear ISO bit atomically (RW)
    #[inline(always)]
    pub fn clr_iso(&self) {
        self.clr_bits(Self::ISO_MASK);
    }

    /// **OD** `[7]` (RW)
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub fn od(&self) -> bool {
        (self.value.read_bits() & Self::OD_MASK) != 0
    }

    /// Set OD bit atomically (RW)
    #[inline(always)]
    pub fn set_od(&self) {
        self.set_bits(Self::OD_MASK);
    }

    /// Clear OD bit atomically (RW)
    #[inline(always)]
    pub fn clr_od(&self) {
        self.clr_bits(Self::OD_MASK);
    }

    /// **IE** `[6]` (RW)
    ///
    /// Input enable
    #[inline(always)]
    pub fn ie(&self) -> bool {
        (self.value.read_bits() & Self::IE_MASK) != 0
    }

    /// Set IE bit atomically (RW)
    #[inline(always)]
    pub fn set_ie(&self) {
        self.set_bits(Self::IE_MASK);
    }

    /// Clear IE bit atomically (RW)
    #[inline(always)]
    pub fn clr_ie(&self) {
        self.clr_bits(Self::IE_MASK);
    }

    /// **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn drive(&self) -> u32 {
        (self.value.read_bits() & Self::DRIVE_MASK) >> Self::DRIVE_OFFSET
    }

    /// Write **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn write_drive(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DRIVE_MASK) | ((value << Self::DRIVE_OFFSET) & Self::DRIVE_MASK));
    }

    /// **PUE** `[3]` (RW)
    ///
    /// Pull up enable
    #[inline(always)]
    pub fn pue(&self) -> bool {
        (self.value.read_bits() & Self::PUE_MASK) != 0
    }

    /// Set PUE bit atomically (RW)
    #[inline(always)]
    pub fn set_pue(&self) {
        self.set_bits(Self::PUE_MASK);
    }

    /// Clear PUE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pue(&self) {
        self.clr_bits(Self::PUE_MASK);
    }

    /// **PDE** `[2]` (RW)
    ///
    /// Pull down enable
    #[inline(always)]
    pub fn pde(&self) -> bool {
        (self.value.read_bits() & Self::PDE_MASK) != 0
    }

    /// Set PDE bit atomically (RW)
    #[inline(always)]
    pub fn set_pde(&self) {
        self.set_bits(Self::PDE_MASK);
    }

    /// Clear PDE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pde(&self) {
        self.clr_bits(Self::PDE_MASK);
    }

    /// **SCHMITT** `[1]` (RW)
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub fn schmitt(&self) -> bool {
        (self.value.read_bits() & Self::SCHMITT_MASK) != 0
    }

    /// Set SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn set_schmitt(&self) {
        self.set_bits(Self::SCHMITT_MASK);
    }

    /// Clear SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn clr_schmitt(&self) {
        self.clr_bits(Self::SCHMITT_MASK);
    }

    /// **SLEWFAST** `[0]` (RW)
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub fn slewfast(&self) -> bool {
        (self.value.read_bits() & Self::SLEWFAST_MASK) != 0
    }

    /// Set SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn set_slewfast(&self) {
        self.set_bits(Self::SLEWFAST_MASK);
    }

    /// Clear SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn clr_slewfast(&self) {
        self.clr_bits(Self::SLEWFAST_MASK);
    }

}

/// GPIO34 value type for batch read/modify/write operations
///
/// Use `gpio34.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpio34.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Gpio34Val {
    bits: u32,
}

impl Gpio34Val {
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
        Self { bits: 0x00000116 }
    }

    /// Read **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn iso(&self) -> bool {
        (self.bits & Gpio34::ISO_MASK) != 0
    }

    /// Read **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn od(&self) -> bool {
        (self.bits & Gpio34::OD_MASK) != 0
    }

    /// Read **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        (self.bits & Gpio34::IE_MASK) != 0
    }

    /// Read **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn drive(&self) -> u32 {
        (self.bits & Gpio34::DRIVE_MASK) >> Gpio34::DRIVE_OFFSET
    }

    /// Read **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn pue(&self) -> bool {
        (self.bits & Gpio34::PUE_MASK) != 0
    }

    /// Read **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn pde(&self) -> bool {
        (self.bits & Gpio34::PDE_MASK) != 0
    }

    /// Read **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn schmitt(&self) -> bool {
        (self.bits & Gpio34::SCHMITT_MASK) != 0
    }

    /// Read **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn slewfast(&self) -> bool {
        (self.bits & Gpio34::SLEWFAST_MASK) != 0
    }

    /// Set **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn with_iso(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio34::ISO_MASK
            } else {
                self.bits & !Gpio34::ISO_MASK
            }
        }
    }

    /// Set **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn with_od(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio34::OD_MASK
            } else {
                self.bits & !Gpio34::OD_MASK
            }
        }
    }

    /// Set **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn with_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio34::IE_MASK
            } else {
                self.bits & !Gpio34::IE_MASK
            }
        }
    }

    /// Set **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn with_drive(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Gpio34::DRIVE_MASK) | ((value << Gpio34::DRIVE_OFFSET) & Gpio34::DRIVE_MASK)
        }
    }

    /// Set **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn with_pue(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio34::PUE_MASK
            } else {
                self.bits & !Gpio34::PUE_MASK
            }
        }
    }

    /// Set **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn with_pde(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio34::PDE_MASK
            } else {
                self.bits & !Gpio34::PDE_MASK
            }
        }
    }

    /// Set **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn with_schmitt(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio34::SCHMITT_MASK
            } else {
                self.bits & !Gpio34::SCHMITT_MASK
            }
        }
    }

    /// Set **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn with_slewfast(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio34::SLEWFAST_MASK
            } else {
                self.bits & !Gpio34::SLEWFAST_MASK
            }
        }
    }

}

impl Default for Gpio34Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Gpio34 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Gpio34Val {
        Gpio34Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Gpio34Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Gpio34Val) -> Gpio34Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// GPIO35 Register
///
/// Address offset: 0x0090
/// Reset value: 0x00000116
///
/// ## Fields
///
/// - **ISO** `[8]` (RW)
///
///   Pad isolation control. Remove this once the pad is configured by software.
///
/// - **OD** `[7]` (RW)
///
///   Output disable. Has priority over output enable from peripherals
///
/// - **IE** `[6]` (RW)
///
///   Input enable
///
/// - **DRIVE** `[5:4]` (RW)
///
///   Drive strength.
///
/// - **PUE** `[3]` (RW)
///
///   Pull up enable
///
/// - **PDE** `[2]` (RW)
///
///   Pull down enable
///
/// - **SCHMITT** `[1]` (RW)
///
///   Enable schmitt trigger
///
/// - **SLEWFAST** `[0]` (RW)
///
///   Slew rate control. 1 = Fast, 0 = Slow
#[repr(transparent)]
pub struct Gpio35 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Gpio35 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Gpio35 {
    // ======== Field Constants ========

    /// **ISO**
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ISO_OFFSET: u32 = 8;
    pub const ISO_MASK: u32 = 0x00000100;
    pub const ISO: u32 = 1 << 8;

    /// **OD**
    ///
    /// Output disable. Has priority over output enable from peripherals
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const OD_OFFSET: u32 = 7;
    pub const OD_MASK: u32 = 0x00000080;
    pub const OD: u32 = 1 << 7;

    /// **IE**
    ///
    /// Input enable
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const IE_OFFSET: u32 = 6;
    pub const IE_MASK: u32 = 0x00000040;
    pub const IE: u32 = 1 << 6;

    /// **DRIVE**
    ///
    /// Drive strength.
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const DRIVE_OFFSET: u32 = 4;
    pub const DRIVE_MASK: u32 = 0x00000030;
    pub const DRIVE_2M_A: u32 = 0 << 4;
    pub const DRIVE_2M_A_VAL: u32 = 0;
    pub const DRIVE_4M_A: u32 = 1 << 4;
    pub const DRIVE_4M_A_VAL: u32 = 1;
    pub const DRIVE_8M_A: u32 = 2 << 4;
    pub const DRIVE_8M_A_VAL: u32 = 2;
    pub const DRIVE_12M_A: u32 = 3 << 4;
    pub const DRIVE_12M_A_VAL: u32 = 3;

    /// **PUE**
    ///
    /// Pull up enable
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const PUE_OFFSET: u32 = 3;
    pub const PUE_MASK: u32 = 0x00000008;
    pub const PUE: u32 = 1 << 3;

    /// **PDE**
    ///
    /// Pull down enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const PDE_OFFSET: u32 = 2;
    pub const PDE_MASK: u32 = 0x00000004;
    pub const PDE: u32 = 1 << 2;

    /// **SCHMITT**
    ///
    /// Enable schmitt trigger
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SCHMITT_OFFSET: u32 = 1;
    pub const SCHMITT_MASK: u32 = 0x00000002;
    pub const SCHMITT: u32 = 1 << 1;

    /// **SLEWFAST**
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SLEWFAST_OFFSET: u32 = 0;
    pub const SLEWFAST_MASK: u32 = 0x00000001;
    pub const SLEWFAST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ISO** `[8]` (RW)
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub fn iso(&self) -> bool {
        (self.value.read_bits() & Self::ISO_MASK) != 0
    }

    /// Set ISO bit atomically (RW)
    #[inline(always)]
    pub fn set_iso(&self) {
        self.set_bits(Self::ISO_MASK);
    }

    /// Clear ISO bit atomically (RW)
    #[inline(always)]
    pub fn clr_iso(&self) {
        self.clr_bits(Self::ISO_MASK);
    }

    /// **OD** `[7]` (RW)
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub fn od(&self) -> bool {
        (self.value.read_bits() & Self::OD_MASK) != 0
    }

    /// Set OD bit atomically (RW)
    #[inline(always)]
    pub fn set_od(&self) {
        self.set_bits(Self::OD_MASK);
    }

    /// Clear OD bit atomically (RW)
    #[inline(always)]
    pub fn clr_od(&self) {
        self.clr_bits(Self::OD_MASK);
    }

    /// **IE** `[6]` (RW)
    ///
    /// Input enable
    #[inline(always)]
    pub fn ie(&self) -> bool {
        (self.value.read_bits() & Self::IE_MASK) != 0
    }

    /// Set IE bit atomically (RW)
    #[inline(always)]
    pub fn set_ie(&self) {
        self.set_bits(Self::IE_MASK);
    }

    /// Clear IE bit atomically (RW)
    #[inline(always)]
    pub fn clr_ie(&self) {
        self.clr_bits(Self::IE_MASK);
    }

    /// **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn drive(&self) -> u32 {
        (self.value.read_bits() & Self::DRIVE_MASK) >> Self::DRIVE_OFFSET
    }

    /// Write **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn write_drive(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DRIVE_MASK) | ((value << Self::DRIVE_OFFSET) & Self::DRIVE_MASK));
    }

    /// **PUE** `[3]` (RW)
    ///
    /// Pull up enable
    #[inline(always)]
    pub fn pue(&self) -> bool {
        (self.value.read_bits() & Self::PUE_MASK) != 0
    }

    /// Set PUE bit atomically (RW)
    #[inline(always)]
    pub fn set_pue(&self) {
        self.set_bits(Self::PUE_MASK);
    }

    /// Clear PUE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pue(&self) {
        self.clr_bits(Self::PUE_MASK);
    }

    /// **PDE** `[2]` (RW)
    ///
    /// Pull down enable
    #[inline(always)]
    pub fn pde(&self) -> bool {
        (self.value.read_bits() & Self::PDE_MASK) != 0
    }

    /// Set PDE bit atomically (RW)
    #[inline(always)]
    pub fn set_pde(&self) {
        self.set_bits(Self::PDE_MASK);
    }

    /// Clear PDE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pde(&self) {
        self.clr_bits(Self::PDE_MASK);
    }

    /// **SCHMITT** `[1]` (RW)
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub fn schmitt(&self) -> bool {
        (self.value.read_bits() & Self::SCHMITT_MASK) != 0
    }

    /// Set SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn set_schmitt(&self) {
        self.set_bits(Self::SCHMITT_MASK);
    }

    /// Clear SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn clr_schmitt(&self) {
        self.clr_bits(Self::SCHMITT_MASK);
    }

    /// **SLEWFAST** `[0]` (RW)
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub fn slewfast(&self) -> bool {
        (self.value.read_bits() & Self::SLEWFAST_MASK) != 0
    }

    /// Set SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn set_slewfast(&self) {
        self.set_bits(Self::SLEWFAST_MASK);
    }

    /// Clear SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn clr_slewfast(&self) {
        self.clr_bits(Self::SLEWFAST_MASK);
    }

}

/// GPIO35 value type for batch read/modify/write operations
///
/// Use `gpio35.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpio35.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Gpio35Val {
    bits: u32,
}

impl Gpio35Val {
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
        Self { bits: 0x00000116 }
    }

    /// Read **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn iso(&self) -> bool {
        (self.bits & Gpio35::ISO_MASK) != 0
    }

    /// Read **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn od(&self) -> bool {
        (self.bits & Gpio35::OD_MASK) != 0
    }

    /// Read **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        (self.bits & Gpio35::IE_MASK) != 0
    }

    /// Read **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn drive(&self) -> u32 {
        (self.bits & Gpio35::DRIVE_MASK) >> Gpio35::DRIVE_OFFSET
    }

    /// Read **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn pue(&self) -> bool {
        (self.bits & Gpio35::PUE_MASK) != 0
    }

    /// Read **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn pde(&self) -> bool {
        (self.bits & Gpio35::PDE_MASK) != 0
    }

    /// Read **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn schmitt(&self) -> bool {
        (self.bits & Gpio35::SCHMITT_MASK) != 0
    }

    /// Read **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn slewfast(&self) -> bool {
        (self.bits & Gpio35::SLEWFAST_MASK) != 0
    }

    /// Set **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn with_iso(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio35::ISO_MASK
            } else {
                self.bits & !Gpio35::ISO_MASK
            }
        }
    }

    /// Set **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn with_od(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio35::OD_MASK
            } else {
                self.bits & !Gpio35::OD_MASK
            }
        }
    }

    /// Set **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn with_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio35::IE_MASK
            } else {
                self.bits & !Gpio35::IE_MASK
            }
        }
    }

    /// Set **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn with_drive(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Gpio35::DRIVE_MASK) | ((value << Gpio35::DRIVE_OFFSET) & Gpio35::DRIVE_MASK)
        }
    }

    /// Set **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn with_pue(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio35::PUE_MASK
            } else {
                self.bits & !Gpio35::PUE_MASK
            }
        }
    }

    /// Set **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn with_pde(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio35::PDE_MASK
            } else {
                self.bits & !Gpio35::PDE_MASK
            }
        }
    }

    /// Set **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn with_schmitt(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio35::SCHMITT_MASK
            } else {
                self.bits & !Gpio35::SCHMITT_MASK
            }
        }
    }

    /// Set **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn with_slewfast(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio35::SLEWFAST_MASK
            } else {
                self.bits & !Gpio35::SLEWFAST_MASK
            }
        }
    }

}

impl Default for Gpio35Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Gpio35 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Gpio35Val {
        Gpio35Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Gpio35Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Gpio35Val) -> Gpio35Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// GPIO36 Register
///
/// Address offset: 0x0094
/// Reset value: 0x00000116
///
/// ## Fields
///
/// - **ISO** `[8]` (RW)
///
///   Pad isolation control. Remove this once the pad is configured by software.
///
/// - **OD** `[7]` (RW)
///
///   Output disable. Has priority over output enable from peripherals
///
/// - **IE** `[6]` (RW)
///
///   Input enable
///
/// - **DRIVE** `[5:4]` (RW)
///
///   Drive strength.
///
/// - **PUE** `[3]` (RW)
///
///   Pull up enable
///
/// - **PDE** `[2]` (RW)
///
///   Pull down enable
///
/// - **SCHMITT** `[1]` (RW)
///
///   Enable schmitt trigger
///
/// - **SLEWFAST** `[0]` (RW)
///
///   Slew rate control. 1 = Fast, 0 = Slow
#[repr(transparent)]
pub struct Gpio36 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Gpio36 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Gpio36 {
    // ======== Field Constants ========

    /// **ISO**
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ISO_OFFSET: u32 = 8;
    pub const ISO_MASK: u32 = 0x00000100;
    pub const ISO: u32 = 1 << 8;

    /// **OD**
    ///
    /// Output disable. Has priority over output enable from peripherals
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const OD_OFFSET: u32 = 7;
    pub const OD_MASK: u32 = 0x00000080;
    pub const OD: u32 = 1 << 7;

    /// **IE**
    ///
    /// Input enable
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const IE_OFFSET: u32 = 6;
    pub const IE_MASK: u32 = 0x00000040;
    pub const IE: u32 = 1 << 6;

    /// **DRIVE**
    ///
    /// Drive strength.
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const DRIVE_OFFSET: u32 = 4;
    pub const DRIVE_MASK: u32 = 0x00000030;
    pub const DRIVE_2M_A: u32 = 0 << 4;
    pub const DRIVE_2M_A_VAL: u32 = 0;
    pub const DRIVE_4M_A: u32 = 1 << 4;
    pub const DRIVE_4M_A_VAL: u32 = 1;
    pub const DRIVE_8M_A: u32 = 2 << 4;
    pub const DRIVE_8M_A_VAL: u32 = 2;
    pub const DRIVE_12M_A: u32 = 3 << 4;
    pub const DRIVE_12M_A_VAL: u32 = 3;

    /// **PUE**
    ///
    /// Pull up enable
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const PUE_OFFSET: u32 = 3;
    pub const PUE_MASK: u32 = 0x00000008;
    pub const PUE: u32 = 1 << 3;

    /// **PDE**
    ///
    /// Pull down enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const PDE_OFFSET: u32 = 2;
    pub const PDE_MASK: u32 = 0x00000004;
    pub const PDE: u32 = 1 << 2;

    /// **SCHMITT**
    ///
    /// Enable schmitt trigger
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SCHMITT_OFFSET: u32 = 1;
    pub const SCHMITT_MASK: u32 = 0x00000002;
    pub const SCHMITT: u32 = 1 << 1;

    /// **SLEWFAST**
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SLEWFAST_OFFSET: u32 = 0;
    pub const SLEWFAST_MASK: u32 = 0x00000001;
    pub const SLEWFAST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ISO** `[8]` (RW)
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub fn iso(&self) -> bool {
        (self.value.read_bits() & Self::ISO_MASK) != 0
    }

    /// Set ISO bit atomically (RW)
    #[inline(always)]
    pub fn set_iso(&self) {
        self.set_bits(Self::ISO_MASK);
    }

    /// Clear ISO bit atomically (RW)
    #[inline(always)]
    pub fn clr_iso(&self) {
        self.clr_bits(Self::ISO_MASK);
    }

    /// **OD** `[7]` (RW)
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub fn od(&self) -> bool {
        (self.value.read_bits() & Self::OD_MASK) != 0
    }

    /// Set OD bit atomically (RW)
    #[inline(always)]
    pub fn set_od(&self) {
        self.set_bits(Self::OD_MASK);
    }

    /// Clear OD bit atomically (RW)
    #[inline(always)]
    pub fn clr_od(&self) {
        self.clr_bits(Self::OD_MASK);
    }

    /// **IE** `[6]` (RW)
    ///
    /// Input enable
    #[inline(always)]
    pub fn ie(&self) -> bool {
        (self.value.read_bits() & Self::IE_MASK) != 0
    }

    /// Set IE bit atomically (RW)
    #[inline(always)]
    pub fn set_ie(&self) {
        self.set_bits(Self::IE_MASK);
    }

    /// Clear IE bit atomically (RW)
    #[inline(always)]
    pub fn clr_ie(&self) {
        self.clr_bits(Self::IE_MASK);
    }

    /// **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn drive(&self) -> u32 {
        (self.value.read_bits() & Self::DRIVE_MASK) >> Self::DRIVE_OFFSET
    }

    /// Write **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn write_drive(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DRIVE_MASK) | ((value << Self::DRIVE_OFFSET) & Self::DRIVE_MASK));
    }

    /// **PUE** `[3]` (RW)
    ///
    /// Pull up enable
    #[inline(always)]
    pub fn pue(&self) -> bool {
        (self.value.read_bits() & Self::PUE_MASK) != 0
    }

    /// Set PUE bit atomically (RW)
    #[inline(always)]
    pub fn set_pue(&self) {
        self.set_bits(Self::PUE_MASK);
    }

    /// Clear PUE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pue(&self) {
        self.clr_bits(Self::PUE_MASK);
    }

    /// **PDE** `[2]` (RW)
    ///
    /// Pull down enable
    #[inline(always)]
    pub fn pde(&self) -> bool {
        (self.value.read_bits() & Self::PDE_MASK) != 0
    }

    /// Set PDE bit atomically (RW)
    #[inline(always)]
    pub fn set_pde(&self) {
        self.set_bits(Self::PDE_MASK);
    }

    /// Clear PDE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pde(&self) {
        self.clr_bits(Self::PDE_MASK);
    }

    /// **SCHMITT** `[1]` (RW)
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub fn schmitt(&self) -> bool {
        (self.value.read_bits() & Self::SCHMITT_MASK) != 0
    }

    /// Set SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn set_schmitt(&self) {
        self.set_bits(Self::SCHMITT_MASK);
    }

    /// Clear SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn clr_schmitt(&self) {
        self.clr_bits(Self::SCHMITT_MASK);
    }

    /// **SLEWFAST** `[0]` (RW)
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub fn slewfast(&self) -> bool {
        (self.value.read_bits() & Self::SLEWFAST_MASK) != 0
    }

    /// Set SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn set_slewfast(&self) {
        self.set_bits(Self::SLEWFAST_MASK);
    }

    /// Clear SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn clr_slewfast(&self) {
        self.clr_bits(Self::SLEWFAST_MASK);
    }

}

/// GPIO36 value type for batch read/modify/write operations
///
/// Use `gpio36.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpio36.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Gpio36Val {
    bits: u32,
}

impl Gpio36Val {
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
        Self { bits: 0x00000116 }
    }

    /// Read **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn iso(&self) -> bool {
        (self.bits & Gpio36::ISO_MASK) != 0
    }

    /// Read **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn od(&self) -> bool {
        (self.bits & Gpio36::OD_MASK) != 0
    }

    /// Read **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        (self.bits & Gpio36::IE_MASK) != 0
    }

    /// Read **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn drive(&self) -> u32 {
        (self.bits & Gpio36::DRIVE_MASK) >> Gpio36::DRIVE_OFFSET
    }

    /// Read **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn pue(&self) -> bool {
        (self.bits & Gpio36::PUE_MASK) != 0
    }

    /// Read **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn pde(&self) -> bool {
        (self.bits & Gpio36::PDE_MASK) != 0
    }

    /// Read **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn schmitt(&self) -> bool {
        (self.bits & Gpio36::SCHMITT_MASK) != 0
    }

    /// Read **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn slewfast(&self) -> bool {
        (self.bits & Gpio36::SLEWFAST_MASK) != 0
    }

    /// Set **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn with_iso(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio36::ISO_MASK
            } else {
                self.bits & !Gpio36::ISO_MASK
            }
        }
    }

    /// Set **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn with_od(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio36::OD_MASK
            } else {
                self.bits & !Gpio36::OD_MASK
            }
        }
    }

    /// Set **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn with_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio36::IE_MASK
            } else {
                self.bits & !Gpio36::IE_MASK
            }
        }
    }

    /// Set **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn with_drive(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Gpio36::DRIVE_MASK) | ((value << Gpio36::DRIVE_OFFSET) & Gpio36::DRIVE_MASK)
        }
    }

    /// Set **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn with_pue(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio36::PUE_MASK
            } else {
                self.bits & !Gpio36::PUE_MASK
            }
        }
    }

    /// Set **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn with_pde(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio36::PDE_MASK
            } else {
                self.bits & !Gpio36::PDE_MASK
            }
        }
    }

    /// Set **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn with_schmitt(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio36::SCHMITT_MASK
            } else {
                self.bits & !Gpio36::SCHMITT_MASK
            }
        }
    }

    /// Set **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn with_slewfast(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio36::SLEWFAST_MASK
            } else {
                self.bits & !Gpio36::SLEWFAST_MASK
            }
        }
    }

}

impl Default for Gpio36Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Gpio36 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Gpio36Val {
        Gpio36Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Gpio36Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Gpio36Val) -> Gpio36Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// GPIO37 Register
///
/// Address offset: 0x0098
/// Reset value: 0x00000116
///
/// ## Fields
///
/// - **ISO** `[8]` (RW)
///
///   Pad isolation control. Remove this once the pad is configured by software.
///
/// - **OD** `[7]` (RW)
///
///   Output disable. Has priority over output enable from peripherals
///
/// - **IE** `[6]` (RW)
///
///   Input enable
///
/// - **DRIVE** `[5:4]` (RW)
///
///   Drive strength.
///
/// - **PUE** `[3]` (RW)
///
///   Pull up enable
///
/// - **PDE** `[2]` (RW)
///
///   Pull down enable
///
/// - **SCHMITT** `[1]` (RW)
///
///   Enable schmitt trigger
///
/// - **SLEWFAST** `[0]` (RW)
///
///   Slew rate control. 1 = Fast, 0 = Slow
#[repr(transparent)]
pub struct Gpio37 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Gpio37 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Gpio37 {
    // ======== Field Constants ========

    /// **ISO**
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ISO_OFFSET: u32 = 8;
    pub const ISO_MASK: u32 = 0x00000100;
    pub const ISO: u32 = 1 << 8;

    /// **OD**
    ///
    /// Output disable. Has priority over output enable from peripherals
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const OD_OFFSET: u32 = 7;
    pub const OD_MASK: u32 = 0x00000080;
    pub const OD: u32 = 1 << 7;

    /// **IE**
    ///
    /// Input enable
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const IE_OFFSET: u32 = 6;
    pub const IE_MASK: u32 = 0x00000040;
    pub const IE: u32 = 1 << 6;

    /// **DRIVE**
    ///
    /// Drive strength.
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const DRIVE_OFFSET: u32 = 4;
    pub const DRIVE_MASK: u32 = 0x00000030;
    pub const DRIVE_2M_A: u32 = 0 << 4;
    pub const DRIVE_2M_A_VAL: u32 = 0;
    pub const DRIVE_4M_A: u32 = 1 << 4;
    pub const DRIVE_4M_A_VAL: u32 = 1;
    pub const DRIVE_8M_A: u32 = 2 << 4;
    pub const DRIVE_8M_A_VAL: u32 = 2;
    pub const DRIVE_12M_A: u32 = 3 << 4;
    pub const DRIVE_12M_A_VAL: u32 = 3;

    /// **PUE**
    ///
    /// Pull up enable
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const PUE_OFFSET: u32 = 3;
    pub const PUE_MASK: u32 = 0x00000008;
    pub const PUE: u32 = 1 << 3;

    /// **PDE**
    ///
    /// Pull down enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const PDE_OFFSET: u32 = 2;
    pub const PDE_MASK: u32 = 0x00000004;
    pub const PDE: u32 = 1 << 2;

    /// **SCHMITT**
    ///
    /// Enable schmitt trigger
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SCHMITT_OFFSET: u32 = 1;
    pub const SCHMITT_MASK: u32 = 0x00000002;
    pub const SCHMITT: u32 = 1 << 1;

    /// **SLEWFAST**
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SLEWFAST_OFFSET: u32 = 0;
    pub const SLEWFAST_MASK: u32 = 0x00000001;
    pub const SLEWFAST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ISO** `[8]` (RW)
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub fn iso(&self) -> bool {
        (self.value.read_bits() & Self::ISO_MASK) != 0
    }

    /// Set ISO bit atomically (RW)
    #[inline(always)]
    pub fn set_iso(&self) {
        self.set_bits(Self::ISO_MASK);
    }

    /// Clear ISO bit atomically (RW)
    #[inline(always)]
    pub fn clr_iso(&self) {
        self.clr_bits(Self::ISO_MASK);
    }

    /// **OD** `[7]` (RW)
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub fn od(&self) -> bool {
        (self.value.read_bits() & Self::OD_MASK) != 0
    }

    /// Set OD bit atomically (RW)
    #[inline(always)]
    pub fn set_od(&self) {
        self.set_bits(Self::OD_MASK);
    }

    /// Clear OD bit atomically (RW)
    #[inline(always)]
    pub fn clr_od(&self) {
        self.clr_bits(Self::OD_MASK);
    }

    /// **IE** `[6]` (RW)
    ///
    /// Input enable
    #[inline(always)]
    pub fn ie(&self) -> bool {
        (self.value.read_bits() & Self::IE_MASK) != 0
    }

    /// Set IE bit atomically (RW)
    #[inline(always)]
    pub fn set_ie(&self) {
        self.set_bits(Self::IE_MASK);
    }

    /// Clear IE bit atomically (RW)
    #[inline(always)]
    pub fn clr_ie(&self) {
        self.clr_bits(Self::IE_MASK);
    }

    /// **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn drive(&self) -> u32 {
        (self.value.read_bits() & Self::DRIVE_MASK) >> Self::DRIVE_OFFSET
    }

    /// Write **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn write_drive(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DRIVE_MASK) | ((value << Self::DRIVE_OFFSET) & Self::DRIVE_MASK));
    }

    /// **PUE** `[3]` (RW)
    ///
    /// Pull up enable
    #[inline(always)]
    pub fn pue(&self) -> bool {
        (self.value.read_bits() & Self::PUE_MASK) != 0
    }

    /// Set PUE bit atomically (RW)
    #[inline(always)]
    pub fn set_pue(&self) {
        self.set_bits(Self::PUE_MASK);
    }

    /// Clear PUE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pue(&self) {
        self.clr_bits(Self::PUE_MASK);
    }

    /// **PDE** `[2]` (RW)
    ///
    /// Pull down enable
    #[inline(always)]
    pub fn pde(&self) -> bool {
        (self.value.read_bits() & Self::PDE_MASK) != 0
    }

    /// Set PDE bit atomically (RW)
    #[inline(always)]
    pub fn set_pde(&self) {
        self.set_bits(Self::PDE_MASK);
    }

    /// Clear PDE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pde(&self) {
        self.clr_bits(Self::PDE_MASK);
    }

    /// **SCHMITT** `[1]` (RW)
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub fn schmitt(&self) -> bool {
        (self.value.read_bits() & Self::SCHMITT_MASK) != 0
    }

    /// Set SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn set_schmitt(&self) {
        self.set_bits(Self::SCHMITT_MASK);
    }

    /// Clear SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn clr_schmitt(&self) {
        self.clr_bits(Self::SCHMITT_MASK);
    }

    /// **SLEWFAST** `[0]` (RW)
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub fn slewfast(&self) -> bool {
        (self.value.read_bits() & Self::SLEWFAST_MASK) != 0
    }

    /// Set SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn set_slewfast(&self) {
        self.set_bits(Self::SLEWFAST_MASK);
    }

    /// Clear SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn clr_slewfast(&self) {
        self.clr_bits(Self::SLEWFAST_MASK);
    }

}

/// GPIO37 value type for batch read/modify/write operations
///
/// Use `gpio37.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpio37.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Gpio37Val {
    bits: u32,
}

impl Gpio37Val {
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
        Self { bits: 0x00000116 }
    }

    /// Read **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn iso(&self) -> bool {
        (self.bits & Gpio37::ISO_MASK) != 0
    }

    /// Read **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn od(&self) -> bool {
        (self.bits & Gpio37::OD_MASK) != 0
    }

    /// Read **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        (self.bits & Gpio37::IE_MASK) != 0
    }

    /// Read **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn drive(&self) -> u32 {
        (self.bits & Gpio37::DRIVE_MASK) >> Gpio37::DRIVE_OFFSET
    }

    /// Read **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn pue(&self) -> bool {
        (self.bits & Gpio37::PUE_MASK) != 0
    }

    /// Read **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn pde(&self) -> bool {
        (self.bits & Gpio37::PDE_MASK) != 0
    }

    /// Read **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn schmitt(&self) -> bool {
        (self.bits & Gpio37::SCHMITT_MASK) != 0
    }

    /// Read **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn slewfast(&self) -> bool {
        (self.bits & Gpio37::SLEWFAST_MASK) != 0
    }

    /// Set **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn with_iso(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio37::ISO_MASK
            } else {
                self.bits & !Gpio37::ISO_MASK
            }
        }
    }

    /// Set **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn with_od(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio37::OD_MASK
            } else {
                self.bits & !Gpio37::OD_MASK
            }
        }
    }

    /// Set **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn with_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio37::IE_MASK
            } else {
                self.bits & !Gpio37::IE_MASK
            }
        }
    }

    /// Set **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn with_drive(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Gpio37::DRIVE_MASK) | ((value << Gpio37::DRIVE_OFFSET) & Gpio37::DRIVE_MASK)
        }
    }

    /// Set **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn with_pue(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio37::PUE_MASK
            } else {
                self.bits & !Gpio37::PUE_MASK
            }
        }
    }

    /// Set **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn with_pde(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio37::PDE_MASK
            } else {
                self.bits & !Gpio37::PDE_MASK
            }
        }
    }

    /// Set **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn with_schmitt(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio37::SCHMITT_MASK
            } else {
                self.bits & !Gpio37::SCHMITT_MASK
            }
        }
    }

    /// Set **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn with_slewfast(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio37::SLEWFAST_MASK
            } else {
                self.bits & !Gpio37::SLEWFAST_MASK
            }
        }
    }

}

impl Default for Gpio37Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Gpio37 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Gpio37Val {
        Gpio37Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Gpio37Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Gpio37Val) -> Gpio37Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// GPIO38 Register
///
/// Address offset: 0x009c
/// Reset value: 0x00000116
///
/// ## Fields
///
/// - **ISO** `[8]` (RW)
///
///   Pad isolation control. Remove this once the pad is configured by software.
///
/// - **OD** `[7]` (RW)
///
///   Output disable. Has priority over output enable from peripherals
///
/// - **IE** `[6]` (RW)
///
///   Input enable
///
/// - **DRIVE** `[5:4]` (RW)
///
///   Drive strength.
///
/// - **PUE** `[3]` (RW)
///
///   Pull up enable
///
/// - **PDE** `[2]` (RW)
///
///   Pull down enable
///
/// - **SCHMITT** `[1]` (RW)
///
///   Enable schmitt trigger
///
/// - **SLEWFAST** `[0]` (RW)
///
///   Slew rate control. 1 = Fast, 0 = Slow
#[repr(transparent)]
pub struct Gpio38 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Gpio38 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Gpio38 {
    // ======== Field Constants ========

    /// **ISO**
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ISO_OFFSET: u32 = 8;
    pub const ISO_MASK: u32 = 0x00000100;
    pub const ISO: u32 = 1 << 8;

    /// **OD**
    ///
    /// Output disable. Has priority over output enable from peripherals
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const OD_OFFSET: u32 = 7;
    pub const OD_MASK: u32 = 0x00000080;
    pub const OD: u32 = 1 << 7;

    /// **IE**
    ///
    /// Input enable
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const IE_OFFSET: u32 = 6;
    pub const IE_MASK: u32 = 0x00000040;
    pub const IE: u32 = 1 << 6;

    /// **DRIVE**
    ///
    /// Drive strength.
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const DRIVE_OFFSET: u32 = 4;
    pub const DRIVE_MASK: u32 = 0x00000030;
    pub const DRIVE_2M_A: u32 = 0 << 4;
    pub const DRIVE_2M_A_VAL: u32 = 0;
    pub const DRIVE_4M_A: u32 = 1 << 4;
    pub const DRIVE_4M_A_VAL: u32 = 1;
    pub const DRIVE_8M_A: u32 = 2 << 4;
    pub const DRIVE_8M_A_VAL: u32 = 2;
    pub const DRIVE_12M_A: u32 = 3 << 4;
    pub const DRIVE_12M_A_VAL: u32 = 3;

    /// **PUE**
    ///
    /// Pull up enable
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const PUE_OFFSET: u32 = 3;
    pub const PUE_MASK: u32 = 0x00000008;
    pub const PUE: u32 = 1 << 3;

    /// **PDE**
    ///
    /// Pull down enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const PDE_OFFSET: u32 = 2;
    pub const PDE_MASK: u32 = 0x00000004;
    pub const PDE: u32 = 1 << 2;

    /// **SCHMITT**
    ///
    /// Enable schmitt trigger
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SCHMITT_OFFSET: u32 = 1;
    pub const SCHMITT_MASK: u32 = 0x00000002;
    pub const SCHMITT: u32 = 1 << 1;

    /// **SLEWFAST**
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SLEWFAST_OFFSET: u32 = 0;
    pub const SLEWFAST_MASK: u32 = 0x00000001;
    pub const SLEWFAST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ISO** `[8]` (RW)
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub fn iso(&self) -> bool {
        (self.value.read_bits() & Self::ISO_MASK) != 0
    }

    /// Set ISO bit atomically (RW)
    #[inline(always)]
    pub fn set_iso(&self) {
        self.set_bits(Self::ISO_MASK);
    }

    /// Clear ISO bit atomically (RW)
    #[inline(always)]
    pub fn clr_iso(&self) {
        self.clr_bits(Self::ISO_MASK);
    }

    /// **OD** `[7]` (RW)
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub fn od(&self) -> bool {
        (self.value.read_bits() & Self::OD_MASK) != 0
    }

    /// Set OD bit atomically (RW)
    #[inline(always)]
    pub fn set_od(&self) {
        self.set_bits(Self::OD_MASK);
    }

    /// Clear OD bit atomically (RW)
    #[inline(always)]
    pub fn clr_od(&self) {
        self.clr_bits(Self::OD_MASK);
    }

    /// **IE** `[6]` (RW)
    ///
    /// Input enable
    #[inline(always)]
    pub fn ie(&self) -> bool {
        (self.value.read_bits() & Self::IE_MASK) != 0
    }

    /// Set IE bit atomically (RW)
    #[inline(always)]
    pub fn set_ie(&self) {
        self.set_bits(Self::IE_MASK);
    }

    /// Clear IE bit atomically (RW)
    #[inline(always)]
    pub fn clr_ie(&self) {
        self.clr_bits(Self::IE_MASK);
    }

    /// **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn drive(&self) -> u32 {
        (self.value.read_bits() & Self::DRIVE_MASK) >> Self::DRIVE_OFFSET
    }

    /// Write **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn write_drive(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DRIVE_MASK) | ((value << Self::DRIVE_OFFSET) & Self::DRIVE_MASK));
    }

    /// **PUE** `[3]` (RW)
    ///
    /// Pull up enable
    #[inline(always)]
    pub fn pue(&self) -> bool {
        (self.value.read_bits() & Self::PUE_MASK) != 0
    }

    /// Set PUE bit atomically (RW)
    #[inline(always)]
    pub fn set_pue(&self) {
        self.set_bits(Self::PUE_MASK);
    }

    /// Clear PUE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pue(&self) {
        self.clr_bits(Self::PUE_MASK);
    }

    /// **PDE** `[2]` (RW)
    ///
    /// Pull down enable
    #[inline(always)]
    pub fn pde(&self) -> bool {
        (self.value.read_bits() & Self::PDE_MASK) != 0
    }

    /// Set PDE bit atomically (RW)
    #[inline(always)]
    pub fn set_pde(&self) {
        self.set_bits(Self::PDE_MASK);
    }

    /// Clear PDE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pde(&self) {
        self.clr_bits(Self::PDE_MASK);
    }

    /// **SCHMITT** `[1]` (RW)
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub fn schmitt(&self) -> bool {
        (self.value.read_bits() & Self::SCHMITT_MASK) != 0
    }

    /// Set SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn set_schmitt(&self) {
        self.set_bits(Self::SCHMITT_MASK);
    }

    /// Clear SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn clr_schmitt(&self) {
        self.clr_bits(Self::SCHMITT_MASK);
    }

    /// **SLEWFAST** `[0]` (RW)
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub fn slewfast(&self) -> bool {
        (self.value.read_bits() & Self::SLEWFAST_MASK) != 0
    }

    /// Set SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn set_slewfast(&self) {
        self.set_bits(Self::SLEWFAST_MASK);
    }

    /// Clear SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn clr_slewfast(&self) {
        self.clr_bits(Self::SLEWFAST_MASK);
    }

}

/// GPIO38 value type for batch read/modify/write operations
///
/// Use `gpio38.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpio38.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Gpio38Val {
    bits: u32,
}

impl Gpio38Val {
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
        Self { bits: 0x00000116 }
    }

    /// Read **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn iso(&self) -> bool {
        (self.bits & Gpio38::ISO_MASK) != 0
    }

    /// Read **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn od(&self) -> bool {
        (self.bits & Gpio38::OD_MASK) != 0
    }

    /// Read **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        (self.bits & Gpio38::IE_MASK) != 0
    }

    /// Read **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn drive(&self) -> u32 {
        (self.bits & Gpio38::DRIVE_MASK) >> Gpio38::DRIVE_OFFSET
    }

    /// Read **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn pue(&self) -> bool {
        (self.bits & Gpio38::PUE_MASK) != 0
    }

    /// Read **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn pde(&self) -> bool {
        (self.bits & Gpio38::PDE_MASK) != 0
    }

    /// Read **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn schmitt(&self) -> bool {
        (self.bits & Gpio38::SCHMITT_MASK) != 0
    }

    /// Read **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn slewfast(&self) -> bool {
        (self.bits & Gpio38::SLEWFAST_MASK) != 0
    }

    /// Set **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn with_iso(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio38::ISO_MASK
            } else {
                self.bits & !Gpio38::ISO_MASK
            }
        }
    }

    /// Set **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn with_od(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio38::OD_MASK
            } else {
                self.bits & !Gpio38::OD_MASK
            }
        }
    }

    /// Set **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn with_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio38::IE_MASK
            } else {
                self.bits & !Gpio38::IE_MASK
            }
        }
    }

    /// Set **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn with_drive(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Gpio38::DRIVE_MASK) | ((value << Gpio38::DRIVE_OFFSET) & Gpio38::DRIVE_MASK)
        }
    }

    /// Set **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn with_pue(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio38::PUE_MASK
            } else {
                self.bits & !Gpio38::PUE_MASK
            }
        }
    }

    /// Set **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn with_pde(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio38::PDE_MASK
            } else {
                self.bits & !Gpio38::PDE_MASK
            }
        }
    }

    /// Set **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn with_schmitt(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio38::SCHMITT_MASK
            } else {
                self.bits & !Gpio38::SCHMITT_MASK
            }
        }
    }

    /// Set **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn with_slewfast(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio38::SLEWFAST_MASK
            } else {
                self.bits & !Gpio38::SLEWFAST_MASK
            }
        }
    }

}

impl Default for Gpio38Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Gpio38 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Gpio38Val {
        Gpio38Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Gpio38Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Gpio38Val) -> Gpio38Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// GPIO39 Register
///
/// Address offset: 0x00a0
/// Reset value: 0x00000116
///
/// ## Fields
///
/// - **ISO** `[8]` (RW)
///
///   Pad isolation control. Remove this once the pad is configured by software.
///
/// - **OD** `[7]` (RW)
///
///   Output disable. Has priority over output enable from peripherals
///
/// - **IE** `[6]` (RW)
///
///   Input enable
///
/// - **DRIVE** `[5:4]` (RW)
///
///   Drive strength.
///
/// - **PUE** `[3]` (RW)
///
///   Pull up enable
///
/// - **PDE** `[2]` (RW)
///
///   Pull down enable
///
/// - **SCHMITT** `[1]` (RW)
///
///   Enable schmitt trigger
///
/// - **SLEWFAST** `[0]` (RW)
///
///   Slew rate control. 1 = Fast, 0 = Slow
#[repr(transparent)]
pub struct Gpio39 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Gpio39 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Gpio39 {
    // ======== Field Constants ========

    /// **ISO**
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ISO_OFFSET: u32 = 8;
    pub const ISO_MASK: u32 = 0x00000100;
    pub const ISO: u32 = 1 << 8;

    /// **OD**
    ///
    /// Output disable. Has priority over output enable from peripherals
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const OD_OFFSET: u32 = 7;
    pub const OD_MASK: u32 = 0x00000080;
    pub const OD: u32 = 1 << 7;

    /// **IE**
    ///
    /// Input enable
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const IE_OFFSET: u32 = 6;
    pub const IE_MASK: u32 = 0x00000040;
    pub const IE: u32 = 1 << 6;

    /// **DRIVE**
    ///
    /// Drive strength.
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const DRIVE_OFFSET: u32 = 4;
    pub const DRIVE_MASK: u32 = 0x00000030;
    pub const DRIVE_2M_A: u32 = 0 << 4;
    pub const DRIVE_2M_A_VAL: u32 = 0;
    pub const DRIVE_4M_A: u32 = 1 << 4;
    pub const DRIVE_4M_A_VAL: u32 = 1;
    pub const DRIVE_8M_A: u32 = 2 << 4;
    pub const DRIVE_8M_A_VAL: u32 = 2;
    pub const DRIVE_12M_A: u32 = 3 << 4;
    pub const DRIVE_12M_A_VAL: u32 = 3;

    /// **PUE**
    ///
    /// Pull up enable
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const PUE_OFFSET: u32 = 3;
    pub const PUE_MASK: u32 = 0x00000008;
    pub const PUE: u32 = 1 << 3;

    /// **PDE**
    ///
    /// Pull down enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const PDE_OFFSET: u32 = 2;
    pub const PDE_MASK: u32 = 0x00000004;
    pub const PDE: u32 = 1 << 2;

    /// **SCHMITT**
    ///
    /// Enable schmitt trigger
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SCHMITT_OFFSET: u32 = 1;
    pub const SCHMITT_MASK: u32 = 0x00000002;
    pub const SCHMITT: u32 = 1 << 1;

    /// **SLEWFAST**
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SLEWFAST_OFFSET: u32 = 0;
    pub const SLEWFAST_MASK: u32 = 0x00000001;
    pub const SLEWFAST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ISO** `[8]` (RW)
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub fn iso(&self) -> bool {
        (self.value.read_bits() & Self::ISO_MASK) != 0
    }

    /// Set ISO bit atomically (RW)
    #[inline(always)]
    pub fn set_iso(&self) {
        self.set_bits(Self::ISO_MASK);
    }

    /// Clear ISO bit atomically (RW)
    #[inline(always)]
    pub fn clr_iso(&self) {
        self.clr_bits(Self::ISO_MASK);
    }

    /// **OD** `[7]` (RW)
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub fn od(&self) -> bool {
        (self.value.read_bits() & Self::OD_MASK) != 0
    }

    /// Set OD bit atomically (RW)
    #[inline(always)]
    pub fn set_od(&self) {
        self.set_bits(Self::OD_MASK);
    }

    /// Clear OD bit atomically (RW)
    #[inline(always)]
    pub fn clr_od(&self) {
        self.clr_bits(Self::OD_MASK);
    }

    /// **IE** `[6]` (RW)
    ///
    /// Input enable
    #[inline(always)]
    pub fn ie(&self) -> bool {
        (self.value.read_bits() & Self::IE_MASK) != 0
    }

    /// Set IE bit atomically (RW)
    #[inline(always)]
    pub fn set_ie(&self) {
        self.set_bits(Self::IE_MASK);
    }

    /// Clear IE bit atomically (RW)
    #[inline(always)]
    pub fn clr_ie(&self) {
        self.clr_bits(Self::IE_MASK);
    }

    /// **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn drive(&self) -> u32 {
        (self.value.read_bits() & Self::DRIVE_MASK) >> Self::DRIVE_OFFSET
    }

    /// Write **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn write_drive(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DRIVE_MASK) | ((value << Self::DRIVE_OFFSET) & Self::DRIVE_MASK));
    }

    /// **PUE** `[3]` (RW)
    ///
    /// Pull up enable
    #[inline(always)]
    pub fn pue(&self) -> bool {
        (self.value.read_bits() & Self::PUE_MASK) != 0
    }

    /// Set PUE bit atomically (RW)
    #[inline(always)]
    pub fn set_pue(&self) {
        self.set_bits(Self::PUE_MASK);
    }

    /// Clear PUE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pue(&self) {
        self.clr_bits(Self::PUE_MASK);
    }

    /// **PDE** `[2]` (RW)
    ///
    /// Pull down enable
    #[inline(always)]
    pub fn pde(&self) -> bool {
        (self.value.read_bits() & Self::PDE_MASK) != 0
    }

    /// Set PDE bit atomically (RW)
    #[inline(always)]
    pub fn set_pde(&self) {
        self.set_bits(Self::PDE_MASK);
    }

    /// Clear PDE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pde(&self) {
        self.clr_bits(Self::PDE_MASK);
    }

    /// **SCHMITT** `[1]` (RW)
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub fn schmitt(&self) -> bool {
        (self.value.read_bits() & Self::SCHMITT_MASK) != 0
    }

    /// Set SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn set_schmitt(&self) {
        self.set_bits(Self::SCHMITT_MASK);
    }

    /// Clear SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn clr_schmitt(&self) {
        self.clr_bits(Self::SCHMITT_MASK);
    }

    /// **SLEWFAST** `[0]` (RW)
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub fn slewfast(&self) -> bool {
        (self.value.read_bits() & Self::SLEWFAST_MASK) != 0
    }

    /// Set SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn set_slewfast(&self) {
        self.set_bits(Self::SLEWFAST_MASK);
    }

    /// Clear SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn clr_slewfast(&self) {
        self.clr_bits(Self::SLEWFAST_MASK);
    }

}

/// GPIO39 value type for batch read/modify/write operations
///
/// Use `gpio39.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpio39.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Gpio39Val {
    bits: u32,
}

impl Gpio39Val {
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
        Self { bits: 0x00000116 }
    }

    /// Read **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn iso(&self) -> bool {
        (self.bits & Gpio39::ISO_MASK) != 0
    }

    /// Read **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn od(&self) -> bool {
        (self.bits & Gpio39::OD_MASK) != 0
    }

    /// Read **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        (self.bits & Gpio39::IE_MASK) != 0
    }

    /// Read **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn drive(&self) -> u32 {
        (self.bits & Gpio39::DRIVE_MASK) >> Gpio39::DRIVE_OFFSET
    }

    /// Read **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn pue(&self) -> bool {
        (self.bits & Gpio39::PUE_MASK) != 0
    }

    /// Read **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn pde(&self) -> bool {
        (self.bits & Gpio39::PDE_MASK) != 0
    }

    /// Read **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn schmitt(&self) -> bool {
        (self.bits & Gpio39::SCHMITT_MASK) != 0
    }

    /// Read **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn slewfast(&self) -> bool {
        (self.bits & Gpio39::SLEWFAST_MASK) != 0
    }

    /// Set **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn with_iso(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio39::ISO_MASK
            } else {
                self.bits & !Gpio39::ISO_MASK
            }
        }
    }

    /// Set **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn with_od(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio39::OD_MASK
            } else {
                self.bits & !Gpio39::OD_MASK
            }
        }
    }

    /// Set **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn with_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio39::IE_MASK
            } else {
                self.bits & !Gpio39::IE_MASK
            }
        }
    }

    /// Set **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn with_drive(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Gpio39::DRIVE_MASK) | ((value << Gpio39::DRIVE_OFFSET) & Gpio39::DRIVE_MASK)
        }
    }

    /// Set **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn with_pue(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio39::PUE_MASK
            } else {
                self.bits & !Gpio39::PUE_MASK
            }
        }
    }

    /// Set **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn with_pde(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio39::PDE_MASK
            } else {
                self.bits & !Gpio39::PDE_MASK
            }
        }
    }

    /// Set **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn with_schmitt(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio39::SCHMITT_MASK
            } else {
                self.bits & !Gpio39::SCHMITT_MASK
            }
        }
    }

    /// Set **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn with_slewfast(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio39::SLEWFAST_MASK
            } else {
                self.bits & !Gpio39::SLEWFAST_MASK
            }
        }
    }

}

impl Default for Gpio39Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Gpio39 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Gpio39Val {
        Gpio39Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Gpio39Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Gpio39Val) -> Gpio39Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// GPIO40 Register
///
/// Address offset: 0x00a4
/// Reset value: 0x00000116
///
/// ## Fields
///
/// - **ISO** `[8]` (RW)
///
///   Pad isolation control. Remove this once the pad is configured by software.
///
/// - **OD** `[7]` (RW)
///
///   Output disable. Has priority over output enable from peripherals
///
/// - **IE** `[6]` (RW)
///
///   Input enable
///
/// - **DRIVE** `[5:4]` (RW)
///
///   Drive strength.
///
/// - **PUE** `[3]` (RW)
///
///   Pull up enable
///
/// - **PDE** `[2]` (RW)
///
///   Pull down enable
///
/// - **SCHMITT** `[1]` (RW)
///
///   Enable schmitt trigger
///
/// - **SLEWFAST** `[0]` (RW)
///
///   Slew rate control. 1 = Fast, 0 = Slow
#[repr(transparent)]
pub struct Gpio40 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Gpio40 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Gpio40 {
    // ======== Field Constants ========

    /// **ISO**
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ISO_OFFSET: u32 = 8;
    pub const ISO_MASK: u32 = 0x00000100;
    pub const ISO: u32 = 1 << 8;

    /// **OD**
    ///
    /// Output disable. Has priority over output enable from peripherals
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const OD_OFFSET: u32 = 7;
    pub const OD_MASK: u32 = 0x00000080;
    pub const OD: u32 = 1 << 7;

    /// **IE**
    ///
    /// Input enable
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const IE_OFFSET: u32 = 6;
    pub const IE_MASK: u32 = 0x00000040;
    pub const IE: u32 = 1 << 6;

    /// **DRIVE**
    ///
    /// Drive strength.
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const DRIVE_OFFSET: u32 = 4;
    pub const DRIVE_MASK: u32 = 0x00000030;
    pub const DRIVE_2M_A: u32 = 0 << 4;
    pub const DRIVE_2M_A_VAL: u32 = 0;
    pub const DRIVE_4M_A: u32 = 1 << 4;
    pub const DRIVE_4M_A_VAL: u32 = 1;
    pub const DRIVE_8M_A: u32 = 2 << 4;
    pub const DRIVE_8M_A_VAL: u32 = 2;
    pub const DRIVE_12M_A: u32 = 3 << 4;
    pub const DRIVE_12M_A_VAL: u32 = 3;

    /// **PUE**
    ///
    /// Pull up enable
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const PUE_OFFSET: u32 = 3;
    pub const PUE_MASK: u32 = 0x00000008;
    pub const PUE: u32 = 1 << 3;

    /// **PDE**
    ///
    /// Pull down enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const PDE_OFFSET: u32 = 2;
    pub const PDE_MASK: u32 = 0x00000004;
    pub const PDE: u32 = 1 << 2;

    /// **SCHMITT**
    ///
    /// Enable schmitt trigger
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SCHMITT_OFFSET: u32 = 1;
    pub const SCHMITT_MASK: u32 = 0x00000002;
    pub const SCHMITT: u32 = 1 << 1;

    /// **SLEWFAST**
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SLEWFAST_OFFSET: u32 = 0;
    pub const SLEWFAST_MASK: u32 = 0x00000001;
    pub const SLEWFAST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ISO** `[8]` (RW)
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub fn iso(&self) -> bool {
        (self.value.read_bits() & Self::ISO_MASK) != 0
    }

    /// Set ISO bit atomically (RW)
    #[inline(always)]
    pub fn set_iso(&self) {
        self.set_bits(Self::ISO_MASK);
    }

    /// Clear ISO bit atomically (RW)
    #[inline(always)]
    pub fn clr_iso(&self) {
        self.clr_bits(Self::ISO_MASK);
    }

    /// **OD** `[7]` (RW)
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub fn od(&self) -> bool {
        (self.value.read_bits() & Self::OD_MASK) != 0
    }

    /// Set OD bit atomically (RW)
    #[inline(always)]
    pub fn set_od(&self) {
        self.set_bits(Self::OD_MASK);
    }

    /// Clear OD bit atomically (RW)
    #[inline(always)]
    pub fn clr_od(&self) {
        self.clr_bits(Self::OD_MASK);
    }

    /// **IE** `[6]` (RW)
    ///
    /// Input enable
    #[inline(always)]
    pub fn ie(&self) -> bool {
        (self.value.read_bits() & Self::IE_MASK) != 0
    }

    /// Set IE bit atomically (RW)
    #[inline(always)]
    pub fn set_ie(&self) {
        self.set_bits(Self::IE_MASK);
    }

    /// Clear IE bit atomically (RW)
    #[inline(always)]
    pub fn clr_ie(&self) {
        self.clr_bits(Self::IE_MASK);
    }

    /// **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn drive(&self) -> u32 {
        (self.value.read_bits() & Self::DRIVE_MASK) >> Self::DRIVE_OFFSET
    }

    /// Write **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn write_drive(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DRIVE_MASK) | ((value << Self::DRIVE_OFFSET) & Self::DRIVE_MASK));
    }

    /// **PUE** `[3]` (RW)
    ///
    /// Pull up enable
    #[inline(always)]
    pub fn pue(&self) -> bool {
        (self.value.read_bits() & Self::PUE_MASK) != 0
    }

    /// Set PUE bit atomically (RW)
    #[inline(always)]
    pub fn set_pue(&self) {
        self.set_bits(Self::PUE_MASK);
    }

    /// Clear PUE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pue(&self) {
        self.clr_bits(Self::PUE_MASK);
    }

    /// **PDE** `[2]` (RW)
    ///
    /// Pull down enable
    #[inline(always)]
    pub fn pde(&self) -> bool {
        (self.value.read_bits() & Self::PDE_MASK) != 0
    }

    /// Set PDE bit atomically (RW)
    #[inline(always)]
    pub fn set_pde(&self) {
        self.set_bits(Self::PDE_MASK);
    }

    /// Clear PDE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pde(&self) {
        self.clr_bits(Self::PDE_MASK);
    }

    /// **SCHMITT** `[1]` (RW)
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub fn schmitt(&self) -> bool {
        (self.value.read_bits() & Self::SCHMITT_MASK) != 0
    }

    /// Set SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn set_schmitt(&self) {
        self.set_bits(Self::SCHMITT_MASK);
    }

    /// Clear SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn clr_schmitt(&self) {
        self.clr_bits(Self::SCHMITT_MASK);
    }

    /// **SLEWFAST** `[0]` (RW)
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub fn slewfast(&self) -> bool {
        (self.value.read_bits() & Self::SLEWFAST_MASK) != 0
    }

    /// Set SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn set_slewfast(&self) {
        self.set_bits(Self::SLEWFAST_MASK);
    }

    /// Clear SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn clr_slewfast(&self) {
        self.clr_bits(Self::SLEWFAST_MASK);
    }

}

/// GPIO40 value type for batch read/modify/write operations
///
/// Use `gpio40.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpio40.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Gpio40Val {
    bits: u32,
}

impl Gpio40Val {
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
        Self { bits: 0x00000116 }
    }

    /// Read **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn iso(&self) -> bool {
        (self.bits & Gpio40::ISO_MASK) != 0
    }

    /// Read **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn od(&self) -> bool {
        (self.bits & Gpio40::OD_MASK) != 0
    }

    /// Read **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        (self.bits & Gpio40::IE_MASK) != 0
    }

    /// Read **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn drive(&self) -> u32 {
        (self.bits & Gpio40::DRIVE_MASK) >> Gpio40::DRIVE_OFFSET
    }

    /// Read **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn pue(&self) -> bool {
        (self.bits & Gpio40::PUE_MASK) != 0
    }

    /// Read **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn pde(&self) -> bool {
        (self.bits & Gpio40::PDE_MASK) != 0
    }

    /// Read **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn schmitt(&self) -> bool {
        (self.bits & Gpio40::SCHMITT_MASK) != 0
    }

    /// Read **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn slewfast(&self) -> bool {
        (self.bits & Gpio40::SLEWFAST_MASK) != 0
    }

    /// Set **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn with_iso(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio40::ISO_MASK
            } else {
                self.bits & !Gpio40::ISO_MASK
            }
        }
    }

    /// Set **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn with_od(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio40::OD_MASK
            } else {
                self.bits & !Gpio40::OD_MASK
            }
        }
    }

    /// Set **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn with_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio40::IE_MASK
            } else {
                self.bits & !Gpio40::IE_MASK
            }
        }
    }

    /// Set **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn with_drive(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Gpio40::DRIVE_MASK) | ((value << Gpio40::DRIVE_OFFSET) & Gpio40::DRIVE_MASK)
        }
    }

    /// Set **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn with_pue(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio40::PUE_MASK
            } else {
                self.bits & !Gpio40::PUE_MASK
            }
        }
    }

    /// Set **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn with_pde(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio40::PDE_MASK
            } else {
                self.bits & !Gpio40::PDE_MASK
            }
        }
    }

    /// Set **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn with_schmitt(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio40::SCHMITT_MASK
            } else {
                self.bits & !Gpio40::SCHMITT_MASK
            }
        }
    }

    /// Set **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn with_slewfast(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio40::SLEWFAST_MASK
            } else {
                self.bits & !Gpio40::SLEWFAST_MASK
            }
        }
    }

}

impl Default for Gpio40Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Gpio40 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Gpio40Val {
        Gpio40Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Gpio40Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Gpio40Val) -> Gpio40Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// GPIO41 Register
///
/// Address offset: 0x00a8
/// Reset value: 0x00000116
///
/// ## Fields
///
/// - **ISO** `[8]` (RW)
///
///   Pad isolation control. Remove this once the pad is configured by software.
///
/// - **OD** `[7]` (RW)
///
///   Output disable. Has priority over output enable from peripherals
///
/// - **IE** `[6]` (RW)
///
///   Input enable
///
/// - **DRIVE** `[5:4]` (RW)
///
///   Drive strength.
///
/// - **PUE** `[3]` (RW)
///
///   Pull up enable
///
/// - **PDE** `[2]` (RW)
///
///   Pull down enable
///
/// - **SCHMITT** `[1]` (RW)
///
///   Enable schmitt trigger
///
/// - **SLEWFAST** `[0]` (RW)
///
///   Slew rate control. 1 = Fast, 0 = Slow
#[repr(transparent)]
pub struct Gpio41 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Gpio41 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Gpio41 {
    // ======== Field Constants ========

    /// **ISO**
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ISO_OFFSET: u32 = 8;
    pub const ISO_MASK: u32 = 0x00000100;
    pub const ISO: u32 = 1 << 8;

    /// **OD**
    ///
    /// Output disable. Has priority over output enable from peripherals
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const OD_OFFSET: u32 = 7;
    pub const OD_MASK: u32 = 0x00000080;
    pub const OD: u32 = 1 << 7;

    /// **IE**
    ///
    /// Input enable
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const IE_OFFSET: u32 = 6;
    pub const IE_MASK: u32 = 0x00000040;
    pub const IE: u32 = 1 << 6;

    /// **DRIVE**
    ///
    /// Drive strength.
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const DRIVE_OFFSET: u32 = 4;
    pub const DRIVE_MASK: u32 = 0x00000030;
    pub const DRIVE_2M_A: u32 = 0 << 4;
    pub const DRIVE_2M_A_VAL: u32 = 0;
    pub const DRIVE_4M_A: u32 = 1 << 4;
    pub const DRIVE_4M_A_VAL: u32 = 1;
    pub const DRIVE_8M_A: u32 = 2 << 4;
    pub const DRIVE_8M_A_VAL: u32 = 2;
    pub const DRIVE_12M_A: u32 = 3 << 4;
    pub const DRIVE_12M_A_VAL: u32 = 3;

    /// **PUE**
    ///
    /// Pull up enable
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const PUE_OFFSET: u32 = 3;
    pub const PUE_MASK: u32 = 0x00000008;
    pub const PUE: u32 = 1 << 3;

    /// **PDE**
    ///
    /// Pull down enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const PDE_OFFSET: u32 = 2;
    pub const PDE_MASK: u32 = 0x00000004;
    pub const PDE: u32 = 1 << 2;

    /// **SCHMITT**
    ///
    /// Enable schmitt trigger
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SCHMITT_OFFSET: u32 = 1;
    pub const SCHMITT_MASK: u32 = 0x00000002;
    pub const SCHMITT: u32 = 1 << 1;

    /// **SLEWFAST**
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SLEWFAST_OFFSET: u32 = 0;
    pub const SLEWFAST_MASK: u32 = 0x00000001;
    pub const SLEWFAST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ISO** `[8]` (RW)
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub fn iso(&self) -> bool {
        (self.value.read_bits() & Self::ISO_MASK) != 0
    }

    /// Set ISO bit atomically (RW)
    #[inline(always)]
    pub fn set_iso(&self) {
        self.set_bits(Self::ISO_MASK);
    }

    /// Clear ISO bit atomically (RW)
    #[inline(always)]
    pub fn clr_iso(&self) {
        self.clr_bits(Self::ISO_MASK);
    }

    /// **OD** `[7]` (RW)
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub fn od(&self) -> bool {
        (self.value.read_bits() & Self::OD_MASK) != 0
    }

    /// Set OD bit atomically (RW)
    #[inline(always)]
    pub fn set_od(&self) {
        self.set_bits(Self::OD_MASK);
    }

    /// Clear OD bit atomically (RW)
    #[inline(always)]
    pub fn clr_od(&self) {
        self.clr_bits(Self::OD_MASK);
    }

    /// **IE** `[6]` (RW)
    ///
    /// Input enable
    #[inline(always)]
    pub fn ie(&self) -> bool {
        (self.value.read_bits() & Self::IE_MASK) != 0
    }

    /// Set IE bit atomically (RW)
    #[inline(always)]
    pub fn set_ie(&self) {
        self.set_bits(Self::IE_MASK);
    }

    /// Clear IE bit atomically (RW)
    #[inline(always)]
    pub fn clr_ie(&self) {
        self.clr_bits(Self::IE_MASK);
    }

    /// **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn drive(&self) -> u32 {
        (self.value.read_bits() & Self::DRIVE_MASK) >> Self::DRIVE_OFFSET
    }

    /// Write **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn write_drive(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DRIVE_MASK) | ((value << Self::DRIVE_OFFSET) & Self::DRIVE_MASK));
    }

    /// **PUE** `[3]` (RW)
    ///
    /// Pull up enable
    #[inline(always)]
    pub fn pue(&self) -> bool {
        (self.value.read_bits() & Self::PUE_MASK) != 0
    }

    /// Set PUE bit atomically (RW)
    #[inline(always)]
    pub fn set_pue(&self) {
        self.set_bits(Self::PUE_MASK);
    }

    /// Clear PUE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pue(&self) {
        self.clr_bits(Self::PUE_MASK);
    }

    /// **PDE** `[2]` (RW)
    ///
    /// Pull down enable
    #[inline(always)]
    pub fn pde(&self) -> bool {
        (self.value.read_bits() & Self::PDE_MASK) != 0
    }

    /// Set PDE bit atomically (RW)
    #[inline(always)]
    pub fn set_pde(&self) {
        self.set_bits(Self::PDE_MASK);
    }

    /// Clear PDE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pde(&self) {
        self.clr_bits(Self::PDE_MASK);
    }

    /// **SCHMITT** `[1]` (RW)
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub fn schmitt(&self) -> bool {
        (self.value.read_bits() & Self::SCHMITT_MASK) != 0
    }

    /// Set SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn set_schmitt(&self) {
        self.set_bits(Self::SCHMITT_MASK);
    }

    /// Clear SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn clr_schmitt(&self) {
        self.clr_bits(Self::SCHMITT_MASK);
    }

    /// **SLEWFAST** `[0]` (RW)
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub fn slewfast(&self) -> bool {
        (self.value.read_bits() & Self::SLEWFAST_MASK) != 0
    }

    /// Set SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn set_slewfast(&self) {
        self.set_bits(Self::SLEWFAST_MASK);
    }

    /// Clear SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn clr_slewfast(&self) {
        self.clr_bits(Self::SLEWFAST_MASK);
    }

}

/// GPIO41 value type for batch read/modify/write operations
///
/// Use `gpio41.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpio41.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Gpio41Val {
    bits: u32,
}

impl Gpio41Val {
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
        Self { bits: 0x00000116 }
    }

    /// Read **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn iso(&self) -> bool {
        (self.bits & Gpio41::ISO_MASK) != 0
    }

    /// Read **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn od(&self) -> bool {
        (self.bits & Gpio41::OD_MASK) != 0
    }

    /// Read **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        (self.bits & Gpio41::IE_MASK) != 0
    }

    /// Read **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn drive(&self) -> u32 {
        (self.bits & Gpio41::DRIVE_MASK) >> Gpio41::DRIVE_OFFSET
    }

    /// Read **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn pue(&self) -> bool {
        (self.bits & Gpio41::PUE_MASK) != 0
    }

    /// Read **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn pde(&self) -> bool {
        (self.bits & Gpio41::PDE_MASK) != 0
    }

    /// Read **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn schmitt(&self) -> bool {
        (self.bits & Gpio41::SCHMITT_MASK) != 0
    }

    /// Read **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn slewfast(&self) -> bool {
        (self.bits & Gpio41::SLEWFAST_MASK) != 0
    }

    /// Set **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn with_iso(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio41::ISO_MASK
            } else {
                self.bits & !Gpio41::ISO_MASK
            }
        }
    }

    /// Set **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn with_od(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio41::OD_MASK
            } else {
                self.bits & !Gpio41::OD_MASK
            }
        }
    }

    /// Set **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn with_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio41::IE_MASK
            } else {
                self.bits & !Gpio41::IE_MASK
            }
        }
    }

    /// Set **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn with_drive(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Gpio41::DRIVE_MASK) | ((value << Gpio41::DRIVE_OFFSET) & Gpio41::DRIVE_MASK)
        }
    }

    /// Set **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn with_pue(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio41::PUE_MASK
            } else {
                self.bits & !Gpio41::PUE_MASK
            }
        }
    }

    /// Set **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn with_pde(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio41::PDE_MASK
            } else {
                self.bits & !Gpio41::PDE_MASK
            }
        }
    }

    /// Set **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn with_schmitt(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio41::SCHMITT_MASK
            } else {
                self.bits & !Gpio41::SCHMITT_MASK
            }
        }
    }

    /// Set **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn with_slewfast(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio41::SLEWFAST_MASK
            } else {
                self.bits & !Gpio41::SLEWFAST_MASK
            }
        }
    }

}

impl Default for Gpio41Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Gpio41 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Gpio41Val {
        Gpio41Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Gpio41Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Gpio41Val) -> Gpio41Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// GPIO42 Register
///
/// Address offset: 0x00ac
/// Reset value: 0x00000116
///
/// ## Fields
///
/// - **ISO** `[8]` (RW)
///
///   Pad isolation control. Remove this once the pad is configured by software.
///
/// - **OD** `[7]` (RW)
///
///   Output disable. Has priority over output enable from peripherals
///
/// - **IE** `[6]` (RW)
///
///   Input enable
///
/// - **DRIVE** `[5:4]` (RW)
///
///   Drive strength.
///
/// - **PUE** `[3]` (RW)
///
///   Pull up enable
///
/// - **PDE** `[2]` (RW)
///
///   Pull down enable
///
/// - **SCHMITT** `[1]` (RW)
///
///   Enable schmitt trigger
///
/// - **SLEWFAST** `[0]` (RW)
///
///   Slew rate control. 1 = Fast, 0 = Slow
#[repr(transparent)]
pub struct Gpio42 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Gpio42 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Gpio42 {
    // ======== Field Constants ========

    /// **ISO**
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ISO_OFFSET: u32 = 8;
    pub const ISO_MASK: u32 = 0x00000100;
    pub const ISO: u32 = 1 << 8;

    /// **OD**
    ///
    /// Output disable. Has priority over output enable from peripherals
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const OD_OFFSET: u32 = 7;
    pub const OD_MASK: u32 = 0x00000080;
    pub const OD: u32 = 1 << 7;

    /// **IE**
    ///
    /// Input enable
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const IE_OFFSET: u32 = 6;
    pub const IE_MASK: u32 = 0x00000040;
    pub const IE: u32 = 1 << 6;

    /// **DRIVE**
    ///
    /// Drive strength.
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const DRIVE_OFFSET: u32 = 4;
    pub const DRIVE_MASK: u32 = 0x00000030;
    pub const DRIVE_2M_A: u32 = 0 << 4;
    pub const DRIVE_2M_A_VAL: u32 = 0;
    pub const DRIVE_4M_A: u32 = 1 << 4;
    pub const DRIVE_4M_A_VAL: u32 = 1;
    pub const DRIVE_8M_A: u32 = 2 << 4;
    pub const DRIVE_8M_A_VAL: u32 = 2;
    pub const DRIVE_12M_A: u32 = 3 << 4;
    pub const DRIVE_12M_A_VAL: u32 = 3;

    /// **PUE**
    ///
    /// Pull up enable
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const PUE_OFFSET: u32 = 3;
    pub const PUE_MASK: u32 = 0x00000008;
    pub const PUE: u32 = 1 << 3;

    /// **PDE**
    ///
    /// Pull down enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const PDE_OFFSET: u32 = 2;
    pub const PDE_MASK: u32 = 0x00000004;
    pub const PDE: u32 = 1 << 2;

    /// **SCHMITT**
    ///
    /// Enable schmitt trigger
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SCHMITT_OFFSET: u32 = 1;
    pub const SCHMITT_MASK: u32 = 0x00000002;
    pub const SCHMITT: u32 = 1 << 1;

    /// **SLEWFAST**
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SLEWFAST_OFFSET: u32 = 0;
    pub const SLEWFAST_MASK: u32 = 0x00000001;
    pub const SLEWFAST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ISO** `[8]` (RW)
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub fn iso(&self) -> bool {
        (self.value.read_bits() & Self::ISO_MASK) != 0
    }

    /// Set ISO bit atomically (RW)
    #[inline(always)]
    pub fn set_iso(&self) {
        self.set_bits(Self::ISO_MASK);
    }

    /// Clear ISO bit atomically (RW)
    #[inline(always)]
    pub fn clr_iso(&self) {
        self.clr_bits(Self::ISO_MASK);
    }

    /// **OD** `[7]` (RW)
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub fn od(&self) -> bool {
        (self.value.read_bits() & Self::OD_MASK) != 0
    }

    /// Set OD bit atomically (RW)
    #[inline(always)]
    pub fn set_od(&self) {
        self.set_bits(Self::OD_MASK);
    }

    /// Clear OD bit atomically (RW)
    #[inline(always)]
    pub fn clr_od(&self) {
        self.clr_bits(Self::OD_MASK);
    }

    /// **IE** `[6]` (RW)
    ///
    /// Input enable
    #[inline(always)]
    pub fn ie(&self) -> bool {
        (self.value.read_bits() & Self::IE_MASK) != 0
    }

    /// Set IE bit atomically (RW)
    #[inline(always)]
    pub fn set_ie(&self) {
        self.set_bits(Self::IE_MASK);
    }

    /// Clear IE bit atomically (RW)
    #[inline(always)]
    pub fn clr_ie(&self) {
        self.clr_bits(Self::IE_MASK);
    }

    /// **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn drive(&self) -> u32 {
        (self.value.read_bits() & Self::DRIVE_MASK) >> Self::DRIVE_OFFSET
    }

    /// Write **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn write_drive(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DRIVE_MASK) | ((value << Self::DRIVE_OFFSET) & Self::DRIVE_MASK));
    }

    /// **PUE** `[3]` (RW)
    ///
    /// Pull up enable
    #[inline(always)]
    pub fn pue(&self) -> bool {
        (self.value.read_bits() & Self::PUE_MASK) != 0
    }

    /// Set PUE bit atomically (RW)
    #[inline(always)]
    pub fn set_pue(&self) {
        self.set_bits(Self::PUE_MASK);
    }

    /// Clear PUE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pue(&self) {
        self.clr_bits(Self::PUE_MASK);
    }

    /// **PDE** `[2]` (RW)
    ///
    /// Pull down enable
    #[inline(always)]
    pub fn pde(&self) -> bool {
        (self.value.read_bits() & Self::PDE_MASK) != 0
    }

    /// Set PDE bit atomically (RW)
    #[inline(always)]
    pub fn set_pde(&self) {
        self.set_bits(Self::PDE_MASK);
    }

    /// Clear PDE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pde(&self) {
        self.clr_bits(Self::PDE_MASK);
    }

    /// **SCHMITT** `[1]` (RW)
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub fn schmitt(&self) -> bool {
        (self.value.read_bits() & Self::SCHMITT_MASK) != 0
    }

    /// Set SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn set_schmitt(&self) {
        self.set_bits(Self::SCHMITT_MASK);
    }

    /// Clear SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn clr_schmitt(&self) {
        self.clr_bits(Self::SCHMITT_MASK);
    }

    /// **SLEWFAST** `[0]` (RW)
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub fn slewfast(&self) -> bool {
        (self.value.read_bits() & Self::SLEWFAST_MASK) != 0
    }

    /// Set SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn set_slewfast(&self) {
        self.set_bits(Self::SLEWFAST_MASK);
    }

    /// Clear SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn clr_slewfast(&self) {
        self.clr_bits(Self::SLEWFAST_MASK);
    }

}

/// GPIO42 value type for batch read/modify/write operations
///
/// Use `gpio42.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpio42.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Gpio42Val {
    bits: u32,
}

impl Gpio42Val {
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
        Self { bits: 0x00000116 }
    }

    /// Read **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn iso(&self) -> bool {
        (self.bits & Gpio42::ISO_MASK) != 0
    }

    /// Read **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn od(&self) -> bool {
        (self.bits & Gpio42::OD_MASK) != 0
    }

    /// Read **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        (self.bits & Gpio42::IE_MASK) != 0
    }

    /// Read **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn drive(&self) -> u32 {
        (self.bits & Gpio42::DRIVE_MASK) >> Gpio42::DRIVE_OFFSET
    }

    /// Read **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn pue(&self) -> bool {
        (self.bits & Gpio42::PUE_MASK) != 0
    }

    /// Read **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn pde(&self) -> bool {
        (self.bits & Gpio42::PDE_MASK) != 0
    }

    /// Read **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn schmitt(&self) -> bool {
        (self.bits & Gpio42::SCHMITT_MASK) != 0
    }

    /// Read **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn slewfast(&self) -> bool {
        (self.bits & Gpio42::SLEWFAST_MASK) != 0
    }

    /// Set **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn with_iso(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio42::ISO_MASK
            } else {
                self.bits & !Gpio42::ISO_MASK
            }
        }
    }

    /// Set **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn with_od(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio42::OD_MASK
            } else {
                self.bits & !Gpio42::OD_MASK
            }
        }
    }

    /// Set **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn with_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio42::IE_MASK
            } else {
                self.bits & !Gpio42::IE_MASK
            }
        }
    }

    /// Set **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn with_drive(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Gpio42::DRIVE_MASK) | ((value << Gpio42::DRIVE_OFFSET) & Gpio42::DRIVE_MASK)
        }
    }

    /// Set **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn with_pue(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio42::PUE_MASK
            } else {
                self.bits & !Gpio42::PUE_MASK
            }
        }
    }

    /// Set **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn with_pde(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio42::PDE_MASK
            } else {
                self.bits & !Gpio42::PDE_MASK
            }
        }
    }

    /// Set **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn with_schmitt(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio42::SCHMITT_MASK
            } else {
                self.bits & !Gpio42::SCHMITT_MASK
            }
        }
    }

    /// Set **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn with_slewfast(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio42::SLEWFAST_MASK
            } else {
                self.bits & !Gpio42::SLEWFAST_MASK
            }
        }
    }

}

impl Default for Gpio42Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Gpio42 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Gpio42Val {
        Gpio42Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Gpio42Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Gpio42Val) -> Gpio42Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// GPIO43 Register
///
/// Address offset: 0x00b0
/// Reset value: 0x00000116
///
/// ## Fields
///
/// - **ISO** `[8]` (RW)
///
///   Pad isolation control. Remove this once the pad is configured by software.
///
/// - **OD** `[7]` (RW)
///
///   Output disable. Has priority over output enable from peripherals
///
/// - **IE** `[6]` (RW)
///
///   Input enable
///
/// - **DRIVE** `[5:4]` (RW)
///
///   Drive strength.
///
/// - **PUE** `[3]` (RW)
///
///   Pull up enable
///
/// - **PDE** `[2]` (RW)
///
///   Pull down enable
///
/// - **SCHMITT** `[1]` (RW)
///
///   Enable schmitt trigger
///
/// - **SLEWFAST** `[0]` (RW)
///
///   Slew rate control. 1 = Fast, 0 = Slow
#[repr(transparent)]
pub struct Gpio43 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Gpio43 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Gpio43 {
    // ======== Field Constants ========

    /// **ISO**
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ISO_OFFSET: u32 = 8;
    pub const ISO_MASK: u32 = 0x00000100;
    pub const ISO: u32 = 1 << 8;

    /// **OD**
    ///
    /// Output disable. Has priority over output enable from peripherals
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const OD_OFFSET: u32 = 7;
    pub const OD_MASK: u32 = 0x00000080;
    pub const OD: u32 = 1 << 7;

    /// **IE**
    ///
    /// Input enable
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const IE_OFFSET: u32 = 6;
    pub const IE_MASK: u32 = 0x00000040;
    pub const IE: u32 = 1 << 6;

    /// **DRIVE**
    ///
    /// Drive strength.
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const DRIVE_OFFSET: u32 = 4;
    pub const DRIVE_MASK: u32 = 0x00000030;
    pub const DRIVE_2M_A: u32 = 0 << 4;
    pub const DRIVE_2M_A_VAL: u32 = 0;
    pub const DRIVE_4M_A: u32 = 1 << 4;
    pub const DRIVE_4M_A_VAL: u32 = 1;
    pub const DRIVE_8M_A: u32 = 2 << 4;
    pub const DRIVE_8M_A_VAL: u32 = 2;
    pub const DRIVE_12M_A: u32 = 3 << 4;
    pub const DRIVE_12M_A_VAL: u32 = 3;

    /// **PUE**
    ///
    /// Pull up enable
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const PUE_OFFSET: u32 = 3;
    pub const PUE_MASK: u32 = 0x00000008;
    pub const PUE: u32 = 1 << 3;

    /// **PDE**
    ///
    /// Pull down enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const PDE_OFFSET: u32 = 2;
    pub const PDE_MASK: u32 = 0x00000004;
    pub const PDE: u32 = 1 << 2;

    /// **SCHMITT**
    ///
    /// Enable schmitt trigger
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SCHMITT_OFFSET: u32 = 1;
    pub const SCHMITT_MASK: u32 = 0x00000002;
    pub const SCHMITT: u32 = 1 << 1;

    /// **SLEWFAST**
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SLEWFAST_OFFSET: u32 = 0;
    pub const SLEWFAST_MASK: u32 = 0x00000001;
    pub const SLEWFAST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ISO** `[8]` (RW)
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub fn iso(&self) -> bool {
        (self.value.read_bits() & Self::ISO_MASK) != 0
    }

    /// Set ISO bit atomically (RW)
    #[inline(always)]
    pub fn set_iso(&self) {
        self.set_bits(Self::ISO_MASK);
    }

    /// Clear ISO bit atomically (RW)
    #[inline(always)]
    pub fn clr_iso(&self) {
        self.clr_bits(Self::ISO_MASK);
    }

    /// **OD** `[7]` (RW)
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub fn od(&self) -> bool {
        (self.value.read_bits() & Self::OD_MASK) != 0
    }

    /// Set OD bit atomically (RW)
    #[inline(always)]
    pub fn set_od(&self) {
        self.set_bits(Self::OD_MASK);
    }

    /// Clear OD bit atomically (RW)
    #[inline(always)]
    pub fn clr_od(&self) {
        self.clr_bits(Self::OD_MASK);
    }

    /// **IE** `[6]` (RW)
    ///
    /// Input enable
    #[inline(always)]
    pub fn ie(&self) -> bool {
        (self.value.read_bits() & Self::IE_MASK) != 0
    }

    /// Set IE bit atomically (RW)
    #[inline(always)]
    pub fn set_ie(&self) {
        self.set_bits(Self::IE_MASK);
    }

    /// Clear IE bit atomically (RW)
    #[inline(always)]
    pub fn clr_ie(&self) {
        self.clr_bits(Self::IE_MASK);
    }

    /// **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn drive(&self) -> u32 {
        (self.value.read_bits() & Self::DRIVE_MASK) >> Self::DRIVE_OFFSET
    }

    /// Write **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn write_drive(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DRIVE_MASK) | ((value << Self::DRIVE_OFFSET) & Self::DRIVE_MASK));
    }

    /// **PUE** `[3]` (RW)
    ///
    /// Pull up enable
    #[inline(always)]
    pub fn pue(&self) -> bool {
        (self.value.read_bits() & Self::PUE_MASK) != 0
    }

    /// Set PUE bit atomically (RW)
    #[inline(always)]
    pub fn set_pue(&self) {
        self.set_bits(Self::PUE_MASK);
    }

    /// Clear PUE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pue(&self) {
        self.clr_bits(Self::PUE_MASK);
    }

    /// **PDE** `[2]` (RW)
    ///
    /// Pull down enable
    #[inline(always)]
    pub fn pde(&self) -> bool {
        (self.value.read_bits() & Self::PDE_MASK) != 0
    }

    /// Set PDE bit atomically (RW)
    #[inline(always)]
    pub fn set_pde(&self) {
        self.set_bits(Self::PDE_MASK);
    }

    /// Clear PDE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pde(&self) {
        self.clr_bits(Self::PDE_MASK);
    }

    /// **SCHMITT** `[1]` (RW)
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub fn schmitt(&self) -> bool {
        (self.value.read_bits() & Self::SCHMITT_MASK) != 0
    }

    /// Set SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn set_schmitt(&self) {
        self.set_bits(Self::SCHMITT_MASK);
    }

    /// Clear SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn clr_schmitt(&self) {
        self.clr_bits(Self::SCHMITT_MASK);
    }

    /// **SLEWFAST** `[0]` (RW)
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub fn slewfast(&self) -> bool {
        (self.value.read_bits() & Self::SLEWFAST_MASK) != 0
    }

    /// Set SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn set_slewfast(&self) {
        self.set_bits(Self::SLEWFAST_MASK);
    }

    /// Clear SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn clr_slewfast(&self) {
        self.clr_bits(Self::SLEWFAST_MASK);
    }

}

/// GPIO43 value type for batch read/modify/write operations
///
/// Use `gpio43.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpio43.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Gpio43Val {
    bits: u32,
}

impl Gpio43Val {
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
        Self { bits: 0x00000116 }
    }

    /// Read **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn iso(&self) -> bool {
        (self.bits & Gpio43::ISO_MASK) != 0
    }

    /// Read **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn od(&self) -> bool {
        (self.bits & Gpio43::OD_MASK) != 0
    }

    /// Read **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        (self.bits & Gpio43::IE_MASK) != 0
    }

    /// Read **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn drive(&self) -> u32 {
        (self.bits & Gpio43::DRIVE_MASK) >> Gpio43::DRIVE_OFFSET
    }

    /// Read **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn pue(&self) -> bool {
        (self.bits & Gpio43::PUE_MASK) != 0
    }

    /// Read **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn pde(&self) -> bool {
        (self.bits & Gpio43::PDE_MASK) != 0
    }

    /// Read **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn schmitt(&self) -> bool {
        (self.bits & Gpio43::SCHMITT_MASK) != 0
    }

    /// Read **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn slewfast(&self) -> bool {
        (self.bits & Gpio43::SLEWFAST_MASK) != 0
    }

    /// Set **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn with_iso(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio43::ISO_MASK
            } else {
                self.bits & !Gpio43::ISO_MASK
            }
        }
    }

    /// Set **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn with_od(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio43::OD_MASK
            } else {
                self.bits & !Gpio43::OD_MASK
            }
        }
    }

    /// Set **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn with_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio43::IE_MASK
            } else {
                self.bits & !Gpio43::IE_MASK
            }
        }
    }

    /// Set **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn with_drive(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Gpio43::DRIVE_MASK) | ((value << Gpio43::DRIVE_OFFSET) & Gpio43::DRIVE_MASK)
        }
    }

    /// Set **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn with_pue(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio43::PUE_MASK
            } else {
                self.bits & !Gpio43::PUE_MASK
            }
        }
    }

    /// Set **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn with_pde(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio43::PDE_MASK
            } else {
                self.bits & !Gpio43::PDE_MASK
            }
        }
    }

    /// Set **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn with_schmitt(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio43::SCHMITT_MASK
            } else {
                self.bits & !Gpio43::SCHMITT_MASK
            }
        }
    }

    /// Set **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn with_slewfast(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio43::SLEWFAST_MASK
            } else {
                self.bits & !Gpio43::SLEWFAST_MASK
            }
        }
    }

}

impl Default for Gpio43Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Gpio43 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Gpio43Val {
        Gpio43Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Gpio43Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Gpio43Val) -> Gpio43Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// GPIO44 Register
///
/// Address offset: 0x00b4
/// Reset value: 0x00000116
///
/// ## Fields
///
/// - **ISO** `[8]` (RW)
///
///   Pad isolation control. Remove this once the pad is configured by software.
///
/// - **OD** `[7]` (RW)
///
///   Output disable. Has priority over output enable from peripherals
///
/// - **IE** `[6]` (RW)
///
///   Input enable
///
/// - **DRIVE** `[5:4]` (RW)
///
///   Drive strength.
///
/// - **PUE** `[3]` (RW)
///
///   Pull up enable
///
/// - **PDE** `[2]` (RW)
///
///   Pull down enable
///
/// - **SCHMITT** `[1]` (RW)
///
///   Enable schmitt trigger
///
/// - **SLEWFAST** `[0]` (RW)
///
///   Slew rate control. 1 = Fast, 0 = Slow
#[repr(transparent)]
pub struct Gpio44 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Gpio44 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Gpio44 {
    // ======== Field Constants ========

    /// **ISO**
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ISO_OFFSET: u32 = 8;
    pub const ISO_MASK: u32 = 0x00000100;
    pub const ISO: u32 = 1 << 8;

    /// **OD**
    ///
    /// Output disable. Has priority over output enable from peripherals
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const OD_OFFSET: u32 = 7;
    pub const OD_MASK: u32 = 0x00000080;
    pub const OD: u32 = 1 << 7;

    /// **IE**
    ///
    /// Input enable
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const IE_OFFSET: u32 = 6;
    pub const IE_MASK: u32 = 0x00000040;
    pub const IE: u32 = 1 << 6;

    /// **DRIVE**
    ///
    /// Drive strength.
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const DRIVE_OFFSET: u32 = 4;
    pub const DRIVE_MASK: u32 = 0x00000030;
    pub const DRIVE_2M_A: u32 = 0 << 4;
    pub const DRIVE_2M_A_VAL: u32 = 0;
    pub const DRIVE_4M_A: u32 = 1 << 4;
    pub const DRIVE_4M_A_VAL: u32 = 1;
    pub const DRIVE_8M_A: u32 = 2 << 4;
    pub const DRIVE_8M_A_VAL: u32 = 2;
    pub const DRIVE_12M_A: u32 = 3 << 4;
    pub const DRIVE_12M_A_VAL: u32 = 3;

    /// **PUE**
    ///
    /// Pull up enable
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const PUE_OFFSET: u32 = 3;
    pub const PUE_MASK: u32 = 0x00000008;
    pub const PUE: u32 = 1 << 3;

    /// **PDE**
    ///
    /// Pull down enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const PDE_OFFSET: u32 = 2;
    pub const PDE_MASK: u32 = 0x00000004;
    pub const PDE: u32 = 1 << 2;

    /// **SCHMITT**
    ///
    /// Enable schmitt trigger
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SCHMITT_OFFSET: u32 = 1;
    pub const SCHMITT_MASK: u32 = 0x00000002;
    pub const SCHMITT: u32 = 1 << 1;

    /// **SLEWFAST**
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SLEWFAST_OFFSET: u32 = 0;
    pub const SLEWFAST_MASK: u32 = 0x00000001;
    pub const SLEWFAST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ISO** `[8]` (RW)
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub fn iso(&self) -> bool {
        (self.value.read_bits() & Self::ISO_MASK) != 0
    }

    /// Set ISO bit atomically (RW)
    #[inline(always)]
    pub fn set_iso(&self) {
        self.set_bits(Self::ISO_MASK);
    }

    /// Clear ISO bit atomically (RW)
    #[inline(always)]
    pub fn clr_iso(&self) {
        self.clr_bits(Self::ISO_MASK);
    }

    /// **OD** `[7]` (RW)
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub fn od(&self) -> bool {
        (self.value.read_bits() & Self::OD_MASK) != 0
    }

    /// Set OD bit atomically (RW)
    #[inline(always)]
    pub fn set_od(&self) {
        self.set_bits(Self::OD_MASK);
    }

    /// Clear OD bit atomically (RW)
    #[inline(always)]
    pub fn clr_od(&self) {
        self.clr_bits(Self::OD_MASK);
    }

    /// **IE** `[6]` (RW)
    ///
    /// Input enable
    #[inline(always)]
    pub fn ie(&self) -> bool {
        (self.value.read_bits() & Self::IE_MASK) != 0
    }

    /// Set IE bit atomically (RW)
    #[inline(always)]
    pub fn set_ie(&self) {
        self.set_bits(Self::IE_MASK);
    }

    /// Clear IE bit atomically (RW)
    #[inline(always)]
    pub fn clr_ie(&self) {
        self.clr_bits(Self::IE_MASK);
    }

    /// **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn drive(&self) -> u32 {
        (self.value.read_bits() & Self::DRIVE_MASK) >> Self::DRIVE_OFFSET
    }

    /// Write **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn write_drive(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DRIVE_MASK) | ((value << Self::DRIVE_OFFSET) & Self::DRIVE_MASK));
    }

    /// **PUE** `[3]` (RW)
    ///
    /// Pull up enable
    #[inline(always)]
    pub fn pue(&self) -> bool {
        (self.value.read_bits() & Self::PUE_MASK) != 0
    }

    /// Set PUE bit atomically (RW)
    #[inline(always)]
    pub fn set_pue(&self) {
        self.set_bits(Self::PUE_MASK);
    }

    /// Clear PUE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pue(&self) {
        self.clr_bits(Self::PUE_MASK);
    }

    /// **PDE** `[2]` (RW)
    ///
    /// Pull down enable
    #[inline(always)]
    pub fn pde(&self) -> bool {
        (self.value.read_bits() & Self::PDE_MASK) != 0
    }

    /// Set PDE bit atomically (RW)
    #[inline(always)]
    pub fn set_pde(&self) {
        self.set_bits(Self::PDE_MASK);
    }

    /// Clear PDE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pde(&self) {
        self.clr_bits(Self::PDE_MASK);
    }

    /// **SCHMITT** `[1]` (RW)
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub fn schmitt(&self) -> bool {
        (self.value.read_bits() & Self::SCHMITT_MASK) != 0
    }

    /// Set SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn set_schmitt(&self) {
        self.set_bits(Self::SCHMITT_MASK);
    }

    /// Clear SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn clr_schmitt(&self) {
        self.clr_bits(Self::SCHMITT_MASK);
    }

    /// **SLEWFAST** `[0]` (RW)
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub fn slewfast(&self) -> bool {
        (self.value.read_bits() & Self::SLEWFAST_MASK) != 0
    }

    /// Set SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn set_slewfast(&self) {
        self.set_bits(Self::SLEWFAST_MASK);
    }

    /// Clear SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn clr_slewfast(&self) {
        self.clr_bits(Self::SLEWFAST_MASK);
    }

}

/// GPIO44 value type for batch read/modify/write operations
///
/// Use `gpio44.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpio44.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Gpio44Val {
    bits: u32,
}

impl Gpio44Val {
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
        Self { bits: 0x00000116 }
    }

    /// Read **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn iso(&self) -> bool {
        (self.bits & Gpio44::ISO_MASK) != 0
    }

    /// Read **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn od(&self) -> bool {
        (self.bits & Gpio44::OD_MASK) != 0
    }

    /// Read **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        (self.bits & Gpio44::IE_MASK) != 0
    }

    /// Read **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn drive(&self) -> u32 {
        (self.bits & Gpio44::DRIVE_MASK) >> Gpio44::DRIVE_OFFSET
    }

    /// Read **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn pue(&self) -> bool {
        (self.bits & Gpio44::PUE_MASK) != 0
    }

    /// Read **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn pde(&self) -> bool {
        (self.bits & Gpio44::PDE_MASK) != 0
    }

    /// Read **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn schmitt(&self) -> bool {
        (self.bits & Gpio44::SCHMITT_MASK) != 0
    }

    /// Read **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn slewfast(&self) -> bool {
        (self.bits & Gpio44::SLEWFAST_MASK) != 0
    }

    /// Set **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn with_iso(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio44::ISO_MASK
            } else {
                self.bits & !Gpio44::ISO_MASK
            }
        }
    }

    /// Set **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn with_od(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio44::OD_MASK
            } else {
                self.bits & !Gpio44::OD_MASK
            }
        }
    }

    /// Set **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn with_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio44::IE_MASK
            } else {
                self.bits & !Gpio44::IE_MASK
            }
        }
    }

    /// Set **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn with_drive(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Gpio44::DRIVE_MASK) | ((value << Gpio44::DRIVE_OFFSET) & Gpio44::DRIVE_MASK)
        }
    }

    /// Set **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn with_pue(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio44::PUE_MASK
            } else {
                self.bits & !Gpio44::PUE_MASK
            }
        }
    }

    /// Set **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn with_pde(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio44::PDE_MASK
            } else {
                self.bits & !Gpio44::PDE_MASK
            }
        }
    }

    /// Set **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn with_schmitt(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio44::SCHMITT_MASK
            } else {
                self.bits & !Gpio44::SCHMITT_MASK
            }
        }
    }

    /// Set **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn with_slewfast(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio44::SLEWFAST_MASK
            } else {
                self.bits & !Gpio44::SLEWFAST_MASK
            }
        }
    }

}

impl Default for Gpio44Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Gpio44 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Gpio44Val {
        Gpio44Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Gpio44Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Gpio44Val) -> Gpio44Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// GPIO45 Register
///
/// Address offset: 0x00b8
/// Reset value: 0x00000116
///
/// ## Fields
///
/// - **ISO** `[8]` (RW)
///
///   Pad isolation control. Remove this once the pad is configured by software.
///
/// - **OD** `[7]` (RW)
///
///   Output disable. Has priority over output enable from peripherals
///
/// - **IE** `[6]` (RW)
///
///   Input enable
///
/// - **DRIVE** `[5:4]` (RW)
///
///   Drive strength.
///
/// - **PUE** `[3]` (RW)
///
///   Pull up enable
///
/// - **PDE** `[2]` (RW)
///
///   Pull down enable
///
/// - **SCHMITT** `[1]` (RW)
///
///   Enable schmitt trigger
///
/// - **SLEWFAST** `[0]` (RW)
///
///   Slew rate control. 1 = Fast, 0 = Slow
#[repr(transparent)]
pub struct Gpio45 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Gpio45 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Gpio45 {
    // ======== Field Constants ========

    /// **ISO**
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ISO_OFFSET: u32 = 8;
    pub const ISO_MASK: u32 = 0x00000100;
    pub const ISO: u32 = 1 << 8;

    /// **OD**
    ///
    /// Output disable. Has priority over output enable from peripherals
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const OD_OFFSET: u32 = 7;
    pub const OD_MASK: u32 = 0x00000080;
    pub const OD: u32 = 1 << 7;

    /// **IE**
    ///
    /// Input enable
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const IE_OFFSET: u32 = 6;
    pub const IE_MASK: u32 = 0x00000040;
    pub const IE: u32 = 1 << 6;

    /// **DRIVE**
    ///
    /// Drive strength.
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const DRIVE_OFFSET: u32 = 4;
    pub const DRIVE_MASK: u32 = 0x00000030;
    pub const DRIVE_2M_A: u32 = 0 << 4;
    pub const DRIVE_2M_A_VAL: u32 = 0;
    pub const DRIVE_4M_A: u32 = 1 << 4;
    pub const DRIVE_4M_A_VAL: u32 = 1;
    pub const DRIVE_8M_A: u32 = 2 << 4;
    pub const DRIVE_8M_A_VAL: u32 = 2;
    pub const DRIVE_12M_A: u32 = 3 << 4;
    pub const DRIVE_12M_A_VAL: u32 = 3;

    /// **PUE**
    ///
    /// Pull up enable
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const PUE_OFFSET: u32 = 3;
    pub const PUE_MASK: u32 = 0x00000008;
    pub const PUE: u32 = 1 << 3;

    /// **PDE**
    ///
    /// Pull down enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const PDE_OFFSET: u32 = 2;
    pub const PDE_MASK: u32 = 0x00000004;
    pub const PDE: u32 = 1 << 2;

    /// **SCHMITT**
    ///
    /// Enable schmitt trigger
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SCHMITT_OFFSET: u32 = 1;
    pub const SCHMITT_MASK: u32 = 0x00000002;
    pub const SCHMITT: u32 = 1 << 1;

    /// **SLEWFAST**
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SLEWFAST_OFFSET: u32 = 0;
    pub const SLEWFAST_MASK: u32 = 0x00000001;
    pub const SLEWFAST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ISO** `[8]` (RW)
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub fn iso(&self) -> bool {
        (self.value.read_bits() & Self::ISO_MASK) != 0
    }

    /// Set ISO bit atomically (RW)
    #[inline(always)]
    pub fn set_iso(&self) {
        self.set_bits(Self::ISO_MASK);
    }

    /// Clear ISO bit atomically (RW)
    #[inline(always)]
    pub fn clr_iso(&self) {
        self.clr_bits(Self::ISO_MASK);
    }

    /// **OD** `[7]` (RW)
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub fn od(&self) -> bool {
        (self.value.read_bits() & Self::OD_MASK) != 0
    }

    /// Set OD bit atomically (RW)
    #[inline(always)]
    pub fn set_od(&self) {
        self.set_bits(Self::OD_MASK);
    }

    /// Clear OD bit atomically (RW)
    #[inline(always)]
    pub fn clr_od(&self) {
        self.clr_bits(Self::OD_MASK);
    }

    /// **IE** `[6]` (RW)
    ///
    /// Input enable
    #[inline(always)]
    pub fn ie(&self) -> bool {
        (self.value.read_bits() & Self::IE_MASK) != 0
    }

    /// Set IE bit atomically (RW)
    #[inline(always)]
    pub fn set_ie(&self) {
        self.set_bits(Self::IE_MASK);
    }

    /// Clear IE bit atomically (RW)
    #[inline(always)]
    pub fn clr_ie(&self) {
        self.clr_bits(Self::IE_MASK);
    }

    /// **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn drive(&self) -> u32 {
        (self.value.read_bits() & Self::DRIVE_MASK) >> Self::DRIVE_OFFSET
    }

    /// Write **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn write_drive(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DRIVE_MASK) | ((value << Self::DRIVE_OFFSET) & Self::DRIVE_MASK));
    }

    /// **PUE** `[3]` (RW)
    ///
    /// Pull up enable
    #[inline(always)]
    pub fn pue(&self) -> bool {
        (self.value.read_bits() & Self::PUE_MASK) != 0
    }

    /// Set PUE bit atomically (RW)
    #[inline(always)]
    pub fn set_pue(&self) {
        self.set_bits(Self::PUE_MASK);
    }

    /// Clear PUE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pue(&self) {
        self.clr_bits(Self::PUE_MASK);
    }

    /// **PDE** `[2]` (RW)
    ///
    /// Pull down enable
    #[inline(always)]
    pub fn pde(&self) -> bool {
        (self.value.read_bits() & Self::PDE_MASK) != 0
    }

    /// Set PDE bit atomically (RW)
    #[inline(always)]
    pub fn set_pde(&self) {
        self.set_bits(Self::PDE_MASK);
    }

    /// Clear PDE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pde(&self) {
        self.clr_bits(Self::PDE_MASK);
    }

    /// **SCHMITT** `[1]` (RW)
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub fn schmitt(&self) -> bool {
        (self.value.read_bits() & Self::SCHMITT_MASK) != 0
    }

    /// Set SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn set_schmitt(&self) {
        self.set_bits(Self::SCHMITT_MASK);
    }

    /// Clear SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn clr_schmitt(&self) {
        self.clr_bits(Self::SCHMITT_MASK);
    }

    /// **SLEWFAST** `[0]` (RW)
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub fn slewfast(&self) -> bool {
        (self.value.read_bits() & Self::SLEWFAST_MASK) != 0
    }

    /// Set SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn set_slewfast(&self) {
        self.set_bits(Self::SLEWFAST_MASK);
    }

    /// Clear SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn clr_slewfast(&self) {
        self.clr_bits(Self::SLEWFAST_MASK);
    }

}

/// GPIO45 value type for batch read/modify/write operations
///
/// Use `gpio45.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpio45.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Gpio45Val {
    bits: u32,
}

impl Gpio45Val {
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
        Self { bits: 0x00000116 }
    }

    /// Read **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn iso(&self) -> bool {
        (self.bits & Gpio45::ISO_MASK) != 0
    }

    /// Read **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn od(&self) -> bool {
        (self.bits & Gpio45::OD_MASK) != 0
    }

    /// Read **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        (self.bits & Gpio45::IE_MASK) != 0
    }

    /// Read **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn drive(&self) -> u32 {
        (self.bits & Gpio45::DRIVE_MASK) >> Gpio45::DRIVE_OFFSET
    }

    /// Read **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn pue(&self) -> bool {
        (self.bits & Gpio45::PUE_MASK) != 0
    }

    /// Read **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn pde(&self) -> bool {
        (self.bits & Gpio45::PDE_MASK) != 0
    }

    /// Read **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn schmitt(&self) -> bool {
        (self.bits & Gpio45::SCHMITT_MASK) != 0
    }

    /// Read **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn slewfast(&self) -> bool {
        (self.bits & Gpio45::SLEWFAST_MASK) != 0
    }

    /// Set **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn with_iso(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio45::ISO_MASK
            } else {
                self.bits & !Gpio45::ISO_MASK
            }
        }
    }

    /// Set **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn with_od(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio45::OD_MASK
            } else {
                self.bits & !Gpio45::OD_MASK
            }
        }
    }

    /// Set **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn with_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio45::IE_MASK
            } else {
                self.bits & !Gpio45::IE_MASK
            }
        }
    }

    /// Set **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn with_drive(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Gpio45::DRIVE_MASK) | ((value << Gpio45::DRIVE_OFFSET) & Gpio45::DRIVE_MASK)
        }
    }

    /// Set **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn with_pue(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio45::PUE_MASK
            } else {
                self.bits & !Gpio45::PUE_MASK
            }
        }
    }

    /// Set **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn with_pde(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio45::PDE_MASK
            } else {
                self.bits & !Gpio45::PDE_MASK
            }
        }
    }

    /// Set **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn with_schmitt(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio45::SCHMITT_MASK
            } else {
                self.bits & !Gpio45::SCHMITT_MASK
            }
        }
    }

    /// Set **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn with_slewfast(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio45::SLEWFAST_MASK
            } else {
                self.bits & !Gpio45::SLEWFAST_MASK
            }
        }
    }

}

impl Default for Gpio45Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Gpio45 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Gpio45Val {
        Gpio45Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Gpio45Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Gpio45Val) -> Gpio45Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// GPIO46 Register
///
/// Address offset: 0x00bc
/// Reset value: 0x00000116
///
/// ## Fields
///
/// - **ISO** `[8]` (RW)
///
///   Pad isolation control. Remove this once the pad is configured by software.
///
/// - **OD** `[7]` (RW)
///
///   Output disable. Has priority over output enable from peripherals
///
/// - **IE** `[6]` (RW)
///
///   Input enable
///
/// - **DRIVE** `[5:4]` (RW)
///
///   Drive strength.
///
/// - **PUE** `[3]` (RW)
///
///   Pull up enable
///
/// - **PDE** `[2]` (RW)
///
///   Pull down enable
///
/// - **SCHMITT** `[1]` (RW)
///
///   Enable schmitt trigger
///
/// - **SLEWFAST** `[0]` (RW)
///
///   Slew rate control. 1 = Fast, 0 = Slow
#[repr(transparent)]
pub struct Gpio46 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Gpio46 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Gpio46 {
    // ======== Field Constants ========

    /// **ISO**
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ISO_OFFSET: u32 = 8;
    pub const ISO_MASK: u32 = 0x00000100;
    pub const ISO: u32 = 1 << 8;

    /// **OD**
    ///
    /// Output disable. Has priority over output enable from peripherals
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const OD_OFFSET: u32 = 7;
    pub const OD_MASK: u32 = 0x00000080;
    pub const OD: u32 = 1 << 7;

    /// **IE**
    ///
    /// Input enable
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const IE_OFFSET: u32 = 6;
    pub const IE_MASK: u32 = 0x00000040;
    pub const IE: u32 = 1 << 6;

    /// **DRIVE**
    ///
    /// Drive strength.
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const DRIVE_OFFSET: u32 = 4;
    pub const DRIVE_MASK: u32 = 0x00000030;
    pub const DRIVE_2M_A: u32 = 0 << 4;
    pub const DRIVE_2M_A_VAL: u32 = 0;
    pub const DRIVE_4M_A: u32 = 1 << 4;
    pub const DRIVE_4M_A_VAL: u32 = 1;
    pub const DRIVE_8M_A: u32 = 2 << 4;
    pub const DRIVE_8M_A_VAL: u32 = 2;
    pub const DRIVE_12M_A: u32 = 3 << 4;
    pub const DRIVE_12M_A_VAL: u32 = 3;

    /// **PUE**
    ///
    /// Pull up enable
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const PUE_OFFSET: u32 = 3;
    pub const PUE_MASK: u32 = 0x00000008;
    pub const PUE: u32 = 1 << 3;

    /// **PDE**
    ///
    /// Pull down enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const PDE_OFFSET: u32 = 2;
    pub const PDE_MASK: u32 = 0x00000004;
    pub const PDE: u32 = 1 << 2;

    /// **SCHMITT**
    ///
    /// Enable schmitt trigger
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SCHMITT_OFFSET: u32 = 1;
    pub const SCHMITT_MASK: u32 = 0x00000002;
    pub const SCHMITT: u32 = 1 << 1;

    /// **SLEWFAST**
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SLEWFAST_OFFSET: u32 = 0;
    pub const SLEWFAST_MASK: u32 = 0x00000001;
    pub const SLEWFAST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ISO** `[8]` (RW)
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub fn iso(&self) -> bool {
        (self.value.read_bits() & Self::ISO_MASK) != 0
    }

    /// Set ISO bit atomically (RW)
    #[inline(always)]
    pub fn set_iso(&self) {
        self.set_bits(Self::ISO_MASK);
    }

    /// Clear ISO bit atomically (RW)
    #[inline(always)]
    pub fn clr_iso(&self) {
        self.clr_bits(Self::ISO_MASK);
    }

    /// **OD** `[7]` (RW)
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub fn od(&self) -> bool {
        (self.value.read_bits() & Self::OD_MASK) != 0
    }

    /// Set OD bit atomically (RW)
    #[inline(always)]
    pub fn set_od(&self) {
        self.set_bits(Self::OD_MASK);
    }

    /// Clear OD bit atomically (RW)
    #[inline(always)]
    pub fn clr_od(&self) {
        self.clr_bits(Self::OD_MASK);
    }

    /// **IE** `[6]` (RW)
    ///
    /// Input enable
    #[inline(always)]
    pub fn ie(&self) -> bool {
        (self.value.read_bits() & Self::IE_MASK) != 0
    }

    /// Set IE bit atomically (RW)
    #[inline(always)]
    pub fn set_ie(&self) {
        self.set_bits(Self::IE_MASK);
    }

    /// Clear IE bit atomically (RW)
    #[inline(always)]
    pub fn clr_ie(&self) {
        self.clr_bits(Self::IE_MASK);
    }

    /// **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn drive(&self) -> u32 {
        (self.value.read_bits() & Self::DRIVE_MASK) >> Self::DRIVE_OFFSET
    }

    /// Write **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn write_drive(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DRIVE_MASK) | ((value << Self::DRIVE_OFFSET) & Self::DRIVE_MASK));
    }

    /// **PUE** `[3]` (RW)
    ///
    /// Pull up enable
    #[inline(always)]
    pub fn pue(&self) -> bool {
        (self.value.read_bits() & Self::PUE_MASK) != 0
    }

    /// Set PUE bit atomically (RW)
    #[inline(always)]
    pub fn set_pue(&self) {
        self.set_bits(Self::PUE_MASK);
    }

    /// Clear PUE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pue(&self) {
        self.clr_bits(Self::PUE_MASK);
    }

    /// **PDE** `[2]` (RW)
    ///
    /// Pull down enable
    #[inline(always)]
    pub fn pde(&self) -> bool {
        (self.value.read_bits() & Self::PDE_MASK) != 0
    }

    /// Set PDE bit atomically (RW)
    #[inline(always)]
    pub fn set_pde(&self) {
        self.set_bits(Self::PDE_MASK);
    }

    /// Clear PDE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pde(&self) {
        self.clr_bits(Self::PDE_MASK);
    }

    /// **SCHMITT** `[1]` (RW)
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub fn schmitt(&self) -> bool {
        (self.value.read_bits() & Self::SCHMITT_MASK) != 0
    }

    /// Set SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn set_schmitt(&self) {
        self.set_bits(Self::SCHMITT_MASK);
    }

    /// Clear SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn clr_schmitt(&self) {
        self.clr_bits(Self::SCHMITT_MASK);
    }

    /// **SLEWFAST** `[0]` (RW)
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub fn slewfast(&self) -> bool {
        (self.value.read_bits() & Self::SLEWFAST_MASK) != 0
    }

    /// Set SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn set_slewfast(&self) {
        self.set_bits(Self::SLEWFAST_MASK);
    }

    /// Clear SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn clr_slewfast(&self) {
        self.clr_bits(Self::SLEWFAST_MASK);
    }

}

/// GPIO46 value type for batch read/modify/write operations
///
/// Use `gpio46.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpio46.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Gpio46Val {
    bits: u32,
}

impl Gpio46Val {
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
        Self { bits: 0x00000116 }
    }

    /// Read **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn iso(&self) -> bool {
        (self.bits & Gpio46::ISO_MASK) != 0
    }

    /// Read **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn od(&self) -> bool {
        (self.bits & Gpio46::OD_MASK) != 0
    }

    /// Read **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        (self.bits & Gpio46::IE_MASK) != 0
    }

    /// Read **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn drive(&self) -> u32 {
        (self.bits & Gpio46::DRIVE_MASK) >> Gpio46::DRIVE_OFFSET
    }

    /// Read **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn pue(&self) -> bool {
        (self.bits & Gpio46::PUE_MASK) != 0
    }

    /// Read **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn pde(&self) -> bool {
        (self.bits & Gpio46::PDE_MASK) != 0
    }

    /// Read **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn schmitt(&self) -> bool {
        (self.bits & Gpio46::SCHMITT_MASK) != 0
    }

    /// Read **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn slewfast(&self) -> bool {
        (self.bits & Gpio46::SLEWFAST_MASK) != 0
    }

    /// Set **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn with_iso(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio46::ISO_MASK
            } else {
                self.bits & !Gpio46::ISO_MASK
            }
        }
    }

    /// Set **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn with_od(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio46::OD_MASK
            } else {
                self.bits & !Gpio46::OD_MASK
            }
        }
    }

    /// Set **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn with_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio46::IE_MASK
            } else {
                self.bits & !Gpio46::IE_MASK
            }
        }
    }

    /// Set **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn with_drive(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Gpio46::DRIVE_MASK) | ((value << Gpio46::DRIVE_OFFSET) & Gpio46::DRIVE_MASK)
        }
    }

    /// Set **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn with_pue(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio46::PUE_MASK
            } else {
                self.bits & !Gpio46::PUE_MASK
            }
        }
    }

    /// Set **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn with_pde(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio46::PDE_MASK
            } else {
                self.bits & !Gpio46::PDE_MASK
            }
        }
    }

    /// Set **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn with_schmitt(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio46::SCHMITT_MASK
            } else {
                self.bits & !Gpio46::SCHMITT_MASK
            }
        }
    }

    /// Set **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn with_slewfast(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio46::SLEWFAST_MASK
            } else {
                self.bits & !Gpio46::SLEWFAST_MASK
            }
        }
    }

}

impl Default for Gpio46Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Gpio46 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Gpio46Val {
        Gpio46Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Gpio46Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Gpio46Val) -> Gpio46Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// GPIO47 Register
///
/// Address offset: 0x00c0
/// Reset value: 0x00000116
///
/// ## Fields
///
/// - **ISO** `[8]` (RW)
///
///   Pad isolation control. Remove this once the pad is configured by software.
///
/// - **OD** `[7]` (RW)
///
///   Output disable. Has priority over output enable from peripherals
///
/// - **IE** `[6]` (RW)
///
///   Input enable
///
/// - **DRIVE** `[5:4]` (RW)
///
///   Drive strength.
///
/// - **PUE** `[3]` (RW)
///
///   Pull up enable
///
/// - **PDE** `[2]` (RW)
///
///   Pull down enable
///
/// - **SCHMITT** `[1]` (RW)
///
///   Enable schmitt trigger
///
/// - **SLEWFAST** `[0]` (RW)
///
///   Slew rate control. 1 = Fast, 0 = Slow
#[repr(transparent)]
pub struct Gpio47 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Gpio47 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Gpio47 {
    // ======== Field Constants ========

    /// **ISO**
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ISO_OFFSET: u32 = 8;
    pub const ISO_MASK: u32 = 0x00000100;
    pub const ISO: u32 = 1 << 8;

    /// **OD**
    ///
    /// Output disable. Has priority over output enable from peripherals
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const OD_OFFSET: u32 = 7;
    pub const OD_MASK: u32 = 0x00000080;
    pub const OD: u32 = 1 << 7;

    /// **IE**
    ///
    /// Input enable
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const IE_OFFSET: u32 = 6;
    pub const IE_MASK: u32 = 0x00000040;
    pub const IE: u32 = 1 << 6;

    /// **DRIVE**
    ///
    /// Drive strength.
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const DRIVE_OFFSET: u32 = 4;
    pub const DRIVE_MASK: u32 = 0x00000030;
    pub const DRIVE_2M_A: u32 = 0 << 4;
    pub const DRIVE_2M_A_VAL: u32 = 0;
    pub const DRIVE_4M_A: u32 = 1 << 4;
    pub const DRIVE_4M_A_VAL: u32 = 1;
    pub const DRIVE_8M_A: u32 = 2 << 4;
    pub const DRIVE_8M_A_VAL: u32 = 2;
    pub const DRIVE_12M_A: u32 = 3 << 4;
    pub const DRIVE_12M_A_VAL: u32 = 3;

    /// **PUE**
    ///
    /// Pull up enable
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const PUE_OFFSET: u32 = 3;
    pub const PUE_MASK: u32 = 0x00000008;
    pub const PUE: u32 = 1 << 3;

    /// **PDE**
    ///
    /// Pull down enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const PDE_OFFSET: u32 = 2;
    pub const PDE_MASK: u32 = 0x00000004;
    pub const PDE: u32 = 1 << 2;

    /// **SCHMITT**
    ///
    /// Enable schmitt trigger
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SCHMITT_OFFSET: u32 = 1;
    pub const SCHMITT_MASK: u32 = 0x00000002;
    pub const SCHMITT: u32 = 1 << 1;

    /// **SLEWFAST**
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SLEWFAST_OFFSET: u32 = 0;
    pub const SLEWFAST_MASK: u32 = 0x00000001;
    pub const SLEWFAST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ISO** `[8]` (RW)
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub fn iso(&self) -> bool {
        (self.value.read_bits() & Self::ISO_MASK) != 0
    }

    /// Set ISO bit atomically (RW)
    #[inline(always)]
    pub fn set_iso(&self) {
        self.set_bits(Self::ISO_MASK);
    }

    /// Clear ISO bit atomically (RW)
    #[inline(always)]
    pub fn clr_iso(&self) {
        self.clr_bits(Self::ISO_MASK);
    }

    /// **OD** `[7]` (RW)
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub fn od(&self) -> bool {
        (self.value.read_bits() & Self::OD_MASK) != 0
    }

    /// Set OD bit atomically (RW)
    #[inline(always)]
    pub fn set_od(&self) {
        self.set_bits(Self::OD_MASK);
    }

    /// Clear OD bit atomically (RW)
    #[inline(always)]
    pub fn clr_od(&self) {
        self.clr_bits(Self::OD_MASK);
    }

    /// **IE** `[6]` (RW)
    ///
    /// Input enable
    #[inline(always)]
    pub fn ie(&self) -> bool {
        (self.value.read_bits() & Self::IE_MASK) != 0
    }

    /// Set IE bit atomically (RW)
    #[inline(always)]
    pub fn set_ie(&self) {
        self.set_bits(Self::IE_MASK);
    }

    /// Clear IE bit atomically (RW)
    #[inline(always)]
    pub fn clr_ie(&self) {
        self.clr_bits(Self::IE_MASK);
    }

    /// **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn drive(&self) -> u32 {
        (self.value.read_bits() & Self::DRIVE_MASK) >> Self::DRIVE_OFFSET
    }

    /// Write **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn write_drive(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DRIVE_MASK) | ((value << Self::DRIVE_OFFSET) & Self::DRIVE_MASK));
    }

    /// **PUE** `[3]` (RW)
    ///
    /// Pull up enable
    #[inline(always)]
    pub fn pue(&self) -> bool {
        (self.value.read_bits() & Self::PUE_MASK) != 0
    }

    /// Set PUE bit atomically (RW)
    #[inline(always)]
    pub fn set_pue(&self) {
        self.set_bits(Self::PUE_MASK);
    }

    /// Clear PUE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pue(&self) {
        self.clr_bits(Self::PUE_MASK);
    }

    /// **PDE** `[2]` (RW)
    ///
    /// Pull down enable
    #[inline(always)]
    pub fn pde(&self) -> bool {
        (self.value.read_bits() & Self::PDE_MASK) != 0
    }

    /// Set PDE bit atomically (RW)
    #[inline(always)]
    pub fn set_pde(&self) {
        self.set_bits(Self::PDE_MASK);
    }

    /// Clear PDE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pde(&self) {
        self.clr_bits(Self::PDE_MASK);
    }

    /// **SCHMITT** `[1]` (RW)
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub fn schmitt(&self) -> bool {
        (self.value.read_bits() & Self::SCHMITT_MASK) != 0
    }

    /// Set SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn set_schmitt(&self) {
        self.set_bits(Self::SCHMITT_MASK);
    }

    /// Clear SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn clr_schmitt(&self) {
        self.clr_bits(Self::SCHMITT_MASK);
    }

    /// **SLEWFAST** `[0]` (RW)
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub fn slewfast(&self) -> bool {
        (self.value.read_bits() & Self::SLEWFAST_MASK) != 0
    }

    /// Set SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn set_slewfast(&self) {
        self.set_bits(Self::SLEWFAST_MASK);
    }

    /// Clear SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn clr_slewfast(&self) {
        self.clr_bits(Self::SLEWFAST_MASK);
    }

}

/// GPIO47 value type for batch read/modify/write operations
///
/// Use `gpio47.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpio47.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Gpio47Val {
    bits: u32,
}

impl Gpio47Val {
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
        Self { bits: 0x00000116 }
    }

    /// Read **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn iso(&self) -> bool {
        (self.bits & Gpio47::ISO_MASK) != 0
    }

    /// Read **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn od(&self) -> bool {
        (self.bits & Gpio47::OD_MASK) != 0
    }

    /// Read **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        (self.bits & Gpio47::IE_MASK) != 0
    }

    /// Read **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn drive(&self) -> u32 {
        (self.bits & Gpio47::DRIVE_MASK) >> Gpio47::DRIVE_OFFSET
    }

    /// Read **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn pue(&self) -> bool {
        (self.bits & Gpio47::PUE_MASK) != 0
    }

    /// Read **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn pde(&self) -> bool {
        (self.bits & Gpio47::PDE_MASK) != 0
    }

    /// Read **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn schmitt(&self) -> bool {
        (self.bits & Gpio47::SCHMITT_MASK) != 0
    }

    /// Read **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn slewfast(&self) -> bool {
        (self.bits & Gpio47::SLEWFAST_MASK) != 0
    }

    /// Set **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn with_iso(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio47::ISO_MASK
            } else {
                self.bits & !Gpio47::ISO_MASK
            }
        }
    }

    /// Set **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn with_od(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio47::OD_MASK
            } else {
                self.bits & !Gpio47::OD_MASK
            }
        }
    }

    /// Set **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn with_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio47::IE_MASK
            } else {
                self.bits & !Gpio47::IE_MASK
            }
        }
    }

    /// Set **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn with_drive(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Gpio47::DRIVE_MASK) | ((value << Gpio47::DRIVE_OFFSET) & Gpio47::DRIVE_MASK)
        }
    }

    /// Set **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn with_pue(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio47::PUE_MASK
            } else {
                self.bits & !Gpio47::PUE_MASK
            }
        }
    }

    /// Set **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn with_pde(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio47::PDE_MASK
            } else {
                self.bits & !Gpio47::PDE_MASK
            }
        }
    }

    /// Set **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn with_schmitt(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio47::SCHMITT_MASK
            } else {
                self.bits & !Gpio47::SCHMITT_MASK
            }
        }
    }

    /// Set **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn with_slewfast(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpio47::SLEWFAST_MASK
            } else {
                self.bits & !Gpio47::SLEWFAST_MASK
            }
        }
    }

}

impl Default for Gpio47Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Gpio47 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Gpio47Val {
        Gpio47Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Gpio47Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Gpio47Val) -> Gpio47Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SWCLK Register
///
/// Address offset: 0x00c4
/// Reset value: 0x0000005a
///
/// ## Fields
///
/// - **ISO** `[8]` (RW)
///
///   Pad isolation control. Remove this once the pad is configured by software.
///
/// - **OD** `[7]` (RW)
///
///   Output disable. Has priority over output enable from peripherals
///
/// - **IE** `[6]` (RW)
///
///   Input enable
///
/// - **DRIVE** `[5:4]` (RW)
///
///   Drive strength.
///
/// - **PUE** `[3]` (RW)
///
///   Pull up enable
///
/// - **PDE** `[2]` (RW)
///
///   Pull down enable
///
/// - **SCHMITT** `[1]` (RW)
///
///   Enable schmitt trigger
///
/// - **SLEWFAST** `[0]` (RW)
///
///   Slew rate control. 1 = Fast, 0 = Slow
#[repr(transparent)]
pub struct Swclk {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Swclk {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Swclk {
    // ======== Field Constants ========

    /// **ISO**
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ISO_OFFSET: u32 = 8;
    pub const ISO_MASK: u32 = 0x00000100;
    pub const ISO: u32 = 1 << 8;

    /// **OD**
    ///
    /// Output disable. Has priority over output enable from peripherals
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const OD_OFFSET: u32 = 7;
    pub const OD_MASK: u32 = 0x00000080;
    pub const OD: u32 = 1 << 7;

    /// **IE**
    ///
    /// Input enable
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const IE_OFFSET: u32 = 6;
    pub const IE_MASK: u32 = 0x00000040;
    pub const IE: u32 = 1 << 6;

    /// **DRIVE**
    ///
    /// Drive strength.
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const DRIVE_OFFSET: u32 = 4;
    pub const DRIVE_MASK: u32 = 0x00000030;
    pub const DRIVE_2M_A: u32 = 0 << 4;
    pub const DRIVE_2M_A_VAL: u32 = 0;
    pub const DRIVE_4M_A: u32 = 1 << 4;
    pub const DRIVE_4M_A_VAL: u32 = 1;
    pub const DRIVE_8M_A: u32 = 2 << 4;
    pub const DRIVE_8M_A_VAL: u32 = 2;
    pub const DRIVE_12M_A: u32 = 3 << 4;
    pub const DRIVE_12M_A_VAL: u32 = 3;

    /// **PUE**
    ///
    /// Pull up enable
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const PUE_OFFSET: u32 = 3;
    pub const PUE_MASK: u32 = 0x00000008;
    pub const PUE: u32 = 1 << 3;

    /// **PDE**
    ///
    /// Pull down enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const PDE_OFFSET: u32 = 2;
    pub const PDE_MASK: u32 = 0x00000004;
    pub const PDE: u32 = 1 << 2;

    /// **SCHMITT**
    ///
    /// Enable schmitt trigger
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SCHMITT_OFFSET: u32 = 1;
    pub const SCHMITT_MASK: u32 = 0x00000002;
    pub const SCHMITT: u32 = 1 << 1;

    /// **SLEWFAST**
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SLEWFAST_OFFSET: u32 = 0;
    pub const SLEWFAST_MASK: u32 = 0x00000001;
    pub const SLEWFAST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ISO** `[8]` (RW)
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub fn iso(&self) -> bool {
        (self.value.read_bits() & Self::ISO_MASK) != 0
    }

    /// Set ISO bit atomically (RW)
    #[inline(always)]
    pub fn set_iso(&self) {
        self.set_bits(Self::ISO_MASK);
    }

    /// Clear ISO bit atomically (RW)
    #[inline(always)]
    pub fn clr_iso(&self) {
        self.clr_bits(Self::ISO_MASK);
    }

    /// **OD** `[7]` (RW)
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub fn od(&self) -> bool {
        (self.value.read_bits() & Self::OD_MASK) != 0
    }

    /// Set OD bit atomically (RW)
    #[inline(always)]
    pub fn set_od(&self) {
        self.set_bits(Self::OD_MASK);
    }

    /// Clear OD bit atomically (RW)
    #[inline(always)]
    pub fn clr_od(&self) {
        self.clr_bits(Self::OD_MASK);
    }

    /// **IE** `[6]` (RW)
    ///
    /// Input enable
    #[inline(always)]
    pub fn ie(&self) -> bool {
        (self.value.read_bits() & Self::IE_MASK) != 0
    }

    /// Set IE bit atomically (RW)
    #[inline(always)]
    pub fn set_ie(&self) {
        self.set_bits(Self::IE_MASK);
    }

    /// Clear IE bit atomically (RW)
    #[inline(always)]
    pub fn clr_ie(&self) {
        self.clr_bits(Self::IE_MASK);
    }

    /// **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn drive(&self) -> u32 {
        (self.value.read_bits() & Self::DRIVE_MASK) >> Self::DRIVE_OFFSET
    }

    /// Write **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn write_drive(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DRIVE_MASK) | ((value << Self::DRIVE_OFFSET) & Self::DRIVE_MASK));
    }

    /// **PUE** `[3]` (RW)
    ///
    /// Pull up enable
    #[inline(always)]
    pub fn pue(&self) -> bool {
        (self.value.read_bits() & Self::PUE_MASK) != 0
    }

    /// Set PUE bit atomically (RW)
    #[inline(always)]
    pub fn set_pue(&self) {
        self.set_bits(Self::PUE_MASK);
    }

    /// Clear PUE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pue(&self) {
        self.clr_bits(Self::PUE_MASK);
    }

    /// **PDE** `[2]` (RW)
    ///
    /// Pull down enable
    #[inline(always)]
    pub fn pde(&self) -> bool {
        (self.value.read_bits() & Self::PDE_MASK) != 0
    }

    /// Set PDE bit atomically (RW)
    #[inline(always)]
    pub fn set_pde(&self) {
        self.set_bits(Self::PDE_MASK);
    }

    /// Clear PDE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pde(&self) {
        self.clr_bits(Self::PDE_MASK);
    }

    /// **SCHMITT** `[1]` (RW)
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub fn schmitt(&self) -> bool {
        (self.value.read_bits() & Self::SCHMITT_MASK) != 0
    }

    /// Set SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn set_schmitt(&self) {
        self.set_bits(Self::SCHMITT_MASK);
    }

    /// Clear SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn clr_schmitt(&self) {
        self.clr_bits(Self::SCHMITT_MASK);
    }

    /// **SLEWFAST** `[0]` (RW)
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub fn slewfast(&self) -> bool {
        (self.value.read_bits() & Self::SLEWFAST_MASK) != 0
    }

    /// Set SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn set_slewfast(&self) {
        self.set_bits(Self::SLEWFAST_MASK);
    }

    /// Clear SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn clr_slewfast(&self) {
        self.clr_bits(Self::SLEWFAST_MASK);
    }

}

/// SWCLK value type for batch read/modify/write operations
///
/// Use `swclk.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `swclk.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SwclkVal {
    bits: u32,
}

impl SwclkVal {
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
        Self { bits: 0x0000005a }
    }

    /// Read **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn iso(&self) -> bool {
        (self.bits & Swclk::ISO_MASK) != 0
    }

    /// Read **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn od(&self) -> bool {
        (self.bits & Swclk::OD_MASK) != 0
    }

    /// Read **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        (self.bits & Swclk::IE_MASK) != 0
    }

    /// Read **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn drive(&self) -> u32 {
        (self.bits & Swclk::DRIVE_MASK) >> Swclk::DRIVE_OFFSET
    }

    /// Read **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn pue(&self) -> bool {
        (self.bits & Swclk::PUE_MASK) != 0
    }

    /// Read **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn pde(&self) -> bool {
        (self.bits & Swclk::PDE_MASK) != 0
    }

    /// Read **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn schmitt(&self) -> bool {
        (self.bits & Swclk::SCHMITT_MASK) != 0
    }

    /// Read **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn slewfast(&self) -> bool {
        (self.bits & Swclk::SLEWFAST_MASK) != 0
    }

    /// Set **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn with_iso(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Swclk::ISO_MASK
            } else {
                self.bits & !Swclk::ISO_MASK
            }
        }
    }

    /// Set **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn with_od(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Swclk::OD_MASK
            } else {
                self.bits & !Swclk::OD_MASK
            }
        }
    }

    /// Set **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn with_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Swclk::IE_MASK
            } else {
                self.bits & !Swclk::IE_MASK
            }
        }
    }

    /// Set **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn with_drive(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Swclk::DRIVE_MASK) | ((value << Swclk::DRIVE_OFFSET) & Swclk::DRIVE_MASK)
        }
    }

    /// Set **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn with_pue(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Swclk::PUE_MASK
            } else {
                self.bits & !Swclk::PUE_MASK
            }
        }
    }

    /// Set **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn with_pde(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Swclk::PDE_MASK
            } else {
                self.bits & !Swclk::PDE_MASK
            }
        }
    }

    /// Set **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn with_schmitt(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Swclk::SCHMITT_MASK
            } else {
                self.bits & !Swclk::SCHMITT_MASK
            }
        }
    }

    /// Set **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn with_slewfast(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Swclk::SLEWFAST_MASK
            } else {
                self.bits & !Swclk::SLEWFAST_MASK
            }
        }
    }

}

impl Default for SwclkVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Swclk {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SwclkVal {
        SwclkVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SwclkVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SwclkVal) -> SwclkVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SWD Register
///
/// Address offset: 0x00c8
/// Reset value: 0x0000005a
///
/// ## Fields
///
/// - **ISO** `[8]` (RW)
///
///   Pad isolation control. Remove this once the pad is configured by software.
///
/// - **OD** `[7]` (RW)
///
///   Output disable. Has priority over output enable from peripherals
///
/// - **IE** `[6]` (RW)
///
///   Input enable
///
/// - **DRIVE** `[5:4]` (RW)
///
///   Drive strength.
///
/// - **PUE** `[3]` (RW)
///
///   Pull up enable
///
/// - **PDE** `[2]` (RW)
///
///   Pull down enable
///
/// - **SCHMITT** `[1]` (RW)
///
///   Enable schmitt trigger
///
/// - **SLEWFAST** `[0]` (RW)
///
///   Slew rate control. 1 = Fast, 0 = Slow
#[repr(transparent)]
pub struct Swd {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Swd {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Swd {
    // ======== Field Constants ========

    /// **ISO**
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ISO_OFFSET: u32 = 8;
    pub const ISO_MASK: u32 = 0x00000100;
    pub const ISO: u32 = 1 << 8;

    /// **OD**
    ///
    /// Output disable. Has priority over output enable from peripherals
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const OD_OFFSET: u32 = 7;
    pub const OD_MASK: u32 = 0x00000080;
    pub const OD: u32 = 1 << 7;

    /// **IE**
    ///
    /// Input enable
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const IE_OFFSET: u32 = 6;
    pub const IE_MASK: u32 = 0x00000040;
    pub const IE: u32 = 1 << 6;

    /// **DRIVE**
    ///
    /// Drive strength.
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const DRIVE_OFFSET: u32 = 4;
    pub const DRIVE_MASK: u32 = 0x00000030;
    pub const DRIVE_2M_A: u32 = 0 << 4;
    pub const DRIVE_2M_A_VAL: u32 = 0;
    pub const DRIVE_4M_A: u32 = 1 << 4;
    pub const DRIVE_4M_A_VAL: u32 = 1;
    pub const DRIVE_8M_A: u32 = 2 << 4;
    pub const DRIVE_8M_A_VAL: u32 = 2;
    pub const DRIVE_12M_A: u32 = 3 << 4;
    pub const DRIVE_12M_A_VAL: u32 = 3;

    /// **PUE**
    ///
    /// Pull up enable
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const PUE_OFFSET: u32 = 3;
    pub const PUE_MASK: u32 = 0x00000008;
    pub const PUE: u32 = 1 << 3;

    /// **PDE**
    ///
    /// Pull down enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const PDE_OFFSET: u32 = 2;
    pub const PDE_MASK: u32 = 0x00000004;
    pub const PDE: u32 = 1 << 2;

    /// **SCHMITT**
    ///
    /// Enable schmitt trigger
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SCHMITT_OFFSET: u32 = 1;
    pub const SCHMITT_MASK: u32 = 0x00000002;
    pub const SCHMITT: u32 = 1 << 1;

    /// **SLEWFAST**
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SLEWFAST_OFFSET: u32 = 0;
    pub const SLEWFAST_MASK: u32 = 0x00000001;
    pub const SLEWFAST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ISO** `[8]` (RW)
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub fn iso(&self) -> bool {
        (self.value.read_bits() & Self::ISO_MASK) != 0
    }

    /// Set ISO bit atomically (RW)
    #[inline(always)]
    pub fn set_iso(&self) {
        self.set_bits(Self::ISO_MASK);
    }

    /// Clear ISO bit atomically (RW)
    #[inline(always)]
    pub fn clr_iso(&self) {
        self.clr_bits(Self::ISO_MASK);
    }

    /// **OD** `[7]` (RW)
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub fn od(&self) -> bool {
        (self.value.read_bits() & Self::OD_MASK) != 0
    }

    /// Set OD bit atomically (RW)
    #[inline(always)]
    pub fn set_od(&self) {
        self.set_bits(Self::OD_MASK);
    }

    /// Clear OD bit atomically (RW)
    #[inline(always)]
    pub fn clr_od(&self) {
        self.clr_bits(Self::OD_MASK);
    }

    /// **IE** `[6]` (RW)
    ///
    /// Input enable
    #[inline(always)]
    pub fn ie(&self) -> bool {
        (self.value.read_bits() & Self::IE_MASK) != 0
    }

    /// Set IE bit atomically (RW)
    #[inline(always)]
    pub fn set_ie(&self) {
        self.set_bits(Self::IE_MASK);
    }

    /// Clear IE bit atomically (RW)
    #[inline(always)]
    pub fn clr_ie(&self) {
        self.clr_bits(Self::IE_MASK);
    }

    /// **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn drive(&self) -> u32 {
        (self.value.read_bits() & Self::DRIVE_MASK) >> Self::DRIVE_OFFSET
    }

    /// Write **DRIVE** `[5:4]` (RW)
    ///
    /// Drive strength.
    #[inline(always)]
    pub fn write_drive(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DRIVE_MASK) | ((value << Self::DRIVE_OFFSET) & Self::DRIVE_MASK));
    }

    /// **PUE** `[3]` (RW)
    ///
    /// Pull up enable
    #[inline(always)]
    pub fn pue(&self) -> bool {
        (self.value.read_bits() & Self::PUE_MASK) != 0
    }

    /// Set PUE bit atomically (RW)
    #[inline(always)]
    pub fn set_pue(&self) {
        self.set_bits(Self::PUE_MASK);
    }

    /// Clear PUE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pue(&self) {
        self.clr_bits(Self::PUE_MASK);
    }

    /// **PDE** `[2]` (RW)
    ///
    /// Pull down enable
    #[inline(always)]
    pub fn pde(&self) -> bool {
        (self.value.read_bits() & Self::PDE_MASK) != 0
    }

    /// Set PDE bit atomically (RW)
    #[inline(always)]
    pub fn set_pde(&self) {
        self.set_bits(Self::PDE_MASK);
    }

    /// Clear PDE bit atomically (RW)
    #[inline(always)]
    pub fn clr_pde(&self) {
        self.clr_bits(Self::PDE_MASK);
    }

    /// **SCHMITT** `[1]` (RW)
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub fn schmitt(&self) -> bool {
        (self.value.read_bits() & Self::SCHMITT_MASK) != 0
    }

    /// Set SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn set_schmitt(&self) {
        self.set_bits(Self::SCHMITT_MASK);
    }

    /// Clear SCHMITT bit atomically (RW)
    #[inline(always)]
    pub fn clr_schmitt(&self) {
        self.clr_bits(Self::SCHMITT_MASK);
    }

    /// **SLEWFAST** `[0]` (RW)
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub fn slewfast(&self) -> bool {
        (self.value.read_bits() & Self::SLEWFAST_MASK) != 0
    }

    /// Set SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn set_slewfast(&self) {
        self.set_bits(Self::SLEWFAST_MASK);
    }

    /// Clear SLEWFAST bit atomically (RW)
    #[inline(always)]
    pub fn clr_slewfast(&self) {
        self.clr_bits(Self::SLEWFAST_MASK);
    }

}

/// SWD value type for batch read/modify/write operations
///
/// Use `swd.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `swd.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SwdVal {
    bits: u32,
}

impl SwdVal {
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
        Self { bits: 0x0000005a }
    }

    /// Read **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn iso(&self) -> bool {
        (self.bits & Swd::ISO_MASK) != 0
    }

    /// Read **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn od(&self) -> bool {
        (self.bits & Swd::OD_MASK) != 0
    }

    /// Read **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        (self.bits & Swd::IE_MASK) != 0
    }

    /// Read **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn drive(&self) -> u32 {
        (self.bits & Swd::DRIVE_MASK) >> Swd::DRIVE_OFFSET
    }

    /// Read **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn pue(&self) -> bool {
        (self.bits & Swd::PUE_MASK) != 0
    }

    /// Read **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn pde(&self) -> bool {
        (self.bits & Swd::PDE_MASK) != 0
    }

    /// Read **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn schmitt(&self) -> bool {
        (self.bits & Swd::SCHMITT_MASK) != 0
    }

    /// Read **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn slewfast(&self) -> bool {
        (self.bits & Swd::SLEWFAST_MASK) != 0
    }

    /// Set **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn with_iso(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Swd::ISO_MASK
            } else {
                self.bits & !Swd::ISO_MASK
            }
        }
    }

    /// Set **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn with_od(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Swd::OD_MASK
            } else {
                self.bits & !Swd::OD_MASK
            }
        }
    }

    /// Set **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn with_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Swd::IE_MASK
            } else {
                self.bits & !Swd::IE_MASK
            }
        }
    }

    /// Set **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn with_drive(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Swd::DRIVE_MASK) | ((value << Swd::DRIVE_OFFSET) & Swd::DRIVE_MASK)
        }
    }

    /// Set **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn with_pue(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Swd::PUE_MASK
            } else {
                self.bits & !Swd::PUE_MASK
            }
        }
    }

    /// Set **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn with_pde(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Swd::PDE_MASK
            } else {
                self.bits & !Swd::PDE_MASK
            }
        }
    }

    /// Set **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn with_schmitt(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Swd::SCHMITT_MASK
            } else {
                self.bits & !Swd::SCHMITT_MASK
            }
        }
    }

    /// Set **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn with_slewfast(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Swd::SLEWFAST_MASK
            } else {
                self.bits & !Swd::SLEWFAST_MASK
            }
        }
    }

}

impl Default for SwdVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Swd {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SwdVal {
        SwdVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SwdVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SwdVal) -> SwdVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// Voltage select. Per bank control
    pub voltage_select: VoltageSelect,
    /// 0x0004
    pub gpio0: Gpio0,
    /// 0x0008
    pub gpio1: Gpio1,
    /// 0x000c
    pub gpio2: Gpio2,
    /// 0x0010
    pub gpio3: Gpio3,
    /// 0x0014
    pub gpio4: Gpio4,
    /// 0x0018
    pub gpio5: Gpio5,
    /// 0x001c
    pub gpio6: Gpio6,
    /// 0x0020
    pub gpio7: Gpio7,
    /// 0x0024
    pub gpio8: Gpio8,
    /// 0x0028
    pub gpio9: Gpio9,
    /// 0x002c
    pub gpio10: Gpio10,
    /// 0x0030
    pub gpio11: Gpio11,
    /// 0x0034
    pub gpio12: Gpio12,
    /// 0x0038
    pub gpio13: Gpio13,
    /// 0x003c
    pub gpio14: Gpio14,
    /// 0x0040
    pub gpio15: Gpio15,
    /// 0x0044
    pub gpio16: Gpio16,
    /// 0x0048
    pub gpio17: Gpio17,
    /// 0x004c
    pub gpio18: Gpio18,
    /// 0x0050
    pub gpio19: Gpio19,
    /// 0x0054
    pub gpio20: Gpio20,
    /// 0x0058
    pub gpio21: Gpio21,
    /// 0x005c
    pub gpio22: Gpio22,
    /// 0x0060
    pub gpio23: Gpio23,
    /// 0x0064
    pub gpio24: Gpio24,
    /// 0x0068
    pub gpio25: Gpio25,
    /// 0x006c
    pub gpio26: Gpio26,
    /// 0x0070
    pub gpio27: Gpio27,
    /// 0x0074
    pub gpio28: Gpio28,
    /// 0x0078
    pub gpio29: Gpio29,
    /// 0x007c
    pub gpio30: Gpio30,
    /// 0x0080
    pub gpio31: Gpio31,
    /// 0x0084
    pub gpio32: Gpio32,
    /// 0x0088
    pub gpio33: Gpio33,
    /// 0x008c
    pub gpio34: Gpio34,
    /// 0x0090
    pub gpio35: Gpio35,
    /// 0x0094
    pub gpio36: Gpio36,
    /// 0x0098
    pub gpio37: Gpio37,
    /// 0x009c
    pub gpio38: Gpio38,
    /// 0x00a0
    pub gpio39: Gpio39,
    /// 0x00a4
    pub gpio40: Gpio40,
    /// 0x00a8
    pub gpio41: Gpio41,
    /// 0x00ac
    pub gpio42: Gpio42,
    /// 0x00b0
    pub gpio43: Gpio43,
    /// 0x00b4
    pub gpio44: Gpio44,
    /// 0x00b8
    pub gpio45: Gpio45,
    /// 0x00bc
    pub gpio46: Gpio46,
    /// 0x00c0
    pub gpio47: Gpio47,
    /// 0x00c4
    pub swclk: Swclk,
    /// 0x00c8
    pub swd: Swd,
}

