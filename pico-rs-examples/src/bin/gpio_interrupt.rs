//! GPIO Interrupt Example for RP2350
//!
//! This example demonstrates how to use GPIO edge-triggered interrupts
//! to detect button presses and toggle an LED.
//!
//! Hardware setup:
//! - LED on GPIO 25 (built-in on Pico)
//! - Button on GPIO 15 with pull-up (active low)
//!
//! Run with: cargo run --release --bin gpio_interrupt

#![no_std]
#![no_main]

use core::sync::atomic::{AtomicBool, Ordering};

use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

use pico_rs::block::ImageDef;
use pico_rs::entry;
use pico_rs::gpio::{self, EdgeLevel, Pin15, Pin25};
use pico_rs::interrupt as irq;
use pico_rs::pac::Interrupt;

/// RP2350 Boot Block
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::secure_exe();

// Atomic flag for LED state
static LED_STATE: AtomicBool = AtomicBool::new(false);
// Debounce flag
static BUTTON_PRESSED: AtomicBool = AtomicBool::new(false);

#[entry]
fn main() -> ! {
    info!("GPIO Interrupt Example");

    // Initialize GPIO
    gpio::init_gpio();

    // Configure LED as output
    let led = Pin25::new();
    led.into_output();
    led.set_low();

    // Configure button as input with pull-up
    let button = Pin15::new();
    button.into_input_pullup();

    // Enable falling edge interrupt on button (button press = falling edge with pull-up)
    button.enable_interrupt(EdgeLevel::EdgeLow);
    info!("Button interrupt enabled on GPIO15");

    // Enable IO_IRQ_BANK0 in NVIC
    unsafe {
        irq::enable(Interrupt::IO_IRQ_BANK0);
    }
    info!("NVIC interrupt enabled");

    // Main loop
    loop {
        // Check if button was pressed (set by interrupt handler)
        if BUTTON_PRESSED.swap(false, Ordering::Relaxed) {
            // Toggle LED
            let new_state = !LED_STATE.load(Ordering::Relaxed);
            LED_STATE.store(new_state, Ordering::Relaxed);

            let led = Pin25::new();
            if new_state {
                led.set_high();
                info!("LED ON");
            } else {
                led.set_low();
                info!("LED OFF");
            }
        }

        // Could do other work here or enter low power mode
        pico_rs_rt::asm::wfi();
    }
}

/// GPIO Bank0 interrupt handler
///
/// This replaces the default weak interrupt handler defined in the vector table.
#[allow(non_snake_case)]
#[unsafe(no_mangle)]
extern "C" fn IO_IRQ_BANK0() {
    // Access GPIO to check and clear interrupt
    let button = Pin15::new();

    // Check if this is our button's falling edge interrupt
    if button.interrupt_status(EdgeLevel::EdgeLow) {
        // Clear the interrupt
        button.clear_interrupt(EdgeLevel::EdgeLow);

        // Signal to main loop that button was pressed
        BUTTON_PRESSED.store(true, Ordering::Relaxed);
    }
}
