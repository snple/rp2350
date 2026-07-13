//! Crystal Oscillator (XOSC) HAL for RP2350
//!
//! This module provides a safe interface for controlling the crystal oscillator.
//! The XOSC provides a stable clock source derived from an external crystal.
//!
//! ## Example
//!
//! ```ignore
//! use pico_rs::clocks::xosc::{Xosc, FrequencyRange};
//! use pico_rs_pac::XOSC;
//!
//! // Get XOSC peripheral
//! let xosc_pac = unsafe { XOSC::steal() };
//!
//! // Initialize XOSC with a 12 MHz crystal (default for Pico 2)
//! let xosc = Xosc::init(xosc_pac, 12_000_000);
//!
//! // Wait for XOSC to stabilize
//! xosc.wait_stable();
//!
//! // Check if stable
//! if xosc.is_stable() {
//!     // XOSC is ready to use
//! }
//! ```

use pico_rs_pac::{
    XOSC,
    xosc::{Ctrl, Dormant},
};

use crate::Steal;

/// Default XOSC frequency for Pico 2 board (12 MHz)
pub const XOSC_HZ: u32 = 12_000_000;

/// Startup delay multiplier (1 = normal, increase for noisy environments)
pub const STARTUP_DELAY_MULTIPLIER: u32 = 1;

/// Crystal oscillator frequency range selection
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FrequencyRange {
    /// 1-15 MHz (default for 12 MHz crystal)
    Range1_15MHz = 0,
    /// 10-30 MHz
    Range10_30MHz = 1,
    /// 25-60 MHz
    Range25_60MHz = 2,
    /// 40-100 MHz
    Range40_100MHz = 3,
}

impl FrequencyRange {
    /// Get the register value for this frequency range
    #[inline]
    pub const fn to_bits(self) -> u32 {
        match self {
            FrequencyRange::Range1_15MHz => 0xaa0,   // 2720
            FrequencyRange::Range10_30MHz => 0xaa1,  // 2721
            FrequencyRange::Range25_60MHz => 0xaa2,  // 2722
            FrequencyRange::Range40_100MHz => 0xaa3, // 2723
        }
    }

    /// Determine the appropriate frequency range for a given frequency in Hz
    #[inline]
    pub const fn from_frequency(freq_hz: u32) -> Self {
        if freq_hz < 10_000_000 {
            FrequencyRange::Range1_15MHz
        } else if freq_hz < 25_000_000 {
            FrequencyRange::Range10_30MHz
        } else if freq_hz < 40_000_000 {
            FrequencyRange::Range25_60MHz
        } else {
            FrequencyRange::Range40_100MHz
        }
    }
}

/// Crystal Oscillator (XOSC) peripheral wrapper
pub struct Xosc {
    xosc: XOSC,
}

impl Xosc {
    /// Initialize the crystal oscillator
    ///
    /// This configures the XOSC with the appropriate frequency range and startup
    /// delay, enables it, and waits for it to stabilize.
    ///
    /// # Arguments
    /// * `xosc` - The XOSC peripheral from PAC
    /// * `freq_hz` - The crystal frequency in Hz (typically 12 MHz for Pico 2)
    ///
    /// # Example
    /// ```ignore
    /// let xosc = Xosc::init(xosc_pac, 12_000_000);
    /// ```
    pub fn init(xosc: XOSC, freq_hz: u32) -> Self {
        // Determine frequency range
        let range = FrequencyRange::from_frequency(freq_hz);

        // Calculate startup delay
        // delay = ((freq_khz + 128) / 256) * multiplier
        // The delay is in units of 256 xosc cycles
        let freq_khz = freq_hz / 1000;
        let startup_delay = ((freq_khz + 128) / 256) * STARTUP_DELAY_MULTIPLIER;

        // Set frequency range (this must be done before enabling)
        // Note: write_freq_range expects the raw value (e.g., 0xaa0), not shifted
        xosc.ctrl.write_freq_range(range.to_bits());

        // Set startup delay
        xosc.startup.write_delay(startup_delay);

        // Enable the oscillator
        // Note: ENABLE_ENABLE is already shifted (4011 << 12), but write_enable expects unshifted value
        // So we need to pass 4011 (0xFAB) directly, not the pre-shifted constant
        const ENABLE_VALUE: u32 = 0xFAB; // "ENABLE" magic value
        xosc.ctrl.write_enable(ENABLE_VALUE);

        // Wait for XOSC to stabilize
        while !xosc.status.stable() {
            core::hint::spin_loop();
        }

        Self { xosc }
    }

    /// Obtain an Xosc handle without ownership checks.
    ///
    /// # Safety
    ///
    /// The caller must ensure [`Xosc::init()`] has been called first
    /// and that no data races occur.
    #[inline]
    pub unsafe fn steal() -> Steal<Self> {
        Steal(Self {
            xosc: unsafe { XOSC::steal() },
        })
    }

    /// Get reference to the underlying register block
    ///
    /// # Safety
    /// Direct register access can bypass the HAL's safety guarantees.
    /// The caller must ensure operations do not conflict with the HAL state.
    #[inline]
    pub unsafe fn regs(&self) -> &pico_rs_pac::xosc::RegisterBlock {
        &self.xosc
    }

    /// Release the XOSC peripheral
    #[inline]
    pub fn release(self) -> XOSC {
        self.xosc
    }

    /// Initialize XOSC with default 12 MHz crystal frequency
    ///
    /// # Arguments
    /// * `xosc` - The XOSC peripheral from PAC
    #[inline]
    pub fn init_default(xosc: XOSC) -> Self {
        Self::init(xosc, XOSC_HZ)
    }

    /// Check if the XOSC is running and stable
    #[inline]
    pub fn stable(&self) -> bool {
        self.xosc.status.stable()
    }

    /// Check if the XOSC is enabled
    #[inline]
    pub fn enabled(&self) -> bool {
        self.xosc.status.enabled()
    }

    /// Wait for the XOSC to become stable
    #[inline]
    pub fn wait_stable(&self) {
        while !self.stable() {
            core::hint::spin_loop();
        }
    }

    /// Enable the crystal oscillator
    ///
    /// Note: You should call `wait_stable()` after enabling to ensure
    /// the oscillator is ready for use.
    #[inline]
    pub fn enable(&self) {
        self.xosc.ctrl.write_enable(Ctrl::ENABLE_ENABLE);
    }

    /// Disable the crystal oscillator
    ///
    /// Warning: Make sure no clocks are depending on XOSC before disabling!
    pub fn disable(&self) {
        self.xosc.ctrl.write_enable(Ctrl::ENABLE_DISABLE);

        // Wait for stable to go away
        while self.stable() {
            core::hint::spin_loop();
        }
    }

    /// Enter dormant mode
    ///
    /// This puts the XOSC into a low-power dormant state. The oscillator will
    /// be woken up by an interrupt or by calling `wake()`.
    ///
    /// # Warning
    /// This will stop the XOSC until woken by an IRQ. Make sure you have
    /// configured wake sources before calling this!
    pub fn dormant(&self) {
        self.xosc.dormant.write_dormant(Dormant::DORMANT_DORMANT);

        // Wait for it to become stable once woken up
        self.wait_stable();
    }

    /// Wake the XOSC from dormant mode
    #[inline]
    pub fn wake(&mut self) {
        self.xosc.dormant.write_dormant(Dormant::DORMANT_WAKE);
    }

    /// Get the current frequency range setting from hardware.
    ///
    /// This returns the raw hardware register value, which encodes the
    /// configured frequency range (1-15 MHz, 10-30 MHz, etc.). The exact
    /// crystal frequency is known only at [`init()`](Self::init) time.
    #[inline]
    pub fn current_freq_range(&self) -> u32 {
        self.xosc.status.freq_range()
    }

    /// Check if a bad write has occurred
    ///
    /// This flag is set when an invalid value has been written to
    /// CTRL_ENABLE, CTRL_FREQ_RANGE, or DORMANT registers.
    #[inline]
    pub fn bad_write(&self) -> bool {
        self.xosc.status.badwrite()
    }

    /// Clear the bad write flag
    #[inline]
    pub fn clear_bad_write(&mut self) {
        self.xosc.status.set_badwrite();
    }

    /// Set the startup delay
    ///
    /// The delay is in units of 256 XOSC cycles. Increasing this value
    /// provides more time for the crystal to stabilize.
    ///
    /// # Arguments
    /// * `delay` - Delay value (0-8191)
    /// * `multiply_by_4` - If true, multiply the delay by 4
    #[inline]
    pub fn set_startup_delay(&mut self, delay: u16, multiply_by_4: bool) {
        self.xosc.startup.write_delay(delay as u32);
        if multiply_by_4 {
            self.xosc.startup.set_x4();
        } else {
            self.xosc.startup.clr_x4();
        }
    }

    /// Start a short delay using the XOSC counter
    ///
    /// The counter counts down from the specified value to 0 at the XOSC
    /// frequency. Minimum count value is 4.
    ///
    /// # Arguments
    /// * `count` - Number of XOSC cycles to delay (minimum 4)
    #[inline]
    pub fn start_count(&mut self, count: u16) {
        self.xosc.count.write_count(count as u32);
    }

    /// Check if the count delay is complete
    ///
    /// Returns true when the counter has reached 0.
    #[inline]
    pub fn count_done(&self) -> bool {
        self.xosc.count.count() == 0
    }

    /// Perform a short delay using the XOSC counter
    ///
    /// This blocks until the specified number of XOSC cycles have elapsed.
    ///
    /// # Arguments
    /// * `cycles` - Number of XOSC cycles to delay (minimum 4)
    pub fn delay_cycles(&mut self, cycles: u16) {
        self.start_count(cycles.max(4));
        while !self.count_done() {
            core::hint::spin_loop();
        }
    }
}
