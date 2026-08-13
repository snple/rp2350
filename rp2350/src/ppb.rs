//! PPB (Private Peripheral Bus) HAL for RP2350 Cortex-M33
//!
//! The PPB provides access to the processor's internal resources:
//! SCB (System Control), NVIC, SysTick, MPU, SAU, DWT, FPB, and ITM.
//!
//! Most of these are already covered by the PAC; this module adds
//! convenient HAL wrappers for commonly-needed operations.
//!
//! **Each core has its own independent PPB.**  When running dual-core,
//! both Core 0 and Core 1 must call these functions separately (or use
//! [`Cpacr::init_per_core_defaults()`] which is the standard way).
//!
//! ## Features
//!
//! - Coprocessor Access Control (CPACR) — enable GPIO coprocessor, FPU, etc.
//!
//! ## Example — typical dual-core startup
//!
//! ```ignore
//! use rp2350::ppb::Cpacr;
//!
//! // Core 0: called automatically in startup::init()
//! Cpacr::init_per_core_defaults();  // enables CP0, CP4, CP10
//!
//! // Core 1: call in its entry point
//! fn core1_main() -> ! {
//!     Cpacr::init_per_core_defaults();
//!     // ... application code ...
//! }
//! ```
//!
//! ## Secure / Non-secure
//!
//! RP2350 has separate PPB register banks for Secure and Non-secure.
//! This module always uses the Secure PPB (`PPB`).

use crate::pac::PPB as PpbPac;

/// Coprocessor Access Control Register (CPACR).
///
/// Each ARM coprocessor (CP0-CP11) has a 2-bit access field:
///
/// | Value | Meaning                  |
/// |-------|--------------------------|
/// | `0b00`| Access denied (default)  |
/// | `0b01`| Privileged access only   |
/// | `0b10`| Reserved                 |
/// | `0b11`| Full access              |
///
/// On RP2350:
/// - **CP0** — GPIO coprocessor instructions (`set_gpio`, `clr_gpio`, `xor_gpio`)
/// - **CP4** — Double-precision float (reserved on RP2350, enabled by default)
/// - **CP7** — Redundancy Coprocessor (RCP, managed by bootrom)
/// - **CP10/CP11** — VFP floating-point unit
///
/// # Per-core note
///
/// CPACR is core-local — each core's PPB has its own CPACR register.
/// Call [`init_per_core_defaults()`](Self::init_per_core_defaults) in each
/// core's entry point, or use the individual `enable_cp*()` methods for
/// fine-grained control.
pub struct Cpacr;

impl Cpacr {
    /// Full access value for a coprocessor field (0b11)
    const FULL_ACCESS: u32 = 0b11;

    /// Get a raw reference to the CPACR register.
    #[inline]
    fn regs() -> &'static rp2350_pac::ppb::Cpacr {
        // SAFETY: PPB is always present at a fixed address
        unsafe { &(*PpbPac::PTR).cpacr }
    }

    // ========================================================================
    // Individual coprocessor enable/disable
    // ========================================================================

    /// Enable coprocessor 0 (GPIO) with full access on the current core.
    #[inline]
    pub fn enable_cp0() {
        Self::regs()
            .modify_bits(|r| r | (Self::FULL_ACCESS << rp2350_pac::ppb::Cpacr::CP0_OFFSET));
    }

    /// Enable coprocessor 4 (double-precision float) on the current core.
    #[inline]
    pub fn enable_cp4() {
        Self::regs()
            .modify_bits(|r| r | (Self::FULL_ACCESS << rp2350_pac::ppb::Cpacr::CP4_OFFSET));
    }

    /// Enable CP10 (VFP floating-point) with full access on the current core.
    #[inline]
    pub fn enable_cp10() {
        Self::regs()
            .modify_bits(|r| r | (Self::FULL_ACCESS << rp2350_pac::ppb::Cpacr::CP10_OFFSET));
    }

    /// Set the access level for any coprocessor (0-11).
    ///
    /// # Safety
    ///
    /// `cp` must be 0-11.  `access` should be `0b11` for full access
    /// or `0b01` for privileged-only; other values are reserved.
    #[inline]
    pub unsafe fn set(cp: u8, access: u32) {
        debug_assert!(cp <= 11, "coprocessor {} out of range", cp);
        debug_assert!(access <= 3, "invalid access level {}", access);
        let offset = cp as u32 * 2;
        Self::regs().modify_bits(|r| (r & !(3 << offset)) | (access << offset));
    }

    // ========================================================================
    // Per-core defaults (pico-sdk-compatible)
    // ========================================================================

    /// Apply the standard per-core coprocessor defaults for the current core.
    ///
    /// This mirrors pico-sdk's `runtime_init_per_core_enable_coprocessors()`:
    ///
    /// | Coprocessor | Condition      |
    /// |-------------|----------------|
    /// | CP10 (VFP)  | Always enabled |
    /// | CP4  (dbl)  | Always enabled |
    /// | CP0  (GPIO) | Always enabled |
    ///
    /// Call this once per core during startup.  This is what
    /// [`startup::init()`](crate::startup::init) calls on Core 0.
    #[inline]
    pub fn enable_cp() {
        // CP10 (VFP float) — always enabled, same as pico-sdk
        Self::enable_cp10();

        // CP4 — gated by feature flag
        Self::enable_cp4();

        // CP0 — gated by feature flag
        Self::enable_cp0();
    }
}

// ============================================================================
// ACTLR — Auxiliary Control Register
// ============================================================================

/// Auxiliary Control Register (ACTLR) for Cortex-M33 on RP2350.
///
/// Address: `0xE000_E008` (PPB).  IMPLEMENTATION DEFINED — controls
/// processor behaviour beyond the standard ARMv8-M architecture.
///
/// # Per-core note
///
/// Like CPACR, ACTLR is **core-local** — each core has its own copy.
/// Call the setters on **both** Core 0 and Core 1 during startup.
///
/// # Register fields
///
/// | Bit | Field           | PAC accessor           | Purpose |
/// |-----|-----------------|------------------------|---------|
/// | 29  | EXTEXCLALL      | `enable_crosscore_atomics()`     | Cross-core LDREX/STREX without MPU shareable regions |
/// | 12  | DISITMATBFLUSH  | `set_disitmatbflush()` | Disable ATB flush |
/// | 10  | FPEXCODIS       | `set_fpexcodis()`      | Disable FPU exception output |
/// | 9   | DISOOFP         | `set_disoofp()`        | Disable out-of-order FP completion |
/// | 2   | DISFOLD         | `set_disfold()`        | Disable dual-issue (branch folding) |
/// | 0   | DISMCYCINT      | `set_dismcycint()`     | Disable dual-issue (multi-cycle) |
///
/// ```ignore
/// use rp2350::ppb::Actlr;
///
/// // Core 0 — handled by startup::init()
/// // Core 1 — call in its entry point:
/// fn core1_main() -> ! {
///     Actlr::enable_crosscore_atomics();  // EXTEXCLALL
///     // ...
/// }
/// ```
pub struct Actlr;

impl Actlr {
    /// Get a raw reference to the ACTLR register.
    #[inline]
    fn regs() -> &'static rp2350_pac::ppb::Actlr {
        // SAFETY: PPB is always present at a fixed address
        unsafe { &(*PpbPac::PTR).actlr }
    }

    // ========================================================================
    // EXTEXCLALL (bit 29)
    // ========================================================================

    /// Enable cross-core exclusive-access (Atomics, spinlocks).
    ///
    /// Sets **EXTEXCLALL** — the single most important bit for multicore
    /// on bare-metal RP2350.
    ///
    /// ## Why it matters
    ///
    /// ARMv8-M `Atomic*` / `LDREX`-`STREX` only synchronise across cores
    /// when the accessed memory is marked **Shareable** by the MPU.
    /// Without this bit, the processor uses a core-local exclusive monitor
    /// and cross-core atomics silently fail (no exception, just corrupt data).
    ///
    /// EXTEXCLALL=1 promotes **all** memory to the global exclusive monitor,
    /// so atomics work between Core 0 and Core 1 with zero MPU configuration.
    ///
    /// ## When to call
    ///
    /// | Environment                             | Status                 |
    /// |-----------------------------------------|------------------------|
    /// | `rp2350::startup::init()` (Core 0)     | ✅ called automatically |
    /// | Core 1 entry point                      | ❗ call this manually   |
    /// | Hardware reset                          | 0 (disabled)           |
    ///
    /// SIO hardware spinlocks (`rp2350::sio::Spinlock`) do **not** need
    /// this bit — they use a dedicated hardware mechanism unrelated to
    /// LDREX/STREX.
    #[inline]
    pub fn enable_crosscore_atomics() {
        Self::regs().set_extexclall();
    }

    /// Disable cross-core exclusive-access.
    ///
    /// Clears EXTEXCLALL.  After this, `Atomic*` reverts to core-local
    /// behaviour.  Only useful if you need per-core exclusive regions
    /// managed by the MPU.
    #[inline]
    pub fn disable_crosscore_atomics() {
        Self::regs().clr_extexclall();
    }

    /// Check whether cross-core atomics are enabled.
    #[inline]
    pub fn crosscore_atomics_enabled() -> bool {
        Self::regs().extexclall()
    }

    // ========================================================================
    // Dual-issue control (bits 2, 0)
    // ========================================================================

    /// Disable dual-issue on the current core.
    ///
    /// Cortex-M33 can dual-issue (execute two instructions per cycle).
    /// Disabling it gives **deterministic cycle counts** at the cost of
    /// roughly halving throughput.  Useful for cycle-accurate timing.
    ///
    /// Sets both **DISFOLD** (bit 2) and **DISMCYCINT** (bit 0).
    #[inline]
    pub fn disable_dual_issue() {
        let regs = Self::regs();
        regs.set_disfold();
        regs.set_dismcycint();
    }

    /// Enable dual-issue on the current core (default).
    ///
    /// Clears both DISFOLD and DISMCYCINT.
    #[inline]
    pub fn enable_dual_issue() {
        let regs = Self::regs();
        regs.clr_disfold();
        regs.clr_dismcycint();
    }

    /// Check whether dual-issue is currently disabled.
    #[inline]
    pub fn dual_issue_disabled() -> bool {
        let regs = Self::regs();
        regs.disfold() || regs.dismcycint()
    }

    // ========================================================================
    // FPU control (bits 10, 9)
    // ========================================================================

    /// Disable out-of-order FP instruction completion.
    ///
    /// Sets **DISOOFP** (bit 9).  Forces FP instructions to retire in
    /// program order, which simplifies FP exception handling at some
    /// cost to FP throughput.
    #[inline]
    pub fn disable_ooo_fp() {
        Self::regs().set_disoofp();
    }

    /// Enable out-of-order FP completion (default).
    #[inline]
    pub fn enable_ooo_fp() {
        Self::regs().clr_disoofp();
    }

    /// Check if out-of-order FP is disabled.
    #[inline]
    pub fn ooo_fp_disabled() -> bool {
        Self::regs().disoofp()
    }

    /// Disable FPU exception outputs.
    ///
    /// Sets **FPEXCODIS** (bit 10).  Suppresses FPU exception assertion.
    /// Use with caution — FP exceptions will be silently lost.
    #[inline]
    pub fn disable_fp_exception_output() {
        Self::regs().set_fpexcodis();
    }

    /// Enable FPU exception outputs (default).
    #[inline]
    pub fn enable_fp_exception_output() {
        Self::regs().clr_fpexcodis();
    }

    /// Check if FP exception outputs are disabled.
    #[inline]
    pub fn fp_exception_output_disabled() -> bool {
        Self::regs().fpexcodis()
    }

    // ========================================================================
    // ATB flush (bit 12)
    // ========================================================================

    /// Disable ATB flush.
    #[inline]
    pub fn disable_atb_flush() {
        Self::regs().set_disitmatbflush();
    }

    /// Enable ATB flush (default).
    #[inline]
    pub fn enable_atb_flush() {
        Self::regs().clr_disitmatbflush();
    }

    /// Check if ATB flush is disabled.
    #[inline]
    pub fn atb_flush_disabled() -> bool {
        Self::regs().disitmatbflush()
    }
}
