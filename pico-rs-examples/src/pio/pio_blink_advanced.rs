//! # PIO Blink Advanced Example
//!
//! Use PIO to blink the LED on GPIO 25 with configurable delay.
//! This demonstrates the full .pio file syntax with pull/out instructions
//! and runtime-configurable blink rate via FIFO.

#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;

use pico_rs::Delay;
use pico_rs::block::ImageDef;
use pico_rs::clocks::{ClockBuilder};
use pico_rs::entry;
use pico_rs::gpio::Pins;
use pico_rs::pac::Peripherals;
use pico_rs::pio;
use pico_rs::pio_asm;
use pico_rs::rt::peripheral::Peripherals as CorePeripherals;

/// RP2350 Boot Block
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::exe();

const LED_PIN: u8 = 25;

#[entry]
fn main() -> ! {
    info!("PIO Blink Advanced Example");

    // Take device peripherals
    let dp = Peripherals::take().unwrap();

    // Initialize clocks (150 MHz sys by default)
    let clocks = ClockBuilder::new().build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);

    // Initialize GPIO pins
    let pins = Pins::new(dp.IO_BANK0, dp.PADS_BANK0);

    // Take PIO0 block
    let mut pio0 = pio::Pio0::new(dp.PIO0);

    // Claim GPIO 25 for PIO0 (takes ownership)
    pio0.claim_pin(pins.pin::<25>());

    // PIO program: delay is read from FIFO for runtime control
    let program = pio_asm!(
        r#"
        .program blink
            pull block      ; Get delay count from FIFO
            out y, 32       ; Move to Y register (our delay counter template)
        .wrap_target
            mov x, y        ; Copy delay to X
            set pins, 1     ; Turn LED on
        lp1:
            jmp x-- lp1     ; Delay for (x + 1) cycles
            mov x, y        ; Copy delay to X again
            set pins, 0     ; Turn LED off
        lp2:
            jmp x-- lp2     ; Delay for the same number of cycles
        .wrap               ; Loop forever
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

            let sm = pio0.sm0();

            // Configure SM using builder pattern
            sm.apply_program(&installed)
                .set_clkdiv(1, 0) // Full speed
                .set_set_pins(LED_PIN, 1)
                .set_consecutive_pindirs(LED_PIN, 1, true)
                .set_out_shift(true, false, 32)
                .init(installed.offset);

            // Push delay value before enabling
            // At 150 MHz, delay of 75_000_000 = 0.5 seconds per state
            let delay_cycles = 75_000_000u32;
            info!(
                "Delay cycles: {} (~{} ms)",
                delay_cycles,
                delay_cycles / (clocks.sys_freq() / 1000)
            );
            sm.tx_fifo_write(delay_cycles);

            // Enable the state machine
            sm.set_enabled(true);

            info!("PIO state machine started - LED blinking!");
        }
        Err(e) => {
            error!("Failed to install program: {:?}", e);
        }
    }

    // Keep running
    let core = CorePeripherals::take().unwrap();
    let delay = Delay::new(core.SYST, clocks.sys_freq());
    loop {
        delay.delay_ms(5000);
        info!("Running...");
    }
}
