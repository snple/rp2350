//! CORESIGHT_TRACE
//! Coresight block - RP specific registers
//!
//! This peripheral supports atomic SET/CLR/XOR register aliases.

#![allow(dead_code)]

pub const BASE: u32 = 0x50700000;

/// CTRL_STATUS Register
///
/// Control and status register
///
/// Address offset: 0x0000
/// Reset value: 0x00000001
///
/// ## Fields
///
/// - **TRACE_CAPTURE_FIFO_OVERFLOW** `[1]` (RW)
///
///   This status flag is set high when trace data has been dropped due to the FIFO being full at the point trace data was sampled. Write 1 to acknowledge and clear the bit.
///
/// - **TRACE_CAPTURE_FIFO_FLUSH** `[0]` (RW)
///
///   Set to 1 to continuously hold the trace FIFO in a flushed state and prevent overflow.
///   Before clearing this flag, configure and start a DMA channel with the correct DREQ for the TRACE_CAPTURE_FIFO register.
///   Clear this flag to begin sampling trace data, and set once again once the trace capture buffer is full. You must configure the TPIU in order to generate trace packets to be captured, as well as components like the ETM further upstream to generate the event stream propagated to the TPIU.
#[repr(transparent)]
pub struct CtrlStatus {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for CtrlStatus {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CtrlStatus {
    // ======== Field Constants ========

    /// **TRACE_CAPTURE_FIFO_OVERFLOW**
    ///
    /// This status flag is set high when trace data has been dropped due to the FIFO being full at the point trace data was sampled. Write 1 to acknowledge and clear the bit.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TRACE_CAPTURE_FIFO_OVERFLOW_OFFSET: u32 = 1;
    pub const TRACE_CAPTURE_FIFO_OVERFLOW_MASK: u32 = 0x00000002;
    pub const TRACE_CAPTURE_FIFO_OVERFLOW: u32 = 1 << 1;

    /// **TRACE_CAPTURE_FIFO_FLUSH**
    ///
    /// Set to 1 to continuously hold the trace FIFO in a flushed state and prevent overflow.
    /// Before clearing this flag, configure and start a DMA channel with the correct DREQ for the TRACE_CAPTURE_FIFO register.
    /// Clear this flag to begin sampling trace data, and set once again once the trace capture buffer is full. You must configure the TPIU in order to generate trace packets to be captured, as well as components like the ETM further upstream to generate the event stream propagated to the TPIU.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const TRACE_CAPTURE_FIFO_FLUSH_OFFSET: u32 = 0;
    pub const TRACE_CAPTURE_FIFO_FLUSH_MASK: u32 = 0x00000001;
    pub const TRACE_CAPTURE_FIFO_FLUSH: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TRACE_CAPTURE_FIFO_OVERFLOW** `[1]` (RW)
    ///
    /// This status flag is set high when trace data has been dropped due to the FIFO being full at the point trace data was sampled. Write 1 to acknowledge and clear the bit.
    #[inline(always)]
    pub fn trace_capture_fifo_overflow(&self) -> bool {
        (self.value.read_bits() & Self::TRACE_CAPTURE_FIFO_OVERFLOW_MASK) != 0
    }

    /// Set TRACE_CAPTURE_FIFO_OVERFLOW bit atomically (RW)
    #[inline(always)]
    pub fn set_trace_capture_fifo_overflow(&self) {
        self.set_bits(Self::TRACE_CAPTURE_FIFO_OVERFLOW_MASK);
    }

    /// Clear TRACE_CAPTURE_FIFO_OVERFLOW bit atomically (RW)
    #[inline(always)]
    pub fn clr_trace_capture_fifo_overflow(&self) {
        self.clr_bits(Self::TRACE_CAPTURE_FIFO_OVERFLOW_MASK);
    }

    /// **TRACE_CAPTURE_FIFO_FLUSH** `[0]` (RW)
    ///
    /// Set to 1 to continuously hold the trace FIFO in a flushed state and prevent overflow.
    /// Before clearing this flag, configure and start a DMA channel with the correct DREQ for the TRACE_CAPTURE_FIFO register.
    /// Clear this flag to begin sampling trace data, and set once again once the trace capture buffer is full. You must configure the TPIU in order to generate trace packets to be captured, as well as components like the ETM further upstream to generate the event stream propagated to the TPIU.
    #[inline(always)]
    pub fn trace_capture_fifo_flush(&self) -> bool {
        (self.value.read_bits() & Self::TRACE_CAPTURE_FIFO_FLUSH_MASK) != 0
    }

    /// Set TRACE_CAPTURE_FIFO_FLUSH bit atomically (RW)
    #[inline(always)]
    pub fn set_trace_capture_fifo_flush(&self) {
        self.set_bits(Self::TRACE_CAPTURE_FIFO_FLUSH_MASK);
    }

    /// Clear TRACE_CAPTURE_FIFO_FLUSH bit atomically (RW)
    #[inline(always)]
    pub fn clr_trace_capture_fifo_flush(&self) {
        self.clr_bits(Self::TRACE_CAPTURE_FIFO_FLUSH_MASK);
    }

}

/// CTRL_STATUS value type for batch read/modify/write operations
///
/// Use `ctrlstatus.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ctrlstatus.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CtrlStatusVal {
    bits: u32,
}

impl CtrlStatusVal {
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
        Self { bits: 0x00000001 }
    }

    /// Read **TRACE_CAPTURE_FIFO_OVERFLOW** `[1:1]`
    ///
    /// This status flag is set high when trace data has been dropped due to the FIFO being full at the point trace data was sampled. Write 1 to acknowledge and clear the bit.
    #[inline(always)]
    pub const fn trace_capture_fifo_overflow(&self) -> bool {
        (self.bits & CtrlStatus::TRACE_CAPTURE_FIFO_OVERFLOW_MASK) != 0
    }

    /// Read **TRACE_CAPTURE_FIFO_FLUSH** `[0:0]`
    ///
    /// Set to 1 to continuously hold the trace FIFO in a flushed state and prevent overflow.
    /// Before clearing this flag, configure and start a DMA channel with the correct DREQ for the TRACE_CAPTURE_FIFO register.
    /// Clear this flag to begin sampling trace data, and set once again once the trace capture buffer is full. You must configure the TPIU in order to generate trace packets to be captured, as well as components like the ETM further upstream to generate the event stream propagated to the TPIU.
    #[inline(always)]
    pub const fn trace_capture_fifo_flush(&self) -> bool {
        (self.bits & CtrlStatus::TRACE_CAPTURE_FIFO_FLUSH_MASK) != 0
    }

    /// Set **TRACE_CAPTURE_FIFO_OVERFLOW** `[1:1]`
    ///
    /// This status flag is set high when trace data has been dropped due to the FIFO being full at the point trace data was sampled. Write 1 to acknowledge and clear the bit.
    #[inline(always)]
    pub const fn with_trace_capture_fifo_overflow(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CtrlStatus::TRACE_CAPTURE_FIFO_OVERFLOW_MASK
            } else {
                self.bits & !CtrlStatus::TRACE_CAPTURE_FIFO_OVERFLOW_MASK
            }
        }
    }

    /// Set **TRACE_CAPTURE_FIFO_FLUSH** `[0:0]`
    ///
    /// Set to 1 to continuously hold the trace FIFO in a flushed state and prevent overflow.
    /// Before clearing this flag, configure and start a DMA channel with the correct DREQ for the TRACE_CAPTURE_FIFO register.
    /// Clear this flag to begin sampling trace data, and set once again once the trace capture buffer is full. You must configure the TPIU in order to generate trace packets to be captured, as well as components like the ETM further upstream to generate the event stream propagated to the TPIU.
    #[inline(always)]
    pub const fn with_trace_capture_fifo_flush(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CtrlStatus::TRACE_CAPTURE_FIFO_FLUSH_MASK
            } else {
                self.bits & !CtrlStatus::TRACE_CAPTURE_FIFO_FLUSH_MASK
            }
        }
    }

}

impl Default for CtrlStatusVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl CtrlStatus {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CtrlStatusVal {
        CtrlStatusVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CtrlStatusVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CtrlStatusVal) -> CtrlStatusVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// TRACE_CAPTURE_FIFO Register
///
/// FIFO for trace data captured from the TPIU
///
/// Address offset: 0x0004
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **RDATA** `[31:0]` (RO)
///
///   Read from an 8 x 32-bit FIFO containing trace data captured from the TPIU.
///   Hardware pushes to the FIFO on rising edges of clk_sys, when either of the following is true:
///   * TPIU TRACECTL output is low (normal trace data)
///   * TPIU TRACETCL output is high, and TPIU TRACEDATA0 and TRACEDATA1 are both low (trigger packet)
///   These conditions are in accordance with Arm Coresight Architecture Spec v3.0 section D3.3.3: Decoding requirements for Trace Capture Devices
///   The data captured into the FIFO is the full 32-bit TRACEDATA bus output by the TPIU. Note that the TPIU is a DDR output at half of clk_sys, therefore this interface can capture the full 32-bit TPIU DDR output bandwidth as it samples once per active edge of the TPIU output clock.
#[repr(transparent)]
pub struct TraceCaptureFifo {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for TraceCaptureFifo {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TraceCaptureFifo {
    // ======== Field Constants ========

    /// **RDATA**
    ///
    /// Read from an 8 x 32-bit FIFO containing trace data captured from the TPIU.
    /// Hardware pushes to the FIFO on rising edges of clk_sys, when either of the following is true:
    /// * TPIU TRACECTL output is low (normal trace data)
    /// * TPIU TRACETCL output is high, and TPIU TRACEDATA0 and TRACEDATA1 are both low (trigger packet)
    /// These conditions are in accordance with Arm Coresight Architecture Spec v3.0 section D3.3.3: Decoding requirements for Trace Capture Devices
    /// The data captured into the FIFO is the full 32-bit TRACEDATA bus output by the TPIU. Note that the TPIU is a DDR output at half of clk_sys, therefore this interface can capture the full 32-bit TPIU DDR output bandwidth as it samples once per active edge of the TPIU output clock.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const RDATA_OFFSET: u32 = 0;
    pub const RDATA_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **RDATA** `[31:0]` (RO)
    ///
    /// Read from an 8 x 32-bit FIFO containing trace data captured from the TPIU.
    /// Hardware pushes to the FIFO on rising edges of clk_sys, when either of the following is true:
    /// * TPIU TRACECTL output is low (normal trace data)
    /// * TPIU TRACETCL output is high, and TPIU TRACEDATA0 and TRACEDATA1 are both low (trigger packet)
    /// These conditions are in accordance with Arm Coresight Architecture Spec v3.0 section D3.3.3: Decoding requirements for Trace Capture Devices
    /// The data captured into the FIFO is the full 32-bit TRACEDATA bus output by the TPIU. Note that the TPIU is a DDR output at half of clk_sys, therefore this interface can capture the full 32-bit TPIU DDR output bandwidth as it samples once per active edge of the TPIU output clock.
    #[inline(always)]
    pub fn rdata(&self) -> u32 {
        (self.value.read_bits() & Self::RDATA_MASK) >> Self::RDATA_OFFSET
    }

}

/// TRACE_CAPTURE_FIFO value type for batch read/modify/write operations
///
/// Use `tracecapturefifo.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `tracecapturefifo.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TraceCaptureFifoVal {
    bits: u32,
}

impl TraceCaptureFifoVal {
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

    /// Read **RDATA** `[31:0]`
    ///
    /// Read from an 8 x 32-bit FIFO containing trace data captured from the TPIU.
    /// Hardware pushes to the FIFO on rising edges of clk_sys, when either of the following is true:
    /// * TPIU TRACECTL output is low (normal trace data)
    /// * TPIU TRACETCL output is high, and TPIU TRACEDATA0 and TRACEDATA1 are both low (trigger packet)
    /// These conditions are in accordance with Arm Coresight Architecture Spec v3.0 section D3.3.3: Decoding requirements for Trace Capture Devices
    /// The data captured into the FIFO is the full 32-bit TRACEDATA bus output by the TPIU. Note that the TPIU is a DDR output at half of clk_sys, therefore this interface can capture the full 32-bit TPIU DDR output bandwidth as it samples once per active edge of the TPIU output clock.
    #[inline(always)]
    pub const fn rdata(&self) -> u32 {
        (self.bits & TraceCaptureFifo::RDATA_MASK) >> TraceCaptureFifo::RDATA_OFFSET
    }

}

impl Default for TraceCaptureFifoVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl TraceCaptureFifo {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TraceCaptureFifoVal {
        TraceCaptureFifoVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TraceCaptureFifoVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TraceCaptureFifoVal) -> TraceCaptureFifoVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// Control and status register
    pub ctrl_status: CtrlStatus,
    /// 0x0004
    /// FIFO for trace data captured from the TPIU
    pub trace_capture_fifo: TraceCaptureFifo,
}

