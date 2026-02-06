//! PLL (Phase-Locked Loop) HAL for RP2350
//!
//! This module provides access to the PLL_SYS and PLL_USB peripherals
//! for generating high-frequency clocks from the crystal oscillator.
//!
//! ## Features
//!
//! - Configure VCO frequency (750-1600 MHz)
//! - Configurable reference divider (1-63)
//! - Configurable feedback divider (16-320)
//! - Post dividers for final output frequency
//! - Lock detection
//!
//! ## PLL Output Frequency
//!
//! The PLL output frequency is calculated as:
//! ```text
//! VCO = (FREF / REFDIV) * FBDIV
//! Output = VCO / (POSTDIV1 * POSTDIV2)
//! ```
//!
//! ## Example
//!
//! ```ignore
//! use pico_rs::pll::{PllSys, PllUsb};
//! use pico_rs_pac::PLL_SYS;
//!
//! // Get PLL_SYS peripheral
//! let pll_sys_pac = unsafe { PLL_SYS::steal() };
//! let pll_sys = PllSys::new(pll_sys_pac);
//!
//! // Configure for 150 MHz (assuming 12 MHz XOSC)
//! // VCO = 12 MHz / 1 * 125 = 1500 MHz
//! // Output = 1500 MHz / (5 * 2) = 150 MHz
//! pll_sys.configure(1, 125, 5, 2);
//!
//! // Wait for lock
//! while !pll_sys.is_locked() {}
//! ```

use pico_rs_pac::{PLL_SYS, PLL_USB};

/// Macro to implement PLL peripheral
macro_rules! impl_pll {
    ($Pll:ident, $PLL:ident, $pll_mod:ident) => {
        /// PLL peripheral wrapper
        pub struct $Pll {
            pll: $PLL,
        }

        impl $Pll {
            /// Create a new PLL instance
            ///
            /// # Arguments
            /// * `pll` - The PLL peripheral from PAC
            #[inline]
            pub fn new(pll: $PLL) -> Self {
                Self { pll }
            }

            /// Get reference to the underlying register block
            ///
            /// # Safety
            /// Direct register access can bypass the HAL's safety guarantees.
            /// The caller must ensure operations do not conflict with the HAL state.
            #[inline]
            pub unsafe fn regs(&self) -> &pico_rs_pac::$pll_mod::RegisterBlock {
                &self.pll
            }

            /// Release the PLL peripheral
            #[inline]
            pub fn release(self) -> $PLL {
                self.pll
            }

            // ================================================================
            // Status
            // ================================================================

            /// Check if PLL is locked
            #[inline]
            pub fn is_locked(&self) -> bool {
                self.pll.cs.lock()
            }

            /// Check if PLL is NOT locked (sticky flag)
            ///
            /// This flag is set when PLL loses lock and stays set until cleared.
            #[inline]
            pub fn is_lock_n(&self) -> bool {
                self.pll.cs.lock_n()
            }

            /// Clear the lock_n sticky flag
            #[inline]
            pub fn clear_lock_n(&self) {
                self.pll.cs.set_lock_n();
            }

            /// Check if bypass mode is enabled
            #[inline]
            pub fn is_bypass(&self) -> bool {
                self.pll.cs.bypass()
            }

            /// Enable or disable bypass mode
            ///
            /// In bypass mode, the reference clock passes directly to output.
            #[inline]
            pub fn set_bypass(&self, bypass: bool) {
                if bypass {
                    self.pll.cs.set_bypass();
                } else {
                    self.pll.cs.clr_bypass();
                }
            }

            /// Get reference divider value
            #[inline]
            pub fn refdiv(&self) -> u8 {
                self.pll.cs.refdiv() as u8
            }

            /// Set reference divider (1-63)
            ///
            /// The reference frequency (XOSC) is divided by this before the PLL.
            /// FREF = XOSC / REFDIV
            #[inline]
            pub fn set_refdiv(&self, div: u8) {
                let div = div.clamp(1, 63) as u32;
                self.pll.cs.write_refdiv(div);
            }

            // ================================================================
            // Power Control
            // ================================================================

            /// Power down the entire PLL
            #[inline]
            pub fn power_down(&self) {
                self.pll.pwr.set_pd();
            }

            /// Power up the PLL (clear power-down bit)
            #[inline]
            pub fn power_up(&self) {
                self.pll.pwr.clr_pd();
            }

            /// Check if PLL is powered down
            #[inline]
            pub fn is_powered_down(&self) -> bool {
                self.pll.pwr.pd()
            }

            /// Power down the VCO
            #[inline]
            pub fn vco_power_down(&self) {
                self.pll.pwr.set_vcopd();
            }

            /// Power up the VCO
            #[inline]
            pub fn vco_power_up(&self) {
                self.pll.pwr.clr_vcopd();
            }

            /// Check if VCO is powered down
            #[inline]
            pub fn is_vco_powered_down(&self) -> bool {
                self.pll.pwr.vcopd()
            }

            /// Power down the post dividers
            #[inline]
            pub fn postdiv_power_down(&self) {
                self.pll.pwr.set_postdivpd();
            }

            /// Power up the post dividers
            #[inline]
            pub fn postdiv_power_up(&self) {
                self.pll.pwr.clr_postdivpd();
            }

            /// Check if post dividers are powered down
            #[inline]
            pub fn is_postdiv_powered_down(&self) -> bool {
                self.pll.pwr.postdivpd()
            }

            /// Power down the DSM (delta-sigma modulator)
            #[inline]
            pub fn dsm_power_down(&self) {
                self.pll.pwr.set_dsmpd();
            }

            /// Power up the DSM
            #[inline]
            pub fn dsm_power_up(&self) {
                self.pll.pwr.clr_dsmpd();
            }

            // ================================================================
            // Divider Configuration
            // ================================================================

            /// Get feedback divider value
            #[inline]
            pub fn fbdiv(&self) -> u16 {
                self.pll.fbdiv_int.fbdiv_int() as u16
            }

            /// Set feedback divider (16-320)
            ///
            /// VCO frequency = (FREF / REFDIV) * FBDIV
            /// VCO must be 750-1600 MHz
            #[inline]
            pub fn set_fbdiv(&self, div: u16) {
                let div = div.clamp(16, 320) as u32;
                self.pll.fbdiv_int.write_fbdiv_int(div);
            }

            /// Get post divider 1 value
            #[inline]
            pub fn postdiv1(&self) -> u8 {
                self.pll.prim.postdiv1() as u8
            }

            /// Get post divider 2 value
            #[inline]
            pub fn postdiv2(&self) -> u8 {
                self.pll.prim.postdiv2() as u8
            }

            /// Set post dividers (1-7 each)
            ///
            /// Output = VCO / (POSTDIV1 * POSTDIV2)
            /// For best performance, use POSTDIV1 >= POSTDIV2
            #[inline]
            pub fn set_postdiv(&self, postdiv1: u8, postdiv2: u8) {
                let postdiv1 = postdiv1.clamp(1, 7) as u32;
                let postdiv2 = postdiv2.clamp(1, 7) as u32;
                self.pll.prim.write_postdiv1(postdiv1);
                self.pll.prim.write_postdiv2(postdiv2);
            }

            // ================================================================
            // Interrupts
            // ================================================================

            /// Check if lock_n interrupt is pending (raw)
            #[inline]
            pub fn is_intr_lock_n(&self) -> bool {
                self.pll.intr.lock_n_sticky()
            }

            /// Clear lock_n interrupt
            #[inline]
            pub fn clear_intr_lock_n(&self) {
                self.pll.intr.set_lock_n_sticky();
            }

            /// Enable lock_n interrupt
            #[inline]
            pub fn enable_intr_lock_n(&self) {
                self.pll.inte.set_lock_n_sticky();
            }

            /// Disable lock_n interrupt
            #[inline]
            pub fn disable_intr_lock_n(&self) {
                self.pll.inte.clr_lock_n_sticky();
            }

            /// Check if lock_n interrupt is enabled
            #[inline]
            pub fn is_intr_lock_n_enabled(&self) -> bool {
                self.pll.inte.lock_n_sticky()
            }

            /// Check masked interrupt status
            #[inline]
            pub fn is_ints_lock_n(&self) -> bool {
                self.pll.ints.lock_n_sticky()
            }

            // ================================================================
            // High-Level Configuration
            // ================================================================

            /// Configure the PLL with all parameters
            ///
            /// # Arguments
            /// * `refdiv` - Reference divider (1-63)
            /// * `fbdiv` - Feedback divider (16-320)
            /// * `postdiv1` - Post divider 1 (1-7)
            /// * `postdiv2` - Post divider 2 (1-7)
            ///
            /// # Example
            ///
            /// For 150 MHz output from 12 MHz XOSC:
            /// ```ignore
            /// pll.configure(1, 125, 5, 2);
            /// // VCO = 12 / 1 * 125 = 1500 MHz
            /// // Out = 1500 / (5 * 2) = 150 MHz
            /// ```
            pub fn configure(&self, refdiv: u8, fbdiv: u16, postdiv1: u8, postdiv2: u8) {
                // After reset, all power bits should be set (PLL off)
                // Configure dividers first while PLL is off
                self.set_refdiv(refdiv);
                self.set_fbdiv(fbdiv);

                // Clear PD and VCOPD to turn on PLL and VCO
                // Keep POSTDIVPD set (post dividers still off)
                // Keep DSMPD set (DSM not used for integer mode)
                let power_on_mask = (1 << 0) | (1 << 5); // PD | VCOPD
                self.pll.pwr.clr_bits(power_on_mask);

                // Wait for VCO to lock
                while !self.is_locked() {
                    pico_rs_rt::asm::nop();
                }

                // Configure post dividers
                self.set_postdiv(postdiv1, postdiv2);

                // Power up post dividers
                self.postdiv_power_up();

                // Clear any lock loss indication
                self.clear_lock_n();
            }

            /// Wait for PLL to lock
            #[inline]
            pub fn wait_lock(&self) {
                while !self.is_locked() {
                    pico_rs_rt::asm::nop();
                }
            }

            /// Disable the PLL (power down all components)
            pub fn disable(&self) {
                self.postdiv_power_down();
                self.vco_power_down();
                self.power_down();
            }
        }
    };
}

// Implement PLL for both PLL_SYS and PLL_USB
impl_pll!(PllSys, PLL_SYS, pll_sys);
impl_pll!(PllUsb, PLL_USB, pll_usb);

/// Common PLL configuration for typical use cases
pub struct PllConfig {
    /// Reference divider
    pub refdiv: u8,
    /// Feedback divider
    pub fbdiv: u16,
    /// Post divider 1
    pub postdiv1: u8,
    /// Post divider 2
    pub postdiv2: u8,
}

impl PllConfig {
    /// Create config for 125 MHz from 12 MHz XOSC
    pub const fn sys_125mhz() -> Self {
        Self {
            refdiv: 1,
            fbdiv: 125,
            postdiv1: 6,
            postdiv2: 2,
        }
    }

    /// Create config for 133 MHz from 12 MHz XOSC
    pub const fn sys_133mhz() -> Self {
        Self {
            refdiv: 1,
            fbdiv: 133,
            postdiv1: 6,
            postdiv2: 2,
        }
    }

    /// Create config for 150 MHz from 12 MHz XOSC
    pub const fn sys_150mhz() -> Self {
        Self {
            refdiv: 1,
            fbdiv: 125,
            postdiv1: 5,
            postdiv2: 2,
        }
    }

    /// Create config for 48 MHz from 12 MHz XOSC (USB)
    pub const fn usb_48mhz() -> Self {
        Self {
            refdiv: 1,
            fbdiv: 100,
            postdiv1: 5,
            postdiv2: 5,
        }
    }

    /// Calculate VCO frequency in kHz
    pub const fn vco_freq_khz(&self, xosc_khz: u32) -> u32 {
        (xosc_khz / self.refdiv as u32) * self.fbdiv as u32
    }

    /// Calculate output frequency in kHz
    pub const fn output_freq_khz(&self, xosc_khz: u32) -> u32 {
        self.vco_freq_khz(xosc_khz) / (self.postdiv1 as u32 * self.postdiv2 as u32)
    }
}
