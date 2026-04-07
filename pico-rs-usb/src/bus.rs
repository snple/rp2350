//! USB Bus implementation for RP2350
//!
//! This module provides the low-level hardware abstraction for the RP2350's USB controller.
//!
//! ## Hardware Errata
//!
//! The RP2350 USB controller requires special handling when setting up buffer control
//! registers. Specifically, you must:
//! 1. Write all fields EXCEPT the AVAILABLE bit first
//! 2. Wait at least 12 CPU cycles
//! 3. Then set only the AVAILABLE bit (using read-modify-write)
//!
//! This module uses the PAC's register access methods and the `ep_regs` module
//! for type-safe endpoint access with proper errata handling.

use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicBool, AtomicU8, Ordering};

use pico_rs_pac::usb_dpram::{Ep0InBufferControl, Ep0InBufferControlVal, Ep0OutBufferControlVal};
use pico_rs_pac::{RESETS, USB, USB_DPRAM};

use crate::CONTROL_MAX_PACKET_SIZE;
use crate::dpram::Dpram;
use crate::endpoint::{
    EndpointAddress, EndpointAllocator, EndpointBuffer, EndpointConfig, EndpointDirection,
    EndpointType,
};
use crate::ep_regs::{
    BufferControlAccess, BufferControlRegister, EndpointControlAccess, buffer_control_bits,
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
        pico_rs_rt::asm::nop();
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
    /// USB DPRAM register block
    dpram: &'static pico_rs_pac::usb_dpram::RegisterBlock,
    /// DPRAM memory accessor
    dpram_mem: Dpram,
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
        Self {
            usb: unsafe { &*(USB::PTR as *const _) },
            dpram: unsafe { &*(USB_DPRAM::PTR as *const _) },
            dpram_mem: Dpram::new(),
            initialized: AtomicBool::new(false),
            address: AtomicU8::new(0),
            allocator: UnsafeCell::new(EndpointAllocator::new()),
            ep0_in_buffer: UnsafeCell::new(EndpointBuffer::new(0x100, CONTROL_MAX_PACKET_SIZE)),
            ep0_out_buffer: UnsafeCell::new(EndpointBuffer::new(
                0x100 + 64,
                CONTROL_MAX_PACKET_SIZE,
            )),
            ep0_in_data_toggle: AtomicBool::new(true), // Start with DATA1 after SETUP
            ep_buffers: UnsafeCell::new([None; 30]),
            ep_data_toggles: [const { AtomicBool::new(false) }; 30],
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

        // Clear USB DPRAM using the Dpram helper
        self.dpram_mem.clear();

        // Configure USB muxing - use internal PHY
        self.usb
            .usb_muxing
            .write_bits(pico_rs_pac::usb::UsbMuxing::TO_PHY | pico_rs_pac::usb::UsbMuxing::SOFTCON);

        // If the board does not route USB VBUS signals to GPIOs, we must
        // override both VBUS_DETECT and VBUS_EN. Program the value bits first,
        // then enable the override bits as required by the RP2350 register docs.
        self.usb
            .usb_pwr
            .write_bits(pico_rs_pac::usb::UsbPwr::VBUS_EN | pico_rs_pac::usb::UsbPwr::VBUS_DETECT);
        self.usb.usb_pwr.set_bits(
            pico_rs_pac::usb::UsbPwr::VBUS_EN_OVERRIDE_EN
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

        // Setup EP0 buffers using PAC methods
        self.setup_ep0()?;

        self.initialized.store(true, Ordering::Release);

        #[cfg(feature = "defmt")]
        defmt::info!("USB: Initialized");

        Ok(())
    }

    /// Setup endpoint 0 for control transfers
    fn setup_ep0(&self) -> UsbResult<()> {
        // EP0 buffer control registers - clear using PAC methods
        self.dpram.ep0_in_buffer_control.write_bits(0);
        self.dpram.ep0_out_buffer_control.write_bits(0);

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

            // Clear any pending buffer status
            self.usb.buff_status.write_bits(0xFFFF_FFFF);

            #[cfg(feature = "defmt")]
            defmt::info!("USB: Bus reset");

            // Reset address to 0
            self.set_address(0);

            // Reset EP0 buffer control registers
            // EP0 IN: clear all bits (with PID set for first DATA1)
            self.dpram
                .ep0_in_buffer_control
                .write_bits(pico_rs_pac::usb_dpram::Ep0InBufferControl::PID_0);
            // EP0 OUT: set PID to DATA1 for first control transfer and prepare to receive
            self.dpram
                .ep0_out_buffer_control
                .write_bits(pico_rs_pac::usb_dpram::Ep0OutBufferControl::PID_0);
            // Apply errata delay before setting AVAILABLE
            pico_rs_rt::asm::delay(12);
            // Arm EP0 OUT to receive the first SETUP packet
            self.dpram
                .ep0_out_buffer_control
                .set_bits(pico_rs_pac::usb_dpram::Ep0OutBufferControl::AVAILABLE_0);

            // Reset data toggle
            self.ep0_in_data_toggle.store(true, Ordering::SeqCst);

            return true;
        }
        false
    }

    /// Check if a setup packet was received
    ///
    /// Returns true if a SETUP packet was received.
    /// IMPORTANT: When this returns true, EP0 IN's AVAILABLE bit has been cleared
    /// to prevent timing issues in release builds (see rp235x-hal work-around).
    pub fn poll_setup(&self) -> bool {
        let ints = self.usb.ints.read_bits();
        if ints & pico_rs_pac::usb::Ints::SETUP_REQ != 0 {
            // CRITICAL: Clear EP0 IN's AVAILABLE bit immediately when SETUP is detected.
            // This prevents a race condition where a stale DataIn packet could be
            // sent instead of the expected StatusIn for SET_ADDRESS.
            // This must happen BEFORE read_setup() is called.
            // See rp235x-hal "Small max_packet_size_ep0 Work-Around" comment.
            self.dpram.ep0_in_buffer_control.write_bits(0);
            return true;
        }
        false
    }

    /// Read the setup packet from DPRAM
    pub fn read_setup(&self) -> [u8; 8] {
        // Clear the setup request interrupt
        self.usb
            .sie_status
            .write_bits(pico_rs_pac::usb::SieStatus::SETUP_REC);

        // Reset data toggle to DATA1 for new control transfer
        self.ep0_in_data_toggle.store(true, Ordering::SeqCst);

        // Read setup packet using PAC register accessors
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

    /// Clear all IN complete bits from buffer status
    ///
    /// This should be called at the start of poll() to prevent interrupt storms.
    /// IN complete events are typically handled implicitly when the next write happens.
    /// Writing 1s to buff_status clears the corresponding bits.
    ///
    /// IMPORTANT: This clears EP1-EP15 IN bits only, preserving EP0 IN (bit 0)
    /// because EP0 IN complete is needed for control transfer state machine.
    pub fn clear_all_in_complete(&self) {
        // 0x5555_5554 = all even bits except bit 0 (EP1-EP15 IN, not EP0 IN)
        // Bit 0 = EP0 IN (preserve for control transfers)
        // Bit 2 = EP1 IN, Bit 4 = EP2 IN, etc.
        self.usb.buff_status.write_bits(0x5555_5554);
    }

    /// Clear all buffer status bits
    pub fn clear_all_buffer_status(&self) {
        self.usb.buff_status.write_bits(0xFFFF_FFFF);
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

        let idx = self.ep_buffer_index(ep_addr);
        let buffers = unsafe { &mut *self.ep_buffers.get() };
        if buffers[idx].is_some() {
            return Err(UsbError::EndpointAllocated);
        }

        let allocator = unsafe { &mut *self.allocator.get() };
        let buffer = allocator.allocate(max_packet_size)?;

        // Store the buffer
        buffers[idx] = Some(buffer);

        // Reset data toggle
        self.ep_data_toggles[idx].store(false, Ordering::SeqCst);

        // Configure endpoint control register using PAC-based accessor
        let ep_ctrl = EndpointControlAccess::new(self.dpram);
        ep_ctrl.configure(ep_addr, ep_type, buffer.offset);

        // For OUT endpoints, also arm the buffer control to receive data
        if ep_addr.direction().is_out() {
            let buf_ctrl = BufferControlAccess::new(self.dpram);
            let buf_ctrl_val = (max_packet_size as u32) & buffer_control_bits::LENGTH_0_MASK;
            buf_ctrl.write_with_errata(ep_addr, buf_ctrl_val);
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

    /// Reset all dynamically configured non-control endpoints.
    pub fn reset_non_control_endpoints(&self) {
        unsafe {
            (*self.allocator.get()).reset();
        }

        let buffers = unsafe { &mut *self.ep_buffers.get() };
        for slot in buffers.iter_mut() {
            *slot = None;
        }

        for toggle in &self.ep_data_toggles {
            toggle.store(false, Ordering::SeqCst);
        }

        let ep_ctrl = EndpointControlAccess::new(self.dpram);
        for ep_num in 1..=15 {
            ep_ctrl.disable(EndpointAddress::in_(ep_num));
            ep_ctrl.disable(EndpointAddress::out(ep_num));
        }

        self.clear_all_buffer_status();
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
        let buffers = unsafe { &*self.ep_buffers.get() };
        let ep_number = (1..=15)
            .find(|&ep_num| {
                let ep_addr = EndpointAddress::new(ep_num, direction);
                let idx = self.ep_buffer_index(ep_addr);
                buffers[idx].is_none()
            })
            .ok_or(UsbError::NoEndpointAvailable)?;

        let ep_addr = EndpointAddress::new(ep_number, direction);
        self.configure_ep(ep_addr, config.ep_type, config.max_packet_size)?;
        Ok(ep_addr)
    }

    /// Configure an endpoint's buffer control
    pub fn configure_endpoint(&self, ep_addr: EndpointAddress, buffer: &EndpointBuffer) {
        if ep_addr.number() > 0 {
            // Configure endpoint control register using PAC-based accessor
            // Default to Bulk type for now - caller should use configure_ep for proper setup
            let ep_ctrl = EndpointControlAccess::new(self.dpram);
            ep_ctrl.configure(ep_addr, EndpointType::Bulk, buffer.offset);
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

        // For non-EP0 endpoints, check if previous transfer is complete
        // and clear the buffer status to prevent interrupt storm
        if ep_addr.number() != 0 {
            let buf_ctrl = BufferControlAccess::new(self.dpram);
            let ctrl_val = buf_ctrl.read(ep_addr);
            // If FULL bit is still set, previous transfer not complete
            if (ctrl_val & buffer_control_bits::FULL_0) != 0 {
                return Err(UsbError::WouldBlock);
            }
            // Clear any pending buffer status for this endpoint
            self.clear_buffer_status(ep_addr);
        }

        let len = core::cmp::min(data.len(), buffer.size as usize);

        // Write data to DPRAM buffer using Dpram helper
        self.dpram_mem.copy_to_buffer(buffer.offset, &data[..len]);

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

        // Build buffer control value
        let mut ctrl_val = (len as u32) | buffer_control_bits::FULL_0;
        if data_toggle {
            ctrl_val |= buffer_control_bits::PID_0; // DATA1
        }

        // Use errata-safe write sequence for all endpoints
        let buf_ctrl = BufferControlAccess::new(self.dpram);
        buf_ctrl.write_with_errata(ep_addr, ctrl_val);

        Ok(len)
    }

    /// Read data from an endpoint buffer.
    ///
    /// Reads one packet's worth of data and clears the BUFF_STATUS bit so the
    /// interrupt does not re-fire immediately.  **The endpoint is NOT re-armed
    /// here**; call [`rearm_out_ep`] when the application is ready to receive
    /// the next packet.  This prevents an interrupt storm where every re-arm
    /// immediately triggers a new interrupt before the main loop gets a chance
    /// to run.
    ///
    /// For EP0 OUT (control transfers) the re-arm is performed automatically
    /// because EP0 must always remain armed.
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

        // Read buffer control to check if data is available using PAC-based accessor
        let buf_ctrl = BufferControlAccess::new(self.dpram);
        let ctrl_val = buf_ctrl.read(ep_addr);

        // Check if buffer has data (FULL bit should be set by hardware when data received)
        if (ctrl_val & buffer_control_bits::FULL_0) == 0 {
            // No data available
            return Err(UsbError::WouldBlock);
        }

        // CRITICAL: Clear the BUFF_STATUS interrupt for this endpoint BEFORE reading data.
        // If not cleared, the interrupt will continuously fire after the handler exits.
        self.clear_buffer_status(ep_addr);

        let received_len = (ctrl_val & buffer_control_bits::LENGTH_0_MASK) as usize;
        let len = core::cmp::min(received_len, data.len());

        // Read data from DPRAM buffer using Dpram helper
        self.dpram_mem
            .copy_from_buffer(buffer.offset, &mut data[..len]);

        // Advance the data toggle so the next re-arm uses the correct PID.
        if ep_addr.number() != 0 {
            let idx = self.ep_buffer_index(ep_addr);
            let toggle = self.ep_data_toggles[idx].load(Ordering::SeqCst);
            self.ep_data_toggles[idx].store(!toggle, Ordering::SeqCst);
        }

        // For EP0 OUT: always re-arm immediately (control transfers require it).
        // For non-EP0 OUT: do NOT re-arm here; the caller must call rearm_out_ep()
        // when it is ready to accept the next packet.  Immediate re-arm would
        // trigger a new BUFF_STATUS interrupt before the main loop can run.
        if ep_addr.number() == 0 {
            let mut new_ctrl = (buffer.size as u32) & buffer_control_bits::LENGTH_0_MASK;
            new_ctrl |= buffer_control_bits::PID_0; // EP0 OUT uses DATA1 after SETUP
            buf_ctrl.write_with_errata(ep_addr, new_ctrl);
        }

        #[cfg(feature = "defmt")]
        defmt::trace!("USB: read {} bytes from {}", len, ep_addr);

        Ok(len)
    }

    /// Re-arm a non-EP0 OUT endpoint to receive the next packet.
    ///
    /// Must be called after [`read`] once the application has consumed the
    /// packet and is ready to accept another one.  Separating re-arming from
    /// reading prevents interrupt storms that starve the main loop.
    pub fn rearm_out_ep(&self, ep_addr: EndpointAddress) {
        if ep_addr.number() == 0 || ep_addr.direction().is_in() {
            return; // EP0 is re-armed inside read(); IN endpoints don't apply
        }

        let idx = self.ep_buffer_index(ep_addr);
        let buffers = unsafe { &*self.ep_buffers.get() };
        let buffer = match buffers[idx] {
            Some(b) => b,
            None => return,
        };

        let toggle = self.ep_data_toggles[idx].load(Ordering::SeqCst);
        let mut new_ctrl = (buffer.size as u32) & buffer_control_bits::LENGTH_0_MASK;
        if toggle {
            new_ctrl |= buffer_control_bits::PID_0; // DATA1
        }

        let buf_ctrl = BufferControlAccess::new(self.dpram);
        buf_ctrl.write_with_errata(ep_addr, new_ctrl);

        #[cfg(feature = "defmt")]
        defmt::trace!("USB: re-armed {}", ep_addr);
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
        // Get current data toggle and flip it for next time
        let data1 = self.ep0_in_data_toggle.load(Ordering::SeqCst);
        self.ep0_in_data_toggle.store(!data1, Ordering::SeqCst);

        // Build buffer control value using PAC Val type
        let val = Ep0InBufferControlVal::from_bits(0)
            .with_length_0(len as u32)
            .with_full_0(true)
            .with_pid_0(data1)
            .with_last_0(is_last);

        #[cfg(feature = "defmt")]
        defmt::debug!(
            "EP0 IN: len={}, ctrl={:#010x}, data1={}",
            len,
            val.bits() | Ep0InBufferControl::AVAILABLE_0,
            data1
        );

        // Use errata-safe write sequence via BufferControlRegister trait
        self.dpram
            .ep0_in_buffer_control
            .write_with_errata(val.bits());
    }

    /// Write data to EP0 IN buffer and start the transfer
    ///
    /// This is the preferred method for control IN transfers as it atomically
    /// writes data and starts the transfer.
    pub fn ep0_write_and_start(&self, data: &[u8], is_last: bool) {
        let buffer = unsafe { *self.ep0_in_buffer.get() };
        let len = core::cmp::min(data.len(), buffer.size as usize);

        // Write data to DPRAM buffer using Dpram helper
        self.dpram_mem.copy_to_buffer(buffer.offset, &data[..len]);

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
        // For status ZLP, use DATA1 PID
        let val = Ep0OutBufferControlVal::from_bits(0)
            .with_length_0(max_len as u32)
            .with_pid_0(true); // DATA1 for status stage

        #[cfg(feature = "defmt")]
        defmt::trace!(
            "USB: ep0_expect_out max_len={} ctrl={:#010x}",
            max_len,
            val.bits() | pico_rs_pac::usb_dpram::Ep0OutBufferControl::AVAILABLE_0
        );

        // Use errata-safe write sequence via BufferControlRegister trait
        self.dpram
            .ep0_out_buffer_control
            .write_with_errata(val.bits());
    }

    /// Prepare EP0 OUT for receiving control OUT data stage
    pub fn ep0_expect_out_data(&self, max_len: usize) {
        // For data stage, use DATA1 PID (first data packet after SETUP)
        let val = Ep0OutBufferControlVal::from_bits(0)
            .with_length_0(core::cmp::min(max_len, 64) as u32)
            .with_pid_0(true); // DATA1 for first data packet after SETUP

        #[cfg(feature = "defmt")]
        defmt::debug!(
            "USB: ep0_expect_out_data max_len={} ctrl={:#010x}",
            max_len,
            val.bits() | pico_rs_pac::usb_dpram::Ep0OutBufferControl::AVAILABLE_0
        );

        // Use errata-safe write sequence via BufferControlRegister trait
        self.dpram
            .ep0_out_buffer_control
            .write_with_errata(val.bits());
    }

    /// Send a zero-length packet on EP0 IN (status stage)
    pub fn ep0_send_zlp(&self) {
        self.ep0_start_in(0, true);
    }

    fn ep0_stall_impl(&self, log_warning: bool) {
        self.usb.ep_stall_arm.write_bits(
            pico_rs_pac::usb::EpStallArm::EP0_IN | pico_rs_pac::usb::EpStallArm::EP0_OUT,
        );

        // Set STALL bit in buffer control using PAC methods
        self.dpram
            .ep0_in_buffer_control
            .write_bits(pico_rs_pac::usb_dpram::Ep0InBufferControl::STALL);
        self.dpram
            .ep0_out_buffer_control
            .write_bits(pico_rs_pac::usb_dpram::Ep0OutBufferControl::STALL);

        #[cfg(feature = "defmt")]
        if log_warning {
            defmt::warn!("USB: EP0 stalled");
        }
    }

    /// Stall endpoint 0 and emit a warning log.
    pub fn ep0_stall(&self) {
        self.ep0_stall_impl(true);
    }

    /// Stall endpoint 0 without emitting a warning log.
    pub fn ep0_stall_silent(&self) {
        self.ep0_stall_impl(false);
    }

    /// Stall an endpoint
    pub fn stall(&self, ep_addr: EndpointAddress) {
        let buf_ctrl = BufferControlAccess::new(self.dpram);
        buf_ctrl.set_stall(ep_addr);
    }

    /// Clear stall on an endpoint and reset data toggle
    pub fn unstall(&self, ep_addr: EndpointAddress) {
        let buf_ctrl = BufferControlAccess::new(self.dpram);
        buf_ctrl.clear_stall(ep_addr);

        // Reset data toggle to DATA0 per USB spec
        self.reset_ep_data_toggle(ep_addr);

        #[cfg(feature = "defmt")]
        defmt::debug!("USB: Unstalled {}, reset data toggle", ep_addr);
    }

    /// Check if an endpoint is stalled
    pub fn is_stalled(&self, ep_addr: EndpointAddress) -> bool {
        let buf_ctrl = BufferControlAccess::new(self.dpram);
        buf_ctrl.is_stalled(ep_addr)
    }

    /// Get access to the raw USB registers
    pub fn regs(&self) -> &pico_rs_pac::usb::RegisterBlock {
        self.usb
    }

    /// Get access to the raw USB DPRAM register block
    pub fn dpram(&self) -> &pico_rs_pac::usb_dpram::RegisterBlock {
        self.dpram
    }
}

impl Default for UsbBus {
    fn default() -> Self {
        Self::new()
    }
}
