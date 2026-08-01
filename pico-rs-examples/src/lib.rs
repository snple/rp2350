//! Shared helpers for the `pico-rs-examples` binaries.
//!
//! Each example is a separate binary crate, so code shared between them
//! (board initialization, LED color helpers, etc.) lives here in the
//! package's lib target.

#![no_std]

pub mod common;
