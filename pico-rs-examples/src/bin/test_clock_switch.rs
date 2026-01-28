//! # Clock Switch Test
//!
//! Test switching system clock to PLL

#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

use pico_rs::Delay;
use pico_rs::block::ImageDef;
use pico_rs::entry;
use pico_rs::regs::clocks::{CLK_REF_SRC_XOSC, CLK_SYS_AUXSRC_PLL_SYS, CLK_SYS_SRC_AUX, Clock};
use pico_rs::regs::pll::PllConfig;
use pico_rs::regs::{Gpio, Pll, Xosc, clocks, gpio, resets, ticks};

/// RP2350 Boot Block
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::secure_exe();

/// LED pin
const LED_PIN: u8 = 25;

#[entry]
fn main() -> ! {
    info!("RP2350 started!");
    info!("Testing clock switch...");

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
    info!("XOSC done");
    led.toggle();
    delay.delay_ms(200);

    // Step 2: Enable tick
    info!("Step 2: Enable tick");
    ticks::Tick::WATCHDOG.configure(12);
    led.toggle();
    delay.delay_ms(200);

    // Step 3: Reset and unreset PLLs
    info!("Step 3: Init PLLs");
    resets::Resets::reset(resets::PLL_SYS | resets::PLL_USB);
    resets::Resets::unreset_wait(resets::PLL_SYS | resets::PLL_USB);
    Pll::SYS.init(&PllConfig::SYS_150MHZ);
    Pll::USB.init(&PllConfig::USB_48MHZ);
    info!("PLLs initialized");
    led.toggle();
    delay.delay_ms(200);

    // Step 4: Switch CLK_REF to XOSC
    info!("Step 4: Switch CLK_REF to XOSC");
    Clock::REF.configure(CLK_REF_SRC_XOSC, 0, 0x100);
    info!("CLK_REF switched");
    led.toggle();

    info!("Delay after CLK_REF...");
    delay.delay_ms(200);
    info!("Delay done");

    // Step 5: Switch CLK_SYS to PLL_SYS
    info!("Step 5: About to switch CLK_SYS to PLL_SYS");
    info!("Setting AUXSRC...");

    // Manual step-by-step for debugging
    use pico_rs::regs::Reg;
    let clk_sys_ctrl = Reg::new(0x40010000 + 0x3c);
    let clk_sys_selected = Reg::new(0x40010000 + 0x44);

    info!("CTRL before: {:#x}", clk_sys_ctrl.read());
    info!("SELECTED before: {:#x}", clk_sys_selected.read());

    // Clear SRC to 0 first
    clk_sys_ctrl.clear_bits(0x1);
    info!("Cleared SRC, waiting for selected...");
    while (clk_sys_selected.read() & 1) == 0 {
        cortex_m::asm::nop();
    }
    info!("Source 0 selected");

    // Set AUXSRC to PLL_SYS (value 0)
    clk_sys_ctrl.modify(|v| (v & !0xe0) | 0x00);
    info!("AUXSRC set to PLL_SYS");

    // Set SRC to AUX (value 1)
    clk_sys_ctrl.modify(|v| (v & !0x1) | 0x1);
    info!("SRC set to AUX, waiting...");
    while (clk_sys_selected.read() & 2) == 0 {
        cortex_m::asm::nop();
    }
    info!("CLK_SYS switched to 150 MHz!");
    led.toggle();

    // Now running at 150 MHz!
    let mut delay = Delay::new(150_000_000);

    info!("Now running at 150 MHz");

    let mut counter: u32 = 0;
    loop {
        led.toggle();
        counter = counter.wrapping_add(1);

        if counter % 2 == 0 {
            info!("Running at 150 MHz: count = {}", counter / 2);
        }

        delay.delay_ms(500);
    }
}
