//! Interrupt Management
//!
//! This module provides interrupt control functions:
//!
//! - Global interrupt enable/disable
//! - Critical sections
//! - Priority-based masking (BASEPRI)
//! - Mutex for interrupt-safe data sharing
//!
//! # Example
//!
//! ```ignore
//! use pico_rs_rt::interrupt;
//!
//! // Disable all interrupts
//! interrupt::disable();
//!
//! // Do critical work...
//!
//! // Re-enable interrupts
//! unsafe { interrupt::enable(); }
//! ```

use core::arch::asm;
use core::cell::UnsafeCell;

use crate::register::primask;

/// Token proving code is running in a critical section (interrupts disabled)
#[derive(Clone, Copy)]
pub struct CriticalSection {
    _private: (),
}

impl CriticalSection {
    /// Create a new critical section token
    ///
    /// # Safety
    ///
    /// Must only be called when interrupts are actually disabled.
    #[inline(always)]
    pub(crate) unsafe fn new() -> Self {
        Self { _private: () }
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
/// use pico_rs_rt::interrupt::{self, Mutex};
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

/// Disable all configurable interrupts
///
/// This sets PRIMASK, preventing all exceptions with configurable priority
/// from occurring. NMI and HardFault are not affected.
#[inline(always)]
pub fn disable() {
    unsafe {
        asm!("cpsid i", options(nomem, nostack, preserves_flags));
    }
}

/// Enable all configurable interrupts
///
/// This clears PRIMASK, allowing interrupts to occur.
///
/// # Safety
///
/// Enabling interrupts can cause interrupt handlers to run at any time.
/// Ensure all interrupt handlers are properly configured and that enabling
/// interrupts won't corrupt any state being modified.
#[inline(always)]
pub unsafe fn enable() {
    unsafe {
        asm!("cpsie i", options(nomem, nostack, preserves_flags));
    }
}

/// Execute a closure with interrupts disabled
///
/// This is a critical section - the closure runs atomically with respect
/// to all configurable interrupts. After the closure returns, the previous
/// interrupt state is restored.
///
/// # Example
///
/// ```ignore
/// use pico_rs_rt::interrupt;
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
    // Save current PRIMASK state
    let primask = primask::read_raw();

    // Disable interrupts
    disable();

    // Execute the closure
    let result = f(unsafe { CriticalSection::new() });

    // Restore previous interrupt state
    unsafe {
        primask::write_raw(primask);
    }

    result
}

/// Check if interrupts are currently enabled
#[inline(always)]
pub fn is_enabled() -> bool {
    primask::read().is_active()
}

// =============================================================================
// Priority-based interrupt masking (BASEPRI)
// =============================================================================

/// Mask interrupts with priority lower than or equal to the given threshold
///
/// This uses BASEPRI, which is more selective than PRIMASK. Interrupts with
/// priority numerically higher (lower urgency) than `priority` are masked.
///
/// # Arguments
///
/// * `priority` - Priority threshold. Set to 0 to disable masking.
///
/// # Safety
///
/// Changing BASEPRI affects which interrupts can occur, which may affect
/// system behavior.
#[inline(always)]
pub unsafe fn set_priority_threshold(priority: u8) {
    unsafe {
        asm!(
            "msr BASEPRI, {0}",
            in(reg) priority as u32,
            options(nomem, nostack, preserves_flags)
        );
    }
}

/// Get the current priority threshold
#[inline(always)]
pub fn priority_threshold() -> u8 {
    let basepri: u32;
    unsafe {
        asm!(
            "mrs {0}, BASEPRI",
            out(reg) basepri,
            options(nomem, nostack, preserves_flags)
        );
    }
    basepri as u8
}

/// Execute a closure with priority-based interrupt masking
///
/// Only interrupts with priority numerically lower (higher urgency) than
/// `priority` can preempt the closure.
///
/// # Safety
///
/// The priority value must be appropriate for your system configuration.
#[inline]
pub unsafe fn free_basepri<F, R>(priority: u8, f: F) -> R
where
    F: FnOnce() -> R,
{
    let old_basepri = priority_threshold();

    // Only raise the priority (lower numerical value)
    unsafe {
        asm!(
            "msr BASEPRI_MAX, {0}",
            in(reg) priority as u32,
            options(nomem, nostack, preserves_flags)
        );
    }

    let result = f();

    // Restore previous BASEPRI
    unsafe {
        asm!(
            "msr BASEPRI, {0}",
            in(reg) old_basepri as u32,
            options(nomem, nostack, preserves_flags)
        );
    }

    result
}
