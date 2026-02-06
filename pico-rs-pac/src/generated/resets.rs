//! RESETS
//!
//! This peripheral supports atomic SET/CLR/XOR register aliases.

#![allow(dead_code)]

pub const BASE: u32 = 0x40020000;

/// RESET Register
///
/// Address offset: 0x0000
/// Reset value: 0x1fffffff
///
/// ## Fields
///
/// - **USBCTRL** `[28]` (RW)
///
/// - **UART1** `[27]` (RW)
///
/// - **UART0** `[26]` (RW)
///
/// - **TRNG** `[25]` (RW)
///
/// - **TIMER1** `[24]` (RW)
///
/// - **TIMER0** `[23]` (RW)
///
/// - **TBMAN** `[22]` (RW)
///
/// - **SYSINFO** `[21]` (RW)
///
/// - **SYSCFG** `[20]` (RW)
///
/// - **SPI1** `[19]` (RW)
///
/// - **SPI0** `[18]` (RW)
///
/// - **SHA256** `[17]` (RW)
///
/// - **PWM** `[16]` (RW)
///
/// - **PLL_USB** `[15]` (RW)
///
/// - **PLL_SYS** `[14]` (RW)
///
/// - **PIO2** `[13]` (RW)
///
/// - **PIO1** `[12]` (RW)
///
/// - **PIO0** `[11]` (RW)
///
/// - **PADS_QSPI** `[10]` (RW)
///
/// - **PADS_BANK0** `[9]` (RW)
///
/// - **JTAG** `[8]` (RW)
///
/// - **IO_QSPI** `[7]` (RW)
///
/// - **IO_BANK0** `[6]` (RW)
///
/// - **I2C1** `[5]` (RW)
///
/// - **I2C0** `[4]` (RW)
///
/// - **HSTX** `[3]` (RW)
///
/// - **DMA** `[2]` (RW)
///
/// - **BUSCTRL** `[1]` (RW)
///
/// - **ADC** `[0]` (RW)
#[repr(transparent)]
pub struct Reset {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Reset {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Reset {
    // ======== Field Constants ========

    /// **USBCTRL**
    /// Bits `[28:28]`, mask `0x10000000`
    pub const USBCTRL_OFFSET: u32 = 28;
    pub const USBCTRL_MASK: u32 = 0x10000000;
    pub const USBCTRL: u32 = 1 << 28;

    /// **UART1**
    /// Bits `[27:27]`, mask `0x08000000`
    pub const UART1_OFFSET: u32 = 27;
    pub const UART1_MASK: u32 = 0x08000000;
    pub const UART1: u32 = 1 << 27;

    /// **UART0**
    /// Bits `[26:26]`, mask `0x04000000`
    pub const UART0_OFFSET: u32 = 26;
    pub const UART0_MASK: u32 = 0x04000000;
    pub const UART0: u32 = 1 << 26;

    /// **TRNG**
    /// Bits `[25:25]`, mask `0x02000000`
    pub const TRNG_OFFSET: u32 = 25;
    pub const TRNG_MASK: u32 = 0x02000000;
    pub const TRNG: u32 = 1 << 25;

    /// **TIMER1**
    /// Bits `[24:24]`, mask `0x01000000`
    pub const TIMER1_OFFSET: u32 = 24;
    pub const TIMER1_MASK: u32 = 0x01000000;
    pub const TIMER1: u32 = 1 << 24;

    /// **TIMER0**
    /// Bits `[23:23]`, mask `0x00800000`
    pub const TIMER0_OFFSET: u32 = 23;
    pub const TIMER0_MASK: u32 = 0x00800000;
    pub const TIMER0: u32 = 1 << 23;

    /// **TBMAN**
    /// Bits `[22:22]`, mask `0x00400000`
    pub const TBMAN_OFFSET: u32 = 22;
    pub const TBMAN_MASK: u32 = 0x00400000;
    pub const TBMAN: u32 = 1 << 22;

    /// **SYSINFO**
    /// Bits `[21:21]`, mask `0x00200000`
    pub const SYSINFO_OFFSET: u32 = 21;
    pub const SYSINFO_MASK: u32 = 0x00200000;
    pub const SYSINFO: u32 = 1 << 21;

    /// **SYSCFG**
    /// Bits `[20:20]`, mask `0x00100000`
    pub const SYSCFG_OFFSET: u32 = 20;
    pub const SYSCFG_MASK: u32 = 0x00100000;
    pub const SYSCFG: u32 = 1 << 20;

    /// **SPI1**
    /// Bits `[19:19]`, mask `0x00080000`
    pub const SPI1_OFFSET: u32 = 19;
    pub const SPI1_MASK: u32 = 0x00080000;
    pub const SPI1: u32 = 1 << 19;

    /// **SPI0**
    /// Bits `[18:18]`, mask `0x00040000`
    pub const SPI0_OFFSET: u32 = 18;
    pub const SPI0_MASK: u32 = 0x00040000;
    pub const SPI0: u32 = 1 << 18;

    /// **SHA256**
    /// Bits `[17:17]`, mask `0x00020000`
    pub const SHA256_OFFSET: u32 = 17;
    pub const SHA256_MASK: u32 = 0x00020000;
    pub const SHA256: u32 = 1 << 17;

    /// **PWM**
    /// Bits `[16:16]`, mask `0x00010000`
    pub const PWM_OFFSET: u32 = 16;
    pub const PWM_MASK: u32 = 0x00010000;
    pub const PWM: u32 = 1 << 16;

    /// **PLL_USB**
    /// Bits `[15:15]`, mask `0x00008000`
    pub const PLL_USB_OFFSET: u32 = 15;
    pub const PLL_USB_MASK: u32 = 0x00008000;
    pub const PLL_USB: u32 = 1 << 15;

    /// **PLL_SYS**
    /// Bits `[14:14]`, mask `0x00004000`
    pub const PLL_SYS_OFFSET: u32 = 14;
    pub const PLL_SYS_MASK: u32 = 0x00004000;
    pub const PLL_SYS: u32 = 1 << 14;

    /// **PIO2**
    /// Bits `[13:13]`, mask `0x00002000`
    pub const PIO2_OFFSET: u32 = 13;
    pub const PIO2_MASK: u32 = 0x00002000;
    pub const PIO2: u32 = 1 << 13;

    /// **PIO1**
    /// Bits `[12:12]`, mask `0x00001000`
    pub const PIO1_OFFSET: u32 = 12;
    pub const PIO1_MASK: u32 = 0x00001000;
    pub const PIO1: u32 = 1 << 12;

    /// **PIO0**
    /// Bits `[11:11]`, mask `0x00000800`
    pub const PIO0_OFFSET: u32 = 11;
    pub const PIO0_MASK: u32 = 0x00000800;
    pub const PIO0: u32 = 1 << 11;

    /// **PADS_QSPI**
    /// Bits `[10:10]`, mask `0x00000400`
    pub const PADS_QSPI_OFFSET: u32 = 10;
    pub const PADS_QSPI_MASK: u32 = 0x00000400;
    pub const PADS_QSPI: u32 = 1 << 10;

    /// **PADS_BANK0**
    /// Bits `[9:9]`, mask `0x00000200`
    pub const PADS_BANK0_OFFSET: u32 = 9;
    pub const PADS_BANK0_MASK: u32 = 0x00000200;
    pub const PADS_BANK0: u32 = 1 << 9;

    /// **JTAG**
    /// Bits `[8:8]`, mask `0x00000100`
    pub const JTAG_OFFSET: u32 = 8;
    pub const JTAG_MASK: u32 = 0x00000100;
    pub const JTAG: u32 = 1 << 8;

    /// **IO_QSPI**
    /// Bits `[7:7]`, mask `0x00000080`
    pub const IO_QSPI_OFFSET: u32 = 7;
    pub const IO_QSPI_MASK: u32 = 0x00000080;
    pub const IO_QSPI: u32 = 1 << 7;

    /// **IO_BANK0**
    /// Bits `[6:6]`, mask `0x00000040`
    pub const IO_BANK0_OFFSET: u32 = 6;
    pub const IO_BANK0_MASK: u32 = 0x00000040;
    pub const IO_BANK0: u32 = 1 << 6;

    /// **I2C1**
    /// Bits `[5:5]`, mask `0x00000020`
    pub const I2C1_OFFSET: u32 = 5;
    pub const I2C1_MASK: u32 = 0x00000020;
    pub const I2C1: u32 = 1 << 5;

    /// **I2C0**
    /// Bits `[4:4]`, mask `0x00000010`
    pub const I2C0_OFFSET: u32 = 4;
    pub const I2C0_MASK: u32 = 0x00000010;
    pub const I2C0: u32 = 1 << 4;

    /// **HSTX**
    /// Bits `[3:3]`, mask `0x00000008`
    pub const HSTX_OFFSET: u32 = 3;
    pub const HSTX_MASK: u32 = 0x00000008;
    pub const HSTX: u32 = 1 << 3;

    /// **DMA**
    /// Bits `[2:2]`, mask `0x00000004`
    pub const DMA_OFFSET: u32 = 2;
    pub const DMA_MASK: u32 = 0x00000004;
    pub const DMA: u32 = 1 << 2;

    /// **BUSCTRL**
    /// Bits `[1:1]`, mask `0x00000002`
    pub const BUSCTRL_OFFSET: u32 = 1;
    pub const BUSCTRL_MASK: u32 = 0x00000002;
    pub const BUSCTRL: u32 = 1 << 1;

    /// **ADC**
    /// Bits `[0:0]`, mask `0x00000001`
    pub const ADC_OFFSET: u32 = 0;
    pub const ADC_MASK: u32 = 0x00000001;
    pub const ADC: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **USBCTRL** `[28]` (RW)
    #[inline(always)]
    pub fn usbctrl(&self) -> bool {
        (self.value.read_bits() & Self::USBCTRL_MASK) != 0
    }

    /// Set USBCTRL bit atomically (RW)
    #[inline(always)]
    pub fn set_usbctrl(&self) {
        self.set_bits(Self::USBCTRL_MASK);
    }

    /// Clear USBCTRL bit atomically (RW)
    #[inline(always)]
    pub fn clr_usbctrl(&self) {
        self.clr_bits(Self::USBCTRL_MASK);
    }

    /// **UART1** `[27]` (RW)
    #[inline(always)]
    pub fn uart1(&self) -> bool {
        (self.value.read_bits() & Self::UART1_MASK) != 0
    }

    /// Set UART1 bit atomically (RW)
    #[inline(always)]
    pub fn set_uart1(&self) {
        self.set_bits(Self::UART1_MASK);
    }

    /// Clear UART1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_uart1(&self) {
        self.clr_bits(Self::UART1_MASK);
    }

    /// **UART0** `[26]` (RW)
    #[inline(always)]
    pub fn uart0(&self) -> bool {
        (self.value.read_bits() & Self::UART0_MASK) != 0
    }

    /// Set UART0 bit atomically (RW)
    #[inline(always)]
    pub fn set_uart0(&self) {
        self.set_bits(Self::UART0_MASK);
    }

    /// Clear UART0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_uart0(&self) {
        self.clr_bits(Self::UART0_MASK);
    }

    /// **TRNG** `[25]` (RW)
    #[inline(always)]
    pub fn trng(&self) -> bool {
        (self.value.read_bits() & Self::TRNG_MASK) != 0
    }

    /// Set TRNG bit atomically (RW)
    #[inline(always)]
    pub fn set_trng(&self) {
        self.set_bits(Self::TRNG_MASK);
    }

    /// Clear TRNG bit atomically (RW)
    #[inline(always)]
    pub fn clr_trng(&self) {
        self.clr_bits(Self::TRNG_MASK);
    }

    /// **TIMER1** `[24]` (RW)
    #[inline(always)]
    pub fn timer1(&self) -> bool {
        (self.value.read_bits() & Self::TIMER1_MASK) != 0
    }

    /// Set TIMER1 bit atomically (RW)
    #[inline(always)]
    pub fn set_timer1(&self) {
        self.set_bits(Self::TIMER1_MASK);
    }

    /// Clear TIMER1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_timer1(&self) {
        self.clr_bits(Self::TIMER1_MASK);
    }

    /// **TIMER0** `[23]` (RW)
    #[inline(always)]
    pub fn timer0(&self) -> bool {
        (self.value.read_bits() & Self::TIMER0_MASK) != 0
    }

    /// Set TIMER0 bit atomically (RW)
    #[inline(always)]
    pub fn set_timer0(&self) {
        self.set_bits(Self::TIMER0_MASK);
    }

    /// Clear TIMER0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_timer0(&self) {
        self.clr_bits(Self::TIMER0_MASK);
    }

    /// **TBMAN** `[22]` (RW)
    #[inline(always)]
    pub fn tbman(&self) -> bool {
        (self.value.read_bits() & Self::TBMAN_MASK) != 0
    }

    /// Set TBMAN bit atomically (RW)
    #[inline(always)]
    pub fn set_tbman(&self) {
        self.set_bits(Self::TBMAN_MASK);
    }

    /// Clear TBMAN bit atomically (RW)
    #[inline(always)]
    pub fn clr_tbman(&self) {
        self.clr_bits(Self::TBMAN_MASK);
    }

    /// **SYSINFO** `[21]` (RW)
    #[inline(always)]
    pub fn sysinfo(&self) -> bool {
        (self.value.read_bits() & Self::SYSINFO_MASK) != 0
    }

    /// Set SYSINFO bit atomically (RW)
    #[inline(always)]
    pub fn set_sysinfo(&self) {
        self.set_bits(Self::SYSINFO_MASK);
    }

    /// Clear SYSINFO bit atomically (RW)
    #[inline(always)]
    pub fn clr_sysinfo(&self) {
        self.clr_bits(Self::SYSINFO_MASK);
    }

    /// **SYSCFG** `[20]` (RW)
    #[inline(always)]
    pub fn syscfg(&self) -> bool {
        (self.value.read_bits() & Self::SYSCFG_MASK) != 0
    }

    /// Set SYSCFG bit atomically (RW)
    #[inline(always)]
    pub fn set_syscfg(&self) {
        self.set_bits(Self::SYSCFG_MASK);
    }

    /// Clear SYSCFG bit atomically (RW)
    #[inline(always)]
    pub fn clr_syscfg(&self) {
        self.clr_bits(Self::SYSCFG_MASK);
    }

    /// **SPI1** `[19]` (RW)
    #[inline(always)]
    pub fn spi1(&self) -> bool {
        (self.value.read_bits() & Self::SPI1_MASK) != 0
    }

    /// Set SPI1 bit atomically (RW)
    #[inline(always)]
    pub fn set_spi1(&self) {
        self.set_bits(Self::SPI1_MASK);
    }

    /// Clear SPI1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_spi1(&self) {
        self.clr_bits(Self::SPI1_MASK);
    }

    /// **SPI0** `[18]` (RW)
    #[inline(always)]
    pub fn spi0(&self) -> bool {
        (self.value.read_bits() & Self::SPI0_MASK) != 0
    }

    /// Set SPI0 bit atomically (RW)
    #[inline(always)]
    pub fn set_spi0(&self) {
        self.set_bits(Self::SPI0_MASK);
    }

    /// Clear SPI0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_spi0(&self) {
        self.clr_bits(Self::SPI0_MASK);
    }

    /// **SHA256** `[17]` (RW)
    #[inline(always)]
    pub fn sha256(&self) -> bool {
        (self.value.read_bits() & Self::SHA256_MASK) != 0
    }

    /// Set SHA256 bit atomically (RW)
    #[inline(always)]
    pub fn set_sha256(&self) {
        self.set_bits(Self::SHA256_MASK);
    }

    /// Clear SHA256 bit atomically (RW)
    #[inline(always)]
    pub fn clr_sha256(&self) {
        self.clr_bits(Self::SHA256_MASK);
    }

    /// **PWM** `[16]` (RW)
    #[inline(always)]
    pub fn pwm(&self) -> bool {
        (self.value.read_bits() & Self::PWM_MASK) != 0
    }

    /// Set PWM bit atomically (RW)
    #[inline(always)]
    pub fn set_pwm(&self) {
        self.set_bits(Self::PWM_MASK);
    }

    /// Clear PWM bit atomically (RW)
    #[inline(always)]
    pub fn clr_pwm(&self) {
        self.clr_bits(Self::PWM_MASK);
    }

    /// **PLL_USB** `[15]` (RW)
    #[inline(always)]
    pub fn pll_usb(&self) -> bool {
        (self.value.read_bits() & Self::PLL_USB_MASK) != 0
    }

    /// Set PLL_USB bit atomically (RW)
    #[inline(always)]
    pub fn set_pll_usb(&self) {
        self.set_bits(Self::PLL_USB_MASK);
    }

    /// Clear PLL_USB bit atomically (RW)
    #[inline(always)]
    pub fn clr_pll_usb(&self) {
        self.clr_bits(Self::PLL_USB_MASK);
    }

    /// **PLL_SYS** `[14]` (RW)
    #[inline(always)]
    pub fn pll_sys(&self) -> bool {
        (self.value.read_bits() & Self::PLL_SYS_MASK) != 0
    }

    /// Set PLL_SYS bit atomically (RW)
    #[inline(always)]
    pub fn set_pll_sys(&self) {
        self.set_bits(Self::PLL_SYS_MASK);
    }

    /// Clear PLL_SYS bit atomically (RW)
    #[inline(always)]
    pub fn clr_pll_sys(&self) {
        self.clr_bits(Self::PLL_SYS_MASK);
    }

    /// **PIO2** `[13]` (RW)
    #[inline(always)]
    pub fn pio2(&self) -> bool {
        (self.value.read_bits() & Self::PIO2_MASK) != 0
    }

    /// Set PIO2 bit atomically (RW)
    #[inline(always)]
    pub fn set_pio2(&self) {
        self.set_bits(Self::PIO2_MASK);
    }

    /// Clear PIO2 bit atomically (RW)
    #[inline(always)]
    pub fn clr_pio2(&self) {
        self.clr_bits(Self::PIO2_MASK);
    }

    /// **PIO1** `[12]` (RW)
    #[inline(always)]
    pub fn pio1(&self) -> bool {
        (self.value.read_bits() & Self::PIO1_MASK) != 0
    }

    /// Set PIO1 bit atomically (RW)
    #[inline(always)]
    pub fn set_pio1(&self) {
        self.set_bits(Self::PIO1_MASK);
    }

    /// Clear PIO1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_pio1(&self) {
        self.clr_bits(Self::PIO1_MASK);
    }

    /// **PIO0** `[11]` (RW)
    #[inline(always)]
    pub fn pio0(&self) -> bool {
        (self.value.read_bits() & Self::PIO0_MASK) != 0
    }

    /// Set PIO0 bit atomically (RW)
    #[inline(always)]
    pub fn set_pio0(&self) {
        self.set_bits(Self::PIO0_MASK);
    }

    /// Clear PIO0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_pio0(&self) {
        self.clr_bits(Self::PIO0_MASK);
    }

    /// **PADS_QSPI** `[10]` (RW)
    #[inline(always)]
    pub fn pads_qspi(&self) -> bool {
        (self.value.read_bits() & Self::PADS_QSPI_MASK) != 0
    }

    /// Set PADS_QSPI bit atomically (RW)
    #[inline(always)]
    pub fn set_pads_qspi(&self) {
        self.set_bits(Self::PADS_QSPI_MASK);
    }

    /// Clear PADS_QSPI bit atomically (RW)
    #[inline(always)]
    pub fn clr_pads_qspi(&self) {
        self.clr_bits(Self::PADS_QSPI_MASK);
    }

    /// **PADS_BANK0** `[9]` (RW)
    #[inline(always)]
    pub fn pads_bank0(&self) -> bool {
        (self.value.read_bits() & Self::PADS_BANK0_MASK) != 0
    }

    /// Set PADS_BANK0 bit atomically (RW)
    #[inline(always)]
    pub fn set_pads_bank0(&self) {
        self.set_bits(Self::PADS_BANK0_MASK);
    }

    /// Clear PADS_BANK0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_pads_bank0(&self) {
        self.clr_bits(Self::PADS_BANK0_MASK);
    }

    /// **JTAG** `[8]` (RW)
    #[inline(always)]
    pub fn jtag(&self) -> bool {
        (self.value.read_bits() & Self::JTAG_MASK) != 0
    }

    /// Set JTAG bit atomically (RW)
    #[inline(always)]
    pub fn set_jtag(&self) {
        self.set_bits(Self::JTAG_MASK);
    }

    /// Clear JTAG bit atomically (RW)
    #[inline(always)]
    pub fn clr_jtag(&self) {
        self.clr_bits(Self::JTAG_MASK);
    }

    /// **IO_QSPI** `[7]` (RW)
    #[inline(always)]
    pub fn io_qspi(&self) -> bool {
        (self.value.read_bits() & Self::IO_QSPI_MASK) != 0
    }

    /// Set IO_QSPI bit atomically (RW)
    #[inline(always)]
    pub fn set_io_qspi(&self) {
        self.set_bits(Self::IO_QSPI_MASK);
    }

    /// Clear IO_QSPI bit atomically (RW)
    #[inline(always)]
    pub fn clr_io_qspi(&self) {
        self.clr_bits(Self::IO_QSPI_MASK);
    }

    /// **IO_BANK0** `[6]` (RW)
    #[inline(always)]
    pub fn io_bank0(&self) -> bool {
        (self.value.read_bits() & Self::IO_BANK0_MASK) != 0
    }

    /// Set IO_BANK0 bit atomically (RW)
    #[inline(always)]
    pub fn set_io_bank0(&self) {
        self.set_bits(Self::IO_BANK0_MASK);
    }

    /// Clear IO_BANK0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_io_bank0(&self) {
        self.clr_bits(Self::IO_BANK0_MASK);
    }

    /// **I2C1** `[5]` (RW)
    #[inline(always)]
    pub fn i2c1(&self) -> bool {
        (self.value.read_bits() & Self::I2C1_MASK) != 0
    }

    /// Set I2C1 bit atomically (RW)
    #[inline(always)]
    pub fn set_i2c1(&self) {
        self.set_bits(Self::I2C1_MASK);
    }

    /// Clear I2C1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_i2c1(&self) {
        self.clr_bits(Self::I2C1_MASK);
    }

    /// **I2C0** `[4]` (RW)
    #[inline(always)]
    pub fn i2c0(&self) -> bool {
        (self.value.read_bits() & Self::I2C0_MASK) != 0
    }

    /// Set I2C0 bit atomically (RW)
    #[inline(always)]
    pub fn set_i2c0(&self) {
        self.set_bits(Self::I2C0_MASK);
    }

    /// Clear I2C0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_i2c0(&self) {
        self.clr_bits(Self::I2C0_MASK);
    }

    /// **HSTX** `[3]` (RW)
    #[inline(always)]
    pub fn hstx(&self) -> bool {
        (self.value.read_bits() & Self::HSTX_MASK) != 0
    }

    /// Set HSTX bit atomically (RW)
    #[inline(always)]
    pub fn set_hstx(&self) {
        self.set_bits(Self::HSTX_MASK);
    }

    /// Clear HSTX bit atomically (RW)
    #[inline(always)]
    pub fn clr_hstx(&self) {
        self.clr_bits(Self::HSTX_MASK);
    }

    /// **DMA** `[2]` (RW)
    #[inline(always)]
    pub fn dma(&self) -> bool {
        (self.value.read_bits() & Self::DMA_MASK) != 0
    }

    /// Set DMA bit atomically (RW)
    #[inline(always)]
    pub fn set_dma(&self) {
        self.set_bits(Self::DMA_MASK);
    }

    /// Clear DMA bit atomically (RW)
    #[inline(always)]
    pub fn clr_dma(&self) {
        self.clr_bits(Self::DMA_MASK);
    }

    /// **BUSCTRL** `[1]` (RW)
    #[inline(always)]
    pub fn busctrl(&self) -> bool {
        (self.value.read_bits() & Self::BUSCTRL_MASK) != 0
    }

    /// Set BUSCTRL bit atomically (RW)
    #[inline(always)]
    pub fn set_busctrl(&self) {
        self.set_bits(Self::BUSCTRL_MASK);
    }

    /// Clear BUSCTRL bit atomically (RW)
    #[inline(always)]
    pub fn clr_busctrl(&self) {
        self.clr_bits(Self::BUSCTRL_MASK);
    }

    /// **ADC** `[0]` (RW)
    #[inline(always)]
    pub fn adc(&self) -> bool {
        (self.value.read_bits() & Self::ADC_MASK) != 0
    }

    /// Set ADC bit atomically (RW)
    #[inline(always)]
    pub fn set_adc(&self) {
        self.set_bits(Self::ADC_MASK);
    }

    /// Clear ADC bit atomically (RW)
    #[inline(always)]
    pub fn clr_adc(&self) {
        self.clr_bits(Self::ADC_MASK);
    }
}

/// RESET value type for batch read/modify/write operations
///
/// Use `reset.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `reset.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ResetVal {
    bits: u32,
}

impl ResetVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x1fffffff }
    }

    /// Read **USBCTRL** `[28:28]`
    #[inline(always)]
    pub const fn usbctrl(&self) -> bool {
        (self.bits & Reset::USBCTRL_MASK) != 0
    }

    /// Read **UART1** `[27:27]`
    #[inline(always)]
    pub const fn uart1(&self) -> bool {
        (self.bits & Reset::UART1_MASK) != 0
    }

    /// Read **UART0** `[26:26]`
    #[inline(always)]
    pub const fn uart0(&self) -> bool {
        (self.bits & Reset::UART0_MASK) != 0
    }

    /// Read **TRNG** `[25:25]`
    #[inline(always)]
    pub const fn trng(&self) -> bool {
        (self.bits & Reset::TRNG_MASK) != 0
    }

    /// Read **TIMER1** `[24:24]`
    #[inline(always)]
    pub const fn timer1(&self) -> bool {
        (self.bits & Reset::TIMER1_MASK) != 0
    }

    /// Read **TIMER0** `[23:23]`
    #[inline(always)]
    pub const fn timer0(&self) -> bool {
        (self.bits & Reset::TIMER0_MASK) != 0
    }

    /// Read **TBMAN** `[22:22]`
    #[inline(always)]
    pub const fn tbman(&self) -> bool {
        (self.bits & Reset::TBMAN_MASK) != 0
    }

    /// Read **SYSINFO** `[21:21]`
    #[inline(always)]
    pub const fn sysinfo(&self) -> bool {
        (self.bits & Reset::SYSINFO_MASK) != 0
    }

    /// Read **SYSCFG** `[20:20]`
    #[inline(always)]
    pub const fn syscfg(&self) -> bool {
        (self.bits & Reset::SYSCFG_MASK) != 0
    }

    /// Read **SPI1** `[19:19]`
    #[inline(always)]
    pub const fn spi1(&self) -> bool {
        (self.bits & Reset::SPI1_MASK) != 0
    }

    /// Read **SPI0** `[18:18]`
    #[inline(always)]
    pub const fn spi0(&self) -> bool {
        (self.bits & Reset::SPI0_MASK) != 0
    }

    /// Read **SHA256** `[17:17]`
    #[inline(always)]
    pub const fn sha256(&self) -> bool {
        (self.bits & Reset::SHA256_MASK) != 0
    }

    /// Read **PWM** `[16:16]`
    #[inline(always)]
    pub const fn pwm(&self) -> bool {
        (self.bits & Reset::PWM_MASK) != 0
    }

    /// Read **PLL_USB** `[15:15]`
    #[inline(always)]
    pub const fn pll_usb(&self) -> bool {
        (self.bits & Reset::PLL_USB_MASK) != 0
    }

    /// Read **PLL_SYS** `[14:14]`
    #[inline(always)]
    pub const fn pll_sys(&self) -> bool {
        (self.bits & Reset::PLL_SYS_MASK) != 0
    }

    /// Read **PIO2** `[13:13]`
    #[inline(always)]
    pub const fn pio2(&self) -> bool {
        (self.bits & Reset::PIO2_MASK) != 0
    }

    /// Read **PIO1** `[12:12]`
    #[inline(always)]
    pub const fn pio1(&self) -> bool {
        (self.bits & Reset::PIO1_MASK) != 0
    }

    /// Read **PIO0** `[11:11]`
    #[inline(always)]
    pub const fn pio0(&self) -> bool {
        (self.bits & Reset::PIO0_MASK) != 0
    }

    /// Read **PADS_QSPI** `[10:10]`
    #[inline(always)]
    pub const fn pads_qspi(&self) -> bool {
        (self.bits & Reset::PADS_QSPI_MASK) != 0
    }

    /// Read **PADS_BANK0** `[9:9]`
    #[inline(always)]
    pub const fn pads_bank0(&self) -> bool {
        (self.bits & Reset::PADS_BANK0_MASK) != 0
    }

    /// Read **JTAG** `[8:8]`
    #[inline(always)]
    pub const fn jtag(&self) -> bool {
        (self.bits & Reset::JTAG_MASK) != 0
    }

    /// Read **IO_QSPI** `[7:7]`
    #[inline(always)]
    pub const fn io_qspi(&self) -> bool {
        (self.bits & Reset::IO_QSPI_MASK) != 0
    }

    /// Read **IO_BANK0** `[6:6]`
    #[inline(always)]
    pub const fn io_bank0(&self) -> bool {
        (self.bits & Reset::IO_BANK0_MASK) != 0
    }

    /// Read **I2C1** `[5:5]`
    #[inline(always)]
    pub const fn i2c1(&self) -> bool {
        (self.bits & Reset::I2C1_MASK) != 0
    }

    /// Read **I2C0** `[4:4]`
    #[inline(always)]
    pub const fn i2c0(&self) -> bool {
        (self.bits & Reset::I2C0_MASK) != 0
    }

    /// Read **HSTX** `[3:3]`
    #[inline(always)]
    pub const fn hstx(&self) -> bool {
        (self.bits & Reset::HSTX_MASK) != 0
    }

    /// Read **DMA** `[2:2]`
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        (self.bits & Reset::DMA_MASK) != 0
    }

    /// Read **BUSCTRL** `[1:1]`
    #[inline(always)]
    pub const fn busctrl(&self) -> bool {
        (self.bits & Reset::BUSCTRL_MASK) != 0
    }

    /// Read **ADC** `[0:0]`
    #[inline(always)]
    pub const fn adc(&self) -> bool {
        (self.bits & Reset::ADC_MASK) != 0
    }

    /// Set **USBCTRL** `[28:28]`
    #[inline(always)]
    pub const fn with_usbctrl(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Reset::USBCTRL_MASK
            } else {
                self.bits & !Reset::USBCTRL_MASK
            }
        }
    }

    /// Set **UART1** `[27:27]`
    #[inline(always)]
    pub const fn with_uart1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Reset::UART1_MASK
            } else {
                self.bits & !Reset::UART1_MASK
            }
        }
    }

    /// Set **UART0** `[26:26]`
    #[inline(always)]
    pub const fn with_uart0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Reset::UART0_MASK
            } else {
                self.bits & !Reset::UART0_MASK
            }
        }
    }

    /// Set **TRNG** `[25:25]`
    #[inline(always)]
    pub const fn with_trng(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Reset::TRNG_MASK
            } else {
                self.bits & !Reset::TRNG_MASK
            }
        }
    }

    /// Set **TIMER1** `[24:24]`
    #[inline(always)]
    pub const fn with_timer1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Reset::TIMER1_MASK
            } else {
                self.bits & !Reset::TIMER1_MASK
            }
        }
    }

    /// Set **TIMER0** `[23:23]`
    #[inline(always)]
    pub const fn with_timer0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Reset::TIMER0_MASK
            } else {
                self.bits & !Reset::TIMER0_MASK
            }
        }
    }

    /// Set **TBMAN** `[22:22]`
    #[inline(always)]
    pub const fn with_tbman(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Reset::TBMAN_MASK
            } else {
                self.bits & !Reset::TBMAN_MASK
            }
        }
    }

    /// Set **SYSINFO** `[21:21]`
    #[inline(always)]
    pub const fn with_sysinfo(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Reset::SYSINFO_MASK
            } else {
                self.bits & !Reset::SYSINFO_MASK
            }
        }
    }

    /// Set **SYSCFG** `[20:20]`
    #[inline(always)]
    pub const fn with_syscfg(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Reset::SYSCFG_MASK
            } else {
                self.bits & !Reset::SYSCFG_MASK
            }
        }
    }

    /// Set **SPI1** `[19:19]`
    #[inline(always)]
    pub const fn with_spi1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Reset::SPI1_MASK
            } else {
                self.bits & !Reset::SPI1_MASK
            }
        }
    }

    /// Set **SPI0** `[18:18]`
    #[inline(always)]
    pub const fn with_spi0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Reset::SPI0_MASK
            } else {
                self.bits & !Reset::SPI0_MASK
            }
        }
    }

    /// Set **SHA256** `[17:17]`
    #[inline(always)]
    pub const fn with_sha256(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Reset::SHA256_MASK
            } else {
                self.bits & !Reset::SHA256_MASK
            }
        }
    }

    /// Set **PWM** `[16:16]`
    #[inline(always)]
    pub const fn with_pwm(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Reset::PWM_MASK
            } else {
                self.bits & !Reset::PWM_MASK
            }
        }
    }

    /// Set **PLL_USB** `[15:15]`
    #[inline(always)]
    pub const fn with_pll_usb(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Reset::PLL_USB_MASK
            } else {
                self.bits & !Reset::PLL_USB_MASK
            }
        }
    }

    /// Set **PLL_SYS** `[14:14]`
    #[inline(always)]
    pub const fn with_pll_sys(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Reset::PLL_SYS_MASK
            } else {
                self.bits & !Reset::PLL_SYS_MASK
            }
        }
    }

    /// Set **PIO2** `[13:13]`
    #[inline(always)]
    pub const fn with_pio2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Reset::PIO2_MASK
            } else {
                self.bits & !Reset::PIO2_MASK
            }
        }
    }

    /// Set **PIO1** `[12:12]`
    #[inline(always)]
    pub const fn with_pio1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Reset::PIO1_MASK
            } else {
                self.bits & !Reset::PIO1_MASK
            }
        }
    }

    /// Set **PIO0** `[11:11]`
    #[inline(always)]
    pub const fn with_pio0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Reset::PIO0_MASK
            } else {
                self.bits & !Reset::PIO0_MASK
            }
        }
    }

    /// Set **PADS_QSPI** `[10:10]`
    #[inline(always)]
    pub const fn with_pads_qspi(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Reset::PADS_QSPI_MASK
            } else {
                self.bits & !Reset::PADS_QSPI_MASK
            }
        }
    }

    /// Set **PADS_BANK0** `[9:9]`
    #[inline(always)]
    pub const fn with_pads_bank0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Reset::PADS_BANK0_MASK
            } else {
                self.bits & !Reset::PADS_BANK0_MASK
            }
        }
    }

    /// Set **JTAG** `[8:8]`
    #[inline(always)]
    pub const fn with_jtag(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Reset::JTAG_MASK
            } else {
                self.bits & !Reset::JTAG_MASK
            }
        }
    }

    /// Set **IO_QSPI** `[7:7]`
    #[inline(always)]
    pub const fn with_io_qspi(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Reset::IO_QSPI_MASK
            } else {
                self.bits & !Reset::IO_QSPI_MASK
            }
        }
    }

    /// Set **IO_BANK0** `[6:6]`
    #[inline(always)]
    pub const fn with_io_bank0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Reset::IO_BANK0_MASK
            } else {
                self.bits & !Reset::IO_BANK0_MASK
            }
        }
    }

    /// Set **I2C1** `[5:5]`
    #[inline(always)]
    pub const fn with_i2c1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Reset::I2C1_MASK
            } else {
                self.bits & !Reset::I2C1_MASK
            }
        }
    }

    /// Set **I2C0** `[4:4]`
    #[inline(always)]
    pub const fn with_i2c0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Reset::I2C0_MASK
            } else {
                self.bits & !Reset::I2C0_MASK
            }
        }
    }

    /// Set **HSTX** `[3:3]`
    #[inline(always)]
    pub const fn with_hstx(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Reset::HSTX_MASK
            } else {
                self.bits & !Reset::HSTX_MASK
            }
        }
    }

    /// Set **DMA** `[2:2]`
    #[inline(always)]
    pub const fn with_dma(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Reset::DMA_MASK
            } else {
                self.bits & !Reset::DMA_MASK
            }
        }
    }

    /// Set **BUSCTRL** `[1:1]`
    #[inline(always)]
    pub const fn with_busctrl(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Reset::BUSCTRL_MASK
            } else {
                self.bits & !Reset::BUSCTRL_MASK
            }
        }
    }

    /// Set **ADC** `[0:0]`
    #[inline(always)]
    pub const fn with_adc(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Reset::ADC_MASK
            } else {
                self.bits & !Reset::ADC_MASK
            }
        }
    }
}

impl Default for ResetVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Reset {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ResetVal {
        ResetVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ResetVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ResetVal) -> ResetVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// WDSEL Register
///
/// Address offset: 0x0004
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **USBCTRL** `[28]` (RW)
///
/// - **UART1** `[27]` (RW)
///
/// - **UART0** `[26]` (RW)
///
/// - **TRNG** `[25]` (RW)
///
/// - **TIMER1** `[24]` (RW)
///
/// - **TIMER0** `[23]` (RW)
///
/// - **TBMAN** `[22]` (RW)
///
/// - **SYSINFO** `[21]` (RW)
///
/// - **SYSCFG** `[20]` (RW)
///
/// - **SPI1** `[19]` (RW)
///
/// - **SPI0** `[18]` (RW)
///
/// - **SHA256** `[17]` (RW)
///
/// - **PWM** `[16]` (RW)
///
/// - **PLL_USB** `[15]` (RW)
///
/// - **PLL_SYS** `[14]` (RW)
///
/// - **PIO2** `[13]` (RW)
///
/// - **PIO1** `[12]` (RW)
///
/// - **PIO0** `[11]` (RW)
///
/// - **PADS_QSPI** `[10]` (RW)
///
/// - **PADS_BANK0** `[9]` (RW)
///
/// - **JTAG** `[8]` (RW)
///
/// - **IO_QSPI** `[7]` (RW)
///
/// - **IO_BANK0** `[6]` (RW)
///
/// - **I2C1** `[5]` (RW)
///
/// - **I2C0** `[4]` (RW)
///
/// - **HSTX** `[3]` (RW)
///
/// - **DMA** `[2]` (RW)
///
/// - **BUSCTRL** `[1]` (RW)
///
/// - **ADC** `[0]` (RW)
#[repr(transparent)]
pub struct Wdsel {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Wdsel {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Wdsel {
    // ======== Field Constants ========

    /// **USBCTRL**
    /// Bits `[28:28]`, mask `0x10000000`
    pub const USBCTRL_OFFSET: u32 = 28;
    pub const USBCTRL_MASK: u32 = 0x10000000;
    pub const USBCTRL: u32 = 1 << 28;

    /// **UART1**
    /// Bits `[27:27]`, mask `0x08000000`
    pub const UART1_OFFSET: u32 = 27;
    pub const UART1_MASK: u32 = 0x08000000;
    pub const UART1: u32 = 1 << 27;

    /// **UART0**
    /// Bits `[26:26]`, mask `0x04000000`
    pub const UART0_OFFSET: u32 = 26;
    pub const UART0_MASK: u32 = 0x04000000;
    pub const UART0: u32 = 1 << 26;

    /// **TRNG**
    /// Bits `[25:25]`, mask `0x02000000`
    pub const TRNG_OFFSET: u32 = 25;
    pub const TRNG_MASK: u32 = 0x02000000;
    pub const TRNG: u32 = 1 << 25;

    /// **TIMER1**
    /// Bits `[24:24]`, mask `0x01000000`
    pub const TIMER1_OFFSET: u32 = 24;
    pub const TIMER1_MASK: u32 = 0x01000000;
    pub const TIMER1: u32 = 1 << 24;

    /// **TIMER0**
    /// Bits `[23:23]`, mask `0x00800000`
    pub const TIMER0_OFFSET: u32 = 23;
    pub const TIMER0_MASK: u32 = 0x00800000;
    pub const TIMER0: u32 = 1 << 23;

    /// **TBMAN**
    /// Bits `[22:22]`, mask `0x00400000`
    pub const TBMAN_OFFSET: u32 = 22;
    pub const TBMAN_MASK: u32 = 0x00400000;
    pub const TBMAN: u32 = 1 << 22;

    /// **SYSINFO**
    /// Bits `[21:21]`, mask `0x00200000`
    pub const SYSINFO_OFFSET: u32 = 21;
    pub const SYSINFO_MASK: u32 = 0x00200000;
    pub const SYSINFO: u32 = 1 << 21;

    /// **SYSCFG**
    /// Bits `[20:20]`, mask `0x00100000`
    pub const SYSCFG_OFFSET: u32 = 20;
    pub const SYSCFG_MASK: u32 = 0x00100000;
    pub const SYSCFG: u32 = 1 << 20;

    /// **SPI1**
    /// Bits `[19:19]`, mask `0x00080000`
    pub const SPI1_OFFSET: u32 = 19;
    pub const SPI1_MASK: u32 = 0x00080000;
    pub const SPI1: u32 = 1 << 19;

    /// **SPI0**
    /// Bits `[18:18]`, mask `0x00040000`
    pub const SPI0_OFFSET: u32 = 18;
    pub const SPI0_MASK: u32 = 0x00040000;
    pub const SPI0: u32 = 1 << 18;

    /// **SHA256**
    /// Bits `[17:17]`, mask `0x00020000`
    pub const SHA256_OFFSET: u32 = 17;
    pub const SHA256_MASK: u32 = 0x00020000;
    pub const SHA256: u32 = 1 << 17;

    /// **PWM**
    /// Bits `[16:16]`, mask `0x00010000`
    pub const PWM_OFFSET: u32 = 16;
    pub const PWM_MASK: u32 = 0x00010000;
    pub const PWM: u32 = 1 << 16;

    /// **PLL_USB**
    /// Bits `[15:15]`, mask `0x00008000`
    pub const PLL_USB_OFFSET: u32 = 15;
    pub const PLL_USB_MASK: u32 = 0x00008000;
    pub const PLL_USB: u32 = 1 << 15;

    /// **PLL_SYS**
    /// Bits `[14:14]`, mask `0x00004000`
    pub const PLL_SYS_OFFSET: u32 = 14;
    pub const PLL_SYS_MASK: u32 = 0x00004000;
    pub const PLL_SYS: u32 = 1 << 14;

    /// **PIO2**
    /// Bits `[13:13]`, mask `0x00002000`
    pub const PIO2_OFFSET: u32 = 13;
    pub const PIO2_MASK: u32 = 0x00002000;
    pub const PIO2: u32 = 1 << 13;

    /// **PIO1**
    /// Bits `[12:12]`, mask `0x00001000`
    pub const PIO1_OFFSET: u32 = 12;
    pub const PIO1_MASK: u32 = 0x00001000;
    pub const PIO1: u32 = 1 << 12;

    /// **PIO0**
    /// Bits `[11:11]`, mask `0x00000800`
    pub const PIO0_OFFSET: u32 = 11;
    pub const PIO0_MASK: u32 = 0x00000800;
    pub const PIO0: u32 = 1 << 11;

    /// **PADS_QSPI**
    /// Bits `[10:10]`, mask `0x00000400`
    pub const PADS_QSPI_OFFSET: u32 = 10;
    pub const PADS_QSPI_MASK: u32 = 0x00000400;
    pub const PADS_QSPI: u32 = 1 << 10;

    /// **PADS_BANK0**
    /// Bits `[9:9]`, mask `0x00000200`
    pub const PADS_BANK0_OFFSET: u32 = 9;
    pub const PADS_BANK0_MASK: u32 = 0x00000200;
    pub const PADS_BANK0: u32 = 1 << 9;

    /// **JTAG**
    /// Bits `[8:8]`, mask `0x00000100`
    pub const JTAG_OFFSET: u32 = 8;
    pub const JTAG_MASK: u32 = 0x00000100;
    pub const JTAG: u32 = 1 << 8;

    /// **IO_QSPI**
    /// Bits `[7:7]`, mask `0x00000080`
    pub const IO_QSPI_OFFSET: u32 = 7;
    pub const IO_QSPI_MASK: u32 = 0x00000080;
    pub const IO_QSPI: u32 = 1 << 7;

    /// **IO_BANK0**
    /// Bits `[6:6]`, mask `0x00000040`
    pub const IO_BANK0_OFFSET: u32 = 6;
    pub const IO_BANK0_MASK: u32 = 0x00000040;
    pub const IO_BANK0: u32 = 1 << 6;

    /// **I2C1**
    /// Bits `[5:5]`, mask `0x00000020`
    pub const I2C1_OFFSET: u32 = 5;
    pub const I2C1_MASK: u32 = 0x00000020;
    pub const I2C1: u32 = 1 << 5;

    /// **I2C0**
    /// Bits `[4:4]`, mask `0x00000010`
    pub const I2C0_OFFSET: u32 = 4;
    pub const I2C0_MASK: u32 = 0x00000010;
    pub const I2C0: u32 = 1 << 4;

    /// **HSTX**
    /// Bits `[3:3]`, mask `0x00000008`
    pub const HSTX_OFFSET: u32 = 3;
    pub const HSTX_MASK: u32 = 0x00000008;
    pub const HSTX: u32 = 1 << 3;

    /// **DMA**
    /// Bits `[2:2]`, mask `0x00000004`
    pub const DMA_OFFSET: u32 = 2;
    pub const DMA_MASK: u32 = 0x00000004;
    pub const DMA: u32 = 1 << 2;

    /// **BUSCTRL**
    /// Bits `[1:1]`, mask `0x00000002`
    pub const BUSCTRL_OFFSET: u32 = 1;
    pub const BUSCTRL_MASK: u32 = 0x00000002;
    pub const BUSCTRL: u32 = 1 << 1;

    /// **ADC**
    /// Bits `[0:0]`, mask `0x00000001`
    pub const ADC_OFFSET: u32 = 0;
    pub const ADC_MASK: u32 = 0x00000001;
    pub const ADC: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **USBCTRL** `[28]` (RW)
    #[inline(always)]
    pub fn usbctrl(&self) -> bool {
        (self.value.read_bits() & Self::USBCTRL_MASK) != 0
    }

    /// Set USBCTRL bit atomically (RW)
    #[inline(always)]
    pub fn set_usbctrl(&self) {
        self.set_bits(Self::USBCTRL_MASK);
    }

    /// Clear USBCTRL bit atomically (RW)
    #[inline(always)]
    pub fn clr_usbctrl(&self) {
        self.clr_bits(Self::USBCTRL_MASK);
    }

    /// **UART1** `[27]` (RW)
    #[inline(always)]
    pub fn uart1(&self) -> bool {
        (self.value.read_bits() & Self::UART1_MASK) != 0
    }

    /// Set UART1 bit atomically (RW)
    #[inline(always)]
    pub fn set_uart1(&self) {
        self.set_bits(Self::UART1_MASK);
    }

    /// Clear UART1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_uart1(&self) {
        self.clr_bits(Self::UART1_MASK);
    }

    /// **UART0** `[26]` (RW)
    #[inline(always)]
    pub fn uart0(&self) -> bool {
        (self.value.read_bits() & Self::UART0_MASK) != 0
    }

    /// Set UART0 bit atomically (RW)
    #[inline(always)]
    pub fn set_uart0(&self) {
        self.set_bits(Self::UART0_MASK);
    }

    /// Clear UART0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_uart0(&self) {
        self.clr_bits(Self::UART0_MASK);
    }

    /// **TRNG** `[25]` (RW)
    #[inline(always)]
    pub fn trng(&self) -> bool {
        (self.value.read_bits() & Self::TRNG_MASK) != 0
    }

    /// Set TRNG bit atomically (RW)
    #[inline(always)]
    pub fn set_trng(&self) {
        self.set_bits(Self::TRNG_MASK);
    }

    /// Clear TRNG bit atomically (RW)
    #[inline(always)]
    pub fn clr_trng(&self) {
        self.clr_bits(Self::TRNG_MASK);
    }

    /// **TIMER1** `[24]` (RW)
    #[inline(always)]
    pub fn timer1(&self) -> bool {
        (self.value.read_bits() & Self::TIMER1_MASK) != 0
    }

    /// Set TIMER1 bit atomically (RW)
    #[inline(always)]
    pub fn set_timer1(&self) {
        self.set_bits(Self::TIMER1_MASK);
    }

    /// Clear TIMER1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_timer1(&self) {
        self.clr_bits(Self::TIMER1_MASK);
    }

    /// **TIMER0** `[23]` (RW)
    #[inline(always)]
    pub fn timer0(&self) -> bool {
        (self.value.read_bits() & Self::TIMER0_MASK) != 0
    }

    /// Set TIMER0 bit atomically (RW)
    #[inline(always)]
    pub fn set_timer0(&self) {
        self.set_bits(Self::TIMER0_MASK);
    }

    /// Clear TIMER0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_timer0(&self) {
        self.clr_bits(Self::TIMER0_MASK);
    }

    /// **TBMAN** `[22]` (RW)
    #[inline(always)]
    pub fn tbman(&self) -> bool {
        (self.value.read_bits() & Self::TBMAN_MASK) != 0
    }

    /// Set TBMAN bit atomically (RW)
    #[inline(always)]
    pub fn set_tbman(&self) {
        self.set_bits(Self::TBMAN_MASK);
    }

    /// Clear TBMAN bit atomically (RW)
    #[inline(always)]
    pub fn clr_tbman(&self) {
        self.clr_bits(Self::TBMAN_MASK);
    }

    /// **SYSINFO** `[21]` (RW)
    #[inline(always)]
    pub fn sysinfo(&self) -> bool {
        (self.value.read_bits() & Self::SYSINFO_MASK) != 0
    }

    /// Set SYSINFO bit atomically (RW)
    #[inline(always)]
    pub fn set_sysinfo(&self) {
        self.set_bits(Self::SYSINFO_MASK);
    }

    /// Clear SYSINFO bit atomically (RW)
    #[inline(always)]
    pub fn clr_sysinfo(&self) {
        self.clr_bits(Self::SYSINFO_MASK);
    }

    /// **SYSCFG** `[20]` (RW)
    #[inline(always)]
    pub fn syscfg(&self) -> bool {
        (self.value.read_bits() & Self::SYSCFG_MASK) != 0
    }

    /// Set SYSCFG bit atomically (RW)
    #[inline(always)]
    pub fn set_syscfg(&self) {
        self.set_bits(Self::SYSCFG_MASK);
    }

    /// Clear SYSCFG bit atomically (RW)
    #[inline(always)]
    pub fn clr_syscfg(&self) {
        self.clr_bits(Self::SYSCFG_MASK);
    }

    /// **SPI1** `[19]` (RW)
    #[inline(always)]
    pub fn spi1(&self) -> bool {
        (self.value.read_bits() & Self::SPI1_MASK) != 0
    }

    /// Set SPI1 bit atomically (RW)
    #[inline(always)]
    pub fn set_spi1(&self) {
        self.set_bits(Self::SPI1_MASK);
    }

    /// Clear SPI1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_spi1(&self) {
        self.clr_bits(Self::SPI1_MASK);
    }

    /// **SPI0** `[18]` (RW)
    #[inline(always)]
    pub fn spi0(&self) -> bool {
        (self.value.read_bits() & Self::SPI0_MASK) != 0
    }

    /// Set SPI0 bit atomically (RW)
    #[inline(always)]
    pub fn set_spi0(&self) {
        self.set_bits(Self::SPI0_MASK);
    }

    /// Clear SPI0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_spi0(&self) {
        self.clr_bits(Self::SPI0_MASK);
    }

    /// **SHA256** `[17]` (RW)
    #[inline(always)]
    pub fn sha256(&self) -> bool {
        (self.value.read_bits() & Self::SHA256_MASK) != 0
    }

    /// Set SHA256 bit atomically (RW)
    #[inline(always)]
    pub fn set_sha256(&self) {
        self.set_bits(Self::SHA256_MASK);
    }

    /// Clear SHA256 bit atomically (RW)
    #[inline(always)]
    pub fn clr_sha256(&self) {
        self.clr_bits(Self::SHA256_MASK);
    }

    /// **PWM** `[16]` (RW)
    #[inline(always)]
    pub fn pwm(&self) -> bool {
        (self.value.read_bits() & Self::PWM_MASK) != 0
    }

    /// Set PWM bit atomically (RW)
    #[inline(always)]
    pub fn set_pwm(&self) {
        self.set_bits(Self::PWM_MASK);
    }

    /// Clear PWM bit atomically (RW)
    #[inline(always)]
    pub fn clr_pwm(&self) {
        self.clr_bits(Self::PWM_MASK);
    }

    /// **PLL_USB** `[15]` (RW)
    #[inline(always)]
    pub fn pll_usb(&self) -> bool {
        (self.value.read_bits() & Self::PLL_USB_MASK) != 0
    }

    /// Set PLL_USB bit atomically (RW)
    #[inline(always)]
    pub fn set_pll_usb(&self) {
        self.set_bits(Self::PLL_USB_MASK);
    }

    /// Clear PLL_USB bit atomically (RW)
    #[inline(always)]
    pub fn clr_pll_usb(&self) {
        self.clr_bits(Self::PLL_USB_MASK);
    }

    /// **PLL_SYS** `[14]` (RW)
    #[inline(always)]
    pub fn pll_sys(&self) -> bool {
        (self.value.read_bits() & Self::PLL_SYS_MASK) != 0
    }

    /// Set PLL_SYS bit atomically (RW)
    #[inline(always)]
    pub fn set_pll_sys(&self) {
        self.set_bits(Self::PLL_SYS_MASK);
    }

    /// Clear PLL_SYS bit atomically (RW)
    #[inline(always)]
    pub fn clr_pll_sys(&self) {
        self.clr_bits(Self::PLL_SYS_MASK);
    }

    /// **PIO2** `[13]` (RW)
    #[inline(always)]
    pub fn pio2(&self) -> bool {
        (self.value.read_bits() & Self::PIO2_MASK) != 0
    }

    /// Set PIO2 bit atomically (RW)
    #[inline(always)]
    pub fn set_pio2(&self) {
        self.set_bits(Self::PIO2_MASK);
    }

    /// Clear PIO2 bit atomically (RW)
    #[inline(always)]
    pub fn clr_pio2(&self) {
        self.clr_bits(Self::PIO2_MASK);
    }

    /// **PIO1** `[12]` (RW)
    #[inline(always)]
    pub fn pio1(&self) -> bool {
        (self.value.read_bits() & Self::PIO1_MASK) != 0
    }

    /// Set PIO1 bit atomically (RW)
    #[inline(always)]
    pub fn set_pio1(&self) {
        self.set_bits(Self::PIO1_MASK);
    }

    /// Clear PIO1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_pio1(&self) {
        self.clr_bits(Self::PIO1_MASK);
    }

    /// **PIO0** `[11]` (RW)
    #[inline(always)]
    pub fn pio0(&self) -> bool {
        (self.value.read_bits() & Self::PIO0_MASK) != 0
    }

    /// Set PIO0 bit atomically (RW)
    #[inline(always)]
    pub fn set_pio0(&self) {
        self.set_bits(Self::PIO0_MASK);
    }

    /// Clear PIO0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_pio0(&self) {
        self.clr_bits(Self::PIO0_MASK);
    }

    /// **PADS_QSPI** `[10]` (RW)
    #[inline(always)]
    pub fn pads_qspi(&self) -> bool {
        (self.value.read_bits() & Self::PADS_QSPI_MASK) != 0
    }

    /// Set PADS_QSPI bit atomically (RW)
    #[inline(always)]
    pub fn set_pads_qspi(&self) {
        self.set_bits(Self::PADS_QSPI_MASK);
    }

    /// Clear PADS_QSPI bit atomically (RW)
    #[inline(always)]
    pub fn clr_pads_qspi(&self) {
        self.clr_bits(Self::PADS_QSPI_MASK);
    }

    /// **PADS_BANK0** `[9]` (RW)
    #[inline(always)]
    pub fn pads_bank0(&self) -> bool {
        (self.value.read_bits() & Self::PADS_BANK0_MASK) != 0
    }

    /// Set PADS_BANK0 bit atomically (RW)
    #[inline(always)]
    pub fn set_pads_bank0(&self) {
        self.set_bits(Self::PADS_BANK0_MASK);
    }

    /// Clear PADS_BANK0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_pads_bank0(&self) {
        self.clr_bits(Self::PADS_BANK0_MASK);
    }

    /// **JTAG** `[8]` (RW)
    #[inline(always)]
    pub fn jtag(&self) -> bool {
        (self.value.read_bits() & Self::JTAG_MASK) != 0
    }

    /// Set JTAG bit atomically (RW)
    #[inline(always)]
    pub fn set_jtag(&self) {
        self.set_bits(Self::JTAG_MASK);
    }

    /// Clear JTAG bit atomically (RW)
    #[inline(always)]
    pub fn clr_jtag(&self) {
        self.clr_bits(Self::JTAG_MASK);
    }

    /// **IO_QSPI** `[7]` (RW)
    #[inline(always)]
    pub fn io_qspi(&self) -> bool {
        (self.value.read_bits() & Self::IO_QSPI_MASK) != 0
    }

    /// Set IO_QSPI bit atomically (RW)
    #[inline(always)]
    pub fn set_io_qspi(&self) {
        self.set_bits(Self::IO_QSPI_MASK);
    }

    /// Clear IO_QSPI bit atomically (RW)
    #[inline(always)]
    pub fn clr_io_qspi(&self) {
        self.clr_bits(Self::IO_QSPI_MASK);
    }

    /// **IO_BANK0** `[6]` (RW)
    #[inline(always)]
    pub fn io_bank0(&self) -> bool {
        (self.value.read_bits() & Self::IO_BANK0_MASK) != 0
    }

    /// Set IO_BANK0 bit atomically (RW)
    #[inline(always)]
    pub fn set_io_bank0(&self) {
        self.set_bits(Self::IO_BANK0_MASK);
    }

    /// Clear IO_BANK0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_io_bank0(&self) {
        self.clr_bits(Self::IO_BANK0_MASK);
    }

    /// **I2C1** `[5]` (RW)
    #[inline(always)]
    pub fn i2c1(&self) -> bool {
        (self.value.read_bits() & Self::I2C1_MASK) != 0
    }

    /// Set I2C1 bit atomically (RW)
    #[inline(always)]
    pub fn set_i2c1(&self) {
        self.set_bits(Self::I2C1_MASK);
    }

    /// Clear I2C1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_i2c1(&self) {
        self.clr_bits(Self::I2C1_MASK);
    }

    /// **I2C0** `[4]` (RW)
    #[inline(always)]
    pub fn i2c0(&self) -> bool {
        (self.value.read_bits() & Self::I2C0_MASK) != 0
    }

    /// Set I2C0 bit atomically (RW)
    #[inline(always)]
    pub fn set_i2c0(&self) {
        self.set_bits(Self::I2C0_MASK);
    }

    /// Clear I2C0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_i2c0(&self) {
        self.clr_bits(Self::I2C0_MASK);
    }

    /// **HSTX** `[3]` (RW)
    #[inline(always)]
    pub fn hstx(&self) -> bool {
        (self.value.read_bits() & Self::HSTX_MASK) != 0
    }

    /// Set HSTX bit atomically (RW)
    #[inline(always)]
    pub fn set_hstx(&self) {
        self.set_bits(Self::HSTX_MASK);
    }

    /// Clear HSTX bit atomically (RW)
    #[inline(always)]
    pub fn clr_hstx(&self) {
        self.clr_bits(Self::HSTX_MASK);
    }

    /// **DMA** `[2]` (RW)
    #[inline(always)]
    pub fn dma(&self) -> bool {
        (self.value.read_bits() & Self::DMA_MASK) != 0
    }

    /// Set DMA bit atomically (RW)
    #[inline(always)]
    pub fn set_dma(&self) {
        self.set_bits(Self::DMA_MASK);
    }

    /// Clear DMA bit atomically (RW)
    #[inline(always)]
    pub fn clr_dma(&self) {
        self.clr_bits(Self::DMA_MASK);
    }

    /// **BUSCTRL** `[1]` (RW)
    #[inline(always)]
    pub fn busctrl(&self) -> bool {
        (self.value.read_bits() & Self::BUSCTRL_MASK) != 0
    }

    /// Set BUSCTRL bit atomically (RW)
    #[inline(always)]
    pub fn set_busctrl(&self) {
        self.set_bits(Self::BUSCTRL_MASK);
    }

    /// Clear BUSCTRL bit atomically (RW)
    #[inline(always)]
    pub fn clr_busctrl(&self) {
        self.clr_bits(Self::BUSCTRL_MASK);
    }

    /// **ADC** `[0]` (RW)
    #[inline(always)]
    pub fn adc(&self) -> bool {
        (self.value.read_bits() & Self::ADC_MASK) != 0
    }

    /// Set ADC bit atomically (RW)
    #[inline(always)]
    pub fn set_adc(&self) {
        self.set_bits(Self::ADC_MASK);
    }

    /// Clear ADC bit atomically (RW)
    #[inline(always)]
    pub fn clr_adc(&self) {
        self.clr_bits(Self::ADC_MASK);
    }
}

/// WDSEL value type for batch read/modify/write operations
///
/// Use `wdsel.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `wdsel.write_val(val)`.
#[derive(Clone, Copy)]
pub struct WdselVal {
    bits: u32,
}

impl WdselVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
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

    /// Read **USBCTRL** `[28:28]`
    #[inline(always)]
    pub const fn usbctrl(&self) -> bool {
        (self.bits & Wdsel::USBCTRL_MASK) != 0
    }

    /// Read **UART1** `[27:27]`
    #[inline(always)]
    pub const fn uart1(&self) -> bool {
        (self.bits & Wdsel::UART1_MASK) != 0
    }

    /// Read **UART0** `[26:26]`
    #[inline(always)]
    pub const fn uart0(&self) -> bool {
        (self.bits & Wdsel::UART0_MASK) != 0
    }

    /// Read **TRNG** `[25:25]`
    #[inline(always)]
    pub const fn trng(&self) -> bool {
        (self.bits & Wdsel::TRNG_MASK) != 0
    }

    /// Read **TIMER1** `[24:24]`
    #[inline(always)]
    pub const fn timer1(&self) -> bool {
        (self.bits & Wdsel::TIMER1_MASK) != 0
    }

    /// Read **TIMER0** `[23:23]`
    #[inline(always)]
    pub const fn timer0(&self) -> bool {
        (self.bits & Wdsel::TIMER0_MASK) != 0
    }

    /// Read **TBMAN** `[22:22]`
    #[inline(always)]
    pub const fn tbman(&self) -> bool {
        (self.bits & Wdsel::TBMAN_MASK) != 0
    }

    /// Read **SYSINFO** `[21:21]`
    #[inline(always)]
    pub const fn sysinfo(&self) -> bool {
        (self.bits & Wdsel::SYSINFO_MASK) != 0
    }

    /// Read **SYSCFG** `[20:20]`
    #[inline(always)]
    pub const fn syscfg(&self) -> bool {
        (self.bits & Wdsel::SYSCFG_MASK) != 0
    }

    /// Read **SPI1** `[19:19]`
    #[inline(always)]
    pub const fn spi1(&self) -> bool {
        (self.bits & Wdsel::SPI1_MASK) != 0
    }

    /// Read **SPI0** `[18:18]`
    #[inline(always)]
    pub const fn spi0(&self) -> bool {
        (self.bits & Wdsel::SPI0_MASK) != 0
    }

    /// Read **SHA256** `[17:17]`
    #[inline(always)]
    pub const fn sha256(&self) -> bool {
        (self.bits & Wdsel::SHA256_MASK) != 0
    }

    /// Read **PWM** `[16:16]`
    #[inline(always)]
    pub const fn pwm(&self) -> bool {
        (self.bits & Wdsel::PWM_MASK) != 0
    }

    /// Read **PLL_USB** `[15:15]`
    #[inline(always)]
    pub const fn pll_usb(&self) -> bool {
        (self.bits & Wdsel::PLL_USB_MASK) != 0
    }

    /// Read **PLL_SYS** `[14:14]`
    #[inline(always)]
    pub const fn pll_sys(&self) -> bool {
        (self.bits & Wdsel::PLL_SYS_MASK) != 0
    }

    /// Read **PIO2** `[13:13]`
    #[inline(always)]
    pub const fn pio2(&self) -> bool {
        (self.bits & Wdsel::PIO2_MASK) != 0
    }

    /// Read **PIO1** `[12:12]`
    #[inline(always)]
    pub const fn pio1(&self) -> bool {
        (self.bits & Wdsel::PIO1_MASK) != 0
    }

    /// Read **PIO0** `[11:11]`
    #[inline(always)]
    pub const fn pio0(&self) -> bool {
        (self.bits & Wdsel::PIO0_MASK) != 0
    }

    /// Read **PADS_QSPI** `[10:10]`
    #[inline(always)]
    pub const fn pads_qspi(&self) -> bool {
        (self.bits & Wdsel::PADS_QSPI_MASK) != 0
    }

    /// Read **PADS_BANK0** `[9:9]`
    #[inline(always)]
    pub const fn pads_bank0(&self) -> bool {
        (self.bits & Wdsel::PADS_BANK0_MASK) != 0
    }

    /// Read **JTAG** `[8:8]`
    #[inline(always)]
    pub const fn jtag(&self) -> bool {
        (self.bits & Wdsel::JTAG_MASK) != 0
    }

    /// Read **IO_QSPI** `[7:7]`
    #[inline(always)]
    pub const fn io_qspi(&self) -> bool {
        (self.bits & Wdsel::IO_QSPI_MASK) != 0
    }

    /// Read **IO_BANK0** `[6:6]`
    #[inline(always)]
    pub const fn io_bank0(&self) -> bool {
        (self.bits & Wdsel::IO_BANK0_MASK) != 0
    }

    /// Read **I2C1** `[5:5]`
    #[inline(always)]
    pub const fn i2c1(&self) -> bool {
        (self.bits & Wdsel::I2C1_MASK) != 0
    }

    /// Read **I2C0** `[4:4]`
    #[inline(always)]
    pub const fn i2c0(&self) -> bool {
        (self.bits & Wdsel::I2C0_MASK) != 0
    }

    /// Read **HSTX** `[3:3]`
    #[inline(always)]
    pub const fn hstx(&self) -> bool {
        (self.bits & Wdsel::HSTX_MASK) != 0
    }

    /// Read **DMA** `[2:2]`
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        (self.bits & Wdsel::DMA_MASK) != 0
    }

    /// Read **BUSCTRL** `[1:1]`
    #[inline(always)]
    pub const fn busctrl(&self) -> bool {
        (self.bits & Wdsel::BUSCTRL_MASK) != 0
    }

    /// Read **ADC** `[0:0]`
    #[inline(always)]
    pub const fn adc(&self) -> bool {
        (self.bits & Wdsel::ADC_MASK) != 0
    }

    /// Set **USBCTRL** `[28:28]`
    #[inline(always)]
    pub const fn with_usbctrl(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Wdsel::USBCTRL_MASK
            } else {
                self.bits & !Wdsel::USBCTRL_MASK
            }
        }
    }

    /// Set **UART1** `[27:27]`
    #[inline(always)]
    pub const fn with_uart1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Wdsel::UART1_MASK
            } else {
                self.bits & !Wdsel::UART1_MASK
            }
        }
    }

    /// Set **UART0** `[26:26]`
    #[inline(always)]
    pub const fn with_uart0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Wdsel::UART0_MASK
            } else {
                self.bits & !Wdsel::UART0_MASK
            }
        }
    }

    /// Set **TRNG** `[25:25]`
    #[inline(always)]
    pub const fn with_trng(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Wdsel::TRNG_MASK
            } else {
                self.bits & !Wdsel::TRNG_MASK
            }
        }
    }

    /// Set **TIMER1** `[24:24]`
    #[inline(always)]
    pub const fn with_timer1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Wdsel::TIMER1_MASK
            } else {
                self.bits & !Wdsel::TIMER1_MASK
            }
        }
    }

    /// Set **TIMER0** `[23:23]`
    #[inline(always)]
    pub const fn with_timer0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Wdsel::TIMER0_MASK
            } else {
                self.bits & !Wdsel::TIMER0_MASK
            }
        }
    }

    /// Set **TBMAN** `[22:22]`
    #[inline(always)]
    pub const fn with_tbman(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Wdsel::TBMAN_MASK
            } else {
                self.bits & !Wdsel::TBMAN_MASK
            }
        }
    }

    /// Set **SYSINFO** `[21:21]`
    #[inline(always)]
    pub const fn with_sysinfo(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Wdsel::SYSINFO_MASK
            } else {
                self.bits & !Wdsel::SYSINFO_MASK
            }
        }
    }

    /// Set **SYSCFG** `[20:20]`
    #[inline(always)]
    pub const fn with_syscfg(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Wdsel::SYSCFG_MASK
            } else {
                self.bits & !Wdsel::SYSCFG_MASK
            }
        }
    }

    /// Set **SPI1** `[19:19]`
    #[inline(always)]
    pub const fn with_spi1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Wdsel::SPI1_MASK
            } else {
                self.bits & !Wdsel::SPI1_MASK
            }
        }
    }

    /// Set **SPI0** `[18:18]`
    #[inline(always)]
    pub const fn with_spi0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Wdsel::SPI0_MASK
            } else {
                self.bits & !Wdsel::SPI0_MASK
            }
        }
    }

    /// Set **SHA256** `[17:17]`
    #[inline(always)]
    pub const fn with_sha256(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Wdsel::SHA256_MASK
            } else {
                self.bits & !Wdsel::SHA256_MASK
            }
        }
    }

    /// Set **PWM** `[16:16]`
    #[inline(always)]
    pub const fn with_pwm(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Wdsel::PWM_MASK
            } else {
                self.bits & !Wdsel::PWM_MASK
            }
        }
    }

    /// Set **PLL_USB** `[15:15]`
    #[inline(always)]
    pub const fn with_pll_usb(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Wdsel::PLL_USB_MASK
            } else {
                self.bits & !Wdsel::PLL_USB_MASK
            }
        }
    }

    /// Set **PLL_SYS** `[14:14]`
    #[inline(always)]
    pub const fn with_pll_sys(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Wdsel::PLL_SYS_MASK
            } else {
                self.bits & !Wdsel::PLL_SYS_MASK
            }
        }
    }

    /// Set **PIO2** `[13:13]`
    #[inline(always)]
    pub const fn with_pio2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Wdsel::PIO2_MASK
            } else {
                self.bits & !Wdsel::PIO2_MASK
            }
        }
    }

    /// Set **PIO1** `[12:12]`
    #[inline(always)]
    pub const fn with_pio1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Wdsel::PIO1_MASK
            } else {
                self.bits & !Wdsel::PIO1_MASK
            }
        }
    }

    /// Set **PIO0** `[11:11]`
    #[inline(always)]
    pub const fn with_pio0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Wdsel::PIO0_MASK
            } else {
                self.bits & !Wdsel::PIO0_MASK
            }
        }
    }

    /// Set **PADS_QSPI** `[10:10]`
    #[inline(always)]
    pub const fn with_pads_qspi(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Wdsel::PADS_QSPI_MASK
            } else {
                self.bits & !Wdsel::PADS_QSPI_MASK
            }
        }
    }

    /// Set **PADS_BANK0** `[9:9]`
    #[inline(always)]
    pub const fn with_pads_bank0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Wdsel::PADS_BANK0_MASK
            } else {
                self.bits & !Wdsel::PADS_BANK0_MASK
            }
        }
    }

    /// Set **JTAG** `[8:8]`
    #[inline(always)]
    pub const fn with_jtag(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Wdsel::JTAG_MASK
            } else {
                self.bits & !Wdsel::JTAG_MASK
            }
        }
    }

    /// Set **IO_QSPI** `[7:7]`
    #[inline(always)]
    pub const fn with_io_qspi(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Wdsel::IO_QSPI_MASK
            } else {
                self.bits & !Wdsel::IO_QSPI_MASK
            }
        }
    }

    /// Set **IO_BANK0** `[6:6]`
    #[inline(always)]
    pub const fn with_io_bank0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Wdsel::IO_BANK0_MASK
            } else {
                self.bits & !Wdsel::IO_BANK0_MASK
            }
        }
    }

    /// Set **I2C1** `[5:5]`
    #[inline(always)]
    pub const fn with_i2c1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Wdsel::I2C1_MASK
            } else {
                self.bits & !Wdsel::I2C1_MASK
            }
        }
    }

    /// Set **I2C0** `[4:4]`
    #[inline(always)]
    pub const fn with_i2c0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Wdsel::I2C0_MASK
            } else {
                self.bits & !Wdsel::I2C0_MASK
            }
        }
    }

    /// Set **HSTX** `[3:3]`
    #[inline(always)]
    pub const fn with_hstx(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Wdsel::HSTX_MASK
            } else {
                self.bits & !Wdsel::HSTX_MASK
            }
        }
    }

    /// Set **DMA** `[2:2]`
    #[inline(always)]
    pub const fn with_dma(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Wdsel::DMA_MASK
            } else {
                self.bits & !Wdsel::DMA_MASK
            }
        }
    }

    /// Set **BUSCTRL** `[1:1]`
    #[inline(always)]
    pub const fn with_busctrl(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Wdsel::BUSCTRL_MASK
            } else {
                self.bits & !Wdsel::BUSCTRL_MASK
            }
        }
    }

    /// Set **ADC** `[0:0]`
    #[inline(always)]
    pub const fn with_adc(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Wdsel::ADC_MASK
            } else {
                self.bits & !Wdsel::ADC_MASK
            }
        }
    }
}

impl Default for WdselVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Wdsel {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> WdselVal {
        WdselVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: WdselVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(WdselVal) -> WdselVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RESET_DONE Register
///
/// Address offset: 0x0008
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **USBCTRL** `[28]` (RO)
///
/// - **UART1** `[27]` (RO)
///
/// - **UART0** `[26]` (RO)
///
/// - **TRNG** `[25]` (RO)
///
/// - **TIMER1** `[24]` (RO)
///
/// - **TIMER0** `[23]` (RO)
///
/// - **TBMAN** `[22]` (RO)
///
/// - **SYSINFO** `[21]` (RO)
///
/// - **SYSCFG** `[20]` (RO)
///
/// - **SPI1** `[19]` (RO)
///
/// - **SPI0** `[18]` (RO)
///
/// - **SHA256** `[17]` (RO)
///
/// - **PWM** `[16]` (RO)
///
/// - **PLL_USB** `[15]` (RO)
///
/// - **PLL_SYS** `[14]` (RO)
///
/// - **PIO2** `[13]` (RO)
///
/// - **PIO1** `[12]` (RO)
///
/// - **PIO0** `[11]` (RO)
///
/// - **PADS_QSPI** `[10]` (RO)
///
/// - **PADS_BANK0** `[9]` (RO)
///
/// - **JTAG** `[8]` (RO)
///
/// - **IO_QSPI** `[7]` (RO)
///
/// - **IO_BANK0** `[6]` (RO)
///
/// - **I2C1** `[5]` (RO)
///
/// - **I2C0** `[4]` (RO)
///
/// - **HSTX** `[3]` (RO)
///
/// - **DMA** `[2]` (RO)
///
/// - **BUSCTRL** `[1]` (RO)
///
/// - **ADC** `[0]` (RO)
#[repr(transparent)]
pub struct ResetDone {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for ResetDone {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ResetDone {
    // ======== Field Constants ========

    /// **USBCTRL**
    /// Bits `[28:28]`, mask `0x10000000`
    pub const USBCTRL_OFFSET: u32 = 28;
    pub const USBCTRL_MASK: u32 = 0x10000000;
    pub const USBCTRL: u32 = 1 << 28;

    /// **UART1**
    /// Bits `[27:27]`, mask `0x08000000`
    pub const UART1_OFFSET: u32 = 27;
    pub const UART1_MASK: u32 = 0x08000000;
    pub const UART1: u32 = 1 << 27;

    /// **UART0**
    /// Bits `[26:26]`, mask `0x04000000`
    pub const UART0_OFFSET: u32 = 26;
    pub const UART0_MASK: u32 = 0x04000000;
    pub const UART0: u32 = 1 << 26;

    /// **TRNG**
    /// Bits `[25:25]`, mask `0x02000000`
    pub const TRNG_OFFSET: u32 = 25;
    pub const TRNG_MASK: u32 = 0x02000000;
    pub const TRNG: u32 = 1 << 25;

    /// **TIMER1**
    /// Bits `[24:24]`, mask `0x01000000`
    pub const TIMER1_OFFSET: u32 = 24;
    pub const TIMER1_MASK: u32 = 0x01000000;
    pub const TIMER1: u32 = 1 << 24;

    /// **TIMER0**
    /// Bits `[23:23]`, mask `0x00800000`
    pub const TIMER0_OFFSET: u32 = 23;
    pub const TIMER0_MASK: u32 = 0x00800000;
    pub const TIMER0: u32 = 1 << 23;

    /// **TBMAN**
    /// Bits `[22:22]`, mask `0x00400000`
    pub const TBMAN_OFFSET: u32 = 22;
    pub const TBMAN_MASK: u32 = 0x00400000;
    pub const TBMAN: u32 = 1 << 22;

    /// **SYSINFO**
    /// Bits `[21:21]`, mask `0x00200000`
    pub const SYSINFO_OFFSET: u32 = 21;
    pub const SYSINFO_MASK: u32 = 0x00200000;
    pub const SYSINFO: u32 = 1 << 21;

    /// **SYSCFG**
    /// Bits `[20:20]`, mask `0x00100000`
    pub const SYSCFG_OFFSET: u32 = 20;
    pub const SYSCFG_MASK: u32 = 0x00100000;
    pub const SYSCFG: u32 = 1 << 20;

    /// **SPI1**
    /// Bits `[19:19]`, mask `0x00080000`
    pub const SPI1_OFFSET: u32 = 19;
    pub const SPI1_MASK: u32 = 0x00080000;
    pub const SPI1: u32 = 1 << 19;

    /// **SPI0**
    /// Bits `[18:18]`, mask `0x00040000`
    pub const SPI0_OFFSET: u32 = 18;
    pub const SPI0_MASK: u32 = 0x00040000;
    pub const SPI0: u32 = 1 << 18;

    /// **SHA256**
    /// Bits `[17:17]`, mask `0x00020000`
    pub const SHA256_OFFSET: u32 = 17;
    pub const SHA256_MASK: u32 = 0x00020000;
    pub const SHA256: u32 = 1 << 17;

    /// **PWM**
    /// Bits `[16:16]`, mask `0x00010000`
    pub const PWM_OFFSET: u32 = 16;
    pub const PWM_MASK: u32 = 0x00010000;
    pub const PWM: u32 = 1 << 16;

    /// **PLL_USB**
    /// Bits `[15:15]`, mask `0x00008000`
    pub const PLL_USB_OFFSET: u32 = 15;
    pub const PLL_USB_MASK: u32 = 0x00008000;
    pub const PLL_USB: u32 = 1 << 15;

    /// **PLL_SYS**
    /// Bits `[14:14]`, mask `0x00004000`
    pub const PLL_SYS_OFFSET: u32 = 14;
    pub const PLL_SYS_MASK: u32 = 0x00004000;
    pub const PLL_SYS: u32 = 1 << 14;

    /// **PIO2**
    /// Bits `[13:13]`, mask `0x00002000`
    pub const PIO2_OFFSET: u32 = 13;
    pub const PIO2_MASK: u32 = 0x00002000;
    pub const PIO2: u32 = 1 << 13;

    /// **PIO1**
    /// Bits `[12:12]`, mask `0x00001000`
    pub const PIO1_OFFSET: u32 = 12;
    pub const PIO1_MASK: u32 = 0x00001000;
    pub const PIO1: u32 = 1 << 12;

    /// **PIO0**
    /// Bits `[11:11]`, mask `0x00000800`
    pub const PIO0_OFFSET: u32 = 11;
    pub const PIO0_MASK: u32 = 0x00000800;
    pub const PIO0: u32 = 1 << 11;

    /// **PADS_QSPI**
    /// Bits `[10:10]`, mask `0x00000400`
    pub const PADS_QSPI_OFFSET: u32 = 10;
    pub const PADS_QSPI_MASK: u32 = 0x00000400;
    pub const PADS_QSPI: u32 = 1 << 10;

    /// **PADS_BANK0**
    /// Bits `[9:9]`, mask `0x00000200`
    pub const PADS_BANK0_OFFSET: u32 = 9;
    pub const PADS_BANK0_MASK: u32 = 0x00000200;
    pub const PADS_BANK0: u32 = 1 << 9;

    /// **JTAG**
    /// Bits `[8:8]`, mask `0x00000100`
    pub const JTAG_OFFSET: u32 = 8;
    pub const JTAG_MASK: u32 = 0x00000100;
    pub const JTAG: u32 = 1 << 8;

    /// **IO_QSPI**
    /// Bits `[7:7]`, mask `0x00000080`
    pub const IO_QSPI_OFFSET: u32 = 7;
    pub const IO_QSPI_MASK: u32 = 0x00000080;
    pub const IO_QSPI: u32 = 1 << 7;

    /// **IO_BANK0**
    /// Bits `[6:6]`, mask `0x00000040`
    pub const IO_BANK0_OFFSET: u32 = 6;
    pub const IO_BANK0_MASK: u32 = 0x00000040;
    pub const IO_BANK0: u32 = 1 << 6;

    /// **I2C1**
    /// Bits `[5:5]`, mask `0x00000020`
    pub const I2C1_OFFSET: u32 = 5;
    pub const I2C1_MASK: u32 = 0x00000020;
    pub const I2C1: u32 = 1 << 5;

    /// **I2C0**
    /// Bits `[4:4]`, mask `0x00000010`
    pub const I2C0_OFFSET: u32 = 4;
    pub const I2C0_MASK: u32 = 0x00000010;
    pub const I2C0: u32 = 1 << 4;

    /// **HSTX**
    /// Bits `[3:3]`, mask `0x00000008`
    pub const HSTX_OFFSET: u32 = 3;
    pub const HSTX_MASK: u32 = 0x00000008;
    pub const HSTX: u32 = 1 << 3;

    /// **DMA**
    /// Bits `[2:2]`, mask `0x00000004`
    pub const DMA_OFFSET: u32 = 2;
    pub const DMA_MASK: u32 = 0x00000004;
    pub const DMA: u32 = 1 << 2;

    /// **BUSCTRL**
    /// Bits `[1:1]`, mask `0x00000002`
    pub const BUSCTRL_OFFSET: u32 = 1;
    pub const BUSCTRL_MASK: u32 = 0x00000002;
    pub const BUSCTRL: u32 = 1 << 1;

    /// **ADC**
    /// Bits `[0:0]`, mask `0x00000001`
    pub const ADC_OFFSET: u32 = 0;
    pub const ADC_MASK: u32 = 0x00000001;
    pub const ADC: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **USBCTRL** `[28]` (RO)
    #[inline(always)]
    pub fn usbctrl(&self) -> bool {
        (self.value.read_bits() & Self::USBCTRL_MASK) != 0
    }

    /// **UART1** `[27]` (RO)
    #[inline(always)]
    pub fn uart1(&self) -> bool {
        (self.value.read_bits() & Self::UART1_MASK) != 0
    }

    /// **UART0** `[26]` (RO)
    #[inline(always)]
    pub fn uart0(&self) -> bool {
        (self.value.read_bits() & Self::UART0_MASK) != 0
    }

    /// **TRNG** `[25]` (RO)
    #[inline(always)]
    pub fn trng(&self) -> bool {
        (self.value.read_bits() & Self::TRNG_MASK) != 0
    }

    /// **TIMER1** `[24]` (RO)
    #[inline(always)]
    pub fn timer1(&self) -> bool {
        (self.value.read_bits() & Self::TIMER1_MASK) != 0
    }

    /// **TIMER0** `[23]` (RO)
    #[inline(always)]
    pub fn timer0(&self) -> bool {
        (self.value.read_bits() & Self::TIMER0_MASK) != 0
    }

    /// **TBMAN** `[22]` (RO)
    #[inline(always)]
    pub fn tbman(&self) -> bool {
        (self.value.read_bits() & Self::TBMAN_MASK) != 0
    }

    /// **SYSINFO** `[21]` (RO)
    #[inline(always)]
    pub fn sysinfo(&self) -> bool {
        (self.value.read_bits() & Self::SYSINFO_MASK) != 0
    }

    /// **SYSCFG** `[20]` (RO)
    #[inline(always)]
    pub fn syscfg(&self) -> bool {
        (self.value.read_bits() & Self::SYSCFG_MASK) != 0
    }

    /// **SPI1** `[19]` (RO)
    #[inline(always)]
    pub fn spi1(&self) -> bool {
        (self.value.read_bits() & Self::SPI1_MASK) != 0
    }

    /// **SPI0** `[18]` (RO)
    #[inline(always)]
    pub fn spi0(&self) -> bool {
        (self.value.read_bits() & Self::SPI0_MASK) != 0
    }

    /// **SHA256** `[17]` (RO)
    #[inline(always)]
    pub fn sha256(&self) -> bool {
        (self.value.read_bits() & Self::SHA256_MASK) != 0
    }

    /// **PWM** `[16]` (RO)
    #[inline(always)]
    pub fn pwm(&self) -> bool {
        (self.value.read_bits() & Self::PWM_MASK) != 0
    }

    /// **PLL_USB** `[15]` (RO)
    #[inline(always)]
    pub fn pll_usb(&self) -> bool {
        (self.value.read_bits() & Self::PLL_USB_MASK) != 0
    }

    /// **PLL_SYS** `[14]` (RO)
    #[inline(always)]
    pub fn pll_sys(&self) -> bool {
        (self.value.read_bits() & Self::PLL_SYS_MASK) != 0
    }

    /// **PIO2** `[13]` (RO)
    #[inline(always)]
    pub fn pio2(&self) -> bool {
        (self.value.read_bits() & Self::PIO2_MASK) != 0
    }

    /// **PIO1** `[12]` (RO)
    #[inline(always)]
    pub fn pio1(&self) -> bool {
        (self.value.read_bits() & Self::PIO1_MASK) != 0
    }

    /// **PIO0** `[11]` (RO)
    #[inline(always)]
    pub fn pio0(&self) -> bool {
        (self.value.read_bits() & Self::PIO0_MASK) != 0
    }

    /// **PADS_QSPI** `[10]` (RO)
    #[inline(always)]
    pub fn pads_qspi(&self) -> bool {
        (self.value.read_bits() & Self::PADS_QSPI_MASK) != 0
    }

    /// **PADS_BANK0** `[9]` (RO)
    #[inline(always)]
    pub fn pads_bank0(&self) -> bool {
        (self.value.read_bits() & Self::PADS_BANK0_MASK) != 0
    }

    /// **JTAG** `[8]` (RO)
    #[inline(always)]
    pub fn jtag(&self) -> bool {
        (self.value.read_bits() & Self::JTAG_MASK) != 0
    }

    /// **IO_QSPI** `[7]` (RO)
    #[inline(always)]
    pub fn io_qspi(&self) -> bool {
        (self.value.read_bits() & Self::IO_QSPI_MASK) != 0
    }

    /// **IO_BANK0** `[6]` (RO)
    #[inline(always)]
    pub fn io_bank0(&self) -> bool {
        (self.value.read_bits() & Self::IO_BANK0_MASK) != 0
    }

    /// **I2C1** `[5]` (RO)
    #[inline(always)]
    pub fn i2c1(&self) -> bool {
        (self.value.read_bits() & Self::I2C1_MASK) != 0
    }

    /// **I2C0** `[4]` (RO)
    #[inline(always)]
    pub fn i2c0(&self) -> bool {
        (self.value.read_bits() & Self::I2C0_MASK) != 0
    }

    /// **HSTX** `[3]` (RO)
    #[inline(always)]
    pub fn hstx(&self) -> bool {
        (self.value.read_bits() & Self::HSTX_MASK) != 0
    }

    /// **DMA** `[2]` (RO)
    #[inline(always)]
    pub fn dma(&self) -> bool {
        (self.value.read_bits() & Self::DMA_MASK) != 0
    }

    /// **BUSCTRL** `[1]` (RO)
    #[inline(always)]
    pub fn busctrl(&self) -> bool {
        (self.value.read_bits() & Self::BUSCTRL_MASK) != 0
    }

    /// **ADC** `[0]` (RO)
    #[inline(always)]
    pub fn adc(&self) -> bool {
        (self.value.read_bits() & Self::ADC_MASK) != 0
    }
}

/// RESET_DONE value type for batch read/modify/write operations
///
/// Use `resetdone.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `resetdone.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ResetDoneVal {
    bits: u32,
}

impl ResetDoneVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
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

    /// Read **USBCTRL** `[28:28]`
    #[inline(always)]
    pub const fn usbctrl(&self) -> bool {
        (self.bits & ResetDone::USBCTRL_MASK) != 0
    }

    /// Read **UART1** `[27:27]`
    #[inline(always)]
    pub const fn uart1(&self) -> bool {
        (self.bits & ResetDone::UART1_MASK) != 0
    }

    /// Read **UART0** `[26:26]`
    #[inline(always)]
    pub const fn uart0(&self) -> bool {
        (self.bits & ResetDone::UART0_MASK) != 0
    }

    /// Read **TRNG** `[25:25]`
    #[inline(always)]
    pub const fn trng(&self) -> bool {
        (self.bits & ResetDone::TRNG_MASK) != 0
    }

    /// Read **TIMER1** `[24:24]`
    #[inline(always)]
    pub const fn timer1(&self) -> bool {
        (self.bits & ResetDone::TIMER1_MASK) != 0
    }

    /// Read **TIMER0** `[23:23]`
    #[inline(always)]
    pub const fn timer0(&self) -> bool {
        (self.bits & ResetDone::TIMER0_MASK) != 0
    }

    /// Read **TBMAN** `[22:22]`
    #[inline(always)]
    pub const fn tbman(&self) -> bool {
        (self.bits & ResetDone::TBMAN_MASK) != 0
    }

    /// Read **SYSINFO** `[21:21]`
    #[inline(always)]
    pub const fn sysinfo(&self) -> bool {
        (self.bits & ResetDone::SYSINFO_MASK) != 0
    }

    /// Read **SYSCFG** `[20:20]`
    #[inline(always)]
    pub const fn syscfg(&self) -> bool {
        (self.bits & ResetDone::SYSCFG_MASK) != 0
    }

    /// Read **SPI1** `[19:19]`
    #[inline(always)]
    pub const fn spi1(&self) -> bool {
        (self.bits & ResetDone::SPI1_MASK) != 0
    }

    /// Read **SPI0** `[18:18]`
    #[inline(always)]
    pub const fn spi0(&self) -> bool {
        (self.bits & ResetDone::SPI0_MASK) != 0
    }

    /// Read **SHA256** `[17:17]`
    #[inline(always)]
    pub const fn sha256(&self) -> bool {
        (self.bits & ResetDone::SHA256_MASK) != 0
    }

    /// Read **PWM** `[16:16]`
    #[inline(always)]
    pub const fn pwm(&self) -> bool {
        (self.bits & ResetDone::PWM_MASK) != 0
    }

    /// Read **PLL_USB** `[15:15]`
    #[inline(always)]
    pub const fn pll_usb(&self) -> bool {
        (self.bits & ResetDone::PLL_USB_MASK) != 0
    }

    /// Read **PLL_SYS** `[14:14]`
    #[inline(always)]
    pub const fn pll_sys(&self) -> bool {
        (self.bits & ResetDone::PLL_SYS_MASK) != 0
    }

    /// Read **PIO2** `[13:13]`
    #[inline(always)]
    pub const fn pio2(&self) -> bool {
        (self.bits & ResetDone::PIO2_MASK) != 0
    }

    /// Read **PIO1** `[12:12]`
    #[inline(always)]
    pub const fn pio1(&self) -> bool {
        (self.bits & ResetDone::PIO1_MASK) != 0
    }

    /// Read **PIO0** `[11:11]`
    #[inline(always)]
    pub const fn pio0(&self) -> bool {
        (self.bits & ResetDone::PIO0_MASK) != 0
    }

    /// Read **PADS_QSPI** `[10:10]`
    #[inline(always)]
    pub const fn pads_qspi(&self) -> bool {
        (self.bits & ResetDone::PADS_QSPI_MASK) != 0
    }

    /// Read **PADS_BANK0** `[9:9]`
    #[inline(always)]
    pub const fn pads_bank0(&self) -> bool {
        (self.bits & ResetDone::PADS_BANK0_MASK) != 0
    }

    /// Read **JTAG** `[8:8]`
    #[inline(always)]
    pub const fn jtag(&self) -> bool {
        (self.bits & ResetDone::JTAG_MASK) != 0
    }

    /// Read **IO_QSPI** `[7:7]`
    #[inline(always)]
    pub const fn io_qspi(&self) -> bool {
        (self.bits & ResetDone::IO_QSPI_MASK) != 0
    }

    /// Read **IO_BANK0** `[6:6]`
    #[inline(always)]
    pub const fn io_bank0(&self) -> bool {
        (self.bits & ResetDone::IO_BANK0_MASK) != 0
    }

    /// Read **I2C1** `[5:5]`
    #[inline(always)]
    pub const fn i2c1(&self) -> bool {
        (self.bits & ResetDone::I2C1_MASK) != 0
    }

    /// Read **I2C0** `[4:4]`
    #[inline(always)]
    pub const fn i2c0(&self) -> bool {
        (self.bits & ResetDone::I2C0_MASK) != 0
    }

    /// Read **HSTX** `[3:3]`
    #[inline(always)]
    pub const fn hstx(&self) -> bool {
        (self.bits & ResetDone::HSTX_MASK) != 0
    }

    /// Read **DMA** `[2:2]`
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        (self.bits & ResetDone::DMA_MASK) != 0
    }

    /// Read **BUSCTRL** `[1:1]`
    #[inline(always)]
    pub const fn busctrl(&self) -> bool {
        (self.bits & ResetDone::BUSCTRL_MASK) != 0
    }

    /// Read **ADC** `[0:0]`
    #[inline(always)]
    pub const fn adc(&self) -> bool {
        (self.bits & ResetDone::ADC_MASK) != 0
    }
}

impl Default for ResetDoneVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl ResetDone {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ResetDoneVal {
        ResetDoneVal::from_bits(self.value.read_bits())
    }
}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    pub reset: Reset,
    /// 0x0004
    pub wdsel: Wdsel,
    /// 0x0008
    pub reset_done: ResetDone,
}

