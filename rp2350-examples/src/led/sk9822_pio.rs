//! # SK9822 / APA102 RGB LED Driver (PIO)
//!
//! Drives SK9822 (APA102-compatible) addressable RGB LEDs using the PIO
//! peripheral to bit-bang an SPI-like protocol. This frees up the hardware
//! SPI for other devices and allows arbitrary pin choices.
//!
//! ## Protocol
//!
//! ```text
//! Start Frame : 4 bytes of 0x00
//! LED Frame   : [111_BBBBB] [Blue] [Green] [Red]   (4 bytes per LED)
//!               ^^^-always 1  ^^^^^-global brightness (0-31)
//! End Frame   : ceil(NUM_LEDS / 2) bits of HIGH on CLK
//!               ≈ (NUM_LEDS / 16 + 1) bytes of 0xFF
//! ```
//!
//! ## PIO Scheme
//!
//! - CLK is driven by **side-set** (1 pin)
//! - DATA is driven by **OUT pins** (1 pin)
//! - 2 instructions, 4 PIO cycles per bit (CPOL=0, CPHA=0)
//!
//! ## Wiring
//!
//! ```text
//!   PIO CLK  (GPIO 2) ──── CKI (SK9822)
//!   PIO DATA (GPIO 3) ──── SDI (SK9822)
//!   GND               ──── GND
//!   VBUS (5V)         ──── VCC
//! ```

#![no_std]
#![no_main]

#[cfg(feature = "defmt")]
use defmt_rtt as _;
use rp2350_examples::log::*;

use rp2350::clocks::ClockBuilder;
use rp2350::gpio::Pins;
use rp2350::main;
use rp2350::pac::Peripherals;
use rp2350::pio;
use rp2350::pio::{FifoJoin, Pio0};
use rp2350::pio_asm;
use rp2350_examples::common::led::hsv_to_rgb;

// Configuration
// ============================================================================

/// Clock pin for the SK9822 CKI line
const CLK_PIN: u8 = 2;

/// Data pin for the SK9822 SDI line
const DATA_PIN: u8 = 3;

/// Number of LEDs in the strip
const NUM_LEDS: usize = 8;

/// Global brightness for all LEDs (0-31)
const BRIGHTNESS: u8 = 4;

/// Target SPI-like clock rate (SK9822 supports up to ~30 MHz)
const CLK_FREQ: u32 = 4_000_000;

// ============================================================================
// Pixel Type
// ============================================================================

/// A single SK9822 LED pixel.
#[derive(Clone, Copy, Debug)]
struct Pixel {
    r: u8,
    g: u8,
    b: u8,
    brightness: u8, // 0-31
}

impl Pixel {
    const fn new(r: u8, g: u8, b: u8, brightness: u8) -> Self {
        Self {
            r,
            g,
            b,
            brightness: if brightness > 31 { 31 } else { brightness },
        }
    }

    /// Encode as a 32-bit word (MSB first: header, B, G, R)
    const fn to_u32(self) -> u32 {
        ((0xE0 | (self.brightness & 0x1F)) as u32) << 24
            | (self.b as u32) << 16
            | (self.g as u32) << 8
            | (self.r as u32)
    }
}

// ============================================================================
// SK9822 PIO Driver
// ============================================================================

/// Initialize PIO0/SM0 for SK9822 (SPI-like bit-bang).
///
/// Uses side-set for CLK and OUT for DATA.
/// The PIO program clocks out 32 bits per FIFO word (autopull).
fn sk9822_pio_init(pio0: &mut Pio0, sys_clk: u32) {
    // PIO program: SPI master TX-only with side-set clock.
    //
    // Protocol: data is sampled on the rising CLK edge (CPOL=0, CPHA=0).
    //   - Drive DATA on side LOW (falling edge / setup)
    //   - Assert side HIGH (rising edge) → SK9822 samples
    //
    // 2 instructions × 2 delay cycles each = 4 PIO cycles per bit.
    let program = pio_asm!(
        r#"
        .program sk9822_spi
        .side_set 1

        .wrap_target
            out pins, 1    side 0 [1]  ; set DATA bit, CLK LOW (setup)
            nop            side 1 [1]  ; CLK HIGH (sample edge)
        .wrap
        "#
    );

    let installed = pio0
        .install_program(&program)
        .expect("SK9822: PIO program install failed");

    // 4 PIO cycles per bit → clk_div = sys_clk / (clk_freq * 4)
    let div_full = sys_clk / (CLK_FREQ * 4);
    let int_div = div_full as u16;

    info!(
        "SK9822 PIO: clkdiv={}, offset={}",
        int_div, installed.offset
    );

    pio0.sm0()
        .apply_program(&installed)
        .set_clkdiv(int_div, 0)
        // DATA pin via OUT
        .set_out_pins(DATA_PIN, 1)
        // CLK pin via side-set
        .set_sideset(CLK_PIN, 1, false, false)
        // Shift left (MSB first), autopull at 32 bits
        .set_out_shift(false, true, 32)
        // TX-only FIFO (8-deep)
        .set_fifo_join(FifoJoin::Tx)
        // Pin directions: both output
        .set_consecutive_pindirs(CLK_PIN, 1, true)
        .set_consecutive_pindirs(DATA_PIN, 1, true)
        .init(installed.offset)
        .enable();
}

/// Transmit a full SK9822 frame via PIO.
fn sk9822_write(pio0: &Pio0, pixels: &[Pixel]) {
    let sm = pio0.sm0();

    // Start frame: 32 bits of 0
    sm.tx_fifo_write(0x00000000);

    // LED frames (each is a 32-bit word)
    for &px in pixels {
        sm.tx_fifo_write(px.to_u32());
    }

    // End frame: ceil(NUM_LEDS/2) bits HIGH → round up to 32-bit words
    let end_bits = pixels.len().div_ceil(2);
    let end_words = end_bits.div_ceil(32);
    for _ in 0..end_words.max(1) {
        sm.tx_fifo_write(0xFFFFFFFF);
    }
}

// ============================================================================
// Entry Point
// ============================================================================

#[main]
fn main() -> ! {
    info!("SK9822 PIO LED example");

    let dp = Peripherals::take().unwrap();
    let clocks = ClockBuilder::new().build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);
    let pins = Pins::init(dp.IO_BANK0, dp.PADS_BANK0);

    let delay = rp2350_examples::common::board::init_delay(&clocks);

    // Take PIO0 and claim CLK + DATA pins
    let mut pio0 = pio::Pio0::init(dp.PIO0);
    pio0.claim_pin(pins.pin::<2>()); // CLK
    pio0.claim_pin(pins.pin::<3>()); // DATA

    sk9822_pio_init(&mut pio0, clocks.sys_freq());

    info!(
        "SK9822 PIO: {} LEDs, brightness={}, clk_freq={} Hz",
        NUM_LEDS, BRIGHTNESS, CLK_FREQ
    );

    // Pixel buffer
    let mut pixels = [Pixel::new(0, 0, 0, BRIGHTNESS); NUM_LEDS];
    let mut hue_offset: u16 = 0;

    loop {
        // Generate a rainbow pattern
        for (i, pixel) in pixels.iter_mut().enumerate() {
            let hue = ((hue_offset + (i as u16 * 256 / NUM_LEDS as u16)) % 256) as u8;
            let (r, g, b) = hsv_to_rgb(hue, 255, 64);
            *pixel = Pixel::new(r, g, b, BRIGHTNESS);
        }

        sk9822_write(&pio0, &pixels);
        delay.delay_ms(20);
        hue_offset = (hue_offset + 1) % 256;
    }
}
