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
//! use pico_rs_pac::SYSINFO;
//!
//! let sysinfo_pac = unsafe { SYSINFO::steal() };
//! let sysinfo = Sysinfo::init(sysinfo_pac);
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

use crate::reset::{self, Peripheral};

/// SYSINFO peripheral wrapper
///
/// Provides read-only access to system information registers.
/// Owns the PAC SYSINFO peripheral to enforce proper ownership semantics.
pub struct Sysinfo {
    sysinfo: pico_rs_pac::SYSINFO,
}

impl Sysinfo {
    /// Create a new Sysinfo instance without initialization
    ///
    /// # Arguments
    /// * `sysinfo` - The SYSINFO peripheral from PAC
    #[inline]
    pub fn new(sysinfo: pico_rs_pac::SYSINFO) -> Self {
        Self { sysinfo }
    }

    /// Initialize the SYSINFO peripheral
    ///
    /// This brings the SYSINFO peripheral out of reset.
    ///
    /// # Arguments
    /// * `sysinfo` - The SYSINFO peripheral from PAC
    pub fn init(sysinfo: pico_rs_pac::SYSINFO) -> Self {
        reset::unreset_block_wait(Peripheral::SYSINFO);
        Self::new(sysinfo)
    }

    /// Get reference to the underlying register block
    ///
    /// # Safety
    /// Direct register access can bypass the HAL's safety guarantees.
    /// The caller must ensure operations do not conflict with the HAL state.
    #[inline]
    pub unsafe fn regs(&self) -> &pico_rs_pac::sysinfo::RegisterBlock {
        &self.sysinfo
    }

    /// Release the SYSINFO peripheral
    #[inline]
    pub fn release(self) -> pico_rs_pac::SYSINFO {
        self.sysinfo
    }

    // ========================================================================
    // Chip ID
    // ========================================================================

    /// Get the raw chip ID register value
    #[inline]
    pub fn chip_id(&self) -> u32 {
        self.sysinfo.chip_id.read_bits()
    }

    /// Get the chip revision (bits 31:28)
    #[inline]
    pub fn revision(&self) -> u8 {
        self.sysinfo.chip_id.revision() as u8
    }

    /// Get the chip part number (bits 27:12)
    #[inline]
    pub fn part(&self) -> u16 {
        self.sysinfo.chip_id.part() as u16
    }

    /// Get the manufacturer ID (bits 11:1)
    #[inline]
    pub fn manufacturer(&self) -> u16 {
        self.sysinfo.chip_id.manufacturer() as u16
    }

    // ========================================================================
    // Package Selection
    // ========================================================================

    /// Get the raw package selection register value
    #[inline]
    pub fn package_sel(&self) -> u32 {
        self.sysinfo.package_sel.read_bits()
    }

    // ========================================================================
    // Platform
    // ========================================================================

    /// Get the raw platform register value
    #[inline]
    pub fn platform(&self) -> u32 {
        self.sysinfo.platform.read_bits()
    }

    /// Check if running on an ASIC (real chip)
    #[inline]
    pub fn is_asic(&self) -> bool {
        self.sysinfo.platform.asic()
    }

    /// Check if running on an FPGA
    #[inline]
    pub fn is_fpga(&self) -> bool {
        self.sysinfo.platform.fpga()
    }

    /// Check if running in HDL simulation
    #[inline]
    pub fn is_hdl_sim(&self) -> bool {
        self.sysinfo.platform.hdlsim()
    }

    // ========================================================================
    // Git Reference
    // ========================================================================

    /// Get the chip source git hash
    ///
    /// This is used to identify the chip version.
    #[inline]
    pub fn gitref(&self) -> u32 {
        self.sysinfo.gitref_rp2350.read_bits()
    }
}
