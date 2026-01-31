//! Peripheral Access Crate for RP2350
//!
//! This crate provides low-level access to the hardware registers of the RP2350
//! microcontroller (Raspberry Pi Pico 2).
//!
//! Auto-generated from RP2350.svd - DO NOT EDIT MANUALLY

#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::identity_op)]
#![allow(clippy::zero_ptr)]
#![allow(clippy::self_named_constructors)]
#![allow(clippy::derivable_impls)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::broken_intra_doc_links)]
#![allow(rustdoc::private_intra_doc_links)]

// Re-export everything from the generated module
mod generated;
pub use generated::*;
