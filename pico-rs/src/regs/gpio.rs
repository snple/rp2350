//! High-level GPIO driver

use super::io_bank::{FUNCSEL_SIO, IoBank0};
use super::pads::PadsBank0;
use super::sio::Sio;

/// GPIO pin direction
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Input,
    Output,
}

/// GPIO pull configuration
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Pull {
    None,
    Up,
    Down,
}

/// A GPIO pin
pub struct Gpio {
    pin: u8,
}

impl Gpio {
    /// Create a new GPIO reference
    ///
    /// Note: This does not initialize the GPIO. Call `init()` first.
    #[inline]
    pub const fn new(pin: u8) -> Self {
        debug_assert!(pin < 48);
        Self { pin }
    }

    /// Get the pin number
    #[inline]
    pub const fn pin(&self) -> u8 {
        self.pin
    }

    /// Get the pin mask (for SIO operations)
    #[inline]
    pub const fn mask(&self) -> u32 {
        1 << (self.pin % 32)
    }

    /// Initialize the GPIO pin for SIO function
    pub fn init(&self) {
        // Set to SIO function
        IoBank0::set_function(self.pin, FUNCSEL_SIO);
    }

    /// Configure the GPIO as output
    pub fn into_output(&self) -> &Self {
        // Make sure pad output is enabled
        PadsBank0::modify_pad(self.pin, |v| v & !super::pads::PAD_OD);

        // Enable output in SIO
        if self.pin < 32 {
            Sio::gpio_oe_set(self.mask());
        } else {
            // For GPIO 32-47, use high registers
            use super::sio::GPIO_HI_OE_SET;
            super::Reg::new(GPIO_HI_OE_SET).write(self.mask());
        }

        self
    }

    /// Configure the GPIO as input
    pub fn into_input(&self) -> &Self {
        // Disable output in SIO
        if self.pin < 32 {
            Sio::gpio_oe_clr(self.mask());
        } else {
            use super::sio::GPIO_HI_OE_CLR;
            super::Reg::new(GPIO_HI_OE_CLR).write(self.mask());
        }

        // Make sure pad input is enabled
        PadsBank0::modify_pad(self.pin, |v| v | super::pads::PAD_IE);

        self
    }

    /// Set pull resistor configuration
    pub fn set_pull(&self, pull: Pull) {
        match pull {
            Pull::None => PadsBank0::disable_pulls(self.pin),
            Pull::Up => PadsBank0::enable_pullup(self.pin),
            Pull::Down => PadsBank0::enable_pulldown(self.pin),
        }
    }

    /// Read the input value
    #[inline]
    pub fn read(&self) -> bool {
        Sio::gpio_read(self.pin)
    }

    /// Set the output high
    #[inline]
    pub fn set_high(&self) {
        if self.pin < 32 {
            Sio::gpio_out_set(self.mask());
        } else {
            use super::sio::GPIO_HI_OUT_SET;
            super::Reg::new(GPIO_HI_OUT_SET).write(self.mask());
        }
    }

    /// Set the output low
    #[inline]
    pub fn set_low(&self) {
        if self.pin < 32 {
            Sio::gpio_out_clr(self.mask());
        } else {
            use super::sio::GPIO_HI_OUT_CLR;
            super::Reg::new(GPIO_HI_OUT_CLR).write(self.mask());
        }
    }

    /// Toggle the output
    #[inline]
    pub fn toggle(&self) {
        if self.pin < 32 {
            Sio::gpio_out_xor(self.mask());
        } else {
            use super::sio::GPIO_HI_OUT_XOR;
            super::Reg::new(GPIO_HI_OUT_XOR).write(self.mask());
        }
    }

    /// Set the output value
    #[inline]
    pub fn set(&self, value: bool) {
        if value {
            self.set_high();
        } else {
            self.set_low();
        }
    }

    /// Check if output is high (reads SIO output register, not pad)
    #[inline]
    pub fn is_set_high(&self) -> bool {
        if self.pin < 32 {
            (super::Reg::new(super::sio::GPIO_OUT).read() & self.mask()) != 0
        } else {
            (super::Reg::new(super::sio::GPIO_HI_OUT).read() & self.mask()) != 0
        }
    }
}

/// Initialize GPIO subsystem (call once at startup)
pub fn init() {
    use super::resets;

    // Reset IO_BANK0 and PADS_BANK0
    resets::Resets::unreset_wait(resets::IO_BANK0 | resets::PADS_BANK0);
}
