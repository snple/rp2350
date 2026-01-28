//! Minimal test for Clock::configure()
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
    clocks::{Clock, CLK_REF_SRC_XOSC},
    Reg,
};

/// RP2350 Boot Block
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::secure_exe();

fn busy_wait() {
    for _ in 0..50_000 {
        cortex_m::asm::nop();
    }
}

#[entry]
fn main() -> ! {
    info!("Minimal Clock::configure test");

    // 1. XOSC
    info!("1. Init XOSC");
    Xosc::init(47);
    busy_wait();
    info!("XOSC stable");

    // 2. Tick
    info!("2. Enable tick");
    Tick::WATCHDOG.configure(12);
    busy_wait();

    // Debug: read CLK_REF registers before
    let clk_ref_ctrl = Reg::new(0x40010000 + 0x30);
    let clk_ref_selected = Reg::new(0x40010000 + 0x38);
    info!("Before: CLK_REF CTRL = 0x{:08x}", clk_ref_ctrl.read());
    info!("Before: CLK_REF SELECTED = 0x{:08x}", clk_ref_selected.read());

    // 3. Try Clock::configure for CLK_REF
    info!("3. Calling Clock::REF.configure(XOSC, 0, 0x100)");
    Clock::REF.configure(CLK_REF_SRC_XOSC, 0, 0x100);
    info!("Clock::REF.configure returned!");

    // Debug: read CLK_REF registers after
    info!("After: CLK_REF CTRL = 0x{:08x}", clk_ref_ctrl.read());
    info!("After: CLK_REF SELECTED = 0x{:08x}", clk_ref_selected.read());

    info!("SUCCESS!");

    loop {
        cortex_m::asm::nop();
    }
}
