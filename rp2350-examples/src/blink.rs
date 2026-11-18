//! # Blink Example
//!
//! Blink the on-board LED on a Raspberry Pi Pico 2 (RP2350).
//! The LED is connected to GPIO 25.

#![no_std]
#![no_main]

#[cfg(feature = "defmt")]
use defmt_rtt as _;
use rp2350_examples::log::*;

use rp2350::Delay;
use rp2350::gpio::Pins;
use rp2350::main;
use rp2350::pac::Peripherals;
use rp2350::rt::peripheral::Peripherals as CorePeripherals;

/// At boot, RP2350 runs from ROSC at approximately 150 MHz
const CLOCK_FREQ_HZ: u32 = 150_000_000;

const LED_PIN: u32 = 25;

#[main]
fn main() -> ! {
    info!("RP2350 started!");
    info!("Blink example with GPIO LED control...");

    // Take device peripherals from PAC
    let dp = Peripherals::take().unwrap();

    // Initialize GPIO pins by consuming IO_BANK0 and PADS_BANK0
    let pins = Pins::init(dp.IO_BANK0, dp.PADS_BANK0);

    // Configure GPIO 25 (LED) as output
    let mut led = pins.pin::<LED_PIN>();
    led.into_output();

    info!("GPIO {} configured as output", LED_PIN);

    // Take core peripherals (NVIC, SCB, SYST, MPU, FPU, SAU)
    let core_peripherals = CorePeripherals::take().unwrap();

    // Initialize SysTick-based delay
    let delay = Delay::init(core_peripherals.SYST, CLOCK_FREQ_HZ);

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
