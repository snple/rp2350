//! Q-format fixed-point arithmetic
//!
//! | Type    | Underlying | Range           | Resolution        |
//! |---------|-----------|-----------------|-------------------|
//! | [`Q15`] | `i16`     | [-1, 1)         | 2^-15 ≈ 3.05e-5   |
//! | [`Q31`] | `i32`     | [-1, 1)         | 2^-31 ≈ 4.66e-10  |
//! | [`Q16`] | `i32`     | [-32768, 32768)  | 2^-16 ≈ 1.53e-5   |
//!
//! All operations saturate rather than wrap.

use core::ops::{Add, Mul, Neg, Sub};

// ---------------------------------------------------------------------------
// Q15  (1 sign bit + 15 fractional bits, stored as i16)
// ---------------------------------------------------------------------------

/// 1.15 fixed-point value in the range [-1, 1).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
#[repr(transparent)]
pub struct Q15(pub i16);

impl Q15 {
    pub const ZERO: Q15 = Q15(0);
    pub const ONE: Q15 = Q15(i16::MAX); // ≈ 0.99997
    pub const NEG_ONE: Q15 = Q15(i16::MIN);
    pub const MAX: Q15 = Q15(i16::MAX);
    pub const MIN: Q15 = Q15(i16::MIN);

    /// Construct from a raw `i16` bit pattern.
    #[inline]
    pub const fn from_bits(bits: i16) -> Self {
        Q15(bits)
    }

    /// Return the raw `i16` bit pattern.
    #[inline]
    pub const fn to_bits(self) -> i16 {
        self.0
    }

    /// Convert a floating-point value (clipped to [-1, 1]) to Q15.
    #[inline]
    pub fn from_f32(x: f32) -> Self {
        let clamped = if x >= 1.0 {
            32767_i16
        } else if x <= -1.0 {
            i16::MIN
        } else {
            (x * 32768.0) as i16
        };
        Q15(clamped)
    }

    /// Convert Q15 to `f32`.
    #[inline]
    pub fn to_f32(self) -> f32 {
        self.0 as f32 / 32768.0
    }

    /// Saturating addition.
    #[inline]
    pub fn sat_add(self, rhs: Q15) -> Q15 {
        Q15(self.0.saturating_add(rhs.0))
    }

    /// Saturating subtraction.
    #[inline]
    pub fn sat_sub(self, rhs: Q15) -> Q15 {
        Q15(self.0.saturating_sub(rhs.0))
    }

    /// Q15 × Q15 → Q15  (with rounding, saturating).
    #[inline]
    pub fn mul_q15(self, rhs: Q15) -> Q15 {
        // Result of multiplying two Q15s is Q1.30; shift right 15 with rounding.
        let product = (self.0 as i32) * (rhs.0 as i32);
        // Round: add 0x4000 before shifting
        let rounded = product.wrapping_add(0x4000) >> 15;
        // Saturate to i16 range
        let sat = rounded.clamp(i16::MIN as i32, i16::MAX as i32) as i16;
        Q15(sat)
    }

    /// Negate (saturating: -MIN stays at MAX to avoid overflow).
    #[inline]
    pub fn sat_neg(self) -> Q15 {
        Q15(self.0.saturating_neg())
    }

    /// Absolute value (saturating).
    #[inline]
    pub fn abs(self) -> Q15 {
        Q15(self.0.saturating_abs())
    }
}

impl Add for Q15 {
    type Output = Q15;
    #[inline]
    fn add(self, rhs: Q15) -> Q15 {
        self.sat_add(rhs)
    }
}

impl Sub for Q15 {
    type Output = Q15;
    #[inline]
    fn sub(self, rhs: Q15) -> Q15 {
        self.sat_sub(rhs)
    }
}

impl Mul for Q15 {
    type Output = Q15;
    #[inline]
    fn mul(self, rhs: Q15) -> Q15 {
        self.mul_q15(rhs)
    }
}

impl Neg for Q15 {
    type Output = Q15;
    #[inline]
    fn neg(self) -> Q15 {
        self.sat_neg()
    }
}

// ---------------------------------------------------------------------------
// Q31  (1 sign bit + 31 fractional bits, stored as i32)
// ---------------------------------------------------------------------------

/// 1.31 fixed-point value in the range [-1, 1).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
#[repr(transparent)]
pub struct Q31(pub i32);

impl Q31 {
    pub const ZERO: Q31 = Q31(0);
    pub const ONE: Q31 = Q31(i32::MAX); // ≈ 1.0
    pub const NEG_ONE: Q31 = Q31(i32::MIN);
    pub const MAX: Q31 = Q31(i32::MAX);
    pub const MIN: Q31 = Q31(i32::MIN);

    #[inline]
    pub const fn from_bits(bits: i32) -> Self {
        Q31(bits)
    }

    #[inline]
    pub const fn to_bits(self) -> i32 {
        self.0
    }

    /// Convert floating-point to Q31 (clamped to [-1, 1]).
    #[inline]
    pub fn from_f32(x: f32) -> Self {
        let clamped = if x >= 1.0 {
            i32::MAX
        } else if x <= -1.0 {
            i32::MIN
        } else {
            (x * 2147483648.0_f32) as i32
        };
        Q31(clamped)
    }

    #[inline]
    pub fn to_f32(self) -> f32 {
        self.0 as f32 / 2147483648.0_f32
    }

    #[inline]
    pub fn sat_add(self, rhs: Q31) -> Q31 {
        Q31(self.0.saturating_add(rhs.0))
    }

    #[inline]
    pub fn sat_sub(self, rhs: Q31) -> Q31 {
        Q31(self.0.saturating_sub(rhs.0))
    }

    /// Q31 × Q31 → Q31 (with rounding, saturating).
    #[inline]
    pub fn mul_q31(self, rhs: Q31) -> Q31 {
        let product = (self.0 as i64) * (rhs.0 as i64);
        let rounded = product.wrapping_add(0x4000_0000) >> 31;
        let sat = rounded.clamp(i32::MIN as i64, i32::MAX as i64) as i32;
        Q31(sat)
    }

    #[inline]
    pub fn sat_neg(self) -> Q31 {
        Q31(self.0.saturating_neg())
    }

    #[inline]
    pub fn abs(self) -> Q31 {
        Q31(self.0.saturating_abs())
    }

    /// Promote to higher precision Q31 from Q15.
    #[inline]
    pub fn from_q15(q: Q15) -> Q31 {
        Q31((q.0 as i32) << 16)
    }

    /// Downconvert to Q15 with rounding.
    #[inline]
    pub fn to_q15(self) -> Q15 {
        let rounded = self.0.wrapping_add(0x8000) >> 16;
        Q15(rounded.clamp(i16::MIN as i32, i16::MAX as i32) as i16)
    }
}

impl Add for Q31 {
    type Output = Q31;
    #[inline]
    fn add(self, rhs: Q31) -> Q31 {
        self.sat_add(rhs)
    }
}

impl Sub for Q31 {
    type Output = Q31;
    #[inline]
    fn sub(self, rhs: Q31) -> Q31 {
        self.sat_sub(rhs)
    }
}

impl Mul for Q31 {
    type Output = Q31;
    #[inline]
    fn mul(self, rhs: Q31) -> Q31 {
        self.mul_q31(rhs)
    }
}

impl Neg for Q31 {
    type Output = Q31;
    #[inline]
    fn neg(self) -> Q31 {
        self.sat_neg()
    }
}

// ---------------------------------------------------------------------------
// Q16  (16 integer bits + 16 fractional bits, stored as i32) — Q16.16
// ---------------------------------------------------------------------------

/// 16.16 signed fixed-point in the range [-32768, 32768).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
#[repr(transparent)]
pub struct Q16(pub i32);

impl Q16 {
    pub const ZERO: Q16 = Q16(0);
    pub const ONE: Q16 = Q16(1 << 16);
    pub const NEG_ONE: Q16 = Q16(-1 << 16);
    pub const MAX: Q16 = Q16(i32::MAX);
    pub const MIN: Q16 = Q16(i32::MIN);

    #[inline]
    pub const fn from_bits(bits: i32) -> Self {
        Q16(bits)
    }

    #[inline]
    pub const fn to_bits(self) -> i32 {
        self.0
    }

    #[inline]
    pub fn from_f32(x: f32) -> Self {
        let v = (x * 65536.0_f32) as i64;
        Q16(v.clamp(i32::MIN as i64, i32::MAX as i64) as i32)
    }

    #[inline]
    pub fn to_f32(self) -> f32 {
        self.0 as f32 / 65536.0_f32
    }

    #[inline]
    pub fn from_i16(x: i16) -> Self {
        Q16((x as i32) << 16)
    }

    /// Truncate fractional part and return integer component.
    #[inline]
    pub fn to_i16(self) -> i16 {
        (self.0 >> 16) as i16
    }

    #[inline]
    pub fn sat_add(self, rhs: Q16) -> Q16 {
        Q16(self.0.saturating_add(rhs.0))
    }

    #[inline]
    pub fn sat_sub(self, rhs: Q16) -> Q16 {
        Q16(self.0.saturating_sub(rhs.0))
    }

    /// Q16.16 × Q16.16 → Q16.16 (saturating, rounded).
    #[inline]
    pub fn mul_q16(self, rhs: Q16) -> Q16 {
        let product = (self.0 as i64) * (rhs.0 as i64);
        let rounded = product.wrapping_add(0x8000) >> 16;
        Q16(rounded.clamp(i32::MIN as i64, i32::MAX as i64) as i32)
    }

    #[inline]
    pub fn sat_neg(self) -> Q16 {
        Q16(self.0.saturating_neg())
    }

    #[inline]
    pub fn abs(self) -> Q16 {
        Q16(self.0.saturating_abs())
    }
}

impl Add for Q16 {
    type Output = Q16;
    #[inline]
    fn add(self, rhs: Q16) -> Q16 {
        self.sat_add(rhs)
    }
}

impl Sub for Q16 {
    type Output = Q16;
    #[inline]
    fn sub(self, rhs: Q16) -> Q16 {
        self.sat_sub(rhs)
    }
}

impl Mul for Q16 {
    type Output = Q16;
    #[inline]
    fn mul(self, rhs: Q16) -> Q16 {
        self.mul_q16(rhs)
    }
}

impl Neg for Q16 {
    type Output = Q16;
    #[inline]
    fn neg(self) -> Q16 {
        self.sat_neg()
    }
}

// ---------------------------------------------------------------------------
// Conversion between formats
// ---------------------------------------------------------------------------

impl From<Q15> for Q31 {
    #[inline]
    fn from(q: Q15) -> Q31 {
        Q31::from_q15(q)
    }
}

impl From<Q31> for Q15 {
    #[inline]
    fn from(q: Q31) -> Q15 {
        q.to_q15()
    }
}
