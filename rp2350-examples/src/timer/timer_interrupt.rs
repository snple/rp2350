//! Timer Interrupt Example for RP2350
//!
//! This example demonstrates how to use hardware timer interrupts
//! to blink an LED at a precise interval.
//!
//! The LED will toggle every 500ms using TIMER0 alarm interrupt.
//!
//! Run with: cargo run --release --bin timer_interrupt

#![no_std]
#![no_main]

use core::cell::RefCell;
use core::sync::atomic::{AtomicBool, Ordering};

#[cfg(feature = "defmt")]
use defmt_rtt as _;
use rp2350::clocks::ClockBuilder;
use rp2350::gpio::Pins;
use rp2350::interrupt::{self, Mutex};
use rp2350::main;
use rp2350::pac::{Interrupt, Peripherals};
use rp2350::ticks::Ticks;
use rp2350::timer::Timer0;
use rp2350_examples::log::*;

static TIMER: Mutex<RefCell<Option<Timer0>>> = Mutex::new(RefCell::new(None));
static LED_STATE: AtomicBool = AtomicBool::new(false);

// Interval in microseconds (500ms)
const BLINK_INTERVAL_US: u32 = 500_000;

#[main]
fn main() -> ! {
    info!("Timer Interrupt Example");

    // Take device peripherals
    let dp = Peripherals::take().unwrap();

    // Initialize clock system
    let clocks = ClockBuilder::new().build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);

    // Initialize GPIO
    let pins = Pins::init(dp.IO_BANK0, dp.PADS_BANK0);
    let mut led = pins.pin::<25>();
    led.into_output();
    led.set_low();
    info!("LED configured on GPIO25");

    // Initialize TIMER0 tick generator (1µs ticks from clk_ref)
    let mut ticks = Ticks::init(dp.TICKS, &clocks);
    ticks.init_timer0();

    // Initialize Timer0
    let timer = Timer0::init(dp.TIMER0);

    // Set up first alarm
    let now = timer.time_us_32();
    timer.set_alarm(0, now.wrapping_add(BLINK_INTERVAL_US));
    timer.enable_alarm_interrupt(0);

    // Store timer in global variable for interrupt handler
    interrupt::free(|cs| {
        TIMER.borrow(cs).replace(Some(timer));
    });

    // Enable TIMER0_IRQ_0 in NVIC
    unsafe {
        interrupt::enable(Interrupt::TIMER0_IRQ_0);
    }
    info!(
        "Timer interrupt enabled, blinking at {}ms interval",
        BLINK_INTERVAL_US / 1000
    );

    // Main loop - the LED toggling happens in the interrupt
    loop {
        // Could do other work here
        rp2350_rt::asm::wfi(); // Wait for interrupt
    }
}

/// Timer0 Alarm 0 interrupt handler
///
/// This replaces the default weak interrupt handler defined in the vector table.
#[unsafe(no_mangle)]
#[allow(non_snake_case)]
fn TIMER0_IRQ_0() {
    // Access the timer and GPIO in a critical section
    interrupt::free(|cs| {
        if let Some(ref timer) = *TIMER.borrow(cs).borrow() {
            // Clear the interrupt
            timer.clear_alarm_interrupt(0);

            // Toggle LED state
            let new_state = !LED_STATE.load(Ordering::Relaxed);
            LED_STATE.store(new_state, Ordering::Relaxed);

            // Toggle the actual LED
            // Note: We access GPIO through Pin type since we know
            // only this interrupt handler modifies the LED
            // Using unsafe steal since we know the pin was initialized
            let sio = unsafe { rp2350::pac::SIO::steal() };
            if new_state {
                sio.gpio_out_set.write_bits(1 << 25);
            } else {
                sio.gpio_out_clr.write_bits(1 << 25);
            }

            // Schedule next alarm
            let now = timer.time_us_32();
            timer.set_alarm(0, now.wrapping_add(BLINK_INTERVAL_US));
        }
    });
}
