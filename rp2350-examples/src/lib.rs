//! Shared helpers for the `rp2350-examples` binaries.
//!
//! Each example is a separate binary crate, so code shared between them
//! (board initialization, LED color helpers, etc.) lives here in the
//! package's lib target.

#![no_std]

pub mod common;

/// Logging facade shared by every example binary.
///
/// With the `defmt` feature these are `defmt`'s own macros; without it they
/// expand to nothing, so the examples keep building (and stay silent) when the
/// feature is disabled.
#[cfg(feature = "defmt")]
pub mod log {
    pub use defmt::*;
}

#[cfg(not(feature = "defmt"))]
pub mod log {
    #[macro_export]
    macro_rules! trace {
        ($($arg:tt)*) => {{ () }};
    }
    #[macro_export]
    macro_rules! debug {
        ($($arg:tt)*) => {{ () }};
    }
    #[macro_export]
    macro_rules! info {
        ($($arg:tt)*) => {{ () }};
    }
    #[macro_export]
    macro_rules! warn {
        ($($arg:tt)*) => {{ () }};
    }
    #[macro_export]
    macro_rules! error {
        ($($arg:tt)*) => {{ () }};
    }

    pub use crate::{debug, error, info, trace, warn};
}
