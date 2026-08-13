//! Event queue
//!
//! Provides a communication mechanism between interrupts and the main loop.
//!
//! ## Design Features
//!
//! - **Lock-free SPSC queue**: single producer (interrupt) / single consumer
//!   (main loop)
//! - **Generic support**: custom event types are supported
//! - **Zero-copy**: events are stored directly in the queue
//!
//! ## Usage Example
//!
//! ### Basic usage
//!
//! ```ignore
//! use rp2350::kit::task::{Event, EventQueue};
//!
//! // define event IDs
//! const EVT_BUTTON: u16 = 1;
//! const EVT_TIMER: u16 = 2;
//!
//! // global event queue
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
//! // send an event from an interrupt
//! #[unsafe(no_mangle)]
//! fn GPIO_IRQ() {
//!     EVENTS.push(Event::new(EVT_BUTTON, 0));
//! }
//! ```
//!
//! ### Custom event types
//!
//! ```ignore
//! use rp2350::kit::task::GenericQueue;
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
// Simple event (u32)
// ============================================================================

/// An event: a simple message containing an ID and data
///
/// Memory layout: `[id: u16][data: u16]`, 4 bytes total
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
#[derive(Default)]
pub struct Event {
    /// Event type ID
    pub id: u16,
    /// Data attached to the event
    pub data: u16,
}

impl Event {
    /// Create a new event
    #[inline]
    pub const fn new(id: u16, data: u16) -> Self {
        Self { id, data }
    }

    /// Create an event with only an ID (data = 0)
    #[inline]
    pub const fn id_only(id: u16) -> Self {
        Self { id, data: 0 }
    }

    /// Decode from a u32
    #[inline]
    pub const fn from_raw(raw: u32) -> Self {
        Self {
            id: (raw >> 16) as u16,
            data: raw as u16,
        }
    }

    /// Encode as a u32
    #[inline]
    pub const fn to_raw(self) -> u32 {
        ((self.id as u32) << 16) | (self.data as u32)
    }
}

// ============================================================================
// Fixed-size event queue
// ============================================================================

/// Default queue size
pub const DEFAULT_QUEUE_SIZE: usize = 16;

/// Lock-free single-producer single-consumer event queue
///
/// This is a ring buffer designed for interrupt → main-loop event delivery.
///
/// **Important**: this is an SPSC queue — there can be only one producer and
/// one consumer. For multiple producers, use a critical section around `push`.
pub struct EventQueue<const N: usize = DEFAULT_QUEUE_SIZE> {
    buffer: UnsafeCell<[u32; N]>,
    head: AtomicUsize, // consumer read position
    tail: AtomicUsize, // producer write position
}

// Safety: single-producer single-consumer pattern, guaranteed by atomic ops
unsafe impl<const N: usize> Sync for EventQueue<N> {}

impl<const N: usize> EventQueue<N> {
    /// Compile-time check that N is a power of 2
    const MASK: usize = {
        assert!(
            N > 0 && (N & (N - 1)) == 0,
            "Queue size must be a power of 2"
        );
        N - 1
    };

    /// Create a new event queue
    pub const fn new() -> Self {
        Self {
            buffer: UnsafeCell::new([0; N]),
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
        }
    }

    /// Push an event (call from an interrupt or the producer)
    ///
    /// Returns `true` on success, `false` if the queue is full
    #[inline]
    pub fn push(&self, event: Event) -> bool {
        let tail = self.tail.load(Ordering::Relaxed);
        let next_tail = (tail + 1) & Self::MASK;

        // check whether the queue is full
        if next_tail == self.head.load(Ordering::Acquire) {
            return false;
        }

        // write the data
        unsafe {
            (*self.buffer.get())[tail] = event.to_raw();
        }

        // publish the write
        self.tail.store(next_tail, Ordering::Release);
        true
    }

    /// Pop an event (call from the main loop or the consumer)
    ///
    /// Returns `Some(event)` if there is an event, `None` if the queue is empty
    #[inline]
    pub fn pop(&self) -> Option<Event> {
        let head = self.head.load(Ordering::Relaxed);

        // check whether the queue is empty
        if head == self.tail.load(Ordering::Acquire) {
            return None;
        }

        // read the data
        let raw = unsafe { (*self.buffer.get())[head] };

        // update the read position
        self.head.store((head + 1) & Self::MASK, Ordering::Release);

        Some(Event::from_raw(raw))
    }

    /// Check whether the queue is empty
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.head.load(Ordering::Relaxed) == self.tail.load(Ordering::Relaxed)
    }

    /// Get the number of events in the queue
    #[inline]
    pub fn len(&self) -> usize {
        let head = self.head.load(Ordering::Relaxed);
        let tail = self.tail.load(Ordering::Relaxed);
        (tail.wrapping_sub(head)) & Self::MASK
    }

    /// Get the queue capacity
    #[inline]
    pub const fn capacity(&self) -> usize {
        N - 1 // ring buffer actual usable space is N-1
    }

    /// Clear the queue
    #[inline]
    pub fn clear(&self) {
        // simply set head to tail
        let tail = self.tail.load(Ordering::Relaxed);
        self.head.store(tail, Ordering::Release);
    }

    /// Try to push an event; if the queue is full, overwrite the oldest event
    ///
    /// **Warning**: this operation is not atomic — use only with interrupts
    /// disabled
    #[inline]
    pub fn push_overwrite(&self, event: Event) {
        let tail = self.tail.load(Ordering::Relaxed);
        let next_tail = (tail + 1) & Self::MASK;
        let head = self.head.load(Ordering::Relaxed);

        // if full, discard the oldest
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
// Generic queue (supports custom event types)
// ============================================================================

/// Generic lock-free SPSC queue
///
/// Allows custom event types instead of just `Event`.
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

    /// Create a new generic queue
    ///
    /// Note: const-safe thanks to MaybeUninit
    pub const fn new() -> Self {
        Self {
            buffer: UnsafeCell::new(unsafe { MaybeUninit::uninit().assume_init() }),
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
        }
    }

    /// Push an element
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

    /// Pop an element
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

    /// Check whether the queue is empty
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.head.load(Ordering::Relaxed) == self.tail.load(Ordering::Relaxed)
    }

    /// Get the number of elements
    #[inline]
    pub fn len(&self) -> usize {
        let head = self.head.load(Ordering::Relaxed);
        let tail = self.tail.load(Ordering::Relaxed);
        (tail.wrapping_sub(head)) & Self::MASK
    }

    /// Clear the queue
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
// Event ID definition macro
// ============================================================================

/// Define event ID constants
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
// Multi-producer support
// ============================================================================
mod mpsc {
    use super::*;

    /// Multi-producer single-consumer event queue
    ///
    /// Protects `push` with a critical section, allowing multiple interrupts
    /// to push events concurrently.
    pub struct MpscQueue<const N: usize = DEFAULT_QUEUE_SIZE> {
        inner: EventQueue<N>,
    }

    impl<const N: usize> MpscQueue<N> {
        /// Create a new queue
        pub const fn new() -> Self {
            Self {
                inner: EventQueue::new(),
            }
        }

        /// Push an event (interrupt-safe)
        #[inline]
        pub fn push(&self, event: Event) -> bool {
            interrupt::free(|_| self.inner.push(event))
        }

        /// Pop an event
        #[inline]
        pub fn pop(&self) -> Option<Event> {
            self.inner.pop()
        }

        /// Check whether the queue is empty
        #[inline]
        pub fn is_empty(&self) -> bool {
            self.inner.is_empty()
        }

        /// Get the number of events
        #[inline]
        pub fn len(&self) -> usize {
            self.inner.len()
        }

        /// Clear the queue
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
