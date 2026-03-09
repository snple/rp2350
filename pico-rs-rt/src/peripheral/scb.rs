//! System Control Block (SCB)
//!
//! The SCB provides system implementation information and system control.
//!
//! # Features
//!
//! - System reset control
//! - Exception priority configuration
//! - Vector table relocation
//! - FPU access control
//! - Cache control (if available)

use core::marker::PhantomData;

use super::{SCB_BASE, VolatileCell, peripheral};

/// SCB Register Block
#[repr(C)]
pub struct RegisterBlock {
    /// Interrupt Control and State Register
    pub icsr: VolatileCell<u32>,
    /// Vector Table Offset Register
    pub vtor: VolatileCell<u32>,
    /// Application Interrupt and Reset Control Register
    pub aircr: VolatileCell<u32>,
    /// System Control Register
    pub scr: VolatileCell<u32>,
    /// Configuration and Control Register
    pub ccr: VolatileCell<u32>,
    /// System Handler Priority Registers (12 bytes for handlers 4-15)
    pub shpr: [VolatileCell<u8>; 12],
    /// System Handler Control and State Register
    pub shcsr: VolatileCell<u32>,
    /// Configurable Fault Status Register
    pub cfsr: VolatileCell<u32>,
    /// HardFault Status Register
    pub hfsr: VolatileCell<u32>,
    /// Debug Fault Status Register
    pub dfsr: VolatileCell<u32>,
    /// MemManage Fault Address Register
    pub mmfar: VolatileCell<u32>,
    /// BusFault Address Register
    pub bfar: VolatileCell<u32>,
    /// Auxiliary Fault Status Register
    pub afsr: VolatileCell<u32>,
    _reserved: [u32; 18],
    /// Coprocessor Access Control Register
    pub cpacr: VolatileCell<u32>,
}

/// System Control Block peripheral
pub struct SCB {
    _marker: PhantomData<*const ()>,
}

peripheral!(SCB, RegisterBlock, SCB_BASE);

unsafe impl Send for SCB {}

// =============================================================================
// Constants
// =============================================================================

/// AIRCR key for write access (must be written with value 0x05FA)
const AIRCR_VECTKEY: u32 = 0x05FA << 16;

/// AIRCR SYSRESETREQ bit
const AIRCR_SYSRESETREQ: u32 = 1 << 2;

/// SCR SLEEPDEEP bit
const SCR_SLEEPDEEP: u32 = 1 << 2;

/// SCR SLEEPONEXIT bit
const SCR_SLEEPONEXIT: u32 = 1 << 1;

/// CPACR FPU enable mask (CP10 and CP11)
const CPACR_FPU_MASK: u32 = 0b11 << 20 | 0b11 << 22;
const CPACR_FPU_ENABLE: u32 = 0b11 << 20 | 0b11 << 22;

// =============================================================================
// Implementation
// =============================================================================

impl SCB {
    /// System reset request
    ///
    /// This will reset the processor. This function never returns.
    #[inline]
    pub fn system_reset() -> ! {
        crate::asm::dsb();

        unsafe {
            (*Self::PTR)
                .aircr
                .write_bits(AIRCR_VECTKEY | AIRCR_SYSRESETREQ | (*Self::PTR).aircr.read_bits());
        }

        crate::asm::dsb();

        loop {
            crate::asm::nop();
        }
    }

    /// Set the vector table address
    ///
    /// # Safety
    ///
    /// The address must point to a valid vector table in memory.
    /// Typically must be aligned to at least 128 bytes (depends on number of interrupts).
    #[inline]
    pub unsafe fn set_vector_table(&mut self, addr: u32) {
        unsafe { (*Self::PTR).vtor.write_bits(addr) };
    }

    /// Get the current vector table address
    #[inline]
    pub fn vector_table(&self) -> u32 {
        unsafe { (*Self::PTR).vtor.read_bits() }
    }

    /// Enable deep sleep mode
    ///
    /// When enabled, WFI/WFE will put the processor into deep sleep instead
    /// of regular sleep.
    #[inline]
    pub fn enable_deep_sleep(&mut self) {
        unsafe {
            (*Self::PTR).scr.modify_bits(|v| v | SCR_SLEEPDEEP);
        }
    }

    /// Disable deep sleep mode
    #[inline]
    pub fn disable_deep_sleep(&mut self) {
        unsafe {
            (*Self::PTR).scr.modify_bits(|v| v & !SCR_SLEEPDEEP);
        }
    }

    /// Enable sleep-on-exit
    ///
    /// When enabled, the processor enters sleep mode when it returns from
    /// an exception handler to Thread mode.
    #[inline]
    pub fn enable_sleep_on_exit(&mut self) {
        unsafe {
            (*Self::PTR).scr.modify_bits(|v| v | SCR_SLEEPONEXIT);
        }
    }

    /// Disable sleep-on-exit
    #[inline]
    pub fn disable_sleep_on_exit(&mut self) {
        unsafe {
            (*Self::PTR).scr.modify_bits(|v| v & !SCR_SLEEPONEXIT);
        }
    }

    /// Enable FPU access
    ///
    /// Enables full access to the floating-point unit for both privileged
    /// and unprivileged code.
    #[inline]
    pub fn enable_fpu(&mut self) {
        unsafe {
            (*Self::PTR).cpacr.modify_bits(|v| v | CPACR_FPU_ENABLE);
        }
        // ISB to ensure FPU is enabled before any FP instructions
        crate::asm::dsb();
        crate::asm::isb();
    }

    /// Disable FPU access
    #[inline]
    pub fn disable_fpu(&mut self) {
        unsafe {
            (*Self::PTR).cpacr.modify_bits(|v| v & !CPACR_FPU_MASK);
        }
    }

    /// Check if FPU is enabled
    #[inline]
    pub fn is_fpu_enabled(&self) -> bool {
        unsafe { ((*Self::PTR).cpacr.read_bits() & CPACR_FPU_MASK) == CPACR_FPU_ENABLE }
    }

    /// Set the priority of a system handler
    ///
    /// Valid handlers: 4 (MemManage), 5 (BusFault), 6 (UsageFault),
    /// 11 (SVCall), 14 (PendSV), 15 (SysTick)
    ///
    /// # Panics
    ///
    /// Panics if `handler` is not a valid system handler number.
    #[inline]
    pub fn set_handler_priority(&mut self, handler: u8, priority: u8) {
        assert!((4..=15).contains(&handler));
        unsafe {
            (*Self::PTR).shpr[(handler - 4) as usize].write_bits(priority);
        }
    }

    /// Get the priority of a system handler
    #[inline]
    pub fn handler_priority(&self, handler: u8) -> u8 {
        assert!((4..=15).contains(&handler));
        unsafe { (*Self::PTR).shpr[(handler - 4) as usize].read_bits() }
    }

    /// Set PendSV pending (commonly used for context switching)
    #[inline]
    pub fn set_pendsv() {
        const ICSR_PENDSVSET: u32 = 1 << 28;
        unsafe {
            (*Self::PTR).icsr.write_bits(ICSR_PENDSVSET);
        }
    }

    /// Clear PendSV pending
    #[inline]
    pub fn clear_pendsv() {
        const ICSR_PENDSVCLR: u32 = 1 << 27;
        unsafe {
            (*Self::PTR).icsr.write_bits(ICSR_PENDSVCLR);
        }
    }

    /// Check if PendSV is pending
    #[inline]
    pub fn is_pendsv_pending() -> bool {
        const ICSR_PENDSVSET: u32 = 1 << 28;
        unsafe { (*Self::PTR).icsr.read_bits() & ICSR_PENDSVSET != 0 }
    }

    /// Set SysTick pending
    #[inline]
    pub fn set_pendst() {
        const ICSR_PENDSTSET: u32 = 1 << 26;
        unsafe {
            (*Self::PTR).icsr.write_bits(ICSR_PENDSTSET);
        }
    }

    /// Clear SysTick pending
    #[inline]
    pub fn clear_pendst() {
        const ICSR_PENDSTCLR: u32 = 1 << 25;
        unsafe {
            (*Self::PTR).icsr.write_bits(ICSR_PENDSTCLR);
        }
    }
}

/// FPU access mode
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FpuAccessMode {
    /// FPU access disabled
    Disabled,
    /// FPU access for privileged code only
    Privileged,
    /// FPU access for all code
    Full,
}
