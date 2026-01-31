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
use crate::descriptor::{class_code, DescriptorWriter, EndpointDescriptor, InterfaceDescriptor};
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
    pub const A: u8 = 0x04;
    pub const B: u8 = 0x05;
    pub const C: u8 = 0x06;
    pub const D: u8 = 0x07;
    pub const E: u8 = 0x08;
    pub const F: u8 = 0x09;
    pub const G: u8 = 0x0A;
    pub const H: u8 = 0x0B;
    pub const I: u8 = 0x0C;
    pub const J: u8 = 0x0D;
    pub const K: u8 = 0x0E;
    pub const L: u8 = 0x0F;
    pub const M: u8 = 0x10;
    pub const N: u8 = 0x11;
    pub const O: u8 = 0x12;
    pub const P: u8 = 0x13;
    pub const Q: u8 = 0x14;
    pub const R: u8 = 0x15;
    pub const S: u8 = 0x16;
    pub const T: u8 = 0x17;
    pub const U: u8 = 0x18;
    pub const V: u8 = 0x19;
    pub const W: u8 = 0x1A;
    pub const X: u8 = 0x1B;
    pub const Y: u8 = 0x1C;
    pub const Z: u8 = 0x1D;
    pub const NUM_1: u8 = 0x1E;
    pub const NUM_2: u8 = 0x1F;
    pub const NUM_3: u8 = 0x20;
    pub const NUM_4: u8 = 0x21;
    pub const NUM_5: u8 = 0x22;
    pub const NUM_6: u8 = 0x23;
    pub const NUM_7: u8 = 0x24;
    pub const NUM_8: u8 = 0x25;
    pub const NUM_9: u8 = 0x26;
    pub const NUM_0: u8 = 0x27;
    pub const ENTER: u8 = 0x28;
    pub const ESCAPE: u8 = 0x29;
    pub const BACKSPACE: u8 = 0x2A;
    pub const TAB: u8 = 0x2B;
    pub const SPACE: u8 = 0x2C;
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

/// HID device configuration
#[derive(Clone)]
pub struct HidConfig {
    /// Report descriptor
    pub report_descriptor: &'static [u8],
    /// Subclass
    pub subclass: u8,
    /// Protocol
    pub protocol: u8,
    /// Report size
    pub report_size: usize,
    /// Polling interval in ms
    pub poll_interval: u8,
}

impl HidConfig {
    /// Keyboard configuration
    pub const fn keyboard() -> Self {
        Self {
            report_descriptor: KEYBOARD_REPORT_DESCRIPTOR,
            subclass: hid_subclass::BOOT,
            protocol: hid_protocol::KEYBOARD,
            report_size: 8,
            poll_interval: 10,
        }
    }

    /// Mouse configuration
    pub const fn mouse() -> Self {
        Self {
            report_descriptor: MOUSE_REPORT_DESCRIPTOR,
            subclass: hid_subclass::BOOT,
            protocol: hid_protocol::MOUSE,
            report_size: 4,
            poll_interval: 10,
        }
    }

    /// Custom HID device
    pub const fn custom(report_descriptor: &'static [u8], report_size: usize) -> Self {
        Self {
            report_descriptor,
            subclass: hid_subclass::NONE,
            protocol: hid_protocol::NONE,
            report_size,
            poll_interval: 10,
        }
    }
}

/// HID class implementation
pub struct HidClass<'a> {
    /// USB bus reference
    bus: &'a UsbBus,
    /// Interface number
    interface: u8,
    /// Interrupt IN endpoint
    int_ep: EndpointAddress,
    /// Configuration
    config: HidConfig,
    /// Idle rate (in 4ms units, 0 = infinite)
    idle_rate: u8,
    /// Current protocol (0 = boot, 1 = report)
    protocol: u8,
    /// Report buffer
    report_buf: [u8; 64],
    /// Report pending flag
    report_pending: bool,
    /// Configured flag
    configured: bool,
}

impl<'a> HidClass<'a> {
    /// Create a new HID class with custom configuration
    pub fn new(bus: &'a UsbBus, config: HidConfig) -> Self {
        Self {
            bus,
            interface: 0,
            int_ep: EndpointAddress::in_(3),
            config,
            idle_rate: 0,
            protocol: 1, // Report protocol by default
            report_buf: [0u8; 64],
            report_pending: false,
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

    /// Check if configured
    pub fn is_configured(&self) -> bool {
        self.configured
    }

    /// Send a HID report
    pub fn send_report(&mut self, report: &[u8]) -> UsbResult<()> {
        if !self.configured {
            return Err(UsbError::NotReady);
        }

        if report.len() > self.report_buf.len() {
            return Err(UsbError::BufferOverflow);
        }

        // Copy report to buffer
        self.report_buf[..report.len()].copy_from_slice(report);
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

    /// Flush pending report
    fn flush_report(&mut self) -> UsbResult<()> {
        if !self.report_pending {
            return Ok(());
        }

        match self
            .bus
            .write(self.int_ep, &self.report_buf[..self.config.report_size])
        {
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
        [
            9,                               // bLength
            hid_descriptor_type::HID,        // bDescriptorType (HID)
            0x11,
            0x01, // bcdHID (1.11)
            0x00, // bCountryCode
            0x01, // bNumDescriptors
            hid_descriptor_type::REPORT,     // bDescriptorType (Report)
            report_len as u8,                // wDescriptorLength (low)
            (report_len >> 8) as u8,         // wDescriptorLength (high)
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
        let len = core::cmp::min(buf.len(), self.config.report_size);
        buf[..len].copy_from_slice(&self.report_buf[..len]);
        Ok(len)
    }

    fn control_out(&mut self, data: &[u8]) -> UsbResult<()> {
        // Handle SET_REPORT (e.g., keyboard LEDs)
        #[cfg(feature = "defmt")]
        defmt::debug!("HID: Received output report: {:02x}", data);
        Ok(())
    }

    fn get_descriptors(&self, writer: &mut DescriptorWriter) -> bool {
        // HID Interface
        let iface = InterfaceDescriptor::new(
            self.interface,
            HID_CLASS,
            self.config.subclass,
            self.config.protocol,
            1, // One interrupt endpoint
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
        let ep_desc = EndpointDescriptor::new(
            self.int_ep,
            EndpointType::Interrupt,
            self.config.report_size as u16,
            self.config.poll_interval,
        );
        if !writer.write_endpoint(&ep_desc) {
            return false;
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
        self.idle_rate = 0;
        self.protocol = 1;
        #[cfg(feature = "defmt")]
        defmt::info!("HID: Reset");
    }

    fn poll(&mut self) {
        if !self.configured {
            return;
        }

        // Try to flush pending report
        let _ = self.flush_report();
    }
}
