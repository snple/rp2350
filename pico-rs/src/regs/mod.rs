//! RP2350 Register definitions and hardware abstraction
//!
//! This module provides a simple, direct way to access hardware registers.
//! Unlike svd2rust-based PAC crates, we prioritize simplicity and ease of use.
//!
//! ## Design Philosophy
//!
//! - **Direct register access**: Use simple pointer-based access with clear addresses
//! - **Atomic RMW aliases**: Utilize RP2350's hardware SET/CLR/XOR registers
//! - **Type safety where it helps**: Provide safe wrappers without excessive ceremony
//! - **No unnecessary abstractions**: Keep the code close to the hardware
//!
//! ## Usage
//!
//! ```ignore
//! use pico_rs::regs::{RESETS, GPIO};
//!
//! // Reset a peripheral
//! RESETS.reset.set_bits(RESETS::IO_BANK0);
//! while RESETS.reset_done.read() & RESETS::IO_BANK0 == 0 {}
//!
//! // Configure GPIO
//! GPIO.oe_set.write(1 << 25);  // Enable output on GPIO25
//! GPIO.out_set.write(1 << 25); // Set GPIO25 high
//! ```

pub mod addr;
pub mod reg;
pub mod clocks;
pub mod gpio;
pub mod io_bank;
pub mod pads;
pub mod pll;
pub mod resets;
pub mod sio;
pub mod ticks;
pub mod watchdog;
pub mod xosc;

// Re-export commonly used types
pub use reg::{Reg, RegRO, RegWO};
pub use gpio::Gpio;
pub use resets::Resets;
pub use watchdog::Watchdog;
pub use clocks::Clock;
pub use sio::{Sio, Spinlock};
pub use pll::Pll;
pub use xosc::Xosc;
pub use ticks::Tick;
