//! Shared RP2350 startup initialisation.
//!
//! This module contains the hardware-specific early initialisation that must
//! run after reset and before user code enables RP2350 coprocessor features or
//! relies on debugger-clean spinlock state.

use core::sync::atomic::{Ordering, compiler_fence};

#[cfg(not(feature = "non-secure"))]
use crate::pac::PPB as StartupPpb;
#[cfg(feature = "non-secure")]
use crate::pac::PPB_NS as StartupPpb;

const CPACR_FULL_ACCESS: u32 = 0b11;

/// Perform RP2350-specific startup initialisation.
///
/// # Safety
///
/// This must run exactly once during early startup, before application code
/// uses hardware spinlocks or executes instructions that require CP0/CP4.
#[inline]
pub unsafe fn init() {
    crate::sio::release_all_spinlocks();

    let ppb = unsafe { StartupPpb::steal() };
    let cpacr = ppb
        .cpacr
        .read_val()
        .with_cp0(CPACR_FULL_ACCESS)
        .with_cp4(CPACR_FULL_ACCESS);
    ppb.cpacr.write_val(cpacr);

    compiler_fence(Ordering::SeqCst);
}
