//! 基于中断的任务系统
//!
//! 提供事件驱动的多任务处理，利用 RP2350 的中断系统实现高效的任务调度。
//!
//! ## 设计理念
//!
//! ```text
//! ┌─────────────────────────────────────┐
//! │         硬件中断 (GPIO, UART...)    │  ← 最高优先级
//! └──────────────┬──────────────────────┘
//!                │ 发送事件
//!                ▼
//! ┌─────────────────────────────────────┐
//! │           事件队列                   │  ← 中断与主循环的桥梁
//! └──────────────┬──────────────────────┘
//!                │ 消费事件
//!                ▼
//! ┌─────────────────────────────────────┐
//! │           主循环                     │  ← 处理事件
//! │   while let Some(e) = queue.pop()   │
//! │   cortex_m::asm::wfi()              │  ← 省电
//! └─────────────────────────────────────┘
//! ```
//!
//! ## 软件中断
//!
//! RP2350 提供 6 个 Spare IRQ (SPARE_IRQ_0 ~ SPARE_IRQ_5)，
//! 可用于实现软件触发的延迟任务。
//!
//! ```ignore
//! use roschi::interrupt::spare;
//!
//! // 初始化 spare IRQ 0，优先级 200
//! spare::init(0, 200);
//!
//! // 触发软件中断
//! spare::trigger(0);
//!
//! // 中断处理器
//! #[unsafe(no_mangle)]
//! fn SPARE_IRQ_0() {
//!     spare::clear(0);
//!     // 处理任务
//! }
//! ```
//!
//! ## 完整示例
//!
//! ```ignore
//! use roschi::kit::task::{Event, EventQueue};
//! use roschi::interrupt::spare;
//!
//! // 事件定义
//! const EVT_BUTTON: u16 = 1;
//! const EVT_TIMER: u16 = 2;
//! const EVT_WORK_DONE: u16 = 3;
//!
//! // 全局事件队列
//! static EVENTS: EventQueue = EventQueue::new();
//!
//! #[entry]
//! fn main() -> ! {
//!     // 初始化 spare IRQ 作为低优先级任务
//!     spare::init(0, 200);
//!
//!     loop {
//!         // 处理所有事件
//!         while let Some(event) = EVENTS.pop() {
//!             match event.id {
//!                 EVT_BUTTON => {
//!                     // 按钮处理，触发后台工作
//!                     spare::trigger(0);
//!                 }
//!                 EVT_TIMER => handle_timer(),
//!                 EVT_WORK_DONE => on_work_done(event.data),
//!                 _ => {}
//!             }
//!         }
//!
//!         // 省电：等待中断
//!         cortex_m::asm::wfi();
//!     }
//! }
//!
//! // GPIO 中断 - 高优先级
//! #[unsafe(no_mangle)]
//! fn IO_IRQ_BANK0() {
//!     // 清除中断标志...
//!     EVENTS.push(Event::new(EVT_BUTTON, pin));
//! }
//!
//! // Spare IRQ - 低优先级后台任务
//! #[unsafe(no_mangle)]
//! fn SPARE_IRQ_0() {
//!     spare::clear(0);
//!
//!     // 执行耗时操作
//!     let result = do_heavy_work();
//!
//!     // 通知主循环
//!     EVENTS.push(Event::new(EVT_WORK_DONE, result));
//! }
//! ```

mod event;

pub use event::MpscQueue;
pub use event::{DEFAULT_QUEUE_SIZE, Event, EventQueue, GenericQueue};
