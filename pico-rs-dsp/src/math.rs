//! Math utilities without `std` / `libm`
//!
//! All functions accept and return Q15 (`i16`, scale 2^15) or Q31 (`i32`,
//! scale 2^31) fixed-point values unless otherwise noted.
//!
//! # Provided functions
//! | Function | Input | Output | Notes |
//! |---|---|---|---|
//! | [`sin_q15`] | angle Q15 [0, 2π) mapped to [0, 0xFFFF) | Q15 | 256-entry table + linear interp |
//! | [`cos_q15`] | same | Q15 | delegates to `sin_q15` with phase offset |
//! | [`sin_cos_q15`] | same | (Q15, Q15) | both at once |
//! | [`sqrt_q15`] | Q15 magnitude (unsigned, 0–1) | Q15 | Newton–Raphson |
//! | [`sqrt_u32`] | unsigned 32-bit integer | u16 | integer square root |
//! | [`atan2_q15`] | (y, x) both Q15 | Q15 angle | CORDIC-style |
//! | [`abs_q15`] | Q15 | Q15 (≥ 0) | saturating |

// ---------------------------------------------------------------------------
// 256-entry sin table, Q15 format.
// Covers one full period (256 entries = 2π).
// Generated offline: round(sin(2π·k/256) * 32768) for k = 0..255
// ---------------------------------------------------------------------------

static SIN_TABLE_Q15: [i16; 256] = [
    0, 804, 1608, 2410, 3212, 4011, 4808, 5602, 6393, 7179, 7962, 8739, 9512, 10278, 11039, 11793,
    12539, 13279, 14010, 14732, 15446, 16151, 16846, 17530, 18204, 18868, 19519, 20159, 20787,
    21403, 22005, 22594, 23170, 23731, 24279, 24811, 25329, 25832, 26319, 26790, 27245, 27683,
    28105, 28510, 28898, 29268, 29621, 29956, 30273, 30571, 30852, 31113, 31356, 31580, 31785,
    31971, 32137, 32285, 32412, 32521, 32609, 32678, 32728, 32757, 32767, 32757, 32728, 32678,
    32609, 32521, 32412, 32285, 32137, 31971, 31785, 31580, 31356, 31113, 30852, 30571, 30273,
    29956, 29621, 29268, 28898, 28510, 28105, 27683, 27245, 26790, 26319, 25832, 25329, 24811,
    24279, 23731, 23170, 22594, 22005, 21403, 20787, 20159, 19519, 18868, 18204, 17530, 16846,
    16151, 15446, 14732, 14010, 13279, 12539, 11793, 11039, 10278, 9512, 8739, 7962, 7179, 6393,
    5602, 4808, 4011, 3212, 2410, 1608, 804, 0, -804, -1608, -2410, -3212, -4011, -4808, -5602,
    -6393, -7179, -7962, -8739, -9512, -10278, -11039, -11793, -12539, -13279, -14010, -14732,
    -15446, -16151, -16846, -17530, -18204, -18868, -19519, -20159, -20787, -21403, -22005, -22594,
    -23170, -23731, -24279, -24811, -25329, -25832, -26319, -26790, -27245, -27683, -28105, -28510,
    -28898, -29268, -29621, -29956, -30273, -30571, -30852, -31113, -31356, -31580, -31785, -31971,
    -32137, -32285, -32412, -32521, -32609, -32678, -32728, -32757, -32767, -32757, -32728, -32678,
    -32609, -32521, -32412, -32285, -32137, -31971, -31785, -31580, -31356, -31113, -30852, -30571,
    -30273, -29956, -29621, -29268, -28898, -28510, -28105, -27683, -27245, -26790, -26319, -25832,
    -25329, -24811, -24279, -23731, -23170, -22594, -22005, -21403, -20787, -20159, -19519, -18868,
    -18204, -17530, -16846, -16151, -15446, -14732, -14010, -13279, -12539, -11793, -11039, -10278,
    -9512, -8739, -7962, -7179, -6393, -5602, -4808, -4011, -3212, -2410, -1608, -804,
];

// ---------------------------------------------------------------------------
// sin / cos
// ---------------------------------------------------------------------------

/// Sine of an angle given as a phase in [0, 65536) (full period = 65536).
///
/// Returns Q15 in the range [-32767, 32767].
/// Phase wraps naturally via u16 arithmetic.
///
/// # Example
/// ```ignore
/// // 90° = phase 16384
/// let s = sin_q15(16384_u16); // ≈ 32767
/// ```
#[inline]
pub fn sin_q15(phase: u16) -> i16 {
    // Top 8 bits: table index; bottom 8 bits: interpolation fraction
    let idx = (phase >> 8) as usize; // 0..255
    let frac = (phase & 0xFF) as i32; // 0..255

    let y0 = SIN_TABLE_Q15[idx] as i32;
    let y1 = SIN_TABLE_Q15[(idx + 1) & 0xFF] as i32;
    // Linear interpolation: y0 + (y1 - y0) * frac / 256
    let result = y0 + (((y1 - y0) * frac) >> 8);
    result as i16
}

/// Cosine — same as `sin_q15(phase + 16384)`.
#[inline]
pub fn cos_q15(phase: u16) -> i16 {
    sin_q15(phase.wrapping_add(0x4000))
}

/// Compute both sine and cosine in one call.
#[inline]
pub fn sin_cos_q15(phase: u16) -> (i16, i16) {
    (sin_q15(phase), cos_q15(phase))
}

// ---------------------------------------------------------------------------
// Integer square root (Newton–Raphson)
// ---------------------------------------------------------------------------

/// Integer square root of a 32-bit unsigned number.
///
/// Returns floor(√x). Uses a fast Newton–Raphson iteration.
#[inline]
pub fn sqrt_u32(x: u32) -> u32 {
    if x == 0 {
        return 0;
    }
    // Initial guess: leading bit position / 2
    let mut est = 1_u32 << ((32 - x.leading_zeros()) / 2);
    // Up to 5 Newton–Raphson iterations (converges for 32-bit in ≤5 steps)
    est = (est + x / est) >> 1;
    est = (est + x / est) >> 1;
    est = (est + x / est) >> 1;
    est = (est + x / est) >> 1;
    est = (est + x / est) >> 1;
    // Ensure floor
    if est * est > x { est - 1 } else { est }
}

/// Q15 square root: input `x` ∈ [0, 1) in Q15 (i.e. i16 in [0, 32767]).
/// Returns Q15 value of √x in the same range.
#[inline]
pub fn sqrt_q15(x: i16) -> i16 {
    if x <= 0 {
        return 0;
    }
    // Scale x up to 32-bit, compute sqrt, scale back
    // x_q15 = x / 32768.0
    // sqrt(x_q15) * 32768 = sqrt(x * 32768)
    let scaled = (x as u32) << 15; // x * 32768
    sqrt_u32(scaled) as i16
}

/// Q31 square root: input `x` ∈ [0, 1) in Q31.
/// Returns Q31 value of √x.
#[inline]
pub fn sqrt_q31(x: i32) -> i32 {
    if x <= 0 {
        return 0;
    }
    let scaled = (x as u64) << 31;
    let r = sqrt_u64(scaled);
    r.min(i32::MAX as u64) as i32
}

/// 64-bit integer square root (Newton–Raphson).
#[inline]
pub fn sqrt_u64(x: u64) -> u64 {
    if x == 0 {
        return 0;
    }
    let leading = x.leading_zeros();
    let mut est = 1_u64 << ((64 - leading) / 2);
    for _ in 0..6 {
        est = (est + x / est) >> 1;
    }
    if est * est > x { est - 1 } else { est }
}

// ---------------------------------------------------------------------------
// atan2 — CORDIC-based, Q15 output
// ---------------------------------------------------------------------------

/// Four-quadrant arctangent.
///
/// Inputs `y` and `x` are Q15 values (i16, representing numbers in [-1, 1)).
/// Returns angle in the range [0, 65536) (full circle = 65536 units).
/// 0 = 0°, 16384 = 90°, 32768 = 180°, 49152 = 270°.
///
/// Uses a 16-iteration CORDIC algorithm (~15 bits of accuracy).
#[inline]
pub fn atan2_phase(y: i16, x: i16) -> u16 {
    // Precomputed CORDIC atan table in the same 0–65536 angle units
    // atan(2^-i) * 65536 / (2π)
    const ATAN_TABLE: [u16; 16] = [
        8192, 4836, 2555, 1297, 651, 326, 163, 81, 41, 20, 10, 5, 2, 1, 0, 0,
    ];

    let mut xn = x as i32;
    let mut yn = y as i32;
    let mut angle: i32 = 0;

    // Rotate into right half-plane, remembering the quadrant
    if xn < 0 {
        if yn >= 0 {
            // 2nd quadrant → rotate -90° first, result in Q2
            let tmp = xn;
            xn = yn;
            yn = -tmp;
            angle = 16384; // 90°
        } else {
            // 3rd quadrant → rotate +90°
            let tmp = xn;
            xn = -yn;
            yn = tmp;
            angle = -16384_i32; // -90° (will wrap to 270°)
        }
    }

    for i in 0..16 {
        let d = if yn < 0 { -1_i32 } else { 1_i32 };
        let xn_new = xn - d * (yn >> i);
        let yn_new = yn + d * (xn >> i);
        xn = xn_new;
        yn = yn_new;
        angle += d * ATAN_TABLE[i] as i32;
    }

    angle as u16
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Compute the magnitude |z| = sqrt(re² + im²) with Q15 inputs.
///
/// Uses the fast approximation: max + 0.4375 * min (Lyons method),
/// which has < 4% error compared to true magnitude.
#[inline]
pub fn magnitude_approx_q15(re: i16, im: i16) -> i16 {
    let a = (re as i32).abs();
    let b = (im as i32).abs();
    let (major, minor) = if a > b { (a, b) } else { (b, a) };
    // ≈ major + 0.4375 * minor  (0.4375 = 7/16)
    let result = major + ((minor * 7) >> 4);
    result.min(i16::MAX as i32) as i16
}

/// True Q15 magnitude using integer sqrt.
#[inline]
pub fn magnitude_q15(re: i16, im: i16) -> i16 {
    let a = re as i32;
    let b = im as i32;
    // sum is at most 2 * 32767^2 < 2^31, safe in u32
    let sq_sum = (a * a + b * b) as u32;
    // sqrt(sq_sum) in Q15: sqrt(re_q15² + im_q15²) where each is scaled by 32768
    // => result is in same scale / 32768
    let raw = sqrt_u32(sq_sum);
    (raw >> 8).min(i16::MAX as u32) as i16
}

// ---------------------------------------------------------------------------
// f32 wrappers using Q15 lookup table (for no_std environments)
// ---------------------------------------------------------------------------

/// Convert an angle in radians to the 0..65536 phase used by the lookup table.
#[inline]
fn rad_to_phase(x: f32) -> u16 {
    // 1 full period (2π) = 65536 phase units
    const SCALE: f32 = 65536.0 / (2.0 * core::f32::consts::PI);
    // Cast to i32 first: wraps negative angles correctly, then to u16
    (x * SCALE) as i32 as u16
}

/// `sin(x)` — valid for any angle (radians), uses Q15 table.
/// Accuracy ≈ 15 bits (limited by the 256-entry table + linear interp).
#[inline]
pub fn sinf_rad(x: f32) -> f32 {
    sin_q15(rad_to_phase(x)) as f32 / 32768.0
}

/// `cos(x)` — valid for any angle (radians), uses Q15 table.
#[inline]
pub fn cosf_rad(x: f32) -> f32 {
    cos_q15(rad_to_phase(x)) as f32 / 32768.0
}

/// `tan(x)` — computed as sin(x)/cos(x), saturates near ±π/2.
#[inline]
pub fn tanf_rad(x: f32) -> f32 {
    let s = sinf_rad(x);
    let c = cosf_rad(x);
    // Guard against near-zero denominator
    if c.abs() < 3.1e-5 {
        if s >= 0.0 { 32767.0_f32 } else { -32767.0_f32 }
    } else {
        s / c
    }
}
