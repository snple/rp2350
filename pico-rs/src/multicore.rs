//! Multicore support for RP2350
//!
//! RP2350 has two ARM Cortex-M33 cores. Core 0 starts first and can launch Core 1.
//! This module provides functionality for:
//!
//! - Launching code on Core 1
//! - Inter-core communication via FIFO
//! - Doorbell interrupts for signaling between cores
//! - Spinlocks for synchronization
//!
//! ## Example: Launch Core 1
//!
//! ```ignore
//! use pico_rs::multicore::{Multicore, Stack};
//!
//! // Define a stack for Core 1
//! static mut CORE1_STACK: Stack<4096> = Stack::new();
//!
//! fn core1_main() -> ! {
//!     // Core 1 code runs here
//!     loop {
//!         pico_rs_rt::asm::wfe();
//!     }
//! }
//!
//! fn main() {
//!     let mut mc = Multicore::new();
//!
//!     // Launch Core 1
//!     unsafe {
//!         mc.launch_core1(core1_main, &mut CORE1_STACK);
//!     }
//!
//!     // Core 0 continues here
//! }
//! ```
//!
//! ## Example: Inter-core Communication
//!
//! ```ignore
//! use pico_rs::multicore::Fifo;
//!
//! // On Core 0:
//! let fifo = Fifo::new();
//! fifo.write(0x12345678);  // Send to Core 1
//!
//! // On Core 1:
//! let fifo = Fifo::new();
//! let value = fifo.read();  // Receive from Core 0
//! ```

use crate::pac::{PPB, PSM, SIO};
use core::sync::atomic::{Ordering, compiler_fence};

// ============================================================================
// Constants
// ============================================================================

/// Core ID for Core 0
pub const CORE0: u32 = 0;

/// Core ID for Core 1
pub const CORE1: u32 = 1;

// ============================================================================
// Stack Definition
// ============================================================================

/// A stack for use by Core 1
///
/// The stack must be statically allocated and have at least 256 bytes.
/// The generic parameter `N` specifies the stack size in bytes.
///
/// # Example
///
/// ```ignore
/// static mut CORE1_STACK: Stack<4096> = Stack::new();
/// ```
#[repr(C, align(8))]
pub struct Stack<const N: usize> {
    data: [u8; N],
}

impl<const N: usize> Stack<N> {
    /// Create a new uninitialized stack
    pub const fn new() -> Self {
        Self { data: [0; N] }
    }

    /// Get the top of the stack (end address)
    ///
    /// ARM uses a full descending stack, so SP starts at the top.
    #[inline]
    pub fn top(&self) -> *mut u32 {
        // Stack pointer must be 8-byte aligned
        let end = self.data.as_ptr() as usize + N;
        (end & !7) as *mut u32
    }
}

impl<const N: usize> Default for Stack<N> {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Core Identification
// ============================================================================

/// Get the current core's ID (0 or 1)
#[inline]
pub fn core_id() -> u32 {
    let sio = unsafe { &*SIO::PTR };
    sio.cpuid.cpuid()
}

/// Returns `true` if running on Core 0
#[inline]
pub fn is_core0() -> bool {
    core_id() == CORE0
}

/// Returns `true` if running on Core 1
#[inline]
pub fn is_core1() -> bool {
    core_id() == CORE1
}

// ============================================================================
// Inter-core FIFO
// ============================================================================

/// Inter-core FIFO for communication between cores
///
/// Each core has a separate view of the FIFO:
/// - Writing puts data into the TX FIFO (sent to the other core)
/// - Reading takes data from the RX FIFO (received from the other core)
///
/// The FIFO can hold up to 8 words in each direction.
pub struct Fifo {
    sio: &'static pico_rs_pac::sio::RegisterBlock,
}

impl Fifo {
    /// Create a new FIFO instance
    ///
    /// # Safety
    /// This does not track ownership. The caller must ensure proper
    /// synchronization when accessing from multiple contexts.
    #[inline]
    pub fn new() -> Self {
        Self {
            sio: unsafe { &*SIO::PTR },
        }
    }

    // ========================================================================
    // Status Methods
    // ========================================================================

    /// Check if the RX FIFO has data available
    #[inline]
    pub fn is_read_ready(&self) -> bool {
        self.sio.fifo_st.vld()
    }

    /// Check if the TX FIFO has space for writing
    #[inline]
    pub fn is_write_ready(&self) -> bool {
        self.sio.fifo_st.rdy()
    }

    /// Check if RX FIFO was read when empty (read-on-empty error)
    #[inline]
    pub fn read_error(&self) -> bool {
        self.sio.fifo_st.roe()
    }

    /// Check if TX FIFO was written when full (write-on-full error)
    #[inline]
    pub fn write_error(&self) -> bool {
        self.sio.fifo_st.wof()
    }

    /// Clear the read-on-empty error flag
    #[inline]
    pub fn clear_read_error(&self) {
        // Write 1 to clear ROE bit
        self.sio.fifo_st.write_bits(0x08);
    }

    /// Clear the write-on-full error flag
    #[inline]
    pub fn clear_write_error(&self) {
        // Write 1 to clear WOF bit
        self.sio.fifo_st.write_bits(0x04);
    }

    /// Clear all error flags
    #[inline]
    pub fn clear_errors(&self) {
        self.sio.fifo_st.write_bits(0x0C);
    }

    // ========================================================================
    // Non-blocking Operations
    // ========================================================================

    /// Try to write a value to the FIFO (non-blocking)
    ///
    /// Returns `Ok(())` if the write succeeded, or `Err(value)` if the FIFO was full.
    #[inline]
    pub fn try_write(&self, value: u32) -> Result<(), u32> {
        if self.is_write_ready() {
            self.sio.fifo_wr.write_bits(value);
            // Ensure the write is visible to the other core
            pico_rs_rt::asm::sev();
            Ok(())
        } else {
            Err(value)
        }
    }

    /// Try to read a value from the FIFO (non-blocking)
    ///
    /// Returns `Some(value)` if data was available, or `None` if the FIFO was empty.
    #[inline]
    pub fn try_read(&self) -> Option<u32> {
        if self.is_read_ready() {
            Some(self.sio.fifo_rd.read_bits())
        } else {
            None
        }
    }

    // ========================================================================
    // Blocking Operations
    // ========================================================================

    /// Write a value to the FIFO (blocking)
    ///
    /// Blocks until the FIFO has space and writes the value.
    #[inline]
    pub fn write(&self, value: u32) {
        while !self.is_write_ready() {
            pico_rs_rt::asm::wfe();
        }
        self.sio.fifo_wr.write_bits(value);
        pico_rs_rt::asm::sev();
    }

    /// Read a value from the FIFO (blocking)
    ///
    /// Blocks until the FIFO has data and returns the value.
    #[inline]
    pub fn read(&self) -> u32 {
        while !self.is_read_ready() {
            pico_rs_rt::asm::wfe();
        }
        self.sio.fifo_rd.read_bits()
    }

    // ========================================================================
    // Utility Methods
    // ========================================================================

    /// Drain the RX FIFO, discarding all values
    #[inline]
    pub fn drain(&self) {
        while self.is_read_ready() {
            let _ = self.sio.fifo_rd.read_bits();
        }
    }
}

impl Default for Fifo {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Doorbell Interrupts
// ============================================================================

/// Doorbell interrupts for signaling between cores
///
/// RP2350 provides 8 doorbell bits (0-7) that can be used to trigger
/// interrupts on the other core. This is more efficient than using
/// the FIFO for simple signaling.
pub struct Doorbell {
    sio: &'static pico_rs_pac::sio::RegisterBlock,
}

impl Doorbell {
    /// Number of available doorbell bits
    pub const COUNT: u8 = 8;

    /// Create a new Doorbell instance
    #[inline]
    pub fn new() -> Self {
        Self {
            sio: unsafe { &*SIO::PTR },
        }
    }

    /// Set doorbell bits on the opposite core (trigger interrupt)
    ///
    /// Write 1 to set the corresponding doorbell interrupt on the other core.
    #[inline]
    pub fn set_remote(&self, mask: u8) {
        self.sio.doorbell_out_set.write_bits(mask as u32);
    }

    /// Clear doorbell bits that were posted to the opposite core
    #[inline]
    pub fn clear_remote(&self, mask: u8) {
        self.sio.doorbell_out_clr.write_bits(mask as u32);
    }

    /// Set doorbell bits on this core (self-trigger)
    #[inline]
    pub fn set_local(&self, mask: u8) {
        self.sio.doorbell_in_set.write_bits(mask as u32);
    }

    /// Clear doorbell bits on this core
    #[inline]
    pub fn clear_local(&self, mask: u8) {
        self.sio.doorbell_in_clr.write_bits(mask as u32);
    }

    /// Get the status of doorbell interrupts on this core
    ///
    /// Returns a bitmask of pending doorbell interrupts.
    #[inline]
    pub fn status(&self) -> u8 {
        // Reading doorbell_in_clr returns the current status
        self.sio.doorbell_in_clr.read_bits() as u8
    }

    /// Check if a specific doorbell is pending on this core
    #[inline]
    pub fn is_pending(&self, doorbell: u8) -> bool {
        debug_assert!(doorbell < Self::COUNT);
        (self.status() & (1 << doorbell)) != 0
    }

    /// Signal the other core using doorbell 0
    #[inline]
    pub fn signal(&self) {
        self.set_remote(1);
    }

    /// Wait for a signal from the other core (doorbell 0)
    #[inline]
    pub fn wait(&self) {
        while !self.is_pending(0) {
            pico_rs_rt::asm::wfe();
        }
        self.clear_local(1);
    }
}

impl Default for Doorbell {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Multicore Controller
// ============================================================================

/// Multicore controller for launching and managing Core 1
///
/// This struct provides methods for launching code on Core 1 and
/// resetting the core.
pub struct Multicore {
    psm: &'static pico_rs_pac::psm::RegisterBlock,
    ppb: &'static pico_rs_pac::ppb::RegisterBlock,
}

impl Multicore {
    /// Create a new Multicore instance
    ///
    /// Should only be called from Core 0.
    pub fn new() -> Self {
        debug_assert!(is_core0(), "Multicore should be created on Core 0");
        Self {
            psm: unsafe { &*PSM::PTR },
            ppb: unsafe { &*PPB::PTR },
        }
    }

    /// Launch code on Core 1
    ///
    /// This function starts Core 1 executing the given entry function with
    /// the provided stack. Core 1 will use the same vector table as Core 0.
    ///
    /// # Safety
    ///
    /// - The stack must have a static lifetime and not be used elsewhere
    /// - The entry function must never return
    /// - This function should only be called once
    ///
    /// # Arguments
    ///
    /// * `entry` - The entry point function for Core 1 (must be `fn() -> !`)
    /// * `stack` - The stack for Core 1 to use
    ///
    /// # Example
    ///
    /// ```ignore
    /// static mut CORE1_STACK: Stack<4096> = Stack::new();
    ///
    /// fn core1_entry() -> ! {
    ///     loop {
    ///         pico_rs_rt::asm::wfe();
    ///     }
    /// }
    ///
    /// let mut mc = Multicore::new();
    /// unsafe {
    ///     mc.launch_core1(core1_entry, &raw mut CORE1_STACK);
    /// }
    /// ```
    #[inline(never)]
    pub unsafe fn launch_core1<const N: usize>(&mut self, entry: fn() -> !, stack: *mut Stack<N>) {
        // Ensure stack is large enough (minimum 256 bytes)
        assert!(N >= 256, "Core 1 stack must be at least 256 bytes");

        // SAFETY: The caller guarantees that stack points to valid memory
        let stack_ptr = unsafe { (*stack).top() as u32 };
        let entry_ptr = entry as *const () as u32;
        let vector_table = self.ppb.vtor.read_bits();

        // Launch Core 1 with the launch sequence
        // SAFETY: We are already in an unsafe fn, and parameters are validated above
        unsafe {
            self.launch_core1_raw(stack_ptr, entry_ptr, vector_table);
        }
    }

    /// Launch Core 1 with raw parameters
    ///
    /// This is a lower-level function that allows specifying all parameters directly.
    ///
    /// # Safety
    ///
    /// - All parameters must be valid
    /// - The stack pointer must point to valid memory
    /// - The entry point must be a valid function address
    /// - The vector table must be properly aligned
    pub unsafe fn launch_core1_raw(&mut self, stack_ptr: u32, entry: u32, vector_table: u32) {
        // Get the FIFO for sending commands to Core 1
        let fifo = Fifo::new();

        // The launch sequence sends commands in this order:
        // 0, 0, 1, vector_table, stack_ptr, entry
        //
        // Core 1's boot ROM waits for this sequence and then:
        // 1. Sets VTOR to vector_table
        // 2. Sets SP to stack_ptr
        // 3. Jumps to entry
        let cmd_sequence: [u32; 6] = [0, 0, 1, vector_table, stack_ptr, entry];

        let mut seq = 0usize;
        loop {
            let cmd = cmd_sequence[seq];

            // For sync commands (0), drain the FIFO first and send SEV
            if cmd == 0 {
                fifo.drain();
                // Execute a SEV as core 1 may be waiting for FIFO space via WFE
                pico_rs_rt::asm::sev();
            }

            // Send the command (blocking)
            fifo.write(cmd);

            // Wait for response (blocking)
            let response = fifo.read();

            // Move to next state on correct response (echo'd value)
            // otherwise start over from the beginning
            if cmd == response {
                seq += 1;
            } else {
                seq = 0;
            }

            if seq >= cmd_sequence.len() {
                break;
            }
        }

        // Ensure all writes are complete before continuing
        compiler_fence(Ordering::SeqCst);
    }

    /// Reset Core 1
    ///
    /// This forces Core 1 into reset state. After calling this,
    /// Core 1 can be relaunched with `launch_core1`.
    ///
    /// The function blocks until Core 1 has completed reset and is ready
    /// to accept launch commands.
    pub fn reset_core1(&mut self) {
        // Force PROC1 into reset by setting FRCE_OFF
        self.psm.frce_off.set_proc1();

        // Wait for Core 1 to be in reset (done bit goes low)
        while self.psm.done.proc1() {
            pico_rs_rt::asm::nop();
        }

        // Clear the force-off to bring Core 1 out of reset
        self.psm.frce_off.clr_proc1();

        // Core 1 will now:
        // 1. Boot from ROM
        // 2. Drain its own FIFO
        // 3. Push a 0 to our FIFO to signal it's ready

        let fifo = Fifo::new();

        // Wait for Core 1's ready signal (blocking read of 0)
        let response = fifo.read();
        debug_assert_eq!(response, 0, "Core 1 should send 0 after reset");
    }

    /// Check if Core 1 is running (not in reset)
    #[inline]
    pub fn is_core1_running(&self) -> bool {
        self.psm.done.proc1()
    }
}

impl Default for Multicore {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Spinlock Re-export
// ============================================================================

// Spinlocks are already implemented in sio.rs, re-export for convenience
pub use crate::sio::{SPINLOCK_COUNT, Spinlock, SpinlockGuard};

// ============================================================================
// Utility Functions
// ============================================================================

/// Execute a closure on the current core while holding a spinlock
///
/// This provides a simple way to protect shared data between cores.
///
/// # Type Parameters
///
/// * `N` - The spinlock number (0-31)
/// * `F` - The closure type
/// * `R` - The return type
///
/// # Example
///
/// ```ignore
/// let result = multicore::with_spinlock::<0, _, _>(|| {
///     // Critical section protected by spinlock 0
///     shared_data.modify()
/// });
/// ```
#[inline]
pub fn with_spinlock<const N: u8, F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    let _guard = Spinlock::<N>::acquire();
    f()
}

/// Send a value to the other core via FIFO
///
/// Convenience function for simple inter-core communication.
#[inline]
pub fn send(value: u32) {
    Fifo::new().write(value);
}

/// Receive a value from the other core via FIFO
///
/// Convenience function for simple inter-core communication.
/// Blocks until a value is available.
#[inline]
pub fn recv() -> u32 {
    Fifo::new().read()
}

/// Try to receive a value from the other core via FIFO
///
/// Returns `None` if no value is available.
#[inline]
pub fn try_recv() -> Option<u32> {
    Fifo::new().try_read()
}

/// Signal the other core using doorbell 0
#[inline]
pub fn signal_other() {
    Doorbell::new().signal();
}

/// Wait for a signal from the other core (doorbell 0)
#[inline]
pub fn wait_for_signal() {
    Doorbell::new().wait();
}
