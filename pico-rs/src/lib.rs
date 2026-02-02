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
//!     let mut delay = Delay::new(150_000_000); // 150 MHz
//!     loop {
//!         delay.delay_ms(500);
//!     }
//! }
//! ```

#![no_std]

pub mod accessctrl;
pub mod adc;
pub mod aon_timer;
pub mod binary_info;
pub mod block;
pub mod busctrl;
pub mod clocks;
pub mod cortex_m33;
pub mod dma;
pub mod eppb;
pub mod gpio;
pub mod i2c;
pub mod interrupt;
pub mod multicore;
pub mod pio;
pub mod pll;
pub mod powman;
pub mod pwm;
pub mod reset;
pub mod rosc;
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
pub mod xip_aux;
pub mod xip_ctrl;
pub mod xosc;

// Critical section implementation for RP2350
#[cfg(feature = "critical-section")]
mod critical_section_impl;

// Task system (interrupt-based, requires critical-section)
// Note: critical-section implementation is provided by cortex-m crate
// when the "critical-section-single-core" feature is enabled
#[cfg(feature = "critical-section")]
pub mod task;

// Re-export pico-rs-pac
pub use pico_rs_pac as pac;
#[cfg(feature = "rt")]
pub use pico_rs_pac::__INTERRUPTS;
pub use pico_rs_pac::{Interrupt, Peripherals, Vector};

// Re-export the entry macro
pub use pico_rs_macro::entry;

// Re-export the pio_asm macro
pub use pico_rs_macro::pio_asm;

// Re-export the bind macro for interrupt handlers
pub use pico_rs_macro::bind;

// Re-export paste for use in macros
#[doc(hidden)]
pub use paste;

// Re-export commonly used types
pub use accessctrl::Accessctrl;
pub use adc::Adc;
pub use aon_timer::AonTimer;
pub use busctrl::Busctrl;
pub use clocks::Clocks;
pub use dma::Dma;
pub use eppb::Eppb;
pub use gpio::GpioBank;
pub use i2c::{I2c0, I2c1};
pub use multicore::{Fifo, Multicore, Stack};
pub use pll::{PllConfig, PllSys, PllUsb};
pub use powman::Powman;
pub use pwm::Pwm;
pub use rosc::Rosc;
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
pub use xip_aux::XipAux;
pub use xip_ctrl::XipCtrl;
pub use xosc::Xosc;

// Re-export pico-rs-rt for convenience
pub use pico_rs_rt;

#[cfg(feature = "defmt")]
pub use defmt;
