//! Implementation helpers for `#[exception]` attribute

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use std::collections::HashSet;
use syn::{
    AttrStyle, Attribute, FnArg, Ident, Item, ItemFn, ItemStatic, ReturnType, Safety, Stmt, Type,
    Visibility,
    parse::{self, Parse},
    parse_macro_input,
    spanned::Spanned,
};

// ============================================================================
// Helper Functions
// ============================================================================

/// Extracts `static mut` vars from the beginning of the given statements
fn extract_static_muts(
    stmts: impl IntoIterator<Item = Stmt>,
) -> Result<(Vec<ItemStatic>, Vec<Stmt>), parse::Error> {
    let mut istmts = stmts.into_iter();

    let mut seen = HashSet::new();
    let mut statics = vec![];
    let mut stmts = vec![];
    for stmt in istmts.by_ref() {
        match stmt {
            Stmt::Item(Item::Static(var)) => match var.mutability {
                syn::StaticMutability::Mut(_) => {
                    if seen.contains(&var.ident) {
                        return Err(parse::Error::new(
                            var.ident.span(),
                            format!("the name `{}` is defined multiple times", var.ident),
                        ));
                    }

                    seen.insert(var.ident.clone());
                    statics.push(var);
                }
                _ => stmts.push(Stmt::Item(Item::Static(var))),
            },
            _ => {
                stmts.push(stmt);
                break;
            }
        }
    }

    stmts.extend(istmts);

    Ok((statics, stmts))
}

/// Extract cfg attributes from a list of attributes
fn extract_cfgs(attrs: Vec<Attribute>) -> (Vec<Attribute>, Vec<Attribute>) {
    let mut cfgs = vec![];
    let mut not_cfgs = vec![];

    for attr in attrs {
        if eq(&attr, "cfg") {
            cfgs.push(attr);
        } else {
            not_cfgs.push(attr);
        }
    }

    (cfgs, not_cfgs)
}

/// Returns `true` if `attr.path` matches `name`
fn eq(attr: &Attribute, name: &str) -> bool {
    matches!(attr.style, AttrStyle::Outer) && attr.path().is_ident(name)
}

/// Caller context for attribute whitelist checking
enum WhiteListCaller {
    Exception,
}

/// Check that only allowed attributes are present
fn check_attr_whitelist(attrs: &[Attribute], caller: WhiteListCaller) -> Result<(), TokenStream> {
    let whitelist = &[
        "doc",
        "link_section",
        "cfg",
        "allow",
        "warn",
        "deny",
        "forbid",
        "cold",
        "naked",
        "expect",
    ];

    'o: for attr in attrs {
        if let Some(attr_name) = attr_name(attr)
            && whitelist.contains(&attr_name.as_str())
        {
            continue 'o;
        }

        let err_str = match caller {
            WhiteListCaller::Exception => "this attribute is not allowed on an exception handler",
        };

        return Err(parse::Error::new(attr.span(), err_str)
            .to_compile_error()
            .into());
    }

    Ok(())
}

/// Get the name of an attribute
fn attr_name(attr: &Attribute) -> Option<String> {
    if !matches!(attr.style, AttrStyle::Outer) {
        return None;
    }

    let name = attr.path().get_ident().map(|x| x.to_string());

    // In Rust 2024 edition, link_section attribute must be marked as unsafe.
    match &name {
        Some(name) if name == "unsafe" => {
            if let Ok(inner_meta) = attr.parse_args::<syn::Meta>() {
                inner_meta.path().get_ident().map(|x| x.to_string())
            } else {
                None
            }
        }
        _ => name,
    }
}

// ============================================================================
// Exception Types
// ============================================================================

/// Exception types supported by Cortex-M33
#[derive(Debug, PartialEq)]
enum Exception {
    DefaultHandler,
    HardFault(HardFaultArgs),
    NonMaskableInt,
    Other,
}

impl std::fmt::Display for Exception {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Exception::DefaultHandler => write!(f, "`DefaultHandler`"),
            Exception::HardFault(_) => write!(f, "`HardFault` handler"),
            Exception::NonMaskableInt => write!(f, "`NonMaskableInt` handler"),
            Exception::Other => write!(f, "Other exception handler"),
        }
    }
}

/// Arguments for HardFault handler
#[derive(Debug, PartialEq)]
struct HardFaultArgs {
    trampoline: bool,
}

impl Default for HardFaultArgs {
    fn default() -> Self {
        Self { trampoline: true }
    }
}

impl Parse for HardFaultArgs {
    fn parse(input: parse::ParseStream) -> syn::Result<Self> {
        let mut items = Vec::new();
        loop {
            if input.is_empty() {
                break;
            }

            let name = input.parse::<Ident>()?;
            input.parse::<syn::Token!(=)>()?;
            let value = input.parse::<syn::Lit>()?;

            items.push((name, value));

            if input.is_empty() {
                break;
            }

            input.parse::<syn::Token!(,)>()?;
        }

        let mut args = Self::default();

        for (name, value) in items {
            match name.to_string().as_str() {
                "trampoline" => match value {
                    syn::Lit::Bool(val) => {
                        args.trampoline = val.value();
                    }
                    _ => {
                        return Err(syn::Error::new_spanned(
                            value,
                            "Not a valid value. `trampoline` takes a boolean literal",
                        ));
                    }
                },
                _ => {
                    return Err(syn::Error::new_spanned(name, "Not a valid argument name"));
                }
            }
        }

        Ok(args)
    }
}

// ============================================================================
// #[exception] Implementation
// ============================================================================

/// Implementation of the `#[exception]` attribute macro.
pub fn exception_impl(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut f = parse_macro_input!(input as ItemFn);

    if let Err(error) = check_attr_whitelist(&f.attrs, WhiteListCaller::Exception) {
        return error;
    }

    let fspan = f.span();
    let ident = f.sig.ident.clone();

    let ident_s = ident.to_string();
    let exn = match &*ident_s {
        "DefaultHandler" => {
            if !args.is_empty() {
                return parse::Error::new(Span::call_site(), "This attribute accepts no arguments")
                    .to_compile_error()
                    .into();
            }
            Exception::DefaultHandler
        }
        "HardFault" => Exception::HardFault(parse_macro_input!(args)),
        "NonMaskableInt" => {
            if !args.is_empty() {
                return parse::Error::new(Span::call_site(), "This attribute accepts no arguments")
                    .to_compile_error()
                    .into();
            }
            Exception::NonMaskableInt
        }
        // Cortex-M33 (ARMv8-M) exceptions
        "MemoryManagement" | "BusFault" | "UsageFault" | "SecureFault" | "SVCall"
        | "DebugMonitor" | "PendSV" | "SysTick" => {
            if !args.is_empty() {
                return parse::Error::new(Span::call_site(), "This attribute accepts no arguments")
                    .to_compile_error()
                    .into();
            }

            Exception::Other
        }
        _ => {
            return parse::Error::new(ident.span(), "This is not a valid exception name")
                .to_compile_error()
                .into();
        }
    };

    // Check if unsafe is required
    if matches!(f.sig.safety, Safety::Default) {
        match exn {
            Exception::DefaultHandler | Exception::HardFault(_) | Exception::NonMaskableInt => {
                let name = format!("{}", exn);
                return parse::Error::new(
                    ident.span(),
                    format_args!("defining a {} is unsafe and requires an `unsafe fn`", name),
                )
                .to_compile_error()
                .into();
            }
            Exception::Other => {}
        }
    }

    let handler = match exn {
        Exception::DefaultHandler => {
            let valid_signature = f.sig.constness.is_none()
                && matches!(f.vis, Visibility::Inherited)
                && f.sig.abi.is_none()
                && f.sig.inputs.len() == 1
                && f.sig.generics.params.is_empty()
                && f.sig.generics.where_clause.is_none()
                && f.sig.variadic.is_none()
                && match f.sig.output {
                    ReturnType::Default => true,
                    ReturnType::Type(_, ref ty) => match **ty {
                        Type::Tuple(ref tuple) => tuple.elems.is_empty(),
                        Type::Never(..) => true,
                        _ => false,
                    },
                };

            if !valid_signature {
                return parse::Error::new(
                    fspan,
                    "`DefaultHandler` must have signature `unsafe fn(i16) [-> !]`",
                )
                .to_compile_error()
                .into();
            }

            f.sig.ident = Ident::new(&format!("__rp2350_{}", f.sig.ident), Span::call_site());
            let tramp_ident = Ident::new(&format!("{}_trampoline", f.sig.ident), Span::call_site());
            let ident = &f.sig.ident;

            let (ref cfgs, ref attrs) = extract_cfgs(f.attrs.clone());

            quote!(
                #(#cfgs)*
                #(#attrs)*
                #[doc(hidden)]
                #[unsafe(export_name = #ident_s)]
                pub unsafe extern "C" fn #tramp_ident() {
                    extern crate core;

                    const SCB_ICSR: *const u32 = 0xE000_ED04 as *const u32;

                    let irqn = unsafe { (core::ptr::read_volatile(SCB_ICSR) & 0x1FF) as i16 - 16 };

                    #ident(irqn)
                }

                #f
            )
        }
        Exception::HardFault(args) => {
            let valid_signature = f.sig.constness.is_none()
                && matches!(f.vis, Visibility::Inherited)
                && f.sig.abi.is_none()
                && if args.trampoline {
                    f.sig.inputs.len() == 1
                        && match &f.sig.inputs[0] {
                            FnArg::Typed(arg) => match arg.ty.as_ref() {
                                Type::Reference(r) => {
                                    r.lifetime.is_none() && r.mutability.is_none()
                                }
                                _ => false,
                            },
                            _ => false,
                        }
                } else {
                    f.sig.inputs.is_empty()
                }
                && f.sig.generics.params.is_empty()
                && f.sig.generics.where_clause.is_none()
                && f.sig.variadic.is_none()
                && match f.sig.output {
                    ReturnType::Default => false,
                    ReturnType::Type(_, ref ty) => matches!(**ty, Type::Never(_)),
                };

            if !valid_signature {
                return parse::Error::new(
                    fspan,
                    if args.trampoline {
                        "`HardFault` handler must have signature `unsafe fn(&ExceptionFrame) -> !`"
                    } else {
                        "`HardFault` handler must have signature `unsafe fn() -> !`"
                    },
                )
                .to_compile_error()
                .into();
            }

            f.sig.ident = Ident::new(&format!("__rp2350_{}", f.sig.ident), Span::call_site());
            let tramp_ident = Ident::new(&format!("{}_trampoline", f.sig.ident), Span::call_site());

            if args.trampoline {
                let ident = &f.sig.ident;

                let (ref cfgs, ref attrs) = extract_cfgs(f.attrs.clone());

                quote!(
                    #(#cfgs)*
                    #(#attrs)*
                    #[doc(hidden)]
                    #[unsafe(export_name = "_HardFault")]
                    #[cfg_attr(target_os = "none", unsafe(link_section = ".HardFault.user"))]
                    unsafe extern "C" fn #tramp_ident(frame: &rp2350::ExceptionFrame) {
                        #ident(frame)
                    }

                    #f

                    // HardFault exceptions are bounced through this trampoline which grabs the stack pointer at
                    // the time of the exception and passes it to the user's HardFault handler in r0.
                    // Depending on the stack mode in EXC_RETURN, fetches stack from either MSP or PSP.
                    core::arch::global_asm!(
                        ".cfi_sections .debug_frame
                        .section .HardFaultTrampoline, \"ax\"
                        .global HardFault
                        .type HardFault,%function
                        .thumb_func
                        .cfi_startproc
                        HardFault:",
                           "mov r0, lr
                            movs r1, #4
                            tst r0, r1
                            bne 0f
                            mrs r0, MSP
                            b _HardFault
                        0:
                            mrs r0, PSP
                            b _HardFault",
                        ".cfi_endproc
                        .size HardFault, . - HardFault",
                    );
                )
            } else {
                quote!(
                    #[doc(hidden)]
                    #[unsafe(export_name = "_HardFault")]
                    unsafe extern "C" fn #tramp_ident() {
                        // This trampoline has no function except making the compiler diagnostics better.
                    }

                    #[unsafe(export_name = "HardFault")]
                    #[cfg_attr(target_os = "none", unsafe(link_section = ".HardFault.user"))]
                    #f
                )
            }
        }
        Exception::NonMaskableInt | Exception::Other => {
            let valid_signature = f.sig.constness.is_none()
                && matches!(f.vis, Visibility::Inherited)
                && f.sig.abi.is_none()
                && f.sig.inputs.is_empty()
                && f.sig.generics.params.is_empty()
                && f.sig.generics.where_clause.is_none()
                && f.sig.variadic.is_none()
                && match f.sig.output {
                    ReturnType::Default => true,
                    ReturnType::Type(_, ref ty) => match **ty {
                        Type::Tuple(ref tuple) => tuple.elems.is_empty(),
                        Type::Never(..) => true,
                        _ => false,
                    },
                };

            if !valid_signature {
                return parse::Error::new(
                    fspan,
                    "`#[exception]` handlers other than `DefaultHandler` and `HardFault` must have \
                     signature `[unsafe] fn() [-> !]`",
                )
                .to_compile_error()
                .into();
            }

            let (statics, stmts) = match extract_static_muts(f.block.stmts) {
                Err(e) => return e.to_compile_error().into(),
                Ok(x) => x,
            };

            f.sig.ident = Ident::new(&format!("__rp2350_{}", f.sig.ident), Span::call_site());
            f.sig.inputs.extend(statics.iter().map(|statik| {
                let ident = &statik.ident;
                let ty = &statik.ty;
                let attrs = &statik.attrs;
                syn::parse::<FnArg>(
                    quote!(#[allow(non_snake_case)] #(#attrs)* #ident: &mut #ty).into(),
                )
                .unwrap()
            }));
            f.block.stmts = stmts;

            let tramp_ident = Ident::new(&format!("{}_trampoline", f.sig.ident), Span::call_site());
            let ident = &f.sig.ident;

            let resource_args = statics
                .iter()
                .map(|statik| {
                    let (ref cfgs, ref attrs) = extract_cfgs(statik.attrs.clone());
                    let ident = &statik.ident;
                    let ty = &statik.ty;
                    let expr = &statik.expr;
                    quote! {
                        #(#cfgs)*
                        {
                            #(#attrs)*
                            static mut #ident: #ty = #expr;
                            unsafe { &mut #ident }
                        }
                    }
                })
                .collect::<Vec<_>>();

            let (ref cfgs, ref attrs) = extract_cfgs(f.attrs.clone());

            quote!(
                #(#cfgs)*
                #(#attrs)*
                #[doc(hidden)]
                #[unsafe(export_name = #ident_s)]
                pub unsafe extern "C" fn #tramp_ident() {
                    #[allow(static_mut_refs)]
                    #ident(
                        #(#resource_args),*
                    )
                }

                #f
            )
        }
    };

    handler.into()
}
