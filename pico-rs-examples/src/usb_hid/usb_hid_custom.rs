//! USB HID Custom Device Example
//!
//! This example demonstrates how to create a fully custom HID device
//! with a custom report descriptor for advanced use cases like:
//! - Custom mechanical keyboards with extra features
//! - Macro pads with special keys
//! - Custom input devices with multiple interfaces

#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;

use pico_rs::Pico;
use pico_rs_usb::{
    class::hid::{HidClass, HidConfig, hid_protocol, hid_subclass},
    device::UsbDevice,
};

/// Custom 4-button macro pad report descriptor
/// Format: buttons (8 bits) + rotary encoder (signed 8-bit)
static MACRO_PAD_REPORT_DESCRIPTOR: &[u8] = &[
    0x06, 0x00, 0xFF, // Usage Page (Vendor Defined 0xFF00)
    0x09, 0x01, // Usage (Vendor Usage 1)
    0xA1, 0x01, // Collection (Application)
    // 8 buttons
    0x05, 0x09, //   Usage Page (Buttons)
    0x19, 0x01, //   Usage Minimum (Button 1)
    0x29, 0x08, //   Usage Maximum (Button 8)
    0x15, 0x00, //   Logical Minimum (0)
    0x25, 0x01, //   Logical Maximum (1)
    0x75, 0x01, //   Report Size (1)
    0x95, 0x08, //   Report Count (8)
    0x81, 0x02, //   Input (Data, Variable, Absolute)
    // Rotary encoder (relative value)
    0x05, 0x01, //   Usage Page (Generic Desktop)
    0x09, 0x37, //   Usage (Dial)
    0x15, 0x81, //   Logical Minimum (-127)
    0x25, 0x7F, //   Logical Maximum (127)
    0x75, 0x08, //   Report Size (8)
    0x95, 0x01, //   Report Count (1)
    0x81, 0x06, //   Input (Data, Variable, Relative)
    // LED output (for button backlight control)
    0x05, 0x08, //   Usage Page (LEDs)
    0x19, 0x01, //   Usage Minimum (Num Lock)
    0x29, 0x08, //   Usage Maximum (8)
    0x15, 0x00, //   Logical Minimum (0)
    0x25, 0x01, //   Logical Maximum (1)
    0x75, 0x01, //   Report Size (1)
    0x95, 0x08, //   Report Count (8)
    0x91, 0x02, //   Output (Data, Variable, Absolute)
    0xC0, // End Collection
];

/// Custom macro pad report (2 bytes input)
#[derive(Clone, Copy, Debug, Default)]
struct MacroPadReport {
    /// Button states (8 buttons)
    buttons: u8,
    /// Rotary encoder delta (-127 to 127)
    encoder: i8,
}

impl MacroPadReport {
    const fn new() -> Self {
        Self {
            buttons: 0,
            encoder: 0,
        }
    }

    const fn with_buttons(mut self, buttons: u8) -> Self {
        self.buttons = buttons;
        self
    }

    const fn with_encoder(mut self, delta: i8) -> Self {
        self.encoder = delta;
        self
    }

    fn to_bytes(&self) -> [u8; 2] {
        [self.buttons, self.encoder as u8]
    }
}

/// Button masks for macro pad
mod macro_button {
    pub const BUTTON_1: u8 = 1 << 0;
    pub const BUTTON_2: u8 = 1 << 1;
    pub const BUTTON_3: u8 = 1 << 2;
    pub const BUTTON_4: u8 = 1 << 3;
    pub const BUTTON_5: u8 = 1 << 4;
    pub const BUTTON_6: u8 = 1 << 5;
    pub const BUTTON_7: u8 = 1 << 6;
    pub const BUTTON_8: u8 = 1 << 7;
}

#[cortex_m_rt::entry]
fn main() -> ! {
    info!("USB HID Custom Device example starting...");

    let pico-rs = Pico::init();
    info!("Pico initialized");

    // Get USB bus
    let usb_bus = pico-rs.usb_bus();

    // Create custom HID configuration
    let config = HidConfig::new(MACRO_PAD_REPORT_DESCRIPTOR)
        .in_report_size(2) // 2 bytes: buttons + encoder
        .out_report_size(1) // 1 byte: LED states
        .poll_interval_ms(1) // 1ms polling for low latency
        .subclass(hid_subclass::NONE)
        .protocol(hid_protocol::NONE)
        .hid_version(0x0111) // HID 1.11
        .country_code(0); // Not localized

    // Create HID class with custom config
    let mut hid = HidClass::new(&usb_bus, config);

    // Create USB device
    let mut usb_dev = UsbDevice::builder(&usb_bus)
        .manufacturer("CustomHID")
        .product("8-Key Macro Pad")
        .serial_number("MACRO001")
        .add_class(&mut hid)
        .build();

    info!("USB Custom HID device ready");

    // State
    let mut counter: u32 = 0;
    let mut last_tick = 0u32;
    let tick_interval = 100_000; // ~100ms between state changes

    let mut button_index = 0u8;
    let mut encoder_direction = 1i8;
    let mut encoder_value = 0i8;

    loop {
        // Poll USB
        usb_dev.poll();

        // Check for output reports (LED control from host)
        if hid.has_output_report() {
            let mut led_report = [0u8; 1];
            if let Some(len) = hid.read_output_report(&mut led_report) {
                info!("Received LED report: {:02X} ({} bytes)", led_report[0], len);
                // Here you would control actual LEDs based on the report
            }
        }

        counter = counter.wrapping_add(1);
        if counter.wrapping_sub(last_tick) >= tick_interval {
            last_tick = counter;

            if hid.is_configured() {
                // Cycle through buttons
                let buttons = 1u8 << (button_index % 8);
                button_index = button_index.wrapping_add(1);

                // Simulate rotary encoder rotation
                encoder_value = encoder_value.wrapping_add(encoder_direction);
                if encoder_value > 10 || encoder_value < -10 {
                    encoder_direction = -encoder_direction;
                }

                let report = MacroPadReport::new()
                    .with_buttons(buttons)
                    .with_encoder(encoder_direction * 2); // Delta of ±2

                let _ = hid.send_report(&report.to_bytes());

                info!(
                    "Sent: buttons={:02X}, encoder={}",
                    buttons,
                    encoder_direction * 2
                );
            }
        }
    }
}
