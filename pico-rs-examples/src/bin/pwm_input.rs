//! PWM Input Mode Example for RP2350
//!
//! This example demonstrates how to use PWM input mode to measure:
//! 1. Frequency of an external signal (edge counting)
//! 2. Duty cycle of an external PWM signal (gated counting)
//!
//! # Hardware Setup
//! - Connect the signal to measure to GPIO 1 (PWM0 B input)
//! - For frequency measurement: any square wave signal
//! - For duty cycle measurement: PWM signal to analyze
//!
//! # PWM Input Modes
//! The RP2350 PWM supports three input modes on the B channel:
//! - GatedByB: Counter runs only when B is high (for duty cycle)
//! - RisingEdgeB: Counter increments on each rising edge (for frequency)
//! - FallingEdgeB: Counter increments on each falling edge

#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

use pico_rs::block::ImageDef;
use pico_rs::entry;
use pico_rs::gpio::GpioBank;
use pico_rs::pwm::{DivMode, Pwm};
use pico_rs::timer::Timer0;

/// RP2350 Boot Block - tells the Boot ROM this is a valid ARM executable
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::secure_exe();

#[entry]
fn main() -> ! {
    info!("=== PWM Input Mode Example ===");

    // Initialize timer for delays
    let timer = Timer0::init();

    // Initialize GPIO
    let gpio = GpioBank::init();

    // Configure GPIO 1 as PWM function - this will be our input pin
    // GPIO 1 maps to PWM0 B channel
    let input_pin = gpio.pin1();
    input_pin.set_function(pico_rs::gpio::Function::Pwm);

    // Also configure LED for visual feedback
    let led = gpio.pin25();
    led.into_output();

    // Initialize PWM
    let pwm = Pwm::init();
    let slice = pwm.slice(0); // PWM slice 0 for GPIO 1

    info!("PWM input configured on GPIO 1 (PWM0 B)");

    // ========================================================================
    // Example 1: Frequency Measurement (Edge Counting)
    // ========================================================================

    info!("--- Frequency Measurement Mode ---");

    // Configure for rising edge counting
    slice.set_div_mode(DivMode::RisingEdgeB);
    slice.set_wrap(0xFFFF); // Max wrap to avoid overflow
    slice.set_counter(0);
    slice.clear_interrupt();
    slice.enable();

    info!("Counting rising edges for 100ms...");

    // Measure frequency over 100ms gate time
    let gate_time_us: u64 = 100_000; // 100ms

    slice.set_counter(0);
    timer.delay_us(gate_time_us);
    let edge_count = slice.get_counter();
    let wrapped = slice.did_wrap();

    // Calculate frequency
    // freq = edges / gate_time_seconds = edges * 1_000_000 / gate_time_us
    let frequency_hz = if edge_count > 0 {
        (edge_count as u64 * 1_000_000 / gate_time_us) as u32
    } else {
        0
    };

    info!("Edge count: {}", edge_count);
    info!("Measured frequency: {} Hz", frequency_hz);
    if wrapped {
        warn!("Counter wrapped - frequency too high or gate time too long");
    }

    led.toggle();
    timer.delay_ms(500);

    // ========================================================================
    // Example 2: Duty Cycle Measurement (Gated Counting)
    // ========================================================================

    info!("--- Duty Cycle Measurement Mode ---");

    // Configure for duty cycle measurement
    // Counter runs at sys_clk rate only when B input is HIGH
    slice.disable();
    slice.set_div_mode(DivMode::GatedByB);
    slice.set_div(1, 0); // Fastest counting (sys_clk rate)
    slice.set_wrap(0xFFFF);
    slice.set_counter(0);
    slice.clear_interrupt();
    slice.enable();

    // For duty cycle measurement, we need to know the input signal period
    // Method: Run for a known time and calculate ratio
    let measure_time_us: u64 = 10_000; // 10ms - adjust based on expected input frequency

    info!("Measuring duty cycle for 10ms...");

    slice.set_counter(0);
    timer.delay_us(measure_time_us);
    let high_count = slice.get_counter() as u32;

    // Total count is based on system clock and divider
    // At 12MHz boot clock with div=1: total_count ≈ 12MHz * 10ms = 120,000
    // Note: This is approximate - for accurate duty cycle, synchronize to input period
    let sys_clock_hz: u32 = 12_000_000; // Boot clock frequency
    let expected_total = (sys_clock_hz as u64 * measure_time_us / 1_000_000) as u32;

    // Calculate duty cycle percentage (approximate)
    let duty_percent = if expected_total > 0 {
        ((high_count as u64 * 100) / expected_total as u64) as u8
    } else {
        0
    };

    info!("High count: {}", high_count);
    info!("Expected total: {}", expected_total);
    info!("Approximate duty cycle: {}%", duty_percent);

    led.toggle();
    timer.delay_ms(500);

    // ========================================================================
    // Example 3: Continuous Pulse Counter
    // ========================================================================

    info!("--- Continuous Pulse Counter Mode ---");

    // Configure for rising edge counting
    slice.disable();
    slice.set_div_mode(DivMode::RisingEdgeB);
    slice.set_wrap(0xFFFF);
    slice.set_counter(0);
    slice.clear_interrupt();
    slice.enable();

    info!("Counting pulses continuously...");

    let mut last_count: u16 = 0;
    let mut total_pulses: u32 = 0;

    loop {
        timer.delay_ms(1000);

        let current_count = slice.get_counter();

        // Handle wrap-around
        let delta = if current_count >= last_count {
            current_count - last_count
        } else {
            // Counter wrapped
            (0xFFFF - last_count) + current_count + 1
        };

        total_pulses += delta as u32;
        last_count = current_count;

        info!("Pulses in last second: {}, Total: {}", delta, total_pulses);

        // Toggle LED to show activity
        led.toggle();

        // Check for overflow
        if slice.did_wrap() {
            slice.clear_interrupt();
            warn!("Counter wrapped!");
        }
    }
}
