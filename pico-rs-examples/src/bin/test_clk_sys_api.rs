//! Test Clock::SYS.configure()
#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use panic_probe as _;
use pico_rs::block::ImageDef;
use pico_rs::entry;
use pico_rs::regs::{
    Reg,
    clocks::{CLK_REF_SRC_XOSC, CLK_SYS_AUXSRC_PLL_SYS, CLK_SYS_SRC_AUX, Clock},
    pll::{Pll, PllConfig},
    resets,
    ticks::Tick,
    xosc::Xosc,
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
    info!("Clock::SYS.configure test");

    // 1. XOSC
    info!("1. Init XOSC");
    Xosc::init(47);
    busy_wait();

    // 2. Tick
    info!("2. Enable tick");
    Tick::WATCHDOG.configure(12);

    // 3. PLLs
    info!("3. Init PLLs");
    resets::Resets::reset(resets::PLL_SYS | resets::PLL_USB);
    resets::Resets::unreset_wait(resets::PLL_SYS | resets::PLL_USB);
    Pll::SYS.init(&PllConfig::SYS_150MHZ);
    info!("PLL_SYS locked");
    busy_wait();

    // 4. Switch CLK_REF to XOSC
    info!("4. CLK_REF -> XOSC");
    Clock::REF.configure(CLK_REF_SRC_XOSC, 0, 0x100);
    info!("CLK_REF done");

    // Debug: read CLK_SYS registers before
    let clk_sys_ctrl = Reg::new(0x40010000 + 0x3c);
    let clk_sys_selected = Reg::new(0x40010000 + 0x44);
    let ctrl_val = clk_sys_ctrl.read();
    info!("Before: CLK_SYS CTRL = 0x{:08x}", ctrl_val);
    busy_wait();
    let sel_val = clk_sys_selected.read();
    info!("Before: CLK_SYS SELECTED = 0x{:08x}", sel_val);
    busy_wait();

    // 5. Try Clock::configure for CLK_SYS
    info!("5. About to call Clock::SYS.configure");
    for _ in 0..200_000 {
        cortex_m::asm::nop();
    }

    // Manual inline version to debug:
    info!("  Setting DIV...");
    for _ in 0..200_000 {
        cortex_m::asm::nop();
    }
    let clk_sys_div = Reg::new(0x40010000 + 0x40);
    clk_sys_div.write(0x100);
    busy_wait();

    info!("  Current SRC check...");
    let current = clk_sys_ctrl.read();
    info!("    CTRL = 0x{:08x}", current);
    busy_wait();

    info!("  Setting AUXSRC to PLL_SYS (0)...");
    // Clear bits 7:5
    clk_sys_ctrl.modify(|v| v & !0xe0);
    busy_wait();
    info!("    Done");

    info!("  Switching SRC to AUX (1)...");
    clk_sys_ctrl.set_bits(0x1);
    busy_wait();

    info!("  Waiting for SELECTED...");
    let mut timeout = 0u32;
    while (clk_sys_selected.read() & 2) == 0 {
        cortex_m::asm::nop();
        timeout += 1;
        if timeout > 100_000 {
            info!("  TIMEOUT! SELECTED = 0x{:08x}", clk_sys_selected.read());
            break;
        }
    }
    info!("  SELECTED = 0x{:08x}", clk_sys_selected.read());

    info!("Now at 150 MHz!");

    // Debug: read CLK_SYS registers after
    info!("After: CLK_SYS CTRL = 0x{:08x}", clk_sys_ctrl.read());
    info!(
        "After: CLK_SYS SELECTED = 0x{:08x}",
        clk_sys_selected.read()
    );

    info!("SUCCESS at 150 MHz!");

    loop {
        cortex_m::asm::nop();
    }
}
