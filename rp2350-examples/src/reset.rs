//! # Reset HAL Demo
//!
//! Demonstrates the reset subsystem HAL for RP2350.
//! Shows how to reset and unreset peripherals.

#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;

use rp2350::Delay;
use rp2350::main;
use rp2350::reset::{self, Resettable};
use rp2350::rt::peripheral::Peripherals as CorePeripherals;

const CLOCK_FREQ_HZ: u32 = 12_000_000;

#[main]
fn main() -> ! {
    info!("RP2350 Reset HAL Demo");

    // Take core peripherals for Delay
    let core_peripherals = CorePeripherals::take().unwrap();

    // Initialize delay
    let delay = Delay::init(core_peripherals.SYST, CLOCK_FREQ_HZ);

    delay.delay_ms(1000);

    // Unreset a single peripheral
    info!("Unresetting UART0...");
    rp2350::pac::UART0::unreset();
    info!("  Done");

    delay.delay_ms(1000);

    // Unreset multiple peripherals at once
    info!("Unresetting I2C0 and I2C1...");
    let mask = rp2350::pac::I2C0::RESET_MASK | rp2350::pac::I2C1::RESET_MASK;
    // SAFETY: mask is composed from valid RESET_MASK constants.
    unsafe { reset::unreset(mask) };
    info!("  Done");

    delay.delay_ms(1000);

    // Reset + unreset (standard init pattern)
    info!("Reset-init UART0...");
    <rp2350::pac::UART0 as Resettable>::init();
    info!("  Done");

    delay.delay_ms(1000);

    // Demonstrate batch reset + unreset
    info!("Batch reset-init UARTs...");
    let uart_mask = rp2350::pac::UART0::RESET_MASK | rp2350::pac::UART1::RESET_MASK;
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
