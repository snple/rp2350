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
//! This implementation uses the `paste` crate to generate dedicated types
//! for each spinlock and interpolator lane.
//!
//! ## Example
//!
//! ```ignore
//! use pico_rs::sio::{Sio, Spinlock};
//!
//! // Get the SIO instance
//! let sio = Sio::new();
//!
//! // Get current core ID
//! let core_id = sio.core_id();
//!
//! // Use a spinlock for synchronization
//! {
//!     let _lock = Spinlock::<0>::acquire();
//!     // Critical section protected by spinlock 0
//! }
//!
//! // Read GPIO input
//! let gpio_state = sio.gpio_in();
//!
//! // Set GPIO output (atomic)
//! sio.gpio_set(1 << 25); // Set GPIO 25 high
//! ```

#![allow(clippy::identity_op)]

use paste::paste;

/// Number of hardware spinlocks
pub const SPINLOCK_COUNT: usize = 32;

// ============================================================================
// SIO Core
// ============================================================================

/// SIO (Single-cycle IO) peripheral wrapper
///
/// Provides access to core-local and inter-core hardware features.
pub struct Sio {
    sio: &'static pico_rs_pac::sio::RegisterBlock,
}

impl Sio {
    /// Create a new SIO instance
    ///
    /// # Safety
    /// This does not track ownership. The caller must ensure exclusive access
    /// to the SIO peripheral if needed.
    #[inline]
    pub fn new() -> Self {
        Self {
            sio: unsafe { &*pico_rs_pac::SIO::PTR },
        }
    }

    // ========================================================================
    // Core Information
    // ========================================================================

    /// Get the current core's ID (0 or 1)
    #[inline]
    pub fn core_id(&self) -> u32 {
        self.sio.cpuid.cpuid()
    }

    // ========================================================================
    // GPIO Input
    // ========================================================================

    /// Read GPIO0..31 input values
    #[inline]
    pub fn gpio_in(&self) -> u32 {
        self.sio.gpio_in.read_bits()
    }

    /// Read GPIO32..47 input values (bits 0-15) plus QSPI/USB pins
    #[inline]
    pub fn gpio_hi_in(&self) -> u32 {
        self.sio.gpio_hi_in.read_bits()
    }

    // ========================================================================
    // GPIO Output
    // ========================================================================

    /// Read current GPIO0..31 output value
    #[inline]
    pub fn gpio_out(&self) -> u32 {
        self.sio.gpio_out.read_bits()
    }

    /// Write GPIO0..31 output value
    #[inline]
    pub fn gpio_out_write(&self, value: u32) {
        self.sio.gpio_out.write_bits(value);
    }

    /// Atomically set bits in GPIO0..31 output
    #[inline]
    pub fn gpio_set(&self, mask: u32) {
        self.sio.gpio_out_set.write_bits(mask);
    }

    /// Atomically clear bits in GPIO0..31 output
    #[inline]
    pub fn gpio_clr(&self, mask: u32) {
        self.sio.gpio_out_clr.write_bits(mask);
    }

    /// Atomically toggle bits in GPIO0..31 output
    #[inline]
    pub fn gpio_xor(&self, mask: u32) {
        self.sio.gpio_out_xor.write_bits(mask);
    }

    /// Read current GPIO32..47 output value
    #[inline]
    pub fn gpio_hi_out(&self) -> u32 {
        self.sio.gpio_hi_out.read_bits()
    }

    /// Write GPIO32..47 output value
    #[inline]
    pub fn gpio_hi_out_write(&self, value: u32) {
        self.sio.gpio_hi_out.write_bits(value);
    }

    /// Atomically set bits in GPIO32..47 output
    #[inline]
    pub fn gpio_hi_set(&self, mask: u32) {
        self.sio.gpio_hi_out_set.write_bits(mask);
    }

    /// Atomically clear bits in GPIO32..47 output
    #[inline]
    pub fn gpio_hi_clr(&self, mask: u32) {
        self.sio.gpio_hi_out_clr.write_bits(mask);
    }

    /// Atomically toggle bits in GPIO32..47 output
    #[inline]
    pub fn gpio_hi_xor(&self, mask: u32) {
        self.sio.gpio_hi_out_xor.write_bits(mask);
    }

    // ========================================================================
    // GPIO Output Enable
    // ========================================================================

    /// Read GPIO0..31 output enable
    #[inline]
    pub fn gpio_oe(&self) -> u32 {
        self.sio.gpio_oe.read_bits()
    }

    /// Write GPIO0..31 output enable
    #[inline]
    pub fn gpio_oe_write(&self, value: u32) {
        self.sio.gpio_oe.write_bits(value);
    }

    /// Atomically set bits in GPIO0..31 output enable
    #[inline]
    pub fn gpio_oe_set(&self, mask: u32) {
        self.sio.gpio_oe_set.write_bits(mask);
    }

    /// Atomically clear bits in GPIO0..31 output enable
    #[inline]
    pub fn gpio_oe_clr(&self, mask: u32) {
        self.sio.gpio_oe_clr.write_bits(mask);
    }

    /// Atomically toggle bits in GPIO0..31 output enable
    #[inline]
    pub fn gpio_oe_xor(&self, mask: u32) {
        self.sio.gpio_oe_xor.write_bits(mask);
    }

    /// Read GPIO32..47 output enable
    #[inline]
    pub fn gpio_hi_oe(&self) -> u32 {
        self.sio.gpio_hi_oe.read_bits()
    }

    /// Write GPIO32..47 output enable
    #[inline]
    pub fn gpio_hi_oe_write(&self, value: u32) {
        self.sio.gpio_hi_oe.write_bits(value);
    }

    /// Atomically set bits in GPIO32..47 output enable
    #[inline]
    pub fn gpio_hi_oe_set(&self, mask: u32) {
        self.sio.gpio_hi_oe_set.write_bits(mask);
    }

    /// Atomically clear bits in GPIO32..47 output enable
    #[inline]
    pub fn gpio_hi_oe_clr(&self, mask: u32) {
        self.sio.gpio_hi_oe_clr.write_bits(mask);
    }

    /// Atomically toggle bits in GPIO32..47 output enable
    #[inline]
    pub fn gpio_hi_oe_xor(&self, mask: u32) {
        self.sio.gpio_hi_oe_xor.write_bits(mask);
    }

    // ========================================================================
    // Inter-core FIFO
    // ========================================================================
    //
    // NOTE: For high-level inter-core FIFO operations, use the `multicore::Fifo`
    // API which provides a more ergonomic interface. The methods below provide
    // direct low-level access to the FIFO registers if needed.
    //
    // See: `crate::multicore::Fifo`

    /// Get raw FIFO status register value
    ///
    /// Bits:
    /// - 0: VLD (RX FIFO has data)
    /// - 1: RDY (TX FIFO has space)
    /// - 2: WOF (write-on-full error)
    /// - 3: ROE (read-on-empty error)
    ///
    /// For higher-level FIFO operations, prefer using `multicore::Fifo`.
    #[inline]
    pub fn fifo_status(&self) -> u32 {
        self.sio.fifo_st.read_bits()
    }

    /// Write raw FIFO status register (for clearing error flags)
    ///
    /// Write 1 to bits 2-3 to clear WOF/ROE error flags.
    #[inline]
    pub fn fifo_status_write(&self, value: u32) {
        self.sio.fifo_st.write_bits(value);
    }

    /// Raw read from FIFO RX register
    ///
    /// For higher-level FIFO operations, prefer using `multicore::Fifo`.
    #[inline]
    pub fn fifo_rd(&self) -> u32 {
        self.sio.fifo_rd.read_bits()
    }

    /// Raw write to FIFO TX register
    ///
    /// For higher-level FIFO operations, prefer using `multicore::Fifo`.
    #[inline]
    pub fn fifo_wr(&self, value: u32) {
        self.sio.fifo_wr.write_bits(value);
    }

    // ========================================================================
    // Spinlock State
    // ========================================================================

    /// Get the state of all 32 spinlocks as a bitmap
    ///
    /// Each bit represents a spinlock: 1 = locked, 0 = unlocked.
    #[inline]
    pub fn spinlock_state(&self) -> u32 {
        self.sio.spinlock_st.read_bits()
    }

    /// Check if a specific spinlock is locked
    #[inline]
    pub fn is_spinlock_locked(&self, lock_num: u8) -> bool {
        assert!(lock_num < 32, "Spinlock number must be 0-31");
        (self.spinlock_state() & (1 << lock_num)) != 0
    }

    // ========================================================================
    // Interpolators
    // ========================================================================

    /// Get access to interpolator 0
    #[inline]
    pub fn interp0(&self) -> Interp0<'_> {
        Interp0 { sio: self.sio }
    }

    /// Get access to interpolator 1
    #[inline]
    pub fn interp1(&self) -> Interp1<'_> {
        Interp1 { sio: self.sio }
    }
}

impl Default for Sio {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Hardware Spinlocks
// ============================================================================

/// A hardware spinlock
///
/// RP2350 provides 32 hardware spinlocks. Reading from a spinlock address will:
/// - Return 0 if the lock is already locked
/// - Return a non-zero value (and claim the lock) if it was unlocked
///
/// Writing any value to the spinlock releases it.
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

/// Guard that releases the spinlock when dropped
///
/// This type is `!Send` and `!Sync` because spinlock guards are core-local.
/// The `PhantomData<*const ()>` marker prevents auto-implementation of these traits.
pub struct SpinlockGuard<const N: u8> {
    // *const () is !Send and !Sync, which prevents SpinlockGuard from being transferred
    _marker: core::marker::PhantomData<*const ()>,
}

impl<const N: u8> Drop for SpinlockGuard<N> {
    fn drop(&mut self) {
        Spinlock::<N>::release();
    }
}

/// Macro to generate spinlock register access
macro_rules! impl_spinlock_access {
    ($($num:literal => $spinlock:ident),+ $(,)?) => {
        paste! {
            impl<const N: u8> Spinlock<N> {
                const _ASSERT: () = assert!(N < 32, "Spinlock number must be 0-31");

                /// Try to acquire the spinlock (non-blocking)
                ///
                /// Returns `Some(SpinlockGuard)` if acquired, `None` if already locked.
                #[inline]
                pub fn try_acquire() -> Option<SpinlockGuard<N>> {
                    let _ = Self::_ASSERT;
                    let sio = unsafe { &*pico_rs_pac::SIO::PTR };
                    let result = match N {
                        $(
                            $num => sio.$spinlock.read_bits(),
                        )+
                        _ => unreachable!(),
                    };
                    if result != 0 {
                        Some(SpinlockGuard { _marker: core::marker::PhantomData })
                    } else {
                        None
                    }
                }

                /// Acquire the spinlock (blocking)
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

                /// Release the spinlock
                ///
                /// # Safety
                /// This should only be called if the lock is held by the current core.
                #[inline]
                pub fn release() {
                    let _ = Self::_ASSERT;
                    let sio = unsafe { &*pico_rs_pac::SIO::PTR };
                    match N {
                        $(
                            $num => sio.$spinlock.write_bits(1),
                        )+
                        _ => unreachable!(),
                    }
                }

                /// Check if the spinlock is currently locked
                #[inline]
                pub fn is_locked() -> bool {
                    let sio = unsafe { &*pico_rs_pac::SIO::PTR };
                    (sio.spinlock_st.read_bits() & (1u32 << N)) != 0
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
    28 => spinlock28, 29 => spinlock29, 30 => spinlock30,
);

// ============================================================================
// Interpolator 0
// ============================================================================

/// Interpolator 0
///
/// The interpolators are hardware-accelerated math units that perform
/// shift-and-mask operations. Each interpolator has two lanes with
/// accumulators, bases, and control registers.
///
/// Interpolator 0 additionally supports BLEND mode for linear interpolation.
pub struct Interp0<'a> {
    sio: &'a pico_rs_pac::sio::RegisterBlock,
}

impl<'a> Interp0<'a> {
    // ========================================================================
    // Accumulator Access
    // ========================================================================

    /// Read accumulator 0
    #[inline]
    pub fn accum0(&self) -> u32 {
        self.sio.interp0_accum0.read_bits()
    }

    /// Write accumulator 0
    #[inline]
    pub fn set_accum0(&self, value: u32) {
        self.sio.interp0_accum0.write_bits(value);
    }

    /// Read accumulator 1
    #[inline]
    pub fn accum1(&self) -> u32 {
        self.sio.interp0_accum1.read_bits()
    }

    /// Write accumulator 1
    #[inline]
    pub fn set_accum1(&self, value: u32) {
        self.sio.interp0_accum1.write_bits(value);
    }

    /// Add to accumulator 0 atomically and return lane 0 result
    #[inline]
    pub fn add_accum0(&self, value: u32) -> u32 {
        self.sio.interp0_accum0_add.write_bits(value);
        self.sio.interp0_accum0_add.read_bits()
    }

    /// Add to accumulator 1 atomically and return lane 1 result
    #[inline]
    pub fn add_accum1(&self, value: u32) -> u32 {
        self.sio.interp0_accum1_add.write_bits(value);
        self.sio.interp0_accum1_add.read_bits()
    }

    // ========================================================================
    // Base Register Access
    // ========================================================================

    /// Read base 0
    #[inline]
    pub fn base0(&self) -> u32 {
        self.sio.interp0_base0.read_bits()
    }

    /// Write base 0
    #[inline]
    pub fn set_base0(&self, value: u32) {
        self.sio.interp0_base0.write_bits(value);
    }

    /// Read base 1
    #[inline]
    pub fn base1(&self) -> u32 {
        self.sio.interp0_base1.read_bits()
    }

    /// Write base 1
    #[inline]
    pub fn set_base1(&self, value: u32) {
        self.sio.interp0_base1.write_bits(value);
    }

    /// Read base 2
    #[inline]
    pub fn base2(&self) -> u32 {
        self.sio.interp0_base2.read_bits()
    }

    /// Write base 2
    #[inline]
    pub fn set_base2(&self, value: u32) {
        self.sio.interp0_base2.write_bits(value);
    }

    /// Write both base 0 (lower 16 bits) and base 1 (upper 16 bits) simultaneously
    #[inline]
    pub fn set_base_1and0(&self, base0: u16, base1: u16) {
        self.sio
            .interp0_base_1and0
            .write_bits((base0 as u32) | ((base1 as u32) << 16));
    }

    // ========================================================================
    // Results (PEEK - read without side effects)
    // ========================================================================

    /// Peek lane 0 result without modifying state
    #[inline]
    pub fn peek_lane0(&self) -> u32 {
        self.sio.interp0_peek_lane0.read_bits()
    }

    /// Peek lane 1 result without modifying state
    #[inline]
    pub fn peek_lane1(&self) -> u32 {
        self.sio.interp0_peek_lane1.read_bits()
    }

    /// Peek full result without modifying state
    #[inline]
    pub fn peek_full(&self) -> u32 {
        self.sio.interp0_peek_full.read_bits()
    }

    // ========================================================================
    // Results (POP - read and update accumulators)
    // ========================================================================

    /// Pop lane 0 result and write lane results to both accumulators
    #[inline]
    pub fn pop_lane0(&self) -> u32 {
        self.sio.interp0_pop_lane0.read_bits()
    }

    /// Pop lane 1 result and write lane results to both accumulators
    #[inline]
    pub fn pop_lane1(&self) -> u32 {
        self.sio.interp0_pop_lane1.read_bits()
    }

    /// Pop full result and write lane results to both accumulators
    #[inline]
    pub fn pop_full(&self) -> u32 {
        self.sio.interp0_pop_full.read_bits()
    }

    // ========================================================================
    // Lane 0 Control
    // ========================================================================

    /// Read lane 0 control register
    #[inline]
    pub fn ctrl_lane0(&self) -> u32 {
        self.sio.interp0_ctrl_lane0.read_bits()
    }

    /// Write lane 0 control register
    #[inline]
    pub fn set_ctrl_lane0(&self, value: u32) {
        self.sio.interp0_ctrl_lane0.write_bits(value);
    }

    /// Configure lane 0 shift and mask
    ///
    /// # Arguments
    /// * `shift` - Right-rotate applied before masking (0-31)
    /// * `mask_lsb` - Least significant bit of mask (0-31)
    /// * `mask_msb` - Most significant bit of mask (0-31)
    /// * `signed` - Enable sign extension
    #[inline]
    pub fn config_lane0(&self, shift: u8, mask_lsb: u8, mask_msb: u8, signed: bool) {
        let ctrl = ((shift as u32) & 0x1F)
            | (((mask_lsb as u32) & 0x1F) << 5)
            | (((mask_msb as u32) & 0x1F) << 10)
            | if signed { 1 << 15 } else { 0 };
        self.sio.interp0_ctrl_lane0.write_bits(ctrl);
    }

    /// Enable or disable BLEND mode (interpolator 0 only)
    #[inline]
    pub fn set_blend(&self, enable: bool) {
        // Note: SIO does not support atomic SET/CLR aliases
        if enable {
            self.sio.interp0_ctrl_lane0.modify_bits(|r| r | (1 << 21));
        } else {
            self.sio.interp0_ctrl_lane0.modify_bits(|r| r & !(1 << 21));
        }
    }

    /// Check if BLEND mode is enabled
    #[inline]
    pub fn blend(&self) -> bool {
        (self.sio.interp0_ctrl_lane0.read_bits() & (1 << 21)) != 0
    }

    // ========================================================================
    // Lane 1 Control
    // ========================================================================

    /// Read lane 1 control register
    #[inline]
    pub fn ctrl_lane1(&self) -> u32 {
        self.sio.interp0_ctrl_lane1.read_bits()
    }

    /// Write lane 1 control register
    #[inline]
    pub fn set_ctrl_lane1(&self, value: u32) {
        self.sio.interp0_ctrl_lane1.write_bits(value);
    }

    /// Configure lane 1 shift and mask
    ///
    /// # Arguments
    /// * `shift` - Right-rotate applied before masking (0-31)
    /// * `mask_lsb` - Least significant bit of mask (0-31)
    /// * `mask_msb` - Most significant bit of mask (0-31)
    /// * `signed` - Enable sign extension
    #[inline]
    pub fn config_lane1(&self, shift: u8, mask_lsb: u8, mask_msb: u8, signed: bool) {
        let ctrl = ((shift as u32) & 0x1F)
            | (((mask_lsb as u32) & 0x1F) << 5)
            | (((mask_msb as u32) & 0x1F) << 10)
            | if signed { 1 << 15 } else { 0 };
        self.sio.interp0_ctrl_lane1.write_bits(ctrl);
    }

    // ========================================================================
    // Overflow Status
    // ========================================================================

    /// Check if accumulator 0 has overflow
    #[inline]
    pub fn overf0(&self) -> bool {
        (self.sio.interp0_ctrl_lane0.read_bits() & (1 << 23)) != 0
    }

    /// Check if accumulator 1 has overflow
    #[inline]
    pub fn overf1(&self) -> bool {
        (self.sio.interp0_ctrl_lane0.read_bits() & (1 << 24)) != 0
    }

    /// Check if either accumulator has overflow
    #[inline]
    pub fn overf(&self) -> bool {
        (self.sio.interp0_ctrl_lane0.read_bits() & (1 << 25)) != 0
    }
}

// ============================================================================
// Interpolator 1
// ============================================================================

/// Interpolator 1
///
/// The interpolators are hardware-accelerated math units that perform
/// shift-and-mask operations. Each interpolator has two lanes with
/// accumulators, bases, and control registers.
///
/// Interpolator 1 additionally supports CLAMP mode for clamping operations.
pub struct Interp1<'a> {
    sio: &'a pico_rs_pac::sio::RegisterBlock,
}

impl<'a> Interp1<'a> {
    // ========================================================================
    // Accumulator Access
    // ========================================================================

    /// Read accumulator 0
    #[inline]
    pub fn accum0(&self) -> u32 {
        self.sio.interp1_accum0.read_bits()
    }

    /// Write accumulator 0
    #[inline]
    pub fn set_accum0(&self, value: u32) {
        self.sio.interp1_accum0.write_bits(value);
    }

    /// Read accumulator 1
    #[inline]
    pub fn accum1(&self) -> u32 {
        self.sio.interp1_accum1.read_bits()
    }

    /// Write accumulator 1
    #[inline]
    pub fn set_accum1(&self, value: u32) {
        self.sio.interp1_accum1.write_bits(value);
    }

    /// Add to accumulator 0 atomically and return lane 0 result
    #[inline]
    pub fn add_accum0(&self, value: u32) -> u32 {
        self.sio.interp1_accum0_add.write_bits(value);
        self.sio.interp1_accum0_add.read_bits()
    }

    /// Add to accumulator 1 atomically and return lane 1 result
    #[inline]
    pub fn add_accum1(&self, value: u32) -> u32 {
        self.sio.interp1_accum1_add.write_bits(value);
        self.sio.interp1_accum1_add.read_bits()
    }

    // ========================================================================
    // Base Register Access
    // ========================================================================

    /// Read base 0
    #[inline]
    pub fn base0(&self) -> u32 {
        self.sio.interp1_base0.read_bits()
    }

    /// Write base 0
    #[inline]
    pub fn set_base0(&self, value: u32) {
        self.sio.interp1_base0.write_bits(value);
    }

    /// Read base 1
    #[inline]
    pub fn base1(&self) -> u32 {
        self.sio.interp1_base1.read_bits()
    }

    /// Write base 1
    #[inline]
    pub fn set_base1(&self, value: u32) {
        self.sio.interp1_base1.write_bits(value);
    }

    /// Read base 2
    #[inline]
    pub fn base2(&self) -> u32 {
        self.sio.interp1_base2.read_bits()
    }

    /// Write base 2
    #[inline]
    pub fn set_base2(&self, value: u32) {
        self.sio.interp1_base2.write_bits(value);
    }

    /// Write both base 0 (lower 16 bits) and base 1 (upper 16 bits) simultaneously
    #[inline]
    pub fn set_base_1and0(&self, base0: u16, base1: u16) {
        self.sio
            .interp1_base_1and0
            .write_bits((base0 as u32) | ((base1 as u32) << 16));
    }

    // ========================================================================
    // Results (PEEK - read without side effects)
    // ========================================================================

    /// Peek lane 0 result without modifying state
    #[inline]
    pub fn peek_lane0(&self) -> u32 {
        self.sio.interp1_peek_lane0.read_bits()
    }

    /// Peek lane 1 result without modifying state
    #[inline]
    pub fn peek_lane1(&self) -> u32 {
        self.sio.interp1_peek_lane1.read_bits()
    }

    /// Peek full result without modifying state
    #[inline]
    pub fn peek_full(&self) -> u32 {
        self.sio.interp1_peek_full.read_bits()
    }

    // ========================================================================
    // Results (POP - read and update accumulators)
    // ========================================================================

    /// Pop lane 0 result and write lane results to both accumulators
    #[inline]
    pub fn pop_lane0(&self) -> u32 {
        self.sio.interp1_pop_lane0.read_bits()
    }

    /// Pop lane 1 result and write lane results to both accumulators
    #[inline]
    pub fn pop_lane1(&self) -> u32 {
        self.sio.interp1_pop_lane1.read_bits()
    }

    /// Pop full result and write lane results to both accumulators
    #[inline]
    pub fn pop_full(&self) -> u32 {
        self.sio.interp1_pop_full.read_bits()
    }

    // ========================================================================
    // Lane 0 Control
    // ========================================================================

    /// Read lane 0 control register
    #[inline]
    pub fn ctrl_lane0(&self) -> u32 {
        self.sio.interp1_ctrl_lane0.read_bits()
    }

    /// Write lane 0 control register
    #[inline]
    pub fn set_ctrl_lane0(&self, value: u32) {
        self.sio.interp1_ctrl_lane0.write_bits(value);
    }

    /// Configure lane 0 shift and mask
    ///
    /// # Arguments
    /// * `shift` - Right-rotate applied before masking (0-31)
    /// * `mask_lsb` - Least significant bit of mask (0-31)
    /// * `mask_msb` - Most significant bit of mask (0-31)
    /// * `signed` - Enable sign extension
    #[inline]
    pub fn config_lane0(&self, shift: u8, mask_lsb: u8, mask_msb: u8, signed: bool) {
        let ctrl = ((shift as u32) & 0x1F)
            | (((mask_lsb as u32) & 0x1F) << 5)
            | (((mask_msb as u32) & 0x1F) << 10)
            | if signed { 1 << 15 } else { 0 };
        self.sio.interp1_ctrl_lane0.write_bits(ctrl);
    }

    /// Enable or disable CLAMP mode (interpolator 1 only)
    #[inline]
    pub fn set_clamp(&self, enable: bool) {
        // Note: SIO does not support atomic SET/CLR aliases
        if enable {
            self.sio.interp1_ctrl_lane0.modify_bits(|r| r | (1 << 22));
        } else {
            self.sio.interp1_ctrl_lane0.modify_bits(|r| r & !(1 << 22));
        }
    }

    /// Check if CLAMP mode is enabled
    #[inline]
    pub fn clamp(&self) -> bool {
        (self.sio.interp1_ctrl_lane0.read_bits() & (1 << 22)) != 0
    }

    // ========================================================================
    // Lane 1 Control
    // ========================================================================

    /// Read lane 1 control register
    #[inline]
    pub fn ctrl_lane1(&self) -> u32 {
        self.sio.interp1_ctrl_lane1.read_bits()
    }

    /// Write lane 1 control register
    #[inline]
    pub fn set_ctrl_lane1(&self, value: u32) {
        self.sio.interp1_ctrl_lane1.write_bits(value);
    }

    /// Configure lane 1 shift and mask
    ///
    /// # Arguments
    /// * `shift` - Right-rotate applied before masking (0-31)
    /// * `mask_lsb` - Least significant bit of mask (0-31)
    /// * `mask_msb` - Most significant bit of mask (0-31)
    /// * `signed` - Enable sign extension
    #[inline]
    pub fn config_lane1(&self, shift: u8, mask_lsb: u8, mask_msb: u8, signed: bool) {
        let ctrl = ((shift as u32) & 0x1F)
            | (((mask_lsb as u32) & 0x1F) << 5)
            | (((mask_msb as u32) & 0x1F) << 10)
            | if signed { 1 << 15 } else { 0 };
        self.sio.interp1_ctrl_lane1.write_bits(ctrl);
    }

    // ========================================================================
    // Overflow Status
    // ========================================================================

    /// Check if accumulator 0 has overflow
    #[inline]
    pub fn overf0(&self) -> bool {
        (self.sio.interp1_ctrl_lane0.read_bits() & (1 << 23)) != 0
    }

    /// Check if accumulator 1 has overflow
    #[inline]
    pub fn overf1(&self) -> bool {
        (self.sio.interp1_ctrl_lane0.read_bits() & (1 << 24)) != 0
    }

    /// Check if either accumulator has overflow
    #[inline]
    pub fn overf(&self) -> bool {
        (self.sio.interp1_ctrl_lane0.read_bits() & (1 << 25)) != 0
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
//
// Raw register access is available through the PAC if needed:
// - doorbell_out_set / doorbell_out_clr: Signal the other core
// - doorbell_in_set / doorbell_in_clr: Manage this core's doorbells
