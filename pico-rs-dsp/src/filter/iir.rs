//! IIR Biquad (second-order section) digital filter
//!
//! A biquad implements the Direct Form I difference equation:
//!
//! ```text
//! y[n] = b0*x[n] + b1*x[n-1] + b2*x[n-2]
//!               - a1*y[n-1] - a2*y[n-2]
//! ```
//!
//! Coefficients are stored Q15 (`i16`); state and accumulation use Q31 (`i32`)
//! for sufficient headroom.
//!
//! Multiple biquads can be chained by calling `process` in sequence (cascade).
//!
//! # Coefficient encoding
//! All coefficients are normalised so that `a0 = 1`. Feedback coefficients
//! `a1` and `a2` are stored **negated** so the accumulation is always addition.

// ---------------------------------------------------------------------------
// Biquad coefficients
// ---------------------------------------------------------------------------

/// Biquad filter coefficients (Q15).
///
/// The `a` coefficients are stored **as given** (not negated). The filter
/// implementation handles the sign internally.
#[derive(Clone, Copy, Debug, Default)]
pub struct BiquadCoeffs {
    pub b0: i16,
    pub b1: i16,
    pub b2: i16,
    /// a1 (denominator, normalised a0=1). Stored positive; subtracted in loop.
    pub a1: i16,
    /// a2 (denominator, normalised a0=1). Stored positive; subtracted in loop.
    pub a2: i16,
}

impl BiquadCoeffs {
    /// Design a 2nd-order Butterworth low-pass filter.
    ///
    /// * `fc_norm` — normalised cutoff in (0, 0.5) (0.5 = Nyquist)
    ///
    /// Returns coefficients in Q15. This uses `f32` computation and is
    /// intended for design-time use (call once at startup or `const` context).
    pub fn butterworth_lowpass(fc_norm: f32) -> Self {
        use core::f32::consts::PI;
        // Bilinear transform of 1st-order Butterworth (cascaded twice gives 2nd)
        // Full 2nd-order direct design:
        let k = crate::math::tanf_rad(PI * fc_norm);
        let k2 = k * k;
        let norm = 1.0 / (1.0 + core::f32::consts::SQRT_2 * k + k2);
        let b0 = k2 * norm;
        let b1 = 2.0 * b0;
        let b2 = b0;
        let a1 = 2.0 * (k2 - 1.0) * norm;
        let a2 = (1.0 - core::f32::consts::SQRT_2 * k + k2) * norm;

        fn to_q15(x: f32) -> i16 {
            (x * 32768.0).clamp(i16::MIN as f32, i16::MAX as f32) as i16
        }

        BiquadCoeffs { b0: to_q15(b0), b1: to_q15(b1), b2: to_q15(b2),
                       a1: to_q15(a1), a2: to_q15(a2) }
    }

    /// Design a 2nd-order Butterworth high-pass filter.
    pub fn butterworth_highpass(fc_norm: f32) -> Self {
        use core::f32::consts::PI;
        let k = crate::math::tanf_rad(PI * fc_norm);
        let k2 = k * k;
        let norm = 1.0 / (1.0 + core::f32::consts::SQRT_2 * k + k2);
        let b0 = norm;
        let b1 = -2.0 * norm;
        let b2 = norm;
        let a1 = 2.0 * (k2 - 1.0) * norm;
        let a2 = (1.0 - core::f32::consts::SQRT_2 * k + k2) * norm;

        fn to_q15(x: f32) -> i16 {
            (x * 32768.0).clamp(i16::MIN as f32, i16::MAX as f32) as i16
        }
        BiquadCoeffs { b0: to_q15(b0), b1: to_q15(b1), b2: to_q15(b2),
                       a1: to_q15(a1), a2: to_q15(a2) }
    }

    /// Design a 2nd-order band-pass filter (peak at `fc_norm`, Q factor `q`).
    pub fn bandpass(fc_norm: f32, q: f32) -> Self {
        use core::f32::consts::PI;
        let k = crate::math::tanf_rad(PI * fc_norm);
        let norm = 1.0 / (1.0 + k / q + k * k);
        let b0 = k / q * norm;
        let b1 = 0.0_f32;
        let b2 = -b0;
        let a1 = 2.0 * (k * k - 1.0) * norm;
        let a2 = (1.0 - k / q + k * k) * norm;

        fn to_q15(x: f32) -> i16 {
            (x * 32768.0).clamp(i16::MIN as f32, i16::MAX as f32) as i16
        }
        BiquadCoeffs { b0: to_q15(b0), b1: to_q15(b1), b2: to_q15(b2),
                       a1: to_q15(a1), a2: to_q15(a2) }
    }

    /// Design a 2nd-order notch (band-reject) filter.
    pub fn notch(fc_norm: f32, q: f32) -> Self {
        use core::f32::consts::PI;
        let k = crate::math::tanf_rad(PI * fc_norm);
        let k2 = k * k;
        let norm = 1.0 / (1.0 + k / q + k2);
        let b0 = (1.0 + k2) * norm;
        let b1 = 2.0 * (k2 - 1.0) * norm;
        let b2 = b0;
        let a1 = b1;
        let a2 = (1.0 - k / q + k2) * norm;

        fn to_q15(x: f32) -> i16 {
            (x * 32768.0).clamp(i16::MIN as f32, i16::MAX as f32) as i16
        }
        BiquadCoeffs { b0: to_q15(b0), b1: to_q15(b1), b2: to_q15(b2),
                       a1: to_q15(a1), a2: to_q15(a2) }
    }
}

// ---------------------------------------------------------------------------
// Biquad filter state & processing
// ---------------------------------------------------------------------------

/// Direct Form I biquad filter state.
///
/// State uses Q31 (`i32`) internally for precision.
#[derive(Clone, Copy, Debug, Default)]
pub struct BiquadState {
    pub x1: i32, // x[n-1]
    pub x2: i32, // x[n-2]
    pub y1: i32, // y[n-1]
    pub y2: i32, // y[n-2]
}

/// A single biquad section.
pub struct Biquad {
    pub coeffs: BiquadCoeffs,
    pub state: BiquadState,
}

impl Biquad {
    pub const fn new(coeffs: BiquadCoeffs) -> Self {
        Biquad { coeffs, state: BiquadState { x1: 0, x2: 0, y1: 0, y2: 0 } }
    }

    pub fn reset(&mut self) {
        self.state = BiquadState::default();
    }

    /// Process one Q15 sample and return Q15 output.
    ///
    /// Accumulation is in Q30 (i32 is sufficient for 5 Q15 products).
    #[inline]
    pub fn process(&mut self, x: i16) -> i16 {
        let x32 = x as i32;
        let b0 = self.coeffs.b0 as i32;
        let b1 = self.coeffs.b1 as i32;
        let b2 = self.coeffs.b2 as i32;
        let a1 = self.coeffs.a1 as i32;
        let a2 = self.coeffs.a2 as i32;

        // Each term is at most 32767*32767 ≈ 1e9 < 2^30; five terms fit in i64
        let mut acc: i64 = (x32 * b0) as i64;
        acc += (self.state.x1 * b1) as i64;
        acc += (self.state.x2 * b2) as i64;
        acc -= (self.state.y1 * a1) as i64;
        acc -= (self.state.y2 * a2) as i64;

        // Round from Q30 back to Q15
        let y = ((acc + 0x4000) >> 15)
            .clamp(i16::MIN as i64, i16::MAX as i64) as i32;

        // Shift state
        self.state.x2 = self.state.x1;
        self.state.x1 = x32;
        self.state.y2 = self.state.y1;
        self.state.y1 = y;

        y as i16
    }

    /// Block processing.
    pub fn process_block(&mut self, buf: &mut [i16]) {
        for s in buf.iter_mut() {
            *s = self.process(*s);
        }
    }
}

// ---------------------------------------------------------------------------
// Cascaded biquad bank
// ---------------------------------------------------------------------------

/// A cascade of up to `S` biquad sections (Series SOS filter).
///
/// Useful for higher-order filters decomposed into second-order sections.
pub struct BiquadCascade<const S: usize> {
    stages: [Biquad; S],
}

impl<const S: usize> BiquadCascade<S> {
    /// Construct from an array of `S` biquad sections.
    pub fn new(stages: [Biquad; S]) -> Self {
        BiquadCascade { stages }
    }

    /// Process one sample through all stages.
    #[inline]
    pub fn process(&mut self, x: i16) -> i16 {
        let mut y = x;
        for stage in self.stages.iter_mut() {
            y = stage.process(y);
        }
        y
    }

    pub fn reset(&mut self) {
        for stage in self.stages.iter_mut() {
            stage.reset();
        }
    }
}
