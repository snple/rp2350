//! Reset subsystem for RP2350
//!
//! Provides a [`Resettable`] trait implemented for each PAC peripheral type,
//! so peripherals know how to reset themselves.
//!
//! # Example
//!
//! ```ignore
//! use pico_rs::reset::Resettable;
//!
//! // Standard init: reset + unreset (guarantees clean register state)
//! pico_rs_pac::SPI0::reset_init();
//!
//! // Unreset only (peripheral already in reset at first boot)
//! pico_rs_pac::UART0::unreset();
//!
//! // Batch operations (unsafe: caller must ensure mask is valid)
//! use pico_rs::reset;
//! unsafe {
//!     reset::reset(pico_rs_pac::IO_BANK0::RESET_MASK | pico_rs_pac::PADS_BANK0::RESET_MASK);
//!     reset::unreset(pico_rs_pac::IO_BANK0::RESET_MASK | pico_rs_pac::PADS_BANK0::RESET_MASK);
//! }
//! ```

use pico_rs_pac::RESETS;

/// Trait for peripherals that can be reset via the RESETS controller.
///
/// Implemented automatically for all PAC peripheral types that have a
/// corresponding bit in the RESETS register.
pub trait Resettable {
    /// Bitmask for this peripheral in the RESETS register.
    const RESET_MASK: u32;

    /// Reset + unreset. Guarantees all registers return to hardware defaults.
    #[inline]
    fn reset_init() {
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
#[inline]
pub unsafe fn unreset(mask: u32) {
    let resets = unsafe { RESETS::steal() };
    resets.reset.clr_bits(mask);
    while (resets.reset_done.read_bits() & mask) != mask {
        core::hint::spin_loop();
    }
}

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
    pico_rs_pac::ADC       => 0,
    pico_rs_pac::BUSCTRL   => 1,
    pico_rs_pac::DMA       => 2,
    pico_rs_pac::HSTX_CTRL => 3,
    pico_rs_pac::I2C0      => 4,
    pico_rs_pac::I2C1      => 5,
    pico_rs_pac::IO_BANK0  => 6,
    pico_rs_pac::IO_QSPI   => 7,
    // Bit 8 = JTAG (no PAC type)
    pico_rs_pac::PADS_BANK0 => 9,
    pico_rs_pac::PADS_QSPI  => 10,
    pico_rs_pac::PIO0      => 11,
    pico_rs_pac::PIO1      => 12,
    pico_rs_pac::PIO2      => 13,
    pico_rs_pac::PLL_SYS   => 14,
    pico_rs_pac::PLL_USB   => 15,
    pico_rs_pac::PWM       => 16,
    pico_rs_pac::SHA256    => 17,
    pico_rs_pac::SPI0      => 18,
    pico_rs_pac::SPI1      => 19,
    pico_rs_pac::SYSCFG    => 20,
    pico_rs_pac::SYSINFO   => 21,
    pico_rs_pac::TBMAN     => 22,
    pico_rs_pac::TIMER0    => 23,
    pico_rs_pac::TIMER1    => 24,
    pico_rs_pac::TRNG      => 25,
    pico_rs_pac::UART0     => 26,
    pico_rs_pac::UART1     => 27,
    pico_rs_pac::USB       => 28,
);
