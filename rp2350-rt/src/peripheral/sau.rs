//! Security Attribution Unit (SAU)
//!
//! The SAU is an optional component of the ARMv8-M architecture that provides
//! TrustZone support. It defines regions of memory as Secure or Non-Secure.
//!
//! On RP2350, TrustZone can be used to isolate sensitive code and data
//! from potentially untrusted code.
//!
//! # Memory Regions
//!
//! The SAU can define up to 8 memory regions, each with:
//! - Base address (32-byte aligned)
//! - Limit address (32-byte aligned, inclusive)
//! - Security attribute (Secure or Non-Secure Callable)
//!
//! Memory not covered by any SAU region uses the default security
//! attribute (Secure if SAU is enabled).

use core::marker::PhantomData;

use super::{SAU_BASE, VolatileCell, peripheral};

/// SAU Register Block
#[repr(C)]
pub struct RegisterBlock {
    /// SAU Control Register
    pub ctrl: VolatileCell<u32>,
    /// SAU Type Register (read-only)
    pub type_: VolatileCell<u32>,
    /// SAU Region Number Register
    pub rnr: VolatileCell<u32>,
    /// SAU Region Base Address Register
    pub rbar: VolatileCell<u32>,
    /// SAU Region Limit Address Register
    pub rlar: VolatileCell<u32>,
    /// Secure Fault Status Register
    pub sfsr: VolatileCell<u32>,
    /// Secure Fault Address Register
    pub sfar: VolatileCell<u32>,
}

/// SAU Control Register bits
pub mod ctrl {
    /// SAU Enable bit
    pub const ENABLE: u32 = 1 << 0;
    /// All Non-Secure Enable bit
    /// When set and SAU disabled, all memory is Non-Secure
    pub const ALLNS: u32 = 1 << 1;
}

/// SAU Region Limit Address Register bits
pub mod rlar {
    /// Region enable bit
    pub const ENABLE: u32 = 1 << 0;
    /// Non-Secure Callable bit
    /// When set, the region is Non-Secure Callable (NSC)
    pub const NSC: u32 = 1 << 1;
}

/// Security attribute for an SAU region
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityAttribute {
    /// Secure memory region
    Secure,
    /// Non-Secure Callable memory region (for secure gateway veneers)
    NonSecureCallable,
}

/// SAU peripheral
pub struct SAU {
    _marker: PhantomData<*const ()>,
}

peripheral!(SAU, RegisterBlock, SAU_BASE);

impl SAU {
    /// Enable the SAU
    ///
    /// When enabled, memory regions not covered by SAU regions are Secure.
    #[inline]
    pub fn enable(&mut self) {
        unsafe {
            (*Self::PTR).ctrl.modify_bits(|v| v | ctrl::ENABLE);
        }
        crate::asm::dsb();
        crate::asm::isb();
    }

    /// Disable the SAU
    ///
    /// When disabled and ALLNS is clear, all memory is Secure.
    /// When disabled and ALLNS is set, all memory is Non-Secure.
    #[inline]
    pub fn disable(&mut self) {
        unsafe {
            (*Self::PTR).ctrl.modify_bits(|v| v & !ctrl::ENABLE);
        }
        crate::asm::dsb();
        crate::asm::isb();
    }

    /// Check if SAU is enabled
    #[inline]
    pub fn is_enabled(&self) -> bool {
        unsafe { (*Self::PTR).ctrl.read_bits() & ctrl::ENABLE != 0 }
    }

    /// Set the ALLNS (All Non-Secure) bit
    ///
    /// When set and SAU is disabled, all memory is treated as Non-Secure.
    #[inline]
    pub fn set_all_non_secure(&mut self, enable: bool) {
        unsafe {
            if enable {
                (*Self::PTR).ctrl.modify_bits(|v| v | ctrl::ALLNS);
            } else {
                (*Self::PTR).ctrl.modify_bits(|v| v & !ctrl::ALLNS);
            }
        }
    }

    /// Get the number of SAU regions supported
    #[inline]
    pub fn num_regions(&self) -> u8 {
        // SREGION field in SAU_TYPE[7:0]
        (unsafe { (*Self::PTR).type_.read_bits() } & 0xFF) as u8
    }

    /// Configure an SAU region
    ///
    /// # Arguments
    ///
    /// * `region` - Region number (0-7 typically)
    /// * `base` - Base address (must be 32-byte aligned)
    /// * `limit` - Limit address (must be 32-byte aligned, inclusive)
    /// * `attr` - Security attribute
    /// * `enable` - Whether to enable the region
    ///
    /// # Panics
    ///
    /// Panics if `region` is >= number of supported regions.
    #[inline]
    pub fn configure_region(
        &mut self,
        region: u8,
        base: u32,
        limit: u32,
        attr: SecurityAttribute,
        enable: bool,
    ) {
        assert!(region < self.num_regions(), "Invalid SAU region number");
        assert!(base & 0x1F == 0, "Base address must be 32-byte aligned");
        assert!((limit + 1) & 0x1F == 0, "Limit+1 must be 32-byte aligned");

        unsafe {
            // Select region
            (*Self::PTR).rnr.write_bits(region as u32);

            // Set base address (bits [31:5])
            (*Self::PTR).rbar.write_bits(base & !0x1F);

            // Set limit address and attributes
            let mut rlar_val = limit & !0x1F;
            if enable {
                rlar_val |= rlar::ENABLE;
            }
            if attr == SecurityAttribute::NonSecureCallable {
                rlar_val |= rlar::NSC;
            }
            (*Self::PTR).rlar.write_bits(rlar_val);
        }

        crate::asm::dsb();
        crate::asm::isb();
    }

    /// Disable an SAU region
    #[inline]
    pub fn disable_region(&mut self, region: u8) {
        assert!(region < self.num_regions(), "Invalid SAU region number");

        unsafe {
            (*Self::PTR).rnr.write_bits(region as u32);
            (*Self::PTR).rlar.modify_bits(|v| v & !rlar::ENABLE);
        }

        crate::asm::dsb();
        crate::asm::isb();
    }

    /// Read the Secure Fault Status Register
    ///
    /// This register contains information about secure faults.
    #[inline]
    pub fn fault_status(&self) -> SecureFaultStatus {
        SecureFaultStatus(unsafe { (*Self::PTR).sfsr.read_bits() })
    }

    /// Clear secure fault status bits
    ///
    /// Write 1 to clear the corresponding bit.
    #[inline]
    pub fn clear_fault_status(&mut self, bits: u32) {
        unsafe { (*Self::PTR).sfsr.write_bits(bits) };
    }

    /// Read the Secure Fault Address Register
    ///
    /// Contains the address that caused a secure fault (if SFARVALID is set).
    #[inline]
    pub fn fault_address(&self) -> u32 {
        unsafe { (*Self::PTR).sfar.read_bits() }
    }
}

/// Secure Fault Status Register flags
#[derive(Debug, Clone, Copy)]
pub struct SecureFaultStatus(pub u32);

impl SecureFaultStatus {
    /// Invalid Entry Point (secure function called with invalid entry)
    pub fn invep(&self) -> bool {
        self.0 & (1 << 0) != 0
    }

    /// Invalid Integrity Signature (stack frame integrity check failed)
    pub fn invis(&self) -> bool {
        self.0 & (1 << 1) != 0
    }

    /// Invalid Exception Return
    pub fn inver(&self) -> bool {
        self.0 & (1 << 2) != 0
    }

    /// Attribution Unit Violation
    pub fn auviol(&self) -> bool {
        self.0 & (1 << 3) != 0
    }

    /// Invalid Transition (branch from secure to non-secure memory)
    pub fn invtran(&self) -> bool {
        self.0 & (1 << 4) != 0
    }

    /// Lazy State Preservation Error
    pub fn lsperr(&self) -> bool {
        self.0 & (1 << 5) != 0
    }

    /// Secure Fault Address Register Valid
    pub fn sfarvalid(&self) -> bool {
        self.0 & (1 << 6) != 0
    }

    /// Lazy State Error
    pub fn lserr(&self) -> bool {
        self.0 & (1 << 7) != 0
    }
}

// Safety: SAU is a single peripheral that should only be accessed from one place
unsafe impl Send for SAU {}
