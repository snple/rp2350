//! SysTick timer driver for Cortex-M33
//!
//! The SysTick timer is a 24-bit down counter built into all Cortex-M processors.
//! This module provides a safe interface using the rp2350-rt SYST peripheral.
//!
//! # Example
//!
//! ```ignore
//! use rp2350_rt::Peripherals;
//! use rp2350::systick::SysTick;
//! use rp2350::time::Duration;
//!
//! // Take the SYST peripheral
//! let cp = Peripherals::take().unwrap();
//!
//! // Initialize SysTick with 150 MHz clock (default RP2350 system clock)
//! let mut systick = SysTick::init(cp.SYST, 150_000_000);
//!
//! // Delay for 500ms
//! systick.delay(Duration::from_millis(500));
//!
//! // Delay for specific units
//! systick.delay_ms(100);
//! systick.delay_us(500);
//! ```

use crate::Steal;
use crate::time::Duration;
use rp2350_rt::peripheral::syst::{self, SYST, SystClkSource};

/// Maximum reload value (24-bit counter)
const SYSTICK_MAX_RELOAD: u32 = syst::MAX_RELOAD;

/// SysTick timer driver
///
/// This driver takes ownership of the rp2350-rt SYST peripheral and provides
/// convenient delay functions.
pub struct SysTick {
    /// The underlying SYST peripheral
    syst: SYST,
    /// Clock frequency in Hz
    freq_hz: u32,
}

impl SysTick {
    /// Initialize SysTick timer with the given clock frequency.
    ///
    /// Configures the SysTick counter with maximum reload and enables it.
    ///
    /// # Arguments
    /// * `syst` - The SYST peripheral from rp2350-rt
    /// * `freq_hz` - The clock frequency in Hz (typically the CPU clock)
    ///
    /// # Example
    /// ```ignore
    /// use rp2350_rt::Peripherals;
    /// use rp2350::systick::SysTick;
    ///
    /// let cp = Peripherals::take().unwrap();
    /// let systick = SysTick::init(cp.SYST, 150_000_000);
    /// ```
    pub fn init(mut syst: SYST, freq_hz: u32) -> Self {
        assert!(freq_hz > 0, "SysTick clock frequency must be non-zero");

        // Configure SysTick
        syst.disable_counter();
        syst.disable_interrupt();
        syst.set_clock_source(SystClkSource::Core);
        syst.clear_current();
        syst.set_reload(SYSTICK_MAX_RELOAD);
        syst.enable_counter();

        Self { syst, freq_hz }
    }

    /// Obtain a SysTick handle without re-initializing.
    ///
    /// # Safety
    ///
    /// The caller must ensure the SysTick timer has already been initialized
    /// (e.g. via [`SysTick::init`]) before use. Prefer [`SysTick::init`] which
    /// handles initialization automatically.
    ///
    /// Note: the returned handle has `freq_hz = 0`, so delay methods will not
    /// work. Use this only for reading counter/status values.
    #[inline]
    pub unsafe fn steal() -> Steal<Self> {
        Steal(Self {
            syst: unsafe { SYST::steal() },
            freq_hz: 0,
        })
    }

    /// Get the clock frequency
    #[inline]
    pub fn freq_hz(&self) -> u32 {
        self.freq_hz
    }

    /// Get the (truncated) number of ticks per microsecond.
    ///
    /// Note: the delay implementation computes ticks in 64-bit arithmetic and
    /// is more accurate than this truncated value.
    #[inline]
    pub fn ticks_per_us(&self) -> u32 {
        self.freq_hz / 1_000_000
    }

    /// Read the current counter value (counts down from reload value)
    #[inline]
    pub fn read_counter(&self) -> u32 {
        SYST::current()
    }

    /// Read the reload value
    #[inline]
    pub fn read_reload(&self) -> u32 {
        SYST::reload()
    }

    /// Check if the counter has reached zero since last check
    #[inline]
    pub fn has_wrapped(&mut self) -> bool {
        // Read CSR - the COUNTFLAG bit is cleared by reading it
        self.syst.has_wrapped()
    }

    /// Get the calibration value in ticks per 10 ms (if available).
    #[inline]
    pub fn ticks_per_10ms(&self) -> Option<u32> {
        SYST::ticks_per_10ms()
    }

    /// Enable the SysTick counter
    #[inline]
    pub fn enable(&mut self) {
        self.syst.enable_counter();
    }

    /// Disable the SysTick counter
    #[inline]
    pub fn disable(&mut self) {
        self.syst.disable_counter();
    }

    /// Check if the counter is enabled
    #[inline]
    pub fn is_enabled(&self) -> bool {
        self.syst.is_counter_enabled()
    }

    /// Enable the SysTick interrupt
    #[inline]
    pub fn enable_interrupt(&mut self) {
        self.syst.enable_interrupt();
    }

    /// Disable the SysTick interrupt
    #[inline]
    pub fn disable_interrupt(&mut self) {
        self.syst.disable_interrupt();
    }

    /// Check if the interrupt is enabled
    #[inline]
    pub fn is_interrupt_enabled(&self) -> bool {
        self.syst.is_interrupt_enabled()
    }

    /// Set the clock source
    #[inline]
    pub fn set_clock_source(&mut self, source: SystClkSource) {
        self.syst.set_clock_source(source);
    }

    /// Set the reload value
    #[inline]
    pub fn set_reload(&mut self, value: u32) {
        self.syst.set_reload(value.min(SYSTICK_MAX_RELOAD));
    }

    /// Delay for a duration (busy-wait).
    pub fn delay(&self, duration: Duration) {
        assert!(
            self.freq_hz > 0,
            "SysTick was stolen without a clock frequency; delay is unavailable"
        );

        let mut remaining_us = duration.as_micros();
        while remaining_us > 0 {
            // Longest delay that fits in one 24-bit reload cycle.
            let chunk_us = remaining_us.min(self.max_chunk_us());
            self.delay_us_internal(chunk_us as u32);
            remaining_us -= chunk_us;
        }
    }

    /// Delay for a number of milliseconds.
    #[inline]
    pub fn delay_ms(&self, ms: u32) {
        self.delay(Duration::from_millis(ms as u64));
    }

    /// Delay for a number of microseconds.
    #[inline]
    pub fn delay_us(&self, us: u32) {
        self.delay(Duration::from_micros(us as u64));
    }

    /// Longest single-shot delay (in microseconds) that fits in one reload
    /// cycle at the configured clock rate.
    #[inline]
    fn max_chunk_us(&self) -> u64 {
        SYSTICK_MAX_RELOAD as u64 * 1_000_000 / self.freq_hz as u64
    }

    /// Busy-wait for up to one reload cycle.
    fn delay_us_internal(&self, us: u32) {
        // Round up so we never return early.
        let ticks = (us as u64 * self.freq_hz as u64).div_ceil(1_000_000);
        let ticks = ticks.min(SYSTICK_MAX_RELOAD as u64) as u32;
        if ticks == 0 {
            return;
        }

        let reload = SYST::reload();

        // Track the previous counter value to detect wraps.
        let mut last = SYST::current();
        let mut elapsed: u32 = 0;

        loop {
            let current = SYST::current();

            // SysTick counts down; accumulate the ticks that passed since the
            // previous read. `current > last` means the counter wrapped:
            // last -> 0 -> reload -> current.
            let delta = if current <= last {
                last - current
            } else {
                last + (reload - current) + 1
            };

            elapsed = elapsed.saturating_add(delta);
            last = current;

            if elapsed >= ticks {
                break;
            }

            // Yield to prevent tight spinning.
            rp2350_rt::asm::nop();
        }
    }

    /// Release the SYST peripheral
    #[inline]
    pub fn release(mut self) -> SYST {
        self.disable();
        self.syst
    }
}

/// A delay provider that implements embedded-hal DelayNs trait pattern
///
/// This is a convenience wrapper around SysTick for use with embedded-hal
/// compatible drivers.
pub struct Delay {
    systick: SysTick,
}

impl Delay {
    /// Initialize a Delay instance from an owned SYST peripheral.
    ///
    /// # Arguments
    /// * `syst` - The SYST peripheral from rp2350-rt
    /// * `freq_hz` - The clock frequency in Hz
    ///
    /// # Example
    /// ```ignore
    /// use rp2350_rt::Peripherals;
    /// use rp2350::systick::Delay;
    ///
    /// let cp = Peripherals::take().unwrap();
    /// let delay = Delay::init(cp.SYST, 150_000_000);
    /// delay.delay_ms(500);
    /// ```
    pub fn init(syst: SYST, freq_hz: u32) -> Self {
        Self {
            systick: SysTick::init(syst, freq_hz),
        }
    }

    /// Obtain a Delay handle without re-initializing.
    ///
    /// # Safety
    ///
    /// The caller must ensure the SysTick timer has already been initialized
    /// (e.g. via [`SysTick::init`]) before use. Prefer [`Delay::init`] which
    /// handles initialization automatically.
    #[inline]
    pub unsafe fn steal() -> Steal<Self> {
        Steal(Self {
            systick: unsafe { SysTick::steal().0 },
        })
    }

    /// Delay for nanoseconds (rounded to microseconds)
    #[inline]
    pub fn delay_ns(&self, ns: u32) {
        // Round up to microseconds
        let us = ns.div_ceil(1000);
        if us > 0 {
            self.systick.delay_us(us);
        }
    }

    /// Delay for microseconds
    #[inline]
    pub fn delay_us(&self, us: u32) {
        self.systick.delay_us(us);
    }

    /// Delay for milliseconds
    #[inline]
    pub fn delay_ms(&self, ms: u32) {
        self.systick.delay_ms(ms);
    }

    /// Get a reference to the underlying SysTick
    #[inline]
    pub fn systick(&self) -> &SysTick {
        &self.systick
    }

    /// Get a mutable reference to the underlying SysTick
    #[inline]
    pub fn systick_mut(&mut self) -> &mut SysTick {
        &mut self.systick
    }

    /// Release the underlying SYST peripheral
    #[inline]
    pub fn release(self) -> SYST {
        self.systick.release()
    }
}

// ============================================================================
// DelayUs trait implementations
// ============================================================================

impl crate::delay::DelayUs for SysTick {
    /// Busy-wait delay in microseconds, delegating to `SysTick::delay_us`
    #[inline]
    fn delay_us(&self, us: u32) {
        SysTick::delay_us(self, us);
    }
}

impl crate::delay::DelayUs for Delay {
    /// Busy-wait delay in microseconds, delegating to `Delay::delay_us`
    #[inline]
    fn delay_us(&self, us: u32) {
        Delay::delay_us(self, us);
    }
}
