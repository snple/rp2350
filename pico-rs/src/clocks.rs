//! Clock Management HAL for RP2350
//!
//! This module provides a safe interface for configuring the RP2350 clock system.
//! The clock system provides clocks for various peripherals from different sources
//! including crystal oscillators, PLLs, and internal oscillators.
//!
//! # Clock Tree Architecture
//!
//! The RP2350 clock tree has a hierarchical structure with clock sources at the top,
//! feeding into configurable clock generators that supply various peripherals.
//!
//! ```text
//! ╔═══════════════════════════════════════════════════════════════════════════════╗
//! ║                         CLOCK SOURCES (Layer 0)                               ║
//! ╠═══════════════════════════════════════════════════════════════════════════════╣
//! ║  ┌────────┐  ┌────────┐  ┌────────┐  ┌─────────┐  ┌─────────┐  ┌──────────┐  ║
//! ║  │  XOSC  │  │  ROSC  │  │ LPOSC  │  │ PLL_SYS │  │ PLL_USB │  │ GPIN0/1  │  ║
//! ║  │ 12 MHz │  │ ~6 MHz │  │ 32 kHz │  │ 125-150 │  │  48 MHz │  │ External │  ║
//! ║  └───┬────┘  └───┬────┘  └───┬────┘  └────┬────┘  └────┬────┘  └────┬─────┘  ║
//! ║      │           │           │            │            │            │        ║
//! ╠══════╪═══════════╪═══════════╪════════════╪════════════╪════════════╪════════╣
//! ║      │           │           │            │            │            │        ║
//! ║      ▼           ▼           ▼            │            ▼            │        ║
//! ║  ┌─────────────────────────────────┐      │     ┌──────────────┐    │        ║
//! ║  │         CLK_REF MUX             │◄─────┼─────│   (auxsrc)   │◄───┤        ║
//! ║  │  src: XOSC|ROSC|LPOSC|AUX       │      │     └──────────────┘    │        ║
//! ║  └───────────────┬─────────────────┘      │                         │        ║
//! ║                  │ ÷DIV (1-255)           │                         │        ║
//! ║                  ▼                        │                         │        ║
//! ║           ┌──────────┐                    │                         │        ║
//! ║           │  clk_ref │ ─────────────────────────────────────────────────►    ║
//! ║           │ (stable) │                    │                         │        ║
//! ║           └────┬─────┘                    │                         │        ║
//! ║                │                          │                         │        ║
//! ╠════════════════╪══════════════════════════╪═════════════════════════╪════════╣
//! ║    SYSTEM      │                          │                         │        ║
//! ║    CLOCK       ▼                          ▼                         ▼        ║
//! ║  (Layer 1)  ┌─────────────────────────────────────────────────────────┐      ║
//! ║             │                   CLK_SYS MUX                           │      ║
//! ║             │  src: CLK_REF (glitchless) | AUX                        │      ║
//! ║             │  auxsrc: PLL_SYS|PLL_USB|ROSC|XOSC|GPIN0|GPIN1          │      ║
//! ║             └───────────────────────┬─────────────────────────────────┘      ║
//! ║                                     │ ÷DIV (16.16 fixed-point)               ║
//! ║                                     ▼                                        ║
//! ║                              ┌──────────────┐                                ║
//! ║                              │   clk_sys    │ ──────► CPU, Cortex-M33        ║
//! ║                              │  (up to 150  │ ──────► AHB Bus                ║
//! ║                              │     MHz)     │ ──────► APB Bus                ║
//! ║                              └──────┬───────┘                                ║
//! ║                                     │                                        ║
//! ╠═════════════════════════════════════╪════════════════════════════════════════╣
//! ║    PERIPHERAL CLOCKS (Layer 2)      │                                        ║
//! ║                                     ▼                                        ║
//! ║  ┌──────────────────────────────────────────────────────────────────────┐    ║
//! ║  │                                                                      │    ║
//! ║  │  ┌─────────────┐   ┌─────────────┐   ┌─────────────┐   ┌──────────┐  │    ║
//! ║  │  │  CLK_PERI   │   │  CLK_USB    │   │  CLK_ADC    │   │ CLK_HSTX │  │    ║
//! ║  │  │  MUX        │   │  MUX        │   │  MUX        │   │ MUX      │  │    ║
//! ║  │  │ ────────    │   │ ────────    │   │ ────────    │   │ ──────── │  │    ║
//! ║  │  │ clk_sys     │   │ PLL_USB ◄── │   │ PLL_USB ◄── │   │ clk_sys  │  │    ║
//! ║  │  │ PLL_SYS     │   │ PLL_SYS     │   │ PLL_SYS     │   │ PLL_SYS  │  │    ║
//! ║  │  │ PLL_USB     │   │ ROSC/XOSC   │   │ ROSC/XOSC   │   │ PLL_USB  │  │    ║
//! ║  │  │ ROSC/XOSC   │   │ GPIN0/1     │   │ GPIN0/1     │   │ GPIN0/1  │  │    ║
//! ║  │  │ GPIN0/1     │   └──────┬──────┘   └──────┬──────┘   └────┬─────┘  │    ║
//! ║  │  └──────┬──────┘          │                 │               │        │    ║
//! ║  │         │                 │ ÷DIV            │ ÷DIV          │ ÷DIV   │    ║
//! ║  │         ▼                 ▼                 ▼               ▼        │    ║
//! ║  │    ┌─────────┐      ┌──────────┐      ┌──────────┐    ┌──────────┐   │    ║
//! ║  │    │clk_peri │      │ clk_usb  │      │ clk_adc  │    │ clk_hstx │   │    ║
//! ║  │    │         │      │ (48 MHz) │      │ (48 MHz) │    │          │   │    ║
//! ║  │    └────┬────┘      └────┬─────┘      └────┬─────┘    └────┬─────┘   │    ║
//! ║  │         │                │                 │               │         │    ║
//! ║  │         ▼                ▼                 ▼               ▼         │    ║
//! ║  │    UART, SPI,       USB Device/       ADC Module       DVI/HSTX     │    ║
//! ║  │    I2C, etc.        Host                                            │    ║
//! ║  └──────────────────────────────────────────────────────────────────────┘    ║
//! ║                                                                              ║
//! ╠══════════════════════════════════════════════════════════════════════════════╣
//! ║    GPIO CLOCK OUTPUTS (Optional)                                             ║
//! ║  ┌──────────────────────────────────────────────────────────────────────┐    ║
//! ║  │  CLK_GPOUT0-3: Can output any internal clock to GPIO pins            │    ║
//! ║  │  Sources: All clocks + PLL outputs + oscillators                     │    ║
//! ║  │  Features: 16.16 fractional divider for precise frequencies          │    ║
//! ║  └──────────────────────────────────────────────────────────────────────┘    ║
//! ╚══════════════════════════════════════════════════════════════════════════════╝
//! ```
//!
//! # Clock Configuration Flow
//!
//! The recommended initialization sequence:
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────────┐
//! │ 1. Boot: ROSC running (~6 MHz)                                  │
//! │         ↓                                                       │
//! │ 2. Start XOSC (12 MHz external crystal)                         │
//! │         ↓ wait for stable                                       │
//! │ 3. Switch clk_ref to XOSC                                       │
//! │         ↓                                                       │
//! │ 4. Switch clk_sys to clk_ref (safe intermediate state)          │
//! │         ↓                                                       │
//! │ 5. Configure PLL_SYS (e.g., 150 MHz) and PLL_USB (48 MHz)       │
//! │         ↓ wait for lock                                         │
//! │ 6. Switch clk_sys to PLL_SYS                                    │
//! │         ↓                                                       │
//! │ 7. Configure peripheral clocks (clk_peri, clk_usb, clk_adc...)  │
//! └─────────────────────────────────────────────────────────────────┘
//! ```
//!
//! # Important Constraints
//!
//! | Clock     | Requirement                  | Typical Source |
//! |-----------|------------------------------|----------------|
//! | clk_usb   | Must be exactly 48 MHz       | PLL_USB        |
//! | clk_adc   | Must be exactly 48 MHz       | PLL_USB        |
//! | clk_ref   | Should be stable (use XOSC)  | XOSC           |
//! | clk_sys   | Max 150 MHz                  | PLL_SYS        |
//!
//! # Example
//!
//! ```ignore
//! use pico_rs::clocks::ClockBuilder;
//!
//! // Easy: Use ClockBuilder with defaults (recommended)
//! let clocks = ClockBuilder::new().build();
//! // clk_sys = 150 MHz, clk_usb = 48 MHz, clk_peri = 150 MHz
//!
//! // Customize system frequency:
//! let clocks = ClockBuilder::new().sys_freq_mhz(125).build();
//! ```

#![allow(clippy::identity_op)]

use pico_rs_pac::CLOCKS;

// ============================================================================
// Clock Tree Layer 0: Clock Sources
// ============================================================================
// These are the fundamental clock sources in the system:
// - XOSC: External crystal oscillator (typically 12 MHz, high stability)
// - ROSC: Internal ring oscillator (~6 MHz, low power, variable)
// - LPOSC: Low-power oscillator (~32 kHz, for sleep modes)
// - PLL_SYS: System PLL (generates high-speed clocks up to 150 MHz)
// - PLL_USB: USB PLL (generates 48 MHz for USB)
// - GPIN0/1: External clock inputs via GPIO
// ============================================================================

/// Number of configurable clocks in the system
pub const CLK_COUNT: usize = 10;

/// Clock identifiers for all configurable clocks in the system
///
/// The clocks are organized by their position in the clock tree:
/// - Layer 0: Clock sources (XOSC, ROSC, PLLs) - not in this enum
/// - Layer 1: Reference and System clocks ([`ClockId::Ref`], [`ClockId::Sys`])
/// - Layer 2: Peripheral clocks ([`ClockId::Peri`], [`ClockId::Usb`], [`ClockId::Adc`], [`ClockId::Hstx`])
/// - Output: GPIO clock outputs ([`ClockId::GpOut0`] - [`ClockId::GpOut3`])
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(usize)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClockId {
    // === GPIO Clock Outputs (optional, directly from any source) ===
    /// General purpose clock output 0 (directly from any source)
    GpOut0 = 0,
    /// General purpose clock output 1 (directly from any source)
    GpOut1 = 1,
    /// General purpose clock output 2 (directly from any source)
    GpOut2 = 2,
    /// General purpose clock output 3 (directly from any source)
    GpOut3 = 3,

    // === Layer 1: Core System Clocks ===
    /// Reference clock - stable clock for system clock mux and watchdog
    /// Typical source: XOSC (12 MHz)
    Ref = 4,
    /// System clock - main clock for CPU and most peripherals
    /// Typical source: PLL_SYS (125-150 MHz)
    Sys = 5,

    // === Layer 2: Peripheral Clocks ===
    /// Peripheral clock - for UART, SPI, I2C, etc.
    /// Typical source: clk_sys or PLL_SYS
    Peri = 6,
    /// High-speed TX clock - for DVI/HSTX peripheral
    /// Typical source: PLL_SYS
    Hstx = 7,
    /// USB clock - MUST be exactly 48 MHz for USB operation
    /// Typical source: PLL_USB
    Usb = 8,
    /// ADC clock - MUST be exactly 48 MHz for correct ADC operation
    /// Typical source: PLL_USB
    Adc = 9,
}

// ============================================================================
// Clock Tree Layer 1: Reference Clock (CLK_REF) Sources
// ============================================================================
// CLK_REF is the first configurable clock in the tree. It provides a stable
// reference for the system clock mux and watchdog timer.
//
//  XOSC ────┐
//  ROSC ────┼──► [CLK_REF MUX] ──► ÷DIV ──► clk_ref
//  LPOSC ───┤         ▲
//  AUX ─────┘         │
//                     └── PLL_USB, GPIN0/1
// ============================================================================

/// Reference clock (CLK_REF) primary source selection
///
/// These are the glitchless source options for the reference clock.
/// For auxiliary sources (PLL_USB, GPIN), use [`ClkRefAuxSrc`] with [`ClkRefSrc::Aux`].
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkRefSrc {
    /// Ring oscillator (ROSC) - ~6 MHz, variable frequency
    Rosc = 0,
    /// Auxiliary source - see [`ClkRefAuxSrc`] for options
    Aux = 1,
    /// Crystal oscillator (XOSC) - 12 MHz, high stability (recommended)
    Xosc = 2,
    /// Low-power oscillator (LPOSC) - ~32 kHz
    Lposc = 3,
}

/// Reference clock (CLK_REF) auxiliary source selection
///
/// Used when [`ClkRefSrc::Aux`] is selected as the primary source.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkRefAuxSrc {
    /// PLL USB output (48 MHz)
    PllUsb = 0,
    /// External clock input via GPIO (GPIN0)
    GpIn0 = 1,
    /// External clock input via GPIO (GPIN1)
    GpIn1 = 2,
    /// PLL USB primary reference (OPCG)
    PllUsbPrimaryRefOpcg = 3,
}

// ============================================================================
// Clock Tree Layer 1: System Clock (CLK_SYS) Sources
// ============================================================================
// CLK_SYS is the main clock for the CPU and most peripherals. It can be
// sourced from clk_ref (glitchless) or from auxiliary sources via a mux.
//
//  clk_ref ─────┐
//               ├──► [CLK_SYS MUX] ──► ÷DIV ──► clk_sys ──► CPU, AHB, APB
//  AUX ─────────┘         ▲
//                         │
//                         └── PLL_SYS, PLL_USB, ROSC, XOSC, GPIN0/1
// ============================================================================

/// System clock (CLK_SYS) primary source selection
///
/// The system clock has glitchless switching between clk_ref and auxiliary sources.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkSysSrc {
    /// Reference clock (clk_ref) - safe fallback, glitchless switching
    ClkRef = 0,
    /// Auxiliary source - see [`ClkSysAuxSrc`] for options
    Aux = 1,
}

/// System clock (CLK_SYS) auxiliary source selection
///
/// Used when [`ClkSysSrc::Aux`] is selected. PLL_SYS is recommended for
/// high-performance operation.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkSysAuxSrc {
    /// PLL System output (125-150 MHz) - recommended for high performance
    PllSys = 0,
    /// PLL USB output (48 MHz) - for low power operation
    PllUsb = 1,
    /// Ring oscillator (ROSC) - ~6 MHz, variable
    Rosc = 2,
    /// Crystal oscillator (XOSC) - 12 MHz, stable
    Xosc = 3,
    /// External clock input via GPIO (GPIN0)
    GpIn0 = 4,
    /// External clock input via GPIO (GPIN1)
    GpIn1 = 5,
}

// ============================================================================
// Clock Tree Layer 2: Peripheral Clocks
// ============================================================================
// These clocks derive from Layer 1 clocks or directly from PLLs/oscillators.
// Each has specific requirements:
//   - CLK_PERI: Flexible, typically clk_sys for simplicity
//   - CLK_USB:  MUST be 48 MHz for USB operation
//   - CLK_ADC:  MUST be 48 MHz for correct ADC sampling
//   - CLK_HSTX: High-speed clock for DVI/HSTX
// ============================================================================

/// Peripheral clock (CLK_PERI) auxiliary source selection
///
/// CLK_PERI supplies UART, SPI, I2C and other peripherals.
/// Using clk_sys is simplest; using a fixed source like PLL_USB ensures
/// peripheral timing doesn't change when adjusting clk_sys.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkPeriAuxSrc {
    /// System clock (clk_sys) - simplest, but baud rates change with clk_sys
    ClkSys = 0,
    /// PLL System output - independent of clk_sys changes
    PllSys = 1,
    /// PLL USB output (48 MHz) - stable reference
    PllUsb = 2,
    /// Ring oscillator (ROSC)
    Rosc = 3,
    /// Crystal oscillator (XOSC)
    Xosc = 4,
    /// External clock input via GPIO (GPIN0)
    GpIn0 = 5,
    /// External clock input via GPIO (GPIN1)
    GpIn1 = 6,
}

/// USB clock (CLK_USB) auxiliary source selection
///
/// **Important**: USB requires exactly 48 MHz. Use PLL_USB or ensure your
/// source provides exactly 48 MHz (or an integer multiple with appropriate divider).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkUsbAuxSrc {
    /// PLL USB output (48 MHz) - recommended
    PllUsb = 0,
    /// PLL System output - requires division to 48 MHz
    PllSys = 1,
    /// Ring oscillator (ROSC) - NOT recommended for USB
    Rosc = 2,
    /// Crystal oscillator (XOSC) - NOT recommended unless 48 MHz crystal
    Xosc = 3,
    /// External clock input via GPIO (GPIN0)
    GpIn0 = 4,
    /// External clock input via GPIO (GPIN1)
    GpIn1 = 5,
}

/// ADC clock (CLK_ADC) auxiliary source selection
///
/// **Important**: ADC requires exactly 48 MHz for correct operation.
/// Use PLL_USB for best results.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkAdcAuxSrc {
    /// PLL USB output (48 MHz) - recommended
    PllUsb = 0,
    /// PLL System output - requires division to 48 MHz
    PllSys = 1,
    /// Ring oscillator (ROSC) - NOT recommended
    Rosc = 2,
    /// Crystal oscillator (XOSC)
    Xosc = 3,
    /// External clock input via GPIO (GPIN0)
    GpIn0 = 4,
    /// External clock input via GPIO (GPIN1)
    GpIn1 = 5,
}

/// HSTX clock (CLK_HSTX) auxiliary source selection
///
/// High-speed TX clock for DVI and other high-speed interfaces.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkHstxAuxSrc {
    /// System clock (clk_sys)
    ClkSys = 0,
    /// PLL System output - recommended for high-speed operation
    PllSys = 1,
    /// PLL USB output (48 MHz)
    PllUsb = 2,
    /// External clock input via GPIO (GPIN0)
    GpIn0 = 3,
    /// External clock input via GPIO (GPIN1)
    GpIn1 = 4,
}

// ============================================================================
// GPIO Clock Outputs (CLK_GPOUT0-3)
// ============================================================================
// GPOUT clocks can output any internal clock to GPIO pins for external use
// or debugging. They have 16.16 fractional dividers for precise frequencies.
// ============================================================================

/// GPOUT clock (CLK_GPOUT0-3) auxiliary source selection
///
/// GPOUT clocks can output almost any internal clock to GPIO pins.
/// Useful for debugging or providing a clock to external devices.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkGpOutAuxSrc {
    /// PLL System output
    PllSys = 0,
    /// External clock input via GPIO (GPIN0)
    GpIn0 = 1,
    /// External clock input via GPIO (GPIN1)
    GpIn1 = 2,
    /// PLL USB output
    PllUsb = 3,
    /// PLL USB primary reference (OPCG)
    PllUsbPrimaryRefOpcg = 4,
    /// Ring oscillator (ROSC)
    Rosc = 5,
    /// Crystal oscillator (XOSC)
    Xosc = 6,
    /// Low-power oscillator (LPOSC)
    Lposc = 7,
    /// System clock (clk_sys)
    ClkSys = 8,
    /// USB clock (clk_usb)
    ClkUsb = 9,
    /// ADC clock (clk_adc)
    ClkAdc = 10,
    /// Reference clock (clk_ref)
    ClkRef = 11,
    /// Peripheral clock (clk_peri)
    ClkPeri = 12,
    /// HSTX clock (clk_hstx)
    ClkHstx = 13,
    /// OTP clock
    OtpClk2Fc = 14,
}

// ============================================================================
// Frequency Counter Sources
// ============================================================================

/// Frequency counter source for clock measurement
///
/// The frequency counter can measure the frequency of various internal clocks.
/// Useful for verifying clock configuration and debugging.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FcSrc {
    /// No source (counter disabled)
    Null = 0,
    /// PLL System primary output
    PllSysPrimary = 1,
    /// PLL USB primary output
    PllUsbPrimary = 2,
    /// ROSC clock source
    RoscClkSrc = 3,
    /// ROSC clock source (phase shifted)
    RoscClkSrcPh = 4,
    /// XOSC clock source
    XoscClkSrc = 5,
    /// External clock input GPIN0
    ClkSrcGpIn0 = 6,
    /// External clock input GPIN1
    ClkSrcGpIn1 = 7,
    /// Reference clock (clk_ref)
    ClkRef = 8,
    /// System clock (clk_sys)
    ClkSys = 9,
    /// Peripheral clock (clk_peri)
    ClkPeri = 10,
    /// USB clock (clk_usb)
    ClkUsb = 11,
    /// ADC clock (clk_adc)
    ClkAdc = 12,
    /// HSTX clock (clk_hstx)
    ClkHstx = 13,
    /// LPOSC clock source
    LposcClkSrc = 14,
    /// OTP clock
    OtpClk2Fc = 15,
    /// PLL USB primary reference (OPCG)
    PllUsbPrimaryRefOpcg = 16,
}

// ============================================================================
// Clock Frequency Tracking
// ============================================================================

/// Internal storage for tracking configured clock frequencies
///
/// This is used by the [`Clocks`] struct to track the current frequency
/// of each clock after configuration.
#[derive(Clone, Copy, Default)]
struct ClockFrequencies {
    // GPIO outputs
    gpout0: u32,
    gpout1: u32,
    gpout2: u32,
    gpout3: u32,
    // Layer 1: Core clocks
    ref_clk: u32,
    sys_clk: u32,
    // Layer 2: Peripheral clocks
    peri_clk: u32,
    hstx_clk: u32,
    usb_clk: u32,
    adc_clk: u32,
}

// ============================================================================
// Main Clock Controller
// ============================================================================

/// Clock management peripheral wrapper
///
/// This struct provides methods to configure all clocks in the RP2350 clock tree.
/// Methods are organized by clock tree layer:
///
/// 1. **Reference Clock (CLK_REF)**: [`configure_ref_src`](Self::configure_ref_src), [`configure_ref_aux`](Self::configure_ref_aux)
/// 2. **System Clock (CLK_SYS)**: [`configure_sys_from_ref`](Self::configure_sys_from_ref), [`configure_sys_aux`](Self::configure_sys_aux)
/// 3. **Peripheral Clocks**: [`configure_peri`](Self::configure_peri), [`configure_usb`](Self::configure_usb), [`configure_adc`](Self::configure_adc), [`configure_hstx`](Self::configure_hstx)
/// 4. **GPIO Outputs**: [`configure_gpout`](Self::configure_gpout)
///
/// For easy setup, use [`ClockBuilder`]:
/// - `ClockBuilder::new().build()` - 150 MHz sys, 48 MHz USB
/// - `ClockBuilder::low_power().build()` - 48 MHz sys for low power
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
    // ██████╗ ██╗      █████╗ ██╗   ██╗███████╗██████╗      ██╗
    // ██╔══██╗██║     ██╔══██╗╚██╗ ██╔╝██╔════╝██╔══██╗    ███║
    // ██████╔╝██║     ███████║ ╚████╔╝ █████╗  ██████╔╝    ╚██║
    // ██╔═══╝ ██║     ██╔══██║  ╚██╔╝  ██╔══╝  ██╔══██╗     ██║
    // ██║     ███████╗██║  ██║   ██║   ███████╗██║  ██║     ██║
    // ╚═╝     ╚══════╝╚═╝  ╚═╝   ╚═╝   ╚══════╝╚═╝  ╚═╝     ╚═╝
    // Reference Clock (CLK_REF) - Stable reference for system clock
    // ========================================================================
    //
    // CLK_REF provides a stable clock source for the system clock mux and
    // watchdog timer. It should be configured first during initialization.
    //
    // Recommended: Use XOSC (12 MHz) for stability
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
    // ██████╗ ██╗      █████╗ ██╗   ██╗███████╗██████╗      ██╗
    // ██╔══██╗██║     ██╔══██╗╚██╗ ██╔╝██╔════╝██╔══██╗    ███║
    // ██████╔╝██║     ███████║ ╚████╔╝ █████╗  ██████╔╝    ╚██║
    // ██╔═══╝ ██║     ██╔══██║  ╚██╔╝  ██╔══╝  ██╔══██╗     ██║
    // ██║     ███████╗██║  ██║   ██║   ███████╗██║  ██║     ██║
    // ╚═╝     ╚══════╝╚═╝  ╚═╝   ╚═╝   ╚══════╝╚═╝  ╚═╝     ╚═╝
    // System Clock (CLK_SYS) - Main clock for CPU and peripherals
    // ========================================================================
    //
    // CLK_SYS is the main clock driving the CPU (Cortex-M33), AHB bus, and
    // APB bus. It supports glitchless switching between clk_ref and aux sources.
    //
    // Configuration sequence for switching to a new source:
    // 1. Switch to clk_ref (safe intermediate state)
    // 2. Configure the new aux source (e.g., PLL_SYS)
    // 3. Set divider if source > target frequency
    // 4. Switch to aux source
    //
    // Maximum frequency: 150 MHz
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

        // Calculate actual frequency: src_freq / (div / 65536) = src_freq * 65536 / div
        self.frequencies.sys_clk = (((src_freq as u64) << 16) / div as u64) as u32;
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

    /// Get the current system clock frequency (alias for [`get_sys_freq`](Self::get_sys_freq))
    #[inline]
    pub fn sys_freq(&self) -> u32 {
        self.frequencies.sys_clk
    }

    /// Set the system clock frequency directly (for tracking purposes)
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
    // Peripheral Clocks - Clocks for specific hardware blocks
    // ========================================================================
    //
    // Layer 2 clocks derive from Layer 1 (clk_sys, clk_ref) or directly from
    // PLLs and oscillators. Each peripheral clock has specific requirements.
    // ========================================================================

    // ------------------------------------------------------------------------
    // CLK_PERI - Peripheral Clock (UART, SPI, I2C, etc.)
    // ------------------------------------------------------------------------
    // CLK_PERI supplies the clock for serial peripherals. Using clk_sys is
    // simplest, but baud rates will change if clk_sys frequency changes.
    // For stable baud rates independent of clk_sys, use PLL_USB (48 MHz).
    // ------------------------------------------------------------------------

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

    // ------------------------------------------------------------------------
    // CLK_USB - USB Clock (MUST be 48 MHz)
    // ------------------------------------------------------------------------
    // USB 1.1/2.0 requires exactly 48 MHz. The hardware will automatically
    // calculate the divider if the source frequency is higher.
    // Recommended source: PLL_USB (already configured to 48 MHz)
    // ------------------------------------------------------------------------

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

    /// Get the current USB clock frequency (alias for [`get_usb_freq`](Self::get_usb_freq))
    #[inline]
    pub fn usb_freq(&self) -> u32 {
        self.frequencies.usb_clk
    }

    // ------------------------------------------------------------------------
    // CLK_ADC - ADC Clock (MUST be 48 MHz)
    // ------------------------------------------------------------------------
    // The ADC requires exactly 48 MHz for correct sampling. The sample rate
    // is derived from this clock (500 ksps max at 48 MHz).
    // Recommended source: PLL_USB (already configured to 48 MHz)
    // ------------------------------------------------------------------------

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

    // ------------------------------------------------------------------------
    // CLK_HSTX - High-Speed TX Clock (for DVI/HSTX peripheral)
    // ------------------------------------------------------------------------
    // HSTX provides high-speed serialized output for DVI and other interfaces.
    // Typically sourced from PLL_SYS for maximum speed.
    // ------------------------------------------------------------------------

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
    //  ██████╗ ██████╗  ██████╗ ██╗   ██╗████████╗
    // ██╔════╝ ██╔══██╗██╔═══██╗██║   ██║╚══██╔══╝
    // ██║  ███╗██████╔╝██║   ██║██║   ██║   ██║
    // ██║   ██║██╔═══╝ ██║   ██║██║   ██║   ██║
    // ╚██████╔╝██║     ╚██████╔╝╚██████╔╝   ██║
    //  ╚═════╝ ╚═╝      ╚═════╝  ╚═════╝    ╚═╝
    // GPIO Clock Outputs - Export clocks to GPIO pins
    // ========================================================================
    //
    // GPOUT clocks can output almost any internal clock to GPIO pins.
    // Useful for:
    // - Debugging clock configuration
    // - Providing clock to external devices
    // - Precise frequency generation with 16.16 fractional divider
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
    // Clock Control Operations
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
    // Frequency Measurement (FC0)
    // ========================================================================
    //
    // The frequency counter can measure any clock source against clk_ref.
    // Useful for verifying clock configuration and debugging.
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
    // System Clock Recovery (RESUS)
    // ========================================================================
    //
    // RESUS automatically switches clk_sys back to clk_ref if clk_sys stops.
    // This is a safety feature to prevent the system from hanging if the
    // clock source fails (e.g., PLL loses lock).
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
    // Frequency Query and Tracking
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
// ███████╗ █████╗ ███████╗██╗   ██╗    ██╗███╗   ██╗██╗████████╗
// ██╔════╝██╔══██╗██╔════╝╚██╗ ██╔╝    ██║████╗  ██║██║╚══██╔══╝
// █████╗  ███████║███████╗ ╚████╔╝     ██║██╔██╗ ██║██║   ██║
// ██╔══╝  ██╔══██║╚════██║  ╚██╔╝      ██║██║╚██╗██║██║   ██║
// ███████╗██║  ██║███████║   ██║       ██║██║ ╚████║██║   ██║
// ╚══════╝╚═╝  ╚═╝╚══════╝   ╚═╝       ╚═╝╚═╝  ╚═══╝╚═╝   ╚═╝
// Convenience Functions and Predefined Profiles
// ============================================================================
//
// These functions provide easy, one-call initialization of the entire clock
// tree for common use cases. The initialization sequence follows the clock
// tree hierarchy:
//
//   1. Start XOSC and wait for stable
//   2. Configure CLK_REF from XOSC
//   3. Configure CLK_SYS from CLK_REF (safe intermediate)
//   4. Configure PLLs (PLL_SYS, PLL_USB)
//   5. Switch CLK_SYS to PLL_SYS
//   6. Configure peripheral clocks (CLK_PERI, CLK_USB, CLK_ADC)
//
// ============================================================================

use crate::pll::{PllConfig, PllSys, PllUsb};
use crate::reset::{self, Peripheral};
use crate::xosc::Xosc;

// ============================================================================
// Clock Builder - Flexible Clock Configuration with Sensible Defaults
// ============================================================================
//
// The ClockBuilder provides a fluent API for configuring the clock tree.
// It follows the builder pattern with sensible defaults:
//
//   Default configuration:
//   - XOSC: 12 MHz (standard Pico crystal)
//   - CLK_SYS: 150 MHz (from PLL_SYS)
//   - CLK_USB: 48 MHz (from PLL_USB)
//   - CLK_PERI: Same as CLK_SYS
//   - CLK_ADC: 48 MHz (from PLL_USB)
//
// ============================================================================

/// Builder for configuring the RP2350 clock system
///
/// Provides a fluent API with sensible defaults for easy clock configuration.
/// All settings have reasonable defaults, so you only need to customize what
/// you want to change.
///
/// # Default Configuration
///
/// | Clock     | Default Value | Source   |
/// |-----------|---------------|----------|
/// | XOSC      | 12 MHz        | Crystal  |
/// | clk_sys   | 150 MHz       | PLL_SYS  |
/// | clk_usb   | 48 MHz        | PLL_USB  |
/// | clk_peri  | 150 MHz       | clk_sys  |
/// | clk_adc   | 48 MHz        | PLL_USB  |
/// | clk_hstx  | (disabled)    | -        |
///
/// # Example
///
/// ```ignore
/// use pico_rs::clocks::ClockBuilder;
///
/// // Use all defaults (150 MHz sys, 48 MHz USB)
/// let clocks = ClockBuilder::new().build();
///
/// // Customize system frequency
/// let clocks = ClockBuilder::new()
///     .sys_freq_mhz(125)
///     .build();
///
/// // Full customization
/// let clocks = ClockBuilder::new()
///     .xosc_freq(12_000_000)
///     .sys_freq_mhz(133)
///     .peri_src(ClkPeriAuxSrc::PllUsb)  // Fixed 48 MHz for stable baud rates
///     .enable_adc()
///     .build();
/// ```
#[derive(Clone)]
pub struct ClockBuilder {
    /// Crystal oscillator frequency (default: 12 MHz)
    xosc_freq: u32,
    /// Target system clock frequency (default: 150 MHz)
    sys_freq: u32,
    /// Peripheral clock source (default: ClkSys)
    peri_src: ClkPeriAuxSrc,
    /// Peripheral clock frequency (only used if peri_src != ClkSys)
    peri_freq: Option<u32>,
    /// Enable USB clock (default: false)
    enable_usb: bool,
    /// Enable ADC clock (default: false)
    enable_adc: bool,
    /// HSTX clock configuration (default: None = disabled)
    hstx_config: Option<(ClkHstxAuxSrc, u32)>,
}

impl Default for ClockBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ClockBuilder {
    /// Create a new ClockBuilder with default configuration
    ///
    /// Default: 12 MHz XOSC, 150 MHz sys clock, 48 MHz USB clock
    pub const fn new() -> Self {
        Self {
            xosc_freq: 12_000_000,
            sys_freq: 150_000_000,
            peri_src: ClkPeriAuxSrc::ClkSys,
            peri_freq: None,
            enable_usb: false,
            enable_adc: false,
            hstx_config: None,
        }
    }

    // ========================================================================
    // Preset Profiles
    // ========================================================================

    /// Configure for low power operation (48 MHz system clock)
    ///
    /// Uses PLL_USB for system clock, PLL_SYS remains off for power savings.
    ///
    /// # Example
    /// ```ignore
    /// let clocks = ClockBuilder::low_power().build();
    /// ```
    pub const fn low_power() -> Self {
        Self {
            xosc_freq: 12_000_000,
            sys_freq: 48_000_000,
            peri_src: ClkPeriAuxSrc::ClkSys,
            peri_freq: None,
            enable_usb: false,
            enable_adc: false,
            hstx_config: None,
        }
    }

    // ========================================================================
    // Clock Source Configuration
    // ========================================================================

    /// Set the crystal oscillator frequency
    ///
    /// Default: 12 MHz (standard for Raspberry Pi Pico boards)
    ///
    /// # Arguments
    /// * `freq` - XOSC frequency in Hz
    ///
    /// # Note
    /// PLL configurations assume 12 MHz XOSC. Other frequencies may require
    /// custom PLL settings for correct output frequencies.
    pub const fn xosc_freq(mut self, freq: u32) -> Self {
        self.xosc_freq = freq;
        self
    }

    /// Set the crystal oscillator frequency in MHz
    ///
    /// Convenience method for [`xosc_freq`](Self::xosc_freq).
    pub const fn xosc_freq_mhz(mut self, mhz: u32) -> Self {
        self.xosc_freq = mhz * 1_000_000;
        self
    }

    // ========================================================================
    // System Clock Configuration
    // ========================================================================

    /// Set the target system clock frequency
    ///
    /// Default: 150 MHz
    ///
    /// # Supported Frequencies
    /// - 48 MHz (uses PLL_USB, low power)
    /// - 125 MHz (uses PLL_SYS)
    /// - 133 MHz (uses PLL_SYS)
    /// - 150 MHz (uses PLL_SYS, maximum)
    ///
    /// # Arguments
    /// * `freq` - Target frequency in Hz
    pub const fn sys_freq(mut self, freq: u32) -> Self {
        self.sys_freq = freq;
        self
    }

    /// Set the target system clock frequency in MHz
    ///
    /// Convenience method for [`sys_freq`](Self::sys_freq).
    ///
    /// # Example
    /// ```ignore
    /// let clocks = ClockBuilder::new()
    ///     .sys_freq_mhz(125)
    ///     .build();
    /// ```
    pub const fn sys_freq_mhz(mut self, mhz: u32) -> Self {
        self.sys_freq = mhz * 1_000_000;
        self
    }

    // ========================================================================
    // Peripheral Clock Configuration
    // ========================================================================

    /// Set the peripheral clock source
    ///
    /// Default: `ClkPeriAuxSrc::ClkSys` (follows system clock)
    ///
    /// # Tradeoffs
    /// - `ClkSys`: Simplest, but baud rates change if sys clock changes
    /// - `PllUsb` (48 MHz): Stable baud rates, independent of sys clock
    /// - `PllSys`: Direct from PLL, independent of sys clock divider
    pub const fn peri_src(mut self, src: ClkPeriAuxSrc) -> Self {
        self.peri_src = src;
        self
    }

    /// Set a fixed peripheral clock frequency
    ///
    /// Use when `peri_src` is not `ClkSys` to specify the source frequency.
    /// Required for correct baud rate calculation.
    pub const fn peri_freq(mut self, freq: u32) -> Self {
        self.peri_freq = Some(freq);
        self
    }

    /// Configure peripheral clock for stable baud rates (48 MHz from PLL_USB)
    ///
    /// This is useful when you want UART/SPI/I2C baud rates to remain
    /// constant even if the system clock frequency changes.
    pub const fn peri_stable_48mhz(mut self) -> Self {
        self.peri_src = ClkPeriAuxSrc::PllUsb;
        self.peri_freq = Some(48_000_000);
        self
    }

    // ========================================================================
    // USB/ADC Clock Configuration
    // ========================================================================

    /// Enable USB clock (48 MHz from PLL_USB)
    ///
    /// Default: enabled
    pub const fn enable_usb(mut self) -> Self {
        self.enable_usb = true;
        self
    }

    /// Disable USB clock
    ///
    /// Use this if USB is not needed to save a small amount of power.
    pub const fn disable_usb(mut self) -> Self {
        self.enable_usb = false;
        self
    }

    /// Enable ADC clock (48 MHz from PLL_USB)
    ///
    /// Default: disabled (enable when using ADC)
    pub const fn enable_adc(mut self) -> Self {
        self.enable_adc = true;
        self
    }

    /// Disable ADC clock
    pub const fn disable_adc(mut self) -> Self {
        self.enable_adc = false;
        self
    }

    // ========================================================================
    // HSTX Clock Configuration
    // ========================================================================

    /// Enable HSTX clock with specified source and frequency
    ///
    /// Default: disabled
    ///
    /// # Arguments
    /// * `src` - HSTX clock source
    /// * `freq` - Source frequency in Hz
    pub const fn enable_hstx(mut self, src: ClkHstxAuxSrc, freq: u32) -> Self {
        self.hstx_config = Some((src, freq));
        self
    }

    /// Enable HSTX clock from system clock
    ///
    /// Convenient shorthand when HSTX should run at sys clock frequency.
    pub const fn enable_hstx_from_sys(mut self) -> Self {
        // Will be resolved to actual sys_freq during build
        self.hstx_config = Some((ClkHstxAuxSrc::ClkSys, 0)); // 0 = use sys_freq
        self
    }

    // ========================================================================
    // Build
    // ========================================================================

    /// Build and apply the clock configuration
    ///
    /// This executes the full clock initialization sequence:
    ///
    /// 1. Start XOSC and wait for stable
    /// 2. Configure CLK_REF from XOSC
    /// 3. Switch CLK_SYS to CLK_REF (safe intermediate)
    /// 4. Configure PLL_SYS (if sys_freq > 48 MHz)
    /// 5. Configure PLL_USB (for USB/ADC at 48 MHz)
    /// 6. Switch CLK_SYS to final source
    /// 7. Configure peripheral clocks
    ///
    /// # Returns
    /// A [`Clocks`] instance with the configured frequencies
    ///
    /// # Panics
    /// Panics if an unsupported system frequency is requested.
    pub fn build(self) -> Clocks {
        // ====================================================================
        // Step 1: Initialize XOSC (Clock Tree Layer 0)
        // ====================================================================
        let xosc = Xosc::init(self.xosc_freq);
        xosc.wait_stable();

        let mut clocks = Clocks::new();

        // ====================================================================
        // Step 2: Configure CLK_REF from XOSC (Clock Tree Layer 1)
        // ====================================================================
        clocks.configure_ref_src(ClkRefSrc::Xosc, self.xosc_freq);

        // ====================================================================
        // Step 3: Switch CLK_SYS to CLK_REF (safe intermediate state)
        // ====================================================================
        clocks.configure_sys_from_ref();

        // ====================================================================
        // Step 4: Configure PLLs based on requested frequencies
        // ====================================================================
        let use_pll_sys = self.sys_freq > 48_000_000;

        if use_pll_sys {
            // Configure PLL_SYS for requested frequency
            reset::reset_block(Peripheral::PLL_SYS);
            reset::unreset_block_wait(Peripheral::PLL_SYS);

            let pll_sys = PllSys::new();
            let sys_config = Self::get_pll_config_for_freq(self.sys_freq);
            pll_sys.configure(
                sys_config.refdiv,
                sys_config.fbdiv,
                sys_config.postdiv1,
                sys_config.postdiv2,
            );
            pll_sys.wait_lock();
        }

        // ====================================================================
        // Step 5: Configure PLL_USB (48 MHz for USB/ADC)
        // ====================================================================
        // Always configure PLL_USB - needed for USB, ADC, or low-power sys clock
        reset::reset_block(Peripheral::PLL_USB);
        reset::unreset_block_wait(Peripheral::PLL_USB);

        let pll_usb = PllUsb::new();
        let usb_config = PllConfig::usb_48mhz();
        pll_usb.configure(
            usb_config.refdiv,
            usb_config.fbdiv,
            usb_config.postdiv1,
            usb_config.postdiv2,
        );
        pll_usb.wait_lock();

        // ====================================================================
        // Step 6: Switch CLK_SYS to final source
        // ====================================================================
        let actual_sys_freq = if use_pll_sys {
            clocks.configure_sys_aux(ClkSysAuxSrc::PllSys, self.sys_freq, self.sys_freq);
            self.sys_freq
        } else {
            // Low power: use PLL_USB
            clocks.configure_sys_aux(ClkSysAuxSrc::PllUsb, 48_000_000, 48_000_000);
            48_000_000
        };

        // ====================================================================
        // Step 7: Configure peripheral clocks (Clock Tree Layer 2)
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
            clocks.configure_usb(ClkUsbAuxSrc::PllUsb, 48_000_000);
        }

        // CLK_ADC
        if self.enable_adc {
            clocks.configure_adc(ClkAdcAuxSrc::PllUsb, 48_000_000);
        }

        // CLK_HSTX
        if let Some((src, freq)) = self.hstx_config {
            let actual_freq = if freq == 0 { actual_sys_freq } else { freq };
            clocks.configure_hstx(src, actual_freq);
        }

        clocks
    }

    /// Get PLL configuration for a target frequency
    fn get_pll_config_for_freq(freq: u32) -> PllConfig {
        match freq {
            125_000_000 => PllConfig::sys_125mhz(),
            133_000_000 => PllConfig::sys_133mhz(),
            150_000_000 => PllConfig::sys_150mhz(),
            f if f <= 48_000_000 => PllConfig::usb_48mhz(), // Will use PLL_USB
            _ => panic!(
                "Unsupported system frequency: {} Hz. Use 48, 125, 133, or 150 MHz.",
                freq
            ),
        }
    }
}
