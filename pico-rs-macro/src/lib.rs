//! Procedural macros for pico-rs
//!
//! This crate provides the following macros:
//!
//! - `#[main]` - Entry point for RP2350 applications
//! - `#[exception]` - Exception handler for Cortex-M exceptions
//! - `#[bind(IRQ)]` - Bind a function to an interrupt
//! - `pio_asm!` - PIO assembly compiler

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use std::collections::HashSet;
use syn::{
    AttrStyle, Attribute, Expr, FnArg, Ident, Item, ItemFn, ItemStatic, ReturnType, Stmt, Token,
    Type, Visibility,
    parse::{self, Parse},
    parse_macro_input,
    punctuated::Punctuated,
    spanned::Spanned,
};

mod pio_asm;

/// Parse optional arguments to `#[main]`.
///
/// Returns `Ok(true)` if `no_image_def` was specified, `Ok(false)` if no args.
fn parse_main_args(args: TokenStream) -> Result<bool, TokenStream> {
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
/// use pico_rs::main;
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
/// use pico_rs::block::ImageDefWithVersion;
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
            static __pico_rs_image_def: pico_rs::block::ImageDef = pico_rs::block::ImageDef::exe();
        )
    };

    quote!(
        #image_def

        #[unsafe(export_name = "main")]
        #(#attrs)*
        pub unsafe fn #ident() -> ! {
            unsafe { pico_rs::startup::init() };

            // User code
            #(#stmts)*
        }
    )
    .into()
}

// ============================================================================
// Exception Macro Implementation
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
/// use pico_rs::exception;
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
    if f.sig.unsafety.is_none() {
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

            f.sig.ident = Ident::new(&format!("__pico_rs_{}", f.sig.ident), Span::call_site());
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

            f.sig.ident = Ident::new(&format!("__pico_rs_{}", f.sig.ident), Span::call_site());
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
                    unsafe extern "C" fn #tramp_ident(frame: &pico_rs::ExceptionFrame) {
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

            f.sig.ident = Ident::new(&format!("__pico_rs_{}", f.sig.ident), Span::call_site());
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

/// Compile PIO assembly code into a `PioProgram`
///
/// Takes PIO assembly source code as a raw string and compiles it at build time
/// into a `pico_rs::pio::PioProgram` containing instruction words and metadata
/// (wrap, origin, side-set configuration).
///
/// # Example
///
/// ```ignore
/// use pico_rs::pio_asm;
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

/// Bind a function to an interrupt handler
///
/// This macro generates a `#[no_mangle]` interrupt handler that calls your
/// function. It handles only the binding — priority must be configured
/// separately via `interrupt::set_priority()` before `interrupt::enable()`.
///
/// ## Usage
///
/// ```ignore
/// use pico_rs::{bind, interrupt, Interrupt};
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

    // 如果用户已手动标注 #[inline] 或 #[inline(always)]，不再重复添加
    let has_inline = fn_attrs.iter().any(|attr| attr.path().is_ident("inline"));

    let inline_attr = if has_inline {
        quote!()
    } else {
        quote!(#[inline])
    };

    quote! {
        #(#fn_attrs)*
        #inline_attr
        #fn_vis fn #user_fn_name() #fn_body

        #[allow(non_snake_case)]
        #[unsafe(no_mangle)]
        fn #irq_name() {
            #user_fn_name();
        }
    }
    .into()
}
