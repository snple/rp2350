//! USB Device implementation
//!
//! This module provides the high-level USB device abstraction.

use crate::bus::UsbBus;
use crate::control::{ControlPipe, ControlState, SetupPacket};
use crate::descriptor::{
    ConfigurationDescriptor, DescriptorWriter, DeviceDescriptor, StringDescriptors,
};
use crate::endpoint::EndpointAddress;
use crate::error::{UsbError, UsbResult};
use crate::types::{DescriptorType, DeviceState, Feature, Recipient, StandardRequest};
use crate::{CONTROL_MAX_PACKET_SIZE, CONTROL_TRANSFER_BUFFER_SIZE};

/// USB class trait
///
/// Implement this trait to create USB device classes (CDC, HID, etc.)
pub trait UsbClass {
    /// Called when a SETUP packet is received for this class
    ///
    /// Return `true` if the request was handled, `false` otherwise.
    fn control_setup(&mut self, setup: &SetupPacket) -> bool {
        let _ = setup;
        false
    }

    /// Called during the data stage of a control IN transfer
    ///
    /// Write data to the provided buffer and return the number of bytes written.
    fn control_in(&mut self, buf: &mut [u8]) -> UsbResult<usize> {
        let _ = buf;
        Err(UsbError::Unsupported)
    }

    /// Called during the data stage of a control OUT transfer
    ///
    /// Process the received data.
    fn control_out(&mut self, data: &[u8]) -> UsbResult<()> {
        let _ = data;
        Err(UsbError::Unsupported)
    }

    /// Get the class-specific descriptors
    fn get_descriptors(&self, writer: &mut DescriptorWriter) -> bool {
        let _ = writer;
        true
    }

    /// Called when the device is configured
    fn configure(&mut self) {}

    /// Called when the device is reset
    fn reset(&mut self) {}

    /// Poll the class for events
    ///
    /// Called from the main USB poll loop.
    fn poll(&mut self) {}
}

/// USB device builder
pub struct UsbDeviceBuilder<'a> {
    bus: &'a UsbBus,
    vendor_id: u16,
    product_id: u16,
    device_release: u16,
    device_class: u8,
    device_subclass: u8,
    device_protocol: u8,
    manufacturer: Option<&'a str>,
    product: Option<&'a str>,
    serial_number: Option<&'a str>,
    max_power_ma: u16,
    self_powered: bool,
    remote_wakeup: bool,
}

impl<'a> UsbDeviceBuilder<'a> {
    /// Create a new USB device builder
    pub fn new(bus: &'a UsbBus) -> Self {
        Self {
            bus,
            vendor_id: 0x1209, // pid.codes VID for testing
            product_id: 0x0001,
            device_release: 0x0100,
            device_class: 0, // Defined at interface level
            device_subclass: 0,
            device_protocol: 0,
            manufacturer: None,
            product: None,
            serial_number: None,
            max_power_ma: 100,
            self_powered: false,
            remote_wakeup: false,
        }
    }

    /// Set the vendor ID
    pub fn vendor_id(mut self, vid: u16) -> Self {
        self.vendor_id = vid;
        self
    }

    /// Set the product ID
    pub fn product_id(mut self, pid: u16) -> Self {
        self.product_id = pid;
        self
    }

    /// Set the device release version (BCD)
    pub fn device_release(mut self, release: u16) -> Self {
        self.device_release = release;
        self
    }

    /// Set the device class code
    pub fn device_class(mut self, class: u8) -> Self {
        self.device_class = class;
        self
    }

    /// Set the device subclass code
    pub fn device_subclass(mut self, subclass: u8) -> Self {
        self.device_subclass = subclass;
        self
    }

    /// Set the device protocol code
    pub fn device_protocol(mut self, protocol: u8) -> Self {
        self.device_protocol = protocol;
        self
    }

    /// Set the manufacturer string
    pub fn manufacturer(mut self, s: &'a str) -> Self {
        self.manufacturer = Some(s);
        self
    }

    /// Set the product string
    pub fn product(mut self, s: &'a str) -> Self {
        self.product = Some(s);
        self
    }

    /// Set the serial number string
    pub fn serial_number(mut self, s: &'a str) -> Self {
        self.serial_number = Some(s);
        self
    }

    /// Set the maximum power consumption in mA
    pub fn max_power(mut self, ma: u16) -> Self {
        self.max_power_ma = ma;
        self
    }

    /// Mark device as self-powered
    pub fn self_powered(mut self, value: bool) -> Self {
        self.self_powered = value;
        self
    }

    /// Enable remote wakeup capability
    pub fn remote_wakeup(mut self, value: bool) -> Self {
        self.remote_wakeup = value;
        self
    }

    /// Build the USB device
    pub fn build(self) -> UsbDevice<'a> {
        let strings = StringDescriptors::new()
            .manufacturer(self.manufacturer.unwrap_or(""))
            .product(self.product.unwrap_or(""))
            .serial_number(self.serial_number.unwrap_or(""));

        let mut device_desc = DeviceDescriptor::new(
            self.vendor_id,
            self.product_id,
            self.device_release,
            self.device_class,
            self.device_subclass,
            self.device_protocol,
        );

        // Set string descriptor indices based on whether strings are provided
        if self.manufacturer.is_some() {
            device_desc.set_manufacturer_index(1); // Index 1
        }
        if self.product.is_some() {
            device_desc.set_product_index(2); // Index 2
        }
        if self.serial_number.is_some() {
            device_desc.set_serial_number_index(3); // Index 3
        }

        let mut config_desc = ConfigurationDescriptor::new(0, 1, self.max_power_ma);
        if self.self_powered {
            config_desc = config_desc.self_powered();
        }
        if self.remote_wakeup {
            config_desc = config_desc.remote_wakeup();
        }

        UsbDevice {
            bus: self.bus,
            state: DeviceState::Detached,
            device_desc,
            config_desc,
            strings,
            control: ControlPipe::new(),
            control_buffer: [0u8; CONTROL_TRANSFER_BUFFER_SIZE],
            pending_address: None,
            configuration: 0,
            remote_wakeup_enabled: false,
            ep0_in_pending: false,
            active_class: None,
        }
    }
}

/// USB Device
pub struct UsbDevice<'a> {
    /// USB bus reference
    bus: &'a UsbBus,
    /// Current device state
    state: DeviceState,
    /// Device descriptor
    device_desc: DeviceDescriptor,
    /// Configuration descriptor
    config_desc: ConfigurationDescriptor,
    /// String descriptors
    strings: StringDescriptors<'a>,
    /// Control transfer pipe
    control: ControlPipe,
    /// Scratch buffer used to stage control responses before chunking on EP0.
    control_buffer: [u8; CONTROL_TRANSFER_BUFFER_SIZE],
    /// Pending address to set after status stage
    pending_address: Option<u8>,
    /// Current configuration value
    configuration: u8,
    /// Remote wakeup enabled
    remote_wakeup_enabled: bool,
    /// EP0 IN transfer in progress (waiting for completion)
    ep0_in_pending: bool,
    /// Index of the class currently owning the active control transfer, if any.
    active_class: Option<usize>,
}

impl<'a> UsbDevice<'a> {
    /// Create a USB device builder
    pub fn builder(bus: &'a UsbBus) -> UsbDeviceBuilder<'a> {
        UsbDeviceBuilder::new(bus)
    }

    /// Initialize and connect the USB device
    pub fn init(&mut self) -> UsbResult<()> {
        self.bus.init()?;
        self.state = DeviceState::Powered;
        self.bus.connect();
        Ok(())
    }

    /// Get the current device state
    pub fn state(&self) -> DeviceState {
        self.state
    }

    /// Check if device is configured
    pub fn is_configured(&self) -> bool {
        self.state == DeviceState::Configured
    }

    /// Poll for USB events and handle them
    ///
    /// Returns `true` if an event was handled.
    pub fn poll(&mut self, classes: &mut [&mut dyn UsbClass]) -> bool {
        let mut handled = false;

        // CRITICAL: Clear all IN complete bits at the start of poll.
        // This prevents interrupt storms when IN transfers complete but we don't
        // immediately queue another packet. The hardware sets these bits when
        // an IN transfer completes, and the interrupt keeps firing until cleared.
        // This matches rp235x-hal behavior.
        self.bus.clear_all_in_complete();

        // Check for bus reset
        if self.bus.poll_reset() {
            self.handle_reset(classes);
            handled = true;
        }

        // Check for suspend
        if self.bus.poll_suspend() {
            self.handle_suspend();
            handled = true;
        }

        // Check for resume
        if self.bus.poll_resume() {
            self.handle_resume();
            handled = true;
        }

        // Check for setup packet
        let setup_handled = self.bus.poll_setup();
        if setup_handled {
            self.handle_setup(classes);
            handled = true;
        }

        // Handle control transfer data stages (but not immediately after setup)
        // This avoids sending the next packet before the first one completes
        // Handle control transfer data stages (but not immediately after setup)
        // This avoids sending the next packet before the first one completes
        if !setup_handled {
            self.handle_control(classes);
        }

        // Poll classes
        for class in classes.iter_mut() {
            class.poll();
        }

        handled
    }

    /// Handle bus reset
    fn handle_reset(&mut self, classes: &mut [&mut dyn UsbClass]) {
        self.state = DeviceState::Default;
        self.configuration = 0;
        self.pending_address = None;
        self.ep0_in_pending = false;
        self.active_class = None;
        self.control.reset();
        self.bus.reset_non_control_endpoints();

        for class in classes.iter_mut() {
            class.reset();
        }

        #[cfg(feature = "defmt")]
        defmt::info!("USB: Reset handled, state = Default");
    }

    /// Handle suspend
    fn handle_suspend(&mut self) {
        if self.state != DeviceState::Suspended {
            self.state = DeviceState::Suspended;
            #[cfg(feature = "defmt")]
            defmt::info!("USB: Suspended");
        }
    }

    /// Handle resume
    fn handle_resume(&mut self) {
        if self.state == DeviceState::Suspended {
            // Restore previous state
            if self.configuration != 0 {
                self.state = DeviceState::Configured;
            } else if self.bus.address() != 0 {
                self.state = DeviceState::Addressed;
            } else {
                self.state = DeviceState::Default;
            }
            #[cfg(feature = "defmt")]
            defmt::info!("USB: Resumed");
        }
    }

    /// Handle setup packet
    fn handle_setup(&mut self, classes: &mut [&mut dyn UsbClass]) {
        let setup_data = self.bus.read_setup();
        let setup = self.control.handle_setup(&setup_data);

        // Copy setup for later use to avoid borrow issues
        let setup_copy = *setup;
        let ctrl_state = self.control.state();

        #[cfg(feature = "defmt")]
        defmt::debug!(
            "USB: SETUP bmRequestType={:#04x} bRequest={:#04x} wValue={:#06x} wIndex={:#06x} wLength={}",
            setup_copy.bm_request_type,
            setup_copy.b_request,
            setup_copy.w_value,
            setup_copy.w_index,
            setup_copy.w_length
        );

        self.active_class = None;

        // Try class handlers first
        for (class_index, class) in classes.iter_mut().enumerate() {
            if class.control_setup(&setup_copy) {
                self.active_class = Some(class_index);

                // If this is a DataOut request, prepare EP0 OUT to receive data
                if ctrl_state == ControlState::DataOut {
                    let expected_len = setup_copy.w_length as usize;
                    self.bus.ep0_expect_out_data(expected_len);
                    #[cfg(feature = "defmt")]
                    defmt::debug!("USB: Prepared EP0 OUT for {} bytes data", expected_len);
                } else if ctrl_state == ControlState::DataIn {
                    let response_len =
                        core::cmp::min(self.control_buffer.len(), setup_copy.w_length as usize);

                    let response_result =
                        class.control_in(&mut self.control_buffer[..response_len]);

                    match response_result {
                        Ok(len) => {
                            if !self.stage_control_buffer_response(len) {
                                self.active_class = None;
                                self.bus.ep0_stall();
                            }
                        }
                        Err(UsbError::Unsupported) => {
                            // Some classes still drive EP0 directly for bespoke
                            // responses, so we only need to arm the status stage.
                            self.bus.ep0_expect_out(0);
                            self.control.set_status_in();
                            #[cfg(feature = "defmt")]
                            defmt::debug!(
                                "USB: Class sent DataIn directly, prepared for status ZLP"
                            );
                        }
                        Err(err) => {
                            self.active_class = None;
                            self.bus.ep0_stall();
                            #[cfg(feature = "defmt")]
                            defmt::warn!("USB: Class control_in failed: {:?}", err);
                        }
                    }
                }
                return;
            }
        }

        // Handle standard requests
        if setup_copy.is_standard() {
            if !self.handle_standard_request(&setup_copy, classes) {
                #[cfg(feature = "defmt")]
                if Self::is_expected_unsupported_descriptor(&setup_copy) {
                    self.bus.ep0_stall_silent();
                    defmt::debug!(
                        "USB: EP0 stalled for unsupported full-speed-only descriptor {:?}",
                        setup_copy.descriptor_type()
                    );
                } else {
                    self.bus.ep0_stall();
                    defmt::warn!("USB: EP0 stalled");
                }
                #[cfg(not(feature = "defmt"))]
                self.bus.ep0_stall();
            }
        } else {
            // Unknown request, stall
            self.bus.ep0_stall();
            #[cfg(feature = "defmt")]
            defmt::warn!("USB: EP0 stalled");
        }
    }

    /// Handle standard USB requests
    fn handle_standard_request(
        &mut self,
        setup: &SetupPacket,
        classes: &mut [&mut dyn UsbClass],
    ) -> bool {
        match setup.as_standard_request() {
            Some(StandardRequest::GetDescriptor) => self.handle_get_descriptor(setup, classes),
            Some(StandardRequest::SetAddress) => self.handle_set_address(setup),
            Some(StandardRequest::SetConfiguration) => {
                self.handle_set_configuration(setup, classes)
            }
            Some(StandardRequest::GetConfiguration) => self.handle_get_configuration(),
            Some(StandardRequest::GetStatus) => self.handle_get_status(setup),
            Some(StandardRequest::ClearFeature) => self.handle_clear_feature(setup),
            Some(StandardRequest::SetFeature) => self.handle_set_feature(setup),
            _ => false,
        }
    }

    fn is_expected_unsupported_descriptor(setup: &SetupPacket) -> bool {
        matches!(
            setup.as_standard_request(),
            Some(StandardRequest::GetDescriptor)
        ) && matches!(
            setup.descriptor_type(),
            Some(DescriptorType::DeviceQualifier | DescriptorType::OtherSpeedConfiguration)
        )
    }

    /// Handle GET_DESCRIPTOR request
    fn handle_get_descriptor(
        &mut self,
        setup: &SetupPacket,
        classes: &mut [&mut dyn UsbClass],
    ) -> bool {
        let desc_type = setup.descriptor_type();
        let desc_index = setup.descriptor_index();

        #[cfg(feature = "defmt")]
        defmt::debug!(
            "USB: GET_DESCRIPTOR type={:?} index={}",
            desc_type,
            desc_index
        );

        let requested_len = setup.w_length as usize;

        let len = match desc_type {
            Some(DescriptorType::Device) => {
                let desc = self.device_desc.to_bytes();
                let len = core::cmp::min(desc.len(), requested_len);
                self.control_buffer[..len].copy_from_slice(&desc[..len]);
                #[cfg(feature = "defmt")]
                defmt::debug!("USB: Device descriptor len={}", len);
                len
            }
            Some(DescriptorType::Configuration) => {
                let control_buffer_len = self.control_buffer.len();
                let total_len = Self::build_config_descriptor(
                    self.config_desc,
                    &mut self.control_buffer,
                    control_buffer_len,
                    classes,
                );
                let len = core::cmp::min(total_len, requested_len);
                #[cfg(feature = "defmt")]
                defmt::debug!("USB: Config descriptor len={} total={}", len, total_len);
                len
            }
            Some(DescriptorType::String) => {
                if desc_index == 0 {
                    let len = self.strings.build_lang_ids(&mut self.control_buffer);
                    let len = core::cmp::min(len, requested_len);
                    #[cfg(feature = "defmt")]
                    defmt::debug!("USB: String lang IDs len={}", len);
                    len
                } else if let Some(s) = self.strings.get(desc_index) {
                    let len = StringDescriptors::build_string(s, &mut self.control_buffer);
                    let len = core::cmp::min(len, requested_len);
                    #[cfg(feature = "defmt")]
                    defmt::debug!("USB: String {} len={}", desc_index, len);
                    len
                } else {
                    #[cfg(feature = "defmt")]
                    defmt::warn!("USB: String {} not found", desc_index);
                    return false;
                }
            }
            _ => {
                return false;
            }
        };

        if len > 0 {
            #[cfg(feature = "defmt")]
            defmt::debug!("USB: Sending descriptor response len={}", len);
            self.stage_control_buffer_response(len)
        } else {
            false
        }
    }

    /// Build configuration descriptor with all interfaces
    fn build_config_descriptor(
        config_desc: ConfigurationDescriptor,
        buf: &mut [u8],
        max_len: usize,
        classes: &mut [&mut dyn UsbClass],
    ) -> usize {
        let mut writer = DescriptorWriter::new(buf);

        // Write configuration header
        let config_pos = match writer.write_configuration(&config_desc) {
            Some(pos) => pos,
            None => return 0,
        };

        // Let each class add their interface/endpoint descriptors
        for class in classes.iter() {
            if !class.get_descriptors(&mut writer) {
                #[cfg(feature = "defmt")]
                defmt::warn!("USB: Failed to write class descriptors");
                return 0;
            }
        }

        writer.finalize_configuration(config_pos);

        let len = writer.position();

        #[cfg(feature = "defmt")]
        {
            // Print configuration descriptor details
            if len >= 9 {
                defmt::info!(
                    "USB: Config desc: total_len={} num_ifaces={}",
                    u16::from_le_bytes([buf[2], buf[3]]),
                    buf[4]
                );
            }
            // Print first 32 bytes of config descriptor
            if len > 32 {
                defmt::debug!("USB: Config desc bytes[0..32]: {:02x}", &buf[..32]);
            }
        }
        core::cmp::min(len, max_len)
    }

    /// Handle SET_ADDRESS request
    fn handle_set_address(&mut self, setup: &SetupPacket) -> bool {
        let address = (setup.w_value & 0x7F) as u8;

        // Address is set after the status stage
        self.pending_address = Some(address);

        // Send ZLP for status stage
        self.bus.ep0_send_zlp();

        #[cfg(feature = "defmt")]
        defmt::debug!("USB: SET_ADDRESS pending = {}", address);

        true
    }

    /// Handle SET_CONFIGURATION request
    fn handle_set_configuration(
        &mut self,
        setup: &SetupPacket,
        classes: &mut [&mut dyn UsbClass],
    ) -> bool {
        let config = setup.w_value as u8;

        if config == 0 {
            // Deconfigured
            self.bus.reset_non_control_endpoints();
            self.configuration = 0;
            self.state = DeviceState::Addressed;
            for class in classes.iter_mut() {
                class.reset();
            }
        } else if config == 1 {
            // Configured
            self.bus.reset_non_control_endpoints();
            self.configuration = config;
            self.state = DeviceState::Configured;

            // Notify classes that device is configured
            for class in classes.iter_mut() {
                class.configure();
            }
        } else {
            // Invalid configuration
            return false;
        }

        // Send ZLP for status stage
        self.bus.ep0_send_zlp();

        #[cfg(feature = "defmt")]
        defmt::info!("USB: SET_CONFIGURATION = {}", config);

        true
    }

    /// Handle GET_CONFIGURATION request
    fn handle_get_configuration(&mut self) -> bool {
        self.stage_control_response(&[self.configuration])
    }

    /// Handle GET_STATUS request
    fn handle_get_status(&mut self, setup: &SetupPacket) -> bool {
        let status: u16 = match setup.recipient() {
            Recipient::Device => {
                let mut s = 0u16;
                if self.config_desc.bm_attributes & 0x40 != 0 {
                    s |= 1; // Self-powered
                }
                if self.remote_wakeup_enabled {
                    s |= 2; // Remote wakeup enabled
                }
                s
            }
            Recipient::Interface => 0,
            Recipient::Endpoint => {
                let ep_addr = EndpointAddress::from(setup.endpoint_address());
                if self.bus.is_stalled(ep_addr) { 1 } else { 0 }
            }
            _ => return false,
        };

        self.stage_control_response(&status.to_le_bytes())
    }

    fn stage_control_response(&mut self, data: &[u8]) -> bool {
        match self.control.set_response(data) {
            Ok(()) => {
                self.send_control_data();
                true
            }
            Err(err) => {
                #[cfg(feature = "defmt")]
                defmt::warn!("USB: Failed to stage control response: {:?}", err);
                false
            }
        }
    }

    fn stage_control_buffer_response(&mut self, len: usize) -> bool {
        let staged = {
            let (control, control_buffer) = (&mut self.control, &self.control_buffer);
            control.set_response(&control_buffer[..len])
        };

        match staged {
            Ok(()) => {
                self.send_control_data();
                true
            }
            Err(err) => {
                #[cfg(feature = "defmt")]
                defmt::warn!("USB: Failed to stage control response: {:?}", err);
                false
            }
        }
    }

    /// Handle CLEAR_FEATURE request
    fn handle_clear_feature(&mut self, setup: &SetupPacket) -> bool {
        match (setup.recipient(), Feature::try_from(setup.w_value)) {
            (Recipient::Device, Ok(Feature::DeviceRemoteWakeup)) => {
                self.remote_wakeup_enabled = false;
                self.bus.ep0_send_zlp();
                true
            }
            (Recipient::Endpoint, Ok(Feature::EndpointHalt)) => {
                let ep_addr = EndpointAddress::from(setup.endpoint_address());
                self.bus.unstall(ep_addr);
                self.bus.ep0_send_zlp();
                true
            }
            _ => false,
        }
    }

    /// Handle SET_FEATURE request
    fn handle_set_feature(&mut self, setup: &SetupPacket) -> bool {
        match (setup.recipient(), Feature::try_from(setup.w_value)) {
            (Recipient::Device, Ok(Feature::DeviceRemoteWakeup)) => {
                self.remote_wakeup_enabled = true;
                self.bus.ep0_send_zlp();
                true
            }
            (Recipient::Endpoint, Ok(Feature::EndpointHalt)) => {
                let ep_addr = EndpointAddress::from(setup.endpoint_address());
                self.bus.stall(ep_addr);
                self.bus.ep0_send_zlp();
                true
            }
            _ => false,
        }
    }

    /// Send control data (IN data stage)
    fn send_control_data(&mut self) {
        #[cfg(feature = "defmt")]
        defmt::debug!(
            "USB: send_control_data ENTRY, pending={}, ctrl_state={:?}",
            self.ep0_in_pending,
            self.control.state()
        );

        // Don't send if there's already a transfer in progress
        if self.ep0_in_pending {
            #[cfg(feature = "defmt")]
            defmt::debug!("USB: send_control_data skipped - transfer pending");
            return;
        }

        let state_before = self.control.state();

        // Get next chunk - this may change state to StatusIn
        let chunk_info = self
            .control
            .get_next_chunk(CONTROL_MAX_PACKET_SIZE as usize)
            .map(|chunk| {
                // Copy chunk data since we need to release the borrow
                let len = chunk.len();
                let mut buf = [0u8; 64];
                buf[..len].copy_from_slice(chunk);
                (buf, len)
            });

        // Now we can check the state after get_next_chunk
        let state_after_chunk = self.control.state();
        let is_last = state_after_chunk == ControlState::StatusIn;

        if let Some((buf, len)) = chunk_info {
            #[cfg(feature = "defmt")]
            defmt::debug!("USB: Sending {} bytes on EP0 IN, is_last={}", len, is_last);

            // Write data to EP0 buffer and start the IN transfer
            self.bus.ep0_write_and_start(&buf[..len], is_last);

            // Check if control state is still DataIn (more data to send)
            if !is_last {
                self.ep0_in_pending = true;
                #[cfg(feature = "defmt")]
                defmt::debug!("USB: ep0_in_pending set to TRUE (more data pending)");
            }
        }

        // If state moved to StatusIn after sending the last chunk,
        // prepare EP0 OUT for the status ZLP from host
        let state_after = self.control.state();
        #[cfg(feature = "defmt")]
        defmt::trace!(
            "USB: Control state: {:?} -> {:?}",
            state_before,
            state_after
        );

        if state_after == ControlState::StatusIn {
            self.bus.ep0_expect_out(0);
            #[cfg(feature = "defmt")]
            defmt::trace!("USB: Prepared EP0 OUT for status ZLP");
        }
    }

    /// Handle ongoing control transfers
    fn handle_control(&mut self, classes: &mut [&mut dyn UsbClass]) {
        let status = self.bus.buffer_status();
        let ctrl_state = self.control.state();

        #[cfg(feature = "defmt")]
        defmt::trace!(
            "USB: handle_control status={:#x} ctrl_state={:?}",
            status,
            ctrl_state
        );

        match ctrl_state {
            ControlState::DataIn => {
                // Check if previous packet was sent
                if status & 1 != 0 {
                    // EP0 IN complete - clear pending flag and buffer status
                    self.ep0_in_pending = false;
                    self.bus.clear_buffer_status(EndpointAddress::in_(0));
                    #[cfg(feature = "defmt")]
                    defmt::trace!("USB: EP0 IN complete, state=DataIn");

                    // Send more data or move to status stage
                    self.send_control_data();
                }
            }
            ControlState::DataOut => {
                // Check if data was received on EP0 OUT
                if status & 2 != 0 {
                    self.bus.clear_buffer_status(EndpointAddress::out(0));
                    #[cfg(feature = "defmt")]
                    defmt::trace!("USB: EP0 OUT data received, state=DataOut");

                    // Read the received data
                    let mut buf = [0u8; 64];
                    if let Ok(len) = self.bus.ep0_read(&mut buf) {
                        // Accept the data into control pipe
                        match self.control.accept_out_data(&buf[..len]) {
                            Ok(()) => {
                                #[cfg(feature = "defmt")]
                                defmt::debug!("USB: Received {} bytes control OUT data", len);
                            }
                            Err(err) => {
                                self.control.set_error();
                                self.active_class = None;
                                self.bus.ep0_stall();
                                #[cfg(feature = "defmt")]
                                defmt::warn!("USB: Control OUT overflow/state error: {:?}", err);
                                return;
                            }
                        }
                    }

                    // If all data received, state moves to StatusOut
                    if self.control.state() == ControlState::StatusOut {
                        // Route control OUT data only to the class that claimed
                        // the SETUP request.
                        let out_data = self.control.out_data();
                        if let Some(class_index) = self.active_class {
                            let _ = classes[class_index].control_out(out_data);
                        }
                        // Send status ZLP
                        self.bus.ep0_send_zlp();
                        #[cfg(feature = "defmt")]
                        defmt::debug!("USB: Sending status ZLP for OUT transfer");
                    } else {
                        // More data expected, re-arm EP0 OUT
                        let remaining = self.control.expected_len() - self.control.buffer_len();
                        self.bus.ep0_expect_out_data(remaining);
                    }
                }
            }
            ControlState::StatusIn => {
                // Waiting for ZLP OUT from host
                if status & 2 != 0 {
                    // EP0 OUT complete (status ZLP received)
                    self.bus.clear_buffer_status(EndpointAddress::out(0));
                    #[cfg(feature = "defmt")]
                    defmt::trace!("USB: Status IN complete (ZLP OUT received)");
                    self.control.complete_status();
                    self.active_class = None;
                }
            }
            ControlState::StatusOut => {
                // Check if ZLP was sent
                if status & 1 != 0 {
                    // EP0 IN complete (status ZLP sent)
                    self.bus.clear_buffer_status(EndpointAddress::in_(0));
                    #[cfg(feature = "defmt")]
                    defmt::trace!("USB: Status OUT complete (ZLP IN sent)");
                    self.control.complete_status();
                    self.active_class = None;

                    // Apply pending address
                    if let Some(addr) = self.pending_address.take() {
                        self.bus.set_address(addr);
                        self.state = if addr == 0 {
                            DeviceState::Default
                        } else {
                            DeviceState::Addressed
                        };
                        #[cfg(feature = "defmt")]
                        defmt::info!("USB: Address applied = {}", addr);
                    }
                }
            }
            _ => {}
        }
    }

    /// Get the USB bus reference
    pub fn bus(&self) -> &UsbBus {
        self.bus
    }
}
