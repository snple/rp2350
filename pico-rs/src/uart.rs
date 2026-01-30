//! UART (Universal Asynchronous Receiver/Transmitter) HAL for RP2350
//!
//! This module provides a safe interface for the RP2350 UART peripherals.
//! The RP2350 has two UART controllers (UART0 and UART1).
//!
//! ## Features
//!
//! - Configurable baud rate (up to ~7.8 Mbps at 125 MHz sys clock)
//! - 5-8 data bits
//! - 1 or 2 stop bits
//! - Optional parity (none, odd, even)
//! - 32-byte TX and RX FIFOs
//! - Hardware flow control (RTS/CTS)
//!
//! ## GPIO Mapping
//!
//! UART0:
//! - TX: GPIO 0, 12, 16, 28
//! - RX: GPIO 1, 13, 17, 29
//! - CTS: GPIO 2, 14, 18
//! - RTS: GPIO 3, 15, 19
//!
//! UART1:
//! - TX: GPIO 4, 8, 20, 24
//! - RX: GPIO 5, 9, 21, 25
//! - CTS: GPIO 6, 10, 22, 26
//! - RTS: GPIO 7, 11, 23, 27
//!
//! ## Example
//!
//! ```ignore
//! use pico_rs::uart::{Uart0, DataBits, StopBits, Parity};
//!
//! // Initialize UART0 at 115200 baud
//! let uart = Uart0::init(115200, 150_000_000);
//!
//! // Configure format (optional, default is 8N1)
//! uart.set_format(DataBits::Eight, StopBits::One, Parity::None);
//!
//! // Write data
//! uart.write(b"Hello, UART!\r\n");
//!
//! // Read byte
//! if let Some(byte) = uart.read_byte() {
//!     // Process received byte
//! }
//! ```

use paste::paste;

use crate::reset::{self, Peripheral};

/// Number of data bits
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DataBits {
    /// 5 data bits
    Five = 0,
    /// 6 data bits
    Six = 1,
    /// 7 data bits
    Seven = 2,
    /// 8 data bits
    Eight = 3,
}

/// Number of stop bits
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StopBits {
    /// 1 stop bit
    One,
    /// 2 stop bits
    Two,
}

/// Parity configuration
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Parity {
    /// No parity
    None,
    /// Even parity
    Even,
    /// Odd parity
    Odd,
}

/// UART error types
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// Overrun error - data lost due to RX FIFO full
    Overrun,
    /// Parity error
    ParityError,
    /// Framing error - invalid stop bit
    FramingError,
    /// Break condition detected
    BreakError,
}

// ============================================================================
// Macro for generating UART implementations
// ============================================================================

macro_rules! impl_uart {
    ($UartName:ident, $uart_num:literal, $UartType:ty, $uart_mod:ident, $reset_peripheral:expr) => {
        paste! {
            /// UART$uart_num peripheral wrapper
            pub struct $UartName {
                regs: &'static pico_rs_pac::$uart_mod::RegisterBlock,
                baudrate: u32,
            }

            impl $UartName {
                /// UART peripheral number
                pub const NUM: u8 = $uart_num;

                /// Create a new UART instance without initialization
                #[inline]
                pub fn new() -> Self {
                    Self {
                        regs: unsafe { &*<$UartType>::PTR },
                        baudrate: 0,
                    }
                }

                /// Get reference to the register block
                #[inline]
                pub fn regs(&self) -> &pico_rs_pac::$uart_mod::RegisterBlock {
                    self.regs
                }

                /// Initialize the UART peripheral with default 8N1 format
                ///
                /// # Arguments
                /// * `baudrate` - Desired baud rate
                /// * `peri_clk` - Peripheral clock frequency in Hz (usually sys_clk)
                pub fn init(baudrate: u32, peri_clk: u32) -> Self {
                    // Bring UART out of reset
                    reset::unreset_block_wait($reset_peripheral);

                    let mut uart = Self::new();

                    // Set baud rate
                    uart.configure_baudrate(baudrate, peri_clk);

                    // Set default format: 8 data bits, 1 stop bit, no parity
                    uart.set_format(DataBits::Eight, StopBits::One, Parity::None);

                    // Enable UART, TX, and RX
                    uart.regs.uartcr.set_uarten();
                    uart.regs.uartcr.set_txe();
                    uart.regs.uartcr.set_rxe();

                    // Enable FIFOs
                    uart.regs.uartlcr_h.set_fen();

                    // Set TX and RX FIFO trigger levels to 1/2
                    uart.regs.uartifls.write_txiflsel(2);
                    uart.regs.uartifls.write_rxiflsel(2);

                    uart
                }

                // ================================================================
                // Configuration
                // ================================================================

                /// Configure the UART baud rate (internal helper)
                fn configure_baudrate(&mut self, baudrate: u32, peri_clk: u32) {
                    // Baud rate divider = peri_clk / (16 * baudrate)
                    // We use a fixed-point calculation with 6 fractional bits
                    // BRD = (64 * peri_clk) / (16 * baudrate) = (4 * peri_clk) / baudrate
                    let baud_div_fixed = (4 * peri_clk as u64) / baudrate as u64;
                    let ibrd = (baud_div_fixed >> 6) as u32;
                    let fbrd = (baud_div_fixed & 0x3f) as u32;

                    // Write baud rate registers
                    self.regs.uartibrd.write_baud_divint(ibrd);
                    self.regs.uartfbrd.write_baud_divfrac(fbrd);

                    // Dummy write to UARTLCR_H to latch the baud rate
                    let lcr = self.regs.uartlcr_h.read_bits();
                    self.regs.uartlcr_h.write_bits(lcr);

                    // Calculate actual baud rate
                    let actual = (4 * peri_clk as u64) / (64 * ibrd as u64 + fbrd as u64);
                    self.baudrate = actual as u32;
                }

                /// Set the UART baud rate
                ///
                /// # Arguments
                /// * `baudrate` - Desired baud rate
                /// * `peri_clk` - Peripheral clock frequency in Hz
                ///
                /// # Returns
                /// The actual baud rate achieved
                pub fn set_baudrate(&mut self, baudrate: u32, peri_clk: u32) -> u32 {
                    self.configure_baudrate(baudrate, peri_clk);
                    self.baudrate
                }

                /// Set the data format
                ///
                /// # Arguments
                /// * `data_bits` - Number of data bits (5-8)
                /// * `stop_bits` - Number of stop bits (1 or 2)
                /// * `parity` - Parity configuration
                pub fn set_format(&self, data_bits: DataBits, stop_bits: StopBits, parity: Parity) {
                    // Set word length (WLEN)
                    self.regs.uartlcr_h.write_wlen(data_bits as u32);

                    // Set stop bits (STP2)
                    if stop_bits == StopBits::Two {
                        self.regs.uartlcr_h.set_stp2();
                    } else {
                        self.regs.uartlcr_h.clr_stp2();
                    }

                    // Set parity
                    match parity {
                        Parity::None => {
                            self.regs.uartlcr_h.clr_pen();
                            self.regs.uartlcr_h.clr_eps();
                        }
                        Parity::Even => {
                            self.regs.uartlcr_h.set_pen();
                            self.regs.uartlcr_h.set_eps();
                        }
                        Parity::Odd => {
                            self.regs.uartlcr_h.set_pen();
                            self.regs.uartlcr_h.clr_eps();
                        }
                    }
                }

                /// Enable/disable the UART
                #[inline]
                pub fn set_enabled(&self, enable: bool) {
                    if enable {
                        self.regs.uartcr.set_uarten();
                    } else {
                        self.regs.uartcr.clr_uarten();
                    }
                }

                /// Enable/disable TX
                #[inline]
                pub fn set_tx_enabled(&self, enable: bool) {
                    if enable {
                        self.regs.uartcr.set_txe();
                    } else {
                        self.regs.uartcr.clr_txe();
                    }
                }

                /// Enable/disable RX
                #[inline]
                pub fn set_rx_enabled(&self, enable: bool) {
                    if enable {
                        self.regs.uartcr.set_rxe();
                    } else {
                        self.regs.uartcr.clr_rxe();
                    }
                }

                /// Enable/disable hardware flow control
                ///
                /// # Arguments
                /// * `cts` - Enable CTS flow control
                /// * `rts` - Enable RTS flow control
                pub fn set_hw_flow(&self, cts: bool, rts: bool) {
                    if cts {
                        self.regs.uartcr.set_ctsen();
                    } else {
                        self.regs.uartcr.clr_ctsen();
                    }
                    if rts {
                        self.regs.uartcr.set_rtsen();
                    } else {
                        self.regs.uartcr.clr_rtsen();
                    }
                }

                /// Set FIFO trigger levels
                ///
                /// # Arguments
                /// * `tx_level` - TX FIFO trigger level (0-4, corresponding to 1/8, 1/4, 1/2, 3/4, 7/8)
                /// * `rx_level` - RX FIFO trigger level (0-4, corresponding to 1/8, 1/4, 1/2, 3/4, 7/8)
                pub fn set_fifo_levels(&self, tx_level: u8, rx_level: u8) {
                    self.regs.uartifls.write_txiflsel(tx_level.min(4) as u32);
                    self.regs.uartifls.write_rxiflsel(rx_level.min(4) as u32);
                }

                // ================================================================
                // Data Transfer
                // ================================================================

                /// Write a single byte (blocking)
                pub fn write_byte(&self, byte: u8) {
                    // Wait for TX FIFO to have space
                    while self.regs.uartfr.txff() {}
                    self.regs.uartdr.write_data(byte as u32);
                }

                /// Write multiple bytes (blocking)
                pub fn write(&self, data: &[u8]) {
                    for &byte in data {
                        self.write_byte(byte);
                    }
                }

                /// Write a string (blocking)
                pub fn write_str(&self, s: &str) {
                    self.write(s.as_bytes());
                }

                /// Read a single byte (non-blocking)
                ///
                /// Returns `None` if no data is available
                pub fn read_byte(&self) -> Option<u8> {
                    // Check if RX FIFO has data
                    if self.regs.uartfr.rxfe() {
                        return None;
                    }
                    Some(self.regs.uartdr.data() as u8)
                }

                /// Read a single byte with error information
                ///
                /// Returns the byte and any associated error
                pub fn read_byte_with_error(&self) -> Option<Result<u8, (u8, Error)>> {
                    if self.regs.uartfr.rxfe() {
                        return None;
                    }

                    let byte = self.regs.uartdr.data() as u8;

                    // Check error bits using accessor methods
                    if self.regs.uartdr.oe() {
                        Some(Err((byte, Error::Overrun)))
                    } else if self.regs.uartdr.be() {
                        Some(Err((byte, Error::BreakError)))
                    } else if self.regs.uartdr.pe() {
                        Some(Err((byte, Error::ParityError)))
                    } else if self.regs.uartdr.fe() {
                        Some(Err((byte, Error::FramingError)))
                    } else {
                        Some(Ok(byte))
                    }
                }

                /// Read a single byte (blocking)
                pub fn read_byte_blocking(&self) -> u8 {
                    loop {
                        if let Some(byte) = self.read_byte() {
                            return byte;
                        }
                    }
                }

                /// Read multiple bytes (blocking)
                pub fn read(&self, buffer: &mut [u8]) {
                    for slot in buffer.iter_mut() {
                        *slot = self.read_byte_blocking();
                    }
                }

                /// Read available bytes into buffer (non-blocking)
                ///
                /// Returns the number of bytes read
                pub fn read_raw(&self, buffer: &mut [u8]) -> usize {
                    let mut count = 0;
                    for slot in buffer.iter_mut() {
                        if let Some(byte) = self.read_byte() {
                            *slot = byte;
                            count += 1;
                        } else {
                            break;
                        }
                    }
                    count
                }

                /// Try to write a byte without blocking
                ///
                /// Returns `true` if the byte was written
                pub fn try_write_byte(&self, byte: u8) -> bool {
                    if self.regs.uartfr.txff() {
                        return false;
                    }
                    self.regs.uartdr.write_data(byte as u32);
                    true
                }

                // ================================================================
                // Status
                // ================================================================

                /// Check if TX FIFO is full
                #[inline]
                pub fn is_tx_full(&self) -> bool {
                    self.regs.uartfr.txff()
                }

                /// Check if TX FIFO is empty
                #[inline]
                pub fn is_tx_empty(&self) -> bool {
                    self.regs.uartfr.txfe()
                }

                /// Check if RX FIFO is full
                #[inline]
                pub fn is_rx_full(&self) -> bool {
                    self.regs.uartfr.rxff()
                }

                /// Check if RX FIFO is empty
                #[inline]
                pub fn is_rx_empty(&self) -> bool {
                    self.regs.uartfr.rxfe()
                }

                /// Check if UART is busy transmitting
                #[inline]
                pub fn is_busy(&self) -> bool {
                    self.regs.uartfr.busy()
                }

                /// Check if there are bytes available to read
                #[inline]
                pub fn is_readable(&self) -> bool {
                    !self.is_rx_empty()
                }

                /// Check if there's space in the TX FIFO
                #[inline]
                pub fn is_writable(&self) -> bool {
                    !self.is_tx_full()
                }

                /// Wait for transmission to complete
                pub fn flush(&self) {
                    // Wait for TX FIFO to empty
                    while !self.regs.uartfr.txfe() {}
                    // Wait for not busy
                    while self.regs.uartfr.busy() {}
                }

                /// Clear all errors
                pub fn clear_errors(&self) {
                    // Clear all error flags using PAC methods
                    self.regs.uartrsr.clr_oe();
                    self.regs.uartrsr.clr_be();
                    self.regs.uartrsr.clr_pe();
                    self.regs.uartrsr.clr_fe();
                }

                /// Get the current baud rate
                #[inline]
                pub fn baudrate(&self) -> u32 {
                    self.baudrate
                }

                // ================================================================
                // Interrupts
                // ================================================================

                /// Enable specific interrupts using mask
                ///
                /// # Arguments
                /// * `mask` - Interrupt bits to enable (use UART_IRQ_* constants)
                pub fn enable_interrupts(&self, mask: u16) {
                    self.regs.uartimsc.set_bits(mask as u32);
                }

                /// Disable specific interrupts using mask
                pub fn disable_interrupts(&self, mask: u16) {
                    self.regs.uartimsc.clr_bits(mask as u32);
                }

                /// Enable receive interrupt (RX FIFO reaches threshold)
                #[inline]
                pub fn enable_rx_interrupt(&self) {
                    self.regs.uartimsc.set_rxim();
                }

                /// Disable receive interrupt
                #[inline]
                pub fn disable_rx_interrupt(&self) {
                    self.regs.uartimsc.clr_rxim();
                }

                /// Enable transmit interrupt (TX FIFO below threshold)
                #[inline]
                pub fn enable_tx_interrupt(&self) {
                    self.regs.uartimsc.set_txim();
                }

                /// Disable transmit interrupt
                #[inline]
                pub fn disable_tx_interrupt(&self) {
                    self.regs.uartimsc.clr_txim();
                }

                /// Enable receive timeout interrupt
                #[inline]
                pub fn enable_rx_timeout_interrupt(&self) {
                    self.regs.uartimsc.set_rtim();
                }

                /// Disable receive timeout interrupt
                #[inline]
                pub fn disable_rx_timeout_interrupt(&self) {
                    self.regs.uartimsc.clr_rtim();
                }

                /// Enable overrun error interrupt
                #[inline]
                pub fn enable_overrun_error_interrupt(&self) {
                    self.regs.uartimsc.set_oeim();
                }

                /// Disable overrun error interrupt
                #[inline]
                pub fn disable_overrun_error_interrupt(&self) {
                    self.regs.uartimsc.clr_oeim();
                }

                /// Enable break error interrupt
                #[inline]
                pub fn enable_break_error_interrupt(&self) {
                    self.regs.uartimsc.set_beim();
                }

                /// Disable break error interrupt
                #[inline]
                pub fn disable_break_error_interrupt(&self) {
                    self.regs.uartimsc.clr_beim();
                }

                /// Enable parity error interrupt
                #[inline]
                pub fn enable_parity_error_interrupt(&self) {
                    self.regs.uartimsc.set_peim();
                }

                /// Disable parity error interrupt
                #[inline]
                pub fn disable_parity_error_interrupt(&self) {
                    self.regs.uartimsc.clr_peim();
                }

                /// Enable framing error interrupt
                #[inline]
                pub fn enable_framing_error_interrupt(&self) {
                    self.regs.uartimsc.set_feim();
                }

                /// Disable framing error interrupt
                #[inline]
                pub fn disable_framing_error_interrupt(&self) {
                    self.regs.uartimsc.clr_feim();
                }

                /// Enable all error interrupts (OE, BE, PE, FE)
                #[inline]
                pub fn enable_error_interrupts(&self) {
                    self.regs.uartimsc.set_oeim();
                    self.regs.uartimsc.set_beim();
                    self.regs.uartimsc.set_peim();
                    self.regs.uartimsc.set_feim();
                }

                /// Disable all error interrupts
                #[inline]
                pub fn disable_error_interrupts(&self) {
                    self.regs.uartimsc.clr_oeim();
                    self.regs.uartimsc.clr_beim();
                    self.regs.uartimsc.clr_peim();
                    self.regs.uartimsc.clr_feim();
                }

                /// Disable all interrupts
                #[inline]
                pub fn disable_all_interrupts(&self) {
                    // Clear all interrupt mask bits (11 bits total)
                    self.regs.uartimsc.clr_bits(0x7ff);
                }

                /// Check if RX interrupt is enabled
                #[inline]
                pub fn is_rx_interrupt_enabled(&self) -> bool {
                    self.regs.uartimsc.rxim()
                }

                /// Check if TX interrupt is enabled
                #[inline]
                pub fn is_tx_interrupt_enabled(&self) -> bool {
                    self.regs.uartimsc.txim()
                }

                /// Get raw interrupt status (before masking)
                #[inline]
                pub fn raw_interrupt_status(&self) -> u16 {
                    (self.regs.uartris.read_bits() & 0x7ff) as u16
                }

                /// Get masked interrupt status (after masking)
                #[inline]
                pub fn masked_interrupt_status(&self) -> u16 {
                    (self.regs.uartmis.read_bits() & 0x7ff) as u16
                }

                /// Check if RX interrupt is pending (masked)
                #[inline]
                pub fn is_rx_interrupt_pending(&self) -> bool {
                    self.regs.uartmis.rxmis()
                }

                /// Check if TX interrupt is pending (masked)
                #[inline]
                pub fn is_tx_interrupt_pending(&self) -> bool {
                    self.regs.uartmis.txmis()
                }

                /// Check if RX timeout interrupt is pending (masked)
                #[inline]
                pub fn is_rx_timeout_interrupt_pending(&self) -> bool {
                    self.regs.uartmis.rtmis()
                }

                /// Check if any error interrupt is pending (masked)
                #[inline]
                pub fn is_error_interrupt_pending(&self) -> bool {
                    self.regs.uartmis.oemis() ||
                    self.regs.uartmis.bemis() ||
                    self.regs.uartmis.pemis() ||
                    self.regs.uartmis.femis()
                }

                /// Clear specific interrupts using mask
                pub fn clear_interrupts(&self, mask: u16) {
                    self.regs.uarticr.set_bits(mask as u32);
                }

                /// Clear RX interrupt
                #[inline]
                pub fn clear_rx_interrupt(&self) {
                    self.regs.uarticr.set_rxic();
                }

                /// Clear TX interrupt
                #[inline]
                pub fn clear_tx_interrupt(&self) {
                    self.regs.uarticr.set_txic();
                }

                /// Clear RX timeout interrupt
                #[inline]
                pub fn clear_rx_timeout_interrupt(&self) {
                    self.regs.uarticr.set_rtic();
                }

                /// Clear all error interrupts
                #[inline]
                pub fn clear_error_interrupts(&self) {
                    self.regs.uarticr.set_oeic();
                    self.regs.uarticr.set_beic();
                    self.regs.uarticr.set_peic();
                    self.regs.uarticr.set_feic();
                }

                /// Clear all pending interrupts
                #[inline]
                pub fn clear_all_interrupts(&self) {
                    self.regs.uarticr.set_bits(0x7ff);
                }
            }

            impl Default for $UartName {
                fn default() -> Self {
                    Self::new()
                }
            }

            impl core::fmt::Write for $UartName {
                fn write_str(&mut self, s: &str) -> core::fmt::Result {
                    self.write(s.as_bytes());
                    Ok(())
                }
            }
        }
    };
}

// ============================================================================
// Generate UART0 and UART1
// ============================================================================

impl_uart!(Uart0, 0, pico_rs_pac::UART0, uart0, Peripheral::UART0);
impl_uart!(Uart1, 1, pico_rs_pac::UART1, uart1, Peripheral::UART1);

// ============================================================================
// Interrupt bit constants
// ============================================================================

/// Ring Indicator modem interrupt
pub const UART_IRQ_RI: u16 = 1 << 0;
/// Clear To Send modem interrupt
pub const UART_IRQ_CTS: u16 = 1 << 1;
/// Data Carrier Detect modem interrupt
pub const UART_IRQ_DCD: u16 = 1 << 2;
/// Data Set Ready modem interrupt
pub const UART_IRQ_DSR: u16 = 1 << 3;
/// Receive interrupt
pub const UART_IRQ_RX: u16 = 1 << 4;
/// Transmit interrupt
pub const UART_IRQ_TX: u16 = 1 << 5;
/// Receive timeout interrupt
pub const UART_IRQ_RT: u16 = 1 << 6;
/// Framing error interrupt
pub const UART_IRQ_FE: u16 = 1 << 7;
/// Parity error interrupt
pub const UART_IRQ_PE: u16 = 1 << 8;
/// Break error interrupt
pub const UART_IRQ_BE: u16 = 1 << 9;
/// Overrun error interrupt
pub const UART_IRQ_OE: u16 = 1 << 10;
