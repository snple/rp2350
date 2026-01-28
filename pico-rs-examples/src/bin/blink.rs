//! # Blink Example
//!
//! Minimal example that boots the RP2350 and prints a message.
//! This demonstrates the basic setup required for an RP2350 application.

#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

use pico_rs::block::ImageDef;
use pico_rs::entry;

/// RP2350 Boot Block - tells the Boot ROM this is a valid ARM executable
#[link_section = ".start_block"]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::secure_exe();

#[entry]
fn main() -> ! {
    info!("RP2350 started!");
    info!("Blink example running...");

    let mut counter: u32 = 0;
    loop {
        // Simple delay loop
        for _ in 0..1_000_000 {
            cortex_m::asm::nop();
        }

        counter = counter.wrapping_add(1);
        info!("Loop iteration: {}", counter);

        cortex_m::asm::wfi();
    }
}
