//! GPIO Coprocessor (GPIOC) for RP2350 Cortex-M33
//!
//! The GPIO coprocessor is a Cortex-M33 custom coprocessor (CP0) that provides
//! single-cycle GPIO access via dedicated coprocessor instructions. It replaces
//! SIO memory-mapped register access for GPIO output, output-enable, and input
//! operations.
//!
//! # Advantages over SIO
//!
//! | Aspect            | SIO (0xd0000000)      | GPIOC (MCR/MRC)        |
//! |-------------------|-----------------------|------------------------|
//! | Access latency    | Multi-cycle bus       | Single-cycle           |
//! | Atomic 64-bit     | Two 32-bit operations | One `MCRR` instruction |
//! | Interrupt safety  | TOGL alias needed     | Inherently atomic      |
//!
//! # Architecture
//!
//! GPIOC uses the Cortex-M33 coprocessor interface with CP0 (`p0`):
//!
//! ```text
//! Instruction format:
//!   MCR  p0, #opc, Rd, c0, cRm    — 32-bit write
//!   MRC  p0, #opc, Rd, c0, cRm    — 32-bit read
//!   MCRR p0, #opc, Rd, Rn, cRm    — 64-bit write
//!   MRRC p0, #opc, Rd, Rn, cRm    — 64-bit read
//!
//! Register groups (cRm):
//!   c0  — OUT low 32 bits (GPIO 0–31)
//!   c1  — OUT high 32 bits (GPIO 32–63)
//!   c4  — OE low 32 bits
//!   c5  — OE high 32 bits
//!   c8  — IN low 32 bits
//!   c9  — IN high 32 bits
//!
//! Operations (#opc):
//!   #0  — put  (write value)
//!   #1  — xor  (toggle mask)
//!   #2  — set  (set mask bits to 1)
//!   #3  — clr  (clear mask bits to 0)
//!   #4  — bit_put  (single bit write)
//!   #5  — bit_xor  (single bit toggle)
//!   #6  — bit_set  (single bit set)
//!   #7  — bit_clr  (single bit clear)
//!   #8  — index_put  (dynamic register index)
//!   #9  — index_xor
//!   #10 — index_set
//!   #11 — index_clr
//! ```
//!
//! # Usage
//!
//! ```ignore
//! use pico_rs::gpioc;
//!
//! // Output — replace sio_hw->gpio_set / gpio_clr / gpio_togl
//! gpioc::out_set(1 << 25);       // drive GPIO 25 high
//! gpioc::out_clr(1 << 25);       // drive GPIO 25 low
//! gpioc::out_xor(1 << 25);       // toggle GPIO 25
//!
//! // Direction — replace sio_hw->gpio_oe_set / gpio_oe_clr
//! gpioc::oe_set(1 << 25);        // set GPIO 25 to output
//! gpioc::oe_clr(1 << 25);        // set GPIO 25 to input
//!
//! // Read — replace sio_hw->gpio_in
//! let pins = gpioc::in_lo();     // sample GPIO 0–31
//!
//! // Atomic 64-bit operations (OUT and OE simultaneously)
//! gpioc::out64_set(1 << 40);     // set GPIO 40 in one cycle
//!
//! // Single-bit operations
//! gpioc::bit_out_put(25, true);  // set GPIO 25
//! gpioc::bit_out_toggle(25);     // toggle GPIO 25
//! ```
//!
//! # Safety
//!
//! All functions are safe to call from any context. GPIOC operations are
//! inherently atomic and concurrency-safe — there are no read-modify-write
//! concerns because each instruction targets a single register directly.
//!
//! # Availability
//!
//! Only available on RP2350 (Cortex-M33). RP2040 (Cortex-M0+) does not have
//! a GPIO coprocessor. This module is conditionally compiled only for ARM targets
//! (`#[cfg(target_arch = "arm")]`).

// ============================================================================
// Low-level coprocessor instruction macros
// ============================================================================
//
// These macros wrap the Cortex-M33 MCR/MRC/MCRR/MRRC instructions used to
// access the GPIO coprocessor (CP0).
//
// Instruction set:
//   MCR  p0, #opc, Rd, c0, cRm  — coprocessor write (Rd → CP0)
//   MRC  p0, #opc, Rd, c0, cRm  — coprocessor read  (CP0 → Rd)
//   MCRR p0, #opc, Rd, Rn, cRm  — coprocessor 64-bit write (Rd,Rn → CP0)
//   MRRC p0, #opc, Rd, Rn, cRm  — coprocessor 64-bit read  (CP0 → Rd,Rn)
//
// Register reference (pico-sdk hardware/gpio_coproc.h):
//   MCR/MRC  cRm values: c0=OUT_LO, c1=OUT_HI, c4=OE_LO, c5=OE_HI,
//                         c8=IN_LO,  c9=IN_HI
//   MCRR/MRRC cRm values: c0=OUT_HILO, c4=OE_HILO, c8=IN_HILO
// ============================================================================

/// 32-bit coprocessor write: `MCR p0, #opc, val, c0, cRm`
macro_rules! gpioc_write {
    ($opc:expr, $reg:expr, $val:expr) => {
        unsafe {
            core::arch::asm!(
                "mcr p0, #{opc}, {val}, c0, c{reg}",
                opc = const $opc,
                reg = const $reg,
                val = in(reg) $val,
                options(nomem, nostack, preserves_flags),
            )
        }
    };
}

/// 32-bit coprocessor read: `MRC p0, #opc, val, c0, cRm`
macro_rules! gpioc_read {
    ($opc:expr, $reg:expr) => {{
        let val: u32;
        unsafe {
            core::arch::asm!(
                "mrc p0, #{opc}, {val}, c0, c{reg}",
                opc = const $opc,
                reg = const $reg,
                val = out(reg) val,
                options(nomem, nostack, preserves_flags),
            );
        }
        val
    }};
}

/// 64-bit coprocessor write: `MCRR p0, #opc, lo, hi, cRm`
macro_rules! gpioc_write64 {
    ($opc:expr, $crm:expr, $lo:expr, $hi:expr) => {
        unsafe {
            core::arch::asm!(
                "mcrr p0, #{opc}, {lo}, {hi}, c{reg}",
                opc = const $opc,
                reg = const $crm,
                lo = in(reg) $lo,
                hi = in(reg) $hi,
                options(nomem, nostack, preserves_flags),
            )
        }
    };
}

/// 64-bit coprocessor read: `MRRC p0, #opc, lo, hi, cRm`
macro_rules! gpioc_read64 {
    ($opc:expr, $crm:expr) => {{
        let lo: u32;
        let hi: u32;
        unsafe {
            core::arch::asm!(
                "mrrc p0, #{opc}, {lo}, {hi}, c{reg}",
                opc = const $opc,
                reg = const $crm,
                lo = out(reg) lo,
                hi = out(reg) hi,
                options(nomem, nostack, preserves_flags),
            );
        }
        ((hi as u64) << 32) | (lo as u64)
    }};
}

/// Single-bit coprocessor write (MCRR with pin + bool)
macro_rules! gpioc_bit_write {
    ($opc:expr, $crm:expr, $pin:expr, $val:expr) => {
        unsafe {
            core::arch::asm!(
                "mcrr p0, #{opc}, {pin}, {val}, c{reg}",
                opc = const $opc,
                reg = const $crm,
                pin = in(reg) $pin,
                val = in(reg) $val as u32,
                options(nomem, nostack, preserves_flags),
            )
        }
    };
}

/// Indexed coprocessor write: `MCRR p0, #opc, val, idx, cRm`
macro_rules! gpioc_index_write {
    ($opc:expr, $crm:expr, $idx:expr, $val:expr) => {
        unsafe {
            core::arch::asm!(
                "mcrr p0, #{opc}, {val}, {idx}, c{reg}",
                opc = const $opc,
                reg = const $crm,
                idx = in(reg) $idx,
                val = in(reg) $val,
                options(nomem, nostack, preserves_flags),
            )
        }
    };
}

// ============================================================================
// Output Register Operations (OUT) — cRm = 0 (lo) / 1 (hi)
// ============================================================================

/// Write the low 32-bit output register. Equivalent to `sio_hw->gpio_out = val`.
#[inline(always)]
pub fn out_lo_put(val: u32) {
    gpioc_write!(0, 0, val);
}

/// Toggle the low 32-bit output register. Equivalent to `sio_hw->gpio_togl = mask`.
#[inline(always)]
pub fn out_lo_xor(mask: u32) {
    gpioc_write!(1, 0, mask);
}

/// Set bits in the low 32-bit output register. Equivalent to `sio_hw->gpio_set = mask`.
#[inline(always)]
pub fn out_lo_set(mask: u32) {
    gpioc_write!(2, 0, mask);
}

/// Clear bits in the low 32-bit output register. Equivalent to `sio_hw->gpio_clr = mask`.
#[inline(always)]
pub fn out_lo_clr(mask: u32) {
    gpioc_write!(3, 0, mask);
}

/// Read the low 32-bit output register. Equivalent to `sio_hw->gpio_out`.
#[inline(always)]
pub fn out_lo_get() -> u32 {
    gpioc_read!(0, 0)
}

/// Write the high 32-bit output register. Equivalent to `sio_hw->gpio_hi_out = val`.
#[inline(always)]
pub fn out_hi_put(val: u32) {
    gpioc_write!(0, 1, val);
}

/// Toggle the high 32-bit output register. Equivalent to `sio_hw->gpio_hi_togl = mask`.
#[inline(always)]
pub fn out_hi_xor(mask: u32) {
    gpioc_write!(1, 1, mask);
}

/// Set bits in the high 32-bit output register. Equivalent to `sio_hw->gpio_hi_set = mask`.
#[inline(always)]
pub fn out_hi_set(mask: u32) {
    gpioc_write!(2, 1, mask);
}

/// Clear bits in the high 32-bit output register. Equivalent to `sio_hw->gpio_hi_clr = mask`.
#[inline(always)]
pub fn out_hi_clr(mask: u32) {
    gpioc_write!(3, 1, mask);
}

/// Read the high 32-bit output register. Equivalent to `sio_hw->gpio_hi_out`.
#[inline(always)]
pub fn out_hi_get() -> u32 {
    gpioc_read!(0, 1)
}

// ============================================================================
// Output 64-bit atomic operations — MCRR/MRRC cRm = 0
// ============================================================================

/// Write both output registers in a single cycle.
/// Equivalent to `sio_hw->gpio_out = val as u32; sio_hw->gpio_hi_out = (val >> 32) as u32`.
#[inline(always)]
pub fn out64_put(val: u64) {
    gpioc_write64!(0, 0, val as u32, (val >> 32) as u32);
}

/// Toggle bits in both output registers in a single cycle.
#[inline(always)]
pub fn out64_xor(mask: u64) {
    gpioc_write64!(1, 0, mask as u32, (mask >> 32) as u32);
}

/// Set bits in both output registers in a single cycle.
#[inline(always)]
pub fn out64_set(mask: u64) {
    gpioc_write64!(2, 0, mask as u32, (mask >> 32) as u32);
}

/// Clear bits in both output registers in a single cycle.
#[inline(always)]
pub fn out64_clr(mask: u64) {
    gpioc_write64!(3, 0, mask as u32, (mask >> 32) as u32);
}

/// Read both output registers in a single cycle.
#[inline(always)]
pub fn out64_get() -> u64 {
    gpioc_read64!(0, 0)
}

// ============================================================================
// Output Convenience Aliases (mask-based, GPIO 0–31 shorthand)
// ============================================================================

/// Set GPIO output bits (shorthand for [`out_lo_set`]).
#[inline(always)]
pub fn out_set(mask: u32) {
    out_lo_set(mask);
}

/// Clear GPIO output bits (shorthand for [`out_lo_clr`]).
#[inline(always)]
pub fn out_clr(mask: u32) {
    out_lo_clr(mask);
}

/// Toggle GPIO output bits (shorthand for [`out_lo_xor`]).
#[inline(always)]
pub fn out_xor(mask: u32) {
    out_lo_xor(mask);
}

/// Read GPIO output state (shorthand for [`out_lo_get`]).
#[inline(always)]
pub fn out_get() -> u32 {
    out_lo_get()
}

// ============================================================================
// Output Pin-Level Operations — single-bit OUT (MCRR, cRm = 0, opc 4–7)
// ============================================================================

/// Write a single GPIO output bit. Equivalent to:
/// ```ignore
/// if val { gpioc::out64_set(1 << pin) } else { gpioc::out64_clr(1 << pin) }
/// ```
#[inline(always)]
pub fn bit_out_put(pin: u32, val: bool) {
    gpioc_bit_write!(4, 0, pin, val);
}

/// Unconditionally toggle a single GPIO output bit.
#[inline(always)]
pub fn bit_out_toggle(pin: u32) {
    gpioc_write!(5, 0, pin);
}

/// Unconditionally set a single GPIO output bit.
#[inline(always)]
pub fn bit_out_set(pin: u32) {
    gpioc_write!(6, 0, pin);
}

/// Unconditionally clear a single GPIO output bit.
#[inline(always)]
pub fn bit_out_clr(pin: u32) {
    gpioc_write!(7, 0, pin);
}

/// Read the output level of a specific GPIO pin (0–63).
///
/// Returns `true` if the pin is being driven high, `false` if low.
/// This reads the output register, not the actual pin voltage —
/// use [`in_pin`] to sample the pad state.
#[inline(always)]
pub fn out_pin_get(pin: u32) -> bool {
    (out64_get() >> pin) & 1 != 0
}

// ============================================================================
// Output Enable Register Operations (OE) — cRm = 4 (lo) / 5 (hi)
// ============================================================================

/// Write the low 32-bit output enable register. Equivalent to `sio_hw->gpio_oe = val`.
#[inline(always)]
pub fn oe_lo_put(val: u32) {
    gpioc_write!(0, 4, val);
}

/// Toggle the low 32-bit output enable register. Equivalent to `sio_hw->gpio_oe_togl = mask`.
#[inline(always)]
pub fn oe_lo_xor(mask: u32) {
    gpioc_write!(1, 4, mask);
}

/// Set bits in the low 32-bit output enable register (set to output).
/// Equivalent to `sio_hw->gpio_oe_set = mask`.
#[inline(always)]
pub fn oe_lo_set(mask: u32) {
    gpioc_write!(2, 4, mask);
}

/// Clear bits in the low 32-bit output enable register (set to input).
/// Equivalent to `sio_hw->gpio_oe_clr = mask`.
#[inline(always)]
pub fn oe_lo_clr(mask: u32) {
    gpioc_write!(3, 4, mask);
}

/// Read the low 32-bit output enable register. Equivalent to `sio_hw->gpio_oe`.
#[inline(always)]
pub fn oe_lo_get() -> u32 {
    gpioc_read!(0, 4)
}

/// Write the high 32-bit output enable register. Equivalent to `sio_hw->gpio_hi_oe = val`.
#[inline(always)]
pub fn oe_hi_put(val: u32) {
    gpioc_write!(0, 5, val);
}

/// Toggle the high 32-bit output enable register.
#[inline(always)]
pub fn oe_hi_xor(mask: u32) {
    gpioc_write!(1, 5, mask);
}

/// Set bits in the high 32-bit output enable register (set to output).
/// Equivalent to `sio_hw->gpio_hi_oe_set = mask`.
#[inline(always)]
pub fn oe_hi_set(mask: u32) {
    gpioc_write!(2, 5, mask);
}

/// Clear bits in the high 32-bit output enable register (set to input).
/// Equivalent to `sio_hw->gpio_hi_oe_clr = mask`.
#[inline(always)]
pub fn oe_hi_clr(mask: u32) {
    gpioc_write!(3, 5, mask);
}

/// Read the high 32-bit output enable register. Equivalent to `sio_hw->gpio_hi_oe`.
#[inline(always)]
pub fn oe_hi_get() -> u32 {
    gpioc_read!(0, 5)
}

// ============================================================================
// OE 64-bit atomic operations — MCRR/MRRC cRm = 4
// ============================================================================

/// Write both output enable registers in a single cycle.
#[inline(always)]
pub fn oe64_put(val: u64) {
    gpioc_write64!(0, 4, val as u32, (val >> 32) as u32);
}

/// Toggle bits in both output enable registers in a single cycle.
#[inline(always)]
pub fn oe64_xor(mask: u64) {
    gpioc_write64!(1, 4, mask as u32, (mask >> 32) as u32);
}

/// Set bits in both output enable registers (to output) in a single cycle.
#[inline(always)]
pub fn oe64_set(mask: u64) {
    gpioc_write64!(2, 4, mask as u32, (mask >> 32) as u32);
}

/// Clear bits in both output enable registers (to input) in a single cycle.
#[inline(always)]
pub fn oe64_clr(mask: u64) {
    gpioc_write64!(3, 4, mask as u32, (mask >> 32) as u32);
}

/// Read both output enable registers in a single cycle.
#[inline(always)]
pub fn oe64_get() -> u64 {
    gpioc_read64!(4, 4)
}

// ============================================================================
// OE Convenience Aliases (mask-based, GPIO 0–31 shorthand)
// ============================================================================

/// Set GPIO output enable bits (shorthand for [`oe_lo_set`]).
#[inline(always)]
pub fn oe_set(mask: u32) {
    oe_lo_set(mask);
}

/// Clear GPIO output enable bits (shorthand for [`oe_lo_clr`]).
#[inline(always)]
pub fn oe_clr(mask: u32) {
    oe_lo_clr(mask);
}

/// Toggle GPIO output enable bits (shorthand for [`oe_lo_xor`]).
#[inline(always)]
pub fn oe_xor(mask: u32) {
    oe_lo_xor(mask);
}

/// Read GPIO output enable state (shorthand for [`oe_lo_get`]).
#[inline(always)]
pub fn oe_get() -> u32 {
    oe_lo_get()
}

// ============================================================================
// OE Pin-Level Operations — single-bit OE (MCRR, cRm = 4, opc 4–7)
// ============================================================================

/// Write a single GPIO output enable bit.
#[inline(always)]
pub fn bit_oe_put(pin: u32, val: bool) {
    gpioc_bit_write!(4, 4, pin, val);
}

/// Unconditionally toggle a single GPIO output enable bit.
#[inline(always)]
pub fn bit_oe_toggle(pin: u32) {
    gpioc_write!(5, 4, pin);
}

/// Unconditionally set a single GPIO output enable bit (to output).
#[inline(always)]
pub fn bit_oe_set(pin: u32) {
    gpioc_write!(6, 4, pin);
}

/// Unconditionally clear a single GPIO output enable bit (to input).
#[inline(always)]
pub fn bit_oe_clr(pin: u32) {
    gpioc_write!(7, 4, pin);
}

/// Read the output enable state of a specific GPIO pin (0–63).
///
/// Returns `true` if the pin is configured as output, `false` if input.
#[inline(always)]
pub fn oe_pin_get(pin: u32) -> bool {
    (oe64_get() >> pin) & 1 != 0
}

// ============================================================================
// Input Register Operations (IN) — cRm = 8 (lo) / 9 (hi)
// ============================================================================

/// Sample the low 32 GPIO input pins. Equivalent to `sio_hw->gpio_in`.
#[inline(always)]
pub fn in_lo() -> u32 {
    gpioc_read!(0, 8)
}

/// Sample the high 32 GPIO input pins. Equivalent to `sio_hw->gpio_hi_in`.
#[inline(always)]
pub fn in_hi() -> u32 {
    gpioc_read!(0, 9)
}

/// Sample all 64 GPIO input pins in a single cycle.
#[inline(always)]
pub fn in64() -> u64 {
    gpioc_read64!(0, 8)
}

// ============================================================================
// Input Pin-Level Operations — single-pin read convenience
// ============================================================================

/// Read the input level of a specific GPIO pin (0–63).
///
/// Returns `true` if the pin is high, `false` if low.
#[inline(always)]
pub fn in_pin(pin: u32) -> bool {
    (in64() >> pin) & 1 != 0
}

// ============================================================================
// Indexed Operations — MCRR with opc 8–11
// ============================================================================
//
// These allow dynamic register selection for situations where the register
// index is not known at compile time. Useful for generic pin manipulation.
//
// `reg_index` selects which 32-bit register to target:
//   0 = low 32 bits (GPIO 0–31), 1 = high 32 bits (GPIO 32–63)
// ============================================================================

/// Write a value to a dynamically selected output register.
#[inline(always)]
pub fn index_out_put(reg_index: u32, val: u32) {
    gpioc_index_write!(8, 0, reg_index, val);
}

/// Toggle bits in a dynamically selected output register.
#[inline(always)]
pub fn index_out_xor(reg_index: u32, mask: u32) {
    gpioc_index_write!(9, 0, reg_index, mask);
}

/// Set bits in a dynamically selected output register.
#[inline(always)]
pub fn index_out_set(reg_index: u32, mask: u32) {
    gpioc_index_write!(10, 0, reg_index, mask);
}

/// Clear bits in a dynamically selected output register.
#[inline(always)]
pub fn index_out_clr(reg_index: u32, mask: u32) {
    gpioc_index_write!(11, 0, reg_index, mask);
}

/// Write a value to a dynamically selected output enable register.
#[inline(always)]
pub fn index_oe_put(reg_index: u32, val: u32) {
    gpioc_index_write!(8, 4, reg_index, val);
}

/// Toggle bits in a dynamically selected output enable register.
#[inline(always)]
pub fn index_oe_xor(reg_index: u32, mask: u32) {
    gpioc_index_write!(9, 4, reg_index, mask);
}

/// Set bits in a dynamically selected output enable register (to output).
#[inline(always)]
pub fn index_oe_set(reg_index: u32, mask: u32) {
    gpioc_index_write!(10, 4, reg_index, mask);
}

/// Clear bits in a dynamically selected output enable register (to input).
#[inline(always)]
pub fn index_oe_clr(reg_index: u32, mask: u32) {
    gpioc_index_write!(11, 4, reg_index, mask);
}
