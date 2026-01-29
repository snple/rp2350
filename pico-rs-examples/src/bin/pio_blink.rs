//! # PIO Blink Example
//!
//! Use PIO to blink the LED on GPIO 25.
//! This demonstrates basic PIO programming using the pio_asm! macro.

#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

use pico_rs::Delay;
use pico_rs::block::ImageDef;
use pico_rs::entry;
use pico_rs::gpio::{self, Gpio};
use pico_rs::pio::{self, PioBlock};
use pico_rs::pio_asm;

/// RP2350 Boot Block
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::secure_exe();

const CLOCK_FREQ_HZ: u32 = 12_000_000;
const LED_PIN: u8 = 25;

#[entry]
fn main() -> ! {
    info!("RP2350 PIO Blink Example");

    // Initialize GPIO subsystem
    gpio::init();

    // Configure GPIO 25 for PIO0 control (funcsel = 6)
    let led = Gpio::new(LED_PIN);
    unsafe {
        let io_bank = &*pico_rs::pac::IO_BANK0::PTR;
        let ctrl = io_bank.gpio_ctrl(LED_PIN);
        ctrl.write_funcsel(6); // PIO0 function
    }

    // Take PIO0 block (this also resets it)
    let mut pio = pio::Pio0::take();

    info!("PIO0 initialized");

    // Simple blink program using pio_asm! macro:
    let program = pio_asm!(
        "set pindirs, 1", // Set pin as output
        ".wrap_target",
        "set pins, 1 [31]", // Set pin high, wait 31 cycles
        "set pins, 0 [31]", // Set pin low, wait 31 cycles
        ".wrap"
    );

    info!(
        "Program compiled: {} instructions",
        program.instructions.len()
    );

    // Install the program
    match pio.install_program(program.instructions, None) {
        Ok(installed) => {
            info!("Program installed at offset {}", installed.offset);

            // Get state machine 0
            let sm = pio.state_machine(0);

            // Configure the state machine
            // Clock divider: 65536 = divide by 256 (slow blink)
            sm.set_clock_divisor(256, 0);

            // Set wrap to loop the program
            sm.set_wrap(program.wrap_target as u8, program.wrap as u8);

            // Configure SET pins - this maps SET instructions to GPIO 25
            sm.set_set_pins(LED_PIN, 1);

            // Set GPIO 25 as output using the state machine
            sm.set_consecutive_pindirs(LED_PIN, 1, true);

            info!("State machine configured");

            // Restart and enable the state machine
            sm.restart();
            sm.restart_clock_divider();
            sm.set_enabled(true);

            info!("PIO state machine started!");
            info!("LED should now be blinking via PIO");
        }
        Err(e) => {
            error!("Failed to install program: {:?}", e);
        }
    }

    // Keep the program running
    let mut delay = Delay::new(CLOCK_FREQ_HZ);
    let mut counter = 0u32;

    loop {
        delay.delay_ms(5000);
        counter += 1;
        info!("Running... {}", counter);
    }
}
