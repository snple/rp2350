//! # Watchdog Example
//!
//! Demonstrates the watchdog timer functionality on RP2350.
//!
//! This example shows:
//! - Detecting reset reason (power-on vs watchdog)
//! - Using watchdog as a regular watchdog timer
//! - Performing a full system reboot
//! - Using scratch registers to persist data across resets
//!
//! ## Behavior
//!
//! 1. On first boot (power-on), the LED blinks slowly for 5 seconds
//! 2. Then triggers a watchdog reboot
//! 3. On watchdog reboot, the LED blinks fast to indicate watchdog reset
//! 4. After 3 seconds, reboots again with a different scratch value

#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;

use pico_rs::Delay;
use pico_rs::gpio::Pins;
use pico_rs::main;
use pico_rs::pac::Peripherals;
use pico_rs::pac::WATCHDOG;
use pico_rs::rt::peripheral::Peripherals as CorePeripherals;
use pico_rs::watchdog::{self, Watchdog};

// Binary info declarations
pico_rs::bi_program_name!("Watchdog Example");
pico_rs::bi_program_version!("1.0.0");
pico_rs::bi_program_description!("Demonstrates watchdog timer and system reboot");
pico_rs::bi_pico_rs_board!("pico-rs2");
pico_rs::bi_program_feature!("Watchdog");
pico_rs::bi_pin_with_func!(25, Sio);
pico_rs::bi_pin_with_name!(25, "LED");

const CLOCK_FREQ_HZ: u32 = 12_000_000;

/// Magic value to track reboot cycles
const BOOT_MAGIC: u32 = 0xDEADBEEF;

#[main]
fn main() -> ! {
    info!("====================================");
    info!("  Watchdog Example - RP2350");
    info!("====================================");

    // Initialize GPIO and LED
    let dp = Peripherals::take().unwrap();
    let pins = Pins::init(dp.IO_BANK0, dp.PADS_BANK0);
    let mut led = pins.pin::<25>();
    led.into_output();

    // Initialize delay (requires SYST peripheral)
    let cp = CorePeripherals::take().unwrap();
    let delay = Delay::init(cp.SYST, CLOCK_FREQ_HZ);

    // Create watchdog instance to check reset reason
    let mut wd = Watchdog::init(unsafe { WATCHDOG::steal() });

    // Check reset reason and scratch register
    let reset_reason = wd.reset_reason();
    let scratch_value = wd.scratch0();

    info!("Reset reason: {:?}", reset_reason);
    info!("Scratch[0] value: {:#010x}", scratch_value);

    match reset_reason {
        pico_rs::watchdog::ResetReason::PowerOn => {
            info!("Power-on reset detected!");
            info!("Starting slow blink for 5 seconds...");

            // Clear scratch register on fresh boot
            wd.set_scratch0(BOOT_MAGIC);
            wd.set_scratch1(1); // Boot counter

            // Slow blink for 5 seconds
            for i in 0..5 {
                led.set_high();
                info!("LED ON  (cycle {})", i + 1);
                delay.delay_ms(500);

                led.set_low();
                info!("LED OFF (cycle {})", i + 1);
                delay.delay_ms(500);
            }

            info!("");
            info!("Triggering watchdog reboot in 100ms...");
            info!("====================================");

            // Trigger a reboot
            watchdog::reboot(100);
        }

        pico_rs::watchdog::ResetReason::WatchdogTimeout
        | pico_rs::watchdog::ResetReason::WatchdogForce => {
            info!("Watchdog reset detected!");

            // Increment boot counter
            let boot_count = wd.scratch1().wrapping_add(1);
            wd.set_scratch1(boot_count);
            info!("Boot count: {}", boot_count);

            if scratch_value == BOOT_MAGIC {
                info!("Scratch register preserved across reset!");
            }

            info!("Starting fast blink for 3 seconds...");

            // Fast blink to indicate watchdog reset
            for i in 0..15 {
                led.set_high();
                delay.delay_ms(100);
                led.set_low();
                delay.delay_ms(100);

                if i % 5 == 4 {
                    info!("Fast blink cycle {}/15", i + 1);
                }
            }

            // Check if we should stop or continue rebooting
            if boot_count >= 3 {
                info!("");
                info!("Reached {} reboots, stopping.", boot_count);
                info!("LED will stay on. Power cycle to restart demo.");
                info!("====================================");

                // Clear scratch and keep LED on
                wd.set_scratch0(0);
                wd.set_scratch1(0);

                led.set_high();
                loop {
                    pico_rs_rt::asm::wfe();
                }
            } else {
                info!("");
                info!("Rebooting again in 100ms...");
                info!("====================================");

                watchdog::reboot(100);
            }
        }

        pico_rs::watchdog::ResetReason::Unknown(reason) => {
            info!("Unknown reset reason: {:#010x}", reason);

            // Just blink normally
            loop {
                led.toggle();
                delay.delay_ms(250);
            }
        }
    }
}
