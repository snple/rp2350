//! RP2350 USB peripheral driver implementing [`UsbBus`].
//!
//! # Usage
//!
//! ```ignore
//! let alloc = UsbBusAllocator::new(Rp2350Bus::new());
//! // … allocate endpoints via alloc …
//! let mut dev = UsbDeviceBuilder::new(&alloc, 0x1234, 0x5678).build();
//! ```
//!
//! # Hardware Errata
//!
//! The RP2350 USB controller requires a specific write sequence for buffer-control
//! registers:
//! 1. Write all fields **except** the AVAILABLE bit.
//! 2. Wait at least 12 CPU cycles.
//! 3. Set only the AVAILABLE bit via read-modify-write.
//!
//! The [`BufferControlRegister::write_with_errata`] helper in `ep_regs` handles
//! this automatically.

use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicBool, Ordering};

use roschi_pac::{RESETS, USB, USB_DPRAM};

use crate::bus::{PollResult, UsbBus};
use crate::dpram::Dpram;
use crate::endpoint::{Direction, EndpointAddress, EndpointType};
use crate::ep_regs::{BufferControlAccess, EndpointControlAccess, buffer_control_bits};
use crate::error::{UsbError, UsbResult};

// ─── Constants ────────────────────────────────────────────────────────────────

const USBCTRL_RESET_BIT: u32 = 1 << 28;

/// EP0 DPRAM buffer offset (fixed by hardware).
const EP0_IN_BUFFER_OFFSET: u16 = 0x100;
/// EP0 OUT buffer follows EP0 IN (64 bytes each).
const EP0_OUT_BUFFER_OFFSET: u16 = 0x100 + 64;

/// EP0 max packet size (fixed at 64 by USB 2.0 full-speed).
pub const CONTROL_MAX_PACKET_SIZE: u16 = 64;
/// Total DPRAM size in bytes.
const DPRAM_SIZE: u16 = 4096;
/// First byte available for dynamically allocated endpoint buffers.
const FIRST_DYNAMIC_OFFSET: u16 = EP0_OUT_BUFFER_OFFSET + CONTROL_MAX_PACKET_SIZE;

// ─── Internal types ───────────────────────────────────────────────────────────

/// A contiguous region of DPRAM assigned to a single endpoint buffer.
#[derive(Clone, Copy, Debug)]
struct EpBuffer {
    /// Byte offset from the start of DPRAM.
    offset: u16,
    /// Allocated size in bytes.
    size: u16,
    /// Endpoint type (needed to re-configure control registers after reset).
    ep_type: EndpointType,
}

/// DPRAM endpoint buffer allocator — bump allocator aligned to 64 bytes.
struct EpAllocator {
    next: u16,
}

impl EpAllocator {
    const fn new() -> Self {
        Self {
            next: FIRST_DYNAMIC_OFFSET,
        }
    }

    fn allocate(&mut self, size: u16) -> UsbResult<(u16, u16)> {
        let aligned = (self.next + 63) & !63;
        let rounded = (size + 63) & !63;
        if aligned + rounded > DPRAM_SIZE {
            return Err(UsbError::EndpointMemoryOverflow);
        }
        self.next = aligned + rounded;
        Ok((aligned, size))
    }
}

// ─── Rp2350Bus ────────────────────────────────────────────────────────────────

/// RP2350 USB peripheral, implementing [`UsbBus`].
///
/// # Construction
///
/// Use [`Rp2350Bus::new()`] (const fn) then pass it to [`UsbBusAllocator::new`].
pub struct Rp2350Bus {
    usb: &'static roschi_pac::usb::RegisterBlock,
    dpram: &'static roschi_pac::usb_dpram::RegisterBlock,
    dpram_mem: Dpram,

    // ── EP0 state ───────────────────────────────────────────────────────────
    /// DATA1 = true after SETUP or status stage.
    ep0_in_data_toggle: AtomicBool,
    /// Set by `poll()` when a SETUP packet is pending; cleared by `read(EP0_OUT)`.
    setup_pending: AtomicBool,

    // ── Non-EP0 state ────────────────────────────────────────────────────────
    allocator: UnsafeCell<EpAllocator>,
    /// Slot layout: IN ep1-15 → indices 0-14, OUT ep1-15 → indices 15-29.
    ep_buffers: UnsafeCell<[Option<EpBuffer>; 30]>,
    ep_data_toggles: [AtomicBool; 30],
}

// Safety: designed for single-threaded embedded; all mutable state is guarded
// by atomics or `UnsafeCell` accessed only from one context.
unsafe impl Send for Rp2350Bus {}
unsafe impl Sync for Rp2350Bus {}

impl Rp2350Bus {
    /// Create a new (uninitialised) RP2350 USB bus.
    ///
    /// Initialisation happens inside [`UsbBus::enable`], which is called by
    /// [`UsbBusAllocator::freeze`].
    ///
    /// # Safety
    /// - Caller must ensure this is only called once (e.g. by only calling from `main()`).
    pub const unsafe fn steal() -> Self {
        Self {
            usb: unsafe { &*(USB::PTR as *const _) },
            dpram: unsafe { &*(USB_DPRAM::PTR as *const _) },
            dpram_mem: Dpram::steal(),
            ep0_in_data_toggle: AtomicBool::new(true),
            setup_pending: AtomicBool::new(false),
            allocator: UnsafeCell::new(EpAllocator::new()),
            ep_buffers: UnsafeCell::new([None; 30]),
            ep_data_toggles: [const { AtomicBool::new(false) }; 30],
        }
    }

    // ── Additional public helpers ─────────────────────────────────────────────

    /// Enable the D+ pull-up (connect to host).
    pub fn connect(&self) {
        self.usb
            .sie_ctrl
            .set_bits(roschi_pac::usb::SieCtrl::PULLUP_EN);
    }

    /// Disable the D+ pull-up (disconnect from host).
    pub fn disconnect(&self) {
        self.usb
            .sie_ctrl
            .clr_bits(roschi_pac::usb::SieCtrl::PULLUP_EN);
    }

    // ── Private helpers ───────────────────────────────────────────────────────

    /// Write 0 directly to a non-EP0 buffer-control register.
    ///
    /// Bypasses the errata write sequence — safe because we are CLEARING
    /// (not setting) the AVAILABLE bit, which is the only bit that requires
    /// the two-step sequence.
    fn clear_ep_buffer_control(&self, ep_addr: EndpointAddress) {
        match (ep_addr.number(), ep_addr.is_in()) {
            (1, true) => self.dpram.ep1_in_buffer_control.write_bits(0),
            (1, false) => self.dpram.ep1_out_buffer_control.write_bits(0),
            (2, true) => self.dpram.ep2_in_buffer_control.write_bits(0),
            (2, false) => self.dpram.ep2_out_buffer_control.write_bits(0),
            (3, true) => self.dpram.ep3_in_buffer_control.write_bits(0),
            (3, false) => self.dpram.ep3_out_buffer_control.write_bits(0),
            (4, true) => self.dpram.ep4_in_buffer_control.write_bits(0),
            (4, false) => self.dpram.ep4_out_buffer_control.write_bits(0),
            (5, true) => self.dpram.ep5_in_buffer_control.write_bits(0),
            (5, false) => self.dpram.ep5_out_buffer_control.write_bits(0),
            (6, true) => self.dpram.ep6_in_buffer_control.write_bits(0),
            (6, false) => self.dpram.ep6_out_buffer_control.write_bits(0),
            (7, true) => self.dpram.ep7_in_buffer_control.write_bits(0),
            (7, false) => self.dpram.ep7_out_buffer_control.write_bits(0),
            (8, true) => self.dpram.ep8_in_buffer_control.write_bits(0),
            (8, false) => self.dpram.ep8_out_buffer_control.write_bits(0),
            (9, true) => self.dpram.ep9_in_buffer_control.write_bits(0),
            (9, false) => self.dpram.ep9_out_buffer_control.write_bits(0),
            (10, true) => self.dpram.ep10_in_buffer_control.write_bits(0),
            (10, false) => self.dpram.ep10_out_buffer_control.write_bits(0),
            (11, true) => self.dpram.ep11_in_buffer_control.write_bits(0),
            (11, false) => self.dpram.ep11_out_buffer_control.write_bits(0),
            (12, true) => self.dpram.ep12_in_buffer_control.write_bits(0),
            (12, false) => self.dpram.ep12_out_buffer_control.write_bits(0),
            (13, true) => self.dpram.ep13_in_buffer_control.write_bits(0),
            (13, false) => self.dpram.ep13_out_buffer_control.write_bits(0),
            (14, true) => self.dpram.ep14_in_buffer_control.write_bits(0),
            (14, false) => self.dpram.ep14_out_buffer_control.write_bits(0),
            (15, true) => self.dpram.ep15_in_buffer_control.write_bits(0),
            (15, false) => self.dpram.ep15_out_buffer_control.write_bits(0),
            _ => {}
        }
    }

    fn ep_index(&self, ep_addr: EndpointAddress) -> usize {
        let n = (ep_addr.number() as usize) - 1;
        if ep_addr.is_in() { n } else { n + 15 }
    }

    fn clear_buffer_status_bit(&self, ep_addr: EndpointAddress) {
        // Even bits = IN, odd bits = OUT; bit0 = EP0 IN / bit1 = EP0 OUT, etc.
        let bit = 1u32 << (ep_addr.number() * 2 + if ep_addr.is_in() { 0 } else { 1 });
        self.usb.buff_status.write_bits(bit);
    }

    fn init_hardware(&mut self) {
        let resets = unsafe { &*RESETS::PTR };

        // Force USB peripheral into reset to guarantee D+ pull-up is
        // de-asserted.  Hold it in reset for ≥20 ms so the host detects
        // a disconnect (USB 2.0 requires ≥2.5 µs SE0; Linux needs ~10 ms
        // in practice).  Without this, a fast release-mode reboot may
        // re-assert PULLUP_EN before the host notices the device went away,
        // causing the host to skip re-enumeration.
        resets.reset.set_bits(USBCTRL_RESET_BIT);
        // 20 ms @ 150 MHz = 3 000 000 cycles
        roschi_rt::asm::delay(3_000_000);

        // Bring USB out of reset.
        resets.reset.clr_bits(USBCTRL_RESET_BIT);
        while resets.reset_done.read_bits() & USBCTRL_RESET_BIT == 0 {
            roschi_rt::asm::nop();
        }

        // Clear DPRAM.
        self.dpram_mem.clear();

        // Use internal PHY.
        self.usb
            .usb_muxing
            .write_bits(roschi_pac::usb::UsbMuxing::TO_PHY | roschi_pac::usb::UsbMuxing::SOFTCON);

        // Override VBUS — single write to avoid a window where the override
        // bits are not yet effective (rp-hal does a single modify).
        self.usb.usb_pwr.write_bits(
            roschi_pac::usb::UsbPwr::VBUS_EN
                | roschi_pac::usb::UsbPwr::VBUS_DETECT
                | roschi_pac::usb::UsbPwr::VBUS_EN_OVERRIDE_EN
                | roschi_pac::usb::UsbPwr::VBUS_DETECT_OVERRIDE_EN,
        );

        // Enable controller in device mode.
        self.usb
            .main_ctrl
            .write_bits(roschi_pac::usb::MainCtrl::CONTROLLER_EN);
        self.usb
            .sie_ctrl
            .write_bits(roschi_pac::usb::SieCtrl::EP0_INT_1BUF);

        // Clear any stale status flags before enabling interrupts.
        // The controller may have latched events (e.g. BUS_RESET, SUSPENDED)
        // between CONTROLLER_EN and here; clearing prevents a spurious first
        // interrupt that could confuse the upper-layer state machine.
        self.usb.sie_status.write_bits(0xFFFF_FFFF); // W1C: clear all flags
        self.usb.buff_status.write_bits(0xFFFF_FFFF); // W1C: clear all

        // Enable required interrupts.
        self.usb.inte.write_bits(
            roschi_pac::usb::Inte::BUS_RESET
                | roschi_pac::usb::Inte::SETUP_REQ
                | roschi_pac::usb::Inte::BUFF_STATUS
                | roschi_pac::usb::Inte::DEV_SUSPEND
                | roschi_pac::usb::Inte::DEV_RESUME_FROM_HOST,
        );

        // Clear EP0 buffer control registers.
        self.dpram.ep0_in_buffer_control.write_bits(0);
        self.dpram.ep0_out_buffer_control.write_bits(0);

        #[cfg(feature = "defmt")]
        defmt::info!("USB: hardware initialised");
    }

    /// Reset EP0 state (called on bus reset).
    ///
    /// Matches rp-hal `ep_reset_all` pattern:
    /// 1. Re-assert EP0_INT_1BUF in SIE_CTRL (defensive; survives spurious clears).
    /// 2. Set EP0 IN buffer control to PID_0 (DATA1 for first IN after SETUP).
    /// 3. Arm EP0 OUT with AVAILABLE via errata-safe two-step write + DSB.
    fn reset_ep0(&self) {
        // Re-assert EP0_INT_1BUF (rp-hal does this on every reset).
        self.usb
            .sie_ctrl
            .set_bits(roschi_pac::usb::SieCtrl::EP0_INT_1BUF);

        // Clear EP0 IN — PID_0 set so first response uses DATA1.
        self.dpram
            .ep0_in_buffer_control
            .write_bits(roschi_pac::usb_dpram::Ep0InBufferControl::PID_0);

        // Arm EP0 OUT for first SETUP packet.
        // Step 1: write config without AVAILABLE.
        self.dpram
            .ep0_out_buffer_control
            .write_bits(roschi_pac::usb_dpram::Ep0OutBufferControl::PID_0);
        // Step 2: DSB + delay (errata: AVAILABLE must be set ≥12 cycles after config write).
        roschi_rt::asm::dsb();
        roschi_rt::asm::delay(12);
        // Step 3: set AVAILABLE via atomic SET alias.
        self.dpram
            .ep0_out_buffer_control
            .set_bits(roschi_pac::usb_dpram::Ep0OutBufferControl::AVAILABLE_0);
        roschi_rt::asm::dsb();

        self.ep0_in_data_toggle.store(true, Ordering::SeqCst);
        self.setup_pending.store(false, Ordering::SeqCst);
    }

    /// Reset all non-EP0 endpoint state (called on bus reset or SET_CONFIGURATION(0)).
    ///
    /// Matches rp-hal's `ep_reset_all` pattern: endpoint allocations are retained
    /// across reset, but control registers are re-configured from stored info because
    /// `init_hardware()` clears DPRAM (via RESETS + dpram_mem.clear()).
    fn reset_non_ep0(&self) {
        for t in &self.ep_data_toggles {
            t.store(false, Ordering::SeqCst);
        }

        let buffers = unsafe { &*self.ep_buffers.get() };
        let bca = BufferControlAccess::new(self.dpram);
        let ep_ctrl = EndpointControlAccess::new(self.dpram);
        for n in 1u8..=15 {
            let in_addr = EndpointAddress::in_(n);
            let out_addr = EndpointAddress::out(n);
            let in_idx = self.ep_index(in_addr);
            let out_idx = self.ep_index(out_addr);

            if let Some(buf) = buffers[in_idx] {
                // Re-configure endpoint control register (cleared by DPRAM clear in init_hardware).
                ep_ctrl.configure(in_addr, buf.ep_type, buf.offset);
                // Clear buffer control — AVAILABLE=0 so hardware NAKs IN tokens until
                // software calls write() to arm the endpoint.
                self.clear_ep_buffer_control(in_addr);
            }
            if let Some(buf) = buffers[out_idx] {
                // Re-configure endpoint control register.
                ep_ctrl.configure(out_addr, buf.ep_type, buf.offset);
                // Re-arm: DATA0 toggle, AVAILABLE=1, length=max_packet.
                let ctrl = (buf.size as u32) & buffer_control_bits::LENGTH_0_MASK;
                bca.write_with_errata(out_addr, ctrl);
            }
        }

        self.usb.buff_status.write_bits(0xFFFF_FFFF);
    }
}

// ─── UsbBus implementation ────────────────────────────────────────────────────

impl UsbBus for Rp2350Bus {
    fn alloc_ep(
        &mut self,
        dir: Direction,
        addr: Option<EndpointAddress>,
        ep_type: EndpointType,
        max_packet: u16,
        _interval: u8,
    ) -> UsbResult<EndpointAddress> {
        // ── EP0 allocation ───────────────────────────────────────────────────
        if let Some(a) = addr
            && a.number() == 0
        {
            return Ok(a); // EP0 buffers are pre-placed at fixed offsets.
        }

        // ── Non-EP0: find a free slot ────────────────────────────────────────
        let is_in = dir == Direction::In;
        let buffers = unsafe { &mut *self.ep_buffers.get() };

        let ep_number = match addr {
            Some(a) => {
                // Specific address requested.
                let n = a.number() as usize;
                let idx = if is_in { n - 1 } else { n - 1 + 15 };
                if buffers[idx].is_some() {
                    return Err(UsbError::EndpointOverflow);
                }
                a.number()
            }
            None => {
                // Auto-assign.
                (1u8..=15)
                    .find(|&n| {
                        let idx = (n - 1) as usize + if is_in { 0 } else { 15 };
                        buffers[idx].is_none()
                    })
                    .ok_or(UsbError::EndpointOverflow)?
            }
        };

        let ep_addr = EndpointAddress::new(ep_number, dir);
        let idx = self.ep_index(ep_addr);

        // Allocate DPRAM.
        let (offset, size) = unsafe { (*self.allocator.get()).allocate(max_packet) }?;
        buffers[idx] = Some(EpBuffer {
            offset,
            size,
            ep_type,
        });
        self.ep_data_toggles[idx].store(false, Ordering::SeqCst);

        #[cfg(feature = "defmt")]
        defmt::debug!(
            "USB: alloc_ep {} type={:?} off={:#x} size={}",
            ep_addr,
            ep_type,
            offset,
            max_packet
        );

        Ok(ep_addr)
    }

    fn enable(&mut self) {
        self.init_hardware();
    }

    fn reset(&self) {
        // Clear bus-reset flag & all buffer status (matches rp-hal reset()).
        self.usb
            .sie_status
            .write_bits(roschi_pac::usb::SieStatus::BUS_RESET);
        self.usb.buff_status.write_bits(0xFFFF_FFFF);

        // Reset address.
        self.usb.addr_endp.write_address(0);

        // Reset EP0.
        self.reset_ep0();

        // Reset all non-EP0 endpoints.
        self.reset_non_ep0();

        #[cfg(feature = "defmt")]
        defmt::info!("USB: reset");
    }

    fn set_device_address(&self, addr: u8) {
        self.usb.addr_endp.write_address(addr as u32);
        #[cfg(feature = "defmt")]
        defmt::debug!("USB: address = {}", addr);
    }

    fn poll(&self) -> PollResult {
        let ints = self.usb.ints.read_bits();

        // ── Bus reset ────────────────────────────────────────────────────────
        if ints & roschi_pac::usb::Ints::BUS_RESET != 0 {
            return PollResult::Reset;
        }

        // ── SETUP / buffer status ────────────────────────────────────────────
        // Check for data events BEFORE suspend/resume so a pending SETUP is
        // never masked by a concurrent DEV_SUSPEND (matches rp-hal ordering).
        let has_setup = ints & roschi_pac::usb::Ints::SETUP_REQ != 0;
        let has_buffer = ints & roschi_pac::usb::Ints::BUFF_STATUS != 0;

        if !has_setup && !has_buffer {
            // No data events — check suspend/resume.
            if ints & roschi_pac::usb::Ints::DEV_SUSPEND != 0 {
                self.usb
                    .sie_status
                    .write_bits(roschi_pac::usb::SieStatus::SUSPENDED);
                return PollResult::Suspend;
            }
            if ints & roschi_pac::usb::Ints::DEV_RESUME_FROM_HOST != 0 {
                self.usb
                    .sie_status
                    .write_bits(roschi_pac::usb::SieStatus::RESUME);
                return PollResult::Resume;
            }
            return PollResult::None;
        }

        let mut ep_setup: u16 = 0;
        let mut ep_out: u16 = 0;
        let mut ep_in_complete: u16 = 0;

        // ── SETUP packet on EP0 ──────────────────────────────────────────────
        if has_setup {
            // CRITICAL: clear EP0 IN immediately to avoid sending stale data.
            self.dpram.ep0_in_buffer_control.write_bits(0);
            self.setup_pending.store(true, Ordering::SeqCst);
            ep_setup |= 1; // bit 0 = EP0
        }

        // ── Buffer status bitmap ─────────────────────────────────────────────
        if has_buffer {
            let status = self.usb.buff_status.read_bits();

            // Bit mapping: bit(2n)   = EP_n IN complete
            //              bit(2n+1) = EP_n OUT data ready
            for n in 0u8..=15 {
                let in_bit = 1u32 << (n * 2);
                let out_bit = 1u32 << (n * 2 + 1);

                if status & in_bit != 0 {
                    ep_in_complete |= 1u16 << n;
                    self.usb.buff_status.write_bits(in_bit);
                }
                if status & out_bit != 0 {
                    if n == 0 && self.setup_pending.load(Ordering::SeqCst) {
                        // EP0 OUT from SETUP packet — will be read as setup.
                        self.usb.buff_status.write_bits(out_bit);
                    } else {
                        ep_out |= 1u16 << n;
                        // Note: do NOT clear out_bit here; read() clears it.
                    }
                }
            }
        }

        if ep_setup | ep_out | ep_in_complete != 0 {
            PollResult::Data {
                ep_out,
                ep_in_complete,
                ep_setup,
            }
        } else {
            PollResult::None
        }
    }

    fn write(&self, addr: EndpointAddress, buf: &[u8]) -> UsbResult<usize> {
        if !addr.is_in() && addr.number() != 0 {
            return Err(UsbError::InvalidEndpoint);
        }

        let (ep_buf, toggle) = if addr.number() == 0 {
            let b = EpBuffer {
                offset: EP0_IN_BUFFER_OFFSET,
                size: CONTROL_MAX_PACKET_SIZE,
                ep_type: EndpointType::Control,
            };
            let t = self.ep0_in_data_toggle.load(Ordering::SeqCst);
            (b, t)
        } else {
            let idx = self.ep_index(addr);
            let buffers = unsafe { &*self.ep_buffers.get() };
            let b = buffers[idx].ok_or(UsbError::InvalidEndpoint)?;
            let t = self.ep_data_toggles[idx].load(Ordering::SeqCst);
            (b, t)
        };

        // For non-EP0 IN: check AVAILABLE_0 directly from the hardware register.
        // The RP2350 clears AVAILABLE_0 after sending the IN packet, matching
        // rp-hal's approach (no software tracking needed).
        if addr.number() != 0 {
            let ctrl = BufferControlAccess::new(self.dpram).read(addr);
            if ctrl & buffer_control_bits::AVAILABLE_0 != 0 {
                return Err(UsbError::WouldBlock);
            }
        }

        let len = core::cmp::min(buf.len(), ep_buf.size as usize);
        self.dpram_mem.copy_to_buffer(ep_buf.offset, &buf[..len]);

        // Toggle DATA0/DATA1.
        if addr.number() == 0 {
            self.ep0_in_data_toggle.store(!toggle, Ordering::SeqCst);
        } else {
            let idx = self.ep_index(addr);
            self.ep_data_toggles[idx].store(!toggle, Ordering::SeqCst);
        }

        let mut ctrl = (len as u32) | buffer_control_bits::FULL_0;
        if toggle {
            ctrl |= buffer_control_bits::PID_0;
        } // DATA1
        BufferControlAccess::new(self.dpram).write_with_errata(addr, ctrl);

        Ok(len)
    }

    fn read(&self, addr: EndpointAddress, buf: &mut [u8]) -> UsbResult<usize> {
        if addr.is_in() && addr.number() != 0 {
            return Err(UsbError::InvalidEndpoint);
        }

        // ── EP0: check for pending SETUP packet ──────────────────────────────
        if addr.number() == 0 && self.setup_pending.load(Ordering::SeqCst) {
            self.setup_pending.store(false, Ordering::SeqCst);
            // Clear the SETUP_REQ interrupt.
            self.usb
                .sie_status
                .write_bits(roschi_pac::usb::SieStatus::SETUP_REC);
            // Reset data toggle — control IN starts at DATA1.
            self.ep0_in_data_toggle.store(true, Ordering::SeqCst);

            // Clear any outstanding EP0 OUT buff_status (e.g. discarded ZLP).
            self.clear_buffer_status_bit(EndpointAddress::EP0_OUT);

            let lo = self.dpram.setup_packet_low.read_bits();
            let hi = self.dpram.setup_packet_high.read_bits();
            let lo_bytes = lo.to_le_bytes();
            let hi_bytes = hi.to_le_bytes();
            let pkt = lo_bytes.iter().chain(hi_bytes.iter()).copied();
            let len = core::cmp::min(8, buf.len());
            for (i, b) in pkt.take(len).enumerate() {
                buf[i] = b;
            }

            // Determine if the host will send a DATA OUT or STATUS OUT phase.
            // - IN requests (bmRequestType bit 7 set): host sends STATUS OUT ZLP
            // - OUT requests with wLength > 0: host sends DATA OUT
            // - No-data OUT requests (wLength == 0): no OUT phase; device sends STATUS IN
            let expect_out = if len >= 8 {
                let is_in = (buf[0] & 0x80) != 0;
                let w_length = u16::from(buf[6]) | (u16::from(buf[7]) << 8);
                is_in || w_length != 0
            } else {
                true // short read — arm defensively
            };

            // Re-arm EP0 OUT for the data/status phase (DATA1 after SETUP).
            let ctrl = (CONTROL_MAX_PACKET_SIZE as u32) & buffer_control_bits::LENGTH_0_MASK
                | buffer_control_bits::PID_0;
            if expect_out {
                BufferControlAccess::new(self.dpram)
                    .write_with_errata(EndpointAddress::EP0_OUT, ctrl);
            } else {
                // No OUT phase expected — set clean state without AVAILABLE.
                self.dpram.ep0_out_buffer_control.write_bits(ctrl);
            }

            #[cfg(feature = "defmt")]
            defmt::trace!("USB: SETUP {:02x}", &buf[..len]);

            return Ok(len);
        }

        // ── Regular OUT / EP0 OUT data stage ─────────────────────────────────
        let ep_buf = if addr.number() == 0 {
            EpBuffer {
                offset: EP0_OUT_BUFFER_OFFSET,
                size: CONTROL_MAX_PACKET_SIZE,
                ep_type: EndpointType::Control,
            }
        } else {
            let idx = self.ep_index(addr);
            let buffers = unsafe { &*self.ep_buffers.get() };
            buffers[idx].ok_or(UsbError::InvalidEndpoint)?
        };

        let buf_ctrl = BufferControlAccess::new(self.dpram);
        let ctrl = buf_ctrl.read(addr);

        if ctrl & buffer_control_bits::FULL_0 == 0 {
            return Err(UsbError::WouldBlock);
        }

        // Clear BUFF_STATUS BEFORE reading to prevent immediate re-interrupt.
        self.clear_buffer_status_bit(addr);

        let received = (ctrl & buffer_control_bits::LENGTH_0_MASK) as usize;
        let len = core::cmp::min(received, buf.len());
        self.dpram_mem
            .copy_from_buffer(ep_buf.offset, &mut buf[..len]);

        // Advance data toggle for non-EP0.
        if addr.number() != 0 {
            let idx = self.ep_index(addr);
            let t = self.ep_data_toggles[idx].load(Ordering::SeqCst);
            self.ep_data_toggles[idx].store(!t, Ordering::SeqCst);
        }

        // Re-arm EP0 OUT immediately (control transfers must always be ready).
        // Re-arm non-EP0 OUT to match rp-hal / usb-device convention: after
        // read() returns Ok, the endpoint is immediately ready for the next packet.
        {
            let idx = if addr.number() == 0 {
                // EP0: always DATA1 after a completed data stage.
                let ctrl = (ep_buf.size as u32) & buffer_control_bits::LENGTH_0_MASK
                    | buffer_control_bits::PID_0;
                buf_ctrl.write_with_errata(addr, ctrl);
                return Ok(len);
            } else {
                self.ep_index(addr)
            };
            let toggle = self.ep_data_toggles[idx].load(Ordering::SeqCst);
            let mut ctrl = (ep_buf.size as u32) & buffer_control_bits::LENGTH_0_MASK;
            if toggle {
                ctrl |= buffer_control_bits::PID_0;
            }
            BufferControlAccess::new(self.dpram).write_with_errata(addr, ctrl);
        }

        Ok(len)
    }

    fn set_stalled(&self, addr: EndpointAddress, stalled: bool) {
        if addr.number() == 0 {
            if stalled {
                // Arm both EP0 IN and OUT stall bits.
                self.usb.ep_stall_arm.write_bits(
                    roschi_pac::usb::EpStallArm::EP0_IN | roschi_pac::usb::EpStallArm::EP0_OUT,
                );
                self.dpram
                    .ep0_in_buffer_control
                    .write_bits(roschi_pac::usb_dpram::Ep0InBufferControl::STALL);
                self.dpram
                    .ep0_out_buffer_control
                    .write_bits(roschi_pac::usb_dpram::Ep0OutBufferControl::STALL);
            } else {
                self.dpram.ep0_in_buffer_control.write_bits(0);
                // Re-arm EP0 OUT with DATA1 and AVAILABLE so the host's STATUS/DATA
                // OUT ZLP is accepted (writing 0 would clear AVAILABLE and cause NAK).
                let ctrl = (CONTROL_MAX_PACKET_SIZE as u32) & buffer_control_bits::LENGTH_0_MASK
                    | buffer_control_bits::PID_0; // DATA1 expected after SETUP
                BufferControlAccess::new(self.dpram)
                    .write_with_errata(EndpointAddress::EP0_OUT, ctrl);
            }
        } else {
            let bca = BufferControlAccess::new(self.dpram);
            if stalled {
                bca.set_stall(addr);
            } else {
                bca.clear_stall(addr);
                // Reset data toggle on unstall (USB §9.4.5).
                if addr.number() != 0 {
                    let idx = self.ep_index(addr);
                    self.ep_data_toggles[idx].store(false, Ordering::SeqCst);
                }
            }
        }
    }

    fn is_stalled(&self, addr: EndpointAddress) -> bool {
        if addr.number() == 0 {
            let ctrl = self.dpram.ep0_in_buffer_control.read_bits();
            ctrl & roschi_pac::usb_dpram::Ep0InBufferControl::STALL != 0
        } else {
            BufferControlAccess::new(self.dpram).is_stalled(addr)
        }
    }

    fn suspend(&self) {
        #[cfg(feature = "defmt")]
        defmt::debug!("USB: suspend");
    }

    fn resume(&self) {
        #[cfg(feature = "defmt")]
        defmt::debug!("USB: resume");
    }
}
