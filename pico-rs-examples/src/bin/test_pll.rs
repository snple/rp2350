//! # PLL Test
//!
//! Test PLL initialization step by step

#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

use pico_rs::Delay;
use pico_rs::block::ImageDef;
use pico_rs::entry;
use pico_rs::regs::pll::PllConfig;
use pico_rs::regs::{Gpio, Pll, Xosc, gpio, resets, ticks};

/// RP2350 Boot Block
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::secure_exe();

/// LED pin
const LED_PIN: u8 = 25;

#[entry]
fn main() -> ! {
    info!("RP2350 started!");
    info!("Testing PLL initialization...");

    // Initialize GPIO first
    gpio::init();
    let led = Gpio::new(LED_PIN);
    led.init();
    led.into_output();
    led.set_high();
    info!("GPIO {} configured", LED_PIN);

    // Still using ROSC (12 MHz) for delay
    let mut delay = Delay::new(12_000_000);

    // Step 1: Initialize XOSC
    info!("Step 1: Init XOSC");
    Xosc::init(47);
    info!("XOSC done, is_stable = {}", Xosc::is_stable());
    led.toggle();
    delay.delay_ms(200);

    // Step 2: Enable watchdog tick (required before PLL)
    info!("Step 2: Enable tick");
    ticks::Tick::WATCHDOG.configure(12); // 12 cycles = 1 µs at 12 MHz
    info!(
        "Tick enabled, is_running = {}",
        ticks::Tick::WATCHDOG.is_running()
    );
    led.toggle();
    delay.delay_ms(200);

    // Step 3: Reset PLLs
    info!("Step 3: Reset PLLs");
    resets::Resets::reset(resets::PLL_SYS | resets::PLL_USB);
    info!("PLLs in reset");
    led.toggle();
    delay.delay_ms(200);

    // Step 4: Unreset PLLs
    info!("Step 4: Unreset PLLs");
    resets::Resets::unreset_wait(resets::PLL_SYS | resets::PLL_USB);
    info!("PLLs out of reset");
    led.toggle();
    delay.delay_ms(200);

    // Step 5: Initialize PLL_SYS
    info!("Step 5: Init PLL_SYS (150 MHz)");
    Pll::SYS.init(&PllConfig::SYS_150MHZ);
    info!("PLL_SYS done, is_locked = {}", Pll::SYS.is_locked());
    led.toggle();
    delay.delay_ms(200);

    // Step 6: Initialize PLL_USB
    info!("Step 6: Init PLL_USB (48 MHz)");
    Pll::USB.init(&PllConfig::USB_48MHZ);
    info!("PLL_USB done, is_locked = {}", Pll::USB.is_locked());
    led.toggle();
    delay.delay_ms(200);

    info!("All PLL initialization complete!");

    let mut counter: u32 = 0;
    loop {
        led.toggle();
        counter = counter.wrapping_add(1);

        if counter % 2 == 0 {
            info!("PLL test running: count = {}", counter / 2);
        }

        delay.delay_ms(500);
    }
}
