//! DMA (Direct Memory Access) HAL for RP2350
//!
//! This module provides access to the DMA controller for high-speed
//! data transfers between memory and peripherals.
//!
//! ## Features
//!
//! - 16 independent DMA channels
//! - Memory-to-memory, memory-to-peripheral, peripheral-to-memory transfers
//! - Configurable data sizes (byte, halfword, word)
//! - Ring buffer support
//! - Channel chaining
//! - CRC/checksum calculation via sniff hardware
//!
//! ## Example
//!
//! ```ignore
//! use pico_rs::dma::{Dma, DataSize, TreqSel};
//!
//! let dma = Dma::new();
//!
//! // Configure channel 0 for memory-to-memory transfer
//! dma.channel(0).configure(
//!     src_addr,
//!     dst_addr,
//!     count,
//!     DataSize::Word,
//!     TreqSel::Permanent,
//! );
//!
//! // Start the transfer
//! dma.channel(0).start();
//!
//! // Wait for completion
//! while dma.channel(0).is_busy() {}
//! ```

use pico_rs_pac::DMA;

/// Number of DMA channels available
pub const NUM_CHANNELS: usize = 16;

/// DMA data transfer size
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

/// DMA transfer request selection
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
    /// Permanent request (always active)
    Permanent = 63,
}

/// Transfer count mode
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u32)]
pub enum TransCountMode {
    /// Normal mode - decrement count until 0
    Normal = 0,
    /// Trigger self - reload from TCR at end
    TriggerSelf = 1,
    /// Endless mode - infinite transfers
    Endless = 15,
}

/// Sniff calculation type
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u32)]
pub enum SniffCalc {
    /// CRC-32 (IEEE802.3 polynomial)
    Crc32 = 0,
    /// CRC-32 reversed
    Crc32Rev = 1,
    /// CRC-16-CCITT
    Crc16 = 2,
    /// CRC-16-CCITT reversed
    Crc16Rev = 3,
    /// XOR accumulate
    Even = 14,
    /// Sum accumulate
    Sum = 15,
}

/// DMA controller wrapper
pub struct Dma {
    regs: &'static pico_rs_pac::dma::RegisterBlock,
}

impl Dma {
    /// Create a new DMA instance
    #[inline]
    pub fn new() -> Self {
        Self {
            regs: unsafe { &*DMA::PTR },
        }
    }

    /// Get reference to the register block
    #[inline]
    pub fn regs(&self) -> &pico_rs_pac::dma::RegisterBlock {
        self.regs
    }

    /// Get the number of DMA channels
    #[inline]
    pub fn num_channels(&self) -> u32 {
        self.regs.n_channels.n_channels()
    }

    // ========================================================================
    // Interrupt Status and Control
    // ========================================================================

    /// Get raw interrupt status
    #[inline]
    pub fn intr(&self) -> u16 {
        self.regs.intr.intr() as u16
    }

    /// Get interrupt status for IRQ 0
    #[inline]
    pub fn ints0(&self) -> u16 {
        self.regs.ints0.ints0() as u16
    }

    /// Enable interrupt for channel on IRQ 0
    #[inline]
    pub fn enable_irq0(&self, channel: u8) {
        if channel < 16 {
            self.regs.inte0.set_bits(1 << channel);
        }
    }

    /// Disable interrupt for channel on IRQ 0
    #[inline]
    pub fn disable_irq0(&self, channel: u8) {
        if channel < 16 {
            self.regs.inte0.clr_bits(1 << channel);
        }
    }

    /// Get interrupt status for IRQ 1
    #[inline]
    pub fn ints1(&self) -> u16 {
        self.regs.ints1.ints1() as u16
    }

    /// Enable interrupt for channel on IRQ 1
    #[inline]
    pub fn enable_irq1(&self, channel: u8) {
        if channel < 16 {
            self.regs.inte1.set_bits(1 << channel);
        }
    }

    /// Disable interrupt for channel on IRQ 1
    #[inline]
    pub fn disable_irq1(&self, channel: u8) {
        if channel < 16 {
            self.regs.inte1.clr_bits(1 << channel);
        }
    }

    // ========================================================================
    // Multi-channel Operations
    // ========================================================================

    /// Trigger multiple channels simultaneously
    #[inline]
    pub fn multi_trigger(&self, channels_mask: u16) {
        self.regs
            .multi_chan_trigger
            .write_multi_chan_trigger(channels_mask as u32);
    }

    /// Abort channels
    #[inline]
    pub fn abort(&self, channels_mask: u16) {
        self.regs.chan_abort.write_chan_abort(channels_mask as u32);
    }

    // ========================================================================
    // Pacing Timers
    // ========================================================================

    /// Configure pacing timer 0 (X/Y fractional)
    ///
    /// Timer produces TREQ assertions at rate: sys_clk * X / Y
    #[inline]
    pub fn set_timer0(&self, x: u16, y: u16) {
        let val = ((x as u32) << 16) | (y as u32);
        self.regs.timer0.write_bits(val);
    }

    /// Configure pacing timer 1
    #[inline]
    pub fn set_timer1(&self, x: u16, y: u16) {
        let val = ((x as u32) << 16) | (y as u32);
        self.regs.timer1.write_bits(val);
    }

    /// Configure pacing timer 2
    #[inline]
    pub fn set_timer2(&self, x: u16, y: u16) {
        let val = ((x as u32) << 16) | (y as u32);
        self.regs.timer2.write_bits(val);
    }

    /// Configure pacing timer 3
    #[inline]
    pub fn set_timer3(&self, x: u16, y: u16) {
        let val = ((x as u32) << 16) | (y as u32);
        self.regs.timer3.write_bits(val);
    }

    // ========================================================================
    // Sniff (CRC/Checksum) Hardware
    // ========================================================================

    /// Enable sniff hardware
    #[inline]
    pub fn sniff_enable(&self, enable: bool) {
        if enable {
            self.regs.sniff_ctrl.set_en();
        } else {
            self.regs.sniff_ctrl.clr_en();
        }
    }

    /// Set sniff channel to monitor
    #[inline]
    pub fn sniff_set_channel(&self, channel: u8) {
        self.regs.sniff_ctrl.write_dmach(channel as u32 & 0xF);
    }

    /// Set sniff calculation type
    #[inline]
    pub fn sniff_set_calc(&self, calc: SniffCalc) {
        self.regs.sniff_ctrl.write_calc(calc as u32);
    }

    /// Get sniff data (CRC/checksum result)
    #[inline]
    pub fn sniff_data(&self) -> u32 {
        self.regs.sniff_data.sniff_data()
    }

    /// Set sniff data (seed value)
    #[inline]
    pub fn sniff_set_data(&self, seed: u32) {
        self.regs.sniff_data.write_sniff_data(seed);
    }

    /// Enable byte swap in sniff
    #[inline]
    pub fn sniff_bswap(&self, enable: bool) {
        if enable {
            self.regs.sniff_ctrl.set_bswap();
        } else {
            self.regs.sniff_ctrl.clr_bswap();
        }
    }

    /// Enable output invert in sniff
    #[inline]
    pub fn sniff_out_inv(&self, enable: bool) {
        if enable {
            self.regs.sniff_ctrl.set_out_inv();
        } else {
            self.regs.sniff_ctrl.clr_out_inv();
        }
    }

    /// Enable output reverse (bit order) in sniff
    #[inline]
    pub fn sniff_out_rev(&self, enable: bool) {
        if enable {
            self.regs.sniff_ctrl.set_out_rev();
        } else {
            self.regs.sniff_ctrl.clr_out_rev();
        }
    }

    // ========================================================================
    // FIFO Debug Levels
    // ========================================================================

    /// Get transfer data FIFO level
    #[inline]
    pub fn tdf_level(&self) -> u8 {
        self.regs.fifo_levels.tdf_lvl() as u8
    }

    /// Get write address FIFO level
    #[inline]
    pub fn waf_level(&self) -> u8 {
        self.regs.fifo_levels.waf_lvl() as u8
    }

    /// Get read address FIFO level
    #[inline]
    pub fn raf_level(&self) -> u8 {
        self.regs.fifo_levels.raf_lvl() as u8
    }

    // ========================================================================
    // Channel Access (direct register access for channel 0 as example)
    // ========================================================================

    /// Configure channel 0 read address
    #[inline]
    pub fn ch0_set_read_addr(&self, addr: u32) {
        self.regs.ch0_read_addr.write_ch0_read_addr(addr);
    }

    /// Configure channel 0 write address
    #[inline]
    pub fn ch0_set_write_addr(&self, addr: u32) {
        self.regs.ch0_write_addr.write_ch0_write_addr(addr);
    }

    /// Configure channel 0 transfer count
    #[inline]
    pub fn ch0_set_trans_count(&self, count: u32) {
        self.regs.ch0_trans_count.write_count(count);
    }

    /// Get channel 0 read address
    #[inline]
    pub fn ch0_read_addr(&self) -> u32 {
        self.regs.ch0_read_addr.ch0_read_addr()
    }

    /// Get channel 0 write address
    #[inline]
    pub fn ch0_write_addr(&self) -> u32 {
        self.regs.ch0_write_addr.ch0_write_addr()
    }

    /// Get channel 0 transfer count
    #[inline]
    pub fn ch0_trans_count(&self) -> u32 {
        self.regs.ch0_trans_count.count()
    }

    /// Enable channel 0
    #[inline]
    pub fn ch0_enable(&self) {
        self.regs.ch0_ctrl_trig.set_en();
    }

    /// Disable channel 0
    #[inline]
    pub fn ch0_disable(&self) {
        self.regs.ch0_ctrl_trig.clr_en();
    }

    /// Check if channel 0 is busy
    #[inline]
    pub fn ch0_is_busy(&self) -> bool {
        self.regs.ch0_ctrl_trig.busy()
    }

    /// Check if channel 0 has read error
    #[inline]
    pub fn ch0_read_error(&self) -> bool {
        self.regs.ch0_ctrl_trig.read_error()
    }

    /// Check if channel 0 has write error
    #[inline]
    pub fn ch0_write_error(&self) -> bool {
        self.regs.ch0_ctrl_trig.write_error()
    }

    /// Clear channel 0 read error
    #[inline]
    pub fn ch0_clear_read_error(&self) {
        self.regs.ch0_ctrl_trig.set_read_error();
    }

    /// Clear channel 0 write error
    #[inline]
    pub fn ch0_clear_write_error(&self) {
        self.regs.ch0_ctrl_trig.set_write_error();
    }

    /// Set channel 0 data size
    #[inline]
    pub fn ch0_set_data_size(&self, size: DataSize) {
        self.regs.ch0_ctrl_trig.write_data_size(size as u32);
    }

    /// Set channel 0 TREQ selection
    #[inline]
    pub fn ch0_set_treq_sel(&self, treq: TreqSel) {
        self.regs.ch0_ctrl_trig.write_treq_sel(treq as u32);
    }

    /// Set channel 0 increment read address
    #[inline]
    pub fn ch0_set_incr_read(&self, incr: bool) {
        if incr {
            self.regs.ch0_ctrl_trig.set_incr_read();
        } else {
            self.regs.ch0_ctrl_trig.clr_incr_read();
        }
    }

    /// Set channel 0 increment write address
    #[inline]
    pub fn ch0_set_incr_write(&self, incr: bool) {
        if incr {
            self.regs.ch0_ctrl_trig.set_incr_write();
        } else {
            self.regs.ch0_ctrl_trig.clr_incr_write();
        }
    }

    /// Set channel 0 chain to
    #[inline]
    pub fn ch0_set_chain_to(&self, channel: u8) {
        self.regs.ch0_ctrl_trig.write_chain_to(channel as u32 & 0xF);
    }

    /// Set channel 0 high priority
    #[inline]
    pub fn ch0_set_high_priority(&self, high: bool) {
        if high {
            self.regs.ch0_ctrl_trig.set_high_priority();
        } else {
            self.regs.ch0_ctrl_trig.clr_high_priority();
        }
    }

    /// Set channel 0 IRQ quiet mode
    #[inline]
    pub fn ch0_set_irq_quiet(&self, quiet: bool) {
        if quiet {
            self.regs.ch0_ctrl_trig.set_irq_quiet();
        } else {
            self.regs.ch0_ctrl_trig.clr_irq_quiet();
        }
    }

    /// Set channel 0 byte swap
    #[inline]
    pub fn ch0_set_bswap(&self, enable: bool) {
        if enable {
            self.regs.ch0_ctrl_trig.set_bswap();
        } else {
            self.regs.ch0_ctrl_trig.clr_bswap();
        }
    }

    /// Enable sniff on channel 0
    #[inline]
    pub fn ch0_set_sniff_en(&self, enable: bool) {
        if enable {
            self.regs.ch0_ctrl_trig.set_sniff_en();
        } else {
            self.regs.ch0_ctrl_trig.clr_sniff_en();
        }
    }

    // ========================================================================
    // Helper for quick channel 0 setup
    // ========================================================================

    /// Configure and start a simple memory-to-memory DMA transfer on channel 0
    ///
    /// # Arguments
    /// * `src` - Source address
    /// * `dst` - Destination address
    /// * `count` - Number of transfers
    /// * `size` - Transfer data size
    /// * `start` - Whether to start immediately
    pub fn ch0_config_and_start(
        &self,
        src: u32,
        dst: u32,
        count: u32,
        size: DataSize,
        start: bool,
    ) {
        // Disable channel first
        self.ch0_disable();

        // Set addresses
        self.ch0_set_read_addr(src);
        self.ch0_set_write_addr(dst);
        self.ch0_set_trans_count(count);

        // Configure control
        self.ch0_set_data_size(size);
        self.ch0_set_treq_sel(TreqSel::Permanent);
        self.ch0_set_incr_read(true);
        self.ch0_set_incr_write(true);
        self.ch0_set_chain_to(0); // No chaining (chain to self disables)

        if start {
            self.ch0_enable();
        }
    }
}

impl Default for Dma {
    fn default() -> Self {
        Self::new()
    }
}
