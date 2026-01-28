//! Procedural macros for pico-rs
//!
//! This crate provides the `#[entry]` macro for RP2350 applications.

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{Item, ItemFn, Stmt, parse, parse_macro_input};

/// Attribute to declare the entry point of the program
///
/// This macro wraps `cortex_m_rt::entry` and adds RP2350-specific initialization:
/// - Unlocks all spinlocks (prevents deadlocks when using debugger)
/// - Enables the Double Co-Processor (DCP) for accelerated operations
/// - Enables the GPIO Co-Processor
///
/// # Example
///
/// ```ignore
/// #[entry]
/// fn main() -> ! {
///     loop {}
/// }
/// ```
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
        // Clear all spinlocks to prevent deadlocks after reset
        // (spinlocks are not automatically cleared on software reset)
        const SIO_BASE: u32 = 0xd000_0000;
        const SPINLOCK0_PTR: *mut u32 = (SIO_BASE + 0x100) as *mut u32;
        const SPINLOCK_COUNT: usize = 32;
        for i in 0..SPINLOCK_COUNT {
            SPINLOCK0_PTR.wrapping_add(i).write_volatile(1);
        }

        // Enable the Double-Co-Pro (DCP) and the GPIO Co-Pro in the CPACR register.
        // We have to do this early, before there's a chance we might call
        // any accelerated functions.
        const SCB_CPACR_PTR: *mut u32 = 0xE000_ED88 as *mut u32;
        const SCB_CPACR_FULL_ACCESS: u32 = 0b11;
        // Do a R-M-W, because the FPU enable is here and that's already been enabled
        let mut temp = SCB_CPACR_PTR.read_volatile();
        // DCP Co-Pro is 4, two-bits per entry
        temp |= SCB_CPACR_FULL_ACCESS << (4 * 2);
        // GPIO Co-Pro is 0, two-bits per entry
        temp |= SCB_CPACR_FULL_ACCESS << (0 * 2);
        SCB_CPACR_PTR.write_volatile(temp);
        // Don't allow any DCP code to be moved before this fence.
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
