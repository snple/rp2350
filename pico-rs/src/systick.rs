//! SysTick timer driver for Cortex-M33
//!
//! The SysTick timer is a 24-bit down counter built into all Cortex-M processors.
//! This module provides a safe interface using the pico-rs-rt SYST peripheral.
//!
//! # Example
//!
//! ```ignore
//! use pico_rs_rt::Peripherals;
//! use pico_rs::systick::SysTick;
//! use pico_rs::time::Duration;
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
use pico_rs_rt::peripheral::syst::{self, SYST, SystClkSource};

/// Maximum reload value (24-bit counter)
const SYSTICK_MAX_RELOAD: u32 = syst::MAX_RELOAD;

/// SysTick timer driver
///
/// This driver takes ownership of the pico-rs-rt SYST peripheral and provides
/// convenient delay functions.
pub struct SysTick {
    /// The underlying SYST peripheral
    syst: SYST,
    /// Clock frequency in Hz
    freq_hz: u32,
    /// Ticks per microsecond
    ticks_per_us: u32,
}

impl SysTick {
    /// Initialize SysTick timer with the given clock frequency.
    ///
    /// Configures the SysTick counter with maximum reload and enables it.
    ///
    /// # Arguments
    /// * `syst` - The SYST peripheral from pico-rs-rt
    /// * `freq_hz` - The clock frequency in Hz (typically the CPU clock)
    ///
    /// # Example
    /// ```ignore
    /// use pico_rs_rt::Peripherals;
    /// use pico_rs::systick::SysTick;
    ///
    /// let cp = Peripherals::take().unwrap();
    /// let systick = SysTick::init(cp.SYST, 150_000_000);
    /// ```
    pub fn init(mut syst: SYST, freq_hz: u32) -> Self {
        let ticks_per_us = freq_hz / 1_000_000;

        // Configure SysTick
        syst.disable_counter();
        syst.disable_interrupt();
        syst.set_clock_source(SystClkSource::Core);
        syst.clear_current();
        syst.set_reload(SYSTICK_MAX_RELOAD);
        syst.enable_counter();

        Self {
            syst,
            freq_hz,
            ticks_per_us,
        }
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
            ticks_per_us: 0,
        })
    }

    /// Get the clock frequency
    #[inline]
    pub fn freq_hz(&self) -> u32 {
        self.freq_hz
    }

    /// Get the ticks per microsecond
    #[inline]
    pub fn ticks_per_us(&self) -> u32 {
        self.ticks_per_us
    }

    /// Read the current counter value (counts down from reload value)
    #[inline]
    pub fn read_counter(&self) -> u32 {
        SYST::get_current()
    }

    /// Read the reload value
    #[inline]
    pub fn read_reload(&self) -> u32 {
        SYST::get_reload()
    }

    /// Check if the counter has reached zero since last check
    #[inline]
    pub fn has_wrapped(&mut self) -> bool {
        // Read CSR - the COUNTFLAG bit is cleared by reading it
        self.syst.has_wrapped()
    }

    /// Get the calibration value (if available)
    #[inline]
    pub fn calibration(&self) -> Option<u32> {
        SYST::get_ticks_per_10ms()
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
    pub fn enabled(&self) -> bool {
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

    /// Delay for a duration
    pub fn delay(&self, duration: Duration) {
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
    pub fn delay_ms(&self, ms: u32) {
        self.delay(Duration::from_millis(ms as u64));
    }

    /// Delay for a number of microseconds
    #[inline]
    pub fn delay_us(&self, us: u32) {
        if us == 0 {
            return;
        }
        self.delay_us_internal(us);
    }

    /// Internal delay implementation for up to max reload ticks
    fn delay_us_internal(&self, us: u32) {
        let ticks = us.saturating_mul(self.ticks_per_us);
        if ticks == 0 {
            return;
        }

        // Clamp to max reload value
        let ticks = ticks.min(SYSTICK_MAX_RELOAD);

        let reload = SYST::get_reload();

        // Read start value and track previous value for wrap detection
        let mut last = SYST::get_current();
        let mut elapsed: u32 = 0;

        // Since SysTick counts down, we accumulate elapsed ticks
        // by detecting when the counter wraps (current > last means wrap occurred)
        loop {
            let current = SYST::get_current();

            // Calculate ticks since last read, handling wrap-around
            let delta = if current <= last {
                // Normal case: counter decreased
                last - current
            } else {
                // Wrap occurred: last -> 0 -> reload -> current
                last + (reload - current) + 1
            };

            elapsed = elapsed.saturating_add(delta);
            last = current;

            if elapsed >= ticks {
                break;
            }

            // Small yield to prevent tight spinning
            pico_rs_rt::asm::nop();
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
    /// * `syst` - The SYST peripheral from pico-rs-rt
    /// * `freq_hz` - The clock frequency in Hz
    ///
    /// # Example
    /// ```ignore
    /// use pico_rs_rt::Peripherals;
    /// use pico_rs::systick::Delay;
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
// Busy-wait delay using simple counter loop
// ============================================================================

// ============================================================================
// DelayUs trait implementations
// ============================================================================

impl crate::delay::DelayUs for SysTick {
    /// 忙等待延时（微秒），委托给 `SysTick::delay_us`
    #[inline]
    fn delay_us(&self, us: u32) {
        SysTick::delay_us(self, us);
    }
}

impl crate::delay::DelayUs for Delay {
    /// 忙等待延时（微秒），委托给 `Delay::delay_us`
    #[inline]
    fn delay_us(&self, us: u32) {
        Delay::delay_us(self, us);
    }
}

// ============================================================================
// Busy-wait delay using simple counter loop
// ============================================================================

/// Busy-wait delay using simple counter loop (less accurate but always works)
///
/// Useful when SysTick is not yet configured or for very short delays.
#[inline]
pub fn delay_cycles(cycles: u32) {
    for _ in 0..cycles {
        pico_rs_rt::asm::nop();
    }
}

/// Approximate delay in microseconds using cycle counting
///
/// This is a rough estimate and depends on the CPU clock frequency.
/// For accurate timing, use SysTick instead.
#[inline]
pub fn delay_us_approx(us: u32, cpu_freq_mhz: u32) {
    // Rough estimate: ~3 cycles per loop iteration
    let cycles = us.saturating_mul(cpu_freq_mhz) / 3;
    delay_cycles(cycles);
}
