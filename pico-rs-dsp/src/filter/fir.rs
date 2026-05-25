//! FIR (Finite Impulse Response) digital filter
//!
//! # Usage
//! ```ignore
//! use pico_rs_dsp::filter::fir::FirFilter;
//!
//! // 8-tap low-pass FIR coefficients in Q15
//! static COEFFS: [i16; 8] = [...];
//! // State buffer (must be ≥ N taps, zero-initialised)
//! static mut STATE: [i16; 8] = [0; 8];
//!
//! let mut fir = FirFilter::<8>::new(&COEFFS, unsafe { &mut STATE });
//! let out = fir.process(sample);
//! ```
//!
//! With `target-cpu=cortex-m33` (or `target-feature=+dsp`), the inner loop
//! uses the SMLAD dual-MAC instruction for 2× throughput on Q15 filters.

// ---------------------------------------------------------------------------
// Q15 FIR — const-generic tap count
// ---------------------------------------------------------------------------

/// FIR filter operating on Q15 samples.
///
/// `N` is the number of taps (filter order + 1).
/// The coefficients and state buffer are stored externally so this struct
/// can live on the stack or in `.data` without heap allocation.
pub struct FirFilter<'a, const N: usize> {
    coeffs: &'a [i16; N],
    state: &'a mut [i16; N], // circular delay line
    pos: usize,              // write head
}

impl<'a, const N: usize> FirFilter<'a, N> {
    /// Create a new FIR filter.
    ///
    /// * `coeffs` — Q15 tap coefficients h[0]…h[N-1] (h[0] = most recent)
    /// * `state`  — zero-initialised delay-line buffer
    pub fn new(coeffs: &'a [i16; N], state: &'a mut [i16; N]) -> Self {
        FirFilter { coeffs, state, pos: 0 }
    }

    /// Process one sample and return the filtered output (Q15).
    ///
    /// Uses a circular buffer; each call is O(N).
    #[inline]
    pub fn process(&mut self, x: i16) -> i16 {
        // Write new sample
        self.state[self.pos] = x;
        let pos = self.pos;
        self.pos = if pos + 1 >= N { 0 } else { pos + 1 };

        self.accumulate(pos)
    }

    /// Reset the delay line to zero.
    pub fn reset(&mut self) {
        for s in self.state.iter_mut() {
            *s = 0;
        }
        self.pos = 0;
    }

    #[inline(always)]
    fn accumulate(&self, newest: usize) -> i16 {
        #[cfg(target_feature = "dsp")]
        {
            self.accumulate_dsp(newest)
        }
        #[cfg(not(target_feature = "dsp"))]
        {
            self.accumulate_soft(newest)
        }
    }

    fn accumulate_soft(&self, newest: usize) -> i16 {
        let mut acc: i32 = 0;
        let mut state_idx = newest;
        for k in 0..N {
            acc += (self.state[state_idx] as i32) * (self.coeffs[k] as i32);
            if state_idx == 0 {
                state_idx = N - 1;
            } else {
                state_idx -= 1;
            }
        }
        // Round and convert from Q30 back to Q15
        ((acc + 0x4000) >> 15).clamp(i16::MIN as i32, i16::MAX as i32) as i16
    }

    #[cfg(target_feature = "dsp")]
    fn accumulate_dsp(&self, newest: usize) -> i16 {
        use crate::intrinsics;
        // Process 2 taps at a time using SMLAD
        let mut acc: i32 = 0;
        let mut state_idx = newest;

        let pairs = N / 2;
        for k in 0..pairs {
            let s0 = self.state[state_idx] as i32;
            let si1 = if state_idx == 0 { N - 1 } else { state_idx - 1 };
            let s1 = self.state[si1] as i32;

            // Pack two samples: lo = s0 (newest), hi = s1
            let sv = s0 | (s1 << 16);
            // Pack two coefficients: lo = h[2k], hi = h[2k+1]
            let cv = (self.coeffs[2 * k] as i32) | ((self.coeffs[2 * k + 1] as i32) << 16);

            acc = intrinsics::smlad(sv, cv, acc);

            state_idx = if si1 == 0 { N - 1 } else { si1 - 1 };
        }
        // Handle odd tap
        if N % 2 == 1 {
            acc += (self.state[state_idx] as i32) * (self.coeffs[N - 1] as i32);
        }
        ((acc + 0x4000) >> 15).clamp(i16::MIN as i32, i16::MAX as i32) as i16
    }
}

// ---------------------------------------------------------------------------
// Block processing helpers
// ---------------------------------------------------------------------------

impl<'a, const N: usize> FirFilter<'a, N> {
    /// Process a block of samples in-place.
    pub fn process_block(&mut self, samples: &mut [i16]) {
        for s in samples.iter_mut() {
            *s = self.process(*s);
        }
    }

    /// Process a block of samples from `src` to `dst`.
    pub fn process_block_into(&mut self, src: &[i16], dst: &mut [i16]) {
        let n = src.len().min(dst.len());
        for i in 0..n {
            dst[i] = self.process(src[i]);
        }
    }
}

// ---------------------------------------------------------------------------
// Q31 FIR
// ---------------------------------------------------------------------------

/// FIR filter operating on Q31 samples.
pub struct FirFilter32<'a, const N: usize> {
    coeffs: &'a [i32; N],
    state: &'a mut [i32; N],
    pos: usize,
}

impl<'a, const N: usize> FirFilter32<'a, N> {
    pub fn new(coeffs: &'a [i32; N], state: &'a mut [i32; N]) -> Self {
        FirFilter32 { coeffs, state, pos: 0 }
    }

    #[inline]
    pub fn process(&mut self, x: i32) -> i32 {
        self.state[self.pos] = x;
        let pos = self.pos;
        self.pos = if pos + 1 >= N { 0 } else { pos + 1 };

        let mut acc: i64 = 0;
        let mut si = pos;
        for k in 0..N {
            acc += (self.state[si] as i64) * (self.coeffs[k] as i64);
            if si == 0 {
                si = N - 1;
            } else {
                si -= 1;
            }
        }
        // Result is Q62; shift back to Q31
        let rounded = acc.wrapping_add(0x4000_0000) >> 31;
        rounded.clamp(i32::MIN as i64, i32::MAX as i64) as i32
    }

    pub fn reset(&mut self) {
        for s in self.state.iter_mut() {
            *s = 0;
        }
        self.pos = 0;
    }
}

// ---------------------------------------------------------------------------
// Coefficient design utilities (window method)
// ---------------------------------------------------------------------------

/// Fill `coeffs` with a windowed-sinc low-pass FIR in Q15.
///
/// * `cutoff_q15` — normalised cutoff frequency in [0, 1) as Q15
///   (1.0 = Nyquist; e.g. 0.25 → cutoff at Fs/4)
/// * `window` — optional window slice of the same length (Q15, 1.0 = 32767);
///   pass `None` for a rectangular window
///
/// This is a design-time utility — call once to initialise static coefficients.
pub fn design_lowpass_q15(coeffs: &mut [i16], cutoff_q15: i16, window: Option<&[i16]>) {
    let n = coeffs.len();
    let fc = cutoff_q15 as f32 / 32768.0; // normalised [0,1)
    let half = (n as f32 - 1.0) / 2.0;

    for (k, c) in coeffs.iter_mut().enumerate() {
        let t = k as f32 - half;
        let sinc = if t == 0.0 {
            2.0 * fc
        } else {
            use core::f32::consts::PI;
            crate::math::sinf_rad(2.0 * PI * fc * t) / (PI * t)
        };
        let w = if let Some(win) = window {
            win.get(k).copied().unwrap_or(i16::MAX) as f32 / 32768.0
        } else {
            1.0
        };
        let v = sinc * w;
        *c = (v * 32768.0)
            .clamp(i16::MIN as f32, i16::MAX as f32) as i16;
    }
}
