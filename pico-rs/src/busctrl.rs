//! BUSCTRL (Bus Control) HAL for RP2350
//!
//! This module provides access to the bus fabric control signals
//! and performance counters for monitoring bus activity.
//!
//! ## Features
//!
//! - Bus arbitration priority control
//! - 4 performance counters for bus activity monitoring
//! - Configurable event selection for each counter
//!
//! ## Example
//!
//! ```ignore
//! use pico_rs::busctrl::{Busctrl, PerfEvent};
//!
//! let busctrl = Busctrl::steal();
//!
//! // Enable performance counters
//! busctrl.enable_perf_counters(true);
//!
//! // Configure counter 0 to count PROC0 access events
//! busctrl.set_perfsel0(PerfEvent::SiobProc0Access);
//!
//! // Read counter value
//! let count = busctrl.perfctr0();
//! ```

use pico_rs_pac::BUSCTRL;

/// Performance counter event selection
///
/// Events that can be counted by the bus fabric performance counters.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u32)]
pub enum PerfEvent {
    /// SIOB PROC1 stall upstream
    SiobProc1StallUpstream = 0,
    /// SIOB PROC1 stall downstream
    SiobProc1StallDownstream = 1,
    /// SIOB PROC1 access contested
    SiobProc1AccessContested = 2,
    /// SIOB PROC1 access
    SiobProc1Access = 3,
    /// SIOB PROC0 stall upstream
    SiobProc0StallUpstream = 4,
    /// SIOB PROC0 stall downstream
    SiobProc0StallDownstream = 5,
    /// SIOB PROC0 access contested
    SiobProc0AccessContested = 6,
    /// SIOB PROC0 access
    SiobProc0Access = 7,
    /// APB stall upstream
    ApbStallUpstream = 8,
    /// APB stall downstream
    ApbStallDownstream = 9,
    /// APB access contested
    ApbAccessContested = 10,
    /// APB access
    ApbAccess = 11,
    /// Fastperi stall upstream
    FastperiStallUpstream = 12,
    /// Fastperi stall downstream
    FastperiStallDownstream = 13,
    /// Fastperi access contested
    FastperiAccessContested = 14,
    /// Fastperi access
    FastperiAccess = 15,
    /// SRAM9 stall upstream
    Sram9StallUpstream = 16,
    /// SRAM9 stall downstream
    Sram9StallDownstream = 17,
    /// SRAM9 access contested
    Sram9AccessContested = 18,
    /// SRAM9 access
    Sram9Access = 19,
    /// SRAM8 stall upstream
    Sram8StallUpstream = 20,
    /// SRAM8 stall downstream
    Sram8StallDownstream = 21,
    /// SRAM8 access contested
    Sram8AccessContested = 22,
    /// SRAM8 access
    Sram8Access = 23,
    /// SRAM7 stall upstream
    Sram7StallUpstream = 24,
    /// SRAM7 stall downstream
    Sram7StallDownstream = 25,
    /// SRAM7 access contested
    Sram7AccessContested = 26,
    /// SRAM7 access
    Sram7Access = 27,
    /// SRAM6 stall upstream
    Sram6StallUpstream = 28,
    /// SRAM6 stall downstream
    Sram6StallDownstream = 29,
    /// SRAM6 access contested
    Sram6AccessContested = 30,
    /// No event (always 0)
    None = 31,
}

/// BUSCTRL peripheral wrapper
///
/// Provides access to bus fabric control and performance counters.
pub struct Busctrl {
    regs: pico_rs_pac::BUSCTRL,
}

impl Busctrl {
    /// Obtain a handle to the BUSCTRL hardware peripheral.
    #[inline]
    pub fn steal() -> Self {
        Self {
            regs: unsafe { BUSCTRL::steal() },
        }
    }

    /// Get reference to the register block
    #[inline]
    pub fn regs(&self) -> &pico_rs_pac::busctrl::RegisterBlock {
        &self.regs
    }

    // ========================================================================
    // Bus Priority
    // ========================================================================

    /// Set PROC0 to high priority
    #[inline]
    pub fn set_proc0_high_priority(&self, high: bool) {
        if high {
            self.regs.bus_priority.set_proc0();
        } else {
            self.regs.bus_priority.clr_proc0();
        }
    }

    /// Set PROC1 to high priority
    #[inline]
    pub fn set_proc1_high_priority(&self, high: bool) {
        if high {
            self.regs.bus_priority.set_proc1();
        } else {
            self.regs.bus_priority.clr_proc1();
        }
    }

    /// Set DMA read to high priority
    #[inline]
    pub fn set_dma_r_high_priority(&self, high: bool) {
        if high {
            self.regs.bus_priority.set_dma_r();
        } else {
            self.regs.bus_priority.clr_dma_r();
        }
    }

    /// Set DMA write to high priority
    #[inline]
    pub fn set_dma_w_high_priority(&self, high: bool) {
        if high {
            self.regs.bus_priority.set_dma_w();
        } else {
            self.regs.bus_priority.clr_dma_w();
        }
    }

    /// Check if PROC0 has high priority
    #[inline]
    pub fn is_proc0_high_priority(&self) -> bool {
        self.regs.bus_priority.proc0()
    }

    /// Check if PROC1 has high priority
    #[inline]
    pub fn is_proc1_high_priority(&self) -> bool {
        self.regs.bus_priority.proc1()
    }

    /// Check if DMA read has high priority
    #[inline]
    pub fn is_dma_r_high_priority(&self) -> bool {
        self.regs.bus_priority.dma_r()
    }

    /// Check if DMA write has high priority
    #[inline]
    pub fn is_dma_w_high_priority(&self) -> bool {
        self.regs.bus_priority.dma_w()
    }

    /// Check if bus priority has been acknowledged by all arbiters
    #[inline]
    pub fn is_priority_ack(&self) -> bool {
        self.regs.bus_priority_ack.bus_priority_ack()
    }

    // ========================================================================
    // Performance Counters
    // ========================================================================

    /// Enable or disable all performance counters
    #[inline]
    pub fn enable_perf_counters(&self, enable: bool) {
        if enable {
            self.regs.perfctr_en.set_perfctr_en();
        } else {
            self.regs.perfctr_en.clr_perfctr_en();
        }
    }

    /// Check if performance counters are enabled
    #[inline]
    pub fn is_perf_counters_enabled(&self) -> bool {
        self.regs.perfctr_en.perfctr_en()
    }

    // ======== Counter 0 ========

    /// Get performance counter 0 value
    #[inline]
    pub fn perfctr0(&self) -> u32 {
        self.regs.perfctr0.perfctr0()
    }

    /// Clear performance counter 0
    #[inline]
    pub fn clear_perfctr0(&self) {
        self.regs.perfctr0.write_perfctr0(0);
    }

    /// Set performance counter 0 event selection
    #[inline]
    pub fn set_perfsel0(&self, event: PerfEvent) {
        self.regs.perfsel0.write_perfsel0(event as u32);
    }

    /// Get performance counter 0 event selection
    #[inline]
    pub fn perfsel0(&self) -> u32 {
        self.regs.perfsel0.perfsel0()
    }

    // ======== Counter 1 ========

    /// Get performance counter 1 value
    #[inline]
    pub fn perfctr1(&self) -> u32 {
        self.regs.perfctr1.perfctr1()
    }

    /// Clear performance counter 1
    #[inline]
    pub fn clear_perfctr1(&self) {
        self.regs.perfctr1.write_perfctr1(0);
    }

    /// Set performance counter 1 event selection
    #[inline]
    pub fn set_perfsel1(&self, event: PerfEvent) {
        self.regs.perfsel1.write_perfsel1(event as u32);
    }

    /// Get performance counter 1 event selection
    #[inline]
    pub fn perfsel1(&self) -> u32 {
        self.regs.perfsel1.perfsel1()
    }

    // ======== Counter 2 ========

    /// Get performance counter 2 value
    #[inline]
    pub fn perfctr2(&self) -> u32 {
        self.regs.perfctr2.perfctr2()
    }

    /// Clear performance counter 2
    #[inline]
    pub fn clear_perfctr2(&self) {
        self.regs.perfctr2.write_perfctr2(0);
    }

    /// Set performance counter 2 event selection
    #[inline]
    pub fn set_perfsel2(&self, event: PerfEvent) {
        self.regs.perfsel2.write_perfsel2(event as u32);
    }

    /// Get performance counter 2 event selection
    #[inline]
    pub fn perfsel2(&self) -> u32 {
        self.regs.perfsel2.perfsel2()
    }

    // ======== Counter 3 ========

    /// Get performance counter 3 value
    #[inline]
    pub fn perfctr3(&self) -> u32 {
        self.regs.perfctr3.perfctr3()
    }

    /// Clear performance counter 3
    #[inline]
    pub fn clear_perfctr3(&self) {
        self.regs.perfctr3.write_perfctr3(0);
    }

    /// Set performance counter 3 event selection
    #[inline]
    pub fn set_perfsel3(&self, event: PerfEvent) {
        self.regs.perfsel3.write_perfsel3(event as u32);
    }

    /// Get performance counter 3 event selection
    #[inline]
    pub fn perfsel3(&self) -> u32 {
        self.regs.perfsel3.perfsel3()
    }

    // ======== Utility Functions ========

    /// Clear all performance counters
    #[inline]
    pub fn clear_all_perfctrs(&self) {
        self.regs.perfctr0.write_perfctr0(0);
        self.regs.perfctr1.write_perfctr1(0);
        self.regs.perfctr2.write_perfctr2(0);
        self.regs.perfctr3.write_perfctr3(0);
    }

    /// Get all performance counter values as an array
    #[inline]
    pub fn all_perfctrs(&self) -> [u32; 4] {
        [
            self.perfctr0(),
            self.perfctr1(),
            self.perfctr2(),
            self.perfctr3(),
        ]
    }
}

impl Default for Busctrl {
    fn default() -> Self {
        Self::steal()
    }
}
