//! WATCHDOG peripheral registers

use super::Reg;
use super::addr::WATCHDOG_BASE;

// Register offsets
pub const CTRL: u32 = WATCHDOG_BASE + 0x00;
pub const LOAD: u32 = WATCHDOG_BASE + 0x04;
pub const REASON: u32 = WATCHDOG_BASE + 0x08;
pub const SCRATCH0: u32 = WATCHDOG_BASE + 0x0C;
pub const SCRATCH1: u32 = WATCHDOG_BASE + 0x10;
pub const SCRATCH2: u32 = WATCHDOG_BASE + 0x14;
pub const SCRATCH3: u32 = WATCHDOG_BASE + 0x18;
pub const SCRATCH4: u32 = WATCHDOG_BASE + 0x1C;
pub const SCRATCH5: u32 = WATCHDOG_BASE + 0x20;
pub const SCRATCH6: u32 = WATCHDOG_BASE + 0x24;
pub const SCRATCH7: u32 = WATCHDOG_BASE + 0x28;
pub const TICK: u32 = WATCHDOG_BASE + 0x2C;

// CTRL bits
pub const CTRL_ENABLE: u32 = 1 << 30;
pub const CTRL_PAUSE_JTAG: u32 = 1 << 26;
pub const CTRL_PAUSE_DBG0: u32 = 1 << 25;
pub const CTRL_PAUSE_DBG1: u32 = 1 << 24;
pub const CTRL_TIME_MASK: u32 = 0x00FFFFFF;

// TICK bits
pub const TICK_ENABLE: u32 = 1 << 9;
pub const TICK_RUNNING: u32 = 1 << 10;
pub const TICK_COUNT_MASK: u32 = 0x1FF;

// REASON bits
pub const REASON_FORCE: u32 = 1 << 1;
pub const REASON_TIMER: u32 = 1 << 0;

/// Watchdog peripheral driver
pub struct Watchdog;

impl Watchdog {
    const CTRL: Reg = Reg::new(CTRL);
    const LOAD: Reg = Reg::new(LOAD);
    const TICK: Reg = Reg::new(TICK);
    const REASON: Reg = Reg::new(REASON);

    /// Enable the watchdog tick generator
    ///
    /// `cycles` is the number of reference clock cycles per tick.
    /// For a 12 MHz reference clock, use cycles=12 for 1 µs ticks.
    #[inline]
    pub fn enable_tick(cycles: u32) {
        // Write cycles and enable bit
        Self::TICK.write((cycles & TICK_COUNT_MASK) | TICK_ENABLE);
    }

    /// Disable the watchdog tick generator
    #[inline]
    pub fn disable_tick() {
        Self::TICK.clear_bits(TICK_ENABLE);
    }

    /// Check if tick generator is running
    #[inline]
    pub fn is_tick_running() -> bool {
        Self::TICK.is_set(TICK_RUNNING)
    }

    /// Start the watchdog timer
    ///
    /// `delay_us` is the timeout in microseconds (max ~8.3 seconds)
    pub fn start(delay_us: u32) {
        // Watchdog counts down at 1 MHz (tick rate), but counts in steps of 2
        // So actual time = (load_value * 2) microseconds
        let load_value = delay_us / 2;

        // Load the counter
        Self::LOAD.write(load_value);

        // Enable watchdog with pauses during debug
        Self::CTRL.write(CTRL_ENABLE | CTRL_PAUSE_JTAG | CTRL_PAUSE_DBG0 | CTRL_PAUSE_DBG1);
    }

    /// Feed (kick/refresh) the watchdog to prevent reset
    #[inline]
    pub fn feed() {
        // Reading LOAD register returns 0, writing any value reloads
        let current = Self::CTRL.read() & CTRL_TIME_MASK;
        Self::LOAD.write(current);
    }

    /// Disable the watchdog timer
    #[inline]
    pub fn disable() {
        Self::CTRL.clear_bits(CTRL_ENABLE);
    }

    /// Check if last reset was caused by watchdog
    #[inline]
    pub fn was_watchdog_reset() -> bool {
        Self::REASON.any_set(REASON_TIMER | REASON_FORCE)
    }

    /// Force a watchdog reset
    pub fn force_reset() -> ! {
        // PSM_WDSEL selects which blocks get reset
        // For full reset, use trigger
        // Writing to CTRL with enable and very small time
        Self::LOAD.write(0);
        Self::CTRL.write(CTRL_ENABLE);
        loop {
            cortex_m::asm::nop();
        }
    }

    /// Get remaining time in microseconds
    #[inline]
    pub fn get_remaining_us() -> u32 {
        (Self::CTRL.read() & CTRL_TIME_MASK) * 2
    }
}
