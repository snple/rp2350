//! Interrupt Management HAL for RP2350
//!
//! This module provides safe and ergonomic interrupt management for the RP2350
//! microcontroller. It wraps the Cortex-M33 NVIC (Nested Vectored Interrupt Controller)
//! and provides additional utilities for RP2350-specific interrupt handling.
//!
//! ## Dual-Core Interrupt Architecture
//!
//! RP2350 has two Cortex-M33 cores, each with its own **independent NVIC**. The NVIC
//! registers reside on the Private Peripheral Bus (PPB), so reads and writes to
//! `0xE000_E100` always access the *current core's* NVIC — there is no way to
//! directly poke another core's NVIC from software.
//!
//! **Key implications:**
//!
//! - `enable()` / `disable()` / `set_priority()` etc. only affect the **calling core**.
//! - Which core receives an interrupt depends on which core's NVIC enabled it.
//! - If both cores enable the same IRQ, both receive it (for level-triggered
//!   interrupts, the first to acknowledge wins).
//! - To route an interrupt exclusively to one core, enable it only on that
//!   core and leave it disabled on the other.
//!
//! ## Recommended Dual-Core Interrupt Pattern
//!
//! Each core configures its own NVIC in its own initialization code:
//!
//! ```ignore
//! // Core 0 main — handles USB and UART
//! #[main]
//! fn main() -> ! {
//!     unsafe {
//!         interrupt::enable(Interrupt::USBCTRL_IRQ);
//!         interrupt::enable(Interrupt::UART0_IRQ);
//!     }
//!     // launch Core 1...
//! }
//!
//! // Core 1 entry — handles DMA and PIO
//! fn core1_main() -> ! {
//!     unsafe {
//!         interrupt::enable(Interrupt::DMA_IRQ_0);
//!         interrupt::enable(Interrupt::PIO0_IRQ_0);
//!     }
//!     loop { rp2350_rt::asm::wfe(); }
//! }
//! ```
//!
//! ## Cross-Core Interrupt Triggering
//!
//! There is no need to remotely poke another core's NVIC. To trigger an
//! interrupt on the other core, use the hardware mechanisms:
//!
//! - **Doorbell** (`SIO_IRQ_BELL`): 8-bit flag bits; writing one triggers the
//!   other core's interrupt
//! - **FIFO** (`SIO_IRQ_FIFO`): sending data also triggers the other core's interrupt
//! - **Spare IRQ**: 6 software interrupts, triggered on the current core via `pend()`
//!
//! ```ignore
//! use rp2350::multicore::Doorbell;
//!
//! // Core 0: notify Core 1 that there is work to do
//! Doorbell::new().signal(0);
//!
//! // Core 1: handle in the SIO_IRQ_BELL handler
//! #[unsafe(no_mangle)]
//! fn SIO_IRQ_BELL() {
//!     let db = Doorbell::new();
//!     db.clear_local(1);
//!     // handle requests from Core 0...
//! }
//! ```
//!
//! ## Priority System (RP2350)
//!
//! RP2350's Cortex-M33 implements priority in the **high 4 bits** of each
//! 8-bit IPR register field, providing **16 effective levels**. Lower numbers
//! mean higher priority.
//!
//! | Level | 8-bit write value | Hex |
//! |------|-------------|---------|
//! |  0   |   0         | 0x00    | ← highest priority (reset default for all interrupts)
//! |  1   |  16         | 0x10    |
//! |  2   |  32         | 0x20    |
//! |  3   |  48         | 0x30    |
//! |  4   |  64         | 0x40    |
//! |  5   |  80         | 0x50    |
//! |  6   |  96         | 0x60    |
//! |  7   | 112         | 0x70    |
//! |  8   | 128         | 0x80    | ← common mid-range value
//! |  9   | 144         | 0x90    |
//! | 10   | 160         | 0xA0    |
//! | 11   | 176         | 0xB0    |
//! | 12   | 192         | 0xC0    |
//! | 13   | 208         | 0xD0    |
//! | 14   | 224         | 0xE0    |
//! | 15   | 240         | 0xF0    | ← lowest priority
//!
//! **Important details:**
//! - Writing a non-aligned value (e.g. 100) is silently truncated by the
//!   hardware to `value & 0xF0` (i.e. 96). Always use multiples of 16 to
//!   avoid ambiguity.
//! - After reset, all interrupt priority registers are **0x00** (highest).
//! - **`set_priority()` must be called before `enable()`**, otherwise the
//!   first trigger of the interrupt still runs at priority 0 (highest).
//!
//! ## Preemption Behavior
//!
//! Cortex-M33 defaults to PRIGROUP=0 (AIRCR register), meaning all 4 priority
//! bits are used for **preemption priority**, with no sub-priority bits.
//! Effects:
//!
//! - An interrupt with a numerically smaller priority can **preempt** a
//!   running interrupt with a larger value.
//! - Interrupts of equal priority **cannot preempt each other** (they are
//!   handled only after the current ISR returns).
//!
//! ```ignore
//! // Typical priority assignment example:
//! // USBCTRL_IRQ  → 0   (highest, ensures timely USB bus response)
//! // DMA_IRQ_0    → 32  (high, data streaming)
//! // TIMER0_IRQ_0 → 64  (medium)
//! // SPARE_IRQ_0  → 128 (low, application event handling)
//! // SysTick priority is configured separately via SCB's SHPR3 register, same range
//!
//! interrupt::set_priority(Interrupt::USBCTRL_IRQ, 0);
//! interrupt::set_priority(Interrupt::SPARE_IRQ_0, 128);
//! // set the priority before enabling so the first trigger runs at the right priority
//! unsafe {
//!     interrupt::enable(Interrupt::USBCTRL_IRQ);
//!     interrupt::enable(Interrupt::SPARE_IRQ_0);
//! }
//! ```
//!
//! ## Binding Interrupts with the `#[bind]` Macro
//!
//! The `bind` macro binds a function to the specified interrupt vector,
//! generating a `#[no_mangle]` ISR entry point:
//!
//! ```ignore
//! use rp2350::bind;
//!
//! #[bind(SPARE_IRQ_0)]
//! fn event_handler() {
//!     // handle application events
//! }
//! ```
//!
//! **Priority must be set separately before `enable()`** (the macro does not
//! handle it):
//!
//! ```ignore
//! interrupt::set_priority(Interrupt::SPARE_IRQ_0, 128); // set priority first
//! unsafe { interrupt::enable(Interrupt::SPARE_IRQ_0); } // then enable
//! ```
//!
//! ## Features
//!
//! - NVIC control (enable, disable, pend, clear, priority)
//! - Critical sections for interrupt-safe code
//! - Re-export of interrupt numbers from PAC
//!
//! ## Example
//!
//! ```ignore
//! use rp2350::interrupt::{self, Interrupt};
//! use rp2350::pac::Interrupt as IrqNum;
//!
//! // Enable a specific interrupt (on the current core)
//! // must set priority before enabling, otherwise the first trigger runs at priority 0
//! interrupt::set_priority(IrqNum::TIMER0_IRQ_0, 64); // priority level 4
//! unsafe { interrupt::enable(IrqNum::TIMER0_IRQ_0); }
//!
//! // Run code in a critical section (all interrupts disabled)
//! interrupt::free(|_| {
//!     // Interrupt-safe code here
//! });
//! ```
//!
//! ## Interrupt Handler Example
//!
//! ```ignore
//!
//! #[unsafe(no_mangle)]
//! fn TIMER0_IRQ_0() {
//!     // Handle timer0 alarm 0 interrupt
//!     // Remember to clear the interrupt flag!
//! }
//! ```

use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicU8, Ordering};
use rp2350_rt::peripheral::NVIC;

// Re-export the interrupt enum from PAC for convenience
pub use crate::pac::Interrupt;

// ============================================================================
// NVIC Operations
// ============================================================================

/// Enable an interrupt in the **current core's** NVIC
///
/// This allows the interrupt to be triggered on the calling core only.
/// Note that the peripheral must also have its interrupt enabled
/// (e.g., timer alarm interrupt).
///
/// # Safety
/// Enabling an interrupt can cause the interrupt handler to run at any time.
/// Make sure the handler is properly set up before enabling.
///
/// # Example
/// ```ignore
/// use rp2350::interrupt::{self, Interrupt};
/// use rp2350::pac::Interrupt as IrqNum;
///
/// // Enable timer0 alarm 0 interrupt on whichever core calls this
/// unsafe { interrupt::enable(IrqNum::TIMER0_IRQ_0); }
/// ```
#[inline]
pub unsafe fn enable(interrupt: Interrupt) {
    unsafe {
        NVIC::enable(interrupt as u16);
    }
}

/// Disable an interrupt in the **current core's** NVIC
///
/// This prevents the interrupt from triggering on the calling core.
/// The interrupt remains pending if it was triggered while disabled.
///
/// # Example
/// ```ignore
/// use rp2350::interrupt::{self, Interrupt};
/// use rp2350::pac::Interrupt as IrqNum;
///
/// interrupt::disable(IrqNum::TIMER0_IRQ_0);
/// ```
#[inline]
pub fn disable(interrupt: Interrupt) {
    NVIC::disable(interrupt as u16);
}

/// Check if an interrupt is enabled in the **current core's** NVIC
#[inline]
pub fn is_enabled(interrupt: Interrupt) -> bool {
    NVIC::is_enabled(interrupt as u16)
}

/// Check if an interrupt is pending in the **current core's** NVIC
#[inline]
pub fn is_pending(interrupt: Interrupt) -> bool {
    NVIC::is_pending(interrupt as u16)
}

/// Set an interrupt as pending in the **current core's** NVIC
///
/// This will cause the interrupt handler to run when the interrupt
/// is enabled and has higher priority than the current execution.
#[inline]
pub fn pend(interrupt: Interrupt) {
    NVIC::pend(interrupt as u16);
}

/// Clear a pending interrupt in the **current core's** NVIC
#[inline]
pub fn unpend(interrupt: Interrupt) {
    NVIC::unpend(interrupt as u16);
}

/// Set the priority of an interrupt in the **current core's** NVIC
///
/// RP2350's Cortex-M33 implements **4-bit priority** (the high 4 bits are
/// effective), giving 16 levels total. Lower numbers mean higher priority.
/// The written value is truncated to `value & 0xF0`, so **use multiples of 16**.
///
/// | 8-bit value | Level | Description |
/// |---------|------|------|
/// |   0     |  0   | highest priority (reset default) |
/// |  16     |  1   | |
/// |  32     |  2   | |
/// |  48     |  3   | |
/// |  64     |  4   | |
/// |  80     |  5   | |
/// |  96     |  6   | |
/// | 112     |  7   | |
/// | 128     |  8   | common mid-range value |
/// | 144..224|9..14 | |
/// | 240     | 15   | lowest priority |
///
/// **Must be called before `enable()`**, otherwise the interrupt's first
/// trigger runs at the reset default (0).
///
/// # Arguments
/// * `interrupt` - the interrupt to configure
/// * `priority`  - priority value, use multiples of 16 (0 = highest, 240 = lowest)
///
/// # Safety
/// This function accesses the NVIC registers of the current core. The caller must ensure
/// that setting the priority does not cause any timing or stability issues. Additionally,
/// it is recommended to call this function before `enable()` to ensure the interrupt is
/// triggered with the correct priority from its first occurrence.
///
/// # Example
/// ```ignore
/// use rp2350::interrupt::{self, Interrupt};
///
/// // set priority first, then enable
/// interrupt::set_priority(Interrupt::SPARE_IRQ_0, 128); // level 8
/// unsafe { interrupt::enable(Interrupt::SPARE_IRQ_0); }
/// ```
#[inline]
pub unsafe fn set_priority(interrupt: Interrupt, priority: u8) {
    unsafe {
        NVIC::set_priority(interrupt as u16, priority);
    }
}

/// Get the priority of an interrupt from the **current core's** NVIC
#[inline]
pub fn priority(interrupt: Interrupt) -> u8 {
    NVIC::priority(interrupt as u16)
}

// ============================================================================
// Additional Utilities
// ============================================================================

/// Check if we are currently in an interrupt handler
#[inline]
pub fn in_interrupt() -> bool {
    // Read the IPSR (Interrupt Program Status Register)
    // If non-zero, we're in an exception/interrupt handler
    let ipsr: u32;
    #[allow(asm_sub_register)]
    unsafe {
        core::arch::asm!("mrs {0}, IPSR", out(reg) ipsr, options(nomem, nostack, preserves_flags));
    }
    (ipsr & 0x1FF) != 0
}

// ============================================================================
// Interrupt Groups
// ============================================================================

// NOTE: PAC's `Interrupt` enum already names every IRQ by peripheral (e.g.
// `Interrupt::TIMER0_IRQ_0`, `Interrupt::PWM_IRQ_WRAP_0`). The per-peripheral
// alias modules that previously lived here were pure re-exports with no extra
// value (and one had an incorrect `ALL` array), so they were removed. Only the
// `spare` group remains, because it adds a `COUNT` constant and an `ALL`
// array that are useful for iterating over the software-triggerable IRQs.

/// Spare/User interrupt numbers (for software-triggered tasks)
///
/// RP2350 provides 6 spare interrupts that can be used as software interrupts.
/// These are ideal for implementing deferred task execution.
pub mod spare {
    use super::*;

    /// Number of available spare interrupts
    pub const COUNT: usize = 6;

    /// Spare SPARE_IRQ 0
    pub const SPARE_IRQ_0: Interrupt = Interrupt::SPARE_IRQ_0;
    /// Spare SPARE_IRQ 1
    pub const SPARE_IRQ_1: Interrupt = Interrupt::SPARE_IRQ_1;
    /// Spare SPARE_IRQ 2
    pub const SPARE_IRQ_2: Interrupt = Interrupt::SPARE_IRQ_2;
    /// Spare SPARE_IRQ 3
    pub const SPARE_IRQ_3: Interrupt = Interrupt::SPARE_IRQ_3;
    /// Spare SPARE_IRQ 4
    pub const SPARE_IRQ_4: Interrupt = Interrupt::SPARE_IRQ_4;
    /// Spare SPARE_IRQ 5
    pub const SPARE_IRQ_5: Interrupt = Interrupt::SPARE_IRQ_5;

    /// All spare interrupts
    pub const ALL: [Interrupt; COUNT] = [
        SPARE_IRQ_0,
        SPARE_IRQ_1,
        SPARE_IRQ_2,
        SPARE_IRQ_3,
        SPARE_IRQ_4,
        SPARE_IRQ_5,
    ];
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Disable all interrupts and return a guard that re-enables them on drop
///
/// This is useful for RAII-style critical sections.
///
/// # Example
/// ```ignore
/// use rp2350::interrupt;
///
/// {
///     let _guard = interrupt::disable_all();
///     // All interrupts disabled here
///     // ...
/// } // Interrupts automatically restored here
/// ```
pub fn disable_all() -> InterruptGuard {
    let was_enabled = rp2350_rt::interrupt::is_enabled();
    rp2350_rt::interrupt::disable();
    InterruptGuard { was_enabled }
}

/// Guard that re-enables interrupts when dropped
pub struct InterruptGuard {
    was_enabled: bool,
}

impl Drop for InterruptGuard {
    fn drop(&mut self) {
        if self.was_enabled {
            unsafe { rp2350_rt::interrupt::enable() };
        }
    }
}

// ============================================================================
// Processor ID Detection
// ============================================================================

/// Get the ID of the currently executing core
///
/// Useful in interrupt handlers to determine which core the current code is
/// running on.
#[inline]
pub fn current_core() -> crate::multicore::CoreId {
    crate::multicore::core_id()
}

/// Token proving code is running in a critical section (interrupts disabled)
#[derive(Clone, Copy)]
pub struct CriticalSection {
    _marker: core::marker::PhantomData<*const ()>,
}

impl CriticalSection {
    /// Create a new critical section token
    ///
    /// # Safety
    ///
    /// Must only be called when interrupts are actually disabled.
    #[inline(always)]
    pub(crate) unsafe fn new() -> Self {
        Self {
            _marker: core::marker::PhantomData,
        }
    }
}

/// A mutual exclusion primitive for protecting shared data in interrupt handlers
///
/// This is similar to `std::sync::Mutex` but designed for use in interrupt
/// handlers on single-core systems. Data can only be accessed within a
/// critical section (with interrupts disabled).
///
/// # Example
///
/// ```ignore
/// use core::cell::RefCell;
/// use rp2350_rt::interrupt::{self, Mutex};
///
/// static COUNTER: Mutex<RefCell<u32>> = Mutex::new(RefCell::new(0));
///
/// fn main() {
///     interrupt::free(|cs| {
///         *COUNTER.borrow(cs).borrow_mut() += 1;
///     });
/// }
/// ```
pub struct Mutex<T> {
    data: UnsafeCell<T>,
}

// Safety: We only allow access through a CriticalSection token,
// which proves interrupts are disabled. On a single-core system,
// this prevents any data races.
unsafe impl<T> Sync for Mutex<T> where T: Send {}

impl<T> Mutex<T> {
    /// Create a new Mutex containing the given value
    #[inline(always)]
    pub const fn new(value: T) -> Self {
        Self {
            data: UnsafeCell::new(value),
        }
    }

    /// Borrow the data inside the mutex
    ///
    /// This requires a `CriticalSection` token proving that interrupts
    /// are disabled.
    #[inline(always)]
    pub fn borrow(&self, _cs: CriticalSection) -> &T {
        // Safety: We have a CriticalSection token, proving interrupts are
        // disabled. On a single-core system, this means no other code can
        // be accessing this data.
        unsafe { &*self.data.get() }
    }

    /// Get a raw pointer to the contained data
    ///
    /// This is useful when you need to pass the data to C code or
    /// perform other low-level operations.
    #[inline(always)]
    pub fn as_ptr(&self) -> *mut T {
        self.data.get()
    }
}

/// Execute a closure with interrupts disabled
///
/// This is a critical section - the closure runs atomically with respect
/// to all configurable interrupts. After the closure returns, the previous
/// interrupt state is restored.
///
/// Implemented directly via the same spinlock-31 and per-core nesting counter
/// used by `critical_section::Impl`, so nesting between `interrupt::free` and
/// `critical_section::with` (e.g. from `defmt`) is fully coherent and
/// deadlock-free.
///
/// # Example
///
/// ```ignore
/// use rp2350_rt::interrupt;
///
/// static mut COUNTER: u32 = 0;
///
/// let value = interrupt::free(|_cs| {
///     // Safe to access COUNTER - interrupts are disabled
///     unsafe {
///         COUNTER += 1;
///         COUNTER
///     }
/// });
/// ```
#[inline]
pub fn free<F, R>(f: F) -> R
where
    F: FnOnce(CriticalSection) -> R,
{
    // Call the low-level implementation directly; it shares the same
    // CS_NESTING counters and spinlock-31 with critical_section::Impl (when
    // enabled), so both paths are fully coherent and deadlock-free.
    let primask = cs_acquire();
    let result = f(unsafe { CriticalSection::new() });
    unsafe { cs_release(primask) };
    result
}

// ============================================================================
// Critical Section Core — spinlock-31 + per-core nesting counter
//
// This lives here so that `free()` can call the same logic directly,
// and `critical_section_impl` (when enabled) is just a thin wrapper that
// registers the same acquire/release pair as the `critical_section::Impl`.
// ============================================================================

/// Per-core critical-section nesting counter (indexed by core ID 0/1)
pub(crate) static CS_NESTING: [AtomicU8; 2] = [AtomicU8::new(0), AtomicU8::new(0)];

#[inline]
fn cs_spinlock_acquire() {
    // Read SIO spinlock-31: a non-zero return means acquisition succeeded
    // (the hardware guarantees atomicity)
    while unsafe { &*crate::sio::REGS }.spinlock31.spinlock31() == 0 {
        core::hint::spin_loop();
    }
}

#[inline]
fn cs_spinlock_release() {
    // Write any value to release spinlock-31
    unsafe { &*crate::sio::REGS }.spinlock31.write_spinlock31(1);
}

/// Enter a critical section: disable the local core's interrupts and acquire
/// spinlock-31 (nesting supported)
///
/// Returns the PRIMASK value before the call, for [`cs_release`] to restore.
#[inline]
pub(crate) fn cs_acquire() -> u32 {
    // 1. Save the current interrupt state
    let primask = rp2350_rt::register::primask::read_raw();
    // 2. Disable the local core's interrupts
    rp2350_rt::interrupt::disable();
    // 3. Update the nesting counter; only acquire spinlock-31 when it goes
    //    0 → 1
    let core = unsafe { &*crate::sio::REGS }.cpuid.cpuid() as usize;
    let prev = CS_NESTING[core].fetch_add(1, Ordering::Acquire);
    if prev == 0 {
        cs_spinlock_acquire();
    }
    primask
}

/// Exit a critical section: release spinlock-31 and restore interrupt state
/// (nesting supported)
///
/// # Safety
/// `primask` must be the value returned by the matching [`cs_acquire`] call.
#[inline]
pub(crate) unsafe fn cs_release(primask: u32) {
    // 1. Update the nesting counter; only release spinlock-31 when it goes
    //    1 → 0
    let core = unsafe { &*crate::sio::REGS }.cpuid.cpuid() as usize;
    let prev = CS_NESTING[core].fetch_sub(1, Ordering::Release);
    if prev == 1 {
        cs_spinlock_release();
    }
    // 2. Restore the previous interrupt state
    unsafe { rp2350_rt::register::primask::write_raw(primask) };
}
