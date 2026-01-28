//! Procedural macros for pico-rs
//!
//! This crate provides the `#[entry]` macro for RP2350 applications.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

/// Attribute to declare the entry point of the program
///
/// This macro wraps `cortex_m_rt::entry` and adds RP2350-specific initialization:
/// - Unlocks all spinlocks (prevents deadlocks when using debugger)
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
pub fn entry(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let func = parse_macro_input!(item as ItemFn);
    let func_name = &func.sig.ident;
    let func_block = &func.block;
    let func_attrs = &func.attrs;

    let expanded = quote! {
        #(#func_attrs)*
        #[cortex_m_rt::entry]
        fn #func_name() -> ! {
            // RP2350 specific initialization:
            // Clear all spinlocks to prevent deadlocks after reset
            // (spinlocks are not automatically cleared on software reset)
            unsafe {
                const SIO_BASE: u32 = 0xd000_0000;
                const SPINLOCK_BASE: u32 = SIO_BASE + 0x100;
                for i in 0..32u32 {
                    let spinlock_addr = (SPINLOCK_BASE + i * 4) as *mut u32;
                    // Reading clears the spinlock
                    core::ptr::read_volatile(spinlock_addr);
                }
            }

            #func_block
        }
    };

    TokenStream::from(expanded)
}
