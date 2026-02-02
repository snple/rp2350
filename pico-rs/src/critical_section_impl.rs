//! Critical Section 实现
//!
//! 为 RP2350 (双核 Cortex-M33) 提供 `critical-section` crate 的实现。
//!
//! ## 实现策略
//!
//! RP2350 是双核处理器，简单地禁用中断不足以保护跨核共享数据。
//! 我们采用以下策略：
//!
//! 1. **禁用本核中断** - 防止本核的中断处理程序打断临界区
//! 2. **获取硬件 Spinlock** - 防止另一个核进入临界区
//!
//! 这确保了临界区在多核环境下的正确性。
//!
//! ## Spinlock 分配
//!
//! 我们使用 **Spinlock 31** 作为 critical-section 的专用锁。
//! 用户代码应避免直接使用 Spinlock 31。
//!
//! ## 嵌套支持
//!
//! 支持嵌套的临界区。只有最外层的 release 会真正释放锁和恢复中断。

use core::sync::atomic::{AtomicU8, Ordering};

/// critical-section 专用的 Spinlock 编号
///
/// 使用 Spinlock 31，这是最高编号的 spinlock，
/// 用户代码应使用 0-30 号 spinlock。
#[allow(dead_code)]
const CRITICAL_SECTION_SPINLOCK: u8 = 31;

/// 每个核的嵌套计数器
///
/// 使用 2 元素数组，索引为核 ID (0 或 1)
static NESTING: [AtomicU8; 2] = [AtomicU8::new(0), AtomicU8::new(0)];

struct Rp2350CriticalSection;

critical_section::set_impl!(Rp2350CriticalSection);

/// 获取当前核 ID (0 或 1)
#[inline]
fn core_id() -> usize {
    // SIO CPUID 寄存器返回当前核的 ID
    let sio = unsafe { &*pico_rs_pac::SIO::PTR };
    sio.cpuid.read_bits() as usize
}

/// 尝试获取 spinlock（非阻塞）
#[inline]
fn try_acquire_spinlock() -> bool {
    let sio = unsafe { &*pico_rs_pac::SIO::PTR };
    // 读取 spinlock 寄存器：返回非零表示获取成功
    sio.spinlock31.read_bits() != 0
}

/// 获取 spinlock（阻塞）
#[inline]
fn acquire_spinlock() {
    while !try_acquire_spinlock() {
        core::hint::spin_loop();
    }
}

/// 释放 spinlock
#[inline]
fn release_spinlock() {
    let sio = unsafe { &*pico_rs_pac::SIO::PTR };
    // 写入任意值释放 spinlock
    sio.spinlock31.write_bits(1);
}

unsafe impl critical_section::Impl for Rp2350CriticalSection {
    unsafe fn acquire() -> critical_section::RawRestoreState {
        // 1. 保存当前中断状态
        let restore_state = pico_rs_rt::register::primask::read_raw();

        // 2. 禁用本核中断
        pico_rs_rt::interrupt::disable();

        // 3. 获取当前核 ID
        let core = core_id();

        // 4. 检查是否已经在临界区（嵌套）
        let nesting = NESTING[core].fetch_add(1, Ordering::Acquire);

        // 5. 只有第一次进入时才获取 spinlock
        if nesting == 0 {
            acquire_spinlock();
        }

        // 返回之前的中断状态
        restore_state
    }

    unsafe fn release(restore_state: critical_section::RawRestoreState) {
        // 1. 获取当前核 ID
        let core = core_id();

        // 2. 减少嵌套计数
        let nesting = NESTING[core].fetch_sub(1, Ordering::Release);

        // 3. 只有最外层临界区才释放 spinlock
        if nesting == 1 {
            release_spinlock();
        }

        // 4. 恢复中断状态（只有之前是启用的才恢复）
        unsafe { pico_rs_rt::register::primask::write_raw(restore_state) };
    }
}
