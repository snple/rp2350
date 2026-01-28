//! # pico-rs
//!
//! Hardware Abstraction Layer for RP2350 microcontrollers (Raspberry Pi Pico 2)
//!
//! ## Features
//!
//! - `rt` - Runtime support (cortex-m-rt)
//! - `critical-section` - Critical section implementation
//! - `atomics` - Portable atomic operations
//! - `defmt` - Defmt logging support
//!
//! ## Example
//!
//! ```ignore
//! #![no_std]
//! #![no_main]
//!
//! use pico_rs::entry;
//!
//! #[entry]
//! fn main() -> ! {
//!     loop {}
//! }
//! ```

#![no_std]

pub mod block;
pub mod interrupt;

// Re-export the entry macro
pub use pico_rs_macro::entry;

// Re-export cortex-m for convenience
pub use cortex_m;
pub use cortex_m_rt;

#[cfg(feature = "critical-section")]
pub use critical_section;

#[cfg(feature = "defmt")]
pub use defmt;
