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
use pico_rs::entry;
use pico_rs::gpio;
use pico_rs::pio;
use pico_rs::pio_asm;

/// RP2350 Boot Block
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::secure_exe();

const CLOCK_FREQ_HZ: u32 = 12_000_000;
const LED_PIN: u8 = 25;

#[entry]
fn main() -> ! {
    info!("PIO Blink Example");

    // Initialize GPIO subsystem
    gpio::init_gpio();

    // Configure GPIO 25 for PIO0 control
    configure_gpio_for_pio(LED_PIN);

    // Take PIO0 block (this also unresets it)
    let mut pio = pio::Pio0::take();

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
    match pio.install_program(program.instructions, None) {
        Ok(installed) => {
            info!("Program installed at offset {}", installed.offset);

            // Get state machine 0 (using typed sm0() API)
            let sm = pio.sm0();

            // Configure the state machine
            // Clock divider: 256 for slow, visible blink
            // At 12MHz / 256 = ~46.875 kHz instruction rate
            // With 32 delay cycles per instruction, ~732 Hz toggle (~1.4ms period)
            sm.set_clock_divisor(256, 0);

            // Set wrap addresses (must add offset!)
            let wrap_bottom = installed.offset + program.wrap_target as u8;
            let wrap_top = installed.offset + program.wrap as u8;
            sm.set_wrap(wrap_bottom, wrap_top);

            // Configure SET pins - maps SET instructions to GPIO 25
            sm.set_set_pins(LED_PIN, 1);

            // Set GPIO 25 as output
            sm.set_consecutive_pindirs(LED_PIN, 1, true);

            // Restart state machine and jump to program start
            sm.restart();
            sm.restart_clock_divider();
            sm.exec(pio::encode_jmp(installed.offset));

            // Enable the state machine
            sm.set_enabled(true);

            info!("PIO state machine started - LED blinking!");
        }
        Err(e) => {
            error!("Failed to install program: {:?}", e);
        }
    }

    // Keep running
    let mut delay = Delay::new(CLOCK_FREQ_HZ);
    loop {
        delay.delay_ms(5000);
        info!("Running...");
    }
}

/// Configure a GPIO pin for PIO0 control
///
/// On RP2350, this requires:
/// 1. Configure the pad (clear ISO, enable output)
/// 2. Set funcsel to 6 (PIO0)
fn configure_gpio_for_pio(pin: u8) {
    unsafe {
        // Configure pad: enable input, disable output-disable, clear isolation
        let pads = &*pico_rs::pac::PADS_BANK0::PTR;
        let pad = pads.gpio(pin);
        pad.set_ie(); // Enable input (for readback)
        pad.clr_od(); // Disable output disable
        pad.clr_iso(); // Clear isolation (CRITICAL for RP2350!)

        // Set GPIO function to PIO0 (funcsel=6), clear all overrides
        let io_bank = &*pico_rs::pac::IO_BANK0::PTR;
        let ctrl = io_bank.gpio_ctrl(pin);
        ctrl.write_bits(6); // funcsel = 6, all other bits = 0
    }
}
