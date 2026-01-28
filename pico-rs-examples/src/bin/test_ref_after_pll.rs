//! Test CLK_REF after PLL init
#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use panic_probe as _;
use pico_rs::block::ImageDef;
use pico_rs::entry;
use pico_rs::regs::{
    Reg,
    clocks::{CLK_REF_SRC_XOSC, Clock},
    pll::{Pll, PllConfig},
    resets,
    ticks::Tick,
    xosc::Xosc,
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

#[entry]
fn main() -> ! {
    info!("Test CLK_REF after PLL");
    busy_wait(100_000);

    // XOSC
    info!("1. XOSC");
    Xosc::init(47);
    busy_wait(100_000);

    // Tick
    info!("2. Tick");
    Tick::WATCHDOG.configure(12);
    busy_wait(100_000);

    // PLLs
    info!("3. PLLs reset");
    resets::Resets::reset(resets::PLL_SYS | resets::PLL_USB);
    resets::Resets::unreset_wait(resets::PLL_SYS | resets::PLL_USB);
    busy_wait(100_000);

    info!("3a. PLL_SYS init");
    Pll::SYS.init(&PllConfig::SYS_150MHZ);
    busy_wait(100_000);

    info!("3b. PLL_USB init");
    Pll::USB.init(&PllConfig::USB_48MHZ);
    busy_wait(100_000);

    info!("4. About to configure CLK_REF");
    busy_wait(100_000);

    // Read before
    let clk_ref_ctrl = Reg::new(0x40010030);
    let clk_ref_selected = Reg::new(0x40010038);
    info!(
        "Before: CTRL=0x{:x} SEL=0x{:x}",
        clk_ref_ctrl.read(),
        clk_ref_selected.read()
    );
    busy_wait(100_000);

    // Configure CLK_REF
    info!("4a. Calling configure()");
    busy_wait(100_000);
    Clock::REF.configure(CLK_REF_SRC_XOSC, 0, 0x100);
    info!("4b. configure() returned");
    busy_wait(100_000);

    // Read after
    info!(
        "After: CTRL=0x{:x} SEL=0x{:x}",
        clk_ref_ctrl.read(),
        clk_ref_selected.read()
    );

    info!("SUCCESS!");
    loop {
        busy_wait(1_000_000);
    }
}
