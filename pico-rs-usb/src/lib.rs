//! # pico-rs-usb
//!
//! USB Device Stack for RP2350 microcontrollers (Raspberry Pi Pico 2)
//!
//! This crate provides a clean, ergonomic USB device implementation for the RP2350.
//!
//! ## Features
//!
//! - Simple, builder-pattern API for device configuration
//! - Support for multiple USB classes (CDC, HID, etc.)
//! - Efficient interrupt-driven operation
//! - Zero-copy data transfers where possible
//! - defmt logging support
//!
//! ## Architecture
//!
//! The USB stack is organized in layers:
//! - **Bus Layer**: Low-level hardware abstraction (`UsbBus`)
//! - **Device Layer**: USB device state machine and control transfers (`UsbDevice`)
//! - **Class Layer**: USB class implementations (CDC, HID, etc.)
//!
//! ## Example
//!
//! ```ignore
//! use pico_rs_usb::{UsbDevice, UsbBus};
//! use pico_rs_usb::class::cdc::{CdcAcm, CdcAcmClass};
//! use pico_rs_usb::descriptor::StringDescriptors;
//!
//! // Create USB bus
//! let usb_bus = UsbBus::new();
//!
//! // Create CDC ACM class (serial port)
//! let mut serial = CdcAcmClass::new(&usb_bus);
//!
//! // Build USB device
//! let mut usb_dev = UsbDevice::builder(&usb_bus)
//!     .vendor_id(0x1234)
//!     .product_id(0x5678)
//!     .manufacturer("My Company")
//!     .product("My USB Device")
//!     .serial_number("001")
//!     .build();
//!
//! // In main loop or interrupt handler
//! loop {
//!     if usb_dev.poll(&mut [&mut serial]) {
//!         // Handle USB events
//!         let mut buf = [0u8; 64];
//!         if let Ok(count) = serial.read(&mut buf) {
//!             // Echo back received data
//!             let _ = serial.write(&buf[..count]);
//!         }
//!     }
//! }
//! ```

#![no_std]
#![warn(missing_docs)]

pub mod bus;
pub mod class;
pub mod control;
pub mod descriptor;
pub mod device;
pub mod endpoint;
pub mod error;
pub mod types;

// Re-exports for convenience
pub use bus::UsbBus;
pub use device::{UsbDevice, UsbDeviceBuilder};
pub use endpoint::{EndpointAddress, EndpointConfig, EndpointDirection, EndpointType};
pub use error::{UsbError, UsbResult};
pub use types::*;

/// Maximum number of endpoints supported by RP2350 USB controller
/// (16 endpoints: EP0-EP15, each can be IN or OUT)
pub const MAX_ENDPOINTS: usize = 16;

/// USB Full-Speed maximum packet size for control endpoints
pub const CONTROL_MAX_PACKET_SIZE: u16 = 64;

/// USB Full-Speed maximum packet size for bulk/interrupt endpoints
pub const BULK_MAX_PACKET_SIZE: u16 = 64;

/// USB Full-Speed maximum packet size for isochronous endpoints
pub const ISO_MAX_PACKET_SIZE: u16 = 1023;

/// Size of the USB DPRAM buffer (4KB)
pub const DPRAM_SIZE: usize = 4096;

/// Size of the SETUP packet buffer
pub const SETUP_PACKET_SIZE: usize = 8;
