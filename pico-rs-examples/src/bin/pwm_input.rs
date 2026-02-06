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
//! - RisingEdge: Counter increments on each rising edge (for frequency)
//! - FallingEdge: Counter increments on each falling edge
//!
//! # Type-Safe Pin Ownership
//! The PWM API uses type-safe pin ownership - the pin is consumed
//! when creating the PWM input, preventing accidental reuse.

#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

use pico_rs::Peripherals;
use pico_rs::block::ImageDef;
use pico_rs::entry;
use pico_rs::gpio::Pins;
use pico_rs::pwm::{InputMode, Pwm, calc_duty_cycle};
use pico_rs::timer::Timer0;

/// RP2350 Boot Block - tells the Boot ROM this is a valid ARM executable
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::secure_exe();

#[entry]
fn main() -> ! {
    info!("=== PWM Input Mode Example ===");

    // Get peripherals
    let dp = Peripherals::take().unwrap();

    // Initialize timer for delays
    let timer = Timer0::init(dp.TIMER0);

    // Initialize GPIO and get pin ownership
    let pins = Pins::new(dp.IO_BANK0, dp.PADS_BANK0);

    // Also configure LED for visual feedback
    let led = pins.p25;
    led.into_output();

    // Boot clock frequency (before PLL setup)
    let sys_clock_hz: u32 = 12_000_000;

    // ========================================================================
    // Example 1: Frequency Measurement (Edge Counting)
    // ========================================================================

    info!("--- Frequency Measurement Mode ---");
    info!("PWM input configured on GPIO 1 (PWM slice 0, channel B)");

    // Create PWM controller (owns the peripheral and slice tokens)
    let pwm = Pwm::new(dp.PWM);

    // Create PWM input using pin ownership - consumes pwm.slice0 token
    // GPIO 1 maps to PWM slice 0, channel B
    let slice0 = pwm
        .slice0
        .input(pins.p1, InputMode::RisingEdge, sys_clock_hz);

    info!("Counting rising edges for 100ms...");

    // Measure frequency over 100ms gate time
    let gate_time_us: u64 = 100_000; // 100ms

    // Start measurement
    slice0.start_measurement();
    timer.delay_us(gate_time_us);
    let edge_count = slice0.stop_measurement();
    let wrapped = slice0.overflow_occurred();

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

    // Switch to duty cycle measurement mode
    slice0.set_div_mode(pico_rs::pwm::DivMode::GatedByB);
    slice0.set_div(1, 0); // Fastest counting (sys_clk rate)

    // For duty cycle measurement, we need to know the input signal period
    // Method: Run for a known time and calculate ratio
    let measure_time_us: u64 = 10_000; // 10ms

    info!("Measuring duty cycle for 10ms...");

    slice0.start_measurement();
    timer.delay_us(measure_time_us);
    let high_count = slice0.stop_measurement() as u32;

    // Total count is based on system clock and divider
    // At 12MHz boot clock with div=1: total_count ≈ 12MHz * 10ms = 120,000
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

    // Or use the helper function
    let duty_calc = calc_duty_cycle(high_count as u16, sys_clock_hz, 1, measure_time_us as u32);
    info!("Calculated duty (helper): {}%", duty_calc);

    led.toggle();
    timer.delay_ms(500);

    // ========================================================================
    // Example 3: Continuous Pulse Counter
    // ========================================================================

    info!("--- Continuous Pulse Counter Mode ---");

    // Switch back to edge counting mode
    slice0.set_div_mode(pico_rs::pwm::DivMode::RisingEdgeB);

    info!("Counting pulses continuously...");

    // Start continuous counting
    slice0.reset_counter();
    slice0.clear_interrupt();
    slice0.enable();

    let mut last_count: u16 = 0;
    let mut total_pulses: u32 = 0;

    loop {
        timer.delay_ms(1000);

        let current_count = slice0.get_counter();

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
        if slice0.did_wrap() {
            slice0.clear_interrupt();
            warn!("Counter wrapped!");
        }
    }
}
