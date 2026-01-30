//! Reset subsystem driver for RP2350
//!
//! This module provides a safe interface for managing peripheral resets
//! using the PAC register access methods.
//!
//! ## Example
//!
//! ```ignore
//! use pico_rs::reset::{self, Peripheral};
//!
//! // Reset and unreset a single peripheral
//! reset::reset_block(Peripheral::UART0);
//! reset::unreset_block_wait(Peripheral::UART0);
//!
//! // Reset multiple peripherals
//! reset::reset_block_mask(Peripheral::UART0 | Peripheral::UART1);
//! reset::unreset_block_wait_mask(Peripheral::UART0 | Peripheral::UART1);
//!
//! // Check if a peripheral is in reset
//! if reset::is_in_reset(Peripheral::I2C0) {
//!     reset::unreset_block_wait(Peripheral::I2C0);
//! }
//!
//! // Use individual peripheral methods
//! reset::reset_uart0();
//! reset::unreset_uart0_wait();
//! ```

use pico_rs_pac::RESETS;

/// Peripheral blocks that can be reset
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
#[allow(non_camel_case_types)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Peripheral {
    ADC = 1 << 0,
    BUSCTRL = 1 << 1,
    DMA = 1 << 2,
    HSTX = 1 << 3,
    I2C0 = 1 << 4,
    I2C1 = 1 << 5,
    IO_BANK0 = 1 << 6,
    IO_QSPI = 1 << 7,
    JTAG = 1 << 8,
    PADS_BANK0 = 1 << 9,
    PADS_QSPI = 1 << 10,
    PIO0 = 1 << 11,
    PIO1 = 1 << 12,
    PIO2 = 1 << 13,
    PLL_SYS = 1 << 14,
    PLL_USB = 1 << 15,
    PWM = 1 << 16,
    SHA256 = 1 << 17,
    SPI0 = 1 << 18,
    SPI1 = 1 << 19,
    SYSCFG = 1 << 20,
    SYSINFO = 1 << 21,
    TBMAN = 1 << 22,
    TIMER0 = 1 << 23,
    TIMER1 = 1 << 24,
    TRNG = 1 << 25,
    UART0 = 1 << 26,
    UART1 = 1 << 27,
    USBCTRL = 1 << 28,
}

impl Peripheral {
    /// Get the bit mask for this peripheral
    #[inline]
    pub const fn mask(self) -> u32 {
        self as u32
    }

    /// Combine multiple peripherals into a bitmask
    #[inline]
    pub const fn or(self, other: Self) -> u32 {
        self.mask() | other.mask()
    }
}

impl core::ops::BitOr for Peripheral {
    type Output = u32;

    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        self.mask() | rhs.mask()
    }
}

impl core::ops::BitOr<u32> for Peripheral {
    type Output = u32;

    #[inline]
    fn bitor(self, rhs: u32) -> Self::Output {
        self.mask() | rhs
    }
}

impl core::ops::BitOr<Peripheral> for u32 {
    type Output = u32;

    #[inline]
    fn bitor(self, rhs: Peripheral) -> Self::Output {
        self | rhs.mask()
    }
}

// ============================================================================
// Mask-based operations (for multiple peripherals)
// ============================================================================

/// Put peripheral(s) into reset state
///
/// # Arguments
/// * `bits` - Bitmask of peripherals to reset
#[inline]
pub fn reset_block_mask(bits: u32) {
    let resets = unsafe { &*RESETS::PTR };
    resets.reset.set_bits(bits);
}

/// Put a single peripheral into reset state
#[inline]
pub fn reset_block(peripheral: Peripheral) {
    reset_block_mask(peripheral.mask());
}

/// Bring peripheral(s) out of reset (without waiting)
#[inline]
pub fn unreset_block_mask(bits: u32) {
    let resets = unsafe { &*RESETS::PTR };
    resets.reset.clr_bits(bits);
}

/// Bring a single peripheral out of reset (without waiting)
#[inline]
pub fn unreset_block(peripheral: Peripheral) {
    unreset_block_mask(peripheral.mask());
}

/// Wait for peripheral(s) to complete reset
#[inline]
pub fn unreset_wait_mask(bits: u32) {
    let resets = unsafe { &*RESETS::PTR };
    while (resets.reset_done.read_bits() & bits) != bits {
        core::hint::spin_loop();
    }
}

/// Wait for a single peripheral to complete reset
#[inline]
pub fn unreset_wait(peripheral: Peripheral) {
    unreset_wait_mask(peripheral.mask());
}

/// Bring peripheral(s) out of reset and wait for completion
#[inline]
pub fn unreset_block_wait_mask(bits: u32) {
    unreset_block_mask(bits);
    unreset_wait_mask(bits);
}

/// Bring a single peripheral out of reset and wait for completion
#[inline]
pub fn unreset_block_wait(peripheral: Peripheral) {
    unreset_block_wait_mask(peripheral.mask());
}

/// Check if peripheral(s) are in reset
#[inline]
pub fn is_in_reset_mask(bits: u32) -> bool {
    let resets = unsafe { &*RESETS::PTR };
    (resets.reset.read_bits() & bits) == bits
}

/// Check if a single peripheral is in reset
#[inline]
pub fn is_in_reset(peripheral: Peripheral) -> bool {
    is_in_reset_mask(peripheral.mask())
}

/// Check if peripheral(s) have completed reset
#[inline]
pub fn is_reset_done_mask(bits: u32) -> bool {
    let resets = unsafe { &*RESETS::PTR };
    (resets.reset_done.read_bits() & bits) == bits
}

/// Check if a single peripheral has completed reset
#[inline]
pub fn is_reset_done(peripheral: Peripheral) -> bool {
    is_reset_done_mask(peripheral.mask())
}

/// Reset all peripherals
#[inline]
pub fn reset_all() {
    let resets = unsafe { &*RESETS::PTR };
    resets.reset.write_bits(0x1fffffff);
}

/// Unreset all peripherals and wait
#[inline]
pub fn unreset_all_wait() {
    unreset_block_wait_mask(0x1fffffff);
}

// ============================================================================
// Individual peripheral reset functions using PAC methods
// ============================================================================

macro_rules! impl_peripheral_reset {
    ($($name:ident),+ $(,)?) => {
        paste::paste! {
            $(
                #[doc = "Put " $name " into reset state"]
                #[inline]
                pub fn [<reset_ $name:lower>]() {
                    let resets = unsafe { &*RESETS::PTR };
                    resets.reset.[<set_ $name:lower>]();
                }

                #[doc = "Bring " $name " out of reset (without waiting)"]
                #[inline]
                pub fn [<unreset_ $name:lower>]() {
                    let resets = unsafe { &*RESETS::PTR };
                    resets.reset.[<clr_ $name:lower>]();
                }

                #[doc = "Wait for " $name " to complete reset"]
                #[inline]
                pub fn [<unreset_ $name:lower _wait>]() {
                    let resets = unsafe { &*RESETS::PTR };
                    resets.reset.[<clr_ $name:lower>]();
                    while !resets.reset_done.[<$name:lower>]() {
                        core::hint::spin_loop();
                    }
                }

                #[doc = "Check if " $name " is in reset"]
                #[inline]
                pub fn [<is_ $name:lower _in_reset>]() -> bool {
                    let resets = unsafe { &*RESETS::PTR };
                    resets.reset.[<$name:lower>]()
                }

                #[doc = "Check if " $name " has completed reset"]
                #[inline]
                pub fn [<is_ $name:lower _reset_done>]() -> bool {
                    let resets = unsafe { &*RESETS::PTR };
                    resets.reset_done.[<$name:lower>]()
                }
            )+
        }
    };
}

impl_peripheral_reset!(
    adc, busctrl, dma, hstx, i2c0, i2c1, io_bank0, io_qspi, jtag, pads_bank0, pads_qspi, pio0,
    pio1, pio2, pll_sys, pll_usb, pwm, sha256, spi0, spi1, syscfg, sysinfo, tbman, timer0, timer1,
    trng, uart0, uart1, usbctrl,
);
