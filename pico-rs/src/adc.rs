//! ADC (Analog-to-Digital Converter) HAL for RP2350
//!
//! This module provides a type-safe interface for the RP2350's 12-bit SAR ADC.
//!
//! # Features
//!
//! - 500 kS/s (using 48 MHz clock)
//! - 12-bit resolution (9.2 ENOB typical)
//! - **RP2350A**: 4 analog inputs (GPIO 26-29) + temperature sensor
//! - **RP2350B**: 8 analog inputs (GPIO 40-47) + temperature sensor
//! - Round-robin sampling mode
//! - FIFO with DMA support
//! - One-shot and free-running modes
//! - **Type-safe pin ownership**: Pins are consumed when configured for ADC
//!
//! # GPIO Mapping
//!
//! | Channel | RP2350A GPIO | RP2350B GPIO |
//! |---------|--------------|--------------|
//! | 0       | GPIO 26      | GPIO 40      |
//! | 1       | GPIO 27      | GPIO 41      |
//! | 2       | GPIO 28      | GPIO 42      |
//! | 3       | GPIO 29      | GPIO 43      |
//! | 4       | -            | GPIO 44      |
//! | 5       | -            | GPIO 45      |
//! | 6       | -            | GPIO 46      |
//! | 7       | -            | GPIO 47      |
//! | 4/8     | Temp Sensor  | Temp Sensor  |
//!
//! # Example (Basic Usage)
//!
//! ```ignore
//! use pico_rs::adc::Adc;
//! use pico_rs::gpio::Pins;
//!
//! let dp = Peripherals::take().unwrap();
//! let pins = Pins::new(dp.IO_BANK0, dp.PADS_BANK0);
//!
//! // Create ADC controller
//! let mut adc = Adc::new(dp.ADC);
//!
//! // Configure pin for ADC - consumes the pin, returns AdcInput
//! let input0 = pins.pin::<26>().into_adc_input();
//!
//! // Read from the configured input
//! let value = adc.read(&input0);
//! let voltage_mv = Adc::to_millivolts(value);
//!
//! // Compile error: pins.pin::<26>() already moved!
//! // pins.pin::<26>().into_output();
//! ```
//!
//! # Example (Temperature Sensor)
//!
//! ```ignore
//! let mut adc = Adc::new(dp.ADC);
//!
//! // Read temperature (no pin needed)
//! let raw = adc.read_temp();
//! let temp_c = Adc::convert_temp_to_celsius(raw);
//! ```
//!
//! # Example (Round-Robin with FIFO)
//!
//! ```ignore
//! let mut adc = Adc::new(dp.ADC);
//!
//! // Configure pins - ownership transferred
//! let input0 = pins.pin::<26>().into_adc_input();
//! let input1 = pins.pin::<27>().into_adc_input();
//!
//! // Setup FIFO
//! adc.fifo_setup(true, false, 1, false, false);
//!
//! // Enable round-robin for channels 0, 1, and temperature
//! adc.set_round_robin(input0.mask() | input1.mask() | (1 << TEMPERATURE_CHANNEL));
//!
//! // Start free-running mode
//! adc.start_free_running();
//!
//! // Read from FIFO
//! while let Some(value) = adc.fifo_pop() {
//!     // Process value...
//! }
//! ```

use crate::gpio::{Function, Pin, Pull};
use crate::reset::Resettable;
use pico_rs_pac::ADC;

// ============================================================================
// Constants
// ============================================================================

/// Number of analog input channels (excluding temperature sensor)
#[cfg(not(feature = "rp2350b"))]
pub const NUM_CHANNELS: usize = 4;

/// Number of analog input channels (excluding temperature sensor)
#[cfg(feature = "rp2350b")]
pub const NUM_CHANNELS: usize = 8;

/// Total number of ADC channels including temperature sensor
pub const NUM_ADC_CHANNELS: usize = NUM_CHANNELS + 1;

/// Base GPIO for ADC inputs (RP2350A: GPIO 26)
#[cfg(not(feature = "rp2350b"))]
pub const ADC_BASE_PIN: u8 = 26;

/// Base GPIO for ADC inputs (RP2350B: GPIO 40)
#[cfg(feature = "rp2350b")]
pub const ADC_BASE_PIN: u8 = 40;

/// Temperature sensor channel number (last channel)
pub const TEMPERATURE_CHANNEL: u8 = NUM_CHANNELS as u8;

/// ADC voltage reference (typically 3.3V)
pub const ADC_VREF: f32 = 3.3;

/// ADC maximum value (12-bit)
pub const ADC_MAX: u16 = 4095;

/// DMA DREQ number for ADC
pub const ADC_DREQ: u8 = 48;

// ============================================================================
// Error Types
// ============================================================================

/// ADC error types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// Conversion error occurred
    ConversionError,
    /// FIFO overflow
    FifoOverflow,
    /// FIFO underflow
    FifoUnderflow,
    /// Invalid channel
    InvalidChannel,
}

// ============================================================================
// ADC Input (Type-Safe Pin Wrapper)
// ============================================================================

/// A GPIO pin configured for ADC input.
///
/// This type holds ownership of a pin that has been configured for analog input.
/// The pin cannot be used for other purposes while this exists.
///
/// # Type Parameters
/// * `const CH` - The ADC channel number (compile-time constant)
pub struct AdcInput<const CH: u8> {
    // We don't need to store the pin, just consume ownership
    _marker: core::marker::PhantomData<*const ()>,
}

impl<const CH: u8> AdcInput<CH> {
    /// Get the channel number for this input.
    #[inline]
    pub const fn channel(&self) -> u8 {
        CH
    }

    /// Get the channel mask for round-robin configuration.
    #[inline]
    pub const fn mask(&self) -> u16 {
        1 << CH
    }
}

impl<const CH: u8> core::fmt::Debug for AdcInput<CH> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("AdcInput").field("channel", &CH).finish()
    }
}

// ============================================================================
// ADC Pin Trait
// ============================================================================

/// Trait for pins that can be used as ADC inputs.
///
/// This trait is implemented for GPIO pins that are connected to ADC channels.
/// - **RP2350A**: GPIO 26-29 (ADC channels 0-3)
/// - **RP2350B**: GPIO 40-47 (ADC channels 0-7)
pub trait AdcPin: Sized {
    /// The ADC channel type with const generic
    type Input;

    /// Configure this pin for ADC input and consume it.
    ///
    /// Returns an `AdcInput` that can be used with `Adc::read()`.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let input = pins.pin::<26>().into_adc_input();
    /// let value = adc.read(&input);
    /// ```
    fn into_adc_input(self) -> Self::Input;
}

// Implement AdcPin for valid ADC pins
macro_rules! impl_adc_pin {
    ($pin_num:literal, $channel:literal) => {
        impl AdcPin for Pin<$pin_num> {
            type Input = AdcInput<$channel>;

            fn into_adc_input(self) -> AdcInput<$channel> {
                // Set to NULL function (high-Z output for ADC)
                self.set_function(Function::Null);
                // Disable pull resistors
                self.set_pull(Pull::None);
                // Initialize pad (clear isolation, enable input)
                self.init_pad();

                AdcInput {
                    _marker: core::marker::PhantomData,
                }
            }
        }
    };
}

// RP2350A ADC pin mappings
#[cfg(not(feature = "rp2350b"))]
mod pin_impls {
    use super::*;
    impl_adc_pin!(26, 0);
    impl_adc_pin!(27, 1);
    impl_adc_pin!(28, 2);
    impl_adc_pin!(29, 3);
}

// RP2350B ADC pin mappings
#[cfg(feature = "rp2350b")]
mod pin_impls {
    use super::*;
    impl_adc_pin!(40, 0);
    impl_adc_pin!(41, 1);
    impl_adc_pin!(42, 2);
    impl_adc_pin!(43, 3);
    impl_adc_pin!(44, 4);
    impl_adc_pin!(45, 5);
    impl_adc_pin!(46, 6);
    impl_adc_pin!(47, 7);
}

// ============================================================================
// ADC Controller
// ============================================================================

/// ADC peripheral controller.
///
/// Provides a type-safe interface for analog-to-digital conversion.
///
/// # Example
///
/// ```ignore
/// let mut adc = Adc::new(dp.ADC);
///
/// // Configure pin for ADC - ownership transferred
/// let input = pins.pin::<26>().into_adc_input();
///
/// // Read from the input
/// let value = adc.read(&input);
/// ```
pub struct Adc {
    adc: ADC,
    temp_enabled: bool,
}

impl Adc {
    /// Create a new ADC controller.
    ///
    /// Initializes the ADC subsystem by taking it out of reset and enabling it.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let mut adc = Adc::new(dp.ADC);
    /// ```
    pub fn new(adc: ADC) -> Self {
        // Reset peripheral to ensure clean register state
        <pico_rs_pac::ADC as Resettable>::init();

        // Enable ADC
        adc.cs.set_en();

        // Wait for ADC to be ready
        while !adc.cs.ready() {
            core::hint::spin_loop();
        }

        Self {
            adc,
            temp_enabled: false,
        }
    }

    /// Get reference to the underlying register block.
    ///
    /// # Safety
    /// Direct register access can bypass the HAL's safety guarantees.
    #[inline]
    pub unsafe fn regs(&self) -> &pico_rs_pac::adc::RegisterBlock {
        &self.adc
    }

    // ========================================================================
    // Single-Shot Conversion
    // ========================================================================

    /// Perform a single conversion on the specified input.
    ///
    /// Blocks until conversion is complete.
    ///
    /// # Arguments
    /// * `input` - Reference to a configured ADC input
    ///
    /// # Example
    ///
    /// ```ignore
    /// let input = pins.pin::<26>().into_adc_input();
    /// let value = adc.read(&input);
    /// let voltage_mv = Adc::to_millivolts(value);
    /// ```
    pub fn read<const CH: u8>(&mut self, _input: &AdcInput<CH>) -> u16 {
        self.read_channel(CH)
    }

    /// Perform a single conversion on the specified channel (internal).
    #[inline]
    fn read_channel(&mut self, channel: u8) -> u16 {
        // Select channel
        self.adc.cs.write_ainsel(channel as u32);

        // Start conversion
        self.adc.cs.trigger_start_once();

        // Wait for completion
        while !self.adc.cs.ready() {
            core::hint::spin_loop();
        }

        self.adc.result.result() as u16
    }

    /// Read temperature sensor.
    ///
    /// Enables the temperature sensor if not already enabled.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let raw = adc.read_temp();
    /// let temp_c = Adc::convert_temp_to_celsius(raw);
    /// ```
    pub fn read_temp(&mut self) -> u16 {
        // Enable temperature sensor if needed
        if !self.temp_enabled {
            self.adc.cs.set_ts_en();
            self.temp_enabled = true;

            // Small delay for sensor to stabilize
            for _ in 0..100 {
                core::hint::spin_loop();
            }
        }

        // Select temperature channel
        self.adc.cs.write_ainsel(TEMPERATURE_CHANNEL as u32);

        // Start conversion
        self.adc.cs.trigger_start_once();

        // Wait for completion
        while !self.adc.cs.ready() {
            core::hint::spin_loop();
        }

        self.adc.result.result() as u16
    }

    /// Start a conversion on the specified input (non-blocking).
    ///
    /// Use `is_ready()` to check for completion, then `read_result()`.
    #[inline]
    pub fn start_conversion<const CH: u8>(&mut self, _input: &AdcInput<CH>) {
        self.adc.cs.write_ainsel(CH as u32);
        self.adc.cs.trigger_start_once();
    }

    /// Read the result of a completed conversion.
    ///
    /// Only valid after a conversion has completed.
    #[inline]
    pub fn read_result(&self) -> u16 {
        self.adc.result.result() as u16
    }

    // ========================================================================
    // Status
    // ========================================================================

    /// Check if ADC is ready for a conversion.
    #[inline]
    pub fn is_ready(&self) -> bool {
        self.adc.cs.ready()
    }

    /// Check if the most recent conversion had an error.
    #[inline]
    pub fn has_error(&self) -> bool {
        self.adc.cs.err()
    }

    /// Check if any past conversion had an error (sticky).
    #[inline]
    pub fn has_sticky_error(&self) -> bool {
        self.adc.cs.err_sticky()
    }

    /// Clear the sticky error flag.
    #[inline]
    pub fn clear_sticky_error(&mut self) {
        self.adc.cs.set_err_sticky();
    }

    /// Get the currently selected channel.
    #[inline]
    pub fn selected_channel(&self) -> u8 {
        self.adc.cs.ainsel() as u8
    }

    // ========================================================================
    // Temperature Sensor Control
    // ========================================================================

    /// Enable the on-chip temperature sensor.
    #[inline]
    pub fn enable_temp_sensor(&mut self) {
        self.adc.cs.set_ts_en();
        self.temp_enabled = true;
    }

    /// Disable the on-chip temperature sensor.
    #[inline]
    pub fn disable_temp_sensor(&mut self) {
        self.adc.cs.clr_ts_en();
        self.temp_enabled = false;
    }

    /// Check if temperature sensor is enabled.
    #[inline]
    pub fn is_temp_sensor_enabled(&self) -> bool {
        self.temp_enabled
    }

    // ========================================================================
    // Round-Robin Sampling
    // ========================================================================

    /// Enable round-robin sampling.
    ///
    /// # Arguments
    /// * `channel_mask` - Bit mask of channels to sample (bit 0 = channel 0, etc.)
    ///
    /// # Example
    ///
    /// ```ignore
    /// let input0 = pins.pin::<26>().into_adc_input();
    /// let input1 = pins.pin::<27>().into_adc_input();
    ///
    /// // Sample channels 0, 1, and temperature sensor
    /// adc.set_round_robin(input0.mask() | input1.mask() | (1 << TEMPERATURE_CHANNEL));
    /// ```
    #[inline]
    pub fn set_round_robin(&mut self, channel_mask: u16) {
        let mask = channel_mask & ((1 << NUM_ADC_CHANNELS) - 1);
        self.adc.cs.write_rrobin(mask as u32);
    }

    /// Disable round-robin sampling.
    #[inline]
    pub fn disable_round_robin(&mut self) {
        self.adc.cs.write_rrobin(0);
    }

    /// Get current round-robin mask.
    #[inline]
    pub fn round_robin_mask(&self) -> u16 {
        self.adc.cs.rrobin() as u16
    }

    // ========================================================================
    // Free-Running Mode
    // ========================================================================

    /// Start free-running mode.
    ///
    /// The ADC will continuously perform conversions.
    #[inline]
    pub fn start_free_running(&mut self) {
        self.adc.cs.set_start_many();
    }

    /// Stop free-running mode.
    #[inline]
    pub fn stop_free_running(&mut self) {
        self.adc.cs.clr_start_many();
    }

    /// Check if free-running mode is active.
    #[inline]
    pub fn is_free_running(&self) -> bool {
        self.adc.cs.start_many()
    }

    // ========================================================================
    // Clock Divisor
    // ========================================================================

    /// Set the clock divisor for free-running mode.
    ///
    /// Total period = (1 + int + frac/256) cycles.
    /// Note: Minimum 96 cycles needed per conversion.
    ///
    /// # Arguments
    /// * `int_div` - Integer part (0-65535)
    /// * `frac_div` - Fractional part (0-255)
    pub fn set_clkdiv(&mut self, int_div: u16, frac_div: u8) {
        self.adc.div.write_int(int_div as u32);
        self.adc.div.write_frac(frac_div as u32);
    }

    /// Set clock divisor from a floating point value.
    pub fn set_clkdiv_float(&mut self, div: f32) {
        let div = div.clamp(0.0, 65536.0);
        let int_div = div as u16;
        let frac_div = ((div - int_div as f32) * 256.0) as u8;
        self.set_clkdiv(int_div, frac_div);
    }

    // ========================================================================
    // FIFO Operations
    // ========================================================================

    /// Setup the ADC FIFO.
    ///
    /// # Arguments
    /// * `enable` - Enable writing results to FIFO
    /// * `dreq_enable` - Enable DMA requests when FIFO has data
    /// * `threshold` - DMA request/IRQ threshold (0-15)
    /// * `error_in_fifo` - Include error flag in FIFO data
    /// * `byte_shift` - Shift results to one byte for byte DMA
    pub fn fifo_setup(
        &mut self,
        enable: bool,
        dreq_enable: bool,
        threshold: u8,
        error_in_fifo: bool,
        byte_shift: bool,
    ) {
        let mut fcs = self.adc.fcs.read_val();
        fcs = fcs.with_en(enable);
        fcs = fcs.with_dreq_en(dreq_enable);
        fcs = fcs.with_thresh(threshold.min(15) as u32);
        fcs = fcs.with_err(error_in_fifo);
        fcs = fcs.with_shift(byte_shift);
        self.adc.fcs.write_val(fcs);
    }

    /// Enable FIFO.
    #[inline]
    pub fn fifo_enable(&mut self) {
        self.adc.fcs.set_en();
    }

    /// Disable FIFO.
    #[inline]
    pub fn fifo_disable(&mut self) {
        self.adc.fcs.clr_en();
    }

    /// Check if FIFO is empty.
    #[inline]
    pub fn fifo_is_empty(&self) -> bool {
        self.adc.fcs.empty()
    }

    /// Check if FIFO is full.
    #[inline]
    pub fn fifo_is_full(&self) -> bool {
        self.adc.fcs.full()
    }

    /// Get FIFO level (number of entries).
    #[inline]
    pub fn fifo_level(&self) -> u8 {
        self.adc.fcs.level() as u8
    }

    /// Check if FIFO has overflowed (sticky).
    #[inline]
    pub fn fifo_has_overflowed(&self) -> bool {
        self.adc.fcs.over()
    }

    /// Clear FIFO overflow flag.
    #[inline]
    pub fn fifo_clear_overflow(&mut self) {
        self.adc.fcs.set_over();
    }

    /// Check if FIFO has underflowed (sticky).
    #[inline]
    pub fn fifo_has_underflowed(&self) -> bool {
        self.adc.fcs.under()
    }

    /// Clear FIFO underflow flag.
    #[inline]
    pub fn fifo_clear_underflow(&mut self) {
        self.adc.fcs.set_under();
    }

    /// Pop a value from the FIFO.
    ///
    /// Returns `None` if FIFO is empty.
    pub fn fifo_pop(&mut self) -> Option<u16> {
        if self.fifo_is_empty() {
            None
        } else {
            Some(self.adc.fifo.val() as u16)
        }
    }

    /// Pop a value from the FIFO, blocking until available.
    pub fn fifo_pop_blocking(&mut self) -> u16 {
        while self.fifo_is_empty() {
            core::hint::spin_loop();
        }
        self.adc.fifo.val() as u16
    }

    /// Pop a value from FIFO with error flag.
    ///
    /// Returns (value, error_flag). Only valid if error_in_fifo was enabled.
    pub fn fifo_pop_with_error(&mut self) -> Option<(u16, bool)> {
        if self.fifo_is_empty() {
            None
        } else {
            let raw = self.adc.fifo.read_bits();
            let value = (raw & 0xFFF) as u16;
            let error = (raw & 0x8000) != 0;
            Some((value, error))
        }
    }

    /// Drain the FIFO, discarding all contents.
    pub fn fifo_drain(&mut self) {
        // Wait for any in-progress conversion
        while !self.is_ready() {
            core::hint::spin_loop();
        }
        // Drain the FIFO
        while !self.fifo_is_empty() {
            let _ = self.adc.fifo.val();
        }
    }

    /// Get the FIFO address for DMA.
    #[inline]
    pub fn fifo_address(&self) -> *const u32 {
        &self.adc.fifo as *const _ as *const u32
    }

    /// Get the FIFO register address as u32 for DMA configuration.
    #[inline]
    pub fn fifo_dma_address(&self) -> u32 {
        &self.adc.fifo as *const _ as u32
    }

    /// DREQ selection for ADC DMA transfers.
    ///
    /// Use this with a DMA channel's `treq_sel` to pace reads
    /// from the ADC FIFO.
    pub const DREQ: crate::dma::TreqSel = crate::dma::TreqSel::Adc;

    /// Get the DMA DREQ number for ADC.
    #[inline]
    pub const fn dreq() -> u8 {
        ADC_DREQ
    }

    // ========================================================================
    // Interrupts
    // ========================================================================

    /// Check if FIFO interrupt is pending (raw).
    #[inline]
    pub fn fifo_irq_raw(&self) -> bool {
        self.adc.intr.fifo()
    }

    /// Enable FIFO interrupt.
    #[inline]
    pub fn enable_fifo_irq(&mut self) {
        self.adc.inte.set_fifo();
    }

    /// Disable FIFO interrupt.
    #[inline]
    pub fn disable_fifo_irq(&mut self) {
        self.adc.inte.clr_fifo();
    }

    /// Check masked interrupt status.
    #[inline]
    pub fn fifo_irq_status(&self) -> bool {
        self.adc.ints.fifo()
    }

    /// Force FIFO interrupt (for testing).
    #[inline]
    pub fn force_fifo_irq(&mut self) {
        self.adc.intf.set_fifo();
    }

    /// Clear forced FIFO interrupt.
    #[inline]
    pub fn clear_forced_fifo_irq(&mut self) {
        self.adc.intf.clr_fifo();
    }

    // ========================================================================
    // Utility Functions
    // ========================================================================

    /// Convert raw ADC value to voltage.
    #[inline]
    pub fn to_voltage(raw: u16) -> f32 {
        (raw as f32) * ADC_VREF / (ADC_MAX as f32 + 1.0)
    }

    /// Convert raw ADC value to millivolts.
    #[inline]
    pub fn to_millivolts(raw: u16) -> u32 {
        ((raw as u32) * 3300) / 4096
    }

    /// Convert raw ADC temperature value to degrees Celsius.
    ///
    /// Formula: T = 27 - (ADC_voltage - 0.706) / 0.001721
    #[inline]
    pub fn convert_temp_to_celsius(raw: u16) -> f32 {
        let voltage = (raw as f32) * ADC_VREF / (ADC_MAX as f32 + 1.0);
        27.0 - (voltage - 0.706) / 0.001721
    }

    /// Release the ADC peripheral.
    ///
    /// Returns the underlying PAC peripheral for reuse.
    #[inline]
    pub fn release(self) -> ADC {
        self.adc
    }
}

// ============================================================================
// Debug implementation
// ============================================================================

impl core::fmt::Debug for Adc {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Adc")
            .field("temp_enabled", &self.temp_enabled)
            .field("selected_channel", &self.selected_channel())
            .field("is_ready", &self.is_ready())
            .finish()
    }
}
