//! Shared pin-bridge helpers for the stepper examples.
//!
//! Every stepper example used to hand-write a `struct MotPins;` with the
//! same [`AxisPins`] (and optional [`TmcPins`]) impl calling
//! `gpio::set`/`gpio::get` with hard-coded pin constants. These
//! const-generic zero-sized types replace that boilerplate: pick the one
//! matching your GPIO access path, alias it with your pin numbers, and use
//! it as the `AxisState` type parameter.
//!
//! Two paths are supported because the examples demonstrate both:
//!
//! - **SIO MMIO** ([`rp2350::gpio`]) — `GpioStepperPins` / `GpioTmcPins`
//! - **GPIOC coprocessor** ([`rp2350::gpioc`]) — `GpiocStepperPins` / `GpiocTmcPins`
//!
//! The `*TmcPins` variants additionally implement [`TmcPins`] (DIAG/INDEX),
//! used by the TMC2209 stall-detection and INDEX-alignment features.

use rp2350::gpio;
use rp2350::gpioc;
use rp2350_drivers::stepper::AxisPins;
use rp2350_drivers::stepper::tmc2209::TmcPins;

// ============================================================================
// SIO MMIO path (rp2350::gpio)
// ============================================================================

/// GPIO (SIO MMIO) STEP/DIR bridge for a single stepper axis.
///
/// # Example
///
/// ```ignore
/// type MotPins = rp2350_examples::common::stepper::GpioStepperPins<{PIN_STEP}, {PIN_DIR}>;
/// static AXIS: AxisState<MotPins> = AxisState::new(...);
/// ```
pub struct GpioStepperPins<const STEP: u32, const DIR: u32>;

impl<const STEP: u32, const DIR: u32> AxisPins for GpioStepperPins<STEP, DIR> {
    #[inline]
    fn set_step(high: bool) {
        // SAFETY: pins are configured as SIO outputs by the example before
        // the axis state machine is started.
        unsafe { gpio::set(STEP, high) }
    }

    #[inline]
    fn set_dir(high: bool) {
        // SAFETY: see `set_step`.
        unsafe { gpio::set(DIR, high) }
    }
}

/// GPIO (SIO MMIO) STEP/DIR + DIAG/INDEX bridge for TMC2209 steppers.
///
/// # Example
///
/// ```ignore
/// type MotPins = rp2350_examples::common::stepper::GpioTmcPins<{PIN_STEP}, {PIN_DIR}, {PIN_DIAG}, {PIN_INDEX}>;
/// static AXIS: AxisState<MotPins> = AxisState::new(...);
/// ```
pub struct GpioTmcPins<const STEP: u32, const DIR: u32, const DIAG: u32, const INDEX: u32>;

impl<const STEP: u32, const DIR: u32, const DIAG: u32, const INDEX: u32> AxisPins
    for GpioTmcPins<STEP, DIR, DIAG, INDEX>
{
    #[inline]
    fn set_step(high: bool) {
        // SAFETY: pins are configured as SIO outputs by the example.
        unsafe { gpio::set(STEP, high) }
    }

    #[inline]
    fn set_dir(high: bool) {
        // SAFETY: see `set_step`.
        unsafe { gpio::set(DIR, high) }
    }
}

impl<const STEP: u32, const DIR: u32, const DIAG: u32, const INDEX: u32> TmcPins
    for GpioTmcPins<STEP, DIR, DIAG, INDEX>
{
    #[inline]
    fn diag() -> bool {
        // SAFETY: DIAG pin is configured as an input by the example.
        unsafe { gpio::get(DIAG) }
    }

    #[inline]
    fn index() -> bool {
        // SAFETY: INDEX pin is configured as an input by the example.
        unsafe { gpio::get(INDEX) }
    }
}

// ============================================================================
// GPIOC coprocessor path (rp2350::gpioc)
// ============================================================================

/// GPIOC (coprocessor) STEP/DIR bridge for a single stepper axis.
pub struct GpiocStepperPins<const STEP: u32, const DIR: u32>;

impl<const STEP: u32, const DIR: u32> AxisPins for GpiocStepperPins<STEP, DIR> {
    #[inline]
    fn set_step(high: bool) {
        // SAFETY: pins are configured as GPIOC outputs by the example.
        unsafe { gpioc::set(STEP, high) }
    }

    #[inline]
    fn set_dir(high: bool) {
        // SAFETY: see `set_step`.
        unsafe { gpioc::set(DIR, high) }
    }
}

/// GPIOC (coprocessor) STEP/DIR + DIAG/INDEX bridge for TMC2209 steppers.
pub struct GpiocTmcPins<const STEP: u32, const DIR: u32, const DIAG: u32, const INDEX: u32>;

impl<const STEP: u32, const DIR: u32, const DIAG: u32, const INDEX: u32> AxisPins
    for GpiocTmcPins<STEP, DIR, DIAG, INDEX>
{
    #[inline]
    fn set_step(high: bool) {
        // SAFETY: pins are configured as GPIOC outputs by the example.
        unsafe { gpioc::set(STEP, high) }
    }

    #[inline]
    fn set_dir(high: bool) {
        // SAFETY: see `set_step`.
        unsafe { gpioc::set(DIR, high) }
    }
}

impl<const STEP: u32, const DIR: u32, const DIAG: u32, const INDEX: u32> TmcPins
    for GpiocTmcPins<STEP, DIR, DIAG, INDEX>
{
    #[inline]
    fn diag() -> bool {
        // SAFETY: DIAG pin is configured as an input by the example.
        unsafe { gpioc::get(DIAG) }
    }

    #[inline]
    fn index() -> bool {
        // SAFETY: INDEX pin is configured as an input by the example.
        unsafe { gpioc::get(INDEX) }
    }
}
