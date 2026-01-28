//! # Clock Initialization Example
//!
//! Initialize the RP2350 to run at 150 MHz from the crystal oscillator.
//! This demonstrates the full clock initialization sequence.

#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

use pico_rs::Delay;
use pico_rs::block::ImageDef;
use pico_rs::entry;
use pico_rs::regs::clocks::{
    CLK_ADC_AUXSRC_PLL_USB, CLK_PERI_AUXSRC_SYS, CLK_REF_SRC_XOSC, CLK_SYS_AUXSRC_PLL_SYS,
    CLK_SYS_SRC_AUX, CLK_USB_AUXSRC_PLL_USB, Clock,
};
use pico_rs::regs::pll::PllConfig;
use pico_rs::regs::{Gpio, Pll, Reg, Xosc, gpio, resets, ticks};

/// RP2350 Boot Block
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::secure_exe();

/// System clock after initialization (150 MHz)
const SYS_CLOCK_HZ: u32 = 150_000_000;

/// LED pin on Pico boards
const LED_PIN: u8 = 25;

// Simple busy wait
fn busy_wait() {
    for _ in 0..100_000 {
        cortex_m::asm::nop();
    }
}

#[entry]
fn main() -> ! {
    info!("RP2350 started!");
    info!("Initializing clocks to 150 MHz...");

    // Initialize GPIO first
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

    // Step 4: Switch CLK_REF to XOSC
    info!("4. CLK_REF -> XOSC");
    // Direct register write - CLK_REF CTRL at CLOCKS_BASE + 0x30
    // SRC = 2 (XOSC)
    let clk_ref_ctrl = Reg::new(0x40010000 + 0x30);
    let clk_ref_selected = Reg::new(0x40010000 + 0x38);
    clk_ref_ctrl.write(0x02); // SRC = XOSC (2)
    // Wait for XOSC to be selected (bit 2)
    while (clk_ref_selected.read() & (1 << 2)) == 0 {
        cortex_m::asm::nop();
    }
    info!("CLK_REF done");
    busy_wait();

    // Step 5: Switch CLK_SYS to PLL_SYS
    info!("5. CLK_SYS -> PLL_SYS");
    // CLK_SYS CTRL at CLOCKS_BASE + 0x3c
    // First switch to REF (src=0), then configure AUXSRC, then switch to AUX (src=1)
    let clk_sys_ctrl = Reg::new(0x40010000 + 0x3c);
    let clk_sys_selected = Reg::new(0x40010000 + 0x44);

    // Current default: SRC=0 (CLK_REF), AUXSRC varies
    // We want: SRC=1 (AUX), AUXSRC=0 (PLL_SYS)

    // Read current
    let current = clk_sys_ctrl.read();
    info!("CLK_SYS CTRL current: {:#x}", current);

    // Make sure we're on SRC=0 (CLK_REF) first
    if (current & 0x1) != 0 {
        clk_sys_ctrl.clear_bits(0x1);
        while (clk_sys_selected.read() & 1) == 0 {
            cortex_m::asm::nop();
        }
    }

    // Set AUXSRC to PLL_SYS (0) - clear bits 7:5
    clk_sys_ctrl.modify(|v| v & !0xe0);

    // Switch to AUX (src=1)
    clk_sys_ctrl.set_bits(0x1);
    // Wait for AUX to be selected (bit 1)
    while (clk_sys_selected.read() & 2) == 0 {
        cortex_m::asm::nop();
    }

    // Now running at 150 MHz!
    info!("Now at 150 MHz!");

    // Step 6: Configure peripheral clocks
    info!("6. Peripheral clocks");
    Clock::PERI.configure_aux_only(CLK_PERI_AUXSRC_SYS, 0x100);
    Clock::USB.configure_aux_only(CLK_USB_AUXSRC_PLL_USB, 0x100);
    Clock::ADC.configure_aux_only(CLK_ADC_AUXSRC_PLL_USB, 0x100);

    info!("Clocks initialized!");
    info!("System clock: {} MHz", SYS_CLOCK_HZ / 1_000_000);

    // Initialize SysTick-based delay with new clock frequency
    let mut delay = Delay::new(SYS_CLOCK_HZ);

    let mut counter: u32 = 0;
    loop {
        led.toggle();
        counter = counter.wrapping_add(1);

        if counter % 2 == 0 {
            info!("count = {}", counter / 2);
        }

        delay.delay_ms(500);
    }
}
