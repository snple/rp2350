//! ADC Example for Pico 2
//!
//! Demonstrates the type-safe ADC API:
//! - Pin ownership: `into_adc_input()` consumes the pin
//! - Type-safe reading: `adc.read(&input)` requires a configured input
//! - Temperature sensor: `adc.read_temp()` for on-chip temperature
//!
//! Wire a potentiometer or voltage source (0-3.3V) to GPIO 26 (ADC channel 0).

#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;

use rp2350::adc::{Adc, AdcPin};
use rp2350::clocks::ClockBuilder;
use rp2350::gpio::Pins;
use rp2350::main;
use rp2350::pac::Peripherals;

#[main]
fn main() -> ! {
    info!("RP2350 started!");
    info!("ADC Type-Safe Example...");

    // Take device peripherals from PAC
    let dp = Peripherals::take().unwrap();

    // Initialize clock system (150 MHz sys, 48 MHz USB)
    let clocks = ClockBuilder::new()
        .enable_usb()
        .enable_adc()
        .build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);
    info!(
        "Clock system initialized: sys={} Hz, usb={} Hz",
        clocks.sys_freq(),
        clocks.usb_freq()
    );

    // Initialize GPIO pins
    let pins = Pins::init(dp.IO_BANK0, dp.PADS_BANK0);

    // Initialize SysTick-based delay (matches actual system clock)
    let delay = rp2350_examples::common::board::init_delay(&clocks);

    info!("Initializing ADC...");

    // Create ADC controller
    let mut adc = Adc::init(dp.ADC);
    info!("ADC initialized");

    // Configure pin for ADC - this consumes the pin!
    // The pin cannot be used for other purposes while AdcInput exists.
    #[cfg(not(feature = "rp2350b"))]
    let input0 = pins.pin::<26>().into_adc_input(); // RP2350A: GPIO 26 = ch 0
    #[cfg(feature = "rp2350b")]
    let input0 = pins.pin::<40>().into_adc_input(); // RP2350B: GPIO 40 = ch 0
    info!("Configured ADC input on channel {}", input0.channel());

    // Trying to use pins.pin::<26>() again would be a compile error:
    // pins.pin::<26>().into_output();  // ERROR: use of moved value

    info!("Starting readings...");

    loop {
        // Read from the configured input - type-safe!
        // Cannot accidentally read from an unconfigured channel.
        let raw = adc.read(&input0);
        let voltage_mv = Adc::to_millivolts(raw);

        info!(
            "ADC Ch{}: raw={}, voltage={}mV",
            input0.channel(),
            raw,
            voltage_mv
        );

        // Read temperature (doesn't need a pin)
        let temp_raw = adc.read_temp();
        let temp_c = Adc::convert_temp_to_celsius(temp_raw);

        // Format temperature with integer and decimal parts
        let temp_int = temp_c as i32;
        let temp_frac = ((temp_c - temp_int as f32).abs() * 10.0) as u32;

        info!(
            "Temperature: raw={}, temp={}.{}C",
            temp_raw, temp_int, temp_frac
        );

        delay.delay_ms(1000);
    }
}
