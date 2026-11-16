//! Implementation helpers for `#[ramfunc]` attribute

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{ToTokens, quote};
use syn::{Attribute, ItemFn, parse_macro_input};

/// Implementation of the `#[ramfunc]` attribute macro.
pub fn ramfunc_impl(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut f = parse_macro_input!(input as ItemFn);

    if !args.is_empty() {
        return syn::Error::new(Span::call_site(), "`#[ramfunc]` takes no arguments")
            .to_compile_error()
            .into();
    }

    // An `async fn` only returns a future; the body — the code that actually
    // runs — ends up in the generated `poll`, which stays wherever the compiler
    // puts it.  Placing the wrapper would look like it worked.
    if let Some(asyncness) = &f.sig.asyncness {
        return syn::Error::new_spanned(
            asyncness,
            "`#[ramfunc]` cannot be applied to an `async fn`: the body runs in the generated \
             future's `poll`, which is not placed in `.ramfunc`",
        )
        .to_compile_error()
        .into();
    }

    // An inlined copy lives in the caller's section, so `#[inline]` would make
    // the code run from flash without any diagnostic.
    if let Some(attr) = f.attrs.iter().find(|a| inline_hint(a) == Some(false)) {
        return syn::Error::new_spanned(
            attr,
            "`#[ramfunc]` implies `#[inline(never)]`: an inlined copy lives in the caller's \
             section and would run from flash.  Call this function from another `#[ramfunc]` \
             function, or drop `#[ramfunc]` if it does not need to run from RAM",
        )
        .to_compile_error()
        .into();
    }

    if let Some(attr) = f.attrs.iter().find(|a| has_link_section(a)) {
        return syn::Error::new_spanned(
            attr,
            "`#[ramfunc]` already places this function in `.ramfunc`; remove the explicit \
             `link_section`",
        )
        .to_compile_error()
        .into();
    }

    // In Rust 2024 edition, link_section must use `unsafe(link_section = ...)`
    let section_attr: Attribute = syn::parse_quote!(
        #[unsafe(link_section = ".ramfunc")]
    );

    // Insert at the beginning so user's own attributes stay visible
    f.attrs.insert(0, section_attr);

    // Small functions are otherwise inlined into their callers at `opt-level 3`
    // (`release` also enables fat LTO), which would silently leave the body in
    // flash.
    if !f.attrs.iter().any(|a| inline_hint(a) == Some(true)) {
        let no_inline_attr: Attribute = syn::parse_quote!(#[inline(never)]);
        f.attrs.insert(0, no_inline_attr);
    }

    quote!(#f).into()
}

/// Classifies `#[inline]`-family attributes.
///
/// `Some(true)` for `#[inline(never)]`, `Some(false)` for `#[inline]` and
/// `#[inline(always)]`, `None` for anything else.
fn inline_hint(attr: &Attribute) -> Option<bool> {
    if !attr.path().is_ident("inline") {
        return None;
    }
    Some(attr.meta.to_token_stream().to_string().contains("never"))
}

/// Matches `#[link_section = "..."]` and its edition-2024 form,
/// `#[unsafe(link_section = "...")]`.
fn has_link_section(attr: &Attribute) -> bool {
    if attr.path().is_ident("link_section") {
        return true;
    }
    if !attr.path().is_ident("unsafe") {
        return false;
    }
    // `#[unsafe(...)]` keeps the attribute it wraps in the token stream, so
    // `meta` reads as `unsafe(link_section = "...")`.
    attr.meta
        .to_token_stream()
        .to_string()
        .contains("link_section")
}
