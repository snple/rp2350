//! # pico-rs-rt
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
//! ## Example
//!
//! ```ignore
//! use pico_rs_rt::{asm, interrupt, peripheral};
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
#![allow(clippy::pointers_in_nomem_asm_block)]

pub mod asm;
pub mod interrupt;
pub mod peripheral;
pub mod register;

#[cfg(feature = "rt")]
mod rt;

#[cfg(feature = "rt")]
pub use rt::Vector;

// Re-export commonly used items
pub use interrupt::{CriticalSection, Mutex};
pub use peripheral::Peripherals;
pub use peripheral::nvic::InterruptNumber;
