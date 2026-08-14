//! OTP (One-Time Programmable memory) HAL for RP2350
//!
//! The RP2350 stores a fixed set of factory-programmed and user-programmed
//! values in OTP. This module provides read-only access to the *predefined*
//! data layout ([`rp2350_pac::OTP_DATA`]), which includes the unique chip
//! identifier and the package GPIO count.
//!
//! Programming OTP is deliberately out of scope here — use the raw
//! [`rp2350_pac::OTP`] / [`rp2350_pac::OTP_DATA_RAW`] registers for that.
//!
//! ## Example
//!
//! ```ignore
//! use rp2350::otp::Otp;
//! use rp2350_pac::OTP_DATA;
//!
//! let otp = Otp::init(unsafe { OTP_DATA::steal() });
//! let id = otp.chip_id();
//! let gpios = otp.num_gpios();
//! ```

use crate::Steal;

/// `OTP_DATA` peripheral wrapper (read-only).
///
/// Owns the PAC `OTP_DATA` peripheral to enforce ownership semantics.
pub struct Otp {
    otp_data: rp2350_pac::OTP_DATA,
}

impl Otp {
    /// Take ownership of the `OTP_DATA` peripheral.
    ///
    /// OTP data is hard-wired after programming; there is no reset to
    /// perform, so this only transfers ownership.
    #[inline]
    pub fn init(otp_data: rp2350_pac::OTP_DATA) -> Self {
        Self { otp_data }
    }

    /// Obtain a handle without initialization.
    ///
    /// # Safety
    ///
    /// The caller must ensure no other handle to `OTP_DATA` is live.
    #[inline]
    pub unsafe fn steal() -> Steal<Self> {
        Steal(Self {
            otp_data: unsafe { rp2350_pac::OTP_DATA::steal() },
        })
    }

    /// Get a reference to the underlying register block.
    #[inline]
    pub fn regs(&self) -> &rp2350_pac::otp_data::RegisterBlock {
        &self.otp_data
    }

    /// Release the `OTP_DATA` peripheral.
    #[inline]
    pub fn release(self) -> rp2350_pac::OTP_DATA {
        self.otp_data
    }

    /// The 64-bit unique chip identifier, assembled from the four `CHIPID`
    /// rows (each holding 16 bits).
    #[inline]
    pub fn chip_id(&self) -> u64 {
        let d = &self.otp_data;
        (d.chipid0.chipid0() as u64)
            | ((d.chipid1.chipid1() as u64) << 16)
            | ((d.chipid2.chipid2() as u64) << 32)
            | ((d.chipid3.chipid3() as u64) << 48)
    }

    /// Number of GPIOs on the package, as recorded in OTP.
    #[inline]
    pub fn num_gpios(&self) -> u16 {
        self.otp_data.num_gpios.num_gpios() as u16
    }
}
