//! SysTick timer driver for Cortex-M33
//!
//! The SysTick timer is a 24-bit down counter built into all Cortex-M processors.
//! This module provides a safe interface using the cortex-m crate's SYST peripheral.
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
use cortex_m::peripheral::SYST;
use cortex_m::peripheral::syst::SystClkSource;

/// Maximum reload value (24-bit counter)
const SYSTICK_MAX_RELOAD: u32 = 0x00FF_FFFF;

/// Global tick counter for overflow tracking (used with interrupt mode)
static TICK_COUNT: AtomicU32 = AtomicU32::new(0);

/// SysTick timer driver
///
/// This driver takes ownership of the cortex-m SYST peripheral and provides
/// convenient delay functions.
pub struct SysTick {
    /// The underlying SYST peripheral (ownership or borrowed)
    _syst: Option<SYST>,
    /// Clock frequency in Hz
    freq_hz: u32,
    /// Ticks per microsecond
    ticks_per_us: u32,
}

impl SysTick {
    /// Create a new SysTick timer with the given clock frequency
    ///
    /// # Arguments
    /// * `syst` - The SYST peripheral from cortex-m
    /// * `freq_hz` - The clock frequency in Hz (typically the CPU clock)
    ///
    /// # Example
    /// ```ignore
    /// use cortex_m::Peripherals;
    /// use pico_rs::systick::SysTick;
    ///
    /// let cp = Peripherals::take().unwrap();
    /// let systick = SysTick::new(cp.SYST, 150_000_000);
    /// ```
    pub fn new(mut syst: SYST, freq_hz: u32) -> Self {
        let ticks_per_us = freq_hz / 1_000_000;

        // Configure SysTick
        syst.disable_counter();
        syst.disable_interrupt();
        syst.set_clock_source(SystClkSource::Core);
        syst.clear_current();
        syst.set_reload(SYSTICK_MAX_RELOAD);
        syst.enable_counter();

        Self {
            _syst: Some(syst),
            freq_hz,
            ticks_per_us,
        }
    }

    /// Create a new SysTick timer using raw pointer access
    ///
    /// # Safety
    /// The caller must ensure no other code is using the SysTick peripheral.
    ///
    /// # Arguments
    /// * `freq_hz` - The clock frequency in Hz (typically the CPU clock)
    pub unsafe fn new_unchecked(freq_hz: u32) -> Self {
        let syst_ptr = SYST::PTR;
        let ticks_per_us = freq_hz / 1_000_000;

        // Configure SysTick using raw pointer access
        // SAFETY: Caller guarantees exclusive access to SysTick
        unsafe {
            (*syst_ptr).csr.write(0); // Disable
            (*syst_ptr).cvr.write(0); // Clear current
            (*syst_ptr).rvr.write(SYSTICK_MAX_RELOAD);
            // Enable with processor clock, no interrupt
            (*syst_ptr).csr.write(0b101); // ENABLE | CLKSOURCE
        }

        Self {
            _syst: None,
            freq_hz,
            ticks_per_us,
        }
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
    pub fn has_wrapped(&self) -> bool {
        // Read CSR - the COUNTFLAG bit is cleared by reading it
        let syst = unsafe { &*SYST::PTR };
        (syst.csr.read() & (1 << 16)) != 0
    }

    /// Get the calibration value (if available)
    #[inline]
    pub fn calibration(&self) -> Option<u32> {
        let calib = SYST::get_ticks_per_10ms();
        if calib == 0 { None } else { Some(calib) }
    }

    /// Enable the SysTick counter
    #[inline]
    pub fn enable(&mut self) {
        let syst = unsafe { &*SYST::PTR };
        unsafe {
            syst.csr.modify(|v| v | 1); // Set ENABLE bit
        }
    }

    /// Disable the SysTick counter
    #[inline]
    pub fn disable(&mut self) {
        let syst = unsafe { &*SYST::PTR };
        unsafe {
            syst.csr.modify(|v| v & !1); // Clear ENABLE bit
        }
    }

    /// Check if the counter is enabled
    #[inline]
    pub fn is_enabled(&self) -> bool {
        let syst = unsafe { &*SYST::PTR };
        (syst.csr.read() & 1) != 0
    }

    /// Enable the SysTick interrupt
    #[inline]
    pub fn enable_interrupt(&mut self) {
        let syst = unsafe { &*SYST::PTR };
        unsafe {
            syst.csr.modify(|v| v | (1 << 1)); // Set TICKINT bit
        }
    }

    /// Disable the SysTick interrupt
    #[inline]
    pub fn disable_interrupt(&mut self) {
        let syst = unsafe { &*SYST::PTR };
        unsafe {
            syst.csr.modify(|v| v & !(1 << 1)); // Clear TICKINT bit
        }
    }

    /// Check if the interrupt is enabled
    #[inline]
    pub fn is_interrupt_enabled(&self) -> bool {
        let syst = unsafe { &*SYST::PTR };
        (syst.csr.read() & (1 << 1)) != 0
    }

    /// Set the clock source
    #[inline]
    pub fn set_clock_source(&mut self, source: SystClkSource) {
        let syst = unsafe { &*SYST::PTR };
        unsafe {
            match source {
                SystClkSource::Core => syst.csr.modify(|v| v | (1 << 2)),
                SystClkSource::External => syst.csr.modify(|v| v & !(1 << 2)),
            }
        }
    }

    /// Set the reload value
    #[inline]
    pub fn set_reload(&mut self, value: u32) {
        let syst = unsafe { &*SYST::PTR };
        unsafe {
            syst.rvr.write(value.min(SYSTICK_MAX_RELOAD));
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

        // Read start value
        let start = SYST::get_current();

        // Since SysTick counts down, we wait until enough ticks have passed
        loop {
            let current = SYST::get_current();
            let reload = SYST::get_reload();

            // Handle wrap-around: counter reloads to RVR when it hits 0
            let elapsed = if current <= start {
                start - current
            } else {
                // Wrapped: start -> 0 -> reload -> current
                start + (reload - current) + 1
            };

            if elapsed >= ticks {
                break;
            }

            // Small yield to prevent tight spinning
            cortex_m::asm::nop();
        }
    }

    /// Release the SYST peripheral (only if it was taken with `new`)
    #[inline]
    pub fn release(mut self) -> Option<SYST> {
        self.disable();
        self._syst.take()
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
    /// Create a new Delay instance
    ///
    /// # Arguments
    /// * `syst` - The SYST peripheral from cortex-m
    /// * `freq_hz` - The clock frequency in Hz
    pub fn new(syst: SYST, freq_hz: u32) -> Self {
        Self {
            systick: SysTick::new(syst, freq_hz),
        }
    }

    /// Create a new Delay instance using raw pointer access
    ///
    /// # Safety
    /// The caller must ensure no other code is using the SysTick peripheral.
    pub unsafe fn new_unchecked(freq_hz: u32) -> Self {
        Self {
            systick: unsafe { SysTick::new_unchecked(freq_hz) },
        }
    }

    /// Delay for nanoseconds (rounded to microseconds)
    #[inline]
    pub fn delay_ns(&mut self, ns: u32) {
        // Round up to microseconds
        let us = ns.div_ceil(1000);
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

    /// Release the underlying SysTick
    #[inline]
    pub fn release(self) -> SysTick {
        self.systick
    }
}

// ============================================================================
// Global tick counter functions (for use with SysTick interrupt)
// ============================================================================

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

// ============================================================================
// Busy-wait delay using simple counter loop
// ============================================================================

/// Busy-wait delay using simple counter loop (less accurate but always works)
///
/// Useful when SysTick is not yet configured or for very short delays.
#[inline]
pub fn delay_cycles(cycles: u32) {
    for _ in 0..cycles {
        cortex_m::asm::nop();
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
