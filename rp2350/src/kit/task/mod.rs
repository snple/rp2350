//! Interrupt-based task system
//!
//! Provides event-driven multi-tasking that leverages RP2350's interrupt
//! system for efficient task scheduling.
//!
//! ## Design philosophy
//!
//! ```text
//! ┌─────────────────────────────────────┐
//! │         hardware interrupts (GPIO, UART...)  │  ← highest priority
//! └──────────────┬──────────────────────┘
//!                │ send event
//!                ▼
//! ┌─────────────────────────────────────┐
//! │           event queue                │  ← bridge between ISR and main loop
//! └──────────────┬──────────────────────┘
//!                │ consume event
//!                ▼
//! ┌─────────────────────────────────────┐
//! │           main loop                 │  ← handles events
//! │   while let Some(e) = queue.pop()   │
//! │   cortex_m::asm::wfi()              │  ← power saving
//! └─────────────────────────────────────┘
//! ```
//!
//! ## Software interrupts
//!
//! RP2350 provides 6 Spare IRQs (SPARE_IRQ_0 ~ SPARE_IRQ_5) that can be used
//! to implement software-triggered deferred tasks.
//!
//! ```ignore
//! use rp2350::interrupt::spare;
//!
//! // initialize spare IRQ 0 with priority 200
//! spare::init(0, 200);
//!
//! // trigger a software interrupt
//! spare::trigger(0);
//!
//! // interrupt handler
//! #[unsafe(no_mangle)]
//! fn SPARE_IRQ_0() {
//!     spare::clear(0);
//!     // handle the task
//! }
//! ```
//!
//! ## Complete example
//!
//! ```ignore
//! use rp2350::kit::task::{Event, EventQueue};
//! use rp2350::interrupt::spare;
//!
//! // event definitions
//! const EVT_BUTTON: u16 = 1;
//! const EVT_TIMER: u16 = 2;
//! const EVT_WORK_DONE: u16 = 3;
//!
//! // global event queue
//! static EVENTS: EventQueue = EventQueue::new();
//!
//! #[main]
//! fn main() -> ! {
//!     // initialize a spare IRQ as a low-priority task
//!     spare::init(0, 200);
//!
//!     loop {
//!         // handle all events
//!         while let Some(event) = EVENTS.pop() {
//!             match event.id {
//!                 EVT_BUTTON => {
//!                     // button handling, trigger background work
//!                     spare::trigger(0);
//!                 }
//!                 EVT_TIMER => handle_timer(),
//!                 EVT_WORK_DONE => on_work_done(event.data),
//!                 _ => {}
//!             }
//!         }
//!
//!         // power saving: wait for an interrupt
//!         cortex_m::asm::wfi();
//!     }
//! }
//!
//! // GPIO interrupt - high priority
//! #[unsafe(no_mangle)]
//! fn IO_IRQ_BANK0() {
//!     // clear interrupt flags...
//!     EVENTS.push(Event::new(EVT_BUTTON, pin));
//! }
//!
//! // Spare IRQ - low-priority background task
//! #[unsafe(no_mangle)]
//! fn SPARE_IRQ_0() {
//!     spare::clear(0);
//!
//!     // perform the time-consuming operation
//!     let result = do_heavy_work();
//!
//!     // notify the main loop
//!     EVENTS.push(Event::new(EVT_WORK_DONE, result));
//! }
//! ```

mod event;

pub use event::MpscQueue;
pub use event::{DEFAULT_QUEUE_SIZE, Event, EventQueue, GenericQueue};
