//! SYSINFO (System Information) HAL for RP2350
//!
//! This module provides read-only access to chip identification and platform
//! information for the RP2350.
//!
//! ## Features
//!
//! - Chip ID (JEDEC JEP-106 compliant)
//! - Package selection
//! - Platform identification (ASIC, FPGA, simulation)
//! - Chip git reference
//!
//! ## Example
//!
//! ```ignore
//! use pico_rs::sysinfo::Sysinfo;
//!
//! let sysinfo = Sysinfo::new();
//!
//! // Get chip ID information
//! let chip_id = sysinfo.chip_id();
//! let part = sysinfo.part();
//! let revision = sysinfo.revision();
//!
//! // Check platform
//! if sysinfo.is_asic() {
//!     // Running on real hardware
//! }
//! ```

use pico_rs_pac::SYSINFO;

/// SYSINFO peripheral wrapper
///
/// Provides read-only access to system information registers.
pub struct Sysinfo {
    regs: &'static pico_rs_pac::sysinfo::RegisterBlock,
}

impl Sysinfo {
    /// Create a new Sysinfo instance
    #[inline]
    pub fn new() -> Self {
        Self {
            regs: unsafe { &*SYSINFO::PTR },
        }
    }

    /// Get reference to the register block
    #[inline]
    pub fn regs(&self) -> &pico_rs_pac::sysinfo::RegisterBlock {
        self.regs
    }

    // ========================================================================
    // Chip ID
    // ========================================================================

    /// Get the raw chip ID register value
    #[inline]
    pub fn chip_id(&self) -> u32 {
        self.regs.chip_id.read_bits()
    }

    /// Get the chip revision (bits 31:28)
    #[inline]
    pub fn revision(&self) -> u8 {
        self.regs.chip_id.revision() as u8
    }

    /// Get the chip part number (bits 27:12)
    #[inline]
    pub fn part(&self) -> u16 {
        self.regs.chip_id.part() as u16
    }

    /// Get the manufacturer ID (bits 11:1)
    #[inline]
    pub fn manufacturer(&self) -> u16 {
        self.regs.chip_id.manufacturer() as u16
    }

    // ========================================================================
    // Package Selection
    // ========================================================================

    /// Get the raw package selection register value
    #[inline]
    pub fn package_sel(&self) -> u32 {
        self.regs.package_sel.read_bits()
    }

    // ========================================================================
    // Platform
    // ========================================================================

    /// Get the raw platform register value
    #[inline]
    pub fn platform(&self) -> u32 {
        self.regs.platform.read_bits()
    }

    /// Check if running on an ASIC (real chip)
    #[inline]
    pub fn is_asic(&self) -> bool {
        self.regs.platform.asic()
    }

    /// Check if running on an FPGA
    #[inline]
    pub fn is_fpga(&self) -> bool {
        self.regs.platform.fpga()
    }

    /// Check if running in HDL simulation
    #[inline]
    pub fn is_hdl_sim(&self) -> bool {
        self.regs.platform.hdlsim()
    }

    // ========================================================================
    // Git Reference
    // ========================================================================

    /// Get the chip source git hash
    ///
    /// This is used to identify the chip version.
    #[inline]
    pub fn gitref(&self) -> u32 {
        self.regs.gitref_rp2350.read_bits()
    }
}

impl Default for Sysinfo {
    fn default() -> Self {
        Self::new()
    }
}
