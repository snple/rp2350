//! # Minimal XOSC Test
//!
//! Test just XOSC initialization without PLL.

#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

use pico_rs::Delay;
use pico_rs::block::ImageDef;
use pico_rs::entry;
use pico_rs::regs::{Gpio, Xosc, gpio};

/// RP2350 Boot Block
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::secure_exe();

/// Still using ROSC (12 MHz) for the test
const CLOCK_FREQ_HZ: u32 = 12_000_000;

/// LED pin
const LED_PIN: u8 = 25;

#[entry]
fn main() -> ! {
    info!("RP2350 started!");
    info!("Testing XOSC initialization...");

    // Initialize GPIO first
    gpio::init();
    let led = Gpio::new(LED_PIN);
    led.init();
    led.into_output();
    info!("GPIO {} configured", LED_PIN);

    // Flash LED to show we're alive
    led.set_high();

    // Try XOSC init
    info!("Initializing XOSC...");
    Xosc::init(47);
    info!("XOSC initialized!");

    // Initialize SysTick delay
    let mut delay = Delay::new(CLOCK_FREQ_HZ);

    let mut counter: u32 = 0;
    loop {
        led.toggle();
        counter = counter.wrapping_add(1);

        if counter % 2 == 0 {
            info!("XOSC test: count = {}", counter / 2);
        }

        delay.delay_ms(500);
    }
}
