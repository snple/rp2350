//! PADS_BANK0 peripheral registers
//!
//! Controls electrical characteristics of GPIO pads

use super::addr::PADS_BANK0_BASE;
use super::Reg;

// Each GPIO pad has one register at offset 4 + gpio * 4
// (offset 0 is voltage select)
fn gpio_pad(gpio: u8) -> u32 {
    PADS_BANK0_BASE + 4 + (gpio as u32) * 4
}

// Voltage select register
pub const VOLTAGE_SELECT: u32 = PADS_BANK0_BASE;

// Pad register bits
pub const PAD_OD: u32 = 1 << 7;        // Output disable
pub const PAD_IE: u32 = 1 << 6;        // Input enable
pub const PAD_DRIVE_MASK: u32 = 0x30;  // Drive strength (bits 5:4)
pub const PAD_DRIVE_2MA: u32 = 0 << 4;
pub const PAD_DRIVE_4MA: u32 = 1 << 4;
pub const PAD_DRIVE_8MA: u32 = 2 << 4;
pub const PAD_DRIVE_12MA: u32 = 3 << 4;
pub const PAD_PUE: u32 = 1 << 3;       // Pull-up enable
pub const PAD_PDE: u32 = 1 << 2;       // Pull-down enable
pub const PAD_SCHMITT: u32 = 1 << 1;   // Schmitt trigger
pub const PAD_SLEWFAST: u32 = 1 << 0;  // Slew rate

// Default pad configuration (from RP2350 datasheet)
// IE=1, DRIVE=4mA, SCHMITT=1 (other bits 0)
pub const PAD_DEFAULT: u32 = PAD_IE | PAD_DRIVE_4MA | PAD_SCHMITT;

// Voltage select values
pub const VOLTAGE_3V3: u32 = 0;
pub const VOLTAGE_1V8: u32 = 1;

/// PADS Bank 0 control
pub struct PadsBank0;

impl PadsBank0 {
    /// Set the pad configuration for a GPIO
    #[inline]
    pub fn set_pad(gpio: u8, config: u32) {
        debug_assert!(gpio < 48);
        Reg::new(gpio_pad(gpio)).write(config);
    }

    /// Modify the pad configuration for a GPIO
    #[inline]
    pub fn modify_pad<F: FnOnce(u32) -> u32>(gpio: u8, f: F) {
        debug_assert!(gpio < 48);
        Reg::new(gpio_pad(gpio)).modify(f);
    }

    /// Get the pad configuration for a GPIO
    #[inline]
    pub fn get_pad(gpio: u8) -> u32 {
        debug_assert!(gpio < 48);
        Reg::new(gpio_pad(gpio)).read()
    }

    /// Enable input for a GPIO pad
    #[inline]
    pub fn enable_input(gpio: u8) {
        Reg::new(gpio_pad(gpio)).set_bits(PAD_IE);
    }

    /// Disable input for a GPIO pad
    #[inline]
    pub fn disable_input(gpio: u8) {
        Reg::new(gpio_pad(gpio)).clear_bits(PAD_IE);
    }

    /// Enable output for a GPIO pad
    #[inline]
    pub fn enable_output(gpio: u8) {
        Reg::new(gpio_pad(gpio)).clear_bits(PAD_OD);
    }

    /// Disable output for a GPIO pad
    #[inline]
    pub fn disable_output(gpio: u8) {
        Reg::new(gpio_pad(gpio)).set_bits(PAD_OD);
    }

    /// Enable pull-up resistor
    #[inline]
    pub fn enable_pullup(gpio: u8) {
        Self::modify_pad(gpio, |v| (v & !PAD_PDE) | PAD_PUE);
    }

    /// Enable pull-down resistor
    #[inline]
    pub fn enable_pulldown(gpio: u8) {
        Self::modify_pad(gpio, |v| (v & !PAD_PUE) | PAD_PDE);
    }

    /// Disable pull resistors
    #[inline]
    pub fn disable_pulls(gpio: u8) {
        Reg::new(gpio_pad(gpio)).clear_bits(PAD_PUE | PAD_PDE);
    }

    /// Set drive strength
    #[inline]
    pub fn set_drive(gpio: u8, drive: u32) {
        Self::modify_pad(gpio, |v| (v & !PAD_DRIVE_MASK) | (drive & PAD_DRIVE_MASK));
    }

    /// Set I/O voltage (affects all pads)
    #[inline]
    pub fn set_voltage(voltage: u32) {
        Reg::new(VOLTAGE_SELECT).write(voltage);
    }
}
