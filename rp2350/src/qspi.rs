//! QSPI (Quad SPI) HAL for RP2350
//!
//! Driver for the QMI (QSPI Memory Interface) peripheral, which provides both
//! a memory-mapped interface (XIP) and a direct serial mode for external
//! SPI/DSPI/QSPI flash or PSRAM devices.
//!
//! # Architecture
//!
//! The QMI has two operating modes:
//!
//! - **Memory-mapped (XIP)**: automatically translates AHB bus accesses into
//!   SPI flash read/write commands. Two chip selects (CS0n/CS1n) with
//!   independent timing, format and command configuration.
//! - **Direct**: software-driven raw SPI frames via TX/RX FIFOs, for flash
//!   programming, erasing, status register access, and other non-memory-mapped
//!   operations.
//!
//! XIP configuration lives on [`Qspi`]; direct-mode operations live on the
//! [`Direct`] handle returned by [`Qspi::direct`].
//!
//! # Features
//!
//! - Single / Dual / Quad bus widths
//! - Two independent chip selects (CS0n, CS1n)
//! - Configurable clock divisor per memory window
//! - Configurable read/write transfer formats with independent phase widths
//! - Command prefix, address, suffix, dummy and data phases
//! - Direct mode for raw SPI command/response
//! - Double Transfer Rate (DTR) support
//! - DMA support for direct mode
//! - Address translation for XIP
//!
//! # QSPI Pin Mapping
//!
//! The QSPI interface uses dedicated pins (not from GPIO bank 0):
//!
//! - QSPI_SCLK — serial clock
//! - QSPI_CSn0 — chip select 0 (active low)
//! - QSPI_CSn1 — chip select 1 (active low), if a second device is connected
//! - QSPI_SD0..SD3 — serial data (SD0 = MOSI, SD1 = MISO in single mode)
//!
//! # Example — direct mode: read flash JEDEC ID
//!
//! ```ignore
//! use rp2350::qspi::{Qspi, Window};
//! use rp2350::Peripherals;
//!
//! let dp = Peripherals::take().unwrap();
//! let qspi = Qspi::init(dp.QMI);
//!
//! let direct = qspi.direct();
//! direct.enable();
//! let (mfr, typ, cap) = direct.read_jedec_id(Window::M0);
//! direct.disable();
//! ```
//!
//! # Example — configure XIP quad read on CS0
//!
//! ```ignore
//! use rp2350::qspi::{BusWidth, Format, Qspi, Window};
//! use rp2350::Peripherals;
//!
//! let dp = Peripherals::take().unwrap();
//! let qspi = Qspi::init(dp.QMI);
//!
//! qspi.set_clkdiv(Window::M0, 2);
//! qspi.configure_read_format(Window::M0, Format {
//!     data_width: BusWidth::Quad,
//!     dummy_len: 2, // 8 dummy bits
//!     ..Format::default()
//! });
//! qspi.set_read_command(Window::M0, 0x6B, 0x00);
//!
//! // XIP reads from 0x1000_0000 now use quad output.
//! ```

use crate::Steal;
use rp2350_pac::QMI;

pub use rp2350_pac::qmi::DirectTxVal;

/// Run `$body` with `reg` bound to the window's register (`m0_*` or `m1_*`).
///
/// The PAC generates a distinct register type per window (`M0Timing` vs
/// `M1Timing`, ...) with identical method surfaces, so this macro emits the
/// body twice — once per window — instead of duplicating the match arms.
macro_rules! with_window {
    ($qmi:expr, $window:expr, $m0:ident, $m1:ident, $reg:ident, $body:block) => {
        match $window {
            Window::M0 => {
                let $reg = &$qmi.$m0;
                $body
            }
            Window::M1 => {
                let $reg = &$qmi.$m1;
                $body
            }
        }
    };
}

/// Write a [`Format`] into a read/write format register.
///
/// `Mx_RFMT` and `Mx_WFMT` share the same field layout and accessor names.
macro_rules! write_format_reg {
    ($reg:expr, $fmt:expr) => {{
        let f = $fmt;
        $reg.write_prefix_width(f.prefix_width as u32);
        $reg.write_addr_width(f.addr_width as u32);
        $reg.write_suffix_width(f.suffix_width as u32);
        $reg.write_dummy_width(f.dummy_width as u32);
        $reg.write_data_width(f.data_width as u32);
        if f.prefix_len {
            $reg.set_prefix_len();
        } else {
            $reg.clr_prefix_len();
        }
        $reg.write_suffix_len(f.suffix_len);
        $reg.write_dummy_len(f.dummy_len);
        if f.dtr {
            $reg.set_dtr();
        } else {
            $reg.clr_dtr();
        }
    }};
}

/// Write a translation window's `size` then `base`.
///
/// `base` is written last because the translation is latched when it changes.
macro_rules! write_atrans {
    ($reg:expr, $base:expr, $size:expr) => {{
        $reg.write_size($size);
        $reg.write_base($base);
    }};
}

// ============================================================================
// Types
// ============================================================================

/// QSPI bus width for transfer phases
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u32)]
pub enum BusWidth {
    /// Single SPI (1-bit): SD0 for output, SD1 for input
    Single = 0,
    /// Dual SPI (2-bit): SD0 and SD1
    Dual = 1,
    /// Quad SPI (4-bit): SD0, SD1, SD2, SD3
    Quad = 2,
}

/// Memory address window (chip select)
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Window {
    /// Memory window 0 (CS0n) — addresses 0x1000_0000..0x10FF_FFFF
    M0,
    /// Memory window 1 (CS1n) — addresses 0x1100_0000..0x11FF_FFFF
    M1,
}

/// Page break configuration for memory-mapped mode
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u32)]
pub enum PageBreak {
    /// No page boundary enforced
    None = 0,
    /// Break at 256-byte page boundaries
    Bytes256 = 1,
    /// Break at 1024-byte quad-page boundaries
    Bytes1024 = 2,
    /// Break at 4096-byte sector boundaries
    Bytes4096 = 3,
}

/// Transfer format for a memory window (read or write).
///
/// Configures the bus width of each phase and the length/presence of the
/// command prefix, suffix and dummy phases. The same shape is used for both
/// read ([`Qspi::configure_read_format`]) and write
/// ([`Qspi::configure_write_format`]) transfers.
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Format {
    /// Width of the command prefix phase
    pub prefix_width: BusWidth,
    /// Width of the address phase (always 24 bits)
    pub addr_width: BusWidth,
    /// Width of the post-address command suffix phase
    pub suffix_width: BusWidth,
    /// Width of the dummy/turnaround phase
    pub dummy_width: BusWidth,
    /// Width of the data phase
    pub data_width: BusWidth,
    /// Emit an 8-bit command prefix (true) or none (false)
    pub prefix_len: bool,
    /// Suffix length: 0 = none, 2 = 8 bits
    pub suffix_len: u32,
    /// Dummy phase length in units of 4 bits (0-7)
    pub dummy_len: u32,
    /// Enable double transfer rate
    pub dtr: bool,
}

impl Default for Format {
    /// Hardware reset value — a basic `0x03` single-width serial read.
    fn default() -> Self {
        Self {
            prefix_width: BusWidth::Single,
            addr_width: BusWidth::Single,
            suffix_width: BusWidth::Single,
            dummy_width: BusWidth::Single,
            data_width: BusWidth::Single,
            prefix_len: true,
            suffix_len: 0,
            dummy_len: 0,
            dtr: false,
        }
    }
}

/// Timing configuration for a memory window
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct TimingConfig {
    /// Clock divisor (1-255, 0 encodes 256)
    pub clkdiv: u8,
    /// RX sample delay in half system clock cycles (0-7)
    pub rxdelay: u8,
    /// Minimum deselect time in system clock cycles (0-31)
    pub min_deselect: u8,
    /// Maximum chip select assertion in 64-cycle units (0=unlimited, 1-63)
    pub max_select: u8,
    /// CS hold time: additional cycles after last SCK falling edge (0-3)
    pub select_hold: u8,
    /// CS setup: add one extra cycle before first SCK rising edge
    pub select_setup: bool,
    /// Page break mode
    pub pagebreak: PageBreak,
    /// Cooldown period (0-3, in 64-cycle units)
    pub cooldown: u8,
}

impl Default for TimingConfig {
    /// Hardware reset value (`0x4000_0004`): clkdiv=4, cooldown=1.
    fn default() -> Self {
        Self {
            clkdiv: 4,
            rxdelay: 0,
            min_deselect: 0,
            max_select: 0,
            select_hold: 0,
            select_setup: false,
            pagebreak: PageBreak::None,
            cooldown: 1,
        }
    }
}

/// Direct-mode transfer error.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// The TX FIFO stayed full — no space to push a frame within the timeout.
    TxFull,
    /// The RX FIFO stayed empty — no data to pop within the timeout.
    RxEmpty,
    /// The QMI stayed busy within the timeout.
    Busy,
}

// ============================================================================
// QSPI peripheral wrapper
// ============================================================================

/// QMI (QSPI Memory Interface) peripheral wrapper.
///
/// Owns the QMI peripheral and provides the memory-mapped (XIP) configuration
/// interface. For direct serial mode, obtain a [`Direct`] handle with
/// [`Qspi::direct`].
///
/// The QSPI controller uses dedicated QSPI pins (SCLK, CSn, SD0-SD3) separate
/// from GPIO bank 0, so no pin ownership management is needed.
pub struct Qspi {
    qmi: QMI,
}

impl Qspi {
    /// Initialize the QSPI driver, taking ownership of the QMI peripheral.
    ///
    /// ```ignore
    /// let dp = Peripherals::take().unwrap();
    /// let qspi = Qspi::init(dp.QMI);
    /// ```
    pub fn init(qmi: QMI) -> Self {
        Self { qmi }
    }

    /// Obtain a Qspi handle without ownership checks.
    ///
    /// # Safety
    ///
    /// The caller must ensure the peripheral has been initialised and that no
    /// data races occur.
    #[inline]
    pub unsafe fn steal() -> Steal<Self> {
        Steal(Self::init(unsafe { QMI::steal() }))
    }

    /// Release the QMI peripheral.
    #[inline]
    pub fn release(self) -> QMI {
        self.qmi
    }

    /// Get a reference to the underlying register block.
    ///
    /// # Safety
    ///
    /// Direct register access can bypass the HAL's safety guarantees.
    #[inline]
    pub unsafe fn regs(&self) -> &rp2350_pac::qmi::RegisterBlock {
        &self.qmi
    }

    /// Borrow a direct-mode handle.
    #[inline]
    pub fn direct(&self) -> Direct<'_> {
        Direct { qmi: &self.qmi }
    }

    // ========================================================================
    // Memory-mapped (XIP) — timing
    // ========================================================================

    /// Set the clock divisor for a memory window.
    ///
    /// Divisors 1..255 are encoded directly; 0 encodes 256. This is the only
    /// timing parameter that may be changed while the QMI is actively
    /// accessing memory.
    #[inline]
    pub fn set_clkdiv(&self, window: Window, div: u8) {
        with_window!(self.qmi, window, m0_timing, m1_timing, reg, {
            reg.write_clkdiv(div as u32);
        });
    }

    /// Get the clock divisor for a memory window.
    #[inline]
    pub fn clkdiv(&self, window: Window) -> u8 {
        with_window!(self.qmi, window, m0_timing, m1_timing, reg, {
            reg.clkdiv() as u8
        })
    }

    /// Apply a complete timing configuration to a memory window.
    ///
    /// All fields other than `clkdiv` must only be changed while the QMI is
    /// idle.
    pub fn configure_timing(&self, window: Window, cfg: TimingConfig) {
        with_window!(self.qmi, window, m0_timing, m1_timing, reg, {
            reg.write_clkdiv(cfg.clkdiv as u32);
            reg.write_rxdelay(cfg.rxdelay as u32);
            reg.write_min_deselect(cfg.min_deselect as u32);
            reg.write_max_select(cfg.max_select as u32);
            reg.write_select_hold(cfg.select_hold as u32);
            if cfg.select_setup {
                reg.set_select_setup();
            } else {
                reg.clr_select_setup();
            }
            reg.write_pagebreak(cfg.pagebreak as u32);
            reg.write_cooldown(cfg.cooldown as u32);
        });
    }

    // ========================================================================
    // Memory-mapped (XIP) — transfer formats and commands
    // ========================================================================

    /// Configure the read transfer format for a memory window.
    pub fn configure_read_format(&self, window: Window, fmt: Format) {
        with_window!(self.qmi, window, m0_rfmt, m1_rfmt, reg, {
            write_format_reg!(reg, fmt);
        });
    }

    /// Configure the write transfer format for a memory window.
    ///
    /// XIP memory is read-only by default; writes must be enabled via the
    /// `XIP_CTRL` `WRITABLE_M0`/`WRITABLE_M1` bits.
    pub fn configure_write_format(&self, window: Window, fmt: Format) {
        with_window!(self.qmi, window, m0_wfmt, m1_wfmt, reg, {
            write_format_reg!(reg, fmt);
        });
    }

    /// Set the read command prefix and suffix bytes for a memory window.
    ///
    /// * `prefix` — command byte (e.g. `0x03` standard read, `0x6B` quad read)
    /// * `suffix` — suffix byte (e.g. `0xA0` for continuous read mode)
    pub fn set_read_command(&self, window: Window, prefix: u8, suffix: u8) {
        with_window!(self.qmi, window, m0_rcmd, m1_rcmd, reg, {
            reg.write_prefix(prefix as u32);
            reg.write_suffix(suffix as u32);
        });
    }

    /// Set the write command prefix and suffix bytes for a memory window.
    ///
    /// * `prefix` — command byte (e.g. `0x02` page program, `0x32` quad program)
    /// * `suffix` — suffix byte
    pub fn set_write_command(&self, window: Window, prefix: u8, suffix: u8) {
        with_window!(self.qmi, window, m0_wcmd, m1_wcmd, reg, {
            reg.write_prefix(prefix as u32);
            reg.write_suffix(suffix as u32);
        });
    }

    // ========================================================================
    // Memory-mapped (XIP) — address translation
    // ========================================================================

    /// Set the address translation for one of the eight 4 MiB XIP windows.
    ///
    /// * `window` — translation window index (0-7)
    /// * `base` — physical base address, in 4 KiB units (12-bit)
    /// * `size` — aperture size, in 4 KiB units
    ///
    /// `size` is written before `base` because the translation is latched when
    /// `base` changes. A cache flush is required after changing translation.
    ///
    /// # Panics
    ///
    /// Panics if `window >= 8`.
    pub fn set_address_translation(&self, window: u8, base: u32, size: u32) {
        assert!(
            window < 8,
            "address translation window must be 0-7, got {window}"
        );
        match window {
            0 => write_atrans!(&self.qmi.atrans0, base, size),
            1 => write_atrans!(&self.qmi.atrans1, base, size),
            2 => write_atrans!(&self.qmi.atrans2, base, size),
            3 => write_atrans!(&self.qmi.atrans3, base, size),
            4 => write_atrans!(&self.qmi.atrans4, base, size),
            5 => write_atrans!(&self.qmi.atrans5, base, size),
            6 => write_atrans!(&self.qmi.atrans6, base, size),
            _ => write_atrans!(&self.qmi.atrans7, base, size),
        }
    }

    // ========================================================================
    // High-level configuration presets
    // ========================================================================

    /// Standard SPI read (`0x03`): single-width, no dummy, no suffix.
    pub fn configure_standard_read(&self, window: Window, clkdiv: u8) {
        self.set_clkdiv(window, clkdiv);
        self.configure_read_format(window, Format::default());
        self.set_read_command(window, 0x03, 0x00);
    }

    /// Fast read (`0x0B`): single-width, 8 dummy bits.
    pub fn configure_fast_read(&self, window: Window, clkdiv: u8) {
        self.set_clkdiv(window, clkdiv);
        self.configure_read_format(
            window,
            Format {
                dummy_len: 2, // 8 dummy bits
                ..Format::default()
            },
        );
        self.set_read_command(window, 0x0B, 0x00);
    }

    /// Dual output read (`0x3B`): single command/address, dual data, 8 dummy bits.
    pub fn configure_dual_read(&self, window: Window, clkdiv: u8) {
        self.set_clkdiv(window, clkdiv);
        self.configure_read_format(
            window,
            Format {
                data_width: BusWidth::Dual,
                dummy_len: 2,
                ..Format::default()
            },
        );
        self.set_read_command(window, 0x3B, 0x00);
    }

    /// Quad output read (`0x6B`): single command/address, quad data, 8 dummy bits.
    pub fn configure_quad_read(&self, window: Window, clkdiv: u8) {
        self.set_clkdiv(window, clkdiv);
        self.configure_read_format(
            window,
            Format {
                data_width: BusWidth::Quad,
                dummy_len: 2,
                ..Format::default()
            },
        );
        self.set_read_command(window, 0x6B, 0x00);
    }

    /// Quad I/O read (`0xEB`): single command, quad address and data, 8-bit
    /// continuous-read suffix, 4 dummy bits.
    pub fn configure_quad_io_read(&self, window: Window, clkdiv: u8) {
        self.set_clkdiv(window, clkdiv);
        self.configure_read_format(
            window,
            Format {
                addr_width: BusWidth::Quad,
                suffix_width: BusWidth::Quad,
                dummy_width: BusWidth::Quad,
                data_width: BusWidth::Quad,
                suffix_len: 2, // 8-bit suffix
                dummy_len: 1,  // 4 dummy bits (1 quad cycle)
                ..Format::default()
            },
        );
        self.set_read_command(window, 0xEB, 0xA0);
    }

    /// Standard page program (`0x02`): single-width write.
    pub fn configure_standard_write(&self, window: Window) {
        self.configure_write_format(window, Format::default());
        self.set_write_command(window, 0x02, 0x00);
    }

    /// Quad page program (`0x32`): single command/address, quad data.
    pub fn configure_quad_write(&self, window: Window) {
        self.configure_write_format(
            window,
            Format {
                data_width: BusWidth::Quad,
                ..Format::default()
            },
        );
        self.set_write_command(window, 0x32, 0x00);
    }
}

// ============================================================================
// Direct mode
// ============================================================================

/// Direct serial mode handle.
///
/// Obtained from [`Qspi::direct`]. In direct mode, software controls the chip
/// select lines and performs raw SPI frames through the TX/RX FIFOs.
/// Memory-mapped (XIP) accesses fault while direct mode is enabled.
///
/// All methods take `&self` and may be called from interrupt context.
pub struct Direct<'a> {
    qmi: &'a QMI,
}

impl Direct<'_> {
    /// DREQ selection for transmitting data via DMA in direct mode.
    pub const TX_DREQ: crate::dma::TreqSel = crate::dma::TreqSel::XipQmiTx;

    /// DREQ selection for receiving data via DMA in direct mode.
    pub const RX_DREQ: crate::dma::TreqSel = crate::dma::TreqSel::XipQmiRx;

    /// Address of the direct TX FIFO register, for DMA.
    #[inline]
    pub fn tx_address(&self) -> u32 {
        core::ptr::addr_of!(self.qmi.direct_tx) as u32
    }

    /// Address of the direct RX FIFO register, for DMA.
    #[inline]
    pub fn rx_address(&self) -> u32 {
        core::ptr::addr_of!(self.qmi.direct_rx) as u32
    }

    // ========================================================================
    // Mode control
    // ========================================================================

    /// Enable direct serial mode.
    ///
    /// Poll [`is_busy`](Self::is_busy) before asserting a chip select to make
    /// sure no memory-mapped transfer is still in progress.
    #[inline]
    pub fn enable(&self) {
        self.qmi.direct_csr.set_en();
    }

    /// Disable direct serial mode, returning to memory-mapped (XIP) mode.
    ///
    /// Ensure all transfers are complete and chip selects are deasserted first.
    #[inline]
    pub fn disable(&self) {
        self.qmi.direct_csr.clr_en();
    }

    /// Check if direct mode is enabled.
    #[inline]
    pub fn is_enabled(&self) -> bool {
        self.qmi.direct_csr.en()
    }

    /// Check if the QMI is busy with a transfer.
    #[inline]
    pub fn is_busy(&self) -> bool {
        self.qmi.direct_csr.busy()
    }

    /// Wait until the QMI is no longer busy.
    ///
    /// # Panics
    ///
    /// Panics if the QMI stays busy within [`crate::spin::SPIN_TIMEOUT`]
    /// iterations.
    #[inline]
    pub fn wait_idle(&self) {
        crate::spin::spin_wait("QMI idle", || !self.qmi.direct_csr.busy());
    }

    /// Wait until the QMI is idle, returning [`Error::Busy`] on timeout.
    #[inline]
    pub fn try_wait_idle(&self) -> Result<(), Error> {
        if crate::spin::try_spin_wait(|| !self.qmi.direct_csr.busy()) {
            Ok(())
        } else {
            Err(Error::Busy)
        }
    }

    // ========================================================================
    // Chip select
    // ========================================================================

    /// Drive a chip select low (`asserted = true`) or release it high.
    #[inline]
    pub fn select(&self, cs: Window, asserted: bool) {
        match cs {
            Window::M0 => {
                if asserted {
                    self.qmi.direct_csr.set_assert_cs0n();
                } else {
                    self.qmi.direct_csr.clr_assert_cs0n();
                }
            }
            Window::M1 => {
                if asserted {
                    self.qmi.direct_csr.set_assert_cs1n();
                } else {
                    self.qmi.direct_csr.clr_assert_cs1n();
                }
            }
        }
    }

    /// Enable or disable automatic chip select assertion whenever BUSY is set.
    #[inline]
    pub fn set_auto_cs(&self, cs: Window, enabled: bool) {
        match cs {
            Window::M0 => {
                if enabled {
                    self.qmi.direct_csr.set_auto_cs0n();
                } else {
                    self.qmi.direct_csr.clr_auto_cs0n();
                }
            }
            Window::M1 => {
                if enabled {
                    self.qmi.direct_csr.set_auto_cs1n();
                } else {
                    self.qmi.direct_csr.clr_auto_cs1n();
                }
            }
        }
    }

    // ========================================================================
    // Clock
    // ========================================================================

    /// Set the direct mode clock divisor (1-255; 0 encodes 256).
    #[inline]
    pub fn set_clkdiv(&self, div: u8) {
        self.qmi.direct_csr.write_clkdiv(div as u32);
    }

    /// Get the direct mode clock divisor.
    #[inline]
    pub fn clkdiv(&self) -> u8 {
        self.qmi.direct_csr.clkdiv() as u8
    }

    /// Set the direct mode RX sample delay (0-3 half-cycles).
    #[inline]
    pub fn set_rxdelay(&self, delay: u8) {
        self.qmi.direct_csr.write_rxdelay((delay & 0x3) as u32);
    }

    // ========================================================================
    // FIFO status
    // ========================================================================

    /// Check if the TX FIFO is full.
    #[inline]
    pub fn tx_full(&self) -> bool {
        self.qmi.direct_csr.txfull()
    }

    /// Check if the TX FIFO is empty.
    #[inline]
    pub fn tx_empty(&self) -> bool {
        self.qmi.direct_csr.txempty()
    }

    /// Check if the RX FIFO is full.
    #[inline]
    pub fn rx_full(&self) -> bool {
        self.qmi.direct_csr.rxfull()
    }

    /// Check if the RX FIFO is empty.
    #[inline]
    pub fn rx_empty(&self) -> bool {
        self.qmi.direct_csr.rxempty()
    }

    /// Current TX FIFO level (0-4).
    #[inline]
    pub fn tx_level(&self) -> u32 {
        self.qmi.direct_csr.txlevel()
    }

    /// Current RX FIFO level (0-4).
    #[inline]
    pub fn rx_level(&self) -> u32 {
        self.qmi.direct_csr.rxlevel()
    }

    // ========================================================================
    // Raw FIFO access
    // ========================================================================

    /// Push a raw [`DirectTxVal`] to the TX FIFO.
    #[inline]
    pub fn tx_raw(&self, val: DirectTxVal) {
        self.qmi.direct_tx.write_val(val);
    }

    /// Pop a raw 16-bit word from the RX FIFO.
    #[inline]
    pub fn rx_raw(&self) -> u16 {
        self.qmi.direct_rx.direct_rx() as u16
    }

    // ========================================================================
    // Transfers
    // ========================================================================

    /// Push one frame into the TX FIFO, blocking until there is space.
    fn push(&self, data: u32, width: BusWidth, oe: bool, no_push: bool, d16: bool) {
        if !crate::spin::try_spin_wait(|| !self.qmi.direct_csr.txfull()) {
            panic!("timed out waiting for QMI TX FIFO space");
        }
        self.write_frame(data, width, oe, no_push, d16);
    }

    /// Push one frame into the TX FIFO, returning [`Error::TxFull`] on timeout.
    fn try_push(
        &self,
        data: u32,
        width: BusWidth,
        oe: bool,
        no_push: bool,
        d16: bool,
    ) -> Result<(), Error> {
        if !crate::spin::try_spin_wait(|| !self.qmi.direct_csr.txfull()) {
            return Err(Error::TxFull);
        }
        self.write_frame(data, width, oe, no_push, d16);
        Ok(())
    }

    /// Write a frame to the TX FIFO without waiting for space.
    fn write_frame(&self, data: u32, width: BusWidth, oe: bool, no_push: bool, d16: bool) {
        let val = DirectTxVal::reset_val()
            .with_data(data)
            .with_iwidth(width as u32)
            .with_oe(oe)
            .with_nopush(no_push)
            .with_dwidth(d16);
        self.qmi.direct_tx.write_val(val);
    }

    /// Transmit one byte (blocking).
    ///
    /// Set `no_push` to suppress the corresponding RX FIFO entry — useful for
    /// command bytes.
    pub fn write_byte(&self, data: u8, width: BusWidth, no_push: bool) {
        self.push(data as u32, width, true, no_push, false);
    }

    /// Transmit one 16-bit word (blocking).
    pub fn write16(&self, data: u16, width: BusWidth, no_push: bool) {
        self.push(data as u32, width, true, no_push, true);
    }

    /// Receive one byte (blocking): clocks in data with a dummy frame and
    /// returns the received byte.
    pub fn read_byte(&self, width: BusWidth) -> u8 {
        self.push(0, width, false, false, false);
        crate::spin::spin_wait("QMI RX data", || !self.qmi.direct_csr.rxempty());
        self.qmi.direct_rx.direct_rx() as u8
    }

    /// Transmit a byte slice in single width (blocking).
    pub fn write(&self, data: &[u8]) {
        for &byte in data {
            self.write_byte(byte, BusWidth::Single, true);
        }
        self.wait_idle();
    }

    /// Receive bytes in single width (blocking).
    pub fn read(&self, buf: &mut [u8]) {
        for slot in buf.iter_mut() {
            *slot = self.read_byte(BusWidth::Single);
        }
    }

    /// Full-duplex transfer in single width (blocking).
    ///
    /// Writes `tx` while simultaneously reading into `rx`; the shorter length
    /// is used.
    pub fn transfer(&self, rx: &mut [u8], tx: &[u8]) {
        let len = rx.len().min(tx.len());
        for i in 0..len {
            self.push(tx[i] as u32, BusWidth::Single, true, false, false);
            crate::spin::spin_wait("QMI RX data", || !self.qmi.direct_csr.rxempty());
            rx[i] = self.qmi.direct_rx.direct_rx() as u8;
        }
    }

    /// Discard all data in the RX FIFO.
    #[inline]
    pub fn drain_rx(&self) {
        while !self.qmi.direct_csr.rxempty() {
            let _ = self.qmi.direct_rx.direct_rx();
        }
    }

    /// Transmit one byte, returning [`Error::TxFull`] on timeout.
    pub fn try_write_byte(&self, data: u8, width: BusWidth, no_push: bool) -> Result<(), Error> {
        self.try_push(data as u32, width, true, no_push, false)
    }

    /// Transmit one 16-bit word, returning [`Error::TxFull`] on timeout.
    pub fn try_write16(&self, data: u16, width: BusWidth, no_push: bool) -> Result<(), Error> {
        self.try_push(data as u32, width, true, no_push, true)
    }

    /// Receive one byte, returning an error on timeout.
    pub fn try_read_byte(&self, width: BusWidth) -> Result<u8, Error> {
        self.try_push(0, width, false, false, false)?;
        if !crate::spin::try_spin_wait(|| !self.qmi.direct_csr.rxempty()) {
            return Err(Error::RxEmpty);
        }
        Ok(self.qmi.direct_rx.direct_rx() as u8)
    }

    /// Transmit a byte slice in single width, returning an error on timeout.
    pub fn try_write(&self, data: &[u8]) -> Result<(), Error> {
        for &byte in data {
            self.try_write_byte(byte, BusWidth::Single, true)?;
        }
        self.try_wait_idle()
    }

    /// Receive bytes in single width, returning an error on timeout.
    pub fn try_read(&self, buf: &mut [u8]) -> Result<(), Error> {
        for slot in buf.iter_mut() {
            *slot = self.try_read_byte(BusWidth::Single)?;
        }
        Ok(())
    }

    /// Full-duplex transfer in single width, returning an error on timeout.
    ///
    /// Writes `tx` while simultaneously reading into `rx`; the shorter length
    /// is used.
    pub fn try_transfer(&self, rx: &mut [u8], tx: &[u8]) -> Result<(), Error> {
        let len = rx.len().min(tx.len());
        for i in 0..len {
            self.try_push(tx[i] as u32, BusWidth::Single, true, false, false)?;
            if !crate::spin::try_spin_wait(|| !self.qmi.direct_csr.rxempty()) {
                return Err(Error::RxEmpty);
            }
            rx[i] = self.qmi.direct_rx.direct_rx() as u8;
        }
        Ok(())
    }

    // ========================================================================
    // Common flash commands
    // ========================================================================

    /// Send a simple command with no response (e.g. Write Enable, Chip Erase).
    pub fn command(&self, cs: Window, cmd: u8) {
        self.select(cs, true);
        self.write_byte(cmd, BusWidth::Single, true);
        self.wait_idle();
        self.select(cs, false);
    }

    /// Send a simple command with no response, returning an error on timeout.
    pub fn try_command(&self, cs: Window, cmd: u8) -> Result<(), Error> {
        self.select(cs, true);
        self.try_write_byte(cmd, BusWidth::Single, true)?;
        self.try_wait_idle()?;
        self.select(cs, false);
        Ok(())
    }

    /// Read the flash status register (`0x05`).
    pub fn read_status(&self, cs: Window) -> u8 {
        self.select(cs, true);
        self.write_byte(0x05, BusWidth::Single, true);
        let status = self.read_byte(BusWidth::Single);
        self.wait_idle();
        self.select(cs, false);
        status
    }

    /// Read the flash status register (`0x05`), returning an error on timeout.
    pub fn try_read_status(&self, cs: Window) -> Result<u8, Error> {
        self.select(cs, true);
        self.try_write_byte(0x05, BusWidth::Single, true)?;
        let status = self.try_read_byte(BusWidth::Single)?;
        self.try_wait_idle()?;
        self.select(cs, false);
        Ok(status)
    }

    /// Wait until the flash WIP (Write-In-Progress) bit clears.
    ///
    /// # Panics
    ///
    /// Panics if the bit does not clear within [`crate::spin::SPIN_TIMEOUT`]
    /// polls.
    pub fn wait_ready(&self, cs: Window) {
        self.try_wait_ready(cs)
            .unwrap_or_else(|_| panic!("timed out waiting for flash to be ready"));
    }

    /// Wait until the flash WIP bit clears, returning [`Error::Busy`] if it
    /// does not clear within [`crate::spin::SPIN_TIMEOUT`] polls.
    pub fn try_wait_ready(&self, cs: Window) -> Result<(), Error> {
        for _ in 0..crate::spin::SPIN_TIMEOUT {
            if self.try_read_status(cs)? & 0x01 == 0 {
                return Ok(());
            }
            core::hint::spin_loop();
        }
        Err(Error::Busy)
    }

    /// Read the JEDEC ID (`0x9F`), returning (manufacturer, type, capacity).
    pub fn read_jedec_id(&self, cs: Window) -> (u8, u8, u8) {
        self.select(cs, true);
        self.write_byte(0x9F, BusWidth::Single, true);
        let mfr = self.read_byte(BusWidth::Single);
        let typ = self.read_byte(BusWidth::Single);
        let cap = self.read_byte(BusWidth::Single);
        self.wait_idle();
        self.select(cs, false);
        (mfr, typ, cap)
    }

    /// Read the JEDEC ID (`0x9F`), returning an error on timeout.
    pub fn try_read_jedec_id(&self, cs: Window) -> Result<(u8, u8, u8), Error> {
        self.select(cs, true);
        self.try_write_byte(0x9F, BusWidth::Single, true)?;
        let mfr = self.try_read_byte(BusWidth::Single)?;
        let typ = self.try_read_byte(BusWidth::Single)?;
        let cap = self.try_read_byte(BusWidth::Single)?;
        self.try_wait_idle()?;
        self.select(cs, false);
        Ok((mfr, typ, cap))
    }
}
