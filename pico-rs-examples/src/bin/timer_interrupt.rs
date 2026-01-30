//! Timer Interrupt Example for RP2350
//!
//! This example demonstrates how to use hardware timer interrupts
//! to blink an LED at a precise interval.
//!
//! The LED will toggle every 500ms using TIMER0 alarm interrupt.

#![no_std]
#![no_main]

use core::cell::RefCell;
use core::sync::atomic::{AtomicBool, Ordering};

use cortex_m::interrupt::Mutex;
use pico_rs::entry;
use pico_rs::gpio::GpioBank;
use pico_rs::interrupt as irq;
use pico_rs::pac::Interrupt;
use pico_rs::timer::Timer0;

// Shared state protected by a mutex
static TIMER: Mutex<RefCell<Option<Timer0>>> = Mutex::new(RefCell::new(None));
static LED_STATE: AtomicBool = AtomicBool::new(false);

// LED pin (GPIO 25 on Pico)
const LED_PIN: u8 = 25;

// Interval in microseconds (500ms)
const BLINK_INTERVAL_US: u32 = 500_000;

#[entry]
fn main() -> ! {
    // Initialize GPIO
    let gpio = GpioBank::init();
    let led = gpio.pin25();
    led.into_output();
    led.set_low();

    // Initialize Timer0
    let timer = Timer0::init();

    // Set up first alarm
    let now = timer.get_time_us_32();
    timer.set_alarm(0, now.wrapping_add(BLINK_INTERVAL_US));
    timer.enable_alarm_interrupt(0);

    // Store timer in global variable for interrupt handler
    cortex_m::interrupt::free(|cs| {
        TIMER.borrow(cs).replace(Some(timer));
    });

    // Enable TIMER0_IRQ_0 in NVIC
    unsafe {
        irq::enable(Interrupt::TIMER0_IRQ_0);
    }

    // Main loop - the LED toggling happens in the interrupt
    loop {
        // Could do other work here
        cortex_m::asm::wfi(); // Wait for interrupt
    }
}

/// Timer0 Alarm 0 interrupt handler
///
/// This replaces the default weak interrupt handler defined in the vector table.
#[unsafe(no_mangle)]
#[allow(non_snake_case)]
fn TIMER0_IRQ_0() {
    // Access the timer and GPIO in a critical section
    cortex_m::interrupt::free(|cs| {
        if let Some(ref timer) = *TIMER.borrow(cs).borrow() {
            // Clear the interrupt
            timer.clear_alarm_interrupt(0);

            // Toggle LED state
            let new_state = !LED_STATE.load(Ordering::Relaxed);
            LED_STATE.store(new_state, Ordering::Relaxed);

            // Toggle the actual LED
            // Note: We access GPIO through unsafe static since we know
            // only this interrupt handler modifies the LED
            unsafe {
                let gpio = GpioBank::new_unchecked();
                let led = gpio.pin25();
                if new_state {
                    led.set_high();
                } else {
                    led.set_low();
                }
            }

            // Schedule next alarm
            let now = timer.get_time_us_32();
            timer.set_alarm(0, now.wrapping_add(BLINK_INTERVAL_US));
        }
    });
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}
