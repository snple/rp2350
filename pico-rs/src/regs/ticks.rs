//! TICKS peripheral registers
//!
//! Provides tick generators for various subsystems

use super::Reg;
use super::addr::TICKS_BASE;

// Each tick generator has 3 registers at 0x0C intervals
// CTRL, CYCLES, COUNT
fn tick_ctrl(index: usize) -> u32 {
    TICKS_BASE + (index as u32) * 0x0C
}

fn tick_cycles(index: usize) -> u32 {
    TICKS_BASE + (index as u32) * 0x0C + 0x04
}

fn tick_count(index: usize) -> u32 {
    TICKS_BASE + (index as u32) * 0x0C + 0x08
}

// Tick generator indices
pub const TICK_PROC0: usize = 0;
pub const TICK_PROC1: usize = 1;
pub const TICK_TIMER0: usize = 2;
pub const TICK_TIMER1: usize = 3;
pub const TICK_WATCHDOG: usize = 4;
pub const TICK_RISCV: usize = 5;

// CTRL bits
pub const CTRL_RUNNING: u32 = 1 << 1;
pub const CTRL_ENABLE: u32 = 1 << 0;

/// Tick generator
pub struct Tick {
    index: usize,
}

impl Tick {
    pub const PROC0: Tick = Tick { index: TICK_PROC0 };
    pub const PROC1: Tick = Tick { index: TICK_PROC1 };
    pub const TIMER0: Tick = Tick { index: TICK_TIMER0 };
    pub const TIMER1: Tick = Tick { index: TICK_TIMER1 };
    pub const WATCHDOG: Tick = Tick {
        index: TICK_WATCHDOG,
    };

    #[inline]
    fn ctrl(&self) -> Reg {
        Reg::new(tick_ctrl(self.index))
    }

    #[inline]
    fn cycles(&self) -> Reg {
        Reg::new(tick_cycles(self.index))
    }

    #[inline]
    fn count(&self) -> Reg {
        Reg::new(tick_count(self.index))
    }

    /// Configure and enable the tick generator
    ///
    /// `cycles` is the number of reference clock cycles per tick.
    /// For clk_ref = 12 MHz and 1 µs ticks, use cycles = 12.
    pub fn configure(&self, cycles: u32) {
        // Disable first
        self.ctrl().write(0);
        // Set cycles (9-bit value)
        self.cycles().write(cycles & 0x1FF);
        // Enable
        self.ctrl().write(CTRL_ENABLE);
    }

    /// Check if tick generator is running
    #[inline]
    pub fn is_running(&self) -> bool {
        self.ctrl().is_set(CTRL_RUNNING)
    }

    /// Disable the tick generator
    #[inline]
    pub fn disable(&self) {
        self.ctrl().clear_bits(CTRL_ENABLE);
    }

    /// Enable the tick generator
    #[inline]
    pub fn enable(&self) {
        self.ctrl().set_bits(CTRL_ENABLE);
    }

    /// Get current count value
    #[inline]
    pub fn get_count(&self) -> u32 {
        self.count().read()
    }
}
