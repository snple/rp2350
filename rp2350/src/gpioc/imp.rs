//! Low-level GPIOC register access — private implementation detail.
//!
//! These macros and functions map directly to Cortex-M33 coprocessor
//! instructions. They are marked `unsafe` because they perform raw hardware
//! register access that bypasses the HAL's safety guarantees; callers must
//! ensure no conflicting access races with them. They are private because
//! the public API is the unified set of functions in the parent module.

#![allow(dead_code)]
#![allow(asm_sub_register)]

// ============================================================================
// Coprocessor instruction macros
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

#[inline(always)]
pub(super) unsafe fn out_lo_put(val: u32) {
    gpioc_write!(0, 0, val);
}

#[inline(always)]
pub(super) unsafe fn out_lo_xor(mask: u32) {
    gpioc_write!(1, 0, mask);
}

#[inline(always)]
pub(super) unsafe fn out_lo_set(mask: u32) {
    gpioc_write!(2, 0, mask);
}

#[inline(always)]
pub(super) unsafe fn out_lo_clr(mask: u32) {
    gpioc_write!(3, 0, mask);
}

#[inline(always)]
pub(super) unsafe fn out_lo_get() -> u32 {
    gpioc_read!(0, 0)
}

#[inline(always)]
pub(super) unsafe fn out_hi_put(val: u32) {
    gpioc_write!(0, 1, val);
}

#[inline(always)]
pub(super) unsafe fn out_hi_xor(mask: u32) {
    gpioc_write!(1, 1, mask);
}

#[inline(always)]
pub(super) unsafe fn out_hi_set(mask: u32) {
    gpioc_write!(2, 1, mask);
}

#[inline(always)]
pub(super) unsafe fn out_hi_clr(mask: u32) {
    gpioc_write!(3, 1, mask);
}

#[inline(always)]
pub(super) unsafe fn out_hi_get() -> u32 {
    gpioc_read!(0, 1)
}

// ============================================================================
// Output 64-bit atomic operations — MCRR/MRRC cRm = 0
// ============================================================================

#[inline(always)]
pub(super) unsafe fn out64_put(val: u64) {
    gpioc_write64!(0, 0, val as u32, (val >> 32) as u32);
}

#[inline(always)]
pub(super) unsafe fn out64_xor(mask: u64) {
    gpioc_write64!(1, 0, mask as u32, (mask >> 32) as u32);
}

#[inline(always)]
pub(super) unsafe fn out64_set(mask: u64) {
    gpioc_write64!(2, 0, mask as u32, (mask >> 32) as u32);
}

#[inline(always)]
pub(super) unsafe fn out64_clr(mask: u64) {
    gpioc_write64!(3, 0, mask as u32, (mask >> 32) as u32);
}

#[inline(always)]
pub(super) unsafe fn out64_get() -> u64 {
    gpioc_read64!(0, 0)
}

// ============================================================================
// Output Pin-Level Operations — single-bit OUT (MCRR, cRm = 0, opc 4–7)
// ============================================================================

#[inline(always)]
pub(super) unsafe fn bit_out_put(pin: u32, val: bool) {
    gpioc_bit_write!(4, 0, pin, val);
}

#[inline(always)]
pub(super) unsafe fn bit_out_toggle(pin: u32) {
    gpioc_write!(5, 0, pin);
}

#[inline(always)]
pub(super) unsafe fn bit_out_set(pin: u32) {
    gpioc_write!(6, 0, pin);
}

#[inline(always)]
pub(super) unsafe fn bit_out_clr(pin: u32) {
    gpioc_write!(7, 0, pin);
}

// ============================================================================
// Output Enable Register Operations (OE) — cRm = 4 (lo) / 5 (hi)
// ============================================================================

#[inline(always)]
pub(super) unsafe fn output_enable_lo_put(val: u32) {
    gpioc_write!(0, 4, val);
}

#[inline(always)]
pub(super) unsafe fn output_enable_lo_xor(mask: u32) {
    gpioc_write!(1, 4, mask);
}

#[inline(always)]
pub(super) unsafe fn output_enable_lo_set(mask: u32) {
    gpioc_write!(2, 4, mask);
}

#[inline(always)]
pub(super) unsafe fn output_enable_lo_clr(mask: u32) {
    gpioc_write!(3, 4, mask);
}

#[inline(always)]
pub(super) unsafe fn output_enable_lo_get() -> u32 {
    gpioc_read!(0, 4)
}

#[inline(always)]
pub(super) unsafe fn output_enable_hi_put(val: u32) {
    gpioc_write!(0, 5, val);
}

#[inline(always)]
pub(super) unsafe fn output_enable_hi_xor(mask: u32) {
    gpioc_write!(1, 5, mask);
}

#[inline(always)]
pub(super) unsafe fn output_enable_hi_set(mask: u32) {
    gpioc_write!(2, 5, mask);
}

#[inline(always)]
pub(super) unsafe fn output_enable_hi_clr(mask: u32) {
    gpioc_write!(3, 5, mask);
}

#[inline(always)]
pub(super) unsafe fn output_enable_hi_get() -> u32 {
    gpioc_read!(0, 5)
}

// ============================================================================
// OE 64-bit atomic operations — MCRR/MRRC cRm = 4
// ============================================================================

#[inline(always)]
pub(super) unsafe fn output_enable64_put(val: u64) {
    gpioc_write64!(0, 4, val as u32, (val >> 32) as u32);
}

#[inline(always)]
pub(super) unsafe fn output_enable64_xor(mask: u64) {
    gpioc_write64!(1, 4, mask as u32, (mask >> 32) as u32);
}

#[inline(always)]
pub(super) unsafe fn output_enable64_set(mask: u64) {
    gpioc_write64!(2, 4, mask as u32, (mask >> 32) as u32);
}

#[inline(always)]
pub(super) unsafe fn output_enable64_clr(mask: u64) {
    gpioc_write64!(3, 4, mask as u32, (mask >> 32) as u32);
}

#[inline(always)]
pub(super) unsafe fn output_enable64_get() -> u64 {
    gpioc_read64!(0, 4)
}

// ============================================================================
// OE Pin-Level Operations — single-bit OE (MCRR, cRm = 4, opc 4–7)
// ============================================================================

#[inline(always)]
pub(super) unsafe fn bit_output_enable_put(pin: u32, val: bool) {
    gpioc_bit_write!(4, 4, pin, val);
}

#[inline(always)]
pub(super) unsafe fn bit_output_enable_toggle(pin: u32) {
    gpioc_write!(5, 4, pin);
}

#[inline(always)]
pub(super) unsafe fn bit_output_enable_set(pin: u32) {
    gpioc_write!(6, 4, pin);
}

#[inline(always)]
pub(super) unsafe fn bit_output_enable_clr(pin: u32) {
    gpioc_write!(7, 4, pin);
}

// ============================================================================
// Input Register Operations (IN) — cRm = 8 (lo) / 9 (hi)
// ============================================================================

#[inline(always)]
pub(super) unsafe fn in_lo() -> u32 {
    gpioc_read!(0, 8)
}

#[inline(always)]
pub(super) unsafe fn in_hi() -> u32 {
    gpioc_read!(0, 9)
}

#[inline(always)]
pub(super) unsafe fn in64() -> u64 {
    gpioc_read64!(0, 8)
}

// ============================================================================
// Indexed Operations — MCRR with opc 8–11
// ============================================================================
//
// These allow dynamic register selection; kept for completeness of the GPIOC
// instruction set but not yet exposed in the public API.
#[inline(always)]
pub(super) unsafe fn index_out_put(reg_index: u32, val: u32) {
    gpioc_index_write!(8, 0, reg_index, val);
}

#[inline(always)]
pub(super) unsafe fn index_out_xor(reg_index: u32, mask: u32) {
    gpioc_index_write!(9, 0, reg_index, mask);
}

#[inline(always)]
pub(super) unsafe fn index_out_set(reg_index: u32, mask: u32) {
    gpioc_index_write!(10, 0, reg_index, mask);
}

#[inline(always)]
pub(super) unsafe fn index_out_clr(reg_index: u32, mask: u32) {
    gpioc_index_write!(11, 0, reg_index, mask);
}

#[inline(always)]
pub(super) unsafe fn index_output_enable_put(reg_index: u32, val: u32) {
    gpioc_index_write!(8, 4, reg_index, val);
}

#[inline(always)]
pub(super) unsafe fn index_output_enable_xor(reg_index: u32, mask: u32) {
    gpioc_index_write!(9, 4, reg_index, mask);
}

#[inline(always)]
pub(super) unsafe fn index_output_enable_set(reg_index: u32, mask: u32) {
    gpioc_index_write!(10, 4, reg_index, mask);
}

#[inline(always)]
pub(super) unsafe fn index_output_enable_clr(reg_index: u32, mask: u32) {
    gpioc_index_write!(11, 4, reg_index, mask);
}
