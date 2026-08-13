//! Hardware Interpolator (INTERP) driver for RP2350
//!
//! Each core has two interpolators (INTERP0 and INTERP1) that accelerate
//! fixed-point linear interpolation in hardware. Each interpolator has two
//! lanes (lane 0 and lane 1) that can operate independently or together.
//!
//! # Data Path
//!
//! Each lane computes per cycle:
//!
//! ```text
//! raw     = (accumulator >>> SHIFT) & mask[MASK_LSB..MASK_MSB]
//! result  = BASE + sign_extend(raw)
//! ```
//!
//! The key feature is the **POP** operation: reading a lane result
//! automatically writes both lane results back to the accumulators,
//! enabling iterative computation without CPU intervention.
//!
//! # Special Modes
//!
//! - **INTERP0 lane 0**: BLEND mode — linear interpolation between BASE0 and
//!   BASE1, controlled by lane 1's low 8 bits.
//! - **INTERP1 lane 0**: CLAMP mode — result is clamped to [BASE0, BASE1].
//!
//! # RP2350 vs RP2040
//!
//! On RP2350 the SHIFT operation is a **right rotate**. On RP2040 it is a
//! logical right shift.
//!
//! # Usage
//!
//! ```ignore
//! use rp2350::interp::{Interp0, Config};
//!
//! let interp0 = Interp0::init();
//!
//! // Configure lane 0: no shift, all bits pass, signed
//! let mut cfg = Config::new();
//! cfg.set_shift(0).set_mask(0, 31).set_signed(true);
//! interp0.lane0().configure(&cfg);
//!
//! // Set up a linear ramp
//! interp0.set_base(0, 0);
//! interp0.set_accumulator(0, 0);
//!
//! // Each POP advances via result feedback
//! let v0 = interp0.pop_lane(0); // → 0
//! let v1 = interp0.pop_lane(0); // → 1
//! ```

use crate::Steal;

// ============================================================================
// Interp0 / Interp1 handles
// ============================================================================

/// Handle to INTERP0 (interpolator 0) on the current core.
///
/// INTERP0 lane 0 supports **BLEND** mode for hardware linear interpolation.
///
/// Obtain a handle through [`Interp0::init()`] (main context) or
/// [`Interp0::steal()`] (ISR context).
pub struct Interp0 {
    _private: (),
}

/// Handle to INTERP1 (interpolator 1) on the current core.
///
/// INTERP1 lane 0 supports **CLAMP** mode for hardware value clamping.
///
/// Obtain a handle through [`Interp1::init()`] (main context) or
/// [`Interp1::steal()`] (ISR context).
pub struct Interp1 {
    _private: (),
}

impl Interp0 {
    /// Create an INTERP0 handle.
    ///
    /// The interpolator is part of SIO and requires no reset sequence.
    /// This constructor is safe.
    #[inline]
    pub fn init() -> Self {
        Interp0 { _private: () }
    }

    /// Obtain an INTERP0 handle without runtime checks.
    ///
    /// # Safety
    ///
    /// The caller must ensure no other code concurrently accesses the same
    /// interpolator registers. Typically called once at startup before
    /// interrupts are enabled.
    #[inline]
    pub unsafe fn steal() -> Steal<Interp0> {
        Steal(Interp0::init())
    }
    #[inline]
    pub fn lane0(&self) -> Lane {
        Lane { idx: 0 }
    }
    #[inline]
    pub fn lane1(&self) -> Lane {
        Lane { idx: 1 }
    }
}

impl Interp1 {
    /// Create an INTERP1 handle.
    ///
    /// The interpolator is part of SIO and requires no reset sequence.
    /// This constructor is safe.
    #[inline]
    pub fn init() -> Self {
        Interp1 { _private: () }
    }

    /// Obtain an INTERP1 handle without runtime checks.
    ///
    /// # Safety
    ///
    /// The caller must ensure no other code concurrently accesses the same
    /// interpolator registers. Typically called once at startup before
    /// interrupts are enabled.
    #[inline]
    pub unsafe fn steal() -> Steal<Interp1> {
        Steal(Interp1::init())
    }
    #[inline]
    pub fn lane0(&self) -> Lane {
        Lane { idx: 0 }
    }
    #[inline]
    pub fn lane1(&self) -> Lane {
        Lane { idx: 1 }
    }
}

// ============================================================================
// Trait for shared Interp0/Interp1 dispatch
// ============================================================================

pub trait InterpIndex {
    fn index() -> usize;
}

impl InterpIndex for Interp0 {
    #[inline]
    fn index() -> usize {
        0
    }
}
impl InterpIndex for Interp1 {
    #[inline]
    fn index() -> usize {
        1
    }
}

// ============================================================================
// Macro: dispatch to the correct PAC register for (interp_idx, lane)
// ============================================================================

/// Dispatches (interp, lane) → correct PAC register field and runs the
/// expression `$expr` with the register bound to `reg`.
///
/// Each PAC register type (Interp0Accum0, Interp0Accum1, Interp1Accum0, …)
/// is a distinct Rust type, but they all implement `Deref<Target =
/// VolatileCell<u32>>` so `reg.read_bits()` / `reg.write_bits(val)` works
/// uniformly.
macro_rules! with_interp_reg {
    // Accumulator registers (2 interps × 2 lanes = 4 variants)
    ($idx:expr, $lane:expr, accum, $reg:ident, $expr:expr) => {{
        let r = unsafe { &*crate::sio::REGS };
        debug_assert!(matches!($idx, 0 | 1), "interp index must be 0 or 1");
        debug_assert!(matches!($lane, 0 | 1), "interp lane must be 0 or 1");
        match ($idx, $lane) {
            (0, 0) => {
                let $reg = &r.interp0_accum0;
                $expr
            }
            (0, 1) => {
                let $reg = &r.interp0_accum1;
                $expr
            }
            (1, 0) => {
                let $reg = &r.interp1_accum0;
                $expr
            }
            _ => {
                let $reg = &r.interp1_accum1;
                $expr
            }
        }
    }};
    // Base registers (2 interps × 3 bases = 6 variants)
    ($idx:expr, $base:expr, base, $reg:ident, $expr:expr) => {{
        let r = unsafe { &*crate::sio::REGS };
        debug_assert!(matches!($idx, 0 | 1), "interp index must be 0 or 1");
        debug_assert!(matches!($base, 0..=2), "interp base must be 0..=2");
        match ($idx, $base) {
            (0, 0) => {
                let $reg = &r.interp0_base0;
                $expr
            }
            (0, 1) => {
                let $reg = &r.interp0_base1;
                $expr
            }
            (0, _) => {
                let $reg = &r.interp0_base2;
                $expr
            }
            (1, 0) => {
                let $reg = &r.interp1_base0;
                $expr
            }
            (1, 1) => {
                let $reg = &r.interp1_base1;
                $expr
            }
            _ => {
                let $reg = &r.interp1_base2;
                $expr
            }
        }
    }};
    // Pop lane (2 interps × 2 lanes = 4)
    ($idx:expr, $lane:expr, pop_lane, $reg:ident, $expr:expr) => {{
        let r = unsafe { &*crate::sio::REGS };
        debug_assert!(matches!($idx, 0 | 1), "interp index must be 0 or 1");
        debug_assert!(matches!($lane, 0 | 1), "interp lane must be 0 or 1");
        match ($idx, $lane) {
            (0, 0) => {
                let $reg = &r.interp0_pop_lane0;
                $expr
            }
            (0, 1) => {
                let $reg = &r.interp0_pop_lane1;
                $expr
            }
            (1, 0) => {
                let $reg = &r.interp1_pop_lane0;
                $expr
            }
            _ => {
                let $reg = &r.interp1_pop_lane1;
                $expr
            }
        }
    }};
    // Peek lane (2 interps × 2 lanes = 4)
    ($idx:expr, $lane:expr, peek_lane, $reg:ident, $expr:expr) => {{
        let r = unsafe { &*crate::sio::REGS };
        debug_assert!(matches!($idx, 0 | 1), "interp index must be 0 or 1");
        debug_assert!(matches!($lane, 0 | 1), "interp lane must be 0 or 1");
        match ($idx, $lane) {
            (0, 0) => {
                let $reg = &r.interp0_peek_lane0;
                $expr
            }
            (0, 1) => {
                let $reg = &r.interp0_peek_lane1;
                $expr
            }
            (1, 0) => {
                let $reg = &r.interp1_peek_lane0;
                $expr
            }
            _ => {
                let $reg = &r.interp1_peek_lane1;
                $expr
            }
        }
    }};
    // Pop full (2 interps)
    ($idx:expr, pop_full, $reg:ident, $expr:expr) => {{
        let r = unsafe { &*crate::sio::REGS };
        debug_assert!(matches!($idx, 0 | 1), "interp index must be 0 or 1");
        match $idx {
            0 => {
                let $reg = &r.interp0_pop_full;
                $expr
            }
            _ => {
                let $reg = &r.interp1_pop_full;
                $expr
            }
        }
    }};
    // Peek full (2 interps)
    ($idx:expr, peek_full, $reg:ident, $expr:expr) => {{
        let r = unsafe { &*crate::sio::REGS };
        debug_assert!(matches!($idx, 0 | 1), "interp index must be 0 or 1");
        match $idx {
            0 => {
                let $reg = &r.interp0_peek_full;
                $expr
            }
            _ => {
                let $reg = &r.interp1_peek_full;
                $expr
            }
        }
    }};
    // Control registers (2 interps × 2 lanes = 4)
    ($idx:expr, $lane:expr, ctrl, $reg:ident, $expr:expr) => {{
        let r = unsafe { &*crate::sio::REGS };
        debug_assert!(matches!($idx, 0 | 1), "interp index must be 0 or 1");
        debug_assert!(matches!($lane, 0 | 1), "interp lane must be 0 or 1");
        match ($idx, $lane) {
            (0, 0) => {
                let $reg = &r.interp0_ctrl_lane0;
                $expr
            }
            (0, 1) => {
                let $reg = &r.interp0_ctrl_lane1;
                $expr
            }
            (1, 0) => {
                let $reg = &r.interp1_ctrl_lane0;
                $expr
            }
            _ => {
                let $reg = &r.interp1_ctrl_lane1;
                $expr
            }
        }
    }};
    // Add raw registers (2 interps × 2 lanes = 4)
    ($idx:expr, $lane:expr, add_raw, $reg:ident, $expr:expr) => {{
        let r = unsafe { &*crate::sio::REGS };
        debug_assert!(matches!($idx, 0 | 1), "interp index must be 0 or 1");
        debug_assert!(matches!($lane, 0 | 1), "interp lane must be 0 or 1");
        match ($idx, $lane) {
            (0, 0) => {
                let $reg = &r.interp0_accum0_add;
                $expr
            }
            (0, 1) => {
                let $reg = &r.interp0_accum1_add;
                $expr
            }
            (1, 0) => {
                let $reg = &r.interp1_accum0_add;
                $expr
            }
            _ => {
                let $reg = &r.interp1_accum1_add;
                $expr
            }
        }
    }};
}

// ============================================================================
// Common interpolator operations (Interp0 / Interp1)
// ============================================================================

/// Operations common to both INTERP0 and INTERP1.
pub trait InterpOps: InterpIndex {
    /// Set the accumulator for a lane (0 or 1).
    #[inline]
    fn set_accumulator(&self, lane: usize, value: u32) {
        with_interp_reg!(Self::index(), lane, accum, reg, reg.write_bits(value));
    }

    /// Get the current accumulator value for a lane.
    #[inline]
    fn accumulator(&self, lane: usize) -> u32 {
        with_interp_reg!(Self::index(), lane, accum, reg, reg.read_bits())
    }

    /// Set a base register (0=BASE0, 1=BASE1, 2=BASE2).
    #[inline]
    fn set_base(&self, base_idx: usize, value: u32) {
        with_interp_reg!(Self::index(), base_idx, base, reg, reg.write_bits(value));
    }

    /// Get the current value of a base register.
    #[inline]
    fn base(&self, base_idx: usize) -> u32 {
        with_interp_reg!(Self::index(), base_idx, base, reg, reg.read_bits())
    }

    /// Set BASE0 (lower 16 bits) and BASE1 (upper 16 bits) simultaneously.
    /// Each half is sign-extended to 32 bits if that lane's SIGNED flag is set.
    #[inline]
    fn set_base_both(&self, value: u32) {
        let r = unsafe { &*crate::sio::REGS };
        if Self::index() == 0 {
            r.interp0_base_1and0.write_bits(value);
        } else {
            r.interp1_base_1and0.write_bits(value);
        }
    }

    /// Pop lane result — reads the result and feeds it back to both accumulators.
    #[inline]
    fn pop_lane(&self, lane: usize) -> u32 {
        with_interp_reg!(Self::index(), lane, pop_lane, reg, reg.read_bits())
    }

    /// Peek lane result — reads without altering internal state.
    #[inline]
    fn peek_lane(&self, lane: usize) -> u32 {
        with_interp_reg!(Self::index(), lane, peek_lane, reg, reg.read_bits())
    }

    /// Pop FULL result: `BASE2 + lane0_raw + lane1_raw`, with feedback.
    #[inline]
    fn pop_full(&self) -> u32 {
        with_interp_reg!(Self::index(), pop_full, reg, reg.read_bits())
    }

    /// Peek FULL result: read without feedback.
    #[inline]
    fn peek_full(&self) -> u32 {
        with_interp_reg!(Self::index(), peek_full, reg, reg.read_bits())
    }

    /// Atomically add a value to a lane's accumulator.
    #[inline]
    fn add_accumulator(&self, lane: usize, value: u32) {
        with_interp_reg!(Self::index(), lane, add_raw, reg, reg.write_bits(value));
    }

    /// Get the raw shift+mask value for a lane (BASE not added).
    #[inline]
    fn raw(&self, lane: usize) -> u32 {
        with_interp_reg!(Self::index(), lane, add_raw, reg, reg.read_bits())
    }

    /// Save the full interpolator state for later restore.
    #[inline]
    fn save(&self) -> State {
        State {
            accum: [self.accumulator(0), self.accumulator(1)],
            base: [self.base(0), self.base(1), self.base(2)],
            ctrl: [
                with_interp_reg!(Self::index(), 0, ctrl, reg, reg.read_bits()),
                with_interp_reg!(Self::index(), 1, ctrl, reg, reg.read_bits()),
            ],
        }
    }

    /// Restore previously saved state.
    #[inline]
    fn restore(&self, state: &State) {
        self.set_accumulator(0, state.accum[0]);
        self.set_accumulator(1, state.accum[1]);
        self.set_base(0, state.base[0]);
        self.set_base(1, state.base[1]);
        self.set_base(2, state.base[2]);
        with_interp_reg!(Self::index(), 0, ctrl, reg, reg.write_bits(state.ctrl[0]));
        with_interp_reg!(Self::index(), 1, ctrl, reg, reg.write_bits(state.ctrl[1]));
    }

    /// Set the FORCE_MSB bits for a lane (ORed into result bits 29:28 on bus).
    #[inline]
    fn set_force_bits(&self, lane: usize, bits: u8) {
        assert!(bits <= 3);
        const MASK: u32 = rp2350_pac::sio::Interp0CtrlLane0::FORCE_MSB_MASK;
        const OFFSET: u32 = rp2350_pac::sio::Interp0CtrlLane0::FORCE_MSB_OFFSET;
        with_interp_reg!(Self::index(), lane, ctrl, reg, {
            let val = reg.read_bits();
            reg.write_bits((val & !MASK) | (((bits as u32) << OFFSET) & MASK));
        });
    }
}

impl InterpOps for Interp0 {}
impl InterpOps for Interp1 {}

// ============================================================================
// Saved state
// ============================================================================

/// Snapshot of an interpolator's complete register state.
#[derive(Clone, Debug)]
pub struct State {
    pub accum: [u32; 2],
    pub base: [u32; 3],
    pub ctrl: [u32; 2],
}

// ============================================================================
// Lane-level operations
// ============================================================================

/// Accessor for a single lane of an interpolator.
pub struct Lane {
    idx: usize,
}

impl Lane {
    #[inline]
    pub fn index(&self) -> usize {
        self.idx
    }

    /// Apply configuration to this lane on the given interpolator.
    ///
    /// # Panics
    ///
    /// Panics if BLEND is set on a non-INTERP0 lane 0, or CLAMP is set on a
    /// non-INTERP1 lane 0.
    #[inline]
    pub fn configure<I: InterpIndex>(&self, _interp: &I, config: &Config) {
        let idx = I::index();
        if config.ctrl & rp2350_pac::sio::Interp0CtrlLane0::BLEND != 0 {
            assert!(
                idx == 0 && self.idx == 0,
                "BLEND only valid on INTERP0 lane 0"
            );
        }
        if config.ctrl & rp2350_pac::sio::Interp1CtrlLane0::CLAMP != 0 {
            assert!(
                idx == 1 && self.idx == 0,
                "CLAMP only valid on INTERP1 lane 0"
            );
        }
        with_interp_reg!(idx, self.idx, ctrl, reg, reg.write_bits(config.ctrl));
    }

    /// Read the current control register value.
    #[inline]
    pub fn read_ctrl<I: InterpIndex>(&self, _interp: &I) -> u32 {
        with_interp_reg!(I::index(), self.idx, ctrl, reg, reg.read_bits())
    }
}

// ============================================================================
// Configuration
// ============================================================================

/// Interpolator lane configuration.
///
/// Build with [`Config::new()`], then apply with [`Lane::configure()`].
#[derive(Clone, Debug)]
pub struct Config {
    ctrl: u32,
}

impl Config {
    /// Create a default configuration (all 32 bits pass through, no shift).
    #[inline]
    pub fn new() -> Self {
        let mut c = Config { ctrl: 0 };
        c.set_mask(0, 31);
        c
    }

    /// Set the right-rotate shift applied to the accumulator before masking.
    /// RP2350 uses right **rotate** (not logical shift like RP2040).
    #[inline]
    pub fn set_shift(&mut self, shift: u8) -> &mut Self {
        assert!(shift < 32);
        const M: u32 = rp2350_pac::sio::Interp0CtrlLane0::SHIFT_MASK;
        const O: u32 = rp2350_pac::sio::Interp0CtrlLane0::SHIFT_OFFSET;
        self.ctrl = (self.ctrl & !M) | (((shift as u32) << O) & M);
        self
    }

    #[inline]
    pub fn set_mask(&mut self, mask_lsb: u8, mask_msb: u8) -> &mut Self {
        assert!(mask_msb < 32 && mask_lsb <= mask_msb);
        const LM: u32 = rp2350_pac::sio::Interp0CtrlLane0::MASK_LSB_MASK;
        const LO: u32 = rp2350_pac::sio::Interp0CtrlLane0::MASK_LSB_OFFSET;
        const MM: u32 = rp2350_pac::sio::Interp0CtrlLane0::MASK_MSB_MASK;
        const MO: u32 = rp2350_pac::sio::Interp0CtrlLane0::MASK_MSB_OFFSET;
        self.ctrl = (self.ctrl & !(LM | MM))
            | (((mask_lsb as u32) << LO) & LM)
            | (((mask_msb as u32) << MO) & MM);
        self
    }

    #[inline]
    pub fn set_signed(&mut self, signed: bool) -> &mut Self {
        if signed {
            self.ctrl |= rp2350_pac::sio::Interp0CtrlLane0::SIGNED;
        } else {
            self.ctrl &= !rp2350_pac::sio::Interp0CtrlLane0::SIGNED;
        }
        self
    }

    #[inline]
    pub fn set_cross_input(&mut self, cross: bool) -> &mut Self {
        if cross {
            self.ctrl |= rp2350_pac::sio::Interp0CtrlLane0::CROSS_INPUT;
        } else {
            self.ctrl &= !rp2350_pac::sio::Interp0CtrlLane0::CROSS_INPUT;
        }
        self
    }

    #[inline]
    pub fn set_cross_result(&mut self, cross: bool) -> &mut Self {
        if cross {
            self.ctrl |= rp2350_pac::sio::Interp0CtrlLane0::CROSS_RESULT;
        } else {
            self.ctrl &= !rp2350_pac::sio::Interp0CtrlLane0::CROSS_RESULT;
        }
        self
    }

    #[inline]
    pub fn set_add_raw(&mut self, add_raw: bool) -> &mut Self {
        if add_raw {
            self.ctrl |= rp2350_pac::sio::Interp0CtrlLane0::ADD_RAW;
        } else {
            self.ctrl &= !rp2350_pac::sio::Interp0CtrlLane0::ADD_RAW;
        }
        self
    }

    /// Enable BLEND mode (**INTERP0 lane 0 only**).
    #[inline]
    pub fn set_blend(&mut self, blend: bool) -> &mut Self {
        if blend {
            self.ctrl |= rp2350_pac::sio::Interp0CtrlLane0::BLEND;
        } else {
            self.ctrl &= !rp2350_pac::sio::Interp0CtrlLane0::BLEND;
        }
        self
    }

    /// Enable CLAMP mode (**INTERP1 lane 0 only**).
    #[inline]
    pub fn set_clamp(&mut self, clamp: bool) -> &mut Self {
        if clamp {
            self.ctrl |= rp2350_pac::sio::Interp1CtrlLane0::CLAMP;
        } else {
            self.ctrl &= !rp2350_pac::sio::Interp1CtrlLane0::CLAMP;
        }
        self
    }

    /// Set FORCE_MSB bits (ORed into result bits 29:28 on the bus).
    #[inline]
    pub fn set_force_bits(&mut self, bits: u8) -> &mut Self {
        assert!(bits <= 3);
        const M: u32 = rp2350_pac::sio::Interp0CtrlLane0::FORCE_MSB_MASK;
        const O: u32 = rp2350_pac::sio::Interp0CtrlLane0::FORCE_MSB_OFFSET;
        self.ctrl = (self.ctrl & !M) | (((bits as u32) << O) & M);
        self
    }
}

impl Default for Config {
    #[inline]
    fn default() -> Self {
        Config::new()
    }
}
