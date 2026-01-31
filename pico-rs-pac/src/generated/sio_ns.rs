//! SIO_NS

#![allow(dead_code)]

pub const BASE: u32 = 0xd0020000;

/// CPUID Register
///
/// Processor core identifier
///
/// Address offset: 0x0000
///
/// ## Fields
///
/// - **CPUID** `[31:0]` (RO)
///   Value is 0 when read from processor core 0, and 1 when read from processor core 1.
#[repr(transparent)]
pub struct Cpuid {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Cpuid {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Cpuid {
    // ======== Field Constants ========

    /// **CPUID**
    /// Value is 0 when read from processor core 0, and 1 when read from processor core 1.
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const CPUID_OFFSET: u32 = 0;
    pub const CPUID_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **CPUID** `[31:0]` (RO)
    /// Value is 0 when read from processor core 0, and 1 when read from processor core 1.
    #[inline(always)]
    pub fn cpuid(&self) -> u32 {
        (self.value.read_bits() & Self::CPUID_MASK) >> Self::CPUID_OFFSET
    }

}

/// CPUID value type for batch read/modify/write operations
///
/// Use `cpuid.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `cpuid.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CpuidVal {
    bits: u32,
}

impl CpuidVal {
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

    /// Read **CPUID** `[31:0]`
    /// Value is 0 when read from processor core 0, and 1 when read from processor core 1.
    #[inline(always)]
    pub const fn cpuid(&self) -> u32 {
        (self.bits & Cpuid::CPUID_MASK) >> Cpuid::CPUID_OFFSET
    }

}

impl Default for CpuidVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Cpuid {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CpuidVal {
        CpuidVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CpuidVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CpuidVal) -> CpuidVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// GPIO_IN Register
///
/// Input value for GPIO0...31.
/// In the Non-secure SIO, Secure-only GPIOs (as per ACCESSCTRL) appear as zero.
///
/// Address offset: 0x0004
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **GPIO_IN** `[31:0]` (RO)
#[repr(transparent)]
pub struct GpioIn {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for GpioIn {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl GpioIn {
    // ======== Field Constants ========

    /// **GPIO_IN**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const GPIO_IN_OFFSET: u32 = 0;
    pub const GPIO_IN_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **GPIO_IN** `[31:0]` (RO)
    #[inline(always)]
    pub fn gpio_in(&self) -> u32 {
        (self.value.read_bits() & Self::GPIO_IN_MASK) >> Self::GPIO_IN_OFFSET
    }

}

/// GPIO_IN value type for batch read/modify/write operations
///
/// Use `gpioin.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpioin.write_val(val)`.
#[derive(Clone, Copy)]
pub struct GpioInVal {
    bits: u32,
}

impl GpioInVal {
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

    /// Read **GPIO_IN** `[31:0]`
    #[inline(always)]
    pub const fn gpio_in(&self) -> u32 {
        (self.bits & GpioIn::GPIO_IN_MASK) >> GpioIn::GPIO_IN_OFFSET
    }

}

impl Default for GpioInVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl GpioIn {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> GpioInVal {
        GpioInVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: GpioInVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(GpioInVal) -> GpioInVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// GPIO_HI_IN Register
///
/// Input value on GPIO32...47, QSPI IOs and USB pins
/// In the Non-secure SIO, Secure-only GPIOs (as per ACCESSCTRL) appear as zero.
///
/// Address offset: 0x0008
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **QSPI_SD** `[31:28]` (RO)
///   Input value on QSPI SD0 (MOSI), SD1 (MISO), SD2 and SD3 pins
/// - **QSPI_CSN** `[27]` (RO)
///   Input value on QSPI CSn pin
/// - **QSPI_SCK** `[26]` (RO)
///   Input value on QSPI SCK pin
/// - **USB_DM** `[25]` (RO)
///   Input value on USB D- pin
/// - **USB_DP** `[24]` (RO)
///   Input value on USB D+ pin
/// - **GPIO** `[15:0]` (RO)
///   Input value on GPIO32...47
#[repr(transparent)]
pub struct GpioHiIn {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for GpioHiIn {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl GpioHiIn {
    // ======== Field Constants ========

    /// **QSPI_SD**
    /// Input value on QSPI SD0 (MOSI), SD1 (MISO), SD2 and SD3 pins
    /// Bits `[31:28]`, mask `0xf0000000`
    pub const QSPI_SD_OFFSET: u32 = 28;
    pub const QSPI_SD_MASK: u32 = 0xf0000000;

    /// **QSPI_CSN**
    /// Input value on QSPI CSn pin
    /// Bits `[27:27]`, mask `0x08000000`
    pub const QSPI_CSN_OFFSET: u32 = 27;
    pub const QSPI_CSN_MASK: u32 = 0x08000000;
    pub const QSPI_CSN: u32 = 1 << 27;

    /// **QSPI_SCK**
    /// Input value on QSPI SCK pin
    /// Bits `[26:26]`, mask `0x04000000`
    pub const QSPI_SCK_OFFSET: u32 = 26;
    pub const QSPI_SCK_MASK: u32 = 0x04000000;
    pub const QSPI_SCK: u32 = 1 << 26;

    /// **USB_DM**
    /// Input value on USB D- pin
    /// Bits `[25:25]`, mask `0x02000000`
    pub const USB_DM_OFFSET: u32 = 25;
    pub const USB_DM_MASK: u32 = 0x02000000;
    pub const USB_DM: u32 = 1 << 25;

    /// **USB_DP**
    /// Input value on USB D+ pin
    /// Bits `[24:24]`, mask `0x01000000`
    pub const USB_DP_OFFSET: u32 = 24;
    pub const USB_DP_MASK: u32 = 0x01000000;
    pub const USB_DP: u32 = 1 << 24;

    /// **GPIO**
    /// Input value on GPIO32...47
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const GPIO_OFFSET: u32 = 0;
    pub const GPIO_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **QSPI_SD** `[31:28]` (RO)
    /// Input value on QSPI SD0 (MOSI), SD1 (MISO), SD2 and SD3 pins
    #[inline(always)]
    pub fn qspi_sd(&self) -> u32 {
        (self.value.read_bits() & Self::QSPI_SD_MASK) >> Self::QSPI_SD_OFFSET
    }

    /// **QSPI_CSN** `[27]` (RO)
    /// Input value on QSPI CSn pin
    #[inline(always)]
    pub fn qspi_csn(&self) -> bool {
        (self.value.read_bits() & Self::QSPI_CSN_MASK) != 0
    }

    /// **QSPI_SCK** `[26]` (RO)
    /// Input value on QSPI SCK pin
    #[inline(always)]
    pub fn qspi_sck(&self) -> bool {
        (self.value.read_bits() & Self::QSPI_SCK_MASK) != 0
    }

    /// **USB_DM** `[25]` (RO)
    /// Input value on USB D- pin
    #[inline(always)]
    pub fn usb_dm(&self) -> bool {
        (self.value.read_bits() & Self::USB_DM_MASK) != 0
    }

    /// **USB_DP** `[24]` (RO)
    /// Input value on USB D+ pin
    #[inline(always)]
    pub fn usb_dp(&self) -> bool {
        (self.value.read_bits() & Self::USB_DP_MASK) != 0
    }

    /// **GPIO** `[15:0]` (RO)
    /// Input value on GPIO32...47
    #[inline(always)]
    pub fn gpio(&self) -> u32 {
        (self.value.read_bits() & Self::GPIO_MASK) >> Self::GPIO_OFFSET
    }

}

/// GPIO_HI_IN value type for batch read/modify/write operations
///
/// Use `gpiohiin.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpiohiin.write_val(val)`.
#[derive(Clone, Copy)]
pub struct GpioHiInVal {
    bits: u32,
}

impl GpioHiInVal {
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

    /// Read **QSPI_SD** `[31:28]`
    /// Input value on QSPI SD0 (MOSI), SD1 (MISO), SD2 and SD3 pins
    #[inline(always)]
    pub const fn qspi_sd(&self) -> u32 {
        (self.bits & GpioHiIn::QSPI_SD_MASK) >> GpioHiIn::QSPI_SD_OFFSET
    }

    /// Read **QSPI_CSN** `[27:27]`
    /// Input value on QSPI CSn pin
    #[inline(always)]
    pub const fn qspi_csn(&self) -> bool {
        (self.bits & GpioHiIn::QSPI_CSN_MASK) != 0
    }

    /// Read **QSPI_SCK** `[26:26]`
    /// Input value on QSPI SCK pin
    #[inline(always)]
    pub const fn qspi_sck(&self) -> bool {
        (self.bits & GpioHiIn::QSPI_SCK_MASK) != 0
    }

    /// Read **USB_DM** `[25:25]`
    /// Input value on USB D- pin
    #[inline(always)]
    pub const fn usb_dm(&self) -> bool {
        (self.bits & GpioHiIn::USB_DM_MASK) != 0
    }

    /// Read **USB_DP** `[24:24]`
    /// Input value on USB D+ pin
    #[inline(always)]
    pub const fn usb_dp(&self) -> bool {
        (self.bits & GpioHiIn::USB_DP_MASK) != 0
    }

    /// Read **GPIO** `[15:0]`
    /// Input value on GPIO32...47
    #[inline(always)]
    pub const fn gpio(&self) -> u32 {
        (self.bits & GpioHiIn::GPIO_MASK) >> GpioHiIn::GPIO_OFFSET
    }

}

impl Default for GpioHiInVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl GpioHiIn {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> GpioHiInVal {
        GpioHiInVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: GpioHiInVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(GpioHiInVal) -> GpioHiInVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// GPIO_OUT Register
///
/// GPIO0...31 output value
///
/// Address offset: 0x0010
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **GPIO_OUT** `[31:0]` (RW)
///   Set output level (1/0 -> high/low) for GPIO0...31. Reading back gives the last value written, NOT the input value from the pins.
///   If core 0 and core 1 both write to GPIO_OUT simultaneously (or to a SET/CLR/XOR alias), the result is as though the write from core 0 took place first, and the write from core 1 was then applied to that intermediate result.
///   In the Non-secure SIO, Secure-only GPIOs (as per ACCESSCTRL) ignore writes, and their output status reads back as zero. This is also true for SET/CLR/XOR aliases of this register.
#[repr(transparent)]
pub struct GpioOut {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for GpioOut {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl GpioOut {
    // ======== Field Constants ========

    /// **GPIO_OUT**
    /// Set output level (1/0 -> high/low) for GPIO0...31. Reading back gives the last value written, NOT the input value from the pins.
    /// If core 0 and core 1 both write to GPIO_OUT simultaneously (or to a SET/CLR/XOR alias), the result is as though the write from core 0 took place first, and the write from core 1 was then applied to that intermediate result.
    /// In the Non-secure SIO, Secure-only GPIOs (as per ACCESSCTRL) ignore writes, and their output status reads back as zero. This is also true for SET/CLR/XOR aliases of this register.
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const GPIO_OUT_OFFSET: u32 = 0;
    pub const GPIO_OUT_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **GPIO_OUT** `[31:0]` (RW)
    /// Set output level (1/0 -> high/low) for GPIO0...31. Reading back gives the last value written, NOT the input value from the pins.
    /// If core 0 and core 1 both write to GPIO_OUT simultaneously (or to a SET/CLR/XOR alias), the result is as though the write from core 0 took place first, and the write from core 1 was then applied to that intermediate result.
    /// In the Non-secure SIO, Secure-only GPIOs (as per ACCESSCTRL) ignore writes, and their output status reads back as zero. This is also true for SET/CLR/XOR aliases of this register.
    #[inline(always)]
    pub fn gpio_out(&self) -> u32 {
        (self.value.read_bits() & Self::GPIO_OUT_MASK) >> Self::GPIO_OUT_OFFSET
    }

    /// Write **GPIO_OUT** `[31:0]` (RW)
    /// Set output level (1/0 -> high/low) for GPIO0...31. Reading back gives the last value written, NOT the input value from the pins.
    /// If core 0 and core 1 both write to GPIO_OUT simultaneously (or to a SET/CLR/XOR alias), the result is as though the write from core 0 took place first, and the write from core 1 was then applied to that intermediate result.
    /// In the Non-secure SIO, Secure-only GPIOs (as per ACCESSCTRL) ignore writes, and their output status reads back as zero. This is also true for SET/CLR/XOR aliases of this register.
    #[inline(always)]
    pub fn write_gpio_out(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::GPIO_OUT_MASK) | ((value << Self::GPIO_OUT_OFFSET) & Self::GPIO_OUT_MASK));
    }

}

/// GPIO_OUT value type for batch read/modify/write operations
///
/// Use `gpioout.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpioout.write_val(val)`.
#[derive(Clone, Copy)]
pub struct GpioOutVal {
    bits: u32,
}

impl GpioOutVal {
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

    /// Read **GPIO_OUT** `[31:0]`
    /// Set output level (1/0 -> high/low) for GPIO0...31. Reading back gives the last value written, NOT the input value from the pins.
    /// If core 0 and core 1 both write to GPIO_OUT simultaneously (or to a SET/CLR/XOR alias), the result is as though the write from core 0 took place first, and the write from core 1 was then applied to that intermediate result.
    /// In the Non-secure SIO, Secure-only GPIOs (as per ACCESSCTRL) ignore writes, and their output status reads back as zero. This is also true for SET/CLR/XOR aliases of this register.
    #[inline(always)]
    pub const fn gpio_out(&self) -> u32 {
        (self.bits & GpioOut::GPIO_OUT_MASK) >> GpioOut::GPIO_OUT_OFFSET
    }

    /// Set **GPIO_OUT** `[31:0]`
    /// Set output level (1/0 -> high/low) for GPIO0...31. Reading back gives the last value written, NOT the input value from the pins.
    /// If core 0 and core 1 both write to GPIO_OUT simultaneously (or to a SET/CLR/XOR alias), the result is as though the write from core 0 took place first, and the write from core 1 was then applied to that intermediate result.
    /// In the Non-secure SIO, Secure-only GPIOs (as per ACCESSCTRL) ignore writes, and their output status reads back as zero. This is also true for SET/CLR/XOR aliases of this register.
    #[inline(always)]
    pub const fn with_gpio_out(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GpioOut::GPIO_OUT_MASK) | ((value << GpioOut::GPIO_OUT_OFFSET) & GpioOut::GPIO_OUT_MASK)
        }
    }

}

impl Default for GpioOutVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl GpioOut {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> GpioOutVal {
        GpioOutVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: GpioOutVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(GpioOutVal) -> GpioOutVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// GPIO_HI_OUT Register
///
/// Output value for GPIO32...47, QSPI IOs and USB pins.
/// Write to set output level (1/0 -> high/low). Reading back gives the last value written, NOT the input value from the pins. If core 0 and core 1 both write to GPIO_HI_OUT simultaneously (or to a SET/CLR/XOR alias), the result is as though the write from core 0 took place first, and the write from core 1 was then applied to that intermediate result.
/// In the Non-secure SIO, Secure-only GPIOs (as per ACCESSCTRL) ignore writes, and their output status reads back as zero. This is also true for SET/CLR/XOR aliases of this register.
///
/// Address offset: 0x0014
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **QSPI_SD** `[31:28]` (RW)
///   Output value for QSPI SD0 (MOSI), SD1 (MISO), SD2 and SD3 pins
/// - **QSPI_CSN** `[27]` (RW)
///   Output value for QSPI CSn pin
/// - **QSPI_SCK** `[26]` (RW)
///   Output value for QSPI SCK pin
/// - **USB_DM** `[25]` (RW)
///   Output value for USB D- pin
/// - **USB_DP** `[24]` (RW)
///   Output value for USB D+ pin
/// - **GPIO** `[15:0]` (RW)
///   Output value for GPIO32...47
#[repr(transparent)]
pub struct GpioHiOut {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for GpioHiOut {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl GpioHiOut {
    // ======== Field Constants ========

    /// **QSPI_SD**
    /// Output value for QSPI SD0 (MOSI), SD1 (MISO), SD2 and SD3 pins
    /// Bits `[31:28]`, mask `0xf0000000`
    pub const QSPI_SD_OFFSET: u32 = 28;
    pub const QSPI_SD_MASK: u32 = 0xf0000000;

    /// **QSPI_CSN**
    /// Output value for QSPI CSn pin
    /// Bits `[27:27]`, mask `0x08000000`
    pub const QSPI_CSN_OFFSET: u32 = 27;
    pub const QSPI_CSN_MASK: u32 = 0x08000000;
    pub const QSPI_CSN: u32 = 1 << 27;

    /// **QSPI_SCK**
    /// Output value for QSPI SCK pin
    /// Bits `[26:26]`, mask `0x04000000`
    pub const QSPI_SCK_OFFSET: u32 = 26;
    pub const QSPI_SCK_MASK: u32 = 0x04000000;
    pub const QSPI_SCK: u32 = 1 << 26;

    /// **USB_DM**
    /// Output value for USB D- pin
    /// Bits `[25:25]`, mask `0x02000000`
    pub const USB_DM_OFFSET: u32 = 25;
    pub const USB_DM_MASK: u32 = 0x02000000;
    pub const USB_DM: u32 = 1 << 25;

    /// **USB_DP**
    /// Output value for USB D+ pin
    /// Bits `[24:24]`, mask `0x01000000`
    pub const USB_DP_OFFSET: u32 = 24;
    pub const USB_DP_MASK: u32 = 0x01000000;
    pub const USB_DP: u32 = 1 << 24;

    /// **GPIO**
    /// Output value for GPIO32...47
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const GPIO_OFFSET: u32 = 0;
    pub const GPIO_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **QSPI_SD** `[31:28]` (RW)
    /// Output value for QSPI SD0 (MOSI), SD1 (MISO), SD2 and SD3 pins
    #[inline(always)]
    pub fn qspi_sd(&self) -> u32 {
        (self.value.read_bits() & Self::QSPI_SD_MASK) >> Self::QSPI_SD_OFFSET
    }

    /// Write **QSPI_SD** `[31:28]` (RW)
    /// Output value for QSPI SD0 (MOSI), SD1 (MISO), SD2 and SD3 pins
    #[inline(always)]
    pub fn write_qspi_sd(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::QSPI_SD_MASK) | ((value << Self::QSPI_SD_OFFSET) & Self::QSPI_SD_MASK));
    }

    /// **QSPI_CSN** `[27]` (RW)
    /// Output value for QSPI CSn pin
    #[inline(always)]
    pub fn qspi_csn(&self) -> bool {
        (self.value.read_bits() & Self::QSPI_CSN_MASK) != 0
    }

    /// Set QSPI_CSN bit atomically (RW)
    #[inline(always)]
    pub fn set_qspi_csn(&self) {
        self.value.set_bits(Self::QSPI_CSN_MASK);
    }

    /// Clear QSPI_CSN bit atomically (RW)
    #[inline(always)]
    pub fn clr_qspi_csn(&self) {
        self.value.clr_bits(Self::QSPI_CSN_MASK);
    }

    /// **QSPI_SCK** `[26]` (RW)
    /// Output value for QSPI SCK pin
    #[inline(always)]
    pub fn qspi_sck(&self) -> bool {
        (self.value.read_bits() & Self::QSPI_SCK_MASK) != 0
    }

    /// Set QSPI_SCK bit atomically (RW)
    #[inline(always)]
    pub fn set_qspi_sck(&self) {
        self.value.set_bits(Self::QSPI_SCK_MASK);
    }

    /// Clear QSPI_SCK bit atomically (RW)
    #[inline(always)]
    pub fn clr_qspi_sck(&self) {
        self.value.clr_bits(Self::QSPI_SCK_MASK);
    }

    /// **USB_DM** `[25]` (RW)
    /// Output value for USB D- pin
    #[inline(always)]
    pub fn usb_dm(&self) -> bool {
        (self.value.read_bits() & Self::USB_DM_MASK) != 0
    }

    /// Set USB_DM bit atomically (RW)
    #[inline(always)]
    pub fn set_usb_dm(&self) {
        self.value.set_bits(Self::USB_DM_MASK);
    }

    /// Clear USB_DM bit atomically (RW)
    #[inline(always)]
    pub fn clr_usb_dm(&self) {
        self.value.clr_bits(Self::USB_DM_MASK);
    }

    /// **USB_DP** `[24]` (RW)
    /// Output value for USB D+ pin
    #[inline(always)]
    pub fn usb_dp(&self) -> bool {
        (self.value.read_bits() & Self::USB_DP_MASK) != 0
    }

    /// Set USB_DP bit atomically (RW)
    #[inline(always)]
    pub fn set_usb_dp(&self) {
        self.value.set_bits(Self::USB_DP_MASK);
    }

    /// Clear USB_DP bit atomically (RW)
    #[inline(always)]
    pub fn clr_usb_dp(&self) {
        self.value.clr_bits(Self::USB_DP_MASK);
    }

    /// **GPIO** `[15:0]` (RW)
    /// Output value for GPIO32...47
    #[inline(always)]
    pub fn gpio(&self) -> u32 {
        (self.value.read_bits() & Self::GPIO_MASK) >> Self::GPIO_OFFSET
    }

    /// Write **GPIO** `[15:0]` (RW)
    /// Output value for GPIO32...47
    #[inline(always)]
    pub fn write_gpio(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::GPIO_MASK) | ((value << Self::GPIO_OFFSET) & Self::GPIO_MASK));
    }

}

/// GPIO_HI_OUT value type for batch read/modify/write operations
///
/// Use `gpiohiout.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpiohiout.write_val(val)`.
#[derive(Clone, Copy)]
pub struct GpioHiOutVal {
    bits: u32,
}

impl GpioHiOutVal {
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

    /// Read **QSPI_SD** `[31:28]`
    /// Output value for QSPI SD0 (MOSI), SD1 (MISO), SD2 and SD3 pins
    #[inline(always)]
    pub const fn qspi_sd(&self) -> u32 {
        (self.bits & GpioHiOut::QSPI_SD_MASK) >> GpioHiOut::QSPI_SD_OFFSET
    }

    /// Read **QSPI_CSN** `[27:27]`
    /// Output value for QSPI CSn pin
    #[inline(always)]
    pub const fn qspi_csn(&self) -> bool {
        (self.bits & GpioHiOut::QSPI_CSN_MASK) != 0
    }

    /// Read **QSPI_SCK** `[26:26]`
    /// Output value for QSPI SCK pin
    #[inline(always)]
    pub const fn qspi_sck(&self) -> bool {
        (self.bits & GpioHiOut::QSPI_SCK_MASK) != 0
    }

    /// Read **USB_DM** `[25:25]`
    /// Output value for USB D- pin
    #[inline(always)]
    pub const fn usb_dm(&self) -> bool {
        (self.bits & GpioHiOut::USB_DM_MASK) != 0
    }

    /// Read **USB_DP** `[24:24]`
    /// Output value for USB D+ pin
    #[inline(always)]
    pub const fn usb_dp(&self) -> bool {
        (self.bits & GpioHiOut::USB_DP_MASK) != 0
    }

    /// Read **GPIO** `[15:0]`
    /// Output value for GPIO32...47
    #[inline(always)]
    pub const fn gpio(&self) -> u32 {
        (self.bits & GpioHiOut::GPIO_MASK) >> GpioHiOut::GPIO_OFFSET
    }

    /// Set **QSPI_SD** `[31:28]`
    /// Output value for QSPI SD0 (MOSI), SD1 (MISO), SD2 and SD3 pins
    #[inline(always)]
    pub const fn with_qspi_sd(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GpioHiOut::QSPI_SD_MASK) | ((value << GpioHiOut::QSPI_SD_OFFSET) & GpioHiOut::QSPI_SD_MASK)
        }
    }

    /// Set **QSPI_CSN** `[27:27]`
    /// Output value for QSPI CSn pin
    #[inline(always)]
    pub const fn with_qspi_csn(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | GpioHiOut::QSPI_CSN_MASK
            } else {
                self.bits & !GpioHiOut::QSPI_CSN_MASK
            }
        }
    }

    /// Set **QSPI_SCK** `[26:26]`
    /// Output value for QSPI SCK pin
    #[inline(always)]
    pub const fn with_qspi_sck(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | GpioHiOut::QSPI_SCK_MASK
            } else {
                self.bits & !GpioHiOut::QSPI_SCK_MASK
            }
        }
    }

    /// Set **USB_DM** `[25:25]`
    /// Output value for USB D- pin
    #[inline(always)]
    pub const fn with_usb_dm(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | GpioHiOut::USB_DM_MASK
            } else {
                self.bits & !GpioHiOut::USB_DM_MASK
            }
        }
    }

    /// Set **USB_DP** `[24:24]`
    /// Output value for USB D+ pin
    #[inline(always)]
    pub const fn with_usb_dp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | GpioHiOut::USB_DP_MASK
            } else {
                self.bits & !GpioHiOut::USB_DP_MASK
            }
        }
    }

    /// Set **GPIO** `[15:0]`
    /// Output value for GPIO32...47
    #[inline(always)]
    pub const fn with_gpio(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GpioHiOut::GPIO_MASK) | ((value << GpioHiOut::GPIO_OFFSET) & GpioHiOut::GPIO_MASK)
        }
    }

}

impl Default for GpioHiOutVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl GpioHiOut {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> GpioHiOutVal {
        GpioHiOutVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: GpioHiOutVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(GpioHiOutVal) -> GpioHiOutVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// GPIO_OUT_SET Register
///
/// GPIO0...31 output value set
///
/// Address offset: 0x0018
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **GPIO_OUT_SET** `[31:0]` (WO)
///   Perform an atomic bit-set on GPIO_OUT, i.e. `GPIO_OUT |= wdata`
#[repr(transparent)]
pub struct GpioOutSet {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for GpioOutSet {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl GpioOutSet {
    // ======== Field Constants ========

    /// **GPIO_OUT_SET**
    /// Perform an atomic bit-set on GPIO_OUT, i.e. `GPIO_OUT |= wdata`
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const GPIO_OUT_SET_OFFSET: u32 = 0;
    pub const GPIO_OUT_SET_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **GPIO_OUT_SET** `[31:0]` (WO)
    /// Perform an atomic bit-set on GPIO_OUT, i.e. `GPIO_OUT |= wdata`
    /// Write **GPIO_OUT_SET** `[31:0]` (WO)
    /// Perform an atomic bit-set on GPIO_OUT, i.e. `GPIO_OUT |= wdata`
    #[inline(always)]
    pub fn write_gpio_out_set(&self, value: u32) {
        self.value.write_bits((value << Self::GPIO_OUT_SET_OFFSET) & Self::GPIO_OUT_SET_MASK);
    }

}

/// GPIO_OUT_SET value type for batch read/modify/write operations
///
/// Use `gpiooutset.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpiooutset.write_val(val)`.
#[derive(Clone, Copy)]
pub struct GpioOutSetVal {
    bits: u32,
}

impl GpioOutSetVal {
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

    /// Set **GPIO_OUT_SET** `[31:0]`
    /// Perform an atomic bit-set on GPIO_OUT, i.e. `GPIO_OUT |= wdata`
    #[inline(always)]
    pub const fn with_gpio_out_set(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GpioOutSet::GPIO_OUT_SET_MASK) | ((value << GpioOutSet::GPIO_OUT_SET_OFFSET) & GpioOutSet::GPIO_OUT_SET_MASK)
        }
    }

}

impl Default for GpioOutSetVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl GpioOutSet {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> GpioOutSetVal {
        GpioOutSetVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: GpioOutSetVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(GpioOutSetVal) -> GpioOutSetVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// GPIO_HI_OUT_SET Register
///
/// Output value set for GPIO32..47, QSPI IOs and USB pins.
/// Perform an atomic bit-set on GPIO_HI_OUT, i.e. `GPIO_HI_OUT |= wdata`
///
/// Address offset: 0x001c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **QSPI_SD** `[31:28]` (WO)
/// - **QSPI_CSN** `[27]` (WO)
/// - **QSPI_SCK** `[26]` (WO)
/// - **USB_DM** `[25]` (WO)
/// - **USB_DP** `[24]` (WO)
/// - **GPIO** `[15:0]` (WO)
#[repr(transparent)]
pub struct GpioHiOutSet {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for GpioHiOutSet {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl GpioHiOutSet {
    // ======== Field Constants ========

    /// **QSPI_SD**
    /// Bits `[31:28]`, mask `0xf0000000`
    pub const QSPI_SD_OFFSET: u32 = 28;
    pub const QSPI_SD_MASK: u32 = 0xf0000000;

    /// **QSPI_CSN**
    /// Bits `[27:27]`, mask `0x08000000`
    pub const QSPI_CSN_OFFSET: u32 = 27;
    pub const QSPI_CSN_MASK: u32 = 0x08000000;
    pub const QSPI_CSN: u32 = 1 << 27;

    /// **QSPI_SCK**
    /// Bits `[26:26]`, mask `0x04000000`
    pub const QSPI_SCK_OFFSET: u32 = 26;
    pub const QSPI_SCK_MASK: u32 = 0x04000000;
    pub const QSPI_SCK: u32 = 1 << 26;

    /// **USB_DM**
    /// Bits `[25:25]`, mask `0x02000000`
    pub const USB_DM_OFFSET: u32 = 25;
    pub const USB_DM_MASK: u32 = 0x02000000;
    pub const USB_DM: u32 = 1 << 25;

    /// **USB_DP**
    /// Bits `[24:24]`, mask `0x01000000`
    pub const USB_DP_OFFSET: u32 = 24;
    pub const USB_DP_MASK: u32 = 0x01000000;
    pub const USB_DP: u32 = 1 << 24;

    /// **GPIO**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const GPIO_OFFSET: u32 = 0;
    pub const GPIO_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **QSPI_SD** `[31:28]` (WO)
    /// Write **QSPI_SD** `[31:28]` (WO)
    #[inline(always)]
    pub fn write_qspi_sd(&self, value: u32) {
        self.value.write_bits((value << Self::QSPI_SD_OFFSET) & Self::QSPI_SD_MASK);
    }

    /// **QSPI_CSN** `[27]` (WO)
    /// Trigger **QSPI_CSN** (WO)
    #[inline(always)]
    pub fn trigger_qspi_csn(&self) {
        self.value.write_bits(Self::QSPI_CSN_MASK);
    }

    /// **QSPI_SCK** `[26]` (WO)
    /// Trigger **QSPI_SCK** (WO)
    #[inline(always)]
    pub fn trigger_qspi_sck(&self) {
        self.value.write_bits(Self::QSPI_SCK_MASK);
    }

    /// **USB_DM** `[25]` (WO)
    /// Trigger **USB_DM** (WO)
    #[inline(always)]
    pub fn trigger_usb_dm(&self) {
        self.value.write_bits(Self::USB_DM_MASK);
    }

    /// **USB_DP** `[24]` (WO)
    /// Trigger **USB_DP** (WO)
    #[inline(always)]
    pub fn trigger_usb_dp(&self) {
        self.value.write_bits(Self::USB_DP_MASK);
    }

    /// **GPIO** `[15:0]` (WO)
    /// Write **GPIO** `[15:0]` (WO)
    #[inline(always)]
    pub fn write_gpio(&self, value: u32) {
        self.value.write_bits((value << Self::GPIO_OFFSET) & Self::GPIO_MASK);
    }

}

/// GPIO_HI_OUT_SET value type for batch read/modify/write operations
///
/// Use `gpiohioutset.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpiohioutset.write_val(val)`.
#[derive(Clone, Copy)]
pub struct GpioHiOutSetVal {
    bits: u32,
}

impl GpioHiOutSetVal {
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

    /// Set **QSPI_SD** `[31:28]`
    #[inline(always)]
    pub const fn with_qspi_sd(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GpioHiOutSet::QSPI_SD_MASK) | ((value << GpioHiOutSet::QSPI_SD_OFFSET) & GpioHiOutSet::QSPI_SD_MASK)
        }
    }

    /// Set **QSPI_CSN** `[27:27]`
    #[inline(always)]
    pub const fn with_qspi_csn(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | GpioHiOutSet::QSPI_CSN_MASK
            } else {
                self.bits & !GpioHiOutSet::QSPI_CSN_MASK
            }
        }
    }

    /// Set **QSPI_SCK** `[26:26]`
    #[inline(always)]
    pub const fn with_qspi_sck(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | GpioHiOutSet::QSPI_SCK_MASK
            } else {
                self.bits & !GpioHiOutSet::QSPI_SCK_MASK
            }
        }
    }

    /// Set **USB_DM** `[25:25]`
    #[inline(always)]
    pub const fn with_usb_dm(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | GpioHiOutSet::USB_DM_MASK
            } else {
                self.bits & !GpioHiOutSet::USB_DM_MASK
            }
        }
    }

    /// Set **USB_DP** `[24:24]`
    #[inline(always)]
    pub const fn with_usb_dp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | GpioHiOutSet::USB_DP_MASK
            } else {
                self.bits & !GpioHiOutSet::USB_DP_MASK
            }
        }
    }

    /// Set **GPIO** `[15:0]`
    #[inline(always)]
    pub const fn with_gpio(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GpioHiOutSet::GPIO_MASK) | ((value << GpioHiOutSet::GPIO_OFFSET) & GpioHiOutSet::GPIO_MASK)
        }
    }

}

impl Default for GpioHiOutSetVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl GpioHiOutSet {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> GpioHiOutSetVal {
        GpioHiOutSetVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: GpioHiOutSetVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(GpioHiOutSetVal) -> GpioHiOutSetVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// GPIO_OUT_CLR Register
///
/// GPIO0...31 output value clear
///
/// Address offset: 0x0020
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **GPIO_OUT_CLR** `[31:0]` (WO)
///   Perform an atomic bit-clear on GPIO_OUT, i.e. `GPIO_OUT &= ~wdata`
#[repr(transparent)]
pub struct GpioOutClr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for GpioOutClr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl GpioOutClr {
    // ======== Field Constants ========

    /// **GPIO_OUT_CLR**
    /// Perform an atomic bit-clear on GPIO_OUT, i.e. `GPIO_OUT &= ~wdata`
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const GPIO_OUT_CLR_OFFSET: u32 = 0;
    pub const GPIO_OUT_CLR_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **GPIO_OUT_CLR** `[31:0]` (WO)
    /// Perform an atomic bit-clear on GPIO_OUT, i.e. `GPIO_OUT &= ~wdata`
    /// Write **GPIO_OUT_CLR** `[31:0]` (WO)
    /// Perform an atomic bit-clear on GPIO_OUT, i.e. `GPIO_OUT &= ~wdata`
    #[inline(always)]
    pub fn write_gpio_out_clr(&self, value: u32) {
        self.value.write_bits((value << Self::GPIO_OUT_CLR_OFFSET) & Self::GPIO_OUT_CLR_MASK);
    }

}

/// GPIO_OUT_CLR value type for batch read/modify/write operations
///
/// Use `gpiooutclr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpiooutclr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct GpioOutClrVal {
    bits: u32,
}

impl GpioOutClrVal {
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

    /// Set **GPIO_OUT_CLR** `[31:0]`
    /// Perform an atomic bit-clear on GPIO_OUT, i.e. `GPIO_OUT &= ~wdata`
    #[inline(always)]
    pub const fn with_gpio_out_clr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GpioOutClr::GPIO_OUT_CLR_MASK) | ((value << GpioOutClr::GPIO_OUT_CLR_OFFSET) & GpioOutClr::GPIO_OUT_CLR_MASK)
        }
    }

}

impl Default for GpioOutClrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl GpioOutClr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> GpioOutClrVal {
        GpioOutClrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: GpioOutClrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(GpioOutClrVal) -> GpioOutClrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// GPIO_HI_OUT_CLR Register
///
/// Output value clear for GPIO32..47, QSPI IOs and USB pins.
/// Perform an atomic bit-clear on GPIO_HI_OUT, i.e. `GPIO_HI_OUT &= ~wdata`
///
/// Address offset: 0x0024
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **QSPI_SD** `[31:28]` (WO)
/// - **QSPI_CSN** `[27]` (WO)
/// - **QSPI_SCK** `[26]` (WO)
/// - **USB_DM** `[25]` (WO)
/// - **USB_DP** `[24]` (WO)
/// - **GPIO** `[15:0]` (WO)
#[repr(transparent)]
pub struct GpioHiOutClr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for GpioHiOutClr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl GpioHiOutClr {
    // ======== Field Constants ========

    /// **QSPI_SD**
    /// Bits `[31:28]`, mask `0xf0000000`
    pub const QSPI_SD_OFFSET: u32 = 28;
    pub const QSPI_SD_MASK: u32 = 0xf0000000;

    /// **QSPI_CSN**
    /// Bits `[27:27]`, mask `0x08000000`
    pub const QSPI_CSN_OFFSET: u32 = 27;
    pub const QSPI_CSN_MASK: u32 = 0x08000000;
    pub const QSPI_CSN: u32 = 1 << 27;

    /// **QSPI_SCK**
    /// Bits `[26:26]`, mask `0x04000000`
    pub const QSPI_SCK_OFFSET: u32 = 26;
    pub const QSPI_SCK_MASK: u32 = 0x04000000;
    pub const QSPI_SCK: u32 = 1 << 26;

    /// **USB_DM**
    /// Bits `[25:25]`, mask `0x02000000`
    pub const USB_DM_OFFSET: u32 = 25;
    pub const USB_DM_MASK: u32 = 0x02000000;
    pub const USB_DM: u32 = 1 << 25;

    /// **USB_DP**
    /// Bits `[24:24]`, mask `0x01000000`
    pub const USB_DP_OFFSET: u32 = 24;
    pub const USB_DP_MASK: u32 = 0x01000000;
    pub const USB_DP: u32 = 1 << 24;

    /// **GPIO**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const GPIO_OFFSET: u32 = 0;
    pub const GPIO_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **QSPI_SD** `[31:28]` (WO)
    /// Write **QSPI_SD** `[31:28]` (WO)
    #[inline(always)]
    pub fn write_qspi_sd(&self, value: u32) {
        self.value.write_bits((value << Self::QSPI_SD_OFFSET) & Self::QSPI_SD_MASK);
    }

    /// **QSPI_CSN** `[27]` (WO)
    /// Trigger **QSPI_CSN** (WO)
    #[inline(always)]
    pub fn trigger_qspi_csn(&self) {
        self.value.write_bits(Self::QSPI_CSN_MASK);
    }

    /// **QSPI_SCK** `[26]` (WO)
    /// Trigger **QSPI_SCK** (WO)
    #[inline(always)]
    pub fn trigger_qspi_sck(&self) {
        self.value.write_bits(Self::QSPI_SCK_MASK);
    }

    /// **USB_DM** `[25]` (WO)
    /// Trigger **USB_DM** (WO)
    #[inline(always)]
    pub fn trigger_usb_dm(&self) {
        self.value.write_bits(Self::USB_DM_MASK);
    }

    /// **USB_DP** `[24]` (WO)
    /// Trigger **USB_DP** (WO)
    #[inline(always)]
    pub fn trigger_usb_dp(&self) {
        self.value.write_bits(Self::USB_DP_MASK);
    }

    /// **GPIO** `[15:0]` (WO)
    /// Write **GPIO** `[15:0]` (WO)
    #[inline(always)]
    pub fn write_gpio(&self, value: u32) {
        self.value.write_bits((value << Self::GPIO_OFFSET) & Self::GPIO_MASK);
    }

}

/// GPIO_HI_OUT_CLR value type for batch read/modify/write operations
///
/// Use `gpiohioutclr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpiohioutclr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct GpioHiOutClrVal {
    bits: u32,
}

impl GpioHiOutClrVal {
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

    /// Set **QSPI_SD** `[31:28]`
    #[inline(always)]
    pub const fn with_qspi_sd(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GpioHiOutClr::QSPI_SD_MASK) | ((value << GpioHiOutClr::QSPI_SD_OFFSET) & GpioHiOutClr::QSPI_SD_MASK)
        }
    }

    /// Set **QSPI_CSN** `[27:27]`
    #[inline(always)]
    pub const fn with_qspi_csn(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | GpioHiOutClr::QSPI_CSN_MASK
            } else {
                self.bits & !GpioHiOutClr::QSPI_CSN_MASK
            }
        }
    }

    /// Set **QSPI_SCK** `[26:26]`
    #[inline(always)]
    pub const fn with_qspi_sck(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | GpioHiOutClr::QSPI_SCK_MASK
            } else {
                self.bits & !GpioHiOutClr::QSPI_SCK_MASK
            }
        }
    }

    /// Set **USB_DM** `[25:25]`
    #[inline(always)]
    pub const fn with_usb_dm(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | GpioHiOutClr::USB_DM_MASK
            } else {
                self.bits & !GpioHiOutClr::USB_DM_MASK
            }
        }
    }

    /// Set **USB_DP** `[24:24]`
    #[inline(always)]
    pub const fn with_usb_dp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | GpioHiOutClr::USB_DP_MASK
            } else {
                self.bits & !GpioHiOutClr::USB_DP_MASK
            }
        }
    }

    /// Set **GPIO** `[15:0]`
    #[inline(always)]
    pub const fn with_gpio(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GpioHiOutClr::GPIO_MASK) | ((value << GpioHiOutClr::GPIO_OFFSET) & GpioHiOutClr::GPIO_MASK)
        }
    }

}

impl Default for GpioHiOutClrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl GpioHiOutClr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> GpioHiOutClrVal {
        GpioHiOutClrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: GpioHiOutClrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(GpioHiOutClrVal) -> GpioHiOutClrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// GPIO_OUT_XOR Register
///
/// GPIO0...31 output value XOR
///
/// Address offset: 0x0028
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **GPIO_OUT_XOR** `[31:0]` (WO)
///   Perform an atomic bitwise XOR on GPIO_OUT, i.e. `GPIO_OUT ^= wdata`
#[repr(transparent)]
pub struct GpioOutXor {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for GpioOutXor {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl GpioOutXor {
    // ======== Field Constants ========

    /// **GPIO_OUT_XOR**
    /// Perform an atomic bitwise XOR on GPIO_OUT, i.e. `GPIO_OUT ^= wdata`
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const GPIO_OUT_XOR_OFFSET: u32 = 0;
    pub const GPIO_OUT_XOR_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **GPIO_OUT_XOR** `[31:0]` (WO)
    /// Perform an atomic bitwise XOR on GPIO_OUT, i.e. `GPIO_OUT ^= wdata`
    /// Write **GPIO_OUT_XOR** `[31:0]` (WO)
    /// Perform an atomic bitwise XOR on GPIO_OUT, i.e. `GPIO_OUT ^= wdata`
    #[inline(always)]
    pub fn write_gpio_out_xor(&self, value: u32) {
        self.value.write_bits((value << Self::GPIO_OUT_XOR_OFFSET) & Self::GPIO_OUT_XOR_MASK);
    }

}

/// GPIO_OUT_XOR value type for batch read/modify/write operations
///
/// Use `gpiooutxor.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpiooutxor.write_val(val)`.
#[derive(Clone, Copy)]
pub struct GpioOutXorVal {
    bits: u32,
}

impl GpioOutXorVal {
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

    /// Set **GPIO_OUT_XOR** `[31:0]`
    /// Perform an atomic bitwise XOR on GPIO_OUT, i.e. `GPIO_OUT ^= wdata`
    #[inline(always)]
    pub const fn with_gpio_out_xor(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GpioOutXor::GPIO_OUT_XOR_MASK) | ((value << GpioOutXor::GPIO_OUT_XOR_OFFSET) & GpioOutXor::GPIO_OUT_XOR_MASK)
        }
    }

}

impl Default for GpioOutXorVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl GpioOutXor {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> GpioOutXorVal {
        GpioOutXorVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: GpioOutXorVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(GpioOutXorVal) -> GpioOutXorVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// GPIO_HI_OUT_XOR Register
///
/// Output value XOR for GPIO32..47, QSPI IOs and USB pins.
/// Perform an atomic bitwise XOR on GPIO_HI_OUT, i.e. `GPIO_HI_OUT ^= wdata`
///
/// Address offset: 0x002c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **QSPI_SD** `[31:28]` (WO)
/// - **QSPI_CSN** `[27]` (WO)
/// - **QSPI_SCK** `[26]` (WO)
/// - **USB_DM** `[25]` (WO)
/// - **USB_DP** `[24]` (WO)
/// - **GPIO** `[15:0]` (WO)
#[repr(transparent)]
pub struct GpioHiOutXor {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for GpioHiOutXor {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl GpioHiOutXor {
    // ======== Field Constants ========

    /// **QSPI_SD**
    /// Bits `[31:28]`, mask `0xf0000000`
    pub const QSPI_SD_OFFSET: u32 = 28;
    pub const QSPI_SD_MASK: u32 = 0xf0000000;

    /// **QSPI_CSN**
    /// Bits `[27:27]`, mask `0x08000000`
    pub const QSPI_CSN_OFFSET: u32 = 27;
    pub const QSPI_CSN_MASK: u32 = 0x08000000;
    pub const QSPI_CSN: u32 = 1 << 27;

    /// **QSPI_SCK**
    /// Bits `[26:26]`, mask `0x04000000`
    pub const QSPI_SCK_OFFSET: u32 = 26;
    pub const QSPI_SCK_MASK: u32 = 0x04000000;
    pub const QSPI_SCK: u32 = 1 << 26;

    /// **USB_DM**
    /// Bits `[25:25]`, mask `0x02000000`
    pub const USB_DM_OFFSET: u32 = 25;
    pub const USB_DM_MASK: u32 = 0x02000000;
    pub const USB_DM: u32 = 1 << 25;

    /// **USB_DP**
    /// Bits `[24:24]`, mask `0x01000000`
    pub const USB_DP_OFFSET: u32 = 24;
    pub const USB_DP_MASK: u32 = 0x01000000;
    pub const USB_DP: u32 = 1 << 24;

    /// **GPIO**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const GPIO_OFFSET: u32 = 0;
    pub const GPIO_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **QSPI_SD** `[31:28]` (WO)
    /// Write **QSPI_SD** `[31:28]` (WO)
    #[inline(always)]
    pub fn write_qspi_sd(&self, value: u32) {
        self.value.write_bits((value << Self::QSPI_SD_OFFSET) & Self::QSPI_SD_MASK);
    }

    /// **QSPI_CSN** `[27]` (WO)
    /// Trigger **QSPI_CSN** (WO)
    #[inline(always)]
    pub fn trigger_qspi_csn(&self) {
        self.value.write_bits(Self::QSPI_CSN_MASK);
    }

    /// **QSPI_SCK** `[26]` (WO)
    /// Trigger **QSPI_SCK** (WO)
    #[inline(always)]
    pub fn trigger_qspi_sck(&self) {
        self.value.write_bits(Self::QSPI_SCK_MASK);
    }

    /// **USB_DM** `[25]` (WO)
    /// Trigger **USB_DM** (WO)
    #[inline(always)]
    pub fn trigger_usb_dm(&self) {
        self.value.write_bits(Self::USB_DM_MASK);
    }

    /// **USB_DP** `[24]` (WO)
    /// Trigger **USB_DP** (WO)
    #[inline(always)]
    pub fn trigger_usb_dp(&self) {
        self.value.write_bits(Self::USB_DP_MASK);
    }

    /// **GPIO** `[15:0]` (WO)
    /// Write **GPIO** `[15:0]` (WO)
    #[inline(always)]
    pub fn write_gpio(&self, value: u32) {
        self.value.write_bits((value << Self::GPIO_OFFSET) & Self::GPIO_MASK);
    }

}

/// GPIO_HI_OUT_XOR value type for batch read/modify/write operations
///
/// Use `gpiohioutxor.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpiohioutxor.write_val(val)`.
#[derive(Clone, Copy)]
pub struct GpioHiOutXorVal {
    bits: u32,
}

impl GpioHiOutXorVal {
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

    /// Set **QSPI_SD** `[31:28]`
    #[inline(always)]
    pub const fn with_qspi_sd(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GpioHiOutXor::QSPI_SD_MASK) | ((value << GpioHiOutXor::QSPI_SD_OFFSET) & GpioHiOutXor::QSPI_SD_MASK)
        }
    }

    /// Set **QSPI_CSN** `[27:27]`
    #[inline(always)]
    pub const fn with_qspi_csn(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | GpioHiOutXor::QSPI_CSN_MASK
            } else {
                self.bits & !GpioHiOutXor::QSPI_CSN_MASK
            }
        }
    }

    /// Set **QSPI_SCK** `[26:26]`
    #[inline(always)]
    pub const fn with_qspi_sck(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | GpioHiOutXor::QSPI_SCK_MASK
            } else {
                self.bits & !GpioHiOutXor::QSPI_SCK_MASK
            }
        }
    }

    /// Set **USB_DM** `[25:25]`
    #[inline(always)]
    pub const fn with_usb_dm(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | GpioHiOutXor::USB_DM_MASK
            } else {
                self.bits & !GpioHiOutXor::USB_DM_MASK
            }
        }
    }

    /// Set **USB_DP** `[24:24]`
    #[inline(always)]
    pub const fn with_usb_dp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | GpioHiOutXor::USB_DP_MASK
            } else {
                self.bits & !GpioHiOutXor::USB_DP_MASK
            }
        }
    }

    /// Set **GPIO** `[15:0]`
    #[inline(always)]
    pub const fn with_gpio(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GpioHiOutXor::GPIO_MASK) | ((value << GpioHiOutXor::GPIO_OFFSET) & GpioHiOutXor::GPIO_MASK)
        }
    }

}

impl Default for GpioHiOutXorVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl GpioHiOutXor {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> GpioHiOutXorVal {
        GpioHiOutXorVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: GpioHiOutXorVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(GpioHiOutXorVal) -> GpioHiOutXorVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// GPIO_OE Register
///
/// GPIO0...31 output enable
///
/// Address offset: 0x0030
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **GPIO_OE** `[31:0]` (RW)
///   Set output enable (1/0 -> output/input) for GPIO0...31. Reading back gives the last value written.
///   If core 0 and core 1 both write to GPIO_OE simultaneously (or to a SET/CLR/XOR alias), the result is as though the write from core 0 took place first, and the write from core 1 was then applied to that intermediate result.
///   In the Non-secure SIO, Secure-only GPIOs (as per ACCESSCTRL) ignore writes, and their output status reads back as zero. This is also true for SET/CLR/XOR aliases of this register.
#[repr(transparent)]
pub struct GpioOe {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for GpioOe {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl GpioOe {
    // ======== Field Constants ========

    /// **GPIO_OE**
    /// Set output enable (1/0 -> output/input) for GPIO0...31. Reading back gives the last value written.
    /// If core 0 and core 1 both write to GPIO_OE simultaneously (or to a SET/CLR/XOR alias), the result is as though the write from core 0 took place first, and the write from core 1 was then applied to that intermediate result.
    /// In the Non-secure SIO, Secure-only GPIOs (as per ACCESSCTRL) ignore writes, and their output status reads back as zero. This is also true for SET/CLR/XOR aliases of this register.
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const GPIO_OE_OFFSET: u32 = 0;
    pub const GPIO_OE_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **GPIO_OE** `[31:0]` (RW)
    /// Set output enable (1/0 -> output/input) for GPIO0...31. Reading back gives the last value written.
    /// If core 0 and core 1 both write to GPIO_OE simultaneously (or to a SET/CLR/XOR alias), the result is as though the write from core 0 took place first, and the write from core 1 was then applied to that intermediate result.
    /// In the Non-secure SIO, Secure-only GPIOs (as per ACCESSCTRL) ignore writes, and their output status reads back as zero. This is also true for SET/CLR/XOR aliases of this register.
    #[inline(always)]
    pub fn gpio_oe(&self) -> u32 {
        (self.value.read_bits() & Self::GPIO_OE_MASK) >> Self::GPIO_OE_OFFSET
    }

    /// Write **GPIO_OE** `[31:0]` (RW)
    /// Set output enable (1/0 -> output/input) for GPIO0...31. Reading back gives the last value written.
    /// If core 0 and core 1 both write to GPIO_OE simultaneously (or to a SET/CLR/XOR alias), the result is as though the write from core 0 took place first, and the write from core 1 was then applied to that intermediate result.
    /// In the Non-secure SIO, Secure-only GPIOs (as per ACCESSCTRL) ignore writes, and their output status reads back as zero. This is also true for SET/CLR/XOR aliases of this register.
    #[inline(always)]
    pub fn write_gpio_oe(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::GPIO_OE_MASK) | ((value << Self::GPIO_OE_OFFSET) & Self::GPIO_OE_MASK));
    }

}

/// GPIO_OE value type for batch read/modify/write operations
///
/// Use `gpiooe.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpiooe.write_val(val)`.
#[derive(Clone, Copy)]
pub struct GpioOeVal {
    bits: u32,
}

impl GpioOeVal {
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

    /// Read **GPIO_OE** `[31:0]`
    /// Set output enable (1/0 -> output/input) for GPIO0...31. Reading back gives the last value written.
    /// If core 0 and core 1 both write to GPIO_OE simultaneously (or to a SET/CLR/XOR alias), the result is as though the write from core 0 took place first, and the write from core 1 was then applied to that intermediate result.
    /// In the Non-secure SIO, Secure-only GPIOs (as per ACCESSCTRL) ignore writes, and their output status reads back as zero. This is also true for SET/CLR/XOR aliases of this register.
    #[inline(always)]
    pub const fn gpio_oe(&self) -> u32 {
        (self.bits & GpioOe::GPIO_OE_MASK) >> GpioOe::GPIO_OE_OFFSET
    }

    /// Set **GPIO_OE** `[31:0]`
    /// Set output enable (1/0 -> output/input) for GPIO0...31. Reading back gives the last value written.
    /// If core 0 and core 1 both write to GPIO_OE simultaneously (or to a SET/CLR/XOR alias), the result is as though the write from core 0 took place first, and the write from core 1 was then applied to that intermediate result.
    /// In the Non-secure SIO, Secure-only GPIOs (as per ACCESSCTRL) ignore writes, and their output status reads back as zero. This is also true for SET/CLR/XOR aliases of this register.
    #[inline(always)]
    pub const fn with_gpio_oe(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GpioOe::GPIO_OE_MASK) | ((value << GpioOe::GPIO_OE_OFFSET) & GpioOe::GPIO_OE_MASK)
        }
    }

}

impl Default for GpioOeVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl GpioOe {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> GpioOeVal {
        GpioOeVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: GpioOeVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(GpioOeVal) -> GpioOeVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// GPIO_HI_OE Register
///
/// Output enable value for GPIO32...47, QSPI IOs and USB pins.
/// Write output enable (1/0 -> output/input). Reading back gives the last value written. If core 0 and core 1 both write to GPIO_HI_OE simultaneously (or to a SET/CLR/XOR alias), the result is as though the write from core 0 took place first, and the write from core 1 was then applied to that intermediate result.
/// In the Non-secure SIO, Secure-only GPIOs (as per ACCESSCTRL) ignore writes, and their output status reads back as zero. This is also true for SET/CLR/XOR aliases of this register.
///
/// Address offset: 0x0034
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **QSPI_SD** `[31:28]` (RW)
///   Output enable value for QSPI SD0 (MOSI), SD1 (MISO), SD2 and SD3 pins
/// - **QSPI_CSN** `[27]` (RW)
///   Output enable value for QSPI CSn pin
/// - **QSPI_SCK** `[26]` (RW)
///   Output enable value for QSPI SCK pin
/// - **USB_DM** `[25]` (RW)
///   Output enable value for USB D- pin
/// - **USB_DP** `[24]` (RW)
///   Output enable value for USB D+ pin
/// - **GPIO** `[15:0]` (RW)
///   Output enable value for GPIO32...47
#[repr(transparent)]
pub struct GpioHiOe {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for GpioHiOe {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl GpioHiOe {
    // ======== Field Constants ========

    /// **QSPI_SD**
    /// Output enable value for QSPI SD0 (MOSI), SD1 (MISO), SD2 and SD3 pins
    /// Bits `[31:28]`, mask `0xf0000000`
    pub const QSPI_SD_OFFSET: u32 = 28;
    pub const QSPI_SD_MASK: u32 = 0xf0000000;

    /// **QSPI_CSN**
    /// Output enable value for QSPI CSn pin
    /// Bits `[27:27]`, mask `0x08000000`
    pub const QSPI_CSN_OFFSET: u32 = 27;
    pub const QSPI_CSN_MASK: u32 = 0x08000000;
    pub const QSPI_CSN: u32 = 1 << 27;

    /// **QSPI_SCK**
    /// Output enable value for QSPI SCK pin
    /// Bits `[26:26]`, mask `0x04000000`
    pub const QSPI_SCK_OFFSET: u32 = 26;
    pub const QSPI_SCK_MASK: u32 = 0x04000000;
    pub const QSPI_SCK: u32 = 1 << 26;

    /// **USB_DM**
    /// Output enable value for USB D- pin
    /// Bits `[25:25]`, mask `0x02000000`
    pub const USB_DM_OFFSET: u32 = 25;
    pub const USB_DM_MASK: u32 = 0x02000000;
    pub const USB_DM: u32 = 1 << 25;

    /// **USB_DP**
    /// Output enable value for USB D+ pin
    /// Bits `[24:24]`, mask `0x01000000`
    pub const USB_DP_OFFSET: u32 = 24;
    pub const USB_DP_MASK: u32 = 0x01000000;
    pub const USB_DP: u32 = 1 << 24;

    /// **GPIO**
    /// Output enable value for GPIO32...47
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const GPIO_OFFSET: u32 = 0;
    pub const GPIO_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **QSPI_SD** `[31:28]` (RW)
    /// Output enable value for QSPI SD0 (MOSI), SD1 (MISO), SD2 and SD3 pins
    #[inline(always)]
    pub fn qspi_sd(&self) -> u32 {
        (self.value.read_bits() & Self::QSPI_SD_MASK) >> Self::QSPI_SD_OFFSET
    }

    /// Write **QSPI_SD** `[31:28]` (RW)
    /// Output enable value for QSPI SD0 (MOSI), SD1 (MISO), SD2 and SD3 pins
    #[inline(always)]
    pub fn write_qspi_sd(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::QSPI_SD_MASK) | ((value << Self::QSPI_SD_OFFSET) & Self::QSPI_SD_MASK));
    }

    /// **QSPI_CSN** `[27]` (RW)
    /// Output enable value for QSPI CSn pin
    #[inline(always)]
    pub fn qspi_csn(&self) -> bool {
        (self.value.read_bits() & Self::QSPI_CSN_MASK) != 0
    }

    /// Set QSPI_CSN bit atomically (RW)
    #[inline(always)]
    pub fn set_qspi_csn(&self) {
        self.value.set_bits(Self::QSPI_CSN_MASK);
    }

    /// Clear QSPI_CSN bit atomically (RW)
    #[inline(always)]
    pub fn clr_qspi_csn(&self) {
        self.value.clr_bits(Self::QSPI_CSN_MASK);
    }

    /// **QSPI_SCK** `[26]` (RW)
    /// Output enable value for QSPI SCK pin
    #[inline(always)]
    pub fn qspi_sck(&self) -> bool {
        (self.value.read_bits() & Self::QSPI_SCK_MASK) != 0
    }

    /// Set QSPI_SCK bit atomically (RW)
    #[inline(always)]
    pub fn set_qspi_sck(&self) {
        self.value.set_bits(Self::QSPI_SCK_MASK);
    }

    /// Clear QSPI_SCK bit atomically (RW)
    #[inline(always)]
    pub fn clr_qspi_sck(&self) {
        self.value.clr_bits(Self::QSPI_SCK_MASK);
    }

    /// **USB_DM** `[25]` (RW)
    /// Output enable value for USB D- pin
    #[inline(always)]
    pub fn usb_dm(&self) -> bool {
        (self.value.read_bits() & Self::USB_DM_MASK) != 0
    }

    /// Set USB_DM bit atomically (RW)
    #[inline(always)]
    pub fn set_usb_dm(&self) {
        self.value.set_bits(Self::USB_DM_MASK);
    }

    /// Clear USB_DM bit atomically (RW)
    #[inline(always)]
    pub fn clr_usb_dm(&self) {
        self.value.clr_bits(Self::USB_DM_MASK);
    }

    /// **USB_DP** `[24]` (RW)
    /// Output enable value for USB D+ pin
    #[inline(always)]
    pub fn usb_dp(&self) -> bool {
        (self.value.read_bits() & Self::USB_DP_MASK) != 0
    }

    /// Set USB_DP bit atomically (RW)
    #[inline(always)]
    pub fn set_usb_dp(&self) {
        self.value.set_bits(Self::USB_DP_MASK);
    }

    /// Clear USB_DP bit atomically (RW)
    #[inline(always)]
    pub fn clr_usb_dp(&self) {
        self.value.clr_bits(Self::USB_DP_MASK);
    }

    /// **GPIO** `[15:0]` (RW)
    /// Output enable value for GPIO32...47
    #[inline(always)]
    pub fn gpio(&self) -> u32 {
        (self.value.read_bits() & Self::GPIO_MASK) >> Self::GPIO_OFFSET
    }

    /// Write **GPIO** `[15:0]` (RW)
    /// Output enable value for GPIO32...47
    #[inline(always)]
    pub fn write_gpio(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::GPIO_MASK) | ((value << Self::GPIO_OFFSET) & Self::GPIO_MASK));
    }

}

/// GPIO_HI_OE value type for batch read/modify/write operations
///
/// Use `gpiohioe.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpiohioe.write_val(val)`.
#[derive(Clone, Copy)]
pub struct GpioHiOeVal {
    bits: u32,
}

impl GpioHiOeVal {
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

    /// Read **QSPI_SD** `[31:28]`
    /// Output enable value for QSPI SD0 (MOSI), SD1 (MISO), SD2 and SD3 pins
    #[inline(always)]
    pub const fn qspi_sd(&self) -> u32 {
        (self.bits & GpioHiOe::QSPI_SD_MASK) >> GpioHiOe::QSPI_SD_OFFSET
    }

    /// Read **QSPI_CSN** `[27:27]`
    /// Output enable value for QSPI CSn pin
    #[inline(always)]
    pub const fn qspi_csn(&self) -> bool {
        (self.bits & GpioHiOe::QSPI_CSN_MASK) != 0
    }

    /// Read **QSPI_SCK** `[26:26]`
    /// Output enable value for QSPI SCK pin
    #[inline(always)]
    pub const fn qspi_sck(&self) -> bool {
        (self.bits & GpioHiOe::QSPI_SCK_MASK) != 0
    }

    /// Read **USB_DM** `[25:25]`
    /// Output enable value for USB D- pin
    #[inline(always)]
    pub const fn usb_dm(&self) -> bool {
        (self.bits & GpioHiOe::USB_DM_MASK) != 0
    }

    /// Read **USB_DP** `[24:24]`
    /// Output enable value for USB D+ pin
    #[inline(always)]
    pub const fn usb_dp(&self) -> bool {
        (self.bits & GpioHiOe::USB_DP_MASK) != 0
    }

    /// Read **GPIO** `[15:0]`
    /// Output enable value for GPIO32...47
    #[inline(always)]
    pub const fn gpio(&self) -> u32 {
        (self.bits & GpioHiOe::GPIO_MASK) >> GpioHiOe::GPIO_OFFSET
    }

    /// Set **QSPI_SD** `[31:28]`
    /// Output enable value for QSPI SD0 (MOSI), SD1 (MISO), SD2 and SD3 pins
    #[inline(always)]
    pub const fn with_qspi_sd(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GpioHiOe::QSPI_SD_MASK) | ((value << GpioHiOe::QSPI_SD_OFFSET) & GpioHiOe::QSPI_SD_MASK)
        }
    }

    /// Set **QSPI_CSN** `[27:27]`
    /// Output enable value for QSPI CSn pin
    #[inline(always)]
    pub const fn with_qspi_csn(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | GpioHiOe::QSPI_CSN_MASK
            } else {
                self.bits & !GpioHiOe::QSPI_CSN_MASK
            }
        }
    }

    /// Set **QSPI_SCK** `[26:26]`
    /// Output enable value for QSPI SCK pin
    #[inline(always)]
    pub const fn with_qspi_sck(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | GpioHiOe::QSPI_SCK_MASK
            } else {
                self.bits & !GpioHiOe::QSPI_SCK_MASK
            }
        }
    }

    /// Set **USB_DM** `[25:25]`
    /// Output enable value for USB D- pin
    #[inline(always)]
    pub const fn with_usb_dm(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | GpioHiOe::USB_DM_MASK
            } else {
                self.bits & !GpioHiOe::USB_DM_MASK
            }
        }
    }

    /// Set **USB_DP** `[24:24]`
    /// Output enable value for USB D+ pin
    #[inline(always)]
    pub const fn with_usb_dp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | GpioHiOe::USB_DP_MASK
            } else {
                self.bits & !GpioHiOe::USB_DP_MASK
            }
        }
    }

    /// Set **GPIO** `[15:0]`
    /// Output enable value for GPIO32...47
    #[inline(always)]
    pub const fn with_gpio(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GpioHiOe::GPIO_MASK) | ((value << GpioHiOe::GPIO_OFFSET) & GpioHiOe::GPIO_MASK)
        }
    }

}

impl Default for GpioHiOeVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl GpioHiOe {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> GpioHiOeVal {
        GpioHiOeVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: GpioHiOeVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(GpioHiOeVal) -> GpioHiOeVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// GPIO_OE_SET Register
///
/// GPIO0...31 output enable set
///
/// Address offset: 0x0038
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **GPIO_OE_SET** `[31:0]` (WO)
///   Perform an atomic bit-set on GPIO_OE, i.e. `GPIO_OE |= wdata`
#[repr(transparent)]
pub struct GpioOeSet {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for GpioOeSet {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl GpioOeSet {
    // ======== Field Constants ========

    /// **GPIO_OE_SET**
    /// Perform an atomic bit-set on GPIO_OE, i.e. `GPIO_OE |= wdata`
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const GPIO_OE_SET_OFFSET: u32 = 0;
    pub const GPIO_OE_SET_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **GPIO_OE_SET** `[31:0]` (WO)
    /// Perform an atomic bit-set on GPIO_OE, i.e. `GPIO_OE |= wdata`
    /// Write **GPIO_OE_SET** `[31:0]` (WO)
    /// Perform an atomic bit-set on GPIO_OE, i.e. `GPIO_OE |= wdata`
    #[inline(always)]
    pub fn write_gpio_oe_set(&self, value: u32) {
        self.value.write_bits((value << Self::GPIO_OE_SET_OFFSET) & Self::GPIO_OE_SET_MASK);
    }

}

/// GPIO_OE_SET value type for batch read/modify/write operations
///
/// Use `gpiooeset.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpiooeset.write_val(val)`.
#[derive(Clone, Copy)]
pub struct GpioOeSetVal {
    bits: u32,
}

impl GpioOeSetVal {
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

    /// Set **GPIO_OE_SET** `[31:0]`
    /// Perform an atomic bit-set on GPIO_OE, i.e. `GPIO_OE |= wdata`
    #[inline(always)]
    pub const fn with_gpio_oe_set(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GpioOeSet::GPIO_OE_SET_MASK) | ((value << GpioOeSet::GPIO_OE_SET_OFFSET) & GpioOeSet::GPIO_OE_SET_MASK)
        }
    }

}

impl Default for GpioOeSetVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl GpioOeSet {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> GpioOeSetVal {
        GpioOeSetVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: GpioOeSetVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(GpioOeSetVal) -> GpioOeSetVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// GPIO_HI_OE_SET Register
///
/// Output enable set for GPIO32...47, QSPI IOs and USB pins.
/// Perform an atomic bit-set on GPIO_HI_OE, i.e. `GPIO_HI_OE |= wdata`
///
/// Address offset: 0x003c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **QSPI_SD** `[31:28]` (WO)
/// - **QSPI_CSN** `[27]` (WO)
/// - **QSPI_SCK** `[26]` (WO)
/// - **USB_DM** `[25]` (WO)
/// - **USB_DP** `[24]` (WO)
/// - **GPIO** `[15:0]` (WO)
#[repr(transparent)]
pub struct GpioHiOeSet {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for GpioHiOeSet {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl GpioHiOeSet {
    // ======== Field Constants ========

    /// **QSPI_SD**
    /// Bits `[31:28]`, mask `0xf0000000`
    pub const QSPI_SD_OFFSET: u32 = 28;
    pub const QSPI_SD_MASK: u32 = 0xf0000000;

    /// **QSPI_CSN**
    /// Bits `[27:27]`, mask `0x08000000`
    pub const QSPI_CSN_OFFSET: u32 = 27;
    pub const QSPI_CSN_MASK: u32 = 0x08000000;
    pub const QSPI_CSN: u32 = 1 << 27;

    /// **QSPI_SCK**
    /// Bits `[26:26]`, mask `0x04000000`
    pub const QSPI_SCK_OFFSET: u32 = 26;
    pub const QSPI_SCK_MASK: u32 = 0x04000000;
    pub const QSPI_SCK: u32 = 1 << 26;

    /// **USB_DM**
    /// Bits `[25:25]`, mask `0x02000000`
    pub const USB_DM_OFFSET: u32 = 25;
    pub const USB_DM_MASK: u32 = 0x02000000;
    pub const USB_DM: u32 = 1 << 25;

    /// **USB_DP**
    /// Bits `[24:24]`, mask `0x01000000`
    pub const USB_DP_OFFSET: u32 = 24;
    pub const USB_DP_MASK: u32 = 0x01000000;
    pub const USB_DP: u32 = 1 << 24;

    /// **GPIO**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const GPIO_OFFSET: u32 = 0;
    pub const GPIO_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **QSPI_SD** `[31:28]` (WO)
    /// Write **QSPI_SD** `[31:28]` (WO)
    #[inline(always)]
    pub fn write_qspi_sd(&self, value: u32) {
        self.value.write_bits((value << Self::QSPI_SD_OFFSET) & Self::QSPI_SD_MASK);
    }

    /// **QSPI_CSN** `[27]` (WO)
    /// Trigger **QSPI_CSN** (WO)
    #[inline(always)]
    pub fn trigger_qspi_csn(&self) {
        self.value.write_bits(Self::QSPI_CSN_MASK);
    }

    /// **QSPI_SCK** `[26]` (WO)
    /// Trigger **QSPI_SCK** (WO)
    #[inline(always)]
    pub fn trigger_qspi_sck(&self) {
        self.value.write_bits(Self::QSPI_SCK_MASK);
    }

    /// **USB_DM** `[25]` (WO)
    /// Trigger **USB_DM** (WO)
    #[inline(always)]
    pub fn trigger_usb_dm(&self) {
        self.value.write_bits(Self::USB_DM_MASK);
    }

    /// **USB_DP** `[24]` (WO)
    /// Trigger **USB_DP** (WO)
    #[inline(always)]
    pub fn trigger_usb_dp(&self) {
        self.value.write_bits(Self::USB_DP_MASK);
    }

    /// **GPIO** `[15:0]` (WO)
    /// Write **GPIO** `[15:0]` (WO)
    #[inline(always)]
    pub fn write_gpio(&self, value: u32) {
        self.value.write_bits((value << Self::GPIO_OFFSET) & Self::GPIO_MASK);
    }

}

/// GPIO_HI_OE_SET value type for batch read/modify/write operations
///
/// Use `gpiohioeset.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpiohioeset.write_val(val)`.
#[derive(Clone, Copy)]
pub struct GpioHiOeSetVal {
    bits: u32,
}

impl GpioHiOeSetVal {
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

    /// Set **QSPI_SD** `[31:28]`
    #[inline(always)]
    pub const fn with_qspi_sd(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GpioHiOeSet::QSPI_SD_MASK) | ((value << GpioHiOeSet::QSPI_SD_OFFSET) & GpioHiOeSet::QSPI_SD_MASK)
        }
    }

    /// Set **QSPI_CSN** `[27:27]`
    #[inline(always)]
    pub const fn with_qspi_csn(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | GpioHiOeSet::QSPI_CSN_MASK
            } else {
                self.bits & !GpioHiOeSet::QSPI_CSN_MASK
            }
        }
    }

    /// Set **QSPI_SCK** `[26:26]`
    #[inline(always)]
    pub const fn with_qspi_sck(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | GpioHiOeSet::QSPI_SCK_MASK
            } else {
                self.bits & !GpioHiOeSet::QSPI_SCK_MASK
            }
        }
    }

    /// Set **USB_DM** `[25:25]`
    #[inline(always)]
    pub const fn with_usb_dm(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | GpioHiOeSet::USB_DM_MASK
            } else {
                self.bits & !GpioHiOeSet::USB_DM_MASK
            }
        }
    }

    /// Set **USB_DP** `[24:24]`
    #[inline(always)]
    pub const fn with_usb_dp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | GpioHiOeSet::USB_DP_MASK
            } else {
                self.bits & !GpioHiOeSet::USB_DP_MASK
            }
        }
    }

    /// Set **GPIO** `[15:0]`
    #[inline(always)]
    pub const fn with_gpio(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GpioHiOeSet::GPIO_MASK) | ((value << GpioHiOeSet::GPIO_OFFSET) & GpioHiOeSet::GPIO_MASK)
        }
    }

}

impl Default for GpioHiOeSetVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl GpioHiOeSet {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> GpioHiOeSetVal {
        GpioHiOeSetVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: GpioHiOeSetVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(GpioHiOeSetVal) -> GpioHiOeSetVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// GPIO_OE_CLR Register
///
/// GPIO0...31 output enable clear
///
/// Address offset: 0x0040
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **GPIO_OE_CLR** `[31:0]` (WO)
///   Perform an atomic bit-clear on GPIO_OE, i.e. `GPIO_OE &= ~wdata`
#[repr(transparent)]
pub struct GpioOeClr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for GpioOeClr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl GpioOeClr {
    // ======== Field Constants ========

    /// **GPIO_OE_CLR**
    /// Perform an atomic bit-clear on GPIO_OE, i.e. `GPIO_OE &= ~wdata`
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const GPIO_OE_CLR_OFFSET: u32 = 0;
    pub const GPIO_OE_CLR_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **GPIO_OE_CLR** `[31:0]` (WO)
    /// Perform an atomic bit-clear on GPIO_OE, i.e. `GPIO_OE &= ~wdata`
    /// Write **GPIO_OE_CLR** `[31:0]` (WO)
    /// Perform an atomic bit-clear on GPIO_OE, i.e. `GPIO_OE &= ~wdata`
    #[inline(always)]
    pub fn write_gpio_oe_clr(&self, value: u32) {
        self.value.write_bits((value << Self::GPIO_OE_CLR_OFFSET) & Self::GPIO_OE_CLR_MASK);
    }

}

/// GPIO_OE_CLR value type for batch read/modify/write operations
///
/// Use `gpiooeclr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpiooeclr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct GpioOeClrVal {
    bits: u32,
}

impl GpioOeClrVal {
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

    /// Set **GPIO_OE_CLR** `[31:0]`
    /// Perform an atomic bit-clear on GPIO_OE, i.e. `GPIO_OE &= ~wdata`
    #[inline(always)]
    pub const fn with_gpio_oe_clr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GpioOeClr::GPIO_OE_CLR_MASK) | ((value << GpioOeClr::GPIO_OE_CLR_OFFSET) & GpioOeClr::GPIO_OE_CLR_MASK)
        }
    }

}

impl Default for GpioOeClrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl GpioOeClr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> GpioOeClrVal {
        GpioOeClrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: GpioOeClrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(GpioOeClrVal) -> GpioOeClrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// GPIO_HI_OE_CLR Register
///
/// Output enable clear for GPIO32...47, QSPI IOs and USB pins.
/// Perform an atomic bit-clear on GPIO_HI_OE, i.e. `GPIO_HI_OE &= ~wdata`
///
/// Address offset: 0x0044
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **QSPI_SD** `[31:28]` (WO)
/// - **QSPI_CSN** `[27]` (WO)
/// - **QSPI_SCK** `[26]` (WO)
/// - **USB_DM** `[25]` (WO)
/// - **USB_DP** `[24]` (WO)
/// - **GPIO** `[15:0]` (WO)
#[repr(transparent)]
pub struct GpioHiOeClr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for GpioHiOeClr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl GpioHiOeClr {
    // ======== Field Constants ========

    /// **QSPI_SD**
    /// Bits `[31:28]`, mask `0xf0000000`
    pub const QSPI_SD_OFFSET: u32 = 28;
    pub const QSPI_SD_MASK: u32 = 0xf0000000;

    /// **QSPI_CSN**
    /// Bits `[27:27]`, mask `0x08000000`
    pub const QSPI_CSN_OFFSET: u32 = 27;
    pub const QSPI_CSN_MASK: u32 = 0x08000000;
    pub const QSPI_CSN: u32 = 1 << 27;

    /// **QSPI_SCK**
    /// Bits `[26:26]`, mask `0x04000000`
    pub const QSPI_SCK_OFFSET: u32 = 26;
    pub const QSPI_SCK_MASK: u32 = 0x04000000;
    pub const QSPI_SCK: u32 = 1 << 26;

    /// **USB_DM**
    /// Bits `[25:25]`, mask `0x02000000`
    pub const USB_DM_OFFSET: u32 = 25;
    pub const USB_DM_MASK: u32 = 0x02000000;
    pub const USB_DM: u32 = 1 << 25;

    /// **USB_DP**
    /// Bits `[24:24]`, mask `0x01000000`
    pub const USB_DP_OFFSET: u32 = 24;
    pub const USB_DP_MASK: u32 = 0x01000000;
    pub const USB_DP: u32 = 1 << 24;

    /// **GPIO**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const GPIO_OFFSET: u32 = 0;
    pub const GPIO_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **QSPI_SD** `[31:28]` (WO)
    /// Write **QSPI_SD** `[31:28]` (WO)
    #[inline(always)]
    pub fn write_qspi_sd(&self, value: u32) {
        self.value.write_bits((value << Self::QSPI_SD_OFFSET) & Self::QSPI_SD_MASK);
    }

    /// **QSPI_CSN** `[27]` (WO)
    /// Trigger **QSPI_CSN** (WO)
    #[inline(always)]
    pub fn trigger_qspi_csn(&self) {
        self.value.write_bits(Self::QSPI_CSN_MASK);
    }

    /// **QSPI_SCK** `[26]` (WO)
    /// Trigger **QSPI_SCK** (WO)
    #[inline(always)]
    pub fn trigger_qspi_sck(&self) {
        self.value.write_bits(Self::QSPI_SCK_MASK);
    }

    /// **USB_DM** `[25]` (WO)
    /// Trigger **USB_DM** (WO)
    #[inline(always)]
    pub fn trigger_usb_dm(&self) {
        self.value.write_bits(Self::USB_DM_MASK);
    }

    /// **USB_DP** `[24]` (WO)
    /// Trigger **USB_DP** (WO)
    #[inline(always)]
    pub fn trigger_usb_dp(&self) {
        self.value.write_bits(Self::USB_DP_MASK);
    }

    /// **GPIO** `[15:0]` (WO)
    /// Write **GPIO** `[15:0]` (WO)
    #[inline(always)]
    pub fn write_gpio(&self, value: u32) {
        self.value.write_bits((value << Self::GPIO_OFFSET) & Self::GPIO_MASK);
    }

}

/// GPIO_HI_OE_CLR value type for batch read/modify/write operations
///
/// Use `gpiohioeclr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpiohioeclr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct GpioHiOeClrVal {
    bits: u32,
}

impl GpioHiOeClrVal {
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

    /// Set **QSPI_SD** `[31:28]`
    #[inline(always)]
    pub const fn with_qspi_sd(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GpioHiOeClr::QSPI_SD_MASK) | ((value << GpioHiOeClr::QSPI_SD_OFFSET) & GpioHiOeClr::QSPI_SD_MASK)
        }
    }

    /// Set **QSPI_CSN** `[27:27]`
    #[inline(always)]
    pub const fn with_qspi_csn(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | GpioHiOeClr::QSPI_CSN_MASK
            } else {
                self.bits & !GpioHiOeClr::QSPI_CSN_MASK
            }
        }
    }

    /// Set **QSPI_SCK** `[26:26]`
    #[inline(always)]
    pub const fn with_qspi_sck(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | GpioHiOeClr::QSPI_SCK_MASK
            } else {
                self.bits & !GpioHiOeClr::QSPI_SCK_MASK
            }
        }
    }

    /// Set **USB_DM** `[25:25]`
    #[inline(always)]
    pub const fn with_usb_dm(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | GpioHiOeClr::USB_DM_MASK
            } else {
                self.bits & !GpioHiOeClr::USB_DM_MASK
            }
        }
    }

    /// Set **USB_DP** `[24:24]`
    #[inline(always)]
    pub const fn with_usb_dp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | GpioHiOeClr::USB_DP_MASK
            } else {
                self.bits & !GpioHiOeClr::USB_DP_MASK
            }
        }
    }

    /// Set **GPIO** `[15:0]`
    #[inline(always)]
    pub const fn with_gpio(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GpioHiOeClr::GPIO_MASK) | ((value << GpioHiOeClr::GPIO_OFFSET) & GpioHiOeClr::GPIO_MASK)
        }
    }

}

impl Default for GpioHiOeClrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl GpioHiOeClr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> GpioHiOeClrVal {
        GpioHiOeClrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: GpioHiOeClrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(GpioHiOeClrVal) -> GpioHiOeClrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// GPIO_OE_XOR Register
///
/// GPIO0...31 output enable XOR
///
/// Address offset: 0x0048
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **GPIO_OE_XOR** `[31:0]` (WO)
///   Perform an atomic bitwise XOR on GPIO_OE, i.e. `GPIO_OE ^= wdata`
#[repr(transparent)]
pub struct GpioOeXor {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for GpioOeXor {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl GpioOeXor {
    // ======== Field Constants ========

    /// **GPIO_OE_XOR**
    /// Perform an atomic bitwise XOR on GPIO_OE, i.e. `GPIO_OE ^= wdata`
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const GPIO_OE_XOR_OFFSET: u32 = 0;
    pub const GPIO_OE_XOR_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **GPIO_OE_XOR** `[31:0]` (WO)
    /// Perform an atomic bitwise XOR on GPIO_OE, i.e. `GPIO_OE ^= wdata`
    /// Write **GPIO_OE_XOR** `[31:0]` (WO)
    /// Perform an atomic bitwise XOR on GPIO_OE, i.e. `GPIO_OE ^= wdata`
    #[inline(always)]
    pub fn write_gpio_oe_xor(&self, value: u32) {
        self.value.write_bits((value << Self::GPIO_OE_XOR_OFFSET) & Self::GPIO_OE_XOR_MASK);
    }

}

/// GPIO_OE_XOR value type for batch read/modify/write operations
///
/// Use `gpiooexor.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpiooexor.write_val(val)`.
#[derive(Clone, Copy)]
pub struct GpioOeXorVal {
    bits: u32,
}

impl GpioOeXorVal {
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

    /// Set **GPIO_OE_XOR** `[31:0]`
    /// Perform an atomic bitwise XOR on GPIO_OE, i.e. `GPIO_OE ^= wdata`
    #[inline(always)]
    pub const fn with_gpio_oe_xor(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GpioOeXor::GPIO_OE_XOR_MASK) | ((value << GpioOeXor::GPIO_OE_XOR_OFFSET) & GpioOeXor::GPIO_OE_XOR_MASK)
        }
    }

}

impl Default for GpioOeXorVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl GpioOeXor {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> GpioOeXorVal {
        GpioOeXorVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: GpioOeXorVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(GpioOeXorVal) -> GpioOeXorVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// GPIO_HI_OE_XOR Register
///
/// Output enable XOR for GPIO32...47, QSPI IOs and USB pins.
/// Perform an atomic bitwise XOR on GPIO_HI_OE, i.e. `GPIO_HI_OE ^= wdata`
///
/// Address offset: 0x004c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **QSPI_SD** `[31:28]` (WO)
/// - **QSPI_CSN** `[27]` (WO)
/// - **QSPI_SCK** `[26]` (WO)
/// - **USB_DM** `[25]` (WO)
/// - **USB_DP** `[24]` (WO)
/// - **GPIO** `[15:0]` (WO)
#[repr(transparent)]
pub struct GpioHiOeXor {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for GpioHiOeXor {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl GpioHiOeXor {
    // ======== Field Constants ========

    /// **QSPI_SD**
    /// Bits `[31:28]`, mask `0xf0000000`
    pub const QSPI_SD_OFFSET: u32 = 28;
    pub const QSPI_SD_MASK: u32 = 0xf0000000;

    /// **QSPI_CSN**
    /// Bits `[27:27]`, mask `0x08000000`
    pub const QSPI_CSN_OFFSET: u32 = 27;
    pub const QSPI_CSN_MASK: u32 = 0x08000000;
    pub const QSPI_CSN: u32 = 1 << 27;

    /// **QSPI_SCK**
    /// Bits `[26:26]`, mask `0x04000000`
    pub const QSPI_SCK_OFFSET: u32 = 26;
    pub const QSPI_SCK_MASK: u32 = 0x04000000;
    pub const QSPI_SCK: u32 = 1 << 26;

    /// **USB_DM**
    /// Bits `[25:25]`, mask `0x02000000`
    pub const USB_DM_OFFSET: u32 = 25;
    pub const USB_DM_MASK: u32 = 0x02000000;
    pub const USB_DM: u32 = 1 << 25;

    /// **USB_DP**
    /// Bits `[24:24]`, mask `0x01000000`
    pub const USB_DP_OFFSET: u32 = 24;
    pub const USB_DP_MASK: u32 = 0x01000000;
    pub const USB_DP: u32 = 1 << 24;

    /// **GPIO**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const GPIO_OFFSET: u32 = 0;
    pub const GPIO_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **QSPI_SD** `[31:28]` (WO)
    /// Write **QSPI_SD** `[31:28]` (WO)
    #[inline(always)]
    pub fn write_qspi_sd(&self, value: u32) {
        self.value.write_bits((value << Self::QSPI_SD_OFFSET) & Self::QSPI_SD_MASK);
    }

    /// **QSPI_CSN** `[27]` (WO)
    /// Trigger **QSPI_CSN** (WO)
    #[inline(always)]
    pub fn trigger_qspi_csn(&self) {
        self.value.write_bits(Self::QSPI_CSN_MASK);
    }

    /// **QSPI_SCK** `[26]` (WO)
    /// Trigger **QSPI_SCK** (WO)
    #[inline(always)]
    pub fn trigger_qspi_sck(&self) {
        self.value.write_bits(Self::QSPI_SCK_MASK);
    }

    /// **USB_DM** `[25]` (WO)
    /// Trigger **USB_DM** (WO)
    #[inline(always)]
    pub fn trigger_usb_dm(&self) {
        self.value.write_bits(Self::USB_DM_MASK);
    }

    /// **USB_DP** `[24]` (WO)
    /// Trigger **USB_DP** (WO)
    #[inline(always)]
    pub fn trigger_usb_dp(&self) {
        self.value.write_bits(Self::USB_DP_MASK);
    }

    /// **GPIO** `[15:0]` (WO)
    /// Write **GPIO** `[15:0]` (WO)
    #[inline(always)]
    pub fn write_gpio(&self, value: u32) {
        self.value.write_bits((value << Self::GPIO_OFFSET) & Self::GPIO_MASK);
    }

}

/// GPIO_HI_OE_XOR value type for batch read/modify/write operations
///
/// Use `gpiohioexor.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpiohioexor.write_val(val)`.
#[derive(Clone, Copy)]
pub struct GpioHiOeXorVal {
    bits: u32,
}

impl GpioHiOeXorVal {
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

    /// Set **QSPI_SD** `[31:28]`
    #[inline(always)]
    pub const fn with_qspi_sd(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GpioHiOeXor::QSPI_SD_MASK) | ((value << GpioHiOeXor::QSPI_SD_OFFSET) & GpioHiOeXor::QSPI_SD_MASK)
        }
    }

    /// Set **QSPI_CSN** `[27:27]`
    #[inline(always)]
    pub const fn with_qspi_csn(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | GpioHiOeXor::QSPI_CSN_MASK
            } else {
                self.bits & !GpioHiOeXor::QSPI_CSN_MASK
            }
        }
    }

    /// Set **QSPI_SCK** `[26:26]`
    #[inline(always)]
    pub const fn with_qspi_sck(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | GpioHiOeXor::QSPI_SCK_MASK
            } else {
                self.bits & !GpioHiOeXor::QSPI_SCK_MASK
            }
        }
    }

    /// Set **USB_DM** `[25:25]`
    #[inline(always)]
    pub const fn with_usb_dm(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | GpioHiOeXor::USB_DM_MASK
            } else {
                self.bits & !GpioHiOeXor::USB_DM_MASK
            }
        }
    }

    /// Set **USB_DP** `[24:24]`
    #[inline(always)]
    pub const fn with_usb_dp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | GpioHiOeXor::USB_DP_MASK
            } else {
                self.bits & !GpioHiOeXor::USB_DP_MASK
            }
        }
    }

    /// Set **GPIO** `[15:0]`
    #[inline(always)]
    pub const fn with_gpio(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GpioHiOeXor::GPIO_MASK) | ((value << GpioHiOeXor::GPIO_OFFSET) & GpioHiOeXor::GPIO_MASK)
        }
    }

}

impl Default for GpioHiOeXorVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl GpioHiOeXor {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> GpioHiOeXorVal {
        GpioHiOeXorVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: GpioHiOeXorVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(GpioHiOeXorVal) -> GpioHiOeXorVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// FIFO_ST Register
///
/// Status register for inter-core FIFOs (mailboxes).
/// There is one FIFO in the core 0 -> core 1 direction, and one core 1 -> core 0. Both are 32 bits wide and 8 words deep.
/// Core 0 can see the read side of the 1->0 FIFO (RX), and the write side of 0->1 FIFO (TX).
/// Core 1 can see the read side of the 0->1 FIFO (RX), and the write side of 1->0 FIFO (TX).
/// The SIO IRQ for each core is the logical OR of the VLD, WOF and ROE fields of its FIFO_ST register.
///
/// Address offset: 0x0050
/// Reset value: 0x00000002
///
/// ## Fields
///
/// - **ROE** `[3]` (RW)
///   Sticky flag indicating the RX FIFO was read when empty. This read was ignored by the FIFO.
/// - **WOF** `[2]` (RW)
///   Sticky flag indicating the TX FIFO was written when full. This write was ignored by the FIFO.
/// - **RDY** `[1]` (RO)
///   Value is 1 if this core's TX FIFO is not full (i.e. if FIFO_WR is ready for more data)
/// - **VLD** `[0]` (RO)
///   Value is 1 if this core's RX FIFO is not empty (i.e. if FIFO_RD is valid)
#[repr(transparent)]
pub struct FifoSt {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for FifoSt {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl FifoSt {
    // ======== Field Constants ========

    /// **ROE**
    /// Sticky flag indicating the RX FIFO was read when empty. This read was ignored by the FIFO.
    /// Bits `[3:3]`, mask `0x00000008`
    pub const ROE_OFFSET: u32 = 3;
    pub const ROE_MASK: u32 = 0x00000008;
    pub const ROE: u32 = 1 << 3;

    /// **WOF**
    /// Sticky flag indicating the TX FIFO was written when full. This write was ignored by the FIFO.
    /// Bits `[2:2]`, mask `0x00000004`
    pub const WOF_OFFSET: u32 = 2;
    pub const WOF_MASK: u32 = 0x00000004;
    pub const WOF: u32 = 1 << 2;

    /// **RDY**
    /// Value is 1 if this core's TX FIFO is not full (i.e. if FIFO_WR is ready for more data)
    /// Bits `[1:1]`, mask `0x00000002`
    pub const RDY_OFFSET: u32 = 1;
    pub const RDY_MASK: u32 = 0x00000002;
    pub const RDY: u32 = 1 << 1;

    /// **VLD**
    /// Value is 1 if this core's RX FIFO is not empty (i.e. if FIFO_RD is valid)
    /// Bits `[0:0]`, mask `0x00000001`
    pub const VLD_OFFSET: u32 = 0;
    pub const VLD_MASK: u32 = 0x00000001;
    pub const VLD: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ROE** `[3]` (RW)
    /// Sticky flag indicating the RX FIFO was read when empty. This read was ignored by the FIFO.
    #[inline(always)]
    pub fn roe(&self) -> bool {
        (self.value.read_bits() & Self::ROE_MASK) != 0
    }

    /// Set ROE bit atomically (RW)
    #[inline(always)]
    pub fn set_roe(&self) {
        self.value.set_bits(Self::ROE_MASK);
    }

    /// Clear ROE bit atomically (RW)
    #[inline(always)]
    pub fn clr_roe(&self) {
        self.value.clr_bits(Self::ROE_MASK);
    }

    /// **WOF** `[2]` (RW)
    /// Sticky flag indicating the TX FIFO was written when full. This write was ignored by the FIFO.
    #[inline(always)]
    pub fn wof(&self) -> bool {
        (self.value.read_bits() & Self::WOF_MASK) != 0
    }

    /// Set WOF bit atomically (RW)
    #[inline(always)]
    pub fn set_wof(&self) {
        self.value.set_bits(Self::WOF_MASK);
    }

    /// Clear WOF bit atomically (RW)
    #[inline(always)]
    pub fn clr_wof(&self) {
        self.value.clr_bits(Self::WOF_MASK);
    }

    /// **RDY** `[1]` (RO)
    /// Value is 1 if this core's TX FIFO is not full (i.e. if FIFO_WR is ready for more data)
    #[inline(always)]
    pub fn rdy(&self) -> bool {
        (self.value.read_bits() & Self::RDY_MASK) != 0
    }

    /// **VLD** `[0]` (RO)
    /// Value is 1 if this core's RX FIFO is not empty (i.e. if FIFO_RD is valid)
    #[inline(always)]
    pub fn vld(&self) -> bool {
        (self.value.read_bits() & Self::VLD_MASK) != 0
    }

}

/// FIFO_ST value type for batch read/modify/write operations
///
/// Use `fifost.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `fifost.write_val(val)`.
#[derive(Clone, Copy)]
pub struct FifoStVal {
    bits: u32,
}

impl FifoStVal {
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
        Self { bits: 0x00000002 }
    }

    /// Read **ROE** `[3:3]`
    /// Sticky flag indicating the RX FIFO was read when empty. This read was ignored by the FIFO.
    #[inline(always)]
    pub const fn roe(&self) -> bool {
        (self.bits & FifoSt::ROE_MASK) != 0
    }

    /// Read **WOF** `[2:2]`
    /// Sticky flag indicating the TX FIFO was written when full. This write was ignored by the FIFO.
    #[inline(always)]
    pub const fn wof(&self) -> bool {
        (self.bits & FifoSt::WOF_MASK) != 0
    }

    /// Read **RDY** `[1:1]`
    /// Value is 1 if this core's TX FIFO is not full (i.e. if FIFO_WR is ready for more data)
    #[inline(always)]
    pub const fn rdy(&self) -> bool {
        (self.bits & FifoSt::RDY_MASK) != 0
    }

    /// Read **VLD** `[0:0]`
    /// Value is 1 if this core's RX FIFO is not empty (i.e. if FIFO_RD is valid)
    #[inline(always)]
    pub const fn vld(&self) -> bool {
        (self.bits & FifoSt::VLD_MASK) != 0
    }

    /// Set **ROE** `[3:3]`
    /// Sticky flag indicating the RX FIFO was read when empty. This read was ignored by the FIFO.
    #[inline(always)]
    pub const fn with_roe(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | FifoSt::ROE_MASK
            } else {
                self.bits & !FifoSt::ROE_MASK
            }
        }
    }

    /// Set **WOF** `[2:2]`
    /// Sticky flag indicating the TX FIFO was written when full. This write was ignored by the FIFO.
    #[inline(always)]
    pub const fn with_wof(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | FifoSt::WOF_MASK
            } else {
                self.bits & !FifoSt::WOF_MASK
            }
        }
    }

}

impl Default for FifoStVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl FifoSt {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> FifoStVal {
        FifoStVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: FifoStVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(FifoStVal) -> FifoStVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// FIFO_WR Register
///
/// Write access to this core's TX FIFO
///
/// Address offset: 0x0054
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **FIFO_WR** `[31:0]` (WO)
#[repr(transparent)]
pub struct FifoWr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for FifoWr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl FifoWr {
    // ======== Field Constants ========

    /// **FIFO_WR**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const FIFO_WR_OFFSET: u32 = 0;
    pub const FIFO_WR_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **FIFO_WR** `[31:0]` (WO)
    /// Write **FIFO_WR** `[31:0]` (WO)
    #[inline(always)]
    pub fn write_fifo_wr(&self, value: u32) {
        self.value.write_bits((value << Self::FIFO_WR_OFFSET) & Self::FIFO_WR_MASK);
    }

}

/// FIFO_WR value type for batch read/modify/write operations
///
/// Use `fifowr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `fifowr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct FifoWrVal {
    bits: u32,
}

impl FifoWrVal {
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

    /// Set **FIFO_WR** `[31:0]`
    #[inline(always)]
    pub const fn with_fifo_wr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !FifoWr::FIFO_WR_MASK) | ((value << FifoWr::FIFO_WR_OFFSET) & FifoWr::FIFO_WR_MASK)
        }
    }

}

impl Default for FifoWrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl FifoWr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> FifoWrVal {
        FifoWrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: FifoWrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(FifoWrVal) -> FifoWrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// FIFO_RD Register
///
/// Read access to this core's RX FIFO
///
/// Address offset: 0x0058
///
/// ## Fields
///
/// - **FIFO_RD** `[31:0]` (RO)
#[repr(transparent)]
pub struct FifoRd {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for FifoRd {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl FifoRd {
    // ======== Field Constants ========

    /// **FIFO_RD**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const FIFO_RD_OFFSET: u32 = 0;
    pub const FIFO_RD_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **FIFO_RD** `[31:0]` (RO)
    #[inline(always)]
    pub fn fifo_rd(&self) -> u32 {
        (self.value.read_bits() & Self::FIFO_RD_MASK) >> Self::FIFO_RD_OFFSET
    }

}

/// FIFO_RD value type for batch read/modify/write operations
///
/// Use `fiford.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `fiford.write_val(val)`.
#[derive(Clone, Copy)]
pub struct FifoRdVal {
    bits: u32,
}

impl FifoRdVal {
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

    /// Read **FIFO_RD** `[31:0]`
    #[inline(always)]
    pub const fn fifo_rd(&self) -> u32 {
        (self.bits & FifoRd::FIFO_RD_MASK) >> FifoRd::FIFO_RD_OFFSET
    }

}

impl Default for FifoRdVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl FifoRd {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> FifoRdVal {
        FifoRdVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: FifoRdVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(FifoRdVal) -> FifoRdVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SPINLOCK_ST Register
///
/// Spinlock state
/// A bitmap containing the state of all 32 spinlocks (1=locked).
/// Mainly intended for debugging.
///
/// Address offset: 0x005c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SPINLOCK_ST** `[31:0]` (RO)
#[repr(transparent)]
pub struct SpinlockSt {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SpinlockSt {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SpinlockSt {
    // ======== Field Constants ========

    /// **SPINLOCK_ST**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SPINLOCK_ST_OFFSET: u32 = 0;
    pub const SPINLOCK_ST_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SPINLOCK_ST** `[31:0]` (RO)
    #[inline(always)]
    pub fn spinlock_st(&self) -> u32 {
        (self.value.read_bits() & Self::SPINLOCK_ST_MASK) >> Self::SPINLOCK_ST_OFFSET
    }

}

/// SPINLOCK_ST value type for batch read/modify/write operations
///
/// Use `spinlockst.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spinlockst.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SpinlockStVal {
    bits: u32,
}

impl SpinlockStVal {
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

    /// Read **SPINLOCK_ST** `[31:0]`
    #[inline(always)]
    pub const fn spinlock_st(&self) -> u32 {
        (self.bits & SpinlockSt::SPINLOCK_ST_MASK) >> SpinlockSt::SPINLOCK_ST_OFFSET
    }

}

impl Default for SpinlockStVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SpinlockSt {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SpinlockStVal {
        SpinlockStVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SpinlockStVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SpinlockStVal) -> SpinlockStVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// INTERP0_ACCUM0 Register
///
/// Read/write access to accumulator 0
///
/// Address offset: 0x0080
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **INTERP0_ACCUM0** `[31:0]` (RW)
#[repr(transparent)]
pub struct Interp0Accum0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Interp0Accum0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Interp0Accum0 {
    // ======== Field Constants ========

    /// **INTERP0_ACCUM0**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const INTERP0_ACCUM0_OFFSET: u32 = 0;
    pub const INTERP0_ACCUM0_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **INTERP0_ACCUM0** `[31:0]` (RW)
    #[inline(always)]
    pub fn interp0_accum0(&self) -> u32 {
        (self.value.read_bits() & Self::INTERP0_ACCUM0_MASK) >> Self::INTERP0_ACCUM0_OFFSET
    }

    /// Write **INTERP0_ACCUM0** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_interp0_accum0(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::INTERP0_ACCUM0_MASK) | ((value << Self::INTERP0_ACCUM0_OFFSET) & Self::INTERP0_ACCUM0_MASK));
    }

}

/// INTERP0_ACCUM0 value type for batch read/modify/write operations
///
/// Use `interp0accum0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `interp0accum0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Interp0Accum0Val {
    bits: u32,
}

impl Interp0Accum0Val {
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

    /// Read **INTERP0_ACCUM0** `[31:0]`
    #[inline(always)]
    pub const fn interp0_accum0(&self) -> u32 {
        (self.bits & Interp0Accum0::INTERP0_ACCUM0_MASK) >> Interp0Accum0::INTERP0_ACCUM0_OFFSET
    }

    /// Set **INTERP0_ACCUM0** `[31:0]`
    #[inline(always)]
    pub const fn with_interp0_accum0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Interp0Accum0::INTERP0_ACCUM0_MASK) | ((value << Interp0Accum0::INTERP0_ACCUM0_OFFSET) & Interp0Accum0::INTERP0_ACCUM0_MASK)
        }
    }

}

impl Default for Interp0Accum0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Interp0Accum0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Interp0Accum0Val {
        Interp0Accum0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Interp0Accum0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Interp0Accum0Val) -> Interp0Accum0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// INTERP0_ACCUM1 Register
///
/// Read/write access to accumulator 1
///
/// Address offset: 0x0084
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **INTERP0_ACCUM1** `[31:0]` (RW)
#[repr(transparent)]
pub struct Interp0Accum1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Interp0Accum1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Interp0Accum1 {
    // ======== Field Constants ========

    /// **INTERP0_ACCUM1**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const INTERP0_ACCUM1_OFFSET: u32 = 0;
    pub const INTERP0_ACCUM1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **INTERP0_ACCUM1** `[31:0]` (RW)
    #[inline(always)]
    pub fn interp0_accum1(&self) -> u32 {
        (self.value.read_bits() & Self::INTERP0_ACCUM1_MASK) >> Self::INTERP0_ACCUM1_OFFSET
    }

    /// Write **INTERP0_ACCUM1** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_interp0_accum1(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::INTERP0_ACCUM1_MASK) | ((value << Self::INTERP0_ACCUM1_OFFSET) & Self::INTERP0_ACCUM1_MASK));
    }

}

/// INTERP0_ACCUM1 value type for batch read/modify/write operations
///
/// Use `interp0accum1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `interp0accum1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Interp0Accum1Val {
    bits: u32,
}

impl Interp0Accum1Val {
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

    /// Read **INTERP0_ACCUM1** `[31:0]`
    #[inline(always)]
    pub const fn interp0_accum1(&self) -> u32 {
        (self.bits & Interp0Accum1::INTERP0_ACCUM1_MASK) >> Interp0Accum1::INTERP0_ACCUM1_OFFSET
    }

    /// Set **INTERP0_ACCUM1** `[31:0]`
    #[inline(always)]
    pub const fn with_interp0_accum1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Interp0Accum1::INTERP0_ACCUM1_MASK) | ((value << Interp0Accum1::INTERP0_ACCUM1_OFFSET) & Interp0Accum1::INTERP0_ACCUM1_MASK)
        }
    }

}

impl Default for Interp0Accum1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Interp0Accum1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Interp0Accum1Val {
        Interp0Accum1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Interp0Accum1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Interp0Accum1Val) -> Interp0Accum1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// INTERP0_BASE0 Register
///
/// Read/write access to BASE0 register.
///
/// Address offset: 0x0088
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **INTERP0_BASE0** `[31:0]` (RW)
#[repr(transparent)]
pub struct Interp0Base0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Interp0Base0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Interp0Base0 {
    // ======== Field Constants ========

    /// **INTERP0_BASE0**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const INTERP0_BASE0_OFFSET: u32 = 0;
    pub const INTERP0_BASE0_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **INTERP0_BASE0** `[31:0]` (RW)
    #[inline(always)]
    pub fn interp0_base0(&self) -> u32 {
        (self.value.read_bits() & Self::INTERP0_BASE0_MASK) >> Self::INTERP0_BASE0_OFFSET
    }

    /// Write **INTERP0_BASE0** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_interp0_base0(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::INTERP0_BASE0_MASK) | ((value << Self::INTERP0_BASE0_OFFSET) & Self::INTERP0_BASE0_MASK));
    }

}

/// INTERP0_BASE0 value type for batch read/modify/write operations
///
/// Use `interp0base0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `interp0base0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Interp0Base0Val {
    bits: u32,
}

impl Interp0Base0Val {
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

    /// Read **INTERP0_BASE0** `[31:0]`
    #[inline(always)]
    pub const fn interp0_base0(&self) -> u32 {
        (self.bits & Interp0Base0::INTERP0_BASE0_MASK) >> Interp0Base0::INTERP0_BASE0_OFFSET
    }

    /// Set **INTERP0_BASE0** `[31:0]`
    #[inline(always)]
    pub const fn with_interp0_base0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Interp0Base0::INTERP0_BASE0_MASK) | ((value << Interp0Base0::INTERP0_BASE0_OFFSET) & Interp0Base0::INTERP0_BASE0_MASK)
        }
    }

}

impl Default for Interp0Base0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Interp0Base0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Interp0Base0Val {
        Interp0Base0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Interp0Base0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Interp0Base0Val) -> Interp0Base0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// INTERP0_BASE1 Register
///
/// Read/write access to BASE1 register.
///
/// Address offset: 0x008c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **INTERP0_BASE1** `[31:0]` (RW)
#[repr(transparent)]
pub struct Interp0Base1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Interp0Base1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Interp0Base1 {
    // ======== Field Constants ========

    /// **INTERP0_BASE1**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const INTERP0_BASE1_OFFSET: u32 = 0;
    pub const INTERP0_BASE1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **INTERP0_BASE1** `[31:0]` (RW)
    #[inline(always)]
    pub fn interp0_base1(&self) -> u32 {
        (self.value.read_bits() & Self::INTERP0_BASE1_MASK) >> Self::INTERP0_BASE1_OFFSET
    }

    /// Write **INTERP0_BASE1** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_interp0_base1(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::INTERP0_BASE1_MASK) | ((value << Self::INTERP0_BASE1_OFFSET) & Self::INTERP0_BASE1_MASK));
    }

}

/// INTERP0_BASE1 value type for batch read/modify/write operations
///
/// Use `interp0base1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `interp0base1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Interp0Base1Val {
    bits: u32,
}

impl Interp0Base1Val {
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

    /// Read **INTERP0_BASE1** `[31:0]`
    #[inline(always)]
    pub const fn interp0_base1(&self) -> u32 {
        (self.bits & Interp0Base1::INTERP0_BASE1_MASK) >> Interp0Base1::INTERP0_BASE1_OFFSET
    }

    /// Set **INTERP0_BASE1** `[31:0]`
    #[inline(always)]
    pub const fn with_interp0_base1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Interp0Base1::INTERP0_BASE1_MASK) | ((value << Interp0Base1::INTERP0_BASE1_OFFSET) & Interp0Base1::INTERP0_BASE1_MASK)
        }
    }

}

impl Default for Interp0Base1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Interp0Base1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Interp0Base1Val {
        Interp0Base1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Interp0Base1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Interp0Base1Val) -> Interp0Base1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// INTERP0_BASE2 Register
///
/// Read/write access to BASE2 register.
///
/// Address offset: 0x0090
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **INTERP0_BASE2** `[31:0]` (RW)
#[repr(transparent)]
pub struct Interp0Base2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Interp0Base2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Interp0Base2 {
    // ======== Field Constants ========

    /// **INTERP0_BASE2**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const INTERP0_BASE2_OFFSET: u32 = 0;
    pub const INTERP0_BASE2_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **INTERP0_BASE2** `[31:0]` (RW)
    #[inline(always)]
    pub fn interp0_base2(&self) -> u32 {
        (self.value.read_bits() & Self::INTERP0_BASE2_MASK) >> Self::INTERP0_BASE2_OFFSET
    }

    /// Write **INTERP0_BASE2** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_interp0_base2(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::INTERP0_BASE2_MASK) | ((value << Self::INTERP0_BASE2_OFFSET) & Self::INTERP0_BASE2_MASK));
    }

}

/// INTERP0_BASE2 value type for batch read/modify/write operations
///
/// Use `interp0base2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `interp0base2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Interp0Base2Val {
    bits: u32,
}

impl Interp0Base2Val {
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

    /// Read **INTERP0_BASE2** `[31:0]`
    #[inline(always)]
    pub const fn interp0_base2(&self) -> u32 {
        (self.bits & Interp0Base2::INTERP0_BASE2_MASK) >> Interp0Base2::INTERP0_BASE2_OFFSET
    }

    /// Set **INTERP0_BASE2** `[31:0]`
    #[inline(always)]
    pub const fn with_interp0_base2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Interp0Base2::INTERP0_BASE2_MASK) | ((value << Interp0Base2::INTERP0_BASE2_OFFSET) & Interp0Base2::INTERP0_BASE2_MASK)
        }
    }

}

impl Default for Interp0Base2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Interp0Base2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Interp0Base2Val {
        Interp0Base2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Interp0Base2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Interp0Base2Val) -> Interp0Base2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// INTERP0_POP_LANE0 Register
///
/// Read LANE0 result, and simultaneously write lane results to both accumulators (POP).
///
/// Address offset: 0x0094
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **INTERP0_POP_LANE0** `[31:0]` (RO)
#[repr(transparent)]
pub struct Interp0PopLane0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Interp0PopLane0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Interp0PopLane0 {
    // ======== Field Constants ========

    /// **INTERP0_POP_LANE0**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const INTERP0_POP_LANE0_OFFSET: u32 = 0;
    pub const INTERP0_POP_LANE0_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **INTERP0_POP_LANE0** `[31:0]` (RO)
    #[inline(always)]
    pub fn interp0_pop_lane0(&self) -> u32 {
        (self.value.read_bits() & Self::INTERP0_POP_LANE0_MASK) >> Self::INTERP0_POP_LANE0_OFFSET
    }

}

/// INTERP0_POP_LANE0 value type for batch read/modify/write operations
///
/// Use `interp0poplane0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `interp0poplane0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Interp0PopLane0Val {
    bits: u32,
}

impl Interp0PopLane0Val {
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

    /// Read **INTERP0_POP_LANE0** `[31:0]`
    #[inline(always)]
    pub const fn interp0_pop_lane0(&self) -> u32 {
        (self.bits & Interp0PopLane0::INTERP0_POP_LANE0_MASK) >> Interp0PopLane0::INTERP0_POP_LANE0_OFFSET
    }

}

impl Default for Interp0PopLane0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Interp0PopLane0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Interp0PopLane0Val {
        Interp0PopLane0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Interp0PopLane0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Interp0PopLane0Val) -> Interp0PopLane0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// INTERP0_POP_LANE1 Register
///
/// Read LANE1 result, and simultaneously write lane results to both accumulators (POP).
///
/// Address offset: 0x0098
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **INTERP0_POP_LANE1** `[31:0]` (RO)
#[repr(transparent)]
pub struct Interp0PopLane1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Interp0PopLane1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Interp0PopLane1 {
    // ======== Field Constants ========

    /// **INTERP0_POP_LANE1**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const INTERP0_POP_LANE1_OFFSET: u32 = 0;
    pub const INTERP0_POP_LANE1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **INTERP0_POP_LANE1** `[31:0]` (RO)
    #[inline(always)]
    pub fn interp0_pop_lane1(&self) -> u32 {
        (self.value.read_bits() & Self::INTERP0_POP_LANE1_MASK) >> Self::INTERP0_POP_LANE1_OFFSET
    }

}

/// INTERP0_POP_LANE1 value type for batch read/modify/write operations
///
/// Use `interp0poplane1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `interp0poplane1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Interp0PopLane1Val {
    bits: u32,
}

impl Interp0PopLane1Val {
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

    /// Read **INTERP0_POP_LANE1** `[31:0]`
    #[inline(always)]
    pub const fn interp0_pop_lane1(&self) -> u32 {
        (self.bits & Interp0PopLane1::INTERP0_POP_LANE1_MASK) >> Interp0PopLane1::INTERP0_POP_LANE1_OFFSET
    }

}

impl Default for Interp0PopLane1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Interp0PopLane1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Interp0PopLane1Val {
        Interp0PopLane1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Interp0PopLane1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Interp0PopLane1Val) -> Interp0PopLane1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// INTERP0_POP_FULL Register
///
/// Read FULL result, and simultaneously write lane results to both accumulators (POP).
///
/// Address offset: 0x009c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **INTERP0_POP_FULL** `[31:0]` (RO)
#[repr(transparent)]
pub struct Interp0PopFull {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Interp0PopFull {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Interp0PopFull {
    // ======== Field Constants ========

    /// **INTERP0_POP_FULL**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const INTERP0_POP_FULL_OFFSET: u32 = 0;
    pub const INTERP0_POP_FULL_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **INTERP0_POP_FULL** `[31:0]` (RO)
    #[inline(always)]
    pub fn interp0_pop_full(&self) -> u32 {
        (self.value.read_bits() & Self::INTERP0_POP_FULL_MASK) >> Self::INTERP0_POP_FULL_OFFSET
    }

}

/// INTERP0_POP_FULL value type for batch read/modify/write operations
///
/// Use `interp0popfull.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `interp0popfull.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Interp0PopFullVal {
    bits: u32,
}

impl Interp0PopFullVal {
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

    /// Read **INTERP0_POP_FULL** `[31:0]`
    #[inline(always)]
    pub const fn interp0_pop_full(&self) -> u32 {
        (self.bits & Interp0PopFull::INTERP0_POP_FULL_MASK) >> Interp0PopFull::INTERP0_POP_FULL_OFFSET
    }

}

impl Default for Interp0PopFullVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Interp0PopFull {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Interp0PopFullVal {
        Interp0PopFullVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Interp0PopFullVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Interp0PopFullVal) -> Interp0PopFullVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// INTERP0_PEEK_LANE0 Register
///
/// Read LANE0 result, without altering any internal state (PEEK).
///
/// Address offset: 0x00a0
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **INTERP0_PEEK_LANE0** `[31:0]` (RO)
#[repr(transparent)]
pub struct Interp0PeekLane0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Interp0PeekLane0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Interp0PeekLane0 {
    // ======== Field Constants ========

    /// **INTERP0_PEEK_LANE0**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const INTERP0_PEEK_LANE0_OFFSET: u32 = 0;
    pub const INTERP0_PEEK_LANE0_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **INTERP0_PEEK_LANE0** `[31:0]` (RO)
    #[inline(always)]
    pub fn interp0_peek_lane0(&self) -> u32 {
        (self.value.read_bits() & Self::INTERP0_PEEK_LANE0_MASK) >> Self::INTERP0_PEEK_LANE0_OFFSET
    }

}

/// INTERP0_PEEK_LANE0 value type for batch read/modify/write operations
///
/// Use `interp0peeklane0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `interp0peeklane0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Interp0PeekLane0Val {
    bits: u32,
}

impl Interp0PeekLane0Val {
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

    /// Read **INTERP0_PEEK_LANE0** `[31:0]`
    #[inline(always)]
    pub const fn interp0_peek_lane0(&self) -> u32 {
        (self.bits & Interp0PeekLane0::INTERP0_PEEK_LANE0_MASK) >> Interp0PeekLane0::INTERP0_PEEK_LANE0_OFFSET
    }

}

impl Default for Interp0PeekLane0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Interp0PeekLane0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Interp0PeekLane0Val {
        Interp0PeekLane0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Interp0PeekLane0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Interp0PeekLane0Val) -> Interp0PeekLane0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// INTERP0_PEEK_LANE1 Register
///
/// Read LANE1 result, without altering any internal state (PEEK).
///
/// Address offset: 0x00a4
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **INTERP0_PEEK_LANE1** `[31:0]` (RO)
#[repr(transparent)]
pub struct Interp0PeekLane1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Interp0PeekLane1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Interp0PeekLane1 {
    // ======== Field Constants ========

    /// **INTERP0_PEEK_LANE1**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const INTERP0_PEEK_LANE1_OFFSET: u32 = 0;
    pub const INTERP0_PEEK_LANE1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **INTERP0_PEEK_LANE1** `[31:0]` (RO)
    #[inline(always)]
    pub fn interp0_peek_lane1(&self) -> u32 {
        (self.value.read_bits() & Self::INTERP0_PEEK_LANE1_MASK) >> Self::INTERP0_PEEK_LANE1_OFFSET
    }

}

/// INTERP0_PEEK_LANE1 value type for batch read/modify/write operations
///
/// Use `interp0peeklane1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `interp0peeklane1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Interp0PeekLane1Val {
    bits: u32,
}

impl Interp0PeekLane1Val {
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

    /// Read **INTERP0_PEEK_LANE1** `[31:0]`
    #[inline(always)]
    pub const fn interp0_peek_lane1(&self) -> u32 {
        (self.bits & Interp0PeekLane1::INTERP0_PEEK_LANE1_MASK) >> Interp0PeekLane1::INTERP0_PEEK_LANE1_OFFSET
    }

}

impl Default for Interp0PeekLane1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Interp0PeekLane1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Interp0PeekLane1Val {
        Interp0PeekLane1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Interp0PeekLane1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Interp0PeekLane1Val) -> Interp0PeekLane1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// INTERP0_PEEK_FULL Register
///
/// Read FULL result, without altering any internal state (PEEK).
///
/// Address offset: 0x00a8
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **INTERP0_PEEK_FULL** `[31:0]` (RO)
#[repr(transparent)]
pub struct Interp0PeekFull {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Interp0PeekFull {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Interp0PeekFull {
    // ======== Field Constants ========

    /// **INTERP0_PEEK_FULL**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const INTERP0_PEEK_FULL_OFFSET: u32 = 0;
    pub const INTERP0_PEEK_FULL_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **INTERP0_PEEK_FULL** `[31:0]` (RO)
    #[inline(always)]
    pub fn interp0_peek_full(&self) -> u32 {
        (self.value.read_bits() & Self::INTERP0_PEEK_FULL_MASK) >> Self::INTERP0_PEEK_FULL_OFFSET
    }

}

/// INTERP0_PEEK_FULL value type for batch read/modify/write operations
///
/// Use `interp0peekfull.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `interp0peekfull.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Interp0PeekFullVal {
    bits: u32,
}

impl Interp0PeekFullVal {
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

    /// Read **INTERP0_PEEK_FULL** `[31:0]`
    #[inline(always)]
    pub const fn interp0_peek_full(&self) -> u32 {
        (self.bits & Interp0PeekFull::INTERP0_PEEK_FULL_MASK) >> Interp0PeekFull::INTERP0_PEEK_FULL_OFFSET
    }

}

impl Default for Interp0PeekFullVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Interp0PeekFull {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Interp0PeekFullVal {
        Interp0PeekFullVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Interp0PeekFullVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Interp0PeekFullVal) -> Interp0PeekFullVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// INTERP0_CTRL_LANE0 Register
///
/// Control register for lane 0
///
/// Address offset: 0x00ac
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **OVERF** `[25]` (RO)
///   Set if either OVERF0 or OVERF1 is set.
/// - **OVERF1** `[24]` (RO)
///   Indicates if any masked-off MSBs in ACCUM1 are set.
/// - **OVERF0** `[23]` (RO)
///   Indicates if any masked-off MSBs in ACCUM0 are set.
/// - **BLEND** `[21]` (RW)
///   Only present on INTERP0 on each core. If BLEND mode is enabled:
///   - LANE1 result is a linear interpolation between BASE0 and BASE1, controlled
///   by the 8 LSBs of lane 1 shift and mask value (a fractional number between
///   0 and 255/256ths)
///   - LANE0 result does not have BASE0 added (yields only the 8 LSBs of lane 1 shift+mask value)
///   - FULL result does not have lane 1 shift+mask value added (BASE2 + lane 0 shift+mask)
///   LANE1 SIGNED flag controls whether the interpolation is signed or unsigned.
/// - **FORCE_MSB** `[20:19]` (RW)
///   ORed into bits 29:28 of the lane result presented to the processor on the bus.
///   No effect on the internal 32-bit datapath. Handy for using a lane to generate sequence
///   of pointers into flash or SRAM.
/// - **ADD_RAW** `[18]` (RW)
///   If 1, mask + shift is bypassed for LANE0 result. This does not affect FULL result.
/// - **CROSS_RESULT** `[17]` (RW)
///   If 1, feed the opposite lane's result into this lane's accumulator on POP.
/// - **CROSS_INPUT** `[16]` (RW)
///   If 1, feed the opposite lane's accumulator into this lane's shift + mask hardware.
///   Takes effect even if ADD_RAW is set (the CROSS_INPUT mux is before the shift+mask bypass)
/// - **SIGNED** `[15]` (RW)
///   If SIGNED is set, the shifted and masked accumulator value is sign-extended to 32 bits
///   before adding to BASE0, and LANE0 PEEK/POP appear extended to 32 bits when read by processor.
/// - **MASK_MSB** `[14:10]` (RW)
///   The most-significant bit allowed to pass by the mask (inclusive)
///   Setting MSB < LSB may cause chip to turn inside-out
/// - **MASK_LSB** `[9:5]` (RW)
///   The least-significant bit allowed to pass by the mask (inclusive)
/// - **SHIFT** `[4:0]` (RW)
///   Right-rotate applied to accumulator before masking. By appropriately configuring the masks, left and right shifts can be synthesised.
#[repr(transparent)]
pub struct Interp0CtrlLane0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Interp0CtrlLane0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Interp0CtrlLane0 {
    // ======== Field Constants ========

    /// **OVERF**
    /// Set if either OVERF0 or OVERF1 is set.
    /// Bits `[25:25]`, mask `0x02000000`
    pub const OVERF_OFFSET: u32 = 25;
    pub const OVERF_MASK: u32 = 0x02000000;
    pub const OVERF: u32 = 1 << 25;

    /// **OVERF1**
    /// Indicates if any masked-off MSBs in ACCUM1 are set.
    /// Bits `[24:24]`, mask `0x01000000`
    pub const OVERF1_OFFSET: u32 = 24;
    pub const OVERF1_MASK: u32 = 0x01000000;
    pub const OVERF1: u32 = 1 << 24;

    /// **OVERF0**
    /// Indicates if any masked-off MSBs in ACCUM0 are set.
    /// Bits `[23:23]`, mask `0x00800000`
    pub const OVERF0_OFFSET: u32 = 23;
    pub const OVERF0_MASK: u32 = 0x00800000;
    pub const OVERF0: u32 = 1 << 23;

    /// **BLEND**
    /// Only present on INTERP0 on each core. If BLEND mode is enabled:
    /// - LANE1 result is a linear interpolation between BASE0 and BASE1, controlled
    /// by the 8 LSBs of lane 1 shift and mask value (a fractional number between
    /// 0 and 255/256ths)
    /// - LANE0 result does not have BASE0 added (yields only the 8 LSBs of lane 1 shift+mask value)
    /// - FULL result does not have lane 1 shift+mask value added (BASE2 + lane 0 shift+mask)
    /// LANE1 SIGNED flag controls whether the interpolation is signed or unsigned.
    /// Bits `[21:21]`, mask `0x00200000`
    pub const BLEND_OFFSET: u32 = 21;
    pub const BLEND_MASK: u32 = 0x00200000;
    pub const BLEND: u32 = 1 << 21;

    /// **FORCE_MSB**
    /// ORed into bits 29:28 of the lane result presented to the processor on the bus.
    /// No effect on the internal 32-bit datapath. Handy for using a lane to generate sequence
    /// of pointers into flash or SRAM.
    /// Bits `[20:19]`, mask `0x00180000`
    pub const FORCE_MSB_OFFSET: u32 = 19;
    pub const FORCE_MSB_MASK: u32 = 0x00180000;

    /// **ADD_RAW**
    /// If 1, mask + shift is bypassed for LANE0 result. This does not affect FULL result.
    /// Bits `[18:18]`, mask `0x00040000`
    pub const ADD_RAW_OFFSET: u32 = 18;
    pub const ADD_RAW_MASK: u32 = 0x00040000;
    pub const ADD_RAW: u32 = 1 << 18;

    /// **CROSS_RESULT**
    /// If 1, feed the opposite lane's result into this lane's accumulator on POP.
    /// Bits `[17:17]`, mask `0x00020000`
    pub const CROSS_RESULT_OFFSET: u32 = 17;
    pub const CROSS_RESULT_MASK: u32 = 0x00020000;
    pub const CROSS_RESULT: u32 = 1 << 17;

    /// **CROSS_INPUT**
    /// If 1, feed the opposite lane's accumulator into this lane's shift + mask hardware.
    /// Takes effect even if ADD_RAW is set (the CROSS_INPUT mux is before the shift+mask bypass)
    /// Bits `[16:16]`, mask `0x00010000`
    pub const CROSS_INPUT_OFFSET: u32 = 16;
    pub const CROSS_INPUT_MASK: u32 = 0x00010000;
    pub const CROSS_INPUT: u32 = 1 << 16;

    /// **SIGNED**
    /// If SIGNED is set, the shifted and masked accumulator value is sign-extended to 32 bits
    /// before adding to BASE0, and LANE0 PEEK/POP appear extended to 32 bits when read by processor.
    /// Bits `[15:15]`, mask `0x00008000`
    pub const SIGNED_OFFSET: u32 = 15;
    pub const SIGNED_MASK: u32 = 0x00008000;
    pub const SIGNED: u32 = 1 << 15;

    /// **MASK_MSB**
    /// The most-significant bit allowed to pass by the mask (inclusive)
    /// Setting MSB < LSB may cause chip to turn inside-out
    /// Bits `[14:10]`, mask `0x00007c00`
    pub const MASK_MSB_OFFSET: u32 = 10;
    pub const MASK_MSB_MASK: u32 = 0x00007c00;

    /// **MASK_LSB**
    /// The least-significant bit allowed to pass by the mask (inclusive)
    /// Bits `[9:5]`, mask `0x000003e0`
    pub const MASK_LSB_OFFSET: u32 = 5;
    pub const MASK_LSB_MASK: u32 = 0x000003e0;

    /// **SHIFT**
    /// Right-rotate applied to accumulator before masking. By appropriately configuring the masks, left and right shifts can be synthesised.
    /// Bits `[4:0]`, mask `0x0000001f`
    pub const SHIFT_OFFSET: u32 = 0;
    pub const SHIFT_MASK: u32 = 0x0000001f;

    // ======== Field Accessors ========

    /// **OVERF** `[25]` (RO)
    /// Set if either OVERF0 or OVERF1 is set.
    #[inline(always)]
    pub fn overf(&self) -> bool {
        (self.value.read_bits() & Self::OVERF_MASK) != 0
    }

    /// **OVERF1** `[24]` (RO)
    /// Indicates if any masked-off MSBs in ACCUM1 are set.
    #[inline(always)]
    pub fn overf1(&self) -> bool {
        (self.value.read_bits() & Self::OVERF1_MASK) != 0
    }

    /// **OVERF0** `[23]` (RO)
    /// Indicates if any masked-off MSBs in ACCUM0 are set.
    #[inline(always)]
    pub fn overf0(&self) -> bool {
        (self.value.read_bits() & Self::OVERF0_MASK) != 0
    }

    /// **BLEND** `[21]` (RW)
    /// Only present on INTERP0 on each core. If BLEND mode is enabled:
    /// - LANE1 result is a linear interpolation between BASE0 and BASE1, controlled
    /// by the 8 LSBs of lane 1 shift and mask value (a fractional number between
    /// 0 and 255/256ths)
    /// - LANE0 result does not have BASE0 added (yields only the 8 LSBs of lane 1 shift+mask value)
    /// - FULL result does not have lane 1 shift+mask value added (BASE2 + lane 0 shift+mask)
    /// LANE1 SIGNED flag controls whether the interpolation is signed or unsigned.
    #[inline(always)]
    pub fn blend(&self) -> bool {
        (self.value.read_bits() & Self::BLEND_MASK) != 0
    }

    /// Set BLEND bit atomically (RW)
    #[inline(always)]
    pub fn set_blend(&self) {
        self.value.set_bits(Self::BLEND_MASK);
    }

    /// Clear BLEND bit atomically (RW)
    #[inline(always)]
    pub fn clr_blend(&self) {
        self.value.clr_bits(Self::BLEND_MASK);
    }

    /// **FORCE_MSB** `[20:19]` (RW)
    /// ORed into bits 29:28 of the lane result presented to the processor on the bus.
    /// No effect on the internal 32-bit datapath. Handy for using a lane to generate sequence
    /// of pointers into flash or SRAM.
    #[inline(always)]
    pub fn force_msb(&self) -> u32 {
        (self.value.read_bits() & Self::FORCE_MSB_MASK) >> Self::FORCE_MSB_OFFSET
    }

    /// Write **FORCE_MSB** `[20:19]` (RW)
    /// ORed into bits 29:28 of the lane result presented to the processor on the bus.
    /// No effect on the internal 32-bit datapath. Handy for using a lane to generate sequence
    /// of pointers into flash or SRAM.
    #[inline(always)]
    pub fn write_force_msb(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::FORCE_MSB_MASK) | ((value << Self::FORCE_MSB_OFFSET) & Self::FORCE_MSB_MASK));
    }

    /// **ADD_RAW** `[18]` (RW)
    /// If 1, mask + shift is bypassed for LANE0 result. This does not affect FULL result.
    #[inline(always)]
    pub fn add_raw(&self) -> bool {
        (self.value.read_bits() & Self::ADD_RAW_MASK) != 0
    }

    /// Set ADD_RAW bit atomically (RW)
    #[inline(always)]
    pub fn set_add_raw(&self) {
        self.value.set_bits(Self::ADD_RAW_MASK);
    }

    /// Clear ADD_RAW bit atomically (RW)
    #[inline(always)]
    pub fn clr_add_raw(&self) {
        self.value.clr_bits(Self::ADD_RAW_MASK);
    }

    /// **CROSS_RESULT** `[17]` (RW)
    /// If 1, feed the opposite lane's result into this lane's accumulator on POP.
    #[inline(always)]
    pub fn cross_result(&self) -> bool {
        (self.value.read_bits() & Self::CROSS_RESULT_MASK) != 0
    }

    /// Set CROSS_RESULT bit atomically (RW)
    #[inline(always)]
    pub fn set_cross_result(&self) {
        self.value.set_bits(Self::CROSS_RESULT_MASK);
    }

    /// Clear CROSS_RESULT bit atomically (RW)
    #[inline(always)]
    pub fn clr_cross_result(&self) {
        self.value.clr_bits(Self::CROSS_RESULT_MASK);
    }

    /// **CROSS_INPUT** `[16]` (RW)
    /// If 1, feed the opposite lane's accumulator into this lane's shift + mask hardware.
    /// Takes effect even if ADD_RAW is set (the CROSS_INPUT mux is before the shift+mask bypass)
    #[inline(always)]
    pub fn cross_input(&self) -> bool {
        (self.value.read_bits() & Self::CROSS_INPUT_MASK) != 0
    }

    /// Set CROSS_INPUT bit atomically (RW)
    #[inline(always)]
    pub fn set_cross_input(&self) {
        self.value.set_bits(Self::CROSS_INPUT_MASK);
    }

    /// Clear CROSS_INPUT bit atomically (RW)
    #[inline(always)]
    pub fn clr_cross_input(&self) {
        self.value.clr_bits(Self::CROSS_INPUT_MASK);
    }

    /// **SIGNED** `[15]` (RW)
    /// If SIGNED is set, the shifted and masked accumulator value is sign-extended to 32 bits
    /// before adding to BASE0, and LANE0 PEEK/POP appear extended to 32 bits when read by processor.
    #[inline(always)]
    pub fn signed(&self) -> bool {
        (self.value.read_bits() & Self::SIGNED_MASK) != 0
    }

    /// Set SIGNED bit atomically (RW)
    #[inline(always)]
    pub fn set_signed(&self) {
        self.value.set_bits(Self::SIGNED_MASK);
    }

    /// Clear SIGNED bit atomically (RW)
    #[inline(always)]
    pub fn clr_signed(&self) {
        self.value.clr_bits(Self::SIGNED_MASK);
    }

    /// **MASK_MSB** `[14:10]` (RW)
    /// The most-significant bit allowed to pass by the mask (inclusive)
    /// Setting MSB < LSB may cause chip to turn inside-out
    #[inline(always)]
    pub fn mask_msb(&self) -> u32 {
        (self.value.read_bits() & Self::MASK_MSB_MASK) >> Self::MASK_MSB_OFFSET
    }

    /// Write **MASK_MSB** `[14:10]` (RW)
    /// The most-significant bit allowed to pass by the mask (inclusive)
    /// Setting MSB < LSB may cause chip to turn inside-out
    #[inline(always)]
    pub fn write_mask_msb(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::MASK_MSB_MASK) | ((value << Self::MASK_MSB_OFFSET) & Self::MASK_MSB_MASK));
    }

    /// **MASK_LSB** `[9:5]` (RW)
    /// The least-significant bit allowed to pass by the mask (inclusive)
    #[inline(always)]
    pub fn mask_lsb(&self) -> u32 {
        (self.value.read_bits() & Self::MASK_LSB_MASK) >> Self::MASK_LSB_OFFSET
    }

    /// Write **MASK_LSB** `[9:5]` (RW)
    /// The least-significant bit allowed to pass by the mask (inclusive)
    #[inline(always)]
    pub fn write_mask_lsb(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::MASK_LSB_MASK) | ((value << Self::MASK_LSB_OFFSET) & Self::MASK_LSB_MASK));
    }

    /// **SHIFT** `[4:0]` (RW)
    /// Right-rotate applied to accumulator before masking. By appropriately configuring the masks, left and right shifts can be synthesised.
    #[inline(always)]
    pub fn shift(&self) -> u32 {
        (self.value.read_bits() & Self::SHIFT_MASK) >> Self::SHIFT_OFFSET
    }

    /// Write **SHIFT** `[4:0]` (RW)
    /// Right-rotate applied to accumulator before masking. By appropriately configuring the masks, left and right shifts can be synthesised.
    #[inline(always)]
    pub fn write_shift(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SHIFT_MASK) | ((value << Self::SHIFT_OFFSET) & Self::SHIFT_MASK));
    }

}

/// INTERP0_CTRL_LANE0 value type for batch read/modify/write operations
///
/// Use `interp0ctrllane0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `interp0ctrllane0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Interp0CtrlLane0Val {
    bits: u32,
}

impl Interp0CtrlLane0Val {
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

    /// Read **OVERF** `[25:25]`
    /// Set if either OVERF0 or OVERF1 is set.
    #[inline(always)]
    pub const fn overf(&self) -> bool {
        (self.bits & Interp0CtrlLane0::OVERF_MASK) != 0
    }

    /// Read **OVERF1** `[24:24]`
    /// Indicates if any masked-off MSBs in ACCUM1 are set.
    #[inline(always)]
    pub const fn overf1(&self) -> bool {
        (self.bits & Interp0CtrlLane0::OVERF1_MASK) != 0
    }

    /// Read **OVERF0** `[23:23]`
    /// Indicates if any masked-off MSBs in ACCUM0 are set.
    #[inline(always)]
    pub const fn overf0(&self) -> bool {
        (self.bits & Interp0CtrlLane0::OVERF0_MASK) != 0
    }

    /// Read **BLEND** `[21:21]`
    /// Only present on INTERP0 on each core. If BLEND mode is enabled:
    /// - LANE1 result is a linear interpolation between BASE0 and BASE1, controlled
    /// by the 8 LSBs of lane 1 shift and mask value (a fractional number between
    /// 0 and 255/256ths)
    /// - LANE0 result does not have BASE0 added (yields only the 8 LSBs of lane 1 shift+mask value)
    /// - FULL result does not have lane 1 shift+mask value added (BASE2 + lane 0 shift+mask)
    /// LANE1 SIGNED flag controls whether the interpolation is signed or unsigned.
    #[inline(always)]
    pub const fn blend(&self) -> bool {
        (self.bits & Interp0CtrlLane0::BLEND_MASK) != 0
    }

    /// Read **FORCE_MSB** `[20:19]`
    /// ORed into bits 29:28 of the lane result presented to the processor on the bus.
    /// No effect on the internal 32-bit datapath. Handy for using a lane to generate sequence
    /// of pointers into flash or SRAM.
    #[inline(always)]
    pub const fn force_msb(&self) -> u32 {
        (self.bits & Interp0CtrlLane0::FORCE_MSB_MASK) >> Interp0CtrlLane0::FORCE_MSB_OFFSET
    }

    /// Read **ADD_RAW** `[18:18]`
    /// If 1, mask + shift is bypassed for LANE0 result. This does not affect FULL result.
    #[inline(always)]
    pub const fn add_raw(&self) -> bool {
        (self.bits & Interp0CtrlLane0::ADD_RAW_MASK) != 0
    }

    /// Read **CROSS_RESULT** `[17:17]`
    /// If 1, feed the opposite lane's result into this lane's accumulator on POP.
    #[inline(always)]
    pub const fn cross_result(&self) -> bool {
        (self.bits & Interp0CtrlLane0::CROSS_RESULT_MASK) != 0
    }

    /// Read **CROSS_INPUT** `[16:16]`
    /// If 1, feed the opposite lane's accumulator into this lane's shift + mask hardware.
    /// Takes effect even if ADD_RAW is set (the CROSS_INPUT mux is before the shift+mask bypass)
    #[inline(always)]
    pub const fn cross_input(&self) -> bool {
        (self.bits & Interp0CtrlLane0::CROSS_INPUT_MASK) != 0
    }

    /// Read **SIGNED** `[15:15]`
    /// If SIGNED is set, the shifted and masked accumulator value is sign-extended to 32 bits
    /// before adding to BASE0, and LANE0 PEEK/POP appear extended to 32 bits when read by processor.
    #[inline(always)]
    pub const fn signed(&self) -> bool {
        (self.bits & Interp0CtrlLane0::SIGNED_MASK) != 0
    }

    /// Read **MASK_MSB** `[14:10]`
    /// The most-significant bit allowed to pass by the mask (inclusive)
    /// Setting MSB < LSB may cause chip to turn inside-out
    #[inline(always)]
    pub const fn mask_msb(&self) -> u32 {
        (self.bits & Interp0CtrlLane0::MASK_MSB_MASK) >> Interp0CtrlLane0::MASK_MSB_OFFSET
    }

    /// Read **MASK_LSB** `[9:5]`
    /// The least-significant bit allowed to pass by the mask (inclusive)
    #[inline(always)]
    pub const fn mask_lsb(&self) -> u32 {
        (self.bits & Interp0CtrlLane0::MASK_LSB_MASK) >> Interp0CtrlLane0::MASK_LSB_OFFSET
    }

    /// Read **SHIFT** `[4:0]`
    /// Right-rotate applied to accumulator before masking. By appropriately configuring the masks, left and right shifts can be synthesised.
    #[inline(always)]
    pub const fn shift(&self) -> u32 {
        (self.bits & Interp0CtrlLane0::SHIFT_MASK) >> Interp0CtrlLane0::SHIFT_OFFSET
    }

    /// Set **BLEND** `[21:21]`
    /// Only present on INTERP0 on each core. If BLEND mode is enabled:
    /// - LANE1 result is a linear interpolation between BASE0 and BASE1, controlled
    /// by the 8 LSBs of lane 1 shift and mask value (a fractional number between
    /// 0 and 255/256ths)
    /// - LANE0 result does not have BASE0 added (yields only the 8 LSBs of lane 1 shift+mask value)
    /// - FULL result does not have lane 1 shift+mask value added (BASE2 + lane 0 shift+mask)
    /// LANE1 SIGNED flag controls whether the interpolation is signed or unsigned.
    #[inline(always)]
    pub const fn with_blend(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Interp0CtrlLane0::BLEND_MASK
            } else {
                self.bits & !Interp0CtrlLane0::BLEND_MASK
            }
        }
    }

    /// Set **FORCE_MSB** `[20:19]`
    /// ORed into bits 29:28 of the lane result presented to the processor on the bus.
    /// No effect on the internal 32-bit datapath. Handy for using a lane to generate sequence
    /// of pointers into flash or SRAM.
    #[inline(always)]
    pub const fn with_force_msb(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Interp0CtrlLane0::FORCE_MSB_MASK) | ((value << Interp0CtrlLane0::FORCE_MSB_OFFSET) & Interp0CtrlLane0::FORCE_MSB_MASK)
        }
    }

    /// Set **ADD_RAW** `[18:18]`
    /// If 1, mask + shift is bypassed for LANE0 result. This does not affect FULL result.
    #[inline(always)]
    pub const fn with_add_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Interp0CtrlLane0::ADD_RAW_MASK
            } else {
                self.bits & !Interp0CtrlLane0::ADD_RAW_MASK
            }
        }
    }

    /// Set **CROSS_RESULT** `[17:17]`
    /// If 1, feed the opposite lane's result into this lane's accumulator on POP.
    #[inline(always)]
    pub const fn with_cross_result(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Interp0CtrlLane0::CROSS_RESULT_MASK
            } else {
                self.bits & !Interp0CtrlLane0::CROSS_RESULT_MASK
            }
        }
    }

    /// Set **CROSS_INPUT** `[16:16]`
    /// If 1, feed the opposite lane's accumulator into this lane's shift + mask hardware.
    /// Takes effect even if ADD_RAW is set (the CROSS_INPUT mux is before the shift+mask bypass)
    #[inline(always)]
    pub const fn with_cross_input(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Interp0CtrlLane0::CROSS_INPUT_MASK
            } else {
                self.bits & !Interp0CtrlLane0::CROSS_INPUT_MASK
            }
        }
    }

    /// Set **SIGNED** `[15:15]`
    /// If SIGNED is set, the shifted and masked accumulator value is sign-extended to 32 bits
    /// before adding to BASE0, and LANE0 PEEK/POP appear extended to 32 bits when read by processor.
    #[inline(always)]
    pub const fn with_signed(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Interp0CtrlLane0::SIGNED_MASK
            } else {
                self.bits & !Interp0CtrlLane0::SIGNED_MASK
            }
        }
    }

    /// Set **MASK_MSB** `[14:10]`
    /// The most-significant bit allowed to pass by the mask (inclusive)
    /// Setting MSB < LSB may cause chip to turn inside-out
    #[inline(always)]
    pub const fn with_mask_msb(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Interp0CtrlLane0::MASK_MSB_MASK) | ((value << Interp0CtrlLane0::MASK_MSB_OFFSET) & Interp0CtrlLane0::MASK_MSB_MASK)
        }
    }

    /// Set **MASK_LSB** `[9:5]`
    /// The least-significant bit allowed to pass by the mask (inclusive)
    #[inline(always)]
    pub const fn with_mask_lsb(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Interp0CtrlLane0::MASK_LSB_MASK) | ((value << Interp0CtrlLane0::MASK_LSB_OFFSET) & Interp0CtrlLane0::MASK_LSB_MASK)
        }
    }

    /// Set **SHIFT** `[4:0]`
    /// Right-rotate applied to accumulator before masking. By appropriately configuring the masks, left and right shifts can be synthesised.
    #[inline(always)]
    pub const fn with_shift(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Interp0CtrlLane0::SHIFT_MASK) | ((value << Interp0CtrlLane0::SHIFT_OFFSET) & Interp0CtrlLane0::SHIFT_MASK)
        }
    }

}

impl Default for Interp0CtrlLane0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Interp0CtrlLane0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Interp0CtrlLane0Val {
        Interp0CtrlLane0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Interp0CtrlLane0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Interp0CtrlLane0Val) -> Interp0CtrlLane0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// INTERP0_CTRL_LANE1 Register
///
/// Control register for lane 1
///
/// Address offset: 0x00b0
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **FORCE_MSB** `[20:19]` (RW)
///   ORed into bits 29:28 of the lane result presented to the processor on the bus.
///   No effect on the internal 32-bit datapath. Handy for using a lane to generate sequence
///   of pointers into flash or SRAM.
/// - **ADD_RAW** `[18]` (RW)
///   If 1, mask + shift is bypassed for LANE1 result. This does not affect FULL result.
/// - **CROSS_RESULT** `[17]` (RW)
///   If 1, feed the opposite lane's result into this lane's accumulator on POP.
/// - **CROSS_INPUT** `[16]` (RW)
///   If 1, feed the opposite lane's accumulator into this lane's shift + mask hardware.
///   Takes effect even if ADD_RAW is set (the CROSS_INPUT mux is before the shift+mask bypass)
/// - **SIGNED** `[15]` (RW)
///   If SIGNED is set, the shifted and masked accumulator value is sign-extended to 32 bits
///   before adding to BASE1, and LANE1 PEEK/POP appear extended to 32 bits when read by processor.
/// - **MASK_MSB** `[14:10]` (RW)
///   The most-significant bit allowed to pass by the mask (inclusive)
///   Setting MSB < LSB may cause chip to turn inside-out
/// - **MASK_LSB** `[9:5]` (RW)
///   The least-significant bit allowed to pass by the mask (inclusive)
/// - **SHIFT** `[4:0]` (RW)
///   Right-rotate applied to accumulator before masking. By appropriately configuring the masks, left and right shifts can be synthesised.
#[repr(transparent)]
pub struct Interp0CtrlLane1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Interp0CtrlLane1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Interp0CtrlLane1 {
    // ======== Field Constants ========

    /// **FORCE_MSB**
    /// ORed into bits 29:28 of the lane result presented to the processor on the bus.
    /// No effect on the internal 32-bit datapath. Handy for using a lane to generate sequence
    /// of pointers into flash or SRAM.
    /// Bits `[20:19]`, mask `0x00180000`
    pub const FORCE_MSB_OFFSET: u32 = 19;
    pub const FORCE_MSB_MASK: u32 = 0x00180000;

    /// **ADD_RAW**
    /// If 1, mask + shift is bypassed for LANE1 result. This does not affect FULL result.
    /// Bits `[18:18]`, mask `0x00040000`
    pub const ADD_RAW_OFFSET: u32 = 18;
    pub const ADD_RAW_MASK: u32 = 0x00040000;
    pub const ADD_RAW: u32 = 1 << 18;

    /// **CROSS_RESULT**
    /// If 1, feed the opposite lane's result into this lane's accumulator on POP.
    /// Bits `[17:17]`, mask `0x00020000`
    pub const CROSS_RESULT_OFFSET: u32 = 17;
    pub const CROSS_RESULT_MASK: u32 = 0x00020000;
    pub const CROSS_RESULT: u32 = 1 << 17;

    /// **CROSS_INPUT**
    /// If 1, feed the opposite lane's accumulator into this lane's shift + mask hardware.
    /// Takes effect even if ADD_RAW is set (the CROSS_INPUT mux is before the shift+mask bypass)
    /// Bits `[16:16]`, mask `0x00010000`
    pub const CROSS_INPUT_OFFSET: u32 = 16;
    pub const CROSS_INPUT_MASK: u32 = 0x00010000;
    pub const CROSS_INPUT: u32 = 1 << 16;

    /// **SIGNED**
    /// If SIGNED is set, the shifted and masked accumulator value is sign-extended to 32 bits
    /// before adding to BASE1, and LANE1 PEEK/POP appear extended to 32 bits when read by processor.
    /// Bits `[15:15]`, mask `0x00008000`
    pub const SIGNED_OFFSET: u32 = 15;
    pub const SIGNED_MASK: u32 = 0x00008000;
    pub const SIGNED: u32 = 1 << 15;

    /// **MASK_MSB**
    /// The most-significant bit allowed to pass by the mask (inclusive)
    /// Setting MSB < LSB may cause chip to turn inside-out
    /// Bits `[14:10]`, mask `0x00007c00`
    pub const MASK_MSB_OFFSET: u32 = 10;
    pub const MASK_MSB_MASK: u32 = 0x00007c00;

    /// **MASK_LSB**
    /// The least-significant bit allowed to pass by the mask (inclusive)
    /// Bits `[9:5]`, mask `0x000003e0`
    pub const MASK_LSB_OFFSET: u32 = 5;
    pub const MASK_LSB_MASK: u32 = 0x000003e0;

    /// **SHIFT**
    /// Right-rotate applied to accumulator before masking. By appropriately configuring the masks, left and right shifts can be synthesised.
    /// Bits `[4:0]`, mask `0x0000001f`
    pub const SHIFT_OFFSET: u32 = 0;
    pub const SHIFT_MASK: u32 = 0x0000001f;

    // ======== Field Accessors ========

    /// **FORCE_MSB** `[20:19]` (RW)
    /// ORed into bits 29:28 of the lane result presented to the processor on the bus.
    /// No effect on the internal 32-bit datapath. Handy for using a lane to generate sequence
    /// of pointers into flash or SRAM.
    #[inline(always)]
    pub fn force_msb(&self) -> u32 {
        (self.value.read_bits() & Self::FORCE_MSB_MASK) >> Self::FORCE_MSB_OFFSET
    }

    /// Write **FORCE_MSB** `[20:19]` (RW)
    /// ORed into bits 29:28 of the lane result presented to the processor on the bus.
    /// No effect on the internal 32-bit datapath. Handy for using a lane to generate sequence
    /// of pointers into flash or SRAM.
    #[inline(always)]
    pub fn write_force_msb(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::FORCE_MSB_MASK) | ((value << Self::FORCE_MSB_OFFSET) & Self::FORCE_MSB_MASK));
    }

    /// **ADD_RAW** `[18]` (RW)
    /// If 1, mask + shift is bypassed for LANE1 result. This does not affect FULL result.
    #[inline(always)]
    pub fn add_raw(&self) -> bool {
        (self.value.read_bits() & Self::ADD_RAW_MASK) != 0
    }

    /// Set ADD_RAW bit atomically (RW)
    #[inline(always)]
    pub fn set_add_raw(&self) {
        self.value.set_bits(Self::ADD_RAW_MASK);
    }

    /// Clear ADD_RAW bit atomically (RW)
    #[inline(always)]
    pub fn clr_add_raw(&self) {
        self.value.clr_bits(Self::ADD_RAW_MASK);
    }

    /// **CROSS_RESULT** `[17]` (RW)
    /// If 1, feed the opposite lane's result into this lane's accumulator on POP.
    #[inline(always)]
    pub fn cross_result(&self) -> bool {
        (self.value.read_bits() & Self::CROSS_RESULT_MASK) != 0
    }

    /// Set CROSS_RESULT bit atomically (RW)
    #[inline(always)]
    pub fn set_cross_result(&self) {
        self.value.set_bits(Self::CROSS_RESULT_MASK);
    }

    /// Clear CROSS_RESULT bit atomically (RW)
    #[inline(always)]
    pub fn clr_cross_result(&self) {
        self.value.clr_bits(Self::CROSS_RESULT_MASK);
    }

    /// **CROSS_INPUT** `[16]` (RW)
    /// If 1, feed the opposite lane's accumulator into this lane's shift + mask hardware.
    /// Takes effect even if ADD_RAW is set (the CROSS_INPUT mux is before the shift+mask bypass)
    #[inline(always)]
    pub fn cross_input(&self) -> bool {
        (self.value.read_bits() & Self::CROSS_INPUT_MASK) != 0
    }

    /// Set CROSS_INPUT bit atomically (RW)
    #[inline(always)]
    pub fn set_cross_input(&self) {
        self.value.set_bits(Self::CROSS_INPUT_MASK);
    }

    /// Clear CROSS_INPUT bit atomically (RW)
    #[inline(always)]
    pub fn clr_cross_input(&self) {
        self.value.clr_bits(Self::CROSS_INPUT_MASK);
    }

    /// **SIGNED** `[15]` (RW)
    /// If SIGNED is set, the shifted and masked accumulator value is sign-extended to 32 bits
    /// before adding to BASE1, and LANE1 PEEK/POP appear extended to 32 bits when read by processor.
    #[inline(always)]
    pub fn signed(&self) -> bool {
        (self.value.read_bits() & Self::SIGNED_MASK) != 0
    }

    /// Set SIGNED bit atomically (RW)
    #[inline(always)]
    pub fn set_signed(&self) {
        self.value.set_bits(Self::SIGNED_MASK);
    }

    /// Clear SIGNED bit atomically (RW)
    #[inline(always)]
    pub fn clr_signed(&self) {
        self.value.clr_bits(Self::SIGNED_MASK);
    }

    /// **MASK_MSB** `[14:10]` (RW)
    /// The most-significant bit allowed to pass by the mask (inclusive)
    /// Setting MSB < LSB may cause chip to turn inside-out
    #[inline(always)]
    pub fn mask_msb(&self) -> u32 {
        (self.value.read_bits() & Self::MASK_MSB_MASK) >> Self::MASK_MSB_OFFSET
    }

    /// Write **MASK_MSB** `[14:10]` (RW)
    /// The most-significant bit allowed to pass by the mask (inclusive)
    /// Setting MSB < LSB may cause chip to turn inside-out
    #[inline(always)]
    pub fn write_mask_msb(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::MASK_MSB_MASK) | ((value << Self::MASK_MSB_OFFSET) & Self::MASK_MSB_MASK));
    }

    /// **MASK_LSB** `[9:5]` (RW)
    /// The least-significant bit allowed to pass by the mask (inclusive)
    #[inline(always)]
    pub fn mask_lsb(&self) -> u32 {
        (self.value.read_bits() & Self::MASK_LSB_MASK) >> Self::MASK_LSB_OFFSET
    }

    /// Write **MASK_LSB** `[9:5]` (RW)
    /// The least-significant bit allowed to pass by the mask (inclusive)
    #[inline(always)]
    pub fn write_mask_lsb(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::MASK_LSB_MASK) | ((value << Self::MASK_LSB_OFFSET) & Self::MASK_LSB_MASK));
    }

    /// **SHIFT** `[4:0]` (RW)
    /// Right-rotate applied to accumulator before masking. By appropriately configuring the masks, left and right shifts can be synthesised.
    #[inline(always)]
    pub fn shift(&self) -> u32 {
        (self.value.read_bits() & Self::SHIFT_MASK) >> Self::SHIFT_OFFSET
    }

    /// Write **SHIFT** `[4:0]` (RW)
    /// Right-rotate applied to accumulator before masking. By appropriately configuring the masks, left and right shifts can be synthesised.
    #[inline(always)]
    pub fn write_shift(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SHIFT_MASK) | ((value << Self::SHIFT_OFFSET) & Self::SHIFT_MASK));
    }

}

/// INTERP0_CTRL_LANE1 value type for batch read/modify/write operations
///
/// Use `interp0ctrllane1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `interp0ctrllane1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Interp0CtrlLane1Val {
    bits: u32,
}

impl Interp0CtrlLane1Val {
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

    /// Read **FORCE_MSB** `[20:19]`
    /// ORed into bits 29:28 of the lane result presented to the processor on the bus.
    /// No effect on the internal 32-bit datapath. Handy for using a lane to generate sequence
    /// of pointers into flash or SRAM.
    #[inline(always)]
    pub const fn force_msb(&self) -> u32 {
        (self.bits & Interp0CtrlLane1::FORCE_MSB_MASK) >> Interp0CtrlLane1::FORCE_MSB_OFFSET
    }

    /// Read **ADD_RAW** `[18:18]`
    /// If 1, mask + shift is bypassed for LANE1 result. This does not affect FULL result.
    #[inline(always)]
    pub const fn add_raw(&self) -> bool {
        (self.bits & Interp0CtrlLane1::ADD_RAW_MASK) != 0
    }

    /// Read **CROSS_RESULT** `[17:17]`
    /// If 1, feed the opposite lane's result into this lane's accumulator on POP.
    #[inline(always)]
    pub const fn cross_result(&self) -> bool {
        (self.bits & Interp0CtrlLane1::CROSS_RESULT_MASK) != 0
    }

    /// Read **CROSS_INPUT** `[16:16]`
    /// If 1, feed the opposite lane's accumulator into this lane's shift + mask hardware.
    /// Takes effect even if ADD_RAW is set (the CROSS_INPUT mux is before the shift+mask bypass)
    #[inline(always)]
    pub const fn cross_input(&self) -> bool {
        (self.bits & Interp0CtrlLane1::CROSS_INPUT_MASK) != 0
    }

    /// Read **SIGNED** `[15:15]`
    /// If SIGNED is set, the shifted and masked accumulator value is sign-extended to 32 bits
    /// before adding to BASE1, and LANE1 PEEK/POP appear extended to 32 bits when read by processor.
    #[inline(always)]
    pub const fn signed(&self) -> bool {
        (self.bits & Interp0CtrlLane1::SIGNED_MASK) != 0
    }

    /// Read **MASK_MSB** `[14:10]`
    /// The most-significant bit allowed to pass by the mask (inclusive)
    /// Setting MSB < LSB may cause chip to turn inside-out
    #[inline(always)]
    pub const fn mask_msb(&self) -> u32 {
        (self.bits & Interp0CtrlLane1::MASK_MSB_MASK) >> Interp0CtrlLane1::MASK_MSB_OFFSET
    }

    /// Read **MASK_LSB** `[9:5]`
    /// The least-significant bit allowed to pass by the mask (inclusive)
    #[inline(always)]
    pub const fn mask_lsb(&self) -> u32 {
        (self.bits & Interp0CtrlLane1::MASK_LSB_MASK) >> Interp0CtrlLane1::MASK_LSB_OFFSET
    }

    /// Read **SHIFT** `[4:0]`
    /// Right-rotate applied to accumulator before masking. By appropriately configuring the masks, left and right shifts can be synthesised.
    #[inline(always)]
    pub const fn shift(&self) -> u32 {
        (self.bits & Interp0CtrlLane1::SHIFT_MASK) >> Interp0CtrlLane1::SHIFT_OFFSET
    }

    /// Set **FORCE_MSB** `[20:19]`
    /// ORed into bits 29:28 of the lane result presented to the processor on the bus.
    /// No effect on the internal 32-bit datapath. Handy for using a lane to generate sequence
    /// of pointers into flash or SRAM.
    #[inline(always)]
    pub const fn with_force_msb(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Interp0CtrlLane1::FORCE_MSB_MASK) | ((value << Interp0CtrlLane1::FORCE_MSB_OFFSET) & Interp0CtrlLane1::FORCE_MSB_MASK)
        }
    }

    /// Set **ADD_RAW** `[18:18]`
    /// If 1, mask + shift is bypassed for LANE1 result. This does not affect FULL result.
    #[inline(always)]
    pub const fn with_add_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Interp0CtrlLane1::ADD_RAW_MASK
            } else {
                self.bits & !Interp0CtrlLane1::ADD_RAW_MASK
            }
        }
    }

    /// Set **CROSS_RESULT** `[17:17]`
    /// If 1, feed the opposite lane's result into this lane's accumulator on POP.
    #[inline(always)]
    pub const fn with_cross_result(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Interp0CtrlLane1::CROSS_RESULT_MASK
            } else {
                self.bits & !Interp0CtrlLane1::CROSS_RESULT_MASK
            }
        }
    }

    /// Set **CROSS_INPUT** `[16:16]`
    /// If 1, feed the opposite lane's accumulator into this lane's shift + mask hardware.
    /// Takes effect even if ADD_RAW is set (the CROSS_INPUT mux is before the shift+mask bypass)
    #[inline(always)]
    pub const fn with_cross_input(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Interp0CtrlLane1::CROSS_INPUT_MASK
            } else {
                self.bits & !Interp0CtrlLane1::CROSS_INPUT_MASK
            }
        }
    }

    /// Set **SIGNED** `[15:15]`
    /// If SIGNED is set, the shifted and masked accumulator value is sign-extended to 32 bits
    /// before adding to BASE1, and LANE1 PEEK/POP appear extended to 32 bits when read by processor.
    #[inline(always)]
    pub const fn with_signed(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Interp0CtrlLane1::SIGNED_MASK
            } else {
                self.bits & !Interp0CtrlLane1::SIGNED_MASK
            }
        }
    }

    /// Set **MASK_MSB** `[14:10]`
    /// The most-significant bit allowed to pass by the mask (inclusive)
    /// Setting MSB < LSB may cause chip to turn inside-out
    #[inline(always)]
    pub const fn with_mask_msb(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Interp0CtrlLane1::MASK_MSB_MASK) | ((value << Interp0CtrlLane1::MASK_MSB_OFFSET) & Interp0CtrlLane1::MASK_MSB_MASK)
        }
    }

    /// Set **MASK_LSB** `[9:5]`
    /// The least-significant bit allowed to pass by the mask (inclusive)
    #[inline(always)]
    pub const fn with_mask_lsb(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Interp0CtrlLane1::MASK_LSB_MASK) | ((value << Interp0CtrlLane1::MASK_LSB_OFFSET) & Interp0CtrlLane1::MASK_LSB_MASK)
        }
    }

    /// Set **SHIFT** `[4:0]`
    /// Right-rotate applied to accumulator before masking. By appropriately configuring the masks, left and right shifts can be synthesised.
    #[inline(always)]
    pub const fn with_shift(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Interp0CtrlLane1::SHIFT_MASK) | ((value << Interp0CtrlLane1::SHIFT_OFFSET) & Interp0CtrlLane1::SHIFT_MASK)
        }
    }

}

impl Default for Interp0CtrlLane1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Interp0CtrlLane1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Interp0CtrlLane1Val {
        Interp0CtrlLane1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Interp0CtrlLane1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Interp0CtrlLane1Val) -> Interp0CtrlLane1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// INTERP0_ACCUM0_ADD Register
///
/// Values written here are atomically added to ACCUM0
/// Reading yields lane 0's raw shift and mask value (BASE0 not added).
///
/// Address offset: 0x00b4
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **INTERP0_ACCUM0_ADD** `[23:0]` (RW)
#[repr(transparent)]
pub struct Interp0Accum0Add {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Interp0Accum0Add {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Interp0Accum0Add {
    // ======== Field Constants ========

    /// **INTERP0_ACCUM0_ADD**
    /// Bits `[23:0]`, mask `0x00ffffff`
    pub const INTERP0_ACCUM0_ADD_OFFSET: u32 = 0;
    pub const INTERP0_ACCUM0_ADD_MASK: u32 = 0x00ffffff;

    // ======== Field Accessors ========

    /// **INTERP0_ACCUM0_ADD** `[23:0]` (RW)
    #[inline(always)]
    pub fn interp0_accum0_add(&self) -> u32 {
        (self.value.read_bits() & Self::INTERP0_ACCUM0_ADD_MASK) >> Self::INTERP0_ACCUM0_ADD_OFFSET
    }

    /// Write **INTERP0_ACCUM0_ADD** `[23:0]` (RW)
    #[inline(always)]
    pub fn write_interp0_accum0_add(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::INTERP0_ACCUM0_ADD_MASK) | ((value << Self::INTERP0_ACCUM0_ADD_OFFSET) & Self::INTERP0_ACCUM0_ADD_MASK));
    }

}

/// INTERP0_ACCUM0_ADD value type for batch read/modify/write operations
///
/// Use `interp0accum0add.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `interp0accum0add.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Interp0Accum0AddVal {
    bits: u32,
}

impl Interp0Accum0AddVal {
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

    /// Read **INTERP0_ACCUM0_ADD** `[23:0]`
    #[inline(always)]
    pub const fn interp0_accum0_add(&self) -> u32 {
        (self.bits & Interp0Accum0Add::INTERP0_ACCUM0_ADD_MASK) >> Interp0Accum0Add::INTERP0_ACCUM0_ADD_OFFSET
    }

    /// Set **INTERP0_ACCUM0_ADD** `[23:0]`
    #[inline(always)]
    pub const fn with_interp0_accum0_add(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Interp0Accum0Add::INTERP0_ACCUM0_ADD_MASK) | ((value << Interp0Accum0Add::INTERP0_ACCUM0_ADD_OFFSET) & Interp0Accum0Add::INTERP0_ACCUM0_ADD_MASK)
        }
    }

}

impl Default for Interp0Accum0AddVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Interp0Accum0Add {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Interp0Accum0AddVal {
        Interp0Accum0AddVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Interp0Accum0AddVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Interp0Accum0AddVal) -> Interp0Accum0AddVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// INTERP0_ACCUM1_ADD Register
///
/// Values written here are atomically added to ACCUM1
/// Reading yields lane 1's raw shift and mask value (BASE1 not added).
///
/// Address offset: 0x00b8
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **INTERP0_ACCUM1_ADD** `[23:0]` (RW)
#[repr(transparent)]
pub struct Interp0Accum1Add {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Interp0Accum1Add {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Interp0Accum1Add {
    // ======== Field Constants ========

    /// **INTERP0_ACCUM1_ADD**
    /// Bits `[23:0]`, mask `0x00ffffff`
    pub const INTERP0_ACCUM1_ADD_OFFSET: u32 = 0;
    pub const INTERP0_ACCUM1_ADD_MASK: u32 = 0x00ffffff;

    // ======== Field Accessors ========

    /// **INTERP0_ACCUM1_ADD** `[23:0]` (RW)
    #[inline(always)]
    pub fn interp0_accum1_add(&self) -> u32 {
        (self.value.read_bits() & Self::INTERP0_ACCUM1_ADD_MASK) >> Self::INTERP0_ACCUM1_ADD_OFFSET
    }

    /// Write **INTERP0_ACCUM1_ADD** `[23:0]` (RW)
    #[inline(always)]
    pub fn write_interp0_accum1_add(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::INTERP0_ACCUM1_ADD_MASK) | ((value << Self::INTERP0_ACCUM1_ADD_OFFSET) & Self::INTERP0_ACCUM1_ADD_MASK));
    }

}

/// INTERP0_ACCUM1_ADD value type for batch read/modify/write operations
///
/// Use `interp0accum1add.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `interp0accum1add.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Interp0Accum1AddVal {
    bits: u32,
}

impl Interp0Accum1AddVal {
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

    /// Read **INTERP0_ACCUM1_ADD** `[23:0]`
    #[inline(always)]
    pub const fn interp0_accum1_add(&self) -> u32 {
        (self.bits & Interp0Accum1Add::INTERP0_ACCUM1_ADD_MASK) >> Interp0Accum1Add::INTERP0_ACCUM1_ADD_OFFSET
    }

    /// Set **INTERP0_ACCUM1_ADD** `[23:0]`
    #[inline(always)]
    pub const fn with_interp0_accum1_add(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Interp0Accum1Add::INTERP0_ACCUM1_ADD_MASK) | ((value << Interp0Accum1Add::INTERP0_ACCUM1_ADD_OFFSET) & Interp0Accum1Add::INTERP0_ACCUM1_ADD_MASK)
        }
    }

}

impl Default for Interp0Accum1AddVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Interp0Accum1Add {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Interp0Accum1AddVal {
        Interp0Accum1AddVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Interp0Accum1AddVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Interp0Accum1AddVal) -> Interp0Accum1AddVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// INTERP0_BASE_1AND0 Register
///
/// On write, the lower 16 bits go to BASE0, upper bits to BASE1 simultaneously.
/// Each half is sign-extended to 32 bits if that lane's SIGNED flag is set.
///
/// Address offset: 0x00bc
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **INTERP0_BASE_1AND0** `[31:0]` (WO)
#[repr(transparent)]
pub struct Interp0Base1and0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Interp0Base1and0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Interp0Base1and0 {
    // ======== Field Constants ========

    /// **INTERP0_BASE_1AND0**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const INTERP0_BASE_1AND0_OFFSET: u32 = 0;
    pub const INTERP0_BASE_1AND0_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **INTERP0_BASE_1AND0** `[31:0]` (WO)
    /// Write **INTERP0_BASE_1AND0** `[31:0]` (WO)
    #[inline(always)]
    pub fn write_interp0_base_1and0(&self, value: u32) {
        self.value.write_bits((value << Self::INTERP0_BASE_1AND0_OFFSET) & Self::INTERP0_BASE_1AND0_MASK);
    }

}

/// INTERP0_BASE_1AND0 value type for batch read/modify/write operations
///
/// Use `interp0base1and0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `interp0base1and0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Interp0Base1and0Val {
    bits: u32,
}

impl Interp0Base1and0Val {
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

    /// Set **INTERP0_BASE_1AND0** `[31:0]`
    #[inline(always)]
    pub const fn with_interp0_base_1and0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Interp0Base1and0::INTERP0_BASE_1AND0_MASK) | ((value << Interp0Base1and0::INTERP0_BASE_1AND0_OFFSET) & Interp0Base1and0::INTERP0_BASE_1AND0_MASK)
        }
    }

}

impl Default for Interp0Base1and0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Interp0Base1and0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Interp0Base1and0Val {
        Interp0Base1and0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Interp0Base1and0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Interp0Base1and0Val) -> Interp0Base1and0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// INTERP1_ACCUM0 Register
///
/// Read/write access to accumulator 0
///
/// Address offset: 0x00c0
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **INTERP1_ACCUM0** `[31:0]` (RW)
#[repr(transparent)]
pub struct Interp1Accum0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Interp1Accum0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Interp1Accum0 {
    // ======== Field Constants ========

    /// **INTERP1_ACCUM0**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const INTERP1_ACCUM0_OFFSET: u32 = 0;
    pub const INTERP1_ACCUM0_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **INTERP1_ACCUM0** `[31:0]` (RW)
    #[inline(always)]
    pub fn interp1_accum0(&self) -> u32 {
        (self.value.read_bits() & Self::INTERP1_ACCUM0_MASK) >> Self::INTERP1_ACCUM0_OFFSET
    }

    /// Write **INTERP1_ACCUM0** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_interp1_accum0(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::INTERP1_ACCUM0_MASK) | ((value << Self::INTERP1_ACCUM0_OFFSET) & Self::INTERP1_ACCUM0_MASK));
    }

}

/// INTERP1_ACCUM0 value type for batch read/modify/write operations
///
/// Use `interp1accum0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `interp1accum0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Interp1Accum0Val {
    bits: u32,
}

impl Interp1Accum0Val {
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

    /// Read **INTERP1_ACCUM0** `[31:0]`
    #[inline(always)]
    pub const fn interp1_accum0(&self) -> u32 {
        (self.bits & Interp1Accum0::INTERP1_ACCUM0_MASK) >> Interp1Accum0::INTERP1_ACCUM0_OFFSET
    }

    /// Set **INTERP1_ACCUM0** `[31:0]`
    #[inline(always)]
    pub const fn with_interp1_accum0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Interp1Accum0::INTERP1_ACCUM0_MASK) | ((value << Interp1Accum0::INTERP1_ACCUM0_OFFSET) & Interp1Accum0::INTERP1_ACCUM0_MASK)
        }
    }

}

impl Default for Interp1Accum0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Interp1Accum0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Interp1Accum0Val {
        Interp1Accum0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Interp1Accum0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Interp1Accum0Val) -> Interp1Accum0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// INTERP1_ACCUM1 Register
///
/// Read/write access to accumulator 1
///
/// Address offset: 0x00c4
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **INTERP1_ACCUM1** `[31:0]` (RW)
#[repr(transparent)]
pub struct Interp1Accum1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Interp1Accum1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Interp1Accum1 {
    // ======== Field Constants ========

    /// **INTERP1_ACCUM1**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const INTERP1_ACCUM1_OFFSET: u32 = 0;
    pub const INTERP1_ACCUM1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **INTERP1_ACCUM1** `[31:0]` (RW)
    #[inline(always)]
    pub fn interp1_accum1(&self) -> u32 {
        (self.value.read_bits() & Self::INTERP1_ACCUM1_MASK) >> Self::INTERP1_ACCUM1_OFFSET
    }

    /// Write **INTERP1_ACCUM1** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_interp1_accum1(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::INTERP1_ACCUM1_MASK) | ((value << Self::INTERP1_ACCUM1_OFFSET) & Self::INTERP1_ACCUM1_MASK));
    }

}

/// INTERP1_ACCUM1 value type for batch read/modify/write operations
///
/// Use `interp1accum1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `interp1accum1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Interp1Accum1Val {
    bits: u32,
}

impl Interp1Accum1Val {
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

    /// Read **INTERP1_ACCUM1** `[31:0]`
    #[inline(always)]
    pub const fn interp1_accum1(&self) -> u32 {
        (self.bits & Interp1Accum1::INTERP1_ACCUM1_MASK) >> Interp1Accum1::INTERP1_ACCUM1_OFFSET
    }

    /// Set **INTERP1_ACCUM1** `[31:0]`
    #[inline(always)]
    pub const fn with_interp1_accum1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Interp1Accum1::INTERP1_ACCUM1_MASK) | ((value << Interp1Accum1::INTERP1_ACCUM1_OFFSET) & Interp1Accum1::INTERP1_ACCUM1_MASK)
        }
    }

}

impl Default for Interp1Accum1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Interp1Accum1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Interp1Accum1Val {
        Interp1Accum1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Interp1Accum1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Interp1Accum1Val) -> Interp1Accum1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// INTERP1_BASE0 Register
///
/// Read/write access to BASE0 register.
///
/// Address offset: 0x00c8
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **INTERP1_BASE0** `[31:0]` (RW)
#[repr(transparent)]
pub struct Interp1Base0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Interp1Base0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Interp1Base0 {
    // ======== Field Constants ========

    /// **INTERP1_BASE0**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const INTERP1_BASE0_OFFSET: u32 = 0;
    pub const INTERP1_BASE0_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **INTERP1_BASE0** `[31:0]` (RW)
    #[inline(always)]
    pub fn interp1_base0(&self) -> u32 {
        (self.value.read_bits() & Self::INTERP1_BASE0_MASK) >> Self::INTERP1_BASE0_OFFSET
    }

    /// Write **INTERP1_BASE0** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_interp1_base0(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::INTERP1_BASE0_MASK) | ((value << Self::INTERP1_BASE0_OFFSET) & Self::INTERP1_BASE0_MASK));
    }

}

/// INTERP1_BASE0 value type for batch read/modify/write operations
///
/// Use `interp1base0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `interp1base0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Interp1Base0Val {
    bits: u32,
}

impl Interp1Base0Val {
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

    /// Read **INTERP1_BASE0** `[31:0]`
    #[inline(always)]
    pub const fn interp1_base0(&self) -> u32 {
        (self.bits & Interp1Base0::INTERP1_BASE0_MASK) >> Interp1Base0::INTERP1_BASE0_OFFSET
    }

    /// Set **INTERP1_BASE0** `[31:0]`
    #[inline(always)]
    pub const fn with_interp1_base0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Interp1Base0::INTERP1_BASE0_MASK) | ((value << Interp1Base0::INTERP1_BASE0_OFFSET) & Interp1Base0::INTERP1_BASE0_MASK)
        }
    }

}

impl Default for Interp1Base0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Interp1Base0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Interp1Base0Val {
        Interp1Base0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Interp1Base0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Interp1Base0Val) -> Interp1Base0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// INTERP1_BASE1 Register
///
/// Read/write access to BASE1 register.
///
/// Address offset: 0x00cc
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **INTERP1_BASE1** `[31:0]` (RW)
#[repr(transparent)]
pub struct Interp1Base1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Interp1Base1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Interp1Base1 {
    // ======== Field Constants ========

    /// **INTERP1_BASE1**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const INTERP1_BASE1_OFFSET: u32 = 0;
    pub const INTERP1_BASE1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **INTERP1_BASE1** `[31:0]` (RW)
    #[inline(always)]
    pub fn interp1_base1(&self) -> u32 {
        (self.value.read_bits() & Self::INTERP1_BASE1_MASK) >> Self::INTERP1_BASE1_OFFSET
    }

    /// Write **INTERP1_BASE1** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_interp1_base1(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::INTERP1_BASE1_MASK) | ((value << Self::INTERP1_BASE1_OFFSET) & Self::INTERP1_BASE1_MASK));
    }

}

/// INTERP1_BASE1 value type for batch read/modify/write operations
///
/// Use `interp1base1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `interp1base1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Interp1Base1Val {
    bits: u32,
}

impl Interp1Base1Val {
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

    /// Read **INTERP1_BASE1** `[31:0]`
    #[inline(always)]
    pub const fn interp1_base1(&self) -> u32 {
        (self.bits & Interp1Base1::INTERP1_BASE1_MASK) >> Interp1Base1::INTERP1_BASE1_OFFSET
    }

    /// Set **INTERP1_BASE1** `[31:0]`
    #[inline(always)]
    pub const fn with_interp1_base1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Interp1Base1::INTERP1_BASE1_MASK) | ((value << Interp1Base1::INTERP1_BASE1_OFFSET) & Interp1Base1::INTERP1_BASE1_MASK)
        }
    }

}

impl Default for Interp1Base1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Interp1Base1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Interp1Base1Val {
        Interp1Base1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Interp1Base1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Interp1Base1Val) -> Interp1Base1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// INTERP1_BASE2 Register
///
/// Read/write access to BASE2 register.
///
/// Address offset: 0x00d0
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **INTERP1_BASE2** `[31:0]` (RW)
#[repr(transparent)]
pub struct Interp1Base2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Interp1Base2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Interp1Base2 {
    // ======== Field Constants ========

    /// **INTERP1_BASE2**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const INTERP1_BASE2_OFFSET: u32 = 0;
    pub const INTERP1_BASE2_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **INTERP1_BASE2** `[31:0]` (RW)
    #[inline(always)]
    pub fn interp1_base2(&self) -> u32 {
        (self.value.read_bits() & Self::INTERP1_BASE2_MASK) >> Self::INTERP1_BASE2_OFFSET
    }

    /// Write **INTERP1_BASE2** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_interp1_base2(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::INTERP1_BASE2_MASK) | ((value << Self::INTERP1_BASE2_OFFSET) & Self::INTERP1_BASE2_MASK));
    }

}

/// INTERP1_BASE2 value type for batch read/modify/write operations
///
/// Use `interp1base2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `interp1base2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Interp1Base2Val {
    bits: u32,
}

impl Interp1Base2Val {
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

    /// Read **INTERP1_BASE2** `[31:0]`
    #[inline(always)]
    pub const fn interp1_base2(&self) -> u32 {
        (self.bits & Interp1Base2::INTERP1_BASE2_MASK) >> Interp1Base2::INTERP1_BASE2_OFFSET
    }

    /// Set **INTERP1_BASE2** `[31:0]`
    #[inline(always)]
    pub const fn with_interp1_base2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Interp1Base2::INTERP1_BASE2_MASK) | ((value << Interp1Base2::INTERP1_BASE2_OFFSET) & Interp1Base2::INTERP1_BASE2_MASK)
        }
    }

}

impl Default for Interp1Base2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Interp1Base2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Interp1Base2Val {
        Interp1Base2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Interp1Base2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Interp1Base2Val) -> Interp1Base2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// INTERP1_POP_LANE0 Register
///
/// Read LANE0 result, and simultaneously write lane results to both accumulators (POP).
///
/// Address offset: 0x00d4
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **INTERP1_POP_LANE0** `[31:0]` (RO)
#[repr(transparent)]
pub struct Interp1PopLane0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Interp1PopLane0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Interp1PopLane0 {
    // ======== Field Constants ========

    /// **INTERP1_POP_LANE0**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const INTERP1_POP_LANE0_OFFSET: u32 = 0;
    pub const INTERP1_POP_LANE0_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **INTERP1_POP_LANE0** `[31:0]` (RO)
    #[inline(always)]
    pub fn interp1_pop_lane0(&self) -> u32 {
        (self.value.read_bits() & Self::INTERP1_POP_LANE0_MASK) >> Self::INTERP1_POP_LANE0_OFFSET
    }

}

/// INTERP1_POP_LANE0 value type for batch read/modify/write operations
///
/// Use `interp1poplane0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `interp1poplane0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Interp1PopLane0Val {
    bits: u32,
}

impl Interp1PopLane0Val {
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

    /// Read **INTERP1_POP_LANE0** `[31:0]`
    #[inline(always)]
    pub const fn interp1_pop_lane0(&self) -> u32 {
        (self.bits & Interp1PopLane0::INTERP1_POP_LANE0_MASK) >> Interp1PopLane0::INTERP1_POP_LANE0_OFFSET
    }

}

impl Default for Interp1PopLane0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Interp1PopLane0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Interp1PopLane0Val {
        Interp1PopLane0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Interp1PopLane0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Interp1PopLane0Val) -> Interp1PopLane0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// INTERP1_POP_LANE1 Register
///
/// Read LANE1 result, and simultaneously write lane results to both accumulators (POP).
///
/// Address offset: 0x00d8
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **INTERP1_POP_LANE1** `[31:0]` (RO)
#[repr(transparent)]
pub struct Interp1PopLane1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Interp1PopLane1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Interp1PopLane1 {
    // ======== Field Constants ========

    /// **INTERP1_POP_LANE1**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const INTERP1_POP_LANE1_OFFSET: u32 = 0;
    pub const INTERP1_POP_LANE1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **INTERP1_POP_LANE1** `[31:0]` (RO)
    #[inline(always)]
    pub fn interp1_pop_lane1(&self) -> u32 {
        (self.value.read_bits() & Self::INTERP1_POP_LANE1_MASK) >> Self::INTERP1_POP_LANE1_OFFSET
    }

}

/// INTERP1_POP_LANE1 value type for batch read/modify/write operations
///
/// Use `interp1poplane1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `interp1poplane1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Interp1PopLane1Val {
    bits: u32,
}

impl Interp1PopLane1Val {
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

    /// Read **INTERP1_POP_LANE1** `[31:0]`
    #[inline(always)]
    pub const fn interp1_pop_lane1(&self) -> u32 {
        (self.bits & Interp1PopLane1::INTERP1_POP_LANE1_MASK) >> Interp1PopLane1::INTERP1_POP_LANE1_OFFSET
    }

}

impl Default for Interp1PopLane1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Interp1PopLane1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Interp1PopLane1Val {
        Interp1PopLane1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Interp1PopLane1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Interp1PopLane1Val) -> Interp1PopLane1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// INTERP1_POP_FULL Register
///
/// Read FULL result, and simultaneously write lane results to both accumulators (POP).
///
/// Address offset: 0x00dc
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **INTERP1_POP_FULL** `[31:0]` (RO)
#[repr(transparent)]
pub struct Interp1PopFull {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Interp1PopFull {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Interp1PopFull {
    // ======== Field Constants ========

    /// **INTERP1_POP_FULL**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const INTERP1_POP_FULL_OFFSET: u32 = 0;
    pub const INTERP1_POP_FULL_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **INTERP1_POP_FULL** `[31:0]` (RO)
    #[inline(always)]
    pub fn interp1_pop_full(&self) -> u32 {
        (self.value.read_bits() & Self::INTERP1_POP_FULL_MASK) >> Self::INTERP1_POP_FULL_OFFSET
    }

}

/// INTERP1_POP_FULL value type for batch read/modify/write operations
///
/// Use `interp1popfull.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `interp1popfull.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Interp1PopFullVal {
    bits: u32,
}

impl Interp1PopFullVal {
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

    /// Read **INTERP1_POP_FULL** `[31:0]`
    #[inline(always)]
    pub const fn interp1_pop_full(&self) -> u32 {
        (self.bits & Interp1PopFull::INTERP1_POP_FULL_MASK) >> Interp1PopFull::INTERP1_POP_FULL_OFFSET
    }

}

impl Default for Interp1PopFullVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Interp1PopFull {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Interp1PopFullVal {
        Interp1PopFullVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Interp1PopFullVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Interp1PopFullVal) -> Interp1PopFullVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// INTERP1_PEEK_LANE0 Register
///
/// Read LANE0 result, without altering any internal state (PEEK).
///
/// Address offset: 0x00e0
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **INTERP1_PEEK_LANE0** `[31:0]` (RO)
#[repr(transparent)]
pub struct Interp1PeekLane0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Interp1PeekLane0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Interp1PeekLane0 {
    // ======== Field Constants ========

    /// **INTERP1_PEEK_LANE0**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const INTERP1_PEEK_LANE0_OFFSET: u32 = 0;
    pub const INTERP1_PEEK_LANE0_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **INTERP1_PEEK_LANE0** `[31:0]` (RO)
    #[inline(always)]
    pub fn interp1_peek_lane0(&self) -> u32 {
        (self.value.read_bits() & Self::INTERP1_PEEK_LANE0_MASK) >> Self::INTERP1_PEEK_LANE0_OFFSET
    }

}

/// INTERP1_PEEK_LANE0 value type for batch read/modify/write operations
///
/// Use `interp1peeklane0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `interp1peeklane0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Interp1PeekLane0Val {
    bits: u32,
}

impl Interp1PeekLane0Val {
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

    /// Read **INTERP1_PEEK_LANE0** `[31:0]`
    #[inline(always)]
    pub const fn interp1_peek_lane0(&self) -> u32 {
        (self.bits & Interp1PeekLane0::INTERP1_PEEK_LANE0_MASK) >> Interp1PeekLane0::INTERP1_PEEK_LANE0_OFFSET
    }

}

impl Default for Interp1PeekLane0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Interp1PeekLane0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Interp1PeekLane0Val {
        Interp1PeekLane0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Interp1PeekLane0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Interp1PeekLane0Val) -> Interp1PeekLane0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// INTERP1_PEEK_LANE1 Register
///
/// Read LANE1 result, without altering any internal state (PEEK).
///
/// Address offset: 0x00e4
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **INTERP1_PEEK_LANE1** `[31:0]` (RO)
#[repr(transparent)]
pub struct Interp1PeekLane1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Interp1PeekLane1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Interp1PeekLane1 {
    // ======== Field Constants ========

    /// **INTERP1_PEEK_LANE1**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const INTERP1_PEEK_LANE1_OFFSET: u32 = 0;
    pub const INTERP1_PEEK_LANE1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **INTERP1_PEEK_LANE1** `[31:0]` (RO)
    #[inline(always)]
    pub fn interp1_peek_lane1(&self) -> u32 {
        (self.value.read_bits() & Self::INTERP1_PEEK_LANE1_MASK) >> Self::INTERP1_PEEK_LANE1_OFFSET
    }

}

/// INTERP1_PEEK_LANE1 value type for batch read/modify/write operations
///
/// Use `interp1peeklane1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `interp1peeklane1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Interp1PeekLane1Val {
    bits: u32,
}

impl Interp1PeekLane1Val {
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

    /// Read **INTERP1_PEEK_LANE1** `[31:0]`
    #[inline(always)]
    pub const fn interp1_peek_lane1(&self) -> u32 {
        (self.bits & Interp1PeekLane1::INTERP1_PEEK_LANE1_MASK) >> Interp1PeekLane1::INTERP1_PEEK_LANE1_OFFSET
    }

}

impl Default for Interp1PeekLane1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Interp1PeekLane1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Interp1PeekLane1Val {
        Interp1PeekLane1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Interp1PeekLane1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Interp1PeekLane1Val) -> Interp1PeekLane1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// INTERP1_PEEK_FULL Register
///
/// Read FULL result, without altering any internal state (PEEK).
///
/// Address offset: 0x00e8
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **INTERP1_PEEK_FULL** `[31:0]` (RO)
#[repr(transparent)]
pub struct Interp1PeekFull {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Interp1PeekFull {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Interp1PeekFull {
    // ======== Field Constants ========

    /// **INTERP1_PEEK_FULL**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const INTERP1_PEEK_FULL_OFFSET: u32 = 0;
    pub const INTERP1_PEEK_FULL_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **INTERP1_PEEK_FULL** `[31:0]` (RO)
    #[inline(always)]
    pub fn interp1_peek_full(&self) -> u32 {
        (self.value.read_bits() & Self::INTERP1_PEEK_FULL_MASK) >> Self::INTERP1_PEEK_FULL_OFFSET
    }

}

/// INTERP1_PEEK_FULL value type for batch read/modify/write operations
///
/// Use `interp1peekfull.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `interp1peekfull.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Interp1PeekFullVal {
    bits: u32,
}

impl Interp1PeekFullVal {
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

    /// Read **INTERP1_PEEK_FULL** `[31:0]`
    #[inline(always)]
    pub const fn interp1_peek_full(&self) -> u32 {
        (self.bits & Interp1PeekFull::INTERP1_PEEK_FULL_MASK) >> Interp1PeekFull::INTERP1_PEEK_FULL_OFFSET
    }

}

impl Default for Interp1PeekFullVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Interp1PeekFull {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Interp1PeekFullVal {
        Interp1PeekFullVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Interp1PeekFullVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Interp1PeekFullVal) -> Interp1PeekFullVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// INTERP1_CTRL_LANE0 Register
///
/// Control register for lane 0
///
/// Address offset: 0x00ec
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **OVERF** `[25]` (RO)
///   Set if either OVERF0 or OVERF1 is set.
/// - **OVERF1** `[24]` (RO)
///   Indicates if any masked-off MSBs in ACCUM1 are set.
/// - **OVERF0** `[23]` (RO)
///   Indicates if any masked-off MSBs in ACCUM0 are set.
/// - **CLAMP** `[22]` (RW)
///   Only present on INTERP1 on each core. If CLAMP mode is enabled:
///   - LANE0 result is shifted and masked ACCUM0, clamped by a lower bound of
///   BASE0 and an upper bound of BASE1.
///   - Signedness of these comparisons is determined by LANE0_CTRL_SIGNED
/// - **FORCE_MSB** `[20:19]` (RW)
///   ORed into bits 29:28 of the lane result presented to the processor on the bus.
///   No effect on the internal 32-bit datapath. Handy for using a lane to generate sequence
///   of pointers into flash or SRAM.
/// - **ADD_RAW** `[18]` (RW)
///   If 1, mask + shift is bypassed for LANE0 result. This does not affect FULL result.
/// - **CROSS_RESULT** `[17]` (RW)
///   If 1, feed the opposite lane's result into this lane's accumulator on POP.
/// - **CROSS_INPUT** `[16]` (RW)
///   If 1, feed the opposite lane's accumulator into this lane's shift + mask hardware.
///   Takes effect even if ADD_RAW is set (the CROSS_INPUT mux is before the shift+mask bypass)
/// - **SIGNED** `[15]` (RW)
///   If SIGNED is set, the shifted and masked accumulator value is sign-extended to 32 bits
///   before adding to BASE0, and LANE0 PEEK/POP appear extended to 32 bits when read by processor.
/// - **MASK_MSB** `[14:10]` (RW)
///   The most-significant bit allowed to pass by the mask (inclusive)
///   Setting MSB < LSB may cause chip to turn inside-out
/// - **MASK_LSB** `[9:5]` (RW)
///   The least-significant bit allowed to pass by the mask (inclusive)
/// - **SHIFT** `[4:0]` (RW)
///   Right-rotate applied to accumulator before masking. By appropriately configuring the masks, left and right shifts can be synthesised.
#[repr(transparent)]
pub struct Interp1CtrlLane0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Interp1CtrlLane0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Interp1CtrlLane0 {
    // ======== Field Constants ========

    /// **OVERF**
    /// Set if either OVERF0 or OVERF1 is set.
    /// Bits `[25:25]`, mask `0x02000000`
    pub const OVERF_OFFSET: u32 = 25;
    pub const OVERF_MASK: u32 = 0x02000000;
    pub const OVERF: u32 = 1 << 25;

    /// **OVERF1**
    /// Indicates if any masked-off MSBs in ACCUM1 are set.
    /// Bits `[24:24]`, mask `0x01000000`
    pub const OVERF1_OFFSET: u32 = 24;
    pub const OVERF1_MASK: u32 = 0x01000000;
    pub const OVERF1: u32 = 1 << 24;

    /// **OVERF0**
    /// Indicates if any masked-off MSBs in ACCUM0 are set.
    /// Bits `[23:23]`, mask `0x00800000`
    pub const OVERF0_OFFSET: u32 = 23;
    pub const OVERF0_MASK: u32 = 0x00800000;
    pub const OVERF0: u32 = 1 << 23;

    /// **CLAMP**
    /// Only present on INTERP1 on each core. If CLAMP mode is enabled:
    /// - LANE0 result is shifted and masked ACCUM0, clamped by a lower bound of
    /// BASE0 and an upper bound of BASE1.
    /// - Signedness of these comparisons is determined by LANE0_CTRL_SIGNED
    /// Bits `[22:22]`, mask `0x00400000`
    pub const CLAMP_OFFSET: u32 = 22;
    pub const CLAMP_MASK: u32 = 0x00400000;
    pub const CLAMP: u32 = 1 << 22;

    /// **FORCE_MSB**
    /// ORed into bits 29:28 of the lane result presented to the processor on the bus.
    /// No effect on the internal 32-bit datapath. Handy for using a lane to generate sequence
    /// of pointers into flash or SRAM.
    /// Bits `[20:19]`, mask `0x00180000`
    pub const FORCE_MSB_OFFSET: u32 = 19;
    pub const FORCE_MSB_MASK: u32 = 0x00180000;

    /// **ADD_RAW**
    /// If 1, mask + shift is bypassed for LANE0 result. This does not affect FULL result.
    /// Bits `[18:18]`, mask `0x00040000`
    pub const ADD_RAW_OFFSET: u32 = 18;
    pub const ADD_RAW_MASK: u32 = 0x00040000;
    pub const ADD_RAW: u32 = 1 << 18;

    /// **CROSS_RESULT**
    /// If 1, feed the opposite lane's result into this lane's accumulator on POP.
    /// Bits `[17:17]`, mask `0x00020000`
    pub const CROSS_RESULT_OFFSET: u32 = 17;
    pub const CROSS_RESULT_MASK: u32 = 0x00020000;
    pub const CROSS_RESULT: u32 = 1 << 17;

    /// **CROSS_INPUT**
    /// If 1, feed the opposite lane's accumulator into this lane's shift + mask hardware.
    /// Takes effect even if ADD_RAW is set (the CROSS_INPUT mux is before the shift+mask bypass)
    /// Bits `[16:16]`, mask `0x00010000`
    pub const CROSS_INPUT_OFFSET: u32 = 16;
    pub const CROSS_INPUT_MASK: u32 = 0x00010000;
    pub const CROSS_INPUT: u32 = 1 << 16;

    /// **SIGNED**
    /// If SIGNED is set, the shifted and masked accumulator value is sign-extended to 32 bits
    /// before adding to BASE0, and LANE0 PEEK/POP appear extended to 32 bits when read by processor.
    /// Bits `[15:15]`, mask `0x00008000`
    pub const SIGNED_OFFSET: u32 = 15;
    pub const SIGNED_MASK: u32 = 0x00008000;
    pub const SIGNED: u32 = 1 << 15;

    /// **MASK_MSB**
    /// The most-significant bit allowed to pass by the mask (inclusive)
    /// Setting MSB < LSB may cause chip to turn inside-out
    /// Bits `[14:10]`, mask `0x00007c00`
    pub const MASK_MSB_OFFSET: u32 = 10;
    pub const MASK_MSB_MASK: u32 = 0x00007c00;

    /// **MASK_LSB**
    /// The least-significant bit allowed to pass by the mask (inclusive)
    /// Bits `[9:5]`, mask `0x000003e0`
    pub const MASK_LSB_OFFSET: u32 = 5;
    pub const MASK_LSB_MASK: u32 = 0x000003e0;

    /// **SHIFT**
    /// Right-rotate applied to accumulator before masking. By appropriately configuring the masks, left and right shifts can be synthesised.
    /// Bits `[4:0]`, mask `0x0000001f`
    pub const SHIFT_OFFSET: u32 = 0;
    pub const SHIFT_MASK: u32 = 0x0000001f;

    // ======== Field Accessors ========

    /// **OVERF** `[25]` (RO)
    /// Set if either OVERF0 or OVERF1 is set.
    #[inline(always)]
    pub fn overf(&self) -> bool {
        (self.value.read_bits() & Self::OVERF_MASK) != 0
    }

    /// **OVERF1** `[24]` (RO)
    /// Indicates if any masked-off MSBs in ACCUM1 are set.
    #[inline(always)]
    pub fn overf1(&self) -> bool {
        (self.value.read_bits() & Self::OVERF1_MASK) != 0
    }

    /// **OVERF0** `[23]` (RO)
    /// Indicates if any masked-off MSBs in ACCUM0 are set.
    #[inline(always)]
    pub fn overf0(&self) -> bool {
        (self.value.read_bits() & Self::OVERF0_MASK) != 0
    }

    /// **CLAMP** `[22]` (RW)
    /// Only present on INTERP1 on each core. If CLAMP mode is enabled:
    /// - LANE0 result is shifted and masked ACCUM0, clamped by a lower bound of
    /// BASE0 and an upper bound of BASE1.
    /// - Signedness of these comparisons is determined by LANE0_CTRL_SIGNED
    #[inline(always)]
    pub fn clamp(&self) -> bool {
        (self.value.read_bits() & Self::CLAMP_MASK) != 0
    }

    /// Set CLAMP bit atomically (RW)
    #[inline(always)]
    pub fn set_clamp(&self) {
        self.value.set_bits(Self::CLAMP_MASK);
    }

    /// Clear CLAMP bit atomically (RW)
    #[inline(always)]
    pub fn clr_clamp(&self) {
        self.value.clr_bits(Self::CLAMP_MASK);
    }

    /// **FORCE_MSB** `[20:19]` (RW)
    /// ORed into bits 29:28 of the lane result presented to the processor on the bus.
    /// No effect on the internal 32-bit datapath. Handy for using a lane to generate sequence
    /// of pointers into flash or SRAM.
    #[inline(always)]
    pub fn force_msb(&self) -> u32 {
        (self.value.read_bits() & Self::FORCE_MSB_MASK) >> Self::FORCE_MSB_OFFSET
    }

    /// Write **FORCE_MSB** `[20:19]` (RW)
    /// ORed into bits 29:28 of the lane result presented to the processor on the bus.
    /// No effect on the internal 32-bit datapath. Handy for using a lane to generate sequence
    /// of pointers into flash or SRAM.
    #[inline(always)]
    pub fn write_force_msb(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::FORCE_MSB_MASK) | ((value << Self::FORCE_MSB_OFFSET) & Self::FORCE_MSB_MASK));
    }

    /// **ADD_RAW** `[18]` (RW)
    /// If 1, mask + shift is bypassed for LANE0 result. This does not affect FULL result.
    #[inline(always)]
    pub fn add_raw(&self) -> bool {
        (self.value.read_bits() & Self::ADD_RAW_MASK) != 0
    }

    /// Set ADD_RAW bit atomically (RW)
    #[inline(always)]
    pub fn set_add_raw(&self) {
        self.value.set_bits(Self::ADD_RAW_MASK);
    }

    /// Clear ADD_RAW bit atomically (RW)
    #[inline(always)]
    pub fn clr_add_raw(&self) {
        self.value.clr_bits(Self::ADD_RAW_MASK);
    }

    /// **CROSS_RESULT** `[17]` (RW)
    /// If 1, feed the opposite lane's result into this lane's accumulator on POP.
    #[inline(always)]
    pub fn cross_result(&self) -> bool {
        (self.value.read_bits() & Self::CROSS_RESULT_MASK) != 0
    }

    /// Set CROSS_RESULT bit atomically (RW)
    #[inline(always)]
    pub fn set_cross_result(&self) {
        self.value.set_bits(Self::CROSS_RESULT_MASK);
    }

    /// Clear CROSS_RESULT bit atomically (RW)
    #[inline(always)]
    pub fn clr_cross_result(&self) {
        self.value.clr_bits(Self::CROSS_RESULT_MASK);
    }

    /// **CROSS_INPUT** `[16]` (RW)
    /// If 1, feed the opposite lane's accumulator into this lane's shift + mask hardware.
    /// Takes effect even if ADD_RAW is set (the CROSS_INPUT mux is before the shift+mask bypass)
    #[inline(always)]
    pub fn cross_input(&self) -> bool {
        (self.value.read_bits() & Self::CROSS_INPUT_MASK) != 0
    }

    /// Set CROSS_INPUT bit atomically (RW)
    #[inline(always)]
    pub fn set_cross_input(&self) {
        self.value.set_bits(Self::CROSS_INPUT_MASK);
    }

    /// Clear CROSS_INPUT bit atomically (RW)
    #[inline(always)]
    pub fn clr_cross_input(&self) {
        self.value.clr_bits(Self::CROSS_INPUT_MASK);
    }

    /// **SIGNED** `[15]` (RW)
    /// If SIGNED is set, the shifted and masked accumulator value is sign-extended to 32 bits
    /// before adding to BASE0, and LANE0 PEEK/POP appear extended to 32 bits when read by processor.
    #[inline(always)]
    pub fn signed(&self) -> bool {
        (self.value.read_bits() & Self::SIGNED_MASK) != 0
    }

    /// Set SIGNED bit atomically (RW)
    #[inline(always)]
    pub fn set_signed(&self) {
        self.value.set_bits(Self::SIGNED_MASK);
    }

    /// Clear SIGNED bit atomically (RW)
    #[inline(always)]
    pub fn clr_signed(&self) {
        self.value.clr_bits(Self::SIGNED_MASK);
    }

    /// **MASK_MSB** `[14:10]` (RW)
    /// The most-significant bit allowed to pass by the mask (inclusive)
    /// Setting MSB < LSB may cause chip to turn inside-out
    #[inline(always)]
    pub fn mask_msb(&self) -> u32 {
        (self.value.read_bits() & Self::MASK_MSB_MASK) >> Self::MASK_MSB_OFFSET
    }

    /// Write **MASK_MSB** `[14:10]` (RW)
    /// The most-significant bit allowed to pass by the mask (inclusive)
    /// Setting MSB < LSB may cause chip to turn inside-out
    #[inline(always)]
    pub fn write_mask_msb(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::MASK_MSB_MASK) | ((value << Self::MASK_MSB_OFFSET) & Self::MASK_MSB_MASK));
    }

    /// **MASK_LSB** `[9:5]` (RW)
    /// The least-significant bit allowed to pass by the mask (inclusive)
    #[inline(always)]
    pub fn mask_lsb(&self) -> u32 {
        (self.value.read_bits() & Self::MASK_LSB_MASK) >> Self::MASK_LSB_OFFSET
    }

    /// Write **MASK_LSB** `[9:5]` (RW)
    /// The least-significant bit allowed to pass by the mask (inclusive)
    #[inline(always)]
    pub fn write_mask_lsb(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::MASK_LSB_MASK) | ((value << Self::MASK_LSB_OFFSET) & Self::MASK_LSB_MASK));
    }

    /// **SHIFT** `[4:0]` (RW)
    /// Right-rotate applied to accumulator before masking. By appropriately configuring the masks, left and right shifts can be synthesised.
    #[inline(always)]
    pub fn shift(&self) -> u32 {
        (self.value.read_bits() & Self::SHIFT_MASK) >> Self::SHIFT_OFFSET
    }

    /// Write **SHIFT** `[4:0]` (RW)
    /// Right-rotate applied to accumulator before masking. By appropriately configuring the masks, left and right shifts can be synthesised.
    #[inline(always)]
    pub fn write_shift(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SHIFT_MASK) | ((value << Self::SHIFT_OFFSET) & Self::SHIFT_MASK));
    }

}

/// INTERP1_CTRL_LANE0 value type for batch read/modify/write operations
///
/// Use `interp1ctrllane0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `interp1ctrllane0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Interp1CtrlLane0Val {
    bits: u32,
}

impl Interp1CtrlLane0Val {
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

    /// Read **OVERF** `[25:25]`
    /// Set if either OVERF0 or OVERF1 is set.
    #[inline(always)]
    pub const fn overf(&self) -> bool {
        (self.bits & Interp1CtrlLane0::OVERF_MASK) != 0
    }

    /// Read **OVERF1** `[24:24]`
    /// Indicates if any masked-off MSBs in ACCUM1 are set.
    #[inline(always)]
    pub const fn overf1(&self) -> bool {
        (self.bits & Interp1CtrlLane0::OVERF1_MASK) != 0
    }

    /// Read **OVERF0** `[23:23]`
    /// Indicates if any masked-off MSBs in ACCUM0 are set.
    #[inline(always)]
    pub const fn overf0(&self) -> bool {
        (self.bits & Interp1CtrlLane0::OVERF0_MASK) != 0
    }

    /// Read **CLAMP** `[22:22]`
    /// Only present on INTERP1 on each core. If CLAMP mode is enabled:
    /// - LANE0 result is shifted and masked ACCUM0, clamped by a lower bound of
    /// BASE0 and an upper bound of BASE1.
    /// - Signedness of these comparisons is determined by LANE0_CTRL_SIGNED
    #[inline(always)]
    pub const fn clamp(&self) -> bool {
        (self.bits & Interp1CtrlLane0::CLAMP_MASK) != 0
    }

    /// Read **FORCE_MSB** `[20:19]`
    /// ORed into bits 29:28 of the lane result presented to the processor on the bus.
    /// No effect on the internal 32-bit datapath. Handy for using a lane to generate sequence
    /// of pointers into flash or SRAM.
    #[inline(always)]
    pub const fn force_msb(&self) -> u32 {
        (self.bits & Interp1CtrlLane0::FORCE_MSB_MASK) >> Interp1CtrlLane0::FORCE_MSB_OFFSET
    }

    /// Read **ADD_RAW** `[18:18]`
    /// If 1, mask + shift is bypassed for LANE0 result. This does not affect FULL result.
    #[inline(always)]
    pub const fn add_raw(&self) -> bool {
        (self.bits & Interp1CtrlLane0::ADD_RAW_MASK) != 0
    }

    /// Read **CROSS_RESULT** `[17:17]`
    /// If 1, feed the opposite lane's result into this lane's accumulator on POP.
    #[inline(always)]
    pub const fn cross_result(&self) -> bool {
        (self.bits & Interp1CtrlLane0::CROSS_RESULT_MASK) != 0
    }

    /// Read **CROSS_INPUT** `[16:16]`
    /// If 1, feed the opposite lane's accumulator into this lane's shift + mask hardware.
    /// Takes effect even if ADD_RAW is set (the CROSS_INPUT mux is before the shift+mask bypass)
    #[inline(always)]
    pub const fn cross_input(&self) -> bool {
        (self.bits & Interp1CtrlLane0::CROSS_INPUT_MASK) != 0
    }

    /// Read **SIGNED** `[15:15]`
    /// If SIGNED is set, the shifted and masked accumulator value is sign-extended to 32 bits
    /// before adding to BASE0, and LANE0 PEEK/POP appear extended to 32 bits when read by processor.
    #[inline(always)]
    pub const fn signed(&self) -> bool {
        (self.bits & Interp1CtrlLane0::SIGNED_MASK) != 0
    }

    /// Read **MASK_MSB** `[14:10]`
    /// The most-significant bit allowed to pass by the mask (inclusive)
    /// Setting MSB < LSB may cause chip to turn inside-out
    #[inline(always)]
    pub const fn mask_msb(&self) -> u32 {
        (self.bits & Interp1CtrlLane0::MASK_MSB_MASK) >> Interp1CtrlLane0::MASK_MSB_OFFSET
    }

    /// Read **MASK_LSB** `[9:5]`
    /// The least-significant bit allowed to pass by the mask (inclusive)
    #[inline(always)]
    pub const fn mask_lsb(&self) -> u32 {
        (self.bits & Interp1CtrlLane0::MASK_LSB_MASK) >> Interp1CtrlLane0::MASK_LSB_OFFSET
    }

    /// Read **SHIFT** `[4:0]`
    /// Right-rotate applied to accumulator before masking. By appropriately configuring the masks, left and right shifts can be synthesised.
    #[inline(always)]
    pub const fn shift(&self) -> u32 {
        (self.bits & Interp1CtrlLane0::SHIFT_MASK) >> Interp1CtrlLane0::SHIFT_OFFSET
    }

    /// Set **CLAMP** `[22:22]`
    /// Only present on INTERP1 on each core. If CLAMP mode is enabled:
    /// - LANE0 result is shifted and masked ACCUM0, clamped by a lower bound of
    /// BASE0 and an upper bound of BASE1.
    /// - Signedness of these comparisons is determined by LANE0_CTRL_SIGNED
    #[inline(always)]
    pub const fn with_clamp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Interp1CtrlLane0::CLAMP_MASK
            } else {
                self.bits & !Interp1CtrlLane0::CLAMP_MASK
            }
        }
    }

    /// Set **FORCE_MSB** `[20:19]`
    /// ORed into bits 29:28 of the lane result presented to the processor on the bus.
    /// No effect on the internal 32-bit datapath. Handy for using a lane to generate sequence
    /// of pointers into flash or SRAM.
    #[inline(always)]
    pub const fn with_force_msb(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Interp1CtrlLane0::FORCE_MSB_MASK) | ((value << Interp1CtrlLane0::FORCE_MSB_OFFSET) & Interp1CtrlLane0::FORCE_MSB_MASK)
        }
    }

    /// Set **ADD_RAW** `[18:18]`
    /// If 1, mask + shift is bypassed for LANE0 result. This does not affect FULL result.
    #[inline(always)]
    pub const fn with_add_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Interp1CtrlLane0::ADD_RAW_MASK
            } else {
                self.bits & !Interp1CtrlLane0::ADD_RAW_MASK
            }
        }
    }

    /// Set **CROSS_RESULT** `[17:17]`
    /// If 1, feed the opposite lane's result into this lane's accumulator on POP.
    #[inline(always)]
    pub const fn with_cross_result(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Interp1CtrlLane0::CROSS_RESULT_MASK
            } else {
                self.bits & !Interp1CtrlLane0::CROSS_RESULT_MASK
            }
        }
    }

    /// Set **CROSS_INPUT** `[16:16]`
    /// If 1, feed the opposite lane's accumulator into this lane's shift + mask hardware.
    /// Takes effect even if ADD_RAW is set (the CROSS_INPUT mux is before the shift+mask bypass)
    #[inline(always)]
    pub const fn with_cross_input(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Interp1CtrlLane0::CROSS_INPUT_MASK
            } else {
                self.bits & !Interp1CtrlLane0::CROSS_INPUT_MASK
            }
        }
    }

    /// Set **SIGNED** `[15:15]`
    /// If SIGNED is set, the shifted and masked accumulator value is sign-extended to 32 bits
    /// before adding to BASE0, and LANE0 PEEK/POP appear extended to 32 bits when read by processor.
    #[inline(always)]
    pub const fn with_signed(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Interp1CtrlLane0::SIGNED_MASK
            } else {
                self.bits & !Interp1CtrlLane0::SIGNED_MASK
            }
        }
    }

    /// Set **MASK_MSB** `[14:10]`
    /// The most-significant bit allowed to pass by the mask (inclusive)
    /// Setting MSB < LSB may cause chip to turn inside-out
    #[inline(always)]
    pub const fn with_mask_msb(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Interp1CtrlLane0::MASK_MSB_MASK) | ((value << Interp1CtrlLane0::MASK_MSB_OFFSET) & Interp1CtrlLane0::MASK_MSB_MASK)
        }
    }

    /// Set **MASK_LSB** `[9:5]`
    /// The least-significant bit allowed to pass by the mask (inclusive)
    #[inline(always)]
    pub const fn with_mask_lsb(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Interp1CtrlLane0::MASK_LSB_MASK) | ((value << Interp1CtrlLane0::MASK_LSB_OFFSET) & Interp1CtrlLane0::MASK_LSB_MASK)
        }
    }

    /// Set **SHIFT** `[4:0]`
    /// Right-rotate applied to accumulator before masking. By appropriately configuring the masks, left and right shifts can be synthesised.
    #[inline(always)]
    pub const fn with_shift(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Interp1CtrlLane0::SHIFT_MASK) | ((value << Interp1CtrlLane0::SHIFT_OFFSET) & Interp1CtrlLane0::SHIFT_MASK)
        }
    }

}

impl Default for Interp1CtrlLane0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Interp1CtrlLane0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Interp1CtrlLane0Val {
        Interp1CtrlLane0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Interp1CtrlLane0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Interp1CtrlLane0Val) -> Interp1CtrlLane0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// INTERP1_CTRL_LANE1 Register
///
/// Control register for lane 1
///
/// Address offset: 0x00f0
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **FORCE_MSB** `[20:19]` (RW)
///   ORed into bits 29:28 of the lane result presented to the processor on the bus.
///   No effect on the internal 32-bit datapath. Handy for using a lane to generate sequence
///   of pointers into flash or SRAM.
/// - **ADD_RAW** `[18]` (RW)
///   If 1, mask + shift is bypassed for LANE1 result. This does not affect FULL result.
/// - **CROSS_RESULT** `[17]` (RW)
///   If 1, feed the opposite lane's result into this lane's accumulator on POP.
/// - **CROSS_INPUT** `[16]` (RW)
///   If 1, feed the opposite lane's accumulator into this lane's shift + mask hardware.
///   Takes effect even if ADD_RAW is set (the CROSS_INPUT mux is before the shift+mask bypass)
/// - **SIGNED** `[15]` (RW)
///   If SIGNED is set, the shifted and masked accumulator value is sign-extended to 32 bits
///   before adding to BASE1, and LANE1 PEEK/POP appear extended to 32 bits when read by processor.
/// - **MASK_MSB** `[14:10]` (RW)
///   The most-significant bit allowed to pass by the mask (inclusive)
///   Setting MSB < LSB may cause chip to turn inside-out
/// - **MASK_LSB** `[9:5]` (RW)
///   The least-significant bit allowed to pass by the mask (inclusive)
/// - **SHIFT** `[4:0]` (RW)
///   Right-rotate applied to accumulator before masking. By appropriately configuring the masks, left and right shifts can be synthesised.
#[repr(transparent)]
pub struct Interp1CtrlLane1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Interp1CtrlLane1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Interp1CtrlLane1 {
    // ======== Field Constants ========

    /// **FORCE_MSB**
    /// ORed into bits 29:28 of the lane result presented to the processor on the bus.
    /// No effect on the internal 32-bit datapath. Handy for using a lane to generate sequence
    /// of pointers into flash or SRAM.
    /// Bits `[20:19]`, mask `0x00180000`
    pub const FORCE_MSB_OFFSET: u32 = 19;
    pub const FORCE_MSB_MASK: u32 = 0x00180000;

    /// **ADD_RAW**
    /// If 1, mask + shift is bypassed for LANE1 result. This does not affect FULL result.
    /// Bits `[18:18]`, mask `0x00040000`
    pub const ADD_RAW_OFFSET: u32 = 18;
    pub const ADD_RAW_MASK: u32 = 0x00040000;
    pub const ADD_RAW: u32 = 1 << 18;

    /// **CROSS_RESULT**
    /// If 1, feed the opposite lane's result into this lane's accumulator on POP.
    /// Bits `[17:17]`, mask `0x00020000`
    pub const CROSS_RESULT_OFFSET: u32 = 17;
    pub const CROSS_RESULT_MASK: u32 = 0x00020000;
    pub const CROSS_RESULT: u32 = 1 << 17;

    /// **CROSS_INPUT**
    /// If 1, feed the opposite lane's accumulator into this lane's shift + mask hardware.
    /// Takes effect even if ADD_RAW is set (the CROSS_INPUT mux is before the shift+mask bypass)
    /// Bits `[16:16]`, mask `0x00010000`
    pub const CROSS_INPUT_OFFSET: u32 = 16;
    pub const CROSS_INPUT_MASK: u32 = 0x00010000;
    pub const CROSS_INPUT: u32 = 1 << 16;

    /// **SIGNED**
    /// If SIGNED is set, the shifted and masked accumulator value is sign-extended to 32 bits
    /// before adding to BASE1, and LANE1 PEEK/POP appear extended to 32 bits when read by processor.
    /// Bits `[15:15]`, mask `0x00008000`
    pub const SIGNED_OFFSET: u32 = 15;
    pub const SIGNED_MASK: u32 = 0x00008000;
    pub const SIGNED: u32 = 1 << 15;

    /// **MASK_MSB**
    /// The most-significant bit allowed to pass by the mask (inclusive)
    /// Setting MSB < LSB may cause chip to turn inside-out
    /// Bits `[14:10]`, mask `0x00007c00`
    pub const MASK_MSB_OFFSET: u32 = 10;
    pub const MASK_MSB_MASK: u32 = 0x00007c00;

    /// **MASK_LSB**
    /// The least-significant bit allowed to pass by the mask (inclusive)
    /// Bits `[9:5]`, mask `0x000003e0`
    pub const MASK_LSB_OFFSET: u32 = 5;
    pub const MASK_LSB_MASK: u32 = 0x000003e0;

    /// **SHIFT**
    /// Right-rotate applied to accumulator before masking. By appropriately configuring the masks, left and right shifts can be synthesised.
    /// Bits `[4:0]`, mask `0x0000001f`
    pub const SHIFT_OFFSET: u32 = 0;
    pub const SHIFT_MASK: u32 = 0x0000001f;

    // ======== Field Accessors ========

    /// **FORCE_MSB** `[20:19]` (RW)
    /// ORed into bits 29:28 of the lane result presented to the processor on the bus.
    /// No effect on the internal 32-bit datapath. Handy for using a lane to generate sequence
    /// of pointers into flash or SRAM.
    #[inline(always)]
    pub fn force_msb(&self) -> u32 {
        (self.value.read_bits() & Self::FORCE_MSB_MASK) >> Self::FORCE_MSB_OFFSET
    }

    /// Write **FORCE_MSB** `[20:19]` (RW)
    /// ORed into bits 29:28 of the lane result presented to the processor on the bus.
    /// No effect on the internal 32-bit datapath. Handy for using a lane to generate sequence
    /// of pointers into flash or SRAM.
    #[inline(always)]
    pub fn write_force_msb(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::FORCE_MSB_MASK) | ((value << Self::FORCE_MSB_OFFSET) & Self::FORCE_MSB_MASK));
    }

    /// **ADD_RAW** `[18]` (RW)
    /// If 1, mask + shift is bypassed for LANE1 result. This does not affect FULL result.
    #[inline(always)]
    pub fn add_raw(&self) -> bool {
        (self.value.read_bits() & Self::ADD_RAW_MASK) != 0
    }

    /// Set ADD_RAW bit atomically (RW)
    #[inline(always)]
    pub fn set_add_raw(&self) {
        self.value.set_bits(Self::ADD_RAW_MASK);
    }

    /// Clear ADD_RAW bit atomically (RW)
    #[inline(always)]
    pub fn clr_add_raw(&self) {
        self.value.clr_bits(Self::ADD_RAW_MASK);
    }

    /// **CROSS_RESULT** `[17]` (RW)
    /// If 1, feed the opposite lane's result into this lane's accumulator on POP.
    #[inline(always)]
    pub fn cross_result(&self) -> bool {
        (self.value.read_bits() & Self::CROSS_RESULT_MASK) != 0
    }

    /// Set CROSS_RESULT bit atomically (RW)
    #[inline(always)]
    pub fn set_cross_result(&self) {
        self.value.set_bits(Self::CROSS_RESULT_MASK);
    }

    /// Clear CROSS_RESULT bit atomically (RW)
    #[inline(always)]
    pub fn clr_cross_result(&self) {
        self.value.clr_bits(Self::CROSS_RESULT_MASK);
    }

    /// **CROSS_INPUT** `[16]` (RW)
    /// If 1, feed the opposite lane's accumulator into this lane's shift + mask hardware.
    /// Takes effect even if ADD_RAW is set (the CROSS_INPUT mux is before the shift+mask bypass)
    #[inline(always)]
    pub fn cross_input(&self) -> bool {
        (self.value.read_bits() & Self::CROSS_INPUT_MASK) != 0
    }

    /// Set CROSS_INPUT bit atomically (RW)
    #[inline(always)]
    pub fn set_cross_input(&self) {
        self.value.set_bits(Self::CROSS_INPUT_MASK);
    }

    /// Clear CROSS_INPUT bit atomically (RW)
    #[inline(always)]
    pub fn clr_cross_input(&self) {
        self.value.clr_bits(Self::CROSS_INPUT_MASK);
    }

    /// **SIGNED** `[15]` (RW)
    /// If SIGNED is set, the shifted and masked accumulator value is sign-extended to 32 bits
    /// before adding to BASE1, and LANE1 PEEK/POP appear extended to 32 bits when read by processor.
    #[inline(always)]
    pub fn signed(&self) -> bool {
        (self.value.read_bits() & Self::SIGNED_MASK) != 0
    }

    /// Set SIGNED bit atomically (RW)
    #[inline(always)]
    pub fn set_signed(&self) {
        self.value.set_bits(Self::SIGNED_MASK);
    }

    /// Clear SIGNED bit atomically (RW)
    #[inline(always)]
    pub fn clr_signed(&self) {
        self.value.clr_bits(Self::SIGNED_MASK);
    }

    /// **MASK_MSB** `[14:10]` (RW)
    /// The most-significant bit allowed to pass by the mask (inclusive)
    /// Setting MSB < LSB may cause chip to turn inside-out
    #[inline(always)]
    pub fn mask_msb(&self) -> u32 {
        (self.value.read_bits() & Self::MASK_MSB_MASK) >> Self::MASK_MSB_OFFSET
    }

    /// Write **MASK_MSB** `[14:10]` (RW)
    /// The most-significant bit allowed to pass by the mask (inclusive)
    /// Setting MSB < LSB may cause chip to turn inside-out
    #[inline(always)]
    pub fn write_mask_msb(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::MASK_MSB_MASK) | ((value << Self::MASK_MSB_OFFSET) & Self::MASK_MSB_MASK));
    }

    /// **MASK_LSB** `[9:5]` (RW)
    /// The least-significant bit allowed to pass by the mask (inclusive)
    #[inline(always)]
    pub fn mask_lsb(&self) -> u32 {
        (self.value.read_bits() & Self::MASK_LSB_MASK) >> Self::MASK_LSB_OFFSET
    }

    /// Write **MASK_LSB** `[9:5]` (RW)
    /// The least-significant bit allowed to pass by the mask (inclusive)
    #[inline(always)]
    pub fn write_mask_lsb(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::MASK_LSB_MASK) | ((value << Self::MASK_LSB_OFFSET) & Self::MASK_LSB_MASK));
    }

    /// **SHIFT** `[4:0]` (RW)
    /// Right-rotate applied to accumulator before masking. By appropriately configuring the masks, left and right shifts can be synthesised.
    #[inline(always)]
    pub fn shift(&self) -> u32 {
        (self.value.read_bits() & Self::SHIFT_MASK) >> Self::SHIFT_OFFSET
    }

    /// Write **SHIFT** `[4:0]` (RW)
    /// Right-rotate applied to accumulator before masking. By appropriately configuring the masks, left and right shifts can be synthesised.
    #[inline(always)]
    pub fn write_shift(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SHIFT_MASK) | ((value << Self::SHIFT_OFFSET) & Self::SHIFT_MASK));
    }

}

/// INTERP1_CTRL_LANE1 value type for batch read/modify/write operations
///
/// Use `interp1ctrllane1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `interp1ctrllane1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Interp1CtrlLane1Val {
    bits: u32,
}

impl Interp1CtrlLane1Val {
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

    /// Read **FORCE_MSB** `[20:19]`
    /// ORed into bits 29:28 of the lane result presented to the processor on the bus.
    /// No effect on the internal 32-bit datapath. Handy for using a lane to generate sequence
    /// of pointers into flash or SRAM.
    #[inline(always)]
    pub const fn force_msb(&self) -> u32 {
        (self.bits & Interp1CtrlLane1::FORCE_MSB_MASK) >> Interp1CtrlLane1::FORCE_MSB_OFFSET
    }

    /// Read **ADD_RAW** `[18:18]`
    /// If 1, mask + shift is bypassed for LANE1 result. This does not affect FULL result.
    #[inline(always)]
    pub const fn add_raw(&self) -> bool {
        (self.bits & Interp1CtrlLane1::ADD_RAW_MASK) != 0
    }

    /// Read **CROSS_RESULT** `[17:17]`
    /// If 1, feed the opposite lane's result into this lane's accumulator on POP.
    #[inline(always)]
    pub const fn cross_result(&self) -> bool {
        (self.bits & Interp1CtrlLane1::CROSS_RESULT_MASK) != 0
    }

    /// Read **CROSS_INPUT** `[16:16]`
    /// If 1, feed the opposite lane's accumulator into this lane's shift + mask hardware.
    /// Takes effect even if ADD_RAW is set (the CROSS_INPUT mux is before the shift+mask bypass)
    #[inline(always)]
    pub const fn cross_input(&self) -> bool {
        (self.bits & Interp1CtrlLane1::CROSS_INPUT_MASK) != 0
    }

    /// Read **SIGNED** `[15:15]`
    /// If SIGNED is set, the shifted and masked accumulator value is sign-extended to 32 bits
    /// before adding to BASE1, and LANE1 PEEK/POP appear extended to 32 bits when read by processor.
    #[inline(always)]
    pub const fn signed(&self) -> bool {
        (self.bits & Interp1CtrlLane1::SIGNED_MASK) != 0
    }

    /// Read **MASK_MSB** `[14:10]`
    /// The most-significant bit allowed to pass by the mask (inclusive)
    /// Setting MSB < LSB may cause chip to turn inside-out
    #[inline(always)]
    pub const fn mask_msb(&self) -> u32 {
        (self.bits & Interp1CtrlLane1::MASK_MSB_MASK) >> Interp1CtrlLane1::MASK_MSB_OFFSET
    }

    /// Read **MASK_LSB** `[9:5]`
    /// The least-significant bit allowed to pass by the mask (inclusive)
    #[inline(always)]
    pub const fn mask_lsb(&self) -> u32 {
        (self.bits & Interp1CtrlLane1::MASK_LSB_MASK) >> Interp1CtrlLane1::MASK_LSB_OFFSET
    }

    /// Read **SHIFT** `[4:0]`
    /// Right-rotate applied to accumulator before masking. By appropriately configuring the masks, left and right shifts can be synthesised.
    #[inline(always)]
    pub const fn shift(&self) -> u32 {
        (self.bits & Interp1CtrlLane1::SHIFT_MASK) >> Interp1CtrlLane1::SHIFT_OFFSET
    }

    /// Set **FORCE_MSB** `[20:19]`
    /// ORed into bits 29:28 of the lane result presented to the processor on the bus.
    /// No effect on the internal 32-bit datapath. Handy for using a lane to generate sequence
    /// of pointers into flash or SRAM.
    #[inline(always)]
    pub const fn with_force_msb(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Interp1CtrlLane1::FORCE_MSB_MASK) | ((value << Interp1CtrlLane1::FORCE_MSB_OFFSET) & Interp1CtrlLane1::FORCE_MSB_MASK)
        }
    }

    /// Set **ADD_RAW** `[18:18]`
    /// If 1, mask + shift is bypassed for LANE1 result. This does not affect FULL result.
    #[inline(always)]
    pub const fn with_add_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Interp1CtrlLane1::ADD_RAW_MASK
            } else {
                self.bits & !Interp1CtrlLane1::ADD_RAW_MASK
            }
        }
    }

    /// Set **CROSS_RESULT** `[17:17]`
    /// If 1, feed the opposite lane's result into this lane's accumulator on POP.
    #[inline(always)]
    pub const fn with_cross_result(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Interp1CtrlLane1::CROSS_RESULT_MASK
            } else {
                self.bits & !Interp1CtrlLane1::CROSS_RESULT_MASK
            }
        }
    }

    /// Set **CROSS_INPUT** `[16:16]`
    /// If 1, feed the opposite lane's accumulator into this lane's shift + mask hardware.
    /// Takes effect even if ADD_RAW is set (the CROSS_INPUT mux is before the shift+mask bypass)
    #[inline(always)]
    pub const fn with_cross_input(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Interp1CtrlLane1::CROSS_INPUT_MASK
            } else {
                self.bits & !Interp1CtrlLane1::CROSS_INPUT_MASK
            }
        }
    }

    /// Set **SIGNED** `[15:15]`
    /// If SIGNED is set, the shifted and masked accumulator value is sign-extended to 32 bits
    /// before adding to BASE1, and LANE1 PEEK/POP appear extended to 32 bits when read by processor.
    #[inline(always)]
    pub const fn with_signed(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Interp1CtrlLane1::SIGNED_MASK
            } else {
                self.bits & !Interp1CtrlLane1::SIGNED_MASK
            }
        }
    }

    /// Set **MASK_MSB** `[14:10]`
    /// The most-significant bit allowed to pass by the mask (inclusive)
    /// Setting MSB < LSB may cause chip to turn inside-out
    #[inline(always)]
    pub const fn with_mask_msb(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Interp1CtrlLane1::MASK_MSB_MASK) | ((value << Interp1CtrlLane1::MASK_MSB_OFFSET) & Interp1CtrlLane1::MASK_MSB_MASK)
        }
    }

    /// Set **MASK_LSB** `[9:5]`
    /// The least-significant bit allowed to pass by the mask (inclusive)
    #[inline(always)]
    pub const fn with_mask_lsb(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Interp1CtrlLane1::MASK_LSB_MASK) | ((value << Interp1CtrlLane1::MASK_LSB_OFFSET) & Interp1CtrlLane1::MASK_LSB_MASK)
        }
    }

    /// Set **SHIFT** `[4:0]`
    /// Right-rotate applied to accumulator before masking. By appropriately configuring the masks, left and right shifts can be synthesised.
    #[inline(always)]
    pub const fn with_shift(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Interp1CtrlLane1::SHIFT_MASK) | ((value << Interp1CtrlLane1::SHIFT_OFFSET) & Interp1CtrlLane1::SHIFT_MASK)
        }
    }

}

impl Default for Interp1CtrlLane1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Interp1CtrlLane1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Interp1CtrlLane1Val {
        Interp1CtrlLane1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Interp1CtrlLane1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Interp1CtrlLane1Val) -> Interp1CtrlLane1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// INTERP1_ACCUM0_ADD Register
///
/// Values written here are atomically added to ACCUM0
/// Reading yields lane 0's raw shift and mask value (BASE0 not added).
///
/// Address offset: 0x00f4
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **INTERP1_ACCUM0_ADD** `[23:0]` (RW)
#[repr(transparent)]
pub struct Interp1Accum0Add {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Interp1Accum0Add {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Interp1Accum0Add {
    // ======== Field Constants ========

    /// **INTERP1_ACCUM0_ADD**
    /// Bits `[23:0]`, mask `0x00ffffff`
    pub const INTERP1_ACCUM0_ADD_OFFSET: u32 = 0;
    pub const INTERP1_ACCUM0_ADD_MASK: u32 = 0x00ffffff;

    // ======== Field Accessors ========

    /// **INTERP1_ACCUM0_ADD** `[23:0]` (RW)
    #[inline(always)]
    pub fn interp1_accum0_add(&self) -> u32 {
        (self.value.read_bits() & Self::INTERP1_ACCUM0_ADD_MASK) >> Self::INTERP1_ACCUM0_ADD_OFFSET
    }

    /// Write **INTERP1_ACCUM0_ADD** `[23:0]` (RW)
    #[inline(always)]
    pub fn write_interp1_accum0_add(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::INTERP1_ACCUM0_ADD_MASK) | ((value << Self::INTERP1_ACCUM0_ADD_OFFSET) & Self::INTERP1_ACCUM0_ADD_MASK));
    }

}

/// INTERP1_ACCUM0_ADD value type for batch read/modify/write operations
///
/// Use `interp1accum0add.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `interp1accum0add.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Interp1Accum0AddVal {
    bits: u32,
}

impl Interp1Accum0AddVal {
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

    /// Read **INTERP1_ACCUM0_ADD** `[23:0]`
    #[inline(always)]
    pub const fn interp1_accum0_add(&self) -> u32 {
        (self.bits & Interp1Accum0Add::INTERP1_ACCUM0_ADD_MASK) >> Interp1Accum0Add::INTERP1_ACCUM0_ADD_OFFSET
    }

    /// Set **INTERP1_ACCUM0_ADD** `[23:0]`
    #[inline(always)]
    pub const fn with_interp1_accum0_add(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Interp1Accum0Add::INTERP1_ACCUM0_ADD_MASK) | ((value << Interp1Accum0Add::INTERP1_ACCUM0_ADD_OFFSET) & Interp1Accum0Add::INTERP1_ACCUM0_ADD_MASK)
        }
    }

}

impl Default for Interp1Accum0AddVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Interp1Accum0Add {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Interp1Accum0AddVal {
        Interp1Accum0AddVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Interp1Accum0AddVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Interp1Accum0AddVal) -> Interp1Accum0AddVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// INTERP1_ACCUM1_ADD Register
///
/// Values written here are atomically added to ACCUM1
/// Reading yields lane 1's raw shift and mask value (BASE1 not added).
///
/// Address offset: 0x00f8
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **INTERP1_ACCUM1_ADD** `[23:0]` (RW)
#[repr(transparent)]
pub struct Interp1Accum1Add {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Interp1Accum1Add {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Interp1Accum1Add {
    // ======== Field Constants ========

    /// **INTERP1_ACCUM1_ADD**
    /// Bits `[23:0]`, mask `0x00ffffff`
    pub const INTERP1_ACCUM1_ADD_OFFSET: u32 = 0;
    pub const INTERP1_ACCUM1_ADD_MASK: u32 = 0x00ffffff;

    // ======== Field Accessors ========

    /// **INTERP1_ACCUM1_ADD** `[23:0]` (RW)
    #[inline(always)]
    pub fn interp1_accum1_add(&self) -> u32 {
        (self.value.read_bits() & Self::INTERP1_ACCUM1_ADD_MASK) >> Self::INTERP1_ACCUM1_ADD_OFFSET
    }

    /// Write **INTERP1_ACCUM1_ADD** `[23:0]` (RW)
    #[inline(always)]
    pub fn write_interp1_accum1_add(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::INTERP1_ACCUM1_ADD_MASK) | ((value << Self::INTERP1_ACCUM1_ADD_OFFSET) & Self::INTERP1_ACCUM1_ADD_MASK));
    }

}

/// INTERP1_ACCUM1_ADD value type for batch read/modify/write operations
///
/// Use `interp1accum1add.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `interp1accum1add.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Interp1Accum1AddVal {
    bits: u32,
}

impl Interp1Accum1AddVal {
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

    /// Read **INTERP1_ACCUM1_ADD** `[23:0]`
    #[inline(always)]
    pub const fn interp1_accum1_add(&self) -> u32 {
        (self.bits & Interp1Accum1Add::INTERP1_ACCUM1_ADD_MASK) >> Interp1Accum1Add::INTERP1_ACCUM1_ADD_OFFSET
    }

    /// Set **INTERP1_ACCUM1_ADD** `[23:0]`
    #[inline(always)]
    pub const fn with_interp1_accum1_add(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Interp1Accum1Add::INTERP1_ACCUM1_ADD_MASK) | ((value << Interp1Accum1Add::INTERP1_ACCUM1_ADD_OFFSET) & Interp1Accum1Add::INTERP1_ACCUM1_ADD_MASK)
        }
    }

}

impl Default for Interp1Accum1AddVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Interp1Accum1Add {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Interp1Accum1AddVal {
        Interp1Accum1AddVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Interp1Accum1AddVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Interp1Accum1AddVal) -> Interp1Accum1AddVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// INTERP1_BASE_1AND0 Register
///
/// On write, the lower 16 bits go to BASE0, upper bits to BASE1 simultaneously.
/// Each half is sign-extended to 32 bits if that lane's SIGNED flag is set.
///
/// Address offset: 0x00fc
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **INTERP1_BASE_1AND0** `[31:0]` (WO)
#[repr(transparent)]
pub struct Interp1Base1and0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Interp1Base1and0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Interp1Base1and0 {
    // ======== Field Constants ========

    /// **INTERP1_BASE_1AND0**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const INTERP1_BASE_1AND0_OFFSET: u32 = 0;
    pub const INTERP1_BASE_1AND0_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **INTERP1_BASE_1AND0** `[31:0]` (WO)
    /// Write **INTERP1_BASE_1AND0** `[31:0]` (WO)
    #[inline(always)]
    pub fn write_interp1_base_1and0(&self, value: u32) {
        self.value.write_bits((value << Self::INTERP1_BASE_1AND0_OFFSET) & Self::INTERP1_BASE_1AND0_MASK);
    }

}

/// INTERP1_BASE_1AND0 value type for batch read/modify/write operations
///
/// Use `interp1base1and0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `interp1base1and0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Interp1Base1and0Val {
    bits: u32,
}

impl Interp1Base1and0Val {
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

    /// Set **INTERP1_BASE_1AND0** `[31:0]`
    #[inline(always)]
    pub const fn with_interp1_base_1and0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Interp1Base1and0::INTERP1_BASE_1AND0_MASK) | ((value << Interp1Base1and0::INTERP1_BASE_1AND0_OFFSET) & Interp1Base1and0::INTERP1_BASE_1AND0_MASK)
        }
    }

}

impl Default for Interp1Base1and0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Interp1Base1and0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Interp1Base1and0Val {
        Interp1Base1and0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Interp1Base1and0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Interp1Base1and0Val) -> Interp1Base1and0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SPINLOCK0 Register
///
/// Reading from a spinlock address will:
/// - Return 0 if lock is already locked
/// - Otherwise return nonzero, and simultaneously claim the lock
/// Writing (any value) releases the lock.
/// If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.
/// The value returned on success is 0x1 << lock number.
///
/// Address offset: 0x0100
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SPINLOCK0** `[31:0]` (RW)
#[repr(transparent)]
pub struct Spinlock0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Spinlock0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Spinlock0 {
    // ======== Field Constants ========

    /// **SPINLOCK0**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SPINLOCK0_OFFSET: u32 = 0;
    pub const SPINLOCK0_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SPINLOCK0** `[31:0]` (RW)
    #[inline(always)]
    pub fn spinlock0(&self) -> u32 {
        (self.value.read_bits() & Self::SPINLOCK0_MASK) >> Self::SPINLOCK0_OFFSET
    }

    /// Write **SPINLOCK0** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_spinlock0(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SPINLOCK0_MASK) | ((value << Self::SPINLOCK0_OFFSET) & Self::SPINLOCK0_MASK));
    }

}

/// SPINLOCK0 value type for batch read/modify/write operations
///
/// Use `spinlock0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spinlock0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Spinlock0Val {
    bits: u32,
}

impl Spinlock0Val {
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

    /// Read **SPINLOCK0** `[31:0]`
    #[inline(always)]
    pub const fn spinlock0(&self) -> u32 {
        (self.bits & Spinlock0::SPINLOCK0_MASK) >> Spinlock0::SPINLOCK0_OFFSET
    }

    /// Set **SPINLOCK0** `[31:0]`
    #[inline(always)]
    pub const fn with_spinlock0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Spinlock0::SPINLOCK0_MASK) | ((value << Spinlock0::SPINLOCK0_OFFSET) & Spinlock0::SPINLOCK0_MASK)
        }
    }

}

impl Default for Spinlock0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Spinlock0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Spinlock0Val {
        Spinlock0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Spinlock0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Spinlock0Val) -> Spinlock0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SPINLOCK1 Register
///
/// Reading from a spinlock address will:
/// - Return 0 if lock is already locked
/// - Otherwise return nonzero, and simultaneously claim the lock
/// Writing (any value) releases the lock.
/// If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.
/// The value returned on success is 0x1 << lock number.
///
/// Address offset: 0x0104
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SPINLOCK1** `[31:0]` (RW)
#[repr(transparent)]
pub struct Spinlock1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Spinlock1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Spinlock1 {
    // ======== Field Constants ========

    /// **SPINLOCK1**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SPINLOCK1_OFFSET: u32 = 0;
    pub const SPINLOCK1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SPINLOCK1** `[31:0]` (RW)
    #[inline(always)]
    pub fn spinlock1(&self) -> u32 {
        (self.value.read_bits() & Self::SPINLOCK1_MASK) >> Self::SPINLOCK1_OFFSET
    }

    /// Write **SPINLOCK1** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_spinlock1(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SPINLOCK1_MASK) | ((value << Self::SPINLOCK1_OFFSET) & Self::SPINLOCK1_MASK));
    }

}

/// SPINLOCK1 value type for batch read/modify/write operations
///
/// Use `spinlock1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spinlock1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Spinlock1Val {
    bits: u32,
}

impl Spinlock1Val {
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

    /// Read **SPINLOCK1** `[31:0]`
    #[inline(always)]
    pub const fn spinlock1(&self) -> u32 {
        (self.bits & Spinlock1::SPINLOCK1_MASK) >> Spinlock1::SPINLOCK1_OFFSET
    }

    /// Set **SPINLOCK1** `[31:0]`
    #[inline(always)]
    pub const fn with_spinlock1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Spinlock1::SPINLOCK1_MASK) | ((value << Spinlock1::SPINLOCK1_OFFSET) & Spinlock1::SPINLOCK1_MASK)
        }
    }

}

impl Default for Spinlock1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Spinlock1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Spinlock1Val {
        Spinlock1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Spinlock1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Spinlock1Val) -> Spinlock1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SPINLOCK2 Register
///
/// Reading from a spinlock address will:
/// - Return 0 if lock is already locked
/// - Otherwise return nonzero, and simultaneously claim the lock
/// Writing (any value) releases the lock.
/// If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.
/// The value returned on success is 0x1 << lock number.
///
/// Address offset: 0x0108
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SPINLOCK2** `[31:0]` (RW)
#[repr(transparent)]
pub struct Spinlock2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Spinlock2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Spinlock2 {
    // ======== Field Constants ========

    /// **SPINLOCK2**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SPINLOCK2_OFFSET: u32 = 0;
    pub const SPINLOCK2_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SPINLOCK2** `[31:0]` (RW)
    #[inline(always)]
    pub fn spinlock2(&self) -> u32 {
        (self.value.read_bits() & Self::SPINLOCK2_MASK) >> Self::SPINLOCK2_OFFSET
    }

    /// Write **SPINLOCK2** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_spinlock2(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SPINLOCK2_MASK) | ((value << Self::SPINLOCK2_OFFSET) & Self::SPINLOCK2_MASK));
    }

}

/// SPINLOCK2 value type for batch read/modify/write operations
///
/// Use `spinlock2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spinlock2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Spinlock2Val {
    bits: u32,
}

impl Spinlock2Val {
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

    /// Read **SPINLOCK2** `[31:0]`
    #[inline(always)]
    pub const fn spinlock2(&self) -> u32 {
        (self.bits & Spinlock2::SPINLOCK2_MASK) >> Spinlock2::SPINLOCK2_OFFSET
    }

    /// Set **SPINLOCK2** `[31:0]`
    #[inline(always)]
    pub const fn with_spinlock2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Spinlock2::SPINLOCK2_MASK) | ((value << Spinlock2::SPINLOCK2_OFFSET) & Spinlock2::SPINLOCK2_MASK)
        }
    }

}

impl Default for Spinlock2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Spinlock2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Spinlock2Val {
        Spinlock2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Spinlock2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Spinlock2Val) -> Spinlock2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SPINLOCK3 Register
///
/// Reading from a spinlock address will:
/// - Return 0 if lock is already locked
/// - Otherwise return nonzero, and simultaneously claim the lock
/// Writing (any value) releases the lock.
/// If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.
/// The value returned on success is 0x1 << lock number.
///
/// Address offset: 0x010c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SPINLOCK3** `[31:0]` (RW)
#[repr(transparent)]
pub struct Spinlock3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Spinlock3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Spinlock3 {
    // ======== Field Constants ========

    /// **SPINLOCK3**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SPINLOCK3_OFFSET: u32 = 0;
    pub const SPINLOCK3_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SPINLOCK3** `[31:0]` (RW)
    #[inline(always)]
    pub fn spinlock3(&self) -> u32 {
        (self.value.read_bits() & Self::SPINLOCK3_MASK) >> Self::SPINLOCK3_OFFSET
    }

    /// Write **SPINLOCK3** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_spinlock3(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SPINLOCK3_MASK) | ((value << Self::SPINLOCK3_OFFSET) & Self::SPINLOCK3_MASK));
    }

}

/// SPINLOCK3 value type for batch read/modify/write operations
///
/// Use `spinlock3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spinlock3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Spinlock3Val {
    bits: u32,
}

impl Spinlock3Val {
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

    /// Read **SPINLOCK3** `[31:0]`
    #[inline(always)]
    pub const fn spinlock3(&self) -> u32 {
        (self.bits & Spinlock3::SPINLOCK3_MASK) >> Spinlock3::SPINLOCK3_OFFSET
    }

    /// Set **SPINLOCK3** `[31:0]`
    #[inline(always)]
    pub const fn with_spinlock3(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Spinlock3::SPINLOCK3_MASK) | ((value << Spinlock3::SPINLOCK3_OFFSET) & Spinlock3::SPINLOCK3_MASK)
        }
    }

}

impl Default for Spinlock3Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Spinlock3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Spinlock3Val {
        Spinlock3Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Spinlock3Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Spinlock3Val) -> Spinlock3Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SPINLOCK4 Register
///
/// Reading from a spinlock address will:
/// - Return 0 if lock is already locked
/// - Otherwise return nonzero, and simultaneously claim the lock
/// Writing (any value) releases the lock.
/// If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.
/// The value returned on success is 0x1 << lock number.
///
/// Address offset: 0x0110
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SPINLOCK4** `[31:0]` (RW)
#[repr(transparent)]
pub struct Spinlock4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Spinlock4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Spinlock4 {
    // ======== Field Constants ========

    /// **SPINLOCK4**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SPINLOCK4_OFFSET: u32 = 0;
    pub const SPINLOCK4_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SPINLOCK4** `[31:0]` (RW)
    #[inline(always)]
    pub fn spinlock4(&self) -> u32 {
        (self.value.read_bits() & Self::SPINLOCK4_MASK) >> Self::SPINLOCK4_OFFSET
    }

    /// Write **SPINLOCK4** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_spinlock4(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SPINLOCK4_MASK) | ((value << Self::SPINLOCK4_OFFSET) & Self::SPINLOCK4_MASK));
    }

}

/// SPINLOCK4 value type for batch read/modify/write operations
///
/// Use `spinlock4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spinlock4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Spinlock4Val {
    bits: u32,
}

impl Spinlock4Val {
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

    /// Read **SPINLOCK4** `[31:0]`
    #[inline(always)]
    pub const fn spinlock4(&self) -> u32 {
        (self.bits & Spinlock4::SPINLOCK4_MASK) >> Spinlock4::SPINLOCK4_OFFSET
    }

    /// Set **SPINLOCK4** `[31:0]`
    #[inline(always)]
    pub const fn with_spinlock4(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Spinlock4::SPINLOCK4_MASK) | ((value << Spinlock4::SPINLOCK4_OFFSET) & Spinlock4::SPINLOCK4_MASK)
        }
    }

}

impl Default for Spinlock4Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Spinlock4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Spinlock4Val {
        Spinlock4Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Spinlock4Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Spinlock4Val) -> Spinlock4Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SPINLOCK5 Register
///
/// Reading from a spinlock address will:
/// - Return 0 if lock is already locked
/// - Otherwise return nonzero, and simultaneously claim the lock
/// Writing (any value) releases the lock.
/// If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.
/// The value returned on success is 0x1 << lock number.
///
/// Address offset: 0x0114
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SPINLOCK5** `[31:0]` (RW)
#[repr(transparent)]
pub struct Spinlock5 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Spinlock5 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Spinlock5 {
    // ======== Field Constants ========

    /// **SPINLOCK5**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SPINLOCK5_OFFSET: u32 = 0;
    pub const SPINLOCK5_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SPINLOCK5** `[31:0]` (RW)
    #[inline(always)]
    pub fn spinlock5(&self) -> u32 {
        (self.value.read_bits() & Self::SPINLOCK5_MASK) >> Self::SPINLOCK5_OFFSET
    }

    /// Write **SPINLOCK5** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_spinlock5(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SPINLOCK5_MASK) | ((value << Self::SPINLOCK5_OFFSET) & Self::SPINLOCK5_MASK));
    }

}

/// SPINLOCK5 value type for batch read/modify/write operations
///
/// Use `spinlock5.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spinlock5.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Spinlock5Val {
    bits: u32,
}

impl Spinlock5Val {
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

    /// Read **SPINLOCK5** `[31:0]`
    #[inline(always)]
    pub const fn spinlock5(&self) -> u32 {
        (self.bits & Spinlock5::SPINLOCK5_MASK) >> Spinlock5::SPINLOCK5_OFFSET
    }

    /// Set **SPINLOCK5** `[31:0]`
    #[inline(always)]
    pub const fn with_spinlock5(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Spinlock5::SPINLOCK5_MASK) | ((value << Spinlock5::SPINLOCK5_OFFSET) & Spinlock5::SPINLOCK5_MASK)
        }
    }

}

impl Default for Spinlock5Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Spinlock5 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Spinlock5Val {
        Spinlock5Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Spinlock5Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Spinlock5Val) -> Spinlock5Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SPINLOCK6 Register
///
/// Reading from a spinlock address will:
/// - Return 0 if lock is already locked
/// - Otherwise return nonzero, and simultaneously claim the lock
/// Writing (any value) releases the lock.
/// If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.
/// The value returned on success is 0x1 << lock number.
///
/// Address offset: 0x0118
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SPINLOCK6** `[31:0]` (RW)
#[repr(transparent)]
pub struct Spinlock6 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Spinlock6 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Spinlock6 {
    // ======== Field Constants ========

    /// **SPINLOCK6**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SPINLOCK6_OFFSET: u32 = 0;
    pub const SPINLOCK6_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SPINLOCK6** `[31:0]` (RW)
    #[inline(always)]
    pub fn spinlock6(&self) -> u32 {
        (self.value.read_bits() & Self::SPINLOCK6_MASK) >> Self::SPINLOCK6_OFFSET
    }

    /// Write **SPINLOCK6** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_spinlock6(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SPINLOCK6_MASK) | ((value << Self::SPINLOCK6_OFFSET) & Self::SPINLOCK6_MASK));
    }

}

/// SPINLOCK6 value type for batch read/modify/write operations
///
/// Use `spinlock6.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spinlock6.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Spinlock6Val {
    bits: u32,
}

impl Spinlock6Val {
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

    /// Read **SPINLOCK6** `[31:0]`
    #[inline(always)]
    pub const fn spinlock6(&self) -> u32 {
        (self.bits & Spinlock6::SPINLOCK6_MASK) >> Spinlock6::SPINLOCK6_OFFSET
    }

    /// Set **SPINLOCK6** `[31:0]`
    #[inline(always)]
    pub const fn with_spinlock6(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Spinlock6::SPINLOCK6_MASK) | ((value << Spinlock6::SPINLOCK6_OFFSET) & Spinlock6::SPINLOCK6_MASK)
        }
    }

}

impl Default for Spinlock6Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Spinlock6 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Spinlock6Val {
        Spinlock6Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Spinlock6Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Spinlock6Val) -> Spinlock6Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SPINLOCK7 Register
///
/// Reading from a spinlock address will:
/// - Return 0 if lock is already locked
/// - Otherwise return nonzero, and simultaneously claim the lock
/// Writing (any value) releases the lock.
/// If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.
/// The value returned on success is 0x1 << lock number.
///
/// Address offset: 0x011c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SPINLOCK7** `[31:0]` (RW)
#[repr(transparent)]
pub struct Spinlock7 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Spinlock7 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Spinlock7 {
    // ======== Field Constants ========

    /// **SPINLOCK7**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SPINLOCK7_OFFSET: u32 = 0;
    pub const SPINLOCK7_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SPINLOCK7** `[31:0]` (RW)
    #[inline(always)]
    pub fn spinlock7(&self) -> u32 {
        (self.value.read_bits() & Self::SPINLOCK7_MASK) >> Self::SPINLOCK7_OFFSET
    }

    /// Write **SPINLOCK7** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_spinlock7(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SPINLOCK7_MASK) | ((value << Self::SPINLOCK7_OFFSET) & Self::SPINLOCK7_MASK));
    }

}

/// SPINLOCK7 value type for batch read/modify/write operations
///
/// Use `spinlock7.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spinlock7.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Spinlock7Val {
    bits: u32,
}

impl Spinlock7Val {
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

    /// Read **SPINLOCK7** `[31:0]`
    #[inline(always)]
    pub const fn spinlock7(&self) -> u32 {
        (self.bits & Spinlock7::SPINLOCK7_MASK) >> Spinlock7::SPINLOCK7_OFFSET
    }

    /// Set **SPINLOCK7** `[31:0]`
    #[inline(always)]
    pub const fn with_spinlock7(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Spinlock7::SPINLOCK7_MASK) | ((value << Spinlock7::SPINLOCK7_OFFSET) & Spinlock7::SPINLOCK7_MASK)
        }
    }

}

impl Default for Spinlock7Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Spinlock7 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Spinlock7Val {
        Spinlock7Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Spinlock7Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Spinlock7Val) -> Spinlock7Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SPINLOCK8 Register
///
/// Reading from a spinlock address will:
/// - Return 0 if lock is already locked
/// - Otherwise return nonzero, and simultaneously claim the lock
/// Writing (any value) releases the lock.
/// If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.
/// The value returned on success is 0x1 << lock number.
///
/// Address offset: 0x0120
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SPINLOCK8** `[31:0]` (RW)
#[repr(transparent)]
pub struct Spinlock8 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Spinlock8 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Spinlock8 {
    // ======== Field Constants ========

    /// **SPINLOCK8**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SPINLOCK8_OFFSET: u32 = 0;
    pub const SPINLOCK8_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SPINLOCK8** `[31:0]` (RW)
    #[inline(always)]
    pub fn spinlock8(&self) -> u32 {
        (self.value.read_bits() & Self::SPINLOCK8_MASK) >> Self::SPINLOCK8_OFFSET
    }

    /// Write **SPINLOCK8** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_spinlock8(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SPINLOCK8_MASK) | ((value << Self::SPINLOCK8_OFFSET) & Self::SPINLOCK8_MASK));
    }

}

/// SPINLOCK8 value type for batch read/modify/write operations
///
/// Use `spinlock8.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spinlock8.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Spinlock8Val {
    bits: u32,
}

impl Spinlock8Val {
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

    /// Read **SPINLOCK8** `[31:0]`
    #[inline(always)]
    pub const fn spinlock8(&self) -> u32 {
        (self.bits & Spinlock8::SPINLOCK8_MASK) >> Spinlock8::SPINLOCK8_OFFSET
    }

    /// Set **SPINLOCK8** `[31:0]`
    #[inline(always)]
    pub const fn with_spinlock8(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Spinlock8::SPINLOCK8_MASK) | ((value << Spinlock8::SPINLOCK8_OFFSET) & Spinlock8::SPINLOCK8_MASK)
        }
    }

}

impl Default for Spinlock8Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Spinlock8 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Spinlock8Val {
        Spinlock8Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Spinlock8Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Spinlock8Val) -> Spinlock8Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SPINLOCK9 Register
///
/// Reading from a spinlock address will:
/// - Return 0 if lock is already locked
/// - Otherwise return nonzero, and simultaneously claim the lock
/// Writing (any value) releases the lock.
/// If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.
/// The value returned on success is 0x1 << lock number.
///
/// Address offset: 0x0124
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SPINLOCK9** `[31:0]` (RW)
#[repr(transparent)]
pub struct Spinlock9 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Spinlock9 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Spinlock9 {
    // ======== Field Constants ========

    /// **SPINLOCK9**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SPINLOCK9_OFFSET: u32 = 0;
    pub const SPINLOCK9_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SPINLOCK9** `[31:0]` (RW)
    #[inline(always)]
    pub fn spinlock9(&self) -> u32 {
        (self.value.read_bits() & Self::SPINLOCK9_MASK) >> Self::SPINLOCK9_OFFSET
    }

    /// Write **SPINLOCK9** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_spinlock9(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SPINLOCK9_MASK) | ((value << Self::SPINLOCK9_OFFSET) & Self::SPINLOCK9_MASK));
    }

}

/// SPINLOCK9 value type for batch read/modify/write operations
///
/// Use `spinlock9.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spinlock9.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Spinlock9Val {
    bits: u32,
}

impl Spinlock9Val {
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

    /// Read **SPINLOCK9** `[31:0]`
    #[inline(always)]
    pub const fn spinlock9(&self) -> u32 {
        (self.bits & Spinlock9::SPINLOCK9_MASK) >> Spinlock9::SPINLOCK9_OFFSET
    }

    /// Set **SPINLOCK9** `[31:0]`
    #[inline(always)]
    pub const fn with_spinlock9(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Spinlock9::SPINLOCK9_MASK) | ((value << Spinlock9::SPINLOCK9_OFFSET) & Spinlock9::SPINLOCK9_MASK)
        }
    }

}

impl Default for Spinlock9Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Spinlock9 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Spinlock9Val {
        Spinlock9Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Spinlock9Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Spinlock9Val) -> Spinlock9Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SPINLOCK10 Register
///
/// Reading from a spinlock address will:
/// - Return 0 if lock is already locked
/// - Otherwise return nonzero, and simultaneously claim the lock
/// Writing (any value) releases the lock.
/// If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.
/// The value returned on success is 0x1 << lock number.
///
/// Address offset: 0x0128
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SPINLOCK10** `[31:0]` (RW)
#[repr(transparent)]
pub struct Spinlock10 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Spinlock10 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Spinlock10 {
    // ======== Field Constants ========

    /// **SPINLOCK10**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SPINLOCK10_OFFSET: u32 = 0;
    pub const SPINLOCK10_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SPINLOCK10** `[31:0]` (RW)
    #[inline(always)]
    pub fn spinlock10(&self) -> u32 {
        (self.value.read_bits() & Self::SPINLOCK10_MASK) >> Self::SPINLOCK10_OFFSET
    }

    /// Write **SPINLOCK10** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_spinlock10(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SPINLOCK10_MASK) | ((value << Self::SPINLOCK10_OFFSET) & Self::SPINLOCK10_MASK));
    }

}

/// SPINLOCK10 value type for batch read/modify/write operations
///
/// Use `spinlock10.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spinlock10.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Spinlock10Val {
    bits: u32,
}

impl Spinlock10Val {
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

    /// Read **SPINLOCK10** `[31:0]`
    #[inline(always)]
    pub const fn spinlock10(&self) -> u32 {
        (self.bits & Spinlock10::SPINLOCK10_MASK) >> Spinlock10::SPINLOCK10_OFFSET
    }

    /// Set **SPINLOCK10** `[31:0]`
    #[inline(always)]
    pub const fn with_spinlock10(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Spinlock10::SPINLOCK10_MASK) | ((value << Spinlock10::SPINLOCK10_OFFSET) & Spinlock10::SPINLOCK10_MASK)
        }
    }

}

impl Default for Spinlock10Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Spinlock10 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Spinlock10Val {
        Spinlock10Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Spinlock10Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Spinlock10Val) -> Spinlock10Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SPINLOCK11 Register
///
/// Reading from a spinlock address will:
/// - Return 0 if lock is already locked
/// - Otherwise return nonzero, and simultaneously claim the lock
/// Writing (any value) releases the lock.
/// If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.
/// The value returned on success is 0x1 << lock number.
///
/// Address offset: 0x012c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SPINLOCK11** `[31:0]` (RW)
#[repr(transparent)]
pub struct Spinlock11 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Spinlock11 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Spinlock11 {
    // ======== Field Constants ========

    /// **SPINLOCK11**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SPINLOCK11_OFFSET: u32 = 0;
    pub const SPINLOCK11_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SPINLOCK11** `[31:0]` (RW)
    #[inline(always)]
    pub fn spinlock11(&self) -> u32 {
        (self.value.read_bits() & Self::SPINLOCK11_MASK) >> Self::SPINLOCK11_OFFSET
    }

    /// Write **SPINLOCK11** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_spinlock11(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SPINLOCK11_MASK) | ((value << Self::SPINLOCK11_OFFSET) & Self::SPINLOCK11_MASK));
    }

}

/// SPINLOCK11 value type for batch read/modify/write operations
///
/// Use `spinlock11.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spinlock11.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Spinlock11Val {
    bits: u32,
}

impl Spinlock11Val {
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

    /// Read **SPINLOCK11** `[31:0]`
    #[inline(always)]
    pub const fn spinlock11(&self) -> u32 {
        (self.bits & Spinlock11::SPINLOCK11_MASK) >> Spinlock11::SPINLOCK11_OFFSET
    }

    /// Set **SPINLOCK11** `[31:0]`
    #[inline(always)]
    pub const fn with_spinlock11(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Spinlock11::SPINLOCK11_MASK) | ((value << Spinlock11::SPINLOCK11_OFFSET) & Spinlock11::SPINLOCK11_MASK)
        }
    }

}

impl Default for Spinlock11Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Spinlock11 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Spinlock11Val {
        Spinlock11Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Spinlock11Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Spinlock11Val) -> Spinlock11Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SPINLOCK12 Register
///
/// Reading from a spinlock address will:
/// - Return 0 if lock is already locked
/// - Otherwise return nonzero, and simultaneously claim the lock
/// Writing (any value) releases the lock.
/// If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.
/// The value returned on success is 0x1 << lock number.
///
/// Address offset: 0x0130
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SPINLOCK12** `[31:0]` (RW)
#[repr(transparent)]
pub struct Spinlock12 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Spinlock12 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Spinlock12 {
    // ======== Field Constants ========

    /// **SPINLOCK12**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SPINLOCK12_OFFSET: u32 = 0;
    pub const SPINLOCK12_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SPINLOCK12** `[31:0]` (RW)
    #[inline(always)]
    pub fn spinlock12(&self) -> u32 {
        (self.value.read_bits() & Self::SPINLOCK12_MASK) >> Self::SPINLOCK12_OFFSET
    }

    /// Write **SPINLOCK12** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_spinlock12(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SPINLOCK12_MASK) | ((value << Self::SPINLOCK12_OFFSET) & Self::SPINLOCK12_MASK));
    }

}

/// SPINLOCK12 value type for batch read/modify/write operations
///
/// Use `spinlock12.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spinlock12.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Spinlock12Val {
    bits: u32,
}

impl Spinlock12Val {
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

    /// Read **SPINLOCK12** `[31:0]`
    #[inline(always)]
    pub const fn spinlock12(&self) -> u32 {
        (self.bits & Spinlock12::SPINLOCK12_MASK) >> Spinlock12::SPINLOCK12_OFFSET
    }

    /// Set **SPINLOCK12** `[31:0]`
    #[inline(always)]
    pub const fn with_spinlock12(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Spinlock12::SPINLOCK12_MASK) | ((value << Spinlock12::SPINLOCK12_OFFSET) & Spinlock12::SPINLOCK12_MASK)
        }
    }

}

impl Default for Spinlock12Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Spinlock12 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Spinlock12Val {
        Spinlock12Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Spinlock12Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Spinlock12Val) -> Spinlock12Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SPINLOCK13 Register
///
/// Reading from a spinlock address will:
/// - Return 0 if lock is already locked
/// - Otherwise return nonzero, and simultaneously claim the lock
/// Writing (any value) releases the lock.
/// If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.
/// The value returned on success is 0x1 << lock number.
///
/// Address offset: 0x0134
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SPINLOCK13** `[31:0]` (RW)
#[repr(transparent)]
pub struct Spinlock13 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Spinlock13 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Spinlock13 {
    // ======== Field Constants ========

    /// **SPINLOCK13**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SPINLOCK13_OFFSET: u32 = 0;
    pub const SPINLOCK13_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SPINLOCK13** `[31:0]` (RW)
    #[inline(always)]
    pub fn spinlock13(&self) -> u32 {
        (self.value.read_bits() & Self::SPINLOCK13_MASK) >> Self::SPINLOCK13_OFFSET
    }

    /// Write **SPINLOCK13** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_spinlock13(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SPINLOCK13_MASK) | ((value << Self::SPINLOCK13_OFFSET) & Self::SPINLOCK13_MASK));
    }

}

/// SPINLOCK13 value type for batch read/modify/write operations
///
/// Use `spinlock13.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spinlock13.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Spinlock13Val {
    bits: u32,
}

impl Spinlock13Val {
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

    /// Read **SPINLOCK13** `[31:0]`
    #[inline(always)]
    pub const fn spinlock13(&self) -> u32 {
        (self.bits & Spinlock13::SPINLOCK13_MASK) >> Spinlock13::SPINLOCK13_OFFSET
    }

    /// Set **SPINLOCK13** `[31:0]`
    #[inline(always)]
    pub const fn with_spinlock13(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Spinlock13::SPINLOCK13_MASK) | ((value << Spinlock13::SPINLOCK13_OFFSET) & Spinlock13::SPINLOCK13_MASK)
        }
    }

}

impl Default for Spinlock13Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Spinlock13 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Spinlock13Val {
        Spinlock13Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Spinlock13Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Spinlock13Val) -> Spinlock13Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SPINLOCK14 Register
///
/// Reading from a spinlock address will:
/// - Return 0 if lock is already locked
/// - Otherwise return nonzero, and simultaneously claim the lock
/// Writing (any value) releases the lock.
/// If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.
/// The value returned on success is 0x1 << lock number.
///
/// Address offset: 0x0138
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SPINLOCK14** `[31:0]` (RW)
#[repr(transparent)]
pub struct Spinlock14 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Spinlock14 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Spinlock14 {
    // ======== Field Constants ========

    /// **SPINLOCK14**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SPINLOCK14_OFFSET: u32 = 0;
    pub const SPINLOCK14_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SPINLOCK14** `[31:0]` (RW)
    #[inline(always)]
    pub fn spinlock14(&self) -> u32 {
        (self.value.read_bits() & Self::SPINLOCK14_MASK) >> Self::SPINLOCK14_OFFSET
    }

    /// Write **SPINLOCK14** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_spinlock14(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SPINLOCK14_MASK) | ((value << Self::SPINLOCK14_OFFSET) & Self::SPINLOCK14_MASK));
    }

}

/// SPINLOCK14 value type for batch read/modify/write operations
///
/// Use `spinlock14.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spinlock14.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Spinlock14Val {
    bits: u32,
}

impl Spinlock14Val {
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

    /// Read **SPINLOCK14** `[31:0]`
    #[inline(always)]
    pub const fn spinlock14(&self) -> u32 {
        (self.bits & Spinlock14::SPINLOCK14_MASK) >> Spinlock14::SPINLOCK14_OFFSET
    }

    /// Set **SPINLOCK14** `[31:0]`
    #[inline(always)]
    pub const fn with_spinlock14(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Spinlock14::SPINLOCK14_MASK) | ((value << Spinlock14::SPINLOCK14_OFFSET) & Spinlock14::SPINLOCK14_MASK)
        }
    }

}

impl Default for Spinlock14Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Spinlock14 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Spinlock14Val {
        Spinlock14Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Spinlock14Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Spinlock14Val) -> Spinlock14Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SPINLOCK15 Register
///
/// Reading from a spinlock address will:
/// - Return 0 if lock is already locked
/// - Otherwise return nonzero, and simultaneously claim the lock
/// Writing (any value) releases the lock.
/// If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.
/// The value returned on success is 0x1 << lock number.
///
/// Address offset: 0x013c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SPINLOCK15** `[31:0]` (RW)
#[repr(transparent)]
pub struct Spinlock15 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Spinlock15 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Spinlock15 {
    // ======== Field Constants ========

    /// **SPINLOCK15**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SPINLOCK15_OFFSET: u32 = 0;
    pub const SPINLOCK15_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SPINLOCK15** `[31:0]` (RW)
    #[inline(always)]
    pub fn spinlock15(&self) -> u32 {
        (self.value.read_bits() & Self::SPINLOCK15_MASK) >> Self::SPINLOCK15_OFFSET
    }

    /// Write **SPINLOCK15** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_spinlock15(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SPINLOCK15_MASK) | ((value << Self::SPINLOCK15_OFFSET) & Self::SPINLOCK15_MASK));
    }

}

/// SPINLOCK15 value type for batch read/modify/write operations
///
/// Use `spinlock15.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spinlock15.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Spinlock15Val {
    bits: u32,
}

impl Spinlock15Val {
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

    /// Read **SPINLOCK15** `[31:0]`
    #[inline(always)]
    pub const fn spinlock15(&self) -> u32 {
        (self.bits & Spinlock15::SPINLOCK15_MASK) >> Spinlock15::SPINLOCK15_OFFSET
    }

    /// Set **SPINLOCK15** `[31:0]`
    #[inline(always)]
    pub const fn with_spinlock15(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Spinlock15::SPINLOCK15_MASK) | ((value << Spinlock15::SPINLOCK15_OFFSET) & Spinlock15::SPINLOCK15_MASK)
        }
    }

}

impl Default for Spinlock15Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Spinlock15 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Spinlock15Val {
        Spinlock15Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Spinlock15Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Spinlock15Val) -> Spinlock15Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SPINLOCK16 Register
///
/// Reading from a spinlock address will:
/// - Return 0 if lock is already locked
/// - Otherwise return nonzero, and simultaneously claim the lock
/// Writing (any value) releases the lock.
/// If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.
/// The value returned on success is 0x1 << lock number.
///
/// Address offset: 0x0140
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SPINLOCK16** `[31:0]` (RW)
#[repr(transparent)]
pub struct Spinlock16 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Spinlock16 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Spinlock16 {
    // ======== Field Constants ========

    /// **SPINLOCK16**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SPINLOCK16_OFFSET: u32 = 0;
    pub const SPINLOCK16_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SPINLOCK16** `[31:0]` (RW)
    #[inline(always)]
    pub fn spinlock16(&self) -> u32 {
        (self.value.read_bits() & Self::SPINLOCK16_MASK) >> Self::SPINLOCK16_OFFSET
    }

    /// Write **SPINLOCK16** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_spinlock16(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SPINLOCK16_MASK) | ((value << Self::SPINLOCK16_OFFSET) & Self::SPINLOCK16_MASK));
    }

}

/// SPINLOCK16 value type for batch read/modify/write operations
///
/// Use `spinlock16.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spinlock16.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Spinlock16Val {
    bits: u32,
}

impl Spinlock16Val {
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

    /// Read **SPINLOCK16** `[31:0]`
    #[inline(always)]
    pub const fn spinlock16(&self) -> u32 {
        (self.bits & Spinlock16::SPINLOCK16_MASK) >> Spinlock16::SPINLOCK16_OFFSET
    }

    /// Set **SPINLOCK16** `[31:0]`
    #[inline(always)]
    pub const fn with_spinlock16(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Spinlock16::SPINLOCK16_MASK) | ((value << Spinlock16::SPINLOCK16_OFFSET) & Spinlock16::SPINLOCK16_MASK)
        }
    }

}

impl Default for Spinlock16Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Spinlock16 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Spinlock16Val {
        Spinlock16Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Spinlock16Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Spinlock16Val) -> Spinlock16Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SPINLOCK17 Register
///
/// Reading from a spinlock address will:
/// - Return 0 if lock is already locked
/// - Otherwise return nonzero, and simultaneously claim the lock
/// Writing (any value) releases the lock.
/// If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.
/// The value returned on success is 0x1 << lock number.
///
/// Address offset: 0x0144
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SPINLOCK17** `[31:0]` (RW)
#[repr(transparent)]
pub struct Spinlock17 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Spinlock17 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Spinlock17 {
    // ======== Field Constants ========

    /// **SPINLOCK17**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SPINLOCK17_OFFSET: u32 = 0;
    pub const SPINLOCK17_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SPINLOCK17** `[31:0]` (RW)
    #[inline(always)]
    pub fn spinlock17(&self) -> u32 {
        (self.value.read_bits() & Self::SPINLOCK17_MASK) >> Self::SPINLOCK17_OFFSET
    }

    /// Write **SPINLOCK17** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_spinlock17(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SPINLOCK17_MASK) | ((value << Self::SPINLOCK17_OFFSET) & Self::SPINLOCK17_MASK));
    }

}

/// SPINLOCK17 value type for batch read/modify/write operations
///
/// Use `spinlock17.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spinlock17.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Spinlock17Val {
    bits: u32,
}

impl Spinlock17Val {
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

    /// Read **SPINLOCK17** `[31:0]`
    #[inline(always)]
    pub const fn spinlock17(&self) -> u32 {
        (self.bits & Spinlock17::SPINLOCK17_MASK) >> Spinlock17::SPINLOCK17_OFFSET
    }

    /// Set **SPINLOCK17** `[31:0]`
    #[inline(always)]
    pub const fn with_spinlock17(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Spinlock17::SPINLOCK17_MASK) | ((value << Spinlock17::SPINLOCK17_OFFSET) & Spinlock17::SPINLOCK17_MASK)
        }
    }

}

impl Default for Spinlock17Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Spinlock17 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Spinlock17Val {
        Spinlock17Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Spinlock17Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Spinlock17Val) -> Spinlock17Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SPINLOCK18 Register
///
/// Reading from a spinlock address will:
/// - Return 0 if lock is already locked
/// - Otherwise return nonzero, and simultaneously claim the lock
/// Writing (any value) releases the lock.
/// If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.
/// The value returned on success is 0x1 << lock number.
///
/// Address offset: 0x0148
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SPINLOCK18** `[31:0]` (RW)
#[repr(transparent)]
pub struct Spinlock18 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Spinlock18 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Spinlock18 {
    // ======== Field Constants ========

    /// **SPINLOCK18**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SPINLOCK18_OFFSET: u32 = 0;
    pub const SPINLOCK18_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SPINLOCK18** `[31:0]` (RW)
    #[inline(always)]
    pub fn spinlock18(&self) -> u32 {
        (self.value.read_bits() & Self::SPINLOCK18_MASK) >> Self::SPINLOCK18_OFFSET
    }

    /// Write **SPINLOCK18** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_spinlock18(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SPINLOCK18_MASK) | ((value << Self::SPINLOCK18_OFFSET) & Self::SPINLOCK18_MASK));
    }

}

/// SPINLOCK18 value type for batch read/modify/write operations
///
/// Use `spinlock18.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spinlock18.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Spinlock18Val {
    bits: u32,
}

impl Spinlock18Val {
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

    /// Read **SPINLOCK18** `[31:0]`
    #[inline(always)]
    pub const fn spinlock18(&self) -> u32 {
        (self.bits & Spinlock18::SPINLOCK18_MASK) >> Spinlock18::SPINLOCK18_OFFSET
    }

    /// Set **SPINLOCK18** `[31:0]`
    #[inline(always)]
    pub const fn with_spinlock18(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Spinlock18::SPINLOCK18_MASK) | ((value << Spinlock18::SPINLOCK18_OFFSET) & Spinlock18::SPINLOCK18_MASK)
        }
    }

}

impl Default for Spinlock18Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Spinlock18 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Spinlock18Val {
        Spinlock18Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Spinlock18Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Spinlock18Val) -> Spinlock18Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SPINLOCK19 Register
///
/// Reading from a spinlock address will:
/// - Return 0 if lock is already locked
/// - Otherwise return nonzero, and simultaneously claim the lock
/// Writing (any value) releases the lock.
/// If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.
/// The value returned on success is 0x1 << lock number.
///
/// Address offset: 0x014c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SPINLOCK19** `[31:0]` (RW)
#[repr(transparent)]
pub struct Spinlock19 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Spinlock19 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Spinlock19 {
    // ======== Field Constants ========

    /// **SPINLOCK19**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SPINLOCK19_OFFSET: u32 = 0;
    pub const SPINLOCK19_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SPINLOCK19** `[31:0]` (RW)
    #[inline(always)]
    pub fn spinlock19(&self) -> u32 {
        (self.value.read_bits() & Self::SPINLOCK19_MASK) >> Self::SPINLOCK19_OFFSET
    }

    /// Write **SPINLOCK19** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_spinlock19(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SPINLOCK19_MASK) | ((value << Self::SPINLOCK19_OFFSET) & Self::SPINLOCK19_MASK));
    }

}

/// SPINLOCK19 value type for batch read/modify/write operations
///
/// Use `spinlock19.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spinlock19.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Spinlock19Val {
    bits: u32,
}

impl Spinlock19Val {
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

    /// Read **SPINLOCK19** `[31:0]`
    #[inline(always)]
    pub const fn spinlock19(&self) -> u32 {
        (self.bits & Spinlock19::SPINLOCK19_MASK) >> Spinlock19::SPINLOCK19_OFFSET
    }

    /// Set **SPINLOCK19** `[31:0]`
    #[inline(always)]
    pub const fn with_spinlock19(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Spinlock19::SPINLOCK19_MASK) | ((value << Spinlock19::SPINLOCK19_OFFSET) & Spinlock19::SPINLOCK19_MASK)
        }
    }

}

impl Default for Spinlock19Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Spinlock19 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Spinlock19Val {
        Spinlock19Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Spinlock19Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Spinlock19Val) -> Spinlock19Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SPINLOCK20 Register
///
/// Reading from a spinlock address will:
/// - Return 0 if lock is already locked
/// - Otherwise return nonzero, and simultaneously claim the lock
/// Writing (any value) releases the lock.
/// If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.
/// The value returned on success is 0x1 << lock number.
///
/// Address offset: 0x0150
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SPINLOCK20** `[31:0]` (RW)
#[repr(transparent)]
pub struct Spinlock20 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Spinlock20 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Spinlock20 {
    // ======== Field Constants ========

    /// **SPINLOCK20**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SPINLOCK20_OFFSET: u32 = 0;
    pub const SPINLOCK20_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SPINLOCK20** `[31:0]` (RW)
    #[inline(always)]
    pub fn spinlock20(&self) -> u32 {
        (self.value.read_bits() & Self::SPINLOCK20_MASK) >> Self::SPINLOCK20_OFFSET
    }

    /// Write **SPINLOCK20** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_spinlock20(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SPINLOCK20_MASK) | ((value << Self::SPINLOCK20_OFFSET) & Self::SPINLOCK20_MASK));
    }

}

/// SPINLOCK20 value type for batch read/modify/write operations
///
/// Use `spinlock20.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spinlock20.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Spinlock20Val {
    bits: u32,
}

impl Spinlock20Val {
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

    /// Read **SPINLOCK20** `[31:0]`
    #[inline(always)]
    pub const fn spinlock20(&self) -> u32 {
        (self.bits & Spinlock20::SPINLOCK20_MASK) >> Spinlock20::SPINLOCK20_OFFSET
    }

    /// Set **SPINLOCK20** `[31:0]`
    #[inline(always)]
    pub const fn with_spinlock20(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Spinlock20::SPINLOCK20_MASK) | ((value << Spinlock20::SPINLOCK20_OFFSET) & Spinlock20::SPINLOCK20_MASK)
        }
    }

}

impl Default for Spinlock20Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Spinlock20 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Spinlock20Val {
        Spinlock20Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Spinlock20Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Spinlock20Val) -> Spinlock20Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SPINLOCK21 Register
///
/// Reading from a spinlock address will:
/// - Return 0 if lock is already locked
/// - Otherwise return nonzero, and simultaneously claim the lock
/// Writing (any value) releases the lock.
/// If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.
/// The value returned on success is 0x1 << lock number.
///
/// Address offset: 0x0154
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SPINLOCK21** `[31:0]` (RW)
#[repr(transparent)]
pub struct Spinlock21 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Spinlock21 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Spinlock21 {
    // ======== Field Constants ========

    /// **SPINLOCK21**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SPINLOCK21_OFFSET: u32 = 0;
    pub const SPINLOCK21_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SPINLOCK21** `[31:0]` (RW)
    #[inline(always)]
    pub fn spinlock21(&self) -> u32 {
        (self.value.read_bits() & Self::SPINLOCK21_MASK) >> Self::SPINLOCK21_OFFSET
    }

    /// Write **SPINLOCK21** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_spinlock21(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SPINLOCK21_MASK) | ((value << Self::SPINLOCK21_OFFSET) & Self::SPINLOCK21_MASK));
    }

}

/// SPINLOCK21 value type for batch read/modify/write operations
///
/// Use `spinlock21.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spinlock21.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Spinlock21Val {
    bits: u32,
}

impl Spinlock21Val {
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

    /// Read **SPINLOCK21** `[31:0]`
    #[inline(always)]
    pub const fn spinlock21(&self) -> u32 {
        (self.bits & Spinlock21::SPINLOCK21_MASK) >> Spinlock21::SPINLOCK21_OFFSET
    }

    /// Set **SPINLOCK21** `[31:0]`
    #[inline(always)]
    pub const fn with_spinlock21(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Spinlock21::SPINLOCK21_MASK) | ((value << Spinlock21::SPINLOCK21_OFFSET) & Spinlock21::SPINLOCK21_MASK)
        }
    }

}

impl Default for Spinlock21Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Spinlock21 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Spinlock21Val {
        Spinlock21Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Spinlock21Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Spinlock21Val) -> Spinlock21Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SPINLOCK22 Register
///
/// Reading from a spinlock address will:
/// - Return 0 if lock is already locked
/// - Otherwise return nonzero, and simultaneously claim the lock
/// Writing (any value) releases the lock.
/// If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.
/// The value returned on success is 0x1 << lock number.
///
/// Address offset: 0x0158
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SPINLOCK22** `[31:0]` (RW)
#[repr(transparent)]
pub struct Spinlock22 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Spinlock22 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Spinlock22 {
    // ======== Field Constants ========

    /// **SPINLOCK22**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SPINLOCK22_OFFSET: u32 = 0;
    pub const SPINLOCK22_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SPINLOCK22** `[31:0]` (RW)
    #[inline(always)]
    pub fn spinlock22(&self) -> u32 {
        (self.value.read_bits() & Self::SPINLOCK22_MASK) >> Self::SPINLOCK22_OFFSET
    }

    /// Write **SPINLOCK22** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_spinlock22(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SPINLOCK22_MASK) | ((value << Self::SPINLOCK22_OFFSET) & Self::SPINLOCK22_MASK));
    }

}

/// SPINLOCK22 value type for batch read/modify/write operations
///
/// Use `spinlock22.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spinlock22.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Spinlock22Val {
    bits: u32,
}

impl Spinlock22Val {
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

    /// Read **SPINLOCK22** `[31:0]`
    #[inline(always)]
    pub const fn spinlock22(&self) -> u32 {
        (self.bits & Spinlock22::SPINLOCK22_MASK) >> Spinlock22::SPINLOCK22_OFFSET
    }

    /// Set **SPINLOCK22** `[31:0]`
    #[inline(always)]
    pub const fn with_spinlock22(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Spinlock22::SPINLOCK22_MASK) | ((value << Spinlock22::SPINLOCK22_OFFSET) & Spinlock22::SPINLOCK22_MASK)
        }
    }

}

impl Default for Spinlock22Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Spinlock22 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Spinlock22Val {
        Spinlock22Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Spinlock22Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Spinlock22Val) -> Spinlock22Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SPINLOCK23 Register
///
/// Reading from a spinlock address will:
/// - Return 0 if lock is already locked
/// - Otherwise return nonzero, and simultaneously claim the lock
/// Writing (any value) releases the lock.
/// If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.
/// The value returned on success is 0x1 << lock number.
///
/// Address offset: 0x015c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SPINLOCK23** `[31:0]` (RW)
#[repr(transparent)]
pub struct Spinlock23 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Spinlock23 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Spinlock23 {
    // ======== Field Constants ========

    /// **SPINLOCK23**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SPINLOCK23_OFFSET: u32 = 0;
    pub const SPINLOCK23_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SPINLOCK23** `[31:0]` (RW)
    #[inline(always)]
    pub fn spinlock23(&self) -> u32 {
        (self.value.read_bits() & Self::SPINLOCK23_MASK) >> Self::SPINLOCK23_OFFSET
    }

    /// Write **SPINLOCK23** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_spinlock23(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SPINLOCK23_MASK) | ((value << Self::SPINLOCK23_OFFSET) & Self::SPINLOCK23_MASK));
    }

}

/// SPINLOCK23 value type for batch read/modify/write operations
///
/// Use `spinlock23.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spinlock23.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Spinlock23Val {
    bits: u32,
}

impl Spinlock23Val {
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

    /// Read **SPINLOCK23** `[31:0]`
    #[inline(always)]
    pub const fn spinlock23(&self) -> u32 {
        (self.bits & Spinlock23::SPINLOCK23_MASK) >> Spinlock23::SPINLOCK23_OFFSET
    }

    /// Set **SPINLOCK23** `[31:0]`
    #[inline(always)]
    pub const fn with_spinlock23(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Spinlock23::SPINLOCK23_MASK) | ((value << Spinlock23::SPINLOCK23_OFFSET) & Spinlock23::SPINLOCK23_MASK)
        }
    }

}

impl Default for Spinlock23Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Spinlock23 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Spinlock23Val {
        Spinlock23Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Spinlock23Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Spinlock23Val) -> Spinlock23Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SPINLOCK24 Register
///
/// Reading from a spinlock address will:
/// - Return 0 if lock is already locked
/// - Otherwise return nonzero, and simultaneously claim the lock
/// Writing (any value) releases the lock.
/// If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.
/// The value returned on success is 0x1 << lock number.
///
/// Address offset: 0x0160
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SPINLOCK24** `[31:0]` (RW)
#[repr(transparent)]
pub struct Spinlock24 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Spinlock24 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Spinlock24 {
    // ======== Field Constants ========

    /// **SPINLOCK24**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SPINLOCK24_OFFSET: u32 = 0;
    pub const SPINLOCK24_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SPINLOCK24** `[31:0]` (RW)
    #[inline(always)]
    pub fn spinlock24(&self) -> u32 {
        (self.value.read_bits() & Self::SPINLOCK24_MASK) >> Self::SPINLOCK24_OFFSET
    }

    /// Write **SPINLOCK24** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_spinlock24(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SPINLOCK24_MASK) | ((value << Self::SPINLOCK24_OFFSET) & Self::SPINLOCK24_MASK));
    }

}

/// SPINLOCK24 value type for batch read/modify/write operations
///
/// Use `spinlock24.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spinlock24.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Spinlock24Val {
    bits: u32,
}

impl Spinlock24Val {
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

    /// Read **SPINLOCK24** `[31:0]`
    #[inline(always)]
    pub const fn spinlock24(&self) -> u32 {
        (self.bits & Spinlock24::SPINLOCK24_MASK) >> Spinlock24::SPINLOCK24_OFFSET
    }

    /// Set **SPINLOCK24** `[31:0]`
    #[inline(always)]
    pub const fn with_spinlock24(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Spinlock24::SPINLOCK24_MASK) | ((value << Spinlock24::SPINLOCK24_OFFSET) & Spinlock24::SPINLOCK24_MASK)
        }
    }

}

impl Default for Spinlock24Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Spinlock24 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Spinlock24Val {
        Spinlock24Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Spinlock24Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Spinlock24Val) -> Spinlock24Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SPINLOCK25 Register
///
/// Reading from a spinlock address will:
/// - Return 0 if lock is already locked
/// - Otherwise return nonzero, and simultaneously claim the lock
/// Writing (any value) releases the lock.
/// If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.
/// The value returned on success is 0x1 << lock number.
///
/// Address offset: 0x0164
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SPINLOCK25** `[31:0]` (RW)
#[repr(transparent)]
pub struct Spinlock25 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Spinlock25 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Spinlock25 {
    // ======== Field Constants ========

    /// **SPINLOCK25**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SPINLOCK25_OFFSET: u32 = 0;
    pub const SPINLOCK25_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SPINLOCK25** `[31:0]` (RW)
    #[inline(always)]
    pub fn spinlock25(&self) -> u32 {
        (self.value.read_bits() & Self::SPINLOCK25_MASK) >> Self::SPINLOCK25_OFFSET
    }

    /// Write **SPINLOCK25** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_spinlock25(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SPINLOCK25_MASK) | ((value << Self::SPINLOCK25_OFFSET) & Self::SPINLOCK25_MASK));
    }

}

/// SPINLOCK25 value type for batch read/modify/write operations
///
/// Use `spinlock25.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spinlock25.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Spinlock25Val {
    bits: u32,
}

impl Spinlock25Val {
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

    /// Read **SPINLOCK25** `[31:0]`
    #[inline(always)]
    pub const fn spinlock25(&self) -> u32 {
        (self.bits & Spinlock25::SPINLOCK25_MASK) >> Spinlock25::SPINLOCK25_OFFSET
    }

    /// Set **SPINLOCK25** `[31:0]`
    #[inline(always)]
    pub const fn with_spinlock25(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Spinlock25::SPINLOCK25_MASK) | ((value << Spinlock25::SPINLOCK25_OFFSET) & Spinlock25::SPINLOCK25_MASK)
        }
    }

}

impl Default for Spinlock25Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Spinlock25 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Spinlock25Val {
        Spinlock25Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Spinlock25Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Spinlock25Val) -> Spinlock25Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SPINLOCK26 Register
///
/// Reading from a spinlock address will:
/// - Return 0 if lock is already locked
/// - Otherwise return nonzero, and simultaneously claim the lock
/// Writing (any value) releases the lock.
/// If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.
/// The value returned on success is 0x1 << lock number.
///
/// Address offset: 0x0168
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SPINLOCK26** `[31:0]` (RW)
#[repr(transparent)]
pub struct Spinlock26 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Spinlock26 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Spinlock26 {
    // ======== Field Constants ========

    /// **SPINLOCK26**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SPINLOCK26_OFFSET: u32 = 0;
    pub const SPINLOCK26_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SPINLOCK26** `[31:0]` (RW)
    #[inline(always)]
    pub fn spinlock26(&self) -> u32 {
        (self.value.read_bits() & Self::SPINLOCK26_MASK) >> Self::SPINLOCK26_OFFSET
    }

    /// Write **SPINLOCK26** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_spinlock26(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SPINLOCK26_MASK) | ((value << Self::SPINLOCK26_OFFSET) & Self::SPINLOCK26_MASK));
    }

}

/// SPINLOCK26 value type for batch read/modify/write operations
///
/// Use `spinlock26.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spinlock26.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Spinlock26Val {
    bits: u32,
}

impl Spinlock26Val {
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

    /// Read **SPINLOCK26** `[31:0]`
    #[inline(always)]
    pub const fn spinlock26(&self) -> u32 {
        (self.bits & Spinlock26::SPINLOCK26_MASK) >> Spinlock26::SPINLOCK26_OFFSET
    }

    /// Set **SPINLOCK26** `[31:0]`
    #[inline(always)]
    pub const fn with_spinlock26(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Spinlock26::SPINLOCK26_MASK) | ((value << Spinlock26::SPINLOCK26_OFFSET) & Spinlock26::SPINLOCK26_MASK)
        }
    }

}

impl Default for Spinlock26Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Spinlock26 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Spinlock26Val {
        Spinlock26Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Spinlock26Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Spinlock26Val) -> Spinlock26Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SPINLOCK27 Register
///
/// Reading from a spinlock address will:
/// - Return 0 if lock is already locked
/// - Otherwise return nonzero, and simultaneously claim the lock
/// Writing (any value) releases the lock.
/// If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.
/// The value returned on success is 0x1 << lock number.
///
/// Address offset: 0x016c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SPINLOCK27** `[31:0]` (RW)
#[repr(transparent)]
pub struct Spinlock27 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Spinlock27 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Spinlock27 {
    // ======== Field Constants ========

    /// **SPINLOCK27**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SPINLOCK27_OFFSET: u32 = 0;
    pub const SPINLOCK27_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SPINLOCK27** `[31:0]` (RW)
    #[inline(always)]
    pub fn spinlock27(&self) -> u32 {
        (self.value.read_bits() & Self::SPINLOCK27_MASK) >> Self::SPINLOCK27_OFFSET
    }

    /// Write **SPINLOCK27** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_spinlock27(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SPINLOCK27_MASK) | ((value << Self::SPINLOCK27_OFFSET) & Self::SPINLOCK27_MASK));
    }

}

/// SPINLOCK27 value type for batch read/modify/write operations
///
/// Use `spinlock27.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spinlock27.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Spinlock27Val {
    bits: u32,
}

impl Spinlock27Val {
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

    /// Read **SPINLOCK27** `[31:0]`
    #[inline(always)]
    pub const fn spinlock27(&self) -> u32 {
        (self.bits & Spinlock27::SPINLOCK27_MASK) >> Spinlock27::SPINLOCK27_OFFSET
    }

    /// Set **SPINLOCK27** `[31:0]`
    #[inline(always)]
    pub const fn with_spinlock27(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Spinlock27::SPINLOCK27_MASK) | ((value << Spinlock27::SPINLOCK27_OFFSET) & Spinlock27::SPINLOCK27_MASK)
        }
    }

}

impl Default for Spinlock27Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Spinlock27 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Spinlock27Val {
        Spinlock27Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Spinlock27Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Spinlock27Val) -> Spinlock27Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SPINLOCK28 Register
///
/// Reading from a spinlock address will:
/// - Return 0 if lock is already locked
/// - Otherwise return nonzero, and simultaneously claim the lock
/// Writing (any value) releases the lock.
/// If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.
/// The value returned on success is 0x1 << lock number.
///
/// Address offset: 0x0170
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SPINLOCK28** `[31:0]` (RW)
#[repr(transparent)]
pub struct Spinlock28 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Spinlock28 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Spinlock28 {
    // ======== Field Constants ========

    /// **SPINLOCK28**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SPINLOCK28_OFFSET: u32 = 0;
    pub const SPINLOCK28_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SPINLOCK28** `[31:0]` (RW)
    #[inline(always)]
    pub fn spinlock28(&self) -> u32 {
        (self.value.read_bits() & Self::SPINLOCK28_MASK) >> Self::SPINLOCK28_OFFSET
    }

    /// Write **SPINLOCK28** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_spinlock28(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SPINLOCK28_MASK) | ((value << Self::SPINLOCK28_OFFSET) & Self::SPINLOCK28_MASK));
    }

}

/// SPINLOCK28 value type for batch read/modify/write operations
///
/// Use `spinlock28.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spinlock28.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Spinlock28Val {
    bits: u32,
}

impl Spinlock28Val {
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

    /// Read **SPINLOCK28** `[31:0]`
    #[inline(always)]
    pub const fn spinlock28(&self) -> u32 {
        (self.bits & Spinlock28::SPINLOCK28_MASK) >> Spinlock28::SPINLOCK28_OFFSET
    }

    /// Set **SPINLOCK28** `[31:0]`
    #[inline(always)]
    pub const fn with_spinlock28(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Spinlock28::SPINLOCK28_MASK) | ((value << Spinlock28::SPINLOCK28_OFFSET) & Spinlock28::SPINLOCK28_MASK)
        }
    }

}

impl Default for Spinlock28Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Spinlock28 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Spinlock28Val {
        Spinlock28Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Spinlock28Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Spinlock28Val) -> Spinlock28Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SPINLOCK29 Register
///
/// Reading from a spinlock address will:
/// - Return 0 if lock is already locked
/// - Otherwise return nonzero, and simultaneously claim the lock
/// Writing (any value) releases the lock.
/// If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.
/// The value returned on success is 0x1 << lock number.
///
/// Address offset: 0x0174
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SPINLOCK29** `[31:0]` (RW)
#[repr(transparent)]
pub struct Spinlock29 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Spinlock29 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Spinlock29 {
    // ======== Field Constants ========

    /// **SPINLOCK29**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SPINLOCK29_OFFSET: u32 = 0;
    pub const SPINLOCK29_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SPINLOCK29** `[31:0]` (RW)
    #[inline(always)]
    pub fn spinlock29(&self) -> u32 {
        (self.value.read_bits() & Self::SPINLOCK29_MASK) >> Self::SPINLOCK29_OFFSET
    }

    /// Write **SPINLOCK29** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_spinlock29(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SPINLOCK29_MASK) | ((value << Self::SPINLOCK29_OFFSET) & Self::SPINLOCK29_MASK));
    }

}

/// SPINLOCK29 value type for batch read/modify/write operations
///
/// Use `spinlock29.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spinlock29.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Spinlock29Val {
    bits: u32,
}

impl Spinlock29Val {
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

    /// Read **SPINLOCK29** `[31:0]`
    #[inline(always)]
    pub const fn spinlock29(&self) -> u32 {
        (self.bits & Spinlock29::SPINLOCK29_MASK) >> Spinlock29::SPINLOCK29_OFFSET
    }

    /// Set **SPINLOCK29** `[31:0]`
    #[inline(always)]
    pub const fn with_spinlock29(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Spinlock29::SPINLOCK29_MASK) | ((value << Spinlock29::SPINLOCK29_OFFSET) & Spinlock29::SPINLOCK29_MASK)
        }
    }

}

impl Default for Spinlock29Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Spinlock29 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Spinlock29Val {
        Spinlock29Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Spinlock29Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Spinlock29Val) -> Spinlock29Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SPINLOCK30 Register
///
/// Reading from a spinlock address will:
/// - Return 0 if lock is already locked
/// - Otherwise return nonzero, and simultaneously claim the lock
/// Writing (any value) releases the lock.
/// If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.
/// The value returned on success is 0x1 << lock number.
///
/// Address offset: 0x0178
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SPINLOCK30** `[31:0]` (RW)
#[repr(transparent)]
pub struct Spinlock30 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Spinlock30 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Spinlock30 {
    // ======== Field Constants ========

    /// **SPINLOCK30**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SPINLOCK30_OFFSET: u32 = 0;
    pub const SPINLOCK30_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SPINLOCK30** `[31:0]` (RW)
    #[inline(always)]
    pub fn spinlock30(&self) -> u32 {
        (self.value.read_bits() & Self::SPINLOCK30_MASK) >> Self::SPINLOCK30_OFFSET
    }

    /// Write **SPINLOCK30** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_spinlock30(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SPINLOCK30_MASK) | ((value << Self::SPINLOCK30_OFFSET) & Self::SPINLOCK30_MASK));
    }

}

/// SPINLOCK30 value type for batch read/modify/write operations
///
/// Use `spinlock30.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spinlock30.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Spinlock30Val {
    bits: u32,
}

impl Spinlock30Val {
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

    /// Read **SPINLOCK30** `[31:0]`
    #[inline(always)]
    pub const fn spinlock30(&self) -> u32 {
        (self.bits & Spinlock30::SPINLOCK30_MASK) >> Spinlock30::SPINLOCK30_OFFSET
    }

    /// Set **SPINLOCK30** `[31:0]`
    #[inline(always)]
    pub const fn with_spinlock30(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Spinlock30::SPINLOCK30_MASK) | ((value << Spinlock30::SPINLOCK30_OFFSET) & Spinlock30::SPINLOCK30_MASK)
        }
    }

}

impl Default for Spinlock30Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Spinlock30 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Spinlock30Val {
        Spinlock30Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Spinlock30Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Spinlock30Val) -> Spinlock30Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SPINLOCK31 Register
///
/// Reading from a spinlock address will:
/// - Return 0 if lock is already locked
/// - Otherwise return nonzero, and simultaneously claim the lock
/// Writing (any value) releases the lock.
/// If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.
/// The value returned on success is 0x1 << lock number.
///
/// Address offset: 0x017c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SPINLOCK31** `[31:0]` (RW)
#[repr(transparent)]
pub struct Spinlock31 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Spinlock31 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Spinlock31 {
    // ======== Field Constants ========

    /// **SPINLOCK31**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SPINLOCK31_OFFSET: u32 = 0;
    pub const SPINLOCK31_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SPINLOCK31** `[31:0]` (RW)
    #[inline(always)]
    pub fn spinlock31(&self) -> u32 {
        (self.value.read_bits() & Self::SPINLOCK31_MASK) >> Self::SPINLOCK31_OFFSET
    }

    /// Write **SPINLOCK31** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_spinlock31(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SPINLOCK31_MASK) | ((value << Self::SPINLOCK31_OFFSET) & Self::SPINLOCK31_MASK));
    }

}

/// SPINLOCK31 value type for batch read/modify/write operations
///
/// Use `spinlock31.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spinlock31.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Spinlock31Val {
    bits: u32,
}

impl Spinlock31Val {
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

    /// Read **SPINLOCK31** `[31:0]`
    #[inline(always)]
    pub const fn spinlock31(&self) -> u32 {
        (self.bits & Spinlock31::SPINLOCK31_MASK) >> Spinlock31::SPINLOCK31_OFFSET
    }

    /// Set **SPINLOCK31** `[31:0]`
    #[inline(always)]
    pub const fn with_spinlock31(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Spinlock31::SPINLOCK31_MASK) | ((value << Spinlock31::SPINLOCK31_OFFSET) & Spinlock31::SPINLOCK31_MASK)
        }
    }

}

impl Default for Spinlock31Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Spinlock31 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Spinlock31Val {
        Spinlock31Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Spinlock31Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Spinlock31Val) -> Spinlock31Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// DOORBELL_OUT_SET Register
///
/// Trigger a doorbell interrupt on the opposite core.
/// Write 1 to a bit to set the corresponding bit in DOORBELL_IN on the opposite core. This raises the opposite core's doorbell interrupt.
/// Read to get the status of the doorbells currently asserted on the opposite core. This is equivalent to that core reading its own DOORBELL_IN status.
///
/// Address offset: 0x0180
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **DOORBELL_OUT_SET** `[7:0]` (RW)
#[repr(transparent)]
pub struct DoorbellOutSet {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DoorbellOutSet {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DoorbellOutSet {
    // ======== Field Constants ========

    /// **DOORBELL_OUT_SET**
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const DOORBELL_OUT_SET_OFFSET: u32 = 0;
    pub const DOORBELL_OUT_SET_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **DOORBELL_OUT_SET** `[7:0]` (RW)
    #[inline(always)]
    pub fn doorbell_out_set(&self) -> u32 {
        (self.value.read_bits() & Self::DOORBELL_OUT_SET_MASK) >> Self::DOORBELL_OUT_SET_OFFSET
    }

    /// Write **DOORBELL_OUT_SET** `[7:0]` (RW)
    #[inline(always)]
    pub fn write_doorbell_out_set(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DOORBELL_OUT_SET_MASK) | ((value << Self::DOORBELL_OUT_SET_OFFSET) & Self::DOORBELL_OUT_SET_MASK));
    }

}

/// DOORBELL_OUT_SET value type for batch read/modify/write operations
///
/// Use `doorbelloutset.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `doorbelloutset.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DoorbellOutSetVal {
    bits: u32,
}

impl DoorbellOutSetVal {
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

    /// Read **DOORBELL_OUT_SET** `[7:0]`
    #[inline(always)]
    pub const fn doorbell_out_set(&self) -> u32 {
        (self.bits & DoorbellOutSet::DOORBELL_OUT_SET_MASK) >> DoorbellOutSet::DOORBELL_OUT_SET_OFFSET
    }

    /// Set **DOORBELL_OUT_SET** `[7:0]`
    #[inline(always)]
    pub const fn with_doorbell_out_set(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DoorbellOutSet::DOORBELL_OUT_SET_MASK) | ((value << DoorbellOutSet::DOORBELL_OUT_SET_OFFSET) & DoorbellOutSet::DOORBELL_OUT_SET_MASK)
        }
    }

}

impl Default for DoorbellOutSetVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl DoorbellOutSet {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DoorbellOutSetVal {
        DoorbellOutSetVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DoorbellOutSetVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DoorbellOutSetVal) -> DoorbellOutSetVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// DOORBELL_OUT_CLR Register
///
/// Clear doorbells which have been posted to the opposite core. This register is intended for debugging and initialisation purposes.
/// Writing 1 to a bit in DOORBELL_OUT_CLR clears the corresponding bit in DOORBELL_IN on the opposite core. Clearing all bits will cause that core's doorbell interrupt to deassert. Since the usual order of events is for software to send events using DOORBELL_OUT_SET, and acknowledge incoming events by writing to DOORBELL_IN_CLR, this register should be used with caution to avoid race conditions.
/// Reading returns the status of the doorbells currently asserted on the other core, i.e. is equivalent to that core reading its own DOORBELL_IN status.
///
/// Address offset: 0x0184
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **DOORBELL_OUT_CLR** `[7:0]` (RW)
#[repr(transparent)]
pub struct DoorbellOutClr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DoorbellOutClr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DoorbellOutClr {
    // ======== Field Constants ========

    /// **DOORBELL_OUT_CLR**
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const DOORBELL_OUT_CLR_OFFSET: u32 = 0;
    pub const DOORBELL_OUT_CLR_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **DOORBELL_OUT_CLR** `[7:0]` (RW)
    #[inline(always)]
    pub fn doorbell_out_clr(&self) -> u32 {
        (self.value.read_bits() & Self::DOORBELL_OUT_CLR_MASK) >> Self::DOORBELL_OUT_CLR_OFFSET
    }

    /// Write **DOORBELL_OUT_CLR** `[7:0]` (RW)
    #[inline(always)]
    pub fn write_doorbell_out_clr(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DOORBELL_OUT_CLR_MASK) | ((value << Self::DOORBELL_OUT_CLR_OFFSET) & Self::DOORBELL_OUT_CLR_MASK));
    }

}

/// DOORBELL_OUT_CLR value type for batch read/modify/write operations
///
/// Use `doorbelloutclr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `doorbelloutclr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DoorbellOutClrVal {
    bits: u32,
}

impl DoorbellOutClrVal {
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

    /// Read **DOORBELL_OUT_CLR** `[7:0]`
    #[inline(always)]
    pub const fn doorbell_out_clr(&self) -> u32 {
        (self.bits & DoorbellOutClr::DOORBELL_OUT_CLR_MASK) >> DoorbellOutClr::DOORBELL_OUT_CLR_OFFSET
    }

    /// Set **DOORBELL_OUT_CLR** `[7:0]`
    #[inline(always)]
    pub const fn with_doorbell_out_clr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DoorbellOutClr::DOORBELL_OUT_CLR_MASK) | ((value << DoorbellOutClr::DOORBELL_OUT_CLR_OFFSET) & DoorbellOutClr::DOORBELL_OUT_CLR_MASK)
        }
    }

}

impl Default for DoorbellOutClrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl DoorbellOutClr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DoorbellOutClrVal {
        DoorbellOutClrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DoorbellOutClrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DoorbellOutClrVal) -> DoorbellOutClrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// DOORBELL_IN_SET Register
///
/// Write 1s to trigger doorbell interrupts on this core. Read to get status of doorbells currently asserted on this core.
///
/// Address offset: 0x0188
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **DOORBELL_IN_SET** `[7:0]` (RW)
#[repr(transparent)]
pub struct DoorbellInSet {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DoorbellInSet {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DoorbellInSet {
    // ======== Field Constants ========

    /// **DOORBELL_IN_SET**
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const DOORBELL_IN_SET_OFFSET: u32 = 0;
    pub const DOORBELL_IN_SET_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **DOORBELL_IN_SET** `[7:0]` (RW)
    #[inline(always)]
    pub fn doorbell_in_set(&self) -> u32 {
        (self.value.read_bits() & Self::DOORBELL_IN_SET_MASK) >> Self::DOORBELL_IN_SET_OFFSET
    }

    /// Write **DOORBELL_IN_SET** `[7:0]` (RW)
    #[inline(always)]
    pub fn write_doorbell_in_set(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DOORBELL_IN_SET_MASK) | ((value << Self::DOORBELL_IN_SET_OFFSET) & Self::DOORBELL_IN_SET_MASK));
    }

}

/// DOORBELL_IN_SET value type for batch read/modify/write operations
///
/// Use `doorbellinset.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `doorbellinset.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DoorbellInSetVal {
    bits: u32,
}

impl DoorbellInSetVal {
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

    /// Read **DOORBELL_IN_SET** `[7:0]`
    #[inline(always)]
    pub const fn doorbell_in_set(&self) -> u32 {
        (self.bits & DoorbellInSet::DOORBELL_IN_SET_MASK) >> DoorbellInSet::DOORBELL_IN_SET_OFFSET
    }

    /// Set **DOORBELL_IN_SET** `[7:0]`
    #[inline(always)]
    pub const fn with_doorbell_in_set(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DoorbellInSet::DOORBELL_IN_SET_MASK) | ((value << DoorbellInSet::DOORBELL_IN_SET_OFFSET) & DoorbellInSet::DOORBELL_IN_SET_MASK)
        }
    }

}

impl Default for DoorbellInSetVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl DoorbellInSet {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DoorbellInSetVal {
        DoorbellInSetVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DoorbellInSetVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DoorbellInSetVal) -> DoorbellInSetVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// DOORBELL_IN_CLR Register
///
/// Check and acknowledge doorbells posted to this core. This core's doorbell interrupt is asserted when any bit in this register is 1.
/// Write 1 to each bit to clear that bit. The doorbell interrupt deasserts once all bits are cleared. Read to get status of doorbells currently asserted on this core.
///
/// Address offset: 0x018c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **DOORBELL_IN_CLR** `[7:0]` (RW)
#[repr(transparent)]
pub struct DoorbellInClr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DoorbellInClr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DoorbellInClr {
    // ======== Field Constants ========

    /// **DOORBELL_IN_CLR**
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const DOORBELL_IN_CLR_OFFSET: u32 = 0;
    pub const DOORBELL_IN_CLR_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **DOORBELL_IN_CLR** `[7:0]` (RW)
    #[inline(always)]
    pub fn doorbell_in_clr(&self) -> u32 {
        (self.value.read_bits() & Self::DOORBELL_IN_CLR_MASK) >> Self::DOORBELL_IN_CLR_OFFSET
    }

    /// Write **DOORBELL_IN_CLR** `[7:0]` (RW)
    #[inline(always)]
    pub fn write_doorbell_in_clr(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DOORBELL_IN_CLR_MASK) | ((value << Self::DOORBELL_IN_CLR_OFFSET) & Self::DOORBELL_IN_CLR_MASK));
    }

}

/// DOORBELL_IN_CLR value type for batch read/modify/write operations
///
/// Use `doorbellinclr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `doorbellinclr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DoorbellInClrVal {
    bits: u32,
}

impl DoorbellInClrVal {
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

    /// Read **DOORBELL_IN_CLR** `[7:0]`
    #[inline(always)]
    pub const fn doorbell_in_clr(&self) -> u32 {
        (self.bits & DoorbellInClr::DOORBELL_IN_CLR_MASK) >> DoorbellInClr::DOORBELL_IN_CLR_OFFSET
    }

    /// Set **DOORBELL_IN_CLR** `[7:0]`
    #[inline(always)]
    pub const fn with_doorbell_in_clr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DoorbellInClr::DOORBELL_IN_CLR_MASK) | ((value << DoorbellInClr::DOORBELL_IN_CLR_OFFSET) & DoorbellInClr::DOORBELL_IN_CLR_MASK)
        }
    }

}

impl Default for DoorbellInClrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl DoorbellInClr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DoorbellInClrVal {
        DoorbellInClrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DoorbellInClrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DoorbellInClrVal) -> DoorbellInClrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// PERI_NONSEC Register
///
/// Detach certain core-local peripherals from Secure SIO, and attach them to Non-secure SIO, so that Non-secure software can use them. Attempting to access one of these peripherals from the Secure SIO when it is attached to the Non-secure SIO, or vice versa, will generate a bus error.
/// This register is per-core, and is only present on the Secure SIO.
/// Most SIO hardware is duplicated across the Secure and Non-secure SIO, so is not listed in this register.
///
/// Address offset: 0x0190
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **TMDS** `[5]` (RW)
///   IF 1, detach TMDS encoder (of this core) from the Secure SIO, and attach to the Non-secure SIO.
/// - **INTERP1** `[1]` (RW)
///   If 1, detach interpolator 1 (of this core) from the Secure SIO, and attach to the Non-secure SIO.
/// - **INTERP0** `[0]` (RW)
///   If 1, detach interpolator 0 (of this core) from the Secure SIO, and attach to the Non-secure SIO.
#[repr(transparent)]
pub struct PeriNonsec {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PeriNonsec {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PeriNonsec {
    // ======== Field Constants ========

    /// **TMDS**
    /// IF 1, detach TMDS encoder (of this core) from the Secure SIO, and attach to the Non-secure SIO.
    /// Bits `[5:5]`, mask `0x00000020`
    pub const TMDS_OFFSET: u32 = 5;
    pub const TMDS_MASK: u32 = 0x00000020;
    pub const TMDS: u32 = 1 << 5;

    /// **INTERP1**
    /// If 1, detach interpolator 1 (of this core) from the Secure SIO, and attach to the Non-secure SIO.
    /// Bits `[1:1]`, mask `0x00000002`
    pub const INTERP1_OFFSET: u32 = 1;
    pub const INTERP1_MASK: u32 = 0x00000002;
    pub const INTERP1: u32 = 1 << 1;

    /// **INTERP0**
    /// If 1, detach interpolator 0 (of this core) from the Secure SIO, and attach to the Non-secure SIO.
    /// Bits `[0:0]`, mask `0x00000001`
    pub const INTERP0_OFFSET: u32 = 0;
    pub const INTERP0_MASK: u32 = 0x00000001;
    pub const INTERP0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TMDS** `[5]` (RW)
    /// IF 1, detach TMDS encoder (of this core) from the Secure SIO, and attach to the Non-secure SIO.
    #[inline(always)]
    pub fn tmds(&self) -> bool {
        (self.value.read_bits() & Self::TMDS_MASK) != 0
    }

    /// Set TMDS bit atomically (RW)
    #[inline(always)]
    pub fn set_tmds(&self) {
        self.value.set_bits(Self::TMDS_MASK);
    }

    /// Clear TMDS bit atomically (RW)
    #[inline(always)]
    pub fn clr_tmds(&self) {
        self.value.clr_bits(Self::TMDS_MASK);
    }

    /// **INTERP1** `[1]` (RW)
    /// If 1, detach interpolator 1 (of this core) from the Secure SIO, and attach to the Non-secure SIO.
    #[inline(always)]
    pub fn interp1(&self) -> bool {
        (self.value.read_bits() & Self::INTERP1_MASK) != 0
    }

    /// Set INTERP1 bit atomically (RW)
    #[inline(always)]
    pub fn set_interp1(&self) {
        self.value.set_bits(Self::INTERP1_MASK);
    }

    /// Clear INTERP1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_interp1(&self) {
        self.value.clr_bits(Self::INTERP1_MASK);
    }

    /// **INTERP0** `[0]` (RW)
    /// If 1, detach interpolator 0 (of this core) from the Secure SIO, and attach to the Non-secure SIO.
    #[inline(always)]
    pub fn interp0(&self) -> bool {
        (self.value.read_bits() & Self::INTERP0_MASK) != 0
    }

    /// Set INTERP0 bit atomically (RW)
    #[inline(always)]
    pub fn set_interp0(&self) {
        self.value.set_bits(Self::INTERP0_MASK);
    }

    /// Clear INTERP0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_interp0(&self) {
        self.value.clr_bits(Self::INTERP0_MASK);
    }

}

/// PERI_NONSEC value type for batch read/modify/write operations
///
/// Use `perinonsec.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `perinonsec.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PeriNonsecVal {
    bits: u32,
}

impl PeriNonsecVal {
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

    /// Read **TMDS** `[5:5]`
    /// IF 1, detach TMDS encoder (of this core) from the Secure SIO, and attach to the Non-secure SIO.
    #[inline(always)]
    pub const fn tmds(&self) -> bool {
        (self.bits & PeriNonsec::TMDS_MASK) != 0
    }

    /// Read **INTERP1** `[1:1]`
    /// If 1, detach interpolator 1 (of this core) from the Secure SIO, and attach to the Non-secure SIO.
    #[inline(always)]
    pub const fn interp1(&self) -> bool {
        (self.bits & PeriNonsec::INTERP1_MASK) != 0
    }

    /// Read **INTERP0** `[0:0]`
    /// If 1, detach interpolator 0 (of this core) from the Secure SIO, and attach to the Non-secure SIO.
    #[inline(always)]
    pub const fn interp0(&self) -> bool {
        (self.bits & PeriNonsec::INTERP0_MASK) != 0
    }

    /// Set **TMDS** `[5:5]`
    /// IF 1, detach TMDS encoder (of this core) from the Secure SIO, and attach to the Non-secure SIO.
    #[inline(always)]
    pub const fn with_tmds(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PeriNonsec::TMDS_MASK
            } else {
                self.bits & !PeriNonsec::TMDS_MASK
            }
        }
    }

    /// Set **INTERP1** `[1:1]`
    /// If 1, detach interpolator 1 (of this core) from the Secure SIO, and attach to the Non-secure SIO.
    #[inline(always)]
    pub const fn with_interp1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PeriNonsec::INTERP1_MASK
            } else {
                self.bits & !PeriNonsec::INTERP1_MASK
            }
        }
    }

    /// Set **INTERP0** `[0:0]`
    /// If 1, detach interpolator 0 (of this core) from the Secure SIO, and attach to the Non-secure SIO.
    #[inline(always)]
    pub const fn with_interp0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PeriNonsec::INTERP0_MASK
            } else {
                self.bits & !PeriNonsec::INTERP0_MASK
            }
        }
    }

}

impl Default for PeriNonsecVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl PeriNonsec {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PeriNonsecVal {
        PeriNonsecVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: PeriNonsecVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(PeriNonsecVal) -> PeriNonsecVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// RISCV_SOFTIRQ Register
///
/// Control the assertion of the standard software interrupt (MIP.MSIP) on the RISC-V cores.
/// Unlike the RISC-V timer, this interrupt is not routed to a normal system-level interrupt line, so can not be used by the Arm cores.
/// It is safe for both cores to write to this register on the same cycle. The set/clear effect is accumulated across both cores, and then applied. If a flag is both set and cleared on the same cycle, only the set takes effect.
///
/// Address offset: 0x01a0
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CORE1_CLR** `[9]` (RW)
///   Write 1 to atomically clear the core 1 software interrupt flag. Read to get the status of this flag.
/// - **CORE0_CLR** `[8]` (RW)
///   Write 1 to atomically clear the core 0 software interrupt flag. Read to get the status of this flag.
/// - **CORE1_SET** `[1]` (RW)
///   Write 1 to atomically set the core 1 software interrupt flag. Read to get the status of this flag.
/// - **CORE0_SET** `[0]` (RW)
///   Write 1 to atomically set the core 0 software interrupt flag. Read to get the status of this flag.
#[repr(transparent)]
pub struct RiscvSoftirq {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RiscvSoftirq {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RiscvSoftirq {
    // ======== Field Constants ========

    /// **CORE1_CLR**
    /// Write 1 to atomically clear the core 1 software interrupt flag. Read to get the status of this flag.
    /// Bits `[9:9]`, mask `0x00000200`
    pub const CORE1_CLR_OFFSET: u32 = 9;
    pub const CORE1_CLR_MASK: u32 = 0x00000200;
    pub const CORE1_CLR: u32 = 1 << 9;

    /// **CORE0_CLR**
    /// Write 1 to atomically clear the core 0 software interrupt flag. Read to get the status of this flag.
    /// Bits `[8:8]`, mask `0x00000100`
    pub const CORE0_CLR_OFFSET: u32 = 8;
    pub const CORE0_CLR_MASK: u32 = 0x00000100;
    pub const CORE0_CLR: u32 = 1 << 8;

    /// **CORE1_SET**
    /// Write 1 to atomically set the core 1 software interrupt flag. Read to get the status of this flag.
    /// Bits `[1:1]`, mask `0x00000002`
    pub const CORE1_SET_OFFSET: u32 = 1;
    pub const CORE1_SET_MASK: u32 = 0x00000002;
    pub const CORE1_SET: u32 = 1 << 1;

    /// **CORE0_SET**
    /// Write 1 to atomically set the core 0 software interrupt flag. Read to get the status of this flag.
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CORE0_SET_OFFSET: u32 = 0;
    pub const CORE0_SET_MASK: u32 = 0x00000001;
    pub const CORE0_SET: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CORE1_CLR** `[9]` (RW)
    /// Write 1 to atomically clear the core 1 software interrupt flag. Read to get the status of this flag.
    #[inline(always)]
    pub fn core1_clr(&self) -> bool {
        (self.value.read_bits() & Self::CORE1_CLR_MASK) != 0
    }

    /// Set CORE1_CLR bit atomically (RW)
    #[inline(always)]
    pub fn set_core1_clr(&self) {
        self.value.set_bits(Self::CORE1_CLR_MASK);
    }

    /// Clear CORE1_CLR bit atomically (RW)
    #[inline(always)]
    pub fn clr_core1_clr(&self) {
        self.value.clr_bits(Self::CORE1_CLR_MASK);
    }

    /// **CORE0_CLR** `[8]` (RW)
    /// Write 1 to atomically clear the core 0 software interrupt flag. Read to get the status of this flag.
    #[inline(always)]
    pub fn core0_clr(&self) -> bool {
        (self.value.read_bits() & Self::CORE0_CLR_MASK) != 0
    }

    /// Set CORE0_CLR bit atomically (RW)
    #[inline(always)]
    pub fn set_core0_clr(&self) {
        self.value.set_bits(Self::CORE0_CLR_MASK);
    }

    /// Clear CORE0_CLR bit atomically (RW)
    #[inline(always)]
    pub fn clr_core0_clr(&self) {
        self.value.clr_bits(Self::CORE0_CLR_MASK);
    }

    /// **CORE1_SET** `[1]` (RW)
    /// Write 1 to atomically set the core 1 software interrupt flag. Read to get the status of this flag.
    #[inline(always)]
    pub fn core1_set(&self) -> bool {
        (self.value.read_bits() & Self::CORE1_SET_MASK) != 0
    }

    /// Set CORE1_SET bit atomically (RW)
    #[inline(always)]
    pub fn set_core1_set(&self) {
        self.value.set_bits(Self::CORE1_SET_MASK);
    }

    /// Clear CORE1_SET bit atomically (RW)
    #[inline(always)]
    pub fn clr_core1_set(&self) {
        self.value.clr_bits(Self::CORE1_SET_MASK);
    }

    /// **CORE0_SET** `[0]` (RW)
    /// Write 1 to atomically set the core 0 software interrupt flag. Read to get the status of this flag.
    #[inline(always)]
    pub fn core0_set(&self) -> bool {
        (self.value.read_bits() & Self::CORE0_SET_MASK) != 0
    }

    /// Set CORE0_SET bit atomically (RW)
    #[inline(always)]
    pub fn set_core0_set(&self) {
        self.value.set_bits(Self::CORE0_SET_MASK);
    }

    /// Clear CORE0_SET bit atomically (RW)
    #[inline(always)]
    pub fn clr_core0_set(&self) {
        self.value.clr_bits(Self::CORE0_SET_MASK);
    }

}

/// RISCV_SOFTIRQ value type for batch read/modify/write operations
///
/// Use `riscvsoftirq.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `riscvsoftirq.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RiscvSoftirqVal {
    bits: u32,
}

impl RiscvSoftirqVal {
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

    /// Read **CORE1_CLR** `[9:9]`
    /// Write 1 to atomically clear the core 1 software interrupt flag. Read to get the status of this flag.
    #[inline(always)]
    pub const fn core1_clr(&self) -> bool {
        (self.bits & RiscvSoftirq::CORE1_CLR_MASK) != 0
    }

    /// Read **CORE0_CLR** `[8:8]`
    /// Write 1 to atomically clear the core 0 software interrupt flag. Read to get the status of this flag.
    #[inline(always)]
    pub const fn core0_clr(&self) -> bool {
        (self.bits & RiscvSoftirq::CORE0_CLR_MASK) != 0
    }

    /// Read **CORE1_SET** `[1:1]`
    /// Write 1 to atomically set the core 1 software interrupt flag. Read to get the status of this flag.
    #[inline(always)]
    pub const fn core1_set(&self) -> bool {
        (self.bits & RiscvSoftirq::CORE1_SET_MASK) != 0
    }

    /// Read **CORE0_SET** `[0:0]`
    /// Write 1 to atomically set the core 0 software interrupt flag. Read to get the status of this flag.
    #[inline(always)]
    pub const fn core0_set(&self) -> bool {
        (self.bits & RiscvSoftirq::CORE0_SET_MASK) != 0
    }

    /// Set **CORE1_CLR** `[9:9]`
    /// Write 1 to atomically clear the core 1 software interrupt flag. Read to get the status of this flag.
    #[inline(always)]
    pub const fn with_core1_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RiscvSoftirq::CORE1_CLR_MASK
            } else {
                self.bits & !RiscvSoftirq::CORE1_CLR_MASK
            }
        }
    }

    /// Set **CORE0_CLR** `[8:8]`
    /// Write 1 to atomically clear the core 0 software interrupt flag. Read to get the status of this flag.
    #[inline(always)]
    pub const fn with_core0_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RiscvSoftirq::CORE0_CLR_MASK
            } else {
                self.bits & !RiscvSoftirq::CORE0_CLR_MASK
            }
        }
    }

    /// Set **CORE1_SET** `[1:1]`
    /// Write 1 to atomically set the core 1 software interrupt flag. Read to get the status of this flag.
    #[inline(always)]
    pub const fn with_core1_set(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RiscvSoftirq::CORE1_SET_MASK
            } else {
                self.bits & !RiscvSoftirq::CORE1_SET_MASK
            }
        }
    }

    /// Set **CORE0_SET** `[0:0]`
    /// Write 1 to atomically set the core 0 software interrupt flag. Read to get the status of this flag.
    #[inline(always)]
    pub const fn with_core0_set(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RiscvSoftirq::CORE0_SET_MASK
            } else {
                self.bits & !RiscvSoftirq::CORE0_SET_MASK
            }
        }
    }

}

impl Default for RiscvSoftirqVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl RiscvSoftirq {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RiscvSoftirqVal {
        RiscvSoftirqVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RiscvSoftirqVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RiscvSoftirqVal) -> RiscvSoftirqVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// MTIME_CTRL Register
///
/// Control register for the RISC-V 64-bit Machine-mode timer. This timer is only present in the Secure SIO, so is only accessible to an Arm core in Secure mode or a RISC-V core in Machine mode.
/// Note whilst this timer follows the RISC-V privileged specification, it is equally usable by the Arm cores. The interrupts are routed to normal system-level interrupt lines as well as to the MIP.MTIP inputs on the RISC-V cores.
///
/// Address offset: 0x01a4
/// Reset value: 0x0000000d
///
/// ## Fields
///
/// - **DBGPAUSE_CORE1** `[3]` (RW)
///   If 1, the timer pauses when core 1 is in the debug halt state.
/// - **DBGPAUSE_CORE0** `[2]` (RW)
///   If 1, the timer pauses when core 0 is in the debug halt state.
/// - **FULLSPEED** `[1]` (RW)
///   If 1, increment the timer every cycle (i.e. run directly from the system clock), rather than incrementing on the system-level timer tick input.
/// - **EN** `[0]` (RW)
///   Timer enable bit. When 0, the timer will not increment automatically.
#[repr(transparent)]
pub struct MtimeCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MtimeCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MtimeCtrl {
    // ======== Field Constants ========

    /// **DBGPAUSE_CORE1**
    /// If 1, the timer pauses when core 1 is in the debug halt state.
    /// Bits `[3:3]`, mask `0x00000008`
    pub const DBGPAUSE_CORE1_OFFSET: u32 = 3;
    pub const DBGPAUSE_CORE1_MASK: u32 = 0x00000008;
    pub const DBGPAUSE_CORE1: u32 = 1 << 3;

    /// **DBGPAUSE_CORE0**
    /// If 1, the timer pauses when core 0 is in the debug halt state.
    /// Bits `[2:2]`, mask `0x00000004`
    pub const DBGPAUSE_CORE0_OFFSET: u32 = 2;
    pub const DBGPAUSE_CORE0_MASK: u32 = 0x00000004;
    pub const DBGPAUSE_CORE0: u32 = 1 << 2;

    /// **FULLSPEED**
    /// If 1, increment the timer every cycle (i.e. run directly from the system clock), rather than incrementing on the system-level timer tick input.
    /// Bits `[1:1]`, mask `0x00000002`
    pub const FULLSPEED_OFFSET: u32 = 1;
    pub const FULLSPEED_MASK: u32 = 0x00000002;
    pub const FULLSPEED: u32 = 1 << 1;

    /// **EN**
    /// Timer enable bit. When 0, the timer will not increment automatically.
    /// Bits `[0:0]`, mask `0x00000001`
    pub const EN_OFFSET: u32 = 0;
    pub const EN_MASK: u32 = 0x00000001;
    pub const EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DBGPAUSE_CORE1** `[3]` (RW)
    /// If 1, the timer pauses when core 1 is in the debug halt state.
    #[inline(always)]
    pub fn dbgpause_core1(&self) -> bool {
        (self.value.read_bits() & Self::DBGPAUSE_CORE1_MASK) != 0
    }

    /// Set DBGPAUSE_CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn set_dbgpause_core1(&self) {
        self.value.set_bits(Self::DBGPAUSE_CORE1_MASK);
    }

    /// Clear DBGPAUSE_CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_dbgpause_core1(&self) {
        self.value.clr_bits(Self::DBGPAUSE_CORE1_MASK);
    }

    /// **DBGPAUSE_CORE0** `[2]` (RW)
    /// If 1, the timer pauses when core 0 is in the debug halt state.
    #[inline(always)]
    pub fn dbgpause_core0(&self) -> bool {
        (self.value.read_bits() & Self::DBGPAUSE_CORE0_MASK) != 0
    }

    /// Set DBGPAUSE_CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn set_dbgpause_core0(&self) {
        self.value.set_bits(Self::DBGPAUSE_CORE0_MASK);
    }

    /// Clear DBGPAUSE_CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_dbgpause_core0(&self) {
        self.value.clr_bits(Self::DBGPAUSE_CORE0_MASK);
    }

    /// **FULLSPEED** `[1]` (RW)
    /// If 1, increment the timer every cycle (i.e. run directly from the system clock), rather than incrementing on the system-level timer tick input.
    #[inline(always)]
    pub fn fullspeed(&self) -> bool {
        (self.value.read_bits() & Self::FULLSPEED_MASK) != 0
    }

    /// Set FULLSPEED bit atomically (RW)
    #[inline(always)]
    pub fn set_fullspeed(&self) {
        self.value.set_bits(Self::FULLSPEED_MASK);
    }

    /// Clear FULLSPEED bit atomically (RW)
    #[inline(always)]
    pub fn clr_fullspeed(&self) {
        self.value.clr_bits(Self::FULLSPEED_MASK);
    }

    /// **EN** `[0]` (RW)
    /// Timer enable bit. When 0, the timer will not increment automatically.
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

/// MTIME_CTRL value type for batch read/modify/write operations
///
/// Use `mtimectrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `mtimectrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct MtimeCtrlVal {
    bits: u32,
}

impl MtimeCtrlVal {
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
        Self { bits: 0x0000000d }
    }

    /// Read **DBGPAUSE_CORE1** `[3:3]`
    /// If 1, the timer pauses when core 1 is in the debug halt state.
    #[inline(always)]
    pub const fn dbgpause_core1(&self) -> bool {
        (self.bits & MtimeCtrl::DBGPAUSE_CORE1_MASK) != 0
    }

    /// Read **DBGPAUSE_CORE0** `[2:2]`
    /// If 1, the timer pauses when core 0 is in the debug halt state.
    #[inline(always)]
    pub const fn dbgpause_core0(&self) -> bool {
        (self.bits & MtimeCtrl::DBGPAUSE_CORE0_MASK) != 0
    }

    /// Read **FULLSPEED** `[1:1]`
    /// If 1, increment the timer every cycle (i.e. run directly from the system clock), rather than incrementing on the system-level timer tick input.
    #[inline(always)]
    pub const fn fullspeed(&self) -> bool {
        (self.bits & MtimeCtrl::FULLSPEED_MASK) != 0
    }

    /// Read **EN** `[0:0]`
    /// Timer enable bit. When 0, the timer will not increment automatically.
    #[inline(always)]
    pub const fn en(&self) -> bool {
        (self.bits & MtimeCtrl::EN_MASK) != 0
    }

    /// Set **DBGPAUSE_CORE1** `[3:3]`
    /// If 1, the timer pauses when core 1 is in the debug halt state.
    #[inline(always)]
    pub const fn with_dbgpause_core1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | MtimeCtrl::DBGPAUSE_CORE1_MASK
            } else {
                self.bits & !MtimeCtrl::DBGPAUSE_CORE1_MASK
            }
        }
    }

    /// Set **DBGPAUSE_CORE0** `[2:2]`
    /// If 1, the timer pauses when core 0 is in the debug halt state.
    #[inline(always)]
    pub const fn with_dbgpause_core0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | MtimeCtrl::DBGPAUSE_CORE0_MASK
            } else {
                self.bits & !MtimeCtrl::DBGPAUSE_CORE0_MASK
            }
        }
    }

    /// Set **FULLSPEED** `[1:1]`
    /// If 1, increment the timer every cycle (i.e. run directly from the system clock), rather than incrementing on the system-level timer tick input.
    #[inline(always)]
    pub const fn with_fullspeed(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | MtimeCtrl::FULLSPEED_MASK
            } else {
                self.bits & !MtimeCtrl::FULLSPEED_MASK
            }
        }
    }

    /// Set **EN** `[0:0]`
    /// Timer enable bit. When 0, the timer will not increment automatically.
    #[inline(always)]
    pub const fn with_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | MtimeCtrl::EN_MASK
            } else {
                self.bits & !MtimeCtrl::EN_MASK
            }
        }
    }

}

impl Default for MtimeCtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl MtimeCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> MtimeCtrlVal {
        MtimeCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: MtimeCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(MtimeCtrlVal) -> MtimeCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// MTIME Register
///
/// Read/write access to the high half of RISC-V Machine-mode timer. This register is shared between both cores. If both cores write on the same cycle, core 1 takes precedence.
///
/// Address offset: 0x01b0
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **MTIME** `[31:0]` (RW)
#[repr(transparent)]
pub struct Mtime {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Mtime {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Mtime {
    // ======== Field Constants ========

    /// **MTIME**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const MTIME_OFFSET: u32 = 0;
    pub const MTIME_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **MTIME** `[31:0]` (RW)
    #[inline(always)]
    pub fn mtime(&self) -> u32 {
        (self.value.read_bits() & Self::MTIME_MASK) >> Self::MTIME_OFFSET
    }

    /// Write **MTIME** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_mtime(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::MTIME_MASK) | ((value << Self::MTIME_OFFSET) & Self::MTIME_MASK));
    }

}

/// MTIME value type for batch read/modify/write operations
///
/// Use `mtime.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `mtime.write_val(val)`.
#[derive(Clone, Copy)]
pub struct MtimeVal {
    bits: u32,
}

impl MtimeVal {
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

    /// Read **MTIME** `[31:0]`
    #[inline(always)]
    pub const fn mtime(&self) -> u32 {
        (self.bits & Mtime::MTIME_MASK) >> Mtime::MTIME_OFFSET
    }

    /// Set **MTIME** `[31:0]`
    #[inline(always)]
    pub const fn with_mtime(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Mtime::MTIME_MASK) | ((value << Mtime::MTIME_OFFSET) & Mtime::MTIME_MASK)
        }
    }

}

impl Default for MtimeVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Mtime {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> MtimeVal {
        MtimeVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: MtimeVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(MtimeVal) -> MtimeVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// MTIMEH Register
///
/// Read/write access to the high half of RISC-V Machine-mode timer. This register is shared between both cores. If both cores write on the same cycle, core 1 takes precedence.
///
/// Address offset: 0x01b4
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **MTIMEH** `[31:0]` (RW)
#[repr(transparent)]
pub struct Mtimeh {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Mtimeh {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Mtimeh {
    // ======== Field Constants ========

    /// **MTIMEH**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const MTIMEH_OFFSET: u32 = 0;
    pub const MTIMEH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **MTIMEH** `[31:0]` (RW)
    #[inline(always)]
    pub fn mtimeh(&self) -> u32 {
        (self.value.read_bits() & Self::MTIMEH_MASK) >> Self::MTIMEH_OFFSET
    }

    /// Write **MTIMEH** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_mtimeh(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::MTIMEH_MASK) | ((value << Self::MTIMEH_OFFSET) & Self::MTIMEH_MASK));
    }

}

/// MTIMEH value type for batch read/modify/write operations
///
/// Use `mtimeh.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `mtimeh.write_val(val)`.
#[derive(Clone, Copy)]
pub struct MtimehVal {
    bits: u32,
}

impl MtimehVal {
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

    /// Read **MTIMEH** `[31:0]`
    #[inline(always)]
    pub const fn mtimeh(&self) -> u32 {
        (self.bits & Mtimeh::MTIMEH_MASK) >> Mtimeh::MTIMEH_OFFSET
    }

    /// Set **MTIMEH** `[31:0]`
    #[inline(always)]
    pub const fn with_mtimeh(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Mtimeh::MTIMEH_MASK) | ((value << Mtimeh::MTIMEH_OFFSET) & Mtimeh::MTIMEH_MASK)
        }
    }

}

impl Default for MtimehVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Mtimeh {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> MtimehVal {
        MtimehVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: MtimehVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(MtimehVal) -> MtimehVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// MTIMECMP Register
///
/// Low half of RISC-V Machine-mode timer comparator. This register is core-local, i.e., each core gets a copy of this register, with the comparison result routed to its own interrupt line.
/// The timer interrupt is asserted whenever MTIME is greater than or equal to MTIMECMP. This comparison is unsigned, and performed on the full 64-bit values.
///
/// Address offset: 0x01b8
/// Reset value: 0xffffffff
///
/// ## Fields
///
/// - **MTIMECMP** `[31:0]` (RW)
#[repr(transparent)]
pub struct Mtimecmp {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Mtimecmp {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Mtimecmp {
    // ======== Field Constants ========

    /// **MTIMECMP**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const MTIMECMP_OFFSET: u32 = 0;
    pub const MTIMECMP_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **MTIMECMP** `[31:0]` (RW)
    #[inline(always)]
    pub fn mtimecmp(&self) -> u32 {
        (self.value.read_bits() & Self::MTIMECMP_MASK) >> Self::MTIMECMP_OFFSET
    }

    /// Write **MTIMECMP** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_mtimecmp(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::MTIMECMP_MASK) | ((value << Self::MTIMECMP_OFFSET) & Self::MTIMECMP_MASK));
    }

}

/// MTIMECMP value type for batch read/modify/write operations
///
/// Use `mtimecmp.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `mtimecmp.write_val(val)`.
#[derive(Clone, Copy)]
pub struct MtimecmpVal {
    bits: u32,
}

impl MtimecmpVal {
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
        Self { bits: 0xffffffff }
    }

    /// Read **MTIMECMP** `[31:0]`
    #[inline(always)]
    pub const fn mtimecmp(&self) -> u32 {
        (self.bits & Mtimecmp::MTIMECMP_MASK) >> Mtimecmp::MTIMECMP_OFFSET
    }

    /// Set **MTIMECMP** `[31:0]`
    #[inline(always)]
    pub const fn with_mtimecmp(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Mtimecmp::MTIMECMP_MASK) | ((value << Mtimecmp::MTIMECMP_OFFSET) & Mtimecmp::MTIMECMP_MASK)
        }
    }

}

impl Default for MtimecmpVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Mtimecmp {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> MtimecmpVal {
        MtimecmpVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: MtimecmpVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(MtimecmpVal) -> MtimecmpVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// MTIMECMPH Register
///
/// High half of RISC-V Machine-mode timer comparator. This register is core-local.
/// The timer interrupt is asserted whenever MTIME is greater than or equal to MTIMECMP. This comparison is unsigned, and performed on the full 64-bit values.
///
/// Address offset: 0x01bc
/// Reset value: 0xffffffff
///
/// ## Fields
///
/// - **MTIMECMPH** `[31:0]` (RW)
#[repr(transparent)]
pub struct Mtimecmph {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Mtimecmph {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Mtimecmph {
    // ======== Field Constants ========

    /// **MTIMECMPH**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const MTIMECMPH_OFFSET: u32 = 0;
    pub const MTIMECMPH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **MTIMECMPH** `[31:0]` (RW)
    #[inline(always)]
    pub fn mtimecmph(&self) -> u32 {
        (self.value.read_bits() & Self::MTIMECMPH_MASK) >> Self::MTIMECMPH_OFFSET
    }

    /// Write **MTIMECMPH** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_mtimecmph(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::MTIMECMPH_MASK) | ((value << Self::MTIMECMPH_OFFSET) & Self::MTIMECMPH_MASK));
    }

}

/// MTIMECMPH value type for batch read/modify/write operations
///
/// Use `mtimecmph.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `mtimecmph.write_val(val)`.
#[derive(Clone, Copy)]
pub struct MtimecmphVal {
    bits: u32,
}

impl MtimecmphVal {
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
        Self { bits: 0xffffffff }
    }

    /// Read **MTIMECMPH** `[31:0]`
    #[inline(always)]
    pub const fn mtimecmph(&self) -> u32 {
        (self.bits & Mtimecmph::MTIMECMPH_MASK) >> Mtimecmph::MTIMECMPH_OFFSET
    }

    /// Set **MTIMECMPH** `[31:0]`
    #[inline(always)]
    pub const fn with_mtimecmph(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Mtimecmph::MTIMECMPH_MASK) | ((value << Mtimecmph::MTIMECMPH_OFFSET) & Mtimecmph::MTIMECMPH_MASK)
        }
    }

}

impl Default for MtimecmphVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Mtimecmph {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> MtimecmphVal {
        MtimecmphVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: MtimecmphVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(MtimecmphVal) -> MtimecmphVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// TMDS_CTRL Register
///
/// Control register for TMDS encoder.
///
/// Address offset: 0x01c0
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CLEAR_BALANCE** `[28]` (WO)
///   Clear the running DC balance state of the TMDS encoders. This bit should be written once at the beginning of each scanline.
/// - **PIX2_NOSHIFT** `[27]` (RW)
///   When encoding two pixels's worth of symbols in one cycle (a read of a PEEK/POP_DOUBLE register), the second encoder sees a shifted version of the colour data register.
///   This control disables that shift, so that both encoder layers see the same pixel data. This is used for pixel doubling.
/// - **PIX_SHIFT** `[26:24]` (RW)
///   Shift applied to the colour data register with each read of a POP alias register.
///   Reading from the POP_SINGLE register, or reading from the POP_DOUBLE register with PIX2_NOSHIFT set (for pixel doubling), shifts by the indicated amount.
///   Reading from a POP_DOUBLE register when PIX2_NOSHIFT is clear will shift by double the indicated amount. (Shift by 32 means no shift.)
/// - **INTERLEAVE** `[23]` (RW)
///   Enable lane interleaving for reads of PEEK_SINGLE/POP_SINGLE.
///   When interleaving is disabled, each of the 3 symbols appears as a contiguous 10-bit field, with lane 0 being the least-significant and starting at bit 0 of the register.
///   When interleaving is enabled, the symbols are packed into 5 chunks of 3 lanes times 2 bits (30 bits total). Each chunk contains two bits of a TMDS symbol per lane, with lane 0 being the least significant.
/// - **L2_NBITS** `[20:18]` (RW)
///   Number of valid colour MSBs for lane 2 (1-8 bits, encoded as 0 through 7). Remaining LSBs are masked to 0 after the rotate.
/// - **L1_NBITS** `[17:15]` (RW)
///   Number of valid colour MSBs for lane 1 (1-8 bits, encoded as 0 through 7). Remaining LSBs are masked to 0 after the rotate.
/// - **L0_NBITS** `[14:12]` (RW)
///   Number of valid colour MSBs for lane 0 (1-8 bits, encoded as 0 through 7). Remaining LSBs are masked to 0 after the rotate.
/// - **L2_ROT** `[11:8]` (RW)
///   Right-rotate the 16 LSBs of the colour accumulator by 0-15 bits, in order to get the MSB of the lane 2 (red) colour data aligned with the MSB of the 8-bit encoder input.
///   For example, for RGB565 (red most significant), red is bits 15:11, so should be right-rotated by 8 bits to align with bits 7:3 of the encoder input.
/// - **L1_ROT** `[7:4]` (RW)
///   Right-rotate the 16 LSBs of the colour accumulator by 0-15 bits, in order to get the MSB of the lane 1 (green) colour data aligned with the MSB of the 8-bit encoder input.
///   For example, for RGB565, green is bits 10:5, so should be right-rotated by 3 bits to align with bits 7:2 of the encoder input.
/// - **L0_ROT** `[3:0]` (RW)
///   Right-rotate the 16 LSBs of the colour accumulator by 0-15 bits, in order to get the MSB of the lane 0 (blue) colour data aligned with the MSB of the 8-bit encoder input.
///   For example, for RGB565 (red most significant), blue is bits 4:0, so should be right-rotated by 13 to align with bits 7:3 of the encoder input.
#[repr(transparent)]
pub struct TmdsCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TmdsCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TmdsCtrl {
    // ======== Field Constants ========

    /// **CLEAR_BALANCE**
    /// Clear the running DC balance state of the TMDS encoders. This bit should be written once at the beginning of each scanline.
    /// Bits `[28:28]`, mask `0x10000000`
    pub const CLEAR_BALANCE_OFFSET: u32 = 28;
    pub const CLEAR_BALANCE_MASK: u32 = 0x10000000;
    pub const CLEAR_BALANCE: u32 = 1 << 28;

    /// **PIX2_NOSHIFT**
    /// When encoding two pixels's worth of symbols in one cycle (a read of a PEEK/POP_DOUBLE register), the second encoder sees a shifted version of the colour data register.
    /// This control disables that shift, so that both encoder layers see the same pixel data. This is used for pixel doubling.
    /// Bits `[27:27]`, mask `0x08000000`
    pub const PIX2_NOSHIFT_OFFSET: u32 = 27;
    pub const PIX2_NOSHIFT_MASK: u32 = 0x08000000;
    pub const PIX2_NOSHIFT: u32 = 1 << 27;

    /// **PIX_SHIFT**
    /// Shift applied to the colour data register with each read of a POP alias register.
    /// Reading from the POP_SINGLE register, or reading from the POP_DOUBLE register with PIX2_NOSHIFT set (for pixel doubling), shifts by the indicated amount.
    /// Reading from a POP_DOUBLE register when PIX2_NOSHIFT is clear will shift by double the indicated amount. (Shift by 32 means no shift.)
    /// Bits `[26:24]`, mask `0x07000000`
    pub const PIX_SHIFT_OFFSET: u32 = 24;
    pub const PIX_SHIFT_MASK: u32 = 0x07000000;
    /// 0 = 0
    /// Do not shift the colour data register.
    pub const PIX_SHIFT_0: u32 = 0 << 24;
    pub const PIX_SHIFT_0_VAL: u32 = 0;
    /// 1 = 1
    /// Shift the colour data register by 1 bit
    pub const PIX_SHIFT_1: u32 = 1 << 24;
    pub const PIX_SHIFT_1_VAL: u32 = 1;
    /// 2 = 2
    /// Shift the colour data register by 2 bits
    pub const PIX_SHIFT_2: u32 = 2 << 24;
    pub const PIX_SHIFT_2_VAL: u32 = 2;
    /// 4 = 3
    /// Shift the colour data register by 4 bits
    pub const PIX_SHIFT_4: u32 = 3 << 24;
    pub const PIX_SHIFT_4_VAL: u32 = 3;
    /// 8 = 4
    /// Shift the colour data register by 8 bits
    pub const PIX_SHIFT_8: u32 = 4 << 24;
    pub const PIX_SHIFT_8_VAL: u32 = 4;
    /// 16 = 5
    /// Shift the colour data register by 16 bits
    pub const PIX_SHIFT_16: u32 = 5 << 24;
    pub const PIX_SHIFT_16_VAL: u32 = 5;

    /// **INTERLEAVE**
    /// Enable lane interleaving for reads of PEEK_SINGLE/POP_SINGLE.
    /// When interleaving is disabled, each of the 3 symbols appears as a contiguous 10-bit field, with lane 0 being the least-significant and starting at bit 0 of the register.
    /// When interleaving is enabled, the symbols are packed into 5 chunks of 3 lanes times 2 bits (30 bits total). Each chunk contains two bits of a TMDS symbol per lane, with lane 0 being the least significant.
    /// Bits `[23:23]`, mask `0x00800000`
    pub const INTERLEAVE_OFFSET: u32 = 23;
    pub const INTERLEAVE_MASK: u32 = 0x00800000;
    pub const INTERLEAVE: u32 = 1 << 23;

    /// **L2_NBITS**
    /// Number of valid colour MSBs for lane 2 (1-8 bits, encoded as 0 through 7). Remaining LSBs are masked to 0 after the rotate.
    /// Bits `[20:18]`, mask `0x001c0000`
    pub const L2_NBITS_OFFSET: u32 = 18;
    pub const L2_NBITS_MASK: u32 = 0x001c0000;

    /// **L1_NBITS**
    /// Number of valid colour MSBs for lane 1 (1-8 bits, encoded as 0 through 7). Remaining LSBs are masked to 0 after the rotate.
    /// Bits `[17:15]`, mask `0x00038000`
    pub const L1_NBITS_OFFSET: u32 = 15;
    pub const L1_NBITS_MASK: u32 = 0x00038000;

    /// **L0_NBITS**
    /// Number of valid colour MSBs for lane 0 (1-8 bits, encoded as 0 through 7). Remaining LSBs are masked to 0 after the rotate.
    /// Bits `[14:12]`, mask `0x00007000`
    pub const L0_NBITS_OFFSET: u32 = 12;
    pub const L0_NBITS_MASK: u32 = 0x00007000;

    /// **L2_ROT**
    /// Right-rotate the 16 LSBs of the colour accumulator by 0-15 bits, in order to get the MSB of the lane 2 (red) colour data aligned with the MSB of the 8-bit encoder input.
    /// For example, for RGB565 (red most significant), red is bits 15:11, so should be right-rotated by 8 bits to align with bits 7:3 of the encoder input.
    /// Bits `[11:8]`, mask `0x00000f00`
    pub const L2_ROT_OFFSET: u32 = 8;
    pub const L2_ROT_MASK: u32 = 0x00000f00;

    /// **L1_ROT**
    /// Right-rotate the 16 LSBs of the colour accumulator by 0-15 bits, in order to get the MSB of the lane 1 (green) colour data aligned with the MSB of the 8-bit encoder input.
    /// For example, for RGB565, green is bits 10:5, so should be right-rotated by 3 bits to align with bits 7:2 of the encoder input.
    /// Bits `[7:4]`, mask `0x000000f0`
    pub const L1_ROT_OFFSET: u32 = 4;
    pub const L1_ROT_MASK: u32 = 0x000000f0;

    /// **L0_ROT**
    /// Right-rotate the 16 LSBs of the colour accumulator by 0-15 bits, in order to get the MSB of the lane 0 (blue) colour data aligned with the MSB of the 8-bit encoder input.
    /// For example, for RGB565 (red most significant), blue is bits 4:0, so should be right-rotated by 13 to align with bits 7:3 of the encoder input.
    /// Bits `[3:0]`, mask `0x0000000f`
    pub const L0_ROT_OFFSET: u32 = 0;
    pub const L0_ROT_MASK: u32 = 0x0000000f;

    // ======== Field Accessors ========

    /// **CLEAR_BALANCE** `[28]` (WO)
    /// Clear the running DC balance state of the TMDS encoders. This bit should be written once at the beginning of each scanline.
    /// Trigger **CLEAR_BALANCE** (WO)
    /// Clear the running DC balance state of the TMDS encoders. This bit should be written once at the beginning of each scanline.
    #[inline(always)]
    pub fn trigger_clear_balance(&self) {
        self.value.write_bits(Self::CLEAR_BALANCE_MASK);
    }

    /// **PIX2_NOSHIFT** `[27]` (RW)
    /// When encoding two pixels's worth of symbols in one cycle (a read of a PEEK/POP_DOUBLE register), the second encoder sees a shifted version of the colour data register.
    /// This control disables that shift, so that both encoder layers see the same pixel data. This is used for pixel doubling.
    #[inline(always)]
    pub fn pix2_noshift(&self) -> bool {
        (self.value.read_bits() & Self::PIX2_NOSHIFT_MASK) != 0
    }

    /// Set PIX2_NOSHIFT bit atomically (RW)
    #[inline(always)]
    pub fn set_pix2_noshift(&self) {
        self.value.set_bits(Self::PIX2_NOSHIFT_MASK);
    }

    /// Clear PIX2_NOSHIFT bit atomically (RW)
    #[inline(always)]
    pub fn clr_pix2_noshift(&self) {
        self.value.clr_bits(Self::PIX2_NOSHIFT_MASK);
    }

    /// **PIX_SHIFT** `[26:24]` (RW)
    /// Shift applied to the colour data register with each read of a POP alias register.
    /// Reading from the POP_SINGLE register, or reading from the POP_DOUBLE register with PIX2_NOSHIFT set (for pixel doubling), shifts by the indicated amount.
    /// Reading from a POP_DOUBLE register when PIX2_NOSHIFT is clear will shift by double the indicated amount. (Shift by 32 means no shift.)
    #[inline(always)]
    pub fn pix_shift(&self) -> u32 {
        (self.value.read_bits() & Self::PIX_SHIFT_MASK) >> Self::PIX_SHIFT_OFFSET
    }

    /// Write **PIX_SHIFT** `[26:24]` (RW)
    /// Shift applied to the colour data register with each read of a POP alias register.
    /// Reading from the POP_SINGLE register, or reading from the POP_DOUBLE register with PIX2_NOSHIFT set (for pixel doubling), shifts by the indicated amount.
    /// Reading from a POP_DOUBLE register when PIX2_NOSHIFT is clear will shift by double the indicated amount. (Shift by 32 means no shift.)
    #[inline(always)]
    pub fn write_pix_shift(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::PIX_SHIFT_MASK) | ((value << Self::PIX_SHIFT_OFFSET) & Self::PIX_SHIFT_MASK));
    }

    /// **INTERLEAVE** `[23]` (RW)
    /// Enable lane interleaving for reads of PEEK_SINGLE/POP_SINGLE.
    /// When interleaving is disabled, each of the 3 symbols appears as a contiguous 10-bit field, with lane 0 being the least-significant and starting at bit 0 of the register.
    /// When interleaving is enabled, the symbols are packed into 5 chunks of 3 lanes times 2 bits (30 bits total). Each chunk contains two bits of a TMDS symbol per lane, with lane 0 being the least significant.
    #[inline(always)]
    pub fn interleave(&self) -> bool {
        (self.value.read_bits() & Self::INTERLEAVE_MASK) != 0
    }

    /// Set INTERLEAVE bit atomically (RW)
    #[inline(always)]
    pub fn set_interleave(&self) {
        self.value.set_bits(Self::INTERLEAVE_MASK);
    }

    /// Clear INTERLEAVE bit atomically (RW)
    #[inline(always)]
    pub fn clr_interleave(&self) {
        self.value.clr_bits(Self::INTERLEAVE_MASK);
    }

    /// **L2_NBITS** `[20:18]` (RW)
    /// Number of valid colour MSBs for lane 2 (1-8 bits, encoded as 0 through 7). Remaining LSBs are masked to 0 after the rotate.
    #[inline(always)]
    pub fn l2_nbits(&self) -> u32 {
        (self.value.read_bits() & Self::L2_NBITS_MASK) >> Self::L2_NBITS_OFFSET
    }

    /// Write **L2_NBITS** `[20:18]` (RW)
    /// Number of valid colour MSBs for lane 2 (1-8 bits, encoded as 0 through 7). Remaining LSBs are masked to 0 after the rotate.
    #[inline(always)]
    pub fn write_l2_nbits(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::L2_NBITS_MASK) | ((value << Self::L2_NBITS_OFFSET) & Self::L2_NBITS_MASK));
    }

    /// **L1_NBITS** `[17:15]` (RW)
    /// Number of valid colour MSBs for lane 1 (1-8 bits, encoded as 0 through 7). Remaining LSBs are masked to 0 after the rotate.
    #[inline(always)]
    pub fn l1_nbits(&self) -> u32 {
        (self.value.read_bits() & Self::L1_NBITS_MASK) >> Self::L1_NBITS_OFFSET
    }

    /// Write **L1_NBITS** `[17:15]` (RW)
    /// Number of valid colour MSBs for lane 1 (1-8 bits, encoded as 0 through 7). Remaining LSBs are masked to 0 after the rotate.
    #[inline(always)]
    pub fn write_l1_nbits(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::L1_NBITS_MASK) | ((value << Self::L1_NBITS_OFFSET) & Self::L1_NBITS_MASK));
    }

    /// **L0_NBITS** `[14:12]` (RW)
    /// Number of valid colour MSBs for lane 0 (1-8 bits, encoded as 0 through 7). Remaining LSBs are masked to 0 after the rotate.
    #[inline(always)]
    pub fn l0_nbits(&self) -> u32 {
        (self.value.read_bits() & Self::L0_NBITS_MASK) >> Self::L0_NBITS_OFFSET
    }

    /// Write **L0_NBITS** `[14:12]` (RW)
    /// Number of valid colour MSBs for lane 0 (1-8 bits, encoded as 0 through 7). Remaining LSBs are masked to 0 after the rotate.
    #[inline(always)]
    pub fn write_l0_nbits(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::L0_NBITS_MASK) | ((value << Self::L0_NBITS_OFFSET) & Self::L0_NBITS_MASK));
    }

    /// **L2_ROT** `[11:8]` (RW)
    /// Right-rotate the 16 LSBs of the colour accumulator by 0-15 bits, in order to get the MSB of the lane 2 (red) colour data aligned with the MSB of the 8-bit encoder input.
    /// For example, for RGB565 (red most significant), red is bits 15:11, so should be right-rotated by 8 bits to align with bits 7:3 of the encoder input.
    #[inline(always)]
    pub fn l2_rot(&self) -> u32 {
        (self.value.read_bits() & Self::L2_ROT_MASK) >> Self::L2_ROT_OFFSET
    }

    /// Write **L2_ROT** `[11:8]` (RW)
    /// Right-rotate the 16 LSBs of the colour accumulator by 0-15 bits, in order to get the MSB of the lane 2 (red) colour data aligned with the MSB of the 8-bit encoder input.
    /// For example, for RGB565 (red most significant), red is bits 15:11, so should be right-rotated by 8 bits to align with bits 7:3 of the encoder input.
    #[inline(always)]
    pub fn write_l2_rot(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::L2_ROT_MASK) | ((value << Self::L2_ROT_OFFSET) & Self::L2_ROT_MASK));
    }

    /// **L1_ROT** `[7:4]` (RW)
    /// Right-rotate the 16 LSBs of the colour accumulator by 0-15 bits, in order to get the MSB of the lane 1 (green) colour data aligned with the MSB of the 8-bit encoder input.
    /// For example, for RGB565, green is bits 10:5, so should be right-rotated by 3 bits to align with bits 7:2 of the encoder input.
    #[inline(always)]
    pub fn l1_rot(&self) -> u32 {
        (self.value.read_bits() & Self::L1_ROT_MASK) >> Self::L1_ROT_OFFSET
    }

    /// Write **L1_ROT** `[7:4]` (RW)
    /// Right-rotate the 16 LSBs of the colour accumulator by 0-15 bits, in order to get the MSB of the lane 1 (green) colour data aligned with the MSB of the 8-bit encoder input.
    /// For example, for RGB565, green is bits 10:5, so should be right-rotated by 3 bits to align with bits 7:2 of the encoder input.
    #[inline(always)]
    pub fn write_l1_rot(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::L1_ROT_MASK) | ((value << Self::L1_ROT_OFFSET) & Self::L1_ROT_MASK));
    }

    /// **L0_ROT** `[3:0]` (RW)
    /// Right-rotate the 16 LSBs of the colour accumulator by 0-15 bits, in order to get the MSB of the lane 0 (blue) colour data aligned with the MSB of the 8-bit encoder input.
    /// For example, for RGB565 (red most significant), blue is bits 4:0, so should be right-rotated by 13 to align with bits 7:3 of the encoder input.
    #[inline(always)]
    pub fn l0_rot(&self) -> u32 {
        (self.value.read_bits() & Self::L0_ROT_MASK) >> Self::L0_ROT_OFFSET
    }

    /// Write **L0_ROT** `[3:0]` (RW)
    /// Right-rotate the 16 LSBs of the colour accumulator by 0-15 bits, in order to get the MSB of the lane 0 (blue) colour data aligned with the MSB of the 8-bit encoder input.
    /// For example, for RGB565 (red most significant), blue is bits 4:0, so should be right-rotated by 13 to align with bits 7:3 of the encoder input.
    #[inline(always)]
    pub fn write_l0_rot(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::L0_ROT_MASK) | ((value << Self::L0_ROT_OFFSET) & Self::L0_ROT_MASK));
    }

}

/// TMDS_CTRL value type for batch read/modify/write operations
///
/// Use `tmdsctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `tmdsctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TmdsCtrlVal {
    bits: u32,
}

impl TmdsCtrlVal {
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

    /// Read **PIX2_NOSHIFT** `[27:27]`
    /// When encoding two pixels's worth of symbols in one cycle (a read of a PEEK/POP_DOUBLE register), the second encoder sees a shifted version of the colour data register.
    /// This control disables that shift, so that both encoder layers see the same pixel data. This is used for pixel doubling.
    #[inline(always)]
    pub const fn pix2_noshift(&self) -> bool {
        (self.bits & TmdsCtrl::PIX2_NOSHIFT_MASK) != 0
    }

    /// Read **PIX_SHIFT** `[26:24]`
    /// Shift applied to the colour data register with each read of a POP alias register.
    /// Reading from the POP_SINGLE register, or reading from the POP_DOUBLE register with PIX2_NOSHIFT set (for pixel doubling), shifts by the indicated amount.
    /// Reading from a POP_DOUBLE register when PIX2_NOSHIFT is clear will shift by double the indicated amount. (Shift by 32 means no shift.)
    #[inline(always)]
    pub const fn pix_shift(&self) -> u32 {
        (self.bits & TmdsCtrl::PIX_SHIFT_MASK) >> TmdsCtrl::PIX_SHIFT_OFFSET
    }

    /// Read **INTERLEAVE** `[23:23]`
    /// Enable lane interleaving for reads of PEEK_SINGLE/POP_SINGLE.
    /// When interleaving is disabled, each of the 3 symbols appears as a contiguous 10-bit field, with lane 0 being the least-significant and starting at bit 0 of the register.
    /// When interleaving is enabled, the symbols are packed into 5 chunks of 3 lanes times 2 bits (30 bits total). Each chunk contains two bits of a TMDS symbol per lane, with lane 0 being the least significant.
    #[inline(always)]
    pub const fn interleave(&self) -> bool {
        (self.bits & TmdsCtrl::INTERLEAVE_MASK) != 0
    }

    /// Read **L2_NBITS** `[20:18]`
    /// Number of valid colour MSBs for lane 2 (1-8 bits, encoded as 0 through 7). Remaining LSBs are masked to 0 after the rotate.
    #[inline(always)]
    pub const fn l2_nbits(&self) -> u32 {
        (self.bits & TmdsCtrl::L2_NBITS_MASK) >> TmdsCtrl::L2_NBITS_OFFSET
    }

    /// Read **L1_NBITS** `[17:15]`
    /// Number of valid colour MSBs for lane 1 (1-8 bits, encoded as 0 through 7). Remaining LSBs are masked to 0 after the rotate.
    #[inline(always)]
    pub const fn l1_nbits(&self) -> u32 {
        (self.bits & TmdsCtrl::L1_NBITS_MASK) >> TmdsCtrl::L1_NBITS_OFFSET
    }

    /// Read **L0_NBITS** `[14:12]`
    /// Number of valid colour MSBs for lane 0 (1-8 bits, encoded as 0 through 7). Remaining LSBs are masked to 0 after the rotate.
    #[inline(always)]
    pub const fn l0_nbits(&self) -> u32 {
        (self.bits & TmdsCtrl::L0_NBITS_MASK) >> TmdsCtrl::L0_NBITS_OFFSET
    }

    /// Read **L2_ROT** `[11:8]`
    /// Right-rotate the 16 LSBs of the colour accumulator by 0-15 bits, in order to get the MSB of the lane 2 (red) colour data aligned with the MSB of the 8-bit encoder input.
    /// For example, for RGB565 (red most significant), red is bits 15:11, so should be right-rotated by 8 bits to align with bits 7:3 of the encoder input.
    #[inline(always)]
    pub const fn l2_rot(&self) -> u32 {
        (self.bits & TmdsCtrl::L2_ROT_MASK) >> TmdsCtrl::L2_ROT_OFFSET
    }

    /// Read **L1_ROT** `[7:4]`
    /// Right-rotate the 16 LSBs of the colour accumulator by 0-15 bits, in order to get the MSB of the lane 1 (green) colour data aligned with the MSB of the 8-bit encoder input.
    /// For example, for RGB565, green is bits 10:5, so should be right-rotated by 3 bits to align with bits 7:2 of the encoder input.
    #[inline(always)]
    pub const fn l1_rot(&self) -> u32 {
        (self.bits & TmdsCtrl::L1_ROT_MASK) >> TmdsCtrl::L1_ROT_OFFSET
    }

    /// Read **L0_ROT** `[3:0]`
    /// Right-rotate the 16 LSBs of the colour accumulator by 0-15 bits, in order to get the MSB of the lane 0 (blue) colour data aligned with the MSB of the 8-bit encoder input.
    /// For example, for RGB565 (red most significant), blue is bits 4:0, so should be right-rotated by 13 to align with bits 7:3 of the encoder input.
    #[inline(always)]
    pub const fn l0_rot(&self) -> u32 {
        (self.bits & TmdsCtrl::L0_ROT_MASK) >> TmdsCtrl::L0_ROT_OFFSET
    }

    /// Set **CLEAR_BALANCE** `[28:28]`
    /// Clear the running DC balance state of the TMDS encoders. This bit should be written once at the beginning of each scanline.
    #[inline(always)]
    pub const fn with_clear_balance(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TmdsCtrl::CLEAR_BALANCE_MASK
            } else {
                self.bits & !TmdsCtrl::CLEAR_BALANCE_MASK
            }
        }
    }

    /// Set **PIX2_NOSHIFT** `[27:27]`
    /// When encoding two pixels's worth of symbols in one cycle (a read of a PEEK/POP_DOUBLE register), the second encoder sees a shifted version of the colour data register.
    /// This control disables that shift, so that both encoder layers see the same pixel data. This is used for pixel doubling.
    #[inline(always)]
    pub const fn with_pix2_noshift(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TmdsCtrl::PIX2_NOSHIFT_MASK
            } else {
                self.bits & !TmdsCtrl::PIX2_NOSHIFT_MASK
            }
        }
    }

    /// Set **PIX_SHIFT** `[26:24]`
    /// Shift applied to the colour data register with each read of a POP alias register.
    /// Reading from the POP_SINGLE register, or reading from the POP_DOUBLE register with PIX2_NOSHIFT set (for pixel doubling), shifts by the indicated amount.
    /// Reading from a POP_DOUBLE register when PIX2_NOSHIFT is clear will shift by double the indicated amount. (Shift by 32 means no shift.)
    #[inline(always)]
    pub const fn with_pix_shift(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TmdsCtrl::PIX_SHIFT_MASK) | ((value << TmdsCtrl::PIX_SHIFT_OFFSET) & TmdsCtrl::PIX_SHIFT_MASK)
        }
    }

    /// Set **INTERLEAVE** `[23:23]`
    /// Enable lane interleaving for reads of PEEK_SINGLE/POP_SINGLE.
    /// When interleaving is disabled, each of the 3 symbols appears as a contiguous 10-bit field, with lane 0 being the least-significant and starting at bit 0 of the register.
    /// When interleaving is enabled, the symbols are packed into 5 chunks of 3 lanes times 2 bits (30 bits total). Each chunk contains two bits of a TMDS symbol per lane, with lane 0 being the least significant.
    #[inline(always)]
    pub const fn with_interleave(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TmdsCtrl::INTERLEAVE_MASK
            } else {
                self.bits & !TmdsCtrl::INTERLEAVE_MASK
            }
        }
    }

    /// Set **L2_NBITS** `[20:18]`
    /// Number of valid colour MSBs for lane 2 (1-8 bits, encoded as 0 through 7). Remaining LSBs are masked to 0 after the rotate.
    #[inline(always)]
    pub const fn with_l2_nbits(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TmdsCtrl::L2_NBITS_MASK) | ((value << TmdsCtrl::L2_NBITS_OFFSET) & TmdsCtrl::L2_NBITS_MASK)
        }
    }

    /// Set **L1_NBITS** `[17:15]`
    /// Number of valid colour MSBs for lane 1 (1-8 bits, encoded as 0 through 7). Remaining LSBs are masked to 0 after the rotate.
    #[inline(always)]
    pub const fn with_l1_nbits(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TmdsCtrl::L1_NBITS_MASK) | ((value << TmdsCtrl::L1_NBITS_OFFSET) & TmdsCtrl::L1_NBITS_MASK)
        }
    }

    /// Set **L0_NBITS** `[14:12]`
    /// Number of valid colour MSBs for lane 0 (1-8 bits, encoded as 0 through 7). Remaining LSBs are masked to 0 after the rotate.
    #[inline(always)]
    pub const fn with_l0_nbits(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TmdsCtrl::L0_NBITS_MASK) | ((value << TmdsCtrl::L0_NBITS_OFFSET) & TmdsCtrl::L0_NBITS_MASK)
        }
    }

    /// Set **L2_ROT** `[11:8]`
    /// Right-rotate the 16 LSBs of the colour accumulator by 0-15 bits, in order to get the MSB of the lane 2 (red) colour data aligned with the MSB of the 8-bit encoder input.
    /// For example, for RGB565 (red most significant), red is bits 15:11, so should be right-rotated by 8 bits to align with bits 7:3 of the encoder input.
    #[inline(always)]
    pub const fn with_l2_rot(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TmdsCtrl::L2_ROT_MASK) | ((value << TmdsCtrl::L2_ROT_OFFSET) & TmdsCtrl::L2_ROT_MASK)
        }
    }

    /// Set **L1_ROT** `[7:4]`
    /// Right-rotate the 16 LSBs of the colour accumulator by 0-15 bits, in order to get the MSB of the lane 1 (green) colour data aligned with the MSB of the 8-bit encoder input.
    /// For example, for RGB565, green is bits 10:5, so should be right-rotated by 3 bits to align with bits 7:2 of the encoder input.
    #[inline(always)]
    pub const fn with_l1_rot(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TmdsCtrl::L1_ROT_MASK) | ((value << TmdsCtrl::L1_ROT_OFFSET) & TmdsCtrl::L1_ROT_MASK)
        }
    }

    /// Set **L0_ROT** `[3:0]`
    /// Right-rotate the 16 LSBs of the colour accumulator by 0-15 bits, in order to get the MSB of the lane 0 (blue) colour data aligned with the MSB of the 8-bit encoder input.
    /// For example, for RGB565 (red most significant), blue is bits 4:0, so should be right-rotated by 13 to align with bits 7:3 of the encoder input.
    #[inline(always)]
    pub const fn with_l0_rot(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TmdsCtrl::L0_ROT_MASK) | ((value << TmdsCtrl::L0_ROT_OFFSET) & TmdsCtrl::L0_ROT_MASK)
        }
    }

}

impl Default for TmdsCtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl TmdsCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TmdsCtrlVal {
        TmdsCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TmdsCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TmdsCtrlVal) -> TmdsCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// TMDS_WDATA Register
///
/// Write-only access to the TMDS colour data register.
///
/// Address offset: 0x01c4
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **TMDS_WDATA** `[31:0]` (WO)
#[repr(transparent)]
pub struct TmdsWdata {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TmdsWdata {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TmdsWdata {
    // ======== Field Constants ========

    /// **TMDS_WDATA**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const TMDS_WDATA_OFFSET: u32 = 0;
    pub const TMDS_WDATA_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **TMDS_WDATA** `[31:0]` (WO)
    /// Write **TMDS_WDATA** `[31:0]` (WO)
    #[inline(always)]
    pub fn write_tmds_wdata(&self, value: u32) {
        self.value.write_bits((value << Self::TMDS_WDATA_OFFSET) & Self::TMDS_WDATA_MASK);
    }

}

/// TMDS_WDATA value type for batch read/modify/write operations
///
/// Use `tmdswdata.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `tmdswdata.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TmdsWdataVal {
    bits: u32,
}

impl TmdsWdataVal {
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

    /// Set **TMDS_WDATA** `[31:0]`
    #[inline(always)]
    pub const fn with_tmds_wdata(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TmdsWdata::TMDS_WDATA_MASK) | ((value << TmdsWdata::TMDS_WDATA_OFFSET) & TmdsWdata::TMDS_WDATA_MASK)
        }
    }

}

impl Default for TmdsWdataVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl TmdsWdata {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TmdsWdataVal {
        TmdsWdataVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TmdsWdataVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TmdsWdataVal) -> TmdsWdataVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// TMDS_PEEK_SINGLE Register
///
/// Get the encoding of one pixel's worth of colour data, packed into a 32-bit value (3x10-bit symbols).
/// The PEEK alias does not shift the colour register when read, but still advances the running DC balance state of each encoder. This is useful for pixel doubling.
///
/// Address offset: 0x01c8
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **TMDS_PEEK_SINGLE** `[31:0]` (RO)
#[repr(transparent)]
pub struct TmdsPeekSingle {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TmdsPeekSingle {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TmdsPeekSingle {
    // ======== Field Constants ========

    /// **TMDS_PEEK_SINGLE**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const TMDS_PEEK_SINGLE_OFFSET: u32 = 0;
    pub const TMDS_PEEK_SINGLE_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **TMDS_PEEK_SINGLE** `[31:0]` (RO)
    #[inline(always)]
    pub fn tmds_peek_single(&self) -> u32 {
        (self.value.read_bits() & Self::TMDS_PEEK_SINGLE_MASK) >> Self::TMDS_PEEK_SINGLE_OFFSET
    }

}

/// TMDS_PEEK_SINGLE value type for batch read/modify/write operations
///
/// Use `tmdspeeksingle.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `tmdspeeksingle.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TmdsPeekSingleVal {
    bits: u32,
}

impl TmdsPeekSingleVal {
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

    /// Read **TMDS_PEEK_SINGLE** `[31:0]`
    #[inline(always)]
    pub const fn tmds_peek_single(&self) -> u32 {
        (self.bits & TmdsPeekSingle::TMDS_PEEK_SINGLE_MASK) >> TmdsPeekSingle::TMDS_PEEK_SINGLE_OFFSET
    }

}

impl Default for TmdsPeekSingleVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl TmdsPeekSingle {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TmdsPeekSingleVal {
        TmdsPeekSingleVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TmdsPeekSingleVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TmdsPeekSingleVal) -> TmdsPeekSingleVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// TMDS_POP_SINGLE Register
///
/// Get the encoding of one pixel's worth of colour data, packed into a 32-bit value. The packing is 5 chunks of 3 lanes times 2 bits (30 bits total). Each chunk contains two bits of a TMDS symbol per lane. This format is intended for shifting out with the HSTX peripheral on RP2350.
/// The POP alias shifts the colour register when read, as well as advancing the running DC balance state of each encoder.
///
/// Address offset: 0x01cc
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **TMDS_POP_SINGLE** `[31:0]` (RO)
#[repr(transparent)]
pub struct TmdsPopSingle {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TmdsPopSingle {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TmdsPopSingle {
    // ======== Field Constants ========

    /// **TMDS_POP_SINGLE**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const TMDS_POP_SINGLE_OFFSET: u32 = 0;
    pub const TMDS_POP_SINGLE_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **TMDS_POP_SINGLE** `[31:0]` (RO)
    #[inline(always)]
    pub fn tmds_pop_single(&self) -> u32 {
        (self.value.read_bits() & Self::TMDS_POP_SINGLE_MASK) >> Self::TMDS_POP_SINGLE_OFFSET
    }

}

/// TMDS_POP_SINGLE value type for batch read/modify/write operations
///
/// Use `tmdspopsingle.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `tmdspopsingle.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TmdsPopSingleVal {
    bits: u32,
}

impl TmdsPopSingleVal {
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

    /// Read **TMDS_POP_SINGLE** `[31:0]`
    #[inline(always)]
    pub const fn tmds_pop_single(&self) -> u32 {
        (self.bits & TmdsPopSingle::TMDS_POP_SINGLE_MASK) >> TmdsPopSingle::TMDS_POP_SINGLE_OFFSET
    }

}

impl Default for TmdsPopSingleVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl TmdsPopSingle {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TmdsPopSingleVal {
        TmdsPopSingleVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TmdsPopSingleVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TmdsPopSingleVal) -> TmdsPopSingleVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// TMDS_PEEK_DOUBLE_L0 Register
///
/// Get lane 0 of the encoding of two pixels' worth of colour data. Two 10-bit TMDS symbols are packed at the bottom of a 32-bit word.
/// The PEEK alias does not shift the colour register when read, but still advances the lane 0 DC balance state. This is useful if all 3 lanes' worth of encode are to be read at once, rather than processing the entire scanline for one lane before moving to the next lane.
///
/// Address offset: 0x01d0
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **TMDS_PEEK_DOUBLE_L0** `[31:0]` (RO)
#[repr(transparent)]
pub struct TmdsPeekDoubleL0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TmdsPeekDoubleL0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TmdsPeekDoubleL0 {
    // ======== Field Constants ========

    /// **TMDS_PEEK_DOUBLE_L0**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const TMDS_PEEK_DOUBLE_L0_OFFSET: u32 = 0;
    pub const TMDS_PEEK_DOUBLE_L0_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **TMDS_PEEK_DOUBLE_L0** `[31:0]` (RO)
    #[inline(always)]
    pub fn tmds_peek_double_l0(&self) -> u32 {
        (self.value.read_bits() & Self::TMDS_PEEK_DOUBLE_L0_MASK) >> Self::TMDS_PEEK_DOUBLE_L0_OFFSET
    }

}

/// TMDS_PEEK_DOUBLE_L0 value type for batch read/modify/write operations
///
/// Use `tmdspeekdoublel0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `tmdspeekdoublel0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TmdsPeekDoubleL0Val {
    bits: u32,
}

impl TmdsPeekDoubleL0Val {
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

    /// Read **TMDS_PEEK_DOUBLE_L0** `[31:0]`
    #[inline(always)]
    pub const fn tmds_peek_double_l0(&self) -> u32 {
        (self.bits & TmdsPeekDoubleL0::TMDS_PEEK_DOUBLE_L0_MASK) >> TmdsPeekDoubleL0::TMDS_PEEK_DOUBLE_L0_OFFSET
    }

}

impl Default for TmdsPeekDoubleL0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl TmdsPeekDoubleL0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TmdsPeekDoubleL0Val {
        TmdsPeekDoubleL0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TmdsPeekDoubleL0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TmdsPeekDoubleL0Val) -> TmdsPeekDoubleL0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// TMDS_POP_DOUBLE_L0 Register
///
/// Get lane 0 of the encoding of two pixels' worth of colour data. Two 10-bit TMDS symbols are packed at the bottom of a 32-bit word.
/// The POP alias shifts the colour register when read, according to the values of PIX_SHIFT and PIX2_NOSHIFT.
///
/// Address offset: 0x01d4
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **TMDS_POP_DOUBLE_L0** `[31:0]` (RO)
#[repr(transparent)]
pub struct TmdsPopDoubleL0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TmdsPopDoubleL0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TmdsPopDoubleL0 {
    // ======== Field Constants ========

    /// **TMDS_POP_DOUBLE_L0**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const TMDS_POP_DOUBLE_L0_OFFSET: u32 = 0;
    pub const TMDS_POP_DOUBLE_L0_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **TMDS_POP_DOUBLE_L0** `[31:0]` (RO)
    #[inline(always)]
    pub fn tmds_pop_double_l0(&self) -> u32 {
        (self.value.read_bits() & Self::TMDS_POP_DOUBLE_L0_MASK) >> Self::TMDS_POP_DOUBLE_L0_OFFSET
    }

}

/// TMDS_POP_DOUBLE_L0 value type for batch read/modify/write operations
///
/// Use `tmdspopdoublel0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `tmdspopdoublel0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TmdsPopDoubleL0Val {
    bits: u32,
}

impl TmdsPopDoubleL0Val {
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

    /// Read **TMDS_POP_DOUBLE_L0** `[31:0]`
    #[inline(always)]
    pub const fn tmds_pop_double_l0(&self) -> u32 {
        (self.bits & TmdsPopDoubleL0::TMDS_POP_DOUBLE_L0_MASK) >> TmdsPopDoubleL0::TMDS_POP_DOUBLE_L0_OFFSET
    }

}

impl Default for TmdsPopDoubleL0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl TmdsPopDoubleL0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TmdsPopDoubleL0Val {
        TmdsPopDoubleL0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TmdsPopDoubleL0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TmdsPopDoubleL0Val) -> TmdsPopDoubleL0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// TMDS_PEEK_DOUBLE_L1 Register
///
/// Get lane 1 of the encoding of two pixels' worth of colour data. Two 10-bit TMDS symbols are packed at the bottom of a 32-bit word.
/// The PEEK alias does not shift the colour register when read, but still advances the lane 1 DC balance state. This is useful if all 3 lanes' worth of encode are to be read at once, rather than processing the entire scanline for one lane before moving to the next lane.
///
/// Address offset: 0x01d8
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **TMDS_PEEK_DOUBLE_L1** `[31:0]` (RO)
#[repr(transparent)]
pub struct TmdsPeekDoubleL1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TmdsPeekDoubleL1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TmdsPeekDoubleL1 {
    // ======== Field Constants ========

    /// **TMDS_PEEK_DOUBLE_L1**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const TMDS_PEEK_DOUBLE_L1_OFFSET: u32 = 0;
    pub const TMDS_PEEK_DOUBLE_L1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **TMDS_PEEK_DOUBLE_L1** `[31:0]` (RO)
    #[inline(always)]
    pub fn tmds_peek_double_l1(&self) -> u32 {
        (self.value.read_bits() & Self::TMDS_PEEK_DOUBLE_L1_MASK) >> Self::TMDS_PEEK_DOUBLE_L1_OFFSET
    }

}

/// TMDS_PEEK_DOUBLE_L1 value type for batch read/modify/write operations
///
/// Use `tmdspeekdoublel1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `tmdspeekdoublel1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TmdsPeekDoubleL1Val {
    bits: u32,
}

impl TmdsPeekDoubleL1Val {
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

    /// Read **TMDS_PEEK_DOUBLE_L1** `[31:0]`
    #[inline(always)]
    pub const fn tmds_peek_double_l1(&self) -> u32 {
        (self.bits & TmdsPeekDoubleL1::TMDS_PEEK_DOUBLE_L1_MASK) >> TmdsPeekDoubleL1::TMDS_PEEK_DOUBLE_L1_OFFSET
    }

}

impl Default for TmdsPeekDoubleL1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl TmdsPeekDoubleL1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TmdsPeekDoubleL1Val {
        TmdsPeekDoubleL1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TmdsPeekDoubleL1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TmdsPeekDoubleL1Val) -> TmdsPeekDoubleL1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// TMDS_POP_DOUBLE_L1 Register
///
/// Get lane 1 of the encoding of two pixels' worth of colour data. Two 10-bit TMDS symbols are packed at the bottom of a 32-bit word.
/// The POP alias shifts the colour register when read, according to the values of PIX_SHIFT and PIX2_NOSHIFT.
///
/// Address offset: 0x01dc
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **TMDS_POP_DOUBLE_L1** `[31:0]` (RO)
#[repr(transparent)]
pub struct TmdsPopDoubleL1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TmdsPopDoubleL1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TmdsPopDoubleL1 {
    // ======== Field Constants ========

    /// **TMDS_POP_DOUBLE_L1**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const TMDS_POP_DOUBLE_L1_OFFSET: u32 = 0;
    pub const TMDS_POP_DOUBLE_L1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **TMDS_POP_DOUBLE_L1** `[31:0]` (RO)
    #[inline(always)]
    pub fn tmds_pop_double_l1(&self) -> u32 {
        (self.value.read_bits() & Self::TMDS_POP_DOUBLE_L1_MASK) >> Self::TMDS_POP_DOUBLE_L1_OFFSET
    }

}

/// TMDS_POP_DOUBLE_L1 value type for batch read/modify/write operations
///
/// Use `tmdspopdoublel1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `tmdspopdoublel1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TmdsPopDoubleL1Val {
    bits: u32,
}

impl TmdsPopDoubleL1Val {
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

    /// Read **TMDS_POP_DOUBLE_L1** `[31:0]`
    #[inline(always)]
    pub const fn tmds_pop_double_l1(&self) -> u32 {
        (self.bits & TmdsPopDoubleL1::TMDS_POP_DOUBLE_L1_MASK) >> TmdsPopDoubleL1::TMDS_POP_DOUBLE_L1_OFFSET
    }

}

impl Default for TmdsPopDoubleL1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl TmdsPopDoubleL1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TmdsPopDoubleL1Val {
        TmdsPopDoubleL1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TmdsPopDoubleL1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TmdsPopDoubleL1Val) -> TmdsPopDoubleL1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// TMDS_PEEK_DOUBLE_L2 Register
///
/// Get lane 2 of the encoding of two pixels' worth of colour data. Two 10-bit TMDS symbols are packed at the bottom of a 32-bit word.
/// The PEEK alias does not shift the colour register when read, but still advances the lane 2 DC balance state. This is useful if all 3 lanes' worth of encode are to be read at once, rather than processing the entire scanline for one lane before moving to the next lane.
///
/// Address offset: 0x01e0
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **TMDS_PEEK_DOUBLE_L2** `[31:0]` (RO)
#[repr(transparent)]
pub struct TmdsPeekDoubleL2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TmdsPeekDoubleL2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TmdsPeekDoubleL2 {
    // ======== Field Constants ========

    /// **TMDS_PEEK_DOUBLE_L2**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const TMDS_PEEK_DOUBLE_L2_OFFSET: u32 = 0;
    pub const TMDS_PEEK_DOUBLE_L2_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **TMDS_PEEK_DOUBLE_L2** `[31:0]` (RO)
    #[inline(always)]
    pub fn tmds_peek_double_l2(&self) -> u32 {
        (self.value.read_bits() & Self::TMDS_PEEK_DOUBLE_L2_MASK) >> Self::TMDS_PEEK_DOUBLE_L2_OFFSET
    }

}

/// TMDS_PEEK_DOUBLE_L2 value type for batch read/modify/write operations
///
/// Use `tmdspeekdoublel2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `tmdspeekdoublel2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TmdsPeekDoubleL2Val {
    bits: u32,
}

impl TmdsPeekDoubleL2Val {
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

    /// Read **TMDS_PEEK_DOUBLE_L2** `[31:0]`
    #[inline(always)]
    pub const fn tmds_peek_double_l2(&self) -> u32 {
        (self.bits & TmdsPeekDoubleL2::TMDS_PEEK_DOUBLE_L2_MASK) >> TmdsPeekDoubleL2::TMDS_PEEK_DOUBLE_L2_OFFSET
    }

}

impl Default for TmdsPeekDoubleL2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl TmdsPeekDoubleL2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TmdsPeekDoubleL2Val {
        TmdsPeekDoubleL2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TmdsPeekDoubleL2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TmdsPeekDoubleL2Val) -> TmdsPeekDoubleL2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// TMDS_POP_DOUBLE_L2 Register
///
/// Get lane 2 of the encoding of two pixels' worth of colour data. Two 10-bit TMDS symbols are packed at the bottom of a 32-bit word.
/// The POP alias shifts the colour register when read, according to the values of PIX_SHIFT and PIX2_NOSHIFT.
///
/// Address offset: 0x01e4
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **TMDS_POP_DOUBLE_L2** `[31:0]` (RO)
#[repr(transparent)]
pub struct TmdsPopDoubleL2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TmdsPopDoubleL2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TmdsPopDoubleL2 {
    // ======== Field Constants ========

    /// **TMDS_POP_DOUBLE_L2**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const TMDS_POP_DOUBLE_L2_OFFSET: u32 = 0;
    pub const TMDS_POP_DOUBLE_L2_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **TMDS_POP_DOUBLE_L2** `[31:0]` (RO)
    #[inline(always)]
    pub fn tmds_pop_double_l2(&self) -> u32 {
        (self.value.read_bits() & Self::TMDS_POP_DOUBLE_L2_MASK) >> Self::TMDS_POP_DOUBLE_L2_OFFSET
    }

}

/// TMDS_POP_DOUBLE_L2 value type for batch read/modify/write operations
///
/// Use `tmdspopdoublel2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `tmdspopdoublel2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TmdsPopDoubleL2Val {
    bits: u32,
}

impl TmdsPopDoubleL2Val {
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

    /// Read **TMDS_POP_DOUBLE_L2** `[31:0]`
    #[inline(always)]
    pub const fn tmds_pop_double_l2(&self) -> u32 {
        (self.bits & TmdsPopDoubleL2::TMDS_POP_DOUBLE_L2_MASK) >> TmdsPopDoubleL2::TMDS_POP_DOUBLE_L2_OFFSET
    }

}

impl Default for TmdsPopDoubleL2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl TmdsPopDoubleL2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TmdsPopDoubleL2Val {
        TmdsPopDoubleL2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TmdsPopDoubleL2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TmdsPopDoubleL2Val) -> TmdsPopDoubleL2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// Processor core identifier
    pub cpuid: Cpuid,
    /// 0x0004
    /// Input value for GPIO0...31.
    /// In the Non-secure SIO, Secure-only GPIOs (as per ACCESSCTRL) appear as zero.
    pub gpio_in: GpioIn,
    /// 0x0008
    /// Input value on GPIO32...47, QSPI IOs and USB pins
    /// In the Non-secure SIO, Secure-only GPIOs (as per ACCESSCTRL) appear as zero.
    pub gpio_hi_in: GpioHiIn,
    _reserved0: [u8; 4],
    /// 0x0010
    /// GPIO0...31 output value
    pub gpio_out: GpioOut,
    /// 0x0014
    /// Output value for GPIO32...47, QSPI IOs and USB pins.
    /// Write to set output level (1/0 -> high/low). Reading back gives the last value written, NOT the input value from the pins. If core 0 and core 1 both write to GPIO_HI_OUT simultaneously (or to a SET/CLR/XOR alias), the result is as though the write from core 0 took place first, and the write from core 1 was then applied to that intermediate result.
    /// In the Non-secure SIO, Secure-only GPIOs (as per ACCESSCTRL) ignore writes, and their output status reads back as zero. This is also true for SET/CLR/XOR aliases of this register.
    pub gpio_hi_out: GpioHiOut,
    /// 0x0018
    /// GPIO0...31 output value set
    pub gpio_out_set: GpioOutSet,
    /// 0x001c
    /// Output value set for GPIO32..47, QSPI IOs and USB pins.
    /// Perform an atomic bit-set on GPIO_HI_OUT, i.e. `GPIO_HI_OUT |= wdata`
    pub gpio_hi_out_set: GpioHiOutSet,
    /// 0x0020
    /// GPIO0...31 output value clear
    pub gpio_out_clr: GpioOutClr,
    /// 0x0024
    /// Output value clear for GPIO32..47, QSPI IOs and USB pins.
    /// Perform an atomic bit-clear on GPIO_HI_OUT, i.e. `GPIO_HI_OUT &= ~wdata`
    pub gpio_hi_out_clr: GpioHiOutClr,
    /// 0x0028
    /// GPIO0...31 output value XOR
    pub gpio_out_xor: GpioOutXor,
    /// 0x002c
    /// Output value XOR for GPIO32..47, QSPI IOs and USB pins.
    /// Perform an atomic bitwise XOR on GPIO_HI_OUT, i.e. `GPIO_HI_OUT ^= wdata`
    pub gpio_hi_out_xor: GpioHiOutXor,
    /// 0x0030
    /// GPIO0...31 output enable
    pub gpio_oe: GpioOe,
    /// 0x0034
    /// Output enable value for GPIO32...47, QSPI IOs and USB pins.
    /// Write output enable (1/0 -> output/input). Reading back gives the last value written. If core 0 and core 1 both write to GPIO_HI_OE simultaneously (or to a SET/CLR/XOR alias), the result is as though the write from core 0 took place first, and the write from core 1 was then applied to that intermediate result.
    /// In the Non-secure SIO, Secure-only GPIOs (as per ACCESSCTRL) ignore writes, and their output status reads back as zero. This is also true for SET/CLR/XOR aliases of this register.
    pub gpio_hi_oe: GpioHiOe,
    /// 0x0038
    /// GPIO0...31 output enable set
    pub gpio_oe_set: GpioOeSet,
    /// 0x003c
    /// Output enable set for GPIO32...47, QSPI IOs and USB pins.
    /// Perform an atomic bit-set on GPIO_HI_OE, i.e. `GPIO_HI_OE |= wdata`
    pub gpio_hi_oe_set: GpioHiOeSet,
    /// 0x0040
    /// GPIO0...31 output enable clear
    pub gpio_oe_clr: GpioOeClr,
    /// 0x0044
    /// Output enable clear for GPIO32...47, QSPI IOs and USB pins.
    /// Perform an atomic bit-clear on GPIO_HI_OE, i.e. `GPIO_HI_OE &= ~wdata`
    pub gpio_hi_oe_clr: GpioHiOeClr,
    /// 0x0048
    /// GPIO0...31 output enable XOR
    pub gpio_oe_xor: GpioOeXor,
    /// 0x004c
    /// Output enable XOR for GPIO32...47, QSPI IOs and USB pins.
    /// Perform an atomic bitwise XOR on GPIO_HI_OE, i.e. `GPIO_HI_OE ^= wdata`
    pub gpio_hi_oe_xor: GpioHiOeXor,
    /// 0x0050
    /// Status register for inter-core FIFOs (mailboxes).
    /// There is one FIFO in the core 0 -> core 1 direction, and one core 1 -> core 0. Both are 32 bits wide and 8 words deep.
    /// Core 0 can see the read side of the 1->0 FIFO (RX), and the write side of 0->1 FIFO (TX).
    /// Core 1 can see the read side of the 0->1 FIFO (RX), and the write side of 1->0 FIFO (TX).
    /// The SIO IRQ for each core is the logical OR of the VLD, WOF and ROE fields of its FIFO_ST register.
    pub fifo_st: FifoSt,
    /// 0x0054
    /// Write access to this core's TX FIFO
    pub fifo_wr: FifoWr,
    /// 0x0058
    /// Read access to this core's RX FIFO
    pub fifo_rd: FifoRd,
    /// 0x005c
    /// Spinlock state
    /// A bitmap containing the state of all 32 spinlocks (1=locked).
    /// Mainly intended for debugging.
    pub spinlock_st: SpinlockSt,
    _reserved1: [u8; 32],
    /// 0x0080
    /// Read/write access to accumulator 0
    pub interp0_accum0: Interp0Accum0,
    /// 0x0084
    /// Read/write access to accumulator 1
    pub interp0_accum1: Interp0Accum1,
    /// 0x0088
    /// Read/write access to BASE0 register.
    pub interp0_base0: Interp0Base0,
    /// 0x008c
    /// Read/write access to BASE1 register.
    pub interp0_base1: Interp0Base1,
    /// 0x0090
    /// Read/write access to BASE2 register.
    pub interp0_base2: Interp0Base2,
    /// 0x0094
    /// Read LANE0 result, and simultaneously write lane results to both accumulators (POP).
    pub interp0_pop_lane0: Interp0PopLane0,
    /// 0x0098
    /// Read LANE1 result, and simultaneously write lane results to both accumulators (POP).
    pub interp0_pop_lane1: Interp0PopLane1,
    /// 0x009c
    /// Read FULL result, and simultaneously write lane results to both accumulators (POP).
    pub interp0_pop_full: Interp0PopFull,
    /// 0x00a0
    /// Read LANE0 result, without altering any internal state (PEEK).
    pub interp0_peek_lane0: Interp0PeekLane0,
    /// 0x00a4
    /// Read LANE1 result, without altering any internal state (PEEK).
    pub interp0_peek_lane1: Interp0PeekLane1,
    /// 0x00a8
    /// Read FULL result, without altering any internal state (PEEK).
    pub interp0_peek_full: Interp0PeekFull,
    /// 0x00ac
    /// Control register for lane 0
    pub interp0_ctrl_lane0: Interp0CtrlLane0,
    /// 0x00b0
    /// Control register for lane 1
    pub interp0_ctrl_lane1: Interp0CtrlLane1,
    /// 0x00b4
    /// Values written here are atomically added to ACCUM0
    /// Reading yields lane 0's raw shift and mask value (BASE0 not added).
    pub interp0_accum0_add: Interp0Accum0Add,
    /// 0x00b8
    /// Values written here are atomically added to ACCUM1
    /// Reading yields lane 1's raw shift and mask value (BASE1 not added).
    pub interp0_accum1_add: Interp0Accum1Add,
    /// 0x00bc
    /// On write, the lower 16 bits go to BASE0, upper bits to BASE1 simultaneously.
    /// Each half is sign-extended to 32 bits if that lane's SIGNED flag is set.
    pub interp0_base_1and0: Interp0Base1and0,
    /// 0x00c0
    /// Read/write access to accumulator 0
    pub interp1_accum0: Interp1Accum0,
    /// 0x00c4
    /// Read/write access to accumulator 1
    pub interp1_accum1: Interp1Accum1,
    /// 0x00c8
    /// Read/write access to BASE0 register.
    pub interp1_base0: Interp1Base0,
    /// 0x00cc
    /// Read/write access to BASE1 register.
    pub interp1_base1: Interp1Base1,
    /// 0x00d0
    /// Read/write access to BASE2 register.
    pub interp1_base2: Interp1Base2,
    /// 0x00d4
    /// Read LANE0 result, and simultaneously write lane results to both accumulators (POP).
    pub interp1_pop_lane0: Interp1PopLane0,
    /// 0x00d8
    /// Read LANE1 result, and simultaneously write lane results to both accumulators (POP).
    pub interp1_pop_lane1: Interp1PopLane1,
    /// 0x00dc
    /// Read FULL result, and simultaneously write lane results to both accumulators (POP).
    pub interp1_pop_full: Interp1PopFull,
    /// 0x00e0
    /// Read LANE0 result, without altering any internal state (PEEK).
    pub interp1_peek_lane0: Interp1PeekLane0,
    /// 0x00e4
    /// Read LANE1 result, without altering any internal state (PEEK).
    pub interp1_peek_lane1: Interp1PeekLane1,
    /// 0x00e8
    /// Read FULL result, without altering any internal state (PEEK).
    pub interp1_peek_full: Interp1PeekFull,
    /// 0x00ec
    /// Control register for lane 0
    pub interp1_ctrl_lane0: Interp1CtrlLane0,
    /// 0x00f0
    /// Control register for lane 1
    pub interp1_ctrl_lane1: Interp1CtrlLane1,
    /// 0x00f4
    /// Values written here are atomically added to ACCUM0
    /// Reading yields lane 0's raw shift and mask value (BASE0 not added).
    pub interp1_accum0_add: Interp1Accum0Add,
    /// 0x00f8
    /// Values written here are atomically added to ACCUM1
    /// Reading yields lane 1's raw shift and mask value (BASE1 not added).
    pub interp1_accum1_add: Interp1Accum1Add,
    /// 0x00fc
    /// On write, the lower 16 bits go to BASE0, upper bits to BASE1 simultaneously.
    /// Each half is sign-extended to 32 bits if that lane's SIGNED flag is set.
    pub interp1_base_1and0: Interp1Base1and0,
    /// 0x0100
    /// Reading from a spinlock address will:
    /// - Return 0 if lock is already locked
    /// - Otherwise return nonzero, and simultaneously claim the lock
    /// Writing (any value) releases the lock.
    /// If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.
    /// The value returned on success is 0x1 << lock number.
    pub spinlock0: Spinlock0,
    /// 0x0104
    /// Reading from a spinlock address will:
    /// - Return 0 if lock is already locked
    /// - Otherwise return nonzero, and simultaneously claim the lock
    /// Writing (any value) releases the lock.
    /// If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.
    /// The value returned on success is 0x1 << lock number.
    pub spinlock1: Spinlock1,
    /// 0x0108
    /// Reading from a spinlock address will:
    /// - Return 0 if lock is already locked
    /// - Otherwise return nonzero, and simultaneously claim the lock
    /// Writing (any value) releases the lock.
    /// If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.
    /// The value returned on success is 0x1 << lock number.
    pub spinlock2: Spinlock2,
    /// 0x010c
    /// Reading from a spinlock address will:
    /// - Return 0 if lock is already locked
    /// - Otherwise return nonzero, and simultaneously claim the lock
    /// Writing (any value) releases the lock.
    /// If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.
    /// The value returned on success is 0x1 << lock number.
    pub spinlock3: Spinlock3,
    /// 0x0110
    /// Reading from a spinlock address will:
    /// - Return 0 if lock is already locked
    /// - Otherwise return nonzero, and simultaneously claim the lock
    /// Writing (any value) releases the lock.
    /// If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.
    /// The value returned on success is 0x1 << lock number.
    pub spinlock4: Spinlock4,
    /// 0x0114
    /// Reading from a spinlock address will:
    /// - Return 0 if lock is already locked
    /// - Otherwise return nonzero, and simultaneously claim the lock
    /// Writing (any value) releases the lock.
    /// If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.
    /// The value returned on success is 0x1 << lock number.
    pub spinlock5: Spinlock5,
    /// 0x0118
    /// Reading from a spinlock address will:
    /// - Return 0 if lock is already locked
    /// - Otherwise return nonzero, and simultaneously claim the lock
    /// Writing (any value) releases the lock.
    /// If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.
    /// The value returned on success is 0x1 << lock number.
    pub spinlock6: Spinlock6,
    /// 0x011c
    /// Reading from a spinlock address will:
    /// - Return 0 if lock is already locked
    /// - Otherwise return nonzero, and simultaneously claim the lock
    /// Writing (any value) releases the lock.
    /// If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.
    /// The value returned on success is 0x1 << lock number.
    pub spinlock7: Spinlock7,
    /// 0x0120
    /// Reading from a spinlock address will:
    /// - Return 0 if lock is already locked
    /// - Otherwise return nonzero, and simultaneously claim the lock
    /// Writing (any value) releases the lock.
    /// If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.
    /// The value returned on success is 0x1 << lock number.
    pub spinlock8: Spinlock8,
    /// 0x0124
    /// Reading from a spinlock address will:
    /// - Return 0 if lock is already locked
    /// - Otherwise return nonzero, and simultaneously claim the lock
    /// Writing (any value) releases the lock.
    /// If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.
    /// The value returned on success is 0x1 << lock number.
    pub spinlock9: Spinlock9,
    /// 0x0128
    /// Reading from a spinlock address will:
    /// - Return 0 if lock is already locked
    /// - Otherwise return nonzero, and simultaneously claim the lock
    /// Writing (any value) releases the lock.
    /// If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.
    /// The value returned on success is 0x1 << lock number.
    pub spinlock10: Spinlock10,
    /// 0x012c
    /// Reading from a spinlock address will:
    /// - Return 0 if lock is already locked
    /// - Otherwise return nonzero, and simultaneously claim the lock
    /// Writing (any value) releases the lock.
    /// If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.
    /// The value returned on success is 0x1 << lock number.
    pub spinlock11: Spinlock11,
    /// 0x0130
    /// Reading from a spinlock address will:
    /// - Return 0 if lock is already locked
    /// - Otherwise return nonzero, and simultaneously claim the lock
    /// Writing (any value) releases the lock.
    /// If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.
    /// The value returned on success is 0x1 << lock number.
    pub spinlock12: Spinlock12,
    /// 0x0134
    /// Reading from a spinlock address will:
    /// - Return 0 if lock is already locked
    /// - Otherwise return nonzero, and simultaneously claim the lock
    /// Writing (any value) releases the lock.
    /// If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.
    /// The value returned on success is 0x1 << lock number.
    pub spinlock13: Spinlock13,
    /// 0x0138
    /// Reading from a spinlock address will:
    /// - Return 0 if lock is already locked
    /// - Otherwise return nonzero, and simultaneously claim the lock
    /// Writing (any value) releases the lock.
    /// If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.
    /// The value returned on success is 0x1 << lock number.
    pub spinlock14: Spinlock14,
    /// 0x013c
    /// Reading from a spinlock address will:
    /// - Return 0 if lock is already locked
    /// - Otherwise return nonzero, and simultaneously claim the lock
    /// Writing (any value) releases the lock.
    /// If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.
    /// The value returned on success is 0x1 << lock number.
    pub spinlock15: Spinlock15,
    /// 0x0140
    /// Reading from a spinlock address will:
    /// - Return 0 if lock is already locked
    /// - Otherwise return nonzero, and simultaneously claim the lock
    /// Writing (any value) releases the lock.
    /// If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.
    /// The value returned on success is 0x1 << lock number.
    pub spinlock16: Spinlock16,
    /// 0x0144
    /// Reading from a spinlock address will:
    /// - Return 0 if lock is already locked
    /// - Otherwise return nonzero, and simultaneously claim the lock
    /// Writing (any value) releases the lock.
    /// If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.
    /// The value returned on success is 0x1 << lock number.
    pub spinlock17: Spinlock17,
    /// 0x0148
    /// Reading from a spinlock address will:
    /// - Return 0 if lock is already locked
    /// - Otherwise return nonzero, and simultaneously claim the lock
    /// Writing (any value) releases the lock.
    /// If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.
    /// The value returned on success is 0x1 << lock number.
    pub spinlock18: Spinlock18,
    /// 0x014c
    /// Reading from a spinlock address will:
    /// - Return 0 if lock is already locked
    /// - Otherwise return nonzero, and simultaneously claim the lock
    /// Writing (any value) releases the lock.
    /// If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.
    /// The value returned on success is 0x1 << lock number.
    pub spinlock19: Spinlock19,
    /// 0x0150
    /// Reading from a spinlock address will:
    /// - Return 0 if lock is already locked
    /// - Otherwise return nonzero, and simultaneously claim the lock
    /// Writing (any value) releases the lock.
    /// If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.
    /// The value returned on success is 0x1 << lock number.
    pub spinlock20: Spinlock20,
    /// 0x0154
    /// Reading from a spinlock address will:
    /// - Return 0 if lock is already locked
    /// - Otherwise return nonzero, and simultaneously claim the lock
    /// Writing (any value) releases the lock.
    /// If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.
    /// The value returned on success is 0x1 << lock number.
    pub spinlock21: Spinlock21,
    /// 0x0158
    /// Reading from a spinlock address will:
    /// - Return 0 if lock is already locked
    /// - Otherwise return nonzero, and simultaneously claim the lock
    /// Writing (any value) releases the lock.
    /// If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.
    /// The value returned on success is 0x1 << lock number.
    pub spinlock22: Spinlock22,
    /// 0x015c
    /// Reading from a spinlock address will:
    /// - Return 0 if lock is already locked
    /// - Otherwise return nonzero, and simultaneously claim the lock
    /// Writing (any value) releases the lock.
    /// If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.
    /// The value returned on success is 0x1 << lock number.
    pub spinlock23: Spinlock23,
    /// 0x0160
    /// Reading from a spinlock address will:
    /// - Return 0 if lock is already locked
    /// - Otherwise return nonzero, and simultaneously claim the lock
    /// Writing (any value) releases the lock.
    /// If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.
    /// The value returned on success is 0x1 << lock number.
    pub spinlock24: Spinlock24,
    /// 0x0164
    /// Reading from a spinlock address will:
    /// - Return 0 if lock is already locked
    /// - Otherwise return nonzero, and simultaneously claim the lock
    /// Writing (any value) releases the lock.
    /// If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.
    /// The value returned on success is 0x1 << lock number.
    pub spinlock25: Spinlock25,
    /// 0x0168
    /// Reading from a spinlock address will:
    /// - Return 0 if lock is already locked
    /// - Otherwise return nonzero, and simultaneously claim the lock
    /// Writing (any value) releases the lock.
    /// If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.
    /// The value returned on success is 0x1 << lock number.
    pub spinlock26: Spinlock26,
    /// 0x016c
    /// Reading from a spinlock address will:
    /// - Return 0 if lock is already locked
    /// - Otherwise return nonzero, and simultaneously claim the lock
    /// Writing (any value) releases the lock.
    /// If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.
    /// The value returned on success is 0x1 << lock number.
    pub spinlock27: Spinlock27,
    /// 0x0170
    /// Reading from a spinlock address will:
    /// - Return 0 if lock is already locked
    /// - Otherwise return nonzero, and simultaneously claim the lock
    /// Writing (any value) releases the lock.
    /// If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.
    /// The value returned on success is 0x1 << lock number.
    pub spinlock28: Spinlock28,
    /// 0x0174
    /// Reading from a spinlock address will:
    /// - Return 0 if lock is already locked
    /// - Otherwise return nonzero, and simultaneously claim the lock
    /// Writing (any value) releases the lock.
    /// If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.
    /// The value returned on success is 0x1 << lock number.
    pub spinlock29: Spinlock29,
    /// 0x0178
    /// Reading from a spinlock address will:
    /// - Return 0 if lock is already locked
    /// - Otherwise return nonzero, and simultaneously claim the lock
    /// Writing (any value) releases the lock.
    /// If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.
    /// The value returned on success is 0x1 << lock number.
    pub spinlock30: Spinlock30,
    /// 0x017c
    /// Reading from a spinlock address will:
    /// - Return 0 if lock is already locked
    /// - Otherwise return nonzero, and simultaneously claim the lock
    /// Writing (any value) releases the lock.
    /// If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins.
    /// The value returned on success is 0x1 << lock number.
    pub spinlock31: Spinlock31,
    /// 0x0180
    /// Trigger a doorbell interrupt on the opposite core.
    /// Write 1 to a bit to set the corresponding bit in DOORBELL_IN on the opposite core. This raises the opposite core's doorbell interrupt.
    /// Read to get the status of the doorbells currently asserted on the opposite core. This is equivalent to that core reading its own DOORBELL_IN status.
    pub doorbell_out_set: DoorbellOutSet,
    /// 0x0184
    /// Clear doorbells which have been posted to the opposite core. This register is intended for debugging and initialisation purposes.
    /// Writing 1 to a bit in DOORBELL_OUT_CLR clears the corresponding bit in DOORBELL_IN on the opposite core. Clearing all bits will cause that core's doorbell interrupt to deassert. Since the usual order of events is for software to send events using DOORBELL_OUT_SET, and acknowledge incoming events by writing to DOORBELL_IN_CLR, this register should be used with caution to avoid race conditions.
    /// Reading returns the status of the doorbells currently asserted on the other core, i.e. is equivalent to that core reading its own DOORBELL_IN status.
    pub doorbell_out_clr: DoorbellOutClr,
    /// 0x0188
    /// Write 1s to trigger doorbell interrupts on this core. Read to get status of doorbells currently asserted on this core.
    pub doorbell_in_set: DoorbellInSet,
    /// 0x018c
    /// Check and acknowledge doorbells posted to this core. This core's doorbell interrupt is asserted when any bit in this register is 1.
    /// Write 1 to each bit to clear that bit. The doorbell interrupt deasserts once all bits are cleared. Read to get status of doorbells currently asserted on this core.
    pub doorbell_in_clr: DoorbellInClr,
    /// 0x0190
    /// Detach certain core-local peripherals from Secure SIO, and attach them to Non-secure SIO, so that Non-secure software can use them. Attempting to access one of these peripherals from the Secure SIO when it is attached to the Non-secure SIO, or vice versa, will generate a bus error.
    /// This register is per-core, and is only present on the Secure SIO.
    /// Most SIO hardware is duplicated across the Secure and Non-secure SIO, so is not listed in this register.
    pub peri_nonsec: PeriNonsec,
    _reserved2: [u8; 12],
    /// 0x01a0
    /// Control the assertion of the standard software interrupt (MIP.MSIP) on the RISC-V cores.
    /// Unlike the RISC-V timer, this interrupt is not routed to a normal system-level interrupt line, so can not be used by the Arm cores.
    /// It is safe for both cores to write to this register on the same cycle. The set/clear effect is accumulated across both cores, and then applied. If a flag is both set and cleared on the same cycle, only the set takes effect.
    pub riscv_softirq: RiscvSoftirq,
    /// 0x01a4
    /// Control register for the RISC-V 64-bit Machine-mode timer. This timer is only present in the Secure SIO, so is only accessible to an Arm core in Secure mode or a RISC-V core in Machine mode.
    /// Note whilst this timer follows the RISC-V privileged specification, it is equally usable by the Arm cores. The interrupts are routed to normal system-level interrupt lines as well as to the MIP.MTIP inputs on the RISC-V cores.
    pub mtime_ctrl: MtimeCtrl,
    _reserved3: [u8; 8],
    /// 0x01b0
    /// Read/write access to the high half of RISC-V Machine-mode timer. This register is shared between both cores. If both cores write on the same cycle, core 1 takes precedence.
    pub mtime: Mtime,
    /// 0x01b4
    /// Read/write access to the high half of RISC-V Machine-mode timer. This register is shared between both cores. If both cores write on the same cycle, core 1 takes precedence.
    pub mtimeh: Mtimeh,
    /// 0x01b8
    /// Low half of RISC-V Machine-mode timer comparator. This register is core-local, i.e., each core gets a copy of this register, with the comparison result routed to its own interrupt line.
    /// The timer interrupt is asserted whenever MTIME is greater than or equal to MTIMECMP. This comparison is unsigned, and performed on the full 64-bit values.
    pub mtimecmp: Mtimecmp,
    /// 0x01bc
    /// High half of RISC-V Machine-mode timer comparator. This register is core-local.
    /// The timer interrupt is asserted whenever MTIME is greater than or equal to MTIMECMP. This comparison is unsigned, and performed on the full 64-bit values.
    pub mtimecmph: Mtimecmph,
    /// 0x01c0
    /// Control register for TMDS encoder.
    pub tmds_ctrl: TmdsCtrl,
    /// 0x01c4
    /// Write-only access to the TMDS colour data register.
    pub tmds_wdata: TmdsWdata,
    /// 0x01c8
    /// Get the encoding of one pixel's worth of colour data, packed into a 32-bit value (3x10-bit symbols).
    /// The PEEK alias does not shift the colour register when read, but still advances the running DC balance state of each encoder. This is useful for pixel doubling.
    pub tmds_peek_single: TmdsPeekSingle,
    /// 0x01cc
    /// Get the encoding of one pixel's worth of colour data, packed into a 32-bit value. The packing is 5 chunks of 3 lanes times 2 bits (30 bits total). Each chunk contains two bits of a TMDS symbol per lane. This format is intended for shifting out with the HSTX peripheral on RP2350.
    /// The POP alias shifts the colour register when read, as well as advancing the running DC balance state of each encoder.
    pub tmds_pop_single: TmdsPopSingle,
    /// 0x01d0
    /// Get lane 0 of the encoding of two pixels' worth of colour data. Two 10-bit TMDS symbols are packed at the bottom of a 32-bit word.
    /// The PEEK alias does not shift the colour register when read, but still advances the lane 0 DC balance state. This is useful if all 3 lanes' worth of encode are to be read at once, rather than processing the entire scanline for one lane before moving to the next lane.
    pub tmds_peek_double_l0: TmdsPeekDoubleL0,
    /// 0x01d4
    /// Get lane 0 of the encoding of two pixels' worth of colour data. Two 10-bit TMDS symbols are packed at the bottom of a 32-bit word.
    /// The POP alias shifts the colour register when read, according to the values of PIX_SHIFT and PIX2_NOSHIFT.
    pub tmds_pop_double_l0: TmdsPopDoubleL0,
    /// 0x01d8
    /// Get lane 1 of the encoding of two pixels' worth of colour data. Two 10-bit TMDS symbols are packed at the bottom of a 32-bit word.
    /// The PEEK alias does not shift the colour register when read, but still advances the lane 1 DC balance state. This is useful if all 3 lanes' worth of encode are to be read at once, rather than processing the entire scanline for one lane before moving to the next lane.
    pub tmds_peek_double_l1: TmdsPeekDoubleL1,
    /// 0x01dc
    /// Get lane 1 of the encoding of two pixels' worth of colour data. Two 10-bit TMDS symbols are packed at the bottom of a 32-bit word.
    /// The POP alias shifts the colour register when read, according to the values of PIX_SHIFT and PIX2_NOSHIFT.
    pub tmds_pop_double_l1: TmdsPopDoubleL1,
    /// 0x01e0
    /// Get lane 2 of the encoding of two pixels' worth of colour data. Two 10-bit TMDS symbols are packed at the bottom of a 32-bit word.
    /// The PEEK alias does not shift the colour register when read, but still advances the lane 2 DC balance state. This is useful if all 3 lanes' worth of encode are to be read at once, rather than processing the entire scanline for one lane before moving to the next lane.
    pub tmds_peek_double_l2: TmdsPeekDoubleL2,
    /// 0x01e4
    /// Get lane 2 of the encoding of two pixels' worth of colour data. Two 10-bit TMDS symbols are packed at the bottom of a 32-bit word.
    /// The POP alias shifts the colour register when read, according to the values of PIX_SHIFT and PIX2_NOSHIFT.
    pub tmds_pop_double_l2: TmdsPopDoubleL2,
}

