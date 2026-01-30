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
//!
//! ## Full System Reboot
//!
//! ```ignore
//! use pico_rs::watchdog;
//!
//! // Reboot into regular flash path after 1ms
//! watchdog::reboot(1);
//!
//! // Reboot to a specific address
//! watchdog::reboot_to_address(0x10000000, 0x20001000, 1);
//! ```

use pico_rs_pac::psm::Wdsel;
use pico_rs_pac::{PSM, WATCHDOG};

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

// ============================================================================
// Module-level Reset Functions
// ============================================================================

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

/// Configure watchdog for system reset
///
/// This is an internal helper that configures PSM wdsel and watchdog settings
/// for a full system reset.
///
/// # Arguments
/// * `delay_ms` - Delay in milliseconds before reset (0 = immediate trigger)
/// * `pause_on_debug` - Whether to pause watchdog during debug
fn configure_watchdog_reset(delay_ms: u32, pause_on_debug: bool) {
    let watchdog = unsafe { &*WATCHDOG::PTR };
    let psm = unsafe { &*PSM::PTR };

    // Disable watchdog before configuration
    watchdog.ctrl.clr_enable();

    // Configure PSM watchdog select register
    // Reset everything except ROSC and XOSC to maintain clock stability
    psm.wdsel.set_bits(PSM_WDSEL_RESET_BITS);

    // Configure debug pause behavior
    if pause_on_debug {
        watchdog.ctrl.set_pause_dbg0();
        watchdog.ctrl.set_pause_dbg1();
        watchdog.ctrl.set_pause_jtag();
    } else {
        watchdog.ctrl.clr_pause_dbg0();
        watchdog.ctrl.clr_pause_dbg1();
        watchdog.ctrl.clr_pause_jtag();
    }

    if delay_ms == 0 {
        // Immediate trigger
        watchdog.ctrl.trigger_trigger();
    } else {
        // Calculate load value (microseconds, capped at 24-bit max)
        let load_value = (delay_ms.saturating_mul(1000)).min(WATCHDOG_LOAD_MAX);

        // Load the counter
        watchdog.load.write_load(load_value);

        // Enable watchdog
        watchdog.ctrl.set_enable();
    }
}

/// Perform a full system reboot
///
/// This function initiates a complete system reset using the watchdog timer.
/// After the specified delay, the system will reset and boot from flash.
///
/// The reboot process:
/// 1. Configures PSM to reset all subsystems (except oscillators)
/// 2. Sets up watchdog timer with the specified delay
/// 3. Triggers watchdog which causes a full system reset
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
    let watchdog = unsafe { &*WATCHDOG::PTR };

    // Clear scratch[4] to indicate regular flash boot path
    watchdog.scratch4.write_bits(0);

    // Configure and trigger reboot
    configure_watchdog_reset(delay_ms, false);

    // Wait for watchdog to trigger reset
    loop {
        cortex_m::asm::wfi();
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
/// # Note
/// This function does not return. The system will reset and begin
/// execution at the specified address.
///
/// # Example
///
/// ```ignore
/// use pico_rs::watchdog;
///
/// // Reboot to a specific application at 0x10010000
/// watchdog::reboot_to_address(0x10010000, 0x20040000, 1);
/// ```
pub fn reboot_to_address(pc: u32, sp: u32, delay_ms: u32) -> ! {
    let watchdog = unsafe { &*WATCHDOG::PTR };

    // Disable watchdog first
    watchdog.ctrl.clr_enable();

    // Set up scratch registers for Boot ROM
    // scratch[4] = magic to indicate reboot with address
    // scratch[5] = PC XOR'd with negative magic (for verification)
    // scratch[6] = SP
    // scratch[7] = PC
    let pc_thumb = pc | 1; // Set thumb mode bit for ARM
    watchdog.scratch4.write_bits(WATCHDOG_REBOOT_MAGIC);
    watchdog
        .scratch5
        .write_bits(pc_thumb ^ (WATCHDOG_REBOOT_MAGIC.wrapping_neg()));
    watchdog.scratch6.write_bits(sp);
    watchdog.scratch7.write_bits(pc_thumb);

    // Configure and trigger reboot (don't pause on debug for reboot)
    configure_watchdog_reset(delay_ms, false);

    // Wait for watchdog to trigger reset
    loop {
        cortex_m::asm::wfi();
    }
}

/// Enable watchdog with the specified timeout
///
/// This enables the watchdog as a regular watchdog timer (not for reboot).
/// The `feed()` function must be called periodically to prevent reset.
///
/// # Arguments
/// * `delay_ms` - Timeout in milliseconds (max ~8300ms)
/// * `pause_on_debug` - Whether to pause the watchdog during debug
///
/// # Example
///
/// ```ignore
/// use pico_rs::watchdog;
///
/// // Enable watchdog with 1 second timeout, pausing on debug
/// watchdog::enable(1000, true);
///
/// loop {
///     // Do work...
///     watchdog::feed();
/// }
/// ```
pub fn enable(delay_ms: u32, pause_on_debug: bool) {
    let watchdog = unsafe { &*WATCHDOG::PTR };

    // Mark as non-reboot watchdog
    watchdog.scratch4.write_bits(WATCHDOG_NON_REBOOT_MAGIC);

    configure_watchdog_reset(delay_ms, pause_on_debug);
}

/// Disable the watchdog timer
///
/// # Example
///
/// ```ignore
/// use pico_rs::watchdog;
///
/// watchdog::disable();
/// ```
#[inline]
pub fn disable() {
    let watchdog = unsafe { &*WATCHDOG::PTR };
    watchdog.ctrl.clr_enable();
}

/// Feed (reload) the watchdog timer
///
/// This reloads the watchdog counter to prevent a reset.
/// Should be called periodically when the watchdog is enabled.
///
/// # Example
///
/// ```ignore
/// use pico_rs::watchdog;
///
/// // Feed the watchdog to prevent reset
/// watchdog::feed();
/// ```
#[inline]
pub fn feed() {
    let watchdog = unsafe { &*WATCHDOG::PTR };
    let time = watchdog.ctrl.time();
    watchdog.load.write_load(time);
}

/// Get the remaining time before watchdog reset in microseconds
#[inline]
pub fn time_remaining_us() -> u32 {
    let watchdog = unsafe { &*WATCHDOG::PTR };
    watchdog.ctrl.time()
}

/// Get the remaining time before watchdog reset in milliseconds
#[inline]
pub fn time_remaining_ms() -> u32 {
    time_remaining_us() / 1000
}

/// Check if the last reset was caused by the watchdog
///
/// Returns `true` if the watchdog timer triggered the reset.
#[inline]
pub fn caused_reboot() -> bool {
    let watchdog = unsafe { &*WATCHDOG::PTR };
    watchdog.reason.read_bits() != 0
}

/// Check if the last reset was caused by `watchdog::enable()`
///
/// Distinguishes between a reset from the regular watchdog timer
/// versus a reboot initiated by `reboot()` or `reboot_to_address()`.
#[inline]
pub fn enable_caused_reboot() -> bool {
    let watchdog = unsafe { &*WATCHDOG::PTR };
    watchdog.reason.read_bits() != 0 && watchdog.scratch4.read_bits() == WATCHDOG_NON_REBOOT_MAGIC
}
