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
//! // Output — pin-level (single-cycle coprocessor)
//! unsafe { gpioc::set_high(25) };    // drive GPIO 25 high
//! unsafe { gpioc::set_low(25) };     // drive GPIO 25 low
//! unsafe { gpioc::toggle(25) };      // toggle GPIO 25
//!
//! // Direction
//! unsafe { gpioc::enable_output(25) };      // set GPIO 25 to output
//! unsafe { gpioc::disable_output(25) };      // set GPIO 25 to input
//!
//! // Read
//! let input = unsafe { gpioc::get(25) };
//!
//! // Mask-level (batch control)
//! unsafe { gpioc::mask_set_low(1 << 25) };
//! let pins = unsafe { gpioc::mask_input_low() };
//! ```
//!
//! # Safety
//!
//! All public functions in this module are `unsafe` because the caller must
//! ensure that the GPIO subsystem has been initialized and pins are configured
//! for SIO — matching the contract of the equivalent functions in
//! [`gpio`](crate::gpio).
//!
//! The low-level coprocessor instructions themselves are inherently atomic and
//! concurrency-safe, but they are private implementation details in
//! [`imp`](self::imp).
//!
//! # Availability
//!
//! Only available on RP2350 (Cortex-M33). RP2040 (Cortex-M0+) does not have
//! a GPIO coprocessor. This module is conditionally compiled only for ARM targets
//! (`#[cfg(target_arch = "arm")]`).

mod imp;

// ============================================================================
// Unified high-level API — same names as gpio
// ============================================================================
// ============================================================================
//
// These functions mirror the [`gpio`](crate::gpio) module API but
// use the GPIOC coprocessor instead of SIO MMIO registers. Users can choose
// between the two modules for identical operations:
//
// ```ignore
// use pico_rs::gpio;
// use pico_rs::gpioc;
//
// // SIO path (MMIO):
// unsafe { gpio::set_high(25) };
//
// // GPIOC path (coprocessor, single-cycle):
// unsafe { gpioc::set_high(25) };
// ```
//
// # Safety
//
// All functions are `unsafe` because the caller must ensure:
// - The GPIO subsystem was initialized via [`gpio::Pins::init()`](crate::gpio::Pins::init).
// - Each pin has been configured for SIO function via [`Pin<N>`](crate::gpio::Pin).
// - No concurrent configuration changes happen while data operations are in flight.

/// Read the input value of a single pin.
///
/// Returns `true` if the pin is high.
///
/// # Safety
///
/// See the [module-level safety contract](crate::gpioc#safety).
#[inline]
pub unsafe fn get(pin: u32) -> bool {
    crate::gpio::assert_pin(pin);
    if pin < 32 {
        unsafe { (imp::in_lo() & (1 << pin)) != 0 }
    } else {
        unsafe { (imp::in_hi() & (1 << (pin - 32))) != 0 }
    }
}

/// Set a single pin output high.
///
/// The pin must already be configured for SIO output.
///
/// # Safety
///
/// See the [module-level safety contract](crate::gpioc#safety).
#[inline]
pub unsafe fn set_high(pin: u32) {
    crate::gpio::assert_pin(pin);
    unsafe { imp::bit_out_set(pin) }
}

/// Set a single pin output low.
///
/// The pin must already be configured for SIO output.
///
/// # Safety
///
/// See the [module-level safety contract](crate::gpioc#safety).
#[inline]
pub unsafe fn set_low(pin: u32) {
    crate::gpio::assert_pin(pin);
    unsafe { imp::bit_out_clr(pin) }
}

/// Toggle a single pin output.
///
/// The pin must already be configured for SIO output.
///
/// # Safety
///
/// See the [module-level safety contract](crate::gpioc#safety).
#[inline]
pub unsafe fn toggle(pin: u32) {
    crate::gpio::assert_pin(pin);
    unsafe { imp::bit_out_toggle(pin) }
}

/// Set a single pin output to a specific value.
///
/// # Safety
///
/// See the [module-level safety contract](crate::gpioc#safety).
#[inline]
pub unsafe fn set(pin: u32, high: bool) {
    crate::gpio::assert_pin(pin);
    unsafe { imp::bit_out_put(pin, high) }
}

/// Read the programmed output level of a single pin.
///
/// This reads the output register, not the actual pad input state. Use
/// [`get`] if you need the live pin level.
///
/// # Safety
///
/// See the [module-level safety contract](crate::gpioc#safety).
#[inline]
pub unsafe fn output(pin: u32) -> bool {
    crate::gpio::assert_pin(pin);
    if pin < 32 {
        unsafe { (imp::out_lo_get() & (1 << pin)) != 0 }
    } else {
        unsafe { (imp::out_hi_get() & (1 << (pin - 32))) != 0 }
    }
}

/// Enable output for a single pin.
///
/// Unconditionally sets the OE bit to 1 (output). See [`set_output_enable`] for a
/// conditional version that accepts a `bool`.
///
/// # Safety
///
/// See the [module-level safety contract](crate::gpioc#safety).
#[inline]
pub unsafe fn enable_output(pin: u32) {
    crate::gpio::assert_pin(pin);
    unsafe { imp::bit_output_enable_set(pin) }
}

/// Disable output for a single pin.
///
/// Unconditionally clears the OE bit to 0 (input). See [`set_output_enable`] for a
/// conditional version that accepts a `bool`.
///
/// # Safety
///
/// See the [module-level safety contract](crate::gpioc#safety).
#[inline]
pub unsafe fn disable_output(pin: u32) {
    crate::gpio::assert_pin(pin);
    unsafe { imp::bit_output_enable_clr(pin) }
}

/// Write the OE bit to a specific value.
///
/// Sets to output if `output` is `true`, input if `false`.
/// More efficient than a separate `if` + [`enable_output`]/[`disable_output`]
/// because GPIOC uses a single `MCRR` instruction.
///
/// # Safety
///
/// See the [module-level safety contract](crate::gpioc#safety).
#[inline]
pub unsafe fn set_output_enable(pin: u32, output: bool) {
    crate::gpio::assert_pin(pin);
    unsafe { imp::bit_output_enable_put(pin, output) }
}

/// Toggle the OE bit for a single pin.
///
/// If the pin was output it becomes input, and vice versa.
///
/// # Safety
///
/// See the [module-level safety contract](crate::gpioc#safety).
#[inline]
pub unsafe fn toggle_output_enable(pin: u32) {
    crate::gpio::assert_pin(pin);
    unsafe { imp::bit_output_enable_toggle(pin) }
}

/// Check whether output is enabled for a single pin.
///
/// # Safety
///
/// See the [module-level safety contract](crate::gpioc#safety).
#[inline]
pub unsafe fn is_output(pin: u32) -> bool {
    crate::gpio::assert_pin(pin);
    if pin < 32 {
        unsafe { (imp::output_enable_lo_get() & (1 << pin)) != 0 }
    } else {
        unsafe { (imp::output_enable_hi_get() & (1 << (pin - 32))) != 0 }
    }
}

// Mask-level operations — low bank -------------------------------------------

/// Read the live input state of GPIO 0-31 as a bitmask.
///
/// # Safety
///
/// See the [module-level safety contract](crate::gpioc#safety).
#[inline]
pub unsafe fn mask_input_low() -> u32 {
    unsafe { imp::in_lo() }
}

/// Read the programmed output register for GPIO 0-31.
///
/// # Safety
///
/// See the [module-level safety contract](crate::gpioc#safety).
#[inline]
pub unsafe fn mask_output_low() -> u32 {
    unsafe { imp::out_lo_get() }
}

/// Replace the entire output register for GPIO 0-31.
///
/// # Safety
///
/// See the [module-level safety contract](crate::gpioc#safety).
#[inline]
pub unsafe fn mask_write_output_low(value: u32) {
    unsafe { imp::out_lo_put(value) }
}

/// Atomically drive selected GPIO 0-31 outputs high.
///
/// # Safety
///
/// See the [module-level safety contract](crate::gpioc#safety).
#[inline]
pub unsafe fn mask_set_low(mask: u32) {
    unsafe { imp::out_lo_set(mask) }
}

/// Atomically drive selected GPIO 0-31 outputs low.
///
/// # Safety
///
/// See the [module-level safety contract](crate::gpioc#safety).
#[inline]
pub unsafe fn mask_clr_lo(mask: u32) {
    unsafe { imp::out_lo_clr(mask) }
}

/// Atomically toggle selected GPIO 0-31 outputs.
///
/// # Safety
///
/// See the [module-level safety contract](crate::gpioc#safety).
#[inline]
pub unsafe fn mask_xor_lo(mask: u32) {
    unsafe { imp::out_lo_xor(mask) }
}

/// Read the output-enable register for GPIO 0-31.
///
/// # Safety
///
/// See the [module-level safety contract](crate::gpioc#safety).
#[inline]
pub unsafe fn mask_output_enable_lo() -> u32 {
    unsafe { imp::output_enable_lo_get() }
}

/// Replace the entire output-enable register for GPIO 0-31.
///
/// # Safety
///
/// See the [module-level safety contract](crate::gpioc#safety).
#[inline]
pub unsafe fn mask_write_output_enable_lo(value: u32) {
    unsafe { imp::output_enable_lo_put(value) }
}

/// Atomically enable output for selected GPIO 0-31 pins.
///
/// # Safety
///
/// See the [module-level safety contract](crate::gpioc#safety).
#[inline]
pub unsafe fn mask_set_output_enable_lo(mask: u32) {
    unsafe { imp::output_enable_lo_set(mask) }
}

/// Atomically disable output for selected GPIO 0-31 pins.
///
/// # Safety
///
/// See the [module-level safety contract](crate::gpioc#safety).
#[inline]
pub unsafe fn mask_clear_output_enable_lo(mask: u32) {
    unsafe { imp::output_enable_lo_clr(mask) }
}

/// Atomically toggle output mode for selected GPIO 0-31 pins.
///
/// # Safety
///
/// See the [module-level safety contract](crate::gpioc#safety).
#[inline]
pub unsafe fn mask_toggle_output_enable_lo(mask: u32) {
    unsafe { imp::output_enable_lo_xor(mask) }
}

// Mask-level operations — high bank ------------------------------------------

/// Read the live input state of GPIO 32-63 as a compact bitmask.
///
/// # Safety
///
/// See the [module-level safety contract](crate::gpioc#safety).
#[inline]
pub unsafe fn mask_input_hi() -> u32 {
    unsafe { imp::in_hi() }
}

/// Read the programmed output register for GPIO 32-63.
///
/// # Safety
///
/// See the [module-level safety contract](crate::gpioc#safety).
#[inline]
pub unsafe fn mask_output_hi() -> u32 {
    unsafe { imp::out_hi_get() }
}

/// Replace the entire output register for GPIO 32-63.
///
/// # Safety
///
/// See the [module-level safety contract](crate::gpioc#safety).
#[inline]
pub unsafe fn mask_write_output_hi(value: u32) {
    unsafe { imp::out_hi_put(value) }
}

/// Atomically drive selected GPIO 32-63 outputs high.
///
/// # Safety
///
/// See the [module-level safety contract](crate::gpioc#safety).
#[inline]
pub unsafe fn mask_set_hi(mask: u32) {
    unsafe { imp::out_hi_set(mask) }
}

/// Atomically drive selected GPIO 32-63 outputs low.
///
/// # Safety
///
/// See the [module-level safety contract](crate::gpioc#safety).
#[inline]
pub unsafe fn mask_clr_hi(mask: u32) {
    unsafe { imp::out_hi_clr(mask) }
}

/// Atomically toggle selected GPIO 32-63 outputs.
///
/// # Safety
///
/// See the [module-level safety contract](crate::gpioc#safety).
#[inline]
pub unsafe fn mask_xor_hi(mask: u32) {
    unsafe { imp::out_hi_xor(mask) }
}

/// Read the output-enable register for GPIO 32-63.
///
/// # Safety
///
/// See the [module-level safety contract](crate::gpioc#safety).
#[inline]
pub unsafe fn mask_output_enable_hi() -> u32 {
    unsafe { imp::output_enable_hi_get() }
}

/// Replace the entire output-enable register for GPIO 32-63.
///
/// # Safety
///
/// See the [module-level safety contract](crate::gpioc#safety).
#[inline]
pub unsafe fn mask_write_output_enable_hi(value: u32) {
    unsafe { imp::output_enable_hi_put(value) }
}

/// Atomically enable output for selected GPIO 32-63 pins.
///
/// # Safety
///
/// See the [module-level safety contract](crate::gpioc#safety).
#[inline]
pub unsafe fn mask_set_output_enable_hi(mask: u32) {
    unsafe { imp::output_enable_hi_set(mask) }
}

/// Atomically disable output for selected GPIO 32-63 pins.
///
/// # Safety
///
/// See the [module-level safety contract](crate::gpioc#safety).
#[inline]
pub unsafe fn mask_clear_output_enable_hi(mask: u32) {
    unsafe { imp::output_enable_hi_clr(mask) }
}

/// Atomically toggle output mode for selected GPIO 32-63 pins.
///
/// # Safety
///
/// See the [module-level safety contract](crate::gpioc#safety).
#[inline]
pub unsafe fn mask_toggle_output_enable_hi(mask: u32) {
    unsafe { imp::output_enable_hi_xor(mask) }
}

// ============================================================================
// 64-bit atomic operations — single-cycle read/write of both low + high banks
// ============================================================================
//
// On GPIOC these are a single `MCRR`/`MRRC` instruction. On SIO they are two
// sequential 32-bit writes — still convenient for API symmetry, but not atomic
// across the 64-bit boundary on the SIO path.

/// Write all 64 output bits in a single GPIOC instruction.
///
/// Bits 0-31 → GPIO 0-31, bits 32-63 → GPIO 32-63.
///
/// # Safety
/// See the [module-level safety contract](crate::gpioc#safety).
#[inline]
pub unsafe fn out64_put(value: u64) {
    unsafe { imp::out64_put(value) }
}

/// Toggle selected output bits across all 64 GPIOs.
///
/// # Safety
/// See the [module-level safety contract](crate::gpioc#safety).
#[inline]
pub unsafe fn out64_xor(mask: u64) {
    unsafe { imp::out64_xor(mask) }
}

/// Set selected output bits high across all 64 GPIOs.
///
/// # Safety
/// See the [module-level safety contract](crate::gpioc#safety).
#[inline]
pub unsafe fn out64_set(mask: u64) {
    unsafe { imp::out64_set(mask) }
}

/// Clear selected output bits low across all 64 GPIOs.
///
/// # Safety
/// See the [module-level safety contract](crate::gpioc#safety).
#[inline]
pub unsafe fn out64_clr(mask: u64) {
    unsafe { imp::out64_clr(mask) }
}

/// Read all 64 output bits in a single GPIOC instruction.
///
/// # Safety
/// See the [module-level safety contract](crate::gpioc#safety).
#[inline]
pub unsafe fn out64_get() -> u64 {
    unsafe { imp::out64_get() }
}

/// Write all 64 output-enable bits in a single GPIOC instruction.
///
/// # Safety
/// See the [module-level safety contract](crate::gpioc#safety).
#[inline]
pub unsafe fn output_enable64_put(value: u64) {
    unsafe { imp::output_enable64_put(value) }
}

/// Toggle selected output-enable bits across all 64 GPIOs.
///
/// # Safety
/// See the [module-level safety contract](crate::gpioc#safety).
#[inline]
pub unsafe fn output_enable64_xor(mask: u64) {
    unsafe { imp::output_enable64_xor(mask) }
}

/// Set selected output-enable bits (to output) across all 64 GPIOs.
///
/// # Safety
/// See the [module-level safety contract](crate::gpioc#safety).
#[inline]
pub unsafe fn output_enable64_set(mask: u64) {
    unsafe { imp::output_enable64_set(mask) }
}

/// Clear selected output-enable bits (to input) across all 64 GPIOs.
///
/// # Safety
/// See the [module-level safety contract](crate::gpioc#safety).
#[inline]
pub unsafe fn output_enable64_clr(mask: u64) {
    unsafe { imp::output_enable64_clr(mask) }
}

/// Read all 64 output-enable bits in a single GPIOC instruction.
///
/// # Safety
/// See the [module-level safety contract](crate::gpioc#safety).
#[inline]
pub unsafe fn output_enable64_get() -> u64 {
    unsafe { imp::output_enable64_get() }
}

// ============================================================================
// Combined-input & single-bank optimizations
// ============================================================================

/// Read all 64 input pins in a single GPIOC instruction.
///
/// # Safety
/// See the [module-level safety contract](crate::gpioc#safety).
#[inline]
pub unsafe fn in64() -> u64 {
    unsafe { imp::in64() }
}
