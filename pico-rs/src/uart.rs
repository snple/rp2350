//! UART (Universal Asynchronous Receiver/Transmitter) HAL for RP2350
//!
//! This module provides a safe interface for the RP2350 UART peripherals.
//! The RP2350 has two UART controllers (UART0 and UART1).
//!
//! # Features
//!
//! - Configurable baud rate (up to ~7.8 Mbps at 125 MHz sys clock)
//! - 5-8 data bits
//! - 1 or 2 stop bits
//! - Optional parity (none, odd, even)
//! - 32-byte TX and RX FIFOs
//! - Hardware flow control (RTS/CTS)
//! - **Type-safe pin ownership**: Pins are consumed by UART, preventing misuse
//!
//! # GPIO Mapping
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
//! # Example (Basic UART with Pin Ownership)
//!
//! ```ignore
//! use pico_rs::gpio::Pins;
//! use pico_rs::uart::{Uart0, DataBits, StopBits, Parity};
//! use pico_rs::Peripherals;
//!
//! let dp = Peripherals::take().unwrap();
//! let pins = Pins::new(dp.IO_BANK0, dp.PADS_BANK0);
//!
//! // Initialize UART0 at 115200 baud, consuming pins p0 (TX) and p1 (RX)
//! let mut uart = Uart0::new(
//!     dp.UART0,
//!     pins.pin::<0>(),      // TX pin - ownership transferred
//!     pins.pin::<1>(),      // RX pin - ownership transferred
//!     115200,
//!     150_000_000,
//! );
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
//!
//! // Compile error: pins.pin::<0>() already moved!
//! // pins.pin::<0>().into_output();
//! ```
//!
//! # Example (UART with Hardware Flow Control)
//!
//! ```ignore
//! use pico_rs::gpio::Pins;
//! use pico_rs::uart::Uart0;
//!
//! let dp = Peripherals::take().unwrap();
//! let pins = Pins::new(dp.IO_BANK0, dp.PADS_BANK0);
//!
//! // Initialize UART0 with CTS/RTS flow control
//! let mut uart = Uart0::new_with_flow_control(
//!     dp.UART0,
//!     pins.pin::<0>(),      // TX
//!     pins.pin::<1>(),      // RX
//!     pins.pin::<2>(),      // CTS
//!     pins.pin::<3>(),      // RTS
//!     115200,
//!     150_000_000,
//! );
//! ```

use paste::paste;

use crate::gpio::{
    Uart0CtsPin, Uart0RtsPin, Uart0RxPin, Uart0TxPin, Uart1CtsPin, Uart1RtsPin, Uart1RxPin,
    Uart1TxPin,
};
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
    ($UartName:ident, $uart_num:literal, $UartType:ty, $uart_mod:ident, $reset_peripheral:expr, $TxTrait:ident, $RxTrait:ident, $CtsTrait:ident, $RtsTrait:ident) => {
        paste! {
            /// UART peripheral wrapper with type-safe pin ownership.
            ///
            /// This struct owns both the UART peripheral and the GPIO pins used for
            /// TX and RX. The pins are configured automatically when the UART
            /// instance is created, and their ownership prevents them from being
            /// used for other purposes.
            ///
            #[doc = concat!("# UART", $uart_num, " Pin Mapping")]
            ///
            #[doc = concat!("See [`", stringify!($TxTrait), "`] for valid TX pins.")]
            ///
            #[doc = concat!("See [`", stringify!($RxTrait), "`] for valid RX pins.")]
            pub struct $UartName<TX, RX>
            where
                TX: $TxTrait,
                RX: $RxTrait,
            {
                uart: $UartType,
                _tx: TX,
                _rx: RX,
                baudrate: u32,
            }

            impl<TX, RX> $UartName<TX, RX>
            where
                TX: $TxTrait,
                RX: $RxTrait,
            {
                /// UART peripheral number
                pub const NUM: u8 = $uart_num;

                /// Get reference to the underlying register block.
                ///
                /// # Safety
                /// Direct register access can bypass the HAL's safety guarantees.
                /// The caller must ensure operations do not conflict with the HAL state.
                #[inline]
                pub unsafe fn regs(&self) -> &pico_rs_pac::$uart_mod::RegisterBlock {
                    &self.uart
                }

                // ================================================================
                // Initialization
                // ================================================================

                /// Initialize UART with TX and RX pins.
                ///
                /// This method consumes the UART peripheral and GPIO pins, configuring
                /// the pins for UART function automatically.
                ///
                /// # Arguments
                ///
                /// * `uart` - The UART peripheral from PAC
                /// * `tx` - TX pin (must implement the appropriate TX trait)
                /// * `rx` - RX pin (must implement the appropriate RX trait)
                /// * `baudrate` - Desired baudrate in Hz
                /// * `peri_clk` - Peripheral clock frequency in Hz (usually sys_clk)
                ///
                /// # Type Safety
                ///
                /// Only pins that implement the correct trait can be used:
                #[doc = concat!("- TX must implement [`", stringify!($TxTrait), "`]")]
                #[doc = concat!("- RX must implement [`", stringify!($RxTrait), "`]")]
                ///
                /// # Example
                ///
                /// ```ignore
                #[doc = concat!("let uart = ", stringify!($UartName), "::new(")]
                #[doc = concat!("    dp.UART", $uart_num, ",")]
                ///     pins.pin::<0>(),      // TX
                ///     pins.pin::<1>(),      // RX
                ///     115200,       // Baud rate
                ///     150_000_000,  // System clock
                /// );
                /// ```
                pub fn new(uart: $UartType, tx: TX, rx: RX, baudrate: u32, peri_clk: u32) -> Self {
                    // Configure pins for UART function
                    tx.[<configure_uart $uart_num _tx>]();
                    rx.[<configure_uart $uart_num _rx>]();

                    // Bring UART out of reset
                    reset::unreset_block_wait($reset_peripheral);

                    let mut uart_dev = Self {
                        uart,
                        _tx: tx,
                        _rx: rx,
                        baudrate: 0,
                    };

                    // Set baud rate
                    uart_dev.configure_baudrate(baudrate, peri_clk);

                    // Set default format: 8 data bits, 1 stop bit, no parity
                    uart_dev.set_format(DataBits::Eight, StopBits::One, Parity::None);

                    // Enable UART, TX, and RX
                    uart_dev.uart.uartcr.set_uarten();
                    uart_dev.uart.uartcr.set_txe();
                    uart_dev.uart.uartcr.set_rxe();

                    // Enable FIFOs
                    uart_dev.uart.uartlcr_h.set_fen();

                    // Set TX and RX FIFO trigger levels to 1/2
                    uart_dev.uart.uartifls.write_txiflsel(2);
                    uart_dev.uart.uartifls.write_rxiflsel(2);

                    uart_dev
                }

                /// Release the UART peripheral and pins.
                ///
                /// Returns a tuple of (UART peripheral, TX pin, RX pin).
                /// The pins can then be reconfigured for other uses.
                #[inline]
                pub fn release(self) -> ($UartType, TX, RX) {
                    (self.uart, self._tx, self._rx)
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
                    self.uart.uartibrd.write_baud_divint(ibrd);
                    self.uart.uartfbrd.write_baud_divfrac(fbrd);

                    // Dummy write to UARTLCR_H to latch the baud rate
                    let lcr = self.uart.uartlcr_h.read_bits();
                    self.uart.uartlcr_h.write_bits(lcr);

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
                    self.uart.uartlcr_h.write_wlen(data_bits as u32);

                    // Set stop bits (STP2)
                    if stop_bits == StopBits::Two {
                        self.uart.uartlcr_h.set_stp2();
                    } else {
                        self.uart.uartlcr_h.clr_stp2();
                    }

                    // Set parity
                    match parity {
                        Parity::None => {
                            self.uart.uartlcr_h.clr_pen();
                            self.uart.uartlcr_h.clr_eps();
                        }
                        Parity::Even => {
                            self.uart.uartlcr_h.set_pen();
                            self.uart.uartlcr_h.set_eps();
                        }
                        Parity::Odd => {
                            self.uart.uartlcr_h.set_pen();
                            self.uart.uartlcr_h.clr_eps();
                        }
                    }
                }

                /// Enable/disable the UART
                #[inline]
                pub fn set_enabled(&self, enable: bool) {
                    if enable {
                        self.uart.uartcr.set_uarten();
                    } else {
                        self.uart.uartcr.clr_uarten();
                    }
                }

                /// Enable/disable TX
                #[inline]
                pub fn set_tx_enabled(&self, enable: bool) {
                    if enable {
                        self.uart.uartcr.set_txe();
                    } else {
                        self.uart.uartcr.clr_txe();
                    }
                }

                /// Enable/disable RX
                #[inline]
                pub fn set_rx_enabled(&self, enable: bool) {
                    if enable {
                        self.uart.uartcr.set_rxe();
                    } else {
                        self.uart.uartcr.clr_rxe();
                    }
                }

                /// Set FIFO trigger levels
                ///
                /// # Arguments
                /// * `tx_level` - TX FIFO trigger level (0-4, corresponding to 1/8, 1/4, 1/2, 3/4, 7/8)
                /// * `rx_level` - RX FIFO trigger level (0-4, corresponding to 1/8, 1/4, 1/2, 3/4, 7/8)
                pub fn set_fifo_levels(&self, tx_level: u8, rx_level: u8) {
                    self.uart.uartifls.write_txiflsel(tx_level.min(4) as u32);
                    self.uart.uartifls.write_rxiflsel(rx_level.min(4) as u32);
                }

                // ================================================================
                // Data Transfer
                // ================================================================

                /// Write a single byte (blocking)
                pub fn write_byte(&self, byte: u8) {
                    // Wait for TX FIFO to have space
                    while self.uart.uartfr.txff() {}
                    self.uart.uartdr.write_data(byte as u32);
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
                    if self.uart.uartfr.rxfe() {
                        return None;
                    }
                    Some(self.uart.uartdr.data() as u8)
                }

                /// Read a single byte with error information
                ///
                /// Returns the byte and any associated error
                pub fn read_byte_with_error(&self) -> Option<Result<u8, (u8, Error)>> {
                    if self.uart.uartfr.rxfe() {
                        return None;
                    }

                    let byte = self.uart.uartdr.data() as u8;

                    // Check error bits using accessor methods
                    if self.uart.uartdr.oe() {
                        Some(Err((byte, Error::Overrun)))
                    } else if self.uart.uartdr.be() {
                        Some(Err((byte, Error::BreakError)))
                    } else if self.uart.uartdr.pe() {
                        Some(Err((byte, Error::ParityError)))
                    } else if self.uart.uartdr.fe() {
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
                    if self.uart.uartfr.txff() {
                        return false;
                    }
                    self.uart.uartdr.write_data(byte as u32);
                    true
                }

                // ================================================================
                // Status
                // ================================================================

                /// Check if TX FIFO is full
                #[inline]
                pub fn is_tx_full(&self) -> bool {
                    self.uart.uartfr.txff()
                }

                /// Check if TX FIFO is empty
                #[inline]
                pub fn is_tx_empty(&self) -> bool {
                    self.uart.uartfr.txfe()
                }

                /// Check if RX FIFO is full
                #[inline]
                pub fn is_rx_full(&self) -> bool {
                    self.uart.uartfr.rxff()
                }

                /// Check if RX FIFO is empty
                #[inline]
                pub fn is_rx_empty(&self) -> bool {
                    self.uart.uartfr.rxfe()
                }

                /// Check if UART is busy transmitting
                #[inline]
                pub fn is_busy(&self) -> bool {
                    self.uart.uartfr.busy()
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
                    while !self.uart.uartfr.txfe() {}
                    // Wait for not busy
                    while self.uart.uartfr.busy() {}
                }

                /// Clear all errors
                pub fn clear_errors(&self) {
                    // Clear all error flags using PAC methods
                    self.uart.uartrsr.clr_oe();
                    self.uart.uartrsr.clr_be();
                    self.uart.uartrsr.clr_pe();
                    self.uart.uartrsr.clr_fe();
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
                    self.uart.uartimsc.set_bits(mask as u32);
                }

                /// Disable specific interrupts using mask
                pub fn disable_interrupts(&self, mask: u16) {
                    self.uart.uartimsc.clr_bits(mask as u32);
                }

                /// Enable receive interrupt (RX FIFO reaches threshold)
                #[inline]
                pub fn enable_rx_interrupt(&self) {
                    self.uart.uartimsc.set_rxim();
                }

                /// Disable receive interrupt
                #[inline]
                pub fn disable_rx_interrupt(&self) {
                    self.uart.uartimsc.clr_rxim();
                }

                /// Enable transmit interrupt (TX FIFO below threshold)
                #[inline]
                pub fn enable_tx_interrupt(&self) {
                    self.uart.uartimsc.set_txim();
                }

                /// Disable transmit interrupt
                #[inline]
                pub fn disable_tx_interrupt(&self) {
                    self.uart.uartimsc.clr_txim();
                }

                /// Enable receive timeout interrupt
                #[inline]
                pub fn enable_rx_timeout_interrupt(&self) {
                    self.uart.uartimsc.set_rtim();
                }

                /// Disable receive timeout interrupt
                #[inline]
                pub fn disable_rx_timeout_interrupt(&self) {
                    self.uart.uartimsc.clr_rtim();
                }

                /// Enable overrun error interrupt
                #[inline]
                pub fn enable_overrun_error_interrupt(&self) {
                    self.uart.uartimsc.set_oeim();
                }

                /// Disable overrun error interrupt
                #[inline]
                pub fn disable_overrun_error_interrupt(&self) {
                    self.uart.uartimsc.clr_oeim();
                }

                /// Enable break error interrupt
                #[inline]
                pub fn enable_break_error_interrupt(&self) {
                    self.uart.uartimsc.set_beim();
                }

                /// Disable break error interrupt
                #[inline]
                pub fn disable_break_error_interrupt(&self) {
                    self.uart.uartimsc.clr_beim();
                }

                /// Enable parity error interrupt
                #[inline]
                pub fn enable_parity_error_interrupt(&self) {
                    self.uart.uartimsc.set_peim();
                }

                /// Disable parity error interrupt
                #[inline]
                pub fn disable_parity_error_interrupt(&self) {
                    self.uart.uartimsc.clr_peim();
                }

                /// Enable framing error interrupt
                #[inline]
                pub fn enable_framing_error_interrupt(&self) {
                    self.uart.uartimsc.set_feim();
                }

                /// Disable framing error interrupt
                #[inline]
                pub fn disable_framing_error_interrupt(&self) {
                    self.uart.uartimsc.clr_feim();
                }

                /// Enable all error interrupts (OE, BE, PE, FE)
                #[inline]
                pub fn enable_error_interrupts(&self) {
                    self.uart.uartimsc.set_oeim();
                    self.uart.uartimsc.set_beim();
                    self.uart.uartimsc.set_peim();
                    self.uart.uartimsc.set_feim();
                }

                /// Disable all error interrupts
                #[inline]
                pub fn disable_error_interrupts(&self) {
                    self.uart.uartimsc.clr_oeim();
                    self.uart.uartimsc.clr_beim();
                    self.uart.uartimsc.clr_peim();
                    self.uart.uartimsc.clr_feim();
                }

                /// Disable all interrupts
                #[inline]
                pub fn disable_all_interrupts(&self) {
                    // Clear all interrupt mask bits (11 bits total)
                    self.uart.uartimsc.clr_bits(0x7ff);
                }

                /// Check if RX interrupt is enabled
                #[inline]
                pub fn is_rx_interrupt_enabled(&self) -> bool {
                    self.uart.uartimsc.rxim()
                }

                /// Check if TX interrupt is enabled
                #[inline]
                pub fn is_tx_interrupt_enabled(&self) -> bool {
                    self.uart.uartimsc.txim()
                }

                /// Get raw interrupt status (before masking)
                #[inline]
                pub fn raw_interrupt_status(&self) -> u16 {
                    (self.uart.uartris.read_bits() & 0x7ff) as u16
                }

                /// Get masked interrupt status (after masking)
                #[inline]
                pub fn masked_interrupt_status(&self) -> u16 {
                    (self.uart.uartmis.read_bits() & 0x7ff) as u16
                }

                /// Check if RX interrupt is pending (masked)
                #[inline]
                pub fn is_rx_interrupt_pending(&self) -> bool {
                    self.uart.uartmis.rxmis()
                }

                /// Check if TX interrupt is pending (masked)
                #[inline]
                pub fn is_tx_interrupt_pending(&self) -> bool {
                    self.uart.uartmis.txmis()
                }

                /// Check if RX timeout interrupt is pending (masked)
                #[inline]
                pub fn is_rx_timeout_interrupt_pending(&self) -> bool {
                    self.uart.uartmis.rtmis()
                }

                /// Check if any error interrupt is pending (masked)
                #[inline]
                pub fn is_error_interrupt_pending(&self) -> bool {
                    self.uart.uartmis.oemis() ||
                    self.uart.uartmis.bemis() ||
                    self.uart.uartmis.pemis() ||
                    self.uart.uartmis.femis()
                }

                /// Clear specific interrupts using mask
                pub fn clear_interrupts(&self, mask: u16) {
                    self.uart.uarticr.set_bits(mask as u32);
                }

                /// Clear RX interrupt
                #[inline]
                pub fn clear_rx_interrupt(&self) {
                    self.uart.uarticr.set_rxic();
                }

                /// Clear TX interrupt
                #[inline]
                pub fn clear_tx_interrupt(&self) {
                    self.uart.uarticr.set_txic();
                }

                /// Clear RX timeout interrupt
                #[inline]
                pub fn clear_rx_timeout_interrupt(&self) {
                    self.uart.uarticr.set_rtic();
                }

                /// Clear all error interrupts
                #[inline]
                pub fn clear_error_interrupts(&self) {
                    self.uart.uarticr.set_oeic();
                    self.uart.uarticr.set_beic();
                    self.uart.uarticr.set_peic();
                    self.uart.uarticr.set_feic();
                }

                /// Clear all pending interrupts
                #[inline]
                pub fn clear_all_interrupts(&self) {
                    self.uart.uarticr.set_bits(0x7ff);
                }

                // ================================================================
                // DMA Support
                // ================================================================

                /// DREQ selection for transmitting data via DMA.
                pub const TX_DREQ: crate::dma::TreqSel = crate::dma::TreqSel::[<Uart $uart_num Tx>];

                /// DREQ selection for receiving data via DMA.
                pub const RX_DREQ: crate::dma::TreqSel = crate::dma::TreqSel::[<Uart $uart_num Rx>];

                /// Get the UART data register (UARTDR) address for DMA transfers.
                ///
                /// UARTDR sits at offset 0x00 from the UART base address, so
                /// this returns the peripheral base address directly.
                #[inline]
                pub fn data_register_address(&self) -> u32 {
                    <$UartType>::PTR as u32
                }

                /// Enable DMA requests for the transmit FIFO.
                #[inline]
                pub fn enable_dma_tx(&self) {
                    self.uart.uartdmacr.set_txdmae();
                }

                /// Disable DMA requests for the transmit FIFO.
                #[inline]
                pub fn disable_dma_tx(&self) {
                    self.uart.uartdmacr.clr_txdmae();
                }

                /// Enable DMA requests for the receive FIFO.
                #[inline]
                pub fn enable_dma_rx(&self) {
                    self.uart.uartdmacr.set_rxdmae();
                }

                /// Disable DMA requests for the receive FIFO.
                #[inline]
                pub fn disable_dma_rx(&self) {
                    self.uart.uartdmacr.clr_rxdmae();
                }
            }

            impl<TX, RX> core::fmt::Write for $UartName<TX, RX>
            where
                TX: $TxTrait,
                RX: $RxTrait,
            {
                fn write_str(&mut self, s: &str) -> core::fmt::Result {
                    self.write(s.as_bytes());
                    Ok(())
                }
            }

            // ================================================================
            // UART with hardware flow control
            // ================================================================

            /// UART peripheral wrapper with hardware flow control (CTS/RTS).
            ///
            /// This struct owns the UART peripheral and all four pins (TX, RX, CTS, RTS).
            #[doc = concat!("# UART", $uart_num, " with Flow Control")]
            pub struct [<$UartName WithFlowControl>]<TX, RX, CTS, RTS>
            where
                TX: $TxTrait,
                RX: $RxTrait,
                CTS: $CtsTrait,
                RTS: $RtsTrait,
            {
                uart: $UartType,
                _tx: TX,
                _rx: RX,
                _cts: CTS,
                _rts: RTS,
                baudrate: u32,
            }

            impl<TX, RX, CTS, RTS> [<$UartName WithFlowControl>]<TX, RX, CTS, RTS>
            where
                TX: $TxTrait,
                RX: $RxTrait,
                CTS: $CtsTrait,
                RTS: $RtsTrait,
            {
                /// UART peripheral number
                pub const NUM: u8 = $uart_num;

                /// Get reference to the underlying register block.
                ///
                /// # Safety
                /// Direct register access can bypass the HAL's safety guarantees.
                /// The caller must ensure operations do not conflict with the HAL state.
                #[inline]
                pub unsafe fn regs(&self) -> &pico_rs_pac::$uart_mod::RegisterBlock {
                    &self.uart
                }

                /// Initialize UART with hardware flow control.
                ///
                /// # Arguments
                ///
                /// * `uart` - The UART peripheral from PAC
                /// * `tx` - TX pin
                /// * `rx` - RX pin
                /// * `cts` - CTS pin (Clear To Send)
                /// * `rts` - RTS pin (Request To Send)
                /// * `baudrate` - Desired baudrate in Hz
                /// * `peri_clk` - Peripheral clock frequency in Hz
                pub fn new(
                    uart: $UartType,
                    tx: TX,
                    rx: RX,
                    cts: CTS,
                    rts: RTS,
                    baudrate: u32,
                    peri_clk: u32,
                ) -> Self {
                    // Configure pins for UART function
                    tx.[<configure_uart $uart_num _tx>]();
                    rx.[<configure_uart $uart_num _rx>]();
                    cts.[<configure_uart $uart_num _cts>]();
                    rts.[<configure_uart $uart_num _rts>]();

                    // Bring UART out of reset
                    reset::unreset_block_wait($reset_peripheral);

                    let mut uart_dev = Self {
                        uart,
                        _tx: tx,
                        _rx: rx,
                        _cts: cts,
                        _rts: rts,
                        baudrate: 0,
                    };

                    // Set baud rate
                    uart_dev.configure_baudrate(baudrate, peri_clk);

                    // Set default format: 8 data bits, 1 stop bit, no parity
                    uart_dev.set_format(DataBits::Eight, StopBits::One, Parity::None);

                    // Enable hardware flow control
                    uart_dev.uart.uartcr.set_ctsen();
                    uart_dev.uart.uartcr.set_rtsen();

                    // Enable UART, TX, and RX
                    uart_dev.uart.uartcr.set_uarten();
                    uart_dev.uart.uartcr.set_txe();
                    uart_dev.uart.uartcr.set_rxe();

                    // Enable FIFOs
                    uart_dev.uart.uartlcr_h.set_fen();

                    // Set TX and RX FIFO trigger levels to 1/2
                    uart_dev.uart.uartifls.write_txiflsel(2);
                    uart_dev.uart.uartifls.write_rxiflsel(2);

                    uart_dev
                }

                /// Release the UART peripheral and all pins.
                #[inline]
                pub fn release(self) -> ($UartType, TX, RX, CTS, RTS) {
                    (self.uart, self._tx, self._rx, self._cts, self._rts)
                }

                /// Configure the UART baud rate (internal helper)
                fn configure_baudrate(&mut self, baudrate: u32, peri_clk: u32) {
                    let baud_div_fixed = (4 * peri_clk as u64) / baudrate as u64;
                    let ibrd = (baud_div_fixed >> 6) as u32;
                    let fbrd = (baud_div_fixed & 0x3f) as u32;

                    self.uart.uartibrd.write_baud_divint(ibrd);
                    self.uart.uartfbrd.write_baud_divfrac(fbrd);

                    let lcr = self.uart.uartlcr_h.read_bits();
                    self.uart.uartlcr_h.write_bits(lcr);

                    let actual = (4 * peri_clk as u64) / (64 * ibrd as u64 + fbrd as u64);
                    self.baudrate = actual as u32;
                }

                /// Set the UART baud rate
                pub fn set_baudrate(&mut self, baudrate: u32, peri_clk: u32) -> u32 {
                    self.configure_baudrate(baudrate, peri_clk);
                    self.baudrate
                }

                /// Set the data format
                pub fn set_format(&self, data_bits: DataBits, stop_bits: StopBits, parity: Parity) {
                    self.uart.uartlcr_h.write_wlen(data_bits as u32);

                    if stop_bits == StopBits::Two {
                        self.uart.uartlcr_h.set_stp2();
                    } else {
                        self.uart.uartlcr_h.clr_stp2();
                    }

                    match parity {
                        Parity::None => {
                            self.uart.uartlcr_h.clr_pen();
                            self.uart.uartlcr_h.clr_eps();
                        }
                        Parity::Even => {
                            self.uart.uartlcr_h.set_pen();
                            self.uart.uartlcr_h.set_eps();
                        }
                        Parity::Odd => {
                            self.uart.uartlcr_h.set_pen();
                            self.uart.uartlcr_h.clr_eps();
                        }
                    }
                }

                /// Enable/disable flow control
                pub fn set_flow_control(&self, cts: bool, rts: bool) {
                    if cts {
                        self.uart.uartcr.set_ctsen();
                    } else {
                        self.uart.uartcr.clr_ctsen();
                    }
                    if rts {
                        self.uart.uartcr.set_rtsen();
                    } else {
                        self.uart.uartcr.clr_rtsen();
                    }
                }

                /// Write a single byte (blocking)
                pub fn write_byte(&self, byte: u8) {
                    while self.uart.uartfr.txff() {}
                    self.uart.uartdr.write_data(byte as u32);
                }

                /// Write multiple bytes (blocking)
                pub fn write(&self, data: &[u8]) {
                    for &byte in data {
                        self.write_byte(byte);
                    }
                }

                /// Read a single byte (non-blocking)
                pub fn read_byte(&self) -> Option<u8> {
                    if self.uart.uartfr.rxfe() {
                        return None;
                    }
                    Some(self.uart.uartdr.data() as u8)
                }

                /// Read a single byte (blocking)
                pub fn read_byte_blocking(&self) -> u8 {
                    loop {
                        if let Some(byte) = self.read_byte() {
                            return byte;
                        }
                    }
                }

                /// Check if TX FIFO is full
                #[inline]
                pub fn is_tx_full(&self) -> bool {
                    self.uart.uartfr.txff()
                }

                /// Check if RX FIFO is empty
                #[inline]
                pub fn is_rx_empty(&self) -> bool {
                    self.uart.uartfr.rxfe()
                }

                /// Check if CTS is asserted (remote device ready to receive)
                #[inline]
                pub fn is_cts_asserted(&self) -> bool {
                    self.uart.uartfr.cts()
                }

                /// Wait for transmission to complete
                pub fn flush(&self) {
                    while !self.uart.uartfr.txfe() {}
                    while self.uart.uartfr.busy() {}
                }

                /// Get the current baud rate
                #[inline]
                pub fn baudrate(&self) -> u32 {
                    self.baudrate
                }
            }

            impl<TX, RX, CTS, RTS> core::fmt::Write for [<$UartName WithFlowControl>]<TX, RX, CTS, RTS>
            where
                TX: $TxTrait,
                RX: $RxTrait,
                CTS: $CtsTrait,
                RTS: $RtsTrait,
            {
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

impl_uart!(
    Uart0,
    0,
    pico_rs_pac::UART0,
    uart0,
    Peripheral::UART0,
    Uart0TxPin,
    Uart0RxPin,
    Uart0CtsPin,
    Uart0RtsPin
);
impl_uart!(
    Uart1,
    1,
    pico_rs_pac::UART1,
    uart1,
    Peripheral::UART1,
    Uart1TxPin,
    Uart1RxPin,
    Uart1CtsPin,
    Uart1RtsPin
);

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
