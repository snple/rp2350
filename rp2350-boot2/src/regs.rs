//! Register addresses, field positions and flash commands shared by the blob variants.
//!
//! ## Why these are literals and not `rp2350-pac` items
//!
//! `rp2350-rt` depends on this crate, and `rp2350-pac` depends on `rp2350-rt`, so
//! taking a dependency on the PAC here would make the dependency graph cyclic.
//! Every constant therefore carries the equivalent `rp2350-pac` path (or SVD field
//! name) in its doc comment, so the two can be compared by eye when the SVD is
//! regenerated. Addresses come from the RP2350 address map, which the PAC also
//! uses verbatim in its `peripheral!` declarations.

#![allow(dead_code)]

// ============================================================================
// Peripheral base addresses
// ============================================================================

/// Base address of the QMI (QSPI memory interface) block.
///
/// `rp2350-pac`: `rp2350_pac::QMI` — `peripheral!(QMI, qmi::RegisterBlock, 0x400d0000)`.
/// SVD: `QMI` peripheral.
pub(crate) const QMI_BASE: u32 = 0x400d_0000;

/// Base address of the QSPI pad control block.
///
/// `rp2350-pac`: `rp2350_pac::PADS_QSPI` — `peripheral!(PADS_QSPI, pads_qspi::RegisterBlock, 0x40040000)`.
pub(crate) const PADS_QSPI_BASE: u32 = 0x4004_0000;

/// Offset of the atomic "clear bits" register alias (`REG_ALIAS_CLR_BITS`).
///
/// A store to `base + REG_ALIAS_CLR` clears the bits that are set in the written
/// value, without a read-modify-write on the peripheral itself.
pub(crate) const REG_ALIAS_CLR: u32 = 0x3000;

/// Alias of the XIP region whose reads do not allocate in the XIP cache.
///
/// `addressmap.h`: `XIP_NOCACHE_NOALLOC_BASE`.
pub(crate) const XIP_NOCACHE_NOALLOC_BASE: u32 = 0x1400_0000;

// ============================================================================
// QMI register offsets
// ============================================================================
//
// `rp2350-pac`: field order of `rp2350_pac::qmi::RegisterBlock`; the offsets are
// documented there in the per-field `/// 0x00xx` comments.

/// Direct mode: control and status.
pub(crate) const QMI_DIRECT_CSR: u32 = 0x00;
/// Direct mode: transmit FIFO.
pub(crate) const QMI_DIRECT_TX: u32 = 0x04;
/// Direct mode: receive FIFO.
pub(crate) const QMI_DIRECT_RX: u32 = 0x08;
/// Timing configuration for memory window 0.
pub(crate) const QMI_M0_TIMING: u32 = 0x0c;
/// Read transfer format for memory window 0.
pub(crate) const QMI_M0_RFMT: u32 = 0x10;
/// Command constants used for reads from memory window 0.
pub(crate) const QMI_M0_RCMD: u32 = 0x14;

// ============================================================================
// QMI_DIRECT_CSR fields
// ============================================================================
//
// `rp2350-pac`: `rp2350_pac::qmi::DirectCsr`.

/// RX sample delay in direct mode, in half clock cycles.
pub(crate) const DIRECT_CSR_RXDELAY_LSB: u32 = 30;
/// Direct mode clock divisor: `clk_sys / CLKDIV`.
pub(crate) const DIRECT_CSR_CLKDIV_LSB: u32 = 22;
pub(crate) const DIRECT_CSR_CLKDIV_MASK: u32 = 0x3fc0_0000;
/// Drive CS0n automatically while the TX FIFO is non-empty.
pub(crate) const DIRECT_CSR_AUTO_CS0N: u32 = 1 << 6;
/// Direct mode transfer in progress / cooldown pending.
pub(crate) const DIRECT_CSR_BUSY_MASK: u32 = 1 << 1;
pub(crate) const DIRECT_CSR_BUSY_LSB: u32 = 1;
/// Enable direct serial mode.
pub(crate) const DIRECT_CSR_EN: u32 = 1 << 0;

// ============================================================================
// QMI_M0_TIMING fields
// ============================================================================
//
// `rp2350-pac`: `rp2350_pac::qmi::M0Timing`.

/// Cooldown period after the last transfer, in `clk_sys` cycles: encoded as
/// `(cycles - 1) / 64`, so `1` means the minimum.
pub(crate) const M0_TIMING_COOLDOWN_LSB: u32 = 30;
/// RX sample delay applied to reads through window 0, in half clock cycles.
pub(crate) const M0_TIMING_RXDELAY_LSB: u32 = 8;
pub(crate) const M0_TIMING_RXDELAY_MASK: u32 = 0x700;
/// Clock divisor for window 0: the interface runs at `clk_sys / CLKDIV`.
pub(crate) const M0_TIMING_CLKDIV_LSB: u32 = 0;
pub(crate) const M0_TIMING_CLKDIV_MASK: u32 = 0xff;

// ============================================================================
// QMI_M0_RFMT fields
// ============================================================================
//
// `rp2350-pac`: `rp2350_pac::qmi::M0Rfmt`.
//
// Width fields take the enum values `S = 0`, `D = 1`, `Q = 2`.

/// Width of the command prefix phase.
pub(crate) const M0_RFMT_PREFIX_WIDTH_LSB: u32 = 0;
/// Width of the address phase (always 24 bits total).
pub(crate) const M0_RFMT_ADDR_WIDTH_LSB: u32 = 2;
/// Width of the post-address command suffix phase.
pub(crate) const M0_RFMT_SUFFIX_WIDTH_LSB: u32 = 4;
/// Width of the dummy phase.
pub(crate) const M0_RFMT_DUMMY_WIDTH_LSB: u32 = 6;
/// Width of the data phase.
pub(crate) const M0_RFMT_DATA_WIDTH_LSB: u32 = 8;
/// Length of the command prefix phase: bit set means an 8-bit prefix.
pub(crate) const M0_RFMT_PREFIX_LEN_LSB: u32 = 12;
/// Mask of the (single bit) prefix length field, used to clear it.
pub(crate) const M0_RFMT_PREFIX_LEN_MASK: u32 = 1 << 12;
/// Length of the command suffix phase, in 4-bit units.
pub(crate) const M0_RFMT_SUFFIX_LEN_LSB: u32 = 14;
/// Length of the dummy phase, in 4-bit units.
pub(crate) const M0_RFMT_DUMMY_LEN_LSB: u32 = 16;

/// Transfer width: single.
pub(crate) const WIDTH_S: u32 = 0;
/// Transfer width: quad.
pub(crate) const WIDTH_Q: u32 = 2;

/// `M0_RFMT_PREFIX_LEN` value for an 8-bit (one byte) prefix.
pub(crate) const PREFIX_LEN_8: u32 = 1;
/// `M0_RFMT_SUFFIX_LEN` value for an 8-bit (two 4-bit units) suffix.
pub(crate) const SUFFIX_LEN_8: u32 = 2;
/// `M0_RFMT_DUMMY_LEN` value for 16 dummy bits, i.e. 4 quad-width cycles.
pub(crate) const DUMMY_LEN_16: u32 = 4;

// ============================================================================
// QMI_M0_RCMD fields
// ============================================================================
//
// `rp2350-pac`: `rp2350_pac::qmi::M0Rcmd`.

/// Command byte sent before the address.
pub(crate) const M0_RCMD_PREFIX_LSB: u32 = 0;
/// Mode/suffix byte sent after the address.
pub(crate) const M0_RCMD_SUFFIX_LSB: u32 = 8;

// ============================================================================
// PADS_QSPI registers
// ============================================================================
//
// `rp2350-pac`: `rp2350_pac::pads_qspi::RegisterBlock`.

pub(crate) const PADS_QSPI_GPIO_QSPI_SCLK: u32 = 0x04;
pub(crate) const PADS_QSPI_GPIO_QSPI_SD0: u32 = 0x08;
pub(crate) const PADS_QSPI_GPIO_QSPI_SD1: u32 = 0x0c;
pub(crate) const PADS_QSPI_GPIO_QSPI_SD2: u32 = 0x10;
pub(crate) const PADS_QSPI_GPIO_QSPI_SD3: u32 = 0x14;

/// Pad drive strength field, in 2 mA steps.
///
/// `rp2350-pac`: `rp2350_pac::pads_qspi::GpioQspiSclk::DRIVE_LSB`.
pub(crate) const PAD_DRIVE_LSB: u32 = 4;
/// Drive strength value for 8 mA (`DRIVE = 2`).
pub(crate) const PAD_DRIVE_8MA: u32 = 2;
/// Fast slew rate. `rp2350-pac`: `GpioQspiSclk::SLEWFAST`.
pub(crate) const PAD_SLEWFAST: u32 = 1 << 0;
/// Schmitt trigger enable. `rp2350-pac`: `GpioQspiSd0::SCHMITT`.
pub(crate) const PAD_SCHMITT: u32 = 1 << 1;

// ============================================================================
// Flash commands and register values
// ============================================================================

/// `EBh` — read data fast quad I/O.
pub(crate) const CMD_READ_FAST_QUAD_IO: u32 = 0xeb;
/// Mode bits for continuous read: `M[7:4] = Ah`.
pub(crate) const MODE_CONTINUOUS_READ: u32 = 0xa0;
/// Dummy clocks required by fast quad I/O reads.
pub(crate) const WAIT_CYCLES: u32 = 4;

/// `03h` — read data (single bit).
pub(crate) const CMD_READ: u32 = 0x03;
/// `06h` — write enable.
pub(crate) const CMD_WRITE_ENABLE: u32 = 0x06;
/// `05h` — read status register 1.
pub(crate) const CMD_READ_STATUS: u32 = 0x05;
/// `35h` — read status register 2.
pub(crate) const CMD_READ_STATUS2: u32 = 0x35;
/// `01h` — write status register 1.
pub(crate) const CMD_WRITE_STATUS: u32 = 0x01;
/// Status register value with the quad-enable (QE) bit set.
pub(crate) const SREG_DATA: u32 = 0x02;
