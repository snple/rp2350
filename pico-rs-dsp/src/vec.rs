//! Vectorised slice DSP operations
//!
//! All functions operate on `&[i16]` / `&mut [i16]` (Q15) or
//! `&[i32]` / `&mut [i32]` (Q31) slices.
//!
//! When compiled with `target-cpu=cortex-m33` (or `target-feature=+dsp`),
//! inner loops use [`crate::intrinsics::smlad`] for 2× throughput.

// ---------------------------------------------------------------------------
// Q15 slice operations
// ---------------------------------------------------------------------------

/// Element-wise saturating add:  `dst[i] = a[i] + b[i]`.
///
/// `dst`, `a`, and `b` must have the same length; shorter slices are silently
/// truncated to `min(a.len(), b.len(), dst.len())`.
#[inline]
pub fn add_q15(dst: &mut [i16], a: &[i16], b: &[i16]) {
    let n = dst.len().min(a.len()).min(b.len());
    for i in 0..n {
        dst[i] = a[i].saturating_add(b[i]);
    }
}

/// Element-wise saturating subtract: `dst[i] = a[i] - b[i]`.
#[inline]
pub fn sub_q15(dst: &mut [i16], a: &[i16], b: &[i16]) {
    let n = dst.len().min(a.len()).min(b.len());
    for i in 0..n {
        dst[i] = a[i].saturating_sub(b[i]);
    }
}

/// Scale every element: `dst[i] = (src[i] * scale) >> 15` (Q15 multiply).
#[inline]
pub fn scale_q15(dst: &mut [i16], src: &[i16], scale: i16) {
    let n = dst.len().min(src.len());
    let scale32 = scale as i32;
    for i in 0..n {
        let product = (src[i] as i32) * scale32;
        dst[i] = ((product + 0x4000) >> 15).clamp(i16::MIN as i32, i16::MAX as i32) as i16;
    }
}

/// Dot product of two Q15 slices; accumulates into Q31.
///
/// Returns the raw i64 accumulator (not scaled back to Q15/Q31).
/// Caller divides by 2^15 to get a Q15 result, or 2^31 for Q31.
///
/// With `target-cpu=cortex-m33`: uses SMLAD to process 2 pairs per cycle.
#[inline]
pub fn dot_q15(a: &[i16], b: &[i16]) -> i64 {
    let n = a.len().min(b.len());

    #[cfg(target_feature = "dsp")]
    {
        use crate::intrinsics;
        let pairs = n / 2;
        let mut acc: i32 = 0;
        for i in 0..pairs {
            // Pack two i16 samples into one i32 (lo=first, hi=second)
            let av = (a[2 * i] as i32) | ((a[2 * i + 1] as i32) << 16);
            let bv = (b[2 * i] as i32) | ((b[2 * i + 1] as i32) << 16);
            acc = intrinsics::smlad(av, bv, acc);
        }
        let mut result = acc as i64;
        if n % 2 == 1 {
            result += (a[n - 1] as i64) * (b[n - 1] as i64);
        }
        result
    }
    #[cfg(not(target_feature = "dsp"))]
    {
        let mut acc: i64 = 0;
        for i in 0..n {
            acc += (a[i] as i64) * (b[i] as i64);
        }
        acc
    }
}

/// L2 norm squared: Σ x[i]² (returns i64).
#[inline]
pub fn norm_sq_q15(x: &[i16]) -> i64 {
    dot_q15(x, x)
}

/// RMS value of a Q15 slice.  Returns Q15.
/// Uses integer sqrt on the mean squared value.
#[inline]
pub fn rms_q15(x: &[i16]) -> i16 {
    if x.is_empty() {
        return 0;
    }
    let sum_sq = norm_sq_q15(x);
    let mean_sq = sum_sq / (x.len() as i64);
    // mean_sq is in units of (i16)^2, i.e. up to 32767^2 ≈ 10^9; fits in i32
    crate::math::sqrt_u32(mean_sq.clamp(0, i32::MAX as i64) as u32) as i16
}

/// Negate every element (saturating): `dst[i] = -src[i]`.
#[inline]
pub fn negate_q15(dst: &mut [i16], src: &[i16]) {
    let n = dst.len().min(src.len());
    for i in 0..n {
        dst[i] = src[i].saturating_neg();
    }
}

/// Element-wise absolute value: `dst[i] = |src[i]|` (saturating).
#[inline]
pub fn abs_q15(dst: &mut [i16], src: &[i16]) {
    let n = dst.len().min(src.len());
    for i in 0..n {
        dst[i] = src[i].saturating_abs();
    }
}

/// Copy and shift right (arithmetic, for gain reduction): `dst[i] = src[i] >> shift`.
#[inline]
pub fn shift_right_q15(dst: &mut [i16], src: &[i16], shift: u32) {
    let n = dst.len().min(src.len());
    for i in 0..n {
        dst[i] = (src[i] as i32 >> shift) as i16;
    }
}

/// Find the maximum absolute value in a Q15 slice.
#[inline]
pub fn max_abs_q15(x: &[i16]) -> i16 {
    x.iter()
        .copied()
        .map(|v| v.saturating_abs())
        .max()
        .unwrap_or(0)
}

/// Clip (saturate) every sample to the given peak: `|dst[i]| ≤ peak`.
#[inline]
pub fn clip_q15(dst: &mut [i16], src: &[i16], peak: i16) {
    let n = dst.len().min(src.len());
    let p = peak.abs();
    for i in 0..n {
        dst[i] = src[i].clamp(-p, p);
    }
}

// ---------------------------------------------------------------------------
// Q31 slice operations
// ---------------------------------------------------------------------------

/// Element-wise saturating add Q31.
#[inline]
pub fn add_q31(dst: &mut [i32], a: &[i32], b: &[i32]) {
    let n = dst.len().min(a.len()).min(b.len());
    for i in 0..n {
        dst[i] = a[i].saturating_add(b[i]);
    }
}

/// Element-wise saturating subtract Q31.
#[inline]
pub fn sub_q31(dst: &mut [i32], a: &[i32], b: &[i32]) {
    let n = dst.len().min(a.len()).min(b.len());
    for i in 0..n {
        dst[i] = a[i].saturating_sub(b[i]);
    }
}

/// Scale Q31: `dst[i] = (src[i] * scale) >> 31`.
#[inline]
pub fn scale_q31(dst: &mut [i32], src: &[i32], scale: i32) {
    let n = dst.len().min(src.len());
    let s = scale as i64;
    for i in 0..n {
        let product = (src[i] as i64) * s;
        let rounded = product.wrapping_add(0x4000_0000) >> 31;
        dst[i] = rounded.clamp(i32::MIN as i64, i32::MAX as i64) as i32;
    }
}

/// Dot product of two Q31 slices, returns i64 accumulator.
#[inline]
pub fn dot_q31(a: &[i32], b: &[i32]) -> i64 {
    let n = a.len().min(b.len());
    let mut acc: i64 = 0;
    for i in 0..n {
        acc = acc.saturating_add(((a[i] as i64) * (b[i] as i64)) >> 31);
    }
    acc
}

/// Interleave two Q15 slices into one: `[a0, b0, a1, b1, ...]`.
/// `dst` must have at least `2 * min(a.len(), b.len())` elements.
#[inline]
pub fn interleave_q15(dst: &mut [i16], a: &[i16], b: &[i16]) {
    let n = a.len().min(b.len());
    assert!(dst.len() >= 2 * n);
    for i in 0..n {
        dst[2 * i] = a[i];
        dst[2 * i + 1] = b[i];
    }
}

/// Deinterleave: `a[i] = src[2*i]`, `b[i] = src[2*i+1]`.
#[inline]
pub fn deinterleave_q15(a: &mut [i16], b: &mut [i16], src: &[i16]) {
    let n = (src.len() / 2).min(a.len()).min(b.len());
    for i in 0..n {
        a[i] = src[2 * i];
        b[i] = src[2 * i + 1];
    }
}
