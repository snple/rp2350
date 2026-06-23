//! `critical-section` crate 实现注册
//!
//! 这是一个瘦包装模块，仅将 `interrupt` 模块中的 `cs_acquire` / `cs_release`
//! 注册为 `critical-section` crate 的实现。
//!
//! 核心逻辑（spinlock-31、嵌套计数器）位于 [`crate::interrupt`] 中，
//! 供 `interrupt::free` 直接调用，保证两条路径共享同一套状态。
//!
//! 此模块通过 `critical-section-impl` feature 按需启用（默认开启）。
//! 若另一个 crate 已注册了实现，可在依赖中关闭该 feature 避免冲突：
//!
//! ```toml
//! [dependencies]
//! pico-rs = { version = "...", default-features = false, features = ["rp2350a"] }
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
