//! Per-GPIO Pin Function Description for RP2350
//!
//! This module provides a complete description of each GPIO pin's available
//! peripheral functions, based on the RP2350 datasheet GPIO function select table.
//!
//! # Function Select (funcsel)
//!
//! Each GPIO pin's function is controlled by the `funcsel` field in
//! `IO_BANK0.GPIOx_CTRL`:
//!
//! | funcsel | Function | Notes                              |
//! |---------|----------|------------------------------------|
//! | 0       | JTAG     | Pins 24-25 only (SWCLK / SWDIO)   |
//! | 1       | SPI      | Instance and signal depend on pin  |
//! | 2       | UART     | Instance and signal depend on pin  |
//! | 3       | I2C      | Instance and signal depend on pin  |
//! | 4       | PWM      | Slice and channel depend on pin    |
//! | 5       | SIO      | All pins — direct CPU I/O          |
//! | 6       | PIO0     | All pins                           |
//! | 7       | PIO1     | All pins                           |
//! | 8       | PIO2     | All pins                           |
//! | 9       | XIP/Clk  | Varies                             |
//! | 10      | USB      | Varies                             |
//! | 31      | NULL     | All pins — disabled / hi-Z         |
//!
//! # RP2350A Pin Function Table (GPIO 0–29)
//!
//! | GPIO | F1 (SPI)  | F2 (UART)  | F3 (I2C)  | F4 (PWM) | ADC  |
//! |------|-----------|------------|-----------|----------|------|
//! |  0   | SPI0 RX   | UART0 TX   | I2C0 SDA  | PWM0 A   | —    |
//! |  1   | SPI0 CSn  | UART0 RX   | I2C0 SCL  | PWM0 B   | —    |
//! |  2   | SPI0 SCK  | UART0 CTS  | I2C1 SDA  | PWM1 A   | —    |
//! |  3   | SPI0 TX   | UART0 RTS  | I2C1 SCL  | PWM1 B   | —    |
//! |  4   | SPI0 RX   | UART1 TX   | I2C0 SDA  | PWM2 A   | —    |
//! |  5   | SPI0 CSn  | UART1 RX   | I2C0 SCL  | PWM2 B   | —    |
//! |  6   | SPI0 SCK  | UART1 CTS  | I2C1 SDA  | PWM3 A   | —    |
//! |  7   | SPI0 TX   | UART1 RTS  | I2C1 SCL  | PWM3 B   | —    |
//! |  8   | SPI1 RX   | UART1 TX   | I2C0 SDA  | PWM4 A   | —    |
//! |  9   | SPI1 CSn  | UART1 RX   | I2C0 SCL  | PWM4 B   | —    |
//! | 10   | SPI1 SCK  | UART1 CTS  | I2C1 SDA  | PWM5 A   | —    |
//! | 11   | SPI1 TX   | UART1 RTS  | I2C1 SCL  | PWM5 B   | —    |
//! | 12   | SPI1 RX   | UART0 TX   | I2C0 SDA  | PWM6 A   | —    |
//! | 13   | SPI1 CSn  | UART0 RX   | I2C0 SCL  | PWM6 B   | —    |
//! | 14   | SPI1 SCK  | UART0 CTS  | I2C1 SDA  | PWM7 A   | —    |
//! | 15   | SPI1 TX   | UART0 RTS  | I2C1 SCL  | PWM7 B   | —    |
//! | 16   | SPI0 RX   | UART0 TX   | I2C0 SDA  | PWM0 A   | —    |
//! | 17   | SPI0 CSn  | UART0 RX   | I2C0 SCL  | PWM0 B   | —    |
//! | 18   | SPI0 SCK  | UART0 CTS  | I2C1 SDA  | PWM1 A   | —    |
//! | 19   | SPI0 TX   | UART0 RTS  | I2C1 SCL  | PWM1 B   | —    |
//! | 20   | SPI0 RX   | UART1 TX   | I2C0 SDA  | PWM2 A   | —    |
//! | 21   | SPI0 CSn  | UART1 RX   | I2C0 SCL  | PWM2 B   | —    |
//! | 22   | SPI0 SCK  | UART1 CTS  | I2C1 SDA  | PWM3 A   | —    |
//! | 23   | SPI0 TX   | UART1 RTS  | I2C1 SCL  | PWM3 B   | —    |
//! | 24   | SPI1 RX   | UART1 TX   | I2C0 SDA  | PWM4 A   | —    |
//! | 25   | SPI1 CSn  | UART1 RX   | I2C0 SCL  | PWM4 B   | —    |
//! | 26   | SPI1 SCK  | UART1 CTS  | I2C1 SDA  | PWM5 A   | ADC0¹|
//! | 27   | SPI1 TX   | UART1 RTS  | I2C1 SCL  | PWM5 B   | ADC1¹|
//! | 28   | SPI1 RX   | UART0 TX   | I2C0 SDA  | PWM6 A   | ADC2¹|
//! | 29   | SPI1 CSn  | UART0 RX   | I2C0 SCL  | PWM6 B   | ADC3¹|
//!
//! ¹ ADC on GPIO 26–29 is only available on RP2350A.
//!
//! *All pins also support: SIO (F5), PIO0 (F6), PIO1 (F7), PIO2 (F8).*
//!
//! # RP2350B Additional Pin Function Table (GPIO 30–47)
//!
//! The RP2350B (QFN-80) provides 18 additional GPIO pins (30–47) with the
//! same periodic function mapping pattern. PWM slices 8–11 are only available
//! on RP2350B. These additional pins do **not** have ADC capability.
//!
//! | GPIO | F1 (SPI)  | F2 (UART)  | F3 (I2C)  | F4 (PWM)  | ADC  |
//! |------|-----------|------------|-----------|-----------| —    |
//! | 30   | SPI1 SCK  | UART0 CTS  | I2C1 SDA  | PWM7 A    | —    |
//! | 31   | SPI1 TX   | UART0 RTS  | I2C1 SCL  | PWM7 B    | —    |
//! | 32   | SPI0 RX   | UART0 TX   | I2C0 SDA  | PWM8 A    | —    |
//! | 33   | SPI0 CSn  | UART0 RX   | I2C0 SCL  | PWM8 B    | —    |
//! | 34   | SPI0 SCK  | UART0 CTS  | I2C1 SDA  | PWM9 A    | —    |
//! | 35   | SPI0 TX   | UART0 RTS  | I2C1 SCL  | PWM9 B    | —    |
//! | 36   | SPI0 RX   | UART1 TX   | I2C0 SDA  | PWM10 A   | —    |
//! | 37   | SPI0 CSn  | UART1 RX   | I2C0 SCL  | PWM10 B   | —    |
//! | 38   | SPI0 SCK  | UART1 CTS  | I2C1 SDA  | PWM11 A   | —    |
//! | 39   | SPI0 TX   | UART1 RTS  | I2C1 SCL  | PWM11 B   | —    |
//! | 40   | SPI1 RX   | UART1 TX   | I2C0 SDA  | PWM8 A    | ADC0 |
//! | 41   | SPI1 CSn  | UART1 RX   | I2C0 SCL  | PWM8 B    | ADC1 |
//! | 42   | SPI1 SCK  | UART1 CTS  | I2C1 SDA  | PWM9 A    | ADC2 |
//! | 43   | SPI1 TX   | UART1 RTS  | I2C1 SCL  | PWM9 B    | ADC3 |
//! | 44   | SPI1 RX   | UART0 TX   | I2C0 SDA  | PWM10 A   | ADC4 |
//! | 45   | SPI1 CSn  | UART0 RX   | I2C0 SCL  | PWM10 B   | ADC5 |
//! | 46   | SPI1 SCK  | UART0 CTS  | I2C1 SDA  | PWM11 A   | ADC6 |
//! | 47   | SPI1 TX   | UART0 RTS  | I2C1 SCL  | PWM11 B   | ADC7 |
//!
//! *All pins also support: SIO (F5), PIO0 (F6), PIO1 (F7), PIO2 (F8).*
//!
//! # ADC Pin Mapping
//!
//! The RP2350 has a 12-bit SAR ADC with 500 ksps (at 48 MHz clock).
//! The ADC GPIO mapping differs between RP2350A and RP2350B:
//!
//! ## RP2350A (4 channels + temperature sensor)
//!
//! | ADC Channel | GPIO | Notes                            |
//! |-------------|------|----------------------------------|
//! | 0           | 26   | External analog input            |
//! | 1           | 27   | External analog input            |
//! | 2           | 28   | External analog input            |
//! | 3           | 29   | External analog input            |
//! | 4           | —    | Internal temperature sensor      |
//!
//! ## RP2350B (8 channels + temperature sensor)
//!
//! | ADC Channel | GPIO | Notes                            |
//! |-------------|------|----------------------------------|
//! | 0           | 40   | External analog input            |
//! | 1           | 41   | External analog input            |
//! | 2           | 42   | External analog input            |
//! | 3           | 43   | External analog input            |
//! | 4           | 44   | External analog input            |
//! | 5           | 45   | External analog input            |
//! | 6           | 46   | External analog input            |
//! | 7           | 47   | External analog input            |
//! | 8           | —    | Internal temperature sensor      |
//!
//! To use a pin as ADC input, the pad must be configured with input disabled
//! (`IE=0`) and output disabled (`OD=1`) to avoid digital interference.
//!
//! # Usage
//!
//! ```ignore
//! use pico_rs::pin::{PinDesc, SpiFn, UartFn};
//!
//! // Compile-time: compute function for a specific GPIO
//! const GPIO4_SPI: SpiFn = PinDesc::new(4).spi;     // SPI0 RX
//! const GPIO4_UART: UartFn = PinDesc::new(4).uart;   // UART1 TX
//!
//! // Runtime: look up all functions via the pre-built table
//! let desc = &PIN_TABLE[4];
//! assert_eq!(desc.spi, SpiFn::Spi0Rx);
//! assert_eq!(desc.uart, UartFn::Uart1Tx);
//! ```

// ============================================================================
// SPI Function
// ============================================================================

/// SPI signal assigned to a specific GPIO pin at funcsel = 1.
///
/// The SPI instance (0 or 1) and signal (RX/CSn/SCK/TX) are determined by
/// the GPIO number:
/// - **Instance**: SPI0 for GPIO 0–7, 16–23 (and 32–39 on RP2350B);
///   SPI1 for GPIO 8–15, 24–31 (and 40–47 on RP2350B).
/// - **Signal**: `gpio % 4` → 0=RX, 1=CSn, 2=SCK, 3=TX.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SpiFn {
    /// SPI0 RX (MISO)
    Spi0Rx,
    /// SPI0 Chip Select (active low)
    Spi0Csn,
    /// SPI0 Clock
    Spi0Sck,
    /// SPI0 TX (MOSI)
    Spi0Tx,
    /// SPI1 RX (MISO)
    Spi1Rx,
    /// SPI1 Chip Select (active low)
    Spi1Csn,
    /// SPI1 Clock
    Spi1Sck,
    /// SPI1 TX (MOSI)
    Spi1Tx,
}

impl SpiFn {
    /// Returns the SPI instance number (0 or 1).
    pub const fn instance(&self) -> u8 {
        match self {
            Self::Spi0Rx | Self::Spi0Csn | Self::Spi0Sck | Self::Spi0Tx => 0,
            Self::Spi1Rx | Self::Spi1Csn | Self::Spi1Sck | Self::Spi1Tx => 1,
        }
    }

    /// Returns the funcsel value for this SPI function (always 1).
    pub const fn funcsel(&self) -> u32 {
        1
    }
}

// ============================================================================
// UART Function
// ============================================================================

/// UART signal assigned to a specific GPIO pin at funcsel = 2.
///
/// The UART instance (0 or 1) and signal (TX/RX/CTS/RTS) are determined by
/// the GPIO number:
/// - **Instance**: alternates in groups of 4, with XOR pattern
///   `((gpio/4) & 1) ^ ((gpio/8) & 1)` selecting the instance.
/// - **Signal**: `gpio % 4` → 0=TX, 1=RX, 2=CTS, 3=RTS.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UartFn {
    /// UART0 Transmit
    Uart0Tx,
    /// UART0 Receive
    Uart0Rx,
    /// UART0 Clear To Send
    Uart0Cts,
    /// UART0 Request To Send
    Uart0Rts,
    /// UART1 Transmit
    Uart1Tx,
    /// UART1 Receive
    Uart1Rx,
    /// UART1 Clear To Send
    Uart1Cts,
    /// UART1 Request To Send
    Uart1Rts,
}

impl UartFn {
    /// Returns the UART instance number (0 or 1).
    pub const fn instance(&self) -> u8 {
        match self {
            Self::Uart0Tx | Self::Uart0Rx | Self::Uart0Cts | Self::Uart0Rts => 0,
            Self::Uart1Tx | Self::Uart1Rx | Self::Uart1Cts | Self::Uart1Rts => 1,
        }
    }

    /// Returns true if this is a data signal (TX or RX).
    pub const fn is_data(&self) -> bool {
        matches!(
            self,
            Self::Uart0Tx | Self::Uart0Rx | Self::Uart1Tx | Self::Uart1Rx
        )
    }

    /// Returns the funcsel value for this UART function (always 2).
    pub const fn funcsel(&self) -> u32 {
        2
    }
}

// ============================================================================
// I2C Function
// ============================================================================

/// I2C signal assigned to a specific GPIO pin at funcsel = 3.
///
/// The I2C instance and signal are determined by `gpio % 4`:
/// - 0 → I2C0 SDA, 1 → I2C0 SCL, 2 → I2C1 SDA, 3 → I2C1 SCL.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I2cFn {
    /// I2C0 Data
    I2c0Sda,
    /// I2C0 Clock
    I2c0Scl,
    /// I2C1 Data
    I2c1Sda,
    /// I2C1 Clock
    I2c1Scl,
}

impl I2cFn {
    /// Returns the I2C instance number (0 or 1).
    pub const fn instance(&self) -> u8 {
        match self {
            Self::I2c0Sda | Self::I2c0Scl => 0,
            Self::I2c1Sda | Self::I2c1Scl => 1,
        }
    }

    /// Returns true if this is the SDA (data) line.
    pub const fn is_sda(&self) -> bool {
        matches!(self, Self::I2c0Sda | Self::I2c1Sda)
    }

    /// Returns the funcsel value for this I2C function (always 3).
    pub const fn funcsel(&self) -> u32 {
        3
    }
}

// ============================================================================
// PWM Channel
// ============================================================================

/// PWM channel (A or B) within a slice.
///
/// - Even GPIO → Channel A
/// - Odd GPIO  → Channel B
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PwmCh {
    /// Channel A (even GPIO pins)
    A,
    /// Channel B (odd GPIO pins)
    B,
}

// ============================================================================
// Pin Description
// ============================================================================

/// Complete function description for a single GPIO pin.
///
/// Describes the pin-specific peripheral mappings. Functions that are available
/// on **all** GPIO pins (SIO, PIO0, PIO1, PIO2, NULL) are not included since
/// they don't vary per pin.
///
/// # ADC
///
/// Only GPIO 26–29 are ADC-capable (channels 0–3). The `adc_channel` field is
/// `None` for all other pins.
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct PinDesc {
    /// GPIO pin number (0–29 for RP2350A, 0–47 for RP2350B).
    pub gpio: u8,
    /// SPI function at funcsel = 1.
    pub spi: SpiFn,
    /// UART function at funcsel = 2.
    pub uart: UartFn,
    /// I2C function at funcsel = 3.
    pub i2c: I2cFn,
    /// PWM slice number at funcsel = 4 (0–7 on RP2350A, 0–11 on RP2350B).
    pub pwm_slice: u8,
    /// PWM channel at funcsel = 4.
    pub pwm_ch: PwmCh,
    /// ADC channel if this pin is ADC-capable, `None` otherwise.
    pub adc_channel: Option<u8>,
}

impl PinDesc {
    /// Compute the complete function description for a GPIO pin number.
    ///
    /// This is a `const fn` and can be used at compile time:
    ///
    /// ```ignore
    /// const GPIO5: PinDesc = PinDesc::new(5);
    /// assert_eq!(GPIO5.spi, SpiFn::Spi0Csn);
    /// ```
    pub const fn new(gpio: u8) -> Self {
        Self {
            gpio,
            spi: compute_spi(gpio),
            uart: compute_uart(gpio),
            i2c: compute_i2c(gpio),
            pwm_slice: compute_pwm_slice(gpio),
            pwm_ch: compute_pwm_ch(gpio),
            adc_channel: compute_adc(gpio),
        }
    }
}

// ============================================================================
// Const computation helpers
// ============================================================================

/// Compute the SPI function for a GPIO pin.
///
/// - Instance: `(gpio / 8) % 2` — SPI0 for groups 0,2,4,… ; SPI1 for groups 1,3,5,…
/// - Signal:   `gpio % 4` — 0=RX, 1=CSn, 2=SCK, 3=TX
const fn compute_spi(gpio: u8) -> SpiFn {
    let is_spi1 = (gpio / 8) & 1 != 0;
    let signal = gpio & 3;
    if is_spi1 {
        match signal {
            0 => SpiFn::Spi1Rx,
            1 => SpiFn::Spi1Csn,
            2 => SpiFn::Spi1Sck,
            _ => SpiFn::Spi1Tx,
        }
    } else {
        match signal {
            0 => SpiFn::Spi0Rx,
            1 => SpiFn::Spi0Csn,
            2 => SpiFn::Spi0Sck,
            _ => SpiFn::Spi0Tx,
        }
    }
}

/// Compute the UART function for a GPIO pin.
///
/// - Instance: `((gpio/4) & 1) ^ ((gpio/8) & 1)` — XOR of two bit positions
///   selects UART0 (=0) or UART1 (=1).
/// - Signal:   `gpio % 4` — 0=TX, 1=RX, 2=CTS, 3=RTS
const fn compute_uart(gpio: u8) -> UartFn {
    let group = gpio / 4;
    let is_uart1 = ((group & 1) ^ ((group >> 1) & 1)) != 0;
    let signal = gpio & 3;
    if is_uart1 {
        match signal {
            0 => UartFn::Uart1Tx,
            1 => UartFn::Uart1Rx,
            2 => UartFn::Uart1Cts,
            _ => UartFn::Uart1Rts,
        }
    } else {
        match signal {
            0 => UartFn::Uart0Tx,
            1 => UartFn::Uart0Rx,
            2 => UartFn::Uart0Cts,
            _ => UartFn::Uart0Rts,
        }
    }
}

/// Compute the I2C function for a GPIO pin.
///
/// - `gpio % 4`: 0=I2C0 SDA, 1=I2C0 SCL, 2=I2C1 SDA, 3=I2C1 SCL
const fn compute_i2c(gpio: u8) -> I2cFn {
    match gpio & 3 {
        0 => I2cFn::I2c0Sda,
        1 => I2cFn::I2c0Scl,
        2 => I2cFn::I2c1Sda,
        _ => I2cFn::I2c1Scl,
    }
}

/// Compute the PWM slice number for a GPIO pin.
///
/// - RP2350A (GPIO 0–29): `(gpio / 2) % 8` → slices 0–7
/// - RP2350B (GPIO 30–31): same formula → slice 7
/// - RP2350B (GPIO 32–47): `8 + ((gpio - 32) / 2) % 4` → slices 8–11
const fn compute_pwm_slice(gpio: u8) -> u8 {
    if gpio < 32 {
        (gpio / 2) % 8
    } else {
        8 + ((gpio - 32) / 2) % 4
    }
}

/// Compute the PWM channel for a GPIO pin.
///
/// Even pins = Channel A, odd pins = Channel B.
const fn compute_pwm_ch(gpio: u8) -> PwmCh {
    if gpio & 1 == 0 { PwmCh::A } else { PwmCh::B }
}

/// Compute the ADC channel for a GPIO pin, if any.
///
/// - **RP2350A**: GPIO 26–29 → ADC channels 0–3
/// - **RP2350B**: GPIO 40–47 → ADC channels 0–7
const fn compute_adc(gpio: u8) -> Option<u8> {
    #[cfg(feature = "rp2350a")]
    {
        match gpio {
            26 => Some(0),
            27 => Some(1),
            28 => Some(2),
            29 => Some(3),
            _ => None,
        }
    }
    #[cfg(feature = "rp2350b")]
    {
        match gpio {
            40 => Some(0),
            41 => Some(1),
            42 => Some(2),
            43 => Some(3),
            44 => Some(4),
            45 => Some(5),
            46 => Some(6),
            47 => Some(7),
            _ => None,
        }
    }
}

// ============================================================================
// Pre-built Pin Table
// ============================================================================

/// Number of GPIO pins on RP2350A.
#[cfg(feature = "rp2350a")]
pub const GPIO_COUNT: usize = 30;

/// Number of GPIO pins on RP2350B.
#[cfg(feature = "rp2350b")]
pub const GPIO_COUNT: usize = 48;

/// Pre-computed pin function table for all GPIO pins.
///
/// Indexed by GPIO number: `PIN_TABLE[gpio_num]`.
///
/// ```ignore
/// use pico_rs::pin::PIN_TABLE;
///
/// let gpio4 = &PIN_TABLE[4];
/// // SPI0 RX, UART1 TX, I2C0 SDA, PWM slice 2 channel A
/// ```
#[cfg(feature = "rp2350a")]
pub const PIN_TABLE: [PinDesc; GPIO_COUNT] = [
    PinDesc::new(0),
    PinDesc::new(1),
    PinDesc::new(2),
    PinDesc::new(3),
    PinDesc::new(4),
    PinDesc::new(5),
    PinDesc::new(6),
    PinDesc::new(7),
    PinDesc::new(8),
    PinDesc::new(9),
    PinDesc::new(10),
    PinDesc::new(11),
    PinDesc::new(12),
    PinDesc::new(13),
    PinDesc::new(14),
    PinDesc::new(15),
    PinDesc::new(16),
    PinDesc::new(17),
    PinDesc::new(18),
    PinDesc::new(19),
    PinDesc::new(20),
    PinDesc::new(21),
    PinDesc::new(22),
    PinDesc::new(23),
    PinDesc::new(24),
    PinDesc::new(25),
    PinDesc::new(26),
    PinDesc::new(27),
    PinDesc::new(28),
    PinDesc::new(29),
];

#[cfg(feature = "rp2350b")]
pub const PIN_TABLE: [PinDesc; GPIO_COUNT] = [
    PinDesc::new(0),
    PinDesc::new(1),
    PinDesc::new(2),
    PinDesc::new(3),
    PinDesc::new(4),
    PinDesc::new(5),
    PinDesc::new(6),
    PinDesc::new(7),
    PinDesc::new(8),
    PinDesc::new(9),
    PinDesc::new(10),
    PinDesc::new(11),
    PinDesc::new(12),
    PinDesc::new(13),
    PinDesc::new(14),
    PinDesc::new(15),
    PinDesc::new(16),
    PinDesc::new(17),
    PinDesc::new(18),
    PinDesc::new(19),
    PinDesc::new(20),
    PinDesc::new(21),
    PinDesc::new(22),
    PinDesc::new(23),
    PinDesc::new(24),
    PinDesc::new(25),
    PinDesc::new(26),
    PinDesc::new(27),
    PinDesc::new(28),
    PinDesc::new(29),
    PinDesc::new(30),
    PinDesc::new(31),
    PinDesc::new(32),
    PinDesc::new(33),
    PinDesc::new(34),
    PinDesc::new(35),
    PinDesc::new(36),
    PinDesc::new(37),
    PinDesc::new(38),
    PinDesc::new(39),
    PinDesc::new(40),
    PinDesc::new(41),
    PinDesc::new(42),
    PinDesc::new(43),
    PinDesc::new(44),
    PinDesc::new(45),
    PinDesc::new(46),
    PinDesc::new(47),
];

// ============================================================================
// Pin lookup helpers
// ============================================================================

/// Get the function description for a GPIO pin at runtime.
///
/// Returns `None` if `gpio` is out of range for the current chip variant.
pub const fn pin_desc(gpio: u8) -> Option<&'static PinDesc> {
    if (gpio as usize) < GPIO_COUNT {
        Some(&PIN_TABLE[gpio as usize])
    } else {
        None
    }
}

/// Find all GPIO pins that can serve as UART TX for a given instance.
///
/// Returns a static slice of GPIO pin numbers.
///
/// ```ignore
/// let uart0_tx_pins = uart_tx_pins(0); // &[0, 12, 16, 28]
/// let uart1_tx_pins = uart_tx_pins(1); // &[4, 8, 20, 24]
/// ```
pub const fn uart_tx_pins(instance: u8) -> &'static [u8] {
    match instance {
        #[cfg(feature = "rp2350a")]
        0 => &[0, 12, 16, 28],
        #[cfg(feature = "rp2350b")]
        0 => &[0, 12, 16, 28, 32, 44],
        #[cfg(feature = "rp2350a")]
        1 => &[4, 8, 20, 24],
        #[cfg(feature = "rp2350b")]
        1 => &[4, 8, 20, 24, 36, 40],
        _ => &[],
    }
}

/// Find all GPIO pins that can serve as UART RX for a given instance.
pub const fn uart_rx_pins(instance: u8) -> &'static [u8] {
    match instance {
        #[cfg(feature = "rp2350a")]
        0 => &[1, 13, 17, 29],
        #[cfg(feature = "rp2350b")]
        0 => &[1, 13, 17, 29, 33, 45],
        #[cfg(feature = "rp2350a")]
        1 => &[5, 9, 21, 25],
        #[cfg(feature = "rp2350b")]
        1 => &[5, 9, 21, 25, 37, 41],
        _ => &[],
    }
}

/// Find all GPIO pins that can serve as I2C SDA for a given instance.
pub const fn i2c_sda_pins(instance: u8) -> &'static [u8] {
    match instance {
        #[cfg(feature = "rp2350a")]
        0 => &[0, 4, 8, 12, 16, 20, 24, 28],
        #[cfg(feature = "rp2350b")]
        0 => &[0, 4, 8, 12, 16, 20, 24, 28, 32, 36, 40, 44],
        #[cfg(feature = "rp2350a")]
        1 => &[2, 6, 10, 14, 18, 22, 26],
        #[cfg(feature = "rp2350b")]
        1 => &[2, 6, 10, 14, 18, 22, 26, 30, 34, 38, 42, 46],
        _ => &[],
    }
}

/// Find all GPIO pins that can serve as I2C SCL for a given instance.
pub const fn i2c_scl_pins(instance: u8) -> &'static [u8] {
    match instance {
        #[cfg(feature = "rp2350a")]
        0 => &[1, 5, 9, 13, 17, 21, 25, 29],
        #[cfg(feature = "rp2350b")]
        0 => &[1, 5, 9, 13, 17, 21, 25, 29, 33, 37, 41, 45],
        #[cfg(feature = "rp2350a")]
        1 => &[3, 7, 11, 15, 19, 23, 27],
        #[cfg(feature = "rp2350b")]
        1 => &[3, 7, 11, 15, 19, 23, 27, 31, 35, 39, 43, 47],
        _ => &[],
    }
}

/// Find all GPIO pins that map to a specific PWM slice.
///
/// Returns a static slice of `(gpio, PwmCh)` pairs.
///
/// ```ignore
/// let slice0_pins = pwm_slice_pins(0); // &[(0, A), (1, B), (16, A), (17, B)]
/// ```
pub const fn pwm_slice_pins(slice: u8) -> &'static [(u8, PwmCh)] {
    match slice {
        0 => &[(0, PwmCh::A), (1, PwmCh::B), (16, PwmCh::A), (17, PwmCh::B)],
        1 => &[(2, PwmCh::A), (3, PwmCh::B), (18, PwmCh::A), (19, PwmCh::B)],
        2 => &[(4, PwmCh::A), (5, PwmCh::B), (20, PwmCh::A), (21, PwmCh::B)],
        3 => &[(6, PwmCh::A), (7, PwmCh::B), (22, PwmCh::A), (23, PwmCh::B)],
        4 => &[(8, PwmCh::A), (9, PwmCh::B), (24, PwmCh::A), (25, PwmCh::B)],
        5 => &[
            (10, PwmCh::A),
            (11, PwmCh::B),
            (26, PwmCh::A),
            (27, PwmCh::B),
        ],
        6 => &[
            (12, PwmCh::A),
            (13, PwmCh::B),
            (28, PwmCh::A),
            (29, PwmCh::B),
        ],
        #[cfg(feature = "rp2350a")]
        7 => &[(14, PwmCh::A), (15, PwmCh::B)],
        #[cfg(feature = "rp2350b")]
        7 => &[
            (14, PwmCh::A),
            (15, PwmCh::B),
            (30, PwmCh::A),
            (31, PwmCh::B),
        ],
        #[cfg(feature = "rp2350b")]
        8 => &[
            (32, PwmCh::A),
            (33, PwmCh::B),
            (40, PwmCh::A),
            (41, PwmCh::B),
        ],
        #[cfg(feature = "rp2350b")]
        9 => &[
            (34, PwmCh::A),
            (35, PwmCh::B),
            (42, PwmCh::A),
            (43, PwmCh::B),
        ],
        #[cfg(feature = "rp2350b")]
        10 => &[
            (36, PwmCh::A),
            (37, PwmCh::B),
            (44, PwmCh::A),
            (45, PwmCh::B),
        ],
        #[cfg(feature = "rp2350b")]
        11 => &[
            (38, PwmCh::A),
            (39, PwmCh::B),
            (46, PwmCh::A),
            (47, PwmCh::B),
        ],
        _ => &[],
    }
}

/// Find all ADC-capable GPIO pins.
///
/// Returns `(gpio, adc_channel)` pairs.
///
/// - RP2350A: GPIO 26–29 (4 channels)
/// - RP2350B: GPIO 40–47 (8 channels)
#[cfg(feature = "rp2350a")]
pub const fn adc_pins() -> &'static [(u8, u8)] {
    &[(26, 0), (27, 1), (28, 2), (29, 3)]
}

/// Find all ADC-capable GPIO pins.
///
/// Returns `(gpio, adc_channel)` pairs.
///
/// - RP2350A: GPIO 26–29 (4 channels)
/// - RP2350B: GPIO 40–47 (8 channels)
#[cfg(feature = "rp2350b")]
pub const fn adc_pins() -> &'static [(u8, u8)] {
    &[
        (40, 0),
        (41, 1),
        (42, 2),
        (43, 3),
        (44, 4),
        (45, 5),
        (46, 6),
        (47, 7),
    ]
}
