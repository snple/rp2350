//! Runtime startup code for Cortex-M33
//!
//! This module provides the reset handler and exception vectors.
//! The actual startup sequence is implemented in assembly for reliability.

/// Registers stacked (pushed into the stack) during an exception.
///
/// This structure is used by the `HardFault` exception handler when using
/// the trampoline mode. The hardware automatically pushes these registers
/// onto the stack when an exception occurs.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct ExceptionFrame {
    /// R0 register
    pub r0: u32,
    /// R1 register
    pub r1: u32,
    /// R2 register
    pub r2: u32,
    /// R3 register
    pub r3: u32,
    /// R12 register
    pub r12: u32,
    /// Link Register (R14) - contains the return address
    pub lr: u32,
    /// Program Counter (R15) - the address of the instruction that caused the exception
    pub pc: u32,
    /// Program Status Register - contains flags and execution state
    pub xpsr: u32,
}

impl ExceptionFrame {
    /// Returns the value of the program counter when the exception occurred.
    #[inline]
    pub fn pc(&self) -> u32 {
        self.pc
    }

    /// Returns the value of the link register when the exception occurred.
    #[inline]
    pub fn lr(&self) -> u32 {
        self.lr
    }

    /// Returns the value of the program status register when the exception occurred.
    #[inline]
    pub fn xpsr(&self) -> u32 {
        self.xpsr
    }
}

/// Default pre-init function (does nothing)
#[doc(hidden)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DefaultPreInit() {}

/// Default exception handler (infinite loop)
#[doc(hidden)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DefaultHandler_() {
    loop {
        crate::asm::wfi();
    }
}

/// Default HardFault handler
#[doc(hidden)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HardFault_() {
    loop {
        crate::asm::wfi();
    }
}

// External symbols from linker script
unsafe extern "C" {
    // Data section
    static __sidata: u32; // LMA of .data
    static mut __sdata: u32; // Start of .data in RAM
    static mut __edata: u32; // End of .data in RAM

    // BSS section
    static mut __sbss: u32; // Start of .bss
    static mut __ebss: u32; // End of .bss

    // Stack
    static _stack_start: u32;
    static _stack_end: u32;

    // User entry point
    fn main() -> !;

    // Pre-init function
    fn __pre_init();
}

/// Reset handler - called on power-on or reset
#[doc(hidden)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Reset() -> ! {
    // Call pre-init (typically empty, but can be overridden)
    unsafe { __pre_init() };

    // Initialize .data section (copy from flash to RAM)
    unsafe {
        let mut src = &raw const __sidata;
        let mut dst = &raw mut __sdata;
        let end = &raw mut __edata;

        while dst < end {
            core::ptr::write_volatile(dst, core::ptr::read_volatile(src));
            src = src.offset(1);
            dst = dst.offset(1);
        }
    }

    // Zero .bss section
    unsafe {
        let mut dst = &raw mut __sbss;
        let end = &raw mut __ebss;

        while dst < end {
            core::ptr::write_volatile(dst, 0);
            dst = dst.offset(1);
        }
    }

    // Set MSPLIM for stack overflow detection (ARMv8-M feature)
    unsafe {
        let stack_end = &raw const _stack_end as u32;
        core::arch::asm!(
            "msr MSPLIM, {0}",
            in(reg) stack_end,
            options(nomem, nostack, preserves_flags)
        );
    }

    // Enable FPU (CP10 and CP11)
    // RP2350's Cortex-M33 has FPU, enable it unconditionally
    unsafe {
        const CPACR: *mut u32 = 0xE000_ED88 as *mut u32;
        let cpacr = core::ptr::read_volatile(CPACR);
        // Full access to CP10 and CP11
        core::ptr::write_volatile(CPACR, cpacr | (0b11 << 20) | (0b11 << 22));
        // Memory barrier to ensure FPU is enabled before use
        core::arch::asm!("dsb", "isb", options(nomem, nostack, preserves_flags));
    }

    // Call user's main function
    unsafe { main() }
}

/// Reset vector (entry point address)
#[doc(hidden)]
#[unsafe(link_section = ".vector_table.reset_vector")]
#[unsafe(no_mangle)]
pub static __RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;

/// Exception handler type
pub union Vector {
    /// Exception handler function
    pub handler: unsafe extern "C" fn(),
    /// Reserved entry (set to 0)
    pub reserved: usize,
}

/// Core exception vectors (handlers 2-15)
///
/// These are the standard Cortex-M exceptions excluding reset.
#[doc(hidden)]
#[unsafe(link_section = ".vector_table.exceptions")]
#[unsafe(no_mangle)]
pub static __EXCEPTIONS: [Vector; 14] = [
    // 2: NMI
    Vector {
        handler: NonMaskableInt,
    },
    // 3: HardFault
    Vector { handler: HardFault },
    // 4: MemManage
    Vector {
        handler: MemoryManagement,
    },
    // 5: BusFault
    Vector { handler: BusFault },
    // 6: UsageFault
    Vector {
        handler: UsageFault,
    },
    // 7: SecureFault (ARMv8-M)
    Vector {
        handler: SecureFault,
    },
    // 8-10: Reserved
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    // 11: SVCall
    Vector { handler: SVCall },
    // 12: DebugMonitor
    Vector {
        handler: DebugMonitor,
    },
    // 13: Reserved
    Vector { reserved: 0 },
    // 14: PendSV
    Vector { handler: PendSV },
    // 15: SysTick
    Vector { handler: SysTick },
];

// External exception handler declarations
unsafe extern "C" {
    fn NonMaskableInt();
    fn HardFault();
    fn MemoryManagement();
    fn BusFault();
    fn UsageFault();
    fn SecureFault();
    fn SVCall();
    fn DebugMonitor();
    fn PendSV();
    fn SysTick();
}
