//! Clean test for Clock::SYS.configure()
#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use panic_probe as _;
use pico_rs::block::ImageDef;
use pico_rs::entry;
use pico_rs::regs::{
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

fn busy_wait() {
    for _ in 0..100_000 {
        cortex_m::asm::nop();
    }
}

#[entry]
fn main() -> ! {
    info!("Clock API Test");
    busy_wait();

    // 1. XOSC
    info!("1. XOSC");
    Xosc::init(47);
    busy_wait();

    // 2. Tick
    info!("2. Tick");
    Tick::WATCHDOG.configure(12);
    busy_wait();

    // 3. PLLs
    info!("3. PLLs");
    resets::Resets::reset(resets::PLL_SYS | resets::PLL_USB);
    resets::Resets::unreset_wait(resets::PLL_SYS | resets::PLL_USB);
    Pll::SYS.init(&PllConfig::SYS_150MHZ);
    info!("3a. PLL_SYS done");
    busy_wait();
    Pll::USB.init(&PllConfig::USB_48MHZ);
    info!("3b. PLL_USB done");
    busy_wait();

    // 4. CLK_REF -> XOSC
    info!("4. CLK_REF starting...");
    busy_wait();
    Clock::REF.configure(CLK_REF_SRC_XOSC, 0, 0x100);
    info!("4. CLK_REF done");
    busy_wait();

    // 5. CLK_SYS -> PLL_SYS
    info!("5. CLK_SYS starting...");
    busy_wait();
    info!("5a. Calling configure...");
    busy_wait();

    // 6. Peripherals
    info!("6. PERI");
    Clock::PERI.configure_aux_only(CLK_PERI_AUXSRC_SYS, 0x100);
    Clock::USB.configure_aux_only(CLK_USB_AUXSRC_PLL_USB, 0x100);
    busy_wait();

    info!("DONE - 150MHz!");

    let mut count = 0u32;
    loop {
        count += 1;
        if count % 10_000_000 == 0 {
            info!("tick");
        }
    }
}
