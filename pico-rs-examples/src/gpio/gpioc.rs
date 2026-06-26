//! GPIOC (GPIO Coprocessor) example for RP2350
//!
//! Demonstrates using the M33 GPIO coprocessor for high-speed GPIO operations.
//! The GPIOC uses CP0 MCR/MRC instructions instead of SIO memory-mapped I/O,
//! providing single-cycle atomic GPIO access.

#![no_std]
#![no_main]

use defmt::info;
use pico_rs::clocks::{ClockBuilder, ClockResources};
use pico_rs::entry;
use pico_rs::gpio::Pins;
use pico_rs::gpioc;
use pico_rs::pac;

const LED_PIN: u8 = 25;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    // Initialize clocks
    let clocks = ClockBuilder::new().build(ClockResources {
        clocks: dp.CLOCKS,
        xosc: dp.XOSC,
        pll_sys: dp.PLL_SYS,
        pll_usb: dp.PLL_USB,
    });

    // Initialize pins — configures IO_BANK0 and PADS_BANK0
    let pins = Pins::new(dp.IO_BANK0, dp.PADS_BANK0);

    // Configure GPIO 25 as SIO output
    let _led = pins.pin::<LED_PIN>().into_output();

    info!("GPIOC example starting — blinking LED via coprocessor");

    loop {
        // --- GPIOC mask-based set/clear (fast, atomic) ---
        gpioc::out_set(1 << LED_PIN);
        delay_ms(clocks.sys_freq(), 500);

        gpioc::out_clr(1 << LED_PIN);
        delay_ms(clocks.sys_freq(), 500);

        // --- GPIOC single-bit operations ---
        gpioc::bit_out_set(LED_PIN.into());
        delay_ms(clocks.sys_freq(), 200);

        gpioc::bit_out_clr(LED_PIN.into());
        delay_ms(clocks.sys_freq(), 200);

        // --- GPIOC XOR toggle ---
        gpioc::out_xor(1 << LED_PIN);
        delay_ms(clocks.sys_freq(), 500);

        gpioc::bit_out_toggle(LED_PIN.into());
        delay_ms(clocks.sys_freq(), 500);

        // --- Read back via GPIOC ---
        let input_state = gpioc::in_lo(); // equivalent to sio_hw->gpio_in
        let output_state = gpioc::out_lo_get(); // equivalent to sio_hw->gpio_out
        info!("in={:#034b}, out={:#034b}", input_state, output_state);
    }
}

fn delay_ms(sys_freq: u32, ms: u32) {
    let cycles = sys_freq / 1000 * ms;
    for _ in 0..cycles {
        core::hint::spin_loop();
    }
}
