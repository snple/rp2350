//! Single-cycle IO (SIO) driver for RP2350
//!
//! The SIO provides core-local and inter-core hardware for the two processors,
//! with single-cycle access. This includes:
//!
//! - GPIO input/output with atomic set/clear/xor
//! - Inter-core FIFOs for communication between cores
//! - Hardware spinlocks for synchronization
//! - Interpolators for fast hardware-accelerated math
//!
//! ## Security Mode
//!
//! RP2350 has two SIO register sets:
//! - **SIO** (`0xd000_0000`): Secure access — full control of all GPIOs
//! - **SIO_NS** (`0xd002_0000`): Non-Secure access — Secure-only GPIOs are hidden
//!
//! The module defaults to Secure mode. Enable the `non-secure` Cargo feature
//! when building a Non-Secure image with
//! [`ImageDef::exe()`](crate::block::ImageDef::exe).
//!
//! ## Usage
//!
//! SIO is a shared, core-local peripheral. Unlike I2C or SPI, it does not
//! require ownership — you can create instances anywhere, anytime:
//!
//! ```ignore
//! use pico_rs::gpio::{Gpio, Pins};
//! use pico_rs::Peripherals;
//! use pico_rs::sio::{Sio, Spinlock};
//!
//! let dp = Peripherals::take().unwrap();
//! let pins = Pins::new(dp.IO_BANK0, dp.PADS_BANK0);
//! pins.pin::<25>().into_output();
//!
//! // Create SIO instance (zero-cost, no ownership required)
//! let sio = Sio::steal();
//!
//! // Read current core ID
//! let core = sio.core_id();
//!
//! // GPIO operations use the runtime-indexed GPIO accessor.
//! // SAFETY: GPIO 25 was configured for SIO output above.
//! let gpio = unsafe { Gpio::steal() };
//! gpio.set_high(25);      // Set GPIO 25 high
//! gpio.set_low(25);       // Set GPIO 25 low
//! let input = gpio.get(25);
//!
//! // Mask-level operations for batch control
//! gpio.set_mask_low(1 << 25); // Set GPIO 25 high (low bank mask)
//!
//! // Spinlock (static, no Sio instance needed)
//! {
//!     let _lock = Spinlock::<0>::acquire();
//!     // Critical section protected by spinlock 0
//! }
//!
//! // Interpolators
//! let interp0 = sio.interp0();
//! interp0.set_accum0(100);
//! ```

#![allow(clippy::identity_op)]

use paste::paste;
use pico_rs_pac::sio::RegisterBlock;

/// Number of hardware spinlocks
pub const SPINLOCK_COUNT: usize = 32;

// ============================================================================
// Security Mode / Register Base
// ============================================================================

/// SIO base address — determined at compile time by the `non-secure` feature.
///
/// - Default (Secure): `0xd000_0000` — full control of all GPIOs
/// - `non-secure` feature: `0xd002_0000` — Secure-only GPIOs are hidden
///
/// Both `sio::RegisterBlock` and `sio_ns::RegisterBlock` have identical memory
/// layouts, so we cast the Non-Secure address to the same type.
#[cfg(not(feature = "non-secure"))]
const SIO_BASE: u32 = pico_rs_pac::sio::BASE;
#[cfg(feature = "non-secure")]
const SIO_BASE: u32 = pico_rs_pac::sio_ns::BASE;

/// Check if SIO is configured for Secure mode (compile-time constant).
#[inline]
pub const fn is_secure() -> bool {
    cfg!(not(feature = "non-secure"))
}

/// Get the SIO register block reference.
///
/// This is the central access point for all SIO operations.
/// Uses the base address selected at compile time by the `non-secure` feature.
///
/// # Safety
///
/// Both `sio::RegisterBlock` and `sio_ns::RegisterBlock` have identical memory
/// layouts (verified from SVD). The cast from SIO_NS address to
/// `sio::RegisterBlock` is safe due to layout equivalence.
#[inline(always)]
pub(crate) const fn regs() -> &'static RegisterBlock {
    unsafe { &*(SIO_BASE as *const RegisterBlock) }
}

#[inline]
pub(crate) fn release_all_spinlocks() {
    let r = regs();

    r.spinlock0.write_spinlock0(1);
    r.spinlock1.write_spinlock1(1);
    r.spinlock2.write_spinlock2(1);
    r.spinlock3.write_spinlock3(1);
    r.spinlock4.write_spinlock4(1);
    r.spinlock5.write_spinlock5(1);
    r.spinlock6.write_spinlock6(1);
    r.spinlock7.write_spinlock7(1);
    r.spinlock8.write_spinlock8(1);
    r.spinlock9.write_spinlock9(1);
    r.spinlock10.write_spinlock10(1);
    r.spinlock11.write_spinlock11(1);
    r.spinlock12.write_spinlock12(1);
    r.spinlock13.write_spinlock13(1);
    r.spinlock14.write_spinlock14(1);
    r.spinlock15.write_spinlock15(1);
    r.spinlock16.write_spinlock16(1);
    r.spinlock17.write_spinlock17(1);
    r.spinlock18.write_spinlock18(1);
    r.spinlock19.write_spinlock19(1);
    r.spinlock20.write_spinlock20(1);
    r.spinlock21.write_spinlock21(1);
    r.spinlock22.write_spinlock22(1);
    r.spinlock23.write_spinlock23(1);
    r.spinlock24.write_spinlock24(1);
    r.spinlock25.write_spinlock25(1);
    r.spinlock26.write_spinlock26(1);
    r.spinlock27.write_spinlock27(1);
    r.spinlock28.write_spinlock28(1);
    r.spinlock29.write_spinlock29(1);
    r.spinlock30.write_spinlock30(1);
    r.spinlock31.write_spinlock31(1);
}

// ============================================================================
// SIO Core
// ============================================================================

/// SIO (Single-cycle IO) peripheral wrapper.
///
/// Creating instances is cheap. All register access goes through the
/// statically configured base address.
///
/// The `_private` field prevents external construction — use [`Sio::steal()`].
///
/// SIO is core-local and does not require ownership tracking. Multiple
/// instances can coexist safely.
#[derive(Copy, Clone, Default, Debug)]
pub struct Sio {
    _marker: core::marker::PhantomData<*const ()>,
}

impl Sio {
    /// Obtain a handle to the SIO hardware registers.
    ///
    /// This is a low-cost operation. No peripheral ownership is taken.
    #[inline]
    pub const fn steal() -> Self {
        Sio {
            _marker: core::marker::PhantomData,
        }
    }

    // ========================================================================
    // Core Information
    // ========================================================================

    /// Get the current core's ID (0 or 1).
    #[inline]
    pub fn core_id(&self) -> u32 {
        regs().cpuid.cpuid()
    }

    // ========================================================================
    // Spinlock State
    // ========================================================================

    /// Get the state of all 32 spinlocks as a bitmap.
    ///
    /// Each bit represents a spinlock: 1 = locked, 0 = unlocked.
    #[inline]
    pub fn spinlock_state(&self) -> u32 {
        regs().spinlock_st.read_bits()
    }

    /// Check if a specific spinlock is locked.
    #[inline]
    pub fn is_spinlock_locked(&self, lock_num: u8) -> bool {
        assert!(lock_num < 32, "Spinlock number must be 0-31");
        (self.spinlock_state() & (1 << lock_num)) != 0
    }

    // ========================================================================
    // Interpolators
    // ========================================================================

    /// Get access to interpolator 0.
    ///
    /// Interpolator 0 supports BLEND mode for linear interpolation.
    #[inline]
    pub const fn interp0(&self) -> Interp0<'_> {
        Interp0 {
            _marker: core::marker::PhantomData,
        }
    }

    /// Get access to interpolator 1.
    ///
    /// Interpolator 1 supports CLAMP mode for clamping operations.
    #[inline]
    pub const fn interp1(&self) -> Interp1<'_> {
        Interp1 {
            _marker: core::marker::PhantomData,
        }
    }
}

// ============================================================================
// Hardware Spinlocks
// ============================================================================

/// A hardware spinlock.
///
/// RP2350 provides 32 hardware spinlocks. Reading from a spinlock register:
/// - Returns 0 if the lock is already held
/// - Returns non-zero (and claims the lock) if it was free
///
/// Writing any value releases the lock.
///
/// ## Example
///
/// ```ignore
/// use pico_rs::sio::Spinlock;
///
/// // Acquire spinlock 0
/// {
///     let _guard = Spinlock::<0>::acquire();
///     // Critical section protected by spinlock 0
/// } // Lock automatically released when guard is dropped
/// ```
pub struct Spinlock<const N: u8>;

/// Guard that releases the spinlock when dropped.
///
/// `!Send` and `!Sync` because spinlock guards are core-local.
pub struct SpinlockGuard<const N: u8> {
    _marker: core::marker::PhantomData<*const ()>,
}

impl<const N: u8> Drop for SpinlockGuard<N> {
    fn drop(&mut self) {
        Spinlock::<N>::release();
    }
}

/// Macro to generate spinlock register access via PAC field methods.
macro_rules! impl_spinlock_access {
    ($($num:literal => $spinlock:ident),+ $(,)?) => {
        paste! {
            impl<const N: u8> Spinlock<N> {
                const _ASSERT: () = assert!(N < 32, "Spinlock number must be 0-31");

                /// Try to acquire the spinlock (non-blocking).
                ///
                /// Returns `Some(SpinlockGuard)` if acquired, `None` if already held.
                #[inline]
                pub fn try_acquire() -> Option<SpinlockGuard<N>> {
                    let _ = Self::_ASSERT;
                    let r = regs();
                    let result = match N {
                        $(
                            $num => r.$spinlock.[<$spinlock>](),
                        )+
                        _ => unreachable!(),
                    };
                    if result != 0 {
                        Some(SpinlockGuard { _marker: core::marker::PhantomData })
                    } else {
                        None
                    }
                }

                /// Acquire the spinlock (blocking).
                ///
                /// Spins until the lock is acquired.
                #[inline]
                pub fn acquire() -> SpinlockGuard<N> {
                    loop {
                        if let Some(guard) = Self::try_acquire() {
                            return guard;
                        }
                        core::hint::spin_loop();
                    }
                }

                /// Release the spinlock.
                ///
                /// # Safety
                ///
                /// This should only be called if the lock is held by the current core.
                /// Normally you should let [`SpinlockGuard`] handle this via `Drop`.
                #[inline]
                pub fn release() {
                    let _ = Self::_ASSERT;
                    let r = regs();
                    match N {
                        $(
                            $num => r.$spinlock.[<write_ $spinlock>](1),
                        )+
                        _ => unreachable!(),
                    }
                }

                /// Check if the spinlock is currently locked.
                #[inline]
                pub fn is_locked() -> bool {
                    (regs().spinlock_st.read_bits() & (1u32 << N)) != 0
                }
            }
        }
    };
}

impl_spinlock_access!(
    0 => spinlock0, 1 => spinlock1, 2 => spinlock2, 3 => spinlock3,
    4 => spinlock4, 5 => spinlock5, 6 => spinlock6, 7 => spinlock7,
    8 => spinlock8, 9 => spinlock9, 10 => spinlock10, 11 => spinlock11,
    12 => spinlock12, 13 => spinlock13, 14 => spinlock14, 15 => spinlock15,
    16 => spinlock16, 17 => spinlock17, 18 => spinlock18, 19 => spinlock19,
    20 => spinlock20, 21 => spinlock21, 22 => spinlock22, 23 => spinlock23,
    24 => spinlock24, 25 => spinlock25, 26 => spinlock26, 27 => spinlock27,
    28 => spinlock28, 29 => spinlock29, 30 => spinlock30, 31 => spinlock31,
);

// ============================================================================
// Interpolator 0
// ============================================================================

/// Interpolator 0.
///
/// Hardware-accelerated shift-and-mask unit with two lanes.
/// Interpolator 0 additionally supports BLEND mode for linear interpolation.
pub struct Interp0<'a> {
    _marker: core::marker::PhantomData<*const &'a ()>,
}

impl<'a> Interp0<'a> {
    // ========================================================================
    // Accumulator Access
    // ========================================================================

    /// Read accumulator 0.
    #[inline]
    pub fn accum0(&self) -> u32 {
        regs().interp0_accum0.read_bits()
    }

    /// Write accumulator 0.
    #[inline]
    pub fn set_accum0(&self, value: u32) {
        regs().interp0_accum0.write_bits(value);
    }

    /// Read accumulator 1.
    #[inline]
    pub fn accum1(&self) -> u32 {
        regs().interp0_accum1.read_bits()
    }

    /// Write accumulator 1.
    #[inline]
    pub fn set_accum1(&self, value: u32) {
        regs().interp0_accum1.write_bits(value);
    }

    /// Add to accumulator 0 atomically and return the add value.
    #[inline]
    pub fn add_accum0(&self, value: u32) -> u32 {
        let r = regs();
        r.interp0_accum0_add.write_interp0_accum0_add(value);
        r.interp0_accum0_add.interp0_accum0_add()
    }

    /// Add to accumulator 1 atomically and return the add value.
    #[inline]
    pub fn add_accum1(&self, value: u32) -> u32 {
        let r = regs();
        r.interp0_accum1_add.write_interp0_accum1_add(value);
        r.interp0_accum1_add.interp0_accum1_add()
    }

    // ========================================================================
    // Base Register Access
    // ========================================================================

    /// Read base 0.
    #[inline]
    pub fn base0(&self) -> u32 {
        regs().interp0_base0.read_bits()
    }

    /// Write base 0.
    #[inline]
    pub fn set_base0(&self, value: u32) {
        regs().interp0_base0.write_bits(value);
    }

    /// Read base 1.
    #[inline]
    pub fn base1(&self) -> u32 {
        regs().interp0_base1.read_bits()
    }

    /// Write base 1.
    #[inline]
    pub fn set_base1(&self, value: u32) {
        regs().interp0_base1.write_bits(value);
    }

    /// Read base 2.
    #[inline]
    pub fn base2(&self) -> u32 {
        regs().interp0_base2.read_bits()
    }

    /// Write base 2.
    #[inline]
    pub fn set_base2(&self, value: u32) {
        regs().interp0_base2.write_bits(value);
    }

    /// Write both base 0 (lower 16 bits) and base 1 (upper 16 bits) simultaneously.
    #[inline]
    pub fn set_base_1and0(&self, base0: u16, base1: u16) {
        regs()
            .interp0_base_1and0
            .write_interp0_base_1and0((base0 as u32) | ((base1 as u32) << 16));
    }

    // ========================================================================
    // Results (PEEK — read without side effects)
    // ========================================================================

    /// Peek lane 0 result without modifying state.
    #[inline]
    pub fn peek_lane0(&self) -> u32 {
        regs().interp0_peek_lane0.read_bits()
    }

    /// Peek lane 1 result without modifying state.
    #[inline]
    pub fn peek_lane1(&self) -> u32 {
        regs().interp0_peek_lane1.read_bits()
    }

    /// Peek full result without modifying state.
    #[inline]
    pub fn peek_full(&self) -> u32 {
        regs().interp0_peek_full.read_bits()
    }

    // ========================================================================
    // Results (POP — read and update accumulators)
    // ========================================================================

    /// Pop lane 0 result and write lane results to both accumulators.
    #[inline]
    pub fn pop_lane0(&self) -> u32 {
        regs().interp0_pop_lane0.read_bits()
    }

    /// Pop lane 1 result and write lane results to both accumulators.
    #[inline]
    pub fn pop_lane1(&self) -> u32 {
        regs().interp0_pop_lane1.read_bits()
    }

    /// Pop full result and write lane results to both accumulators.
    #[inline]
    pub fn pop_full(&self) -> u32 {
        regs().interp0_pop_full.read_bits()
    }

    // ========================================================================
    // Lane 0 Control
    // ========================================================================

    /// Read lane 0 control register raw value.
    #[inline]
    pub fn ctrl_lane0(&self) -> u32 {
        regs().interp0_ctrl_lane0.read_bits()
    }

    /// Write lane 0 control register raw value.
    #[inline]
    pub fn set_ctrl_lane0(&self, value: u32) {
        regs().interp0_ctrl_lane0.write_bits(value);
    }

    /// Configure lane 0 shift and mask using PAC field methods.
    ///
    /// # Arguments
    /// * `shift` — Right-rotate applied before masking (0–31)
    /// * `mask_lsb` — Least significant bit of mask (0–31)
    /// * `mask_msb` — Most significant bit of mask (0–31)
    /// * `signed` — Enable sign extension
    #[inline]
    pub fn config_lane0(&self, shift: u8, mask_lsb: u8, mask_msb: u8, signed: bool) {
        regs().interp0_ctrl_lane0.modify_val(|v| {
            v.with_shift(shift as u32)
                .with_mask_lsb(mask_lsb as u32)
                .with_mask_msb(mask_msb as u32)
                .with_signed(signed)
        });
    }

    /// Enable or disable BLEND mode (interpolator 0 only).
    #[inline]
    pub fn set_blend(&self, enable: bool) {
        regs()
            .interp0_ctrl_lane0
            .modify_val(|v| v.with_blend(enable));
    }

    /// Check if BLEND mode is enabled.
    #[inline]
    pub fn blend(&self) -> bool {
        regs().interp0_ctrl_lane0.blend()
    }

    // ========================================================================
    // Lane 1 Control
    // ========================================================================

    /// Read lane 1 control register raw value.
    #[inline]
    pub fn ctrl_lane1(&self) -> u32 {
        regs().interp0_ctrl_lane1.read_bits()
    }

    /// Write lane 1 control register raw value.
    #[inline]
    pub fn set_ctrl_lane1(&self, value: u32) {
        regs().interp0_ctrl_lane1.write_bits(value);
    }

    /// Configure lane 1 shift and mask.
    #[inline]
    pub fn config_lane1(&self, shift: u8, mask_lsb: u8, mask_msb: u8, signed: bool) {
        regs().interp0_ctrl_lane1.modify_val(|v| {
            v.with_shift(shift as u32)
                .with_mask_lsb(mask_lsb as u32)
                .with_mask_msb(mask_msb as u32)
                .with_signed(signed)
        });
    }

    // ========================================================================
    // Overflow Status (read-only)
    // ========================================================================

    /// Check if accumulator 0 has overflow.
    #[inline]
    pub fn overf0(&self) -> bool {
        regs().interp0_ctrl_lane0.overf0()
    }

    /// Check if accumulator 1 has overflow.
    #[inline]
    pub fn overf1(&self) -> bool {
        regs().interp0_ctrl_lane0.overf1()
    }

    /// Check if either accumulator has overflow.
    #[inline]
    pub fn overf(&self) -> bool {
        regs().interp0_ctrl_lane0.overf()
    }
}

// ============================================================================
// Interpolator 1
// ============================================================================

/// Interpolator 1.
///
/// Hardware-accelerated shift-and-mask unit with two lanes.
/// Interpolator 1 additionally supports CLAMP mode for clamping operations.
pub struct Interp1<'a> {
    _marker: core::marker::PhantomData<*const &'a ()>,
}

impl<'a> Interp1<'a> {
    // ========================================================================
    // Accumulator Access
    // ========================================================================

    /// Read accumulator 0.
    #[inline]
    pub fn accum0(&self) -> u32 {
        regs().interp1_accum0.read_bits()
    }

    /// Write accumulator 0.
    #[inline]
    pub fn set_accum0(&self, value: u32) {
        regs().interp1_accum0.write_bits(value);
    }

    /// Read accumulator 1.
    #[inline]
    pub fn accum1(&self) -> u32 {
        regs().interp1_accum1.read_bits()
    }

    /// Write accumulator 1.
    #[inline]
    pub fn set_accum1(&self, value: u32) {
        regs().interp1_accum1.write_bits(value);
    }

    /// Add to accumulator 0 atomically and return the add value.
    #[inline]
    pub fn add_accum0(&self, value: u32) -> u32 {
        let r = regs();
        r.interp1_accum0_add.write_interp1_accum0_add(value);
        r.interp1_accum0_add.interp1_accum0_add()
    }

    /// Add to accumulator 1 atomically and return the add value.
    #[inline]
    pub fn add_accum1(&self, value: u32) -> u32 {
        let r = regs();
        r.interp1_accum1_add.write_interp1_accum1_add(value);
        r.interp1_accum1_add.interp1_accum1_add()
    }

    // ========================================================================
    // Base Register Access
    // ========================================================================

    /// Read base 0.
    #[inline]
    pub fn base0(&self) -> u32 {
        regs().interp1_base0.read_bits()
    }

    /// Write base 0.
    #[inline]
    pub fn set_base0(&self, value: u32) {
        regs().interp1_base0.write_bits(value);
    }

    /// Read base 1.
    #[inline]
    pub fn base1(&self) -> u32 {
        regs().interp1_base1.read_bits()
    }

    /// Write base 1.
    #[inline]
    pub fn set_base1(&self, value: u32) {
        regs().interp1_base1.write_bits(value);
    }

    /// Read base 2.
    #[inline]
    pub fn base2(&self) -> u32 {
        regs().interp1_base2.read_bits()
    }

    /// Write base 2.
    #[inline]
    pub fn set_base2(&self, value: u32) {
        regs().interp1_base2.write_bits(value);
    }

    /// Write both base 0 (lower 16 bits) and base 1 (upper 16 bits) simultaneously.
    #[inline]
    pub fn set_base_1and0(&self, base0: u16, base1: u16) {
        regs()
            .interp1_base_1and0
            .write_interp1_base_1and0((base0 as u32) | ((base1 as u32) << 16));
    }

    // ========================================================================
    // Results (PEEK — read without side effects)
    // ========================================================================

    /// Peek lane 0 result without modifying state.
    #[inline]
    pub fn peek_lane0(&self) -> u32 {
        regs().interp1_peek_lane0.read_bits()
    }

    /// Peek lane 1 result without modifying state.
    #[inline]
    pub fn peek_lane1(&self) -> u32 {
        regs().interp1_peek_lane1.read_bits()
    }

    /// Peek full result without modifying state.
    #[inline]
    pub fn peek_full(&self) -> u32 {
        regs().interp1_peek_full.read_bits()
    }

    // ========================================================================
    // Results (POP — read and update accumulators)
    // ========================================================================

    /// Pop lane 0 result and write lane results to both accumulators.
    #[inline]
    pub fn pop_lane0(&self) -> u32 {
        regs().interp1_pop_lane0.read_bits()
    }

    /// Pop lane 1 result and write lane results to both accumulators.
    #[inline]
    pub fn pop_lane1(&self) -> u32 {
        regs().interp1_pop_lane1.read_bits()
    }

    /// Pop full result and write lane results to both accumulators.
    #[inline]
    pub fn pop_full(&self) -> u32 {
        regs().interp1_pop_full.read_bits()
    }

    // ========================================================================
    // Lane 0 Control
    // ========================================================================

    /// Read lane 0 control register raw value.
    #[inline]
    pub fn ctrl_lane0(&self) -> u32 {
        regs().interp1_ctrl_lane0.read_bits()
    }

    /// Write lane 0 control register raw value.
    #[inline]
    pub fn set_ctrl_lane0(&self, value: u32) {
        regs().interp1_ctrl_lane0.write_bits(value);
    }

    /// Configure lane 0 shift and mask using PAC field methods.
    #[inline]
    pub fn config_lane0(&self, shift: u8, mask_lsb: u8, mask_msb: u8, signed: bool) {
        regs().interp1_ctrl_lane0.modify_val(|v| {
            v.with_shift(shift as u32)
                .with_mask_lsb(mask_lsb as u32)
                .with_mask_msb(mask_msb as u32)
                .with_signed(signed)
        });
    }

    /// Enable or disable CLAMP mode (interpolator 1 only).
    #[inline]
    pub fn set_clamp(&self, enable: bool) {
        regs()
            .interp1_ctrl_lane0
            .modify_val(|v| v.with_clamp(enable));
    }

    /// Check if CLAMP mode is enabled.
    #[inline]
    pub fn clamp(&self) -> bool {
        regs().interp1_ctrl_lane0.clamp()
    }

    // ========================================================================
    // Lane 1 Control
    // ========================================================================

    /// Read lane 1 control register raw value.
    #[inline]
    pub fn ctrl_lane1(&self) -> u32 {
        regs().interp1_ctrl_lane1.read_bits()
    }

    /// Write lane 1 control register raw value.
    #[inline]
    pub fn set_ctrl_lane1(&self, value: u32) {
        regs().interp1_ctrl_lane1.write_bits(value);
    }

    /// Configure lane 1 shift and mask.
    #[inline]
    pub fn config_lane1(&self, shift: u8, mask_lsb: u8, mask_msb: u8, signed: bool) {
        regs().interp1_ctrl_lane1.modify_val(|v| {
            v.with_shift(shift as u32)
                .with_mask_lsb(mask_lsb as u32)
                .with_mask_msb(mask_msb as u32)
                .with_signed(signed)
        });
    }

    // ========================================================================
    // Overflow Status (read-only)
    // ========================================================================

    /// Check if accumulator 0 has overflow.
    #[inline]
    pub fn overf0(&self) -> bool {
        regs().interp1_ctrl_lane0.overf0()
    }

    /// Check if accumulator 1 has overflow.
    #[inline]
    pub fn overf1(&self) -> bool {
        regs().interp1_ctrl_lane0.overf1()
    }

    /// Check if either accumulator has overflow.
    #[inline]
    pub fn overf(&self) -> bool {
        regs().interp1_ctrl_lane0.overf()
    }
}

// ============================================================================
// Doorbell Interrupts (RP2350 specific)
// ============================================================================
//
// NOTE: For doorbell interrupt operations, use the `multicore::Doorbell` API
// which provides a more ergonomic interface with proper safety guarantees.
//
// See: `crate::multicore::Doorbell`
