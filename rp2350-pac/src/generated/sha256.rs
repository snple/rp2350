//! SHA256
//! SHA-256 hash function implementation
//!
//! This peripheral supports atomic SET/CLR/XOR register aliases.

#![allow(dead_code)]

pub const BASE: u32 = 0x400f8000;

/// CSR Register
///
/// Control and status register
///
/// Address offset: 0x0000
/// Reset value: 0x00001206
///
/// ## Fields
///
/// - **BSWAP** `[12]` (RW)
///
///   Enable byte swapping of 32-bit values at the point they are committed to the SHA message scheduler.
///   This block's bus interface assembles byte/halfword data into message words in little-endian order, so that DMAing the same buffer with different transfer sizes always gives the same result on a little-endian system like RP2350.
///   However, when marshalling bytes into blocks, SHA expects that the first byte is the *most significant* in each message word. To resolve this, once the bus interface has accumulated 32 bits of data (either a word write, two halfword writes in little-endian order, or four byte writes in little-endian order) the final value can be byte-swapped before passing to the actual SHA core.
///   This feature is enabled by default because using the SHA core to checksum byte buffers is expected to be more common than having preformatted SHA message words lying around.
///
/// - **DMA_SIZE** `[9:8]` (RW)
///
///   Configure DREQ logic for the correct DMA data size. Must be configured before the DMA channel is triggered.
///   The SHA-256 core's DREQ logic requests one entire block of data at once, since there is no FIFO, and data goes straight into the core's message schedule and digest hardware. Therefore, when transferring data with DMA, CSR_DMA_SIZE must be configured in advance so that the correct number of transfers can be requested per block.
///
/// - **ERR_WDATA_NOT_RDY** `[4]` (RW)
///
///   Set when a write occurs whilst the SHA-256 core is not ready for data (WDATA_RDY is low). Write one to clear.
///
/// - **SUM_VLD** `[2]` (RO)
///
///   If 1, the SHA-256 checksum presented in registers SUM0 through SUM7 is currently valid.
///   Goes low when WDATA is first written, then returns high once 16 words have been written and the digest of the current 512-bit block has subsequently completed.
///
/// - **WDATA_RDY** `[1]` (RO)
///
///   If 1, the SHA-256 core is ready to accept more data through the WDATA register.
///   After writing 16 words, this flag will go low for 57 cycles whilst the core completes its digest.
///
/// - **START** `[0]` (WO)
///
///   Write 1 to prepare the SHA-256 core for a new checksum.
///   The SUMx registers are initialised to the proper values (fractional bits of square roots of first 8 primes) and internal counters are cleared. This immediately forces WDATA_RDY and SUM_VLD high.
///   START must be written before initiating a DMA transfer to the SHA-256 core, because the core will always request 16 transfers at a time (1 512-bit block). Additionally, the DMA channel should be configured for a multiple of 16 32-bit transfers.
#[repr(transparent)]
pub struct Csr {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Csr {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Csr {
    // ======== Field Constants ========

    /// **BSWAP**
    ///
    /// Enable byte swapping of 32-bit values at the point they are committed to the SHA message scheduler.
    /// This block's bus interface assembles byte/halfword data into message words in little-endian order, so that DMAing the same buffer with different transfer sizes always gives the same result on a little-endian system like RP2350.
    /// However, when marshalling bytes into blocks, SHA expects that the first byte is the *most significant* in each message word. To resolve this, once the bus interface has accumulated 32 bits of data (either a word write, two halfword writes in little-endian order, or four byte writes in little-endian order) the final value can be byte-swapped before passing to the actual SHA core.
    /// This feature is enabled by default because using the SHA core to checksum byte buffers is expected to be more common than having preformatted SHA message words lying around.
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const BSWAP_OFFSET: u32 = 12;
    pub const BSWAP_MASK: u32 = 0x00001000;
    pub const BSWAP: u32 = 1 << 12;

    /// **DMA_SIZE**
    ///
    /// Configure DREQ logic for the correct DMA data size. Must be configured before the DMA channel is triggered.
    /// The SHA-256 core's DREQ logic requests one entire block of data at once, since there is no FIFO, and data goes straight into the core's message schedule and digest hardware. Therefore, when transferring data with DMA, CSR_DMA_SIZE must be configured in advance so that the correct number of transfers can be requested per block.
    ///
    /// Bits `[9:8]`, mask `0x00000300`
    pub const DMA_SIZE_OFFSET: u32 = 8;
    pub const DMA_SIZE_MASK: u32 = 0x00000300;
    pub const DMA_SIZE_8BIT: u32 = 0 << 8;
    pub const DMA_SIZE_8BIT_VAL: u32 = 0;
    pub const DMA_SIZE_16BIT: u32 = 1 << 8;
    pub const DMA_SIZE_16BIT_VAL: u32 = 1;
    pub const DMA_SIZE_32BIT: u32 = 2 << 8;
    pub const DMA_SIZE_32BIT_VAL: u32 = 2;

    /// **ERR_WDATA_NOT_RDY**
    ///
    /// Set when a write occurs whilst the SHA-256 core is not ready for data (WDATA_RDY is low). Write one to clear.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const ERR_WDATA_NOT_RDY_OFFSET: u32 = 4;
    pub const ERR_WDATA_NOT_RDY_MASK: u32 = 0x00000010;
    pub const ERR_WDATA_NOT_RDY: u32 = 1 << 4;

    /// **SUM_VLD**
    ///
    /// If 1, the SHA-256 checksum presented in registers SUM0 through SUM7 is currently valid.
    /// Goes low when WDATA is first written, then returns high once 16 words have been written and the digest of the current 512-bit block has subsequently completed.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SUM_VLD_OFFSET: u32 = 2;
    pub const SUM_VLD_MASK: u32 = 0x00000004;
    pub const SUM_VLD: u32 = 1 << 2;

    /// **WDATA_RDY**
    ///
    /// If 1, the SHA-256 core is ready to accept more data through the WDATA register.
    /// After writing 16 words, this flag will go low for 57 cycles whilst the core completes its digest.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const WDATA_RDY_OFFSET: u32 = 1;
    pub const WDATA_RDY_MASK: u32 = 0x00000002;
    pub const WDATA_RDY: u32 = 1 << 1;

    /// **START**
    ///
    /// Write 1 to prepare the SHA-256 core for a new checksum.
    /// The SUMx registers are initialised to the proper values (fractional bits of square roots of first 8 primes) and internal counters are cleared. This immediately forces WDATA_RDY and SUM_VLD high.
    /// START must be written before initiating a DMA transfer to the SHA-256 core, because the core will always request 16 transfers at a time (1 512-bit block). Additionally, the DMA channel should be configured for a multiple of 16 32-bit transfers.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const START_OFFSET: u32 = 0;
    pub const START_MASK: u32 = 0x00000001;
    pub const START: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **BSWAP** `[12]` (RW)
    ///
    /// Enable byte swapping of 32-bit values at the point they are committed to the SHA message scheduler.
    /// This block's bus interface assembles byte/halfword data into message words in little-endian order, so that DMAing the same buffer with different transfer sizes always gives the same result on a little-endian system like RP2350.
    /// However, when marshalling bytes into blocks, SHA expects that the first byte is the *most significant* in each message word. To resolve this, once the bus interface has accumulated 32 bits of data (either a word write, two halfword writes in little-endian order, or four byte writes in little-endian order) the final value can be byte-swapped before passing to the actual SHA core.
    /// This feature is enabled by default because using the SHA core to checksum byte buffers is expected to be more common than having preformatted SHA message words lying around.
    #[inline(always)]
    pub fn bswap(&self) -> bool {
        (self.value.read_bits() & Self::BSWAP_MASK) != 0
    }

    /// Set BSWAP bit atomically (RW)
    #[inline(always)]
    pub fn set_bswap(&self) {
        self.set_bits(Self::BSWAP_MASK);
    }

    /// Clear BSWAP bit atomically (RW)
    #[inline(always)]
    pub fn clr_bswap(&self) {
        self.clr_bits(Self::BSWAP_MASK);
    }

    /// **DMA_SIZE** `[9:8]` (RW)
    ///
    /// Configure DREQ logic for the correct DMA data size. Must be configured before the DMA channel is triggered.
    /// The SHA-256 core's DREQ logic requests one entire block of data at once, since there is no FIFO, and data goes straight into the core's message schedule and digest hardware. Therefore, when transferring data with DMA, CSR_DMA_SIZE must be configured in advance so that the correct number of transfers can be requested per block.
    #[inline(always)]
    pub fn dma_size(&self) -> u32 {
        (self.value.read_bits() & Self::DMA_SIZE_MASK) >> Self::DMA_SIZE_OFFSET
    }

    /// Write **DMA_SIZE** `[9:8]` (RW)
    ///
    /// Configure DREQ logic for the correct DMA data size. Must be configured before the DMA channel is triggered.
    /// The SHA-256 core's DREQ logic requests one entire block of data at once, since there is no FIFO, and data goes straight into the core's message schedule and digest hardware. Therefore, when transferring data with DMA, CSR_DMA_SIZE must be configured in advance so that the correct number of transfers can be requested per block.
    #[inline(always)]
    pub fn write_dma_size(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DMA_SIZE_MASK) | ((value << Self::DMA_SIZE_OFFSET) & Self::DMA_SIZE_MASK)
        });
    }

    /// **ERR_WDATA_NOT_RDY** `[4]` (RW)
    ///
    /// Set when a write occurs whilst the SHA-256 core is not ready for data (WDATA_RDY is low). Write one to clear.
    #[inline(always)]
    pub fn err_wdata_not_rdy(&self) -> bool {
        (self.value.read_bits() & Self::ERR_WDATA_NOT_RDY_MASK) != 0
    }

    /// Set ERR_WDATA_NOT_RDY bit atomically (RW)
    #[inline(always)]
    pub fn set_err_wdata_not_rdy(&self) {
        self.set_bits(Self::ERR_WDATA_NOT_RDY_MASK);
    }

    /// Clear ERR_WDATA_NOT_RDY bit atomically (RW)
    #[inline(always)]
    pub fn clr_err_wdata_not_rdy(&self) {
        self.clr_bits(Self::ERR_WDATA_NOT_RDY_MASK);
    }

    /// **SUM_VLD** `[2]` (RO)
    ///
    /// If 1, the SHA-256 checksum presented in registers SUM0 through SUM7 is currently valid.
    /// Goes low when WDATA is first written, then returns high once 16 words have been written and the digest of the current 512-bit block has subsequently completed.
    #[inline(always)]
    pub fn sum_vld(&self) -> bool {
        (self.value.read_bits() & Self::SUM_VLD_MASK) != 0
    }

    /// **WDATA_RDY** `[1]` (RO)
    ///
    /// If 1, the SHA-256 core is ready to accept more data through the WDATA register.
    /// After writing 16 words, this flag will go low for 57 cycles whilst the core completes its digest.
    #[inline(always)]
    pub fn wdata_rdy(&self) -> bool {
        (self.value.read_bits() & Self::WDATA_RDY_MASK) != 0
    }

    /// Trigger **START** (WO)
    ///
    /// Write 1 to prepare the SHA-256 core for a new checksum.
    /// The SUMx registers are initialised to the proper values (fractional bits of square roots of first 8 primes) and internal counters are cleared. This immediately forces WDATA_RDY and SUM_VLD high.
    /// START must be written before initiating a DMA transfer to the SHA-256 core, because the core will always request 16 transfers at a time (1 512-bit block). Additionally, the DMA channel should be configured for a multiple of 16 32-bit transfers.
    #[inline(always)]
    pub fn trigger_start(&self) {
        self.set_bits(Self::START_MASK);
    }
}

/// CSR value type for batch read/modify/write operations
///
/// Use `csr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `csr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CsrVal {
    bits: u32,
}

impl CsrVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00001206 }
    }

    /// Read **BSWAP** `[12:12]`
    ///
    /// Enable byte swapping of 32-bit values at the point they are committed to the SHA message scheduler.
    /// This block's bus interface assembles byte/halfword data into message words in little-endian order, so that DMAing the same buffer with different transfer sizes always gives the same result on a little-endian system like RP2350.
    /// However, when marshalling bytes into blocks, SHA expects that the first byte is the *most significant* in each message word. To resolve this, once the bus interface has accumulated 32 bits of data (either a word write, two halfword writes in little-endian order, or four byte writes in little-endian order) the final value can be byte-swapped before passing to the actual SHA core.
    /// This feature is enabled by default because using the SHA core to checksum byte buffers is expected to be more common than having preformatted SHA message words lying around.
    #[inline(always)]
    pub const fn bswap(&self) -> bool {
        (self.bits & Csr::BSWAP_MASK) != 0
    }

    /// Read **DMA_SIZE** `[9:8]`
    ///
    /// Configure DREQ logic for the correct DMA data size. Must be configured before the DMA channel is triggered.
    /// The SHA-256 core's DREQ logic requests one entire block of data at once, since there is no FIFO, and data goes straight into the core's message schedule and digest hardware. Therefore, when transferring data with DMA, CSR_DMA_SIZE must be configured in advance so that the correct number of transfers can be requested per block.
    #[inline(always)]
    pub const fn dma_size(&self) -> u32 {
        (self.bits & Csr::DMA_SIZE_MASK) >> Csr::DMA_SIZE_OFFSET
    }

    /// Read **ERR_WDATA_NOT_RDY** `[4:4]`
    ///
    /// Set when a write occurs whilst the SHA-256 core is not ready for data (WDATA_RDY is low). Write one to clear.
    #[inline(always)]
    pub const fn err_wdata_not_rdy(&self) -> bool {
        (self.bits & Csr::ERR_WDATA_NOT_RDY_MASK) != 0
    }

    /// Read **SUM_VLD** `[2:2]`
    ///
    /// If 1, the SHA-256 checksum presented in registers SUM0 through SUM7 is currently valid.
    /// Goes low when WDATA is first written, then returns high once 16 words have been written and the digest of the current 512-bit block has subsequently completed.
    #[inline(always)]
    pub const fn sum_vld(&self) -> bool {
        (self.bits & Csr::SUM_VLD_MASK) != 0
    }

    /// Read **WDATA_RDY** `[1:1]`
    ///
    /// If 1, the SHA-256 core is ready to accept more data through the WDATA register.
    /// After writing 16 words, this flag will go low for 57 cycles whilst the core completes its digest.
    #[inline(always)]
    pub const fn wdata_rdy(&self) -> bool {
        (self.bits & Csr::WDATA_RDY_MASK) != 0
    }

    /// Set **BSWAP** `[12:12]`
    ///
    /// Enable byte swapping of 32-bit values at the point they are committed to the SHA message scheduler.
    /// This block's bus interface assembles byte/halfword data into message words in little-endian order, so that DMAing the same buffer with different transfer sizes always gives the same result on a little-endian system like RP2350.
    /// However, when marshalling bytes into blocks, SHA expects that the first byte is the *most significant* in each message word. To resolve this, once the bus interface has accumulated 32 bits of data (either a word write, two halfword writes in little-endian order, or four byte writes in little-endian order) the final value can be byte-swapped before passing to the actual SHA core.
    /// This feature is enabled by default because using the SHA core to checksum byte buffers is expected to be more common than having preformatted SHA message words lying around.
    #[inline(always)]
    pub const fn with_bswap(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Csr::BSWAP_MASK
            } else {
                self.bits & !Csr::BSWAP_MASK
            },
        }
    }

    /// Set **DMA_SIZE** `[9:8]`
    ///
    /// Configure DREQ logic for the correct DMA data size. Must be configured before the DMA channel is triggered.
    /// The SHA-256 core's DREQ logic requests one entire block of data at once, since there is no FIFO, and data goes straight into the core's message schedule and digest hardware. Therefore, when transferring data with DMA, CSR_DMA_SIZE must be configured in advance so that the correct number of transfers can be requested per block.
    #[inline(always)]
    pub const fn with_dma_size(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Csr::DMA_SIZE_MASK)
                | ((value << Csr::DMA_SIZE_OFFSET) & Csr::DMA_SIZE_MASK),
        }
    }

    /// Set **ERR_WDATA_NOT_RDY** `[4:4]`
    ///
    /// Set when a write occurs whilst the SHA-256 core is not ready for data (WDATA_RDY is low). Write one to clear.
    #[inline(always)]
    pub const fn with_err_wdata_not_rdy(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Csr::ERR_WDATA_NOT_RDY_MASK
            } else {
                self.bits & !Csr::ERR_WDATA_NOT_RDY_MASK
            },
        }
    }

    /// Set **START** `[0:0]`
    ///
    /// Write 1 to prepare the SHA-256 core for a new checksum.
    /// The SUMx registers are initialised to the proper values (fractional bits of square roots of first 8 primes) and internal counters are cleared. This immediately forces WDATA_RDY and SUM_VLD high.
    /// START must be written before initiating a DMA transfer to the SHA-256 core, because the core will always request 16 transfers at a time (1 512-bit block). Additionally, the DMA channel should be configured for a multiple of 16 32-bit transfers.
    #[inline(always)]
    pub const fn with_start(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Csr::START_MASK
            } else {
                self.bits & !Csr::START_MASK
            },
        }
    }
}

impl Default for CsrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Csr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CsrVal {
        CsrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CsrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CsrVal) -> CsrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// WDATA Register
///
/// Write data register
///
/// Address offset: 0x0004
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **WDATA** `[31:0]` (WO)
///
///   After pulsing START and writing 16 words of data to this register, WDATA_RDY will go low and the SHA-256 core will complete the digest of the current 512-bit block.
///   Software is responsible for ensuring the data is correctly padded and terminated to a whole number of 512-bit blocks.
///   After this, WDATA_RDY will return high, and more data can be written (if any).
///   This register supports word, halfword and byte writes, so that DMA from non-word-aligned buffers can be supported. The total amount of data per block remains the same (16 words, 32 halfwords or 64 bytes) and byte/halfword transfers must not be mixed within a block.
#[repr(transparent)]
pub struct Wdata {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Wdata {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Wdata {
    // ======== Field Constants ========

    /// **WDATA**
    ///
    /// After pulsing START and writing 16 words of data to this register, WDATA_RDY will go low and the SHA-256 core will complete the digest of the current 512-bit block.
    /// Software is responsible for ensuring the data is correctly padded and terminated to a whole number of 512-bit blocks.
    /// After this, WDATA_RDY will return high, and more data can be written (if any).
    /// This register supports word, halfword and byte writes, so that DMA from non-word-aligned buffers can be supported. The total amount of data per block remains the same (16 words, 32 halfwords or 64 bytes) and byte/halfword transfers must not be mixed within a block.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const WDATA_OFFSET: u32 = 0;
    pub const WDATA_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// Write **WDATA** `[31:0]` (WO)
    ///
    /// After pulsing START and writing 16 words of data to this register, WDATA_RDY will go low and the SHA-256 core will complete the digest of the current 512-bit block.
    /// Software is responsible for ensuring the data is correctly padded and terminated to a whole number of 512-bit blocks.
    /// After this, WDATA_RDY will return high, and more data can be written (if any).
    /// This register supports word, halfword and byte writes, so that DMA from non-word-aligned buffers can be supported. The total amount of data per block remains the same (16 words, 32 halfwords or 64 bytes) and byte/halfword transfers must not be mixed within a block.
    #[inline(always)]
    pub fn write_wdata(&self, value: u32) {
        self.value
            .write_bits((value << Self::WDATA_OFFSET) & Self::WDATA_MASK);
    }
}

/// WDATA value type for batch read/modify/write operations
///
/// Use `wdata.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `wdata.write_val(val)`.
#[derive(Clone, Copy)]
pub struct WdataVal {
    bits: u32,
}

impl WdataVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Set **WDATA** `[31:0]`
    ///
    /// After pulsing START and writing 16 words of data to this register, WDATA_RDY will go low and the SHA-256 core will complete the digest of the current 512-bit block.
    /// Software is responsible for ensuring the data is correctly padded and terminated to a whole number of 512-bit blocks.
    /// After this, WDATA_RDY will return high, and more data can be written (if any).
    /// This register supports word, halfword and byte writes, so that DMA from non-word-aligned buffers can be supported. The total amount of data per block remains the same (16 words, 32 halfwords or 64 bytes) and byte/halfword transfers must not be mixed within a block.
    #[inline(always)]
    pub const fn with_wdata(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Wdata::WDATA_MASK)
                | ((value << Wdata::WDATA_OFFSET) & Wdata::WDATA_MASK),
        }
    }
}

impl Default for WdataVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Wdata {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: WdataVal) {
        self.value.write_bits(val.bits())
    }
}

/// SUM0 Register
///
/// 256-bit checksum result. Contents are undefined when CSR_SUM_VLD is 0.
///
/// Address offset: 0x0008
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SUM0** `[31:0]` (RO)
#[repr(transparent)]
pub struct Sum0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Sum0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sum0 {
    // ======== Field Constants ========

    /// **SUM0**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SUM0_OFFSET: u32 = 0;
    pub const SUM0_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SUM0** `[31:0]` (RO)
    #[inline(always)]
    pub fn sum0(&self) -> u32 {
        (self.value.read_bits() & Self::SUM0_MASK) >> Self::SUM0_OFFSET
    }
}

/// SUM0 value type for batch read/modify/write operations
///
/// Use `sum0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sum0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Sum0Val {
    bits: u32,
}

impl Sum0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **SUM0** `[31:0]`
    #[inline(always)]
    pub const fn sum0(&self) -> u32 {
        (self.bits & Sum0::SUM0_MASK) >> Sum0::SUM0_OFFSET
    }
}

impl Default for Sum0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Sum0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Sum0Val {
        Sum0Val::from_bits(self.value.read_bits())
    }
}

/// SUM1 Register
///
/// 256-bit checksum result. Contents are undefined when CSR_SUM_VLD is 0.
///
/// Address offset: 0x000c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SUM1** `[31:0]` (RO)
#[repr(transparent)]
pub struct Sum1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Sum1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sum1 {
    // ======== Field Constants ========

    /// **SUM1**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SUM1_OFFSET: u32 = 0;
    pub const SUM1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SUM1** `[31:0]` (RO)
    #[inline(always)]
    pub fn sum1(&self) -> u32 {
        (self.value.read_bits() & Self::SUM1_MASK) >> Self::SUM1_OFFSET
    }
}

/// SUM1 value type for batch read/modify/write operations
///
/// Use `sum1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sum1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Sum1Val {
    bits: u32,
}

impl Sum1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **SUM1** `[31:0]`
    #[inline(always)]
    pub const fn sum1(&self) -> u32 {
        (self.bits & Sum1::SUM1_MASK) >> Sum1::SUM1_OFFSET
    }
}

impl Default for Sum1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Sum1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Sum1Val {
        Sum1Val::from_bits(self.value.read_bits())
    }
}

/// SUM2 Register
///
/// 256-bit checksum result. Contents are undefined when CSR_SUM_VLD is 0.
///
/// Address offset: 0x0010
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SUM2** `[31:0]` (RO)
#[repr(transparent)]
pub struct Sum2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Sum2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sum2 {
    // ======== Field Constants ========

    /// **SUM2**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SUM2_OFFSET: u32 = 0;
    pub const SUM2_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SUM2** `[31:0]` (RO)
    #[inline(always)]
    pub fn sum2(&self) -> u32 {
        (self.value.read_bits() & Self::SUM2_MASK) >> Self::SUM2_OFFSET
    }
}

/// SUM2 value type for batch read/modify/write operations
///
/// Use `sum2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sum2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Sum2Val {
    bits: u32,
}

impl Sum2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **SUM2** `[31:0]`
    #[inline(always)]
    pub const fn sum2(&self) -> u32 {
        (self.bits & Sum2::SUM2_MASK) >> Sum2::SUM2_OFFSET
    }
}

impl Default for Sum2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Sum2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Sum2Val {
        Sum2Val::from_bits(self.value.read_bits())
    }
}

/// SUM3 Register
///
/// 256-bit checksum result. Contents are undefined when CSR_SUM_VLD is 0.
///
/// Address offset: 0x0014
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SUM3** `[31:0]` (RO)
#[repr(transparent)]
pub struct Sum3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Sum3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sum3 {
    // ======== Field Constants ========

    /// **SUM3**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SUM3_OFFSET: u32 = 0;
    pub const SUM3_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SUM3** `[31:0]` (RO)
    #[inline(always)]
    pub fn sum3(&self) -> u32 {
        (self.value.read_bits() & Self::SUM3_MASK) >> Self::SUM3_OFFSET
    }
}

/// SUM3 value type for batch read/modify/write operations
///
/// Use `sum3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sum3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Sum3Val {
    bits: u32,
}

impl Sum3Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **SUM3** `[31:0]`
    #[inline(always)]
    pub const fn sum3(&self) -> u32 {
        (self.bits & Sum3::SUM3_MASK) >> Sum3::SUM3_OFFSET
    }
}

impl Default for Sum3Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Sum3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Sum3Val {
        Sum3Val::from_bits(self.value.read_bits())
    }
}

/// SUM4 Register
///
/// 256-bit checksum result. Contents are undefined when CSR_SUM_VLD is 0.
///
/// Address offset: 0x0018
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SUM4** `[31:0]` (RO)
#[repr(transparent)]
pub struct Sum4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Sum4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sum4 {
    // ======== Field Constants ========

    /// **SUM4**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SUM4_OFFSET: u32 = 0;
    pub const SUM4_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SUM4** `[31:0]` (RO)
    #[inline(always)]
    pub fn sum4(&self) -> u32 {
        (self.value.read_bits() & Self::SUM4_MASK) >> Self::SUM4_OFFSET
    }
}

/// SUM4 value type for batch read/modify/write operations
///
/// Use `sum4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sum4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Sum4Val {
    bits: u32,
}

impl Sum4Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **SUM4** `[31:0]`
    #[inline(always)]
    pub const fn sum4(&self) -> u32 {
        (self.bits & Sum4::SUM4_MASK) >> Sum4::SUM4_OFFSET
    }
}

impl Default for Sum4Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Sum4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Sum4Val {
        Sum4Val::from_bits(self.value.read_bits())
    }
}

/// SUM5 Register
///
/// 256-bit checksum result. Contents are undefined when CSR_SUM_VLD is 0.
///
/// Address offset: 0x001c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SUM5** `[31:0]` (RO)
#[repr(transparent)]
pub struct Sum5 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Sum5 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sum5 {
    // ======== Field Constants ========

    /// **SUM5**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SUM5_OFFSET: u32 = 0;
    pub const SUM5_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SUM5** `[31:0]` (RO)
    #[inline(always)]
    pub fn sum5(&self) -> u32 {
        (self.value.read_bits() & Self::SUM5_MASK) >> Self::SUM5_OFFSET
    }
}

/// SUM5 value type for batch read/modify/write operations
///
/// Use `sum5.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sum5.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Sum5Val {
    bits: u32,
}

impl Sum5Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **SUM5** `[31:0]`
    #[inline(always)]
    pub const fn sum5(&self) -> u32 {
        (self.bits & Sum5::SUM5_MASK) >> Sum5::SUM5_OFFSET
    }
}

impl Default for Sum5Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Sum5 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Sum5Val {
        Sum5Val::from_bits(self.value.read_bits())
    }
}

/// SUM6 Register
///
/// 256-bit checksum result. Contents are undefined when CSR_SUM_VLD is 0.
///
/// Address offset: 0x0020
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SUM6** `[31:0]` (RO)
#[repr(transparent)]
pub struct Sum6 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Sum6 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sum6 {
    // ======== Field Constants ========

    /// **SUM6**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SUM6_OFFSET: u32 = 0;
    pub const SUM6_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SUM6** `[31:0]` (RO)
    #[inline(always)]
    pub fn sum6(&self) -> u32 {
        (self.value.read_bits() & Self::SUM6_MASK) >> Self::SUM6_OFFSET
    }
}

/// SUM6 value type for batch read/modify/write operations
///
/// Use `sum6.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sum6.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Sum6Val {
    bits: u32,
}

impl Sum6Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **SUM6** `[31:0]`
    #[inline(always)]
    pub const fn sum6(&self) -> u32 {
        (self.bits & Sum6::SUM6_MASK) >> Sum6::SUM6_OFFSET
    }
}

impl Default for Sum6Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Sum6 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Sum6Val {
        Sum6Val::from_bits(self.value.read_bits())
    }
}

/// SUM7 Register
///
/// 256-bit checksum result. Contents are undefined when CSR_SUM_VLD is 0.
///
/// Address offset: 0x0024
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SUM7** `[31:0]` (RO)
#[repr(transparent)]
pub struct Sum7 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Sum7 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sum7 {
    // ======== Field Constants ========

    /// **SUM7**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SUM7_OFFSET: u32 = 0;
    pub const SUM7_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SUM7** `[31:0]` (RO)
    #[inline(always)]
    pub fn sum7(&self) -> u32 {
        (self.value.read_bits() & Self::SUM7_MASK) >> Self::SUM7_OFFSET
    }
}

/// SUM7 value type for batch read/modify/write operations
///
/// Use `sum7.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sum7.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Sum7Val {
    bits: u32,
}

impl Sum7Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **SUM7** `[31:0]`
    #[inline(always)]
    pub const fn sum7(&self) -> u32 {
        (self.bits & Sum7::SUM7_MASK) >> Sum7::SUM7_OFFSET
    }
}

impl Default for Sum7Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Sum7 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Sum7Val {
        Sum7Val::from_bits(self.value.read_bits())
    }
}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// Control and status register
    pub csr: Csr,
    /// 0x0004
    /// Write data register
    pub wdata: Wdata,
    /// 0x0008
    /// 256-bit checksum result. Contents are undefined when CSR_SUM_VLD is 0.
    pub sum0: Sum0,
    /// 0x000c
    /// 256-bit checksum result. Contents are undefined when CSR_SUM_VLD is 0.
    pub sum1: Sum1,
    /// 0x0010
    /// 256-bit checksum result. Contents are undefined when CSR_SUM_VLD is 0.
    pub sum2: Sum2,
    /// 0x0014
    /// 256-bit checksum result. Contents are undefined when CSR_SUM_VLD is 0.
    pub sum3: Sum3,
    /// 0x0018
    /// 256-bit checksum result. Contents are undefined when CSR_SUM_VLD is 0.
    pub sum4: Sum4,
    /// 0x001c
    /// 256-bit checksum result. Contents are undefined when CSR_SUM_VLD is 0.
    pub sum5: Sum5,
    /// 0x0020
    /// 256-bit checksum result. Contents are undefined when CSR_SUM_VLD is 0.
    pub sum6: Sum6,
    /// 0x0024
    /// 256-bit checksum result. Contents are undefined when CSR_SUM_VLD is 0.
    pub sum7: Sum7,
}
