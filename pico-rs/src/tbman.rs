//! TBMAN (Testbench Manager) HAL for RP2350
//!
//! This module provides read-only access to the platform identification
//! register used for simulation testbenches.
//!
//! ## Features
//!
//! - Platform identification (ASIC, FPGA, simulation)
//!
//! ## Example
//!
//! ```ignore
//! use pico_rs::tbman::Tbman;
//!
//! let tbman = Tbman::new();
//!
//! // Check platform type
//! if tbman.is_asic() {
//!     // Running on real hardware
//! } else if tbman.is_fpga() {
//!     // Running on FPGA
//! } else if tbman.is_hdl_sim() {
//!     // Running in simulation
//! }
//! ```

use pico_rs_pac::TBMAN;

/// TBMAN peripheral wrapper
///
/// Provides read-only access to platform identification.
pub struct Tbman {
    regs: &'static pico_rs_pac::tbman::RegisterBlock,
}

impl Tbman {
    /// Create a new Tbman instance
    #[inline]
    pub fn new() -> Self {
        Self {
            regs: unsafe { &*TBMAN::PTR },
        }
    }

    /// Get reference to the register block
    #[inline]
    pub fn regs(&self) -> &pico_rs_pac::tbman::RegisterBlock {
        self.regs
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
}

impl Default for Tbman {
    fn default() -> Self {
        Self::new()
    }
}
