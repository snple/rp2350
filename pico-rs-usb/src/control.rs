//! USB Control transfer handling
//!
//! This module handles USB control requests and responses.

use crate::error::{UsbError, UsbResult};
use crate::types::{DescriptorType, Direction, Recipient, RequestType, StandardRequest};

/// Parsed USB Setup packet
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SetupPacket {
    /// Request type and direction
    pub bm_request_type: u8,
    /// Request code
    pub b_request: u8,
    /// Request-specific value
    pub w_value: u16,
    /// Request-specific index
    pub w_index: u16,
    /// Length of data stage
    pub w_length: u16,
}

impl SetupPacket {
    /// Parse a setup packet from raw bytes
    pub fn parse(data: &[u8; 8]) -> Self {
        Self {
            bm_request_type: data[0],
            b_request: data[1],
            w_value: u16::from_le_bytes([data[2], data[3]]),
            w_index: u16::from_le_bytes([data[4], data[5]]),
            w_length: u16::from_le_bytes([data[6], data[7]]),
        }
    }

    /// Get the request direction
    pub fn direction(&self) -> Direction {
        Direction::from(self.bm_request_type)
    }

    /// Get the request type
    pub fn request_type(&self) -> RequestType {
        RequestType::from(self.bm_request_type)
    }

    /// Get the request recipient
    pub fn recipient(&self) -> Recipient {
        Recipient::from(self.bm_request_type)
    }

    /// Check if this is a standard request
    pub fn is_standard(&self) -> bool {
        self.request_type() == RequestType::Standard
    }

    /// Check if this is a class request
    pub fn is_class(&self) -> bool {
        self.request_type() == RequestType::Class
    }

    /// Check if this is a vendor request
    pub fn is_vendor(&self) -> bool {
        self.request_type() == RequestType::Vendor
    }

    /// Get as standard request (if applicable)
    pub fn as_standard_request(&self) -> Option<StandardRequest> {
        if self.is_standard() {
            StandardRequest::try_from(self.b_request).ok()
        } else {
            None
        }
    }

    /// Get the descriptor type from wValue (high byte)
    pub fn descriptor_type(&self) -> Option<DescriptorType> {
        DescriptorType::try_from((self.w_value >> 8) as u8).ok()
    }

    /// Get the descriptor index from wValue (low byte)
    pub fn descriptor_index(&self) -> u8 {
        (self.w_value & 0xff) as u8
    }

    /// Check if this is a GET_DESCRIPTOR request
    pub fn is_get_descriptor(&self) -> bool {
        self.is_standard()
            && self.b_request == StandardRequest::GetDescriptor as u8
            && self.direction() == Direction::In
    }

    /// Check if this is a SET_ADDRESS request
    pub fn is_set_address(&self) -> bool {
        self.is_standard()
            && self.b_request == StandardRequest::SetAddress as u8
            && self.direction() == Direction::Out
    }

    /// Check if this is a SET_CONFIGURATION request
    pub fn is_set_configuration(&self) -> bool {
        self.is_standard()
            && self.b_request == StandardRequest::SetConfiguration as u8
            && self.direction() == Direction::Out
    }

    /// Check if this is a GET_CONFIGURATION request
    pub fn is_get_configuration(&self) -> bool {
        self.is_standard()
            && self.b_request == StandardRequest::GetConfiguration as u8
            && self.direction() == Direction::In
    }

    /// Get the endpoint address for endpoint-targeted requests
    pub fn endpoint_address(&self) -> u8 {
        (self.w_index & 0xff) as u8
    }

    /// Get the interface number for interface-targeted requests
    pub fn interface_number(&self) -> u8 {
        (self.w_index & 0xff) as u8
    }
}

/// Control transfer state machine states
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ControlState {
    /// Idle, waiting for SETUP packet
    #[default]
    Idle,
    /// Received SETUP, processing request
    Setup,
    /// IN data stage (device to host)
    DataIn,
    /// OUT data stage (host to device)
    DataOut,
    /// Status stage for IN transfer (expecting ZLP OUT)
    StatusIn,
    /// Status stage for OUT transfer (sending ZLP IN)
    StatusOut,
    /// Error/stall state
    Error,
}

/// Control transfer handler
pub struct ControlPipe {
    /// Current state
    state: ControlState,
    /// Current setup packet
    setup: Option<SetupPacket>,
    /// Buffer for data stage
    buffer: [u8; 256],
    /// Amount of data in buffer
    buffer_len: usize,
    /// Amount of data already sent/received
    buffer_offset: usize,
    /// Expected total length
    expected_len: usize,
}

impl ControlPipe {
    /// Create a new control pipe
    pub const fn new() -> Self {
        Self {
            state: ControlState::Idle,
            setup: None,
            buffer: [0u8; 256],
            buffer_len: 0,
            buffer_offset: 0,
            expected_len: 0,
        }
    }

    /// Reset the control pipe to idle state
    pub fn reset(&mut self) {
        self.state = ControlState::Idle;
        self.setup = None;
        self.buffer_len = 0;
        self.buffer_offset = 0;
        self.expected_len = 0;
    }

    /// Handle a new SETUP packet
    pub fn handle_setup(&mut self, data: &[u8; 8]) -> &SetupPacket {
        let setup = SetupPacket::parse(data);

        self.setup = Some(setup);
        self.state = ControlState::Setup;
        self.buffer_len = 0;
        self.buffer_offset = 0;
        self.expected_len = setup.w_length as usize;

        // Determine next state based on direction and length
        if setup.w_length == 0 {
            // No data stage, go directly to status
            self.state = ControlState::StatusOut;
        } else if setup.direction() == Direction::In {
            // Host wants to read data from device
            self.state = ControlState::DataIn;
        } else {
            // Host wants to write data to device
            self.state = ControlState::DataOut;
        }

        self.setup.as_ref().unwrap()
    }

    /// Get the current setup packet
    pub fn setup(&self) -> Option<&SetupPacket> {
        self.setup.as_ref()
    }

    /// Get the current state
    pub fn state(&self) -> ControlState {
        self.state
    }

    /// Get the expected data length
    pub fn expected_len(&self) -> usize {
        self.expected_len
    }

    /// Get the current buffer length
    pub fn buffer_len(&self) -> usize {
        self.buffer_len
    }

    /// Set response data for IN transfers
    pub fn set_response(&mut self, data: &[u8]) -> UsbResult<()> {
        if data.len() > self.buffer.len() {
            return Err(UsbError::BufferOverflow);
        }

        self.buffer[..data.len()].copy_from_slice(data);
        self.buffer_len = core::cmp::min(data.len(), self.expected_len);
        self.buffer_offset = 0;

        Ok(())
    }

    /// Get the next chunk of data to send for IN transfers
    pub fn get_next_chunk(&mut self, max_packet_size: usize) -> Option<&[u8]> {
        if self.state != ControlState::DataIn {
            return None;
        }

        if self.buffer_offset >= self.buffer_len {
            // All data sent, move to status stage
            self.state = ControlState::StatusIn;
            return None;
        }

        let remaining = self.buffer_len - self.buffer_offset;
        let chunk_size = core::cmp::min(remaining, max_packet_size);
        let chunk = &self.buffer[self.buffer_offset..self.buffer_offset + chunk_size];

        self.buffer_offset += chunk_size;

        // If this was the last chunk or a short packet, move to status
        if self.buffer_offset >= self.buffer_len || chunk_size < max_packet_size {
            self.state = ControlState::StatusIn;
        }

        Some(chunk)
    }

    /// Accept received OUT data
    pub fn accept_out_data(&mut self, data: &[u8]) -> UsbResult<()> {
        if self.state != ControlState::DataOut {
            return Err(UsbError::InvalidState);
        }

        let space = self.buffer.len() - self.buffer_len;
        if data.len() > space {
            return Err(UsbError::BufferOverflow);
        }

        self.buffer[self.buffer_len..self.buffer_len + data.len()].copy_from_slice(data);
        self.buffer_len += data.len();

        // Check if we've received all expected data
        if self.buffer_len >= self.expected_len {
            self.state = ControlState::StatusOut;
        }

        Ok(())
    }

    /// Get the received OUT data
    pub fn out_data(&self) -> &[u8] {
        &self.buffer[..self.buffer_len]
    }

    /// Complete the status stage
    pub fn complete_status(&mut self) {
        self.state = ControlState::Idle;
        self.setup = None;
    }

    /// Set state to StatusIn (waiting for ZLP OUT from host)
    /// Used when class directly sends DataIn response
    pub fn set_status_in(&mut self) {
        self.state = ControlState::StatusIn;
    }

    /// Set error state
    pub fn set_error(&mut self) {
        self.state = ControlState::Error;
    }

    /// Check if in error state
    pub fn is_error(&self) -> bool {
        self.state == ControlState::Error
    }
}

impl Default for ControlPipe {
    fn default() -> Self {
        Self::new()
    }
}
