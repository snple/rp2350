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
//!
//! // Initialize timer 0
//! let timer = Timer0::init();
//!
//! // Read current time
//! let now = timer.get_time_us();
//!
//! // Set an alarm to fire in 1 second
//! timer.set_alarm(0, now + 1_000_000);
//! timer.enable_alarm_interrupt(0);
//! ```

use crate::reset::{self, Peripheral};

/// Number of alarms per timer
pub const ALARM_COUNT: usize = 4;

/// Macro to generate Timer types for both TIMER0 and TIMER1
macro_rules! impl_timer {
    ($TimerName:ident, $timer_mod:ident, $TIMER:ident, $reset_peripheral:expr) => {
        /// Timer peripheral wrapper
        pub struct $TimerName {
            regs: &'static pico_rs_pac::$timer_mod::RegisterBlock,
        }

        impl $TimerName {
            /// Create a new Timer instance without initialization
            #[inline]
            pub fn new() -> Self {
                Self {
                    regs: unsafe { &*pico_rs_pac::$TIMER::PTR },
                }
            }

            /// Initialize the timer peripheral
            ///
            /// This brings the timer out of reset and enables it.
            pub fn init() -> Self {
                // Bring timer out of reset
                reset::unreset_block_wait($reset_peripheral);
                Self::new()
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
                let lo = self.regs.timelr.timelr();
                let hi = self.regs.timehr.timehr();
                ((hi as u64) << 32) | (lo as u64)
            }

            /// Get the current time (low 32 bits only)
            ///
            /// This is faster but wraps every ~71 minutes.
            #[inline]
            pub fn get_time_us_32(&self) -> u32 {
                self.regs.timerawl.timerawl()
            }

            /// Get raw time without side effects
            ///
            /// Unlike `get_time_us()`, this doesn't latch the high word.
            #[inline]
            pub fn get_time_raw(&self) -> (u32, u32) {
                let lo = self.regs.timerawl.timerawl();
                let hi = self.regs.timerawh.timerawh();
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
                self.regs.timelw.write_timelw(lo);
                self.regs.timehw.write_timehw(hi);
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
                    0 => self.regs.alarm0.write_alarm0(time),
                    1 => self.regs.alarm1.write_alarm1(time),
                    2 => self.regs.alarm2.write_alarm2(time),
                    3 => self.regs.alarm3.write_alarm3(time),
                    _ => unreachable!(),
                }
            }

            /// Check if an alarm is armed
            #[inline]
            pub fn is_alarm_armed(&self, alarm: u8) -> bool {
                assert!(alarm < 4, "Alarm must be 0-3");
                (self.regs.armed.armed() & (1 << alarm)) != 0
            }

            /// Disarm an alarm (cancel it before it fires)
            ///
            /// Write 1 to the corresponding bit to disarm.
            #[inline]
            pub fn disarm_alarm(&self, alarm: u8) {
                assert!(alarm < 4, "Alarm must be 0-3");
                self.regs.armed.write_armed(1 << alarm);
            }

            /// Disarm all alarms
            #[inline]
            pub fn disarm_all_alarms(&self) {
                self.regs.armed.write_armed(0xf);
            }

            /// Get armed status of all alarms
            #[inline]
            pub fn armed_status(&self) -> u8 {
                (self.regs.armed.armed() & 0xf) as u8
            }

            // ========================================================================
            // Interrupts
            // ========================================================================

            /// Enable interrupt for an alarm
            #[inline]
            pub fn enable_alarm_interrupt(&self, alarm: u8) {
                assert!(alarm < 4, "Alarm must be 0-3");
                match alarm {
                    0 => self.regs.inte.set_alarm_0(),
                    1 => self.regs.inte.set_alarm_1(),
                    2 => self.regs.inte.set_alarm_2(),
                    3 => self.regs.inte.set_alarm_3(),
                    _ => unreachable!(),
                }
            }

            /// Disable interrupt for an alarm
            #[inline]
            pub fn disable_alarm_interrupt(&self, alarm: u8) {
                assert!(alarm < 4, "Alarm must be 0-3");
                match alarm {
                    0 => self.regs.inte.clr_alarm_0(),
                    1 => self.regs.inte.clr_alarm_1(),
                    2 => self.regs.inte.clr_alarm_2(),
                    3 => self.regs.inte.clr_alarm_3(),
                    _ => unreachable!(),
                }
            }

            /// Enable all alarm interrupts
            #[inline]
            pub fn enable_all_alarm_interrupts(&self) {
                self.regs.inte.set_alarm_0();
                self.regs.inte.set_alarm_1();
                self.regs.inte.set_alarm_2();
                self.regs.inte.set_alarm_3();
            }

            /// Disable all alarm interrupts
            #[inline]
            pub fn disable_all_alarm_interrupts(&self) {
                self.regs.inte.clr_alarm_0();
                self.regs.inte.clr_alarm_1();
                self.regs.inte.clr_alarm_2();
                self.regs.inte.clr_alarm_3();
            }

            /// Check if an alarm interrupt is enabled
            #[inline]
            pub fn is_alarm_interrupt_enabled(&self, alarm: u8) -> bool {
                assert!(alarm < 4, "Alarm must be 0-3");
                match alarm {
                    0 => self.regs.inte.alarm_0(),
                    1 => self.regs.inte.alarm_1(),
                    2 => self.regs.inte.alarm_2(),
                    3 => self.regs.inte.alarm_3(),
                    _ => unreachable!(),
                }
            }

            /// Check if an alarm interrupt is pending (raw, before masking)
            #[inline]
            pub fn alarm_interrupt_raw(&self, alarm: u8) -> bool {
                assert!(alarm < 4, "Alarm must be 0-3");
                match alarm {
                    0 => self.regs.intr.alarm_0(),
                    1 => self.regs.intr.alarm_1(),
                    2 => self.regs.intr.alarm_2(),
                    3 => self.regs.intr.alarm_3(),
                    _ => unreachable!(),
                }
            }

            /// Check if an alarm interrupt is pending (after masking)
            #[inline]
            pub fn alarm_interrupt_status(&self, alarm: u8) -> bool {
                assert!(alarm < 4, "Alarm must be 0-3");
                match alarm {
                    0 => self.regs.ints.alarm_0(),
                    1 => self.regs.ints.alarm_1(),
                    2 => self.regs.ints.alarm_2(),
                    3 => self.regs.ints.alarm_3(),
                    _ => unreachable!(),
                }
            }

            /// Clear an alarm interrupt (write 1 to clear)
            #[inline]
            pub fn clear_alarm_interrupt(&self, alarm: u8) {
                assert!(alarm < 4, "Alarm must be 0-3");
                match alarm {
                    0 => self.regs.intr.set_alarm_0(),
                    1 => self.regs.intr.set_alarm_1(),
                    2 => self.regs.intr.set_alarm_2(),
                    3 => self.regs.intr.set_alarm_3(),
                    _ => unreachable!(),
                }
            }

            /// Clear all alarm interrupts
            #[inline]
            pub fn clear_all_alarm_interrupts(&self) {
                self.regs.intr.set_alarm_0();
                self.regs.intr.set_alarm_1();
                self.regs.intr.set_alarm_2();
                self.regs.intr.set_alarm_3();
            }

            /// Force an alarm interrupt (for testing)
            #[inline]
            pub fn force_alarm_interrupt(&self, alarm: u8) {
                assert!(alarm < 4, "Alarm must be 0-3");
                match alarm {
                    0 => self.regs.intf.set_alarm_0(),
                    1 => self.regs.intf.set_alarm_1(),
                    2 => self.regs.intf.set_alarm_2(),
                    3 => self.regs.intf.set_alarm_3(),
                    _ => unreachable!(),
                }
            }

            /// Clear forced alarm interrupt
            #[inline]
            pub fn clear_forced_alarm_interrupt(&self, alarm: u8) {
                assert!(alarm < 4, "Alarm must be 0-3");
                match alarm {
                    0 => self.regs.intf.clr_alarm_0(),
                    1 => self.regs.intf.clr_alarm_1(),
                    2 => self.regs.intf.clr_alarm_2(),
                    3 => self.regs.intf.clr_alarm_3(),
                    _ => unreachable!(),
                }
            }

            // ========================================================================
            // Control
            // ========================================================================

            /// Pause the timer
            #[inline]
            pub fn pause(&self) {
                self.regs.pause.set_pause();
            }

            /// Resume the timer
            #[inline]
            pub fn resume(&self) {
                self.regs.pause.clr_pause();
            }

            /// Check if the timer is paused
            #[inline]
            pub fn is_paused(&self) -> bool {
                self.regs.pause.pause()
            }

            /// Lock the timer (disable write access)
            ///
            /// Once locked, the timer cannot be unlocked until reset.
            #[inline]
            pub fn lock(&self) {
                self.regs.locked.set_locked();
            }

            /// Check if the timer is locked
            #[inline]
            pub fn is_locked(&self) -> bool {
                self.regs.locked.locked()
            }
        }

        impl Default for $TimerName {
            fn default() -> Self {
                Self::new()
            }
        }
    };
}

// Generate Timer0 and Timer1 types
impl_timer!(Timer0, timer0, TIMER0, Peripheral::TIMER0);
impl_timer!(Timer1, timer1, TIMER1, Peripheral::TIMER1);

// ============================================================================
// Convenience Functions
// ============================================================================

/// Get current time from TIMER0 in microseconds
#[inline]
pub fn get_time_us() -> u64 {
    Timer0::new().get_time_us()
}

/// Get current time from TIMER0 (low 32 bits only)
#[inline]
pub fn get_time_us_32() -> u32 {
    Timer0::new().get_time_us_32()
}

/// Simple busy-wait delay using TIMER0
pub fn delay_us(us: u32) {
    let timer = Timer0::new();
    let start = timer.get_time_us_32();
    while timer.get_time_us_32().wrapping_sub(start) < us {
        core::hint::spin_loop();
    }
}

/// Simple busy-wait delay in milliseconds using TIMER0
pub fn delay_ms(ms: u32) {
    delay_us(ms * 1000);
}
