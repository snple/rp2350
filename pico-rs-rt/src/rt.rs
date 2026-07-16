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

    // Text section (run-from-ram only)
    #[cfg(feature = "run-from-ram")]
    static __sitext: u32; // LMA of .text
    #[cfg(feature = "run-from-ram")]
    static mut __stext: u32; // Start of .text in RAM
    #[cfg(feature = "run-from-ram")]
    static mut __etext: u32; // End of .text in RAM

    // Rodata section (run-from-ram only)
    #[cfg(feature = "run-from-ram")]
    static __sirodata: u32; // LMA of .rodata
    #[cfg(feature = "run-from-ram")]
    static mut __srodata: u32; // Start of .rodata in RAM
    #[cfg(feature = "run-from-ram")]
    static mut __erodata: u32; // End of .rodata in RAM

    // RAM function section
    static __siramfunc: u32; // LMA of .ramfunc
    static mut __sramfunc: u32; // Start of .ramfunc in RAM
    static mut __eramfunc: u32; // End of .ramfunc in RAM

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
    // ── run-from-ram: copy .text FIRST so all subsequent code is callable ──
    // Inline asm avoids calling read_volatile / write_volatile (which live
    // in .text and haven't been copied to RAM yet in debug builds).
    #[cfg(feature = "run-from-ram")]
    unsafe {
        let mut src = &raw const __sitext as u32;
        let mut dst = &raw mut __stext as u32;
        let end = &raw mut __etext as u32;

        core::arch::asm!(
            "1:",
            "cmp {dst}, {end}",
            "bhs 2f",
            "ldr {tmp}, [{src}]",
            "str {tmp}, [{dst}]",
            "adds {src}, #4",
            "adds {dst}, #4",
            "b 1b",
            "2:",
            src = inout(reg) src,
            dst = inout(reg) dst,
            end = in(reg) end,
            tmp = out(reg) _,
            options(nomem, nostack, preserves_flags),
        );
        // 'src' and 'dst' are consumed by the asm loop; silence unused-assign
        _ = src;
        _ = dst;

        // Self-check: verify the first word was copied.
        let first_src: u32;
        let first_dst: u32;
        core::arch::asm!(
            "ldr {s}, [{src}]",
            "ldr {d}, [{dst}]",
            src = in(reg) &raw const __sitext,
            dst = in(reg) &raw const __stext,
            s = out(reg) first_src,
            d = out(reg) first_dst,
            options(nomem, nostack, preserves_flags),
        );
        if first_src != first_dst {
            loop {
                core::arch::asm!("bkpt #1", options(nomem, nostack, preserves_flags));
            }
        }
    }

    #[cfg(feature = "run-from-ram")]
    unsafe {
        let mut src = &raw const __sirodata;
        let mut dst = &raw mut __srodata;
        let end = &raw mut __erodata;

        while dst < end {
            core::ptr::write_volatile(dst, core::ptr::read_volatile(src));
            src = src.offset(1);
            dst = dst.offset(1);
        }

        // I-cache coherency: code is now in RAM, make it visible to I-side.
        // DSB+ISB alone only flushes the pipeline; if the Cortex-M33 has an
        // I-cache (configurable), it may still hold stale data.
        core::arch::asm!(
            "dsb sy",
            "isb sy",
            "movw r0, #0xEF50",
            "movt r0, #0xE000",
            "str r0, [r0]",    // ICIALLU — invalidate entire I-cache
            "dsb sy",
            "isb sy",
            out("r0") _,
            options(nomem, nostack, preserves_flags),
        );
    }

    // Call pre-init (safe now — .text is in RAM when run-from-ram is active)
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

    // Copy .ramfunc section (code to run from RAM)
    unsafe {
        let mut src = &raw const __siramfunc;
        let mut dst = &raw mut __sramfunc;
        let end = &raw mut __eramfunc;

        while dst < end {
            core::ptr::write_volatile(dst, core::ptr::read_volatile(src));
            src = src.offset(1);
            dst = dst.offset(1);
        }

        // Instruction cache coherency: ensure the newly-written code is visible
        // to the instruction fetch unit before any #[ramfunc] is called.
        core::arch::asm!("dsb sy", "isb sy", options(nomem, nostack, preserves_flags));
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

    // ── Platform init: FPU + unaligned access ──────────────────────────
    unsafe {
        // CPACR (Coprocessor Access Control Register) — enable FPU.
        // CP10 = bits [21:20], CP11 = bits [23:22]; 0b11 = full access.
        const CPACR: *mut u32 = 0xE000_ED88 as *mut u32;
        let cpacr = core::ptr::read_volatile(CPACR);
        core::ptr::write_volatile(CPACR, cpacr | (0b11 << 20) | (0b11 << 22));

        // CCR (Configuration & Control Register) at 0xE000_ED14.
        // Bit 3 (UNALIGN_TRP): clear to allow unaligned LDR/STR on Normal memory.
        const CCR: *mut u32 = 0xE000_ED14 as *mut u32;
        let ccr = core::ptr::read_volatile(CCR);
        core::ptr::write_volatile(CCR, ccr & !(1 << 3));

        core::arch::asm!("dsb sy", "isb sy", options(nomem, nostack, preserves_flags));
    }

    // Call user's main function.
    //
    // In run-from-ram mode, Reset is in FLASH (0x10xxxxxx) while main is
    // linked to RAM (0x20xxxxxx).  A direct BL would exceed the ±16 MB
    // range and require a linker-generated veneer.  We use BLX <reg>
    // instead, which has unlimited range and needs no veneer.
    #[cfg(feature = "run-from-ram")]
    unsafe {
        let main_addr = main as *const () as u32;
        core::arch::asm!(
            "blx {main}",
            main = in(reg) main_addr,
            options(noreturn, nomem, nostack)
        );
    }

    #[cfg(not(feature = "run-from-ram"))]
    unsafe {
        main()
    }
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
