//! # Blink Example
//!
//! Blink the on-board LED on a Raspberry Pi Pico 2 (RP2350).
//! The LED is connected to GPIO 25.

#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

use pico_rs::Delay;
use pico_rs::block::ImageDef;
use pico_rs::entry;
use pico_rs::gpio::{self, Pin25};

/// RP2350 Boot Block - tells the Boot ROM this is a valid ARM executable
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::secure_exe();

/// RP2350 default boot clock frequency
/// At boot, RP2350 runs from ROSC at approximately 12 MHz
const CLOCK_FREQ_HZ: u32 = 12_000_000;

#[entry]
fn main() -> ! {
    info!("RP2350 started!");
    info!("Blink example with GPIO LED control...");

    // Initialize GPIO subsystem (unresets IO_BANK0 and PADS_BANK0)
    gpio::init_gpio();

    // Configure GPIO 25 (LED) as output
    let led = Pin25::new();
    led.into_output();

    info!("GPIO 25 configured as output");

    // Initialize SysTick-based delay
    let mut delay = Delay::new(CLOCK_FREQ_HZ);

    let mut led_on = false;
    loop {
        // Toggle LED
        led.toggle();
        led_on = !led_on;

        if led_on {
            info!("LED ON");
        } else {
            info!("LED OFF");
        }

        // Delay for 500ms
        delay.delay_ms(500);
    }
}
