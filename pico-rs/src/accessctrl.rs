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
//! use pico_rs::Peripherals;
//!
//! let dp = Peripherals::take().unwrap();
//! let accessctrl = Accessctrl::init(dp.ACCESSCTRL);
//!
//! // Lock core0 from modifying access control
//! accessctrl.lock_core0();
//!
//! // Check if DMA access is locked
//! if accessctrl.is_dma_locked() {
//!     // DMA cannot modify access control
//! }
//! ```

use crate::Steal;

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
/// Owns the PAC ACCESSCTRL peripheral to enforce proper ownership semantics.
pub struct Accessctrl {
    accessctrl: pico_rs_pac::ACCESSCTRL,
}

impl Accessctrl {
    /// Initialize the Accessctrl driver.
    ///
    /// # Arguments
    /// * `accessctrl` - The ACCESSCTRL peripheral from PAC
    #[inline]
    pub fn init(accessctrl: pico_rs_pac::ACCESSCTRL) -> Self {
        Self { accessctrl }
    }

    /// Obtain an Accessctrl handle without ownership checks.
    ///
    /// # Safety
    ///
    /// The caller must ensure the peripheral has been initialised
    /// and that no data races occur (e.g. only called from one core
    /// or properly synchronised).
    #[inline]
    pub unsafe fn steal() -> Steal<Self> {
        Steal(Self::init(unsafe { pico_rs_pac::ACCESSCTRL::steal() }))
    }

    /// Get reference to the underlying register block
    ///
    /// # Safety
    /// Direct register access can bypass the HAL's safety guarantees.
    /// The caller must ensure operations do not conflict with the HAL state.
    #[inline]
    pub unsafe fn regs(&self) -> &pico_rs_pac::accessctrl::RegisterBlock {
        &self.accessctrl
    }

    /// Release the ACCESSCTRL peripheral
    #[inline]
    pub fn release(self) -> pico_rs_pac::ACCESSCTRL {
        self.accessctrl
    }

    // ========================================================================
    // Lock Control
    // ========================================================================

    /// Lock core 0 from modifying access control
    ///
    /// Once locked, core 0 cannot modify ACCESSCTRL registers until reset.
    #[inline]
    pub fn lock_core0(&self) {
        self.accessctrl.lock.set_core0();
    }

    /// Lock core 1 from modifying access control
    ///
    /// Once locked, core 1 cannot modify ACCESSCTRL registers until reset.
    #[inline]
    pub fn lock_core1(&self) {
        self.accessctrl.lock.set_core1();
    }

    /// Lock debugger from modifying access control
    ///
    /// Once locked, debugger cannot modify ACCESSCTRL registers until reset.
    #[inline]
    pub fn lock_debug(&self) {
        self.accessctrl.lock.set_debug();
    }

    /// Check if core 0 is locked from modifying access control
    #[inline]
    pub fn is_core0_locked(&self) -> bool {
        self.accessctrl.lock.core0()
    }

    /// Check if core 1 is locked from modifying access control
    #[inline]
    pub fn is_core1_locked(&self) -> bool {
        self.accessctrl.lock.core1()
    }

    /// Check if DMA is locked (always locked, read-only)
    #[inline]
    pub fn is_dma_locked(&self) -> bool {
        self.accessctrl.lock.dma()
    }

    /// Check if debugger is locked from modifying access control
    #[inline]
    pub fn is_debug_locked(&self) -> bool {
        self.accessctrl.lock.debug()
    }

    /// Get the raw lock register value
    #[inline]
    pub fn lock_status(&self) -> u32 {
        self.accessctrl.lock.read_bits()
    }

    // ========================================================================
    // Force Core Permission (for specific masters on individual peripherals)
    // ========================================================================

    /// Configure GPIO bank 0 access permissions for core 0
    #[inline]
    pub fn set_io_bank0_core0_ns(&self, non_secure: bool) {
        if non_secure {
            self.accessctrl.io_bank0.set_nsu();
        } else {
            self.accessctrl.io_bank0.clr_nsu();
        }
    }

    /// Configure GPIO bank 0 access permissions for core 0 privilege
    #[inline]
    pub fn set_io_bank0_core0_su(&self, secure_unprivileged: bool) {
        if secure_unprivileged {
            self.accessctrl.io_bank0.set_su();
        } else {
            self.accessctrl.io_bank0.clr_su();
        }
    }

    // ========================================================================
    // Password Check
    // ========================================================================

    /// Check if password is required for CFGRESET
    #[inline]
    pub fn cfgreset_requires_password(&self) -> bool {
        self.accessctrl.cfgreset.read_bits() != 0
    }

    /// Write password for CFGRESET (if required)
    #[inline]
    pub fn write_cfgreset_password(&self, password: u32) {
        self.accessctrl.cfgreset.write_bits(password);
    }

    // ========================================================================
    // Peripheral Access Status (Read-only status for various peripherals)
    // ========================================================================

    /// Get ROM access control register value
    #[inline]
    pub fn rom_access(&self) -> u32 {
        self.accessctrl.rom.read_bits()
    }

    /// Get XIP main access control register value
    #[inline]
    pub fn xip_main_access(&self) -> u32 {
        self.accessctrl.xip_main.read_bits()
    }

    /// Get SRAM0 access control register value
    #[inline]
    pub fn sram0_access(&self) -> u32 {
        self.accessctrl.sram0.read_bits()
    }

    /// Get DMA access control register value
    #[inline]
    pub fn dma_access(&self) -> u32 {
        self.accessctrl.dma.read_bits()
    }

    /// Get UART0 access control register value
    #[inline]
    pub fn uart0_access(&self) -> u32 {
        self.accessctrl.uart0.read_bits()
    }

    /// Get UART1 access control register value
    #[inline]
    pub fn uart1_access(&self) -> u32 {
        self.accessctrl.uart1.read_bits()
    }

    /// Get SPI0 access control register value
    #[inline]
    pub fn spi0_access(&self) -> u32 {
        self.accessctrl.spi0.read_bits()
    }

    /// Get SPI1 access control register value
    #[inline]
    pub fn spi1_access(&self) -> u32 {
        self.accessctrl.spi1.read_bits()
    }

    /// Get I2C0 access control register value
    #[inline]
    pub fn i2c0_access(&self) -> u32 {
        self.accessctrl.i2c0.read_bits()
    }

    /// Get I2C1 access control register value
    #[inline]
    pub fn i2c1_access(&self) -> u32 {
        self.accessctrl.i2c1.read_bits()
    }

    /// Get PWM access control register value
    #[inline]
    pub fn pwm_access(&self) -> u32 {
        self.accessctrl.pwm.read_bits()
    }

    /// Get PIO0 access control register value
    #[inline]
    pub fn pio0_access(&self) -> u32 {
        self.accessctrl.pio0.read_bits()
    }

    /// Get PIO1 access control register value
    #[inline]
    pub fn pio1_access(&self) -> u32 {
        self.accessctrl.pio1.read_bits()
    }

    /// Get PIO2 access control register value
    #[inline]
    pub fn pio2_access(&self) -> u32 {
        self.accessctrl.pio2.read_bits()
    }

    /// Get ADC access control register value
    #[inline]
    pub fn adc0_access(&self) -> u32 {
        self.accessctrl.adc0.read_bits()
    }

    /// Get HSTX access control register value
    #[inline]
    pub fn hstx_access(&self) -> u32 {
        self.accessctrl.hstx.read_bits()
    }

    /// Get TIMER0 access control register value
    #[inline]
    pub fn timer0_access(&self) -> u32 {
        self.accessctrl.timer0.read_bits()
    }

    /// Get TIMER1 access control register value
    #[inline]
    pub fn timer1_access(&self) -> u32 {
        self.accessctrl.timer1.read_bits()
    }

    /// Get WATCHDOG access control register value
    #[inline]
    pub fn watchdog_access(&self) -> u32 {
        self.accessctrl.watchdog.read_bits()
    }

    /// Get TRNG access control register value
    #[inline]
    pub fn trng_access(&self) -> u32 {
        self.accessctrl.trng.read_bits()
    }

    /// Get SHA256 access control register value
    #[inline]
    pub fn sha256_access(&self) -> u32 {
        self.accessctrl.sha256.read_bits()
    }

    /// Get CLOCKS access control register value
    #[inline]
    pub fn clocks_access(&self) -> u32 {
        self.accessctrl.clocks.read_bits()
    }

    /// Get XOSC access control register value
    #[inline]
    pub fn xosc_access(&self) -> u32 {
        self.accessctrl.xosc.read_bits()
    }

    /// Get ROSC access control register value
    #[inline]
    pub fn rosc_access(&self) -> u32 {
        self.accessctrl.rosc.read_bits()
    }

    /// Get PLL_SYS access control register value
    #[inline]
    pub fn pll_sys_access(&self) -> u32 {
        self.accessctrl.pll_sys.read_bits()
    }

    /// Get PLL_USB access control register value
    #[inline]
    pub fn pll_usb_access(&self) -> u32 {
        self.accessctrl.pll_usb.read_bits()
    }

    /// Get OTP access control register value
    #[inline]
    pub fn otp_access(&self) -> u32 {
        self.accessctrl.otp.read_bits()
    }
}
