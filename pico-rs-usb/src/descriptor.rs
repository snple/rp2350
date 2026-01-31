//! USB descriptor generation and handling
//!
//! This module provides types and utilities for building USB descriptors.

use crate::endpoint::{EndpointAddress, EndpointType};
use crate::types::{DescriptorType, LangId};

/// Device descriptor (18 bytes)
#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
pub struct DeviceDescriptor {
    /// Size of this descriptor (18)
    pub b_length: u8,
    /// Descriptor type (DEVICE = 1)
    pub b_descriptor_type: u8,
    /// USB specification version (BCD)
    pub bcd_usb: u16,
    /// Device class code
    pub b_device_class: u8,
    /// Device subclass code
    pub b_device_sub_class: u8,
    /// Device protocol code
    pub b_device_protocol: u8,
    /// Maximum packet size for EP0
    pub b_max_packet_size0: u8,
    /// Vendor ID
    pub id_vendor: u16,
    /// Product ID
    pub id_product: u16,
    /// Device release number (BCD)
    pub bcd_device: u16,
    /// Index of manufacturer string
    pub i_manufacturer: u8,
    /// Index of product string
    pub i_product: u8,
    /// Index of serial number string
    pub i_serial_number: u8,
    /// Number of configurations
    pub b_num_configurations: u8,
}

impl DeviceDescriptor {
    /// Size of device descriptor
    pub const SIZE: usize = 18;

    /// Create a new device descriptor
    pub const fn new(
        vendor_id: u16,
        product_id: u16,
        device_release: u16,
        class: u8,
        subclass: u8,
        protocol: u8,
    ) -> Self {
        Self {
            b_length: Self::SIZE as u8,
            b_descriptor_type: DescriptorType::Device as u8,
            bcd_usb: 0x0200, // USB 2.0
            b_device_class: class,
            b_device_sub_class: subclass,
            b_device_protocol: protocol,
            b_max_packet_size0: 64,
            id_vendor: vendor_id,
            id_product: product_id,
            bcd_device: device_release,
            i_manufacturer: 0,
            i_product: 0,
            i_serial_number: 0,
            b_num_configurations: 1,
        }
    }

    /// Convert to bytes
    pub fn to_bytes(&self) -> [u8; Self::SIZE] {
        unsafe { core::mem::transmute(*self) }
    }

    /// Set manufacturer string index
    pub fn set_manufacturer_index(&mut self, index: u8) {
        self.i_manufacturer = index;
    }

    /// Set product string index
    pub fn set_product_index(&mut self, index: u8) {
        self.i_product = index;
    }

    /// Set serial number string index
    pub fn set_serial_number_index(&mut self, index: u8) {
        self.i_serial_number = index;
    }
}

/// Configuration descriptor header (9 bytes)
#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
pub struct ConfigurationDescriptor {
    /// Size of this descriptor (9)
    pub b_length: u8,
    /// Descriptor type (CONFIGURATION = 2)
    pub b_descriptor_type: u8,
    /// Total length including all interface/endpoint descriptors
    pub w_total_length: u16,
    /// Number of interfaces
    pub b_num_interfaces: u8,
    /// Configuration value
    pub b_configuration_value: u8,
    /// Index of configuration string
    pub i_configuration: u8,
    /// Configuration attributes
    pub bm_attributes: u8,
    /// Maximum power (in 2mA units)
    pub b_max_power: u8,
}

impl ConfigurationDescriptor {
    /// Size of configuration descriptor
    pub const SIZE: usize = 9;

    /// Create a new configuration descriptor
    pub const fn new(num_interfaces: u8, config_value: u8, max_power_ma: u16) -> Self {
        Self {
            b_length: Self::SIZE as u8,
            b_descriptor_type: DescriptorType::Configuration as u8,
            w_total_length: Self::SIZE as u16, // Will be updated when building
            b_num_interfaces: num_interfaces,
            b_configuration_value: config_value,
            i_configuration: 0,
            bm_attributes: 0x80, // Bus-powered
            b_max_power: (max_power_ma / 2) as u8,
        }
    }

    /// Add self-powered attribute
    pub const fn self_powered(mut self) -> Self {
        self.bm_attributes |= 0x40;
        self
    }

    /// Add remote wakeup attribute
    pub const fn remote_wakeup(mut self) -> Self {
        self.bm_attributes |= 0x20;
        self
    }
}

/// Interface descriptor (9 bytes)
#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
pub struct InterfaceDescriptor {
    /// Size of this descriptor (9)
    pub b_length: u8,
    /// Descriptor type (INTERFACE = 4)
    pub b_descriptor_type: u8,
    /// Interface number
    pub b_interface_number: u8,
    /// Alternate setting
    pub b_alternate_setting: u8,
    /// Number of endpoints (excluding EP0)
    pub b_num_endpoints: u8,
    /// Interface class code
    pub b_interface_class: u8,
    /// Interface subclass code
    pub b_interface_sub_class: u8,
    /// Interface protocol code
    pub b_interface_protocol: u8,
    /// Index of interface string
    pub i_interface: u8,
}

impl InterfaceDescriptor {
    /// Size of interface descriptor
    pub const SIZE: usize = 9;

    /// Create a new interface descriptor
    pub const fn new(
        interface_number: u8,
        class: u8,
        subclass: u8,
        protocol: u8,
        num_endpoints: u8,
    ) -> Self {
        Self {
            b_length: Self::SIZE as u8,
            b_descriptor_type: DescriptorType::Interface as u8,
            b_interface_number: interface_number,
            b_alternate_setting: 0,
            b_num_endpoints: num_endpoints,
            b_interface_class: class,
            b_interface_sub_class: subclass,
            b_interface_protocol: protocol,
            i_interface: 0,
        }
    }
}

/// Endpoint descriptor (7 bytes)
#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
pub struct EndpointDescriptor {
    /// Size of this descriptor (7)
    pub b_length: u8,
    /// Descriptor type (ENDPOINT = 5)
    pub b_descriptor_type: u8,
    /// Endpoint address
    pub b_endpoint_address: u8,
    /// Endpoint attributes
    pub bm_attributes: u8,
    /// Maximum packet size
    pub w_max_packet_size: u16,
    /// Polling interval (for interrupt/isochronous)
    pub b_interval: u8,
}

impl EndpointDescriptor {
    /// Size of endpoint descriptor
    pub const SIZE: usize = 7;

    /// Create a new endpoint descriptor
    pub const fn new(
        address: EndpointAddress,
        ep_type: EndpointType,
        max_packet_size: u16,
        interval: u8,
    ) -> Self {
        Self {
            b_length: Self::SIZE as u8,
            b_descriptor_type: DescriptorType::Endpoint as u8,
            b_endpoint_address: address.raw(),
            bm_attributes: ep_type.to_bits(),
            w_max_packet_size: max_packet_size,
            b_interval: interval,
        }
    }
}

/// String descriptor index constants
pub mod string_index {
    /// Language IDs descriptor (index 0)
    pub const LANG_IDS: u8 = 0;
    /// Manufacturer string (index 1)
    pub const MANUFACTURER: u8 = 1;
    /// Product string (index 2)
    pub const PRODUCT: u8 = 2;
    /// Serial number string (index 3)
    pub const SERIAL_NUMBER: u8 = 3;
}

/// String descriptors collection
pub struct StringDescriptors<'a> {
    /// Language IDs
    pub lang_ids: &'a [LangId],
    /// Manufacturer string
    pub manufacturer: Option<&'a str>,
    /// Product string
    pub product: Option<&'a str>,
    /// Serial number string
    pub serial_number: Option<&'a str>,
    /// Additional strings
    pub extra: &'a [&'a str],
}

impl<'a> StringDescriptors<'a> {
    /// Create new string descriptors with default language (English US)
    pub const fn new() -> Self {
        Self {
            lang_ids: &[LangId::EnglishUS],
            manufacturer: None,
            product: None,
            serial_number: None,
            extra: &[],
        }
    }

    /// Set manufacturer string
    pub const fn manufacturer(mut self, s: &'a str) -> Self {
        self.manufacturer = Some(s);
        self
    }

    /// Set product string
    pub const fn product(mut self, s: &'a str) -> Self {
        self.product = Some(s);
        self
    }

    /// Set serial number string
    pub const fn serial_number(mut self, s: &'a str) -> Self {
        self.serial_number = Some(s);
        self
    }

    /// Get string by index
    pub fn get(&self, index: u8) -> Option<&'a str> {
        match index {
            0 => None, // Language IDs handled separately
            1 => self.manufacturer,
            2 => self.product,
            3 => self.serial_number,
            i if (i as usize) < 4 + self.extra.len() => Some(self.extra[i as usize - 4]),
            _ => None,
        }
    }

    /// Build language IDs descriptor
    pub fn build_lang_ids(&self, buf: &mut [u8]) -> usize {
        let len = 2 + self.lang_ids.len() * 2;
        if buf.len() < len {
            return 0;
        }

        buf[0] = len as u8;
        buf[1] = DescriptorType::String as u8;

        for (i, lang) in self.lang_ids.iter().enumerate() {
            let offset = 2 + i * 2;
            buf[offset..offset + 2].copy_from_slice(&(*lang as u16).to_le_bytes());
        }

        len
    }

    /// Build a string descriptor from a UTF-8 string
    pub fn build_string(s: &str, buf: &mut [u8]) -> usize {
        // Convert UTF-8 to UTF-16LE
        let mut len = 2; // Header

        for c in s.chars() {
            if len + 2 > buf.len() {
                break;
            }
            let code = c as u16;
            buf[len] = code as u8;
            buf[len + 1] = (code >> 8) as u8;
            len += 2;
        }

        buf[0] = len as u8;
        buf[1] = DescriptorType::String as u8;

        len
    }
}

impl Default for StringDescriptors<'_> {
    fn default() -> Self {
        Self::new()
    }
}

/// Descriptor writer for building composite descriptors
pub struct DescriptorWriter<'a> {
    buffer: &'a mut [u8],
    position: usize,
    interface_count: u8,
}

impl<'a> DescriptorWriter<'a> {
    /// Create a new descriptor writer
    pub fn new(buffer: &'a mut [u8]) -> Self {
        Self {
            buffer,
            position: 0,
            interface_count: 0,
        }
    }

    /// Get current position
    pub fn position(&self) -> usize {
        self.position
    }

    /// Get remaining capacity
    pub fn remaining(&self) -> usize {
        self.buffer.len() - self.position
    }

    /// Get interface count
    pub fn interface_count(&self) -> u8 {
        self.interface_count
    }

    /// Write raw bytes
    pub fn write(&mut self, data: &[u8]) -> bool {
        if data.len() > self.remaining() {
            return false;
        }
        self.buffer[self.position..self.position + data.len()].copy_from_slice(data);
        self.position += data.len();
        true
    }

    /// Write a configuration descriptor header (placeholder for total length)
    pub fn write_configuration(&mut self, config: &ConfigurationDescriptor) -> Option<usize> {
        let pos = self.position;
        if !self.write(&[
            config.b_length,
            config.b_descriptor_type,
            0,
            0, // w_total_length placeholder
            config.b_num_interfaces,
            config.b_configuration_value,
            config.i_configuration,
            config.bm_attributes,
            config.b_max_power,
        ]) {
            return None;
        }
        Some(pos)
    }

    /// Write an interface descriptor
    pub fn write_interface(&mut self, iface: &InterfaceDescriptor) -> bool {
        let result = self.write(&[
            iface.b_length,
            iface.b_descriptor_type,
            iface.b_interface_number,
            iface.b_alternate_setting,
            iface.b_num_endpoints,
            iface.b_interface_class,
            iface.b_interface_sub_class,
            iface.b_interface_protocol,
            iface.i_interface,
        ]);
        if result {
            self.interface_count += 1;
        }
        result
    }

    /// Write an endpoint descriptor
    pub fn write_endpoint(&mut self, ep: &EndpointDescriptor) -> bool {
        self.write(&[
            ep.b_length,
            ep.b_descriptor_type,
            ep.b_endpoint_address,
            ep.bm_attributes,
            ep.w_max_packet_size as u8,
            (ep.w_max_packet_size >> 8) as u8,
            ep.b_interval,
        ])
    }

    /// Finalize configuration descriptor (update w_total_length and b_num_interfaces)
    pub fn finalize_configuration(&mut self, config_pos: usize) {
        let total_len = self.position - config_pos;
        // Update wTotalLength (bytes 2-3)
        self.buffer[config_pos + 2] = total_len as u8;
        self.buffer[config_pos + 3] = (total_len >> 8) as u8;
        // Update bNumInterfaces (byte 4)
        self.buffer[config_pos + 4] = self.interface_count;
    }

    /// Get the written data
    pub fn into_bytes(self) -> &'a [u8] {
        &self.buffer[..self.position]
    }
}

/// USB class codes
pub mod class_code {
    /// Use class information in Interface Descriptors
    pub const MISC: u8 = 0xEF;
    /// Communications and CDC Control
    pub const CDC: u8 = 0x02;
    /// CDC Data
    pub const CDC_DATA: u8 = 0x0A;
    /// Human Interface Device
    pub const HID: u8 = 0x03;
    /// Mass Storage
    pub const MASS_STORAGE: u8 = 0x08;
    /// Vendor Specific
    pub const VENDOR: u8 = 0xFF;
}

/// CDC subclass codes
pub mod cdc_subclass {
    /// Abstract Control Model
    pub const ACM: u8 = 0x02;
}

/// CDC protocol codes
pub mod cdc_protocol {
    /// No class specific protocol required
    pub const NONE: u8 = 0x00;
    /// AT Commands
    pub const AT: u8 = 0x01;
}

/// CDC functional descriptor types
pub mod cdc_descriptor {
    /// Header Functional Descriptor
    pub const HEADER: u8 = 0x00;
    /// Call Management Functional Descriptor
    pub const CALL_MANAGEMENT: u8 = 0x01;
    /// Abstract Control Management Functional Descriptor
    pub const ACM: u8 = 0x02;
    /// Union Functional Descriptor
    pub const UNION: u8 = 0x06;
}
