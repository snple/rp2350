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
//! let fifo = Fifo::init();
//! fifo.write(0x12345678);  // Send to Core 1
//!
//! // On Core 1:
//! let fifo = Fifo::init();
//! let value = fifo.read();  // Receive from Core 0
//! ```

use core::sync::atomic::{Ordering, compiler_fence};

use crate::Steal;

// ============================================================================
// Core ID
// ============================================================================

/// RP2350 核心标识
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u32)]
pub enum CoreId {
    /// Core 0 (默认启动核心)
    Core0 = 0,
    /// Core 1 (需要手动启动)
    Core1 = 1,
}

impl CoreId {
    /// 从原始值转换，非 0/1 时返回 `None`
    #[inline]
    pub const fn from_raw(val: u32) -> Option<Self> {
        match val {
            0 => Some(Self::Core0),
            1 => Some(Self::Core1),
            _ => None,
        }
    }

    /// 转为 `usize`，适用于数组索引
    #[inline]
    pub const fn as_index(self) -> usize {
        self as usize
    }

    /// 返回另一个核心
    #[inline]
    pub const fn other(self) -> Self {
        match self {
            Self::Core0 => Self::Core1,
            Self::Core1 => Self::Core0,
        }
    }
}

impl core::fmt::Display for CoreId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Core0 => write!(f, "Core 0"),
            Self::Core1 => write!(f, "Core 1"),
        }
    }
}

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

/// 获取当前核心的 ID
#[inline]
pub fn core_id() -> CoreId {
    match unsafe { &*crate::sio::REGS }.cpuid.cpuid() {
        0 => CoreId::Core0,
        _ => CoreId::Core1,
    }
}

/// 当前是否在 Core 0 上运行
#[inline]
pub fn is_core0() -> bool {
    core_id() == CoreId::Core0
}

/// 当前是否在 Core 1 上运行
#[inline]
pub fn is_core1() -> bool {
    core_id() == CoreId::Core1
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
    /// Create a handle to the inter-core FIFO.
    ///
    /// The FIFO is a hardware peripheral that is always available; this
    /// constructor is safe and does not require any PAC peripheral.
    #[inline]
    pub fn init() -> Self {
        Self {
            sio: unsafe { &*crate::sio::REGS },
        }
    }

    /// Obtain a handle to the inter-core FIFO without runtime checks.
    ///
    /// # Safety
    /// This does not track ownership. The caller must ensure proper
    /// synchronization when accessing from multiple contexts.
    #[inline]
    pub unsafe fn steal() -> Steal<Self> {
        Steal(Self {
            sio: unsafe { &*crate::sio::REGS },
        })
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

// ============================================================================
// Doorbell Interrupts
// ============================================================================

/// Inter-core doorbell for low-latency signaling
///
/// RP2350 provides **8 doorbell bits** (0–7) per core, implemented via four
/// SIO registers on each core.  They are designed for *event notification*
/// rather than bulk data transfer.
///
/// # Hardware model
///
/// ```text
///          Core 0                              Core 1
///   ┌──────────────────┐              ┌──────────────────┐
///   │  doorbell_in_set  │◄── set ──────│ doorbell_out_set │
///   │  doorbell_in_clr  │──► status    │ doorbell_out_clr │
///   │  doorbell_out_set │───── set ──►│  doorbell_in_set  │
///   │  doorbell_out_clr │              │  doorbell_in_clr  │
///   └──────────────────┘              └──────────────────┘
/// ```
///
/// * **out** registers — written by *this* core, visible on the *other* core.
///   `out_set` asserts bits; `out_clr` de-asserts them.
/// * **in** registers — written by the *other* core, read by *this* core.
///   `in_set` / `in_clr` work the same way but represent incoming signals.
///   Reading `in_clr` returns the current pending bitmask.
///
/// A write to `doorbell_out_set` on Core 0 immediately appears as a pending
/// bit in `doorbell_in_clr` on Core 1, and also triggers the `SIO_IRQ_BELL`
/// interrupt on Core 1 if enabled.
///
/// # Usage patterns
///
/// * **Simple event** — use [`signal()`](Self::signal) / [`wait()`](Self::wait)
///   to notify the other core that work is ready.  Each doorbell index acts as
///   a separate event channel.
/// * **Multiple events** — assign different doorbell indices to different event
///   types (e.g. index 0 = "data ready", index 1 = "error occurred").
/// * **Tiny data** — write an arbitrary 8-bit pattern with
///   [`set_remote()`](Self::set_remote) and read it back on the other side
///   with [`status()`](Self::status).  Note that this is a *one-word* channel
///   with no FIFO — if the receiver hasn't consumed the previous value, new
///   writes will merge via bitwise OR.
///
/// # Comparison with FIFO
///
/// | Feature          | Doorbell                          | FIFO                        |
/// |------------------|-----------------------------------|-----------------------------|
/// | Data width       | 8 bits (bitmask)                  | 32 bits per word            |
/// | Queue depth      | 1 (per bit, sticky until cleared) | 8 words each direction      |
/// | Interrupt source | `SIO_IRQ_BELL`                    | `SIO_IRQ_FIFO`              |
/// | Best for         | Event notification, flags         | Streaming data, commands    |
pub struct Doorbell {
    sio: &'static pico_rs_pac::sio::RegisterBlock,
}

impl Doorbell {
    /// Number of available doorbell bits
    pub const COUNT: u8 = 8;

    /// Create a handle to the doorbell hardware.
    ///
    /// The doorbell is always available; this constructor is safe and does
    /// not require any PAC peripheral.
    #[inline]
    pub fn init() -> Self {
        Self {
            sio: unsafe { &*crate::sio::REGS },
        }
    }

    /// Obtain a handle to the doorbell hardware without runtime checks.
    ///
    /// # Safety
    ///
    /// The caller must ensure no data races occur when accessing doorbell
    /// registers from multiple contexts.
    #[inline]
    pub unsafe fn steal() -> Steal<Self> {
        Steal(Self {
            sio: unsafe { &*crate::sio::REGS },
        })
    }

    /// Assert doorbell bits on the **other** core
    ///
    /// Writing `1` to a bit in `mask` sets the corresponding doorbell on the
    /// remote core.  This also triggers `SIO_IRQ_BELL` on the remote core if
    /// the interrupt is enabled.
    ///
    /// Bits written as `0` are ignored — only `1` bits have effect.
    #[inline]
    pub fn set_remote(&self, mask: u8) {
        self.sio.doorbell_out_set.write_bits(mask as u32);
    }

    /// De-assert doorbell bits that were previously posted to the other core
    ///
    /// Writing `1` to a bit in `mask` clears the corresponding doorbell on the
    /// remote core.  Bits written as `0` are ignored.
    #[inline]
    pub fn clear_remote(&self, mask: u8) {
        self.sio.doorbell_out_clr.write_bits(mask as u32);
    }

    /// Assert doorbell bits on **this** core (self-trigger)
    ///
    /// This is typically used for testing or to trigger the local
    /// `SIO_IRQ_BELL` handler from software.
    #[inline]
    pub fn set_local(&self, mask: u8) {
        self.sio.doorbell_in_set.write_bits(mask as u32);
    }

    /// De-assert doorbell bits on **this** core
    ///
    /// Call this from an interrupt handler or after a [`wait()`](Self::wait)
    /// to acknowledge and clear the pending bits.
    #[inline]
    pub fn clear_local(&self, mask: u8) {
        self.sio.doorbell_in_clr.write_bits(mask as u32);
    }

    /// Read the pending doorbell status on this core
    ///
    /// Returns a bitmask where each set bit represents a doorbell that has
    /// been signaled by the other core and not yet cleared.  Reading
    /// `doorbell_in_clr` returns the current state.
    #[inline]
    pub fn status(&self) -> u8 {
        self.sio.doorbell_in_clr.read_bits() as u8
    }

    /// Check if a specific doorbell index is pending on this core
    ///
    /// Returns `true` if the given doorbell bit is currently asserted.
    ///
    /// # Panics
    ///
    /// Panics in debug if `doorbell >= Doorbell::COUNT`.
    #[inline]
    pub fn pending(&self, doorbell: u8) -> bool {
        debug_assert!(doorbell < Self::COUNT);
        (self.status() & (1 << doorbell)) != 0
    }

    /// Signal the other core on a specific doorbell
    ///
    /// Sets the given doorbell bit on the remote core, waking it from
    /// [`wait()`](Self::wait) on the same doorbell index.
    ///
    /// This is the simplest way to send a notification.  For more control
    /// (e.g. setting multiple bits at once), use [`set_remote()`](Self::set_remote).
    ///
    /// # Panics
    ///
    /// Panics in debug if `doorbell >= Doorbell::COUNT`.
    #[inline]
    pub fn signal(&self, doorbell: u8) {
        debug_assert!(doorbell < Self::COUNT);
        self.set_remote(1 << doorbell);
    }

    /// Wait for a signal from the other core on a specific doorbell
    ///
    /// Blocks until the given doorbell bit is set by the other core, then
    /// clears it.  Uses `WFE` for low-power waiting; spurious wakeups are
    /// handled by re-checking the pending status.
    ///
    /// After this returns, the doorbell bit is cleared and ready for the
    /// next signal.  For non-blocking polling, use
    /// [`is_pending()`](Self::is_pending) + [`clear_local()`](Self::clear_local).
    ///
    /// # Panics
    ///
    /// Panics in debug if `doorbell >= Doorbell::COUNT`.
    #[inline]
    pub fn wait(&self, doorbell: u8) {
        debug_assert!(doorbell < Self::COUNT);
        while !self.pending(doorbell) {
            pico_rs_rt::asm::wfe();
        }
        self.clear_local(1 << doorbell);
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
    psm: pico_rs_pac::PSM,
    ppb: pico_rs_pac::PPB,
}

impl Multicore {
    /// Initialize the Multicore driver.
    ///
    /// Should only be called from Core 0.
    pub fn init(psm: pico_rs_pac::PSM, ppb: pico_rs_pac::PPB) -> Self {
        debug_assert!(is_core0(), "Multicore should be created on Core 0");
        Self { psm, ppb }
    }

    /// Obtain a Multicore handle without ownership checks.
    ///
    /// # Safety
    ///
    /// The caller must ensure [`Multicore::init()`] has been called first
    /// and that no data races occur. This should only be used from Core 0.
    #[inline]
    pub unsafe fn steal() -> Steal<Self> {
        Steal(Self {
            psm: unsafe { pico_rs_pac::PSM::steal() },
            ppb: unsafe { pico_rs_pac::PPB::steal() },
        })
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
    /// let mut mc = Multicore::init(psm, ppb);
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
        let fifo = unsafe { Fifo::steal() };

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

        let fifo = unsafe { Fifo::steal() };

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
