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
//! - Full system reboot support
//!
//! ## Example
//!
//! ```ignore
//! use pico_rs::watchdog::Watchdog;
//! use pico_rs_pac::WATCHDOG;
//!
//! let watchdog_pac = unsafe { WATCHDOG::steal() };
//! let wd = Watchdog::new(watchdog_pac);
//!
//! // Check reset reason
//! let reason = wd.reset_reason();
//!
//! // Configure for 1 second timeout
//! wd.start(1_000_000);
//!
//! // In main loop, feed the watchdog
//! loop {
//!     // Do work...
//!     wd.feed();
//! }
//! ```
//!
//! ## Full System Reboot
//!
//! ```ignore
//! use pico_rs::watchdog::Watchdog;
//! use pico_rs_pac::WATCHDOG;
//!
//! let watchdog_pac = unsafe { WATCHDOG::steal() };
//! let wd = Watchdog::new(watchdog_pac);
//!
//! // Reboot into regular flash path after 100ms
//! wd.reboot(100);
//! ```

use pico_rs_pac::psm::Wdsel;
use pico_rs_pac::{PSM, WATCHDOG};

/// Magic value indicating a non-reboot watchdog reset
const WATCHDOG_NON_REBOOT_MAGIC: u32 = 0x6ab73121;

/// Magic value for reboot with specific PC/SP
const WATCHDOG_REBOOT_MAGIC: u32 = 0xb007c0d3;

/// All PSM blocks to reset on watchdog (everything except ROSC and XOSC)
///
/// This mask includes all power state machine blocks that should be reset
/// during a full system reboot. ROSC and XOSC are excluded to maintain
/// clock stability during the reset process.
const PSM_WDSEL_RESET_BITS: u32 = Wdsel::PROC1
    | Wdsel::PROC0
    | Wdsel::ACCESSCTRL
    | Wdsel::SIO
    | Wdsel::XIP
    | Wdsel::SRAM9
    | Wdsel::SRAM8
    | Wdsel::SRAM7
    | Wdsel::SRAM6
    | Wdsel::SRAM5
    | Wdsel::SRAM4
    | Wdsel::SRAM3
    | Wdsel::SRAM2
    | Wdsel::SRAM1
    | Wdsel::SRAM0
    | Wdsel::BOOTRAM
    | Wdsel::ROM
    | Wdsel::BUSFABRIC
    | Wdsel::PSM_READY
    | Wdsel::CLOCKS
    | Wdsel::RESETS
    | Wdsel::OTP
    | Wdsel::PROC_COLD;
// Excluded: ROSC and XOSC to maintain clock stability

/// Maximum watchdog load value (24-bit counter)
const WATCHDOG_LOAD_MAX: u32 = 0x00ffffff;

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
/// Owns the PAC WATCHDOG peripheral to enforce proper ownership semantics.
pub struct Watchdog {
    watchdog: pico_rs_pac::WATCHDOG,
}

impl Watchdog {
    /// Create a new Watchdog instance without initialization
    ///
    /// # Arguments
    /// * `watchdog` - The WATCHDOG peripheral from PAC
    #[inline]
    pub fn new(watchdog: pico_rs_pac::WATCHDOG) -> Self {
        Self { watchdog }
    }

    /// Get reference to the underlying register block
    ///
    /// # Safety
    /// Direct register access can bypass the HAL's safety guarantees.
    /// The caller must ensure operations do not conflict with the HAL state.
    #[inline]
    pub unsafe fn regs(&self) -> &pico_rs_pac::watchdog::RegisterBlock {
        &self.watchdog
    }

    /// Release the WATCHDOG peripheral
    #[inline]
    pub fn release(self) -> pico_rs_pac::WATCHDOG {
        self.watchdog
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
        self.watchdog.load.write_load(timeout_us);
        self.watchdog.ctrl.set_enable();
    }

    /// Feed (reload) the watchdog to prevent reset
    ///
    /// This should be called regularly within the timeout period.
    #[inline]
    pub fn feed(&self) {
        let time = self.watchdog.ctrl.time();
        self.watchdog.load.write_load(time);
    }

    /// Feed the watchdog with a specific timeout value
    #[inline]
    pub fn feed_with(&self, timeout_us: u32) {
        self.watchdog.load.write_load(timeout_us);
    }

    /// Disable the watchdog
    #[inline]
    pub fn disable(&self) {
        self.watchdog.ctrl.clr_enable();
    }

    /// Check if the watchdog is enabled
    #[inline]
    pub fn is_enabled(&self) -> bool {
        self.watchdog.ctrl.enable()
    }

    /// Get the current countdown time in microseconds
    #[inline]
    pub fn time_remaining(&self) -> u32 {
        self.watchdog.ctrl.time()
    }

    /// Get the remaining time before watchdog reset in milliseconds
    #[inline]
    pub fn time_remaining_ms(&self) -> u32 {
        self.time_remaining() / 1000
    }

    /// Trigger an immediate watchdog reset
    #[inline]
    pub fn force_reset(&self) {
        self.watchdog.ctrl.trigger_trigger();
    }

    // ========================================================================
    // Debug Control
    // ========================================================================

    /// Pause watchdog when processor 0 is in debug mode
    #[inline]
    pub fn pause_on_debug0(&self, pause: bool) {
        if pause {
            self.watchdog.ctrl.set_pause_dbg0();
        } else {
            self.watchdog.ctrl.clr_pause_dbg0();
        }
    }

    /// Pause watchdog when processor 1 is in debug mode
    #[inline]
    pub fn pause_on_debug1(&self, pause: bool) {
        if pause {
            self.watchdog.ctrl.set_pause_dbg1();
        } else {
            self.watchdog.ctrl.clr_pause_dbg1();
        }
    }

    /// Pause watchdog when JTAG is accessing the bus fabric
    #[inline]
    pub fn pause_on_jtag(&self, pause: bool) {
        if pause {
            self.watchdog.ctrl.set_pause_jtag();
        } else {
            self.watchdog.ctrl.clr_pause_jtag();
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
        let reason = self.watchdog.reason.read_bits();

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
        self.watchdog.reason.timer()
    }

    /// Check if last reset was due to watchdog force trigger
    #[inline]
    pub fn was_force_reset(&self) -> bool {
        self.watchdog.reason.force()
    }

    /// Check if the last reset was caused by the watchdog (any reason)
    #[inline]
    pub fn caused_reboot(&self) -> bool {
        self.watchdog.reason.read_bits() != 0
    }

    /// Check if the last reset was caused by `enable_watchdog()`
    ///
    /// Distinguishes between a reset from the regular watchdog timer
    /// versus a reboot initiated by `reboot()` or `reboot_to_address()`.
    #[inline]
    pub fn enable_caused_reboot(&self) -> bool {
        self.watchdog.reason.read_bits() != 0
            && self.watchdog.scratch4.read_bits() == WATCHDOG_NON_REBOOT_MAGIC
    }

    // ========================================================================
    // Scratch Registers
    // ========================================================================

    /// Read a scratch register (0-7)
    ///
    /// Scratch registers persist through soft resets.
    pub fn read_scratch(&self, index: usize) -> u32 {
        match index {
            0 => self.watchdog.scratch0.read_bits(),
            1 => self.watchdog.scratch1.read_bits(),
            2 => self.watchdog.scratch2.read_bits(),
            3 => self.watchdog.scratch3.read_bits(),
            4 => self.watchdog.scratch4.read_bits(),
            5 => self.watchdog.scratch5.read_bits(),
            6 => self.watchdog.scratch6.read_bits(),
            7 => self.watchdog.scratch7.read_bits(),
            _ => 0,
        }
    }

    /// Write a scratch register (0-7)
    ///
    /// Scratch registers persist through soft resets.
    pub fn write_scratch(&self, index: usize, value: u32) {
        match index {
            0 => self.watchdog.scratch0.write_bits(value),
            1 => self.watchdog.scratch1.write_bits(value),
            2 => self.watchdog.scratch2.write_bits(value),
            3 => self.watchdog.scratch3.write_bits(value),
            4 => self.watchdog.scratch4.write_bits(value),
            5 => self.watchdog.scratch5.write_bits(value),
            6 => self.watchdog.scratch6.write_bits(value),
            7 => self.watchdog.scratch7.write_bits(value),
            _ => {}
        }
    }

    /// Read scratch register 0
    #[inline]
    pub fn scratch0(&self) -> u32 {
        self.watchdog.scratch0.read_bits()
    }

    /// Write scratch register 0
    #[inline]
    pub fn set_scratch0(&self, value: u32) {
        self.watchdog.scratch0.write_bits(value);
    }

    /// Read scratch register 1
    #[inline]
    pub fn scratch1(&self) -> u32 {
        self.watchdog.scratch1.read_bits()
    }

    /// Write scratch register 1
    #[inline]
    pub fn set_scratch1(&self, value: u32) {
        self.watchdog.scratch1.write_bits(value);
    }

    /// Read scratch register 2
    #[inline]
    pub fn scratch2(&self) -> u32 {
        self.watchdog.scratch2.read_bits()
    }

    /// Write scratch register 2
    #[inline]
    pub fn set_scratch2(&self, value: u32) {
        self.watchdog.scratch2.write_bits(value);
    }

    /// Read scratch register 3
    #[inline]
    pub fn scratch3(&self) -> u32 {
        self.watchdog.scratch3.read_bits()
    }

    /// Write scratch register 3
    #[inline]
    pub fn set_scratch3(&self, value: u32) {
        self.watchdog.scratch3.write_bits(value);
    }

    /// Read scratch register 4
    #[inline]
    pub fn scratch4(&self) -> u32 {
        self.watchdog.scratch4.read_bits()
    }

    /// Write scratch register 4
    #[inline]
    pub fn set_scratch4(&self, value: u32) {
        self.watchdog.scratch4.write_bits(value);
    }

    /// Read scratch register 5
    #[inline]
    pub fn scratch5(&self) -> u32 {
        self.watchdog.scratch5.read_bits()
    }

    /// Write scratch register 5
    #[inline]
    pub fn set_scratch5(&self, value: u32) {
        self.watchdog.scratch5.write_bits(value);
    }

    /// Read scratch register 6
    #[inline]
    pub fn scratch6(&self) -> u32 {
        self.watchdog.scratch6.read_bits()
    }

    /// Write scratch register 6
    #[inline]
    pub fn set_scratch6(&self, value: u32) {
        self.watchdog.scratch6.write_bits(value);
    }

    /// Read scratch register 7
    #[inline]
    pub fn scratch7(&self) -> u32 {
        self.watchdog.scratch7.read_bits()
    }

    /// Write scratch register 7
    #[inline]
    pub fn set_scratch7(&self, value: u32) {
        self.watchdog.scratch7.write_bits(value);
    }

    // ========================================================================
    // Watchdog-based Enable/Reboot
    // ========================================================================

    /// Enable watchdog with the specified timeout
    ///
    /// This enables the watchdog as a regular watchdog timer (not for reboot).
    /// The `feed()` function must be called periodically to prevent reset.
    ///
    /// # Arguments
    /// * `delay_ms` - Timeout in milliseconds (max ~8300ms)
    /// * `pause_on_debug` - Whether to pause the watchdog during debug
    pub fn enable_watchdog(&self, delay_ms: u32, pause_on_debug: bool) {
        self.watchdog.scratch4.write_bits(WATCHDOG_NON_REBOOT_MAGIC);

        self.configure_watchdog_reset(delay_ms, pause_on_debug);
    }

    /// Perform a full system reboot
    ///
    /// This function initiates a complete system reset using the watchdog timer.
    /// After the specified delay, the system will reset and boot from flash.
    ///
    /// # Arguments
    /// * `delay_ms` - Delay in milliseconds before reset (0 = immediate)
    ///
    /// # Note
    /// This function does not return. The system will reset after the delay.
    pub fn reboot(&self, delay_ms: u32) -> ! {
        self.watchdog.scratch4.write_bits(0);
        self.configure_watchdog_reset(delay_ms, false);

        loop {
            pico_rs_rt::asm::wfi();
        }
    }

    /// Perform a full system reboot to a specific address
    ///
    /// This function initiates a complete system reset and configures the
    /// Boot ROM to jump to a specific program counter (PC) and stack pointer (SP)
    /// after the reset.
    ///
    /// # Arguments
    /// * `pc` - Program counter (entry point) to jump to after reset
    /// * `sp` - Stack pointer value to set before jumping
    /// * `delay_ms` - Delay in milliseconds before reset (0 = immediate)
    ///
    /// # Safety
    ///
    /// `pc` must be a valid, aligned code address and `sp` must point to
    /// a valid stack region. Invalid values cause undefined behaviour
    /// after reset.
    pub unsafe fn reboot_to_address(&self, pc: u32, sp: u32, delay_ms: u32) -> ! {
        self.watchdog.ctrl.clr_enable();

        let pc_thumb = pc | 1;
        self.watchdog.scratch4.write_bits(WATCHDOG_REBOOT_MAGIC);
        self.watchdog
            .scratch5
            .write_bits(pc_thumb ^ (WATCHDOG_REBOOT_MAGIC.wrapping_neg()));
        self.watchdog.scratch6.write_bits(sp);
        self.watchdog.scratch7.write_bits(pc_thumb);

        self.configure_watchdog_reset(delay_ms, false);

        loop {
            pico_rs_rt::asm::wfi();
        }
    }

    // ========================================================================
    // Internal Helpers
    // ========================================================================

    /// Configure watchdog for system reset (internal helper)
    fn configure_watchdog_reset(&self, delay_ms: u32, pause_on_debug: bool) {
        let psm = unsafe { PSM::steal() };

        self.watchdog.ctrl.clr_enable();
        psm.wdsel.set_bits(PSM_WDSEL_RESET_BITS);

        if pause_on_debug {
            self.watchdog.ctrl.set_pause_dbg0();
            self.watchdog.ctrl.set_pause_dbg1();
            self.watchdog.ctrl.set_pause_jtag();
        } else {
            self.watchdog.ctrl.clr_pause_dbg0();
            self.watchdog.ctrl.clr_pause_dbg1();
            self.watchdog.ctrl.clr_pause_jtag();
        }

        if delay_ms == 0 {
            self.watchdog.ctrl.trigger_trigger();
        } else {
            let load_value = (delay_ms.saturating_mul(1000)).min(WATCHDOG_LOAD_MAX);
            self.watchdog.load.write_load(load_value);
            self.watchdog.ctrl.set_enable();
        }
    }
}

// ============================================================================
// Module-level Convenience Functions
// ============================================================================

/// Perform a full system reboot (module-level convenience function)
///
/// This function uses unsafe global access to the WATCHDOG peripheral.
/// Prefer using `Watchdog::reboot()` when you have an owned instance.
///
/// # Arguments
/// * `delay_ms` - Delay in milliseconds before reset (0 = immediate)
///
/// # Note
/// This function does not return. The system will reset after the delay.
///
/// # Example
///
/// ```ignore
/// use pico_rs::watchdog;
///
/// // Reboot after 10ms
/// watchdog::reboot(10);
/// ```
pub fn reboot(delay_ms: u32) -> ! {
    let watchdog = unsafe { WATCHDOG::steal() };
    let psm = unsafe { PSM::steal() };

    watchdog.scratch4.write_bits(0);
    watchdog.ctrl.clr_enable();
    psm.wdsel.set_bits(PSM_WDSEL_RESET_BITS);

    watchdog.ctrl.clr_pause_dbg0();
    watchdog.ctrl.clr_pause_dbg1();
    watchdog.ctrl.clr_pause_jtag();

    if delay_ms == 0 {
        watchdog.ctrl.trigger_trigger();
    } else {
        let load_value = (delay_ms.saturating_mul(1000)).min(WATCHDOG_LOAD_MAX);
        watchdog.load.write_load(load_value);
        watchdog.ctrl.set_enable();
    }

    loop {
        pico_rs_rt::asm::wfi();
    }
}

/// Perform a full system reboot to a specific address (module-level convenience)
///
/// # Arguments
/// * `pc` - Program counter (entry point) to jump to after reset
/// * `sp` - Stack pointer value to set before jumping
/// * `delay_ms` - Delay in milliseconds before reset (0 = immediate)
///
/// # Safety
///
/// `pc` must be a valid, aligned code address and `sp` must point to
/// a valid stack region. Invalid values cause undefined behaviour
/// after reset.
pub unsafe fn reboot_to_address(pc: u32, sp: u32, delay_ms: u32) -> ! {
    let watchdog = unsafe { WATCHDOG::steal() };
    let psm = unsafe { PSM::steal() };

    watchdog.ctrl.clr_enable();

    let pc_thumb = pc | 1;
    watchdog.scratch4.write_bits(WATCHDOG_REBOOT_MAGIC);
    watchdog
        .scratch5
        .write_bits(pc_thumb ^ (WATCHDOG_REBOOT_MAGIC.wrapping_neg()));
    watchdog.scratch6.write_bits(sp);
    watchdog.scratch7.write_bits(pc_thumb);

    psm.wdsel.set_bits(PSM_WDSEL_RESET_BITS);
    watchdog.ctrl.clr_pause_dbg0();
    watchdog.ctrl.clr_pause_dbg1();
    watchdog.ctrl.clr_pause_jtag();

    if delay_ms == 0 {
        watchdog.ctrl.trigger_trigger();
    } else {
        let load_value = (delay_ms.saturating_mul(1000)).min(WATCHDOG_LOAD_MAX);
        watchdog.load.write_load(load_value);
        watchdog.ctrl.set_enable();
    }

    loop {
        pico_rs_rt::asm::wfi();
    }
}

/// Check if the last reset was caused by the watchdog (module-level convenience)
#[inline]
pub fn caused_reboot() -> bool {
    let watchdog = unsafe { WATCHDOG::steal() };
    watchdog.reason.read_bits() != 0
}

/// Check if the last reset was caused by `enable_watchdog()` (module-level convenience)
#[inline]
pub fn enable_caused_reboot() -> bool {
    let watchdog = unsafe { WATCHDOG::steal() };
    watchdog.reason.read_bits() != 0 && watchdog.scratch4.read_bits() == WATCHDOG_NON_REBOOT_MAGIC
}
