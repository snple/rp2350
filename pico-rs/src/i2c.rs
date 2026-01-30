//! I2C (Inter-Integrated Circuit) HAL for RP2350
//!
//! This module provides a safe interface for the RP2350 I2C peripherals.
//! The RP2350 has two I2C controllers (I2C0 and I2C1).
//!
//! ## Features
//!
//! - Master and slave modes
//! - Standard (100 kHz), Fast (400 kHz), and Fast Mode Plus (1 MHz)
//! - 7-bit and 10-bit addressing
//! - 16-byte TX and RX FIFOs
//!
//! ## GPIO Mapping
//!
//! I2C0:
//! - SDA: GPIO 0, 4, 8, 12, 16, 20, 24, 28
//! - SCL: GPIO 1, 5, 9, 13, 17, 21, 25, 29
//!
//! I2C1:
//! - SDA: GPIO 2, 6, 10, 14, 18, 22, 26
//! - SCL: GPIO 3, 7, 11, 15, 19, 23, 27
//!
//! ## Example (Master Mode)
//!
//! ```ignore
//! use pico_rs::i2c::I2c0;
//!
//! // Initialize I2C0 at 400 kHz
//! let mut i2c = I2c0::init_master(400_000, 150_000_000);
//!
//! // Write to a device
//! i2c.write(0x50, &[0x00, 0x01, 0x02]).unwrap();
//!
//! // Read from a device
//! let mut buf = [0u8; 4];
//! i2c.read(0x50, &mut buf).unwrap();
//! ```
//!
//! ## Example (Slave Mode)
//!
//! ```ignore
//! use pico_rs::i2c::I2c0;
//!
//! // Initialize I2C0 as slave with address 0x55
//! let i2c = I2c0::init_slave(0x55, 150_000_000);
//!
//! // Wait for and read data from master
//! let mut buf = [0u8; 4];
//! let count = i2c.slave_read(&mut buf).unwrap();
//!
//! // Write data for master to read
//! i2c.slave_write(&[0x01, 0x02, 0x03]).unwrap();
//! ```

use paste::paste;

use crate::reset::{self, Peripheral};

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
// Macro for generating I2C implementations
// ============================================================================

macro_rules! impl_i2c {
    ($I2cName:ident, $i2c_num:literal, $I2cType:ty, $i2c_mod:ident, $reset_peripheral:expr) => {
        paste! {
            /// I2C$i2c_num peripheral wrapper
            pub struct $I2cName {
                regs: &'static pico_rs_pac::$i2c_mod::RegisterBlock,
                baudrate: u32,
                mode: Mode,
            }

            impl $I2cName {
                /// I2C peripheral number
                pub const NUM: u8 = $i2c_num;

                /// Create a new I2C instance without initialization
                #[inline]
                pub fn new() -> Self {
                    Self {
                        regs: unsafe { &*<$I2cType>::PTR },
                        baudrate: 0,
                        mode: Mode::Master,
                    }
                }

                /// Get reference to the register block
                #[inline]
                pub fn regs(&self) -> &pico_rs_pac::$i2c_mod::RegisterBlock {
                    self.regs
                }

                // ================================================================
                // Initialization
                // ================================================================

                /// Initialize I2C in master mode
                ///
                /// # Arguments
                /// * `baudrate` - Desired baudrate in Hz (e.g., 100_000 for 100 kHz)
                /// * `sys_clk` - System clock frequency in Hz
                pub fn init_master(baudrate: u32, sys_clk: u32) -> Self {
                    // Bring I2C out of reset
                    reset::unreset_block_wait($reset_peripheral);

                    let mut i2c = Self::new();
                    i2c.mode = Mode::Master;

                    // Disable I2C to configure
                    i2c.regs.ic_enable.clr_enable();

                    // Configure as master: clear all, then set required bits
                    i2c.regs.ic_con.write_bits(0);
                    i2c.regs.ic_con.set_master_mode();
                    i2c.regs.ic_con.set_ic_restart_en();
                    i2c.regs.ic_con.set_ic_slave_disable();
                    i2c.regs.ic_con.set_tx_empty_ctrl();

                    // Set baudrate (also configures speed mode in IC_CON)
                    i2c.configure_baudrate(baudrate, sys_clk);

                    // Set FIFO thresholds
                    i2c.regs.ic_tx_tl.write_tx_tl(0);
                    i2c.regs.ic_rx_tl.write_rx_tl(0);

                    // Enable I2C
                    i2c.regs.ic_enable.set_enable();

                    i2c.baudrate = baudrate;
                    i2c
                }

                /// Initialize I2C in slave mode
                ///
                /// # Arguments
                /// * `addr` - 7-bit slave address
                /// * `sys_clk` - System clock frequency in Hz
                pub fn init_slave(addr: u8, sys_clk: u32) -> Self {
                    // Bring I2C out of reset
                    reset::unreset_block_wait($reset_peripheral);

                    let mut i2c = Self::new();
                    i2c.mode = Mode::Slave;

                    // Disable I2C to configure
                    i2c.regs.ic_enable.clr_enable();

                    // Configure as slave: clear all, then set required bits
                    i2c.regs.ic_con.write_bits(0);
                    i2c.regs.ic_con.write_speed(Speed::Fast as u32);
                    i2c.regs.ic_con.set_stop_det_ifaddressed();
                    i2c.regs.ic_con.set_rx_fifo_full_hld_ctrl();
                    i2c.regs.ic_con.set_tx_empty_ctrl();

                    // Set slave address
                    i2c.regs.ic_sar.write_ic_sar(addr as u32);

                    // Set FIFO thresholds
                    i2c.regs.ic_tx_tl.write_tx_tl(0);
                    i2c.regs.ic_rx_tl.write_rx_tl(0);

                    // Configure timing for slave (use sensible defaults)
                    let sda_hold = (sys_clk / 1_000_000 / 3).max(1);
                    i2c.regs.ic_sda_hold.write_ic_sda_tx_hold(sda_hold);

                    // Enable I2C
                    i2c.regs.ic_enable.set_enable();

                    i2c
                }

                /// Initialize I2C in slave mode with 10-bit address
                ///
                /// # Arguments
                /// * `addr` - 10-bit slave address
                /// * `sys_clk` - System clock frequency in Hz
                pub fn init_slave_10bit(addr: u16, sys_clk: u32) -> Self {
                    // Bring I2C out of reset
                    reset::unreset_block_wait($reset_peripheral);

                    let mut i2c = Self::new();
                    i2c.mode = Mode::Slave;

                    // Disable I2C to configure
                    i2c.regs.ic_enable.clr_enable();

                    // Configure as slave with 10-bit addressing: clear all, then set required bits
                    i2c.regs.ic_con.write_bits(0);
                    i2c.regs.ic_con.write_speed(Speed::Fast as u32);
                    i2c.regs.ic_con.set_ic_10bitaddr_slave();
                    i2c.regs.ic_con.set_stop_det_ifaddressed();
                    i2c.regs.ic_con.set_rx_fifo_full_hld_ctrl();
                    i2c.regs.ic_con.set_tx_empty_ctrl();

                    // Set slave address
                    i2c.regs.ic_sar.write_ic_sar(addr as u32);

                    // Set FIFO thresholds
                    i2c.regs.ic_tx_tl.write_tx_tl(0);
                    i2c.regs.ic_rx_tl.write_rx_tl(0);

                    // Configure timing for slave
                    let sda_hold = (sys_clk / 1_000_000 / 3).max(1);
                    i2c.regs.ic_sda_hold.write_ic_sda_tx_hold(sda_hold);

                    // Enable I2C
                    i2c.regs.ic_enable.set_enable();

                    i2c
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
                    self.regs.ic_con.write_speed(speed as u32);

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
                            self.regs.ic_ss_scl_hcnt.write_ic_ss_scl_hcnt(hcnt);
                            self.regs.ic_ss_scl_lcnt.write_ic_ss_scl_lcnt(lcnt);
                        }
                        Speed::Fast => {
                            self.regs.ic_fs_scl_hcnt.write_ic_fs_scl_hcnt(hcnt);
                            self.regs.ic_fs_scl_lcnt.write_ic_fs_scl_lcnt(lcnt);
                        }
                    }

                    // Set spike suppression
                    let spklen = if baudrate >= 1_000_000 { 1 } else { 7 };
                    self.regs.ic_fs_spklen.write_ic_fs_spklen(spklen);

                    // Set SDA hold time
                    let sda_hold = (sys_clk / 1_000_000 / 3).max(1);
                    self.regs.ic_sda_hold.write_ic_sda_tx_hold(sda_hold);

                    self.baudrate = baudrate;
                }

                /// Set the I2C baudrate (master mode only)
                pub fn set_baudrate(&mut self, baudrate: u32, sys_clk: u32) {
                    self.regs.ic_enable.clr_enable();
                    self.configure_baudrate(baudrate, sys_clk);
                    self.regs.ic_enable.set_enable();
                }

                /// Enable/disable I2C peripheral
                #[inline]
                pub fn set_enabled(&self, enable: bool) {
                    if enable {
                        self.regs.ic_enable.set_enable();
                    } else {
                        self.regs.ic_enable.clr_enable();
                    }
                }

                /// Set the target address for master mode transactions
                #[inline]
                pub fn set_target_addr(&self, addr: u16, addr_10bit: bool) {
                    // Disable to change address
                    self.regs.ic_enable.clr_enable();

                    // Update 10-bit addressing in IC_CON
                    if addr_10bit {
                        self.regs.ic_con.set_ic_10bitaddr_master();
                    } else {
                        self.regs.ic_con.clr_ic_10bitaddr_master();
                    }

                    // Set target address
                    self.regs.ic_tar.write_ic_tar(addr as u32);

                    // Re-enable
                    self.regs.ic_enable.set_enable();
                }

                // ================================================================
                // Master Mode Operations
                // ================================================================

                /// Write data to an I2C device (master mode)
                ///
                /// # Arguments
                /// * `addr` - 7-bit I2C address
                /// * `data` - Data bytes to write
                pub fn write(&self, addr: u8, data: &[u8]) -> Result<(), Error> {
                    use pico_rs_pac::$i2c_mod::IcDataCmd;

                    if data.is_empty() {
                        return Ok(());
                    }

                    self.set_target_addr(addr as u16, false);

                    for (i, &byte) in data.iter().enumerate() {
                        let last = i == data.len() - 1;

                        // Wait for TX FIFO not full
                        while !self.regs.ic_status.tfnf() {
                            self.check_abort()?;
                        }

                        // Write data command
                        let cmd = (byte as u32) | if last { IcDataCmd::STOP } else { 0 };
                        self.regs.ic_data_cmd.write_bits(cmd);
                    }

                    // Wait for TX FIFO to empty
                    while !self.regs.ic_status.tfe() {
                        self.check_abort()?;
                    }

                    // Wait for not busy
                    while self.regs.ic_status.mst_activity() {
                        self.check_abort()?;
                    }

                    Ok(())
                }

                /// Read data from an I2C device (master mode)
                ///
                /// # Arguments
                /// * `addr` - 7-bit I2C address
                /// * `buffer` - Buffer to store received data
                pub fn read(&self, addr: u8, buffer: &mut [u8]) -> Result<(), Error> {
                    use pico_rs_pac::$i2c_mod::IcDataCmd;

                    if buffer.is_empty() {
                        return Ok(());
                    }

                    self.set_target_addr(addr as u16, false);

                    let mut tx_idx = 0;
                    let mut rx_idx = 0;
                    let len = buffer.len();

                    while rx_idx < len {
                        // Send read commands while TX FIFO has space
                        while tx_idx < len && self.regs.ic_status.tfnf() {
                            let last = tx_idx == len - 1;
                            let cmd = IcDataCmd::CMD | if last { IcDataCmd::STOP } else { 0 };
                            self.regs.ic_data_cmd.write_bits(cmd);
                            tx_idx += 1;
                        }

                        // Read from RX FIFO
                        while rx_idx < len && self.regs.ic_status.rfne() {
                            buffer[rx_idx] = self.regs.ic_data_cmd.dat() as u8;
                            rx_idx += 1;
                        }

                        self.check_abort()?;
                    }

                    Ok(())
                }

                /// Write then read (combined transaction with repeated start)
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
                    use pico_rs_pac::$i2c_mod::IcDataCmd;

                    if write_data.is_empty() && read_buffer.is_empty() {
                        return Ok(());
                    }

                    self.set_target_addr(addr as u16, false);

                    // Write phase (no STOP)
                    for &byte in write_data {
                        while !self.regs.ic_status.tfnf() {
                            self.check_abort()?;
                        }
                        self.regs.ic_data_cmd.write_dat(byte as u32);
                    }

                    // Read phase
                    let mut tx_idx = 0;
                    let mut rx_idx = 0;
                    let len = read_buffer.len();

                    while rx_idx < len {
                        while tx_idx < len && self.regs.ic_status.tfnf() {
                            let last = tx_idx == len - 1;
                            let cmd = IcDataCmd::CMD | if last { IcDataCmd::STOP } else { 0 };
                            self.regs.ic_data_cmd.write_bits(cmd);
                            tx_idx += 1;
                        }

                        while rx_idx < len && self.regs.ic_status.rfne() {
                            read_buffer[rx_idx] = self.regs.ic_data_cmd.dat() as u8;
                            rx_idx += 1;
                        }

                        self.check_abort()?;
                    }

                    Ok(())
                }

                // ================================================================
                // Slave Mode Operations
                // ================================================================

                /// Read data sent by master (slave mode)
                ///
                /// Returns the number of bytes read. Blocks until STOP condition.
                pub fn slave_read(&self, buffer: &mut [u8]) -> Result<usize, Error> {
                    let mut count = 0;

                    // Clear any pending interrupts
                    let _ = self.regs.ic_clr_stop_det.read_bits();

                    loop {
                        // Check for STOP condition
                        if self.regs.ic_raw_intr_stat.stop_det() {
                            let _ = self.regs.ic_clr_stop_det.read_bits();
                            break;
                        }

                        // Read from RX FIFO if data available
                        if self.regs.ic_status.rfne() {
                            if count < buffer.len() {
                                buffer[count] = self.regs.ic_data_cmd.dat() as u8;
                                count += 1;
                            } else {
                                // Buffer full, discard data
                                let _ = self.regs.ic_data_cmd.dat();
                            }
                        }

                        // Check for errors
                        if self.regs.ic_raw_intr_stat.rx_over() {
                            let _ = self.regs.ic_clr_rx_over.read_bits();
                            return Err(Error::RxOverflow);
                        }
                    }

                    Ok(count)
                }

                /// Read data sent by master (slave mode, non-blocking)
                ///
                /// Returns the number of bytes read.
                pub fn slave_read_available(&self, buffer: &mut [u8]) -> usize {
                    let mut count = 0;

                    while count < buffer.len() {
                        if self.regs.ic_status.rfne() {
                            buffer[count] = self.regs.ic_data_cmd.dat() as u8;
                            count += 1;
                        } else {
                            break;
                        }
                    }

                    count
                }

                /// Write data for master to read (slave mode)
                ///
                /// This writes data to the TX FIFO when master requests a read.
                /// Call this when you detect a read request (RD_REQ interrupt).
                pub fn slave_write(&self, data: &[u8]) -> Result<(), Error> {
                    for &byte in data {
                        // Wait for TX FIFO not full or read request
                        loop {
                            if self.regs.ic_status.tfnf() {
                                break;
                            }

                            // Check for TX abort
                            if self.regs.ic_raw_intr_stat.tx_abrt() {
                                let _ = self.regs.ic_clr_tx_abrt.read_bits();
                                return Err(Error::Abort);
                            }
                        }

                        self.regs.ic_data_cmd.write_dat(byte as u32);
                    }

                    Ok(())
                }

                /// Check if master is requesting a read (slave mode)
                #[inline]
                pub fn slave_read_requested(&self) -> bool {
                    self.regs.ic_raw_intr_stat.rd_req()
                }

                /// Clear read request flag (slave mode)
                #[inline]
                pub fn slave_clear_read_request(&self) {
                    let _ = self.regs.ic_clr_rd_req.read_bits();
                }

                /// Check if a STOP condition was detected (slave mode)
                #[inline]
                pub fn slave_stop_detected(&self) -> bool {
                    self.regs.ic_raw_intr_stat.stop_det()
                }

                /// Clear STOP detection flag (slave mode)
                #[inline]
                pub fn slave_clear_stop(&self) {
                    let _ = self.regs.ic_clr_stop_det.read_bits();
                }

                /// Check if a START condition was detected (slave mode)
                #[inline]
                pub fn slave_start_detected(&self) -> bool {
                    self.regs.ic_raw_intr_stat.start_det()
                }

                /// Clear START detection flag (slave mode)
                #[inline]
                pub fn slave_clear_start(&self) {
                    let _ = self.regs.ic_clr_start_det.read_bits();
                }

                // ================================================================
                // Status and Errors
                // ================================================================

                /// Check for abort conditions
                fn check_abort(&self) -> Result<(), Error> {
                    let abort_src = self.regs.ic_tx_abrt_source.read_bits();

                    if abort_src == 0 {
                        return Ok(());
                    }

                    // Clear abort by reading IC_CLR_TX_ABRT
                    let _ = self.regs.ic_clr_tx_abrt.read_bits();

                    // Determine error type
                    if abort_src & (1 << 12) != 0 {
                        Err(Error::ArbitrationLoss)
                    } else if abort_src & 0x1f != 0 {
                        Err(Error::NoAcknowledge)
                    } else {
                        Err(Error::Abort)
                    }
                }

                /// Get the TX FIFO level
                #[inline]
                pub fn tx_fifo_level(&self) -> u8 {
                    self.regs.ic_txflr.txflr() as u8
                }

                /// Get the RX FIFO level
                #[inline]
                pub fn rx_fifo_level(&self) -> u8 {
                    self.regs.ic_rxflr.rxflr() as u8
                }

                /// Check if TX FIFO is empty
                #[inline]
                pub fn is_tx_empty(&self) -> bool {
                    self.regs.ic_status.tfe()
                }

                /// Check if RX FIFO is not empty
                #[inline]
                pub fn is_rx_not_empty(&self) -> bool {
                    self.regs.ic_status.rfne()
                }

                /// Check if I2C is busy
                #[inline]
                pub fn is_busy(&self) -> bool {
                    self.regs.ic_status.activity()
                }

                /// Get the current baudrate
                #[inline]
                pub fn baudrate(&self) -> u32 {
                    self.baudrate
                }

                /// Get the current mode
                #[inline]
                pub fn mode(&self) -> Mode {
                    self.mode
                }

                // ================================================================
                // Interrupts
                // ================================================================

                // ---- Mask operations (enable/disable) ----

                /// Enable specific interrupts using mask
                pub fn enable_interrupts(&self, mask: u32) {
                    self.regs.ic_intr_mask.set_bits(mask);
                }

                /// Disable specific interrupts using mask
                pub fn disable_interrupts(&self, mask: u32) {
                    self.regs.ic_intr_mask.clr_bits(mask);
                }

                /// Disable all interrupts
                #[inline]
                pub fn disable_all_interrupts(&self) {
                    self.regs.ic_intr_mask.clr_bits(0x1fff);
                }

                // ---- RX FULL interrupt ----

                /// Enable RX FULL interrupt (RX FIFO reaches or exceeds threshold)
                #[inline]
                pub fn enable_rx_full_interrupt(&self) {
                    self.regs.ic_intr_mask.set_m_rx_full();
                }

                /// Disable RX FULL interrupt
                #[inline]
                pub fn disable_rx_full_interrupt(&self) {
                    self.regs.ic_intr_mask.clr_m_rx_full();
                }

                /// Check if RX FULL interrupt is enabled
                #[inline]
                pub fn is_rx_full_interrupt_enabled(&self) -> bool {
                    self.regs.ic_intr_mask.m_rx_full()
                }

                // ---- TX EMPTY interrupt ----

                /// Enable TX EMPTY interrupt (TX FIFO at or below threshold)
                #[inline]
                pub fn enable_tx_empty_interrupt(&self) {
                    self.regs.ic_intr_mask.set_m_tx_empty();
                }

                /// Disable TX EMPTY interrupt
                #[inline]
                pub fn disable_tx_empty_interrupt(&self) {
                    self.regs.ic_intr_mask.clr_m_tx_empty();
                }

                /// Check if TX EMPTY interrupt is enabled
                #[inline]
                pub fn is_tx_empty_interrupt_enabled(&self) -> bool {
                    self.regs.ic_intr_mask.m_tx_empty()
                }

                // ---- TX ABRT interrupt ----

                /// Enable TX abort interrupt
                #[inline]
                pub fn enable_tx_abort_interrupt(&self) {
                    self.regs.ic_intr_mask.set_m_tx_abrt();
                }

                /// Disable TX abort interrupt
                #[inline]
                pub fn disable_tx_abort_interrupt(&self) {
                    self.regs.ic_intr_mask.clr_m_tx_abrt();
                }

                // ---- RD REQ interrupt (slave mode) ----

                /// Enable read request interrupt (slave mode: master wants to read)
                #[inline]
                pub fn enable_rd_req_interrupt(&self) {
                    self.regs.ic_intr_mask.set_m_rd_req();
                }

                /// Disable read request interrupt
                #[inline]
                pub fn disable_rd_req_interrupt(&self) {
                    self.regs.ic_intr_mask.clr_m_rd_req();
                }

                // ---- STOP DET interrupt ----

                /// Enable stop detected interrupt
                #[inline]
                pub fn enable_stop_det_interrupt(&self) {
                    self.regs.ic_intr_mask.set_m_stop_det();
                }

                /// Disable stop detected interrupt
                #[inline]
                pub fn disable_stop_det_interrupt(&self) {
                    self.regs.ic_intr_mask.clr_m_stop_det();
                }

                // ---- START DET interrupt ----

                /// Enable start detected interrupt
                #[inline]
                pub fn enable_start_det_interrupt(&self) {
                    self.regs.ic_intr_mask.set_m_start_det();
                }

                /// Disable start detected interrupt
                #[inline]
                pub fn disable_start_det_interrupt(&self) {
                    self.regs.ic_intr_mask.clr_m_start_det();
                }

                // ---- ACTIVITY interrupt ----

                /// Enable activity interrupt
                #[inline]
                pub fn enable_activity_interrupt(&self) {
                    self.regs.ic_intr_mask.set_m_activity();
                }

                /// Disable activity interrupt
                #[inline]
                pub fn disable_activity_interrupt(&self) {
                    self.regs.ic_intr_mask.clr_m_activity();
                }

                // ---- Error interrupts ----

                /// Enable RX overflow interrupt
                #[inline]
                pub fn enable_rx_over_interrupt(&self) {
                    self.regs.ic_intr_mask.set_m_rx_over();
                }

                /// Disable RX overflow interrupt
                #[inline]
                pub fn disable_rx_over_interrupt(&self) {
                    self.regs.ic_intr_mask.clr_m_rx_over();
                }

                /// Enable RX underflow interrupt
                #[inline]
                pub fn enable_rx_under_interrupt(&self) {
                    self.regs.ic_intr_mask.set_m_rx_under();
                }

                /// Disable RX underflow interrupt
                #[inline]
                pub fn disable_rx_under_interrupt(&self) {
                    self.regs.ic_intr_mask.clr_m_rx_under();
                }

                /// Enable TX overflow interrupt
                #[inline]
                pub fn enable_tx_over_interrupt(&self) {
                    self.regs.ic_intr_mask.set_m_tx_over();
                }

                /// Disable TX overflow interrupt
                #[inline]
                pub fn disable_tx_over_interrupt(&self) {
                    self.regs.ic_intr_mask.clr_m_tx_over();
                }

                /// Enable all error interrupts (RX_OVER, RX_UNDER, TX_OVER, TX_ABRT)
                #[inline]
                pub fn enable_error_interrupts(&self) {
                    self.regs.ic_intr_mask.set_m_rx_over();
                    self.regs.ic_intr_mask.set_m_rx_under();
                    self.regs.ic_intr_mask.set_m_tx_over();
                    self.regs.ic_intr_mask.set_m_tx_abrt();
                }

                /// Disable all error interrupts
                #[inline]
                pub fn disable_error_interrupts(&self) {
                    self.regs.ic_intr_mask.clr_m_rx_over();
                    self.regs.ic_intr_mask.clr_m_rx_under();
                    self.regs.ic_intr_mask.clr_m_tx_over();
                    self.regs.ic_intr_mask.clr_m_tx_abrt();
                }

                // ---- Status checking ----

                /// Get raw interrupt status (before masking)
                #[inline]
                pub fn raw_interrupt_status(&self) -> u32 {
                    self.regs.ic_raw_intr_stat.read_bits()
                }

                /// Get masked interrupt status (after masking)
                #[inline]
                pub fn interrupt_status(&self) -> u32 {
                    self.regs.ic_intr_stat.read_bits()
                }

                /// Check if RX FULL interrupt is pending (masked)
                #[inline]
                pub fn is_rx_full_interrupt_pending(&self) -> bool {
                    self.regs.ic_intr_stat.r_rx_full()
                }

                /// Check if TX EMPTY interrupt is pending (masked)
                #[inline]
                pub fn is_tx_empty_interrupt_pending(&self) -> bool {
                    self.regs.ic_intr_stat.r_tx_empty()
                }

                /// Check if TX ABRT interrupt is pending (masked)
                #[inline]
                pub fn is_tx_abort_interrupt_pending(&self) -> bool {
                    self.regs.ic_intr_stat.r_tx_abrt()
                }

                /// Check if RD REQ interrupt is pending (masked, slave mode)
                #[inline]
                pub fn is_rd_req_interrupt_pending(&self) -> bool {
                    self.regs.ic_intr_stat.r_rd_req()
                }

                /// Check if STOP DET interrupt is pending (masked)
                #[inline]
                pub fn is_stop_det_interrupt_pending(&self) -> bool {
                    self.regs.ic_intr_stat.r_stop_det()
                }

                /// Check if START DET interrupt is pending (masked)
                #[inline]
                pub fn is_start_det_interrupt_pending(&self) -> bool {
                    self.regs.ic_intr_stat.r_start_det()
                }

                // ---- Clearing interrupts ----

                /// Clear all interrupts by reading CLR_INTR register
                pub fn clear_all_interrupts(&self) {
                    let _ = self.regs.ic_clr_intr.read_bits();
                }

                /// Clear RX UNDER interrupt
                #[inline]
                pub fn clear_rx_under_interrupt(&self) {
                    let _ = self.regs.ic_clr_rx_under.read_bits();
                }

                /// Clear RX OVER interrupt
                #[inline]
                pub fn clear_rx_over_interrupt(&self) {
                    let _ = self.regs.ic_clr_rx_over.read_bits();
                }

                /// Clear TX OVER interrupt
                #[inline]
                pub fn clear_tx_over_interrupt(&self) {
                    let _ = self.regs.ic_clr_tx_over.read_bits();
                }

                /// Clear RD REQ interrupt
                #[inline]
                pub fn clear_rd_req_interrupt(&self) {
                    let _ = self.regs.ic_clr_rd_req.read_bits();
                }

                /// Clear TX ABRT interrupt
                #[inline]
                pub fn clear_tx_abort_interrupt(&self) {
                    let _ = self.regs.ic_clr_tx_abrt.read_bits();
                }

                /// Clear ACTIVITY interrupt
                #[inline]
                pub fn clear_activity_interrupt(&self) {
                    let _ = self.regs.ic_clr_activity.read_bits();
                }

                /// Clear STOP DET interrupt
                #[inline]
                pub fn clear_stop_det_interrupt(&self) {
                    let _ = self.regs.ic_clr_stop_det.read_bits();
                }

                /// Clear START DET interrupt
                #[inline]
                pub fn clear_start_det_interrupt(&self) {
                    let _ = self.regs.ic_clr_start_det.read_bits();
                }

                /// Clear GEN CALL interrupt
                #[inline]
                pub fn clear_gen_call_interrupt(&self) {
                    let _ = self.regs.ic_clr_gen_call.read_bits();
                }

                /// Clear RESTART DET interrupt
                #[inline]
                pub fn clear_restart_det_interrupt(&self) {
                    let _ = self.regs.ic_clr_restart_det.read_bits();
                }
            }

            impl Default for $I2cName {
                fn default() -> Self {
                    Self::new()
                }
            }
        }
    };
}

// ============================================================================
// Generate I2C0 and I2C1
// ============================================================================

impl_i2c!(I2c0, 0, pico_rs_pac::I2C0, i2c0, Peripheral::I2C0);
impl_i2c!(I2c1, 1, pico_rs_pac::I2C1, i2c1, Peripheral::I2C1);
