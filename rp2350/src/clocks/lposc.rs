//! Low-Power Oscillator (LPOSC) driver for RP2350
//!
//! The LPOSC is an ultra-low-power on-chip oscillator running at approximately
//! 32 kHz. It is always running and provides the clock for the power manager
//! (POWMAN) and can be selected as a source for CLK_REF.
//!
//! The LPOSC is controlled via the POWMAN peripheral, not through the CLOCKS
//! register block. It has a TRIM field for frequency calibration and a MODE
//! field (deprecated on RP2350).
//!
//! ## Key Characteristics
//!
//! | Parameter | Value |
//! |-----------|-------|
//! | Nominal frequency | ~32.768 kHz |
//! | Trim range | ±15% (6-bit, ~1% per step) |
//! | Always running | Yes (cannot be disabled) |
//! | Calibration | OTP LPOSC_CALIB register |
//!
//! ## Example
//!
//! ```ignore
//! use rp2350::clocks::lposc::Lposc;
//! use rp2350::Peripherals;
//!
//! let dp = Peripherals::take().unwrap();
//! let lposc = Lposc::init(dp.POWMAN);
//!
//! // Read OTP calibration and apply trim
//! lposc.apply_otp_calib();
//!
//! // Check current trim
//! let trim = lposc.trim();
//! ```

use crate::Steal;
use rp2350_pac::POWMAN;

/// LPOSC nominal frequency (Hz)
pub const LPOSC_HZ: u32 = 32_768;

/// Maximum trim value (6-bit: 0-63)
pub const LPOSC_TRIM_MAX: u32 = 0x3F;

/// Low-Power Oscillator driver
///
/// Provides access to the LPOSC trim and mode registers within POWMAN.
/// The LPOSC is always running and cannot be disabled by software.
pub struct Lposc {
    powman: POWMAN,
}

impl Lposc {
    /// Initialize the LPOSC driver.
    ///
    /// Takes ownership of the POWMAN peripheral. The LPOSC is always running
    /// and does not require a reset sequence.
    #[inline]
    pub fn init(powman: POWMAN) -> Self {
        Self { powman }
    }

    /// Create an LPOSC driver by stealing the peripheral
    ///
    /// # Safety
    /// This steals the peripheral globally. Only call once.
    #[inline]
    pub unsafe fn steal() -> Steal<Self> {
        // SAFETY: caller guarantees single ownership
        Steal(Self {
            powman: unsafe { POWMAN::steal() },
        })
    }

    /// Get a reference to the LPOSC register
    #[inline]
    fn regs(&self) -> &rp2350_pac::powman::Lposc {
        &self.powman.lposc
    }

    /// Get the current trim value (0-63)
    ///
    /// The trim value adjusts the LPOSC frequency. Each step changes
    /// the frequency by approximately ±1% from the reset value.
    #[inline]
    pub fn trim(&self) -> u8 {
        self.regs().trim() as u8
    }

    /// Set the trim value (0-63)
    ///
    /// # Arguments
    /// * `trim` - Trim value (0-63). Clamped to valid range.
    ///
    /// Higher trim values generally increase frequency.
    #[inline]
    pub fn set_trim(&mut self, trim: u8) {
        let trim = (trim as u32).min(LPOSC_TRIM_MAX);
        self.regs().write_trim(trim);
    }

    /// Apply the OTP calibration trim value
    ///
    /// The RP2350 stores a factory-calibrated LPOSC trim value in OTP
    /// (LPOSC_CALIB register). This method reads that value and applies it,
    /// giving the most accurate 32 kHz frequency.
    ///
    /// Returns `true` if calibration was applied, `false` if OTP was unprogrammed.
    pub fn apply_otp_calib(&mut self) -> bool {
        unsafe {
            let otp = rp2350_pac::OTP_DATA::steal();
            let calib = otp.lposc_calib.lposc_calib() as u16;
            if calib != 0 && calib != 0xFFFF {
                self.set_trim(calib as u8);
                true
            } else {
                false
            }
        }
    }

    /// Get the nominal LPOSC frequency in Hz
    #[inline]
    pub const fn frequency(&self) -> u32 {
        LPOSC_HZ
    }
}
