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
//! use pico_rs::systick::Delay;
//!
//! #[entry]
//! fn main() -> ! {
//!     let mut delay = Delay::new(150_000_000); // 150 MHz
//!     loop {
//!         delay.delay_ms(500);
//!     }
//! }
//! ```

#![no_std]

pub mod block;
pub mod interrupt;
pub mod regs;
pub mod systick;
pub mod time;

// Re-export the entry macro
pub use pico_rs_macro::entry;

// Re-export commonly used types
pub use regs::{Clock, Gpio, Pll, Resets, Sio, Tick, Watchdog, Xosc};
pub use systick::{Delay, SysTick};
pub use time::{Duration, Instant};

// Re-export cortex-m for convenience
pub use cortex_m;
pub use cortex_m_rt;

#[cfg(feature = "critical-section")]
pub use critical_section;

#[cfg(feature = "defmt")]
pub use defmt;
