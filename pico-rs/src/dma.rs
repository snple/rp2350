//! DMA (Direct Memory Access) HAL for RP2350
//!
//! This module provides a type-safe, builder-pattern interface to the RP2350
//! DMA controller, supporting all 16 independent channels with configurable
//! transfers between memory and peripherals.
//!
//! # Architecture
//!
//! ```text
//!                    ┌─────────────────────────────────────────┐
//!                    │           DMA Controller                │
//!                    │                                         │
//!   ┌──────────┐    │  ┌────────┐ ┌────────┐           ×16  │
//!   │  Memory  │◄──►│  │  Ch 0  │ │  Ch 1  │  ...           │
//!   └──────────┘    │  │ rd/wr  │ │ rd/wr  │                │
//!                    │  │ count  │ │ count  │                │
//!   ┌──────────┐    │  │ ctrl   │ │ ctrl   │                │
//!   │  UART    │◄──►│  └────────┘ └────────┘                │
//!   └──────────┘    │                                         │
//!                    │  ┌──────────┐  ┌───────────┐           │
//!   ┌──────────┐    │  │  Sniff   │  │  Pacing   │           │
//!   │  SPI     │◄──►│  │  (CRC)   │  │  Timers   │           │
//!   └──────────┘    │  └──────────┘  └───────────┘           │
//!                    └─────────────────────────────────────────┘
//! ```
//!
//! # Builder Pattern
//!
//! Every channel setter returns `&Self`, enabling fluent configuration
//! chains that read naturally and are hard to misconfigure:
//!
//! ```ignore
//! ch.set_data_size(DataSize::Word)
//!     .set_treq_sel(TreqSel::Permanent)
//!     .set_incr_read(true)
//!     .set_incr_write(true)
//!     .set_addrs(src, dst)
//!     .set_trans_count(4)
//!     .trigger()
//!     .wait();
//! ```
//!
//! # Transfer Modes
//!
//! - **Memory-to-Memory**: Use [`TreqSel::Permanent`] for maximum throughput
//! - **Memory-to-Peripheral (TX)**: Paced by peripheral DREQ (e.g. UART TX FIFO)
//! - **Peripheral-to-Memory (RX)**: Paced by peripheral DREQ (e.g. UART RX FIFO)
//!
//! # Example — Memory-to-Memory
//!
//! ```ignore
//! use pico_rs::dma::{Dma, DataSize, TreqSel};
//!
//! let dma = Dma::new(dp.DMA);
//! let ch = dma.channel::<0>();
//!
//! let src = [1u32, 2, 3, 4];
//! let mut dst = [0u32; 4];
//!
//! ch.mem_to_mem(
//!     src.as_ptr() as u32,
//!     dst.as_mut_ptr() as u32,
//!     src.len() as u32,
//!     DataSize::Word,
//! )
//! .trigger()
//! .wait();
//! ```
//!
//! # Example — UART TX via DMA
//!
//! ```ignore
//! use pico_rs::dma::{Dma, DataSize};
//! use pico_rs::uart::Uart0;
//!
//! let dma = Dma::new(dp.DMA);
//! let ch = dma.channel::<0>();
//!
//! uart.enable_dma_tx();
//!
//! ch.mem_to_periph(
//!     data.as_ptr() as u32,
//!     uart.data_register_address(),
//!     data.len() as u32,
//!     Uart0::<_, _>::TX_DREQ,
//!     DataSize::Byte,
//! )
//! .trigger()
//! .wait();
//! ```

use core::marker::PhantomData;

use paste::paste;
use pico_rs_pac::DMA;

use crate::reset::Resettable;

/// Total number of DMA channels available on RP2350.
pub const NUM_CHANNELS: usize = 16;

// ============================================================================
// Enumerations
// ============================================================================

/// DMA data transfer size.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u32)]
pub enum DataSize {
    /// 8-bit (byte) transfers
    Byte = 0,
    /// 16-bit (halfword) transfers
    HalfWord = 1,
    /// 32-bit (word) transfers
    Word = 2,
}

/// DMA transfer request (DREQ) selection.
///
/// Selects the pacing source for a DMA channel. Each peripheral asserts its
/// own DREQ signal when ready for data. Use [`TreqSel::Permanent`] for
/// unpaced (maximum rate) transfers such as memory-to-memory.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u32)]
pub enum TreqSel {
    /// PIO0 TX0
    Pio0Tx0 = 0,
    /// PIO0 TX1
    Pio0Tx1 = 1,
    /// PIO0 TX2
    Pio0Tx2 = 2,
    /// PIO0 TX3
    Pio0Tx3 = 3,
    /// PIO0 RX0
    Pio0Rx0 = 4,
    /// PIO0 RX1
    Pio0Rx1 = 5,
    /// PIO0 RX2
    Pio0Rx2 = 6,
    /// PIO0 RX3
    Pio0Rx3 = 7,
    /// PIO1 TX0
    Pio1Tx0 = 8,
    /// PIO1 TX1
    Pio1Tx1 = 9,
    /// PIO1 TX2
    Pio1Tx2 = 10,
    /// PIO1 TX3
    Pio1Tx3 = 11,
    /// PIO1 RX0
    Pio1Rx0 = 12,
    /// PIO1 RX1
    Pio1Rx1 = 13,
    /// PIO1 RX2
    Pio1Rx2 = 14,
    /// PIO1 RX3
    Pio1Rx3 = 15,
    /// PIO2 TX0
    Pio2Tx0 = 16,
    /// PIO2 TX1
    Pio2Tx1 = 17,
    /// PIO2 TX2
    Pio2Tx2 = 18,
    /// PIO2 TX3
    Pio2Tx3 = 19,
    /// PIO2 RX0
    Pio2Rx0 = 20,
    /// PIO2 RX1
    Pio2Rx1 = 21,
    /// PIO2 RX2
    Pio2Rx2 = 22,
    /// PIO2 RX3
    Pio2Rx3 = 23,
    /// SPI0 TX
    Spi0Tx = 24,
    /// SPI0 RX
    Spi0Rx = 25,
    /// SPI1 TX
    Spi1Tx = 26,
    /// SPI1 RX
    Spi1Rx = 27,
    /// UART0 TX
    Uart0Tx = 28,
    /// UART0 RX
    Uart0Rx = 29,
    /// UART1 TX
    Uart1Tx = 30,
    /// UART1 RX
    Uart1Rx = 31,
    /// PWM Wrap 0
    PwmWrap0 = 32,
    /// PWM Wrap 1
    PwmWrap1 = 33,
    /// PWM Wrap 2
    PwmWrap2 = 34,
    /// PWM Wrap 3
    PwmWrap3 = 35,
    /// PWM Wrap 4
    PwmWrap4 = 36,
    /// PWM Wrap 5
    PwmWrap5 = 37,
    /// PWM Wrap 6
    PwmWrap6 = 38,
    /// PWM Wrap 7
    PwmWrap7 = 39,
    /// PWM Wrap 8
    PwmWrap8 = 40,
    /// PWM Wrap 9
    PwmWrap9 = 41,
    /// PWM Wrap 10
    PwmWrap10 = 42,
    /// PWM Wrap 11
    PwmWrap11 = 43,
    /// I2C0 TX
    I2c0Tx = 44,
    /// I2C0 RX
    I2c0Rx = 45,
    /// I2C1 TX
    I2c1Tx = 46,
    /// I2C1 RX
    I2c1Rx = 47,
    /// ADC
    Adc = 48,
    /// XIP Stream
    XipStream = 49,
    /// XIP QMI TX
    XipQmiTx = 50,
    /// XIP QMI RX
    XipQmiRx = 51,
    /// HSTX
    Hstx = 52,
    /// Coresight
    Coresight = 53,
    /// SHA256
    Sha256 = 54,
    /// Timer 0
    Timer0 = 59,
    /// Timer 1
    Timer1 = 60,
    /// Timer 2
    Timer2 = 61,
    /// Timer 3
    Timer3 = 62,
    /// Permanent request — always asserted, for unpaced transfers
    Permanent = 63,
}

/// Transfer count mode (upper 4 bits of TRANS_COUNT register).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u32)]
pub enum TransCountMode {
    /// Normal mode — decrement count until zero, then raise IRQ
    Normal = 0,
    /// Trigger self — reload count from DBG_TCR when complete
    TriggerSelf = 1,
    /// Endless — infinite transfers, never raises completion IRQ
    Endless = 15,
}

/// Sniff (CRC / checksum) calculation type.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u32)]
pub enum SniffCalc {
    /// CRC-32 (IEEE 802.3 polynomial)
    Crc32 = 0,
    /// CRC-32 with bit-reversed data
    Crc32Rev = 1,
    /// CRC-16 CCITT
    Crc16 = 2,
    /// CRC-16 CCITT with bit-reversed data
    Crc16Rev = 3,
    /// XOR reduction (even parity)
    Even = 14,
    /// Summation
    Sum = 15,
}

// ============================================================================
// DMA Controller
// ============================================================================

/// DMA controller, owning the PAC peripheral.
///
/// Create once via [`Dma::new()`], then obtain individual [`Channel`] handles
/// via [`Dma::channel()`].
pub struct Dma {
    dma: pico_rs_pac::DMA,
}

impl Dma {
    /// Create a new DMA controller, consuming the PAC peripheral.
    ///
    /// This brings the DMA block out of reset and returns a controller
    /// from which individual channels can be obtained.
    ///
    /// # Arguments
    ///
    /// * `dma` — The DMA peripheral from `Peripherals::take()`
    pub fn new(dma: pico_rs_pac::DMA) -> Self {
        pico_rs_pac::DMA::unreset();
        Self { dma }
    }

    // ========================================================================
    // Channel Access
    // ========================================================================

    /// Obtain a handle to DMA channel `N` (0–15).
    ///
    /// The returned [`Channel`] is a zero-sized type that accesses the
    /// hardware registers directly. Multiple channels can coexist
    /// simultaneously.
    ///
    /// # Panics
    ///
    /// Compile-time error if `N >= 16`.
    #[inline]
    pub const fn channel<const N: u8>(&self) -> Channel<N> {
        const { assert!(N < 16, "DMA channel number must be 0-15") };
        Channel {
            _marker: PhantomData,
        }
    }

    // ========================================================================
    // Global DMA Register Helpers
    // ========================================================================

    #[inline]
    fn regs(&self) -> &pico_rs_pac::dma::RegisterBlock {
        &self.dma
    }

    /// Report the number of DMA channels present in hardware.
    #[inline]
    pub fn num_channels(&self) -> u32 {
        self.regs().n_channels.n_channels()
    }

    // ========================================================================
    // Multi-channel Operations
    // ========================================================================

    /// Trigger multiple channels simultaneously.
    ///
    /// Each set bit in `mask` triggers the corresponding channel.
    #[inline]
    pub fn multi_trigger(&self, mask: u16) {
        self.regs()
            .multi_chan_trigger
            .write_multi_chan_trigger(mask as u32);
    }

    /// Abort transfers on channels indicated by `mask`.
    #[inline]
    pub fn abort(&self, mask: u16) {
        self.regs().chan_abort.write_chan_abort(mask as u32);
    }

    // ========================================================================
    // Interrupt Control — IRQ 0
    // ========================================================================

    /// Read raw (unmasked) interrupt status for all channels.
    #[inline]
    pub fn raw_interrupts(&self) -> u16 {
        self.regs().intr.intr() as u16
    }

    /// Enable completion interrupt on IRQ 0 for the given channel.
    #[inline]
    pub fn enable_irq0(&self, channel: u8) {
        debug_assert!(channel < 16);
        self.regs().inte0.set_bits(1 << channel);
    }

    /// Disable completion interrupt on IRQ 0 for the given channel.
    #[inline]
    pub fn disable_irq0(&self, channel: u8) {
        debug_assert!(channel < 16);
        self.regs().inte0.clr_bits(1 << channel);
    }

    /// Read masked interrupt status for IRQ 0.
    #[inline]
    pub fn ints0(&self) -> u16 {
        self.regs().ints0.ints0() as u16
    }

    /// Acknowledge (clear) IRQ 0 interrupts for the given channel mask.
    #[inline]
    pub fn ack_irq0(&self, mask: u16) {
        self.regs().ints0.write_bits(mask as u32);
    }

    // ========================================================================
    // Interrupt Control — IRQ 1
    // ========================================================================

    /// Enable completion interrupt on IRQ 1 for the given channel.
    #[inline]
    pub fn enable_irq1(&self, channel: u8) {
        debug_assert!(channel < 16);
        self.regs().inte1.set_bits(1 << channel);
    }

    /// Disable completion interrupt on IRQ 1 for the given channel.
    #[inline]
    pub fn disable_irq1(&self, channel: u8) {
        debug_assert!(channel < 16);
        self.regs().inte1.clr_bits(1 << channel);
    }

    /// Read masked interrupt status for IRQ 1.
    #[inline]
    pub fn ints1(&self) -> u16 {
        self.regs().ints1.ints1() as u16
    }

    /// Acknowledge (clear) IRQ 1 interrupts for the given channel mask.
    #[inline]
    pub fn ack_irq1(&self, mask: u16) {
        self.regs().ints1.write_bits(mask as u32);
    }

    // ========================================================================
    // Pacing Timers
    // ========================================================================

    /// Configure a fractional pacing timer.
    ///
    /// The timer fires at rate: `sys_clk × numerator / denominator`.
    /// Timer index must be 0–3.
    pub fn set_timer(&self, index: u8, numerator: u16, denominator: u16) {
        let val = ((numerator as u32) << 16) | (denominator as u32);
        let regs = self.regs();
        match index {
            0 => regs.timer0.write_bits(val),
            1 => regs.timer1.write_bits(val),
            2 => regs.timer2.write_bits(val),
            3 => regs.timer3.write_bits(val),
            _ => panic!("DMA timer index must be 0-3"),
        }
    }

    // ========================================================================
    // Sniff (CRC / Checksum) Hardware
    // ========================================================================

    /// Enable or disable the sniff hardware block.
    #[inline]
    pub fn sniff_enable(&self, enable: bool) {
        if enable {
            self.regs().sniff_ctrl.set_en();
        } else {
            self.regs().sniff_ctrl.clr_en();
        }
    }

    /// Set which channel the sniff hardware monitors.
    #[inline]
    pub fn sniff_set_channel(&self, channel: u8) {
        self.regs().sniff_ctrl.write_dmach(channel as u32 & 0xF);
    }

    /// Set the sniff calculation type.
    #[inline]
    pub fn sniff_set_calc(&self, calc: SniffCalc) {
        self.regs().sniff_ctrl.write_calc(calc as u32);
    }

    /// Read the accumulated sniff result (CRC / checksum).
    #[inline]
    pub fn sniff_data(&self) -> u32 {
        self.regs().sniff_data.sniff_data()
    }

    /// Write a seed value into the sniff accumulator.
    #[inline]
    pub fn sniff_set_data(&self, seed: u32) {
        self.regs().sniff_data.write_sniff_data(seed);
    }

    /// Enable or disable byte-swap on sniff input data.
    #[inline]
    pub fn sniff_set_bswap(&self, enable: bool) {
        if enable {
            self.regs().sniff_ctrl.set_bswap();
        } else {
            self.regs().sniff_ctrl.clr_bswap();
        }
    }

    /// Enable or disable bit inversion on sniff output.
    #[inline]
    pub fn sniff_set_out_inv(&self, enable: bool) {
        if enable {
            self.regs().sniff_ctrl.set_out_inv();
        } else {
            self.regs().sniff_ctrl.clr_out_inv();
        }
    }

    /// Enable or disable bit reversal on sniff output.
    #[inline]
    pub fn sniff_set_out_rev(&self, enable: bool) {
        if enable {
            self.regs().sniff_ctrl.set_out_rev();
        } else {
            self.regs().sniff_ctrl.clr_out_rev();
        }
    }

    // ========================================================================
    // FIFO Debug Levels
    // ========================================================================

    /// Get the transfer-data FIFO level.
    #[inline]
    pub fn tdf_level(&self) -> u8 {
        self.regs().fifo_levels.tdf_lvl() as u8
    }

    /// Get the write-address FIFO level.
    #[inline]
    pub fn waf_level(&self) -> u8 {
        self.regs().fifo_levels.waf_lvl() as u8
    }

    /// Get the read-address FIFO level.
    #[inline]
    pub fn raf_level(&self) -> u8 {
        self.regs().fifo_levels.raf_lvl() as u8
    }
}

// ============================================================================
// DMA Channel
// ============================================================================

/// A single DMA channel, parameterised by channel number `N` (0–15).
///
/// This is a zero-sized handle that provides type-safe, per-channel operations.
/// All setter methods return `&Self` for builder-pattern chaining.
///
/// # Example
///
/// ```ignore
/// dma.channel::<0>()
///     .disable()
///     .set_data_size(DataSize::Word)
///     .set_treq_sel(TreqSel::Permanent)
///     .set_incr_read(true)
///     .set_incr_write(true)
///     .set_addrs(src, dst)
///     .set_trans_count(count)
///     .trigger()
///     .wait();
/// ```
pub struct Channel<const N: u8> {
    _marker: PhantomData<()>,
}

/// Internal helper — shared register-block accessor.
#[inline(always)]
fn dma_regs() -> pico_rs_pac::DMA {
    unsafe { DMA::steal() }
}

// Generate per-channel implementations for all 16 channels.
// Each channel's registers are individually named in the PAC (ch0_*, ch1_*, …, ch15_*),
// so we use `paste!` to assemble the correct field / method names.
macro_rules! impl_channel {
    ($n:literal) => {
        paste! {
            impl Channel<$n> {
                // ============================================================
                // Address Registers
                // ============================================================

                /// Set the source (read) address for this channel.
                #[inline]
                pub fn set_read_addr(&self, addr: u32) -> &Self {
                    dma_regs().[<ch $n _read_addr>].[<write_ch $n _read_addr>](addr);
                    self
                }

                /// Get the current source (read) address.
                #[inline]
                pub fn read_addr(&self) -> u32 {
                    dma_regs().[<ch $n _read_addr>].[<ch $n _read_addr>]()
                }

                /// Set the destination (write) address for this channel.
                #[inline]
                pub fn set_write_addr(&self, addr: u32) -> &Self {
                    dma_regs().[<ch $n _write_addr>].[<write_ch $n _write_addr>](addr);
                    self
                }

                /// Get the current destination (write) address.
                #[inline]
                pub fn write_addr(&self) -> u32 {
                    dma_regs().[<ch $n _write_addr>].[<ch $n _write_addr>]()
                }

                /// Set both read and write addresses in one call.
                #[inline]
                pub fn set_addrs(&self, read: u32, write: u32) -> &Self {
                    self.set_read_addr(read);
                    self.set_write_addr(write);
                    self
                }

                // ============================================================
                // Transfer Count
                // ============================================================

                /// Set the transfer count (number of data elements).
                #[inline]
                pub fn set_trans_count(&self, count: u32) -> &Self {
                    dma_regs().[<ch $n _trans_count>].write_count(count);
                    self
                }

                /// Set the transfer count mode (normal, trigger-self, endless).
                #[inline]
                pub fn set_trans_count_mode(&self, mode: TransCountMode) -> &Self {
                    dma_regs().[<ch $n _trans_count>].write_mode(mode as u32);
                    self
                }

                /// Get the remaining transfer count.
                #[inline]
                pub fn trans_count(&self) -> u32 {
                    dma_regs().[<ch $n _trans_count>].count()
                }

                // ============================================================
                // Control / Status (CTRL_TRIG)
                // ============================================================

                /// Enable (start) this channel.
                #[inline]
                pub fn enable(&self) -> &Self {
                    dma_regs().[<ch $n _ctrl_trig>].set_en();
                    self
                }

                /// Disable this channel.
                #[inline]
                pub fn disable(&self) -> &Self {
                    dma_regs().[<ch $n _ctrl_trig>].clr_en();
                    self
                }

                /// Check whether this channel is currently transferring data.
                #[inline]
                pub fn is_busy(&self) -> bool {
                    dma_regs().[<ch $n _ctrl_trig>].busy()
                }

                /// Set the transfer data-element size.
                #[inline]
                pub fn set_data_size(&self, size: DataSize) -> &Self {
                    dma_regs().[<ch $n _ctrl_trig>].write_data_size(size as u32);
                    self
                }

                /// Set the DREQ pacing source.
                #[inline]
                pub fn set_treq_sel(&self, treq: TreqSel) -> &Self {
                    dma_regs().[<ch $n _ctrl_trig>].write_treq_sel(treq as u32);
                    self
                }

                /// Enable or disable read-address increment.
                #[inline]
                pub fn set_incr_read(&self, incr: bool) -> &Self {
                    if incr {
                        dma_regs().[<ch $n _ctrl_trig>].set_incr_read();
                    } else {
                        dma_regs().[<ch $n _ctrl_trig>].clr_incr_read();
                    }
                    self
                }

                /// Enable or disable write-address increment.
                #[inline]
                pub fn set_incr_write(&self, incr: bool) -> &Self {
                    if incr {
                        dma_regs().[<ch $n _ctrl_trig>].set_incr_write();
                    } else {
                        dma_regs().[<ch $n _ctrl_trig>].clr_incr_write();
                    }
                    self
                }

                /// Set the chain-to channel (triggers channel `ch` on completion).
                ///
                /// Set to this channel's own number (`N`) to disable chaining.
                #[inline]
                pub fn set_chain_to(&self, ch: u8) -> &Self {
                    dma_regs().[<ch $n _ctrl_trig>].write_chain_to(ch as u32 & 0xF);
                    self
                }

                /// Set the ring-buffer wrap side (read or write).
                ///
                /// * `false` — ring wraps the read address
                /// * `true` — ring wraps the write address
                #[inline]
                pub fn set_ring_sel(&self, write_side: bool) -> &Self {
                    if write_side {
                        dma_regs().[<ch $n _ctrl_trig>].set_ring_sel();
                    } else {
                        dma_regs().[<ch $n _ctrl_trig>].clr_ring_sel();
                    }
                    self
                }

                /// Set the ring-buffer size as log₂(bytes). 0 disables the ring.
                #[inline]
                pub fn set_ring_size(&self, log2_bytes: u8) -> &Self {
                    dma_regs().[<ch $n _ctrl_trig>]
                        .write_ring_size(log2_bytes as u32 & 0xF);
                    self
                }

                /// Enable or disable high-priority scheduling.
                #[inline]
                pub fn set_high_priority(&self, high: bool) -> &Self {
                    if high {
                        dma_regs().[<ch $n _ctrl_trig>].set_high_priority();
                    } else {
                        dma_regs().[<ch $n _ctrl_trig>].clr_high_priority();
                    }
                    self
                }

                /// Enable or disable byte-swap on transfer data.
                #[inline]
                pub fn set_bswap(&self, enable: bool) -> &Self {
                    if enable {
                        dma_regs().[<ch $n _ctrl_trig>].set_bswap();
                    } else {
                        dma_regs().[<ch $n _ctrl_trig>].clr_bswap();
                    }
                    self
                }

                /// Enable or disable IRQ quiet mode.
                ///
                /// When quiet, the channel does not raise an interrupt on completion.
                #[inline]
                pub fn set_irq_quiet(&self, quiet: bool) -> &Self {
                    if quiet {
                        dma_regs().[<ch $n _ctrl_trig>].set_irq_quiet();
                    } else {
                        dma_regs().[<ch $n _ctrl_trig>].clr_irq_quiet();
                    }
                    self
                }

                /// Enable or disable sniff hardware on this channel.
                #[inline]
                pub fn set_sniff_en(&self, enable: bool) -> &Self {
                    if enable {
                        dma_regs().[<ch $n _ctrl_trig>].set_sniff_en();
                    } else {
                        dma_regs().[<ch $n _ctrl_trig>].clr_sniff_en();
                    }
                    self
                }

                // ============================================================
                // Error Handling
                // ============================================================

                /// Check for a read-bus error.
                #[inline]
                pub fn read_error(&self) -> bool {
                    dma_regs().[<ch $n _ctrl_trig>].read_error()
                }

                /// Check for a write-bus error.
                #[inline]
                pub fn write_error(&self) -> bool {
                    dma_regs().[<ch $n _ctrl_trig>].write_error()
                }

                /// Clear the read-bus error flag (write-1-to-clear).
                #[inline]
                pub fn clear_read_error(&self) -> &Self {
                    dma_regs().[<ch $n _ctrl_trig>].set_read_error();
                    self
                }

                /// Clear the write-bus error flag (write-1-to-clear).
                #[inline]
                pub fn clear_write_error(&self) -> &Self {
                    dma_regs().[<ch $n _ctrl_trig>].set_write_error();
                    self
                }

                /// Check for an AHB bus error.
                #[inline]
                pub fn ahb_error(&self) -> bool {
                    dma_regs().[<ch $n _ctrl_trig>].ahb_error()
                }

                // ============================================================
                // Alias Registers (trigger variants)
                // ============================================================

                /// Write to the Alias-1 TRANS_COUNT_TRIG register.
                ///
                /// Writing here sets the transfer count **and** triggers
                /// the channel to start.
                #[inline]
                pub fn write_trans_count_trig(&self, count: u32) -> &Self {
                    dma_regs().[<ch $n _al1_trans_count_trig>]
                        .[<write_ch $n _al1_trans_count_trig>](count);
                    self
                }

                /// Write to the Alias-3 READ_ADDR_TRIG register.
                ///
                /// Writing here sets the read address **and** triggers
                /// the channel to start.
                #[inline]
                pub fn write_read_addr_trig(&self, addr: u32) -> &Self {
                    dma_regs().[<ch $n _al3_read_addr_trig>]
                        .[<write_ch $n _al3_read_addr_trig>](addr);
                    self
                }

                /// Write to the Alias-2 WRITE_ADDR_TRIG register.
                ///
                /// Writing here sets the write address **and** triggers
                /// the channel to start.
                #[inline]
                pub fn write_write_addr_trig(&self, addr: u32) -> &Self {
                    dma_regs().[<ch $n _al2_write_addr_trig>]
                        .[<write_ch $n _al2_write_addr_trig>](addr);
                    self
                }

                // ============================================================
                // Trigger / Wait / Abort
                // ============================================================

                /// Start (trigger) this channel by setting the EN bit.
                #[inline]
                pub fn trigger(&self) -> &Self {
                    dma_regs().[<ch $n _ctrl_trig>].set_en();
                    self
                }

                /// Spin-wait until this channel's transfer completes.
                #[inline]
                pub fn wait(&self) -> &Self {
                    while self.is_busy() {
                        core::hint::spin_loop();
                    }
                    self
                }

                /// Abort this channel's in-progress transfer.
                #[inline]
                pub fn abort(&self) -> &Self {
                    dma_regs().chan_abort.write_chan_abort(1 << $n);
                    // Wait for abort to take effect
                    while self.is_busy() {
                        core::hint::spin_loop();
                    }
                    self
                }

                // ============================================================
                // Convenience — Common Transfer Patterns
                // ============================================================

                /// Set up a memory-to-memory transfer.
                ///
                /// Configures the channel for an unpaced transfer with both
                /// address pointers incrementing. Does **not** start the
                /// transfer — chain `.trigger()` to begin.
                ///
                /// ```ignore
                /// ch.mem_to_mem(src, dst, 4, DataSize::Word)
                ///     .trigger()
                ///     .wait();
                /// ```
                pub fn mem_to_mem(
                    &self,
                    src: u32,
                    dst: u32,
                    count: u32,
                    data_size: DataSize,
                ) -> &Self {
                    self.disable()
                        .set_data_size(data_size)
                        .set_treq_sel(TreqSel::Permanent)
                        .set_incr_read(true)
                        .set_incr_write(true)
                        .set_chain_to($n)
                        .set_irq_quiet(false)
                        .set_addrs(src, dst)
                        .set_trans_count(count)
                }

                /// Set up a memory-to-peripheral (TX) transfer.
                ///
                /// Read pointer increments through source buffer;
                /// write pointer is fixed at the peripheral data register.
                ///
                /// ```ignore
                /// ch.mem_to_periph(
                ///     data.as_ptr() as u32,
                ///     uart.data_register_address(),
                ///     data.len() as u32,
                ///     Uart0::<_, _>::TX_DREQ,
                ///     DataSize::Byte,
                /// )
                /// .trigger()
                /// .wait();
                /// ```
                pub fn mem_to_periph(
                    &self,
                    src: u32,
                    periph_addr: u32,
                    count: u32,
                    treq: TreqSel,
                    data_size: DataSize,
                ) -> &Self {
                    self.disable()
                        .set_data_size(data_size)
                        .set_treq_sel(treq)
                        .set_incr_read(true)
                        .set_incr_write(false)
                        .set_chain_to($n)
                        .set_irq_quiet(false)
                        .set_addrs(src, periph_addr)
                        .set_trans_count(count)
                }

                /// Set up a peripheral-to-memory (RX) transfer.
                ///
                /// Read pointer is fixed at the peripheral data register;
                /// write pointer increments through destination buffer.
                ///
                /// ```ignore
                /// ch.periph_to_mem(
                ///     uart.data_register_address(),
                ///     buf.as_mut_ptr() as u32,
                ///     buf.len() as u32,
                ///     Uart0::<_, _>::RX_DREQ,
                ///     DataSize::Byte,
                /// )
                /// .trigger()
                /// .wait();
                /// ```
                pub fn periph_to_mem(
                    &self,
                    periph_addr: u32,
                    dst: u32,
                    count: u32,
                    treq: TreqSel,
                    data_size: DataSize,
                ) -> &Self {
                    self.disable()
                        .set_data_size(data_size)
                        .set_treq_sel(treq)
                        .set_incr_read(false)
                        .set_incr_write(true)
                        .set_chain_to($n)
                        .set_irq_quiet(false)
                        .set_addrs(periph_addr, dst)
                        .set_trans_count(count)
                }
            }
        }
    };
}

// Expand for all 16 channels
impl_channel!(0);
impl_channel!(1);
impl_channel!(2);
impl_channel!(3);
impl_channel!(4);
impl_channel!(5);
impl_channel!(6);
impl_channel!(7);
impl_channel!(8);
impl_channel!(9);
impl_channel!(10);
impl_channel!(11);
impl_channel!(12);
impl_channel!(13);
impl_channel!(14);
impl_channel!(15);
