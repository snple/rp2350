//! # Dormant Mode Example
//!
//! Demonstrates entering dormant (ultra-low power) mode and waking up via GPIO.
//!
//! Hardware:
//! - LED on GPIO 25 (on-board LED)
//! - Button on GPIO 15 with pull-up (active low)
//!
//! Behavior:
//! - Blinks LED briefly
//! - Enters dormant mode
//! - Wakes on button press (falling edge)
//! - Blinks LED again after wake

#![no_std]
#![no_main]

#[cfg(feature = "defmt")]
use defmt_rtt as _;
use rp2350_examples::log::*;

use rp2350::clocks::ClockBuilder;
use rp2350::gpio::{EdgeLevel, Pins};
use rp2350::main;
use rp2350::pac::Peripherals;

#[main]
fn main() -> ! {
    info!("Dormant mode example starting...");

    // Take device peripherals
    let dp = Peripherals::take().unwrap();

    // Initialize clocks (150 MHz sys by default)
    let mut clocks = ClockBuilder::new().build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);

    // Initialize GPIO pins
    let pins = Pins::init(dp.IO_BANK0, dp.PADS_BANK0);
    let mut led = pins.pin::<25>();
    led.into_output();

    // Configure button as input with pull-up and dormant wake on falling edge
    let mut button = pins.pin::<15>();
    button.into_input_pullup();
    // Configure button to wake from dormant on falling edge
    button.set_dormant_wake(EdgeLevel::EdgeLow);

    // Initialize delay (matches actual system clock)
    let delay = rp2350_examples::common::board::init_delay(&clocks);

    let mut wake_count: u32 = 0;

    loop {
        // Indicate we are about to sleep
        info!("Wake count: {}", wake_count);
        led.set_high();
        delay.delay_ms(1000);
        led.set_low();
        delay.delay_ms(1000);

        info!("Entering dormant mode. Press GPIO15 to wake.");

        // Clear any stale edge interrupt BEFORE entering dormant.
        // The INTR register is shared between proc0 interrupts and dormant_wake.
        // If a previous edge flag is still set, DORMANT_WAKE_INTS = (INTR & DORMANT_WAKE_INTE)
        // would be non-zero, causing the oscillator to wake immediately.
        button.clear_interrupt(EdgeLevel::EdgeLow);

        // Enter dormant mode (CPU stops here until wake event)
        clocks.dormant();

        // NOTE: Do NOT call clear_dormant_wake() here — that clears DORMANT_WAKE_INTE
        // (the enable register), which would disable the wake source for subsequent
        // dormant entries. The raw edge status in INTR is cleared above before
        // the next dormant entry instead.

        wake_count += 1;
        info!("Woke from dormant mode (count={}).", wake_count);

        // Blink a few times after wake
        for _ in 0..3 {
            led.toggle();
            delay.delay_ms(150);
        }
    }
}
