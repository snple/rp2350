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
use pico_rs::reset::{self, Peripheral};
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
    let mut delay = Delay::new(core_peripherals.SYST, CLOCK_FREQ_HZ);

    // Check initial reset status
    info!("Checking initial reset status...");
    info!(
        "  UART0 in reset: {}",
        reset::is_in_reset(Peripheral::UART0)
    );
    info!("  I2C0 in reset: {}", reset::is_in_reset(Peripheral::I2C0));
    info!("  SPI0 in reset: {}", reset::is_in_reset(Peripheral::SPI0));

    delay.delay_ms(1000);

    // Unreset a single peripheral
    info!("Unresetting UART0...");
    reset::unreset_block_wait(Peripheral::UART0);
    info!(
        "  UART0 in reset: {}",
        reset::is_in_reset(Peripheral::UART0)
    );
    info!(
        "  UART0 reset done: {}",
        reset::is_reset_done(Peripheral::UART0)
    );

    delay.delay_ms(1000);

    // Unreset multiple peripherals at once
    info!("Unresetting I2C0 and I2C1...");
    reset::unreset_block_wait_mask(Peripheral::I2C0 | Peripheral::I2C1);
    info!("  I2C0 in reset: {}", reset::is_in_reset(Peripheral::I2C0));
    info!("  I2C1 in reset: {}", reset::is_in_reset(Peripheral::I2C1));

    delay.delay_ms(1000);

    // Reset a peripheral back
    info!("Resetting UART0 back...");
    reset::reset_block(Peripheral::UART0);
    info!(
        "  UART0 in reset: {}",
        reset::is_in_reset(Peripheral::UART0)
    );

    delay.delay_ms(1000);

    // Demonstrate unreset without waiting
    info!("Unresetting SPI0 without waiting...");
    reset::unreset_block(Peripheral::SPI0);
    info!(
        "  SPI0 reset done (immediate): {}",
        reset::is_reset_done(Peripheral::SPI0)
    );

    // Wait for it to complete
    info!("Waiting for SPI0 reset to complete...");
    reset::unreset_wait(Peripheral::SPI0);
    info!(
        "  SPI0 reset done (after wait): {}",
        reset::is_reset_done(Peripheral::SPI0)
    );

    delay.delay_ms(1000);

    // Demonstrate mask operations
    info!("Working with multiple peripherals...");
    let uart_mask = Peripheral::UART0 | Peripheral::UART1;
    reset::unreset_block_wait_mask(uart_mask);
    info!(
        "  Both UARTs ready: {}",
        reset::is_reset_done_mask(uart_mask)
    );

    info!("Reset HAL Demo completed!");

    // Loop forever
    loop {
        delay.delay_ms(5000);
        info!("Reset HAL demo still running...");
    }
}
