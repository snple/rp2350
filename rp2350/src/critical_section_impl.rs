//! `critical-section` crate implementation registration
//!
//! This is a thin wrapper module that registers `cs_acquire` / `cs_release`
//! from the `interrupt` module as the implementation for the `critical-section`
//! crate.
//!
//! The core logic (spinlock-31, nesting counter) lives in [`crate::interrupt`]
//! and is called directly by `interrupt::free`, so both paths share the same
//! state.
//!
//! This module is enabled on demand via the `critical-section-impl` feature
//! (on by default). If another crate has already registered an implementation,
//! disable the feature in your dependencies to avoid a conflict:
//!
//! ```toml
//! [dependencies]
//! rp2350 = { version = "...", default-features = false, features = ["rp2350a"] }
//! ```

struct Rp2350CriticalSection;

critical_section::set_impl!(Rp2350CriticalSection);

unsafe impl critical_section::Impl for Rp2350CriticalSection {
    unsafe fn acquire() -> critical_section::RawRestoreState {
        crate::interrupt::cs_acquire()
    }

    unsafe fn release(primask: critical_section::RawRestoreState) {
        unsafe { crate::interrupt::cs_release(primask) };
    }
}
