//! Test Clock API - direct copy from clocks_150mhz but using Clock::configure
#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

use pico_rs::Delay;
use pico_rs::block::ImageDef;
use pico_rs::entry;
use pico_rs::regs::clocks::{
    CLK_PERI_AUXSRC_SYS, CLK_REF_SRC_XOSC, CLK_SYS_AUXSRC_PLL_SYS, CLK_SYS_SRC_AUX,
    CLK_USB_AUXSRC_PLL_USB, Clock,
};
use pico_rs::regs::pll::PllConfig;
use pico_rs::regs::{Gpio, Pll, Reg, Xosc, gpio, resets, ticks};

/// RP2350 Boot Block
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::secure_exe();

const SYS_CLOCK_HZ: u32 = 150_000_000;
const LED_PIN: u8 = 25;

fn busy_wait() {
    for _ in 0..100_000 {
        cortex_m::asm::nop();
    }
}

#[entry]
fn main() -> ! {
    info!("RP2350 started!");
    info!("Testing Clock API...");

    // Initialize GPIO first (same as clocks_150mhz)
    gpio::init();
    let led = Gpio::new(LED_PIN);
    led.init();
    led.into_output();
    led.set_high();

    // Step 1: Initialize XOSC
    info!("1. Init XOSC");
    Xosc::init(47);
    busy_wait();

    // Step 2: Enable tick generator
    info!("2. Enable tick");
    ticks::Tick::WATCHDOG.configure(12);
    busy_wait();

    // Step 3: Initialize PLLs
    info!("3. Init PLLs");
    resets::Resets::reset(resets::PLL_SYS | resets::PLL_USB);
    resets::Resets::unreset_wait(resets::PLL_SYS | resets::PLL_USB);
    Pll::SYS.init(&PllConfig::SYS_150MHZ);
    Pll::USB.init(&PllConfig::USB_48MHZ);
    busy_wait();

    // Step 4: Switch CLK_REF to XOSC - using Clock API
    info!("4. CLK_REF -> XOSC (using API)");
    Clock::REF.configure(CLK_REF_SRC_XOSC, 0, 0x100);
    info!("CLK_REF done");
    busy_wait();

    // Step 5: Switch CLK_SYS to PLL_SYS - using Clock API
    info!("5. CLK_SYS -> PLL_SYS (using API)");
    info!(
        "  src={}, auxsrc={}",
        CLK_SYS_SRC_AUX, CLK_SYS_AUXSRC_PLL_SYS
    );
    busy_wait();
    Clock::SYS.configure(CLK_SYS_SRC_AUX, CLK_SYS_AUXSRC_PLL_SYS, 0x100);
    info!("Now at 150 MHz!");

    // Step 6: Configure peripheral clocks
    info!("6. Peripheral clocks");
    Clock::PERI.configure_aux_only(CLK_PERI_AUXSRC_SYS, 0x100);
    Clock::USB.configure_aux_only(CLK_USB_AUXSRC_PLL_USB, 0x100);

    info!("Clocks initialized via API!");
    info!("System clock: {} MHz", SYS_CLOCK_HZ / 1_000_000);

    // Update SysTick for new clock frequency
    let mut delay = Delay::new(SYS_CLOCK_HZ);

    let mut count = 0u32;
    loop {
        count += 1;
        info!("count = {}", count);
        led.set_high();
        delay.delay_ms(250);
        led.set_low();
        delay.delay_ms(250);
    }
}
