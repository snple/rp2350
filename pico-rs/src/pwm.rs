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
//! │        └── PWM ──► Pwm::new(pwm)                                 │
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
//! let pins = Pins::new(dp.IO_BANK0, dp.PADS_BANK0);
//!
//! // Create PWM controller (owns the PWM peripheral)
//! let pwm = Pwm::new(dp.PWM);
//!
//! // Create slice 0 with output on channel A
//! let slice0 = pwm.slice::<0>().output_a(pins.pin::<0>(), 150_000_000);
//!
//! // Configure for 1 kHz PWM
//! slice0.set_wrap(65535);
//! slice0.set_div_int(2);
//! slice0.set_duty_a(32768);  // 50% duty cycle
//! slice0.enable();
//!
//! // Create another slice
//! let slice1 = pwm.slice::<1>().output_ab(pins.pin::<2>(), pins.pin::<3>(), 150_000_000);
//! slice1.set_wrap(1000);
//! slice1.enable();
//! ```
//!
//! # Example (Input Mode)
//!
//! ```ignore
//! use pico_rs::pwm::{Pwm, InputMode};
//!
//! let pwm = Pwm::new(dp.PWM);
//!
//! // Initialize PWM slice 0 for input measurement
//! let slice0 = pwm.slice::<0>().input(pins.pin::<1>(), InputMode::RisingEdge, 150_000_000);
//!
//! // Measure frequency
//! slice0.start_measurement();
//! timer.delay_ms(100);  // 100ms gate time
//! let edges = slice0.stop_measurement();
//! let freq_hz = edges as u32 * 10;  // Convert to Hz
//! ```

use paste::paste;

use crate::gpio::pins::{NoPin, PwmAPin, PwmBPin};
use crate::reset::{self, Peripheral};

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
    /// Counter runs only when input is HIGH (duty cycle measurement)
    GatedByB,
    /// Counter increments on rising edge (frequency measurement)
    RisingEdge,
    /// Counter increments on falling edge
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
    /// Output on channel A only
    OutputA,
    /// Output on both channels A and B
    OutputAB,
    /// Input measurement using channel B
    Input,
    /// Output on A + input measurement on B
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
    _pin_a: A,
    _pin_b: B,
    mode: Mode,
}

impl<const N: u8, A, B> Slice<N, A, B> {
    /// Create a PWM slice handle without going through [`Pwm`].
    ///
    /// # Safety
    ///
    /// The caller must ensure the PWM peripheral has been initialised
    /// (i.e. taken out of reset) before using the returned handle.
    #[inline]
    pub const unsafe fn new() -> Slice<N, NoPin, NoPin> {
        Slice {
            _pin_a: NoPin,
            _pin_b: NoPin,
            mode: Mode::OutputA,
        }
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
/// let pwm = Pwm::new(dp.PWM);
///
/// // Get an unconfigured slice, then bind a pin
/// let slice0 = pwm.slice::<0>().output_a(pins.pin::<0>(), sys_clk);
///
/// // Create slice 1
/// let slice1 = pwm.slice::<1>().output_ab(pins.pin::<2>(), pins.pin::<3>(), sys_clk);
/// ```
pub struct Pwm {
    _pwm: pico_rs_pac::PWM,
}

impl Pwm {
    /// Create a new PWM controller from the peripheral.
    ///
    /// This initializes the PWM subsystem by taking it out of reset.
    /// The peripheral is consumed to ensure only one `Pwm` instance exists.
    pub fn new(pwm: pico_rs_pac::PWM) -> Self {
        reset::unreset_block_wait(Peripheral::PWM);
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
            _pin_a: NoPin,
            _pin_b: NoPin,
            mode: Mode::OutputA,
        }
    }

    #[inline]
    fn regs(&self) -> &pico_rs_pac::pwm::RegisterBlock {
        // Safety: We own the PWM peripheral
        unsafe { &*pico_rs_pac::PWM::PTR }
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
                /// * `_sys_clk` - System clock frequency in Hz (for reference)
                pub fn output_a<A: PwmAPin>(self, pin_a: A, _sys_clk: u32) -> Slice<$slice_num, A, NoPin> {
                    // Verify the pin is for this slice
                    assert_eq!(A::PWM_SLICE, $slice_num, "Pin must be for PWM slice {}", $slice_num);

                    // Configure pin for PWM function
                    PwmAPin::configure_pwm_a(&pin_a);

                    // Initialize via direct register access (avoids const-generic
                    // method resolution ambiguity)
                    let pwm = unsafe { &*pico_rs_pac::PWM::PTR };
                    pwm.[<ch $slice_num _csr>].clr_en();
                    pwm.[<ch $slice_num _top>].write_bits(0xffff);
                    pwm.[<ch $slice_num _div>].write_bits(1u32 << 4);
                    pwm.[<ch $slice_num _cc>].write_a(0);
                    pwm.intr.write_bits(1 << $slice_num);
                    pwm.[<ch $slice_num _csr>].write_divmode(DivMode::FreeRunning as u32);

                    Slice {
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
                /// * `_sys_clk` - System clock frequency in Hz
                pub fn output_ab<A: PwmAPin, B: PwmBPin>(self, pin_a: A, pin_b: B, _sys_clk: u32) -> Slice<$slice_num, A, B> {
                    // Verify pins are for this slice
                    assert_eq!(A::PWM_SLICE, $slice_num, "Pin A must be for PWM slice {}", $slice_num);
                    assert_eq!(B::PWM_SLICE, $slice_num, "Pin B must be for PWM slice {}", $slice_num);

                    // Configure pins for PWM function
                    PwmAPin::configure_pwm_a(&pin_a);
                    PwmBPin::configure_pwm_b(&pin_b);

                    // Initialize via direct register access
                    let pwm = unsafe { &*pico_rs_pac::PWM::PTR };
                    pwm.[<ch $slice_num _csr>].clr_en();
                    pwm.[<ch $slice_num _top>].write_bits(0xffff);
                    pwm.[<ch $slice_num _div>].write_bits(1u32 << 4);
                    pwm.[<ch $slice_num _cc>].write_a(0);
                    pwm.[<ch $slice_num _cc>].write_b(0);
                    pwm.intr.write_bits(1 << $slice_num);
                    pwm.[<ch $slice_num _csr>].write_divmode(DivMode::FreeRunning as u32);

                    Slice {
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
                /// # Arguments
                ///
                /// * `pin_b` - Channel B pin for input
                /// * `mode` - Input mode (GatedByB, RisingEdge, FallingEdge)
                /// * `_sys_clk` - System clock frequency in Hz
                pub fn input<B: PwmBPin>(self, pin_b: B, mode: InputMode, _sys_clk: u32) -> Slice<$slice_num, NoPin, B> {
                    // Verify pin is for this slice
                    assert_eq!(B::PWM_SLICE, $slice_num, "Pin B must be for PWM slice {}", $slice_num);

                    // Configure pin for PWM function
                    PwmBPin::configure_pwm_b(&pin_b);

                    // Initialize for input via direct register access
                    let pwm = unsafe { &*pico_rs_pac::PWM::PTR };
                    pwm.[<ch $slice_num _csr>].clr_en();
                    pwm.[<ch $slice_num _top>].write_bits(0xffff);
                    pwm.[<ch $slice_num _div>].write_bits(1u32 << 4);
                    pwm.[<ch $slice_num _ctr>].write_bits(0);
                    pwm.intr.write_bits(1 << $slice_num);
                    pwm.[<ch $slice_num _csr>].write_divmode(DivMode::from(mode) as u32);

                    Slice {
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
                /// # Arguments
                ///
                /// * `pin_a` - Channel A pin for output
                /// * `pin_b` - Channel B pin for input measurement
                /// * `mode` - Input mode for measurement
                /// * `_sys_clk` - System clock frequency in Hz
                pub fn output_a_input<A: PwmAPin, B: PwmBPin>(
                    self,
                    pin_a: A,
                    pin_b: B,
                    mode: InputMode,
                    _sys_clk: u32,
                ) -> Slice<$slice_num, A, B> {
                    // Verify pins are for this slice
                    assert_eq!(A::PWM_SLICE, $slice_num, "Pin A must be for PWM slice {}", $slice_num);
                    assert_eq!(B::PWM_SLICE, $slice_num, "Pin B must be for PWM slice {}", $slice_num);

                    // Configure pins
                    PwmAPin::configure_pwm_a(&pin_a);
                    PwmBPin::configure_pwm_b(&pin_b);

                    // Initialize via direct register access
                    let pwm = unsafe { &*pico_rs_pac::PWM::PTR };
                    pwm.[<ch $slice_num _csr>].clr_en();
                    pwm.[<ch $slice_num _top>].write_bits(0xffff);
                    pwm.[<ch $slice_num _div>].write_bits(1u32 << 4);
                    pwm.[<ch $slice_num _cc>].write_a(0);
                    pwm.[<ch $slice_num _ctr>].write_bits(0);
                    pwm.intr.write_bits(1 << $slice_num);
                    pwm.[<ch $slice_num _csr>].write_divmode(DivMode::from(mode) as u32);

                    Slice {
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
                    // Safety: PWM peripheral is owned by Pwm struct,
                    // and slices are created from Pwm methods
                    unsafe { &*pico_rs_pac::PWM::PTR }
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
                #[inline]
                pub fn set_div_mode(&self, mode: DivMode) {
                    self.csr().write_divmode(mode as u32);
                }

                /// Get the current divider mode.
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
                /// The output goes high when counter < value.
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
                /// The output goes high when counter < value.
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

                /// Set duty cycle for both channels.
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

                /// Start a measurement (resets counter and enables).
                #[inline]
                pub fn start_measurement(&self) {
                    self.reset_counter();
                    self.clear_interrupt();
                    self.enable();
                }

                /// Stop measurement and return the count.
                #[inline]
                pub fn stop_measurement(&self) -> u16 {
                    self.disable();
                    self.get_counter()
                }

                /// Check if overflow occurred during measurement.
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

/// Calculate clock divider for desired frequency.
///
/// # Arguments
/// * `sys_clock` - System clock frequency in Hz
/// * `target_freq` - Desired PWM frequency in Hz
/// * `wrap` - Counter wrap value (TOP)
///
/// # Returns
/// Tuple of (int_divider, frac_divider) for the DIV register
pub const fn calc_divider(sys_clock: u32, target_freq: u32, wrap: u16) -> (u8, u8) {
    // freq = sys_clock / (div * (wrap + 1))
    // div = sys_clock / (freq * (wrap + 1))
    let div_16 = ((sys_clock as u64) << 4) / ((target_freq as u64) * ((wrap as u64) + 1));
    let div_16 = if div_16 > 0xfff { 0xfff } else { div_16 as u32 };
    let int_part = ((div_16 >> 4) & 0xff) as u8;
    let frac_part = (div_16 & 0xf) as u8;
    let int_part = if int_part == 0 { 1 } else { int_part };
    (int_part, frac_part)
}

/// Calculate the actual PWM frequency given configuration.
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

/// Measure frequency in Hz.
///
/// This is a helper that calculates frequency from edge count
/// over a known gate time.
///
/// # Arguments
/// * `edge_count` - Number of edges counted
/// * `gate_time_us` - Gate time in microseconds
///
/// # Returns
/// Frequency in Hz
#[inline]
pub const fn calc_input_frequency(edge_count: u16, gate_time_us: u32) -> u32 {
    (edge_count as u32) * 1_000_000 / gate_time_us
}

/// Calculate duty cycle from gated count.
///
/// # Arguments
/// * `high_count` - Counter value (time input was HIGH)
/// * `sys_clk` - System clock frequency in Hz
/// * `divider` - Clock divider (integer part)
/// * `gate_time_us` - Measurement time in microseconds
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
