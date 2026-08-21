//! 定时器抽象 trait。
//!
//! HAL（`rp2350`）不再内置 `MonotonicTimer` / `AlarmTimer` 等抽象 trait，
//! 需要抽象时由调用者（driver crate 或最终应用）自行定义并实现。
//!
//! 本 crate 的设备驱动（如 [`crate::stepper`]）依赖这两个 trait，因此在此
//! 定义，并为 RP2350 的 `Timer0` / `Timer1`（含 [`Steal`](rp2350::Steal)
//! 句柄）提供实现，保证驱动开箱即用。

use rp2350::timer::{Timer0, Timer1};

/// 单调递增的微秒时间源 trait，供驱动做超时检查或耗时测量。
///
/// 只要求一个自由运行的微秒时间源，与具体硬件计时器类型解耦。
pub trait MonotonicTimer {
    /// 当前 64 位微秒时间。
    fn time_us(&self) -> u64;

    /// 当前时间的低 32 位（更快，但约 71 分钟回绕一次）。
    fn time_us_32(&self) -> u32;
}

// Blanket 转发：让 stolen 句柄（`Steal<T>`）也能满足只要求 `&self` 方法的
// trait bound。通过 `Deref` 访问内层，无需访问 `Steal` 的私有字段。
impl<T: MonotonicTimer> MonotonicTimer for rp2350::Steal<T> {
    #[inline]
    fn time_us(&self) -> u64 {
        (**self).time_us()
    }

    #[inline]
    fn time_us_32(&self) -> u32 {
        (**self).time_us_32()
    }
}

/// 支持硬件 alarm 调度与中断控制的 timer trait。
///
/// 供需要 alarm 调度和 alarm 中断控制、但不想依赖具体计时器外设类型的代码使用。
pub trait AlarmTimer {
    /// Set an alarm to fire at a specific 32-bit timer value.
    fn set_alarm(&self, alarm: u8, time: u32);

    /// Set an alarm with race-condition protection.
    fn set_alarm_safe(&self, alarm: u8, time: u32);

    /// Check if an alarm is currently armed.
    fn is_alarm_armed(&self, alarm: u8) -> bool;

    /// Disarm an alarm.
    fn disarm_alarm(&self, alarm: u8);

    /// Enable interrupt delivery for an alarm.
    fn enable_alarm_interrupt(&self, alarm: u8);

    /// Disable interrupt delivery for an alarm.
    fn disable_alarm_interrupt(&self, alarm: u8);

    /// Check whether an alarm interrupt is enabled.
    fn is_alarm_interrupt_enabled(&self, alarm: u8) -> bool;

    /// Check raw alarm interrupt state before masking.
    fn alarm_interrupt_raw(&self, alarm: u8) -> bool;

    /// Check masked alarm interrupt state.
    fn alarm_interrupt_status(&self, alarm: u8) -> bool;

    /// Clear an alarm interrupt.
    fn clear_alarm_interrupt(&self, alarm: u8);

    /// Force an alarm interrupt.
    fn force_alarm_interrupt(&self, alarm: u8);

    /// Clear a forced alarm interrupt.
    fn clear_forced_alarm_interrupt(&self, alarm: u8);
}

impl<T: AlarmTimer> AlarmTimer for rp2350::Steal<T> {
    #[inline]
    fn set_alarm(&self, alarm: u8, time: u32) {
        (**self).set_alarm(alarm, time)
    }

    #[inline]
    fn set_alarm_safe(&self, alarm: u8, time: u32) {
        (**self).set_alarm_safe(alarm, time)
    }

    #[inline]
    fn is_alarm_armed(&self, alarm: u8) -> bool {
        (**self).is_alarm_armed(alarm)
    }

    #[inline]
    fn disarm_alarm(&self, alarm: u8) {
        (**self).disarm_alarm(alarm)
    }

    #[inline]
    fn enable_alarm_interrupt(&self, alarm: u8) {
        (**self).enable_alarm_interrupt(alarm)
    }

    #[inline]
    fn disable_alarm_interrupt(&self, alarm: u8) {
        (**self).disable_alarm_interrupt(alarm)
    }

    #[inline]
    fn is_alarm_interrupt_enabled(&self, alarm: u8) -> bool {
        (**self).is_alarm_interrupt_enabled(alarm)
    }

    #[inline]
    fn alarm_interrupt_raw(&self, alarm: u8) -> bool {
        (**self).alarm_interrupt_raw(alarm)
    }

    #[inline]
    fn alarm_interrupt_status(&self, alarm: u8) -> bool {
        (**self).alarm_interrupt_status(alarm)
    }

    #[inline]
    fn clear_alarm_interrupt(&self, alarm: u8) {
        (**self).clear_alarm_interrupt(alarm)
    }

    #[inline]
    fn force_alarm_interrupt(&self, alarm: u8) {
        (**self).force_alarm_interrupt(alarm)
    }

    #[inline]
    fn clear_forced_alarm_interrupt(&self, alarm: u8) {
        (**self).clear_forced_alarm_interrupt(alarm)
    }
}

// ============================================================================
// 为 RP2350 硬件计时器实现 trait
// ============================================================================

macro_rules! impl_timer_traits {
    ($Timer:ident) => {
        impl MonotonicTimer for $Timer {
            #[inline]
            fn time_us(&self) -> u64 {
                $Timer::time_us(self)
            }

            #[inline]
            fn time_us_32(&self) -> u32 {
                $Timer::time_us_32(self)
            }
        }

        impl AlarmTimer for $Timer {
            #[inline]
            fn set_alarm(&self, alarm: u8, time: u32) {
                $Timer::set_alarm(self, alarm, time);
            }

            #[inline]
            fn set_alarm_safe(&self, alarm: u8, time: u32) {
                $Timer::set_alarm_safe(self, alarm, time);
            }

            #[inline]
            fn is_alarm_armed(&self, alarm: u8) -> bool {
                $Timer::is_alarm_armed(self, alarm)
            }

            #[inline]
            fn disarm_alarm(&self, alarm: u8) {
                $Timer::disarm_alarm(self, alarm);
            }

            #[inline]
            fn enable_alarm_interrupt(&self, alarm: u8) {
                $Timer::enable_alarm_interrupt(self, alarm);
            }

            #[inline]
            fn disable_alarm_interrupt(&self, alarm: u8) {
                $Timer::disable_alarm_interrupt(self, alarm);
            }

            #[inline]
            fn is_alarm_interrupt_enabled(&self, alarm: u8) -> bool {
                $Timer::is_alarm_interrupt_enabled(self, alarm)
            }

            #[inline]
            fn alarm_interrupt_raw(&self, alarm: u8) -> bool {
                $Timer::alarm_interrupt_raw(self, alarm)
            }

            #[inline]
            fn alarm_interrupt_status(&self, alarm: u8) -> bool {
                $Timer::alarm_interrupt_status(self, alarm)
            }

            #[inline]
            fn clear_alarm_interrupt(&self, alarm: u8) {
                $Timer::clear_alarm_interrupt(self, alarm);
            }

            #[inline]
            fn force_alarm_interrupt(&self, alarm: u8) {
                $Timer::force_alarm_interrupt(self, alarm);
            }

            #[inline]
            fn clear_forced_alarm_interrupt(&self, alarm: u8) {
                $Timer::clear_forced_alarm_interrupt(self, alarm);
            }
        }
    };
}

impl_timer_traits!(Timer0);
impl_timer_traits!(Timer1);
