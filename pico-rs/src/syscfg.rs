//! System Configuration (SYSCFG) HAL for RP2350
//!
//! The SYSCFG block provides control over various chip-level features including:
//! - Processor status monitoring
//! - GPIO input synchronizer bypass
//! - Debug port control
//! - Memory power management
//!
//! ## Example
//!
//! ```ignore
//! use pico_rs::syscfg::Syscfg;
//! use pico_rs_pac::SYSCFG;
//!
//! let syscfg_pac = unsafe { SYSCFG::steal() };
//! let mut syscfg = Syscfg::init(syscfg_pac);
//!
//! // Check processor status
//! if syscfg.is_proc0_halted() {
//!     // Handle processor 0 halted
//! }
//!
//! // Put SRAM bank 5 into low power mode
//! syscfg.set_sram_power_down(5, true);
//! ```

use crate::reset::Resettable;
use pico_rs_pac::syscfg::Mempowerdown;

/// Memory bank identifiers for power control
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MemoryBank {
    /// SRAM bank 0
    Sram0 = 0,
    /// SRAM bank 1
    Sram1 = 1,
    /// SRAM bank 2
    Sram2 = 2,
    /// SRAM bank 3
    Sram3 = 3,
    /// SRAM bank 4
    Sram4 = 4,
    /// SRAM bank 5
    Sram5 = 5,
    /// SRAM bank 6
    Sram6 = 6,
    /// SRAM bank 7
    Sram7 = 7,
    /// SRAM bank 8
    Sram8 = 8,
    /// SRAM bank 9
    Sram9 = 9,
    /// USB memory
    Usb = 10,
    /// ROM
    Rom = 11,
    /// Boot RAM
    BootRam = 12,
}

impl MemoryBank {
    /// Get the bit mask for this memory bank
    #[inline]
    pub const fn mask(self) -> u32 {
        1 << (self as u32)
    }
}

/// System Configuration driver
///
/// Owns the PAC SYSCFG peripheral to enforce proper ownership semantics.
pub struct Syscfg {
    syscfg: pico_rs_pac::SYSCFG,
}

impl Syscfg {
    /// Initialize the SYSCFG driver.
    ///
    /// Takes ownership of the [`pico_rs_pac::SYSCFG`] register block, resets
    /// the peripheral, and brings it out of reset.
    ///
    /// # Arguments
    /// * `syscfg` - The SYSCFG peripheral from PAC
    #[inline]
    pub fn init(syscfg: pico_rs_pac::SYSCFG) -> Self {
        // Reset then unreset to guarantee clean register state
        <pico_rs_pac::SYSCFG as Resettable>::init();
        Self { syscfg }
    }

    /// Get reference to the underlying register block
    ///
    /// # Safety
    /// Direct register access can bypass the HAL's safety guarantees.
    /// The caller must ensure operations do not conflict with the HAL state.
    #[inline]
    pub unsafe fn regs(&self) -> &pico_rs_pac::syscfg::RegisterBlock {
        &self.syscfg
    }

    /// Release the SYSCFG peripheral
    #[inline]
    pub fn release(self) -> pico_rs_pac::SYSCFG {
        self.syscfg
    }

    // ========================================================================
    // Processor Status
    // ========================================================================

    /// Check if processor 0 has halted
    #[inline]
    pub fn is_proc0_halted(&self) -> bool {
        self.syscfg.proc_config.proc0_halted()
    }

    /// Check if processor 1 has halted
    #[inline]
    pub fn is_proc1_halted(&self) -> bool {
        self.syscfg.proc_config.proc1_halted()
    }

    // ========================================================================
    // GPIO Input Synchronizer Bypass
    // ========================================================================

    /// Bypass the input synchronizer for a GPIO pin (0-31)
    ///
    /// # Warning
    /// Bypassing the synchronizer can inject metastabilities into the processor.
    /// Only use this if you need to save 2 cycles of input latency.
    #[inline]
    pub fn set_gpio_sync_bypass(&self, pin: u8, bypass: bool) {
        if pin >= 32 {
            return;
        }
        let mask = 1u32 << pin;
        if bypass {
            self.syscfg.proc_in_sync_bypass.set_bits(mask);
        } else {
            self.syscfg.proc_in_sync_bypass.clr_bits(mask);
        }
    }

    /// Get the sync bypass state for a GPIO pin (0-31)
    #[inline]
    pub fn get_gpio_sync_bypass(&self, pin: u8) -> bool {
        if pin >= 32 {
            return false;
        }
        (self.syscfg.proc_in_sync_bypass.gpio() & (1u32 << pin)) != 0
    }

    /// Set sync bypass for multiple GPIO pins at once (0-31)
    #[inline]
    pub fn set_gpio_sync_bypass_mask(&self, mask: u32, bypass: bool) {
        if bypass {
            self.syscfg.proc_in_sync_bypass.set_bits(mask);
        } else {
            self.syscfg.proc_in_sync_bypass.clr_bits(mask);
        }
    }

    /// Get the sync bypass mask for GPIO pins 0-31
    #[inline]
    pub fn get_gpio_sync_bypass_mask(&self) -> u32 {
        self.syscfg.proc_in_sync_bypass.gpio()
    }

    /// Bypass the input synchronizer for a high GPIO pin (32-47)
    #[inline]
    pub fn set_gpio_hi_sync_bypass(&self, pin: u8, bypass: bool) {
        if !(32..=47).contains(&pin) {
            return;
        }
        let mask = 1u32 << (pin - 32);
        if bypass {
            self.syscfg.proc_in_sync_bypass_hi.set_bits(mask);
        } else {
            self.syscfg.proc_in_sync_bypass_hi.clr_bits(mask);
        }
    }

    /// Bypass the input synchronizer for USB D+ pin
    #[inline]
    pub fn set_usb_dp_sync_bypass(&self, bypass: bool) {
        if bypass {
            self.syscfg.proc_in_sync_bypass_hi.set_usb_dp();
        } else {
            self.syscfg.proc_in_sync_bypass_hi.clr_usb_dp();
        }
    }

    /// Bypass the input synchronizer for USB D- pin
    #[inline]
    pub fn set_usb_dm_sync_bypass(&self, bypass: bool) {
        if bypass {
            self.syscfg.proc_in_sync_bypass_hi.set_usb_dm();
        } else {
            self.syscfg.proc_in_sync_bypass_hi.clr_usb_dm();
        }
    }

    /// Bypass the input synchronizer for QSPI SCK pin
    #[inline]
    pub fn set_qspi_sck_sync_bypass(&self, bypass: bool) {
        if bypass {
            self.syscfg.proc_in_sync_bypass_hi.set_qspi_sck();
        } else {
            self.syscfg.proc_in_sync_bypass_hi.clr_qspi_sck();
        }
    }

    /// Bypass the input synchronizer for QSPI CSn pin
    #[inline]
    pub fn set_qspi_csn_sync_bypass(&self, bypass: bool) {
        if bypass {
            self.syscfg.proc_in_sync_bypass_hi.set_qspi_csn();
        } else {
            self.syscfg.proc_in_sync_bypass_hi.clr_qspi_csn();
        }
    }

    // ========================================================================
    // Debug Port Control
    // ========================================================================

    /// Attach the debug port to SYSCFG controls
    ///
    /// When attached, the SWD pins are controlled by SYSCFG instead of external pins.
    #[inline]
    pub fn debug_attach(&self) {
        self.syscfg.dbgforce.set_attach();
    }

    /// Detach the debug port from SYSCFG controls
    ///
    /// Returns control of SWD pins to external connections.
    #[inline]
    pub fn debug_detach(&self) {
        self.syscfg.dbgforce.clr_attach();
    }

    /// Check if debug port is attached to SYSCFG
    #[inline]
    pub fn is_debug_attached(&self) -> bool {
        self.syscfg.dbgforce.attach()
    }

    /// Set the SWCLK signal (only effective when attached)
    #[inline]
    pub fn set_swclk(&self, high: bool) {
        if high {
            self.syscfg.dbgforce.set_swclk();
        } else {
            self.syscfg.dbgforce.clr_swclk();
        }
    }

    /// Set the SWDI (SWDIO input) signal (only effective when attached)
    #[inline]
    pub fn set_swdi(&self, high: bool) {
        if high {
            self.syscfg.dbgforce.set_swdi();
        } else {
            self.syscfg.dbgforce.clr_swdi();
        }
    }

    /// Read the SWDO (SWDIO output) signal
    #[inline]
    pub fn get_swdo(&self) -> bool {
        self.syscfg.dbgforce.swdo()
    }

    // ========================================================================
    // Memory Power Control
    // ========================================================================

    /// Set power-down state for a specific memory bank
    ///
    /// When powered down, memories retain their contents but cannot be accessed.
    /// Use with caution!
    #[inline]
    pub fn set_memory_power_down(&self, bank: MemoryBank, power_down: bool) {
        if power_down {
            self.syscfg.mempowerdown.set_bits(bank.mask());
        } else {
            self.syscfg.mempowerdown.clr_bits(bank.mask());
        }
    }

    /// Get power-down state for a specific memory bank
    #[inline]
    pub fn is_memory_powered_down(&self, bank: MemoryBank) -> bool {
        (self.syscfg.mempowerdown.read_bits() & bank.mask()) != 0
    }

    /// Set power-down state for SRAM bank by index (0-9)
    #[inline]
    pub fn set_sram_power_down(&self, bank: u8, power_down: bool) {
        if bank > 9 {
            return;
        }
        let mask = 1u32 << bank;
        if power_down {
            self.syscfg.mempowerdown.set_bits(mask);
        } else {
            self.syscfg.mempowerdown.clr_bits(mask);
        }
    }

    /// Set power-down state for multiple SRAM banks at once
    ///
    /// # Arguments
    /// * `mask` - Bitmask of SRAM banks (bits 0-9 for SRAM0-9)
    /// * `power_down` - True to power down, false to power up
    #[inline]
    pub fn set_sram_power_down_mask(&self, mask: u16, power_down: bool) {
        let mask = (mask as u32) & 0x3FF; // Only bits 0-9 are SRAM
        if power_down {
            self.syscfg.mempowerdown.set_bits(mask);
        } else {
            self.syscfg.mempowerdown.clr_bits(mask);
        }
    }

    /// Power down all SRAM banks
    #[inline]
    pub fn power_down_all_sram(&self) {
        self.syscfg.mempowerdown.set_bits(0x3FF);
    }

    /// Power up all SRAM banks
    #[inline]
    pub fn power_up_all_sram(&self) {
        self.syscfg.mempowerdown.clr_bits(0x3FF);
    }

    /// Set power-down state for USB memory
    #[inline]
    pub fn set_usb_memory_power_down(&self, power_down: bool) {
        if power_down {
            self.syscfg.mempowerdown.set_bits(Mempowerdown::USB);
        } else {
            self.syscfg.mempowerdown.clr_bits(Mempowerdown::USB);
        }
    }

    /// Set power-down state for ROM
    #[inline]
    pub fn set_rom_power_down(&self, power_down: bool) {
        if power_down {
            self.syscfg.mempowerdown.set_bits(Mempowerdown::ROM);
        } else {
            self.syscfg.mempowerdown.clr_bits(Mempowerdown::ROM);
        }
    }

    /// Set power-down state for Boot RAM
    #[inline]
    pub fn set_bootram_power_down(&self, power_down: bool) {
        if power_down {
            self.syscfg.mempowerdown.set_bits(Mempowerdown::BOOTRAM);
        } else {
            self.syscfg.mempowerdown.clr_bits(Mempowerdown::BOOTRAM);
        }
    }

    /// Get the raw memory power-down register value
    #[inline]
    pub fn get_memory_power_down_mask(&self) -> u32 {
        self.syscfg.mempowerdown.read_bits()
    }

    // ========================================================================
    // Auxiliary Control
    // ========================================================================

    /// Set the auxiliary control register
    ///
    /// Bit 2: Mask OTP power analogue power supply detection
    #[inline]
    pub fn set_auxctrl(&self, value: u8) {
        self.syscfg.auxctrl.write_auxctrl(value as u32);
    }

    /// Get the auxiliary control register value
    #[inline]
    pub fn get_auxctrl(&self) -> u8 {
        self.syscfg.auxctrl.auxctrl() as u8
    }
}
