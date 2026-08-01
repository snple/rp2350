//! Bounded spin-wait helpers.
//!
//! Several HAL drivers busy-wait for hardware to reach a state — reset done,
//! clock stable, PLL locked, FIFO space, peripheral ready, and so on. Without a
//! bound, a misconfigured peripheral (gated clock, held in reset, missing
//! device, wrong DMA request) turns into an unbounded hang.
//!
//! These helpers bound such waits with a *spin count*. The default
//! [`spin_wait`] panics on timeout — a hang-guard for waits the hardware is
//! guaranteed to satisfy once configured. [`try_spin_wait`] reports failure as
//! a `bool` for callers that want to recover instead of aborting, and
//! [`spin_wait_max`] / [`try_spin_wait_max`] accept a custom bound per call
//! site for waits whose duration is legitimately variable (e.g. XOSC startup
//! delay), or whose timeout should be recoverable.

/// Default maximum spin iterations for [`spin_wait`].
///
/// This is a *hang guard*, not a calibrated timeout: a `spin_loop` iteration
/// takes a few cycles, so 1_000_000 iterations is roughly tens of milliseconds
/// at 150 MHz but several times longer at the boot-time 12 MHz clock. It is
/// sized to sit far above any legitimate hardware settle time while still
/// bounding a stuck peripheral.
///
/// Call sites that need a different bound should use [`spin_wait_max`] or
/// [`try_spin_wait_max`] instead of relying on this default.
pub(crate) const SPIN_TIMEOUT: u32 = 1_000_000;

/// Spin until `cond()` returns `true`, panicking after [`SPIN_TIMEOUT`]
/// iterations.
///
/// `what` is included in the panic message to identify the stalled operation.
///
/// Prefer this for waits the hardware is guaranteed to satisfy once configured
/// (reset done, PLL lock, FIFO space). Use [`try_spin_wait`] when the caller
/// wants to recover from a stuck wait, and [`spin_wait_max`] when the wait can
/// legitimately take far longer than the default bound.
#[inline(always)]
pub(crate) fn spin_wait(what: &str, mut cond: impl FnMut() -> bool) {
    if !spin_wait_max_bool(&mut cond, SPIN_TIMEOUT) {
        panic!("timed out waiting for {what}");
    }
}

/// Spin until `cond()` returns `true`, returning `false` after
/// [`SPIN_TIMEOUT`] iterations without panicking.
///
/// Use this when the caller wants to handle a stuck wait (report an error,
/// retry, or fall back) instead of aborting the whole program. For example,
/// SPI `transfer`/`read` and UART blocking reads use this to return
/// `Error::Timeout` on a stuck peripheral.
#[inline(always)]
pub(crate) fn try_spin_wait(mut cond: impl FnMut() -> bool) -> bool {
    spin_wait_max_bool(&mut cond, SPIN_TIMEOUT)
}

/// Spin until `cond()` returns `true` or `max` iterations elapse, returning
/// `false` on timeout without panicking.
///
/// Combine the recoverable semantics of [`try_spin_wait`] with a per-call-site
/// bound, for waits that must both be recoverable and legitimately take longer
/// than [`SPIN_TIMEOUT`]. Currently no internal call site needs a custom bound,
/// so it is kept available for callers without a current user.
#[allow(dead_code)]
#[inline(always)]
pub(crate) fn try_spin_wait_max(mut cond: impl FnMut() -> bool, max: u32) -> bool {
    spin_wait_max_bool(&mut cond, max)
}

/// Spin until `cond()` returns `true` or `max` iterations elapse, panicking on
/// timeout.
///
/// Use a larger `max` than [`SPIN_TIMEOUT`] for waits whose duration is
/// legitimately variable and can exceed the default bound (e.g. XOSC startup
/// with a raised `STARTUP_DELAY_MULTIPLIER`).
#[inline(always)]
pub(crate) fn spin_wait_max(what: &str, mut cond: impl FnMut() -> bool, max: u32) {
    if !spin_wait_max_bool(&mut cond, max) {
        panic!("timed out waiting for {what}");
    }
}

#[inline(always)]
fn spin_wait_max_bool(cond: &mut impl FnMut() -> bool, max: u32) -> bool {
    let mut spins = 0u32;
    while !cond() {
        core::hint::spin_loop();
        spins += 1;
        if spins >= max {
            return false;
        }
    }
    true
}
