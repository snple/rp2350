//! # rp2350-rt
//!
//! Runtime support for RP2350 Cortex-M33 microcontrollers.
//!
//! This crate provides:
//! - Low-level assembly instructions (WFI, WFE, SEV, NOP, etc.)
//! - Core register access (PRIMASK, CONTROL, MSP, PSP, etc.)
//! - Core peripherals (NVIC, SCB, SysTick, etc.)
//! - Interrupt management
//! - Critical section implementation
//! - Runtime startup code (reset handler, vector table)
//!
//! ## Target
//!
//! This crate is specifically designed for the RP2350 which uses Cortex-M33
//! (ARMv8-M Mainline) architecture.
//!
//! ## Features
//!
//! - `rt` - Runtime support (startup code, vector table, linker script)
//!
//! ## Linker scripts and `device.x`
//!
//! This crate ships `link.x` / `link_ram.x`, which `INCLUDE device.x` for the
//! device-specific interrupt handler symbols. `device.x` is **not** provided by
//! this crate: it is generated and exposed by [`rp2350-pac`]'s build script
//! (`cargo:rustc-link-search`).
//!
//! As a result, any binary that links against `rp2350-rt` (and uses its linker
//! script) must also depend on `rp2350-pac` so the linker can find `device.x`.
//! In practice this happens automatically when you use the `rp2350` HAL, which
//! depends on both crates.
//!
//! [`rp2350-pac`]: https://crates.io/crates/rp2350-pac
//!
//! ## Example
//!
//! ```ignore
//! use rp2350_rt::{asm, interrupt, peripheral};
//!
//! // Use assembly instructions
//! asm::nop();
//! asm::wfi();
//!
//! // Manage interrupts
//! interrupt::disable();
//! unsafe { interrupt::enable(); }
//! ```

#![no_std]
#![deny(missing_docs)]
#![allow(asm_sub_register)]
#![allow(binary_asm_labels)]
#![allow(clippy::pointers_in_nomem_asm_block)]

pub mod asm;
pub mod interrupt;
pub mod peripheral;
pub mod register;
mod rt;

pub use rt::ExceptionFrame;
pub use rt::Vector;

// Re-export commonly used items
pub use peripheral::Peripherals;
pub use peripheral::nvic::InterruptNumber;
