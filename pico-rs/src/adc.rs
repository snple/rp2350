//! ADC (Analog-to-Digital Converter) HAL for RP2350
//!
//! This module provides a safe interface for the RP2350's 12-bit SAR ADC.
//!
//! ## Features
//!
//! - 500 kS/s (using 48 MHz clock)
//! - 12-bit resolution (9.2 ENOB typical)
//! - 5 channels on RP2350A (GPIO 26-29 + temperature sensor)
//! - 9 channels on RP2350B (GPIO 40-47 + temperature sensor)
//! - Round-robin sampling mode
//! - FIFO with DMA support
//! - One-shot and free-running modes
//!
//! ## Example
//!
//! ```ignore
//! use pico_rs::adc::Adc;
//! use pico_rs::gpio;
//!
//! // Initialize GPIO (required before using ADC pins)
//! gpio::init_gpio();
//!
//! // Initialize ADC
//! let adc = Adc::init();
//!
//! // Read from channel 0 (GPIO 26 on RP2350A, GPIO 40 on RP2350B)
//! let value = adc.read_channel(0);
//!
//! // Read temperature sensor
//! let temp_raw = adc.read_temperature_raw();
//! let temp_c = Adc::convert_temperature(temp_raw);
//! ```

use crate::reset::{self, Peripheral};
use pico_rs_pac::ADC;

/// ADC input channels
#[cfg(feature = "rp2350a")]
pub const NUM_ADC_CHANNELS: usize = 5;

#[cfg(feature = "rp2350b")]
pub const NUM_ADC_CHANNELS: usize = 9;

#[cfg(not(any(feature = "rp2350a", feature = "rp2350b")))]
pub const NUM_ADC_CHANNELS: usize = 5;

/// Base GPIO for ADC inputs
#[cfg(feature = "rp2350b")]
pub const ADC_BASE_PIN: u8 = 40;

#[cfg(not(feature = "rp2350b"))]
pub const ADC_BASE_PIN: u8 = 26;

/// Temperature sensor channel number (last channel)
pub const TEMPERATURE_CHANNEL: u8 = (NUM_ADC_CHANNELS - 1) as u8;

/// ADC voltage reference (typically 3.3V)
pub const ADC_VREF: f32 = 3.3;

/// ADC maximum value (12-bit)
pub const ADC_MAX: u16 = 4095;

/// ADC error types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AdcError {
    /// Conversion error occurred
    ConversionError,
    /// FIFO overflow
    FifoOverflow,
    /// FIFO underflow
    FifoUnderflow,
    /// Invalid channel
    InvalidChannel,
}

/// ADC peripheral wrapper
pub struct Adc {
    regs: &'static pico_rs_pac::adc::RegisterBlock,
}

impl Adc {
    /// Create a new ADC instance without initialization
    ///
    /// Use `init()` to properly initialize the ADC for use.
    #[inline]
    pub fn new() -> Self {
        Self {
            regs: unsafe { &*ADC::PTR },
        }
    }

    /// Initialize and enable the ADC
    ///
    /// This resets the ADC block, enables it, and waits for it to be ready.
    pub fn init() -> Self {
        // Reset the ADC block
        reset::reset_block(Peripheral::ADC);
        reset::unreset_block_wait(Peripheral::ADC);

        let adc = Self::new();

        // Enable ADC
        adc.regs.cs.set_en();

        // Wait for ADC to be ready
        while !adc.is_ready() {
            core::hint::spin_loop();
        }

        adc
    }

    // ========================================================================
    // Status Methods
    // ========================================================================

    /// Check if ADC is ready for a conversion
    #[inline]
    pub fn is_ready(&self) -> bool {
        self.regs.cs.ready()
    }

    /// Check if the most recent conversion had an error
    #[inline]
    pub fn has_error(&self) -> bool {
        self.regs.cs.err()
    }

    /// Check if any past conversion had an error (sticky)
    #[inline]
    pub fn has_sticky_error(&self) -> bool {
        self.regs.cs.err_sticky()
    }

    /// Clear the sticky error flag
    #[inline]
    pub fn clear_sticky_error(&self) {
        self.regs.cs.set_err_sticky();
    }

    // ========================================================================
    // Channel Selection
    // ========================================================================

    /// Select the ADC input channel
    ///
    /// # Arguments
    /// * `channel` - Channel number (0-4 on RP2350A, 0-8 on RP2350B)
    #[inline]
    pub fn select_channel(&self, channel: u8) {
        let channel = channel.min(TEMPERATURE_CHANNEL);
        self.regs.cs.write_ainsel(channel as u32);
    }

    /// Get the currently selected channel
    #[inline]
    pub fn get_selected_channel(&self) -> u8 {
        self.regs.cs.ainsel() as u8
    }

    /// Enable round-robin sampling
    ///
    /// # Arguments
    /// * `channel_mask` - Bit mask of channels to sample (bit 0 = channel 0, etc.)
    #[inline]
    pub fn set_round_robin(&self, channel_mask: u16) {
        let mask = channel_mask & ((1 << NUM_ADC_CHANNELS) - 1);
        self.regs.cs.write_rrobin(mask as u32);
    }

    /// Disable round-robin sampling
    #[inline]
    pub fn disable_round_robin(&self) {
        self.regs.cs.write_rrobin(0);
    }

    // ========================================================================
    // Temperature Sensor
    // ========================================================================

    /// Enable the on-chip temperature sensor
    #[inline]
    pub fn enable_temperature_sensor(&self) {
        self.regs.cs.set_ts_en();
    }

    /// Disable the on-chip temperature sensor
    #[inline]
    pub fn disable_temperature_sensor(&self) {
        self.regs.cs.clr_ts_en();
    }

    /// Check if temperature sensor is enabled
    #[inline]
    pub fn is_temperature_sensor_enabled(&self) -> bool {
        self.regs.cs.ts_en()
    }

    // ========================================================================
    // Single Conversion
    // ========================================================================

    /// Perform a single conversion on the currently selected channel
    ///
    /// Blocks until conversion is complete.
    pub fn read(&self) -> u16 {
        // Start conversion (START_ONCE is write-only, self-clearing)
        self.regs.cs.trigger_start_once();

        // Wait for completion
        while !self.is_ready() {
            core::hint::spin_loop();
        }

        self.regs.result.result() as u16
    }

    /// Perform a single conversion on a specific channel
    ///
    /// # Arguments
    /// * `channel` - Channel to read from
    pub fn read_channel(&self, channel: u8) -> u16 {
        self.select_channel(channel);
        self.read()
    }

    /// Read the temperature sensor raw value
    ///
    /// Returns the raw 12-bit ADC value. Use `convert_temperature()` to
    /// convert to degrees Celsius.
    pub fn read_temperature_raw(&self) -> u16 {
        // Enable temperature sensor if not already enabled
        let was_enabled = self.is_temperature_sensor_enabled();
        if !was_enabled {
            self.enable_temperature_sensor();
            // Small delay for sensor to stabilize
            for _ in 0..100 {
                core::hint::spin_loop();
            }
        }

        let result = self.read_channel(TEMPERATURE_CHANNEL);

        // Restore previous state
        if !was_enabled {
            self.disable_temperature_sensor();
        }

        result
    }

    /// Convert raw ADC temperature value to degrees Celsius
    ///
    /// Formula: T = 27 - (ADC_voltage - 0.706) / 0.001721
    /// Where ADC_voltage = ADC_value * VREF / 4096
    #[inline]
    pub fn convert_temperature(raw: u16) -> f32 {
        let voltage = (raw as f32) * ADC_VREF / (ADC_MAX as f32 + 1.0);
        27.0 - (voltage - 0.706) / 0.001721
    }

    // ========================================================================
    // Free-Running Mode
    // ========================================================================

    /// Start free-running mode
    ///
    /// The ADC will continuously perform conversions.
    #[inline]
    pub fn start_free_running(&self) {
        self.regs.cs.set_start_many();
    }

    /// Stop free-running mode
    #[inline]
    pub fn stop_free_running(&self) {
        self.regs.cs.clr_start_many();
    }

    /// Check if free-running mode is active
    #[inline]
    pub fn is_free_running(&self) -> bool {
        self.regs.cs.start_many()
    }

    // ========================================================================
    // Clock Divisor
    // ========================================================================

    /// Set the clock divisor for free-running mode
    ///
    /// Total period = (1 + int + frac/256) cycles
    /// Note: Minimum 96 cycles needed per conversion
    ///
    /// # Arguments
    /// * `int_div` - Integer part (0-65535)
    /// * `frac_div` - Fractional part (0-255)
    pub fn set_clkdiv(&self, int_div: u16, frac_div: u8) {
        self.regs.div.write_int(int_div as u32);
        self.regs.div.write_frac(frac_div as u32);
    }

    /// Set clock divisor from a floating point value
    pub fn set_clkdiv_float(&self, div: f32) {
        let div = div.clamp(0.0, 65536.0);
        let int_div = div as u16;
        let frac_div = ((div - int_div as f32) * 256.0) as u8;
        self.set_clkdiv(int_div, frac_div);
    }

    // ========================================================================
    // FIFO Operations
    // ========================================================================

    /// Setup the ADC FIFO
    ///
    /// # Arguments
    /// * `enable` - Enable writing results to FIFO
    /// * `dreq_enable` - Enable DMA requests when FIFO has data
    /// * `threshold` - DMA request/IRQ threshold (0-15)
    /// * `error_in_fifo` - Include error flag in FIFO data
    /// * `byte_shift` - Shift results to one byte for byte DMA
    pub fn fifo_setup(
        &self,
        enable: bool,
        dreq_enable: bool,
        threshold: u8,
        error_in_fifo: bool,
        byte_shift: bool,
    ) {
        let mut fcs = self.regs.fcs.read_val();
        fcs = fcs.with_en(enable);
        fcs = fcs.with_dreq_en(dreq_enable);
        fcs = fcs.with_thresh(threshold.min(15) as u32);
        fcs = fcs.with_err(error_in_fifo);
        fcs = fcs.with_shift(byte_shift);
        self.regs.fcs.write_val(fcs);
    }

    /// Check if FIFO is empty
    #[inline]
    pub fn fifo_is_empty(&self) -> bool {
        self.regs.fcs.empty()
    }

    /// Check if FIFO is full
    #[inline]
    pub fn fifo_is_full(&self) -> bool {
        self.regs.fcs.full()
    }

    /// Get FIFO level (number of entries)
    #[inline]
    pub fn fifo_level(&self) -> u8 {
        self.regs.fcs.level() as u8
    }

    /// Check if FIFO has overflowed (sticky)
    #[inline]
    pub fn fifo_has_overflowed(&self) -> bool {
        self.regs.fcs.over()
    }

    /// Clear FIFO overflow flag
    #[inline]
    pub fn fifo_clear_overflow(&self) {
        self.regs.fcs.set_over();
    }

    /// Check if FIFO has underflowed (sticky)
    #[inline]
    pub fn fifo_has_underflowed(&self) -> bool {
        self.regs.fcs.under()
    }

    /// Clear FIFO underflow flag
    #[inline]
    pub fn fifo_clear_underflow(&self) {
        self.regs.fcs.set_under();
    }

    /// Pop a value from the FIFO
    ///
    /// Returns `None` if FIFO is empty.
    pub fn fifo_pop(&self) -> Option<u16> {
        if self.fifo_is_empty() {
            None
        } else {
            Some(self.regs.fifo.val() as u16)
        }
    }

    /// Pop a value from the FIFO, blocking until available
    pub fn fifo_pop_blocking(&self) -> u16 {
        while self.fifo_is_empty() {
            core::hint::spin_loop();
        }
        self.regs.fifo.val() as u16
    }

    /// Pop a value from FIFO with error flag
    ///
    /// Returns (value, error_flag). Only valid if error_in_fifo was enabled.
    pub fn fifo_pop_with_error(&self) -> Option<(u16, bool)> {
        if self.fifo_is_empty() {
            None
        } else {
            let raw = self.regs.fifo.read_bits();
            let value = (raw & 0xFFF) as u16;
            let error = (raw & 0x8000) != 0;
            Some((value, error))
        }
    }

    /// Drain the FIFO, discarding all contents
    pub fn fifo_drain(&self) {
        // Wait for any in-progress conversion
        while !self.is_ready() {
            core::hint::spin_loop();
        }
        // Drain the FIFO
        while !self.fifo_is_empty() {
            let _ = self.regs.fifo.val();
        }
    }

    // ========================================================================
    // Interrupts
    // ========================================================================

    /// Check if FIFO interrupt is pending (raw)
    #[inline]
    pub fn fifo_irq_raw(&self) -> bool {
        self.regs.intr.fifo()
    }

    /// Enable FIFO interrupt
    #[inline]
    pub fn enable_fifo_irq(&self) {
        self.regs.inte.set_fifo();
    }

    /// Disable FIFO interrupt
    #[inline]
    pub fn disable_fifo_irq(&self) {
        self.regs.inte.clr_fifo();
    }

    /// Check masked interrupt status
    #[inline]
    pub fn fifo_irq_status(&self) -> bool {
        self.regs.ints.fifo()
    }

    // ========================================================================
    // Utility Functions
    // ========================================================================

    /// Convert raw ADC value to voltage
    #[inline]
    pub fn to_voltage(raw: u16) -> f32 {
        (raw as f32) * ADC_VREF / (ADC_MAX as f32 + 1.0)
    }

    /// Convert raw ADC value to millivolts
    #[inline]
    pub fn to_millivolts(raw: u16) -> u32 {
        ((raw as u32) * 3300) / 4096
    }

    /// Get the DMA DREQ number for ADC
    #[inline]
    pub const fn dreq() -> u8 {
        36 // DREQ_ADC from RP2350 datasheet
    }

    /// Get the FIFO address for DMA
    #[inline]
    pub fn fifo_address(&self) -> *const u32 {
        &self.regs.fifo as *const _ as *const u32
    }
}

impl Default for Adc {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait for pins that can be used as ADC inputs
///
/// This trait is implemented for GPIO pins that are connected to ADC channels.
/// For RP2350A: GPIO 26-29 (ADC channels 0-3)
/// For RP2350B: GPIO 40-47 (ADC channels 0-7)
pub trait AdcPin {
    /// The ADC channel number for this pin
    const CHANNEL: u8;

    /// Initialize the pin for ADC use
    ///
    /// This sets the pin to NULL function (high-Z) and disables pulls.
    fn init_adc(&self);
}

/// Macro to implement AdcPin for a GPIO pin type
#[macro_export]
macro_rules! impl_adc_pin {
    ($pin_type:ty, $channel:expr) => {
        impl $crate::adc::AdcPin for $pin_type {
            const CHANNEL: u8 = $channel;

            fn init_adc(&self) {
                // Set to NULL function (high-Z output for ADC)
                self.set_function($crate::gpio::Function::Null);
                // Disable pull resistors
                self.set_pull($crate::gpio::Pull::None);
            }
        }
    };
}

// Note: AdcPin implementations should be generated in the gpio module or
// by the user when needed, using the impl_adc_pin! macro.
// For RP2350A: impl_adc_pin!(Pin26<'_>, 0); etc.
// For RP2350B: impl_adc_pin!(Pin40<'_>, 0); etc.
