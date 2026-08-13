//! I2C (Inter-Integrated Circuit) HAL for RP2350
//!
//! This module provides a safe interface for the RP2350 I2C peripherals.
//! The RP2350 has two I2C controllers (I2C0 and I2C1).
//!
//! # Features
//!
//! - Master and slave modes
//! - Standard (100 kHz), Fast (400 kHz), and Fast Mode Plus (1 MHz)
//! - 7-bit and 10-bit addressing
//! - 16-byte TX and RX FIFOs
//! - **Type-safe pin ownership**: Pins are consumed by I2C, preventing misuse
//!
//! # GPIO Mapping
//!
//! I2C0:
//! - SDA: GPIO 0, 4, 8, 12, 16, 20, 24, 28
//! - SCL: GPIO 1, 5, 9, 13, 17, 21, 25, 29
//!
//! I2C1:
//! - SDA: GPIO 2, 6, 10, 14, 18, 22, 26
//! - SCL: GPIO 3, 7, 11, 15, 19, 23, 27
//!
//! # Example (Master Mode with Pin Ownership)
//!
//! ```ignore
//! use rp2350::gpio::Pins;
//! use rp2350::i2c::I2c0;
//! use rp2350::Peripherals;
//!
//! let dp = Peripherals::take().unwrap();
//! let pins = Pins::init(dp.IO_BANK0, dp.PADS_BANK0);
//!
//! // Initialize I2C0 at 400 kHz, consuming pins p4 (SDA) and p5 (SCL)
//! let i2c = I2c0::init(
//!     dp.I2C0,
//!     pins.pin::<4>(),      // SDA pin - ownership transferred
//!     pins.pin::<5>(),      // SCL pin - ownership transferred
//!     400_000,      // 400 kHz
//!     &clocks,
//! );
//!
//! // Write to a device
//! i2c.write(0x50, &[0x00, 0x01, 0x02]).unwrap();
//!
//! // Read from a device
//! let mut buf = [0u8; 4];
//! i2c.read(0x50, &mut buf).unwrap();
//!
//! // Compile error: pins.pin::<4>() already moved!
//! // pins.pin::<4>().into_output();
//! ```
//!
//! # Example (Slave Mode)
//!
//! ```ignore
//! use rp2350::gpio::Pins;
//! use rp2350::i2c::I2c0Slave;
//! use rp2350::Peripherals;
//!
//! let dp = Peripherals::take().unwrap();
//! let pins = Pins::init(dp.IO_BANK0, dp.PADS_BANK0);
//!
//! // Initialize I2C0 as slave with address 0x55
//! let i2c = I2c0Slave::init(
//!     dp.I2C0,
//!     pins.pin::<4>(),
//!     pins.pin::<5>(),
//!     0x55,
//!     150_000_000,
//! );
//!
//! // Wait for and read data from master
//! let mut buf = [0u8; 4];
//! let count = i2c.read(&mut buf).unwrap();
//!
//! // Write data for master to read
//! i2c.write(&[0x01, 0x02, 0x03]).unwrap();
//! ```

use paste::paste;

use crate::clocks::Clocks;
use crate::gpio::{I2c0SclPin, I2c0SdaPin, I2c1SclPin, I2c1SdaPin};
use crate::reset::Resettable;

/// Abstract I2C bus trait for driver implementations.
///
/// This keeps drivers decoupled from concrete I2C types while preserving
/// the HAL error type.
pub trait I2cBus {
    /// Error type returned by the bus.
    type Error;

    /// Write data to a 7-bit I2C address.
    fn write(&self, addr: u8, data: &[u8]) -> Result<(), Self::Error>;

    /// Read data from a 7-bit I2C address.
    fn read(&self, addr: u8, buffer: &mut [u8]) -> Result<(), Self::Error>;

    /// Write then read with a repeated start.
    fn write_read(
        &self,
        addr: u8,
        write_data: &[u8],
        read_buffer: &mut [u8],
    ) -> Result<(), Self::Error>;
}

/// I2C speed mode
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Speed {
    /// Standard mode (100 kHz)
    Standard = 1,
    /// Fast mode (400 kHz) and Fast mode plus (1 MHz)
    /// Both use the same IC_CON speed setting, frequency is determined by clock timing
    Fast = 2,
}

/// I2C error types
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// Arbitration lost
    ArbitrationLoss,
    /// No acknowledgement received
    NoAcknowledge,
    /// TX FIFO overflow
    TxOverflow,
    /// RX FIFO overflow
    RxOverflow,
    /// RX FIFO underflow
    RxUnderflow,
    /// Abort condition
    Abort,
    /// Timeout
    Timeout,
}

/// I2C mode
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mode {
    /// Master mode
    Master,
    /// Slave mode
    Slave,
}

// ============================================================================
// Macro for generating I2C master implementations
// ============================================================================

macro_rules! impl_i2c {
    ($I2cName:ident, $i2c_num:literal, $I2cType:ty, $i2c_mod:ident, $SdaPin:ident, $SclPin:ident) => {
        paste! {
            /// I2C master peripheral wrapper with type-safe pin ownership.
            ///
            /// This struct owns both the I2C peripheral and the GPIO pins used for
            /// SDA and SCL. The pins are configured automatically when the I2C
            /// instance is created, and their ownership prevents them from being
            /// used for other purposes.
            ///
            #[doc = concat!("# I2C", $i2c_num, " Master Pin Mapping")]
            ///
            #[doc = concat!("See [`", stringify!($SdaPin), "`] for valid SDA pins.")]
            ///
            #[doc = concat!("See [`", stringify!($SclPin), "`] for valid SCL pins.")]
            ///
            /// For slave mode, use
            #[doc = concat!("[`", stringify!($I2cName), "Slave`].")]
            pub struct $I2cName<SDA, SCL>
            where
                SDA: $SdaPin,
                SCL: $SclPin,
            {
                i2c: $I2cType,
                _sda: SDA,
                _scl: SCL,
            }

            impl<SDA, SCL> $I2cName<SDA, SCL>
            where
                SDA: $SdaPin,
                SCL: $SclPin,
            {
                /// I2C peripheral number
                pub const NUM: u8 = $i2c_num;

                // ================================================================
                // Initialization (Master Mode)
                // ================================================================

                /// Initialize I2C in master mode.
                ///
                /// This method consumes the I2C peripheral and GPIO pins, configuring
                /// the pins for I2C function automatically (including pull-up resistors).
                ///
                /// # Arguments
                ///
                /// * `i2c` - The I2C peripheral from PAC
                /// * `sda` - SDA pin (must implement the appropriate SDA trait)
                /// * `scl` - SCL pin (must implement the appropriate SCL trait)
                /// * `baudrate` - Desired baudrate in Hz (e.g., 100_000 for 100 kHz)
                /// * `clocks` - System clocks
                ///
                /// # Type Safety
                ///
                /// Only pins that implement the correct trait can be used:
                #[doc = concat!("- SDA must implement [`", stringify!($SdaPin), "`]")]
                #[doc = concat!("- SCL must implement [`", stringify!($SclPin), "`]")]
                ///
                /// # Example
                ///
                /// ```ignore
                #[doc = concat!("let i2c = ", stringify!($I2cName), "::init(")]
                #[doc = concat!("    dp.I2C", $i2c_num, ",")]
                ///     pins.pin::<4>(),      // SDA
                ///     pins.pin::<5>(),      // SCL
                ///     400_000,      // 400 kHz
                ///     &clocks,
                /// );
                /// ```
                pub fn init(i2c: $I2cType, mut sda: SDA, mut scl: SCL, baudrate: u32, clocks: &Clocks) -> Self {
                    // Configure pins for I2C function (includes pull-up)
                    sda.[<configure_i2c $i2c_num _sda>]();
                    scl.[<configure_i2c $i2c_num _scl>]();

                    // Reset peripheral to ensure clean register state
                    <$I2cType as Resettable>::init();

                    let mut i2c_dev = Self {
                        i2c,
                        _sda: sda,
                        _scl: scl,
                    };

                    // Disable I2C to configure
                    i2c_dev.i2c.ic_enable.clr_enable();

                    // Disable all interrupts and clear pending
                    i2c_dev.i2c.ic_intr_mask.write_bits(0);
                    let _ = i2c_dev.i2c.ic_clr_intr.read_bits();

                    // Configure as master: clear all, then set required bits
                    i2c_dev.i2c.ic_con.write_bits(0);
                    i2c_dev.i2c.ic_con.set_master_mode();
                    i2c_dev.i2c.ic_con.set_ic_restart_en();
                    i2c_dev.i2c.ic_con.set_ic_slave_disable();
                    i2c_dev.i2c.ic_con.set_tx_empty_ctrl();

                    // Set baudrate (also configures speed mode in IC_CON)
                    i2c_dev.configure_baudrate(baudrate, clocks.sys_freq());

                    // Set FIFO thresholds
                    i2c_dev.i2c.ic_tx_tl.write_tx_tl(0);
                    i2c_dev.i2c.ic_rx_tl.write_rx_tl(0);

                    // Enable I2C
                    i2c_dev.i2c.ic_enable.set_enable();

                    i2c_dev
                }

                /// Obtain a pin-erased I2C handle without ownership.
                ///
                /// Returns `$I2cName<ErasedPin, ErasedPin>` — a handle that can access
                /// all I2C registers without carrying pin type parameters.
                /// This is useful for ISR context or global storage where the
                /// concrete pin types are inconvenient.
                ///
                /// # Safety
                ///
                /// The caller must ensure:
                /// 1. The I2C peripheral has already been initialised (pins
                ///    configured, baud rate set, peripheral taken out of reset).
                /// 2. No data races: the returned handle must not be used
                ///    concurrently with the owning instance without proper
                ///    synchronisation (e.g. `Mutex`).
                ///
                /// # Example
                ///
                /// ```ignore
                #[doc = concat!("// In init code: create the full I2C with pins")]
                #[doc = concat!("let i2c = ", stringify!($I2cName), "::init(")]
                #[doc = concat!("    dp.I2C", $i2c_num, ", sda_pin, scl_pin, 400_000, &clocks,")]
                /// );
                ///
                /// // Later, in ISR or another context:
                #[doc = concat!("let i2c = unsafe { ", stringify!($I2cName), "::<ErasedPin, ErasedPin>::steal() };")]
                /// let mut buf = [0u8; 2];
                /// i2c.read(0x50, &mut buf).unwrap();
                /// ```
                #[inline]
                pub unsafe fn steal() -> crate::Steal<$I2cName<crate::gpio::pins::ErasedPin, crate::gpio::pins::ErasedPin>> {
                    crate::Steal($I2cName {
                        i2c: unsafe { <$I2cType>::steal() },
                        _sda: crate::gpio::pins::ErasedPin,
                        _scl: crate::gpio::pins::ErasedPin,
                    })
                }

                /// Get reference to the underlying register block.
                ///
                /// # Safety
                /// Direct register access can bypass the HAL's safety guarantees.
                /// The caller must ensure operations do not conflict with the HAL state.
                #[inline]
                pub unsafe fn regs(&self) -> &rp2350_pac::$i2c_mod::RegisterBlock {
                    &self.i2c
                }

                /// Release the I2C peripheral and pins.
                ///
                /// Returns a tuple of (I2C peripheral, SDA pin, SCL pin).
                /// The pins can then be reconfigured for other uses.
                #[inline]
                pub fn release(self) -> ($I2cType, SDA, SCL) {
                    (self.i2c, self._sda, self._scl)
                }

                // ================================================================
                // Configuration
                // ================================================================

                /// Configure the I2C baudrate (internal helper)
                fn configure_baudrate(&mut self, baudrate: u32, sys_clk: u32) {
                    // Determine speed mode
                    let speed = if baudrate >= 400_000 {
                        Speed::Fast
                    } else {
                        Speed::Standard
                    };

                    // Update speed in IC_CON
                    self.i2c.ic_con.write_speed(speed as u32);

                    // Calculate clock counts
                    let period = sys_clk / baudrate;

                    let (lcnt, hcnt) = match speed {
                        Speed::Standard => {
                            let hcnt = period * 4 / 10;
                            let lcnt = period - hcnt;
                            (lcnt, hcnt)
                        }
                        Speed::Fast => {
                            let hcnt = period * 3 / 10;
                            let lcnt = period - hcnt;
                            (lcnt, hcnt)
                        }
                    };

                    // Apply minimum values and subtract IC internal delays
                    let hcnt = hcnt.saturating_sub(8).max(6);
                    let lcnt = lcnt.saturating_sub(1).max(8);

                    // Write SCL timing registers
                    match speed {
                        Speed::Standard => {
                            self.i2c.ic_ss_scl_hcnt.write_ic_ss_scl_hcnt(hcnt);
                            self.i2c.ic_ss_scl_lcnt.write_ic_ss_scl_lcnt(lcnt);
                        }
                        Speed::Fast => {
                            self.i2c.ic_fs_scl_hcnt.write_ic_fs_scl_hcnt(hcnt);
                            self.i2c.ic_fs_scl_lcnt.write_ic_fs_scl_lcnt(lcnt);
                        }
                    }

                    // Set spike suppression
                    let spklen = if baudrate >= 1_000_000 { 1 } else { 7 };
                    self.i2c.ic_fs_spklen.write_ic_fs_spklen(spklen);

                    // Set SDA hold time
                    let sda_hold = (sys_clk / 1_000_000 / 3).max(1);
                    self.i2c.ic_sda_hold.write_ic_sda_tx_hold(sda_hold);
                }

                /// Set the I2C baudrate (master mode only).
                pub fn set_baudrate(&mut self, baudrate: u32, sys_clk: u32) {
                    self.i2c.ic_enable.clr_enable();
                    self.configure_baudrate(baudrate, sys_clk);
                    self.i2c.ic_enable.set_enable();
                }

                // ================================================================
                // Control
                // ================================================================

                /// Enable the I2C peripheral.
                #[inline]
                pub fn enable(&self) {
                    self.i2c.ic_enable.set_enable();
                }

                /// Disable the I2C peripheral.
                #[inline]
                pub fn disable(&self) {
                    self.i2c.ic_enable.clr_enable();
                }

                /// Check if I2C peripheral is enabled.
                #[inline]
                pub fn is_enabled(&self) -> bool {
                    self.i2c.ic_enable.enable()
                }

                /// Set the target address for master mode transactions.
                ///
                /// This is a no-op when the requested address (and 10-bit flag)
                /// already matches the hardware, so repeated calls from
                /// consecutive transactions don't pay the disable → configure →
                /// enable cost on every bus access.
                #[inline]
                pub fn set_target_addr(&self, addr: u16, addr_10bit: bool) {
                    if self.i2c.ic_tar.ic_tar() == addr as u32
                        && self.i2c.ic_con.ic_10bitaddr_master() == addr_10bit
                    {
                        return;
                    }

                    // Disable to change address
                    self.i2c.ic_enable.clr_enable();

                    // Update 10-bit addressing in IC_CON
                    if addr_10bit {
                        self.i2c.ic_con.set_ic_10bitaddr_master();
                    } else {
                        self.i2c.ic_con.clr_ic_10bitaddr_master();
                    }

                    // Set target address
                    self.i2c.ic_tar.write_ic_tar(addr as u32);

                    // Re-enable
                    self.i2c.ic_enable.set_enable();
                }

                // ================================================================
                // Master Mode Operations
                // ================================================================

                /// Write data to an I2C device (master mode).
                ///
                /// Writing an empty slice performs an **address probe**: it issues
                /// START + address + STOP and reports whether a device acknowledged
                /// its address (see [`Error::NoAcknowledge`]).
                ///
                /// # Arguments
                /// * `addr` - 7-bit I2C address
                /// * `data` - Data bytes to write
                pub fn write(&self, addr: u8, data: &[u8]) -> Result<(), Error> {
                    use rp2350_pac::$i2c_mod::IcDataCmd;

                    self.set_target_addr(addr as u16, false);

                    if data.is_empty() {
                        // Address probe: send a single zero byte with STOP. If the
                        // target NACKs its address, the wait below reports it via
                        // `check_abort`.
                        self.wait_cond(|| self.i2c.ic_status.tfnf())?;
                        self.i2c.ic_data_cmd.write_bits(IcDataCmd::STOP);
                    } else {
                        for (i, &byte) in data.iter().enumerate() {
                            let last = i == data.len() - 1;

                            // Wait for TX FIFO not full (bounded)
                            self.wait_cond(|| self.i2c.ic_status.tfnf())?;

                            // Write data command
                            let cmd = (byte as u32) | if last { IcDataCmd::STOP } else { 0 };
                            self.i2c.ic_data_cmd.write_bits(cmd);
                        }
                    }

                    // Wait for TX FIFO to empty (bounded)
                    self.wait_cond(|| self.i2c.ic_status.tfe())?;

                    // Wait for not busy (bounded)
                    self.wait_cond(|| !self.i2c.ic_status.mst_activity())?;

                    Ok(())
                }

                /// Read data from an I2C device (master mode).
                ///
                /// # Arguments
                /// * `addr` - 7-bit I2C address
                /// * `buffer` - Buffer to store received data
                pub fn read(&self, addr: u8, buffer: &mut [u8]) -> Result<(), Error> {
                    use rp2350_pac::$i2c_mod::IcDataCmd;

                    if buffer.is_empty() {
                        return Ok(());
                    }

                    self.set_target_addr(addr as u16, false);

                    let mut tx_idx = 0;
                    let mut rx_idx = 0;
                    let len = buffer.len();

                    while rx_idx < len {
                        // Send read commands while TX FIFO has space
                        while tx_idx < len && self.i2c.ic_status.tfnf() {
                            let last = tx_idx == len - 1;
                            let cmd = IcDataCmd::CMD | if last { IcDataCmd::STOP } else { 0 };
                            self.i2c.ic_data_cmd.write_bits(cmd);
                            tx_idx += 1;
                        }

                        // Read from RX FIFO
                        while rx_idx < len && self.i2c.ic_status.rfne() {
                            buffer[rx_idx] = self.i2c.ic_data_cmd.dat() as u8;
                            rx_idx += 1;
                        }

                        // Wait (bounded) for RX data or TX FIFO space so the
                        // loop can make progress.
                        self.wait_cond(|| {
                            self.i2c.ic_status.rfne()
                                || (tx_idx < len && self.i2c.ic_status.tfnf())
                        })?;
                    }

                    // Wait for the STOP to finish and the bus to idle, so a
                    // back-to-back transaction cannot race it.
                    self.wait_cond(|| !self.i2c.ic_status.mst_activity())?;

                    Ok(())
                }

                /// Write then read (combined transaction with repeated start).
                ///
                /// # Arguments
                /// * `addr` - 7-bit I2C address
                /// * `write_data` - Data to write
                /// * `read_buffer` - Buffer for data to read
                pub fn write_read(
                    &self,
                    addr: u8,
                    write_data: &[u8],
                    read_buffer: &mut [u8],
                ) -> Result<(), Error> {
                    use rp2350_pac::$i2c_mod::IcDataCmd;

                    if write_data.is_empty() && read_buffer.is_empty() {
                        return Ok(());
                    }

                    // A write with no read phase is just a plain write; delegate
                    // so the transaction is terminated with a STOP.
                    if read_buffer.is_empty() {
                        return self.write(addr, write_data);
                    }

                    self.set_target_addr(addr as u16, false);

                    // Write phase (no STOP — a repeated START follows)
                    for &byte in write_data {
                        self.wait_cond(|| self.i2c.ic_status.tfnf())?;
                        self.i2c.ic_data_cmd.write_dat(byte as u32);
                    }

                    // Read phase
                    let mut tx_idx = 0;
                    let mut rx_idx = 0;
                    let len = read_buffer.len();

                    while rx_idx < len {
                        // Send read commands
                        while tx_idx < len && self.i2c.ic_status.tfnf() {
                            let last = tx_idx == len - 1;
                            let cmd = IcDataCmd::CMD | if last { IcDataCmd::STOP } else { 0 };
                            self.i2c.ic_data_cmd.write_bits(cmd);
                            tx_idx += 1;
                        }

                        // Read from RX FIFO
                        while rx_idx < len && self.i2c.ic_status.rfne() {
                            read_buffer[rx_idx] = self.i2c.ic_data_cmd.dat() as u8;
                            rx_idx += 1;
                        }

                        // Wait (bounded) for RX data or TX FIFO space so the
                        // loop can make progress.
                        self.wait_cond(|| {
                            self.i2c.ic_status.rfne()
                                || (tx_idx < len && self.i2c.ic_status.tfnf())
                        })?;
                    }

                    // Wait for the STOP to finish and the bus to idle.
                    self.wait_cond(|| !self.i2c.ic_status.mst_activity())?;

                    Ok(())
                }

                // ================================================================
                // Status and Errors
                // ================================================================

                /// Check for abort conditions.
                fn check_abort(&self) -> Result<(), Error> {
                    if self.i2c.ic_raw_intr_stat.tx_abrt() {
                        let source = self.i2c.ic_tx_abrt_source.read_bits();
                        // Clear abort flag
                        let _ = self.i2c.ic_clr_tx_abrt.read_bits();

                        if source & (1 << 0) != 0 {
                            return Err(Error::NoAcknowledge);
                        }
                        if source & (1 << 12) != 0 {
                            return Err(Error::ArbitrationLoss);
                        }
                        return Err(Error::Abort);
                    }
                    Ok(())
                }

                /// Spin (bounded) until `cond` is true, checking for I2C abort
                /// each iteration. Returns [`Error::Timeout`] if the hardware
                /// does not make progress within
                /// [`crate::spin::SPIN_TIMEOUT`] iterations.
                fn wait_cond(&self, cond: impl Fn() -> bool) -> Result<(), Error> {
                    let mut spins = 0u32;
                    while !cond() {
                        self.check_abort()?;
                        core::hint::spin_loop();
                        spins += 1;
                        if spins >= crate::spin::SPIN_TIMEOUT {
                            return Err(Error::Timeout);
                        }
                    }
                    Ok(())
                }

                /// Get the TX FIFO level.
                #[inline]
                pub fn tx_fifo_level(&self) -> u8 {
                    self.i2c.ic_txflr.txflr() as u8
                }

                /// Get the RX FIFO level.
                #[inline]
                pub fn rx_fifo_level(&self) -> u8 {
                    self.i2c.ic_rxflr.rxflr() as u8
                }

                /// Check if TX FIFO is empty.
                #[inline]
                pub fn is_tx_empty(&self) -> bool {
                    self.i2c.ic_status.tfe()
                }

                /// Check if RX FIFO is not empty.
                #[inline]
                pub fn is_rx_not_empty(&self) -> bool {
                    self.i2c.ic_status.rfne()
                }

                /// Check if this master is actively transferring data.
                #[inline]
                pub fn is_busy(&self) -> bool {
                    self.i2c.ic_status.mst_activity()
                }

                /// Get the current baudrate by reading hardware registers.
                ///
                /// # Arguments
                /// * `sys_clk` - System clock frequency in Hz
                #[inline]
                pub fn baudrate(&self, sys_clk: u32) -> u32 {
                    let speed = self.i2c.ic_con.speed();
                    let (hcnt, lcnt) = if speed >= Speed::Fast as u32 {
                        (
                            self.i2c.ic_fs_scl_hcnt.ic_fs_scl_hcnt(),
                            self.i2c.ic_fs_scl_lcnt.ic_fs_scl_lcnt(),
                        )
                    } else {
                        (
                            self.i2c.ic_ss_scl_hcnt.ic_ss_scl_hcnt(),
                            self.i2c.ic_ss_scl_lcnt.ic_ss_scl_lcnt(),
                        )
                    };
                    let total = hcnt + 8 + lcnt + 1;
                    if total == 0 {
                        return 0;
                    }
                    sys_clk / total
                }

                /// Get the current mode by reading the IC_CON register.
                #[inline]
                pub fn mode(&self) -> Mode {
                    if self.i2c.ic_con.master_mode() {
                        Mode::Master
                    } else {
                        Mode::Slave
                    }
                }

                // ================================================================
                // Interrupts
                // ================================================================

                // ---- Mask operations (enable/disable) ----

                /// Enable specific interrupts using mask.
                pub fn enable_interrupts(&self, mask: u32) {
                    self.i2c.ic_intr_mask.set_bits(mask);
                }

                /// Disable specific interrupts using mask.
                pub fn disable_interrupts(&self, mask: u32) {
                    self.i2c.ic_intr_mask.clr_bits(mask);
                }

                /// Disable all interrupts.
                #[inline]
                pub fn disable_all_interrupts(&self) {
                    self.i2c.ic_intr_mask.write_bits(0);
                }

                // ---- RX FULL interrupt ----

                /// Enable RX FULL interrupt (RX FIFO reaches or exceeds threshold).
                #[inline]
                pub fn enable_rx_full_interrupt(&self) {
                    self.i2c.ic_intr_mask.set_m_rx_full();
                }

                /// Disable RX FULL interrupt.
                #[inline]
                pub fn disable_rx_full_interrupt(&self) {
                    self.i2c.ic_intr_mask.clr_m_rx_full();
                }

                /// Check if RX FULL interrupt is enabled.
                #[inline]
                pub fn is_rx_full_interrupt_enabled(&self) -> bool {
                    self.i2c.ic_intr_mask.m_rx_full()
                }

                // ---- TX EMPTY interrupt ----

                /// Enable TX EMPTY interrupt (TX FIFO at or below threshold).
                #[inline]
                pub fn enable_tx_empty_interrupt(&self) {
                    self.i2c.ic_intr_mask.set_m_tx_empty();
                }

                /// Disable TX EMPTY interrupt.
                #[inline]
                pub fn disable_tx_empty_interrupt(&self) {
                    self.i2c.ic_intr_mask.clr_m_tx_empty();
                }

                /// Check if TX EMPTY interrupt is enabled.
                #[inline]
                pub fn is_tx_empty_interrupt_enabled(&self) -> bool {
                    self.i2c.ic_intr_mask.m_tx_empty()
                }

                // ---- TX ABRT interrupt ----

                /// Enable TX abort interrupt.
                #[inline]
                pub fn enable_tx_abort_interrupt(&self) {
                    self.i2c.ic_intr_mask.set_m_tx_abrt();
                }

                /// Disable TX abort interrupt.
                #[inline]
                pub fn disable_tx_abort_interrupt(&self) {
                    self.i2c.ic_intr_mask.clr_m_tx_abrt();
                }

                // ---- STOP DET interrupt ----

                /// Enable stop detected interrupt.
                #[inline]
                pub fn enable_stop_det_interrupt(&self) {
                    self.i2c.ic_intr_mask.set_m_stop_det();
                }

                /// Disable stop detected interrupt.
                #[inline]
                pub fn disable_stop_det_interrupt(&self) {
                    self.i2c.ic_intr_mask.clr_m_stop_det();
                }

                // ---- START DET interrupt ----

                /// Enable start detected interrupt.
                #[inline]
                pub fn enable_start_det_interrupt(&self) {
                    self.i2c.ic_intr_mask.set_m_start_det();
                }

                /// Disable start detected interrupt.
                #[inline]
                pub fn disable_start_det_interrupt(&self) {
                    self.i2c.ic_intr_mask.clr_m_start_det();
                }

                // ---- ACTIVITY interrupt ----

                /// Enable activity interrupt.
                #[inline]
                pub fn enable_activity_interrupt(&self) {
                    self.i2c.ic_intr_mask.set_m_activity();
                }

                /// Disable activity interrupt.
                #[inline]
                pub fn disable_activity_interrupt(&self) {
                    self.i2c.ic_intr_mask.clr_m_activity();
                }

                // ---- Error interrupts ----

                /// Enable RX overflow interrupt.
                #[inline]
                pub fn enable_rx_over_interrupt(&self) {
                    self.i2c.ic_intr_mask.set_m_rx_over();
                }

                /// Disable RX overflow interrupt.
                #[inline]
                pub fn disable_rx_over_interrupt(&self) {
                    self.i2c.ic_intr_mask.clr_m_rx_over();
                }

                /// Enable RX underflow interrupt.
                #[inline]
                pub fn enable_rx_under_interrupt(&self) {
                    self.i2c.ic_intr_mask.set_m_rx_under();
                }

                /// Disable RX underflow interrupt.
                #[inline]
                pub fn disable_rx_under_interrupt(&self) {
                    self.i2c.ic_intr_mask.clr_m_rx_under();
                }

                /// Enable TX overflow interrupt.
                #[inline]
                pub fn enable_tx_over_interrupt(&self) {
                    self.i2c.ic_intr_mask.set_m_tx_over();
                }

                /// Disable TX overflow interrupt.
                #[inline]
                pub fn disable_tx_over_interrupt(&self) {
                    self.i2c.ic_intr_mask.clr_m_tx_over();
                }

                // ---- Status reading ----

                /// Get raw interrupt status.
                #[inline]
                pub fn raw_interrupt_status(&self) -> u32 {
                    self.i2c.ic_raw_intr_stat.read_bits()
                }

                /// Get masked interrupt status.
                #[inline]
                pub fn interrupt_status(&self) -> u32 {
                    self.i2c.ic_intr_stat.read_bits()
                }

                // ---- Clear interrupts ----

                /// Clear all interrupts.
                #[inline]
                pub fn clear_all_interrupts(&self) {
                    let _ = self.i2c.ic_clr_intr.read_bits();
                }

                /// Clear RX UNDER interrupt.
                #[inline]
                pub fn clear_rx_under_interrupt(&self) {
                    let _ = self.i2c.ic_clr_rx_under.read_bits();
                }

                /// Clear RX OVER interrupt.
                #[inline]
                pub fn clear_rx_over_interrupt(&self) {
                    let _ = self.i2c.ic_clr_rx_over.read_bits();
                }

                /// Clear TX OVER interrupt.
                #[inline]
                pub fn clear_tx_over_interrupt(&self) {
                    let _ = self.i2c.ic_clr_tx_over.read_bits();
                }

                /// Clear TX ABRT interrupt.
                #[inline]
                pub fn clear_tx_abort_interrupt(&self) {
                    let _ = self.i2c.ic_clr_tx_abrt.read_bits();
                }

                /// Clear ACTIVITY interrupt.
                #[inline]
                pub fn clear_activity_interrupt(&self) {
                    let _ = self.i2c.ic_clr_activity.read_bits();
                }

                /// Clear STOP DET interrupt.
                #[inline]
                pub fn clear_stop_det_interrupt(&self) {
                    let _ = self.i2c.ic_clr_stop_det.read_bits();
                }

                /// Clear START DET interrupt.
                #[inline]
                pub fn clear_start_det_interrupt(&self) {
                    let _ = self.i2c.ic_clr_start_det.read_bits();
                }

                /// Clear GEN CALL interrupt.
                #[inline]
                pub fn clear_gen_call_interrupt(&self) {
                    let _ = self.i2c.ic_clr_gen_call.read_bits();
                }

                /// Clear RESTART DET interrupt.
                #[inline]
                pub fn clear_restart_det_interrupt(&self) {
                    let _ = self.i2c.ic_clr_restart_det.read_bits();
                }

                // ================================================================
                // DMA Support
                // ================================================================

                /// DREQ selection for transmitting data via DMA.
                pub const TX_DREQ: crate::dma::TreqSel = crate::dma::TreqSel::[<I2c $i2c_num Tx>];

                /// DREQ selection for receiving data via DMA.
                pub const RX_DREQ: crate::dma::TreqSel = crate::dma::TreqSel::[<I2c $i2c_num Rx>];

                /// Get the I2C data/command register (IC_DATA_CMD) address for DMA transfers.
                ///
                /// IC_DATA_CMD is at offset `0x10` from the I2C base address.
                #[inline]
                pub fn data_register_address(&self) -> u32 {
                    <$I2cType>::PTR as u32 + 0x10
                }

                /// Enable DMA requests for the transmit FIFO.
                #[inline]
                pub fn enable_dma_tx(&self) {
                    self.i2c.ic_dma_cr.set_tdmae();
                }

                /// Disable DMA requests for the transmit FIFO.
                #[inline]
                pub fn disable_dma_tx(&self) {
                    self.i2c.ic_dma_cr.clr_tdmae();
                }

                /// Enable DMA requests for the receive FIFO.
                #[inline]
                pub fn enable_dma_rx(&self) {
                    self.i2c.ic_dma_cr.set_rdmae();
                }

                /// Disable DMA requests for the receive FIFO.
                #[inline]
                pub fn disable_dma_rx(&self) {
                    self.i2c.ic_dma_cr.clr_rdmae();
                }

                /// Set the DMA transmit data level.
                ///
                /// Controls the level at which a DMA request is generated for TX.
                /// The DMA TX request fires when the number of entries in the
                /// TX FIFO is at or below this threshold.
                #[inline]
                pub fn set_dma_tx_data_level(&self, level: u8) {
                    self.i2c.ic_dma_tdlr.write_dmatdl(level as u32);
                }

                /// Set the DMA receive data level.
                ///
                /// Controls the level at which a DMA request is generated for RX.
                /// The DMA RX request fires when the number of entries in the
                /// RX FIFO is at or above this threshold + 1.
                #[inline]
                pub fn set_dma_rx_data_level(&self, level: u8) {
                    self.i2c.ic_dma_rdlr.write_dmardl(level as u32);
                }
            }

            impl<SDA, SCL> I2cBus for $I2cName<SDA, SCL>
            where
                SDA: $SdaPin,
                SCL: $SclPin,
            {
                type Error = Error;

                #[inline]
                fn write(&self, addr: u8, data: &[u8]) -> Result<(), Self::Error> {
                    Self::write(self, addr, data)
                }

                #[inline]
                fn read(&self, addr: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
                    Self::read(self, addr, buffer)
                }

                #[inline]
                fn write_read(
                    &self,
                    addr: u8,
                    write_data: &[u8],
                    read_buffer: &mut [u8],
                ) -> Result<(), Self::Error> {
                    Self::write_read(self, addr, write_data, read_buffer)
                }
            }
        }
    };
}

// ============================================================================
// Macro for generating I2C slave implementations
// ============================================================================

macro_rules! impl_i2c_slave {
    ($I2cName:ident, $i2c_num:literal, $I2cType:ty, $i2c_mod:ident, $SdaPin:ident, $SclPin:ident) => {
        paste! {
            /// I2C slave peripheral wrapper with type-safe pin ownership.
            ///
            /// This struct owns both the I2C peripheral and the GPIO pins used for
            /// SDA and SCL. The pins are configured automatically when the I2C
            /// instance is created, and their ownership prevents them from being
            /// used for other purposes.
            ///
            #[doc = concat!("# I2C", $i2c_num, " Slave Pin Mapping")]
            ///
            #[doc = concat!("See [`", stringify!($SdaPin), "`] for valid SDA pins.")]
            ///
            #[doc = concat!("See [`", stringify!($SclPin), "`] for valid SCL pins.")]
            pub struct $I2cName<SDA, SCL>
            where
                SDA: $SdaPin,
                SCL: $SclPin,
            {
                i2c: $I2cType,
                _sda: SDA,
                _scl: SCL,
            }

            impl<SDA, SCL> $I2cName<SDA, SCL>
            where
                SDA: $SdaPin,
                SCL: $SclPin,
            {
                /// I2C peripheral number
                pub const NUM: u8 = $i2c_num;

                // ================================================================
                // Initialization (Slave Mode)
                // ================================================================

                /// Initialize I2C in slave mode with 7-bit address.
                ///
                /// # Arguments
                ///
                /// * `i2c` - The I2C peripheral from PAC
                /// * `sda` - SDA pin (must implement the appropriate SDA trait)
                /// * `scl` - SCL pin (must implement the appropriate SCL trait)
                /// * `addr` - 7-bit slave address
                /// * `sys_clk` - System clock frequency in Hz
                ///
                /// # Example
                ///
                /// ```ignore
                #[doc = concat!("let i2c = ", stringify!($I2cName), "::init(")]
                #[doc = concat!("    dp.I2C", $i2c_num, ",")]
                ///     pins.pin::<4>(),      // SDA
                ///     pins.pin::<5>(),      // SCL
                ///     0x55,         // Slave address
                ///     150_000_000,  // System clock
                /// );
                /// ```
                pub fn init(i2c: $I2cType, sda: SDA, scl: SCL, addr: u8, sys_clk: u32) -> Self {
                    Self::init_impl(i2c, sda, scl, addr as u16, false, sys_clk)
                }

                /// Initialize I2C in slave mode with 10-bit address.
                ///
                /// # Arguments
                ///
                /// * `i2c` - The I2C peripheral from PAC
                /// * `sda` - SDA pin (must implement the appropriate SDA trait)
                /// * `scl` - SCL pin (must implement the appropriate SCL trait)
                /// * `addr` - 10-bit slave address
                /// * `sys_clk` - System clock frequency in Hz
                pub fn init_10bit(i2c: $I2cType, sda: SDA, scl: SCL, addr: u16, sys_clk: u32) -> Self {
                    Self::init_impl(i2c, sda, scl, addr, true, sys_clk)
                }

                /// Shared slave-mode initialization.
                fn init_impl(i2c: $I2cType, mut sda: SDA, mut scl: SCL, addr: u16, addr_10bit: bool, sys_clk: u32) -> Self {
                    // Configure pins for I2C function
                    sda.[<configure_i2c $i2c_num _sda>]();
                    scl.[<configure_i2c $i2c_num _scl>]();

                    // Reset peripheral to ensure clean register state
                    <$I2cType as Resettable>::init();

                    let i2c_dev = Self {
                        i2c,
                        _sda: sda,
                        _scl: scl,
                    };

                    // Disable I2C to configure
                    i2c_dev.i2c.ic_enable.clr_enable();

                    // Disable all interrupts and clear pending
                    i2c_dev.i2c.ic_intr_mask.write_bits(0);
                    let _ = i2c_dev.i2c.ic_clr_intr.read_bits();

                    // Configure as slave: clear all, then set required bits
                    i2c_dev.i2c.ic_con.write_bits(0);
                    i2c_dev.i2c.ic_con.write_speed(Speed::Fast as u32);
                    if addr_10bit {
                        i2c_dev.i2c.ic_con.set_ic_10bitaddr_slave();
                    }
                    i2c_dev.i2c.ic_con.set_stop_det_ifaddressed();
                    i2c_dev.i2c.ic_con.set_rx_fifo_full_hld_ctrl();
                    i2c_dev.i2c.ic_con.set_tx_empty_ctrl();

                    // Set slave address
                    i2c_dev.i2c.ic_sar.write_ic_sar(addr as u32);

                    // Set FIFO thresholds
                    i2c_dev.i2c.ic_tx_tl.write_tx_tl(0);
                    i2c_dev.i2c.ic_rx_tl.write_rx_tl(0);

                    // Configure timing for slave (use sensible defaults)
                    let sda_hold = (sys_clk / 1_000_000 / 3).max(1);
                    i2c_dev.i2c.ic_sda_hold.write_ic_sda_tx_hold(sda_hold);

                    // Enable I2C
                    i2c_dev.i2c.ic_enable.set_enable();

                    i2c_dev
                }

                // ================================================================
                // Steal / Release
                // ================================================================

                /// Obtain a pin-erased I2C slave handle without ownership.
                ///
                /// # Safety
                ///
                /// The caller must ensure:
                /// 1. The I2C peripheral has already been initialised (pins
                ///    configured, slave address set, peripheral taken out of reset).
                /// 2. No data races: the returned handle must not be used
                ///    concurrently with the owning instance without proper
                ///    synchronisation (e.g. `Mutex`).
                #[inline]
                pub unsafe fn steal() -> crate::Steal<$I2cName<crate::gpio::pins::ErasedPin, crate::gpio::pins::ErasedPin>> {
                    crate::Steal($I2cName {
                        i2c: unsafe { <$I2cType>::steal() },
                        _sda: crate::gpio::pins::ErasedPin,
                        _scl: crate::gpio::pins::ErasedPin,
                    })
                }

                /// Get reference to the underlying register block.
                ///
                /// # Safety
                /// Direct register access can bypass the HAL's safety guarantees.
                /// The caller must ensure operations do not conflict with the HAL state.
                #[inline]
                pub unsafe fn regs(&self) -> &rp2350_pac::$i2c_mod::RegisterBlock {
                    &self.i2c
                }

                /// Release the I2C peripheral and pins.
                ///
                /// Returns a tuple of (I2C peripheral, SDA pin, SCL pin).
                /// The pins can then be reconfigured for other uses.
                #[inline]
                pub fn release(self) -> ($I2cType, SDA, SCL) {
                    (self.i2c, self._sda, self._scl)
                }

                // ================================================================
                // Slave Mode Operations
                // ================================================================

                /// Read data sent by master (slave mode).
                ///
                /// Returns the number of bytes read. Blocks until STOP condition.
                pub fn read(&self, buffer: &mut [u8]) -> Result<usize, Error> {
                    let mut count = 0;

                    // Clear any pending interrupts
                    let _ = self.i2c.ic_clr_intr.read_bits();

                    // Wait for data or STOP
                    loop {
                        // Check for RX data
                        while self.i2c.ic_status.rfne() && count < buffer.len() {
                            buffer[count] = self.i2c.ic_data_cmd.dat() as u8;
                            count += 1;
                        }

                        // Check for STOP condition
                        if self.i2c.ic_raw_intr_stat.stop_det() {
                            // Read any remaining data
                            while self.i2c.ic_status.rfne() && count < buffer.len() {
                                buffer[count] = self.i2c.ic_data_cmd.dat() as u8;
                                count += 1;
                            }
                            // Clear STOP flag
                            let _ = self.i2c.ic_clr_stop_det.read_bits();
                            break;
                        }

                        // Check for errors
                        if self.i2c.ic_raw_intr_stat.rx_over() {
                            let _ = self.i2c.ic_clr_rx_over.read_bits();
                            return Err(Error::RxOverflow);
                        }
                    }

                    Ok(count)
                }

                /// Read data sent by master (slave mode, non-blocking).
                ///
                /// Returns the number of bytes read.
                pub fn read_available(&self, buffer: &mut [u8]) -> usize {
                    let mut count = 0;
                    while self.i2c.ic_status.rfne() && count < buffer.len() {
                        buffer[count] = self.i2c.ic_data_cmd.dat() as u8;
                        count += 1;
                    }
                    count
                }

                /// Write data for master to read (slave mode).
                ///
                /// This writes data to the TX FIFO when master requests a read.
                /// Call this when you detect a read request (RD_REQ interrupt).
                pub fn write(&self, data: &[u8]) -> Result<(), Error> {
                    for &byte in data {
                        // Wait for TX FIFO space
                        while !self.i2c.ic_status.tfnf() {
                            if self.i2c.ic_raw_intr_stat.tx_over() {
                                let _ = self.i2c.ic_clr_tx_over.read_bits();
                                return Err(Error::TxOverflow);
                            }
                        }

                        // Write data
                        self.i2c.ic_data_cmd.write_dat(byte as u32);
                    }

                    // Clear read request flag
                    let _ = self.i2c.ic_clr_rd_req.read_bits();

                    Ok(())
                }

                /// Check if master is requesting a read (slave mode).
                #[inline]
                pub fn read_requested(&self) -> bool {
                    self.i2c.ic_raw_intr_stat.rd_req()
                }

                /// Clear read request flag (slave mode).
                #[inline]
                pub fn clear_read_request(&self) {
                    let _ = self.i2c.ic_clr_rd_req.read_bits();
                }

                /// Check if a STOP condition was detected (slave mode).
                #[inline]
                pub fn stop_detected(&self) -> bool {
                    self.i2c.ic_raw_intr_stat.stop_det()
                }

                /// Clear STOP detection flag (slave mode).
                #[inline]
                pub fn clear_stop(&self) {
                    let _ = self.i2c.ic_clr_stop_det.read_bits();
                }

                /// Check if a START condition was detected (slave mode).
                #[inline]
                pub fn start_detected(&self) -> bool {
                    self.i2c.ic_raw_intr_stat.start_det()
                }

                /// Clear START detection flag (slave mode).
                #[inline]
                pub fn clear_start(&self) {
                    let _ = self.i2c.ic_clr_start_det.read_bits();
                }

                // ================================================================
                // Control
                // ================================================================

                /// Enable the I2C peripheral.
                #[inline]
                pub fn enable(&self) {
                    self.i2c.ic_enable.set_enable();
                }

                /// Disable the I2C peripheral.
                #[inline]
                pub fn disable(&self) {
                    self.i2c.ic_enable.clr_enable();
                }

                /// Check if I2C peripheral is enabled.
                #[inline]
                pub fn is_enabled(&self) -> bool {
                    self.i2c.ic_enable.enable()
                }

                // ================================================================
                // Status
                // ================================================================

                /// Get the TX FIFO level.
                #[inline]
                pub fn tx_fifo_level(&self) -> u8 {
                    self.i2c.ic_txflr.txflr() as u8
                }

                /// Get the RX FIFO level.
                #[inline]
                pub fn rx_fifo_level(&self) -> u8 {
                    self.i2c.ic_rxflr.rxflr() as u8
                }

                /// Check if TX FIFO is empty.
                #[inline]
                pub fn is_tx_empty(&self) -> bool {
                    self.i2c.ic_status.tfe()
                }

                /// Check if RX FIFO is not empty.
                #[inline]
                pub fn is_rx_not_empty(&self) -> bool {
                    self.i2c.ic_status.rfne()
                }

                /// Check if I2C is busy.
                #[inline]
                pub fn is_busy(&self) -> bool {
                    self.i2c.ic_status.activity()
                }

                /// Get the current mode.
                #[inline]
                pub fn mode(&self) -> Mode {
                    Mode::Slave
                }

                // ================================================================
                // Interrupts
                // ================================================================

                // ---- Mask operations (enable/disable) ----

                /// Enable specific interrupts using mask.
                pub fn enable_interrupts(&self, mask: u32) {
                    self.i2c.ic_intr_mask.set_bits(mask);
                }

                /// Disable specific interrupts using mask.
                pub fn disable_interrupts(&self, mask: u32) {
                    self.i2c.ic_intr_mask.clr_bits(mask);
                }

                /// Disable all interrupts.
                #[inline]
                pub fn disable_all_interrupts(&self) {
                    self.i2c.ic_intr_mask.write_bits(0);
                }

                // ---- RX FULL interrupt ----

                /// Enable RX FULL interrupt (RX FIFO reaches or exceeds threshold).
                #[inline]
                pub fn enable_rx_full_interrupt(&self) {
                    self.i2c.ic_intr_mask.set_m_rx_full();
                }

                /// Disable RX FULL interrupt.
                #[inline]
                pub fn disable_rx_full_interrupt(&self) {
                    self.i2c.ic_intr_mask.clr_m_rx_full();
                }

                /// Check if RX FULL interrupt is enabled.
                #[inline]
                pub fn is_rx_full_interrupt_enabled(&self) -> bool {
                    self.i2c.ic_intr_mask.m_rx_full()
                }

                // ---- TX EMPTY interrupt ----

                /// Enable TX EMPTY interrupt (TX FIFO at or below threshold).
                #[inline]
                pub fn enable_tx_empty_interrupt(&self) {
                    self.i2c.ic_intr_mask.set_m_tx_empty();
                }

                /// Disable TX EMPTY interrupt.
                #[inline]
                pub fn disable_tx_empty_interrupt(&self) {
                    self.i2c.ic_intr_mask.clr_m_tx_empty();
                }

                /// Check if TX EMPTY interrupt is enabled.
                #[inline]
                pub fn is_tx_empty_interrupt_enabled(&self) -> bool {
                    self.i2c.ic_intr_mask.m_tx_empty()
                }

                // ---- TX ABRT interrupt ----

                /// Enable TX abort interrupt.
                #[inline]
                pub fn enable_tx_abort_interrupt(&self) {
                    self.i2c.ic_intr_mask.set_m_tx_abrt();
                }

                /// Disable TX abort interrupt.
                #[inline]
                pub fn disable_tx_abort_interrupt(&self) {
                    self.i2c.ic_intr_mask.clr_m_tx_abrt();
                }

                // ---- RD REQ interrupt ----

                /// Enable read request interrupt (master wants to read).
                #[inline]
                pub fn enable_rd_req_interrupt(&self) {
                    self.i2c.ic_intr_mask.set_m_rd_req();
                }

                /// Disable read request interrupt.
                #[inline]
                pub fn disable_rd_req_interrupt(&self) {
                    self.i2c.ic_intr_mask.clr_m_rd_req();
                }

                // ---- STOP DET interrupt ----

                /// Enable stop detected interrupt.
                #[inline]
                pub fn enable_stop_det_interrupt(&self) {
                    self.i2c.ic_intr_mask.set_m_stop_det();
                }

                /// Disable stop detected interrupt.
                #[inline]
                pub fn disable_stop_det_interrupt(&self) {
                    self.i2c.ic_intr_mask.clr_m_stop_det();
                }

                // ---- START DET interrupt ----

                /// Enable start detected interrupt.
                #[inline]
                pub fn enable_start_det_interrupt(&self) {
                    self.i2c.ic_intr_mask.set_m_start_det();
                }

                /// Disable start detected interrupt.
                #[inline]
                pub fn disable_start_det_interrupt(&self) {
                    self.i2c.ic_intr_mask.clr_m_start_det();
                }

                // ---- ACTIVITY interrupt ----

                /// Enable activity interrupt.
                #[inline]
                pub fn enable_activity_interrupt(&self) {
                    self.i2c.ic_intr_mask.set_m_activity();
                }

                /// Disable activity interrupt.
                #[inline]
                pub fn disable_activity_interrupt(&self) {
                    self.i2c.ic_intr_mask.clr_m_activity();
                }

                // ---- Error interrupts ----

                /// Enable RX overflow interrupt.
                #[inline]
                pub fn enable_rx_over_interrupt(&self) {
                    self.i2c.ic_intr_mask.set_m_rx_over();
                }

                /// Disable RX overflow interrupt.
                #[inline]
                pub fn disable_rx_over_interrupt(&self) {
                    self.i2c.ic_intr_mask.clr_m_rx_over();
                }

                /// Enable RX underflow interrupt.
                #[inline]
                pub fn enable_rx_under_interrupt(&self) {
                    self.i2c.ic_intr_mask.set_m_rx_under();
                }

                /// Disable RX underflow interrupt.
                #[inline]
                pub fn disable_rx_under_interrupt(&self) {
                    self.i2c.ic_intr_mask.clr_m_rx_under();
                }

                /// Enable TX overflow interrupt.
                #[inline]
                pub fn enable_tx_over_interrupt(&self) {
                    self.i2c.ic_intr_mask.set_m_tx_over();
                }

                /// Disable TX overflow interrupt.
                #[inline]
                pub fn disable_tx_over_interrupt(&self) {
                    self.i2c.ic_intr_mask.clr_m_tx_over();
                }

                // ---- Status reading ----

                /// Get raw interrupt status.
                #[inline]
                pub fn raw_interrupt_status(&self) -> u32 {
                    self.i2c.ic_raw_intr_stat.read_bits()
                }

                /// Get masked interrupt status.
                #[inline]
                pub fn interrupt_status(&self) -> u32 {
                    self.i2c.ic_intr_stat.read_bits()
                }

                // ---- Clear interrupts ----

                /// Clear all interrupts.
                #[inline]
                pub fn clear_all_interrupts(&self) {
                    let _ = self.i2c.ic_clr_intr.read_bits();
                }

                /// Clear RX UNDER interrupt.
                #[inline]
                pub fn clear_rx_under_interrupt(&self) {
                    let _ = self.i2c.ic_clr_rx_under.read_bits();
                }

                /// Clear RX OVER interrupt.
                #[inline]
                pub fn clear_rx_over_interrupt(&self) {
                    let _ = self.i2c.ic_clr_rx_over.read_bits();
                }

                /// Clear TX OVER interrupt.
                #[inline]
                pub fn clear_tx_over_interrupt(&self) {
                    let _ = self.i2c.ic_clr_tx_over.read_bits();
                }

                /// Clear TX ABRT interrupt.
                #[inline]
                pub fn clear_tx_abort_interrupt(&self) {
                    let _ = self.i2c.ic_clr_tx_abrt.read_bits();
                }

                /// Clear ACTIVITY interrupt.
                #[inline]
                pub fn clear_activity_interrupt(&self) {
                    let _ = self.i2c.ic_clr_activity.read_bits();
                }

                /// Clear STOP DET interrupt.
                #[inline]
                pub fn clear_stop_det_interrupt(&self) {
                    let _ = self.i2c.ic_clr_stop_det.read_bits();
                }

                /// Clear START DET interrupt.
                #[inline]
                pub fn clear_start_det_interrupt(&self) {
                    let _ = self.i2c.ic_clr_start_det.read_bits();
                }

                /// Clear GEN CALL interrupt.
                #[inline]
                pub fn clear_gen_call_interrupt(&self) {
                    let _ = self.i2c.ic_clr_gen_call.read_bits();
                }

                /// Clear RESTART DET interrupt.
                #[inline]
                pub fn clear_restart_det_interrupt(&self) {
                    let _ = self.i2c.ic_clr_restart_det.read_bits();
                }

                // ================================================================
                // DMA Support
                // ================================================================

                /// DREQ selection for transmitting data via DMA.
                pub const TX_DREQ: crate::dma::TreqSel = crate::dma::TreqSel::[<I2c $i2c_num Tx>];

                /// DREQ selection for receiving data via DMA.
                pub const RX_DREQ: crate::dma::TreqSel = crate::dma::TreqSel::[<I2c $i2c_num Rx>];

                /// Get the I2C data/command register (IC_DATA_CMD) address for DMA transfers.
                ///
                /// IC_DATA_CMD is at offset `0x10` from the I2C base address.
                #[inline]
                pub fn data_register_address(&self) -> u32 {
                    <$I2cType>::PTR as u32 + 0x10
                }

                /// Enable DMA requests for the transmit FIFO.
                #[inline]
                pub fn enable_dma_tx(&self) {
                    self.i2c.ic_dma_cr.set_tdmae();
                }

                /// Disable DMA requests for the transmit FIFO.
                #[inline]
                pub fn disable_dma_tx(&self) {
                    self.i2c.ic_dma_cr.clr_tdmae();
                }

                /// Enable DMA requests for the receive FIFO.
                #[inline]
                pub fn enable_dma_rx(&self) {
                    self.i2c.ic_dma_cr.set_rdmae();
                }

                /// Disable DMA requests for the receive FIFO.
                #[inline]
                pub fn disable_dma_rx(&self) {
                    self.i2c.ic_dma_cr.clr_rdmae();
                }

                /// Set the DMA transmit data level.
                ///
                /// Controls the level at which a DMA request is generated for TX.
                /// The DMA TX request fires when the number of entries in the
                /// TX FIFO is at or below this threshold.
                #[inline]
                pub fn set_dma_tx_data_level(&self, level: u8) {
                    self.i2c.ic_dma_tdlr.write_dmatdl(level as u32);
                }

                /// Set the DMA receive data level.
                ///
                /// Controls the level at which a DMA request is generated for RX.
                /// The DMA RX request fires when the number of entries in the
                /// RX FIFO is at or above this threshold + 1.
                #[inline]
                pub fn set_dma_rx_data_level(&self, level: u8) {
                    self.i2c.ic_dma_rdlr.write_dmardl(level as u32);
                }
            }
        }
    };
}

// ============================================================================
// Generate I2C0 and I2C1 (Master)
// ============================================================================

impl_i2c!(I2c0, 0, rp2350_pac::I2C0, i2c0, I2c0SdaPin, I2c0SclPin);
impl_i2c!(I2c1, 1, rp2350_pac::I2C1, i2c1, I2c1SdaPin, I2c1SclPin);

// ============================================================================
// Generate I2C0Slave and I2C1Slave
// ============================================================================

impl_i2c_slave!(I2c0Slave, 0, rp2350_pac::I2C0, i2c0, I2c0SdaPin, I2c0SclPin);

impl_i2c_slave!(I2c1Slave, 1, rp2350_pac::I2C1, i2c1, I2c1SdaPin, I2c1SclPin);
