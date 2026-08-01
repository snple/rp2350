//! # WS2812 RGB LED Driver (PIO)
//!
//! Drives WS2812 / WS2812B / NeoPixel addressable RGB LEDs using the PIO
//! peripheral on RP2350. The strict single-wire timing requirements of WS2812
//! are met by a compact 4-instruction PIO program with side-set.
//!
//! ## Protocol
//!
//! - Data format: **GRB**, 8 bits per channel, MSB first (24 bits/LED)
//! - Bit encoding (800 kHz):
//!   - `0`: ~350 ns HIGH, ~900 ns LOW
//!   - `1`: ~900 ns HIGH, ~350 ns LOW
//! - Reset: ≥ 280 µs LOW (we use 300 µs)
//!
//! ## Wiring
//!
//! ```text
//!   Pico GPIO ──── DIN (WS2812)
//!   GND       ──── GND
//!   VBUS(5V)  ──── VCC (level shifter recommended for long strips)
//! ```

#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;

use pico_rs::clocks::ClockBuilder;
use pico_rs::gpio::Pins;
use pico_rs::main;
use pico_rs::pac::Peripherals;
use pico_rs::pio;
use pico_rs::pio::{FifoJoin, Pio0};
use pico_rs::pio_asm;
use pico_rs_examples::common::led::{hsv_to_rgb, urgb_u32, ws2812_clkdiv};

// Configuration
// ============================================================================

/// Data pin for the WS2812 DIN line
const WS2812_PIN: u8 = 16;

/// Number of LEDs in the strip
const NUM_LEDS: usize = 8;

/// WS2812 serial data rate
const WS2812_FREQ_HZ: u32 = 800_000;

// ============================================================================
// WS2812 PIO Driver
// ============================================================================

/// Initialize PIO0/SM0 for WS2812 output on `WS2812_PIN`.
///
/// Returns the configured (but already enabled) PIO block.
fn ws2812_pio_init(pio0: &mut Pio0, sys_clk: u32) {
    // ------------------------------------------------------------------
    // PIO program: WS2812 bit-banging with side-set
    //
    //   T1 = 2   (high time for '0' bit)
    //   T2 = 5   (additional high for '1', or low for '0')
    //   T3 = 3   (low tail / autopull stall region)
    //
    // Total: 10 cycles per bit → 800 kHz at correct divider.
    //
    // The OUT instruction uses autopull (threshold 24) so the SM stalls
    // automatically when the TX FIFO is empty, keeping the pin LOW (reset).
    // ------------------------------------------------------------------
    let program = pio_asm!(
        r#"
        .program ws2812
        .side_set 1

        .define public T1 2
        .define public T2 5
        .define public T3 3

        .wrap_target
        bitloop:
            out x, 1       side 0 [T3 - 1]  ; shift 1 bit, drive LOW for T3
            jmp !x do_zero side 1 [T1 - 1]  ; start HIGH pulse, branch on bit value
        do_one:
            jmp bitloop    side 1 [T2 - 1]  ; bit=1 → stay HIGH for T2 more
        do_zero:
            nop            side 0 [T2 - 1]  ; bit=0 → drive LOW for T2
        .wrap
        "#
    );

    let installed = pio0
        .install_program(&program)
        .expect("WS2812: PIO program install failed");

    let (int_div, frac_div) = ws2812_clkdiv(sys_clk, WS2812_FREQ_HZ);

    info!(
        "WS2812: clkdiv int={} frac={}, offset={}",
        int_div, frac_div, installed.offset
    );

    // Configure SM0
    pio0.sm0()
        .apply_program(&installed)
        .set_clkdiv(int_div, frac_div)
        // side-set pin = WS2812_PIN
        .set_sideset(WS2812_PIN, 1, false, false)
        // OUT shifts left (MSB first), autopull after 24 bits
        .set_out_shift(false, true, 24)
        // Join FIFOs: TX only (8-deep TX FIFO)
        .set_fifo_join(FifoJoin::Tx)
        // Pin direction: output
        .set_consecutive_pindirs(WS2812_PIN, 1, true)
        // Start the state machine
        .init(installed.offset)
        .enable();
}

/// Write a buffer of GRB pixel words to the WS2812 strip via PIO TX FIFO.
fn ws2812_put_pixels(pio0: &Pio0, pixels: &[u32]) {
    let sm = pio0.sm0();
    for &pixel in pixels {
        sm.tx_fifo_write(pixel);
    }
}

// ============================================================================
// Entry Point
// ============================================================================

#[main]
fn main() -> ! {
    info!("WS2812 PIO LED example");

    let dp = Peripherals::take().unwrap();

    // Initialize clocks (150 MHz by default)
    let clocks = ClockBuilder::new().build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);

    // Initialize GPIO pins
    let pins = Pins::init(dp.IO_BANK0, dp.PADS_BANK0);

    // Take PIO0 and claim the data pin
    let mut pio0 = pio::Pio0::init(dp.PIO0);
    pio0.claim_pin(pins.pin::<16>());

    // Initialize the WS2812 PIO driver
    ws2812_pio_init(&mut pio0, clocks.sys_freq());

    // Delay source (matches actual system clock)
    let delay = pico_rs_examples::common::board::init_delay(&clocks);

    info!("WS2812: {} LEDs on GPIO {}", NUM_LEDS, WS2812_PIN);

    // Pixel buffer
    let mut pixels = [0u32; NUM_LEDS];
    let mut hue_offset: u16 = 0;

    loop {
        // Generate a rainbow pattern
        for (i, pixel) in pixels.iter_mut().enumerate() {
            let hue = ((hue_offset + (i as u16 * 256 / NUM_LEDS as u16)) % 256) as u8;
            let (r, g, b) = hsv_to_rgb(hue, 255, 32); // low brightness to protect eyes
            *pixel = urgb_u32(r, g, b);
        }

        // Send pixel data
        ws2812_put_pixels(&pio0, &pixels);

        // Wait for reset time (>280 µs) plus frame delay
        delay.delay_ms(20);

        // Advance the hue
        hue_offset = (hue_offset + 1) % 256;
    }
}
