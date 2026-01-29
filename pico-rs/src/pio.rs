//! Programmable I/O (PIO) driver for RP2350
//!
//! The PIO is a versatile hardware block that can emulate a variety of IO
//! standards and protocols. It provides 3 PIO blocks (PIO0, PIO1, PIO2),
//! each with 4 state machines and 32 instruction words of program memory.
//!
//! ## Example
//!
//! ```ignore
//! use pico_rs::pio::{self, PioBlock};
//! use pico_rs::reset::{self, Peripheral};
//!
//! // Reset and initialize PIO0
//! reset::unreset_block_wait(Peripheral::PIO0);
//! let mut pio = pio::PIO0::take();
//!
//! // Install a program using the pio_asm! macro
//! let program = pio_asm!(
//!     "set pindirs, 1",
//!     ".wrap_target",
//!     "set pins, 1 [31]",
//!     "set pins, 0 [31]",
//!     ".wrap"
//! );
//! let offset = pio.install_program(program.instructions, None).unwrap();
//!
//! // Configure and start state machine 0
//! let sm = pio.state_machine(0);
//! sm.set_clock_divisor(1, 0);
//! sm.set_wrap(program.wrap_target as u8, program.wrap as u8);
//! sm.restart();
//! sm.set_enabled(true);
//! ```

// Re-export the pio_asm! macro
pub use pico_rs_macro::pio_asm;

use crate::reset::{self, Peripheral};
use pico_rs_pac::{PIO0, PIO1, PIO2};

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

/// Trait for PIO blocks (PIO0, PIO1, PIO2)
pub trait PioBlock {
    /// PIO block instance (peripheral pointer)
    type Pio;

    /// Get the PIO block number (0, 1, or 2)
    fn id() -> u8;

    /// Get the reset peripheral for this PIO
    fn reset_peripheral() -> Peripheral;

    /// Take ownership of the PIO block
    fn take() -> Pio<Self>
    where
        Self: Sized;

    /// Get a reference to the register block
    fn get_block(pio: &Self::Pio) -> &pico_rs_pac::pio0::RegisterBlock;
}

/// PIO0 block
pub struct Pio0;

impl PioBlock for Pio0 {
    type Pio = PIO0;

    fn id() -> u8 {
        0
    }

    fn reset_peripheral() -> Peripheral {
        Peripheral::PIO0
    }

    fn take() -> Pio<Self> {
        reset::unreset_block_wait(Self::reset_peripheral());
        Pio {
            pio: unsafe { PIO0::steal() },
            used_instruction_space: 0,
            _phantom: core::marker::PhantomData,
        }
    }

    fn get_block(pio: &Self::Pio) -> &pico_rs_pac::pio0::RegisterBlock {
        &*pio
    }
}

/// PIO1 block
pub struct Pio1;

impl PioBlock for Pio1 {
    type Pio = PIO1;

    fn id() -> u8 {
        1
    }

    fn reset_peripheral() -> Peripheral {
        Peripheral::PIO1
    }

    fn take() -> Pio<Self> {
        reset::unreset_block_wait(Self::reset_peripheral());
        Pio {
            pio: unsafe { PIO1::steal() },
            used_instruction_space: 0,
            _phantom: core::marker::PhantomData,
        }
    }

    fn get_block(pio: &Self::Pio) -> &pico_rs_pac::pio0::RegisterBlock {
        // SAFETY: PIO1 has the same register layout as PIO0
        unsafe { &*((&**pio) as *const _ as *const pico_rs_pac::pio0::RegisterBlock) }
    }
}

/// PIO2 block
pub struct Pio2;

impl PioBlock for Pio2 {
    type Pio = PIO2;

    fn id() -> u8 {
        2
    }

    fn reset_peripheral() -> Peripheral {
        Peripheral::PIO2
    }

    fn take() -> Pio<Self> {
        reset::unreset_block_wait(Self::reset_peripheral());
        Pio {
            pio: unsafe { PIO2::steal() },
            used_instruction_space: 0,
            _phantom: core::marker::PhantomData,
        }
    }

    fn get_block(pio: &Self::Pio) -> &pico_rs_pac::pio0::RegisterBlock {
        // SAFETY: PIO2 has the same register layout as PIO0
        unsafe { &*((&**pio) as *const _ as *const pico_rs_pac::pio0::RegisterBlock) }
    }
}

/// PIO peripheral wrapper
pub struct Pio<P: PioBlock> {
    pio: P::Pio,
    used_instruction_space: u32, // Bitmap of used instruction slots
    _phantom: core::marker::PhantomData<P>,
}

impl<P: PioBlock> Pio<P> {
    /// Get access to a specific state machine (0-3)
    #[inline]
    pub fn state_machine(&mut self, sm: u8) -> StateMachine<'_, P> {
        assert!(sm < 4, "State machine index must be 0-3");
        let block = P::get_block(&self.pio);
        StateMachine {
            pio: block,
            sm,
            _phantom: core::marker::PhantomData,
        }
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

        // Install instructions
        unsafe {
            for (i, &instr) in program.iter().enumerate() {
                self.write_instruction(offset + i as u8, instr);
            }
        }

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
    ///
    /// # Safety
    /// This bypasses the allocation tracking. Only use if you know what you're doing.
    pub unsafe fn write_instruction(&self, addr: u8, instr: u16) {
        assert!(addr < PIO_INSTRUCTION_COUNT as u8);

        let block = P::get_block(&self.pio);
        // Access instruction memory through the register block
        // The PAC should have instr_mem0..instr_mem31 registers
        unsafe {
            let instr_mem_base = &block.instr_mem0 as *const _ as *const u32;
            let instr_addr = instr_mem_base.add(addr as usize);
            core::ptr::write_volatile(instr_addr as *mut u32, instr as u32);
        }
    }

    /// Clear all instruction memory
    pub fn clear_instruction_memory(&mut self) {
        unsafe {
            for i in 0..PIO_INSTRUCTION_COUNT as u8 {
                // Write JMP to same address (safe no-op)
                self.write_instruction(i, encode_jmp(i));
            }
        }
        self.used_instruction_space = 0;
    }

    /// Get raw IRQ flags (8 bits, 4 visible to host)
    #[inline]
    pub fn get_irq_raw(&self) -> u8 {
        let block = P::get_block(&self.pio);
        (block.irq.read_bits() & 0xFF) as u8
    }

    /// Clear IRQ flags
    #[inline]
    pub fn clear_irq(&self, flags: u8) {
        let block = P::get_block(&self.pio);
        block.irq.write_bits(flags as u32);
    }

    /// Force IRQ flags
    #[inline]
    pub fn force_irq(&self, flags: u8) {
        let block = P::get_block(&self.pio);
        block.irq_force.write_bits(flags as u32);
    }
}

/// State machine wrapper (0-3)
pub struct StateMachine<'a, P: PioBlock> {
    pio: &'a pico_rs_pac::pio0::RegisterBlock,
    sm: u8,
    _phantom: core::marker::PhantomData<P>,
}

impl<'a, P: PioBlock> StateMachine<'a, P> {
    /// Enable or disable the state machine
    #[inline]
    pub fn set_enabled(&self, enabled: bool) {
        let mask = 1u32 << self.sm;
        if enabled {
            self.pio.ctrl.set_bits(mask);
        } else {
            self.pio.ctrl.clr_bits(mask);
        }
    }

    /// Restart the state machine
    #[inline]
    pub fn restart(&self) {
        let mask = 1u32 << (self.sm + 4); // SM_RESTART is bits 7:4
        self.pio.ctrl.write_bits(mask);
    }

    /// Restart the state machine's clock divider
    #[inline]
    pub fn restart_clock_divider(&self) {
        let mask = 1u32 << (self.sm + 8); // CLKDIV_RESTART is bits 11:8
        self.pio.ctrl.write_bits(mask);
    }

    /// Set the clock divisor for the state machine
    ///
    /// # Arguments
    /// * `int` - Integer part of divisor (16 bits)
    /// * `frac` - Fractional part of divisor (8 bits, in units of 1/256)
    pub fn set_clock_divisor(&self, int: u16, frac: u8) {
        let divisor = ((int as u32) << 16) | ((frac as u32) << 8);
        unsafe {
            let sm_base = &self.pio.sm0_clkdiv as *const _ as *const u32;
            let sm_clkdiv = sm_base.add(self.sm as usize * 12); // Each SM is 12 words apart
            core::ptr::write_volatile(sm_clkdiv as *mut u32, divisor);
        }
    }

    /// Set wrap addresses for the program
    ///
    /// # Arguments
    /// * `wrap_target` - Address to wrap to (entry point)
    /// * `wrap` - Address to wrap from (last instruction)
    pub fn set_wrap(&self, wrap_target: u8, wrap: u8) {
        unsafe {
            let sm_base = &self.pio.sm0_clkdiv as *const _ as *const u32;
            let sm_execctrl = sm_base.add(self.sm as usize * 12 + 1); // execctrl is +1 from clkdiv
            let execctrl = core::ptr::read_volatile(sm_execctrl);
            let new_execctrl = (execctrl & !0x1F07F000) // Clear wrap bits
                | ((wrap as u32) << 12)
                | ((wrap_target as u32) << 7);
            core::ptr::write_volatile(sm_execctrl as *mut u32, new_execctrl);
        }
    }

    /// Configure SET pins
    ///
    /// # Arguments
    /// * `base` - The lowest-numbered pin that will be affected by a SET PINS or SET PINDIRS instruction
    /// * `count` - The number of pins (1-5)
    pub fn set_set_pins(&self, base: u8, count: u8) {
        assert!(count > 0 && count <= 5, "SET pin count must be 1-5");
        unsafe {
            let sm_base = &self.pio.sm0_clkdiv as *const _ as *const u32;
            let sm_pinctrl = sm_base.add(self.sm as usize * 12 + 3); // pinctrl is +3 from clkdiv
            let pinctrl = core::ptr::read_volatile(sm_pinctrl);
            let new_pinctrl = (pinctrl & !0x1F07C000) // Clear SET_BASE and SET_COUNT
                | ((base as u32) << 26)
                | (((count - 1) as u32) << 14);
            core::ptr::write_volatile(sm_pinctrl as *mut u32, new_pinctrl);
        }
    }

    /// Configure OUT pins
    ///
    /// # Arguments
    /// * `base` - The lowest-numbered pin that will be affected by an OUT PINS, OUT PINDIRS or MOV PINS instruction
    /// * `count` - The number of pins (0-32)
    pub fn set_out_pins(&self, base: u8, count: u8) {
        unsafe {
            let sm_base = &self.pio.sm0_clkdiv as *const _ as *const u32;
            let sm_pinctrl = sm_base.add(self.sm as usize * 12 + 3);
            let pinctrl = core::ptr::read_volatile(sm_pinctrl);
            let new_pinctrl = (pinctrl & !0x3F03F000) // Clear OUT_BASE and OUT_COUNT
                | ((base as u32) << 20)
                | ((count as u32) << 12);
            core::ptr::write_volatile(sm_pinctrl as *mut u32, new_pinctrl);
        }
    }

    /// Configure IN pins
    ///
    /// # Arguments
    /// * `base` - The pin to use as the first IN pin
    pub fn set_in_pins(&self, base: u8) {
        unsafe {
            let sm_base = &self.pio.sm0_clkdiv as *const _ as *const u32;
            let sm_pinctrl = sm_base.add(self.sm as usize * 12 + 3);
            let pinctrl = core::ptr::read_volatile(sm_pinctrl);
            let new_pinctrl = (pinctrl & !0x1F000000) // Clear IN_BASE
                | ((base as u32) << 24);
            core::ptr::write_volatile(sm_pinctrl as *mut u32, new_pinctrl);
        }
    }

    /// Configure side-set pins
    ///
    /// # Arguments
    /// * `base` - The lowest-numbered pin that will be affected by a side-set operation
    /// * `count` - The number of bits used for side-set (0-5)
    /// * `optional` - If true, bit 4 of the instruction is used to enable side-set
    /// * `pindirs` - If true, side-set affects pin directions instead of values
    pub fn set_sideset_pins(&self, base: u8, count: u8, optional: bool, pindirs: bool) {
        assert!(count <= 5, "Side-set count must be 0-5");
        unsafe {
            let sm_base = &self.pio.sm0_clkdiv as *const _ as *const u32;
            let sm_pinctrl = sm_base.add(self.sm as usize * 12 + 3);
            let pinctrl = core::ptr::read_volatile(sm_pinctrl);
            let new_pinctrl = (pinctrl & !0xF8000000) // Clear SIDESET_BASE and SIDESET_COUNT
                | ((base as u32) << 29)
                | ((count as u32) << 27);

            core::ptr::write_volatile(sm_pinctrl as *mut u32, new_pinctrl);

            // Configure optional and pindirs in EXECCTRL
            let sm_execctrl = sm_base.add(self.sm as usize * 12 + 1);
            let execctrl = core::ptr::read_volatile(sm_execctrl);
            let new_execctrl = (execctrl & !0x60000000) // Clear SIDE_EN and SIDE_PINDIR
                | ((optional as u32) << 30)
                | ((pindirs as u32) << 29);
            core::ptr::write_volatile(sm_execctrl as *mut u32, new_execctrl);
        }
    }

    /// Set consecutive pin directions using the state machine
    ///
    /// # Arguments
    /// * `pin_base` - The first pin to set
    /// * `pin_count` - Number of consecutive pins
    /// * `is_out` - true for output, false for input
    pub fn set_consecutive_pindirs(&self, pin_base: u8, pin_count: u8, is_out: bool) {
        let pindir_val = if is_out { (1u32 << pin_count) - 1 } else { 0 };

        // Temporarily set SET pins configuration
        self.set_set_pins(pin_base, pin_count);

        // Execute SET PINDIRS instruction
        self.exec(encode_set_pindirs(pindir_val as u8));
    }

    /// Execute an instruction immediately
    #[inline]
    pub fn exec(&self, instr: u16) {
        unsafe {
            let sm_base = &self.pio.sm0_clkdiv as *const _ as *const u32;
            let sm_instr = sm_base.add(self.sm as usize * 12 + 4); // instr is +4 from clkdiv
            core::ptr::write_volatile(sm_instr as *mut u32, instr as u32);
        }
    }

    /// Check if TX FIFO is full
    #[inline]
    pub fn tx_fifo_is_full(&self) -> bool {
        let mask = 1u32 << (self.sm + 16); // TXFULL bits start at 16
        (self.pio.fstat.read_bits() & mask) != 0
    }

    /// Check if TX FIFO is empty
    #[inline]
    pub fn tx_fifo_is_empty(&self) -> bool {
        let mask = 1u32 << (self.sm + 24); // TXEMPTY bits start at 24
        (self.pio.fstat.read_bits() & mask) != 0
    }

    /// Check if RX FIFO is full
    #[inline]
    pub fn rx_fifo_is_full(&self) -> bool {
        let mask = 1u32 << (self.sm + 8); // RXFULL bits start at 8
        (self.pio.fstat.read_bits() & mask) != 0
    }

    /// Check if RX FIFO is empty
    #[inline]
    pub fn rx_fifo_is_empty(&self) -> bool {
        let mask = 1u32 << self.sm; // RXEMPTY bits start at 0
        (self.pio.fstat.read_bits() & mask) != 0
    }

    /// Write to TX FIFO (blocking)
    pub fn tx_fifo_write(&self, data: u32) {
        while self.tx_fifo_is_full() {
            core::hint::spin_loop();
        }

        unsafe {
            let txf_base = &self.pio.txf0 as *const _ as *const u32;
            let txf = txf_base.add(self.sm as usize);
            core::ptr::write_volatile(txf as *mut u32, data);
        }
    }

    /// Try to write to TX FIFO (non-blocking)
    pub fn tx_fifo_try_write(&self, data: u32) -> bool {
        if self.tx_fifo_is_full() {
            return false;
        }

        unsafe {
            let txf_base = &self.pio.txf0 as *const _ as *const u32;
            let txf = txf_base.add(self.sm as usize);
            core::ptr::write_volatile(txf as *mut u32, data);
        }
        true
    }

    /// Read from RX FIFO (blocking)
    pub fn rx_fifo_read(&self) -> u32 {
        while self.rx_fifo_is_empty() {
            core::hint::spin_loop();
        }

        unsafe {
            let rxf_base = &self.pio.rxf0 as *const _ as *const u32;
            let rxf = rxf_base.add(self.sm as usize);
            core::ptr::read_volatile(rxf)
        }
    }

    /// Try to read from RX FIFO (non-blocking)
    pub fn rx_fifo_try_read(&self) -> Option<u32> {
        if self.rx_fifo_is_empty() {
            return None;
        }

        unsafe {
            let rxf_base = &self.pio.rxf0 as *const _ as *const u32;
            let rxf = rxf_base.add(self.sm as usize);
            Some(core::ptr::read_volatile(rxf))
        }
    }

    /// Clear both TX and RX FIFOs
    pub fn clear_fifos(&self) {
        unsafe {
            let sm_base = &self.pio.sm0_clkdiv as *const _ as *const u32;
            let sm_shiftctrl = sm_base.add(self.sm as usize * 12 + 2);
            let shiftctrl = core::ptr::read_volatile(sm_shiftctrl);

            // Set FJOIN_RX and FJOIN_TX temporarily to flush
            let flush = shiftctrl | (1 << 31) | (1 << 30);
            core::ptr::write_volatile(sm_shiftctrl as *mut u32, flush);
            core::ptr::write_volatile(sm_shiftctrl as *mut u32, flush);

            // Restore original
            core::ptr::write_volatile(sm_shiftctrl as *mut u32, shiftctrl);
        }
    }
}

// Helper functions for encoding PIO instructions

/// Encode a JMP instruction
#[inline]
pub const fn encode_jmp(addr: u8) -> u16 {
    0x0000 | (addr as u16 & 0x1F)
}

/// Encode a WAIT instruction (wait for pin)
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

/// Encode a NOP instruction
#[inline]
pub const fn encode_nop() -> u16 {
    0xA042 // MOV y, y
}
