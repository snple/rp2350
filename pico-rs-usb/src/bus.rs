//! USB Bus implementation for RP2350
//!
//! This module provides the low-level hardware abstraction for the RP2350's USB controller.

use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicBool, AtomicU8, Ordering};

use pico_rs_pac::{RESETS, USB, USB_DPRAM};

use crate::CONTROL_MAX_PACKET_SIZE;
use crate::endpoint::{
    EndpointAddress, EndpointAllocator, EndpointBuffer, EndpointConfig, EndpointDirection,
    EndpointType,
};
use crate::error::{UsbError, UsbResult};

/// USBCTRL reset bit
const USBCTRL_RESET_BIT: u32 = 1 << 28;

/// Bring USB controller out of reset
fn unreset_usb() {
    let resets = unsafe { &*RESETS::PTR };

    // Clear the reset bit
    resets.reset.clr_bits(USBCTRL_RESET_BIT);

    // Wait for reset to complete
    while resets.reset_done.read_bits() & USBCTRL_RESET_BIT == 0 {
        cortex_m::asm::nop();
    }
}

/// USB Bus state
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[allow(dead_code)]
enum BusState {
    /// Bus is not initialized
    Uninitialized,
    /// Bus is initialized and ready
    Ready,
    /// Bus is connected to host
    Connected,
    /// Bus is suspended
    Suspended,
}

/// USB Bus - Low-level hardware abstraction
///
/// This struct provides direct access to the RP2350's USB controller hardware.
/// It handles endpoint allocation, buffer management, and basic USB operations.
pub struct UsbBus {
    /// USB controller registers
    usb: &'static pico_rs_pac::usb::RegisterBlock,
    /// USB DPRAM
    dpram: &'static pico_rs_pac::usb_dpram::RegisterBlock,
    /// Bus initialized flag
    initialized: AtomicBool,
    /// Current device address
    address: AtomicU8,
    /// Endpoint allocator (interior mutability for use in embedded context)
    allocator: UnsafeCell<EndpointAllocator>,
    /// Endpoint 0 IN buffer
    ep0_in_buffer: UnsafeCell<EndpointBuffer>,
    /// Endpoint 0 OUT buffer
    ep0_out_buffer: UnsafeCell<EndpointBuffer>,
    /// Endpoint 0 IN data toggle (true = DATA1, false = DATA0)
    ep0_in_data_toggle: AtomicBool,
    /// Non-EP0 endpoint buffers (indices: 0-14 for IN ep1-15, 15-29 for OUT ep1-15)
    ep_buffers: UnsafeCell<[Option<EndpointBuffer>; 30]>,
    /// Non-EP0 endpoint data toggles
    ep_data_toggles: [AtomicBool; 30],
}

// Safety: UsbBus uses atomic operations and is designed for single-threaded embedded use
unsafe impl Send for UsbBus {}
unsafe impl Sync for UsbBus {}

impl UsbBus {
    /// Create a new USB bus instance
    ///
    /// This does not initialize the hardware. Call `init()` to initialize.
    pub const fn new() -> Self {
        const NONE_BUFFER: Option<EndpointBuffer> = None;
        const FALSE_TOGGLE: AtomicBool = AtomicBool::new(false);
        Self {
            usb: unsafe { &*(USB::PTR as *const _) },
            dpram: unsafe { &*(USB_DPRAM::PTR as *const _) },
            initialized: AtomicBool::new(false),
            address: AtomicU8::new(0),
            allocator: UnsafeCell::new(EndpointAllocator::new()),
            ep0_in_buffer: UnsafeCell::new(EndpointBuffer::new(0x100, CONTROL_MAX_PACKET_SIZE)),
            ep0_out_buffer: UnsafeCell::new(EndpointBuffer::new(
                0x100 + 64,
                CONTROL_MAX_PACKET_SIZE,
            )),
            ep0_in_data_toggle: AtomicBool::new(true), // Start with DATA1 after SETUP
            ep_buffers: UnsafeCell::new([NONE_BUFFER; 30]),
            ep_data_toggles: [FALSE_TOGGLE; 30],
        }
    }

    /// Initialize the USB controller hardware
    ///
    /// This function:
    /// 1. Brings USB out of reset
    /// 2. Clears DPRAM
    /// 3. Enables the USB controller in device mode
    /// 4. Configures endpoint 0 for control transfers
    pub fn init(&self) -> UsbResult<()> {
        if self.initialized.load(Ordering::Relaxed) {
            return Ok(());
        }

        // Bring USB out of reset
        unreset_usb();

        // Clear USB DPRAM
        self.clear_dpram();

        // Configure USB muxing - use internal PHY
        self.usb
            .usb_muxing
            .write_bits(pico_rs_pac::usb::UsbMuxing::TO_PHY | pico_rs_pac::usb::UsbMuxing::SOFTCON);

        // Configure USB power - enable VBUS detect
        self.usb.usb_pwr.write_bits(
            pico_rs_pac::usb::UsbPwr::VBUS_DETECT
                | pico_rs_pac::usb::UsbPwr::VBUS_DETECT_OVERRIDE_EN,
        );

        // Enable USB controller in device mode
        self.usb
            .main_ctrl
            .write_bits(pico_rs_pac::usb::MainCtrl::CONTROLLER_EN);

        // Enable interrupts
        self.usb
            .sie_ctrl
            .write_bits(pico_rs_pac::usb::SieCtrl::EP0_INT_1BUF);

        // Enable required interrupts
        self.usb.inte.write_bits(
            pico_rs_pac::usb::Inte::BUS_RESET
                | pico_rs_pac::usb::Inte::SETUP_REQ
                | pico_rs_pac::usb::Inte::BUFF_STATUS
                | pico_rs_pac::usb::Inte::DEV_SUSPEND
                | pico_rs_pac::usb::Inte::DEV_RESUME_FROM_HOST,
        );

        // Reset allocator
        unsafe {
            (*self.allocator.get()).reset();
        }

        // Setup EP0 buffers
        self.setup_ep0()?;

        self.initialized.store(true, Ordering::Release);

        #[cfg(feature = "defmt")]
        defmt::info!("USB: Initialized");

        Ok(())
    }

    /// Clear the USB DPRAM
    fn clear_dpram(&self) {
        let dpram_ptr = pico_rs_pac::usb_dpram::BASE as *mut u32;
        for i in 0..(crate::DPRAM_SIZE / 4) {
            unsafe {
                core::ptr::write_volatile(dpram_ptr.add(i), 0);
            }
        }
    }

    /// Setup endpoint 0 for control transfers
    fn setup_ep0(&self) -> UsbResult<()> {
        // EP0 buffer control registers are at the start of DPRAM
        // The actual buffers are placed after the control registers

        // Configure EP0 IN buffer control
        let ep0_in_ctrl = self.get_ep_buffer_control(EndpointAddress::in_(0));
        unsafe {
            core::ptr::write_volatile(ep0_in_ctrl, 0);
        }

        // Configure EP0 OUT buffer control
        let ep0_out_ctrl = self.get_ep_buffer_control(EndpointAddress::out(0));
        unsafe {
            core::ptr::write_volatile(ep0_out_ctrl, 0);
        }

        Ok(())
    }

    /// Connect the device to the USB bus (enable pull-up)
    pub fn connect(&self) {
        self.usb
            .sie_ctrl
            .set_bits(pico_rs_pac::usb::SieCtrl::PULLUP_EN);

        #[cfg(feature = "defmt")]
        defmt::debug!("USB: Connected (pull-up enabled)");
    }

    /// Disconnect the device from the USB bus (disable pull-up)
    pub fn disconnect(&self) {
        self.usb
            .sie_ctrl
            .clr_bits(pico_rs_pac::usb::SieCtrl::PULLUP_EN);

        #[cfg(feature = "defmt")]
        defmt::debug!("USB: Disconnected (pull-up disabled)");
    }

    /// Set the device address
    pub fn set_address(&self, address: u8) {
        self.address.store(address, Ordering::SeqCst);
        self.usb.addr_endp.write_address(address as u32);

        #[cfg(feature = "defmt")]
        defmt::debug!("USB: Address set to {}", address);
    }

    /// Get the current device address
    pub fn address(&self) -> u8 {
        self.address.load(Ordering::Relaxed)
    }

    /// Check if a bus reset occurred
    pub fn poll_reset(&self) -> bool {
        let ints = self.usb.ints.read_bits();
        if ints & pico_rs_pac::usb::Ints::BUS_RESET != 0 {
            // Clear the interrupt
            self.usb
                .sie_status
                .write_bits(pico_rs_pac::usb::SieStatus::BUS_RESET);

            #[cfg(feature = "defmt")]
            defmt::info!("USB: Bus reset");

            // Reset address to 0
            self.set_address(0);

            return true;
        }
        false
    }

    /// Check if a setup packet was received
    pub fn poll_setup(&self) -> bool {
        let ints = self.usb.ints.read_bits();
        ints & pico_rs_pac::usb::Ints::SETUP_REQ != 0
    }

    /// Read the setup packet from DPRAM
    pub fn read_setup(&self) -> [u8; 8] {
        // Clear the setup request interrupt
        self.usb
            .sie_status
            .write_bits(pico_rs_pac::usb::SieStatus::SETUP_REC);

        // Reset data toggle to DATA1 for new control transfer
        self.ep0_in_data_toggle.store(true, Ordering::SeqCst);

        let setup_low = self.dpram.setup_packet_low.read_bits();
        let setup_high = self.dpram.setup_packet_high.read_bits();

        let mut packet = [0u8; 8];
        packet[0..4].copy_from_slice(&setup_low.to_le_bytes());
        packet[4..8].copy_from_slice(&setup_high.to_le_bytes());

        #[cfg(feature = "defmt")]
        defmt::trace!("USB: Setup packet: {:02x}", packet);

        packet
    }

    /// Check if suspend occurred
    pub fn poll_suspend(&self) -> bool {
        let ints = self.usb.ints.read_bits();
        if ints & pico_rs_pac::usb::Ints::DEV_SUSPEND != 0 {
            self.usb
                .sie_status
                .write_bits(pico_rs_pac::usb::SieStatus::SUSPENDED);
            return true;
        }
        false
    }

    /// Check if resume occurred
    pub fn poll_resume(&self) -> bool {
        let ints = self.usb.ints.read_bits();
        if ints & pico_rs_pac::usb::Ints::DEV_RESUME_FROM_HOST != 0 {
            self.usb
                .sie_status
                .write_bits(pico_rs_pac::usb::SieStatus::RESUME);
            return true;
        }
        false
    }

    /// Get the buffer status register
    pub fn buffer_status(&self) -> u32 {
        self.usb.buff_status.read_bits()
    }

    /// Clear buffer status for an endpoint
    pub fn clear_buffer_status(&self, ep_addr: EndpointAddress) {
        let bit = 1u32 << (ep_addr.number() * 2 + if ep_addr.direction().is_in() { 0 } else { 1 });
        self.usb.buff_status.write_bits(bit);
    }

    /// Get pointer to endpoint buffer control register
    fn get_ep_buffer_control(&self, ep_addr: EndpointAddress) -> *mut u32 {
        let base = pico_rs_pac::usb_dpram::BASE as *mut u32;
        // EP buffer controls start at offset 0x80 in DPRAM
        // Each endpoint has two 32-bit words: IN then OUT
        let offset = 0x80 / 4
            + (ep_addr.number() as usize) * 2
            + if ep_addr.direction().is_in() { 0 } else { 1 };
        unsafe { base.add(offset) }
    }

    /// Get the index for endpoint buffer/toggle arrays
    /// IN ep1-15 -> indices 0-14
    /// OUT ep1-15 -> indices 15-29
    fn ep_buffer_index(&self, ep_addr: EndpointAddress) -> usize {
        let num = (ep_addr.number() - 1) as usize; // ep1 -> 0, ep2 -> 1, etc
        if ep_addr.direction().is_in() {
            num
        } else {
            num + 15
        }
    }

    /// Configure and enable a non-control endpoint
    ///
    /// This sets up the endpoint control register and allocates a buffer.
    pub fn configure_ep(
        &self,
        ep_addr: EndpointAddress,
        ep_type: EndpointType,
        max_packet_size: u16,
    ) -> UsbResult<()> {
        if ep_addr.number() == 0 {
            return Ok(()); // EP0 is configured separately
        }

        let allocator = unsafe { &mut *self.allocator.get() };
        let buffer = allocator.allocate(max_packet_size)?;

        // Store the buffer
        let idx = self.ep_buffer_index(ep_addr);
        let buffers = unsafe { &mut *self.ep_buffers.get() };
        buffers[idx] = Some(buffer);

        // Reset data toggle
        self.ep_data_toggles[idx].store(false, Ordering::SeqCst);

        // Configure endpoint control register
        // EP control registers are at offsets 0x08, 0x0c, 0x10, ... in DPRAM
        // EP1_IN_CONTROL = 0x08, EP1_OUT_CONTROL = 0x0c, EP2_IN_CONTROL = 0x10, etc.
        let ep_ctrl_base = pico_rs_pac::usb_dpram::BASE as *mut u32;
        let ctrl_offset = if ep_addr.direction().is_in() {
            (ep_addr.number() as usize) * 2 // IN: 0x08, 0x10, 0x18, ...
        } else {
            (ep_addr.number() as usize) * 2 + 1 // OUT: 0x0c, 0x14, 0x1c, ...
        };

        // Build endpoint control register value
        // Bit 31: ENABLE
        // Bit 29: INTERRUPT_PER_BUFF
        // Bits 27:26: ENDPOINT_TYPE (0=Control, 1=Iso, 2=Bulk, 3=Interrupt)
        // Bits 15:0: BUFFER_ADDRESS (64-byte aligned offset in DPRAM)
        let ep_type_bits = match ep_type {
            EndpointType::Control => 0,
            EndpointType::Isochronous => 1,
            EndpointType::Bulk => 2,
            EndpointType::Interrupt => 3,
        };

        let ctrl_val = (1 << 31)                         // ENABLE
            | (1 << 29)                                   // INTERRUPT_PER_BUFF
            | ((ep_type_bits as u32) << 26)               // ENDPOINT_TYPE
            | ((buffer.offset as u32) & 0xFFFF); // BUFFER_ADDRESS

        unsafe {
            core::ptr::write_volatile(ep_ctrl_base.add(ctrl_offset), ctrl_val);
        }

        // For OUT endpoints, also arm the buffer control to receive data
        if ep_addr.direction().is_out() {
            let buf_ctrl = self.get_ep_buffer_control(ep_addr);
            let buf_ctrl_val = (max_packet_size as u32) & 0x3FF  // LENGTH
                | (1 << 10); // AVAILABLE
            unsafe {
                core::ptr::write_volatile(buf_ctrl, buf_ctrl_val);
            }
        }

        #[cfg(feature = "defmt")]
        defmt::info!(
            "USB: Configured {} type={:?} buffer_offset={:#x} size={}",
            ep_addr,
            ep_type,
            buffer.offset,
            max_packet_size
        );

        Ok(())
    }

    /// Reset data toggle for an endpoint (used after CLEAR_FEATURE HALT)
    pub fn reset_ep_data_toggle(&self, ep_addr: EndpointAddress) {
        if ep_addr.number() == 0 {
            self.ep0_in_data_toggle.store(true, Ordering::SeqCst);
        } else {
            let idx = self.ep_buffer_index(ep_addr);
            self.ep_data_toggles[idx].store(false, Ordering::SeqCst);
        }
    }

    /// Allocate an endpoint
    pub fn alloc_endpoint(
        &self,
        direction: EndpointDirection,
        config: &EndpointConfig,
    ) -> UsbResult<EndpointAddress> {
        let allocator = unsafe { &mut *self.allocator.get() };

        // Find a free endpoint number (1-15, 0 is reserved for control)
        // For simplicity, we'll just allocate sequentially
        // In a real implementation, you'd track which endpoints are in use

        // Allocate buffer space
        let buffer = allocator.allocate(config.max_packet_size)?;

        // For this implementation, we'll use a simple scheme:
        // odd endpoints for IN, even for OUT (except EP0)
        // This is just an example - real implementations may differ

        #[cfg(feature = "defmt")]
        defmt::debug!(
            "USB: Allocated endpoint buffer at offset {:#x}, size {}",
            buffer.offset,
            buffer.size
        );

        // Return a placeholder - real implementation would track endpoints
        Ok(EndpointAddress::new(1, direction))
    }

    /// Configure an endpoint's buffer control
    pub fn configure_endpoint(&self, ep_addr: EndpointAddress, buffer: &EndpointBuffer) {
        // Get the endpoint control register in DPRAM
        // Endpoint controls are at the start of DPRAM (0x00-0x7F)
        let ep_ctrl_base = pico_rs_pac::usb_dpram::BASE as *mut u32;
        let ep_num = ep_addr.number() as usize;

        if ep_num > 0 {
            // Non-control endpoints: configure endpoint control register
            let ctrl_offset = if ep_addr.direction().is_in() {
                ep_num * 2 // IN endpoint control
            } else {
                ep_num * 2 + 1 // OUT endpoint control (after IN)
            };

            // Set buffer address and enable interrupt
            let ctrl_val = (buffer.offset as u32) | (1 << 29) | (1 << 31); // INTERRUPT_PER_BUFF | ENABLE

            unsafe {
                core::ptr::write_volatile(ep_ctrl_base.add(ctrl_offset), ctrl_val);
            }
        }

        #[cfg(feature = "defmt")]
        defmt::debug!(
            "USB: Configured {} with buffer at offset {:#x}",
            ep_addr,
            buffer.offset
        );
    }

    /// Write data to an endpoint buffer
    pub fn write(&self, ep_addr: EndpointAddress, data: &[u8]) -> UsbResult<usize> {
        if !ep_addr.direction().is_in() && ep_addr.number() != 0 {
            return Err(UsbError::InvalidEndpoint);
        }

        let (buffer, data_toggle) = if ep_addr.number() == 0 {
            let buf = unsafe { *self.ep0_in_buffer.get() };
            let toggle = self.ep0_in_data_toggle.load(Ordering::SeqCst);
            (buf, toggle)
        } else {
            // For non-EP0 endpoints, look up the allocated buffer
            let idx = self.ep_buffer_index(ep_addr);
            let buffers = unsafe { &*self.ep_buffers.get() };
            let buf = buffers[idx].ok_or(UsbError::InvalidEndpoint)?;
            let toggle = self.ep_data_toggles[idx].load(Ordering::SeqCst);
            (buf, toggle)
        };

        let len = core::cmp::min(data.len(), buffer.size as usize);

        // Write data to DPRAM buffer
        let buf_ptr = buffer.dpram_address() as *mut u8;
        unsafe {
            core::ptr::copy_nonoverlapping(data.as_ptr(), buf_ptr, len);
        }

        // Set up buffer control for transmission
        // Bit 15: FULL_0 - Buffer is full
        // Bit 13: PID_0 - DATA toggle (DATA1 if set, DATA0 if clear)
        // Bit 10: AVAILABLE_0 - Buffer available for hardware
        let ctrl = self.get_ep_buffer_control(ep_addr);
        let mut ctrl_val = (len as u32)  // LENGTH_0
            | (1 << 10)                   // AVAILABLE_0
            | (1 << 15); // FULL_0

        if data_toggle {
            ctrl_val |= 1 << 13; // PID_0 = DATA1
        }

        // Toggle for next transfer
        if ep_addr.number() == 0 {
            self.ep0_in_data_toggle
                .store(!data_toggle, Ordering::SeqCst);
        } else {
            let idx = self.ep_buffer_index(ep_addr);
            self.ep_data_toggles[idx].store(!data_toggle, Ordering::SeqCst);
        }

        #[cfg(feature = "defmt")]
        defmt::trace!(
            "USB: write {} bytes to {}, toggle={}",
            len,
            ep_addr,
            data_toggle
        );

        unsafe {
            core::ptr::write_volatile(ctrl, ctrl_val);
        }

        Ok(len)
    }

    /// Read data from an endpoint buffer
    pub fn read(&self, ep_addr: EndpointAddress, data: &mut [u8]) -> UsbResult<usize> {
        if ep_addr.direction().is_in() && ep_addr.number() != 0 {
            return Err(UsbError::InvalidEndpoint);
        }

        let buffer = if ep_addr.number() == 0 {
            unsafe { *self.ep0_out_buffer.get() }
        } else {
            // For non-EP0 endpoints, look up the allocated buffer
            let idx = self.ep_buffer_index(ep_addr);
            let buffers = unsafe { &*self.ep_buffers.get() };
            buffers[idx].ok_or(UsbError::InvalidEndpoint)?
        };

        // Read buffer control to check if data is available
        let ctrl = self.get_ep_buffer_control(ep_addr);
        let ctrl_val = unsafe { core::ptr::read_volatile(ctrl) };

        // Check if buffer has data (FULL bit should be set by hardware when data received)
        if (ctrl_val & (1 << 15)) == 0 {
            // No data available
            return Err(UsbError::WouldBlock);
        }

        let received_len = (ctrl_val & 0x3FF) as usize;
        let len = core::cmp::min(received_len, data.len());

        // Read data from DPRAM buffer
        let buf_ptr = buffer.dpram_address() as *const u8;
        unsafe {
            core::ptr::copy_nonoverlapping(buf_ptr, data.as_mut_ptr(), len);
        }

        // Toggle data for next transfer
        if ep_addr.number() != 0 {
            let idx = self.ep_buffer_index(ep_addr);
            let toggle = self.ep_data_toggles[idx].load(Ordering::SeqCst);
            self.ep_data_toggles[idx].store(!toggle, Ordering::SeqCst);
        }

        // Re-arm the endpoint to receive more data
        let toggle = if ep_addr.number() == 0 {
            true // EP0 always uses DATA1 for OUT after setup
        } else {
            let idx = self.ep_buffer_index(ep_addr);
            self.ep_data_toggles[idx].load(Ordering::SeqCst)
        };

        let mut new_ctrl = (buffer.size as u32) & 0x3FF  // Max length
            | (1 << 10); // AVAILABLE_0
        if toggle {
            new_ctrl |= 1 << 13; // PID_0 = DATA1
        }

        unsafe {
            core::ptr::write_volatile(ctrl, new_ctrl);
        }

        #[cfg(feature = "defmt")]
        defmt::trace!("USB: read {} bytes from {}", len, ep_addr);

        Ok(len)
    }

    /// Write data to EP0 IN (for control transfers)
    pub fn ep0_write(&self, data: &[u8]) -> UsbResult<usize> {
        self.write(EndpointAddress::in_(0), data)
    }

    /// Read data from EP0 OUT (for control transfers)
    pub fn ep0_read(&self, data: &mut [u8]) -> UsbResult<usize> {
        self.read(EndpointAddress::out(0), data)
    }

    /// Start EP0 IN transaction with proper DATA toggle
    pub fn ep0_start_in(&self, len: usize, is_last: bool) {
        let buffer = unsafe { *self.ep0_in_buffer.get() };
        let ctrl = self.get_ep_buffer_control(EndpointAddress::in_(0));

        // Get current data toggle and flip it for next time
        let data1 = self.ep0_in_data_toggle.load(Ordering::SeqCst);
        self.ep0_in_data_toggle.store(!data1, Ordering::SeqCst);

        // Buffer control bits for EP0 IN:
        // Bit 15: FULL_0 - Buffer is full (for IN/TX)
        // Bit 14: LAST_0 - Last buffer of transfer
        // Bit 13: PID_0 - DATA1 PID (set for DATA1, clear for DATA0)
        // Bit 10: AVAILABLE_0 - Buffer is available
        // Bits 9:0: LENGTH_0 - Data length

        let mut ctrl_val = (len as u32) & 0x3FF; // LENGTH_0
        ctrl_val |= 1 << 10; // AVAILABLE_0
        ctrl_val |= 1 << 15; // FULL_0 (buffer is full, ready to send)

        if data1 {
            ctrl_val |= 1 << 13; // PID_0 = DATA1
        }
        // else PID_0 = 0 (DATA0)

        if is_last {
            ctrl_val |= 1 << 14; // LAST_0
        }

        #[cfg(feature = "defmt")]
        defmt::debug!(
            "EP0 IN: len={}, ctrl={:#010x}, data1={}",
            len,
            ctrl_val,
            data1
        );

        unsafe {
            core::ptr::write_volatile(ctrl, ctrl_val);
        }
    }

    /// Write data to EP0 IN buffer and start the transfer
    ///
    /// This is the preferred method for control IN transfers as it atomically
    /// writes data and starts the transfer.
    pub fn ep0_write_and_start(&self, data: &[u8], is_last: bool) {
        let buffer = unsafe { *self.ep0_in_buffer.get() };
        let len = core::cmp::min(data.len(), buffer.size as usize);

        // Write data to DPRAM buffer
        let buf_ptr = buffer.dpram_address() as *mut u8;
        unsafe {
            core::ptr::copy_nonoverlapping(data.as_ptr(), buf_ptr, len);
        }

        #[cfg(feature = "defmt")]
        {
            defmt::debug!(
                "USB: ep0_write_and_start len={} buf_addr={:#x}",
                len,
                buffer.dpram_address()
            );
            if len > 0 && len <= 8 {
                let slice = &data[..len];
                defmt::debug!("USB: Data: {:02x}", slice);
            }
        }

        // Start the transfer
        self.ep0_start_in(len, is_last);
    }

    /// Prepare EP0 OUT for receiving data
    pub fn ep0_expect_out(&self, max_len: usize) {
        let ctrl = self.get_ep_buffer_control(EndpointAddress::out(0));

        // For status ZLP, use DATA1 PID
        let ctrl_val = (max_len as u32) & 0x3FF  // Length
            | (1 << 10)                           // AVAILABLE_0
            | (1 << 13); // PID_0 = DATA1 (for status stage)

        #[cfg(feature = "defmt")]
        defmt::trace!(
            "USB: ep0_expect_out max_len={} ctrl={:#010x}",
            max_len,
            ctrl_val
        );

        unsafe {
            core::ptr::write_volatile(ctrl, ctrl_val);
        }
    }

    /// Send a zero-length packet on EP0 IN (status stage)
    pub fn ep0_send_zlp(&self) {
        self.ep0_start_in(0, true);
    }

    /// Stall endpoint 0
    pub fn ep0_stall(&self) {
        self.usb.ep_stall_arm.write_bits(
            pico_rs_pac::usb::EpStallArm::EP0_IN | pico_rs_pac::usb::EpStallArm::EP0_OUT,
        );

        // Set STALL bit in buffer control
        let ctrl_in = self.get_ep_buffer_control(EndpointAddress::in_(0));
        let ctrl_out = self.get_ep_buffer_control(EndpointAddress::out(0));

        unsafe {
            core::ptr::write_volatile(ctrl_in, 1 << 11); // STALL
            core::ptr::write_volatile(ctrl_out, 1 << 11); // STALL
        }

        #[cfg(feature = "defmt")]
        defmt::warn!("USB: EP0 stalled");
    }

    /// Stall an endpoint
    pub fn stall(&self, ep_addr: EndpointAddress) {
        let ctrl = self.get_ep_buffer_control(ep_addr);
        unsafe {
            let val = core::ptr::read_volatile(ctrl);
            core::ptr::write_volatile(ctrl, val | (1 << 11)); // STALL bit
        }
    }

    /// Clear stall on an endpoint and reset data toggle
    pub fn unstall(&self, ep_addr: EndpointAddress) {
        // Clear STALL bit
        let ctrl = self.get_ep_buffer_control(ep_addr);
        unsafe {
            let val = core::ptr::read_volatile(ctrl);
            core::ptr::write_volatile(ctrl, val & !(1 << 11)); // Clear STALL bit
        }

        // Reset data toggle to DATA0 per USB spec
        self.reset_ep_data_toggle(ep_addr);

        #[cfg(feature = "defmt")]
        defmt::debug!("USB: Unstalled {}, reset data toggle", ep_addr);
    }

    /// Check if an endpoint is stalled
    pub fn is_stalled(&self, ep_addr: EndpointAddress) -> bool {
        let ctrl = self.get_ep_buffer_control(ep_addr);
        let val = unsafe { core::ptr::read_volatile(ctrl) };
        (val & (1 << 11)) != 0
    }

    /// Get access to the raw USB registers
    pub fn regs(&self) -> &pico_rs_pac::usb::RegisterBlock {
        self.usb
    }

    /// Get access to the raw USB DPRAM
    pub fn dpram(&self) -> &pico_rs_pac::usb_dpram::RegisterBlock {
        self.dpram
    }
}

impl Default for UsbBus {
    fn default() -> Self {
        Self::new()
    }
}
