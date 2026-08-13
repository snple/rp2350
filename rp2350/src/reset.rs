//! Reset subsystem for RP2350
//!
//! Provides a [`Resettable`] trait implemented for each PAC peripheral type,
//! so peripherals know how to reset themselves.
//!
//! # Example
//!
//! ```ignore
//! use rp2350::reset::Resettable;
//!
//! // Standard init: reset + unreset (guarantees clean register state)
//! rp2350_pac::SPI0::reset_init();
//!
//! // Unreset only (peripheral already in reset at first boot)
//! rp2350_pac::UART0::unreset();
//!
//! // Batch operations (unsafe: caller must ensure mask is valid)
//! use rp2350::reset;
//! unsafe {
//!     reset::reset(rp2350_pac::IO_BANK0::RESET_MASK | rp2350_pac::PADS_BANK0::RESET_MASK);
//!     reset::unreset(rp2350_pac::IO_BANK0::RESET_MASK | rp2350_pac::PADS_BANK0::RESET_MASK);
//! }
//! ```

use rp2350_pac::RESETS;

/// Trait for peripherals that can be reset via the RESETS controller.
///
/// Implemented automatically for all PAC peripheral types that have a
/// corresponding bit in the RESETS register.
pub trait Resettable {
    /// Bitmask for this peripheral in the RESETS register.
    const RESET_MASK: u32;

    /// Reset + unreset. Guarantees all registers return to hardware defaults.
    #[inline]
    fn init() {
        // SAFETY: RESET_MASK is set by impl_resettable! and always valid.
        unsafe {
            reset(Self::RESET_MASK);
            unreset(Self::RESET_MASK);
        }
    }

    /// Put this peripheral into reset state.
    #[inline]
    fn reset() {
        // SAFETY: RESET_MASK is set by impl_resettable! and always valid.
        unsafe { reset(Self::RESET_MASK) }
    }

    /// Bring this peripheral out of reset and wait for completion.
    #[inline]
    fn unreset() {
        // SAFETY: RESET_MASK is set by impl_resettable! and always valid.
        unsafe { unreset(Self::RESET_MASK) }
    }
}

/// Put peripheral(s) into reset (raw mask).
///
/// # Safety
///
/// `mask` must only contain valid RESETS register bits (bits 0..=28).
/// Prefer [`Resettable::reset()`] for type-safe single-peripheral reset.
#[inline]
pub unsafe fn reset(mask: u32) {
    let resets = unsafe { RESETS::steal() };
    resets.reset.set_bits(mask);
}

/// Bring peripheral(s) out of reset and wait (raw mask).
///
/// # Safety
///
/// `mask` must only contain valid RESETS register bits (bits 0..=28).
/// Prefer [`Resettable::unreset()`] for type-safe single-peripheral unreset.
///
/// # Panics
///
/// Panics if the peripheral does not leave reset within
/// [`RESET_TIMEOUT_SPINS`] spin iterations. This normally indicates a
/// configuration error, such as the peripheral being held in reset by
/// ACCESSCTRL or its clock being gated — reporting the failure is preferable
/// to hanging indefinitely.
#[inline]
pub unsafe fn unreset(mask: u32) {
    let resets = unsafe { RESETS::steal() };
    resets.reset.clr_bits(mask);
    crate::spin::spin_wait("peripheral to leave reset", || {
        (resets.reset_done.read_bits() & mask) == mask
    });
}

/// Maximum spin iterations before [`unreset`] reports a timeout.
///
/// Alias for [`crate::spin::SPIN_TIMEOUT`], kept for documentation and
/// backwards compatibility.
pub const RESET_TIMEOUT_SPINS: u32 = crate::spin::SPIN_TIMEOUT;

// ============================================================================
// Implement Resettable for each PAC peripheral type
// ============================================================================

macro_rules! impl_resettable {
    ($($pac_type:ty => $bit:literal),+ $(,)?) => {
        $(
            impl Resettable for $pac_type {
                const RESET_MASK: u32 = 1 << $bit;
            }
        )+
    };
}

impl_resettable!(
    rp2350_pac::ADC       => 0,
    rp2350_pac::BUSCTRL   => 1,
    rp2350_pac::DMA       => 2,
    rp2350_pac::HSTX_CTRL => 3,
    rp2350_pac::I2C0      => 4,
    rp2350_pac::I2C1      => 5,
    rp2350_pac::IO_BANK0  => 6,
    rp2350_pac::IO_QSPI   => 7,
    // Bit 8 = JTAG (no PAC type)
    rp2350_pac::PADS_BANK0 => 9,
    rp2350_pac::PADS_QSPI  => 10,
    rp2350_pac::PIO0      => 11,
    rp2350_pac::PIO1      => 12,
    rp2350_pac::PIO2      => 13,
    rp2350_pac::PLL_SYS   => 14,
    rp2350_pac::PLL_USB   => 15,
    rp2350_pac::PWM       => 16,
    rp2350_pac::SHA256    => 17,
    rp2350_pac::SPI0      => 18,
    rp2350_pac::SPI1      => 19,
    rp2350_pac::SYSCFG    => 20,
    rp2350_pac::SYSINFO   => 21,
    rp2350_pac::TBMAN     => 22,
    rp2350_pac::TIMER0    => 23,
    rp2350_pac::TIMER1    => 24,
    rp2350_pac::TRNG      => 25,
    rp2350_pac::UART0     => 26,
    rp2350_pac::UART1     => 27,
    rp2350_pac::USB       => 28,
);
