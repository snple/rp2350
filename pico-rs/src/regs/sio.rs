//! SIO (Single-cycle I/O) peripheral registers
//!
//! Fast single-cycle access to GPIO, spinlocks, and core-local peripherals

use super::Reg;
use super::addr::SIO_BASE;

// Register offsets
pub const CPUID: u32 = SIO_BASE + 0x000;

// GPIO registers
pub const GPIO_IN: u32 = SIO_BASE + 0x004;
pub const GPIO_HI_IN: u32 = SIO_BASE + 0x008;
pub const GPIO_OUT: u32 = SIO_BASE + 0x010;
pub const GPIO_HI_OUT: u32 = SIO_BASE + 0x014;
pub const GPIO_OUT_SET: u32 = SIO_BASE + 0x018;
pub const GPIO_HI_OUT_SET: u32 = SIO_BASE + 0x01C;
pub const GPIO_OUT_CLR: u32 = SIO_BASE + 0x020;
pub const GPIO_HI_OUT_CLR: u32 = SIO_BASE + 0x024;
pub const GPIO_OUT_XOR: u32 = SIO_BASE + 0x028;
pub const GPIO_HI_OUT_XOR: u32 = SIO_BASE + 0x02C;
pub const GPIO_OE: u32 = SIO_BASE + 0x030;
pub const GPIO_HI_OE: u32 = SIO_BASE + 0x034;
pub const GPIO_OE_SET: u32 = SIO_BASE + 0x038;
pub const GPIO_HI_OE_SET: u32 = SIO_BASE + 0x03C;
pub const GPIO_OE_CLR: u32 = SIO_BASE + 0x040;
pub const GPIO_HI_OE_CLR: u32 = SIO_BASE + 0x044;
pub const GPIO_OE_XOR: u32 = SIO_BASE + 0x048;
pub const GPIO_HI_OE_XOR: u32 = SIO_BASE + 0x04C;

// FIFO registers
pub const FIFO_ST: u32 = SIO_BASE + 0x050;
pub const FIFO_WR: u32 = SIO_BASE + 0x054;
pub const FIFO_RD: u32 = SIO_BASE + 0x058;

// Spinlock base (32 spinlocks)
pub const SPINLOCK_BASE: u32 = SIO_BASE + 0x100;
pub const SPINLOCK_ST: u32 = SIO_BASE + 0x05C;

// FIFO status bits
pub const FIFO_ST_ROE: u32 = 1 << 3; // Read on empty
pub const FIFO_ST_WOF: u32 = 1 << 2; // Write on full
pub const FIFO_ST_RDY: u32 = 1 << 1; // Ready to read
pub const FIFO_ST_VLD: u32 = 1 << 0; // Valid data in FIFO

/// SIO peripheral driver
pub struct Sio;

impl Sio {
    /// Get current CPU core ID (0 or 1)
    #[inline]
    pub fn core_id() -> u32 {
        unsafe { core::ptr::read_volatile(CPUID as *const u32) }
    }

    /// Read all GPIO pins (GPIO 0-31)
    #[inline]
    pub fn gpio_in() -> u32 {
        Reg::new(GPIO_IN).read()
    }

    /// Read GPIO pins 32-47
    #[inline]
    pub fn gpio_hi_in() -> u32 {
        Reg::new(GPIO_HI_IN).read()
    }

    /// Set GPIO output value (GPIO 0-31)
    #[inline]
    pub fn gpio_out_set(mask: u32) {
        Reg::new(GPIO_OUT_SET).write(mask);
    }

    /// Clear GPIO output value (GPIO 0-31)
    #[inline]
    pub fn gpio_out_clr(mask: u32) {
        Reg::new(GPIO_OUT_CLR).write(mask);
    }

    /// Toggle GPIO output value (GPIO 0-31)
    #[inline]
    pub fn gpio_out_xor(mask: u32) {
        Reg::new(GPIO_OUT_XOR).write(mask);
    }

    /// Write GPIO output value (GPIO 0-31)
    #[inline]
    pub fn gpio_out_write(value: u32) {
        Reg::new(GPIO_OUT).write(value);
    }

    /// Enable GPIO output (GPIO 0-31)
    #[inline]
    pub fn gpio_oe_set(mask: u32) {
        Reg::new(GPIO_OE_SET).write(mask);
    }

    /// Disable GPIO output (GPIO 0-31)
    #[inline]
    pub fn gpio_oe_clr(mask: u32) {
        Reg::new(GPIO_OE_CLR).write(mask);
    }

    /// Read a specific GPIO pin
    #[inline]
    pub fn gpio_read(pin: u8) -> bool {
        if pin < 32 {
            (Self::gpio_in() & (1 << pin)) != 0
        } else {
            (Self::gpio_hi_in() & (1 << (pin - 32))) != 0
        }
    }
}

/// Hardware spinlock (0-31)
pub struct Spinlock {
    index: u8,
}

impl Spinlock {
    /// Create a new spinlock reference
    #[inline]
    pub const fn new(index: u8) -> Self {
        debug_assert!(index < 32);
        Self { index }
    }

    /// Get the register address for this spinlock
    #[inline]
    const fn addr(&self) -> u32 {
        SPINLOCK_BASE + (self.index as u32) * 4
    }

    /// Try to acquire the spinlock (non-blocking)
    ///
    /// Returns true if lock was acquired, false if already held
    #[inline]
    pub fn try_lock(&self) -> bool {
        unsafe { core::ptr::read_volatile(self.addr() as *const u32) != 0 }
    }

    /// Acquire the spinlock (blocking)
    #[inline]
    pub fn lock(&self) {
        while !self.try_lock() {
            cortex_m::asm::nop();
        }
    }

    /// Release the spinlock
    #[inline]
    pub fn unlock(&self) {
        unsafe { core::ptr::write_volatile(self.addr() as *mut u32, 1) }
    }

    /// Check which spinlocks are currently held
    #[inline]
    pub fn all_status() -> u32 {
        Reg::new(SPINLOCK_ST).read()
    }

    /// Release all spinlocks (typically done at startup)
    pub fn release_all() {
        for i in 0..32 {
            Spinlock::new(i).unlock();
        }
    }
}
