//! # pico-rs
//!
//! Hardware Abstraction Layer for RP2350 microcontrollers (Raspberry Pi Pico 2)
//!
//! ## Features
//!
//! - `rt` - Runtime support (pico-rs-rt)
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

#[cfg(all(feature = "panic-handler", target_os = "none"))]
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

// Blanket trait forwarding for Steal<T> — allows stolen handles to satisfy
// trait bounds that only require `&self` methods.

impl<T: timer::MonotonicTimer> timer::MonotonicTimer for Steal<T> {
    #[inline]
    fn get_time_us(&self) -> u64 {
        self.0.get_time_us()
    }
    #[inline]
    fn get_time_us_32(&self) -> u32 {
        self.0.get_time_us_32()
    }
}

impl<T: timer::AlarmTimer> timer::AlarmTimer for Steal<T> {
    #[inline]
    fn set_alarm(&self, alarm: u8, time: u32) {
        self.0.set_alarm(alarm, time)
    }
    #[inline]
    fn set_alarm_safe(&self, alarm: u8, time: u32) {
        self.0.set_alarm_safe(alarm, time)
    }
    #[inline]
    fn is_alarm_armed(&self, alarm: u8) -> bool {
        self.0.is_alarm_armed(alarm)
    }
    #[inline]
    fn disarm_alarm(&self, alarm: u8) {
        self.0.disarm_alarm(alarm)
    }
    #[inline]
    fn enable_alarm_interrupt(&self, alarm: u8) {
        self.0.enable_alarm_interrupt(alarm)
    }
    #[inline]
    fn disable_alarm_interrupt(&self, alarm: u8) {
        self.0.disable_alarm_interrupt(alarm)
    }
    #[inline]
    fn is_alarm_interrupt_enabled(&self, alarm: u8) -> bool {
        self.0.is_alarm_interrupt_enabled(alarm)
    }
    #[inline]
    fn alarm_interrupt_raw(&self, alarm: u8) -> bool {
        self.0.alarm_interrupt_raw(alarm)
    }
    #[inline]
    fn alarm_interrupt_status(&self, alarm: u8) -> bool {
        self.0.alarm_interrupt_status(alarm)
    }
    #[inline]
    fn clear_alarm_interrupt(&self, alarm: u8) {
        self.0.clear_alarm_interrupt(alarm)
    }
    #[inline]
    fn force_alarm_interrupt(&self, alarm: u8) {
        self.0.force_alarm_interrupt(alarm)
    }
    #[inline]
    fn clear_forced_alarm_interrupt(&self, alarm: u8) {
        self.0.clear_forced_alarm_interrupt(alarm)
    }
}

impl<T: delay::DelayUs> delay::DelayUs for Steal<T> {
    #[inline]
    fn delay_us(&self, us: u32) {
        self.0.delay_us(us)
    }
}

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
