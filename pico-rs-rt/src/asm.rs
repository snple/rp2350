//! Cortex-M33 Assembly Instructions
//!
//! Low-level assembly instructions for the Cortex-M33 processor.
//! These are implemented using inline assembly for optimal performance.
//!
//! ## Memory Barriers
//!
//! - `dmb()` - Data Memory Barrier
//! - `dsb()` - Data Synchronization Barrier
//! - `isb()` - Instruction Synchronization Barrier
//!
//! ## Control Instructions
//!
//! - `nop()` - No operation
//! - `wfi()` - Wait for interrupt
//! - `wfe()` - Wait for event
//! - `sev()` - Send event
//! - `bkpt()` - Breakpoint
//! - `udf()` - Undefined instruction (causes exception)

use core::arch::asm;

// =============================================================================
// Basic Instructions
// =============================================================================

/// No operation
///
/// Useful to prevent delay loops from being optimized away.
#[inline(always)]
pub fn nop() {
    unsafe { asm!("nop", options(nomem, nostack, preserves_flags)) }
}

/// Wait for interrupt
///
/// Puts the processor into a low-power sleep state until an interrupt occurs.
/// This instruction only returns after an interrupt is received.
#[inline(always)]
pub fn wfi() {
    unsafe { asm!("wfi", options(nomem, nostack, preserves_flags)) }
}

/// Wait for event
///
/// If the event register is clear, puts the processor into a low-power state
/// until either an event or an interrupt occurs. If the event register is set,
/// clears it and returns immediately.
#[inline(always)]
pub fn wfe() {
    unsafe { asm!("wfe", options(nomem, nostack, preserves_flags)) }
}

/// Send event
///
/// Sets the event register on all processors in a multiprocessor system.
/// This can be used to wake a processor that is waiting on WFE.
#[inline(always)]
pub fn sev() {
    unsafe { asm!("sev", options(nomem, nostack, preserves_flags)) }
}

/// Breakpoint
///
/// Triggers a debug breakpoint. If no debugger is connected, this causes
/// a HardFault exception.
#[inline(always)]
pub fn bkpt() {
    unsafe { asm!("bkpt", options(nomem, nostack, preserves_flags)) }
}

/// Undefined instruction
///
/// Causes an undefined instruction exception. This can be used as a
/// `panic!`-like abort mechanism that will never return.
#[inline(always)]
pub fn udf() -> ! {
    unsafe {
        asm!("udf #0", options(noreturn, nomem, nostack));
    }
}

// =============================================================================
// Memory Barriers
// =============================================================================

/// Data Memory Barrier
///
/// Ensures that all explicit memory accesses that appear in program order
/// before the DMB instruction are observed before any explicit memory accesses
/// that appear in program order after the DMB instruction.
///
/// Use this when you need to ensure memory operations are completed in order.
#[inline(always)]
pub fn dmb() {
    unsafe { asm!("dmb sy", options(nomem, nostack, preserves_flags)) }
}

/// Data Synchronization Barrier
///
/// Acts as a special kind of memory barrier. No instruction in program order
/// after this instruction can execute until this instruction completes.
/// This instruction completes when:
/// - All explicit memory accesses before this instruction are complete
/// - All cache and branch predictor maintenance before this instruction complete
///
/// Use this when you need a stronger guarantee than DMB, such as when
/// modifying memory-mapped registers that affect execution.
#[inline(always)]
pub fn dsb() {
    unsafe { asm!("dsb sy", options(nomem, nostack, preserves_flags)) }
}

/// Instruction Synchronization Barrier
///
/// Flushes the pipeline, so all instructions following the ISB are fetched
/// from cache or memory after the ISB instruction has completed.
///
/// Use this after modifying the instruction stream, such as when
/// self-modifying code or changing vector tables.
#[inline(always)]
pub fn isb() {
    unsafe { asm!("isb sy", options(nomem, nostack, preserves_flags)) }
}

// =============================================================================
// Delay
// =============================================================================

/// Block for at least `cycles` CPU cycles
///
/// This is implemented as a counted loop in assembly to provide a timing
/// reference that is independent of compiler optimizations. However, the
/// actual delay may be longer due to:
/// - Interrupts being serviced
/// - Memory wait states
/// - Cache behavior
///
/// # Arguments
///
/// * `cycles` - Minimum number of CPU cycles to delay
///
/// # Note
///
/// Each iteration takes approximately 4 cycles (on Cortex-M33), but this
/// may vary with cache and memory conditions.
#[inline(always)]
pub fn delay(cycles: u32) {
    // Each loop iteration: subs (1 cycle) + bne (1-3 cycles)
    // On Cortex-M33, this is approximately 3-4 cycles per iteration
    unsafe {
        asm!(
            "1:",
            "subs {0}, {0}, #1",
            "bne 1b",
            inout(reg) cycles => _,
            options(nomem, nostack),
        )
    }
}

// =============================================================================
// ARMv8-M Specific Instructions
// =============================================================================

/// Test Target
///
/// Queries the Security state and access permissions of a memory location.
/// Returns a Test Target Response Payload.
///
/// This instruction is available on ARMv8-M (Cortex-M33).
///
/// # Safety
///
/// The pointer is not dereferenced, only its address is queried.
#[inline(always)]
pub fn tt(addr: *mut u32) -> u32 {
    let result: u32;
    unsafe {
        asm!(
            "tt {0}, {1}",
            out(reg) result,
            in(reg) addr,
            options(nomem, nostack, preserves_flags),
        )
    }
    result
}

/// Test Target Unprivileged
///
/// Queries the Security state and access permissions of a memory location
/// for an unprivileged access to that location.
#[inline(always)]
pub fn ttt(addr: *mut u32) -> u32 {
    let result: u32;
    unsafe {
        asm!(
            "ttt {0}, {1}",
            out(reg) result,
            in(reg) addr,
            options(nomem, nostack, preserves_flags),
        )
    }
    result
}

/// Test Target Alternate Domain
///
/// Queries the Security state and access permissions for a Non-Secure access.
/// Only valid when executing in Secure state.
#[inline(always)]
pub fn tta(addr: *mut u32) -> u32 {
    let result: u32;
    unsafe {
        asm!(
            "tta {0}, {1}",
            out(reg) result,
            in(reg) addr,
            options(nomem, nostack, preserves_flags),
        )
    }
    result
}

/// Test Target Alternate Domain Unprivileged
///
/// Queries the Security state and access permissions for a Non-Secure
/// unprivileged access. Only valid when executing in Secure state.
#[inline(always)]
pub fn ttat(addr: *mut u32) -> u32 {
    let result: u32;
    unsafe {
        asm!(
            "ttat {0}, {1}",
            out(reg) result,
            in(reg) addr,
            options(nomem, nostack, preserves_flags),
        )
    }
    result
}

/// Branch and Exchange Non-secure
///
/// Branches to a Non-Secure address. Only valid when executing in Secure state.
///
/// # Safety
///
/// The caller must ensure the target address is valid Non-Secure code.
#[inline(always)]
pub unsafe fn bx_ns(addr: u32) {
    unsafe {
        asm!(
            "bxns {0}",
            in(reg) addr,
            options(nomem, nostack),
        )
    }
}

// =============================================================================
// Semihosting
// =============================================================================

/// Semihosting syscall
///
/// Performs a semihosting call using the BKPT instruction.
/// This is used by debugging tools to provide host-side services
/// like file I/O and console output.
///
/// # Arguments
///
/// * `nr` - Semihosting operation number (in r0)
/// * `arg` - Pointer to argument block (in r1)
///
/// # Returns
///
/// The result of the semihosting call (from r0)
///
/// # Safety
///
/// The `arg` parameter must be a valid pointer to properly formatted
/// semihosting data structures for the given operation number.
#[cfg(target_arch = "arm")]
#[inline(always)]
pub unsafe fn semihosting_syscall(nr: u32, arg: u32) -> u32 {
    let result: u32;
    // SAFETY: Caller guarantees arg is valid for the given operation
    unsafe {
        asm!(
            "bkpt #0xab",
            inout("r0") nr => result,
            in("r1") arg,
            options(nomem, nostack),
        );
    }
    result
}

/// Semihosting syscall using HLT instruction (ARMv8-M)
///
/// Performs a semihosting call using the HLT instruction introduced in ARMv8-M.
/// This is an alternative to BKPT for semihosting on Cortex-M33.
///
/// # Arguments
///
/// * `nr` - Semihosting operation number (in r0)
/// * `arg` - Pointer to argument block (in r1)
///
/// # Returns
///
/// The result of the semihosting call (from r0)
///
/// # Safety
///
/// The `arg` parameter must be a valid pointer to properly formatted
/// semihosting data structures for the given operation number.
#[cfg(target_arch = "arm")]
#[inline(always)]
pub unsafe fn semihosting_syscall_hlt(nr: u32, arg: u32) -> u32 {
    let result: u32;
    // SAFETY: Caller guarantees arg is valid for the given operation
    unsafe {
        asm!(
            "hlt #0x3c",
            inout("r0") nr => result,
            in("r1") arg,
            options(nomem, nostack),
        );
    }
    result
}

// =============================================================================
// Exclusive Access Instructions (ARMv8-M)
// =============================================================================

/// Clear Exclusive
///
/// Clears the local exclusive monitor. Used to reset the exclusive access
/// state after an exclusive load without a matching store, such as when
/// a context switch occurs.
#[inline(always)]
pub fn clrex() {
    unsafe { asm!("clrex", options(nomem, nostack, preserves_flags)) }
}

/// Load Exclusive (32-bit)
///
/// Performs an exclusive load from memory. Must be paired with a store
/// exclusive (`strex`) for atomic read-modify-write operations.
///
/// # Safety
///
/// `addr` must be a valid, aligned pointer.
#[inline(always)]
pub unsafe fn ldrex(addr: *const u32) -> u32 {
    let result: u32;
    unsafe {
        asm!(
            "ldrex {0}, [{1}]",
            out(reg) result,
            in(reg) addr,
            options(nostack, preserves_flags),
        )
    }
    result
}

/// Store Exclusive (32-bit)
///
/// Attempts to store a value to memory if the exclusive monitor still holds.
/// Returns 0 on success, 1 on failure (exclusive access lost).
///
/// # Safety
///
/// `addr` must be a valid, aligned pointer.
#[inline(always)]
pub unsafe fn strex(value: u32, addr: *mut u32) -> u32 {
    let result: u32;
    unsafe {
        asm!(
            "strex {0}, {1}, [{2}]",
            out(reg) result,
            in(reg) value,
            in(reg) addr,
            options(nostack, preserves_flags),
        )
    }
    result
}

/// Load Exclusive (8-bit)
///
/// # Safety
///
/// `addr` must be a valid pointer.
#[inline(always)]
pub unsafe fn ldrexb(addr: *const u8) -> u8 {
    let result: u32;
    unsafe {
        asm!(
            "ldrexb {0}, [{1}]",
            out(reg) result,
            in(reg) addr,
            options(nostack, preserves_flags),
        )
    }
    result as u8
}

/// Store Exclusive (8-bit)
///
/// # Safety
///
/// `addr` must be a valid pointer.
#[inline(always)]
pub unsafe fn strexb(value: u8, addr: *mut u8) -> u32 {
    let result: u32;
    unsafe {
        asm!(
            "strexb {0}, {1}, [{2}]",
            out(reg) result,
            in(reg) value as u32,
            in(reg) addr,
            options(nostack, preserves_flags),
        )
    }
    result
}

/// Load Exclusive (16-bit)
///
/// # Safety
///
/// `addr` must be a valid, aligned pointer.
#[inline(always)]
pub unsafe fn ldrexh(addr: *const u16) -> u16 {
    let result: u32;
    unsafe {
        asm!(
            "ldrexh {0}, [{1}]",
            out(reg) result,
            in(reg) addr,
            options(nostack, preserves_flags),
        )
    }
    result as u16
}

/// Store Exclusive (16-bit)
///
/// # Safety
///
/// `addr` must be a valid, aligned pointer.
#[inline(always)]
pub unsafe fn strexh(value: u16, addr: *mut u16) -> u32 {
    let result: u32;
    unsafe {
        asm!(
            "strexh {0}, {1}, [{2}]",
            out(reg) result,
            in(reg) value as u32,
            in(reg) addr,
            options(nostack, preserves_flags),
        )
    }
    result
}

// =============================================================================
// Load-Acquire / Store-Release (ARMv8-M)
// =============================================================================

/// Load-Acquire (32-bit)
///
/// Performs a load with acquire semantics. All memory accesses after this
/// instruction in program order are guaranteed to occur after this load.
///
/// # Safety
///
/// `addr` must be a valid, aligned pointer.
#[inline(always)]
pub unsafe fn lda(addr: *const u32) -> u32 {
    let result: u32;
    unsafe {
        asm!(
            "lda {0}, [{1}]",
            out(reg) result,
            in(reg) addr,
            options(nostack),
        )
    }
    result
}

/// Store-Release (32-bit)
///
/// Performs a store with release semantics. All memory accesses before this
/// instruction in program order are guaranteed to complete before this store.
///
/// # Safety
///
/// `addr` must be a valid, aligned pointer.
#[inline(always)]
pub unsafe fn stl(value: u32, addr: *mut u32) {
    unsafe {
        asm!(
            "stl {0}, [{1}]",
            in(reg) value,
            in(reg) addr,
            options(nostack),
        )
    }
}

/// Load-Acquire Exclusive (32-bit)
///
/// Combines exclusive load with acquire semantics.
///
/// # Safety
///
/// `addr` must be a valid, aligned pointer.
#[inline(always)]
pub unsafe fn ldaex(addr: *const u32) -> u32 {
    let result: u32;
    unsafe {
        asm!(
            "ldaex {0}, [{1}]",
            out(reg) result,
            in(reg) addr,
            options(nostack),
        )
    }
    result
}

/// Store-Release Exclusive (32-bit)
///
/// Combines exclusive store with release semantics.
/// Returns 0 on success, 1 on failure.
///
/// # Safety
///
/// `addr` must be a valid, aligned pointer.
#[inline(always)]
pub unsafe fn stlex(value: u32, addr: *mut u32) -> u32 {
    let result: u32;
    unsafe {
        asm!(
            "stlex {0}, {1}, [{2}]",
            out(reg) result,
            in(reg) value,
            in(reg) addr,
            options(nostack),
        )
    }
    result
}

/// Load-Acquire (8-bit)
///
/// # Safety
///
/// `addr` must be a valid pointer.
#[inline(always)]
pub unsafe fn ldab(addr: *const u8) -> u8 {
    let result: u32;
    unsafe {
        asm!(
            "ldab {0}, [{1}]",
            out(reg) result,
            in(reg) addr,
            options(nostack),
        )
    }
    result as u8
}

/// Store-Release (8-bit)
///
/// # Safety
///
/// `addr` must be a valid pointer.
#[inline(always)]
pub unsafe fn stlb(value: u8, addr: *mut u8) {
    unsafe {
        asm!(
            "stlb {0}, [{1}]",
            in(reg) value as u32,
            in(reg) addr,
            options(nostack),
        )
    }
}

/// Load-Acquire (16-bit)
///
/// # Safety
///
/// `addr` must be a valid, aligned pointer.
#[inline(always)]
pub unsafe fn ldah(addr: *const u16) -> u16 {
    let result: u32;
    unsafe {
        asm!(
            "ldah {0}, [{1}]",
            out(reg) result,
            in(reg) addr,
            options(nostack),
        )
    }
    result as u16
}

/// Store-Release (16-bit)
///
/// # Safety
///
/// `addr` must be a valid, aligned pointer.
#[inline(always)]
pub unsafe fn stlh(value: u16, addr: *mut u16) {
    unsafe {
        asm!(
            "stlh {0}, [{1}]",
            in(reg) value as u32,
            in(reg) addr,
            options(nostack),
        )
    }
}

// =============================================================================
// Saturation Instructions (ARMv8-M Mainline)
// =============================================================================

/// Signed Saturate
///
/// Saturates a signed value to a signed range of -2^(sat-1) to 2^(sat-1)-1.
/// The saturation bit position is specified at compile time (1-32).
///
/// # Arguments
///
/// * `value` - The signed value to saturate
/// * `SAT` - The saturation bit position (1-32), specified as a const generic
///
/// # Returns
///
/// The saturated value
#[inline(always)]
pub fn ssat<const SAT: u32>(value: i32) -> i32 {
    let result: i32;
    unsafe {
        asm!(
            "ssat {0}, #{sat}, {1}",
            out(reg) result,
            in(reg) value,
            sat = const SAT,
            options(nomem, nostack, preserves_flags),
        )
    }
    result
}

/// Unsigned Saturate
///
/// Saturates a signed value to an unsigned range of 0 to 2^sat - 1.
/// The saturation bit position is specified at compile time (0-31).
///
/// # Arguments
///
/// * `value` - The signed value to saturate
/// * `SAT` - The saturation bit position (0-31), specified as a const generic
///
/// # Returns
///
/// The saturated value (always positive)
#[inline(always)]
pub fn usat<const SAT: u32>(value: i32) -> u32 {
    let result: u32;
    unsafe {
        asm!(
            "usat {0}, #{sat}, {1}",
            out(reg) result,
            in(reg) value,
            sat = const SAT,
            options(nomem, nostack, preserves_flags),
        )
    }
    result
}

// =============================================================================
// Bit Manipulation Instructions
// =============================================================================

/// Count Leading Zeros
///
/// Returns the number of leading zeros in a 32-bit value.
/// Returns 32 if the value is 0.
#[inline(always)]
pub fn clz(value: u32) -> u32 {
    let result: u32;
    unsafe {
        asm!(
            "clz {0}, {1}",
            out(reg) result,
            in(reg) value,
            options(nomem, nostack, preserves_flags),
        )
    }
    result
}

/// Reverse Bits
///
/// Reverses the bit order of a 32-bit value.
#[inline(always)]
pub fn rbit(value: u32) -> u32 {
    let result: u32;
    unsafe {
        asm!(
            "rbit {0}, {1}",
            out(reg) result,
            in(reg) value,
            options(nomem, nostack, preserves_flags),
        )
    }
    result
}

/// Reverse Bytes in Word
///
/// Reverses the byte order of a 32-bit value (endianness swap).
#[inline(always)]
pub fn rev(value: u32) -> u32 {
    let result: u32;
    unsafe {
        asm!(
            "rev {0}, {1}",
            out(reg) result,
            in(reg) value,
            options(nomem, nostack, preserves_flags),
        )
    }
    result
}

/// Reverse Bytes in Halfwords
///
/// Reverses the byte order in each 16-bit halfword independently.
#[inline(always)]
pub fn rev16(value: u32) -> u32 {
    let result: u32;
    unsafe {
        asm!(
            "rev16 {0}, {1}",
            out(reg) result,
            in(reg) value,
            options(nomem, nostack, preserves_flags),
        )
    }
    result
}

/// Reverse Bytes in Signed Halfword
///
/// Reverses the byte order of the low 16 bits and sign-extends to 32 bits.
#[inline(always)]
pub fn revsh(value: i16) -> i32 {
    let result: i32;
    unsafe {
        asm!(
            "revsh {0}, {1}",
            out(reg) result,
            in(reg) value as i32,
            options(nomem, nostack, preserves_flags),
        )
    }
    result
}

// =============================================================================
// DSP Instructions (ARMv8-M Mainline with DSP extension)
// =============================================================================

/// Signed Multiply Accumulate Long
///
/// Multiplies two 32-bit signed values to produce a 64-bit result,
/// then adds it to a 64-bit accumulator.
///
/// # Returns
///
/// (low 32 bits, high 32 bits) of the 64-bit result
#[inline(always)]
pub fn smlal(a: i32, b: i32, acc_lo: u32, acc_hi: u32) -> (u32, u32) {
    let result_lo: u32;
    let result_hi: u32;
    unsafe {
        asm!(
            "smlal {lo}, {hi}, {a}, {b}",
            lo = inout(reg) acc_lo => result_lo,
            hi = inout(reg) acc_hi => result_hi,
            a = in(reg) a,
            b = in(reg) b,
            options(nomem, nostack, preserves_flags),
        )
    }
    (result_lo, result_hi)
}

/// Unsigned Multiply Accumulate Long
///
/// Multiplies two 32-bit unsigned values to produce a 64-bit result,
/// then adds it to a 64-bit accumulator.
#[inline(always)]
pub fn umlal(a: u32, b: u32, acc_lo: u32, acc_hi: u32) -> (u32, u32) {
    let result_lo: u32;
    let result_hi: u32;
    unsafe {
        asm!(
            "umlal {lo}, {hi}, {a}, {b}",
            lo = inout(reg) acc_lo => result_lo,
            hi = inout(reg) acc_hi => result_hi,
            a = in(reg) a,
            b = in(reg) b,
            options(nomem, nostack, preserves_flags),
        )
    }
    (result_lo, result_hi)
}

/// Signed Multiply Long
///
/// Multiplies two 32-bit signed values to produce a 64-bit result.
///
/// # Returns
///
/// (low 32 bits, high 32 bits) of the 64-bit result
#[inline(always)]
pub fn smull(a: i32, b: i32) -> (u32, u32) {
    let result_lo: u32;
    let result_hi: u32;
    unsafe {
        asm!(
            "smull {lo}, {hi}, {a}, {b}",
            lo = out(reg) result_lo,
            hi = out(reg) result_hi,
            a = in(reg) a,
            b = in(reg) b,
            options(nomem, nostack, preserves_flags),
        )
    }
    (result_lo, result_hi)
}

/// Unsigned Multiply Long
///
/// Multiplies two 32-bit unsigned values to produce a 64-bit result.
#[inline(always)]
pub fn umull(a: u32, b: u32) -> (u32, u32) {
    let result_lo: u32;
    let result_hi: u32;
    unsafe {
        asm!(
            "umull {lo}, {hi}, {a}, {b}",
            lo = out(reg) result_lo,
            hi = out(reg) result_hi,
            a = in(reg) a,
            b = in(reg) b,
            options(nomem, nostack, preserves_flags),
        )
    }
    (result_lo, result_hi)
}

/// Signed Divide
///
/// Performs signed 32-bit division.
/// If the divisor is 0, the result is 0 (no exception raised on Cortex-M33).
#[inline(always)]
pub fn sdiv(dividend: i32, divisor: i32) -> i32 {
    let result: i32;
    unsafe {
        asm!(
            "sdiv {0}, {1}, {2}",
            out(reg) result,
            in(reg) dividend,
            in(reg) divisor,
            options(nomem, nostack, preserves_flags),
        )
    }
    result
}

/// Unsigned Divide
///
/// Performs unsigned 32-bit division.
/// If the divisor is 0, the result is 0 (no exception raised on Cortex-M33).
#[inline(always)]
pub fn udiv(dividend: u32, divisor: u32) -> u32 {
    let result: u32;
    unsafe {
        asm!(
            "udiv {0}, {1}, {2}",
            out(reg) result,
            in(reg) dividend,
            in(reg) divisor,
            options(nomem, nostack, preserves_flags),
        )
    }
    result
}
