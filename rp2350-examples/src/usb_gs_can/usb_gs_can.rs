//! # USB GS_USB (CAN Adapter) Example
//!
//! Demonstrates the GS_USB / candleLight protocol over USB.
//! The Pico 2 will appear as a candleLight-compatible USB-CAN adapter.
//!
//! ## Hardware
//! - Raspberry Pi Pico 2 (RP2350)
//! - USB cable connected to the Pico's USB port
//! - LED on GPIO 25 (on-board)
//!
//! ## Testing on Linux
//!
//! ```bash
//! # Flash the firmware, then connect USB.  Verify with:
//! sudo dmesg | tail -20
//! # Expected: "gs_usb 1-1:1.0: Configuring for interface 0"
//!
//! sudo ip link set can0 type can bitrate 500000
//! sudo ip link set up can0
//!
//! # Send a CAN frame:
//! cansend can0 123#DEADBEEF
//!
//! # Receive frames (the device echoes back what it receives in this demo):
//! candump can0
//! ```
//!
//! ## CAN hardware integration
//!
//! This demo echoes frames back (USB loopback).  To connect real CAN hardware
//! (e.g. TCAN4550 or built-in CAN on RP2350A/B), replace the stub
//! `can_send_frame` / `can_recv_frame` functions with your CAN driver calls.
//!
#![no_std]
#![no_main]

#[cfg(feature = "defmt")]
use defmt_rtt as _;
use rp2350_examples::log::*;

use rp2350::clocks::ClockBuilder;
use rp2350::gpio::Pins;
use rp2350::main;
use rp2350::pac::Peripherals;

use rp2350_usb::UsbBusAllocator;
use rp2350_usb::class::gs_usb::{GsHostFrame, GsUsbClass, gs_can_mode};
use rp2350_usb::device::UsbDeviceBuilder;
use rp2350_usb::rp2350::Rp2350Bus;

// ─── Hardware stubs ─────────────────────────────────────────────────────────

/// Placeholder: initialise your CAN controller here.
fn can_init() {
    info!("CAN controller initialised (stub)");
}

/// Placeholder: set CAN bit timing.
#[allow(dead_code)]
fn can_set_bit_timing(_brp: u32, _tseg1: u32, _tseg2: u32, _sjw: u32) {
    info!(
        "CAN bit timing: brp={}, tseg1={}, tseg2={}, sjw={}",
        _brp, _tseg1, _tseg2, _sjw
    );
}

/// Placeholder: put CAN controller into the given mode.
fn can_set_mode(mode: u32) {
    match mode {
        gs_can_mode::RESET => info!("CAN mode: RESET"),
        gs_can_mode::START => info!("CAN mode: START"),
        _ => warn!("CAN mode: UNKNOWN ({})", mode),
    }
}

/// Placeholder: send a CAN frame to the bus.
/// Return `true` if the frame was accepted by the controller.
fn can_send_frame(_frame: &GsHostFrame) -> bool {
    // Copy fields from packed struct to avoid unaligned references.
    let id = _frame.can_id;
    let dlc = _frame.can_dlc;
    let flags = _frame.flags;
    let data = _frame.data;
    info!(
        "CAN TX: id={:#X}, dlc={}, flags={:#X}, data={:?}",
        id,
        dlc,
        flags,
        &data[..dlc as usize]
    );
    // Stub: always "succeeds"
    true
}

/// Placeholder: poll the CAN controller for received frames.
/// Return `Some(frame)` if a frame was received.
fn can_poll_rx() -> Option<GsHostFrame> {
    // Stub: no frames from real CAN bus in this demo.
    None
}

// ─── Main ────────────────────────────────────────────────────────────────────

#[main]
fn main() -> ! {
    info!("USB GS_USB CAN Adapter Example starting...");

    // Take peripherals
    let dp = Peripherals::take().unwrap();

    // Clocks: 150 MHz sys, 48 MHz USB
    let clocks = ClockBuilder::new()
        .enable_usb()
        .build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);
    info!(
        "Clocks: sys={} Hz, usb={} Hz",
        clocks.sys_freq(),
        clocks.usb_freq()
    );

    // GPIO
    let pins = Pins::init(dp.IO_BANK0, dp.PADS_BANK0);

    // On-board LED
    let mut led = pins.pin::<25>();
    led.into_output();
    led.set_low();

    // ── USB setup ──────────────────────────────────────────────────────────

    let alloc = UsbBusAllocator::new(unsafe { Rp2350Bus::steal() });

    let mut gs_usb = GsUsbClass::new(&alloc);

    // NOTE: change to candleLight VID/PID (0x1D50, 0x606F) after verifying
    // enumeration works.  The Linux gs_usb driver binds to that VID/PID and
    // its probe may interfere if the firmware doesn't handle all requests.
    let mut usb_dev = UsbDeviceBuilder::new(&alloc, 0x1234, 0x5678)
        .device_release(0x0100)
        .manufacturer("candleLight")
        .product("candleLight RP2350")
        .serial_number("CAN001")
        .build();

    usb_dev.bus().connect();
    info!("USB connected, waiting for enumeration...");

    // ── CAN init ────────────────────────────────────────────────────────────

    can_init();

    #[allow(unused_mut)]
    let mut delay = rp2350_examples::common::board::init_delay(&clocks);
    let mut configured = false;
    let mut echo_id: u32 = 0;
    let mut led_toggle_count: u32 = 0;

    loop {
        // ── USB poll ───────────────────────────────────────────────────────

        usb_dev.poll(&mut [&mut gs_usb]);

        // ── Configuration state tracking ───────────────────────────────────

        if gs_usb.is_configured() && !configured {
            configured = true;
            led.set_high();
            info!("GS_USB configured by host");
        }
        if !gs_usb.is_configured() && configured {
            configured = false;
            led.set_low();
            info!("GS_USB deconfigured");
        }

        if configured {
            // ── Check for mode changes ────────────────────────────────────

            let mode = gs_usb.mode();
            // Static mode tracking would require storing previous mode,
            // but for this demo we just act on it each iteration.
            match mode {
                gs_can_mode::START => {
                    // Apply bit timing if we haven't already.
                    can_set_mode(gs_can_mode::START);
                }
                gs_can_mode::RESET => {
                    can_set_mode(gs_can_mode::RESET);
                }
                _ => {}
            }

            // ── USB → CAN: read frames from host ──────────────────────────

            match gs_usb.try_recv() {
                Ok(Some(frame)) => {
                    // Copy fields from packed struct to avoid unaligned references.
                    let can_id = frame.can_id;
                    let can_dlc = frame.can_dlc;
                    let flags = frame.flags;
                    let data = frame.data;
                    info!(
                        "USB→CAN: id={:#X}, dlc={}, flags={:#X}",
                        can_id, can_dlc, flags
                    );

                    // Echo the frame back as a TX-complete confirmation.
                    let response = GsHostFrame {
                        echo_id,
                        can_id,
                        can_dlc,
                        channel: 0,
                        flags,
                        reserved: 0,
                        data,
                    };
                    echo_id = echo_id.wrapping_add(1);

                    // Forward to CAN bus
                    if can_send_frame(&frame) {
                        // TX complete: send echo back to host.
                        if let Err(e) = gs_usb.try_send(&response) {
                            warn!("USB TX echo failed: {:?}", e);
                        }
                    }
                    led_toggle_count += 1;
                }
                Ok(None) => { /* no frame waiting */ }
                Err(e) => {
                    warn!("USB recv error: {:?}", e);
                }
            }

            // ── CAN → USB: poll CAN controller for received frames ────────

            if let Some(frame) = can_poll_rx() {
                let can_id = frame.can_id;
                let can_dlc = frame.can_dlc;
                info!("CAN→USB: id={:#X}, dlc={}", can_id, can_dlc);
                if let Err(e) = gs_usb.try_send(&frame) {
                    warn!("USB TX error: {:?}", e);
                }
                led_toggle_count += 1;
            }

            // Blink LED on activity (every 4 frames).
            if led_toggle_count >= 4 {
                led.toggle();
                led_toggle_count = 0;
            }
        } else {
            // Not configured: slow blink to show we're waiting.
            delay.delay_ms(100);
            led.toggle();
            delay.delay_ms(100);
            led.toggle();
        }

        delay.delay_us(100);
    }
}
