//! # USB HID Keyboard Example
//!
//! This example demonstrates USB HID keyboard functionality.
//! The Pico 2 will appear as a USB keyboard and type "Hello!" when a button is pressed.
//!
//! ## Hardware
//! - Raspberry Pi Pico 2 (RP2350)
//! - USB cable connected to the Pico's USB port
//! - Button connected to GPIO 15 (with internal pull-up)
//!
//! ## Testing
//! 1. Flash this example to the Pico 2
//! 2. Connect USB cable to your computer
//! 3. Open a text editor
//! 4. Press the button on GPIO 15 - it will type "Hello!"
//! 5. The LED (GPIO 25) indicates HID activity

#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

use pico_rs::Delay;
use pico_rs::block::ImageDef;
use pico_rs::clocks::ClockBuilder;
use pico_rs::entry;
use pico_rs::gpio::{self, Pin15, Pin25};

use pico_rs_usb::bus::UsbBus;
use pico_rs_usb::class::hid::{HidClass, KeyboardReport, keycode, modifier};
use pico_rs_usb::device::UsbDeviceBuilder;

/// RP2350 Boot Block
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::exe();

/// Type a string by sending individual key reports
fn type_string(hid: &mut HidClass, text: &str, delay: &mut Delay) {
    for ch in text.chars() {
        let (keycode, shift) = char_to_keycode(ch);
        if keycode != 0 {
            // Key down
            let mut report = KeyboardReport::new().with_key(keycode);
            if shift {
                report = report.with_modifier(modifier::LEFT_SHIFT);
            }
            let _ = hid.send_keyboard_report(&report);
            delay.delay_ms(10);

            // Key up
            let _ = hid.send_keyboard_report(&KeyboardReport::new());
            delay.delay_ms(10);
        }
    }
}

/// Convert ASCII character to USB HID keycode
fn char_to_keycode(ch: char) -> (u8, bool) {
    match ch {
        'a'..='z' => ((ch as u8 - b'a' + keycode::A), false),
        'A'..='Z' => ((ch as u8 - b'A' + keycode::A), true),
        '1'..='9' => ((ch as u8 - b'1' + keycode::NUM_1), false),
        '0' => (keycode::NUM_0, false),
        ' ' => (keycode::SPACE, false),
        '\n' => (keycode::ENTER, false),
        '\t' => (keycode::TAB, false),
        '!' => (keycode::NUM_1, true),
        _ => (0, false),
    }
}

#[entry]
fn main() -> ! {
    info!("USB HID Keyboard Example starting...");

    // Initialize clocks using the convenient API (150 MHz sys, 48 MHz USB)
    let clocks = ClockBuilder::new().enable_usb().build();
    info!(
        "Clock system initialized: sys={} Hz, usb={} Hz",
        clocks.sys_freq(),
        clocks.usb_freq()
    );

    // Initialize GPIO
    gpio::init_gpio();

    // LED on GPIO 25
    let led = Pin25::new();
    led.into_output();
    led.set_low();

    // Button on GPIO 15 with pull-up
    let button = Pin15::new();
    button.into_input_pullup();

    // Initialize USB bus
    let usb_bus = UsbBus::new();
    let _ = usb_bus.init();
    info!("USB bus initialized");

    // Create HID keyboard class
    let mut hid = HidClass::keyboard(&usb_bus);
    info!("HID keyboard class created");

    // Build USB device
    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus)
        .vendor_id(0x2E8A) // Raspberry Pi
        .product_id(0x000B) // Custom HID
        .device_release(0x0100)
        .manufacturer("Raspberry Pi")
        .product("Pico HID Keyboard")
        .serial_number("HID001")
        .build();

    info!("USB device built, connecting...");

    // Connect to host
    usb_bus.connect();
    info!("USB connected, waiting for enumeration...");

    let delay = Delay::new(clocks.sys_freq());
    let mut button_prev = true; // Pull-up, so high when not pressed
    let mut configured = false;

    loop {
        // Poll USB
        usb_dev.poll(&mut [&mut hid]);

        // Check if configured
        if hid.is_configured() && !configured {
            configured = true;
            led.set_high();
            info!("HID keyboard configured and ready!");
        }

        if configured {
            // Read button state (low = pressed due to pull-up)
            let button_now = button.read();

            // Detect falling edge (button press)
            if button_prev && !button_now {
                info!("Button pressed, typing 'Hello!'");
                led.toggle();

                // Type "Hello!"
                type_string(&mut hid, "Hello!", &mut delay);

                led.toggle();
            }

            button_prev = button_now;
        }

        // Small delay
        delay.delay_us(100);
    }
}
