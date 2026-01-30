//! Clock Management HAL for RP2350
//!
//! This module provides a safe interface for configuring the RP2350 clock system.
//! The clock system provides clocks for various peripherals from different sources
//! including crystal oscillators, PLLs, and internal oscillators.
//!
//! ## Clock Architecture
//!
//! The RP2350 has several configurable clocks:
//! - `clk_ref`: Reference clock (typically XOSC)
//! - `clk_sys`: System clock (CPU and most peripherals)
//! - `clk_peri`: Peripheral clock
//! - `clk_usb`: USB clock (must be 48 MHz for USB)
//! - `clk_adc`: ADC clock
//! - `clk_hstx`: High-speed TX clock
//! - `clk_gpout0..3`: General purpose clock outputs
//!
//! ## Example
//!
//! ```ignore
//! use pico_rs::clocks::{Clocks, ClockId, ClkSysAuxSrc};
//!
//! // Get clocks instance
//! let clocks = Clocks::new();
//!
//! // Configure sys clock from PLL_SYS at 150 MHz
//! clocks.configure_sys_clock(ClkSysAuxSrc::PllSys, 150_000_000, 150_000_000);
//!
//! // Get current system clock frequency
//! let sys_freq = clocks.get_sys_freq();
//! ```

#![allow(clippy::identity_op)]

use pico_rs_pac::CLOCKS;

/// Number of configurable clocks
pub const CLK_COUNT: usize = 10;

/// Clock identifiers
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(usize)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClockId {
    /// General purpose clock output 0
    GpOut0 = 0,
    /// General purpose clock output 1
    GpOut1 = 1,
    /// General purpose clock output 2
    GpOut2 = 2,
    /// General purpose clock output 3
    GpOut3 = 3,
    /// Reference clock
    Ref = 4,
    /// System clock
    Sys = 5,
    /// Peripheral clock
    Peri = 6,
    /// High-speed TX clock
    Hstx = 7,
    /// USB clock
    Usb = 8,
    /// ADC clock
    Adc = 9,
}

/// Reference clock source selection
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkRefSrc {
    /// Ring oscillator (ROSC)
    Rosc = 0,
    /// Auxiliary source (use with ClkRefAuxSrc)
    Aux = 1,
    /// Crystal oscillator (XOSC)
    Xosc = 2,
    /// Low-power oscillator (LPOSC)
    Lposc = 3,
}

/// Reference clock auxiliary source selection
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkRefAuxSrc {
    /// PLL USB
    PllUsb = 0,
    /// GPIO input 0
    GpIn0 = 1,
    /// GPIO input 1
    GpIn1 = 2,
    /// PLL USB primary reference OPCG
    PllUsbPrimaryRefOpcg = 3,
}

/// System clock source selection
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkSysSrc {
    /// Reference clock
    ClkRef = 0,
    /// Auxiliary source (use with ClkSysAuxSrc)
    Aux = 1,
}

/// System clock auxiliary source selection
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkSysAuxSrc {
    /// PLL System
    PllSys = 0,
    /// PLL USB
    PllUsb = 1,
    /// Ring oscillator (ROSC)
    Rosc = 2,
    /// Crystal oscillator (XOSC)
    Xosc = 3,
    /// GPIO input 0
    GpIn0 = 4,
    /// GPIO input 1
    GpIn1 = 5,
}

/// Peripheral clock auxiliary source selection
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkPeriAuxSrc {
    /// System clock
    ClkSys = 0,
    /// PLL System
    PllSys = 1,
    /// PLL USB
    PllUsb = 2,
    /// Ring oscillator (ROSC)
    Rosc = 3,
    /// Crystal oscillator (XOSC)
    Xosc = 4,
    /// GPIO input 0
    GpIn0 = 5,
    /// GPIO input 1
    GpIn1 = 6,
}

/// USB clock auxiliary source selection
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkUsbAuxSrc {
    /// PLL USB
    PllUsb = 0,
    /// PLL System
    PllSys = 1,
    /// Ring oscillator (ROSC)
    Rosc = 2,
    /// Crystal oscillator (XOSC)
    Xosc = 3,
    /// GPIO input 0
    GpIn0 = 4,
    /// GPIO input 1
    GpIn1 = 5,
}

/// ADC clock auxiliary source selection
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkAdcAuxSrc {
    /// PLL USB
    PllUsb = 0,
    /// PLL System
    PllSys = 1,
    /// Ring oscillator (ROSC)
    Rosc = 2,
    /// Crystal oscillator (XOSC)
    Xosc = 3,
    /// GPIO input 0
    GpIn0 = 4,
    /// GPIO input 1
    GpIn1 = 5,
}

/// HSTX clock auxiliary source selection
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkHstxAuxSrc {
    /// System clock
    ClkSys = 0,
    /// PLL System
    PllSys = 1,
    /// PLL USB
    PllUsb = 2,
    /// GPIO input 0
    GpIn0 = 3,
    /// GPIO input 1
    GpIn1 = 4,
}

/// GPOUT clock auxiliary source selection
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkGpOutAuxSrc {
    /// PLL System
    PllSys = 0,
    /// GPIO input 0
    GpIn0 = 1,
    /// GPIO input 1
    GpIn1 = 2,
    /// PLL USB
    PllUsb = 3,
    /// PLL USB primary reference OPCG
    PllUsbPrimaryRefOpcg = 4,
    /// Ring oscillator (ROSC)
    Rosc = 5,
    /// Crystal oscillator (XOSC)
    Xosc = 6,
    /// Low-power oscillator (LPOSC)
    Lposc = 7,
    /// System clock
    ClkSys = 8,
    /// USB clock
    ClkUsb = 9,
    /// ADC clock
    ClkAdc = 10,
    /// Reference clock
    ClkRef = 11,
    /// Peripheral clock
    ClkPeri = 12,
    /// HSTX clock
    ClkHstx = 13,
    /// OTP clock to FC
    OtpClk2Fc = 14,
}

/// Frequency counter source
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FcSrc {
    Null = 0,
    PllSysPrimary = 1,
    PllUsbPrimary = 2,
    RoscClkSrc = 3,
    RoscClkSrcPh = 4,
    XoscClkSrc = 5,
    ClkSrcGpIn0 = 6,
    ClkSrcGpIn1 = 7,
    ClkRef = 8,
    ClkSys = 9,
    ClkPeri = 10,
    ClkUsb = 11,
    ClkAdc = 12,
    ClkHstx = 13,
    LposcClkSrc = 14,
    OtpClk2Fc = 15,
    PllUsbPrimaryRefOpcg = 16,
}

/// Clock configuration state
#[derive(Clone, Copy, Default)]
struct ClockFrequencies {
    gpout0: u32,
    gpout1: u32,
    gpout2: u32,
    gpout3: u32,
    ref_clk: u32,
    sys_clk: u32,
    peri_clk: u32,
    hstx_clk: u32,
    usb_clk: u32,
    adc_clk: u32,
}

/// Clock management peripheral wrapper
pub struct Clocks {
    clocks: &'static pico_rs_pac::clocks::RegisterBlock,
    frequencies: ClockFrequencies,
}

impl Clocks {
    /// Create a new Clocks instance
    ///
    /// # Safety
    /// This does not track ownership. The caller must ensure exclusive access
    /// to the CLOCKS peripheral if needed.
    #[inline]
    pub fn new() -> Self {
        Self {
            clocks: unsafe { &*CLOCKS::PTR },
            frequencies: ClockFrequencies::default(),
        }
    }

    // ========================================================================
    // Reference Clock (CLK_REF) Configuration
    // ========================================================================

    /// Configure the reference clock source
    ///
    /// The reference clock provides a stable clock for the system clock mux
    /// and other peripherals. It should typically be sourced from XOSC.
    ///
    /// # Arguments
    /// * `src` - The clock source
    /// * `src_freq` - The source frequency in Hz
    #[inline]
    pub fn configure_ref_src(&mut self, src: ClkRefSrc, src_freq: u32) {
        // Write source selection
        self.clocks.clk_ref_ctrl.write_src(src as u32);

        // Wait for the source to be selected
        while (self.clocks.clk_ref_selected.clk_ref_selected() & (1 << src as u32)) == 0 {
            core::hint::spin_loop();
        }

        self.frequencies.ref_clk = src_freq;
    }

    /// Configure the reference clock with auxiliary source
    ///
    /// # Arguments
    /// * `auxsrc` - The auxiliary source
    /// * `src_freq` - The source frequency in Hz
    pub fn configure_ref_aux(&mut self, auxsrc: ClkRefAuxSrc, src_freq: u32) {
        // First switch away from aux to avoid glitches
        self.clocks.clk_ref_ctrl.write_src(ClkRefSrc::Rosc as u32);
        while (self.clocks.clk_ref_selected.clk_ref_selected() & 1) == 0 {
            core::hint::spin_loop();
        }

        // Set aux source
        self.clocks.clk_ref_ctrl.write_auxsrc(auxsrc as u32);

        // Switch to aux
        self.clocks.clk_ref_ctrl.write_src(ClkRefSrc::Aux as u32);
        while (self.clocks.clk_ref_selected.clk_ref_selected() & (1 << ClkRefSrc::Aux as u32)) == 0
        {
            core::hint::spin_loop();
        }

        self.frequencies.ref_clk = src_freq;
    }

    /// Set the reference clock divider
    ///
    /// # Arguments
    /// * `int_div` - Integer divider (1-255, 0 = divide by 256)
    #[inline]
    pub fn set_ref_div(&mut self, int_div: u8) {
        self.clocks.clk_ref_div.write_int(int_div as u32);
        if int_div != 0 {
            self.frequencies.ref_clk /= int_div as u32;
        }
    }

    /// Get the current reference clock frequency
    #[inline]
    pub fn get_ref_freq(&self) -> u32 {
        self.frequencies.ref_clk
    }

    // ========================================================================
    // System Clock (CLK_SYS) Configuration
    // ========================================================================

    /// Configure the system clock from reference clock
    ///
    /// This sets the system clock to use the reference clock directly.
    #[inline]
    pub fn configure_sys_from_ref(&mut self) {
        // Switch to ref clock (glitchless)
        self.clocks.clk_sys_ctrl.clr_src();
        while (self.clocks.clk_sys_selected.clk_sys_selected() & 1) == 0 {
            core::hint::spin_loop();
        }
        self.frequencies.sys_clk = self.frequencies.ref_clk;
    }

    /// Configure the system clock from an auxiliary source
    ///
    /// # Arguments
    /// * `auxsrc` - The auxiliary source (typically PLL_SYS)
    /// * `src_freq` - The source frequency in Hz
    /// * `target_freq` - The desired system clock frequency in Hz
    pub fn configure_sys_aux(&mut self, auxsrc: ClkSysAuxSrc, src_freq: u32, target_freq: u32) {
        // Calculate divisor (8.16 fixed point format)
        let div = if src_freq > target_freq {
            (((src_freq as u64) << 16) / target_freq as u64) as u32
        } else {
            1 << 16 // Divisor of 1
        };

        // If increasing divisor, set divisor before source
        let current_div = (self.clocks.clk_sys_div.int() << 16) | self.clocks.clk_sys_div.frac();
        if div > current_div {
            self.clocks.clk_sys_div.write_int((div >> 16) & 0xffff);
            self.clocks.clk_sys_div.write_frac(div & 0xffff);
        }

        // Switch away from aux first to avoid glitches
        self.clocks.clk_sys_ctrl.clr_src();
        while (self.clocks.clk_sys_selected.clk_sys_selected() & 1) == 0 {
            core::hint::spin_loop();
        }

        // Set aux source
        self.clocks.clk_sys_ctrl.write_auxsrc(auxsrc as u32);

        // Switch to aux
        self.clocks.clk_sys_ctrl.set_src();
        while (self.clocks.clk_sys_selected.clk_sys_selected() & 2) == 0 {
            core::hint::spin_loop();
        }

        // Set divisor (now safe after source is configured)
        self.clocks.clk_sys_div.write_int((div >> 16) & 0xffff);
        self.clocks.clk_sys_div.write_frac(div & 0xffff);

        // Calculate actual frequency
        self.frequencies.sys_clk = ((src_freq as u64) << 16) as u32 / div;
    }

    /// Set the system clock divisor
    ///
    /// # Arguments
    /// * `int_div` - Integer part of divisor (16 bits)
    /// * `frac_div` - Fractional part of divisor (16 bits)
    #[inline]
    pub fn set_sys_div(&mut self, int_div: u16, frac_div: u16) {
        self.clocks.clk_sys_div.write_int(int_div as u32);
        self.clocks.clk_sys_div.write_frac(frac_div as u32);
    }

    /// Get the current system clock frequency
    #[inline]
    pub fn get_sys_freq(&self) -> u32 {
        self.frequencies.sys_clk
    }

    /// Set the system clock frequency directly (for tracking purposes)
    #[inline]
    pub fn set_sys_freq(&mut self, freq: u32) {
        self.frequencies.sys_clk = freq;
    }

    // ========================================================================
    // Peripheral Clock (CLK_PERI) Configuration
    // ========================================================================

    /// Configure the peripheral clock
    ///
    /// # Arguments
    /// * `auxsrc` - The auxiliary source
    /// * `src_freq` - The source frequency in Hz
    pub fn configure_peri(&mut self, auxsrc: ClkPeriAuxSrc, src_freq: u32) {
        // Disable clock first
        self.clocks.clk_peri_ctrl.clr_enable();

        // Small delay for disable to propagate
        for _ in 0..3 {
            core::hint::spin_loop();
        }

        // Set aux source
        self.clocks.clk_peri_ctrl.write_auxsrc(auxsrc as u32);

        // Enable clock
        self.clocks.clk_peri_ctrl.set_enable();

        self.frequencies.peri_clk = src_freq;
    }

    /// Get the current peripheral clock frequency
    #[inline]
    pub fn get_peri_freq(&self) -> u32 {
        self.frequencies.peri_clk
    }

    // ========================================================================
    // USB Clock (CLK_USB) Configuration
    // ========================================================================

    /// Configure the USB clock
    ///
    /// USB requires a 48 MHz clock. This configures the USB clock from the
    /// specified source.
    ///
    /// # Arguments
    /// * `auxsrc` - The auxiliary source (typically PLL_USB)
    /// * `src_freq` - The source frequency in Hz (should be 48 MHz or divisible to 48 MHz)
    pub fn configure_usb(&mut self, auxsrc: ClkUsbAuxSrc, src_freq: u32) {
        // Disable clock first
        self.clocks.clk_usb_ctrl.clr_enable();

        // Small delay for disable to propagate
        for _ in 0..3 {
            core::hint::spin_loop();
        }

        // Set aux source
        self.clocks.clk_usb_ctrl.write_auxsrc(auxsrc as u32);

        // Set divisor for 48 MHz if needed
        if src_freq > 48_000_000 {
            let div = src_freq / 48_000_000;
            self.clocks.clk_usb_div.write_int(div);
            self.frequencies.usb_clk = 48_000_000;
        } else {
            self.clocks.clk_usb_div.write_int(1);
            self.frequencies.usb_clk = src_freq;
        }

        // Enable clock
        self.clocks.clk_usb_ctrl.set_enable();
    }

    /// Get the current USB clock frequency
    #[inline]
    pub fn get_usb_freq(&self) -> u32 {
        self.frequencies.usb_clk
    }

    // ========================================================================
    // ADC Clock (CLK_ADC) Configuration
    // ========================================================================

    /// Configure the ADC clock
    ///
    /// ADC requires a 48 MHz clock for correct operation.
    ///
    /// # Arguments
    /// * `auxsrc` - The auxiliary source (typically PLL_USB)
    /// * `src_freq` - The source frequency in Hz
    pub fn configure_adc(&mut self, auxsrc: ClkAdcAuxSrc, src_freq: u32) {
        // Disable clock first
        self.clocks.clk_adc_ctrl.clr_enable();

        // Small delay
        for _ in 0..3 {
            core::hint::spin_loop();
        }

        // Set aux source
        self.clocks.clk_adc_ctrl.write_auxsrc(auxsrc as u32);

        // Set divisor for 48 MHz if needed
        if src_freq > 48_000_000 {
            let div = src_freq / 48_000_000;
            self.clocks.clk_adc_div.write_int(div);
            self.frequencies.adc_clk = 48_000_000;
        } else {
            self.clocks.clk_adc_div.write_int(1);
            self.frequencies.adc_clk = src_freq;
        }

        // Enable clock
        self.clocks.clk_adc_ctrl.set_enable();
    }

    /// Get the current ADC clock frequency
    #[inline]
    pub fn get_adc_freq(&self) -> u32 {
        self.frequencies.adc_clk
    }

    // ========================================================================
    // HSTX Clock (CLK_HSTX) Configuration
    // ========================================================================

    /// Configure the HSTX clock
    ///
    /// # Arguments
    /// * `auxsrc` - The auxiliary source
    /// * `src_freq` - The source frequency in Hz
    pub fn configure_hstx(&mut self, auxsrc: ClkHstxAuxSrc, src_freq: u32) {
        // Disable clock first
        self.clocks.clk_hstx_ctrl.clr_enable();

        // Small delay
        for _ in 0..3 {
            core::hint::spin_loop();
        }

        // Set aux source
        self.clocks.clk_hstx_ctrl.write_auxsrc(auxsrc as u32);

        // Set divisor to 1
        self.clocks.clk_hstx_div.write_int(1);

        // Enable clock
        self.clocks.clk_hstx_ctrl.set_enable();

        self.frequencies.hstx_clk = src_freq;
    }

    /// Get the current HSTX clock frequency
    #[inline]
    pub fn get_hstx_freq(&self) -> u32 {
        self.frequencies.hstx_clk
    }

    // ========================================================================
    // GPOUT Clock Configuration
    // ========================================================================

    /// Configure a GPOUT clock output
    ///
    /// # Arguments
    /// * `gpout` - Which GPOUT to configure (0-3)
    /// * `auxsrc` - The auxiliary source
    /// * `src_freq` - The source frequency in Hz
    /// * `target_freq` - The desired output frequency in Hz
    pub fn configure_gpout(
        &mut self,
        gpout: u8,
        auxsrc: ClkGpOutAuxSrc,
        src_freq: u32,
        target_freq: u32,
    ) {
        assert!(gpout < 4, "GPOUT must be 0-3");

        // Calculate divisor (16.16 fixed point)
        let div = if src_freq > target_freq {
            (((src_freq as u64) << 16) / target_freq as u64) as u32
        } else {
            1 << 16
        };

        match gpout {
            0 => {
                self.clocks.clk_gpout0_ctrl.clr_enable();
                self.clocks.clk_gpout0_ctrl.write_auxsrc(auxsrc as u32);
                self.clocks.clk_gpout0_div.write_int((div >> 16) & 0xffff);
                self.clocks.clk_gpout0_div.write_frac(div & 0xffff);
                self.clocks.clk_gpout0_ctrl.set_enable();
                self.frequencies.gpout0 = ((src_freq as u64) << 16) as u32 / div;
            }
            1 => {
                self.clocks.clk_gpout1_ctrl.clr_enable();
                self.clocks.clk_gpout1_ctrl.write_auxsrc(auxsrc as u32);
                self.clocks.clk_gpout1_div.write_int((div >> 16) & 0xffff);
                self.clocks.clk_gpout1_div.write_frac(div & 0xffff);
                self.clocks.clk_gpout1_ctrl.set_enable();
                self.frequencies.gpout1 = ((src_freq as u64) << 16) as u32 / div;
            }
            2 => {
                self.clocks.clk_gpout2_ctrl.clr_enable();
                self.clocks.clk_gpout2_ctrl.write_auxsrc(auxsrc as u32);
                self.clocks.clk_gpout2_div.write_int((div >> 16) & 0xffff);
                self.clocks.clk_gpout2_div.write_frac(div & 0xffff);
                self.clocks.clk_gpout2_ctrl.set_enable();
                self.frequencies.gpout2 = ((src_freq as u64) << 16) as u32 / div;
            }
            3 => {
                self.clocks.clk_gpout3_ctrl.clr_enable();
                self.clocks.clk_gpout3_ctrl.write_auxsrc(auxsrc as u32);
                self.clocks.clk_gpout3_div.write_int((div >> 16) & 0xffff);
                self.clocks.clk_gpout3_div.write_frac(div & 0xffff);
                self.clocks.clk_gpout3_ctrl.set_enable();
                self.frequencies.gpout3 = ((src_freq as u64) << 16) as u32 / div;
            }
            _ => unreachable!(),
        }
    }

    /// Disable a GPOUT clock output
    #[inline]
    pub fn disable_gpout(&mut self, gpout: u8) {
        match gpout {
            0 => self.clocks.clk_gpout0_ctrl.clr_enable(),
            1 => self.clocks.clk_gpout1_ctrl.clr_enable(),
            2 => self.clocks.clk_gpout2_ctrl.clr_enable(),
            3 => self.clocks.clk_gpout3_ctrl.clr_enable(),
            _ => panic!("GPOUT must be 0-3"),
        }
    }

    // ========================================================================
    // Clock Stopping
    // ========================================================================

    /// Stop a clock generator
    ///
    /// # Arguments
    /// * `clock` - The clock to stop
    pub fn stop_clock(&mut self, clock: ClockId) {
        match clock {
            ClockId::GpOut0 => {
                self.clocks.clk_gpout0_ctrl.clr_enable();
                self.frequencies.gpout0 = 0;
            }
            ClockId::GpOut1 => {
                self.clocks.clk_gpout1_ctrl.clr_enable();
                self.frequencies.gpout1 = 0;
            }
            ClockId::GpOut2 => {
                self.clocks.clk_gpout2_ctrl.clr_enable();
                self.frequencies.gpout2 = 0;
            }
            ClockId::GpOut3 => {
                self.clocks.clk_gpout3_ctrl.clr_enable();
                self.frequencies.gpout3 = 0;
            }
            ClockId::Peri => {
                self.clocks.clk_peri_ctrl.clr_enable();
                self.frequencies.peri_clk = 0;
            }
            ClockId::Hstx => {
                self.clocks.clk_hstx_ctrl.clr_enable();
                self.frequencies.hstx_clk = 0;
            }
            ClockId::Usb => {
                self.clocks.clk_usb_ctrl.clr_enable();
                self.frequencies.usb_clk = 0;
            }
            ClockId::Adc => {
                self.clocks.clk_adc_ctrl.clr_enable();
                self.frequencies.adc_clk = 0;
            }
            // Ref and Sys cannot be stopped
            ClockId::Ref | ClockId::Sys => {}
        }
    }

    // ========================================================================
    // Frequency Counter
    // ========================================================================

    /// Measure a clock frequency using the frequency counter
    ///
    /// # Arguments
    /// * `src` - The clock source to measure
    ///
    /// # Returns
    /// The measured frequency in kHz
    pub fn frequency_count_khz(&self, src: FcSrc) -> u32 {
        // Wait for any previous measurement to complete
        while self.clocks.fc0_status.running() {
            core::hint::spin_loop();
        }

        // Set reference frequency
        self.clocks
            .fc0_ref_khz
            .write_fc0_ref_khz(self.frequencies.ref_clk / 1000);

        // Set test interval (1us * 2^10 = ~1ms measurement)
        self.clocks.fc0_interval.write_fc0_interval(10);

        // No min/max limits
        self.clocks.fc0_min_khz.write_fc0_min_khz(0);
        self.clocks.fc0_max_khz.write_fc0_max_khz(0x1ffffff);

        // Start measurement by writing source
        self.clocks.fc0_src.write_fc0_src(src as u32);

        // Wait for done
        while !self.clocks.fc0_status.done() {
            core::hint::spin_loop();
        }

        // Return result (kHz in upper bits)
        self.clocks.fc0_result.khz()
    }

    // ========================================================================
    // Resus (System Clock Recovery)
    // ========================================================================

    /// Enable system clock resuscitation
    ///
    /// If clk_sys stops, it will be automatically switched back to clk_ref.
    ///
    /// # Arguments
    /// * `timeout` - Timeout in clk_ref cycles before resus kicks in
    #[inline]
    pub fn enable_resus(&self, timeout: u8) {
        self.clocks.clk_sys_resus_ctrl.set_enable();
        self.clocks.clk_sys_resus_ctrl.write_timeout(timeout as u32);
    }

    /// Disable system clock resuscitation
    #[inline]
    pub fn disable_resus(&self) {
        self.clocks.clk_sys_resus_ctrl.clr_enable();
    }

    /// Check if resus has been triggered
    #[inline]
    pub fn resus_triggered(&self) -> bool {
        self.clocks.clk_sys_resus_status.resussed()
    }

    /// Clear the resus status
    #[inline]
    pub fn clear_resus(&self) {
        self.clocks.clk_sys_resus_ctrl.set_bits(1 << 16); // CLEAR bit
        self.clocks.clk_sys_resus_ctrl.clr_bits(1 << 16);
    }

    // ========================================================================
    // Utility Functions
    // ========================================================================

    /// Get the frequency of a specific clock
    #[inline]
    pub fn get_freq(&self, clock: ClockId) -> u32 {
        match clock {
            ClockId::GpOut0 => self.frequencies.gpout0,
            ClockId::GpOut1 => self.frequencies.gpout1,
            ClockId::GpOut2 => self.frequencies.gpout2,
            ClockId::GpOut3 => self.frequencies.gpout3,
            ClockId::Ref => self.frequencies.ref_clk,
            ClockId::Sys => self.frequencies.sys_clk,
            ClockId::Peri => self.frequencies.peri_clk,
            ClockId::Hstx => self.frequencies.hstx_clk,
            ClockId::Usb => self.frequencies.usb_clk,
            ClockId::Adc => self.frequencies.adc_clk,
        }
    }

    /// Set a reported frequency (for tracking when configured externally)
    #[inline]
    pub fn set_freq(&mut self, clock: ClockId, freq: u32) {
        match clock {
            ClockId::GpOut0 => self.frequencies.gpout0 = freq,
            ClockId::GpOut1 => self.frequencies.gpout1 = freq,
            ClockId::GpOut2 => self.frequencies.gpout2 = freq,
            ClockId::GpOut3 => self.frequencies.gpout3 = freq,
            ClockId::Ref => self.frequencies.ref_clk = freq,
            ClockId::Sys => self.frequencies.sys_clk = freq,
            ClockId::Peri => self.frequencies.peri_clk = freq,
            ClockId::Hstx => self.frequencies.hstx_clk = freq,
            ClockId::Usb => self.frequencies.usb_clk = freq,
            ClockId::Adc => self.frequencies.adc_clk = freq,
        }
    }

    /// Get reference to the underlying register block
    ///
    /// # Safety
    /// Direct register access can bypass the HAL's safety guarantees.
    #[inline]
    pub unsafe fn registers(&self) -> &pico_rs_pac::clocks::RegisterBlock {
        self.clocks
    }
}

impl Default for Clocks {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Convenience Functions
// ============================================================================

/// Initialize clocks to a typical configuration with 150 MHz system clock
///
/// This assumes:
/// - 12 MHz crystal oscillator
/// - PLL_SYS configured for 150 MHz
/// - PLL_USB configured for 48 MHz
///
/// # Arguments
/// * `xosc` - Reference to an initialized XOSC
///
/// # Returns
/// A configured Clocks instance with typical frequencies
pub fn init_default(xosc_freq: u32) -> Clocks {
    let mut clocks = Clocks::new();

    // Configure reference clock from XOSC
    clocks.configure_ref_src(ClkRefSrc::Xosc, xosc_freq);

    clocks
}
