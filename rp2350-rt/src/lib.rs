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
//! - `boot2` - Call a linked `rp2350-boot2` blob from `Reset` to set the QMI up
//!   for fast XIP; `boot2-w25q80` / `boot2-generic-03h` select the flash part
//!   and imply it
//! - `run-from-ram` - Copy `.text` and `.rodata` into RAM at startup
//!
//! ## Startup sequence
//!
//! `Reset` (in `rt.rs`) runs, in order:
//!
//! 1. `run-from-ram`: copy `.text`, then `.rodata`, into RAM
//! 2. copy `.ramfunc` into RAM, so `#[ramfunc]` code is callable from the
//!    remaining steps as well as from fault handlers
//! 3. `boot2`: run the linked XIP setup blob (copied onto the stack first)
//! 4. call `__pre_init`
//! 5. copy `.data` from flash
//! 6. zero `.bss`
//! 7. set `MSPLIM` and enable the FPU
//! 8. call `main`
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
//! Both scripts also define the optional `.boot2` output section, bounded by the
//! `__boot2_start__` / `__boot2_end__` symbols and asserted to fit in 256 bytes.
//! It holds the XIP setup blob from `rp2350-boot2` and is empty unless the
//! `boot2` feature is enabled.
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
