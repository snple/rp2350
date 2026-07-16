//! Nested Vectored Interrupt Controller (NVIC)
//!
//! The NVIC is responsible for managing interrupts. It provides:
//! - Interrupt enable/disable
//! - Interrupt pending/clear
//! - Priority configuration
//!
//! # Priority Levels (RP2350)
//!
//! The RP2350 Cortex-M33 implements **4 bits** of interrupt priority in the
//! upper nibble of each 8-bit IPR register field. This gives **16 distinct
//! priority levels**. Lower numerical values mean higher priority.
//!
//! Valid 8-bit values (only the top 4 bits are significant):
//!
//! | Level | 8-bit value | Description |
//! |-------|-------------|-------------|
//! |  0    |   0 (0x00)  | Highest priority |
//! |  1    |  16 (0x10)  | |
//! |  2    |  32 (0x20)  | |
//! |  3    |  48 (0x30)  | |
//! |  4    |  64 (0x40)  | |
//! |  5    |  80 (0x50)  | |
//! |  6    |  96 (0x60)  | |
//! |  7    | 112 (0x70)  | |
//! |  8    | 128 (0x80)  | (mid-point, commonly used) |
//! |  9    | 144 (0x90)  | |
//! | 10    | 160 (0xA0)  | |
//! | 11    | 176 (0xB0)  | |
//! | 12    | 192 (0xC0)  | |
//! | 13    | 208 (0xD0)  | |
//! | 14    | 224 (0xE0)  | |
//! | 15    | 240 (0xF0)  | Lowest priority |
//!
//! Values not in the table (e.g. 1, 100, 255) are silently truncated by the
//! hardware: only `value & 0xF0` is stored. Use multiples of 16 to be explicit.
//!
//! # Example
//!
//! ```ignore
//! use pico_rs_rt::peripheral::NVIC;
//!
//! // Enable interrupt 0
//! unsafe { NVIC::enable(0); }
//!
//! // Set priority (lower = higher priority; use multiples of 16)
//! unsafe { NVIC::set_priority(0, 128); }
//!
//! // Check if pending
//! if NVIC::is_pending(0) {
//!     NVIC::unpend(0);
//! }
//! ```

use core::marker::PhantomData;

use super::{NVIC_BASE, VolatileCell, peripheral};

/// NVIC Register Block
#[repr(C)]
pub struct RegisterBlock {
    /// Interrupt Set-Enable Registers (16 x 32-bit = 512 interrupts max)
    pub iser: [VolatileCell<u32>; 16],
    _reserved0: [u32; 16],
    /// Interrupt Clear-Enable Registers
    pub icer: [VolatileCell<u32>; 16],
    _reserved1: [u32; 16],
    /// Interrupt Set-Pending Registers
    pub ispr: [VolatileCell<u32>; 16],
    _reserved2: [u32; 16],
    /// Interrupt Clear-Pending Registers
    pub icpr: [VolatileCell<u32>; 16],
    _reserved3: [u32; 16],
    /// Interrupt Active Bit Registers (read-only)
    pub iabr: [VolatileCell<u32>; 16],
    _reserved4: [u32; 16],
    /// Interrupt Target Non-Secure Registers (ARMv8-M)
    pub itns: [VolatileCell<u32>; 16],
    _reserved5: [u32; 16],
    /// Interrupt Priority Registers (8-bit each, 496 total)
    pub ipr: [VolatileCell<u8>; 496],
}

/// NVIC peripheral
pub struct NVIC {
    _marker: PhantomData<*const ()>,
}

peripheral!(NVIC, RegisterBlock, NVIC_BASE);

// Make NVIC Send but not Sync (operations require care in multi-core)
unsafe impl Send for NVIC {}

impl NVIC {
    /// Enable an interrupt
    ///
    /// # Safety
    ///
    /// Enabling an interrupt allows its handler to run at any time.
    /// Ensure the handler is properly set up before enabling.
    #[inline]
    pub unsafe fn enable(irq: u16) {
        let nr = irq as usize;
        let reg = nr / 32;
        let bit = nr % 32;

        unsafe {
            (*Self::PTR).iser[reg].write_bits(1 << bit);
        }
    }

    /// Disable an interrupt
    #[inline]
    pub fn disable(irq: u16) {
        let nr = irq as usize;
        let reg = nr / 32;
        let bit = nr % 32;

        unsafe {
            (*Self::PTR).icer[reg].write_bits(1 << bit);
        }
    }

    /// Check if an interrupt is enabled
    #[inline]
    pub fn is_enabled(irq: u16) -> bool {
        let nr = irq as usize;
        let reg = nr / 32;
        let bit = nr % 32;

        unsafe { (*Self::PTR).iser[reg].read_bits() & (1 << bit) != 0 }
    }

    /// Set an interrupt as pending
    #[inline]
    pub fn pend(irq: u16) {
        let nr = irq as usize;
        let reg = nr / 32;
        let bit = nr % 32;

        unsafe {
            (*Self::PTR).ispr[reg].write_bits(1 << bit);
        }
    }

    /// Clear a pending interrupt
    #[inline]
    pub fn unpend(irq: u16) {
        let nr = irq as usize;
        let reg = nr / 32;
        let bit = nr % 32;

        unsafe {
            (*Self::PTR).icpr[reg].write_bits(1 << bit);
        }
    }

    /// Check if an interrupt is pending
    #[inline]
    pub fn is_pending(irq: u16) -> bool {
        let nr = irq as usize;
        let reg = nr / 32;
        let bit = nr % 32;

        unsafe { (*Self::PTR).ispr[reg].read_bits() & (1 << bit) != 0 }
    }

    /// Check if an interrupt is active (currently being handled)
    #[inline]
    pub fn active(irq: u16) -> bool {
        let nr = irq as usize;
        let reg = nr / 32;
        let bit = nr % 32;

        unsafe { (*Self::PTR).iabr[reg].read_bits() & (1 << bit) != 0 }
    }

    /// Set the priority of an interrupt
    ///
    /// Lower values mean higher priority. The RP2350 Cortex-M33 implements
    /// **4 priority bits** in the upper nibble of the 8-bit IPR field, giving
    /// **16 effective levels**. Use multiples of 16 to be explicit:
    /// `0, 16, 32, 48, 64, 80, 96, 112, 128, 144, 160, 176, 192, 208, 224, 240`.
    ///
    /// Any value written is masked to `value & 0xF0` by the hardware.
    ///
    /// # Safety
    ///
    /// Must be called before `enable()` for the priority to take effect on
    /// the first invocation. Changing priorities after enable can affect
    /// interrupt latency and system behavior.
    #[inline]
    pub unsafe fn set_priority(irq: u16, priority: u8) {
        unsafe {
            (*Self::PTR).ipr[irq as usize].write_bits(priority);
        }
    }

    /// Get the priority of an interrupt
    #[inline]
    pub fn priority(irq: u16) -> u8 {
        unsafe { (*Self::PTR).ipr[irq as usize].read_bits() }
    }

    /// Software-trigger an interrupt (writes to STIR)
    ///
    /// This is equivalent to setting the interrupt as pending.
    #[inline]
    pub fn request(irq: u16) {
        // STIR is at 0xE000EF00
        const STIR: *mut u32 = 0xE000_EF00 as *mut u32;
        unsafe {
            core::ptr::write_volatile(STIR, irq as u32);
        }
    }
}

/// Trait for types that represent interrupt numbers
///
/// # Safety
///
/// Implementations must return unique, valid interrupt numbers.
pub unsafe trait InterruptNumber: Copy {
    /// Return the interrupt number
    fn number(self) -> u16;
}

// =============================================================================
// Type-safe NVIC operations using InterruptNumber trait
// =============================================================================

impl NVIC {
    /// Enable an interrupt (type-safe version)
    ///
    /// # Safety
    ///
    /// Same as `enable()`
    #[inline]
    pub unsafe fn unmask<I: InterruptNumber>(interrupt: I) {
        unsafe { Self::enable(interrupt.number()) }
    }

    /// Disable an interrupt (type-safe version)
    #[inline]
    pub fn mask<I: InterruptNumber>(interrupt: I) {
        Self::disable(interrupt.number())
    }

    /// Check if enabled (type-safe version)
    #[inline]
    pub fn is_enabled_int<I: InterruptNumber>(interrupt: I) -> bool {
        Self::is_enabled(interrupt.number())
    }

    /// Set pending (type-safe version)
    #[inline]
    pub fn pend_int<I: InterruptNumber>(interrupt: I) {
        Self::pend(interrupt.number())
    }

    /// Clear pending (type-safe version)
    #[inline]
    pub fn unpend_int<I: InterruptNumber>(interrupt: I) {
        Self::unpend(interrupt.number())
    }

    /// Check if pending (type-safe version)
    #[inline]
    pub fn is_pending_int<I: InterruptNumber>(interrupt: I) -> bool {
        Self::is_pending(interrupt.number())
    }

    /// Set priority (type-safe version)
    ///
    /// # Safety
    #[inline]
    pub unsafe fn set_priority_int<I: InterruptNumber>(interrupt: I, priority: u8) {
        unsafe { Self::set_priority(interrupt.number(), priority) }
    }

    /// Get priority (type-safe version)
    #[inline]
    pub fn priority_int<I: InterruptNumber>(interrupt: I) -> u8 {
        Self::priority(interrupt.number())
    }
}
