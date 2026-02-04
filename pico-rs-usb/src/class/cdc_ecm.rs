//! CDC-ECM (Communication Device Class - Ethernet Control Model) implementation
//!
//! This module provides a USB CDC-ECM class for Ethernet over USB.
//!
//! CDC-ECM creates a virtual Ethernet adapter that can be used for network
//! communication between the device and host.
//!
//! ## Features
//!
//! - Standard Ethernet frame transmission (up to 1514 bytes)
//! - MAC address configuration
//! - Link state notification
//! - Connection speed notification
//!
//! ## Example
//!
//! ```ignore
//! use pico_rs_usb::{UsbDevice, UsbBus};
//! use pico_rs_usb::class::cdc_ecm::CdcEcm;
//!
//! let usb_bus = UsbBus::new();
//! let mac_address = [0x02, 0x00, 0x00, 0x00, 0x00, 0x01];
//! let mut ecm = CdcEcm::new(&usb_bus, mac_address);
//!
//! let mut usb_dev = UsbDevice::builder(&usb_bus)
//!     .vendor_id(0x1234)
//!     .product_id(0x5678)
//!     .build();
//!
//! loop {
//!     if usb_dev.poll(&mut [&mut ecm]) {
//!         // Receive Ethernet frame
//!         let mut buf = [0u8; 1514];
//!         if let Ok(len) = ecm.receive(&mut buf) {
//!             // Process received frame
//!         }
//!
//!         // Send Ethernet frame
//!         let frame = [0u8; 64]; // Example frame
//!         let _ = ecm.transmit(&frame);
//!     }
//! }
//! ```

use crate::BULK_MAX_PACKET_SIZE;
use crate::bus::UsbBus;
use crate::control::SetupPacket;
use crate::descriptor::{
    DescriptorWriter, EndpointDescriptor, InterfaceDescriptor, cdc_descriptor, class_code,
};
use crate::device::UsbClass;
use crate::endpoint::{EndpointAddress, EndpointType};
use crate::error::{UsbError, UsbResult};
use crate::types::{Recipient, RequestType};

/// CDC-ECM subclass code
pub const CDC_ECM_SUBCLASS: u8 = 0x06;

/// CDC-ECM protocol code (No class specific protocol required)
pub const CDC_ECM_PROTOCOL: u8 = 0x00;

/// Maximum Ethernet frame size (MTU 1500 + 14 bytes header)
pub const MAX_ETHERNET_FRAME_SIZE: usize = 1514;

/// Maximum segment size for USB bulk transfer
pub const MAX_SEGMENT_SIZE: u16 = 1514;

/// CDC-ECM Functional Descriptor subtype
pub const CDC_ECM_FUNCTIONAL_DESCRIPTOR: u8 = 0x0F;

/// CDC-ECM class-specific notification codes
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum EcmNotification {
    /// Network connection status
    NetworkConnection = 0x00,
    /// Response available (not used in ECM)
    ResponseAvailable = 0x01,
    /// Connection speed change
    ConnectionSpeedChange = 0x2A,
}

/// CDC-ECM class-specific request codes
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum EcmRequest {
    /// Set Ethernet Multicast Filters
    SetEthernetMulticastFilters = 0x40,
    /// Set Ethernet Power Management Pattern Filter
    SetEthernetPowerManagementPatternFilter = 0x41,
    /// Get Ethernet Power Management Pattern Filter
    GetEthernetPowerManagementPatternFilter = 0x42,
    /// Set Ethernet Packet Filter
    SetEthernetPacketFilter = 0x43,
    /// Get Ethernet Statistic
    GetEthernetStatistic = 0x44,
}

impl TryFrom<u8> for EcmRequest {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x40 => Ok(EcmRequest::SetEthernetMulticastFilters),
            0x41 => Ok(EcmRequest::SetEthernetPowerManagementPatternFilter),
            0x42 => Ok(EcmRequest::GetEthernetPowerManagementPatternFilter),
            0x43 => Ok(EcmRequest::SetEthernetPacketFilter),
            0x44 => Ok(EcmRequest::GetEthernetStatistic),
            v => Err(v),
        }
    }
}

/// Ethernet packet filter bitmap
#[derive(Clone, Copy, Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct PacketFilter {
    /// Pass packets sent to the device (unicast)
    pub promiscuous: bool,
    /// Pass all multicast packets
    pub all_multicast: bool,
    /// Pass packets to the registered multicast addresses
    pub directed: bool,
    /// Pass broadcast packets
    pub broadcast: bool,
    /// Pass packets addressed to multicast addresses in filter
    pub multicast: bool,
}

impl PacketFilter {
    /// Create a new packet filter with default settings
    pub const fn new() -> Self {
        Self {
            promiscuous: false,
            all_multicast: false,
            directed: true,
            broadcast: true,
            multicast: true,
        }
    }

    /// Create from raw value
    pub fn from_raw(value: u16) -> Self {
        Self {
            promiscuous: value & 0x01 != 0,
            all_multicast: value & 0x02 != 0,
            directed: value & 0x04 != 0,
            broadcast: value & 0x08 != 0,
            multicast: value & 0x10 != 0,
        }
    }

    /// Convert to raw value
    pub fn to_raw(&self) -> u16 {
        let mut value = 0u16;
        if self.promiscuous {
            value |= 0x01;
        }
        if self.all_multicast {
            value |= 0x02;
        }
        if self.directed {
            value |= 0x04;
        }
        if self.broadcast {
            value |= 0x08;
        }
        if self.multicast {
            value |= 0x10;
        }
        value
    }
}

/// Connection speed change notification data
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ConnectionSpeed {
    /// Downstream bit rate (host to device) in bits per second
    pub downstream_bit_rate: u32,
    /// Upstream bit rate (device to host) in bits per second
    pub upstream_bit_rate: u32,
}

impl ConnectionSpeed {
    /// Create new connection speed with default 12 Mbps (USB Full Speed)
    pub const fn new() -> Self {
        Self {
            downstream_bit_rate: 12_000_000,
            upstream_bit_rate: 12_000_000,
        }
    }

    /// Create connection speed with custom rates
    pub const fn with_rates(downstream: u32, upstream: u32) -> Self {
        Self {
            downstream_bit_rate: downstream,
            upstream_bit_rate: upstream,
        }
    }

    /// Convert to bytes for notification
    pub fn to_bytes(&self) -> [u8; 8] {
        let down = self.downstream_bit_rate.to_le_bytes();
        let up = self.upstream_bit_rate.to_le_bytes();
        [
            down[0], down[1], down[2], down[3], up[0], up[1], up[2], up[3],
        ]
    }
}

impl Default for ConnectionSpeed {
    fn default() -> Self {
        Self::new()
    }
}

/// Network connection state
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NetworkState {
    /// Network is disconnected
    Disconnected,
    /// Network is connected
    Connected,
}

/// CDC-ECM Ethernet adapter
pub struct CdcEcm<'a> {
    /// USB bus reference
    bus: &'a UsbBus,
    /// Communication interface number
    comm_if: u8,
    /// Data interface number
    data_if: u8,
    /// Interrupt IN endpoint address (for notifications)
    int_ep: EndpointAddress,
    /// Bulk IN endpoint address (device to host data)
    bulk_in_ep: EndpointAddress,
    /// Bulk OUT endpoint address (host to device data)
    bulk_out_ep: EndpointAddress,
    /// MAC address (6 bytes)
    mac_address: [u8; 6],
    /// MAC address string index in string descriptor
    mac_string_index: u8,
    /// Packet filter settings
    packet_filter: PacketFilter,
    /// Network connection state
    network_state: NetworkState,
    /// Connection speed
    connection_speed: ConnectionSpeed,
    /// Receive buffer for incoming Ethernet frames
    rx_buf: [u8; MAX_ETHERNET_FRAME_SIZE],
    /// Number of bytes in receive buffer
    rx_len: usize,
    /// Receive offset (for reading partial frames)
    rx_offset: usize,
    /// Transmit buffer for outgoing Ethernet frames
    tx_buf: [u8; MAX_ETHERNET_FRAME_SIZE],
    /// Number of bytes in transmit buffer
    tx_len: usize,
    /// Transmit offset (for partial transmission)
    tx_offset: usize,
    /// Device is configured
    configured: bool,
    /// Alternate setting (0 = no data endpoints, 1 = data endpoints active)
    alternate_setting: u8,
    /// Pending notification to send
    pending_notification: Option<NotificationType>,
}

/// Type of pending notification
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
enum NotificationType {
    /// Network connection state changed
    NetworkConnection(bool),
    /// Connection speed changed
    ConnectionSpeedChange,
}

impl<'a> CdcEcm<'a> {
    /// Create a new CDC-ECM instance with the specified MAC address
    ///
    /// # Arguments
    ///
    /// * `bus` - USB bus reference
    /// * `mac_address` - 6-byte MAC address for the virtual Ethernet adapter
    ///
    /// # Notes
    ///
    /// The MAC address should be a locally administered address (bit 1 of first byte set).
    /// Example: `[0x02, 0x00, 0x00, 0x00, 0x00, 0x01]`
    pub fn new(bus: &'a UsbBus, mac_address: [u8; 6]) -> Self {
        Self {
            bus,
            comm_if: 0,
            data_if: 1,
            int_ep: EndpointAddress::in_(1),
            bulk_in_ep: EndpointAddress::in_(2),
            bulk_out_ep: EndpointAddress::out(2),
            mac_address,
            mac_string_index: 4, // After manufacturer, product, serial
            packet_filter: PacketFilter::new(),
            network_state: NetworkState::Disconnected,
            connection_speed: ConnectionSpeed::new(),
            rx_buf: [0u8; MAX_ETHERNET_FRAME_SIZE],
            rx_len: 0,
            rx_offset: 0,
            tx_buf: [0u8; MAX_ETHERNET_FRAME_SIZE],
            tx_len: 0,
            tx_offset: 0,
            configured: false,
            alternate_setting: 0,
            pending_notification: None,
        }
    }

    /// Create a new CDC-ECM instance with custom interface and endpoint numbers
    pub fn with_config(
        bus: &'a UsbBus,
        mac_address: [u8; 6],
        comm_if: u8,
        data_if: u8,
        int_ep_num: u8,
        bulk_ep_num: u8,
        mac_string_index: u8,
    ) -> Self {
        Self {
            bus,
            comm_if,
            data_if,
            int_ep: EndpointAddress::in_(int_ep_num),
            bulk_in_ep: EndpointAddress::in_(bulk_ep_num),
            bulk_out_ep: EndpointAddress::out(bulk_ep_num),
            mac_address,
            mac_string_index,
            packet_filter: PacketFilter::new(),
            network_state: NetworkState::Disconnected,
            connection_speed: ConnectionSpeed::new(),
            rx_buf: [0u8; MAX_ETHERNET_FRAME_SIZE],
            rx_len: 0,
            rx_offset: 0,
            tx_buf: [0u8; MAX_ETHERNET_FRAME_SIZE],
            tx_len: 0,
            tx_offset: 0,
            configured: false,
            alternate_setting: 0,
            pending_notification: None,
        }
    }

    /// Get the MAC address
    pub fn mac_address(&self) -> &[u8; 6] {
        &self.mac_address
    }

    /// Get the MAC address as a string (12 hex characters, uppercase)
    pub fn mac_address_string(&self) -> [u8; 12] {
        let mut result = [0u8; 12];
        for (i, &byte) in self.mac_address.iter().enumerate() {
            result[i * 2] = Self::nibble_to_hex(byte >> 4);
            result[i * 2 + 1] = Self::nibble_to_hex(byte & 0x0F);
        }
        result
    }

    /// Convert a nibble (0-15) to hex character
    fn nibble_to_hex(nibble: u8) -> u8 {
        if nibble < 10 {
            b'0' + nibble
        } else {
            b'A' + nibble - 10
        }
    }

    /// Get the current packet filter settings
    pub fn packet_filter(&self) -> &PacketFilter {
        &self.packet_filter
    }

    /// Get the network connection state
    pub fn network_state(&self) -> NetworkState {
        self.network_state
    }

    /// Set the network connection state and queue notification
    pub fn set_network_state(&mut self, connected: bool) {
        let new_state = if connected {
            NetworkState::Connected
        } else {
            NetworkState::Disconnected
        };

        if self.network_state != new_state {
            self.network_state = new_state;
            self.pending_notification = Some(NotificationType::NetworkConnection(connected));

            #[cfg(feature = "defmt")]
            defmt::info!(
                "CDC-ECM: Network state changed to {}",
                if connected {
                    "connected"
                } else {
                    "disconnected"
                }
            );
        }
    }

    /// Get the connection speed
    pub fn connection_speed(&self) -> &ConnectionSpeed {
        &self.connection_speed
    }

    /// Set the connection speed and queue notification
    pub fn set_connection_speed(&mut self, downstream: u32, upstream: u32) {
        self.connection_speed = ConnectionSpeed::with_rates(downstream, upstream);
        self.pending_notification = Some(NotificationType::ConnectionSpeedChange);

        #[cfg(feature = "defmt")]
        defmt::info!(
            "CDC-ECM: Connection speed set to {} / {} bps",
            downstream,
            upstream
        );
    }

    /// Check if the device is configured and ready
    pub fn is_ready(&self) -> bool {
        self.configured && self.alternate_setting == 1
    }

    /// Transmit an Ethernet frame
    ///
    /// Returns the number of bytes queued for transmission.
    /// The frame will be sent in the next poll cycle.
    pub fn transmit(&mut self, frame: &[u8]) -> UsbResult<usize> {
        if !self.is_ready() {
            return Err(UsbError::NotReady);
        }

        if frame.len() > MAX_ETHERNET_FRAME_SIZE {
            return Err(UsbError::BufferOverflow);
        }

        if self.tx_len > 0 {
            return Err(UsbError::WouldBlock);
        }

        self.tx_buf[..frame.len()].copy_from_slice(frame);
        self.tx_len = frame.len();
        self.tx_offset = 0;

        // Try to send immediately
        self.flush_tx()?;

        Ok(frame.len())
    }

    /// Receive an Ethernet frame
    ///
    /// Returns the number of bytes received, or `WouldBlock` if no frame is available.
    pub fn receive(&mut self, buf: &mut [u8]) -> UsbResult<usize> {
        if !self.is_ready() {
            return Err(UsbError::NotReady);
        }

        // First check if we have buffered data
        if self.rx_len > self.rx_offset {
            let available = self.rx_len - self.rx_offset;
            let len = core::cmp::min(buf.len(), available);
            buf[..len].copy_from_slice(&self.rx_buf[self.rx_offset..self.rx_offset + len]);
            self.rx_offset += len;

            // If we've read the entire frame, reset
            if self.rx_offset >= self.rx_len {
                self.rx_len = 0;
                self.rx_offset = 0;
            }

            return Ok(len);
        }

        // Try to receive new data
        match self.bus.read(self.bulk_out_ep, &mut self.rx_buf) {
            Ok(n) if n > 0 => {
                self.rx_len = n;
                self.rx_offset = 0;

                let len = core::cmp::min(buf.len(), n);
                buf[..len].copy_from_slice(&self.rx_buf[..len]);
                self.rx_offset = len;

                if self.rx_offset >= self.rx_len {
                    self.rx_len = 0;
                    self.rx_offset = 0;
                }

                Ok(len)
            }
            Ok(_) => Err(UsbError::WouldBlock),
            Err(e) => Err(e),
        }
    }

    /// Flush transmit buffer
    fn flush_tx(&mut self) -> UsbResult<()> {
        if self.tx_len == 0 || self.tx_offset >= self.tx_len {
            self.tx_len = 0;
            self.tx_offset = 0;
            return Ok(());
        }

        let remaining = self.tx_len - self.tx_offset;
        let chunk_size = core::cmp::min(remaining, BULK_MAX_PACKET_SIZE as usize);

        match self.bus.write(
            self.bulk_in_ep,
            &self.tx_buf[self.tx_offset..self.tx_offset + chunk_size],
        ) {
            Ok(n) => {
                self.tx_offset += n;

                // Check if we need to send a ZLP to terminate
                if self.tx_offset >= self.tx_len {
                    // If the last packet was exactly max packet size, send ZLP
                    if chunk_size == BULK_MAX_PACKET_SIZE as usize {
                        // Queue ZLP
                        let _ = self.bus.write(self.bulk_in_ep, &[]);
                    }
                    self.tx_len = 0;
                    self.tx_offset = 0;
                }

                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    /// Send pending notification
    fn send_notification(&mut self) -> UsbResult<()> {
        let notification = match self.pending_notification.take() {
            Some(n) => n,
            None => return Ok(()),
        };

        match notification {
            NotificationType::NetworkConnection(connected) => {
                // Notification header (8 bytes)
                let data = [
                    0xA1,                                     // bmRequestType: Class, Interface, Device-to-Host
                    EcmNotification::NetworkConnection as u8, // bNotification
                    if connected { 1 } else { 0 },            // wValue (low)
                    0,                                        // wValue (high)
                    self.comm_if,                             // wIndex (low)
                    0,                                        // wIndex (high)
                    0,                                        // wLength (low)
                    0,                                        // wLength (high)
                ];

                match self.bus.write(self.int_ep, &data) {
                    Ok(_) => {
                        #[cfg(feature = "defmt")]
                        defmt::debug!(
                            "CDC-ECM: Sent NetworkConnection notification: {}",
                            connected
                        );
                        Ok(())
                    }
                    Err(UsbError::WouldBlock) => {
                        // Re-queue the notification
                        self.pending_notification = Some(notification);
                        Ok(())
                    }
                    Err(e) => Err(e),
                }
            }
            NotificationType::ConnectionSpeedChange => {
                // Notification header (8 bytes) + speed data (8 bytes)
                let speed = self.connection_speed.to_bytes();
                let mut data = [0u8; 16];
                data[0] = 0xA1; // bmRequestType
                data[1] = EcmNotification::ConnectionSpeedChange as u8;
                data[2] = 0; // wValue (low)
                data[3] = 0; // wValue (high)
                data[4] = self.comm_if; // wIndex (low)
                data[5] = 0; // wIndex (high)
                data[6] = 8; // wLength (low)
                data[7] = 0; // wLength (high)
                data[8..16].copy_from_slice(&speed);

                match self.bus.write(self.int_ep, &data) {
                    Ok(_) => {
                        #[cfg(feature = "defmt")]
                        defmt::debug!("CDC-ECM: Sent ConnectionSpeedChange notification");
                        Ok(())
                    }
                    Err(UsbError::WouldBlock) => {
                        self.pending_notification = Some(notification);
                        Ok(())
                    }
                    Err(e) => Err(e),
                }
            }
        }
    }

    /// Handle CDC-ECM class requests
    fn handle_ecm_request(&mut self, setup: &SetupPacket) -> bool {
        let request = match EcmRequest::try_from(setup.b_request) {
            Ok(r) => r,
            Err(_) => return false,
        };

        match request {
            EcmRequest::SetEthernetPacketFilter => {
                self.packet_filter = PacketFilter::from_raw(setup.w_value);

                #[cfg(feature = "defmt")]
                defmt::info!(
                    "CDC-ECM: Packet filter set: promisc={}, multicast={}, directed={}, broadcast={}",
                    self.packet_filter.promiscuous,
                    self.packet_filter.all_multicast,
                    self.packet_filter.directed,
                    self.packet_filter.broadcast
                );

                // Send ZLP acknowledgment
                self.bus.ep0_send_zlp();
                true
            }
            EcmRequest::SetEthernetMulticastFilters => {
                // Acknowledge but don't actually implement multicast filtering
                self.bus.ep0_send_zlp();
                true
            }
            EcmRequest::GetEthernetStatistic => {
                // Not implemented - STALL
                false
            }
            _ => false,
        }
    }

    /// Handle SET_INTERFACE request for alternate settings
    fn handle_set_interface(&mut self, interface: u8, alt_setting: u8) -> bool {
        if interface != self.data_if {
            return false;
        }

        #[cfg(feature = "defmt")]
        defmt::info!(
            "CDC-ECM: SET_INTERFACE: interface={}, alt_setting={}",
            interface,
            alt_setting
        );

        match alt_setting {
            0 => {
                // Alternate setting 0: No data endpoints
                self.alternate_setting = 0;
                // Clear buffers
                self.rx_len = 0;
                self.rx_offset = 0;
                self.tx_len = 0;
                self.tx_offset = 0;
                true
            }
            1 => {
                // Alternate setting 1: Data endpoints active
                self.alternate_setting = 1;

                // Configure bulk endpoints
                if let Err(e) =
                    self.bus
                        .configure_ep(self.bulk_in_ep, EndpointType::Bulk, BULK_MAX_PACKET_SIZE)
                {
                    #[cfg(feature = "defmt")]
                    defmt::error!("CDC-ECM: Failed to configure bulk IN EP: {:?}", e);
                    return false;
                }

                if let Err(e) = self.bus.configure_ep(
                    self.bulk_out_ep,
                    EndpointType::Bulk,
                    BULK_MAX_PACKET_SIZE,
                ) {
                    #[cfg(feature = "defmt")]
                    defmt::error!("CDC-ECM: Failed to configure bulk OUT EP: {:?}", e);
                    return false;
                }

                // Queue network connected notification
                self.set_network_state(true);

                // Queue connection speed notification
                self.pending_notification = Some(NotificationType::ConnectionSpeedChange);

                true
            }
            _ => false,
        }
    }

    /// Get the MAC address string index (for string descriptors)
    pub fn mac_string_index(&self) -> u8 {
        self.mac_string_index
    }

    /// Build MAC address string descriptor
    ///
    /// Returns the MAC address as a USB string descriptor (UTF-16LE)
    pub fn build_mac_string_descriptor(&self, buf: &mut [u8]) -> usize {
        let mac_str = self.mac_address_string();

        // String descriptor: length, type, UTF-16LE data
        let len = 2 + mac_str.len() * 2;
        if buf.len() < len {
            return 0;
        }

        buf[0] = len as u8;
        buf[1] = 0x03; // String descriptor type

        for (i, &c) in mac_str.iter().enumerate() {
            buf[2 + i * 2] = c;
            buf[2 + i * 2 + 1] = 0;
        }

        len
    }
}

impl<'a> UsbClass for CdcEcm<'a> {
    fn control_setup(&mut self, setup: &SetupPacket) -> bool {
        // Handle SET_INTERFACE for alternate settings
        if setup.request_type() == RequestType::Standard
            && setup.recipient() == Recipient::Interface
            && setup.b_request == 0x0B
        {
            // SET_INTERFACE
            return self.handle_set_interface(setup.w_index as u8, setup.w_value as u8);
        }

        // Handle CDC class requests
        if setup.request_type() == RequestType::Class
            && setup.recipient() == Recipient::Interface
            && setup.interface_number() == self.comm_if
        {
            return self.handle_ecm_request(setup);
        }

        false
    }

    fn control_in(&mut self, buf: &mut [u8]) -> UsbResult<usize> {
        // No control IN data for ECM requests
        let _ = buf;
        Err(UsbError::Unsupported)
    }

    fn control_out(&mut self, data: &[u8]) -> UsbResult<()> {
        // No control OUT data for ECM requests
        let _ = data;
        Err(UsbError::Unsupported)
    }

    fn get_descriptors(&self, writer: &mut DescriptorWriter) -> bool {
        // CDC Communication Interface (Control)
        let comm_iface = InterfaceDescriptor::new(
            self.comm_if,
            class_code::CDC,
            CDC_ECM_SUBCLASS,
            CDC_ECM_PROTOCOL,
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
            0x20,
            0x01, // bcdCDC (1.20)
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

        // ECM Functional Descriptor
        if !writer.write(&[
            13,                            // bLength
            0x24,                          // bDescriptorType (CS_INTERFACE)
            CDC_ECM_FUNCTIONAL_DESCRIPTOR, // bDescriptorSubtype (Ethernet Networking)
            self.mac_string_index,         // iMACAddress (string index)
            0x00,
            0x00,
            0x00,
            0x00,                            // bmEthernetStatistics (none supported)
            (MAX_SEGMENT_SIZE & 0xFF) as u8, // wMaxSegmentSize (low)
            (MAX_SEGMENT_SIZE >> 8) as u8,   // wMaxSegmentSize (high)
            0x00,
            0x00, // wNumberMCFilters
            0x00, // bNumberPowerFilters
        ]) {
            return false;
        }

        // Interrupt IN endpoint (for notifications)
        let int_ep_desc = EndpointDescriptor::new(
            self.int_ep,
            EndpointType::Interrupt,
            16, // Max packet size for notifications
            32, // Polling interval (32ms)
        );
        if !writer.write_endpoint(&int_ep_desc) {
            return false;
        }

        // CDC Data Interface - Alternate Setting 0 (no endpoints)
        let data_iface_alt0 = InterfaceDescriptor::new(
            self.data_if,
            class_code::CDC_DATA,
            0, // No subclass
            0, // No protocol
            0, // No endpoints
        );
        if !writer.write_interface(&data_iface_alt0) {
            return false;
        }

        // CDC Data Interface - Alternate Setting 1 (with endpoints)
        // Write alternate setting interface manually
        if !writer.write(&[
            9,    // bLength
            0x04, // bDescriptorType (INTERFACE)
            self.data_if,
            1, // bAlternateSetting = 1
            2, // bNumEndpoints
            class_code::CDC_DATA,
            0, // bInterfaceSubClass
            0, // bInterfaceProtocol
            0, // iInterface
        ]) {
            return false;
        }

        // Bulk OUT endpoint (host to device)
        let bulk_out_desc = EndpointDescriptor::new(
            self.bulk_out_ep,
            EndpointType::Bulk,
            BULK_MAX_PACKET_SIZE,
            0,
        );
        if !writer.write_endpoint(&bulk_out_desc) {
            return false;
        }

        // Bulk IN endpoint (device to host)
        let bulk_in_desc =
            EndpointDescriptor::new(self.bulk_in_ep, EndpointType::Bulk, BULK_MAX_PACKET_SIZE, 0);
        if !writer.write_endpoint(&bulk_in_desc) {
            return false;
        }

        true
    }

    fn configure(&mut self) {
        // Configure the interrupt endpoint
        if let Err(e) = self
            .bus
            .configure_ep(self.int_ep, EndpointType::Interrupt, 16)
        {
            #[cfg(feature = "defmt")]
            defmt::error!("CDC-ECM: Failed to configure interrupt EP: {:?}", e);
            return;
        }

        // Data endpoints are configured when alternate setting 1 is selected
        self.configured = true;
        self.alternate_setting = 0; // Start with no data endpoints

        #[cfg(feature = "defmt")]
        defmt::info!("CDC-ECM: Configured - waiting for alternate setting");
    }

    fn reset(&mut self) {
        self.configured = false;
        self.alternate_setting = 0;
        self.network_state = NetworkState::Disconnected;
        self.rx_len = 0;
        self.rx_offset = 0;
        self.tx_len = 0;
        self.tx_offset = 0;
        self.pending_notification = None;

        #[cfg(feature = "defmt")]
        defmt::info!("CDC-ECM: Reset");
    }

    fn poll(&mut self) {
        if !self.configured {
            return;
        }

        // Send any pending notifications
        let _ = self.send_notification();

        // Flush transmit buffer
        if self.alternate_setting == 1 {
            let _ = self.flush_tx();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mac_address_string() {
        // We can't test with actual bus, but we can test the conversion
        let mac = [0x02, 0xAB, 0xCD, 0xEF, 0x12, 0x34];
        let mut result = [0u8; 12];
        for (i, &byte) in mac.iter().enumerate() {
            result[i * 2] = CdcEcm::nibble_to_hex(byte >> 4);
            result[i * 2 + 1] = CdcEcm::nibble_to_hex(byte & 0x0F);
        }
        assert_eq!(&result, b"02ABCDEF1234");
    }

    #[test]
    fn test_packet_filter() {
        let filter = PacketFilter::from_raw(0x1F);
        assert!(filter.promiscuous);
        assert!(filter.all_multicast);
        assert!(filter.directed);
        assert!(filter.broadcast);
        assert!(filter.multicast);
        assert_eq!(filter.to_raw(), 0x1F);
    }

    #[test]
    fn test_connection_speed() {
        let speed = ConnectionSpeed::with_rates(100_000_000, 100_000_000);
        let bytes = speed.to_bytes();
        assert_eq!(bytes[0..4], 100_000_000u32.to_le_bytes());
        assert_eq!(bytes[4..8], 100_000_000u32.to_le_bytes());
    }
}
