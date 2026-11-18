//! # SK9822 / APA102 RGB LED Driver (SPI)
//!
//! Drives SK9822 (APA102-compatible) addressable RGB LEDs using the hardware
//! SPI peripheral. The two-wire SPI protocol (CLK + DATA) maps directly to
//! SPI SCK and MOSI, making this the simplest approach.
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
//!   SPI0 SCK  (GPIO 2) ──── CKI (SK9822)
//!   SPI0 MOSI (GPIO 3) ──── SDI (SK9822)
//!   GND                ──── GND
//!   VBUS (5V)          ──── VCC
//! ```

#![no_std]
#![no_main]

#[cfg(feature = "defmt")]
use defmt_rtt as _;
use rp2350_examples::log::*;

use rp2350::clocks::ClockBuilder;
use rp2350::gpio::{ErasedPin, Pins};
use rp2350::main;
use rp2350::pac::Peripherals;
use rp2350::spi::{Mode, Spi0};
use rp2350_examples::common::led::hsv_to_rgb;

// Configuration
// ============================================================================

/// Number of LEDs in the strip
const NUM_LEDS: usize = 8;

/// Global brightness for all LEDs (0-31)
const BRIGHTNESS: u8 = 4;

/// SPI baudrate (SK9822 supports up to ~30 MHz, we use 4 MHz)
const SPI_BAUDRATE: u32 = 4_000_000;

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

    /// Encode the 4-byte LED frame: [0xE0 | brightness] [B] [G] [R]
    const fn to_bytes(self) -> [u8; 4] {
        [0xE0 | (self.brightness & 0x1F), self.b, self.g, self.r]
    }
}

// ============================================================================
// SK9822 SPI Driver
// ============================================================================

/// Number of end-frame bytes needed: ceil(NUM_LEDS / 2) bits → bytes
const fn end_frame_bytes(num_leds: usize) -> usize {
    num_leds.div_ceil(16) * 4
}

/// Transmit a full SK9822 frame via SPI.
fn sk9822_write<SCK, TX, RX>(spi: &Spi0<SCK, TX, RX>, pixels: &[Pixel])
where
    SCK: rp2350::gpio::Spi0SckPin,
    TX: rp2350::gpio::Spi0TxPin,
    RX: rp2350::gpio::Spi0RxPin,
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
// Entry Point
// ============================================================================

#[main]
fn main() -> ! {
    info!("SK9822 SPI LED example");

    let dp = Peripherals::take().unwrap();
    let clocks = ClockBuilder::new().build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);
    let pins = Pins::init(dp.IO_BANK0, dp.PADS_BANK0);

    let delay = rp2350_examples::common::board::init_delay(&clocks);

    // Initialize SPI0 (TX-only, no MISO needed)
    let mut spi = Spi0::init(
        dp.SPI0,
        pins.pin::<2>(), // SCK  → CKI
        pins.pin::<3>(), // MOSI → SDI
        ErasedPin,       // No MISO (TX-only)
        SPI_BAUDRATE,
        clocks.sys_freq(),
    );
    spi.set_mode(Mode::Mode0);

    info!(
        "SK9822 SPI: {} LEDs, brightness={}, baudrate={} Hz",
        NUM_LEDS,
        BRIGHTNESS,
        spi.baudrate(clocks.sys_freq())
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

        sk9822_write(&spi, &pixels);
        delay.delay_ms(20);
        hue_offset = (hue_offset + 1) % 256;
    }
}
