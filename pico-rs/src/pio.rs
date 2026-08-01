//! Programmable I/O (PIO) driver for RP2350
//!
//! The RP2350 has 3 PIO blocks (PIO0, PIO1, PIO2), each with 4 state machines
//! and 32 instruction words of shared program memory.
//!
//! ## RP2350 Enhancements over RP2040
//!
//! - 3 PIO blocks (vs 2)
//! - GPIO Base: address pins 0-31 or 16-47
//! - Extended FIFO (PUTGET) modes for direct random access
//! - IN pin count: limit pins read by IN instruction
//! - MOV STATUS IRQ source
//! - Cross-PIO synchronization
//! - 8 IRQ flags (vs 4)
//! - MOV to PINDIRS destination
//!
//! ## Quick Start
//!
//! ```ignore
//! use pico_rs::gpio::{Pins, Pio0Pin};
//! use pico_rs::pio::{self, Pio0, FifoJoin};
//!
//! let pins = Pins::init(dp.IO_BANK0, dp.PADS_BANK0);
//! let mut pio0 = Pio0::init(dp.PIO0);
//!
//! // Claim GPIO pin (takes ownership, configures for PIO0)
//! pio0.claim_pin(pins.pin::<25>());
//!
//! // Install program
//! let program = pio_asm!(r#"
//!     set pindirs, 1
//!     .wrap_target
//!     set pins, 1 [31]
//!     set pins, 0 [31]
//!     .wrap
//! "#);
//! let installed = pio0.install_program(&program).unwrap();
//!
//! // Configure and start SM using builder pattern
//! pio0.sm0()
//!     .apply_program(&installed)  // sets wrap + side-set from program
//!     .set_clkdiv(256, 0)
//!     .set_set_pins(25, 1)
//!     .set_consecutive_pindirs(25, 1, true)
//!     .init(installed.offset);
//! ```

use paste::paste;

pub use pico_rs_macro::pio_asm;

use crate::Steal;
use crate::gpio;
use crate::reset::Resettable;

// ============================================================================
// Constants
// ============================================================================

/// Number of instruction slots in PIO program memory
pub const PIO_INSTRUCTION_COUNT: usize = 32;

// CTRL register bit positions (for atomic set/clr operations)
const CTRL_SM_ENABLE_LSB: u32 = 0;
const CTRL_SM_RESTART_LSB: u32 = 4;
const CTRL_CLKDIV_RESTART_LSB: u32 = 8;

// ============================================================================
// Error Types
// ============================================================================

/// Errors that can occur when installing a PIO program
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum InstallError {
    /// Not enough contiguous free instruction memory
    NoSpace,
    /// Requested origin address is invalid or overlaps
    InvalidOrigin,
}

// ============================================================================
// Enums
// ============================================================================

/// FIFO join modes
///
/// Controls how the TX and RX FIFOs are combined. Each SM normally has
/// 4-entry TX and 4-entry RX FIFOs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FifoJoin {
    /// Separate TX and RX FIFOs (4 entries each)
    None = 0,
    /// TX steals RX storage (8 TX, 0 RX)
    Tx = 1,
    /// RX steals TX storage (0 TX, 8 RX)
    Rx = 2,
    /// (RP2350) Extended joined FIFO, no direct access
    PutGet0 = 3,
    /// (RP2350) Extended FIFO with direct put
    PutGet1 = 7,
    /// (RP2350) Extended FIFO with direct get
    PutGet2 = 11,
    /// (RP2350) Extended FIFO with direct put and get
    PutGet3 = 15,
}

/// MOV STATUS source selection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MovStatusType {
    /// STATUS is all-ones if TX FIFO level < N
    TxLessThan = 0,
    /// STATUS is all-ones if RX FIFO level < N
    RxLessThan = 1,
    /// (RP2350) STATUS is all-ones if IRQ flag N is set
    IrqSet = 2,
}

// ============================================================================
// InstalledProgram
// ============================================================================

/// A compiled PIO program produced by the `pio_asm!` macro
///
/// Contains instruction words and metadata (wrap, origin, side-set).
#[derive(Debug, Clone, Copy)]
pub struct PioProgram {
    /// Machine-code instructions
    pub instructions: &'static [u16],
    /// Wrap target (bottom) offset within the program
    pub wrap_target: u8,
    /// Wrap top offset within the program
    pub wrap: u8,
    /// Fixed origin address, or `None` for automatic placement
    pub origin: Option<u8>,
    /// Number of side-set bits (0-5)
    pub side_set_count: u8,
    /// Whether the MSB of side-set is an enable flag
    pub side_set_opt: bool,
    /// Whether side-set affects pin directions instead of values
    pub side_set_pindirs: bool,
}

/// Handle to a program installed in PIO instruction memory
///
/// Contains relocated addresses ready for SM configuration.
#[derive(Debug, Clone, Copy)]
pub struct InstalledProgram {
    /// Offset in instruction memory where the program starts
    pub offset: u8,
    /// Number of instructions
    pub length: u8,
    /// Relocated wrap bottom (wrap_target + offset)
    pub wrap_bottom: u8,
    /// Relocated wrap top (wrap + offset)
    pub wrap_top: u8,
    /// Number of side-set bits
    pub side_set_count: u8,
    /// Whether side-set is optional
    pub side_set_opt: bool,
    /// Whether side-set affects pin directions
    pub side_set_pindirs: bool,
}

// ============================================================================
// Source/Destination Constants for Instruction Encoding
// ============================================================================

/// MOV source register indices
pub mod mov_src {
    pub const PINS: u8 = 0;
    pub const X: u8 = 1;
    pub const Y: u8 = 2;
    pub const NULL: u8 = 3;
    pub const STATUS: u8 = 5;
    pub const ISR: u8 = 6;
    pub const OSR: u8 = 7;
}

/// MOV destination register indices
pub mod mov_dest {
    pub const PINS: u8 = 0;
    pub const X: u8 = 1;
    pub const Y: u8 = 2;
    /// RP2350: MOV to pin directions
    pub const PINDIRS: u8 = 3;
    pub const EXEC: u8 = 4;
    pub const PC: u8 = 5;
    pub const ISR: u8 = 6;
    pub const OSR: u8 = 7;
}

/// IN source register indices
pub mod in_src {
    pub const PINS: u8 = 0;
    pub const X: u8 = 1;
    pub const Y: u8 = 2;
    pub const NULL: u8 = 3;
    pub const ISR: u8 = 6;
    pub const OSR: u8 = 7;
}

/// OUT destination register indices
pub mod out_dest {
    pub const PINS: u8 = 0;
    pub const X: u8 = 1;
    pub const Y: u8 = 2;
    pub const NULL: u8 = 3;
    pub const PINDIRS: u8 = 4;
    pub const PC: u8 = 5;
    pub const ISR: u8 = 6;
    pub const EXEC: u8 = 7;
}

/// SET destination register indices
pub mod set_dest {
    pub const PINS: u8 = 0;
    pub const X: u8 = 1;
    pub const Y: u8 = 2;
    pub const PINDIRS: u8 = 4;
}

// ============================================================================
// State Machine Implementation Macro
// ============================================================================

macro_rules! impl_state_machine {
    ($SmName:ident, $sm_index:literal, $pio_num:literal) => {
        paste! {
        impl $SmName<'_> {
            /// State machine index (0-3)
            pub const INDEX: u8 = $sm_index;

            // ================================================================
            // Initialization
            // ================================================================

            /// Initialize the state machine
            ///
            /// Disables the SM, clears FIFOs, resets internal state, and
            /// jumps to `initial_pc`. The SM is left **disabled**.
            pub fn init(&mut self, initial_pc: u8) -> &mut Self {
                self.disable();
                self.clear_fifos();
                self.restart();
                self.restart_clock_divider();
                self.exec(encode_jmp(initial_pc));
                self
            }

            // ================================================================
            // Control
            // ================================================================

            /// Enable the state machine
            #[inline]
            pub fn enable(&mut self) -> &mut Self {
                let mask = 1u32 << $sm_index;
                self.pio.ctrl.set_bits(mask);
                self
            }

            /// Disable the state machine
            #[inline]
            pub fn disable(&mut self) -> &mut Self {
                let mask = 1u32 << $sm_index;
                self.pio.ctrl.clr_bits(mask);
                self
            }

            /// Check if the state machine is enabled
            #[inline]
            pub fn is_enabled(&self) -> bool {
                (self.pio.ctrl.sm_enable() & (1u32 << $sm_index)) != 0
            }

            /// Restart the state machine (clears ISR, shift counters, etc.)
            #[inline]
            pub fn restart(&mut self) -> &mut Self {
                self.pio
                    .ctrl
                    .set_bits(1u32 << (CTRL_SM_RESTART_LSB + $sm_index));
                self
            }

            /// Restart the clock divider from phase 0
            #[inline]
            pub fn restart_clock_divider(&mut self) -> &mut Self {
                self.pio
                    .ctrl
                    .set_bits(1u32 << (CTRL_CLKDIV_RESTART_LSB + $sm_index));
                self
            }

            /// Get the current program counter
            #[inline]
            pub fn pc(&self) -> u8 {
                self.pio.[<sm $sm_index _addr>].read_bits() as u8 & 0x1F
            }

            // ================================================================
            // Clock Configuration (Builder)
            // ================================================================

            /// Set clock divider
            ///
            /// SM runs at `sysclk / (int_div + frac_div/256)`.
            /// - `int_div`: 1-65535 (0 means 65536)
            /// - `frac_div`: 0-255
            pub fn set_clkdiv(&mut self, int_div: u16, frac_div: u8) -> &mut Self {
                self.pio.[<sm $sm_index _clkdiv>].write_int(int_div as u32);
                self.pio.[<sm $sm_index _clkdiv>].write_frac(frac_div as u32);
                self
            }

            // ================================================================
            // Program Configuration (Builder)
            // ================================================================

            /// Apply an installed program's wrap and side-set configuration
            ///
            /// Sets wrap addresses and side-set parameters from `InstalledProgram`.
            /// Does not set pin-base — call `set_sideset_pins()` separately.
            pub fn apply_program(&mut self, prog: &InstalledProgram) -> &mut Self {
                self.set_wrap(prog.wrap_bottom, prog.wrap_top);
                if prog.side_set_count > 0 {
                    self.pio.[<sm $sm_index _pinctrl>].write_sideset_count(prog.side_set_count as u32);
                    if prog.side_set_opt {
                        self.pio.[<sm $sm_index _execctrl>].set_side_en();
                    } else {
                        self.pio.[<sm $sm_index _execctrl>].clr_side_en();
                    }
                    if prog.side_set_pindirs {
                        self.pio.[<sm $sm_index _execctrl>].set_side_pindir();
                    } else {
                        self.pio.[<sm $sm_index _execctrl>].clr_side_pindir();
                    }
                }
                self
            }

            // ================================================================
            // Wrap Configuration (Builder)
            // ================================================================

            /// Set program wrap addresses
            ///
            /// When PC reaches `wrap_top`, it wraps to `wrap_target` next cycle.
            pub fn set_wrap(&mut self, wrap_target: u8, wrap_top: u8) -> &mut Self {
                self.pio.[<sm $sm_index _execctrl>].write_wrap_bottom(wrap_target as u32);
                self.pio.[<sm $sm_index _execctrl>].write_wrap_top(wrap_top as u32);
                self
            }

            // ================================================================
            // Pin Configuration (Builder)
            // ================================================================

            /// Set OUT pin base and count
            pub fn set_out_pins(&mut self, base: u8, count: u8) -> &mut Self {
                self.pio.[<sm $sm_index _pinctrl>].write_out_base((base & 31) as u32);
                self.pio.[<sm $sm_index _pinctrl>].write_out_count(count as u32);
                self
            }

            /// Set SET pin base and count (count must be 0-5)
            pub fn set_set_pins(&mut self, base: u8, count: u8) -> &mut Self {
                self.pio.[<sm $sm_index _pinctrl>].write_set_base((base & 31) as u32);
                self.pio.[<sm $sm_index _pinctrl>].write_set_count(count as u32);
                self
            }

            /// Set IN pin base
            pub fn set_in_pins(&mut self, base: u8) -> &mut Self {
                self.pio.[<sm $sm_index _pinctrl>].write_in_base((base & 31) as u32);
                self
            }

            /// (RP2350) Set IN pin count
            ///
            /// Limits the number of pins read by IN instruction. 0 means 32.
            pub fn set_in_pin_count(&mut self, count: u8) -> &mut Self {
                self.pio.[<sm $sm_index _shiftctrl>].write_in_count((count & 31) as u32);
                self
            }

            /// Set sideset configuration
            ///
            /// - `base`: first sideset pin
            /// - `bit_count`: number of sideset bits (0-5)
            /// - `optional`: MSB acts as enable flag
            /// - `pindirs`: sideset affects pin directions
            pub fn set_sideset(
                &mut self,
                base: u8,
                bit_count: u8,
                optional: bool,
                pindirs: bool,
            ) -> &mut Self {
                self.pio.[<sm $sm_index _pinctrl>].write_sideset_base((base & 31) as u32);
                self.pio.[<sm $sm_index _pinctrl>].write_sideset_count(bit_count as u32);
                if optional {
                    self.pio.[<sm $sm_index _execctrl>].set_side_en();
                } else {
                    self.pio.[<sm $sm_index _execctrl>].clr_side_en();
                }
                if pindirs {
                    self.pio.[<sm $sm_index _execctrl>].set_side_pindir();
                } else {
                    self.pio.[<sm $sm_index _execctrl>].clr_side_pindir();
                }
                self
            }

            /// Set JMP PIN source
            pub fn set_jmp_pin(&mut self, pin: u8) -> &mut Self {
                self.pio.[<sm $sm_index _execctrl>].write_jmp_pin((pin & 31) as u32);
                self
            }

            /// Set consecutive pin directions using the PIO hardware
            ///
            /// Temporarily modifies SET configuration to execute `SET PINDIRS`
            /// instructions. Restores original configuration afterward.
            pub fn set_consecutive_pindirs(
                &mut self,
                pin_base: u8,
                pin_count: u8,
                is_out: bool,
            ) -> &mut Self {
                let pindir_val = if is_out { 0x1F } else { 0 };

                // Save current register values
                let pinctrl_saved = self.pio.[<sm $sm_index _pinctrl>].read_bits();
                let execctrl_saved = self.pio.[<sm $sm_index _execctrl>].read_bits();

                // Clear OUT_STICKY to avoid side effects
                self.pio.[<sm $sm_index _execctrl>].clr_out_sticky();

                let mut remaining = pin_count;
                let mut pin = pin_base;

                while remaining > 5 {
                    self.pio.[<sm $sm_index _pinctrl>].write_set_base((pin & 31) as u32);
                    self.pio.[<sm $sm_index _pinctrl>].write_set_count(5);
                    self.exec(encode_set_pindirs(pindir_val));
                    remaining -= 5;
                    pin = pin.wrapping_add(5) & 0x1F;
                }

                if remaining > 0 {
                    self.pio.[<sm $sm_index _pinctrl>].write_set_base((pin & 31) as u32);
                    self.pio.[<sm $sm_index _pinctrl>].write_set_count(remaining as u32);
                    self.exec(encode_set_pindirs(pindir_val));
                }

                // Restore
                self.pio.[<sm $sm_index _pinctrl>].write_bits(pinctrl_saved);
                self.pio.[<sm $sm_index _execctrl>].write_bits(execctrl_saved);
                self
            }

            // ================================================================
            // Shift Configuration (Builder)
            // ================================================================

            /// Configure IN shifting
            ///
            /// - `shift_right`: true = right, false = left
            /// - `autopush`: push ISR when threshold reached
            /// - `push_threshold`: bits before push (1-32; 32 encoded as 0)
            pub fn set_in_shift(
                &mut self,
                shift_right: bool,
                autopush: bool,
                push_threshold: u8,
            ) -> &mut Self {
                if shift_right {
                    self.pio.[<sm $sm_index _shiftctrl>].set_in_shiftdir();
                } else {
                    self.pio.[<sm $sm_index _shiftctrl>].clr_in_shiftdir();
                }
                if autopush {
                    self.pio.[<sm $sm_index _shiftctrl>].set_autopush();
                } else {
                    self.pio.[<sm $sm_index _shiftctrl>].clr_autopush();
                }
                self.pio.[<sm $sm_index _shiftctrl>]
                    .write_push_thresh((push_threshold & 0x1F) as u32);
                self
            }

            /// Configure OUT shifting
            ///
            /// - `shift_right`: true = right, false = left
            /// - `autopull`: pull OSR when threshold reached
            /// - `pull_threshold`: bits before pull (1-32; 32 encoded as 0)
            pub fn set_out_shift(
                &mut self,
                shift_right: bool,
                autopull: bool,
                pull_threshold: u8,
            ) -> &mut Self {
                if shift_right {
                    self.pio.[<sm $sm_index _shiftctrl>].set_out_shiftdir();
                } else {
                    self.pio.[<sm $sm_index _shiftctrl>].clr_out_shiftdir();
                }
                if autopull {
                    self.pio.[<sm $sm_index _shiftctrl>].set_autopull();
                } else {
                    self.pio.[<sm $sm_index _shiftctrl>].clr_autopull();
                }
                self.pio.[<sm $sm_index _shiftctrl>]
                    .write_pull_thresh((pull_threshold & 0x1F) as u32);
                self
            }

            // ================================================================
            // FIFO Configuration (Builder)
            // ================================================================

            /// Set FIFO join mode
            pub fn set_fifo_join(&mut self, join: FifoJoin) -> &mut Self {
                let join_val = join as u32;

                // Clear all join bits
                self.pio.[<sm $sm_index _shiftctrl>].clr_fjoin_tx();
                self.pio.[<sm $sm_index _shiftctrl>].clr_fjoin_rx();
                self.pio.[<sm $sm_index _shiftctrl>].clr_fjoin_rx_put();
                self.pio.[<sm $sm_index _shiftctrl>].clr_fjoin_rx_get();

                // Set appropriate bits based on encoding:
                // bits [1:0] → FJOIN_TX (bit 30), FJOIN_RX (bit 31)
                // bits [3:2] → FJOIN_RX_PUT (bit 15), FJOIN_RX_GET (bit 14)
                if join_val & 1 != 0 {
                    self.pio.[<sm $sm_index _shiftctrl>].set_fjoin_tx();
                }
                if join_val & 2 != 0 {
                    self.pio.[<sm $sm_index _shiftctrl>].set_fjoin_rx();
                }
                if join_val & 4 != 0 {
                    self.pio.[<sm $sm_index _shiftctrl>].set_fjoin_rx_put();
                }
                if join_val & 8 != 0 {
                    self.pio.[<sm $sm_index _shiftctrl>].set_fjoin_rx_get();
                }
                self
            }

            // ================================================================
            // Special Configuration (Builder)
            // ================================================================

            /// Set OUT special features
            ///
            /// - `sticky`: re-assert last OUT/SET pin values continuously
            /// - `has_enable_pin`: use an OUT data bit as output enable
            /// - `enable_bit_index`: which OUT data bit is the enable (0-31)
            pub fn set_out_special(
                &mut self,
                sticky: bool,
                has_enable_pin: bool,
                enable_bit_index: u8,
            ) -> &Self {
                if sticky {
                    self.pio.[<sm $sm_index _execctrl>].set_out_sticky();
                } else {
                    self.pio.[<sm $sm_index _execctrl>].clr_out_sticky();
                }
                if has_enable_pin {
                    self.pio.[<sm $sm_index _execctrl>].set_inline_out_en();
                } else {
                    self.pio.[<sm $sm_index _execctrl>].clr_inline_out_en();
                }
                self.pio.[<sm $sm_index _execctrl>].write_out_en_sel(enable_bit_index as u32);
                self
            }

            /// Set MOV STATUS configuration
            ///
            /// - `status_sel`: which condition to monitor
            /// - `status_n`: comparison level or IRQ index
            pub fn set_mov_status(&mut self, status_sel: MovStatusType, status_n: u8) -> &Self {
                self.pio.[<sm $sm_index _execctrl>].write_status_sel(status_sel as u32);
                self.pio.[<sm $sm_index _execctrl>].write_status_n(status_n as u32);
                self
            }

            // ================================================================
            // Instruction Execution
            // ================================================================

            /// Execute an instruction immediately
            #[inline]
            pub fn exec(&self, instr: u16) -> &Self {
                self.pio.[<sm $sm_index _instr>].write_bits(instr as u32);
                self
            }

            /// Execute an instruction and wait for completion
            pub fn exec_wait_blocking(&self, instr: u16) -> &Self {
                self.exec(instr);
                crate::spin::spin_wait("PIO instruction execute", || !self.is_exec_stalled());
                self
            }

            /// Check if executed instruction is stalled
            #[inline]
            pub fn is_exec_stalled(&self) -> bool {
                self.pio.[<sm $sm_index _execctrl>].exec_stalled()
            }

            // ================================================================
            // FIFO Operations
            // ================================================================

            /// Check if TX FIFO is full
            #[inline]
            pub fn tx_fifo_is_full(&self) -> bool {
                (self.pio.fstat.txfull() & (1 << $sm_index)) != 0
            }

            /// Check if TX FIFO is empty
            #[inline]
            pub fn tx_fifo_is_empty(&self) -> bool {
                (self.pio.fstat.txempty() & (1 << $sm_index)) != 0
            }

            /// Check if RX FIFO is full
            #[inline]
            pub fn rx_fifo_is_full(&self) -> bool {
                (self.pio.fstat.rxfull() & (1 << $sm_index)) != 0
            }

            /// Check if RX FIFO is empty
            #[inline]
            pub fn rx_fifo_is_empty(&self) -> bool {
                (self.pio.fstat.rxempty() & (1 << $sm_index)) != 0
            }

            /// Get TX FIFO level
            #[inline]
            pub fn tx_fifo_level(&self) -> u8 {
                self.pio.flevel.[<tx $sm_index>]() as u8
            }

            /// Get RX FIFO level
            #[inline]
            pub fn rx_fifo_level(&self) -> u8 {
                self.pio.flevel.[<rx $sm_index>]() as u8
            }

            /// Write to TX FIFO (blocking)
            pub fn tx_fifo_write(&self, data: u32) {
                crate::spin::spin_wait("PIO TX FIFO space", || !self.tx_fifo_is_full());
                self.pio.[<txf $sm_index>].write_bits(data);
            }

            /// Try to write to TX FIFO (non-blocking)
            ///
            /// Returns `true` if written, `false` if FIFO was full.
            pub fn tx_fifo_try_write(&self, data: u32) -> bool {
                if self.tx_fifo_is_full() {
                    return false;
                }
                self.pio.[<txf $sm_index>].write_bits(data);
                true
            }

            /// Write to TX FIFO without checking
            #[inline]
            pub fn tx_fifo_put(&self, data: u32) {
                self.pio.[<txf $sm_index>].write_bits(data);
            }

            /// Read from RX FIFO (blocking)
            ///
            /// # Panics
            ///
            /// Panics if no RX data arrives within
            /// [`crate::spin::SPIN_TIMEOUT`] iterations.
            pub fn rx_fifo_read(&self) -> u32 {
                crate::spin::spin_wait("PIO RX FIFO data", || !self.rx_fifo_is_empty());
                self.pio.[<rxf $sm_index>].read_bits()
            }

            /// Try to read from RX FIFO (non-blocking)
            pub fn rx_fifo_try_read(&self) -> Option<u32> {
                if self.rx_fifo_is_empty() {
                    return None;
                }
                Some(self.pio.[<rxf $sm_index>].read_bits())
            }

            /// Read from RX FIFO without checking
            #[inline]
            pub fn rx_fifo_get(&self) -> u32 {
                self.pio.[<rxf $sm_index>].read_bits()
            }

            /// Clear both TX and RX FIFOs
            ///
            /// XOR-toggles `FJOIN_RX` twice to flush both FIFOs.
            pub fn clear_fifos(&self) -> &Self {
                // Toggle FJOIN_RX bit: this clears both FIFOs
                // FJOIN_RX is bit 31 of SHIFTCTRL
                self.pio.[<sm $sm_index _shiftctrl>].xor_bits(1 << 31);
                self.pio.[<sm $sm_index _shiftctrl>].xor_bits(1 << 31);
                self
            }

            /// Drain the TX FIFO
            ///
            /// Consumes all TX FIFO entries. Disturbs OSR contents.
            ///
            /// # Panics
            ///
            /// Panics if the FIFO does not drain within
            /// [`crate::spin::SPIN_TIMEOUT`] iterations (e.g. a stuck state
            /// machine that never accepts a new word).
            pub fn drain_tx_fifo(&self) {
                let autopull = self.pio.[<sm $sm_index _shiftctrl>].autopull();
                let instr = if autopull {
                    encode_out(out_dest::NULL, 32)
                } else {
                    encode_pull(false, false)
                };
                crate::spin::spin_wait("PIO TX FIFO drain", || {
                    if self.tx_fifo_is_empty() {
                        return true;
                    }
                    self.exec(instr);
                    false
                });
            }

            // ================================================================
            // PUTGET FIFO Access (RP2350)
            // ================================================================

            /// (RP2350) Read directly from FIFO entry via PUTGET register
            ///
            /// Requires FJOIN\_RX\_GET to be set (see [`FifoJoin::PutGet2`]/[`FifoJoin::PutGet3`]).
            /// `index`: 0-3
            pub fn putget_read(&self, index: u8) -> u32 {
                assert!(index < 4, "PUTGET index must be 0-3");
                let base = self.pio as *const _ as usize;
                let offset = 0x128 + ($sm_index as usize * 0x10) + (index as usize * 4);
                unsafe { core::ptr::read_volatile((base + offset) as *const u32) }
            }

            /// (RP2350) Write directly to FIFO entry via PUTGET register
            ///
            /// Requires FJOIN\_RX\_PUT to be set (see [`FifoJoin::PutGet1`]/[`FifoJoin::PutGet3`]).
            /// `index`: 0-3
            pub fn putget_write(&self, index: u8, data: u32) {
                assert!(index < 4, "PUTGET index must be 0-3");
                let base = self.pio as *const _ as usize;
                let offset = 0x128 + ($sm_index as usize * 0x10) + (index as usize * 4);
                unsafe { core::ptr::write_volatile((base + offset) as *mut u32, data) }
            }

            // ================================================================
            // DMA Support
            // ================================================================

            /// DREQ selection for writing to TX FIFO via DMA.
            pub const TX_DREQ: crate::dma::TreqSel = crate::dma::TreqSel::[<Pio $pio_num Tx $sm_index>];

            /// DREQ selection for reading from RX FIFO via DMA.
            pub const RX_DREQ: crate::dma::TreqSel = crate::dma::TreqSel::[<Pio $pio_num Rx $sm_index>];

            /// Get the TX FIFO register address for DMA writes.
            ///
            /// DMA should write 32-bit words to this address to push data
            /// into the state machine's TX FIFO.
            #[inline]
            pub fn tx_fifo_address(&self) -> u32 {
                &self.pio.[<txf $sm_index>] as *const _ as u32
            }

            /// Get the RX FIFO register address for DMA reads.
            ///
            /// DMA should read 32-bit words from this address to pull data
            /// from the state machine's RX FIFO.
            #[inline]
            pub fn rx_fifo_address(&self) -> u32 {
                &self.pio.[<rxf $sm_index>] as *const _ as u32
            }
        }
        } // paste!
    };
}

// ============================================================================
// PIO Block Implementation Macro
// ============================================================================

macro_rules! impl_pio_block {
    ($PioName:ident, $pio_num:literal) => {
        paste! {
            // State machine types
            /// State machine 0 for $PioName
            pub struct [<$PioName Sm0>]<'a> {
                pio: &'a pico_rs_pac::[<pio $pio_num>]::RegisterBlock,
            }
            /// State machine 1 for $PioName
            pub struct [<$PioName Sm1>]<'a> {
                pio: &'a pico_rs_pac::[<pio $pio_num>]::RegisterBlock,
            }
            /// State machine 2 for $PioName
            pub struct [<$PioName Sm2>]<'a> {
                pio: &'a pico_rs_pac::[<pio $pio_num>]::RegisterBlock,
            }
            /// State machine 3 for $PioName
            pub struct [<$PioName Sm3>]<'a> {
                pio: &'a pico_rs_pac::[<pio $pio_num>]::RegisterBlock,
            }

            impl_state_machine!([<$PioName Sm0>], 0, $pio_num);
            impl_state_machine!([<$PioName Sm1>], 1, $pio_num);
            impl_state_machine!([<$PioName Sm2>], 2, $pio_num);
            impl_state_machine!([<$PioName Sm3>], 3, $pio_num);

            // PIO block
            pub struct $PioName {
                pio: pico_rs_pac::[<PIO $pio_num>],
                used_instruction_space: u32,
            }

            impl $PioName {
                /// PIO block index (0, 1, or 2)
                pub const ID: u8 = $pio_num;

                /// Initialize a PIO block, taking ownership of the PAC peripheral.
                ///
                /// Unresets the PIO peripheral. Call once per PIO block.
                ///
                /// # Example
                ///
                /// ```ignore
                /// let dp = Peripherals::take().unwrap();
                /// let mut pio0 = Pio0::init(dp.PIO0);
                /// ```
                pub fn init(pio: pico_rs_pac::[<PIO $pio_num>]) -> Self {
                    // Reset then unreset to guarantee clean register state
                    <pico_rs_pac::[<PIO $pio_num>] as Resettable>::init();
                    Self {
                        pio,
                        used_instruction_space: 0,
                    }
                }

                /// Obtain a read-only PIO handle without ownership.
                ///
                /// Returns [`Steal<Self>`], which dereferences to `&Self`
                /// only — preventing access to `&mut self` methods such as
                /// [`install_program`](Self::install_program) and
                /// [`clear_instruction_memory`](Self::clear_instruction_memory).
                ///
                /// Useful for ISR context where only FIFO reads / status
                /// queries are needed.
                ///
                /// # Safety
                ///
                /// The caller must ensure:
                /// 1. The PIO peripheral has already been initialised via [`init`](Self::init).
                /// 2. No data races: the returned handle must not be used
                ///    concurrently with the owning instance without proper
                ///    synchronisation (e.g. critical section or single-core ISR).
                ///
                /// # Example
                ///
                /// ```ignore
                #[doc = concat!("let pio0 = unsafe { ", stringify!($PioName), "::steal() };")]
                /// while let Some(val) = pio0.sm0().rx_fifo_try_read() {
                ///     // process val
                /// }
                /// ```
                #[inline]
                pub unsafe fn steal() -> Steal<Self> {
                    Steal(Self {
                        pio: unsafe { pico_rs_pac::[<PIO $pio_num>]::steal() },
                        used_instruction_space: 0,
                    })
                }

                // ============================================================
                // State Machine Access
                // ============================================================

                /// Get state machine 0
                #[inline]
                pub fn sm0(&self) -> [<$PioName Sm0>]<'_> {
                    [<$PioName Sm0>] { pio: &*self.pio }
                }

                /// Get state machine 1
                #[inline]
                pub fn sm1(&self) -> [<$PioName Sm1>]<'_> {
                    [<$PioName Sm1>] { pio: &*self.pio }
                }

                /// Get state machine 2
                #[inline]
                pub fn sm2(&self) -> [<$PioName Sm2>]<'_> {
                    [<$PioName Sm2>] { pio: &*self.pio }
                }

                /// Get state machine 3
                #[inline]
                pub fn sm3(&self) -> [<$PioName Sm3>]<'_> {
                    [<$PioName Sm3>] { pio: &*self.pio }
                }

                // ============================================================
                // GPIO Pin Ownership
                // ============================================================

                /// Claim a GPIO pin for this PIO block
                ///
                /// Takes ownership of the pin and configures it for this PIO
                /// instance (sets funcsel and pad configuration). The pin
                /// cannot be used by other peripherals after this call.
                ///
                /// ```ignore
                /// pio0.claim_pin(pins.pin::<25>());
                /// // pins.pin::<25>() is consumed — compile error if reused
                /// ```
                pub fn claim_pin<P: gpio::[<Pio $pio_num Pin>]>(&self, mut pin: P) {
                    pin.[<configure_pio $pio_num>]();
                    // pin is dropped — ownership transferred, ZST has no cost
                    let _ = pin;
                }

                // ============================================================
                // GPIO Base (RP2350)
                // ============================================================

                /// (RP2350) Set GPIO base for this PIO instance
                ///
                /// - `false` / clear: address pins 0-31
                /// - `true` / set: address pins 16-47 (RP2350B with 48 GPIOs)
                pub fn set_gpio_base(&mut self, high: bool) {
                    if high {
                        self.pio.gpiobase.set_gpiobase();
                    } else {
                        self.pio.gpiobase.clr_gpiobase();
                    }
                }

                /// (RP2350) Get current GPIO base
                pub fn gpio_base(&self) -> bool {
                    self.pio.gpiobase.gpiobase()
                }

                /// Set input synchroniser bypass for specified pins
                pub fn set_input_sync_bypass(&mut self, pin_mask: u32) {
                    self.pio.input_sync_bypass.write_input_sync_bypass(pin_mask);
                }

                // ============================================================
                // Program Management
                // ============================================================

                /// Install a program into instruction memory
                ///
                /// JMP instructions are automatically relocated.
                /// The returned `InstalledProgram` contains relocated wrap addresses
                /// and side-set configuration, ready for `apply_program()`.
                pub fn install_program(
                    &mut self,
                    program: &PioProgram,
                ) -> Result<InstalledProgram, InstallError> {
                    let instructions = program.instructions;
                    if instructions.is_empty() || instructions.len() > PIO_INSTRUCTION_COUNT {
                        return Err(InstallError::NoSpace);
                    }

                    let length = instructions.len() as u8;
                    let program_mask = (1u32 << length) - 1;

                    let offset = if let Some(orig) = program.origin {
                        if orig as usize + instructions.len() > PIO_INSTRUCTION_COUNT {
                            return Err(InstallError::InvalidOrigin);
                        }
                        if (self.used_instruction_space & (program_mask << orig)) != 0 {
                            return Err(InstallError::NoSpace);
                        }
                        orig
                    } else {
                        let mut found = None;
                        for i in (0..=(PIO_INSTRUCTION_COUNT - instructions.len())).rev() {
                            if (self.used_instruction_space & (program_mask << i)) == 0 {
                                found = Some(i as u8);
                                break;
                            }
                        }
                        found.ok_or(InstallError::NoSpace)?
                    };

                    for (i, &instr) in instructions.iter().enumerate() {
                        let relocated = if (instr >> 13) == 0b000 {
                            // JMP: relocate address in bits [4:0]
                            let addr = instr & 0x1F;
                            let new_addr = (addr + offset as u16) & 0x1F;
                            (instr & !0x1F) | new_addr
                        } else {
                            instr
                        };
                        self.write_instruction(offset + i as u8, relocated);
                    }

                    self.used_instruction_space |= program_mask << offset;

                    Ok(InstalledProgram {
                        offset,
                        length,
                        wrap_bottom: offset + program.wrap_target,
                        wrap_top: offset + program.wrap,
                        side_set_count: program.side_set_count,
                        side_set_opt: program.side_set_opt,
                        side_set_pindirs: program.side_set_pindirs,
                    })
                }

                /// Uninstall a program, freeing its instruction memory
                pub fn uninstall_program(&mut self, program: InstalledProgram) {
                    let program_mask = (1u32 << program.length) - 1;
                    self.used_instruction_space &= !(program_mask << program.offset);
                }

                /// Write a single instruction to instruction memory
                fn write_instruction(&self, addr: u8, instr: u16) {
                    assert!((addr as usize) < PIO_INSTRUCTION_COUNT);
                    let base = &self.pio.instr_mem0 as *const _ as usize;
                    let target = (base + (addr as usize * 4)) as *mut u32;
                    unsafe { core::ptr::write_volatile(target, instr as u32); }
                }

                /// Clear all instruction memory
                pub fn clear_instruction_memory(&mut self) {
                    for i in 0..PIO_INSTRUCTION_COUNT as u8 {
                        self.write_instruction(i, 0);
                    }
                    self.used_instruction_space = 0;
                }

                // ============================================================
                // Multi-SM Control
                // ============================================================

                /// Enable/disable multiple state machines simultaneously
                pub fn set_sm_mask_enabled(&mut self, mask: u32, enabled: bool) {
                    let mask = mask & 0xF;
                    if enabled {
                        self.pio.ctrl.set_bits(mask << CTRL_SM_ENABLE_LSB);
                    } else {
                        self.pio.ctrl.clr_bits(mask << CTRL_SM_ENABLE_LSB);
                    }
                }

                /// Restart multiple state machines simultaneously
                pub fn restart_sm_mask(&mut self, mask: u32) {
                    self.pio.ctrl.set_bits((mask & 0xF) << CTRL_SM_RESTART_LSB);
                }

                /// Restart clock dividers for multiple state machines
                pub fn clkdiv_restart_sm_mask(&mut self, mask: u32) {
                    self.pio.ctrl.set_bits((mask & 0xF) << CTRL_CLKDIV_RESTART_LSB);
                }

                /// Enable multiple state machines with synchronized clocks
                pub fn enable_sm_mask_in_sync(&mut self, mask: u32) {
                    let mask = mask & 0xF;
                    self.pio.ctrl.set_bits(
                        (mask << CTRL_CLKDIV_RESTART_LSB)
                        | (mask << CTRL_SM_ENABLE_LSB)
                    );
                }

                /// (RP2350) Enable/disable SMs across neighboring PIO blocks
                pub fn set_sm_multi_mask_enabled(
                    &self,
                    mask_prev: u32,
                    mask: u32,
                    mask_next: u32,
                    enabled: bool,
                ) {
                    self.pio.ctrl.modify_val(|v| {
                        let v = v
                            .with_sm_enable(if enabled { mask & 0xF } else { 0 })
                            .with_prev_pio_mask(mask_prev & 0xF)
                            .with_next_pio_mask(mask_next & 0xF);
                        if enabled {
                            v.with_nextprev_sm_enable(true)
                        } else {
                            v.with_nextprev_sm_disable(true)
                        }
                    });
                }

                /// (RP2350) Enable SMs across multiple PIOs with synchronized clocks
                pub fn enable_sm_multi_mask_in_sync(
                    &self,
                    mask_prev: u32,
                    mask: u32,
                    mask_next: u32,
                ) {
                    self.pio.ctrl.modify_val(|v| {
                        v.with_sm_enable(mask & 0xF)
                            .with_clkdiv_restart(mask & 0xF)
                            .with_prev_pio_mask(mask_prev & 0xF)
                            .with_next_pio_mask(mask_next & 0xF)
                            .with_nextprev_clkdiv_restart(true)
                            .with_nextprev_sm_enable(true)
                    });
                }

                // ============================================================
                // IRQ Management
                // ============================================================

                /// Get raw IRQ flags (8 bits)
                #[inline]
                pub fn irq_raw(&self) -> u8 {
                    self.pio.irq.irq() as u8
                }

                /// Check if a PIO interrupt flag is set (0-7)
                #[inline]
                pub fn interrupt_get(&self, irq_num: u8) -> bool {
                    (self.pio.irq.irq() & (1 << irq_num)) != 0
                }

                /// Clear PIO interrupt flags (bitmask, W1C)
                #[inline]
                pub fn interrupt_clear(&self, flags: u8) {
                    self.pio.irq.write_irq(flags as u32);
                }

                /// Force PIO interrupt flags (for testing)
                #[inline]
                pub fn force_irq(&self, flags: u8) {
                    self.pio.irq_force.write_irq_force(flags as u32);
                }

                /// Enable/disable an interrupt source on IRQ0
                pub fn set_irq0_source_enabled(&mut self, source: u32, enabled: bool) {
                    if enabled {
                        self.pio.irq0_inte.set_bits(1 << source);
                    } else {
                        self.pio.irq0_inte.clr_bits(1 << source);
                    }
                }

                /// Enable/disable an interrupt source on IRQ1
                pub fn set_irq1_source_enabled(&mut self, source: u32, enabled: bool) {
                    if enabled {
                        self.pio.irq1_inte.set_bits(1 << source);
                    } else {
                        self.pio.irq1_inte.clr_bits(1 << source);
                    }
                }

                // ============================================================
                // Debug
                // ============================================================

                /// Get current PIO-driven GPIO output values
                #[inline]
                pub fn dbg_padout(&self) -> u32 {
                    self.pio.dbg_padout.read_bits()
                }

                /// Get current PIO-driven GPIO output enables
                #[inline]
                pub fn dbg_padoe(&self) -> u32 {
                    self.pio.dbg_padoe.read_bits()
                }
            }
        }
    };
}

// ============================================================================
// Generate PIO0, PIO1, PIO2
// ============================================================================

impl_pio_block!(Pio0, 0);
impl_pio_block!(Pio1, 1);
impl_pio_block!(Pio2, 2);

// ============================================================================
// PIO Instruction Encoding
// ============================================================================

// --- Delay / Side-set ---

/// Encode delay cycles (OR with any instruction)
#[inline]
pub const fn encode_delay(cycles: u8) -> u16 {
    ((cycles as u16) & 0x1F) << 8
}

/// Encode sideset value (non-optional mode)
#[inline]
pub const fn encode_sideset(bit_count: u8, value: u8) -> u16 {
    (value as u16) << (13 - bit_count)
}

/// Encode sideset value (optional mode, sets enable bit)
#[inline]
pub const fn encode_sideset_opt(bit_count: u8, value: u8) -> u16 {
    0x1000 | ((value as u16) << (13 - bit_count))
}

// --- JMP (opcode 000) ---

/// Encode unconditional `JMP addr`
#[inline]
pub const fn encode_jmp(addr: u8) -> u16 {
    (addr as u16) & 0x1F
}

/// Encode `JMP cond, addr`
#[inline]
pub const fn encode_jmp_cond(cond: u8, addr: u8) -> u16 {
    ((cond as u16 & 7) << 5) | (addr as u16 & 0x1F)
}

/// Encode `JMP !X, addr`
#[inline]
pub const fn encode_jmp_not_x(addr: u8) -> u16 {
    encode_jmp_cond(1, addr)
}
/// Encode `JMP X--, addr`
#[inline]
pub const fn encode_jmp_x_dec(addr: u8) -> u16 {
    encode_jmp_cond(2, addr)
}
/// Encode `JMP !Y, addr`
#[inline]
pub const fn encode_jmp_not_y(addr: u8) -> u16 {
    encode_jmp_cond(3, addr)
}
/// Encode `JMP Y--, addr`
#[inline]
pub const fn encode_jmp_y_dec(addr: u8) -> u16 {
    encode_jmp_cond(4, addr)
}
/// Encode `JMP X!=Y, addr`
#[inline]
pub const fn encode_jmp_x_ne_y(addr: u8) -> u16 {
    encode_jmp_cond(5, addr)
}
/// Encode `JMP PIN, addr`
#[inline]
pub const fn encode_jmp_pin(addr: u8) -> u16 {
    encode_jmp_cond(6, addr)
}
/// Encode `JMP !OSRE, addr`
#[inline]
pub const fn encode_jmp_not_osre(addr: u8) -> u16 {
    encode_jmp_cond(7, addr)
}

// --- WAIT (opcode 001) ---

/// Encode `WAIT polarity GPIO pin`
#[inline]
pub const fn encode_wait_gpio(polarity: bool, pin: u8) -> u16 {
    let pol = if polarity { 4u16 } else { 0 };
    0x2000 | (pol << 5) | (pin as u16 & 0x1F)
}

/// Encode `WAIT polarity PIN pin`
#[inline]
pub const fn encode_wait_pin(polarity: bool, pin: u8) -> u16 {
    let pol = if polarity { 4u16 } else { 0 };
    0x2000 | ((pol | 1) << 5) | (pin as u16 & 0x1F)
}

/// Encode `WAIT polarity IRQ irq [rel]`
#[inline]
pub const fn encode_wait_irq(polarity: bool, relative: bool, irq: u8) -> u16 {
    let pol = if polarity { 4u16 } else { 0 };
    let rel = if relative { 0x10u16 } else { 0 };
    0x2000 | ((pol | 2) << 5) | rel | (irq as u16 & 0x07)
}

// --- IN (opcode 010) ---

/// Encode `IN src, bit_count` (bit_count: 1-32, 32 encoded as 0)
#[inline]
pub const fn encode_in(src: u8, bit_count: u8) -> u16 {
    0x4000 | ((src as u16 & 7) << 5) | (bit_count as u16 & 0x1F)
}

// --- OUT (opcode 011) ---

/// Encode `OUT dest, bit_count` (bit_count: 1-32, 32 encoded as 0)
#[inline]
pub const fn encode_out(dest: u8, bit_count: u8) -> u16 {
    0x6000 | ((dest as u16 & 7) << 5) | (bit_count as u16 & 0x1F)
}

// --- PUSH / PULL (opcode 100) ---

/// Encode `PUSH [iffull] [block]`
#[inline]
pub const fn encode_push(if_full: bool, block: bool) -> u16 {
    let iffull = if if_full { 1u16 << 6 } else { 0 };
    let blk = if block { 1u16 << 5 } else { 0 };
    0x8000 | iffull | blk
}

/// Encode `PULL [ifempty] [block]`
#[inline]
pub const fn encode_pull(if_empty: bool, block: bool) -> u16 {
    let ifempty = if if_empty { 1u16 << 6 } else { 0 };
    let blk = if block { 1u16 << 5 } else { 0 };
    0x8080 | ifempty | blk
}

// --- MOV (opcode 101) ---

/// Encode `MOV dest, src`
#[inline]
pub const fn encode_mov(dest: u8, src: u8) -> u16 {
    0xA000 | ((dest as u16 & 7) << 5) | (src as u16 & 7)
}

/// Encode `MOV dest, ~src` (bit-invert)
#[inline]
pub const fn encode_mov_not(dest: u8, src: u8) -> u16 {
    0xA000 | ((dest as u16 & 7) << 5) | (1 << 3) | (src as u16 & 7)
}

/// Encode `MOV dest, ::src` (bit-reverse)
#[inline]
pub const fn encode_mov_reverse(dest: u8, src: u8) -> u16 {
    0xA000 | ((dest as u16 & 7) << 5) | (2 << 3) | (src as u16 & 7)
}

/// Encode `NOP`
#[inline]
pub const fn encode_nop() -> u16 {
    encode_mov(mov_dest::Y, mov_src::Y)
}

// --- IRQ (opcode 110) ---

/// Encode `IRQ SET irq [rel]`
#[inline]
pub const fn encode_irq_set(relative: bool, irq: u8) -> u16 {
    let rel = if relative { 0x10u16 } else { 0 };
    0xC000 | rel | (irq as u16 & 0x07)
}

/// Encode `IRQ WAIT irq [rel]`
#[inline]
pub const fn encode_irq_wait(relative: bool, irq: u8) -> u16 {
    let rel = if relative { 0x10u16 } else { 0 };
    0xC000 | (1 << 5) | rel | (irq as u16 & 0x07)
}

/// Encode `IRQ CLEAR irq [rel]`
#[inline]
pub const fn encode_irq_clear(relative: bool, irq: u8) -> u16 {
    let rel = if relative { 0x10u16 } else { 0 };
    0xC000 | (2 << 5) | rel | (irq as u16 & 0x07)
}

// --- SET (opcode 111) ---

/// Encode `SET dest, value` (value: 0-31)
#[inline]
pub const fn encode_set(dest: u8, value: u8) -> u16 {
    0xE000 | ((dest as u16 & 7) << 5) | (value as u16 & 0x1F)
}

/// Encode `SET PINS, value`
#[inline]
pub const fn encode_set_pins(value: u8) -> u16 {
    encode_set(set_dest::PINS, value)
}
/// Encode `SET X, value`
#[inline]
pub const fn encode_set_x(value: u8) -> u16 {
    encode_set(set_dest::X, value)
}
/// Encode `SET Y, value`
#[inline]
pub const fn encode_set_y(value: u8) -> u16 {
    encode_set(set_dest::Y, value)
}
/// Encode `SET PINDIRS, value`
#[inline]
pub const fn encode_set_pindirs(value: u8) -> u16 {
    encode_set(set_dest::PINDIRS, value)
}
