//! # ADC Example
//!
//! Demonstrates using the RP2350's ADC to:
//! - Read analog inputs
//! - Read the on-chip temperature sensor
//! - Configure FIFO for continuous sampling
//!
//! ## Hardware Setup
//!
//! Connect a potentiometer or analog sensor to:
//! - RP2350A: GPIO 26 (ADC0)
//! - RP2350B: GPIO 40 (ADC0)
//!
//! The on-chip temperature sensor doesn't require external connections.

#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

use pico_rs::adc::{ADC_BASE_PIN, Adc};
use pico_rs::block::ImageDef;
use pico_rs::entry;
use pico_rs::gpio::{Function, GpioBank, Pull};
use pico_rs::systick::Delay;

/// RP2350 Boot Block
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::secure_exe();

/// Clock frequency at boot (ROSC ~12 MHz)
const CLOCK_FREQ_HZ: u32 = 12_000_000;

#[entry]
fn main() -> ! {
    info!("=== RP2350 ADC Example ===");

    // Initialize GPIO for ADC pins
    let gpio = GpioBank::init();

    // Configure ADC pin for channel 0
    // For RP2350A: GPIO 26, for RP2350B: GPIO 40
    info!("ADC base pin: GPIO {}", ADC_BASE_PIN);

    // Set the ADC pin to NULL function (high-Z) and disable pulls
    // This example assumes RP2350A (GPIO 26) by default.
    // For RP2350B, enable the rp2350b feature and this will use GPIO 40.
    #[cfg(not(feature = "rp2350b"))]
    {
        let adc_pin = gpio.pin26();
        adc_pin.set_function(Function::Null);
        adc_pin.set_pull(Pull::None);
    }
    #[cfg(feature = "rp2350b")]
    {
        let adc_pin = gpio.pin40();
        adc_pin.set_function(Function::Null);
        adc_pin.set_pull(Pull::None);
    }

    // Initialize ADC
    let adc = Adc::init();
    info!("ADC initialized");

    // Initialize delay
    let mut delay = Delay::new(CLOCK_FREQ_HZ);

    // ========================================================================
    // Part 1: Single-shot ADC reading
    // ========================================================================
    info!("\n--- Single-shot ADC readings ---");

    for i in 0..5 {
        // Read channel 0 (first analog input)
        let raw = adc.read_channel(0);
        let voltage_mv = Adc::to_millivolts(raw);
        info!("Reading {}: Raw={}, Voltage={}mV", i + 1, raw, voltage_mv);

        delay.delay_ms(500);
    }

    // ========================================================================
    // Part 2: Temperature sensor reading
    // ========================================================================
    info!("\n--- Temperature Sensor ---");

    adc.enable_temperature_sensor();
    delay.delay_ms(10); // Let sensor stabilize

    for i in 0..5 {
        let raw = adc.read_temperature_raw();
        // Note: convert_temperature requires floating point
        // For integer math: T_C ≈ 27 - (raw * 3300 / 4096 - 706) / 1721 * 1000
        let voltage_mv = Adc::to_millivolts(raw);

        // Simplified integer temperature calculation
        // T = 27 - (V - 0.706) / 0.001721
        // With V in mV: T = 27 - (V_mV - 706) / 1.721
        let temp_c_x10 = 270i32 - ((voltage_mv as i32 - 706) * 10 / 17);

        info!(
            "Temp {}: Raw={}, Voltage={}mV, Temp=~{}.{}C",
            i + 1,
            raw,
            voltage_mv,
            temp_c_x10 / 10,
            (temp_c_x10 % 10).abs()
        );

        delay.delay_ms(1000);
    }

    adc.disable_temperature_sensor();

    // ========================================================================
    // Part 3: Round-robin sampling
    // ========================================================================
    info!("\n--- Round-robin sampling (CH0 + Temp) ---");

    // Enable round-robin for channel 0 and temperature (last channel)
    let temp_channel = pico_rs::adc::TEMPERATURE_CHANNEL;
    let channel_mask = (1 << 0) | (1 << temp_channel);
    adc.set_round_robin(channel_mask);
    adc.enable_temperature_sensor();

    for i in 0..10 {
        let raw = adc.read();
        let channel = adc.get_selected_channel();
        let voltage_mv = Adc::to_millivolts(raw);

        if channel == temp_channel {
            info!("Sample {}: Temp sensor, {}mV", i + 1, voltage_mv);
        } else {
            info!("Sample {}: CH{}, {}mV", i + 1, channel, voltage_mv);
        }

        delay.delay_ms(200);
    }

    adc.disable_round_robin();

    // ========================================================================
    // Part 4: FIFO mode (continuous sampling)
    // ========================================================================
    info!("\n--- FIFO continuous sampling ---");

    // Setup FIFO: enable, no DMA, threshold 4, no error in FIFO, no shift
    adc.fifo_setup(true, false, 4, false, false);

    // Set slow sample rate (about 10 samples/sec at 48 MHz ADC clock)
    // Divisor calculation: ADC clock is typically 48 MHz
    // For 10 S/s: div = 48_000_000 / 10 = 4_800_000
    // But we're at 12 MHz boot clock, so ADC clock is different
    // Just use a reasonable divisor for demonstration
    adc.set_clkdiv(65535, 0); // Very slow for demo

    // Select channel 0
    adc.select_channel(0);

    // Start free-running
    adc.start_free_running();

    // Read some samples from FIFO
    for i in 0..10 {
        // Wait for data
        if let Some(raw) = adc.fifo_pop() {
            let voltage_mv = Adc::to_millivolts(raw);
            info!("FIFO sample {}: Raw={}, {}mV", i + 1, raw, voltage_mv);
        } else {
            // Wait and try again
            delay.delay_ms(100);
            if let Some(raw) = adc.fifo_pop() {
                let voltage_mv = Adc::to_millivolts(raw);
                info!("FIFO sample {}: Raw={}, {}mV", i + 1, raw, voltage_mv);
            }
        }
    }

    adc.stop_free_running();
    adc.fifo_drain();

    info!("\n=== ADC Example Complete ===");

    // Keep reading and displaying
    info!("Continuous reading from CH0...");
    loop {
        let raw = adc.read_channel(0);
        let voltage_mv = Adc::to_millivolts(raw);
        info!("CH0: {}mV", voltage_mv);
        delay.delay_ms(1000);
    }
}
