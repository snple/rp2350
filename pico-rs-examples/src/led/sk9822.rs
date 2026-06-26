//! # SK9822 / APA102 RGB LED Driver (SPI + PIO)
//!
//! Drives SK9822 (APA102-compatible) addressable RGB LEDs. These LEDs use a
//! two-wire SPI-like protocol (CLK + DATA) which allows reliable communication
//! at a wide range of clock speeds.
//!
//! This example demonstrates **two** approaches:
//! 1. **SPI** — simplest, uses the hardware SPI peripheral
//! 2. **PIO** — bit-bangs the SPI-like protocol, freeing up the SPI peripheral
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
//! ## Wiring
//!
//! ```text
//!   Pico GPIO (CLK)  ──── CKI (SK9822)
//!   Pico GPIO (DATA) ──── SDI (SK9822)
//!   GND              ──── GND
//!   VBUS(5V)         ──── VCC
//! ```

#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;

use pico_rs::block::ImageDef;
use pico_rs::clocks::{ClockBuilder, ClockResources};
use pico_rs::entry;
use pico_rs::gpio::Pins;
use pico_rs::pac::Peripherals;
use pico_rs::pio;
use pico_rs::pio::{FifoJoin, Pio0};
use pico_rs::pio_asm;
use pico_rs::rt::peripheral::Peripherals as CorePeripherals;
use pico_rs::spi::{Mode, Spi0};
use pico_rs::systick::Delay;

/// RP2350 Boot Block
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::exe();

// ============================================================================
// Configuration — choose ONE backend at compile time
// ============================================================================

/// Set to `true` to use PIO, `false` to use SPI.
const USE_PIO: bool = false;

/// Number of LEDs in the strip
const NUM_LEDS: usize = 8;

/// Global brightness for all LEDs (0-31)
const BRIGHTNESS: u8 = 4;

// -- SPI pins (used when USE_PIO == false) --
// SPI0: SCK = GPIO 2, TX(MOSI) = GPIO 3, RX(MISO) = GPIO 0 (unused but required)
#[allow(dead_code)]
const SPI_SCK_PIN: u8 = 2;
#[allow(dead_code)]
const SPI_TX_PIN: u8 = 3;
#[allow(dead_code)]
const SPI_RX_PIN: u8 = 0;

// -- PIO pins (used when USE_PIO == true) --
const PIO_CLK_PIN: u8 = 2;
const PIO_DATA_PIN: u8 = 3;

/// SPI baudrate (SK9822 supports up to ~30 MHz, we use 4 MHz)
const SPI_BAUDRATE: u32 = 4_000_000;

// ============================================================================
// Color Helpers
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

    /// Encode the 4-byte LED frame: [0xE0 | brightness] [B] [G] [R]
    const fn to_bytes(self) -> [u8; 4] {
        [0xE0 | (self.brightness & 0x1F), self.b, self.g, self.r]
    }

    /// Encode as a 32-bit word (MSB first: header, B, G, R)
    const fn to_u32(self) -> u32 {
        ((0xE0 | (self.brightness & 0x1F)) as u32) << 24
            | (self.b as u32) << 16
            | (self.g as u32) << 8
            | (self.r as u32)
    }
}

/// Simple HSV to RGB conversion (all values 0-255).
fn hsv_to_rgb(h: u8, s: u8, v: u8) -> (u8, u8, u8) {
    if s == 0 {
        return (v, v, v);
    }

    let region = h / 43;
    let remainder = (h as u16 - region as u16 * 43) * 6;

    let p = ((v as u16 * (255 - s as u16)) >> 8) as u8;
    let q = ((v as u16 * (255 - ((s as u16 * remainder) >> 8))) >> 8) as u8;
    let t = ((v as u16 * (255 - ((s as u16 * (255 - remainder)) >> 8))) >> 8) as u8;

    match region {
        0 => (v, t, p),
        1 => (q, v, p),
        2 => (p, v, t),
        3 => (p, q, v),
        4 => (t, p, v),
        _ => (v, p, q),
    }
}

// ============================================================================
// SK9822 Protocol — shared helpers
// ============================================================================

/// Number of end-frame bytes needed: ceil(NUM_LEDS / 2) bits → bytes
const fn end_frame_bytes(num_leds: usize) -> usize {
    (num_leds + 15) / 16 * 4
}

// ============================================================================
// Backend: SPI
// ============================================================================

/// Transmit full frame via SPI.
fn sk9822_spi_write<SCK, TX, RX>(spi: &Spi0<SCK, TX, RX>, pixels: &[Pixel])
where
    SCK: pico_rs::gpio::Spi0SckPin,
    TX: pico_rs::gpio::Spi0TxPin,
    RX: pico_rs::gpio::Spi0RxPin,
{
    // Start frame: 4 × 0x00
    spi.write(&[0x00; 4]);

    // LED frames
    for &px in pixels {
        spi.write(&px.to_bytes());
    }

    // End frame
    let end_len = end_frame_bytes(pixels.len());
    for _ in 0..end_len {
        spi.write(&[0xFF]);
    }
}

// ============================================================================
// Backend: PIO
// ============================================================================

/// Initialize PIO0/SM0 for SK9822 (SPI-like bit-bang).
///
/// Uses side-set for CLK and OUT for DATA.
/// The PIO program clocks out 8 bits per word at the configured rate.
fn sk9822_pio_init(pio0: &mut Pio0, sys_clk: u32) {
    // PIO program: SPI master TX-only with side-set clock.
    //
    // Protocol: data is sampled on the rising CLK edge (CPOL=0, CPHA=0).
    //   - Drive DATA on side LOW (falling edge)
    //   - Assert side HIGH (rising edge) → SK9822 samples
    //
    // We shift 32 bits per FIFO word (autopull threshold=32).
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

    // 4 PIO cycles per bit → clk_div = sys_clk / (spi_rate * 4)
    let div_full = sys_clk / (SPI_BAUDRATE * 4);
    let int_div = div_full as u16;

    info!(
        "SK9822 PIO: clkdiv={}, offset={}",
        int_div, installed.offset
    );

    pio0.sm0()
        .apply_program(&installed)
        .set_clkdiv(int_div, 0)
        // DATA pin = PIO_DATA_PIN via OUT
        .set_out_pins(PIO_DATA_PIN, 1)
        // CLK pin = PIO_CLK_PIN via side-set
        .set_sideset(PIO_CLK_PIN, 1, false, false)
        // Shift left (MSB first), autopull at 32 bits
        .set_out_shift(false, true, 32)
        // TX-only FIFO
        .set_fifo_join(FifoJoin::Tx)
        // Pin directions: both output
        .set_consecutive_pindirs(PIO_CLK_PIN, 1, true)
        .set_consecutive_pindirs(PIO_DATA_PIN, 1, true)
        .init(installed.offset)
        .set_enabled(true);
}

/// Transmit full frame via PIO.
fn sk9822_pio_write(pio0: &Pio0, pixels: &[Pixel]) {
    let sm = pio0.sm0();

    // Start frame: 32 bits of 0
    sm.tx_fifo_write(0x00000000);

    // LED frames (each is a 32-bit word)
    for &px in pixels {
        sm.tx_fifo_write(px.to_u32());
    }

    // End frame: ceil(NUM_LEDS/2) bits HIGH → round up to 32-bit words
    let end_bits = (pixels.len() + 1) / 2;
    let end_words = (end_bits + 31) / 32;
    for _ in 0..end_words.max(1) {
        sm.tx_fifo_write(0xFFFFFFFF);
    }
}

// ============================================================================
// Entry Point
// ============================================================================

#[entry]
fn main() -> ! {
    info!(
        "SK9822 LED example ({})",
        if USE_PIO { "PIO" } else { "SPI" }
    );

    let dp = Peripherals::take().unwrap();
    let clocks = ClockBuilder::new().build(ClockResources { clocks: dp.CLOCKS, xosc: dp.XOSC, pll_sys: dp.PLL_SYS, pll_usb: dp.PLL_USB });
    let pins = Pins::new(dp.IO_BANK0, dp.PADS_BANK0);

    let core = CorePeripherals::take().unwrap();
    let delay = Delay::new(core.SYST, clocks.sys_freq());

    info!("SK9822: {} LEDs, brightness={}", NUM_LEDS, BRIGHTNESS);

    // Pixel buffer
    let mut pixels = [Pixel::new(0, 0, 0, BRIGHTNESS); NUM_LEDS];
    let mut hue_offset: u16 = 0;

    if USE_PIO {
        // ---- PIO backend ----
        let mut pio0 = pio::Pio0::new(dp.PIO0);
        pio0.claim_pin(pins.pin::<2>()); // CLK
        pio0.claim_pin(pins.pin::<3>()); // DATA

        sk9822_pio_init(&mut pio0, clocks.sys_freq());

        loop {
            for i in 0..NUM_LEDS {
                let hue = ((hue_offset + (i as u16 * 256 / NUM_LEDS as u16)) % 256) as u8;
                let (r, g, b) = hsv_to_rgb(hue, 255, 64);
                pixels[i] = Pixel::new(r, g, b, BRIGHTNESS);
            }
            sk9822_pio_write(&pio0, &pixels);
            delay.delay_ms(20);
            hue_offset = (hue_offset + 1) % 256;
        }
    } else {
        // ---- SPI backend ----
        let spi = Spi0::new(
            dp.SPI0,
            pins.pin::<2>(), // SCK
            pins.pin::<3>(), // TX (MOSI → DATA)
            pins.pin::<0>(), // RX (MISO, unused but required by API)
            SPI_BAUDRATE,
            clocks.sys_freq(),
        );
        // SK9822 uses CPOL=0, CPHA=0 (Mode 0) — already the default
        spi.set_mode(Mode::Mode0);

        info!("SPI0 baudrate = {} Hz", spi.baudrate(clocks.sys_freq()));

        loop {
            for i in 0..NUM_LEDS {
                let hue = ((hue_offset + (i as u16 * 256 / NUM_LEDS as u16)) % 256) as u8;
                let (r, g, b) = hsv_to_rgb(hue, 255, 64);
                pixels[i] = Pixel::new(r, g, b, BRIGHTNESS);
            }
            sk9822_spi_write(&spi, &pixels);
            delay.delay_ms(20);
            hue_offset = (hue_offset + 1) % 256;
        }
    }
}
