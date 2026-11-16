//! RPi reset interface: let a host reboot the microcontroller itself.
//!
//! This is `pico_usb_reset` expressed as a [`UsbClass`]. It is a
//! vendor-specific interface with no endpoints whose only job is to accept two
//! control requests:
//!
//! * reset into BOOTSEL, so the device's own firmware can be reflashed over USB
//!   without touching a button;
//! * reset through the normal flash boot path.
//!
//! `picotool` drives both. Note that this reboots *the device running this
//! firmware*, which for a debug probe is the probe itself; resetting a debug
//! *target* is `DAP_ResetTarget` or the nRESET line, not this.
//!
//! The wire protocol is fixed by `pico/usb_reset_interface.h`: the interface
//! advertises class/subclass/protocol `0xFF/0x00/0x01`, and both requests are
//! vendor requests addressed to it.
//!
//! A device usually also needs a Microsoft OS 2.0 descriptor for Windows to bind
//! WinUSB to this interface without a driver. That is a separate concern shared
//! with the CMSIS-DAP vendor interface and is not handled here.

use crate::bus::{InterfaceNumber, StringIndex, UsbBus, UsbBusAllocator};
use crate::class::UsbClass;
use crate::control::{ControlOut, Recipient, RequestType};
use crate::descriptor::DescriptorWriter;
use crate::error::UsbResult;

/// `bInterfaceSubClass` of the reset interface.
pub const RESET_INTERFACE_SUBCLASS: u8 = 0x00;
/// `bInterfaceProtocol` of the reset interface, which distinguishes it from
/// other `0xFF/0x00` vendor functions on the same device.
pub const RESET_INTERFACE_PROTOCOL: u8 = 0x01;

/// Request: reboot into BOOTSEL (`bRequest`).
pub const RESET_REQUEST_BOOTSEL: u8 = 0x01;
/// Request: reboot through the normal flash boot path (`bRequest`).
pub const RESET_REQUEST_FLASH: u8 = 0x02;

/// Optional activity indicator for the BOOTSEL bootloader.
///
/// Mirrors the bootrom's `p1` argument plus the active-low flag.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ActivityLed {
    /// GPIO number to blink while the bootloader waits (0-127).
    pub pin: u8,
    /// `true` when the LED is wired active-low.
    pub active_low: bool,
}

/// Options carried in `wValue` of [`RESET_REQUEST_BOOTSEL`].
///
/// ```text
/// bit  0     disable the bootloader's mass-storage interface
/// bit  1     disable the bootloader's picoboot interface
/// bit  7     activity LED is active-low
/// bit  8     an activity LED GPIO is being specified
/// bits 9-15  activity LED GPIO number
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct BootSelOptions {
    /// Hide the bootloader's picoboot interface.
    pub picoboot_disabled: bool,
    /// Hide the bootloader's mass-storage interface.
    pub msd_disabled: bool,
    /// Blink this pin while the bootloader waits for a host.
    pub activity_led: Option<ActivityLed>,
}

impl BootSelOptions {
    /// Decode the `wValue` of a BOOTSEL request.
    pub const fn from_wvalue(value: u16) -> Self {
        let led_specified = value & (1 << 8) != 0;
        Self {
            msd_disabled: value & (1 << 0) != 0,
            picoboot_disabled: value & (1 << 1) != 0,
            activity_led: if led_specified {
                Some(ActivityLed {
                    pin: ((value >> 9) & 0x7F) as u8,
                    active_low: value & (1 << 7) != 0,
                })
            } else {
                None
            },
        }
    }
}

/// What the host asked the device to do.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ResetAction {
    /// Reboot into the USB bootloader.
    BootSel(BootSelOptions),
    /// Reboot through the normal flash boot path.
    Flash,
}

/// Performs the reset the host asked for.
///
/// Implementations should not return, because the reset is the whole point of
/// the request; returning simply leaves the device running.
pub trait ResetHandler {
    /// Carry out `action`.
    fn reset(&mut self, action: ResetAction);
}

/// The reset interface, parameterised by whatever performs the reboot.
pub struct ResetInterface<H: ResetHandler> {
    interface: InterfaceNumber,
    interface_string: StringIndex,
    handler: H,
}

impl<H: ResetHandler> ResetInterface<H> {
    /// Allocate the (endpoint-less) interface and take ownership of `handler`.
    pub fn new<B: UsbBus>(alloc: &UsbBusAllocator<B>, handler: H) -> Self {
        Self {
            interface: alloc.interface(),
            interface_string: alloc.string(),
            handler,
        }
    }

    /// The interface number this function was assigned.
    pub fn interface(&self) -> InterfaceNumber {
        self.interface
    }
}

impl<H: ResetHandler, B: UsbBus> UsbClass<B> for ResetInterface<H> {
    fn configuration_descriptors(&self, writer: &mut DescriptorWriter) -> UsbResult<()> {
        writer.interface_with_string(
            self.interface,
            0, // no endpoints
            0xFF,
            RESET_INTERFACE_SUBCLASS,
            RESET_INTERFACE_PROTOCOL,
            self.interface_string,
        )
    }

    fn string(&self, index: StringIndex, _lang_id: u16) -> Option<&str> {
        if index == self.interface_string {
            Some("Raspberry Pi Reset Interface")
        } else {
            None
        }
    }

    fn control_out(&mut self, xfer: ControlOut<'_, B>) {
        let req = *xfer.request();
        if req.request_type != RequestType::Vendor
            || req.recipient != Recipient::Interface
            || req.index != u16::from(u8::from(self.interface))
        {
            return;
        }

        let action = match req.request {
            RESET_REQUEST_BOOTSEL => ResetAction::BootSel(BootSelOptions::from_wvalue(req.value)),
            RESET_REQUEST_FLASH => ResetAction::Flash,
            _ => {
                let _ = xfer.reject();
                return;
            }
        };

        // Acknowledge before rebooting so the host sees the request succeed. The
        // reset itself is asynchronous: the bootrom arms the watchdog and the
        // chip goes down a few hundred milliseconds later, which is long enough
        // for the status stage to finish.
        let _ = xfer.accept();
        self.handler.reset(action);
    }
}
