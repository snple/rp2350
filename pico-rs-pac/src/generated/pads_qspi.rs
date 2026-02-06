//! PADS_QSPI
//!
//! This peripheral supports atomic SET/CLR/XOR register aliases.

#![allow(dead_code)]

pub const BASE: u32 = 0x40040000;

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

/// GPIO_QSPI_SCLK Register
///
/// Address offset: 0x0004
/// Reset value: 0x00000156
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
pub struct GpioQspiSclk {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for GpioQspiSclk {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl GpioQspiSclk {
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

/// GPIO_QSPI_SCLK value type for batch read/modify/write operations
///
/// Use `gpioqspisclk.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpioqspisclk.write_val(val)`.
#[derive(Clone, Copy)]
pub struct GpioQspiSclkVal {
    bits: u32,
}

impl GpioQspiSclkVal {
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
        Self { bits: 0x00000156 }
    }

    /// Read **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn iso(&self) -> bool {
        (self.bits & GpioQspiSclk::ISO_MASK) != 0
    }

    /// Read **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn od(&self) -> bool {
        (self.bits & GpioQspiSclk::OD_MASK) != 0
    }

    /// Read **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        (self.bits & GpioQspiSclk::IE_MASK) != 0
    }

    /// Read **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn drive(&self) -> u32 {
        (self.bits & GpioQspiSclk::DRIVE_MASK) >> GpioQspiSclk::DRIVE_OFFSET
    }

    /// Read **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn pue(&self) -> bool {
        (self.bits & GpioQspiSclk::PUE_MASK) != 0
    }

    /// Read **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn pde(&self) -> bool {
        (self.bits & GpioQspiSclk::PDE_MASK) != 0
    }

    /// Read **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn schmitt(&self) -> bool {
        (self.bits & GpioQspiSclk::SCHMITT_MASK) != 0
    }

    /// Read **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn slewfast(&self) -> bool {
        (self.bits & GpioQspiSclk::SLEWFAST_MASK) != 0
    }

    /// Set **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn with_iso(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | GpioQspiSclk::ISO_MASK
            } else {
                self.bits & !GpioQspiSclk::ISO_MASK
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
                self.bits | GpioQspiSclk::OD_MASK
            } else {
                self.bits & !GpioQspiSclk::OD_MASK
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
                self.bits | GpioQspiSclk::IE_MASK
            } else {
                self.bits & !GpioQspiSclk::IE_MASK
            }
        }
    }

    /// Set **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn with_drive(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GpioQspiSclk::DRIVE_MASK) | ((value << GpioQspiSclk::DRIVE_OFFSET) & GpioQspiSclk::DRIVE_MASK)
        }
    }

    /// Set **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn with_pue(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | GpioQspiSclk::PUE_MASK
            } else {
                self.bits & !GpioQspiSclk::PUE_MASK
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
                self.bits | GpioQspiSclk::PDE_MASK
            } else {
                self.bits & !GpioQspiSclk::PDE_MASK
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
                self.bits | GpioQspiSclk::SCHMITT_MASK
            } else {
                self.bits & !GpioQspiSclk::SCHMITT_MASK
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
                self.bits | GpioQspiSclk::SLEWFAST_MASK
            } else {
                self.bits & !GpioQspiSclk::SLEWFAST_MASK
            }
        }
    }

}

impl Default for GpioQspiSclkVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl GpioQspiSclk {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> GpioQspiSclkVal {
        GpioQspiSclkVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: GpioQspiSclkVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(GpioQspiSclkVal) -> GpioQspiSclkVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// GPIO_QSPI_SD0 Register
///
/// Address offset: 0x0008
/// Reset value: 0x00000156
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
pub struct GpioQspiSd0 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for GpioQspiSd0 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl GpioQspiSd0 {
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

/// GPIO_QSPI_SD0 value type for batch read/modify/write operations
///
/// Use `gpioqspisd0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpioqspisd0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct GpioQspiSd0Val {
    bits: u32,
}

impl GpioQspiSd0Val {
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
        Self { bits: 0x00000156 }
    }

    /// Read **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn iso(&self) -> bool {
        (self.bits & GpioQspiSd0::ISO_MASK) != 0
    }

    /// Read **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn od(&self) -> bool {
        (self.bits & GpioQspiSd0::OD_MASK) != 0
    }

    /// Read **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        (self.bits & GpioQspiSd0::IE_MASK) != 0
    }

    /// Read **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn drive(&self) -> u32 {
        (self.bits & GpioQspiSd0::DRIVE_MASK) >> GpioQspiSd0::DRIVE_OFFSET
    }

    /// Read **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn pue(&self) -> bool {
        (self.bits & GpioQspiSd0::PUE_MASK) != 0
    }

    /// Read **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn pde(&self) -> bool {
        (self.bits & GpioQspiSd0::PDE_MASK) != 0
    }

    /// Read **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn schmitt(&self) -> bool {
        (self.bits & GpioQspiSd0::SCHMITT_MASK) != 0
    }

    /// Read **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn slewfast(&self) -> bool {
        (self.bits & GpioQspiSd0::SLEWFAST_MASK) != 0
    }

    /// Set **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn with_iso(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | GpioQspiSd0::ISO_MASK
            } else {
                self.bits & !GpioQspiSd0::ISO_MASK
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
                self.bits | GpioQspiSd0::OD_MASK
            } else {
                self.bits & !GpioQspiSd0::OD_MASK
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
                self.bits | GpioQspiSd0::IE_MASK
            } else {
                self.bits & !GpioQspiSd0::IE_MASK
            }
        }
    }

    /// Set **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn with_drive(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GpioQspiSd0::DRIVE_MASK) | ((value << GpioQspiSd0::DRIVE_OFFSET) & GpioQspiSd0::DRIVE_MASK)
        }
    }

    /// Set **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn with_pue(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | GpioQspiSd0::PUE_MASK
            } else {
                self.bits & !GpioQspiSd0::PUE_MASK
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
                self.bits | GpioQspiSd0::PDE_MASK
            } else {
                self.bits & !GpioQspiSd0::PDE_MASK
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
                self.bits | GpioQspiSd0::SCHMITT_MASK
            } else {
                self.bits & !GpioQspiSd0::SCHMITT_MASK
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
                self.bits | GpioQspiSd0::SLEWFAST_MASK
            } else {
                self.bits & !GpioQspiSd0::SLEWFAST_MASK
            }
        }
    }

}

impl Default for GpioQspiSd0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl GpioQspiSd0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> GpioQspiSd0Val {
        GpioQspiSd0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: GpioQspiSd0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(GpioQspiSd0Val) -> GpioQspiSd0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// GPIO_QSPI_SD1 Register
///
/// Address offset: 0x000c
/// Reset value: 0x00000156
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
pub struct GpioQspiSd1 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for GpioQspiSd1 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl GpioQspiSd1 {
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

/// GPIO_QSPI_SD1 value type for batch read/modify/write operations
///
/// Use `gpioqspisd1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpioqspisd1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct GpioQspiSd1Val {
    bits: u32,
}

impl GpioQspiSd1Val {
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
        Self { bits: 0x00000156 }
    }

    /// Read **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn iso(&self) -> bool {
        (self.bits & GpioQspiSd1::ISO_MASK) != 0
    }

    /// Read **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn od(&self) -> bool {
        (self.bits & GpioQspiSd1::OD_MASK) != 0
    }

    /// Read **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        (self.bits & GpioQspiSd1::IE_MASK) != 0
    }

    /// Read **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn drive(&self) -> u32 {
        (self.bits & GpioQspiSd1::DRIVE_MASK) >> GpioQspiSd1::DRIVE_OFFSET
    }

    /// Read **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn pue(&self) -> bool {
        (self.bits & GpioQspiSd1::PUE_MASK) != 0
    }

    /// Read **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn pde(&self) -> bool {
        (self.bits & GpioQspiSd1::PDE_MASK) != 0
    }

    /// Read **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn schmitt(&self) -> bool {
        (self.bits & GpioQspiSd1::SCHMITT_MASK) != 0
    }

    /// Read **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn slewfast(&self) -> bool {
        (self.bits & GpioQspiSd1::SLEWFAST_MASK) != 0
    }

    /// Set **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn with_iso(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | GpioQspiSd1::ISO_MASK
            } else {
                self.bits & !GpioQspiSd1::ISO_MASK
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
                self.bits | GpioQspiSd1::OD_MASK
            } else {
                self.bits & !GpioQspiSd1::OD_MASK
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
                self.bits | GpioQspiSd1::IE_MASK
            } else {
                self.bits & !GpioQspiSd1::IE_MASK
            }
        }
    }

    /// Set **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn with_drive(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GpioQspiSd1::DRIVE_MASK) | ((value << GpioQspiSd1::DRIVE_OFFSET) & GpioQspiSd1::DRIVE_MASK)
        }
    }

    /// Set **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn with_pue(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | GpioQspiSd1::PUE_MASK
            } else {
                self.bits & !GpioQspiSd1::PUE_MASK
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
                self.bits | GpioQspiSd1::PDE_MASK
            } else {
                self.bits & !GpioQspiSd1::PDE_MASK
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
                self.bits | GpioQspiSd1::SCHMITT_MASK
            } else {
                self.bits & !GpioQspiSd1::SCHMITT_MASK
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
                self.bits | GpioQspiSd1::SLEWFAST_MASK
            } else {
                self.bits & !GpioQspiSd1::SLEWFAST_MASK
            }
        }
    }

}

impl Default for GpioQspiSd1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl GpioQspiSd1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> GpioQspiSd1Val {
        GpioQspiSd1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: GpioQspiSd1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(GpioQspiSd1Val) -> GpioQspiSd1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// GPIO_QSPI_SD2 Register
///
/// Address offset: 0x0010
/// Reset value: 0x0000015a
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
pub struct GpioQspiSd2 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for GpioQspiSd2 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl GpioQspiSd2 {
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

/// GPIO_QSPI_SD2 value type for batch read/modify/write operations
///
/// Use `gpioqspisd2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpioqspisd2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct GpioQspiSd2Val {
    bits: u32,
}

impl GpioQspiSd2Val {
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
        Self { bits: 0x0000015a }
    }

    /// Read **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn iso(&self) -> bool {
        (self.bits & GpioQspiSd2::ISO_MASK) != 0
    }

    /// Read **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn od(&self) -> bool {
        (self.bits & GpioQspiSd2::OD_MASK) != 0
    }

    /// Read **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        (self.bits & GpioQspiSd2::IE_MASK) != 0
    }

    /// Read **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn drive(&self) -> u32 {
        (self.bits & GpioQspiSd2::DRIVE_MASK) >> GpioQspiSd2::DRIVE_OFFSET
    }

    /// Read **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn pue(&self) -> bool {
        (self.bits & GpioQspiSd2::PUE_MASK) != 0
    }

    /// Read **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn pde(&self) -> bool {
        (self.bits & GpioQspiSd2::PDE_MASK) != 0
    }

    /// Read **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn schmitt(&self) -> bool {
        (self.bits & GpioQspiSd2::SCHMITT_MASK) != 0
    }

    /// Read **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn slewfast(&self) -> bool {
        (self.bits & GpioQspiSd2::SLEWFAST_MASK) != 0
    }

    /// Set **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn with_iso(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | GpioQspiSd2::ISO_MASK
            } else {
                self.bits & !GpioQspiSd2::ISO_MASK
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
                self.bits | GpioQspiSd2::OD_MASK
            } else {
                self.bits & !GpioQspiSd2::OD_MASK
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
                self.bits | GpioQspiSd2::IE_MASK
            } else {
                self.bits & !GpioQspiSd2::IE_MASK
            }
        }
    }

    /// Set **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn with_drive(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GpioQspiSd2::DRIVE_MASK) | ((value << GpioQspiSd2::DRIVE_OFFSET) & GpioQspiSd2::DRIVE_MASK)
        }
    }

    /// Set **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn with_pue(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | GpioQspiSd2::PUE_MASK
            } else {
                self.bits & !GpioQspiSd2::PUE_MASK
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
                self.bits | GpioQspiSd2::PDE_MASK
            } else {
                self.bits & !GpioQspiSd2::PDE_MASK
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
                self.bits | GpioQspiSd2::SCHMITT_MASK
            } else {
                self.bits & !GpioQspiSd2::SCHMITT_MASK
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
                self.bits | GpioQspiSd2::SLEWFAST_MASK
            } else {
                self.bits & !GpioQspiSd2::SLEWFAST_MASK
            }
        }
    }

}

impl Default for GpioQspiSd2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl GpioQspiSd2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> GpioQspiSd2Val {
        GpioQspiSd2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: GpioQspiSd2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(GpioQspiSd2Val) -> GpioQspiSd2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// GPIO_QSPI_SD3 Register
///
/// Address offset: 0x0014
/// Reset value: 0x0000015a
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
pub struct GpioQspiSd3 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for GpioQspiSd3 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl GpioQspiSd3 {
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

/// GPIO_QSPI_SD3 value type for batch read/modify/write operations
///
/// Use `gpioqspisd3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpioqspisd3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct GpioQspiSd3Val {
    bits: u32,
}

impl GpioQspiSd3Val {
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
        Self { bits: 0x0000015a }
    }

    /// Read **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn iso(&self) -> bool {
        (self.bits & GpioQspiSd3::ISO_MASK) != 0
    }

    /// Read **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn od(&self) -> bool {
        (self.bits & GpioQspiSd3::OD_MASK) != 0
    }

    /// Read **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        (self.bits & GpioQspiSd3::IE_MASK) != 0
    }

    /// Read **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn drive(&self) -> u32 {
        (self.bits & GpioQspiSd3::DRIVE_MASK) >> GpioQspiSd3::DRIVE_OFFSET
    }

    /// Read **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn pue(&self) -> bool {
        (self.bits & GpioQspiSd3::PUE_MASK) != 0
    }

    /// Read **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn pde(&self) -> bool {
        (self.bits & GpioQspiSd3::PDE_MASK) != 0
    }

    /// Read **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn schmitt(&self) -> bool {
        (self.bits & GpioQspiSd3::SCHMITT_MASK) != 0
    }

    /// Read **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn slewfast(&self) -> bool {
        (self.bits & GpioQspiSd3::SLEWFAST_MASK) != 0
    }

    /// Set **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn with_iso(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | GpioQspiSd3::ISO_MASK
            } else {
                self.bits & !GpioQspiSd3::ISO_MASK
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
                self.bits | GpioQspiSd3::OD_MASK
            } else {
                self.bits & !GpioQspiSd3::OD_MASK
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
                self.bits | GpioQspiSd3::IE_MASK
            } else {
                self.bits & !GpioQspiSd3::IE_MASK
            }
        }
    }

    /// Set **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn with_drive(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GpioQspiSd3::DRIVE_MASK) | ((value << GpioQspiSd3::DRIVE_OFFSET) & GpioQspiSd3::DRIVE_MASK)
        }
    }

    /// Set **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn with_pue(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | GpioQspiSd3::PUE_MASK
            } else {
                self.bits & !GpioQspiSd3::PUE_MASK
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
                self.bits | GpioQspiSd3::PDE_MASK
            } else {
                self.bits & !GpioQspiSd3::PDE_MASK
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
                self.bits | GpioQspiSd3::SCHMITT_MASK
            } else {
                self.bits & !GpioQspiSd3::SCHMITT_MASK
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
                self.bits | GpioQspiSd3::SLEWFAST_MASK
            } else {
                self.bits & !GpioQspiSd3::SLEWFAST_MASK
            }
        }
    }

}

impl Default for GpioQspiSd3Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl GpioQspiSd3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> GpioQspiSd3Val {
        GpioQspiSd3Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: GpioQspiSd3Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(GpioQspiSd3Val) -> GpioQspiSd3Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// GPIO_QSPI_SS Register
///
/// Address offset: 0x0018
/// Reset value: 0x0000015a
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
pub struct GpioQspiSs {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for GpioQspiSs {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl GpioQspiSs {
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

/// GPIO_QSPI_SS value type for batch read/modify/write operations
///
/// Use `gpioqspiss.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpioqspiss.write_val(val)`.
#[derive(Clone, Copy)]
pub struct GpioQspiSsVal {
    bits: u32,
}

impl GpioQspiSsVal {
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
        Self { bits: 0x0000015a }
    }

    /// Read **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn iso(&self) -> bool {
        (self.bits & GpioQspiSs::ISO_MASK) != 0
    }

    /// Read **OD** `[7:7]`
    ///
    /// Output disable. Has priority over output enable from peripherals
    #[inline(always)]
    pub const fn od(&self) -> bool {
        (self.bits & GpioQspiSs::OD_MASK) != 0
    }

    /// Read **IE** `[6:6]`
    ///
    /// Input enable
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        (self.bits & GpioQspiSs::IE_MASK) != 0
    }

    /// Read **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn drive(&self) -> u32 {
        (self.bits & GpioQspiSs::DRIVE_MASK) >> GpioQspiSs::DRIVE_OFFSET
    }

    /// Read **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn pue(&self) -> bool {
        (self.bits & GpioQspiSs::PUE_MASK) != 0
    }

    /// Read **PDE** `[2:2]`
    ///
    /// Pull down enable
    #[inline(always)]
    pub const fn pde(&self) -> bool {
        (self.bits & GpioQspiSs::PDE_MASK) != 0
    }

    /// Read **SCHMITT** `[1:1]`
    ///
    /// Enable schmitt trigger
    #[inline(always)]
    pub const fn schmitt(&self) -> bool {
        (self.bits & GpioQspiSs::SCHMITT_MASK) != 0
    }

    /// Read **SLEWFAST** `[0:0]`
    ///
    /// Slew rate control. 1 = Fast, 0 = Slow
    #[inline(always)]
    pub const fn slewfast(&self) -> bool {
        (self.bits & GpioQspiSs::SLEWFAST_MASK) != 0
    }

    /// Set **ISO** `[8:8]`
    ///
    /// Pad isolation control. Remove this once the pad is configured by software.
    #[inline(always)]
    pub const fn with_iso(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | GpioQspiSs::ISO_MASK
            } else {
                self.bits & !GpioQspiSs::ISO_MASK
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
                self.bits | GpioQspiSs::OD_MASK
            } else {
                self.bits & !GpioQspiSs::OD_MASK
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
                self.bits | GpioQspiSs::IE_MASK
            } else {
                self.bits & !GpioQspiSs::IE_MASK
            }
        }
    }

    /// Set **DRIVE** `[5:4]`
    ///
    /// Drive strength.
    #[inline(always)]
    pub const fn with_drive(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GpioQspiSs::DRIVE_MASK) | ((value << GpioQspiSs::DRIVE_OFFSET) & GpioQspiSs::DRIVE_MASK)
        }
    }

    /// Set **PUE** `[3:3]`
    ///
    /// Pull up enable
    #[inline(always)]
    pub const fn with_pue(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | GpioQspiSs::PUE_MASK
            } else {
                self.bits & !GpioQspiSs::PUE_MASK
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
                self.bits | GpioQspiSs::PDE_MASK
            } else {
                self.bits & !GpioQspiSs::PDE_MASK
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
                self.bits | GpioQspiSs::SCHMITT_MASK
            } else {
                self.bits & !GpioQspiSs::SCHMITT_MASK
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
                self.bits | GpioQspiSs::SLEWFAST_MASK
            } else {
                self.bits & !GpioQspiSs::SLEWFAST_MASK
            }
        }
    }

}

impl Default for GpioQspiSsVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl GpioQspiSs {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> GpioQspiSsVal {
        GpioQspiSsVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: GpioQspiSsVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(GpioQspiSsVal) -> GpioQspiSsVal>(&self, f: F) {
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
    pub gpio_qspi_sclk: GpioQspiSclk,
    /// 0x0008
    pub gpio_qspi_sd0: GpioQspiSd0,
    /// 0x000c
    pub gpio_qspi_sd1: GpioQspiSd1,
    /// 0x0010
    pub gpio_qspi_sd2: GpioQspiSd2,
    /// 0x0014
    pub gpio_qspi_sd3: GpioQspiSd3,
    /// 0x0018
    pub gpio_qspi_ss: GpioQspiSs,
}

