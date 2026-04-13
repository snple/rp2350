//! # pico-rs
//!
//! Hardware Abstraction Layer for RP2350 microcontrollers (Raspberry Pi Pico 2)
//!
//! ## Features
//!
//! - `rt` - Runtime support (pico-rs-rt)
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
//!     let delay = Delay::new(150_000_000); // 150 MHz
//!     loop {
//!         delay.delay_ms(500);
//!     }
//! }
//! ```

#![no_std]

pub mod accessctrl;
pub mod adc;
pub mod binary_info;
pub mod block;
pub mod busctrl;
pub mod clocks;
pub mod delay;
pub mod dma;
pub mod eppb;
pub mod gpio;
pub mod i2c;
pub mod interrupt;
pub mod multicore;
pub mod pin;
pub mod pio;
pub mod powman;
pub mod pwm;
pub mod qspi;
pub mod reset;
pub mod sha256;
pub mod sio;
pub mod spi;
pub mod syscfg;
pub mod sysinfo;
pub mod systick;
pub mod ticks;
pub mod time;
pub mod timer;
pub mod trng;
pub mod uart;
pub mod watchdog;

pub mod kit;

#[cfg(feature = "critical-section-impl")]
mod critical_section_impl;

// Re-export pico-rs-rt for convenience
pub use pico_rs_rt as rt;

// Re-export pico-rs-pac
pub use pico_rs_pac as pac;
pub use pico_rs_pac::__INTERRUPTS;
pub use pico_rs_pac::{Interrupt, Peripherals, Vector};

// Re-export the entry macro
pub use pico_rs_macro::entry;

// Re-export the exception macro
pub use pico_rs_macro::exception;

// Re-export the pio_asm macro
pub use pico_rs_macro::pio_asm;

// Re-export the bind macro for interrupt handlers
pub use pico_rs_macro::bind;

// Re-export ExceptionFrame from pico-rs-rt for use with HardFault handler
pub use pico_rs_rt::ExceptionFrame;

// Re-export paste for use in macros
#[doc(hidden)]
pub use paste;

// Re-export commonly used types
pub use accessctrl::Accessctrl;
pub use adc::Adc;
pub use busctrl::Busctrl;
pub use clocks::Clocks;
pub use dma::{Channel, Dma};
pub use eppb::Eppb;
pub use gpio::{Gpio, Pins};
pub use i2c::{I2c0, I2c1};
pub use multicore::{Fifo, Multicore, Stack};
pub use powman::{AonTimer, Powman, Timespec};
pub use pwm::Pwm;
pub use sha256::Sha256;
pub use spi::{Spi0, Spi1};
pub use syscfg::Syscfg;
pub use sysinfo::Sysinfo;
pub use systick::{Delay, SysTick};
pub use ticks::Ticks;
pub use time::{Duration, Instant};
pub use timer::{Timer0, Timer1};
pub use trng::Trng;
pub use uart::{Uart0, Uart1};
pub use watchdog::Watchdog;

#[cfg(feature = "defmt")]
pub use defmt;
