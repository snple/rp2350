//! Discrete-time PID controller
//!
//! Implements a standard velocity-form (incremental) and position-form PID
//! with anti-windup, derivative filter, and output clamping.
//!
//! All arithmetic uses `i32` / `i64` in Q16.16 format to avoid `f32`.
//!
//! # Example
//! ```ignore
//! use pico_rs_dsp::pid::{Pid, PidConfig};
//!
//! let cfg = PidConfig {
//!     kp: Q16::from_f32(1.0),
//!     ki: Q16::from_f32(0.01),
//!     kd: Q16::from_f32(0.1),
//!     tau: Q16::from_f32(0.02), // derivative LPF time constant (≥ sample period)
//!     out_min: Q16::from_i16(-1000),
//!     out_max: Q16::from_i16(1000),
//! };
//! let mut pid = Pid::new(cfg);
//! let output = pid.update(setpoint, measurement);
//! ```

use crate::fixed::Q16;

// ---------------------------------------------------------------------------
// Configuration
// ---------------------------------------------------------------------------

/// PID configuration (all fields in Q16.16).
#[derive(Clone, Copy, Debug)]
pub struct PidConfig {
    /// Proportional gain.
    pub kp: Q16,
    /// Integral gain. Set to Q16::ZERO to disable.
    pub ki: Q16,
    /// Derivative gain. Set to Q16::ZERO to disable.
    pub kd: Q16,
    /// Derivative low-pass filter time constant τ (sample-period units).
    /// Set equal to the sample period to disable filtering (τ ≤ sample_time).
    /// Larger τ → smoother derivative.
    pub tau: Q16,
    /// Minimum output (for clamping and anti-windup).
    pub out_min: Q16,
    /// Maximum output.
    pub out_max: Q16,
}

impl PidConfig {
    /// Proportional-only controller.
    pub const fn p_only(kp: Q16, out_min: Q16, out_max: Q16) -> Self {
        PidConfig {
            kp,
            ki: Q16::ZERO,
            kd: Q16::ZERO,
            tau: Q16::ONE,
            out_min,
            out_max,
        }
    }
}

// ---------------------------------------------------------------------------
// PID state
// ---------------------------------------------------------------------------

/// PID controller state.
pub struct Pid {
    cfg: PidConfig,
    integrator: i64,       // Q32 accumulator (Q16 × Q16 product)
    prev_error: i32,       // Q16
    differentiator: i64,   // Q32 derivative filtered value
    prev_measurement: i32, // Q16 (for derivative-on-measurement variant)
}

impl Pid {
    /// Create a new PID from the given configuration.
    pub const fn new(cfg: PidConfig) -> Self {
        Pid {
            cfg,
            integrator: 0,
            prev_error: 0,
            differentiator: 0,
            prev_measurement: 0,
        }
    }

    /// Reset the integrator, differentiator, and previous state.
    pub fn reset(&mut self) {
        self.integrator = 0;
        self.prev_error = 0;
        self.differentiator = 0;
        self.prev_measurement = 0;
    }

    /// Update the PID given `setpoint` and `measurement` (both Q16.16).
    ///
    /// Returns the controller output clamped to [out_min, out_max].
    ///
    /// Call at a fixed sample rate equal to the period implied by `ki` and `kd`.
    pub fn update(&mut self, setpoint: Q16, measurement: Q16) -> Q16 {
        let error = setpoint.0 as i64 - measurement.0 as i64;

        // ---- Proportional ----
        let p = mul_q16_i64(self.cfg.kp.0, error);

        // ---- Integral (trapezoidal, with anti-windup clamping) ----
        // integrator += ki * (error + prev_error) / 2
        let ki = self.cfg.ki.0 as i64;
        let i_delta = ki * (error + self.prev_error as i64) / 2;
        let new_integrator = self.integrator + i_delta;

        // ---- Derivative-on-measurement (avoids derivative kick on setpoint change) ----
        // differentiator = tau * prev_diff + kd * (meas - prev_meas)
        // Using a first-order LPF:  diff = (2τ-T)/(2τ+T) * prev + 2kd/(2τ+T) * Δmeas
        // Here T=1 (normalised), so coefficients become:
        // alpha = (2τ - 1) / (2τ + 1)   (Q16)
        // beta  = 2·kd / (2τ + 1)        (Q16)
        let tau2 = self.cfg.tau.0 as i64 * 2; // 2τ in Q16
        let denom = tau2 + 65536; // 2τ + 1 (where 1 = Q16::ONE = 65536)
        let alpha_num = tau2 - 65536; // may be negative for small τ

        let delta_meas = measurement.0 as i64 - self.prev_measurement as i64;
        let kd = self.cfg.kd.0 as i64;
        let beta_num = 2 * kd;

        let d_part =
            (alpha_num * self.differentiator + beta_num * (-delta_meas) * 65536) / (denom * 65536);

        // ---- Sum ----
        let out_q16_i64 = (p + new_integrator + d_part) >> 16;
        let out_min = self.cfg.out_min.0 as i64;
        let out_max = self.cfg.out_max.0 as i64;

        let clamped = out_q16_i64.clamp(out_min, out_max);
        let saturated = clamped != out_q16_i64;

        // Anti-windup: only accumulate integrator when output is not saturated
        if !saturated {
            self.integrator = new_integrator;
        } else {
            // Conditional integration: allow integrator to move only if it reduces error
            let would_reduce = (clamped > out_max && i_delta < 0)
                || (clamped < out_min && i_delta > 0)
                || !saturated;
            if would_reduce {
                self.integrator = new_integrator;
            }
        }

        self.differentiator = d_part;
        self.prev_error = error as i32;
        self.prev_measurement = measurement.0;

        Q16(clamped as i32)
    }

    /// Set integrator directly (e.g. bumpless transfer on mode switch).
    pub fn set_integrator(&mut self, val: Q16) {
        self.integrator = (val.0 as i64) << 16;
    }

    /// Read current integral contribution (Q16).
    pub fn integrator_q16(&self) -> Q16 {
        Q16((self.integrator >> 16) as i32)
    }
}

// ---------------------------------------------------------------------------
// Helper: Q16 * i64(Q16-scaled) → i64(Q32)
// ---------------------------------------------------------------------------

#[inline]
fn mul_q16_i64(gain: i32, val: i64) -> i64 {
    // gain is Q16 (scale 2^16), val is Q16 (scale 2^16) → product is Q32
    gain as i64 * val // keep as Q32, divide later when summing
}

// ---------------------------------------------------------------------------
// Velocity-form (incremental) PID
// ---------------------------------------------------------------------------

/// Velocity-form PID: outputs Δu (change in output) rather than absolute u.
///
/// Suitable for actuators that accumulate position (stepper motors, integrating
/// processes). The external integrator then accumulates Δu.
pub struct IncrementalPid {
    cfg: PidConfig,
    prev_error: i32,
    prev_prev_error: i32,
}

impl IncrementalPid {
    pub const fn new(cfg: PidConfig) -> Self {
        IncrementalPid {
            cfg,
            prev_error: 0,
            prev_prev_error: 0,
        }
    }

    pub fn reset(&mut self) {
        self.prev_error = 0;
        self.prev_prev_error = 0;
    }

    /// Returns Δu (Q16) — the increment to add to the output register.
    pub fn update(&mut self, setpoint: Q16, measurement: Q16) -> Q16 {
        let error = (setpoint.0 - measurement.0) as i64;
        let e1 = self.prev_error as i64;
        let e2 = self.prev_prev_error as i64;

        let kp = self.cfg.kp.0 as i64;
        let ki = self.cfg.ki.0 as i64;
        let kd = self.cfg.kd.0 as i64;

        // Δu = Kp*(e-e1) + Ki*e + Kd*(e - 2*e1 + e2)
        let delta = (kp * (error - e1) + ki * error + kd * (error - 2 * e1 + e2)) >> 16;
        let clamped = delta.clamp(self.cfg.out_min.0 as i64, self.cfg.out_max.0 as i64);

        self.prev_prev_error = self.prev_error;
        self.prev_error = error as i32;

        Q16(clamped as i32)
    }
}
