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
//! ┌─────────────────────────────────────────────────────────────────┐
//! │                    PWM Ownership Model                          │
//! ├─────────────────────────────────────────────────────────────────┤
//! │                                                                 │
//! │  Peripherals::take()                                            │
//! │        │                                                        │
//! │        └── PWM ──► Pwm::new(pwm)                                │
//! │                          │                                      │
//! │                          ├── slice0: Slice0 ──► .output_a(pin)  │
//! │                          │                          │           │
//! │                          │                    PwmSlice0<A, NoPin>│
//! │                          │                   (token consumed)   │
//! │                          │                                      │
//! │                          ├── slice1: Slice1 ──► .output_ab(...) │
//! │                          │                          │           │
//! │                          │                    PwmSlice1<A, B>   │
//! │                          │                   (token consumed)   │
//! │                          │                                      │
//! │                          └── ...                                │
//! │                                                                 │
//! └─────────────────────────────────────────────────────────────────┘
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
//! // Create PWM controller (owns the PWM peripheral and slice tokens)
//! let pwm = Pwm::new(dp.PWM);
//!
//! // Create slice 0 with output on channel A - consumes pwm.slice0 token
//! let slice0 = pwm.slice0.output_a(pins.p0, 150_000_000);
//!
//! // Configure for 1 kHz PWM
//! slice0.set_wrap(65535);
//! slice0.set_div_int(2);
//! slice0.set_duty_a(32768);  // 50% duty cycle
//! slice0.enable();
//!
//! // Create another slice - consumes pwm.slice1 token
//! let slice1 = pwm.slice1.output_ab(pins.p2, pins.p3, 150_000_000);
//! slice1.set_wrap(1000);
//! slice1.enable();
//!
//! // This would fail to compile - pwm.slice0 already consumed:
//! // let slice0_again = pwm.slice0.output_a(pins.p16, 150_000_000);
//! ```
//!
//! # Example (Input Mode)
//!
//! ```ignore
//! use pico_rs::pwm::{Pwm, InputMode};
//!
//! let pwm = Pwm::new(dp.PWM);
//!
//! // Initialize PWM slice 0 for input measurement - consumes pwm.slice0 token
//! let slice0 = pwm.slice0.input(pins.p1, InputMode::RisingEdge, 150_000_000);
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
// PWM Global Controller
// ============================================================================

/// PWM peripheral controller.
///
/// This struct owns the PWM peripheral and provides slice tokens as public fields.
/// Each slice token can be consumed to create a configured PWM slice, ensuring
/// that each slice can only be created once.
///
/// # Ownership Model
///
/// ```text
/// Pwm::new(dp.PWM)
///     │
///     ├── slice0: Slice0  ──► pwm.slice0.output_a(pin) ──► PwmSlice0<A, NoPin>
///     │                           (token consumed)
///     ├── slice1: Slice1  ──► pwm.slice1.output_ab(pin_a, pin_b) ──► PwmSlice1<A, B>
///     │                           (token consumed)
///     └── ...
/// ```
///
/// # Example
///
/// ```ignore
/// let pwm = Pwm::new(dp.PWM);
///
/// // Create slice 0 - consumes pwm.slice0 token
/// let slice0 = pwm.slice0.output_a(pins.p0, sys_clk);
///
/// // Create slice 1 - consumes pwm.slice1 token
/// let slice1 = pwm.slice1.output_ab(pins.p2, pins.p3, sys_clk);
///
/// // This would fail to compile - pwm.slice0 already consumed:
/// // let slice0_again = pwm.slice0.output_a(pins.p16, sys_clk);
/// ```
pub struct Pwm {
    _pwm: pico_rs_pac::PWM,
    /// PWM slice 0 token
    pub slice0: Slice0,
    /// PWM slice 1 token
    pub slice1: Slice1,
    /// PWM slice 2 token
    pub slice2: Slice2,
    /// PWM slice 3 token
    pub slice3: Slice3,
    /// PWM slice 4 token
    pub slice4: Slice4,
    /// PWM slice 5 token
    pub slice5: Slice5,
    /// PWM slice 6 token
    pub slice6: Slice6,
    /// PWM slice 7 token
    pub slice7: Slice7,
    #[cfg(feature = "rp2350b")]
    /// PWM slice 8 token (RP2350B only)
    pub slice8: Slice8,
    #[cfg(feature = "rp2350b")]
    /// PWM slice 9 token (RP2350B only)
    pub slice9: Slice9,
    #[cfg(feature = "rp2350b")]
    /// PWM slice 10 token (RP2350B only)
    pub slice10: Slice10,
    #[cfg(feature = "rp2350b")]
    /// PWM slice 11 token (RP2350B only)
    pub slice11: Slice11,
}

impl Pwm {
    /// Create a new PWM controller from the peripheral.
    ///
    /// This initializes the PWM subsystem by taking it out of reset.
    /// The peripheral is consumed to ensure only one `Pwm` instance exists.
    /// All slice tokens are created and can be consumed to create individual slices.
    pub fn new(pwm: pico_rs_pac::PWM) -> Self {
        reset::unreset_block_wait(Peripheral::PWM);
        Self {
            _pwm: pwm,
            slice0: Slice0::new(),
            slice1: Slice1::new(),
            slice2: Slice2::new(),
            slice3: Slice3::new(),
            slice4: Slice4::new(),
            slice5: Slice5::new(),
            slice6: Slice6::new(),
            slice7: Slice7::new(),
            #[cfg(feature = "rp2350b")]
            slice8: Slice8::new(),
            #[cfg(feature = "rp2350b")]
            slice9: Slice9::new(),
            #[cfg(feature = "rp2350b")]
            slice10: Slice10::new(),
            #[cfg(feature = "rp2350b")]
            slice11: Slice11::new(),
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
// Macro for generating PWM slice implementations and token constructors
// ============================================================================

macro_rules! impl_pwm_slice {
    ($SliceToken:ident, $PwmSlice:ident, $slice_num:literal) => {
        paste! {
            // ================================================================
            // Slice Token
            // ================================================================

            #[doc = concat!("PWM slice ", $slice_num, " token.")]
            ///
            /// This token is consumed when creating a configured PWM slice,
            /// ensuring each slice can only be created once.
            pub struct $SliceToken {
                _private: (),
            }

            impl $SliceToken {
                #[inline]
                const fn new() -> Self {
                    Self { _private: () }
                }

                #[doc = concat!("Create PWM slice ", $slice_num, " with output on channel A only.")]
                ///
                /// Consumes this token, ensuring the slice can only be created once.
                ///
                /// # Arguments
                ///
                /// * `pin_a` - Channel A pin (must be for this slice)
                /// * `_sys_clk` - System clock frequency in Hz (for reference)
                pub fn output_a<A: PwmAPin>(self, pin_a: A, _sys_clk: u32) -> $PwmSlice<A, NoPin> {
                    // Verify the pin is for this slice
                    assert_eq!(A::PWM_SLICE, $slice_num, "Pin must be for PWM slice {}", $slice_num);

                    // Configure pin for PWM function
                    PwmAPin::configure_pwm_a(&pin_a);

                    let instance = $PwmSlice {
                        _pin_a: pin_a,
                        _pin_b: NoPin,
                        mode: Mode::OutputA,
                    };

                    // Initialize with sensible defaults
                    instance.disable();
                    instance.set_wrap(0xffff);
                    instance.set_div(1, 0);
                    instance.set_duty_a(0);
                    instance.clear_interrupt();
                    instance.set_div_mode(DivMode::FreeRunning);

                    instance
                }

                #[doc = concat!("Create PWM slice ", $slice_num, " with output on both channels A and B.")]
                ///
                /// Consumes this token, ensuring the slice can only be created once.
                ///
                /// # Arguments
                ///
                /// * `pin_a` - Channel A pin
                /// * `pin_b` - Channel B pin
                /// * `_sys_clk` - System clock frequency in Hz
                pub fn output_ab<A: PwmAPin, B: PwmBPin>(self, pin_a: A, pin_b: B, _sys_clk: u32) -> $PwmSlice<A, B> {
                    // Verify pins are for this slice
                    assert_eq!(A::PWM_SLICE, $slice_num, "Pin A must be for PWM slice {}", $slice_num);
                    assert_eq!(B::PWM_SLICE, $slice_num, "Pin B must be for PWM slice {}", $slice_num);

                    // Configure pins for PWM function
                    PwmAPin::configure_pwm_a(&pin_a);
                    PwmBPin::configure_pwm_b(&pin_b);

                    let instance = $PwmSlice {
                        _pin_a: pin_a,
                        _pin_b: pin_b,
                        mode: Mode::OutputAB,
                    };

                    // Initialize with sensible defaults
                    instance.disable();
                    instance.set_wrap(0xffff);
                    instance.set_div(1, 0);
                    instance.set_duty_a(0);
                    instance.set_duty_b(0);
                    instance.clear_interrupt();
                    instance.set_div_mode(DivMode::FreeRunning);

                    instance
                }

                #[doc = concat!("Create PWM slice ", $slice_num, " for input measurement using channel B.")]
                ///
                /// Consumes this token, ensuring the slice can only be created once.
                ///
                /// # Arguments
                ///
                /// * `pin_b` - Channel B pin for input
                /// * `mode` - Input mode (GatedByB, RisingEdge, FallingEdge)
                /// * `_sys_clk` - System clock frequency in Hz
                pub fn input<B: PwmBPin>(self, pin_b: B, mode: InputMode, _sys_clk: u32) -> $PwmSlice<NoPin, B> {
                    // Verify pin is for this slice
                    assert_eq!(B::PWM_SLICE, $slice_num, "Pin B must be for PWM slice {}", $slice_num);

                    // Configure pin for PWM function
                    PwmBPin::configure_pwm_b(&pin_b);

                    let instance = $PwmSlice {
                        _pin_a: NoPin,
                        _pin_b: pin_b,
                        mode: Mode::Input,
                    };

                    // Initialize for input mode
                    instance.disable();
                    instance.set_wrap(0xffff);
                    instance.set_div(1, 0);
                    instance.reset_counter();
                    instance.clear_interrupt();
                    instance.set_div_mode(mode.into());

                    instance
                }

                #[doc = concat!("Create PWM slice ", $slice_num, " with output on A and input on B.")]
                ///
                /// Consumes this token, ensuring the slice can only be created once.
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
                ) -> $PwmSlice<A, B> {
                    // Verify pins are for this slice
                    assert_eq!(A::PWM_SLICE, $slice_num, "Pin A must be for PWM slice {}", $slice_num);
                    assert_eq!(B::PWM_SLICE, $slice_num, "Pin B must be for PWM slice {}", $slice_num);

                    // Configure pins
                    PwmAPin::configure_pwm_a(&pin_a);
                    PwmBPin::configure_pwm_b(&pin_b);

                    let instance = $PwmSlice {
                        _pin_a: pin_a,
                        _pin_b: pin_b,
                        mode: Mode::OutputAWithInput,
                    };

                    // Initialize
                    instance.disable();
                    instance.set_wrap(0xffff);
                    instance.set_div(1, 0);
                    instance.set_duty_a(0);
                    instance.reset_counter();
                    instance.clear_interrupt();
                    instance.set_div_mode(mode.into());

                    instance
                }
            }

            // ================================================================
            // PWM Slice Struct
            // ================================================================

            /// PWM slice wrapper with type-safe pin ownership.
            ///
            /// This struct holds references to the GPIO pins used for output/input.
            /// The pins are configured automatically when the slice is created,
            /// and their ownership prevents them from being used for other purposes.
            ///
            #[doc = concat!("# PWM Slice ", $slice_num)]
            pub struct $PwmSlice<A, B> {
                _pin_a: A,
                _pin_b: B,
                mode: Mode,
            }

            impl<A, B> $PwmSlice<A, B> {
                /// PWM slice number
                pub const SLICE: u8 = $slice_num;

                // ================================================================
                // Private register accessors
                // ================================================================

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

                // ================================================================
                // Enable/Disable
                // ================================================================

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

                // ================================================================
                // Configuration
                // ================================================================

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

                // ================================================================
                // Clock Divider
                // ================================================================

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

                // ================================================================
                // Counter and Wrap
                // ================================================================

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

                // ================================================================
                // Phase Control
                // ================================================================

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

                // ================================================================
                // Channel A (Output)
                // ================================================================

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

                // ================================================================
                // Channel B (Output/Input)
                // ================================================================

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

                // ================================================================
                // Interrupt Support
                // ================================================================

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

                // ================================================================
                // Status and Mode
                // ================================================================

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

                // ================================================================
                // Input Mode Helpers
                // ================================================================

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
                /// Note: The slice token is not returned - creating a new slice
                /// for this slice number is no longer possible.
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
            }
        }
    };
}

// ============================================================================
// Generate PWM slices
// ============================================================================

// Slices 0-7 are available on both RP2350A and RP2350B
impl_pwm_slice!(Slice0, PwmSlice0, 0);
impl_pwm_slice!(Slice1, PwmSlice1, 1);
impl_pwm_slice!(Slice2, PwmSlice2, 2);
impl_pwm_slice!(Slice3, PwmSlice3, 3);
impl_pwm_slice!(Slice4, PwmSlice4, 4);
impl_pwm_slice!(Slice5, PwmSlice5, 5);
impl_pwm_slice!(Slice6, PwmSlice6, 6);
impl_pwm_slice!(Slice7, PwmSlice7, 7);

// Slices 8-11 are only available on RP2350B
#[cfg(feature = "rp2350b")]
impl_pwm_slice!(Slice8, PwmSlice8, 8);
#[cfg(feature = "rp2350b")]
impl_pwm_slice!(Slice9, PwmSlice9, 9);
#[cfg(feature = "rp2350b")]
impl_pwm_slice!(Slice10, PwmSlice10, 10);
#[cfg(feature = "rp2350b")]
impl_pwm_slice!(Slice11, PwmSlice11, 11);

// ============================================================================
// Type Aliases for backward compatibility
// ============================================================================

/// Type alias for PwmSlice0
pub type Pwm0<A, B> = PwmSlice0<A, B>;
/// Type alias for PwmSlice1
pub type Pwm1<A, B> = PwmSlice1<A, B>;
/// Type alias for PwmSlice2
pub type Pwm2<A, B> = PwmSlice2<A, B>;
/// Type alias for PwmSlice3
pub type Pwm3<A, B> = PwmSlice3<A, B>;
/// Type alias for PwmSlice4
pub type Pwm4<A, B> = PwmSlice4<A, B>;
/// Type alias for PwmSlice5
pub type Pwm5<A, B> = PwmSlice5<A, B>;
/// Type alias for PwmSlice6
pub type Pwm6<A, B> = PwmSlice6<A, B>;
/// Type alias for PwmSlice7
pub type Pwm7<A, B> = PwmSlice7<A, B>;

#[cfg(feature = "rp2350b")]
/// Type alias for PwmSlice8
pub type Pwm8<A, B> = PwmSlice8<A, B>;
#[cfg(feature = "rp2350b")]
/// Type alias for PwmSlice9
pub type Pwm9<A, B> = PwmSlice9<A, B>;
#[cfg(feature = "rp2350b")]
/// Type alias for PwmSlice10
pub type Pwm10<A, B> = PwmSlice10<A, B>;
#[cfg(feature = "rp2350b")]
/// Type alias for PwmSlice11
pub type Pwm11<A, B> = PwmSlice11<A, B>;

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
