//! GPIO HAL Example for RP2350
//!
//! This example demonstrates the new type-safe GPIO API with dedicated pin types.
//!
//! Features demonstrated:
//! - Pins::new() initialization
//! - Pin ownership model
//! - SIO function (direct CPU control)
//! - Output/Input configuration
//! - Pull resistor configuration
//! - Drive strength and slew rate
//!
//! Run with: cargo run --release --bin gpio_example

#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use panic_probe as _;
use pico_rs::Delay;
use pico_rs::block::ImageDef;
use pico_rs::entry;
use pico_rs::gpio::{DriveStrength, Pin, Pins, Pull, SlewRate};
use pico_rs::pac::Peripherals;
use pico_rs::pico_rs_rt::peripheral::Peripherals as CorePeripherals;
use pico_rs::sio::Sio;

/// RP2350 Boot Block
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::secure_exe();

const CLOCK_FREQ_HZ: u32 = 12_000_000;

#[entry]
fn main() -> ! {
    info!("=== GPIO HAL Example ===");

    // Take device peripherals
    let dp = Peripherals::take().unwrap();

    // Initialize GPIO bank using Pins::new() - consumes IO_BANK0 and PADS_BANK0
    let pins = Pins::new(dp.IO_BANK0, dp.PADS_BANK0);
    info!("GPIO bank initialized");

    // Take core peripherals for Delay
    let core_peripherals = CorePeripherals::take().unwrap();

    // Initialize delay (12MHz)
    let mut delay = Delay::new(core_peripherals.SYST, CLOCK_FREQ_HZ);

    // Get individual pins from the Pins struct
    let led = pins.p25;
    let pin0 = pins.p0;
    let pin1 = pins.p1;
    let pin2 = pins.p2;

    // ========================================================================
    // Example 1: Basic LED Blink
    // ========================================================================
    info!("--- Example 1: Basic LED on Pin25 ---");

    // Configure LED pin as output using the convenience method
    led.into_output();
    info!("Pin25 configured as SIO output");

    // Demonstrate output operations
    led.set_high();
    info!("LED set high");

    delay.delay_ms(100);

    led.set_low();
    info!("LED set low");

    led.toggle();
    info!("LED toggled (now high)");

    // Read back output level
    info!("Output level: {}", led.output_level());
    info!("Is output: {}", led.is_output());

    // ========================================================================
    // Example 2: Input with Pull Resistors
    // ========================================================================
    info!("--- Example 2: Input with Pull Resistors ---");

    // Configure pin0 as input with pull-up
    pin0.into_input_pullup();
    info!("Pin0 configured as input with pull-up");

    // Read input value
    let value = pin0.read();
    info!("Pin0 input value: {} (expected true with pull-up)", value);

    // Configure pin1 as input with pull-down
    pin1.into_input_pulldown();
    info!("Pin1 configured as input with pull-down");

    let value = pin1.read();
    info!(
        "Pin1 input value: {} (expected false with pull-down)",
        value
    );

    // ========================================================================
    // Example 3: Pad Configuration
    // ========================================================================
    info!("--- Example 3: Pad Configuration ---");

    // Configure pin2 with custom pad settings
    pin2.init_pad();
    pin2.set_function_sio();
    pin2.set_direction_out();

    // Set drive strength
    pin2.set_drive_strength(DriveStrength::_12mA);
    info!("Pin2 drive strength set to 12mA");

    // Set slew rate
    pin2.set_slew_rate(SlewRate::Fast);
    info!("Pin2 slew rate set to fast");

    // Enable Schmitt trigger
    pin2.set_schmitt(true);
    info!("Pin2 Schmitt trigger enabled");

    // Configure pull resistors
    pin2.set_pull(Pull::Up);
    info!("Pin2 pull-up enabled");

    // ========================================================================
    // Example 4: Using Pin Constants
    // ========================================================================
    info!("--- Example 4: Pin Constants ---");

    // Pin<25> has compile-time constants
    info!("Pin25 number: {}", Pin::<25>::PIN);
    info!("Pin25 mask: {:#010X}", Pin::<25>::MASK);

    #[cfg(feature = "rp2350b")]
    {
        info!("Pin32 number: {}", Pin::<32>::PIN);
        info!("Pin32 mask: {:#010X}", Pin::<32>::MASK);
    }

    // ========================================================================
    // Example 5: PIO Function
    // ========================================================================
    info!("--- Example 5: PIO Function Select ---");

    let pin3 = pins.p3;
    pin3.init_pad();
    pin3.set_function_pio0();
    info!("Pin3 configured for PIO0");
    info!("Pin3 function: {:?}", pin3.function());

    let pin4 = pins.p4;
    pin4.into_pio1();
    info!("Pin4 configured for PIO1");
    info!("Pin4 function: {:?}", pin4.function());

    // ========================================================================
    // Example 6: Direct SIO Access (via SIO HAL)
    // ========================================================================
    info!("--- Example 6: Direct SIO Access ---");

    let sio = Sio::new();

    // Read GPIO input for low bank (pins 0-31)
    let gpio = sio.gpio();
    let gpio_in = gpio.input_low();
    info!("GPIO IN (0-31): {:#010X}", gpio_in);

    // Read GPIO output for low bank
    let gpio_out = gpio.output_low();
    info!("GPIO OUT (0-31): {:#010X}", gpio_out);

    // Read GPIO OE for low bank
    let gpio_oe = gpio.oe_low();
    info!("GPIO OE (0-31): {:#010X}", gpio_oe);

    // ========================================================================
    // LED Blink Loop
    // ========================================================================
    info!("=== Starting LED Blink ===");

    let mut counter = 0u32;
    loop {
        led.toggle();
        counter = counter.wrapping_add(1);

        if counter % 10 == 0 {
            info!("BLINK Count: {}, LED: {}", counter, led.output_level());
        }

        delay.delay_ms(200);
    }
}
