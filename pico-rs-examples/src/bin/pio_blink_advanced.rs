//! PIO Blink Example with .pio style syntax
//!
//! This example demonstrates the use of the pio_asm! macro with
//! the full .pio file syntax, including .program, labels, and comments.

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
    info!("PIO Blink with .pio syntax");

    // Initialize GPIO
    gpio::init();

    // Configure GPIO 25 for PIO0
    unsafe {
        let io_bank = &*pico_rs::pac::IO_BANK0::PTR;
        let ctrl = io_bank.gpio_ctrl(LED_PIN);
        ctrl.write_funcsel(6); // PIO0 function
    }

    // Take PIO0
    let mut pio = pio::Pio0::take();

    // PIO program using full .pio syntax
    // This mimics the blink.pio example from pico-examples
    let program = pio_asm!(
        "; SET pin 0 should be mapped to your LED GPIO",
        "",
        ".pio_version 0",
        "",
        ".program blink",
        "    pull block",
        "    out y, 32",
        ".wrap_target",
        "    mov x, y",
        "    set pins, 1   ; Turn LED on",
        "lp1:",
        "    jmp x-- lp1   ; Delay for (x + 1) cycles",
        "    mov x, y",
        "    set pins, 0   ; Turn LED off",
        "lp2:",
        "    jmp x-- lp2   ; Delay for the same number of cycles",
        ".wrap             ; Blink forever!"
    );

    info!(
        "Program compiled: {} instructions",
        program.instructions.len()
    );
    info!("Wrap: {} -> {}", program.wrap_target, program.wrap);

    // Install the program
    match pio.install_program(program.instructions, program.origin) {
        Ok(installed) => {
            info!("Program installed at offset {}", installed.offset);

            // Get state machine 0
            let sm = pio.state_machine(0);

            // Configure
            sm.set_clock_divisor(1, 0); // No division
            sm.set_wrap(program.wrap_target as u8, program.wrap as u8);
            sm.set_set_pins(LED_PIN, 1);

            // Set GPIO direction
            sm.set_consecutive_pindirs(LED_PIN, 1, true);

            // Restart and enable
            sm.restart();
            sm.restart_clock_divider();

            // Push delay value (in cycles)
            // At 12 MHz, delay of 6_000_000 = 0.5 seconds
            let delay_cycles = 6_000_000u32;
            sm.tx_fifo_write(delay_cycles);

            sm.set_enabled(true);

            info!("State machine started - LED blinking via PIO!");
        }
        Err(e) => {
            error!("Failed to install program: {:?}", e);
        }
    }

    // Keep running
    let mut delay = Delay::new(CLOCK_FREQ_HZ);
    loop {
        delay.delay_ms(1000);
        info!("Running...");
    }
}
