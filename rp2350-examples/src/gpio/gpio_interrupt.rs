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

use rp2350::gpio::{EdgeLevel, Pin, Pins};
use rp2350::interrupt;
use rp2350::main;
use rp2350::pac::{Interrupt, Peripherals};

static LED_STATE: AtomicBool = AtomicBool::new(false);
// Debounce flag
static BUTTON_PRESSED: AtomicBool = AtomicBool::new(false);

/// Button GPIO pin number
const BUTTON_PIN: u32 = 15;

#[main]
fn main() -> ! {
    info!("GPIO Interrupt Example");

    // Take device peripherals
    let dp = Peripherals::take().unwrap();

    // Initialize GPIO via Pins (consumes IO_BANK0 and PADS_BANK0)
    let pins = Pins::init(dp.IO_BANK0, dp.PADS_BANK0);

    // Configure LED as output
    let mut led = pins.pin::<25>();
    led.into_output();
    led.set_low();

    // Configure button as input with pull-up
    let mut button = pins.pin::<15>();
    button.into_input_pullup();

    // Enable falling edge interrupt on button (button press = falling edge with pull-up)
    button.enable_interrupt(EdgeLevel::EdgeLow);
    info!("Button interrupt enabled on GPIO15");

    // Enable IO_IRQ_BANK0 in NVIC
    unsafe {
        interrupt::enable(Interrupt::IO_IRQ_BANK0);
    }
    info!("NVIC interrupt enabled");

    // Main loop
    loop {
        // Check if button was pressed (set by interrupt handler)
        if BUTTON_PRESSED.swap(false, Ordering::Relaxed) {
            // Toggle LED using SIO atomic registers (interrupt-safe)
            let new_state = !LED_STATE.load(Ordering::Relaxed);
            LED_STATE.store(new_state, Ordering::Relaxed);

            if new_state {
                led.set_high();
                info!("LED ON");
            } else {
                info!("LED OFF");
                led.set_low();
            }
        }

        // Could do other work here or enter low power mode
        rp2350_rt::asm::wfi();
    }
}

/// GPIO Bank0 interrupt handler
///
/// This replaces the default weak interrupt handler defined in the vector table.
/// Uses `Pin::new()` for safe register access without needing Pin ownership.
#[allow(non_snake_case)]
#[unsafe(no_mangle)]
extern "C" fn IO_IRQ_BANK0() {
    // SAFETY: GPIO subsystem was initialised in main() via Pins::init().
    // Only interrupt-status registers are accessed here — no reconfiguration.
    let button = unsafe { Pin::<BUTTON_PIN>::steal() };

    // Check and clear GPIO15 falling edge interrupt in one call
    if button.check_and_clear(EdgeLevel::EdgeLow) {
        // Signal to main loop that button was pressed
        BUTTON_PRESSED.store(true, Ordering::Relaxed);
    }
}
