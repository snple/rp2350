//! Peripheral clock gating for RP2350
//!
//! This module defines [`ClkPeripheral`], an enum that maps each peripheral's
//! clock enable bit in the `WAKE_EN0/1`, `SLEEP_EN0/1`, and `ENABLED0/1`
//! register pairs.
//!
//! # Register Layout
//!
//! The clock enable state of all peripherals is controlled by two 32-bit
//! register pairs:
//!
//! ```text
//! WAKE_EN0  / SLEEP_EN0  / ENABLED0  →  bits  0–31  (ClkPeripheral variants 0–31)
//! WAKE_EN1  / SLEEP_EN1  / ENABLED1  →  bits  0–31  (ClkPeripheral variants 32–62)
//! ```
//!
//! # Usage
//!
//! Use [`Clocks::is_peripheral_clocked`], [`Clocks::set_wake_enabled`], and
//! [`Clocks::set_sleep_enabled`] to control clock gating at runtime.
//!
//! [`Clocks::is_peripheral_clocked`]: super::controller::Clocks::is_peripheral_clocked
//! [`Clocks::set_wake_enabled`]: super::controller::Clocks::set_wake_enabled
//! [`Clocks::set_sleep_enabled`]: super::controller::Clocks::set_sleep_enabled

// ============================================================================
// Peripheral Clock Identifiers
// ============================================================================
//
// The WAKE_EN0/1, SLEEP_EN0/1, and ENABLED0/1 registers control which
// peripherals receive clocks during wake, sleep, and the current active state.
// Each bit corresponds to exactly one peripheral clock.
//
// Variant values encode both the register index and bit position:
//   bits [5..N] → register index  (0 = EN0, 1 = EN1)
//   bits [0..4] → bit position within that register
//
// Use `ClkPeripheral::reg_bit()` to decode both at once.
// ============================================================================

/// Peripheral clock identifier for wake/sleep clock gating
///
/// Each variant corresponds to one bit in the `WAKE_EN0/1` or `SLEEP_EN0/1`
/// register pair. Variants 0–31 live in register 0; variants 32–62 in register 1.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkPeripheral {
    // =========================================================
    // ENABLED0 / WAKE_EN0 / SLEEP_EN0  (bits 0–31)
    // =========================================================
    /// CLOCKS peripheral itself
    Clocks = 0,
    /// Access control
    AccessCtrl = 1,
    /// ADC analog logic clock (clk_adc)
    Adc = 2,
    /// ADC system clock (clk_sys)
    SysAdc = 3,
    /// Boot RAM
    BootRam = 4,
    /// Bus control
    BusCtrl = 5,
    /// Bus fabric
    BusFabric = 6,
    /// DMA controller
    Dma = 7,
    /// Glitch detector peripheral
    GlitchDetector = 8,
    /// HSTX peripheral clock (clk_hstx)
    Hstx = 9,
    /// HSTX system clock (clk_sys)
    SysHstx = 10,
    /// I2C0
    I2c0 = 11,
    /// I2C1
    I2c1 = 12,
    /// IO bank controller
    Io = 13,
    /// JTAG debug interface
    Jtag = 14,
    /// OTP reference clock (clk_ref)
    RefOtp = 15,
    /// OTP system clock (clk_sys)
    SysOtp = 16,
    /// Pad controller
    Pads = 17,
    /// PIO0
    Pio0 = 18,
    /// PIO1
    Pio1 = 19,
    /// PIO2
    Pio2 = 20,
    /// PLL_SYS system clock
    SysPllSys = 21,
    /// PLL_USB system clock
    SysPllUsb = 22,
    /// Power manager reference clock (clk_ref)
    RefPowman = 23,
    /// Power manager system clock (clk_sys)
    SysPowman = 24,
    /// PWM
    Pwm = 25,
    /// Reset controller
    Resets = 26,
    /// ROM
    Rom = 27,
    /// ROSC controller (not the oscillator itself)
    SysRosc = 28,
    /// Power state machine
    Psm = 29,
    /// SHA-256 peripheral
    Sha256 = 30,
    /// SIO controller
    Sio = 31,

    // =========================================================
    // ENABLED1 / WAKE_EN1 / SLEEP_EN1  (bits 0–30, i.e. variants 32–62)
    // =========================================================
    /// SPI0 peripheral clock (clk_peri)
    PeriSpi0 = 32,
    /// SPI0 system clock (clk_sys)
    SysSpi0 = 33,
    /// SPI1 peripheral clock (clk_peri)
    PeriSpi1 = 34,
    /// SPI1 system clock (clk_sys)
    SysSpi1 = 35,
    /// SRAM bank 0
    Sram0 = 36,
    /// SRAM bank 1
    Sram1 = 37,
    /// SRAM bank 2
    Sram2 = 38,
    /// SRAM bank 3
    Sram3 = 39,
    /// SRAM bank 4
    Sram4 = 40,
    /// SRAM bank 5
    Sram5 = 41,
    /// SRAM bank 6
    Sram6 = 42,
    /// SRAM bank 7
    Sram7 = 43,
    /// SRAM bank 8
    Sram8 = 44,
    /// SRAM bank 9
    Sram9 = 45,
    /// System configuration registers
    SysCfg = 46,
    /// System information peripheral
    SysInfo = 47,
    /// Testbench manager
    Tbman = 48,
    /// TICKS reference clock (clk_ref)
    RefTicks = 49,
    /// TICKS system clock (clk_sys)
    SysTicks = 50,
    /// Timer 0
    Timer0 = 51,
    /// Timer 1
    Timer1 = 52,
    /// TrustZone Random Number Generator
    Trng = 53,
    /// UART0 peripheral clock (clk_peri)
    PeriUart0 = 54,
    /// UART0 system clock (clk_sys)
    SysUart0 = 55,
    /// UART1 peripheral clock (clk_peri)
    PeriUart1 = 56,
    /// UART1 system clock (clk_sys)
    SysUart1 = 57,
    /// USB controller system clock
    SysUsbCtrl = 58,
    /// USB clock (clk_usb)
    Usb = 59,
    /// Watchdog controller
    Watchdog = 60,
    /// XIP controller
    Xip = 61,
    /// XOSC controller (not the oscillator itself)
    SysXosc = 62,
}

impl ClkPeripheral {
    /// Decode the register index (0 or 1) and the bitmask for this peripheral
    ///
    /// Returns `(register_index, bit_mask)` where:
    /// - `register_index` is `0` for `WAKE/SLEEP/ENABLED0` or `1` for `…1`
    /// - `bit_mask` has exactly one bit set corresponding to this peripheral
    ///
    /// # Example
    /// ```ignore
    /// let (reg, bit) = ClkPeripheral::Usb.reg_bit();
    /// // reg == 1, bit == (1 << 27)
    /// ```
    #[inline]
    pub const fn reg_bit(self) -> (usize, u32) {
        let val = self as u32;
        ((val >> 5) as usize, 1u32 << (val & 0x1F))
    }
}
