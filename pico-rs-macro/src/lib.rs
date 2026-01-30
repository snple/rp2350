//! Procedural macros for pico-rs
//!
//! This crate provides the `#[entry]` macro for RP2350 applications
//! and the `pio_asm!` macro for PIO assembly.

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{Item, ItemFn, Stmt, parse, parse_macro_input};

mod pio_asm;

/// Attribute to declare the entry point of the program
///
/// This macro wraps `cortex_m_rt::entry` and adds RP2350-specific initialization:
///
/// ## Initialization performed:
///
/// 1. **Spinlock Clearing**: Releases all 32 hardware spinlocks to prevent deadlocks
///    when using a debugger (spinlocks persist across software reset)
///
/// 2. **Co-Processor Enabling**: Enables the following co-processors in CPACR:
///    - **DCP (Double Co-Processor)**: Provides accelerated double-precision floating-point
///      and other mathematical operations
///    - **GPIO Co-Processor**: Enables GPIO acceleration instructions
///
/// 3. **Memory Barrier**: Ensures initialization completes before any user code runs
///
/// ## Usage
///
/// ```ignore
/// use pico_rs::entry;
/// use pico_rs::block::ImageDef;
///
/// // Boot block is required for RP2350
/// #[link_section = ".start_block"]
/// #[used]
/// pub static IMAGE_DEF: ImageDef = ImageDef::secure_exe();
///
/// #[entry]
/// fn main() -> ! {
///     loop {}
/// }
/// ```
///
/// ## Note
///
/// The entry function must have the signature `fn() -> !` (never returns).
/// Static variables declared at the top of main will still be processed
/// correctly by cortex-m-rt.
#[proc_macro_attribute]
pub fn entry(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut f = parse_macro_input!(input as ItemFn);

    if !args.is_empty() {
        return parse::Error::new(Span::call_site(), "This attribute accepts no arguments")
            .to_compile_error()
            .into();
    }

    // RP2350 ARM-specific initialization code
    let init_code: TokenStream = quote!(unsafe {
        // ============================================================
        // RP2350 Hardware Initialization
        // ============================================================

        // 1. Clear all spinlocks to prevent deadlocks after reset
        // Spinlocks are NOT automatically cleared on software reset, only on
        // power-on reset. This can cause deadlocks when using a debugger.
        const SIO_BASE: u32 = 0xd000_0000;
        const SPINLOCK0_PTR: *mut u32 = (SIO_BASE + 0x100) as *mut u32;
        const SPINLOCK_COUNT: usize = 32;
        for i in 0..SPINLOCK_COUNT {
            // Writing any value releases the spinlock
            SPINLOCK0_PTR.wrapping_add(i).write_volatile(1);
        }

        // 2. Enable co-processors in the Coprocessor Access Control Register (CPACR)
        // This must be done early, before any code attempts to use these features.
        const SCB_CPACR_PTR: *mut u32 = 0xE000_ED88 as *mut u32;
        const CPACR_FULL_ACCESS: u32 = 0b11;

        // Read current CPACR (FPU may already be enabled by cortex-m-rt)
        let mut cpacr = SCB_CPACR_PTR.read_volatile();

        // Enable co-processors:
        // - CP0 (bits 1:0): GPIO Co-Processor
        // - CP4 (bits 9:8): DCP (Double Co-Processor) for accelerated math
        // Note: CP10/CP11 (FPU) are handled by cortex-m-rt
        cpacr |= CPACR_FULL_ACCESS << (0 * 2);  // CP0 - GPIO Co-Pro
        cpacr |= CPACR_FULL_ACCESS << (4 * 2);  // CP4 - DCP

        SCB_CPACR_PTR.write_volatile(cpacr);

        // 3. Memory barrier to ensure all initialization is complete
        // Prevents any accelerated code from being reordered before this point
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    })
    .into();
    let init_stmt = parse_macro_input!(init_code as Stmt);

    // Insert init code after any static declarations (statics must stay first
    // so cortex_m_rt::entry still finds them)
    let stmts = insert_after_static(f.block.stmts, init_stmt);
    f.block.stmts = stmts;

    quote!(
        #[cortex_m_rt::entry]
        #f
    )
    .into()
}

/// Insert new statements after initial block of statics
fn insert_after_static(stmts: impl IntoIterator<Item = Stmt>, insert: Stmt) -> Vec<Stmt> {
    let mut istmts = stmts.into_iter();
    let mut result = vec![];

    for stmt in istmts.by_ref() {
        match stmt {
            Stmt::Item(Item::Static(var)) => {
                result.push(Stmt::Item(Item::Static(var)));
            }
            _ => {
                result.push(insert);
                result.push(stmt);
                break;
            }
        }
    }
    result.extend(istmts);

    result
}

/// Compile PIO assembly code into machine code
///
/// This macro takes PIO assembly source code and compiles it into an array
/// of 16-bit instruction words that can be loaded into PIO instruction memory.
///
/// # Example
///
/// ```ignore
/// use pico_rs::pio_asm;
///
/// let program = pio_asm!(
///     "set pindirs, 1",
///     ".wrap_target",
///     "set pins, 1 [31]",
///     "set pins, 0 [31]",
///     ".wrap"
/// );
/// ```
#[proc_macro]
pub fn pio_asm(input: TokenStream) -> TokenStream {
    pio_asm::pio_asm_impl(input)
}
