//! DPRAM (Data Packet RAM) access helpers
//!
//! This module provides safe, type-safe access to the RP2350's USB DPRAM.
//! It abstracts away raw pointer usage and provides a clean API for reading
//! and writing endpoint buffers.
//!
//! ## Memory Layout
//!
//! The USB DPRAM (4KB) is organized as follows:
//! - 0x000-0x007: SETUP packet (8 bytes)
//! - 0x008-0x07F: Endpoint control registers (EP1-EP15, IN and OUT)
//! - 0x080-0x0FF: Buffer control registers (EP0-EP15, IN and OUT)
//! - 0x100-0xFFF: Data buffers (endpoint data)
//!
//! ## Note on volatile access
//!
//! The data buffer area (0x100-0xFFF) is raw memory, not registers. While the PAC
//! provides type-safe register access, the data buffer area requires direct memory
//! access. This module encapsulates that access using `VolatileMemory` to provide
//! consistent volatile semantics similar to the PAC's approach.

use rp2350_pac::USB_DPRAM;

/// DPRAM base address
const DPRAM_BASE: u32 = rp2350_pac::usb_dpram::BASE;

/// Size of DPRAM in bytes
pub const DPRAM_SIZE: usize = 4096;

/// Start offset of data buffer area in DPRAM
pub const DATA_BUFFER_START: u16 = 0x100;

/// Wrapper for volatile memory operations
///
/// This struct provides volatile read/write access to memory regions,
/// similar to how the PAC provides access to registers but for raw memory.
/// It encapsulates the unsafe volatile operations in a safe API.
struct VolatileMemory {
    base: *mut u8,
}

impl VolatileMemory {
    /// Create a new volatile memory accessor
    const fn new(base: *mut u8) -> Self {
        Self { base }
    }

    /// Write a single byte with volatile semantics
    #[inline(always)]
    fn write_byte(&self, offset: usize, value: u8) {
        // Safety: We trust that the offset is within bounds (enforced by callers)
        // and that the base address is valid for the DPRAM region.
        unsafe {
            let ptr = self.base.add(offset);
            core::ptr::write_volatile(ptr, value);
        }
    }

    /// Read a single byte with volatile semantics
    #[inline(always)]
    fn read_byte(&self, offset: usize) -> u8 {
        // Safety: Same as write_byte
        unsafe {
            let ptr = self.base.add(offset);
            core::ptr::read_volatile(ptr)
        }
    }

    /// Write a u32 word with volatile semantics (word-aligned)
    #[inline(always)]
    fn write_word(&self, word_offset: usize, value: u32) {
        // Safety: Caller ensures word alignment and bounds
        unsafe {
            let ptr = (self.base as *mut u32).add(word_offset);
            core::ptr::write_volatile(ptr, value);
        }
    }

    /// Write multiple bytes with volatile semantics
    fn write_bytes(&self, offset: usize, data: &[u8]) {
        for (i, &byte) in data.iter().enumerate() {
            self.write_byte(offset + i, byte);
        }
    }

    /// Read multiple bytes with volatile semantics
    fn read_bytes(&self, offset: usize, data: &mut [u8]) {
        for (i, byte) in data.iter_mut().enumerate() {
            *byte = self.read_byte(offset + i);
        }
    }
}

// Safety: VolatileMemory is designed for single-threaded embedded use
unsafe impl Send for VolatileMemory {}
unsafe impl Sync for VolatileMemory {}

/// DPRAM accessor for safe memory operations
pub struct Dpram {
    mem: VolatileMemory,
}

impl Dpram {
    /// Obtain a handle to the USB DPRAM.
    ///
    /// # Safety
    /// This is safe because DPRAM has a fixed address and we only create one accessor.
    pub const fn steal() -> Self {
        Self {
            mem: VolatileMemory::new(DPRAM_BASE as *mut u8),
        }
    }

    /// Get the DPRAM register block for control/buffer control registers
    pub fn regs(&self) -> &'static rp2350_pac::usb_dpram::RegisterBlock {
        unsafe { &*USB_DPRAM::PTR }
    }

    /// Clear entire DPRAM to zeros
    ///
    /// This uses word-aligned writes for efficiency.
    pub fn clear(&self) {
        for i in 0..(DPRAM_SIZE / 4) {
            self.mem.write_word(i, 0);
        }
    }

    /// Write data to a specific offset in DPRAM
    ///
    /// # Arguments
    /// * `offset` - Byte offset from DPRAM base
    /// * `data` - Data to write
    ///
    /// # Panics
    /// Panics if the write would exceed DPRAM bounds.
    #[inline]
    pub fn write_buffer(&self, offset: u16, data: &[u8]) {
        debug_assert!(
            (offset as usize) + data.len() <= DPRAM_SIZE,
            "DPRAM write out of bounds"
        );

        self.mem.write_bytes(offset as usize, data);
    }

    /// Read data from a specific offset in DPRAM
    ///
    /// # Arguments
    /// * `offset` - Byte offset from DPRAM base
    /// * `data` - Buffer to read into
    ///
    /// # Panics
    /// Panics if the read would exceed DPRAM bounds.
    #[inline]
    pub fn read_buffer(&self, offset: u16, data: &mut [u8]) {
        debug_assert!(
            (offset as usize) + data.len() <= DPRAM_SIZE,
            "DPRAM read out of bounds"
        );

        self.mem.read_bytes(offset as usize, data);
    }

    /// Copy data from source to DPRAM buffer
    ///
    /// This writes data to DPRAM with volatile semantics and ensures
    /// all writes complete with a DSB barrier.
    #[inline]
    pub fn copy_to_buffer(&self, offset: u16, data: &[u8]) {
        debug_assert!(
            (offset as usize) + data.len() <= DPRAM_SIZE,
            "DPRAM copy out of bounds"
        );

        self.mem.write_bytes(offset as usize, data);

        // Ensure all writes complete before returning
        rp2350_rt::asm::dsb();
    }

    /// Copy data from DPRAM buffer to destination
    ///
    /// This reads data from DPRAM with volatile semantics.
    #[inline]
    pub fn copy_from_buffer(&self, offset: u16, data: &mut [u8]) {
        debug_assert!(
            (offset as usize) + data.len() <= DPRAM_SIZE,
            "DPRAM copy out of bounds"
        );

        self.mem.read_bytes(offset as usize, data);
    }
}

impl Default for Dpram {
    fn default() -> Self {
        Self::steal()
    }
}

// Safety: Dpram is designed for single-threaded embedded use with volatile access
unsafe impl Send for Dpram {}
unsafe impl Sync for Dpram {}
