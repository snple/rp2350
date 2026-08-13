//! Implementation helpers for `#[main]` attribute

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
    Ident, ItemFn, ReturnType, Type,
    parse::{self},
    parse_macro_input,
};

/// Parse optional arguments to `#[main]`.
///
/// Returns `Ok(true)` if `no_image_def` was specified, `Ok(false)` if no args.
pub fn parse_main_args(args: TokenStream) -> Result<bool, TokenStream> {
    if args.is_empty() {
        return Ok(false);
    }

    let ident: Ident = match syn::parse(args) {
        Ok(id) => id,
        Err(e) => return Err(e.to_compile_error().into()),
    };

    match ident.to_string().as_str() {
        "no_image_def" => Ok(true),
        other => {
            let msg = format!("unknown argument `{}`; expected `no_image_def`", other);
            Err(syn::Error::new(Span::call_site(), msg)
                .to_compile_error()
                .into())
        }
    }
}

/// Generate the expanded output for `#[main]`.
pub fn main_impl(args: TokenStream, input: TokenStream) -> TokenStream {
    let f = parse_macro_input!(input as ItemFn);

    // Parse optional argument: no_image_def
    let no_image_def = match parse_main_args(args) {
        Ok(v) => v,
        Err(e) => return e,
    };

    // Check that the function has no arguments
    if !f.sig.inputs.is_empty() {
        return parse::Error::new(
            Span::call_site(),
            "#[main] function must have signature fn() -> !",
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
                    "#[main] function must have return type `!`",
                )
                .to_compile_error()
                .into();
            }
        }
        ReturnType::Default => {
            return parse::Error::new(
                Span::call_site(),
                "#[main] function must have return type `!`",
            )
            .to_compile_error()
            .into();
        }
    }

    let attrs = &f.attrs;
    let stmts = &f.block.stmts;
    let ident = &f.sig.ident;

    // Generate the image definition block (unless opt-out) plus the main entry point
    let image_def = if no_image_def {
        quote!()
    } else {
        quote!(
            #[doc(hidden)]
            #[unsafe(link_section = ".start_block")]
            #[used]
            static __rp2350_image_def: rp2350::block::ImageDef = rp2350::block::ImageDef::exe();
        )
    };

    quote!(
        #image_def

        #[unsafe(export_name = "main")]
        #(#attrs)*
        pub unsafe fn #ident() -> ! {
            unsafe { rp2350::startup::init() };

            // User code
            #(#stmts)*
        }
    )
    .into()
}
