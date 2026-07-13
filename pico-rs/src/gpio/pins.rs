//! GPIO Pin Function Mapping for RP2350
//!
//! This module provides compile-time verified GPIO pin function mappings
//! using Rust's trait system. It enables type-safe peripheral initialization
//! where only valid pins can be used with each peripheral.
//!
//! # Design Philosophy
//!
//! The RP2350 has strict pin-to-peripheral mappings defined in its datasheet.
//! This module encodes these mappings as traits, allowing:
//!
//! 1. **Compile-time verification**: Invalid pin assignments fail to compile
//! 2. **Automatic configuration**: Peripherals configure pins automatically
//! 3. **Ownership transfer**: Pins are consumed by peripherals, preventing reuse
//!
//! # Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────────────────┐
//! │                     Pin Trait Hierarchy                                  │
//! ├─────────────────────────────────────────────────────────────────────────┤
//! │                                                                         │
//! │  GpioPin (base trait)                                                   │
//! │      │                                                                  │
//! │      ├── I2c0SdaPin ──────► Pin<0>, Pin<4>, Pin<8>, Pin<12>, ...       │
//! │      ├── I2c0SclPin ──────► Pin<1>, Pin<5>, Pin<9>, Pin<13>, ...       │
//! │      ├── I2c1SdaPin ──────► Pin<2>, Pin<6>, Pin<10>, Pin<14>, ...      │
//! │      ├── I2c1SclPin ──────► Pin<3>, Pin<7>, Pin<11>, Pin<15>, ...      │
//! │      │                                                                  │
//! │      ├── Uart0TxPin ──────► Pin<0>, Pin<12>, Pin<16>, Pin<28>          │
//! │      ├── Uart0RxPin ──────► Pin<1>, Pin<13>, Pin<17>, Pin<29>          │
//! │      ├── ...                                                            │
//! │      │                                                                  │
//! │      ├── Spi0RxPin ───────► Pin<0>, Pin<4>, Pin<16>, Pin<20>           │
//! │      ├── ...                                                            │
//! │      │                                                                  │
//! │      └── PwmAPin<SLICE> ──► Slice-specific pins                        │
//! │                                                                         │
//! └─────────────────────────────────────────────────────────────────────────┘
//! ```
//!
//! # Usage Example
//!
//! ```ignore
//! use pico_rs::gpio::{Pins, Pin};
//! use pico_rs::i2c::I2c0;
//!
//! let dp = Peripherals::take().unwrap();
//! let pins = Pins::init(dp.IO_BANK0, dp.PADS_BANK0);
//!
//! // I2C0::new requires pins that implement I2c0SdaPin and I2c0SclPin
//! // Pin<4> implements I2c0SdaPin, Pin<5> implements I2c0SclPin
//! let i2c = I2c0::init(dp.I2C0, pins.pin::<4>(), pins.pin::<5>(), 400_000, sys_clk);
//!
//! // Compile error: Pin<3> does not implement I2c0SdaPin!
//! // let i2c = I2c0::init(dp.I2C0, pins.pin::<3>(), pins.pin::<5>(), 400_000, sys_clk);
//!
//! // Compile error: pins.pin::<4>() already moved!
//! // pins.pin::<4>().into_output();
//! ```

use super::{Function, Pin};

// ============================================================================
// I2C Pin Traits
// ============================================================================

/// Marker trait for pins that can be used as I2C0 SDA.
///
/// Valid pins: 0, 4, 8, 12, 16, 20, 24, 28 (RP2350A)
/// Additional: 32, 36, 40, 44 (RP2350B)
pub trait I2c0SdaPin {
    /// Configure this pin for I2C0 SDA function
    fn configure_i2c0_sda(&self);
}

/// Marker trait for pins that can be used as I2C0 SCL.
///
/// Valid pins: 1, 5, 9, 13, 17, 21, 25, 29 (RP2350A)
/// Additional: 33, 37, 41, 45 (RP2350B)
pub trait I2c0SclPin {
    /// Configure this pin for I2C0 SCL function
    fn configure_i2c0_scl(&self);
}

/// Marker trait for pins that can be used as I2C1 SDA.
///
/// Valid pins: 2, 6, 10, 14, 18, 22, 26 (RP2350A)
/// Additional: 30, 34, 38, 42, 46 (RP2350B)
pub trait I2c1SdaPin {
    /// Configure this pin for I2C1 SDA function
    fn configure_i2c1_sda(&self);
}

/// Marker trait for pins that can be used as I2C1 SCL.
///
/// Valid pins: 3, 7, 11, 15, 19, 23, 27 (RP2350A)
/// Additional: 31, 35, 39, 43, 47 (RP2350B)
pub trait I2c1SclPin {
    /// Configure this pin for I2C1 SCL function
    fn configure_i2c1_scl(&self);
}

/// Macro for implementing I2C pin traits
macro_rules! impl_i2c_pins {
    // I2C0 SDA
    (i2c0_sda: $($n:literal),+ $(,)?) => {
        $(
            impl I2c0SdaPin for Pin<$n> {
                #[inline]
                fn configure_i2c0_sda(&self) {
                    self.init_pad();
                    self.set_function(Function::I2c);
                    self.set_pull_up();  // I2C requires pull-up
                }
            }
        )+
    };
    // I2C0 SCL
    (i2c0_scl: $($n:literal),+ $(,)?) => {
        $(
            impl I2c0SclPin for Pin<$n> {
                #[inline]
                fn configure_i2c0_scl(&self) {
                    self.init_pad();
                    self.set_function(Function::I2c);
                    self.set_pull_up();  // I2C requires pull-up
                }
            }
        )+
    };
    // I2C1 SDA
    (i2c1_sda: $($n:literal),+ $(,)?) => {
        $(
            impl I2c1SdaPin for Pin<$n> {
                #[inline]
                fn configure_i2c1_sda(&self) {
                    self.init_pad();
                    self.set_function(Function::I2c);
                    self.set_pull_up();
                }
            }
        )+
    };
    // I2C1 SCL
    (i2c1_scl: $($n:literal),+ $(,)?) => {
        $(
            impl I2c1SclPin for Pin<$n> {
                #[inline]
                fn configure_i2c1_scl(&self) {
                    self.init_pad();
                    self.set_function(Function::I2c);
                    self.set_pull_up();
                }
            }
        )+
    };
}

// ============================================================================
// UART Pin Traits
// ============================================================================

/// Marker trait for pins that can be used as UART0 TX.
///
/// Valid pins: 0, 12, 16, 28 (RP2350A)
pub trait Uart0TxPin {
    /// Configure this pin for UART0 TX function
    fn configure_uart0_tx(&self);
}

/// Marker trait for pins that can be used as UART0 RX.
///
/// Valid pins: 1, 13, 17, 29 (RP2350A)
pub trait Uart0RxPin {
    /// Configure this pin for UART0 RX function
    fn configure_uart0_rx(&self);
}

/// Marker trait for pins that can be used as UART0 CTS.
///
/// Valid pins: 2, 14, 18 (RP2350A)
pub trait Uart0CtsPin {
    /// Configure this pin for UART0 CTS function
    fn configure_uart0_cts(&self);
}

/// Marker trait for pins that can be used as UART0 RTS.
///
/// Valid pins: 3, 15, 19 (RP2350A)
pub trait Uart0RtsPin {
    /// Configure this pin for UART0 RTS function
    fn configure_uart0_rts(&self);
}

/// Marker trait for pins that can be used as UART1 TX.
///
/// Valid pins: 4, 8, 20, 24 (RP2350A)
pub trait Uart1TxPin {
    /// Configure this pin for UART1 TX function
    fn configure_uart1_tx(&self);
}

/// Marker trait for pins that can be used as UART1 RX.
///
/// Valid pins: 5, 9, 21, 25 (RP2350A)
pub trait Uart1RxPin {
    /// Configure this pin for UART1 RX function
    fn configure_uart1_rx(&self);
}

/// Marker trait for pins that can be used as UART1 CTS.
///
/// Valid pins: 6, 10, 22, 26 (RP2350A)
pub trait Uart1CtsPin {
    /// Configure this pin for UART1 CTS function
    fn configure_uart1_cts(&self);
}

/// Marker trait for pins that can be used as UART1 RTS.
///
/// Valid pins: 7, 11, 23, 27 (RP2350A)
pub trait Uart1RtsPin {
    /// Configure this pin for UART1 RTS function
    fn configure_uart1_rts(&self);
}

/// Macro for implementing UART pin traits
macro_rules! impl_uart_pins {
    (uart0_tx: $($n:literal),+ $(,)?) => {
        $(
            impl Uart0TxPin for Pin<$n> {
                #[inline]
                fn configure_uart0_tx(&self) {
                    self.init_pad();
                    self.set_function(Function::Uart);
                }
            }
        )+
    };
    (uart0_rx: $($n:literal),+ $(,)?) => {
        $(
            impl Uart0RxPin for Pin<$n> {
                #[inline]
                fn configure_uart0_rx(&self) {
                    self.init_pad();
                    self.set_function(Function::Uart);
                }
            }
        )+
    };
    (uart0_cts: $($n:literal),+ $(,)?) => {
        $(
            impl Uart0CtsPin for Pin<$n> {
                #[inline]
                fn configure_uart0_cts(&self) {
                    self.init_pad();
                    self.set_function(Function::Uart);
                }
            }
        )+
    };
    (uart0_rts: $($n:literal),+ $(,)?) => {
        $(
            impl Uart0RtsPin for Pin<$n> {
                #[inline]
                fn configure_uart0_rts(&self) {
                    self.init_pad();
                    self.set_function(Function::Uart);
                }
            }
        )+
    };
    (uart1_tx: $($n:literal),+ $(,)?) => {
        $(
            impl Uart1TxPin for Pin<$n> {
                #[inline]
                fn configure_uart1_tx(&self) {
                    self.init_pad();
                    self.set_function(Function::Uart);
                }
            }
        )+
    };
    (uart1_rx: $($n:literal),+ $(,)?) => {
        $(
            impl Uart1RxPin for Pin<$n> {
                #[inline]
                fn configure_uart1_rx(&self) {
                    self.init_pad();
                    self.set_function(Function::Uart);
                }
            }
        )+
    };
    (uart1_cts: $($n:literal),+ $(,)?) => {
        $(
            impl Uart1CtsPin for Pin<$n> {
                #[inline]
                fn configure_uart1_cts(&self) {
                    self.init_pad();
                    self.set_function(Function::Uart);
                }
            }
        )+
    };
    (uart1_rts: $($n:literal),+ $(,)?) => {
        $(
            impl Uart1RtsPin for Pin<$n> {
                #[inline]
                fn configure_uart1_rts(&self) {
                    self.init_pad();
                    self.set_function(Function::Uart);
                }
            }
        )+
    };
}

// ============================================================================
// SPI Pin Traits
// ============================================================================

/// Marker trait for pins that can be used as SPI0 RX (MISO).
///
/// Valid pins: 0, 4, 16, 20 (RP2350A)
pub trait Spi0RxPin {
    /// Configure this pin for SPI0 RX function
    fn configure_spi0_rx(&self);
}

/// Marker trait for pins that can be used as SPI0 CSn (Chip Select).
///
/// Valid pins: 1, 5, 17, 21 (RP2350A)
pub trait Spi0CsnPin {
    /// Configure this pin for SPI0 CSn function
    fn configure_spi0_csn(&self);
}

/// Marker trait for pins that can be used as SPI0 SCK (Clock).
///
/// Valid pins: 2, 6, 18, 22 (RP2350A)
pub trait Spi0SckPin {
    /// Configure this pin for SPI0 SCK function
    fn configure_spi0_sck(&self);
}

/// Marker trait for pins that can be used as SPI0 TX (MOSI).
///
/// Valid pins: 3, 7, 19, 23 (RP2350A)
pub trait Spi0TxPin {
    /// Configure this pin for SPI0 TX function
    fn configure_spi0_tx(&self);
}

/// Marker trait for pins that can be used as SPI1 RX (MISO).
///
/// Valid pins: 8, 12, 24, 28 (RP2350A)
pub trait Spi1RxPin {
    /// Configure this pin for SPI1 RX function
    fn configure_spi1_rx(&self);
}

/// Marker trait for pins that can be used as SPI1 CSn (Chip Select).
///
/// Valid pins: 9, 13, 25, 29 (RP2350A)
pub trait Spi1CsnPin {
    /// Configure this pin for SPI1 CSn function
    fn configure_spi1_csn(&self);
}

/// Marker trait for pins that can be used as SPI1 SCK (Clock).
///
/// Valid pins: 10, 14, 26 (RP2350A)
pub trait Spi1SckPin {
    /// Configure this pin for SPI1 SCK function
    fn configure_spi1_sck(&self);
}

/// Marker trait for pins that can be used as SPI1 TX (MOSI).
///
/// Valid pins: 11, 15, 27 (RP2350A)
pub trait Spi1TxPin {
    /// Configure this pin for SPI1 TX function
    fn configure_spi1_tx(&self);
}

/// Macro for implementing SPI pin traits
macro_rules! impl_spi_pins {
    (spi0_rx: $($n:literal),+ $(,)?) => {
        $(
            impl Spi0RxPin for Pin<$n> {
                #[inline]
                fn configure_spi0_rx(&self) {
                    self.init_pad();
                    self.set_function(Function::Spi);
                }
            }
        )+
    };
    (spi0_csn: $($n:literal),+ $(,)?) => {
        $(
            impl Spi0CsnPin for Pin<$n> {
                #[inline]
                fn configure_spi0_csn(&self) {
                    self.init_pad();
                    self.set_function(Function::Spi);
                }
            }
        )+
    };
    (spi0_sck: $($n:literal),+ $(,)?) => {
        $(
            impl Spi0SckPin for Pin<$n> {
                #[inline]
                fn configure_spi0_sck(&self) {
                    self.init_pad();
                    self.set_function(Function::Spi);
                }
            }
        )+
    };
    (spi0_tx: $($n:literal),+ $(,)?) => {
        $(
            impl Spi0TxPin for Pin<$n> {
                #[inline]
                fn configure_spi0_tx(&self) {
                    self.init_pad();
                    self.set_function(Function::Spi);
                }
            }
        )+
    };
    (spi1_rx: $($n:literal),+ $(,)?) => {
        $(
            impl Spi1RxPin for Pin<$n> {
                #[inline]
                fn configure_spi1_rx(&self) {
                    self.init_pad();
                    self.set_function(Function::Spi);
                }
            }
        )+
    };
    (spi1_csn: $($n:literal),+ $(,)?) => {
        $(
            impl Spi1CsnPin for Pin<$n> {
                #[inline]
                fn configure_spi1_csn(&self) {
                    self.init_pad();
                    self.set_function(Function::Spi);
                }
            }
        )+
    };
    (spi1_sck: $($n:literal),+ $(,)?) => {
        $(
            impl Spi1SckPin for Pin<$n> {
                #[inline]
                fn configure_spi1_sck(&self) {
                    self.init_pad();
                    self.set_function(Function::Spi);
                }
            }
        )+
    };
    (spi1_tx: $($n:literal),+ $(,)?) => {
        $(
            impl Spi1TxPin for Pin<$n> {
                #[inline]
                fn configure_spi1_tx(&self) {
                    self.init_pad();
                    self.set_function(Function::Spi);
                }
            }
        )+
    };
}

// ============================================================================
// PWM Pin Traits
// ============================================================================

/// Marker trait for pins that can be used as PWM channel A.
///
/// The `SLICE` const parameter indicates the PWM slice number (0-11).
pub trait PwmAPin {
    /// The PWM slice number (0-11)
    const PWM_SLICE: u8;
    /// Configure this pin for PWM channel A function
    fn configure_pwm_a(&self);
}

/// Marker trait for pins that can be used as PWM channel B.
///
/// The `SLICE` const parameter indicates the PWM slice number (0-11).
pub trait PwmBPin {
    /// The PWM slice number (0-11)
    const PWM_SLICE: u8;
    /// Configure this pin for PWM channel B function
    fn configure_pwm_b(&self);
}

/// Macro for implementing PWM pin traits
macro_rules! impl_pwm_pins {
    (pwm_a: $slice:literal, $($n:literal),+ $(,)?) => {
        $(
            impl PwmAPin for Pin<$n> {
                const PWM_SLICE: u8 = $slice;
                #[inline]
                fn configure_pwm_a(&self) {
                    self.init_pad();
                    self.set_function(Function::Pwm);
                }
            }
        )+
    };
    (pwm_b: $slice:literal, $($n:literal),+ $(,)?) => {
        $(
            impl PwmBPin for Pin<$n> {
                const PWM_SLICE: u8 = $slice;
                #[inline]
                fn configure_pwm_b(&self) {
                    self.init_pad();
                    self.set_function(Function::Pwm);
                }
            }
        )+
    };
}

// ============================================================================
// PIO Pin Traits
// ============================================================================

/// Marker trait for pins that can be used with PIO0.
///
/// All GPIO pins support PIO. On RP2350A, pins 0-29 are available.
/// On RP2350B, pins 0-47 are available.
pub trait Pio0Pin {
    /// Configure this pin for PIO0 function
    fn configure_pio0(&self);
}

/// Marker trait for pins that can be used with PIO1.
pub trait Pio1Pin {
    /// Configure this pin for PIO1 function
    fn configure_pio1(&self);
}

/// Marker trait for pins that can be used with PIO2.
pub trait Pio2Pin {
    /// Configure this pin for PIO2 function
    fn configure_pio2(&self);
}

/// Macro for implementing PIO pin traits
macro_rules! impl_pio_pins {
    (pio0: $($n:literal),+ $(,)?) => {
        $(
            impl Pio0Pin for Pin<$n> {
                #[inline]
                fn configure_pio0(&self) {
                    self.init_pad();
                    self.set_function(Function::Pio0);
                }
            }
        )+
    };
    (pio1: $($n:literal),+ $(,)?) => {
        $(
            impl Pio1Pin for Pin<$n> {
                #[inline]
                fn configure_pio1(&self) {
                    self.init_pad();
                    self.set_function(Function::Pio1);
                }
            }
        )+
    };
    (pio2: $($n:literal),+ $(,)?) => {
        $(
            impl Pio2Pin for Pin<$n> {
                #[inline]
                fn configure_pio2(&self) {
                    self.init_pad();
                    self.set_function(Function::Pio2);
                }
            }
        )+
    };
}

// I2C0 pins (RP2350A)
// SDA: Pins where (pin_num % 4 == 0)
// SCL: Pins where (pin_num % 4 == 1)
impl_i2c_pins!(i2c0_sda: 0, 4, 8, 12, 16, 20, 24, 28);
impl_i2c_pins!(i2c0_scl: 1, 5, 9, 13, 17, 21, 25, 29);

// I2C1 pins (RP2350A)
// SDA: Pins where (pin_num % 4 == 2)
// SCL: Pins where (pin_num % 4 == 3)
impl_i2c_pins!(i2c1_sda: 2, 6, 10, 14, 18, 22, 26);
impl_i2c_pins!(i2c1_scl: 3, 7, 11, 15, 19, 23, 27);

// UART0 pins (RP2350A)
impl_uart_pins!(uart0_tx: 0, 12, 16, 28);
impl_uart_pins!(uart0_rx: 1, 13, 17, 29);
impl_uart_pins!(uart0_cts: 2, 14, 18);
impl_uart_pins!(uart0_rts: 3, 15, 19);

// UART1 pins (RP2350A)
impl_uart_pins!(uart1_tx: 4, 8, 20, 24);
impl_uart_pins!(uart1_rx: 5, 9, 21, 25);
impl_uart_pins!(uart1_cts: 6, 10, 22, 26);
impl_uart_pins!(uart1_rts: 7, 11, 23, 27);

// SPI0 pins (RP2350A)
impl_spi_pins!(spi0_rx: 0, 4, 16, 20);
impl_spi_pins!(spi0_csn: 1, 5, 17, 21);
impl_spi_pins!(spi0_sck: 2, 6, 18, 22);
impl_spi_pins!(spi0_tx: 3, 7, 19, 23);

// SPI1 pins (RP2350A)
impl_spi_pins!(spi1_rx: 8, 12, 24, 28);
impl_spi_pins!(spi1_csn: 9, 13, 25, 29);
impl_spi_pins!(spi1_sck: 10, 14, 26);
impl_spi_pins!(spi1_tx: 11, 15, 27);

// PWM pins (RP2350A) - slices 0-7
// Each slice has two channels: A (even pins) and B (odd pins)
// Slice = (pin_num / 2) % 8
impl_pwm_pins!(pwm_a: 0, 0, 16);
impl_pwm_pins!(pwm_b: 0, 1, 17);
impl_pwm_pins!(pwm_a: 1, 2, 18);
impl_pwm_pins!(pwm_b: 1, 3, 19);
impl_pwm_pins!(pwm_a: 2, 4, 20);
impl_pwm_pins!(pwm_b: 2, 5, 21);
impl_pwm_pins!(pwm_a: 3, 6, 22);
impl_pwm_pins!(pwm_b: 3, 7, 23);
impl_pwm_pins!(pwm_a: 4, 8, 24);
impl_pwm_pins!(pwm_b: 4, 9, 25);
impl_pwm_pins!(pwm_a: 5, 10, 26);
impl_pwm_pins!(pwm_b: 5, 11, 27);
impl_pwm_pins!(pwm_a: 6, 12, 28);
impl_pwm_pins!(pwm_b: 6, 13, 29);
impl_pwm_pins!(pwm_a: 7, 14);
impl_pwm_pins!(pwm_b: 7, 15);

// ============================================================================
// RP2350A Pin Mappings (30 GPIO pins: 0-29)
// Based on RP2350 datasheet Table 294
// ============================================================================

// PIO pins (RP2350A) — all GPIO pins support all PIO blocks
impl_pio_pins!(pio0: 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
                     16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29);
impl_pio_pins!(pio1: 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
                     16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29);
impl_pio_pins!(pio2: 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
                     16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29);

// ============================================================================
// RP2350B Additional Pin Mappings (pins 30-47)
// ============================================================================

#[cfg(feature = "rp2350b")]
mod rp2350b_pins {
    use super::*;

    // Additional I2C0 pins for RP2350B
    impl_i2c_pins!(i2c0_sda: 32, 36, 40, 44);
    impl_i2c_pins!(i2c0_scl: 33, 37, 41, 45);

    // Additional I2C1 pins for RP2350B
    impl_i2c_pins!(i2c1_sda: 30, 34, 38, 42, 46);
    impl_i2c_pins!(i2c1_scl: 31, 35, 39, 43, 47);

    // Additional UART0 pins for RP2350B
    impl_uart_pins!(uart0_tx: 32, 44);
    impl_uart_pins!(uart0_rx: 33, 45);
    impl_uart_pins!(uart0_cts: 34, 46);
    impl_uart_pins!(uart0_rts: 35, 47);

    // Additional UART1 pins for RP2350B
    impl_uart_pins!(uart1_tx: 36, 40);
    impl_uart_pins!(uart1_rx: 37, 41);
    impl_uart_pins!(uart1_cts: 38, 42);
    impl_uart_pins!(uart1_rts: 39, 43);

    // Additional SPI0 pins for RP2350B
    impl_spi_pins!(spi0_rx: 32, 36);
    impl_spi_pins!(spi0_csn: 33, 37);
    impl_spi_pins!(spi0_sck: 34, 38);
    impl_spi_pins!(spi0_tx: 35, 39);

    // Additional SPI1 pins for RP2350B
    impl_spi_pins!(spi1_rx: 40, 44);
    impl_spi_pins!(spi1_csn: 41, 45);
    impl_spi_pins!(spi1_sck: 42, 46);
    impl_spi_pins!(spi1_tx: 43, 47);

    // Additional PWM pins for RP2350B - slices 7-11
    impl_pwm_pins!(pwm_a: 7, 30);
    impl_pwm_pins!(pwm_b: 7, 31);
    impl_pwm_pins!(pwm_a: 8, 32, 40);
    impl_pwm_pins!(pwm_b: 8, 33, 41);
    impl_pwm_pins!(pwm_a: 9, 34, 42);
    impl_pwm_pins!(pwm_b: 9, 35, 43);
    impl_pwm_pins!(pwm_a: 10, 36, 44);
    impl_pwm_pins!(pwm_b: 10, 37, 45);
    impl_pwm_pins!(pwm_a: 11, 38, 46);
    impl_pwm_pins!(pwm_b: 11, 39, 47);

    // Additional PIO pins for RP2350B (pins 30-47, all PIO blocks)
    impl_pio_pins!(pio0: 30, 31, 32, 33, 34, 35, 36, 37, 38, 39,
                         40, 41, 42, 43, 44, 45, 46, 47);
    impl_pio_pins!(pio1: 30, 31, 32, 33, 34, 35, 36, 37, 38, 39,
                         40, 41, 42, 43, 44, 45, 46, 47);
    impl_pio_pins!(pio2: 30, 31, 32, 33, 34, 35, 36, 37, 38, 39,
                         40, 41, 42, 43, 44, 45, 46, 47);
}

/// NoPin - a placeholder for unused peripheral pins.
///
/// Use `NoPin` when a peripheral API requires a pin you don't need.
/// For example:
/// - PWM: only one channel → pass `NoPin` for the other
/// - SPI TX-only: no RX needed → pass `NoPin` for RX
/// - SPI RX-only: no TX needed → pass `NoPin` for TX
///
/// ```ignore
/// use pico_rs::gpio::pins::NoPin;
/// use pico_rs::spi::Spi0;
///
/// // TX-only SPI: no MISO pin needed
/// let spi = Spi0::init(dp.SPI0, sck_pin, tx_pin, NoPin, 1_000_000, sys_clk);
/// ```
pub struct NoPin;

// PWM
impl PwmAPin for NoPin {
    const PWM_SLICE: u8 = 255;
    fn configure_pwm_a(&self) {}
}

impl PwmBPin for NoPin {
    const PWM_SLICE: u8 = 255;
    fn configure_pwm_b(&self) {}
}

// SPI0
impl Spi0RxPin for NoPin {
    fn configure_spi0_rx(&self) {}
}
impl Spi0TxPin for NoPin {
    fn configure_spi0_tx(&self) {}
}
impl Spi0CsnPin for NoPin {
    fn configure_spi0_csn(&self) {}
}

// SPI0 SCK
impl Spi0SckPin for NoPin {
    fn configure_spi0_sck(&self) {}
}

// SPI1
impl Spi1RxPin for NoPin {
    fn configure_spi1_rx(&self) {}
}
impl Spi1TxPin for NoPin {
    fn configure_spi1_tx(&self) {}
}
impl Spi1CsnPin for NoPin {
    fn configure_spi1_csn(&self) {}
}
impl Spi1SckPin for NoPin {
    fn configure_spi1_sck(&self) {}
}

// UART0
impl Uart0TxPin for NoPin {
    fn configure_uart0_tx(&self) {}
}
impl Uart0RxPin for NoPin {
    fn configure_uart0_rx(&self) {}
}
impl Uart0CtsPin for NoPin {
    fn configure_uart0_cts(&self) {}
}
impl Uart0RtsPin for NoPin {
    fn configure_uart0_rts(&self) {}
}

// UART1
impl Uart1TxPin for NoPin {
    fn configure_uart1_tx(&self) {}
}
impl Uart1RxPin for NoPin {
    fn configure_uart1_rx(&self) {}
}
impl Uart1CtsPin for NoPin {
    fn configure_uart1_cts(&self) {}
}
impl Uart1RtsPin for NoPin {
    fn configure_uart1_rts(&self) {}
}

// I2C0
impl I2c0SdaPin for NoPin {
    fn configure_i2c0_sda(&self) {}
}
impl I2c0SclPin for NoPin {
    fn configure_i2c0_scl(&self) {}
}

// I2C1
impl I2c1SdaPin for NoPin {
    fn configure_i2c1_sda(&self) {}
}
impl I2c1SclPin for NoPin {
    fn configure_i2c1_scl(&self) {}
}
