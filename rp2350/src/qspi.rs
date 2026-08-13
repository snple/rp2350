//! QSPI (Quad SPI) HAL for RP2350
//!
//! This module provides a safe interface for the RP2350 QMI (QSPI Memory Interface) peripheral.
//! The QMI provides both a memory-mapped interface (XIP) and a direct serial mode for
//! programming, configuration, and control of external SPI/DSPI/QSPI flash or PSRAM devices.
//!
//! # Architecture
//!
//! The QMI peripheral has two operating modes:
//!
//! - **Memory-mapped mode (XIP)**: Automatically translates AHB bus accesses to SPI flash
//!   read/write commands. Supports two chip selects (CS0n, CS1n) with independent configurations.
//! - **Direct mode**: Software-controlled raw SPI transfers via TX/RX FIFOs. Used for
//!   flash programming, erasing, status register access, and other non-memory-mapped operations.
//!
//! # Features
//!
//! - Single / Dual / Quad SPI bus widths
//! - Two independent chip selects (CS0n, CS1n) for flash/PSRAM
//! - Configurable clock divider per memory window
//! - Configurable read/write transfer formats with independent phase widths
//! - Command prefix, address, suffix, dummy, and data phases
//! - Direct mode for raw SPI command/response
//! - Double Transfer Rate (DTR) support
//! - DMA support for direct mode
//! - Address translation for XIP
//!
//! # QSPI Pin Mapping
//!
//! The QSPI interface uses dedicated pins (not from GPIO bank0):
//! - QSPI_SCLK: Serial clock
//! - QSPI_CSn0: Chip select 0 (active low)
//! - QSPI_CSn1: Chip select 1 (active low) — if a second device is connected
//! - QSPI_SD0: Serial data 0 (MOSI in single SPI mode)
//! - QSPI_SD1: Serial data 1 (MISO in single SPI mode)
//! - QSPI_SD2: Serial data 2 (active in quad mode)
//! - QSPI_SD3: Serial data 3 (active in quad mode)
//!
//! # Example (Direct mode — read flash JEDEC ID)
//!
//! ```ignore
//! use rp2350::qspi::Qspi;
//! use rp2350::Peripherals;
//!
//! let dp = Peripherals::take().unwrap();
//! let mut qspi = Qspi::init(dp.QMI);
//!
//! // Enter direct mode and select CS0
//! qspi.enable_direct_mode();
//! qspi.assert_cs0();
//!
//! // Send JEDEC ID command (0x9F) and read 3 bytes
//! qspi.direct_write_blocking(0x9F, false);
//! let mfr = qspi.direct_read_blocking();  // Manufacturer ID
//! let id1 = qspi.direct_read_blocking();  // Device ID (MSB)
//! let id2 = qspi.direct_read_blocking();  // Device ID (LSB)
//!
//! qspi.deassert_cs0();
//! qspi.disable_direct_mode();
//! ```
//!
//! # Example (Configure XIP quad read for flash on CS0)
//!
//! ```ignore
//! use rp2350::qspi::{Qspi, BusWidth};
//!
//! let dp = Peripherals::take().unwrap();
//! let mut qspi = Qspi::init(dp.QMI);
//!
//! // Configure memory window 0 for quad read (0x6B) at higher speed
//! qspi.set_clkdiv(Window::M0, 2);
//! qspi.configure_read_format(Window::M0, ReadFormat {
//!     prefix_width: BusWidth::Single,
//!     addr_width: BusWidth::Single,
//!     suffix_width: BusWidth::Single,
//!     dummy_width: BusWidth::Single,
//!     data_width: BusWidth::Quad,
//!     prefix_len: true,      // 8-bit command prefix
//!     suffix_len: 0,         // No suffix
//!     dummy_len: 2,          // 8 dummy bits
//!     dtr: false,
//! });
//! qspi.set_read_command(Window::M0, 0x6B, 0x00);
//!
//! // Now XIP flash reads from 0x10000000 will use quad output
//! ```

use crate::Steal;

use rp2350_pac::QMI;

use rp2350_pac::qmi::DirectTxVal;

// ============================================================================
// Types
// ============================================================================

/// QSPI bus width for transfer phases
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u32)]
pub enum BusWidth {
    /// Single SPI (1-bit): SD0 for output, SD1 for input
    Single = 0,
    /// Dual SPI (2-bit): SD0 and SD1
    Dual = 1,
    /// Quad SPI (4-bit): SD0, SD1, SD2, SD3
    Quad = 2,
}

/// Memory address window (chip select)
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Window {
    /// Memory window 0 (CS0n) — addresses 0x1000_0000..0x10FF_FFFF
    M0,
    /// Memory window 1 (CS1n) — addresses 0x1100_0000..0x11FF_FFFF
    M1,
}

/// Page break configuration for memory-mapped mode
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u32)]
pub enum PageBreak {
    /// No page boundary enforced
    None = 0,
    /// Break at 256-byte page boundaries
    Bytes256 = 1,
    /// Break at 1024-byte quad-page boundaries
    Bytes1024 = 2,
    /// Break at 4096-byte sector boundaries
    Bytes4096 = 3,
}

/// Read transfer format configuration
///
/// Configures the bus width of each phase and the length/presence of
/// command prefix, suffix, and dummy phases for memory-mapped reads.
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ReadFormat {
    /// Width for command prefix phase
    pub prefix_width: BusWidth,
    /// Width for address phase (24-bit address)
    pub addr_width: BusWidth,
    /// Width for command suffix phase
    pub suffix_width: BusWidth,
    /// Width for dummy/turnaround phase
    pub dummy_width: BusWidth,
    /// Width for data phase
    pub data_width: BusWidth,
    /// Enable 8-bit command prefix (true = 8 bits, false = none)
    pub prefix_len: bool,
    /// Suffix length: 0 = none, 2 = 8 bits
    pub suffix_len: u32,
    /// Dummy phase length in units of 4 bits (0-7): 0=none, 1=4b, 2=8b, …, 7=28b
    pub dummy_len: u32,
    /// Enable double transfer rate
    pub dtr: bool,
}

/// Write transfer format configuration
///
/// Configures the bus width of each phase and the length/presence of
/// command prefix, suffix, and dummy phases for memory-mapped writes.
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct WriteFormat {
    /// Width for command prefix phase
    pub prefix_width: BusWidth,
    /// Width for address phase (24-bit address)
    pub addr_width: BusWidth,
    /// Width for command suffix phase
    pub suffix_width: BusWidth,
    /// Width for dummy/turnaround phase
    pub dummy_width: BusWidth,
    /// Width for data phase
    pub data_width: BusWidth,
    /// Enable 8-bit command prefix (true = 8 bits, false = none)
    pub prefix_len: bool,
    /// Suffix length: 0 = none, 2 = 8 bits
    pub suffix_len: u32,
    /// Dummy phase length in units of 4 bits (0-7)
    pub dummy_len: u32,
    /// Enable double transfer rate
    pub dtr: bool,
}

/// Timing configuration for a memory window
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct TimingConfig {
    /// Clock divisor (1-255, 0 encodes 256)
    pub clkdiv: u8,
    /// RX sample delay in half system clock cycles (0-7)
    pub rxdelay: u8,
    /// Minimum deselect time in system clock cycles (0-31)
    pub min_deselect: u8,
    /// Maximum chip select assertion in 64-cycle units (0=unlimited, 1-63)
    pub max_select: u8,
    /// CS hold time: additional cycles after last SCK falling edge (0-3)
    pub select_hold: u8,
    /// CS setup: add one extra cycle before first SCK rising edge
    pub select_setup: bool,
    /// Page break mode
    pub pagebreak: PageBreak,
    /// Cooldown period (0-3, in 64-cycle units)
    pub cooldown: u8,
}

impl Default for TimingConfig {
    /// Reset default: clkdiv=4, rxdelay=0, cooldown=1
    fn default() -> Self {
        Self {
            clkdiv: 4,
            rxdelay: 0,
            min_deselect: 0,
            max_select: 0,
            select_hold: 0,
            select_setup: false,
            pagebreak: PageBreak::None,
            cooldown: 1,
        }
    }
}

/// QSPI error types
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// TX FIFO full when trying to write
    TxFull,
    /// RX FIFO empty when trying to read
    RxEmpty,
    /// Busy timeout
    Busy,
}

// ============================================================================
// QSPI peripheral wrapper
// ============================================================================

/// QMI (QSPI Memory Interface) peripheral wrapper.
///
/// Provides safe access to the RP2350's QSPI controller for both
/// memory-mapped (XIP) and direct serial mode operations.
///
/// The QSPI controller uses dedicated QSPI pins (SCLK, CSn, SD0-SD3) that
/// are separate from the GPIO bank0 pins, so no pin ownership management
/// is needed.
pub struct Qspi {
    qmi: QMI,
}

impl Qspi {
    /// Initialize the QSPI driver.
    ///
    /// Takes ownership of the QMI peripheral. The QSPI pins are dedicated
    /// and don't need to be configured separately.
    ///
    /// # Arguments
    ///
    /// * `qmi` - The QMI peripheral from PAC
    ///
    /// # Example
    ///
    /// ```ignore
    /// let dp = Peripherals::take().unwrap();
    /// let qspi = Qspi::init(dp.QMI);
    /// ```
    pub fn init(qmi: QMI) -> Self {
        Self { qmi }
    }

    /// Obtain a Qspi handle without ownership checks.
    ///
    /// # Safety
    ///
    /// The caller must ensure the peripheral has been initialised
    /// and that no data races occur.
    #[inline]
    pub unsafe fn steal() -> Steal<Self> {
        Steal(Self::init(unsafe { QMI::steal() }))
    }

    /// Release the QMI peripheral.
    #[inline]
    pub fn release(self) -> QMI {
        self.qmi
    }

    /// Get reference to the underlying register block.
    ///
    /// # Safety
    /// Direct register access can bypass the HAL's safety guarantees.
    #[inline]
    pub unsafe fn regs(&self) -> &rp2350_pac::qmi::RegisterBlock {
        &self.qmi
    }

    // ========================================================================
    // Direct Mode — Control
    // ========================================================================

    /// Enable direct serial mode.
    ///
    /// In direct mode, software controls chip select lines and performs
    /// raw SPI transfers via the TX/RX FIFOs. Memory-mapped (XIP) accesses
    /// will generate bus errors while direct mode is enabled.
    ///
    /// # Important
    ///
    /// Before asserting chip select, poll [`is_busy()`](Self::is_busy) to
    /// ensure no memory-mapped transfer is still in progress.
    #[inline]
    pub fn enable_direct_mode(&self) {
        self.qmi.direct_csr.set_en();
    }

    /// Disable direct serial mode.
    ///
    /// Returns to memory-mapped (XIP) mode. Ensure all direct transfers
    /// are complete and chip selects are deasserted before calling this.
    #[inline]
    pub fn disable_direct_mode(&self) {
        self.qmi.direct_csr.clr_en();
    }

    /// Check if direct mode is enabled.
    #[inline]
    pub fn is_direct_mode(&self) -> bool {
        self.qmi.direct_csr.en()
    }

    /// Check if the QMI is busy with a transfer.
    #[inline]
    pub fn is_busy(&self) -> bool {
        self.qmi.direct_csr.busy()
    }

    /// Wait until the QMI is no longer busy.
    ///
    /// # Panics
    ///
    /// Panics if the QMI stays busy within [`crate::spin::SPIN_TIMEOUT`]
    /// iterations.
    #[inline]
    pub fn wait_idle(&self) {
        crate::spin::spin_wait("QMI idle", || !self.qmi.direct_csr.busy());
    }

    // ========================================================================
    // Direct Mode — Chip Select
    // ========================================================================

    /// Assert CS0n (drive low) manually.
    ///
    /// This applies even when direct mode is disabled.
    #[inline]
    pub fn assert_cs0(&self) {
        self.qmi.direct_csr.set_assert_cs0n();
    }

    /// Deassert CS0n (release high) manually.
    #[inline]
    pub fn deassert_cs0(&self) {
        self.qmi.direct_csr.clr_assert_cs0n();
    }

    /// Assert CS1n (drive low) manually.
    #[inline]
    pub fn assert_cs1(&self) {
        self.qmi.direct_csr.set_assert_cs1n();
    }

    /// Deassert CS1n (release high) manually.
    #[inline]
    pub fn deassert_cs1(&self) {
        self.qmi.direct_csr.clr_assert_cs1n();
    }

    /// Enable automatic CS0n assertion whenever BUSY is set.
    #[inline]
    pub fn enable_auto_cs0(&self) {
        self.qmi.direct_csr.set_auto_cs0n();
    }

    /// Disable automatic CS0n assertion.
    #[inline]
    pub fn disable_auto_cs0(&self) {
        self.qmi.direct_csr.clr_auto_cs0n();
    }

    /// Enable automatic CS1n assertion whenever BUSY is set.
    #[inline]
    pub fn enable_auto_cs1(&self) {
        self.qmi.direct_csr.set_auto_cs1n();
    }

    /// Disable automatic CS1n assertion.
    #[inline]
    pub fn disable_auto_cs1(&self) {
        self.qmi.direct_csr.clr_auto_cs1n();
    }

    // ========================================================================
    // Direct Mode — Clock
    // ========================================================================

    /// Set the direct mode clock divisor.
    ///
    /// Divisors 1..255 are encoded directly.
    /// A divisor of 256 is encoded as 0.
    #[inline]
    pub fn set_direct_clkdiv(&self, div: u8) {
        self.qmi.direct_csr.write_clkdiv(div as u32);
    }

    /// Get the direct mode clock divisor.
    #[inline]
    pub fn direct_clkdiv(&self) -> u8 {
        self.qmi.direct_csr.clkdiv() as u8
    }

    /// Set the direct mode RX sample delay (0-3 half-cycles).
    #[inline]
    pub fn set_direct_rxdelay(&self, delay: u8) {
        self.qmi.direct_csr.write_rxdelay((delay & 0x3) as u32);
    }

    // ========================================================================
    // Direct Mode — FIFO Status
    // ========================================================================

    /// Check if the direct TX FIFO is full.
    #[inline]
    pub fn is_direct_tx_full(&self) -> bool {
        self.qmi.direct_csr.txfull()
    }

    /// Check if the direct TX FIFO is empty.
    #[inline]
    pub fn is_direct_tx_empty(&self) -> bool {
        self.qmi.direct_csr.txempty()
    }

    /// Check if the direct RX FIFO is full.
    #[inline]
    pub fn is_direct_rx_full(&self) -> bool {
        self.qmi.direct_csr.rxfull()
    }

    /// Check if the direct RX FIFO is empty.
    #[inline]
    pub fn is_direct_rx_empty(&self) -> bool {
        self.qmi.direct_csr.rxempty()
    }

    /// Get the current TX FIFO level (0-4).
    #[inline]
    pub fn direct_tx_level(&self) -> u32 {
        self.qmi.direct_csr.txlevel()
    }

    /// Get the current RX FIFO level (0-4).
    #[inline]
    pub fn direct_rx_level(&self) -> u32 {
        self.qmi.direct_csr.rxlevel()
    }

    // ========================================================================
    // Direct Mode — Data Transfer (raw register writes)
    // ========================================================================

    /// Push a raw value to the direct TX FIFO.
    ///
    /// The value encodes data, bus width, output enable, and NOPUSH flags.
    /// Use the `DirectTxVal` builder for convenient construction:
    ///
    /// ```ignore
    /// use rp2350_pac::qmi::DirectTxVal;
    ///
    /// qspi.direct_tx_raw(
    ///     DirectTxVal::reset_val()
    ///         .with_data(0x9F)
    ///         .with_iwidth(0)   // Single
    ///         .with_oe(true)    // Output enable
    ///         .with_nopush(true) // Don't push RX for this byte
    /// );
    /// ```
    #[inline]
    pub fn direct_tx_raw(&self, val: DirectTxVal) {
        self.qmi.direct_tx.write_val(val);
    }

    /// Pop a raw 16-bit word from the direct RX FIFO.
    #[inline]
    pub fn direct_rx_raw(&self) -> u16 {
        self.qmi.direct_rx.direct_rx() as u16
    }

    // ========================================================================
    // Direct Mode — Convenient Transfer Methods
    // ========================================================================

    /// Write a single byte in direct mode (single-width SPI, blocking).
    ///
    /// Pushes one byte to TX FIFO with output enabled. Waits if TX FIFO is full.
    /// The `nopush` flag controls whether a corresponding RX FIFO entry is generated.
    ///
    /// # Arguments
    ///
    /// * `data` - Byte to transmit
    /// * `nopush` - If true, inhibit RX FIFO push (useful for command bytes)
    pub fn direct_write_blocking(&self, data: u8, nopush: bool) {
        // Wait for TX FIFO space
        crate::spin::spin_wait("QMI TX FIFO space", || !self.qmi.direct_csr.txfull());
        let val = DirectTxVal::reset_val()
            .with_data(data as u32)
            .with_iwidth(BusWidth::Single as u32)
            .with_oe(true)
            .with_nopush(nopush);
        self.qmi.direct_tx.write_val(val);
    }

    /// Write a single byte in direct mode with specified bus width (blocking).
    ///
    /// # Arguments
    ///
    /// * `data` - Byte to transmit
    /// * `width` - Bus width (Single/Dual/Quad)
    /// * `nopush` - If true, inhibit RX FIFO push
    pub fn direct_write_width_blocking(&self, data: u8, width: BusWidth, nopush: bool) {
        crate::spin::spin_wait("QMI TX FIFO space", || !self.qmi.direct_csr.txfull());
        let val = DirectTxVal::reset_val()
            .with_data(data as u32)
            .with_iwidth(width as u32)
            .with_oe(true)
            .with_nopush(nopush);
        self.qmi.direct_tx.write_val(val);
    }

    /// Write a 16-bit word in direct mode (blocking).
    ///
    /// # Arguments
    ///
    /// * `data` - 16-bit word to transmit
    /// * `width` - Bus width
    /// * `nopush` - If true, inhibit RX FIFO push
    pub fn direct_write16_blocking(&self, data: u16, width: BusWidth, nopush: bool) {
        crate::spin::spin_wait("QMI TX FIFO space", || !self.qmi.direct_csr.txfull());
        let val = DirectTxVal::reset_val()
            .with_data(data as u32)
            .with_iwidth(width as u32)
            .with_oe(true)
            .with_dwidth(true) // 16-bit mode
            .with_nopush(nopush);
        self.qmi.direct_tx.write_val(val);
    }

    /// Read a single byte from direct mode RX FIFO (blocking).
    ///
    /// Sends a dummy byte (0x00) with output disabled to clock in data,
    /// then reads the received byte.
    pub fn direct_read_blocking(&self) -> u8 {
        // Send dummy byte to generate clock (OE=false for read)
        crate::spin::spin_wait("QMI TX FIFO space", || !self.qmi.direct_csr.txfull());
        let val = DirectTxVal::reset_val()
            .with_data(0)
            .with_iwidth(BusWidth::Single as u32)
            .with_oe(false)
            .with_nopush(false);
        self.qmi.direct_tx.write_val(val);

        // Wait for RX data
        crate::spin::spin_wait("QMI RX data", || !self.qmi.direct_csr.rxempty());
        self.qmi.direct_rx.direct_rx() as u8
    }

    /// Read a single byte with specified bus width from direct mode (blocking).
    ///
    /// # Arguments
    ///
    /// * `width` - Bus width for the read transfer
    pub fn direct_read_width_blocking(&self, width: BusWidth) -> u8 {
        crate::spin::spin_wait("QMI TX FIFO space", || !self.qmi.direct_csr.txfull());
        let val = DirectTxVal::reset_val()
            .with_data(0)
            .with_iwidth(width as u32)
            .with_oe(false)
            .with_nopush(false);
        self.qmi.direct_tx.write_val(val);

        crate::spin::spin_wait("QMI RX data", || !self.qmi.direct_csr.rxempty());
        self.qmi.direct_rx.direct_rx() as u8
    }

    /// Write multiple bytes in direct mode (single-width, blocking).
    ///
    /// All bytes are sent with output enabled and NOPUSH set (RX FIFO entries suppressed).
    pub fn direct_write(&self, data: &[u8]) {
        for &byte in data {
            self.direct_write_blocking(byte, true);
        }
        self.wait_idle();
    }

    /// Read multiple bytes in direct mode (single-width, blocking).
    ///
    /// Sends dummy bytes and collects received data from RX FIFO.
    pub fn direct_read(&self, buf: &mut [u8]) {
        for slot in buf.iter_mut() {
            *slot = self.direct_read_blocking();
        }
    }

    /// Perform a full-duplex direct mode transfer (single-width, blocking).
    ///
    /// Writes data from `tx` and simultaneously reads into `rx`.
    /// Both buffers should be the same length; the shorter length is used.
    pub fn direct_transfer(&self, rx: &mut [u8], tx: &[u8]) {
        let len = rx.len().min(tx.len());
        for i in 0..len {
            // Write with OE enabled, don't inhibit RX push
            crate::spin::spin_wait("QMI TX FIFO space", || !self.qmi.direct_csr.txfull());
            let val = DirectTxVal::reset_val()
                .with_data(tx[i] as u32)
                .with_iwidth(BusWidth::Single as u32)
                .with_oe(true)
                .with_nopush(false);
            self.qmi.direct_tx.write_val(val);

            // Read response
            crate::spin::spin_wait("QMI RX data", || !self.qmi.direct_csr.rxempty());
            rx[i] = self.qmi.direct_rx.direct_rx() as u8;
        }
    }

    /// Drain (discard) all data from the direct RX FIFO.
    #[inline]
    pub fn drain_direct_rx(&self) {
        while !self.qmi.direct_csr.rxempty() {
            let _ = self.qmi.direct_rx.direct_rx();
        }
    }

    // ========================================================================
    // Direct Mode — Common Flash Commands
    // ========================================================================

    /// Send a simple command with no response data (e.g., Write Enable, Chip Erase).
    ///
    /// Asserts CS0, sends the command byte, waits for completion, deasserts CS0.
    pub fn flash_command_cs0(&self, cmd: u8) {
        self.assert_cs0();
        self.direct_write_blocking(cmd, true);
        self.wait_idle();
        self.deassert_cs0();
    }

    /// Send a simple command on CS1.
    pub fn flash_command_cs1(&self, cmd: u8) {
        self.assert_cs1();
        self.direct_write_blocking(cmd, true);
        self.wait_idle();
        self.deassert_cs1();
    }

    /// Read flash status register (command 0x05) on CS0.
    pub fn flash_read_status_cs0(&self) -> u8 {
        self.assert_cs0();
        self.direct_write_blocking(0x05, true);
        let status = self.direct_read_blocking();
        self.wait_idle();
        self.deassert_cs0();
        status
    }

    /// Read flash status register (command 0x05) on CS1.
    pub fn flash_read_status_cs1(&self) -> u8 {
        self.assert_cs1();
        self.direct_write_blocking(0x05, true);
        let status = self.direct_read_blocking();
        self.wait_idle();
        self.deassert_cs1();
        status
    }

    /// Wait until flash WIP (Write-In-Progress) bit clears on CS0.
    ///
    /// Reads status register 1 (command 0x05) repeatedly until bit 0 is clear.
    pub fn flash_wait_ready_cs0(&self) {
        loop {
            let st = self.flash_read_status_cs0();
            if st & 0x01 == 0 {
                break;
            }
            core::hint::spin_loop();
        }
    }

    /// Wait until flash WIP bit clears on CS1.
    pub fn flash_wait_ready_cs1(&self) {
        loop {
            let st = self.flash_read_status_cs1();
            if st & 0x01 == 0 {
                break;
            }
            core::hint::spin_loop();
        }
    }

    /// Read JEDEC ID (command 0x9F) on CS0. Returns (manufacturer, mem_type, capacity).
    pub fn flash_read_jedec_id_cs0(&self) -> (u8, u8, u8) {
        self.assert_cs0();
        self.direct_write_blocking(0x9F, true);
        let mfr = self.direct_read_blocking();
        let typ = self.direct_read_blocking();
        let cap = self.direct_read_blocking();
        self.wait_idle();
        self.deassert_cs0();
        (mfr, typ, cap)
    }

    // ========================================================================
    // Memory-mapped Mode — Timing Configuration
    // ========================================================================

    /// Set the clock divisor for a memory window.
    ///
    /// Divisors 1..255 are encoded directly. 0 encodes 256.
    /// Can be changed on-the-fly even during active memory access.
    #[inline]
    pub fn set_clkdiv(&self, window: Window, div: u8) {
        match window {
            Window::M0 => self.qmi.m0_timing.write_clkdiv(div as u32),
            Window::M1 => self.qmi.m1_timing.write_clkdiv(div as u32),
        }
    }

    /// Get the clock divisor for a memory window.
    #[inline]
    pub fn clkdiv(&self, window: Window) -> u8 {
        match window {
            Window::M0 => self.qmi.m0_timing.clkdiv() as u8,
            Window::M1 => self.qmi.m1_timing.clkdiv() as u8,
        }
    }

    /// Set the RX sample delay for a memory window.
    ///
    /// In units of one half system clock cycle (0-7).
    #[inline]
    pub fn set_rxdelay(&self, window: Window, delay: u8) {
        match window {
            Window::M0 => self.qmi.m0_timing.write_rxdelay((delay & 0x7) as u32),
            Window::M1 => self.qmi.m1_timing.write_rxdelay((delay & 0x7) as u32),
        }
    }

    /// Set the minimum CS deselect time in system clock cycles (0-31).
    #[inline]
    pub fn set_min_deselect(&self, window: Window, cycles: u8) {
        match window {
            Window::M0 => self
                .qmi
                .m0_timing
                .write_min_deselect((cycles & 0x1F) as u32),
            Window::M1 => self
                .qmi
                .m1_timing
                .write_min_deselect((cycles & 0x1F) as u32),
        }
    }

    /// Set the maximum CS assertion duration in 64-cycle units (0=unlimited).
    #[inline]
    pub fn set_max_select(&self, window: Window, val: u8) {
        match window {
            Window::M0 => self.qmi.m0_timing.write_max_select((val & 0x3F) as u32),
            Window::M1 => self.qmi.m1_timing.write_max_select((val & 0x3F) as u32),
        }
    }

    /// Set CS hold time (0-3 additional system clock cycles).
    #[inline]
    pub fn set_select_hold(&self, window: Window, hold: u8) {
        match window {
            Window::M0 => self.qmi.m0_timing.write_select_hold((hold & 0x3) as u32),
            Window::M1 => self.qmi.m1_timing.write_select_hold((hold & 0x3) as u32),
        }
    }

    /// Set CS setup time (add one extra cycle before first SCK).
    #[inline]
    pub fn set_select_setup(&self, window: Window, enable: bool) {
        match window {
            Window::M0 => {
                if enable {
                    self.qmi.m0_timing.set_select_setup();
                } else {
                    self.qmi.m0_timing.clr_select_setup();
                }
            }
            Window::M1 => {
                if enable {
                    self.qmi.m1_timing.set_select_setup();
                } else {
                    self.qmi.m1_timing.clr_select_setup();
                }
            }
        }
    }

    /// Set page break mode for a memory window.
    #[inline]
    pub fn set_pagebreak(&self, window: Window, pb: PageBreak) {
        match window {
            Window::M0 => self.qmi.m0_timing.write_pagebreak(pb as u32),
            Window::M1 => self.qmi.m1_timing.write_pagebreak(pb as u32),
        }
    }

    /// Set cooldown period (0-3, in 64 system clock cycle units).
    #[inline]
    pub fn set_cooldown(&self, window: Window, cooldown: u8) {
        match window {
            Window::M0 => self.qmi.m0_timing.write_cooldown((cooldown & 0x3) as u32),
            Window::M1 => self.qmi.m1_timing.write_cooldown((cooldown & 0x3) as u32),
        }
    }

    /// Apply a complete timing configuration to a memory window.
    pub fn configure_timing(&self, window: Window, cfg: &TimingConfig) {
        match window {
            Window::M0 => {
                self.qmi.m0_timing.write_val(
                    rp2350_pac::qmi::M0TimingVal::reset_val()
                        .with_clkdiv(cfg.clkdiv as u32)
                        .with_rxdelay(cfg.rxdelay as u32)
                        .with_min_deselect(cfg.min_deselect as u32)
                        .with_max_select(cfg.max_select as u32)
                        .with_select_hold(cfg.select_hold as u32)
                        .with_select_setup(cfg.select_setup)
                        .with_pagebreak(cfg.pagebreak as u32)
                        .with_cooldown(cfg.cooldown as u32),
                );
            }
            Window::M1 => {
                self.qmi.m1_timing.write_val(
                    rp2350_pac::qmi::M1TimingVal::reset_val()
                        .with_clkdiv(cfg.clkdiv as u32)
                        .with_rxdelay(cfg.rxdelay as u32)
                        .with_min_deselect(cfg.min_deselect as u32)
                        .with_max_select(cfg.max_select as u32)
                        .with_select_hold(cfg.select_hold as u32)
                        .with_select_setup(cfg.select_setup)
                        .with_pagebreak(cfg.pagebreak as u32)
                        .with_cooldown(cfg.cooldown as u32),
                );
            }
        }
    }

    // ========================================================================
    // Memory-mapped Mode — Read Format
    // ========================================================================

    /// Configure the read transfer format for a memory window.
    ///
    /// This controls how memory-mapped read accesses are translated into
    /// SPI flash read commands, including bus widths for each phase.
    pub fn configure_read_format(&self, window: Window, fmt: ReadFormat) {
        match window {
            Window::M0 => {
                self.qmi.m0_rfmt.write_val(
                    rp2350_pac::qmi::M0RfmtVal::reset_val()
                        .with_prefix_width(fmt.prefix_width as u32)
                        .with_addr_width(fmt.addr_width as u32)
                        .with_suffix_width(fmt.suffix_width as u32)
                        .with_dummy_width(fmt.dummy_width as u32)
                        .with_data_width(fmt.data_width as u32)
                        .with_prefix_len(fmt.prefix_len)
                        .with_suffix_len(fmt.suffix_len)
                        .with_dummy_len(fmt.dummy_len)
                        .with_dtr(fmt.dtr),
                );
            }
            Window::M1 => {
                self.qmi.m1_rfmt.write_val(
                    rp2350_pac::qmi::M1RfmtVal::reset_val()
                        .with_prefix_width(fmt.prefix_width as u32)
                        .with_addr_width(fmt.addr_width as u32)
                        .with_suffix_width(fmt.suffix_width as u32)
                        .with_dummy_width(fmt.dummy_width as u32)
                        .with_data_width(fmt.data_width as u32)
                        .with_prefix_len(fmt.prefix_len)
                        .with_suffix_len(fmt.suffix_len)
                        .with_dummy_len(fmt.dummy_len)
                        .with_dtr(fmt.dtr),
                );
            }
        }
    }

    /// Set the read command prefix and suffix bytes for a memory window.
    ///
    /// # Arguments
    ///
    /// * `window` - Memory window (M0 or M1)
    /// * `prefix` - Command byte (e.g., 0x03 for standard read, 0x6B for quad output read)
    /// * `suffix` - Suffix byte (e.g., 0xA0 for continuous read mode)
    pub fn set_read_command(&self, window: Window, prefix: u8, suffix: u8) {
        match window {
            Window::M0 => {
                self.qmi.m0_rcmd.write_val(
                    rp2350_pac::qmi::M0RcmdVal::reset_val()
                        .with_prefix(prefix as u32)
                        .with_suffix(suffix as u32),
                );
            }
            Window::M1 => {
                self.qmi.m1_rcmd.write_val(
                    rp2350_pac::qmi::M1RcmdVal::reset_val()
                        .with_prefix(prefix as u32)
                        .with_suffix(suffix as u32),
                );
            }
        }
    }

    // ========================================================================
    // Memory-mapped Mode — Write Format
    // ========================================================================

    /// Configure the write transfer format for a memory window.
    ///
    /// Note: XIP memory is read-only by default. Writes must be enabled
    /// via XIP_CTRL WRITABLE_M0 / WRITABLE_M1 bits.
    pub fn configure_write_format(&self, window: Window, fmt: WriteFormat) {
        match window {
            Window::M0 => {
                self.qmi.m0_wfmt.write_val(
                    rp2350_pac::qmi::M0WfmtVal::reset_val()
                        .with_prefix_width(fmt.prefix_width as u32)
                        .with_addr_width(fmt.addr_width as u32)
                        .with_suffix_width(fmt.suffix_width as u32)
                        .with_dummy_width(fmt.dummy_width as u32)
                        .with_data_width(fmt.data_width as u32)
                        .with_prefix_len(fmt.prefix_len)
                        .with_suffix_len(fmt.suffix_len)
                        .with_dummy_len(fmt.dummy_len)
                        .with_dtr(fmt.dtr),
                );
            }
            Window::M1 => {
                self.qmi.m1_wfmt.write_val(
                    rp2350_pac::qmi::M1WfmtVal::reset_val()
                        .with_prefix_width(fmt.prefix_width as u32)
                        .with_addr_width(fmt.addr_width as u32)
                        .with_suffix_width(fmt.suffix_width as u32)
                        .with_dummy_width(fmt.dummy_width as u32)
                        .with_data_width(fmt.data_width as u32)
                        .with_prefix_len(fmt.prefix_len)
                        .with_suffix_len(fmt.suffix_len)
                        .with_dummy_len(fmt.dummy_len)
                        .with_dtr(fmt.dtr),
                );
            }
        }
    }

    /// Set the write command prefix and suffix bytes for a memory window.
    ///
    /// # Arguments
    ///
    /// * `window` - Memory window (M0 or M1)
    /// * `prefix` - Command byte (e.g., 0x02 for page program, 0x32 for quad page program)
    /// * `suffix` - Suffix byte
    pub fn set_write_command(&self, window: Window, prefix: u8, suffix: u8) {
        match window {
            Window::M0 => {
                self.qmi.m0_wcmd.write_val(
                    rp2350_pac::qmi::M0WcmdVal::reset_val()
                        .with_prefix(prefix as u32)
                        .with_suffix(suffix as u32),
                );
            }
            Window::M1 => {
                self.qmi.m1_wcmd.write_val(
                    rp2350_pac::qmi::M1WcmdVal::reset_val()
                        .with_prefix(prefix as u32)
                        .with_suffix(suffix as u32),
                );
            }
        }
    }

    // ========================================================================
    // Memory-mapped Mode — Address Translation
    // ========================================================================

    /// Set address translation offset for a 4 MiB window.
    ///
    /// There are 8 translation windows (0-7), each covering a 4 MiB region
    /// of XIP virtual address space. The `offset` is added to the virtual
    /// address to produce the physical flash address.
    ///
    /// At reset, all windows are identity-mapped (offset = window_base).
    ///
    /// # Arguments
    ///
    /// * `window` - Translation window index (0-7)
    /// * `base` - Physical base address (in 4 KiB units, 12-bit)
    /// * `size` - Window size override (in 4 KiB units, 12-bit)
    ///
    /// # Note
    ///
    /// A cache flush is required after changing address translation.
    pub fn set_address_translation(&self, window_idx: u8, base: u32, size: u32) {
        match window_idx {
            0 => {
                self.qmi.atrans0.write_base(base);
                self.qmi.atrans0.write_size(size);
            }
            1 => {
                self.qmi.atrans1.write_base(base);
                self.qmi.atrans1.write_size(size);
            }
            2 => {
                self.qmi.atrans2.write_base(base);
                self.qmi.atrans2.write_size(size);
            }
            3 => {
                self.qmi.atrans3.write_base(base);
                self.qmi.atrans3.write_size(size);
            }
            4 => {
                self.qmi.atrans4.write_base(base);
                self.qmi.atrans4.write_size(size);
            }
            5 => {
                self.qmi.atrans5.write_base(base);
                self.qmi.atrans5.write_size(size);
            }
            6 => {
                self.qmi.atrans6.write_base(base);
                self.qmi.atrans6.write_size(size);
            }
            7 => {
                self.qmi.atrans7.write_base(base);
                self.qmi.atrans7.write_size(size);
            }
            _ => {} // Silently ignore invalid indices
        }
    }

    // ========================================================================
    // DMA Support
    // ========================================================================

    /// DREQ selection for transmitting data via DMA in direct mode.
    pub const TX_DREQ: crate::dma::TreqSel = crate::dma::TreqSel::XipQmiTx;

    /// DREQ selection for receiving data via DMA in direct mode.
    pub const RX_DREQ: crate::dma::TreqSel = crate::dma::TreqSel::XipQmiRx;

    /// Get the direct TX FIFO register address for DMA.
    #[inline]
    pub fn direct_tx_address(&self) -> u32 {
        QMI::PTR as u32 + 0x04 // DIRECT_TX offset
    }

    /// Get the direct RX FIFO register address for DMA.
    #[inline]
    pub fn direct_rx_address(&self) -> u32 {
        QMI::PTR as u32 + 0x08 // DIRECT_RX offset
    }

    // ========================================================================
    // High-level Configuration Presets
    // ========================================================================

    /// Configure a memory window for standard SPI read (command 0x03).
    ///
    /// This is the default after reset: single-width serial read, 8-bit command prefix,
    /// no dummy cycles, no suffix.
    pub fn configure_standard_read(&self, window: Window, clkdiv: u8) {
        self.set_clkdiv(window, clkdiv);
        self.configure_read_format(
            window,
            ReadFormat {
                prefix_width: BusWidth::Single,
                addr_width: BusWidth::Single,
                suffix_width: BusWidth::Single,
                dummy_width: BusWidth::Single,
                data_width: BusWidth::Single,
                prefix_len: true,
                suffix_len: 0,
                dummy_len: 0,
                dtr: false,
            },
        );
        self.set_read_command(window, 0x03, 0x00);
    }

    /// Configure a memory window for fast read (command 0x0B).
    ///
    /// Single-width serial fast read with 8 dummy bits.
    pub fn configure_fast_read(&self, window: Window, clkdiv: u8) {
        self.set_clkdiv(window, clkdiv);
        self.configure_read_format(
            window,
            ReadFormat {
                prefix_width: BusWidth::Single,
                addr_width: BusWidth::Single,
                suffix_width: BusWidth::Single,
                dummy_width: BusWidth::Single,
                data_width: BusWidth::Single,
                prefix_len: true,
                suffix_len: 0,
                dummy_len: 2, // 8 dummy bits = 2 x 4
                dtr: false,
            },
        );
        self.set_read_command(window, 0x0B, 0x00);
    }

    /// Configure a memory window for dual output read (command 0x3B).
    ///
    /// Command and address in single-width, data in dual-width, 8 dummy bits.
    pub fn configure_dual_read(&self, window: Window, clkdiv: u8) {
        self.set_clkdiv(window, clkdiv);
        self.configure_read_format(
            window,
            ReadFormat {
                prefix_width: BusWidth::Single,
                addr_width: BusWidth::Single,
                suffix_width: BusWidth::Single,
                dummy_width: BusWidth::Single,
                data_width: BusWidth::Dual,
                prefix_len: true,
                suffix_len: 0,
                dummy_len: 2, // 8 dummy bits
                dtr: false,
            },
        );
        self.set_read_command(window, 0x3B, 0x00);
    }

    /// Configure a memory window for quad output read (command 0x6B).
    ///
    /// Command and address in single-width, data in quad-width, 8 dummy bits.
    pub fn configure_quad_read(&self, window: Window, clkdiv: u8) {
        self.set_clkdiv(window, clkdiv);
        self.configure_read_format(
            window,
            ReadFormat {
                prefix_width: BusWidth::Single,
                addr_width: BusWidth::Single,
                suffix_width: BusWidth::Single,
                dummy_width: BusWidth::Single,
                data_width: BusWidth::Quad,
                prefix_len: true,
                suffix_len: 0,
                dummy_len: 2, // 8 dummy bits
                dtr: false,
            },
        );
        self.set_read_command(window, 0x6B, 0x00);
    }

    /// Configure a memory window for quad I/O read (command 0xEB).
    ///
    /// Command in single-width, address in quad-width, data in quad-width.
    /// Uses continuous read mode suffix (0xA0) and 4 dummy cycles (= 2 x 4 bits in quad).
    pub fn configure_quad_io_read(&self, window: Window, clkdiv: u8) {
        self.set_clkdiv(window, clkdiv);
        self.configure_read_format(
            window,
            ReadFormat {
                prefix_width: BusWidth::Single,
                addr_width: BusWidth::Quad,
                suffix_width: BusWidth::Quad,
                dummy_width: BusWidth::Quad,
                data_width: BusWidth::Quad,
                prefix_len: true,
                suffix_len: 2, // 8-bit suffix
                dummy_len: 1,  // 4 dummy bits (1 quad cycle)
                dtr: false,
            },
        );
        self.set_read_command(window, 0xEB, 0xA0);
    }

    /// Configure a memory window for standard page program (command 0x02).
    pub fn configure_standard_write(&self, window: Window) {
        self.configure_write_format(
            window,
            WriteFormat {
                prefix_width: BusWidth::Single,
                addr_width: BusWidth::Single,
                suffix_width: BusWidth::Single,
                dummy_width: BusWidth::Single,
                data_width: BusWidth::Single,
                prefix_len: true,
                suffix_len: 0,
                dummy_len: 0,
                dtr: false,
            },
        );
        self.set_write_command(window, 0x02, 0x00);
    }

    /// Configure a memory window for quad page program (command 0x32).
    pub fn configure_quad_write(&self, window: Window) {
        self.configure_write_format(
            window,
            WriteFormat {
                prefix_width: BusWidth::Single,
                addr_width: BusWidth::Single,
                suffix_width: BusWidth::Single,
                dummy_width: BusWidth::Single,
                data_width: BusWidth::Quad,
                prefix_len: true,
                suffix_len: 0,
                dummy_len: 0,
                dtr: false,
            },
        );
        self.set_write_command(window, 0x32, 0x00);
    }
}
