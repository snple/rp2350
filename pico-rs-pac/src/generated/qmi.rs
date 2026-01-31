//! QMI
//! QSPI Memory Interface.
//! Provides a memory-mapped interface to up to two SPI/DSPI/QSPI flash or PSRAM devices. Also provides a serial interface for programming and configuration of the external device.

#![allow(dead_code)]

pub const BASE: u32 = 0x400d0000;

/// DIRECT_CSR Register
///
/// Control and status for direct serial mode
/// Direct serial mode allows the processor to send and receive raw serial frames, for programming, configuration and control of the external memory devices. Only SPI mode 0 (CPOL=0 CPHA=0) is supported.
///
/// Address offset: 0x0000
/// Reset value: 0x01800000
///
/// ## Fields
///
/// - **RXDELAY** `[31:30]` (RW)
///   Delay the read data sample timing, in units of one half of a system clock cycle. (Not necessarily half of an SCK cycle.)
/// - **CLKDIV** `[29:22]` (RW)
///   Clock divisor for direct serial mode. Divisors of 1..255 are encoded directly, and the maximum divisor of 256 is encoded by a value of CLKDIV=0.
///   The clock divisor can be changed on-the-fly by software, without halting or otherwise coordinating with the serial interface. The serial interface will sample the latest clock divisor each time it begins the transmission of a new byte.
/// - **RXLEVEL** `[20:18]` (RO)
///   Current level of DIRECT_RX FIFO
/// - **RXFULL** `[17]` (RO)
///   When 1, the DIRECT_RX FIFO is currently full. The serial interface will be stalled until data is popped; the interface will not begin a new serial frame when the DIRECT_TX FIFO is empty or the DIRECT_RX FIFO is full.
/// - **RXEMPTY** `[16]` (RO)
///   When 1, the DIRECT_RX FIFO is currently empty. If the processor attempts to read more data, the FIFO state is not affected, but the value returned to the processor is undefined.
/// - **TXLEVEL** `[14:12]` (RO)
///   Current level of DIRECT_TX FIFO
/// - **TXEMPTY** `[11]` (RO)
///   When 1, the DIRECT_TX FIFO is currently empty. Unless the processor pushes more data, transmission will stop and BUSY will go low once the current 8-bit serial frame completes.
/// - **TXFULL** `[10]` (RO)
///   When 1, the DIRECT_TX FIFO is currently full. If the processor tries to write more data, that data will be ignored.
/// - **AUTO_CS1N** `[7]` (RW)
///   When 1, automatically assert the CS1n chip select line whenever the BUSY flag is set.
/// - **AUTO_CS0N** `[6]` (RW)
///   When 1, automatically assert the CS0n chip select line whenever the BUSY flag is set.
/// - **ASSERT_CS1N** `[3]` (RW)
///   When 1, assert (i.e. drive low) the CS1n chip select line.
///   Note that this applies even when DIRECT_CSR_EN is 0.
/// - **ASSERT_CS0N** `[2]` (RW)
///   When 1, assert (i.e. drive low) the CS0n chip select line.
///   Note that this applies even when DIRECT_CSR_EN is 0.
/// - **BUSY** `[1]` (RO)
///   Direct mode busy flag. If 1, data is currently being shifted in/out (or would be if the interface were not stalled on the RX FIFO), and the chip select must not yet be deasserted.
///   The busy flag will also be set to 1 if a memory-mapped transfer is still in progress when direct mode is enabled. Direct mode blocks new memory-mapped transfers, but can't halt a transfer that is already in progress. If there is a chance that memory-mapped transfers may be in progress, the busy flag should be polled for 0 before asserting the chip select.
///   (In practice you will usually discover this timing condition through other means, because any subsequent memory-mapped transfers when direct mode is enabled will return bus errors, which are difficult to ignore.)
/// - **EN** `[0]` (RW)
///   Enable direct mode.
///   In direct mode, software controls the chip select lines, and can perform direct SPI transfers by pushing data to the DIRECT_TX FIFO, and popping the same amount of data from the DIRECT_RX FIFO.
///   Memory-mapped accesses will generate bus errors when direct serial mode is enabled.
#[repr(transparent)]
pub struct DirectCsr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DirectCsr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DirectCsr {
    // ======== Field Constants ========

    /// **RXDELAY**
    /// Delay the read data sample timing, in units of one half of a system clock cycle. (Not necessarily half of an SCK cycle.)
    /// Bits `[31:30]`, mask `0xc0000000`
    pub const RXDELAY_OFFSET: u32 = 30;
    pub const RXDELAY_MASK: u32 = 0xc0000000;

    /// **CLKDIV**
    /// Clock divisor for direct serial mode. Divisors of 1..255 are encoded directly, and the maximum divisor of 256 is encoded by a value of CLKDIV=0.
    /// The clock divisor can be changed on-the-fly by software, without halting or otherwise coordinating with the serial interface. The serial interface will sample the latest clock divisor each time it begins the transmission of a new byte.
    /// Bits `[29:22]`, mask `0x3fc00000`
    pub const CLKDIV_OFFSET: u32 = 22;
    pub const CLKDIV_MASK: u32 = 0x3fc00000;

    /// **RXLEVEL**
    /// Current level of DIRECT_RX FIFO
    /// Bits `[20:18]`, mask `0x001c0000`
    pub const RXLEVEL_OFFSET: u32 = 18;
    pub const RXLEVEL_MASK: u32 = 0x001c0000;

    /// **RXFULL**
    /// When 1, the DIRECT_RX FIFO is currently full. The serial interface will be stalled until data is popped; the interface will not begin a new serial frame when the DIRECT_TX FIFO is empty or the DIRECT_RX FIFO is full.
    /// Bits `[17:17]`, mask `0x00020000`
    pub const RXFULL_OFFSET: u32 = 17;
    pub const RXFULL_MASK: u32 = 0x00020000;
    pub const RXFULL: u32 = 1 << 17;

    /// **RXEMPTY**
    /// When 1, the DIRECT_RX FIFO is currently empty. If the processor attempts to read more data, the FIFO state is not affected, but the value returned to the processor is undefined.
    /// Bits `[16:16]`, mask `0x00010000`
    pub const RXEMPTY_OFFSET: u32 = 16;
    pub const RXEMPTY_MASK: u32 = 0x00010000;
    pub const RXEMPTY: u32 = 1 << 16;

    /// **TXLEVEL**
    /// Current level of DIRECT_TX FIFO
    /// Bits `[14:12]`, mask `0x00007000`
    pub const TXLEVEL_OFFSET: u32 = 12;
    pub const TXLEVEL_MASK: u32 = 0x00007000;

    /// **TXEMPTY**
    /// When 1, the DIRECT_TX FIFO is currently empty. Unless the processor pushes more data, transmission will stop and BUSY will go low once the current 8-bit serial frame completes.
    /// Bits `[11:11]`, mask `0x00000800`
    pub const TXEMPTY_OFFSET: u32 = 11;
    pub const TXEMPTY_MASK: u32 = 0x00000800;
    pub const TXEMPTY: u32 = 1 << 11;

    /// **TXFULL**
    /// When 1, the DIRECT_TX FIFO is currently full. If the processor tries to write more data, that data will be ignored.
    /// Bits `[10:10]`, mask `0x00000400`
    pub const TXFULL_OFFSET: u32 = 10;
    pub const TXFULL_MASK: u32 = 0x00000400;
    pub const TXFULL: u32 = 1 << 10;

    /// **AUTO_CS1N**
    /// When 1, automatically assert the CS1n chip select line whenever the BUSY flag is set.
    /// Bits `[7:7]`, mask `0x00000080`
    pub const AUTO_CS1N_OFFSET: u32 = 7;
    pub const AUTO_CS1N_MASK: u32 = 0x00000080;
    pub const AUTO_CS1N: u32 = 1 << 7;

    /// **AUTO_CS0N**
    /// When 1, automatically assert the CS0n chip select line whenever the BUSY flag is set.
    /// Bits `[6:6]`, mask `0x00000040`
    pub const AUTO_CS0N_OFFSET: u32 = 6;
    pub const AUTO_CS0N_MASK: u32 = 0x00000040;
    pub const AUTO_CS0N: u32 = 1 << 6;

    /// **ASSERT_CS1N**
    /// When 1, assert (i.e. drive low) the CS1n chip select line.
    /// Note that this applies even when DIRECT_CSR_EN is 0.
    /// Bits `[3:3]`, mask `0x00000008`
    pub const ASSERT_CS1N_OFFSET: u32 = 3;
    pub const ASSERT_CS1N_MASK: u32 = 0x00000008;
    pub const ASSERT_CS1N: u32 = 1 << 3;

    /// **ASSERT_CS0N**
    /// When 1, assert (i.e. drive low) the CS0n chip select line.
    /// Note that this applies even when DIRECT_CSR_EN is 0.
    /// Bits `[2:2]`, mask `0x00000004`
    pub const ASSERT_CS0N_OFFSET: u32 = 2;
    pub const ASSERT_CS0N_MASK: u32 = 0x00000004;
    pub const ASSERT_CS0N: u32 = 1 << 2;

    /// **BUSY**
    /// Direct mode busy flag. If 1, data is currently being shifted in/out (or would be if the interface were not stalled on the RX FIFO), and the chip select must not yet be deasserted.
    /// The busy flag will also be set to 1 if a memory-mapped transfer is still in progress when direct mode is enabled. Direct mode blocks new memory-mapped transfers, but can't halt a transfer that is already in progress. If there is a chance that memory-mapped transfers may be in progress, the busy flag should be polled for 0 before asserting the chip select.
    /// (In practice you will usually discover this timing condition through other means, because any subsequent memory-mapped transfers when direct mode is enabled will return bus errors, which are difficult to ignore.)
    /// Bits `[1:1]`, mask `0x00000002`
    pub const BUSY_OFFSET: u32 = 1;
    pub const BUSY_MASK: u32 = 0x00000002;
    pub const BUSY: u32 = 1 << 1;

    /// **EN**
    /// Enable direct mode.
    /// In direct mode, software controls the chip select lines, and can perform direct SPI transfers by pushing data to the DIRECT_TX FIFO, and popping the same amount of data from the DIRECT_RX FIFO.
    /// Memory-mapped accesses will generate bus errors when direct serial mode is enabled.
    /// Bits `[0:0]`, mask `0x00000001`
    pub const EN_OFFSET: u32 = 0;
    pub const EN_MASK: u32 = 0x00000001;
    pub const EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **RXDELAY** `[31:30]` (RW)
    /// Delay the read data sample timing, in units of one half of a system clock cycle. (Not necessarily half of an SCK cycle.)
    #[inline(always)]
    pub fn rxdelay(&self) -> u32 {
        (self.value.read_bits() & Self::RXDELAY_MASK) >> Self::RXDELAY_OFFSET
    }

    /// Write **RXDELAY** `[31:30]` (RW)
    /// Delay the read data sample timing, in units of one half of a system clock cycle. (Not necessarily half of an SCK cycle.)
    #[inline(always)]
    pub fn write_rxdelay(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::RXDELAY_MASK) | ((value << Self::RXDELAY_OFFSET) & Self::RXDELAY_MASK));
    }

    /// **CLKDIV** `[29:22]` (RW)
    /// Clock divisor for direct serial mode. Divisors of 1..255 are encoded directly, and the maximum divisor of 256 is encoded by a value of CLKDIV=0.
    /// The clock divisor can be changed on-the-fly by software, without halting or otherwise coordinating with the serial interface. The serial interface will sample the latest clock divisor each time it begins the transmission of a new byte.
    #[inline(always)]
    pub fn clkdiv(&self) -> u32 {
        (self.value.read_bits() & Self::CLKDIV_MASK) >> Self::CLKDIV_OFFSET
    }

    /// Write **CLKDIV** `[29:22]` (RW)
    /// Clock divisor for direct serial mode. Divisors of 1..255 are encoded directly, and the maximum divisor of 256 is encoded by a value of CLKDIV=0.
    /// The clock divisor can be changed on-the-fly by software, without halting or otherwise coordinating with the serial interface. The serial interface will sample the latest clock divisor each time it begins the transmission of a new byte.
    #[inline(always)]
    pub fn write_clkdiv(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::CLKDIV_MASK) | ((value << Self::CLKDIV_OFFSET) & Self::CLKDIV_MASK));
    }

    /// **RXLEVEL** `[20:18]` (RO)
    /// Current level of DIRECT_RX FIFO
    #[inline(always)]
    pub fn rxlevel(&self) -> u32 {
        (self.value.read_bits() & Self::RXLEVEL_MASK) >> Self::RXLEVEL_OFFSET
    }

    /// **RXFULL** `[17]` (RO)
    /// When 1, the DIRECT_RX FIFO is currently full. The serial interface will be stalled until data is popped; the interface will not begin a new serial frame when the DIRECT_TX FIFO is empty or the DIRECT_RX FIFO is full.
    #[inline(always)]
    pub fn rxfull(&self) -> bool {
        (self.value.read_bits() & Self::RXFULL_MASK) != 0
    }

    /// **RXEMPTY** `[16]` (RO)
    /// When 1, the DIRECT_RX FIFO is currently empty. If the processor attempts to read more data, the FIFO state is not affected, but the value returned to the processor is undefined.
    #[inline(always)]
    pub fn rxempty(&self) -> bool {
        (self.value.read_bits() & Self::RXEMPTY_MASK) != 0
    }

    /// **TXLEVEL** `[14:12]` (RO)
    /// Current level of DIRECT_TX FIFO
    #[inline(always)]
    pub fn txlevel(&self) -> u32 {
        (self.value.read_bits() & Self::TXLEVEL_MASK) >> Self::TXLEVEL_OFFSET
    }

    /// **TXEMPTY** `[11]` (RO)
    /// When 1, the DIRECT_TX FIFO is currently empty. Unless the processor pushes more data, transmission will stop and BUSY will go low once the current 8-bit serial frame completes.
    #[inline(always)]
    pub fn txempty(&self) -> bool {
        (self.value.read_bits() & Self::TXEMPTY_MASK) != 0
    }

    /// **TXFULL** `[10]` (RO)
    /// When 1, the DIRECT_TX FIFO is currently full. If the processor tries to write more data, that data will be ignored.
    #[inline(always)]
    pub fn txfull(&self) -> bool {
        (self.value.read_bits() & Self::TXFULL_MASK) != 0
    }

    /// **AUTO_CS1N** `[7]` (RW)
    /// When 1, automatically assert the CS1n chip select line whenever the BUSY flag is set.
    #[inline(always)]
    pub fn auto_cs1n(&self) -> bool {
        (self.value.read_bits() & Self::AUTO_CS1N_MASK) != 0
    }

    /// Set AUTO_CS1N bit atomically (RW)
    #[inline(always)]
    pub fn set_auto_cs1n(&self) {
        self.value.set_bits(Self::AUTO_CS1N_MASK);
    }

    /// Clear AUTO_CS1N bit atomically (RW)
    #[inline(always)]
    pub fn clr_auto_cs1n(&self) {
        self.value.clr_bits(Self::AUTO_CS1N_MASK);
    }

    /// **AUTO_CS0N** `[6]` (RW)
    /// When 1, automatically assert the CS0n chip select line whenever the BUSY flag is set.
    #[inline(always)]
    pub fn auto_cs0n(&self) -> bool {
        (self.value.read_bits() & Self::AUTO_CS0N_MASK) != 0
    }

    /// Set AUTO_CS0N bit atomically (RW)
    #[inline(always)]
    pub fn set_auto_cs0n(&self) {
        self.value.set_bits(Self::AUTO_CS0N_MASK);
    }

    /// Clear AUTO_CS0N bit atomically (RW)
    #[inline(always)]
    pub fn clr_auto_cs0n(&self) {
        self.value.clr_bits(Self::AUTO_CS0N_MASK);
    }

    /// **ASSERT_CS1N** `[3]` (RW)
    /// When 1, assert (i.e. drive low) the CS1n chip select line.
    /// Note that this applies even when DIRECT_CSR_EN is 0.
    #[inline(always)]
    pub fn assert_cs1n(&self) -> bool {
        (self.value.read_bits() & Self::ASSERT_CS1N_MASK) != 0
    }

    /// Set ASSERT_CS1N bit atomically (RW)
    #[inline(always)]
    pub fn set_assert_cs1n(&self) {
        self.value.set_bits(Self::ASSERT_CS1N_MASK);
    }

    /// Clear ASSERT_CS1N bit atomically (RW)
    #[inline(always)]
    pub fn clr_assert_cs1n(&self) {
        self.value.clr_bits(Self::ASSERT_CS1N_MASK);
    }

    /// **ASSERT_CS0N** `[2]` (RW)
    /// When 1, assert (i.e. drive low) the CS0n chip select line.
    /// Note that this applies even when DIRECT_CSR_EN is 0.
    #[inline(always)]
    pub fn assert_cs0n(&self) -> bool {
        (self.value.read_bits() & Self::ASSERT_CS0N_MASK) != 0
    }

    /// Set ASSERT_CS0N bit atomically (RW)
    #[inline(always)]
    pub fn set_assert_cs0n(&self) {
        self.value.set_bits(Self::ASSERT_CS0N_MASK);
    }

    /// Clear ASSERT_CS0N bit atomically (RW)
    #[inline(always)]
    pub fn clr_assert_cs0n(&self) {
        self.value.clr_bits(Self::ASSERT_CS0N_MASK);
    }

    /// **BUSY** `[1]` (RO)
    /// Direct mode busy flag. If 1, data is currently being shifted in/out (or would be if the interface were not stalled on the RX FIFO), and the chip select must not yet be deasserted.
    /// The busy flag will also be set to 1 if a memory-mapped transfer is still in progress when direct mode is enabled. Direct mode blocks new memory-mapped transfers, but can't halt a transfer that is already in progress. If there is a chance that memory-mapped transfers may be in progress, the busy flag should be polled for 0 before asserting the chip select.
    /// (In practice you will usually discover this timing condition through other means, because any subsequent memory-mapped transfers when direct mode is enabled will return bus errors, which are difficult to ignore.)
    #[inline(always)]
    pub fn busy(&self) -> bool {
        (self.value.read_bits() & Self::BUSY_MASK) != 0
    }

    /// **EN** `[0]` (RW)
    /// Enable direct mode.
    /// In direct mode, software controls the chip select lines, and can perform direct SPI transfers by pushing data to the DIRECT_TX FIFO, and popping the same amount of data from the DIRECT_RX FIFO.
    /// Memory-mapped accesses will generate bus errors when direct serial mode is enabled.
    #[inline(always)]
    pub fn en(&self) -> bool {
        (self.value.read_bits() & Self::EN_MASK) != 0
    }

    /// Set EN bit atomically (RW)
    #[inline(always)]
    pub fn set_en(&self) {
        self.value.set_bits(Self::EN_MASK);
    }

    /// Clear EN bit atomically (RW)
    #[inline(always)]
    pub fn clr_en(&self) {
        self.value.clr_bits(Self::EN_MASK);
    }

}

/// DIRECT_CSR value type for batch read/modify/write operations
///
/// Use `directcsr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `directcsr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DirectCsrVal {
    bits: u32,
}

impl DirectCsrVal {
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
        Self { bits: 0x01800000 }
    }

    /// Read **RXDELAY** `[31:30]`
    /// Delay the read data sample timing, in units of one half of a system clock cycle. (Not necessarily half of an SCK cycle.)
    #[inline(always)]
    pub const fn rxdelay(&self) -> u32 {
        (self.bits & DirectCsr::RXDELAY_MASK) >> DirectCsr::RXDELAY_OFFSET
    }

    /// Read **CLKDIV** `[29:22]`
    /// Clock divisor for direct serial mode. Divisors of 1..255 are encoded directly, and the maximum divisor of 256 is encoded by a value of CLKDIV=0.
    /// The clock divisor can be changed on-the-fly by software, without halting or otherwise coordinating with the serial interface. The serial interface will sample the latest clock divisor each time it begins the transmission of a new byte.
    #[inline(always)]
    pub const fn clkdiv(&self) -> u32 {
        (self.bits & DirectCsr::CLKDIV_MASK) >> DirectCsr::CLKDIV_OFFSET
    }

    /// Read **RXLEVEL** `[20:18]`
    /// Current level of DIRECT_RX FIFO
    #[inline(always)]
    pub const fn rxlevel(&self) -> u32 {
        (self.bits & DirectCsr::RXLEVEL_MASK) >> DirectCsr::RXLEVEL_OFFSET
    }

    /// Read **RXFULL** `[17:17]`
    /// When 1, the DIRECT_RX FIFO is currently full. The serial interface will be stalled until data is popped; the interface will not begin a new serial frame when the DIRECT_TX FIFO is empty or the DIRECT_RX FIFO is full.
    #[inline(always)]
    pub const fn rxfull(&self) -> bool {
        (self.bits & DirectCsr::RXFULL_MASK) != 0
    }

    /// Read **RXEMPTY** `[16:16]`
    /// When 1, the DIRECT_RX FIFO is currently empty. If the processor attempts to read more data, the FIFO state is not affected, but the value returned to the processor is undefined.
    #[inline(always)]
    pub const fn rxempty(&self) -> bool {
        (self.bits & DirectCsr::RXEMPTY_MASK) != 0
    }

    /// Read **TXLEVEL** `[14:12]`
    /// Current level of DIRECT_TX FIFO
    #[inline(always)]
    pub const fn txlevel(&self) -> u32 {
        (self.bits & DirectCsr::TXLEVEL_MASK) >> DirectCsr::TXLEVEL_OFFSET
    }

    /// Read **TXEMPTY** `[11:11]`
    /// When 1, the DIRECT_TX FIFO is currently empty. Unless the processor pushes more data, transmission will stop and BUSY will go low once the current 8-bit serial frame completes.
    #[inline(always)]
    pub const fn txempty(&self) -> bool {
        (self.bits & DirectCsr::TXEMPTY_MASK) != 0
    }

    /// Read **TXFULL** `[10:10]`
    /// When 1, the DIRECT_TX FIFO is currently full. If the processor tries to write more data, that data will be ignored.
    #[inline(always)]
    pub const fn txfull(&self) -> bool {
        (self.bits & DirectCsr::TXFULL_MASK) != 0
    }

    /// Read **AUTO_CS1N** `[7:7]`
    /// When 1, automatically assert the CS1n chip select line whenever the BUSY flag is set.
    #[inline(always)]
    pub const fn auto_cs1n(&self) -> bool {
        (self.bits & DirectCsr::AUTO_CS1N_MASK) != 0
    }

    /// Read **AUTO_CS0N** `[6:6]`
    /// When 1, automatically assert the CS0n chip select line whenever the BUSY flag is set.
    #[inline(always)]
    pub const fn auto_cs0n(&self) -> bool {
        (self.bits & DirectCsr::AUTO_CS0N_MASK) != 0
    }

    /// Read **ASSERT_CS1N** `[3:3]`
    /// When 1, assert (i.e. drive low) the CS1n chip select line.
    /// Note that this applies even when DIRECT_CSR_EN is 0.
    #[inline(always)]
    pub const fn assert_cs1n(&self) -> bool {
        (self.bits & DirectCsr::ASSERT_CS1N_MASK) != 0
    }

    /// Read **ASSERT_CS0N** `[2:2]`
    /// When 1, assert (i.e. drive low) the CS0n chip select line.
    /// Note that this applies even when DIRECT_CSR_EN is 0.
    #[inline(always)]
    pub const fn assert_cs0n(&self) -> bool {
        (self.bits & DirectCsr::ASSERT_CS0N_MASK) != 0
    }

    /// Read **BUSY** `[1:1]`
    /// Direct mode busy flag. If 1, data is currently being shifted in/out (or would be if the interface were not stalled on the RX FIFO), and the chip select must not yet be deasserted.
    /// The busy flag will also be set to 1 if a memory-mapped transfer is still in progress when direct mode is enabled. Direct mode blocks new memory-mapped transfers, but can't halt a transfer that is already in progress. If there is a chance that memory-mapped transfers may be in progress, the busy flag should be polled for 0 before asserting the chip select.
    /// (In practice you will usually discover this timing condition through other means, because any subsequent memory-mapped transfers when direct mode is enabled will return bus errors, which are difficult to ignore.)
    #[inline(always)]
    pub const fn busy(&self) -> bool {
        (self.bits & DirectCsr::BUSY_MASK) != 0
    }

    /// Read **EN** `[0:0]`
    /// Enable direct mode.
    /// In direct mode, software controls the chip select lines, and can perform direct SPI transfers by pushing data to the DIRECT_TX FIFO, and popping the same amount of data from the DIRECT_RX FIFO.
    /// Memory-mapped accesses will generate bus errors when direct serial mode is enabled.
    #[inline(always)]
    pub const fn en(&self) -> bool {
        (self.bits & DirectCsr::EN_MASK) != 0
    }

    /// Set **RXDELAY** `[31:30]`
    /// Delay the read data sample timing, in units of one half of a system clock cycle. (Not necessarily half of an SCK cycle.)
    #[inline(always)]
    pub const fn with_rxdelay(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DirectCsr::RXDELAY_MASK) | ((value << DirectCsr::RXDELAY_OFFSET) & DirectCsr::RXDELAY_MASK)
        }
    }

    /// Set **CLKDIV** `[29:22]`
    /// Clock divisor for direct serial mode. Divisors of 1..255 are encoded directly, and the maximum divisor of 256 is encoded by a value of CLKDIV=0.
    /// The clock divisor can be changed on-the-fly by software, without halting or otherwise coordinating with the serial interface. The serial interface will sample the latest clock divisor each time it begins the transmission of a new byte.
    #[inline(always)]
    pub const fn with_clkdiv(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DirectCsr::CLKDIV_MASK) | ((value << DirectCsr::CLKDIV_OFFSET) & DirectCsr::CLKDIV_MASK)
        }
    }

    /// Set **AUTO_CS1N** `[7:7]`
    /// When 1, automatically assert the CS1n chip select line whenever the BUSY flag is set.
    #[inline(always)]
    pub const fn with_auto_cs1n(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DirectCsr::AUTO_CS1N_MASK
            } else {
                self.bits & !DirectCsr::AUTO_CS1N_MASK
            }
        }
    }

    /// Set **AUTO_CS0N** `[6:6]`
    /// When 1, automatically assert the CS0n chip select line whenever the BUSY flag is set.
    #[inline(always)]
    pub const fn with_auto_cs0n(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DirectCsr::AUTO_CS0N_MASK
            } else {
                self.bits & !DirectCsr::AUTO_CS0N_MASK
            }
        }
    }

    /// Set **ASSERT_CS1N** `[3:3]`
    /// When 1, assert (i.e. drive low) the CS1n chip select line.
    /// Note that this applies even when DIRECT_CSR_EN is 0.
    #[inline(always)]
    pub const fn with_assert_cs1n(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DirectCsr::ASSERT_CS1N_MASK
            } else {
                self.bits & !DirectCsr::ASSERT_CS1N_MASK
            }
        }
    }

    /// Set **ASSERT_CS0N** `[2:2]`
    /// When 1, assert (i.e. drive low) the CS0n chip select line.
    /// Note that this applies even when DIRECT_CSR_EN is 0.
    #[inline(always)]
    pub const fn with_assert_cs0n(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DirectCsr::ASSERT_CS0N_MASK
            } else {
                self.bits & !DirectCsr::ASSERT_CS0N_MASK
            }
        }
    }

    /// Set **EN** `[0:0]`
    /// Enable direct mode.
    /// In direct mode, software controls the chip select lines, and can perform direct SPI transfers by pushing data to the DIRECT_TX FIFO, and popping the same amount of data from the DIRECT_RX FIFO.
    /// Memory-mapped accesses will generate bus errors when direct serial mode is enabled.
    #[inline(always)]
    pub const fn with_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DirectCsr::EN_MASK
            } else {
                self.bits & !DirectCsr::EN_MASK
            }
        }
    }

}

impl Default for DirectCsrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl DirectCsr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DirectCsrVal {
        DirectCsrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DirectCsrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DirectCsrVal) -> DirectCsrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// DIRECT_TX Register
///
/// Transmit FIFO for direct mode
///
/// Address offset: 0x0004
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NOPUSH** `[20]` (WO)
///   Inhibit the RX FIFO push that would correspond to this TX FIFO entry.
///   Useful to avoid garbage appearing in the RX FIFO when pushing the command at the beginning of a SPI transfer.
/// - **OE** `[19]` (WO)
///   Output enable (active-high). For single width (SPI), this field is ignored, and SD0 is always set to output, with SD1 always set to input.
///   For dual and quad width (DSPI/QSPI), this sets whether the relevant SDx pads are set to output whilst transferring this FIFO record. In this case the command/address should have OE set, and the data transfer should have OE set or clear depending on the direction of the transfer.
/// - **DWIDTH** `[18]` (WO)
///   Data width. If 0, hardware will transmit the 8 LSBs of the DIRECT_TX DATA field, and return an 8-bit value in the 8 LSBs of DIRECT_RX. If 1, the full 16-bit width is used. 8-bit and 16-bit transfers can be mixed freely.
/// - **IWIDTH** `[17:16]` (WO)
///   Configure whether this FIFO record is transferred with single/dual/quad interface width (0/1/2). Different widths can be mixed freely.
/// - **DATA** `[15:0]` (WO)
///   Data pushed here will be clocked out falling edges of SCK (or before the very first rising edge of SCK, if this is the first pulse). For each byte clocked out, the interface will simultaneously sample one byte, on rising edges of SCK, and push this to the DIRECT_RX FIFO.
///   For 16-bit data, the least-significant byte is transmitted first.
#[repr(transparent)]
pub struct DirectTx {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DirectTx {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DirectTx {
    // ======== Field Constants ========

    /// **NOPUSH**
    /// Inhibit the RX FIFO push that would correspond to this TX FIFO entry.
    /// Useful to avoid garbage appearing in the RX FIFO when pushing the command at the beginning of a SPI transfer.
    /// Bits `[20:20]`, mask `0x00100000`
    pub const NOPUSH_OFFSET: u32 = 20;
    pub const NOPUSH_MASK: u32 = 0x00100000;
    pub const NOPUSH: u32 = 1 << 20;

    /// **OE**
    /// Output enable (active-high). For single width (SPI), this field is ignored, and SD0 is always set to output, with SD1 always set to input.
    /// For dual and quad width (DSPI/QSPI), this sets whether the relevant SDx pads are set to output whilst transferring this FIFO record. In this case the command/address should have OE set, and the data transfer should have OE set or clear depending on the direction of the transfer.
    /// Bits `[19:19]`, mask `0x00080000`
    pub const OE_OFFSET: u32 = 19;
    pub const OE_MASK: u32 = 0x00080000;
    pub const OE: u32 = 1 << 19;

    /// **DWIDTH**
    /// Data width. If 0, hardware will transmit the 8 LSBs of the DIRECT_TX DATA field, and return an 8-bit value in the 8 LSBs of DIRECT_RX. If 1, the full 16-bit width is used. 8-bit and 16-bit transfers can be mixed freely.
    /// Bits `[18:18]`, mask `0x00040000`
    pub const DWIDTH_OFFSET: u32 = 18;
    pub const DWIDTH_MASK: u32 = 0x00040000;
    pub const DWIDTH: u32 = 1 << 18;

    /// **IWIDTH**
    /// Configure whether this FIFO record is transferred with single/dual/quad interface width (0/1/2). Different widths can be mixed freely.
    /// Bits `[17:16]`, mask `0x00030000`
    pub const IWIDTH_OFFSET: u32 = 16;
    pub const IWIDTH_MASK: u32 = 0x00030000;
    /// S = 0
    /// Single width
    pub const IWIDTH_S: u32 = 0 << 16;
    pub const IWIDTH_S_VAL: u32 = 0;
    /// D = 1
    /// Dual width
    pub const IWIDTH_D: u32 = 1 << 16;
    pub const IWIDTH_D_VAL: u32 = 1;
    /// Q = 2
    /// Quad width
    pub const IWIDTH_Q: u32 = 2 << 16;
    pub const IWIDTH_Q_VAL: u32 = 2;

    /// **DATA**
    /// Data pushed here will be clocked out falling edges of SCK (or before the very first rising edge of SCK, if this is the first pulse). For each byte clocked out, the interface will simultaneously sample one byte, on rising edges of SCK, and push this to the DIRECT_RX FIFO.
    /// For 16-bit data, the least-significant byte is transmitted first.
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const DATA_OFFSET: u32 = 0;
    pub const DATA_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **NOPUSH** `[20]` (WO)
    /// Inhibit the RX FIFO push that would correspond to this TX FIFO entry.
    /// Useful to avoid garbage appearing in the RX FIFO when pushing the command at the beginning of a SPI transfer.
    /// Trigger **NOPUSH** (WO)
    /// Inhibit the RX FIFO push that would correspond to this TX FIFO entry.
    /// Useful to avoid garbage appearing in the RX FIFO when pushing the command at the beginning of a SPI transfer.
    #[inline(always)]
    pub fn trigger_nopush(&self) {
        self.value.write_bits(Self::NOPUSH_MASK);
    }

    /// **OE** `[19]` (WO)
    /// Output enable (active-high). For single width (SPI), this field is ignored, and SD0 is always set to output, with SD1 always set to input.
    /// For dual and quad width (DSPI/QSPI), this sets whether the relevant SDx pads are set to output whilst transferring this FIFO record. In this case the command/address should have OE set, and the data transfer should have OE set or clear depending on the direction of the transfer.
    /// Trigger **OE** (WO)
    /// Output enable (active-high). For single width (SPI), this field is ignored, and SD0 is always set to output, with SD1 always set to input.
    /// For dual and quad width (DSPI/QSPI), this sets whether the relevant SDx pads are set to output whilst transferring this FIFO record. In this case the command/address should have OE set, and the data transfer should have OE set or clear depending on the direction of the transfer.
    #[inline(always)]
    pub fn trigger_oe(&self) {
        self.value.write_bits(Self::OE_MASK);
    }

    /// **DWIDTH** `[18]` (WO)
    /// Data width. If 0, hardware will transmit the 8 LSBs of the DIRECT_TX DATA field, and return an 8-bit value in the 8 LSBs of DIRECT_RX. If 1, the full 16-bit width is used. 8-bit and 16-bit transfers can be mixed freely.
    /// Trigger **DWIDTH** (WO)
    /// Data width. If 0, hardware will transmit the 8 LSBs of the DIRECT_TX DATA field, and return an 8-bit value in the 8 LSBs of DIRECT_RX. If 1, the full 16-bit width is used. 8-bit and 16-bit transfers can be mixed freely.
    #[inline(always)]
    pub fn trigger_dwidth(&self) {
        self.value.write_bits(Self::DWIDTH_MASK);
    }

    /// **IWIDTH** `[17:16]` (WO)
    /// Configure whether this FIFO record is transferred with single/dual/quad interface width (0/1/2). Different widths can be mixed freely.
    /// Write **IWIDTH** `[17:16]` (WO)
    /// Configure whether this FIFO record is transferred with single/dual/quad interface width (0/1/2). Different widths can be mixed freely.
    #[inline(always)]
    pub fn write_iwidth(&self, value: u32) {
        self.value.write_bits((value << Self::IWIDTH_OFFSET) & Self::IWIDTH_MASK);
    }

    /// **DATA** `[15:0]` (WO)
    /// Data pushed here will be clocked out falling edges of SCK (or before the very first rising edge of SCK, if this is the first pulse). For each byte clocked out, the interface will simultaneously sample one byte, on rising edges of SCK, and push this to the DIRECT_RX FIFO.
    /// For 16-bit data, the least-significant byte is transmitted first.
    /// Write **DATA** `[15:0]` (WO)
    /// Data pushed here will be clocked out falling edges of SCK (or before the very first rising edge of SCK, if this is the first pulse). For each byte clocked out, the interface will simultaneously sample one byte, on rising edges of SCK, and push this to the DIRECT_RX FIFO.
    /// For 16-bit data, the least-significant byte is transmitted first.
    #[inline(always)]
    pub fn write_data(&self, value: u32) {
        self.value.write_bits((value << Self::DATA_OFFSET) & Self::DATA_MASK);
    }

}

/// DIRECT_TX value type for batch read/modify/write operations
///
/// Use `directtx.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `directtx.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DirectTxVal {
    bits: u32,
}

impl DirectTxVal {
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

    /// Set **NOPUSH** `[20:20]`
    /// Inhibit the RX FIFO push that would correspond to this TX FIFO entry.
    /// Useful to avoid garbage appearing in the RX FIFO when pushing the command at the beginning of a SPI transfer.
    #[inline(always)]
    pub const fn with_nopush(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DirectTx::NOPUSH_MASK
            } else {
                self.bits & !DirectTx::NOPUSH_MASK
            }
        }
    }

    /// Set **OE** `[19:19]`
    /// Output enable (active-high). For single width (SPI), this field is ignored, and SD0 is always set to output, with SD1 always set to input.
    /// For dual and quad width (DSPI/QSPI), this sets whether the relevant SDx pads are set to output whilst transferring this FIFO record. In this case the command/address should have OE set, and the data transfer should have OE set or clear depending on the direction of the transfer.
    #[inline(always)]
    pub const fn with_oe(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DirectTx::OE_MASK
            } else {
                self.bits & !DirectTx::OE_MASK
            }
        }
    }

    /// Set **DWIDTH** `[18:18]`
    /// Data width. If 0, hardware will transmit the 8 LSBs of the DIRECT_TX DATA field, and return an 8-bit value in the 8 LSBs of DIRECT_RX. If 1, the full 16-bit width is used. 8-bit and 16-bit transfers can be mixed freely.
    #[inline(always)]
    pub const fn with_dwidth(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DirectTx::DWIDTH_MASK
            } else {
                self.bits & !DirectTx::DWIDTH_MASK
            }
        }
    }

    /// Set **IWIDTH** `[17:16]`
    /// Configure whether this FIFO record is transferred with single/dual/quad interface width (0/1/2). Different widths can be mixed freely.
    #[inline(always)]
    pub const fn with_iwidth(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DirectTx::IWIDTH_MASK) | ((value << DirectTx::IWIDTH_OFFSET) & DirectTx::IWIDTH_MASK)
        }
    }

    /// Set **DATA** `[15:0]`
    /// Data pushed here will be clocked out falling edges of SCK (or before the very first rising edge of SCK, if this is the first pulse). For each byte clocked out, the interface will simultaneously sample one byte, on rising edges of SCK, and push this to the DIRECT_RX FIFO.
    /// For 16-bit data, the least-significant byte is transmitted first.
    #[inline(always)]
    pub const fn with_data(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DirectTx::DATA_MASK) | ((value << DirectTx::DATA_OFFSET) & DirectTx::DATA_MASK)
        }
    }

}

impl Default for DirectTxVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl DirectTx {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DirectTxVal {
        DirectTxVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DirectTxVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DirectTxVal) -> DirectTxVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// DIRECT_RX Register
///
/// Receive FIFO for direct mode
///
/// Address offset: 0x0008
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **DIRECT_RX** `[15:0]` (RO)
///   With each byte clocked out on the serial interface, one byte will simultaneously be clocked in, and will appear in this FIFO. The serial interface will stall when this FIFO is full, to avoid dropping data.
///   When 16-bit data is pushed into the TX FIFO, the corresponding RX FIFO push will also contain 16 bits of data. The least-significant byte is the first one received.
#[repr(transparent)]
pub struct DirectRx {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DirectRx {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DirectRx {
    // ======== Field Constants ========

    /// **DIRECT_RX**
    /// With each byte clocked out on the serial interface, one byte will simultaneously be clocked in, and will appear in this FIFO. The serial interface will stall when this FIFO is full, to avoid dropping data.
    /// When 16-bit data is pushed into the TX FIFO, the corresponding RX FIFO push will also contain 16 bits of data. The least-significant byte is the first one received.
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const DIRECT_RX_OFFSET: u32 = 0;
    pub const DIRECT_RX_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **DIRECT_RX** `[15:0]` (RO)
    /// With each byte clocked out on the serial interface, one byte will simultaneously be clocked in, and will appear in this FIFO. The serial interface will stall when this FIFO is full, to avoid dropping data.
    /// When 16-bit data is pushed into the TX FIFO, the corresponding RX FIFO push will also contain 16 bits of data. The least-significant byte is the first one received.
    #[inline(always)]
    pub fn direct_rx(&self) -> u32 {
        (self.value.read_bits() & Self::DIRECT_RX_MASK) >> Self::DIRECT_RX_OFFSET
    }

}

/// DIRECT_RX value type for batch read/modify/write operations
///
/// Use `directrx.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `directrx.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DirectRxVal {
    bits: u32,
}

impl DirectRxVal {
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

    /// Read **DIRECT_RX** `[15:0]`
    /// With each byte clocked out on the serial interface, one byte will simultaneously be clocked in, and will appear in this FIFO. The serial interface will stall when this FIFO is full, to avoid dropping data.
    /// When 16-bit data is pushed into the TX FIFO, the corresponding RX FIFO push will also contain 16 bits of data. The least-significant byte is the first one received.
    #[inline(always)]
    pub const fn direct_rx(&self) -> u32 {
        (self.bits & DirectRx::DIRECT_RX_MASK) >> DirectRx::DIRECT_RX_OFFSET
    }

}

impl Default for DirectRxVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl DirectRx {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DirectRxVal {
        DirectRxVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DirectRxVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DirectRxVal) -> DirectRxVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// M0_TIMING Register
///
/// Timing configuration register for memory address window 0.
///
/// Address offset: 0x000c
/// Reset value: 0x40000004
///
/// ## Fields
///
/// - **COOLDOWN** `[31:30]` (RW)
///   Chip select cooldown period. When a memory transfer finishes, the chip select remains asserted for 64 x COOLDOWN system clock cycles, plus half an SCK clock period (rounded up for odd SCK divisors). After this cooldown expires, the chip select is always deasserted to save power.
///   If the next memory access arrives within the cooldown period, the QMI may be able to append more SCK cycles to the currently ongoing SPI transfer, rather than starting a new transfer. This reduces access latency and increases bus throughput.
///   Specifically, the next access must be in the same direction (read/write), access the same memory window (chip select 0/1), and follow sequentially the address of the last transfer. If any of these are false, the new access will first deassert the chip select, then begin a new transfer.
///   If COOLDOWN is 0, the address alignment configured by PAGEBREAK has been reached, or the total chip select assertion limit MAX_SELECT has been reached, the cooldown period is skipped, and the chip select will always be deasserted one half SCK period after the transfer finishes.
/// - **PAGEBREAK** `[29:28]` (RW)
///   When page break is enabled, chip select will automatically deassert when crossing certain power-of-2-aligned address boundaries. The next access will always begin a new read/write SPI burst, even if the address of the next access follows in sequence with the last access before the page boundary.
///   Some flash and PSRAM devices forbid crossing page boundaries with a single read/write transfer, or restrict the operating frequency for transfers that do cross page a boundary. This option allows the QMI to safely support those devices.
///   This field has no effect when COOLDOWN is disabled.
/// - **SELECT_SETUP** `[25]` (RW)
///   Add up to one additional system clock cycle of setup between chip select assertion and the first rising edge of SCK.
///   The default setup time is one half SCK period, which is usually sufficient except for very high SCK frequencies with some flash devices.
/// - **SELECT_HOLD** `[24:23]` (RW)
///   Add up to three additional system clock cycles of active hold between the last falling edge of SCK and the deassertion of this window's chip select.
///   The default hold time is one system clock cycle. Note that flash datasheets usually give chip select active hold time from the last *rising* edge of SCK, and so even zero hold from the last falling edge would be safe.
///   Note that this is a minimum hold time guaranteed by the QMI: the actual chip select active hold may be slightly longer for read transfers with low clock divisors and/or high sample delays. Specifically, if the point two cycles after the last RX data sample is later than the last SCK falling edge, then the hold time is measured from *this* point.
///   Note also that, in case the final SCK pulse is masked to save energy (true for non-DTR reads when COOLDOWN is disabled or PAGE_BREAK is reached), all of QMI's timing logic behaves as though the clock pulse were still present. The SELECT_HOLD time is applied from the point where the last SCK falling edge would be if the clock pulse were not masked.
/// - **MAX_SELECT** `[22:17]` (RW)
///   Enforce a maximum assertion duration for this window's chip select, in units of 64 system clock cycles. If 0, the QMI is permitted to keep the chip select asserted indefinitely when servicing sequential memory accesses (see COOLDOWN).
///   This feature is required to meet timing constraints of PSRAM devices, which specify a maximum chip select assertion so they can perform DRAM refresh cycles. See also MIN_DESELECT, which can enforce a minimum deselect time.
///   If a memory access is in progress at the time MAX_SELECT is reached, the QMI will wait for the access to complete before deasserting the chip select. This additional time must be accounted for to calculate a safe MAX_SELECT value. In the worst case, this may be a fully-formed serial transfer, including command prefix and address, with a data payload as large as one cache line.
/// - **MIN_DESELECT** `[16:12]` (RW)
///   After this window's chip select is deasserted, it remains deasserted for half an SCK cycle (rounded up to an integer number of system clock cycles), plus MIN_DESELECT additional system clock cycles, before the QMI reasserts either chip select pin.
///   Nonzero values may be required for PSRAM devices which enforce a longer minimum CS deselect time, so that they can perform internal DRAM refresh cycles whilst deselected.
/// - **RXDELAY** `[10:8]` (RW)
///   Delay the read data sample timing, in units of one half of a system clock cycle. (Not necessarily half of an SCK cycle.) An RXDELAY of 0 means the sample is captured at the SDI input registers simultaneously with the rising edge of SCK launched from the SCK output register.
///   At higher SCK frequencies, RXDELAY may need to be increased to account for the round trip delay of the pads, and the clock-to-Q delay of the QSPI memory device.
/// - **CLKDIV** `[7:0]` (RW)
///   Clock divisor. Odd and even divisors are supported. Defines the SCK clock period in units of 1 system clock cycle. Divisors 1..255 are encoded directly, and a divisor of 256 is encoded with a value of CLKDIV=0.
///   The clock divisor can be changed on-the-fly, even when the QMI is currently accessing memory in this address window. All other parameters must only be changed when the QMI is idle.
///   If software is increasing CLKDIV in anticipation of an increase in the system clock frequency, a dummy access to either memory window (and appropriate processor barriers/fences) must be inserted after the Mx_TIMING write to ensure the SCK divisor change is in effect _before_ the system clock is changed.
#[repr(transparent)]
pub struct M0Timing {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for M0Timing {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl M0Timing {
    // ======== Field Constants ========

    /// **COOLDOWN**
    /// Chip select cooldown period. When a memory transfer finishes, the chip select remains asserted for 64 x COOLDOWN system clock cycles, plus half an SCK clock period (rounded up for odd SCK divisors). After this cooldown expires, the chip select is always deasserted to save power.
    /// If the next memory access arrives within the cooldown period, the QMI may be able to append more SCK cycles to the currently ongoing SPI transfer, rather than starting a new transfer. This reduces access latency and increases bus throughput.
    /// Specifically, the next access must be in the same direction (read/write), access the same memory window (chip select 0/1), and follow sequentially the address of the last transfer. If any of these are false, the new access will first deassert the chip select, then begin a new transfer.
    /// If COOLDOWN is 0, the address alignment configured by PAGEBREAK has been reached, or the total chip select assertion limit MAX_SELECT has been reached, the cooldown period is skipped, and the chip select will always be deasserted one half SCK period after the transfer finishes.
    /// Bits `[31:30]`, mask `0xc0000000`
    pub const COOLDOWN_OFFSET: u32 = 30;
    pub const COOLDOWN_MASK: u32 = 0xc0000000;

    /// **PAGEBREAK**
    /// When page break is enabled, chip select will automatically deassert when crossing certain power-of-2-aligned address boundaries. The next access will always begin a new read/write SPI burst, even if the address of the next access follows in sequence with the last access before the page boundary.
    /// Some flash and PSRAM devices forbid crossing page boundaries with a single read/write transfer, or restrict the operating frequency for transfers that do cross page a boundary. This option allows the QMI to safely support those devices.
    /// This field has no effect when COOLDOWN is disabled.
    /// Bits `[29:28]`, mask `0x30000000`
    pub const PAGEBREAK_OFFSET: u32 = 28;
    pub const PAGEBREAK_MASK: u32 = 0x30000000;
    /// NONE = 0
    /// No page boundary is enforced
    pub const PAGEBREAK_NONE: u32 = 0 << 28;
    pub const PAGEBREAK_NONE_VAL: u32 = 0;
    /// 256 = 1
    /// Break bursts crossing a 256-byte page boundary
    pub const PAGEBREAK_256: u32 = 1 << 28;
    pub const PAGEBREAK_256_VAL: u32 = 1;
    /// 1024 = 2
    /// Break bursts crossing a 1024-byte quad-page boundary
    pub const PAGEBREAK_1024: u32 = 2 << 28;
    pub const PAGEBREAK_1024_VAL: u32 = 2;
    /// 4096 = 3
    /// Break bursts crossing a 4096-byte sector boundary
    pub const PAGEBREAK_4096: u32 = 3 << 28;
    pub const PAGEBREAK_4096_VAL: u32 = 3;

    /// **SELECT_SETUP**
    /// Add up to one additional system clock cycle of setup between chip select assertion and the first rising edge of SCK.
    /// The default setup time is one half SCK period, which is usually sufficient except for very high SCK frequencies with some flash devices.
    /// Bits `[25:25]`, mask `0x02000000`
    pub const SELECT_SETUP_OFFSET: u32 = 25;
    pub const SELECT_SETUP_MASK: u32 = 0x02000000;
    pub const SELECT_SETUP: u32 = 1 << 25;

    /// **SELECT_HOLD**
    /// Add up to three additional system clock cycles of active hold between the last falling edge of SCK and the deassertion of this window's chip select.
    /// The default hold time is one system clock cycle. Note that flash datasheets usually give chip select active hold time from the last *rising* edge of SCK, and so even zero hold from the last falling edge would be safe.
    /// Note that this is a minimum hold time guaranteed by the QMI: the actual chip select active hold may be slightly longer for read transfers with low clock divisors and/or high sample delays. Specifically, if the point two cycles after the last RX data sample is later than the last SCK falling edge, then the hold time is measured from *this* point.
    /// Note also that, in case the final SCK pulse is masked to save energy (true for non-DTR reads when COOLDOWN is disabled or PAGE_BREAK is reached), all of QMI's timing logic behaves as though the clock pulse were still present. The SELECT_HOLD time is applied from the point where the last SCK falling edge would be if the clock pulse were not masked.
    /// Bits `[24:23]`, mask `0x01800000`
    pub const SELECT_HOLD_OFFSET: u32 = 23;
    pub const SELECT_HOLD_MASK: u32 = 0x01800000;

    /// **MAX_SELECT**
    /// Enforce a maximum assertion duration for this window's chip select, in units of 64 system clock cycles. If 0, the QMI is permitted to keep the chip select asserted indefinitely when servicing sequential memory accesses (see COOLDOWN).
    /// This feature is required to meet timing constraints of PSRAM devices, which specify a maximum chip select assertion so they can perform DRAM refresh cycles. See also MIN_DESELECT, which can enforce a minimum deselect time.
    /// If a memory access is in progress at the time MAX_SELECT is reached, the QMI will wait for the access to complete before deasserting the chip select. This additional time must be accounted for to calculate a safe MAX_SELECT value. In the worst case, this may be a fully-formed serial transfer, including command prefix and address, with a data payload as large as one cache line.
    /// Bits `[22:17]`, mask `0x007e0000`
    pub const MAX_SELECT_OFFSET: u32 = 17;
    pub const MAX_SELECT_MASK: u32 = 0x007e0000;

    /// **MIN_DESELECT**
    /// After this window's chip select is deasserted, it remains deasserted for half an SCK cycle (rounded up to an integer number of system clock cycles), plus MIN_DESELECT additional system clock cycles, before the QMI reasserts either chip select pin.
    /// Nonzero values may be required for PSRAM devices which enforce a longer minimum CS deselect time, so that they can perform internal DRAM refresh cycles whilst deselected.
    /// Bits `[16:12]`, mask `0x0001f000`
    pub const MIN_DESELECT_OFFSET: u32 = 12;
    pub const MIN_DESELECT_MASK: u32 = 0x0001f000;

    /// **RXDELAY**
    /// Delay the read data sample timing, in units of one half of a system clock cycle. (Not necessarily half of an SCK cycle.) An RXDELAY of 0 means the sample is captured at the SDI input registers simultaneously with the rising edge of SCK launched from the SCK output register.
    /// At higher SCK frequencies, RXDELAY may need to be increased to account for the round trip delay of the pads, and the clock-to-Q delay of the QSPI memory device.
    /// Bits `[10:8]`, mask `0x00000700`
    pub const RXDELAY_OFFSET: u32 = 8;
    pub const RXDELAY_MASK: u32 = 0x00000700;

    /// **CLKDIV**
    /// Clock divisor. Odd and even divisors are supported. Defines the SCK clock period in units of 1 system clock cycle. Divisors 1..255 are encoded directly, and a divisor of 256 is encoded with a value of CLKDIV=0.
    /// The clock divisor can be changed on-the-fly, even when the QMI is currently accessing memory in this address window. All other parameters must only be changed when the QMI is idle.
    /// If software is increasing CLKDIV in anticipation of an increase in the system clock frequency, a dummy access to either memory window (and appropriate processor barriers/fences) must be inserted after the Mx_TIMING write to ensure the SCK divisor change is in effect _before_ the system clock is changed.
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const CLKDIV_OFFSET: u32 = 0;
    pub const CLKDIV_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **COOLDOWN** `[31:30]` (RW)
    /// Chip select cooldown period. When a memory transfer finishes, the chip select remains asserted for 64 x COOLDOWN system clock cycles, plus half an SCK clock period (rounded up for odd SCK divisors). After this cooldown expires, the chip select is always deasserted to save power.
    /// If the next memory access arrives within the cooldown period, the QMI may be able to append more SCK cycles to the currently ongoing SPI transfer, rather than starting a new transfer. This reduces access latency and increases bus throughput.
    /// Specifically, the next access must be in the same direction (read/write), access the same memory window (chip select 0/1), and follow sequentially the address of the last transfer. If any of these are false, the new access will first deassert the chip select, then begin a new transfer.
    /// If COOLDOWN is 0, the address alignment configured by PAGEBREAK has been reached, or the total chip select assertion limit MAX_SELECT has been reached, the cooldown period is skipped, and the chip select will always be deasserted one half SCK period after the transfer finishes.
    #[inline(always)]
    pub fn cooldown(&self) -> u32 {
        (self.value.read_bits() & Self::COOLDOWN_MASK) >> Self::COOLDOWN_OFFSET
    }

    /// Write **COOLDOWN** `[31:30]` (RW)
    /// Chip select cooldown period. When a memory transfer finishes, the chip select remains asserted for 64 x COOLDOWN system clock cycles, plus half an SCK clock period (rounded up for odd SCK divisors). After this cooldown expires, the chip select is always deasserted to save power.
    /// If the next memory access arrives within the cooldown period, the QMI may be able to append more SCK cycles to the currently ongoing SPI transfer, rather than starting a new transfer. This reduces access latency and increases bus throughput.
    /// Specifically, the next access must be in the same direction (read/write), access the same memory window (chip select 0/1), and follow sequentially the address of the last transfer. If any of these are false, the new access will first deassert the chip select, then begin a new transfer.
    /// If COOLDOWN is 0, the address alignment configured by PAGEBREAK has been reached, or the total chip select assertion limit MAX_SELECT has been reached, the cooldown period is skipped, and the chip select will always be deasserted one half SCK period after the transfer finishes.
    #[inline(always)]
    pub fn write_cooldown(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::COOLDOWN_MASK) | ((value << Self::COOLDOWN_OFFSET) & Self::COOLDOWN_MASK));
    }

    /// **PAGEBREAK** `[29:28]` (RW)
    /// When page break is enabled, chip select will automatically deassert when crossing certain power-of-2-aligned address boundaries. The next access will always begin a new read/write SPI burst, even if the address of the next access follows in sequence with the last access before the page boundary.
    /// Some flash and PSRAM devices forbid crossing page boundaries with a single read/write transfer, or restrict the operating frequency for transfers that do cross page a boundary. This option allows the QMI to safely support those devices.
    /// This field has no effect when COOLDOWN is disabled.
    #[inline(always)]
    pub fn pagebreak(&self) -> u32 {
        (self.value.read_bits() & Self::PAGEBREAK_MASK) >> Self::PAGEBREAK_OFFSET
    }

    /// Write **PAGEBREAK** `[29:28]` (RW)
    /// When page break is enabled, chip select will automatically deassert when crossing certain power-of-2-aligned address boundaries. The next access will always begin a new read/write SPI burst, even if the address of the next access follows in sequence with the last access before the page boundary.
    /// Some flash and PSRAM devices forbid crossing page boundaries with a single read/write transfer, or restrict the operating frequency for transfers that do cross page a boundary. This option allows the QMI to safely support those devices.
    /// This field has no effect when COOLDOWN is disabled.
    #[inline(always)]
    pub fn write_pagebreak(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::PAGEBREAK_MASK) | ((value << Self::PAGEBREAK_OFFSET) & Self::PAGEBREAK_MASK));
    }

    /// **SELECT_SETUP** `[25]` (RW)
    /// Add up to one additional system clock cycle of setup between chip select assertion and the first rising edge of SCK.
    /// The default setup time is one half SCK period, which is usually sufficient except for very high SCK frequencies with some flash devices.
    #[inline(always)]
    pub fn select_setup(&self) -> bool {
        (self.value.read_bits() & Self::SELECT_SETUP_MASK) != 0
    }

    /// Set SELECT_SETUP bit atomically (RW)
    #[inline(always)]
    pub fn set_select_setup(&self) {
        self.value.set_bits(Self::SELECT_SETUP_MASK);
    }

    /// Clear SELECT_SETUP bit atomically (RW)
    #[inline(always)]
    pub fn clr_select_setup(&self) {
        self.value.clr_bits(Self::SELECT_SETUP_MASK);
    }

    /// **SELECT_HOLD** `[24:23]` (RW)
    /// Add up to three additional system clock cycles of active hold between the last falling edge of SCK and the deassertion of this window's chip select.
    /// The default hold time is one system clock cycle. Note that flash datasheets usually give chip select active hold time from the last *rising* edge of SCK, and so even zero hold from the last falling edge would be safe.
    /// Note that this is a minimum hold time guaranteed by the QMI: the actual chip select active hold may be slightly longer for read transfers with low clock divisors and/or high sample delays. Specifically, if the point two cycles after the last RX data sample is later than the last SCK falling edge, then the hold time is measured from *this* point.
    /// Note also that, in case the final SCK pulse is masked to save energy (true for non-DTR reads when COOLDOWN is disabled or PAGE_BREAK is reached), all of QMI's timing logic behaves as though the clock pulse were still present. The SELECT_HOLD time is applied from the point where the last SCK falling edge would be if the clock pulse were not masked.
    #[inline(always)]
    pub fn select_hold(&self) -> u32 {
        (self.value.read_bits() & Self::SELECT_HOLD_MASK) >> Self::SELECT_HOLD_OFFSET
    }

    /// Write **SELECT_HOLD** `[24:23]` (RW)
    /// Add up to three additional system clock cycles of active hold between the last falling edge of SCK and the deassertion of this window's chip select.
    /// The default hold time is one system clock cycle. Note that flash datasheets usually give chip select active hold time from the last *rising* edge of SCK, and so even zero hold from the last falling edge would be safe.
    /// Note that this is a minimum hold time guaranteed by the QMI: the actual chip select active hold may be slightly longer for read transfers with low clock divisors and/or high sample delays. Specifically, if the point two cycles after the last RX data sample is later than the last SCK falling edge, then the hold time is measured from *this* point.
    /// Note also that, in case the final SCK pulse is masked to save energy (true for non-DTR reads when COOLDOWN is disabled or PAGE_BREAK is reached), all of QMI's timing logic behaves as though the clock pulse were still present. The SELECT_HOLD time is applied from the point where the last SCK falling edge would be if the clock pulse were not masked.
    #[inline(always)]
    pub fn write_select_hold(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SELECT_HOLD_MASK) | ((value << Self::SELECT_HOLD_OFFSET) & Self::SELECT_HOLD_MASK));
    }

    /// **MAX_SELECT** `[22:17]` (RW)
    /// Enforce a maximum assertion duration for this window's chip select, in units of 64 system clock cycles. If 0, the QMI is permitted to keep the chip select asserted indefinitely when servicing sequential memory accesses (see COOLDOWN).
    /// This feature is required to meet timing constraints of PSRAM devices, which specify a maximum chip select assertion so they can perform DRAM refresh cycles. See also MIN_DESELECT, which can enforce a minimum deselect time.
    /// If a memory access is in progress at the time MAX_SELECT is reached, the QMI will wait for the access to complete before deasserting the chip select. This additional time must be accounted for to calculate a safe MAX_SELECT value. In the worst case, this may be a fully-formed serial transfer, including command prefix and address, with a data payload as large as one cache line.
    #[inline(always)]
    pub fn max_select(&self) -> u32 {
        (self.value.read_bits() & Self::MAX_SELECT_MASK) >> Self::MAX_SELECT_OFFSET
    }

    /// Write **MAX_SELECT** `[22:17]` (RW)
    /// Enforce a maximum assertion duration for this window's chip select, in units of 64 system clock cycles. If 0, the QMI is permitted to keep the chip select asserted indefinitely when servicing sequential memory accesses (see COOLDOWN).
    /// This feature is required to meet timing constraints of PSRAM devices, which specify a maximum chip select assertion so they can perform DRAM refresh cycles. See also MIN_DESELECT, which can enforce a minimum deselect time.
    /// If a memory access is in progress at the time MAX_SELECT is reached, the QMI will wait for the access to complete before deasserting the chip select. This additional time must be accounted for to calculate a safe MAX_SELECT value. In the worst case, this may be a fully-formed serial transfer, including command prefix and address, with a data payload as large as one cache line.
    #[inline(always)]
    pub fn write_max_select(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::MAX_SELECT_MASK) | ((value << Self::MAX_SELECT_OFFSET) & Self::MAX_SELECT_MASK));
    }

    /// **MIN_DESELECT** `[16:12]` (RW)
    /// After this window's chip select is deasserted, it remains deasserted for half an SCK cycle (rounded up to an integer number of system clock cycles), plus MIN_DESELECT additional system clock cycles, before the QMI reasserts either chip select pin.
    /// Nonzero values may be required for PSRAM devices which enforce a longer minimum CS deselect time, so that they can perform internal DRAM refresh cycles whilst deselected.
    #[inline(always)]
    pub fn min_deselect(&self) -> u32 {
        (self.value.read_bits() & Self::MIN_DESELECT_MASK) >> Self::MIN_DESELECT_OFFSET
    }

    /// Write **MIN_DESELECT** `[16:12]` (RW)
    /// After this window's chip select is deasserted, it remains deasserted for half an SCK cycle (rounded up to an integer number of system clock cycles), plus MIN_DESELECT additional system clock cycles, before the QMI reasserts either chip select pin.
    /// Nonzero values may be required for PSRAM devices which enforce a longer minimum CS deselect time, so that they can perform internal DRAM refresh cycles whilst deselected.
    #[inline(always)]
    pub fn write_min_deselect(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::MIN_DESELECT_MASK) | ((value << Self::MIN_DESELECT_OFFSET) & Self::MIN_DESELECT_MASK));
    }

    /// **RXDELAY** `[10:8]` (RW)
    /// Delay the read data sample timing, in units of one half of a system clock cycle. (Not necessarily half of an SCK cycle.) An RXDELAY of 0 means the sample is captured at the SDI input registers simultaneously with the rising edge of SCK launched from the SCK output register.
    /// At higher SCK frequencies, RXDELAY may need to be increased to account for the round trip delay of the pads, and the clock-to-Q delay of the QSPI memory device.
    #[inline(always)]
    pub fn rxdelay(&self) -> u32 {
        (self.value.read_bits() & Self::RXDELAY_MASK) >> Self::RXDELAY_OFFSET
    }

    /// Write **RXDELAY** `[10:8]` (RW)
    /// Delay the read data sample timing, in units of one half of a system clock cycle. (Not necessarily half of an SCK cycle.) An RXDELAY of 0 means the sample is captured at the SDI input registers simultaneously with the rising edge of SCK launched from the SCK output register.
    /// At higher SCK frequencies, RXDELAY may need to be increased to account for the round trip delay of the pads, and the clock-to-Q delay of the QSPI memory device.
    #[inline(always)]
    pub fn write_rxdelay(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::RXDELAY_MASK) | ((value << Self::RXDELAY_OFFSET) & Self::RXDELAY_MASK));
    }

    /// **CLKDIV** `[7:0]` (RW)
    /// Clock divisor. Odd and even divisors are supported. Defines the SCK clock period in units of 1 system clock cycle. Divisors 1..255 are encoded directly, and a divisor of 256 is encoded with a value of CLKDIV=0.
    /// The clock divisor can be changed on-the-fly, even when the QMI is currently accessing memory in this address window. All other parameters must only be changed when the QMI is idle.
    /// If software is increasing CLKDIV in anticipation of an increase in the system clock frequency, a dummy access to either memory window (and appropriate processor barriers/fences) must be inserted after the Mx_TIMING write to ensure the SCK divisor change is in effect _before_ the system clock is changed.
    #[inline(always)]
    pub fn clkdiv(&self) -> u32 {
        (self.value.read_bits() & Self::CLKDIV_MASK) >> Self::CLKDIV_OFFSET
    }

    /// Write **CLKDIV** `[7:0]` (RW)
    /// Clock divisor. Odd and even divisors are supported. Defines the SCK clock period in units of 1 system clock cycle. Divisors 1..255 are encoded directly, and a divisor of 256 is encoded with a value of CLKDIV=0.
    /// The clock divisor can be changed on-the-fly, even when the QMI is currently accessing memory in this address window. All other parameters must only be changed when the QMI is idle.
    /// If software is increasing CLKDIV in anticipation of an increase in the system clock frequency, a dummy access to either memory window (and appropriate processor barriers/fences) must be inserted after the Mx_TIMING write to ensure the SCK divisor change is in effect _before_ the system clock is changed.
    #[inline(always)]
    pub fn write_clkdiv(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::CLKDIV_MASK) | ((value << Self::CLKDIV_OFFSET) & Self::CLKDIV_MASK));
    }

}

/// M0_TIMING value type for batch read/modify/write operations
///
/// Use `m0timing.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `m0timing.write_val(val)`.
#[derive(Clone, Copy)]
pub struct M0TimingVal {
    bits: u32,
}

impl M0TimingVal {
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
        Self { bits: 0x40000004 }
    }

    /// Read **COOLDOWN** `[31:30]`
    /// Chip select cooldown period. When a memory transfer finishes, the chip select remains asserted for 64 x COOLDOWN system clock cycles, plus half an SCK clock period (rounded up for odd SCK divisors). After this cooldown expires, the chip select is always deasserted to save power.
    /// If the next memory access arrives within the cooldown period, the QMI may be able to append more SCK cycles to the currently ongoing SPI transfer, rather than starting a new transfer. This reduces access latency and increases bus throughput.
    /// Specifically, the next access must be in the same direction (read/write), access the same memory window (chip select 0/1), and follow sequentially the address of the last transfer. If any of these are false, the new access will first deassert the chip select, then begin a new transfer.
    /// If COOLDOWN is 0, the address alignment configured by PAGEBREAK has been reached, or the total chip select assertion limit MAX_SELECT has been reached, the cooldown period is skipped, and the chip select will always be deasserted one half SCK period after the transfer finishes.
    #[inline(always)]
    pub const fn cooldown(&self) -> u32 {
        (self.bits & M0Timing::COOLDOWN_MASK) >> M0Timing::COOLDOWN_OFFSET
    }

    /// Read **PAGEBREAK** `[29:28]`
    /// When page break is enabled, chip select will automatically deassert when crossing certain power-of-2-aligned address boundaries. The next access will always begin a new read/write SPI burst, even if the address of the next access follows in sequence with the last access before the page boundary.
    /// Some flash and PSRAM devices forbid crossing page boundaries with a single read/write transfer, or restrict the operating frequency for transfers that do cross page a boundary. This option allows the QMI to safely support those devices.
    /// This field has no effect when COOLDOWN is disabled.
    #[inline(always)]
    pub const fn pagebreak(&self) -> u32 {
        (self.bits & M0Timing::PAGEBREAK_MASK) >> M0Timing::PAGEBREAK_OFFSET
    }

    /// Read **SELECT_SETUP** `[25:25]`
    /// Add up to one additional system clock cycle of setup between chip select assertion and the first rising edge of SCK.
    /// The default setup time is one half SCK period, which is usually sufficient except for very high SCK frequencies with some flash devices.
    #[inline(always)]
    pub const fn select_setup(&self) -> bool {
        (self.bits & M0Timing::SELECT_SETUP_MASK) != 0
    }

    /// Read **SELECT_HOLD** `[24:23]`
    /// Add up to three additional system clock cycles of active hold between the last falling edge of SCK and the deassertion of this window's chip select.
    /// The default hold time is one system clock cycle. Note that flash datasheets usually give chip select active hold time from the last *rising* edge of SCK, and so even zero hold from the last falling edge would be safe.
    /// Note that this is a minimum hold time guaranteed by the QMI: the actual chip select active hold may be slightly longer for read transfers with low clock divisors and/or high sample delays. Specifically, if the point two cycles after the last RX data sample is later than the last SCK falling edge, then the hold time is measured from *this* point.
    /// Note also that, in case the final SCK pulse is masked to save energy (true for non-DTR reads when COOLDOWN is disabled or PAGE_BREAK is reached), all of QMI's timing logic behaves as though the clock pulse were still present. The SELECT_HOLD time is applied from the point where the last SCK falling edge would be if the clock pulse were not masked.
    #[inline(always)]
    pub const fn select_hold(&self) -> u32 {
        (self.bits & M0Timing::SELECT_HOLD_MASK) >> M0Timing::SELECT_HOLD_OFFSET
    }

    /// Read **MAX_SELECT** `[22:17]`
    /// Enforce a maximum assertion duration for this window's chip select, in units of 64 system clock cycles. If 0, the QMI is permitted to keep the chip select asserted indefinitely when servicing sequential memory accesses (see COOLDOWN).
    /// This feature is required to meet timing constraints of PSRAM devices, which specify a maximum chip select assertion so they can perform DRAM refresh cycles. See also MIN_DESELECT, which can enforce a minimum deselect time.
    /// If a memory access is in progress at the time MAX_SELECT is reached, the QMI will wait for the access to complete before deasserting the chip select. This additional time must be accounted for to calculate a safe MAX_SELECT value. In the worst case, this may be a fully-formed serial transfer, including command prefix and address, with a data payload as large as one cache line.
    #[inline(always)]
    pub const fn max_select(&self) -> u32 {
        (self.bits & M0Timing::MAX_SELECT_MASK) >> M0Timing::MAX_SELECT_OFFSET
    }

    /// Read **MIN_DESELECT** `[16:12]`
    /// After this window's chip select is deasserted, it remains deasserted for half an SCK cycle (rounded up to an integer number of system clock cycles), plus MIN_DESELECT additional system clock cycles, before the QMI reasserts either chip select pin.
    /// Nonzero values may be required for PSRAM devices which enforce a longer minimum CS deselect time, so that they can perform internal DRAM refresh cycles whilst deselected.
    #[inline(always)]
    pub const fn min_deselect(&self) -> u32 {
        (self.bits & M0Timing::MIN_DESELECT_MASK) >> M0Timing::MIN_DESELECT_OFFSET
    }

    /// Read **RXDELAY** `[10:8]`
    /// Delay the read data sample timing, in units of one half of a system clock cycle. (Not necessarily half of an SCK cycle.) An RXDELAY of 0 means the sample is captured at the SDI input registers simultaneously with the rising edge of SCK launched from the SCK output register.
    /// At higher SCK frequencies, RXDELAY may need to be increased to account for the round trip delay of the pads, and the clock-to-Q delay of the QSPI memory device.
    #[inline(always)]
    pub const fn rxdelay(&self) -> u32 {
        (self.bits & M0Timing::RXDELAY_MASK) >> M0Timing::RXDELAY_OFFSET
    }

    /// Read **CLKDIV** `[7:0]`
    /// Clock divisor. Odd and even divisors are supported. Defines the SCK clock period in units of 1 system clock cycle. Divisors 1..255 are encoded directly, and a divisor of 256 is encoded with a value of CLKDIV=0.
    /// The clock divisor can be changed on-the-fly, even when the QMI is currently accessing memory in this address window. All other parameters must only be changed when the QMI is idle.
    /// If software is increasing CLKDIV in anticipation of an increase in the system clock frequency, a dummy access to either memory window (and appropriate processor barriers/fences) must be inserted after the Mx_TIMING write to ensure the SCK divisor change is in effect _before_ the system clock is changed.
    #[inline(always)]
    pub const fn clkdiv(&self) -> u32 {
        (self.bits & M0Timing::CLKDIV_MASK) >> M0Timing::CLKDIV_OFFSET
    }

    /// Set **COOLDOWN** `[31:30]`
    /// Chip select cooldown period. When a memory transfer finishes, the chip select remains asserted for 64 x COOLDOWN system clock cycles, plus half an SCK clock period (rounded up for odd SCK divisors). After this cooldown expires, the chip select is always deasserted to save power.
    /// If the next memory access arrives within the cooldown period, the QMI may be able to append more SCK cycles to the currently ongoing SPI transfer, rather than starting a new transfer. This reduces access latency and increases bus throughput.
    /// Specifically, the next access must be in the same direction (read/write), access the same memory window (chip select 0/1), and follow sequentially the address of the last transfer. If any of these are false, the new access will first deassert the chip select, then begin a new transfer.
    /// If COOLDOWN is 0, the address alignment configured by PAGEBREAK has been reached, or the total chip select assertion limit MAX_SELECT has been reached, the cooldown period is skipped, and the chip select will always be deasserted one half SCK period after the transfer finishes.
    #[inline(always)]
    pub const fn with_cooldown(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !M0Timing::COOLDOWN_MASK) | ((value << M0Timing::COOLDOWN_OFFSET) & M0Timing::COOLDOWN_MASK)
        }
    }

    /// Set **PAGEBREAK** `[29:28]`
    /// When page break is enabled, chip select will automatically deassert when crossing certain power-of-2-aligned address boundaries. The next access will always begin a new read/write SPI burst, even if the address of the next access follows in sequence with the last access before the page boundary.
    /// Some flash and PSRAM devices forbid crossing page boundaries with a single read/write transfer, or restrict the operating frequency for transfers that do cross page a boundary. This option allows the QMI to safely support those devices.
    /// This field has no effect when COOLDOWN is disabled.
    #[inline(always)]
    pub const fn with_pagebreak(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !M0Timing::PAGEBREAK_MASK) | ((value << M0Timing::PAGEBREAK_OFFSET) & M0Timing::PAGEBREAK_MASK)
        }
    }

    /// Set **SELECT_SETUP** `[25:25]`
    /// Add up to one additional system clock cycle of setup between chip select assertion and the first rising edge of SCK.
    /// The default setup time is one half SCK period, which is usually sufficient except for very high SCK frequencies with some flash devices.
    #[inline(always)]
    pub const fn with_select_setup(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | M0Timing::SELECT_SETUP_MASK
            } else {
                self.bits & !M0Timing::SELECT_SETUP_MASK
            }
        }
    }

    /// Set **SELECT_HOLD** `[24:23]`
    /// Add up to three additional system clock cycles of active hold between the last falling edge of SCK and the deassertion of this window's chip select.
    /// The default hold time is one system clock cycle. Note that flash datasheets usually give chip select active hold time from the last *rising* edge of SCK, and so even zero hold from the last falling edge would be safe.
    /// Note that this is a minimum hold time guaranteed by the QMI: the actual chip select active hold may be slightly longer for read transfers with low clock divisors and/or high sample delays. Specifically, if the point two cycles after the last RX data sample is later than the last SCK falling edge, then the hold time is measured from *this* point.
    /// Note also that, in case the final SCK pulse is masked to save energy (true for non-DTR reads when COOLDOWN is disabled or PAGE_BREAK is reached), all of QMI's timing logic behaves as though the clock pulse were still present. The SELECT_HOLD time is applied from the point where the last SCK falling edge would be if the clock pulse were not masked.
    #[inline(always)]
    pub const fn with_select_hold(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !M0Timing::SELECT_HOLD_MASK) | ((value << M0Timing::SELECT_HOLD_OFFSET) & M0Timing::SELECT_HOLD_MASK)
        }
    }

    /// Set **MAX_SELECT** `[22:17]`
    /// Enforce a maximum assertion duration for this window's chip select, in units of 64 system clock cycles. If 0, the QMI is permitted to keep the chip select asserted indefinitely when servicing sequential memory accesses (see COOLDOWN).
    /// This feature is required to meet timing constraints of PSRAM devices, which specify a maximum chip select assertion so they can perform DRAM refresh cycles. See also MIN_DESELECT, which can enforce a minimum deselect time.
    /// If a memory access is in progress at the time MAX_SELECT is reached, the QMI will wait for the access to complete before deasserting the chip select. This additional time must be accounted for to calculate a safe MAX_SELECT value. In the worst case, this may be a fully-formed serial transfer, including command prefix and address, with a data payload as large as one cache line.
    #[inline(always)]
    pub const fn with_max_select(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !M0Timing::MAX_SELECT_MASK) | ((value << M0Timing::MAX_SELECT_OFFSET) & M0Timing::MAX_SELECT_MASK)
        }
    }

    /// Set **MIN_DESELECT** `[16:12]`
    /// After this window's chip select is deasserted, it remains deasserted for half an SCK cycle (rounded up to an integer number of system clock cycles), plus MIN_DESELECT additional system clock cycles, before the QMI reasserts either chip select pin.
    /// Nonzero values may be required for PSRAM devices which enforce a longer minimum CS deselect time, so that they can perform internal DRAM refresh cycles whilst deselected.
    #[inline(always)]
    pub const fn with_min_deselect(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !M0Timing::MIN_DESELECT_MASK) | ((value << M0Timing::MIN_DESELECT_OFFSET) & M0Timing::MIN_DESELECT_MASK)
        }
    }

    /// Set **RXDELAY** `[10:8]`
    /// Delay the read data sample timing, in units of one half of a system clock cycle. (Not necessarily half of an SCK cycle.) An RXDELAY of 0 means the sample is captured at the SDI input registers simultaneously with the rising edge of SCK launched from the SCK output register.
    /// At higher SCK frequencies, RXDELAY may need to be increased to account for the round trip delay of the pads, and the clock-to-Q delay of the QSPI memory device.
    #[inline(always)]
    pub const fn with_rxdelay(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !M0Timing::RXDELAY_MASK) | ((value << M0Timing::RXDELAY_OFFSET) & M0Timing::RXDELAY_MASK)
        }
    }

    /// Set **CLKDIV** `[7:0]`
    /// Clock divisor. Odd and even divisors are supported. Defines the SCK clock period in units of 1 system clock cycle. Divisors 1..255 are encoded directly, and a divisor of 256 is encoded with a value of CLKDIV=0.
    /// The clock divisor can be changed on-the-fly, even when the QMI is currently accessing memory in this address window. All other parameters must only be changed when the QMI is idle.
    /// If software is increasing CLKDIV in anticipation of an increase in the system clock frequency, a dummy access to either memory window (and appropriate processor barriers/fences) must be inserted after the Mx_TIMING write to ensure the SCK divisor change is in effect _before_ the system clock is changed.
    #[inline(always)]
    pub const fn with_clkdiv(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !M0Timing::CLKDIV_MASK) | ((value << M0Timing::CLKDIV_OFFSET) & M0Timing::CLKDIV_MASK)
        }
    }

}

impl Default for M0TimingVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl M0Timing {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> M0TimingVal {
        M0TimingVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: M0TimingVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(M0TimingVal) -> M0TimingVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// M0_RFMT Register
///
/// Read transfer format configuration for memory address window 0.
/// Configure the bus width of each transfer phase individually, and configure the length or presence of the command prefix, command suffix and dummy/turnaround transfer phases. Only 24-bit addresses are supported.
/// The reset value of the M0_RFMT register is configured to support a basic 03h serial read transfer with no additional configuration.
///
/// Address offset: 0x0010
/// Reset value: 0x00001000
///
/// ## Fields
///
/// - **DTR** `[28]` (RW)
///   Enable double transfer rate (DTR) for read commands: address, suffix and read data phases are active on both edges of SCK. SDO data is launched centre-aligned on each SCK edge, and SDI data is captured on the SCK edge that follows its launch.
///   DTR is implemented by halving the clock rate; SCK has a period of 2 x CLK_DIV throughout the transfer. The prefix and dummy phases are still single transfer rate.
///   If the suffix is quad-width, it must be 0 or 8 bits in length, to ensure an even number of SCK edges.
/// - **DUMMY_LEN** `[18:16]` (RW)
///   Length of dummy phase between command suffix and data phase, in units of 4 bits. (i.e. 1 cycle for quad width, 2 for dual, 4 for single)
/// - **SUFFIX_LEN** `[15:14]` (RW)
///   Length of post-address command suffix, in units of 4 bits. (i.e. 1 cycle for quad width, 2 for dual, 4 for single)
///   Only values of 0 and 8 bits are supported.
/// - **PREFIX_LEN** `[12]` (RW)
///   Length of command prefix, in units of 8 bits. (i.e. 2 cycles for quad width, 4 for dual, 8 for single)
/// - **DATA_WIDTH** `[9:8]` (RW)
///   The width used for the data transfer
/// - **DUMMY_WIDTH** `[7:6]` (RW)
///   The width used for the dummy phase, if any.
///   If width is single, SD0/MOSI is held asserted low during the dummy phase, and SD1...SD3 are tristated. If width is dual/quad, all IOs are tristated during the dummy phase.
/// - **SUFFIX_WIDTH** `[5:4]` (RW)
///   The width used for the post-address command suffix, if any
/// - **ADDR_WIDTH** `[3:2]` (RW)
///   The transfer width used for the address. The address phase always transfers 24 bits in total.
/// - **PREFIX_WIDTH** `[1:0]` (RW)
///   The transfer width used for the command prefix, if any
#[repr(transparent)]
pub struct M0Rfmt {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for M0Rfmt {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl M0Rfmt {
    // ======== Field Constants ========

    /// **DTR**
    /// Enable double transfer rate (DTR) for read commands: address, suffix and read data phases are active on both edges of SCK. SDO data is launched centre-aligned on each SCK edge, and SDI data is captured on the SCK edge that follows its launch.
    /// DTR is implemented by halving the clock rate; SCK has a period of 2 x CLK_DIV throughout the transfer. The prefix and dummy phases are still single transfer rate.
    /// If the suffix is quad-width, it must be 0 or 8 bits in length, to ensure an even number of SCK edges.
    /// Bits `[28:28]`, mask `0x10000000`
    pub const DTR_OFFSET: u32 = 28;
    pub const DTR_MASK: u32 = 0x10000000;
    pub const DTR: u32 = 1 << 28;

    /// **DUMMY_LEN**
    /// Length of dummy phase between command suffix and data phase, in units of 4 bits. (i.e. 1 cycle for quad width, 2 for dual, 4 for single)
    /// Bits `[18:16]`, mask `0x00070000`
    pub const DUMMY_LEN_OFFSET: u32 = 16;
    pub const DUMMY_LEN_MASK: u32 = 0x00070000;
    /// NONE = 0
    /// No dummy phase
    pub const DUMMY_LEN_NONE: u32 = 0 << 16;
    pub const DUMMY_LEN_NONE_VAL: u32 = 0;
    /// 4 = 1
    /// 4 dummy bits
    pub const DUMMY_LEN_4: u32 = 1 << 16;
    pub const DUMMY_LEN_4_VAL: u32 = 1;
    /// 8 = 2
    /// 8 dummy bits
    pub const DUMMY_LEN_8: u32 = 2 << 16;
    pub const DUMMY_LEN_8_VAL: u32 = 2;
    /// 12 = 3
    /// 12 dummy bits
    pub const DUMMY_LEN_12: u32 = 3 << 16;
    pub const DUMMY_LEN_12_VAL: u32 = 3;
    /// 16 = 4
    /// 16 dummy bits
    pub const DUMMY_LEN_16: u32 = 4 << 16;
    pub const DUMMY_LEN_16_VAL: u32 = 4;
    /// 20 = 5
    /// 20 dummy bits
    pub const DUMMY_LEN_20: u32 = 5 << 16;
    pub const DUMMY_LEN_20_VAL: u32 = 5;
    /// 24 = 6
    /// 24 dummy bits
    pub const DUMMY_LEN_24: u32 = 6 << 16;
    pub const DUMMY_LEN_24_VAL: u32 = 6;
    /// 28 = 7
    /// 28 dummy bits
    pub const DUMMY_LEN_28: u32 = 7 << 16;
    pub const DUMMY_LEN_28_VAL: u32 = 7;

    /// **SUFFIX_LEN**
    /// Length of post-address command suffix, in units of 4 bits. (i.e. 1 cycle for quad width, 2 for dual, 4 for single)
    /// Only values of 0 and 8 bits are supported.
    /// Bits `[15:14]`, mask `0x0000c000`
    pub const SUFFIX_LEN_OFFSET: u32 = 14;
    pub const SUFFIX_LEN_MASK: u32 = 0x0000c000;
    /// NONE = 0
    /// No suffix
    pub const SUFFIX_LEN_NONE: u32 = 0 << 14;
    pub const SUFFIX_LEN_NONE_VAL: u32 = 0;
    /// 8 = 2
    /// 8-bit suffix
    pub const SUFFIX_LEN_8: u32 = 2 << 14;
    pub const SUFFIX_LEN_8_VAL: u32 = 2;

    /// **PREFIX_LEN**
    /// Length of command prefix, in units of 8 bits. (i.e. 2 cycles for quad width, 4 for dual, 8 for single)
    /// Bits `[12:12]`, mask `0x00001000`
    pub const PREFIX_LEN_OFFSET: u32 = 12;
    pub const PREFIX_LEN_MASK: u32 = 0x00001000;
    pub const PREFIX_LEN: u32 = 1 << 12;
    /// NONE = 0
    /// No prefix
    pub const PREFIX_LEN_NONE: u32 = 0 << 12;
    pub const PREFIX_LEN_NONE_VAL: u32 = 0;
    /// 8 = 1
    /// 8-bit prefix
    pub const PREFIX_LEN_8: u32 = 1 << 12;
    pub const PREFIX_LEN_8_VAL: u32 = 1;

    /// **DATA_WIDTH**
    /// The width used for the data transfer
    /// Bits `[9:8]`, mask `0x00000300`
    pub const DATA_WIDTH_OFFSET: u32 = 8;
    pub const DATA_WIDTH_MASK: u32 = 0x00000300;
    /// S = 0
    /// Single width
    pub const DATA_WIDTH_S: u32 = 0 << 8;
    pub const DATA_WIDTH_S_VAL: u32 = 0;
    /// D = 1
    /// Dual width
    pub const DATA_WIDTH_D: u32 = 1 << 8;
    pub const DATA_WIDTH_D_VAL: u32 = 1;
    /// Q = 2
    /// Quad width
    pub const DATA_WIDTH_Q: u32 = 2 << 8;
    pub const DATA_WIDTH_Q_VAL: u32 = 2;

    /// **DUMMY_WIDTH**
    /// The width used for the dummy phase, if any.
    /// If width is single, SD0/MOSI is held asserted low during the dummy phase, and SD1...SD3 are tristated. If width is dual/quad, all IOs are tristated during the dummy phase.
    /// Bits `[7:6]`, mask `0x000000c0`
    pub const DUMMY_WIDTH_OFFSET: u32 = 6;
    pub const DUMMY_WIDTH_MASK: u32 = 0x000000c0;
    /// S = 0
    /// Single width
    pub const DUMMY_WIDTH_S: u32 = 0 << 6;
    pub const DUMMY_WIDTH_S_VAL: u32 = 0;
    /// D = 1
    /// Dual width
    pub const DUMMY_WIDTH_D: u32 = 1 << 6;
    pub const DUMMY_WIDTH_D_VAL: u32 = 1;
    /// Q = 2
    /// Quad width
    pub const DUMMY_WIDTH_Q: u32 = 2 << 6;
    pub const DUMMY_WIDTH_Q_VAL: u32 = 2;

    /// **SUFFIX_WIDTH**
    /// The width used for the post-address command suffix, if any
    /// Bits `[5:4]`, mask `0x00000030`
    pub const SUFFIX_WIDTH_OFFSET: u32 = 4;
    pub const SUFFIX_WIDTH_MASK: u32 = 0x00000030;
    /// S = 0
    /// Single width
    pub const SUFFIX_WIDTH_S: u32 = 0 << 4;
    pub const SUFFIX_WIDTH_S_VAL: u32 = 0;
    /// D = 1
    /// Dual width
    pub const SUFFIX_WIDTH_D: u32 = 1 << 4;
    pub const SUFFIX_WIDTH_D_VAL: u32 = 1;
    /// Q = 2
    /// Quad width
    pub const SUFFIX_WIDTH_Q: u32 = 2 << 4;
    pub const SUFFIX_WIDTH_Q_VAL: u32 = 2;

    /// **ADDR_WIDTH**
    /// The transfer width used for the address. The address phase always transfers 24 bits in total.
    /// Bits `[3:2]`, mask `0x0000000c`
    pub const ADDR_WIDTH_OFFSET: u32 = 2;
    pub const ADDR_WIDTH_MASK: u32 = 0x0000000c;
    /// S = 0
    /// Single width
    pub const ADDR_WIDTH_S: u32 = 0 << 2;
    pub const ADDR_WIDTH_S_VAL: u32 = 0;
    /// D = 1
    /// Dual width
    pub const ADDR_WIDTH_D: u32 = 1 << 2;
    pub const ADDR_WIDTH_D_VAL: u32 = 1;
    /// Q = 2
    /// Quad width
    pub const ADDR_WIDTH_Q: u32 = 2 << 2;
    pub const ADDR_WIDTH_Q_VAL: u32 = 2;

    /// **PREFIX_WIDTH**
    /// The transfer width used for the command prefix, if any
    /// Bits `[1:0]`, mask `0x00000003`
    pub const PREFIX_WIDTH_OFFSET: u32 = 0;
    pub const PREFIX_WIDTH_MASK: u32 = 0x00000003;
    /// S = 0
    /// Single width
    pub const PREFIX_WIDTH_S: u32 = 0 << 0;
    pub const PREFIX_WIDTH_S_VAL: u32 = 0;
    /// D = 1
    /// Dual width
    pub const PREFIX_WIDTH_D: u32 = 1 << 0;
    pub const PREFIX_WIDTH_D_VAL: u32 = 1;
    /// Q = 2
    /// Quad width
    pub const PREFIX_WIDTH_Q: u32 = 2 << 0;
    pub const PREFIX_WIDTH_Q_VAL: u32 = 2;

    // ======== Field Accessors ========

    /// **DTR** `[28]` (RW)
    /// Enable double transfer rate (DTR) for read commands: address, suffix and read data phases are active on both edges of SCK. SDO data is launched centre-aligned on each SCK edge, and SDI data is captured on the SCK edge that follows its launch.
    /// DTR is implemented by halving the clock rate; SCK has a period of 2 x CLK_DIV throughout the transfer. The prefix and dummy phases are still single transfer rate.
    /// If the suffix is quad-width, it must be 0 or 8 bits in length, to ensure an even number of SCK edges.
    #[inline(always)]
    pub fn dtr(&self) -> bool {
        (self.value.read_bits() & Self::DTR_MASK) != 0
    }

    /// Set DTR bit atomically (RW)
    #[inline(always)]
    pub fn set_dtr(&self) {
        self.value.set_bits(Self::DTR_MASK);
    }

    /// Clear DTR bit atomically (RW)
    #[inline(always)]
    pub fn clr_dtr(&self) {
        self.value.clr_bits(Self::DTR_MASK);
    }

    /// **DUMMY_LEN** `[18:16]` (RW)
    /// Length of dummy phase between command suffix and data phase, in units of 4 bits. (i.e. 1 cycle for quad width, 2 for dual, 4 for single)
    #[inline(always)]
    pub fn dummy_len(&self) -> u32 {
        (self.value.read_bits() & Self::DUMMY_LEN_MASK) >> Self::DUMMY_LEN_OFFSET
    }

    /// Write **DUMMY_LEN** `[18:16]` (RW)
    /// Length of dummy phase between command suffix and data phase, in units of 4 bits. (i.e. 1 cycle for quad width, 2 for dual, 4 for single)
    #[inline(always)]
    pub fn write_dummy_len(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DUMMY_LEN_MASK) | ((value << Self::DUMMY_LEN_OFFSET) & Self::DUMMY_LEN_MASK));
    }

    /// **SUFFIX_LEN** `[15:14]` (RW)
    /// Length of post-address command suffix, in units of 4 bits. (i.e. 1 cycle for quad width, 2 for dual, 4 for single)
    /// Only values of 0 and 8 bits are supported.
    #[inline(always)]
    pub fn suffix_len(&self) -> u32 {
        (self.value.read_bits() & Self::SUFFIX_LEN_MASK) >> Self::SUFFIX_LEN_OFFSET
    }

    /// Write **SUFFIX_LEN** `[15:14]` (RW)
    /// Length of post-address command suffix, in units of 4 bits. (i.e. 1 cycle for quad width, 2 for dual, 4 for single)
    /// Only values of 0 and 8 bits are supported.
    #[inline(always)]
    pub fn write_suffix_len(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SUFFIX_LEN_MASK) | ((value << Self::SUFFIX_LEN_OFFSET) & Self::SUFFIX_LEN_MASK));
    }

    /// **PREFIX_LEN** `[12]` (RW)
    /// Length of command prefix, in units of 8 bits. (i.e. 2 cycles for quad width, 4 for dual, 8 for single)
    #[inline(always)]
    pub fn prefix_len(&self) -> bool {
        (self.value.read_bits() & Self::PREFIX_LEN_MASK) != 0
    }

    /// Set PREFIX_LEN bit atomically (RW)
    #[inline(always)]
    pub fn set_prefix_len(&self) {
        self.value.set_bits(Self::PREFIX_LEN_MASK);
    }

    /// Clear PREFIX_LEN bit atomically (RW)
    #[inline(always)]
    pub fn clr_prefix_len(&self) {
        self.value.clr_bits(Self::PREFIX_LEN_MASK);
    }

    /// **DATA_WIDTH** `[9:8]` (RW)
    /// The width used for the data transfer
    #[inline(always)]
    pub fn data_width(&self) -> u32 {
        (self.value.read_bits() & Self::DATA_WIDTH_MASK) >> Self::DATA_WIDTH_OFFSET
    }

    /// Write **DATA_WIDTH** `[9:8]` (RW)
    /// The width used for the data transfer
    #[inline(always)]
    pub fn write_data_width(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DATA_WIDTH_MASK) | ((value << Self::DATA_WIDTH_OFFSET) & Self::DATA_WIDTH_MASK));
    }

    /// **DUMMY_WIDTH** `[7:6]` (RW)
    /// The width used for the dummy phase, if any.
    /// If width is single, SD0/MOSI is held asserted low during the dummy phase, and SD1...SD3 are tristated. If width is dual/quad, all IOs are tristated during the dummy phase.
    #[inline(always)]
    pub fn dummy_width(&self) -> u32 {
        (self.value.read_bits() & Self::DUMMY_WIDTH_MASK) >> Self::DUMMY_WIDTH_OFFSET
    }

    /// Write **DUMMY_WIDTH** `[7:6]` (RW)
    /// The width used for the dummy phase, if any.
    /// If width is single, SD0/MOSI is held asserted low during the dummy phase, and SD1...SD3 are tristated. If width is dual/quad, all IOs are tristated during the dummy phase.
    #[inline(always)]
    pub fn write_dummy_width(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DUMMY_WIDTH_MASK) | ((value << Self::DUMMY_WIDTH_OFFSET) & Self::DUMMY_WIDTH_MASK));
    }

    /// **SUFFIX_WIDTH** `[5:4]` (RW)
    /// The width used for the post-address command suffix, if any
    #[inline(always)]
    pub fn suffix_width(&self) -> u32 {
        (self.value.read_bits() & Self::SUFFIX_WIDTH_MASK) >> Self::SUFFIX_WIDTH_OFFSET
    }

    /// Write **SUFFIX_WIDTH** `[5:4]` (RW)
    /// The width used for the post-address command suffix, if any
    #[inline(always)]
    pub fn write_suffix_width(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SUFFIX_WIDTH_MASK) | ((value << Self::SUFFIX_WIDTH_OFFSET) & Self::SUFFIX_WIDTH_MASK));
    }

    /// **ADDR_WIDTH** `[3:2]` (RW)
    /// The transfer width used for the address. The address phase always transfers 24 bits in total.
    #[inline(always)]
    pub fn addr_width(&self) -> u32 {
        (self.value.read_bits() & Self::ADDR_WIDTH_MASK) >> Self::ADDR_WIDTH_OFFSET
    }

    /// Write **ADDR_WIDTH** `[3:2]` (RW)
    /// The transfer width used for the address. The address phase always transfers 24 bits in total.
    #[inline(always)]
    pub fn write_addr_width(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::ADDR_WIDTH_MASK) | ((value << Self::ADDR_WIDTH_OFFSET) & Self::ADDR_WIDTH_MASK));
    }

    /// **PREFIX_WIDTH** `[1:0]` (RW)
    /// The transfer width used for the command prefix, if any
    #[inline(always)]
    pub fn prefix_width(&self) -> u32 {
        (self.value.read_bits() & Self::PREFIX_WIDTH_MASK) >> Self::PREFIX_WIDTH_OFFSET
    }

    /// Write **PREFIX_WIDTH** `[1:0]` (RW)
    /// The transfer width used for the command prefix, if any
    #[inline(always)]
    pub fn write_prefix_width(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::PREFIX_WIDTH_MASK) | ((value << Self::PREFIX_WIDTH_OFFSET) & Self::PREFIX_WIDTH_MASK));
    }

}

/// M0_RFMT value type for batch read/modify/write operations
///
/// Use `m0rfmt.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `m0rfmt.write_val(val)`.
#[derive(Clone, Copy)]
pub struct M0RfmtVal {
    bits: u32,
}

impl M0RfmtVal {
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
        Self { bits: 0x00001000 }
    }

    /// Read **DTR** `[28:28]`
    /// Enable double transfer rate (DTR) for read commands: address, suffix and read data phases are active on both edges of SCK. SDO data is launched centre-aligned on each SCK edge, and SDI data is captured on the SCK edge that follows its launch.
    /// DTR is implemented by halving the clock rate; SCK has a period of 2 x CLK_DIV throughout the transfer. The prefix and dummy phases are still single transfer rate.
    /// If the suffix is quad-width, it must be 0 or 8 bits in length, to ensure an even number of SCK edges.
    #[inline(always)]
    pub const fn dtr(&self) -> bool {
        (self.bits & M0Rfmt::DTR_MASK) != 0
    }

    /// Read **DUMMY_LEN** `[18:16]`
    /// Length of dummy phase between command suffix and data phase, in units of 4 bits. (i.e. 1 cycle for quad width, 2 for dual, 4 for single)
    #[inline(always)]
    pub const fn dummy_len(&self) -> u32 {
        (self.bits & M0Rfmt::DUMMY_LEN_MASK) >> M0Rfmt::DUMMY_LEN_OFFSET
    }

    /// Read **SUFFIX_LEN** `[15:14]`
    /// Length of post-address command suffix, in units of 4 bits. (i.e. 1 cycle for quad width, 2 for dual, 4 for single)
    /// Only values of 0 and 8 bits are supported.
    #[inline(always)]
    pub const fn suffix_len(&self) -> u32 {
        (self.bits & M0Rfmt::SUFFIX_LEN_MASK) >> M0Rfmt::SUFFIX_LEN_OFFSET
    }

    /// Read **PREFIX_LEN** `[12:12]`
    /// Length of command prefix, in units of 8 bits. (i.e. 2 cycles for quad width, 4 for dual, 8 for single)
    #[inline(always)]
    pub const fn prefix_len(&self) -> bool {
        (self.bits & M0Rfmt::PREFIX_LEN_MASK) != 0
    }

    /// Read **DATA_WIDTH** `[9:8]`
    /// The width used for the data transfer
    #[inline(always)]
    pub const fn data_width(&self) -> u32 {
        (self.bits & M0Rfmt::DATA_WIDTH_MASK) >> M0Rfmt::DATA_WIDTH_OFFSET
    }

    /// Read **DUMMY_WIDTH** `[7:6]`
    /// The width used for the dummy phase, if any.
    /// If width is single, SD0/MOSI is held asserted low during the dummy phase, and SD1...SD3 are tristated. If width is dual/quad, all IOs are tristated during the dummy phase.
    #[inline(always)]
    pub const fn dummy_width(&self) -> u32 {
        (self.bits & M0Rfmt::DUMMY_WIDTH_MASK) >> M0Rfmt::DUMMY_WIDTH_OFFSET
    }

    /// Read **SUFFIX_WIDTH** `[5:4]`
    /// The width used for the post-address command suffix, if any
    #[inline(always)]
    pub const fn suffix_width(&self) -> u32 {
        (self.bits & M0Rfmt::SUFFIX_WIDTH_MASK) >> M0Rfmt::SUFFIX_WIDTH_OFFSET
    }

    /// Read **ADDR_WIDTH** `[3:2]`
    /// The transfer width used for the address. The address phase always transfers 24 bits in total.
    #[inline(always)]
    pub const fn addr_width(&self) -> u32 {
        (self.bits & M0Rfmt::ADDR_WIDTH_MASK) >> M0Rfmt::ADDR_WIDTH_OFFSET
    }

    /// Read **PREFIX_WIDTH** `[1:0]`
    /// The transfer width used for the command prefix, if any
    #[inline(always)]
    pub const fn prefix_width(&self) -> u32 {
        (self.bits & M0Rfmt::PREFIX_WIDTH_MASK) >> M0Rfmt::PREFIX_WIDTH_OFFSET
    }

    /// Set **DTR** `[28:28]`
    /// Enable double transfer rate (DTR) for read commands: address, suffix and read data phases are active on both edges of SCK. SDO data is launched centre-aligned on each SCK edge, and SDI data is captured on the SCK edge that follows its launch.
    /// DTR is implemented by halving the clock rate; SCK has a period of 2 x CLK_DIV throughout the transfer. The prefix and dummy phases are still single transfer rate.
    /// If the suffix is quad-width, it must be 0 or 8 bits in length, to ensure an even number of SCK edges.
    #[inline(always)]
    pub const fn with_dtr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | M0Rfmt::DTR_MASK
            } else {
                self.bits & !M0Rfmt::DTR_MASK
            }
        }
    }

    /// Set **DUMMY_LEN** `[18:16]`
    /// Length of dummy phase between command suffix and data phase, in units of 4 bits. (i.e. 1 cycle for quad width, 2 for dual, 4 for single)
    #[inline(always)]
    pub const fn with_dummy_len(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !M0Rfmt::DUMMY_LEN_MASK) | ((value << M0Rfmt::DUMMY_LEN_OFFSET) & M0Rfmt::DUMMY_LEN_MASK)
        }
    }

    /// Set **SUFFIX_LEN** `[15:14]`
    /// Length of post-address command suffix, in units of 4 bits. (i.e. 1 cycle for quad width, 2 for dual, 4 for single)
    /// Only values of 0 and 8 bits are supported.
    #[inline(always)]
    pub const fn with_suffix_len(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !M0Rfmt::SUFFIX_LEN_MASK) | ((value << M0Rfmt::SUFFIX_LEN_OFFSET) & M0Rfmt::SUFFIX_LEN_MASK)
        }
    }

    /// Set **PREFIX_LEN** `[12:12]`
    /// Length of command prefix, in units of 8 bits. (i.e. 2 cycles for quad width, 4 for dual, 8 for single)
    #[inline(always)]
    pub const fn with_prefix_len(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | M0Rfmt::PREFIX_LEN_MASK
            } else {
                self.bits & !M0Rfmt::PREFIX_LEN_MASK
            }
        }
    }

    /// Set **DATA_WIDTH** `[9:8]`
    /// The width used for the data transfer
    #[inline(always)]
    pub const fn with_data_width(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !M0Rfmt::DATA_WIDTH_MASK) | ((value << M0Rfmt::DATA_WIDTH_OFFSET) & M0Rfmt::DATA_WIDTH_MASK)
        }
    }

    /// Set **DUMMY_WIDTH** `[7:6]`
    /// The width used for the dummy phase, if any.
    /// If width is single, SD0/MOSI is held asserted low during the dummy phase, and SD1...SD3 are tristated. If width is dual/quad, all IOs are tristated during the dummy phase.
    #[inline(always)]
    pub const fn with_dummy_width(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !M0Rfmt::DUMMY_WIDTH_MASK) | ((value << M0Rfmt::DUMMY_WIDTH_OFFSET) & M0Rfmt::DUMMY_WIDTH_MASK)
        }
    }

    /// Set **SUFFIX_WIDTH** `[5:4]`
    /// The width used for the post-address command suffix, if any
    #[inline(always)]
    pub const fn with_suffix_width(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !M0Rfmt::SUFFIX_WIDTH_MASK) | ((value << M0Rfmt::SUFFIX_WIDTH_OFFSET) & M0Rfmt::SUFFIX_WIDTH_MASK)
        }
    }

    /// Set **ADDR_WIDTH** `[3:2]`
    /// The transfer width used for the address. The address phase always transfers 24 bits in total.
    #[inline(always)]
    pub const fn with_addr_width(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !M0Rfmt::ADDR_WIDTH_MASK) | ((value << M0Rfmt::ADDR_WIDTH_OFFSET) & M0Rfmt::ADDR_WIDTH_MASK)
        }
    }

    /// Set **PREFIX_WIDTH** `[1:0]`
    /// The transfer width used for the command prefix, if any
    #[inline(always)]
    pub const fn with_prefix_width(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !M0Rfmt::PREFIX_WIDTH_MASK) | ((value << M0Rfmt::PREFIX_WIDTH_OFFSET) & M0Rfmt::PREFIX_WIDTH_MASK)
        }
    }

}

impl Default for M0RfmtVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl M0Rfmt {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> M0RfmtVal {
        M0RfmtVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: M0RfmtVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(M0RfmtVal) -> M0RfmtVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// M0_RCMD Register
///
/// Command constants used for reads from memory address window 0.
/// The reset value of the M0_RCMD register is configured to support a basic 03h serial read transfer with no additional configuration.
///
/// Address offset: 0x0014
/// Reset value: 0x0000a003
///
/// ## Fields
///
/// - **SUFFIX** `[15:8]` (RW)
///   The command suffix bits following the address, if Mx_RFMT_SUFFIX_LEN is nonzero.
/// - **PREFIX** `[7:0]` (RW)
///   The command prefix bits to prepend on each new transfer, if Mx_RFMT_PREFIX_LEN is nonzero.
#[repr(transparent)]
pub struct M0Rcmd {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for M0Rcmd {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl M0Rcmd {
    // ======== Field Constants ========

    /// **SUFFIX**
    /// The command suffix bits following the address, if Mx_RFMT_SUFFIX_LEN is nonzero.
    /// Bits `[15:8]`, mask `0x0000ff00`
    pub const SUFFIX_OFFSET: u32 = 8;
    pub const SUFFIX_MASK: u32 = 0x0000ff00;

    /// **PREFIX**
    /// The command prefix bits to prepend on each new transfer, if Mx_RFMT_PREFIX_LEN is nonzero.
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const PREFIX_OFFSET: u32 = 0;
    pub const PREFIX_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **SUFFIX** `[15:8]` (RW)
    /// The command suffix bits following the address, if Mx_RFMT_SUFFIX_LEN is nonzero.
    #[inline(always)]
    pub fn suffix(&self) -> u32 {
        (self.value.read_bits() & Self::SUFFIX_MASK) >> Self::SUFFIX_OFFSET
    }

    /// Write **SUFFIX** `[15:8]` (RW)
    /// The command suffix bits following the address, if Mx_RFMT_SUFFIX_LEN is nonzero.
    #[inline(always)]
    pub fn write_suffix(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SUFFIX_MASK) | ((value << Self::SUFFIX_OFFSET) & Self::SUFFIX_MASK));
    }

    /// **PREFIX** `[7:0]` (RW)
    /// The command prefix bits to prepend on each new transfer, if Mx_RFMT_PREFIX_LEN is nonzero.
    #[inline(always)]
    pub fn prefix(&self) -> u32 {
        (self.value.read_bits() & Self::PREFIX_MASK) >> Self::PREFIX_OFFSET
    }

    /// Write **PREFIX** `[7:0]` (RW)
    /// The command prefix bits to prepend on each new transfer, if Mx_RFMT_PREFIX_LEN is nonzero.
    #[inline(always)]
    pub fn write_prefix(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::PREFIX_MASK) | ((value << Self::PREFIX_OFFSET) & Self::PREFIX_MASK));
    }

}

/// M0_RCMD value type for batch read/modify/write operations
///
/// Use `m0rcmd.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `m0rcmd.write_val(val)`.
#[derive(Clone, Copy)]
pub struct M0RcmdVal {
    bits: u32,
}

impl M0RcmdVal {
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
        Self { bits: 0x0000a003 }
    }

    /// Read **SUFFIX** `[15:8]`
    /// The command suffix bits following the address, if Mx_RFMT_SUFFIX_LEN is nonzero.
    #[inline(always)]
    pub const fn suffix(&self) -> u32 {
        (self.bits & M0Rcmd::SUFFIX_MASK) >> M0Rcmd::SUFFIX_OFFSET
    }

    /// Read **PREFIX** `[7:0]`
    /// The command prefix bits to prepend on each new transfer, if Mx_RFMT_PREFIX_LEN is nonzero.
    #[inline(always)]
    pub const fn prefix(&self) -> u32 {
        (self.bits & M0Rcmd::PREFIX_MASK) >> M0Rcmd::PREFIX_OFFSET
    }

    /// Set **SUFFIX** `[15:8]`
    /// The command suffix bits following the address, if Mx_RFMT_SUFFIX_LEN is nonzero.
    #[inline(always)]
    pub const fn with_suffix(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !M0Rcmd::SUFFIX_MASK) | ((value << M0Rcmd::SUFFIX_OFFSET) & M0Rcmd::SUFFIX_MASK)
        }
    }

    /// Set **PREFIX** `[7:0]`
    /// The command prefix bits to prepend on each new transfer, if Mx_RFMT_PREFIX_LEN is nonzero.
    #[inline(always)]
    pub const fn with_prefix(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !M0Rcmd::PREFIX_MASK) | ((value << M0Rcmd::PREFIX_OFFSET) & M0Rcmd::PREFIX_MASK)
        }
    }

}

impl Default for M0RcmdVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl M0Rcmd {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> M0RcmdVal {
        M0RcmdVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: M0RcmdVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(M0RcmdVal) -> M0RcmdVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// M0_WFMT Register
///
/// Write transfer format configuration for memory address window 0.
/// Configure the bus width of each transfer phase individually, and configure the length or presence of the command prefix, command suffix and dummy/turnaround transfer phases. Only 24-bit addresses are supported.
/// The reset value of the M0_WFMT register is configured to support a basic 02h serial write transfer. However, writes to this window must first be enabled via the XIP_CTRL_WRITABLE_M0 bit, as XIP memory is read-only by default.
///
/// Address offset: 0x0018
/// Reset value: 0x00001000
///
/// ## Fields
///
/// - **DTR** `[28]` (RW)
///   Enable double transfer rate (DTR) for write commands: address, suffix and write data phases are active on both edges of SCK. SDO data is launched centre-aligned on each SCK edge, and SDI data is captured on the SCK edge that follows its launch.
///   DTR is implemented by halving the clock rate; SCK has a period of 2 x CLK_DIV throughout the transfer. The prefix and dummy phases are still single transfer rate.
///   If the suffix is quad-width, it must be 0 or 8 bits in length, to ensure an even number of SCK edges.
/// - **DUMMY_LEN** `[18:16]` (RW)
///   Length of dummy phase between command suffix and data phase, in units of 4 bits. (i.e. 1 cycle for quad width, 2 for dual, 4 for single)
/// - **SUFFIX_LEN** `[15:14]` (RW)
///   Length of post-address command suffix, in units of 4 bits. (i.e. 1 cycle for quad width, 2 for dual, 4 for single)
///   Only values of 0 and 8 bits are supported.
/// - **PREFIX_LEN** `[12]` (RW)
///   Length of command prefix, in units of 8 bits. (i.e. 2 cycles for quad width, 4 for dual, 8 for single)
/// - **DATA_WIDTH** `[9:8]` (RW)
///   The width used for the data transfer
/// - **DUMMY_WIDTH** `[7:6]` (RW)
///   The width used for the dummy phase, if any.
///   If width is single, SD0/MOSI is held asserted low during the dummy phase, and SD1...SD3 are tristated. If width is dual/quad, all IOs are tristated during the dummy phase.
/// - **SUFFIX_WIDTH** `[5:4]` (RW)
///   The width used for the post-address command suffix, if any
/// - **ADDR_WIDTH** `[3:2]` (RW)
///   The transfer width used for the address. The address phase always transfers 24 bits in total.
/// - **PREFIX_WIDTH** `[1:0]` (RW)
///   The transfer width used for the command prefix, if any
#[repr(transparent)]
pub struct M0Wfmt {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for M0Wfmt {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl M0Wfmt {
    // ======== Field Constants ========

    /// **DTR**
    /// Enable double transfer rate (DTR) for write commands: address, suffix and write data phases are active on both edges of SCK. SDO data is launched centre-aligned on each SCK edge, and SDI data is captured on the SCK edge that follows its launch.
    /// DTR is implemented by halving the clock rate; SCK has a period of 2 x CLK_DIV throughout the transfer. The prefix and dummy phases are still single transfer rate.
    /// If the suffix is quad-width, it must be 0 or 8 bits in length, to ensure an even number of SCK edges.
    /// Bits `[28:28]`, mask `0x10000000`
    pub const DTR_OFFSET: u32 = 28;
    pub const DTR_MASK: u32 = 0x10000000;
    pub const DTR: u32 = 1 << 28;

    /// **DUMMY_LEN**
    /// Length of dummy phase between command suffix and data phase, in units of 4 bits. (i.e. 1 cycle for quad width, 2 for dual, 4 for single)
    /// Bits `[18:16]`, mask `0x00070000`
    pub const DUMMY_LEN_OFFSET: u32 = 16;
    pub const DUMMY_LEN_MASK: u32 = 0x00070000;
    /// NONE = 0
    /// No dummy phase
    pub const DUMMY_LEN_NONE: u32 = 0 << 16;
    pub const DUMMY_LEN_NONE_VAL: u32 = 0;
    /// 4 = 1
    /// 4 dummy bits
    pub const DUMMY_LEN_4: u32 = 1 << 16;
    pub const DUMMY_LEN_4_VAL: u32 = 1;
    /// 8 = 2
    /// 8 dummy bits
    pub const DUMMY_LEN_8: u32 = 2 << 16;
    pub const DUMMY_LEN_8_VAL: u32 = 2;
    /// 12 = 3
    /// 12 dummy bits
    pub const DUMMY_LEN_12: u32 = 3 << 16;
    pub const DUMMY_LEN_12_VAL: u32 = 3;
    /// 16 = 4
    /// 16 dummy bits
    pub const DUMMY_LEN_16: u32 = 4 << 16;
    pub const DUMMY_LEN_16_VAL: u32 = 4;
    /// 20 = 5
    /// 20 dummy bits
    pub const DUMMY_LEN_20: u32 = 5 << 16;
    pub const DUMMY_LEN_20_VAL: u32 = 5;
    /// 24 = 6
    /// 24 dummy bits
    pub const DUMMY_LEN_24: u32 = 6 << 16;
    pub const DUMMY_LEN_24_VAL: u32 = 6;
    /// 28 = 7
    /// 28 dummy bits
    pub const DUMMY_LEN_28: u32 = 7 << 16;
    pub const DUMMY_LEN_28_VAL: u32 = 7;

    /// **SUFFIX_LEN**
    /// Length of post-address command suffix, in units of 4 bits. (i.e. 1 cycle for quad width, 2 for dual, 4 for single)
    /// Only values of 0 and 8 bits are supported.
    /// Bits `[15:14]`, mask `0x0000c000`
    pub const SUFFIX_LEN_OFFSET: u32 = 14;
    pub const SUFFIX_LEN_MASK: u32 = 0x0000c000;
    /// NONE = 0
    /// No suffix
    pub const SUFFIX_LEN_NONE: u32 = 0 << 14;
    pub const SUFFIX_LEN_NONE_VAL: u32 = 0;
    /// 8 = 2
    /// 8-bit suffix
    pub const SUFFIX_LEN_8: u32 = 2 << 14;
    pub const SUFFIX_LEN_8_VAL: u32 = 2;

    /// **PREFIX_LEN**
    /// Length of command prefix, in units of 8 bits. (i.e. 2 cycles for quad width, 4 for dual, 8 for single)
    /// Bits `[12:12]`, mask `0x00001000`
    pub const PREFIX_LEN_OFFSET: u32 = 12;
    pub const PREFIX_LEN_MASK: u32 = 0x00001000;
    pub const PREFIX_LEN: u32 = 1 << 12;
    /// NONE = 0
    /// No prefix
    pub const PREFIX_LEN_NONE: u32 = 0 << 12;
    pub const PREFIX_LEN_NONE_VAL: u32 = 0;
    /// 8 = 1
    /// 8-bit prefix
    pub const PREFIX_LEN_8: u32 = 1 << 12;
    pub const PREFIX_LEN_8_VAL: u32 = 1;

    /// **DATA_WIDTH**
    /// The width used for the data transfer
    /// Bits `[9:8]`, mask `0x00000300`
    pub const DATA_WIDTH_OFFSET: u32 = 8;
    pub const DATA_WIDTH_MASK: u32 = 0x00000300;
    /// S = 0
    /// Single width
    pub const DATA_WIDTH_S: u32 = 0 << 8;
    pub const DATA_WIDTH_S_VAL: u32 = 0;
    /// D = 1
    /// Dual width
    pub const DATA_WIDTH_D: u32 = 1 << 8;
    pub const DATA_WIDTH_D_VAL: u32 = 1;
    /// Q = 2
    /// Quad width
    pub const DATA_WIDTH_Q: u32 = 2 << 8;
    pub const DATA_WIDTH_Q_VAL: u32 = 2;

    /// **DUMMY_WIDTH**
    /// The width used for the dummy phase, if any.
    /// If width is single, SD0/MOSI is held asserted low during the dummy phase, and SD1...SD3 are tristated. If width is dual/quad, all IOs are tristated during the dummy phase.
    /// Bits `[7:6]`, mask `0x000000c0`
    pub const DUMMY_WIDTH_OFFSET: u32 = 6;
    pub const DUMMY_WIDTH_MASK: u32 = 0x000000c0;
    /// S = 0
    /// Single width
    pub const DUMMY_WIDTH_S: u32 = 0 << 6;
    pub const DUMMY_WIDTH_S_VAL: u32 = 0;
    /// D = 1
    /// Dual width
    pub const DUMMY_WIDTH_D: u32 = 1 << 6;
    pub const DUMMY_WIDTH_D_VAL: u32 = 1;
    /// Q = 2
    /// Quad width
    pub const DUMMY_WIDTH_Q: u32 = 2 << 6;
    pub const DUMMY_WIDTH_Q_VAL: u32 = 2;

    /// **SUFFIX_WIDTH**
    /// The width used for the post-address command suffix, if any
    /// Bits `[5:4]`, mask `0x00000030`
    pub const SUFFIX_WIDTH_OFFSET: u32 = 4;
    pub const SUFFIX_WIDTH_MASK: u32 = 0x00000030;
    /// S = 0
    /// Single width
    pub const SUFFIX_WIDTH_S: u32 = 0 << 4;
    pub const SUFFIX_WIDTH_S_VAL: u32 = 0;
    /// D = 1
    /// Dual width
    pub const SUFFIX_WIDTH_D: u32 = 1 << 4;
    pub const SUFFIX_WIDTH_D_VAL: u32 = 1;
    /// Q = 2
    /// Quad width
    pub const SUFFIX_WIDTH_Q: u32 = 2 << 4;
    pub const SUFFIX_WIDTH_Q_VAL: u32 = 2;

    /// **ADDR_WIDTH**
    /// The transfer width used for the address. The address phase always transfers 24 bits in total.
    /// Bits `[3:2]`, mask `0x0000000c`
    pub const ADDR_WIDTH_OFFSET: u32 = 2;
    pub const ADDR_WIDTH_MASK: u32 = 0x0000000c;
    /// S = 0
    /// Single width
    pub const ADDR_WIDTH_S: u32 = 0 << 2;
    pub const ADDR_WIDTH_S_VAL: u32 = 0;
    /// D = 1
    /// Dual width
    pub const ADDR_WIDTH_D: u32 = 1 << 2;
    pub const ADDR_WIDTH_D_VAL: u32 = 1;
    /// Q = 2
    /// Quad width
    pub const ADDR_WIDTH_Q: u32 = 2 << 2;
    pub const ADDR_WIDTH_Q_VAL: u32 = 2;

    /// **PREFIX_WIDTH**
    /// The transfer width used for the command prefix, if any
    /// Bits `[1:0]`, mask `0x00000003`
    pub const PREFIX_WIDTH_OFFSET: u32 = 0;
    pub const PREFIX_WIDTH_MASK: u32 = 0x00000003;
    /// S = 0
    /// Single width
    pub const PREFIX_WIDTH_S: u32 = 0 << 0;
    pub const PREFIX_WIDTH_S_VAL: u32 = 0;
    /// D = 1
    /// Dual width
    pub const PREFIX_WIDTH_D: u32 = 1 << 0;
    pub const PREFIX_WIDTH_D_VAL: u32 = 1;
    /// Q = 2
    /// Quad width
    pub const PREFIX_WIDTH_Q: u32 = 2 << 0;
    pub const PREFIX_WIDTH_Q_VAL: u32 = 2;

    // ======== Field Accessors ========

    /// **DTR** `[28]` (RW)
    /// Enable double transfer rate (DTR) for write commands: address, suffix and write data phases are active on both edges of SCK. SDO data is launched centre-aligned on each SCK edge, and SDI data is captured on the SCK edge that follows its launch.
    /// DTR is implemented by halving the clock rate; SCK has a period of 2 x CLK_DIV throughout the transfer. The prefix and dummy phases are still single transfer rate.
    /// If the suffix is quad-width, it must be 0 or 8 bits in length, to ensure an even number of SCK edges.
    #[inline(always)]
    pub fn dtr(&self) -> bool {
        (self.value.read_bits() & Self::DTR_MASK) != 0
    }

    /// Set DTR bit atomically (RW)
    #[inline(always)]
    pub fn set_dtr(&self) {
        self.value.set_bits(Self::DTR_MASK);
    }

    /// Clear DTR bit atomically (RW)
    #[inline(always)]
    pub fn clr_dtr(&self) {
        self.value.clr_bits(Self::DTR_MASK);
    }

    /// **DUMMY_LEN** `[18:16]` (RW)
    /// Length of dummy phase between command suffix and data phase, in units of 4 bits. (i.e. 1 cycle for quad width, 2 for dual, 4 for single)
    #[inline(always)]
    pub fn dummy_len(&self) -> u32 {
        (self.value.read_bits() & Self::DUMMY_LEN_MASK) >> Self::DUMMY_LEN_OFFSET
    }

    /// Write **DUMMY_LEN** `[18:16]` (RW)
    /// Length of dummy phase between command suffix and data phase, in units of 4 bits. (i.e. 1 cycle for quad width, 2 for dual, 4 for single)
    #[inline(always)]
    pub fn write_dummy_len(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DUMMY_LEN_MASK) | ((value << Self::DUMMY_LEN_OFFSET) & Self::DUMMY_LEN_MASK));
    }

    /// **SUFFIX_LEN** `[15:14]` (RW)
    /// Length of post-address command suffix, in units of 4 bits. (i.e. 1 cycle for quad width, 2 for dual, 4 for single)
    /// Only values of 0 and 8 bits are supported.
    #[inline(always)]
    pub fn suffix_len(&self) -> u32 {
        (self.value.read_bits() & Self::SUFFIX_LEN_MASK) >> Self::SUFFIX_LEN_OFFSET
    }

    /// Write **SUFFIX_LEN** `[15:14]` (RW)
    /// Length of post-address command suffix, in units of 4 bits. (i.e. 1 cycle for quad width, 2 for dual, 4 for single)
    /// Only values of 0 and 8 bits are supported.
    #[inline(always)]
    pub fn write_suffix_len(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SUFFIX_LEN_MASK) | ((value << Self::SUFFIX_LEN_OFFSET) & Self::SUFFIX_LEN_MASK));
    }

    /// **PREFIX_LEN** `[12]` (RW)
    /// Length of command prefix, in units of 8 bits. (i.e. 2 cycles for quad width, 4 for dual, 8 for single)
    #[inline(always)]
    pub fn prefix_len(&self) -> bool {
        (self.value.read_bits() & Self::PREFIX_LEN_MASK) != 0
    }

    /// Set PREFIX_LEN bit atomically (RW)
    #[inline(always)]
    pub fn set_prefix_len(&self) {
        self.value.set_bits(Self::PREFIX_LEN_MASK);
    }

    /// Clear PREFIX_LEN bit atomically (RW)
    #[inline(always)]
    pub fn clr_prefix_len(&self) {
        self.value.clr_bits(Self::PREFIX_LEN_MASK);
    }

    /// **DATA_WIDTH** `[9:8]` (RW)
    /// The width used for the data transfer
    #[inline(always)]
    pub fn data_width(&self) -> u32 {
        (self.value.read_bits() & Self::DATA_WIDTH_MASK) >> Self::DATA_WIDTH_OFFSET
    }

    /// Write **DATA_WIDTH** `[9:8]` (RW)
    /// The width used for the data transfer
    #[inline(always)]
    pub fn write_data_width(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DATA_WIDTH_MASK) | ((value << Self::DATA_WIDTH_OFFSET) & Self::DATA_WIDTH_MASK));
    }

    /// **DUMMY_WIDTH** `[7:6]` (RW)
    /// The width used for the dummy phase, if any.
    /// If width is single, SD0/MOSI is held asserted low during the dummy phase, and SD1...SD3 are tristated. If width is dual/quad, all IOs are tristated during the dummy phase.
    #[inline(always)]
    pub fn dummy_width(&self) -> u32 {
        (self.value.read_bits() & Self::DUMMY_WIDTH_MASK) >> Self::DUMMY_WIDTH_OFFSET
    }

    /// Write **DUMMY_WIDTH** `[7:6]` (RW)
    /// The width used for the dummy phase, if any.
    /// If width is single, SD0/MOSI is held asserted low during the dummy phase, and SD1...SD3 are tristated. If width is dual/quad, all IOs are tristated during the dummy phase.
    #[inline(always)]
    pub fn write_dummy_width(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DUMMY_WIDTH_MASK) | ((value << Self::DUMMY_WIDTH_OFFSET) & Self::DUMMY_WIDTH_MASK));
    }

    /// **SUFFIX_WIDTH** `[5:4]` (RW)
    /// The width used for the post-address command suffix, if any
    #[inline(always)]
    pub fn suffix_width(&self) -> u32 {
        (self.value.read_bits() & Self::SUFFIX_WIDTH_MASK) >> Self::SUFFIX_WIDTH_OFFSET
    }

    /// Write **SUFFIX_WIDTH** `[5:4]` (RW)
    /// The width used for the post-address command suffix, if any
    #[inline(always)]
    pub fn write_suffix_width(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SUFFIX_WIDTH_MASK) | ((value << Self::SUFFIX_WIDTH_OFFSET) & Self::SUFFIX_WIDTH_MASK));
    }

    /// **ADDR_WIDTH** `[3:2]` (RW)
    /// The transfer width used for the address. The address phase always transfers 24 bits in total.
    #[inline(always)]
    pub fn addr_width(&self) -> u32 {
        (self.value.read_bits() & Self::ADDR_WIDTH_MASK) >> Self::ADDR_WIDTH_OFFSET
    }

    /// Write **ADDR_WIDTH** `[3:2]` (RW)
    /// The transfer width used for the address. The address phase always transfers 24 bits in total.
    #[inline(always)]
    pub fn write_addr_width(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::ADDR_WIDTH_MASK) | ((value << Self::ADDR_WIDTH_OFFSET) & Self::ADDR_WIDTH_MASK));
    }

    /// **PREFIX_WIDTH** `[1:0]` (RW)
    /// The transfer width used for the command prefix, if any
    #[inline(always)]
    pub fn prefix_width(&self) -> u32 {
        (self.value.read_bits() & Self::PREFIX_WIDTH_MASK) >> Self::PREFIX_WIDTH_OFFSET
    }

    /// Write **PREFIX_WIDTH** `[1:0]` (RW)
    /// The transfer width used for the command prefix, if any
    #[inline(always)]
    pub fn write_prefix_width(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::PREFIX_WIDTH_MASK) | ((value << Self::PREFIX_WIDTH_OFFSET) & Self::PREFIX_WIDTH_MASK));
    }

}

/// M0_WFMT value type for batch read/modify/write operations
///
/// Use `m0wfmt.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `m0wfmt.write_val(val)`.
#[derive(Clone, Copy)]
pub struct M0WfmtVal {
    bits: u32,
}

impl M0WfmtVal {
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
        Self { bits: 0x00001000 }
    }

    /// Read **DTR** `[28:28]`
    /// Enable double transfer rate (DTR) for write commands: address, suffix and write data phases are active on both edges of SCK. SDO data is launched centre-aligned on each SCK edge, and SDI data is captured on the SCK edge that follows its launch.
    /// DTR is implemented by halving the clock rate; SCK has a period of 2 x CLK_DIV throughout the transfer. The prefix and dummy phases are still single transfer rate.
    /// If the suffix is quad-width, it must be 0 or 8 bits in length, to ensure an even number of SCK edges.
    #[inline(always)]
    pub const fn dtr(&self) -> bool {
        (self.bits & M0Wfmt::DTR_MASK) != 0
    }

    /// Read **DUMMY_LEN** `[18:16]`
    /// Length of dummy phase between command suffix and data phase, in units of 4 bits. (i.e. 1 cycle for quad width, 2 for dual, 4 for single)
    #[inline(always)]
    pub const fn dummy_len(&self) -> u32 {
        (self.bits & M0Wfmt::DUMMY_LEN_MASK) >> M0Wfmt::DUMMY_LEN_OFFSET
    }

    /// Read **SUFFIX_LEN** `[15:14]`
    /// Length of post-address command suffix, in units of 4 bits. (i.e. 1 cycle for quad width, 2 for dual, 4 for single)
    /// Only values of 0 and 8 bits are supported.
    #[inline(always)]
    pub const fn suffix_len(&self) -> u32 {
        (self.bits & M0Wfmt::SUFFIX_LEN_MASK) >> M0Wfmt::SUFFIX_LEN_OFFSET
    }

    /// Read **PREFIX_LEN** `[12:12]`
    /// Length of command prefix, in units of 8 bits. (i.e. 2 cycles for quad width, 4 for dual, 8 for single)
    #[inline(always)]
    pub const fn prefix_len(&self) -> bool {
        (self.bits & M0Wfmt::PREFIX_LEN_MASK) != 0
    }

    /// Read **DATA_WIDTH** `[9:8]`
    /// The width used for the data transfer
    #[inline(always)]
    pub const fn data_width(&self) -> u32 {
        (self.bits & M0Wfmt::DATA_WIDTH_MASK) >> M0Wfmt::DATA_WIDTH_OFFSET
    }

    /// Read **DUMMY_WIDTH** `[7:6]`
    /// The width used for the dummy phase, if any.
    /// If width is single, SD0/MOSI is held asserted low during the dummy phase, and SD1...SD3 are tristated. If width is dual/quad, all IOs are tristated during the dummy phase.
    #[inline(always)]
    pub const fn dummy_width(&self) -> u32 {
        (self.bits & M0Wfmt::DUMMY_WIDTH_MASK) >> M0Wfmt::DUMMY_WIDTH_OFFSET
    }

    /// Read **SUFFIX_WIDTH** `[5:4]`
    /// The width used for the post-address command suffix, if any
    #[inline(always)]
    pub const fn suffix_width(&self) -> u32 {
        (self.bits & M0Wfmt::SUFFIX_WIDTH_MASK) >> M0Wfmt::SUFFIX_WIDTH_OFFSET
    }

    /// Read **ADDR_WIDTH** `[3:2]`
    /// The transfer width used for the address. The address phase always transfers 24 bits in total.
    #[inline(always)]
    pub const fn addr_width(&self) -> u32 {
        (self.bits & M0Wfmt::ADDR_WIDTH_MASK) >> M0Wfmt::ADDR_WIDTH_OFFSET
    }

    /// Read **PREFIX_WIDTH** `[1:0]`
    /// The transfer width used for the command prefix, if any
    #[inline(always)]
    pub const fn prefix_width(&self) -> u32 {
        (self.bits & M0Wfmt::PREFIX_WIDTH_MASK) >> M0Wfmt::PREFIX_WIDTH_OFFSET
    }

    /// Set **DTR** `[28:28]`
    /// Enable double transfer rate (DTR) for write commands: address, suffix and write data phases are active on both edges of SCK. SDO data is launched centre-aligned on each SCK edge, and SDI data is captured on the SCK edge that follows its launch.
    /// DTR is implemented by halving the clock rate; SCK has a period of 2 x CLK_DIV throughout the transfer. The prefix and dummy phases are still single transfer rate.
    /// If the suffix is quad-width, it must be 0 or 8 bits in length, to ensure an even number of SCK edges.
    #[inline(always)]
    pub const fn with_dtr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | M0Wfmt::DTR_MASK
            } else {
                self.bits & !M0Wfmt::DTR_MASK
            }
        }
    }

    /// Set **DUMMY_LEN** `[18:16]`
    /// Length of dummy phase between command suffix and data phase, in units of 4 bits. (i.e. 1 cycle for quad width, 2 for dual, 4 for single)
    #[inline(always)]
    pub const fn with_dummy_len(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !M0Wfmt::DUMMY_LEN_MASK) | ((value << M0Wfmt::DUMMY_LEN_OFFSET) & M0Wfmt::DUMMY_LEN_MASK)
        }
    }

    /// Set **SUFFIX_LEN** `[15:14]`
    /// Length of post-address command suffix, in units of 4 bits. (i.e. 1 cycle for quad width, 2 for dual, 4 for single)
    /// Only values of 0 and 8 bits are supported.
    #[inline(always)]
    pub const fn with_suffix_len(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !M0Wfmt::SUFFIX_LEN_MASK) | ((value << M0Wfmt::SUFFIX_LEN_OFFSET) & M0Wfmt::SUFFIX_LEN_MASK)
        }
    }

    /// Set **PREFIX_LEN** `[12:12]`
    /// Length of command prefix, in units of 8 bits. (i.e. 2 cycles for quad width, 4 for dual, 8 for single)
    #[inline(always)]
    pub const fn with_prefix_len(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | M0Wfmt::PREFIX_LEN_MASK
            } else {
                self.bits & !M0Wfmt::PREFIX_LEN_MASK
            }
        }
    }

    /// Set **DATA_WIDTH** `[9:8]`
    /// The width used for the data transfer
    #[inline(always)]
    pub const fn with_data_width(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !M0Wfmt::DATA_WIDTH_MASK) | ((value << M0Wfmt::DATA_WIDTH_OFFSET) & M0Wfmt::DATA_WIDTH_MASK)
        }
    }

    /// Set **DUMMY_WIDTH** `[7:6]`
    /// The width used for the dummy phase, if any.
    /// If width is single, SD0/MOSI is held asserted low during the dummy phase, and SD1...SD3 are tristated. If width is dual/quad, all IOs are tristated during the dummy phase.
    #[inline(always)]
    pub const fn with_dummy_width(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !M0Wfmt::DUMMY_WIDTH_MASK) | ((value << M0Wfmt::DUMMY_WIDTH_OFFSET) & M0Wfmt::DUMMY_WIDTH_MASK)
        }
    }

    /// Set **SUFFIX_WIDTH** `[5:4]`
    /// The width used for the post-address command suffix, if any
    #[inline(always)]
    pub const fn with_suffix_width(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !M0Wfmt::SUFFIX_WIDTH_MASK) | ((value << M0Wfmt::SUFFIX_WIDTH_OFFSET) & M0Wfmt::SUFFIX_WIDTH_MASK)
        }
    }

    /// Set **ADDR_WIDTH** `[3:2]`
    /// The transfer width used for the address. The address phase always transfers 24 bits in total.
    #[inline(always)]
    pub const fn with_addr_width(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !M0Wfmt::ADDR_WIDTH_MASK) | ((value << M0Wfmt::ADDR_WIDTH_OFFSET) & M0Wfmt::ADDR_WIDTH_MASK)
        }
    }

    /// Set **PREFIX_WIDTH** `[1:0]`
    /// The transfer width used for the command prefix, if any
    #[inline(always)]
    pub const fn with_prefix_width(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !M0Wfmt::PREFIX_WIDTH_MASK) | ((value << M0Wfmt::PREFIX_WIDTH_OFFSET) & M0Wfmt::PREFIX_WIDTH_MASK)
        }
    }

}

impl Default for M0WfmtVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl M0Wfmt {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> M0WfmtVal {
        M0WfmtVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: M0WfmtVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(M0WfmtVal) -> M0WfmtVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// M0_WCMD Register
///
/// Command constants used for writes to memory address window 0.
/// The reset value of the M0_WCMD register is configured to support a basic 02h serial write transfer with no additional configuration.
///
/// Address offset: 0x001c
/// Reset value: 0x0000a002
///
/// ## Fields
///
/// - **SUFFIX** `[15:8]` (RW)
///   The command suffix bits following the address, if Mx_WFMT_SUFFIX_LEN is nonzero.
/// - **PREFIX** `[7:0]` (RW)
///   The command prefix bits to prepend on each new transfer, if Mx_WFMT_PREFIX_LEN is nonzero.
#[repr(transparent)]
pub struct M0Wcmd {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for M0Wcmd {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl M0Wcmd {
    // ======== Field Constants ========

    /// **SUFFIX**
    /// The command suffix bits following the address, if Mx_WFMT_SUFFIX_LEN is nonzero.
    /// Bits `[15:8]`, mask `0x0000ff00`
    pub const SUFFIX_OFFSET: u32 = 8;
    pub const SUFFIX_MASK: u32 = 0x0000ff00;

    /// **PREFIX**
    /// The command prefix bits to prepend on each new transfer, if Mx_WFMT_PREFIX_LEN is nonzero.
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const PREFIX_OFFSET: u32 = 0;
    pub const PREFIX_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **SUFFIX** `[15:8]` (RW)
    /// The command suffix bits following the address, if Mx_WFMT_SUFFIX_LEN is nonzero.
    #[inline(always)]
    pub fn suffix(&self) -> u32 {
        (self.value.read_bits() & Self::SUFFIX_MASK) >> Self::SUFFIX_OFFSET
    }

    /// Write **SUFFIX** `[15:8]` (RW)
    /// The command suffix bits following the address, if Mx_WFMT_SUFFIX_LEN is nonzero.
    #[inline(always)]
    pub fn write_suffix(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SUFFIX_MASK) | ((value << Self::SUFFIX_OFFSET) & Self::SUFFIX_MASK));
    }

    /// **PREFIX** `[7:0]` (RW)
    /// The command prefix bits to prepend on each new transfer, if Mx_WFMT_PREFIX_LEN is nonzero.
    #[inline(always)]
    pub fn prefix(&self) -> u32 {
        (self.value.read_bits() & Self::PREFIX_MASK) >> Self::PREFIX_OFFSET
    }

    /// Write **PREFIX** `[7:0]` (RW)
    /// The command prefix bits to prepend on each new transfer, if Mx_WFMT_PREFIX_LEN is nonzero.
    #[inline(always)]
    pub fn write_prefix(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::PREFIX_MASK) | ((value << Self::PREFIX_OFFSET) & Self::PREFIX_MASK));
    }

}

/// M0_WCMD value type for batch read/modify/write operations
///
/// Use `m0wcmd.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `m0wcmd.write_val(val)`.
#[derive(Clone, Copy)]
pub struct M0WcmdVal {
    bits: u32,
}

impl M0WcmdVal {
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
        Self { bits: 0x0000a002 }
    }

    /// Read **SUFFIX** `[15:8]`
    /// The command suffix bits following the address, if Mx_WFMT_SUFFIX_LEN is nonzero.
    #[inline(always)]
    pub const fn suffix(&self) -> u32 {
        (self.bits & M0Wcmd::SUFFIX_MASK) >> M0Wcmd::SUFFIX_OFFSET
    }

    /// Read **PREFIX** `[7:0]`
    /// The command prefix bits to prepend on each new transfer, if Mx_WFMT_PREFIX_LEN is nonzero.
    #[inline(always)]
    pub const fn prefix(&self) -> u32 {
        (self.bits & M0Wcmd::PREFIX_MASK) >> M0Wcmd::PREFIX_OFFSET
    }

    /// Set **SUFFIX** `[15:8]`
    /// The command suffix bits following the address, if Mx_WFMT_SUFFIX_LEN is nonzero.
    #[inline(always)]
    pub const fn with_suffix(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !M0Wcmd::SUFFIX_MASK) | ((value << M0Wcmd::SUFFIX_OFFSET) & M0Wcmd::SUFFIX_MASK)
        }
    }

    /// Set **PREFIX** `[7:0]`
    /// The command prefix bits to prepend on each new transfer, if Mx_WFMT_PREFIX_LEN is nonzero.
    #[inline(always)]
    pub const fn with_prefix(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !M0Wcmd::PREFIX_MASK) | ((value << M0Wcmd::PREFIX_OFFSET) & M0Wcmd::PREFIX_MASK)
        }
    }

}

impl Default for M0WcmdVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl M0Wcmd {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> M0WcmdVal {
        M0WcmdVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: M0WcmdVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(M0WcmdVal) -> M0WcmdVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// M1_TIMING Register
///
/// Timing configuration register for memory address window 1.
///
/// Address offset: 0x0020
/// Reset value: 0x40000004
///
/// ## Fields
///
/// - **COOLDOWN** `[31:30]` (RW)
///   Chip select cooldown period. When a memory transfer finishes, the chip select remains asserted for 64 x COOLDOWN system clock cycles, plus half an SCK clock period (rounded up for odd SCK divisors). After this cooldown expires, the chip select is always deasserted to save power.
///   If the next memory access arrives within the cooldown period, the QMI may be able to append more SCK cycles to the currently ongoing SPI transfer, rather than starting a new transfer. This reduces access latency and increases bus throughput.
///   Specifically, the next access must be in the same direction (read/write), access the same memory window (chip select 0/1), and follow sequentially the address of the last transfer. If any of these are false, the new access will first deassert the chip select, then begin a new transfer.
///   If COOLDOWN is 0, the address alignment configured by PAGEBREAK has been reached, or the total chip select assertion limit MAX_SELECT has been reached, the cooldown period is skipped, and the chip select will always be deasserted one half SCK period after the transfer finishes.
/// - **PAGEBREAK** `[29:28]` (RW)
///   When page break is enabled, chip select will automatically deassert when crossing certain power-of-2-aligned address boundaries. The next access will always begin a new read/write SPI burst, even if the address of the next access follows in sequence with the last access before the page boundary.
///   Some flash and PSRAM devices forbid crossing page boundaries with a single read/write transfer, or restrict the operating frequency for transfers that do cross page a boundary. This option allows the QMI to safely support those devices.
///   This field has no effect when COOLDOWN is disabled.
/// - **SELECT_SETUP** `[25]` (RW)
///   Add up to one additional system clock cycle of setup between chip select assertion and the first rising edge of SCK.
///   The default setup time is one half SCK period, which is usually sufficient except for very high SCK frequencies with some flash devices.
/// - **SELECT_HOLD** `[24:23]` (RW)
///   Add up to three additional system clock cycles of active hold between the last falling edge of SCK and the deassertion of this window's chip select.
///   The default hold time is one system clock cycle. Note that flash datasheets usually give chip select active hold time from the last *rising* edge of SCK, and so even zero hold from the last falling edge would be safe.
///   Note that this is a minimum hold time guaranteed by the QMI: the actual chip select active hold may be slightly longer for read transfers with low clock divisors and/or high sample delays. Specifically, if the point two cycles after the last RX data sample is later than the last SCK falling edge, then the hold time is measured from *this* point.
///   Note also that, in case the final SCK pulse is masked to save energy (true for non-DTR reads when COOLDOWN is disabled or PAGE_BREAK is reached), all of QMI's timing logic behaves as though the clock pulse were still present. The SELECT_HOLD time is applied from the point where the last SCK falling edge would be if the clock pulse were not masked.
/// - **MAX_SELECT** `[22:17]` (RW)
///   Enforce a maximum assertion duration for this window's chip select, in units of 64 system clock cycles. If 0, the QMI is permitted to keep the chip select asserted indefinitely when servicing sequential memory accesses (see COOLDOWN).
///   This feature is required to meet timing constraints of PSRAM devices, which specify a maximum chip select assertion so they can perform DRAM refresh cycles. See also MIN_DESELECT, which can enforce a minimum deselect time.
///   If a memory access is in progress at the time MAX_SELECT is reached, the QMI will wait for the access to complete before deasserting the chip select. This additional time must be accounted for to calculate a safe MAX_SELECT value. In the worst case, this may be a fully-formed serial transfer, including command prefix and address, with a data payload as large as one cache line.
/// - **MIN_DESELECT** `[16:12]` (RW)
///   After this window's chip select is deasserted, it remains deasserted for half an SCK cycle (rounded up to an integer number of system clock cycles), plus MIN_DESELECT additional system clock cycles, before the QMI reasserts either chip select pin.
///   Nonzero values may be required for PSRAM devices which enforce a longer minimum CS deselect time, so that they can perform internal DRAM refresh cycles whilst deselected.
/// - **RXDELAY** `[10:8]` (RW)
///   Delay the read data sample timing, in units of one half of a system clock cycle. (Not necessarily half of an SCK cycle.) An RXDELAY of 0 means the sample is captured at the SDI input registers simultaneously with the rising edge of SCK launched from the SCK output register.
///   At higher SCK frequencies, RXDELAY may need to be increased to account for the round trip delay of the pads, and the clock-to-Q delay of the QSPI memory device.
/// - **CLKDIV** `[7:0]` (RW)
///   Clock divisor. Odd and even divisors are supported. Defines the SCK clock period in units of 1 system clock cycle. Divisors 1..255 are encoded directly, and a divisor of 256 is encoded with a value of CLKDIV=0.
///   The clock divisor can be changed on-the-fly, even when the QMI is currently accessing memory in this address window. All other parameters must only be changed when the QMI is idle.
///   If software is increasing CLKDIV in anticipation of an increase in the system clock frequency, a dummy access to either memory window (and appropriate processor barriers/fences) must be inserted after the Mx_TIMING write to ensure the SCK divisor change is in effect _before_ the system clock is changed.
#[repr(transparent)]
pub struct M1Timing {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for M1Timing {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl M1Timing {
    // ======== Field Constants ========

    /// **COOLDOWN**
    /// Chip select cooldown period. When a memory transfer finishes, the chip select remains asserted for 64 x COOLDOWN system clock cycles, plus half an SCK clock period (rounded up for odd SCK divisors). After this cooldown expires, the chip select is always deasserted to save power.
    /// If the next memory access arrives within the cooldown period, the QMI may be able to append more SCK cycles to the currently ongoing SPI transfer, rather than starting a new transfer. This reduces access latency and increases bus throughput.
    /// Specifically, the next access must be in the same direction (read/write), access the same memory window (chip select 0/1), and follow sequentially the address of the last transfer. If any of these are false, the new access will first deassert the chip select, then begin a new transfer.
    /// If COOLDOWN is 0, the address alignment configured by PAGEBREAK has been reached, or the total chip select assertion limit MAX_SELECT has been reached, the cooldown period is skipped, and the chip select will always be deasserted one half SCK period after the transfer finishes.
    /// Bits `[31:30]`, mask `0xc0000000`
    pub const COOLDOWN_OFFSET: u32 = 30;
    pub const COOLDOWN_MASK: u32 = 0xc0000000;

    /// **PAGEBREAK**
    /// When page break is enabled, chip select will automatically deassert when crossing certain power-of-2-aligned address boundaries. The next access will always begin a new read/write SPI burst, even if the address of the next access follows in sequence with the last access before the page boundary.
    /// Some flash and PSRAM devices forbid crossing page boundaries with a single read/write transfer, or restrict the operating frequency for transfers that do cross page a boundary. This option allows the QMI to safely support those devices.
    /// This field has no effect when COOLDOWN is disabled.
    /// Bits `[29:28]`, mask `0x30000000`
    pub const PAGEBREAK_OFFSET: u32 = 28;
    pub const PAGEBREAK_MASK: u32 = 0x30000000;
    /// NONE = 0
    /// No page boundary is enforced
    pub const PAGEBREAK_NONE: u32 = 0 << 28;
    pub const PAGEBREAK_NONE_VAL: u32 = 0;
    /// 256 = 1
    /// Break bursts crossing a 256-byte page boundary
    pub const PAGEBREAK_256: u32 = 1 << 28;
    pub const PAGEBREAK_256_VAL: u32 = 1;
    /// 1024 = 2
    /// Break bursts crossing a 1024-byte quad-page boundary
    pub const PAGEBREAK_1024: u32 = 2 << 28;
    pub const PAGEBREAK_1024_VAL: u32 = 2;
    /// 4096 = 3
    /// Break bursts crossing a 4096-byte sector boundary
    pub const PAGEBREAK_4096: u32 = 3 << 28;
    pub const PAGEBREAK_4096_VAL: u32 = 3;

    /// **SELECT_SETUP**
    /// Add up to one additional system clock cycle of setup between chip select assertion and the first rising edge of SCK.
    /// The default setup time is one half SCK period, which is usually sufficient except for very high SCK frequencies with some flash devices.
    /// Bits `[25:25]`, mask `0x02000000`
    pub const SELECT_SETUP_OFFSET: u32 = 25;
    pub const SELECT_SETUP_MASK: u32 = 0x02000000;
    pub const SELECT_SETUP: u32 = 1 << 25;

    /// **SELECT_HOLD**
    /// Add up to three additional system clock cycles of active hold between the last falling edge of SCK and the deassertion of this window's chip select.
    /// The default hold time is one system clock cycle. Note that flash datasheets usually give chip select active hold time from the last *rising* edge of SCK, and so even zero hold from the last falling edge would be safe.
    /// Note that this is a minimum hold time guaranteed by the QMI: the actual chip select active hold may be slightly longer for read transfers with low clock divisors and/or high sample delays. Specifically, if the point two cycles after the last RX data sample is later than the last SCK falling edge, then the hold time is measured from *this* point.
    /// Note also that, in case the final SCK pulse is masked to save energy (true for non-DTR reads when COOLDOWN is disabled or PAGE_BREAK is reached), all of QMI's timing logic behaves as though the clock pulse were still present. The SELECT_HOLD time is applied from the point where the last SCK falling edge would be if the clock pulse were not masked.
    /// Bits `[24:23]`, mask `0x01800000`
    pub const SELECT_HOLD_OFFSET: u32 = 23;
    pub const SELECT_HOLD_MASK: u32 = 0x01800000;

    /// **MAX_SELECT**
    /// Enforce a maximum assertion duration for this window's chip select, in units of 64 system clock cycles. If 0, the QMI is permitted to keep the chip select asserted indefinitely when servicing sequential memory accesses (see COOLDOWN).
    /// This feature is required to meet timing constraints of PSRAM devices, which specify a maximum chip select assertion so they can perform DRAM refresh cycles. See also MIN_DESELECT, which can enforce a minimum deselect time.
    /// If a memory access is in progress at the time MAX_SELECT is reached, the QMI will wait for the access to complete before deasserting the chip select. This additional time must be accounted for to calculate a safe MAX_SELECT value. In the worst case, this may be a fully-formed serial transfer, including command prefix and address, with a data payload as large as one cache line.
    /// Bits `[22:17]`, mask `0x007e0000`
    pub const MAX_SELECT_OFFSET: u32 = 17;
    pub const MAX_SELECT_MASK: u32 = 0x007e0000;

    /// **MIN_DESELECT**
    /// After this window's chip select is deasserted, it remains deasserted for half an SCK cycle (rounded up to an integer number of system clock cycles), plus MIN_DESELECT additional system clock cycles, before the QMI reasserts either chip select pin.
    /// Nonzero values may be required for PSRAM devices which enforce a longer minimum CS deselect time, so that they can perform internal DRAM refresh cycles whilst deselected.
    /// Bits `[16:12]`, mask `0x0001f000`
    pub const MIN_DESELECT_OFFSET: u32 = 12;
    pub const MIN_DESELECT_MASK: u32 = 0x0001f000;

    /// **RXDELAY**
    /// Delay the read data sample timing, in units of one half of a system clock cycle. (Not necessarily half of an SCK cycle.) An RXDELAY of 0 means the sample is captured at the SDI input registers simultaneously with the rising edge of SCK launched from the SCK output register.
    /// At higher SCK frequencies, RXDELAY may need to be increased to account for the round trip delay of the pads, and the clock-to-Q delay of the QSPI memory device.
    /// Bits `[10:8]`, mask `0x00000700`
    pub const RXDELAY_OFFSET: u32 = 8;
    pub const RXDELAY_MASK: u32 = 0x00000700;

    /// **CLKDIV**
    /// Clock divisor. Odd and even divisors are supported. Defines the SCK clock period in units of 1 system clock cycle. Divisors 1..255 are encoded directly, and a divisor of 256 is encoded with a value of CLKDIV=0.
    /// The clock divisor can be changed on-the-fly, even when the QMI is currently accessing memory in this address window. All other parameters must only be changed when the QMI is idle.
    /// If software is increasing CLKDIV in anticipation of an increase in the system clock frequency, a dummy access to either memory window (and appropriate processor barriers/fences) must be inserted after the Mx_TIMING write to ensure the SCK divisor change is in effect _before_ the system clock is changed.
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const CLKDIV_OFFSET: u32 = 0;
    pub const CLKDIV_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **COOLDOWN** `[31:30]` (RW)
    /// Chip select cooldown period. When a memory transfer finishes, the chip select remains asserted for 64 x COOLDOWN system clock cycles, plus half an SCK clock period (rounded up for odd SCK divisors). After this cooldown expires, the chip select is always deasserted to save power.
    /// If the next memory access arrives within the cooldown period, the QMI may be able to append more SCK cycles to the currently ongoing SPI transfer, rather than starting a new transfer. This reduces access latency and increases bus throughput.
    /// Specifically, the next access must be in the same direction (read/write), access the same memory window (chip select 0/1), and follow sequentially the address of the last transfer. If any of these are false, the new access will first deassert the chip select, then begin a new transfer.
    /// If COOLDOWN is 0, the address alignment configured by PAGEBREAK has been reached, or the total chip select assertion limit MAX_SELECT has been reached, the cooldown period is skipped, and the chip select will always be deasserted one half SCK period after the transfer finishes.
    #[inline(always)]
    pub fn cooldown(&self) -> u32 {
        (self.value.read_bits() & Self::COOLDOWN_MASK) >> Self::COOLDOWN_OFFSET
    }

    /// Write **COOLDOWN** `[31:30]` (RW)
    /// Chip select cooldown period. When a memory transfer finishes, the chip select remains asserted for 64 x COOLDOWN system clock cycles, plus half an SCK clock period (rounded up for odd SCK divisors). After this cooldown expires, the chip select is always deasserted to save power.
    /// If the next memory access arrives within the cooldown period, the QMI may be able to append more SCK cycles to the currently ongoing SPI transfer, rather than starting a new transfer. This reduces access latency and increases bus throughput.
    /// Specifically, the next access must be in the same direction (read/write), access the same memory window (chip select 0/1), and follow sequentially the address of the last transfer. If any of these are false, the new access will first deassert the chip select, then begin a new transfer.
    /// If COOLDOWN is 0, the address alignment configured by PAGEBREAK has been reached, or the total chip select assertion limit MAX_SELECT has been reached, the cooldown period is skipped, and the chip select will always be deasserted one half SCK period after the transfer finishes.
    #[inline(always)]
    pub fn write_cooldown(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::COOLDOWN_MASK) | ((value << Self::COOLDOWN_OFFSET) & Self::COOLDOWN_MASK));
    }

    /// **PAGEBREAK** `[29:28]` (RW)
    /// When page break is enabled, chip select will automatically deassert when crossing certain power-of-2-aligned address boundaries. The next access will always begin a new read/write SPI burst, even if the address of the next access follows in sequence with the last access before the page boundary.
    /// Some flash and PSRAM devices forbid crossing page boundaries with a single read/write transfer, or restrict the operating frequency for transfers that do cross page a boundary. This option allows the QMI to safely support those devices.
    /// This field has no effect when COOLDOWN is disabled.
    #[inline(always)]
    pub fn pagebreak(&self) -> u32 {
        (self.value.read_bits() & Self::PAGEBREAK_MASK) >> Self::PAGEBREAK_OFFSET
    }

    /// Write **PAGEBREAK** `[29:28]` (RW)
    /// When page break is enabled, chip select will automatically deassert when crossing certain power-of-2-aligned address boundaries. The next access will always begin a new read/write SPI burst, even if the address of the next access follows in sequence with the last access before the page boundary.
    /// Some flash and PSRAM devices forbid crossing page boundaries with a single read/write transfer, or restrict the operating frequency for transfers that do cross page a boundary. This option allows the QMI to safely support those devices.
    /// This field has no effect when COOLDOWN is disabled.
    #[inline(always)]
    pub fn write_pagebreak(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::PAGEBREAK_MASK) | ((value << Self::PAGEBREAK_OFFSET) & Self::PAGEBREAK_MASK));
    }

    /// **SELECT_SETUP** `[25]` (RW)
    /// Add up to one additional system clock cycle of setup between chip select assertion and the first rising edge of SCK.
    /// The default setup time is one half SCK period, which is usually sufficient except for very high SCK frequencies with some flash devices.
    #[inline(always)]
    pub fn select_setup(&self) -> bool {
        (self.value.read_bits() & Self::SELECT_SETUP_MASK) != 0
    }

    /// Set SELECT_SETUP bit atomically (RW)
    #[inline(always)]
    pub fn set_select_setup(&self) {
        self.value.set_bits(Self::SELECT_SETUP_MASK);
    }

    /// Clear SELECT_SETUP bit atomically (RW)
    #[inline(always)]
    pub fn clr_select_setup(&self) {
        self.value.clr_bits(Self::SELECT_SETUP_MASK);
    }

    /// **SELECT_HOLD** `[24:23]` (RW)
    /// Add up to three additional system clock cycles of active hold between the last falling edge of SCK and the deassertion of this window's chip select.
    /// The default hold time is one system clock cycle. Note that flash datasheets usually give chip select active hold time from the last *rising* edge of SCK, and so even zero hold from the last falling edge would be safe.
    /// Note that this is a minimum hold time guaranteed by the QMI: the actual chip select active hold may be slightly longer for read transfers with low clock divisors and/or high sample delays. Specifically, if the point two cycles after the last RX data sample is later than the last SCK falling edge, then the hold time is measured from *this* point.
    /// Note also that, in case the final SCK pulse is masked to save energy (true for non-DTR reads when COOLDOWN is disabled or PAGE_BREAK is reached), all of QMI's timing logic behaves as though the clock pulse were still present. The SELECT_HOLD time is applied from the point where the last SCK falling edge would be if the clock pulse were not masked.
    #[inline(always)]
    pub fn select_hold(&self) -> u32 {
        (self.value.read_bits() & Self::SELECT_HOLD_MASK) >> Self::SELECT_HOLD_OFFSET
    }

    /// Write **SELECT_HOLD** `[24:23]` (RW)
    /// Add up to three additional system clock cycles of active hold between the last falling edge of SCK and the deassertion of this window's chip select.
    /// The default hold time is one system clock cycle. Note that flash datasheets usually give chip select active hold time from the last *rising* edge of SCK, and so even zero hold from the last falling edge would be safe.
    /// Note that this is a minimum hold time guaranteed by the QMI: the actual chip select active hold may be slightly longer for read transfers with low clock divisors and/or high sample delays. Specifically, if the point two cycles after the last RX data sample is later than the last SCK falling edge, then the hold time is measured from *this* point.
    /// Note also that, in case the final SCK pulse is masked to save energy (true for non-DTR reads when COOLDOWN is disabled or PAGE_BREAK is reached), all of QMI's timing logic behaves as though the clock pulse were still present. The SELECT_HOLD time is applied from the point where the last SCK falling edge would be if the clock pulse were not masked.
    #[inline(always)]
    pub fn write_select_hold(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SELECT_HOLD_MASK) | ((value << Self::SELECT_HOLD_OFFSET) & Self::SELECT_HOLD_MASK));
    }

    /// **MAX_SELECT** `[22:17]` (RW)
    /// Enforce a maximum assertion duration for this window's chip select, in units of 64 system clock cycles. If 0, the QMI is permitted to keep the chip select asserted indefinitely when servicing sequential memory accesses (see COOLDOWN).
    /// This feature is required to meet timing constraints of PSRAM devices, which specify a maximum chip select assertion so they can perform DRAM refresh cycles. See also MIN_DESELECT, which can enforce a minimum deselect time.
    /// If a memory access is in progress at the time MAX_SELECT is reached, the QMI will wait for the access to complete before deasserting the chip select. This additional time must be accounted for to calculate a safe MAX_SELECT value. In the worst case, this may be a fully-formed serial transfer, including command prefix and address, with a data payload as large as one cache line.
    #[inline(always)]
    pub fn max_select(&self) -> u32 {
        (self.value.read_bits() & Self::MAX_SELECT_MASK) >> Self::MAX_SELECT_OFFSET
    }

    /// Write **MAX_SELECT** `[22:17]` (RW)
    /// Enforce a maximum assertion duration for this window's chip select, in units of 64 system clock cycles. If 0, the QMI is permitted to keep the chip select asserted indefinitely when servicing sequential memory accesses (see COOLDOWN).
    /// This feature is required to meet timing constraints of PSRAM devices, which specify a maximum chip select assertion so they can perform DRAM refresh cycles. See also MIN_DESELECT, which can enforce a minimum deselect time.
    /// If a memory access is in progress at the time MAX_SELECT is reached, the QMI will wait for the access to complete before deasserting the chip select. This additional time must be accounted for to calculate a safe MAX_SELECT value. In the worst case, this may be a fully-formed serial transfer, including command prefix and address, with a data payload as large as one cache line.
    #[inline(always)]
    pub fn write_max_select(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::MAX_SELECT_MASK) | ((value << Self::MAX_SELECT_OFFSET) & Self::MAX_SELECT_MASK));
    }

    /// **MIN_DESELECT** `[16:12]` (RW)
    /// After this window's chip select is deasserted, it remains deasserted for half an SCK cycle (rounded up to an integer number of system clock cycles), plus MIN_DESELECT additional system clock cycles, before the QMI reasserts either chip select pin.
    /// Nonzero values may be required for PSRAM devices which enforce a longer minimum CS deselect time, so that they can perform internal DRAM refresh cycles whilst deselected.
    #[inline(always)]
    pub fn min_deselect(&self) -> u32 {
        (self.value.read_bits() & Self::MIN_DESELECT_MASK) >> Self::MIN_DESELECT_OFFSET
    }

    /// Write **MIN_DESELECT** `[16:12]` (RW)
    /// After this window's chip select is deasserted, it remains deasserted for half an SCK cycle (rounded up to an integer number of system clock cycles), plus MIN_DESELECT additional system clock cycles, before the QMI reasserts either chip select pin.
    /// Nonzero values may be required for PSRAM devices which enforce a longer minimum CS deselect time, so that they can perform internal DRAM refresh cycles whilst deselected.
    #[inline(always)]
    pub fn write_min_deselect(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::MIN_DESELECT_MASK) | ((value << Self::MIN_DESELECT_OFFSET) & Self::MIN_DESELECT_MASK));
    }

    /// **RXDELAY** `[10:8]` (RW)
    /// Delay the read data sample timing, in units of one half of a system clock cycle. (Not necessarily half of an SCK cycle.) An RXDELAY of 0 means the sample is captured at the SDI input registers simultaneously with the rising edge of SCK launched from the SCK output register.
    /// At higher SCK frequencies, RXDELAY may need to be increased to account for the round trip delay of the pads, and the clock-to-Q delay of the QSPI memory device.
    #[inline(always)]
    pub fn rxdelay(&self) -> u32 {
        (self.value.read_bits() & Self::RXDELAY_MASK) >> Self::RXDELAY_OFFSET
    }

    /// Write **RXDELAY** `[10:8]` (RW)
    /// Delay the read data sample timing, in units of one half of a system clock cycle. (Not necessarily half of an SCK cycle.) An RXDELAY of 0 means the sample is captured at the SDI input registers simultaneously with the rising edge of SCK launched from the SCK output register.
    /// At higher SCK frequencies, RXDELAY may need to be increased to account for the round trip delay of the pads, and the clock-to-Q delay of the QSPI memory device.
    #[inline(always)]
    pub fn write_rxdelay(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::RXDELAY_MASK) | ((value << Self::RXDELAY_OFFSET) & Self::RXDELAY_MASK));
    }

    /// **CLKDIV** `[7:0]` (RW)
    /// Clock divisor. Odd and even divisors are supported. Defines the SCK clock period in units of 1 system clock cycle. Divisors 1..255 are encoded directly, and a divisor of 256 is encoded with a value of CLKDIV=0.
    /// The clock divisor can be changed on-the-fly, even when the QMI is currently accessing memory in this address window. All other parameters must only be changed when the QMI is idle.
    /// If software is increasing CLKDIV in anticipation of an increase in the system clock frequency, a dummy access to either memory window (and appropriate processor barriers/fences) must be inserted after the Mx_TIMING write to ensure the SCK divisor change is in effect _before_ the system clock is changed.
    #[inline(always)]
    pub fn clkdiv(&self) -> u32 {
        (self.value.read_bits() & Self::CLKDIV_MASK) >> Self::CLKDIV_OFFSET
    }

    /// Write **CLKDIV** `[7:0]` (RW)
    /// Clock divisor. Odd and even divisors are supported. Defines the SCK clock period in units of 1 system clock cycle. Divisors 1..255 are encoded directly, and a divisor of 256 is encoded with a value of CLKDIV=0.
    /// The clock divisor can be changed on-the-fly, even when the QMI is currently accessing memory in this address window. All other parameters must only be changed when the QMI is idle.
    /// If software is increasing CLKDIV in anticipation of an increase in the system clock frequency, a dummy access to either memory window (and appropriate processor barriers/fences) must be inserted after the Mx_TIMING write to ensure the SCK divisor change is in effect _before_ the system clock is changed.
    #[inline(always)]
    pub fn write_clkdiv(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::CLKDIV_MASK) | ((value << Self::CLKDIV_OFFSET) & Self::CLKDIV_MASK));
    }

}

/// M1_TIMING value type for batch read/modify/write operations
///
/// Use `m1timing.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `m1timing.write_val(val)`.
#[derive(Clone, Copy)]
pub struct M1TimingVal {
    bits: u32,
}

impl M1TimingVal {
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
        Self { bits: 0x40000004 }
    }

    /// Read **COOLDOWN** `[31:30]`
    /// Chip select cooldown period. When a memory transfer finishes, the chip select remains asserted for 64 x COOLDOWN system clock cycles, plus half an SCK clock period (rounded up for odd SCK divisors). After this cooldown expires, the chip select is always deasserted to save power.
    /// If the next memory access arrives within the cooldown period, the QMI may be able to append more SCK cycles to the currently ongoing SPI transfer, rather than starting a new transfer. This reduces access latency and increases bus throughput.
    /// Specifically, the next access must be in the same direction (read/write), access the same memory window (chip select 0/1), and follow sequentially the address of the last transfer. If any of these are false, the new access will first deassert the chip select, then begin a new transfer.
    /// If COOLDOWN is 0, the address alignment configured by PAGEBREAK has been reached, or the total chip select assertion limit MAX_SELECT has been reached, the cooldown period is skipped, and the chip select will always be deasserted one half SCK period after the transfer finishes.
    #[inline(always)]
    pub const fn cooldown(&self) -> u32 {
        (self.bits & M1Timing::COOLDOWN_MASK) >> M1Timing::COOLDOWN_OFFSET
    }

    /// Read **PAGEBREAK** `[29:28]`
    /// When page break is enabled, chip select will automatically deassert when crossing certain power-of-2-aligned address boundaries. The next access will always begin a new read/write SPI burst, even if the address of the next access follows in sequence with the last access before the page boundary.
    /// Some flash and PSRAM devices forbid crossing page boundaries with a single read/write transfer, or restrict the operating frequency for transfers that do cross page a boundary. This option allows the QMI to safely support those devices.
    /// This field has no effect when COOLDOWN is disabled.
    #[inline(always)]
    pub const fn pagebreak(&self) -> u32 {
        (self.bits & M1Timing::PAGEBREAK_MASK) >> M1Timing::PAGEBREAK_OFFSET
    }

    /// Read **SELECT_SETUP** `[25:25]`
    /// Add up to one additional system clock cycle of setup between chip select assertion and the first rising edge of SCK.
    /// The default setup time is one half SCK period, which is usually sufficient except for very high SCK frequencies with some flash devices.
    #[inline(always)]
    pub const fn select_setup(&self) -> bool {
        (self.bits & M1Timing::SELECT_SETUP_MASK) != 0
    }

    /// Read **SELECT_HOLD** `[24:23]`
    /// Add up to three additional system clock cycles of active hold between the last falling edge of SCK and the deassertion of this window's chip select.
    /// The default hold time is one system clock cycle. Note that flash datasheets usually give chip select active hold time from the last *rising* edge of SCK, and so even zero hold from the last falling edge would be safe.
    /// Note that this is a minimum hold time guaranteed by the QMI: the actual chip select active hold may be slightly longer for read transfers with low clock divisors and/or high sample delays. Specifically, if the point two cycles after the last RX data sample is later than the last SCK falling edge, then the hold time is measured from *this* point.
    /// Note also that, in case the final SCK pulse is masked to save energy (true for non-DTR reads when COOLDOWN is disabled or PAGE_BREAK is reached), all of QMI's timing logic behaves as though the clock pulse were still present. The SELECT_HOLD time is applied from the point where the last SCK falling edge would be if the clock pulse were not masked.
    #[inline(always)]
    pub const fn select_hold(&self) -> u32 {
        (self.bits & M1Timing::SELECT_HOLD_MASK) >> M1Timing::SELECT_HOLD_OFFSET
    }

    /// Read **MAX_SELECT** `[22:17]`
    /// Enforce a maximum assertion duration for this window's chip select, in units of 64 system clock cycles. If 0, the QMI is permitted to keep the chip select asserted indefinitely when servicing sequential memory accesses (see COOLDOWN).
    /// This feature is required to meet timing constraints of PSRAM devices, which specify a maximum chip select assertion so they can perform DRAM refresh cycles. See also MIN_DESELECT, which can enforce a minimum deselect time.
    /// If a memory access is in progress at the time MAX_SELECT is reached, the QMI will wait for the access to complete before deasserting the chip select. This additional time must be accounted for to calculate a safe MAX_SELECT value. In the worst case, this may be a fully-formed serial transfer, including command prefix and address, with a data payload as large as one cache line.
    #[inline(always)]
    pub const fn max_select(&self) -> u32 {
        (self.bits & M1Timing::MAX_SELECT_MASK) >> M1Timing::MAX_SELECT_OFFSET
    }

    /// Read **MIN_DESELECT** `[16:12]`
    /// After this window's chip select is deasserted, it remains deasserted for half an SCK cycle (rounded up to an integer number of system clock cycles), plus MIN_DESELECT additional system clock cycles, before the QMI reasserts either chip select pin.
    /// Nonzero values may be required for PSRAM devices which enforce a longer minimum CS deselect time, so that they can perform internal DRAM refresh cycles whilst deselected.
    #[inline(always)]
    pub const fn min_deselect(&self) -> u32 {
        (self.bits & M1Timing::MIN_DESELECT_MASK) >> M1Timing::MIN_DESELECT_OFFSET
    }

    /// Read **RXDELAY** `[10:8]`
    /// Delay the read data sample timing, in units of one half of a system clock cycle. (Not necessarily half of an SCK cycle.) An RXDELAY of 0 means the sample is captured at the SDI input registers simultaneously with the rising edge of SCK launched from the SCK output register.
    /// At higher SCK frequencies, RXDELAY may need to be increased to account for the round trip delay of the pads, and the clock-to-Q delay of the QSPI memory device.
    #[inline(always)]
    pub const fn rxdelay(&self) -> u32 {
        (self.bits & M1Timing::RXDELAY_MASK) >> M1Timing::RXDELAY_OFFSET
    }

    /// Read **CLKDIV** `[7:0]`
    /// Clock divisor. Odd and even divisors are supported. Defines the SCK clock period in units of 1 system clock cycle. Divisors 1..255 are encoded directly, and a divisor of 256 is encoded with a value of CLKDIV=0.
    /// The clock divisor can be changed on-the-fly, even when the QMI is currently accessing memory in this address window. All other parameters must only be changed when the QMI is idle.
    /// If software is increasing CLKDIV in anticipation of an increase in the system clock frequency, a dummy access to either memory window (and appropriate processor barriers/fences) must be inserted after the Mx_TIMING write to ensure the SCK divisor change is in effect _before_ the system clock is changed.
    #[inline(always)]
    pub const fn clkdiv(&self) -> u32 {
        (self.bits & M1Timing::CLKDIV_MASK) >> M1Timing::CLKDIV_OFFSET
    }

    /// Set **COOLDOWN** `[31:30]`
    /// Chip select cooldown period. When a memory transfer finishes, the chip select remains asserted for 64 x COOLDOWN system clock cycles, plus half an SCK clock period (rounded up for odd SCK divisors). After this cooldown expires, the chip select is always deasserted to save power.
    /// If the next memory access arrives within the cooldown period, the QMI may be able to append more SCK cycles to the currently ongoing SPI transfer, rather than starting a new transfer. This reduces access latency and increases bus throughput.
    /// Specifically, the next access must be in the same direction (read/write), access the same memory window (chip select 0/1), and follow sequentially the address of the last transfer. If any of these are false, the new access will first deassert the chip select, then begin a new transfer.
    /// If COOLDOWN is 0, the address alignment configured by PAGEBREAK has been reached, or the total chip select assertion limit MAX_SELECT has been reached, the cooldown period is skipped, and the chip select will always be deasserted one half SCK period after the transfer finishes.
    #[inline(always)]
    pub const fn with_cooldown(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !M1Timing::COOLDOWN_MASK) | ((value << M1Timing::COOLDOWN_OFFSET) & M1Timing::COOLDOWN_MASK)
        }
    }

    /// Set **PAGEBREAK** `[29:28]`
    /// When page break is enabled, chip select will automatically deassert when crossing certain power-of-2-aligned address boundaries. The next access will always begin a new read/write SPI burst, even if the address of the next access follows in sequence with the last access before the page boundary.
    /// Some flash and PSRAM devices forbid crossing page boundaries with a single read/write transfer, or restrict the operating frequency for transfers that do cross page a boundary. This option allows the QMI to safely support those devices.
    /// This field has no effect when COOLDOWN is disabled.
    #[inline(always)]
    pub const fn with_pagebreak(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !M1Timing::PAGEBREAK_MASK) | ((value << M1Timing::PAGEBREAK_OFFSET) & M1Timing::PAGEBREAK_MASK)
        }
    }

    /// Set **SELECT_SETUP** `[25:25]`
    /// Add up to one additional system clock cycle of setup between chip select assertion and the first rising edge of SCK.
    /// The default setup time is one half SCK period, which is usually sufficient except for very high SCK frequencies with some flash devices.
    #[inline(always)]
    pub const fn with_select_setup(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | M1Timing::SELECT_SETUP_MASK
            } else {
                self.bits & !M1Timing::SELECT_SETUP_MASK
            }
        }
    }

    /// Set **SELECT_HOLD** `[24:23]`
    /// Add up to three additional system clock cycles of active hold between the last falling edge of SCK and the deassertion of this window's chip select.
    /// The default hold time is one system clock cycle. Note that flash datasheets usually give chip select active hold time from the last *rising* edge of SCK, and so even zero hold from the last falling edge would be safe.
    /// Note that this is a minimum hold time guaranteed by the QMI: the actual chip select active hold may be slightly longer for read transfers with low clock divisors and/or high sample delays. Specifically, if the point two cycles after the last RX data sample is later than the last SCK falling edge, then the hold time is measured from *this* point.
    /// Note also that, in case the final SCK pulse is masked to save energy (true for non-DTR reads when COOLDOWN is disabled or PAGE_BREAK is reached), all of QMI's timing logic behaves as though the clock pulse were still present. The SELECT_HOLD time is applied from the point where the last SCK falling edge would be if the clock pulse were not masked.
    #[inline(always)]
    pub const fn with_select_hold(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !M1Timing::SELECT_HOLD_MASK) | ((value << M1Timing::SELECT_HOLD_OFFSET) & M1Timing::SELECT_HOLD_MASK)
        }
    }

    /// Set **MAX_SELECT** `[22:17]`
    /// Enforce a maximum assertion duration for this window's chip select, in units of 64 system clock cycles. If 0, the QMI is permitted to keep the chip select asserted indefinitely when servicing sequential memory accesses (see COOLDOWN).
    /// This feature is required to meet timing constraints of PSRAM devices, which specify a maximum chip select assertion so they can perform DRAM refresh cycles. See also MIN_DESELECT, which can enforce a minimum deselect time.
    /// If a memory access is in progress at the time MAX_SELECT is reached, the QMI will wait for the access to complete before deasserting the chip select. This additional time must be accounted for to calculate a safe MAX_SELECT value. In the worst case, this may be a fully-formed serial transfer, including command prefix and address, with a data payload as large as one cache line.
    #[inline(always)]
    pub const fn with_max_select(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !M1Timing::MAX_SELECT_MASK) | ((value << M1Timing::MAX_SELECT_OFFSET) & M1Timing::MAX_SELECT_MASK)
        }
    }

    /// Set **MIN_DESELECT** `[16:12]`
    /// After this window's chip select is deasserted, it remains deasserted for half an SCK cycle (rounded up to an integer number of system clock cycles), plus MIN_DESELECT additional system clock cycles, before the QMI reasserts either chip select pin.
    /// Nonzero values may be required for PSRAM devices which enforce a longer minimum CS deselect time, so that they can perform internal DRAM refresh cycles whilst deselected.
    #[inline(always)]
    pub const fn with_min_deselect(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !M1Timing::MIN_DESELECT_MASK) | ((value << M1Timing::MIN_DESELECT_OFFSET) & M1Timing::MIN_DESELECT_MASK)
        }
    }

    /// Set **RXDELAY** `[10:8]`
    /// Delay the read data sample timing, in units of one half of a system clock cycle. (Not necessarily half of an SCK cycle.) An RXDELAY of 0 means the sample is captured at the SDI input registers simultaneously with the rising edge of SCK launched from the SCK output register.
    /// At higher SCK frequencies, RXDELAY may need to be increased to account for the round trip delay of the pads, and the clock-to-Q delay of the QSPI memory device.
    #[inline(always)]
    pub const fn with_rxdelay(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !M1Timing::RXDELAY_MASK) | ((value << M1Timing::RXDELAY_OFFSET) & M1Timing::RXDELAY_MASK)
        }
    }

    /// Set **CLKDIV** `[7:0]`
    /// Clock divisor. Odd and even divisors are supported. Defines the SCK clock period in units of 1 system clock cycle. Divisors 1..255 are encoded directly, and a divisor of 256 is encoded with a value of CLKDIV=0.
    /// The clock divisor can be changed on-the-fly, even when the QMI is currently accessing memory in this address window. All other parameters must only be changed when the QMI is idle.
    /// If software is increasing CLKDIV in anticipation of an increase in the system clock frequency, a dummy access to either memory window (and appropriate processor barriers/fences) must be inserted after the Mx_TIMING write to ensure the SCK divisor change is in effect _before_ the system clock is changed.
    #[inline(always)]
    pub const fn with_clkdiv(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !M1Timing::CLKDIV_MASK) | ((value << M1Timing::CLKDIV_OFFSET) & M1Timing::CLKDIV_MASK)
        }
    }

}

impl Default for M1TimingVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl M1Timing {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> M1TimingVal {
        M1TimingVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: M1TimingVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(M1TimingVal) -> M1TimingVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// M1_RFMT Register
///
/// Read transfer format configuration for memory address window 1.
/// Configure the bus width of each transfer phase individually, and configure the length or presence of the command prefix, command suffix and dummy/turnaround transfer phases. Only 24-bit addresses are supported.
/// The reset value of the M1_RFMT register is configured to support a basic 03h serial read transfer with no additional configuration.
///
/// Address offset: 0x0024
/// Reset value: 0x00001000
///
/// ## Fields
///
/// - **DTR** `[28]` (RW)
///   Enable double transfer rate (DTR) for read commands: address, suffix and read data phases are active on both edges of SCK. SDO data is launched centre-aligned on each SCK edge, and SDI data is captured on the SCK edge that follows its launch.
///   DTR is implemented by halving the clock rate; SCK has a period of 2 x CLK_DIV throughout the transfer. The prefix and dummy phases are still single transfer rate.
///   If the suffix is quad-width, it must be 0 or 8 bits in length, to ensure an even number of SCK edges.
/// - **DUMMY_LEN** `[18:16]` (RW)
///   Length of dummy phase between command suffix and data phase, in units of 4 bits. (i.e. 1 cycle for quad width, 2 for dual, 4 for single)
/// - **SUFFIX_LEN** `[15:14]` (RW)
///   Length of post-address command suffix, in units of 4 bits. (i.e. 1 cycle for quad width, 2 for dual, 4 for single)
///   Only values of 0 and 8 bits are supported.
/// - **PREFIX_LEN** `[12]` (RW)
///   Length of command prefix, in units of 8 bits. (i.e. 2 cycles for quad width, 4 for dual, 8 for single)
/// - **DATA_WIDTH** `[9:8]` (RW)
///   The width used for the data transfer
/// - **DUMMY_WIDTH** `[7:6]` (RW)
///   The width used for the dummy phase, if any.
///   If width is single, SD0/MOSI is held asserted low during the dummy phase, and SD1...SD3 are tristated. If width is dual/quad, all IOs are tristated during the dummy phase.
/// - **SUFFIX_WIDTH** `[5:4]` (RW)
///   The width used for the post-address command suffix, if any
/// - **ADDR_WIDTH** `[3:2]` (RW)
///   The transfer width used for the address. The address phase always transfers 24 bits in total.
/// - **PREFIX_WIDTH** `[1:0]` (RW)
///   The transfer width used for the command prefix, if any
#[repr(transparent)]
pub struct M1Rfmt {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for M1Rfmt {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl M1Rfmt {
    // ======== Field Constants ========

    /// **DTR**
    /// Enable double transfer rate (DTR) for read commands: address, suffix and read data phases are active on both edges of SCK. SDO data is launched centre-aligned on each SCK edge, and SDI data is captured on the SCK edge that follows its launch.
    /// DTR is implemented by halving the clock rate; SCK has a period of 2 x CLK_DIV throughout the transfer. The prefix and dummy phases are still single transfer rate.
    /// If the suffix is quad-width, it must be 0 or 8 bits in length, to ensure an even number of SCK edges.
    /// Bits `[28:28]`, mask `0x10000000`
    pub const DTR_OFFSET: u32 = 28;
    pub const DTR_MASK: u32 = 0x10000000;
    pub const DTR: u32 = 1 << 28;

    /// **DUMMY_LEN**
    /// Length of dummy phase between command suffix and data phase, in units of 4 bits. (i.e. 1 cycle for quad width, 2 for dual, 4 for single)
    /// Bits `[18:16]`, mask `0x00070000`
    pub const DUMMY_LEN_OFFSET: u32 = 16;
    pub const DUMMY_LEN_MASK: u32 = 0x00070000;
    /// NONE = 0
    /// No dummy phase
    pub const DUMMY_LEN_NONE: u32 = 0 << 16;
    pub const DUMMY_LEN_NONE_VAL: u32 = 0;
    /// 4 = 1
    /// 4 dummy bits
    pub const DUMMY_LEN_4: u32 = 1 << 16;
    pub const DUMMY_LEN_4_VAL: u32 = 1;
    /// 8 = 2
    /// 8 dummy bits
    pub const DUMMY_LEN_8: u32 = 2 << 16;
    pub const DUMMY_LEN_8_VAL: u32 = 2;
    /// 12 = 3
    /// 12 dummy bits
    pub const DUMMY_LEN_12: u32 = 3 << 16;
    pub const DUMMY_LEN_12_VAL: u32 = 3;
    /// 16 = 4
    /// 16 dummy bits
    pub const DUMMY_LEN_16: u32 = 4 << 16;
    pub const DUMMY_LEN_16_VAL: u32 = 4;
    /// 20 = 5
    /// 20 dummy bits
    pub const DUMMY_LEN_20: u32 = 5 << 16;
    pub const DUMMY_LEN_20_VAL: u32 = 5;
    /// 24 = 6
    /// 24 dummy bits
    pub const DUMMY_LEN_24: u32 = 6 << 16;
    pub const DUMMY_LEN_24_VAL: u32 = 6;
    /// 28 = 7
    /// 28 dummy bits
    pub const DUMMY_LEN_28: u32 = 7 << 16;
    pub const DUMMY_LEN_28_VAL: u32 = 7;

    /// **SUFFIX_LEN**
    /// Length of post-address command suffix, in units of 4 bits. (i.e. 1 cycle for quad width, 2 for dual, 4 for single)
    /// Only values of 0 and 8 bits are supported.
    /// Bits `[15:14]`, mask `0x0000c000`
    pub const SUFFIX_LEN_OFFSET: u32 = 14;
    pub const SUFFIX_LEN_MASK: u32 = 0x0000c000;
    /// NONE = 0
    /// No suffix
    pub const SUFFIX_LEN_NONE: u32 = 0 << 14;
    pub const SUFFIX_LEN_NONE_VAL: u32 = 0;
    /// 8 = 2
    /// 8-bit suffix
    pub const SUFFIX_LEN_8: u32 = 2 << 14;
    pub const SUFFIX_LEN_8_VAL: u32 = 2;

    /// **PREFIX_LEN**
    /// Length of command prefix, in units of 8 bits. (i.e. 2 cycles for quad width, 4 for dual, 8 for single)
    /// Bits `[12:12]`, mask `0x00001000`
    pub const PREFIX_LEN_OFFSET: u32 = 12;
    pub const PREFIX_LEN_MASK: u32 = 0x00001000;
    pub const PREFIX_LEN: u32 = 1 << 12;
    /// NONE = 0
    /// No prefix
    pub const PREFIX_LEN_NONE: u32 = 0 << 12;
    pub const PREFIX_LEN_NONE_VAL: u32 = 0;
    /// 8 = 1
    /// 8-bit prefix
    pub const PREFIX_LEN_8: u32 = 1 << 12;
    pub const PREFIX_LEN_8_VAL: u32 = 1;

    /// **DATA_WIDTH**
    /// The width used for the data transfer
    /// Bits `[9:8]`, mask `0x00000300`
    pub const DATA_WIDTH_OFFSET: u32 = 8;
    pub const DATA_WIDTH_MASK: u32 = 0x00000300;
    /// S = 0
    /// Single width
    pub const DATA_WIDTH_S: u32 = 0 << 8;
    pub const DATA_WIDTH_S_VAL: u32 = 0;
    /// D = 1
    /// Dual width
    pub const DATA_WIDTH_D: u32 = 1 << 8;
    pub const DATA_WIDTH_D_VAL: u32 = 1;
    /// Q = 2
    /// Quad width
    pub const DATA_WIDTH_Q: u32 = 2 << 8;
    pub const DATA_WIDTH_Q_VAL: u32 = 2;

    /// **DUMMY_WIDTH**
    /// The width used for the dummy phase, if any.
    /// If width is single, SD0/MOSI is held asserted low during the dummy phase, and SD1...SD3 are tristated. If width is dual/quad, all IOs are tristated during the dummy phase.
    /// Bits `[7:6]`, mask `0x000000c0`
    pub const DUMMY_WIDTH_OFFSET: u32 = 6;
    pub const DUMMY_WIDTH_MASK: u32 = 0x000000c0;
    /// S = 0
    /// Single width
    pub const DUMMY_WIDTH_S: u32 = 0 << 6;
    pub const DUMMY_WIDTH_S_VAL: u32 = 0;
    /// D = 1
    /// Dual width
    pub const DUMMY_WIDTH_D: u32 = 1 << 6;
    pub const DUMMY_WIDTH_D_VAL: u32 = 1;
    /// Q = 2
    /// Quad width
    pub const DUMMY_WIDTH_Q: u32 = 2 << 6;
    pub const DUMMY_WIDTH_Q_VAL: u32 = 2;

    /// **SUFFIX_WIDTH**
    /// The width used for the post-address command suffix, if any
    /// Bits `[5:4]`, mask `0x00000030`
    pub const SUFFIX_WIDTH_OFFSET: u32 = 4;
    pub const SUFFIX_WIDTH_MASK: u32 = 0x00000030;
    /// S = 0
    /// Single width
    pub const SUFFIX_WIDTH_S: u32 = 0 << 4;
    pub const SUFFIX_WIDTH_S_VAL: u32 = 0;
    /// D = 1
    /// Dual width
    pub const SUFFIX_WIDTH_D: u32 = 1 << 4;
    pub const SUFFIX_WIDTH_D_VAL: u32 = 1;
    /// Q = 2
    /// Quad width
    pub const SUFFIX_WIDTH_Q: u32 = 2 << 4;
    pub const SUFFIX_WIDTH_Q_VAL: u32 = 2;

    /// **ADDR_WIDTH**
    /// The transfer width used for the address. The address phase always transfers 24 bits in total.
    /// Bits `[3:2]`, mask `0x0000000c`
    pub const ADDR_WIDTH_OFFSET: u32 = 2;
    pub const ADDR_WIDTH_MASK: u32 = 0x0000000c;
    /// S = 0
    /// Single width
    pub const ADDR_WIDTH_S: u32 = 0 << 2;
    pub const ADDR_WIDTH_S_VAL: u32 = 0;
    /// D = 1
    /// Dual width
    pub const ADDR_WIDTH_D: u32 = 1 << 2;
    pub const ADDR_WIDTH_D_VAL: u32 = 1;
    /// Q = 2
    /// Quad width
    pub const ADDR_WIDTH_Q: u32 = 2 << 2;
    pub const ADDR_WIDTH_Q_VAL: u32 = 2;

    /// **PREFIX_WIDTH**
    /// The transfer width used for the command prefix, if any
    /// Bits `[1:0]`, mask `0x00000003`
    pub const PREFIX_WIDTH_OFFSET: u32 = 0;
    pub const PREFIX_WIDTH_MASK: u32 = 0x00000003;
    /// S = 0
    /// Single width
    pub const PREFIX_WIDTH_S: u32 = 0 << 0;
    pub const PREFIX_WIDTH_S_VAL: u32 = 0;
    /// D = 1
    /// Dual width
    pub const PREFIX_WIDTH_D: u32 = 1 << 0;
    pub const PREFIX_WIDTH_D_VAL: u32 = 1;
    /// Q = 2
    /// Quad width
    pub const PREFIX_WIDTH_Q: u32 = 2 << 0;
    pub const PREFIX_WIDTH_Q_VAL: u32 = 2;

    // ======== Field Accessors ========

    /// **DTR** `[28]` (RW)
    /// Enable double transfer rate (DTR) for read commands: address, suffix and read data phases are active on both edges of SCK. SDO data is launched centre-aligned on each SCK edge, and SDI data is captured on the SCK edge that follows its launch.
    /// DTR is implemented by halving the clock rate; SCK has a period of 2 x CLK_DIV throughout the transfer. The prefix and dummy phases are still single transfer rate.
    /// If the suffix is quad-width, it must be 0 or 8 bits in length, to ensure an even number of SCK edges.
    #[inline(always)]
    pub fn dtr(&self) -> bool {
        (self.value.read_bits() & Self::DTR_MASK) != 0
    }

    /// Set DTR bit atomically (RW)
    #[inline(always)]
    pub fn set_dtr(&self) {
        self.value.set_bits(Self::DTR_MASK);
    }

    /// Clear DTR bit atomically (RW)
    #[inline(always)]
    pub fn clr_dtr(&self) {
        self.value.clr_bits(Self::DTR_MASK);
    }

    /// **DUMMY_LEN** `[18:16]` (RW)
    /// Length of dummy phase between command suffix and data phase, in units of 4 bits. (i.e. 1 cycle for quad width, 2 for dual, 4 for single)
    #[inline(always)]
    pub fn dummy_len(&self) -> u32 {
        (self.value.read_bits() & Self::DUMMY_LEN_MASK) >> Self::DUMMY_LEN_OFFSET
    }

    /// Write **DUMMY_LEN** `[18:16]` (RW)
    /// Length of dummy phase between command suffix and data phase, in units of 4 bits. (i.e. 1 cycle for quad width, 2 for dual, 4 for single)
    #[inline(always)]
    pub fn write_dummy_len(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DUMMY_LEN_MASK) | ((value << Self::DUMMY_LEN_OFFSET) & Self::DUMMY_LEN_MASK));
    }

    /// **SUFFIX_LEN** `[15:14]` (RW)
    /// Length of post-address command suffix, in units of 4 bits. (i.e. 1 cycle for quad width, 2 for dual, 4 for single)
    /// Only values of 0 and 8 bits are supported.
    #[inline(always)]
    pub fn suffix_len(&self) -> u32 {
        (self.value.read_bits() & Self::SUFFIX_LEN_MASK) >> Self::SUFFIX_LEN_OFFSET
    }

    /// Write **SUFFIX_LEN** `[15:14]` (RW)
    /// Length of post-address command suffix, in units of 4 bits. (i.e. 1 cycle for quad width, 2 for dual, 4 for single)
    /// Only values of 0 and 8 bits are supported.
    #[inline(always)]
    pub fn write_suffix_len(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SUFFIX_LEN_MASK) | ((value << Self::SUFFIX_LEN_OFFSET) & Self::SUFFIX_LEN_MASK));
    }

    /// **PREFIX_LEN** `[12]` (RW)
    /// Length of command prefix, in units of 8 bits. (i.e. 2 cycles for quad width, 4 for dual, 8 for single)
    #[inline(always)]
    pub fn prefix_len(&self) -> bool {
        (self.value.read_bits() & Self::PREFIX_LEN_MASK) != 0
    }

    /// Set PREFIX_LEN bit atomically (RW)
    #[inline(always)]
    pub fn set_prefix_len(&self) {
        self.value.set_bits(Self::PREFIX_LEN_MASK);
    }

    /// Clear PREFIX_LEN bit atomically (RW)
    #[inline(always)]
    pub fn clr_prefix_len(&self) {
        self.value.clr_bits(Self::PREFIX_LEN_MASK);
    }

    /// **DATA_WIDTH** `[9:8]` (RW)
    /// The width used for the data transfer
    #[inline(always)]
    pub fn data_width(&self) -> u32 {
        (self.value.read_bits() & Self::DATA_WIDTH_MASK) >> Self::DATA_WIDTH_OFFSET
    }

    /// Write **DATA_WIDTH** `[9:8]` (RW)
    /// The width used for the data transfer
    #[inline(always)]
    pub fn write_data_width(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DATA_WIDTH_MASK) | ((value << Self::DATA_WIDTH_OFFSET) & Self::DATA_WIDTH_MASK));
    }

    /// **DUMMY_WIDTH** `[7:6]` (RW)
    /// The width used for the dummy phase, if any.
    /// If width is single, SD0/MOSI is held asserted low during the dummy phase, and SD1...SD3 are tristated. If width is dual/quad, all IOs are tristated during the dummy phase.
    #[inline(always)]
    pub fn dummy_width(&self) -> u32 {
        (self.value.read_bits() & Self::DUMMY_WIDTH_MASK) >> Self::DUMMY_WIDTH_OFFSET
    }

    /// Write **DUMMY_WIDTH** `[7:6]` (RW)
    /// The width used for the dummy phase, if any.
    /// If width is single, SD0/MOSI is held asserted low during the dummy phase, and SD1...SD3 are tristated. If width is dual/quad, all IOs are tristated during the dummy phase.
    #[inline(always)]
    pub fn write_dummy_width(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DUMMY_WIDTH_MASK) | ((value << Self::DUMMY_WIDTH_OFFSET) & Self::DUMMY_WIDTH_MASK));
    }

    /// **SUFFIX_WIDTH** `[5:4]` (RW)
    /// The width used for the post-address command suffix, if any
    #[inline(always)]
    pub fn suffix_width(&self) -> u32 {
        (self.value.read_bits() & Self::SUFFIX_WIDTH_MASK) >> Self::SUFFIX_WIDTH_OFFSET
    }

    /// Write **SUFFIX_WIDTH** `[5:4]` (RW)
    /// The width used for the post-address command suffix, if any
    #[inline(always)]
    pub fn write_suffix_width(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SUFFIX_WIDTH_MASK) | ((value << Self::SUFFIX_WIDTH_OFFSET) & Self::SUFFIX_WIDTH_MASK));
    }

    /// **ADDR_WIDTH** `[3:2]` (RW)
    /// The transfer width used for the address. The address phase always transfers 24 bits in total.
    #[inline(always)]
    pub fn addr_width(&self) -> u32 {
        (self.value.read_bits() & Self::ADDR_WIDTH_MASK) >> Self::ADDR_WIDTH_OFFSET
    }

    /// Write **ADDR_WIDTH** `[3:2]` (RW)
    /// The transfer width used for the address. The address phase always transfers 24 bits in total.
    #[inline(always)]
    pub fn write_addr_width(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::ADDR_WIDTH_MASK) | ((value << Self::ADDR_WIDTH_OFFSET) & Self::ADDR_WIDTH_MASK));
    }

    /// **PREFIX_WIDTH** `[1:0]` (RW)
    /// The transfer width used for the command prefix, if any
    #[inline(always)]
    pub fn prefix_width(&self) -> u32 {
        (self.value.read_bits() & Self::PREFIX_WIDTH_MASK) >> Self::PREFIX_WIDTH_OFFSET
    }

    /// Write **PREFIX_WIDTH** `[1:0]` (RW)
    /// The transfer width used for the command prefix, if any
    #[inline(always)]
    pub fn write_prefix_width(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::PREFIX_WIDTH_MASK) | ((value << Self::PREFIX_WIDTH_OFFSET) & Self::PREFIX_WIDTH_MASK));
    }

}

/// M1_RFMT value type for batch read/modify/write operations
///
/// Use `m1rfmt.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `m1rfmt.write_val(val)`.
#[derive(Clone, Copy)]
pub struct M1RfmtVal {
    bits: u32,
}

impl M1RfmtVal {
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
        Self { bits: 0x00001000 }
    }

    /// Read **DTR** `[28:28]`
    /// Enable double transfer rate (DTR) for read commands: address, suffix and read data phases are active on both edges of SCK. SDO data is launched centre-aligned on each SCK edge, and SDI data is captured on the SCK edge that follows its launch.
    /// DTR is implemented by halving the clock rate; SCK has a period of 2 x CLK_DIV throughout the transfer. The prefix and dummy phases are still single transfer rate.
    /// If the suffix is quad-width, it must be 0 or 8 bits in length, to ensure an even number of SCK edges.
    #[inline(always)]
    pub const fn dtr(&self) -> bool {
        (self.bits & M1Rfmt::DTR_MASK) != 0
    }

    /// Read **DUMMY_LEN** `[18:16]`
    /// Length of dummy phase between command suffix and data phase, in units of 4 bits. (i.e. 1 cycle for quad width, 2 for dual, 4 for single)
    #[inline(always)]
    pub const fn dummy_len(&self) -> u32 {
        (self.bits & M1Rfmt::DUMMY_LEN_MASK) >> M1Rfmt::DUMMY_LEN_OFFSET
    }

    /// Read **SUFFIX_LEN** `[15:14]`
    /// Length of post-address command suffix, in units of 4 bits. (i.e. 1 cycle for quad width, 2 for dual, 4 for single)
    /// Only values of 0 and 8 bits are supported.
    #[inline(always)]
    pub const fn suffix_len(&self) -> u32 {
        (self.bits & M1Rfmt::SUFFIX_LEN_MASK) >> M1Rfmt::SUFFIX_LEN_OFFSET
    }

    /// Read **PREFIX_LEN** `[12:12]`
    /// Length of command prefix, in units of 8 bits. (i.e. 2 cycles for quad width, 4 for dual, 8 for single)
    #[inline(always)]
    pub const fn prefix_len(&self) -> bool {
        (self.bits & M1Rfmt::PREFIX_LEN_MASK) != 0
    }

    /// Read **DATA_WIDTH** `[9:8]`
    /// The width used for the data transfer
    #[inline(always)]
    pub const fn data_width(&self) -> u32 {
        (self.bits & M1Rfmt::DATA_WIDTH_MASK) >> M1Rfmt::DATA_WIDTH_OFFSET
    }

    /// Read **DUMMY_WIDTH** `[7:6]`
    /// The width used for the dummy phase, if any.
    /// If width is single, SD0/MOSI is held asserted low during the dummy phase, and SD1...SD3 are tristated. If width is dual/quad, all IOs are tristated during the dummy phase.
    #[inline(always)]
    pub const fn dummy_width(&self) -> u32 {
        (self.bits & M1Rfmt::DUMMY_WIDTH_MASK) >> M1Rfmt::DUMMY_WIDTH_OFFSET
    }

    /// Read **SUFFIX_WIDTH** `[5:4]`
    /// The width used for the post-address command suffix, if any
    #[inline(always)]
    pub const fn suffix_width(&self) -> u32 {
        (self.bits & M1Rfmt::SUFFIX_WIDTH_MASK) >> M1Rfmt::SUFFIX_WIDTH_OFFSET
    }

    /// Read **ADDR_WIDTH** `[3:2]`
    /// The transfer width used for the address. The address phase always transfers 24 bits in total.
    #[inline(always)]
    pub const fn addr_width(&self) -> u32 {
        (self.bits & M1Rfmt::ADDR_WIDTH_MASK) >> M1Rfmt::ADDR_WIDTH_OFFSET
    }

    /// Read **PREFIX_WIDTH** `[1:0]`
    /// The transfer width used for the command prefix, if any
    #[inline(always)]
    pub const fn prefix_width(&self) -> u32 {
        (self.bits & M1Rfmt::PREFIX_WIDTH_MASK) >> M1Rfmt::PREFIX_WIDTH_OFFSET
    }

    /// Set **DTR** `[28:28]`
    /// Enable double transfer rate (DTR) for read commands: address, suffix and read data phases are active on both edges of SCK. SDO data is launched centre-aligned on each SCK edge, and SDI data is captured on the SCK edge that follows its launch.
    /// DTR is implemented by halving the clock rate; SCK has a period of 2 x CLK_DIV throughout the transfer. The prefix and dummy phases are still single transfer rate.
    /// If the suffix is quad-width, it must be 0 or 8 bits in length, to ensure an even number of SCK edges.
    #[inline(always)]
    pub const fn with_dtr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | M1Rfmt::DTR_MASK
            } else {
                self.bits & !M1Rfmt::DTR_MASK
            }
        }
    }

    /// Set **DUMMY_LEN** `[18:16]`
    /// Length of dummy phase between command suffix and data phase, in units of 4 bits. (i.e. 1 cycle for quad width, 2 for dual, 4 for single)
    #[inline(always)]
    pub const fn with_dummy_len(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !M1Rfmt::DUMMY_LEN_MASK) | ((value << M1Rfmt::DUMMY_LEN_OFFSET) & M1Rfmt::DUMMY_LEN_MASK)
        }
    }

    /// Set **SUFFIX_LEN** `[15:14]`
    /// Length of post-address command suffix, in units of 4 bits. (i.e. 1 cycle for quad width, 2 for dual, 4 for single)
    /// Only values of 0 and 8 bits are supported.
    #[inline(always)]
    pub const fn with_suffix_len(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !M1Rfmt::SUFFIX_LEN_MASK) | ((value << M1Rfmt::SUFFIX_LEN_OFFSET) & M1Rfmt::SUFFIX_LEN_MASK)
        }
    }

    /// Set **PREFIX_LEN** `[12:12]`
    /// Length of command prefix, in units of 8 bits. (i.e. 2 cycles for quad width, 4 for dual, 8 for single)
    #[inline(always)]
    pub const fn with_prefix_len(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | M1Rfmt::PREFIX_LEN_MASK
            } else {
                self.bits & !M1Rfmt::PREFIX_LEN_MASK
            }
        }
    }

    /// Set **DATA_WIDTH** `[9:8]`
    /// The width used for the data transfer
    #[inline(always)]
    pub const fn with_data_width(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !M1Rfmt::DATA_WIDTH_MASK) | ((value << M1Rfmt::DATA_WIDTH_OFFSET) & M1Rfmt::DATA_WIDTH_MASK)
        }
    }

    /// Set **DUMMY_WIDTH** `[7:6]`
    /// The width used for the dummy phase, if any.
    /// If width is single, SD0/MOSI is held asserted low during the dummy phase, and SD1...SD3 are tristated. If width is dual/quad, all IOs are tristated during the dummy phase.
    #[inline(always)]
    pub const fn with_dummy_width(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !M1Rfmt::DUMMY_WIDTH_MASK) | ((value << M1Rfmt::DUMMY_WIDTH_OFFSET) & M1Rfmt::DUMMY_WIDTH_MASK)
        }
    }

    /// Set **SUFFIX_WIDTH** `[5:4]`
    /// The width used for the post-address command suffix, if any
    #[inline(always)]
    pub const fn with_suffix_width(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !M1Rfmt::SUFFIX_WIDTH_MASK) | ((value << M1Rfmt::SUFFIX_WIDTH_OFFSET) & M1Rfmt::SUFFIX_WIDTH_MASK)
        }
    }

    /// Set **ADDR_WIDTH** `[3:2]`
    /// The transfer width used for the address. The address phase always transfers 24 bits in total.
    #[inline(always)]
    pub const fn with_addr_width(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !M1Rfmt::ADDR_WIDTH_MASK) | ((value << M1Rfmt::ADDR_WIDTH_OFFSET) & M1Rfmt::ADDR_WIDTH_MASK)
        }
    }

    /// Set **PREFIX_WIDTH** `[1:0]`
    /// The transfer width used for the command prefix, if any
    #[inline(always)]
    pub const fn with_prefix_width(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !M1Rfmt::PREFIX_WIDTH_MASK) | ((value << M1Rfmt::PREFIX_WIDTH_OFFSET) & M1Rfmt::PREFIX_WIDTH_MASK)
        }
    }

}

impl Default for M1RfmtVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl M1Rfmt {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> M1RfmtVal {
        M1RfmtVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: M1RfmtVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(M1RfmtVal) -> M1RfmtVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// M1_RCMD Register
///
/// Command constants used for reads from memory address window 1.
/// The reset value of the M1_RCMD register is configured to support a basic 03h serial read transfer with no additional configuration.
///
/// Address offset: 0x0028
/// Reset value: 0x0000a003
///
/// ## Fields
///
/// - **SUFFIX** `[15:8]` (RW)
///   The command suffix bits following the address, if Mx_RFMT_SUFFIX_LEN is nonzero.
/// - **PREFIX** `[7:0]` (RW)
///   The command prefix bits to prepend on each new transfer, if Mx_RFMT_PREFIX_LEN is nonzero.
#[repr(transparent)]
pub struct M1Rcmd {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for M1Rcmd {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl M1Rcmd {
    // ======== Field Constants ========

    /// **SUFFIX**
    /// The command suffix bits following the address, if Mx_RFMT_SUFFIX_LEN is nonzero.
    /// Bits `[15:8]`, mask `0x0000ff00`
    pub const SUFFIX_OFFSET: u32 = 8;
    pub const SUFFIX_MASK: u32 = 0x0000ff00;

    /// **PREFIX**
    /// The command prefix bits to prepend on each new transfer, if Mx_RFMT_PREFIX_LEN is nonzero.
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const PREFIX_OFFSET: u32 = 0;
    pub const PREFIX_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **SUFFIX** `[15:8]` (RW)
    /// The command suffix bits following the address, if Mx_RFMT_SUFFIX_LEN is nonzero.
    #[inline(always)]
    pub fn suffix(&self) -> u32 {
        (self.value.read_bits() & Self::SUFFIX_MASK) >> Self::SUFFIX_OFFSET
    }

    /// Write **SUFFIX** `[15:8]` (RW)
    /// The command suffix bits following the address, if Mx_RFMT_SUFFIX_LEN is nonzero.
    #[inline(always)]
    pub fn write_suffix(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SUFFIX_MASK) | ((value << Self::SUFFIX_OFFSET) & Self::SUFFIX_MASK));
    }

    /// **PREFIX** `[7:0]` (RW)
    /// The command prefix bits to prepend on each new transfer, if Mx_RFMT_PREFIX_LEN is nonzero.
    #[inline(always)]
    pub fn prefix(&self) -> u32 {
        (self.value.read_bits() & Self::PREFIX_MASK) >> Self::PREFIX_OFFSET
    }

    /// Write **PREFIX** `[7:0]` (RW)
    /// The command prefix bits to prepend on each new transfer, if Mx_RFMT_PREFIX_LEN is nonzero.
    #[inline(always)]
    pub fn write_prefix(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::PREFIX_MASK) | ((value << Self::PREFIX_OFFSET) & Self::PREFIX_MASK));
    }

}

/// M1_RCMD value type for batch read/modify/write operations
///
/// Use `m1rcmd.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `m1rcmd.write_val(val)`.
#[derive(Clone, Copy)]
pub struct M1RcmdVal {
    bits: u32,
}

impl M1RcmdVal {
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
        Self { bits: 0x0000a003 }
    }

    /// Read **SUFFIX** `[15:8]`
    /// The command suffix bits following the address, if Mx_RFMT_SUFFIX_LEN is nonzero.
    #[inline(always)]
    pub const fn suffix(&self) -> u32 {
        (self.bits & M1Rcmd::SUFFIX_MASK) >> M1Rcmd::SUFFIX_OFFSET
    }

    /// Read **PREFIX** `[7:0]`
    /// The command prefix bits to prepend on each new transfer, if Mx_RFMT_PREFIX_LEN is nonzero.
    #[inline(always)]
    pub const fn prefix(&self) -> u32 {
        (self.bits & M1Rcmd::PREFIX_MASK) >> M1Rcmd::PREFIX_OFFSET
    }

    /// Set **SUFFIX** `[15:8]`
    /// The command suffix bits following the address, if Mx_RFMT_SUFFIX_LEN is nonzero.
    #[inline(always)]
    pub const fn with_suffix(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !M1Rcmd::SUFFIX_MASK) | ((value << M1Rcmd::SUFFIX_OFFSET) & M1Rcmd::SUFFIX_MASK)
        }
    }

    /// Set **PREFIX** `[7:0]`
    /// The command prefix bits to prepend on each new transfer, if Mx_RFMT_PREFIX_LEN is nonzero.
    #[inline(always)]
    pub const fn with_prefix(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !M1Rcmd::PREFIX_MASK) | ((value << M1Rcmd::PREFIX_OFFSET) & M1Rcmd::PREFIX_MASK)
        }
    }

}

impl Default for M1RcmdVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl M1Rcmd {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> M1RcmdVal {
        M1RcmdVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: M1RcmdVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(M1RcmdVal) -> M1RcmdVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// M1_WFMT Register
///
/// Write transfer format configuration for memory address window 1.
/// Configure the bus width of each transfer phase individually, and configure the length or presence of the command prefix, command suffix and dummy/turnaround transfer phases. Only 24-bit addresses are supported.
/// The reset value of the M1_WFMT register is configured to support a basic 02h serial write transfer. However, writes to this window must first be enabled via the XIP_CTRL_WRITABLE_M1 bit, as XIP memory is read-only by default.
///
/// Address offset: 0x002c
/// Reset value: 0x00001000
///
/// ## Fields
///
/// - **DTR** `[28]` (RW)
///   Enable double transfer rate (DTR) for write commands: address, suffix and write data phases are active on both edges of SCK. SDO data is launched centre-aligned on each SCK edge, and SDI data is captured on the SCK edge that follows its launch.
///   DTR is implemented by halving the clock rate; SCK has a period of 2 x CLK_DIV throughout the transfer. The prefix and dummy phases are still single transfer rate.
///   If the suffix is quad-width, it must be 0 or 8 bits in length, to ensure an even number of SCK edges.
/// - **DUMMY_LEN** `[18:16]` (RW)
///   Length of dummy phase between command suffix and data phase, in units of 4 bits. (i.e. 1 cycle for quad width, 2 for dual, 4 for single)
/// - **SUFFIX_LEN** `[15:14]` (RW)
///   Length of post-address command suffix, in units of 4 bits. (i.e. 1 cycle for quad width, 2 for dual, 4 for single)
///   Only values of 0 and 8 bits are supported.
/// - **PREFIX_LEN** `[12]` (RW)
///   Length of command prefix, in units of 8 bits. (i.e. 2 cycles for quad width, 4 for dual, 8 for single)
/// - **DATA_WIDTH** `[9:8]` (RW)
///   The width used for the data transfer
/// - **DUMMY_WIDTH** `[7:6]` (RW)
///   The width used for the dummy phase, if any.
///   If width is single, SD0/MOSI is held asserted low during the dummy phase, and SD1...SD3 are tristated. If width is dual/quad, all IOs are tristated during the dummy phase.
/// - **SUFFIX_WIDTH** `[5:4]` (RW)
///   The width used for the post-address command suffix, if any
/// - **ADDR_WIDTH** `[3:2]` (RW)
///   The transfer width used for the address. The address phase always transfers 24 bits in total.
/// - **PREFIX_WIDTH** `[1:0]` (RW)
///   The transfer width used for the command prefix, if any
#[repr(transparent)]
pub struct M1Wfmt {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for M1Wfmt {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl M1Wfmt {
    // ======== Field Constants ========

    /// **DTR**
    /// Enable double transfer rate (DTR) for write commands: address, suffix and write data phases are active on both edges of SCK. SDO data is launched centre-aligned on each SCK edge, and SDI data is captured on the SCK edge that follows its launch.
    /// DTR is implemented by halving the clock rate; SCK has a period of 2 x CLK_DIV throughout the transfer. The prefix and dummy phases are still single transfer rate.
    /// If the suffix is quad-width, it must be 0 or 8 bits in length, to ensure an even number of SCK edges.
    /// Bits `[28:28]`, mask `0x10000000`
    pub const DTR_OFFSET: u32 = 28;
    pub const DTR_MASK: u32 = 0x10000000;
    pub const DTR: u32 = 1 << 28;

    /// **DUMMY_LEN**
    /// Length of dummy phase between command suffix and data phase, in units of 4 bits. (i.e. 1 cycle for quad width, 2 for dual, 4 for single)
    /// Bits `[18:16]`, mask `0x00070000`
    pub const DUMMY_LEN_OFFSET: u32 = 16;
    pub const DUMMY_LEN_MASK: u32 = 0x00070000;
    /// NONE = 0
    /// No dummy phase
    pub const DUMMY_LEN_NONE: u32 = 0 << 16;
    pub const DUMMY_LEN_NONE_VAL: u32 = 0;
    /// 4 = 1
    /// 4 dummy bits
    pub const DUMMY_LEN_4: u32 = 1 << 16;
    pub const DUMMY_LEN_4_VAL: u32 = 1;
    /// 8 = 2
    /// 8 dummy bits
    pub const DUMMY_LEN_8: u32 = 2 << 16;
    pub const DUMMY_LEN_8_VAL: u32 = 2;
    /// 12 = 3
    /// 12 dummy bits
    pub const DUMMY_LEN_12: u32 = 3 << 16;
    pub const DUMMY_LEN_12_VAL: u32 = 3;
    /// 16 = 4
    /// 16 dummy bits
    pub const DUMMY_LEN_16: u32 = 4 << 16;
    pub const DUMMY_LEN_16_VAL: u32 = 4;
    /// 20 = 5
    /// 20 dummy bits
    pub const DUMMY_LEN_20: u32 = 5 << 16;
    pub const DUMMY_LEN_20_VAL: u32 = 5;
    /// 24 = 6
    /// 24 dummy bits
    pub const DUMMY_LEN_24: u32 = 6 << 16;
    pub const DUMMY_LEN_24_VAL: u32 = 6;
    /// 28 = 7
    /// 28 dummy bits
    pub const DUMMY_LEN_28: u32 = 7 << 16;
    pub const DUMMY_LEN_28_VAL: u32 = 7;

    /// **SUFFIX_LEN**
    /// Length of post-address command suffix, in units of 4 bits. (i.e. 1 cycle for quad width, 2 for dual, 4 for single)
    /// Only values of 0 and 8 bits are supported.
    /// Bits `[15:14]`, mask `0x0000c000`
    pub const SUFFIX_LEN_OFFSET: u32 = 14;
    pub const SUFFIX_LEN_MASK: u32 = 0x0000c000;
    /// NONE = 0
    /// No suffix
    pub const SUFFIX_LEN_NONE: u32 = 0 << 14;
    pub const SUFFIX_LEN_NONE_VAL: u32 = 0;
    /// 8 = 2
    /// 8-bit suffix
    pub const SUFFIX_LEN_8: u32 = 2 << 14;
    pub const SUFFIX_LEN_8_VAL: u32 = 2;

    /// **PREFIX_LEN**
    /// Length of command prefix, in units of 8 bits. (i.e. 2 cycles for quad width, 4 for dual, 8 for single)
    /// Bits `[12:12]`, mask `0x00001000`
    pub const PREFIX_LEN_OFFSET: u32 = 12;
    pub const PREFIX_LEN_MASK: u32 = 0x00001000;
    pub const PREFIX_LEN: u32 = 1 << 12;
    /// NONE = 0
    /// No prefix
    pub const PREFIX_LEN_NONE: u32 = 0 << 12;
    pub const PREFIX_LEN_NONE_VAL: u32 = 0;
    /// 8 = 1
    /// 8-bit prefix
    pub const PREFIX_LEN_8: u32 = 1 << 12;
    pub const PREFIX_LEN_8_VAL: u32 = 1;

    /// **DATA_WIDTH**
    /// The width used for the data transfer
    /// Bits `[9:8]`, mask `0x00000300`
    pub const DATA_WIDTH_OFFSET: u32 = 8;
    pub const DATA_WIDTH_MASK: u32 = 0x00000300;
    /// S = 0
    /// Single width
    pub const DATA_WIDTH_S: u32 = 0 << 8;
    pub const DATA_WIDTH_S_VAL: u32 = 0;
    /// D = 1
    /// Dual width
    pub const DATA_WIDTH_D: u32 = 1 << 8;
    pub const DATA_WIDTH_D_VAL: u32 = 1;
    /// Q = 2
    /// Quad width
    pub const DATA_WIDTH_Q: u32 = 2 << 8;
    pub const DATA_WIDTH_Q_VAL: u32 = 2;

    /// **DUMMY_WIDTH**
    /// The width used for the dummy phase, if any.
    /// If width is single, SD0/MOSI is held asserted low during the dummy phase, and SD1...SD3 are tristated. If width is dual/quad, all IOs are tristated during the dummy phase.
    /// Bits `[7:6]`, mask `0x000000c0`
    pub const DUMMY_WIDTH_OFFSET: u32 = 6;
    pub const DUMMY_WIDTH_MASK: u32 = 0x000000c0;
    /// S = 0
    /// Single width
    pub const DUMMY_WIDTH_S: u32 = 0 << 6;
    pub const DUMMY_WIDTH_S_VAL: u32 = 0;
    /// D = 1
    /// Dual width
    pub const DUMMY_WIDTH_D: u32 = 1 << 6;
    pub const DUMMY_WIDTH_D_VAL: u32 = 1;
    /// Q = 2
    /// Quad width
    pub const DUMMY_WIDTH_Q: u32 = 2 << 6;
    pub const DUMMY_WIDTH_Q_VAL: u32 = 2;

    /// **SUFFIX_WIDTH**
    /// The width used for the post-address command suffix, if any
    /// Bits `[5:4]`, mask `0x00000030`
    pub const SUFFIX_WIDTH_OFFSET: u32 = 4;
    pub const SUFFIX_WIDTH_MASK: u32 = 0x00000030;
    /// S = 0
    /// Single width
    pub const SUFFIX_WIDTH_S: u32 = 0 << 4;
    pub const SUFFIX_WIDTH_S_VAL: u32 = 0;
    /// D = 1
    /// Dual width
    pub const SUFFIX_WIDTH_D: u32 = 1 << 4;
    pub const SUFFIX_WIDTH_D_VAL: u32 = 1;
    /// Q = 2
    /// Quad width
    pub const SUFFIX_WIDTH_Q: u32 = 2 << 4;
    pub const SUFFIX_WIDTH_Q_VAL: u32 = 2;

    /// **ADDR_WIDTH**
    /// The transfer width used for the address. The address phase always transfers 24 bits in total.
    /// Bits `[3:2]`, mask `0x0000000c`
    pub const ADDR_WIDTH_OFFSET: u32 = 2;
    pub const ADDR_WIDTH_MASK: u32 = 0x0000000c;
    /// S = 0
    /// Single width
    pub const ADDR_WIDTH_S: u32 = 0 << 2;
    pub const ADDR_WIDTH_S_VAL: u32 = 0;
    /// D = 1
    /// Dual width
    pub const ADDR_WIDTH_D: u32 = 1 << 2;
    pub const ADDR_WIDTH_D_VAL: u32 = 1;
    /// Q = 2
    /// Quad width
    pub const ADDR_WIDTH_Q: u32 = 2 << 2;
    pub const ADDR_WIDTH_Q_VAL: u32 = 2;

    /// **PREFIX_WIDTH**
    /// The transfer width used for the command prefix, if any
    /// Bits `[1:0]`, mask `0x00000003`
    pub const PREFIX_WIDTH_OFFSET: u32 = 0;
    pub const PREFIX_WIDTH_MASK: u32 = 0x00000003;
    /// S = 0
    /// Single width
    pub const PREFIX_WIDTH_S: u32 = 0 << 0;
    pub const PREFIX_WIDTH_S_VAL: u32 = 0;
    /// D = 1
    /// Dual width
    pub const PREFIX_WIDTH_D: u32 = 1 << 0;
    pub const PREFIX_WIDTH_D_VAL: u32 = 1;
    /// Q = 2
    /// Quad width
    pub const PREFIX_WIDTH_Q: u32 = 2 << 0;
    pub const PREFIX_WIDTH_Q_VAL: u32 = 2;

    // ======== Field Accessors ========

    /// **DTR** `[28]` (RW)
    /// Enable double transfer rate (DTR) for write commands: address, suffix and write data phases are active on both edges of SCK. SDO data is launched centre-aligned on each SCK edge, and SDI data is captured on the SCK edge that follows its launch.
    /// DTR is implemented by halving the clock rate; SCK has a period of 2 x CLK_DIV throughout the transfer. The prefix and dummy phases are still single transfer rate.
    /// If the suffix is quad-width, it must be 0 or 8 bits in length, to ensure an even number of SCK edges.
    #[inline(always)]
    pub fn dtr(&self) -> bool {
        (self.value.read_bits() & Self::DTR_MASK) != 0
    }

    /// Set DTR bit atomically (RW)
    #[inline(always)]
    pub fn set_dtr(&self) {
        self.value.set_bits(Self::DTR_MASK);
    }

    /// Clear DTR bit atomically (RW)
    #[inline(always)]
    pub fn clr_dtr(&self) {
        self.value.clr_bits(Self::DTR_MASK);
    }

    /// **DUMMY_LEN** `[18:16]` (RW)
    /// Length of dummy phase between command suffix and data phase, in units of 4 bits. (i.e. 1 cycle for quad width, 2 for dual, 4 for single)
    #[inline(always)]
    pub fn dummy_len(&self) -> u32 {
        (self.value.read_bits() & Self::DUMMY_LEN_MASK) >> Self::DUMMY_LEN_OFFSET
    }

    /// Write **DUMMY_LEN** `[18:16]` (RW)
    /// Length of dummy phase between command suffix and data phase, in units of 4 bits. (i.e. 1 cycle for quad width, 2 for dual, 4 for single)
    #[inline(always)]
    pub fn write_dummy_len(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DUMMY_LEN_MASK) | ((value << Self::DUMMY_LEN_OFFSET) & Self::DUMMY_LEN_MASK));
    }

    /// **SUFFIX_LEN** `[15:14]` (RW)
    /// Length of post-address command suffix, in units of 4 bits. (i.e. 1 cycle for quad width, 2 for dual, 4 for single)
    /// Only values of 0 and 8 bits are supported.
    #[inline(always)]
    pub fn suffix_len(&self) -> u32 {
        (self.value.read_bits() & Self::SUFFIX_LEN_MASK) >> Self::SUFFIX_LEN_OFFSET
    }

    /// Write **SUFFIX_LEN** `[15:14]` (RW)
    /// Length of post-address command suffix, in units of 4 bits. (i.e. 1 cycle for quad width, 2 for dual, 4 for single)
    /// Only values of 0 and 8 bits are supported.
    #[inline(always)]
    pub fn write_suffix_len(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SUFFIX_LEN_MASK) | ((value << Self::SUFFIX_LEN_OFFSET) & Self::SUFFIX_LEN_MASK));
    }

    /// **PREFIX_LEN** `[12]` (RW)
    /// Length of command prefix, in units of 8 bits. (i.e. 2 cycles for quad width, 4 for dual, 8 for single)
    #[inline(always)]
    pub fn prefix_len(&self) -> bool {
        (self.value.read_bits() & Self::PREFIX_LEN_MASK) != 0
    }

    /// Set PREFIX_LEN bit atomically (RW)
    #[inline(always)]
    pub fn set_prefix_len(&self) {
        self.value.set_bits(Self::PREFIX_LEN_MASK);
    }

    /// Clear PREFIX_LEN bit atomically (RW)
    #[inline(always)]
    pub fn clr_prefix_len(&self) {
        self.value.clr_bits(Self::PREFIX_LEN_MASK);
    }

    /// **DATA_WIDTH** `[9:8]` (RW)
    /// The width used for the data transfer
    #[inline(always)]
    pub fn data_width(&self) -> u32 {
        (self.value.read_bits() & Self::DATA_WIDTH_MASK) >> Self::DATA_WIDTH_OFFSET
    }

    /// Write **DATA_WIDTH** `[9:8]` (RW)
    /// The width used for the data transfer
    #[inline(always)]
    pub fn write_data_width(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DATA_WIDTH_MASK) | ((value << Self::DATA_WIDTH_OFFSET) & Self::DATA_WIDTH_MASK));
    }

    /// **DUMMY_WIDTH** `[7:6]` (RW)
    /// The width used for the dummy phase, if any.
    /// If width is single, SD0/MOSI is held asserted low during the dummy phase, and SD1...SD3 are tristated. If width is dual/quad, all IOs are tristated during the dummy phase.
    #[inline(always)]
    pub fn dummy_width(&self) -> u32 {
        (self.value.read_bits() & Self::DUMMY_WIDTH_MASK) >> Self::DUMMY_WIDTH_OFFSET
    }

    /// Write **DUMMY_WIDTH** `[7:6]` (RW)
    /// The width used for the dummy phase, if any.
    /// If width is single, SD0/MOSI is held asserted low during the dummy phase, and SD1...SD3 are tristated. If width is dual/quad, all IOs are tristated during the dummy phase.
    #[inline(always)]
    pub fn write_dummy_width(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DUMMY_WIDTH_MASK) | ((value << Self::DUMMY_WIDTH_OFFSET) & Self::DUMMY_WIDTH_MASK));
    }

    /// **SUFFIX_WIDTH** `[5:4]` (RW)
    /// The width used for the post-address command suffix, if any
    #[inline(always)]
    pub fn suffix_width(&self) -> u32 {
        (self.value.read_bits() & Self::SUFFIX_WIDTH_MASK) >> Self::SUFFIX_WIDTH_OFFSET
    }

    /// Write **SUFFIX_WIDTH** `[5:4]` (RW)
    /// The width used for the post-address command suffix, if any
    #[inline(always)]
    pub fn write_suffix_width(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SUFFIX_WIDTH_MASK) | ((value << Self::SUFFIX_WIDTH_OFFSET) & Self::SUFFIX_WIDTH_MASK));
    }

    /// **ADDR_WIDTH** `[3:2]` (RW)
    /// The transfer width used for the address. The address phase always transfers 24 bits in total.
    #[inline(always)]
    pub fn addr_width(&self) -> u32 {
        (self.value.read_bits() & Self::ADDR_WIDTH_MASK) >> Self::ADDR_WIDTH_OFFSET
    }

    /// Write **ADDR_WIDTH** `[3:2]` (RW)
    /// The transfer width used for the address. The address phase always transfers 24 bits in total.
    #[inline(always)]
    pub fn write_addr_width(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::ADDR_WIDTH_MASK) | ((value << Self::ADDR_WIDTH_OFFSET) & Self::ADDR_WIDTH_MASK));
    }

    /// **PREFIX_WIDTH** `[1:0]` (RW)
    /// The transfer width used for the command prefix, if any
    #[inline(always)]
    pub fn prefix_width(&self) -> u32 {
        (self.value.read_bits() & Self::PREFIX_WIDTH_MASK) >> Self::PREFIX_WIDTH_OFFSET
    }

    /// Write **PREFIX_WIDTH** `[1:0]` (RW)
    /// The transfer width used for the command prefix, if any
    #[inline(always)]
    pub fn write_prefix_width(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::PREFIX_WIDTH_MASK) | ((value << Self::PREFIX_WIDTH_OFFSET) & Self::PREFIX_WIDTH_MASK));
    }

}

/// M1_WFMT value type for batch read/modify/write operations
///
/// Use `m1wfmt.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `m1wfmt.write_val(val)`.
#[derive(Clone, Copy)]
pub struct M1WfmtVal {
    bits: u32,
}

impl M1WfmtVal {
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
        Self { bits: 0x00001000 }
    }

    /// Read **DTR** `[28:28]`
    /// Enable double transfer rate (DTR) for write commands: address, suffix and write data phases are active on both edges of SCK. SDO data is launched centre-aligned on each SCK edge, and SDI data is captured on the SCK edge that follows its launch.
    /// DTR is implemented by halving the clock rate; SCK has a period of 2 x CLK_DIV throughout the transfer. The prefix and dummy phases are still single transfer rate.
    /// If the suffix is quad-width, it must be 0 or 8 bits in length, to ensure an even number of SCK edges.
    #[inline(always)]
    pub const fn dtr(&self) -> bool {
        (self.bits & M1Wfmt::DTR_MASK) != 0
    }

    /// Read **DUMMY_LEN** `[18:16]`
    /// Length of dummy phase between command suffix and data phase, in units of 4 bits. (i.e. 1 cycle for quad width, 2 for dual, 4 for single)
    #[inline(always)]
    pub const fn dummy_len(&self) -> u32 {
        (self.bits & M1Wfmt::DUMMY_LEN_MASK) >> M1Wfmt::DUMMY_LEN_OFFSET
    }

    /// Read **SUFFIX_LEN** `[15:14]`
    /// Length of post-address command suffix, in units of 4 bits. (i.e. 1 cycle for quad width, 2 for dual, 4 for single)
    /// Only values of 0 and 8 bits are supported.
    #[inline(always)]
    pub const fn suffix_len(&self) -> u32 {
        (self.bits & M1Wfmt::SUFFIX_LEN_MASK) >> M1Wfmt::SUFFIX_LEN_OFFSET
    }

    /// Read **PREFIX_LEN** `[12:12]`
    /// Length of command prefix, in units of 8 bits. (i.e. 2 cycles for quad width, 4 for dual, 8 for single)
    #[inline(always)]
    pub const fn prefix_len(&self) -> bool {
        (self.bits & M1Wfmt::PREFIX_LEN_MASK) != 0
    }

    /// Read **DATA_WIDTH** `[9:8]`
    /// The width used for the data transfer
    #[inline(always)]
    pub const fn data_width(&self) -> u32 {
        (self.bits & M1Wfmt::DATA_WIDTH_MASK) >> M1Wfmt::DATA_WIDTH_OFFSET
    }

    /// Read **DUMMY_WIDTH** `[7:6]`
    /// The width used for the dummy phase, if any.
    /// If width is single, SD0/MOSI is held asserted low during the dummy phase, and SD1...SD3 are tristated. If width is dual/quad, all IOs are tristated during the dummy phase.
    #[inline(always)]
    pub const fn dummy_width(&self) -> u32 {
        (self.bits & M1Wfmt::DUMMY_WIDTH_MASK) >> M1Wfmt::DUMMY_WIDTH_OFFSET
    }

    /// Read **SUFFIX_WIDTH** `[5:4]`
    /// The width used for the post-address command suffix, if any
    #[inline(always)]
    pub const fn suffix_width(&self) -> u32 {
        (self.bits & M1Wfmt::SUFFIX_WIDTH_MASK) >> M1Wfmt::SUFFIX_WIDTH_OFFSET
    }

    /// Read **ADDR_WIDTH** `[3:2]`
    /// The transfer width used for the address. The address phase always transfers 24 bits in total.
    #[inline(always)]
    pub const fn addr_width(&self) -> u32 {
        (self.bits & M1Wfmt::ADDR_WIDTH_MASK) >> M1Wfmt::ADDR_WIDTH_OFFSET
    }

    /// Read **PREFIX_WIDTH** `[1:0]`
    /// The transfer width used for the command prefix, if any
    #[inline(always)]
    pub const fn prefix_width(&self) -> u32 {
        (self.bits & M1Wfmt::PREFIX_WIDTH_MASK) >> M1Wfmt::PREFIX_WIDTH_OFFSET
    }

    /// Set **DTR** `[28:28]`
    /// Enable double transfer rate (DTR) for write commands: address, suffix and write data phases are active on both edges of SCK. SDO data is launched centre-aligned on each SCK edge, and SDI data is captured on the SCK edge that follows its launch.
    /// DTR is implemented by halving the clock rate; SCK has a period of 2 x CLK_DIV throughout the transfer. The prefix and dummy phases are still single transfer rate.
    /// If the suffix is quad-width, it must be 0 or 8 bits in length, to ensure an even number of SCK edges.
    #[inline(always)]
    pub const fn with_dtr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | M1Wfmt::DTR_MASK
            } else {
                self.bits & !M1Wfmt::DTR_MASK
            }
        }
    }

    /// Set **DUMMY_LEN** `[18:16]`
    /// Length of dummy phase between command suffix and data phase, in units of 4 bits. (i.e. 1 cycle for quad width, 2 for dual, 4 for single)
    #[inline(always)]
    pub const fn with_dummy_len(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !M1Wfmt::DUMMY_LEN_MASK) | ((value << M1Wfmt::DUMMY_LEN_OFFSET) & M1Wfmt::DUMMY_LEN_MASK)
        }
    }

    /// Set **SUFFIX_LEN** `[15:14]`
    /// Length of post-address command suffix, in units of 4 bits. (i.e. 1 cycle for quad width, 2 for dual, 4 for single)
    /// Only values of 0 and 8 bits are supported.
    #[inline(always)]
    pub const fn with_suffix_len(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !M1Wfmt::SUFFIX_LEN_MASK) | ((value << M1Wfmt::SUFFIX_LEN_OFFSET) & M1Wfmt::SUFFIX_LEN_MASK)
        }
    }

    /// Set **PREFIX_LEN** `[12:12]`
    /// Length of command prefix, in units of 8 bits. (i.e. 2 cycles for quad width, 4 for dual, 8 for single)
    #[inline(always)]
    pub const fn with_prefix_len(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | M1Wfmt::PREFIX_LEN_MASK
            } else {
                self.bits & !M1Wfmt::PREFIX_LEN_MASK
            }
        }
    }

    /// Set **DATA_WIDTH** `[9:8]`
    /// The width used for the data transfer
    #[inline(always)]
    pub const fn with_data_width(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !M1Wfmt::DATA_WIDTH_MASK) | ((value << M1Wfmt::DATA_WIDTH_OFFSET) & M1Wfmt::DATA_WIDTH_MASK)
        }
    }

    /// Set **DUMMY_WIDTH** `[7:6]`
    /// The width used for the dummy phase, if any.
    /// If width is single, SD0/MOSI is held asserted low during the dummy phase, and SD1...SD3 are tristated. If width is dual/quad, all IOs are tristated during the dummy phase.
    #[inline(always)]
    pub const fn with_dummy_width(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !M1Wfmt::DUMMY_WIDTH_MASK) | ((value << M1Wfmt::DUMMY_WIDTH_OFFSET) & M1Wfmt::DUMMY_WIDTH_MASK)
        }
    }

    /// Set **SUFFIX_WIDTH** `[5:4]`
    /// The width used for the post-address command suffix, if any
    #[inline(always)]
    pub const fn with_suffix_width(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !M1Wfmt::SUFFIX_WIDTH_MASK) | ((value << M1Wfmt::SUFFIX_WIDTH_OFFSET) & M1Wfmt::SUFFIX_WIDTH_MASK)
        }
    }

    /// Set **ADDR_WIDTH** `[3:2]`
    /// The transfer width used for the address. The address phase always transfers 24 bits in total.
    #[inline(always)]
    pub const fn with_addr_width(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !M1Wfmt::ADDR_WIDTH_MASK) | ((value << M1Wfmt::ADDR_WIDTH_OFFSET) & M1Wfmt::ADDR_WIDTH_MASK)
        }
    }

    /// Set **PREFIX_WIDTH** `[1:0]`
    /// The transfer width used for the command prefix, if any
    #[inline(always)]
    pub const fn with_prefix_width(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !M1Wfmt::PREFIX_WIDTH_MASK) | ((value << M1Wfmt::PREFIX_WIDTH_OFFSET) & M1Wfmt::PREFIX_WIDTH_MASK)
        }
    }

}

impl Default for M1WfmtVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl M1Wfmt {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> M1WfmtVal {
        M1WfmtVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: M1WfmtVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(M1WfmtVal) -> M1WfmtVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// M1_WCMD Register
///
/// Command constants used for writes to memory address window 1.
/// The reset value of the M1_WCMD register is configured to support a basic 02h serial write transfer with no additional configuration.
///
/// Address offset: 0x0030
/// Reset value: 0x0000a002
///
/// ## Fields
///
/// - **SUFFIX** `[15:8]` (RW)
///   The command suffix bits following the address, if Mx_WFMT_SUFFIX_LEN is nonzero.
/// - **PREFIX** `[7:0]` (RW)
///   The command prefix bits to prepend on each new transfer, if Mx_WFMT_PREFIX_LEN is nonzero.
#[repr(transparent)]
pub struct M1Wcmd {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for M1Wcmd {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl M1Wcmd {
    // ======== Field Constants ========

    /// **SUFFIX**
    /// The command suffix bits following the address, if Mx_WFMT_SUFFIX_LEN is nonzero.
    /// Bits `[15:8]`, mask `0x0000ff00`
    pub const SUFFIX_OFFSET: u32 = 8;
    pub const SUFFIX_MASK: u32 = 0x0000ff00;

    /// **PREFIX**
    /// The command prefix bits to prepend on each new transfer, if Mx_WFMT_PREFIX_LEN is nonzero.
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const PREFIX_OFFSET: u32 = 0;
    pub const PREFIX_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **SUFFIX** `[15:8]` (RW)
    /// The command suffix bits following the address, if Mx_WFMT_SUFFIX_LEN is nonzero.
    #[inline(always)]
    pub fn suffix(&self) -> u32 {
        (self.value.read_bits() & Self::SUFFIX_MASK) >> Self::SUFFIX_OFFSET
    }

    /// Write **SUFFIX** `[15:8]` (RW)
    /// The command suffix bits following the address, if Mx_WFMT_SUFFIX_LEN is nonzero.
    #[inline(always)]
    pub fn write_suffix(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SUFFIX_MASK) | ((value << Self::SUFFIX_OFFSET) & Self::SUFFIX_MASK));
    }

    /// **PREFIX** `[7:0]` (RW)
    /// The command prefix bits to prepend on each new transfer, if Mx_WFMT_PREFIX_LEN is nonzero.
    #[inline(always)]
    pub fn prefix(&self) -> u32 {
        (self.value.read_bits() & Self::PREFIX_MASK) >> Self::PREFIX_OFFSET
    }

    /// Write **PREFIX** `[7:0]` (RW)
    /// The command prefix bits to prepend on each new transfer, if Mx_WFMT_PREFIX_LEN is nonzero.
    #[inline(always)]
    pub fn write_prefix(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::PREFIX_MASK) | ((value << Self::PREFIX_OFFSET) & Self::PREFIX_MASK));
    }

}

/// M1_WCMD value type for batch read/modify/write operations
///
/// Use `m1wcmd.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `m1wcmd.write_val(val)`.
#[derive(Clone, Copy)]
pub struct M1WcmdVal {
    bits: u32,
}

impl M1WcmdVal {
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
        Self { bits: 0x0000a002 }
    }

    /// Read **SUFFIX** `[15:8]`
    /// The command suffix bits following the address, if Mx_WFMT_SUFFIX_LEN is nonzero.
    #[inline(always)]
    pub const fn suffix(&self) -> u32 {
        (self.bits & M1Wcmd::SUFFIX_MASK) >> M1Wcmd::SUFFIX_OFFSET
    }

    /// Read **PREFIX** `[7:0]`
    /// The command prefix bits to prepend on each new transfer, if Mx_WFMT_PREFIX_LEN is nonzero.
    #[inline(always)]
    pub const fn prefix(&self) -> u32 {
        (self.bits & M1Wcmd::PREFIX_MASK) >> M1Wcmd::PREFIX_OFFSET
    }

    /// Set **SUFFIX** `[15:8]`
    /// The command suffix bits following the address, if Mx_WFMT_SUFFIX_LEN is nonzero.
    #[inline(always)]
    pub const fn with_suffix(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !M1Wcmd::SUFFIX_MASK) | ((value << M1Wcmd::SUFFIX_OFFSET) & M1Wcmd::SUFFIX_MASK)
        }
    }

    /// Set **PREFIX** `[7:0]`
    /// The command prefix bits to prepend on each new transfer, if Mx_WFMT_PREFIX_LEN is nonzero.
    #[inline(always)]
    pub const fn with_prefix(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !M1Wcmd::PREFIX_MASK) | ((value << M1Wcmd::PREFIX_OFFSET) & M1Wcmd::PREFIX_MASK)
        }
    }

}

impl Default for M1WcmdVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl M1Wcmd {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> M1WcmdVal {
        M1WcmdVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: M1WcmdVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(M1WcmdVal) -> M1WcmdVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// ATRANS0 Register
///
/// Configure address translation for XIP virtual addresses 0x000000 through 0x3fffff (a 4 MiB window starting at +0 MiB).
/// Address translation allows a program image to be executed in place at multiple physical flash addresses (for example, a double-buffered flash image for over-the-air updates), without the overhead of position-independent code.
/// At reset, the address translation registers are initialised to an identity mapping, so that they can be ignored if address translation is not required.
/// Note that the XIP cache is fully virtually addressed, so a cache flush is required after changing the address translation.
///
/// Address offset: 0x0034
/// Reset value: 0x04000000
///
/// ## Fields
///
/// - **SIZE** `[26:16]` (RW)
///   Translation aperture size for this virtual address range, in units of 4 kiB (one flash sector).
///   Bits 21:12 of the virtual address are compared to SIZE. Offsets greater than SIZE return a bus error, and do not cause a QSPI access.
/// - **BASE** `[11:0]` (RW)
///   Physical address base for this virtual address range, in units of 4 kiB (one flash sector).
///   Taking a 24-bit virtual address, firstly bits 23:22 (the two MSBs) are masked to zero, and then BASE is added to bits 23:12 (the upper 12 bits) to form the physical address. Translation wraps on a 16 MiB boundary.
#[repr(transparent)]
pub struct Atrans0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Atrans0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Atrans0 {
    // ======== Field Constants ========

    /// **SIZE**
    /// Translation aperture size for this virtual address range, in units of 4 kiB (one flash sector).
    /// Bits 21:12 of the virtual address are compared to SIZE. Offsets greater than SIZE return a bus error, and do not cause a QSPI access.
    /// Bits `[26:16]`, mask `0x07ff0000`
    pub const SIZE_OFFSET: u32 = 16;
    pub const SIZE_MASK: u32 = 0x07ff0000;

    /// **BASE**
    /// Physical address base for this virtual address range, in units of 4 kiB (one flash sector).
    /// Taking a 24-bit virtual address, firstly bits 23:22 (the two MSBs) are masked to zero, and then BASE is added to bits 23:12 (the upper 12 bits) to form the physical address. Translation wraps on a 16 MiB boundary.
    /// Bits `[11:0]`, mask `0x00000fff`
    pub const BASE_OFFSET: u32 = 0;
    pub const BASE_MASK: u32 = 0x00000fff;

    // ======== Field Accessors ========

    /// **SIZE** `[26:16]` (RW)
    /// Translation aperture size for this virtual address range, in units of 4 kiB (one flash sector).
    /// Bits 21:12 of the virtual address are compared to SIZE. Offsets greater than SIZE return a bus error, and do not cause a QSPI access.
    #[inline(always)]
    pub fn size(&self) -> u32 {
        (self.value.read_bits() & Self::SIZE_MASK) >> Self::SIZE_OFFSET
    }

    /// Write **SIZE** `[26:16]` (RW)
    /// Translation aperture size for this virtual address range, in units of 4 kiB (one flash sector).
    /// Bits 21:12 of the virtual address are compared to SIZE. Offsets greater than SIZE return a bus error, and do not cause a QSPI access.
    #[inline(always)]
    pub fn write_size(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SIZE_MASK) | ((value << Self::SIZE_OFFSET) & Self::SIZE_MASK));
    }

    /// **BASE** `[11:0]` (RW)
    /// Physical address base for this virtual address range, in units of 4 kiB (one flash sector).
    /// Taking a 24-bit virtual address, firstly bits 23:22 (the two MSBs) are masked to zero, and then BASE is added to bits 23:12 (the upper 12 bits) to form the physical address. Translation wraps on a 16 MiB boundary.
    #[inline(always)]
    pub fn base(&self) -> u32 {
        (self.value.read_bits() & Self::BASE_MASK) >> Self::BASE_OFFSET
    }

    /// Write **BASE** `[11:0]` (RW)
    /// Physical address base for this virtual address range, in units of 4 kiB (one flash sector).
    /// Taking a 24-bit virtual address, firstly bits 23:22 (the two MSBs) are masked to zero, and then BASE is added to bits 23:12 (the upper 12 bits) to form the physical address. Translation wraps on a 16 MiB boundary.
    #[inline(always)]
    pub fn write_base(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::BASE_MASK) | ((value << Self::BASE_OFFSET) & Self::BASE_MASK));
    }

}

/// ATRANS0 value type for batch read/modify/write operations
///
/// Use `atrans0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `atrans0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Atrans0Val {
    bits: u32,
}

impl Atrans0Val {
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
        Self { bits: 0x04000000 }
    }

    /// Read **SIZE** `[26:16]`
    /// Translation aperture size for this virtual address range, in units of 4 kiB (one flash sector).
    /// Bits 21:12 of the virtual address are compared to SIZE. Offsets greater than SIZE return a bus error, and do not cause a QSPI access.
    #[inline(always)]
    pub const fn size(&self) -> u32 {
        (self.bits & Atrans0::SIZE_MASK) >> Atrans0::SIZE_OFFSET
    }

    /// Read **BASE** `[11:0]`
    /// Physical address base for this virtual address range, in units of 4 kiB (one flash sector).
    /// Taking a 24-bit virtual address, firstly bits 23:22 (the two MSBs) are masked to zero, and then BASE is added to bits 23:12 (the upper 12 bits) to form the physical address. Translation wraps on a 16 MiB boundary.
    #[inline(always)]
    pub const fn base(&self) -> u32 {
        (self.bits & Atrans0::BASE_MASK) >> Atrans0::BASE_OFFSET
    }

    /// Set **SIZE** `[26:16]`
    /// Translation aperture size for this virtual address range, in units of 4 kiB (one flash sector).
    /// Bits 21:12 of the virtual address are compared to SIZE. Offsets greater than SIZE return a bus error, and do not cause a QSPI access.
    #[inline(always)]
    pub const fn with_size(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Atrans0::SIZE_MASK) | ((value << Atrans0::SIZE_OFFSET) & Atrans0::SIZE_MASK)
        }
    }

    /// Set **BASE** `[11:0]`
    /// Physical address base for this virtual address range, in units of 4 kiB (one flash sector).
    /// Taking a 24-bit virtual address, firstly bits 23:22 (the two MSBs) are masked to zero, and then BASE is added to bits 23:12 (the upper 12 bits) to form the physical address. Translation wraps on a 16 MiB boundary.
    #[inline(always)]
    pub const fn with_base(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Atrans0::BASE_MASK) | ((value << Atrans0::BASE_OFFSET) & Atrans0::BASE_MASK)
        }
    }

}

impl Default for Atrans0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Atrans0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Atrans0Val {
        Atrans0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Atrans0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Atrans0Val) -> Atrans0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// ATRANS1 Register
///
/// Configure address translation for XIP virtual addresses 0x400000 through 0x7fffff (a 4 MiB window starting at +4 MiB).
/// Address translation allows a program image to be executed in place at multiple physical flash addresses (for example, a double-buffered flash image for over-the-air updates), without the overhead of position-independent code.
/// At reset, the address translation registers are initialised to an identity mapping, so that they can be ignored if address translation is not required.
/// Note that the XIP cache is fully virtually addressed, so a cache flush is required after changing the address translation.
///
/// Address offset: 0x0038
/// Reset value: 0x04000400
///
/// ## Fields
///
/// - **SIZE** `[26:16]` (RW)
///   Translation aperture size for this virtual address range, in units of 4 kiB (one flash sector).
///   Bits 21:12 of the virtual address are compared to SIZE. Offsets greater than SIZE return a bus error, and do not cause a QSPI access.
/// - **BASE** `[11:0]` (RW)
///   Physical address base for this virtual address range, in units of 4 kiB (one flash sector).
///   Taking a 24-bit virtual address, firstly bits 23:22 (the two MSBs) are masked to zero, and then BASE is added to bits 23:12 (the upper 12 bits) to form the physical address. Translation wraps on a 16 MiB boundary.
#[repr(transparent)]
pub struct Atrans1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Atrans1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Atrans1 {
    // ======== Field Constants ========

    /// **SIZE**
    /// Translation aperture size for this virtual address range, in units of 4 kiB (one flash sector).
    /// Bits 21:12 of the virtual address are compared to SIZE. Offsets greater than SIZE return a bus error, and do not cause a QSPI access.
    /// Bits `[26:16]`, mask `0x07ff0000`
    pub const SIZE_OFFSET: u32 = 16;
    pub const SIZE_MASK: u32 = 0x07ff0000;

    /// **BASE**
    /// Physical address base for this virtual address range, in units of 4 kiB (one flash sector).
    /// Taking a 24-bit virtual address, firstly bits 23:22 (the two MSBs) are masked to zero, and then BASE is added to bits 23:12 (the upper 12 bits) to form the physical address. Translation wraps on a 16 MiB boundary.
    /// Bits `[11:0]`, mask `0x00000fff`
    pub const BASE_OFFSET: u32 = 0;
    pub const BASE_MASK: u32 = 0x00000fff;

    // ======== Field Accessors ========

    /// **SIZE** `[26:16]` (RW)
    /// Translation aperture size for this virtual address range, in units of 4 kiB (one flash sector).
    /// Bits 21:12 of the virtual address are compared to SIZE. Offsets greater than SIZE return a bus error, and do not cause a QSPI access.
    #[inline(always)]
    pub fn size(&self) -> u32 {
        (self.value.read_bits() & Self::SIZE_MASK) >> Self::SIZE_OFFSET
    }

    /// Write **SIZE** `[26:16]` (RW)
    /// Translation aperture size for this virtual address range, in units of 4 kiB (one flash sector).
    /// Bits 21:12 of the virtual address are compared to SIZE. Offsets greater than SIZE return a bus error, and do not cause a QSPI access.
    #[inline(always)]
    pub fn write_size(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SIZE_MASK) | ((value << Self::SIZE_OFFSET) & Self::SIZE_MASK));
    }

    /// **BASE** `[11:0]` (RW)
    /// Physical address base for this virtual address range, in units of 4 kiB (one flash sector).
    /// Taking a 24-bit virtual address, firstly bits 23:22 (the two MSBs) are masked to zero, and then BASE is added to bits 23:12 (the upper 12 bits) to form the physical address. Translation wraps on a 16 MiB boundary.
    #[inline(always)]
    pub fn base(&self) -> u32 {
        (self.value.read_bits() & Self::BASE_MASK) >> Self::BASE_OFFSET
    }

    /// Write **BASE** `[11:0]` (RW)
    /// Physical address base for this virtual address range, in units of 4 kiB (one flash sector).
    /// Taking a 24-bit virtual address, firstly bits 23:22 (the two MSBs) are masked to zero, and then BASE is added to bits 23:12 (the upper 12 bits) to form the physical address. Translation wraps on a 16 MiB boundary.
    #[inline(always)]
    pub fn write_base(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::BASE_MASK) | ((value << Self::BASE_OFFSET) & Self::BASE_MASK));
    }

}

/// ATRANS1 value type for batch read/modify/write operations
///
/// Use `atrans1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `atrans1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Atrans1Val {
    bits: u32,
}

impl Atrans1Val {
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
        Self { bits: 0x04000400 }
    }

    /// Read **SIZE** `[26:16]`
    /// Translation aperture size for this virtual address range, in units of 4 kiB (one flash sector).
    /// Bits 21:12 of the virtual address are compared to SIZE. Offsets greater than SIZE return a bus error, and do not cause a QSPI access.
    #[inline(always)]
    pub const fn size(&self) -> u32 {
        (self.bits & Atrans1::SIZE_MASK) >> Atrans1::SIZE_OFFSET
    }

    /// Read **BASE** `[11:0]`
    /// Physical address base for this virtual address range, in units of 4 kiB (one flash sector).
    /// Taking a 24-bit virtual address, firstly bits 23:22 (the two MSBs) are masked to zero, and then BASE is added to bits 23:12 (the upper 12 bits) to form the physical address. Translation wraps on a 16 MiB boundary.
    #[inline(always)]
    pub const fn base(&self) -> u32 {
        (self.bits & Atrans1::BASE_MASK) >> Atrans1::BASE_OFFSET
    }

    /// Set **SIZE** `[26:16]`
    /// Translation aperture size for this virtual address range, in units of 4 kiB (one flash sector).
    /// Bits 21:12 of the virtual address are compared to SIZE. Offsets greater than SIZE return a bus error, and do not cause a QSPI access.
    #[inline(always)]
    pub const fn with_size(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Atrans1::SIZE_MASK) | ((value << Atrans1::SIZE_OFFSET) & Atrans1::SIZE_MASK)
        }
    }

    /// Set **BASE** `[11:0]`
    /// Physical address base for this virtual address range, in units of 4 kiB (one flash sector).
    /// Taking a 24-bit virtual address, firstly bits 23:22 (the two MSBs) are masked to zero, and then BASE is added to bits 23:12 (the upper 12 bits) to form the physical address. Translation wraps on a 16 MiB boundary.
    #[inline(always)]
    pub const fn with_base(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Atrans1::BASE_MASK) | ((value << Atrans1::BASE_OFFSET) & Atrans1::BASE_MASK)
        }
    }

}

impl Default for Atrans1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Atrans1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Atrans1Val {
        Atrans1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Atrans1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Atrans1Val) -> Atrans1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// ATRANS2 Register
///
/// Configure address translation for XIP virtual addresses 0x800000 through 0xbfffff (a 4 MiB window starting at +8 MiB).
/// Address translation allows a program image to be executed in place at multiple physical flash addresses (for example, a double-buffered flash image for over-the-air updates), without the overhead of position-independent code.
/// At reset, the address translation registers are initialised to an identity mapping, so that they can be ignored if address translation is not required.
/// Note that the XIP cache is fully virtually addressed, so a cache flush is required after changing the address translation.
///
/// Address offset: 0x003c
/// Reset value: 0x04000800
///
/// ## Fields
///
/// - **SIZE** `[26:16]` (RW)
///   Translation aperture size for this virtual address range, in units of 4 kiB (one flash sector).
///   Bits 21:12 of the virtual address are compared to SIZE. Offsets greater than SIZE return a bus error, and do not cause a QSPI access.
/// - **BASE** `[11:0]` (RW)
///   Physical address base for this virtual address range, in units of 4 kiB (one flash sector).
///   Taking a 24-bit virtual address, firstly bits 23:22 (the two MSBs) are masked to zero, and then BASE is added to bits 23:12 (the upper 12 bits) to form the physical address. Translation wraps on a 16 MiB boundary.
#[repr(transparent)]
pub struct Atrans2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Atrans2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Atrans2 {
    // ======== Field Constants ========

    /// **SIZE**
    /// Translation aperture size for this virtual address range, in units of 4 kiB (one flash sector).
    /// Bits 21:12 of the virtual address are compared to SIZE. Offsets greater than SIZE return a bus error, and do not cause a QSPI access.
    /// Bits `[26:16]`, mask `0x07ff0000`
    pub const SIZE_OFFSET: u32 = 16;
    pub const SIZE_MASK: u32 = 0x07ff0000;

    /// **BASE**
    /// Physical address base for this virtual address range, in units of 4 kiB (one flash sector).
    /// Taking a 24-bit virtual address, firstly bits 23:22 (the two MSBs) are masked to zero, and then BASE is added to bits 23:12 (the upper 12 bits) to form the physical address. Translation wraps on a 16 MiB boundary.
    /// Bits `[11:0]`, mask `0x00000fff`
    pub const BASE_OFFSET: u32 = 0;
    pub const BASE_MASK: u32 = 0x00000fff;

    // ======== Field Accessors ========

    /// **SIZE** `[26:16]` (RW)
    /// Translation aperture size for this virtual address range, in units of 4 kiB (one flash sector).
    /// Bits 21:12 of the virtual address are compared to SIZE. Offsets greater than SIZE return a bus error, and do not cause a QSPI access.
    #[inline(always)]
    pub fn size(&self) -> u32 {
        (self.value.read_bits() & Self::SIZE_MASK) >> Self::SIZE_OFFSET
    }

    /// Write **SIZE** `[26:16]` (RW)
    /// Translation aperture size for this virtual address range, in units of 4 kiB (one flash sector).
    /// Bits 21:12 of the virtual address are compared to SIZE. Offsets greater than SIZE return a bus error, and do not cause a QSPI access.
    #[inline(always)]
    pub fn write_size(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SIZE_MASK) | ((value << Self::SIZE_OFFSET) & Self::SIZE_MASK));
    }

    /// **BASE** `[11:0]` (RW)
    /// Physical address base for this virtual address range, in units of 4 kiB (one flash sector).
    /// Taking a 24-bit virtual address, firstly bits 23:22 (the two MSBs) are masked to zero, and then BASE is added to bits 23:12 (the upper 12 bits) to form the physical address. Translation wraps on a 16 MiB boundary.
    #[inline(always)]
    pub fn base(&self) -> u32 {
        (self.value.read_bits() & Self::BASE_MASK) >> Self::BASE_OFFSET
    }

    /// Write **BASE** `[11:0]` (RW)
    /// Physical address base for this virtual address range, in units of 4 kiB (one flash sector).
    /// Taking a 24-bit virtual address, firstly bits 23:22 (the two MSBs) are masked to zero, and then BASE is added to bits 23:12 (the upper 12 bits) to form the physical address. Translation wraps on a 16 MiB boundary.
    #[inline(always)]
    pub fn write_base(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::BASE_MASK) | ((value << Self::BASE_OFFSET) & Self::BASE_MASK));
    }

}

/// ATRANS2 value type for batch read/modify/write operations
///
/// Use `atrans2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `atrans2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Atrans2Val {
    bits: u32,
}

impl Atrans2Val {
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
        Self { bits: 0x04000800 }
    }

    /// Read **SIZE** `[26:16]`
    /// Translation aperture size for this virtual address range, in units of 4 kiB (one flash sector).
    /// Bits 21:12 of the virtual address are compared to SIZE. Offsets greater than SIZE return a bus error, and do not cause a QSPI access.
    #[inline(always)]
    pub const fn size(&self) -> u32 {
        (self.bits & Atrans2::SIZE_MASK) >> Atrans2::SIZE_OFFSET
    }

    /// Read **BASE** `[11:0]`
    /// Physical address base for this virtual address range, in units of 4 kiB (one flash sector).
    /// Taking a 24-bit virtual address, firstly bits 23:22 (the two MSBs) are masked to zero, and then BASE is added to bits 23:12 (the upper 12 bits) to form the physical address. Translation wraps on a 16 MiB boundary.
    #[inline(always)]
    pub const fn base(&self) -> u32 {
        (self.bits & Atrans2::BASE_MASK) >> Atrans2::BASE_OFFSET
    }

    /// Set **SIZE** `[26:16]`
    /// Translation aperture size for this virtual address range, in units of 4 kiB (one flash sector).
    /// Bits 21:12 of the virtual address are compared to SIZE. Offsets greater than SIZE return a bus error, and do not cause a QSPI access.
    #[inline(always)]
    pub const fn with_size(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Atrans2::SIZE_MASK) | ((value << Atrans2::SIZE_OFFSET) & Atrans2::SIZE_MASK)
        }
    }

    /// Set **BASE** `[11:0]`
    /// Physical address base for this virtual address range, in units of 4 kiB (one flash sector).
    /// Taking a 24-bit virtual address, firstly bits 23:22 (the two MSBs) are masked to zero, and then BASE is added to bits 23:12 (the upper 12 bits) to form the physical address. Translation wraps on a 16 MiB boundary.
    #[inline(always)]
    pub const fn with_base(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Atrans2::BASE_MASK) | ((value << Atrans2::BASE_OFFSET) & Atrans2::BASE_MASK)
        }
    }

}

impl Default for Atrans2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Atrans2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Atrans2Val {
        Atrans2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Atrans2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Atrans2Val) -> Atrans2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// ATRANS3 Register
///
/// Configure address translation for XIP virtual addresses 0xc00000 through 0xffffff (a 4 MiB window starting at +12 MiB).
/// Address translation allows a program image to be executed in place at multiple physical flash addresses (for example, a double-buffered flash image for over-the-air updates), without the overhead of position-independent code.
/// At reset, the address translation registers are initialised to an identity mapping, so that they can be ignored if address translation is not required.
/// Note that the XIP cache is fully virtually addressed, so a cache flush is required after changing the address translation.
///
/// Address offset: 0x0040
/// Reset value: 0x04000c00
///
/// ## Fields
///
/// - **SIZE** `[26:16]` (RW)
///   Translation aperture size for this virtual address range, in units of 4 kiB (one flash sector).
///   Bits 21:12 of the virtual address are compared to SIZE. Offsets greater than SIZE return a bus error, and do not cause a QSPI access.
/// - **BASE** `[11:0]` (RW)
///   Physical address base for this virtual address range, in units of 4 kiB (one flash sector).
///   Taking a 24-bit virtual address, firstly bits 23:22 (the two MSBs) are masked to zero, and then BASE is added to bits 23:12 (the upper 12 bits) to form the physical address. Translation wraps on a 16 MiB boundary.
#[repr(transparent)]
pub struct Atrans3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Atrans3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Atrans3 {
    // ======== Field Constants ========

    /// **SIZE**
    /// Translation aperture size for this virtual address range, in units of 4 kiB (one flash sector).
    /// Bits 21:12 of the virtual address are compared to SIZE. Offsets greater than SIZE return a bus error, and do not cause a QSPI access.
    /// Bits `[26:16]`, mask `0x07ff0000`
    pub const SIZE_OFFSET: u32 = 16;
    pub const SIZE_MASK: u32 = 0x07ff0000;

    /// **BASE**
    /// Physical address base for this virtual address range, in units of 4 kiB (one flash sector).
    /// Taking a 24-bit virtual address, firstly bits 23:22 (the two MSBs) are masked to zero, and then BASE is added to bits 23:12 (the upper 12 bits) to form the physical address. Translation wraps on a 16 MiB boundary.
    /// Bits `[11:0]`, mask `0x00000fff`
    pub const BASE_OFFSET: u32 = 0;
    pub const BASE_MASK: u32 = 0x00000fff;

    // ======== Field Accessors ========

    /// **SIZE** `[26:16]` (RW)
    /// Translation aperture size for this virtual address range, in units of 4 kiB (one flash sector).
    /// Bits 21:12 of the virtual address are compared to SIZE. Offsets greater than SIZE return a bus error, and do not cause a QSPI access.
    #[inline(always)]
    pub fn size(&self) -> u32 {
        (self.value.read_bits() & Self::SIZE_MASK) >> Self::SIZE_OFFSET
    }

    /// Write **SIZE** `[26:16]` (RW)
    /// Translation aperture size for this virtual address range, in units of 4 kiB (one flash sector).
    /// Bits 21:12 of the virtual address are compared to SIZE. Offsets greater than SIZE return a bus error, and do not cause a QSPI access.
    #[inline(always)]
    pub fn write_size(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SIZE_MASK) | ((value << Self::SIZE_OFFSET) & Self::SIZE_MASK));
    }

    /// **BASE** `[11:0]` (RW)
    /// Physical address base for this virtual address range, in units of 4 kiB (one flash sector).
    /// Taking a 24-bit virtual address, firstly bits 23:22 (the two MSBs) are masked to zero, and then BASE is added to bits 23:12 (the upper 12 bits) to form the physical address. Translation wraps on a 16 MiB boundary.
    #[inline(always)]
    pub fn base(&self) -> u32 {
        (self.value.read_bits() & Self::BASE_MASK) >> Self::BASE_OFFSET
    }

    /// Write **BASE** `[11:0]` (RW)
    /// Physical address base for this virtual address range, in units of 4 kiB (one flash sector).
    /// Taking a 24-bit virtual address, firstly bits 23:22 (the two MSBs) are masked to zero, and then BASE is added to bits 23:12 (the upper 12 bits) to form the physical address. Translation wraps on a 16 MiB boundary.
    #[inline(always)]
    pub fn write_base(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::BASE_MASK) | ((value << Self::BASE_OFFSET) & Self::BASE_MASK));
    }

}

/// ATRANS3 value type for batch read/modify/write operations
///
/// Use `atrans3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `atrans3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Atrans3Val {
    bits: u32,
}

impl Atrans3Val {
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
        Self { bits: 0x04000c00 }
    }

    /// Read **SIZE** `[26:16]`
    /// Translation aperture size for this virtual address range, in units of 4 kiB (one flash sector).
    /// Bits 21:12 of the virtual address are compared to SIZE. Offsets greater than SIZE return a bus error, and do not cause a QSPI access.
    #[inline(always)]
    pub const fn size(&self) -> u32 {
        (self.bits & Atrans3::SIZE_MASK) >> Atrans3::SIZE_OFFSET
    }

    /// Read **BASE** `[11:0]`
    /// Physical address base for this virtual address range, in units of 4 kiB (one flash sector).
    /// Taking a 24-bit virtual address, firstly bits 23:22 (the two MSBs) are masked to zero, and then BASE is added to bits 23:12 (the upper 12 bits) to form the physical address. Translation wraps on a 16 MiB boundary.
    #[inline(always)]
    pub const fn base(&self) -> u32 {
        (self.bits & Atrans3::BASE_MASK) >> Atrans3::BASE_OFFSET
    }

    /// Set **SIZE** `[26:16]`
    /// Translation aperture size for this virtual address range, in units of 4 kiB (one flash sector).
    /// Bits 21:12 of the virtual address are compared to SIZE. Offsets greater than SIZE return a bus error, and do not cause a QSPI access.
    #[inline(always)]
    pub const fn with_size(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Atrans3::SIZE_MASK) | ((value << Atrans3::SIZE_OFFSET) & Atrans3::SIZE_MASK)
        }
    }

    /// Set **BASE** `[11:0]`
    /// Physical address base for this virtual address range, in units of 4 kiB (one flash sector).
    /// Taking a 24-bit virtual address, firstly bits 23:22 (the two MSBs) are masked to zero, and then BASE is added to bits 23:12 (the upper 12 bits) to form the physical address. Translation wraps on a 16 MiB boundary.
    #[inline(always)]
    pub const fn with_base(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Atrans3::BASE_MASK) | ((value << Atrans3::BASE_OFFSET) & Atrans3::BASE_MASK)
        }
    }

}

impl Default for Atrans3Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Atrans3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Atrans3Val {
        Atrans3Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Atrans3Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Atrans3Val) -> Atrans3Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// ATRANS4 Register
///
/// Configure address translation for XIP virtual addresses 0x1000000 through 0x13fffff (a 4 MiB window starting at +16 MiB).
/// Address translation allows a program image to be executed in place at multiple physical flash addresses (for example, a double-buffered flash image for over-the-air updates), without the overhead of position-independent code.
/// At reset, the address translation registers are initialised to an identity mapping, so that they can be ignored if address translation is not required.
/// Note that the XIP cache is fully virtually addressed, so a cache flush is required after changing the address translation.
///
/// Address offset: 0x0044
/// Reset value: 0x04000000
///
/// ## Fields
///
/// - **SIZE** `[26:16]` (RW)
///   Translation aperture size for this virtual address range, in units of 4 kiB (one flash sector).
///   Bits 21:12 of the virtual address are compared to SIZE. Offsets greater than SIZE return a bus error, and do not cause a QSPI access.
/// - **BASE** `[11:0]` (RW)
///   Physical address base for this virtual address range, in units of 4 kiB (one flash sector).
///   Taking a 24-bit virtual address, firstly bits 23:22 (the two MSBs) are masked to zero, and then BASE is added to bits 23:12 (the upper 12 bits) to form the physical address. Translation wraps on a 16 MiB boundary.
#[repr(transparent)]
pub struct Atrans4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Atrans4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Atrans4 {
    // ======== Field Constants ========

    /// **SIZE**
    /// Translation aperture size for this virtual address range, in units of 4 kiB (one flash sector).
    /// Bits 21:12 of the virtual address are compared to SIZE. Offsets greater than SIZE return a bus error, and do not cause a QSPI access.
    /// Bits `[26:16]`, mask `0x07ff0000`
    pub const SIZE_OFFSET: u32 = 16;
    pub const SIZE_MASK: u32 = 0x07ff0000;

    /// **BASE**
    /// Physical address base for this virtual address range, in units of 4 kiB (one flash sector).
    /// Taking a 24-bit virtual address, firstly bits 23:22 (the two MSBs) are masked to zero, and then BASE is added to bits 23:12 (the upper 12 bits) to form the physical address. Translation wraps on a 16 MiB boundary.
    /// Bits `[11:0]`, mask `0x00000fff`
    pub const BASE_OFFSET: u32 = 0;
    pub const BASE_MASK: u32 = 0x00000fff;

    // ======== Field Accessors ========

    /// **SIZE** `[26:16]` (RW)
    /// Translation aperture size for this virtual address range, in units of 4 kiB (one flash sector).
    /// Bits 21:12 of the virtual address are compared to SIZE. Offsets greater than SIZE return a bus error, and do not cause a QSPI access.
    #[inline(always)]
    pub fn size(&self) -> u32 {
        (self.value.read_bits() & Self::SIZE_MASK) >> Self::SIZE_OFFSET
    }

    /// Write **SIZE** `[26:16]` (RW)
    /// Translation aperture size for this virtual address range, in units of 4 kiB (one flash sector).
    /// Bits 21:12 of the virtual address are compared to SIZE. Offsets greater than SIZE return a bus error, and do not cause a QSPI access.
    #[inline(always)]
    pub fn write_size(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SIZE_MASK) | ((value << Self::SIZE_OFFSET) & Self::SIZE_MASK));
    }

    /// **BASE** `[11:0]` (RW)
    /// Physical address base for this virtual address range, in units of 4 kiB (one flash sector).
    /// Taking a 24-bit virtual address, firstly bits 23:22 (the two MSBs) are masked to zero, and then BASE is added to bits 23:12 (the upper 12 bits) to form the physical address. Translation wraps on a 16 MiB boundary.
    #[inline(always)]
    pub fn base(&self) -> u32 {
        (self.value.read_bits() & Self::BASE_MASK) >> Self::BASE_OFFSET
    }

    /// Write **BASE** `[11:0]` (RW)
    /// Physical address base for this virtual address range, in units of 4 kiB (one flash sector).
    /// Taking a 24-bit virtual address, firstly bits 23:22 (the two MSBs) are masked to zero, and then BASE is added to bits 23:12 (the upper 12 bits) to form the physical address. Translation wraps on a 16 MiB boundary.
    #[inline(always)]
    pub fn write_base(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::BASE_MASK) | ((value << Self::BASE_OFFSET) & Self::BASE_MASK));
    }

}

/// ATRANS4 value type for batch read/modify/write operations
///
/// Use `atrans4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `atrans4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Atrans4Val {
    bits: u32,
}

impl Atrans4Val {
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
        Self { bits: 0x04000000 }
    }

    /// Read **SIZE** `[26:16]`
    /// Translation aperture size for this virtual address range, in units of 4 kiB (one flash sector).
    /// Bits 21:12 of the virtual address are compared to SIZE. Offsets greater than SIZE return a bus error, and do not cause a QSPI access.
    #[inline(always)]
    pub const fn size(&self) -> u32 {
        (self.bits & Atrans4::SIZE_MASK) >> Atrans4::SIZE_OFFSET
    }

    /// Read **BASE** `[11:0]`
    /// Physical address base for this virtual address range, in units of 4 kiB (one flash sector).
    /// Taking a 24-bit virtual address, firstly bits 23:22 (the two MSBs) are masked to zero, and then BASE is added to bits 23:12 (the upper 12 bits) to form the physical address. Translation wraps on a 16 MiB boundary.
    #[inline(always)]
    pub const fn base(&self) -> u32 {
        (self.bits & Atrans4::BASE_MASK) >> Atrans4::BASE_OFFSET
    }

    /// Set **SIZE** `[26:16]`
    /// Translation aperture size for this virtual address range, in units of 4 kiB (one flash sector).
    /// Bits 21:12 of the virtual address are compared to SIZE. Offsets greater than SIZE return a bus error, and do not cause a QSPI access.
    #[inline(always)]
    pub const fn with_size(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Atrans4::SIZE_MASK) | ((value << Atrans4::SIZE_OFFSET) & Atrans4::SIZE_MASK)
        }
    }

    /// Set **BASE** `[11:0]`
    /// Physical address base for this virtual address range, in units of 4 kiB (one flash sector).
    /// Taking a 24-bit virtual address, firstly bits 23:22 (the two MSBs) are masked to zero, and then BASE is added to bits 23:12 (the upper 12 bits) to form the physical address. Translation wraps on a 16 MiB boundary.
    #[inline(always)]
    pub const fn with_base(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Atrans4::BASE_MASK) | ((value << Atrans4::BASE_OFFSET) & Atrans4::BASE_MASK)
        }
    }

}

impl Default for Atrans4Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Atrans4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Atrans4Val {
        Atrans4Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Atrans4Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Atrans4Val) -> Atrans4Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// ATRANS5 Register
///
/// Configure address translation for XIP virtual addresses 0x1400000 through 0x17fffff (a 4 MiB window starting at +20 MiB).
/// Address translation allows a program image to be executed in place at multiple physical flash addresses (for example, a double-buffered flash image for over-the-air updates), without the overhead of position-independent code.
/// At reset, the address translation registers are initialised to an identity mapping, so that they can be ignored if address translation is not required.
/// Note that the XIP cache is fully virtually addressed, so a cache flush is required after changing the address translation.
///
/// Address offset: 0x0048
/// Reset value: 0x04000400
///
/// ## Fields
///
/// - **SIZE** `[26:16]` (RW)
///   Translation aperture size for this virtual address range, in units of 4 kiB (one flash sector).
///   Bits 21:12 of the virtual address are compared to SIZE. Offsets greater than SIZE return a bus error, and do not cause a QSPI access.
/// - **BASE** `[11:0]` (RW)
///   Physical address base for this virtual address range, in units of 4 kiB (one flash sector).
///   Taking a 24-bit virtual address, firstly bits 23:22 (the two MSBs) are masked to zero, and then BASE is added to bits 23:12 (the upper 12 bits) to form the physical address. Translation wraps on a 16 MiB boundary.
#[repr(transparent)]
pub struct Atrans5 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Atrans5 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Atrans5 {
    // ======== Field Constants ========

    /// **SIZE**
    /// Translation aperture size for this virtual address range, in units of 4 kiB (one flash sector).
    /// Bits 21:12 of the virtual address are compared to SIZE. Offsets greater than SIZE return a bus error, and do not cause a QSPI access.
    /// Bits `[26:16]`, mask `0x07ff0000`
    pub const SIZE_OFFSET: u32 = 16;
    pub const SIZE_MASK: u32 = 0x07ff0000;

    /// **BASE**
    /// Physical address base for this virtual address range, in units of 4 kiB (one flash sector).
    /// Taking a 24-bit virtual address, firstly bits 23:22 (the two MSBs) are masked to zero, and then BASE is added to bits 23:12 (the upper 12 bits) to form the physical address. Translation wraps on a 16 MiB boundary.
    /// Bits `[11:0]`, mask `0x00000fff`
    pub const BASE_OFFSET: u32 = 0;
    pub const BASE_MASK: u32 = 0x00000fff;

    // ======== Field Accessors ========

    /// **SIZE** `[26:16]` (RW)
    /// Translation aperture size for this virtual address range, in units of 4 kiB (one flash sector).
    /// Bits 21:12 of the virtual address are compared to SIZE. Offsets greater than SIZE return a bus error, and do not cause a QSPI access.
    #[inline(always)]
    pub fn size(&self) -> u32 {
        (self.value.read_bits() & Self::SIZE_MASK) >> Self::SIZE_OFFSET
    }

    /// Write **SIZE** `[26:16]` (RW)
    /// Translation aperture size for this virtual address range, in units of 4 kiB (one flash sector).
    /// Bits 21:12 of the virtual address are compared to SIZE. Offsets greater than SIZE return a bus error, and do not cause a QSPI access.
    #[inline(always)]
    pub fn write_size(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SIZE_MASK) | ((value << Self::SIZE_OFFSET) & Self::SIZE_MASK));
    }

    /// **BASE** `[11:0]` (RW)
    /// Physical address base for this virtual address range, in units of 4 kiB (one flash sector).
    /// Taking a 24-bit virtual address, firstly bits 23:22 (the two MSBs) are masked to zero, and then BASE is added to bits 23:12 (the upper 12 bits) to form the physical address. Translation wraps on a 16 MiB boundary.
    #[inline(always)]
    pub fn base(&self) -> u32 {
        (self.value.read_bits() & Self::BASE_MASK) >> Self::BASE_OFFSET
    }

    /// Write **BASE** `[11:0]` (RW)
    /// Physical address base for this virtual address range, in units of 4 kiB (one flash sector).
    /// Taking a 24-bit virtual address, firstly bits 23:22 (the two MSBs) are masked to zero, and then BASE is added to bits 23:12 (the upper 12 bits) to form the physical address. Translation wraps on a 16 MiB boundary.
    #[inline(always)]
    pub fn write_base(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::BASE_MASK) | ((value << Self::BASE_OFFSET) & Self::BASE_MASK));
    }

}

/// ATRANS5 value type for batch read/modify/write operations
///
/// Use `atrans5.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `atrans5.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Atrans5Val {
    bits: u32,
}

impl Atrans5Val {
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
        Self { bits: 0x04000400 }
    }

    /// Read **SIZE** `[26:16]`
    /// Translation aperture size for this virtual address range, in units of 4 kiB (one flash sector).
    /// Bits 21:12 of the virtual address are compared to SIZE. Offsets greater than SIZE return a bus error, and do not cause a QSPI access.
    #[inline(always)]
    pub const fn size(&self) -> u32 {
        (self.bits & Atrans5::SIZE_MASK) >> Atrans5::SIZE_OFFSET
    }

    /// Read **BASE** `[11:0]`
    /// Physical address base for this virtual address range, in units of 4 kiB (one flash sector).
    /// Taking a 24-bit virtual address, firstly bits 23:22 (the two MSBs) are masked to zero, and then BASE is added to bits 23:12 (the upper 12 bits) to form the physical address. Translation wraps on a 16 MiB boundary.
    #[inline(always)]
    pub const fn base(&self) -> u32 {
        (self.bits & Atrans5::BASE_MASK) >> Atrans5::BASE_OFFSET
    }

    /// Set **SIZE** `[26:16]`
    /// Translation aperture size for this virtual address range, in units of 4 kiB (one flash sector).
    /// Bits 21:12 of the virtual address are compared to SIZE. Offsets greater than SIZE return a bus error, and do not cause a QSPI access.
    #[inline(always)]
    pub const fn with_size(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Atrans5::SIZE_MASK) | ((value << Atrans5::SIZE_OFFSET) & Atrans5::SIZE_MASK)
        }
    }

    /// Set **BASE** `[11:0]`
    /// Physical address base for this virtual address range, in units of 4 kiB (one flash sector).
    /// Taking a 24-bit virtual address, firstly bits 23:22 (the two MSBs) are masked to zero, and then BASE is added to bits 23:12 (the upper 12 bits) to form the physical address. Translation wraps on a 16 MiB boundary.
    #[inline(always)]
    pub const fn with_base(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Atrans5::BASE_MASK) | ((value << Atrans5::BASE_OFFSET) & Atrans5::BASE_MASK)
        }
    }

}

impl Default for Atrans5Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Atrans5 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Atrans5Val {
        Atrans5Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Atrans5Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Atrans5Val) -> Atrans5Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// ATRANS6 Register
///
/// Configure address translation for XIP virtual addresses 0x1800000 through 0x1bfffff (a 4 MiB window starting at +24 MiB).
/// Address translation allows a program image to be executed in place at multiple physical flash addresses (for example, a double-buffered flash image for over-the-air updates), without the overhead of position-independent code.
/// At reset, the address translation registers are initialised to an identity mapping, so that they can be ignored if address translation is not required.
/// Note that the XIP cache is fully virtually addressed, so a cache flush is required after changing the address translation.
///
/// Address offset: 0x004c
/// Reset value: 0x04000800
///
/// ## Fields
///
/// - **SIZE** `[26:16]` (RW)
///   Translation aperture size for this virtual address range, in units of 4 kiB (one flash sector).
///   Bits 21:12 of the virtual address are compared to SIZE. Offsets greater than SIZE return a bus error, and do not cause a QSPI access.
/// - **BASE** `[11:0]` (RW)
///   Physical address base for this virtual address range, in units of 4 kiB (one flash sector).
///   Taking a 24-bit virtual address, firstly bits 23:22 (the two MSBs) are masked to zero, and then BASE is added to bits 23:12 (the upper 12 bits) to form the physical address. Translation wraps on a 16 MiB boundary.
#[repr(transparent)]
pub struct Atrans6 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Atrans6 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Atrans6 {
    // ======== Field Constants ========

    /// **SIZE**
    /// Translation aperture size for this virtual address range, in units of 4 kiB (one flash sector).
    /// Bits 21:12 of the virtual address are compared to SIZE. Offsets greater than SIZE return a bus error, and do not cause a QSPI access.
    /// Bits `[26:16]`, mask `0x07ff0000`
    pub const SIZE_OFFSET: u32 = 16;
    pub const SIZE_MASK: u32 = 0x07ff0000;

    /// **BASE**
    /// Physical address base for this virtual address range, in units of 4 kiB (one flash sector).
    /// Taking a 24-bit virtual address, firstly bits 23:22 (the two MSBs) are masked to zero, and then BASE is added to bits 23:12 (the upper 12 bits) to form the physical address. Translation wraps on a 16 MiB boundary.
    /// Bits `[11:0]`, mask `0x00000fff`
    pub const BASE_OFFSET: u32 = 0;
    pub const BASE_MASK: u32 = 0x00000fff;

    // ======== Field Accessors ========

    /// **SIZE** `[26:16]` (RW)
    /// Translation aperture size for this virtual address range, in units of 4 kiB (one flash sector).
    /// Bits 21:12 of the virtual address are compared to SIZE. Offsets greater than SIZE return a bus error, and do not cause a QSPI access.
    #[inline(always)]
    pub fn size(&self) -> u32 {
        (self.value.read_bits() & Self::SIZE_MASK) >> Self::SIZE_OFFSET
    }

    /// Write **SIZE** `[26:16]` (RW)
    /// Translation aperture size for this virtual address range, in units of 4 kiB (one flash sector).
    /// Bits 21:12 of the virtual address are compared to SIZE. Offsets greater than SIZE return a bus error, and do not cause a QSPI access.
    #[inline(always)]
    pub fn write_size(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SIZE_MASK) | ((value << Self::SIZE_OFFSET) & Self::SIZE_MASK));
    }

    /// **BASE** `[11:0]` (RW)
    /// Physical address base for this virtual address range, in units of 4 kiB (one flash sector).
    /// Taking a 24-bit virtual address, firstly bits 23:22 (the two MSBs) are masked to zero, and then BASE is added to bits 23:12 (the upper 12 bits) to form the physical address. Translation wraps on a 16 MiB boundary.
    #[inline(always)]
    pub fn base(&self) -> u32 {
        (self.value.read_bits() & Self::BASE_MASK) >> Self::BASE_OFFSET
    }

    /// Write **BASE** `[11:0]` (RW)
    /// Physical address base for this virtual address range, in units of 4 kiB (one flash sector).
    /// Taking a 24-bit virtual address, firstly bits 23:22 (the two MSBs) are masked to zero, and then BASE is added to bits 23:12 (the upper 12 bits) to form the physical address. Translation wraps on a 16 MiB boundary.
    #[inline(always)]
    pub fn write_base(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::BASE_MASK) | ((value << Self::BASE_OFFSET) & Self::BASE_MASK));
    }

}

/// ATRANS6 value type for batch read/modify/write operations
///
/// Use `atrans6.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `atrans6.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Atrans6Val {
    bits: u32,
}

impl Atrans6Val {
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
        Self { bits: 0x04000800 }
    }

    /// Read **SIZE** `[26:16]`
    /// Translation aperture size for this virtual address range, in units of 4 kiB (one flash sector).
    /// Bits 21:12 of the virtual address are compared to SIZE. Offsets greater than SIZE return a bus error, and do not cause a QSPI access.
    #[inline(always)]
    pub const fn size(&self) -> u32 {
        (self.bits & Atrans6::SIZE_MASK) >> Atrans6::SIZE_OFFSET
    }

    /// Read **BASE** `[11:0]`
    /// Physical address base for this virtual address range, in units of 4 kiB (one flash sector).
    /// Taking a 24-bit virtual address, firstly bits 23:22 (the two MSBs) are masked to zero, and then BASE is added to bits 23:12 (the upper 12 bits) to form the physical address. Translation wraps on a 16 MiB boundary.
    #[inline(always)]
    pub const fn base(&self) -> u32 {
        (self.bits & Atrans6::BASE_MASK) >> Atrans6::BASE_OFFSET
    }

    /// Set **SIZE** `[26:16]`
    /// Translation aperture size for this virtual address range, in units of 4 kiB (one flash sector).
    /// Bits 21:12 of the virtual address are compared to SIZE. Offsets greater than SIZE return a bus error, and do not cause a QSPI access.
    #[inline(always)]
    pub const fn with_size(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Atrans6::SIZE_MASK) | ((value << Atrans6::SIZE_OFFSET) & Atrans6::SIZE_MASK)
        }
    }

    /// Set **BASE** `[11:0]`
    /// Physical address base for this virtual address range, in units of 4 kiB (one flash sector).
    /// Taking a 24-bit virtual address, firstly bits 23:22 (the two MSBs) are masked to zero, and then BASE is added to bits 23:12 (the upper 12 bits) to form the physical address. Translation wraps on a 16 MiB boundary.
    #[inline(always)]
    pub const fn with_base(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Atrans6::BASE_MASK) | ((value << Atrans6::BASE_OFFSET) & Atrans6::BASE_MASK)
        }
    }

}

impl Default for Atrans6Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Atrans6 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Atrans6Val {
        Atrans6Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Atrans6Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Atrans6Val) -> Atrans6Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// ATRANS7 Register
///
/// Configure address translation for XIP virtual addresses 0x1c00000 through 0x1ffffff (a 4 MiB window starting at +28 MiB).
/// Address translation allows a program image to be executed in place at multiple physical flash addresses (for example, a double-buffered flash image for over-the-air updates), without the overhead of position-independent code.
/// At reset, the address translation registers are initialised to an identity mapping, so that they can be ignored if address translation is not required.
/// Note that the XIP cache is fully virtually addressed, so a cache flush is required after changing the address translation.
///
/// Address offset: 0x0050
/// Reset value: 0x04000c00
///
/// ## Fields
///
/// - **SIZE** `[26:16]` (RW)
///   Translation aperture size for this virtual address range, in units of 4 kiB (one flash sector).
///   Bits 21:12 of the virtual address are compared to SIZE. Offsets greater than SIZE return a bus error, and do not cause a QSPI access.
/// - **BASE** `[11:0]` (RW)
///   Physical address base for this virtual address range, in units of 4 kiB (one flash sector).
///   Taking a 24-bit virtual address, firstly bits 23:22 (the two MSBs) are masked to zero, and then BASE is added to bits 23:12 (the upper 12 bits) to form the physical address. Translation wraps on a 16 MiB boundary.
#[repr(transparent)]
pub struct Atrans7 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Atrans7 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Atrans7 {
    // ======== Field Constants ========

    /// **SIZE**
    /// Translation aperture size for this virtual address range, in units of 4 kiB (one flash sector).
    /// Bits 21:12 of the virtual address are compared to SIZE. Offsets greater than SIZE return a bus error, and do not cause a QSPI access.
    /// Bits `[26:16]`, mask `0x07ff0000`
    pub const SIZE_OFFSET: u32 = 16;
    pub const SIZE_MASK: u32 = 0x07ff0000;

    /// **BASE**
    /// Physical address base for this virtual address range, in units of 4 kiB (one flash sector).
    /// Taking a 24-bit virtual address, firstly bits 23:22 (the two MSBs) are masked to zero, and then BASE is added to bits 23:12 (the upper 12 bits) to form the physical address. Translation wraps on a 16 MiB boundary.
    /// Bits `[11:0]`, mask `0x00000fff`
    pub const BASE_OFFSET: u32 = 0;
    pub const BASE_MASK: u32 = 0x00000fff;

    // ======== Field Accessors ========

    /// **SIZE** `[26:16]` (RW)
    /// Translation aperture size for this virtual address range, in units of 4 kiB (one flash sector).
    /// Bits 21:12 of the virtual address are compared to SIZE. Offsets greater than SIZE return a bus error, and do not cause a QSPI access.
    #[inline(always)]
    pub fn size(&self) -> u32 {
        (self.value.read_bits() & Self::SIZE_MASK) >> Self::SIZE_OFFSET
    }

    /// Write **SIZE** `[26:16]` (RW)
    /// Translation aperture size for this virtual address range, in units of 4 kiB (one flash sector).
    /// Bits 21:12 of the virtual address are compared to SIZE. Offsets greater than SIZE return a bus error, and do not cause a QSPI access.
    #[inline(always)]
    pub fn write_size(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SIZE_MASK) | ((value << Self::SIZE_OFFSET) & Self::SIZE_MASK));
    }

    /// **BASE** `[11:0]` (RW)
    /// Physical address base for this virtual address range, in units of 4 kiB (one flash sector).
    /// Taking a 24-bit virtual address, firstly bits 23:22 (the two MSBs) are masked to zero, and then BASE is added to bits 23:12 (the upper 12 bits) to form the physical address. Translation wraps on a 16 MiB boundary.
    #[inline(always)]
    pub fn base(&self) -> u32 {
        (self.value.read_bits() & Self::BASE_MASK) >> Self::BASE_OFFSET
    }

    /// Write **BASE** `[11:0]` (RW)
    /// Physical address base for this virtual address range, in units of 4 kiB (one flash sector).
    /// Taking a 24-bit virtual address, firstly bits 23:22 (the two MSBs) are masked to zero, and then BASE is added to bits 23:12 (the upper 12 bits) to form the physical address. Translation wraps on a 16 MiB boundary.
    #[inline(always)]
    pub fn write_base(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::BASE_MASK) | ((value << Self::BASE_OFFSET) & Self::BASE_MASK));
    }

}

/// ATRANS7 value type for batch read/modify/write operations
///
/// Use `atrans7.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `atrans7.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Atrans7Val {
    bits: u32,
}

impl Atrans7Val {
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
        Self { bits: 0x04000c00 }
    }

    /// Read **SIZE** `[26:16]`
    /// Translation aperture size for this virtual address range, in units of 4 kiB (one flash sector).
    /// Bits 21:12 of the virtual address are compared to SIZE. Offsets greater than SIZE return a bus error, and do not cause a QSPI access.
    #[inline(always)]
    pub const fn size(&self) -> u32 {
        (self.bits & Atrans7::SIZE_MASK) >> Atrans7::SIZE_OFFSET
    }

    /// Read **BASE** `[11:0]`
    /// Physical address base for this virtual address range, in units of 4 kiB (one flash sector).
    /// Taking a 24-bit virtual address, firstly bits 23:22 (the two MSBs) are masked to zero, and then BASE is added to bits 23:12 (the upper 12 bits) to form the physical address. Translation wraps on a 16 MiB boundary.
    #[inline(always)]
    pub const fn base(&self) -> u32 {
        (self.bits & Atrans7::BASE_MASK) >> Atrans7::BASE_OFFSET
    }

    /// Set **SIZE** `[26:16]`
    /// Translation aperture size for this virtual address range, in units of 4 kiB (one flash sector).
    /// Bits 21:12 of the virtual address are compared to SIZE. Offsets greater than SIZE return a bus error, and do not cause a QSPI access.
    #[inline(always)]
    pub const fn with_size(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Atrans7::SIZE_MASK) | ((value << Atrans7::SIZE_OFFSET) & Atrans7::SIZE_MASK)
        }
    }

    /// Set **BASE** `[11:0]`
    /// Physical address base for this virtual address range, in units of 4 kiB (one flash sector).
    /// Taking a 24-bit virtual address, firstly bits 23:22 (the two MSBs) are masked to zero, and then BASE is added to bits 23:12 (the upper 12 bits) to form the physical address. Translation wraps on a 16 MiB boundary.
    #[inline(always)]
    pub const fn with_base(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Atrans7::BASE_MASK) | ((value << Atrans7::BASE_OFFSET) & Atrans7::BASE_MASK)
        }
    }

}

impl Default for Atrans7Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Atrans7 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Atrans7Val {
        Atrans7Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Atrans7Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Atrans7Val) -> Atrans7Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// Control and status for direct serial mode
    /// Direct serial mode allows the processor to send and receive raw serial frames, for programming, configuration and control of the external memory devices. Only SPI mode 0 (CPOL=0 CPHA=0) is supported.
    pub direct_csr: DirectCsr,
    /// 0x0004
    /// Transmit FIFO for direct mode
    pub direct_tx: DirectTx,
    /// 0x0008
    /// Receive FIFO for direct mode
    pub direct_rx: DirectRx,
    /// 0x000c
    /// Timing configuration register for memory address window 0.
    pub m0_timing: M0Timing,
    /// 0x0010
    /// Read transfer format configuration for memory address window 0.
    /// Configure the bus width of each transfer phase individually, and configure the length or presence of the command prefix, command suffix and dummy/turnaround transfer phases. Only 24-bit addresses are supported.
    /// The reset value of the M0_RFMT register is configured to support a basic 03h serial read transfer with no additional configuration.
    pub m0_rfmt: M0Rfmt,
    /// 0x0014
    /// Command constants used for reads from memory address window 0.
    /// The reset value of the M0_RCMD register is configured to support a basic 03h serial read transfer with no additional configuration.
    pub m0_rcmd: M0Rcmd,
    /// 0x0018
    /// Write transfer format configuration for memory address window 0.
    /// Configure the bus width of each transfer phase individually, and configure the length or presence of the command prefix, command suffix and dummy/turnaround transfer phases. Only 24-bit addresses are supported.
    /// The reset value of the M0_WFMT register is configured to support a basic 02h serial write transfer. However, writes to this window must first be enabled via the XIP_CTRL_WRITABLE_M0 bit, as XIP memory is read-only by default.
    pub m0_wfmt: M0Wfmt,
    /// 0x001c
    /// Command constants used for writes to memory address window 0.
    /// The reset value of the M0_WCMD register is configured to support a basic 02h serial write transfer with no additional configuration.
    pub m0_wcmd: M0Wcmd,
    /// 0x0020
    /// Timing configuration register for memory address window 1.
    pub m1_timing: M1Timing,
    /// 0x0024
    /// Read transfer format configuration for memory address window 1.
    /// Configure the bus width of each transfer phase individually, and configure the length or presence of the command prefix, command suffix and dummy/turnaround transfer phases. Only 24-bit addresses are supported.
    /// The reset value of the M1_RFMT register is configured to support a basic 03h serial read transfer with no additional configuration.
    pub m1_rfmt: M1Rfmt,
    /// 0x0028
    /// Command constants used for reads from memory address window 1.
    /// The reset value of the M1_RCMD register is configured to support a basic 03h serial read transfer with no additional configuration.
    pub m1_rcmd: M1Rcmd,
    /// 0x002c
    /// Write transfer format configuration for memory address window 1.
    /// Configure the bus width of each transfer phase individually, and configure the length or presence of the command prefix, command suffix and dummy/turnaround transfer phases. Only 24-bit addresses are supported.
    /// The reset value of the M1_WFMT register is configured to support a basic 02h serial write transfer. However, writes to this window must first be enabled via the XIP_CTRL_WRITABLE_M1 bit, as XIP memory is read-only by default.
    pub m1_wfmt: M1Wfmt,
    /// 0x0030
    /// Command constants used for writes to memory address window 1.
    /// The reset value of the M1_WCMD register is configured to support a basic 02h serial write transfer with no additional configuration.
    pub m1_wcmd: M1Wcmd,
    /// 0x0034
    /// Configure address translation for XIP virtual addresses 0x000000 through 0x3fffff (a 4 MiB window starting at +0 MiB).
    /// Address translation allows a program image to be executed in place at multiple physical flash addresses (for example, a double-buffered flash image for over-the-air updates), without the overhead of position-independent code.
    /// At reset, the address translation registers are initialised to an identity mapping, so that they can be ignored if address translation is not required.
    /// Note that the XIP cache is fully virtually addressed, so a cache flush is required after changing the address translation.
    pub atrans0: Atrans0,
    /// 0x0038
    /// Configure address translation for XIP virtual addresses 0x400000 through 0x7fffff (a 4 MiB window starting at +4 MiB).
    /// Address translation allows a program image to be executed in place at multiple physical flash addresses (for example, a double-buffered flash image for over-the-air updates), without the overhead of position-independent code.
    /// At reset, the address translation registers are initialised to an identity mapping, so that they can be ignored if address translation is not required.
    /// Note that the XIP cache is fully virtually addressed, so a cache flush is required after changing the address translation.
    pub atrans1: Atrans1,
    /// 0x003c
    /// Configure address translation for XIP virtual addresses 0x800000 through 0xbfffff (a 4 MiB window starting at +8 MiB).
    /// Address translation allows a program image to be executed in place at multiple physical flash addresses (for example, a double-buffered flash image for over-the-air updates), without the overhead of position-independent code.
    /// At reset, the address translation registers are initialised to an identity mapping, so that they can be ignored if address translation is not required.
    /// Note that the XIP cache is fully virtually addressed, so a cache flush is required after changing the address translation.
    pub atrans2: Atrans2,
    /// 0x0040
    /// Configure address translation for XIP virtual addresses 0xc00000 through 0xffffff (a 4 MiB window starting at +12 MiB).
    /// Address translation allows a program image to be executed in place at multiple physical flash addresses (for example, a double-buffered flash image for over-the-air updates), without the overhead of position-independent code.
    /// At reset, the address translation registers are initialised to an identity mapping, so that they can be ignored if address translation is not required.
    /// Note that the XIP cache is fully virtually addressed, so a cache flush is required after changing the address translation.
    pub atrans3: Atrans3,
    /// 0x0044
    /// Configure address translation for XIP virtual addresses 0x1000000 through 0x13fffff (a 4 MiB window starting at +16 MiB).
    /// Address translation allows a program image to be executed in place at multiple physical flash addresses (for example, a double-buffered flash image for over-the-air updates), without the overhead of position-independent code.
    /// At reset, the address translation registers are initialised to an identity mapping, so that they can be ignored if address translation is not required.
    /// Note that the XIP cache is fully virtually addressed, so a cache flush is required after changing the address translation.
    pub atrans4: Atrans4,
    /// 0x0048
    /// Configure address translation for XIP virtual addresses 0x1400000 through 0x17fffff (a 4 MiB window starting at +20 MiB).
    /// Address translation allows a program image to be executed in place at multiple physical flash addresses (for example, a double-buffered flash image for over-the-air updates), without the overhead of position-independent code.
    /// At reset, the address translation registers are initialised to an identity mapping, so that they can be ignored if address translation is not required.
    /// Note that the XIP cache is fully virtually addressed, so a cache flush is required after changing the address translation.
    pub atrans5: Atrans5,
    /// 0x004c
    /// Configure address translation for XIP virtual addresses 0x1800000 through 0x1bfffff (a 4 MiB window starting at +24 MiB).
    /// Address translation allows a program image to be executed in place at multiple physical flash addresses (for example, a double-buffered flash image for over-the-air updates), without the overhead of position-independent code.
    /// At reset, the address translation registers are initialised to an identity mapping, so that they can be ignored if address translation is not required.
    /// Note that the XIP cache is fully virtually addressed, so a cache flush is required after changing the address translation.
    pub atrans6: Atrans6,
    /// 0x0050
    /// Configure address translation for XIP virtual addresses 0x1c00000 through 0x1ffffff (a 4 MiB window starting at +28 MiB).
    /// Address translation allows a program image to be executed in place at multiple physical flash addresses (for example, a double-buffered flash image for over-the-air updates), without the overhead of position-independent code.
    /// At reset, the address translation registers are initialised to an identity mapping, so that they can be ignored if address translation is not required.
    /// Note that the XIP cache is fully virtually addressed, so a cache flush is required after changing the address translation.
    pub atrans7: Atrans7,
}

