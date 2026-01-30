//! WATCHDOG HAL for RP2350
//!
//! This module provides access to the watchdog timer for system reset
//! and scratch registers for data persistence through resets.
//!
//! ## Features
//!
//! - Configurable watchdog timeout (up to ~8.3 seconds at 1µs tick)
//! - Pause during debug
//! - Software reset trigger
//! - 8 scratch registers for data persistence
//! - Reset reason detection
//!
//! ## Example
//!
//! ```ignore
//! use pico_rs::watchdog::Watchdog;
//!
//! let mut watchdog = Watchdog::new();
//!
//! // Configure for 1 second timeout
//! watchdog.start(1_000_000);
//!
//! // In main loop, feed the watchdog
//! loop {
//!     // Do work...
//!     watchdog.feed();
//! }
//! ```

use pico_rs_pac::WATCHDOG;

/// Reset reason
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ResetReason {
    /// Hardware reset or power-on
    PowerOn,
    /// Watchdog timer expired
    WatchdogTimeout,
    /// Watchdog force triggered
    WatchdogForce,
    /// Unknown/multiple reasons
    Unknown(u32),
}

/// WATCHDOG peripheral wrapper
///
/// Provides access to watchdog timer and scratch registers.
pub struct Watchdog {
    regs: &'static pico_rs_pac::watchdog::RegisterBlock,
}

impl Watchdog {
    /// Create a new Watchdog instance
    #[inline]
    pub fn new() -> Self {
        Self {
            regs: unsafe { &*WATCHDOG::PTR },
        }
    }

    /// Get reference to the register block
    #[inline]
    pub fn regs(&self) -> &pico_rs_pac::watchdog::RegisterBlock {
        self.regs
    }

    // ========================================================================
    // Watchdog Control
    // ========================================================================

    /// Start the watchdog with the specified timeout in microseconds
    ///
    /// The watchdog will reset the system if not fed within the timeout period.
    /// Maximum timeout is approximately 8.3 seconds (0x7FFFFF µs).
    ///
    /// # Arguments
    /// * `timeout_us` - Timeout in microseconds (max 0x7FFFFF = ~8.3s)
    pub fn start(&self, timeout_us: u32) {
        // Load the watchdog counter (value is halved internally)
        // Counter counts down and triggers reset at 0
        self.regs.load.write_load(timeout_us);

        // Enable the watchdog
        self.regs.ctrl.set_enable();
    }

    /// Feed (reload) the watchdog to prevent reset
    ///
    /// This should be called regularly within the timeout period.
    #[inline]
    pub fn feed(&self) {
        // Reload to maximum to give most time before next feed
        // The LOAD register is double-buffered, writing triggers reload
        let time = self.regs.ctrl.time();
        self.regs.load.write_load(time);
    }

    /// Feed the watchdog with a specific timeout value
    #[inline]
    pub fn feed_with(&self, timeout_us: u32) {
        self.regs.load.write_load(timeout_us);
    }

    /// Disable the watchdog
    #[inline]
    pub fn disable(&self) {
        self.regs.ctrl.clr_enable();
    }

    /// Check if the watchdog is enabled
    #[inline]
    pub fn is_enabled(&self) -> bool {
        self.regs.ctrl.enable()
    }

    /// Get the current countdown time in microseconds
    #[inline]
    pub fn time_remaining(&self) -> u32 {
        self.regs.ctrl.time()
    }

    /// Trigger an immediate watchdog reset
    #[inline]
    pub fn force_reset(&self) {
        self.regs.ctrl.trigger_trigger();
    }

    // ========================================================================
    // Debug Control
    // ========================================================================

    /// Pause watchdog when processor 0 is in debug mode
    #[inline]
    pub fn pause_on_debug0(&self, pause: bool) {
        if pause {
            self.regs.ctrl.set_pause_dbg0();
        } else {
            self.regs.ctrl.clr_pause_dbg0();
        }
    }

    /// Pause watchdog when processor 1 is in debug mode
    #[inline]
    pub fn pause_on_debug1(&self, pause: bool) {
        if pause {
            self.regs.ctrl.set_pause_dbg1();
        } else {
            self.regs.ctrl.clr_pause_dbg1();
        }
    }

    /// Pause watchdog when JTAG is accessing the bus fabric
    #[inline]
    pub fn pause_on_jtag(&self, pause: bool) {
        if pause {
            self.regs.ctrl.set_pause_jtag();
        } else {
            self.regs.ctrl.clr_pause_jtag();
        }
    }

    /// Enable all debug pausing (convenience method)
    pub fn enable_debug_pause(&self) {
        self.pause_on_debug0(true);
        self.pause_on_debug1(true);
        self.pause_on_jtag(true);
    }

    /// Disable all debug pausing (convenience method)
    pub fn disable_debug_pause(&self) {
        self.pause_on_debug0(false);
        self.pause_on_debug1(false);
        self.pause_on_jtag(false);
    }

    // ========================================================================
    // Reset Reason
    // ========================================================================

    /// Get the reason for the last reset
    pub fn reset_reason(&self) -> ResetReason {
        let reason = self.regs.reason.read_bits();

        if reason == 0 {
            ResetReason::PowerOn
        } else if (reason & 0x01) != 0 {
            ResetReason::WatchdogTimeout
        } else if (reason & 0x02) != 0 {
            ResetReason::WatchdogForce
        } else {
            ResetReason::Unknown(reason)
        }
    }

    /// Check if last reset was due to watchdog timeout
    #[inline]
    pub fn was_watchdog_reset(&self) -> bool {
        self.regs.reason.timer()
    }

    /// Check if last reset was due to watchdog force trigger
    #[inline]
    pub fn was_force_reset(&self) -> bool {
        self.regs.reason.force()
    }

    // ========================================================================
    // Scratch Registers
    // ========================================================================

    /// Read a scratch register (0-7)
    ///
    /// Scratch registers persist through soft resets.
    pub fn read_scratch(&self, index: usize) -> u32 {
        match index {
            0 => self.regs.scratch0.read_bits(),
            1 => self.regs.scratch1.read_bits(),
            2 => self.regs.scratch2.read_bits(),
            3 => self.regs.scratch3.read_bits(),
            4 => self.regs.scratch4.read_bits(),
            5 => self.regs.scratch5.read_bits(),
            6 => self.regs.scratch6.read_bits(),
            7 => self.regs.scratch7.read_bits(),
            _ => 0,
        }
    }

    /// Write a scratch register (0-7)
    ///
    /// Scratch registers persist through soft resets.
    pub fn write_scratch(&self, index: usize, value: u32) {
        match index {
            0 => self.regs.scratch0.write_bits(value),
            1 => self.regs.scratch1.write_bits(value),
            2 => self.regs.scratch2.write_bits(value),
            3 => self.regs.scratch3.write_bits(value),
            4 => self.regs.scratch4.write_bits(value),
            5 => self.regs.scratch5.write_bits(value),
            6 => self.regs.scratch6.write_bits(value),
            7 => self.regs.scratch7.write_bits(value),
            _ => {}
        }
    }

    /// Read scratch register 0
    #[inline]
    pub fn scratch0(&self) -> u32 {
        self.regs.scratch0.read_bits()
    }

    /// Write scratch register 0
    #[inline]
    pub fn set_scratch0(&self, value: u32) {
        self.regs.scratch0.write_bits(value);
    }

    /// Read scratch register 1
    #[inline]
    pub fn scratch1(&self) -> u32 {
        self.regs.scratch1.read_bits()
    }

    /// Write scratch register 1
    #[inline]
    pub fn set_scratch1(&self, value: u32) {
        self.regs.scratch1.write_bits(value);
    }

    /// Read scratch register 2
    #[inline]
    pub fn scratch2(&self) -> u32 {
        self.regs.scratch2.read_bits()
    }

    /// Write scratch register 2
    #[inline]
    pub fn set_scratch2(&self, value: u32) {
        self.regs.scratch2.write_bits(value);
    }

    /// Read scratch register 3
    #[inline]
    pub fn scratch3(&self) -> u32 {
        self.regs.scratch3.read_bits()
    }

    /// Write scratch register 3
    #[inline]
    pub fn set_scratch3(&self, value: u32) {
        self.regs.scratch3.write_bits(value);
    }

    /// Read scratch register 4
    #[inline]
    pub fn scratch4(&self) -> u32 {
        self.regs.scratch4.read_bits()
    }

    /// Write scratch register 4
    #[inline]
    pub fn set_scratch4(&self, value: u32) {
        self.regs.scratch4.write_bits(value);
    }

    /// Read scratch register 5
    #[inline]
    pub fn scratch5(&self) -> u32 {
        self.regs.scratch5.read_bits()
    }

    /// Write scratch register 5
    #[inline]
    pub fn set_scratch5(&self, value: u32) {
        self.regs.scratch5.write_bits(value);
    }

    /// Read scratch register 6
    #[inline]
    pub fn scratch6(&self) -> u32 {
        self.regs.scratch6.read_bits()
    }

    /// Write scratch register 6
    #[inline]
    pub fn set_scratch6(&self, value: u32) {
        self.regs.scratch6.write_bits(value);
    }

    /// Read scratch register 7
    #[inline]
    pub fn scratch7(&self) -> u32 {
        self.regs.scratch7.read_bits()
    }

    /// Write scratch register 7
    #[inline]
    pub fn set_scratch7(&self, value: u32) {
        self.regs.scratch7.write_bits(value);
    }
}

impl Default for Watchdog {
    fn default() -> Self {
        Self::new()
    }
}
