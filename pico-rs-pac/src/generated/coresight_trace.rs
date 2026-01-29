//! CORESIGHT_TRACE - Coresight block - RP specific registers

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
/// - **TRACE_CAPTURE_FIFO_OVERFLOW** [1] (RW): This status flag is set high when trace data has been dropped due to the FI...
/// - **TRACE_CAPTURE_FIFO_FLUSH** [0] (RW): Set to 1 to continuously hold the trace FIFO in a flushed state and prevent...
#[repr(transparent)]
pub struct CtrlStatus {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for CtrlStatus {}
unsafe impl Sync for CtrlStatus {}

impl CtrlStatus {
    // ======== Field Constants ========

    /// TRACE_CAPTURE_FIFO_OVERFLOW - This status flag is set high when trace data has been dropped due to the FI...
    /// Bits [1:1], mask 0x00000002
    pub const TRACE_CAPTURE_FIFO_OVERFLOW_OFFSET: u32 = 1;
    pub const TRACE_CAPTURE_FIFO_OVERFLOW_MASK: u32 = 0x00000002;
    pub const TRACE_CAPTURE_FIFO_OVERFLOW: u32 = 1 << 1;

    /// TRACE_CAPTURE_FIFO_FLUSH - Set to 1 to continuously hold the trace FIFO in a flushed state and prevent...
    /// Bits [0:0], mask 0x00000001
    pub const TRACE_CAPTURE_FIFO_FLUSH_OFFSET: u32 = 0;
    pub const TRACE_CAPTURE_FIFO_FLUSH_MASK: u32 = 0x00000001;
    pub const TRACE_CAPTURE_FIFO_FLUSH: u32 = 1 << 0;

    // ======== Basic Access Methods ========

    /// Read the raw register value as u32
    #[inline(always)]
    pub fn read_bits(&self) -> u32 {
        unsafe { core::ptr::read_volatile(self.value.get()) }
    }

    /// Write a raw u32 value to the register
    #[inline(always)]
    pub fn write_bits(&self, value: u32) {
        unsafe { core::ptr::write_volatile(self.value.get(), value) }
    }

    /// Modify the raw register value
    #[inline(always)]
    pub fn modify_bits<F: FnOnce(u32) -> u32>(&self, f: F) {
        self.write_bits(f(self.read_bits()));
    }

    /// Atomic SET - sets bits that are 1 (hardware alias at +0x2000)
    #[inline(always)]
    pub fn set_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x2000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic CLR - clears bits that are 1 (hardware alias at +0x3000)
    #[inline(always)]
    pub fn clr_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x3000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic XOR - toggles bits that are 1 (hardware alias at +0x1000)
    #[inline(always)]
    pub fn xor_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x1000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    // ======== Field Accessors ========

    /// **TRACE_CAPTURE_FIFO_OVERFLOW** [1:1]: This status flag is set high when trace data has been dropped due to the FI...
    #[inline(always)]
    pub fn trace_capture_fifo_overflow(&self) -> bool {
        (self.read_bits() & Self::TRACE_CAPTURE_FIFO_OVERFLOW_MASK) != 0
    }

    /// Set TRACE_CAPTURE_FIFO_OVERFLOW bit
    #[inline(always)]
    pub fn set_trace_capture_fifo_overflow(&self) {
        self.set_bits(Self::TRACE_CAPTURE_FIFO_OVERFLOW_MASK);
    }

    /// Clear TRACE_CAPTURE_FIFO_OVERFLOW bit
    #[inline(always)]
    pub fn clr_trace_capture_fifo_overflow(&self) {
        self.clr_bits(Self::TRACE_CAPTURE_FIFO_OVERFLOW_MASK);
    }

    /// **TRACE_CAPTURE_FIFO_FLUSH** [0:0]: Set to 1 to continuously hold the trace FIFO in a flushed state and prevent...
    #[inline(always)]
    pub fn trace_capture_fifo_flush(&self) -> bool {
        (self.read_bits() & Self::TRACE_CAPTURE_FIFO_FLUSH_MASK) != 0
    }

    /// Set TRACE_CAPTURE_FIFO_FLUSH bit
    #[inline(always)]
    pub fn set_trace_capture_fifo_flush(&self) {
        self.set_bits(Self::TRACE_CAPTURE_FIFO_FLUSH_MASK);
    }

    /// Clear TRACE_CAPTURE_FIFO_FLUSH bit
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

    /// Read TRACE_CAPTURE_FIFO_OVERFLOW [1:1]: This status flag is set high when trace data has been dropped due to the FI...
    #[inline(always)]
    pub const fn trace_capture_fifo_overflow(&self) -> bool {
        (self.bits & CtrlStatus::TRACE_CAPTURE_FIFO_OVERFLOW_MASK) != 0
    }

    /// Read TRACE_CAPTURE_FIFO_FLUSH [0:0]: Set to 1 to continuously hold the trace FIFO in a flushed state and prevent...
    #[inline(always)]
    pub const fn trace_capture_fifo_flush(&self) -> bool {
        (self.bits & CtrlStatus::TRACE_CAPTURE_FIFO_FLUSH_MASK) != 0
    }

    /// Set TRACE_CAPTURE_FIFO_OVERFLOW [1:1]: This status flag is set high when trace data has been dropped due to the FI...
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

    /// Set TRACE_CAPTURE_FIFO_FLUSH [0:0]: Set to 1 to continuously hold the trace FIFO in a flushed state and prevent...
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
    pub fn read(&self) -> CtrlStatusVal {
        CtrlStatusVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: CtrlStatusVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(CtrlStatusVal) -> CtrlStatusVal>(&self, f: F) {
        self.write(f(self.read()))
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
/// - **RDATA** [31:0] (RO): Read from an 8 x 32-bit FIFO containing trace data captured from the TPIU. ...
#[repr(transparent)]
pub struct TraceCaptureFifo {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for TraceCaptureFifo {}
unsafe impl Sync for TraceCaptureFifo {}

impl TraceCaptureFifo {
    // ======== Field Constants ========

    /// RDATA - Read from an 8 x 32-bit FIFO containing trace data captured from the TPIU. ...
    /// Bits [31:0], mask 0xffffffff
    pub const RDATA_OFFSET: u32 = 0;
    pub const RDATA_MASK: u32 = 0xffffffff;

    // ======== Basic Access Methods ========

    /// Read the raw register value as u32
    #[inline(always)]
    pub fn read_bits(&self) -> u32 {
        unsafe { core::ptr::read_volatile(self.value.get()) }
    }

    /// Write a raw u32 value to the register
    #[inline(always)]
    pub fn write_bits(&self, value: u32) {
        unsafe { core::ptr::write_volatile(self.value.get(), value) }
    }

    /// Modify the raw register value
    #[inline(always)]
    pub fn modify_bits<F: FnOnce(u32) -> u32>(&self, f: F) {
        self.write_bits(f(self.read_bits()));
    }

    /// Atomic SET - sets bits that are 1 (hardware alias at +0x2000)
    #[inline(always)]
    pub fn set_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x2000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic CLR - clears bits that are 1 (hardware alias at +0x3000)
    #[inline(always)]
    pub fn clr_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x3000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic XOR - toggles bits that are 1 (hardware alias at +0x1000)
    #[inline(always)]
    pub fn xor_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x1000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    // ======== Field Accessors ========

    /// **RDATA** [31:0]: Read from an 8 x 32-bit FIFO containing trace data captured from the TPIU. ...
    #[inline(always)]
    pub fn rdata(&self) -> u32 {
        (self.read_bits() & Self::RDATA_MASK) >> Self::RDATA_OFFSET
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

    /// Read RDATA [31:0]: Read from an 8 x 32-bit FIFO containing trace data captured from the TPIU. ...
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
    pub fn read(&self) -> TraceCaptureFifoVal {
        TraceCaptureFifoVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: TraceCaptureFifoVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(TraceCaptureFifoVal) -> TraceCaptureFifoVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000 - Control and status register
    pub ctrl_status: CtrlStatus,
    /// 0x0004 - FIFO for trace data captured from the TPIU
    pub trace_capture_fifo: TraceCaptureFifo,
}

