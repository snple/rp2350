//! GPIO and Pad Control HAL for RP2350
//!
//! This module provides a type-safe interface for GPIO operations using the
//! `paste` crate to generate dedicated types for each GPIO pin.
//!
//! ## Chip Variants
//!
//! - **RP2350A** (default): 30 GPIO pins (0-29) - Use feature `rp2350a`
//! - **RP2350B**: 48 GPIO pins (0-47) - Use feature `rp2350b`
//!
//! ## Example
//!
//! ```ignore
//! use pico_rs::gpio::{GpioBank, Pin25};
//! use pico_rs::gpio::pins::{Uart0TxPin, I2c0SdaPin};
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
//!
//! // Configure pin 0 as UART0 TX (type-safe, compile-time verified)
//! let tx = gpio.pin0();
//! tx.into_uart0_tx();
//!
//! // Configure I2C0 on pins 4, 5 (with automatic pull-up)
//! let sda = gpio.pin4();
//! let scl = gpio.pin5();
//! sda.into_i2c0_sda();
//! scl.into_i2c0_scl();
//!
//! // Configure PWM on pin 0
//! let pwm_pin = gpio.pin0();
//! pwm_pin.into_pwm_a();  // PWM slice 0 channel A
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

/// GPIO interrupt trigger edge/level
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EdgeLevel {
    /// Trigger on low level
    LevelLow,
    /// Trigger on high level
    LevelHigh,
    /// Trigger on falling edge
    EdgeLow,
    /// Trigger on rising edge
    EdgeHigh,
}

// ============================================================================
// Macro for generating GPIO Pin implementations
// ============================================================================

macro_rules! impl_gpio_pin {
    ($PinName:ident, $pin_num:expr, $inte_reg:ident, $intr_reg:ident, $ints_reg:ident) => {
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
                // Interrupt Support
                // ================================================================

                /// Enable interrupt for this pin on the specified edge/level
                ///
                /// This enables the interrupt for the current processor (proc0 by default).
                /// You must also enable IO_IRQ_BANK0 in the NVIC.
                #[inline]
                pub fn enable_interrupt(&self, trigger: EdgeLevel) {
                    match trigger {
                        EdgeLevel::LevelLow => self.io_bank.$inte_reg.[<set_gpio $pin_num _level_low>](),
                        EdgeLevel::LevelHigh => self.io_bank.$inte_reg.[<set_gpio $pin_num _level_high>](),
                        EdgeLevel::EdgeLow => self.io_bank.$inte_reg.[<set_gpio $pin_num _edge_low>](),
                        EdgeLevel::EdgeHigh => self.io_bank.$inte_reg.[<set_gpio $pin_num _edge_high>](),
                    }
                }

                /// Disable interrupt for this pin on the specified edge/level
                #[inline]
                pub fn disable_interrupt(&self, trigger: EdgeLevel) {
                    match trigger {
                        EdgeLevel::LevelLow => self.io_bank.$inte_reg.[<clr_gpio $pin_num _level_low>](),
                        EdgeLevel::LevelHigh => self.io_bank.$inte_reg.[<clr_gpio $pin_num _level_high>](),
                        EdgeLevel::EdgeLow => self.io_bank.$inte_reg.[<clr_gpio $pin_num _edge_low>](),
                        EdgeLevel::EdgeHigh => self.io_bank.$inte_reg.[<clr_gpio $pin_num _edge_high>](),
                    }
                }

                /// Disable all interrupts for this pin
                #[inline]
                pub fn disable_all_interrupts(&self) {
                    self.io_bank.$inte_reg.[<clr_gpio $pin_num _level_low>]();
                    self.io_bank.$inte_reg.[<clr_gpio $pin_num _level_high>]();
                    self.io_bank.$inte_reg.[<clr_gpio $pin_num _edge_low>]();
                    self.io_bank.$inte_reg.[<clr_gpio $pin_num _edge_high>]();
                }

                /// Check if interrupt is enabled for this pin
                #[inline]
                pub fn is_interrupt_enabled(&self, trigger: EdgeLevel) -> bool {
                    match trigger {
                        EdgeLevel::LevelLow => self.io_bank.$inte_reg.[<gpio $pin_num _level_low>](),
                        EdgeLevel::LevelHigh => self.io_bank.$inte_reg.[<gpio $pin_num _level_high>](),
                        EdgeLevel::EdgeLow => self.io_bank.$inte_reg.[<gpio $pin_num _edge_low>](),
                        EdgeLevel::EdgeHigh => self.io_bank.$inte_reg.[<gpio $pin_num _edge_high>](),
                    }
                }

                /// Check if interrupt is pending (raw, before masking)
                #[inline]
                pub fn interrupt_raw(&self, trigger: EdgeLevel) -> bool {
                    match trigger {
                        EdgeLevel::LevelLow => self.io_bank.$intr_reg.[<gpio $pin_num _level_low>](),
                        EdgeLevel::LevelHigh => self.io_bank.$intr_reg.[<gpio $pin_num _level_high>](),
                        EdgeLevel::EdgeLow => self.io_bank.$intr_reg.[<gpio $pin_num _edge_low>](),
                        EdgeLevel::EdgeHigh => self.io_bank.$intr_reg.[<gpio $pin_num _edge_high>](),
                    }
                }

                /// Check if interrupt is pending (after masking)
                #[inline]
                pub fn interrupt_status(&self, trigger: EdgeLevel) -> bool {
                    match trigger {
                        EdgeLevel::LevelLow => self.io_bank.$ints_reg.[<gpio $pin_num _level_low>](),
                        EdgeLevel::LevelHigh => self.io_bank.$ints_reg.[<gpio $pin_num _level_high>](),
                        EdgeLevel::EdgeLow => self.io_bank.$ints_reg.[<gpio $pin_num _edge_low>](),
                        EdgeLevel::EdgeHigh => self.io_bank.$ints_reg.[<gpio $pin_num _edge_high>](),
                    }
                }

                /// Clear edge-triggered interrupt (write 1 to clear)
                ///
                /// Note: Level interrupts cannot be cleared here - they clear
                /// automatically when the pin level changes.
                #[inline]
                pub fn clear_interrupt(&self, trigger: EdgeLevel) {
                    match trigger {
                        // Level interrupts auto-clear when pin level changes
                        EdgeLevel::LevelLow | EdgeLevel::LevelHigh => {},
                        EdgeLevel::EdgeLow => self.io_bank.$intr_reg.[<set_gpio $pin_num _edge_low>](),
                        EdgeLevel::EdgeHigh => self.io_bank.$intr_reg.[<set_gpio $pin_num _edge_high>](),
                    }
                }

                /// Clear all pending edge interrupts for this pin
                #[inline]
                pub fn clear_all_interrupts(&self) {
                    self.io_bank.$intr_reg.[<set_gpio $pin_num _edge_low>]();
                    self.io_bank.$intr_reg.[<set_gpio $pin_num _edge_high>]();
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
    ($($pin_num:literal => $PinName:ident => $getter:ident => [$inte_reg:ident, $intr_reg:ident, $ints_reg:ident]),+ $(,)?) => {
        paste! {
            // Generate all pin types
            $(
                impl_gpio_pin!($PinName, $pin_num, $inte_reg, $intr_reg, $ints_reg);
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
// Generate GPIO pins based on chip variant
// ============================================================================

// RP2350A: 30 GPIO pins (0-29)
#[cfg(feature = "rp2350a")]
impl_gpio_bank!(
    0 => Pin0 => pin0 => [proc0_inte0, intr0, proc0_ints0],
    1 => Pin1 => pin1 => [proc0_inte0, intr0, proc0_ints0],
    2 => Pin2 => pin2 => [proc0_inte0, intr0, proc0_ints0],
    3 => Pin3 => pin3 => [proc0_inte0, intr0, proc0_ints0],
    4 => Pin4 => pin4 => [proc0_inte0, intr0, proc0_ints0],
    5 => Pin5 => pin5 => [proc0_inte0, intr0, proc0_ints0],
    6 => Pin6 => pin6 => [proc0_inte0, intr0, proc0_ints0],
    7 => Pin7 => pin7 => [proc0_inte0, intr0, proc0_ints0],
    8 => Pin8 => pin8 => [proc0_inte1, intr1, proc0_ints1],
    9 => Pin9 => pin9 => [proc0_inte1, intr1, proc0_ints1],
    10 => Pin10 => pin10 => [proc0_inte1, intr1, proc0_ints1],
    11 => Pin11 => pin11 => [proc0_inte1, intr1, proc0_ints1],
    12 => Pin12 => pin12 => [proc0_inte1, intr1, proc0_ints1],
    13 => Pin13 => pin13 => [proc0_inte1, intr1, proc0_ints1],
    14 => Pin14 => pin14 => [proc0_inte1, intr1, proc0_ints1],
    15 => Pin15 => pin15 => [proc0_inte1, intr1, proc0_ints1],
    16 => Pin16 => pin16 => [proc0_inte2, intr2, proc0_ints2],
    17 => Pin17 => pin17 => [proc0_inte2, intr2, proc0_ints2],
    18 => Pin18 => pin18 => [proc0_inte2, intr2, proc0_ints2],
    19 => Pin19 => pin19 => [proc0_inte2, intr2, proc0_ints2],
    20 => Pin20 => pin20 => [proc0_inte2, intr2, proc0_ints2],
    21 => Pin21 => pin21 => [proc0_inte2, intr2, proc0_ints2],
    22 => Pin22 => pin22 => [proc0_inte2, intr2, proc0_ints2],
    23 => Pin23 => pin23 => [proc0_inte2, intr2, proc0_ints2],
    24 => Pin24 => pin24 => [proc0_inte3, intr3, proc0_ints3],
    25 => Pin25 => pin25 => [proc0_inte3, intr3, proc0_ints3],
    26 => Pin26 => pin26 => [proc0_inte3, intr3, proc0_ints3],
    27 => Pin27 => pin27 => [proc0_inte3, intr3, proc0_ints3],
    28 => Pin28 => pin28 => [proc0_inte3, intr3, proc0_ints3],
    29 => Pin29 => pin29 => [proc0_inte3, intr3, proc0_ints3],
);

// RP2350B: 48 GPIO pins (0-47)
#[cfg(feature = "rp2350b")]
impl_gpio_bank!(
    0 => Pin0 => pin0 => [proc0_inte0, intr0, proc0_ints0],
    1 => Pin1 => pin1 => [proc0_inte0, intr0, proc0_ints0],
    2 => Pin2 => pin2 => [proc0_inte0, intr0, proc0_ints0],
    3 => Pin3 => pin3 => [proc0_inte0, intr0, proc0_ints0],
    4 => Pin4 => pin4 => [proc0_inte0, intr0, proc0_ints0],
    5 => Pin5 => pin5 => [proc0_inte0, intr0, proc0_ints0],
    6 => Pin6 => pin6 => [proc0_inte0, intr0, proc0_ints0],
    7 => Pin7 => pin7 => [proc0_inte0, intr0, proc0_ints0],
    8 => Pin8 => pin8 => [proc0_inte1, intr1, proc0_ints1],
    9 => Pin9 => pin9 => [proc0_inte1, intr1, proc0_ints1],
    10 => Pin10 => pin10 => [proc0_inte1, intr1, proc0_ints1],
    11 => Pin11 => pin11 => [proc0_inte1, intr1, proc0_ints1],
    12 => Pin12 => pin12 => [proc0_inte1, intr1, proc0_ints1],
    13 => Pin13 => pin13 => [proc0_inte1, intr1, proc0_ints1],
    14 => Pin14 => pin14 => [proc0_inte1, intr1, proc0_ints1],
    15 => Pin15 => pin15 => [proc0_inte1, intr1, proc0_ints1],
    16 => Pin16 => pin16 => [proc0_inte2, intr2, proc0_ints2],
    17 => Pin17 => pin17 => [proc0_inte2, intr2, proc0_ints2],
    18 => Pin18 => pin18 => [proc0_inte2, intr2, proc0_ints2],
    19 => Pin19 => pin19 => [proc0_inte2, intr2, proc0_ints2],
    20 => Pin20 => pin20 => [proc0_inte2, intr2, proc0_ints2],
    21 => Pin21 => pin21 => [proc0_inte2, intr2, proc0_ints2],
    22 => Pin22 => pin22 => [proc0_inte2, intr2, proc0_ints2],
    23 => Pin23 => pin23 => [proc0_inte2, intr2, proc0_ints2],
    24 => Pin24 => pin24 => [proc0_inte3, intr3, proc0_ints3],
    25 => Pin25 => pin25 => [proc0_inte3, intr3, proc0_ints3],
    26 => Pin26 => pin26 => [proc0_inte3, intr3, proc0_ints3],
    27 => Pin27 => pin27 => [proc0_inte3, intr3, proc0_ints3],
    28 => Pin28 => pin28 => [proc0_inte3, intr3, proc0_ints3],
    29 => Pin29 => pin29 => [proc0_inte3, intr3, proc0_ints3],
    30 => Pin30 => pin30 => [proc0_inte3, intr3, proc0_ints3],
    31 => Pin31 => pin31 => [proc0_inte3, intr3, proc0_ints3],
    32 => Pin32 => pin32 => [proc0_inte4, intr4, proc0_ints4],
    33 => Pin33 => pin33 => [proc0_inte4, intr4, proc0_ints4],
    34 => Pin34 => pin34 => [proc0_inte4, intr4, proc0_ints4],
    35 => Pin35 => pin35 => [proc0_inte4, intr4, proc0_ints4],
    36 => Pin36 => pin36 => [proc0_inte4, intr4, proc0_ints4],
    37 => Pin37 => pin37 => [proc0_inte4, intr4, proc0_ints4],
    38 => Pin38 => pin38 => [proc0_inte4, intr4, proc0_ints4],
    39 => Pin39 => pin39 => [proc0_inte4, intr4, proc0_ints4],
    40 => Pin40 => pin40 => [proc0_inte5, intr5, proc0_ints5],
    41 => Pin41 => pin41 => [proc0_inte5, intr5, proc0_ints5],
    42 => Pin42 => pin42 => [proc0_inte5, intr5, proc0_ints5],
    43 => Pin43 => pin43 => [proc0_inte5, intr5, proc0_ints5],
    44 => Pin44 => pin44 => [proc0_inte5, intr5, proc0_ints5],
    45 => Pin45 => pin45 => [proc0_inte5, intr5, proc0_ints5],
    46 => Pin46 => pin46 => [proc0_inte5, intr5, proc0_ints5],
    47 => Pin47 => pin47 => [proc0_inte5, intr5, proc0_ints5],
);

// ============================================================================
// Pin function mapping submodule
// ============================================================================

pub mod pins;

// Re-export pin function traits for convenience
pub use pins::{
    // I2C traits
    I2c0SclPin,
    I2c0SdaPin,
    I2c1SclPin,
    I2c1SdaPin,
    // PWM traits
    PwmAPin,
    PwmBPin,
    // SPI traits
    Spi0CsnPin,
    Spi0RxPin,
    Spi0SckPin,
    Spi0TxPin,
    Spi1CsnPin,
    Spi1RxPin,
    Spi1SckPin,
    Spi1TxPin,
    // UART traits
    Uart0CtsPin,
    Uart0RtsPin,
    Uart0RxPin,
    Uart0TxPin,
    Uart1CtsPin,
    Uart1RtsPin,
    Uart1RxPin,
    Uart1TxPin,
};
