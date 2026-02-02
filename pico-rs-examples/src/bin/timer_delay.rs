//! # Timer Delay Example
//!
//! Test the hardware timer delay functionality.
//! Uses the 64-bit microsecond timer for precise delays.
//!
//! The RP2350 hardware timer runs at 1 MHz (1 tick = 1 microsecond),
//! independent of the CPU clock frequency.

#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

use pico_rs::block::ImageDef;
use pico_rs::entry;
use pico_rs::gpio::GpioBank;
use pico_rs::timer::Timer0;

/// RP2350 Boot Block
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::secure_exe();

#[entry]
fn main() -> ! {
    info!("=== Timer Delay Test ===");

    // Initialize the hardware timer
    let timer = Timer0::init();
    info!("Timer0 initialized");

    // Show initial timer value
    let start_time = timer.get_time_us();
    info!("Initial timer value: {} us", start_time);

    // Initialize GPIO for LED
    let gpio = GpioBank::init();
    let led = gpio.pin25();
    led.into_output();
    info!("LED configured on GPIO 25");

    info!("Testing delay accuracy...");
    info!("LED should blink at 1 Hz (500ms on, 500ms off)");

    let mut blink_count: u32 = 0;
    let test_start = timer.get_time_us();

    loop {
        let loop_start = timer.get_time_us();

        // LED ON
        led.set_high();
        timer.delay_ms(500);

        // LED OFF
        led.set_low();
        timer.delay_ms(500);

        blink_count += 1;

        // Every 5 blinks, report timing accuracy
        if blink_count.is_multiple_of(5) {
            let elapsed = timer.get_time_us() - test_start;
            let expected = blink_count as u64 * 1_000_000; // 1 second per blink
            let drift_us = elapsed.abs_diff(expected);

            info!(
                "Blink #{}: elapsed={} us, expected={} us, drift={} us",
                blink_count, elapsed, expected, drift_us
            );

            // Also show single cycle timing
            let cycle_time = timer.get_time_us() - loop_start;
            info!("  Last cycle: {} us (should be ~1000000 us)", cycle_time);
        }
    }
}
