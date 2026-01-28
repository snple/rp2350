//! Test the fixed Clock::configure() API
#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use panic_probe as _;
use pico_rs::block::ImageDef;
use pico_rs::entry;
use pico_rs::regs::{
    gpio::Gpio,
    pll::{Pll, PllConfig},
    resets,
    ticks::Tick,
    xosc::Xosc,
    clocks::{Clock, CLK_REF_SRC_XOSC, CLK_SYS_SRC_AUX, CLK_SYS_AUXSRC_PLL_SYS,
             CLK_PERI_AUXSRC_SYS, CLK_USB_AUXSRC_PLL_USB, CLK_ADC_AUXSRC_PLL_USB},
};
use pico_rs::systick::Delay;

/// RP2350 Boot Block
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::secure_exe();

#[entry]
fn main() -> ! {
    info!("Testing Clock::configure() API");

    // 1. XOSC
    info!("1. Init XOSC");
    Xosc::init(47);

    // 2. Tick
    info!("2. Enable tick");
    Tick::WATCHDOG.configure(12);

    // 3. PLLs
    info!("3. Init PLLs");
    resets::Resets::reset(resets::PLL_SYS | resets::PLL_USB);
    resets::Resets::unreset_wait(resets::PLL_SYS | resets::PLL_USB);
    Pll::SYS.init(&PllConfig::SYS_150MHZ);
    Pll::USB.init(&PllConfig::USB_48MHZ);

    // 4. Switch CLK_REF to XOSC using Clock::configure()
    info!("4. CLK_REF -> XOSC (using Clock::configure)");
    Clock::REF.configure(CLK_REF_SRC_XOSC, 0, 0x100);
    info!("CLK_REF done!");

    // 5. Switch CLK_SYS to PLL_SYS using Clock::configure()
    info!("5. CLK_SYS -> PLL_SYS (using Clock::configure)");
    Clock::SYS.configure(CLK_SYS_SRC_AUX, CLK_SYS_AUXSRC_PLL_SYS, 0x100);
    info!("CLK_SYS done! Now at 150 MHz");

    // 6. Peripheral clocks
    info!("6. Peripheral clocks");
    Clock::PERI.configure_aux_only(CLK_PERI_AUXSRC_SYS, 0x100);
    Clock::USB.configure_aux_only(CLK_USB_AUXSRC_PLL_USB, 0x100);
    Clock::ADC.configure_aux_only(CLK_ADC_AUXSRC_PLL_USB, 0x100);

    info!("All clocks configured via Clock::configure() API!");

    // Setup LED
    resets::Resets::unreset_wait(resets::IO_BANK0 | resets::PADS_BANK0);
    let gpio25 = Gpio::new(25);
    let led = gpio25.into_output();

    // Update SysTick for 150 MHz
    let mut delay = Delay::new(150_000_000);

    info!("Blinking at 150 MHz...");
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
