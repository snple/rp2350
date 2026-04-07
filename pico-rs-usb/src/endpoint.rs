//! USB endpoint management
//!
//! This module provides types and utilities for managing USB endpoints.

use crate::error::{UsbError, UsbResult};

/// Endpoint direction
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EndpointDirection {
    /// OUT endpoint (host to device)
    Out,
    /// IN endpoint (device to host)
    In,
}

impl EndpointDirection {
    /// Returns true if this is an IN endpoint
    pub const fn is_in(&self) -> bool {
        matches!(self, EndpointDirection::In)
    }

    /// Returns true if this is an OUT endpoint
    pub const fn is_out(&self) -> bool {
        matches!(self, EndpointDirection::Out)
    }
}

/// Endpoint transfer type
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EndpointType {
    /// Control endpoint
    Control,
    /// Isochronous endpoint
    Isochronous,
    /// Bulk endpoint
    Bulk,
    /// Interrupt endpoint
    Interrupt,
}

impl EndpointType {
    /// Convert to bmAttributes bits
    pub const fn to_bits(&self) -> u8 {
        match self {
            EndpointType::Control => 0x00,
            EndpointType::Isochronous => 0x01,
            EndpointType::Bulk => 0x02,
            EndpointType::Interrupt => 0x03,
        }
    }
}

/// USB endpoint address
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct EndpointAddress(u8);

impl EndpointAddress {
    /// Create a new endpoint address
    pub const fn new(number: u8, direction: EndpointDirection) -> Self {
        let dir_bit = match direction {
            EndpointDirection::Out => 0x00,
            EndpointDirection::In => 0x80,
        };
        Self(number | dir_bit)
    }

    /// Create an OUT endpoint address
    pub const fn out(number: u8) -> Self {
        Self(number & 0x0f)
    }

    /// Create an IN endpoint address
    pub const fn in_(number: u8) -> Self {
        Self((number & 0x0f) | 0x80)
    }

    /// Get the raw address byte
    pub const fn raw(&self) -> u8 {
        self.0
    }

    /// Get the endpoint number (0-15)
    pub const fn number(&self) -> u8 {
        self.0 & 0x0f
    }

    /// Get the endpoint direction
    pub const fn direction(&self) -> EndpointDirection {
        if self.0 & 0x80 != 0 {
            EndpointDirection::In
        } else {
            EndpointDirection::Out
        }
    }

    /// Returns true if this is control endpoint 0
    pub const fn is_control(&self) -> bool {
        self.number() == 0
    }

    /// Calculate the index for this endpoint in arrays (0-31)
    /// OUT endpoints: 0-15, IN endpoints: 16-31
    pub const fn index(&self) -> usize {
        let num = self.number() as usize;
        if self.0 & 0x80 != 0 {
            num + 16 // IN endpoints
        } else {
            num // OUT endpoints
        }
    }
}

impl core::fmt::Debug for EndpointAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "EP{} {}",
            self.number(),
            if self.direction().is_in() {
                "IN"
            } else {
                "OUT"
            }
        )
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for EndpointAddress {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EP{} {}",
            self.number(),
            if self.direction().is_in() {
                "IN"
            } else {
                "OUT"
            }
        )
    }
}

impl From<u8> for EndpointAddress {
    fn from(val: u8) -> Self {
        Self(val)
    }
}

impl From<EndpointAddress> for u8 {
    fn from(val: EndpointAddress) -> Self {
        val.0
    }
}

/// Endpoint configuration
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct EndpointConfig {
    /// Endpoint type
    pub ep_type: EndpointType,
    /// Maximum packet size
    pub max_packet_size: u16,
    /// Polling interval (for interrupt/isochronous endpoints)
    pub interval: u8,
}

impl EndpointConfig {
    /// Create a control endpoint configuration
    pub const fn control(max_packet_size: u16) -> Self {
        Self {
            ep_type: EndpointType::Control,
            max_packet_size,
            interval: 0,
        }
    }

    /// Create a bulk endpoint configuration
    pub const fn bulk(max_packet_size: u16) -> Self {
        Self {
            ep_type: EndpointType::Bulk,
            max_packet_size,
            interval: 0,
        }
    }

    /// Create an interrupt endpoint configuration
    pub const fn interrupt(max_packet_size: u16, interval: u8) -> Self {
        Self {
            ep_type: EndpointType::Interrupt,
            max_packet_size,
            interval,
        }
    }

    /// Create an isochronous endpoint configuration
    pub const fn isochronous(max_packet_size: u16, interval: u8) -> Self {
        Self {
            ep_type: EndpointType::Isochronous,
            max_packet_size,
            interval,
        }
    }
}

/// State of an endpoint
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EndpointState {
    /// Endpoint is not allocated
    #[default]
    Disabled,
    /// Endpoint is allocated but idle
    Idle,
    /// Data transfer in progress
    DataIn,
    /// Data transfer in progress
    DataOut,
    /// Endpoint is stalled
    Stalled,
}

/// Endpoint buffer information
#[derive(Clone, Copy, Debug)]
pub struct EndpointBuffer {
    /// Offset in DPRAM
    pub offset: u16,
    /// Buffer size
    pub size: u16,
}

impl EndpointBuffer {
    /// Create a new endpoint buffer
    pub const fn new(offset: u16, size: u16) -> Self {
        Self { offset, size }
    }

    /// Get the DPRAM address for this buffer
    pub const fn dpram_address(&self) -> u32 {
        pico_rs_pac::usb_dpram::BASE + self.offset as u32
    }
}

/// Endpoint descriptor for the device
pub struct Endpoint {
    /// Endpoint address
    pub address: EndpointAddress,
    /// Endpoint configuration
    pub config: EndpointConfig,
    /// Endpoint state
    pub state: EndpointState,
    /// Buffer for this endpoint
    pub buffer: Option<EndpointBuffer>,
    /// Current DATA toggle (DATA0/DATA1)
    pub data_toggle: bool,
}

impl Endpoint {
    /// Create a new endpoint
    pub const fn new(address: EndpointAddress, config: EndpointConfig) -> Self {
        Self {
            address,
            config,
            state: EndpointState::Disabled,
            buffer: None,
            data_toggle: false,
        }
    }

    /// Reset the data toggle to DATA0
    pub fn reset_data_toggle(&mut self) {
        self.data_toggle = false;
    }

    /// Toggle the data toggle
    pub fn toggle_data(&mut self) {
        self.data_toggle = !self.data_toggle;
    }

    /// Check if endpoint is stalled
    pub fn is_stalled(&self) -> bool {
        self.state == EndpointState::Stalled
    }

    /// Set endpoint state to stalled
    pub fn stall(&mut self) {
        self.state = EndpointState::Stalled;
    }

    /// Clear stall condition
    pub fn unstall(&mut self) {
        if self.state == EndpointState::Stalled {
            self.state = EndpointState::Idle;
            self.reset_data_toggle();
        }
    }
}

/// Endpoint allocator for managing DPRAM buffer space
pub struct EndpointAllocator {
    /// Next available offset in DPRAM
    next_offset: u16,
    /// Total available DPRAM size
    total_size: u16,
}

impl EndpointAllocator {
    /// First byte in DPRAM available for endpoint payload buffers.
    const EP_BUFFER_OFFSET: u16 = 0x100;

    /// EP0 uses dedicated fixed IN/OUT buffers at the start of endpoint DPRAM.
    const RESERVED_EP0_BYTES: u16 = crate::CONTROL_MAX_PACKET_SIZE * 2;

    /// First offset available for dynamically allocated non-EP0 endpoint buffers.
    const FIRST_DYNAMIC_OFFSET: u16 = Self::EP_BUFFER_OFFSET + Self::RESERVED_EP0_BYTES;

    /// Create a new endpoint allocator
    pub const fn new() -> Self {
        Self {
            // Reserve EP0 IN/OUT buffers so dynamically allocated endpoints do
            // not overlap control transfer memory.
            next_offset: Self::FIRST_DYNAMIC_OFFSET,
            total_size: crate::DPRAM_SIZE as u16,
        }
    }

    /// Reset the allocator
    pub fn reset(&mut self) {
        self.next_offset = Self::FIRST_DYNAMIC_OFFSET;
    }

    /// Allocate buffer space for an endpoint
    pub fn allocate(&mut self, size: u16) -> UsbResult<EndpointBuffer> {
        // Align to 64 bytes for best performance
        let aligned_offset = (self.next_offset + 63) & !63;
        let aligned_size = (size + 63) & !63;

        if aligned_offset + aligned_size > self.total_size {
            return Err(UsbError::BufferMemoryExhausted);
        }

        let buffer = EndpointBuffer::new(aligned_offset, size);
        self.next_offset = aligned_offset + aligned_size;

        Ok(buffer)
    }

    /// Get remaining buffer space
    pub fn remaining(&self) -> u16 {
        self.total_size.saturating_sub(self.next_offset)
    }
}

impl Default for EndpointAllocator {
    fn default() -> Self {
        Self::new()
    }
}
