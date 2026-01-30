//! Always-On Timer (AON Timer) HAL for RP2350
//!
//! This module provides a high-level interface to the Always-On timer in POWMAN.
//! The AON timer continues running in low-power modes and can be used for:
//!
//! - Real-time clock functionality
//! - Waking from low-power states at a specific time
//! - Long-duration timing (64-bit millisecond counter)
//!
//! ## Features
//!
//! - 64-bit millisecond counter
//! - Multiple clock sources (LPOSC, XOSC, GPIO)
//! - Alarm with optional wakeup capability
//! - Calendar time support via `Timespec`
//!
//! ## Example
//!
//! ```ignore
//! use pico_rs::aon_timer::{AonTimer, Timespec};
//!
//! // Initialize AON timer with XOSC as source
//! let aon = AonTimer::init_with_xosc(12_000_000);
//!
//! // Set current time
//! aon.set_time(&Timespec::from_secs(0));
//!
//! // Get current time
//! let now = aon.get_time();
//!
//! // Set an alarm for 10 seconds from now
//! let alarm_time = Timespec::from_secs(now.as_secs() + 10);
//! aon.enable_alarm(&alarm_time, false);
//! ```

use crate::powman::Powman;

/// Time representation (seconds and nanoseconds since epoch)
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Timespec {
    /// Seconds since epoch
    pub tv_sec: i64,
    /// Nanoseconds (0-999,999,999)
    pub tv_nsec: i64,
}

impl Timespec {
    /// Create a new Timespec
    pub const fn new(secs: i64, nsecs: i64) -> Self {
        Self {
            tv_sec: secs,
            tv_nsec: nsecs,
        }
    }

    /// Create from seconds only
    pub const fn from_secs(secs: i64) -> Self {
        Self {
            tv_sec: secs,
            tv_nsec: 0,
        }
    }

    /// Create from milliseconds
    pub const fn from_millis(millis: i64) -> Self {
        Self {
            tv_sec: millis / 1000,
            tv_nsec: (millis % 1000) * 1_000_000,
        }
    }

    /// Get total seconds
    pub const fn as_secs(&self) -> i64 {
        self.tv_sec
    }

    /// Get total milliseconds
    pub const fn as_millis(&self) -> i64 {
        self.tv_sec * 1000 + self.tv_nsec / 1_000_000
    }

    /// Convert to milliseconds (u64)
    pub fn to_ms(&self) -> u64 {
        let secs = self.tv_sec.max(0) as u64;
        let nsecs = self.tv_nsec.max(0) as u64;
        secs * 1000 + nsecs / 1_000_000
    }

    /// Create from milliseconds (u64)
    pub fn from_ms(ms: u64) -> Self {
        Self {
            tv_sec: (ms / 1000) as i64,
            tv_nsec: ((ms % 1000) * 1_000_000) as i64,
        }
    }
}

/// Clock source for AON timer
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AonClockSource {
    /// Low-power oscillator (~32 kHz)
    Lposc,
    /// Crystal oscillator (accurate)
    Xosc,
    /// External GPIO (1 kHz signal)
    Gpio(u8),
}

/// Alarm handler function type
pub type AlarmHandler = fn();

/// Always-On Timer
pub struct AonTimer {
    powman: Powman,
}

impl AonTimer {
    /// Create a new AON timer instance
    ///
    /// Note: You should call one of the init methods to configure the clock source.
    #[inline]
    pub fn new() -> Self {
        Self {
            powman: Powman::new(),
        }
    }

    /// Initialize AON timer with LPOSC as clock source
    ///
    /// LPOSC is less accurate but available in all power modes.
    pub fn init_with_lposc() -> Self {
        let timer = Self::new();
        timer.powman.timer_set_1khz_tick_source_lposc();
        timer
    }

    /// Initialize AON timer with LPOSC and known frequency
    pub fn init_with_lposc_hz(lposc_freq_hz: u32) -> Self {
        let timer = Self::new();
        timer
            .powman
            .timer_set_1khz_tick_source_lposc_with_hz(lposc_freq_hz);
        timer
    }

    /// Initialize AON timer with XOSC as clock source
    ///
    /// XOSC is more accurate but requires crystal to be running.
    pub fn init_with_xosc(xosc_freq_hz: u32) -> Self {
        let timer = Self::new();
        timer
            .powman
            .timer_set_1khz_tick_source_xosc_with_hz(xosc_freq_hz);
        timer
    }

    /// Initialize AON timer with GPIO as clock source
    ///
    /// # Arguments
    /// * `gpio` - GPIO number (must be 12, 14, 20, or 22)
    pub fn init_with_gpio(gpio: u8) -> Self {
        let timer = Self::new();
        timer.powman.timer_set_1khz_tick_source_gpio(gpio);
        timer
    }

    // ========================================================================
    // Timer Control
    // ========================================================================

    /// Start the AON timer
    #[inline]
    pub fn start(&self) {
        self.powman.timer_start();
    }

    /// Stop the AON timer
    #[inline]
    pub fn stop(&self) {
        self.powman.timer_stop();
    }

    /// Check if timer is running
    #[inline]
    pub fn is_running(&self) -> bool {
        self.powman.timer_is_running()
    }

    // ========================================================================
    // Time Get/Set
    // ========================================================================

    /// Set the current time
    pub fn set_time(&self, ts: &Timespec) -> bool {
        self.powman.timer_set_ms(ts.to_ms());
        true
    }

    /// Get the current time
    pub fn get_time(&self) -> Timespec {
        Timespec::from_ms(self.powman.timer_get_ms())
    }

    /// Get the current time in milliseconds
    #[inline]
    pub fn get_time_ms(&self) -> u64 {
        self.powman.timer_get_ms()
    }

    /// Set the current time in milliseconds
    #[inline]
    pub fn set_time_ms(&self, ms: u64) {
        self.powman.timer_set_ms(ms);
    }

    // ========================================================================
    // Alarm Control
    // ========================================================================

    /// Enable alarm at a specific time
    ///
    /// # Arguments
    /// * `ts` - Time at which alarm should fire
    /// * `wakeup_from_low_power` - If true, will wake system from low-power mode
    pub fn enable_alarm(&self, ts: &Timespec, wakeup_from_low_power: bool) {
        if wakeup_from_low_power {
            self.powman.enable_alarm_wakeup_at_ms(ts.to_ms());
        } else {
            self.powman.timer_enable_alarm_at_ms(ts.to_ms());
        }
    }

    /// Enable alarm at a specific time in milliseconds
    pub fn enable_alarm_at_ms(&self, alarm_time_ms: u64, wakeup_from_low_power: bool) {
        if wakeup_from_low_power {
            self.powman.enable_alarm_wakeup_at_ms(alarm_time_ms);
        } else {
            self.powman.timer_enable_alarm_at_ms(alarm_time_ms);
        }
    }

    /// Disable the alarm
    pub fn disable_alarm(&self) {
        self.powman.timer_disable_alarm();
    }

    /// Check if alarm has fired
    #[inline]
    pub fn alarm_fired(&self) -> bool {
        self.powman.timer_alarm_fired()
    }

    /// Clear the alarm flag
    #[inline]
    pub fn clear_alarm(&self) {
        self.powman.timer_clear_alarm();
    }

    // ========================================================================
    // IRQ Handling
    // ========================================================================

    /// Get the IRQ number for AON timer alarm
    pub const fn irq_num() -> crate::pac::Interrupt {
        crate::pac::Interrupt::POWMAN_IRQ_TIMER
    }

    /// Check if timer interrupt is pending
    #[inline]
    pub fn irq_pending(&self) -> bool {
        self.powman.timer_irq_raw()
    }

    // ========================================================================
    // Convenience Functions
    // ========================================================================

    /// Sleep until the specified time (busy wait)
    ///
    /// Note: This is a busy wait and does not put the system to sleep.
    /// For true low-power sleep, use the alarm wakeup feature.
    pub fn wait_until(&self, ts: &Timespec) {
        let target_ms = ts.to_ms();
        while self.get_time_ms() < target_ms {
            core::hint::spin_loop();
        }
    }

    /// Delay for the specified duration (busy wait)
    pub fn delay_ms(&self, ms: u64) {
        let target = self.get_time_ms() + ms;
        while self.get_time_ms() < target {
            core::hint::spin_loop();
        }
    }

    /// Get reference to underlying Powman
    #[inline]
    pub fn powman(&self) -> &Powman {
        &self.powman
    }
}

impl Default for AonTimer {
    fn default() -> Self {
        Self::new()
    }
}

/// Convert between calendar time (tm) and timespec
pub mod calendar {
    use super::Timespec;

    /// Simple date/time structure
    #[derive(Clone, Copy, Debug, Default)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub struct DateTime {
        /// Year (e.g., 2024)
        pub year: u16,
        /// Month (1-12)
        pub month: u8,
        /// Day of month (1-31)
        pub day: u8,
        /// Hour (0-23)
        pub hour: u8,
        /// Minute (0-59)
        pub minute: u8,
        /// Second (0-59)
        pub second: u8,
        /// Day of week (0=Sunday, 6=Saturday)
        pub day_of_week: u8,
    }

    impl DateTime {
        /// Create a new DateTime
        pub const fn new(year: u16, month: u8, day: u8, hour: u8, minute: u8, second: u8) -> Self {
            Self {
                year,
                month,
                day,
                hour,
                minute,
                second,
                day_of_week: 0,
            }
        }

        /// Convert to seconds since Unix epoch (simplified, no timezone)
        ///
        /// Note: This is a simplified calculation and does not account for
        /// leap seconds or timezone. For accurate time, use a proper RTC library.
        pub fn to_unix_secs(&self) -> i64 {
            // Days from year
            let mut days = 0i64;
            for y in 1970..self.year {
                days += if is_leap_year(y) { 366 } else { 365 };
            }

            // Days from months
            let days_in_months = if is_leap_year(self.year) {
                [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
            } else {
                [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
            };

            for &days_in_month in days_in_months
                .iter()
                .take(self.month.saturating_sub(1) as usize)
            {
                days += days_in_month as i64;
            }

            // Days from day of month
            days += (self.day.saturating_sub(1)) as i64;

            // Convert to seconds
            days * 86400
                + (self.hour as i64) * 3600
                + (self.minute as i64) * 60
                + self.second as i64
        }

        /// Convert from seconds since Unix epoch (simplified)
        pub fn from_unix_secs(secs: i64) -> Self {
            if secs < 0 {
                return Self::default();
            }

            let mut remaining = secs as u64;

            // Extract time
            let second = (remaining % 60) as u8;
            remaining /= 60;
            let minute = (remaining % 60) as u8;
            remaining /= 60;
            let hour = (remaining % 24) as u8;
            remaining /= 24; // Now in days since epoch

            // Calculate day of week (1970-01-01 was Thursday = 4)
            let day_of_week = ((remaining + 4) % 7) as u8;

            // Extract year
            let mut year = 1970u16;
            loop {
                let days_in_year = if is_leap_year(year) { 366 } else { 365 };
                if remaining < days_in_year {
                    break;
                }
                remaining -= days_in_year;
                year += 1;
            }

            // Extract month and day
            let days_in_months = if is_leap_year(year) {
                [31u64, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
            } else {
                [31u64, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
            };

            let mut month = 1u8;
            for days_in_month in days_in_months.iter() {
                if remaining < *days_in_month {
                    break;
                }
                remaining -= *days_in_month;
                month += 1;
            }

            let day = remaining as u8 + 1;

            Self {
                year,
                month,
                day,
                hour,
                minute,
                second,
                day_of_week,
            }
        }

        /// Convert to Timespec
        pub fn to_timespec(&self) -> Timespec {
            Timespec::from_secs(self.to_unix_secs())
        }

        /// Convert from Timespec
        pub fn from_timespec(ts: &Timespec) -> Self {
            Self::from_unix_secs(ts.tv_sec)
        }
    }

    /// Check if year is a leap year
    pub const fn is_leap_year(year: u16) -> bool {
        (year.is_multiple_of(4) && !year.is_multiple_of(100)) || year.is_multiple_of(400)
    }
}
