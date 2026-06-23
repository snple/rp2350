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
//! 4. **[`Gpio`] struct**: A raw, zero-cost SIO GPIO accessor for cases where
//!    the pin number is only known at runtime or when batch operations across
//!    many pins are needed. This type does not participate in ownership
//!    tracking, so constructing it is `unsafe`.
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
//!                      │
//!                      └──► let gpio = unsafe { Gpio::steal() };
//!                            gpio.toggle(25);
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
//! if !button.get() {
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
//! ## Runtime-Indexed GPIO Access
//!
//! When the pin number is only known at runtime, or when code needs to update a
//! variable set of pins using masks, use [`Gpio`]. The configuration and
//! ownership model still lives in [`Pins`] and [`Pin<N>`]; [`Gpio`] is only the
//! data-plane accessor for the SIO GPIO registers.
//!
//! ```ignore
//! let dp = Peripherals::take().unwrap();
//! let pins = Pins::new(dp.IO_BANK0, dp.PADS_BANK0);
//!
//! let led = pins.pin::<25>();
//! led.into_output();
//!
//! // SAFETY: GPIO 25 was configured for SIO output above, and the program keeps
//! // ownership discipline through `Pin<25>` for configuration changes.
//! let gpio = unsafe { Gpio::steal() };
//! gpio.toggle(25);
//! ```
//!
//! ## Interrupt Handling (ISR)
//!
//! In ISR context, `Pin<N>` can be obtained via `unsafe { Pin::<N>::steal() }`
//! without ownership, since the GPIO subsystem was already initialised:
//!
//! ```ignore
//! use pico_rs::gpio::{Pin, EdgeLevel};
//!
//! #[unsafe(no_mangle)]
//! extern "C" fn IO_IRQ_BANK0() {
//!     let button = unsafe { Pin::<15>::steal() };
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

use crate::reset::{self, Resettable};

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

                /// Obtain a `Pin<N>` handle without ownership checks.
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
                pub const unsafe fn steal() -> crate::Steal<Self> {
                    crate::Steal(Self { _marker: core::marker::PhantomData })
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

                /// Get the input value.
                ///
                /// Returns `true` if the pin is high, `false` if low.
                #[inline]
                pub fn get(&self) -> bool {
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
                /// Use `get()` to get the actual pin state.
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
                /// let button = unsafe { Pin::<15>::steal() };
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

                /// Obtain a `Pin<N>` handle without ownership checks.
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
                pub const unsafe fn steal() -> crate::Steal<Self> {
                    crate::Steal(Self { _marker: core::marker::PhantomData<*const ()> })
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
                pub fn get(&self) -> bool {
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
/// In ISR context, a pin handle can be obtained via `unsafe { Pin::<N>::steal() }`,
/// provided the GPIO subsystem has already been initialised.
pub struct Pin<const N: u8> {
    _marker: core::marker::PhantomData<*const ()>,
}

// SAFETY: Pin<N> is a zero-sized ownership token.
// Ownership can safely transfer between cores / ISRs (Send),
// but &Pin<N> must not be shared across cores (!Sync, guaranteed by
// PhantomData<*const ()>), preventing concurrent hardware register access.
unsafe impl<const N: u8> Send for Pin<N> {}

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
        let mask = pico_rs_pac::IO_BANK0::RESET_MASK | pico_rs_pac::PADS_BANK0::RESET_MASK;
        // SAFETY: mask is composed from valid Resettable::RESET_MASK constants.
        unsafe {
            reset::reset(mask);
            reset::unreset(mask);
        }

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
    // Placeholder for unused pins
    NoPin,
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

/// Maximum GPIO pin number for the selected RP2350A variant.
#[cfg(feature = "rp2350a")]
pub const GPIO_PIN_MAX: u8 = 29;

/// Maximum GPIO pin number for the selected RP2350B variant.
#[cfg(feature = "rp2350b")]
pub const GPIO_PIN_MAX: u8 = 47;

// ============================================================================
// Runtime-indexed SIO GPIO access
// ============================================================================

/// Raw SIO GPIO accessor for pins 0-47 on RP2350B, or 0-29 on RP2350A.
///
/// [`Gpio`] is the runtime-indexed counterpart to [`Pin<N>`]. It provides
/// direct access to the SIO GPIO input, output, and output-enable registers,
/// automatically routing pin numbers 0-31 to the low bank and 32-47 to the
/// high bank.
///
/// This type intentionally does not model pin ownership. That omission makes it
/// useful for dynamic pin tables, loops over pin numbers, and mask-based IO, but
/// it also means the compiler can no longer prevent reconfiguration conflicts.
/// For that reason, construction is `unsafe`.
///
/// # Safety
///
/// The caller must ensure all of the following:
///
/// - The GPIO subsystem was initialized first, typically by calling
///   [`Pins::new()`].
/// - Each pin accessed through this handle has already been configured for SIO
///   via [`Pin<N>`] or equivalent low-level setup.
/// - Code using [`Gpio`] does not race with other code that reconfigures the same
///   pins through [`Pin<N>`], peripheral drivers, or direct PAC access.
/// - Ownership decisions remain centralized in the higher-level GPIO API. In
///   practice, [`Gpio`] should be treated as a data-path helper, not as a
///   replacement for pin configuration.
///
/// # Relationship To [`Pin<N>`]
///
/// [`Pin<N>`] remains the safe API for:
///
/// - taking ownership of a specific pin,
/// - selecting the pin function,
/// - configuring pad state and pulls,
/// - configuring direction and interrupts.
///
/// [`Gpio`] is for the narrower case where that setup already happened and the
/// remaining need is efficient runtime-indexed SIO access.
#[derive(Copy, Clone, Default, Debug)]
pub struct Gpio {
    _marker: core::marker::PhantomData<*const ()>,
}

// SAFETY: Gpio is a zero-sized SIO accessor using only atomic SET/CLR/XOR registers.
// Ownership can safely transfer (Send), but shared references must not cross
// cores (!Sync).
unsafe impl Send for Gpio {}

impl Gpio {
    /// Obtain a handle to the SIO GPIO registers.
    ///
    /// The GPIO hardware already exists; this method simply returns a
    /// zero-cost accessor. The returned handle does not own any pins and
    /// can be copied freely.
    ///
    /// # Safety
    ///
    /// See the type-level safety contract on [`Gpio`]. In short: initialize and
    /// configure pins through [`Pins`] and [`Pin<N>`] first, then use [`Gpio`]
    /// only for SIO data access.
    #[inline]
    pub const unsafe fn steal() -> crate::Steal<Self> {
        crate::Steal(Self {
            _marker: core::marker::PhantomData,
        })
    }

    #[inline(always)]
    fn assert_pin(pin: u8) {
        debug_assert!(pin <= GPIO_PIN_MAX);
    }

    // ========================================================================
    // Pin-level operations (auto-routing by pin number)
    // ========================================================================

    /// Read the input value of a single pin.
    ///
    /// Returns `true` if the pin is high.
    #[inline]
    pub fn get(&self, pin: u8) -> bool {
        Self::assert_pin(pin);
        if pin < 32 {
            (crate::sio::regs().gpio_in.gpio_in() & (1 << pin)) != 0
        } else {
            (crate::sio::regs().gpio_hi_in.gpio() & (1 << (pin - 32))) != 0
        }
    }

    /// Set a single pin output high.
    ///
    /// The pin must already be configured for SIO output.
    #[inline]
    pub fn set_high(&self, pin: u8) {
        Self::assert_pin(pin);
        if pin < 32 {
            crate::sio::regs().gpio_out_set.write_gpio_out_set(1 << pin);
        } else {
            crate::sio::regs()
                .gpio_hi_out_set
                .write_gpio(1 << (pin - 32));
        }
    }

    /// Set a single pin output low.
    ///
    /// The pin must already be configured for SIO output.
    #[inline]
    pub fn set_low(&self, pin: u8) {
        Self::assert_pin(pin);
        if pin < 32 {
            crate::sio::regs().gpio_out_clr.write_gpio_out_clr(1 << pin);
        } else {
            crate::sio::regs()
                .gpio_hi_out_clr
                .write_gpio(1 << (pin - 32));
        }
    }

    /// Toggle a single pin output.
    ///
    /// The pin must already be configured for SIO output.
    #[inline]
    pub fn toggle(&self, pin: u8) {
        Self::assert_pin(pin);
        if pin < 32 {
            crate::sio::regs().gpio_out_xor.write_gpio_out_xor(1 << pin);
        } else {
            crate::sio::regs()
                .gpio_hi_out_xor
                .write_gpio(1 << (pin - 32));
        }
    }

    /// Set a single pin output to a specific value.
    ///
    /// This dispatches to [`Self::set_high`] or [`Self::set_low`].
    #[inline]
    pub fn set(&self, pin: u8, high: bool) {
        if high {
            self.set_high(pin);
        } else {
            self.set_low(pin);
        }
    }

    /// Read the programmed output level of a single pin.
    ///
    /// This reads the SIO output register, not the actual pad input state. Use
    /// [`Self::get`] if you need the live pin level.
    #[inline]
    pub fn output(&self, pin: u8) -> bool {
        Self::assert_pin(pin);
        if pin < 32 {
            (crate::sio::regs().gpio_out.gpio_out() & (1 << pin)) != 0
        } else {
            (crate::sio::regs().gpio_hi_out.gpio() & (1 << (pin - 32))) != 0
        }
    }

    /// Enable output for a single pin.
    ///
    /// This only touches the SIO output-enable register. It does not select the
    /// SIO function or configure the pad; that must have been done earlier via
    /// [`Pin<N>`].
    #[inline]
    pub fn oe_set(&self, pin: u8) {
        Self::assert_pin(pin);
        if pin < 32 {
            crate::sio::regs().gpio_oe_set.write_gpio_oe_set(1 << pin);
        } else {
            crate::sio::regs()
                .gpio_hi_oe_set
                .write_gpio(1 << (pin - 32));
        }
    }

    /// Disable output for a single pin.
    #[inline]
    pub fn oe_clr(&self, pin: u8) {
        Self::assert_pin(pin);
        if pin < 32 {
            crate::sio::regs().gpio_oe_clr.write_gpio_oe_clr(1 << pin);
        } else {
            crate::sio::regs()
                .gpio_hi_oe_clr
                .write_gpio(1 << (pin - 32));
        }
    }

    /// Check whether output is enabled for a single pin.
    #[inline]
    pub fn is_output(&self, pin: u8) -> bool {
        Self::assert_pin(pin);
        if pin < 32 {
            (crate::sio::regs().gpio_oe.gpio_oe() & (1 << pin)) != 0
        } else {
            (crate::sio::regs().gpio_hi_oe.gpio() & (1 << (pin - 32))) != 0
        }
    }

    // ========================================================================
    // Mask-level operations - low bank (pins 0-31)
    // ========================================================================

    /// Read the live input state of GPIO 0-31 as a bitmask.
    ///
    /// Bit `n` in the returned value corresponds directly to GPIO `n`.
    #[inline]
    pub fn mask_input_low(&self) -> u32 {
        crate::sio::regs().gpio_in.gpio_in()
    }

    /// Read the programmed SIO output register for GPIO 0-31.
    ///
    /// This reports the value currently stored in `GPIO_OUT`, not the live pad
    /// input level. Use [`Self::mask_input_low`] to sample the pins.
    #[inline]
    pub fn mask_output_low(&self) -> u32 {
        crate::sio::regs().gpio_out.gpio_out()
    }

    /// Replace the entire SIO output register for GPIO 0-31.
    ///
    /// Each bit in `value` maps directly to the output state of the
    /// corresponding GPIO in the low bank.
    #[inline]
    pub fn mask_write_output_low(&self, value: u32) {
        crate::sio::regs().gpio_out.write_gpio_out(value);
    }

    /// Atomically drive selected GPIO 0-31 outputs high.
    ///
    /// Bits that are `1` in `mask` are set; bits that are `0` are left
    /// unchanged.
    #[inline]
    pub fn mask_set_low(&self, mask: u32) {
        crate::sio::regs().gpio_out_set.write_gpio_out_set(mask);
    }

    /// Atomically drive selected GPIO 0-31 outputs low.
    ///
    /// Bits that are `1` in `mask` are cleared; bits that are `0` are left
    /// unchanged.
    #[inline]
    pub fn mask_clr_low(&self, mask: u32) {
        crate::sio::regs().gpio_out_clr.write_gpio_out_clr(mask);
    }

    /// Atomically toggle selected GPIO 0-31 outputs.
    ///
    /// Bits that are `1` in `mask` are inverted; bits that are `0` are left
    /// unchanged.
    #[inline]
    pub fn mask_xor_low(&self, mask: u32) {
        crate::sio::regs().gpio_out_xor.write_gpio_out_xor(mask);
    }

    /// Read the SIO output-enable register for GPIO 0-31.
    ///
    /// A bit value of `1` means the corresponding GPIO is configured as an
    /// output in SIO. A bit value of `0` means input/high-impedance at the SIO
    /// data path level.
    #[inline]
    pub fn mask_oe_low(&self) -> u32 {
        crate::sio::regs().gpio_oe.gpio_oe()
    }

    /// Replace the entire SIO output-enable register for GPIO 0-31.
    ///
    /// Each bit in `value` directly controls whether the corresponding GPIO is
    /// driven as an output (`1`) or left as input (`0`).
    #[inline]
    pub fn mask_write_oe_low(&self, value: u32) {
        crate::sio::regs().gpio_oe.write_gpio_oe(value);
    }

    /// Atomically enable SIO output mode for selected GPIO 0-31 pins.
    #[inline]
    pub fn mask_oe_set_low(&self, mask: u32) {
        crate::sio::regs().gpio_oe_set.write_gpio_oe_set(mask);
    }

    /// Atomically disable SIO output mode for selected GPIO 0-31 pins.
    #[inline]
    pub fn mask_oe_clr_low(&self, mask: u32) {
        crate::sio::regs().gpio_oe_clr.write_gpio_oe_clr(mask);
    }

    /// Atomically toggle SIO output mode for selected GPIO 0-31 pins.
    #[inline]
    pub fn mask_oe_xor_low(&self, mask: u32) {
        crate::sio::regs().gpio_oe_xor.write_gpio_oe_xor(mask);
    }

    // ========================================================================
    // Mask-level operations - high bank (pins 32-47)
    // ========================================================================

    /// Read the live input state of GPIO 32-47 as a compact bitmask.
    ///
    /// Bit `0` corresponds to GPIO 32, bit `1` to GPIO 33, and so on up to
    /// bit `15` for GPIO 47.
    #[inline]
    pub fn mask_input_high(&self) -> u32 {
        crate::sio::regs().gpio_hi_in.gpio()
    }

    /// Read the programmed SIO output register for GPIO 32-47.
    ///
    /// The returned mask uses bits `0..=15` for GPIO 32-47, not bits 32-47.
    #[inline]
    pub fn mask_output_high(&self) -> u32 {
        crate::sio::regs().gpio_hi_out.gpio()
    }

    /// Replace the entire SIO output register for GPIO 32-47.
    ///
    /// Bit `0` in `value` controls GPIO 32, bit `1` controls GPIO 33, and so
    /// on.
    #[inline]
    pub fn mask_write_output_high(&self, value: u32) {
        crate::sio::regs().gpio_hi_out.write_gpio(value);
    }

    /// Atomically drive selected GPIO 32-47 outputs high.
    #[inline]
    pub fn mask_set_high(&self, mask: u32) {
        crate::sio::regs().gpio_hi_out_set.write_gpio(mask);
    }

    /// Atomically drive selected GPIO 32-47 outputs low.
    #[inline]
    pub fn mask_clr_high(&self, mask: u32) {
        crate::sio::regs().gpio_hi_out_clr.write_gpio(mask);
    }

    /// Atomically toggle selected GPIO 32-47 outputs.
    #[inline]
    pub fn mask_xor_high(&self, mask: u32) {
        crate::sio::regs().gpio_hi_out_xor.write_gpio(mask);
    }

    /// Read the SIO output-enable register for GPIO 32-47.
    ///
    /// As with the other high-bank mask APIs, bit `0` corresponds to GPIO 32.
    #[inline]
    pub fn mask_oe_high(&self) -> u32 {
        crate::sio::regs().gpio_hi_oe.gpio()
    }

    /// Replace the entire SIO output-enable register for GPIO 32-47.
    #[inline]
    pub fn mask_write_oe_high(&self, value: u32) {
        crate::sio::regs().gpio_hi_oe.write_gpio(value);
    }

    /// Atomically enable SIO output mode for selected GPIO 32-47 pins.
    #[inline]
    pub fn mask_oe_set_high(&self, mask: u32) {
        crate::sio::regs().gpio_hi_oe_set.write_gpio(mask);
    }

    /// Atomically disable SIO output mode for selected GPIO 32-47 pins.
    #[inline]
    pub fn mask_oe_clr_high(&self, mask: u32) {
        crate::sio::regs().gpio_hi_oe_clr.write_gpio(mask);
    }

    /// Atomically toggle SIO output mode for selected GPIO 32-47 pins.
    #[inline]
    pub fn mask_oe_xor_high(&self, mask: u32) {
        crate::sio::regs().gpio_hi_oe_xor.write_gpio(mask);
    }
}
