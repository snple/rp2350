//! Delay abstraction trait
//!
//! Provides a unified [`DelayUs`] trait implemented by
//! [`crate::systick::SysTick`], [`crate::systick::Delay`],
//! [`crate::timer::Timer0`] and [`crate::timer::Timer1`], so upper-layer code
//! stays agnostic to the delay source.
//!
//! # Example
//!
//! ```ignore
//! use pico_rs::delay::DelayUs;
//! use pico_rs::timer::Timer0;
//! use pico_rs_pac::TIMER0;
//!
//! fn blink(delay: &impl DelayUs) {
//!     delay.delay_ms(500);
//! }
//!
//! let timer = Timer0::init(unsafe { TIMER0::steal() });
//! blink(&timer);
//! ```

/// Unified microsecond delay trait
///
/// Implemented by [`crate::systick::SysTick`], [`crate::systick::Delay`],
/// [`crate::timer::Timer0`] and [`crate::timer::Timer1`].
///
/// Upper-layer code can decouple from a concrete delay implementation via
/// generics or dynamic dispatch (`&dyn DelayUs`):
///
/// ```ignore
/// fn wait(delay: &impl DelayUs) {
///     delay.delay_ms(100);
/// }
/// ```
pub trait DelayUs {
    /// Delay for the given number of microseconds (busy-wait)
    fn delay_us(&self, us: u32);

    /// Delay for the given number of milliseconds
    ///
    /// The default implementation calls `delay_us(ms * 1_000)`; override it
    /// for better efficiency if needed.
    #[inline]
    fn delay_ms(&self, ms: u32) {
        self.delay_us(ms.saturating_mul(1_000));
    }

    /// Delay for the given number of seconds
    ///
    /// The default implementation loops calling `delay_ms(1_000)` to avoid
    /// overflow of `ms * 1_000`.
    #[inline]
    fn delay_secs(&self, secs: u32) {
        for _ in 0..secs {
            self.delay_ms(1_000);
        }
    }
}

impl<T: DelayUs> DelayUs for crate::Steal<T> {
    #[inline]
    fn delay_us(&self, us: u32) {
        self.0.delay_us(us)
    }
}
