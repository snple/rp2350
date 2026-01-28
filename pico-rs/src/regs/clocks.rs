//! CLOCKS peripheral registers
//!
//! Manages all the clocks in the RP2350

use super::Reg;
use super::addr::CLOCKS_BASE;

// Clock indices
pub const CLK_GPOUT0: usize = 0;
pub const CLK_GPOUT1: usize = 1;
pub const CLK_GPOUT2: usize = 2;
pub const CLK_GPOUT3: usize = 3;
pub const CLK_REF: usize = 4;
pub const CLK_SYS: usize = 5;
pub const CLK_PERI: usize = 6;
pub const CLK_HSTX: usize = 7;
pub const CLK_USB: usize = 8;
pub const CLK_ADC: usize = 9;

// Each clock has 4 registers at 12-byte intervals
// CTRL, DIV, SELECTED for most clocks
fn clk_ctrl(index: usize) -> u32 {
    CLOCKS_BASE + (index as u32) * 0x0C
}

fn clk_div(index: usize) -> u32 {
    CLOCKS_BASE + (index as u32) * 0x0C + 0x04
}

fn clk_selected(index: usize) -> u32 {
    CLOCKS_BASE + (index as u32) * 0x0C + 0x08
}

// CTRL register bits (vary by clock)
pub const CTRL_ENABLE: u32 = 1 << 11;
pub const CTRL_KILL: u32 = 1 << 10;
pub const CTRL_AUXSRC_SHIFT: u32 = 5;
// AUXSRC is 3 bits (7:5) for most clocks
pub const CTRL_AUXSRC_MASK: u32 = 0x7 << CTRL_AUXSRC_SHIFT;
pub const CTRL_SRC_SHIFT: u32 = 0;
// SRC is 2 bits for CLK_REF, 1 bit for CLK_SYS
pub const CTRL_SRC_MASK: u32 = 0x3;

// CLK_REF sources (SRC field)
pub const CLK_REF_SRC_ROSC: u32 = 0;
pub const CLK_REF_SRC_AUX: u32 = 1;
pub const CLK_REF_SRC_XOSC: u32 = 2;
pub const CLK_REF_SRC_LPOSC: u32 = 3;

// CLK_REF aux sources (AUXSRC field)
pub const CLK_REF_AUXSRC_PLL_USB: u32 = 0 << CTRL_AUXSRC_SHIFT;

// CLK_SYS sources (SRC field)
pub const CLK_SYS_SRC_REF: u32 = 0;
pub const CLK_SYS_SRC_AUX: u32 = 1;

// CLK_SYS aux sources (AUXSRC field)
pub const CLK_SYS_AUXSRC_PLL_SYS: u32 = 0 << CTRL_AUXSRC_SHIFT;
pub const CLK_SYS_AUXSRC_PLL_USB: u32 = 1 << CTRL_AUXSRC_SHIFT;
pub const CLK_SYS_AUXSRC_ROSC: u32 = 2 << CTRL_AUXSRC_SHIFT;
pub const CLK_SYS_AUXSRC_XOSC: u32 = 3 << CTRL_AUXSRC_SHIFT;

// CLK_PERI aux sources
pub const CLK_PERI_AUXSRC_SYS: u32 = 0 << CTRL_AUXSRC_SHIFT;
pub const CLK_PERI_AUXSRC_PLL_SYS: u32 = 1 << CTRL_AUXSRC_SHIFT;
pub const CLK_PERI_AUXSRC_PLL_USB: u32 = 2 << CTRL_AUXSRC_SHIFT;
pub const CLK_PERI_AUXSRC_ROSC: u32 = 3 << CTRL_AUXSRC_SHIFT;
pub const CLK_PERI_AUXSRC_XOSC: u32 = 4 << CTRL_AUXSRC_SHIFT;

// CLK_USB aux sources
pub const CLK_USB_AUXSRC_PLL_USB: u32 = 0 << CTRL_AUXSRC_SHIFT;
pub const CLK_USB_AUXSRC_PLL_SYS: u32 = 1 << CTRL_AUXSRC_SHIFT;
pub const CLK_USB_AUXSRC_ROSC: u32 = 2 << CTRL_AUXSRC_SHIFT;
pub const CLK_USB_AUXSRC_XOSC: u32 = 3 << CTRL_AUXSRC_SHIFT;

// CLK_ADC aux sources
pub const CLK_ADC_AUXSRC_PLL_USB: u32 = 0 << CTRL_AUXSRC_SHIFT;
pub const CLK_ADC_AUXSRC_PLL_SYS: u32 = 1 << CTRL_AUXSRC_SHIFT;
pub const CLK_ADC_AUXSRC_ROSC: u32 = 2 << CTRL_AUXSRC_SHIFT;
pub const CLK_ADC_AUXSRC_XOSC: u32 = 3 << CTRL_AUXSRC_SHIFT;

/// Generic clock configuration
pub struct Clock {
    index: usize,
}

impl Clock {
    pub const REF: Clock = Clock { index: CLK_REF };
    pub const SYS: Clock = Clock { index: CLK_SYS };
    pub const PERI: Clock = Clock { index: CLK_PERI };
    pub const USB: Clock = Clock { index: CLK_USB };
    pub const ADC: Clock = Clock { index: CLK_ADC };

    #[inline]
    fn ctrl(&self) -> Reg {
        Reg::new(clk_ctrl(self.index))
    }

    #[inline]
    fn div(&self) -> Reg {
        Reg::new(clk_div(self.index))
    }

    #[inline]
    fn selected(&self) -> Reg {
        Reg::new(clk_selected(self.index))
    }

    /// Configure the clock (for CLK_REF and CLK_SYS which have glitchless mux)
    ///
    /// `src` - Source selection (SRC field)
    /// `auxsrc` - Auxiliary source (AUXSRC field), only used when src selects AUX
    /// `div` - Divider value in 8.8 fixed point format (0x100 = divide by 1)
    pub fn configure(&self, src: u32, auxsrc: u32, div: u32) {
        // Set divider first if provided
        if div > 0 {
            self.div().write(div);
        }

        if self.index == CLK_SYS {
            // CLK_SYS: SRC is 1 bit (0=CLK_REF, 1=AUX)
            // First ensure we're on CLK_REF (SRC=0) for safe switching
            let current = self.ctrl().read();
            if (current & 0x1) != 0 {
                self.ctrl().clear_bits(0x1);
                // Wait for CLK_REF to be selected (bit 0)
                while (self.selected().read() & 1) == 0 {
                    cortex_m::asm::nop();
                }
            }

            // Configure AUXSRC while on CLK_REF (safe)
            self.ctrl()
                .modify(|v| (v & !CTRL_AUXSRC_MASK) | (auxsrc & CTRL_AUXSRC_MASK));

            // Switch to AUX source if requested
            if src == 1 {
                self.ctrl().set_bits(0x1);
                // Wait for AUX to be selected (bit 1)
                while (self.selected().read() & 2) == 0 {
                    cortex_m::asm::nop();
                }
            }
        } else if self.index == CLK_REF {
            // CLK_REF: SRC is 2 bits (0=ROSC, 1=AUX, 2=XOSC, 3=LPOSC)
            // For non-AUX sources, just set SRC directly
            let ctrl_val = (auxsrc & CTRL_AUXSRC_MASK) | (src & CTRL_SRC_MASK);
            self.ctrl().write(ctrl_val);
            // Wait for new source to be selected
            let expected = 1 << src;
            while (self.selected().read() & expected) == 0 {
                cortex_m::asm::nop();
            }
        } else {
            // GPOUT clocks also have glitchless mux
            let ctrl_val = (auxsrc & CTRL_AUXSRC_MASK) | (src & CTRL_SRC_MASK);
            self.ctrl().write(ctrl_val);
        }
    }

    /// Configure a clock that only has AUXSRC (no SRC mux)
    ///
    /// For CLK_PERI, CLK_USB, CLK_ADC etc.
    pub fn configure_aux_only(&self, auxsrc: u32, div: u32) {
        // Disable clock while changing
        self.ctrl().clear_bits(CTRL_ENABLE);

        // Set divider
        if div > 0 {
            self.div().write(div);
        }

        // Set aux source and enable
        self.ctrl()
            .modify(|v| (v & !CTRL_AUXSRC_MASK) | auxsrc | CTRL_ENABLE);
    }

    /// Disable the clock
    #[inline]
    pub fn disable(&self) {
        self.ctrl().clear_bits(CTRL_ENABLE);
    }

    /// Enable the clock
    #[inline]
    pub fn enable(&self) {
        self.ctrl().set_bits(CTRL_ENABLE);
    }

    /// Set divider (8.8 fixed point)
    #[inline]
    pub fn set_div(&self, div: u32) {
        self.div().write(div);
    }
}

/// Initialize clocks for typical 150 MHz operation from 12 MHz crystal
pub fn init_default() {
    use super::pll::{Pll, PllConfig};
    use super::resets;
    use super::ticks::Tick;
    use super::xosc::Xosc;

    // Start XOSC (12 MHz crystal)
    Xosc::init(47); // ~1ms startup

    // Enable tick generator (12 cycles = 1 µs at 12 MHz reference)
    // This is required before configuring PLLs
    Tick::WATCHDOG.configure(12);

    // Reset PLLs
    resets::Resets::reset(resets::PLL_SYS | resets::PLL_USB);
    resets::Resets::unreset_wait(resets::PLL_SYS | resets::PLL_USB);

    // Configure PLLs
    Pll::SYS.init(&PllConfig::SYS_150MHZ); // 150 MHz
    Pll::USB.init(&PllConfig::USB_48MHZ); // 48 MHz

    // Switch clk_ref to XOSC
    Clock::REF.configure(CLK_REF_SRC_XOSC, 0, 0x100);

    // Switch clk_sys to PLL_SYS via aux
    Clock::SYS.configure(CLK_SYS_SRC_AUX, CLK_SYS_AUXSRC_PLL_SYS, 0x100);

    // Setup peripheral clock from clk_sys
    Clock::PERI.configure_aux_only(CLK_PERI_AUXSRC_SYS, 0x100);

    // Setup USB clock from PLL_USB
    Clock::USB.configure_aux_only(CLK_USB_AUXSRC_PLL_USB, 0x100);

    // Setup ADC clock from PLL_USB (48 MHz)
    Clock::ADC.configure_aux_only(CLK_ADC_AUXSRC_PLL_USB, 0x100);
}
