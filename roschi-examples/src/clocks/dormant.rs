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

use defmt::*;
use defmt_rtt as _;

use roschi::Delay;
use roschi::block::ImageDef;
use roschi::clocks::ClockBuilder;
use roschi::entry;
use roschi::gpio::{EdgeLevel, Pins};
use roschi::pac::Peripherals;
use roschi::rt::peripheral::Peripherals as CorePeripherals;

/// RP2350 Boot Block
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::exe();

#[entry]
fn main() -> ! {
    info!("Dormant mode example starting...");

    // Take device peripherals
    let dp = Peripherals::take().unwrap();

    // Initialize clocks (150 MHz sys by default)
    let mut clocks = ClockBuilder::new().build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);

    // Initialize GPIO pins
    let pins = Pins::new(dp.IO_BANK0, dp.PADS_BANK0);
    let led = pins.pin::<25>();
    led.into_output();

    // Configure button as input with pull-up and dormant wake on falling edge
    let button = pins.pin::<15>();
    button.into_input_pullup();
    // Configure button to wake from dormant on falling edge
    button.set_dormant_wake(EdgeLevel::EdgeLow);

    // Initialize delay
    let core = CorePeripherals::take().unwrap();
    let delay = Delay::new(core.SYST, clocks.sys_freq());

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
