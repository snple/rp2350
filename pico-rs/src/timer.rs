//! Timer HAL for RP2350
//!
//! This module provides a safe interface for the RP2350 hardware timers.
//! The RP2350 has two 64-bit timer peripherals (TIMER0 and TIMER1), each with
//! 4 alarm comparators.
//!
//! ## Features
//!
//! - 64-bit microsecond time counter
//! - 4 alarm channels per timer
//! - Interrupt support
//!
//! ## Example
//!
//! ```ignore
//! use pico_rs::timer::Timer0;
//! use pico_rs_pac::TIMER0;
//!
//! // Get TIMER0 peripheral
//! let timer0_pac = unsafe { TIMER0::steal() };
//!
//! // Initialize timer 0
//! let timer = Timer0::init(timer0_pac);
//!
//! // Read current time
//! let now = timer.get_time_us();
//!
//! // Set an alarm to fire in 1 second
//! timer.set_alarm(0, now + 1_000_000);
//! timer.enable_alarm_interrupt(0);
//! ```

use crate::reset::Resettable;

/// Number of alarms per timer
pub const ALARM_COUNT: usize = 4;

/// Abstract monotonic timer trait for driver implementations.
///
/// This keeps drivers decoupled from concrete timer types when they only need
/// a free-running microsecond time source for timeout checks or elapsed-time
/// measurements.
pub trait MonotonicTimer {
    /// Get the current 64-bit time value in microseconds.
    fn get_time_us(&self) -> u64;

    /// Get the current time as the low 32 bits only.
    fn get_time_us_32(&self) -> u32;
}

// Blanket trait forwarding for Steal<T> — allows stolen handles to satisfy
// trait bounds that only require `&self` methods.

impl<T: MonotonicTimer> MonotonicTimer for crate::Steal<T> {
    #[inline]
    fn get_time_us(&self) -> u64 {
        self.0.get_time_us()
    }
    #[inline]
    fn get_time_us_32(&self) -> u32 {
        self.0.get_time_us_32()
    }
}

/// Abstract alarm-capable timer trait for driver implementations.
///
/// This is intended for code that needs hardware alarm scheduling and alarm
/// interrupt control without depending on a specific timer peripheral type.
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

impl<T: AlarmTimer> AlarmTimer for crate::Steal<T> {
    #[inline]
    fn set_alarm(&self, alarm: u8, time: u32) {
        self.0.set_alarm(alarm, time)
    }
    #[inline]
    fn set_alarm_safe(&self, alarm: u8, time: u32) {
        self.0.set_alarm_safe(alarm, time)
    }
    #[inline]
    fn is_alarm_armed(&self, alarm: u8) -> bool {
        self.0.is_alarm_armed(alarm)
    }
    #[inline]
    fn disarm_alarm(&self, alarm: u8) {
        self.0.disarm_alarm(alarm)
    }
    #[inline]
    fn enable_alarm_interrupt(&self, alarm: u8) {
        self.0.enable_alarm_interrupt(alarm)
    }
    #[inline]
    fn disable_alarm_interrupt(&self, alarm: u8) {
        self.0.disable_alarm_interrupt(alarm)
    }
    #[inline]
    fn is_alarm_interrupt_enabled(&self, alarm: u8) -> bool {
        self.0.is_alarm_interrupt_enabled(alarm)
    }
    #[inline]
    fn alarm_interrupt_raw(&self, alarm: u8) -> bool {
        self.0.alarm_interrupt_raw(alarm)
    }
    #[inline]
    fn alarm_interrupt_status(&self, alarm: u8) -> bool {
        self.0.alarm_interrupt_status(alarm)
    }
    #[inline]
    fn clear_alarm_interrupt(&self, alarm: u8) {
        self.0.clear_alarm_interrupt(alarm)
    }
    #[inline]
    fn force_alarm_interrupt(&self, alarm: u8) {
        self.0.force_alarm_interrupt(alarm)
    }
    #[inline]
    fn clear_forced_alarm_interrupt(&self, alarm: u8) {
        self.0.clear_forced_alarm_interrupt(alarm)
    }
}

/// Macro to generate Timer types for both TIMER0 and TIMER1
macro_rules! impl_timer {
    ($TimerName:ident, $timer_mod:ident, $TIMER:ident) => {
        /// Timer peripheral wrapper
        pub struct $TimerName {
            timer: pico_rs_pac::$TIMER,
        }

        impl $TimerName {
            /// Initialize the timer peripheral
            ///
            /// This brings the timer out of reset and returns the timer ready for use.
            ///
            /// **Note**: The corresponding tick generator in the TICKS block must be
            /// configured and enabled before using the timer. For example, call
            /// [`Ticks::init_timer0()`] or [`Ticks::init_timer1()`] for the
            /// corresponding timer.
            ///
            /// # Arguments
            /// * `timer` - The Timer peripheral from PAC
            pub fn init(timer: pico_rs_pac::$TIMER) -> Self {
                <pico_rs_pac::$TIMER as Resettable>::init();
                Self { timer }
            }

            /// Release the Timer peripheral
            #[inline]
            pub fn release(self) -> pico_rs_pac::$TIMER {
                self.timer
            }

            /// Create a Timer handle without ownership of the PAC peripheral.
            ///
            /// This is analogous to [`Pin::new()`](crate::gpio::Pin::new) —
            /// it allows ISR or other contexts to access the timer registers
            /// when the real owner lives elsewhere (e.g. in `main`).
            ///
            /// # Safety
            ///
            /// The caller must ensure that:
            /// - The timer has already been initialised (via [`init()`](Self::init)
            ///   or equivalent).
            /// - No conflicting ownership exists — i.e. the stolen handle is not
            ///   used to release or re-initialise the peripheral.
            #[inline]
            pub unsafe fn steal() -> crate::Steal<Self> {
                crate::Steal(Self {
                    timer: unsafe { pico_rs_pac::$TIMER::steal() },
                })
            }

            /// Get reference to the underlying register block
            ///
            /// # Safety
            /// Direct register access can bypass the HAL's safety guarantees.
            /// The caller must ensure operations do not conflict with the HAL state.
            #[inline]
            pub unsafe fn regs(&self) -> &pico_rs_pac::$timer_mod::RegisterBlock {
                &self.timer
            }

            // ========================================================================
            // Time Reading
            // ========================================================================

            /// Get the current 64-bit time value in microseconds
            ///
            /// This reads both the low and high words atomically.
            #[inline]
            pub fn get_time_us(&self) -> u64 {
                // Read low first, which latches high
                let lo = self.timer.timelr.timelr();
                let hi = self.timer.timehr.timehr();
                ((hi as u64) << 32) | (lo as u64)
            }

            /// Get the current time (low 32 bits only)
            ///
            /// This is faster but wraps every ~71 minutes.
            #[inline]
            pub fn get_time_us_32(&self) -> u32 {
                self.timer.timerawl.timerawl()
            }

            /// Get raw time without side effects
            ///
            /// Unlike `get_time_us()`, this doesn't latch the high word.
            #[inline]
            pub fn get_time_raw(&self) -> (u32, u32) {
                let lo = self.timer.timerawl.timerawl();
                let hi = self.timer.timerawh.timerawh();
                (lo, hi)
            }

            // ========================================================================
            // Time Writing
            // ========================================================================

            /// Set the 64-bit time value
            ///
            /// Note: Write low word first, then high word.
            #[inline]
            pub fn set_time_us(&self, time: u64) {
                let lo = time as u32;
                let hi = (time >> 32) as u32;
                // Write low first, then high (latches on high write)
                self.timer.timelw.write_timelw(lo);
                self.timer.timehw.write_timehw(hi);
            }

            // ========================================================================
            // Alarms
            // ========================================================================

            /// Set an alarm to fire at a specific time
            ///
            /// Writing to the alarm register arms the alarm automatically.
            ///
            /// # Arguments
            /// * `alarm` - Alarm number (0-3)
            /// * `time` - Time in microseconds when alarm should fire (low 32 bits)
            #[inline]
            pub fn set_alarm(&self, alarm: u8, time: u32) {
                assert!(alarm < 4, "Alarm must be 0-3");
                match alarm {
                    0 => self.timer.alarm0.write_alarm0(time),
                    1 => self.timer.alarm1.write_alarm1(time),
                    2 => self.timer.alarm2.write_alarm2(time),
                    3 => self.timer.alarm3.write_alarm3(time),
                    _ => unreachable!(),
                }
            }

            /// Set an alarm with race-condition protection
            ///
            /// RP2350 timer alarms fire on exact match: when `TIMERAWL == ALARM`.
            /// If the target time has already passed by the time the alarm register
            /// is written, the alarm will never fire (it would take ~71 minutes for
            /// the 32-bit counter to wrap around and match again).
            ///
            /// This method writes the alarm and then immediately checks whether the
            /// target was missed:
            /// - If the target is still in the future → normal, hardware will fire.
            /// - If the target is in the past and the alarm is still armed →
            ///   disarms the stale alarm and forces an interrupt via INTF.
            ///
            /// **Note**: The corresponding ISR must clear both INTR and INTF at
            /// entry to prevent infinite re-entry from the forced interrupt:
            /// ```ignore
            /// timer.clear_alarm_interrupt(n);
            /// timer.clear_forced_alarm_interrupt(n);
            /// ```
            ///
            /// The "past vs future" check uses unsigned wrapping subtraction:
            /// `now.wrapping_sub(target) < 0x8000_0000` is true when `target` is
            /// in the past half of the 32-bit number circle (within ~35 minutes).
            /// This is safe for any interval shorter than ~35 minutes.
            ///
            /// # Arguments
            /// * `alarm` - Alarm number (0-3)
            /// * `time`  - Target time in microseconds (low 32 bits)
            #[inline]
            pub fn set_alarm_safe(&self, alarm: u8, time: u32) {
                self.set_alarm(alarm, time);
                let now = self.get_time_us_32();
                if now.wrapping_sub(time) < 0x8000_0000 {
                    if self.is_alarm_armed(alarm) {
                        self.disarm_alarm(alarm);
                        self.force_alarm_interrupt(alarm);
                    }
                }
            }

            /// Check if an alarm is armed
            #[inline]
            pub fn is_alarm_armed(&self, alarm: u8) -> bool {
                assert!(alarm < 4, "Alarm must be 0-3");
                (self.timer.armed.armed() & (1 << alarm)) != 0
            }

            /// Disarm an alarm (cancel it before it fires)
            ///
            /// Write 1 to the corresponding bit to disarm.
            #[inline]
            pub fn disarm_alarm(&self, alarm: u8) {
                assert!(alarm < 4, "Alarm must be 0-3");
                self.timer.armed.write_armed(1 << alarm);
            }

            /// Disarm all alarms
            #[inline]
            pub fn disarm_all_alarms(&self) {
                self.timer.armed.write_armed(0xf);
            }

            /// Get armed status of all alarms
            #[inline]
            pub fn armed_status(&self) -> u8 {
                (self.timer.armed.armed() & 0xf) as u8
            }

            // ========================================================================
            // Interrupts
            // ========================================================================

            /// Enable interrupt for an alarm
            #[inline]
            pub fn enable_alarm_interrupt(&self, alarm: u8) {
                assert!(alarm < 4, "Alarm must be 0-3");
                match alarm {
                    0 => self.timer.inte.set_alarm_0(),
                    1 => self.timer.inte.set_alarm_1(),
                    2 => self.timer.inte.set_alarm_2(),
                    3 => self.timer.inte.set_alarm_3(),
                    _ => unreachable!(),
                }
            }

            /// Disable interrupt for an alarm
            #[inline]
            pub fn disable_alarm_interrupt(&self, alarm: u8) {
                assert!(alarm < 4, "Alarm must be 0-3");
                match alarm {
                    0 => self.timer.inte.clr_alarm_0(),
                    1 => self.timer.inte.clr_alarm_1(),
                    2 => self.timer.inte.clr_alarm_2(),
                    3 => self.timer.inte.clr_alarm_3(),
                    _ => unreachable!(),
                }
            }

            /// Enable all alarm interrupts
            #[inline]
            pub fn enable_all_alarm_interrupts(&self) {
                self.timer.inte.set_alarm_0();
                self.timer.inte.set_alarm_1();
                self.timer.inte.set_alarm_2();
                self.timer.inte.set_alarm_3();
            }

            /// Disable all alarm interrupts
            #[inline]
            pub fn disable_all_alarm_interrupts(&self) {
                self.timer.inte.clr_alarm_0();
                self.timer.inte.clr_alarm_1();
                self.timer.inte.clr_alarm_2();
                self.timer.inte.clr_alarm_3();
            }

            /// Check if an alarm interrupt is enabled
            #[inline]
            pub fn is_alarm_interrupt_enabled(&self, alarm: u8) -> bool {
                assert!(alarm < 4, "Alarm must be 0-3");
                match alarm {
                    0 => self.timer.inte.alarm_0(),
                    1 => self.timer.inte.alarm_1(),
                    2 => self.timer.inte.alarm_2(),
                    3 => self.timer.inte.alarm_3(),
                    _ => unreachable!(),
                }
            }

            /// Check if an alarm interrupt is pending (raw, before masking)
            #[inline]
            pub fn alarm_interrupt_raw(&self, alarm: u8) -> bool {
                assert!(alarm < 4, "Alarm must be 0-3");
                match alarm {
                    0 => self.timer.intr.alarm_0(),
                    1 => self.timer.intr.alarm_1(),
                    2 => self.timer.intr.alarm_2(),
                    3 => self.timer.intr.alarm_3(),
                    _ => unreachable!(),
                }
            }

            /// Check if an alarm interrupt is pending (after masking)
            #[inline]
            pub fn alarm_interrupt_status(&self, alarm: u8) -> bool {
                assert!(alarm < 4, "Alarm must be 0-3");
                match alarm {
                    0 => self.timer.ints.alarm_0(),
                    1 => self.timer.ints.alarm_1(),
                    2 => self.timer.ints.alarm_2(),
                    3 => self.timer.ints.alarm_3(),
                    _ => unreachable!(),
                }
            }

            /// Clear an alarm interrupt (write 1 to clear)
            ///
            /// The INTR register uses write-1-to-clear semantics, so we
            /// must directly write the bit mask, not use atomic set.
            #[inline]
            pub fn clear_alarm_interrupt(&self, alarm: u8) {
                assert!(alarm < 4, "Alarm must be 0-3");
                // Write 1 to the specific alarm bit to clear the interrupt
                // This is a W1C register, so we write directly
                self.timer.intr.write_bits(1 << alarm);
            }

            /// Clear all alarm interrupts
            #[inline]
            pub fn clear_all_alarm_interrupts(&self) {
                // Write 0xF to clear all 4 alarm bits
                self.timer.intr.write_bits(0xF);
            }

            /// Force an alarm interrupt (for testing)
            #[inline]
            pub fn force_alarm_interrupt(&self, alarm: u8) {
                assert!(alarm < 4, "Alarm must be 0-3");
                match alarm {
                    0 => self.timer.intf.set_alarm_0(),
                    1 => self.timer.intf.set_alarm_1(),
                    2 => self.timer.intf.set_alarm_2(),
                    3 => self.timer.intf.set_alarm_3(),
                    _ => unreachable!(),
                }
            }

            /// Clear forced alarm interrupt
            #[inline]
            pub fn clear_forced_alarm_interrupt(&self, alarm: u8) {
                assert!(alarm < 4, "Alarm must be 0-3");
                match alarm {
                    0 => self.timer.intf.clr_alarm_0(),
                    1 => self.timer.intf.clr_alarm_1(),
                    2 => self.timer.intf.clr_alarm_2(),
                    3 => self.timer.intf.clr_alarm_3(),
                    _ => unreachable!(),
                }
            }

            // ========================================================================
            // Control
            // ========================================================================

            /// Pause the timer
            #[inline]
            pub fn pause(&self) {
                self.timer.pause.set_pause();
            }

            /// Resume the timer
            #[inline]
            pub fn resume(&self) {
                self.timer.pause.clr_pause();
            }

            /// Check if the timer is paused
            #[inline]
            pub fn is_paused(&self) -> bool {
                self.timer.pause.pause()
            }

            /// Lock the timer (disable write access)
            ///
            /// Once locked, the timer cannot be unlocked until reset.
            #[inline]
            pub fn lock(&mut self) {
                self.timer.locked.set_locked();
            }

            /// Check if the timer is locked
            #[inline]
            pub fn is_locked(&self) -> bool {
                self.timer.locked.locked()
            }

            // ========================================================================
            // Delay Functions
            // ========================================================================

            /// Delay for a number of microseconds
            ///
            /// This is a busy-wait delay using the hardware timer.
            /// The timer runs at 1 MHz (1 tick = 1 microsecond).
            #[inline]
            pub fn delay_us(&self, us: u64) {
                let target = self.get_time_us().wrapping_add(us);
                while self.get_time_us() < target {
                    pico_rs_rt::asm::nop();
                }
            }

            /// Delay for a number of milliseconds
            #[inline]
            pub fn delay_ms(&self, ms: u32) {
                self.delay_us(ms as u64 * 1000);
            }

            /// Delay for a number of seconds
            #[inline]
            pub fn delay_secs(&self, secs: u32) {
                self.delay_us(secs as u64 * 1_000_000);
            }

            /// Delay until a specific time (in microseconds)
            ///
            /// If the target time has already passed, returns immediately.
            #[inline]
            pub fn delay_until(&self, target_us: u64) {
                while self.get_time_us() < target_us {
                    pico_rs_rt::asm::nop();
                }
            }
        }

        impl crate::delay::DelayUs for $TimerName {
            /// 忙等待延时（微秒），使用硬件定时器，精度高于 SysTick
            #[inline]
            fn delay_us(&self, us: u32) {
                $TimerName::delay_us(self, us as u64);
            }
        }

        impl MonotonicTimer for $TimerName {
            #[inline]
            fn get_time_us(&self) -> u64 {
                $TimerName::get_time_us(self)
            }

            #[inline]
            fn get_time_us_32(&self) -> u32 {
                $TimerName::get_time_us_32(self)
            }
        }

        impl AlarmTimer for $TimerName {
            #[inline]
            fn set_alarm(&self, alarm: u8, time: u32) {
                $TimerName::set_alarm(self, alarm, time);
            }

            #[inline]
            fn set_alarm_safe(&self, alarm: u8, time: u32) {
                $TimerName::set_alarm_safe(self, alarm, time);
            }

            #[inline]
            fn is_alarm_armed(&self, alarm: u8) -> bool {
                $TimerName::is_alarm_armed(self, alarm)
            }

            #[inline]
            fn disarm_alarm(&self, alarm: u8) {
                $TimerName::disarm_alarm(self, alarm);
            }

            #[inline]
            fn enable_alarm_interrupt(&self, alarm: u8) {
                $TimerName::enable_alarm_interrupt(self, alarm);
            }

            #[inline]
            fn disable_alarm_interrupt(&self, alarm: u8) {
                $TimerName::disable_alarm_interrupt(self, alarm);
            }

            #[inline]
            fn is_alarm_interrupt_enabled(&self, alarm: u8) -> bool {
                $TimerName::is_alarm_interrupt_enabled(self, alarm)
            }

            #[inline]
            fn alarm_interrupt_raw(&self, alarm: u8) -> bool {
                $TimerName::alarm_interrupt_raw(self, alarm)
            }

            #[inline]
            fn alarm_interrupt_status(&self, alarm: u8) -> bool {
                $TimerName::alarm_interrupt_status(self, alarm)
            }

            #[inline]
            fn clear_alarm_interrupt(&self, alarm: u8) {
                $TimerName::clear_alarm_interrupt(self, alarm);
            }

            #[inline]
            fn force_alarm_interrupt(&self, alarm: u8) {
                $TimerName::force_alarm_interrupt(self, alarm);
            }

            #[inline]
            fn clear_forced_alarm_interrupt(&self, alarm: u8) {
                $TimerName::clear_forced_alarm_interrupt(self, alarm);
            }
        }
    };
}

// Generate Timer0 and Timer1 types
impl_timer!(Timer0, timer0, TIMER0);
impl_timer!(Timer1, timer1, TIMER1);
