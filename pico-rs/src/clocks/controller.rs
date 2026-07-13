//! Core clock controller for RP2350
//!
//! This module provides the [`Clocks`] struct — the central object for
//! configuring and querying all clocks in the RP2350 clock tree.
//!
//! # Initialization
//!
//! Use [`ClockBuilder`] for typical initialization. Constructing `Clocks`
//! directly via [`Clocks::new`] is the lower-level path for custom setups.
//!
//! [`ClockBuilder`]: super::builder::ClockBuilder

use pico_rs_pac::{PLL_SYS, PLL_USB};

use super::gate::ClkPeripheral;
use super::pll::{PllConfig, PllSys, PllUsb};
use super::rosc::{self, Rosc};
use super::types::{
    ClkAdcAuxSrc, ClkGpOutAuxSrc, ClkHstxAuxSrc, ClkPeriAuxSrc, ClkRefAuxSrc, ClkRefSrc,
    ClkSysAuxSrc, ClkUsbAuxSrc, ClockId, DftClkSrc, FcSrc, GpOutId,
};
use super::xosc::{self, Xosc};
use crate::reset::Resettable;

// ============================================================================
// Clock Frequency Tracking
// ============================================================================

/// Internal record of configured clock frequencies (Hz)
///
/// Updated by every `configure_*` call so callers can query frequencies
/// without reading hardware registers.
#[derive(Clone, Copy, Default)]
pub(super) struct ClockFreqs {
    // GPIO clock outputs
    pub gpout0: u32,
    pub gpout1: u32,
    pub gpout2: u32,
    pub gpout3: u32,
    // Layer 1: core clocks
    pub ref_clk: u32,
    pub sys_clk: u32,
    // Layer 2: peripheral clocks
    pub peri_clk: u32,
    pub hstx_clk: u32,
    pub usb_clk: u32,
    pub adc_clk: u32,
}

// ============================================================================
// Main Clock Controller
// ============================================================================

/// Clock management peripheral wrapper
///
/// Provides methods to configure every clock in the RP2350 clock tree.
/// Methods are grouped by clock tree layer:
///
/// **Layer 1 — Reference Clock**
/// - [`configure_ref_src`](Self::configure_ref_src)
/// - [`configure_ref_aux`](Self::configure_ref_aux)
/// - [`set_ref_div`](Self::set_ref_div)
/// - [`ref_freq`](Self::ref_freq)
///
/// **Layer 1 — System Clock**
/// - [`configure_sys_from_ref`](Self::configure_sys_from_ref)
/// - [`configure_sys_aux`](Self::configure_sys_aux)
/// - [`sys_freq`](Self::sys_freq)
///
/// **Layer 2 — Peripheral Clocks**
/// - [`configure_peri`](Self::configure_peri), [`peri_freq`](Self::peri_freq)
/// - [`configure_usb`](Self::configure_usb), [`usb_freq`](Self::usb_freq)
/// - [`configure_adc`](Self::configure_adc), [`adc_freq`](Self::adc_freq)
/// - [`configure_hstx`](Self::configure_hstx), [`hstx_freq`](Self::hstx_freq)
///
/// **GPIO Clock Outputs**
/// - [`configure_gpout`](Self::configure_gpout), [`disable_gpout`](Self::disable_gpout)
///
/// **Utilities**
/// - [`frequency_count_khz`](Self::frequency_count_khz)
/// - [`enable_resus`](Self::enable_resus)
/// - [`dormant`](Self::dormant)
///
/// For easy one-shot initialization, prefer [`ClockBuilder`]:
/// ```ignore
/// let clocks = ClockBuilder::new().build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);
/// // clk_sys = 150 MHz, clk_usb = 48 MHz, clk_peri = 150 MHz
/// ```
///
/// [`ClockBuilder`]: super::builder::ClockBuilder
pub struct Clocks {
    pub(super) clocks: pico_rs_pac::CLOCKS,
    pub(super) xosc: Xosc,
    pub(super) frequencies: ClockFreqs,
    /// Current reference clock source — drives dormant mode logic
    pub(super) ref_src: ClkRefSrc,
    /// Saved PLL_SYS configuration for dormant wake restoration
    pub(super) pll_sys_config: Option<PllConfig>,
    /// Saved PLL_USB configuration for dormant wake restoration
    pub(super) pll_usb_config: Option<PllConfig>,
}

impl Clocks {
    /// Initialize a bare `Clocks` instance
    ///
    /// Initialises frequency tracking to zero and assumes the reference clock
    /// is still being driven by ROSC (the hardware default after reset).
    ///
    /// # Arguments
    /// * `clocks` — CLOCKS peripheral from PAC
    /// * `xosc`   — Initialised and stable XOSC instance
    ///
    /// # Note
    /// Prefer [`ClockBuilder::build`] for a complete clock-tree setup.
    ///
    /// [`ClockBuilder::build`]: super::builder::ClockBuilder::build
    #[inline]
    pub fn init(clocks: pico_rs_pac::CLOCKS, xosc: Xosc) -> Self {
        Self {
            clocks,
            xosc,
            frequencies: ClockFreqs::default(),
            ref_src: ClkRefSrc::Rosc, // hardware default at boot
            pll_sys_config: None,
            pll_usb_config: None,
        }
    }

    // ========================================================================
    // Accessors
    // ========================================================================

    /// Borrow the XOSC instance
    #[inline]
    pub fn xosc(&self) -> &Xosc {
        &self.xosc
    }

    /// Raw access to the CLOCKS register block
    ///
    /// # Safety
    /// Direct register writes can bypass HAL state tracking.
    #[inline]
    pub unsafe fn regs(&self) -> &pico_rs_pac::clocks::RegisterBlock {
        &self.clocks
    }

    /// Consume `Clocks` and release the underlying CLOCKS peripheral
    #[inline]
    pub fn release(self) -> pico_rs_pac::CLOCKS {
        self.clocks
    }

    // ========================================================================
    // ██████╗ ██╗      █████╗ ██╗   ██╗███████╗██████╗      ██╗
    // ██╔══██╗██║     ██╔══██╗╚██╗ ██╔╝██╔════╝██╔══██╗    ███║
    // ██████╔╝██║     ███████║ ╚████╔╝ █████╗  ██████╔╝    ╚██║
    // ██╔═══╝ ██║     ██╔══██║  ╚██╔╝  ██╔══╝  ██╔══██╗     ██║
    // ██║     ███████╗██║  ██║   ██║   ███████╗██║  ██║     ██║
    // ╚═╝     ╚══════╝╚═╝  ╚═╝   ╚═╝   ╚══════╝╚═╝  ╚═╝     ╚═╝
    // Reference Clock (CLK_REF) — stable clock for clk_sys mux & watchdog
    // ========================================================================
    //
    // CLK_REF has a glitchless mux with four direct sources (ROSC, XOSC,
    // LPOSC, AUX) and an auxiliary mux for PLL_USB or GPIN signals.
    //
    // Typical configuration:
    //   1. Configure ref clock to XOSC (12 MHz)
    //   2. Use clk_ref as the safe intermediate source while switching clk_sys
    //
    // ========================================================================

    /// Configure the reference clock from a glitchless source
    ///
    /// Writes the source selection and polls until the hardware confirms it.
    ///
    /// # Arguments
    /// * `src`      — Glitchless source to select
    /// * `src_freq` — Frequency of that source in Hz (used for tracking only)
    #[inline]
    pub fn configure_ref_src(&mut self, src: ClkRefSrc, src_freq: u32) {
        self.clocks.clk_ref_ctrl.write_src(src as u32);
        while (self.clocks.clk_ref_selected.clk_ref_selected() & (1 << src as u32)) == 0 {
            core::hint::spin_loop();
        }
        self.ref_src = src;
        self.frequencies.ref_clk = src_freq;
    }

    /// Configure the reference clock through the auxiliary mux
    ///
    /// Safely switches via ROSC first to avoid glitches on the AUX path.
    ///
    /// # Arguments
    /// * `auxsrc`   — Auxiliary source (PLL_USB, GPIN0, GPIN1, …)
    /// * `src_freq` — Frequency of that auxiliary source in Hz
    pub fn configure_ref_aux(&mut self, auxsrc: ClkRefAuxSrc, src_freq: u32) {
        // Step 1: briefly switch to ROSC to avoid glitches while changing aux
        self.clocks.clk_ref_ctrl.write_src(ClkRefSrc::Rosc as u32);
        while (self.clocks.clk_ref_selected.clk_ref_selected() & 1) == 0 {
            core::hint::spin_loop();
        }

        // Step 2: set auxiliary source and switch the main mux to AUX
        self.clocks.clk_ref_ctrl.write_auxsrc(auxsrc as u32);
        self.clocks.clk_ref_ctrl.write_src(ClkRefSrc::Aux as u32);
        while (self.clocks.clk_ref_selected.clk_ref_selected() & (1 << ClkRefSrc::Aux as u32)) == 0
        {
            core::hint::spin_loop();
        }

        self.ref_src = ClkRefSrc::Aux;
        self.frequencies.ref_clk = src_freq;
    }

    /// Set the reference clock integer divider
    ///
    /// # Arguments
    /// * `int_div` — Integer divisor (1–255; writing 0 divides by 256)
    ///
    /// # Note
    /// This also updates the tracked `ref_clk` frequency to `prev / int_div`.
    /// Call before any dependent clocks are configured.
    #[inline]
    pub fn set_ref_div(&mut self, int_div: u8) {
        self.clocks.clk_ref_div.write_int(int_div as u32);
        if int_div != 0 {
            self.frequencies.ref_clk /= int_div as u32;
        }
    }

    /// Current reference clock frequency in Hz
    #[inline]
    pub fn ref_freq(&self) -> u32 {
        self.frequencies.ref_clk
    }

    // ========================================================================
    // ██████╗ ██╗      █████╗ ██╗   ██╗███████╗██████╗      ██╗
    // ██╔══██╗██║     ██╔══██╗╚██╗ ██╔╝██╔════╝██╔══██╗    ███║
    // ██████╔╝██║     ███████║ ╚████╔╝ █████╗  ██████╔╝    ╚██║
    // ██╔═══╝ ██║     ██╔══██║  ╚██╔╝  ██╔══╝  ██╔══██╗     ██║
    // ██║     ███████╗██║  ██║   ██║   ███████╗██║  ██║     ██║
    // ╚═╝     ╚══════╝╚═╝  ╚═╝   ╚═╝   ╚══════╝╚═╝  ╚═╝     ╚═╝
    // System Clock (CLK_SYS) — CPU, AHB, APB; max 150 MHz
    // ========================================================================
    //
    // CLK_SYS has a glitchless mux between clk_ref (safe fallback) and an
    // auxiliary mux (PLL_SYS, PLL_USB, ROSC, XOSC, GPIN).
    //
    // Safe switching sequence when moving to a faster aux source:
    //   1. Switch clk_sys to clk_ref  ← glitchless, safe
    //   2. Configure the aux source
    //   3. If div increases, set divider before switching source
    //   4. Switch clk_sys to AUX
    //   5. Apply final divider
    //
    // ========================================================================

    /// Switch the system clock to the reference clock
    ///
    /// Used as a safe intermediate step when reconfiguring an auxiliary source.
    /// The switch is glitchless and does not require a divider change.
    #[inline]
    pub fn configure_sys_from_ref(&mut self) {
        self.clocks.clk_sys_ctrl.clr_src();
        while (self.clocks.clk_sys_selected.clk_sys_selected() & 1) == 0 {
            core::hint::spin_loop();
        }
        self.frequencies.sys_clk = self.frequencies.ref_clk;
    }

    /// Configure the system clock from an auxiliary source with optional division
    ///
    /// Computes the 16.16 fixed-point divider and switches safely via `clk_ref`.
    ///
    /// # Arguments
    /// * `auxsrc`      — Auxiliary source (typically `PllSys`)
    /// * `src_freq`    — Frequency of the selected source in Hz
    /// * `target_freq` — Desired `clk_sys` frequency in Hz (≤ `src_freq`)
    pub fn configure_sys_aux(&mut self, auxsrc: ClkSysAuxSrc, src_freq: u32, target_freq: u32) {
        // Compute 16.16 fixed-point divider: 1.0 minimum (no multiplication)
        let div = if src_freq > target_freq {
            (((src_freq as u64) << 16) / target_freq as u64) as u32
        } else {
            1 << 16
        };

        // If the new divisor is larger, set it first to prevent momentary overspeed
        let current_div = (self.clocks.clk_sys_div.int() << 16) | self.clocks.clk_sys_div.frac();
        if div > current_div {
            self.clocks.clk_sys_div.write_int((div >> 16) & 0xffff);
            self.clocks.clk_sys_div.write_frac(div & 0xffff);
        }

        // Switch away from AUX first to avoid glitches while changing aux mux
        self.clocks.clk_sys_ctrl.clr_src();
        while (self.clocks.clk_sys_selected.clk_sys_selected() & 1) == 0 {
            core::hint::spin_loop();
        }

        // Select auxiliary source then switch the glitchless mux to AUX
        self.clocks.clk_sys_ctrl.write_auxsrc(auxsrc as u32);
        self.clocks.clk_sys_ctrl.set_src();
        while (self.clocks.clk_sys_selected.clk_sys_selected() & 2) == 0 {
            core::hint::spin_loop();
        }

        // Apply the final divider now that the source is stable
        self.clocks.clk_sys_div.write_int((div >> 16) & 0xffff);
        self.clocks.clk_sys_div.write_frac(div & 0xffff);

        // Track actual frequency: src_freq × 65536 / div
        self.frequencies.sys_clk = (((src_freq as u64) << 16) / div as u64) as u32;
    }

    /// Set the system clock 16.16 fractional divider directly
    #[inline]
    pub fn set_sys_div(&mut self, int_div: u16, frac_div: u16) {
        self.clocks.clk_sys_div.write_int(int_div as u32);
        self.clocks.clk_sys_div.write_frac(frac_div as u32);
    }

    /// Current system clock frequency in Hz
    #[inline]
    pub fn sys_freq(&self) -> u32 {
        self.frequencies.sys_clk
    }

    /// Override the tracked system clock frequency
    ///
    /// Only needed when the hardware was configured through means other than
    /// this HAL (e.g. directly via `regs()`).
    #[inline]
    pub fn set_sys_freq(&mut self, freq: u32) {
        self.frequencies.sys_clk = freq;
    }

    // ========================================================================
    // ██████╗ ██╗      █████╗ ██╗   ██╗███████╗██████╗     ██████╗
    // ██╔══██╗██║     ██╔══██╗╚██╗ ██╔╝██╔════╝██╔══██╗    ╚════██╗
    // ██████╔╝██║     ███████║ ╚████╔╝ █████╗  ██████╔╝     █████╔╝
    // ██╔═══╝ ██║     ██╔══██║  ╚██╔╝  ██╔══╝  ██╔══██╗    ██╔═══╝
    // ██║     ███████╗██║  ██║   ██║   ███████╗██║  ██║    ███████╗
    // ╚═╝     ╚══════╝╚═╝  ╚═╝   ╚═╝   ╚══════╝╚═╝  ╚═╝    ╚══════╝
    // Peripheral Clocks (Layer 2) — CLK_PERI, CLK_USB, CLK_ADC, CLK_HSTX
    // ========================================================================
    //
    // These clocks have no glitchless mux. The safe reconfiguration sequence:
    //   1. Disable the clock (wait for propagation)
    //   2. Select the new auxiliary source
    //   3. Apply the divider
    //   4. Re-enable the clock
    //
    // ========================================================================

    // -------------------------------------------------------------------------
    // CLK_PERI — Serial Peripherals (UART, SPI, I2C)
    // -------------------------------------------------------------------------

    /// Configure the peripheral clock (CLK_PERI)
    ///
    /// CLK_PERI is a non-divisible clock (no integer divider in hardware).
    ///
    /// # Arguments
    /// * `auxsrc`   — Source to route through the CLK_PERI mux
    /// * `src_freq` — Frequency of that source in Hz
    pub fn configure_peri(&mut self, auxsrc: ClkPeriAuxSrc, src_freq: u32) {
        self.clocks.clk_peri_ctrl.clr_enable();
        // Brief pause for ENABLE de-assertion to propagate (~3 clk_peri cycles)
        cortex_m_delay(3);
        self.clocks.clk_peri_ctrl.write_auxsrc(auxsrc as u32);
        self.clocks.clk_peri_ctrl.set_enable();
        self.frequencies.peri_clk = src_freq;
    }

    /// Current peripheral clock frequency in Hz
    #[inline]
    pub fn peri_freq(&self) -> u32 {
        self.frequencies.peri_clk
    }

    // -------------------------------------------------------------------------
    // CLK_USB — USB Controller (must be exactly 48 MHz)
    // -------------------------------------------------------------------------

    /// Configure the USB clock (CLK_USB)
    ///
    /// The USB hardware requires exactly 48 MHz. Set `target_freq` to
    /// `48_000_000` in normal usage. If `src_freq` is an integer multiple
    /// of `target_freq`, the correct integer divider is computed automatically.
    ///
    /// # Arguments
    /// * `auxsrc`      — Source to route through CLK_USB mux (use `PllUsb`)
    /// * `src_freq`    — Frequency of that source in Hz
    /// * `target_freq` — Desired USB clock frequency in Hz (must equal 48 MHz)
    pub fn configure_usb(&mut self, auxsrc: ClkUsbAuxSrc, src_freq: u32, target_freq: u32) {
        self.clocks.clk_usb_ctrl.clr_enable();
        cortex_m_delay(3);
        self.clocks.clk_usb_ctrl.write_auxsrc(auxsrc as u32);
        let div = src_freq.div_ceil(target_freq); // ceiling division
        self.clocks.clk_usb_div.write_int(div.max(1));
        self.clocks.clk_usb_ctrl.set_enable();
        self.frequencies.usb_clk = src_freq / div.max(1);
    }

    /// Current USB clock frequency in Hz
    #[inline]
    pub fn usb_freq(&self) -> u32 {
        self.frequencies.usb_clk
    }

    // -------------------------------------------------------------------------
    // CLK_ADC — ADC Sampling Clock (must be exactly 48 MHz)
    // -------------------------------------------------------------------------

    /// Configure the ADC clock (CLK_ADC)
    ///
    /// The ADC sampler requires exactly 48 MHz for correct sample rates.
    ///
    /// # Arguments
    /// * `auxsrc`      — Source to route through CLK_ADC mux (use `PllUsb`)
    /// * `src_freq`    — Frequency of that source in Hz
    /// * `target_freq` — Desired ADC clock frequency in Hz (must equal 48 MHz)
    pub fn configure_adc(&mut self, auxsrc: ClkAdcAuxSrc, src_freq: u32, target_freq: u32) {
        self.clocks.clk_adc_ctrl.clr_enable();
        cortex_m_delay(3);
        self.clocks.clk_adc_ctrl.write_auxsrc(auxsrc as u32);
        let div = src_freq.div_ceil(target_freq); // ceiling division
        self.clocks.clk_adc_div.write_int(div.max(1));
        self.clocks.clk_adc_ctrl.set_enable();
        self.frequencies.adc_clk = src_freq / div.max(1);
    }

    /// Current ADC clock frequency in Hz
    #[inline]
    pub fn adc_freq(&self) -> u32 {
        self.frequencies.adc_clk
    }

    // -------------------------------------------------------------------------
    // CLK_HSTX — High-Speed TX (DVI / HSTX)
    // -------------------------------------------------------------------------

    /// Configure the HSTX clock (CLK_HSTX) with optional integer division
    ///
    /// # Arguments
    /// * `auxsrc`      — Source to route through CLK_HSTX mux
    /// * `src_freq`    — Frequency of that source in Hz
    /// * `target_freq` — Desired HSTX clock frequency in Hz (≤ `src_freq`)
    pub fn configure_hstx(&mut self, auxsrc: ClkHstxAuxSrc, src_freq: u32, target_freq: u32) {
        self.clocks.clk_hstx_ctrl.clr_enable();
        cortex_m_delay(3);
        self.clocks.clk_hstx_ctrl.write_auxsrc(auxsrc as u32);
        // CLK_HSTX has an integer-only divider (no fractional bits)
        let div = if src_freq > target_freq {
            src_freq.div_ceil(target_freq) // ceiling integer division
        } else {
            1
        };
        self.clocks.clk_hstx_div.write_int(div.max(1));
        self.clocks.clk_hstx_ctrl.set_enable();
        self.frequencies.hstx_clk = src_freq / div.max(1);
    }

    /// Current HSTX clock frequency in Hz
    #[inline]
    pub fn hstx_freq(&self) -> u32 {
        self.frequencies.hstx_clk
    }

    // ========================================================================
    //  ██████╗ ██████╗  ██████╗ ██╗   ██╗████████╗
    // ██╔════╝ ██╔══██╗██╔═══██╗██║   ██║╚══██╔══╝
    // ██║  ███╗██████╔╝██║   ██║██║   ██║   ██║
    // ██║   ██║██╔═══╝ ██║   ██║██║   ██║   ██║
    // ╚██████╔╝██║     ╚██████╔╝╚██████╔╝   ██║
    //  ╚═════╝ ╚═╝      ╚═════╝  ╚═════╝    ╚═╝
    // GPIO Clock Outputs (CLK_GPOUT0–3)
    // ========================================================================
    //
    // Each GPOUT has a 16.16 fractional divider and can export almost any
    // internal clock to a GPIO pin for external measurement or as a reference.
    //
    // ========================================================================

    /// Configure a GPIO clock output
    ///
    /// # Arguments
    /// * `id`          — Which GPOUT to configure ([`GpOutId::GpOut0`]–[`GpOutId::GpOut3`])
    /// * `auxsrc`      — Source clock to route to the GPIO
    /// * `src_freq`    — Frequency of that source in Hz
    /// * `target_freq` — Desired output frequency in Hz (≤ `src_freq`)
    pub fn configure_gpout(
        &mut self,
        id: GpOutId,
        auxsrc: ClkGpOutAuxSrc,
        src_freq: u32,
        target_freq: u32,
    ) {
        let div = if src_freq > target_freq {
            (((src_freq as u64) << 16) / target_freq as u64) as u32
        } else {
            1 << 16
        };
        let actual_freq = (((src_freq as u64) << 16) / div as u64) as u32;
        let aux = auxsrc as u32;
        let int_part = (div >> 16) & 0xffff;
        let frac_part = div & 0xffff;

        // All four GPOUT have identical register layouts; just vary by instance.
        match id {
            GpOutId::GpOut0 => {
                self.clocks.clk_gpout0_ctrl.clr_enable();
                self.clocks.clk_gpout0_ctrl.write_auxsrc(aux);
                self.clocks.clk_gpout0_div.write_int(int_part);
                self.clocks.clk_gpout0_div.write_frac(frac_part);
                self.clocks.clk_gpout0_ctrl.set_enable();
                self.frequencies.gpout0 = actual_freq;
            }
            GpOutId::GpOut1 => {
                self.clocks.clk_gpout1_ctrl.clr_enable();
                self.clocks.clk_gpout1_ctrl.write_auxsrc(aux);
                self.clocks.clk_gpout1_div.write_int(int_part);
                self.clocks.clk_gpout1_div.write_frac(frac_part);
                self.clocks.clk_gpout1_ctrl.set_enable();
                self.frequencies.gpout1 = actual_freq;
            }
            GpOutId::GpOut2 => {
                self.clocks.clk_gpout2_ctrl.clr_enable();
                self.clocks.clk_gpout2_ctrl.write_auxsrc(aux);
                self.clocks.clk_gpout2_div.write_int(int_part);
                self.clocks.clk_gpout2_div.write_frac(frac_part);
                self.clocks.clk_gpout2_ctrl.set_enable();
                self.frequencies.gpout2 = actual_freq;
            }
            GpOutId::GpOut3 => {
                self.clocks.clk_gpout3_ctrl.clr_enable();
                self.clocks.clk_gpout3_ctrl.write_auxsrc(aux);
                self.clocks.clk_gpout3_div.write_int(int_part);
                self.clocks.clk_gpout3_div.write_frac(frac_part);
                self.clocks.clk_gpout3_ctrl.set_enable();
                self.frequencies.gpout3 = actual_freq;
            }
        }
    }

    /// Disable a GPIO clock output
    #[inline]
    pub fn disable_gpout(&mut self, id: GpOutId) {
        match id {
            GpOutId::GpOut0 => {
                self.clocks.clk_gpout0_ctrl.clr_enable();
                self.frequencies.gpout0 = 0;
            }
            GpOutId::GpOut1 => {
                self.clocks.clk_gpout1_ctrl.clr_enable();
                self.frequencies.gpout1 = 0;
            }
            GpOutId::GpOut2 => {
                self.clocks.clk_gpout2_ctrl.clr_enable();
                self.frequencies.gpout2 = 0;
            }
            GpOutId::GpOut3 => {
                self.clocks.clk_gpout3_ctrl.clr_enable();
                self.frequencies.gpout3 = 0;
            }
        }
    }

    // ========================================================================
    // Clock Control — Start / Stop
    // ========================================================================

    /// Disable a clock generator
    ///
    /// `Ref` and `Sys` cannot be stopped (they have no ENABLE bit).
    pub fn stop_clock(&mut self, clock: ClockId) {
        match clock {
            ClockId::GpOut0 => self.disable_gpout(GpOutId::GpOut0),
            ClockId::GpOut1 => self.disable_gpout(GpOutId::GpOut1),
            ClockId::GpOut2 => self.disable_gpout(GpOutId::GpOut2),
            ClockId::GpOut3 => self.disable_gpout(GpOutId::GpOut3),
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
            // Ref and Sys have no ENABLE bit — no-op
            ClockId::Ref | ClockId::Sys => {}
        }
    }

    // ========================================================================
    // Frequency Query and Tracking
    // ========================================================================

    /// Current frequency of a clock generator in Hz
    #[inline]
    pub fn freq(&self, clock: ClockId) -> u32 {
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

    /// Override the tracked frequency for a clock generator
    ///
    /// Useful when the hardware was configured outside this HAL.
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

    // ========================================================================
    // Frequency Counter (FC0)
    // ========================================================================
    //
    // FC0 counts cycles of any internal clock source against clk_ref.
    // Useful for verifying PLL lock and clock tree configuration.
    //
    // ========================================================================

    /// Measure a clock frequency using the hardware frequency counter
    ///
    /// Blocks until the measurement completes (~1 ms at 12 MHz clk_ref).
    ///
    /// # Returns
    /// Measured frequency in kHz.
    pub fn frequency_count_khz(&self, src: FcSrc) -> u32 {
        // Wait for any in-progress measurement to finish
        while self.clocks.fc0_status.running() {
            core::hint::spin_loop();
        }

        self.clocks
            .fc0_ref_khz
            .write_fc0_ref_khz(self.frequencies.ref_clk / 1000);

        // Measurement window: 2^10 clk_ref cycles ≈ 1 ms at 12 MHz
        self.clocks.fc0_interval.write_fc0_interval(10);

        // No min/max frequency check
        self.clocks.fc0_min_khz.write_fc0_min_khz(0);
        self.clocks.fc0_max_khz.write_fc0_max_khz(0x1fff_ffff);

        // Writing the source register starts the measurement
        self.clocks.fc0_src.write_fc0_src(src as u32);

        while !self.clocks.fc0_status.done() {
            core::hint::spin_loop();
        }

        self.clocks.fc0_result.khz()
    }

    // ========================================================================
    // System Clock Recovery (RESUS)
    // ========================================================================
    //
    // RESUS monitors clk_sys and automatically falls back to clk_ref if
    // clk_sys stops (e.g. PLL loses lock). This prevents the system from
    // hanging silently after a clock failure.
    //
    // ========================================================================

    /// Enable system clock resuscitation
    ///
    /// If `clk_sys` does not toggle for `timeout` `clk_ref` cycles, it is
    /// automatically switched back to `clk_ref`.
    ///
    /// # Arguments
    /// * `timeout` — Inactivity threshold in `clk_ref` cycles (1–255)
    #[inline]
    pub fn enable_resus(&mut self, timeout: u8) {
        self.clocks.clk_sys_resus_ctrl.write_timeout(timeout as u32);
        self.clocks.clk_sys_resus_ctrl.set_enable();
    }

    /// Disable system clock resuscitation
    #[inline]
    pub fn disable_resus(&mut self) {
        self.clocks.clk_sys_resus_ctrl.clr_enable();
    }

    /// Check whether RESUS has triggered since the last [`clear_resus`](Self::clear_resus)
    #[inline]
    pub fn resus_triggered(&self) -> bool {
        self.clocks.clk_sys_resus_status.resussed()
    }

    /// Clear the RESUS status flag
    #[inline]
    pub fn clear_resus(&self) {
        self.clocks.clk_sys_resus_ctrl.set_bits(1 << 16); // CLEAR bit
        self.clocks.clk_sys_resus_ctrl.clr_bits(1 << 16);
    }

    // ========================================================================
    // ███████╗██╗     ███████╗███████╗██████╗     ██╗    ██╗ █████╗ ██╗  ██╗███████╗
    // ██╔════╝██║     ██╔════╝██╔════╝██╔══██╗    ██║    ██║██╔══██╗██║ ██╔╝██╔════╝
    // ███████╗██║     █████╗  █████╗  ██████╔╝    ██║ █╗ ██║███████║█████╔╝ █████╗
    // ╚════██║██║     ██╔══╝  ██╔══╝  ██╔═══╝     ██║███╗██║██╔══██║██╔═██╗ ██╔══╝
    // ███████║███████╗███████╗███████╗██║         ╚███╔███╔╝██║  ██║██║  ██╗███████╗
    // ╚══════╝╚══════╝╚══════╝╚══════╝╚═╝          ╚══╝╚══╝ ╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝
    // Sleep / Wake Clock Gate Control
    // ========================================================================
    //
    // WAKE_EN0/1  — which peripherals receive their clock in wake mode
    // SLEEP_EN0/1 — which peripherals receive their clock in sleep mode
    // ENABLED0/1  — read-only status: which peripherals are currently clocked
    //
    // All 64 bits default to enabled (1) after reset.
    // ========================================================================

    /// Check whether a peripheral is currently receiving its clock
    ///
    /// Reads the hardware `ENABLED0/1` status registers.
    #[inline]
    pub fn is_peripheral_clocked(&self, peripheral: ClkPeripheral) -> bool {
        let (reg_idx, bit) = peripheral.reg_bit();
        let val = if reg_idx == 0 {
            self.clocks.enabled0.read_bits()
        } else {
            self.clocks.enabled1.read_bits()
        };
        (val & bit) != 0
    }

    /// Read raw enabled-status bitmasks
    ///
    /// Returns `(enabled0_bits, enabled1_bits)`.
    #[inline]
    pub fn enabled_mask(&self) -> (u32, u32) {
        (
            self.clocks.enabled0.read_bits(),
            self.clocks.enabled1.read_bits(),
        )
    }

    /// Set or clear a peripheral's wake-mode clock enable
    #[inline]
    pub fn set_wake_enabled(&self, peripheral: ClkPeripheral, enabled: bool) {
        let (reg_idx, bit) = peripheral.reg_bit();
        if reg_idx == 0 {
            if enabled {
                self.clocks.wake_en0.set_bits(bit);
            } else {
                self.clocks.wake_en0.clr_bits(bit);
            }
        } else if enabled {
            self.clocks.wake_en1.set_bits(bit);
        } else {
            self.clocks.wake_en1.clr_bits(bit);
        }
    }

    /// Check whether a peripheral's wake-mode clock enable is set
    #[inline]
    pub fn is_wake_enabled(&self, peripheral: ClkPeripheral) -> bool {
        let (reg_idx, bit) = peripheral.reg_bit();
        let val = if reg_idx == 0 {
            self.clocks.wake_en0.read_bits()
        } else {
            self.clocks.wake_en1.read_bits()
        };
        (val & bit) != 0
    }

    /// Set or clear a peripheral's sleep-mode clock enable
    ///
    /// Disabling unused peripherals during sleep reduces standby current.
    /// All bits default to 1 (enabled) after reset.
    #[inline]
    pub fn set_sleep_enabled(&self, peripheral: ClkPeripheral, enabled: bool) {
        let (reg_idx, bit) = peripheral.reg_bit();
        if reg_idx == 0 {
            if enabled {
                self.clocks.sleep_en0.set_bits(bit);
            } else {
                self.clocks.sleep_en0.clr_bits(bit);
            }
        } else if enabled {
            self.clocks.sleep_en1.set_bits(bit);
        } else {
            self.clocks.sleep_en1.clr_bits(bit);
        }
    }

    /// Check whether a peripheral's sleep-mode clock enable is set
    #[inline]
    pub fn is_sleep_enabled(&self, peripheral: ClkPeripheral) -> bool {
        let (reg_idx, bit) = peripheral.reg_bit();
        let val = if reg_idx == 0 {
            self.clocks.sleep_en0.read_bits()
        } else {
            self.clocks.sleep_en1.read_bits()
        };
        (val & bit) != 0
    }

    /// Raw wake-enable bitmasks — `(wake_en0_bits, wake_en1_bits)`
    #[inline]
    pub fn wake_enable_mask(&self) -> (u32, u32) {
        (
            self.clocks.wake_en0.read_bits(),
            self.clocks.wake_en1.read_bits(),
        )
    }

    /// Raw sleep-enable bitmasks — `(sleep_en0_bits, sleep_en1_bits)`
    #[inline]
    pub fn sleep_enable_mask(&self) -> (u32, u32) {
        (
            self.clocks.sleep_en0.read_bits(),
            self.clocks.sleep_en1.read_bits(),
        )
    }

    /// Disable non-essential peripheral clocks for sleep mode
    ///
    /// Keeps only the peripherals required for reliable wake-up: CLOCKS,
    /// SRAM, ROM, bus fabrics, IO pads, system config, resets, ROSC, XOSC,
    /// POWMAN, TICKS, TIMER0/1, WATCHDOG, XIP.
    ///
    /// Everything else (DMA, PIO, SPI, UART, I2C, USB, ADC, HSTX, SHA256,
    /// TRNG, JTAG, …) is gated off.
    ///
    /// Use [`set_sleep_enabled`](Self::set_sleep_enabled) for fine-grained control.
    pub fn sleep_power_save(&self) {
        for &p in &[
            ClkPeripheral::Adc,
            ClkPeripheral::SysAdc,
            ClkPeripheral::Dma,
            ClkPeripheral::Hstx,
            ClkPeripheral::SysHstx,
            ClkPeripheral::I2c0,
            ClkPeripheral::I2c1,
            ClkPeripheral::Jtag,
            ClkPeripheral::Pio0,
            ClkPeripheral::Pio1,
            ClkPeripheral::Pio2,
            ClkPeripheral::Pwm,
            ClkPeripheral::Sha256,
            ClkPeripheral::Sio,
            ClkPeripheral::RefOtp,
            ClkPeripheral::SysOtp,
            ClkPeripheral::PeriSpi0,
            ClkPeripheral::SysSpi0,
            ClkPeripheral::PeriSpi1,
            ClkPeripheral::SysSpi1,
            ClkPeripheral::PeriUart0,
            ClkPeripheral::SysUart0,
            ClkPeripheral::PeriUart1,
            ClkPeripheral::SysUart1,
            ClkPeripheral::Usb,
            ClkPeripheral::SysUsbCtrl,
            ClkPeripheral::Trng,
        ] {
            self.set_sleep_enabled(p, false);
        }
    }

    // ========================================================================
    // ██╗███╗   ██╗████████╗███████╗██████╗ ██████╗ ██╗   ██╗██████╗ ████████╗
    // ██║████╗  ██║╚══██╔══╝██╔════╝██╔══██╗██╔══██╗██║   ██║██╔══██╗╚══██╔══╝
    // ██║██╔██╗ ██║   ██║   █████╗  ██████╔╝██████╔╝██║   ██║██████╔╝   ██║
    // ██║██║╚██╗██║   ██║   ██╔══╝  ██╔══██╗██╔══██╗██║   ██║██╔═══╝    ██║
    // ██║██║ ╚████║   ██║   ███████╗██║  ██║██║  ██║╚██████╔╝██║        ██║
    // ╚═╝╚═╝  ╚═══╝   ╚═╝   ╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝ ╚═════╝ ╚═╝        ╚═╝
    // CLK_SYS_RESUS Interrupt
    // ========================================================================

    /// Raw (unmasked) CLK_SYS_RESUS interrupt status
    #[inline]
    pub fn is_intr_clk_sys_resus(&self) -> bool {
        self.clocks.intr.clk_sys_resus()
    }

    /// Enable CLK_SYS_RESUS interrupt
    #[inline]
    pub fn enable_intr_clk_sys_resus(&self) {
        self.clocks.inte.set_clk_sys_resus();
    }

    /// Disable CLK_SYS_RESUS interrupt
    #[inline]
    pub fn disable_intr_clk_sys_resus(&self) {
        self.clocks.inte.clr_clk_sys_resus();
    }

    /// Check whether CLK_SYS_RESUS interrupt is enabled
    #[inline]
    pub fn is_intr_clk_sys_resus_enabled(&self) -> bool {
        self.clocks.inte.clk_sys_resus()
    }

    /// Force CLK_SYS_RESUS interrupt (for testing)
    #[inline]
    pub fn force_intr_clk_sys_resus(&self) {
        self.clocks.intf.set_clk_sys_resus();
    }

    /// Clear forced CLK_SYS_RESUS interrupt
    #[inline]
    pub fn unforce_intr_clk_sys_resus(&self) {
        self.clocks.intf.clr_clk_sys_resus();
    }

    /// Masked CLK_SYS_RESUS interrupt status (after INTE and INTF)
    #[inline]
    pub fn is_ints_clk_sys_resus(&self) -> bool {
        self.clocks.ints.clk_sys_resus()
    }

    // ========================================================================
    // ██████╗ ███████╗████████╗     ██████╗ ███████╗██████╗ ██╗   ██╗ ██████╗
    // ██╔══██╗██╔════╝╚══██╔══╝    ██╔══██╗██╔════╝██╔══██╗██║   ██║██╔════╝
    // ██║  ██║█████╗     ██║       ██║  ██║█████╗  ██████╔╝██║   ██║██║  ███╗
    // ██║  ██║██╔══╝     ██║       ██║  ██║██╔══╝  ██╔══██╗██║   ██║██║   ██║
    // ██████╔╝██║        ██║       ██████╔╝███████╗██████╔╝╚██████╔╝╚██████╔╝
    // ╚═════╝ ╚═╝        ╚═╝       ╚═════╝ ╚══════╝╚═════╝  ╚═════╝  ╚═════╝
    // DFT Debug Clock Override
    // ========================================================================
    //
    // Each oscillator output can be replaced by a debug clock for silicon
    // validation. Normal firmware should leave all sources at `DftClkSrc::Null`.
    //
    // ========================================================================

    /// Override XOSC output with a DFT debug clock source
    #[inline]
    pub fn set_dft_xosc_src(&self, src: DftClkSrc) {
        self.clocks.dftclk_xosc_ctrl.write_src(src as u32);
    }

    /// Read the current XOSC DFT debug clock source
    #[inline]
    pub fn dft_xosc_src(&self) -> DftClkSrc {
        match self.clocks.dftclk_xosc_ctrl.src() {
            1 => DftClkSrc::DebugClk,
            2 => DftClkSrc::GpIn,
            _ => DftClkSrc::Null,
        }
    }

    /// Override ROSC output with a DFT debug clock source
    #[inline]
    pub fn set_dft_rosc_src(&self, src: DftClkSrc) {
        self.clocks.dftclk_rosc_ctrl.write_src(src as u32);
    }

    /// Read the current ROSC DFT debug clock source
    #[inline]
    pub fn dft_rosc_src(&self) -> DftClkSrc {
        match self.clocks.dftclk_rosc_ctrl.src() {
            1 => DftClkSrc::DebugClk,
            2 => DftClkSrc::GpIn,
            _ => DftClkSrc::Null,
        }
    }

    /// Override LPOSC output with a DFT debug clock source
    #[inline]
    pub fn set_dft_lposc_src(&self, src: DftClkSrc) {
        self.clocks.dftclk_lposc_ctrl.write_src(src as u32);
    }

    /// Read the current LPOSC DFT debug clock source
    #[inline]
    pub fn dft_lposc_src(&self) -> DftClkSrc {
        match self.clocks.dftclk_lposc_ctrl.src() {
            1 => DftClkSrc::DebugClk,
            2 => DftClkSrc::GpIn,
            _ => DftClkSrc::Null,
        }
    }

    // ========================================================================
    //  ██████╗ ██████╗ ██╗███╗   ██╗
    // ██╔════╝ ██╔══██╗██║████╗  ██║
    // ██║  ███╗██████╔╝██║██╔██╗ ██║
    // ██║   ██║██╔═══╝ ██║██║╚██╗██║
    // ╚██████╔╝██║     ██║██║ ╚████║
    //  ╚═════╝ ╚═╝     ╚═╝╚═╝  ╚═══╝
    // External GPIO Clock Inputs (GPIN0 / GPIN1)
    // ========================================================================
    //
    // GPIN0 = GPIO20, GPIN1 = GPIO22 on RP2350.
    // Configure the GPIO function to FunctionClock before using as a source.
    //
    // ========================================================================

    /// Return `true` if GPIN0 (GPIO20) is supplying a measurable clock
    ///
    /// Uses the frequency counter; may temporarily affect ongoing measurements.
    pub fn is_gpin0_clock_present(&self) -> bool {
        self.frequency_count_khz(FcSrc::ClkSrcGpIn0) > 0
    }

    /// Return `true` if GPIN1 (GPIO22) is supplying a measurable clock
    ///
    /// Uses the frequency counter; may temporarily affect ongoing measurements.
    pub fn is_gpin1_clock_present(&self) -> bool {
        self.frequency_count_khz(FcSrc::ClkSrcGpIn1) > 0
    }

    // ========================================================================
    // ██████╗  ██████╗ ██████╗ ███╗   ███╗ █████╗ ███╗   ██╗████████╗
    // ██╔══██╗██╔═══██╗██╔══██╗████╗ ████║██╔══██╗████╗  ██║╚══██╔══╝
    // ██║  ██║██║   ██║██████╔╝██╔████╔██║███████║██╔██╗ ██║   ██║
    // ██║  ██║██║   ██║██╔══██╗██║╚██╔╝██║██╔══██║██║╚██╗██║   ██║
    // ██████╔╝╚██████╔╝██║  ██║██║ ╚═╝ ██║██║  ██║██║ ╚████║   ██║
    // ╚═════╝  ╚═════╝ ╚═╝  ╚═╝╚═╝     ╚═╝╚═╝  ╚═╝╚═╝  ╚═══╝   ╚═╝
    // Dormant (Ultra-Low-Power) Mode
    // ========================================================================
    //
    // Dormant mode stops all clocks and most oscillators, reducing current to
    // a few µA. The CPU halts until a wake-up interrupt fires (GPIO edge, RTC,
    // etc.).
    //
    // This method automatically:
    //   1. Moves clk_sys back to clk_ref (safe, glitchless)
    //   2. Moves clk_ref to the non-sleeping oscillator
    //   3. Powers down both PLLs
    //   4. Puts the primary oscillator into DORMANT mode — CPU halts here
    //   5. On wake: restores the primary oscillator, PLLs, and clk_sys
    //
    // The "primary oscillator" is selected based on the current clk_ref source:
    //   clk_ref = XOSC  → XOSC sleeps; ROSC keeps clk_ref alive during dormant
    //   clk_ref = ROSC  → ROSC sleeps; XOSC keeps clk_ref alive during dormant
    //
    // **Requirements**:
    //   - A wake-up source (GPIO dormant-wake interrupt) MUST be configured
    //     before calling this method.
    //   - PLL_SYS and PLL_USB configurations must have been stored via
    //     [`ClockBuilder`] or [`set_pll_configs`](Self::set_pll_configs).
    //
    // **Warning**: DORMANT mode breaks the SWD debug connection.
    //
    // ========================================================================

    /// Store the PLL configurations used for dormant-mode restoration
    ///
    /// [`ClockBuilder::build`] calls this automatically. Only needed when
    /// configuring PLLs manually without the builder.
    ///
    /// [`ClockBuilder::build`]: super::builder::ClockBuilder::build
    #[inline]
    pub fn set_pll_configs(&mut self, pll_sys: Option<PllConfig>, pll_usb: Option<PllConfig>) {
        self.pll_sys_config = pll_sys;
        self.pll_usb_config = pll_usb;
    }

    /// Enter dormant (ultra-low-power) mode
    ///
    /// The CPU halts inside this call until the configured wake-up interrupt
    /// fires. All clocks and PLLs are restored before this function returns.
    ///
    /// # Panics
    /// This function does not panic, but incorrect behaviour will result if
    /// no wake-up source is configured — the system will sleep indefinitely.
    pub fn dormant(&mut self) {
        let saved_sys_freq = self.frequencies.sys_clk;

        match self.ref_src {
            // XOSC is the reference — put XOSC to sleep, keep ROSC running
            ClkRefSrc::Xosc | ClkRefSrc::Aux | ClkRefSrc::Lposc => {
                self.configure_sys_from_ref();
                self.configure_ref_src(ClkRefSrc::Rosc, rosc::ROSC_HZ);
                self.power_down_plls();
                // XOSC enters dormant — CPU halts here until wake interrupt
                self.xosc.dormant();
                // === execution resumes after wake ===
                // XOSC.dormant() already waited for XOSC to re-stabilise
                self.configure_ref_src(ClkRefSrc::Xosc, xosc::XOSC_HZ);
                self.restore_plls();
                self.restore_sys_clk(saved_sys_freq);
            }
            // ROSC is the reference — put ROSC to sleep, use XOSC as backup
            ClkRefSrc::Rosc => {
                self.configure_sys_from_ref();
                self.configure_ref_src(ClkRefSrc::Xosc, xosc::XOSC_HZ);
                self.power_down_plls();
                // ROSC enters dormant — CPU halts here until wake interrupt
                // Safety: clk_ref has been moved to XOSC so ROSC is idle
                unsafe { Rosc::steal().dormant() };
                // === execution resumes after wake ===
                self.configure_ref_src(ClkRefSrc::Rosc, rosc::ROSC_HZ);
                self.restore_plls();
                self.restore_sys_clk(saved_sys_freq);
            }
        }
    }

    /// Restore clk_sys using the same PLL source that was active before dormant
    fn restore_sys_clk(&mut self, target_freq: u32) {
        if self.pll_sys_config.is_some() {
            // Standard path: PLL_SYS was configured
            self.configure_sys_aux(ClkSysAuxSrc::PllSys, target_freq, target_freq);
        } else {
            // Low-power path: only PLL_USB is available (48 MHz)
            self.configure_sys_aux(ClkSysAuxSrc::PllUsb, 48_000_000, 48_000_000);
        }
    }

    // -----------------------------------------------------------------------
    // Private dormant helpers
    // -----------------------------------------------------------------------

    /// Power down both PLLs before entering dormant
    fn power_down_plls(&mut self) {
        unsafe {
            let pll_sys = PLL_SYS::steal();
            let pll_usb = PLL_USB::steal();
            pll_sys.pwr.set_pd();
            pll_sys.pwr.set_vcopd();
            pll_sys.pwr.set_postdivpd();
            pll_sys.pwr.set_dsmpd();
            pll_usb.pwr.set_pd();
            pll_usb.pwr.set_vcopd();
            pll_usb.pwr.set_postdivpd();
            pll_usb.pwr.set_dsmpd();
        }
    }

    /// Re-initialise PLLs after waking from dormant mode
    ///
    /// Uses the saved [`PllConfig`] values set during [`ClockBuilder::build`]
    /// (or via [`set_pll_configs`](Self::set_pll_configs)).
    fn restore_plls(&mut self) {
        unsafe {
            if let Some(ref cfg) = self.pll_sys_config {
                <PLL_SYS as Resettable>::init();
                let mut pll = PllSys::init(PLL_SYS::steal());
                pll.configure(cfg.refdiv, cfg.fbdiv, cfg.postdiv1, cfg.postdiv2);
                pll.wait_lock();
            }

            if let Some(ref cfg) = self.pll_usb_config {
                <PLL_USB as Resettable>::init();
                let mut pll = PllUsb::init(PLL_USB::steal());
                pll.configure(cfg.refdiv, cfg.fbdiv, cfg.postdiv1, cfg.postdiv2);
                pll.wait_lock();
            }
        }
    }
}

// ============================================================================
// Internal utility
// ============================================================================

/// Burn a small number of CPU cycles to let a peripheral register propagate.
///
/// Used after clearing an ENABLE bit before switching a clock source, giving
/// the hardware ~N clk_sys cycles to de-assert the enable.
#[inline(always)]
fn cortex_m_delay(cycles: u32) {
    for _ in 0..cycles {
        core::hint::spin_loop();
    }
}
