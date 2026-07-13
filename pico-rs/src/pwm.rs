//! PWM (Pulse Width Modulation) HAL for RP2350
//!
//! This module provides a safe interface for the RP2350 PWM peripheral.
//! The RP2350 has PWM slices, each with two outputs (A and B).
//!
//! - **RP2350A**: 8 slices (0-7)
//! - **RP2350B**: 12 slices (0-11)
//!
//! # Features
//!
//! - 16-bit counter with configurable wrap value
//! - Fractional clock divider (8.4 format)
//! - Phase-correct or edge-aligned modes
//! - **Type-safe pin ownership**: Pins are consumed by PWM, preventing misuse
//! - **Output mode**: PWM waveform generation on A and/or B channels
//! - **Input mode**: Frequency/duty cycle measurement using B channel
//!
//! # Frequency And Duty Cycle
//!
//! A PWM slice is controlled by three pieces of state:
//!
//! - The PWM input clock, usually `clocks.sys_freq()`
//! - The fractional divider `DIV = div_int + div_frac / 16`
//! - The counter wrap value `TOP`
//!
//! The hardware always works in terms of a counter. The divider first slows the
//! PWM input clock down to a counter clock, and then the counter period is set
//! by `TOP`.
//!
//! ```text
//! counter_clock = f_sys / DIV
//! pwm_period_steps = TOP + 1          (edge-aligned)
//! pwm_period_steps = TOP * 2          (phase-correct, in this HAL's model)
//! ```
//!
//! In normal edge-aligned mode the counter runs from `0` to `TOP`, so one PWM
//! period contains `TOP + 1` counter steps:
//!
//! ```text
//! f_pwm = f_sys / (DIV * (TOP + 1))
//! ```
//!
//! In phase-correct mode the counter counts up to `TOP` and then back down,
//! so one PWM period contains `2 * TOP` counter steps in this HAL's model:
//!
//! ```text
//! f_pwm = f_sys / (DIV * TOP * 2)
//! ```
//!
//! Channel A and channel B each have a compare register (`CC`). The output is
//! HIGH while `counter < CC`, so in edge-aligned mode the duty cycle is most
//! usefully understood as:
//!
//! ```text
//! duty ~= CC / (TOP + 1)
//! ```
//!
//! This is why a true 100% duty cycle needs `CC = TOP + 1`. Because the
//! compare register is only 16 bits wide, avoid `TOP = 0xffff` if you need a
//! representable full-scale duty cycle. The default reset value used by this
//! HAL is `0xfffe`, so `CC = 0xffff` still means fully on.
//!
//! In other words:
//!
//! - `CC = 0` means always LOW
//! - `CC = (TOP + 1) / 2` is approximately 50%
//! - `CC = TOP` is slightly below 100%
//! - `CC = TOP + 1` is true 100%, if it fits in 16 bits
//!
//! # Divider Quantization
//!
//! The divider register is stored as an 8.4 fixed-point value, so only steps of
//! `1/16` are representable. This means a requested frequency is usually not
//! programmed exactly unless the math divides cleanly.
//!
//! The helper methods and utility functions in this module therefore work in two
//! stages:
//!
//! 1. Compute the nearest representable divider allowed by the hardware format.
//! 2. Recompute the actual PWM frequency from that quantized divider.
//!
//! This is why [`configure_frequency()`](Slice::configure_frequency) and
//! [`configure_frequency_with_wrap()`](Slice::configure_frequency_with_wrap)
//! return the realized frequency instead of pretending the request was exact.
//!
//! When the requested combination of `sys_clock`, `target_freq`, and `TOP`
//! would require a divider smaller than the hardware minimum, the divider is
//! clamped to `1.0`. When it would require a divider larger than the hardware
//! maximum, it is clamped to `255 + 15/16`.
//!
//! # Choosing Parameters
//!
//! In practice PWM setup usually follows this order:
//!
//! 1. Pick `TOP` based on the duty-cycle resolution you want.
//! 2. Pick `DIV` so the resulting period matches your target frequency.
//! 3. Pick `CC` values for the duty cycle on channel A and/or B.
//!
//! A larger `TOP` gives finer duty-cycle resolution, but also lowers the PWM
//! frequency unless you reduce the divider. A smaller `TOP` raises the
//! frequency, but you lose duty-cycle granularity.
//!
//! A good mental model is:
//!
//! - `TOP` chooses how many duty steps you want.
//! - `DIV` chooses how fast the counter walks through those steps.
//! - `CC` chooses how many of those steps are HIGH.
//!
//! Some common tradeoffs:
//!
//! - LED dimming usually benefits from a larger `TOP`, because finer duty steps
//!   are visible as smoother brightness changes.
//! - Motor and fan control often prefer higher PWM frequencies, so `TOP` may
//!   need to be smaller to keep `DIV` near 1.
//! - Servo control benefits from a long period and a large `TOP`, because pulse
//!   width accuracy matters more than carrier frequency.
//!
//! This module exposes both layers:
//!
//! - Low-level register-style methods like [`set_wrap()`](Slice::set_wrap),
//!   [`set_div()`](Slice::set_div), and [`set_duty_a()`](Slice::set_duty_a)
//! - Helper methods like
//!   [`configure_frequency_with_wrap()`](Slice::configure_frequency_with_wrap)
//!   and [`get_frequency()`](Slice::get_frequency)
//!
//! The helper methods do not hide the hardware model. They simply calculate the
//! closest representable divider for a requested frequency and then return the
//! actual frequency that was programmed.
//!
//! # Parameter Meanings
//!
//! - `sys_clock`: The PWM input clock in Hz, usually `clocks.sys_freq()`.
//! - `div_int`: Integer part of the 8.4 divider.
//! - `div_frac`: Fractional part of the divider, in sixteenths.
//! - `wrap` / `TOP`: Counter limit that defines the PWM period.
//! - `CC`: Compare value that determines how long the output stays HIGH.
//!
//! The names used in this HAL map directly to the hardware registers:
//!
//! - `DIV` register = `div_int` + `div_frac`
//! - `TOP` register = `wrap`
//! - `CC_A` / `CC_B` fields = duty value for channel A / B
//!
//! If you see application notes or other HALs using the terms `period`, `arr`,
//! or `compare`, they correspond roughly to `TOP` and `CC` here.
//!
//! # Shared Slice Resources
//!
//! A and B are not two independent PWM timers. They share the same slice-level
//! counter, divider, wrap value, phase-correct setting, and enable bit.
//!
//! This means:
//!
//! - Channel A and channel B on the same slice always have the same base
//!   frequency.
//! - They can use different compare values, so their duty cycles can differ.
//! - Input mode on channel B reuses the same slice counter and divider logic.
//!
//! For example, if you are driving a 4-wire PC fan, a practical arrangement is
//! usually:
//!
//! - One slice in output mode to generate the control PWM signal
//! - A different slice in input mode to count tach pulses
//!
//! Trying to use one slice as both an ordinary free-running PWM generator and an
//! independent tach counter is usually the wrong model, because both uses need
//! control of the same counter.
//!
//! # Input Mode
//!
//! Input mode uses channel B as an external signal source for the divider:
//!
//! - [`InputMode::RisingEdge`] counts rising edges on the B pin.
//! - [`InputMode::FallingEdge`] counts falling edges on the B pin.
//! - [`InputMode::GatedByB`] counts while the B pin is HIGH.
//!
//! This is useful for frequency measurement, pulse counting, and gated duty
//! measurements. The helper functions [`calc_input_frequency`] and
//! [`calc_duty_cycle`] convert the raw counter value back into application-level
//! units.
//!
//! The three input modes are meant for slightly different jobs:
//!
//! - `RisingEdge`: best for pulse-per-period signals such as tach outputs.
//! - `FallingEdge`: same idea, when the falling edge is the better reference.
//! - `GatedByB`: best when you want to measure how long a signal stayed HIGH
//!   during a known gate interval.
//!
//! Input mode is counting-based measurement, not edge timestamp capture. If you
//! need per-edge timestamps or very fine pulse-width capture with independent
//! start/stop edges, a timer/capture peripheral would be a better match. PWM
//! input mode is excellent when you want a lightweight hardware counter.
//!
//! ## What The Counter Means In Each Input Mode
//!
//! The most important thing to understand is that input mode does not change the
//! meaning of the slice counter into a generic "measurement register". It only
//! changes what causes that counter to advance.
//!
//! - In [`InputMode::RisingEdge`], each rising edge increments the counter by 1.
//!   After a gate interval, the counter value is an edge count.
//! - In [`InputMode::FallingEdge`], each falling edge increments the counter by
//!   1. After a gate interval, the counter value is also an edge count.
//! - In [`InputMode::GatedByB`], the counter advances at the divided PWM clock
//!   rate, but only while the B input is HIGH. After a gate interval, the
//!   counter value is a HIGH-time count, not a period count.
//!
//! That difference is why edge-counting results are normally converted with
//! [`calc_input_frequency`], while gated results are normally converted with
//! [`calc_duty_cycle`] or equivalent application-specific math.
//!
//! ## Measurement Resolution And Gate Time
//!
//! Input-mode accuracy is set by the same two ideas that govern most counters:
//!
//! - How long you count for
//! - How fine the counter clock is
//!
//! For edge counting:
//!
//! - A short gate time gives faster updates but fewer counted edges, so the
//!   result is coarser and more sensitive to jitter.
//! - A long gate time gives better averaging and finer frequency resolution,
//!   but slower updates and a higher chance of counter overflow.
//!
//! For gated duty measurement:
//!
//! - A smaller divider gives a faster counter clock and finer time resolution.
//! - A longer gate time averages more cycles but increases the chance that the
//!   16-bit counter wraps.
//!
//! In practice, pick the shortest gate time that still gives a stable result,
//! then verify that the counter does not overflow in the worst case.
//!
//! ## Overflow Behavior
//!
//! The PWM input counter is still only 16 bits wide. If the measurement runs
//! too long, or the input is too fast, the counter can wrap.
//!
//! This matters differently by mode:
//!
//! - In edge-counting modes, overflow usually means the measured frequency is
//!   too high for the chosen gate interval.
//! - In gated mode, overflow usually means the product of counter clock and HIGH
//!   time was too large for the 16-bit counter.
//!
//! Use [`overflow_occurred()`](Slice::overflow_occurred) or
//! [`did_wrap()`](Slice::did_wrap) to detect this condition, and reduce gate
//! time or divider when necessary.
//!
//! ## Using Output A Together With Input B
//!
//! [`output_a_input()`](Slice::output_a_input) is useful when channel A should
//! still drive a waveform while channel B is used as an external input source.
//! However, this does not create two independent timing domains. Output and
//! measurement still share one slice counter and divider mode.
//!
//! This is a good fit when the output and the measurement intentionally belong
//! to the same slice-level timing scheme. It is not a substitute for a separate
//! free-running PWM output plus an independent capture timer.
//!
//! ## Practical Input-Mode Recipes
//!
//! - Fan tachometer: use `RisingEdge` or `FallingEdge`, choose a gate interval
//!   long enough to count several pulses, then convert with
//!   [`calc_input_frequency`].
//! - External PWM duty inspection: use `GatedByB`, set a fast divider such as
//!   `1.0`, measure over a known interval, then convert with
//!   [`calc_duty_cycle`].
//! - Pulse accumulation: keep the slice enabled in edge-counting mode and poll
//!   [`get_counter()`](Slice::get_counter), clearing wrap flags as needed.
//!
//! # Configuration Workflow
//!
//! The recommended workflow for ordinary PWM output is:
//!
//! 1. Create a slice with [`output_a()`](Slice::output_a) or
//!    [`output_ab()`](Slice::output_ab).
//! 2. Choose `TOP` based on duty resolution.
//! 3. Call [`configure_frequency_with_wrap()`](Slice::configure_frequency_with_wrap)
//!    or manually set `DIV`.
//! 4. Program one or both compare values with [`set_duty_a()`](Slice::set_duty_a),
//!    [`set_duty_b()`](Slice::set_duty_b), or [`set_duty()`](Slice::set_duty).
//! 5. Call [`enable()`](Slice::enable).
//!
//! The recommended workflow for measurement is:
//!
//! 1. Create a slice with [`input()`](Slice::input) or
//!    [`output_a_input()`](Slice::output_a_input).
//! 2. Select an input mode that matches the signal you want to count.
//! 3. Call [`start_measurement()`](Slice::start_measurement).
//! 4. Wait for a known gate interval in software.
//! 5. Call [`stop_measurement()`](Slice::stop_measurement) and convert the
//!    returned count with [`calc_input_frequency`] or [`calc_duty_cycle`].
//!
//! # Worked Examples
//!
//! The following combinations are commonly useful with a 150 MHz PWM clock:
//!
//! | Use case | Target frequency | TOP | Divider | Notes |
//! |----------|------------------|-----|---------|-------|
//! | LED dimming | 1 kHz | 999 | 150.0 | 1000 duty steps |
//! | Servo | 50 Hz | 65217 | about 46.0 | Long period, fine pulse-width steps |
//! | Motor PWM | 20 kHz | 7499 | 1.0 | Good high-frequency control example |
//! | 4-wire fan control | 25 kHz | 5999 | 1.0 | Common PC fan control frequency |
//!
//! For the 1 kHz LED case:
//!
//! ```text
//! f_pwm = 150_000_000 / (150 * (999 + 1)) = 1_000 Hz
//! CC = 500 => duty ~= 500 / 1000 = 50%
//! ```
//!
//! For the 25 kHz fan-control case:
//!
//! ```text
//! f_pwm = 150_000_000 / (1 * (5999 + 1)) = 25_000 Hz
//! ```
//!
//! These examples are exactly the kind of configuration that
//! [`configure_frequency_with_wrap()`](Slice::configure_frequency_with_wrap)
//! is meant to simplify.
//!
//! # API Conventions In This HAL
//!
//! The builder methods such as [`output_a()`](Slice::output_a) and
//! [`input()`](Slice::input) intentionally do not take a system clock argument.
//! Their job is only to bind pins, choose the slice mode, and initialize the
//! registers into a safe default state.
//!
//! Frequency calculation is handled separately because the clock alone is not
//! enough to decide a useful configuration. A real frequency choice also needs
//! either a target period or a target resolution.
//!
//! # Notes And Caveats
//!
//! - A larger `TOP` improves duty resolution but reduces the maximum reachable
//!   frequency.
//! - A smaller `TOP` makes higher frequencies possible but reduces duty granularity.
//! - `calc_duty_cycle` currently accepts only the integer divider input used by
//!   the measurement setup. If your measurement model depends on a fractional
//!   divider, compute the timing from the raw count yourself.
//! - `calc_input_frequency` and `calc_duty_cycle` expect a non-zero gate time.
//! - The helper functions compute the actual frequency after divider quantization;
//!   if exactness matters, always inspect the returned value.
//!
//! # GPIO Mapping
//!
//! Each GPIO can be mapped to a PWM channel:
//!
//! ```text
//! ┌─────────┬─────────┬─────────┬─────────┐
//! │  Slice  │ Channel │  GPIO   │  GPIO   │
//! │         │         │ (0-29)  │ (32-47) │
//! ├─────────┼─────────┼─────────┼─────────┤
//! │    0    │    A    │  0, 16  │   32    │
//! │    0    │    B    │  1, 17  │   33    │
//! │    1    │    A    │  2, 18  │   34    │
//! │    1    │    B    │  3, 19  │   35    │
//! │    2    │    A    │  4, 20  │   36    │
//! │    2    │    B    │  5, 21  │   37    │
//! │   ...   │   ...   │   ...   │   ...   │
//! │    7    │    A    │ 14, 30  │   46    │
//! │    7    │    B    │ 15, 31  │   47    │
//! └─────────┴─────────┴─────────┴─────────┘
//!
//! PWM slice = (gpio_num / 2) % 8 (RP2350A)
//! PWM slice = (gpio_num / 2) % 12 (RP2350B)
//! PWM channel = gpio_num % 2 (0 = A, 1 = B)
//! ```
//!
//! # Architecture
//!
//! ```text
//! ┌──────────────────────────────────────────────────────────────────┐
//! │                     PWM Ownership Model                          │
//! ├──────────────────────────────────────────────────────────────────┤
//! │                                                                  │
//! │  Peripherals::take()                                             │
//! │        │                                                         │
//! │        └── PWM ──► Pwm::init(pwm)                                 │
//! │                          │                                       │
//! │                          ├── pwm.slice::<0>()                    │
//! │                          │        ──► Slice<0>                   │
//! │                          │        .output_a(pin)                 │
//! │                          │              └──► Slice<0, A, NoPin>  │
//! │                          │                                       │
//! │                          ├── pwm.slice::<1>()                    │
//! │                          │        ──► Slice<1>                   │
//! │                          │        .output_ab(pin_a, pin_b)       │
//! │                          │              └──► Slice<1, A, B>      │
//! │                          │                                       │
//! │                          └── ...                                 │
//! │                                                                  │
//! └──────────────────────────────────────────────────────────────────┘
//! ```
//!
//! # Example (Output Mode with Pin Ownership)
//!
//! ```ignore
//! use pico_rs::gpio::Pins;
//! use pico_rs::pwm::Pwm;
//! use pico_rs::Peripherals;
//!
//! let dp = Peripherals::take().unwrap();
//! let pins = Pins::init(dp.IO_BANK0, dp.PADS_BANK0);
//!
//! // Create PWM controller (owns the PWM peripheral)
//! let pwm = Pwm::init(dp.PWM);
//!
//! // Create slice 0 with output on channel A
//! let slice0 = pwm.slice::<0>().output_a(pins.pin::<0>());
//!
//! // Configure for 1 kHz PWM with 1000 duty steps.
//! let actual_hz = slice0.configure_frequency_with_wrap(150_000_000, 1_000, 999);
//! slice0.set_duty_a(500);  // 50% duty cycle
//! slice0.enable();
//!
//! // Create another slice
//! let slice1 = pwm.slice::<1>().output_ab(pins.pin::<2>(), pins.pin::<3>());
//! let actual_hz_1 = slice1.configure_frequency_with_wrap(150_000_000, 20_000, 7_499);
//! slice1.set_duty(1_875, 5_625);  // 25% on A, 75% on B
//! slice1.enable();
//! ```
//!
//! # Example (Input Mode)
//!
//! ```ignore
//! use pico_rs::pwm::{Pwm, InputMode};
//!
//! let pwm = Pwm::init(dp.PWM);
//!
//! // Initialize PWM slice 0 for input measurement
//! let slice0 = pwm.slice::<0>().input(pins.pin::<1>(), InputMode::RisingEdge);
//!
//! // Measure frequency
//! slice0.start_measurement();
//! timer.delay_ms(100);  // 100ms gate time
//! let edges = slice0.stop_measurement();
//! let freq_hz = calc_input_frequency(edges, 100_000);
//! ```

use paste::paste;

use crate::gpio::pins::{NoPin, PwmAPin, PwmBPin};
use crate::reset::Resettable;

/// Number of PWM slices for RP2350A
#[cfg(not(feature = "rp2350b"))]
pub const SLICE_COUNT: usize = 8;

/// Number of PWM slices for RP2350B
#[cfg(feature = "rp2350b")]
pub const SLICE_COUNT: usize = 12;

/// PWM clock divider mode
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DivMode {
    /// Free-running counting at rate dictated by fractional divider
    FreeRunning = 0,
    /// Fractional divider operation is gated by the PWM B pin
    GatedByB = 1,
    /// Counter advances with each rising edge of the PWM B pin
    RisingEdgeB = 2,
    /// Counter advances with each falling edge of the PWM B pin
    FallingEdgeB = 3,
}

impl DivMode {
    /// Check if this mode uses the B pin as input
    #[inline]
    pub const fn is_input_mode(self) -> bool {
        matches!(
            self,
            DivMode::GatedByB | DivMode::RisingEdgeB | DivMode::FallingEdgeB
        )
    }

    /// Check if this mode counts edges
    #[inline]
    pub const fn is_edge_counting(self) -> bool {
        matches!(self, DivMode::RisingEdgeB | DivMode::FallingEdgeB)
    }
}

/// Input mode for PWM measurement
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum InputMode {
    /// Counter runs only while the B input is HIGH.
    ///
    /// This is useful when you want to measure the HIGH time of a pulse train
    /// across a known gate interval.
    GatedByB,
    /// Counter increments on each rising edge seen on the B input.
    ///
    /// This is the most common mode for tachometer or pulse-frequency
    /// measurement.
    RisingEdge,
    /// Counter increments on each falling edge seen on the B input.
    FallingEdge,
}

impl From<InputMode> for DivMode {
    fn from(mode: InputMode) -> Self {
        match mode {
            InputMode::GatedByB => DivMode::GatedByB,
            InputMode::RisingEdge => DivMode::RisingEdgeB,
            InputMode::FallingEdge => DivMode::FallingEdgeB,
        }
    }
}

/// PWM output mode
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mode {
    /// Output on channel A only.
    OutputA,
    /// Output on both channels A and B.
    OutputAB,
    /// Input measurement using channel B.
    Input,
    /// Output on A while using B as an input measurement source.
    OutputAWithInput,
}

// ============================================================================
// PWM Slice
// ============================================================================

/// PWM slice with type-safe pin ownership.
///
/// Obtained from [`Pwm::slice()`] in an unconfigured state
/// (`Slice<N, NoPin, NoPin>`). Call builder methods like
/// [`output_a()`](Slice::output_a) to configure the slice and bind
/// GPIO pins.
///
/// The const parameter `N` identifies which hardware PWM slice
/// (0–7 on RP2350A, 0–11 on RP2350B) this instance controls.
///
/// # ISR Usage
///
/// In interrupt context, a `Slice` can be created directly
/// via `unsafe { Slice::<N, NoPin, NoPin>::new() }` without going
/// through [`Pwm`].
pub struct Slice<const N: u8, A = NoPin, B = NoPin> {
    pwm: pico_rs_pac::PWM,
    _pin_a: A,
    _pin_b: B,
    mode: Mode,
}

impl<const N: u8, A, B> Slice<N, A, B> {
    /// Obtain a PWM slice handle without going through [`Pwm`].
    ///
    /// # Safety
    ///
    /// The caller must ensure the PWM peripheral has been initialised
    /// (i.e. taken out of reset) before using the returned handle.
    #[inline]
    pub const unsafe fn steal() -> crate::Steal<Slice<N, NoPin, NoPin>> {
        crate::Steal(Slice {
            pwm: unsafe { pico_rs_pac::PWM::steal() },
            _pin_a: NoPin,
            _pin_b: NoPin,
            mode: Mode::OutputA,
        })
    }
}

// ============================================================================
// PWM Global Controller
// ============================================================================

/// PWM peripheral controller.
///
/// Owns the PWM peripheral and provides [`Slice`] handles via the
/// [`slice()`](Pwm::slice) method.
///
/// # Example
///
/// ```ignore
/// let pwm = Pwm::init(dp.PWM);
///
/// // Get an unconfigured slice, then bind a pin
/// let slice0 = pwm.slice::<0>().output_a(pins.pin::<0>());
///
/// // Create slice 1
/// let slice1 = pwm.slice::<1>().output_ab(pins.pin::<2>(), pins.pin::<3>());
/// ```
pub struct Pwm {
    _pwm: pico_rs_pac::PWM,
}

impl Pwm {
    /// Initialize the PWM controller from the peripheral.
    ///
    /// This resets then unresets the PWM subsystem to guarantee clean
    /// register state. The peripheral is consumed to ensure only one
    /// `Pwm` instance exists.
    pub fn init(pwm: pico_rs_pac::PWM) -> Self {
        // Reset then unreset to guarantee clean register state
        <pico_rs_pac::PWM as Resettable>::init();
        Self { _pwm: pwm }
    }

    /// Obtain an unconfigured [`Slice`] for PWM slice `N`.
    ///
    /// Call builder methods like [`output_a()`](Slice::output_a)
    /// on the returned value to configure the slice and bind pins.
    ///
    /// # Slice Ranges
    ///
    /// - RP2350A: `N` must be 0–7
    /// - RP2350B (feature `rp2350b`): `N` must be 0–11
    ///
    /// # Panics
    ///
    /// Compile-time error if `N` is out of range.
    #[inline]
    pub fn slice<const N: u8>(&self) -> Slice<N> {
        #[cfg(not(feature = "rp2350b"))]
        const {
            assert!(N < 8, "PWM slice number must be 0-7 (RP2350A)")
        };
        #[cfg(feature = "rp2350b")]
        const {
            assert!(N < 12, "PWM slice number must be 0-11 (RP2350B)")
        };

        Slice {
            pwm: unsafe { pico_rs_pac::PWM::steal() },
            _pin_a: NoPin,
            _pin_b: NoPin,
            mode: Mode::OutputA,
        }
    }

    #[inline]
    fn regs(&self) -> &pico_rs_pac::pwm::RegisterBlock {
        &self._pwm
    }

    // ========================================================================
    // Global Enable/Disable
    // ========================================================================

    /// Enable multiple PWM slices simultaneously.
    ///
    /// # Arguments
    /// * `mask` - Bitmask of slices to enable (bit 0 = slice 0, etc.)
    #[inline]
    pub fn enable_mask(&self, mask: u16) {
        self.regs().en.set_bits(mask as u32);
    }

    /// Disable multiple PWM slices simultaneously.
    ///
    /// # Arguments
    /// * `mask` - Bitmask of slices to disable
    #[inline]
    pub fn disable_mask(&self, mask: u16) {
        self.regs().en.clr_bits(mask as u32);
    }

    /// Get enabled status of all slices.
    #[inline]
    pub fn enabled_mask(&self) -> u16 {
        (self.regs().en.read_bits() & 0xfff) as u16
    }

    // ========================================================================
    // Interrupt Control (IRQ0)
    // ========================================================================

    /// Get raw interrupt status (before masking).
    #[inline]
    pub fn interrupt_raw(&self) -> u16 {
        (self.regs().intr.read_bits() & 0xfff) as u16
    }

    /// Check if raw interrupt is pending for a slice.
    #[inline]
    pub fn is_interrupt_raw(&self, slice: u8) -> bool {
        (self.regs().intr.read_bits() & (1 << slice)) != 0
    }

    /// Clear interrupt for slices using mask.
    #[inline]
    pub fn clear_interrupt(&self, mask: u16) {
        self.regs().intr.write_bits(mask as u32);
    }

    /// Clear interrupt for a specific slice.
    #[inline]
    pub fn clear_slice_interrupt(&self, slice: u8) {
        self.regs().intr.write_bits(1 << slice);
    }

    /// Clear all PWM interrupts.
    #[inline]
    pub fn clear_all_interrupts(&self) {
        self.regs().intr.write_bits(0xfff);
    }

    /// Enable interrupt for slices using mask (IRQ0).
    #[inline]
    pub fn enable_interrupt(&self, mask: u16) {
        self.regs().irq0_inte.set_bits(mask as u32);
    }

    /// Disable interrupt for slices using mask (IRQ0).
    #[inline]
    pub fn disable_interrupt(&self, mask: u16) {
        self.regs().irq0_inte.clr_bits(mask as u32);
    }

    /// Enable interrupt for a specific slice (IRQ0).
    #[inline]
    pub fn enable_slice_interrupt(&self, slice: u8) {
        self.regs().irq0_inte.set_bits(1 << slice);
    }

    /// Disable interrupt for a specific slice (IRQ0).
    #[inline]
    pub fn disable_slice_interrupt(&self, slice: u8) {
        self.regs().irq0_inte.clr_bits(1 << slice);
    }

    /// Disable all interrupts (IRQ0).
    #[inline]
    pub fn disable_all_interrupts(&self) {
        self.regs().irq0_inte.clr_bits(0xfff);
    }

    /// Check if interrupt is enabled for a slice (IRQ0).
    #[inline]
    pub fn is_slice_interrupt_enabled(&self, slice: u8) -> bool {
        (self.regs().irq0_inte.read_bits() & (1 << slice)) != 0
    }

    /// Get interrupt enable mask (IRQ0).
    #[inline]
    pub fn interrupt_enabled_mask(&self) -> u16 {
        (self.regs().irq0_inte.read_bits() & 0xfff) as u16
    }

    /// Get interrupt status after masking (IRQ0).
    #[inline]
    pub fn interrupt_status(&self) -> u16 {
        (self.regs().irq0_ints.read_bits() & 0xfff) as u16
    }

    /// Check if interrupt is pending for a slice (masked, IRQ0).
    #[inline]
    pub fn is_slice_interrupt_pending(&self, slice: u8) -> bool {
        (self.regs().irq0_ints.read_bits() & (1 << slice)) != 0
    }

    /// Force interrupt for slices (for testing).
    #[inline]
    pub fn force_interrupt(&self, mask: u16) {
        self.regs().irq0_intf.set_bits(mask as u32);
    }

    /// Clear forced interrupt for slices.
    #[inline]
    pub fn clear_forced_interrupt(&self, mask: u16) {
        self.regs().irq0_intf.clr_bits(mask as u32);
    }

    // ========================================================================
    // Interrupt Control (IRQ1) - Secondary interrupt output
    // ========================================================================

    /// Enable interrupt for slices (IRQ1).
    #[inline]
    pub fn enable_interrupt_irq1(&self, mask: u16) {
        self.regs().irq1_inte.set_bits(mask as u32);
    }

    /// Disable interrupt for slices (IRQ1).
    #[inline]
    pub fn disable_interrupt_irq1(&self, mask: u16) {
        self.regs().irq1_inte.clr_bits(mask as u32);
    }

    /// Get interrupt status after masking (IRQ1).
    #[inline]
    pub fn interrupt_status_irq1(&self) -> u16 {
        (self.regs().irq1_ints.read_bits() & 0xfff) as u16
    }

    /// Get reference to the underlying register block.
    ///
    /// # Safety
    /// Direct register access can bypass the HAL's safety guarantees.
    #[inline]
    pub unsafe fn registers(&self) -> &pico_rs_pac::pwm::RegisterBlock {
        self.regs()
    }
}

// ============================================================================
// Macro for generating PWM slice implementations
// ============================================================================

macro_rules! impl_pwm_slice {
    ($slice_num:literal) => {
        paste! {
            // ================================================================
            // Slice<N, NoPin, NoPin> builder methods
            // ================================================================

            impl Slice<$slice_num, NoPin, NoPin> {
                #[doc = concat!("Create PWM slice ", $slice_num, " with output on channel A only.")]
                ///
                /// Consumes this unconfigured slice. The pin is configured
                /// for PWM automatically.
                ///
                /// # Arguments
                ///
                /// * `pin_a` - Channel A pin (must be for this slice)
                pub fn output_a<A: PwmAPin>(self, pin_a: A) -> Slice<$slice_num, A, NoPin> {
                    // Verify the pin is for this slice
                    assert_eq!(A::PWM_SLICE, $slice_num, "Pin must be for PWM slice {}", $slice_num);

                    // Configure pin for PWM function
                    PwmAPin::configure_pwm_a(&pin_a);

                    // Initialize via direct register access (avoids const-generic
                    // method resolution ambiguity)
                    let pwm = unsafe { pico_rs_pac::PWM::steal() };
                    pwm.[<ch $slice_num _csr>].clr_en();
                    pwm.[<ch $slice_num _top>].write_bits(0xfffe);
                    pwm.[<ch $slice_num _div>].write_bits(1u32 << 4);
                    pwm.[<ch $slice_num _cc>].write_a(0);
                    pwm.intr.write_bits(1 << $slice_num);
                    pwm.[<ch $slice_num _csr>].write_divmode(DivMode::FreeRunning as u32);

                    Slice {
                        pwm,
                        _pin_a: pin_a,
                        _pin_b: NoPin,
                        mode: Mode::OutputA,
                    }
                }

                #[doc = concat!("Create PWM slice ", $slice_num, " with output on both channels A and B.")]
                ///
                /// Consumes this unconfigured slice. Both pins are configured
                /// for PWM automatically.
                ///
                /// # Arguments
                ///
                /// * `pin_a` - Channel A pin
                /// * `pin_b` - Channel B pin
                pub fn output_ab<A: PwmAPin, B: PwmBPin>(self, pin_a: A, pin_b: B) -> Slice<$slice_num, A, B> {
                    // Verify pins are for this slice
                    assert_eq!(A::PWM_SLICE, $slice_num, "Pin A must be for PWM slice {}", $slice_num);
                    assert_eq!(B::PWM_SLICE, $slice_num, "Pin B must be for PWM slice {}", $slice_num);

                    // Configure pins for PWM function
                    PwmAPin::configure_pwm_a(&pin_a);
                    PwmBPin::configure_pwm_b(&pin_b);

                    // Initialize via direct register access
                    let pwm = unsafe { pico_rs_pac::PWM::steal() };
                    pwm.[<ch $slice_num _csr>].clr_en();
                    pwm.[<ch $slice_num _top>].write_bits(0xfffe);
                    pwm.[<ch $slice_num _div>].write_bits(1u32 << 4);
                    pwm.[<ch $slice_num _cc>].write_a(0);
                    pwm.[<ch $slice_num _cc>].write_b(0);
                    pwm.intr.write_bits(1 << $slice_num);
                    pwm.[<ch $slice_num _csr>].write_divmode(DivMode::FreeRunning as u32);

                    Slice {
                        pwm,
                        _pin_a: pin_a,
                        _pin_b: pin_b,
                        mode: Mode::OutputAB,
                    }
                }

                #[doc = concat!("Create PWM slice ", $slice_num, " for input measurement using channel B.")]
                ///
                /// Consumes this unconfigured slice. The pin is configured
                /// for PWM automatically.
                ///
                /// The slice starts in measurement mode, with channel B acting as the
                /// external source that drives the divider logic according to `mode`.
                /// No output waveform is generated on channel A.
                ///
                /// The counter is initialized to zero, the divider is initialized to `1.0`,
                /// the wrap value is initialized to `0xfffe`, and the selected divider mode
                /// is programmed immediately.
                ///
                /// Typical use cases:
                ///
                /// - `InputMode::RisingEdge` for tach or pulse-frequency measurement
                /// - `InputMode::FallingEdge` when the falling edge is the natural reference
                /// - `InputMode::GatedByB` for HIGH-time or duty-style measurement
                ///
                /// # Arguments
                ///
                /// * `pin_b` - Channel B pin for input
                /// * `mode` - Input mode (GatedByB, RisingEdge, FallingEdge)
                pub fn input<B: PwmBPin>(self, pin_b: B, mode: InputMode) -> Slice<$slice_num, NoPin, B> {
                    // Verify pin is for this slice
                    assert_eq!(B::PWM_SLICE, $slice_num, "Pin B must be for PWM slice {}", $slice_num);

                    // Configure pin for PWM function
                    PwmBPin::configure_pwm_b(&pin_b);

                    // Initialize for input via direct register access
                    let pwm = unsafe { pico_rs_pac::PWM::steal() };
                    pwm.[<ch $slice_num _csr>].clr_en();
                    pwm.[<ch $slice_num _top>].write_bits(0xfffe);
                    pwm.[<ch $slice_num _div>].write_bits(1u32 << 4);
                    pwm.[<ch $slice_num _ctr>].write_bits(0);
                    pwm.intr.write_bits(1 << $slice_num);
                    pwm.[<ch $slice_num _csr>].write_divmode(DivMode::from(mode) as u32);

                    Slice {
                        pwm,
                        _pin_a: NoPin,
                        _pin_b: pin_b,
                        mode: Mode::Input,
                    }
                }

                #[doc = concat!("Create PWM slice ", $slice_num, " with output on A and input on B.")]
                ///
                /// Consumes this unconfigured slice. Both pins are configured
                /// automatically.
                ///
                /// This mode combines an A-channel PWM output with B-channel-driven counter
                /// control. It is useful only when both behaviors are intentionally tied to
                /// the same slice-level counter and divider mode.
                ///
                /// Because channel A output and channel B input still share one slice,
                /// this is not equivalent to “one independent PWM generator plus one
                /// independent measurement channel”.
                ///
                /// # Arguments
                ///
                /// * `pin_a` - Channel A pin for output
                /// * `pin_b` - Channel B pin for input measurement
                /// * `mode` - Input mode for measurement
                pub fn output_a_input<A: PwmAPin, B: PwmBPin>(
                    self,
                    pin_a: A,
                    pin_b: B,
                    mode: InputMode,
                ) -> Slice<$slice_num, A, B> {
                    // Verify pins are for this slice
                    assert_eq!(A::PWM_SLICE, $slice_num, "Pin A must be for PWM slice {}", $slice_num);
                    assert_eq!(B::PWM_SLICE, $slice_num, "Pin B must be for PWM slice {}", $slice_num);

                    // Configure pins
                    PwmAPin::configure_pwm_a(&pin_a);
                    PwmBPin::configure_pwm_b(&pin_b);

                    // Initialize via direct register access
                    let pwm = unsafe { pico_rs_pac::PWM::steal() };
                    pwm.[<ch $slice_num _csr>].clr_en();
                    pwm.[<ch $slice_num _top>].write_bits(0xfffe);
                    pwm.[<ch $slice_num _div>].write_bits(1u32 << 4);
                    pwm.[<ch $slice_num _cc>].write_a(0);
                    pwm.[<ch $slice_num _ctr>].write_bits(0);
                    pwm.intr.write_bits(1 << $slice_num);
                    pwm.[<ch $slice_num _csr>].write_divmode(DivMode::from(mode) as u32);

                    Slice {
                        pwm,
                        _pin_a: pin_a,
                        _pin_b: pin_b,
                        mode: Mode::OutputAWithInput,
                    }
                }
            }

            // ================================================================
            // Slice<N, A, B> register-access methods
            // ================================================================

            impl<A, B> Slice<$slice_num, A, B> {
                /// PWM slice number.
                pub const SLICE: u8 = $slice_num;

                // ============================================================
                // Private register accessors
                // ============================================================

                #[inline]
                fn pwm(&self) -> &pico_rs_pac::pwm::RegisterBlock {
                    &self.pwm
                }

                #[inline]
                fn csr(&self) -> &pico_rs_pac::pwm::[<Ch $slice_num Csr>] {
                    &self.pwm().[<ch $slice_num _csr>]
                }

                #[inline]
                fn div(&self) -> &pico_rs_pac::pwm::[<Ch $slice_num Div>] {
                    &self.pwm().[<ch $slice_num _div>]
                }

                #[inline]
                fn ctr(&self) -> &pico_rs_pac::pwm::[<Ch $slice_num Ctr>] {
                    &self.pwm().[<ch $slice_num _ctr>]
                }

                #[inline]
                fn cc(&self) -> &pico_rs_pac::pwm::[<Ch $slice_num Cc>] {
                    &self.pwm().[<ch $slice_num _cc>]
                }

                #[inline]
                fn top(&self) -> &pico_rs_pac::pwm::[<Ch $slice_num Top>] {
                    &self.pwm().[<ch $slice_num _top>]
                }

                // ============================================================
                // Enable/Disable
                // ============================================================

                /// Enable this PWM slice.
                #[inline]
                pub fn enable(&self) {
                    self.csr().set_en();
                }

                /// Disable this PWM slice.
                #[inline]
                pub fn disable(&self) {
                    self.csr().clr_en();
                }

                /// Check if this slice is enabled.
                #[inline]
                pub fn is_enabled(&self) -> bool {
                    self.csr().en()
                }

                // ============================================================
                // Configuration
                // ============================================================

                /// Set phase-correct mode.
                ///
                /// In phase-correct mode, the counter counts up to TOP then back down,
                /// giving symmetric PWM waveforms.
                #[inline]
                pub fn set_phase_correct(&self, enable: bool) {
                    if enable {
                        self.csr().set_ph_correct();
                    } else {
                        self.csr().clr_ph_correct();
                    }
                }

                /// Check if phase-correct mode is enabled.
                #[inline]
                pub fn is_phase_correct(&self) -> bool {
                    self.csr().ph_correct()
                }

                /// Set the divider mode.
                ///
                /// In ordinary output use this is usually left as
                /// [`DivMode::FreeRunning`]. In input-oriented setups it determines what
                /// event on channel B advances the slice counter.
                ///
                /// Changing this mode changes the meaning of subsequent counter values, so
                /// callers should treat any in-flight measurement as invalid after switching.
                #[inline]
                pub fn set_div_mode(&self, mode: DivMode) {
                    self.csr().write_divmode(mode as u32);
                }

                /// Get the current divider mode.
                ///
                /// This is useful when a generic helper wants to distinguish between
                /// free-running output mode and one of the B-input-driven counting modes.
                #[inline]
                pub fn get_div_mode(&self) -> DivMode {
                    match self.csr().divmode() {
                        0 => DivMode::FreeRunning,
                        1 => DivMode::GatedByB,
                        2 => DivMode::RisingEdgeB,
                        3 => DivMode::FallingEdgeB,
                        _ => unreachable!(),
                    }
                }

                // ============================================================
                // Clock Divider
                // ============================================================

                /// Set the integer part of the clock divider (1-255).
                ///
                /// The PWM counter advances at: sys_clk / (int + frac/16)
                #[inline]
                pub fn set_div_int(&self, div: u8) {
                    self.div().write_int(div as u32);
                }

                /// Set the fractional part of the clock divider (0-15).
                #[inline]
                pub fn set_div_frac(&self, frac: u8) {
                    self.div().write_frac(frac as u32);
                }

                /// Set both integer and fractional divider parts.
                ///
                /// The effective divider is `int_part + frac_part / 16`.
                /// For example, `set_div(3, 8)` configures a divider of `3.5`.
                ///
                /// This directly affects the counter clock:
                ///
                /// ```text
                /// counter_clock = sys_clock / (int_part + frac_part / 16)
                /// ```
                #[inline]
                pub fn set_div(&self, int_part: u8, frac_part: u8) {
                    self.div().write_bits(((int_part as u32) << 4) | ((frac_part as u32) & 0xf));
                }

                /// Get the current divider value (8.4 fixed point format).
                #[inline]
                pub fn get_div(&self) -> (u8, u8) {
                    let int_part = self.div().int() as u8;
                    let frac_part = self.div().frac() as u8;
                    (int_part, frac_part)
                }

                // ============================================================
                // Counter and Wrap
                // ============================================================

                /// Set the wrap (TOP) value.
                ///
                /// The counter wraps (or reverses in phase-correct mode) when it reaches
                /// this value. Maximum is 65535.
                ///
                /// In edge-aligned mode, one PWM period contains `TOP + 1` counter steps.
                /// This means `TOP` is really a period-resolution control as much as it is a
                /// frequency control.
                ///
                /// Typical examples:
                ///
                /// - `TOP = 999` gives 1000 duty steps
                /// - `TOP = 7499` works well for 20 kHz PWM at 150 MHz with divider 1
                /// - `TOP = 65217` is useful for 50 Hz servo pulses with fine resolution
                ///
                /// If you need a true 100% duty cycle with [`set_duty_a()`](Self::set_duty_a)
                /// or [`set_duty_b()`](Self::set_duty_b), avoid `0xffff`: the compare value
                /// would need to be `TOP + 1`, which does not fit in the 16-bit CC register.
                #[inline]
                pub fn set_wrap(&self, wrap: u16) {
                    self.top().write_bits(wrap as u32);
                }

                /// Get the current wrap value.
                #[inline]
                pub fn get_wrap(&self) -> u16 {
                    self.top().read_bits() as u16
                }

                /// Set the counter value directly.
                #[inline]
                pub fn set_counter(&self, value: u16) {
                    self.ctr().write_bits(value as u32);
                }

                /// Get the current counter value.
                #[inline]
                pub fn get_counter(&self) -> u16 {
                    self.ctr().read_bits() as u16
                }

                /// Reset counter to zero.
                #[inline]
                pub fn reset_counter(&self) {
                    self.ctr().write_bits(0);
                }

                // ============================================================
                // Phase Control
                // ============================================================

                /// Advance phase by one count.
                ///
                /// Self-clearing; write 1 to advance phase while running.
                #[inline]
                pub fn advance_phase(&self) {
                    self.csr().trigger_ph_adv();
                }

                /// Retard phase by one count.
                ///
                /// Self-clearing; write 1 to retard phase while running.
                #[inline]
                pub fn retard_phase(&self) {
                    self.csr().trigger_ph_ret();
                }

                // ============================================================
                // Channel A (Output)
                // ============================================================

                /// Set duty cycle for channel A.
                ///
                /// The output goes high when `counter < value`.
                ///
                /// In edge-aligned mode, a useful approximation is:
                ///
                /// ```text
                /// duty ~= value / (TOP + 1)
                /// ```
                ///
                /// So with `TOP = 999`:
                ///
                /// - `value = 0`   => 0%
                /// - `value = 500` => about 50%
                /// - `value = 1000` => 100%, if representable
                #[inline]
                pub fn set_duty_a(&self, value: u16) {
                    self.cc().write_a(value as u32);
                }

                /// Get duty cycle for channel A.
                #[inline]
                pub fn get_duty_a(&self) -> u16 {
                    self.cc().a() as u16
                }

                /// Invert output A.
                #[inline]
                pub fn set_invert_a(&self, invert: bool) {
                    if invert {
                        self.csr().set_a_inv();
                    } else {
                        self.csr().clr_a_inv();
                    }
                }

                /// Check if output A is inverted.
                #[inline]
                pub fn is_invert_a(&self) -> bool {
                    self.csr().a_inv()
                }

                // ============================================================
                // Channel B (Output/Input)
                // ============================================================

                /// Set duty cycle for channel B.
                ///
                /// The output goes high when `counter < value`.
                ///
                /// Channel B shares the same counter, divider, and wrap value as channel A,
                /// so only the duty can differ between them.
                #[inline]
                pub fn set_duty_b(&self, value: u16) {
                    self.cc().write_b(value as u32);
                }

                /// Get duty cycle for channel B.
                #[inline]
                pub fn get_duty_b(&self) -> u16 {
                    self.cc().b() as u16
                }

                /// Invert output B.
                #[inline]
                pub fn set_invert_b(&self, invert: bool) {
                    if invert {
                        self.csr().set_b_inv();
                    } else {
                        self.csr().clr_b_inv();
                    }
                }

                /// Check if output B is inverted.
                #[inline]
                pub fn is_invert_b(&self) -> bool {
                    self.csr().b_inv()
                }

                /// Set duty cycle for both channels in one register write.
                ///
                /// This is useful when both outputs on the same slice should be updated
                /// together without a transient mismatch between separate writes.
                #[inline]
                pub fn set_duty(&self, a: u16, b: u16) {
                    self.cc().write_bits((a as u32) | ((b as u32) << 16));
                }

                // ============================================================
                // Interrupt Support
                // ============================================================

                /// Enable interrupt for this slice (IRQ0).
                #[inline]
                pub fn enable_interrupt(&self) {
                    self.pwm().irq0_inte.set_bits(1 << Self::SLICE);
                }

                /// Disable interrupt for this slice (IRQ0).
                #[inline]
                pub fn disable_interrupt(&self) {
                    self.pwm().irq0_inte.clr_bits(1 << Self::SLICE);
                }

                /// Check if interrupt is enabled for this slice (IRQ0).
                #[inline]
                pub fn is_interrupt_enabled(&self) -> bool {
                    (self.pwm().irq0_inte.read_bits() & (1 << Self::SLICE)) != 0
                }

                /// Check if raw interrupt is pending for this slice.
                #[inline]
                pub fn is_interrupt_raw(&self) -> bool {
                    (self.pwm().intr.read_bits() & (1 << Self::SLICE)) != 0
                }

                /// Check if interrupt is pending for this slice (masked).
                #[inline]
                pub fn is_interrupt_pending(&self) -> bool {
                    (self.pwm().irq0_ints.read_bits() & (1 << Self::SLICE)) != 0
                }

                /// Clear interrupt for this slice.
                #[inline]
                pub fn clear_interrupt(&self) {
                    self.pwm().intr.write_bits(1 << Self::SLICE);
                }

                /// Force interrupt for this slice (for testing).
                #[inline]
                pub fn force_interrupt(&self) {
                    self.pwm().irq0_intf.set_bits(1 << Self::SLICE);
                }

                /// Clear forced interrupt for this slice.
                #[inline]
                pub fn clear_forced_interrupt(&self) {
                    self.pwm().irq0_intf.clr_bits(1 << Self::SLICE);
                }

                /// Check if a counter wrap occurred.
                #[inline]
                pub fn did_wrap(&self) -> bool {
                    self.is_interrupt_raw()
                }

                // ============================================================
                // Status and Mode
                // ============================================================

                /// Get the current mode.
                #[inline]
                pub fn mode(&self) -> Mode {
                    self.mode
                }

                /// Get the slice number.
                #[inline]
                pub const fn slice_num(&self) -> u8 {
                    Self::SLICE
                }

                // ============================================================
                // Input Mode Helpers
                // ============================================================

                /// Start a measurement.
                ///
                /// This resets the counter to zero, clears any pending wrap flag for this
                /// slice, and then enables the counter.
                ///
                /// The caller is expected to wait for a known gate interval in software and
                /// then call [`stop_measurement()`](Self::stop_measurement).
                #[inline]
                pub fn start_measurement(&self) {
                    self.reset_counter();
                    self.clear_interrupt();
                    self.enable();
                }

                /// Stop measurement and return the final counter value.
                ///
                /// The meaning of the returned count depends on the configured divider mode:
                ///
                /// - Rising/falling-edge modes return an edge count.
                /// - Gated mode returns the number of counter ticks accumulated while the
                ///   input was HIGH.
                #[inline]
                pub fn stop_measurement(&self) -> u16 {
                    self.disable();
                    self.get_counter()
                }

                /// Check if overflow occurred during measurement.
                ///
                /// A `true` result means the 16-bit counter wrapped at least once since the
                /// wrap flag was last cleared. The current count alone is then insufficient to
                /// reconstruct the full measurement result unless you also tracked wraps in
                /// software.
                #[inline]
                pub fn overflow_occurred(&self) -> bool {
                    self.is_interrupt_raw()
                }

                /// Release the pins.
                ///
                /// Returns a tuple of (channel A pin, channel B pin).
                pub fn release(self) -> (A, B) {
                    self.disable();
                    (self._pin_a, self._pin_b)
                }

                /// Get reference to the underlying register block.
                ///
                /// # Safety
                /// Direct register access can bypass the HAL's safety guarantees.
                #[inline]
                pub unsafe fn regs(&self) -> &pico_rs_pac::pwm::RegisterBlock {
                    self.pwm()
                }

                // ============================================================
                // DMA Support
                // ============================================================

                /// DREQ selection for PWM wrap event DMA pacing.
                ///
                /// The PWM DREQ fires once per counter wrap, allowing DMA
                /// to update duty cycle / TOP values at the PWM frequency.
                pub const WRAP_DREQ: crate::dma::TreqSel = crate::dma::TreqSel::[<PwmWrap $slice_num>];

                /// Get the CC (counter compare) register address for DMA.
                ///
                /// DMA can write 32-bit values here to update both channel A
                /// (low 16 bits) and channel B (high 16 bits) duty in one transfer.
                #[inline]
                pub fn cc_address(&self) -> u32 {
                    let pwm = self.pwm();
                    &pwm.[<ch $slice_num _cc>] as *const _ as u32
                }

                /// Get the TOP register address for DMA.
                ///
                /// DMA can write here to update the wrap value (period).
                #[inline]
                pub fn top_address(&self) -> u32 {
                    let pwm = self.pwm();
                    &pwm.[<ch $slice_num _top>] as *const _ as u32
                }

                // ============================================================
                // Helper Methods for Frequency Configuration
                // ============================================================

                /// Configure the divider for a target PWM frequency using the current wrap value.
                ///
                /// This helper keeps the current TOP value unchanged and derives the closest
                /// representable fractional divider for `target_freq`.
                ///
                /// It automatically accounts for the current phase-correct setting:
                /// in edge-aligned mode it uses [`calc_divider`], and in phase-correct mode it
                /// uses [`calc_divider_phase_correct`].
                ///
                /// Because the divider is stored in 8.4 fixed-point format, the realized
                /// frequency can differ slightly from the requested one. The actual programmed
                /// frequency is returned so callers can log or validate the result.
                ///
                /// # Arguments
                ///
                /// * `sys_clock` - PWM input clock in Hz, typically `clocks.sys_freq()`
                /// * `target_freq` - Desired PWM frequency in Hz. Must be non-zero.
                ///
                /// # Returns
                ///
                /// The actual frequency in Hz after divider quantization.
                ///
                /// # Panics
                ///
                /// Panics if `target_freq` is zero.
                /// Panics in phase-correct mode if the current wrap value is zero.
                ///
                /// # Example
                ///
                /// ```ignore
                /// let slice = pwm.slice::<0>().output_a(pins.pin::<0>());
                /// slice.set_wrap(999);
                /// let actual = slice.configure_frequency(150_000_000, 1_000);
                /// assert!(actual > 0);
                /// ```
                #[inline]
                pub fn configure_frequency(&self, sys_clock: u32, target_freq: u32) -> u32 {
                    self.configure_frequency_with_wrap(sys_clock, target_freq, self.get_wrap())
                }

                /// Configure wrap and divider together for a target PWM frequency.
                ///
                /// This is the most convenient high-level configuration method when you already
                /// know the period resolution you want. It writes TOP first, then computes and
                /// programs the closest divider for `target_freq`, and finally returns the actual
                /// realized frequency.
                ///
                /// If you need a true 100% duty cycle, prefer `wrap = 0xfffe` instead of
                /// `0xffff`, so the 16-bit compare register can still represent `TOP + 1`.
                ///
                /// Like [`configure_frequency`](Self::configure_frequency), this method respects
                /// the current phase-correct setting when choosing the divider formula.
                ///
                /// # Arguments
                ///
                /// * `sys_clock` - PWM input clock in Hz, typically `clocks.sys_freq()`
                /// * `target_freq` - Desired PWM frequency in Hz. Must be non-zero.
                /// * `wrap` - TOP value to program before calculating the divider.
                ///
                /// # Returns
                ///
                /// The actual frequency in Hz after programming TOP and DIV.
                ///
                /// # Panics
                ///
                /// Panics if `target_freq` is zero.
                /// Panics in phase-correct mode if `wrap` is zero.
                ///
                /// # Example
                ///
                /// ```ignore
                /// let slice = pwm.slice::<0>().output_a(pins.pin::<0>());
                /// let actual = slice.configure_frequency_with_wrap(150_000_000, 20_000, 7_499);
                /// assert!(actual > 0);
                /// ```
                #[inline]
                pub fn configure_frequency_with_wrap(&self, sys_clock: u32, target_freq: u32, wrap: u16) -> u32 {
                    assert!(target_freq != 0, "target PWM frequency must be non-zero");

                    self.set_wrap(wrap);

                    let (div_int, div_frac) = if self.is_phase_correct() {
                        calc_divider_phase_correct(sys_clock, target_freq, wrap)
                    } else {
                        calc_divider(sys_clock, target_freq, wrap)
                    };

                    self.set_div(div_int, div_frac);

                    if self.is_phase_correct() {
                        calc_phase_correct_frequency(sys_clock, div_int, div_frac, wrap)
                    } else {
                        calc_frequency(sys_clock, div_int, div_frac, wrap)
                    }
                }

                /// Get the currently configured PWM frequency in Hz.
                ///
                /// This method reads the live TOP and DIV registers and converts them back into a
                /// frequency using the current phase-correct setting. It is useful for diagnostics,
                /// logging, and verifying the result returned by
                /// [`configure_frequency`](Self::configure_frequency).
                ///
                /// # Arguments
                ///
                /// * `sys_clock` - PWM input clock in Hz, typically `clocks.sys_freq()`
                ///
                /// # Returns
                ///
                /// The effective PWM frequency in Hz for the current slice configuration.
                ///
                /// # Example
                ///
                /// ```ignore
                /// let slice = pwm.slice::<0>().output_a(pins.pin::<0>());
                /// slice.set_wrap(999);
                /// slice.set_div(150, 0);
                /// let freq = slice.get_frequency(150_000_000);
                /// assert_eq!(freq, 1_000);
                /// ```
                #[inline]
                pub fn get_frequency(&self, sys_clock: u32) -> u32 {
                    let (div_int, div_frac) = self.get_div();
                    let wrap = self.get_wrap();

                    if self.is_phase_correct() {
                        calc_phase_correct_frequency(sys_clock, div_int, div_frac, wrap)
                    } else {
                        calc_frequency(sys_clock, div_int, div_frac, wrap)
                    }
                }
            }
        }
    };
}

// ============================================================================
// Generate PWM slice implementations
// ============================================================================

// Slices 0-7 are available on both RP2350A and RP2350B
impl_pwm_slice!(0);
impl_pwm_slice!(1);
impl_pwm_slice!(2);
impl_pwm_slice!(3);
impl_pwm_slice!(4);
impl_pwm_slice!(5);
impl_pwm_slice!(6);
impl_pwm_slice!(7);

// Slices 8-11 are only available on RP2350B
#[cfg(feature = "rp2350b")]
impl_pwm_slice!(8);
#[cfg(feature = "rp2350b")]
impl_pwm_slice!(9);
#[cfg(feature = "rp2350b")]
impl_pwm_slice!(10);
#[cfg(feature = "rp2350b")]
impl_pwm_slice!(11);

// ============================================================================
// Utility Functions
// ============================================================================

/// Get PWM slice number for a GPIO pin (RP2350A: 8 slices).
#[cfg(not(feature = "rp2350b"))]
#[inline]
pub const fn gpio_to_slice(gpio: u8) -> u8 {
    (gpio / 2) % 8
}

/// Get PWM slice number for a GPIO pin (RP2350B: 12 slices).
#[cfg(feature = "rp2350b")]
#[inline]
pub const fn gpio_to_slice(gpio: u8) -> u8 {
    (gpio / 2) % 12
}

/// Get PWM channel (A=0, B=1) for a GPIO pin.
#[inline]
pub const fn gpio_to_channel(gpio: u8) -> u8 {
    gpio % 2
}

/// Calculate the edge-aligned clock divider for a desired PWM frequency.
///
/// This function implements:
///
/// ```text
/// DIV = f_sys / (target_freq * (TOP + 1))
/// ```
///
/// The result is quantized to the hardware's 8.4 fixed-point divider format.
/// If the ideal divider is outside the hardware range, it is clamped to the
/// nearest representable value.
///
/// # Arguments
/// * `sys_clock` - System clock frequency in Hz
/// * `target_freq` - Desired PWM frequency in Hz
/// * `wrap` - Counter wrap value (TOP)
///
/// # Returns
/// Tuple of (int_divider, frac_divider) for the DIV register
pub const fn calc_divider(sys_clock: u32, target_freq: u32, wrap: u16) -> (u8, u8) {
    let div_16 = ((sys_clock as u64) << 4) / ((target_freq as u64) * ((wrap as u64) + 1));
    let div_16 = if div_16 > 0xfff { 0xfff } else { div_16 as u32 };
    let int_part = ((div_16 >> 4) & 0xff) as u8;
    let frac_part = (div_16 & 0xf) as u8;
    let int_part = if int_part == 0 { 1 } else { int_part };
    (int_part, frac_part)
}

/// Calculate the phase-correct clock divider for a desired PWM frequency.
///
/// This uses the phase-correct timing model, where one full PWM cycle takes
/// `2 * wrap` counter steps.
///
/// It therefore implements:
///
/// ```text
/// DIV = f_sys / (target_freq * wrap * 2)
/// ```
///
/// The result is quantized to the hardware's 8.4 fixed-point divider format.
///
/// # Arguments
/// * `sys_clock` - System clock frequency in Hz
/// * `target_freq` - Desired PWM frequency in Hz
/// * `wrap` - Counter wrap value (TOP). Must be non-zero.
///
/// # Returns
/// Tuple of `(int_divider, frac_divider)` for the DIV register.
pub const fn calc_divider_phase_correct(sys_clock: u32, target_freq: u32, wrap: u16) -> (u8, u8) {
    assert!(
        wrap != 0,
        "phase-correct PWM requires a non-zero wrap value"
    );
    let div_16 = ((sys_clock as u64) << 4) / ((target_freq as u64) * (wrap as u64) * 2);
    let div_16 = if div_16 > 0xfff { 0xfff } else { div_16 as u32 };
    let int_part = ((div_16 >> 4) & 0xff) as u8;
    let frac_part = (div_16 & 0xf) as u8;
    let int_part = if int_part == 0 { 1 } else { int_part };
    (int_part, frac_part)
}

/// Calculate the actual edge-aligned PWM frequency given configuration.
///
/// This is the inverse of [`calc_divider`] after quantization:
///
/// ```text
/// f_pwm = f_sys / (DIV * (TOP + 1))
/// ```
///
/// # Arguments
/// * `sys_clock` - System clock frequency in Hz
/// * `div_int` - Integer part of divider
/// * `div_frac` - Fractional part of divider (0-15)
/// * `wrap` - Counter wrap value (TOP)
///
/// # Returns
/// Actual PWM frequency in Hz
pub const fn calc_frequency(sys_clock: u32, div_int: u8, div_frac: u8, wrap: u16) -> u32 {
    let div_16 = ((div_int as u32) << 4) | ((div_frac as u32) & 0xf);
    if div_16 == 0 {
        return 0;
    }
    let freq = ((sys_clock as u64) << 4) / (div_16 as u64) / ((wrap as u64) + 1);
    freq as u32
}

/// Calculate the actual phase-correct PWM frequency given configuration.
///
/// This uses the phase-correct timing model, where one full PWM cycle takes
/// `2 * wrap` counter steps.
///
/// ```text
/// f_pwm = f_sys / (DIV * wrap * 2)
/// ```
///
/// # Arguments
/// * `sys_clock` - System clock frequency in Hz
/// * `div_int` - Integer part of divider
/// * `div_frac` - Fractional part of divider (0-15)
/// * `wrap` - Counter wrap value (TOP). Must be non-zero.
///
/// # Returns
/// Actual PWM frequency in Hz
pub const fn calc_phase_correct_frequency(
    sys_clock: u32,
    div_int: u8,
    div_frac: u8,
    wrap: u16,
) -> u32 {
    if wrap == 0 {
        return 0;
    }
    let div_16 = ((div_int as u32) << 4) | ((div_frac as u32) & 0xf);
    if div_16 == 0 {
        return 0;
    }
    let freq = ((sys_clock as u64) << 4) / (div_16 as u64) / (wrap as u64) / 2;
    freq as u32
}

/// Convert an edge count over a known gate interval into frequency in Hz.
///
/// This is a helper that calculates frequency from edge count
/// over a known gate time.
///
/// It is intended for [`InputMode::RisingEdge`] or [`InputMode::FallingEdge`],
/// where the counter value directly represents the number of observed edges.
///
/// Example: if 240 rising edges are counted in a 100 ms gate interval, the
/// measured frequency is `240 / 0.1 s = 2400 Hz`.
///
/// # Arguments
/// * `edge_count` - Number of edges counted
/// * `gate_time_us` - Gate time in microseconds. Must be non-zero.
///
/// # Returns
/// Frequency in Hz
#[inline]
pub const fn calc_input_frequency(edge_count: u16, gate_time_us: u32) -> u32 {
    (edge_count as u32) * 1_000_000 / gate_time_us
}

/// Calculate duty cycle from a gated HIGH-time count.
///
/// This helper is intended for [`InputMode::GatedByB`]. In that mode the PWM
/// slice counter advances only while the B input is HIGH. If you know the gate
/// interval and the counter clock, you can estimate the input duty cycle.
///
/// The model used here is:
///
/// ```text
/// total_ticks = (sys_clk / divider) * gate_time
/// duty = high_count / total_ticks
/// ```
///
/// This helper currently accepts only the integer divider term. That matches the
/// most common measurement setup, but if you are relying on a fractional divider
/// you should compute the duty from the raw count yourself.
///
/// Example: with a 12 MHz clock, divider `1`, and a 10 ms gate interval, the
/// total available ticks are about `120_000`. If the measured gated count is
/// `60_000`, the input duty cycle is about 50%.
///
/// # Arguments
/// * `high_count` - Counter value (time input was HIGH)
/// * `sys_clk` - System clock frequency in Hz
/// * `divider` - Clock divider (integer part)
/// * `gate_time_us` - Measurement time in microseconds. Must be non-zero.
///
/// # Returns
/// Duty cycle as percentage (0-100)
pub const fn calc_duty_cycle(high_count: u16, sys_clk: u32, divider: u8, gate_time_us: u32) -> u8 {
    let div = if divider == 0 { 1 } else { divider as u32 };
    let ticks_per_us = sys_clk / 1_000_000 / div;
    let total_ticks = ticks_per_us * gate_time_us;
    if total_ticks == 0 {
        return 0;
    }
    let duty = (high_count as u32) * 100 / total_ticks;
    if duty > 100 { 100 } else { duty as u8 }
}
