/* RP2350 Linker Script (run-from-RAM variant) for pico-rs-rt
 *
 * This variant places .text and .rodata in RAM (copied from FLASH at startup).
 * Only the vector table, Reset handler, and HardFault trampoline remain in
 * FLASH so the CPU can start executing before the copy completes.
 *
 * Enable with:  pico-rs-rt = { features = ["run-from-ram"] }
 */

/* Include the memory layout (provided by application or BSP) */
INCLUDE memory.x

/* `DefaultHandler_` is the runtime fallback implementation.
 * `DefaultHandler` is the public weak alias used by this script and device.x.
 * A user-defined `DefaultHandler` function overrides this alias.
 */
PROVIDE(DefaultHandler = DefaultHandler_);

/* Include device-specific interrupt handlers (provided by PAC) */
INCLUDE device.x

/* Entry point */
ENTRY(Reset);

/* External symbols that must be provided */
EXTERN(__RESET_VECTOR);
EXTERN(Reset);
EXTERN(__EXCEPTIONS);
EXTERN(DefaultHandler);

/* Default exception handlers (can be overridden) */
PROVIDE(NonMaskableInt = DefaultHandler);
PROVIDE(HardFault = HardFault_);
PROVIDE(MemoryManagement = DefaultHandler);
PROVIDE(BusFault = DefaultHandler);
PROVIDE(UsageFault = DefaultHandler);
PROVIDE(SecureFault = DefaultHandler);
PROVIDE(SVCall = DefaultHandler);
PROVIDE(DebugMonitor = DefaultHandler);
PROVIDE(PendSV = DefaultHandler);
PROVIDE(SysTick = DefaultHandler);

/* Device interrupts (provided by device crate) */
EXTERN(__INTERRUPTS);

/* Pre-init function (optional, for assembly-level early init) */
PROVIDE(__pre_init = DefaultPreInit);

SECTIONS
{
    PROVIDE(_ram_start = ORIGIN(RAM));
    PROVIDE(_ram_end = ORIGIN(RAM) + LENGTH(RAM));
    PROVIDE(_stack_start = _ram_end);

    /* ================================================================
     * FLASH-resident Sections (NOT copied to RAM)
     * ================================================================ */

    /* Vector Table - must be at the start of flash */
    .vector_table ORIGIN(FLASH) :
    {
        __vector_table = .;

        /* Initial Stack Pointer (8-byte aligned) */
        LONG(_stack_start & 0xFFFFFFF8);

        /* Reset Vector */
        KEEP(*(.vector_table.reset_vector));

        /* Exception Vectors (handlers 2-15) */
        __exceptions = .;
        KEEP(*(.vector_table.exceptions));
        __eexceptions = .;

        /* Device-specific Interrupt Vectors */
        KEEP(*(.vector_table.interrupts));
    } > FLASH

    /* Minimal init code that MUST stay in flash:
     *   - Reset handler (copies everything to RAM)
     *   - HardFault trampoline (must work even if RAM copy fails)
     */
    .init :
    {
        __sinit = .;
        *(.text.Reset);
        *(.HardFaultTrampoline);
        *(.HardFault.*);
        . = ALIGN(4);
        __einit = .;
    } > FLASH

    /* ================================================================
     * RAM-loaded Sections (copied from FLASH at startup)
     * ================================================================ */

    /* Code — runs from RAM after copy */
    .text : ALIGN(4)
    {
        __stext = .;
        *(.text .text.*);
        . = ALIGN(4);
        __etext = .;
    } > RAM AT>FLASH
    __sitext = LOADADDR(.text);

    /* Read-only Data */
    .rodata : ALIGN(4)
    {
        __srodata = .;
        *(.rodata .rodata.*);
        . = ALIGN(4);
        __erodata = .;
    } > RAM AT>FLASH
    __sirodata = LOADADDR(.rodata);

    /* ================================================================
     * RAM Functions (.ramfunc) — code loaded into RAM at startup
     * ================================================================
     *
     * Functions marked with #[ramfunc] are placed here.  The linker
     * allocates space for them in RAM (VMA) but stores the initial
     * content in FLASH (LMA).  The startup code copies them to RAM
     * before calling main().
     *
     * Usage:
     *   #[ramfunc]
     *   fn flash_write(...) { ... }
     */
    .ramfunc : ALIGN(4)
    {
        __sramfunc = .;
        *(.ramfunc .ramfunc.*);
        . = ALIGN(4);
        __eramfunc = .;
    } > RAM AT>FLASH
    __siramfunc = LOADADDR(.ramfunc);

    /* Initialized Data - copied from FLASH to RAM at startup */
    .data : ALIGN(4)
    {
        __sdata = .;
        *(.data .data.*);
        . = ALIGN(4);
        __edata = .;
    } > RAM AT>FLASH
    __sidata = LOADADDR(.data);

    /* TrustZone Veneers (must stay in FLASH) */
    .gnu.sgstubs : ALIGN(32)
    {
        __veneer_base = .;
        *(.gnu.sgstubs*)
        . = ALIGN(32);
        __veneer_limit = .;
    } > FLASH

    /* ================================================================
     * Zero-initialised RAM Sections
     * ================================================================ */

    /* Uninitialized Data - zeroed at startup */
    .bss (NOLOAD) : ALIGN(4)
    {
        __sbss = .;
        *(.bss .bss.*);
        *(COMMON);
        . = ALIGN(4);
        __ebss = .;
    } > RAM

    /* Explicitly Uninitialized Data - NOT zeroed at startup */
    .uninit (NOLOAD) : ALIGN(4)
    {
        __suninit = .;
        *(.uninit .uninit.*);
        . = ALIGN(4);
        __euninit = .;
    } > RAM

    /* Dynamic relocations check — must precede heap/stack boundary */
    .got (NOLOAD) :
    {
        KEEP(*(.got .got.*));
    }

    /* Heap starts here */
    . = ALIGN(4);
    PROVIDE(__sheap = .);
    PROVIDE(_stack_end = .);

    /* Discard unused exception info */
    /DISCARD/ :
    {
        *(.ARM.exidx);
        *(.ARM.exidx.*);
        *(.ARM.extab.*);
    }
}

/* ================================================================
 * Alignment Assertions
 * ================================================================ */

ASSERT(ORIGIN(FLASH) % 4 == 0, "
ERROR(pico-rs-rt): FLASH origin must be 4-byte aligned");

ASSERT(ORIGIN(RAM) % 4 == 0, "
ERROR(pico-rs-rt): RAM origin must be 4-byte aligned");

ASSERT(__stext % 4 == 0 && __etext % 4 == 0, "
BUG(pico-rs-rt): .text is not 4-byte aligned");

ASSERT(__sitext % 4 == 0, "
BUG(pico-rs-rt): LMA of .text is not 4-byte aligned");

ASSERT(__srodata % 4 == 0 && __erodata % 4 == 0, "
BUG(pico-rs-rt): .rodata is not 4-byte aligned");

ASSERT(__sirodata % 4 == 0, "
BUG(pico-rs-rt): LMA of .rodata is not 4-byte aligned");

ASSERT(__sramfunc % 4 == 0 && __eramfunc % 4 == 0, "
BUG(pico-rs-rt): .ramfunc is not 4-byte aligned");

ASSERT(__siramfunc % 4 == 0, "
BUG(pico-rs-rt): LMA of .ramfunc is not 4-byte aligned");

ASSERT(__sdata % 4 == 0 && __edata % 4 == 0, "
BUG(pico-rs-rt): .data is not 4-byte aligned");

ASSERT(__sidata % 4 == 0, "
BUG(pico-rs-rt): LMA of .data is not 4-byte aligned");

ASSERT(__sbss % 4 == 0 && __ebss % 4 == 0, "
BUG(pico-rs-rt): .bss is not 4-byte aligned");

ASSERT(__sheap % 4 == 0, "
BUG(pico-rs-rt): heap start is not 4-byte aligned");

ASSERT(_stack_start % 8 == 0, "
ERROR(pico-rs-rt): stack start must be 8-byte aligned");

ASSERT(_stack_end % 4 == 0, "
ERROR(pico-rs-rt): stack end must be 4-byte aligned");

ASSERT(_stack_start >= _stack_end, "
ERROR(pico-rs-rt): stack end must be below stack start");

/* Vector table checks */
ASSERT(__exceptions == ADDR(.vector_table) + 0x8, "
BUG(pico-rs-rt): reset vector is missing");

ASSERT(__eexceptions == ADDR(.vector_table) + 0x40, "
BUG(pico-rs-rt): exception vectors are missing");

/* Dynamic relocation check */
ASSERT(SIZEOF(.got) == 0, "
ERROR(pico-rs-rt): .got section detected - dynamic relocations not supported");
