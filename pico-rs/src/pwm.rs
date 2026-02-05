//! PWM (Pulse Width Modulation) HAL for RP2350
//!
//! This module provides a safe interface for the RP2350 PWM peripheral.
//! The RP2350 has 12 PWM slices (channels), each with two outputs (A and B).
//!
//! ## Features
//!
//! - 12 independent PWM slices
//! - 16-bit counter with configurable wrap value
//! - Fractional clock divider (8.4 format)
//! - Phase-correct or edge-aligned modes
//! - Individual A and B outputs per slice
//! - **Input mode support** for frequency/duty cycle measurement
//!
//! ## GPIO Mapping
//!
//! Each GPIO can be mapped to a PWM output/input:
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
//! │   11    │    A    │ 22, 28  │   44    │
//! │   11    │    B    │ 23, 29  │   45    │
//! └─────────┴─────────┴─────────┴─────────┘
//!
//! PWM slice = (gpio_num / 2) % 12
//! PWM channel = gpio_num % 2 (0 = A, 1 = B)
//! ```
//!
//! ## Output Mode Example
//!
//! ```ignore
//! use pico_rs::pwm::Pwm;
//!
//! let pwm = Pwm::init();
//! let slice = pwm.slice(4); // For GPIO 8/9
//!
//! // Configure for 1 kHz at 150 MHz sys clock
//! slice.set_wrap(65535);
//! slice.set_div_int(2);
//! slice.set_channel_a(32768); // 50% duty cycle
//! slice.enable();
//! ```
//!
//! # Input Mode
//!
//! The PWM can measure external signals using the B channel as input.
//! Three input modes are available via the `DIVMODE` register field:
//!
//! ```text
//! ┌────────────────┬──────────────────────────────────────────────────────┐
//! │     Mode       │                    Description                       │
//! ├────────────────┼──────────────────────────────────────────────────────┤
//! │  GatedByB      │ Counter runs at divider rate ONLY when B pin is HIGH │
//! │                │ → Use for: Duty cycle measurement, pulse width       │
//! ├────────────────┼──────────────────────────────────────────────────────┤
//! │  RisingEdgeB   │ Counter increments on each RISING edge of B pin      │
//! │                │ → Use for: Frequency measurement, pulse counting     │
//! ├────────────────┼──────────────────────────────────────────────────────┤
//! │  FallingEdgeB  │ Counter increments on each FALLING edge of B pin     │
//! │                │ → Use for: Frequency measurement, pulse counting     │
//! └────────────────┴──────────────────────────────────────────────────────┘
//! ```
//!
//! ## Input Mode: Frequency Measurement
//!
//! Count edges over a known time period to measure frequency:
//!
//! ```ignore
//! use pico_rs::pwm::{Pwm, DivMode};
//!
//! let pwm = Pwm::init();
//! let slice = pwm.slice(0); // GPIO 1 is PWM0 B input
//!
//! // Configure for rising edge counting
//! slice.set_div_mode(DivMode::RisingEdgeB);
//! slice.set_wrap(0xFFFF);  // Max wrap to avoid overflow
//! slice.set_counter(0);
//! slice.enable();
//!
//! // Measure: count edges over 100ms
//! let start_count = slice.get_counter();
//! timer.delay_ms(100);
//! let end_count = slice.get_counter();
//!
//! let edges = end_count.wrapping_sub(start_count);
//! let frequency_hz = edges as u32 * 10; // edges per 100ms → Hz
//! ```
//!
//! ## Input Mode: Duty Cycle Measurement
//!
//! Use gated counting to measure how long the signal is HIGH:
//!
//! ```ignore
//! use pico_rs::pwm::{Pwm, DivMode};
//!
//! let pwm = Pwm::init();
//! let slice = pwm.slice(0);
//!
//! // Configure: counter runs only when B input is HIGH
//! slice.set_div_mode(DivMode::GatedByB);
//! slice.set_div(1, 0);     // Fastest counting (sys_clk rate)
//! slice.set_wrap(0xFFFF);
//! slice.set_counter(0);
//! slice.enable();
//!
//! // Measure over a known period
//! timer.delay_us(1000); // 1ms measurement window
//! let high_count = slice.get_counter();
//!
//! // At 150MHz with div=1, 1ms = 150,000 total counts
//! // duty_cycle = high_count / 150_000
//! ```
//!
//! ## Input Mode: Pulse Counting
//!
//! Simply count pulses without time reference:
//!
//! ```ignore
//! slice.set_div_mode(DivMode::RisingEdgeB);
//! slice.set_wrap(0xFFFF);
//! slice.set_counter(0);
//! slice.enable();
//!
//! // ... later ...
//! let pulse_count = slice.get_counter();
//! // Check for overflow: slice.did_wrap()
//! ```

use crate::reset::{self, Peripheral};
use pico_rs_pac::PWM;

/// Number of PWM slices
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

/// PWM peripheral wrapper
pub struct Pwm {
    pwm: &'static pico_rs_pac::pwm::RegisterBlock,
}

impl Pwm {
    /// Create a new PWM instance without initialization
    #[inline]
    pub fn new() -> Self {
        Self {
            pwm: unsafe { &*PWM::PTR },
        }
    }

    /// Initialize the PWM peripheral
    ///
    /// This brings the PWM out of reset.
    pub fn init() -> Self {
        reset::unreset_block_wait(Peripheral::PWM);
        Self::new()
    }

    /// Get a reference to a PWM slice
    #[inline]
    pub fn slice(&self, slice: u8) -> PwmSlice<'_> {
        assert!(slice < 12, "PWM slice must be 0-11");
        PwmSlice {
            pwm: self.pwm,
            slice,
        }
    }

    /// Enable multiple PWM slices simultaneously
    ///
    /// # Arguments
    /// * `mask` - Bitmask of slices to enable (bit 0 = slice 0, etc.)
    #[inline]
    pub fn enable_mask(&self, mask: u16) {
        self.pwm.en.set_bits(mask as u32);
    }

    /// Disable multiple PWM slices simultaneously
    ///
    /// # Arguments
    /// * `mask` - Bitmask of slices to disable
    #[inline]
    pub fn disable_mask(&self, mask: u16) {
        self.pwm.en.clr_bits(mask as u32);
    }

    /// Get enabled status of all slices
    #[inline]
    pub fn enabled_mask(&self) -> u16 {
        (self.pwm.en.read_bits() & 0xfff) as u16
    }

    // ========================================================================
    // Interrupt Control (IRQ0)
    // ========================================================================

    /// Get raw interrupt status (before masking)
    #[inline]
    pub fn interrupt_raw(&self) -> u16 {
        (self.pwm.intr.read_bits() & 0xfff) as u16
    }

    /// Check if raw interrupt is pending for a slice
    #[inline]
    pub fn is_interrupt_raw(&self, slice: u8) -> bool {
        (self.pwm.intr.read_bits() & (1 << slice)) != 0
    }

    /// Clear interrupt for slices using mask
    #[inline]
    pub fn clear_interrupt(&self, mask: u16) {
        self.pwm.intr.write_bits(mask as u32);
    }

    /// Clear interrupt for a specific slice
    #[inline]
    pub fn clear_slice_interrupt(&self, slice: u8) {
        self.pwm.intr.write_bits(1 << slice);
    }

    /// Clear all PWM interrupts
    #[inline]
    pub fn clear_all_interrupts(&self) {
        self.pwm.intr.write_bits(0xfff);
    }

    /// Enable interrupt for slices using mask (IRQ0)
    #[inline]
    pub fn enable_interrupt(&self, mask: u16) {
        self.pwm.irq0_inte.set_bits(mask as u32);
    }

    /// Disable interrupt for slices using mask (IRQ0)
    #[inline]
    pub fn disable_interrupt(&self, mask: u16) {
        self.pwm.irq0_inte.clr_bits(mask as u32);
    }

    /// Enable interrupt for a specific slice (IRQ0)
    #[inline]
    pub fn enable_slice_interrupt(&self, slice: u8) {
        self.pwm.irq0_inte.set_bits(1 << slice);
    }

    /// Disable interrupt for a specific slice (IRQ0)
    #[inline]
    pub fn disable_slice_interrupt(&self, slice: u8) {
        self.pwm.irq0_inte.clr_bits(1 << slice);
    }

    /// Disable all interrupts (IRQ0)
    #[inline]
    pub fn disable_all_interrupts(&self) {
        self.pwm.irq0_inte.clr_bits(0xfff);
    }

    /// Check if interrupt is enabled for a slice (IRQ0)
    #[inline]
    pub fn is_slice_interrupt_enabled(&self, slice: u8) -> bool {
        (self.pwm.irq0_inte.read_bits() & (1 << slice)) != 0
    }

    /// Get interrupt enable mask (IRQ0)
    #[inline]
    pub fn interrupt_enabled_mask(&self) -> u16 {
        (self.pwm.irq0_inte.read_bits() & 0xfff) as u16
    }

    /// Get interrupt status after masking (IRQ0)
    #[inline]
    pub fn interrupt_status(&self) -> u16 {
        (self.pwm.irq0_ints.read_bits() & 0xfff) as u16
    }

    /// Check if interrupt is pending for a slice (masked, IRQ0)
    #[inline]
    pub fn is_slice_interrupt_pending(&self, slice: u8) -> bool {
        (self.pwm.irq0_ints.read_bits() & (1 << slice)) != 0
    }

    /// Force interrupt for slices (for testing)
    #[inline]
    pub fn force_interrupt(&self, mask: u16) {
        self.pwm.irq0_intf.set_bits(mask as u32);
    }

    /// Clear forced interrupt for slices
    #[inline]
    pub fn clear_forced_interrupt(&self, mask: u16) {
        self.pwm.irq0_intf.clr_bits(mask as u32);
    }

    // ========================================================================
    // Interrupt Control (IRQ1) - Secondary interrupt output
    // ========================================================================

    /// Enable interrupt for slices (IRQ1)
    #[inline]
    pub fn enable_interrupt_irq1(&self, mask: u16) {
        self.pwm.irq1_inte.set_bits(mask as u32);
    }

    /// Disable interrupt for slices (IRQ1)
    #[inline]
    pub fn disable_interrupt_irq1(&self, mask: u16) {
        self.pwm.irq1_inte.clr_bits(mask as u32);
    }

    /// Get interrupt status after masking (IRQ1)
    #[inline]
    pub fn interrupt_status_irq1(&self) -> u16 {
        (self.pwm.irq1_ints.read_bits() & 0xfff) as u16
    }

    /// Get reference to the underlying register block
    ///
    /// # Safety
    /// Direct register access can bypass the HAL's safety guarantees.
    #[inline]
    pub unsafe fn registers(&self) -> &pico_rs_pac::pwm::RegisterBlock {
        self.pwm
    }
}

impl Default for Pwm {
    fn default() -> Self {
        Self::new()
    }
}

/// A single PWM slice (channel)
pub struct PwmSlice<'a> {
    pwm: &'a pico_rs_pac::pwm::RegisterBlock,
    slice: u8,
}

impl<'a> PwmSlice<'a> {
    /// Get the slice number
    #[inline]
    pub fn slice_num(&self) -> u8 {
        self.slice
    }

    // ========================================================================
    // Enable/Disable
    // ========================================================================

    /// Enable this PWM slice
    #[inline]
    pub fn enable(&self) {
        self.csr_set_bits(1); // EN bit
    }

    /// Disable this PWM slice
    #[inline]
    pub fn disable(&self) {
        self.csr_clr_bits(1); // EN bit
    }

    /// Check if this slice is enabled
    #[inline]
    pub fn is_enabled(&self) -> bool {
        self.csr_read() & 1 != 0
    }

    // ========================================================================
    // Configuration
    // ========================================================================

    /// Set phase-correct mode
    ///
    /// In phase-correct mode, the counter counts up to TOP then back down,
    /// giving symmetric PWM waveforms.
    #[inline]
    pub fn set_phase_correct(&self, enable: bool) {
        if enable {
            self.csr_set_bits(1 << 1);
        } else {
            self.csr_clr_bits(1 << 1);
        }
    }

    /// Set clock divider mode
    #[inline]
    pub fn set_div_mode(&self, mode: DivMode) {
        let csr = self.csr_read();
        let csr = (csr & !(0x3 << 4)) | ((mode as u32) << 4);
        self.csr_write(csr);
    }

    /// Invert output A
    #[inline]
    pub fn set_invert_a(&self, invert: bool) {
        if invert {
            self.csr_set_bits(1 << 2);
        } else {
            self.csr_clr_bits(1 << 2);
        }
    }

    /// Invert output B
    #[inline]
    pub fn set_invert_b(&self, invert: bool) {
        if invert {
            self.csr_set_bits(1 << 3);
        } else {
            self.csr_clr_bits(1 << 3);
        }
    }

    // ========================================================================
    // Clock Divider
    // ========================================================================

    /// Set the integer part of the clock divider (1-255)
    ///
    /// The PWM counter advances at: sys_clk / (int + frac/16)
    #[inline]
    pub fn set_div_int(&self, div: u8) {
        let current = self.div_read();
        let new_val = (current & 0xf) | ((div as u32) << 4);
        self.div_write(new_val);
    }

    /// Set the fractional part of the clock divider (0-15)
    #[inline]
    pub fn set_div_frac(&self, frac: u8) {
        let current = self.div_read();
        let new_val = (current & !0xf) | ((frac as u32) & 0xf);
        self.div_write(new_val);
    }

    /// Set both integer and fractional divider parts
    #[inline]
    pub fn set_div(&self, int_part: u8, frac_part: u8) {
        let val = ((int_part as u32) << 4) | ((frac_part as u32) & 0xf);
        self.div_write(val);
    }

    /// Get the current divider value (8.4 fixed point format)
    #[inline]
    pub fn get_div(&self) -> (u8, u8) {
        let val = self.div_read();
        let int_part = ((val >> 4) & 0xff) as u8;
        let frac_part = (val & 0xf) as u8;
        (int_part, frac_part)
    }

    // ========================================================================
    // Counter and Wrap
    // ========================================================================

    /// Set the wrap (TOP) value
    ///
    /// The counter wraps (or reverses in phase-correct mode) when it reaches
    /// this value. Maximum is 65535.
    #[inline]
    pub fn set_wrap(&self, wrap: u16) {
        self.top_write(wrap as u32);
    }

    /// Get the current wrap value
    #[inline]
    pub fn get_wrap(&self) -> u16 {
        self.top_read() as u16
    }

    /// Set the counter value directly
    #[inline]
    pub fn set_counter(&self, value: u16) {
        self.ctr_write(value as u32);
    }

    /// Get the current counter value
    #[inline]
    pub fn get_counter(&self) -> u16 {
        self.ctr_read() as u16
    }

    // ========================================================================
    // Compare Values (Duty Cycle)
    // ========================================================================

    /// Set compare value for channel A
    ///
    /// Output A goes high when counter < compare value (in normal mode).
    #[inline]
    pub fn set_channel_a(&self, value: u16) {
        let cc = self.cc_read();
        let cc = (cc & 0xffff0000) | (value as u32);
        self.cc_write(cc);
    }

    /// Set compare value for channel B
    #[inline]
    pub fn set_channel_b(&self, value: u16) {
        let cc = self.cc_read();
        let cc = (cc & 0x0000ffff) | ((value as u32) << 16);
        self.cc_write(cc);
    }

    /// Set both channel compare values
    #[inline]
    pub fn set_channels(&self, a: u16, b: u16) {
        let cc = (a as u32) | ((b as u32) << 16);
        self.cc_write(cc);
    }

    /// Get compare value for channel A
    #[inline]
    pub fn get_channel_a(&self) -> u16 {
        (self.cc_read() & 0xffff) as u16
    }

    /// Get compare value for channel B
    #[inline]
    pub fn get_channel_b(&self) -> u16 {
        ((self.cc_read() >> 16) & 0xffff) as u16
    }

    // ========================================================================
    // Phase Control
    // ========================================================================

    /// Advance phase by one count
    ///
    /// Self-clearing; write 1 to advance phase while running.
    #[inline]
    pub fn advance_phase(&self) {
        self.csr_set_bits(1 << 7);
    }

    /// Retard phase by one count
    ///
    /// Self-clearing; write 1 to retard phase while running.
    #[inline]
    pub fn retard_phase(&self) {
        self.csr_set_bits(1 << 6);
    }

    // ========================================================================
    // Input Mode Support
    // ========================================================================
    //
    // The PWM can measure external signals using the B channel as input.
    // Three input modes are available:
    //
    //   DivMode::GatedByB    - Counter runs only when B is HIGH
    //                          Use for: duty cycle measurement, pulse width
    //
    //   DivMode::RisingEdgeB - Counter increments on rising edge
    //                          Use for: frequency measurement, pulse counting
    //
    //   DivMode::FallingEdgeB - Counter increments on falling edge
    //                           Use for: frequency measurement, pulse counting
    //
    // Basic usage:
    //   1. Configure GPIO for PWM function
    //   2. Set div_mode to an input mode
    //   3. Set wrap to 0xFFFF (max) to avoid overflow
    //   4. Optionally set divider (for gated mode timing)
    //   5. Reset counter with set_counter(0)
    //   6. Enable slice
    //   7. Read counter after measurement period
    //
    // ========================================================================

    /// Get the current divider mode
    #[inline]
    pub fn get_div_mode(&self) -> DivMode {
        let csr = self.csr_read();
        let mode = (csr >> 4) & 0x3;
        match mode {
            0 => DivMode::FreeRunning,
            1 => DivMode::GatedByB,
            2 => DivMode::RisingEdgeB,
            3 => DivMode::FallingEdgeB,
            _ => unreachable!(),
        }
    }

    /// Check if this slice is configured for input mode
    ///
    /// Returns true if `div_mode` is `GatedByB`, `RisingEdgeB`, or `FallingEdgeB`.
    #[inline]
    pub fn is_input_mode(&self) -> bool {
        self.get_div_mode().is_input_mode()
    }

    /// Check if a counter wrap occurred (useful for overflow detection)
    ///
    /// In input mode, a wrap indicates the counter overflowed during measurement.
    /// The wrap flag is set when the counter reaches the TOP value.
    ///
    /// # Example
    /// ```ignore
    /// slice.set_counter(0);
    /// slice.enable();
    /// // ... measure ...
    /// if slice.did_wrap() {
    ///     // Overflow occurred - use larger divider or shorter gate time
    /// }
    /// ```
    #[inline]
    pub fn did_wrap(&self) -> bool {
        self.is_interrupt_raw()
    }

    /// Get the primary B input GPIO number for this slice
    ///
    /// Returns the lowest-numbered GPIO that maps to this slice's B channel.
    ///
    /// | Slice | B GPIO |
    /// |-------|--------|
    /// |   0   |   1    |
    /// |   1   |   3    |
    /// |   2   |   5    |
    /// |  ...  |  ...   |
    /// |  11   |  23    |
    #[inline]
    pub const fn b_input_gpio(&self) -> u8 {
        self.slice * 2 + 1
    }

    // ========================================================================
    // Interrupt Support
    // ========================================================================

    /// Enable interrupt for this slice (IRQ0)
    #[inline]
    pub fn enable_interrupt(&self) {
        self.pwm.irq0_inte.set_bits(1 << self.slice);
    }

    /// Disable interrupt for this slice (IRQ0)
    #[inline]
    pub fn disable_interrupt(&self) {
        self.pwm.irq0_inte.clr_bits(1 << self.slice);
    }

    /// Check if interrupt is enabled for this slice (IRQ0)
    #[inline]
    pub fn is_interrupt_enabled(&self) -> bool {
        (self.pwm.irq0_inte.read_bits() & (1 << self.slice)) != 0
    }

    /// Check if raw interrupt is pending for this slice
    #[inline]
    pub fn is_interrupt_raw(&self) -> bool {
        (self.pwm.intr.read_bits() & (1 << self.slice)) != 0
    }

    /// Check if interrupt is pending for this slice (masked)
    #[inline]
    pub fn is_interrupt_pending(&self) -> bool {
        (self.pwm.irq0_ints.read_bits() & (1 << self.slice)) != 0
    }

    /// Clear interrupt for this slice
    #[inline]
    pub fn clear_interrupt(&self) {
        self.pwm.intr.write_bits(1 << self.slice);
    }

    /// Force interrupt for this slice (for testing)
    #[inline]
    pub fn force_interrupt(&self) {
        self.pwm.irq0_intf.set_bits(1 << self.slice);
    }

    /// Clear forced interrupt for this slice
    #[inline]
    pub fn clear_forced_interrupt(&self) {
        self.pwm.irq0_intf.clr_bits(1 << self.slice);
    }

    // ========================================================================
    // Register Access Helpers
    // ========================================================================

    #[inline]
    fn csr_offset(&self) -> usize {
        (self.slice as usize) * 0x14
    }

    #[inline]
    fn csr_read(&self) -> u32 {
        unsafe {
            let base = self.pwm as *const _ as *const u8;
            let ptr = base.add(self.csr_offset()) as *const u32;
            core::ptr::read_volatile(ptr)
        }
    }

    #[inline]
    fn csr_write(&self, value: u32) {
        unsafe {
            let base = self.pwm as *const _ as *mut u8;
            let ptr = base.add(self.csr_offset()) as *mut u32;
            core::ptr::write_volatile(ptr, value);
        }
    }

    #[inline]
    fn csr_set_bits(&self, bits: u32) {
        unsafe {
            let base = self.pwm as *const _ as *const u8;
            let ptr = base.add(self.csr_offset() + 0x2000) as *mut u32;
            core::ptr::write_volatile(ptr, bits);
        }
    }

    #[inline]
    fn csr_clr_bits(&self, bits: u32) {
        unsafe {
            let base = self.pwm as *const _ as *const u8;
            let ptr = base.add(self.csr_offset() + 0x3000) as *mut u32;
            core::ptr::write_volatile(ptr, bits);
        }
    }

    #[inline]
    fn div_read(&self) -> u32 {
        unsafe {
            let base = self.pwm as *const _ as *const u8;
            let ptr = base.add(self.csr_offset() + 0x4) as *const u32;
            core::ptr::read_volatile(ptr)
        }
    }

    #[inline]
    fn div_write(&self, value: u32) {
        unsafe {
            let base = self.pwm as *const _ as *mut u8;
            let ptr = base.add(self.csr_offset() + 0x4) as *mut u32;
            core::ptr::write_volatile(ptr, value);
        }
    }

    #[inline]
    fn ctr_read(&self) -> u32 {
        unsafe {
            let base = self.pwm as *const _ as *const u8;
            let ptr = base.add(self.csr_offset() + 0x8) as *const u32;
            core::ptr::read_volatile(ptr)
        }
    }

    #[inline]
    fn ctr_write(&self, value: u32) {
        unsafe {
            let base = self.pwm as *const _ as *mut u8;
            let ptr = base.add(self.csr_offset() + 0x8) as *mut u32;
            core::ptr::write_volatile(ptr, value);
        }
    }

    #[inline]
    fn cc_read(&self) -> u32 {
        unsafe {
            let base = self.pwm as *const _ as *const u8;
            let ptr = base.add(self.csr_offset() + 0xc) as *const u32;
            core::ptr::read_volatile(ptr)
        }
    }

    #[inline]
    fn cc_write(&self, value: u32) {
        unsafe {
            let base = self.pwm as *const _ as *mut u8;
            let ptr = base.add(self.csr_offset() + 0xc) as *mut u32;
            core::ptr::write_volatile(ptr, value);
        }
    }

    #[inline]
    fn top_read(&self) -> u32 {
        unsafe {
            let base = self.pwm as *const _ as *const u8;
            let ptr = base.add(self.csr_offset() + 0x10) as *const u32;
            core::ptr::read_volatile(ptr)
        }
    }

    #[inline]
    fn top_write(&self, value: u32) {
        unsafe {
            let base = self.pwm as *const _ as *mut u8;
            let ptr = base.add(self.csr_offset() + 0x10) as *mut u32;
            core::ptr::write_volatile(ptr, value);
        }
    }
}

// ============================================================================
// Utility Functions
// ============================================================================

/// Get PWM slice number for a GPIO pin
#[inline]
pub const fn gpio_to_slice(gpio: u8) -> u8 {
    (gpio / 2) % 12
}

/// Get PWM channel (A=0, B=1) for a GPIO pin
#[inline]
pub const fn gpio_to_channel(gpio: u8) -> u8 {
    gpio % 2
}

/// Calculate clock divider for desired frequency
///
/// # Arguments
/// * `sys_clock` - System clock frequency in Hz
/// * `target_freq` - Desired PWM frequency in Hz
/// * `wrap` - Counter wrap value (TOP)
///
/// # Returns
/// Tuple of (int_divider, frac_divider) for the DIV register
pub fn calc_divider(sys_clock: u32, target_freq: u32, wrap: u16) -> (u8, u8) {
    // freq = sys_clock / (div * (wrap + 1))
    // div = sys_clock / (freq * (wrap + 1))
    let div_16 = ((sys_clock as u64) << 4) / ((target_freq as u64) * ((wrap as u64) + 1));
    let div_16 = div_16.min(0xfff) as u32; // 8.4 format max
    let int_part = ((div_16 >> 4) & 0xff) as u8;
    let frac_part = (div_16 & 0xf) as u8;
    (int_part.max(1), frac_part) // Minimum divider is 1
}
