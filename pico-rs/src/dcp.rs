//! DCP (Double-precision Co-Processor) HAL for RP2350 Arm
//!
//! The DCP is a Cortex-M33 custom coprocessor (CP4) that accelerates IEEE 754
//! single-precision and double-precision floating-point operations. It is
//! accessed via dedicated coprocessor instructions (`CDP`, `MCRR`, `MRRC`,
//! `MRC`, `MCRR2`, `MRRC2`).
//!
//! # Availability
//!
//! Only available on RP2350 Arm (Cortex-M33). RISC-V cores and RP2040
//! (Cortex-M0+) do **not** have the DCP.
//!
//! # Architecture
//!
//! The DCP is **not** a memory-mapped peripheral — it has no PAC register block.
//! All data is exchanged through ARM coprocessor instructions:
//!
//! ```text
//! CDP  p4, #opc1, c0, c0, cRm, #opc2  — control (add, sub, sqrt, normalize)
//! MCRR p4, #opc,  Rd, Rn, cRm          — 64-bit write to coprocessor
//! MRRC p4, #opc,  Rd, Rn, cRm          — 64-bit read from coprocessor
//! MRC  p4, #opc,  Rd, c0, cRm          — 32-bit read from coprocessor
//! ```
//!
//! The DCP operates in steps: operands are unpacked into an internal
//! representation, a computation is performed, and the result is normalized
//! and rounded back to IEEE 754 format.
//!
//! # Precision variants
//!
//! - **Correctly-rounded**: `fdiv`, `fsqrt`, `ddiv`, `dsqrt` — full IEEE 754
//!   rounding, uses Newton-Raphson refinement.
//! - **Fast**: `fdiv_fast`, `fsqrt_fast`, `ddiv_fast`, `dsqrt_fast` — ~11 bits
//!   of precision, significantly faster. Useful for graphics, approximate math,
//!   and initial guesses.
//!
//! # Usage
//!
//! The DCP is a shared resource. Obtain a handle via [`Dcp::steal()`]:
//!
//! ```ignore
//! use pico_rs::dcp::Dcp;
//!
//! let dcp = unsafe { Dcp::steal() };
//! let sum = dcp.fadd(1.0f32, 2.0f32);
//! let quotient = dcp.ddiv(1.0f64, 3.0f64);
//! ```
//!
//! All methods take `&self` and may be called concurrently from interrupts
//! (each operation sets up and consumes the DCP state within its own asm block).
//!
//! # Safety
//!
//! The DCP is enabled in CPACR by the `#[entry]` macro during startup.
//! Calling DCP methods before `#[entry]` runs will cause an UNDEFINSTR
//! exception (the coprocessor is disabled).

// Suppress warnings for scratch registers whose final values are unused after
// the asm! block — these are temporary working registers consumed entirely
// within the inline assembly.
#![allow(unused_assignments)]

use core::f32;
use core::f64;

use crate::Steal;

// ============================================================================
// Dcp handle
// ============================================================================

/// Handle to the RP2350 Double-precision Co-Processor (DCP).
///
/// The DCP is a shared resource — only one handle should exist. Obtain it
/// through [`Dcp::steal()`].
pub struct Dcp {
    _private: (),
}

impl Dcp {
    /// Obtain a DCP handle without runtime checks.
    ///
    /// # Safety
    ///
    /// The caller must ensure no other `Dcp` handle exists. Typically called
    /// once at startup before any interrupts are enabled.
    #[inline]
    pub unsafe fn steal() -> Steal<Dcp> {
        Steal(Dcp { _private: () })
    }
}

// ============================================================================
// Helper: split / join f64 as (u32, u32)
// ============================================================================

#[inline]
fn f64_to_pair(v: f64) -> (u32, u32) {
    let bits = v.to_bits();
    (bits as u32, (bits >> 32) as u32)
}

#[inline]
fn pair_to_f64(lo: u32, hi: u32) -> f64 {
    f64::from_bits((hi as u64) << 32 | lo as u64)
}

// ============================================================================
// Float (f32) operations
// ============================================================================

impl Dcp {
    /// Single-precision floating-point addition: `a + b`.
    ///
    /// Fully IEEE 754 compliant (correctly rounded).
    ///
    /// Canned sequence: `dcp_fadd_m`
    #[inline]
    pub fn fadd(&self, a: f32, b: f32) -> f32 {
        let a_bits = a.to_bits();
        let b_bits = b.to_bits();
        let result: u32;
        unsafe {
            core::arch::asm!(
                "mcrr p4, #1, {rx}, {ry}, c2",   // WXYU — write X and Y, unpack float
                "cdp  p4, #0, c0, c0, c1, #0",   // ADD0
                "cdp  p4, #1, c0, c0, c1, #0",   // ADD1
                "cdp  p4, #8, c0, c0, c2, #1",   // NRDF — normalize & round float
                "mrc  p4, #0, {rz}, c0, c2, #0", // RDFA — read float add result
                rx = in(reg) a_bits,
                ry = in(reg) b_bits,
                rz = out(reg) result,
                options(nomem, nostack),
            );
        }
        f32::from_bits(result)
    }

    /// Single-precision floating-point subtraction: `a - b`.
    ///
    /// Fully IEEE 754 compliant (correctly rounded).
    ///
    /// Canned sequence: `dcp_fsub_m`
    #[inline]
    pub fn fsub(&self, a: f32, b: f32) -> f32 {
        let a_bits = a.to_bits();
        let b_bits = b.to_bits();
        let result: u32;
        unsafe {
            core::arch::asm!(
                "mcrr p4, #1, {rx}, {ry}, c2",   // WXYU
                "cdp  p4, #0, c0, c0, c1, #0",   // ADD0
                "cdp  p4, #1, c0, c0, c1, #1",   // SUB1
                "cdp  p4, #8, c0, c0, c2, #1",   // NRDF
                "mrc  p4, #0, {rz}, c0, c2, #1", // RDFS
                rx = in(reg) a_bits,
                ry = in(reg) b_bits,
                rz = out(reg) result,
                options(nomem, nostack),
            );
        }
        f32::from_bits(result)
    }

    /// Single-precision floating-point multiplication: `a * b`.
    ///
    /// Fully IEEE 754 compliant (correctly rounded).
    ///
    /// Canned sequence: `dcp_fmul_m`
    #[inline]
    pub fn fmul(&self, a: f32, b: f32) -> f32 {
        let a_bits = a.to_bits();
        let b_bits = b.to_bits();
        let mut t0 = 0u32;
        let mut t1 = 0u32;
        let result: u32;
        unsafe {
            core::arch::asm!(
                "mcrr p4, #1, {rx}, {ry}, c2",    // WXYU
                "mrrc p4, #1, {t0}, {t1}, c1",    // RXYH
                "umull {t0}, {t1}, {t0}, {t1}",   // umull t0, t1, t0, t1
                "mcrr p4, #10, {t0}, {t1}, c0",   // WXFM
                "cdp  p4, #8, c0, c0, c2, #1",    // NRDF
                "mrc  p4, #0, {rz}, c0, c2, #2",  // RDFM
                rx = in(reg) a_bits,
                ry = in(reg) b_bits,
                t0 = inout(reg) t0,
                t1 = inout(reg) t1,
                rz = out(reg) result,
                options(nomem, nostack),
            );
        }
        f32::from_bits(result)
    }

    /// Extended single-precision product: returns the full 128-bit product of
    /// `a * b` as a (lo, hi) pair of `u32` values.
    ///
    /// This is the unpacked significand product before normalization,
    /// useful for implementing extended-precision algorithms.
    ///
    /// Canned sequence: `dcp_fxprod_m`
    #[inline]
    pub fn fxprod(&self, a: f32, b: f32) -> (u32, u32) {
        let a_bits = a.to_bits();
        let b_bits = b.to_bits();
        let mut t0 = 0u32;
        let mut t1 = 0u32;
        let lo: u32;
        let hi: u32;
        unsafe {
            core::arch::asm!(
                "mcrr p4, #1, {rx}, {ry}, c2",    // WXYU
                "mrrc p4, #1, {t0}, {t1}, c1",    // RXYH
                "umull {t0}, {t1}, {t0}, {t1}",   // umull t0, t1, t0, t1
                "mcrr p4, #3, {t0}, {t1}, c0",    // WXMO
                "cdp  p4, #8, c0, c0, c0, #1",    // NRDD
                "mrrc p4, #5, {lo}, {hi}, c0",    // RDDM
                rx = in(reg) a_bits,
                ry = in(reg) b_bits,
                t0 = inout(reg) t0,
                t1 = inout(reg) t1,
                lo = out(reg) lo,
                hi = out(reg) hi,
                options(nomem, nostack),
            );
        }
        (lo, hi)
    }

    /// Single-precision correctly-rounded division: `a / b`.
    ///
    /// Uses Newton-Raphson refinement for full IEEE 754 rounding.
    /// Requires ~7 scratch registers.
    ///
    /// Canned sequence: `dcp_fdiv_m`
    #[inline]
    pub fn fdiv(&self, a: f32, b: f32) -> f32 {
        let a_bits = a.to_bits();
        let b_bits = b.to_bits();
        let mut t0 = 0u32;
        let mut t1 = 0u32;
        let mut t2 = 0u32;
        let mut t3 = 0u32;
        let result: u32;
        unsafe {
            core::arch::asm!(
                // dcp_fdiv_m rz, rx, ry, ra, rb, rc, rd
                "mcrr p4, #1, {rx}, {ry}, c2",     // WXYU
                "mrrc p4, #2, {t0}, {t1}, c1",     // RYMR  t0:t1 = Y reciprocal mantissa
                "umull {t1}, {t2}, {t0}, {t1}",     // umull t1, t2, t0, t1
                "mvn {t2}, {t2}, lsl #2",           // mvn
                "smmlar {t0}, {t2}, {t0}, {t0}",    // smmlar
                "smmulr {t2}, {t2}, {t2}",          // smmulr
                "smmlar {t0}, {t2}, {t0}, {t0}",    // smmlar
                "mrrc p4, #1, {t1}, {t2}, c1",     // RXYH  t1:t2 = X*Y high
                "umull {t3}, {t0}, {t0}, {t1}",     // umull t3, t0, t0, t1
                "orr {t0}, {t0}, {t0}, lsr #24",    // orr
                "bic {t0}, {t0}, {t0}, lsr #25",    // bic
                "bic {t0}, {t0}, #15",              // bic
                "mov {t2}, {t2}, lsr #7",           // mov
                "mul {t3}, {t0}, {t2}",             // mul
                "rsb {t3}, {t3}, {t1}, lsl #22",    // rsb
                "sub {t0}, {t0}, {t3}, lsr #31",    // sub
                "mcrr p4, #11, {t0}, {t0}, c0",    // WXFD
                "cdp  p4, #8, c0, c0, c2, #1",     // NRDF
                "mrc  p4, #0, {rz}, c0, c2, #3",   // RDFD
                rx = in(reg) a_bits,
                ry = in(reg) b_bits,
                t0 = inout(reg) t0,
                t1 = inout(reg) t1,
                t2 = inout(reg) t2,
                t3 = inout(reg) t3,
                rz = out(reg) result,
                options(nomem, nostack),
            );
        }
        f32::from_bits(result)
    }

    /// Single-precision fast division: `a / b`.
    ///
    /// ~11 bits of precision — significantly faster than [`fdiv`](Self::fdiv).
    /// Suitable for graphics, approximate math, and initial guesses.
    ///
    /// Canned sequence: `dcp_fdiv_fast_m`
    #[inline]
    pub fn fdiv_fast(&self, a: f32, b: f32) -> f32 {
        let a_bits = a.to_bits();
        let b_bits = b.to_bits();
        let mut t0 = 0u32;
        let mut t1 = 0u32;
        let mut t2 = 0u32;
        let result: u32;
        unsafe {
            core::arch::asm!(
                "mcrr p4, #1, {rx}, {ry}, c2",     // WXYU
                "mrrc p4, #2, {t0}, {t1}, c1",     // RYMR
                "umull {t1}, {t2}, {t0}, {t1}",     // umull
                "mvn {t2}, {t2}, lsl #2",           // mvn
                "smmlar {t0}, {t2}, {t0}, {t0}",    // smmlar
                "smmulr {t2}, {t2}, {t2}",          // smmulr
                "smmlar {t0}, {t2}, {t0}, {t0}",    // smmlar
                "mrrc p4, #1, {t1}, {t2}, c1",     // RXYH
                "umull {t0}, {t1}, {t0}, {t1}",     // umull
                "mcrr p4, #11, {t1}, {t1}, c0",    // WXFD
                "cdp  p4, #8, c0, c0, c2, #1",     // NRDF
                "mrc  p4, #0, {rz}, c0, c2, #3",   // RDFD
                rx = in(reg) a_bits,
                ry = in(reg) b_bits,
                t0 = inout(reg) t0,
                t1 = inout(reg) t1,
                t2 = inout(reg) t2,
                rz = out(reg) result,
                options(nomem, nostack),
            );
        }
        f32::from_bits(result)
    }

    /// Single-precision correctly-rounded square root: `sqrt(a)`.
    ///
    /// Uses Newton-Raphson refinement for full IEEE 754 rounding.
    ///
    /// Canned sequence: `dcp_fsqrt_m`
    #[inline]
    pub fn fsqrt(&self, a: f32) -> f32 {
        let a_bits = a.to_bits();
        let mut t0 = 0u32;
        let mut t1 = 0u32;
        let mut t2 = 0u32;
        let mut t3 = 0u32;
        let result: u32;
        unsafe {
            core::arch::asm!(
                // dcp_fsqrt_m rz, rx, ra, rb, rc, rd
                "mcrr p4, #1, {rx}, {rx}, c2",     // WXYU rx, rx
                "cdp  p4, #2, c0, c0, c1, #0",     // SQR0
                "mrrc p4, #4, {t2}, {t3}, c1",     // RXMQ  t2:t3 = mantissa
                "umull {t0}, {t1}, {t2}, {t2}",     // umull
                "umull {t0}, {t1}, {t1}, {t3}",     // umull
                "mov {t1}, {t1}, lsl #3",           // mov
                "sub {t1}, {t1}, #2147483648",      // sub
                "smmlsr {t2}, {t2}, {t1}, {t2}",    // smmlsr
                "umull {t0}, {t1}, {t2}, {t2}",     // umull
                "umull {t0}, {t1}, {t1}, {t3}",     // umull
                "movs {t0}, {t0}, lsr #28",         // movs
                "adc {t1}, {t0}, {t1}, lsl #4",     // adc
                "smmulr {t0}, {t1}, {t2}",          // smmulr
                "sub {t2}, {t2}, {t0}, asr #1",     // sub
                "umull {t0}, {t1}, {t2}, {t3}",     // umull
                "orr {t1}, {t1}, #63",              // orr
                "mov {t0}, {t1}, lsr #5",           // mov
                "mul {t0}, {t0}, {t0}",             // mul
                "rsb {t0}, {t0}, {t3}, lsl #18",    // rsb
                "bic {t1}, {t1}, {t0}, lsr #26",    // bic
                "mcrr p4, #12, {t0}, {t1}, c0",    // WXFQ
                "cdp  p4, #8, c0, c0, c2, #1",     // NRDF
                "mrc  p4, #0, {rz}, c0, c2, #4",   // RDFQ
                rx = in(reg) a_bits,
                t0 = inout(reg) t0,
                t1 = inout(reg) t1,
                t2 = inout(reg) t2,
                t3 = inout(reg) t3,
                rz = out(reg) result,
                options(nomem, nostack),
            );
        }
        f32::from_bits(result)
    }

    /// Single-precision fast square root: `sqrt(a)`.
    ///
    /// ~11 bits of precision — significantly faster than [`fsqrt`](Self::fsqrt).
    ///
    /// Canned sequence: `dcp_fsqrt_fast_m`
    #[inline]
    pub fn fsqrt_fast(&self, a: f32) -> f32 {
        let a_bits = a.to_bits();
        let mut t0 = 0u32;
        let mut t1 = 0u32;
        let mut t2 = 0u32;
        let mut t3 = 0u32;
        let result: u32;
        unsafe {
            core::arch::asm!(
                // dcp_fsqrt_fast_m rz, rx, ra, rb, rc, rd
                "mcrr p4, #1, {rx}, {rx}, c2",     // WXYU rx, rx
                "cdp  p4, #2, c0, c0, c1, #0",     // SQR0
                "mrrc p4, #4, {t2}, {t3}, c1",     // RXMQ
                "umull {t0}, {t1}, {t2}, {t2}",     // umull
                "umull {t0}, {t1}, {t1}, {t3}",     // umull
                "mov {t1}, {t1}, lsl #3",           // mov
                "sub {t1}, {t1}, #2147483648",      // sub
                "smmlsr {t2}, {t2}, {t1}, {t2}",    // smmlsr
                "umull {t0}, {t1}, {t2}, {t2}",     // umull
                "umull {t0}, {t1}, {t1}, {t3}",     // umull
                "movs {t0}, {t0}, lsr #28",         // movs
                "adc {t1}, {t0}, {t1}, lsl #4",     // adc
                "smmulr {t0}, {t1}, {t2}",          // smmulr
                "sub {t2}, {t2}, {t0}, asr #1",     // sub
                "umull {t0}, {t1}, {t2}, {t3}",     // umull
                "mcrr p4, #12, {t0}, {t1}, c0",    // WXFQ
                "cdp  p4, #8, c0, c0, c2, #1",     // NRDF
                "mrc  p4, #0, {rz}, c0, c2, #4",   // RDFQ
                rx = in(reg) a_bits,
                t0 = inout(reg) t0,
                t1 = inout(reg) t1,
                t2 = inout(reg) t2,
                t3 = inout(reg) t3,
                rz = out(reg) result,
                options(nomem, nostack),
            );
        }
        f32::from_bits(result)
    }

    /// Classify a single-precision floating-point value.
    ///
    /// Returns a bitmask using the ARM APSR bit layout:
    ///
    /// | Bit | APSR | Meaning |
    /// |-----|------|---------|
    /// | 31 | N | Sign (1 = negative) |
    /// | 30 | Z | Zero (and subnormals) |
    /// | 29 | C | Infinity |
    /// | 28 | V | NaN |
    /// | 2 | — | Always set (result valid) |
    ///
    /// Note: subnormal values are reported the same as zero.
    ///
    /// Canned sequence: `dcp_fclassify_m`
    #[inline]
    pub fn fclassify(&self, a: f32) -> u32 {
        let a_bits = a.to_bits();
        let result: u32;
        unsafe {
            core::arch::asm!(
                "mcrr p4, #1, {rx}, {rx}, c2",     // WXYU
                "mrc  p4, #0, {rz}, c0, c0, #0",   // RXVD
                rx = in(reg) a_bits,
                rz = out(reg) result,
                options(nomem, nostack),
            );
        }
        result
    }

    /// Compare two single-precision floating-point values.
    ///
    /// Returns the comparison result encoded in ARM APSR bit layout:
    ///
    /// | Bit | APSR | Meaning |
    /// |-----|------|---------|
    /// | 31 | N | (unused by comparison logic) |
    /// | 30 | Z | Equal — set if a == b |
    /// | 29 | C | Greater-or-equal — set if a >= b |
    /// | 28 | V | Unordered — set if either operand is NaN |
    ///
    /// Use [`Self::fcmp_eq`], [`Self::fcmp_lt`], etc. for semantic comparisons.
    ///
    /// Canned sequence: `dcp_fcmp_m`
    #[inline]
    pub fn fcmp(&self, a: f32, b: f32) -> u32 {
        let a_bits = a.to_bits();
        let b_bits = b.to_bits();
        let result: u32;
        unsafe {
            core::arch::asm!(
                "mcrr p4, #1, {rx}, {ry}, c2",     // WXYU
                "cdp  p4, #0, c0, c0, c1, #0",     // ADD0
                "mrc  p4, #0, {rz}, c0, c0, #1",   // RCMP
                rx = in(reg) a_bits,
                ry = in(reg) b_bits,
                rz = out(reg) result,
                options(nomem, nostack),
            );
        }
        result
    }
}

// ============================================================================
// Double (f64) operations
// ============================================================================

impl Dcp {
    /// Double-precision floating-point addition: `a + b`.
    ///
    /// Fully IEEE 754 compliant (correctly rounded).
    ///
    /// Canned sequence: `dcp_dadd_m`
    #[inline]
    pub fn dadd(&self, a: f64, b: f64) -> f64 {
        let (a_lo, a_hi) = f64_to_pair(a);
        let (b_lo, b_hi) = f64_to_pair(b);
        let lo: u32;
        let hi: u32;
        unsafe {
            core::arch::asm!(
                "mcrr p4, #1, {rxl}, {rxh}, c0",   // WXUP — write X, unpack double
                "mcrr p4, #1, {ryl}, {ryh}, c1",   // WYUP — write Y, unpack double
                "cdp  p4, #0, c0, c0, c1, #0",     // ADD0
                "cdp  p4, #1, c0, c0, c1, #0",     // ADD1
                "cdp  p4, #8, c0, c0, c0, #1",     // NRDD — normalize & round double
                "mrrc p4, #1, {rzl}, {rzh}, c0",   // RDDA — read double add result
                rxl = in(reg) a_lo,
                rxh = in(reg) a_hi,
                ryl = in(reg) b_lo,
                ryh = in(reg) b_hi,
                rzl = out(reg) lo,
                rzh = out(reg) hi,
                options(nomem, nostack),
            );
        }
        pair_to_f64(lo, hi)
    }

    /// Double-precision floating-point subtraction: `a - b`.
    ///
    /// Fully IEEE 754 compliant (correctly rounded).
    ///
    /// Canned sequence: `dcp_dsub_m`
    #[inline]
    pub fn dsub(&self, a: f64, b: f64) -> f64 {
        let (a_lo, a_hi) = f64_to_pair(a);
        let (b_lo, b_hi) = f64_to_pair(b);
        let lo: u32;
        let hi: u32;
        unsafe {
            core::arch::asm!(
                "mcrr p4, #1, {rxl}, {rxh}, c0",   // WXUP
                "mcrr p4, #1, {ryl}, {ryh}, c1",   // WYUP
                "cdp  p4, #0, c0, c0, c1, #0",     // ADD0
                "cdp  p4, #1, c0, c0, c1, #1",     // SUB1
                "cdp  p4, #8, c0, c0, c0, #1",     // NRDD
                "mrrc p4, #3, {rzl}, {rzh}, c0",   // RDDS
                rxl = in(reg) a_lo,
                rxh = in(reg) a_hi,
                ryl = in(reg) b_lo,
                ryh = in(reg) b_hi,
                rzl = out(reg) lo,
                rzh = out(reg) hi,
                options(nomem, nostack),
            );
        }
        pair_to_f64(lo, hi)
    }

    /// Double-precision floating-point multiplication: `a * b`.
    ///
    /// Fully IEEE 754 compliant (correctly rounded).
    /// Requires 11 scratch registers.
    ///
    /// Canned sequence: `dcp_dmul_m`
    #[inline]
    pub fn dmul(&self, a: f64, b: f64) -> f64 {
        let (a_lo, a_hi) = f64_to_pair(a);
        let (b_lo, b_hi) = f64_to_pair(b);
        let mut t0 = 0u32;
        let mut t1 = 0u32;
        let mut t2 = 0u32;
        let mut t3 = 0u32;
        let mut t4 = 0u32;
        let mut t5 = 0u32;
        let mut t6 = 0u32;
        let lo: u32;
        let hi: u32;
        unsafe {
            core::arch::asm!(
                // dcp_dmul_m rzl, rzh, rxl, rxh, ryl, ryh, ra..rg
                "mcrr p4, #1, {rxl}, {rxh}, c0",   // WXUP
                "mcrr p4, #1, {ryl}, {ryh}, c1",   // WYUP
                "mrrc p4, #0, {t0}, {t1}, c4",     // RXMS t0,t1,0
                "mrrc p4, #0, {t2}, {t3}, c5",     // RYMS t2,t3,0
                "umull {t4}, {t5}, {t0}, {t2}",     // umull
                "movs {t6}, #0",                    // movs
                "umlal {t5}, {t6}, {t0}, {t3}",     // umlal
                "umlal {t5}, {t6}, {t1}, {t2}",     // umlal
                "mcrr p4, #2, {t4}, {t5}, c0",     // WXMS
                "movs {t4}, #0",                    // movs
                "umlal {t6}, {t4}, {t1}, {t3}",     // umlal
                "mcrr p4, #3, {t6}, {t4}, c0",     // WXMO
                "cdp  p4, #8, c0, c0, c0, #1",     // NRDD
                "mrrc p4, #5, {rzl}, {rzh}, c0",   // RDDM
                rxl = in(reg) a_lo,
                rxh = in(reg) a_hi,
                ryl = in(reg) b_lo,
                ryh = in(reg) b_hi,
                t0 = inout(reg) t0,
                t1 = inout(reg) t1,
                t2 = inout(reg) t2,
                t3 = inout(reg) t3,
                t4 = inout(reg) t4,
                t5 = inout(reg) t5,
                t6 = inout(reg) t6,
                rzl = out(reg) lo,
                rzh = out(reg) hi,
                options(nomem, nostack),
            );
        }
        pair_to_f64(lo, hi)
    }

    /// Double-precision correctly-rounded division: `a / b`.
    ///
    /// Uses Newton-Raphson refinement for full IEEE 754 rounding.
    ///
    /// Canned sequence: `dcp_ddiv_m`
    #[inline]
    pub fn ddiv(&self, a: f64, b: f64) -> f64 {
        let (a_lo, a_hi) = f64_to_pair(a);
        let (b_lo, b_hi) = f64_to_pair(b);
        let mut t0 = 0u32;
        let mut t1 = 0u32;
        let mut t2 = 0u32;
        let mut t3 = 0u32;
        let mut t4 = 0u32;
        let lo: u32;
        let hi: u32;
        unsafe {
            core::arch::asm!(
                // dcp_ddiv_m rzl,rzh,rxl,rxh,ryl,ryh,ra..re
                "mcrr p4, #1, {rxl}, {rxh}, c0",   // WXUP
                "mcrr p4, #1, {ryl}, {ryh}, c1",   // WYUP
                "mrrc p4, #2, {t0}, {t1}, c1",     // RYMR
                "umull {t1}, {t2}, {t0}, {t1}",     // umull
                "mvn {t2}, {t2}, lsl #2",           // mvn
                "smmlar {t0}, {t2}, {t0}, {t0}",    // smmlar
                "smmulr {t2}, {t2}, {t2}",          // smmulr
                "smmlar {t0}, {t2}, {t0}, {t0}",    // smmlar
                "sub {t4}, {t0}, {t0}, lsr #31",    // sub
                "mrrc p4, #0, {t2}, {t3}, c4",     // RXMS t2,t3,0
                "smmulr {t1}, {t4}, {t3}",          // smmulr
                "mrrc p4, #1, {t2}, {t3}, c5",     // RYMS t2,t3,1
                "umull {t2}, {t0}, {t1}, {t2}",     // umull
                "mla {t0}, {t1}, {t3}, {t0}",       // mla
                "mrrc p4, #4, {t2}, {t3}, c4",     // RXMS t2,t3,4
                "sub {t0}, {t2}, {t0}",             // sub
                "smmulr {t2}, {t0}, {t4}",          // smmulr
                "mov {t3}, {t1}, lsr #4",           // mov
                "adds {t0}, {t2}, {t1}, lsl #28",   // adds
                "adc {t1}, {t3}, {t2}, asr #31",    // adc
                "orr {t0}, {t0}, {t1}, lsr #21",    // orr
                "bic {t0}, {t0}, {t1}, lsr #22",    // bic
                "bic {t0}, {t0}, #7",               // bic
                "mrrc p4, #7, {t2}, {t3}, c5",     // RYMS t2,t3,7
                "umull {t3}, {t4}, {t0}, {t2}",     // umull
                "mrrc p4, #7, {t2}, {t3}, c5",     // RYMS t2,t3,7
                "mla {t4}, {t0}, {t3}, {t4}",       // mla
                "mla {t4}, {t1}, {t2}, {t4}",       // mla
                "mrrc p4, #0, {t2}, {t3}, c4",     // RXMS t2,t3,0
                "sub {t4}, {t4}, {t2}, lsl #18",    // sub
                "orr {t0}, {t0}, {t4}, lsr #29",    // orr
                "sub {t0}, {t0}, #1",               // sub
                "mcrr p4, #4, {t0}, {t1}, c0",     // WXDD
                "cdp  p4, #8, c0, c0, c0, #1",     // NRDD
                "mrrc p4, #7, {rzl}, {rzh}, c0",   // RDDD
                rxl = in(reg) a_lo,
                rxh = in(reg) a_hi,
                ryl = in(reg) b_lo,
                ryh = in(reg) b_hi,
                t0 = inout(reg) t0,
                t1 = inout(reg) t1,
                t2 = inout(reg) t2,
                t3 = inout(reg) t3,
                t4 = inout(reg) t4,
                rzl = out(reg) lo,
                rzh = out(reg) hi,
                options(nomem, nostack),
            );
        }
        pair_to_f64(lo, hi)
    }

    /// Double-precision fast division: `a / b`.
    ///
    /// ~11 bits of precision — significantly faster than [`ddiv`](Self::ddiv).
    ///
    /// Canned sequence: `dcp_ddiv_fast_m`
    #[inline]
    pub fn ddiv_fast(&self, a: f64, b: f64) -> f64 {
        let (a_lo, a_hi) = f64_to_pair(a);
        let (b_lo, b_hi) = f64_to_pair(b);
        let mut t0 = 0u32;
        let mut t1 = 0u32;
        let mut t2 = 0u32;
        let mut t3 = 0u32;
        let mut t4 = 0u32;
        let lo: u32;
        let hi: u32;
        unsafe {
            core::arch::asm!(
                // dcp_ddiv_fast_m rzl,rzh,rxl,rxh,ryl,ryh,ra..re
                "mcrr p4, #1, {rxl}, {rxh}, c0",   // WXUP
                "mcrr p4, #1, {ryl}, {ryh}, c1",   // WYUP
                "mrrc p4, #2, {t0}, {t1}, c1",     // RYMR
                "umull {t1}, {t2}, {t0}, {t1}",     // umull
                "mvn {t2}, {t2}, lsl #2",           // mvn
                "smmlar {t0}, {t2}, {t0}, {t0}",    // smmlar
                "smmulr {t2}, {t2}, {t2}",          // smmulr
                "smmlar {t0}, {t2}, {t0}, {t0}",    // smmlar
                "sub {t4}, {t0}, {t0}, lsr #31",    // sub
                "mrrc p4, #0, {t2}, {t3}, c4",     // RXMS t2,t3,0
                "smmulr {t1}, {t4}, {t3}",          // smmulr
                "mrrc p4, #1, {t2}, {t3}, c5",     // RYMS t2,t3,1
                "umull {t2}, {t0}, {t1}, {t2}",     // umull
                "mla {t0}, {t1}, {t3}, {t0}",       // mla
                "mrrc p4, #4, {t2}, {t3}, c4",     // RXMS t2,t3,4
                "sub {t0}, {t2}, {t0}",             // sub
                "smmulr {t2}, {t0}, {t4}",          // smmulr
                "mov {t3}, {t1}, lsr #4",           // mov
                "adds {t0}, {t2}, {t1}, lsl #28",   // adds
                "adc {t1}, {t3}, {t2}, asr #31",    // adc
                "mcrr p4, #4, {t0}, {t1}, c0",     // WXDD
                "cdp  p4, #8, c0, c0, c0, #1",     // NRDD
                "mrrc p4, #7, {rzl}, {rzh}, c0",   // RDDD
                rxl = in(reg) a_lo,
                rxh = in(reg) a_hi,
                ryl = in(reg) b_lo,
                ryh = in(reg) b_hi,
                t0 = inout(reg) t0,
                t1 = inout(reg) t1,
                t2 = inout(reg) t2,
                t3 = inout(reg) t3,
                t4 = inout(reg) t4,
                rzl = out(reg) lo,
                rzh = out(reg) hi,
                options(nomem, nostack),
            );
        }
        pair_to_f64(lo, hi)
    }

    /// Double-precision correctly-rounded square root: `sqrt(a)`.
    ///
    /// Uses Newton-Raphson refinement for full IEEE 754 rounding.
    ///
    /// Canned sequence: `dcp_dsqrt_m`
    #[inline]
    pub fn dsqrt(&self, a: f64) -> f64 {
        let (a_lo, a_hi) = f64_to_pair(a);
        let mut t0 = 0u32;
        let mut t1 = 0u32;
        let mut t2 = 0u32;
        let mut t3 = 0u32;
        let mut t4 = 0u32;
        let lo: u32;
        let hi: u32;
        unsafe {
            core::arch::asm!(
                // dcp_dsqrt_m rzl,rzh,rxl,rxh,ra,..,re
                "mcrr p4, #1, {rxl}, {rxh}, c0",   // WXUP
                "cdp  p4, #2, c0, c0, c1, #0",     // SQR0
                "mrrc p4, #4, {t2}, {t3}, c1",     // RXMQ
                "umull {t0}, {t1}, {t2}, {t2}",     // umull
                "umull {t0}, {t1}, {t1}, {t3}",     // umull
                "mov {t1}, {t1}, lsl #3",           // mov
                "sub {t1}, {t1}, #2147483648",      // sub
                "smmlsr {t2}, {t2}, {t1}, {t2}",    // smmlsr
                "umull {t0}, {t1}, {t2}, {t2}",     // umull
                "umull {t0}, {t1}, {t1}, {t3}",     // umull
                "movs {t0}, {t0}, lsr #28",         // movs
                "adc {t1}, {t0}, {t1}, lsl #4",     // adc
                "smmulr {t0}, {t1}, {t2}",          // smmulr
                "sub {t2}, {t2}, {t0}, asr #1",     // sub
                "umull {t0}, {t1}, {t2}, {t3}",     // umull
                "mov {t1}, {t1}, lsr #1",           // mov
                "umull {t3}, {t0}, {t1}, {t1}",     // umull
                "mrrc p4, #6, {t0}, {t4}, c4",     // RXMS t0,t4,6
                "sub {t3}, {t0}, {t3}",             // sub
                "smmulr {t3}, {t3}, {t2}",          // smmulr
                "add {t1}, {t1}, {t3}, asr #28",    // add
                "mov {t3}, {t3}, lsl #4",           // mov
                "bic {t3}, {t3}, #255",             // bic
                "orr {t3}, {t3}, #128",             // orr
                "umull {t4}, {t2}, {t3}, {t3}",     // umull
                "mul {t4}, {t1}, {t1}",             // mul
                "umlal {t2}, {t4}, {t1}, {t3}",     // umlal
                "umlal {t2}, {t4}, {t1}, {t3}",     // umlal
                "sub {t4}, {t4}, {t0}",             // sub
                "orr {t3}, {t3}, {t4}, lsr #24",    // orr
                "sub {t3}, {t3}, #11",              // sub
                "mcrr p4, #5, {t3}, {t1}, c0",     // WXDQ
                "cdp  p4, #8, c0, c0, c0, #1",     // NRDD
                "mrrc p4, #9, {rzl}, {rzh}, c0",   // RDDQ
                rxl = in(reg) a_lo,
                rxh = in(reg) a_hi,
                t0 = inout(reg) t0,
                t1 = inout(reg) t1,
                t2 = inout(reg) t2,
                t3 = inout(reg) t3,
                t4 = inout(reg) t4,
                rzl = out(reg) lo,
                rzh = out(reg) hi,
                options(nomem, nostack),
            );
        }
        pair_to_f64(lo, hi)
    }

    /// Double-precision fast square root: `sqrt(a)`.
    ///
    /// ~11 bits of precision — significantly faster than [`dsqrt`](Self::dsqrt).
    ///
    /// Canned sequence: `dcp_dsqrt_fast_m`
    #[inline]
    pub fn dsqrt_fast(&self, a: f64) -> f64 {
        let (a_lo, a_hi) = f64_to_pair(a);
        let mut t0 = 0u32;
        let mut t1 = 0u32;
        let mut t2 = 0u32;
        let mut t3 = 0u32;
        let mut t4 = 0u32;
        let lo: u32;
        let hi: u32;
        unsafe {
            core::arch::asm!(
                // dcp_dsqrt_fast_m rzl,rzh,rxl,rxh,ra..re
                "mcrr p4, #1, {rxl}, {rxh}, c0",   // WXUP
                "cdp  p4, #2, c0, c0, c1, #0",     // SQR0
                "mrrc p4, #4, {t2}, {t3}, c1",     // RXMQ
                "umull {t0}, {t1}, {t2}, {t2}",     // umull
                "umull {t0}, {t1}, {t1}, {t3}",     // umull
                "mov {t1}, {t1}, lsl #3",           // mov
                "sub {t1}, {t1}, #2147483648",      // sub
                "smmlsr {t2}, {t2}, {t1}, {t2}",    // smmlsr
                "umull {t0}, {t1}, {t2}, {t2}",     // umull
                "umull {t0}, {t1}, {t1}, {t3}",     // umull
                "movs {t0}, {t0}, lsr #28",         // movs
                "adc {t1}, {t0}, {t1}, lsl #4",     // adc
                "smmulr {t0}, {t1}, {t2}",          // smmulr
                "sub {t2}, {t2}, {t0}, asr #1",     // sub
                "umull {t0}, {t1}, {t2}, {t3}",     // umull
                "mov {t1}, {t1}, lsr #1",           // mov
                "umull {t3}, {t0}, {t1}, {t1}",     // umull
                "mrrc p4, #6, {t0}, {t4}, c4",     // RXMS t0,t4,6
                "sub {t0}, {t0}, {t3}",             // sub
                "smmulr {t0}, {t0}, {t2}",          // smmulr
                "add {t1}, {t1}, {t0}, asr #28",    // add
                "mov {t0}, {t0}, lsl #4",           // mov
                "mcrr p4, #5, {t0}, {t1}, c0",     // WXDQ
                "cdp  p4, #8, c0, c0, c0, #1",     // NRDD
                "mrrc p4, #9, {rzl}, {rzh}, c0",   // RDDQ
                rxl = in(reg) a_lo,
                rxh = in(reg) a_hi,
                t0 = inout(reg) t0,
                t1 = inout(reg) t1,
                t2 = inout(reg) t2,
                t3 = inout(reg) t3,
                t4 = inout(reg) t4,
                rzl = out(reg) lo,
                rzh = out(reg) hi,
                options(nomem, nostack),
            );
        }
        pair_to_f64(lo, hi)
    }

    /// Classify a double-precision floating-point value.
    ///
    /// Returns a bitmask using the ARM APSR bit layout:
    ///
    /// | Bit | APSR | Meaning |
    /// |-----|------|---------|
    /// | 31 | N | Sign (1 = negative) |
    /// | 30 | Z | Zero (and subnormals) |
    /// | 29 | C | Infinity |
    /// | 28 | V | NaN |
    /// | 2 | — | Always set (result valid) |
    ///
    /// Note: subnormal values are reported the same as zero.
    ///
    /// Canned sequence: `dcp_dclassify_m`
    #[inline]
    pub fn dclassify(&self, a: f64) -> u32 {
        let (a_lo, a_hi) = f64_to_pair(a);
        let result: u32;
        unsafe {
            core::arch::asm!(
                "mcrr p4, #1, {rxl}, {rxh}, c0",   // WXUP
                "mrc  p4, #0, {rz}, c0, c0, #0",   // RXVD
                rxl = in(reg) a_lo,
                rxh = in(reg) a_hi,
                rz = out(reg) result,
                options(nomem, nostack),
            );
        }
        result
    }

    /// Compare two double-precision floating-point values.
    ///
    /// Returns the comparison result encoded in ARM APSR bit layout:
    ///
    /// | Bit | APSR | Meaning |
    /// |-----|------|---------|
    /// | 31 | N | (unused by comparison logic) |
    /// | 30 | Z | Equal — set if a == b |
    /// | 29 | C | Greater-or-equal — set if a >= b |
    /// | 28 | V | Unordered — set if either operand is NaN |
    ///
    /// Use [`Self::dcmp_eq`], [`Self::dcmp_lt`], etc. for semantic comparisons.
    ///
    /// Canned sequence: `dcp_dcmp_m`
    #[inline]
    pub fn dcmp(&self, a: f64, b: f64) -> u32 {
        let (a_lo, a_hi) = f64_to_pair(a);
        let (b_lo, b_hi) = f64_to_pair(b);
        let result: u32;
        unsafe {
            core::arch::asm!(
                "mcrr p4, #1, {rxl}, {rxh}, c0",   // WXUP
                "mcrr p4, #1, {ryl}, {ryh}, c1",   // WYUP
                "cdp  p4, #0, c0, c0, c1, #0",     // ADD0
                "mrc  p4, #0, {rz}, c0, c0, #1",   // RCMP
                rxl = in(reg) a_lo,
                rxh = in(reg) a_hi,
                ryl = in(reg) b_lo,
                ryh = in(reg) b_hi,
                rz = out(reg) result,
                options(nomem, nostack),
            );
        }
        result
    }
}

// ============================================================================
// Type conversions
// ============================================================================

impl Dcp {
    /// Convert `f32` to `f64` with correct rounding.
    #[inline]
    pub fn float2double(&self, a: f32) -> f64 {
        let a_bits = a.to_bits();
        let lo: u32;
        let hi: u32;
        unsafe {
            core::arch::asm!(
                "mcrr p4, #1, {rx}, {rx}, c2",     // WXYU
                "cdp  p4, #8, c0, c0, c0, #1",     // NRDD
                "mrrc p4, #11, {rzl}, {rzh}, c0",  // RDDG
                rx = in(reg) a_bits,
                rzl = out(reg) lo,
                rzh = out(reg) hi,
                options(nomem, nostack),
            );
        }
        pair_to_f64(lo, hi)
    }

    /// Convert `f64` to `f32` with correct rounding.
    #[inline]
    pub fn double2float(&self, a: f64) -> f32 {
        let (a_lo, a_hi) = f64_to_pair(a);
        let result: u32;
        unsafe {
            core::arch::asm!(
                "mcrr p4, #1, {rxl}, {rxh}, c0",   // WXUP
                "cdp  p4, #8, c0, c0, c2, #1",     // NRDF
                "mrc  p4, #0, {rz}, c0, c2, #5",   // RDFG
                rxl = in(reg) a_lo,
                rxh = in(reg) a_hi,
                rz = out(reg) result,
                options(nomem, nostack),
            );
        }
        f32::from_bits(result)
    }

    /// Convert `i32` to `f32`.
    #[inline]
    pub fn int2float(&self, a: i32) -> f32 {
        let result: u32;
        unsafe {
            core::arch::asm!(
                "mcrr p4, #7, {rx}, {rx}, c0",     // WXIC — write X, signed int convert
                "cdp  p4, #0, c0, c0, c1, #0",     // ADD0
                "cdp  p4, #1, c0, c0, c1, #1",     // SUB1
                "cdp  p4, #8, c0, c0, c2, #1",     // NRDF
                "mrc  p4, #0, {rz}, c0, c2, #1",   // RDFS
                rx = in(reg) a as u32,
                rz = out(reg) result,
                options(nomem, nostack),
            );
        }
        f32::from_bits(result)
    }

    /// Convert `u32` to `f32`.
    #[inline]
    pub fn uint2float(&self, a: u32) -> f32 {
        let result: u32;
        unsafe {
            core::arch::asm!(
                "mcrr p4, #6, {rx}, {rx}, c0",     // WXUC — write X, unsigned int convert
                "cdp  p4, #0, c0, c0, c1, #0",     // ADD0
                "cdp  p4, #1, c0, c0, c1, #1",     // SUB1
                "cdp  p4, #8, c0, c0, c2, #1",     // NRDF
                "mrc  p4, #0, {rz}, c0, c2, #1",   // RDFS
                rx = in(reg) a,
                rz = out(reg) result,
                options(nomem, nostack),
            );
        }
        f32::from_bits(result)
    }

    /// Convert `i32` to `f64`.
    #[inline]
    pub fn int2double(&self, a: i32) -> f64 {
        let lo: u32;
        let hi: u32;
        unsafe {
            core::arch::asm!(
                "mcrr p4, #7, {rx}, {rx}, c0",     // WXIC
                "cdp  p4, #0, c0, c0, c1, #0",     // ADD0
                "cdp  p4, #1, c0, c0, c1, #1",     // SUB1
                "cdp  p4, #8, c0, c0, c0, #1",     // NRDD
                "mrrc p4, #3, {rzl}, {rzh}, c0",   // RDDS
                rx = in(reg) a as u32,
                rzl = out(reg) lo,
                rzh = out(reg) hi,
                options(nomem, nostack),
            );
        }
        pair_to_f64(lo, hi)
    }

    /// Convert `u32` to `f64`.
    #[inline]
    pub fn uint2double(&self, a: u32) -> f64 {
        let lo: u32;
        let hi: u32;
        unsafe {
            core::arch::asm!(
                "mcrr p4, #6, {rx}, {rx}, c0",     // WXUC
                "cdp  p4, #0, c0, c0, c1, #0",     // ADD0
                "cdp  p4, #1, c0, c0, c1, #1",     // SUB1
                "cdp  p4, #8, c0, c0, c0, #1",     // NRDD
                "mrrc p4, #3, {rzl}, {rzh}, c0",   // RDDS
                rx = in(reg) a,
                rzl = out(reg) lo,
                rzh = out(reg) hi,
                options(nomem, nostack),
            );
        }
        pair_to_f64(lo, hi)
    }

    /// Convert `f32` to `i32` (truncation toward zero).
    #[inline]
    pub fn float2int(&self, a: f32) -> i32 {
        let a_bits = a.to_bits();
        let result: u32;
        unsafe {
            core::arch::asm!(
                "mcrr p4, #9, {rx}, {rx}, c2",     // WXFC — write X, float convert
                "cdp  p4, #0, c0, c0, c1, #0",     // ADD0
                "cdp  p4, #1, c0, c0, c1, #0",     // ADD1
                "cdp  p4, #8, c0, c0, c0, #2",     // NTDC — normalize & truncate double
                "mrc  p4, #0, {rz}, c0, c3, #0",   // RDIC — read signed int convert
                rx = in(reg) a_bits,
                rz = out(reg) result,
                options(nomem, nostack),
            );
        }
        result as i32
    }

    /// Convert `f32` to `u32` (truncation toward zero).
    #[inline]
    pub fn float2uint(&self, a: f32) -> u32 {
        let a_bits = a.to_bits();
        let result: u32;
        unsafe {
            core::arch::asm!(
                "mcrr p4, #9, {rx}, {rx}, c2",     // WXFC
                "cdp  p4, #0, c0, c0, c1, #0",     // ADD0
                "cdp  p4, #1, c0, c0, c1, #0",     // ADD1
                "cdp  p4, #8, c0, c0, c0, #2",     // NTDC
                "mrc  p4, #0, {rz}, c0, c3, #1",   // RDUC — read unsigned int convert
                rx = in(reg) a_bits,
                rz = out(reg) result,
                options(nomem, nostack),
            );
        }
        result
    }

    /// Convert `f64` to `i32` (truncation toward zero).
    #[inline]
    pub fn double2int(&self, a: f64) -> i32 {
        let (a_lo, a_hi) = f64_to_pair(a);
        let result: u32;
        unsafe {
            core::arch::asm!(
                "mcrr p4, #8, {rxl}, {rxh}, c0",   // WXDC — write X, double convert
                "cdp  p4, #0, c0, c0, c1, #0",     // ADD0
                "cdp  p4, #1, c0, c0, c1, #0",     // ADD1
                "cdp  p4, #8, c0, c0, c0, #2",     // NTDC
                "mrc  p4, #0, {rz}, c0, c3, #0",   // RDIC
                rxl = in(reg) a_lo,
                rxh = in(reg) a_hi,
                rz = out(reg) result,
                options(nomem, nostack),
            );
        }
        result as i32
    }

    /// Convert `f64` to `u32` (truncation toward zero).
    #[inline]
    pub fn double2uint(&self, a: f64) -> u32 {
        let (a_lo, a_hi) = f64_to_pair(a);
        let result: u32;
        unsafe {
            core::arch::asm!(
                "mcrr p4, #8, {rxl}, {rxh}, c0",   // WXDC
                "cdp  p4, #0, c0, c0, c1, #0",     // ADD0
                "cdp  p4, #1, c0, c0, c1, #0",     // ADD1
                "cdp  p4, #8, c0, c0, c0, #2",     // NTDC
                "mrc  p4, #0, {rz}, c0, c3, #1",   // RDUC
                rxl = in(reg) a_lo,
                rxh = in(reg) a_hi,
                rz = out(reg) result,
                options(nomem, nostack),
            );
        }
        result
    }

    /// Convert `f32` to `i32` (round to nearest, ties to even).
    #[inline]
    pub fn float2int_r(&self, a: f32) -> i32 {
        let a_bits = a.to_bits();
        let result: u32;
        unsafe {
            core::arch::asm!(
                "mcrr p4, #9, {rx}, {rx}, c2",     // WXFC
                "cdp  p4, #0, c0, c0, c1, #0",     // ADD0
                "cdp  p4, #1, c0, c0, c1, #0",     // ADD1
                "cdp  p4, #8, c0, c0, c0, #3",     // NRDC — normalize & round double
                "mrc  p4, #0, {rz}, c0, c3, #0",   // RDIC
                rx = in(reg) a_bits,
                rz = out(reg) result,
                options(nomem, nostack),
            );
        }
        result as i32
    }

    /// Convert `f32` to `u32` (round to nearest, ties to even).
    #[inline]
    pub fn float2uint_r(&self, a: f32) -> u32 {
        let a_bits = a.to_bits();
        let result: u32;
        unsafe {
            core::arch::asm!(
                "mcrr p4, #9, {rx}, {rx}, c2",     // WXFC
                "cdp  p4, #0, c0, c0, c1, #0",     // ADD0
                "cdp  p4, #1, c0, c0, c1, #0",     // ADD1
                "cdp  p4, #8, c0, c0, c0, #3",     // NRDC
                "mrc  p4, #0, {rz}, c0, c3, #1",   // RDUC
                rx = in(reg) a_bits,
                rz = out(reg) result,
                options(nomem, nostack),
            );
        }
        result
    }

    /// Convert `f64` to `i32` (round to nearest, ties to even).
    #[inline]
    pub fn double2int_r(&self, a: f64) -> i32 {
        let (a_lo, a_hi) = f64_to_pair(a);
        let result: u32;
        unsafe {
            core::arch::asm!(
                "mcrr p4, #8, {rxl}, {rxh}, c0",   // WXDC
                "cdp  p4, #0, c0, c0, c1, #0",     // ADD0
                "cdp  p4, #1, c0, c0, c1, #0",     // ADD1
                "cdp  p4, #8, c0, c0, c0, #3",     // NRDC
                "mrc  p4, #0, {rz}, c0, c3, #0",   // RDIC
                rxl = in(reg) a_lo,
                rxh = in(reg) a_hi,
                rz = out(reg) result,
                options(nomem, nostack),
            );
        }
        result as i32
    }

    /// Convert `f64` to `u32` (round to nearest, ties to even).
    #[inline]
    pub fn double2uint_r(&self, a: f64) -> u32 {
        let (a_lo, a_hi) = f64_to_pair(a);
        let result: u32;
        unsafe {
            core::arch::asm!(
                "mcrr p4, #8, {rxl}, {rxh}, c0",   // WXDC
                "cdp  p4, #0, c0, c0, c1, #0",     // ADD0
                "cdp  p4, #1, c0, c0, c1, #0",     // ADD1
                "cdp  p4, #8, c0, c0, c0, #3",     // NRDC
                "mrc  p4, #0, {rz}, c0, c3, #1",   // RDUC
                rxl = in(reg) a_lo,
                rxh = in(reg) a_hi,
                rz = out(reg) result,
                options(nomem, nostack),
            );
        }
        result
    }
}

// ============================================================================
// Convenience: float comparison helpers
// ============================================================================

/// DCP comparison result bit meanings.
///
/// The result from [`Dcp::fcmp`] / [`Dcp::dcmp`] encodes the IEEE 754
/// comparison outcome using the ARM APSR bit layout:
///
/// | Bit | APSR | Meaning |
/// |-----|------|---------|
/// | 28 | V | Unordered — set if either operand is NaN |
/// | 29 | C | Greater-or-equal — set if a >= b |
/// | 30 | Z | Equal — set if a == b |
///
/// There is no dedicated "less-than" bit; LT is implied by C=0 (and V=0).
/// There is no dedicated "greater-than" bit; GT is C=1 and Z=0 (and V=0).
#[allow(dead_code)]
mod cmp_bits {
    /// Set if either operand is NaN (unordered).  Bit 28 = APSR V.
    pub(crate) const NAN: u32 = 1 << 28;
    /// Set if `a >= b`.  Bit 29 = APSR C.
    pub(crate) const GE: u32 = 1 << 29;
    /// Set if `a == b`.  Bit 30 = APSR Z.
    pub(crate) const EQUAL: u32 = 1 << 30;
}

impl Dcp {
    /// Test if `a == b` (false if either is NaN).
    #[inline]
    pub fn fcmp_eq(&self, a: f32, b: f32) -> bool {
        let r = self.fcmp(a, b);
        (r & cmp_bits::EQUAL) != 0 && (r & cmp_bits::NAN) == 0
    }

    /// Test if `a < b` (false if either is NaN).
    #[inline]
    pub fn fcmp_lt(&self, a: f32, b: f32) -> bool {
        let r = self.fcmp(a, b);
        // LT = not GE and not NaN
        (r & cmp_bits::GE) == 0 && (r & cmp_bits::NAN) == 0
    }

    /// Test if `a <= b` (false if either is NaN).
    #[inline]
    pub fn fcmp_le(&self, a: f32, b: f32) -> bool {
        let r = self.fcmp(a, b);
        // LE = not GT = not (GE && !EQ) = !GE || EQ, excluding NaN
        ((r & cmp_bits::GE) == 0 || (r & cmp_bits::EQUAL) != 0) && (r & cmp_bits::NAN) == 0
    }

    /// Test if `a > b` (false if either is NaN).
    #[inline]
    pub fn fcmp_gt(&self, a: f32, b: f32) -> bool {
        let r = self.fcmp(a, b);
        // GT = GE && !EQ && !NaN
        (r & cmp_bits::GE) != 0 && (r & cmp_bits::EQUAL) == 0 && (r & cmp_bits::NAN) == 0
    }

    /// Test if `a >= b` (false if either is NaN).
    #[inline]
    pub fn fcmp_ge(&self, a: f32, b: f32) -> bool {
        let r = self.fcmp(a, b);
        (r & cmp_bits::GE) != 0 && (r & cmp_bits::NAN) == 0
    }

    /// Test if either operand is NaN.
    #[inline]
    pub fn fcmp_unordered(&self, a: f32, b: f32) -> bool {
        let r = self.fcmp(a, b);
        (r & cmp_bits::NAN) != 0
    }

    /// Test if `a == b` (false if either is NaN).
    #[inline]
    pub fn dcmp_eq(&self, a: f64, b: f64) -> bool {
        let r = self.dcmp(a, b);
        (r & cmp_bits::EQUAL) != 0 && (r & cmp_bits::NAN) == 0
    }

    /// Test if `a < b` (false if either is NaN).
    #[inline]
    pub fn dcmp_lt(&self, a: f64, b: f64) -> bool {
        let r = self.dcmp(a, b);
        (r & cmp_bits::GE) == 0 && (r & cmp_bits::NAN) == 0
    }

    /// Test if `a <= b` (false if either is NaN).
    #[inline]
    pub fn dcmp_le(&self, a: f64, b: f64) -> bool {
        let r = self.dcmp(a, b);
        ((r & cmp_bits::GE) == 0 || (r & cmp_bits::EQUAL) != 0) && (r & cmp_bits::NAN) == 0
    }

    /// Test if `a > b` (false if either is NaN).
    #[inline]
    pub fn dcmp_gt(&self, a: f64, b: f64) -> bool {
        let r = self.dcmp(a, b);
        (r & cmp_bits::GE) != 0 && (r & cmp_bits::EQUAL) == 0 && (r & cmp_bits::NAN) == 0
    }

    /// Test if `a >= b` (false if either is NaN).
    #[inline]
    pub fn dcmp_ge(&self, a: f64, b: f64) -> bool {
        let r = self.dcmp(a, b);
        (r & cmp_bits::GE) != 0 && (r & cmp_bits::NAN) == 0
    }

    /// Test if either operand is NaN.
    #[inline]
    pub fn dcmp_unordered(&self, a: f64, b: f64) -> bool {
        let r = self.dcmp(a, b);
        (r & cmp_bits::NAN) != 0
    }
}
