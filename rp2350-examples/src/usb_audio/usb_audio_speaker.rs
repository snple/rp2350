//! # USB Audio Speaker Example
//!
//! This example demonstrates USB Audio Class 1.0 output (speaker) functionality.
//! The Pico 2 enumerates as a USB speaker and receives PCM audio data from the host.
//!
//! ## Hardware
//! - Raspberry Pi Pico 2 (RP2350)
//! - USB cable connected to the Pico's USB port
//!
//! ## Testing
//! 1. Flash this example to the Pico 2
//! 2. Connect USB cable to your computer
//! 3. The device will appear as a USB audio output device (speaker)
//! 4. Select "Pico USB Speaker" as your audio output device
//! 5. Play audio — the Pico will receive PCM data at 48 kHz stereo 16-bit
//! 6. The on-board LED blinks to indicate audio data is being received
//!
//! ## Audio Format
//! - 48 kHz sample rate
//! - 2 channels (stereo)
//! - 16-bit signed little-endian PCM
//! - 192 bytes per USB frame (1 ms)

#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;

use rp2350::SysTick;
use rp2350::clocks::ClockBuilder;
use rp2350::gpio::Pins;
use rp2350::main;
use rp2350::pac::Peripherals;
use rp2350::rt::peripheral::Peripherals as CorePeripherals;

use rp2350_usb::UsbBusAllocator;
use rp2350_usb::class::UsbClass;
use rp2350_usb::class::uac::{AudioClassBuilder, Format, StreamConfig, TerminalType};
use rp2350_usb::device::UsbDeviceBuilder;
use rp2350_usb::rp2350::Rp2350Bus;

// ─── Constants ────────────────────────────────────────────────────────────────

const SYS_CLK_HZ: u32 = 150_000_000;

/// Supported sampling rates.
static RATES_48K: [u32; 1] = [48000];

// Average over 1000 ms to compute audio data rate
const RATE_WINDOW_MS: u32 = 1000;

const LED_PIN: u32 = 6;

// ─── Main ─────────────────────────────────────────────────────────────────────

#[main]
fn main() -> ! {
    info!("USB Audio Speaker Example starting...");

    let dp = Peripherals::take().unwrap();
    let cp = CorePeripherals::take().unwrap();

    // ── Clocks ──────────────────────────────────────────────────────────────
    let clocks = ClockBuilder::new()
        .enable_usb()
        .build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);
    info!(
        "Clocks: sys={} Hz, usb={} Hz",
        clocks.sys_freq(),
        clocks.usb_freq()
    );

    // ── GPIO ────────────────────────────────────────────────────────────────
    let pins = Pins::init(dp.IO_BANK0, dp.PADS_BANK0);

    // On-board LED — blinks when audio data flows.
    let mut led = pins.pin::<LED_PIN>();
    led.into_output();
    led.set_low();

    // ── USB Audio setup ─────────────────────────────────────────────────────
    let alloc = UsbBusAllocator::new(unsafe { Rp2350Bus::steal() });

    // Configure a stereo speaker output at 48 kHz, 16-bit PCM.
    // This creates:
    //   - 1 Audio Control interface (no endpoints)
    //   - 1 Audio Streaming interface (1 isochronous OUT endpoint)
    // Total: 2 interfaces (linked by an IAD), 1 isochronous endpoint.
    let speaker_cfg = StreamConfig::new_discrete(
        Format::S16le, // 16-bit signed PCM
        2,             // stereo
        &RATES_48K,    // 48 kHz only
        TerminalType::OutSpeaker,
    )
    .unwrap();

    let mut audio = AudioClassBuilder::new().output(speaker_cfg).build(&alloc);

    let mut usb_dev = UsbDeviceBuilder::new(&alloc, 0x1209, 0x1561)
        .device_release(0x0100)
        .manufacturer("Raspberry Pi")
        .product("Pico USB Speaker")
        .serial_number("UAC001")
        .build();

    // Connect D+ pull-up: signals USB presence to the host.
    usb_dev.bus().connect();
    info!("USB connected, waiting for enumeration...");

    // ── Main loop ───────────────────────────────────────────────────────────
    let systick = SysTick::init(cp.SYST, SYS_CLK_HZ);
    let mut audio_buf = [0u8; 192]; // 48 kHz × 2 ch × 2 bytes = 192 bytes/ms

    // Statistics
    let mut total_bytes: u64 = 0;
    let mut last_report_ms: u32 = 0;
    let mut uptime_ms: u32 = 0;
    let mut led_counter: u32 = 0;

    loop {
        // Poll USB at ≤ 1 ms intervals.
        usb_dev.poll(&mut [&mut audio as &mut dyn UsbClass<Rp2350Bus>]);

        // ── Read audio data from the host ────────────────────────────────────
        // When the host is streaming audio, read() returns the number of bytes
        // in the isochronous packet.  When not streaming (alt setting 0),
        // it returns InvalidState.
        match audio.read(&mut audio_buf) {
            Ok(n) if n > 0 => {
                total_bytes += n as u64;

                // Toggle LED every ~64 KB to show activity.
                led_counter = led_counter.wrapping_add(1);
                if led_counter.is_multiple_of(64) {
                    led.toggle();
                }

                // In a real application, you would send audio_buf[..n] to a DAC,
                // I2S codec, or PWM output here.
            }
            Ok(_) => {
                // Zero-length packet — host sent silence.
            }
            Err(_) => {
                // WouldBlock (no data) or InvalidState (not streaming).
                // This is normal between USB frames.
            }
        }

        // ── Periodic statistics ─────────────────────────────────────────────
        uptime_ms = uptime_ms.wrapping_add(1);
        let elapsed = uptime_ms.wrapping_sub(last_report_ms);
        if elapsed >= RATE_WINDOW_MS {
            let kbps = total_bytes as u32 * 8 / elapsed;
            info!(
                "Audio: {} total bytes, ~{} kbps, output_active={}",
                total_bytes,
                kbps,
                audio.output_active()
            );
            total_bytes = 0;
            last_report_ms = uptime_ms;
        }

        // Yield ~1 ms to match the USB frame interval.
        systick.delay_us(1000);
    }
}
