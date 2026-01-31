//! XIP_AUX
//! Auxiliary DMA access to XIP FIFOs, via fast AHB bus access
//!
//! This peripheral supports atomic SET/CLR/XOR register aliases.

#![allow(dead_code)]

pub const BASE: u32 = 0x50500000;

/// STREAM Register
///
/// Read the XIP stream FIFO (fast bus access to XIP_CTRL_STREAM_FIFO)
///
/// Address offset: 0x0000
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **STREAM** `[31:0]` (RO)
#[repr(transparent)]
pub struct Stream {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Stream {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Stream {
    // ======== Field Constants ========

    /// **STREAM**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const STREAM_OFFSET: u32 = 0;
    pub const STREAM_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **STREAM** `[31:0]` (RO)
    #[inline(always)]
    pub fn stream(&self) -> u32 {
        (self.value.read_bits() & Self::STREAM_MASK) >> Self::STREAM_OFFSET
    }

}

/// STREAM value type for batch read/modify/write operations
///
/// Use `stream.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `stream.write_val(val)`.
#[derive(Clone, Copy)]
pub struct StreamVal {
    bits: u32,
}

impl StreamVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **STREAM** `[31:0]`
    #[inline(always)]
    pub const fn stream(&self) -> u32 {
        (self.bits & Stream::STREAM_MASK) >> Stream::STREAM_OFFSET
    }

}

impl Default for StreamVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Stream {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> StreamVal {
        StreamVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: StreamVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(StreamVal) -> StreamVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// QMI_DIRECT_TX Register
///
/// Write to the QMI direct-mode TX FIFO (fast bus access to QMI_DIRECT_TX)
///
/// Address offset: 0x0004
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NOPUSH** `[20]` (WO)
///
///   Inhibit the RX FIFO push that would correspond to this TX FIFO entry.
///   Useful to avoid garbage appearing in the RX FIFO when pushing the command at the beginning of a SPI transfer.
///
/// - **OE** `[19]` (WO)
///
///   Output enable (active-high). For single width (SPI), this field is ignored, and SD0 is always set to output, with SD1 always set to input.
///   For dual and quad width (DSPI/QSPI), this sets whether the relevant SDx pads are set to output whilst transferring this FIFO record. In this case the command/address should have OE set, and the data transfer should have OE set or clear depending on the direction of the transfer.
///
/// - **DWIDTH** `[18]` (WO)
///
///   Data width. If 0, hardware will transmit the 8 LSBs of the DIRECT_TX DATA field, and return an 8-bit value in the 8 LSBs of DIRECT_RX. If 1, the full 16-bit width is used. 8-bit and 16-bit transfers can be mixed freely.
///
/// - **IWIDTH** `[17:16]` (WO)
///
///   Configure whether this FIFO record is transferred with single/dual/quad interface width (0/1/2). Different widths can be mixed freely.
///
/// - **DATA** `[15:0]` (WO)
///
///   Data pushed here will be clocked out falling edges of SCK (or before the very first rising edge of SCK, if this is the first pulse). For each byte clocked out, the interface will simultaneously sample one byte, on rising edges of SCK, and push this to the DIRECT_RX FIFO.
///   For 16-bit data, the least-significant byte is transmitted first.
#[repr(transparent)]
pub struct QmiDirectTx {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for QmiDirectTx {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl QmiDirectTx {
    // ======== Field Constants ========

    /// **NOPUSH**
    ///
    /// Inhibit the RX FIFO push that would correspond to this TX FIFO entry.
    /// Useful to avoid garbage appearing in the RX FIFO when pushing the command at the beginning of a SPI transfer.
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const NOPUSH_OFFSET: u32 = 20;
    pub const NOPUSH_MASK: u32 = 0x00100000;
    pub const NOPUSH: u32 = 1 << 20;

    /// **OE**
    ///
    /// Output enable (active-high). For single width (SPI), this field is ignored, and SD0 is always set to output, with SD1 always set to input.
    /// For dual and quad width (DSPI/QSPI), this sets whether the relevant SDx pads are set to output whilst transferring this FIFO record. In this case the command/address should have OE set, and the data transfer should have OE set or clear depending on the direction of the transfer.
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const OE_OFFSET: u32 = 19;
    pub const OE_MASK: u32 = 0x00080000;
    pub const OE: u32 = 1 << 19;

    /// **DWIDTH**
    ///
    /// Data width. If 0, hardware will transmit the 8 LSBs of the DIRECT_TX DATA field, and return an 8-bit value in the 8 LSBs of DIRECT_RX. If 1, the full 16-bit width is used. 8-bit and 16-bit transfers can be mixed freely.
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const DWIDTH_OFFSET: u32 = 18;
    pub const DWIDTH_MASK: u32 = 0x00040000;
    pub const DWIDTH: u32 = 1 << 18;

    /// **IWIDTH**
    ///
    /// Configure whether this FIFO record is transferred with single/dual/quad interface width (0/1/2). Different widths can be mixed freely.
    ///
    /// Bits `[17:16]`, mask `0x00030000`
    pub const IWIDTH_OFFSET: u32 = 16;
    pub const IWIDTH_MASK: u32 = 0x00030000;
    /// S = 0
    /// Single width
    pub const IWIDTH_S: u32 = 0 << 16;
    pub const IWIDTH_S_VAL: u32 = 0;
    /// D = 1
    /// Dual width
    pub const IWIDTH_D: u32 = 1 << 16;
    pub const IWIDTH_D_VAL: u32 = 1;
    /// Q = 2
    /// Quad width
    pub const IWIDTH_Q: u32 = 2 << 16;
    pub const IWIDTH_Q_VAL: u32 = 2;

    /// **DATA**
    ///
    /// Data pushed here will be clocked out falling edges of SCK (or before the very first rising edge of SCK, if this is the first pulse). For each byte clocked out, the interface will simultaneously sample one byte, on rising edges of SCK, and push this to the DIRECT_RX FIFO.
    /// For 16-bit data, the least-significant byte is transmitted first.
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const DATA_OFFSET: u32 = 0;
    pub const DATA_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// Trigger **NOPUSH** (WO)
    ///
    /// Inhibit the RX FIFO push that would correspond to this TX FIFO entry.
    /// Useful to avoid garbage appearing in the RX FIFO when pushing the command at the beginning of a SPI transfer.
    #[inline(always)]
    pub fn trigger_nopush(&self) {
        self.value.write_bits(Self::NOPUSH_MASK);
    }

    /// Trigger **OE** (WO)
    ///
    /// Output enable (active-high). For single width (SPI), this field is ignored, and SD0 is always set to output, with SD1 always set to input.
    /// For dual and quad width (DSPI/QSPI), this sets whether the relevant SDx pads are set to output whilst transferring this FIFO record. In this case the command/address should have OE set, and the data transfer should have OE set or clear depending on the direction of the transfer.
    #[inline(always)]
    pub fn trigger_oe(&self) {
        self.value.write_bits(Self::OE_MASK);
    }

    /// Trigger **DWIDTH** (WO)
    ///
    /// Data width. If 0, hardware will transmit the 8 LSBs of the DIRECT_TX DATA field, and return an 8-bit value in the 8 LSBs of DIRECT_RX. If 1, the full 16-bit width is used. 8-bit and 16-bit transfers can be mixed freely.
    #[inline(always)]
    pub fn trigger_dwidth(&self) {
        self.value.write_bits(Self::DWIDTH_MASK);
    }

    /// Write **IWIDTH** `[17:16]` (WO)
    ///
    /// Configure whether this FIFO record is transferred with single/dual/quad interface width (0/1/2). Different widths can be mixed freely.
    #[inline(always)]
    pub fn write_iwidth(&self, value: u32) {
        self.value.write_bits((value << Self::IWIDTH_OFFSET) & Self::IWIDTH_MASK);
    }

    /// Write **DATA** `[15:0]` (WO)
    ///
    /// Data pushed here will be clocked out falling edges of SCK (or before the very first rising edge of SCK, if this is the first pulse). For each byte clocked out, the interface will simultaneously sample one byte, on rising edges of SCK, and push this to the DIRECT_RX FIFO.
    /// For 16-bit data, the least-significant byte is transmitted first.
    #[inline(always)]
    pub fn write_data(&self, value: u32) {
        self.value.write_bits((value << Self::DATA_OFFSET) & Self::DATA_MASK);
    }

}

/// QMI_DIRECT_TX value type for batch read/modify/write operations
///
/// Use `qmidirecttx.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `qmidirecttx.write_val(val)`.
#[derive(Clone, Copy)]
pub struct QmiDirectTxVal {
    bits: u32,
}

impl QmiDirectTxVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Set **NOPUSH** `[20:20]`
    ///
    /// Inhibit the RX FIFO push that would correspond to this TX FIFO entry.
    /// Useful to avoid garbage appearing in the RX FIFO when pushing the command at the beginning of a SPI transfer.
    #[inline(always)]
    pub const fn with_nopush(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | QmiDirectTx::NOPUSH_MASK
            } else {
                self.bits & !QmiDirectTx::NOPUSH_MASK
            }
        }
    }

    /// Set **OE** `[19:19]`
    ///
    /// Output enable (active-high). For single width (SPI), this field is ignored, and SD0 is always set to output, with SD1 always set to input.
    /// For dual and quad width (DSPI/QSPI), this sets whether the relevant SDx pads are set to output whilst transferring this FIFO record. In this case the command/address should have OE set, and the data transfer should have OE set or clear depending on the direction of the transfer.
    #[inline(always)]
    pub const fn with_oe(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | QmiDirectTx::OE_MASK
            } else {
                self.bits & !QmiDirectTx::OE_MASK
            }
        }
    }

    /// Set **DWIDTH** `[18:18]`
    ///
    /// Data width. If 0, hardware will transmit the 8 LSBs of the DIRECT_TX DATA field, and return an 8-bit value in the 8 LSBs of DIRECT_RX. If 1, the full 16-bit width is used. 8-bit and 16-bit transfers can be mixed freely.
    #[inline(always)]
    pub const fn with_dwidth(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | QmiDirectTx::DWIDTH_MASK
            } else {
                self.bits & !QmiDirectTx::DWIDTH_MASK
            }
        }
    }

    /// Set **IWIDTH** `[17:16]`
    ///
    /// Configure whether this FIFO record is transferred with single/dual/quad interface width (0/1/2). Different widths can be mixed freely.
    #[inline(always)]
    pub const fn with_iwidth(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !QmiDirectTx::IWIDTH_MASK) | ((value << QmiDirectTx::IWIDTH_OFFSET) & QmiDirectTx::IWIDTH_MASK)
        }
    }

    /// Set **DATA** `[15:0]`
    ///
    /// Data pushed here will be clocked out falling edges of SCK (or before the very first rising edge of SCK, if this is the first pulse). For each byte clocked out, the interface will simultaneously sample one byte, on rising edges of SCK, and push this to the DIRECT_RX FIFO.
    /// For 16-bit data, the least-significant byte is transmitted first.
    #[inline(always)]
    pub const fn with_data(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !QmiDirectTx::DATA_MASK) | ((value << QmiDirectTx::DATA_OFFSET) & QmiDirectTx::DATA_MASK)
        }
    }

}

impl Default for QmiDirectTxVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl QmiDirectTx {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> QmiDirectTxVal {
        QmiDirectTxVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: QmiDirectTxVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(QmiDirectTxVal) -> QmiDirectTxVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// QMI_DIRECT_RX Register
///
/// Read from the QMI direct-mode RX FIFO (fast bus access to QMI_DIRECT_RX)
///
/// Address offset: 0x0008
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **QMI_DIRECT_RX** `[15:0]` (RO)
///
///   With each byte clocked out on the serial interface, one byte will simultaneously be clocked in, and will appear in this FIFO. The serial interface will stall when this FIFO is full, to avoid dropping data.
///   When 16-bit data is pushed into the TX FIFO, the corresponding RX FIFO push will also contain 16 bits of data. The least-significant byte is the first one received.
#[repr(transparent)]
pub struct QmiDirectRx {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for QmiDirectRx {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl QmiDirectRx {
    // ======== Field Constants ========

    /// **QMI_DIRECT_RX**
    ///
    /// With each byte clocked out on the serial interface, one byte will simultaneously be clocked in, and will appear in this FIFO. The serial interface will stall when this FIFO is full, to avoid dropping data.
    /// When 16-bit data is pushed into the TX FIFO, the corresponding RX FIFO push will also contain 16 bits of data. The least-significant byte is the first one received.
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const QMI_DIRECT_RX_OFFSET: u32 = 0;
    pub const QMI_DIRECT_RX_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **QMI_DIRECT_RX** `[15:0]` (RO)
    ///
    /// With each byte clocked out on the serial interface, one byte will simultaneously be clocked in, and will appear in this FIFO. The serial interface will stall when this FIFO is full, to avoid dropping data.
    /// When 16-bit data is pushed into the TX FIFO, the corresponding RX FIFO push will also contain 16 bits of data. The least-significant byte is the first one received.
    #[inline(always)]
    pub fn qmi_direct_rx(&self) -> u32 {
        (self.value.read_bits() & Self::QMI_DIRECT_RX_MASK) >> Self::QMI_DIRECT_RX_OFFSET
    }

}

/// QMI_DIRECT_RX value type for batch read/modify/write operations
///
/// Use `qmidirectrx.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `qmidirectrx.write_val(val)`.
#[derive(Clone, Copy)]
pub struct QmiDirectRxVal {
    bits: u32,
}

impl QmiDirectRxVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **QMI_DIRECT_RX** `[15:0]`
    ///
    /// With each byte clocked out on the serial interface, one byte will simultaneously be clocked in, and will appear in this FIFO. The serial interface will stall when this FIFO is full, to avoid dropping data.
    /// When 16-bit data is pushed into the TX FIFO, the corresponding RX FIFO push will also contain 16 bits of data. The least-significant byte is the first one received.
    #[inline(always)]
    pub const fn qmi_direct_rx(&self) -> u32 {
        (self.bits & QmiDirectRx::QMI_DIRECT_RX_MASK) >> QmiDirectRx::QMI_DIRECT_RX_OFFSET
    }

}

impl Default for QmiDirectRxVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl QmiDirectRx {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> QmiDirectRxVal {
        QmiDirectRxVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: QmiDirectRxVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(QmiDirectRxVal) -> QmiDirectRxVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// Read the XIP stream FIFO (fast bus access to XIP_CTRL_STREAM_FIFO)
    pub stream: Stream,
    /// 0x0004
    /// Write to the QMI direct-mode TX FIFO (fast bus access to QMI_DIRECT_TX)
    pub qmi_direct_tx: QmiDirectTx,
    /// 0x0008
    /// Read from the QMI direct-mode RX FIFO (fast bus access to QMI_DIRECT_RX)
    pub qmi_direct_rx: QmiDirectRx,
}

