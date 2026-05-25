//! DSP utilities for RP2350 Cortex-M33
//!
//! The RP2350 contains dual Cortex-M33 cores with full ARMv8-M DSP extensions:
//! - SIMD packed 8/16-bit arithmetic
//! - Saturating add/subtract/multiply (QADD, QSUB, SSAT, USAT)
//! - Dual 16×16 MAC (SMLAD, SMUAD)
//! - 32×32→64-bit multiply-accumulate
//!
//! Hardware DSP instructions are enabled automatically when compiling with
//! `target-cpu=cortex-m33` (recommended) or `target-feature=+dsp`.
//! Add to `.cargo/config.toml`:
//! ```toml
//! [target.thumbv8m.main-none-eabihf]
//! rustflags = ["-C", "target-cpu=cortex-m33"]
//! ```
//!
//! # Modules
//! - [`fixed`] — Q-format fixed-point arithmetic (Q15, Q31, Q16.16)
//! - [`intrinsics`] — ARM Cortex-M33 DSP instruction wrappers
//! - [`math`] — `sin`, `cos`, `sqrt`, `atan2` approximations (no std)
//! - [`vec`] — Vectorised slice operations (dot-product, scale, add, norm)
//! - [`filter`] — FIR and IIR (biquad) digital filters
//! - [`window`] — Window functions (Hanning, Hamming, Blackman, rectangular)
//! - [`fft`] — In-place fixed-point FFT / IFFT (radix-2 DIT)
//! - [`pid`] — Discrete-time PID controller

#![no_std]
#![allow(dead_code)]

pub mod fft;
pub mod filter;
pub mod fixed;
pub mod intrinsics;
pub mod math;
pub mod pid;
pub mod vec;
pub mod window;
