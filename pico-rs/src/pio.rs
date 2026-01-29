//! Programmable I/O (PIO) driver for RP2350
//!
//! The PIO is a versatile hardware block that can emulate a variety of IO
//! standards and protocols. It provides 3 PIO blocks (PIO0, PIO1, PIO2),
//! each with 4 state machines and 32 instruction words of program memory.
//!
//! This implementation uses the `paste` crate to generate dedicated types
//! for each PIO block and state machine combination, avoiding type casts.
//!
//! ## Example
//!
//! ```ignore
//! use pico_rs::pio::{self, Pio0};
//!
//! // Take PIO0 (this also unresets it)
//! let mut pio = Pio0::take();
//!
//! // Install a program using the pio_asm! macro
//! let program = pio_asm!(
//!     r#"
//!     set pindirs, 1      ; Set pin as output
//!     .wrap_target
//!     set pins, 1 [31]    ; Set pin high
//!     set pins, 0 [31]    ; Set pin low
//!     .wrap
//!     "#
//! );
//!
//! let installed = pio.install_program(program.instructions, None).unwrap();
//!
//! // Configure and start state machine 0
//! let sm = pio.sm0();
//! sm.set_clock_divisor(256, 0);
//! sm.set_wrap(
//!     installed.offset + program.wrap_target as u8,
//!     installed.offset + program.wrap as u8,
//! );
//! sm.set_set_pins(LED_PIN, 1);
//! sm.set_consecutive_pindirs(LED_PIN, 1, true);
//! sm.restart();
//! sm.exec(pio::encode_jmp(installed.offset));
//! sm.set_enabled(true);
//! ```

#![allow(clippy::identity_op)]

use paste::paste;

// Re-export the pio_asm! macro
pub use pico_rs_macro::pio_asm;

use crate::reset::{self, Peripheral};

/// Maximum number of instructions in PIO program memory
pub const PIO_INSTRUCTION_COUNT: usize = 32;

/// Errors that can occur when installing a PIO program
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum InstallError {
    /// Program is too large for instruction memory
    NoSpace,
    /// Invalid program origin
    InvalidOrigin,
}

/// A handle to an installed PIO program
#[derive(Debug, Clone, Copy)]
pub struct InstalledProgram {
    /// Offset in instruction memory where the program is installed
    pub offset: u8,
    /// Length of the program in instructions
    pub length: u8,
}

// ============================================================================
// Macro for generating State Machine implementations
// ============================================================================

macro_rules! impl_state_machine {
    ($SmName:ident, $pio_mod:ident, $sm_index:expr,
     $clkdiv:ident, $execctrl:ident, $shiftctrl:ident, $addr:ident, $instr:ident, $pinctrl:ident,
     $txf:ident, $rxf:ident) => {
        impl $SmName<'_> {
            /// State machine index (0-3)
            pub const INDEX: u8 = $sm_index;

            // ================================================================
            // Control Methods
            // ================================================================

            /// Enable or disable the state machine
            #[inline]
            pub fn set_enabled(&self, enabled: bool) {
                let mask = 1u32 << $sm_index;
                if enabled {
                    self.pio.ctrl.set_bits(mask);
                } else {
                    self.pio.ctrl.clr_bits(mask);
                }
            }

            /// Restart the state machine
            #[inline]
            pub fn restart(&self) {
                // SM_RESTART is bits [7:4]
                self.pio.ctrl.set_bits(1u32 << ($sm_index + 4));
            }

            /// Restart the state machine's clock divider
            #[inline]
            pub fn restart_clock_divider(&self) {
                // CLKDIV_RESTART is bits [11:8]
                self.pio.ctrl.set_bits(1u32 << ($sm_index + 8));
            }

            /// Get the current program counter (instruction address)
            #[inline]
            pub fn get_pc(&self) -> u8 {
                (self.pio.$addr.read_bits() & 0x1F) as u8
            }

            // ================================================================
            // Clock Configuration
            // ================================================================

            /// Set the clock divisor for the state machine
            ///
            /// # Arguments
            /// * `int` - Integer part of divisor (16 bits)
            /// * `frac` - Fractional part of divisor (8 bits, in units of 1/256)
            #[inline]
            pub fn set_clock_divisor(&self, int: u16, frac: u8) {
                self.pio.$clkdiv.write_int(int as u32);
                self.pio.$clkdiv.write_frac(frac as u32);
            }

            // ================================================================
            // Wrap Configuration
            // ================================================================

            /// Set wrap addresses for the program
            ///
            /// # Arguments
            /// * `wrap_target` - Address to wrap to (bottom, entry point)
            /// * `wrap` - Address to wrap from (top, last instruction)
            #[inline]
            pub fn set_wrap(&self, wrap_target: u8, wrap: u8) {
                self.pio.$execctrl.write_wrap_bottom(wrap_target as u32);
                self.pio.$execctrl.write_wrap_top(wrap as u32);
            }

            // ================================================================
            // Pin Configuration
            // ================================================================

            /// Configure SET pins
            ///
            /// # Arguments
            /// * `base` - The lowest-numbered pin that will be affected by SET instructions
            /// * `count` - The number of pins (1-5)
            #[inline]
            pub fn set_set_pins(&self, base: u8, count: u8) {
                assert!(count > 0 && count <= 5, "SET pin count must be 1-5");
                self.pio.$pinctrl.write_set_base(base as u32);
                self.pio.$pinctrl.write_set_count(count as u32);
            }

            /// Configure OUT pins
            ///
            /// # Arguments
            /// * `base` - The lowest-numbered pin for OUT instructions
            /// * `count` - The number of pins (0-32)
            #[inline]
            pub fn set_out_pins(&self, base: u8, count: u8) {
                self.pio.$pinctrl.write_out_base(base as u32);
                self.pio.$pinctrl.write_out_count(count as u32);
            }

            /// Configure IN pins
            ///
            /// # Arguments
            /// * `base` - The pin to use as the first IN pin
            #[inline]
            pub fn set_in_pins(&self, base: u8) {
                self.pio.$pinctrl.write_in_base(base as u32);
            }

            /// Configure side-set pins
            ///
            /// # Arguments
            /// * `base` - The lowest-numbered pin for side-set
            /// * `count` - The number of bits used for side-set (0-5)
            /// * `optional` - If true, MSB of delay/sideset enables side-set
            /// * `pindirs` - If true, side-set affects pin directions instead of values
            #[inline]
            pub fn set_sideset_pins(&self, base: u8, count: u8, optional: bool, pindirs: bool) {
                assert!(count <= 5, "Side-set count must be 0-5");
                self.pio.$pinctrl.write_sideset_base(base as u32);
                self.pio.$pinctrl.write_sideset_count(count as u32);
                if optional {
                    self.pio.$execctrl.set_side_en();
                } else {
                    self.pio.$execctrl.clr_side_en();
                }
                if pindirs {
                    self.pio.$execctrl.set_side_pindir();
                } else {
                    self.pio.$execctrl.clr_side_pindir();
                }
            }

            /// Set consecutive pin directions using the state machine
            ///
            /// This is equivalent to pico-sdk's pio_sm_set_consecutive_pindirs().
            /// It temporarily modifies SET_BASE and SET_COUNT to execute SET PINDIRS
            /// instructions that configure the pin directions.
            ///
            /// # Arguments
            /// * `pin_base` - The first pin to set
            /// * `pin_count` - Number of consecutive pins
            /// * `is_out` - true for output, false for input
            pub fn set_consecutive_pindirs(&self, pin_base: u8, pin_count: u8, is_out: bool) {
                let pindir_val: u8 = if is_out { 0x1f } else { 0 };

                // Save current state
                let pinctrl_saved = self.pio.$pinctrl.read_bits();
                let execctrl_saved = self.pio.$execctrl.read_bits();

                // Clear OUT_STICKY bit in EXECCTRL
                self.pio.$execctrl.clr_out_sticky();

                // Process pins in chunks of up to 5 (SET instruction limit)
                let mut remaining = pin_count;
                let mut pin = pin_base;

                while remaining > 5 {
                    self.pio.$pinctrl.write_set_base(pin as u32);
                    self.pio.$pinctrl.write_set_count(5);
                    self.exec(encode_set_pindirs(pindir_val));
                    remaining -= 5;
                    pin = (pin + 5) & 0x1f;
                }

                // Handle remaining pins (1-5)
                if remaining > 0 {
                    self.pio.$pinctrl.write_set_base(pin as u32);
                    self.pio.$pinctrl.write_set_count(remaining as u32);
                    self.exec(encode_set_pindirs(pindir_val));
                }

                // Restore saved state
                self.pio.$pinctrl.write_bits(pinctrl_saved);
                self.pio.$execctrl.write_bits(execctrl_saved);
            }

            // ================================================================
            // Instruction Execution
            // ================================================================

            /// Execute an instruction immediately
            #[inline]
            pub fn exec(&self, instr: u16) {
                self.pio.$instr.write_bits(instr as u32);
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

            /// Write to TX FIFO (blocking)
            pub fn tx_fifo_write(&self, data: u32) {
                while self.tx_fifo_is_full() {
                    core::hint::spin_loop();
                }
                self.pio.$txf.write_bits(data);
            }

            /// Try to write to TX FIFO (non-blocking)
            pub fn tx_fifo_try_write(&self, data: u32) -> bool {
                if self.tx_fifo_is_full() {
                    return false;
                }
                self.pio.$txf.write_bits(data);
                true
            }

            /// Read from RX FIFO (blocking)
            pub fn rx_fifo_read(&self) -> u32 {
                while self.rx_fifo_is_empty() {
                    core::hint::spin_loop();
                }
                self.pio.$rxf.read_bits()
            }

            /// Try to read from RX FIFO (non-blocking)
            pub fn rx_fifo_try_read(&self) -> Option<u32> {
                if self.rx_fifo_is_empty() {
                    return None;
                }
                Some(self.pio.$rxf.read_bits())
            }

            /// Clear both TX and RX FIFOs
            pub fn clear_fifos(&self) {
                let shiftctrl = self.pio.$shiftctrl.read_bits();
                // Set FJOIN_RX and FJOIN_TX temporarily to flush
                let flush = shiftctrl | (1 << 31) | (1 << 30);
                self.pio.$shiftctrl.write_bits(flush);
                self.pio.$shiftctrl.write_bits(flush);
                // Restore original
                self.pio.$shiftctrl.write_bits(shiftctrl);
            }
        }
    };
}

// ============================================================================
// Macro for generating PIO block and its state machines
// ============================================================================

macro_rules! impl_pio_block {
    ($pio_num:literal, $PioName:ident, $PioType:ty, $pio_mod:ident, $reset_peripheral:expr) => {
        paste! {
            // ================================================================
            // State Machine types for this PIO block
            // ================================================================

            /// State machine 0 for $PioName
            pub struct [<$PioName Sm0>]<'a> {
                pio: &'a pico_rs_pac::$pio_mod::RegisterBlock,
            }

            /// State machine 1 for $PioName
            pub struct [<$PioName Sm1>]<'a> {
                pio: &'a pico_rs_pac::$pio_mod::RegisterBlock,
            }

            /// State machine 2 for $PioName
            pub struct [<$PioName Sm2>]<'a> {
                pio: &'a pico_rs_pac::$pio_mod::RegisterBlock,
            }

            /// State machine 3 for $PioName
            pub struct [<$PioName Sm3>]<'a> {
                pio: &'a pico_rs_pac::$pio_mod::RegisterBlock,
            }

            // Implement state machine methods for each SM
            impl_state_machine!([<$PioName Sm0>], $pio_mod, 0,
                sm0_clkdiv, sm0_execctrl, sm0_shiftctrl, sm0_addr, sm0_instr, sm0_pinctrl,
                txf0, rxf0);
            impl_state_machine!([<$PioName Sm1>], $pio_mod, 1,
                sm1_clkdiv, sm1_execctrl, sm1_shiftctrl, sm1_addr, sm1_instr, sm1_pinctrl,
                txf1, rxf1);
            impl_state_machine!([<$PioName Sm2>], $pio_mod, 2,
                sm2_clkdiv, sm2_execctrl, sm2_shiftctrl, sm2_addr, sm2_instr, sm2_pinctrl,
                txf2, rxf2);
            impl_state_machine!([<$PioName Sm3>], $pio_mod, 3,
                sm3_clkdiv, sm3_execctrl, sm3_shiftctrl, sm3_addr, sm3_instr, sm3_pinctrl,
                txf3, rxf3);

            // ================================================================
            // PIO block struct
            // ================================================================

            /// PIO block $pio_num wrapper
            pub struct $PioName {
                pio: $PioType,
                used_instruction_space: u32,
            }

            impl $PioName {
                /// PIO block ID
                pub const ID: u8 = $pio_num;

                pub fn id() -> u8 {
                    $pio_num
                }

                /// Take ownership of this PIO block (unresets it automatically)
                pub fn take() -> Self {
                    reset::unreset_block_wait($reset_peripheral);
                    Self {
                        pio: unsafe { <$PioType>::steal() },
                        used_instruction_space: 0,
                    }
                }

                /// Get access to state machine 0
                #[inline]
                pub fn sm0(&self) -> [<$PioName Sm0>]<'_> {
                    [<$PioName Sm0>] { pio: &self.pio }
                }

                /// Get access to state machine 1
                #[inline]
                pub fn sm1(&self) -> [<$PioName Sm1>]<'_> {
                    [<$PioName Sm1>] { pio: &self.pio }
                }

                /// Get access to state machine 2
                #[inline]
                pub fn sm2(&self) -> [<$PioName Sm2>]<'_> {
                    [<$PioName Sm2>] { pio: &self.pio }
                }

                /// Get access to state machine 3
                #[inline]
                pub fn sm3(&self) -> [<$PioName Sm3>]<'_> {
                    [<$PioName Sm3>] { pio: &self.pio }
                }

                /// Install a program into instruction memory
                ///
                /// # Arguments
                /// * `program` - Array of 16-bit instruction words
                /// * `origin` - Optional fixed origin. If None, finds first available space
                ///
                /// # Returns
                /// Ok(InstalledProgram) with offset and length, or Err(InstallError)
                pub fn install_program(
                    &mut self,
                    program: &[u16],
                    origin: Option<u8>,
                ) -> Result<InstalledProgram, InstallError> {
                    if program.len() > PIO_INSTRUCTION_COUNT {
                        return Err(InstallError::NoSpace);
                    }

                    let length = program.len() as u8;
                    let program_mask = (1u32 << length) - 1;

                    // Find offset for program
                    let offset = if let Some(orig) = origin {
                        if orig as usize + program.len() > PIO_INSTRUCTION_COUNT {
                            return Err(InstallError::InvalidOrigin);
                        }
                        if (self.used_instruction_space & (program_mask << orig)) != 0 {
                            return Err(InstallError::NoSpace);
                        }
                        orig
                    } else {
                        // Find first available space, working down from top
                        let mut found = None;
                        for i in (0..=(PIO_INSTRUCTION_COUNT - program.len())).rev() {
                            if (self.used_instruction_space & (program_mask << i)) == 0 {
                                found = Some(i as u8);
                                break;
                            }
                        }
                        found.ok_or(InstallError::NoSpace)?
                    };

                    // Disable ALL state machines before writing INSTR_MEM
                    let ctrl_saved = self.pio.ctrl.read_bits();
                    self.pio.ctrl.clr_bits(0xF); // Clear SM_ENABLE bits

                    for (i, &instr) in program.iter().enumerate() {
                        // Relocate JMP instructions
                        let relocated_instr = if (instr >> 13) == 0b000 {
                            let jmp_addr = (instr & 0x1F) as u8;
                            let relocated_addr = (offset + jmp_addr) & 0x1F;
                            (instr & !0x1F) | (relocated_addr as u16)
                        } else {
                            instr
                        };
                        self.write_instruction(offset + i as u8, relocated_instr);
                    }

                    // Restore CTRL register
                    self.pio.ctrl.write_bits(ctrl_saved);

                    // Mark space as used
                    self.used_instruction_space |= program_mask << offset;

                    Ok(InstalledProgram { offset, length })
                }

                /// Uninstall a program, freeing instruction memory
                pub fn uninstall_program(&mut self, program: InstalledProgram) {
                    let program_mask = (1u32 << program.length) - 1;
                    self.used_instruction_space &= !(program_mask << program.offset);
                }

                /// Write a single instruction to instruction memory
                fn write_instruction(&self, addr: u8, instr: u16) {
                    assert!(addr < PIO_INSTRUCTION_COUNT as u8);
                    // INSTR_MEM is at offset 0x48 from base, each register is 4 bytes
                    let instr_mem_base = &self.pio.instr_mem0 as *const _ as usize;
                    let instr_addr = (instr_mem_base + (addr as usize * 4)) as *mut u32;
                    unsafe {
                        core::ptr::write_volatile(instr_addr, instr as u32);
                    }
                }

                /// Clear all instruction memory
                pub fn clear_instruction_memory(&mut self) {
                    for i in 0..PIO_INSTRUCTION_COUNT as u8 {
                        self.write_instruction(i, encode_jmp(i));
                    }
                    self.used_instruction_space = 0;
                }

                /// Get raw IRQ flags (8 bits)
                #[inline]
                pub fn get_irq_raw(&self) -> u8 {
                    self.pio.irq.irq() as u8
                }

                /// Clear IRQ flags
                #[inline]
                pub fn clear_irq(&self, flags: u8) {
                    self.pio.irq.write_irq(flags as u32);
                }

                /// Force IRQ flags
                #[inline]
                pub fn force_irq(&self, flags: u8) {
                    self.pio.irq_force.write_irq_force(flags as u32);
                }
            }
        }
    };
}

// ============================================================================
// Generate PIO0, PIO1, PIO2 with their state machines
// ============================================================================

impl_pio_block!(0, Pio0, pico_rs_pac::PIO0, pio0, Peripheral::PIO0);
impl_pio_block!(1, Pio1, pico_rs_pac::PIO1, pio1, Peripheral::PIO1);
impl_pio_block!(2, Pio2, pico_rs_pac::PIO2, pio2, Peripheral::PIO2);

// ============================================================================
// PIO Instruction Encoding Helpers
// ============================================================================

/// Encode a JMP instruction
#[inline]
pub const fn encode_jmp(addr: u8) -> u16 {
    0x0000 | (addr as u16 & 0x1F)
}

/// Encode a JMP instruction with condition
#[inline]
pub const fn encode_jmp_cond(cond: u8, addr: u8) -> u16 {
    0x0000 | ((cond as u16 & 0x7) << 5) | (addr as u16 & 0x1F)
}

/// Encode a WAIT instruction (wait for GPIO pin)
#[inline]
pub const fn encode_wait_gpio(pin: u8, polarity: bool) -> u16 {
    let pol = if polarity { 1 } else { 0 };
    0x2000 | (pol << 7) | (1 << 5) | (pin as u16 & 0x1F)
}

/// Encode a WAIT instruction (wait for pin mapped via IN)
#[inline]
pub const fn encode_wait_pin(pin: u8, polarity: bool) -> u16 {
    let pol = if polarity { 1 } else { 0 };
    0x2000 | (pol << 7) | (pin as u16 & 0x1F)
}

/// Encode a SET pins instruction
#[inline]
pub const fn encode_set_pins(value: u8) -> u16 {
    0xE000 | (value as u16 & 0x1F)
}

/// Encode a SET pindirs instruction
#[inline]
pub const fn encode_set_pindirs(value: u8) -> u16 {
    0xE080 | (value as u16 & 0x1F)
}

/// Encode an OUT pins instruction
#[inline]
pub const fn encode_out_pins(bit_count: u8) -> u16 {
    0x6000 | (bit_count as u16 & 0x1F)
}

/// Encode an IN pins instruction
#[inline]
pub const fn encode_in_pins(bit_count: u8) -> u16 {
    0x4000 | (bit_count as u16 & 0x1F)
}

/// Encode a PUSH instruction
#[inline]
pub const fn encode_push(if_full: bool, block: bool) -> u16 {
    let iffull = if if_full { 1 } else { 0 };
    let blk = if block { 1 } else { 0 };
    0x8000 | (iffull << 6) | (blk << 5)
}

/// Encode a PULL instruction
#[inline]
pub const fn encode_pull(if_empty: bool, block: bool) -> u16 {
    let ifempty = if if_empty { 1 } else { 0 };
    let blk = if block { 1 } else { 0 };
    0x8080 | (ifempty << 6) | (blk << 5)
}

/// Encode a NOP instruction (MOV y, y)
#[inline]
pub const fn encode_nop() -> u16 {
    0xA042
}
