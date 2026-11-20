//! [`UsbClass`] trait and USB class implementations.
//!
//! The generic classes ([`UsbClass`], [`cdc_acm`], [`gs_usb`], [`hid`],
//! [`i2c_tiny_usb`], [`uac`]) live in the shared [`usb_stack`] crate; the
//! `reset` interface below is RP2350-specific.

pub use usb_stack::class::*;

pub mod reset;
