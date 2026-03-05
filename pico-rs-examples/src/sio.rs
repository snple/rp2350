//! SIO Example - Demonstrates SIO HAL features
//!
//! This example demonstrates:
//! - Reading core ID
//! - Using GPIO via SIO for fast output
//! - Using hardware spinlocks
//! - Using the interpolator for hardware-accelerated math
//!
//! Run with: cargo run --release --bin sio_example

#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use panic_probe as _;
use pico_rs::block::ImageDef;
use pico_rs::entry;
use pico_rs::pac::Peripherals;
use pico_rs::rt::peripheral::Peripherals as CorePeripherals;
use pico_rs::sio::{Sio, Spinlock};
use pico_rs::systick::Delay;

/// RP2350 Boot Block
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::exe();

const LED_PIN: u8 = 25;
const CLOCK_FREQ_HZ: u32 = 150_000_000;

#[entry]
fn main() -> ! {
    info!("SIO Example");

    // Take device peripherals
    let dp = Peripherals::take().unwrap();

    // Get SIO instance
    let sio = Sio::new();

    // Print core ID
    let core_id = sio.core_id();
    info!("Running on core {}", core_id);

    // Take core peripherals for Delay
    let core_peripherals = CorePeripherals::take().unwrap();

    // Initialize delay
    let delay = Delay::new(core_peripherals.SYST, CLOCK_FREQ_HZ);

    // Configure GPIO 25 (LED) as output via SIO
    // First need to set up the GPIO pad and function
    let io_bank0 = dp.IO_BANK0;
    let pads_bank0 = dp.PADS_BANK0;

    // Configure pad
    pads_bank0.gpio25.write_bits(
        (1 << 6) |  // IE (input enable)
        (0 << 4) |  // OD (output disable = 0, output enabled)
        (1 << 3) |  // PUE (pull-up enable, not strictly needed)
        (0 << 2) |  // PDE (pull-down enable)
        (1 << 1) |  // SCHMITT (Schmitt trigger)
        (1 << 0), // SLEWFAST
    );

    // Set GPIO function to SIO (function 5)
    io_bank0.gpio25_ctrl.write_bits(5);

    // Enable output via SIO
    sio.gpio().pin_oe_set(LED_PIN);

    info!("GPIO {} configured as SIO output", LED_PIN);

    // Demonstrate spinlock usage
    info!("Testing spinlock...");
    {
        let _guard = Spinlock::<0>::acquire();
        info!("  Spinlock 0 acquired");
        // Critical section protected by spinlock 0
        info!("  Spinlock 0 is_locked: {}", Spinlock::<0>::is_locked());
    }
    info!("  Spinlock 0 released");
    info!("  Spinlock 0 is_locked: {}", Spinlock::<0>::is_locked());

    // Demonstrate interpolator usage for byte extraction
    info!("Testing interpolator 0 (byte extraction)...");
    {
        let interp = sio.interp0();

        // Configure for simple shift-and-mask
        // Extract bits 8-15 (second byte) from a 32-bit value
        interp.config_lane0(8, 0, 7, false); // shift=8, mask bits 0-7, unsigned

        // Set up accumulators
        interp.set_accum0(0x12345678);
        interp.set_base0(0);

        // Read result
        let result = interp.peek_lane0();
        info!("  Input: 0x{:08x}", 0x12345678u32);
        info!("  Extracted byte (bits 8-15): 0x{:02x}", result);
        // Expected: 0x56 (the second byte)
    }

    // Demonstrate interpolator for simple accumulator addition
    info!("Testing interpolator 0 (accumulator with base)...");
    {
        let interp = sio.interp0();

        // Configure lane0: shift=0, mask all 32 bits (lsb=0, msb=31)
        // This passes accum0 through unchanged
        interp.config_lane0(0, 0, 31, false);

        // Configure lane1: shift=0, mask all 32 bits
        interp.config_lane1(0, 0, 31, false);

        // For address calculation:
        // - Lane 0 result = (accum0 >> shift) & mask + base0
        // - Lane 1 result = (accum1 >> shift) & mask + base1
        // - FULL result = lane0_result + lane1_result + base2
        //
        // With our config: lane0_result = accum0 + base0
        // Set base0 = base address, accum0 = offset

        interp.set_base0(0x2000_0000); // Base address
        interp.set_base1(0);
        interp.set_base2(0);
        interp.set_accum1(0);

        for i in 0u32..5 {
            interp.set_accum0(i * 4); // Index * 4 for word access
            let result = interp.peek_lane0();
            info!(
                "  base0 + accum0: 0x{:08x} + {} = 0x{:08x}",
                0x2000_0000u32,
                i * 4,
                result
            );
        }
    }

    info!("Starting LED blink via SIO...");

    let gpio = sio.gpio();

    loop {
        // Turn LED on using atomic set
        gpio.pin_set(LED_PIN);
        info!("LED ON");
        delay.delay_ms(250);

        // Turn LED off using atomic clear
        gpio.pin_clr(LED_PIN);
        info!("LED OFF");
        delay.delay_ms(250);

        // Toggle LED using atomic XOR
        gpio.pin_toggle(LED_PIN);
        info!("LED TOGGLE (ON)");
        delay.delay_ms(250);

        gpio.pin_toggle(LED_PIN);
        info!("LED TOGGLE (OFF)");
        delay.delay_ms(250);
    }
}
