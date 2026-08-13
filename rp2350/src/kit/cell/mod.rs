//! Core cell types for static, single-use storage.
//!
//! These types provide safe alternatives to `static mut` patterns commonly
//! found in embedded `no_std` code where a peripheral or resource must be
//! allocated statically and initialized exactly once at runtime.
//!
//! | Type | Initialized | Taken at runtime |
//! |------|-------------|------------------|
//! | [`StaticCell<T>`] | at runtime | via `init` / `init_with` |
//! | [`ConstStaticCell<T>`] | at compile time (`const`) | via `take` |
//!
//! Both are interrupt-safe: the internal atomic flag guarantees that only one
//! caller ever obtains the `&'static mut T`, even under concurrent access.

mod static_cell;

pub use static_cell::{ConstStaticCell, StaticCell};
