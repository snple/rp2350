//! XIP (Execute-In-Place) HAL for RP2350
//!
//! The XIP block translates AHB bus accesses into QSPI flash reads and
//! provides the flash cache plus a background streaming interface. This
//! module wraps [`rp2350_pac::XIP_CTRL`] with cache control, cache
//! maintenance and streaming helpers.
//!
//! Memory-mapped read/write *format* configuration lives in [`crate::qspi`];
//! this module controls the *cache* layered on top of it.
//!
//! ## Features
//!
//! - Cache enable / disable / power-down
//! - Per-window write enable (for PSRAM on CS0/CS1)
//! - Cache flush (invalidate) and clean (write-back)
//! - Streaming read of a linear flash range
//!
//! ## Example
//!
//! ```ignore
//! use rp2350::qspi::Window;
//! use rp2350::xip::Xip;
//! use rp2350_pac::XIP_CTRL;
//!
//! let xip = Xip::init(unsafe { XIP_CTRL::steal() });
//!
//! // Allow writes to memory window 0 (e.g. PSRAM).
//! xip.set_writable(Window::M0, true);
//!
//! // After reprogramming flash, drop stale cache lines.
//! xip.flush_cache();
//! ```

use crate::Steal;
use crate::qspi::Window;

/// Base of the cache-maintenance address window. Writing a byte here performs
/// the maintenance operation encoded in the low address bits.
const XIP_MAINTENANCE_BASE: u32 = 0x1800_0000;

/// Size of the cacheable XIP address space (`XIP_END - XIP_BASE`).
const XIP_ADDRESS_SPACE_SIZE: u32 = 0x0400_0000;

/// XIP cache size (16 KiB).
const XIP_CACHE_SIZE: u32 = 16 * 1024;

/// XIP cache line size (8 bytes).
const XIP_CACHE_LINE_SIZE: u32 = 8;

/// Cache-maintenance operation codes (RP2350 datasheet, §cache maintenance).
const OP_INVALIDATE_BY_SET_WAY: u32 = 0;
const OP_CLEAN_BY_SET_WAY: u32 = 1;

/// XIP peripheral wrapper.
///
/// Owns the PAC `XIP_CTRL` peripheral.
pub struct Xip {
    xip_ctrl: rp2350_pac::XIP_CTRL,
}

impl Xip {
    /// Take ownership of the `XIP_CTRL` peripheral.
    ///
    /// No reset is performed: the XIP cache is configured by the bootrom and
    /// is actively servicing flash reads while code runs from flash.
    #[inline]
    pub fn init(xip_ctrl: rp2350_pac::XIP_CTRL) -> Self {
        Self { xip_ctrl }
    }

    /// Obtain a handle without ownership checks.
    ///
    /// # Safety
    ///
    /// The caller must ensure no other handle to `XIP_CTRL` is live.
    #[inline]
    pub unsafe fn steal() -> Steal<Self> {
        Steal(Self {
            xip_ctrl: unsafe { rp2350_pac::XIP_CTRL::steal() },
        })
    }

    /// Get a reference to the underlying register block.
    ///
    /// # Safety
    /// Direct register access can bypass the HAL's safety guarantees.
    #[inline]
    pub unsafe fn regs(&self) -> &rp2350_pac::xip_ctrl::RegisterBlock {
        &self.xip_ctrl
    }

    /// Release the `XIP_CTRL` peripheral.
    #[inline]
    pub fn release(self) -> rp2350_pac::XIP_CTRL {
        self.xip_ctrl
    }

    // ========================================================================
    // Cache control
    // ========================================================================

    /// Enable the cache for both Secure and Non-secure accesses.
    #[inline]
    pub fn enable_cache(&self) {
        self.xip_ctrl.ctrl.set_en_secure();
        self.xip_ctrl.ctrl.set_en_nonsecure();
    }

    /// Disable the cache; all XIP accesses go straight to the QSPI device.
    #[inline]
    pub fn disable_cache(&self) {
        self.xip_ctrl.ctrl.clr_en_secure();
        self.xip_ctrl.ctrl.clr_en_nonsecure();
    }

    /// Whether the cache is enabled for Secure accesses.
    #[inline]
    pub fn cache_enabled(&self) -> bool {
        self.xip_ctrl.ctrl.en_secure()
    }

    /// Power down the cache memories (retains state, reduces leakage).
    #[inline]
    pub fn power_down_cache(&self) {
        self.xip_ctrl.ctrl.set_power_down();
    }

    /// Whether the cache memories are powered down.
    #[inline]
    pub fn cache_powered_down(&self) -> bool {
        self.xip_ctrl.ctrl.power_down()
    }

    /// Enable or disable writes to a memory window.
    ///
    /// XIP memory is read-only by default; enable writes only when a
    /// writable device (e.g. PSRAM) is attached to the window.
    #[inline]
    pub fn set_writable(&self, window: Window, enable: bool) {
        match window {
            Window::M0 => {
                if enable {
                    self.xip_ctrl.ctrl.set_writable_m0();
                } else {
                    self.xip_ctrl.ctrl.clr_writable_m0();
                }
            }
            Window::M1 => {
                if enable {
                    self.xip_ctrl.ctrl.set_writable_m1();
                } else {
                    self.xip_ctrl.ctrl.clr_writable_m1();
                }
            }
        }
    }

    /// Whether writes are enabled for a memory window.
    #[inline]
    pub fn writable(&self, window: Window) -> bool {
        match window {
            Window::M0 => self.xip_ctrl.ctrl.writable_m0(),
            Window::M1 => self.xip_ctrl.ctrl.writable_m1(),
        }
    }

    /// Route Secure and Non-secure cached accesses into separate cache ways.
    ///
    /// A full cache flush is required when changing this; perform the flush
    /// while `split_ways` is disabled so both ways are reachable.
    #[inline]
    pub fn set_split_ways(&self, enable: bool) {
        if enable {
            self.xip_ctrl.ctrl.set_split_ways();
        } else {
            self.xip_ctrl.ctrl.clr_split_ways();
        }
    }

    // ========================================================================
    // Cache maintenance
    // ========================================================================

    /// Invalidate the entire cache.
    ///
    /// Call after reprogramming flash so subsequent reads see the new data.
    /// Any pending write data held in the cache is **lost** — call
    /// [`clean_cache`](Self::clean_cache) first if needed.
    pub fn flush_cache(&self) {
        self.maintain(
            XIP_ADDRESS_SPACE_SIZE - XIP_CACHE_SIZE,
            XIP_CACHE_SIZE,
            OP_INVALIDATE_BY_SET_WAY,
        );
    }

    /// Write back (clean) all dirty cache lines.
    ///
    /// Note: on RP2350 this also effectively invalidates every line due to the
    /// RP2350-E11 workaround; the next access to each line will miss.
    pub fn clean_cache(&self) {
        self.maintain(
            XIP_ADDRESS_SPACE_SIZE - XIP_CACHE_SIZE,
            XIP_CACHE_SIZE,
            OP_CLEAN_BY_SET_WAY,
        );
    }

    /// Apply a cache-maintenance operation over `size` bytes starting at
    /// `start` (both line-aligned), then drain the maintenance pipeline.
    fn maintain(&self, start: u32, size: u32, op: u32) {
        let end = start + size;
        let mut offset = start;
        while offset < end {
            // SAFETY: the maintenance window is a fixed MMIO region; a byte
            // write there performs the operation encoded in the low bits.
            unsafe {
                core::ptr::write_volatile((XIP_MAINTENANCE_BASE + offset + op) as *mut u8, 0);
            }
            offset += XIP_CACHE_LINE_SIZE;
        }
        // Ensure subsequent accesses observe the maintained cache state.
        unsafe {
            core::arch::asm!("dsb sy", "isb sy", options(nomem, nostack, preserves_flags));
        }
    }

    // ========================================================================
    // Streaming
    // ========================================================================

    /// Start a background streaming read of `count` words from `addr`.
    ///
    /// The stream uses flash idle cycles; poll [`stream_fifo_empty`] and
    /// drain with [`read_stream`]. Use [`stop_stream`] to abort.
    #[inline]
    pub fn start_stream(&self, addr: u32, count: u32) {
        self.xip_ctrl.stream_addr.write_stream_addr(addr);
        self.xip_ctrl.stream_ctr.write_stream_ctr(count);
    }

    /// Halt an in-progress stream and discard in-flight data.
    #[inline]
    pub fn stop_stream(&self) {
        self.xip_ctrl.stream_ctr.write_stream_ctr(0);
    }

    /// Whether the stream FIFO is completely empty.
    #[inline]
    pub fn stream_fifo_empty(&self) -> bool {
        self.xip_ctrl.stat.fifo_empty()
    }

    /// Whether the stream FIFO is completely full.
    #[inline]
    pub fn stream_fifo_full(&self) -> bool {
        self.xip_ctrl.stat.fifo_full()
    }

    /// Pop one word from the stream FIFO (normal bus access).
    #[inline]
    pub fn read_stream(&self) -> u32 {
        self.xip_ctrl.stream_fifo.stream_fifo()
    }

    /// Address of the XIP_AUX stream FIFO, for fast-bus (DMA) reads that avoid
    /// stalling on other XIP traffic.
    #[inline]
    pub fn stream_aux_address(&self) -> u32 {
        rp2350_pac::XIP_AUX::PTR as u32
    }
}
