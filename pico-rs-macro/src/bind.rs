//! Implementation helpers for `#[bind(IRQ)]` attribute

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
    Expr, ItemFn, Token,
    parse_macro_input,
    punctuated::Punctuated,
};

/// Implementation of the `#[bind(IRQ)]` attribute macro.
pub fn bind_impl(args: TokenStream, input: TokenStream) -> TokenStream {
    let f = parse_macro_input!(input as ItemFn);

    // Parse arguments — only the interrupt name is accepted
    let args_parsed = parse_macro_input!(args with Punctuated::<Expr, Token![,]>::parse_terminated);

    if args_parsed.is_empty() {
        return syn::Error::new(Span::call_site(), "Expected #[bind(IRQ_NAME)]")
            .to_compile_error()
            .into();
    }

    if args_parsed.len() > 1 {
        return syn::Error::new_spanned(
            &args_parsed[1],
            "#[bind] takes only the interrupt name — set priority via interrupt::set_priority() before interrupt::enable()",
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

    let user_fn_name = &f.sig.ident;
    let fn_body = &f.block;
    let fn_vis = &f.vis;
    let fn_attrs = &f.attrs;

    let irq_name_str = irq_name.to_string();

    quote! {
        #[unsafe(export_name = #irq_name_str)]
        #(#fn_attrs)*
        #fn_vis fn #user_fn_name() #fn_body
    }
    .into()
}
