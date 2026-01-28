//! RESETS peripheral registers
//!
//! Controls reset state of all peripherals

use super::addr::RESETS_BASE;

// Register offsets
pub const RESET: u32 = RESETS_BASE + 0x00;
pub const WDSEL: u32 = RESETS_BASE + 0x04;
pub const RESET_DONE: u32 = RESETS_BASE + 0x08;

// Reset bit definitions (directly from RP2350 datasheet)
pub const ADC: u32 = 1 << 0;
pub const BUSCTRL: u32 = 1 << 1;
pub const DMA: u32 = 1 << 2;
pub const HSTX: u32 = 1 << 3;
pub const I2C0: u32 = 1 << 4;
pub const I2C1: u32 = 1 << 5;
pub const IO_BANK0: u32 = 1 << 6;
pub const IO_QSPI: u32 = 1 << 7;
pub const JTAG: u32 = 1 << 8;
pub const PADS_BANK0: u32 = 1 << 9;
pub const PADS_QSPI: u32 = 1 << 10;
pub const PIO0: u32 = 1 << 11;
pub const PIO1: u32 = 1 << 12;
pub const PIO2: u32 = 1 << 13;
pub const PLL_SYS: u32 = 1 << 14;
pub const PLL_USB: u32 = 1 << 15;
pub const PWM: u32 = 1 << 16;
pub const SHA256: u32 = 1 << 17;
pub const SPI0: u32 = 1 << 18;
pub const SPI1: u32 = 1 << 19;
pub const SYSCFG: u32 = 1 << 20;
pub const SYSINFO: u32 = 1 << 21;
pub const TBMAN: u32 = 1 << 22;
pub const TIMER0: u32 = 1 << 23;
pub const TIMER1: u32 = 1 << 24;
pub const TRNG: u32 = 1 << 25;
pub const UART0: u32 = 1 << 26;
pub const UART1: u32 = 1 << 27;
pub const USBCTRL: u32 = 1 << 28;

/// All peripherals bitmask
pub const ALL: u32 = 0x1FFF_FFFF;

use super::Reg;

/// RESETS register block
pub struct Resets;

impl Resets {
    const RESET: Reg = Reg::new(RESET);
    const RESET_DONE: Reg = Reg::new(RESET_DONE);

    /// Reset specific peripherals (put them into reset state)
    #[inline]
    pub fn reset(peripherals: u32) {
        Self::RESET.set_bits(peripherals);
    }

    /// Unreset specific peripherals (bring them out of reset)
    #[inline]
    pub fn unreset(peripherals: u32) {
        Self::RESET.clear_bits(peripherals);
    }

    /// Unreset peripherals and wait for them to be ready
    #[inline]
    pub fn unreset_wait(peripherals: u32) {
        Self::RESET.clear_bits(peripherals);
        Self::RESET_DONE.wait_set(peripherals);
    }

    /// Check if peripherals are out of reset
    #[inline]
    pub fn is_reset_done(peripherals: u32) -> bool {
        Self::RESET_DONE.is_set(peripherals)
    }
}
