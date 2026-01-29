//! High-level GPIO driver for RP2350
//!
//! This module provides a simple, safe interface for GPIO operations using the
//! pico-rs-pac peripheral access crate.
//!
//! ## Example
//!
//! ```ignore
//! use pico_rs::gpio::{Gpio, init};
//!
//! // Initialize GPIO subsystem (unresets IO_BANK0 and PADS_BANK0)
//! init();
//!
//! // Configure GPIO 25 (LED) as output
//! let led = Gpio::new(25);
//! led.init();
//! led.into_output();
//!
//! // Toggle LED
//! led.toggle();
//! ```

use crate::reset::{self, Peripheral};
use pico_rs_pac::{IO_BANK0, PADS_BANK0, SIO};

// Function select values
const FUNCSEL_SIO: u32 = 5; // GPIO
const FUNCSEL_PIO0: u32 = 6; // PIO0
const FUNCSEL_PIO1: u32 = 7; // PIO1
const FUNCSEL_PIO2: u32 = 8; // PIO2 (RP2350 only)

/// GPIO pin direction
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Direction {
    Input,
    Output,
}

/// GPIO pull configuration
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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
        let io_bank = unsafe { &*IO_BANK0::PTR };
        let pads = unsafe { &*PADS_BANK0::PTR };

        unsafe {
            let pad = pads.gpio(self.pin);

            // Configure pad: enable input, disable output disable
            pad.set_ie();
            pad.clr_od();

            // Clear pad isolation (RP2350 specific)
            // This is critical - the pad is isolated by default after reset
            pad.clr_iso();
        }

        // Set to SIO function via IO_BANK0 GPIO control register
        unsafe {
            let ctrl = io_bank.gpio_ctrl(self.pin);
            ctrl.write_funcsel(FUNCSEL_SIO);
        }
    }

    /// Configure the GPIO as output
    pub fn into_output(&self) -> &Self {
        let sio = unsafe { &*SIO::PTR };

        // Enable output in SIO
        if self.pin < 32 {
            sio.gpio_oe_set.write_bits(self.mask());
        } else {
            sio.gpio_hi_oe_set.write_bits(self.mask());
        }

        self
    }

    /// Configure the GPIO as input
    pub fn into_input(&self) -> &Self {
        let sio = unsafe { &*SIO::PTR };

        // Disable output in SIO
        if self.pin < 32 {
            sio.gpio_oe_clr.write_bits(self.mask());
        } else {
            sio.gpio_hi_oe_clr.write_bits(self.mask());
        }

        self
    }

    /// Set pull resistor configuration
    pub fn set_pull(&self, pull: Pull) {
        let pads = unsafe { &*PADS_BANK0::PTR };
        unsafe {
            let pad = pads.gpio(self.pin);
            match pull {
                Pull::None => {
                    pad.clr_pue();
                    pad.clr_pde();
                }
                Pull::Up => {
                    pad.set_pue();
                    pad.clr_pde();
                }
                Pull::Down => {
                    pad.clr_pue();
                    pad.set_pde();
                }
            }
        }
    }

    /// Read the input value
    #[inline]
    pub fn read(&self) -> bool {
        let sio = unsafe { &*SIO::PTR };
        if self.pin < 32 {
            (sio.gpio_in.read_bits() & self.mask()) != 0
        } else {
            (sio.gpio_hi_in.read_bits() & self.mask()) != 0
        }
    }

    /// Set the output high
    #[inline]
    pub fn set_high(&self) {
        let sio = unsafe { &*SIO::PTR };
        if self.pin < 32 {
            sio.gpio_out_set.write_bits(self.mask());
        } else {
            sio.gpio_hi_out_set.write_bits(self.mask());
        }
    }

    /// Set the output low
    #[inline]
    pub fn set_low(&self) {
        let sio = unsafe { &*SIO::PTR };
        if self.pin < 32 {
            sio.gpio_out_clr.write_bits(self.mask());
        } else {
            sio.gpio_hi_out_clr.write_bits(self.mask());
        }
    }

    /// Toggle the output
    #[inline]
    pub fn toggle(&self) {
        let sio = unsafe { &*SIO::PTR };
        if self.pin < 32 {
            sio.gpio_out_xor.write_bits(self.mask());
        } else {
            sio.gpio_hi_out_xor.write_bits(self.mask());
        }
    }

    /// Set the output to the given level
    #[inline]
    pub fn set(&self, high: bool) {
        if high {
            self.set_high();
        } else {
            self.set_low();
        }
    }

    /// Check if output is currently enabled
    #[inline]
    pub fn is_output(&self) -> bool {
        let sio = unsafe { &*SIO::PTR };
        if self.pin < 32 {
            (sio.gpio_oe.read_bits() & self.mask()) != 0
        } else {
            (sio.gpio_hi_oe.read_bits() & self.mask()) != 0
        }
    }

    /// Get the current output level (not the actual pin state)
    #[inline]
    pub fn output_level(&self) -> bool {
        let sio = unsafe { &*SIO::PTR };
        if self.pin < 32 {
            (sio.gpio_out.read_bits() & self.mask()) != 0
        } else {
            (sio.gpio_hi_out.read_bits() & self.mask()) != 0
        }
    }

    /// Set the GPIO function to PIO0
    pub fn set_function_pio0(&self) {
        let io_bank = unsafe { &*IO_BANK0::PTR };
        let pads = unsafe { &*PADS_BANK0::PTR };

        unsafe {
            let pad = pads.gpio(self.pin);
            pad.set_ie();
            pad.clr_od();
            pad.clr_iso();

            let ctrl = io_bank.gpio_ctrl(self.pin);
            ctrl.write_funcsel(FUNCSEL_PIO0);
        }
    }

    /// Set the GPIO function to PIO1
    pub fn set_function_pio1(&self) {
        let io_bank = unsafe { &*IO_BANK0::PTR };
        let pads = unsafe { &*PADS_BANK0::PTR };

        unsafe {
            let pad = pads.gpio(self.pin);
            pad.set_ie();
            pad.clr_od();
            pad.clr_iso();

            let ctrl = io_bank.gpio_ctrl(self.pin);
            ctrl.write_funcsel(FUNCSEL_PIO1);
        }
    }

    /// Set the GPIO function to PIO2
    pub fn set_function_pio2(&self) {
        let io_bank = unsafe { &*IO_BANK0::PTR };
        let pads = unsafe { &*PADS_BANK0::PTR };

        unsafe {
            let pad = pads.gpio(self.pin);
            pad.set_ie();
            pad.clr_od();
            pad.clr_iso();

            let ctrl = io_bank.gpio_ctrl(self.pin);
            ctrl.write_funcsel(FUNCSEL_PIO2);
        }
    }
}

/// Initialize GPIO subsystem
///
/// This function unresets IO_BANK0 and PADS_BANK0 peripherals.
/// Must be called before using any GPIO functions.
pub fn init() {
    reset::unreset_block_wait_mask(Peripheral::IO_BANK0 | Peripheral::PADS_BANK0);
}
