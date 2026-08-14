//! PPB (Private Peripheral Bus) HAL for RP2350 Cortex-M33
//!
//! The PPB provides access to the processor's internal resources:
//! SCB (System Control), NVIC, SysTick, MPU, SAU, DWT, FPB, and ITM.
//!
//! Most of these are already covered by the PAC; this module adds
//! convenient HAL wrappers for two commonly-needed register blocks:
//!
//! - **CPACR** ([`Cpacr`]) — coprocessor access control (GPIO coprocessor,
//!   DCP, FPU).
//! - **ACTLR** ([`Actlr`]) — cross-core atomics, dual-issue, FP/ATB tuning.
//!
//! **Each core has its own independent PPB.** When running dual-core, both
//! Core 0 and Core 1 must configure these registers separately:
//!
//! ```ignore
//! use rp2350::ppb::{Access, Actlr, Coproc, Cpacr};
//!
//! // Per-core coprocessor defaults — called on Core 0 by startup::init();
//! // call it again from the Core 1 entry point.
//! Cpacr::enable_defaults(); // CP0 (GPIO), CP4 (DCP), CP10+CP11 (FPU)
//!
//! // Fine-grained control is also available.
//! Cpacr::set_access(Coproc::Cp7, Access::Full);
//!
//! // Cross-core exclusive access (Core 1).
//! Actlr::enable_crosscore_atomics();
//! ```
//!
//! ## Secure / Non-secure
//!
//! RP2350 has separate PPB register banks for Secure and Non-secure.
//! This module always uses the Secure PPB (`PPB`).

use crate::pac::PPB as PpbPac;

/// Coprocessor access level, as encoded in the 2-bit CPACR fields.
///
/// | Value | Meaning                  |
/// |-------|--------------------------|
/// | `0b00`| Access denied (default)  |
/// | `0b01`| Privileged access only   |
/// | `0b10`| Reserved                 |
/// | `0b11`| Full access              |
#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Access {
    /// Access denied (default)
    Denied = 0,
    /// Privileged access only
    Privileged = 1,
    /// Full (unprivileged) access
    Full = 3,
}

/// ARM coprocessor number for CPACR access control.
///
/// Each variant maps to the corresponding 2-bit field in CPACR
/// (field `CPn` lives at bits `2*n+1..2*n`).
///
/// On RP2350 the ones that matter are:
/// - [`Coproc::Cp0`] — GPIO coprocessor (`set_gpio`, `clr_gpio`, `xor_gpio`)
/// - [`Coproc::Cp4`] — DCP (double-precision co-processor)
/// - [`Coproc::Cp7`] — Redundancy Coprocessor (RCP, managed by bootrom)
/// - [`Coproc::Cp10`] / [`Coproc::Cp11`] — VFP floating-point unit
///   (must be programmed identically)
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Coproc {
    Cp0 = 0,
    Cp1 = 1,
    Cp2 = 2,
    Cp3 = 3,
    Cp4 = 4,
    Cp5 = 5,
    Cp6 = 6,
    Cp7 = 7,
    Cp8 = 8,
    Cp9 = 9,
    Cp10 = 10,
    Cp11 = 11,
}

/// Coprocessor Access Control Register (CPACR).
///
/// CPACR is core-local: each core's PPB has its own register, so every
/// method here applies to the **current** core only.
pub struct Cpacr;

impl Cpacr {
    /// Get a raw reference to the CPACR register.
    #[inline]
    fn regs() -> &'static rp2350_pac::ppb::Cpacr {
        // SAFETY: PPB is always present at a fixed address
        unsafe { &(*PpbPac::PTR).cpacr }
    }

    /// Read the current access level for `cp`.
    #[inline]
    pub fn access(cp: Coproc) -> Access {
        let field = (Self::regs().read_bits() >> (cp as u32 * 2)) & 0b11;
        match field {
            0 => Access::Denied,
            1 => Access::Privileged,
            _ => Access::Full,
        }
    }

    /// Set the access level for `cp` on the current core.
    #[inline]
    pub fn set_access(cp: Coproc, access: Access) {
        let offset = cp as u32 * 2;
        Self::regs().modify_bits(|r| (r & !(0b11 << offset)) | ((access as u32) << offset));
    }

    /// Grant full access to `cp` on the current core.
    #[inline]
    pub fn enable(cp: Coproc) {
        Self::set_access(cp, Access::Full);
    }

    /// Deny access to `cp` on the current core.
    #[inline]
    pub fn disable(cp: Coproc) {
        Self::set_access(cp, Access::Denied);
    }

    /// Apply the standard per-core coprocessor defaults.
    ///
    /// This mirrors pico-sdk's `runtime_init_per_core_enable_coprocessors()`:
    ///
    /// | Coprocessor | Purpose                   |
    /// |-------------|---------------------------|
    /// | CP10 + CP11 | VFP floating-point unit   |
    /// | CP4         | DCP (double co-processor) |
    /// | CP0         | GPIO coprocessor          |
    ///
    /// CP10 and CP11 must be programmed identically (CP11 mirrors CP10), so
    /// both are enabled together.
    ///
    /// Call this once per core during startup. On Core 0 it is called by
    /// [`startup::init()`](crate::startup::init); Core 1 must call it from
    /// its own entry point.
    #[inline]
    pub fn enable_defaults() {
        Self::enable(Coproc::Cp10);
        Self::enable(Coproc::Cp11);
        Self::enable(Coproc::Cp4);
        Self::enable(Coproc::Cp0);
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
