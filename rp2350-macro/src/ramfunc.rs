//! Implementation helpers for `#[ramfunc]` attribute

use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, ItemFn, parse_macro_input};

/// Implementation of the `#[ramfunc]` attribute macro.
pub fn ramfunc_impl(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut f = parse_macro_input!(input as ItemFn);

    // In Rust 2024 edition, link_section must use `unsafe(link_section = ...)`
    let section_attr: Attribute = syn::parse_quote!(
        #[unsafe(link_section = ".ramfunc")]
    );

    // Insert at the beginning so user's own attributes stay visible
    f.attrs.insert(0, section_attr);

    quote!(#f).into()
}
