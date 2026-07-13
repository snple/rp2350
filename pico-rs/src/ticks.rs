//! TICKS (Tick Generator) HAL for RP2350
//!
//! This module provides access to the tick generators for various system
//! components including processors, timers, and watchdog.
//!
//! The tick generators derive a 1µs tick from the reference clock for use
//! by timers and the watchdog. The reference clock frequency is obtained
//! from the [`Clocks`](crate::clocks::Clocks) peripheral after initialization,
//! so the correct cycles-per-tick value is always used regardless of which
//! clock source (XOSC, ROSC, etc.) feeds `clk_ref`.
//!
//! ## Features
//!
//! - Per-component tick generators (PROC0, PROC1, TIMER0, TIMER1, WATCHDOG)
//! - Configurable cycles per tick (auto-computed from `clk_ref`)
//! - Running status
//!
//! ## Example
//!
//! ```ignore
//! use pico_rs::clocks::ClockBuilder;
//! use pico_rs::ticks::Ticks;
//!
//! let clocks = ClockBuilder::new().build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);
//! let ticks = Ticks::init(dp.TICKS, &clocks);
//!
//! // Initialize only the tick generators you need
//! ticks.init_timer0();
//! ticks.init_watchdog();
//!
//! // Low-level control is also available
//! ticks.set_timer1_cycles(24);
//! ticks.enable_timer1();
//!
//! // Check if running
//! if ticks.timer0_running() {
//!     // Tick generator is active
//! }
//! ```

use crate::clocks::Clocks;

/// TICKS peripheral wrapper
///
/// Provides access to tick generator configuration and control.
/// The cycles-per-tick value is computed from `clk_ref` at construction time.
pub struct Ticks {
    ticks: pico_rs_pac::TICKS,
    /// Pre-computed cycles per 1µs tick, derived from `clk_ref`
    cycles_per_tick: u16,
}

impl Ticks {
    /// Initialize the TICKS peripheral.
    ///
    /// Computes the correct cycles-per-tick value from the reference clock
    /// frequency tracked by `clocks`.
    ///
    /// # Arguments
    /// * `ticks` - The TICKS peripheral from PAC
    /// * `clocks` - Reference to the initialized clock system
    #[inline]
    pub fn init(ticks: pico_rs_pac::TICKS, clocks: &Clocks) -> Self {
        let cycles_per_tick = (clocks.ref_freq() / 1_000_000) as u16;
        Self {
            ticks,
            cycles_per_tick,
        }
    }

    /// Get the configured cycles per tick
    #[inline]
    pub fn cycles_per_tick(&self) -> u16 {
        self.cycles_per_tick
    }

    /// Get reference to the underlying register block
    ///
    /// # Safety
    /// Direct register access can bypass the HAL's safety guarantees.
    /// The caller must ensure operations do not conflict with the HAL state.
    #[inline]
    pub unsafe fn regs(&self) -> &pico_rs_pac::ticks::RegisterBlock {
        &self.ticks
    }

    /// Release the TICKS peripheral
    #[inline]
    pub fn release(self) -> pico_rs_pac::TICKS {
        self.ticks
    }

    // ========================================================================
    // PROC0
    // ========================================================================

    /// Enable PROC0 tick generator
    #[inline]
    pub fn enable_proc0(&self) {
        self.ticks.proc0_ctrl.set_enable();
    }

    /// Disable PROC0 tick generator
    #[inline]
    pub fn disable_proc0(&self) {
        self.ticks.proc0_ctrl.clr_enable();
    }

    /// Check if PROC0 tick generator is running
    #[inline]
    pub fn proc0_running(&self) -> bool {
        self.ticks.proc0_ctrl.running()
    }

    /// Check if PROC0 tick generator is enabled
    #[inline]
    pub fn proc0_enabled(&self) -> bool {
        self.ticks.proc0_ctrl.enable()
    }

    /// Set PROC0 cycles per tick
    #[inline]
    pub fn set_proc0_cycles(&self, cycles: u16) {
        self.ticks.proc0_cycles.write_proc0_cycles(cycles as u32);
    }

    /// Get PROC0 cycles per tick
    #[inline]
    pub fn proc0_cycles(&self) -> u16 {
        self.ticks.proc0_cycles.proc0_cycles() as u16
    }

    /// Get PROC0 tick count
    #[inline]
    pub fn proc0_count(&self) -> u16 {
        self.ticks.proc0_count.proc0_count() as u16
    }

    // ========================================================================
    // PROC1
    // ========================================================================

    /// Enable PROC1 tick generator
    #[inline]
    pub fn enable_proc1(&self) {
        self.ticks.proc1_ctrl.set_enable();
    }

    /// Disable PROC1 tick generator
    #[inline]
    pub fn disable_proc1(&self) {
        self.ticks.proc1_ctrl.clr_enable();
    }

    /// Check if PROC1 tick generator is running
    #[inline]
    pub fn proc1_running(&self) -> bool {
        self.ticks.proc1_ctrl.running()
    }

    /// Check if PROC1 tick generator is enabled
    #[inline]
    pub fn proc1_enabled(&self) -> bool {
        self.ticks.proc1_ctrl.enable()
    }

    /// Set PROC1 cycles per tick
    #[inline]
    pub fn set_proc1_cycles(&self, cycles: u16) {
        self.ticks.proc1_cycles.write_proc1_cycles(cycles as u32);
    }

    /// Get PROC1 cycles per tick
    #[inline]
    pub fn proc1_cycles(&self) -> u16 {
        self.ticks.proc1_cycles.proc1_cycles() as u16
    }

    /// Get PROC1 tick count
    #[inline]
    pub fn proc1_count(&self) -> u16 {
        self.ticks.proc1_count.proc1_count() as u16
    }

    // ========================================================================
    // TIMER0
    // ========================================================================

    /// Enable TIMER0 tick generator
    #[inline]
    pub fn enable_timer0(&self) {
        self.ticks.timer0_ctrl.set_enable();
    }

    /// Disable TIMER0 tick generator
    #[inline]
    pub fn disable_timer0(&self) {
        self.ticks.timer0_ctrl.clr_enable();
    }

    /// Check if TIMER0 tick generator is running
    #[inline]
    pub fn timer0_running(&self) -> bool {
        self.ticks.timer0_ctrl.running()
    }

    /// Check if TIMER0 tick generator is enabled
    #[inline]
    pub fn timer0_enabled(&self) -> bool {
        self.ticks.timer0_ctrl.enable()
    }

    /// Set TIMER0 cycles per tick
    #[inline]
    pub fn set_timer0_cycles(&self, cycles: u16) {
        self.ticks.timer0_cycles.write_timer0_cycles(cycles as u32);
    }

    /// Get TIMER0 cycles per tick
    #[inline]
    pub fn timer0_cycles(&self) -> u16 {
        self.ticks.timer0_cycles.timer0_cycles() as u16
    }

    /// Get TIMER0 tick count
    #[inline]
    pub fn timer0_count(&self) -> u16 {
        self.ticks.timer0_count.timer0_count() as u16
    }

    // ========================================================================
    // TIMER1
    // ========================================================================

    /// Enable TIMER1 tick generator
    #[inline]
    pub fn enable_timer1(&self) {
        self.ticks.timer1_ctrl.set_enable();
    }

    /// Disable TIMER1 tick generator
    #[inline]
    pub fn disable_timer1(&self) {
        self.ticks.timer1_ctrl.clr_enable();
    }

    /// Check if TIMER1 tick generator is running
    #[inline]
    pub fn timer1_running(&self) -> bool {
        self.ticks.timer1_ctrl.running()
    }

    /// Check if TIMER1 tick generator is enabled
    #[inline]
    pub fn timer1_enabled(&self) -> bool {
        self.ticks.timer1_ctrl.enable()
    }

    /// Set TIMER1 cycles per tick
    #[inline]
    pub fn set_timer1_cycles(&self, cycles: u16) {
        self.ticks.timer1_cycles.write_timer1_cycles(cycles as u32);
    }

    /// Get TIMER1 cycles per tick
    #[inline]
    pub fn timer1_cycles(&self) -> u16 {
        self.ticks.timer1_cycles.timer1_cycles() as u16
    }

    /// Get TIMER1 tick count
    #[inline]
    pub fn timer1_count(&self) -> u16 {
        self.ticks.timer1_count.timer1_count() as u16
    }

    // ========================================================================
    // WATCHDOG
    // ========================================================================

    /// Enable WATCHDOG tick generator
    #[inline]
    pub fn enable_watchdog(&self) {
        self.ticks.watchdog_ctrl.set_enable();
    }

    /// Disable WATCHDOG tick generator
    #[inline]
    pub fn disable_watchdog(&self) {
        self.ticks.watchdog_ctrl.clr_enable();
    }

    /// Check if WATCHDOG tick generator is running
    #[inline]
    pub fn watchdog_running(&self) -> bool {
        self.ticks.watchdog_ctrl.running()
    }

    /// Check if WATCHDOG tick generator is enabled
    #[inline]
    pub fn watchdog_enabled(&self) -> bool {
        self.ticks.watchdog_ctrl.enable()
    }

    /// Set WATCHDOG cycles per tick
    #[inline]
    pub fn set_watchdog_cycles(&self, cycles: u16) {
        self.ticks
            .watchdog_cycles
            .write_watchdog_cycles(cycles as u32);
    }

    /// Get WATCHDOG cycles per tick
    #[inline]
    pub fn watchdog_cycles(&self) -> u16 {
        self.ticks.watchdog_cycles.watchdog_cycles() as u16
    }

    /// Get WATCHDOG tick count
    #[inline]
    pub fn watchdog_count(&self) -> u16 {
        self.ticks.watchdog_count.watchdog_count() as u16
    }

    // ========================================================================
    // Per-Generator Init (disable → configure → enable)
    // ========================================================================

    /// Initialize PROC0 tick generator (disable, set cycles, enable)
    #[inline]
    pub fn init_proc0(&self) {
        self.disable_proc0();
        self.set_proc0_cycles(self.cycles_per_tick);
        self.enable_proc0();
    }

    /// Initialize PROC1 tick generator (disable, set cycles, enable)
    #[inline]
    pub fn init_proc1(&self) {
        self.disable_proc1();
        self.set_proc1_cycles(self.cycles_per_tick);
        self.enable_proc1();
    }

    /// Initialize TIMER0 tick generator (disable, set cycles, enable)
    #[inline]
    pub fn init_timer0(&self) {
        self.disable_timer0();
        self.set_timer0_cycles(self.cycles_per_tick);
        self.enable_timer0();
    }

    /// Initialize TIMER1 tick generator (disable, set cycles, enable)
    #[inline]
    pub fn init_timer1(&self) {
        self.disable_timer1();
        self.set_timer1_cycles(self.cycles_per_tick);
        self.enable_timer1();
    }

    /// Initialize WATCHDOG tick generator (disable, set cycles, enable)
    #[inline]
    pub fn init_watchdog(&self) {
        self.disable_watchdog();
        self.set_watchdog_cycles(self.cycles_per_tick);
        self.enable_watchdog();
    }
}
