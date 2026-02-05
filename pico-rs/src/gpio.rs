//! GPIO (General Purpose Input/Output) HAL for RP2350
//!
//! This module provides a type-safe interface for configuring and using GPIO pins.
//! Each pin can be configured for various functions including SIO, PIO, UART, SPI, I2C, and PWM.
//!
//! # GPIO Architecture
//!
//! The RP2350 GPIO system consists of three main components:
//!
//! ```text
//! ╔══════════════════════════════════════════════════════════════════════════════════╗
//! ║                              GPIO PIN ARCHITECTURE                               ║
//! ╠══════════════════════════════════════════════════════════════════════════════════╣
//! ║                                                                                  ║
//! ║  ┌────────────────────────────────────────────────────────────────────────────┐  ║
//! ║  │                           IO_BANK0 (Control)                               │  ║
//! ║  │  ┌──────────────────────────────────────────────────────────────────────┐  │  ║
//! ║  │  │  Function Selection (funcsel)                                        │  │  ║
//! ║  │  │    0: JTAG   1: SPI     2: UART    3: I2C     4: PWM                  │  │  ║
//! ║  │  │    5: SIO    6: PIO0    7: PIO1    8: PIO2    9: USB                  │  │  ║
//! ║  │  │   31: NULL (disable)                                                  │  │  ║
//! ║  │  └──────────────────────────────────────────────────────────────────────┘  │  ║
//! ║  │  ┌──────────────────────────────────────────────────────────────────────┐  │  ║
//! ║  │  │  Override Controls                                                   │  │  ║
//! ║  │  │    OUTOVER: Output override (normal, invert, low, high)              │  │  ║
//! ║  │  │    OEOVER:  Output enable override                                   │  │  ║
//! ║  │  │    INOVER:  Input override (normal, invert, low, high)               │  │  ║
//! ║  │  │    IRQOVER: Interrupt override                                       │  │  ║
//! ║  │  └──────────────────────────────────────────────────────────────────────┘  │  ║
//! ║  │  ┌──────────────────────────────────────────────────────────────────────┐  │  ║
//! ║  │  │  Interrupts                                                          │  │  ║
//! ║  │  │    INTR:  Raw interrupt status (write to clear edge)                 │  │  ║
//! ║  │  │    INTE:  Interrupt enable per CPU                                   │  │  ║
//! ║  │  │    INTF:  Interrupt force                                            │  │  ║
//! ║  │  │    INTS:  Masked interrupt status                                    │  │  ║
//! ║  │  └──────────────────────────────────────────────────────────────────────┘  │  ║
//! ║  └────────────────────────────────────────────────────────────────────────────┘  ║
//! ║                                                                                  ║
//! ║  ┌────────────────────────────────────────────────────────────────────────────┐  ║
//! ║  │                        PADS_BANK0 (Electrical)                             │  ║
//! ║  │  ┌──────────────────────────────────────────────────────────────────────┐  │  ║
//! ║  │  │  Pad Configuration                                                   │  │  ║
//! ║  │  │    OD:       Output disable (1 = disable output driver)              │  │  ║
//! ║  │  │    IE:       Input enable  (1 = enable input receiver)               │  │  ║
//! ║  │  │    DRIVE:    Drive strength (2/4/8/12 mA)                            │  │  ║
//! ║  │  │    PUE:      Pull-up enable                                          │  │  ║
//! ║  │  │    PDE:      Pull-down enable                                        │  │  ║
//! ║  │  │    SCHMITT:  Schmitt trigger enable                                  │  │  ║
//! ║  │  │    SLEWFAST: Slew rate (0 = slow, 1 = fast)                          │  │  ║
//! ║  │  │    ISO:      Isolation (for pad retention during sleep)              │  │  ║
//! ║  │  └──────────────────────────────────────────────────────────────────────┘  │  ║
//! ║  └────────────────────────────────────────────────────────────────────────────┘  ║
//! ║                                                                                  ║
//! ║  ┌────────────────────────────────────────────────────────────────────────────┐  ║
//! ║  │                         SIO (Single-cycle I/O)                             │  ║
//! ║  │  ┌──────────────────────────────────────────────────────────────────────┐  │  ║
//! ║  │  │  GPIO Registers (when funcsel = SIO)                                 │  │  ║
//! ║  │  │    GPIO_IN:     Read input values                                    │  │  ║
//! ║  │  │    GPIO_OUT:    Output level                                         │  │  ║
//! ║  │  │    GPIO_OE:     Output enable (direction)                            │  │  ║
//! ║  │  │    *_SET/CLR/XOR: Atomic set/clear/toggle registers                  │  │  ║
//! ║  │  │    GPIO_HI_*:   Registers for pins 32-47 (RP2350B only)              │  │  ║
//! ║  │  └──────────────────────────────────────────────────────────────────────┘  │  ║
//! ║  └────────────────────────────────────────────────────────────────────────────┘  ║
//! ║                                                                                  ║
//! ╚══════════════════════════════════════════════════════════════════════════════════╝
//! ```
//!
//! # Pin Count by Variant
//!
//! | Variant  | GPIO Pins | Notes                        |
//! |----------|-----------|------------------------------|
//! | RP2350A  | 30 (0-29) | Pico 2 board                 |
//! | RP2350B  | 48 (0-47) | QFN-80 package               |
//!
//! # GPIO Function Mapping
//!
//! Each GPIO pin can be assigned to one of several peripheral functions:
//!
//! ```text
//! ┌───────┬────────────────────────────────────────────────────────────────┐
//! │ GPIO  │  F1(SPI)   F2(UART)  F3(I2C)  F4(PWM)   F5(SIO)  F6/7/8(PIO)  │
//! ├───────┼────────────────────────────────────────────────────────────────┤
//! │  0    │  SPI0 RX   UART0 TX  I2C0 SDA  PWM0 A    SIO      PIO          │
//! │  1    │  SPI0 CSn  UART0 RX  I2C0 SCL  PWM0 B    SIO      PIO          │
//! │  2    │  SPI0 SCK  UART0 CTS I2C1 SDA  PWM1 A    SIO      PIO          │
//! │  3    │  SPI0 TX   UART0 RTS I2C1 SCL  PWM1 B    SIO      PIO          │
//! │  4    │  SPI0 RX   UART1 TX  I2C0 SDA  PWM2 A    SIO      PIO          │
//! │  5    │  SPI0 CSn  UART1 RX  I2C0 SCL  PWM2 B    SIO      PIO          │
//! │  6    │  SPI0 SCK  UART1 CTS I2C1 SDA  PWM3 A    SIO      PIO          │
//! │  7    │  SPI0 TX   UART1 RTS I2C1 SCL  PWM3 B    SIO      PIO          │
//! │  8    │  SPI1 RX   UART1 TX  I2C0 SDA  PWM4 A    SIO      PIO          │
//! │  9    │  SPI1 CSn  UART1 RX  I2C0 SCL  PWM4 B    SIO      PIO          │
//! │  10   │  SPI1 SCK  UART1 CTS I2C1 SDA  PWM5 A    SIO      PIO          │
//! │  11   │  SPI1 TX   UART1 RTS I2C1 SCL  PWM5 B    SIO      PIO          │
//! │  12   │  SPI1 RX   UART0 TX  I2C0 SDA  PWM6 A    SIO      PIO          │
//! │  13   │  SPI1 CSn  UART0 RX  I2C0 SCL  PWM6 B    SIO      PIO          │
//! │  14   │  SPI1 SCK  UART0 CTS I2C1 SDA  PWM7 A    SIO      PIO          │
//! │  15   │  SPI1 TX   UART0 RTS I2C1 SCL  PWM7 B    SIO      PIO          │
//! │  ...  │  (pattern repeats with variations)                             │
//! │  25   │                                PWM4 B    SIO(LED) PIO          │
//! │  26   │  ADC0                          PWM5 A    SIO      PIO          │
//! │  27   │  ADC1                          PWM5 B    SIO      PIO          │
//! │  28   │  ADC2                          PWM6 A    SIO      PIO          │
//! │  29   │  ADC3                          PWM6 B    SIO      PIO          │
//! └───────┴────────────────────────────────────────────────────────────────┘
//! ```
//!
//! # Initialization Requirements
//!
//! **Important**: Before using GPIO pins, you must bring the `IO_BANK0` and `PADS_BANK0`
//! peripherals out of reset. Use the [`init_gpio()`] function:
//!
//! ```ignore
//! use pico_rs::gpio;
//!
//! // Initialize GPIO subsystem (unresets IO_BANK0 and PADS_BANK0)
//! gpio::init_gpio();
//!
//! // Now you can use pins
//! let led = gpio::Pin25::new();
//! led.into_output();
//! led.set_high();
//! ```
//!
//! If you skip this step, GPIO operations may have no effect or behave unexpectedly.
//!
//! # Usage Examples
//!
//! ## Basic Output (LED Blink)
//!
//! ```ignore
//! use pico_rs::gpio;
//!
//! gpio::init_gpio();
//! let led = gpio::Pin25::new();
//! led.into_output();
//!
//! loop {
//!     led.toggle();
//!     // delay...
//! }
//! ```
//!
//! ## Basic Input (Button)
//!
//! ```ignore
//! use pico_rs::gpio;
//!
//! gpio::init_gpio();
//! let button = gpio::Pin14::new();
//! button.into_input_pullup();
//!
//! if !button.read() {  // Active low
//!     // Button pressed
//! }
//! ```
//!
//! ## Using PinBuilder for Complex Configuration
//!
//! ```ignore
//! use pico_rs::gpio::{self, DriveStrength, SlewRate};
//!
//! gpio::init_gpio();
//!
//! // Configure a pin with specific electrical characteristics
//! let pin = gpio::Pin25::new();
//! pin.builder()
//!     .output()
//!     .drive_strength(DriveStrength::Ma8)
//!     .slew_rate(SlewRate::Fast)
//!     .build();
//!
//! pin.set_high();
//! ```
//!
//! ## Configuring for Peripheral Functions
//!
//! ```ignore
//! use pico_rs::gpio;
//!
//! gpio::init_gpio();
//!
//! // Configure for UART
//! let tx_pin = gpio::Pin0::new();
//! tx_pin.init_pad();
//! tx_pin.set_function(gpio::Function::Uart);
//!
//! // Configure for PWM
//! let pwm_pin = gpio::Pin16::new();
//! pwm_pin.into_pwm();
//! ```
//!
//! ## Edge Interrupt
//!
//! ```ignore
//! use pico_rs::gpio::{self, EdgeLevel};
//!
//! gpio::init_gpio();
//! let button = gpio::Pin14::new();
//! button.into_input_pullup();
//! button.enable_interrupt(EdgeLevel::EdgeLow);
//!
//! // In interrupt handler:
//! // button.clear_interrupt(EdgeLevel::EdgeLow);
//! ```
//!
//! # Safety Considerations
//!
//! - [`init_gpio()`] should be called once before using any GPIO pins
//! - Multiple instances of the same pin type can exist (no ownership enforcement)
//! - The user is responsible for avoiding conflicting configurations

#![allow(clippy::identity_op)]

use paste::paste;
use pico_rs_pac::{IO_BANK0, PADS_BANK0, SIO};

use crate::reset::{self, Peripheral};

// ============================================================================
// GPIO Subsystem Initialization
// ============================================================================

/// Initialize the GPIO subsystem by bringing IO_BANK0 and PADS_BANK0 out of reset.
///
/// This function must be called before using any GPIO pins. It performs
/// a reset cycle on both peripherals to ensure they start in a known state.
///
/// # What this does
///
/// 1. Resets `IO_BANK0` (GPIO control registers)
/// 2. Resets `PADS_BANK0` (GPIO electrical configuration)
/// 3. Waits for both peripherals to complete their reset cycle
///
/// # Example
///
/// ```ignore
/// use pico_rs::gpio;
///
/// // Call once at startup
/// gpio::init_gpio();
///
/// // Now GPIO pins can be used
/// let led = gpio::Pin25::new();
/// led.into_output();
/// ```
///
/// # Note
///
/// This function is idempotent - calling it multiple times is safe but
/// will reset all GPIO configurations each time.
#[inline]
pub fn init_gpio() {
    // Reset both peripherals
    let mask = Peripheral::IO_BANK0.mask() | Peripheral::PADS_BANK0.mask();
    reset::reset_block_mask(mask);
    reset::unreset_block_mask(mask);
    reset::unreset_wait_mask(mask);
}

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
            #[doc = concat!("GPIO Pin ", stringify!($pin_num))]
            ///
            /// This type provides access to a specific GPIO pin with type-safe configuration
            /// methods. Create an instance using [`new()`](Self::new) after calling [`init_gpio()`].
            ///
            /// # Example
            ///
            /// ```ignore
            #[doc = concat!("use pico_rs::gpio::{self, ", stringify!($PinName), "};")]
            ///
            /// gpio::init_gpio();
            #[doc = concat!("let pin = ", stringify!($PinName), "::new();")]
            /// pin.into_output();
            /// pin.set_high();
            /// ```
            #[derive(Clone, Copy)]
            pub struct $PinName {
                io_bank: &'static pico_rs_pac::io_bank0::RegisterBlock,
                pads_bank: &'static pico_rs_pac::pads_bank0::RegisterBlock,
                sio: &'static pico_rs_pac::sio::RegisterBlock,
            }

            impl $PinName {
                /// Pin number
                pub const PIN: u8 = $pin_num;

                /// Pin mask for SIO operations
                pub const MASK: u32 = 1u32 << ($pin_num % 32);

                /// Whether this is a high GPIO (32-47)
                pub const IS_HIGH: bool = $pin_num >= 32;

                /// Create a new pin instance.
                ///
                /// # Prerequisites
                ///
                /// You must call [`init_gpio()`] before creating pin instances to ensure
                /// the GPIO peripherals (IO_BANK0 and PADS_BANK0) are out of reset.
                ///
                /// # Example
                ///
                /// ```ignore
                /// use pico_rs::gpio;
                ///
                /// gpio::init_gpio();
                #[doc = concat!("let pin = gpio::", stringify!($PinName), "::new();")]
                /// pin.into_output();
                /// ```
                #[inline]
                pub fn new() -> Self {
                    unsafe {
                        Self {
                            io_bank: &*IO_BANK0::PTR,
                            pads_bank: &*PADS_BANK0::PTR,
                            sio: &*SIO::PTR,
                        }
                    }
                }

                /// Create a builder for fluent configuration of this pin.
                ///
                /// # Example
                ///
                /// ```ignore
                /// use pico_rs::gpio::{self, DriveStrength, SlewRate};
                ///
                /// gpio::init_gpio();
                #[doc = concat!("let pin = gpio::", stringify!($PinName), "::new();")]
                /// pin.builder()
                ///     .output()
                ///     .drive_strength(DriveStrength::_8mA)
                ///     .slew_rate(SlewRate::Fast)
                ///     .build();
                /// ```
                #[inline]
                pub fn builder(&self) -> [<$PinName Builder>] {
                    [<$PinName Builder>]::new(*self)
                }

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

                /// Initialize as PWM pin
                #[inline]
                pub fn into_pwm(&self) -> &Self {
                    self.init_pad();
                    self.set_function(Function::Pwm);
                    self
                }
            }

            // ================================================================
            // Default trait implementation
            // ================================================================

            impl Default for $PinName {
                fn default() -> Self {
                    Self::new()
                }
            }

            // ================================================================
            // Debug trait implementation
            // ================================================================

            impl core::fmt::Debug for $PinName {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.debug_struct(stringify!($PinName))
                        .field("pin", &Self::PIN)
                        .field("function", &self.function())
                        .field("is_output", &self.is_output())
                        .finish()
                }
            }

            // ================================================================
            // PinBuilder for fluent configuration
            // ================================================================

            #[doc = concat!("Builder for configuring ", stringify!($PinName))]
            ///
            /// Provides a fluent interface for configuring pin parameters.
            /// Call [`build()`](Self::build) to apply the configuration.
            ///
            /// # Example
            ///
            /// ```ignore
            /// use pico_rs::gpio::{self, DriveStrength, SlewRate};
            ///
            /// gpio::init_gpio();
            #[doc = concat!("let pin = gpio::", stringify!($PinName), "::new();")]
            /// pin.builder()
            ///     .output()
            ///     .drive_strength(DriveStrength::_8mA)
            ///     .slew_rate(SlewRate::Fast)
            ///     .pull(Pull::None)
            ///     .build();
            /// ```
            pub struct [<$PinName Builder>] {
                pin: $PinName,
                function: Option<Function>,
                direction_out: Option<bool>,
                pull: Option<Pull>,
                drive_strength: Option<DriveStrength>,
                slew_rate: Option<SlewRate>,
                schmitt_trigger: Option<bool>,
            }

            impl [<$PinName Builder>] {
                /// Create a new builder for the given pin
                #[inline]
                fn new(pin: $PinName) -> Self {
                    Self {
                        pin,
                        function: None,
                        direction_out: None,
                        pull: None,
                        drive_strength: None,
                        slew_rate: None,
                        schmitt_trigger: None,
                    }
                }

                /// Set the pin function
                #[inline]
                pub fn function(mut self, func: Function) -> Self {
                    self.function = Some(func);
                    self
                }

                /// Configure as SIO output
                #[inline]
                pub fn output(mut self) -> Self {
                    self.function = Some(Function::Sio);
                    self.direction_out = Some(true);
                    self
                }

                /// Configure as SIO input
                #[inline]
                pub fn input(mut self) -> Self {
                    self.function = Some(Function::Sio);
                    self.direction_out = Some(false);
                    self
                }

                /// Set pull resistor configuration
                #[inline]
                pub fn pull(mut self, pull: Pull) -> Self {
                    self.pull = Some(pull);
                    self
                }

                /// Enable pull-up resistor
                #[inline]
                pub fn pull_up(self) -> Self {
                    self.pull(Pull::Up)
                }

                /// Enable pull-down resistor
                #[inline]
                pub fn pull_down(self) -> Self {
                    self.pull(Pull::Down)
                }

                /// Set drive strength
                #[inline]
                pub fn drive_strength(mut self, strength: DriveStrength) -> Self {
                    self.drive_strength = Some(strength);
                    self
                }

                /// Set slew rate
                #[inline]
                pub fn slew_rate(mut self, rate: SlewRate) -> Self {
                    self.slew_rate = Some(rate);
                    self
                }

                /// Enable or disable Schmitt trigger
                #[inline]
                pub fn schmitt_trigger(mut self, enable: bool) -> Self {
                    self.schmitt_trigger = Some(enable);
                    self
                }

                /// Apply the configuration and return the pin
                #[inline]
                pub fn build(self) -> $PinName {
                    // Initialize pad first
                    self.pin.init_pad();

                    // Apply function if set
                    if let Some(func) = self.function {
                        self.pin.set_function(func);
                    }

                    // Apply direction if set (only for SIO)
                    if let Some(is_output) = self.direction_out {
                        if is_output {
                            self.pin.set_direction_out();
                        } else {
                            self.pin.set_direction_in();
                        }
                    }

                    // Apply pull if set
                    if let Some(pull) = self.pull {
                        self.pin.set_pull(pull);
                    }

                    // Apply drive strength if set
                    if let Some(strength) = self.drive_strength {
                        self.pin.set_drive_strength(strength);
                    }

                    // Apply slew rate if set
                    if let Some(rate) = self.slew_rate {
                        self.pin.set_slew_rate(rate);
                    }

                    // Apply Schmitt trigger if set
                    if let Some(enable) = self.schmitt_trigger {
                        if enable {
                            self.pin.set_schmitt(true);
                        } else {
                            self.pin.set_schmitt(false);
                        }
                    }

                    self.pin
                }
            }
        }
    };
}

// ============================================================================
// Macro for generating all pins
// ============================================================================

macro_rules! impl_gpio_pins {
    ($($pin_num:literal => $PinName:ident => [$inte_reg:ident, $intr_reg:ident, $ints_reg:ident]),+ $(,)?) => {
        paste! {
            // Generate all pin types
            $(
                impl_gpio_pin!($PinName, $pin_num, $inte_reg, $intr_reg, $ints_reg);
            )+
        }
    };
}

// ============================================================================
// Generate GPIO pins based on chip variant
// ============================================================================

// RP2350A: 30 GPIO pins (0-29)
#[cfg(feature = "rp2350a")]
impl_gpio_pins!(
    0 => Pin0 => [proc0_inte0, intr0, proc0_ints0],
    1 => Pin1 => [proc0_inte0, intr0, proc0_ints0],
    2 => Pin2 => [proc0_inte0, intr0, proc0_ints0],
    3 => Pin3 => [proc0_inte0, intr0, proc0_ints0],
    4 => Pin4 => [proc0_inte0, intr0, proc0_ints0],
    5 => Pin5 => [proc0_inte0, intr0, proc0_ints0],
    6 => Pin6 => [proc0_inte0, intr0, proc0_ints0],
    7 => Pin7 => [proc0_inte0, intr0, proc0_ints0],
    8 => Pin8 => [proc0_inte1, intr1, proc0_ints1],
    9 => Pin9 => [proc0_inte1, intr1, proc0_ints1],
    10 => Pin10 => [proc0_inte1, intr1, proc0_ints1],
    11 => Pin11 => [proc0_inte1, intr1, proc0_ints1],
    12 => Pin12 => [proc0_inte1, intr1, proc0_ints1],
    13 => Pin13 => [proc0_inte1, intr1, proc0_ints1],
    14 => Pin14 => [proc0_inte1, intr1, proc0_ints1],
    15 => Pin15 => [proc0_inte1, intr1, proc0_ints1],
    16 => Pin16 => [proc0_inte2, intr2, proc0_ints2],
    17 => Pin17 => [proc0_inte2, intr2, proc0_ints2],
    18 => Pin18 => [proc0_inte2, intr2, proc0_ints2],
    19 => Pin19 => [proc0_inte2, intr2, proc0_ints2],
    20 => Pin20 => [proc0_inte2, intr2, proc0_ints2],
    21 => Pin21 => [proc0_inte2, intr2, proc0_ints2],
    22 => Pin22 => [proc0_inte2, intr2, proc0_ints2],
    23 => Pin23 => [proc0_inte2, intr2, proc0_ints2],
    24 => Pin24 => [proc0_inte3, intr3, proc0_ints3],
    25 => Pin25 => [proc0_inte3, intr3, proc0_ints3],
    26 => Pin26 => [proc0_inte3, intr3, proc0_ints3],
    27 => Pin27 => [proc0_inte3, intr3, proc0_ints3],
    28 => Pin28 => [proc0_inte3, intr3, proc0_ints3],
    29 => Pin29 => [proc0_inte3, intr3, proc0_ints3],
);

// RP2350B: 48 GPIO pins (0-47)
#[cfg(feature = "rp2350b")]
impl_gpio_pins!(
    0 => Pin0 => [proc0_inte0, intr0, proc0_ints0],
    1 => Pin1 => [proc0_inte0, intr0, proc0_ints0],
    2 => Pin2 => [proc0_inte0, intr0, proc0_ints0],
    3 => Pin3 => [proc0_inte0, intr0, proc0_ints0],
    4 => Pin4 => [proc0_inte0, intr0, proc0_ints0],
    5 => Pin5 => [proc0_inte0, intr0, proc0_ints0],
    6 => Pin6 => [proc0_inte0, intr0, proc0_ints0],
    7 => Pin7 => [proc0_inte0, intr0, proc0_ints0],
    8 => Pin8 => [proc0_inte1, intr1, proc0_ints1],
    9 => Pin9 => [proc0_inte1, intr1, proc0_ints1],
    10 => Pin10 => [proc0_inte1, intr1, proc0_ints1],
    11 => Pin11 => [proc0_inte1, intr1, proc0_ints1],
    12 => Pin12 => [proc0_inte1, intr1, proc0_ints1],
    13 => Pin13 => [proc0_inte1, intr1, proc0_ints1],
    14 => Pin14 => [proc0_inte1, intr1, proc0_ints1],
    15 => Pin15 => [proc0_inte1, intr1, proc0_ints1],
    16 => Pin16 => [proc0_inte2, intr2, proc0_ints2],
    17 => Pin17 => [proc0_inte2, intr2, proc0_ints2],
    18 => Pin18 => [proc0_inte2, intr2, proc0_ints2],
    19 => Pin19 => [proc0_inte2, intr2, proc0_ints2],
    20 => Pin20 => [proc0_inte2, intr2, proc0_ints2],
    21 => Pin21 => [proc0_inte2, intr2, proc0_ints2],
    22 => Pin22 => [proc0_inte2, intr2, proc0_ints2],
    23 => Pin23 => [proc0_inte2, intr2, proc0_ints2],
    24 => Pin24 => [proc0_inte3, intr3, proc0_ints3],
    25 => Pin25 => [proc0_inte3, intr3, proc0_ints3],
    26 => Pin26 => [proc0_inte3, intr3, proc0_ints3],
    27 => Pin27 => [proc0_inte3, intr3, proc0_ints3],
    28 => Pin28 => [proc0_inte3, intr3, proc0_ints3],
    29 => Pin29 => [proc0_inte3, intr3, proc0_ints3],
    30 => Pin30 => [proc0_inte3, intr3, proc0_ints3],
    31 => Pin31 => [proc0_inte3, intr3, proc0_ints3],
    32 => Pin32 => [proc0_inte4, intr4, proc0_ints4],
    33 => Pin33 => [proc0_inte4, intr4, proc0_ints4],
    34 => Pin34 => [proc0_inte4, intr4, proc0_ints4],
    35 => Pin35 => [proc0_inte4, intr4, proc0_ints4],
    36 => Pin36 => [proc0_inte4, intr4, proc0_ints4],
    37 => Pin37 => [proc0_inte4, intr4, proc0_ints4],
    38 => Pin38 => [proc0_inte4, intr4, proc0_ints4],
    39 => Pin39 => [proc0_inte4, intr4, proc0_ints4],
    40 => Pin40 => [proc0_inte5, intr5, proc0_ints5],
    41 => Pin41 => [proc0_inte5, intr5, proc0_ints5],
    42 => Pin42 => [proc0_inte5, intr5, proc0_ints5],
    43 => Pin43 => [proc0_inte5, intr5, proc0_ints5],
    44 => Pin44 => [proc0_inte5, intr5, proc0_ints5],
    45 => Pin45 => [proc0_inte5, intr5, proc0_ints5],
    46 => Pin46 => [proc0_inte5, intr5, proc0_ints5],
    47 => Pin47 => [proc0_inte5, intr5, proc0_ints5],
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
    Uart0RxPin,
    Uart0TxPin,
    Uart1CtsPin,
    Uart1RtsPin,
    Uart1RxPin,
    Uart1TxPin,
};
