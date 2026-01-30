//! XIP_CTRL (XIP Cache Controller) HAL for RP2350
//!
//! This module provides access to the QSPI flash execute-in-place cache
//! controller for the RP2350.
//!
//! ## Features
//!
//! - Cache enable/disable for Secure and Non-secure accesses
//! - Cache hit/access counters for performance monitoring
//! - Stream FIFO for bulk flash reads
//! - Cache power control
//! - Memory window write enable
//!
//! ## Example
//!
//! ```ignore
//! use pico_rs::xip_ctrl::XipCtrl;
//!
//! let xip = XipCtrl::new();
//!
//! // Enable cache for secure accesses
//! xip.enable_secure();
//!
//! // Check cache statistics
//! let hits = xip.hit_count();
//! let accesses = xip.access_count();
//! let hit_rate = hits as f32 / accesses as f32;
//! ```

use pico_rs_pac::XIP_CTRL;

/// XIP_CTRL peripheral wrapper
///
/// Provides access to the XIP cache controller.
pub struct XipCtrl {
    regs: &'static pico_rs_pac::xip_ctrl::RegisterBlock,
}

impl XipCtrl {
    /// Create a new XipCtrl instance
    #[inline]
    pub fn new() -> Self {
        Self {
            regs: unsafe { &*XIP_CTRL::PTR },
        }
    }

    /// Get reference to the register block
    #[inline]
    pub fn regs(&self) -> &pico_rs_pac::xip_ctrl::RegisterBlock {
        self.regs
    }

    // ========================================================================
    // Cache Enable/Disable
    // ========================================================================

    /// Enable cache for Secure accesses
    #[inline]
    pub fn enable_secure(&self) {
        self.regs.ctrl.set_en_secure();
    }

    /// Disable cache for Secure accesses
    #[inline]
    pub fn disable_secure(&self) {
        self.regs.ctrl.clr_en_secure();
    }

    /// Check if cache is enabled for Secure accesses
    #[inline]
    pub fn is_secure_enabled(&self) -> bool {
        self.regs.ctrl.en_secure()
    }

    /// Enable cache for Non-secure accesses
    #[inline]
    pub fn enable_nonsecure(&self) {
        self.regs.ctrl.set_en_nonsecure();
    }

    /// Disable cache for Non-secure accesses
    #[inline]
    pub fn disable_nonsecure(&self) {
        self.regs.ctrl.clr_en_nonsecure();
    }

    /// Check if cache is enabled for Non-secure accesses
    #[inline]
    pub fn is_nonsecure_enabled(&self) -> bool {
        self.regs.ctrl.en_nonsecure()
    }

    /// Enable cache for both Secure and Non-secure accesses
    #[inline]
    pub fn enable(&self) {
        self.enable_secure();
        self.enable_nonsecure();
    }

    /// Disable cache for both Secure and Non-secure accesses
    #[inline]
    pub fn disable(&self) {
        self.disable_secure();
        self.disable_nonsecure();
    }

    // ========================================================================
    // Power Control
    // ========================================================================

    /// Power down the cache memories
    ///
    /// Cache memories retain state but cannot be read or written.
    #[inline]
    pub fn power_down(&self) {
        self.regs.ctrl.set_power_down();
    }

    /// Power up the cache memories
    #[inline]
    pub fn power_up(&self) {
        self.regs.ctrl.clr_power_down();
    }

    /// Check if cache is powered down
    #[inline]
    pub fn is_powered_down(&self) -> bool {
        self.regs.ctrl.power_down()
    }

    // ========================================================================
    // Access Control
    // ========================================================================

    /// Enable writes to XIP memory window 0 (0x10000000-0x10FFFFFF)
    #[inline]
    pub fn enable_writable_m0(&self) {
        self.regs.ctrl.set_writable_m0();
    }

    /// Disable writes to XIP memory window 0
    #[inline]
    pub fn disable_writable_m0(&self) {
        self.regs.ctrl.clr_writable_m0();
    }

    /// Check if writes are enabled for memory window 0
    #[inline]
    pub fn is_writable_m0(&self) -> bool {
        self.regs.ctrl.writable_m0()
    }

    /// Enable writes to XIP memory window 1 (0x11000000-0x11FFFFFF)
    #[inline]
    pub fn enable_writable_m1(&self) {
        self.regs.ctrl.set_writable_m1();
    }

    /// Disable writes to XIP memory window 1
    #[inline]
    pub fn disable_writable_m1(&self) {
        self.regs.ctrl.clr_writable_m1();
    }

    /// Check if writes are enabled for memory window 1
    #[inline]
    pub fn is_writable_m1(&self) -> bool {
        self.regs.ctrl.writable_m1()
    }

    // ========================================================================
    // Cache Way Splitting
    // ========================================================================

    /// Enable split ways mode
    ///
    /// When enabled, Secure accesses use way 0 and Non-secure accesses use way 1.
    #[inline]
    pub fn enable_split_ways(&self) {
        self.regs.ctrl.set_split_ways();
    }

    /// Disable split ways mode
    #[inline]
    pub fn disable_split_ways(&self) {
        self.regs.ctrl.clr_split_ways();
    }

    /// Check if split ways mode is enabled
    #[inline]
    pub fn is_split_ways(&self) -> bool {
        self.regs.ctrl.split_ways()
    }

    // ========================================================================
    // Access Restrictions
    // ========================================================================

    /// Block Secure uncached accesses
    #[inline]
    pub fn block_uncached_secure(&self, block: bool) {
        if block {
            self.regs.ctrl.set_no_uncached_sec();
        } else {
            self.regs.ctrl.clr_no_uncached_sec();
        }
    }

    /// Block Non-secure uncached accesses
    #[inline]
    pub fn block_uncached_nonsecure(&self, block: bool) {
        if block {
            self.regs.ctrl.set_no_uncached_nonsec();
        } else {
            self.regs.ctrl.clr_no_uncached_nonsec();
        }
    }

    /// Block Secure untranslated accesses
    #[inline]
    pub fn block_untranslated_secure(&self, block: bool) {
        if block {
            self.regs.ctrl.set_no_untranslated_sec();
        } else {
            self.regs.ctrl.clr_no_untranslated_sec();
        }
    }

    /// Block Non-secure untranslated accesses
    #[inline]
    pub fn block_untranslated_nonsecure(&self, block: bool) {
        if block {
            self.regs.ctrl.set_no_untranslated_nonsec();
        } else {
            self.regs.ctrl.clr_no_untranslated_nonsec();
        }
    }

    // ========================================================================
    // Maintenance
    // ========================================================================

    /// Enable Non-secure access to cache maintenance
    #[inline]
    pub fn enable_maint_nonsecure(&self) {
        self.regs.ctrl.set_maint_nonsec();
    }

    /// Disable Non-secure access to cache maintenance
    #[inline]
    pub fn disable_maint_nonsecure(&self) {
        self.regs.ctrl.clr_maint_nonsec();
    }

    // ========================================================================
    // Statistics
    // ========================================================================

    /// Get cache hit count
    #[inline]
    pub fn hit_count(&self) -> u32 {
        self.regs.ctr_hit.read_bits()
    }

    /// Get cache access count
    #[inline]
    pub fn access_count(&self) -> u32 {
        self.regs.ctr_acc.read_bits()
    }

    /// Reset cache hit counter
    #[inline]
    pub fn reset_hit_counter(&self) {
        self.regs.ctr_hit.write_bits(0);
    }

    /// Reset cache access counter
    #[inline]
    pub fn reset_access_counter(&self) {
        self.regs.ctr_acc.write_bits(0);
    }

    /// Reset both counters
    #[inline]
    pub fn reset_counters(&self) {
        self.reset_hit_counter();
        self.reset_access_counter();
    }

    // ========================================================================
    // Status
    // ========================================================================

    /// Check if the cache FIFO is empty
    #[inline]
    pub fn is_fifo_empty(&self) -> bool {
        self.regs.stat.fifo_empty()
    }

    /// Check if the cache FIFO is full
    #[inline]
    pub fn is_fifo_full(&self) -> bool {
        self.regs.stat.fifo_full()
    }

    // ========================================================================
    // Stream FIFO
    // ========================================================================

    /// Set the stream address for bulk reads
    #[inline]
    pub fn set_stream_address(&self, addr: u32) {
        self.regs.stream_addr.write_bits(addr);
    }

    /// Get the current stream address
    #[inline]
    pub fn stream_address(&self) -> u32 {
        self.regs.stream_addr.read_bits()
    }

    /// Set the stream transfer count (in 32-bit words)
    #[inline]
    pub fn set_stream_count(&self, count: u32) {
        self.regs.stream_ctr.write_bits(count);
    }

    /// Get the remaining stream transfer count
    #[inline]
    pub fn stream_count(&self) -> u32 {
        self.regs.stream_ctr.read_bits()
    }

    /// Read from the stream FIFO
    #[inline]
    pub fn read_stream(&self) -> u32 {
        self.regs.stream_fifo.read_bits()
    }

    /// Start a stream transfer
    ///
    /// # Arguments
    /// * `addr` - Start address in flash
    /// * `count` - Number of 32-bit words to transfer
    pub fn start_stream(&self, addr: u32, count: u32) {
        self.set_stream_address(addr);
        self.set_stream_count(count);
    }

    /// Get the stream FIFO address for DMA
    #[inline]
    pub fn stream_fifo_address(&self) -> u32 {
        XIP_CTRL::PTR as u32 + 0x1C // STREAM_FIFO offset
    }
}

impl Default for XipCtrl {
    fn default() -> Self {
        Self::new()
    }
}
