//! SPI (Serial Peripheral Interface) HAL for RP2350
//!
//! This module provides a safe interface for the RP2350 SPI peripherals.
//! The RP2350 has two SPI controllers (SPI0 and SPI1) based on the ARM PrimeCell SSP (PL022).
//!
//! ## Features
//!
//! - Master and slave modes
//! - 4-16 data bits
//! - Clock polarity and phase configuration (Mode 0-3)
//! - Motorola SPI, TI Synchronous Serial, and Microwire frame formats
//! - 8-entry TX and RX FIFOs
//! - DMA support
//!
//! ## GPIO Mapping
//!
//! SPI0:
//! - RX: GPIO 0, 4, 16, 20
//! - CSn: GPIO 1, 5, 17, 21
//! - SCK: GPIO 2, 6, 18, 22
//! - TX: GPIO 3, 7, 19, 23
//!
//! SPI1:
//! - RX: GPIO 8, 12, 24, 28
//! - CSn: GPIO 9, 13, 25, 29
//! - SCK: GPIO 10, 14, 26
//! - TX: GPIO 11, 15, 27
//!
//! ## Example
//!
//! ```ignore
//! use pico_rs::spi::{Spi0, Mode};
//!
//! // Initialize SPI0 at 1 MHz
//! let mut spi = Spi0::init(1_000_000, 150_000_000);
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
//! ```

use paste::paste;

use crate::reset::{self, Peripheral};

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

// ============================================================================
// Macro for generating SPI implementations
// ============================================================================

macro_rules! impl_spi {
    ($SpiName:ident, $spi_num:literal, $SpiType:ty, $spi_mod:ident, $reset_peripheral:expr) => {
        paste! {
            /// SPI$spi_num peripheral wrapper
            pub struct $SpiName {
                regs: &'static pico_rs_pac::$spi_mod::RegisterBlock,
                baudrate: u32,
            }

            impl $SpiName {
                /// SPI peripheral number
                pub const NUM: u8 = $spi_num;

                /// Create a new SPI instance without initialization
                #[inline]
                pub fn new() -> Self {
                    Self {
                        regs: unsafe { &*<$SpiType>::PTR },
                        baudrate: 0,
                    }
                }

                /// Get reference to the register block
                #[inline]
                pub fn regs(&self) -> &pico_rs_pac::$spi_mod::RegisterBlock {
                    self.regs
                }

                // ================================================================
                // Initialization
                // ================================================================

                /// Initialize the SPI peripheral with default settings (8 data bits, Mode0)
                ///
                /// # Arguments
                /// * `baudrate` - Desired baudrate in Hz
                /// * `peri_clk` - Peripheral clock frequency in Hz
                pub fn init(baudrate: u32, peri_clk: u32) -> Self {
                    // Bring SPI out of reset
                    reset::unreset_block_wait($reset_peripheral);

                    let mut spi = Self::new();

                    // Disable first
                    spi.regs.sspcr1.clr_sse();

                    // Set baudrate
                    spi.configure_baudrate(baudrate, peri_clk);

                    // Set default format: 8 data bits, Mode0, Motorola SPI
                    spi.set_format(8, Mode::Mode0, FrameFormat::MotorolaSpi);

                    // Configure as master (MS=0), no loopback
                    spi.regs.sspcr1.write_bits(0);

                    // Enable
                    spi.regs.sspcr1.set_sse();

                    spi
                }

                /// Initialize the SPI peripheral in slave mode
                ///
                /// # Arguments
                /// * `peri_clk` - Peripheral clock frequency in Hz
                pub fn init_slave(peri_clk: u32) -> Self {
                    // Bring SPI out of reset
                    reset::unreset_block_wait($reset_peripheral);

                    let mut spi = Self::new();

                    // Disable first
                    spi.regs.sspcr1.clr_sse();

                    // Set a reasonable default baudrate for timing
                    spi.configure_baudrate(1_000_000, peri_clk);

                    // Set default format
                    spi.set_format(8, Mode::Mode0, FrameFormat::MotorolaSpi);

                    // Configure as slave (MS=1)
                    spi.regs.sspcr1.set_ms();

                    // Enable
                    spi.regs.sspcr1.set_sse();

                    spi
                }

                // ================================================================
                // Configuration
                // ================================================================

                /// Configure the SPI baudrate (internal helper)
                ///
                /// Baudrate = peri_clk / (CPSDVSR * (1 + SCR))
                fn configure_baudrate(&mut self, baudrate: u32, peri_clk: u32) {
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
                    self.regs.sspcpsr.write_cpsdvsr(best_cpsdvsr);

                    // Update SCR in CR0 (bits 15:8)
                    let cr0 = self.regs.sspcr0.read_bits();
                    let cr0 = (cr0 & 0x00ff) | (best_scr << 8);
                    self.regs.sspcr0.write_bits(cr0);

                    self.baudrate = peri_clk / (best_cpsdvsr * (1 + best_scr));
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
                    self.regs.sspcr1.clr_sse();
                    self.configure_baudrate(baudrate, peri_clk);
                    self.regs.sspcr1.set_sse();
                    self.baudrate
                }

                /// Set the data format
                ///
                /// # Arguments
                /// * `data_bits` - Number of data bits (4-16)
                /// * `mode` - SPI mode (clock polarity and phase)
                /// * `format` - Frame format
                pub fn set_format(&self, data_bits: u8, mode: Mode, format: FrameFormat) {
                    let data_bits = data_bits.clamp(4, 16);

                    // Read current CR0 to preserve SCR
                    let scr = self.regs.sspcr0.scr();

                    // Build new CR0 value
                    let cr0 = (scr << 8)
                        | (if mode.phase() { 1 << 7 } else { 0 })
                        | (if mode.polarity() { 1 << 6 } else { 0 })
                        | ((format as u32) << 4)
                        | ((data_bits - 1) as u32);

                    self.regs.sspcr0.write_bits(cr0);
                }

                /// Set the SPI mode
                pub fn set_mode(&self, mode: Mode) {
                    if mode.polarity() {
                        self.regs.sspcr0.set_spo();
                    } else {
                        self.regs.sspcr0.clr_spo();
                    }

                    if mode.phase() {
                        self.regs.sspcr0.set_sph();
                    } else {
                        self.regs.sspcr0.clr_sph();
                    }
                }

                /// Enable/disable the SPI peripheral
                #[inline]
                pub fn set_enabled(&self, enable: bool) {
                    if enable {
                        self.regs.sspcr1.set_sse();
                    } else {
                        self.regs.sspcr1.clr_sse();
                    }
                }

                /// Get the current baudrate
                #[inline]
                pub fn baudrate(&self) -> u32 {
                    self.baudrate
                }

                // ================================================================
                // Status
                // ================================================================

                /// Check if the SPI is busy
                #[inline]
                pub fn is_busy(&self) -> bool {
                    self.regs.sspsr.bsy()
                }

                /// Check if the TX FIFO is full
                #[inline]
                pub fn is_tx_full(&self) -> bool {
                    !self.regs.sspsr.tnf()
                }

                /// Check if the TX FIFO is empty
                #[inline]
                pub fn is_tx_empty(&self) -> bool {
                    self.regs.sspsr.tfe()
                }

                /// Check if the RX FIFO is empty
                #[inline]
                pub fn is_rx_empty(&self) -> bool {
                    !self.regs.sspsr.rne()
                }

                /// Check if the RX FIFO is full
                #[inline]
                pub fn is_rx_full(&self) -> bool {
                    self.regs.sspsr.rff()
                }

                /// Check if readable (RX FIFO has data)
                #[inline]
                pub fn is_readable(&self) -> bool {
                    self.regs.sspsr.rne()
                }

                /// Check if writable (TX FIFO has space)
                #[inline]
                pub fn is_writable(&self) -> bool {
                    self.regs.sspsr.tnf()
                }

                // ================================================================
                // Data Transfer
                // ================================================================

                /// Write a single word to the TX FIFO (non-blocking)
                ///
                /// Returns true if the word was written, false if FIFO is full.
                #[inline]
                pub fn try_write_word(&self, word: u16) -> bool {
                    if self.regs.sspsr.tnf() {
                        self.regs.sspdr.write_data(word as u32);
                        true
                    } else {
                        false
                    }
                }

                /// Write a single word to the TX FIFO (blocking)
                #[inline]
                pub fn write_word(&self, word: u16) {
                    while !self.regs.sspsr.tnf() {
                        core::hint::spin_loop();
                    }
                    self.regs.sspdr.write_data(word as u32);
                }

                /// Read a single word from the RX FIFO (non-blocking)
                ///
                /// Returns Some(word) if data available, None if FIFO is empty.
                #[inline]
                pub fn try_read_word(&self) -> Option<u16> {
                    if self.regs.sspsr.rne() {
                        Some(self.regs.sspdr.data() as u16)
                    } else {
                        None
                    }
                }

                /// Read a single word from the RX FIFO (blocking)
                #[inline]
                pub fn read_word(&self) -> u16 {
                    while !self.regs.sspsr.rne() {
                        core::hint::spin_loop();
                    }
                    self.regs.sspdr.data() as u16
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
                        while tx_idx < len && self.regs.sspsr.tnf() {
                            self.regs.sspdr.write_data(write[tx_idx] as u32);
                            tx_idx += 1;
                        }

                        // Read from RX FIFO
                        while rx_idx < tx_idx && self.regs.sspsr.rne() {
                            read[rx_idx] = self.regs.sspdr.data() as u8;
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
                        while tx_idx < len && self.regs.sspsr.tnf() {
                            self.regs.sspdr.write_data(data[tx_idx] as u32);
                            tx_idx += 1;
                        }

                        // Read from RX FIFO
                        while rx_idx < tx_idx && self.regs.sspsr.rne() {
                            data[rx_idx] = self.regs.sspdr.data() as u8;
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
                        while tx_idx < len && self.regs.sspsr.tnf() {
                            self.regs.sspdr.write_data(0);
                            tx_idx += 1;
                        }

                        // Read from RX FIFO
                        while rx_idx < tx_idx && self.regs.sspsr.rne() {
                            data[rx_idx] = self.regs.sspdr.data() as u8;
                            rx_idx += 1;
                        }
                    }
                }

                /// Flush - wait for all transfers to complete
                #[inline]
                pub fn flush(&self) {
                    while self.regs.sspsr.bsy() {
                        core::hint::spin_loop();
                    }
                }

                // ================================================================
                // Interrupts
                // ================================================================

                /// Enable TX FIFO interrupt (half empty or less)
                #[inline]
                pub fn enable_tx_interrupt(&self) {
                    self.regs.sspimsc.set_txim();
                }

                /// Disable TX FIFO interrupt
                #[inline]
                pub fn disable_tx_interrupt(&self) {
                    self.regs.sspimsc.clr_txim();
                }

                /// Enable RX FIFO interrupt (half full or more)
                #[inline]
                pub fn enable_rx_interrupt(&self) {
                    self.regs.sspimsc.set_rxim();
                }

                /// Disable RX FIFO interrupt
                #[inline]
                pub fn disable_rx_interrupt(&self) {
                    self.regs.sspimsc.clr_rxim();
                }

                /// Enable RX timeout interrupt
                #[inline]
                pub fn enable_rx_timeout_interrupt(&self) {
                    self.regs.sspimsc.set_rtim();
                }

                /// Disable RX timeout interrupt
                #[inline]
                pub fn disable_rx_timeout_interrupt(&self) {
                    self.regs.sspimsc.clr_rtim();
                }

                /// Enable RX overrun interrupt
                #[inline]
                pub fn enable_rx_overrun_interrupt(&self) {
                    self.regs.sspimsc.set_rorim();
                }

                /// Disable RX overrun interrupt
                #[inline]
                pub fn disable_rx_overrun_interrupt(&self) {
                    self.regs.sspimsc.clr_rorim();
                }

                /// Check raw TX interrupt status
                #[inline]
                pub fn raw_tx_interrupt(&self) -> bool {
                    self.regs.sspris.txris()
                }

                /// Check raw RX interrupt status
                #[inline]
                pub fn raw_rx_interrupt(&self) -> bool {
                    self.regs.sspris.rxris()
                }

                /// Check raw RX timeout interrupt status
                #[inline]
                pub fn raw_rx_timeout_interrupt(&self) -> bool {
                    self.regs.sspris.rtris()
                }

                /// Check raw RX overrun interrupt status
                #[inline]
                pub fn raw_rx_overrun_interrupt(&self) -> bool {
                    self.regs.sspris.rorris()
                }

                /// Check masked TX interrupt status
                #[inline]
                pub fn masked_tx_interrupt(&self) -> bool {
                    self.regs.sspmis.txmis()
                }

                /// Check masked RX interrupt status
                #[inline]
                pub fn masked_rx_interrupt(&self) -> bool {
                    self.regs.sspmis.rxmis()
                }

                /// Clear RX timeout interrupt
                #[inline]
                pub fn clear_rx_timeout_interrupt(&self) {
                    self.regs.sspicr.set_rtic();
                }

                /// Clear RX overrun interrupt
                #[inline]
                pub fn clear_rx_overrun_interrupt(&self) {
                    self.regs.sspicr.set_roric();
                }

                // ================================================================
                // DMA
                // ================================================================

                /// Enable DMA for transmit
                #[inline]
                pub fn enable_dma_tx(&self) {
                    self.regs.sspdmacr.set_txdmae();
                }

                /// Disable DMA for transmit
                #[inline]
                pub fn disable_dma_tx(&self) {
                    self.regs.sspdmacr.clr_txdmae();
                }

                /// Enable DMA for receive
                #[inline]
                pub fn enable_dma_rx(&self) {
                    self.regs.sspdmacr.set_rxdmae();
                }

                /// Disable DMA for receive
                #[inline]
                pub fn disable_dma_rx(&self) {
                    self.regs.sspdmacr.clr_rxdmae();
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
                pub fn enable_loopback(&self) {
                    self.regs.sspcr1.set_lbm();
                }

                /// Disable loopback mode
                #[inline]
                pub fn disable_loopback(&self) {
                    self.regs.sspcr1.clr_lbm();
                }
            }

            impl Default for $SpiName {
                fn default() -> Self {
                    Self::new()
                }
            }
        }
    };
}

// ============================================================================
// Generate SPI0 and SPI1
// ============================================================================

impl_spi!(Spi0, 0, pico_rs_pac::SPI0, spi0, Peripheral::SPI0);
impl_spi!(Spi1, 1, pico_rs_pac::SPI1, spi1, Peripheral::SPI1);
