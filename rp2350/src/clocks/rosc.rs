//! Ring Oscillator (ROSC) driver for RP2350
//!
//! The ROSC is an on-chip oscillator that provides a clock source that doesn't
//! require an external crystal. It can also be used as a source of random bits.
//!
//! ## Example
//!
//! ```ignore
//! use rp2350::clocks::rosc::{Rosc, FreqRange};
//! use rp2350::Peripherals;
//!
//! let dp = Peripherals::take().unwrap();
//! let rosc = Rosc::init(dp.ROSC);
//!
//! // Enable the oscillator
//! rosc.enable();
//!
//! // Wait for stable
//! while !rosc.is_stable() {}
//!
//! // Set frequency range
//! rosc.set_freq_range(FreqRange::Medium);
//!
//! // Get random bits
//! let random_bit = rosc.get_random_bit();
//! let random_byte = rosc.get_random_byte();
//! ```

use crate::Steal;
use rp2350_pac::ROSC;
use rp2350_pac::rosc::{Ctrl, Freqa, Freqb};

/// Nominal ROSC frequency (~6.5 MHz, varies with temperature and voltage)
pub const ROSC_HZ: u32 = 6_500_000;

/// ROSC frequency range
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FreqRange {
    /// Low frequency range (~1-15 MHz)
    Low = 0xFA4,
    /// Medium frequency range (~4-45 MHz)
    Medium = 0xFA5,
    /// High frequency range (~15-100 MHz)
    High = 0xFA7,
    /// Too high (not recommended)
    TooHigh = 0xFA6,
}

/// ROSC drive strength (0-7)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct DriveStrength(pub u8);

impl DriveStrength {
    /// Minimum drive strength
    pub const MIN: Self = Self(0);
    /// Maximum drive strength
    pub const MAX: Self = Self(7);

    /// Create a new drive strength value, clamped to valid range
    #[inline]
    pub const fn new(value: u8) -> Self {
        Self(if value > 7 { 7 } else { value })
    }
}

/// ROSC divider value (1-32)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Divider(pub u8);

impl Divider {
    /// No division (pass-through)
    pub const DIV1: Self = Self(0);
    /// Divide by 2
    pub const DIV2: Self = Self(2);
    /// Divide by 4
    pub const DIV4: Self = Self(4);
    /// Divide by 8
    pub const DIV8: Self = Self(8);
    /// Divide by 16
    pub const DIV16: Self = Self(16);
    /// Divide by 32
    pub const DIV32: Self = Self(32);

    /// Create a new divider value, clamped to valid range (0-32)
    #[inline]
    pub const fn new(value: u8) -> Self {
        Self(if value > 32 { 32 } else { value })
    }
}

/// ROSC password for protected writes
const ROSC_PASSWORD: u32 = 0x9696;

/// Value to write to DORMANT register to enter dormant mode
const ROSC_DORMANT_VALUE: u32 = 0x636f6d61;

/// Ring Oscillator driver
pub struct Rosc {
    rosc: rp2350_pac::ROSC,
}

impl Rosc {
    /// Initialize the ROSC driver.
    ///
    /// Takes ownership of the ROSC peripheral. The ring oscillator does not
    /// require a reset sequence — it runs from the always-on power domain.
    #[inline]
    pub fn init(rosc: ROSC) -> Self {
        Self { rosc }
    }

    /// Obtain a handle to the ROSC hardware peripheral.
    ///
    /// # Safety
    /// The caller must ensure only one instance of this driver exists at a time.
    #[inline]
    pub unsafe fn steal() -> Steal<Self> {
        Steal(Self {
            rosc: unsafe { ROSC::steal() },
        })
    }

    /// Get a reference to the ROSC registers
    #[inline]
    fn regs(&self) -> &rp2350_pac::rosc::RegisterBlock {
        &self.rosc
    }

    // ========================================================================
    // Enable/Disable Control
    // ========================================================================

    /// Enable the ring oscillator
    #[inline]
    pub fn enable(&self) {
        let rosc = self.regs();
        rosc.ctrl
            .write_enable(Ctrl::ENABLE_ENABLE >> Ctrl::ENABLE_OFFSET);
    }

    /// Disable the ring oscillator
    ///
    /// # Safety
    /// The system clock must not be using ROSC when disabling it.
    #[inline]
    pub fn disable(&self) {
        let rosc = self.regs();
        rosc.ctrl
            .write_enable(Ctrl::ENABLE_DISABLE >> Ctrl::ENABLE_OFFSET);
    }

    /// Check if the oscillator is enabled
    #[inline]
    pub fn is_enabled(&self) -> bool {
        self.regs().status.enabled()
    }

    /// Check if the oscillator is stable
    #[inline]
    pub fn is_stable(&self) -> bool {
        self.regs().status.stable()
    }

    /// Check if the divider is running
    #[inline]
    pub fn is_divider_running(&self) -> bool {
        self.regs().status.div_running()
    }

    /// Check if a bad write occurred
    #[inline]
    pub fn had_badwrite(&self) -> bool {
        self.regs().status.badwrite()
    }

    /// Clear the badwrite flag
    #[inline]
    pub fn clear_badwrite(&mut self) {
        self.regs().status.clr_badwrite();
    }

    // ========================================================================
    // Frequency Control
    // ========================================================================

    /// Set the frequency range
    #[inline]
    pub fn set_freq_range(&mut self, range: FreqRange) {
        let rosc = self.regs();
        rosc.ctrl.write_freq_range(range as u32);
    }

    /// Get the current frequency range setting
    #[inline]
    pub fn freq_range(&self) -> u32 {
        self.regs().ctrl.freq_range()
    }

    /// Set drive strength for stage 0
    ///
    /// # Arguments
    /// * `strength` - Drive strength (0-7)
    /// * `randomize` - If true, randomize this stage's drive strength
    #[inline]
    pub fn set_stage0_drive(&mut self, strength: DriveStrength, randomize: bool) {
        let rosc = self.regs();
        let mut val = rosc.freqa.read_bits();
        val = (val & !Freqa::DS0_MASK) | ((strength.0 as u32) << Freqa::DS0_OFFSET);
        if randomize {
            val |= Freqa::DS0_RANDOM_MASK;
        } else {
            val &= !Freqa::DS0_RANDOM_MASK;
        }
        // Apply password and write
        val = (val & 0x0000FFFF) | (ROSC_PASSWORD << Freqa::PASSWD_OFFSET);
        rosc.freqa.write_bits(val);
    }

    /// Set drive strength for stage 1
    #[inline]
    pub fn set_stage1_drive(&mut self, strength: DriveStrength, randomize: bool) {
        let rosc = self.regs();
        let mut val = rosc.freqa.read_bits();
        val = (val & !Freqa::DS1_MASK) | ((strength.0 as u32) << Freqa::DS1_OFFSET);
        if randomize {
            val |= Freqa::DS1_RANDOM_MASK;
        } else {
            val &= !Freqa::DS1_RANDOM_MASK;
        }
        val = (val & 0x0000FFFF) | (ROSC_PASSWORD << Freqa::PASSWD_OFFSET);
        rosc.freqa.write_bits(val);
    }

    /// Set drive strength for stage 2
    #[inline]
    pub fn set_stage2_drive(&mut self, strength: DriveStrength) {
        let rosc = self.regs();
        let mut val = rosc.freqa.read_bits();
        val = (val & !Freqa::DS2_MASK) | ((strength.0 as u32) << Freqa::DS2_OFFSET);
        val = (val & 0x0000FFFF) | (ROSC_PASSWORD << Freqa::PASSWD_OFFSET);
        rosc.freqa.write_bits(val);
    }

    /// Set drive strength for stage 3
    #[inline]
    pub fn set_stage3_drive(&mut self, strength: DriveStrength) {
        let rosc = self.regs();
        let mut val = rosc.freqa.read_bits();
        val = (val & !Freqa::DS3_MASK) | ((strength.0 as u32) << Freqa::DS3_OFFSET);
        val = (val & 0x0000FFFF) | (ROSC_PASSWORD << Freqa::PASSWD_OFFSET);
        rosc.freqa.write_bits(val);
    }

    /// Set drive strength for stages 4-7 using FREQB register
    ///
    /// # Arguments
    /// * `ds4` - Stage 4 drive strength
    /// * `ds5` - Stage 5 drive strength
    /// * `ds6` - Stage 6 drive strength
    /// * `ds7` - Stage 7 drive strength
    #[inline]
    pub fn set_stages_4_7_drive(
        &mut self,
        ds4: DriveStrength,
        ds5: DriveStrength,
        ds6: DriveStrength,
        ds7: DriveStrength,
    ) {
        let rosc = self.regs();
        let val = (ROSC_PASSWORD << Freqb::PASSWD_OFFSET)
            | ((ds7.0 as u32) << Freqb::DS7_OFFSET)
            | ((ds6.0 as u32) << Freqb::DS6_OFFSET)
            | ((ds5.0 as u32) << Freqb::DS5_OFFSET)
            | ((ds4.0 as u32) << Freqb::DS4_OFFSET);
        rosc.freqb.write_bits(val);
    }

    /// Enable drive strength randomization for better random number generation
    #[inline]
    pub fn enable_randomization(&mut self) {
        let rosc = self.regs();
        let mut val = rosc.freqa.read_bits();
        val |= Freqa::DS0_RANDOM_MASK | Freqa::DS1_RANDOM_MASK;
        val = (val & 0x0000FFFF) | (ROSC_PASSWORD << Freqa::PASSWD_OFFSET);
        rosc.freqa.write_bits(val);
    }

    /// Disable drive strength randomization
    #[inline]
    pub fn disable_randomization(&mut self) {
        let rosc = self.regs();
        let mut val = rosc.freqa.read_bits();
        val &= !(Freqa::DS0_RANDOM_MASK | Freqa::DS1_RANDOM_MASK);
        val = (val & 0x0000FFFF) | (ROSC_PASSWORD << Freqa::PASSWD_OFFSET);
        rosc.freqa.write_bits(val);
    }

    // ========================================================================
    // Divider Control
    // ========================================================================

    /// Set the output divider
    ///
    /// # Arguments
    /// * `div` - Divider value (0 = pass-through, 1-32 = divide by that amount)
    #[inline]
    pub fn set_divider(&mut self, div: Divider) {
        let rosc = self.regs();
        // DIV register uses 0xaa0 + divider value encoding
        let val = if div.0 == 0 {
            0xaa0 // Pass-through
        } else {
            0xaa00 | (div.0 as u32)
        };
        rosc.div.write_bits(val);
    }

    // ========================================================================
    // Phase Shift Control
    // ========================================================================

    /// Enable the phase-shifted output
    ///
    /// # Arguments
    /// * `shift` - Phase shift amount (0-3 clocks)
    /// * `invert` - Invert the output (ignored when div=1)
    #[inline]
    pub fn enable_phase_output(&mut self, shift: u8, invert: bool) {
        let rosc = self.regs();
        let shift = (shift & 0x3) as u32;
        let val = (0xAA << 4) // Password
            | (1 << 3)       // Enable
            | (if invert { 1 << 2 } else { 0 })
            | shift;
        rosc.phase.write_bits(val);
    }

    /// Disable the phase-shifted output
    #[inline]
    pub fn disable_phase_output(&mut self) {
        let rosc = self.regs();
        let val = 0xAA << 4; // Password, enable=0
        rosc.phase.write_bits(val);
    }

    // ========================================================================
    // Random Number Generation
    // ========================================================================

    /// Get a single random bit from the ROSC
    ///
    /// This is not suitable for cryptographic purposes.
    #[inline]
    pub fn random_bit(&self) -> bool {
        self.regs().randombit.randombit()
    }

    /// Get a random byte by collecting 8 random bits
    ///
    /// This is not suitable for cryptographic purposes.
    #[inline]
    pub fn random_byte(&self) -> u8 {
        let rosc = self.regs();
        let mut byte = 0u8;
        for _ in 0..8 {
            byte = (byte << 1) | (rosc.randombit.randombit() as u8);
        }
        byte
    }

    /// Get a random u32 by collecting 32 random bits
    ///
    /// This is not suitable for cryptographic purposes.
    #[inline]
    pub fn random_u32(&self) -> u32 {
        let mut val = 0u32;
        for _ in 0..32 {
            val = (val << 1) | (self.random_bit() as u32);
        }
        val
    }

    /// Fill a buffer with random bytes
    ///
    /// This is not suitable for cryptographic purposes.
    pub fn fill_random(&self, buf: &mut [u8]) {
        for byte in buf.iter_mut() {
            *byte = self.random_byte();
        }
    }

    /// Load a seed value into the LFSR randomizer
    #[inline]
    pub fn seed_random(&mut self, seed: u32) {
        self.regs().random.write_bits(seed);
    }

    // ========================================================================
    // Count/Delay
    // ========================================================================

    /// Start the down counter with a value
    ///
    /// The counter counts down at the ROSC frequency and stops at zero.
    /// This can be used for short software delays.
    #[inline]
    pub fn start_count(&mut self, count: u16) {
        self.regs().count.write_count(count as u32);
    }

    /// Read the current counter value
    #[inline]
    pub fn read_count(&self) -> u16 {
        self.regs().count.count() as u16
    }

    /// Wait for the counter to reach zero
    #[inline]
    pub fn wait_count(&self) {
        while self.read_count() != 0 {
            core::hint::spin_loop();
        }
    }

    /// Delay using the ROSC counter
    ///
    /// # Arguments
    /// * `count` - Number of ROSC cycles to wait
    #[inline]
    pub fn delay(&mut self, count: u16) {
        self.start_count(count);
        self.wait_count();
    }

    // ========================================================================
    // Dormant Mode
    // ========================================================================

    /// Put the ROSC into dormant (low-power) mode
    ///
    /// # Safety
    /// - PLLs must be stopped before calling this
    /// - IRQs must be properly configured for wake-up
    /// - The system clock must not be using ROSC (or must handle wake-up)
    ///
    /// This method returns after the processor awakens.
    /// After waking up, ROSC restarts in approximately 1µs.
    #[inline]
    pub unsafe fn dormant(&self) {
        self.regs().dormant.write_bits(ROSC_DORMANT_VALUE);
    }
}

// ============================================================================
// RNG trait implementations (optional, requires rand_core crate)
// Note: Add rand_core feature to Cargo.toml if you want to use this.
// ============================================================================

// Uncomment below if you add rand_core as a feature:
// #[cfg(feature = "rand_core")]
// impl rand_core::RngCore for Rosc {
//     fn next_u32(&mut self) -> u32 {
//         self.get_random_u32()
//     }
//
//     fn next_u64(&mut self) -> u64 {
//         ((self.get_random_u32() as u64) << 32) | (self.get_random_u32() as u64)
//     }
//
//     fn fill_bytes(&mut self, dest: &mut [u8]) {
//         self.fill_random(dest);
//     }
//
//     fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
//         self.fill_bytes(dest);
//         Ok(())
//     }
// }
