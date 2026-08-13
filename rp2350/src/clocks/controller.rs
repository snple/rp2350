//! Core clock controller for RP2350
//!
//! This module provides the [`Clocks`] struct — the central object for
//! configuring and querying all clocks in the RP2350 clock tree.
//!
//! # Initialization
//!
//! Use [`ClockBuilder`] for typical initialization. Constructing `Clocks`
//! directly via [`Clocks::init`] is the lower-level path for custom setups.
//!
//! [`ClockBuilder`]: super::builder::ClockBuilder

use rp2350_pac::{PLL_SYS, PLL_USB};

use super::gate::ClkPeripheral;
use super::lposc;
use super::pll::{PllConfig, PllSys, PllUsb};
use super::rosc::{self, Rosc};
use super::types::{
    ClkAdcAuxSrc, ClkGpOutAuxSrc, ClkHstxAuxSrc, ClkPeriAuxSrc, ClkRefAuxSrc, ClkRefSrc,
    ClkSysAuxSrc, ClkUsbAuxSrc, ClockId, DftClkSrc, FcSrc, GpOutId,
};
use super::xosc::{self, Xosc};
use crate::reset::Resettable;

/// Number of GPIO clock outputs.
const GPOUT_COUNT: usize = 4;

/// Bind the CTRL/DIV register pair for a given GPOUT id.
///
/// The four GPOUT register types are structurally identical but generated as
/// distinct types, so a macro is the simplest way to share the configuration
/// code between all four.
macro_rules! each_gpout {
    ($clocks:expr, $id:expr, |$ctrl:ident, $div:ident| $body:expr) => {
        match $id {
            GpOutId::GpOut0 => {
                let $ctrl = &$clocks.clk_gpout0_ctrl;
                let $div = &$clocks.clk_gpout0_div;
                $body
            }
            GpOutId::GpOut1 => {
                let $ctrl = &$clocks.clk_gpout1_ctrl;
                let $div = &$clocks.clk_gpout1_div;
                $body
            }
            GpOutId::GpOut2 => {
                let $ctrl = &$clocks.clk_gpout2_ctrl;
                let $div = &$clocks.clk_gpout2_div;
                $body
            }
            GpOutId::GpOut3 => {
                let $ctrl = &$clocks.clk_gpout3_ctrl;
                let $div = &$clocks.clk_gpout3_div;
                $body
            }
        }
    };
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
    pub(super) clocks: rp2350_pac::CLOCKS,
    pub(super) xosc: Xosc,
    /// Tracked output frequencies (Hz), indexed by [`ClockId`].
    pub(super) freqs: [u32; ClockId::COUNT],
    /// Tracked GPOUT output frequencies (Hz), indexed by [`GpOutId`].
    pub(super) gpout_freqs: [u32; GPOUT_COUNT],
    /// Frequency of the source feeding clk_ref, before the ref divider.
    pub(super) ref_src_freq: u32,
    /// Current reference clock source — drives dormant mode logic
    pub(super) ref_src: ClkRefSrc,
    /// Auxiliary source last selected for clk_ref (used to restore after dormant)
    pub(super) ref_aux: ClkRefAuxSrc,
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
    pub fn init(clocks: rp2350_pac::CLOCKS, xosc: Xosc) -> Self {
        Self {
            clocks,
            xosc,
            freqs: [0; ClockId::COUNT],
            gpout_freqs: [0; GPOUT_COUNT],
            ref_src_freq: 0,
            ref_src: ClkRefSrc::Rosc, // hardware default at boot
            ref_aux: ClkRefAuxSrc::PllUsb,
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
    pub unsafe fn regs(&self) -> &rp2350_pac::clocks::RegisterBlock {
        &self.clocks
    }

    /// Consume `Clocks` and release the underlying CLOCKS peripheral
    #[inline]
    pub fn release(self) -> rp2350_pac::CLOCKS {
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
    ///
    /// # Panics
    /// Panics (debug builds) if `src` is [`ClkRefSrc::Aux`]; auxiliary sources
    /// must be configured via [`configure_ref_aux`](Self::configure_ref_aux),
    /// which also selects the auxiliary mux.
    #[inline]
    pub fn configure_ref_src(&mut self, src: ClkRefSrc, src_freq: u32) {
        debug_assert_ne!(
            src,
            ClkRefSrc::Aux,
            "use Clocks::configure_ref_aux for auxiliary clk_ref sources"
        );
        self.clocks.clk_ref_ctrl.write_src(src as u32);
        crate::spin::spin_wait("clk_ref source selected", || {
            self.clocks.clk_ref_selected.clk_ref_selected() & (1 << src as u32) != 0
        });
        self.ref_src = src;
        self.ref_src_freq = src_freq;
        self.set_freq(ClockId::Ref, apply_ref_div(&self.clocks, src_freq));
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
        crate::spin::spin_wait("clk_ref switched to ROSC", || {
            self.clocks.clk_ref_selected.clk_ref_selected() & 1 != 0
        });

        // Step 2: set auxiliary source and switch the main mux to AUX
        self.clocks.clk_ref_ctrl.write_auxsrc(auxsrc as u32);
        self.clocks.clk_ref_ctrl.write_src(ClkRefSrc::Aux as u32);
        crate::spin::spin_wait("clk_ref switched to AUX", || {
            self.clocks.clk_ref_selected.clk_ref_selected() & (1 << ClkRefSrc::Aux as u32) != 0
        });

        self.ref_src = ClkRefSrc::Aux;
        self.ref_aux = auxsrc;
        self.ref_src_freq = src_freq;
        self.set_freq(ClockId::Ref, apply_ref_div(&self.clocks, src_freq));
    }

    /// Set the reference clock integer divider
    ///
    /// # Arguments
    /// * `int_div` — Integer divisor (1–255; writing 0 divides by 256)
    ///
    /// # Note
    /// This also updates the tracked `ref_clk` frequency. Call before any
    /// dependent clocks are configured.
    #[inline]
    pub fn set_ref_div(&mut self, int_div: u8) {
        self.clocks.clk_ref_div.write_int(int_div as u32);
        self.set_freq(ClockId::Ref, apply_ref_div(&self.clocks, self.ref_src_freq));
    }

    /// Current reference clock frequency in Hz
    #[inline]
    pub fn ref_freq(&self) -> u32 {
        self.freq(ClockId::Ref)
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
        crate::spin::spin_wait("clk_sys switched to ref", || {
            self.clocks.clk_sys_selected.clk_sys_selected() & 1 != 0
        });
        self.set_freq(ClockId::Sys, self.ref_freq());
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
        // 16.16 fixed-point divider, clamped to a minimum of 1.0.
        let div = div_16_16(src_freq, target_freq);

        // If the new divisor is larger, set it first while still on clk_ref to
        // avoid a momentary over-frequency when the faster source is selected.
        let current_div = (self.clocks.clk_sys_div.int() << 16) | self.clocks.clk_sys_div.frac();
        if div > current_div {
            self.clocks.clk_sys_div.write_int((div >> 16) & 0xffff);
            self.clocks.clk_sys_div.write_frac(div & 0xffff);
        }

        // Switch away from AUX first to avoid glitches while changing aux mux
        self.clocks.clk_sys_ctrl.clr_src();
        crate::spin::spin_wait("clk_sys switched to ref", || {
            self.clocks.clk_sys_selected.clk_sys_selected() & 1 != 0
        });

        // Select auxiliary source then switch the glitchless mux to AUX
        self.clocks.clk_sys_ctrl.write_auxsrc(auxsrc as u32);
        self.clocks.clk_sys_ctrl.set_src();
        crate::spin::spin_wait("clk_sys switched to AUX", || {
            self.clocks.clk_sys_selected.clk_sys_selected() & 2 != 0
        });

        // Apply the final divider now that the source is stable
        self.clocks.clk_sys_div.write_int((div >> 16) & 0xffff);
        self.clocks.clk_sys_div.write_frac(div & 0xffff);

        self.set_freq(ClockId::Sys, actual_freq_16_16(src_freq, div));
    }

    /// Set the system clock 16.16 fractional divider directly
    ///
    /// Raw escape hatch: the tracked `clk_sys` frequency is **not** updated.
    /// Call [`set_freq`](Self::set_freq) afterwards if you use this.
    #[inline]
    pub fn set_sys_div(&mut self, int_div: u16, frac_div: u16) {
        self.clocks.clk_sys_div.write_int(int_div as u32);
        self.clocks.clk_sys_div.write_frac(frac_div as u32);
    }

    /// Current system clock frequency in Hz
    #[inline]
    pub fn sys_freq(&self) -> u32 {
        self.freq(ClockId::Sys)
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
        propagate_disable();
        self.clocks.clk_peri_ctrl.write_auxsrc(auxsrc as u32);
        self.clocks.clk_peri_ctrl.set_enable();
        self.set_freq(ClockId::Peri, src_freq);
    }

    /// Current peripheral clock frequency in Hz
    #[inline]
    pub fn peri_freq(&self) -> u32 {
        self.freq(ClockId::Peri)
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
        propagate_disable();
        self.clocks.clk_usb_ctrl.write_auxsrc(auxsrc as u32);
        let div = src_freq.div_ceil(target_freq).max(1);
        self.clocks.clk_usb_div.write_int(div);
        self.clocks.clk_usb_ctrl.set_enable();
        self.set_freq(ClockId::Usb, src_freq / div);
    }

    /// Current USB clock frequency in Hz
    #[inline]
    pub fn usb_freq(&self) -> u32 {
        self.freq(ClockId::Usb)
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
        propagate_disable();
        self.clocks.clk_adc_ctrl.write_auxsrc(auxsrc as u32);
        let div = src_freq.div_ceil(target_freq).max(1);
        self.clocks.clk_adc_div.write_int(div);
        self.clocks.clk_adc_ctrl.set_enable();
        self.set_freq(ClockId::Adc, src_freq / div);
    }

    /// Current ADC clock frequency in Hz
    #[inline]
    pub fn adc_freq(&self) -> u32 {
        self.freq(ClockId::Adc)
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
        propagate_disable();
        self.clocks.clk_hstx_ctrl.write_auxsrc(auxsrc as u32);
        // CLK_HSTX has an integer-only divider (no fractional bits)
        let div = if src_freq > target_freq {
            src_freq.div_ceil(target_freq)
        } else {
            1
        };
        let div = div.max(1);
        self.clocks.clk_hstx_div.write_int(div);
        self.clocks.clk_hstx_ctrl.set_enable();
        self.set_freq(ClockId::Hstx, src_freq / div);
    }

    /// Current HSTX clock frequency in Hz
    #[inline]
    pub fn hstx_freq(&self) -> u32 {
        self.freq(ClockId::Hstx)
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
        let div = div_16_16(src_freq, target_freq);
        let int_part = (div >> 16) & 0xffff;
        let frac_part = div & 0xffff;
        let aux = auxsrc as u32;

        each_gpout!(self.clocks, id, |ctrl, divreg| {
            ctrl.clr_enable();
            ctrl.write_auxsrc(aux);
            divreg.write_int(int_part);
            divreg.write_frac(frac_part);
            ctrl.set_enable();
        });

        self.gpout_freqs[id.index()] = actual_freq_16_16(src_freq, div);
    }

    /// Disable a GPIO clock output
    #[inline]
    pub fn disable_gpout(&mut self, id: GpOutId) {
        each_gpout!(self.clocks, id, |ctrl, _div| ctrl.clr_enable());
        self.gpout_freqs[id.index()] = 0;
    }

    /// Current frequency of a GPIO clock output in Hz
    #[inline]
    pub fn gpout_freq(&self, id: GpOutId) -> u32 {
        self.gpout_freqs[id.index()]
    }

    // ========================================================================
    // Frequency Query and Tracking
    // ========================================================================

    /// Current frequency of a core clock generator in Hz
    #[inline]
    pub fn freq(&self, clock: ClockId) -> u32 {
        self.freqs[clock.index()]
    }

    /// Override the tracked frequency for a core clock generator
    ///
    /// Useful when the hardware was configured outside this HAL.
    #[inline]
    pub fn set_freq(&mut self, clock: ClockId, freq: u32) {
        self.freqs[clock.index()] = freq;
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
        // Wait for any in-progress measurement to finish (bounded)
        crate::spin::spin_wait("frequency counter idle", || {
            !self.clocks.fc0_status.running()
        });

        self.clocks
            .fc0_ref_khz
            .write_fc0_ref_khz(self.ref_freq() / 1000);

        // Measurement window: 2^10 clk_ref cycles ≈ 1 ms at 12 MHz
        self.clocks.fc0_interval.write_fc0_interval(10);

        // No min/max frequency check
        self.clocks.fc0_min_khz.write_fc0_min_khz(0);
        self.clocks.fc0_max_khz.write_fc0_max_khz(0x1fff_ffff);

        // Writing the source register starts the measurement
        self.clocks.fc0_src.write_fc0_src(src as u32);

        crate::spin::spin_wait("frequency counter done", || self.clocks.fc0_status.done());

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
        let saved_sys_freq = self.sys_freq();
        let saved_ref = self.ref_src;
        let saved_ref_freq = self.ref_src_freq;

        // Move clk_sys to clk_ref, then move clk_ref onto the oscillator that
        // stays awake during dormant so the primary oscillator can sleep.
        match saved_ref {
            // clk_ref = ROSC → ROSC sleeps; XOSC keeps clk_ref alive.
            ClkRefSrc::Rosc => {
                self.configure_sys_from_ref();
                self.configure_ref_src(ClkRefSrc::Xosc, xosc::XOSC_HZ);
                self.power_down_plls();
                // Safety: clk_ref has been moved to XOSC so ROSC is idle.
                unsafe { Rosc::steal().dormant() };
            }
            // clk_ref = XOSC/LPOSC/AUX → XOSC sleeps; ROSC keeps clk_ref alive.
            _ => {
                self.configure_sys_from_ref();
                self.configure_ref_src(ClkRefSrc::Rosc, rosc::ROSC_HZ);
                self.power_down_plls();
                // XOSC enters dormant — CPU halts here until the wake interrupt.
                self.xosc.dormant();
            }
        }

        // Execution resumes after the wake interrupt. Restore PLLs first (an
        // AUX clk_ref may depend on them), then clk_ref, then clk_sys.
        self.restore_plls();
        match saved_ref {
            ClkRefSrc::Rosc => self.configure_ref_src(ClkRefSrc::Rosc, rosc::ROSC_HZ),
            ClkRefSrc::Xosc => self.configure_ref_src(ClkRefSrc::Xosc, xosc::XOSC_HZ),
            ClkRefSrc::Lposc => self.configure_ref_src(ClkRefSrc::Lposc, lposc::LPOSC_HZ),
            ClkRefSrc::Aux => self.configure_ref_aux(self.ref_aux, saved_ref_freq),
        }
        self.restore_sys_clk(saved_sys_freq);
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
// Internal utilities
// ============================================================================

/// Apply the hardware clk_ref divider to a source frequency.
fn apply_ref_div(clocks: &rp2350_pac::CLOCKS, src_freq: u32) -> u32 {
    let int = clocks.clk_ref_div.int();
    // A value of 0 divides by 256 per the datasheet.
    let div = if int == 0 { 256 } else { int };
    src_freq / div
}

/// Compute the 16.16 fixed-point divisor needed to reach `target` from `src`,
/// clamped to a minimum of 1.0.
#[inline]
fn div_16_16(src: u32, target: u32) -> u32 {
    if src > target {
        ((src as u64 * 0x1_0000) / target as u64) as u32
    } else {
        0x1_0000
    }
}

/// Actual frequency produced by a 16.16 divisor applied to a source frequency.
#[inline]
fn actual_freq_16_16(src: u32, div: u32) -> u32 {
    ((src as u64 * 0x1_0000) / div as u64) as u32
}

/// Burn a few CPU cycles so a CLK_*_CTRL.ENABLE de-assertion can propagate
/// before the source is changed.
#[inline(always)]
fn propagate_disable() {
    for _ in 0..4 {
        core::hint::spin_loop();
    }
}
