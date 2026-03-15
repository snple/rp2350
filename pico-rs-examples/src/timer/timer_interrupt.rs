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

use defmt::*;
use defmt_rtt as _;
use panic_probe as _;
use pico_rs::block::ImageDef;
use pico_rs::clocks::ClockBuilder;
use pico_rs::entry;
use pico_rs::gpio::Pins;
use pico_rs::interrupt::{self, Mutex};
use pico_rs::pac::{Interrupt, Peripherals};
use pico_rs::ticks::Ticks;
use pico_rs::timer::Timer0;

/// RP2350 Boot Block
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::exe();

// Shared state protected by a mutex
static TIMER: Mutex<RefCell<Option<Timer0>>> = Mutex::new(RefCell::new(None));
static LED_STATE: AtomicBool = AtomicBool::new(false);

// Interval in microseconds (500ms)
const BLINK_INTERVAL_US: u32 = 500_000;

#[entry]
fn main() -> ! {
    info!("Timer Interrupt Example");

    // Take device peripherals
    let dp = Peripherals::take().unwrap();

    // Initialize clock system
    let clocks = ClockBuilder::new().build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);

    // Initialize GPIO
    let pins = Pins::new(dp.IO_BANK0, dp.PADS_BANK0);
    let led = pins.pin::<25>();
    led.into_output();
    led.set_low();
    info!("LED configured on GPIO25");

    // Initialize TIMER0 tick generator (1µs ticks from clk_ref)
    let ticks = Ticks::new(dp.TICKS, &clocks);
    ticks.init_timer0();

    // Initialize Timer0
    let timer = Timer0::init(dp.TIMER0);

    // Set up first alarm
    let now = timer.get_time_us_32();
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
        pico_rs_rt::asm::wfi(); // Wait for interrupt
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
            let sio = unsafe { pico_rs::pac::SIO::steal() };
            if new_state {
                sio.gpio_out_set.write_bits(1 << 25);
            } else {
                sio.gpio_out_clr.write_bits(1 << 25);
            }

            // Schedule next alarm
            let now = timer.get_time_us_32();
            timer.set_alarm(0, now.wrapping_add(BLINK_INTERVAL_US));
        }
    });
}
