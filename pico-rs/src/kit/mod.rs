//! Platform-independent utility library.
//!
//! This module groups general-purpose building blocks that do **not** access
//! hardware registers directly. It is the `no_std` equivalent of standard
//! library utilities for the pico-rs ecosystem.
//!
//! | Sub-module | Purpose |
//! |------------|---------|
//! | [`cell`]   | Static single-use storage ([`StaticCell`](cell::StaticCell), [`ConstStaticCell`](cell::ConstStaticCell)) |
//! | [`task`]   | Interrupt-driven event queues and task primitives |

pub mod cell;
pub mod task;
