//! # rp2350
//!
//! Hardware Abstraction Layer for RP2350 microcontrollers (Raspberry Pi Pico 2)
//!
//! ## Features
//!
//! - `rt` - Runtime support (rp2350-rt)
//! - `atomics` - Portable atomic operations
//! - `defmt` - Defmt logging support
//! - `panic-handler` - Built-in embedded panic handler for bare-metal targets
//!
//! ## Panic And Logging
//!
//! `panic-handler` is enabled by default on embedded targets. Disable it if the
//! final binary provides its own `#[panic_handler]`.
//!
//! - With `defmt` enabled, panic messages are emitted through the active defmt
//!   transport selected by the binary, such as `defmt-rtt` in the examples.
//! - Without `defmt`, the panic handler skips formatted output and terminates the
//!   core immediately.
//! - In both modes, interrupts are disabled first, `SHCSR.USGFAULTENA` is
//!   cleared through the runtime crate's SCB definition, and execution ends in
//!   `udf` so the failure reliably escalates to HardFault.
//! - The examples no longer depend on `panic-probe`; they use this crate's
//!   built-in panic handler directly.
//!
//! ## Example
//!
//! ```ignore
//! #![no_std]
//! #![no_main]
//!
//! use rp2350::main;
//! use rp2350::systick::Delay;
//!
//! #[main]
//! fn main() -> ! {
//!     let delay = Delay::init(cp.SYST, 150_000_000); // 150 MHz
//!     loop {
//!         delay.delay_ms(500);
//!     }
//! }
//! ```

#![no_std]

#[cfg(feature = "panic-handler")]
mod panic_handler;

// ============================================================================
// Steal wrapper — returned by `steal()` methods across HAL drivers
// ============================================================================

/// Zero-cost wrapper returned by `steal()` methods.
///
/// Implements [`Deref`](core::ops::Deref) but **not** `DerefMut`, so the
/// caller can only obtain `&T` — preventing access to `&mut self` methods
/// (e.g. `install_program`, `init`, `clear_instruction_memory`).
///
/// ## Example
///
/// ```ignore
/// let pio0 = unsafe { Pio0::steal() }; // Steal<Pio0>
/// pio0.sm0().rx_fifo_try_read();        // &self — OK via Deref
/// pio0.install_program(&prog);           // &mut self — compile error
/// ```
pub struct Steal<T>(pub(crate) T);

impl<T> core::ops::Deref for Steal<T> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> AsRef<T> for Steal<T> {
    #[inline]
    fn as_ref(&self) -> &T {
        &self.0
    }
}

pub mod accessctrl;
pub mod adc;
pub mod binary_info;
pub mod block;
pub mod busctrl;
pub mod clocks;
pub mod dcp;
pub mod delay;
pub mod dma;
pub mod eppb;
pub mod gpio;
pub mod gpioc;
pub mod i2c;
pub mod interp;
pub mod interrupt;
pub mod multicore;
pub mod otp;
pub mod pads_qspi;
pub mod pin;
pub mod pio;
pub mod powman;
pub mod ppb;
pub mod pwm;
pub mod qspi;
pub mod reset;
pub mod sha256;
pub mod sio;
pub mod spi;
mod spin;
pub mod startup;
pub mod syscfg;
pub mod sysinfo;
pub mod systick;
pub mod ticks;
pub mod time;
pub mod timer;
pub mod trng;
pub mod uart;
pub mod watchdog;
pub mod xip;

pub mod kit;

#[cfg(feature = "critical-section-impl")]
mod critical_section_impl;

// Re-export rp2350-rt for convenience
pub use rp2350_rt as rt;
pub use rp2350_rt::Peripherals as CorePeripherals;

// Re-export rp2350-pac
pub use rp2350_pac as pac;
pub use rp2350_pac::__INTERRUPTS;
pub use rp2350_pac::{Interrupt, Peripherals, Vector};

// Re-export the entry macro
pub use rp2350_macro::main;

// Re-export the exception macro
pub use rp2350_macro::exception;

// Re-export the pio_asm macro
pub use rp2350_macro::pio_asm;

// Re-export the bind macro for interrupt handlers
pub use rp2350_macro::bind;

// Re-export the ramfunc macro
pub use rp2350_macro::ramfunc;

// Re-export ExceptionFrame from rp2350-rt for use with HardFault handler
pub use rp2350_rt::ExceptionFrame;

// Re-export paste for use in macros
#[doc(hidden)]
pub use paste;

// Re-export commonly used types
pub use accessctrl::Accessctrl;
pub use adc::Adc;
pub use busctrl::Busctrl;
pub use clocks::Clocks;
pub use dcp::Dcp;
pub use dma::{Channel, Dma};
pub use eppb::Eppb;
pub use gpio::Pins;
pub use i2c::{I2c0, I2c0Slave, I2c1, I2c1Slave};
pub use multicore::{Fifo, Multicore, Stack};
pub use otp::Otp;
pub use pads_qspi::PadsQspi;
pub use powman::{AonTimer, Powman, Timespec};
pub use pwm::Pwm;
pub use sha256::Sha256;
pub use spi::{Spi0, Spi0Slave, Spi1, Spi1Slave};
pub use syscfg::Syscfg;
pub use sysinfo::Sysinfo;
pub use systick::{Delay, SysTick};
pub use ticks::Ticks;
pub use time::{Duration, Instant};
pub use timer::{Timer0, Timer1};
pub use trng::Trng;
pub use uart::{Uart0, Uart1};
pub use watchdog::Watchdog;
pub use xip::Xip;

#[cfg(feature = "defmt")]
pub use defmt;
