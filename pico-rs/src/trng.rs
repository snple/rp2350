//! TRNG (True Random Number Generator) HAL for RP2350
//!
//! This module provides access to the ARM TrustZone True Random Number Generator
//! for generating cryptographically secure random numbers.
//!
//! ## Features
//!
//! - True random number generation using hardware entropy source
//! - 192-bit entropy collection (EHR - Entropy Holding Registers)
//! - Configurable ROSC sample rate
//! - Error detection (Von Neumann, CRNGT, Autocorrelation)
//! - Interrupt support
//!
//! ## Example
//!
//! ```ignore
//! use pico_rs::trng::Trng;
//!
//! let mut trng = Trng::init();
//!
//! // Generate random u32
//! let random = trng.random_u32();
//!
//! // Generate random bytes
//! let mut buffer = [0u8; 16];
//! trng.fill_random(&mut buffer);
//! ```

use crate::reset::Resettable;
use pico_rs_pac::TRNG;

/// TRNG error types
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// Von Neumann error (32 consecutive identical bits)
    VonNeumannError,
    /// CRNGT error (two consecutive identical 16-bit values)
    CrngtError,
    /// Autocorrelation test failed
    AutocorrError,
}

/// TRNG peripheral wrapper
///
/// Provides access to the True Random Number Generator.
pub struct Trng {
    // regs: &'static pico_rs_pac::trng::RegisterBlock,
    trng: TRNG,
}

impl Trng {
    /// Create a new Trng instance without initialization
    #[inline]
    pub fn new() -> Self {
        Self {
            trng: unsafe { TRNG::steal() },
        }
    }

    /// Get reference to the register block
    #[inline]
    pub fn regs(&self) -> &pico_rs_pac::trng::RegisterBlock {
        &self.trng
    }

    // ========================================================================
    // Initialization
    // ========================================================================

    /// Initialize the TRNG peripheral
    pub fn init() -> Self {
        // Reset peripheral to ensure clean register state
        <pico_rs_pac::TRNG as Resettable>::init();

        let trng = Self::new();

        // Reset the TRNG
        trng.regs().trng_sw_reset.write_bits(1);

        // Wait for reset to complete
        while trng.regs().trng_sw_reset.read_bits() != 0 {
            core::hint::spin_loop();
        }

        // Configure ROSC sample rate (default is fine for most uses)
        // sample_cnt1 controls the number of clock cycles between samples
        trng.regs().sample_cnt1.write_bits(0xFFFF);

        // Enable the random source
        trng.regs().rnd_source_enable.set_rnd_src_en();

        // Unmask EHR valid interrupt (we poll, but it helps for checking)
        trng.regs().rng_imr.clr_ehr_valid_int_mask();

        trng
    }

    // ========================================================================
    // Random Number Generation
    // ========================================================================

    /// Wait for entropy to be ready
    fn wait_for_entropy(&self) -> Result<(), Error> {
        loop {
            // Check for errors
            if self.regs().rng_isr.vn_err() {
                self.regs().rng_icr.set_vn_err();
                return Err(Error::VonNeumannError);
            }
            if self.regs().rng_isr.crngt_err() {
                self.regs().rng_icr.set_crngt_err();
                return Err(Error::CrngtError);
            }
            if self.regs().rng_isr.autocorr_err() {
                return Err(Error::AutocorrError);
            }

            // Check if entropy is valid
            if self.regs().rng_isr.ehr_valid() {
                return Ok(());
            }

            core::hint::spin_loop();
        }
    }

    /// Check if entropy is available (non-blocking)
    #[inline]
    pub fn entropy_available(&self) -> bool {
        self.regs().rng_isr.ehr_valid()
    }

    /// Read the 192-bit entropy holding registers (6 x 32-bit words)
    ///
    /// This automatically clears the EHR valid flag.
    pub fn read_ehr(&self) -> Result<[u32; 6], Error> {
        self.wait_for_entropy()?;

        let data = [
            self.regs().ehr_data0.read_bits(),
            self.regs().ehr_data1.read_bits(),
            self.regs().ehr_data2.read_bits(),
            self.regs().ehr_data3.read_bits(),
            self.regs().ehr_data4.read_bits(),
            self.regs().ehr_data5.read_bits(),
        ];

        // Clear the EHR valid flag to start collecting new entropy
        self.regs().rng_icr.set_ehr_valid();

        Ok(data)
    }

    /// Generate a random u32
    pub fn random_u32(&self) -> Result<u32, Error> {
        let ehr = self.read_ehr()?;
        Ok(ehr[0])
    }

    /// Generate a random u64
    pub fn random_u64(&self) -> Result<u64, Error> {
        let ehr = self.read_ehr()?;
        Ok((ehr[0] as u64) | ((ehr[1] as u64) << 32))
    }

    /// Fill a buffer with random bytes
    pub fn fill_random(&self, buffer: &mut [u8]) -> Result<(), Error> {
        let mut offset = 0;

        while offset < buffer.len() {
            let ehr = self.read_ehr()?;

            for word in ehr.iter() {
                let bytes = word.to_le_bytes();
                for &byte in bytes.iter() {
                    if offset >= buffer.len() {
                        return Ok(());
                    }
                    buffer[offset] = byte;
                    offset += 1;
                }
            }
        }

        Ok(())
    }

    /// Generate a random number in range [0, max)
    ///
    /// Uses rejection sampling for uniform distribution.
    pub fn random_range(&self, max: u32) -> Result<u32, Error> {
        if max == 0 {
            return Ok(0);
        }

        let threshold = u32::MAX - (u32::MAX % max);

        loop {
            let r = self.random_u32()?;
            if r < threshold {
                return Ok(r % max);
            }
        }
    }

    // ========================================================================
    // Control
    // ========================================================================

    /// Enable the random source
    #[inline]
    pub fn enable(&self) {
        self.regs().rnd_source_enable.set_rnd_src_en();
    }

    /// Disable the random source
    #[inline]
    pub fn disable(&self) {
        self.regs().rnd_source_enable.clr_rnd_src_en();
    }

    /// Check if the TRNG is busy
    #[inline]
    pub fn is_busy(&self) -> bool {
        self.regs().trng_busy.trng_busy()
    }

    /// Reset the TRNG
    pub fn reset(&self) {
        self.regs().trng_sw_reset.write_bits(1);
        while self.regs().trng_sw_reset.read_bits() != 0 {
            core::hint::spin_loop();
        }
    }

    /// Reset the bits counter
    #[inline]
    pub fn reset_bits_counter(&self) {
        self.regs().rst_bits_counter.set_rst_bits_counter();
    }

    // ========================================================================
    // Configuration
    // ========================================================================

    /// Set the sample count (number of clock cycles between ROSC samples)
    #[inline]
    pub fn set_sample_count(&self, count: u32) {
        self.regs().sample_cnt1.write_bits(count);
    }

    /// Get the current sample count
    #[inline]
    pub fn sample_count(&self) -> u32 {
        self.regs().sample_cnt1.read_bits()
    }

    /// Configure the TRNG ROSC length
    ///
    /// Selects the inverter chain length for the ring oscillator.
    #[inline]
    pub fn set_rosc_length(&self, length: u8) {
        self.regs().trng_config.write_rnd_src_sel(length as u32);
    }

    // ========================================================================
    // Interrupts
    // ========================================================================

    /// Enable EHR valid interrupt
    #[inline]
    pub fn enable_ehr_interrupt(&self) {
        self.regs().rng_imr.clr_ehr_valid_int_mask();
    }

    /// Disable EHR valid interrupt
    #[inline]
    pub fn disable_ehr_interrupt(&self) {
        self.regs().rng_imr.set_ehr_valid_int_mask();
    }

    /// Enable autocorrelation error interrupt
    #[inline]
    pub fn enable_autocorr_interrupt(&self) {
        self.regs().rng_imr.clr_autocorr_err_int_mask();
    }

    /// Disable autocorrelation error interrupt
    #[inline]
    pub fn disable_autocorr_interrupt(&self) {
        self.regs().rng_imr.set_autocorr_err_int_mask();
    }

    /// Enable CRNGT error interrupt
    #[inline]
    pub fn enable_crngt_interrupt(&self) {
        self.regs().rng_imr.clr_crngt_err_int_mask();
    }

    /// Disable CRNGT error interrupt
    #[inline]
    pub fn disable_crngt_interrupt(&self) {
        self.regs().rng_imr.set_crngt_err_int_mask();
    }

    /// Enable Von Neumann error interrupt
    #[inline]
    pub fn enable_vn_interrupt(&self) {
        self.regs().rng_imr.clr_vn_err_int_mask();
    }

    /// Disable Von Neumann error interrupt
    #[inline]
    pub fn disable_vn_interrupt(&self) {
        self.regs().rng_imr.set_vn_err_int_mask();
    }

    // ========================================================================
    // Status
    // ========================================================================

    /// Check for Von Neumann error
    #[inline]
    pub fn vn_error(&self) -> bool {
        self.regs().rng_isr.vn_err()
    }

    /// Check for CRNGT error
    #[inline]
    pub fn crngt_error(&self) -> bool {
        self.regs().rng_isr.crngt_err()
    }

    /// Check for autocorrelation error
    #[inline]
    pub fn autocorr_error(&self) -> bool {
        self.regs().rng_isr.autocorr_err()
    }

    /// Clear Von Neumann error flag
    #[inline]
    pub fn clear_vn_error(&self) {
        self.regs().rng_icr.set_vn_err();
    }

    /// Clear CRNGT error flag
    #[inline]
    pub fn clear_crngt_error(&self) {
        self.regs().rng_icr.set_crngt_err();
    }

    /// Clear EHR valid flag
    #[inline]
    pub fn clear_ehr_valid(&self) {
        self.regs().rng_icr.set_ehr_valid();
    }

    /// Get the TRNG version
    #[inline]
    pub fn version(&self) -> u32 {
        self.regs().rng_version.read_bits()
    }

    // ========================================================================
    // Statistics
    // ========================================================================

    /// Get autocorrelation test statistics
    #[inline]
    pub fn autocorr_statistic(&self) -> u32 {
        self.regs().autocorr_statistic.read_bits()
    }
}

impl Default for Trng {
    fn default() -> Self {
        Self::new()
    }
}
