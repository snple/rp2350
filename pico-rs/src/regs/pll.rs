//! PLL (Phase-Locked Loop) peripheral registers
//!
//! RP2350 has two PLLs: PLL_SYS and PLL_USB

use super::addr::{PLL_SYS_BASE, PLL_USB_BASE};
use super::Reg;

// Register offsets (same for both PLLs)
pub const CS: u32 = 0x00;        // Control and Status
pub const PWR: u32 = 0x04;       // Power control
pub const FBDIV_INT: u32 = 0x08; // Feedback divisor
pub const PRIM: u32 = 0x0C;      // Post dividers

// CS bits
pub const CS_LOCK: u32 = 1 << 31;
pub const CS_BYPASS: u32 = 1 << 8;
pub const CS_REFDIV_MASK: u32 = 0x3F;

// PWR bits (active low - 1 means powered down)
pub const PWR_VCOPD: u32 = 1 << 5;     // VCO power down
pub const PWR_POSTDIVPD: u32 = 1 << 3; // Post divider power down
pub const PWR_DSMPD: u32 = 1 << 2;     // DSM power down
pub const PWR_PD: u32 = 1 << 0;        // PLL power down

// PRIM bits
pub const PRIM_POSTDIV1_SHIFT: u32 = 16;
pub const PRIM_POSTDIV2_SHIFT: u32 = 12;
pub const PRIM_POSTDIV1_MASK: u32 = 0x7 << PRIM_POSTDIV1_SHIFT;
pub const PRIM_POSTDIV2_MASK: u32 = 0x7 << PRIM_POSTDIV2_SHIFT;

/// PLL configuration
#[derive(Clone, Copy)]
pub struct PllConfig {
    /// Reference divider (1-63)
    pub refdiv: u8,
    /// Feedback divider (16-320)
    pub fbdiv: u16,
    /// Post divider 1 (1-7)
    pub post_div1: u8,
    /// Post divider 2 (1-7)
    pub post_div2: u8,
}

impl PllConfig {
    /// Configuration for 150 MHz from 12 MHz crystal
    /// VCO = 12 MHz / 1 * 125 = 1500 MHz
    /// Output = 1500 MHz / 5 / 2 = 150 MHz
    pub const SYS_150MHZ: PllConfig = PllConfig {
        refdiv: 1,
        fbdiv: 125,
        post_div1: 5,
        post_div2: 2,
    };

    /// Configuration for 48 MHz from 12 MHz crystal (for USB)
    /// VCO = 12 MHz / 1 * 100 = 1200 MHz
    /// Output = 1200 MHz / 5 / 5 = 48 MHz
    pub const USB_48MHZ: PllConfig = PllConfig {
        refdiv: 1,
        fbdiv: 100,
        post_div1: 5,
        post_div2: 5,
    };
}

/// Generic PLL driver
pub struct Pll {
    base: u32,
}

impl Pll {
    /// System PLL
    pub const SYS: Pll = Pll { base: PLL_SYS_BASE };
    /// USB PLL
    pub const USB: Pll = Pll { base: PLL_USB_BASE };

    #[inline]
    fn cs(&self) -> Reg {
        Reg::new(self.base + CS)
    }

    #[inline]
    fn pwr(&self) -> Reg {
        Reg::new(self.base + PWR)
    }

    #[inline]
    fn fbdiv_int(&self) -> Reg {
        Reg::new(self.base + FBDIV_INT)
    }

    #[inline]
    fn prim(&self) -> Reg {
        Reg::new(self.base + PRIM)
    }

    /// Initialize and configure the PLL
    ///
    /// Assumes the reference clock (XOSC) is already running
    pub fn init(&self, config: &PllConfig) {
        // Reset PLL to known state
        self.pwr().write(PWR_VCOPD | PWR_POSTDIVPD | PWR_DSMPD | PWR_PD);

        // Set reference divider
        self.cs().write(config.refdiv as u32);

        // Set feedback divider
        self.fbdiv_int().write(config.fbdiv as u32);

        // Power on PLL (clear PD bit) and VCO
        self.pwr().clear_bits(PWR_PD | PWR_VCOPD);

        // Wait for PLL to lock
        self.cs().wait_set(CS_LOCK);

        // Set post dividers
        let prim = ((config.post_div1 as u32) << PRIM_POSTDIV1_SHIFT)
            | ((config.post_div2 as u32) << PRIM_POSTDIV2_SHIFT);
        self.prim().write(prim);

        // Power on post dividers
        self.pwr().clear_bits(PWR_POSTDIVPD);
    }

    /// Check if PLL is locked
    #[inline]
    pub fn is_locked(&self) -> bool {
        self.cs().is_set(CS_LOCK)
    }

    /// Power down the PLL
    #[inline]
    pub fn power_down(&self) {
        self.pwr().set_bits(PWR_VCOPD | PWR_POSTDIVPD | PWR_PD);
    }
}
