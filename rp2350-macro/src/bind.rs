//! Implementation helpers for `#[bind(IRQ)]` attribute

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{Expr, ItemFn, Token, parse_macro_input, punctuated::Punctuated};

/// Valid interrupt names for `#[bind(IRQ)]`.
///
/// Mirrors the `Interrupt` enum generated from `RP2350.svd` in
/// `rp2350-pac/src/generated/mod.rs`. Keep in sync when the SVD or the PAC
/// generator changes.
const VALID_IRQS: &[&str] = &[
    "TIMER0_IRQ_0",
    "TIMER0_IRQ_1",
    "TIMER0_IRQ_2",
    "TIMER0_IRQ_3",
    "TIMER1_IRQ_0",
    "TIMER1_IRQ_1",
    "TIMER1_IRQ_2",
    "TIMER1_IRQ_3",
    "PWM_IRQ_WRAP_0",
    "PWM_IRQ_WRAP_1",
    "DMA_IRQ_0",
    "DMA_IRQ_1",
    "DMA_IRQ_2",
    "DMA_IRQ_3",
    "USBCTRL_IRQ",
    "PIO0_IRQ_0",
    "PIO0_IRQ_1",
    "PIO1_IRQ_0",
    "PIO1_IRQ_1",
    "PIO2_IRQ_0",
    "PIO2_IRQ_1",
    "IO_IRQ_BANK0",
    "IO_IRQ_BANK0_NS",
    "IO_IRQ_QSPI",
    "IO_IRQ_QSPI_NS",
    "SIO_IRQ_FIFO",
    "SIO_IRQ_BELL",
    "SIO_IRQ_FIFO_NS",
    "SIO_IRQ_BELL_NS",
    "SIO_IRQ_MTIMECMP",
    "CLOCKS_IRQ",
    "SPI0_IRQ",
    "SPI1_IRQ",
    "UART0_IRQ",
    "UART1_IRQ",
    "ADC_IRQ_FIFO",
    "I2C0_IRQ",
    "I2C1_IRQ",
    "OTP_IRQ",
    "TRNG_IRQ",
    "PLL_SYS_IRQ",
    "PLL_USB_IRQ",
    "POWMAN_IRQ_POW",
    "POWMAN_IRQ_TIMER",
    "SPARE_IRQ_0",
    "SPARE_IRQ_1",
    "SPARE_IRQ_2",
    "SPARE_IRQ_3",
    "SPARE_IRQ_4",
    "SPARE_IRQ_5",
];

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

    // The ISR is invoked by hardware with no arguments.  Reject anything else
    // here — the previous implementation silently dropped the signature, which
    // produced confusing "cannot find value" errors inside the body.
    if !f.sig.inputs.is_empty() {
        return syn::Error::new_spanned(
            &f.sig,
            "#[bind] handler must have signature `fn()` (no arguments)",
        )
        .to_compile_error()
        .into();
    }

    let irq_name_str = irq_name.to_string();

    // Validate the interrupt name against the known RP2350 interrupt set.
    // A misspelled name would otherwise silently link to `DefaultHandler`
    // via the `PROVIDE` fallback in the linker script.
    if !VALID_IRQS.contains(&irq_name_str.as_str()) {
        let valid = VALID_IRQS.join(", ");
        return syn::Error::new_spanned(
            &args_parsed[0],
            format!("unknown interrupt `{irq_name_str}`; valid interrupts are: {valid}"),
        )
        .to_compile_error()
        .into();
    }

    // The function itself is the ISR (no trampoline), so `#[ramfunc]` keeps the
    // whole interrupt path in RAM.  The vector table stores handlers as
    // `unsafe extern "C" fn()`, hence the C ABI.
    quote! {
        #[unsafe(export_name = #irq_name_str)]
        #(#fn_attrs)*
        #fn_vis unsafe extern "C" fn #user_fn_name() #fn_body
    }
    .into()
}
