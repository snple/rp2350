//! HID (Human Interface Device) class implementation
//!
//! This module provides USB HID class support for keyboards, mice, and custom devices.
//!
//! ## Example
//!
//! ```ignore
//! use pico_rs_usb::{UsbDevice, UsbBus};
//! use pico_rs_usb::class::hid::{HidClass, HidReportType};
//!
//! let usb_bus = UsbBus::new();
//!
//! // Create a keyboard HID device
//! let mut keyboard = HidClass::keyboard(&usb_bus);
//!
//! // Or create a mouse
//! let mut mouse = HidClass::mouse(&usb_bus);
//!
//! // Poll and send reports
//! loop {
//!     usb_dev.poll(&mut [&mut keyboard]);
//!
//!     // Send keyboard report
//!     let report = KeyboardReport::new().key(KeyCode::A);
//!     keyboard.send_report(&report.to_bytes())?;
//! }
//! ```

use crate::bus::UsbBus;
use crate::control::SetupPacket;
use crate::descriptor::{DescriptorWriter, EndpointDescriptor, InterfaceDescriptor, class_code};
use crate::device::UsbClass;
use crate::endpoint::{EndpointAddress, EndpointType};
use crate::error::{UsbError, UsbResult};
use crate::types::{Recipient, RequestType};

/// HID class code
pub const HID_CLASS: u8 = class_code::HID;

/// HID subclass codes
pub mod hid_subclass {
    /// No subclass
    pub const NONE: u8 = 0x00;
    /// Boot interface subclass
    pub const BOOT: u8 = 0x01;
}

/// HID protocol codes
pub mod hid_protocol {
    /// No specific protocol
    pub const NONE: u8 = 0x00;
    /// Keyboard protocol
    pub const KEYBOARD: u8 = 0x01;
    /// Mouse protocol
    pub const MOUSE: u8 = 0x02;
}

/// HID descriptor type codes
pub mod hid_descriptor_type {
    /// HID descriptor
    pub const HID: u8 = 0x21;
    /// Report descriptor
    pub const REPORT: u8 = 0x22;
    /// Physical descriptor
    pub const PHYSICAL: u8 = 0x23;
}

/// HID class request codes
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum HidRequest {
    /// Get report
    GetReport = 0x01,
    /// Get idle rate
    GetIdle = 0x02,
    /// Get protocol
    GetProtocol = 0x03,
    /// Set report
    SetReport = 0x09,
    /// Set idle rate
    SetIdle = 0x0A,
    /// Set protocol
    SetProtocol = 0x0B,
}

impl TryFrom<u8> for HidRequest {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(HidRequest::GetReport),
            0x02 => Ok(HidRequest::GetIdle),
            0x03 => Ok(HidRequest::GetProtocol),
            0x09 => Ok(HidRequest::SetReport),
            0x0A => Ok(HidRequest::SetIdle),
            0x0B => Ok(HidRequest::SetProtocol),
            v => Err(v),
        }
    }
}

/// HID report types
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum HidReportType {
    /// Input report (device to host)
    Input = 1,
    /// Output report (host to device)
    Output = 2,
    /// Feature report (bidirectional)
    Feature = 3,
}

/// Standard keyboard report descriptor
pub const KEYBOARD_REPORT_DESCRIPTOR: &[u8] = &[
    0x05, 0x01, // Usage Page (Generic Desktop)
    0x09, 0x06, // Usage (Keyboard)
    0xA1, 0x01, // Collection (Application)
    // Modifier keys
    0x05, 0x07, //   Usage Page (Keyboard/Keypad)
    0x19, 0xE0, //   Usage Minimum (Left Control)
    0x29, 0xE7, //   Usage Maximum (Right GUI)
    0x15, 0x00, //   Logical Minimum (0)
    0x25, 0x01, //   Logical Maximum (1)
    0x75, 0x01, //   Report Size (1)
    0x95, 0x08, //   Report Count (8)
    0x81, 0x02, //   Input (Data, Variable, Absolute)
    // Reserved byte
    0x75, 0x08, //   Report Size (8)
    0x95, 0x01, //   Report Count (1)
    0x81, 0x01, //   Input (Constant)
    // LEDs
    0x05, 0x08, //   Usage Page (LEDs)
    0x19, 0x01, //   Usage Minimum (Num Lock)
    0x29, 0x05, //   Usage Maximum (Kana)
    0x75, 0x01, //   Report Size (1)
    0x95, 0x05, //   Report Count (5)
    0x91, 0x02, //   Output (Data, Variable, Absolute)
    0x75, 0x03, //   Report Size (3)
    0x95, 0x01, //   Report Count (1)
    0x91, 0x01, //   Output (Constant)
    // Key codes
    0x05, 0x07, //   Usage Page (Keyboard/Keypad)
    0x19, 0x00, //   Usage Minimum (0)
    0x29, 0xFF, //   Usage Maximum (255)
    0x15, 0x00, //   Logical Minimum (0)
    0x26, 0xFF, 0x00, // Logical Maximum (255)
    0x75, 0x08, //   Report Size (8)
    0x95, 0x06, //   Report Count (6)
    0x81, 0x00, //   Input (Data, Array)
    0xC0, // End Collection
];

/// Standard mouse report descriptor
pub const MOUSE_REPORT_DESCRIPTOR: &[u8] = &[
    0x05, 0x01, // Usage Page (Generic Desktop)
    0x09, 0x02, // Usage (Mouse)
    0xA1, 0x01, // Collection (Application)
    0x09, 0x01, //   Usage (Pointer)
    0xA1, 0x00, //   Collection (Physical)
    // Buttons
    0x05, 0x09, //     Usage Page (Buttons)
    0x19, 0x01, //     Usage Minimum (Button 1)
    0x29, 0x03, //     Usage Maximum (Button 3)
    0x15, 0x00, //     Logical Minimum (0)
    0x25, 0x01, //     Logical Maximum (1)
    0x75, 0x01, //     Report Size (1)
    0x95, 0x03, //     Report Count (3)
    0x81, 0x02, //     Input (Data, Variable, Absolute)
    // Padding
    0x75, 0x05, //     Report Size (5)
    0x95, 0x01, //     Report Count (1)
    0x81, 0x01, //     Input (Constant)
    // X, Y movement
    0x05, 0x01, //     Usage Page (Generic Desktop)
    0x09, 0x30, //     Usage (X)
    0x09, 0x31, //     Usage (Y)
    0x15, 0x81, //     Logical Minimum (-127)
    0x25, 0x7F, //     Logical Maximum (127)
    0x75, 0x08, //     Report Size (8)
    0x95, 0x02, //     Report Count (2)
    0x81, 0x06, //     Input (Data, Variable, Relative)
    // Wheel
    0x09, 0x38, //     Usage (Wheel)
    0x15, 0x81, //     Logical Minimum (-127)
    0x25, 0x7F, //     Logical Maximum (127)
    0x75, 0x08, //     Report Size (8)
    0x95, 0x01, //     Report Count (1)
    0x81, 0x06, //     Input (Data, Variable, Relative)
    0xC0, //   End Collection
    0xC0, // End Collection
];

/// Absolute mouse (touchpad) report descriptor - 6 bytes
/// Format: buttons(1), x_low(1), x_high(1), y_low(1), y_high(1), wheel(1)
pub const TOUCHPAD_REPORT_DESCRIPTOR: &[u8] = &[
    0x05, 0x01, // Usage Page (Generic Desktop)
    0x09, 0x02, // Usage (Mouse)
    0xA1, 0x01, // Collection (Application)
    0x09, 0x01, //   Usage (Pointer)
    0xA1, 0x00, //   Collection (Physical)
    // Buttons (5 buttons for touchpad gestures)
    0x05, 0x09, //     Usage Page (Buttons)
    0x19, 0x01, //     Usage Minimum (Button 1)
    0x29, 0x05, //     Usage Maximum (Button 5)
    0x15, 0x00, //     Logical Minimum (0)
    0x25, 0x01, //     Logical Maximum (1)
    0x75, 0x01, //     Report Size (1)
    0x95, 0x05, //     Report Count (5)
    0x81, 0x02, //     Input (Data, Variable, Absolute)
    // Padding
    0x75, 0x03, //     Report Size (3)
    0x95, 0x01, //     Report Count (1)
    0x81, 0x01, //     Input (Constant)
    // X position (absolute, 16-bit)
    0x05, 0x01, //     Usage Page (Generic Desktop)
    0x09, 0x30, //     Usage (X)
    0x15, 0x00, //     Logical Minimum (0)
    0x26, 0xFF, 0x7F, // Logical Maximum (32767)
    0x75, 0x10, //     Report Size (16)
    0x95, 0x01, //     Report Count (1)
    0x81, 0x02, //     Input (Data, Variable, Absolute)
    // Y position (absolute, 16-bit)
    0x09, 0x31, //     Usage (Y)
    0x15, 0x00, //     Logical Minimum (0)
    0x26, 0xFF, 0x7F, // Logical Maximum (32767)
    0x75, 0x10, //     Report Size (16)
    0x95, 0x01, //     Report Count (1)
    0x81, 0x02, //     Input (Data, Variable, Absolute)
    0xC0, //   End Collection
    0xC0, // End Collection
];

/// Consumer control (multimedia keys) report descriptor - 2 bytes
pub const CONSUMER_CONTROL_REPORT_DESCRIPTOR: &[u8] = &[
    0x05, 0x0C, // Usage Page (Consumer)
    0x09, 0x01, // Usage (Consumer Control)
    0xA1, 0x01, // Collection (Application)
    0x15, 0x00, //   Logical Minimum (0)
    0x26, 0xFF, 0x03, // Logical Maximum (1023)
    0x19, 0x00, //   Usage Minimum (0)
    0x2A, 0xFF, 0x03, // Usage Maximum (1023)
    0x75, 0x10, //   Report Size (16)
    0x95, 0x01, //   Report Count (1)
    0x81, 0x00, //   Input (Data, Array)
    0xC0, // End Collection
];

/// Gamepad report descriptor - 7 bytes
/// Format: buttons_low(1), buttons_high(1), hat(1), lx(1), ly(1), rx(1), ry(1)
pub const GAMEPAD_REPORT_DESCRIPTOR: &[u8] = &[
    0x05, 0x01, // Usage Page (Generic Desktop)
    0x09, 0x05, // Usage (Gamepad)
    0xA1, 0x01, // Collection (Application)
    // 16 buttons
    0x05, 0x09, //   Usage Page (Buttons)
    0x19, 0x01, //   Usage Minimum (Button 1)
    0x29, 0x10, //   Usage Maximum (Button 16)
    0x15, 0x00, //   Logical Minimum (0)
    0x25, 0x01, //   Logical Maximum (1)
    0x75, 0x01, //   Report Size (1)
    0x95, 0x10, //   Report Count (16)
    0x81, 0x02, //   Input (Data, Variable, Absolute)
    // Hat switch (D-pad)
    0x05, 0x01, //   Usage Page (Generic Desktop)
    0x09, 0x39, //   Usage (Hat switch)
    0x15, 0x00, //   Logical Minimum (0)
    0x25, 0x07, //   Logical Maximum (7)
    0x35, 0x00, //   Physical Minimum (0)
    0x46, 0x3B, 0x01, // Physical Maximum (315)
    0x65, 0x14, //   Unit (Degrees)
    0x75, 0x04, //   Report Size (4)
    0x95, 0x01, //   Report Count (1)
    0x81, 0x42, //   Input (Data, Variable, Absolute, Null State)
    // Padding
    0x75, 0x04, //   Report Size (4)
    0x95, 0x01, //   Report Count (1)
    0x81, 0x01, //   Input (Constant)
    // Left stick X, Y
    0x09, 0x30, //   Usage (X)
    0x09, 0x31, //   Usage (Y)
    0x15, 0x00, //   Logical Minimum (0)
    0x26, 0xFF, 0x00, // Logical Maximum (255)
    0x75, 0x08, //   Report Size (8)
    0x95, 0x02, //   Report Count (2)
    0x81, 0x02, //   Input (Data, Variable, Absolute)
    // Right stick X, Y
    0x09, 0x32, //   Usage (Z)
    0x09, 0x35, //   Usage (Rz)
    0x15, 0x00, //   Logical Minimum (0)
    0x26, 0xFF, 0x00, // Logical Maximum (255)
    0x75, 0x08, //   Report Size (8)
    0x95, 0x02, //   Report Count (2)
    0x81, 0x02, //   Input (Data, Variable, Absolute)
    0xC0, // End Collection
];

/// Digitizer (touch screen) report descriptor - single touch
/// Format: tip_switch(1), x_low(1), x_high(1), y_low(1), y_high(1)
pub const DIGITIZER_REPORT_DESCRIPTOR: &[u8] = &[
    0x05, 0x0D, // Usage Page (Digitizer)
    0x09, 0x04, // Usage (Touch Screen)
    0xA1, 0x01, // Collection (Application)
    0x09, 0x22, //   Usage (Finger)
    0xA1, 0x02, //   Collection (Logical)
    // Tip switch (touch down)
    0x09, 0x42, //     Usage (Tip Switch)
    0x15, 0x00, //     Logical Minimum (0)
    0x25, 0x01, //     Logical Maximum (1)
    0x75, 0x01, //     Report Size (1)
    0x95, 0x01, //     Report Count (1)
    0x81, 0x02, //     Input (Data, Variable, Absolute)
    // In-range
    0x09, 0x32, //     Usage (In Range)
    0x81, 0x02, //     Input (Data, Variable, Absolute)
    // Padding
    0x75, 0x06, //     Report Size (6)
    0x95, 0x01, //     Report Count (1)
    0x81, 0x01, //     Input (Constant)
    // X position
    0x05, 0x01, //     Usage Page (Generic Desktop)
    0x09, 0x30, //     Usage (X)
    0x15, 0x00, //     Logical Minimum (0)
    0x26, 0xFF, 0x7F, // Logical Maximum (32767)
    0x75, 0x10, //     Report Size (16)
    0x95, 0x01, //     Report Count (1)
    0x81, 0x02, //     Input (Data, Variable, Absolute)
    // Y position
    0x09, 0x31, //     Usage (Y)
    0x15, 0x00, //     Logical Minimum (0)
    0x26, 0xFF, 0x7F, // Logical Maximum (32767)
    0x75, 0x10, //     Report Size (16)
    0x95, 0x01, //     Report Count (1)
    0x81, 0x02, //     Input (Data, Variable, Absolute)
    0xC0, //   End Collection
    0xC0, // End Collection
];

/// System control report descriptor (power buttons) - 1 byte
pub const SYSTEM_CONTROL_REPORT_DESCRIPTOR: &[u8] = &[
    0x05, 0x01, // Usage Page (Generic Desktop)
    0x09, 0x80, // Usage (System Control)
    0xA1, 0x01, // Collection (Application)
    0x19, 0x81, //   Usage Minimum (System Power Down)
    0x29, 0x83, //   Usage Maximum (System Wake Up)
    0x15, 0x00, //   Logical Minimum (0)
    0x25, 0x01, //   Logical Maximum (1)
    0x75, 0x01, //   Report Size (1)
    0x95, 0x03, //   Report Count (3)
    0x81, 0x02, //   Input (Data, Variable, Absolute)
    // Padding
    0x75, 0x05, //   Report Size (5)
    0x95, 0x01, //   Report Count (1)
    0x81, 0x01, //   Input (Constant)
    0xC0, // End Collection
];

/// Keyboard report (8 bytes)
#[derive(Clone, Copy, Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct KeyboardReport {
    /// Modifier keys (Ctrl, Shift, Alt, GUI)
    pub modifier: u8,
    /// Reserved byte
    pub reserved: u8,
    /// Key codes (up to 6 simultaneous keys)
    pub keycodes: [u8; 6],
}

impl KeyboardReport {
    /// Create a new empty keyboard report
    pub const fn new() -> Self {
        Self {
            modifier: 0,
            reserved: 0,
            keycodes: [0; 6],
        }
    }

    /// Set modifier keys
    pub const fn with_modifier(mut self, modifier: u8) -> Self {
        self.modifier = modifier;
        self
    }

    /// Add a key code
    pub fn with_key(mut self, keycode: u8) -> Self {
        for slot in &mut self.keycodes {
            if *slot == 0 {
                *slot = keycode;
                break;
            }
        }
        self
    }

    /// Convert to bytes
    pub fn to_bytes(&self) -> [u8; 8] {
        [
            self.modifier,
            self.reserved,
            self.keycodes[0],
            self.keycodes[1],
            self.keycodes[2],
            self.keycodes[3],
            self.keycodes[4],
            self.keycodes[5],
        ]
    }
}

/// Keyboard modifier key bits
pub mod modifier {
    /// Left Control
    pub const LEFT_CTRL: u8 = 1 << 0;
    /// Left Shift
    pub const LEFT_SHIFT: u8 = 1 << 1;
    /// Left Alt
    pub const LEFT_ALT: u8 = 1 << 2;
    /// Left GUI (Windows/Command)
    pub const LEFT_GUI: u8 = 1 << 3;
    /// Right Control
    pub const RIGHT_CTRL: u8 = 1 << 4;
    /// Right Shift
    pub const RIGHT_SHIFT: u8 = 1 << 5;
    /// Right Alt
    pub const RIGHT_ALT: u8 = 1 << 6;
    /// Right GUI
    pub const RIGHT_GUI: u8 = 1 << 7;
}

/// Common USB HID key codes
pub mod keycode {
    /// Key A
    pub const A: u8 = 0x04;
    /// Key B
    pub const B: u8 = 0x05;
    /// Key C
    pub const C: u8 = 0x06;
    /// Key D
    pub const D: u8 = 0x07;
    /// Key E
    pub const E: u8 = 0x08;
    /// Key F
    pub const F: u8 = 0x09;
    /// Key G
    pub const G: u8 = 0x0A;
    /// Key H
    pub const H: u8 = 0x0B;
    /// Key I
    pub const I: u8 = 0x0C;
    /// Key J
    pub const J: u8 = 0x0D;
    /// Key K
    pub const K: u8 = 0x0E;
    /// Key L
    pub const L: u8 = 0x0F;
    /// Key M
    pub const M: u8 = 0x10;
    /// Key N
    pub const N: u8 = 0x11;
    /// Key O
    pub const O: u8 = 0x12;
    /// Key P
    pub const P: u8 = 0x13;
    /// Key Q
    pub const Q: u8 = 0x14;
    /// Key R
    pub const R: u8 = 0x15;
    /// Key S
    pub const S: u8 = 0x16;
    /// Key T
    pub const T: u8 = 0x17;
    /// Key U
    pub const U: u8 = 0x18;
    /// Key V
    pub const V: u8 = 0x19;
    /// Key W
    pub const W: u8 = 0x1A;
    /// Key X
    pub const X: u8 = 0x1B;
    /// Key Y
    pub const Y: u8 = 0x1C;
    /// Key Z
    pub const Z: u8 = 0x1D;
    /// Number 1
    pub const NUM_1: u8 = 0x1E;
    /// Number 2
    pub const NUM_2: u8 = 0x1F;
    /// Number 3
    pub const NUM_3: u8 = 0x20;
    /// Number 4
    pub const NUM_4: u8 = 0x21;
    /// Number 5
    pub const NUM_5: u8 = 0x22;
    /// Number 6
    pub const NUM_6: u8 = 0x23;
    /// Number 7
    pub const NUM_7: u8 = 0x24;
    /// Number 8
    pub const NUM_8: u8 = 0x25;
    /// Number 9
    pub const NUM_9: u8 = 0x26;
    /// Number 0
    pub const NUM_0: u8 = 0x27;
    /// Enter key
    pub const ENTER: u8 = 0x28;
    /// Escape key
    pub const ESCAPE: u8 = 0x29;
    /// Backspace key
    pub const BACKSPACE: u8 = 0x2A;
    /// Tab key
    pub const TAB: u8 = 0x2B;
    /// Space key
    pub const SPACE: u8 = 0x2C;
    /// F1 key
    pub const F1: u8 = 0x3A;
    /// F2 key
    pub const F2: u8 = 0x3B;
    /// F3 key
    pub const F3: u8 = 0x3C;
    /// F4 key
    pub const F4: u8 = 0x3D;
    /// F5 key
    pub const F5: u8 = 0x3E;
    /// F6 key
    pub const F6: u8 = 0x3F;
    /// F7 key
    pub const F7: u8 = 0x40;
    /// F8 key
    pub const F8: u8 = 0x41;
    /// F9 key
    pub const F9: u8 = 0x42;
    /// F10 key
    pub const F10: u8 = 0x43;
    /// F11 key
    pub const F11: u8 = 0x44;
    /// F12 key
    pub const F12: u8 = 0x45;
    /// Print Screen key
    pub const PRINT_SCREEN: u8 = 0x46;
    /// Scroll Lock key
    pub const SCROLL_LOCK: u8 = 0x47;
    /// Pause key
    pub const PAUSE: u8 = 0x48;
    /// Insert key
    pub const INSERT: u8 = 0x49;
    /// Home key
    pub const HOME: u8 = 0x4A;
    /// Page Up key
    pub const PAGE_UP: u8 = 0x4B;
    /// Delete key
    pub const DELETE: u8 = 0x4C;
    /// End key
    pub const END: u8 = 0x4D;
    /// Page Down key
    pub const PAGE_DOWN: u8 = 0x4E;
    /// Right Arrow key
    pub const RIGHT_ARROW: u8 = 0x4F;
    /// Left Arrow key
    pub const LEFT_ARROW: u8 = 0x50;
    /// Down Arrow key
    pub const DOWN_ARROW: u8 = 0x51;
    /// Up Arrow key
    pub const UP_ARROW: u8 = 0x52;
    /// Caps Lock key
    pub const CAPS_LOCK: u8 = 0x39;
    /// Minus key
    pub const MINUS: u8 = 0x2D;
    /// Equals key
    pub const EQUALS: u8 = 0x2E;
    /// Left bracket key
    pub const LEFT_BRACKET: u8 = 0x2F;
    /// Right bracket key
    pub const RIGHT_BRACKET: u8 = 0x30;
    /// Backslash key
    pub const BACKSLASH: u8 = 0x31;
    /// Semicolon key
    pub const SEMICOLON: u8 = 0x33;
    /// Quote key
    pub const QUOTE: u8 = 0x34;
    /// Grave/Tilde key
    pub const GRAVE: u8 = 0x35;
    /// Comma key
    pub const COMMA: u8 = 0x36;
    /// Period key
    pub const PERIOD: u8 = 0x37;
    /// Slash key
    pub const SLASH: u8 = 0x38;
}

/// Mouse report (4 bytes)
#[derive(Clone, Copy, Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct MouseReport {
    /// Button states
    pub buttons: u8,
    /// X movement (-127 to 127)
    pub x: i8,
    /// Y movement (-127 to 127)
    pub y: i8,
    /// Wheel movement (-127 to 127)
    pub wheel: i8,
}

impl MouseReport {
    /// Create a new empty mouse report
    pub const fn new() -> Self {
        Self {
            buttons: 0,
            x: 0,
            y: 0,
            wheel: 0,
        }
    }

    /// Set button states
    pub const fn with_buttons(mut self, buttons: u8) -> Self {
        self.buttons = buttons;
        self
    }

    /// Set X movement
    pub const fn with_x(mut self, x: i8) -> Self {
        self.x = x;
        self
    }

    /// Set Y movement
    pub const fn with_y(mut self, y: i8) -> Self {
        self.y = y;
        self
    }

    /// Set wheel movement
    pub const fn with_wheel(mut self, wheel: i8) -> Self {
        self.wheel = wheel;
        self
    }

    /// Convert to bytes
    pub fn to_bytes(&self) -> [u8; 4] {
        [self.buttons, self.x as u8, self.y as u8, self.wheel as u8]
    }
}

/// Mouse button bits
pub mod mouse_button {
    /// Left button
    pub const LEFT: u8 = 1 << 0;
    /// Right button
    pub const RIGHT: u8 = 1 << 1;
    /// Middle button
    pub const MIDDLE: u8 = 1 << 2;
}

/// Touchpad (absolute mouse) report (5 bytes)
#[derive(Clone, Copy, Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct TouchpadReport {
    /// Button states (5 buttons for gestures)
    pub buttons: u8,
    /// Absolute X position (0-32767)
    pub x: u16,
    /// Absolute Y position (0-32767)
    pub y: u16,
}

impl TouchpadReport {
    /// Create a new empty touchpad report
    pub const fn new() -> Self {
        Self {
            buttons: 0,
            x: 0,
            y: 0,
        }
    }

    /// Set button states
    pub const fn with_buttons(mut self, buttons: u8) -> Self {
        self.buttons = buttons;
        self
    }

    /// Set X position (0-32767)
    pub const fn with_x(mut self, x: u16) -> Self {
        self.x = x;
        self
    }

    /// Set Y position (0-32767)
    pub const fn with_y(mut self, y: u16) -> Self {
        self.y = y;
        self
    }

    /// Set both X and Y positions
    pub const fn with_position(mut self, x: u16, y: u16) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    /// Convert to bytes
    pub fn to_bytes(&self) -> [u8; 5] {
        [
            self.buttons,
            self.x as u8,
            (self.x >> 8) as u8,
            self.y as u8,
            (self.y >> 8) as u8,
        ]
    }
}

/// Touchpad button bits
pub mod touchpad_button {
    /// Primary tap/click
    pub const TAP: u8 = 1 << 0;
    /// Secondary tap (two-finger tap)
    pub const SECONDARY: u8 = 1 << 1;
    /// Middle tap (three-finger tap)
    pub const MIDDLE: u8 = 1 << 2;
    /// Forward gesture
    pub const FORWARD: u8 = 1 << 3;
    /// Back gesture
    pub const BACK: u8 = 1 << 4;
}

/// Consumer control report (2 bytes)
#[derive(Clone, Copy, Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ConsumerReport {
    /// Consumer usage ID
    pub usage_id: u16,
}

impl ConsumerReport {
    /// Create a new empty consumer report
    pub const fn new() -> Self {
        Self { usage_id: 0 }
    }

    /// Set usage ID
    pub const fn with_usage(mut self, usage_id: u16) -> Self {
        self.usage_id = usage_id;
        self
    }

    /// Convert to bytes
    pub fn to_bytes(&self) -> [u8; 2] {
        [self.usage_id as u8, (self.usage_id >> 8) as u8]
    }
}

/// Common consumer control usage IDs
pub mod consumer_usage {
    /// Play/Pause toggle
    pub const PLAY_PAUSE: u16 = 0xCD;
    /// Stop
    pub const STOP: u16 = 0xB7;
    /// Next track
    pub const NEXT_TRACK: u16 = 0xB5;
    /// Previous track
    pub const PREV_TRACK: u16 = 0xB6;
    /// Mute toggle
    pub const MUTE: u16 = 0xE2;
    /// Volume up
    pub const VOLUME_UP: u16 = 0xE9;
    /// Volume down
    pub const VOLUME_DOWN: u16 = 0xEA;
    /// Browser home
    pub const BROWSER_HOME: u16 = 0x223;
    /// Browser back
    pub const BROWSER_BACK: u16 = 0x224;
    /// Browser forward
    pub const BROWSER_FORWARD: u16 = 0x225;
    /// Browser refresh
    pub const BROWSER_REFRESH: u16 = 0x227;
    /// Browser search
    pub const BROWSER_SEARCH: u16 = 0x221;
    /// Calculator
    pub const CALCULATOR: u16 = 0x192;
    /// Email
    pub const EMAIL: u16 = 0x18A;
    /// Media select
    pub const MEDIA_SELECT: u16 = 0x183;
    /// Brightness up
    pub const BRIGHTNESS_UP: u16 = 0x6F;
    /// Brightness down
    pub const BRIGHTNESS_DOWN: u16 = 0x70;
}

/// Gamepad report (7 bytes)
#[derive(Clone, Copy, Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct GamepadReport {
    /// Button states (16 buttons)
    pub buttons: u16,
    /// Hat switch (D-pad) - 0-7 for directions, 8 for neutral
    pub hat: u8,
    /// Left stick X (0-255, 128 = center)
    pub left_x: u8,
    /// Left stick Y (0-255, 128 = center)
    pub left_y: u8,
    /// Right stick X (0-255, 128 = center)
    pub right_x: u8,
    /// Right stick Y (0-255, 128 = center)
    pub right_y: u8,
}

impl GamepadReport {
    /// Center value for analog sticks
    pub const CENTER: u8 = 128;

    /// Create a new centered gamepad report
    pub const fn new() -> Self {
        Self {
            buttons: 0,
            hat: 8, // Neutral position
            left_x: 128,
            left_y: 128,
            right_x: 128,
            right_y: 128,
        }
    }

    /// Set button states
    pub const fn with_buttons(mut self, buttons: u16) -> Self {
        self.buttons = buttons;
        self
    }

    /// Set a single button pressed
    pub const fn with_button(mut self, button: u16) -> Self {
        self.buttons |= button;
        self
    }

    /// Set hat switch (D-pad) direction
    pub const fn with_hat(mut self, hat: u8) -> Self {
        self.hat = hat;
        self
    }

    /// Set left stick position
    pub const fn with_left_stick(mut self, x: u8, y: u8) -> Self {
        self.left_x = x;
        self.left_y = y;
        self
    }

    /// Set right stick position
    pub const fn with_right_stick(mut self, x: u8, y: u8) -> Self {
        self.right_x = x;
        self.right_y = y;
        self
    }

    /// Convert to bytes
    pub fn to_bytes(&self) -> [u8; 7] {
        [
            self.buttons as u8,
            (self.buttons >> 8) as u8,
            self.hat,
            self.left_x,
            self.left_y,
            self.right_x,
            self.right_y,
        ]
    }
}

/// Gamepad button bits
pub mod gamepad_button {
    // Face buttons (standard layout)
    /// A / Cross button
    pub const A: u16 = 1 << 0;
    /// B / Circle button
    pub const B: u16 = 1 << 1;
    /// X / Square button
    pub const X: u16 = 1 << 2;
    /// Y / Triangle button
    pub const Y: u16 = 1 << 3;
    // Shoulder buttons
    /// Left bumper / L1
    pub const LEFT_BUMPER: u16 = 1 << 4;
    /// Right bumper / R1
    pub const RIGHT_BUMPER: u16 = 1 << 5;
    /// Left trigger / L2
    pub const LEFT_TRIGGER: u16 = 1 << 6;
    /// Right trigger / R2
    pub const RIGHT_TRIGGER: u16 = 1 << 7;
    // System buttons
    /// Back / Select / Share
    pub const BACK: u16 = 1 << 8;
    /// Start / Options
    pub const START: u16 = 1 << 9;
    /// Left stick press / L3
    pub const LEFT_STICK: u16 = 1 << 10;
    /// Right stick press / R3
    pub const RIGHT_STICK: u16 = 1 << 11;
    /// Home / Guide / PS button
    pub const HOME: u16 = 1 << 12;
    /// Capture / Touchpad press
    pub const CAPTURE: u16 = 1 << 13;
}

/// Hat switch (D-pad) directions
pub mod hat_direction {
    /// Up
    pub const UP: u8 = 0;
    /// Up-Right
    pub const UP_RIGHT: u8 = 1;
    /// Right
    pub const RIGHT: u8 = 2;
    /// Down-Right
    pub const DOWN_RIGHT: u8 = 3;
    /// Down
    pub const DOWN: u8 = 4;
    /// Down-Left
    pub const DOWN_LEFT: u8 = 5;
    /// Left
    pub const LEFT: u8 = 6;
    /// Up-Left
    pub const UP_LEFT: u8 = 7;
    /// Neutral (no direction)
    pub const NEUTRAL: u8 = 8;
}

/// Digitizer (touch screen) report (5 bytes)
#[derive(Clone, Copy, Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct DigitizerReport {
    /// Tip switch (touch down) and in-range flags
    pub flags: u8,
    /// X position (0-32767)
    pub x: u16,
    /// Y position (0-32767)
    pub y: u16,
}

impl DigitizerReport {
    /// Create a new empty digitizer report
    pub const fn new() -> Self {
        Self {
            flags: 0,
            x: 0,
            y: 0,
        }
    }

    /// Set touch down (tip switch)
    pub const fn with_touch(mut self, touched: bool) -> Self {
        if touched {
            self.flags |= 0x01;
        } else {
            self.flags &= !0x01;
        }
        self
    }

    /// Set in-range flag
    pub const fn with_in_range(mut self, in_range: bool) -> Self {
        if in_range {
            self.flags |= 0x02;
        } else {
            self.flags &= !0x02;
        }
        self
    }

    /// Set X position (0-32767)
    pub const fn with_x(mut self, x: u16) -> Self {
        self.x = x;
        self
    }

    /// Set Y position (0-32767)
    pub const fn with_y(mut self, y: u16) -> Self {
        self.y = y;
        self
    }

    /// Set position
    pub const fn with_position(mut self, x: u16, y: u16) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    /// Convert to bytes
    pub fn to_bytes(&self) -> [u8; 5] {
        [
            self.flags,
            self.x as u8,
            (self.x >> 8) as u8,
            self.y as u8,
            (self.y >> 8) as u8,
        ]
    }
}

/// Digitizer flags
pub mod digitizer_flag {
    /// Tip switch (finger touching screen)
    pub const TIP_SWITCH: u8 = 1 << 0;
    /// In range (finger near screen)
    pub const IN_RANGE: u8 = 1 << 1;
}

/// System control report (1 byte)
#[derive(Clone, Copy, Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SystemControlReport {
    /// System control flags
    pub flags: u8,
}

impl SystemControlReport {
    /// Create a new empty system control report
    pub const fn new() -> Self {
        Self { flags: 0 }
    }

    /// Set power down flag
    pub const fn with_power_down(mut self, pressed: bool) -> Self {
        if pressed {
            self.flags |= 0x01;
        } else {
            self.flags &= !0x01;
        }
        self
    }

    /// Set sleep flag
    pub const fn with_sleep(mut self, pressed: bool) -> Self {
        if pressed {
            self.flags |= 0x02;
        } else {
            self.flags &= !0x02;
        }
        self
    }

    /// Set wake up flag
    pub const fn with_wake_up(mut self, pressed: bool) -> Self {
        if pressed {
            self.flags |= 0x04;
        } else {
            self.flags &= !0x04;
        }
        self
    }

    /// Convert to bytes
    pub fn to_bytes(&self) -> [u8; 1] {
        [self.flags]
    }
}

/// System control button bits
pub mod system_control {
    /// Power down
    pub const POWER_DOWN: u8 = 1 << 0;
    /// Sleep
    pub const SLEEP: u8 = 1 << 1;
    /// Wake up
    pub const WAKE_UP: u8 = 1 << 2;
}

/// HID device configuration
#[derive(Clone)]
pub struct HidConfig {
    /// Report descriptor
    pub report_descriptor: &'static [u8],
    /// Subclass
    pub subclass: u8,
    /// Protocol
    pub protocol: u8,
    /// Input report size (device to host)
    pub in_report_size: usize,
    /// Output report size (host to device, e.g., keyboard LEDs)
    pub out_report_size: usize,
    /// Polling interval in ms
    pub poll_interval: u8,
    /// Whether to use an OUT endpoint for output reports (vs control pipe)
    pub use_out_endpoint: bool,
    /// HID version (BCD, default 0x0111 = 1.11)
    pub hid_version: u16,
    /// Country code (0 = not supported)
    pub country_code: u8,
}

impl HidConfig {
    /// Create a new custom HID configuration builder
    pub const fn new(report_descriptor: &'static [u8]) -> Self {
        Self {
            report_descriptor,
            subclass: hid_subclass::NONE,
            protocol: hid_protocol::NONE,
            in_report_size: 8,
            out_report_size: 0,
            poll_interval: 10,
            use_out_endpoint: false,
            hid_version: 0x0111, // HID 1.11
            country_code: 0,
        }
    }

    /// Keyboard configuration
    pub const fn keyboard() -> Self {
        Self {
            report_descriptor: KEYBOARD_REPORT_DESCRIPTOR,
            subclass: hid_subclass::BOOT,
            protocol: hid_protocol::KEYBOARD,
            in_report_size: 8,
            out_report_size: 1, // LED report
            poll_interval: 10,
            use_out_endpoint: false,
            hid_version: 0x0111,
            country_code: 0,
        }
    }

    /// Mouse configuration
    pub const fn mouse() -> Self {
        Self {
            report_descriptor: MOUSE_REPORT_DESCRIPTOR,
            subclass: hid_subclass::BOOT,
            protocol: hid_protocol::MOUSE,
            in_report_size: 4,
            out_report_size: 0,
            poll_interval: 10,
            use_out_endpoint: false,
            hid_version: 0x0111,
            country_code: 0,
        }
    }

    /// Touchpad (absolute mouse) configuration
    pub const fn touchpad() -> Self {
        Self {
            report_descriptor: TOUCHPAD_REPORT_DESCRIPTOR,
            subclass: hid_subclass::NONE,
            protocol: hid_protocol::MOUSE,
            in_report_size: 5, // buttons(1) + x(2) + y(2)
            out_report_size: 0,
            poll_interval: 10,
            use_out_endpoint: false,
            hid_version: 0x0111,
            country_code: 0,
        }
    }

    /// Gamepad configuration
    pub const fn gamepad() -> Self {
        Self {
            report_descriptor: GAMEPAD_REPORT_DESCRIPTOR,
            subclass: hid_subclass::NONE,
            protocol: hid_protocol::NONE,
            in_report_size: 7, // buttons(2) + hat(1) + lx(1) + ly(1) + rx(1) + ry(1)
            out_report_size: 0,
            poll_interval: 10,
            use_out_endpoint: false,
            hid_version: 0x0111,
            country_code: 0,
        }
    }

    /// Consumer control (multimedia keys) configuration
    pub const fn consumer_control() -> Self {
        Self {
            report_descriptor: CONSUMER_CONTROL_REPORT_DESCRIPTOR,
            subclass: hid_subclass::NONE,
            protocol: hid_protocol::NONE,
            in_report_size: 2, // usage id (16-bit)
            out_report_size: 0,
            poll_interval: 10,
            use_out_endpoint: false,
            hid_version: 0x0111,
            country_code: 0,
        }
    }

    /// Digitizer (touch screen) configuration
    pub const fn digitizer() -> Self {
        Self {
            report_descriptor: DIGITIZER_REPORT_DESCRIPTOR,
            subclass: hid_subclass::NONE,
            protocol: hid_protocol::NONE,
            in_report_size: 5, // flags(1) + x(2) + y(2)
            out_report_size: 0,
            poll_interval: 10,
            use_out_endpoint: false,
            hid_version: 0x0111,
            country_code: 0,
        }
    }

    /// System control (power buttons) configuration
    pub const fn system_control() -> Self {
        Self {
            report_descriptor: SYSTEM_CONTROL_REPORT_DESCRIPTOR,
            subclass: hid_subclass::NONE,
            protocol: hid_protocol::NONE,
            in_report_size: 1,
            out_report_size: 0,
            poll_interval: 50, // Power buttons don't need frequent polling
            use_out_endpoint: false,
            hid_version: 0x0111,
            country_code: 0,
        }
    }

    /// Custom HID device (legacy compatibility)
    pub const fn custom(report_descriptor: &'static [u8], report_size: usize) -> Self {
        Self {
            report_descriptor,
            subclass: hid_subclass::NONE,
            protocol: hid_protocol::NONE,
            in_report_size: report_size,
            out_report_size: 0,
            poll_interval: 10,
            use_out_endpoint: false,
            hid_version: 0x0111,
            country_code: 0,
        }
    }

    /// Set the HID subclass
    pub const fn subclass(mut self, subclass: u8) -> Self {
        self.subclass = subclass;
        self
    }

    /// Set the HID protocol
    pub const fn protocol(mut self, protocol: u8) -> Self {
        self.protocol = protocol;
        self
    }

    /// Set input report size
    pub const fn in_report_size(mut self, size: usize) -> Self {
        self.in_report_size = size;
        self
    }

    /// Set output report size
    pub const fn out_report_size(mut self, size: usize) -> Self {
        self.out_report_size = size;
        self
    }

    /// Set polling interval in milliseconds
    pub const fn poll_interval_ms(mut self, ms: u8) -> Self {
        self.poll_interval = ms;
        self
    }

    /// Enable OUT endpoint for output reports
    pub const fn with_out_endpoint(mut self) -> Self {
        self.use_out_endpoint = true;
        self
    }

    /// Set country code
    pub const fn country_code(mut self, code: u8) -> Self {
        self.country_code = code;
        self
    }

    /// Set HID version (BCD format, e.g., 0x0111 for 1.11)
    pub const fn hid_version(mut self, version: u16) -> Self {
        self.hid_version = version;
        self
    }

    /// For legacy compatibility - get report_size
    pub const fn report_size(&self) -> usize {
        self.in_report_size
    }
}

/// HID class implementation
pub struct HidClass<'a> {
    /// USB bus reference
    bus: &'a UsbBus,
    /// Interface number
    interface: u8,
    /// Interrupt IN endpoint
    int_in_ep: EndpointAddress,
    /// Interrupt OUT endpoint (optional)
    int_out_ep: Option<EndpointAddress>,
    /// Configuration
    config: HidConfig,
    /// Idle rate (in 4ms units, 0 = infinite)
    idle_rate: u8,
    /// Current protocol (0 = boot, 1 = report)
    protocol: u8,
    /// Input report buffer
    in_report_buf: [u8; 64],
    /// Output report buffer (for received reports)
    out_report_buf: [u8; 64],
    /// Output report received length
    out_report_len: usize,
    /// Report pending flag
    report_pending: bool,
    /// Output report available flag
    out_report_available: bool,
    /// Configured flag
    configured: bool,
}

impl<'a> HidClass<'a> {
    /// Create a new HID class with custom configuration
    pub fn new(bus: &'a UsbBus, config: HidConfig) -> Self {
        let int_out_ep = if config.use_out_endpoint && config.out_report_size > 0 {
            Some(EndpointAddress::out(3))
        } else {
            None
        };

        Self {
            bus,
            interface: 0,
            int_in_ep: EndpointAddress::in_(3),
            int_out_ep,
            config,
            idle_rate: 0,
            protocol: 1, // Report protocol by default
            in_report_buf: [0u8; 64],
            out_report_buf: [0u8; 64],
            out_report_len: 0,
            report_pending: false,
            out_report_available: false,
            configured: false,
        }
    }

    /// Create a keyboard HID device
    pub fn keyboard(bus: &'a UsbBus) -> Self {
        Self::new(bus, HidConfig::keyboard())
    }

    /// Create a mouse HID device
    pub fn mouse(bus: &'a UsbBus) -> Self {
        Self::new(bus, HidConfig::mouse())
    }

    /// Create a touchpad HID device
    pub fn touchpad(bus: &'a UsbBus) -> Self {
        Self::new(bus, HidConfig::touchpad())
    }

    /// Create a gamepad HID device
    pub fn gamepad(bus: &'a UsbBus) -> Self {
        Self::new(bus, HidConfig::gamepad())
    }

    /// Create a consumer control (multimedia keys) HID device
    pub fn consumer_control(bus: &'a UsbBus) -> Self {
        Self::new(bus, HidConfig::consumer_control())
    }

    /// Create a digitizer (touch screen) HID device
    pub fn digitizer(bus: &'a UsbBus) -> Self {
        Self::new(bus, HidConfig::digitizer())
    }

    /// Create a system control (power buttons) HID device
    pub fn system_control(bus: &'a UsbBus) -> Self {
        Self::new(bus, HidConfig::system_control())
    }

    /// Check if configured
    pub fn is_configured(&self) -> bool {
        self.configured
    }

    /// Get the current protocol (0 = boot, 1 = report)
    pub fn protocol(&self) -> u8 {
        self.protocol
    }

    /// Get the idle rate (in 4ms units, 0 = infinite)
    pub fn idle_rate(&self) -> u8 {
        self.idle_rate
    }

    /// Check if an output report is available
    pub fn has_output_report(&self) -> bool {
        self.out_report_available
    }

    /// Read the output report (e.g., keyboard LEDs)
    ///
    /// Returns the number of bytes read, or None if no report is available.
    pub fn read_output_report(&mut self, buf: &mut [u8]) -> Option<usize> {
        if !self.out_report_available {
            return None;
        }

        let len = core::cmp::min(buf.len(), self.out_report_len);
        buf[..len].copy_from_slice(&self.out_report_buf[..len]);
        self.out_report_available = false;
        Some(len)
    }

    /// Send a HID input report
    pub fn send_report(&mut self, report: &[u8]) -> UsbResult<()> {
        if !self.configured {
            return Err(UsbError::NotReady);
        }

        if report.len() > self.in_report_buf.len() {
            return Err(UsbError::BufferOverflow);
        }

        // Copy report to buffer
        self.in_report_buf[..report.len()].copy_from_slice(report);
        self.report_pending = true;

        // Try to send immediately
        self.flush_report()
    }

    /// Send a keyboard report
    pub fn send_keyboard_report(&mut self, report: &KeyboardReport) -> UsbResult<()> {
        self.send_report(&report.to_bytes())
    }

    /// Send a mouse report
    pub fn send_mouse_report(&mut self, report: &MouseReport) -> UsbResult<()> {
        self.send_report(&report.to_bytes())
    }

    /// Send a touchpad report
    pub fn send_touchpad_report(&mut self, report: &TouchpadReport) -> UsbResult<()> {
        self.send_report(&report.to_bytes())
    }

    /// Send a consumer control report
    pub fn send_consumer_report(&mut self, report: &ConsumerReport) -> UsbResult<()> {
        self.send_report(&report.to_bytes())
    }

    /// Send a gamepad report
    pub fn send_gamepad_report(&mut self, report: &GamepadReport) -> UsbResult<()> {
        self.send_report(&report.to_bytes())
    }

    /// Send a digitizer (touch screen) report
    pub fn send_digitizer_report(&mut self, report: &DigitizerReport) -> UsbResult<()> {
        self.send_report(&report.to_bytes())
    }

    /// Send a system control report
    pub fn send_system_control_report(&mut self, report: &SystemControlReport) -> UsbResult<()> {
        self.send_report(&report.to_bytes())
    }

    /// Flush pending report
    fn flush_report(&mut self) -> UsbResult<()> {
        if !self.report_pending {
            return Ok(());
        }

        match self.bus.write(
            self.int_in_ep,
            &self.in_report_buf[..self.config.in_report_size],
        ) {
            Ok(_) => {
                self.report_pending = false;
                Ok(())
            }
            Err(UsbError::WouldBlock) => Err(UsbError::WouldBlock),
            Err(e) => Err(e),
        }
    }

    /// Handle HID class requests
    fn handle_hid_request(&mut self, setup: &SetupPacket) -> bool {
        let request = match HidRequest::try_from(setup.b_request) {
            Ok(r) => r,
            Err(_) => return false,
        };

        match request {
            HidRequest::GetReport => {
                // Will be handled in control_in
                true
            }
            HidRequest::GetIdle => {
                // Will be handled in control_in
                true
            }
            HidRequest::GetProtocol => {
                // Will be handled in control_in
                true
            }
            HidRequest::SetReport => {
                // Will be handled in control_out
                true
            }
            HidRequest::SetIdle => {
                self.idle_rate = (setup.w_value >> 8) as u8;
                self.bus.ep0_send_zlp();
                #[cfg(feature = "defmt")]
                defmt::debug!("HID: Set idle rate to {}", self.idle_rate);
                true
            }
            HidRequest::SetProtocol => {
                self.protocol = setup.w_value as u8;
                self.bus.ep0_send_zlp();
                #[cfg(feature = "defmt")]
                defmt::debug!("HID: Set protocol to {}", self.protocol);
                true
            }
        }
    }

    /// Build HID descriptor
    fn build_hid_descriptor(&self) -> [u8; 9] {
        let report_len = self.config.report_descriptor.len() as u16;
        let hid_version = self.config.hid_version;
        [
            9,                           // bLength
            hid_descriptor_type::HID,    // bDescriptorType (HID)
            hid_version as u8,           // bcdHID (low)
            (hid_version >> 8) as u8,    // bcdHID (high)
            self.config.country_code,    // bCountryCode
            0x01,                        // bNumDescriptors
            hid_descriptor_type::REPORT, // bDescriptorType (Report)
            report_len as u8,            // wDescriptorLength (low)
            (report_len >> 8) as u8,     // wDescriptorLength (high)
        ]
    }
}

impl<'a> UsbClass for HidClass<'a> {
    fn control_setup(&mut self, setup: &SetupPacket) -> bool {
        // Handle GET_DESCRIPTOR for HID/Report descriptors
        if setup.request_type() == RequestType::Standard
            && setup.recipient() == Recipient::Interface
            && setup.interface_number() == self.interface
        {
            let desc_type = (setup.w_value >> 8) as u8;

            if setup.b_request == 0x06 {
                // GET_DESCRIPTOR
                match desc_type {
                    hid_descriptor_type::HID => {
                        // Return HID descriptor
                        return true;
                    }
                    hid_descriptor_type::REPORT => {
                        // Return report descriptor
                        return true;
                    }
                    _ => {}
                }
            }
        }

        // Handle class-specific requests
        if setup.request_type() == RequestType::Class
            && setup.recipient() == Recipient::Interface
            && setup.interface_number() == self.interface
        {
            return self.handle_hid_request(setup);
        }

        false
    }

    fn control_in(&mut self, buf: &mut [u8]) -> UsbResult<usize> {
        // This would be called for GET_REPORT, GET_IDLE, GET_PROTOCOL
        // For now, return the current report or appropriate data
        let len = core::cmp::min(buf.len(), self.config.in_report_size);
        buf[..len].copy_from_slice(&self.in_report_buf[..len]);
        Ok(len)
    }

    fn control_out(&mut self, data: &[u8]) -> UsbResult<()> {
        // Handle SET_REPORT (e.g., keyboard LEDs)
        let len = core::cmp::min(data.len(), self.out_report_buf.len());
        self.out_report_buf[..len].copy_from_slice(&data[..len]);
        self.out_report_len = len;
        self.out_report_available = true;

        #[cfg(feature = "defmt")]
        defmt::debug!("HID: Received output report: {:02x}", data);
        Ok(())
    }

    fn get_descriptors(&self, writer: &mut DescriptorWriter) -> bool {
        // Count endpoints
        let num_endpoints = if self.int_out_ep.is_some() { 2 } else { 1 };

        // HID Interface
        let iface = InterfaceDescriptor::new(
            self.interface,
            HID_CLASS,
            self.config.subclass,
            self.config.protocol,
            num_endpoints,
        );
        if !writer.write_interface(&iface) {
            return false;
        }

        // HID Descriptor
        let hid_desc = self.build_hid_descriptor();
        if !writer.write(&hid_desc) {
            return false;
        }

        // Interrupt IN Endpoint
        let ep_in_desc = EndpointDescriptor::new(
            self.int_in_ep,
            EndpointType::Interrupt,
            self.config.in_report_size as u16,
            self.config.poll_interval,
        );
        if !writer.write_endpoint(&ep_in_desc) {
            return false;
        }

        // Optional Interrupt OUT Endpoint
        if let Some(out_ep) = self.int_out_ep {
            let ep_out_desc = EndpointDescriptor::new(
                out_ep,
                EndpointType::Interrupt,
                self.config.out_report_size as u16,
                self.config.poll_interval,
            );
            if !writer.write_endpoint(&ep_out_desc) {
                return false;
            }
        }

        true
    }

    fn configure(&mut self) {
        self.configured = true;
        #[cfg(feature = "defmt")]
        defmt::info!("HID: Configured");
    }

    fn reset(&mut self) {
        self.configured = false;
        self.report_pending = false;
        self.out_report_available = false;
        self.out_report_len = 0;
        self.idle_rate = 0;
        self.protocol = 1;
        #[cfg(feature = "defmt")]
        defmt::info!("HID: Reset");
    }

    fn poll(&mut self) {
        if !self.configured {
            return;
        }

        // Try to flush pending input report
        let _ = self.flush_report();

        // Check for incoming data on OUT endpoint
        if let Some(out_ep) = self.int_out_ep {
            let mut buf = [0u8; 64];
            if let Ok(len) = self.bus.read(out_ep, &mut buf)
                && len > 0
            {
                let copy_len = core::cmp::min(len, self.out_report_buf.len());
                self.out_report_buf[..copy_len].copy_from_slice(&buf[..copy_len]);
                self.out_report_len = copy_len;
                self.out_report_available = true;
                #[cfg(feature = "defmt")]
                defmt::debug!("HID: Received OUT report: {:02x}", &buf[..len]);
            }
        }
    }
}
