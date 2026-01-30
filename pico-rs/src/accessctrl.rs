//! ACCESSCTRL (Access Control) HAL for RP2350
//!
//! This module provides access to the hardware access control registers
//! for managing peripheral and memory access permissions.
//!
//! ## Features
//!
//! - Lock access control for individual masters
//! - Configure peripheral access permissions
//! - Security level configuration (Secure/Non-secure, Privileged/Unprivileged)
//!
//! ## Example
//!
//! ```ignore
//! use pico_rs::accessctrl::Accessctrl;
//!
//! let accessctrl = Accessctrl::new();
//!
//! // Lock core0 from modifying access control
//! accessctrl.lock_core0();
//!
//! // Check if DMA access is locked
//! if accessctrl.is_dma_locked() {
//!     // DMA cannot modify access control
//! }
//! ```

use pico_rs_pac::ACCESSCTRL;

/// Security/Privilege level for peripheral access
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AccessLevel {
    /// Secure Privileged access only
    SecurePrivileged,
    /// Secure access (privileged or unprivileged)
    Secure,
    /// Non-secure Privileged access only
    NonSecurePrivileged,
    /// Any access level
    Any,
}

/// ACCESSCTRL peripheral wrapper
///
/// Provides access to hardware access control configuration.
pub struct Accessctrl {
    regs: &'static pico_rs_pac::accessctrl::RegisterBlock,
}

impl Accessctrl {
    /// Create a new Accessctrl instance
    #[inline]
    pub fn new() -> Self {
        Self {
            regs: unsafe { &*ACCESSCTRL::PTR },
        }
    }

    /// Get reference to the register block
    #[inline]
    pub fn regs(&self) -> &pico_rs_pac::accessctrl::RegisterBlock {
        self.regs
    }

    // ========================================================================
    // Lock Control
    // ========================================================================

    /// Lock core 0 from modifying access control
    ///
    /// Once locked, core 0 cannot modify ACCESSCTRL registers until reset.
    #[inline]
    pub fn lock_core0(&self) {
        self.regs.lock.set_core0();
    }

    /// Lock core 1 from modifying access control
    ///
    /// Once locked, core 1 cannot modify ACCESSCTRL registers until reset.
    #[inline]
    pub fn lock_core1(&self) {
        self.regs.lock.set_core1();
    }

    /// Lock debugger from modifying access control
    ///
    /// Once locked, debugger cannot modify ACCESSCTRL registers until reset.
    #[inline]
    pub fn lock_debug(&self) {
        self.regs.lock.set_debug();
    }

    /// Check if core 0 is locked from modifying access control
    #[inline]
    pub fn is_core0_locked(&self) -> bool {
        self.regs.lock.core0()
    }

    /// Check if core 1 is locked from modifying access control
    #[inline]
    pub fn is_core1_locked(&self) -> bool {
        self.regs.lock.core1()
    }

    /// Check if DMA is locked (always locked, read-only)
    #[inline]
    pub fn is_dma_locked(&self) -> bool {
        self.regs.lock.dma()
    }

    /// Check if debugger is locked from modifying access control
    #[inline]
    pub fn is_debug_locked(&self) -> bool {
        self.regs.lock.debug()
    }

    /// Get the raw lock register value
    #[inline]
    pub fn lock_status(&self) -> u32 {
        self.regs.lock.read_bits()
    }

    // ========================================================================
    // Force Core Permission (for specific masters on individual peripherals)
    // ========================================================================

    /// Configure GPIO bank 0 access permissions for core 0
    #[inline]
    pub fn set_io_bank0_core0_ns(&self, non_secure: bool) {
        if non_secure {
            self.regs.io_bank0.set_nsu();
        } else {
            self.regs.io_bank0.clr_nsu();
        }
    }

    /// Configure GPIO bank 0 access permissions for core 0 privilege
    #[inline]
    pub fn set_io_bank0_core0_su(&self, secure_unprivileged: bool) {
        if secure_unprivileged {
            self.regs.io_bank0.set_su();
        } else {
            self.regs.io_bank0.clr_su();
        }
    }

    // ========================================================================
    // Password Check
    // ========================================================================

    /// Check if password is required for CFGRESET
    #[inline]
    pub fn cfgreset_requires_password(&self) -> bool {
        self.regs.cfgreset.read_bits() != 0
    }

    /// Write password for CFGRESET (if required)
    #[inline]
    pub fn write_cfgreset_password(&self, password: u32) {
        self.regs.cfgreset.write_bits(password);
    }

    // ========================================================================
    // Peripheral Access Status (Read-only status for various peripherals)
    // ========================================================================

    /// Get ROM access control register value
    #[inline]
    pub fn rom_access(&self) -> u32 {
        self.regs.rom.read_bits()
    }

    /// Get XIP main access control register value
    #[inline]
    pub fn xip_main_access(&self) -> u32 {
        self.regs.xip_main.read_bits()
    }

    /// Get SRAM0 access control register value
    #[inline]
    pub fn sram0_access(&self) -> u32 {
        self.regs.sram0.read_bits()
    }

    /// Get DMA access control register value
    #[inline]
    pub fn dma_access(&self) -> u32 {
        self.regs.dma.read_bits()
    }

    /// Get UART0 access control register value
    #[inline]
    pub fn uart0_access(&self) -> u32 {
        self.regs.uart0.read_bits()
    }

    /// Get UART1 access control register value
    #[inline]
    pub fn uart1_access(&self) -> u32 {
        self.regs.uart1.read_bits()
    }

    /// Get SPI0 access control register value
    #[inline]
    pub fn spi0_access(&self) -> u32 {
        self.regs.spi0.read_bits()
    }

    /// Get SPI1 access control register value
    #[inline]
    pub fn spi1_access(&self) -> u32 {
        self.regs.spi1.read_bits()
    }

    /// Get I2C0 access control register value
    #[inline]
    pub fn i2c0_access(&self) -> u32 {
        self.regs.i2c0.read_bits()
    }

    /// Get I2C1 access control register value
    #[inline]
    pub fn i2c1_access(&self) -> u32 {
        self.regs.i2c1.read_bits()
    }

    /// Get PWM access control register value
    #[inline]
    pub fn pwm_access(&self) -> u32 {
        self.regs.pwm.read_bits()
    }

    /// Get PIO0 access control register value
    #[inline]
    pub fn pio0_access(&self) -> u32 {
        self.regs.pio0.read_bits()
    }

    /// Get PIO1 access control register value
    #[inline]
    pub fn pio1_access(&self) -> u32 {
        self.regs.pio1.read_bits()
    }

    /// Get PIO2 access control register value
    #[inline]
    pub fn pio2_access(&self) -> u32 {
        self.regs.pio2.read_bits()
    }

    /// Get ADC access control register value
    #[inline]
    pub fn adc0_access(&self) -> u32 {
        self.regs.adc0.read_bits()
    }

    /// Get HSTX access control register value
    #[inline]
    pub fn hstx_access(&self) -> u32 {
        self.regs.hstx.read_bits()
    }

    /// Get TIMER0 access control register value
    #[inline]
    pub fn timer0_access(&self) -> u32 {
        self.regs.timer0.read_bits()
    }

    /// Get TIMER1 access control register value
    #[inline]
    pub fn timer1_access(&self) -> u32 {
        self.regs.timer1.read_bits()
    }

    /// Get WATCHDOG access control register value
    #[inline]
    pub fn watchdog_access(&self) -> u32 {
        self.regs.watchdog.read_bits()
    }

    /// Get TRNG access control register value
    #[inline]
    pub fn trng_access(&self) -> u32 {
        self.regs.trng.read_bits()
    }

    /// Get SHA256 access control register value
    #[inline]
    pub fn sha256_access(&self) -> u32 {
        self.regs.sha256.read_bits()
    }

    /// Get CLOCKS access control register value
    #[inline]
    pub fn clocks_access(&self) -> u32 {
        self.regs.clocks.read_bits()
    }

    /// Get XOSC access control register value
    #[inline]
    pub fn xosc_access(&self) -> u32 {
        self.regs.xosc.read_bits()
    }

    /// Get ROSC access control register value
    #[inline]
    pub fn rosc_access(&self) -> u32 {
        self.regs.rosc.read_bits()
    }

    /// Get PLL_SYS access control register value
    #[inline]
    pub fn pll_sys_access(&self) -> u32 {
        self.regs.pll_sys.read_bits()
    }

    /// Get PLL_USB access control register value
    #[inline]
    pub fn pll_usb_access(&self) -> u32 {
        self.regs.pll_usb.read_bits()
    }

    /// Get OTP access control register value
    #[inline]
    pub fn otp_access(&self) -> u32 {
        self.regs.otp.read_bits()
    }
}

impl Default for Accessctrl {
    fn default() -> Self {
        Self::new()
    }
}
