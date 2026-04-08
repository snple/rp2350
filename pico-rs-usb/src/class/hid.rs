//! USB HID (Human Interface Device) class.
//!
//! Supports keyboards, mice, and custom HID devices using a report-descriptor
//! approach.  Pass a `&'static [u8]` report descriptor via [`HidConfig`].
//!
//! # Example — keyboard
//!
//! ```ignore
//! use pico_rs_usb::{UsbBusAllocator, UsbDeviceBuilder};
//! use pico_rs_usb::class::hid::{HidClass, HidConfig, KEYBOARD_REPORT_DESCRIPTOR, KeyboardReport};
//! use pico_rs_usb::rp2350::Rp2350Bus;
//!
//! let alloc = UsbBusAllocator::new(Rp2350Bus::new());
//! let mut kbd = HidClass::keyboard(&alloc);
//! let mut dev = UsbDeviceBuilder::new(&alloc, 0x1234, 0x5678)
//!     .product("Keyboard")
//!     .build();
//!
//! loop {
//!     dev.poll(&mut [&mut kbd]);
//!     let report = KeyboardReport::default();
//!     let _ = kbd.send_report(&report.to_bytes());
//! }
//! ```

use crate::bus::{InterfaceNumber, UsbBus, UsbBusAllocator};
use crate::class::UsbClass;
use crate::control::{ControlIn, ControlOut, Recipient, RequestType, StandardRequest};
use crate::descriptor::{DescriptorWriter, class_code};
use crate::endpoint::{EndpointAddress, EndpointIn, EndpointOut};
use crate::error::{UsbError, UsbResult};

// ─── HID constants ────────────────────────────────────────────────────────────

pub mod hid_subclass {
    pub const NONE: u8 = 0x00;
    pub const BOOT: u8 = 0x01;
}

pub mod hid_protocol {
    pub const NONE: u8 = 0x00;
    pub const KEYBOARD: u8 = 0x01;
    pub const MOUSE: u8 = 0x02;
}

pub mod hid_descriptor_type {
    pub const HID: u8 = 0x21;
    pub const REPORT: u8 = 0x22;
    pub const PHYSICAL: u8 = 0x23;
}

mod hid_req {
    pub const GET_REPORT: u8 = 0x01;
    pub const GET_IDLE: u8 = 0x02;
    pub const GET_PROTOCOL: u8 = 0x03;
    pub const SET_REPORT: u8 = 0x09;
    pub const SET_IDLE: u8 = 0x0A;
    pub const SET_PROTOCOL: u8 = 0x0B;
}

// ─── Standard report descriptors ─────────────────────────────────────────────

/// Boot-compatible keyboard report descriptor (8-byte input, 1-byte LED output).
pub const KEYBOARD_REPORT_DESCRIPTOR: &[u8] = &[
    0x05, 0x01, // Usage Page (Generic Desktop)
    0x09, 0x06, // Usage (Keyboard)
    0xA1, 0x01, // Collection (Application)
    // Modifier keys (8 bits)
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
    // LED output (5 bits + 3 padding)
    0x05, 0x08, //   Usage Page (LEDs)
    0x19, 0x01, //   Usage Minimum (Num Lock)
    0x29, 0x05, //   Usage Maximum (Kana)
    0x75, 0x01, //   Report Size (1)
    0x95, 0x05, //   Report Count (5)
    0x91, 0x02, //   Output (Data, Variable, Absolute)
    0x75, 0x03, //   Report Size (3)
    0x95, 0x01, //   Report Count (1)
    0x91, 0x01, //   Output (Constant)
    // Key array (6 keys)
    0x05, 0x07, //   Usage Page (Keyboard/Keypad)
    0x19, 0x00, //   Usage Minimum (0)
    0x29, 0xFF, //   Usage Maximum (255)
    0x15, 0x00, //   Logical Minimum (0)
    0x26, 0xFF, 0x00, //   Logical Maximum (255)
    0x75, 0x08, //   Report Size (8)
    0x95, 0x06, //   Report Count (6)
    0x81, 0x00, //   Input (Data, Array)
    0xC0, // End Collection
];

/// Standard 3-button + wheel mouse report descriptor.
pub const MOUSE_REPORT_DESCRIPTOR: &[u8] = &[
    0x05, 0x01, // Usage Page (Generic Desktop)
    0x09, 0x02, // Usage (Mouse)
    0xA1, 0x01, // Collection (Application)
    0x09, 0x01, //   Usage (Pointer)
    0xA1, 0x00, //   Collection (Physical)
    // Buttons (3 bits + 5 padding)
    0x05, 0x09, //     Usage Page (Buttons)
    0x19, 0x01, //     Usage Minimum (Button 1)
    0x29, 0x03, //     Usage Maximum (Button 3)
    0x15, 0x00, //     Logical Minimum (0)
    0x25, 0x01, //     Logical Maximum (1)
    0x75, 0x01, //     Report Size (1)
    0x95, 0x03, //     Report Count (3)
    0x81, 0x02, //     Input (Data, Variable, Absolute)
    0x75, 0x05, //     Report Size (5) – padding
    0x95, 0x01, //     Report Count (1)
    0x81, 0x01, //     Input (Constant)
    // X, Y relative movement
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

// ─── Report structs ───────────────────────────────────────────────────────────

/// 8-byte boot keyboard report.
#[derive(Clone, Copy, Debug, Default)]
pub struct KeyboardReport {
    /// Modifier bits (see [`modifier`]).
    pub modifiers: u8,
    _reserved: u8,
    /// Up to 6 simultaneous keycodes.
    pub keys: [u8; 6],
}

impl KeyboardReport {
    pub fn to_bytes(self) -> [u8; 8] {
        [
            self.modifiers,
            0,
            self.keys[0],
            self.keys[1],
            self.keys[2],
            self.keys[3],
            self.keys[4],
            self.keys[5],
        ]
    }
}

/// Modifier key bitmask constants.
pub mod modifier {
    pub const LEFT_CTRL: u8 = 1 << 0;
    pub const LEFT_SHIFT: u8 = 1 << 1;
    pub const LEFT_ALT: u8 = 1 << 2;
    pub const LEFT_GUI: u8 = 1 << 3;
    pub const RIGHT_CTRL: u8 = 1 << 4;
    pub const RIGHT_SHIFT: u8 = 1 << 5;
    pub const RIGHT_ALT: u8 = 1 << 6;
    pub const RIGHT_GUI: u8 = 1 << 7;
}

/// 4-byte mouse report: buttons, X, Y, wheel.
#[derive(Clone, Copy, Debug, Default)]
pub struct MouseReport {
    pub buttons: u8,
    pub x: i8,
    pub y: i8,
    pub wheel: i8,
}

impl MouseReport {
    pub const BUTTON_LEFT: u8 = 0x01;
    pub const BUTTON_RIGHT: u8 = 0x02;
    pub const BUTTON_MIDDLE: u8 = 0x04;

    pub fn to_bytes(self) -> [u8; 4] {
        [self.buttons, self.x as u8, self.y as u8, self.wheel as u8]
    }
}

// ─── HidConfig ────────────────────────────────────────────────────────────────

/// Configuration for [`HidClass`].
pub struct HidConfig {
    /// HID subclass (boot = 1 for boot-compatible devices).
    pub subclass: u8,
    /// HID protocol (keyboard = 1, mouse = 2, custom = 0).
    pub protocol: u8,
    /// Report descriptor.  Must be `'static` so it can be returned by reference.
    pub report_descriptor: &'static [u8],
    /// Maximum input report size (≤ 64 bytes for full-speed).
    pub in_report_size: u16,
    /// `true` to allocate an interrupt OUT endpoint for output/feature reports.
    pub use_out_endpoint: bool,
    /// Polling interval in ms for the interrupt IN endpoint.
    pub poll_interval_ms: u8,
}

impl HidConfig {
    pub fn keyboard() -> Self {
        Self {
            subclass: hid_subclass::BOOT,
            protocol: hid_protocol::KEYBOARD,
            report_descriptor: KEYBOARD_REPORT_DESCRIPTOR,
            in_report_size: 8u16,
            use_out_endpoint: true, // LED output report
            poll_interval_ms: 10,
        }
    }

    pub fn mouse() -> Self {
        Self {
            subclass: hid_subclass::BOOT,
            protocol: hid_protocol::MOUSE,
            report_descriptor: MOUSE_REPORT_DESCRIPTOR,
            in_report_size: 4u16,
            use_out_endpoint: false,
            poll_interval_ms: 8,
        }
    }

    pub fn custom(report_descriptor: &'static [u8], in_report_size: u16) -> Self {
        Self {
            subclass: hid_subclass::NONE,
            protocol: hid_protocol::NONE,
            report_descriptor,
            in_report_size,
            use_out_endpoint: false,
            poll_interval_ms: 10,
        }
    }
}

// ─── HidClass ────────────────────────────────────────────────────────────────

/// USB HID class.
pub struct HidClass<'a, B: UsbBus> {
    interface: InterfaceNumber,
    int_in: EndpointIn<'a, B>,
    int_out: Option<EndpointOut<'a, B>>,
    config: HidConfig,
    idle_rate: u8,
    protocol: u8,
    configured: bool,
    // Buffered output report from SET_REPORT or from the OUT endpoint.
    out_report: [u8; 64],
    out_report_len: usize,
    out_report_available: bool,
}

impl<'a, B: UsbBus> HidClass<'a, B> {
    /// Create an HID class from a custom `HidConfig`.
    pub fn new(alloc: &'a UsbBusAllocator<B>, config: HidConfig) -> Self {
        let interface = alloc.interface();
        let int_in = alloc.interrupt(config.in_report_size, config.poll_interval_ms);
        let int_out = if config.use_out_endpoint {
            Some(alloc.interrupt::<crate::endpoint::Out>(64, config.poll_interval_ms))
        } else {
            None
        };
        let protocol = if config.subclass == hid_subclass::BOOT {
            1
        } else {
            0
        };
        Self {
            interface,
            int_in,
            int_out,
            config,
            idle_rate: 0,
            protocol,
            configured: false,
            out_report: [0; 64],
            out_report_len: 0,
            out_report_available: false,
        }
    }

    /// Create a boot-compatible keyboard HID device.
    pub fn keyboard(alloc: &'a UsbBusAllocator<B>) -> Self {
        Self::new(alloc, HidConfig::keyboard())
    }

    /// Create a boot-compatible mouse HID device.
    pub fn mouse(alloc: &'a UsbBusAllocator<B>) -> Self {
        Self::new(alloc, HidConfig::mouse())
    }

    // ── Public API ────────────────────────────────────────────────────────────

    /// Send an input report to the host.
    ///
    /// Returns [`UsbError::WouldBlock`] when the previous report has not yet
    /// been sent (call again when [`endpoint_in_complete`] fires).
    pub fn send_report(&self, report: &[u8]) -> UsbResult<()> {
        if !self.configured {
            return Err(UsbError::InvalidState);
        }
        self.int_in.write(report).map(|_| ())
    }

    /// Send a keyboard report.
    pub fn send_keyboard_report(&self, report: &KeyboardReport) -> UsbResult<()> {
        self.send_report(&report.to_bytes())
    }

    /// Send a mouse report.
    pub fn send_mouse_report(&self, report: &MouseReport) -> UsbResult<()> {
        self.send_report(&report.to_bytes())
    }

    /// Read a received output or feature report (e.g., keyboard LED state).
    ///
    /// Returns `Some(len)` if a report is available, `None` otherwise.
    pub fn read_output_report(&mut self, buf: &mut [u8]) -> Option<usize> {
        if !self.out_report_available {
            return None;
        }
        self.out_report_available = false;
        let n = core::cmp::min(self.out_report_len, buf.len());
        buf[..n].copy_from_slice(&self.out_report[..n]);
        Some(n)
    }

    /// `true` if fully configured by the host.
    pub fn is_configured(&self) -> bool {
        self.configured
    }

    /// Current HID protocol (0 = boot, 1 = report).
    pub fn protocol(&self) -> u8 {
        self.protocol
    }
}

// ─── UsbClass<B> implementation ───────────────────────────────────────────────

impl<B: UsbBus> UsbClass<B> for HidClass<'_, B> {
    fn get_configuration_descriptors(&self, w: &mut DescriptorWriter) -> UsbResult<()> {
        let num_eps: u8 = 1 + self.int_out.is_some() as u8;
        w.interface(
            self.interface,
            num_eps,
            class_code::HID,
            self.config.subclass,
            self.config.protocol,
        )?;

        // HID descriptor.
        let rdesc_len = self.config.report_descriptor.len() as u16;
        w.write(
            hid_descriptor_type::HID,
            &[
                0x11,
                0x01, // bcdHID 1.11
                0x00, // bCountryCode (not localised)
                0x01, // bNumDescriptors
                hid_descriptor_type::REPORT,
                rdesc_len as u8,
                (rdesc_len >> 8) as u8,
            ],
        )?;

        // Interrupt IN endpoint.
        w.endpoint(
            self.int_in.address(),
            self.int_in.ep_type(),
            self.int_in.max_packet_size(),
            self.config.poll_interval_ms,
        )?;

        // Optional interrupt OUT endpoint.
        if let Some(ref ep) = self.int_out {
            w.endpoint(
                ep.address(),
                ep.ep_type(),
                ep.max_packet_size(),
                self.config.poll_interval_ms,
            )?;
        }

        Ok(())
    }

    fn control_in(&mut self, xfer: ControlIn<'_, B>) {
        let req = xfer.request();

        if req.request_type != RequestType::Standard && req.request_type != RequestType::Class {
            return;
        }
        if req.recipient != Recipient::Interface {
            return;
        }
        if req.index as u8 != u8::from(self.interface) {
            return;
        }

        match req.request_type {
            RequestType::Standard => {
                // GET_DESCRIPTOR for the HID report descriptor.
                if req.request == StandardRequest::GET_DESCRIPTOR {
                    let (desc_type, _) = req.descriptor_type_index();
                    if desc_type == hid_descriptor_type::REPORT {
                        let rdesc = self.config.report_descriptor;
                        let len = core::cmp::min(rdesc.len(), req.length as usize);
                        let _ = xfer.accept_with(&rdesc[..len]);
                    }
                }
            }
            RequestType::Class => {
                match req.request {
                    hid_req::GET_REPORT => {
                        // Return a zero-length report (no stored report).
                        let n = core::cmp::min(self.config.in_report_size, req.length) as usize;
                        let buf = [0u8; 64];
                        let _ = xfer.accept_with(&buf[..n]);
                    }
                    hid_req::GET_IDLE => {
                        let _ = xfer.accept_with(&[self.idle_rate]);
                    }
                    hid_req::GET_PROTOCOL => {
                        let _ = xfer.accept_with(&[self.protocol]);
                    }
                    _ => {
                        let _ = xfer.reject();
                    }
                }
            }
            _ => {}
        }
    }

    fn control_out(&mut self, xfer: ControlOut<'_, B>) {
        let req = xfer.request();

        if req.request_type != RequestType::Class || req.recipient != Recipient::Interface {
            return;
        }
        if req.index as u8 != u8::from(self.interface) {
            return;
        }

        match req.request {
            hid_req::SET_REPORT => {
                let data = xfer.data();
                let n = core::cmp::min(data.len(), 64);
                self.out_report[..n].copy_from_slice(&data[..n]);
                self.out_report_len = n;
                self.out_report_available = true;
                let _ = xfer.accept();
            }
            hid_req::SET_IDLE => {
                self.idle_rate = (req.value >> 8) as u8;
                let _ = xfer.accept();
            }
            hid_req::SET_PROTOCOL => {
                self.protocol = req.value as u8;
                let _ = xfer.accept();
            }
            _ => {
                let _ = xfer.reject();
            }
        }
    }

    fn endpoint_out(&mut self, addr: EndpointAddress) {
        if let Some(ep) = &self.int_out {
            if ep.address() != addr {
                return;
            }
            let mut buf = [0u8; 64];
            if let Ok(n) = ep.read(&mut buf) {
                self.out_report[..n].copy_from_slice(&buf[..n]);
                self.out_report_len = n;
                self.out_report_available = true;
            }
        }
    }

    fn configure(&mut self) {
        self.configured = true;
    }

    fn deconfigure(&mut self) {
        self.configured = false;
    }

    fn reset(&mut self) {
        self.configured = false;
        self.out_report_available = false;
        self.out_report_len = 0;
        self.protocol = if self.config.subclass == hid_subclass::BOOT {
            1
        } else {
            0
        };
        self.idle_rate = 0;
    }
}
