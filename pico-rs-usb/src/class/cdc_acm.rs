//! CDC ACM (Communication Device Class - Abstract Control Model) implementation
//!
//! This module provides a USB CDC ACM class for serial communication over USB.
//!
//! CDC ACM creates a virtual serial port that can be accessed from a host computer.
//!
//! ## Example
//!
//! ```ignore
//! use pico_rs_usb::{UsbDevice, UsbBus};
//! use pico_rs_usb::class::cdc::CdcAcm;
//!
//! let usb_bus = UsbBus::new();
//! let mut serial = CdcAcm::new(&usb_bus);
//!
//! let mut usb_dev = UsbDevice::builder(&usb_bus)
//!     .vendor_id(0x1234)
//!     .product_id(0x5678)
//!     .build();
//!
//! loop {
//!     if usb_dev.poll(&mut [&mut serial]) {
//!         let mut buf = [0u8; 64];
//!         if let Ok(count) = serial.read(&mut buf) {
//!             // Echo data back
//!             let _ = serial.write(&buf[..count]);
//!         }
//!     }
//! }
//! ```

use crate::BULK_MAX_PACKET_SIZE;
use crate::bus::UsbBus;
use crate::control::SetupPacket;
use crate::descriptor::{
    DescriptorWriter, EndpointDescriptor, InterfaceDescriptor, cdc_descriptor, cdc_protocol,
    cdc_subclass, class_code,
};
use crate::device::UsbClass;
use crate::endpoint::{EndpointAddress, EndpointType};
use crate::error::{UsbError, UsbResult};
use crate::types::{Recipient, RequestType};

/// CDC class-specific request codes
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum CdcRequest {
    /// Send Encapsulated Command
    SendEncapsulatedCommand = 0x00,
    /// Get Encapsulated Response
    GetEncapsulatedResponse = 0x01,
    /// Set Comm Feature
    SetCommFeature = 0x02,
    /// Get Comm Feature
    GetCommFeature = 0x03,
    /// Clear Comm Feature
    ClearCommFeature = 0x04,
    /// Set Line Coding
    SetLineCoding = 0x20,
    /// Get Line Coding
    GetLineCoding = 0x21,
    /// Set Control Line State
    SetControlLineState = 0x22,
    /// Send Break
    SendBreak = 0x23,
}

impl TryFrom<u8> for CdcRequest {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(CdcRequest::SendEncapsulatedCommand),
            0x01 => Ok(CdcRequest::GetEncapsulatedResponse),
            0x02 => Ok(CdcRequest::SetCommFeature),
            0x03 => Ok(CdcRequest::GetCommFeature),
            0x04 => Ok(CdcRequest::ClearCommFeature),
            0x20 => Ok(CdcRequest::SetLineCoding),
            0x21 => Ok(CdcRequest::GetLineCoding),
            0x22 => Ok(CdcRequest::SetControlLineState),
            0x23 => Ok(CdcRequest::SendBreak),
            v => Err(v),
        }
    }
}

/// Line coding structure (7 bytes)
#[derive(Clone, Copy, Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct LineCoding {
    /// Data terminal rate in bits per second
    dw_dte_rate: u32,
    /// Stop bits: 0=1, 1=1.5, 2=2
    b_char_format: u8,
    /// Parity: 0=None, 1=Odd, 2=Even, 3=Mark, 4=Space
    b_parity_type: u8,
    /// Data bits: 5, 6, 7, 8, or 16
    b_data_bits: u8,
}

impl LineCoding {
    /// Size of line coding structure
    pub const SIZE: usize = 7;

    /// Create a new line coding with default settings (115200 8N1)
    pub const fn new() -> Self {
        Self {
            dw_dte_rate: 115200,
            b_char_format: 0, // 1 stop bit
            b_parity_type: 0, // No parity
            b_data_bits: 8,
        }
    }

    /// Get the baud rate
    pub fn baud_rate(&self) -> u32 {
        self.dw_dte_rate
    }

    /// Get the number of data bits
    pub fn data_bits(&self) -> u8 {
        self.b_data_bits
    }

    /// Get the stop bits setting
    pub fn stop_bits(&self) -> u8 {
        self.b_char_format
    }

    /// Get the parity setting
    pub fn parity(&self) -> u8 {
        self.b_parity_type
    }

    /// Convert to bytes
    pub fn to_bytes(&self) -> [u8; Self::SIZE] {
        let rate_bytes = self.dw_dte_rate.to_le_bytes();
        [
            rate_bytes[0],
            rate_bytes[1],
            rate_bytes[2],
            rate_bytes[3],
            self.b_char_format,
            self.b_parity_type,
            self.b_data_bits,
        ]
    }

    /// Parse from bytes
    pub fn from_bytes(data: &[u8]) -> Option<Self> {
        if data.len() < Self::SIZE {
            return None;
        }
        Some(Self {
            dw_dte_rate: u32::from_le_bytes([data[0], data[1], data[2], data[3]]),
            b_char_format: data[4],
            b_parity_type: data[5],
            b_data_bits: data[6],
        })
    }
}

/// Control line state flags
#[derive(Clone, Copy, Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ControlLineState {
    /// DTR (Data Terminal Ready)
    pub dtr: bool,
    /// RTS (Request To Send)
    pub rts: bool,
}

impl ControlLineState {
    /// Create from raw value
    pub fn from_raw(value: u16) -> Self {
        Self {
            dtr: value & 0x01 != 0,
            rts: value & 0x02 != 0,
        }
    }
}

/// CDC ACM serial port
pub struct CdcAcm<'a> {
    /// USB bus reference
    bus: &'a UsbBus,
    /// Communication interface number
    comm_if: u8,
    /// Data interface number
    data_if: u8,
    /// Interrupt IN endpoint address
    int_ep: EndpointAddress,
    /// Bulk IN endpoint address
    bulk_in_ep: EndpointAddress,
    /// Bulk OUT endpoint address
    bulk_out_ep: EndpointAddress,
    /// Current line coding
    line_coding: LineCoding,
    /// Control line state
    control_line_state: ControlLineState,
    /// Read buffer
    read_buf: [u8; 64],
    /// Bytes available in read buffer
    read_available: usize,
    /// Write buffer
    write_buf: [u8; 64],
    /// Bytes in write buffer
    write_len: usize,
    /// DTR changed flag
    dtr_changed: bool,
    /// Port is open (DTR is set)
    port_open: bool,
    /// Configured flag
    configured: bool,
}

impl<'a> CdcAcm<'a> {
    /// Create a new CDC ACM instance
    pub fn new(bus: &'a UsbBus) -> Self {
        Self {
            bus,
            comm_if: 0,
            data_if: 1,
            int_ep: EndpointAddress::in_(1),
            bulk_in_ep: EndpointAddress::in_(2),
            bulk_out_ep: EndpointAddress::out(2),
            line_coding: LineCoding::new(),
            control_line_state: ControlLineState::default(),
            read_buf: [0u8; 64],
            read_available: 0,
            write_buf: [0u8; 64],
            write_len: 0,
            dtr_changed: false,
            port_open: false,
            configured: false,
        }
    }

    /// Get the current line coding
    pub fn line_coding(&self) -> &LineCoding {
        &self.line_coding
    }

    /// Get the control line state
    pub fn control_line_state(&self) -> &ControlLineState {
        &self.control_line_state
    }

    /// Check if DTR changed since last call
    pub fn dtr_changed(&mut self) -> bool {
        let changed = self.dtr_changed;
        self.dtr_changed = false;
        changed
    }

    /// Check if the port is open (DTR is set by the host)
    pub fn port_open(&self) -> bool {
        self.port_open
    }

    /// Check if the device is configured
    pub fn is_configured(&self) -> bool {
        self.configured
    }

    /// Write data to the serial port
    ///
    /// Returns the number of bytes written, which may be less than
    /// the buffer size if the internal buffer is full.
    pub fn write(&mut self, data: &[u8]) -> UsbResult<usize> {
        if !self.configured {
            return Err(UsbError::NotReady);
        }

        // Copy data to write buffer
        let space = self.write_buf.len() - self.write_len;
        let len = core::cmp::min(data.len(), space);

        if len == 0 {
            return Err(UsbError::WouldBlock);
        }

        self.write_buf[self.write_len..self.write_len + len].copy_from_slice(&data[..len]);
        self.write_len += len;

        // Try to send
        self.flush_write()?;

        Ok(len)
    }

    /// Write all data, blocking until complete
    pub fn write_all(&mut self, data: &[u8]) -> UsbResult<()> {
        let mut offset = 0;
        while offset < data.len() {
            match self.write(&data[offset..]) {
                Ok(n) => offset += n,
                Err(UsbError::WouldBlock) => continue,
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }

    /// Read data from the serial port
    ///
    /// Returns the number of bytes read, or `WouldBlock` if no data is available.
    pub fn read(&mut self, buf: &mut [u8]) -> UsbResult<usize> {
        if !self.configured {
            return Err(UsbError::NotReady);
        }

        if self.read_available == 0 {
            // Try to receive more data
            match self.bus.read(self.bulk_out_ep, &mut self.read_buf) {
                Ok(n) if n > 0 => {
                    self.read_available = n;
                }
                Ok(_) => return Err(UsbError::WouldBlock),
                Err(e) => return Err(e),
            }
        }

        let len = core::cmp::min(buf.len(), self.read_available);
        buf[..len].copy_from_slice(&self.read_buf[..len]);

        // Shift remaining data
        if len < self.read_available {
            self.read_buf.copy_within(len..self.read_available, 0);
        }
        self.read_available -= len;

        Ok(len)
    }

    /// Read a single byte
    pub fn read_byte(&mut self) -> Option<u8> {
        let mut buf = [0u8; 1];
        match self.read(&mut buf) {
            Ok(1) => Some(buf[0]),
            _ => None,
        }
    }

    /// Flush the write buffer
    fn flush_write(&mut self) -> UsbResult<()> {
        if self.write_len == 0 {
            return Ok(());
        }

        match self
            .bus
            .write(self.bulk_in_ep, &self.write_buf[..self.write_len])
        {
            Ok(n) => {
                // Shift remaining data
                if n < self.write_len {
                    self.write_buf.copy_within(n..self.write_len, 0);
                }
                self.write_len -= n;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    /// Handle CDC class requests
    fn handle_cdc_request(&mut self, setup: &SetupPacket) -> bool {
        let request = match CdcRequest::try_from(setup.b_request) {
            Ok(r) => r,
            Err(_) => return false,
        };

        match request {
            CdcRequest::GetLineCoding => {
                // Will be handled in control_in
                true
            }
            CdcRequest::SetLineCoding => {
                // Will be handled in control_out
                true
            }
            CdcRequest::SetControlLineState => {
                let old_dtr = self.control_line_state.dtr;
                self.control_line_state = ControlLineState::from_raw(setup.w_value);

                if old_dtr != self.control_line_state.dtr {
                    self.dtr_changed = true;
                    self.port_open = self.control_line_state.dtr;

                    #[cfg(feature = "defmt")]
                    defmt::info!(
                        "CDC: DTR={}, RTS={}",
                        self.control_line_state.dtr,
                        self.control_line_state.rts
                    );
                }

                // Send ZLP
                self.bus.ep0_send_zlp();
                true
            }
            CdcRequest::SendBreak => {
                // Acknowledge break signal
                self.bus.ep0_send_zlp();
                true
            }
            _ => false,
        }
    }
}

impl<'a> UsbClass for CdcAcm<'a> {
    fn control_setup(&mut self, setup: &SetupPacket) -> bool {
        // Check if this is a CDC class request for our interface
        if setup.request_type() == RequestType::Class
            && setup.recipient() == Recipient::Interface
            && setup.interface_number() == self.comm_if
        {
            return self.handle_cdc_request(setup);
        }
        false
    }

    fn control_in(&mut self, buf: &mut [u8]) -> UsbResult<usize> {
        // GET_LINE_CODING response
        let coding = self.line_coding.to_bytes();
        let len = core::cmp::min(buf.len(), coding.len());
        buf[..len].copy_from_slice(&coding[..len]);
        Ok(len)
    }

    fn control_out(&mut self, data: &[u8]) -> UsbResult<()> {
        // SET_LINE_CODING data
        if let Some(coding) = LineCoding::from_bytes(data) {
            #[cfg(feature = "defmt")]
            defmt::info!(
                "CDC: Line coding: {} baud, {} data bits",
                coding.baud_rate(),
                coding.data_bits()
            );
            self.line_coding = coding;
        }
        Ok(())
    }

    fn get_descriptors(&self, writer: &mut DescriptorWriter) -> bool {
        // CDC Communication Interface
        let comm_iface = InterfaceDescriptor::new(
            self.comm_if,
            class_code::CDC,
            cdc_subclass::ACM,
            cdc_protocol::NONE,
            1, // One interrupt endpoint
        );
        if !writer.write_interface(&comm_iface) {
            return false;
        }

        // CDC Functional Descriptors

        // Header Functional Descriptor
        if !writer.write(&[
            5,                      // bLength
            0x24,                   // bDescriptorType (CS_INTERFACE)
            cdc_descriptor::HEADER, // bDescriptorSubtype
            0x10,
            0x01, // bcdCDC (1.10)
        ]) {
            return false;
        }

        // Call Management Functional Descriptor
        if !writer.write(&[
            5,                               // bLength
            0x24,                            // bDescriptorType (CS_INTERFACE)
            cdc_descriptor::CALL_MANAGEMENT, // bDescriptorSubtype
            0x00,                            // bmCapabilities
            self.data_if,                    // bDataInterface
        ]) {
            return false;
        }

        // ACM Functional Descriptor
        if !writer.write(&[
            4,                   // bLength
            0x24,                // bDescriptorType (CS_INTERFACE)
            cdc_descriptor::ACM, // bDescriptorSubtype
            0x02,                // bmCapabilities (line coding + break)
        ]) {
            return false;
        }

        // Union Functional Descriptor
        if !writer.write(&[
            5,                     // bLength
            0x24,                  // bDescriptorType (CS_INTERFACE)
            cdc_descriptor::UNION, // bDescriptorSubtype
            self.comm_if,          // bControlInterface
            self.data_if,          // bSubordinateInterface0
        ]) {
            return false;
        }

        // Interrupt IN endpoint
        let int_ep_desc = EndpointDescriptor::new(
            self.int_ep,
            EndpointType::Interrupt,
            8,   // Max packet size
            255, // Polling interval
        );
        if !writer.write_endpoint(&int_ep_desc) {
            return false;
        }

        // CDC Data Interface
        let data_iface = InterfaceDescriptor::new(
            self.data_if,
            class_code::CDC_DATA,
            0,
            0,
            2, // Two bulk endpoints
        );
        if !writer.write_interface(&data_iface) {
            return false;
        }

        // Bulk OUT endpoint
        let bulk_out_desc = EndpointDescriptor::new(
            self.bulk_out_ep,
            EndpointType::Bulk,
            BULK_MAX_PACKET_SIZE,
            0,
        );
        if !writer.write_endpoint(&bulk_out_desc) {
            return false;
        }

        // Bulk IN endpoint
        let bulk_in_desc =
            EndpointDescriptor::new(self.bulk_in_ep, EndpointType::Bulk, BULK_MAX_PACKET_SIZE, 0);
        if !writer.write_endpoint(&bulk_in_desc) {
            return false;
        }

        true
    }

    fn configure(&mut self) {
        // Configure the CDC endpoints in hardware
        // Interrupt IN endpoint (EP1 IN)
        if let Err(e) = self
            .bus
            .configure_ep(self.int_ep, EndpointType::Interrupt, 8)
        {
            #[cfg(feature = "defmt")]
            defmt::error!("CDC: Failed to configure interrupt EP: {:?}", e);
            return;
        }

        // Bulk IN endpoint (EP2 IN)
        if let Err(e) =
            self.bus
                .configure_ep(self.bulk_in_ep, EndpointType::Bulk, BULK_MAX_PACKET_SIZE)
        {
            #[cfg(feature = "defmt")]
            defmt::error!("CDC: Failed to configure bulk IN EP: {:?}", e);
            return;
        }

        // Bulk OUT endpoint (EP2 OUT)
        if let Err(e) =
            self.bus
                .configure_ep(self.bulk_out_ep, EndpointType::Bulk, BULK_MAX_PACKET_SIZE)
        {
            #[cfg(feature = "defmt")]
            defmt::error!("CDC: Failed to configure bulk OUT EP: {:?}", e);
            return;
        }

        self.configured = true;
        #[cfg(feature = "defmt")]
        defmt::info!("CDC: Configured - endpoints initialized");
    }

    fn reset(&mut self) {
        self.configured = false;
        self.port_open = false;
        self.read_available = 0;
        self.write_len = 0;
        #[cfg(feature = "defmt")]
        defmt::info!("CDC: Reset");
    }

    fn poll(&mut self) {
        if !self.configured {
            return;
        }

        // Try to flush write buffer
        let _ = self.flush_write();
    }
}
