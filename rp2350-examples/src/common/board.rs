//! Board-level initialization helpers shared by example binaries.
//!
//! Every example follows the same boilerplate:
//!
//! ```text
//! let dp   = Peripherals::take().unwrap();
//! let pins = Pins::init(dp.IO_BANK0, dp.PADS_BANK0);
//! let clocks = ClockBuilder::new().build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);
//! let core = CorePeripherals::take().unwrap();
//! let delay = Delay::init(core.SYST, clocks.sys_freq());
//! ```
//!
//! This module centralizes that so examples stay short and consistent.
//! Use [`Board::init`] when the example does not need the device
//! peripherals (`dp`) afterwards, or [`init_delay`] to only create a
//! `Delay` from already-configured clocks.

use rp2350::clocks::{ClockBuilder, Clocks};
use rp2350::gpio::Pins;
use rp2350::pac::Peripherals;
use rp2350::rt::peripheral::Peripherals as CorePeripherals;
use rp2350::systick::Delay;

/// Board initialized with the default clocks (150 MHz) and GPIO pins.
///
/// Consumes the clock and GPIO peripherals, so only use this when the
/// example does not need `Peripherals` (`dp`) afterwards.
pub struct Board {
    /// Configured system clocks (default: 150 MHz).
    pub clocks: Clocks,
    /// Initialized GPIO pins.
    pub pins: Pins,
}

impl Board {
    /// Take device peripherals, build default clocks and initialize pins.
    ///
    /// # Panics
    ///
    /// Panics if [`Peripherals::take`] returns `None` (already taken).
    pub fn init() -> Self {
        let dp = Peripherals::take().unwrap();
        let clocks = ClockBuilder::new().build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);
        let pins = Pins::init(dp.IO_BANK0, dp.PADS_BANK0);
        Self { clocks, pins }
    }

    /// Build a `Delay` from the configured system clock frequency.
    ///
    /// # Panics
    ///
    /// Panics if [`CorePeripherals::take`] fails (already taken).
    pub fn delay(&self) -> Delay {
        init_delay(&self.clocks)
    }
}

/// Build a SysTick-based [`Delay`] from already-configured clocks.
///
/// Uses `clocks.sys_freq()` so the delay always matches the actual system
/// clock instead of a hard-coded constant.
///
/// # Panics
///
/// Panics if [`CorePeripherals::take`] fails (already taken, e.g. by
/// interrupt setup).
pub fn init_delay(clocks: &Clocks) -> Delay {
    let core = CorePeripherals::take().unwrap();
    Delay::init(core.SYST, clocks.sys_freq())
}
