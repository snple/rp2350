//! # Simple CLK_REF Test
//!
//! Test just switching CLK_REF without touching CLK_SYS

#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

use pico_rs::block::ImageDef;
use pico_rs::entry;
use pico_rs::regs::{Gpio, Reg, Xosc, gpio};

/// RP2350 Boot Block
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::secure_exe();

const CLOCKS_BASE: u32 = 0x40010000;

#[entry]
fn main() -> ! {
    info!("RP2350 started!");

    // Initialize GPIO first
    gpio::init();
    let led = Gpio::new(25);
    led.init();
    led.into_output();
    led.set_high();
    info!("LED on");

    // Simple busy wait (no SysTick) - reduced iterations
    fn busy_wait() {
        for _ in 0..100_000 {
            cortex_m::asm::nop();
        }
    }

    // Check initial clock state
    let clk_ref_ctrl = Reg::new(CLOCKS_BASE + 0x30);
    let clk_ref_div = Reg::new(CLOCKS_BASE + 0x34);
    let clk_ref_selected = Reg::new(CLOCKS_BASE + 0x38);

    info!("Initial CLK_REF state:");
    info!("  CTRL: {:#x}", clk_ref_ctrl.read());
    info!("  DIV: {:#x}", clk_ref_div.read());
    info!("  SELECTED: {:#x}", clk_ref_selected.read());

    let clk_sys_ctrl = Reg::new(CLOCKS_BASE + 0x3c);
    let clk_sys_div = Reg::new(CLOCKS_BASE + 0x40);
    let clk_sys_selected = Reg::new(CLOCKS_BASE + 0x44);

    info!("Initial CLK_SYS state:");
    info!("  CTRL: {:#x}", clk_sys_ctrl.read());
    info!("  DIV: {:#x}", clk_sys_div.read());
    info!("  SELECTED: {:#x}", clk_sys_selected.read());

    busy_wait();
    led.toggle();

    // Initialize XOSC
    info!("Initializing XOSC...");
    Xosc::init(47);
    info!("XOSC stable");

    busy_wait();
    led.toggle();

    // Now try to switch CLK_REF to XOSC (src = 2)
    info!("Switching CLK_REF to XOSC...");

    // Set divider
    clk_ref_div.write(0x100);
    info!("DIV set");

    // Read current state
    let ctrl = clk_ref_ctrl.read();
    info!("CTRL before switch: {:#x}", ctrl);

    // CLK_REF: SRC bits 1:0, AUXSRC bits 6:5
    // SRC values: 0=ROSC, 1=AUX, 2=XOSC, 3=LPOSC

    // Just set SRC to XOSC (value 2) directly
    clk_ref_ctrl.write(0x02); // SRC = XOSC
    info!("CTRL written, waiting for selected...");

    // Wait for XOSC to be selected (bit 2 in SELECTED)
    let mut count = 0u32;
    loop {
        let selected = clk_ref_selected.read();
        if (selected & (1 << 2)) != 0 {
            info!("CLK_REF switched to XOSC! SELECTED={:#x}", selected);
            break;
        }
        count += 1;
        if count > 1_000_000 {
            info!("Timeout! SELECTED={:#x}", selected);
            break;
        }
    }

    info!("About to do busy_wait 1");
    busy_wait();
    info!("busy_wait 1 done");
    led.toggle();

    info!("Final CLK_REF state:");
    info!("  CTRL: {:#x}", clk_ref_ctrl.read());
    info!("  SELECTED: {:#x}", clk_ref_selected.read());

    info!("Test complete!");

    let mut c = 0u32;
    loop {
        busy_wait();
        led.toggle();
        c += 1;
        if c % 2 == 0 {
            info!("Running: {}", c / 2);
        }
    }
}
