//! In-place fixed-point FFT / IFFT — Radix-2 Decimation-In-Time (DIT)
//!
//! Operates on complex Q15 samples stored as interleaved `[re, im, re, im, …]`
//! in a `&mut [i16]` buffer of length `2N` (N must be a power of two).
//!
//! The twiddle factors are read from a 256-entry sine/cosine lookup table
//! (the same table used by [`crate::math`]).
//!
//! # Scaling
//! Each butterfly divides by 2 to prevent overflow, giving a total scale
//! factor of `1/N`. Multiply outputs by `N` (or shift left by `log2(N)`)
//! to recover true amplitudes.
//!
//! # Example
//! ```ignore
//! let mut buf = [0_i16; 64]; // 32-point complex FFT
//! // fill buf with interleaved (re, im) pairs …
//! pico_rs_dsp::fft::fft(&mut buf);
//! // buf now contains frequency-domain data scaled by 1/32
//! ```

use crate::math::{cos_q15, sin_q15};

/// In-place FFT.
///
/// `buf` must have length `2 * N` where `N` is a power of two, `N ≥ 2`.
/// Data layout: `[re0, im0, re1, im1, …, re(N-1), im(N-1)]`.
///
/// Panics (debug) if `buf.len()` is not `2 * power_of_two`.
pub fn fft(buf: &mut [i16]) {
    let two_n = buf.len();
    debug_assert!(two_n >= 4 && two_n.is_power_of_two());
    let n = two_n / 2;
    bit_reverse_permute(buf, n);
    butterfly_passes(buf, n, false);
}

/// In-place IFFT.
///
/// Same constraints as [`fft`]. Result is scaled by `1/N`.
pub fn ifft(buf: &mut [i16]) {
    let two_n = buf.len();
    debug_assert!(two_n >= 4 && two_n.is_power_of_two());
    let n = two_n / 2;
    bit_reverse_permute(buf, n);
    butterfly_passes(buf, n, true);
}

// ---------------------------------------------------------------------------
// Bit-reversal permutation
// ---------------------------------------------------------------------------

fn bit_reverse_permute(buf: &mut [i16], n: usize) {
    let bits = n.trailing_zeros() as usize;
    for i in 0..n {
        let j = bit_reverse(i, bits);
        if j > i {
            buf.swap(2 * i, 2 * j);
            buf.swap(2 * i + 1, 2 * j + 1);
        }
    }
}

#[inline]
fn bit_reverse(mut x: usize, bits: usize) -> usize {
    let mut result = 0_usize;
    for _ in 0..bits {
        result = (result << 1) | (x & 1);
        x >>= 1;
    }
    result
}

// ---------------------------------------------------------------------------
// Butterfly passes (Cooley-Tukey DIT)
// ---------------------------------------------------------------------------

fn butterfly_passes(buf: &mut [i16], n: usize, inverse: bool) {
    let mut len = 2; // current DFT sub-problem size
    while len <= n {
        let half = len / 2;
        // Twiddle step: phase increment per butterfly pair (full period = 65536)
        // W_len^k = exp(-j 2π k / len) → phase step = 65536 / len
        let phase_step: u32 = 65536 / len as u32;

        let mut k = 0;
        while k < n {
            let mut phase: u16 = 0;
            for j in 0..half {
                let u_re = buf[2 * (k + j)] as i32;
                let u_im = buf[2 * (k + j) + 1] as i32;

                let tw_re = cos_q15(phase) as i32;
                let tw_im = if inverse {
                    sin_q15(phase) as i32 // conjugate twiddle for IFFT
                } else {
                    -(sin_q15(phase) as i32)
                };

                let v_re = buf[2 * (k + j + half)] as i32;
                let v_im = buf[2 * (k + j + half) + 1] as i32;

                // Complex multiply (V * W), result in Q30
                // Then butterfly: U ± (V*W), divide by 2 for each stage
                let tvr = (v_re * tw_re - v_im * tw_im) >> 15;
                let tvi = (v_re * tw_im + v_im * tw_re) >> 15;

                // Divide by 2 at each stage to prevent overflow
                buf[2 * (k + j)]     = ((u_re + tvr) >> 1) as i16;
                buf[2 * (k + j) + 1] = ((u_im + tvi) >> 1) as i16;
                buf[2 * (k + j + half)]     = ((u_re - tvr) >> 1) as i16;
                buf[2 * (k + j + half) + 1] = ((u_im - tvi) >> 1) as i16;

                phase = phase.wrapping_add((phase_step * j as u32) as u16);
            }
            k += len;
        }
        len <<= 1;
    }
}

// ---------------------------------------------------------------------------
// Power spectrum helpers
// ---------------------------------------------------------------------------

/// Compute power spectrum from FFT output.
///
/// `fft_buf` is an in-place FFT result (length `2N`).
/// `power` receives `N/2 + 1` magnitude-squared values (DC to Nyquist).
/// Each value is in arbitrary units proportional to signal power.
pub fn power_spectrum(fft_buf: &[i16], power: &mut [u32]) {
    let n = fft_buf.len() / 2;
    let bins = (n / 2 + 1).min(power.len());
    for k in 0..bins {
        let re = fft_buf[2 * k] as i32;
        let im = fft_buf[2 * k + 1] as i32;
        power[k] = (re * re + im * im) as u32;
    }
}

/// Find the bin index with maximum power (dominant frequency bin).
pub fn peak_bin(power: &[u32]) -> usize {
    power
        .iter()
        .enumerate()
        .max_by_key(|&(_, &v)| v)
        .map(|(i, _)| i)
        .unwrap_or(0)
}

// ---------------------------------------------------------------------------
// Real FFT helper: pack N real samples into N/2 complex, run FFT, unpack
// ---------------------------------------------------------------------------

/// In-place real FFT.
///
/// `buf` must have length `2N` where N is a power of two. Pack the N real
/// samples into `buf[0], buf[2], buf[4], …` (even indices), leaving odd
/// indices zero, then call this function. It runs an N-point complex FFT and
/// the result occupies the full buffer.
///
/// This is a convenience wrapper; a proper split-radix real FFT would be
/// ~2× faster, but this is sufficient for most embedded use cases.
pub fn rfft(buf: &mut [i16]) {
    // Simply treat as complex (imaginary parts were zeroed by caller)
    fft(buf);
}
