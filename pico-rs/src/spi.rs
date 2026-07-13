//! SPI (Serial Peripheral Interface) HAL for RP2350
//!
//! This module provides a safe interface for the RP2350 SPI peripherals.
//! The RP2350 has two SPI controllers (SPI0 and SPI1) based on the ARM PrimeCell SSP (PL022).
//!
//! # Features
//!
//! - Master and slave modes
//! - 4-16 data bits
//! - Clock polarity and phase configuration (Mode 0-3)
//! - Motorola SPI, TI Synchronous Serial, and Microwire frame formats
//! - 8-entry TX and RX FIFOs
//! - DMA support
//! - **Type-safe pin ownership**: Pins are consumed by SPI, preventing misuse
//!
//! # GPIO Mapping
//!
//! SPI0:
//! - RX (MISO): GPIO 0, 4, 16, 20
//! - CSn: GPIO 1, 5, 17, 21
//! - SCK: GPIO 2, 6, 18, 22
//! - TX (MOSI): GPIO 3, 7, 19, 23
//!
//! SPI1:
//! - RX (MISO): GPIO 8, 12, 24, 28
//! - CSn: GPIO 9, 13, 25, 29
//! - SCK: GPIO 10, 14, 26
//! - TX (MOSI): GPIO 11, 15, 27
//!
//! # Example (Master Mode with Pin Ownership)
//!
//! ```ignore
//! use pico_rs::gpio::Pins;
//! use pico_rs::spi::{Spi0, Mode};
//! use pico_rs::Peripherals;
//!
//! let dp = Peripherals::take().unwrap();
//! let pins = Pins::init(dp.IO_BANK0, dp.PADS_BANK0);
//!
//! // Initialize SPI0 at 1 MHz, consuming pins
//! let mut spi = Spi0::init(
//!     dp.SPI0,
//!     pins.pin::<2>(),      // SCK pin - ownership transferred
//!     pins.pin::<3>(),      // TX (MOSI) pin - ownership transferred
//!     pins.pin::<0>(),      // RX (MISO) pin - ownership transferred
//!     1_000_000,    // 1 MHz
//!     150_000_000,  // System clock
//! );
//!
//! // Configure mode (optional, default is Mode0)
//! spi.set_mode(Mode::Mode0);
//!
//! // Write data
//! spi.write(&[0x01, 0x02, 0x03]);
//!
//! // Transfer data
//! let mut buf = [0u8; 3];
//! spi.transfer(&mut buf, &[0x01, 0x02, 0x03]);
//!
//! // Compile error: pins.pin::<2>() already moved!
//! // pins.pin::<2>().into_output();
//! ```
//!
//! # Example (Master Mode with Chip Select)
//!
//! ```ignore
//! use pico_rs::gpio::Pins;
//! use pico_rs::spi::Spi0;
//!
//! let dp = Peripherals::take().unwrap();
//! let pins = Pins::init(dp.IO_BANK0, dp.PADS_BANK0);
//!
//! // Initialize SPI0 with hardware chip select
//! let mut spi = Spi0WithCs::init(
//!     dp.SPI0,
//!     pins.pin::<2>(),      // SCK
//!     pins.pin::<3>(),      // TX (MOSI)
//!     pins.pin::<0>(),      // RX (MISO)
//!     pins.pin::<1>(),      // CSn (hardware chip select)
//!     1_000_000,
//!     150_000_000,
//! );
//! ```
//!
//! # Example (Slave Mode)
//!
//! ```ignore
//! use pico_rs::gpio::Pins;
//! use pico_rs::spi::Spi0;
//!
//! let dp = Peripherals::take().unwrap();
//! let pins = Pins::init(dp.IO_BANK0, dp.PADS_BANK0);
//!
//! // Initialize SPI0 as slave (requires CSn pin)
//! let mut spi = Spi0WithCs::init_slave(
//!     dp.SPI0,
//!     pins.pin::<2>(),      // SCK
//!     pins.pin::<3>(),      // TX (MOSI from slave perspective)
//!     pins.pin::<0>(),      // RX (MISO from slave perspective)
//!     pins.pin::<1>(),      // CSn
//!     150_000_000,
//! );
//! ```

use paste::paste;

use crate::gpio::{
    Spi0CsnPin, Spi0RxPin, Spi0SckPin, Spi0TxPin, Spi1CsnPin, Spi1RxPin, Spi1SckPin, Spi1TxPin,
};
use crate::reset::Resettable;

/// SPI mode (clock polarity and phase)
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mode {
    /// CPOL=0, CPHA=0: Clock idle low, sample on rising edge
    Mode0,
    /// CPOL=0, CPHA=1: Clock idle low, sample on falling edge
    Mode1,
    /// CPOL=1, CPHA=0: Clock idle high, sample on falling edge
    Mode2,
    /// CPOL=1, CPHA=1: Clock idle high, sample on rising edge
    Mode3,
}

impl Mode {
    /// Get clock polarity (CPOL)
    #[inline]
    pub fn polarity(self) -> bool {
        matches!(self, Mode::Mode2 | Mode::Mode3)
    }

    /// Get clock phase (CPHA)
    #[inline]
    pub fn phase(self) -> bool {
        matches!(self, Mode::Mode1 | Mode::Mode3)
    }
}

/// SPI frame format
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FrameFormat {
    /// Motorola SPI format (most common)
    MotorolaSpi = 0,
    /// TI synchronous serial format
    TiSynchronousSerial = 1,
    /// National Microwire format
    NationalMicrowire = 2,
}

/// SPI error types
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// RX FIFO overrun
    Overrun,
    /// Timeout waiting for operation
    Timeout,
}

/// Abstract SPI bus trait for driver implementations.
///
/// This keeps drivers decoupled from concrete SPI types while preserving
/// the HAL's type-safe pin ownership model. A driver only needs
/// `impl SpiBus` instead of carrying the full generic parameters of
/// [`Spi0`]/[`Spi1`].
///
/// # Example
///
/// ```ignore
/// use pico_rs::spi::SpiBus;
///
/// fn read_register(spi: &impl SpiBus, cs: &impl GpioPin, reg: u8) -> u8 {
///     cs.set_low();
///     spi.write(&[reg]);
///     let mut buf = [0u8; 1];
///     spi.read(&mut buf);
///     cs.set_high();
///     buf[0]
/// }
/// ```
pub trait SpiBus {
    /// Write multiple bytes to the SPI bus (8-bit mode).
    fn write(&self, data: &[u8]);

    /// Read multiple bytes from the SPI bus by sending dummy bytes (8-bit mode).
    fn read(&self, data: &mut [u8]);

    /// Full-duplex transfer: write and read simultaneously (8-bit mode).
    ///
    /// The transfer length is `min(read.len(), write.len())`.
    fn transfer(&self, read: &mut [u8], write: &[u8]);

    /// Full-duplex transfer in-place: the same buffer is used for TX and RX.
    fn transfer_in_place(&self, data: &mut [u8]);

    /// Block until all pending transfers are complete.
    fn flush(&self);
}

// ============================================================================
// Macro for generating SPI implementations
// ============================================================================

macro_rules! impl_spi {
    ($SpiName:ident, $spi_num:literal, $SpiType:ty, $spi_mod:ident, $SckPin:ident, $TxPin:ident, $RxPin:ident, $CsnPin:ident) => {
        paste! {
            /// SPI peripheral wrapper with type-safe pin ownership.
            ///
            /// This struct owns the SPI peripheral and the GPIO pins used for
            /// SCK, TX (MOSI), and RX (MISO). The pins are configured automatically
            /// when the SPI instance is created.
            ///
            #[doc = concat!("# SPI", $spi_num, " Pin Mapping")]
            ///
            #[doc = concat!("See [`", stringify!($SckPin), "`] for valid SCK pins.")]
            ///
            #[doc = concat!("See [`", stringify!($TxPin), "`] for valid TX (MOSI) pins.")]
            ///
            #[doc = concat!("See [`", stringify!($RxPin), "`] for valid RX (MISO) pins.")]
            pub struct $SpiName<SCK, TX, RX>
            where
                SCK: $SckPin,
                TX: $TxPin,
                RX: $RxPin,
            {
                spi: $SpiType,
                _sck: SCK,
                _tx: TX,
                _rx: RX,
            }

            impl<SCK, TX, RX> $SpiName<SCK, TX, RX>
            where
                SCK: $SckPin,
                TX: $TxPin,
                RX: $RxPin,
            {
                /// SPI peripheral number
                pub const NUM: u8 = $spi_num;

                /// Obtain a pin-erased SPI handle without ownership.
                ///
                /// Returns `$SpiName<NoPin, NoPin, NoPin>` — a handle that can
                /// access all SPI registers without carrying pin type parameters.
                /// This is useful for ISR context or global storage where the
                /// concrete pin types are inconvenient.
                ///
                /// # Safety
                ///
                /// The caller must ensure:
                /// 1. The SPI peripheral has already been initialised (pins
                ///    configured, clock set, peripheral taken out of reset).
                /// 2. No data races: the returned handle must not be used
                ///    concurrently with the owning instance without proper
                ///    synchronisation (e.g. `Mutex`).
                ///
                /// # Example
                ///
                /// ```ignore
                #[doc = concat!("// In init code: create the full SPI with pins")]
                #[doc = concat!("let spi = ", stringify!($SpiName), "::init(")]
                #[doc = concat!("    dp.SPI", $spi_num, ", sck, tx, rx, 1_000_000, sys_clk,")]
                /// );
                ///
                /// // Later, in ISR or another context:
                #[doc = concat!("let spi = unsafe { ", stringify!($SpiName), "::<NoPin, NoPin, NoPin>::steal() };")]
                /// spi.write(&[0x01, 0x02]);
                /// ```
                #[inline]
                pub unsafe fn steal() -> crate::Steal<$SpiName<crate::gpio::pins::NoPin, crate::gpio::pins::NoPin, crate::gpio::pins::NoPin>> {
                    crate::Steal($SpiName {
                        spi: unsafe { <$SpiType>::steal() },
                        _sck: crate::gpio::pins::NoPin,
                        _tx: crate::gpio::pins::NoPin,
                        _rx: crate::gpio::pins::NoPin,
                    })
                }

                /// Get reference to the underlying register block.
                ///
                /// # Safety
                /// Direct register access can bypass the HAL's safety guarantees.
                /// The caller must ensure operations do not conflict with the HAL state.
                #[inline]
                pub unsafe fn regs(&self) -> &pico_rs_pac::$spi_mod::RegisterBlock {
                    &self.spi
                }

                // ================================================================
                // Initialization
                // ================================================================

                /// Initialize SPI in master mode without hardware chip select.
                ///
                /// Use this when you want to control the chip select manually via
                /// a separate GPIO pin, or when connecting to a single device that
                /// doesn't require CS toggling.
                ///
                /// # Arguments
                ///
                /// * `spi` - The SPI peripheral from PAC
                /// * `sck` - SCK (clock) pin
                /// * `tx` - TX (MOSI) pin
                /// * `rx` - RX (MISO) pin
                /// * `baudrate` - Desired baudrate in Hz
                /// * `peri_clk` - Peripheral clock frequency in Hz
                ///
                /// # Example
                ///
                /// ```ignore
                #[doc = concat!("let spi = ", stringify!($SpiName), "::init(")]
                #[doc = concat!("    dp.SPI", $spi_num, ",")]
                ///     pins.pin::<2>(),      // SCK
                ///     pins.pin::<3>(),      // TX (MOSI)
                ///     pins.pin::<0>(),      // RX (MISO)
                ///     1_000_000,    // 1 MHz
                ///     150_000_000,  // System clock
                /// );
                /// ```
                pub fn init(
                    spi: $SpiType,
                    mut sck: SCK,
                    mut tx: TX,
                    mut rx: RX,
                    baudrate: u32,
                    peri_clk: u32,
                ) -> Self {
                    // Configure pins for SPI function
                    sck.[<configure_spi $spi_num _sck>]();
                    tx.[<configure_spi $spi_num _tx>]();
                    rx.[<configure_spi $spi_num _rx>]();

                    // Reset peripheral to ensure clean register state
                    <$SpiType as Resettable>::init();

                    let mut spi_dev = Self {
                        spi,
                        _sck: sck,
                        _tx: tx,
                        _rx: rx,
                    };

                    // Disable first
                    spi_dev.spi.sspcr1.clr_sse();

                    // Set baudrate
                    spi_dev.configure_baudrate(baudrate, peri_clk);

                    // Set default format: 8 data bits, Mode0, Motorola SPI
                    spi_dev.set_format(8, Mode::Mode0, FrameFormat::MotorolaSpi);

                    // Configure as master (MS=0), no loopback
                    spi_dev.spi.sspcr1.clr_ms();

                    // Enable
                    spi_dev.spi.sspcr1.set_sse();

                    spi_dev
                }

                /// Release the SPI peripheral and pins.
                ///
                /// Returns a tuple of (SPI peripheral, SCK pin, TX pin, RX pin).
                #[inline]
                pub fn release(self) -> ($SpiType, SCK, TX, RX) {
                    (self.spi, self._sck, self._tx, self._rx)
                }

                // ================================================================
                // Configuration
                // ================================================================

                /// Configure the SPI baudrate (internal helper)
                ///
                /// Baudrate = peri_clk / (CPSDVSR * (1 + SCR))
                fn configure_baudrate(&mut self, baudrate: u32, peri_clk: u32) -> u32 {
                    let mut best_cpsdvsr = 2u32;
                    let mut best_scr = 0u32;
                    let mut best_error = u32::MAX;

                    // CPSDVSR must be even, 2-254
                    for cpsdvsr in (2u32..=254).step_by(2) {
                        // SCR can be 0-255
                        let scr = peri_clk / (cpsdvsr * baudrate);
                        let scr = if scr > 0 { scr - 1 } else { 0 };

                        if scr > 255 {
                            continue;
                        }

                        let actual = peri_clk / (cpsdvsr * (1 + scr));
                        let error = if actual > baudrate {
                            actual - baudrate
                        } else {
                            baudrate - actual
                        };

                        if error < best_error {
                            best_error = error;
                            best_cpsdvsr = cpsdvsr;
                            best_scr = scr;
                        }

                        if error == 0 {
                            break;
                        }
                    }

                    // Write prescaler
                    self.spi.sspcpsr.write_cpsdvsr(best_cpsdvsr);

                    // Update SCR in CR0 (bits 15:8)
                    let cr0 = self.spi.sspcr0.read_bits();
                    let cr0 = (cr0 & 0x00ff) | (best_scr << 8);
                    self.spi.sspcr0.write_bits(cr0);

                    peri_clk / (best_cpsdvsr * (1 + best_scr))
                }

                /// Set the SPI baudrate
                ///
                /// # Arguments
                /// * `baudrate` - Desired baudrate in Hz
                /// * `peri_clk` - Peripheral clock frequency in Hz
                ///
                /// # Returns
                /// The actual baudrate achieved
                pub fn set_baudrate(&mut self, baudrate: u32, peri_clk: u32) -> u32 {
                    self.spi.sspcr1.clr_sse();
                    let actual = self.configure_baudrate(baudrate, peri_clk);
                    self.spi.sspcr1.set_sse();
                    actual
                }

                /// Set the data format
                ///
                /// # Arguments
                /// * `data_bits` - Number of data bits (4-16)
                /// * `mode` - SPI mode (clock polarity and phase)
                /// * `format` - Frame format
                pub fn set_format(&mut self, data_bits: u8, mode: Mode, format: FrameFormat) {
                    let data_bits = data_bits.clamp(4, 16);

                    // Read current CR0 to preserve SCR
                    let scr = self.spi.sspcr0.scr();

                    // Build new CR0 value
                    let cr0 = (scr << 8)
                        | (if mode.phase() { 1 << 7 } else { 0 })
                        | (if mode.polarity() { 1 << 6 } else { 0 })
                        | ((format as u32) << 4)
                        | ((data_bits - 1) as u32);

                    self.spi.sspcr0.write_bits(cr0);
                }

                /// Set the SPI mode
                pub fn set_mode(&mut self, mode: Mode) {
                    if mode.polarity() {
                        self.spi.sspcr0.set_spo();
                    } else {
                        self.spi.sspcr0.clr_spo();
                    }

                    if mode.phase() {
                        self.spi.sspcr0.set_sph();
                    } else {
                        self.spi.sspcr0.clr_sph();
                    }
                }

                /// Enable/disable the SPI peripheral
                #[inline]
                pub fn set_enabled(&mut self, enable: bool) {
                    if enable {
                        self.spi.sspcr1.set_sse();
                    } else {
                        self.spi.sspcr1.clr_sse();
                    }
                }

                /// Get the current baudrate by reading hardware registers
                ///
                /// # Arguments
                /// * `peri_clk` - Peripheral clock frequency in Hz
                #[inline]
                pub fn baudrate(&self, peri_clk: u32) -> u32 {
                    let cpsdvsr = self.spi.sspcpsr.cpsdvsr();
                    let scr = self.spi.sspcr0.scr();
                    let divisor = cpsdvsr * (1 + scr);
                    if divisor == 0 {
                        return 0;
                    }
                    peri_clk / divisor
                }

                // ================================================================
                // Status
                // ================================================================

                /// Check if the SPI is busy
                #[inline]
                pub fn is_busy(&self) -> bool {
                    self.spi.sspsr.bsy()
                }

                /// Check if the TX FIFO is full
                #[inline]
                pub fn is_tx_full(&self) -> bool {
                    !self.spi.sspsr.tnf()
                }

                /// Check if the TX FIFO is empty
                #[inline]
                pub fn is_tx_empty(&self) -> bool {
                    self.spi.sspsr.tfe()
                }

                /// Check if the RX FIFO is empty
                #[inline]
                pub fn is_rx_empty(&self) -> bool {
                    !self.spi.sspsr.rne()
                }

                /// Check if the RX FIFO is full
                #[inline]
                pub fn is_rx_full(&self) -> bool {
                    self.spi.sspsr.rff()
                }

                /// Check if readable (RX FIFO has data)
                #[inline]
                pub fn is_readable(&self) -> bool {
                    self.spi.sspsr.rne()
                }

                /// Check if writable (TX FIFO has space)
                #[inline]
                pub fn is_writable(&self) -> bool {
                    self.spi.sspsr.tnf()
                }

                // ================================================================
                // Data Transfer
                // ================================================================

                /// Write a single word to the TX FIFO (non-blocking)
                ///
                /// Returns true if the word was written, false if FIFO is full.
                #[inline]
                pub fn try_write_word(&self, word: u16) -> bool {
                    if self.spi.sspsr.tnf() {
                        self.spi.sspdr.write_data(word as u32);
                        true
                    } else {
                        false
                    }
                }

                /// Write a single word to the TX FIFO (blocking)
                #[inline]
                pub fn write_word(&self, word: u16) {
                    while !self.spi.sspsr.tnf() {
                        core::hint::spin_loop();
                    }
                    self.spi.sspdr.write_data(word as u32);
                }

                /// Read a single word from the RX FIFO (non-blocking)
                ///
                /// Returns Some(word) if data available, None if FIFO is empty.
                #[inline]
                pub fn try_read_word(&self) -> Option<u16> {
                    if self.spi.sspsr.rne() {
                        Some(self.spi.sspdr.data() as u16)
                    } else {
                        None
                    }
                }

                /// Read a single word from the RX FIFO (blocking)
                #[inline]
                pub fn read_word(&self) -> u16 {
                    while !self.spi.sspsr.rne() {
                        core::hint::spin_loop();
                    }
                    self.spi.sspdr.data() as u16
                }

                /// Write multiple bytes (8-bit mode)
                pub fn write(&self, data: &[u8]) {
                    for &byte in data {
                        self.write_word(byte as u16);
                    }
                    // Wait for completion and drain RX FIFO
                    self.flush();
                    while self.try_read_word().is_some() {}
                }

                /// Write multiple 16-bit words
                pub fn write_words(&self, data: &[u16]) {
                    for &word in data {
                        self.write_word(word);
                    }
                    // Wait for completion and drain RX FIFO
                    self.flush();
                    while self.try_read_word().is_some() {}
                }

                /// Transfer data (write and read simultaneously)
                ///
                /// For each byte written, one byte is read.
                pub fn transfer(&self, read: &mut [u8], write: &[u8]) {
                    let len = read.len().min(write.len());
                    let mut tx_idx = 0;
                    let mut rx_idx = 0;

                    while rx_idx < len {
                        // Fill TX FIFO
                        while tx_idx < len && self.spi.sspsr.tnf() {
                            self.spi.sspdr.write_data(write[tx_idx] as u32);
                            tx_idx += 1;
                        }

                        // Read from RX FIFO
                        while rx_idx < tx_idx && self.spi.sspsr.rne() {
                            read[rx_idx] = self.spi.sspdr.data() as u8;
                            rx_idx += 1;
                        }
                    }
                }

                /// Transfer data in-place (same buffer for read and write)
                pub fn transfer_in_place(&self, data: &mut [u8]) {
                    let mut tx_idx = 0;
                    let mut rx_idx = 0;
                    let len = data.len();

                    while rx_idx < len {
                        // Fill TX FIFO
                        while tx_idx < len && self.spi.sspsr.tnf() {
                            self.spi.sspdr.write_data(data[tx_idx] as u32);
                            tx_idx += 1;
                        }

                        // Read from RX FIFO
                        while rx_idx < tx_idx && self.spi.sspsr.rne() {
                            data[rx_idx] = self.spi.sspdr.data() as u8;
                            rx_idx += 1;
                        }
                    }
                }

                /// Read data by writing dummy bytes
                pub fn read(&self, data: &mut [u8]) {
                    let mut tx_idx = 0;
                    let mut rx_idx = 0;
                    let len = data.len();

                    while rx_idx < len {
                        // Fill TX FIFO with dummy bytes
                        while tx_idx < len && self.spi.sspsr.tnf() {
                            self.spi.sspdr.write_data(0);
                            tx_idx += 1;
                        }

                        // Read from RX FIFO
                        while rx_idx < tx_idx && self.spi.sspsr.rne() {
                            data[rx_idx] = self.spi.sspdr.data() as u8;
                            rx_idx += 1;
                        }
                    }
                }

                /// Flush - wait for all transfers to complete
                #[inline]
                pub fn flush(&self) {
                    while self.spi.sspsr.bsy() {
                        core::hint::spin_loop();
                    }
                }

                // ================================================================
                // Interrupts
                // ================================================================

                /// Enable TX FIFO interrupt (half empty or less)
                #[inline]
                pub fn enable_tx_interrupt(&self) {
                    self.spi.sspimsc.set_txim();
                }

                /// Disable TX FIFO interrupt
                #[inline]
                pub fn disable_tx_interrupt(&self) {
                    self.spi.sspimsc.clr_txim();
                }

                /// Enable RX FIFO interrupt (half full or more)
                #[inline]
                pub fn enable_rx_interrupt(&self) {
                    self.spi.sspimsc.set_rxim();
                }

                /// Disable RX FIFO interrupt
                #[inline]
                pub fn disable_rx_interrupt(&self) {
                    self.spi.sspimsc.clr_rxim();
                }

                /// Enable RX timeout interrupt
                #[inline]
                pub fn enable_rx_timeout_interrupt(&self) {
                    self.spi.sspimsc.set_rtim();
                }

                /// Disable RX timeout interrupt
                #[inline]
                pub fn disable_rx_timeout_interrupt(&self) {
                    self.spi.sspimsc.clr_rtim();
                }

                /// Enable RX overrun interrupt
                #[inline]
                pub fn enable_rx_overrun_interrupt(&self) {
                    self.spi.sspimsc.set_rorim();
                }

                /// Disable RX overrun interrupt
                #[inline]
                pub fn disable_rx_overrun_interrupt(&self) {
                    self.spi.sspimsc.clr_rorim();
                }

                /// Check raw TX interrupt status
                #[inline]
                pub fn raw_tx_interrupt(&self) -> bool {
                    self.spi.sspris.txris()
                }

                /// Check raw RX interrupt status
                #[inline]
                pub fn raw_rx_interrupt(&self) -> bool {
                    self.spi.sspris.rxris()
                }

                /// Check raw RX timeout interrupt status
                #[inline]
                pub fn raw_rx_timeout_interrupt(&self) -> bool {
                    self.spi.sspris.rtris()
                }

                /// Check raw RX overrun interrupt status
                #[inline]
                pub fn raw_rx_overrun_interrupt(&self) -> bool {
                    self.spi.sspris.rorris()
                }

                /// Check masked TX interrupt status
                #[inline]
                pub fn masked_tx_interrupt(&self) -> bool {
                    self.spi.sspmis.txmis()
                }

                /// Check masked RX interrupt status
                #[inline]
                pub fn masked_rx_interrupt(&self) -> bool {
                    self.spi.sspmis.rxmis()
                }

                /// Clear RX timeout interrupt
                #[inline]
                pub fn clear_rx_timeout_interrupt(&self) {
                    self.spi.sspicr.set_rtic();
                }

                /// Clear RX overrun interrupt
                #[inline]
                pub fn clear_rx_overrun_interrupt(&self) {
                    self.spi.sspicr.set_roric();
                }

                // ================================================================
                // DMA
                // ================================================================

                /// DREQ selection for transmitting data via DMA.
                pub const TX_DREQ: crate::dma::TreqSel = crate::dma::TreqSel::[<Spi $spi_num Tx>];

                /// DREQ selection for receiving data via DMA.
                pub const RX_DREQ: crate::dma::TreqSel = crate::dma::TreqSel::[<Spi $spi_num Rx>];

                /// Enable DMA for transmit
                #[inline]
                pub fn enable_dma_tx(&self) {
                    self.spi.sspdmacr.set_txdmae();
                }

                /// Disable DMA for transmit
                #[inline]
                pub fn disable_dma_tx(&self) {
                    self.spi.sspdmacr.clr_txdmae();
                }

                /// Enable DMA for receive
                #[inline]
                pub fn enable_dma_rx(&self) {
                    self.spi.sspdmacr.set_rxdmae();
                }

                /// Disable DMA for receive
                #[inline]
                pub fn disable_dma_rx(&self) {
                    self.spi.sspdmacr.clr_rxdmae();
                }

                /// Get the data register address for DMA
                #[inline]
                pub fn data_register_address(&self) -> u32 {
                    <$SpiType>::PTR as u32 + 0x08 // SSPDR offset
                }

                // ================================================================
                // Loopback
                // ================================================================

                /// Enable loopback mode (TX output fed back to RX input)
                #[inline]
                pub fn enable_loopback(&mut self) {
                    self.spi.sspcr1.set_lbm();
                }

                /// Disable loopback mode
                #[inline]
                pub fn disable_loopback(&mut self) {
                    self.spi.sspcr1.clr_lbm();
                }
            }

            impl<SCK, TX, RX> SpiBus for $SpiName<SCK, TX, RX>
            where
                SCK: $SckPin,
                TX: $TxPin,
                RX: $RxPin,
            {
                #[inline]
                fn write(&self, data: &[u8]) {
                    Self::write(self, data);
                }

                #[inline]
                fn read(&self, data: &mut [u8]) {
                    Self::read(self, data);
                }

                #[inline]
                fn transfer(&self, read: &mut [u8], write: &[u8]) {
                    Self::transfer(self, read, write);
                }

                #[inline]
                fn transfer_in_place(&self, data: &mut [u8]) {
                    Self::transfer_in_place(self, data);
                }

                #[inline]
                fn flush(&self) {
                    Self::flush(self);
                }
            }

            // ================================================================
            // SPI with hardware chip select (Master/Slave modes)
            // ================================================================

            /// SPI peripheral wrapper with hardware chip select.
            ///
            /// This struct owns the SPI peripheral and all four pins (SCK, TX, RX, CSn).
            /// It supports both master and slave modes.
            ///
            /// # Master Mode
            ///
            /// Use [`init()`](Self::init) to initialize in master mode. The hardware
            /// controls the CSn pin automatically during transfers.
            ///
            /// # Slave Mode
            ///
            /// Use [`init_slave()`](Self::init_slave) to initialize in slave mode.
            /// The CSn pin is used by the master to select this slave device.
            ///
            #[doc = concat!("# SPI", $spi_num, " Pin Mapping")]
            ///
            #[doc = concat!("See [`", stringify!($SckPin), "`] for valid SCK pins.")]
            ///
            #[doc = concat!("See [`", stringify!($TxPin), "`] for valid TX pins.")]
            ///
            #[doc = concat!("See [`", stringify!($RxPin), "`] for valid RX pins.")]
            ///
            #[doc = concat!("See [`", stringify!($CsnPin), "`] for valid CSn pins.")]
            ///
            /// # Example (Master Mode)
            ///
            /// ```ignore
            #[doc = concat!("let spi = ", stringify!($SpiName), "WithCs::init(")]
            #[doc = concat!("    dp.SPI", $spi_num, ",")]
            ///     pins.pin::<2>(),      // SCK
            ///     pins.pin::<3>(),      // TX (MOSI)
            ///     pins.pin::<0>(),      // RX (MISO)
            ///     pins.pin::<1>(),      // CSn
            ///     1_000_000,    // 1 MHz
            ///     150_000_000,  // System clock
            /// );
            /// ```
            ///
            /// # Example (Slave Mode)
            ///
            /// ```ignore
            #[doc = concat!("let spi = ", stringify!($SpiName), "WithCs::init_slave(")]
            #[doc = concat!("    dp.SPI", $spi_num, ",")]
            ///     pins.pin::<2>(),      // SCK (input from master)
            ///     pins.pin::<3>(),      // TX (MISO - output to master)
            ///     pins.pin::<0>(),      // RX (MOSI - input from master)
            ///     pins.pin::<1>(),      // CSn (chip select from master)
            ///     150_000_000,  // System clock
            /// );
            ///
            /// // Wait for data from master
            /// while spi.is_rx_empty() {}
            /// let data = spi.read_word();
            ///
            /// // Send response to master
            /// spi.write_word(0x42);
            /// ```
            pub struct [<$SpiName WithCs>]<SCK, TX, RX, CSN>
            where
                SCK: $SckPin,
                TX: $TxPin,
                RX: $RxPin,
                CSN: $CsnPin,
            {
                spi: $SpiType,
                _sck: SCK,
                _tx: TX,
                _rx: RX,
                _csn: CSN,
            }

            impl<SCK, TX, RX, CSN> [<$SpiName WithCs>]<SCK, TX, RX, CSN>
            where
                SCK: $SckPin,
                TX: $TxPin,
                RX: $RxPin,
                CSN: $CsnPin,
            {
                /// SPI peripheral number
                pub const NUM: u8 = $spi_num;

                /// Obtain a pin-erased SPI handle without ownership (with-CS variant).
                ///
                /// # Safety
                ///
                /// Same requirements as [`$SpiName::steal()`].
                #[inline]
                pub unsafe fn steal() -> crate::Steal<[<$SpiName WithCs>]<
                    crate::gpio::pins::NoPin,
                    crate::gpio::pins::NoPin,
                    crate::gpio::pins::NoPin,
                    crate::gpio::pins::NoPin,
                >> {
                    crate::Steal([<$SpiName WithCs>] {
                        spi: unsafe { <$SpiType>::steal() },
                        _sck: crate::gpio::pins::NoPin,
                        _tx: crate::gpio::pins::NoPin,
                        _rx: crate::gpio::pins::NoPin,
                        _csn: crate::gpio::pins::NoPin,
                    })
                }

                /// Get reference to the underlying register block.
                ///
                /// # Safety
                /// Direct register access can bypass the HAL's safety guarantees.
                /// The caller must ensure operations do not conflict with the HAL state.
                #[inline]
                pub unsafe fn regs(&self) -> &pico_rs_pac::$spi_mod::RegisterBlock {
                    &self.spi
                }

                /// Initialize SPI in master mode with hardware chip select.
                ///
                /// # Arguments
                ///
                /// * `spi` - The SPI peripheral from PAC
                /// * `sck` - SCK (clock) pin
                /// * `tx` - TX (MOSI) pin
                /// * `rx` - RX (MISO) pin
                /// * `csn` - CSn (chip select) pin
                /// * `baudrate` - Desired baudrate in Hz
                /// * `peri_clk` - Peripheral clock frequency in Hz
                pub fn init(
                    spi: $SpiType,
                    mut sck: SCK,
                    mut tx: TX,
                    mut rx: RX,
                    mut csn: CSN,
                    baudrate: u32,
                    peri_clk: u32,
                ) -> Self {
                    // Configure pins for SPI function
                    sck.[<configure_spi $spi_num _sck>]();
                    tx.[<configure_spi $spi_num _tx>]();
                    rx.[<configure_spi $spi_num _rx>]();
                    csn.[<configure_spi $spi_num _csn>]();

                    // Reset peripheral to ensure clean register state
                    <$SpiType as Resettable>::init();

                    let mut spi_dev = Self {
                        spi,
                        _sck: sck,
                        _tx: tx,
                        _rx: rx,
                        _csn: csn,
                    };

                    // Disable first
                    spi_dev.spi.sspcr1.clr_sse();

                    // Set baudrate
                    spi_dev.configure_baudrate(baudrate, peri_clk);

                    // Set default format: 8 data bits, Mode0, Motorola SPI
                    spi_dev.set_format(8, Mode::Mode0, FrameFormat::MotorolaSpi);

                    // Configure as master (MS=0)
                    spi_dev.spi.sspcr1.clr_ms();

                    // Enable
                    spi_dev.spi.sspcr1.set_sse();

                    spi_dev
                }

                /// Initialize SPI in slave mode.
                ///
                /// In slave mode, CSn is required as it's used by the master
                /// to select this device.
                ///
                /// # Arguments
                ///
                /// * `spi` - The SPI peripheral from PAC
                /// * `sck` - SCK (clock) pin
                /// * `tx` - TX pin (slave output)
                /// * `rx` - RX pin (slave input)
                /// * `csn` - CSn (chip select) pin
                /// * `peri_clk` - Peripheral clock frequency in Hz
                pub fn init_slave(
                    spi: $SpiType,
                    mut sck: SCK,
                    mut tx: TX,
                    mut rx: RX,
                    mut csn: CSN,
                    peri_clk: u32,
                ) -> Self {
                    // Configure pins for SPI function
                    sck.[<configure_spi $spi_num _sck>]();
                    tx.[<configure_spi $spi_num _tx>]();
                    rx.[<configure_spi $spi_num _rx>]();
                    csn.[<configure_spi $spi_num _csn>]();

                    // Reset peripheral to ensure clean register state
                    <$SpiType as Resettable>::init();

                    let mut spi_dev = Self {
                        spi,
                        _sck: sck,
                        _tx: tx,
                        _rx: rx,
                        _csn: csn,
                    };

                    // Disable first
                    spi_dev.spi.sspcr1.clr_sse();

                    // Set a reasonable default baudrate for timing
                    spi_dev.configure_baudrate(1_000_000, peri_clk);

                    // Set default format
                    spi_dev.set_format(8, Mode::Mode0, FrameFormat::MotorolaSpi);

                    // Configure as slave (MS=1)
                    spi_dev.spi.sspcr1.set_ms();

                    // Enable
                    spi_dev.spi.sspcr1.set_sse();

                    spi_dev
                }

                /// Release the SPI peripheral and all pins.
                #[inline]
                pub fn release(self) -> ($SpiType, SCK, TX, RX, CSN) {
                    (self.spi, self._sck, self._tx, self._rx, self._csn)
                }

                /// Configure the SPI baudrate (internal helper)
                fn configure_baudrate(&mut self, baudrate: u32, peri_clk: u32) -> u32 {
                    let mut best_cpsdvsr = 2u32;
                    let mut best_scr = 0u32;
                    let mut best_error = u32::MAX;

                    for cpsdvsr in (2u32..=254).step_by(2) {
                        let scr = peri_clk / (cpsdvsr * baudrate);
                        let scr = if scr > 0 { scr - 1 } else { 0 };

                        if scr > 255 {
                            continue;
                        }

                        let actual = peri_clk / (cpsdvsr * (1 + scr));
                        let error = if actual > baudrate {
                            actual - baudrate
                        } else {
                            baudrate - actual
                        };

                        if error < best_error {
                            best_error = error;
                            best_cpsdvsr = cpsdvsr;
                            best_scr = scr;
                        }

                        if error == 0 {
                            break;
                        }
                    }

                    self.spi.sspcpsr.write_cpsdvsr(best_cpsdvsr);
                    let cr0 = self.spi.sspcr0.read_bits();
                    let cr0 = (cr0 & 0x00ff) | (best_scr << 8);
                    self.spi.sspcr0.write_bits(cr0);

                    peri_clk / (best_cpsdvsr * (1 + best_scr))
                }

                /// Set the SPI baudrate
                pub fn set_baudrate(&mut self, baudrate: u32, peri_clk: u32) -> u32 {
                    self.spi.sspcr1.clr_sse();
                    let actual = self.configure_baudrate(baudrate, peri_clk);
                    self.spi.sspcr1.set_sse();
                    actual
                }

                /// Set the data format
                pub fn set_format(&mut self, data_bits: u8, mode: Mode, format: FrameFormat) {
                    let data_bits = data_bits.clamp(4, 16);
                    let scr = self.spi.sspcr0.scr();
                    let cr0 = (scr << 8)
                        | (if mode.phase() { 1 << 7 } else { 0 })
                        | (if mode.polarity() { 1 << 6 } else { 0 })
                        | ((format as u32) << 4)
                        | ((data_bits - 1) as u32);
                    self.spi.sspcr0.write_bits(cr0);
                }

                /// Set the SPI mode
                pub fn set_mode(&mut self, mode: Mode) {
                    if mode.polarity() {
                        self.spi.sspcr0.set_spo();
                    } else {
                        self.spi.sspcr0.clr_spo();
                    }

                    if mode.phase() {
                        self.spi.sspcr0.set_sph();
                    } else {
                        self.spi.sspcr0.clr_sph();
                    }
                }

                /// Enable/disable the SPI peripheral
                #[inline]
                pub fn set_enabled(&mut self, enable: bool) {
                    if enable {
                        self.spi.sspcr1.set_sse();
                    } else {
                        self.spi.sspcr1.clr_sse();
                    }
                }

                /// Get the current baudrate by reading hardware registers
                ///
                /// # Arguments
                /// * `peri_clk` - Peripheral clock frequency in Hz
                #[inline]
                pub fn baudrate(&self, peri_clk: u32) -> u32 {
                    let cpsdvsr = self.spi.sspcpsr.cpsdvsr();
                    let scr = self.spi.sspcr0.scr();
                    let divisor = cpsdvsr * (1 + scr);
                    if divisor == 0 {
                        return 0;
                    }
                    peri_clk / divisor
                }

                /// Check if the SPI is busy
                #[inline]
                pub fn is_busy(&self) -> bool {
                    self.spi.sspsr.bsy()
                }

                /// Check if the TX FIFO is full
                #[inline]
                pub fn is_tx_full(&self) -> bool {
                    !self.spi.sspsr.tnf()
                }

                /// Check if the RX FIFO is empty
                #[inline]
                pub fn is_rx_empty(&self) -> bool {
                    !self.spi.sspsr.rne()
                }

                /// Write a single word to the TX FIFO (blocking)
                #[inline]
                pub fn write_word(&self, word: u16) {
                    while !self.spi.sspsr.tnf() {
                        core::hint::spin_loop();
                    }
                    self.spi.sspdr.write_data(word as u32);
                }

                /// Read a single word from the RX FIFO (blocking)
                #[inline]
                pub fn read_word(&self) -> u16 {
                    while !self.spi.sspsr.rne() {
                        core::hint::spin_loop();
                    }
                    self.spi.sspdr.data() as u16
                }

                /// Read a single word from the RX FIFO (non-blocking)
                #[inline]
                pub fn try_read_word(&self) -> Option<u16> {
                    if self.spi.sspsr.rne() {
                        Some(self.spi.sspdr.data() as u16)
                    } else {
                        None
                    }
                }

                /// Write multiple bytes (8-bit mode)
                pub fn write(&self, data: &[u8]) {
                    for &byte in data {
                        self.write_word(byte as u16);
                    }
                    self.flush();
                    while self.try_read_word().is_some() {}
                }

                /// Transfer data (write and read simultaneously)
                pub fn transfer(&self, read: &mut [u8], write: &[u8]) {
                    let len = read.len().min(write.len());
                    let mut tx_idx = 0;
                    let mut rx_idx = 0;

                    while rx_idx < len {
                        while tx_idx < len && self.spi.sspsr.tnf() {
                            self.spi.sspdr.write_data(write[tx_idx] as u32);
                            tx_idx += 1;
                        }

                        while rx_idx < tx_idx && self.spi.sspsr.rne() {
                            read[rx_idx] = self.spi.sspdr.data() as u8;
                            rx_idx += 1;
                        }
                    }
                }

                /// Transfer data in-place
                pub fn transfer_in_place(&self, data: &mut [u8]) {
                    let mut tx_idx = 0;
                    let mut rx_idx = 0;
                    let len = data.len();

                    while rx_idx < len {
                        while tx_idx < len && self.spi.sspsr.tnf() {
                            self.spi.sspdr.write_data(data[tx_idx] as u32);
                            tx_idx += 1;
                        }

                        while rx_idx < tx_idx && self.spi.sspsr.rne() {
                            data[rx_idx] = self.spi.sspdr.data() as u8;
                            rx_idx += 1;
                        }
                    }
                }

                /// Read data by writing dummy bytes
                pub fn read(&self, data: &mut [u8]) {
                    let mut tx_idx = 0;
                    let mut rx_idx = 0;
                    let len = data.len();

                    while rx_idx < len {
                        while tx_idx < len && self.spi.sspsr.tnf() {
                            self.spi.sspdr.write_data(0);
                            tx_idx += 1;
                        }

                        while rx_idx < tx_idx && self.spi.sspsr.rne() {
                            data[rx_idx] = self.spi.sspdr.data() as u8;
                            rx_idx += 1;
                        }
                    }
                }

                /// Flush - wait for all transfers to complete
                #[inline]
                pub fn flush(&self) {
                    while self.spi.sspsr.bsy() {
                        core::hint::spin_loop();
                    }
                }

                /// Enable DMA for transmit
                #[inline]
                pub fn enable_dma_tx(&self) {
                    self.spi.sspdmacr.set_txdmae();
                }

                /// Disable DMA for transmit
                #[inline]
                pub fn disable_dma_tx(&self) {
                    self.spi.sspdmacr.clr_txdmae();
                }

                /// Enable DMA for receive
                #[inline]
                pub fn enable_dma_rx(&self) {
                    self.spi.sspdmacr.set_rxdmae();
                }

                /// Disable DMA for receive
                #[inline]
                pub fn disable_dma_rx(&self) {
                    self.spi.sspdmacr.clr_rxdmae();
                }

                /// Get the data register address for DMA
                #[inline]
                pub fn data_register_address(&self) -> u32 {
                    <$SpiType>::PTR as u32 + 0x08
                }

                /// DREQ selection for transmitting data via DMA.
                pub const TX_DREQ: crate::dma::TreqSel = crate::dma::TreqSel::[<Spi $spi_num Tx>];

                /// DREQ selection for receiving data via DMA.
                pub const RX_DREQ: crate::dma::TreqSel = crate::dma::TreqSel::[<Spi $spi_num Rx>];

                // ================================================================
                // Status (additional methods)
                // ================================================================

                /// Check if the TX FIFO is empty
                #[inline]
                pub fn is_tx_empty(&self) -> bool {
                    self.spi.sspsr.tfe()
                }

                /// Check if the RX FIFO is full
                #[inline]
                pub fn is_rx_full(&self) -> bool {
                    self.spi.sspsr.rff()
                }

                /// Check if readable (RX FIFO has data)
                #[inline]
                pub fn is_readable(&self) -> bool {
                    self.spi.sspsr.rne()
                }

                /// Check if writable (TX FIFO has space)
                #[inline]
                pub fn is_writable(&self) -> bool {
                    self.spi.sspsr.tnf()
                }

                // ================================================================
                // Interrupts
                // ================================================================

                /// Enable TX FIFO interrupt (half empty or less)
                #[inline]
                pub fn enable_tx_interrupt(&self) {
                    self.spi.sspimsc.set_txim();
                }

                /// Disable TX FIFO interrupt
                #[inline]
                pub fn disable_tx_interrupt(&self) {
                    self.spi.sspimsc.clr_txim();
                }

                /// Enable RX FIFO interrupt (half full or more)
                #[inline]
                pub fn enable_rx_interrupt(&self) {
                    self.spi.sspimsc.set_rxim();
                }

                /// Disable RX FIFO interrupt
                #[inline]
                pub fn disable_rx_interrupt(&self) {
                    self.spi.sspimsc.clr_rxim();
                }

                /// Enable RX timeout interrupt
                #[inline]
                pub fn enable_rx_timeout_interrupt(&self) {
                    self.spi.sspimsc.set_rtim();
                }

                /// Disable RX timeout interrupt
                #[inline]
                pub fn disable_rx_timeout_interrupt(&self) {
                    self.spi.sspimsc.clr_rtim();
                }

                /// Enable RX overrun interrupt
                #[inline]
                pub fn enable_rx_overrun_interrupt(&self) {
                    self.spi.sspimsc.set_rorim();
                }

                /// Disable RX overrun interrupt
                #[inline]
                pub fn disable_rx_overrun_interrupt(&self) {
                    self.spi.sspimsc.clr_rorim();
                }

                /// Check raw TX interrupt status
                #[inline]
                pub fn raw_tx_interrupt(&self) -> bool {
                    self.spi.sspris.txris()
                }

                /// Check raw RX interrupt status
                #[inline]
                pub fn raw_rx_interrupt(&self) -> bool {
                    self.spi.sspris.rxris()
                }

                /// Check raw RX timeout interrupt status
                #[inline]
                pub fn raw_rx_timeout_interrupt(&self) -> bool {
                    self.spi.sspris.rtris()
                }

                /// Check raw RX overrun interrupt status
                #[inline]
                pub fn raw_rx_overrun_interrupt(&self) -> bool {
                    self.spi.sspris.rorris()
                }

                /// Check masked TX interrupt status
                #[inline]
                pub fn masked_tx_interrupt(&self) -> bool {
                    self.spi.sspmis.txmis()
                }

                /// Check masked RX interrupt status
                #[inline]
                pub fn masked_rx_interrupt(&self) -> bool {
                    self.spi.sspmis.rxmis()
                }

                /// Clear RX timeout interrupt
                #[inline]
                pub fn clear_rx_timeout_interrupt(&self) {
                    self.spi.sspicr.set_rtic();
                }

                /// Clear RX overrun interrupt
                #[inline]
                pub fn clear_rx_overrun_interrupt(&self) {
                    self.spi.sspicr.set_roric();
                }

                // ================================================================
                // Loopback
                // ================================================================

                /// Enable loopback mode (TX output fed back to RX input)
                #[inline]
                pub fn enable_loopback(&mut self) {
                    self.spi.sspcr1.set_lbm();
                }

                /// Disable loopback mode
                #[inline]
                pub fn disable_loopback(&mut self) {
                    self.spi.sspcr1.clr_lbm();
                }

                // ================================================================
                // Slave-specific methods
                // ================================================================

                /// Check if this SPI is configured as slave.
                ///
                /// Returns true if MS bit is set (slave mode).
                #[inline]
                pub fn is_slave(&self) -> bool {
                    self.spi.sspcr1.ms()
                }

                /// Disable slave output (SOD).
                ///
                /// When set, the SPI will not drive the TX pin in slave mode.
                /// Useful for multi-slave configurations where only one slave
                /// should respond at a time.
                #[inline]
                pub fn set_slave_output_disabled(&self, disabled: bool) {
                    if disabled {
                        self.spi.sspcr1.set_sod();
                    } else {
                        self.spi.sspcr1.clr_sod();
                    }
                }

                /// Check if slave output is disabled.
                #[inline]
                pub fn is_slave_output_disabled(&self) -> bool {
                    self.spi.sspcr1.sod()
                }
            }

            impl<SCK, TX, RX, CSN> SpiBus for [<$SpiName WithCs>]<SCK, TX, RX, CSN>
            where
                SCK: $SckPin,
                TX: $TxPin,
                RX: $RxPin,
                CSN: $CsnPin,
            {
                #[inline]
                fn write(&self, data: &[u8]) {
                    Self::write(self, data);
                }

                #[inline]
                fn read(&self, data: &mut [u8]) {
                    Self::read(self, data);
                }

                #[inline]
                fn transfer(&self, read: &mut [u8], write: &[u8]) {
                    Self::transfer(self, read, write);
                }

                #[inline]
                fn transfer_in_place(&self, data: &mut [u8]) {
                    Self::transfer_in_place(self, data);
                }

                #[inline]
                fn flush(&self) {
                    Self::flush(self);
                }
            }
        }
    };
}

// ============================================================================
// Generate SPI0 and SPI1
// ============================================================================

impl_spi!(
    Spi0,
    0,
    pico_rs_pac::SPI0,
    spi0,
    Spi0SckPin,
    Spi0TxPin,
    Spi0RxPin,
    Spi0CsnPin
);
impl_spi!(
    Spi1,
    1,
    pico_rs_pac::SPI1,
    spi1,
    Spi1SckPin,
    Spi1TxPin,
    Spi1RxPin,
    Spi1CsnPin
);
