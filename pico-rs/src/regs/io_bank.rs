//! IO_BANK0 peripheral registers
//!
//! Controls GPIO pin function selection and interrupt configuration

use super::addr::IO_BANK0_BASE;
use super::Reg;

// Each GPIO has 2 registers: STATUS and CTRL
// STATUS at offset 0x000 + gpio * 8
// CTRL at offset 0x004 + gpio * 8
fn gpio_status(gpio: u8) -> u32 {
    IO_BANK0_BASE + (gpio as u32) * 8
}

fn gpio_ctrl(gpio: u8) -> u32 {
    IO_BANK0_BASE + (gpio as u32) * 8 + 4
}

// CTRL register function selection (FUNCSEL field, bits 4:0)
pub const FUNCSEL_MASK: u32 = 0x1F;
pub const FUNCSEL_XIP: u32 = 0;
pub const FUNCSEL_SPI: u32 = 1;
pub const FUNCSEL_UART: u32 = 2;
pub const FUNCSEL_I2C: u32 = 3;
pub const FUNCSEL_PWM: u32 = 4;
pub const FUNCSEL_SIO: u32 = 5;
pub const FUNCSEL_PIO0: u32 = 6;
pub const FUNCSEL_PIO1: u32 = 7;
pub const FUNCSEL_PIO2: u32 = 8;
pub const FUNCSEL_USB: u32 = 10;
pub const FUNCSEL_UART_AUX: u32 = 11;
pub const FUNCSEL_NULL: u32 = 31;

// CTRL register other fields
pub const CTRL_OUTOVER_SHIFT: u32 = 12;
pub const CTRL_OEOVER_SHIFT: u32 = 14;
pub const CTRL_INOVER_SHIFT: u32 = 16;
pub const CTRL_IRQOVER_SHIFT: u32 = 28;

// Override values
pub const OVER_NORMAL: u32 = 0;
pub const OVER_INVERT: u32 = 1;
pub const OVER_LOW: u32 = 2;
pub const OVER_HIGH: u32 = 3;

// STATUS register fields
pub const STATUS_OUTFROMPERI: u32 = 1 << 8;
pub const STATUS_OUTTOPAD: u32 = 1 << 9;
pub const STATUS_OEFROMPERI: u32 = 1 << 12;
pub const STATUS_OETOPAD: u32 = 1 << 13;
pub const STATUS_INFROMPAD: u32 = 1 << 17;
pub const STATUS_INTOPERI: u32 = 1 << 19;
pub const STATUS_IRQFROMPAD: u32 = 1 << 24;
pub const STATUS_IRQTOPROC: u32 = 1 << 26;

/// IO Bank 0 GPIO control
pub struct IoBank0;

impl IoBank0 {
    /// Set the function for a GPIO pin
    #[inline]
    pub fn set_function(gpio: u8, func: u32) {
        debug_assert!(gpio < 48);
        Reg::new(gpio_ctrl(gpio)).modify(|v| (v & !FUNCSEL_MASK) | (func & FUNCSEL_MASK));
    }

    /// Get the current function for a GPIO pin
    #[inline]
    pub fn get_function(gpio: u8) -> u32 {
        debug_assert!(gpio < 48);
        Reg::new(gpio_ctrl(gpio)).read() & FUNCSEL_MASK
    }

    /// Set GPIO to SIO function (standard GPIO)
    #[inline]
    pub fn set_sio(gpio: u8) {
        Self::set_function(gpio, FUNCSEL_SIO);
    }

    /// Disable GPIO function (set to NULL)
    #[inline]
    pub fn disable(gpio: u8) {
        Self::set_function(gpio, FUNCSEL_NULL);
    }

    /// Read the GPIO status register
    #[inline]
    pub fn status(gpio: u8) -> u32 {
        debug_assert!(gpio < 48);
        Reg::new(gpio_status(gpio)).read()
    }

    /// Read the input value from the pad
    #[inline]
    pub fn read_input(gpio: u8) -> bool {
        (Self::status(gpio) & STATUS_INFROMPAD) != 0
    }
}
