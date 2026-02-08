//! # PIO Blink Example
//!
//! Use PIO to blink the LED on GPIO 25.
//! This demonstrates basic PIO programming using the pio_asm! macro.

#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

use pico_rs::Delay;
use pico_rs::block::ImageDef;
use pico_rs::clocks::ClockBuilder;
use pico_rs::entry;
use pico_rs::gpio::Pins;
use pico_rs::pac::Peripherals;
use pico_rs::rt::peripheral::Peripherals as CorePeripherals;
use pico_rs::pio;
use pico_rs::pio_asm;

/// RP2350 Boot Block
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::exe();

const LED_PIN: u8 = 25;

#[entry]
fn main() -> ! {
    info!("PIO Blink Example");

    // Take device peripherals
    let dp = Peripherals::take().unwrap();

    // Initialize clocks (150 MHz sys by default)
    let clocks = ClockBuilder::new().build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);

    // Initialize GPIO pins
    let pins = Pins::new(dp.IO_BANK0, dp.PADS_BANK0);

    // Take PIO0 block (this also unresets it)
    let mut pio0 = pio::Pio0::take();

    // Claim GPIO 25 for PIO0 (takes ownership, configures pad + funcsel)
    pio0.claim_pin(pins.p25);

    // Simple blink program: toggles pin with 32-cycle delays
    let program = pio_asm!(
        r#"
        set pindirs, 1      ; Set pin as output (executed once)
        .wrap_target
        set pins, 1 [31]    ; Set pin high, delay 31 cycles
        set pins, 0 [31]    ; Set pin low, delay 31 cycles
        .wrap
        "#
    );

    info!(
        "Program: {} instructions, wrap {} -> {}",
        program.instructions.len(),
        program.wrap_target,
        program.wrap
    );

    // Install the program
    match pio0.install_program(&program) {
        Ok(installed) => {
            info!("Program installed at offset {}", installed.offset);

            // Configure and init SM using builder pattern
            // Clock: 12MHz / 256 = ~46.875 kHz instruction rate
            pio0.sm0()
                .apply_program(&installed)
                .set_clkdiv(256, 0)
                .set_set_pins(LED_PIN, 1)
                .set_consecutive_pindirs(LED_PIN, 1, true)
                .init(installed.offset)
                .set_enabled(true);

            info!("PIO state machine started - LED blinking!");
        }
        Err(e) => {
            error!("Failed to install program: {:?}", e);
        }
    }

    // Keep running
    // Initialize delay
    let core = CorePeripherals::take().unwrap();
    let mut delay = Delay::new(core.SYST, clocks.sys_freq());
    loop {
        delay.delay_ms(5000);
        info!("Running...");
    }
}
