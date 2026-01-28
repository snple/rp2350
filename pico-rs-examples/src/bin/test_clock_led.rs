//! Test Clock API with LED indication
#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use panic_probe as _;
use pico_rs::block::ImageDef;
use pico_rs::entry;
use pico_rs::regs::{
    gpio::Gpio,
    resets,
    ticks::Tick,
    xosc::Xosc,
    pll::{Pll, PllConfig},
    clocks::{Clock, CLK_REF_SRC_XOSC, CLK_SYS_SRC_AUX, CLK_SYS_AUXSRC_PLL_SYS,
             CLK_PERI_AUXSRC_SYS, CLK_USB_AUXSRC_PLL_USB},
};

/// RP2350 Boot Block
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::secure_exe();

fn busy_wait(n: u32) {
    for _ in 0..n {
        cortex_m::asm::nop();
    }
}

fn blink_led(led: &Gpio, times: u32) {
    for _ in 0..times {
        led.set_high();
        busy_wait(100_000);
        led.set_low();
        busy_wait(100_000);
    }
}

#[entry]
fn main() -> ! {
    info!("Clock API LED Test");

    // XOSC
    Xosc::init(47);
    info!("1. XOSC done");

    // Tick
    Tick::WATCHDOG.configure(12);
    info!("2. Tick done");

    // PLLs
    resets::Resets::reset(resets::PLL_SYS | resets::PLL_USB);
    resets::Resets::unreset_wait(resets::PLL_SYS | resets::PLL_USB);
    Pll::SYS.init(&PllConfig::SYS_150MHZ);
    Pll::USB.init(&PllConfig::USB_48MHZ);
    info!("3. PLLs done");

    // CLK_REF -> XOSC
    Clock::REF.configure(CLK_REF_SRC_XOSC, 0, 0x100);
    info!("4. CLK_REF done");

    // CLK_SYS -> PLL_SYS
    Clock::SYS.configure(CLK_SYS_SRC_AUX, CLK_SYS_AUXSRC_PLL_SYS, 0x100);
    info!("5. CLK_SYS done - 150MHz!");

    // Peripherals
    Clock::PERI.configure_aux_only(CLK_PERI_AUXSRC_SYS, 0x100);
    Clock::USB.configure_aux_only(CLK_USB_AUXSRC_PLL_USB, 0x100);
    info!("6. Peripherals done");

    // Now init GPIO after clocks are set up
    resets::Resets::unreset_wait(resets::IO_BANK0 | resets::PADS_BANK0);
    let led = Gpio::new(25);
    led.init();
    led.into_output();

    info!("SUCCESS! All clocks configured via API");

    // Rapid blink = success
    loop {
        led.set_high();
        busy_wait(100_000);
        led.set_low();
        busy_wait(100_000);
    }
}
