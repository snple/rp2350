//! Delay abstraction trait
//!
//! 提供统一的 [`DelayUs`] trait，由 [`crate::systick::SysTick`]、
//! [`crate::systick::Delay`]、[`crate::timer::Timer0`] 和
//! [`crate::timer::Timer1`] 共同实现，使上层代码可以对延时来源保持无感知。
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

/// 统一的微秒级延时 trait
///
/// 该 trait 由 [`crate::systick::SysTick`]、[`crate::systick::Delay`]、
/// [`crate::timer::Timer0`] 和 [`crate::timer::Timer1`] 实现。
///
/// 上层代码可通过泛型或动态分发（`&dyn DelayUs`）与具体延时实现解耦：
///
/// ```ignore
/// fn wait(delay: &impl DelayUs) {
///     delay.delay_ms(100);
/// }
/// ```
pub trait DelayUs {
    /// 延时指定微秒数（忙等待）
    fn delay_us(&self, us: u32);

    /// 延时指定毫秒数
    ///
    /// 默认实现调用 `delay_us(ms * 1_000)`，可按需覆盖以提高效率。
    #[inline]
    fn delay_ms(&self, ms: u32) {
        self.delay_us(ms.saturating_mul(1_000));
    }

    /// 延时指定秒数
    ///
    /// 默认实现循环调用 `delay_ms(1_000)`，避免 `ms * 1_000` 溢出。
    #[inline]
    fn delay_secs(&self, secs: u32) {
        for _ in 0..secs {
            self.delay_ms(1_000);
        }
    }
}
