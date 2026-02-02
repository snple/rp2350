//! 事件队列
//!
//! 提供中断与主循环之间的通信机制。
//!
//! ## 设计特点
//!
//! - **无锁 SPSC 队列**: 单生产者(中断)单消费者(主循环)
//! - **泛型支持**: 可自定义事件类型
//! - **零拷贝**: 事件直接存储在队列中
//!
//! ## 使用示例
//!
//! ### 基础用法
//!
//! ```ignore
//! use pico_rs::task::{Event, EventQueue};
//!
//! // 定义事件 ID
//! const EVT_BUTTON: u16 = 1;
//! const EVT_TIMER: u16 = 2;
//!
//! // 全局事件队列
//! static EVENTS: EventQueue = EventQueue::new();
//!
//! fn main() -> ! {
//!     loop {
//!         while let Some(event) = EVENTS.pop() {
//!             match event.id {
//!                 EVT_BUTTON => handle_button(event.data),
//!                 EVT_TIMER => handle_timer(),
//!                 _ => {}
//!             }
//!         }
//!         cortex_m::asm::wfi();
//!     }
//! }
//!
//! // 在中断中发送事件
//! #[unsafe(no_mangle)]
//! fn GPIO_IRQ() {
//!     EVENTS.push(Event::new(EVT_BUTTON, 0));
//! }
//! ```
//!
//! ### 自定义事件类型
//!
//! ```ignore
//! use pico_rs::task::GenericQueue;
//!
//! #[derive(Clone, Copy)]
//! enum MyEvent {
//!     ButtonPress(u8),
//!     TimerTick,
//!     UartRx(u8),
//! }
//!
//! static EVENTS: GenericQueue<MyEvent, 16> = GenericQueue::new();
//! ```

use core::cell::UnsafeCell;
use core::mem::MaybeUninit;
use core::sync::atomic::{AtomicUsize, Ordering};

use crate::interrupt;

// ============================================================================
// 简单事件 (u32)
// ============================================================================

/// 事件：包含 ID 和数据的简单消息
///
/// 内存布局：`[id: u16][data: u16]`，共 4 字节
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
#[derive(Default)]
pub struct Event {
    /// 事件类型 ID
    pub id: u16,
    /// 事件附带数据
    pub data: u16,
}

impl Event {
    /// 创建新事件
    #[inline]
    pub const fn new(id: u16, data: u16) -> Self {
        Self { id, data }
    }

    /// 仅带 ID 的事件（data = 0）
    #[inline]
    pub const fn id_only(id: u16) -> Self {
        Self { id, data: 0 }
    }

    /// 从 u32 解码
    #[inline]
    pub const fn from_raw(raw: u32) -> Self {
        Self {
            id: (raw >> 16) as u16,
            data: raw as u16,
        }
    }

    /// 编码为 u32
    #[inline]
    pub const fn to_raw(self) -> u32 {
        ((self.id as u32) << 16) | (self.data as u32)
    }
}

// ============================================================================
// 固定大小事件队列
// ============================================================================

/// 默认队列大小
pub const DEFAULT_QUEUE_SIZE: usize = 16;

/// 无锁单生产者单消费者事件队列
///
/// 这是一个环形缓冲区，专为中断 -> 主循环的事件传递设计。
///
/// **重要**：这是 SPSC 队列，只能有一个生产者和一个消费者。
/// 如果需要多生产者，请在 push 时使用 critical section。
pub struct EventQueue<const N: usize = DEFAULT_QUEUE_SIZE> {
    buffer: UnsafeCell<[u32; N]>,
    head: AtomicUsize, // 消费者读取位置
    tail: AtomicUsize, // 生产者写入位置
}

// Safety: 单生产者单消费者模式，通过原子操作保证安全
unsafe impl<const N: usize> Sync for EventQueue<N> {}

impl<const N: usize> EventQueue<N> {
    /// 编译时检查 N 是 2 的幂
    const MASK: usize = {
        assert!(
            N > 0 && (N & (N - 1)) == 0,
            "Queue size must be a power of 2"
        );
        N - 1
    };

    /// 创建新的事件队列
    pub const fn new() -> Self {
        Self {
            buffer: UnsafeCell::new([0; N]),
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
        }
    }

    /// 推入事件（从中断或生产者调用）
    ///
    /// 返回 `true` 表示成功，`false` 表示队列已满
    #[inline]
    pub fn push(&self, event: Event) -> bool {
        let tail = self.tail.load(Ordering::Relaxed);
        let next_tail = (tail + 1) & Self::MASK;

        // 检查队列是否满
        if next_tail == self.head.load(Ordering::Acquire) {
            return false;
        }

        // 写入数据
        unsafe {
            (*self.buffer.get())[tail] = event.to_raw();
        }

        // 发布写入
        self.tail.store(next_tail, Ordering::Release);
        true
    }

    /// 弹出事件（从主循环或消费者调用）
    ///
    /// 返回 `Some(event)` 如果有事件，`None` 如果队列为空
    #[inline]
    pub fn pop(&self) -> Option<Event> {
        let head = self.head.load(Ordering::Relaxed);

        // 检查队列是否空
        if head == self.tail.load(Ordering::Acquire) {
            return None;
        }

        // 读取数据
        let raw = unsafe { (*self.buffer.get())[head] };

        // 更新读取位置
        self.head.store((head + 1) & Self::MASK, Ordering::Release);

        Some(Event::from_raw(raw))
    }

    /// 检查队列是否为空
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.head.load(Ordering::Relaxed) == self.tail.load(Ordering::Relaxed)
    }

    /// 获取队列中的事件数量
    #[inline]
    pub fn len(&self) -> usize {
        let head = self.head.load(Ordering::Relaxed);
        let tail = self.tail.load(Ordering::Relaxed);
        (tail.wrapping_sub(head)) & Self::MASK
    }

    /// 获取队列容量
    #[inline]
    pub const fn capacity(&self) -> usize {
        N - 1 // 环形缓冲区实际可用空间是 N-1
    }

    /// 清空队列
    #[inline]
    pub fn clear(&self) {
        // 直接将 head 设置为 tail 即可
        let tail = self.tail.load(Ordering::Relaxed);
        self.head.store(tail, Ordering::Release);
    }

    /// 尝试推入事件，如果队列满则覆盖最旧的事件
    ///
    /// **警告**: 这个操作不是原子的，仅应在禁用中断时使用
    #[inline]
    pub fn push_overwrite(&self, event: Event) {
        let tail = self.tail.load(Ordering::Relaxed);
        let next_tail = (tail + 1) & Self::MASK;
        let head = self.head.load(Ordering::Relaxed);

        // 如果满了，丢弃最旧的
        if next_tail == head {
            self.head.store((head + 1) & Self::MASK, Ordering::Relaxed);
        }

        unsafe {
            (*self.buffer.get())[tail] = event.to_raw();
        }
        self.tail.store(next_tail, Ordering::Release);
    }
}

impl<const N: usize> Default for EventQueue<N> {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 泛型队列（支持自定义事件类型）
// ============================================================================

/// 泛型无锁 SPSC 队列
///
/// 允许使用自定义事件类型，而不仅仅是 `Event`。
pub struct GenericQueue<T: Copy, const N: usize> {
    buffer: UnsafeCell<[MaybeUninit<T>; N]>,
    head: AtomicUsize,
    tail: AtomicUsize,
}

unsafe impl<T: Copy + Send, const N: usize> Sync for GenericQueue<T, N> {}

impl<T: Copy, const N: usize> GenericQueue<T, N> {
    const MASK: usize = {
        assert!(
            N > 0 && (N & (N - 1)) == 0,
            "Queue size must be a power of 2"
        );
        N - 1
    };

    /// 创建新的泛型队列
    ///
    /// 注意：由于 MaybeUninit，这是 const 安全的
    pub const fn new() -> Self {
        Self {
            buffer: UnsafeCell::new(unsafe { MaybeUninit::uninit().assume_init() }),
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
        }
    }

    /// 推入元素
    #[inline]
    pub fn push(&self, item: T) -> bool {
        let tail = self.tail.load(Ordering::Relaxed);
        let next_tail = (tail + 1) & Self::MASK;

        if next_tail == self.head.load(Ordering::Acquire) {
            return false;
        }

        unsafe {
            (*self.buffer.get())[tail].write(item);
        }

        self.tail.store(next_tail, Ordering::Release);
        true
    }

    /// 弹出元素
    #[inline]
    pub fn pop(&self) -> Option<T> {
        let head = self.head.load(Ordering::Relaxed);

        if head == self.tail.load(Ordering::Acquire) {
            return None;
        }

        let item = unsafe { (*self.buffer.get())[head].assume_init_read() };
        self.head.store((head + 1) & Self::MASK, Ordering::Release);

        Some(item)
    }

    /// 检查是否为空
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.head.load(Ordering::Relaxed) == self.tail.load(Ordering::Relaxed)
    }

    /// 获取元素数量
    #[inline]
    pub fn len(&self) -> usize {
        let head = self.head.load(Ordering::Relaxed);
        let tail = self.tail.load(Ordering::Relaxed);
        (tail.wrapping_sub(head)) & Self::MASK
    }

    /// 清空队列
    #[inline]
    pub fn clear(&self) {
        let tail = self.tail.load(Ordering::Relaxed);
        self.head.store(tail, Ordering::Release);
    }
}

impl<T: Copy, const N: usize> Default for GenericQueue<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 事件 ID 定义宏
// ============================================================================

/// 定义事件 ID 常量
///
/// ```ignore
/// define_events! {
///     BUTTON_PRESS = 1,
///     TIMER_TICK = 2,
///     UART_RX = 3,
/// }
/// ```
#[macro_export]
macro_rules! define_events {
    ($($name:ident = $value:expr),* $(,)?) => {
        $(
            #[allow(dead_code)]
            pub const $name: u16 = $value;
        )*
    };
}

// ============================================================================
// 多生产者支持
// ============================================================================
mod mpsc {
    use super::*;

    /// 多生产者单消费者事件队列
    ///
    /// 通过 critical section 保护 push 操作，允许多个中断同时推入事件。
    pub struct MpscQueue<const N: usize = DEFAULT_QUEUE_SIZE> {
        inner: EventQueue<N>,
    }

    impl<const N: usize> MpscQueue<N> {
        /// 创建新队列
        pub const fn new() -> Self {
            Self {
                inner: EventQueue::new(),
            }
        }

        /// 推入事件（中断安全）
        #[inline]
        pub fn push(&self, event: Event) -> bool {
            interrupt::free(|_| self.inner.push(event))
        }

        /// 弹出事件
        #[inline]
        pub fn pop(&self) -> Option<Event> {
            self.inner.pop()
        }

        /// 检查是否为空
        #[inline]
        pub fn is_empty(&self) -> bool {
            self.inner.is_empty()
        }

        /// 获取事件数量
        #[inline]
        pub fn len(&self) -> usize {
            self.inner.len()
        }

        /// 清空队列
        #[inline]
        pub fn clear(&self) {
            interrupt::free(|_| self.inner.clear())
        }
    }

    impl<const N: usize> Default for MpscQueue<N> {
        fn default() -> Self {
            Self::new()
        }
    }

    // Safety: protected by critical section
    unsafe impl<const N: usize> Sync for MpscQueue<N> {}
}

pub use mpsc::MpscQueue;
