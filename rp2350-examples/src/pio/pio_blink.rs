//! # PIO Blink Example
//!
//! Use PIO to blink the LED on GPIO 25.
//! This demonstrates basic PIO programming using the pio_asm! macro.

#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;

use rp2350::clocks::ClockBuilder;
use rp2350::gpio::Pins;
use rp2350::main;
use rp2350::pac::Peripherals;
use rp2350::pio;
use rp2350::pio_asm;

const LED_PIN: u8 = 25;

#[main]
fn main() -> ! {
    info!("PIO Blink Example");

    // Take device peripherals
    let dp = Peripherals::take().unwrap();

    // Initialize clocks (150 MHz sys by default)
    let clocks = ClockBuilder::new().build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);

    // Initialize GPIO pins
    let pins = Pins::init(dp.IO_BANK0, dp.PADS_BANK0);

    // Take PIO0 block (this also unresets it)
    let mut pio0 = pio::Pio0::init(dp.PIO0);

    // Claim GPIO 25 for PIO0 (takes ownership, configures pad + funcsel)
    pio0.claim_pin(pins.pin::<25>());

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
            let mut sm0 = pio0.sm0();
            sm0.apply_program(&installed)
                .set_clkdiv(256, 0)
                .set_set_pins(LED_PIN, 1)
                .set_consecutive_pindirs(LED_PIN, 1, true)
                .init(installed.offset)
                .enable();

            info!("PIO state machine started - LED blinking!");
        }
        Err(e) => {
            error!("Failed to install program: {:?}", e);
        }
    }

    // Keep running
    // Initialize delay (matches actual system clock)
    let delay = rp2350_examples::common::board::init_delay(&clocks);
    loop {
        delay.delay_ms(5000);
        info!("Running...");
    }
}
