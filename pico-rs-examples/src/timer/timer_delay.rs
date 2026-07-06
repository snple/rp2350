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

use pico_rs::clocks::ClockBuilder;
use pico_rs::gpio::Pins;
use pico_rs::main;
use pico_rs::pac::Peripherals;
use pico_rs::ticks::Ticks;
use pico_rs::timer::Timer0;

#[main]
fn main() -> ! {
    info!("=== Timer Delay Test ===");

    // Take device peripherals
    let dp = Peripherals::take().unwrap();

    // Initialize clock system (150 MHz sys, 48 MHz USB)
    let clocks = ClockBuilder::new().build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);
    info!("Clock system initialized: sys={} Hz", clocks.sys_freq());

    // Initialize TIMER0 tick generator (1µs ticks from clk_ref)
    let ticks = Ticks::new(dp.TICKS, &clocks);
    ticks.init_timer0();

    // Initialize the hardware timer
    let timer = Timer0::init(dp.TIMER0);
    info!("Timer0 initialized");

    // Show initial timer value
    let start_time = timer.get_time_us();
    info!("Initial timer value: {} us", start_time);

    // Initialize GPIO for LED
    let pins = Pins::new(dp.IO_BANK0, dp.PADS_BANK0);
    let led = pins.pin::<25>();
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
