//! GPIO Pin Function Mapping for RP2350
//!
//! This module provides compile-time verified GPIO pin function mappings
//! based on the RP2350 datasheet. It supports both RP2350A (30 pins) and
//! RP2350B (48 pins) variants through feature flags.
//!
//! ## Example
//!
//! ```ignore
//! use pico_rs::gpio::{self, Pin0, Pin4, Pin5};
//! use pico_rs::gpio::pins::{Uart0TxPin, I2c0SdaPin, I2c0SclPin};
//!
//! gpio::init_gpio();
//!
//! // Configure UART0 TX on pin 0
//! let tx = Pin0::new();
//! tx.into_uart0_tx();
//!
//! // Configure I2C0 on pins 4, 5
//! let sda = Pin4::new();
//! let scl = Pin5::new();
//! sda.into_i2c0_sda();
//! scl.into_i2c0_scl();
//! ```

use super::Function;

// ============================================================================
// UART Pin Function Traits
// ============================================================================

/// Marker trait for pins that can be used as UART0 TX
pub trait Uart0TxPin {
    /// Configure this pin as UART0 TX
    fn configure_uart0_tx(&self);
}

/// Marker trait for pins that can be used as UART0 RX
pub trait Uart0RxPin {
    /// Configure this pin as UART0 RX
    fn configure_uart0_rx(&self);
}

/// Marker trait for pins that can be used as UART0 CTS
pub trait Uart0CtsPin {
    /// Configure this pin as UART0 CTS
    fn configure_uart0_cts(&self);
}

/// Marker trait for pins that can be used as UART0 RTS
pub trait Uart0RtsPin {
    /// Configure this pin as UART0 RTS
    fn configure_uart0_rts(&self);
}

/// Marker trait for pins that can be used as UART1 TX
pub trait Uart1TxPin {
    /// Configure this pin as UART1 TX
    fn configure_uart1_tx(&self);
}

/// Marker trait for pins that can be used as UART1 RX
pub trait Uart1RxPin {
    /// Configure this pin as UART1 RX
    fn configure_uart1_rx(&self);
}

/// Marker trait for pins that can be used as UART1 CTS
pub trait Uart1CtsPin {
    /// Configure this pin as UART1 CTS
    fn configure_uart1_cts(&self);
}

/// Marker trait for pins that can be used as UART1 RTS
pub trait Uart1RtsPin {
    /// Configure this pin as UART1 RTS
    fn configure_uart1_rts(&self);
}

// ============================================================================
// I2C Pin Function Traits
// ============================================================================

/// Marker trait for pins that can be used as I2C0 SDA
pub trait I2c0SdaPin {
    /// Configure this pin as I2C0 SDA
    fn configure_i2c0_sda(&self);
}

/// Marker trait for pins that can be used as I2C0 SCL
pub trait I2c0SclPin {
    /// Configure this pin as I2C0 SCL
    fn configure_i2c0_scl(&self);
}

/// Marker trait for pins that can be used as I2C1 SDA
pub trait I2c1SdaPin {
    /// Configure this pin as I2C1 SDA
    fn configure_i2c1_sda(&self);
}

/// Marker trait for pins that can be used as I2C1 SCL
pub trait I2c1SclPin {
    /// Configure this pin as I2C1 SCL
    fn configure_i2c1_scl(&self);
}

// ============================================================================
// SPI Pin Function Traits
// ============================================================================

/// Marker trait for pins that can be used as SPI0 RX (MISO)
pub trait Spi0RxPin {
    /// Configure this pin as SPI0 RX
    fn configure_spi0_rx(&self);
}

/// Marker trait for pins that can be used as SPI0 CSN (Chip Select)
pub trait Spi0CsnPin {
    /// Configure this pin as SPI0 CSN
    fn configure_spi0_csn(&self);
}

/// Marker trait for pins that can be used as SPI0 SCK (Clock)
pub trait Spi0SckPin {
    /// Configure this pin as SPI0 SCK
    fn configure_spi0_sck(&self);
}

/// Marker trait for pins that can be used as SPI0 TX (MOSI)
pub trait Spi0TxPin {
    /// Configure this pin as SPI0 TX
    fn configure_spi0_tx(&self);
}

/// Marker trait for pins that can be used as SPI1 RX (MISO)
pub trait Spi1RxPin {
    /// Configure this pin as SPI1 RX
    fn configure_spi1_rx(&self);
}

/// Marker trait for pins that can be used as SPI1 CSN (Chip Select)
pub trait Spi1CsnPin {
    /// Configure this pin as SPI1 CSN
    fn configure_spi1_csn(&self);
}

/// Marker trait for pins that can be used as SPI1 SCK (Clock)
pub trait Spi1SckPin {
    /// Configure this pin as SPI1 SCK
    fn configure_spi1_sck(&self);
}

/// Marker trait for pins that can be used as SPI1 TX (MOSI)
pub trait Spi1TxPin {
    /// Configure this pin as SPI1 TX
    fn configure_spi1_tx(&self);
}

// ============================================================================
// PWM Pin Function Traits
// ============================================================================

/// Marker trait for pins that can be used as PWM channel A
pub trait PwmAPin {
    /// The PWM slice number (0-11)
    const PWM_SLICE: u8;
    /// Configure this pin as PWM channel A
    fn configure_pwm_a(&self);
}

/// Marker trait for pins that can be used as PWM channel B
pub trait PwmBPin {
    /// The PWM slice number (0-11)
    const PWM_SLICE: u8;
    /// Configure this pin as PWM channel B
    fn configure_pwm_b(&self);
}

// ============================================================================
// Macro for implementing pin function traits
// ============================================================================

macro_rules! impl_uart_pin {
    // UART0 TX
    (uart0_tx: $($pin:ident),+ $(,)?) => {
        $(
            impl Uart0TxPin for super::$pin {
                #[inline]
                fn configure_uart0_tx(&self) {
                    self.init_pad();
                    self.set_function(Function::Uart);
                }
            }

            impl super::$pin {
                /// Configure this pin as UART0 TX
                #[inline]
                pub fn into_uart0_tx(&self) -> &Self {
                    <Self as Uart0TxPin>::configure_uart0_tx(self);
                    self
                }
            }
        )+
    };
    // UART0 RX
    (uart0_rx: $($pin:ident),+ $(,)?) => {
        $(
            impl Uart0RxPin for super::$pin {
                #[inline]
                fn configure_uart0_rx(&self) {
                    self.init_pad();
                    self.set_function(Function::Uart);
                }
            }

            impl super::$pin {
                /// Configure this pin as UART0 RX
                #[inline]
                pub fn into_uart0_rx(&self) -> &Self {
                    <Self as Uart0RxPin>::configure_uart0_rx(self);
                    self
                }
            }
        )+
    };
    // UART0 CTS
    (uart0_cts: $($pin:ident),+ $(,)?) => {
        $(
            impl Uart0CtsPin for super::$pin {
                #[inline]
                fn configure_uart0_cts(&self) {
                    self.init_pad();
                    self.set_function(Function::Uart);
                }
            }

            impl super::$pin {
                /// Configure this pin as UART0 CTS
                #[inline]
                pub fn into_uart0_cts(&self) -> &Self {
                    <Self as Uart0CtsPin>::configure_uart0_cts(self);
                    self
                }
            }
        )+
    };
    // UART0 RTS
    (uart0_rts: $($pin:ident),+ $(,)?) => {
        $(
            impl Uart0RtsPin for super::$pin {
                #[inline]
                fn configure_uart0_rts(&self) {
                    self.init_pad();
                    self.set_function(Function::Uart);
                }
            }

            impl super::$pin {
                /// Configure this pin as UART0 RTS
                #[inline]
                pub fn into_uart0_rts(&self) -> &Self {
                    <Self as Uart0RtsPin>::configure_uart0_rts(self);
                    self
                }
            }
        )+
    };
    // UART1 TX
    (uart1_tx: $($pin:ident),+ $(,)?) => {
        $(
            impl Uart1TxPin for super::$pin {
                #[inline]
                fn configure_uart1_tx(&self) {
                    self.init_pad();
                    self.set_function(Function::Uart);
                }
            }

            impl super::$pin {
                /// Configure this pin as UART1 TX
                #[inline]
                pub fn into_uart1_tx(&self) -> &Self {
                    <Self as Uart1TxPin>::configure_uart1_tx(self);
                    self
                }
            }
        )+
    };
    // UART1 RX
    (uart1_rx: $($pin:ident),+ $(,)?) => {
        $(
            impl Uart1RxPin for super::$pin {
                #[inline]
                fn configure_uart1_rx(&self) {
                    self.init_pad();
                    self.set_function(Function::Uart);
                }
            }

            impl super::$pin {
                /// Configure this pin as UART1 RX
                #[inline]
                pub fn into_uart1_rx(&self) -> &Self {
                    <Self as Uart1RxPin>::configure_uart1_rx(self);
                    self
                }
            }
        )+
    };
    // UART1 CTS
    (uart1_cts: $($pin:ident),+ $(,)?) => {
        $(
            impl Uart1CtsPin for super::$pin {
                #[inline]
                fn configure_uart1_cts(&self) {
                    self.init_pad();
                    self.set_function(Function::Uart);
                }
            }

            impl super::$pin {
                /// Configure this pin as UART1 CTS
                #[inline]
                pub fn into_uart1_cts(&self) -> &Self {
                    <Self as Uart1CtsPin>::configure_uart1_cts(self);
                    self
                }
            }
        )+
    };
    // UART1 RTS
    (uart1_rts: $($pin:ident),+ $(,)?) => {
        $(
            impl Uart1RtsPin for super::$pin {
                #[inline]
                fn configure_uart1_rts(&self) {
                    self.init_pad();
                    self.set_function(Function::Uart);
                }
            }

            impl super::$pin {
                /// Configure this pin as UART1 RTS
                #[inline]
                pub fn into_uart1_rts(&self) -> &Self {
                    <Self as Uart1RtsPin>::configure_uart1_rts(self);
                    self
                }
            }
        )+
    };
}

macro_rules! impl_i2c_pin {
    // I2C0 SDA
    (i2c0_sda: $($pin:ident),+ $(,)?) => {
        $(
            impl I2c0SdaPin for super::$pin {
                #[inline]
                fn configure_i2c0_sda(&self) {
                    self.init_pad();
                    self.set_pull_up();  // I2C requires pull-ups
                    self.set_function(Function::I2c);
                }
            }

            impl super::$pin {
                /// Configure this pin as I2C0 SDA (with pull-up)
                #[inline]
                pub fn into_i2c0_sda(&self) -> &Self {
                    <Self as I2c0SdaPin>::configure_i2c0_sda(self);
                    self
                }
            }
        )+
    };
    // I2C0 SCL
    (i2c0_scl: $($pin:ident),+ $(,)?) => {
        $(
            impl I2c0SclPin for super::$pin {
                #[inline]
                fn configure_i2c0_scl(&self) {
                    self.init_pad();
                    self.set_pull_up();  // I2C requires pull-ups
                    self.set_function(Function::I2c);
                }
            }

            impl super::$pin {
                /// Configure this pin as I2C0 SCL (with pull-up)
                #[inline]
                pub fn into_i2c0_scl(&self) -> &Self {
                    <Self as I2c0SclPin>::configure_i2c0_scl(self);
                    self
                }
            }
        )+
    };
    // I2C1 SDA
    (i2c1_sda: $($pin:ident),+ $(,)?) => {
        $(
            impl I2c1SdaPin for super::$pin {
                #[inline]
                fn configure_i2c1_sda(&self) {
                    self.init_pad();
                    self.set_pull_up();  // I2C requires pull-ups
                    self.set_function(Function::I2c);
                }
            }

            impl super::$pin {
                /// Configure this pin as I2C1 SDA (with pull-up)
                #[inline]
                pub fn into_i2c1_sda(&self) -> &Self {
                    <Self as I2c1SdaPin>::configure_i2c1_sda(self);
                    self
                }
            }
        )+
    };
    // I2C1 SCL
    (i2c1_scl: $($pin:ident),+ $(,)?) => {
        $(
            impl I2c1SclPin for super::$pin {
                #[inline]
                fn configure_i2c1_scl(&self) {
                    self.init_pad();
                    self.set_pull_up();  // I2C requires pull-ups
                    self.set_function(Function::I2c);
                }
            }

            impl super::$pin {
                /// Configure this pin as I2C1 SCL (with pull-up)
                #[inline]
                pub fn into_i2c1_scl(&self) -> &Self {
                    <Self as I2c1SclPin>::configure_i2c1_scl(self);
                    self
                }
            }
        )+
    };
}

macro_rules! impl_spi_pin {
    // SPI0 RX
    (spi0_rx: $($pin:ident),+ $(,)?) => {
        $(
            impl Spi0RxPin for super::$pin {
                #[inline]
                fn configure_spi0_rx(&self) {
                    self.init_pad();
                    self.set_function(Function::Spi);
                }
            }

            impl super::$pin {
                /// Configure this pin as SPI0 RX (MISO)
                #[inline]
                pub fn into_spi0_rx(&self) -> &Self {
                    <Self as Spi0RxPin>::configure_spi0_rx(self);
                    self
                }
            }
        )+
    };
    // SPI0 CSN
    (spi0_csn: $($pin:ident),+ $(,)?) => {
        $(
            impl Spi0CsnPin for super::$pin {
                #[inline]
                fn configure_spi0_csn(&self) {
                    self.init_pad();
                    self.set_function(Function::Spi);
                }
            }

            impl super::$pin {
                /// Configure this pin as SPI0 CSN (Chip Select)
                #[inline]
                pub fn into_spi0_csn(&self) -> &Self {
                    <Self as Spi0CsnPin>::configure_spi0_csn(self);
                    self
                }
            }
        )+
    };
    // SPI0 SCK
    (spi0_sck: $($pin:ident),+ $(,)?) => {
        $(
            impl Spi0SckPin for super::$pin {
                #[inline]
                fn configure_spi0_sck(&self) {
                    self.init_pad();
                    self.set_function(Function::Spi);
                }
            }

            impl super::$pin {
                /// Configure this pin as SPI0 SCK (Clock)
                #[inline]
                pub fn into_spi0_sck(&self) -> &Self {
                    <Self as Spi0SckPin>::configure_spi0_sck(self);
                    self
                }
            }
        )+
    };
    // SPI0 TX
    (spi0_tx: $($pin:ident),+ $(,)?) => {
        $(
            impl Spi0TxPin for super::$pin {
                #[inline]
                fn configure_spi0_tx(&self) {
                    self.init_pad();
                    self.set_function(Function::Spi);
                }
            }

            impl super::$pin {
                /// Configure this pin as SPI0 TX (MOSI)
                #[inline]
                pub fn into_spi0_tx(&self) -> &Self {
                    <Self as Spi0TxPin>::configure_spi0_tx(self);
                    self
                }
            }
        )+
    };
    // SPI1 RX
    (spi1_rx: $($pin:ident),+ $(,)?) => {
        $(
            impl Spi1RxPin for super::$pin {
                #[inline]
                fn configure_spi1_rx(&self) {
                    self.init_pad();
                    self.set_function(Function::Spi);
                }
            }

            impl super::$pin {
                /// Configure this pin as SPI1 RX (MISO)
                #[inline]
                pub fn into_spi1_rx(&self) -> &Self {
                    <Self as Spi1RxPin>::configure_spi1_rx(self);
                    self
                }
            }
        )+
    };
    // SPI1 CSN
    (spi1_csn: $($pin:ident),+ $(,)?) => {
        $(
            impl Spi1CsnPin for super::$pin {
                #[inline]
                fn configure_spi1_csn(&self) {
                    self.init_pad();
                    self.set_function(Function::Spi);
                }
            }

            impl super::$pin {
                /// Configure this pin as SPI1 CSN (Chip Select)
                #[inline]
                pub fn into_spi1_csn(&self) -> &Self {
                    <Self as Spi1CsnPin>::configure_spi1_csn(self);
                    self
                }
            }
        )+
    };
    // SPI1 SCK
    (spi1_sck: $($pin:ident),+ $(,)?) => {
        $(
            impl Spi1SckPin for super::$pin {
                #[inline]
                fn configure_spi1_sck(&self) {
                    self.init_pad();
                    self.set_function(Function::Spi);
                }
            }

            impl super::$pin {
                /// Configure this pin as SPI1 SCK (Clock)
                #[inline]
                pub fn into_spi1_sck(&self) -> &Self {
                    <Self as Spi1SckPin>::configure_spi1_sck(self);
                    self
                }
            }
        )+
    };
    // SPI1 TX
    (spi1_tx: $($pin:ident),+ $(,)?) => {
        $(
            impl Spi1TxPin for super::$pin {
                #[inline]
                fn configure_spi1_tx(&self) {
                    self.init_pad();
                    self.set_function(Function::Spi);
                }
            }

            impl super::$pin {
                /// Configure this pin as SPI1 TX (MOSI)
                #[inline]
                pub fn into_spi1_tx(&self) -> &Self {
                    <Self as Spi1TxPin>::configure_spi1_tx(self);
                    self
                }
            }
        )+
    };
}

macro_rules! impl_pwm_pin {
    // PWM Channel A
    (pwm_a: $slice:expr, $($pin:ident),+ $(,)?) => {
        $(
            impl PwmAPin for super::$pin {
                const PWM_SLICE: u8 = $slice;

                #[inline]
                fn configure_pwm_a(&self) {
                    self.init_pad();
                    self.set_function(Function::Pwm);
                }
            }

            impl super::$pin {
                #[doc = concat!("Configure this pin as PWM slice ", $slice, " channel A")]
                #[inline]
                pub fn into_pwm_a(&self) -> &Self {
                    <Self as PwmAPin>::configure_pwm_a(self);
                    self
                }
            }
        )+
    };
    // PWM Channel B
    (pwm_b: $slice:expr, $($pin:ident),+ $(,)?) => {
        $(
            impl PwmBPin for super::$pin {
                const PWM_SLICE: u8 = $slice;

                #[inline]
                fn configure_pwm_b(&self) {
                    self.init_pad();
                    self.set_function(Function::Pwm);
                }
            }

            impl super::$pin {
                #[doc = concat!("Configure this pin as PWM slice ", $slice, " channel B")]
                #[inline]
                pub fn into_pwm_b(&self) -> &Self {
                    <Self as PwmBPin>::configure_pwm_b(self);
                    self
                }
            }
        )+
    };
}

// ============================================================================
// RP2350A Pin Mappings (30 GPIO pins: 0-29)
// Based on rp2350a_interface_pins.json
// ============================================================================

// UART0 pins (RP2350A)
impl_uart_pin!(uart0_tx: Pin0, Pin2, Pin12, Pin14, Pin16, Pin18, Pin28);
impl_uart_pin!(uart0_rx: Pin1, Pin3, Pin13, Pin15, Pin17, Pin19, Pin29);
impl_uart_pin!(uart0_cts: Pin2, Pin14, Pin18);
impl_uart_pin!(uart0_rts: Pin3, Pin15, Pin19);

// UART1 pins (RP2350A)
impl_uart_pin!(uart1_tx: Pin4, Pin6, Pin8, Pin10, Pin20, Pin22, Pin24, Pin26);
impl_uart_pin!(uart1_rx: Pin5, Pin7, Pin9, Pin11, Pin21, Pin23, Pin25, Pin27);
impl_uart_pin!(uart1_cts: Pin6, Pin10, Pin22, Pin26);
impl_uart_pin!(uart1_rts: Pin7, Pin11, Pin23, Pin27);

// I2C0 pins (RP2350A)
impl_i2c_pin!(i2c0_sda: Pin0, Pin4, Pin8, Pin12, Pin16, Pin20, Pin24, Pin28);
impl_i2c_pin!(i2c0_scl: Pin1, Pin5, Pin9, Pin13, Pin17, Pin21, Pin25, Pin29);

// I2C1 pins (RP2350A)
impl_i2c_pin!(i2c1_sda: Pin2, Pin6, Pin10, Pin14, Pin18, Pin22, Pin26);
impl_i2c_pin!(i2c1_scl: Pin3, Pin7, Pin11, Pin15, Pin19, Pin23, Pin27);

// SPI0 pins (RP2350A)
impl_spi_pin!(spi0_rx: Pin0, Pin4, Pin16, Pin20);
impl_spi_pin!(spi0_csn: Pin1, Pin5, Pin17, Pin21);
impl_spi_pin!(spi0_sck: Pin2, Pin6, Pin18, Pin22);
impl_spi_pin!(spi0_tx: Pin3, Pin7, Pin19, Pin23);

// SPI1 pins (RP2350A)
impl_spi_pin!(spi1_rx: Pin8, Pin12, Pin24, Pin28);
impl_spi_pin!(spi1_csn: Pin9, Pin13, Pin25, Pin29);
impl_spi_pin!(spi1_sck: Pin10, Pin14, Pin26);
impl_spi_pin!(spi1_tx: Pin11, Pin15, Pin27);

// PWM pins (RP2350A) - slices 0-7
impl_pwm_pin!(pwm_a: 0, Pin0, Pin16);
impl_pwm_pin!(pwm_b: 0, Pin1, Pin17);
impl_pwm_pin!(pwm_a: 1, Pin2, Pin18);
impl_pwm_pin!(pwm_b: 1, Pin3, Pin19);
impl_pwm_pin!(pwm_a: 2, Pin4, Pin20);
impl_pwm_pin!(pwm_b: 2, Pin5, Pin21);
impl_pwm_pin!(pwm_a: 3, Pin6, Pin22);
impl_pwm_pin!(pwm_b: 3, Pin7, Pin23);
impl_pwm_pin!(pwm_a: 4, Pin8, Pin24);
impl_pwm_pin!(pwm_b: 4, Pin9, Pin25);
impl_pwm_pin!(pwm_a: 5, Pin10, Pin26);
impl_pwm_pin!(pwm_b: 5, Pin11, Pin27);
impl_pwm_pin!(pwm_a: 6, Pin12, Pin28);
impl_pwm_pin!(pwm_b: 6, Pin13, Pin29);
impl_pwm_pin!(pwm_a: 7, Pin14);
impl_pwm_pin!(pwm_b: 7, Pin15);

// ============================================================================
// RP2350B Additional Pin Mappings (pins 30-47)
// Based on rp2350b_interface_pins.json
// ============================================================================

// Additional UART0 pins for RP2350B
#[cfg(feature = "rp2350b")]
impl_uart_pin!(uart0_tx: Pin30, Pin32, Pin34, Pin44, Pin46);
#[cfg(feature = "rp2350b")]
impl_uart_pin!(uart0_rx: Pin31, Pin33, Pin35, Pin45, Pin47);
#[cfg(feature = "rp2350b")]
impl_uart_pin!(uart0_cts: Pin30, Pin34, Pin46);
#[cfg(feature = "rp2350b")]
impl_uart_pin!(uart0_rts: Pin31, Pin35, Pin47);

// Additional UART1 pins for RP2350B
#[cfg(feature = "rp2350b")]
impl_uart_pin!(uart1_tx: Pin36, Pin38, Pin40, Pin42);
#[cfg(feature = "rp2350b")]
impl_uart_pin!(uart1_rx: Pin37, Pin39, Pin41, Pin43);
#[cfg(feature = "rp2350b")]
impl_uart_pin!(uart1_cts: Pin38, Pin42);
#[cfg(feature = "rp2350b")]
impl_uart_pin!(uart1_rts: Pin39, Pin43);

// Additional I2C0 pins for RP2350B
#[cfg(feature = "rp2350b")]
impl_i2c_pin!(i2c0_sda: Pin32, Pin36, Pin40, Pin44);
#[cfg(feature = "rp2350b")]
impl_i2c_pin!(i2c0_scl: Pin33, Pin37, Pin41, Pin45);

// Additional I2C1 pins for RP2350B
#[cfg(feature = "rp2350b")]
impl_i2c_pin!(i2c1_sda: Pin30, Pin34, Pin38, Pin42, Pin46);
#[cfg(feature = "rp2350b")]
impl_i2c_pin!(i2c1_scl: Pin31, Pin35, Pin39, Pin43, Pin47);

// Additional SPI0 pins for RP2350B
#[cfg(feature = "rp2350b")]
impl_spi_pin!(spi0_rx: Pin32, Pin36);
#[cfg(feature = "rp2350b")]
impl_spi_pin!(spi0_csn: Pin33, Pin37);
#[cfg(feature = "rp2350b")]
impl_spi_pin!(spi0_sck: Pin34, Pin38);
#[cfg(feature = "rp2350b")]
impl_spi_pin!(spi0_tx: Pin35, Pin39);

// Additional SPI1 pins for RP2350B
#[cfg(feature = "rp2350b")]
impl_spi_pin!(spi1_rx: Pin40, Pin44);
#[cfg(feature = "rp2350b")]
impl_spi_pin!(spi1_csn: Pin41, Pin45);
#[cfg(feature = "rp2350b")]
impl_spi_pin!(spi1_sck: Pin42, Pin46);
#[cfg(feature = "rp2350b")]
impl_spi_pin!(spi1_tx: Pin43, Pin47);

// Additional PWM pins for RP2350B - slices 8-11
#[cfg(feature = "rp2350b")]
impl_pwm_pin!(pwm_a: 8, Pin32, Pin40);
#[cfg(feature = "rp2350b")]
impl_pwm_pin!(pwm_b: 8, Pin33, Pin41);
#[cfg(feature = "rp2350b")]
impl_pwm_pin!(pwm_a: 9, Pin34, Pin42);
#[cfg(feature = "rp2350b")]
impl_pwm_pin!(pwm_b: 9, Pin35, Pin43);
#[cfg(feature = "rp2350b")]
impl_pwm_pin!(pwm_a: 10, Pin36, Pin44);
#[cfg(feature = "rp2350b")]
impl_pwm_pin!(pwm_b: 10, Pin37, Pin45);
#[cfg(feature = "rp2350b")]
impl_pwm_pin!(pwm_a: 11, Pin38, Pin46);
#[cfg(feature = "rp2350b")]
impl_pwm_pin!(pwm_b: 11, Pin39, Pin47);

// RP2350B also has Pin30/Pin31 available for PWM7 (extension from RP2350A)
#[cfg(feature = "rp2350b")]
impl_pwm_pin!(pwm_a: 7, Pin30);
#[cfg(feature = "rp2350b")]
impl_pwm_pin!(pwm_b: 7, Pin31);
