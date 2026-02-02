//! Procedural macros for pico-rs
//!
//! This crate provides the following macros:
//!
//! - `#[entry]` - Entry point for RP2350 applications
//! - `#[bind(IRQ, priority)]` - Bind a function to an interrupt
//! - `pio_asm!` - PIO assembly compiler

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
<<<<<<< HEAD
use syn::{Expr, Item, ItemFn, Lit, Stmt, Token, parse, parse_macro_input, punctuated::Punctuated};
=======
use syn::{ItemFn, ReturnType, Type, parse, parse_macro_input};
>>>>>>> c466689 (impl pico-rs-rt)

mod pio_asm;

/// Attribute to declare the entry point of the program
///
/// This macro generates the `main` entry point that the runtime calls after
/// initialization, adding RP2350-specific initialization code.
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
#[proc_macro_attribute]
pub fn entry(args: TokenStream, input: TokenStream) -> TokenStream {
    let f = parse_macro_input!(input as ItemFn);

    if !args.is_empty() {
        return parse::Error::new(Span::call_site(), "This attribute accepts no arguments")
            .to_compile_error()
            .into();
    }

    // Check that the function has no arguments
    if !f.sig.inputs.is_empty() {
        return parse::Error::new(
            Span::call_site(),
            "#[entry] function must have signature fn() -> !",
        )
        .to_compile_error()
        .into();
    }

    // Check that the return type is `!` (never)
    match &f.sig.output {
        ReturnType::Type(_, ty) => {
            if let Type::Never(_) = **ty {
                // OK
            } else {
                return parse::Error::new(
                    Span::call_site(),
                    "#[entry] function must have return type `!`",
                )
                .to_compile_error()
                .into();
            }
        }
<<<<<<< HEAD

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
        cpacr |= CPACR_FULL_ACCESS << (0 * 2); // CP0 - GPIO Co-Pro
        cpacr |= CPACR_FULL_ACCESS << (4 * 2); // CP4 - DCP

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
=======
        ReturnType::Default => {
            return parse::Error::new(
                Span::call_site(),
                "#[entry] function must have return type `!`",
            )
            .to_compile_error()
            .into();
>>>>>>> c466689 (impl pico-rs-rt)
        }
    }

    let attrs = &f.attrs;
    let stmts = &f.block.stmts;
    let ident = &f.sig.ident;

    // Generate the main function that the runtime calls
    quote!(
        #[unsafe(export_name = "main")]
        #(#attrs)*
        pub unsafe fn #ident() -> ! {
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
                unsafe { SPINLOCK0_PTR.wrapping_add(i).write_volatile(1) };
            }

            // 2. Enable co-processors in the Coprocessor Access Control Register (CPACR)
            // This must be done early, before any code attempts to use these features.
            const SCB_CPACR_PTR: *mut u32 = 0xE000_ED88 as *mut u32;
            const CPACR_FULL_ACCESS: u32 = 0b11;

            // Read current CPACR (FPU is already enabled by the runtime)
            let mut cpacr = unsafe { SCB_CPACR_PTR.read_volatile() };

            // Enable co-processors:
            // - CP0 (bits 1:0): GPIO Co-Processor
            // - CP4 (bits 9:8): DCP (Double Co-Processor) for accelerated math
            // Note: CP10/CP11 (FPU) are already enabled by the runtime
            cpacr |= CPACR_FULL_ACCESS << (0 * 2);  // CP0 - GPIO Co-Pro
            cpacr |= CPACR_FULL_ACCESS << (4 * 2);  // CP4 - DCP

            unsafe { SCB_CPACR_PTR.write_volatile(cpacr) };

            // 3. Memory barrier to ensure all initialization is complete
            // Prevents any accelerated code from being reordered before this point
            core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);

            // User code
            #(#stmts)*
        }
    )
    .into()
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

/// Bind a function to an interrupt with automatic initialization
///
/// This macro generates an interrupt handler function that calls your function.
///
/// ## Usage
///
/// ```ignore
/// use pico_rs::bind;
///
/// // Bind to SPARE_IRQ_0 with default priority
/// #[bind(SPARE_IRQ_0)]
/// fn my_task() {
///     // This code runs when the interrupt is triggered
/// }
///
/// // Bind to TIMER0_IRQ_0 with specific priority (lower = higher priority)
/// #[bind(TIMER0_IRQ_0, priority = 64)]
/// fn timer_handler() {
///     // Handle timer interrupt
/// }
/// ```
///
/// ## Generated Code
///
/// The macro generates an interrupt handler function with `#[unsafe(no_mangle)]`
/// that calls your function.
///
/// ## Parameters
///
/// - First parameter: Interrupt name (e.g., `SPARE_IRQ_0`, `TIMER0_IRQ_0`)
/// - `priority = N`: Optional priority (0-255, lower = higher priority)
///
/// ## Note
///
/// Priority should be set during initialization using `interrupt::set_priority()`
/// or `spare::init()`, not in the interrupt handler itself.
#[proc_macro_attribute]
pub fn bind(args: TokenStream, input: TokenStream) -> TokenStream {
    let f = parse_macro_input!(input as ItemFn);

    // Parse arguments
    let args_parsed = parse_macro_input!(args with Punctuated::<Expr, Token![,]>::parse_terminated);

    if args_parsed.is_empty() {
        return syn::Error::new(
            Span::call_site(),
            "Expected #[bind(IRQ_NAME)] or #[bind(IRQ_NAME, priority = N)]",
        )
        .to_compile_error()
        .into();
    }

    // Get interrupt name (first argument)
    let irq_name = match &args_parsed[0] {
        Expr::Path(path) => {
            let seg = path.path.segments.last().unwrap();
            seg.ident.clone()
        }
        _ => {
            return syn::Error::new_spanned(&args_parsed[0], "Expected interrupt name")
                .to_compile_error()
                .into();
        }
    };

    // Parse optional priority = N
    let mut priority: Option<u8> = None;
    for arg in args_parsed.iter().skip(1) {
        if let Expr::Assign(assign) = arg {
            // Check if left side is "priority"
            if let Expr::Path(path) = &*assign.left
                && let Some(seg) = path.path.segments.last()
                && seg.ident == "priority"
            {
                // Get the value
                if let Expr::Lit(lit) = &*assign.right
                    && let Lit::Int(int) = &lit.lit
                {
                    priority = int.base10_parse::<u8>().ok();
                }
            }
        }
    }

    // Get original function name and body
    let user_fn_name = &f.sig.ident;
    let fn_body = &f.block;
    let fn_vis = &f.vis;
    let fn_attrs = &f.attrs;

    // Generate output based on whether priority is specified
    let output = if let Some(prio) = priority {
        // Generate with priority initialization
        let init_flag_name = quote::format_ident!("__BIND_{}_INIT", irq_name);

        quote! {
            // User's original function
            #(#fn_attrs)*
            #fn_vis fn #user_fn_name() #fn_body

            // Static flag for one-time priority initialization
            static #init_flag_name: core::sync::atomic::AtomicBool =
                core::sync::atomic::AtomicBool::new(false);

            /// Interrupt handler generated by #[bind] macro
            #[allow(non_snake_case)]
            #[unsafe(no_mangle)]
            fn #irq_name() {
                // Lazy initialization of priority (only on first trigger)
                if !#init_flag_name.load(core::sync::atomic::Ordering::Relaxed) {
                    #init_flag_name.store(true, core::sync::atomic::Ordering::Relaxed);
                    pico_rs::interrupt::set_priority(
                        pico_rs::pac::Interrupt::#irq_name,
                        #prio
                    );
                }

                // Call user's function
                #user_fn_name();
            }
        }
    } else {
        // Generate without priority initialization
        quote! {
            // User's original function
            #(#fn_attrs)*
            #fn_vis fn #user_fn_name() #fn_body

            /// Interrupt handler generated by #[bind] macro
            #[allow(non_snake_case)]
            #[unsafe(no_mangle)]
            fn #irq_name() {
                // Call user's function
                #user_fn_name();
            }
        }
    };

    output.into()
}
