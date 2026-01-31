//! USB HID Consumer Control (Multimedia Keys) Example
//!
//! This example demonstrates how to create a USB consumer control device
//! for multimedia keys like volume, play/pause, etc.

#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

use pico_rs::Pico;
use pico_rs_usb::{
    class::hid::{consumer_usage, ConsumerReport, HidClass},
    device::UsbDevice,
};

#[cortex_m_rt::entry]
fn main() -> ! {
    info!("USB HID Consumer Control example starting...");

    let pico = Pico::init();
    info!("Pico initialized");

    // Get USB bus
    let usb_bus = pico.usb_bus();

    // Create a consumer control HID device
    let mut hid = HidClass::consumer_control(&usb_bus);

    // Create USB device
    let mut usb_dev = UsbDevice::builder(&usb_bus)
        .manufacturer("Pico-RS")
        .product("USB Media Controller")
        .serial_number("MC001")
        .add_class(&mut hid)
        .build();

    info!("USB Consumer Control ready");

    // State
    let mut counter: u32 = 0;
    let mut last_tick = 0u32;
    let tick_interval = 2_000_000; // ~2 seconds

    // List of media keys to cycle through
    let media_keys = [
        (consumer_usage::PLAY_PAUSE, "Play/Pause"),
        (consumer_usage::NEXT_TRACK, "Next Track"),
        (consumer_usage::PREV_TRACK, "Previous Track"),
        (consumer_usage::VOLUME_UP, "Volume Up"),
        (consumer_usage::VOLUME_DOWN, "Volume Down"),
        (consumer_usage::MUTE, "Mute"),
    ];
    let mut key_index = 0usize;

    loop {
        // Poll USB
        usb_dev.poll();

        counter = counter.wrapping_add(1);
        if counter.wrapping_sub(last_tick) >= tick_interval {
            last_tick = counter;

            if hid.is_configured() {
                // Get current media key
                let (usage_id, name) = media_keys[key_index % media_keys.len()];
                info!("Sending media key: {}", name);

                // Send key press
                let report = ConsumerReport::new().with_usage(usage_id);
                let _ = hid.send_consumer_report(&report);

                // Small delay simulation
                for _ in 0..10000 {
                    usb_dev.poll();
                }

                // Send key release (usage_id = 0)
                let release = ConsumerReport::new();
                let _ = hid.send_consumer_report(&release);

                key_index += 1;
            }
        }
    }
}
