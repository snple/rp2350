//! SHA-256 hardware accelerator driver for RP2350
//!
//! The RP2350 includes a hardware SHA-256 accelerator that can compute
//! SHA-256 hashes much faster than a software implementation.
//!
//! ## Example
//!
//! ```ignore
//! use pico_rs::sha256::Sha256;
//!
//! let mut sha = Sha256::new();
//!
//! // Start a new hash computation
//! sha.start();
//!
//! // Write data (must be 64-byte blocks until final)
//! let data = b"Hello, World!";
//! let hash = sha.hash(data);
//!
//! // Print the hash
//! for byte in hash.iter() {
//!     print!("{:02x}", byte);
//! }
//! ```

use crate::reset;
use pico_rs_pac::SHA256;

/// SHA-256 block size in bytes
pub const BLOCK_SIZE: usize = 64;

/// SHA-256 digest size in bytes
pub const DIGEST_SIZE: usize = 32;

/// DMA transfer size configuration
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaSize {
    /// 8-bit DMA transfers
    Bit8 = 0,
    /// 16-bit DMA transfers
    Bit16 = 1,
    /// 32-bit DMA transfers
    Bit32 = 2,
}

/// SHA-256 hardware accelerator driver
pub struct Sha256 {
    _private: (),
}

impl Sha256 {
    /// Create a new SHA-256 driver instance
    ///
    /// This will bring the SHA-256 peripheral out of reset.
    #[inline]
    pub fn new() -> Self {
        // Unreset the SHA256 peripheral
        reset::unreset_sha256_wait();

        Self { _private: () }
    }

    /// Get a reference to the SHA256 registers
    #[inline]
    fn regs(&self) -> &pico_rs_pac::sha256::RegisterBlock {
        unsafe { &*SHA256::PTR }
    }

    // ========================================================================
    // Control and Status
    // ========================================================================

    /// Start a new SHA-256 hash computation
    ///
    /// This prepares the core for a new checksum. The SUMx registers are
    /// initialised to the proper values for the start of a SHA-256 hash.
    #[inline]
    pub fn start(&mut self) {
        self.regs().csr.trigger_start();
    }

    /// Check if the core is ready to accept more data
    #[inline]
    pub fn is_wdata_ready(&self) -> bool {
        self.regs().csr.wdata_rdy()
    }

    /// Check if the checksum in SUMx registers is valid
    #[inline]
    pub fn is_sum_valid(&self) -> bool {
        self.regs().csr.sum_vld()
    }

    /// Check if a write error occurred (write when not ready)
    #[inline]
    pub fn has_write_error(&self) -> bool {
        self.regs().csr.err_wdata_not_rdy()
    }

    /// Clear the write error flag
    #[inline]
    pub fn clear_write_error(&mut self) {
        self.regs().csr.clr_err_wdata_not_rdy();
    }

    /// Enable byte swapping of 32-bit values
    ///
    /// When enabled, bytes are swapped at the point they are committed
    /// to the SHA-256 message schedule.
    #[inline]
    pub fn enable_byte_swap(&mut self) {
        self.regs().csr.set_bswap();
    }

    /// Disable byte swapping of 32-bit values
    #[inline]
    pub fn disable_byte_swap(&mut self) {
        self.regs().csr.clr_bswap();
    }

    /// Check if byte swapping is enabled
    #[inline]
    pub fn is_byte_swap_enabled(&self) -> bool {
        self.regs().csr.bswap()
    }

    /// Configure DMA transfer size
    #[inline]
    pub fn set_dma_size(&mut self, size: DmaSize) {
        self.regs().csr.write_dma_size(size as u32);
    }

    /// Get the current DMA transfer size configuration
    #[inline]
    pub fn get_dma_size(&self) -> DmaSize {
        match self.regs().csr.dma_size() {
            0 => DmaSize::Bit8,
            1 => DmaSize::Bit16,
            _ => DmaSize::Bit32,
        }
    }

    // ========================================================================
    // Data Writing
    // ========================================================================

    /// Wait until the core is ready to accept data
    #[inline]
    pub fn wait_ready(&self) {
        while !self.is_wdata_ready() {
            core::hint::spin_loop();
        }
    }

    /// Wait until the checksum is valid
    #[inline]
    pub fn wait_sum_valid(&self) {
        while !self.is_sum_valid() {
            core::hint::spin_loop();
        }
    }

    /// Write a single 32-bit word of data
    ///
    /// You must write exactly 16 words (64 bytes) per block.
    /// After each block, wait for `is_wdata_ready()` to become true.
    #[inline]
    pub fn write_word(&mut self, word: u32) {
        self.regs().wdata.write_wdata(word);
    }

    /// Write a single 32-bit word of data, waiting if necessary
    #[inline]
    pub fn write_word_blocking(&mut self, word: u32) {
        self.wait_ready();
        self.write_word(word);
    }

    /// Write a block of 64 bytes (16 words)
    ///
    /// # Panics
    /// Panics if `block.len()` is not exactly 64.
    pub fn write_block(&mut self, block: &[u8]) {
        assert_eq!(
            block.len(),
            BLOCK_SIZE,
            "SHA-256 block must be exactly 64 bytes"
        );

        for chunk in block.chunks_exact(4) {
            let word = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
            self.write_word_blocking(word);
        }
    }

    /// Write a block of 16 32-bit words
    ///
    /// # Panics
    /// Panics if `words.len()` is not exactly 16.
    pub fn write_block_words(&mut self, words: &[u32]) {
        assert_eq!(words.len(), 16, "SHA-256 block must be exactly 16 words");

        for &word in words {
            self.write_word_blocking(word);
        }
    }

    // ========================================================================
    // Reading the Hash
    // ========================================================================

    /// Read the hash result as an array of 8 32-bit words
    ///
    /// Note: The checksum is only valid when `is_sum_valid()` returns true.
    #[inline]
    pub fn read_hash_words(&self) -> [u32; 8] {
        let regs = self.regs();
        [
            regs.sum0.read_bits(),
            regs.sum1.read_bits(),
            regs.sum2.read_bits(),
            regs.sum3.read_bits(),
            regs.sum4.read_bits(),
            regs.sum5.read_bits(),
            regs.sum6.read_bits(),
            regs.sum7.read_bits(),
        ]
    }

    /// Read the hash result as a 32-byte array
    ///
    /// Note: The checksum is only valid when `is_sum_valid()` returns true.
    #[inline]
    pub fn read_hash(&self) -> [u8; DIGEST_SIZE] {
        let words = self.read_hash_words();
        let mut hash = [0u8; DIGEST_SIZE];

        for (i, word) in words.iter().enumerate() {
            let bytes = word.to_be_bytes();
            hash[i * 4..(i + 1) * 4].copy_from_slice(&bytes);
        }

        hash
    }

    /// Read the hash result, waiting for it to be valid first
    #[inline]
    pub fn read_hash_blocking(&self) -> [u8; DIGEST_SIZE] {
        self.wait_sum_valid();
        self.read_hash()
    }

    // ========================================================================
    // High-Level API
    // ========================================================================

    /// Compute SHA-256 hash of the given data
    ///
    /// This handles all padding automatically according to the SHA-256 spec:
    /// - Append a '1' bit
    /// - Append '0' bits until length ≡ 448 (mod 512)
    /// - Append original message length as 64-bit big-endian
    pub fn hash(&mut self, data: &[u8]) -> [u8; DIGEST_SIZE] {
        self.start();

        let bit_len = (data.len() as u64) * 8;

        // Process complete 64-byte blocks
        let mut offset = 0;
        while offset + BLOCK_SIZE <= data.len() {
            self.write_block(&data[offset..offset + BLOCK_SIZE]);
            offset += BLOCK_SIZE;
        }

        // Handle final block(s) with padding
        let remaining = data.len() - offset;
        let mut final_block = [0u8; BLOCK_SIZE * 2]; // May need two blocks

        // Copy remaining data
        final_block[..remaining].copy_from_slice(&data[offset..]);

        // Append '1' bit (0x80)
        final_block[remaining] = 0x80;

        // Calculate how many final blocks we need
        let final_blocks = if remaining + 9 > BLOCK_SIZE { 2 } else { 1 };

        // Append length at the end of the last block
        let len_offset = final_blocks * BLOCK_SIZE - 8;
        final_block[len_offset..len_offset + 8].copy_from_slice(&bit_len.to_be_bytes());

        // Write final block(s)
        for i in 0..final_blocks {
            self.write_block(&final_block[i * BLOCK_SIZE..(i + 1) * BLOCK_SIZE]);
        }

        self.read_hash_blocking()
    }

    /// Compute SHA-256 hash of the given data (without allocating)
    ///
    /// Same as `hash()` but takes ownership of self and returns it.
    #[inline]
    pub fn hash_into(mut self, data: &[u8]) -> ([u8; DIGEST_SIZE], Self) {
        let hash = self.hash(data);
        (hash, self)
    }
}

impl Default for Sha256 {
    fn default() -> Self {
        Self::new()
    }
}

/// Incremental SHA-256 hasher
///
/// This allows computing a hash incrementally by calling `update()` multiple times.
pub struct Sha256Hasher {
    sha: Sha256,
    buffer: [u8; BLOCK_SIZE],
    buffer_len: usize,
    total_len: u64,
}

impl Sha256Hasher {
    /// Create a new incremental hasher
    pub fn new() -> Self {
        let mut sha = Sha256::new();
        sha.start();

        Self {
            sha,
            buffer: [0u8; BLOCK_SIZE],
            buffer_len: 0,
            total_len: 0,
        }
    }

    /// Update the hash with more data
    pub fn update(&mut self, data: &[u8]) {
        self.total_len += data.len() as u64;
        let mut offset = 0;

        // If we have buffered data, try to complete a block
        if self.buffer_len > 0 {
            let needed = BLOCK_SIZE - self.buffer_len;
            if data.len() >= needed {
                self.buffer[self.buffer_len..].copy_from_slice(&data[..needed]);
                self.sha.write_block(&self.buffer);
                self.buffer_len = 0;
                offset = needed;
            } else {
                self.buffer[self.buffer_len..self.buffer_len + data.len()].copy_from_slice(data);
                self.buffer_len += data.len();
                return;
            }
        }

        // Process complete blocks
        while offset + BLOCK_SIZE <= data.len() {
            self.sha.write_block(&data[offset..offset + BLOCK_SIZE]);
            offset += BLOCK_SIZE;
        }

        // Buffer remaining data
        if offset < data.len() {
            let remaining = data.len() - offset;
            self.buffer[..remaining].copy_from_slice(&data[offset..]);
            self.buffer_len = remaining;
        }
    }

    /// Finalize the hash and return the result
    pub fn finalize(mut self) -> [u8; DIGEST_SIZE] {
        let bit_len = self.total_len * 8;

        // Create final padding
        let mut final_block = [0u8; BLOCK_SIZE * 2];
        final_block[..self.buffer_len].copy_from_slice(&self.buffer[..self.buffer_len]);
        final_block[self.buffer_len] = 0x80;

        let final_blocks = if self.buffer_len + 9 > BLOCK_SIZE {
            2
        } else {
            1
        };
        let len_offset = final_blocks * BLOCK_SIZE - 8;
        final_block[len_offset..len_offset + 8].copy_from_slice(&bit_len.to_be_bytes());

        for i in 0..final_blocks {
            self.sha
                .write_block(&final_block[i * BLOCK_SIZE..(i + 1) * BLOCK_SIZE]);
        }

        self.sha.read_hash_blocking()
    }
}

impl Default for Sha256Hasher {
    fn default() -> Self {
        Self::new()
    }
}
