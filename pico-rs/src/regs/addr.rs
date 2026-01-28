//! RP2350 Memory Map - Base addresses for all peripherals

// Atomic register access aliases (offset from base address)
/// Read-Modify-Write (normal access)
pub const REG_RW: u32 = 0x0000;
/// XOR alias - XOR value with current register
pub const REG_XOR: u32 = 0x1000;
/// SET alias - Set bits in register
pub const REG_SET: u32 = 0x2000;
/// CLR alias - Clear bits in register
pub const REG_CLR: u32 = 0x3000;

// Memory regions
pub const ROM_BASE: u32 = 0x0000_0000;
pub const XIP_BASE: u32 = 0x1000_0000;
pub const SRAM_BASE: u32 = 0x2000_0000;
pub const SRAM_END: u32 = 0x2008_2000;
pub const SRAM8_BASE: u32 = 0x2008_0000;
pub const SRAM9_BASE: u32 = 0x2008_1000;

// APB Peripherals
pub const SYSINFO_BASE: u32 = 0x4000_0000;
pub const SYSCFG_BASE: u32 = 0x4000_8000;
pub const CLOCKS_BASE: u32 = 0x4001_0000;
pub const PSM_BASE: u32 = 0x4001_8000;
pub const RESETS_BASE: u32 = 0x4002_0000;
pub const IO_BANK0_BASE: u32 = 0x4002_8000;
pub const IO_QSPI_BASE: u32 = 0x4003_0000;
pub const PADS_BANK0_BASE: u32 = 0x4003_8000;
pub const PADS_QSPI_BASE: u32 = 0x4004_0000;
pub const XOSC_BASE: u32 = 0x4004_8000;
pub const PLL_SYS_BASE: u32 = 0x4005_0000;
pub const PLL_USB_BASE: u32 = 0x4005_8000;
pub const ACCESSCTRL_BASE: u32 = 0x4006_0000;
pub const BUSCTRL_BASE: u32 = 0x4006_8000;
pub const UART0_BASE: u32 = 0x4007_0000;
pub const UART1_BASE: u32 = 0x4007_8000;
pub const SPI0_BASE: u32 = 0x4008_0000;
pub const SPI1_BASE: u32 = 0x4008_8000;
pub const I2C0_BASE: u32 = 0x4009_0000;
pub const I2C1_BASE: u32 = 0x4009_8000;
pub const ADC_BASE: u32 = 0x400A_0000;
pub const PWM_BASE: u32 = 0x400A_8000;
pub const TIMER0_BASE: u32 = 0x400B_0000;
pub const TIMER1_BASE: u32 = 0x400B_8000;
pub const HSTX_CTRL_BASE: u32 = 0x400C_0000;
pub const XIP_CTRL_BASE: u32 = 0x400C_8000;
pub const XIP_QMI_BASE: u32 = 0x400D_0000;
pub const WATCHDOG_BASE: u32 = 0x400D_8000;
pub const BOOTRAM_BASE: u32 = 0x400E_0000;
pub const ROSC_BASE: u32 = 0x400E_8000;
pub const TRNG_BASE: u32 = 0x400F_0000;
pub const SHA256_BASE: u32 = 0x400F_8000;
pub const POWMAN_BASE: u32 = 0x4010_0000;
pub const TICKS_BASE: u32 = 0x4010_8000;
pub const OTP_BASE: u32 = 0x4012_0000;
pub const OTP_DATA_BASE: u32 = 0x4013_0000;

// AHB-Lite Peripherals
pub const DMA_BASE: u32 = 0x5000_0000;
pub const USBCTRL_BASE: u32 = 0x5010_0000;
pub const USBCTRL_DPRAM_BASE: u32 = 0x5010_0000;
pub const USBCTRL_REGS_BASE: u32 = 0x5011_0000;
pub const PIO0_BASE: u32 = 0x5020_0000;
pub const PIO1_BASE: u32 = 0x5030_0000;
pub const PIO2_BASE: u32 = 0x5040_0000;
pub const XIP_AUX_BASE: u32 = 0x5050_0000;
pub const HSTX_FIFO_BASE: u32 = 0x5060_0000;

// Cortex-M33 Internal Peripherals
pub const SIO_BASE: u32 = 0xD000_0000;
pub const SIO_NONSEC_BASE: u32 = 0xD002_0000;
pub const PPB_BASE: u32 = 0xE000_0000;
pub const PPB_NONSEC_BASE: u32 = 0xE002_0000;
