//! EPPB (Extended Private Peripheral Bus) HAL for RP2350
//!
//! This module provides access to the Cortex-M33 EPPB vendor register block
//! for RP2350, including NMI mask control and sleep configuration.
//!
//! ## Features
//!
//! - NMI (Non-Maskable Interrupt) mask configuration for IRQs 0-51
//! - Sleep control (light sleep vs deep sleep)
//! - WIC (Wake-up Interrupt Controller) enable
//!
//! ## Example
//!
//! ```ignore
//! use rp2350::eppb::Eppb;
//! use rp2350::Peripherals;
//!
//! let dp = Peripherals::take().unwrap();
//! let eppb = Eppb::init(dp.EPPB);
//!
//! // Mask IRQ 5 as NMI
//! eppb.set_nmi_mask0(1 << 5);
//!
//! // Enable light sleep mode
//! eppb.set_light_sleep(true);
//! ```
//!
//! ## Note
//!
//! These registers are core-local and are reset by a processor warm reset.

use rp2350_pac::EPPB;

use crate::Steal;

/// EPPB peripheral wrapper
///
/// Provides access to Extended Private Peripheral Bus registers
/// for NMI masking and sleep control.
pub struct Eppb {
    eppb: EPPB,
}

impl Eppb {
    /// Initialize the Eppb driver.
    ///
    /// # Arguments
    /// * `eppb` - The EPPB peripheral from PAC
    #[inline]
    pub fn init(eppb: EPPB) -> Self {
        Self { eppb }
    }

    /// Obtain an Eppb handle without ownership checks.
    ///
    /// # Safety
    ///
    /// The caller must ensure the peripheral has been initialised
    /// and that no data races occur.
    #[inline]
    pub unsafe fn steal() -> Steal<Self> {
        Steal(Self::init(unsafe { EPPB::steal() }))
    }

    /// Get reference to the underlying register block
    ///
    /// # Safety
    /// Direct register access can bypass the HAL's safety guarantees.
    /// The caller must ensure operations do not conflict with the HAL state.
    #[inline]
    pub unsafe fn regs(&self) -> &rp2350_pac::eppb::RegisterBlock {
        &self.eppb
    }

    /// Release the EPPB peripheral
    #[inline]
    pub fn release(self) -> EPPB {
        self.eppb
    }

    // ========================================================================
    // NMI Mask Control
    // ========================================================================

    /// Get NMI mask for IRQs 0-31
    #[inline]
    pub fn nmi_mask0(&self) -> u32 {
        self.eppb.nmi_mask0.nmi_mask0()
    }

    /// Set NMI mask for IRQs 0-31
    ///
    /// Each bit corresponds to an IRQ. Setting a bit causes that IRQ
    /// to be escalated to an NMI (Non-Maskable Interrupt).
    #[inline]
    pub fn set_nmi_mask0(&self, mask: u32) {
        self.eppb.nmi_mask0.write_nmi_mask0(mask);
    }

    /// Enable NMI for a specific IRQ (0-51).
    ///
    /// IRQ 0-31 map to `NMI_MASK0`, IRQ 32-51 map to `NMI_MASK1`.
    ///
    /// # Panics
    /// Panics if `irq > 51`.
    #[inline]
    pub fn enable_nmi(&self, irq: u8) {
        match irq {
            0..=31 => self.eppb.nmi_mask0.modify_bits(|r| r | (1 << irq)),
            32..=51 => self.eppb.nmi_mask1.modify_bits(|r| r | (1 << (irq - 32))),
            _ => panic!("invalid IRQ for NMI: {irq}"),
        }
    }

    /// Disable NMI for a specific IRQ (0-51).
    ///
    /// # Panics
    /// Panics if `irq > 51`.
    #[inline]
    pub fn disable_nmi(&self, irq: u8) {
        match irq {
            0..=31 => self.eppb.nmi_mask0.modify_bits(|r| r & !(1 << irq)),
            32..=51 => self.eppb.nmi_mask1.modify_bits(|r| r & !(1 << (irq - 32))),
            _ => panic!("invalid IRQ for NMI: {irq}"),
        }
    }

    /// Get NMI mask for IRQs 32-51
    #[inline]
    pub fn nmi_mask1(&self) -> u32 {
        self.eppb.nmi_mask1.nmi_mask1()
    }

    /// Set NMI mask for IRQs 32-51
    ///
    /// Only bits [19:0] are valid (for IRQs 32-51).
    #[inline]
    pub fn set_nmi_mask1(&self, mask: u32) {
        self.eppb.nmi_mask1.write_nmi_mask1(mask & 0x000F_FFFF);
    }

    // ========================================================================
    // Sleep Control
    // ========================================================================

    /// Get light sleep status
    ///
    /// When true, processor sleep will NOT deassert the system-level
    /// clock request (light sleep mode).
    #[inline]
    pub fn is_light_sleep(&self) -> bool {
        self.eppb.sleepctrl.light_sleep()
    }

    /// Set light sleep mode
    ///
    /// When enabled, any processor sleep will NOT deassert the system-level
    /// clock request, keeping clocks running during WFI/WFE.
    /// When disabled (default), processor sleep deasserts clock request.
    #[inline]
    pub fn set_light_sleep(&self, enable: bool) {
        if enable {
            self.eppb.sleepctrl.set_light_sleep();
        } else {
            self.eppb.sleepctrl.clr_light_sleep();
        }
    }

    /// Get WIC enable request status
    ///
    /// Returns true if WIC (Wake-up Interrupt Controller) sleep is requested.
    #[inline]
    pub fn is_wicenreq(&self) -> bool {
        self.eppb.sleepctrl.wicenreq()
    }

    /// Request WIC sleep for next deep sleep
    ///
    /// After setting this, poll `is_wicenack()` until it returns true
    /// to confirm WIC mode is ready, then issue WFI/WFE.
    #[inline]
    pub fn set_wicenreq(&self, enable: bool) {
        if enable {
            self.eppb.sleepctrl.set_wicenreq();
        } else {
            self.eppb.sleepctrl.clr_wicenreq();
        }
    }

    /// Get WIC enable acknowledge status
    ///
    /// Returns true when the interrupt controller has acknowledged
    /// the WIC enable request. Poll this after setting `set_wicenreq(true)`
    /// before entering deep sleep.
    #[inline]
    pub fn is_wicenack(&self) -> bool {
        self.eppb.sleepctrl.wicenack()
    }

    /// Wait for WIC enable to be acknowledged
    ///
    /// Blocks until WIC mode is ready after requesting WIC sleep.
    ///
    /// # Panics
    ///
    /// Panics if WIC is not acknowledged within
    /// [`crate::spin::SPIN_TIMEOUT`] iterations.
    #[inline]
    pub fn wait_wicenack(&self) {
        crate::spin::spin_wait("WIC enable acknowledgement", || self.is_wicenack());
    }

    /// Prepare for WIC sleep
    ///
    /// Convenience method that requests WIC sleep and waits for acknowledgment.
    #[inline]
    pub fn prepare_wic_sleep(&self) {
        self.set_wicenreq(true);
        self.wait_wicenack();
    }
}
