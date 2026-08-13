//! Clock type definitions for RP2350
//!
//! This module contains all enumerations used to select clock sources,
//! auxiliary sources, frequency counter sources, and debug clock sources
//! throughout the clock tree.
//!
//! # Clock Tree Layers
//!
//! Types are grouped by their position in the clock tree hierarchy:
//!
//! - **Layer 0 (Sources)**: The fundamental oscillators and PLLs are defined
//!   in the sibling `xosc`, `rosc`, `lposc`, and `pll` modules.
//!
//! - **Layer 1 (Reference & System Clocks)**: [`ClkRefSrc`], [`ClkRefAuxSrc`],
//!   [`ClkSysAuxSrc`]
//!
//! - **Layer 2 (Peripheral Clocks)**: [`ClkPeriAuxSrc`], [`ClkUsbAuxSrc`],
//!   [`ClkAdcAuxSrc`], [`ClkHstxAuxSrc`]
//!
//! - **GPIO Outputs**: [`ClkGpOutAuxSrc`], [`GpOutId`]
//!
//! - **Miscellaneous**: [`ClockId`], [`FcSrc`], [`DftClkSrc`]

// ============================================================================
// Clock Identifiers
// ============================================================================

/// Number of core clock generators tracked by [`Clocks`].
///
/// The four GPIO clock outputs are tracked separately via [`GpOutId`].
pub const CLK_COUNT: usize = 6;

/// Identifiers for the six core clock generators.
///
/// The clocks are organised by their position in the clock tree:
/// - Layer 0: Clock sources (XOSC, ROSC, PLLs) — not in this enum
/// - Layer 1: Reference and System clocks ([`ClockId::Ref`], [`ClockId::Sys`])
/// - Layer 2: Peripheral clocks ([`ClockId::Peri`], [`ClockId::Usb`],
///   [`ClockId::Adc`], [`ClockId::Hstx`])
///
/// GPIO clock outputs are handled by the separate [`GpOutId`] enum.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(usize)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClockId {
    /// Reference clock — stable clock for system clock mux and watchdog
    Ref = 0,
    /// System clock — main clock for CPU and most peripherals
    Sys = 1,
    /// Peripheral clock (UART, SPI, I2C, etc.)
    Peri = 2,
    /// High-speed TX clock (DVI / HSTX peripheral)
    Hstx = 3,
    /// USB clock — MUST be exactly 48 MHz
    Usb = 4,
    /// ADC clock — MUST be exactly 48 MHz
    Adc = 5,
}

impl ClockId {
    /// Number of core clock generators.
    pub const COUNT: usize = CLK_COUNT;

    /// Index of this clock into the internal frequency table.
    #[inline]
    pub const fn index(self) -> usize {
        self as usize
    }
}

// ============================================================================
// GPIO Clock Output Identifier
// ============================================================================

/// Identifies one of the four GPIO clock output generators (GPOUT0–3)
///
/// Replaces bare `u8` indices to provide compile-time safety when configuring
/// or disabling GPIO clock outputs.
///
/// # Example
/// ```ignore
/// clocks.configure_gpout(GpOutId::GpOut0, ClkGpOutAuxSrc::ClkSys, sys_freq, sys_freq / 4);
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(usize)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GpOutId {
    /// GPIO clock output 0
    GpOut0 = 0,
    /// GPIO clock output 1
    GpOut1 = 1,
    /// GPIO clock output 2
    GpOut2 = 2,
    /// GPIO clock output 3
    GpOut3 = 3,
}

impl GpOutId {
    /// Index of this output into the internal GPOUT frequency table.
    #[inline]
    pub const fn index(self) -> usize {
        self as usize
    }
}

// ============================================================================
// Clock Tree Layer 1: Reference Clock (CLK_REF) Sources
// ============================================================================
//
// CLK_REF provides a stable clock for the system clock mux and watchdog timer.
//
//  ROSC  ────┐
//  LPOSC ────┼──► [CLK_REF glitchless MUX] ──► ÷DIV ──► clk_ref
//  XOSC  ────┤
//  AUX   ────┘   (AUX = PLL_USB, GPIN0/1; selected via ClkRefAuxSrc)
//
// ============================================================================

/// Reference clock (CLK_REF) primary source selection
///
/// These options go through the glitchless mux. Use [`ClkRefAuxSrc`] together
/// with [`ClkRefSrc::Aux`] to select auxiliary sources such as PLL_USB or GPIN.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkRefSrc {
    /// Ring oscillator (ROSC) — ~6.5 MHz, variable, default at boot
    Rosc = 0,
    /// Auxiliary source — see [`ClkRefAuxSrc`] for available options
    Aux = 1,
    /// Crystal oscillator (XOSC) — 12 MHz, high stability (recommended)
    Xosc = 2,
    /// Low-power oscillator (LPOSC) — ~32 kHz
    Lposc = 3,
}

/// Reference clock (CLK_REF) auxiliary source selection
///
/// Active when [`ClkRefSrc::Aux`] is selected as the primary source.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkRefAuxSrc {
    /// PLL USB output (48 MHz)
    PllUsb = 0,
    /// External clock input via GPIO20 (GPIN0)
    GpIn0 = 1,
    /// External clock input via GPIO22 (GPIN1)
    GpIn1 = 2,
    /// PLL USB primary reference (OPCG)
    PllUsbPrimaryRefOpcg = 3,
}

// ============================================================================
// Clock Tree Layer 1: System Clock (CLK_SYS) Sources
// ============================================================================
//
// CLK_SYS drives the CPU (Cortex-M33), AHB bus, and APB bus.
//
//  clk_ref ────┐
//              ├──► [CLK_SYS glitchless MUX] ──► ÷DIV (16.16) ──► clk_sys
//  AUX     ────┘   (AUX selected via ClkSysAuxSrc)
//
// ============================================================================

/// System clock (CLK_SYS) auxiliary source selection
///
/// Selects the source fed through the auxiliary mux. Use [`Clocks::configure_sys_aux`]
/// to switch clk_sys to one of these sources. PLL_SYS is recommended for maximum
/// performance (125–150 MHz); PLL_USB (48 MHz) is useful for low-power operation.
///
/// [`Clocks::configure_sys_aux`]: super::controller::Clocks::configure_sys_aux
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkSysAuxSrc {
    /// PLL System output (125–150 MHz) — recommended for high performance
    PllSys = 0,
    /// PLL USB output (48 MHz) — useful for low-power operation
    PllUsb = 1,
    /// Ring oscillator (ROSC) — ~6.5 MHz, variable
    Rosc = 2,
    /// Crystal oscillator (XOSC) — 12 MHz, stable
    Xosc = 3,
    /// External clock input via GPIO20 (GPIN0)
    GpIn0 = 4,
    /// External clock input via GPIO22 (GPIN1)
    GpIn1 = 5,
}

// ============================================================================
// Clock Tree Layer 2: Peripheral Clocks
// ============================================================================
//
// These clocks derive from Layer 1 or directly from PLLs / oscillators.
// Key constraints:
//   CLK_USB  — MUST be exactly 48 MHz
//   CLK_ADC  — MUST be exactly 48 MHz
//   CLK_PERI — Flexible; clk_sys is simplest
//   CLK_HSTX — High-speed clock for DVI/HSTX
//
// ============================================================================

/// Peripheral clock (CLK_PERI) auxiliary source selection
///
/// CLK_PERI supplies UART, SPI, I2C and similar peripherals.
///
/// | Choice       | Trade-off                                           |
/// |--------------|-----------------------------------------------------|
/// | `ClkSys`     | Simplest, but baud rates change with clk_sys        |
/// | `PllUsb`     | 48 MHz fixed — stable baud rates                    |
/// | `PllSys`     | Direct PLL — independent of clk_sys divider         |
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkPeriAuxSrc {
    /// System clock (clk_sys)
    ClkSys = 0,
    /// PLL System output — independent of clk_sys changes
    PllSys = 1,
    /// PLL USB output (48 MHz) — stable, independent reference
    PllUsb = 2,
    /// Ring oscillator (ROSC)
    Rosc = 3,
    /// Crystal oscillator (XOSC)
    Xosc = 4,
    /// External clock input via GPIO20 (GPIN0)
    GpIn0 = 5,
    /// External clock input via GPIO22 (GPIN1)
    GpIn1 = 6,
}

/// USB clock (CLK_USB) auxiliary source selection
///
/// **USB requires exactly 48 MHz.** Use `PllUsb` for correct operation.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkUsbAuxSrc {
    /// PLL USB output (48 MHz) — **recommended**
    PllUsb = 0,
    /// PLL System output — only if divided down to 48 MHz
    PllSys = 1,
    /// Ring oscillator (ROSC) — **not recommended** for USB
    Rosc = 2,
    /// Crystal oscillator (XOSC) — not recommended unless 48 MHz crystal
    Xosc = 3,
    /// External clock input via GPIO20 (GPIN0)
    GpIn0 = 4,
    /// External clock input via GPIO22 (GPIN1)
    GpIn1 = 5,
}

/// ADC clock (CLK_ADC) auxiliary source selection
///
/// **ADC requires exactly 48 MHz.** Use `PllUsb` for correct operation.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkAdcAuxSrc {
    /// PLL USB output (48 MHz) — **recommended**
    PllUsb = 0,
    /// PLL System output — only if divided down to 48 MHz
    PllSys = 1,
    /// Ring oscillator (ROSC) — **not recommended**
    Rosc = 2,
    /// Crystal oscillator (XOSC)
    Xosc = 3,
    /// External clock input via GPIO20 (GPIN0)
    GpIn0 = 4,
    /// External clock input via GPIO22 (GPIN1)
    GpIn1 = 5,
}

/// HSTX clock (CLK_HSTX) auxiliary source selection
///
/// High-speed TX clock for DVI and other high-speed serial interfaces.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkHstxAuxSrc {
    /// System clock (clk_sys)
    ClkSys = 0,
    /// PLL System output — recommended for maximum speed
    PllSys = 1,
    /// PLL USB output (48 MHz)
    PllUsb = 2,
    /// External clock input via GPIO20 (GPIN0)
    GpIn0 = 3,
    /// External clock input via GPIO22 (GPIN1)
    GpIn1 = 4,
}

// ============================================================================
// GPIO Clock Outputs (CLK_GPOUT0–3)
// ============================================================================
//
// GPOUT clocks export any internal clock to GPIO pins. Each output has a
// 16.16 fractional divider for precise frequency generation.
//
// ============================================================================

/// GPOUT clock (CLK_GPOUT0–3) auxiliary source selection
///
/// GPOUT outputs can expose almost any internal clock on a GPIO pin — useful
/// for debugging clock configuration or providing a reference to external ICs.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkGpOutAuxSrc {
    /// PLL System output
    PllSys = 0,
    /// External clock input via GPIO20 (GPIN0)
    GpIn0 = 1,
    /// External clock input via GPIO22 (GPIN1)
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

/// Frequency counter (FC0) source selection
///
/// The FC0 counter can measure any of these sources against `clk_ref`.
/// Useful for verifying clock configuration and debugging.
///
/// # Usage
/// See [`Clocks::frequency_count_khz`](super::controller::Clocks::frequency_count_khz).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FcSrc {
    /// No source — counter disabled
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
    /// Low-power oscillator clock source
    LposcClkSrc = 14,
    /// OTP clock
    OtpClk2Fc = 15,
    /// PLL USB primary reference (OPCG)
    PllUsbPrimaryRefOpcg = 16,
}

// ============================================================================
// DFT Debug Clock Sources
// ============================================================================

/// DFT (Design-for-Test) debug clock source for oscillator bypass
///
/// Each oscillator (XOSC, ROSC, LPOSC) can have its output replaced by a
/// debug source for silicon testing and fault isolation. The exact source
/// depends on which oscillator is being configured:
///
/// | Oscillator | [`DebugClk`](DftClkSrc::DebugClk) source | [`GpIn`](DftClkSrc::GpIn) source |
/// |------------|------------------------------------------|----------------------------------|
/// | XOSC       | PLL_USB primary                          | GPIN0                            |
/// | ROSC       | PLL_SYS primary                          | GPIN1                            |
/// | LPOSC      | PLL_USB primary                          | GPIN1                            |
///
/// **Warning**: Misuse of DFT clocks can cause unstable operation.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DftClkSrc {
    /// Normal operation — no debug override
    Null = 0,
    /// Replace oscillator output with a PLL clock (source depends on oscillator)
    DebugClk = 1,
    /// Replace oscillator output with a GPIN clock (pin depends on oscillator)
    GpIn = 2,
}
