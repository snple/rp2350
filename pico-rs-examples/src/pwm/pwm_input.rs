//! PWM Input Mode Example for RP2350
//!
//! This example demonstrates the three most practical ways to use PWM input mode:
//!
//! 1. Count edges over a known gate interval and convert that count into frequency
//! 2. Count HIGH-time ticks in gated mode and estimate duty cycle
//! 3. Leave the slice running as a continuous pulse counter
//!
//! # What PWM Input Mode Actually Does
//!
//! The RP2350 PWM peripheral does not turn into a timestamp capture block in
//! input mode. Instead, it keeps using the slice counter and changes what causes
//! that counter to advance:
//!
//! - `RisingEdge`: increment once per rising edge on channel B
//! - `FallingEdge`: increment once per falling edge on channel B
//! - `GatedByB`: advance at the divided PWM clock rate only while channel B is HIGH
//!
//! That means the interpretation of the final counter value depends on the mode:
//!
//! - In edge-counting mode, the count is an edge total
//! - In gated mode, the count is accumulated HIGH time in counter ticks
//!
//! # Hardware Setup
//!
//! - Connect the signal to measure to GPIO 1 (PWM slice 0, channel B)
//! - For frequency measurement, feed in a square wave or tach-like pulse train
//! - For duty-cycle measurement, feed in a PWM signal whose HIGH ratio you want
//!   to estimate
//!
//! # Measurement Strategy
//!
//! Two parameters dominate the quality of the result:
//!
//! - Gate time: longer gate times give more stable counts but slower updates
//! - Divider: smaller dividers give finer time resolution in `GatedByB` mode but
//!   can overflow the 16-bit counter sooner
//!
//! For edge counting, choose a gate time long enough to accumulate several
//! pulses. For gated duty measurement, choose the fastest divider that still
//! keeps the counter from wrapping during the gate interval.
//!
//! # Overflow
//!
//! The counter is still only 16 bits wide. If it wraps, the reported count is no
//! longer the full measurement result unless the application also tracks wraps.
//! This example checks the wrap flag after each measurement to show when that
//! happens.
//!
//! # Type-Safe Pin Ownership
//!
//! The PWM API uses type-safe pin ownership: the B-channel pin is consumed when
//! creating the PWM input, preventing accidental reuse by some other peripheral.

#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;

use pico_rs::Peripherals;
use pico_rs::gpio::Pins;
use pico_rs::main;
use pico_rs::pwm::{InputMode, Pwm, calc_duty_cycle, calc_input_frequency};
use pico_rs::timer::Timer0;

#[main]
fn main() -> ! {
    info!("=== PWM Input Mode Example ===");

    // Get peripherals
    let dp = Peripherals::take().unwrap();

    // Initialize timer for delays
    let timer = Timer0::init(dp.TIMER0);

    // Initialize GPIO and get pin ownership
    let pins = Pins::init(dp.IO_BANK0, dp.PADS_BANK0);

    // Also configure LED for visual feedback
    let mut led = pins.pin::<25>();
    led.into_output();

    // Boot clock frequency (before PLL setup)
    let sys_clock_hz: u32 = 12_000_000;

    // ========================================================================
    // Example 1: Frequency Measurement (Edge Counting)
    // ========================================================================

    info!("--- Frequency Measurement Mode ---");
    info!("PWM input configured on GPIO 1 (PWM slice 0, channel B)");

    // Create PWM controller (owns the peripheral and slice tokens)
    let pwm = Pwm::init(dp.PWM);

    // Create PWM input using pin ownership
    // GPIO 1 maps to PWM slice 0, channel B
    let mut slice0 = pwm
        .slice::<0>()
        .input(pins.pin::<1>(), InputMode::RisingEdge);

    info!("Counting rising edges for 100ms...");

    // Measure frequency over 100ms gate time
    let gate_time_us: u64 = 100_000; // 100ms

    // Start measurement
    slice0.start_measurement();
    timer.delay_us(gate_time_us);
    let edge_count = slice0.stop_measurement();
    let wrapped = slice0.overflow_occurred();

    // In RisingEdge mode, the counter value is an edge count over the gate interval.
    let frequency_hz = calc_input_frequency(edge_count, gate_time_us as u32);

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

    // In gated mode, the counter advances at the PWM counter clock only while
    // the input is HIGH. With div=1, that counter clock is the system clock.
    let measure_time_us: u64 = 10_000; // 10ms

    info!("Measuring duty cycle for 10ms...");

    slice0.start_measurement();
    timer.delay_us(measure_time_us);
    let high_count = slice0.stop_measurement() as u32;

    // Total available ticks are based on system clock and divider.
    // At 12 MHz boot clock with div=1: total_count ≈ 12 MHz * 10 ms = 120,000.
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

        let current_count = slice0.counter();

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
