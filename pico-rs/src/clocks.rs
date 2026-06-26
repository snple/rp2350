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
//! ╔══════════════════════════════════════════════════════════════════════════════╗
//! ║                         CLOCK SOURCES (Layer 0)                              ║
//! ╠══════════════════════════════════════════════════════════════════════════════╣
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
//! ║  │  ┌─────────────┐   ┌─────────────┐   ┌─────────────┐   ┌──────────┐  │    ║
//! ║  │  │  CLK_PERI   │   │  CLK_USB    │   │  CLK_ADC    │   │ CLK_HSTX │  │    ║
//! ║  │  └──────┬──────┘   └──────┬──────┘   └──────┬──────┘   └────┬─────┘  │    ║
//! ║  │         ▼                 ▼                 ▼               ▼         │    ║
//! ║  │    UART, SPI,       USB Device/       ADC Module       DVI/HSTX      │    ║
//! ║  │    I2C, etc.        Host                                              │    ║
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
//! ```text
//! ┌─────────────────────────────────────────────────────────────────┐
//! │ 1. Boot: ROSC running (~6.5 MHz)                                │
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
//! let dp = pico_rs_pac::Peripherals::take().unwrap();
//!
//! // Recommended: use ClockBuilder
//! let clocks = ClockBuilder::new()
//!     .enable_usb()
//!     .enable_adc()
//!     .build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);
//! // clk_sys = 150 MHz, clk_usb = 48 MHz, clk_peri = 150 MHz
//!
//! // Custom: 125 MHz sys, stable 48 MHz for UART baud rates
//! let clocks = ClockBuilder::new()
//!     .sys_freq_mhz(125)
//!     .peri_stable_48mhz()
//!     .build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);
//! ```

// ============================================================================
// Sub-modules
// ============================================================================

// Clock sources (Layer 0)
pub mod lposc;
pub mod pll;
pub mod rosc;
pub mod xosc;

// Clock tree logic (private sub-modules, types re-exported below)
mod builder;
mod controller;
mod gate;
mod types;

// ============================================================================
// Re-exports — public API
// ============================================================================

// Clock sources
pub use lposc::Lposc;
pub use pll::{PllConfig, PllSys, PllUsb};
pub use rosc::Rosc;
pub use xosc::Xosc;

// Clock tree types
pub use types::{
    CLK_COUNT, ClkAdcAuxSrc, ClkGpOutAuxSrc, ClkHstxAuxSrc, ClkPeriAuxSrc, ClkRefAuxSrc, ClkRefSrc,
    ClkSysAuxSrc, ClkUsbAuxSrc, ClockId, DftClkSrc, FcSrc, GpOutId,
};

// Clock gate / sleep control
pub use gate::ClkPeripheral;

// Core clock controller
pub use controller::Clocks;

// Clock builder
pub use builder::ClockBuilder;
