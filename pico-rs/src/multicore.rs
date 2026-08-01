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
//!     let mut mc = Multicore::init(psm, ppb);
//!
//!     // Launch Core 1 (automatically resets it first)
//!     unsafe {
//!         mc.launch_core1(core1_main, &raw mut CORE1_STACK);
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

/// RP2350 core identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u32)]
pub enum CoreId {
    /// Core 0 (default boot core)
    Core0 = 0,
    /// Core 1 (must be started manually)
    Core1 = 1,
}

impl CoreId {
    /// Convert from a raw value; returns `None` for anything other than 0/1
    #[inline]
    pub const fn from_raw(val: u32) -> Option<Self> {
        match val {
            0 => Some(Self::Core0),
            1 => Some(Self::Core1),
            _ => None,
        }
    }

    /// Convert to `usize`, suitable for array indexing
    #[inline]
    pub const fn as_index(self) -> usize {
        self as usize
    }

    /// Return the other core
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

/// Get the ID of the current core
#[inline]
pub fn core_id() -> CoreId {
    match unsafe { &*crate::sio::REGS }.cpuid.cpuid() {
        0 => CoreId::Core0,
        _ => CoreId::Core1,
    }
}

/// Whether the current core is Core 0
#[inline]
pub fn is_core0() -> bool {
    core_id() == CoreId::Core0
}

/// Whether the current core is Core 1
#[inline]
pub fn is_core1() -> bool {
    core_id() == CoreId::Core1
}

// ============================================================================
// Inter-core FIFO
// ============================================================================

/// Inter-core FIFO for communication between cores
///
/// The SIO contains **two** physical FIFOs, each 32 bits wide and 8 words deep:
///
/// - **FIFO 0→1** — written by Core 0, read by Core 1
/// - **FIFO 1→0** — written by Core 1, read by Core 0
///
/// Each core sees one FIFO as its TX (outgoing, via [`write()`](Self::write) /
/// [`try_write()`](Self::try_write)) and the other as its RX (incoming, via
/// [`read()`](Self::read) / [`try_read()`](Self::try_read)).
///
/// On Core 0: TX = FIFO 0→1 (write side), RX = FIFO 1→0 (read side).
/// On Core 1: TX = FIFO 1→0 (write side), RX = FIFO 0→1 (read side).
///
/// # Usage on both cores
///
/// The `Fifo` struct works identically on both cores — the hardware
/// automatically routes `FIFO_WR` / `FIFO_RD` to the correct physical FIFO.
/// Simply call [`Fifo::init()`] on each core:
///
/// ```ignore
/// // Core 0 — sends 0x42, then waits for a reply
/// let fifo = Fifo::init();
/// fifo.write(0x42);
/// let reply = fifo.read();
///
/// // Core 1 — blocks until data arrives, then responds
/// let fifo = Fifo::init();
/// let cmd = fifo.read();
/// fifo.write(cmd + 1);
/// ```
///
/// # Interrupts
///
/// Each core has its own independent FIFO interrupt line (same IRQ number,
/// separate NVIC): `SIO_IRQ_FIFO` (IRQ 25) for secure, and
/// `SIO_IRQ_FIFO_NS` (IRQ 27) for non-secure.  The interrupt is the logical OR
/// of the VLD, ROE, and WOF bits in [`FifoSt`](pico_rs_pac::sio::FifoSt) — it
/// fires whenever data arrives **or** a sticky error flag is set.
///
/// To clear the interrupt:
/// - **VLD**: read from the RX FIFO until it is empty
/// - **ROE/WOF**: write any value to `FIFO_ST` (this clears *both* sticky flags
///   at once — they cannot be cleared independently)
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

    /// Check if RX FIFO was read when empty (sticky read-on-empty error).
    ///
    /// Once set, this flag remains asserted until cleared by [`clear_errors()`](Self::clear_errors).
    #[inline]
    pub fn read_error(&self) -> bool {
        self.sio.fifo_st.roe()
    }

    /// Check if TX FIFO was written when full (sticky write-on-full error).
    ///
    /// Once set, this flag remains asserted until cleared by [`clear_errors()`](Self::clear_errors).
    #[inline]
    pub fn write_error(&self) -> bool {
        self.sio.fifo_st.wof()
    }

    /// Clear both sticky error flags (ROE and WOF).
    ///
    /// **Hardware note:** writing *any* value to `FIFO_ST` clears **both** ROE
    /// and WOF simultaneously — they cannot be cleared independently.
    /// Call this from the FIFO interrupt handler if either error flag is set.
    #[inline]
    pub fn clear_errors(&self) {
        // Write any value to FIFO_ST to clear both ROE and WOF
        self.sio.fifo_st.write_bits(0);
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
/// RP2350 provides **8 doorbell bits** (0–7) in each direction, backed by
/// four SIO registers per core.  Unlike the FIFO, doorbells are designed for
/// *accumulative* events — they can be posted multiple times but only need to
/// be handled once, and can be processed in any order.
///
/// Each core has its own independent doorbell interrupt line (same IRQ
/// number, separate NVIC): `SIO_IRQ_BELL` (IRQ 26) for secure, and
/// `SIO_IRQ_BELL_NS` (IRQ 28) for non-secure.  Each core can independently
/// enable the interrupt and register its own handler.  The interrupt fires
/// when the opposite core writes a non-zero value to its `DOORBELL_OUT_SET`,
/// and remains asserted **until all 8 bits are cleared**.
///
/// The typical interrupt-handler flow is:
///
/// 1. Read [`status()`](Self::status) (= `DOORBELL_IN_CLR`) to get the mask
///    of currently-pending doorbell bits
/// 2. Process each asserted bit
/// 3. Acknowledge by writing the same mask to
///    [`clear_local()`](Self::clear_local) (= `DOORBELL_IN_CLR`)
///
/// # Usage on both cores
///
/// Like [`Fifo`], the `Doorbell` struct works identically on both cores —
/// the hardware automatically routes `doorbell_out_*` / `doorbell_in_*` to
/// the correct direction:
///
/// ```ignore
/// // Core 0 — signal Core 1 that data is ready, then wait for ack
/// let db = Doorbell::init();
/// db.signal(0);       // fire doorbell 0 on Core 1
/// db.wait(1);         // wait for Core 1 to ring doorbell 1
///
/// // Core 1 — wait for notification, do work, then acknowledge
/// let db = Doorbell::init();
/// db.wait(0);         // block until Core 0 rings doorbell 0
/// // ... process data ...
/// db.signal(1);       // tell Core 0 we're done
/// ```
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
///   `out_set` asserts bits; `out_clr` de-asserts them.  Reading either
///   `out_set` or `out_clr` returns the **same** value — the current status
///   of doorbells posted to the remote core.
/// * **in** registers — written by the *other* core, read by *this* core.
///   `in_set` / `in_clr` work the same way but represent incoming signals.
///   Reading either returns the current pending bitmask.
///
/// A write to `doorbell_out_set` on Core 0 immediately asserts the
/// corresponding bits in `doorbell_in_clr` on Core 1, and raises Core 1's
/// `SIO_IRQ_BELL` interrupt if enabled.
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
/// | Interrupt source | `SIO_IRQ_BELL` (IRQ 26)           | `SIO_IRQ_FIFO` (IRQ 25)     |
/// | Non-secure IRQ   | `SIO_IRQ_BELL_NS` (IRQ 28)        | `SIO_IRQ_FIFO_NS` (IRQ 27)  |
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
    /// remote core.  Writing *any* non-zero value raises the remote core's
    /// `SIO_IRQ_BELL` interrupt (IRQ 26) if enabled.
    ///
    /// Bits written as `0` are ignored — only `1` bits have effect.
    ///
    /// After calling this, use [`remote_status()`](Self::remote_status) to
    /// check which doorbells are still pending on the other core.
    #[inline]
    pub fn set_remote(&self, mask: u8) {
        self.sio.doorbell_out_set.write_bits(mask as u32);
    }

    /// De-assert doorbell bits that were previously posted to the other core
    ///
    /// Writing `1` to a bit in `mask` clears the corresponding doorbell on the
    /// remote core.  Bits written as `0` are ignored.
    ///
    /// # ⚠️ Race condition warning
    ///
    /// This register is intended for **debugging and initialisation only**.
    /// The normal flow is for the *receiving* core to acknowledge events by
    /// writing to its own [`clear_local()`](Self::clear_local).  Using
    /// `clear_remote` can race with the remote core's interrupt handler and
    /// should generally be avoided in production code.
    #[inline]
    pub fn clear_remote(&self, mask: u8) {
        self.sio.doorbell_out_clr.write_bits(mask as u32);
    }

    /// Read the status of doorbells currently posted to the **other** core
    ///
    /// Returns the same value that the remote core would see when reading its
    /// own [`status()`](Self::status).  This is useful for diagnostics: you can
    /// check which doorbells have been asserted on the other core but not yet
    /// acknowledged.
    ///
    /// Both `DOORBELL_OUT_SET` and `DOORBELL_OUT_CLR` return the same value,
    /// so this method reads either one (they are equivalent).
    #[inline]
    pub fn remote_status(&self) -> u8 {
        self.sio.doorbell_out_set.read_bits() as u8
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
    pub fn is_pending(&self, doorbell: u8) -> bool {
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
    /// [`pending()`](Self::pending) + [`clear_local()`](Self::clear_local).
    ///
    /// # Panics
    ///
    /// Panics in debug if `doorbell >= Doorbell::COUNT`.
    #[inline]
    pub fn wait(&self, doorbell: u8) {
        debug_assert!(doorbell < Self::COUNT);
        while !self.is_pending(doorbell) {
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
/// Core 1 is automatically reset before launching, so there is no need to
/// call [`reset_core1()`](Self::reset_core1) separately (unless you want to
/// hold Core 1 in reset without launching it immediately).
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
        debug_assert!(is_core0(), "Multicore should be created on Core 0");
        Steal(Self {
            psm: unsafe { pico_rs_pac::PSM::steal() },
            ppb: unsafe { pico_rs_pac::PPB::steal() },
        })
    }

    /// Launch code on Core 1
    ///
    /// Resets Core 1 (if it was previously running), then starts it executing
    /// the given entry function with the provided stack.  Core 1 uses the
    /// same vector table as Core 0.
    ///
    /// # Safety
    ///
    /// - The stack must have a static lifetime and not be used elsewhere
    /// - The entry function must never return
    /// - This function should only be called once (call [`reset_core1()`](Self::reset_core1) + launch again to restart)
    ///
    /// # Example
    ///
    /// ```ignore
    /// static mut CORE1_STACK: Stack<4096> = Stack::new();
    ///
    /// fn core1_entry() -> ! {
    ///     loop { pico_rs_rt::asm::wfe(); }
    /// }
    ///
    /// let mut mc = Multicore::init(psm, ppb);
    /// unsafe { mc.launch_core1(core1_entry, &raw mut CORE1_STACK); }
    /// ```
    #[inline(never)]
    pub unsafe fn launch_core1<const N: usize>(&mut self, entry: fn() -> !, stack: *mut Stack<N>) {
        // Ensure stack is large enough (minimum 256 bytes)
        assert!(N >= 256, "Core 1 stack must be at least 256 bytes");

        // Reset Core 1 to a known state before launching.
        // This is safe to call even if Core 1 was never launched —
        // the bootrom handshake ensures both sides agree on the state.
        self.reset_core1();

        // SAFETY: The caller guarantees that stack points to valid memory
        let stack_ptr = unsafe { (*stack).top() as u32 };
        let entry_ptr = entry as *const () as u32;
        let vector_table = self.ppb.vtor.read_bits();

        // SAFETY: Parameters are validated above, Core 1 is in reset state
        unsafe {
            self.launch_core1_raw(stack_ptr, entry_ptr, vector_table);
        }
    }

    /// Launch Core 1 with raw parameters.
    ///
    /// Lower-level than [`launch_core1()`](Self::launch_core1) — expects
    /// Core 1 to already be in reset state (call [`reset_core1()`](Self::reset_core1)
    /// first unless using [`launch_core1()`](Self::launch_core1) which does it for you).
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
