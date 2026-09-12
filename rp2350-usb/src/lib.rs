//! # rp2350-usb
//!
//! USB Device Stack for RP2350 microcontrollers (Raspberry Pi Pico 2).
//!
//! ## Quick start
//!
//! ```ignore
//! use rp2350_usb::{UsbBus, UsbBusAllocator, UsbDeviceBuilder};
//! use rp2350_usb::class::cdc_acm::CdcAcm;
//! use rp2350_usb::rp2350::Rp2350Bus;
//!
//! let alloc = UsbBusAllocator::new(unsafe { Rp2350Bus::steal() });
//! let mut serial = CdcAcm::new(&alloc);
//! let mut dev = UsbDeviceBuilder::new(&alloc, 0x1234, 0x5678)
//!     .product("My Device")
//!     .build();
//!
//! loop {
//!     if dev.poll(&mut [&mut serial]) {
//!         let mut buf = [0u8; 64];
//!         if let Ok(n) = serial.read(&mut buf) {
//!             let _ = serial.write(&buf[..n]);
//!         }
//!     }
//! }
//! ```

#![no_std]

// ── Modules ──────────────────────────────────────────────────────────────────

pub mod bus;
pub mod class;
pub mod control;
pub mod descriptor;
pub mod device;
pub mod dpram;
pub mod endpoint;
pub mod ep_regs;
pub mod error;
pub mod rp2350;

// ── Re-exports ────────────────────────────────────────────────────────────────

pub use bus::{InterfaceNumber, PollResult, StringIndex, UsbBus, UsbBusAllocator};
pub use class::UsbClass;
pub use descriptor::DescriptorWriter;
pub use device::{DeviceInfo, DeviceState, UsbDevice, UsbDeviceBuilder};
pub use endpoint::{
    Direction, EndpointAddress, EndpointIn, EndpointOut, EndpointType, In, IsoSynchronization,
    IsoUsage, Out,
};
pub use error::{UsbError, UsbResult};
pub use rp2350::Rp2350Bus;

// ── Crate-level constants ─────────────────────────────────────────────────────

/// Maximum number of endpoints (EP0–EP15, IN and OUT).
pub const MAX_ENDPOINTS: usize = 16;

/// Maximum packet size for EP0 (full-speed USB 2.0).
pub const CONTROL_MAX_PACKET_SIZE: u16 = 64;

/// Maximum packet size for bulk/interrupt endpoints (full-speed).
pub const BULK_MAX_PACKET_SIZE: u16 = 64;

/// USB DPRAM size in bytes.
pub const DPRAM_SIZE: usize = 4096;
