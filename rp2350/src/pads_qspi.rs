//! QSPI pad configuration (`PADS_QSPI`) for RP2350
//!
//! The QSPI interface uses dedicated pads (SCLK, CSn, SD0..SD3) whose drive
//! strength, slew rate, Schmitt trigger and pull resistors are configured
//! here — separately from the GPIO bank-0 pads in [`crate::gpio`].
//!
//! ## Example
//!
//! ```ignore
//! use rp2350::gpio::{DriveStrength, SlewRate};
//! use rp2350::pads_qspi::PadsQspi;
//! use rp2350_pac::PADS_QSPI;
//!
//! let pads = PadsQspi::init(unsafe { PADS_QSPI::steal() });
//!
//! // High-speed flash: fast slew, high drive on the clock.
//! pads.sclk().set_drive_strength(DriveStrength::_12mA);
//! pads.sclk().set_slew_rate(SlewRate::Fast);
//! ```

use crate::Steal;
use crate::gpio::{DriveStrength, Pull, SlewRate};

/// QSPI bank supply voltage.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u32)]
pub enum Voltage {
    /// 3.3 V supply (DVDD ≥ 2.5 V)
    V3_3 = 0,
    /// 1.8 V supply (DVDD ≤ 1.8 V)
    V1_8 = 1,
}

/// Common interface implemented by every QSPI pad control register.
///
/// The PAC generates a distinct type per pad (`GpioQspiSclk`, `GpioQspiSd0`,
/// ...) with identical accessors; this trait lets [`Pad`] address any of them.
trait QspiPadReg {
    fn write_drive(&self, value: u32);
    fn set_slewfast(&self);
    fn clr_slewfast(&self);
    fn set_schmitt(&self);
    fn clr_schmitt(&self);
    fn set_pue(&self);
    fn clr_pue(&self);
    fn set_pde(&self);
    fn clr_pde(&self);
    fn set_ie(&self);
    fn clr_ie(&self);
}

macro_rules! impl_qspi_pad_reg {
    ($($ty:ty),+ $(,)?) => {
        $(
            impl QspiPadReg for $ty {
                #[inline]
                fn write_drive(&self, value: u32) {
                    self.write_drive(value);
                }
                #[inline]
                fn set_slewfast(&self) {
                    self.set_slewfast();
                }
                #[inline]
                fn clr_slewfast(&self) {
                    self.clr_slewfast();
                }
                #[inline]
                fn set_schmitt(&self) {
                    self.set_schmitt();
                }
                #[inline]
                fn clr_schmitt(&self) {
                    self.clr_schmitt();
                }
                #[inline]
                fn set_pue(&self) {
                    self.set_pue();
                }
                #[inline]
                fn clr_pue(&self) {
                    self.clr_pue();
                }
                #[inline]
                fn set_pde(&self) {
                    self.set_pde();
                }
                #[inline]
                fn clr_pde(&self) {
                    self.clr_pde();
                }
                #[inline]
                fn set_ie(&self) {
                    self.set_ie();
                }
                #[inline]
                fn clr_ie(&self) {
                    self.clr_ie();
                }
            }
        )+
    };
}

impl_qspi_pad_reg!(
    rp2350_pac::pads_qspi::GpioQspiSclk,
    rp2350_pac::pads_qspi::GpioQspiSd0,
    rp2350_pac::pads_qspi::GpioQspiSd1,
    rp2350_pac::pads_qspi::GpioQspiSd2,
    rp2350_pac::pads_qspi::GpioQspiSd3,
    rp2350_pac::pads_qspi::GpioQspiSs,
);

/// Handle to a single QSPI pad's control register.
pub struct Pad<'a> {
    reg: &'a dyn QspiPadReg,
}

impl Pad<'_> {
    /// Set the output drive strength.
    #[inline]
    pub fn set_drive_strength(&self, strength: DriveStrength) {
        self.reg.write_drive(strength as u32);
    }

    /// Set the output slew rate.
    #[inline]
    pub fn set_slew_rate(&self, rate: SlewRate) {
        match rate {
            SlewRate::Slow => self.reg.clr_slewfast(),
            SlewRate::Fast => self.reg.set_slewfast(),
        }
    }

    /// Enable or disable the Schmitt trigger on the input.
    #[inline]
    pub fn set_schmitt(&self, enable: bool) {
        if enable {
            self.reg.set_schmitt();
        } else {
            self.reg.clr_schmitt();
        }
    }

    /// Configure the pull resistor.
    #[inline]
    pub fn set_pull(&self, pull: Pull) {
        match pull {
            Pull::Up => {
                self.reg.set_pue();
                self.reg.clr_pde();
            }
            Pull::Down => {
                self.reg.clr_pue();
                self.reg.set_pde();
            }
            Pull::None => {
                self.reg.clr_pue();
                self.reg.clr_pde();
            }
        }
    }

    /// Enable the input receiver.
    #[inline]
    pub fn enable_input(&self) {
        self.reg.set_ie();
    }

    /// Disable the input receiver.
    #[inline]
    pub fn disable_input(&self) {
        self.reg.clr_ie();
    }
}

/// `PADS_QSPI` peripheral wrapper.
///
/// Owns the PAC `PADS_QSPI` peripheral. The QSPI pads are dedicated to the
/// QSPI interface (SCLK, CSn, SD0..SD3) and are configured independently of
/// the GPIO bank-0 pads.
pub struct PadsQspi {
    pads_qspi: rp2350_pac::PADS_QSPI,
}

impl PadsQspi {
    /// Take ownership of the `PADS_QSPI` peripheral.
    ///
    /// No reset is performed: resetting the flash pads while executing from
    /// flash would disrupt XIP access (same policy as [`crate::qspi::Qspi`]).
    #[inline]
    pub fn init(pads_qspi: rp2350_pac::PADS_QSPI) -> Self {
        Self { pads_qspi }
    }

    /// Obtain a handle without ownership checks.
    ///
    /// # Safety
    ///
    /// The caller must ensure no other handle to `PADS_QSPI` is live.
    #[inline]
    pub unsafe fn steal() -> Steal<Self> {
        Steal(Self {
            pads_qspi: unsafe { rp2350_pac::PADS_QSPI::steal() },
        })
    }

    /// Get a reference to the underlying register block.
    ///
    /// # Safety
    /// Direct register access can bypass the HAL's safety guarantees.
    #[inline]
    pub unsafe fn regs(&self) -> &rp2350_pac::pads_qspi::RegisterBlock {
        &self.pads_qspi
    }

    /// Release the `PADS_QSPI` peripheral.
    #[inline]
    pub fn release(self) -> rp2350_pac::PADS_QSPI {
        self.pads_qspi
    }

    /// Set the QSPI bank supply voltage.
    #[inline]
    pub fn set_voltage(&self, voltage: Voltage) {
        match voltage {
            Voltage::V3_3 => self.pads_qspi.voltage_select.clr_voltage_select(),
            Voltage::V1_8 => self.pads_qspi.voltage_select.set_voltage_select(),
        }
    }

    /// Get the QSPI bank supply voltage.
    #[inline]
    pub fn voltage(&self) -> Voltage {
        if self.pads_qspi.voltage_select.voltage_select() {
            Voltage::V1_8
        } else {
            Voltage::V3_3
        }
    }

    /// Serial clock pad.
    #[inline]
    pub fn sclk(&self) -> Pad<'_> {
        Pad {
            reg: &self.pads_qspi.gpio_qspi_sclk,
        }
    }

    /// Serial data 0 (MOSI in single-SPI mode).
    #[inline]
    pub fn sd0(&self) -> Pad<'_> {
        Pad {
            reg: &self.pads_qspi.gpio_qspi_sd0,
        }
    }

    /// Serial data 1 (MISO in single-SPI mode).
    #[inline]
    pub fn sd1(&self) -> Pad<'_> {
        Pad {
            reg: &self.pads_qspi.gpio_qspi_sd1,
        }
    }

    /// Serial data 2 (active in quad mode).
    #[inline]
    pub fn sd2(&self) -> Pad<'_> {
        Pad {
            reg: &self.pads_qspi.gpio_qspi_sd2,
        }
    }

    /// Serial data 3 (active in quad mode).
    #[inline]
    pub fn sd3(&self) -> Pad<'_> {
        Pad {
            reg: &self.pads_qspi.gpio_qspi_sd3,
        }
    }

    /// Chip select.
    #[inline]
    pub fn csn(&self) -> Pad<'_> {
        Pad {
            reg: &self.pads_qspi.gpio_qspi_ss,
        }
    }
}
