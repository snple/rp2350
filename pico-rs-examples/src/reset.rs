//! # Reset HAL Demo
//!
//! Demonstrates the reset subsystem HAL for RP2350.
//! Shows how to reset and unreset peripherals.

#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

use pico_rs::Delay;
use pico_rs::block::ImageDef;
use pico_rs::entry;
use pico_rs::reset::{self, Resettable};
use pico_rs::rt::peripheral::Peripherals as CorePeripherals;

/// RP2350 Boot Block - tells the Boot ROM this is a valid ARM executable
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::exe();

/// RP2350 default boot clock frequency
const CLOCK_FREQ_HZ: u32 = 12_000_000;

#[entry]
fn main() -> ! {
    info!("RP2350 Reset HAL Demo");

    // Take core peripherals for Delay
    let core_peripherals = CorePeripherals::take().unwrap();

    // Initialize delay
    let delay = Delay::new(core_peripherals.SYST, CLOCK_FREQ_HZ);

    delay.delay_ms(1000);

    // Unreset a single peripheral
    info!("Unresetting UART0...");
    pico_rs::pac::UART0::unreset();
    info!("  Done");

    delay.delay_ms(1000);

    // Unreset multiple peripherals at once
    info!("Unresetting I2C0 and I2C1...");
    let mask = pico_rs::pac::I2C0::RESET_MASK | pico_rs::pac::I2C1::RESET_MASK;
    // SAFETY: mask is composed from valid RESET_MASK constants.
    unsafe { reset::unreset(mask) };
    info!("  Done");

    delay.delay_ms(1000);

    // Reset + unreset (standard init pattern)
    info!("Reset-init UART0...");
    pico_rs::pac::UART0::reset_init();
    info!("  Done");

    delay.delay_ms(1000);

    // Demonstrate batch reset + unreset
    info!("Batch reset-init UARTs...");
    let uart_mask = pico_rs::pac::UART0::RESET_MASK | pico_rs::pac::UART1::RESET_MASK;
    // SAFETY: mask is composed from valid RESET_MASK constants.
    unsafe {
        reset::reset(uart_mask);
        reset::unreset(uart_mask);
    }
    info!("  Done");

    info!("Reset HAL Demo completed!");

    // Loop forever
    loop {
        delay.delay_ms(5000);
        info!("Reset HAL demo still running...");
    }
}
