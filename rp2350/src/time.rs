//! Time and Duration types for embedded systems
//!
//! This module provides basic time-related types without external dependencies.

use core::ops::{Add, AddAssign, Sub, SubAssign};

/// A duration of time, represented in microseconds
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Duration {
    micros: u64,
}

impl Duration {
    /// Zero duration
    pub const ZERO: Duration = Duration { micros: 0 };

    /// Maximum duration
    pub const MAX: Duration = Duration { micros: u64::MAX };

    /// Create a duration from microseconds
    #[inline]
    pub const fn from_micros(micros: u64) -> Self {
        Self { micros }
    }

    /// Create a duration from milliseconds
    #[inline]
    pub const fn from_millis(millis: u64) -> Self {
        Self {
            micros: millis * 1_000,
        }
    }

    /// Create a duration from seconds
    #[inline]
    pub const fn from_secs(secs: u64) -> Self {
        Self {
            micros: secs * 1_000_000,
        }
    }

    /// Create a duration from a frequency in Hz (period = 1/freq)
    ///
    /// # Panics
    /// Panics if `hz == 0`.
    #[inline]
    pub const fn from_hz(hz: u32) -> Self {
        assert!(hz > 0, "frequency must be non-zero");
        Self {
            micros: 1_000_000 / hz as u64,
        }
    }

    /// Get the duration in microseconds
    #[inline]
    pub const fn as_micros(&self) -> u64 {
        self.micros
    }

    /// Get the duration in milliseconds
    #[inline]
    pub const fn as_millis(&self) -> u64 {
        self.micros / 1_000
    }

    /// Get the duration in seconds
    #[inline]
    pub const fn as_secs(&self) -> u64 {
        self.micros / 1_000_000
    }

    /// Get the duration as ticks for a given frequency
    ///
    /// # Panics
    /// Panics if `freq_hz == 0`.
    #[inline]
    pub const fn as_ticks(&self, freq_hz: u32) -> u64 {
        assert!(freq_hz > 0, "frequency must be non-zero");
        self.micros * freq_hz as u64 / 1_000_000
    }

    /// Checked addition
    #[inline]
    pub const fn checked_add(self, rhs: Duration) -> Option<Duration> {
        match self.micros.checked_add(rhs.micros) {
            Some(micros) => Some(Duration { micros }),
            None => None,
        }
    }

    /// Checked subtraction
    #[inline]
    pub const fn checked_sub(self, rhs: Duration) -> Option<Duration> {
        match self.micros.checked_sub(rhs.micros) {
            Some(micros) => Some(Duration { micros }),
            None => None,
        }
    }

    /// Saturating addition
    #[inline]
    pub const fn saturating_add(self, rhs: Duration) -> Duration {
        Duration {
            micros: self.micros.saturating_add(rhs.micros),
        }
    }

    /// Saturating subtraction
    #[inline]
    pub const fn saturating_sub(self, rhs: Duration) -> Duration {
        Duration {
            micros: self.micros.saturating_sub(rhs.micros),
        }
    }
}

impl Add for Duration {
    type Output = Duration;

    #[inline]
    fn add(self, rhs: Duration) -> Duration {
        Duration {
            micros: self.micros + rhs.micros,
        }
    }
}

impl AddAssign for Duration {
    #[inline]
    fn add_assign(&mut self, rhs: Duration) {
        self.micros += rhs.micros;
    }
}

impl Sub for Duration {
    type Output = Duration;

    #[inline]
    fn sub(self, rhs: Duration) -> Duration {
        Duration {
            micros: self.micros - rhs.micros,
        }
    }
}

impl SubAssign for Duration {
    #[inline]
    fn sub_assign(&mut self, rhs: Duration) {
        self.micros -= rhs.micros;
    }
}

/// An instant in time, represented as microseconds since some epoch
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Instant {
    micros: u64,
}

impl Instant {
    /// Create an instant from microseconds
    #[inline]
    pub const fn from_micros(micros: u64) -> Self {
        Self { micros }
    }

    /// Get the instant in microseconds
    #[inline]
    pub const fn as_micros(&self) -> u64 {
        self.micros
    }

    /// Calculate duration since another instant
    #[inline]
    pub const fn duration_since(&self, earlier: Instant) -> Duration {
        Duration {
            micros: self.micros.saturating_sub(earlier.micros),
        }
    }

    /// Check if this instant is after another
    #[inline]
    pub const fn is_after(&self, other: Instant) -> bool {
        self.micros > other.micros
    }

    /// Checked addition of duration
    #[inline]
    pub const fn checked_add(self, duration: Duration) -> Option<Instant> {
        match self.micros.checked_add(duration.micros) {
            Some(micros) => Some(Instant { micros }),
            None => None,
        }
    }

    /// Checked subtraction of duration
    #[inline]
    pub const fn checked_sub(self, duration: Duration) -> Option<Instant> {
        match self.micros.checked_sub(duration.micros) {
            Some(micros) => Some(Instant { micros }),
            None => None,
        }
    }
}

impl Add<Duration> for Instant {
    type Output = Instant;

    #[inline]
    fn add(self, rhs: Duration) -> Instant {
        Instant {
            micros: self.micros + rhs.micros,
        }
    }
}

impl AddAssign<Duration> for Instant {
    #[inline]
    fn add_assign(&mut self, rhs: Duration) {
        self.micros += rhs.micros;
    }
}

impl Sub<Duration> for Instant {
    type Output = Instant;

    #[inline]
    fn sub(self, rhs: Duration) -> Instant {
        Instant {
            micros: self.micros - rhs.micros,
        }
    }
}

impl SubAssign<Duration> for Instant {
    #[inline]
    fn sub_assign(&mut self, rhs: Duration) {
        self.micros -= rhs.micros;
    }
}

impl Sub<Instant> for Instant {
    type Output = Duration;

    #[inline]
    fn sub(self, rhs: Instant) -> Duration {
        self.duration_since(rhs)
    }
}
