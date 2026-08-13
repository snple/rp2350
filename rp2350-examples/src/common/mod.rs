//! Shared modules used across example binaries.
//!
//! - [`board`]: board-level initialization helpers (clocks, pins, delay)
//! - [`led`]: shared color/timing helpers for LED examples
//! - [`stepper`]: const-generic pin bridges for stepper examples

pub mod board;
pub mod led;
pub mod stepper;
