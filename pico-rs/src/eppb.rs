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
//! use pico_rs::eppb::Eppb;
//!
//! let eppb = Eppb::new();
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

use pico_rs_pac::EPPB;

/// EPPB peripheral wrapper
///
/// Provides access to Extended Private Peripheral Bus registers
/// for NMI masking and sleep control.
pub struct Eppb {
    regs: &'static pico_rs_pac::eppb::RegisterBlock,
}

impl Eppb {
    /// Create a new Eppb instance
    #[inline]
    pub fn new() -> Self {
        Self {
            regs: unsafe { &*EPPB::PTR },
        }
    }

    /// Get reference to the register block
    #[inline]
    pub fn regs(&self) -> &pico_rs_pac::eppb::RegisterBlock {
        self.regs
    }

    // ========================================================================
    // NMI Mask Control
    // ========================================================================

    /// Get NMI mask for IRQs 0-31
    #[inline]
    pub fn nmi_mask0(&self) -> u32 {
        self.regs.nmi_mask0.nmi_mask0()
    }

    /// Set NMI mask for IRQs 0-31
    ///
    /// Each bit corresponds to an IRQ. Setting a bit causes that IRQ
    /// to be escalated to an NMI (Non-Maskable Interrupt).
    #[inline]
    pub fn set_nmi_mask0(&self, mask: u32) {
        self.regs.nmi_mask0.write_nmi_mask0(mask);
    }

    /// Enable NMI for a specific IRQ (0-31)
    #[inline]
    pub fn enable_nmi_irq0_31(&self, irq: u8) {
        if irq < 32 {
            self.regs.nmi_mask0.modify_bits(|r| r | (1 << irq));
        }
    }

    /// Disable NMI for a specific IRQ (0-31)
    #[inline]
    pub fn disable_nmi_irq0_31(&self, irq: u8) {
        if irq < 32 {
            self.regs.nmi_mask0.modify_bits(|r| r & !(1 << irq));
        }
    }

    /// Get NMI mask for IRQs 32-51
    #[inline]
    pub fn nmi_mask1(&self) -> u32 {
        self.regs.nmi_mask1.nmi_mask1()
    }

    /// Set NMI mask for IRQs 32-51
    ///
    /// Only bits [19:0] are valid (for IRQs 32-51).
    #[inline]
    pub fn set_nmi_mask1(&self, mask: u32) {
        self.regs.nmi_mask1.write_nmi_mask1(mask & 0x000F_FFFF);
    }

    /// Enable NMI for a specific IRQ (32-51)
    #[inline]
    pub fn enable_nmi_irq32_51(&self, irq: u8) {
        if (32..52).contains(&irq) {
            self.regs.nmi_mask1.modify_bits(|r| r | (1 << (irq - 32)));
        }
    }

    /// Disable NMI for a specific IRQ (32-51)
    #[inline]
    pub fn disable_nmi_irq32_51(&self, irq: u8) {
        if (32..52).contains(&irq) {
            self.regs.nmi_mask1.modify_bits(|r| r & !(1 << (irq - 32)));
        }
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
        self.regs.sleepctrl.light_sleep()
    }

    /// Set light sleep mode
    ///
    /// When enabled, any processor sleep will NOT deassert the system-level
    /// clock request, keeping clocks running during WFI/WFE.
    /// When disabled (default), processor sleep deasserts clock request.
    #[inline]
    pub fn set_light_sleep(&self, enable: bool) {
        if enable {
            self.regs.sleepctrl.set_light_sleep();
        } else {
            self.regs.sleepctrl.clr_light_sleep();
        }
    }

    /// Get WIC enable request status
    ///
    /// Returns true if WIC (Wake-up Interrupt Controller) sleep is requested.
    #[inline]
    pub fn is_wicenreq(&self) -> bool {
        self.regs.sleepctrl.wicenreq()
    }

    /// Request WIC sleep for next deep sleep
    ///
    /// After setting this, poll `is_wicenack()` until it returns true
    /// to confirm WIC mode is ready, then issue WFI/WFE.
    #[inline]
    pub fn set_wicenreq(&self, enable: bool) {
        if enable {
            self.regs.sleepctrl.set_wicenreq();
        } else {
            self.regs.sleepctrl.clr_wicenreq();
        }
    }

    /// Get WIC enable acknowledge status
    ///
    /// Returns true when the interrupt controller has acknowledged
    /// the WIC enable request. Poll this after setting `set_wicenreq(true)`
    /// before entering deep sleep.
    #[inline]
    pub fn is_wicenack(&self) -> bool {
        self.regs.sleepctrl.wicenack()
    }

    /// Wait for WIC enable to be acknowledged
    ///
    /// Blocks until WIC mode is ready after requesting WIC sleep.
    #[inline]
    pub fn wait_wicenack(&self) {
        while !self.is_wicenack() {
            pico_rs_rt::asm::nop();
        }
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

impl Default for Eppb {
    fn default() -> Self {
        Self::new()
    }
}
