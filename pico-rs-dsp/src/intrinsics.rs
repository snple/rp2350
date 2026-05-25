//! ARM Cortex-M33 DSP instruction wrappers
//!
//! This module exposes only the instructions that LLVM **cannot** auto-generate
//! from plain Rust code, even with `target-feature=+dsp`:
//!
//! | Function | Instruction | Why asm is needed |
//! |---|---|---|
//! | [`smulbb`] / [`smultt`] | `SMULBB` / `SMULTT` | LLVM uses 32-bit `MUL` for `(a as i16) * (b as i16)` |
//! | [`smlad`] / [`smladx`]  | `SMLAD` / `SMLADX`  | Dual-MAC; LLVM won't fuse two products into one insn |
//! | [`smuad`]               | `SMUAD`              | Same reason as `SMLAD` |
//!
//! The following are **not** here because LLVM already emits the right instruction
//! when `+dsp` is active:
//! - `QADD`/`QSUB` ← `i32::saturating_add/sub`
//! - `QADD16`/`QSUB16` ← `i16::saturating_add/sub`
//! - `SADD16`/`SSUB16` ← `i16::wrapping_add/sub`
//! - `SSAT #N` ← `i32::clamp` with compile-time-constant bounds
//!
//! [`pkhbt`] and [`pkhtb`] are pure bit-manipulation with no corresponding
//! instruction advantage; they are kept as named helpers.
//!
//! # Enabling hardware DSP
//! DSP is enabled automatically with `target-cpu=cortex-m33`. Add to
//! `.cargo/config.toml`:
//! ```toml
//! [target.thumbv8m.main-none-eabihf]
//! rustflags = ["-C", "target-cpu=cortex-m33"]
//! ```
//! The asm paths activate when `cfg(target_feature = "dsp")` is set.

// ---------------------------------------------------------------------------
// PKHBT / PKHTB — pack halfwords (pure bit manipulation)
// ---------------------------------------------------------------------------

/// Pack: lower 16 bits of `a` and upper 16 bits of `b`.
#[inline(always)]
pub fn pkhbt(a: u32, b: u32) -> u32 {
    (a & 0x0000_FFFF) | (b & 0xFFFF_0000)
}

/// Pack: upper 16 bits of `a` and lower 16 bits of `b`.
#[inline(always)]
pub fn pkhtb(a: u32, b: u32) -> u32 {
    (a & 0xFFFF_0000) | (b & 0x0000_FFFF)
}

// ---------------------------------------------------------------------------
// SMULBB / SMULTT — 16×16 single multiply (LLVM uses MUL, not SMULBB)
// These DO need explicit asm to get the single-cycle 16×16 instruction.
// ---------------------------------------------------------------------------

/// Signed multiply bottom-half × bottom-half (`SMULBB`).
///
/// LLVM does **not** automatically emit `SMULBB` for
/// `(a as i16) * (b as i16)` — it uses a standard 32-bit multiply.
/// Use this wrapper to get the single-cycle 16×16 form.
#[inline]
pub fn smulbb(a: i32, b: i32) -> i32 {
    #[cfg(target_feature = "dsp")]
    {
        let result: i32;
        // SAFETY: pure arithmetic, no memory access, single instruction.
        unsafe {
            core::arch::asm!(
                "smulbb {0}, {1}, {2}",
                out(reg) result,
                in(reg) a,
                in(reg) b,
                options(pure, nomem, nostack),
            );
        }
        result
    }
    #[cfg(not(target_feature = "dsp"))]
    {
        ((a as i16) as i32) * ((b as i16) as i32)
    }
}

/// Signed multiply top-half × top-half (`SMULTT`).
#[inline]
pub fn smultt(a: i32, b: i32) -> i32 {
    #[cfg(target_feature = "dsp")]
    {
        let result: i32;
        unsafe {
            core::arch::asm!(
                "smultt {0}, {1}, {2}",
                out(reg) result,
                in(reg) a,
                in(reg) b,
                options(pure, nomem, nostack),
            );
        }
        result
    }
    #[cfg(not(target_feature = "dsp"))]
    {
        ((a >> 16) as i16 as i32) * ((b >> 16) as i16 as i32)
    }
}

// ---------------------------------------------------------------------------
// SMLAD — dual 16×16 multiply-accumulate  ← MUST use asm, not auto-generated
// ---------------------------------------------------------------------------

/// Dual signed 16×16 multiply-accumulate (`SMLAD`).
///
/// Computes `acc + (a[15:0] * b[15:0]) + (a[31:16] * b[31:16])`.
///
/// LLVM does **not** auto-generate `SMLAD` from scalar Rust loops; this
/// is the primary instruction this module exists to expose.  Used in the
/// FIR filter hot-path to process two tap-coefficient pairs per cycle.
#[inline]
pub fn smlad(a: i32, b: i32, acc: i32) -> i32 {
    #[cfg(target_feature = "dsp")]
    {
        let result: i32;
        unsafe {
            core::arch::asm!(
                "smlad {0}, {1}, {2}, {3}",
                out(reg) result,
                in(reg) a,
                in(reg) b,
                in(reg) acc,
                options(pure, nomem, nostack),
            );
        }
        result
    }
    #[cfg(not(target_feature = "dsp"))]
    {
        let p0 = ((a as i16) as i32) * ((b as i16) as i32);
        let p1 = ((a >> 16) as i16 as i32) * ((b >> 16) as i16 as i32);
        acc.wrapping_add(p0).wrapping_add(p1)
    }
}

/// Dual signed 16×16 multiply-accumulate, cross (`SMLADX`).
///
/// Computes `acc + (a[15:0] * b[31:16]) + (a[31:16] * b[15:0])`.
#[inline]
pub fn smladx(a: i32, b: i32, acc: i32) -> i32 {
    #[cfg(target_feature = "dsp")]
    {
        let result: i32;
        unsafe {
            core::arch::asm!(
                "smladx {0}, {1}, {2}, {3}",
                out(reg) result,
                in(reg) a,
                in(reg) b,
                in(reg) acc,
                options(pure, nomem, nostack),
            );
        }
        result
    }
    #[cfg(not(target_feature = "dsp"))]
    {
        let p0 = ((a as i16) as i32) * ((b >> 16) as i16 as i32);
        let p1 = ((a >> 16) as i16 as i32) * ((b as i16) as i32);
        acc.wrapping_add(p0).wrapping_add(p1)
    }
}

// ---------------------------------------------------------------------------
// SMUAD — dual 16×16 multiply, sum (no accumulate)  ← MUST use asm
// ---------------------------------------------------------------------------

/// Dual signed 16×16 multiply, return sum of products (`SMUAD`).
///
/// Computes `(a[15:0] * b[15:0]) + (a[31:16] * b[31:16])`.
#[inline]
pub fn smuad(a: i32, b: i32) -> i32 {
    #[cfg(target_feature = "dsp")]
    {
        let result: i32;
        unsafe {
            core::arch::asm!(
                "smuad {0}, {1}, {2}",
                out(reg) result,
                in(reg) a,
                in(reg) b,
                options(pure, nomem, nostack),
            );
        }
        result
    }
    #[cfg(not(target_feature = "dsp"))]
    {
        let p0 = ((a as i16) as i32) * ((b as i16) as i32);
        let p1 = ((a >> 16) as i16 as i32) * ((b >> 16) as i16 as i32);
        p0.wrapping_add(p1)
    }
}
