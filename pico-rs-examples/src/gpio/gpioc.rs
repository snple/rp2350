//! GPIOC (GPIO Coprocessor) example for RP2350
//!
//! Demonstrates using the M33 GPIO coprocessor for high-speed GPIO operations.
//! The GPIOC uses CP0 MCR/MRC instructions instead of SIO memory-mapped I/O,
//! providing single-cycle atomic GPIO access.

#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;

use pico_rs::Delay;
use pico_rs::block::ImageDef;
use pico_rs::entry;
use pico_rs::gpio::Pins;
use pico_rs::gpioc;
use pico_rs::pac;
use pico_rs::rt::peripheral::Peripherals as CorePeripherals;

/// RP2350 Boot Block - tells the Boot ROM this is a valid ARM executable
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::exe();

/// RP2350 default boot clock frequency
/// At boot, RP2350 runs from ROSC at approximately 12 MHz
const CLOCK_FREQ_HZ: u32 = 12_000_000;

const LED_PIN: u8 = 6;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    // Initialize pins — configures IO_BANK0 and PADS_BANK0
    let pins = Pins::new(dp.IO_BANK0, dp.PADS_BANK0);

    // Configure GPIO as SIO output
    let _led = pins.pin::<LED_PIN>().into_output();

    info!("GPIOC example starting — blinking LED via coprocessor");

    // Take core peripherals (NVIC, SCB, SYST, MPU, FPU, SAU)
    let core_peripherals = CorePeripherals::take().unwrap();

    // Initialize SysTick-based delay
    let delay = Delay::new(core_peripherals.SYST, CLOCK_FREQ_HZ);

    loop {
        // --- GPIOC mask-based set/clear (fast, atomic) ---
        gpioc::out_set(1 << LED_PIN);
        delay.delay_ms(500);

        gpioc::out_clr(1 << LED_PIN);
        delay.delay_ms(500);

        // --- GPIOC single-bit operations ---
        gpioc::bit_out_set(LED_PIN.into());
        delay.delay_ms(500);

        gpioc::bit_out_clr(LED_PIN.into());
        delay.delay_ms(500);

        // --- GPIOC XOR toggle ---
        gpioc::out_xor(1 << LED_PIN);
        delay.delay_ms(500);

        gpioc::bit_out_toggle(LED_PIN.into());
        delay.delay_ms(500);

        // --- Read back via GPIOC ---
        let input_state = gpioc::in_lo(); // equivalent to sio_hw->gpio_in
        let output_state = gpioc::out_lo_get(); // equivalent to sio_hw->gpio_out
        info!("in={:#034b}, out={:#034b}", input_state, output_state);
    }
}
