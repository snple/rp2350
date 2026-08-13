//! Procedural macros for rp2350
//!
//! This crate provides the following macros:
//!
//! - `#[main]` - Entry point for RP2350 applications
//! - `#[exception]` - Exception handler for Cortex-M exceptions
//! - `#[bind(IRQ)]` - Bind a function to an interrupt
//! - `#[ramfunc]` - Mark a function to execute from RAM
//! - `pio_asm!` - PIO assembly compiler

use proc_macro::TokenStream;

mod bind;
mod exception;
mod main_impl;
mod pio_asm;
mod ramfunc;

// ---------------------------------------------------------------------------
// #[main]
// ---------------------------------------------------------------------------

/// Attribute to declare the entry point of the program
///
/// This macro generates the `main` entry point that the runtime calls after
/// initialization, adding RP2350-specific initialization code.
///
/// ## Initialization performed:
///
/// 1. **PICOBIN Image Definition**: Automatically generates the boot block
///    (`ImageDef::exe()`) in the `.start_block` section. No manual
///    `IMAGE_DEF` static is needed in user code.
///
/// 2. **Spinlock Clearing**: Releases all 32 hardware spinlocks to prevent deadlocks
///    when using a debugger (spinlocks persist across software reset)
///
/// 3. **Co-Processor Enabling**: Enables the following co-processors in CPACR:
///    - **DCP (Double Co-Processor)**: Provides accelerated double-precision floating-point
///      and other mathematical operations
///    - **GPIO Co-Processor**: Enables GPIO acceleration instructions
///
/// 4. **Memory Barrier**: Ensures initialization completes before any user code runs
///
/// ## Usage
///
/// ```ignore
/// use rp2350::main;
///
/// /// Default — auto-generates ImageDef::exe()
/// #[main]
/// fn main() -> ! {
///     loop {}
/// }
/// ```
///
/// To supply your own image definition (e.g. `ImageDefWithVersion` or RISC-V),
/// opt out of automatic generation:
///
/// ```ignore
/// use rp2350::block::ImageDefWithVersion;
///
/// #[unsafe(link_section = ".start_block")]
/// #[used]
/// pub static IMAGE_DEF: ImageDefWithVersion =
///     ImageDefWithVersion::exe_with_version(1, 0, 0);
///
/// #[main(no_image_def)]
/// fn main() -> ! {
///     loop {}
/// }
/// ```
///
/// ## Note
///
/// The entry function must have the signature `fn() -> !` (never returns).
#[proc_macro_attribute]
pub fn main(args: TokenStream, input: TokenStream) -> TokenStream {
    main_impl::main_impl(args, input)
}

// ---------------------------------------------------------------------------
// #[exception]
// ---------------------------------------------------------------------------

/// Attribute to declare an exception handler
///
/// This macro is used to declare exception handlers for Cortex-M33 exceptions.
/// It supports the following exceptions:
///
/// - `DefaultHandler` - Called when no specific handler is defined
/// - `HardFault` - Hard fault handler (with optional trampoline)
/// - `NonMaskableInt` - Non-maskable interrupt handler
/// - `MemoryManagement` - Memory management fault handler
/// - `BusFault` - Bus fault handler
/// - `UsageFault` - Usage fault handler
/// - `SecureFault` - Secure fault handler (ARMv8-M)
/// - `SVCall` - Supervisor call handler
/// - `DebugMonitor` - Debug monitor handler
/// - `PendSV` - Pendable service call handler
/// - `SysTick` - System tick timer handler
///
/// ## Usage
///
/// ```ignore
/// use rp2350::exception;
///
/// // Define a SysTick handler
/// #[exception]
/// fn SysTick() {
///     // Handle SysTick interrupt
/// }
///
/// // Define a HardFault handler with ExceptionFrame
/// #[exception]
/// unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
///     panic!("HardFault at {:?}", ef);
/// }
///
/// // Define a HardFault handler without trampoline
/// #[exception(trampoline = false)]
/// unsafe fn HardFault() -> ! {
///     loop {}
/// }
///
/// // Define DefaultHandler
/// #[exception]
/// unsafe fn DefaultHandler(irqn: i16) {
///     // Handle unknown interrupt
/// }
/// ```
///
/// ## Local `static mut` Variables
///
/// Exception handlers (except `DefaultHandler` and `HardFault`) support
/// local `static mut` variables that are converted to safe references:
///
/// ```ignore
/// #[exception]
/// fn SysTick() {
///     static mut COUNT: u32 = 0;
///     *COUNT += 1;  // Safe to use as &mut reference
/// }
/// ```
///
/// ## Safety
///
/// - `DefaultHandler`, `HardFault`, and `NonMaskableInt` must be declared as `unsafe fn`
/// - Other exception handlers can be safe or unsafe
#[proc_macro_attribute]
pub fn exception(args: TokenStream, input: TokenStream) -> TokenStream {
    exception::exception_impl(args, input)
}

// ---------------------------------------------------------------------------
// pio_asm!
// ---------------------------------------------------------------------------

/// Compile PIO assembly code into a `PioProgram`
///
/// Takes PIO assembly source code as a raw string and compiles it at build time
/// into a `rp2350::pio::PioProgram` containing instruction words and metadata
/// (wrap, origin, side-set configuration).
///
/// # Example
///
/// ```ignore
/// use rp2350::pio_asm;
///
/// let program = pio_asm!(r#"
///     .program blink
///     set pindirs, 1
///     .wrap_target
///     set pins, 1 [31]
///     set pins, 0 [31]
///     .wrap
/// "#);
/// // program.instructions, program.wrap_target, program.wrap, etc.
/// ```
#[proc_macro]
pub fn pio_asm(input: TokenStream) -> TokenStream {
    pio_asm::pio_asm_impl(input)
}

// ---------------------------------------------------------------------------
// #[ramfunc]
// ---------------------------------------------------------------------------

/// Mark a function to be executed from RAM.
///
/// Functions marked with `#[ramfunc]` are placed in the `.ramfunc` linker
/// section.  The startup code copies them from flash to RAM before `main()`
/// runs, so they execute at full speed even when the flash controller is
/// busy (e.g. during flash erase/program operations or when XIP caching is
/// disabled).
///
/// ## Usage
///
/// ```ignore
/// use rp2350::ramfunc;
///
/// #[ramfunc]
/// fn flash_range_erase(addr: u32, len: u32) {
///     // This code runs from RAM — safe to call while flash is busy
/// }
/// ```
///
/// ## Notes
///
/// - RAM-resident code consumes both flash (for initial storage) and RAM
///   (for execution).  Use sparingly.
/// - `#[ramfunc]` functions should be kept small — the entire `.ramfunc`
///   section is copied at startup.
/// - `#[inline]` and `#[ramfunc]` can be combined; the inlined copy will
///   also reside in RAM if the caller is `#[ramfunc]`-annotated.
#[proc_macro_attribute]
pub fn ramfunc(args: TokenStream, input: TokenStream) -> TokenStream {
    ramfunc::ramfunc_impl(args, input)
}

// ---------------------------------------------------------------------------
// #[bind(IRQ)]
// ---------------------------------------------------------------------------

/// Bind a function to an interrupt handler
///
/// This macro re-exports the annotated function under the interrupt's linker
/// name (e.g. `SPARE_IRQ_0`) via `#[unsafe(export_name)]`.  There is no
/// trampoline — the function **is** the ISR.  Any attribute applied to the
/// function (such as `#[ramfunc]` adding `link_section`) automatically
/// propagates to the interrupt entry point.
///
/// Priority must be configured separately via `interrupt::set_priority()`
/// before `interrupt::enable()`.
///
/// ## Usage
///
/// ```ignore
/// use rp2350::{bind, interrupt, Interrupt};
///
/// #[bind(SPARE_IRQ_0)]
/// fn event_handler() {
///     // Runs when SPARE_IRQ_0 fires
/// }
///
/// // In main — set priority before enable:
/// interrupt::set_priority(Interrupt::SPARE_IRQ_0, 128);
/// unsafe { interrupt::enable(Interrupt::SPARE_IRQ_0); }
/// ```
///
/// Combine with `#[ramfunc]` to run the ISR from RAM:
///
/// ```ignore
/// #[bind(SPARE_IRQ_0)]
/// #[ramfunc]
/// fn flash_safe_isr() {
///     // Entire interrupt path runs from RAM — safe during flash ops
/// }
/// ```
///
/// ## Priority
///
/// Priority cannot be set within the macro because `set_priority()` must be
/// called **before** `interrupt::enable()` — setting it inside the ISR would
/// only affect future invocations, not the first one (which runs at the
/// reset-default priority of 0). Always call `set_priority()` explicitly
/// in your initialisation code.
///
/// ## Parameters
///
/// - First (and only) parameter: Interrupt name (e.g., `SPARE_IRQ_0`, `TIMER0_IRQ_0`)
#[proc_macro_attribute]
pub fn bind(args: TokenStream, input: TokenStream) -> TokenStream {
    bind::bind_impl(args, input)
}
