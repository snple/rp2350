//! USB HID Touchpad Example
//!
//! This example demonstrates how to create a USB touchpad device
//! with absolute positioning (like a touchscreen or digitizer tablet).

#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;

use roschi::Pico;
use roschi_usb::{
    class::hid::{HidClass, TouchpadReport, touchpad_button},
    device::UsbDevice,
};

#[cortex_m_rt::entry]
fn main() -> ! {
    info!("USB HID Touchpad example starting...");

    let pico = Pico::init();
    info!("Pico initialized");

    // Get USB bus
    let usb_bus = pico.usb_bus();

    // Create a touchpad HID device
    let mut hid = HidClass::touchpad(&usb_bus);

    // Create USB device
    let mut usb_dev = UsbDevice::builder(&usb_bus)
        .manufacturer("Roschi")
        .product("USB Touchpad")
        .serial_number("TP001")
        .add_class(&mut hid)
        .build();

    info!("USB Touchpad ready");

    // Animation state
    let mut counter: u32 = 0;
    let mut last_tick = 0u32;
    let tick_interval = 20_000; // ~20ms for smooth movement

    // Position (start at center)
    let mut x: u16 = 16384; // Half of 32767
    let mut y: u16 = 16384;
    let mut angle: f32 = 0.0;

    loop {
        // Poll USB
        usb_dev.poll();

        counter = counter.wrapping_add(1);
        if counter.wrapping_sub(last_tick) >= tick_interval {
            last_tick = counter;

            if hid.is_configured() {
                // Move cursor in a circle pattern
                angle += 0.05;
                if angle > 6.28318 {
                    angle = 0.0;
                }

                // Calculate position in a circle (radius = 8000 units)
                let radius: f32 = 8000.0;
                let center_x: f32 = 16384.0;
                let center_y: f32 = 16384.0;

                x = (center_x + radius * libm::cosf(angle)) as u16;
                y = (center_y + radius * libm::sinf(angle)) as u16;

                // Determine if we should "click" (every full rotation)
                let phase = (angle * 10.0) as u8;
                let buttons = if phase == 0 { touchpad_button::TAP } else { 0 };

                let report = TouchpadReport::new()
                    .with_buttons(buttons)
                    .with_position(x, y);

                match hid.send_touchpad_report(&report) {
                    Ok(_) => {}
                    Err(_) => {} // Ignore errors
                }
            }
        }
    }
}
