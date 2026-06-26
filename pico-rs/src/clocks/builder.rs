//! Clock tree builder for RP2350
//!
//! This module provides:
//!
//! - [`ClockResources`] — bundles all PAC peripherals needed to build a clock tree
//! - [`ClockBuilder`]  — fluent API for configuring the entire clock tree with
//!   sensible defaults and compile-time customisation
//!
//! # Quick Start
//!
//! ```ignore
//! use pico_rs::clocks::{ClockBuilder, ClockResources};
//!
//! let dp = pico_rs_pac::Peripherals::take().unwrap();
//!
//! let clocks = ClockBuilder::new().build(ClockResources {
//!     clocks: dp.CLOCKS,
//!     xosc:   dp.XOSC,
//!     pll_sys: dp.PLL_SYS,
//!     pll_usb: dp.PLL_USB,
//! });
//! // clk_sys = 150 MHz, clk_usb = 48 MHz, clk_peri = 150 MHz (from clk_sys)
//! ```

use pico_rs_pac::{CLOCKS, PLL_SYS, PLL_USB, XOSC};

use super::controller::Clocks;
use super::lposc;
use super::pll::{PllConfig, PllSys, PllUsb};
use super::rosc;
use super::types::{
    ClkAdcAuxSrc, ClkHstxAuxSrc, ClkPeriAuxSrc, ClkRefSrc, ClkSysAuxSrc, ClkUsbAuxSrc,
};
use super::xosc::Xosc;
use crate::reset::Resettable;

// ============================================================================
// Clock Resource Bundle
// ============================================================================

/// All PAC peripherals required to initialise the RP2350 clock tree
///
/// Pass this to [`ClockBuilder::build`] to avoid listing four separate
/// arguments each time.
///
/// # Example
/// ```ignore
/// let dp = pico_rs_pac::Peripherals::take().unwrap();
/// let res = ClockResources {
///     clocks:  dp.CLOCKS,
///     xosc:    dp.XOSC,
///     pll_sys: dp.PLL_SYS,
///     pll_usb: dp.PLL_USB,
/// };
/// ```
pub struct ClockResources {
    /// CLOCKS register block
    pub clocks: CLOCKS,
    /// Crystal oscillator (XOSC)
    pub xosc: XOSC,
    /// System PLL
    pub pll_sys: PLL_SYS,
    /// USB PLL
    pub pll_usb: PLL_USB,
}

// ============================================================================
// ███████╗ █████╗ ███████╗██╗   ██╗    ██╗███╗   ██╗██╗████████╗
// ██╔════╝██╔══██╗██╔════╝╚██╗ ██╔╝    ██║████╗  ██║██║╚══██╔══╝
// █████╗  ███████║███████╗ ╚████╔╝     ██║██╔██╗ ██║██║   ██║
// ██╔══╝  ██╔══██║╚════██║  ╚██╔╝      ██║██║╚██╗██║██║   ██║
// ███████╗██║  ██║███████║   ██║       ██║██║ ╚████║██║   ██║
// ╚══════╝╚═╝  ╚═╝╚══════╝   ╚═╝       ╚═╝╚═╝  ╚═══╝╚═╝   ╚═╝
// Clock Builder — flexible clock configuration with sensible defaults
// ============================================================================
//
// Default configuration produced by `ClockBuilder::new().build(...)`:
//
//   XOSC      →  12 MHz  (crystal)
//   clk_ref   →  12 MHz  (from XOSC)
//   clk_sys   → 150 MHz  (PLL_SYS)
//   clk_peri  → 150 MHz  (follows clk_sys)
//   clk_usb   →  48 MHz  (PLL_USB)  ← disabled by default, use .enable_usb()
//   clk_adc   →  48 MHz  (PLL_USB)  ← disabled by default, use .enable_adc()
//   clk_hstx  → disabled by default, use .enable_hstx(...)
//
// ============================================================================

/// Builder for configuring the RP2350 clock system
///
/// Provides a fluent API with sensible defaults. Only customise what you need.
///
/// # Default Configuration
///
/// | Clock    | Frequency | Source      |
/// |----------|-----------|-------------|
/// | XOSC     | 12 MHz    | Crystal     |
/// | clk_ref  | 12 MHz    | XOSC        |
/// | clk_sys  | 150 MHz   | PLL_SYS     |
/// | clk_peri | 150 MHz   | clk_sys     |
/// | clk_usb  | disabled  | —           |
/// | clk_adc  | disabled  | —           |
/// | clk_hstx | disabled  | —           |
///
/// # Example
///
/// ```ignore
/// use pico_rs::clocks::{ClockBuilder, ClockResources, ClkPeriAuxSrc};
///
/// // Default: 150 MHz sys, enable USB and ADC
/// let clocks = ClockBuilder::new()
///     .enable_usb()
///     .enable_adc()
///     .build(ClockResources { clocks, xosc, pll_sys, pll_usb });
///
/// // Custom: 125 MHz sys, stable 48 MHz peri for UART baud rates
/// let clocks = ClockBuilder::new()
///     .sys_freq_mhz(125)
///     .peri_stable_48mhz()
///     .build(ClockResources { clocks, xosc, pll_sys, pll_usb });
/// ```
#[derive(Clone)]
pub struct ClockBuilder {
    /// Crystal frequency in Hz (default: 12 MHz)
    xosc_freq: u32,
    /// Reference clock source (default: XOSC)
    ref_src: ClkRefSrc,
    /// Target system clock frequency in Hz (default: 150 MHz)
    sys_freq: u32,
    /// Peripheral clock source (default: ClkSys — follows clk_sys)
    peri_src: ClkPeriAuxSrc,
    /// Peripheral clock frequency override (only used when peri_src ≠ ClkSys)
    peri_freq: Option<u32>,
    /// Enable and configure the USB clock
    enable_usb: bool,
    /// Enable and configure the ADC clock
    enable_adc: bool,
    /// HSTX clock: `Some((src, freq))` to enable, `None` to leave disabled
    hstx_config: Option<(ClkHstxAuxSrc, u32)>,
}

impl Default for ClockBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ClockBuilder {
    // ========================================================================
    // Constructors / presets
    // ========================================================================

    /// Create a builder with the default configuration
    ///
    /// Produces: 12 MHz XOSC → 150 MHz clk_sys, USB/ADC/HSTX disabled.
    pub const fn new() -> Self {
        Self {
            xosc_freq: 12_000_000,
            ref_src: ClkRefSrc::Xosc,
            sys_freq: 150_000_000,
            peri_src: ClkPeriAuxSrc::ClkSys,
            peri_freq: None,
            enable_usb: false,
            enable_adc: false,
            hstx_config: None,
        }
    }

    /// Preset for low-power operation (48 MHz system clock from PLL_USB)
    ///
    /// PLL_SYS is not initialised, saving ~1 mA. USB / ADC still work because
    /// PLL_USB is always started for the 48 MHz reference.
    pub const fn low_power() -> Self {
        Self {
            xosc_freq: 12_000_000,
            ref_src: ClkRefSrc::Xosc,
            sys_freq: 48_000_000,
            peri_src: ClkPeriAuxSrc::ClkSys,
            peri_freq: None,
            enable_usb: false,
            enable_adc: false,
            hstx_config: None,
        }
    }

    // ========================================================================
    // Crystal / reference clock
    // ========================================================================

    /// Set the crystal oscillator frequency in Hz (default: 12 MHz)
    ///
    /// # Note
    /// The built-in [`PllConfig`] presets assume a 12 MHz crystal.
    /// Supplying a different frequency here requires using a custom PLL
    /// configuration — currently only supported for 12 MHz crystals.
    pub const fn xosc_freq(mut self, freq: u32) -> Self {
        self.xosc_freq = freq;
        self
    }

    /// Set the crystal oscillator frequency in MHz (convenience wrapper)
    pub const fn xosc_freq_mhz(mut self, mhz: u32) -> Self {
        self.xosc_freq = mhz * 1_000_000;
        self
    }

    /// Set the reference clock source (default: [`ClkRefSrc::Xosc`])
    ///
    /// The choice also governs dormant-mode behaviour:
    /// - `Xosc` → XOSC sleeps during dormant, ROSC keeps clk_ref alive
    /// - `Rosc` → ROSC sleeps during dormant, XOSC keeps clk_ref alive
    pub const fn ref_src(mut self, src: ClkRefSrc) -> Self {
        self.ref_src = src;
        self
    }

    // ========================================================================
    // System clock
    // ========================================================================

    /// Set the target system clock frequency in Hz (default: 150 MHz)
    ///
    /// # Supported frequencies
    /// | Value       | Source   |
    /// |-------------|----------|
    /// | 48 MHz      | PLL_USB  |
    /// | 125 MHz     | PLL_SYS  |
    /// | 133 MHz     | PLL_SYS  |
    /// | 150 MHz     | PLL_SYS  |
    ///
    /// Other values will panic during [`build`](Self::build).
    pub const fn sys_freq(mut self, freq: u32) -> Self {
        self.sys_freq = freq;
        self
    }

    /// Set the target system clock frequency in MHz (convenience wrapper)
    ///
    /// ```ignore
    /// ClockBuilder::new().sys_freq_mhz(125)
    /// ```
    pub const fn sys_freq_mhz(mut self, mhz: u32) -> Self {
        self.sys_freq = mhz * 1_000_000;
        self
    }

    // ========================================================================
    // Peripheral clock (CLK_PERI)
    // ========================================================================

    /// Set the peripheral clock source (default: `ClkSys` — follows clk_sys)
    ///
    /// | Source   | Trade-off                                              |
    /// |----------|--------------------------------------------------------|
    /// | `ClkSys` | Simplest; baud rates change if clk_sys frequency changes |
    /// | `PllUsb` | 48 MHz fixed; baud rates stable regardless of clk_sys   |
    /// | `PllSys` | Direct PLL output; independent of clk_sys divider       |
    pub const fn peri_src(mut self, src: ClkPeriAuxSrc) -> Self {
        self.peri_src = src;
        self
    }

    /// Set an explicit peripheral clock frequency
    ///
    /// Required when `peri_src` is not `ClkSys`, so the HAL can track the
    /// correct frequency for baud-rate calculations.
    pub const fn peri_freq(mut self, freq: u32) -> Self {
        self.peri_freq = Some(freq);
        self
    }

    /// Route CLK_PERI from PLL_USB at a stable 48 MHz
    ///
    /// Keeps UART/SPI/I2C baud rates constant even if clk_sys changes.
    pub const fn peri_stable_48mhz(mut self) -> Self {
        self.peri_src = ClkPeriAuxSrc::PllUsb;
        self.peri_freq = Some(48_000_000);
        self
    }

    // ========================================================================
    // USB and ADC clocks
    // ========================================================================

    /// Enable the 48 MHz USB clock (CLK_USB) from PLL_USB
    ///
    /// Required for the USB device/host controller.
    pub const fn enable_usb(mut self) -> Self {
        self.enable_usb = true;
        self
    }

    /// Disable the USB clock (default)
    pub const fn disable_usb(mut self) -> Self {
        self.enable_usb = false;
        self
    }

    /// Enable the 48 MHz ADC clock (CLK_ADC) from PLL_USB
    ///
    /// Required when using the ADC peripheral.
    pub const fn enable_adc(mut self) -> Self {
        self.enable_adc = true;
        self
    }

    /// Disable the ADC clock (default)
    pub const fn disable_adc(mut self) -> Self {
        self.enable_adc = false;
        self
    }

    // ========================================================================
    // HSTX clock
    // ========================================================================

    /// Enable the HSTX clock with an explicit source and frequency
    ///
    /// Use when the HSTX peripheral (DVI, etc.) needs a specific clock.
    pub const fn enable_hstx(mut self, src: ClkHstxAuxSrc, freq: u32) -> Self {
        self.hstx_config = Some((src, freq));
        self
    }

    /// Enable HSTX at the same frequency as clk_sys
    pub const fn enable_hstx_from_sys(mut self) -> Self {
        // freq=0 is resolved to the actual sys_freq during build()
        self.hstx_config = Some((ClkHstxAuxSrc::ClkSys, 0));
        self
    }

    /// Disable the HSTX clock (default)
    pub const fn disable_hstx(mut self) -> Self {
        self.hstx_config = None;
        self
    }

    // ========================================================================
    // Build
    // ========================================================================

    /// Build and apply the clock configuration, consuming the PAC peripherals
    ///
    /// # Initialisation sequence
    ///
    /// ```text
    /// 1. Start XOSC, wait for stable
    /// 2. Configure clk_ref from XOSC (12 MHz)
    /// 3. Switch clk_sys to clk_ref  ← safe intermediate state
    /// 4. Initialise PLL_SYS  (if sys_freq > 48 MHz)
    /// 5. Initialise PLL_USB  (always — needed for USB, ADC, and low-power sys)
    /// 6. Switch clk_sys to final source
    /// 7. Configure clk_peri, clk_usb, clk_adc, clk_hstx as requested
    /// ```
    ///
    /// # Panics
    /// Panics if `sys_freq` is not one of the supported values (48, 125, 133,
    /// 150 MHz).
    pub fn build(self, res: ClockResources) -> Clocks {
        // ====================================================================
        // Step 1 — Start XOSC
        // ====================================================================
        let xosc = Xosc::init(res.xosc, self.xosc_freq);
        xosc.wait_stable();

        let mut clocks = Clocks::new(res.clocks, xosc);

        // ====================================================================
        // Step 2 — Configure clk_ref
        // ====================================================================
        let ref_freq = match self.ref_src {
            ClkRefSrc::Xosc => self.xosc_freq,
            ClkRefSrc::Rosc => rosc::ROSC_HZ,
            ClkRefSrc::Lposc => lposc::LPOSC_HZ,
            ClkRefSrc::Aux => self.xosc_freq, // caller-supplied; assume XOSC frequency
        };
        clocks.configure_ref_src(self.ref_src, ref_freq);

        // ====================================================================
        // Step 3 — Move clk_sys to clk_ref as a safe intermediate
        // ====================================================================
        clocks.configure_sys_from_ref();

        // ====================================================================
        // Steps 4 & 5 — Initialise PLLs
        // ====================================================================
        let use_pll_sys = self.sys_freq > 48_000_000;

        let sys_pll_cfg = if use_pll_sys {
            let cfg = Self::pll_config_for_freq(self.sys_freq);
            <PLL_SYS as Resettable>::init();
            let pll = PllSys::new(res.pll_sys);
            pll.configure(cfg.refdiv, cfg.fbdiv, cfg.postdiv1, cfg.postdiv2);
            pll.wait_lock();
            Some(cfg)
        } else {
            // pll_sys not needed at this system frequency
            let _ = res.pll_sys;
            None
        };

        // PLL_USB always provides the 48 MHz reference
        let usb_pll_cfg = PllConfig::usb_48mhz();
        <PLL_USB as Resettable>::init();
        let pll = PllUsb::new(res.pll_usb);
        pll.configure(
            usb_pll_cfg.refdiv,
            usb_pll_cfg.fbdiv,
            usb_pll_cfg.postdiv1,
            usb_pll_cfg.postdiv2,
        );
        pll.wait_lock();

        // Save configs so dormant mode can restore them without guessing
        clocks.set_pll_configs(sys_pll_cfg, Some(usb_pll_cfg));

        // ====================================================================
        // Step 6 — Switch clk_sys to its final source
        // ====================================================================
        let actual_sys_freq = if use_pll_sys {
            clocks.configure_sys_aux(ClkSysAuxSrc::PllSys, self.sys_freq, self.sys_freq);
            self.sys_freq
        } else {
            // Low-power path: 48 MHz from PLL_USB
            clocks.configure_sys_aux(ClkSysAuxSrc::PllUsb, 48_000_000, 48_000_000);
            48_000_000
        };

        // ====================================================================
        // Step 7 — Peripheral clocks
        // ====================================================================

        // CLK_PERI
        let peri_freq = match self.peri_src {
            ClkPeriAuxSrc::ClkSys => actual_sys_freq,
            ClkPeriAuxSrc::PllUsb => 48_000_000,
            _ => self.peri_freq.unwrap_or(actual_sys_freq),
        };
        clocks.configure_peri(self.peri_src, peri_freq);

        // CLK_USB
        if self.enable_usb {
            clocks.configure_usb(ClkUsbAuxSrc::PllUsb, 48_000_000, 48_000_000);
        }

        // CLK_ADC
        if self.enable_adc {
            clocks.configure_adc(ClkAdcAuxSrc::PllUsb, 48_000_000, 48_000_000);
        }

        // CLK_HSTX
        if let Some((src, freq)) = self.hstx_config {
            let actual = if freq == 0 { actual_sys_freq } else { freq };
            clocks.configure_hstx(src, actual, actual);
        }

        clocks
    }

    // -----------------------------------------------------------------------
    // Private helpers
    // -----------------------------------------------------------------------

    /// Select the appropriate PLL_SYS configuration for a target frequency
    ///
    /// # Panics
    /// Panics with a descriptive message for unsupported frequencies.
    fn pll_config_for_freq(freq: u32) -> PllConfig {
        match freq {
            125_000_000 => PllConfig::sys_125mhz(),
            133_000_000 => PllConfig::sys_133mhz(),
            150_000_000 => PllConfig::sys_150mhz(),
            _ => panic!(
                "pico-rs: unsupported sys_freq {} Hz — supported values: 125, 133, 150 MHz",
                freq
            ),
        }
    }
}
