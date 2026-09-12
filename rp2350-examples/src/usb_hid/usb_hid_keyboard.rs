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

use rp2350::Delay;
use rp2350::clocks::ClockBuilder;
use rp2350::gpio::Pins;
use rp2350::main;
use rp2350::pac::Peripherals;

use rp2350_usb::UsbBusAllocator;
use rp2350_usb::class::hid::HidClass;
use rp2350_usb::class::hid::keyboard::{Keyboard, KeyboardReport, modifier};
use rp2350_usb::device::UsbDeviceBuilder;
use rp2350_usb::rp2350::Rp2350Bus;

mod keycode {
    pub const A: u8 = 0x04;
    pub const NUM_1: u8 = 0x1E;
    pub const NUM_0: u8 = 0x27;
    pub const ENTER: u8 = 0x28;
    pub const TAB: u8 = 0x2B;
    pub const SPACE: u8 = 0x2C;
}

/// Send one keyboard report, dropping it if the host is not ready yet.
fn send(hid: &HidClass<'_, Rp2350Bus, Keyboard>, report: &KeyboardReport) {
    let _ = hid.send_input(&report.to_bytes());
}

/// Type a string by sending individual key reports
fn type_string(hid: &HidClass<'_, Rp2350Bus, Keyboard>, text: &str, delay: &mut Delay) {
    for ch in text.chars() {
        let (kc, shift) = char_to_keycode(ch);
        if kc != 0 {
            // Key down
            let mut report = KeyboardReport::default();
            report.keys[0] = kc;
            if shift {
                report.modifiers = modifier::LEFT_SHIFT;
            }
            send(hid, &report);
            delay.delay_ms(10);

            // Key up
            send(hid, &KeyboardReport::default());
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

#[main]
fn main() -> ! {
    info!("USB HID Keyboard Example starting...");

    // Take peripherals
    let dp = Peripherals::take().unwrap();

    // Initialize clocks using the convenient API (150 MHz sys, 48 MHz USB)
    let clocks = ClockBuilder::new()
        .enable_usb()
        .build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);
    info!(
        "Clock system initialized: sys={} Hz, usb={} Hz",
        clocks.sys_freq(),
        clocks.usb_freq()
    );

    // Initialize GPIO
    let pins = Pins::init(dp.IO_BANK0, dp.PADS_BANK0);

    // LED on GPIO 25
    let mut led = pins.pin::<25>();
    led.into_output();
    led.set_low();

    // Button on GPIO 15 with pull-up
    let mut button = pins.pin::<15>();
    button.into_input_pullup();

    // Initialize USB bus allocator
    let alloc = UsbBusAllocator::new(unsafe { Rp2350Bus::steal() });
    info!("USB bus allocator created");

    // Create HID keyboard class
    let mut hid = HidClass::new(&alloc, Keyboard::new());
    info!("HID keyboard class created");

    // Build USB device (also freezes the allocator and enables the peripheral)
    let mut usb_dev = UsbDeviceBuilder::new(&alloc, 0x2E8A, 0x000B)
        .device_release(0x0100)
        .manufacturer("Raspberry Pi")
        .product("Pico HID Keyboard")
        .serial_number("HID001")
        .build();

    info!("USB device built, connecting...");

    // Connect to host (enable D+ pull-up)
    usb_dev.bus().connect();
    info!("USB connected, waiting for enumeration...");

    let mut delay = rp2350_examples::common::board::init_delay(&clocks);
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
            // Get button state (low = pressed due to pull-up)
            let button_now = button.get();

            // Detect falling edge (button press)
            if button_prev && !button_now {
                info!("Button pressed, typing 'Hello!'");
                led.toggle();

                // Type "Hello!"
                type_string(&hid, "Hello!", &mut delay);

                led.toggle();
            }

            button_prev = button_now;
        }

        // Small delay
        delay.delay_us(100);
    }
}
