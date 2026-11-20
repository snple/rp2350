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

/// Logging facade (re-exported from [`usb_stack`]).
pub use usb_stack::log;

// ── Shared stack ──────────────────────────────────────────────────────────────
//
// Everything that is MCU-independent lives in `usb-stack`; it is re-exported
// here so `rp2350_usb::class::…`, `rp2350_usb::bus::…`, … keep working.

pub use usb_stack::{bus, control, descriptor, device, endpoint, error};

// Local module: re-exports the shared classes and adds the RP2350-only ones.
pub mod class;

pub use usb_stack::{
    BULK_MAX_PACKET_SIZE, CONTROL_MAX_PACKET_SIZE, DescriptorWriter, DeviceInfo, DeviceState,
    Direction, EndpointAddress, EndpointIn, EndpointOut, EndpointType, In, InterfaceNumber,
    IsoSynchronization, IsoUsage, Out, PollResult, StringIndex, UsbBus, UsbBusAllocator, UsbClass,
    UsbDevice, UsbDeviceBuilder, UsbError, UsbResult,
};

// ── RP2350 implementation ─────────────────────────────────────────────────────

pub mod dpram;
pub mod ep_regs;
pub mod rp2350;

pub use rp2350::Rp2350Bus;

/// Number of endpoints supported by the RP2350 USB controller (EP0–EP15).
pub const MAX_ENDPOINTS: usize = 16;

/// USB DPRAM size in bytes.
pub const DPRAM_SIZE: usize = 4096;
