//! Reset subsystem driver for RP2350
//!
//! This module provides a safe interface for managing peripheral resets.
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
//! ```

use pico_rs_pac::RESETS;

/// Peripheral blocks that can be reset
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
#[allow(non_camel_case_types)]
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

/// Put peripheral(s) into reset state
///
/// # Arguments
/// * `bits` - Bitmask of peripherals to reset
///
/// # Example
/// ```ignore
/// reset::reset_block_mask(Peripheral::UART0 | Peripheral::UART1);
/// ```
#[inline]
pub fn reset_block_mask(bits: u32) {
    let resets = unsafe { &*RESETS::PTR };
    resets.reset.set_bits(bits);
}

/// Put a single peripheral into reset state
///
/// # Arguments
/// * `peripheral` - The peripheral to reset
///
/// # Example
/// ```ignore
/// reset::reset_block(Peripheral::UART0);
/// ```
#[inline]
pub fn reset_block(peripheral: Peripheral) {
    reset_block_mask(peripheral.mask());
}

/// Bring peripheral(s) out of reset (without waiting)
///
/// # Arguments
/// * `bits` - Bitmask of peripherals to unreset
///
/// # Example
/// ```ignore
/// reset::unreset_block_mask(Peripheral::UART0 | Peripheral::UART1);
/// // Must wait for reset_done separately if needed
/// reset::unreset_wait_mask(Peripheral::UART0 | Peripheral::UART1);
/// ```
#[inline]
pub fn unreset_block_mask(bits: u32) {
    let resets = unsafe { &*RESETS::PTR };
    resets.reset.clr_bits(bits);
}

/// Bring a single peripheral out of reset (without waiting)
///
/// # Arguments
/// * `peripheral` - The peripheral to unreset
///
/// # Example
/// ```ignore
/// reset::unreset_block(Peripheral::UART0);
/// // Must wait for reset_done separately if needed
/// reset::unreset_wait(Peripheral::UART0);
/// ```
#[inline]
pub fn unreset_block(peripheral: Peripheral) {
    unreset_block_mask(peripheral.mask());
}

/// Wait for peripheral(s) to complete reset
///
/// # Arguments
/// * `bits` - Bitmask of peripherals to wait for
///
/// # Example
/// ```ignore
/// reset::unreset_block_mask(Peripheral::UART0 | Peripheral::UART1);
/// reset::unreset_wait_mask(Peripheral::UART0 | Peripheral::UART1);
/// ```
#[inline]
pub fn unreset_wait_mask(bits: u32) {
    let resets = unsafe { &*RESETS::PTR };
    while (resets.reset_done.read_bits() & bits) != bits {
        core::hint::spin_loop();
    }
}

/// Wait for a single peripheral to complete reset
///
/// # Arguments
/// * `peripheral` - The peripheral to wait for
///
/// # Example
/// ```ignore
/// reset::unreset_block(Peripheral::UART0);
/// reset::unreset_wait(Peripheral::UART0);
/// ```
#[inline]
pub fn unreset_wait(peripheral: Peripheral) {
    unreset_wait_mask(peripheral.mask());
}

/// Bring peripheral(s) out of reset and wait for completion
///
/// This is the most common operation - it unresets and waits in one call.
///
/// # Arguments
/// * `bits` - Bitmask of peripherals to unreset
///
/// # Example
/// ```ignore
/// reset::unreset_block_wait_mask(Peripheral::UART0 | Peripheral::UART1);
/// ```
#[inline]
pub fn unreset_block_wait_mask(bits: u32) {
    unreset_block_mask(bits);
    unreset_wait_mask(bits);
}

/// Bring a single peripheral out of reset and wait for completion
///
/// This is the most common operation - it unresets and waits in one call.
///
/// # Arguments
/// * `peripheral` - The peripheral to unreset
///
/// # Example
/// ```ignore
/// reset::unreset_block_wait(Peripheral::UART0);
/// ```
#[inline]
pub fn unreset_block_wait(peripheral: Peripheral) {
    unreset_block_wait_mask(peripheral.mask());
}

/// Check if peripheral(s) are in reset
///
/// # Arguments
/// * `bits` - Bitmask of peripherals to check
///
/// # Returns
/// `true` if all specified peripherals are in reset, `false` otherwise
///
/// # Example
/// ```ignore
/// if reset::is_in_reset_mask(Peripheral::UART0 | Peripheral::UART1) {
///     // Both UART0 and UART1 are in reset
/// }
/// ```
#[inline]
pub fn is_in_reset_mask(bits: u32) -> bool {
    let resets = unsafe { &*RESETS::PTR };
    (resets.reset.read_bits() & bits) == bits
}

/// Check if a single peripheral is in reset
///
/// # Arguments
/// * `peripheral` - The peripheral to check
///
/// # Returns
/// `true` if the peripheral is in reset, `false` otherwise
///
/// # Example
/// ```ignore
/// if reset::is_in_reset(Peripheral::UART0) {
///     reset::unreset_block_wait(Peripheral::UART0);
/// }
/// ```
#[inline]
pub fn is_in_reset(peripheral: Peripheral) -> bool {
    is_in_reset_mask(peripheral.mask())
}

/// Check if peripheral(s) have completed reset
///
/// # Arguments
/// * `bits` - Bitmask of peripherals to check
///
/// # Returns
/// `true` if all specified peripherals have completed reset, `false` otherwise
///
/// # Example
/// ```ignore
/// if reset::is_reset_done_mask(Peripheral::UART0 | Peripheral::UART1) {
///     // Both UART0 and UART1 are ready to use
/// }
/// ```
#[inline]
pub fn is_reset_done_mask(bits: u32) -> bool {
    let resets = unsafe { &*RESETS::PTR };
    (resets.reset_done.read_bits() & bits) == bits
}

/// Check if a single peripheral has completed reset
///
/// # Arguments
/// * `peripheral` - The peripheral to check
///
/// # Returns
/// `true` if the peripheral has completed reset, `false` otherwise
///
/// # Example
/// ```ignore
/// if reset::is_reset_done(Peripheral::UART0) {
///     // UART0 is ready to use
/// }
/// ```
#[inline]
pub fn is_reset_done(peripheral: Peripheral) -> bool {
    is_reset_done_mask(peripheral.mask())
}

/// Reset all peripherals
///
/// This puts all peripherals into reset. Use with caution!
///
/// # Example
/// ```ignore
/// reset::reset_all();
/// ```
#[inline]
pub fn reset_all() {
    let resets = unsafe { &*RESETS::PTR };
    resets.reset.write_bits(0x1fffffff); // All 29 peripherals
}

/// Unreset all peripherals and wait
///
/// This brings all peripherals out of reset. Use with caution!
///
/// # Example
/// ```ignore
/// reset::unreset_all_wait();
/// ```
#[inline]
pub fn unreset_all_wait() {
    unreset_block_wait_mask(0x1fffffff); // All 29 peripherals
}
