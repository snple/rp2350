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
//! use rp2350::gpio::Pins;
//! use rp2350::uart::{Uart0, DataBits, StopBits, Parity};
//! use rp2350::Peripherals;
//!
//! let dp = Peripherals::take().unwrap();
//! let pins = Pins::init(dp.IO_BANK0, dp.PADS_BANK0);
//!
//! // Initialize UART0 at 115200 baud, consuming pins p0 (TX) and p1 (RX)
//! let mut uart = Uart0::init(
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
//! use rp2350::gpio::Pins;
//! use rp2350::uart::Uart0;
//!
//! let dp = Peripherals::take().unwrap();
//! let pins = Pins::init(dp.IO_BANK0, dp.PADS_BANK0);
//!
//! // Initialize UART0 with CTS/RTS flow control
//! let mut uart = Uart0WithFlowControl::init(
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
use crate::reset::Resettable;

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
    /// Timeout waiting for data / TX FIFO space
    Timeout,
}

/// Abstract UART bus trait for driver implementations.
///
/// This keeps drivers decoupled from concrete UART types while preserving
/// the HAL error type.
pub trait UartBus {
    /// Error type returned by the bus.
    type Error;

    /// Write bytes to the UART.
    fn write(&self, data: &[u8]) -> Result<(), Self::Error>;

    /// Read bytes from the UART, blocking until the buffer is full.
    fn read(&self, buffer: &mut [u8]) -> Result<(), Self::Error>;

    /// Read all currently available bytes without blocking.
    ///
    /// May return an error if a framing/parity/overrun error is detected
    /// on any byte.  For raw byte access that ignores error flags (useful
    /// for half-duplex echo draining), use [`read_raw`](UartBus::read_raw).
    fn read_available(&self, buffer: &mut [u8]) -> Result<usize, Self::Error>;

    /// Read available bytes **without error checking**.
    ///
    /// Unlike [`read_available`](UartBus::read_available), this method returns
    /// the raw data bytes and ignores framing / parity / overrun flags.
    /// Essential for single-wire half-duplex buses where the echo may carry
    /// spurious framing errors due to resistor-network level contention.
    fn read_raw(&self, buffer: &mut [u8]) -> usize;

    /// Block until all pending transfers are complete.
    fn flush(&self) -> Result<(), Self::Error>;
}

// ============================================================================
// Macro for generating UART implementations
// ============================================================================

macro_rules! impl_uart {
    ($UartName:ident, $uart_num:literal, $UartType:ty, $uart_mod:ident, $TxPin:ident, $RxPin:ident, $CtsPin:ident, $RtsPin:ident) => {
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
            #[doc = concat!("See [`", stringify!($TxPin), "`] for valid TX pins.")]
            ///
            #[doc = concat!("See [`", stringify!($RxPin), "`] for valid RX pins.")]
            pub struct $UartName<TX, RX>
            where
                TX: $TxPin,
                RX: $RxPin,
            {
                uart: $UartType,
                _tx: TX,
                _rx: RX,
            }

            impl<TX, RX> $UartName<TX, RX>
            where
                TX: $TxPin,
                RX: $RxPin,
            {
                /// UART peripheral number
                pub const NUM: u8 = $uart_num;

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
                #[doc = concat!("- TX must implement [`", stringify!($TxPin), "`]")]
                #[doc = concat!("- RX must implement [`", stringify!($RxPin), "`]")]
                ///
                /// # Example
                ///
                /// ```ignore
                #[doc = concat!("let uart = ", stringify!($UartName), "::init(")]
                #[doc = concat!("    dp.UART", $uart_num, ",")]
                ///     pins.pin::<0>(),      // TX
                ///     pins.pin::<1>(),      // RX
                ///     115200,       // Baud rate
                ///     150_000_000,  // System clock
                /// );
                /// ```
                pub fn init(uart: $UartType, mut tx: TX, mut rx: RX, baudrate: u32, peri_clk: u32) -> Self {
                    // Configure pins for UART function
                    tx.[<configure_uart $uart_num _tx>]();
                    rx.[<configure_uart $uart_num _rx>]();

                    // Reset peripheral to ensure clean register state
                    <$UartType as Resettable>::init();

                    let mut uart_dev = Self {
                        uart,
                        _tx: tx,
                        _rx: rx,
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

                /// Obtain a pin-erased UART handle without ownership.
                ///
                /// Returns `$UartName<ErasedPin, ErasedPin>` — a handle that can
                /// access all UART registers without carrying pin type parameters.
                /// This is useful for ISR context or global storage where the
                /// concrete pin types are inconvenient.
                ///
                /// # Safety
                ///
                /// The caller must ensure:
                /// 1. The UART peripheral has already been initialised (pins
                ///    configured, baud rate set, peripheral taken out of reset).
                /// 2. No data races: the returned handle must not be used
                ///    concurrently with the owning instance without proper
                ///    synchronisation (e.g. `Mutex`).
                ///
                /// # Example
                ///
                /// ```ignore
                #[doc = concat!("// In init code: create the full UART with pins")]
                #[doc = concat!("let uart = ", stringify!($UartName), "::init(")]
                #[doc = concat!("    dp.UART", $uart_num, ", tx_pin, rx_pin, 115200, sys_clk,")]
                /// );
                /// // Store the owning instance somewhere (e.g. Mutex)
                ///
                /// // Later, in ISR or another context:
                #[doc = concat!("let uart = unsafe { ", stringify!($UartName), "::<ErasedPin, ErasedPin>::steal() };")]
                /// uart.write(b"hello from ISR\r\n");
                /// ```
                #[inline]
                pub unsafe fn steal() -> crate::Steal<$UartName<crate::gpio::pins::ErasedPin, crate::gpio::pins::ErasedPin>> {
                    crate::Steal($UartName {
                        uart: unsafe { <$UartType>::steal() },
                        _tx: crate::gpio::pins::ErasedPin,
                        _rx: crate::gpio::pins::ErasedPin,
                    })
                }

                /// Get reference to the underlying register block.
                ///
                /// # Safety
                /// Direct register access can bypass the HAL's safety guarantees.
                /// The caller must ensure operations do not conflict with the HAL state.
                #[inline]
                pub unsafe fn regs(&self) -> &rp2350_pac::$uart_mod::RegisterBlock {
                    &self.uart
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
                ///
                /// Returns the actual baud rate achieved.
                fn configure_baudrate(&mut self, baudrate: u32, peri_clk: u32) -> u32 {
                    if baudrate == 0 {
                        return 0;
                    }
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

                    // Return actual baud rate
                    let divisor = 64 * ibrd as u64 + fbrd as u64;
                    if divisor == 0 { 0 } else { (4 * peri_clk as u64 / divisor) as u32 }
                }

                /// Set the UART baud rate
                ///
                /// Flushes any pending transmission, disables the UART, updates the
                /// baud rate registers, and re-enables the UART.
                ///
                /// # Arguments
                /// * `baudrate` - Desired baud rate
                /// * `peri_clk` - Peripheral clock frequency in Hz
                ///
                /// # Returns
                /// The actual baud rate achieved
                pub fn set_baudrate(&mut self, baudrate: u32, peri_clk: u32) -> u32 {
                    self.flush();
                    self.uart.uartcr.clr_uarten();
                    let actual = self.configure_baudrate(baudrate, peri_clk);
                    self.uart.uartcr.set_uarten();
                    actual
                }

                /// Set the data format
                ///
                /// # Arguments
                /// * `data_bits` - Number of data bits (5-8)
                /// * `stop_bits` - Number of stop bits (1 or 2)
                /// * `parity` - Parity configuration
                pub fn set_format(&mut self, data_bits: DataBits, stop_bits: StopBits, parity: Parity) {
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

                /// Enable the UART (set UARTEN bit)
                #[inline]
                pub fn enable(&self) {
                    self.uart.uartcr.set_uarten();
                }

                /// Disable the UART (clear UARTEN bit)
                #[inline]
                pub fn disable(&self) {
                    self.uart.uartcr.clr_uarten();
                }

                /// Check if the UART is enabled
                #[inline]
                pub fn is_enabled(&self) -> bool {
                    self.uart.uartcr.uarten()
                }

                /// Enable TX (set TXE bit)
                #[inline]
                pub fn enable_tx(&self) {
                    self.uart.uartcr.set_txe();
                }

                /// Disable TX (clear TXE bit)
                #[inline]
                pub fn disable_tx(&self) {
                    self.uart.uartcr.clr_txe();
                }

                /// Check if TX is enabled
                #[inline]
                pub fn tx_enabled(&self) -> bool {
                    self.uart.uartcr.txe()
                }

                /// Enable RX (set RXE bit)
                #[inline]
                pub fn enable_rx(&self) {
                    self.uart.uartcr.set_rxe();
                }

                /// Disable RX (clear RXE bit)
                #[inline]
                pub fn disable_rx(&self) {
                    self.uart.uartcr.clr_rxe();
                }

                /// Check if RX is enabled
                #[inline]
                pub fn rx_enabled(&self) -> bool {
                    self.uart.uartcr.rxe()
                }

                /// Enable or disable hardware flow control (CTS/RTS).
                ///
                /// Has no effect unless CTS/RTS pins are connected. The
                /// `WithFlowControl` variant calls this during initialization.
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

                /// Check if CTS is asserted (remote device ready to receive).
                #[inline]
                pub fn is_cts_asserted(&self) -> bool {
                    self.uart.uartfr.cts()
                }

                /// Set FIFO trigger levels
                ///
                /// # Arguments
                /// * `tx_level` - TX FIFO trigger level (0-4, corresponding to 1/8, 1/4, 1/2, 3/4, 7/8)
                /// * `rx_level` - RX FIFO trigger level (0-4, corresponding to 1/8, 1/4, 1/2, 3/4, 7/8)
                pub fn set_fifo_levels(&mut self, tx_level: u8, rx_level: u8) {
                    self.uart.uartifls.write_txiflsel(tx_level.min(4) as u32);
                    self.uart.uartifls.write_rxiflsel(rx_level.min(4) as u32);
                }

                // ================================================================
                // Data Transfer
                // ================================================================

                /// Write a single byte (blocking).
                ///
                /// Blocks until the TX FIFO has room for the byte.
                pub fn write_byte(&self, byte: u8) {
                    while self.uart.uartfr.txff() {
                        core::hint::spin_loop();
                    }
                    self.uart.uartdr.write_data(byte as u32);
                }

                /// Write multiple bytes (blocking).
                ///
                /// Blocks until every byte has been pushed to the TX FIFO.
                pub fn write(&self, data: &[u8]) {
                    // Fill the TX FIFO in bursts (up to 32 entries) to reduce
                    // status-register polling per byte.
                    let mut i = 0;
                    while i < data.len() {
                        while self.uart.uartfr.txff() {
                            core::hint::spin_loop();
                        }
                        // Write as many bytes as the FIFO currently accepts.
                        while i < data.len() && !self.uart.uartfr.txff() {
                            self.uart.uartdr.write_data(data[i] as u32);
                            i += 1;
                        }
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
                /// Returns the byte and any associated error.
                ///
                /// The UARTDR register is read **once** as a snapshot so that the
                /// returned data byte and its error flags belong to the *same*
                /// FIFO entry (reading DR pops the FIFO and clears the flags, so
                /// separate accessor reads would observe the *next* entry).
                pub fn read_byte_with_error(&self) -> Option<Result<u8, (u8, Error)>> {
                    if self.uart.uartfr.rxfe() {
                        return None;
                    }

                    let v = self.uart.uartdr.read_val();
                    let byte = v.data() as u8;

                    if v.oe() {
                        Some(Err((byte, Error::Overrun)))
                    } else if v.be() {
                        Some(Err((byte, Error::BreakError)))
                    } else if v.pe() {
                        Some(Err((byte, Error::ParityError)))
                    } else if v.fe() {
                        Some(Err((byte, Error::FramingError)))
                    } else {
                        Some(Ok(byte))
                    }
                }

                /// Read a single byte (blocking).
                ///
                /// Blocks until a byte arrives in the RX FIFO.
                pub fn read_byte_blocking(&self) -> u8 {
                    while self.uart.uartfr.rxfe() {
                        core::hint::spin_loop();
                    }
                    self.uart.uartdr.data() as u8
                }

                /// Read multiple bytes (blocking)
                ///
                /// Returns [`Error::Timeout`] if no data arrives within
                /// [`crate::spin::SPIN_TIMEOUT`] iterations, or a frame/parity/
                /// overrun error reported by the hardware.
                pub fn read(&self, buffer: &mut [u8]) -> Result<(), Error> {
                    for slot in buffer.iter_mut() {
                        if !crate::spin::try_spin_wait(|| !self.uart.uartfr.rxfe()) {
                            return Err(Error::Timeout);
                        }
                        match self.read_byte_with_error() {
                            Some(Ok(byte)) => *slot = byte,
                            Some(Err((_, err))) => return Err(err),
                            None => unreachable!(),
                        }
                    }
                    Ok(())
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

                /// Wait for transmission to complete.
                ///
                /// Blocks until the TX FIFO is empty and the shift register has
                /// finished transmitting.
                pub fn flush(&self) {
                    while !self.uart.uartfr.txfe() {
                        core::hint::spin_loop();
                    }
                    while self.uart.uartfr.busy() {
                        core::hint::spin_loop();
                    }
                }

                /// Clear all receive error flags.
                ///
                /// The UART error flags (OE/BE/PE/FE) are write-1-to-clear:
                /// writing 1 to the bit clears the corresponding error.
                pub fn clear_errors(&self) {
                    self.uart.uartrsr.set_oe();
                    self.uart.uartrsr.set_be();
                    self.uart.uartrsr.set_pe();
                    self.uart.uartrsr.set_fe();
                }

                /// Get the current baud rate by reading hardware registers
                ///
                /// # Arguments
                /// * `peri_clk` - Peripheral clock frequency in Hz
                #[inline]
                pub fn baudrate(&self, peri_clk: u32) -> u32 {
                    let ibrd = self.uart.uartibrd.baud_divint() as u64;
                    let fbrd = self.uart.uartfbrd.baud_divfrac() as u64;
                    let divisor = 64 * ibrd + fbrd;
                    if divisor == 0 {
                        return 0;
                    }
                    (4 * peri_clk as u64 / divisor) as u32
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
                TX: $TxPin,
                RX: $RxPin,
            {
                fn write_str(&mut self, s: &str) -> core::fmt::Result {
                    self.write(s.as_bytes());
                    Ok(())
                }
            }

            impl<TX, RX> UartBus for $UartName<TX, RX>
            where
                TX: $TxPin,
                RX: $RxPin,
            {
                type Error = Error;

                #[inline]
                fn write(&self, data: &[u8]) -> Result<(), Self::Error> {
                    self.write(data);
                    Ok(())
                }

                #[inline]
                fn read(&self, buffer: &mut [u8]) -> Result<(), Self::Error> {
                    self.read(buffer)
                }

                #[inline]
                fn read_available(&self, buffer: &mut [u8]) -> Result<usize, Self::Error> {
                    let mut count = 0;

                    for slot in buffer.iter_mut() {
                        match self.read_byte_with_error() {
                            Some(Ok(byte)) => {
                                *slot = byte;
                                count += 1;
                            }
                            Some(Err((_, err))) => return Err(err),
                            None => break,
                        }
                    }

                    Ok(count)
                }

                #[inline]
                fn read_raw(&self, buffer: &mut [u8]) -> usize {
                    self.read_raw(buffer)
                }

                #[inline]
                fn flush(&self) -> Result<(), Self::Error> {
                    self.flush();
                    Ok(())
                }
            }

            // ================================================================
            // UART with hardware flow control
            // ================================================================

            /// UART peripheral wrapper with hardware flow control (CTS/RTS).
            ///
            /// Wraps a plain [`$UartName`] and adds CTS/RTS pin ownership. The full
            /// base UART API is available through `Deref`; this wrapper only adds
            /// flow-control setup.
            #[doc = concat!("# UART", $uart_num, " with Flow Control")]
            pub struct [<$UartName WithFlowControl>]<TX, RX, CTS, RTS>
            where
                TX: $TxPin,
                RX: $RxPin,
                CTS: $CtsPin,
                RTS: $RtsPin,
            {
                inner: $UartName<TX, RX>,
                _cts: CTS,
                _rts: RTS,
            }

            impl<TX, RX, CTS, RTS> [<$UartName WithFlowControl>]<TX, RX, CTS, RTS>
            where
                TX: $TxPin,
                RX: $RxPin,
                CTS: $CtsPin,
                RTS: $RtsPin,
            {
                /// UART peripheral number
                pub const NUM: u8 = $uart_num;

                /// Obtain a pin-erased UART handle without ownership (flow control variant).
                ///
                /// # Safety
                ///
                /// Same requirements as [`$UartName::steal()`].
                #[inline]
                pub unsafe fn steal() -> crate::Steal<[<$UartName WithFlowControl>]<
                    crate::gpio::pins::ErasedPin,
                    crate::gpio::pins::ErasedPin,
                    crate::gpio::pins::ErasedPin,
                    crate::gpio::pins::ErasedPin,
                >> {
                    crate::Steal([<$UartName WithFlowControl>] {
                        inner: $UartName {
                            uart: unsafe { <$UartType>::steal() },
                            _tx: crate::gpio::pins::ErasedPin,
                            _rx: crate::gpio::pins::ErasedPin,
                        },
                        _cts: crate::gpio::pins::ErasedPin,
                        _rts: crate::gpio::pins::ErasedPin,
                    })
                }

                /// Get reference to the underlying register block.
                ///
                /// # Safety
                /// Direct register access can bypass the HAL's safety guarantees.
                /// The caller must ensure operations do not conflict with the HAL state.
                #[inline]
                pub unsafe fn regs(&self) -> &rp2350_pac::$uart_mod::RegisterBlock {
                    &self.inner.uart
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
                pub fn init(
                    uart: $UartType,
                    tx: TX,
                    rx: RX,
                    mut cts: CTS,
                    mut rts: RTS,
                    baudrate: u32,
                    peri_clk: u32,
                ) -> Self {
                    let inner = $UartName::init(uart, tx, rx, baudrate, peri_clk);

                    // Configure CTS/RTS pins and enable flow control.
                    cts.[<configure_uart $uart_num _cts>]();
                    rts.[<configure_uart $uart_num _rts>]();
                    inner.set_flow_control(true, true);

                    Self {
                        inner,
                        _cts: cts,
                        _rts: rts,
                    }
                }

                /// Release the UART peripheral and all pins.
                #[inline]
                pub fn release(self) -> ($UartType, TX, RX, CTS, RTS) {
                    let (uart, tx, rx) = self.inner.release();
                    (uart, tx, rx, self._cts, self._rts)
                }
            }

            impl<TX, RX, CTS, RTS> core::ops::Deref for [<$UartName WithFlowControl>]<TX, RX, CTS, RTS>
            where
                TX: $TxPin,
                RX: $RxPin,
                CTS: $CtsPin,
                RTS: $RtsPin,
            {
                type Target = $UartName<TX, RX>;

                fn deref(&self) -> &Self::Target {
                    &self.inner
                }
            }

            impl<TX, RX, CTS, RTS> core::ops::DerefMut for [<$UartName WithFlowControl>]<TX, RX, CTS, RTS>
            where
                TX: $TxPin,
                RX: $RxPin,
                CTS: $CtsPin,
                RTS: $RtsPin,
            {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.inner
                }
            }

            impl<TX, RX, CTS, RTS> core::fmt::Write for [<$UartName WithFlowControl>]<TX, RX, CTS, RTS>
            where
                TX: $TxPin,
                RX: $RxPin,
                CTS: $CtsPin,
                RTS: $RtsPin,
            {
                fn write_str(&mut self, s: &str) -> core::fmt::Result {
                    self.inner.write(s.as_bytes());
                    Ok(())
                }
            }

            impl<TX, RX, CTS, RTS> UartBus for [<$UartName WithFlowControl>]<TX, RX, CTS, RTS>
            where
                TX: $TxPin,
                RX: $RxPin,
                CTS: $CtsPin,
                RTS: $RtsPin,
            {
                type Error = Error;

                #[inline]
                fn write(&self, data: &[u8]) -> Result<(), Self::Error> {
                    UartBus::write(&self.inner, data)
                }

                #[inline]
                fn read(&self, buffer: &mut [u8]) -> Result<(), Self::Error> {
                    UartBus::read(&self.inner, buffer)
                }

                #[inline]
                fn read_available(&self, buffer: &mut [u8]) -> Result<usize, Self::Error> {
                    UartBus::read_available(&self.inner, buffer)
                }

                #[inline]
                fn read_raw(&self, buffer: &mut [u8]) -> usize {
                    UartBus::read_raw(&self.inner, buffer)
                }

                #[inline]
                fn flush(&self) -> Result<(), Self::Error> {
                    UartBus::flush(&self.inner)
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
    rp2350_pac::UART0,
    uart0,
    Uart0TxPin,
    Uart0RxPin,
    Uart0CtsPin,
    Uart0RtsPin
);
impl_uart!(
    Uart1,
    1,
    rp2350_pac::UART1,
    uart1,
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
