//! Endpoint hardware abstraction
//!
//! This module provides type-safe access to the RP2350's USB endpoint control
//! and buffer control registers through the PAC.
//!
//! ## Architecture
//!
//! The USB DPRAM contains two types of endpoint registers:
//!
//! 1. **Endpoint Control Registers** (EP1-EP15): Configure endpoint type, buffer address, etc.
//!    - EP0 doesn't have control registers (it's always a control endpoint)
//!    - Located at offsets 0x08-0x7F in DPRAM
//!
//! 2. **Buffer Control Registers** (EP0-EP15): Control data transfer state
//!    - Located at offsets 0x80-0xFF in DPRAM
//!    - Requires special "errata-safe" write sequence (write config, delay, set AVAILABLE)
//!
//! ## Hardware Errata
//!
//! The RP2350 USB controller requires that the AVAILABLE bit in buffer control
//! registers be set separately from other bits, with a delay in between.

use rp2350_pac::usb_dpram::{self, *};

use crate::endpoint::{EndpointAddress, EndpointType};

/// Delay cycles required between buffer control writes (RP2350 errata)
const BUFFER_CONTROL_DELAY_CYCLES: u32 = 12;

/// Buffer control common bits
pub mod buffer_control_bits {
    /// Buffer 0 full bit
    pub const FULL_0: u32 = 1 << 15;
    /// Buffer 0 last bit
    pub const LAST_0: u32 = 1 << 14;
    /// Buffer 0 PID (DATA1) bit
    pub const PID_0: u32 = 1 << 13;
    /// Reset buffer select to buffer 0
    pub const RESET: u32 = 1 << 12;
    /// Stall bit
    pub const STALL: u32 = 1 << 11;
    /// Buffer 0 available bit
    pub const AVAILABLE_0: u32 = 1 << 10;
    /// Length mask (bits 9:0)
    pub const LENGTH_0_MASK: u32 = 0x3FF;
}

/// Endpoint control common bits
pub mod ep_control_bits {
    /// Enable bit
    pub const ENABLE: u32 = 1 << 31;
    /// Double buffered bit
    pub const DOUBLE_BUFFERED: u32 = 1 << 30;
    /// Interrupt per buffer bit
    pub const INTERRUPT_PER_BUFF: u32 = 1 << 29;
    /// Interrupt per double buffer bit
    pub const INTERRUPT_PER_DOUBLE_BUFF: u32 = 1 << 28;
    /// Endpoint type offset
    pub const ENDPOINT_TYPE_OFFSET: u32 = 26;
    /// Endpoint type mask
    pub const ENDPOINT_TYPE_MASK: u32 = 0x3 << 26;
    /// Buffer address mask
    pub const BUFFER_ADDRESS_MASK: u32 = 0xFFFF;

    /// Endpoint type: Control
    pub const EP_TYPE_CONTROL: u32 = 0;
    /// Endpoint type: Isochronous
    pub const EP_TYPE_ISOCHRONOUS: u32 = 1;
    /// Endpoint type: Bulk
    pub const EP_TYPE_BULK: u32 = 2;
    /// Endpoint type: Interrupt
    pub const EP_TYPE_INTERRUPT: u32 = 3;
}

/// Trait for buffer control register operations
///
/// This trait provides a unified interface for all endpoint buffer control registers.
/// It includes the errata-safe write sequence.
pub trait BufferControlRegister {
    /// Constant for AVAILABLE_0 bit
    const AVAILABLE_0: u32 = buffer_control_bits::AVAILABLE_0;
    /// Constant for STALL bit
    const STALL: u32 = buffer_control_bits::STALL;
    /// Constant for FULL_0 bit
    const FULL_0: u32 = buffer_control_bits::FULL_0;

    /// Read the raw register value
    fn read_raw(&self) -> u32;

    /// Write the raw register value (bypasses errata handling - use with caution)
    fn write_raw(&self, val: u32);

    /// Modify the register value
    fn modify_raw<F: FnOnce(u32) -> u32>(&self, f: F);

    /// Set bits atomically
    fn set_bits_atomic(&self, bits: u32);

    /// Clear bits atomically
    fn clear_bits_atomic(&self, bits: u32);

    /// Write buffer control with proper errata handling
    ///
    /// This function:
    /// 1. Writes all fields EXCEPT AVAILABLE
    /// 2. DSB to ensure write completes
    /// 3. Delay for buffer control to settle
    /// 4. Read-modify-write to set AVAILABLE bit only
    #[inline(never)] // Prevent inlining to ensure proper sequencing
    fn write_with_errata(&self, val: u32) {
        // Step 1: Write configuration without AVAILABLE bit
        self.write_raw(val & !Self::AVAILABLE_0);

        // Step 2: Memory barrier to ensure write is complete
        rp2350_rt::asm::dsb();

        // Step 3: Wait for buffer control to settle (RP2350 errata)
        rp2350_rt::asm::delay(BUFFER_CONTROL_DELAY_CYCLES);

        // Step 4: Set only AVAILABLE bit via read-modify-write
        // This preserves all other bits we just wrote
        self.modify_raw(|v| v | Self::AVAILABLE_0);

        // Final barrier to ensure AVAILABLE write is visible
        rp2350_rt::asm::dsb();
    }

    /// Check if stalled
    fn is_stalled(&self) -> bool {
        (self.read_raw() & Self::STALL) != 0
    }

    /// Set stall
    fn set_stall(&self) {
        self.set_bits_atomic(Self::STALL);
    }

    /// Clear stall
    fn clear_stall(&self) {
        self.clear_bits_atomic(Self::STALL);
    }

    /// Check if buffer is full (has data ready)
    fn is_full(&self) -> bool {
        (self.read_raw() & Self::FULL_0) != 0
    }

    /// Get the length field
    fn length(&self) -> u16 {
        (self.read_raw() & buffer_control_bits::LENGTH_0_MASK) as u16
    }
}

/// Trait for endpoint control register operations (EP1-EP15)
pub trait EndpointControlRegister {
    /// Read the raw register value
    fn read_raw(&self) -> u32;

    /// Write the raw register value
    fn write_raw(&self, val: u32);

    /// Configure the endpoint
    fn configure(&self, ep_type: EndpointType, buffer_offset: u16) {
        use ep_control_bits::*;

        let ep_type_val = match ep_type {
            EndpointType::Control => EP_TYPE_CONTROL,
            EndpointType::Isochronous { .. } => EP_TYPE_ISOCHRONOUS,
            EndpointType::Bulk => EP_TYPE_BULK,
            EndpointType::Interrupt => EP_TYPE_INTERRUPT,
        };

        let val = ENABLE
            | INTERRUPT_PER_BUFF
            | (ep_type_val << ENDPOINT_TYPE_OFFSET)
            | ((buffer_offset as u32) & BUFFER_ADDRESS_MASK);

        self.write_raw(val);
    }

    /// Disable the endpoint
    fn disable(&self) {
        self.write_raw(0);
    }

    /// Check if enabled
    fn is_enabled(&self) -> bool {
        (self.read_raw() & ep_control_bits::ENABLE) != 0
    }
}

// ============================================================================
// Macro implementations for Buffer Control registers
// ============================================================================

macro_rules! impl_buffer_control {
    ($type:ty) => {
        impl BufferControlRegister for $type {
            #[inline(always)]
            fn read_raw(&self) -> u32 {
                self.read_bits()
            }

            #[inline(always)]
            fn write_raw(&self, val: u32) {
                self.write_bits(val);
            }

            #[inline(always)]
            fn modify_raw<F: FnOnce(u32) -> u32>(&self, f: F) {
                self.modify_bits(f);
            }

            #[inline(always)]
            fn set_bits_atomic(&self, bits: u32) {
                self.set_bits(bits);
            }

            #[inline(always)]
            fn clear_bits_atomic(&self, bits: u32) {
                self.clr_bits(bits);
            }
        }
    };
}

// EP0 buffer control
impl_buffer_control!(Ep0InBufferControl);
impl_buffer_control!(Ep0OutBufferControl);

// EP1-15 buffer control
impl_buffer_control!(Ep1InBufferControl);
impl_buffer_control!(Ep1OutBufferControl);
impl_buffer_control!(Ep2InBufferControl);
impl_buffer_control!(Ep2OutBufferControl);
impl_buffer_control!(Ep3InBufferControl);
impl_buffer_control!(Ep3OutBufferControl);
impl_buffer_control!(Ep4InBufferControl);
impl_buffer_control!(Ep4OutBufferControl);
impl_buffer_control!(Ep5InBufferControl);
impl_buffer_control!(Ep5OutBufferControl);
impl_buffer_control!(Ep6InBufferControl);
impl_buffer_control!(Ep6OutBufferControl);
impl_buffer_control!(Ep7InBufferControl);
impl_buffer_control!(Ep7OutBufferControl);
impl_buffer_control!(Ep8InBufferControl);
impl_buffer_control!(Ep8OutBufferControl);
impl_buffer_control!(Ep9InBufferControl);
impl_buffer_control!(Ep9OutBufferControl);
impl_buffer_control!(Ep10InBufferControl);
impl_buffer_control!(Ep10OutBufferControl);
impl_buffer_control!(Ep11InBufferControl);
impl_buffer_control!(Ep11OutBufferControl);
impl_buffer_control!(Ep12InBufferControl);
impl_buffer_control!(Ep12OutBufferControl);
impl_buffer_control!(Ep13InBufferControl);
impl_buffer_control!(Ep13OutBufferControl);
impl_buffer_control!(Ep14InBufferControl);
impl_buffer_control!(Ep14OutBufferControl);
impl_buffer_control!(Ep15InBufferControl);
impl_buffer_control!(Ep15OutBufferControl);

// ============================================================================
// Macro implementations for Endpoint Control registers
// ============================================================================

macro_rules! impl_ep_control {
    ($type:ty) => {
        impl EndpointControlRegister for $type {
            #[inline(always)]
            fn read_raw(&self) -> u32 {
                self.read_bits()
            }

            #[inline(always)]
            fn write_raw(&self, val: u32) {
                self.write_bits(val);
            }
        }
    };
}

// EP1-15 control registers
impl_ep_control!(Ep1InControl);
impl_ep_control!(Ep1OutControl);
impl_ep_control!(Ep2InControl);
impl_ep_control!(Ep2OutControl);
impl_ep_control!(Ep3InControl);
impl_ep_control!(Ep3OutControl);
impl_ep_control!(Ep4InControl);
impl_ep_control!(Ep4OutControl);
impl_ep_control!(Ep5InControl);
impl_ep_control!(Ep5OutControl);
impl_ep_control!(Ep6InControl);
impl_ep_control!(Ep6OutControl);
impl_ep_control!(Ep7InControl);
impl_ep_control!(Ep7OutControl);
impl_ep_control!(Ep8InControl);
impl_ep_control!(Ep8OutControl);
impl_ep_control!(Ep9InControl);
impl_ep_control!(Ep9OutControl);
impl_ep_control!(Ep10InControl);
impl_ep_control!(Ep10OutControl);
impl_ep_control!(Ep11InControl);
impl_ep_control!(Ep11OutControl);
impl_ep_control!(Ep12InControl);
impl_ep_control!(Ep12OutControl);
impl_ep_control!(Ep13InControl);
impl_ep_control!(Ep13OutControl);
impl_ep_control!(Ep14InControl);
impl_ep_control!(Ep14OutControl);
impl_ep_control!(Ep15InControl);
impl_ep_control!(Ep15OutControl);

// ============================================================================
// Endpoint access by number
// ============================================================================

/// Bind the buffer-control register for a given endpoint address.
///
/// The 32 EP0–EP15 buffer-control register types are generated as distinct
/// PAC types, so a macro is the simplest way to share the same operation
/// across all of them.
macro_rules! buffer_control_match {
    ($dpram:expr, $ep_addr:expr, |$reg:ident| $body:expr) => {{
        match ($ep_addr.number(), $ep_addr.is_in()) {
            (0, true) => {
                let $reg = &$dpram.ep0_in_buffer_control;
                $body
            }
            (0, false) => {
                let $reg = &$dpram.ep0_out_buffer_control;
                $body
            }
            (1, true) => {
                let $reg = &$dpram.ep1_in_buffer_control;
                $body
            }
            (1, false) => {
                let $reg = &$dpram.ep1_out_buffer_control;
                $body
            }
            (2, true) => {
                let $reg = &$dpram.ep2_in_buffer_control;
                $body
            }
            (2, false) => {
                let $reg = &$dpram.ep2_out_buffer_control;
                $body
            }
            (3, true) => {
                let $reg = &$dpram.ep3_in_buffer_control;
                $body
            }
            (3, false) => {
                let $reg = &$dpram.ep3_out_buffer_control;
                $body
            }
            (4, true) => {
                let $reg = &$dpram.ep4_in_buffer_control;
                $body
            }
            (4, false) => {
                let $reg = &$dpram.ep4_out_buffer_control;
                $body
            }
            (5, true) => {
                let $reg = &$dpram.ep5_in_buffer_control;
                $body
            }
            (5, false) => {
                let $reg = &$dpram.ep5_out_buffer_control;
                $body
            }
            (6, true) => {
                let $reg = &$dpram.ep6_in_buffer_control;
                $body
            }
            (6, false) => {
                let $reg = &$dpram.ep6_out_buffer_control;
                $body
            }
            (7, true) => {
                let $reg = &$dpram.ep7_in_buffer_control;
                $body
            }
            (7, false) => {
                let $reg = &$dpram.ep7_out_buffer_control;
                $body
            }
            (8, true) => {
                let $reg = &$dpram.ep8_in_buffer_control;
                $body
            }
            (8, false) => {
                let $reg = &$dpram.ep8_out_buffer_control;
                $body
            }
            (9, true) => {
                let $reg = &$dpram.ep9_in_buffer_control;
                $body
            }
            (9, false) => {
                let $reg = &$dpram.ep9_out_buffer_control;
                $body
            }
            (10, true) => {
                let $reg = &$dpram.ep10_in_buffer_control;
                $body
            }
            (10, false) => {
                let $reg = &$dpram.ep10_out_buffer_control;
                $body
            }
            (11, true) => {
                let $reg = &$dpram.ep11_in_buffer_control;
                $body
            }
            (11, false) => {
                let $reg = &$dpram.ep11_out_buffer_control;
                $body
            }
            (12, true) => {
                let $reg = &$dpram.ep12_in_buffer_control;
                $body
            }
            (12, false) => {
                let $reg = &$dpram.ep12_out_buffer_control;
                $body
            }
            (13, true) => {
                let $reg = &$dpram.ep13_in_buffer_control;
                $body
            }
            (13, false) => {
                let $reg = &$dpram.ep13_out_buffer_control;
                $body
            }
            (14, true) => {
                let $reg = &$dpram.ep14_in_buffer_control;
                $body
            }
            (14, false) => {
                let $reg = &$dpram.ep14_out_buffer_control;
                $body
            }
            (15, true) => {
                let $reg = &$dpram.ep15_in_buffer_control;
                $body
            }
            (15, false) => {
                let $reg = &$dpram.ep15_out_buffer_control;
                $body
            }
            _ => panic!("invalid endpoint address"),
        }
    }};
}

/// Bind the endpoint-control register for a given endpoint address (EP1–EP15).
macro_rules! endpoint_control_match {
    ($dpram:expr, $ep_addr:expr, |$reg:ident| $body:expr) => {{
        match ($ep_addr.number(), $ep_addr.is_in()) {
            (1, true) => {
                let $reg = &$dpram.ep1_in_control;
                $body
            }
            (1, false) => {
                let $reg = &$dpram.ep1_out_control;
                $body
            }
            (2, true) => {
                let $reg = &$dpram.ep2_in_control;
                $body
            }
            (2, false) => {
                let $reg = &$dpram.ep2_out_control;
                $body
            }
            (3, true) => {
                let $reg = &$dpram.ep3_in_control;
                $body
            }
            (3, false) => {
                let $reg = &$dpram.ep3_out_control;
                $body
            }
            (4, true) => {
                let $reg = &$dpram.ep4_in_control;
                $body
            }
            (4, false) => {
                let $reg = &$dpram.ep4_out_control;
                $body
            }
            (5, true) => {
                let $reg = &$dpram.ep5_in_control;
                $body
            }
            (5, false) => {
                let $reg = &$dpram.ep5_out_control;
                $body
            }
            (6, true) => {
                let $reg = &$dpram.ep6_in_control;
                $body
            }
            (6, false) => {
                let $reg = &$dpram.ep6_out_control;
                $body
            }
            (7, true) => {
                let $reg = &$dpram.ep7_in_control;
                $body
            }
            (7, false) => {
                let $reg = &$dpram.ep7_out_control;
                $body
            }
            (8, true) => {
                let $reg = &$dpram.ep8_in_control;
                $body
            }
            (8, false) => {
                let $reg = &$dpram.ep8_out_control;
                $body
            }
            (9, true) => {
                let $reg = &$dpram.ep9_in_control;
                $body
            }
            (9, false) => {
                let $reg = &$dpram.ep9_out_control;
                $body
            }
            (10, true) => {
                let $reg = &$dpram.ep10_in_control;
                $body
            }
            (10, false) => {
                let $reg = &$dpram.ep10_out_control;
                $body
            }
            (11, true) => {
                let $reg = &$dpram.ep11_in_control;
                $body
            }
            (11, false) => {
                let $reg = &$dpram.ep11_out_control;
                $body
            }
            (12, true) => {
                let $reg = &$dpram.ep12_in_control;
                $body
            }
            (12, false) => {
                let $reg = &$dpram.ep12_out_control;
                $body
            }
            (13, true) => {
                let $reg = &$dpram.ep13_in_control;
                $body
            }
            (13, false) => {
                let $reg = &$dpram.ep13_out_control;
                $body
            }
            (14, true) => {
                let $reg = &$dpram.ep14_in_control;
                $body
            }
            (14, false) => {
                let $reg = &$dpram.ep14_out_control;
                $body
            }
            (15, true) => {
                let $reg = &$dpram.ep15_in_control;
                $body
            }
            (15, false) => {
                let $reg = &$dpram.ep15_out_control;
                $body
            }
            _ => panic!("invalid endpoint address"),
        }
    }};
}

/// Access buffer control register by endpoint address
///
/// This provides dynamic access to buffer control registers based on endpoint number/direction.
pub struct BufferControlAccess<'a> {
    dpram: &'a usb_dpram::RegisterBlock,
}

impl<'a> BufferControlAccess<'a> {
    /// Create a new buffer control accessor
    pub fn new(dpram: &'a usb_dpram::RegisterBlock) -> Self {
        Self { dpram }
    }

    /// Write buffer control with errata handling for any endpoint
    pub fn write_with_errata(&self, ep_addr: EndpointAddress, val: u32) {
        buffer_control_match!(self.dpram, ep_addr, |reg| reg.write_with_errata(val));
    }

    /// Read buffer control for any endpoint
    pub fn read(&self, ep_addr: EndpointAddress) -> u32 {
        buffer_control_match!(self.dpram, ep_addr, |reg| reg.read_raw())
    }

    /// Set stall for any endpoint
    pub fn set_stall(&self, ep_addr: EndpointAddress) {
        buffer_control_match!(self.dpram, ep_addr, |reg| reg.set_stall());
    }

    /// Clear stall for any endpoint
    pub fn clear_stall(&self, ep_addr: EndpointAddress) {
        buffer_control_match!(self.dpram, ep_addr, |reg| reg.clear_stall());
    }

    /// Check if endpoint is stalled
    pub fn is_stalled(&self, ep_addr: EndpointAddress) -> bool {
        buffer_control_match!(self.dpram, ep_addr, |reg| reg.is_stalled())
    }
}

/// Access endpoint control register by endpoint address (EP1-EP15 only)
pub struct EndpointControlAccess<'a> {
    dpram: &'a usb_dpram::RegisterBlock,
}

impl<'a> EndpointControlAccess<'a> {
    /// Create a new endpoint control accessor
    pub fn new(dpram: &'a usb_dpram::RegisterBlock) -> Self {
        Self { dpram }
    }

    /// Configure an endpoint (EP1-EP15)
    pub fn configure(&self, ep_addr: EndpointAddress, ep_type: EndpointType, buffer_offset: u16) {
        if ep_addr.number() == 0 {
            // EP0 doesn't have control registers
            return;
        }
        endpoint_control_match!(self.dpram, ep_addr, |reg| reg
            .configure(ep_type, buffer_offset));
    }

    /// Disable an endpoint (EP1-EP15)
    pub fn disable(&self, ep_addr: EndpointAddress) {
        if ep_addr.number() == 0 {
            return;
        }
        endpoint_control_match!(self.dpram, ep_addr, |reg| reg.disable());
    }
}
