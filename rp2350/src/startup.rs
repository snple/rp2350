//! Shared RP2350 startup initialisation.
//!
//! This module contains the hardware-specific early initialisation that must
//! run after reset and before user code enables RP2350 coprocessor features or
//! relies on debugger-clean spinlock state.

use core::sync::atomic::{Ordering, compiler_fence};

/// Perform RP2350-specific startup initialisation.
///
/// On Core 0 this:
/// - Releases all spinlocks
/// - Enables coprocessor access (CP0/CP4/CP10)
///
/// # Safety
///
/// This must run exactly once during early startup, before application code
/// uses hardware spinlocks or executes instructions that require CP0/CP4.
#[inline]
pub unsafe fn init() {
    crate::sio::release_all_spinlocks();

    crate::ppb::Cpacr::enable_cp();

    compiler_fence(Ordering::SeqCst);
}
