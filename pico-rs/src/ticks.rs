//! TICKS (Tick Generator) HAL for RP2350
//!
//! This module provides access to the tick generators for various system
//! components including processors, timers, watchdog, and RISC-V.
//!
//! The tick generators derive a 1µs tick from the reference clock for use
//! by timers and the watchdog.
//!
//! ## Features
//!
//! - Per-component tick generators (PROC0, PROC1, TIMER0, TIMER1, WATCHDOG, RISCV)
//! - Configurable cycles per tick
//! - Enable/disable control
//! - Running status
//!
//! ## Example
//!
//! ```ignore
//! use pico_rs::ticks::Ticks;
//! use pico_rs_pac::TICKS;
//!
//! let ticks_pac = unsafe { TICKS::steal() };
//! let ticks = Ticks::new(ticks_pac);
//!
//! // Configure all tick generators for 12 MHz reference clock
//! // 12 cycles per tick = 1µs tick period
//! ticks.init(12_000_000);
//!
//! // Or configure individual generators
//! ticks.set_timer0_cycles(12);
//! ticks.enable_timer0();
//!
//! // Check if running
//! if ticks.timer0_running() {
//!     // Tick generator is active
//! }
//! ```

/// Tick generator target
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TickTarget {
    /// Processor 0
    Proc0,
    /// Processor 1
    Proc1,
    /// Timer 0
    Timer0,
    /// Timer 1
    Timer1,
    /// Watchdog
    Watchdog,
    /// RISC-V
    Riscv,
}

/// TICKS peripheral wrapper
///
/// Provides access to tick generator configuration and control.
/// Owns the PAC TICKS peripheral to enforce proper ownership semantics.
pub struct Ticks {
    ticks: pico_rs_pac::TICKS,
}

impl Ticks {
    /// Create a new Ticks instance
    ///
    /// # Arguments
    /// * `ticks` - The TICKS peripheral from PAC
    #[inline]
    pub fn new(ticks: pico_rs_pac::TICKS) -> Self {
        Self { ticks }
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
    // RISCV
    // ========================================================================

    /// Enable RISCV tick generator
    #[inline]
    pub fn enable_riscv(&self) {
        self.ticks.riscv_ctrl.set_enable();
    }

    /// Disable RISCV tick generator
    #[inline]
    pub fn disable_riscv(&self) {
        self.ticks.riscv_ctrl.clr_enable();
    }

    /// Check if RISCV tick generator is running
    #[inline]
    pub fn riscv_running(&self) -> bool {
        self.ticks.riscv_ctrl.running()
    }

    /// Check if RISCV tick generator is enabled
    #[inline]
    pub fn riscv_enabled(&self) -> bool {
        self.ticks.riscv_ctrl.enable()
    }

    /// Set RISCV cycles per tick
    #[inline]
    pub fn set_riscv_cycles(&self, cycles: u16) {
        self.ticks.riscv_cycles.write_riscv_cycles(cycles as u32);
    }

    /// Get RISCV cycles per tick
    #[inline]
    pub fn riscv_cycles(&self) -> u16 {
        self.ticks.riscv_cycles.riscv_cycles() as u16
    }

    /// Get RISCV tick count
    #[inline]
    pub fn riscv_count(&self) -> u16 {
        self.ticks.riscv_count.riscv_count() as u16
    }

    // ========================================================================
    // Generic Operations
    // ========================================================================

    /// Initialize all tick generators for a given reference clock frequency
    ///
    /// This sets up all tick generators to produce 1µs ticks.
    ///
    /// # Arguments
    /// * `ref_clk_hz` - Reference clock frequency in Hz
    pub fn init(&self, ref_clk_hz: u32) {
        let cycles = (ref_clk_hz / 1_000_000) as u16;

        self.set_proc0_cycles(cycles);
        self.set_proc1_cycles(cycles);
        self.set_timer0_cycles(cycles);
        self.set_timer1_cycles(cycles);
        self.set_watchdog_cycles(cycles);
        self.set_riscv_cycles(cycles);
    }

    /// Enable all tick generators
    pub fn enable_all(&self) {
        self.enable_proc0();
        self.enable_proc1();
        self.enable_timer0();
        self.enable_timer1();
        self.enable_watchdog();
        self.enable_riscv();
    }

    /// Disable all tick generators
    pub fn disable_all(&self) {
        self.disable_proc0();
        self.disable_proc1();
        self.disable_timer0();
        self.disable_timer1();
        self.disable_watchdog();
        self.disable_riscv();
    }
}
