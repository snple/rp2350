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
//! use rp2350::syscfg::{MemoryBank, Syscfg};
//! use rp2350_pac::SYSCFG;
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
//! syscfg.set_memory_power_down(MemoryBank::Sram5, true);
//! ```

/// Write a single-bit bypass field based on a `bypass` flag.
macro_rules! set_bypass {
    ($reg:expr, $set:ident, $clr:ident, $bypass:expr) => {
        if $bypass {
            $reg.$set();
        } else {
            $reg.$clr();
        }
    };
}

use crate::Steal;
use crate::reset::Resettable;

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
    syscfg: rp2350_pac::SYSCFG,
}

impl Syscfg {
    /// Initialize the SYSCFG driver.
    ///
    /// Takes ownership of the [`rp2350_pac::SYSCFG`] register block, resets
    /// the peripheral, and brings it out of reset.
    ///
    /// # Arguments
    /// * `syscfg` - The SYSCFG peripheral from PAC
    #[inline]
    pub fn init(syscfg: rp2350_pac::SYSCFG) -> Self {
        // Reset then unreset to guarantee clean register state
        <rp2350_pac::SYSCFG as Resettable>::init();
        Self { syscfg }
    }

    /// Obtain a Syscfg handle without ownership checks.
    ///
    /// # Safety
    ///
    /// The caller must ensure [`Syscfg::init()`] has been called first
    /// and that no data races occur.
    #[inline]
    pub unsafe fn steal() -> Steal<Self> {
        Steal(Self {
            syscfg: unsafe { rp2350_pac::SYSCFG::steal() },
        })
    }

    /// Get reference to the underlying register block
    ///
    /// # Safety
    /// Direct register access can bypass the HAL's safety guarantees.
    /// The caller must ensure operations do not conflict with the HAL state.
    #[inline]
    pub unsafe fn regs(&self) -> &rp2350_pac::syscfg::RegisterBlock {
        &self.syscfg
    }

    /// Release the SYSCFG peripheral
    #[inline]
    pub fn release(self) -> rp2350_pac::SYSCFG {
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

    /// Bypass the input synchronizer for a GPIO pin.
    ///
    /// The synchronizer-bypass bits are scattered across two registers:
    ///
    /// | GPIO              | Register / field                    |
    /// |-------------------|-------------------------------------|
    /// | 0..=31            | `PROC_IN_SYNC_BYPASS.GPIO`         |
    /// | 32..=47           | `PROC_IN_SYNC_BYPASS_HI.GPIO`      |
    /// | 56 (USB D+)       | `PROC_IN_SYNC_BYPASS_HI.USB_DP`    |
    /// | 57 (USB D-)       | `PROC_IN_SYNC_BYPASS_HI.USB_DM`    |
    /// | 58 (QSPI SCK)     | `PROC_IN_SYNC_BYPASS_HI.QSPI_SCK`  |
    /// | 59 (QSPI CSn)     | `PROC_IN_SYNC_BYPASS_HI.QSPI_CSN`  |
    /// | 60..=63 (QSPI SD) | `PROC_IN_SYNC_BYPASS_HI.QSPI_SD`  |
    ///
    /// GPIO 48..=55 do not exist and will panic.
    ///
    /// # Warning
    /// Bypassing the synchronizer can inject metastabilities into the
    /// processor. Only use this to save two cycles of input latency.
    #[inline]
    pub fn set_sync_bypass(&self, pin: u8, bypass: bool) {
        match pin {
            0..=31 => {
                let bit = 1u32 << pin;
                if bypass {
                    self.syscfg.proc_in_sync_bypass.set_bits(bit);
                } else {
                    self.syscfg.proc_in_sync_bypass.clr_bits(bit);
                }
            }
            32..=47 => {
                let bit = 1u32 << (pin - 32);
                if bypass {
                    self.syscfg.proc_in_sync_bypass_hi.set_bits(bit);
                } else {
                    self.syscfg.proc_in_sync_bypass_hi.clr_bits(bit);
                }
            }
            56 => set_bypass!(
                self.syscfg.proc_in_sync_bypass_hi,
                set_usb_dp,
                clr_usb_dp,
                bypass
            ),
            57 => set_bypass!(
                self.syscfg.proc_in_sync_bypass_hi,
                set_usb_dm,
                clr_usb_dm,
                bypass
            ),
            58 => set_bypass!(
                self.syscfg.proc_in_sync_bypass_hi,
                set_qspi_sck,
                clr_qspi_sck,
                bypass
            ),
            59 => set_bypass!(
                self.syscfg.proc_in_sync_bypass_hi,
                set_qspi_csn,
                clr_qspi_csn,
                bypass
            ),
            60..=63 => {
                // QSPI SD0..SD3 occupy bits 28..31.
                let bit = 1u32 << (28 + (pin - 60));
                if bypass {
                    self.syscfg.proc_in_sync_bypass_hi.set_bits(bit);
                } else {
                    self.syscfg.proc_in_sync_bypass_hi.clr_bits(bit);
                }
            }
            _ => panic!("invalid GPIO pin for sync bypass: {pin}"),
        }
    }

    /// Get the sync bypass state for a GPIO pin.
    #[inline]
    pub fn sync_bypass(&self, pin: u8) -> bool {
        match pin {
            0..=31 => (self.syscfg.proc_in_sync_bypass.gpio() & (1u32 << pin)) != 0,
            32..=47 => (self.syscfg.proc_in_sync_bypass_hi.gpio() & (1u32 << (pin - 32))) != 0,
            56 => self.syscfg.proc_in_sync_bypass_hi.usb_dp(),
            57 => self.syscfg.proc_in_sync_bypass_hi.usb_dm(),
            58 => self.syscfg.proc_in_sync_bypass_hi.qspi_sck(),
            59 => self.syscfg.proc_in_sync_bypass_hi.qspi_csn(),
            60..=63 => (self.syscfg.proc_in_sync_bypass_hi.qspi_sd() & (1u32 << (pin - 60))) != 0,
            _ => panic!("invalid GPIO pin for sync bypass: {pin}"),
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
    pub fn swdo(&self) -> bool {
        self.syscfg.dbgforce.swdo()
    }

    // ========================================================================
    // Memory Power Control
    // ========================================================================

    /// Set the power-down state of a single memory bank.
    ///
    /// When powered down, memories retain their contents but cannot be
    /// accessed. Use with caution!
    #[inline]
    pub fn set_memory_power_down(&self, bank: MemoryBank, power_down: bool) {
        if power_down {
            self.syscfg.mempowerdown.set_bits(bank.mask());
        } else {
            self.syscfg.mempowerdown.clr_bits(bank.mask());
        }
    }

    /// Check whether a memory bank is powered down.
    #[inline]
    pub fn is_memory_powered_down(&self, bank: MemoryBank) -> bool {
        (self.syscfg.mempowerdown.read_bits() & bank.mask()) != 0
    }

    /// Set the power-down state of multiple memory banks at once.
    ///
    /// `mask` is a bitmask over [`MemoryBank`] positions
    /// (`1 << bank as u32`).
    #[inline]
    pub fn set_memory_power_down_mask(&self, mask: u32, power_down: bool) {
        if power_down {
            self.syscfg.mempowerdown.set_bits(mask);
        } else {
            self.syscfg.mempowerdown.clr_bits(mask);
        }
    }

    /// Get the raw memory power-down register value.
    #[inline]
    pub fn memory_power_down_mask(&self) -> u32 {
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
    pub fn auxctrl(&self) -> u8 {
        self.syscfg.auxctrl.auxctrl() as u8
    }
}
