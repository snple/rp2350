//! GPIO (General Purpose Input/Output) HAL for RP2350
//!
//! This module provides a type-safe interface for configuring and using GPIO pins
//! on the RP2350 microcontroller. It leverages Rust's ownership system to prevent
//! common GPIO programming errors at compile time.
//!
//! # Architecture Overview
//!
//! The RP2350 GPIO system consists of three main hardware blocks:
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────────────────────┐
//! │                          GPIO Hardware Architecture                          │
//! ├─────────────────────────────────────────────────────────────────────────────┤
//! │                                                                             │
//! │  ┌─────────────────────────────────────────────────────────────────────┐   │
//! │  │                    IO_BANK0 (Function Control)                       │   │
//! │  │                                                                      │   │
//! │  │  • Function selection (funcsel): SIO, UART, SPI, I2C, PWM, PIO, etc │   │
//! │  │  • Output/Input/OE overrides for signal manipulation                 │   │
//! │  │  • Interrupt configuration and status                                │   │
//! │  │                                                                      │   │
//! │  │  Registers per pin: GPIOx_STATUS, GPIOx_CTRL                        │   │
//! │  └─────────────────────────────────────────────────────────────────────┘   │
//! │                                                                             │
//! │  ┌─────────────────────────────────────────────────────────────────────┐   │
//! │  │                 PADS_BANK0 (Electrical Configuration)                │   │
//! │  │                                                                      │   │
//! │  │  • Drive strength: 2mA, 4mA, 8mA, 12mA                              │   │
//! │  │  • Pull resistors: None, Up, Down                                   │   │
//! │  │  • Slew rate: Slow, Fast                                            │   │
//! │  │  • Schmitt trigger enable                                           │   │
//! │  │  • Input enable (IE) and Output disable (OD)                        │   │
//! │  │  • Pad isolation (ISO) for low-power modes                          │   │
//! │  │                                                                      │   │
//! │  │  Registers per pin: GPIOx                                           │   │
//! │  └─────────────────────────────────────────────────────────────────────┘   │
//! │                                                                             │
//! │  ┌─────────────────────────────────────────────────────────────────────┐   │
//! │  │                    SIO (Single-cycle I/O)                            │   │
//! │  │                                                                      │   │
//! │  │  • Direct CPU-controlled GPIO when funcsel = SIO                    │   │
//! │  │  • Single-cycle read/write operations                               │   │
//! │  │  • Atomic SET/CLR/XOR registers for safe bit manipulation           │   │
//! │  │                                                                      │   │
//! │  │  Registers: GPIO_IN, GPIO_OUT, GPIO_OE (+ SET/CLR/XOR variants)    │   │
//! │  │            GPIO_HI_* for pins 32-47 (RP2350B only)                  │   │
//! │  └─────────────────────────────────────────────────────────────────────┘   │
//! │                                                                             │
//! └─────────────────────────────────────────────────────────────────────────────┘
//! ```
//!
//! # Pin Count by Variant
//!
//! | Variant  | GPIO Pins | Package  | Notes                    |
//! |----------|-----------|----------|--------------------------|
//! | RP2350A  | 30 (0-29) | QFN-60   | Raspberry Pi Pico 2      |
//! | RP2350B  | 48 (0-47) | QFN-80   | More pins, same core     |
//!
//! # Ownership Model
//!
//! This module uses Rust's ownership system to ensure GPIO pins cannot be
//! misconfigured or used by multiple drivers simultaneously:
//!
//! 1. **`Pins` struct**: Created by consuming `IO_BANK0` and `PADS_BANK0`
//!    peripherals. This initializes the GPIO subsystem.
//!
//! 2. **`Pin<N>` struct**: Obtained via [`Pins::pin()`]. Each GPIO pin is
//!    represented by a unique type. Pins can be consumed by peripheral drivers.
//!
//! 3. **Peripheral ownership**: Drivers like I2C or SPI can consume pins,
//!    making them unavailable for other uses (enforced at compile time).
//!
//! ```text
//! Peripherals::take()
//!       │
//!       ├── IO_BANK0 ──┐
//!       │              │
//!       ├── PADS_BANK0 ┼──► Pins::new() ──► pins.pin::<N>()
//!       │              │                          │
//!       ├── I2C0 ──────┼──────────────────────────┼──► I2c0::new(i2c0, pin4, pin5, ...)
//!       │              │                          │         ▲
//!       └── ...        │                          │    pin4, pin5 consumed!
//!                      │                          │
//!                      │                          └──► let led = pins.pin::<25>();
//!                      │                                led.into_output();
//!                      │                                led.set_high();
//! ```
//!
//! # Function Selection
//!
//! Each GPIO pin can be assigned to different peripheral functions:
//!
//! | funcsel | Function | Description                              |
//! |---------|----------|------------------------------------------|
//! | 0       | JTAG     | Debug interface                          |
//! | 1       | SPI      | Serial Peripheral Interface              |
//! | 2       | UART     | Serial communication                     |
//! | 3       | I2C      | Inter-Integrated Circuit                 |
//! | 4       | PWM      | Pulse Width Modulation                   |
//! | 5       | SIO      | Software I/O (direct CPU control)        |
//! | 6       | PIO0     | Programmable I/O block 0                 |
//! | 7       | PIO1     | Programmable I/O block 1                 |
//! | 8       | PIO2     | Programmable I/O block 2                 |
//! | 9       | XIP      | Execute-in-place (QSPI)                  |
//! | 10      | USB      | USB controller                           |
//! | 31      | NULL     | Disabled (high impedance)                |
//!
//! # Usage Examples
//!
//! ## Basic LED Blink (Output)
//!
//! ```ignore
//! use pico_rs::gpio::Pins;
//! use pico_rs::Peripherals;
//!
//! // Take device peripherals (singleton)
//! let dp = Peripherals::take().unwrap();
//!
//! // Create GPIO controller by consuming IO_BANK0 and PADS_BANK0
//! let pins = Pins::new(dp.IO_BANK0, dp.PADS_BANK0);
//!
//! // Configure pin 25 (on-board LED) as output
//! let led = pins.pin::<25>();
//! led.into_output();
//!
//! loop {
//!     led.toggle();
//!     delay_ms(500);
//! }
//! ```
//!
//! ## Button Input with Pull-up
//!
//! ```ignore
//! use pico_rs::gpio::Pins;
//!
//! let dp = Peripherals::take().unwrap();
//! let pins = Pins::new(dp.IO_BANK0, dp.PADS_BANK0);
//!
//! // Configure pin 14 as input with internal pull-up
//! let button = pins.pin::<14>();
//! button.into_input_pullup();
//!
//! // Button is active-low (pressed = low)
//! if !button.read() {
//!     // Button is pressed
//! }
//! ```
//!
//! ## I2C with Pin Ownership Transfer
//!
//! ```ignore
//! use pico_rs::gpio::Pins;
//! use pico_rs::i2c::I2c0;
//!
//! let dp = Peripherals::take().unwrap();
//! let pins = Pins::new(dp.IO_BANK0, dp.PADS_BANK0);
//!
//! // Initialize I2C, consuming the SDA and SCL pins
//! let i2c = I2c0::new(
//!     dp.I2C0,
//!     pins.pin::<4>(),  // SDA - ownership transferred
//!     pins.pin::<5>(),  // SCL - ownership transferred
//!     400_000,
//!     sys_clk,
//! );
//! ```
//!
//! ## PWM Output
//!
//! ```ignore
//! let pins = Pins::new(dp.IO_BANK0, dp.PADS_BANK0);
//!
//! // Configure pin for PWM function
//! let pwm_pin = pins.pin::<16>();
//! pwm_pin.into_pwm();
//!
//! // Now configure the PWM peripheral...
//! ```
//!
//! ## PIO for Custom Protocols
//!
//! ```ignore
//! let pins = Pins::new(dp.IO_BANK0, dp.PADS_BANK0);
//!
//! // Configure pins for PIO0
//! pins.pin::<0>().into_pio0();
//! pins.pin::<1>().into_pio0();
//!
//! // Load and run PIO program...
//! ```
//!
//! ## Interrupt Handling (ISR)
//!
//! In ISR context, `Pin<N>` can be created via `unsafe { Pin::<N>::new() }`
//! without ownership, since the GPIO subsystem was already initialised:
//!
//! ```ignore
//! use pico_rs::gpio::{Pin, EdgeLevel};
//!
//! #[unsafe(no_mangle)]
//! extern "C" fn IO_IRQ_BANK0() {
//!     let button = unsafe { Pin::<15>::new() };
//!     if button.check_and_clear(EdgeLevel::EdgeLow) {
//!         // Handle button press...
//!     }
//! }
//! ```
//!
//! # SIO (Single-cycle I/O) Details
//!
//! When a pin's function is set to SIO, it can be directly controlled by the CPU:
//!
//! - **GPIO_OUT**: Output level register (read/write)
//! - **GPIO_OUT_SET**: Atomic set (write 1 to set bits high)
//! - **GPIO_OUT_CLR**: Atomic clear (write 1 to set bits low)
//! - **GPIO_OUT_XOR**: Atomic toggle (write 1 to invert bits)
//! - **GPIO_OE**: Output enable (direction) register
//! - **GPIO_IN**: Input value (read-only)
//!
//! The SET/CLR/XOR registers allow atomic bit manipulation without
//! read-modify-write cycles, which is essential for interrupt safety.
//!
//! # Safety Considerations
//!
//! - Pin ownership prevents accidental conflicts at compile time
//! - Internal register access uses `steal()` for zero-cost abstraction
//! - SIO uses atomic registers, safe for use from interrupts
//! - Each pin type (`Pin<N>`) has compile-time known register offsets

#![allow(clippy::identity_op)]

use paste::paste;
use pico_rs_pac::sio::RegisterBlock as SioRegisterBlock;
use pico_rs_pac::{IO_BANK0, PADS_BANK0};

use crate::reset::{self, Peripheral};

// ============================================================================
// Function Select Values (from RP2350 datasheet Table 294)
// ============================================================================

/// GPIO function selection values.
///
/// Each GPIO pin can be connected to one of several peripheral functions.
/// The function is selected by writing to the `funcsel` field of the
/// pin's control register (`GPIOx_CTRL`).
///
/// # Example
///
/// ```ignore
/// // Configure pin for UART function
/// pin.set_function(Function::Uart);
///
/// // Or use convenience methods
/// pin.into_uart();
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Function {
    /// JTAG debug interface (pin-specific)
    Jtag = 0,
    /// SPI peripheral (pin-specific for RX/TX/SCK/CS)
    Spi = 1,
    /// UART peripheral (pin-specific for TX/RX/CTS/RTS)
    Uart = 2,
    /// I2C peripheral (pin-specific for SDA/SCL)
    I2c = 3,
    /// PWM output (pin determines slice and channel)
    Pwm = 4,
    /// SIO - Software I/O, direct CPU control
    Sio = 5,
    /// PIO0 - Programmable I/O block 0
    Pio0 = 6,
    /// PIO1 - Programmable I/O block 1
    Pio1 = 7,
    /// PIO2 - Programmable I/O block 2 (RP2350 only)
    Pio2 = 8,
    /// XIP - Execute-in-place for QSPI (pin-specific)
    Xip = 9,
    /// USB controller (pin-specific)
    Usb = 10,
    /// NULL - Disabled, pin is high-impedance
    Null = 31,
}

/// Pull resistor configuration.
///
/// Internal pull resistors can be used to define a default state
/// when a pin is not actively driven.
///
/// # Example
///
/// ```ignore
/// // Enable pull-up for a button (active-low)
/// button.set_pull(Pull::Up);
///
/// // Or use convenience method
/// button.into_input_pullup();
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pull {
    /// No pull resistor (floating input)
    None,
    /// Pull-up resistor (~50kΩ to VDD)
    Up,
    /// Pull-down resistor (~50kΩ to GND)
    Down,
}

/// Output drive strength.
///
/// Controls the maximum current a pin can source or sink.
/// Higher drive strength allows driving heavier loads but
/// increases power consumption and EMI.
///
/// # Example
///
/// ```ignore
/// // High-speed LED driver
/// led.set_drive_strength(DriveStrength::_12mA);
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DriveStrength {
    /// 2mA drive (lowest power, suitable for slow signals)
    _2mA = 0,
    /// 4mA drive (default)
    _4mA = 1,
    /// 8mA drive (good for LEDs)
    _8mA = 2,
    /// 12mA drive (maximum, for heavy loads)
    _12mA = 3,
}

/// Output slew rate control.
///
/// Controls how fast the output transitions between high and low.
/// Fast slew rate enables higher frequencies but increases EMI.
///
/// # Example
///
/// ```ignore
/// // High-speed SPI clock
/// sck_pin.set_slew_rate(SlewRate::Fast);
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SlewRate {
    /// Slow slew rate (reduced EMI, ~10ns rise/fall)
    Slow,
    /// Fast slew rate (higher speed, ~2ns rise/fall)
    Fast,
}

/// Output override mode.
///
/// Allows forcing the output value regardless of the peripheral signal.
/// Useful for debugging or special control scenarios.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OutputOverride {
    /// Use the peripheral's output signal normally
    Normal = 0,
    /// Invert the peripheral's output signal
    Invert = 1,
    /// Force output low (ignore peripheral)
    Low = 2,
    /// Force output high (ignore peripheral)
    High = 3,
}

/// Output enable override mode.
///
/// Controls whether the output driver is enabled (pin drives actively)
/// or disabled (high-impedance state).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OutputEnableOverride {
    /// Use the peripheral's output enable signal
    Normal = 0,
    /// Invert the peripheral's output enable
    Invert = 1,
    /// Force output disabled (high-impedance)
    Disable = 2,
    /// Force output enabled
    Enable = 3,
}

/// Input override mode.
///
/// Allows modifying the input signal seen by the peripheral.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum InputOverride {
    /// Use the pad's input signal normally
    Normal = 0,
    /// Invert the input signal
    Invert = 1,
    /// Force input low (ignore pad)
    Low = 2,
    /// Force input high (ignore pad)
    High = 3,
}

/// Interrupt override mode.
///
/// Allows modifying the interrupt signal generated by the pin.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IrqOverride {
    /// Use the generated interrupt signal
    Normal = 0,
    /// Invert the interrupt signal
    Invert = 1,
    /// Force interrupt inactive
    Low = 2,
    /// Force interrupt active
    High = 3,
}

/// GPIO interrupt trigger condition.
///
/// Specifies what condition should trigger an interrupt.
/// Multiple conditions can be enabled simultaneously.
///
/// # Example
///
/// ```ignore
/// // Trigger interrupt on button press (falling edge)
/// button.enable_interrupt(EdgeLevel::EdgeLow);
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EdgeLevel {
    /// Trigger while input is low
    LevelLow,
    /// Trigger while input is high
    LevelHigh,
    /// Trigger on falling edge (high to low transition)
    EdgeLow,
    /// Trigger on rising edge (low to high transition)
    EdgeHigh,
}

// ============================================================================
// Pin Implementation Macro
// ============================================================================

/// Macro to implement a GPIO pin with compile-time known register offsets.
///
/// This macro generates a `Pin<N>` implementation that uses hardcoded
/// register names (e.g., `gpio25_ctrl`, `gpio25`) instead of runtime
/// offset calculations, resulting in optimal code generation.
macro_rules! impl_pin {
    // Standard pin (0-31) - uses GPIO_OUT/GPIO_OE
    ($n:literal, low) => {
        paste! {
            impl Pin<$n> {
                /// Pin number
                pub const PIN: u8 = $n;

                /// Bit mask for this pin in the 32-bit GPIO registers
                pub const MASK: u32 = 1u32 << $n;

                /// Create a new `Pin<N>` instance without ownership checks.
                ///
                /// This is useful in ISR context where pin ownership is held
                /// by `main()` and not accessible.
                ///
                /// # Safety
                ///
                /// The caller must ensure that:
                /// - The GPIO subsystem (IO_BANK0, PADS_BANK0) has been initialised
                ///   (e.g. via [`Pins::new()`]).
                /// - No data races occur — i.e. the same pin is not being
                ///   concurrently reconfigured from another context.
                #[inline]
                pub const unsafe fn new() -> Self {
                    Self { _marker: core::marker::PhantomData }
                }

                // ================================================================
                // Internal register accessors (using steal)
                // ================================================================

                #[inline(always)]
                fn io_bank(&self) -> pico_rs_pac::IO_BANK0 {
                    unsafe { IO_BANK0::steal() }
                }

                #[inline(always)]
                fn pads_bank(&self) -> pico_rs_pac::PADS_BANK0 {
                    unsafe { PADS_BANK0::steal() }
                }

                #[inline(always)]
                fn sio(&self) -> &'static SioRegisterBlock {
                    crate::sio::regs()
                }

                // ================================================================
                // Function Selection
                // ================================================================

                /// Set the GPIO function.
                ///
                /// This determines which peripheral (if any) controls the pin.
                /// For direct CPU control, use `Function::Sio`.
                ///
                /// # Example
                ///
                /// ```ignore
                /// pin.set_function(Function::Uart);
                /// ```
                #[inline]
                pub fn set_function(&self, func: Function) {
                    self.io_bank().[<gpio $n _ctrl>].write_funcsel(func as u32);
                }

                /// Get the current GPIO function.
                #[inline]
                pub fn function(&self) -> u32 {
                    self.io_bank().[<gpio $n _ctrl>].funcsel()
                }

                /// Set function to SIO (direct CPU control).
                #[inline]
                pub fn set_function_sio(&self) {
                    self.set_function(Function::Sio);
                }

                /// Set function to PIO0.
                #[inline]
                pub fn set_function_pio0(&self) {
                    self.set_function(Function::Pio0);
                }

                /// Set function to PIO1.
                #[inline]
                pub fn set_function_pio1(&self) {
                    self.set_function(Function::Pio1);
                }

                /// Set function to PIO2.
                #[inline]
                pub fn set_function_pio2(&self) {
                    self.set_function(Function::Pio2);
                }

                /// Set function to NULL (disabled).
                #[inline]
                pub fn set_function_null(&self) {
                    self.set_function(Function::Null);
                }

                // ================================================================
                // Pad Configuration
                // ================================================================

                /// Initialize the pad for normal operation.
                ///
                /// This method:
                /// - Clears pad isolation (critical for RP2350)
                /// - Enables input receiver
                /// - Clears output disable
                ///
                /// Call this before using a pin for the first time.
                #[inline]
                pub fn init_pad(&self) {
                    let pad = &self.pads_bank().[<gpio $n>];
                    pad.clr_iso();  // Remove pad isolation
                    pad.set_ie();   // Enable input
                    pad.clr_od();   // Clear output disable
                }

                /// Set pull resistor configuration.
                ///
                /// # Arguments
                ///
                /// * `pull` - The pull resistor configuration
                ///
                /// # Example
                ///
                /// ```ignore
                /// button.set_pull(Pull::Up);
                /// ```
                #[inline]
                pub fn set_pull(&self, pull: Pull) {
                    let pad = &self.pads_bank().[<gpio $n>];
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

                /// Enable pull-up resistor.
                #[inline]
                pub fn set_pull_up(&self) {
                    self.set_pull(Pull::Up);
                }

                /// Enable pull-down resistor.
                #[inline]
                pub fn set_pull_down(&self) {
                    self.set_pull(Pull::Down);
                }

                /// Disable pull resistor.
                #[inline]
                pub fn set_pull_none(&self) {
                    self.set_pull(Pull::None);
                }

                /// Set output drive strength.
                ///
                /// # Arguments
                ///
                /// * `strength` - Drive current capability
                #[inline]
                pub fn set_drive_strength(&self, strength: DriveStrength) {
                    self.pads_bank().[<gpio $n>].write_drive(strength as u32);
                }

                /// Set output slew rate.
                ///
                /// # Arguments
                ///
                /// * `rate` - Slew rate (Slow or Fast)
                #[inline]
                pub fn set_slew_rate(&self, rate: SlewRate) {
                    let pad = &self.pads_bank().[<gpio $n>];
                    match rate {
                        SlewRate::Slow => pad.clr_slewfast(),
                        SlewRate::Fast => pad.set_slewfast(),
                    }
                }

                /// Enable or disable Schmitt trigger.
                ///
                /// Schmitt trigger adds hysteresis to the input, improving
                /// noise immunity for slow or noisy signals.
                #[inline]
                pub fn set_schmitt(&self, enable: bool) {
                    let pad = &self.pads_bank().[<gpio $n>];
                    if enable {
                        pad.set_schmitt();
                    } else {
                        pad.clr_schmitt();
                    }
                }

                /// Enable input receiver.
                #[inline]
                pub fn enable_input(&self) {
                    self.pads_bank().[<gpio $n>].set_ie();
                }

                /// Disable input receiver.
                #[inline]
                pub fn disable_input(&self) {
                    self.pads_bank().[<gpio $n>].clr_ie();
                }

                /// Set output disable.
                ///
                /// When disabled, the output driver is in high-impedance state.
                #[inline]
                pub fn set_output_disable(&self, disable: bool) {
                    let pad = &self.pads_bank().[<gpio $n>];
                    if disable {
                        pad.set_od();
                    } else {
                        pad.clr_od();
                    }
                }

                // ================================================================
                // GPIO Control Overrides
                // ================================================================

                /// Set output override.
                #[inline]
                pub fn set_output_override(&self, ov: OutputOverride) {
                    self.io_bank().[<gpio $n _ctrl>].write_outover(ov as u32);
                }

                /// Set output enable override.
                #[inline]
                pub fn set_output_enable_override(&self, ov: OutputEnableOverride) {
                    self.io_bank().[<gpio $n _ctrl>].write_oeover(ov as u32);
                }

                /// Set input override.
                #[inline]
                pub fn set_input_override(&self, ov: InputOverride) {
                    self.io_bank().[<gpio $n _ctrl>].write_inover(ov as u32);
                }

                /// Set IRQ override.
                #[inline]
                pub fn set_irq_override(&self, ov: IrqOverride) {
                    self.io_bank().[<gpio $n _ctrl>].write_irqover(ov as u32);
                }

                // ================================================================
                // SIO Direction Control
                // ================================================================

                /// Set pin direction to output.
                ///
                /// Uses the atomic SET register for interrupt safety.
                #[inline]
                pub fn set_direction_out(&self) {
                    self.sio().gpio_oe_set.write_gpio_oe_set(Self::MASK);
                }

                /// Set pin direction to input.
                ///
                /// Uses the atomic CLR register for interrupt safety.
                #[inline]
                pub fn set_direction_in(&self) {
                    self.sio().gpio_oe_clr.write_gpio_oe_clr(Self::MASK);
                }

                /// Check if pin is configured as output.
                #[inline]
                pub fn is_output(&self) -> bool {
                    (self.sio().gpio_oe.gpio_oe() & Self::MASK) != 0
                }

                // ================================================================
                // SIO Input/Output Operations
                // ================================================================

                /// Read the input value.
                ///
                /// Returns `true` if the pin is high, `false` if low.
                #[inline]
                pub fn read(&self) -> bool {
                    (self.sio().gpio_in.gpio_in() & Self::MASK) != 0
                }

                /// Set output high.
                ///
                /// Uses the atomic SET register for interrupt safety.
                #[inline]
                pub fn set_high(&self) {
                    self.sio().gpio_out_set.write_gpio_out_set(Self::MASK);
                }

                /// Set output low.
                ///
                /// Uses the atomic CLR register for interrupt safety.
                #[inline]
                pub fn set_low(&self) {
                    self.sio().gpio_out_clr.write_gpio_out_clr(Self::MASK);
                }

                /// Toggle output.
                ///
                /// Uses the atomic XOR register for interrupt safety.
                #[inline]
                pub fn toggle(&self) {
                    self.sio().gpio_out_xor.write_gpio_out_xor(Self::MASK);
                }

                /// Set output to a specific value.
                ///
                /// # Arguments
                ///
                /// * `high` - `true` for high, `false` for low
                #[inline]
                pub fn set(&self, high: bool) {
                    if high {
                        self.set_high();
                    } else {
                        self.set_low();
                    }
                }

                /// Get the current output level (not the actual pin state).
                ///
                /// This reads the output register, not the input.
                /// Use `read()` to get the actual pin state.
                #[inline]
                pub fn output_level(&self) -> bool {
                    (self.sio().gpio_out.gpio_out() & Self::MASK) != 0
                }

                // ================================================================
                // Convenience Methods
                // ================================================================

                /// Configure as SIO output.
                ///
                /// This method:
                /// 1. Initializes the pad
                /// 2. Sets function to SIO
                /// 3. Sets direction to output
                ///
                /// # Returns
                ///
                /// Reference to self for method chaining.
                ///
                /// # Example
                ///
                /// ```ignore
                /// led.into_output().set_high();
                /// ```
                #[inline]
                pub fn into_output(&self) -> &Self {
                    self.init_pad();
                    self.set_function_sio();
                    self.set_direction_out();
                    self
                }

                /// Configure as SIO input (no pull).
                ///
                /// This method:
                /// 1. Initializes the pad
                /// 2. Sets function to SIO
                /// 3. Sets direction to input
                #[inline]
                pub fn into_input(&self) -> &Self {
                    self.init_pad();
                    self.set_function_sio();
                    self.set_direction_in();
                    self
                }

                /// Configure as SIO input with pull-up.
                ///
                /// Suitable for active-low buttons and open-drain signals.
                #[inline]
                pub fn into_input_pullup(&self) -> &Self {
                    self.into_input();
                    self.set_pull_up();
                    self
                }

                /// Configure as SIO input with pull-down.
                ///
                /// Suitable for active-high buttons.
                #[inline]
                pub fn into_input_pulldown(&self) -> &Self {
                    self.into_input();
                    self.set_pull_down();
                    self
                }

                /// Configure as PIO0 pin.
                #[inline]
                pub fn into_pio0(&self) -> &Self {
                    self.init_pad();
                    self.set_function_pio0();
                    self
                }

                /// Configure as PIO1 pin.
                #[inline]
                pub fn into_pio1(&self) -> &Self {
                    self.init_pad();
                    self.set_function_pio1();
                    self
                }

                /// Configure as PIO2 pin.
                #[inline]
                pub fn into_pio2(&self) -> &Self {
                    self.init_pad();
                    self.set_function_pio2();
                    self
                }

                /// Configure as PWM pin.
                #[inline]
                pub fn into_pwm(&self) -> &Self {
                    self.init_pad();
                    self.set_function(Function::Pwm);
                    self
                }

                /// Configure as I2C pin.
                ///
                /// Also enables pull-up (required for I2C).
                #[inline]
                pub fn into_i2c(&self) -> &Self {
                    self.init_pad();
                    self.set_function(Function::I2c);
                    self.set_pull_up();
                    self
                }

                /// Configure as SPI pin.
                #[inline]
                pub fn into_spi(&self) -> &Self {
                    self.init_pad();
                    self.set_function(Function::Spi);
                    self
                }

                /// Configure as UART pin.
                #[inline]
                pub fn into_uart(&self) -> &Self {
                    self.init_pad();
                    self.set_function(Function::Uart);
                    self
                }

                // ================================================================
                // Interrupt Configuration
                // ================================================================

                /// Enable a GPIO interrupt for processor 0.
                ///
                /// Configures the GPIO to generate an interrupt on the specified
                /// edge or level condition. You must also enable `IO_IRQ_BANK0`
                /// in the NVIC for the interrupt handler to fire.
                ///
                /// # Arguments
                ///
                /// * `edge` - The edge/level condition to trigger the interrupt
                ///
                /// # Example
                ///
                /// ```ignore
                /// use pico_rs::gpio::{Pins, EdgeLevel};
                /// use pico_rs::interrupt;
                /// use pico_rs::pac::Interrupt;
                ///
                /// let pins = Pins::new(dp.IO_BANK0, dp.PADS_BANK0);
                /// let button = pins.pin::<15>();
                /// button.into_input_pullup();
                /// button.enable_interrupt(EdgeLevel::EdgeLow);
                ///
                /// unsafe { interrupt::enable(Interrupt::IO_IRQ_BANK0); }
                /// ```
                #[inline]
                pub fn enable_interrupt(&self, edge: EdgeLevel) {
                    const REG_IDX: usize = $n / 8;
                    const BIT_OFFSET: u32 = (($n % 8) * 4) as u32;

                    let bit = match edge {
                        EdgeLevel::LevelLow => 1 << (BIT_OFFSET + 0),
                        EdgeLevel::LevelHigh => 1 << (BIT_OFFSET + 1),
                        EdgeLevel::EdgeLow => 1 << (BIT_OFFSET + 2),
                        EdgeLevel::EdgeHigh => 1 << (BIT_OFFSET + 3),
                    };

                    let io = self.io_bank();
                    match REG_IDX {
                        0 => io.proc0_inte0.set_bits(bit),
                        1 => io.proc0_inte1.set_bits(bit),
                        2 => io.proc0_inte2.set_bits(bit),
                        3 => io.proc0_inte3.set_bits(bit),
                        _ => {} // Pins 0-29 only use INTE0-INTE3
                    }
                }

                /// Disable a GPIO interrupt for processor 0.
                ///
                /// Clears the interrupt enable for the specified edge/level condition.
                ///
                /// # Arguments
                ///
                /// * `edge` - The edge/level condition to disable
                #[inline]
                pub fn disable_interrupt(&self, edge: EdgeLevel) {
                    const REG_IDX: usize = $n / 8;
                    const BIT_OFFSET: u32 = (($n % 8) * 4) as u32;

                    let bit = match edge {
                        EdgeLevel::LevelLow => 1 << (BIT_OFFSET + 0),
                        EdgeLevel::LevelHigh => 1 << (BIT_OFFSET + 1),
                        EdgeLevel::EdgeLow => 1 << (BIT_OFFSET + 2),
                        EdgeLevel::EdgeHigh => 1 << (BIT_OFFSET + 3),
                    };

                    let io = self.io_bank();
                    match REG_IDX {
                        0 => io.proc0_inte0.clr_bits(bit),
                        1 => io.proc0_inte1.clr_bits(bit),
                        2 => io.proc0_inte2.clr_bits(bit),
                        3 => io.proc0_inte3.clr_bits(bit),
                        _ => {}
                    }
                }

                /// Check if a GPIO interrupt is pending (masked status).
                ///
                /// Returns `true` if the specified interrupt condition is active
                /// and enabled for processor 0.
                ///
                /// # Arguments
                ///
                /// * `edge` - The edge/level condition to check
                #[inline]
                pub fn interrupt_status(&self, edge: EdgeLevel) -> bool {
                    const REG_IDX: usize = $n / 8;
                    const BIT_OFFSET: u32 = (($n % 8) * 4) as u32;

                    let bit = match edge {
                        EdgeLevel::LevelLow => 1 << (BIT_OFFSET + 0),
                        EdgeLevel::LevelHigh => 1 << (BIT_OFFSET + 1),
                        EdgeLevel::EdgeLow => 1 << (BIT_OFFSET + 2),
                        EdgeLevel::EdgeHigh => 1 << (BIT_OFFSET + 3),
                    };

                    let io = self.io_bank();
                    let status = match REG_IDX {
                        0 => io.proc0_ints0.read_bits(),
                        1 => io.proc0_ints1.read_bits(),
                        2 => io.proc0_ints2.read_bits(),
                        3 => io.proc0_ints3.read_bits(),
                        _ => 0,
                    };
                    (status & bit) != 0
                }

                /// Clear a GPIO edge interrupt.
                ///
                /// For edge-triggered interrupts, the interrupt flag must be
                /// explicitly cleared by writing to the raw interrupt register.
                /// Level-triggered interrupts are cleared automatically when the
                /// level condition is no longer met.
                ///
                /// # Arguments
                ///
                /// * `edge` - The edge/level condition to acknowledge
                ///
                /// # Example
                ///
                /// ```ignore
                /// // In the IO_IRQ_BANK0 interrupt handler:
                /// if button.interrupt_status(EdgeLevel::EdgeLow) {
                ///     button.clear_interrupt(EdgeLevel::EdgeLow);
                ///     // Handle the interrupt...
                /// }
                /// ```
                #[inline]
                pub fn clear_interrupt(&self, edge: EdgeLevel) {
                    const REG_IDX: usize = $n / 8;
                    const BIT_OFFSET: u32 = (($n % 8) * 4) as u32;

                    let bit = match edge {
                        EdgeLevel::LevelLow => 1 << (BIT_OFFSET + 0),
                        EdgeLevel::LevelHigh => 1 << (BIT_OFFSET + 1),
                        EdgeLevel::EdgeLow => 1 << (BIT_OFFSET + 2),
                        EdgeLevel::EdgeHigh => 1 << (BIT_OFFSET + 3),
                    };

                    // INTR registers are W1C (Write-1-to-Clear).
                    // Must use write_bits() to write directly to the register.
                    // Do NOT use set_bits() — it writes to the SET alias (+0x2000),
                    // which has undefined behaviour on W1C registers.
                    let io = self.io_bank();
                    match REG_IDX {
                        0 => io.intr0.write_bits(bit),
                        1 => io.intr1.write_bits(bit),
                        2 => io.intr2.write_bits(bit),
                        3 => io.intr3.write_bits(bit),
                        _ => {}
                    }
                }

                /// Disable all interrupt conditions for this pin.
                ///
                /// Clears all four interrupt enable bits (level_low, level_high,
                /// edge_low, edge_high) for processor 0.
                #[inline]
                pub fn disable_all_interrupts(&self) {
                    const REG_IDX: usize = $n / 8;
                    const BIT_OFFSET: u32 = (($n % 8) * 4) as u32;
                    const MASK: u32 = 0xF << BIT_OFFSET;

                    let io = self.io_bank();
                    match REG_IDX {
                        0 => io.proc0_inte0.clr_bits(MASK),
                        1 => io.proc0_inte1.clr_bits(MASK),
                        2 => io.proc0_inte2.clr_bits(MASK),
                        3 => io.proc0_inte3.clr_bits(MASK),
                        _ => {}
                    }
                }

                /// Get the raw masked interrupt status (all 4 conditions).
                ///
                /// Returns bits `[3:0]` = `EDGE_HIGH | EDGE_LOW | LEVEL_HIGH | LEVEL_LOW`.
                #[inline]
                pub fn pending_flags(&self) -> u32 {
                    const REG_IDX: usize = $n / 8;
                    const BIT_OFFSET: u32 = (($n % 8) * 4) as u32;

                    let io = self.io_bank();
                    let raw = match REG_IDX {
                        0 => io.proc0_ints0.read_bits(),
                        1 => io.proc0_ints1.read_bits(),
                        2 => io.proc0_ints2.read_bits(),
                        3 => io.proc0_ints3.read_bits(),
                        _ => 0,
                    };
                    (raw >> BIT_OFFSET) & 0xF
                }

                /// Clear all pending edge interrupts for this pin.
                ///
                /// Writes to the raw interrupt register (`INTR`) which is W1C.
                /// Level-triggered bits are read-only and unaffected.
                #[inline]
                pub fn clear_all_interrupts(&self) {
                    const REG_IDX: usize = $n / 8;
                    const BIT_OFFSET: u32 = (($n % 8) * 4) as u32;
                    const MASK: u32 = 0xF << BIT_OFFSET;

                    let io = self.io_bank();
                    match REG_IDX {
                        0 => io.intr0.write_bits(MASK),
                        1 => io.intr1.write_bits(MASK),
                        2 => io.intr2.write_bits(MASK),
                        3 => io.intr3.write_bits(MASK),
                        _ => {}
                    }
                }

                /// Check and clear an edge interrupt in one call.
                ///
                /// Returns `true` if the interrupt was pending (and is now cleared).
                /// This is the typical pattern for edge-triggered interrupt handlers.
                ///
                /// # Example
                ///
                /// ```ignore
                /// // In the IO_IRQ_BANK0 interrupt handler:
                /// let button = unsafe { Pin::<15>::new() };
                /// if button.check_and_clear(EdgeLevel::EdgeLow) {
                ///     // Handle the button press...
                /// }
                /// ```
                #[inline]
                pub fn check_and_clear(&self, edge: EdgeLevel) -> bool {
                    if self.interrupt_status(edge) {
                        self.clear_interrupt(edge);
                        true
                    } else {
                        false
                    }
                }

                // ================================================================
                // Dormant Wake Configuration
                // ================================================================

                /// Enable this pin as a wake source for dormant mode.
                ///
                /// Configures the GPIO to generate a dormant wake interrupt
                /// on the specified edge/level condition.
                ///
                /// # Arguments
                ///
                /// * `edge` - The edge/level condition to trigger wake
                ///
                /// # Example
                ///
                /// ```ignore
                /// // Configure pin 0 to wake on falling edge (button press)
                /// let p0 = pins.pin::<0>();
                /// p0.into_input_pullup();
                /// p0.set_dormant_wake(EdgeLevel::EdgeLow);
                ///
                /// // Enter dormant mode...
                /// clocks.dormant(DormantSource::Xosc, &xosc);
                /// ```
                #[inline]
                pub fn set_dormant_wake(&self, edge: EdgeLevel) {
                    // Each DORMANT_WAKE_INTE register covers 8 GPIOs (4 bits each)
                    // Pin N is in register N/8, at bit offset (N%8)*4
                    const REG_IDX: usize = $n / 8;
                    const BIT_OFFSET: u32 = (($n % 8) * 4) as u32;

                    let bit = match edge {
                        EdgeLevel::LevelLow => 1 << (BIT_OFFSET + 0),
                        EdgeLevel::LevelHigh => 1 << (BIT_OFFSET + 1),
                        EdgeLevel::EdgeLow => 1 << (BIT_OFFSET + 2),
                        EdgeLevel::EdgeHigh => 1 << (BIT_OFFSET + 3),
                    };

                    let io = self.io_bank();
                    match REG_IDX {
                        0 => io.dormant_wake_inte0.set_bits(bit),
                        1 => io.dormant_wake_inte1.set_bits(bit),
                        2 => io.dormant_wake_inte2.set_bits(bit),
                        3 => io.dormant_wake_inte3.set_bits(bit),
                        _ => {} // Pins 0-29 only use INTE0-INTE3
                    }
                }

                /// Disable dormant wake for this pin.
                ///
                /// Clears all dormant wake conditions for this pin.
                #[inline]
                pub fn clear_dormant_wake(&self) {
                    const REG_IDX: usize = $n / 8;
                    const BIT_OFFSET: u32 = (($n % 8) * 4) as u32;
                    const MASK: u32 = 0xF << BIT_OFFSET;

                    let io = self.io_bank();
                    match REG_IDX {
                        0 => io.dormant_wake_inte0.clr_bits(MASK),
                        1 => io.dormant_wake_inte1.clr_bits(MASK),
                        2 => io.dormant_wake_inte2.clr_bits(MASK),
                        3 => io.dormant_wake_inte3.clr_bits(MASK),
                        _ => {}
                    }
                }
            }

            impl core::fmt::Debug for Pin<$n> {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.debug_struct("Pin")
                        .field("number", &$n)
                        .field("function", &self.function())
                        .field("is_output", &self.is_output())
                        .finish()
                }
            }
        }
    };

    // High pin (32-47) - uses GPIO_HI_OUT/GPIO_HI_OE (RP2350B only)
    ($n:literal, high) => {
        paste! {
            impl Pin<$n> {
                /// Pin number
                pub const PIN: u8 = $n;

                /// Bit mask for this pin in the high GPIO registers (32-47 → bits 0-15)
                pub const MASK: u32 = 1u32 << ($n - 32);

                /// Create a new `Pin<N>` instance without ownership checks.
                ///
                /// This is useful in ISR context where pin ownership is held
                /// by `main()` and not accessible.
                ///
                /// # Safety
                ///
                /// The caller must ensure that:
                /// - The GPIO subsystem (IO_BANK0, PADS_BANK0) has been initialised
                ///   (e.g. via [`Pins::new()`]).
                /// - No data races occur — i.e. the same pin is not being
                ///   concurrently reconfigured from another context.
                #[inline]
                pub const unsafe fn new() -> Self {
                    Self { _marker: core::marker::PhantomData<*const ()> }
                }

                #[inline(always)]
                fn io_bank(&self) -> pico_rs_pac::IO_BANK0 {
                    unsafe { IO_BANK0::steal() }
                }

                #[inline(always)]
                fn pads_bank(&self) -> pico_rs_pac::PADS_BANK0 {
                    unsafe { PADS_BANK0::steal() }
                }

                #[inline(always)]
                fn sio(&self) -> &'static SioRegisterBlock {
                    crate::sio::regs()
                }

                // Function selection (same as low pins)
                #[inline]
                pub fn set_function(&self, func: Function) {
                    self.io_bank().[<gpio $n _ctrl>].write_funcsel(func as u32);
                }

                #[inline]
                pub fn function(&self) -> u32 {
                    self.io_bank().[<gpio $n _ctrl>].funcsel()
                }

                #[inline]
                pub fn set_function_sio(&self) {
                    self.set_function(Function::Sio);
                }

                #[inline]
                pub fn set_function_pio0(&self) {
                    self.set_function(Function::Pio0);
                }

                #[inline]
                pub fn set_function_pio1(&self) {
                    self.set_function(Function::Pio1);
                }

                #[inline]
                pub fn set_function_pio2(&self) {
                    self.set_function(Function::Pio2);
                }

                #[inline]
                pub fn set_function_null(&self) {
                    self.set_function(Function::Null);
                }

                // Pad configuration (same as low pins)
                #[inline]
                pub fn init_pad(&self) {
                    let pad = &self.pads_bank().[<gpio $n>];
                    pad.clr_iso();
                    pad.set_ie();
                    pad.clr_od();
                }

                #[inline]
                pub fn set_pull(&self, pull: Pull) {
                    let pad = &self.pads_bank().[<gpio $n>];
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

                #[inline]
                pub fn set_pull_up(&self) {
                    self.set_pull(Pull::Up);
                }

                #[inline]
                pub fn set_pull_down(&self) {
                    self.set_pull(Pull::Down);
                }

                #[inline]
                pub fn set_pull_none(&self) {
                    self.set_pull(Pull::None);
                }

                #[inline]
                pub fn set_drive_strength(&self, strength: DriveStrength) {
                    self.pads_bank().[<gpio $n>].write_drive(strength as u32);
                }

                #[inline]
                pub fn set_slew_rate(&self, rate: SlewRate) {
                    let pad = &self.pads_bank().[<gpio $n>];
                    match rate {
                        SlewRate::Slow => pad.clr_slewfast(),
                        SlewRate::Fast => pad.set_slewfast(),
                    }
                }

                #[inline]
                pub fn set_schmitt(&self, enable: bool) {
                    let pad = &self.pads_bank().[<gpio $n>];
                    if enable {
                        pad.set_schmitt();
                    } else {
                        pad.clr_schmitt();
                    }
                }

                #[inline]
                pub fn enable_input(&self) {
                    self.pads_bank().[<gpio $n>].set_ie();
                }

                #[inline]
                pub fn disable_input(&self) {
                    self.pads_bank().[<gpio $n>].clr_ie();
                }

                #[inline]
                pub fn set_output_disable(&self, disable: bool) {
                    let pad = &self.pads_bank().[<gpio $n>];
                    if disable {
                        pad.set_od();
                    } else {
                        pad.clr_od();
                    }
                }

                // Control overrides (same as low pins)
                #[inline]
                pub fn set_output_override(&self, ov: OutputOverride) {
                    self.io_bank().[<gpio $n _ctrl>].write_outover(ov as u32);
                }

                #[inline]
                pub fn set_output_enable_override(&self, ov: OutputEnableOverride) {
                    self.io_bank().[<gpio $n _ctrl>].write_oeover(ov as u32);
                }

                #[inline]
                pub fn set_input_override(&self, ov: InputOverride) {
                    self.io_bank().[<gpio $n _ctrl>].write_inover(ov as u32);
                }

                #[inline]
                pub fn set_irq_override(&self, ov: IrqOverride) {
                    self.io_bank().[<gpio $n _ctrl>].write_irqover(ov as u32);
                }

                // SIO operations - use GPIO_HI_* registers
                #[inline]
                pub fn set_direction_out(&self) {
                    self.sio().gpio_hi_oe_set.write_gpio(Self::MASK);
                }

                #[inline]
                pub fn set_direction_in(&self) {
                    self.sio().gpio_hi_oe_clr.write_gpio(Self::MASK);
                }

                #[inline]
                pub fn is_output(&self) -> bool {
                    (self.sio().gpio_hi_oe.gpio() & Self::MASK) != 0
                }

                #[inline]
                pub fn read(&self) -> bool {
                    (self.sio().gpio_hi_in.gpio() & Self::MASK) != 0
                }

                #[inline]
                pub fn set_high(&self) {
                    self.sio().gpio_hi_out_set.write_gpio(Self::MASK);
                }

                #[inline]
                pub fn set_low(&self) {
                    self.sio().gpio_hi_out_clr.write_gpio(Self::MASK);
                }

                #[inline]
                pub fn toggle(&self) {
                    self.sio().gpio_hi_out_xor.write_gpio(Self::MASK);
                }

                #[inline]
                pub fn set(&self, high: bool) {
                    if high {
                        self.set_high();
                    } else {
                        self.set_low();
                    }
                }

                #[inline]
                pub fn output_level(&self) -> bool {
                    (self.sio().gpio_hi_out.gpio() & Self::MASK) != 0
                }

                // Convenience methods (same as low pins)
                #[inline]
                pub fn into_output(&self) -> &Self {
                    self.init_pad();
                    self.set_function_sio();
                    self.set_direction_out();
                    self
                }

                #[inline]
                pub fn into_input(&self) -> &Self {
                    self.init_pad();
                    self.set_function_sio();
                    self.set_direction_in();
                    self
                }

                #[inline]
                pub fn into_input_pullup(&self) -> &Self {
                    self.into_input();
                    self.set_pull_up();
                    self
                }

                #[inline]
                pub fn into_input_pulldown(&self) -> &Self {
                    self.into_input();
                    self.set_pull_down();
                    self
                }

                #[inline]
                pub fn into_pio0(&self) -> &Self {
                    self.init_pad();
                    self.set_function_pio0();
                    self
                }

                #[inline]
                pub fn into_pio1(&self) -> &Self {
                    self.init_pad();
                    self.set_function_pio1();
                    self
                }

                #[inline]
                pub fn into_pio2(&self) -> &Self {
                    self.init_pad();
                    self.set_function_pio2();
                    self
                }

                #[inline]
                pub fn into_pwm(&self) -> &Self {
                    self.init_pad();
                    self.set_function(Function::Pwm);
                    self
                }

                #[inline]
                pub fn into_i2c(&self) -> &Self {
                    self.init_pad();
                    self.set_function(Function::I2c);
                    self.set_pull_up();
                    self
                }

                #[inline]
                pub fn into_spi(&self) -> &Self {
                    self.init_pad();
                    self.set_function(Function::Spi);
                    self
                }

                #[inline]
                pub fn into_uart(&self) -> &Self {
                    self.init_pad();
                    self.set_function(Function::Uart);
                    self
                }

                // ================================================================
                // Interrupt Configuration
                // ================================================================

                /// Enable a GPIO interrupt for processor 0.
                ///
                /// Configures the GPIO to generate an interrupt on the specified
                /// edge or level condition. You must also enable `IO_IRQ_BANK0`
                /// in the NVIC for the interrupt handler to fire.
                #[inline]
                pub fn enable_interrupt(&self, edge: EdgeLevel) {
                    const REG_IDX: usize = $n / 8;
                    const BIT_OFFSET: u32 = (($n % 8) * 4) as u32;

                    let bit = match edge {
                        EdgeLevel::LevelLow => 1 << (BIT_OFFSET + 0),
                        EdgeLevel::LevelHigh => 1 << (BIT_OFFSET + 1),
                        EdgeLevel::EdgeLow => 1 << (BIT_OFFSET + 2),
                        EdgeLevel::EdgeHigh => 1 << (BIT_OFFSET + 3),
                    };

                    let io = self.io_bank();
                    match REG_IDX {
                        4 => io.proc0_inte4.set_bits(bit),
                        5 => io.proc0_inte5.set_bits(bit),
                        _ => {}
                    }
                }

                /// Disable a GPIO interrupt for processor 0.
                #[inline]
                pub fn disable_interrupt(&self, edge: EdgeLevel) {
                    const REG_IDX: usize = $n / 8;
                    const BIT_OFFSET: u32 = (($n % 8) * 4) as u32;

                    let bit = match edge {
                        EdgeLevel::LevelLow => 1 << (BIT_OFFSET + 0),
                        EdgeLevel::LevelHigh => 1 << (BIT_OFFSET + 1),
                        EdgeLevel::EdgeLow => 1 << (BIT_OFFSET + 2),
                        EdgeLevel::EdgeHigh => 1 << (BIT_OFFSET + 3),
                    };

                    let io = self.io_bank();
                    match REG_IDX {
                        4 => io.proc0_inte4.clr_bits(bit),
                        5 => io.proc0_inte5.clr_bits(bit),
                        _ => {}
                    }
                }

                /// Check if a GPIO interrupt is pending (masked status).
                #[inline]
                pub fn interrupt_status(&self, edge: EdgeLevel) -> bool {
                    const REG_IDX: usize = $n / 8;
                    const BIT_OFFSET: u32 = (($n % 8) * 4) as u32;

                    let bit = match edge {
                        EdgeLevel::LevelLow => 1 << (BIT_OFFSET + 0),
                        EdgeLevel::LevelHigh => 1 << (BIT_OFFSET + 1),
                        EdgeLevel::EdgeLow => 1 << (BIT_OFFSET + 2),
                        EdgeLevel::EdgeHigh => 1 << (BIT_OFFSET + 3),
                    };

                    let io = self.io_bank();
                    let status = match REG_IDX {
                        4 => io.proc0_ints4.read_bits(),
                        5 => io.proc0_ints5.read_bits(),
                        _ => 0,
                    };
                    (status & bit) != 0
                }

                /// Clear a GPIO edge interrupt.
                #[inline]
                pub fn clear_interrupt(&self, edge: EdgeLevel) {
                    const REG_IDX: usize = $n / 8;
                    const BIT_OFFSET: u32 = (($n % 8) * 4) as u32;

                    let bit = match edge {
                        EdgeLevel::LevelLow => 1 << (BIT_OFFSET + 0),
                        EdgeLevel::LevelHigh => 1 << (BIT_OFFSET + 1),
                        EdgeLevel::EdgeLow => 1 << (BIT_OFFSET + 2),
                        EdgeLevel::EdgeHigh => 1 << (BIT_OFFSET + 3),
                    };

                    // INTR registers are W1C — must use write_bits(), not set_bits().
                    let io = self.io_bank();
                    match REG_IDX {
                        4 => io.intr4.write_bits(bit),
                        5 => io.intr5.write_bits(bit),
                        _ => {}
                    }
                }

                /// Disable all interrupt conditions for this pin.
                #[inline]
                pub fn disable_all_interrupts(&self) {
                    const REG_IDX: usize = $n / 8;
                    const BIT_OFFSET: u32 = (($n % 8) * 4) as u32;
                    const MASK: u32 = 0xF << BIT_OFFSET;

                    let io = self.io_bank();
                    match REG_IDX {
                        4 => io.proc0_inte4.clr_bits(MASK),
                        5 => io.proc0_inte5.clr_bits(MASK),
                        _ => {}
                    }
                }

                /// Get the raw masked interrupt status (all 4 conditions).
                ///
                /// Returns bits `[3:0]` = `EDGE_HIGH | EDGE_LOW | LEVEL_HIGH | LEVEL_LOW`.
                #[inline]
                pub fn pending_flags(&self) -> u32 {
                    const REG_IDX: usize = $n / 8;
                    const BIT_OFFSET: u32 = (($n % 8) * 4) as u32;

                    let io = self.io_bank();
                    let raw = match REG_IDX {
                        4 => io.proc0_ints4.read_bits(),
                        5 => io.proc0_ints5.read_bits(),
                        _ => 0,
                    };
                    (raw >> BIT_OFFSET) & 0xF
                }

                /// Clear all pending edge interrupts for this pin.
                ///
                /// Writes to the raw interrupt register (`INTR`) which is W1C.
                /// Level-triggered bits are read-only and unaffected.
                #[inline]
                pub fn clear_all_interrupts(&self) {
                    const REG_IDX: usize = $n / 8;
                    const BIT_OFFSET: u32 = (($n % 8) * 4) as u32;
                    const MASK: u32 = 0xF << BIT_OFFSET;

                    let io = self.io_bank();
                    match REG_IDX {
                        4 => io.intr4.write_bits(MASK),
                        5 => io.intr5.write_bits(MASK),
                        _ => {}
                    }
                }

                /// Check and clear an edge interrupt in one call.
                ///
                /// Returns `true` if the interrupt was pending (and is now cleared).
                #[inline]
                pub fn check_and_clear(&self, edge: EdgeLevel) -> bool {
                    if self.interrupt_status(edge) {
                        self.clear_interrupt(edge);
                        true
                    } else {
                        false
                    }
                }

                // ================================================================
                // Dormant Wake Configuration
                // ================================================================

                /// Enable this pin as a wake source for dormant mode.
                ///
                /// Configures the GPIO to generate a dormant wake interrupt
                /// on the specified edge/level condition.
                #[inline]
                pub fn set_dormant_wake(&self, edge: EdgeLevel) {
                    // High pins (32-47) use INTE4 and INTE5
                    const REG_IDX: usize = $n / 8;  // 32-39 → 4, 40-47 → 5
                    const BIT_OFFSET: u32 = (($n % 8) * 4) as u32;

                    let bit = match edge {
                        EdgeLevel::LevelLow => 1 << (BIT_OFFSET + 0),
                        EdgeLevel::LevelHigh => 1 << (BIT_OFFSET + 1),
                        EdgeLevel::EdgeLow => 1 << (BIT_OFFSET + 2),
                        EdgeLevel::EdgeHigh => 1 << (BIT_OFFSET + 3),
                    };

                    let io = self.io_bank();
                    match REG_IDX {
                        4 => io.dormant_wake_inte4.set_bits(bit),
                        5 => io.dormant_wake_inte5.set_bits(bit),
                        _ => {}
                    }
                }

                /// Disable dormant wake for this pin.
                #[inline]
                pub fn clear_dormant_wake(&self) {
                    const REG_IDX: usize = $n / 8;
                    const BIT_OFFSET: u32 = (($n % 8) * 4) as u32;
                    const MASK: u32 = 0xF << BIT_OFFSET;

                    let io = self.io_bank();
                    match REG_IDX {
                        4 => io.dormant_wake_inte4.clr_bits(MASK),
                        5 => io.dormant_wake_inte5.clr_bits(MASK),
                        _ => {}
                    }
                }
            }

            impl core::fmt::Debug for Pin<$n> {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.debug_struct("Pin")
                        .field("number", &$n)
                        .field("function", &self.function())
                        .field("is_output", &self.is_output())
                        .finish()
                }
            }
        }
    };
}

// ============================================================================
// Pin Type Definition
// ============================================================================

/// A GPIO pin with compile-time known pin number.
///
/// The pin number `N` is a const generic parameter, allowing the compiler
/// to generate optimal code with hardcoded register addresses.
///
/// # Obtaining a Pin
///
/// Pins are normally obtained from [`Pins::pin()`]:
///
/// ```ignore
/// let pins = Pins::new(dp.IO_BANK0, dp.PADS_BANK0);
/// let led = pins.pin::<25>();
/// led.into_output();
/// led.set_high();
/// ```
///
/// In ISR context, a pin handle can be created via `unsafe { Pin::<N>::new() }`,
/// provided the GPIO subsystem has already been initialised.
pub struct Pin<const N: u8> {
    _marker: core::marker::PhantomData<*const ()>,
}

// ============================================================================
// Generate Pin implementations for all pins
// ============================================================================

// RP2350A: Pins 0-29 (low bank)
impl_pin!(0, low);
impl_pin!(1, low);
impl_pin!(2, low);
impl_pin!(3, low);
impl_pin!(4, low);
impl_pin!(5, low);
impl_pin!(6, low);
impl_pin!(7, low);
impl_pin!(8, low);
impl_pin!(9, low);
impl_pin!(10, low);
impl_pin!(11, low);
impl_pin!(12, low);
impl_pin!(13, low);
impl_pin!(14, low);
impl_pin!(15, low);
impl_pin!(16, low);
impl_pin!(17, low);
impl_pin!(18, low);
impl_pin!(19, low);
impl_pin!(20, low);
impl_pin!(21, low);
impl_pin!(22, low);
impl_pin!(23, low);
impl_pin!(24, low);
impl_pin!(25, low);
impl_pin!(26, low);
impl_pin!(27, low);
impl_pin!(28, low);
impl_pin!(29, low);

// RP2350B: Additional pins 30-31 (still in low bank)
#[cfg(feature = "rp2350b")]
impl_pin!(30, low);
#[cfg(feature = "rp2350b")]
impl_pin!(31, low);

// RP2350B: Pins 32-47 (high bank)
#[cfg(feature = "rp2350b")]
impl_pin!(32, high);
#[cfg(feature = "rp2350b")]
impl_pin!(33, high);
#[cfg(feature = "rp2350b")]
impl_pin!(34, high);
#[cfg(feature = "rp2350b")]
impl_pin!(35, high);
#[cfg(feature = "rp2350b")]
impl_pin!(36, high);
#[cfg(feature = "rp2350b")]
impl_pin!(37, high);
#[cfg(feature = "rp2350b")]
impl_pin!(38, high);
#[cfg(feature = "rp2350b")]
impl_pin!(39, high);
#[cfg(feature = "rp2350b")]
impl_pin!(40, high);
#[cfg(feature = "rp2350b")]
impl_pin!(41, high);
#[cfg(feature = "rp2350b")]
impl_pin!(42, high);
#[cfg(feature = "rp2350b")]
impl_pin!(43, high);
#[cfg(feature = "rp2350b")]
impl_pin!(44, high);
#[cfg(feature = "rp2350b")]
impl_pin!(45, high);
#[cfg(feature = "rp2350b")]
impl_pin!(46, high);
#[cfg(feature = "rp2350b")]
impl_pin!(47, high);

// ============================================================================
// Pins Container
// ============================================================================

/// GPIO controller, owning the PAC peripherals.
///
/// Create once via [`Pins::new()`], then obtain individual [`Pin`] handles
/// via [`Pins::pin()`]. This mirrors the pattern used by [`Dma::channel()`].
///
/// # Example
///
/// ```ignore
/// use pico_rs::gpio::Pins;
/// use pico_rs::Peripherals;
///
/// let dp = Peripherals::take().unwrap();
/// let pins = Pins::new(dp.IO_BANK0, dp.PADS_BANK0);
///
/// let led = pins.pin::<25>();
/// led.into_output();
/// led.set_high();
///
/// let button = pins.pin::<14>();
/// button.into_input_pullup();
/// ```
pub struct Pins {
    _io_bank0: IO_BANK0,
    _pads_bank0: PADS_BANK0,
}

impl Pins {
    /// Create a new `Pins` controller by consuming GPIO peripherals.
    ///
    /// This brings IO_BANK0 and PADS_BANK0 out of reset and returns a
    /// controller from which individual [`Pin`] handles can be obtained.
    ///
    /// # Arguments
    ///
    /// * `io_bank0`   — IO_BANK0 peripheral (consumed)
    /// * `pads_bank0` — PADS_BANK0 peripheral (consumed)
    ///
    /// # Example
    ///
    /// ```ignore
    /// use pico_rs::gpio::Pins;
    /// use pico_rs::Peripherals;
    ///
    /// let dp = Peripherals::take().unwrap();
    /// let pins = Pins::new(dp.IO_BANK0, dp.PADS_BANK0);
    /// ```
    #[inline]
    pub fn new(io_bank0: IO_BANK0, pads_bank0: PADS_BANK0) -> Self {
        // Initialize GPIO subsystem: reset and unreset peripherals
        let mask = Peripheral::IO_BANK0.mask() | Peripheral::PADS_BANK0.mask();
        reset::reset_block_mask(mask);
        reset::unreset_block_mask(mask);
        reset::unreset_wait_mask(mask);

        Self {
            _io_bank0: io_bank0,
            _pads_bank0: pads_bank0,
        }
    }

    /// Obtain a handle to GPIO pin `N`.
    ///
    /// The returned [`Pin`] is a zero-sized type that accesses the
    /// hardware registers directly. Multiple pins can coexist
    /// simultaneously.
    ///
    /// # Pin Ranges
    ///
    /// - RP2350A: `N` must be 0–29
    /// - RP2350B (feature `rp2350b`): `N` must be 0–47
    ///
    /// # Panics
    ///
    /// Compile-time error if `N` is out of range.
    #[inline]
    pub fn pin<const N: u8>(&self) -> Pin<N> {
        #[cfg(not(feature = "rp2350b"))]
        const {
            assert!(N < 30, "GPIO pin number must be 0-29 (RP2350A)")
        };
        #[cfg(feature = "rp2350b")]
        const {
            assert!(N < 48, "GPIO pin number must be 0-47 (RP2350B)")
        };

        // SAFETY: `Pins::new()` guarantees the GPIO subsystem is initialised.
        // The const assert above ensures N is in the valid range.
        Pin {
            _marker: core::marker::PhantomData,
        }
    }
}

// ============================================================================
// Pin Traits for Peripheral Compatibility
// ============================================================================

/// Marker trait for any GPIO pin.
///
/// This trait is implemented for all `Pin<N>` types and provides
/// a way to work with pins generically when needed.
pub trait GpioPin {
    /// The pin number (0-47)
    const PIN: u8;
}

impl<const N: u8> GpioPin for Pin<N> {
    const PIN: u8 = N;
}

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
    // PIO traits
    Pio0Pin,
    Pio1Pin,
    Pio2Pin,
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
