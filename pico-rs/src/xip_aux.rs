//! XIP_AUX (XIP Auxiliary) HAL for RP2350
//!
//! This module provides fast AHB bus access to XIP FIFOs and QMI direct mode.
//!
//! ## Features
//!
//! - Fast stream FIFO access for DMA
//! - QMI direct mode TX/RX FIFO access
//!
//! ## Example
//!
//! ```ignore
//! use pico_rs::xip_aux::XipAux;
//!
//! let xip_aux = XipAux::new();
//!
//! // Read from stream FIFO
//! let data = xip_aux.read_stream();
//!
//! // Direct QMI access
//! xip_aux.write_qmi_direct(0x1234);
//! let rx = xip_aux.read_qmi_direct();
//! ```

use pico_rs_pac::XIP_AUX;

/// QMI direct mode interface width
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum InterfaceWidth {
    /// Single width (SPI)
    Single = 0,
    /// Dual width
    Dual = 1,
    /// Quad width
    Quad = 2,
}

/// XIP_AUX peripheral wrapper
///
/// Provides fast access to XIP streaming and QMI direct mode.
pub struct XipAux {
    regs: &'static pico_rs_pac::xip_aux::RegisterBlock,
}

impl XipAux {
    /// Create a new XipAux instance
    #[inline]
    pub fn new() -> Self {
        Self {
            regs: unsafe { &*XIP_AUX::PTR },
        }
    }

    /// Get reference to the register block
    #[inline]
    pub fn regs(&self) -> &pico_rs_pac::xip_aux::RegisterBlock {
        self.regs
    }

    // ========================================================================
    // Stream FIFO
    // ========================================================================

    /// Read from the XIP stream FIFO
    ///
    /// This provides fast bus access to XIP_CTRL_STREAM_FIFO.
    #[inline]
    pub fn read_stream(&self) -> u32 {
        self.regs.stream.read_bits()
    }

    /// Get the stream FIFO address for DMA
    #[inline]
    pub fn stream_fifo_address(&self) -> u32 {
        XIP_AUX::PTR as u32
    }

    // ========================================================================
    // QMI Direct Mode TX
    // ========================================================================

    /// Write to the QMI direct-mode TX FIFO
    ///
    /// # Arguments
    /// * `data` - Data to transmit (8 or 16 bits depending on dwidth)
    /// * `width` - Interface width (single/dual/quad)
    /// * `dwidth` - Data width (false = 8 bits, true = 16 bits)
    /// * `output_enable` - Enable output (for dual/quad modes)
    /// * `no_push` - Inhibit RX FIFO push for this transfer
    pub fn write_qmi_tx(
        &self,
        data: u16,
        width: InterfaceWidth,
        dwidth: bool,
        output_enable: bool,
        no_push: bool,
    ) {
        let value = (data as u32)
            | ((width as u32) << 16)
            | (if dwidth { 1 << 18 } else { 0 })
            | (if output_enable { 1 << 19 } else { 0 })
            | (if no_push { 1 << 20 } else { 0 });

        self.regs.qmi_direct_tx.write_bits(value);
    }

    /// Write data to QMI direct TX (simple 8-bit SPI mode)
    #[inline]
    pub fn write_qmi_direct(&self, data: u8) {
        self.regs.qmi_direct_tx.write_bits(data as u32 | (1 << 19)); // OE=1 for TX
    }

    /// Write data to QMI direct TX with no RX push
    #[inline]
    pub fn write_qmi_direct_no_rx(&self, data: u8) {
        self.regs
            .qmi_direct_tx
            .write_bits(data as u32 | (1 << 19) | (1 << 20)); // OE=1, NOPUSH=1
    }

    /// Get the QMI TX FIFO address for DMA
    #[inline]
    pub fn qmi_tx_address(&self) -> u32 {
        XIP_AUX::PTR as u32 + 0x04
    }

    // ========================================================================
    // QMI Direct Mode RX
    // ========================================================================

    /// Read from the QMI direct-mode RX FIFO
    #[inline]
    pub fn read_qmi_rx(&self) -> u16 {
        self.regs.qmi_direct_rx.read_bits() as u16
    }

    /// Read the raw RX FIFO value (includes level information)
    #[inline]
    pub fn read_qmi_rx_raw(&self) -> u32 {
        self.regs.qmi_direct_rx.read_bits()
    }

    /// Read byte from QMI direct RX
    #[inline]
    pub fn read_qmi_direct(&self) -> u8 {
        self.regs.qmi_direct_rx.read_bits() as u8
    }

    /// Get the QMI RX FIFO address for DMA
    #[inline]
    pub fn qmi_rx_address(&self) -> u32 {
        XIP_AUX::PTR as u32 + 0x08
    }
}

impl Default for XipAux {
    fn default() -> Self {
        Self::new()
    }
}
