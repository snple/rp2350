//! SysTick timer driver for Cortex-M33
//!
//! The SysTick timer is a 24-bit down counter built into all Cortex-M processors.
//! It can be used for simple delays and as a timebase.
//!
//! # Example
//!
//! ```ignore
//! use pico_rs::systick::SysTick;
//! use pico_rs::time::Duration;
//!
//! // Initialize SysTick with 150 MHz clock (default RP2350 system clock)
//! let mut systick = SysTick::new(150_000_000);
//!
//! // Delay for 500ms
//! systick.delay(Duration::from_millis(500));
//!
//! // Delay for specific units
//! systick.delay_ms(100);
//! systick.delay_us(500);
//! ```

use crate::time::Duration;
use core::sync::atomic::{AtomicU32, Ordering};

/// SysTick Control and Status Register
const SYST_CSR: *mut u32 = 0xE000_E010 as *mut u32;
/// SysTick Reload Value Register
const SYST_RVR: *mut u32 = 0xE000_E014 as *mut u32;
/// SysTick Current Value Register
const SYST_CVR: *mut u32 = 0xE000_E018 as *mut u32;
/// SysTick Calibration Value Register
const SYST_CALIB: *const u32 = 0xE000_E01C as *const u32;

/// CSR bit: Counter enable
const SYST_CSR_ENABLE: u32 = 1 << 0;
/// CSR bit: Interrupt enable (for future use)
#[allow(dead_code)]
const SYST_CSR_TICKINT: u32 = 1 << 1;
/// CSR bit: Clock source (0 = external, 1 = processor)
const SYST_CSR_CLKSOURCE: u32 = 1 << 2;
/// CSR bit: Count flag (set when counter reaches 0)
const SYST_CSR_COUNTFLAG: u32 = 1 << 16;

/// Maximum reload value (24-bit counter)
const SYSTICK_MAX_RELOAD: u32 = 0x00FF_FFFF;

/// Global tick counter for overflow tracking
static TICK_COUNT: AtomicU32 = AtomicU32::new(0);

/// SysTick timer driver
pub struct SysTick {
    /// Clock frequency in Hz
    freq_hz: u32,
    /// Ticks per microsecond
    ticks_per_us: u32,
}

impl SysTick {
    /// Create a new SysTick timer with the given clock frequency
    ///
    /// # Arguments
    /// * `freq_hz` - The clock frequency in Hz (typically the CPU clock)
    ///
    /// # Example
    /// ```ignore
    /// // RP2350 default clock is 150 MHz
    /// let systick = SysTick::new(150_000_000);
    /// ```
    pub fn new(freq_hz: u32) -> Self {
        let ticks_per_us = freq_hz / 1_000_000;

        // Configure SysTick: processor clock, no interrupt, disabled
        unsafe {
            // Disable SysTick first
            SYST_CSR.write_volatile(0);
            // Clear current value
            SYST_CVR.write_volatile(0);
            // Set reload to max
            SYST_RVR.write_volatile(SYSTICK_MAX_RELOAD);
            // Enable with processor clock, no interrupt
            SYST_CSR.write_volatile(SYST_CSR_ENABLE | SYST_CSR_CLKSOURCE);
        }

        Self { freq_hz, ticks_per_us }
    }

    /// Get the clock frequency
    #[inline]
    pub fn freq_hz(&self) -> u32 {
        self.freq_hz
    }

    /// Read the current counter value (counts down from reload value)
    #[inline]
    pub fn read_counter(&self) -> u32 {
        unsafe { SYST_CVR.read_volatile() }
    }

    /// Check if the counter has reached zero since last check
    #[inline]
    pub fn has_wrapped(&self) -> bool {
        unsafe { (SYST_CSR.read_volatile() & SYST_CSR_COUNTFLAG) != 0 }
    }

    /// Get the calibration value (if available)
    pub fn calibration(&self) -> Option<u32> {
        let calib = unsafe { SYST_CALIB.read_volatile() };
        if calib & (1 << 31) != 0 {
            // NOREF bit set - no reference clock
            None
        } else {
            Some(calib & 0x00FF_FFFF)
        }
    }

    /// Delay for a duration
    pub fn delay(&mut self, duration: Duration) {
        let total_us = duration.as_micros();
        if total_us == 0 {
            return;
        }

        // For very long delays, break into chunks
        let max_delay_us = (SYSTICK_MAX_RELOAD / self.ticks_per_us) as u64;

        let mut remaining_us = total_us;
        while remaining_us > 0 {
            let chunk_us = remaining_us.min(max_delay_us) as u32;
            self.delay_us_internal(chunk_us);
            remaining_us -= chunk_us as u64;
        }
    }

    /// Delay for a number of milliseconds
    #[inline]
    pub fn delay_ms(&mut self, ms: u32) {
        self.delay(Duration::from_millis(ms as u64));
    }

    /// Delay for a number of microseconds
    #[inline]
    pub fn delay_us(&mut self, us: u32) {
        if us == 0 {
            return;
        }
        self.delay_us_internal(us);
    }

    /// Internal delay implementation for up to max reload ticks
    fn delay_us_internal(&mut self, us: u32) {
        let ticks = us.saturating_mul(self.ticks_per_us);
        if ticks == 0 {
            return;
        }

        // Clamp to max reload value
        let ticks = ticks.min(SYSTICK_MAX_RELOAD);

        unsafe {
            // Read start value
            let start = SYST_CVR.read_volatile();

            // Since SysTick counts down, we wait until enough ticks have passed
            loop {
                let current = SYST_CVR.read_volatile();

                // Handle wrap-around: counter reloads to RVR when it hits 0
                let elapsed = if current <= start {
                    start - current
                } else {
                    // Wrapped: start -> 0 -> reload -> current
                    start + (SYSTICK_MAX_RELOAD - current) + 1
                };

                if elapsed >= ticks {
                    break;
                }

                // Small yield to prevent tight spinning
                cortex_m::asm::nop();
            }
        }
    }

    /// Busy-wait delay using simple counter loop (less accurate but always works)
    /// Useful when SysTick is not yet configured
    #[inline]
    pub fn delay_cycles(cycles: u32) {
        for _ in 0..cycles {
            cortex_m::asm::nop();
        }
    }
}

/// A delay provider that implements embedded-hal DelayNs trait pattern
pub struct Delay {
    systick: SysTick,
}

impl Delay {
    /// Create a new Delay instance
    pub fn new(freq_hz: u32) -> Self {
        Self {
            systick: SysTick::new(freq_hz),
        }
    }

    /// Delay for nanoseconds (rounded to microseconds)
    #[inline]
    pub fn delay_ns(&mut self, ns: u32) {
        // Round up to microseconds
        let us = (ns + 999) / 1000;
        if us > 0 {
            self.systick.delay_us(us);
        }
    }

    /// Delay for microseconds
    #[inline]
    pub fn delay_us(&mut self, us: u32) {
        self.systick.delay_us(us);
    }

    /// Delay for milliseconds
    #[inline]
    pub fn delay_ms(&mut self, ms: u32) {
        self.systick.delay_ms(ms);
    }
}

/// Increment tick counter (call from SysTick interrupt handler if using interrupts)
#[inline]
pub fn tick() {
    TICK_COUNT.fetch_add(1, Ordering::Relaxed);
}

/// Get current tick count
#[inline]
pub fn get_tick_count() -> u32 {
    TICK_COUNT.load(Ordering::Relaxed)
}

/// Reset tick counter
#[inline]
pub fn reset_tick_count() {
    TICK_COUNT.store(0, Ordering::Relaxed);
}
