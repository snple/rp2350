//! XOSC (Crystal Oscillator) peripheral registers

use super::Reg;
use super::addr::XOSC_BASE;

// Register offsets
pub const CTRL: u32 = XOSC_BASE + 0x00;
pub const STATUS: u32 = XOSC_BASE + 0x04;
pub const DORMANT: u32 = XOSC_BASE + 0x08;
pub const STARTUP: u32 = XOSC_BASE + 0x0C;
pub const COUNT: u32 = XOSC_BASE + 0x10;

// CTRL bits
pub const CTRL_ENABLE: u32 = 0xFAB << 12;
pub const CTRL_DISABLE: u32 = 0xD1E << 12;
pub const CTRL_ENABLE_MASK: u32 = 0xFFF << 12;
pub const CTRL_FREQ_1_15MHZ: u32 = 0xAA0;
pub const CTRL_FREQ_MASK: u32 = 0xFFF;

// STATUS bits
pub const STATUS_STABLE: u32 = 1 << 31;
pub const STATUS_BADWRITE: u32 = 1 << 24;
pub const STATUS_ENABLED: u32 = 1 << 12;
pub const STATUS_FREQ_1_15MHZ: u32 = 0;

/// XOSC (Crystal Oscillator) driver
pub struct Xosc;

impl Xosc {
    const CTRL: Reg = Reg::new(CTRL);
    const STATUS: Reg = Reg::new(STATUS);
    const STARTUP: Reg = Reg::new(STARTUP);

    /// Initialize and enable the crystal oscillator
    ///
    /// `startup_delay` is the number of clock cycles to wait for startup.
    /// For a 12 MHz crystal, typical value is about 47 (1 ms startup)
    pub fn init(startup_delay: u32) {
        // Set frequency range (1-15 MHz covers most crystals including 12 MHz)
        Self::CTRL.write(CTRL_FREQ_1_15MHZ);

        // Set startup delay
        // Delay = (startup_delay + 1) * 256 / f_xosc
        // For 12 MHz and 1ms: delay = 12000 / 256 - 1 ≈ 47
        Self::STARTUP.write(startup_delay);

        // Enable XOSC
        Self::CTRL.set_bits(CTRL_ENABLE);

        // Wait for stable
        Self::STATUS.wait_set(STATUS_STABLE);
    }

    /// Disable the crystal oscillator
    #[inline]
    pub fn disable() {
        Self::CTRL.modify(|v| (v & !CTRL_ENABLE_MASK) | CTRL_DISABLE);
    }

    /// Check if XOSC is stable
    #[inline]
    pub fn is_stable() -> bool {
        Self::STATUS.is_set(STATUS_STABLE)
    }

    /// Check if XOSC is enabled
    #[inline]
    pub fn is_enabled() -> bool {
        Self::STATUS.is_set(STATUS_ENABLED)
    }
}
