//! # PIO + DMA Example
//!
//! Demonstrates using DMA to feed data into a PIO state machine's TX FIFO.
//! The PIO program outputs the data as a serial bitstream on a GPIO pin.
//!
//! ## Wiring
//!
//! - GPIO 25 → On-board LED (Pico 2 board)
//!
//! The LED will blink in a pattern defined by the data buffer.
//!
//! ## What this example does
//!
//! 1. Loads a simple PIO "autopull + OUT PINS" program
//! 2. Fills a DMA buffer with blink pattern data
//! 3. DMA feeds the pattern into PIO's TX FIFO
//! 4. PIO shifts bits out to the LED

#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;

use rp2350::clocks::ClockBuilder;
use rp2350::dma::{DataSize, Dma};
use rp2350::gpio::Pins;
use rp2350::main;
use rp2350::pac::Peripherals;
use rp2350::pio;
use rp2350::pio::Pio0Sm0;
use rp2350::pio_asm;
/// Blink pattern: each bit controls the LED for one PIO cycle.
/// At ~1 Hz per bit (with appropriate clock divider), this creates
/// a visible blink pattern.
///
/// Each u32 = 32 bits of LED on/off pattern.
static PATTERN: [u32; 4] = [
    0b11111111_00000000_11110000_11001100, // complex pattern
    0b10101010_01010101_11100111_00011000, // alternating
    0b11111111_11111111_00000000_00000000, // long on, long off
    0b10001000_10001000_10001000_10001000, // short blinks
];

#[main]
fn main() -> ! {
    info!("PIO + DMA blink pattern example");

    let dp = Peripherals::take().unwrap();
    let _clocks = ClockBuilder::new().build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);
    let pins = Pins::init(dp.IO_BANK0, dp.PADS_BANK0);

    // ------------------------------------------------------------------
    // PIO setup
    // ------------------------------------------------------------------
    let mut pio0 = pio::Pio0::init(dp.PIO0);
    pio0.claim_pin(pins.pin::<25>());

    // PIO program: autopull 32 bits, shift out 1 bit at a time to pin
    let program = pio_asm!(
        r#"
        .wrap_target
        out pins, 1 [31]     ; shift 1 bit to pin, delay 31 cycles
        .wrap
        "#
    );

    let installed = match pio0.install_program(&program) {
        Ok(p) => p,
        Err(e) => {
            error!("Failed to install PIO program: {:?}", e);
            loop {
                core::hint::spin_loop();
            }
        }
    };

    // Configure SM0:
    // - ~1 Hz per bit: 150 MHz / 65535 / 32 ≈ ~0.07 Hz per bit
    //   Use divider 46875 for ~100 ms per bit (150M / 46875 / 32 ≈ 100Hz)
    let mut sm0 = pio0.sm0();
    sm0.apply_program(&installed)
        .set_clkdiv(46875, 0) // ~3.2 kHz instruction rate, ~100 Hz per bit with delay
        .set_out_pins(25, 1)
        .set_set_pins(25, 1)
        .set_consecutive_pindirs(25, 1, true)
        .set_out_shift(true, true, 32) // shift right, autopull, threshold=32
        .init(installed.offset)
        .enable();

    info!("PIO state machine started");

    // ------------------------------------------------------------------
    // DMA: memory → PIO TX FIFO
    // ------------------------------------------------------------------
    let dma = Dma::init(dp.DMA);

    // Total bits: 4 words × 32 bits = 128 LED state changes
    dma.channel::<0>()
        .mem_to_periph(
            PATTERN.as_ptr() as u32,
            sm0.tx_fifo_address(),
            PATTERN.len() as u32,
            Pio0Sm0::TX_DREQ,
            DataSize::Word,
        )
        .trigger();

    info!("DMA started — {} words of pattern data", PATTERN.len());

    // Wait for the DMA transfer to complete
    dma.channel::<0>().wait();
    info!("DMA complete — all pattern data sent to PIO");

    // PIO may still be shifting out the last bits in its FIFO
    info!("Waiting for PIO to finish shifting...");
    loop {
        core::hint::spin_loop();
    }
}
