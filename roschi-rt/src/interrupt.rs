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
//! use roschi_rt::interrupt;
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

use crate::register::primask;

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
