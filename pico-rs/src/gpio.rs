//! GPIO and Pad Control HAL for RP2350
//!
//! This module provides a type-safe interface for GPIO operations using the
//! `paste` crate to generate dedicated types for each GPIO pin.
//!
//! ## Example
//!
//! ```ignore
//! use pico_rs::gpio::{GpioBank, Pin25};
//!
//! // Initialize GPIO subsystem
//! let gpio = GpioBank::init();
//!
//! // Get a specific pin
//! let led = gpio.pin25();
//!
//! // Configure as SIO output
//! led.into_output();
//!
//! // Toggle LED
//! led.set_high();
//! led.set_low();
//! led.toggle();
//! ```

#![allow(clippy::identity_op)]

use paste::paste;

use crate::reset::{self, Peripheral};

// ============================================================================
// Function Select Values (from RP2350 datasheet)
// ============================================================================

/// GPIO function select values
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Function {
    /// JTAG function (pin-specific)
    Jtag = 0,
    /// SPI function (pin-specific)
    Spi = 1,
    /// UART function (pin-specific)
    Uart = 2,
    /// I2C function (pin-specific)
    I2c = 3,
    /// PWM function (pin-specific)
    Pwm = 4,
    /// SIO (Software I/O) - direct CPU control
    Sio = 5,
    /// PIO0
    Pio0 = 6,
    /// PIO1
    Pio1 = 7,
    /// PIO2 (RP2350 only)
    Pio2 = 8,
    /// XIP (pin-specific)
    Xip = 9,
    /// USB function (pin-specific)
    Usb = 10,
    /// Null function (disabled)
    Null = 31,
}

/// GPIO pull resistor configuration
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pull {
    /// No pull resistor
    None,
    /// Pull-up resistor
    Up,
    /// Pull-down resistor
    Down,
}

/// GPIO drive strength
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DriveStrength {
    /// 2mA drive
    _2mA = 0,
    /// 4mA drive
    _4mA = 1,
    /// 8mA drive
    _8mA = 2,
    /// 12mA drive
    _12mA = 3,
}

/// GPIO slew rate
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SlewRate {
    /// Slow slew rate
    Slow,
    /// Fast slew rate
    Fast,
}

/// Output override mode
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OutputOverride {
    /// Use peripheral signal
    Normal = 0,
    /// Invert peripheral signal
    Invert = 1,
    /// Drive low
    Low = 2,
    /// Drive high
    High = 3,
}

/// Output enable override mode
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OutputEnableOverride {
    /// Use peripheral signal
    Normal = 0,
    /// Invert peripheral signal
    Invert = 1,
    /// Disable output
    Disable = 2,
    /// Enable output
    Enable = 3,
}

/// Input override mode
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum InputOverride {
    /// Use peripheral signal
    Normal = 0,
    /// Invert peripheral signal
    Invert = 1,
    /// Force low
    Low = 2,
    /// Force high
    High = 3,
}

/// Interrupt override mode
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IrqOverride {
    /// Use peripheral signal
    Normal = 0,
    /// Invert peripheral signal
    Invert = 1,
    /// Force low
    Low = 2,
    /// Force high
    High = 3,
}

// ============================================================================
// Macro for generating GPIO Pin implementations
// ============================================================================

macro_rules! impl_gpio_pin {
    ($PinName:ident, $pin_num:expr) => {
        paste! {
            /// GPIO Pin $pin_num
            pub struct $PinName<'a> {
                io_bank: &'a pico_rs_pac::io_bank0::RegisterBlock,
                pads_bank: &'a pico_rs_pac::pads_bank0::RegisterBlock,
                sio: &'a pico_rs_pac::sio::RegisterBlock,
            }

            impl<'a> $PinName<'a> {
                /// Pin number
                pub const PIN: u8 = $pin_num;

                /// Pin mask for SIO operations
                pub const MASK: u32 = 1u32 << ($pin_num % 32);

                /// Whether this is a high GPIO (32-47)
                pub const IS_HIGH: bool = $pin_num >= 32;

                // ================================================================
                // Function Selection
                // ================================================================

                /// Set the GPIO function
                #[inline]
                pub fn set_function(&self, func: Function) {
                    unsafe {
                        let ctrl = self.io_bank.gpio_ctrl($pin_num);
                        ctrl.write_funcsel(func as u32);
                    }
                }

                /// Get the current GPIO function
                #[inline]
                pub fn function(&self) -> u32 {
                    unsafe {
                        let ctrl = self.io_bank.gpio_ctrl($pin_num);
                        ctrl.funcsel()
                    }
                }

                /// Set function to SIO (direct CPU control)
                #[inline]
                pub fn set_function_sio(&self) {
                    self.set_function(Function::Sio);
                }

                /// Set function to PIO0
                #[inline]
                pub fn set_function_pio0(&self) {
                    self.set_function(Function::Pio0);
                }

                /// Set function to PIO1
                #[inline]
                pub fn set_function_pio1(&self) {
                    self.set_function(Function::Pio1);
                }

                /// Set function to PIO2
                #[inline]
                pub fn set_function_pio2(&self) {
                    self.set_function(Function::Pio2);
                }

                /// Set function to NULL (disabled)
                #[inline]
                pub fn set_function_null(&self) {
                    self.set_function(Function::Null);
                }

                // ================================================================
                // Pad Configuration
                // ================================================================

                /// Initialize the pad for normal operation
                ///
                /// - Clears pad isolation (critical for RP2350)
                /// - Enables input
                /// - Clears output disable
                #[inline]
                pub fn init_pad(&self) {
                    unsafe {
                        let pad = self.pads_bank.gpio($pin_num);
                        pad.clr_iso();  // Remove pad isolation (RP2350 critical)
                        pad.set_ie();   // Enable input
                        pad.clr_od();   // Clear output disable
                    }
                }

                /// Set pull resistor configuration
                #[inline]
                pub fn set_pull(&self, pull: Pull) {
                    unsafe {
                        let pad = self.pads_bank.gpio($pin_num);
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

                /// Enable pull-up resistor
                #[inline]
                pub fn set_pull_up(&self) {
                    self.set_pull(Pull::Up);
                }

                /// Enable pull-down resistor
                #[inline]
                pub fn set_pull_down(&self) {
                    self.set_pull(Pull::Down);
                }

                /// Disable pull resistor
                #[inline]
                pub fn set_pull_none(&self) {
                    self.set_pull(Pull::None);
                }

                /// Set drive strength
                #[inline]
                pub fn set_drive_strength(&self, strength: DriveStrength) {
                    unsafe {
                        let pad = self.pads_bank.gpio($pin_num);
                        pad.write_drive(strength as u32);
                    }
                }

                /// Set slew rate
                #[inline]
                pub fn set_slew_rate(&self, rate: SlewRate) {
                    unsafe {
                        let pad = self.pads_bank.gpio($pin_num);
                        match rate {
                            SlewRate::Slow => pad.clr_slewfast(),
                            SlewRate::Fast => pad.set_slewfast(),
                        }
                    }
                }

                /// Enable or disable Schmitt trigger
                #[inline]
                pub fn set_schmitt(&self, enable: bool) {
                    unsafe {
                        let pad = self.pads_bank.gpio($pin_num);
                        if enable {
                            pad.set_schmitt();
                        } else {
                            pad.clr_schmitt();
                        }
                    }
                }

                /// Enable input
                #[inline]
                pub fn enable_input(&self) {
                    unsafe {
                        let pad = self.pads_bank.gpio($pin_num);
                        pad.set_ie();
                    }
                }

                /// Disable input
                #[inline]
                pub fn disable_input(&self) {
                    unsafe {
                        let pad = self.pads_bank.gpio($pin_num);
                        pad.clr_ie();
                    }
                }

                /// Set output disable (forces output off regardless of SIO)
                #[inline]
                pub fn set_output_disable(&self, disable: bool) {
                    unsafe {
                        let pad = self.pads_bank.gpio($pin_num);
                        if disable {
                            pad.set_od();
                        } else {
                            pad.clr_od();
                        }
                    }
                }

                // ================================================================
                // GPIO Control Overrides
                // ================================================================

                /// Set output override
                #[inline]
                pub fn set_output_override(&self, ov: OutputOverride) {
                    unsafe {
                        let ctrl = self.io_bank.gpio_ctrl($pin_num);
                        ctrl.write_outover(ov as u32);
                    }
                }

                /// Set output enable override
                #[inline]
                pub fn set_output_enable_override(&self, ov: OutputEnableOverride) {
                    unsafe {
                        let ctrl = self.io_bank.gpio_ctrl($pin_num);
                        ctrl.write_oeover(ov as u32);
                    }
                }

                /// Set input override
                #[inline]
                pub fn set_input_override(&self, ov: InputOverride) {
                    unsafe {
                        let ctrl = self.io_bank.gpio_ctrl($pin_num);
                        ctrl.write_inover(ov as u32);
                    }
                }

                /// Set IRQ override
                #[inline]
                pub fn set_irq_override(&self, ov: IrqOverride) {
                    unsafe {
                        let ctrl = self.io_bank.gpio_ctrl($pin_num);
                        ctrl.write_irqover(ov as u32);
                    }
                }

                // ================================================================
                // SIO Direction Control
                // ================================================================

                /// Set pin direction to output
                #[inline]
                pub fn set_direction_out(&self) {
                    if Self::IS_HIGH {
                        self.sio.gpio_hi_oe_set.write_bits(Self::MASK);
                    } else {
                        self.sio.gpio_oe_set.write_bits(Self::MASK);
                    }
                }

                /// Set pin direction to input
                #[inline]
                pub fn set_direction_in(&self) {
                    if Self::IS_HIGH {
                        self.sio.gpio_hi_oe_clr.write_bits(Self::MASK);
                    } else {
                        self.sio.gpio_oe_clr.write_bits(Self::MASK);
                    }
                }

                /// Check if pin is configured as output
                #[inline]
                pub fn is_output(&self) -> bool {
                    if Self::IS_HIGH {
                        (self.sio.gpio_hi_oe.read_bits() & Self::MASK) != 0
                    } else {
                        (self.sio.gpio_oe.read_bits() & Self::MASK) != 0
                    }
                }

                // ================================================================
                // SIO Input/Output Operations
                // ================================================================

                /// Read the input value
                #[inline]
                pub fn read(&self) -> bool {
                    if Self::IS_HIGH {
                        (self.sio.gpio_hi_in.read_bits() & Self::MASK) != 0
                    } else {
                        (self.sio.gpio_in.read_bits() & Self::MASK) != 0
                    }
                }

                /// Set output high
                #[inline]
                pub fn set_high(&self) {
                    if Self::IS_HIGH {
                        self.sio.gpio_hi_out_set.write_bits(Self::MASK);
                    } else {
                        self.sio.gpio_out_set.write_bits(Self::MASK);
                    }
                }

                /// Set output low
                #[inline]
                pub fn set_low(&self) {
                    if Self::IS_HIGH {
                        self.sio.gpio_hi_out_clr.write_bits(Self::MASK);
                    } else {
                        self.sio.gpio_out_clr.write_bits(Self::MASK);
                    }
                }

                /// Toggle output
                #[inline]
                pub fn toggle(&self) {
                    if Self::IS_HIGH {
                        self.sio.gpio_hi_out_xor.write_bits(Self::MASK);
                    } else {
                        self.sio.gpio_out_xor.write_bits(Self::MASK);
                    }
                }

                /// Set output to a specific value
                #[inline]
                pub fn set(&self, high: bool) {
                    if high {
                        self.set_high();
                    } else {
                        self.set_low();
                    }
                }

                /// Get the current output level (not the actual pin state)
                #[inline]
                pub fn output_level(&self) -> bool {
                    if Self::IS_HIGH {
                        (self.sio.gpio_hi_out.read_bits() & Self::MASK) != 0
                    } else {
                        (self.sio.gpio_out.read_bits() & Self::MASK) != 0
                    }
                }

                // ================================================================
                // Convenience Methods
                // ================================================================

                /// Initialize as SIO output
                ///
                /// - Initializes pad
                /// - Sets function to SIO
                /// - Sets direction to output
                #[inline]
                pub fn into_output(&self) -> &Self {
                    self.init_pad();
                    self.set_function_sio();
                    self.set_direction_out();
                    self
                }

                /// Initialize as SIO input
                ///
                /// - Initializes pad
                /// - Sets function to SIO
                /// - Sets direction to input
                #[inline]
                pub fn into_input(&self) -> &Self {
                    self.init_pad();
                    self.set_function_sio();
                    self.set_direction_in();
                    self
                }

                /// Initialize as SIO input with pull-up
                #[inline]
                pub fn into_input_pullup(&self) -> &Self {
                    self.into_input();
                    self.set_pull_up();
                    self
                }

                /// Initialize as SIO input with pull-down
                #[inline]
                pub fn into_input_pulldown(&self) -> &Self {
                    self.into_input();
                    self.set_pull_down();
                    self
                }

                /// Initialize as PIO0 pin
                #[inline]
                pub fn into_pio0(&self) -> &Self {
                    self.init_pad();
                    self.set_function_pio0();
                    self
                }

                /// Initialize as PIO1 pin
                #[inline]
                pub fn into_pio1(&self) -> &Self {
                    self.init_pad();
                    self.set_function_pio1();
                    self
                }

                /// Initialize as PIO2 pin
                #[inline]
                pub fn into_pio2(&self) -> &Self {
                    self.init_pad();
                    self.set_function_pio2();
                    self
                }
            }
        }
    };
}

// ============================================================================
// Macro for generating GpioBank with all pins
// ============================================================================

macro_rules! impl_gpio_bank {
    ($($pin_num:literal => $PinName:ident => $getter:ident),+ $(,)?) => {
        paste! {
            // Generate all pin types
            $(
                impl_gpio_pin!($PinName, $pin_num);
            )+

            /// GPIO Bank - provides access to all GPIO pins
            ///
            /// This struct holds references to the IO_BANK0, PADS_BANK0, and SIO
            /// peripherals and provides methods to access individual pins.
            pub struct GpioBank {
                io_bank: &'static pico_rs_pac::io_bank0::RegisterBlock,
                pads_bank: &'static pico_rs_pac::pads_bank0::RegisterBlock,
                sio: &'static pico_rs_pac::sio::RegisterBlock,
            }

            impl GpioBank {
                /// Initialize the GPIO bank
                ///
                /// This unresets IO_BANK0 and PADS_BANK0 peripherals.
                pub fn init() -> Self {
                    // Unreset IO_BANK0 and PADS_BANK0
                    reset::unreset_block_wait(Peripheral::IO_BANK0);
                    reset::unreset_block_wait(Peripheral::PADS_BANK0);

                    Self {
                        io_bank: unsafe { &*pico_rs_pac::IO_BANK0::PTR },
                        pads_bank: unsafe { &*pico_rs_pac::PADS_BANK0::PTR },
                        sio: unsafe { &*pico_rs_pac::SIO::PTR },
                    }
                }

                /// Create a new GpioBank without initializing peripherals
                ///
                /// # Safety
                /// The caller must ensure that IO_BANK0 and PADS_BANK0 have been unreset.
                pub unsafe fn new_unchecked() -> Self {
                    unsafe {
                        Self {
                            io_bank: &*pico_rs_pac::IO_BANK0::PTR,
                            pads_bank: &*pico_rs_pac::PADS_BANK0::PTR,
                            sio: &*pico_rs_pac::SIO::PTR,
                        }
                    }
                }

                // Generate getter methods for each pin
                $(
                    #[doc = concat!("Get GPIO pin ", stringify!($pin_num))]
                    #[inline]
                    pub fn $getter(&self) -> $PinName<'_> {
                        $PinName {
                            io_bank: self.io_bank,
                            pads_bank: self.pads_bank,
                            sio: self.sio,
                        }
                    }
                )+
            }
        }
    };
}

// ============================================================================
// Generate all 48 GPIO pins
// ============================================================================

impl_gpio_bank!(
    0 => Pin0 => pin0,
    1 => Pin1 => pin1,
    2 => Pin2 => pin2,
    3 => Pin3 => pin3,
    4 => Pin4 => pin4,
    5 => Pin5 => pin5,
    6 => Pin6 => pin6,
    7 => Pin7 => pin7,
    8 => Pin8 => pin8,
    9 => Pin9 => pin9,
    10 => Pin10 => pin10,
    11 => Pin11 => pin11,
    12 => Pin12 => pin12,
    13 => Pin13 => pin13,
    14 => Pin14 => pin14,
    15 => Pin15 => pin15,
    16 => Pin16 => pin16,
    17 => Pin17 => pin17,
    18 => Pin18 => pin18,
    19 => Pin19 => pin19,
    20 => Pin20 => pin20,
    21 => Pin21 => pin21,
    22 => Pin22 => pin22,
    23 => Pin23 => pin23,
    24 => Pin24 => pin24,
    25 => Pin25 => pin25,
    26 => Pin26 => pin26,
    27 => Pin27 => pin27,
    28 => Pin28 => pin28,
    29 => Pin29 => pin29,
    30 => Pin30 => pin30,
    31 => Pin31 => pin31,
    32 => Pin32 => pin32,
    33 => Pin33 => pin33,
    34 => Pin34 => pin34,
    35 => Pin35 => pin35,
    36 => Pin36 => pin36,
    37 => Pin37 => pin37,
    38 => Pin38 => pin38,
    39 => Pin39 => pin39,
    40 => Pin40 => pin40,
    41 => Pin41 => pin41,
    42 => Pin42 => pin42,
    43 => Pin43 => pin43,
    44 => Pin44 => pin44,
    45 => Pin45 => pin45,
    46 => Pin46 => pin46,
    47 => Pin47 => pin47,
);
