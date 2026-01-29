//! Direct GPIO test to verify hardware

#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

use pico_rs::Delay;
use pico_rs::block::ImageDef;
use pico_rs::entry;
use pico_rs::gpio::GpioBank;

#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::secure_exe();

const CLOCK_FREQ_HZ: u32 = 12_000_000;

#[entry]
fn main() -> ! {
    info!("Direct GPIO Test");

    let gpio = GpioBank::init();

    let led = gpio.pin25();
    led.into_output();

    info!("LED configured as output");

    let mut delay = Delay::new(CLOCK_FREQ_HZ);

    loop {
        led.set_high();
        info!("LED HIGH");
        delay.delay_ms(500);

        led.set_low();
        info!("LED LOW");
        delay.delay_ms(500);
    }
}
