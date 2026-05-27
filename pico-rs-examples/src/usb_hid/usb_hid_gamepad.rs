//! USB HID Gamepad Example
//!
//! This example demonstrates how to create a USB gamepad device.
//! The gamepad has 16 buttons, a D-pad (hat switch), and two analog sticks.

#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;

use pico_rs::Pico;
use pico_rs_usb::{
    class::hid::{
        gamepad_button, hat_direction, GamepadReport, HidClass, HidConfig,
    },
    device::UsbDevice,
};

#[cortex_m_rt::entry]
fn main() -> ! {
    info!("USB HID Gamepad example starting...");

    let pico = Pico::init();
    info!("Pico initialized");

    // Get USB bus
    let usb_bus = pico.usb_bus();

    // Create a gamepad HID device
    let mut hid = HidClass::gamepad(&usb_bus);

    // Create USB device with gamepad
    let mut usb_dev = UsbDevice::builder(&usb_bus)
        .manufacturer("Pico-RS")
        .product("USB Gamepad")
        .serial_number("GP001")
        .add_class(&mut hid)
        .build();

    info!("USB Gamepad ready");

    // Animation state
    let mut counter: u32 = 0;
    let mut last_tick = 0u32;
    let tick_interval = 50_000; // ~50ms at ~1MHz

    loop {
        // Poll USB
        usb_dev.poll();

        // Simple timing based on loop iterations
        counter = counter.wrapping_add(1);
        if counter.wrapping_sub(last_tick) >= tick_interval {
            last_tick = counter;

            if hid.is_configured() {
                // Create a gamepad report that cycles through different states
                let phase = (counter / tick_interval) as u8;

                let mut report = GamepadReport::new();

                // Cycle through buttons
                match phase % 16 {
                    0 => report = report.with_button(gamepad_button::A),
                    1 => report = report.with_button(gamepad_button::B),
                    2 => report = report.with_button(gamepad_button::X),
                    3 => report = report.with_button(gamepad_button::Y),
                    4 => report = report.with_button(gamepad_button::LEFT_BUMPER),
                    5 => report = report.with_button(gamepad_button::RIGHT_BUMPER),
                    6 => report = report.with_button(gamepad_button::LEFT_TRIGGER),
                    7 => report = report.with_button(gamepad_button::RIGHT_TRIGGER),
                    8 => report = report.with_button(gamepad_button::BACK),
                    9 => report = report.with_button(gamepad_button::START),
                    10 => report = report.with_button(gamepad_button::LEFT_STICK),
                    11 => report = report.with_button(gamepad_button::RIGHT_STICK),
                    12 => report = report.with_button(gamepad_button::HOME),
                    _ => {}
                }

                // Cycle D-pad direction
                let hat = match (phase / 16) % 9 {
                    0 => hat_direction::UP,
                    1 => hat_direction::UP_RIGHT,
                    2 => hat_direction::RIGHT,
                    3 => hat_direction::DOWN_RIGHT,
                    4 => hat_direction::DOWN,
                    5 => hat_direction::DOWN_LEFT,
                    6 => hat_direction::LEFT,
                    7 => hat_direction::UP_LEFT,
                    _ => hat_direction::NEUTRAL,
                };
                report = report.with_hat(hat);

                // Animate left stick in a circle
                let angle = (phase as f32) * 0.1;
                let lx = (128.0 + 100.0 * libm::cosf(angle)) as u8;
                let ly = (128.0 + 100.0 * libm::sinf(angle)) as u8;
                report = report.with_left_stick(lx, ly);

                // Keep right stick centered
                report = report.with_right_stick(128, 128);

                match hid.send_gamepad_report(&report) {
                    Ok(_) => {}
                    Err(_) => {} // Ignore errors (might be busy)
                }
            }
        }
    }
}
