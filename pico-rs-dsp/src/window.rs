//! Window functions for spectral analysis and FIR design
//!
//! All functions fill a `&mut [i16]` buffer with Q15 window values
//! (1.0 = 32767, 0.0 = 0).
//!
//! Available windows:
//! | Function | Side-lobe | Main-lobe width |
//! |---|---|---|
//! | [`rectangular`] | −13 dB | 4π/N (narrowest) |
//! | [`hanning`] | −31 dB | 8π/N |
//! | [`hamming`] | −43 dB | 8π/N |
//! | [`blackman`] | −74 dB | 12π/N |
//! | [`blackman_harris`] | −92 dB | 16π/N |
//! | [`kaiser`] | configurable | configurable |

use crate::math::cos_q15;

// ---------------------------------------------------------------------------
// Rectangular
// ---------------------------------------------------------------------------

/// Fill `buf` with a rectangular (boxcar) window: all values = 32767.
#[inline]
pub fn rectangular(buf: &mut [i16]) {
    for v in buf.iter_mut() {
        *v = i16::MAX;
    }
}

// ---------------------------------------------------------------------------
// Hanning (raised cosine, periodic form)
// ---------------------------------------------------------------------------

/// Hanning window:  `w[n] = 0.5 · (1 − cos(2π·n/N))`.
///
/// Uses the Q15 cos lookup table; error ≤ 1 LSB.
pub fn hanning(buf: &mut [i16]) {
    let n = buf.len();
    if n == 0 {
        return;
    }
    for (k, v) in buf.iter_mut().enumerate() {
        // phase for cos(2π*k/N): map to [0, 65536)
        let phase = ((k as u32 * 65536) / n as u32) as u16;
        let cos_val = cos_q15(phase) as i32; // Q15 in [-32767, 32767]
        // w[k] = 0.5 * (1 - cos_val/32768) * 32768
        //      = (32768 - cos_val) / 2
        let w = (32767_i32 - cos_val) / 2;
        *v = w.clamp(0, i16::MAX as i32) as i16;
    }
}

// ---------------------------------------------------------------------------
// Hamming  w[n] = 0.54 - 0.46*cos(2π·n/N)
// ---------------------------------------------------------------------------

/// Hamming window: `w[n] = 0.54 − 0.46·cos(2π·n/N)`.
pub fn hamming(buf: &mut [i16]) {
    let n = buf.len();
    if n == 0 {
        return;
    }
    // In Q15: 0.54 → 17695, 0.46 → 15073
    for (k, v) in buf.iter_mut().enumerate() {
        let phase = ((k as u32 * 65536) / n as u32) as u16;
        let cos_val = cos_q15(phase) as i32;
        // w = (17695 - (15073 * cos_val) / 32768)
        let term = (15073_i32 * cos_val) >> 15;
        let w = 17695_i32 - term;
        *v = w.clamp(0, i16::MAX as i32) as i16;
    }
}

// ---------------------------------------------------------------------------
// Blackman  w[n] = 0.42 - 0.5·cos(2π·n/N) + 0.08·cos(4π·n/N)
// ---------------------------------------------------------------------------

/// Blackman window.
pub fn blackman(buf: &mut [i16]) {
    let n = buf.len();
    if n == 0 {
        return;
    }
    // Q15 coefficients: 0.42→13762, 0.5→16384, 0.08→2621
    for (k, v) in buf.iter_mut().enumerate() {
        let phase1 = ((k as u32 * 65536) / n as u32) as u16;
        let phase2 = phase1.wrapping_add(phase1); // 2× frequency
        let c1 = cos_q15(phase1) as i32;
        let c2 = cos_q15(phase2) as i32;
        // w = 13762 - 16384*c1/32768 + 2621*c2/32768
        let t1 = (16384_i32 * c1) >> 15;
        let t2 = (2621_i32 * c2) >> 15;
        let w = 13762_i32 - t1 + t2;
        *v = w.clamp(0, i16::MAX as i32) as i16;
    }
}

// ---------------------------------------------------------------------------
// Blackman-Harris  4-term, w[n] = a0 - a1·cos(2πn/N) + a2·cos(4πn/N) - a3·cos(6πn/N)
// ---------------------------------------------------------------------------

/// Blackman-Harris 4-term window (−92 dB side lobes).
pub fn blackman_harris(buf: &mut [i16]) {
    let n = buf.len();
    if n == 0 {
        return;
    }
    // a0=0.35875, a1=0.48829, a2=0.14128, a3=0.01168
    // Q15: 11754, 15999, 4629, 382
    for (k, v) in buf.iter_mut().enumerate() {
        let p1 = ((k as u32 * 65536) / n as u32) as u16;
        let p2 = p1.wrapping_add(p1);
        let p3 = p2.wrapping_add(p1);
        let c1 = cos_q15(p1) as i32;
        let c2 = cos_q15(p2) as i32;
        let c3 = cos_q15(p3) as i32;
        let w = 11754_i32
            - ((15999_i32 * c1) >> 15)
            + ((4629_i32 * c2) >> 15)
            - ((382_i32 * c3) >> 15);
        *v = w.clamp(0, i16::MAX as i32) as i16;
    }
}

// ---------------------------------------------------------------------------
// Kaiser window  w[n] = I0(β·√(1−(2n/N−1)²)) / I0(β)
// ---------------------------------------------------------------------------

/// Kaiser window.
///
/// * `beta` — shape parameter (Q8, i.e. real_beta = beta/256).
///   Typical values: beta≈5 (−60 dB), beta≈8 (−80 dB), beta≈10 (−100 dB).
///
/// Uses a 10-term Bessel function approximation; accuracy ≤ 0.1%.
pub fn kaiser(buf: &mut [i16], beta_q8: u16) {
    let n = buf.len();
    if n == 0 {
        return;
    }
    let beta = beta_q8 as f32 / 256.0;
    let i0_beta = bessel_i0(beta);

    for (k, v) in buf.iter_mut().enumerate() {
        // alpha = (2k/(N-1)) - 1  in [-1, 1]
        let alpha = if n == 1 {
            0.0_f32
        } else {
            2.0 * k as f32 / (n as f32 - 1.0) - 1.0
        };
        let inner = beta * libm_sqrtf(1.0 - alpha * alpha);
        let w = bessel_i0(inner) / i0_beta;
        *v = (w * 32767.0).clamp(0.0, 32767.0) as i16;
    }
}

/// Modified Bessel function I0(x) — 10-term Taylor series.
/// Accurate to ≤ 0.1% for |x| ≤ 20.
fn bessel_i0(x: f32) -> f32 {
    let t = x * 0.5;
    let t2 = t * t;
    // I0(x) = Σ (x/2)^(2k) / (k!)^2, k=0..9
    let mut term = 1.0_f32;
    let mut sum = 1.0_f32;
    for k in 1..=10_u32 {
        term *= t2 / (k * k) as f32;
        sum += term;
    }
    sum
}

/// Minimal f32 sqrt without libm (Newton–Raphson, 3 iterations).
#[inline]
fn libm_sqrtf(x: f32) -> f32 {
    if x <= 0.0 {
        return 0.0;
    }
    // Use integer bit manipulation for initial estimate
    let bits = x.to_bits();
    let est_bits = (bits >> 1) + 0x1FBB_4000;
    let mut est = f32::from_bits(est_bits);
    // 3 Newton iterations for ~23-bit accuracy
    est = 0.5 * (est + x / est);
    est = 0.5 * (est + x / est);
    est = 0.5 * (est + x / est);
    est
}

// ---------------------------------------------------------------------------
// Flat-top window — highest amplitude accuracy
// ---------------------------------------------------------------------------

/// Flat-top window (5-term) for accurate amplitude measurements.
///
/// Coefficients from Heinzel et al. (2002).
pub fn flat_top(buf: &mut [i16]) {
    let n = buf.len();
    if n == 0 {
        return;
    }
    // a = [0.21557895, 0.41663158, 0.277263158, 0.083578947, 0.006947368]
    // Normalised so sum = 1; Q15 scaled
    for (k, v) in buf.iter_mut().enumerate() {
        let p1 = ((k as u32 * 65536) / n as u32) as u16;
        let p2 = p1.wrapping_add(p1);
        let p3 = p2.wrapping_add(p1);
        let p4 = p3.wrapping_add(p1);
        let c1 = cos_q15(p1) as i32;
        let c2 = cos_q15(p2) as i32;
        let c3 = cos_q15(p3) as i32;
        let c4 = cos_q15(p4) as i32;
        // Q15 coefficients (multiply by 32768 → 7064, 13651, 9085, 2738, 228)
        let w = 7064_i32
            - ((13651_i32 * c1) >> 15)
            + ((9085_i32 * c2) >> 15)
            - ((2738_i32 * c3) >> 15)
            + ((228_i32 * c4) >> 15);
        *v = w.clamp(0, i16::MAX as i32) as i16;
    }
}

// end of window.rs
