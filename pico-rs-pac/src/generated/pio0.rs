//! PIO0
//! Programmable IO block
//!
//! This peripheral supports atomic SET/CLR/XOR register aliases.

#![allow(dead_code)]

pub const BASE: u32 = 0x50200000;

/// CTRL Register
///
/// PIO control register
///
/// Address offset: 0x0000
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NEXTPREV_CLKDIV_RESTART** `[26]` (WO)
///
///   Write 1 to restart the clock dividers of state machines in neighbouring PIO blocks, as specified by NEXT_PIO_MASK and PREV_PIO_MASK in the same write.
///   This is equivalent to writing 1 to the corresponding CLKDIV_RESTART bits in those PIOs' CTRL registers.
///
/// - **NEXTPREV_SM_DISABLE** `[25]` (WO)
///
///   Write 1 to disable state machines in neighbouring PIO blocks, as specified by NEXT_PIO_MASK and PREV_PIO_MASK in the same write.
///   This is equivalent to clearing the corresponding SM_ENABLE bits in those PIOs' CTRL registers.
///
/// - **NEXTPREV_SM_ENABLE** `[24]` (WO)
///
///   Write 1 to enable state machines in neighbouring PIO blocks, as specified by NEXT_PIO_MASK and PREV_PIO_MASK in the same write.
///   This is equivalent to setting the corresponding SM_ENABLE bits in those PIOs' CTRL registers.
///   If both OTHERS_SM_ENABLE and OTHERS_SM_DISABLE are set, the disable takes precedence.
///
/// - **NEXT_PIO_MASK** `[23:20]` (WO)
///
///   A mask of state machines in the neighbouring higher-numbered PIO block in the system (or PIO block 0 if this is the highest-numbered PIO block) to which to apply the operations specified by NEXTPREV_CLKDIV_RESTART, NEXTPREV_SM_ENABLE, and NEXTPREV_SM_DISABLE in the same write.
///   This allows state machines in a neighbouring PIO block to be started/stopped/clock-synced exactly simultaneously with a write to this PIO block's CTRL register.
///   Note that in a system with two PIOs, NEXT_PIO_MASK and PREV_PIO_MASK actually indicate the same PIO block. In this case the effects are applied cumulatively (as though the masks were OR'd together).
///   Neighbouring PIO blocks are disconnected (status signals tied to 0 and control signals ignored) if one block is accessible to NonSecure code, and one is not.
///
/// - **PREV_PIO_MASK** `[19:16]` (WO)
///
///   A mask of state machines in the neighbouring lower-numbered PIO block in the system (or the highest-numbered PIO block if this is PIO block 0) to which to apply the operations specified by OP_CLKDIV_RESTART, OP_ENABLE, OP_DISABLE in the same write.
///   This allows state machines in a neighbouring PIO block to be started/stopped/clock-synced exactly simultaneously with a write to this PIO block's CTRL register.
///   Neighbouring PIO blocks are disconnected (status signals tied to 0 and control signals ignored) if one block is accessible to NonSecure code, and one is not.
///
/// - **CLKDIV_RESTART** `[11:8]` (WO)
///
///   Restart a state machine's clock divider from an initial phase of 0. Clock dividers are free-running, so once started, their output (including fractional jitter) is completely determined by the integer/fractional divisor configured in SMx_CLKDIV. This means that, if multiple clock dividers with the same divisor are restarted simultaneously, by writing multiple 1 bits to this field, the execution clocks of those state machines will run in precise lockstep.
///   Note that setting/clearing SM_ENABLE does not stop the clock divider from running, so once multiple state machines' clocks are synchronised, it is safe to disable/reenable a state machine, whilst keeping the clock dividers in sync.
///   Note also that CLKDIV_RESTART can be written to whilst the state machine is running, and this is useful to resynchronise clock dividers after the divisors (SMx_CLKDIV) have been changed on-the-fly.
///
/// - **SM_RESTART** `[7:4]` (WO)
///
///   Write 1 to instantly clear internal SM state which may be otherwise difficult to access and will affect future execution.
///   Specifically, the following are cleared: input and output shift counters; the contents of the input shift register; the delay counter; the waiting-on-IRQ state; any stalled instruction written to SMx_INSTR or run by OUT/MOV EXEC; any pin write left asserted due to OUT_STICKY.
///   The contents of the output shift register and the X/Y scratch registers are not affected.
///
/// - **SM_ENABLE** `[3:0]` (RW)
///
///   Enable/disable each of the four state machines by writing 1/0 to each of these four bits. When disabled, a state machine will cease executing instructions, except those written directly to SMx_INSTR by the system. Multiple bits can be set/cleared at once to run/halt multiple state machines simultaneously.
#[repr(transparent)]
pub struct Ctrl {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Ctrl {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ctrl {
    // ======== Field Constants ========

    /// **NEXTPREV_CLKDIV_RESTART**
    ///
    /// Write 1 to restart the clock dividers of state machines in neighbouring PIO blocks, as specified by NEXT_PIO_MASK and PREV_PIO_MASK in the same write.
    /// This is equivalent to writing 1 to the corresponding CLKDIV_RESTART bits in those PIOs' CTRL registers.
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const NEXTPREV_CLKDIV_RESTART_OFFSET: u32 = 26;
    pub const NEXTPREV_CLKDIV_RESTART_MASK: u32 = 0x04000000;
    pub const NEXTPREV_CLKDIV_RESTART: u32 = 1 << 26;

    /// **NEXTPREV_SM_DISABLE**
    ///
    /// Write 1 to disable state machines in neighbouring PIO blocks, as specified by NEXT_PIO_MASK and PREV_PIO_MASK in the same write.
    /// This is equivalent to clearing the corresponding SM_ENABLE bits in those PIOs' CTRL registers.
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const NEXTPREV_SM_DISABLE_OFFSET: u32 = 25;
    pub const NEXTPREV_SM_DISABLE_MASK: u32 = 0x02000000;
    pub const NEXTPREV_SM_DISABLE: u32 = 1 << 25;

    /// **NEXTPREV_SM_ENABLE**
    ///
    /// Write 1 to enable state machines in neighbouring PIO blocks, as specified by NEXT_PIO_MASK and PREV_PIO_MASK in the same write.
    /// This is equivalent to setting the corresponding SM_ENABLE bits in those PIOs' CTRL registers.
    /// If both OTHERS_SM_ENABLE and OTHERS_SM_DISABLE are set, the disable takes precedence.
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const NEXTPREV_SM_ENABLE_OFFSET: u32 = 24;
    pub const NEXTPREV_SM_ENABLE_MASK: u32 = 0x01000000;
    pub const NEXTPREV_SM_ENABLE: u32 = 1 << 24;

    /// **NEXT_PIO_MASK**
    ///
    /// A mask of state machines in the neighbouring higher-numbered PIO block in the system (or PIO block 0 if this is the highest-numbered PIO block) to which to apply the operations specified by NEXTPREV_CLKDIV_RESTART, NEXTPREV_SM_ENABLE, and NEXTPREV_SM_DISABLE in the same write.
    /// This allows state machines in a neighbouring PIO block to be started/stopped/clock-synced exactly simultaneously with a write to this PIO block's CTRL register.
    /// Note that in a system with two PIOs, NEXT_PIO_MASK and PREV_PIO_MASK actually indicate the same PIO block. In this case the effects are applied cumulatively (as though the masks were OR'd together).
    /// Neighbouring PIO blocks are disconnected (status signals tied to 0 and control signals ignored) if one block is accessible to NonSecure code, and one is not.
    ///
    /// Bits `[23:20]`, mask `0x00f00000`
    pub const NEXT_PIO_MASK_OFFSET: u32 = 20;
    pub const NEXT_PIO_MASK_MASK: u32 = 0x00f00000;

    /// **PREV_PIO_MASK**
    ///
    /// A mask of state machines in the neighbouring lower-numbered PIO block in the system (or the highest-numbered PIO block if this is PIO block 0) to which to apply the operations specified by OP_CLKDIV_RESTART, OP_ENABLE, OP_DISABLE in the same write.
    /// This allows state machines in a neighbouring PIO block to be started/stopped/clock-synced exactly simultaneously with a write to this PIO block's CTRL register.
    /// Neighbouring PIO blocks are disconnected (status signals tied to 0 and control signals ignored) if one block is accessible to NonSecure code, and one is not.
    ///
    /// Bits `[19:16]`, mask `0x000f0000`
    pub const PREV_PIO_MASK_OFFSET: u32 = 16;
    pub const PREV_PIO_MASK_MASK: u32 = 0x000f0000;

    /// **CLKDIV_RESTART**
    ///
    /// Restart a state machine's clock divider from an initial phase of 0. Clock dividers are free-running, so once started, their output (including fractional jitter) is completely determined by the integer/fractional divisor configured in SMx_CLKDIV. This means that, if multiple clock dividers with the same divisor are restarted simultaneously, by writing multiple 1 bits to this field, the execution clocks of those state machines will run in precise lockstep.
    /// Note that setting/clearing SM_ENABLE does not stop the clock divider from running, so once multiple state machines' clocks are synchronised, it is safe to disable/reenable a state machine, whilst keeping the clock dividers in sync.
    /// Note also that CLKDIV_RESTART can be written to whilst the state machine is running, and this is useful to resynchronise clock dividers after the divisors (SMx_CLKDIV) have been changed on-the-fly.
    ///
    /// Bits `[11:8]`, mask `0x00000f00`
    pub const CLKDIV_RESTART_OFFSET: u32 = 8;
    pub const CLKDIV_RESTART_MASK: u32 = 0x00000f00;

    /// **SM_RESTART**
    ///
    /// Write 1 to instantly clear internal SM state which may be otherwise difficult to access and will affect future execution.
    /// Specifically, the following are cleared: input and output shift counters; the contents of the input shift register; the delay counter; the waiting-on-IRQ state; any stalled instruction written to SMx_INSTR or run by OUT/MOV EXEC; any pin write left asserted due to OUT_STICKY.
    /// The contents of the output shift register and the X/Y scratch registers are not affected.
    ///
    /// Bits `[7:4]`, mask `0x000000f0`
    pub const SM_RESTART_OFFSET: u32 = 4;
    pub const SM_RESTART_MASK: u32 = 0x000000f0;

    /// **SM_ENABLE**
    ///
    /// Enable/disable each of the four state machines by writing 1/0 to each of these four bits. When disabled, a state machine will cease executing instructions, except those written directly to SMx_INSTR by the system. Multiple bits can be set/cleared at once to run/halt multiple state machines simultaneously.
    ///
    /// Bits `[3:0]`, mask `0x0000000f`
    pub const SM_ENABLE_OFFSET: u32 = 0;
    pub const SM_ENABLE_MASK: u32 = 0x0000000f;

    // ======== Field Accessors ========

    /// Trigger **NEXTPREV_CLKDIV_RESTART** (WO)
    ///
    /// Write 1 to restart the clock dividers of state machines in neighbouring PIO blocks, as specified by NEXT_PIO_MASK and PREV_PIO_MASK in the same write.
    /// This is equivalent to writing 1 to the corresponding CLKDIV_RESTART bits in those PIOs' CTRL registers.
    #[inline(always)]
    pub fn trigger_nextprev_clkdiv_restart(&self) {
        self.value.write_bits(Self::NEXTPREV_CLKDIV_RESTART_MASK);
    }

    /// Trigger **NEXTPREV_SM_DISABLE** (WO)
    ///
    /// Write 1 to disable state machines in neighbouring PIO blocks, as specified by NEXT_PIO_MASK and PREV_PIO_MASK in the same write.
    /// This is equivalent to clearing the corresponding SM_ENABLE bits in those PIOs' CTRL registers.
    #[inline(always)]
    pub fn trigger_nextprev_sm_disable(&self) {
        self.value.write_bits(Self::NEXTPREV_SM_DISABLE_MASK);
    }

    /// Trigger **NEXTPREV_SM_ENABLE** (WO)
    ///
    /// Write 1 to enable state machines in neighbouring PIO blocks, as specified by NEXT_PIO_MASK and PREV_PIO_MASK in the same write.
    /// This is equivalent to setting the corresponding SM_ENABLE bits in those PIOs' CTRL registers.
    /// If both OTHERS_SM_ENABLE and OTHERS_SM_DISABLE are set, the disable takes precedence.
    #[inline(always)]
    pub fn trigger_nextprev_sm_enable(&self) {
        self.value.write_bits(Self::NEXTPREV_SM_ENABLE_MASK);
    }

    /// Write **NEXT_PIO_MASK** `[23:20]` (WO)
    ///
    /// A mask of state machines in the neighbouring higher-numbered PIO block in the system (or PIO block 0 if this is the highest-numbered PIO block) to which to apply the operations specified by NEXTPREV_CLKDIV_RESTART, NEXTPREV_SM_ENABLE, and NEXTPREV_SM_DISABLE in the same write.
    /// This allows state machines in a neighbouring PIO block to be started/stopped/clock-synced exactly simultaneously with a write to this PIO block's CTRL register.
    /// Note that in a system with two PIOs, NEXT_PIO_MASK and PREV_PIO_MASK actually indicate the same PIO block. In this case the effects are applied cumulatively (as though the masks were OR'd together).
    /// Neighbouring PIO blocks are disconnected (status signals tied to 0 and control signals ignored) if one block is accessible to NonSecure code, and one is not.
    #[inline(always)]
    pub fn write_next_pio_mask(&self, value: u32) {
        self.value.write_bits((value << Self::NEXT_PIO_MASK_OFFSET) & Self::NEXT_PIO_MASK_MASK);
    }

    /// Write **PREV_PIO_MASK** `[19:16]` (WO)
    ///
    /// A mask of state machines in the neighbouring lower-numbered PIO block in the system (or the highest-numbered PIO block if this is PIO block 0) to which to apply the operations specified by OP_CLKDIV_RESTART, OP_ENABLE, OP_DISABLE in the same write.
    /// This allows state machines in a neighbouring PIO block to be started/stopped/clock-synced exactly simultaneously with a write to this PIO block's CTRL register.
    /// Neighbouring PIO blocks are disconnected (status signals tied to 0 and control signals ignored) if one block is accessible to NonSecure code, and one is not.
    #[inline(always)]
    pub fn write_prev_pio_mask(&self, value: u32) {
        self.value.write_bits((value << Self::PREV_PIO_MASK_OFFSET) & Self::PREV_PIO_MASK_MASK);
    }

    /// Write **CLKDIV_RESTART** `[11:8]` (WO)
    ///
    /// Restart a state machine's clock divider from an initial phase of 0. Clock dividers are free-running, so once started, their output (including fractional jitter) is completely determined by the integer/fractional divisor configured in SMx_CLKDIV. This means that, if multiple clock dividers with the same divisor are restarted simultaneously, by writing multiple 1 bits to this field, the execution clocks of those state machines will run in precise lockstep.
    /// Note that setting/clearing SM_ENABLE does not stop the clock divider from running, so once multiple state machines' clocks are synchronised, it is safe to disable/reenable a state machine, whilst keeping the clock dividers in sync.
    /// Note also that CLKDIV_RESTART can be written to whilst the state machine is running, and this is useful to resynchronise clock dividers after the divisors (SMx_CLKDIV) have been changed on-the-fly.
    #[inline(always)]
    pub fn write_clkdiv_restart(&self, value: u32) {
        self.value.write_bits((value << Self::CLKDIV_RESTART_OFFSET) & Self::CLKDIV_RESTART_MASK);
    }

    /// Write **SM_RESTART** `[7:4]` (WO)
    ///
    /// Write 1 to instantly clear internal SM state which may be otherwise difficult to access and will affect future execution.
    /// Specifically, the following are cleared: input and output shift counters; the contents of the input shift register; the delay counter; the waiting-on-IRQ state; any stalled instruction written to SMx_INSTR or run by OUT/MOV EXEC; any pin write left asserted due to OUT_STICKY.
    /// The contents of the output shift register and the X/Y scratch registers are not affected.
    #[inline(always)]
    pub fn write_sm_restart(&self, value: u32) {
        self.value.write_bits((value << Self::SM_RESTART_OFFSET) & Self::SM_RESTART_MASK);
    }

    /// **SM_ENABLE** `[3:0]` (RW)
    ///
    /// Enable/disable each of the four state machines by writing 1/0 to each of these four bits. When disabled, a state machine will cease executing instructions, except those written directly to SMx_INSTR by the system. Multiple bits can be set/cleared at once to run/halt multiple state machines simultaneously.
    #[inline(always)]
    pub fn sm_enable(&self) -> u32 {
        (self.value.read_bits() & Self::SM_ENABLE_MASK) >> Self::SM_ENABLE_OFFSET
    }

    /// Write **SM_ENABLE** `[3:0]` (RW)
    ///
    /// Enable/disable each of the four state machines by writing 1/0 to each of these four bits. When disabled, a state machine will cease executing instructions, except those written directly to SMx_INSTR by the system. Multiple bits can be set/cleared at once to run/halt multiple state machines simultaneously.
    #[inline(always)]
    pub fn write_sm_enable(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SM_ENABLE_MASK) | ((value << Self::SM_ENABLE_OFFSET) & Self::SM_ENABLE_MASK));
    }

}

/// CTRL value type for batch read/modify/write operations
///
/// Use `ctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CtrlVal {
    bits: u32,
}

impl CtrlVal {
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

    /// Read **SM_ENABLE** `[3:0]`
    ///
    /// Enable/disable each of the four state machines by writing 1/0 to each of these four bits. When disabled, a state machine will cease executing instructions, except those written directly to SMx_INSTR by the system. Multiple bits can be set/cleared at once to run/halt multiple state machines simultaneously.
    #[inline(always)]
    pub const fn sm_enable(&self) -> u32 {
        (self.bits & Ctrl::SM_ENABLE_MASK) >> Ctrl::SM_ENABLE_OFFSET
    }

    /// Set **NEXTPREV_CLKDIV_RESTART** `[26:26]`
    ///
    /// Write 1 to restart the clock dividers of state machines in neighbouring PIO blocks, as specified by NEXT_PIO_MASK and PREV_PIO_MASK in the same write.
    /// This is equivalent to writing 1 to the corresponding CLKDIV_RESTART bits in those PIOs' CTRL registers.
    #[inline(always)]
    pub const fn with_nextprev_clkdiv_restart(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctrl::NEXTPREV_CLKDIV_RESTART_MASK
            } else {
                self.bits & !Ctrl::NEXTPREV_CLKDIV_RESTART_MASK
            }
        }
    }

    /// Set **NEXTPREV_SM_DISABLE** `[25:25]`
    ///
    /// Write 1 to disable state machines in neighbouring PIO blocks, as specified by NEXT_PIO_MASK and PREV_PIO_MASK in the same write.
    /// This is equivalent to clearing the corresponding SM_ENABLE bits in those PIOs' CTRL registers.
    #[inline(always)]
    pub const fn with_nextprev_sm_disable(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctrl::NEXTPREV_SM_DISABLE_MASK
            } else {
                self.bits & !Ctrl::NEXTPREV_SM_DISABLE_MASK
            }
        }
    }

    /// Set **NEXTPREV_SM_ENABLE** `[24:24]`
    ///
    /// Write 1 to enable state machines in neighbouring PIO blocks, as specified by NEXT_PIO_MASK and PREV_PIO_MASK in the same write.
    /// This is equivalent to setting the corresponding SM_ENABLE bits in those PIOs' CTRL registers.
    /// If both OTHERS_SM_ENABLE and OTHERS_SM_DISABLE are set, the disable takes precedence.
    #[inline(always)]
    pub const fn with_nextprev_sm_enable(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctrl::NEXTPREV_SM_ENABLE_MASK
            } else {
                self.bits & !Ctrl::NEXTPREV_SM_ENABLE_MASK
            }
        }
    }

    /// Set **NEXT_PIO_MASK** `[23:20]`
    ///
    /// A mask of state machines in the neighbouring higher-numbered PIO block in the system (or PIO block 0 if this is the highest-numbered PIO block) to which to apply the operations specified by NEXTPREV_CLKDIV_RESTART, NEXTPREV_SM_ENABLE, and NEXTPREV_SM_DISABLE in the same write.
    /// This allows state machines in a neighbouring PIO block to be started/stopped/clock-synced exactly simultaneously with a write to this PIO block's CTRL register.
    /// Note that in a system with two PIOs, NEXT_PIO_MASK and PREV_PIO_MASK actually indicate the same PIO block. In this case the effects are applied cumulatively (as though the masks were OR'd together).
    /// Neighbouring PIO blocks are disconnected (status signals tied to 0 and control signals ignored) if one block is accessible to NonSecure code, and one is not.
    #[inline(always)]
    pub const fn with_next_pio_mask(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ctrl::NEXT_PIO_MASK_MASK) | ((value << Ctrl::NEXT_PIO_MASK_OFFSET) & Ctrl::NEXT_PIO_MASK_MASK)
        }
    }

    /// Set **PREV_PIO_MASK** `[19:16]`
    ///
    /// A mask of state machines in the neighbouring lower-numbered PIO block in the system (or the highest-numbered PIO block if this is PIO block 0) to which to apply the operations specified by OP_CLKDIV_RESTART, OP_ENABLE, OP_DISABLE in the same write.
    /// This allows state machines in a neighbouring PIO block to be started/stopped/clock-synced exactly simultaneously with a write to this PIO block's CTRL register.
    /// Neighbouring PIO blocks are disconnected (status signals tied to 0 and control signals ignored) if one block is accessible to NonSecure code, and one is not.
    #[inline(always)]
    pub const fn with_prev_pio_mask(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ctrl::PREV_PIO_MASK_MASK) | ((value << Ctrl::PREV_PIO_MASK_OFFSET) & Ctrl::PREV_PIO_MASK_MASK)
        }
    }

    /// Set **CLKDIV_RESTART** `[11:8]`
    ///
    /// Restart a state machine's clock divider from an initial phase of 0. Clock dividers are free-running, so once started, their output (including fractional jitter) is completely determined by the integer/fractional divisor configured in SMx_CLKDIV. This means that, if multiple clock dividers with the same divisor are restarted simultaneously, by writing multiple 1 bits to this field, the execution clocks of those state machines will run in precise lockstep.
    /// Note that setting/clearing SM_ENABLE does not stop the clock divider from running, so once multiple state machines' clocks are synchronised, it is safe to disable/reenable a state machine, whilst keeping the clock dividers in sync.
    /// Note also that CLKDIV_RESTART can be written to whilst the state machine is running, and this is useful to resynchronise clock dividers after the divisors (SMx_CLKDIV) have been changed on-the-fly.
    #[inline(always)]
    pub const fn with_clkdiv_restart(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ctrl::CLKDIV_RESTART_MASK) | ((value << Ctrl::CLKDIV_RESTART_OFFSET) & Ctrl::CLKDIV_RESTART_MASK)
        }
    }

    /// Set **SM_RESTART** `[7:4]`
    ///
    /// Write 1 to instantly clear internal SM state which may be otherwise difficult to access and will affect future execution.
    /// Specifically, the following are cleared: input and output shift counters; the contents of the input shift register; the delay counter; the waiting-on-IRQ state; any stalled instruction written to SMx_INSTR or run by OUT/MOV EXEC; any pin write left asserted due to OUT_STICKY.
    /// The contents of the output shift register and the X/Y scratch registers are not affected.
    #[inline(always)]
    pub const fn with_sm_restart(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ctrl::SM_RESTART_MASK) | ((value << Ctrl::SM_RESTART_OFFSET) & Ctrl::SM_RESTART_MASK)
        }
    }

    /// Set **SM_ENABLE** `[3:0]`
    ///
    /// Enable/disable each of the four state machines by writing 1/0 to each of these four bits. When disabled, a state machine will cease executing instructions, except those written directly to SMx_INSTR by the system. Multiple bits can be set/cleared at once to run/halt multiple state machines simultaneously.
    #[inline(always)]
    pub const fn with_sm_enable(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ctrl::SM_ENABLE_MASK) | ((value << Ctrl::SM_ENABLE_OFFSET) & Ctrl::SM_ENABLE_MASK)
        }
    }

}

impl Default for CtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ctrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CtrlVal {
        CtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CtrlVal) -> CtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// FSTAT Register
///
/// FIFO status register
///
/// Address offset: 0x0004
/// Reset value: 0x0f000f00
///
/// ## Fields
///
/// - **TXEMPTY** `[27:24]` (RO)
///
///   State machine TX FIFO is empty
///
/// - **TXFULL** `[19:16]` (RO)
///
///   State machine TX FIFO is full
///
/// - **RXEMPTY** `[11:8]` (RO)
///
///   State machine RX FIFO is empty
///
/// - **RXFULL** `[3:0]` (RO)
///
///   State machine RX FIFO is full
#[repr(transparent)]
pub struct Fstat {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Fstat {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Fstat {
    // ======== Field Constants ========

    /// **TXEMPTY**
    ///
    /// State machine TX FIFO is empty
    ///
    /// Bits `[27:24]`, mask `0x0f000000`
    pub const TXEMPTY_OFFSET: u32 = 24;
    pub const TXEMPTY_MASK: u32 = 0x0f000000;

    /// **TXFULL**
    ///
    /// State machine TX FIFO is full
    ///
    /// Bits `[19:16]`, mask `0x000f0000`
    pub const TXFULL_OFFSET: u32 = 16;
    pub const TXFULL_MASK: u32 = 0x000f0000;

    /// **RXEMPTY**
    ///
    /// State machine RX FIFO is empty
    ///
    /// Bits `[11:8]`, mask `0x00000f00`
    pub const RXEMPTY_OFFSET: u32 = 8;
    pub const RXEMPTY_MASK: u32 = 0x00000f00;

    /// **RXFULL**
    ///
    /// State machine RX FIFO is full
    ///
    /// Bits `[3:0]`, mask `0x0000000f`
    pub const RXFULL_OFFSET: u32 = 0;
    pub const RXFULL_MASK: u32 = 0x0000000f;

    // ======== Field Accessors ========

    /// **TXEMPTY** `[27:24]` (RO)
    ///
    /// State machine TX FIFO is empty
    #[inline(always)]
    pub fn txempty(&self) -> u32 {
        (self.value.read_bits() & Self::TXEMPTY_MASK) >> Self::TXEMPTY_OFFSET
    }

    /// **TXFULL** `[19:16]` (RO)
    ///
    /// State machine TX FIFO is full
    #[inline(always)]
    pub fn txfull(&self) -> u32 {
        (self.value.read_bits() & Self::TXFULL_MASK) >> Self::TXFULL_OFFSET
    }

    /// **RXEMPTY** `[11:8]` (RO)
    ///
    /// State machine RX FIFO is empty
    #[inline(always)]
    pub fn rxempty(&self) -> u32 {
        (self.value.read_bits() & Self::RXEMPTY_MASK) >> Self::RXEMPTY_OFFSET
    }

    /// **RXFULL** `[3:0]` (RO)
    ///
    /// State machine RX FIFO is full
    #[inline(always)]
    pub fn rxfull(&self) -> u32 {
        (self.value.read_bits() & Self::RXFULL_MASK) >> Self::RXFULL_OFFSET
    }

}

/// FSTAT value type for batch read/modify/write operations
///
/// Use `fstat.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `fstat.write_val(val)`.
#[derive(Clone, Copy)]
pub struct FstatVal {
    bits: u32,
}

impl FstatVal {
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
        Self { bits: 0x0f000f00 }
    }

    /// Read **TXEMPTY** `[27:24]`
    ///
    /// State machine TX FIFO is empty
    #[inline(always)]
    pub const fn txempty(&self) -> u32 {
        (self.bits & Fstat::TXEMPTY_MASK) >> Fstat::TXEMPTY_OFFSET
    }

    /// Read **TXFULL** `[19:16]`
    ///
    /// State machine TX FIFO is full
    #[inline(always)]
    pub const fn txfull(&self) -> u32 {
        (self.bits & Fstat::TXFULL_MASK) >> Fstat::TXFULL_OFFSET
    }

    /// Read **RXEMPTY** `[11:8]`
    ///
    /// State machine RX FIFO is empty
    #[inline(always)]
    pub const fn rxempty(&self) -> u32 {
        (self.bits & Fstat::RXEMPTY_MASK) >> Fstat::RXEMPTY_OFFSET
    }

    /// Read **RXFULL** `[3:0]`
    ///
    /// State machine RX FIFO is full
    #[inline(always)]
    pub const fn rxfull(&self) -> u32 {
        (self.bits & Fstat::RXFULL_MASK) >> Fstat::RXFULL_OFFSET
    }

}

impl Default for FstatVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Fstat {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> FstatVal {
        FstatVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: FstatVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(FstatVal) -> FstatVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// FDEBUG Register
///
/// FIFO debug register
///
/// Address offset: 0x0008
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **TXSTALL** `[27:24]` (RW)
///
///   State machine has stalled on empty TX FIFO during a blocking PULL, or an OUT with autopull enabled. Write 1 to clear.
///
/// - **TXOVER** `[19:16]` (RW)
///
///   TX FIFO overflow (i.e. write-on-full by the system) has occurred. Write 1 to clear. Note that write-on-full does not alter the state or contents of the FIFO in any way, but the data that the system attempted to write is dropped, so if this flag is set, your software has quite likely dropped some data on the floor.
///
/// - **RXUNDER** `[11:8]` (RW)
///
///   RX FIFO underflow (i.e. read-on-empty by the system) has occurred. Write 1 to clear. Note that read-on-empty does not perturb the state of the FIFO in any way, but the data returned by reading from an empty FIFO is undefined, so this flag generally only becomes set due to some kind of software error.
///
/// - **RXSTALL** `[3:0]` (RW)
///
///   State machine has stalled on full RX FIFO during a blocking PUSH, or an IN with autopush enabled. This flag is also set when a nonblocking PUSH to a full FIFO took place, in which case the state machine has dropped data. Write 1 to clear.
#[repr(transparent)]
pub struct Fdebug {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Fdebug {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Fdebug {
    // ======== Field Constants ========

    /// **TXSTALL**
    ///
    /// State machine has stalled on empty TX FIFO during a blocking PULL, or an OUT with autopull enabled. Write 1 to clear.
    ///
    /// Bits `[27:24]`, mask `0x0f000000`
    pub const TXSTALL_OFFSET: u32 = 24;
    pub const TXSTALL_MASK: u32 = 0x0f000000;

    /// **TXOVER**
    ///
    /// TX FIFO overflow (i.e. write-on-full by the system) has occurred. Write 1 to clear. Note that write-on-full does not alter the state or contents of the FIFO in any way, but the data that the system attempted to write is dropped, so if this flag is set, your software has quite likely dropped some data on the floor.
    ///
    /// Bits `[19:16]`, mask `0x000f0000`
    pub const TXOVER_OFFSET: u32 = 16;
    pub const TXOVER_MASK: u32 = 0x000f0000;

    /// **RXUNDER**
    ///
    /// RX FIFO underflow (i.e. read-on-empty by the system) has occurred. Write 1 to clear. Note that read-on-empty does not perturb the state of the FIFO in any way, but the data returned by reading from an empty FIFO is undefined, so this flag generally only becomes set due to some kind of software error.
    ///
    /// Bits `[11:8]`, mask `0x00000f00`
    pub const RXUNDER_OFFSET: u32 = 8;
    pub const RXUNDER_MASK: u32 = 0x00000f00;

    /// **RXSTALL**
    ///
    /// State machine has stalled on full RX FIFO during a blocking PUSH, or an IN with autopush enabled. This flag is also set when a nonblocking PUSH to a full FIFO took place, in which case the state machine has dropped data. Write 1 to clear.
    ///
    /// Bits `[3:0]`, mask `0x0000000f`
    pub const RXSTALL_OFFSET: u32 = 0;
    pub const RXSTALL_MASK: u32 = 0x0000000f;

    // ======== Field Accessors ========

    /// **TXSTALL** `[27:24]` (RW)
    ///
    /// State machine has stalled on empty TX FIFO during a blocking PULL, or an OUT with autopull enabled. Write 1 to clear.
    #[inline(always)]
    pub fn txstall(&self) -> u32 {
        (self.value.read_bits() & Self::TXSTALL_MASK) >> Self::TXSTALL_OFFSET
    }

    /// Write **TXSTALL** `[27:24]` (RW)
    ///
    /// State machine has stalled on empty TX FIFO during a blocking PULL, or an OUT with autopull enabled. Write 1 to clear.
    #[inline(always)]
    pub fn write_txstall(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::TXSTALL_MASK) | ((value << Self::TXSTALL_OFFSET) & Self::TXSTALL_MASK));
    }

    /// **TXOVER** `[19:16]` (RW)
    ///
    /// TX FIFO overflow (i.e. write-on-full by the system) has occurred. Write 1 to clear. Note that write-on-full does not alter the state or contents of the FIFO in any way, but the data that the system attempted to write is dropped, so if this flag is set, your software has quite likely dropped some data on the floor.
    #[inline(always)]
    pub fn txover(&self) -> u32 {
        (self.value.read_bits() & Self::TXOVER_MASK) >> Self::TXOVER_OFFSET
    }

    /// Write **TXOVER** `[19:16]` (RW)
    ///
    /// TX FIFO overflow (i.e. write-on-full by the system) has occurred. Write 1 to clear. Note that write-on-full does not alter the state or contents of the FIFO in any way, but the data that the system attempted to write is dropped, so if this flag is set, your software has quite likely dropped some data on the floor.
    #[inline(always)]
    pub fn write_txover(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::TXOVER_MASK) | ((value << Self::TXOVER_OFFSET) & Self::TXOVER_MASK));
    }

    /// **RXUNDER** `[11:8]` (RW)
    ///
    /// RX FIFO underflow (i.e. read-on-empty by the system) has occurred. Write 1 to clear. Note that read-on-empty does not perturb the state of the FIFO in any way, but the data returned by reading from an empty FIFO is undefined, so this flag generally only becomes set due to some kind of software error.
    #[inline(always)]
    pub fn rxunder(&self) -> u32 {
        (self.value.read_bits() & Self::RXUNDER_MASK) >> Self::RXUNDER_OFFSET
    }

    /// Write **RXUNDER** `[11:8]` (RW)
    ///
    /// RX FIFO underflow (i.e. read-on-empty by the system) has occurred. Write 1 to clear. Note that read-on-empty does not perturb the state of the FIFO in any way, but the data returned by reading from an empty FIFO is undefined, so this flag generally only becomes set due to some kind of software error.
    #[inline(always)]
    pub fn write_rxunder(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::RXUNDER_MASK) | ((value << Self::RXUNDER_OFFSET) & Self::RXUNDER_MASK));
    }

    /// **RXSTALL** `[3:0]` (RW)
    ///
    /// State machine has stalled on full RX FIFO during a blocking PUSH, or an IN with autopush enabled. This flag is also set when a nonblocking PUSH to a full FIFO took place, in which case the state machine has dropped data. Write 1 to clear.
    #[inline(always)]
    pub fn rxstall(&self) -> u32 {
        (self.value.read_bits() & Self::RXSTALL_MASK) >> Self::RXSTALL_OFFSET
    }

    /// Write **RXSTALL** `[3:0]` (RW)
    ///
    /// State machine has stalled on full RX FIFO during a blocking PUSH, or an IN with autopush enabled. This flag is also set when a nonblocking PUSH to a full FIFO took place, in which case the state machine has dropped data. Write 1 to clear.
    #[inline(always)]
    pub fn write_rxstall(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::RXSTALL_MASK) | ((value << Self::RXSTALL_OFFSET) & Self::RXSTALL_MASK));
    }

}

/// FDEBUG value type for batch read/modify/write operations
///
/// Use `fdebug.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `fdebug.write_val(val)`.
#[derive(Clone, Copy)]
pub struct FdebugVal {
    bits: u32,
}

impl FdebugVal {
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

    /// Read **TXSTALL** `[27:24]`
    ///
    /// State machine has stalled on empty TX FIFO during a blocking PULL, or an OUT with autopull enabled. Write 1 to clear.
    #[inline(always)]
    pub const fn txstall(&self) -> u32 {
        (self.bits & Fdebug::TXSTALL_MASK) >> Fdebug::TXSTALL_OFFSET
    }

    /// Read **TXOVER** `[19:16]`
    ///
    /// TX FIFO overflow (i.e. write-on-full by the system) has occurred. Write 1 to clear. Note that write-on-full does not alter the state or contents of the FIFO in any way, but the data that the system attempted to write is dropped, so if this flag is set, your software has quite likely dropped some data on the floor.
    #[inline(always)]
    pub const fn txover(&self) -> u32 {
        (self.bits & Fdebug::TXOVER_MASK) >> Fdebug::TXOVER_OFFSET
    }

    /// Read **RXUNDER** `[11:8]`
    ///
    /// RX FIFO underflow (i.e. read-on-empty by the system) has occurred. Write 1 to clear. Note that read-on-empty does not perturb the state of the FIFO in any way, but the data returned by reading from an empty FIFO is undefined, so this flag generally only becomes set due to some kind of software error.
    #[inline(always)]
    pub const fn rxunder(&self) -> u32 {
        (self.bits & Fdebug::RXUNDER_MASK) >> Fdebug::RXUNDER_OFFSET
    }

    /// Read **RXSTALL** `[3:0]`
    ///
    /// State machine has stalled on full RX FIFO during a blocking PUSH, or an IN with autopush enabled. This flag is also set when a nonblocking PUSH to a full FIFO took place, in which case the state machine has dropped data. Write 1 to clear.
    #[inline(always)]
    pub const fn rxstall(&self) -> u32 {
        (self.bits & Fdebug::RXSTALL_MASK) >> Fdebug::RXSTALL_OFFSET
    }

    /// Set **TXSTALL** `[27:24]`
    ///
    /// State machine has stalled on empty TX FIFO during a blocking PULL, or an OUT with autopull enabled. Write 1 to clear.
    #[inline(always)]
    pub const fn with_txstall(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Fdebug::TXSTALL_MASK) | ((value << Fdebug::TXSTALL_OFFSET) & Fdebug::TXSTALL_MASK)
        }
    }

    /// Set **TXOVER** `[19:16]`
    ///
    /// TX FIFO overflow (i.e. write-on-full by the system) has occurred. Write 1 to clear. Note that write-on-full does not alter the state or contents of the FIFO in any way, but the data that the system attempted to write is dropped, so if this flag is set, your software has quite likely dropped some data on the floor.
    #[inline(always)]
    pub const fn with_txover(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Fdebug::TXOVER_MASK) | ((value << Fdebug::TXOVER_OFFSET) & Fdebug::TXOVER_MASK)
        }
    }

    /// Set **RXUNDER** `[11:8]`
    ///
    /// RX FIFO underflow (i.e. read-on-empty by the system) has occurred. Write 1 to clear. Note that read-on-empty does not perturb the state of the FIFO in any way, but the data returned by reading from an empty FIFO is undefined, so this flag generally only becomes set due to some kind of software error.
    #[inline(always)]
    pub const fn with_rxunder(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Fdebug::RXUNDER_MASK) | ((value << Fdebug::RXUNDER_OFFSET) & Fdebug::RXUNDER_MASK)
        }
    }

    /// Set **RXSTALL** `[3:0]`
    ///
    /// State machine has stalled on full RX FIFO during a blocking PUSH, or an IN with autopush enabled. This flag is also set when a nonblocking PUSH to a full FIFO took place, in which case the state machine has dropped data. Write 1 to clear.
    #[inline(always)]
    pub const fn with_rxstall(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Fdebug::RXSTALL_MASK) | ((value << Fdebug::RXSTALL_OFFSET) & Fdebug::RXSTALL_MASK)
        }
    }

}

impl Default for FdebugVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Fdebug {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> FdebugVal {
        FdebugVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: FdebugVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(FdebugVal) -> FdebugVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// FLEVEL Register
///
/// FIFO levels
///
/// Address offset: 0x000c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **RX3** `[31:28]` (RO)
///
/// - **TX3** `[27:24]` (RO)
///
/// - **RX2** `[23:20]` (RO)
///
/// - **TX2** `[19:16]` (RO)
///
/// - **RX1** `[15:12]` (RO)
///
/// - **TX1** `[11:8]` (RO)
///
/// - **RX0** `[7:4]` (RO)
///
/// - **TX0** `[3:0]` (RO)
#[repr(transparent)]
pub struct Flevel {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Flevel {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Flevel {
    // ======== Field Constants ========

    /// **RX3**
    /// Bits `[31:28]`, mask `0xf0000000`
    pub const RX3_OFFSET: u32 = 28;
    pub const RX3_MASK: u32 = 0xf0000000;

    /// **TX3**
    /// Bits `[27:24]`, mask `0x0f000000`
    pub const TX3_OFFSET: u32 = 24;
    pub const TX3_MASK: u32 = 0x0f000000;

    /// **RX2**
    /// Bits `[23:20]`, mask `0x00f00000`
    pub const RX2_OFFSET: u32 = 20;
    pub const RX2_MASK: u32 = 0x00f00000;

    /// **TX2**
    /// Bits `[19:16]`, mask `0x000f0000`
    pub const TX2_OFFSET: u32 = 16;
    pub const TX2_MASK: u32 = 0x000f0000;

    /// **RX1**
    /// Bits `[15:12]`, mask `0x0000f000`
    pub const RX1_OFFSET: u32 = 12;
    pub const RX1_MASK: u32 = 0x0000f000;

    /// **TX1**
    /// Bits `[11:8]`, mask `0x00000f00`
    pub const TX1_OFFSET: u32 = 8;
    pub const TX1_MASK: u32 = 0x00000f00;

    /// **RX0**
    /// Bits `[7:4]`, mask `0x000000f0`
    pub const RX0_OFFSET: u32 = 4;
    pub const RX0_MASK: u32 = 0x000000f0;

    /// **TX0**
    /// Bits `[3:0]`, mask `0x0000000f`
    pub const TX0_OFFSET: u32 = 0;
    pub const TX0_MASK: u32 = 0x0000000f;

    // ======== Field Accessors ========

    /// **RX3** `[31:28]` (RO)
    #[inline(always)]
    pub fn rx3(&self) -> u32 {
        (self.value.read_bits() & Self::RX3_MASK) >> Self::RX3_OFFSET
    }

    /// **TX3** `[27:24]` (RO)
    #[inline(always)]
    pub fn tx3(&self) -> u32 {
        (self.value.read_bits() & Self::TX3_MASK) >> Self::TX3_OFFSET
    }

    /// **RX2** `[23:20]` (RO)
    #[inline(always)]
    pub fn rx2(&self) -> u32 {
        (self.value.read_bits() & Self::RX2_MASK) >> Self::RX2_OFFSET
    }

    /// **TX2** `[19:16]` (RO)
    #[inline(always)]
    pub fn tx2(&self) -> u32 {
        (self.value.read_bits() & Self::TX2_MASK) >> Self::TX2_OFFSET
    }

    /// **RX1** `[15:12]` (RO)
    #[inline(always)]
    pub fn rx1(&self) -> u32 {
        (self.value.read_bits() & Self::RX1_MASK) >> Self::RX1_OFFSET
    }

    /// **TX1** `[11:8]` (RO)
    #[inline(always)]
    pub fn tx1(&self) -> u32 {
        (self.value.read_bits() & Self::TX1_MASK) >> Self::TX1_OFFSET
    }

    /// **RX0** `[7:4]` (RO)
    #[inline(always)]
    pub fn rx0(&self) -> u32 {
        (self.value.read_bits() & Self::RX0_MASK) >> Self::RX0_OFFSET
    }

    /// **TX0** `[3:0]` (RO)
    #[inline(always)]
    pub fn tx0(&self) -> u32 {
        (self.value.read_bits() & Self::TX0_MASK) >> Self::TX0_OFFSET
    }

}

/// FLEVEL value type for batch read/modify/write operations
///
/// Use `flevel.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `flevel.write_val(val)`.
#[derive(Clone, Copy)]
pub struct FlevelVal {
    bits: u32,
}

impl FlevelVal {
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

    /// Read **RX3** `[31:28]`
    #[inline(always)]
    pub const fn rx3(&self) -> u32 {
        (self.bits & Flevel::RX3_MASK) >> Flevel::RX3_OFFSET
    }

    /// Read **TX3** `[27:24]`
    #[inline(always)]
    pub const fn tx3(&self) -> u32 {
        (self.bits & Flevel::TX3_MASK) >> Flevel::TX3_OFFSET
    }

    /// Read **RX2** `[23:20]`
    #[inline(always)]
    pub const fn rx2(&self) -> u32 {
        (self.bits & Flevel::RX2_MASK) >> Flevel::RX2_OFFSET
    }

    /// Read **TX2** `[19:16]`
    #[inline(always)]
    pub const fn tx2(&self) -> u32 {
        (self.bits & Flevel::TX2_MASK) >> Flevel::TX2_OFFSET
    }

    /// Read **RX1** `[15:12]`
    #[inline(always)]
    pub const fn rx1(&self) -> u32 {
        (self.bits & Flevel::RX1_MASK) >> Flevel::RX1_OFFSET
    }

    /// Read **TX1** `[11:8]`
    #[inline(always)]
    pub const fn tx1(&self) -> u32 {
        (self.bits & Flevel::TX1_MASK) >> Flevel::TX1_OFFSET
    }

    /// Read **RX0** `[7:4]`
    #[inline(always)]
    pub const fn rx0(&self) -> u32 {
        (self.bits & Flevel::RX0_MASK) >> Flevel::RX0_OFFSET
    }

    /// Read **TX0** `[3:0]`
    #[inline(always)]
    pub const fn tx0(&self) -> u32 {
        (self.bits & Flevel::TX0_MASK) >> Flevel::TX0_OFFSET
    }

}

impl Default for FlevelVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Flevel {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> FlevelVal {
        FlevelVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: FlevelVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(FlevelVal) -> FlevelVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// TXF0 Register
///
/// Direct write access to the TX FIFO for this state machine. Each write pushes one word to the FIFO. Attempting to write to a full FIFO has no effect on the FIFO state or contents, and sets the sticky FDEBUG_TXOVER error flag for this FIFO.
///
/// Address offset: 0x0010
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **TXF0** `[31:0]` (WO)
#[repr(transparent)]
pub struct Txf0 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Txf0 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Txf0 {
    // ======== Field Constants ========

    /// **TXF0**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const TXF0_OFFSET: u32 = 0;
    pub const TXF0_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// Write **TXF0** `[31:0]` (WO)
    #[inline(always)]
    pub fn write_txf0(&self, value: u32) {
        self.value.write_bits((value << Self::TXF0_OFFSET) & Self::TXF0_MASK);
    }

}

/// TXF0 value type for batch read/modify/write operations
///
/// Use `txf0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `txf0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Txf0Val {
    bits: u32,
}

impl Txf0Val {
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

    /// Set **TXF0** `[31:0]`
    #[inline(always)]
    pub const fn with_txf0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Txf0::TXF0_MASK) | ((value << Txf0::TXF0_OFFSET) & Txf0::TXF0_MASK)
        }
    }

}

impl Default for Txf0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Txf0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Txf0Val {
        Txf0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Txf0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Txf0Val) -> Txf0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// TXF1 Register
///
/// Direct write access to the TX FIFO for this state machine. Each write pushes one word to the FIFO. Attempting to write to a full FIFO has no effect on the FIFO state or contents, and sets the sticky FDEBUG_TXOVER error flag for this FIFO.
///
/// Address offset: 0x0014
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **TXF1** `[31:0]` (WO)
#[repr(transparent)]
pub struct Txf1 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Txf1 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Txf1 {
    // ======== Field Constants ========

    /// **TXF1**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const TXF1_OFFSET: u32 = 0;
    pub const TXF1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// Write **TXF1** `[31:0]` (WO)
    #[inline(always)]
    pub fn write_txf1(&self, value: u32) {
        self.value.write_bits((value << Self::TXF1_OFFSET) & Self::TXF1_MASK);
    }

}

/// TXF1 value type for batch read/modify/write operations
///
/// Use `txf1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `txf1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Txf1Val {
    bits: u32,
}

impl Txf1Val {
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

    /// Set **TXF1** `[31:0]`
    #[inline(always)]
    pub const fn with_txf1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Txf1::TXF1_MASK) | ((value << Txf1::TXF1_OFFSET) & Txf1::TXF1_MASK)
        }
    }

}

impl Default for Txf1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Txf1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Txf1Val {
        Txf1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Txf1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Txf1Val) -> Txf1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// TXF2 Register
///
/// Direct write access to the TX FIFO for this state machine. Each write pushes one word to the FIFO. Attempting to write to a full FIFO has no effect on the FIFO state or contents, and sets the sticky FDEBUG_TXOVER error flag for this FIFO.
///
/// Address offset: 0x0018
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **TXF2** `[31:0]` (WO)
#[repr(transparent)]
pub struct Txf2 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Txf2 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Txf2 {
    // ======== Field Constants ========

    /// **TXF2**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const TXF2_OFFSET: u32 = 0;
    pub const TXF2_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// Write **TXF2** `[31:0]` (WO)
    #[inline(always)]
    pub fn write_txf2(&self, value: u32) {
        self.value.write_bits((value << Self::TXF2_OFFSET) & Self::TXF2_MASK);
    }

}

/// TXF2 value type for batch read/modify/write operations
///
/// Use `txf2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `txf2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Txf2Val {
    bits: u32,
}

impl Txf2Val {
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

    /// Set **TXF2** `[31:0]`
    #[inline(always)]
    pub const fn with_txf2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Txf2::TXF2_MASK) | ((value << Txf2::TXF2_OFFSET) & Txf2::TXF2_MASK)
        }
    }

}

impl Default for Txf2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Txf2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Txf2Val {
        Txf2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Txf2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Txf2Val) -> Txf2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// TXF3 Register
///
/// Direct write access to the TX FIFO for this state machine. Each write pushes one word to the FIFO. Attempting to write to a full FIFO has no effect on the FIFO state or contents, and sets the sticky FDEBUG_TXOVER error flag for this FIFO.
///
/// Address offset: 0x001c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **TXF3** `[31:0]` (WO)
#[repr(transparent)]
pub struct Txf3 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Txf3 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Txf3 {
    // ======== Field Constants ========

    /// **TXF3**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const TXF3_OFFSET: u32 = 0;
    pub const TXF3_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// Write **TXF3** `[31:0]` (WO)
    #[inline(always)]
    pub fn write_txf3(&self, value: u32) {
        self.value.write_bits((value << Self::TXF3_OFFSET) & Self::TXF3_MASK);
    }

}

/// TXF3 value type for batch read/modify/write operations
///
/// Use `txf3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `txf3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Txf3Val {
    bits: u32,
}

impl Txf3Val {
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

    /// Set **TXF3** `[31:0]`
    #[inline(always)]
    pub const fn with_txf3(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Txf3::TXF3_MASK) | ((value << Txf3::TXF3_OFFSET) & Txf3::TXF3_MASK)
        }
    }

}

impl Default for Txf3Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Txf3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Txf3Val {
        Txf3Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Txf3Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Txf3Val) -> Txf3Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// RXF0 Register
///
/// Direct read access to the RX FIFO for this state machine. Each read pops one word from the FIFO. Attempting to read from an empty FIFO has no effect on the FIFO state, and sets the sticky FDEBUG_RXUNDER error flag for this FIFO. The data returned to the system on a read from an empty FIFO is undefined.
///
/// Address offset: 0x0020
///
/// ## Fields
///
/// - **RXF0** `[31:0]` (RO)
#[repr(transparent)]
pub struct Rxf0 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Rxf0 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Rxf0 {
    // ======== Field Constants ========

    /// **RXF0**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const RXF0_OFFSET: u32 = 0;
    pub const RXF0_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **RXF0** `[31:0]` (RO)
    #[inline(always)]
    pub fn rxf0(&self) -> u32 {
        (self.value.read_bits() & Self::RXF0_MASK) >> Self::RXF0_OFFSET
    }

}

/// RXF0 value type for batch read/modify/write operations
///
/// Use `rxf0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxf0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Rxf0Val {
    bits: u32,
}

impl Rxf0Val {
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

    /// Read **RXF0** `[31:0]`
    #[inline(always)]
    pub const fn rxf0(&self) -> u32 {
        (self.bits & Rxf0::RXF0_MASK) >> Rxf0::RXF0_OFFSET
    }

}

impl Default for Rxf0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Rxf0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Rxf0Val {
        Rxf0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Rxf0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Rxf0Val) -> Rxf0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// RXF1 Register
///
/// Direct read access to the RX FIFO for this state machine. Each read pops one word from the FIFO. Attempting to read from an empty FIFO has no effect on the FIFO state, and sets the sticky FDEBUG_RXUNDER error flag for this FIFO. The data returned to the system on a read from an empty FIFO is undefined.
///
/// Address offset: 0x0024
///
/// ## Fields
///
/// - **RXF1** `[31:0]` (RO)
#[repr(transparent)]
pub struct Rxf1 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Rxf1 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Rxf1 {
    // ======== Field Constants ========

    /// **RXF1**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const RXF1_OFFSET: u32 = 0;
    pub const RXF1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **RXF1** `[31:0]` (RO)
    #[inline(always)]
    pub fn rxf1(&self) -> u32 {
        (self.value.read_bits() & Self::RXF1_MASK) >> Self::RXF1_OFFSET
    }

}

/// RXF1 value type for batch read/modify/write operations
///
/// Use `rxf1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxf1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Rxf1Val {
    bits: u32,
}

impl Rxf1Val {
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

    /// Read **RXF1** `[31:0]`
    #[inline(always)]
    pub const fn rxf1(&self) -> u32 {
        (self.bits & Rxf1::RXF1_MASK) >> Rxf1::RXF1_OFFSET
    }

}

impl Default for Rxf1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Rxf1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Rxf1Val {
        Rxf1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Rxf1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Rxf1Val) -> Rxf1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// RXF2 Register
///
/// Direct read access to the RX FIFO for this state machine. Each read pops one word from the FIFO. Attempting to read from an empty FIFO has no effect on the FIFO state, and sets the sticky FDEBUG_RXUNDER error flag for this FIFO. The data returned to the system on a read from an empty FIFO is undefined.
///
/// Address offset: 0x0028
///
/// ## Fields
///
/// - **RXF2** `[31:0]` (RO)
#[repr(transparent)]
pub struct Rxf2 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Rxf2 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Rxf2 {
    // ======== Field Constants ========

    /// **RXF2**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const RXF2_OFFSET: u32 = 0;
    pub const RXF2_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **RXF2** `[31:0]` (RO)
    #[inline(always)]
    pub fn rxf2(&self) -> u32 {
        (self.value.read_bits() & Self::RXF2_MASK) >> Self::RXF2_OFFSET
    }

}

/// RXF2 value type for batch read/modify/write operations
///
/// Use `rxf2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxf2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Rxf2Val {
    bits: u32,
}

impl Rxf2Val {
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

    /// Read **RXF2** `[31:0]`
    #[inline(always)]
    pub const fn rxf2(&self) -> u32 {
        (self.bits & Rxf2::RXF2_MASK) >> Rxf2::RXF2_OFFSET
    }

}

impl Default for Rxf2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Rxf2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Rxf2Val {
        Rxf2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Rxf2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Rxf2Val) -> Rxf2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// RXF3 Register
///
/// Direct read access to the RX FIFO for this state machine. Each read pops one word from the FIFO. Attempting to read from an empty FIFO has no effect on the FIFO state, and sets the sticky FDEBUG_RXUNDER error flag for this FIFO. The data returned to the system on a read from an empty FIFO is undefined.
///
/// Address offset: 0x002c
///
/// ## Fields
///
/// - **RXF3** `[31:0]` (RO)
#[repr(transparent)]
pub struct Rxf3 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Rxf3 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Rxf3 {
    // ======== Field Constants ========

    /// **RXF3**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const RXF3_OFFSET: u32 = 0;
    pub const RXF3_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **RXF3** `[31:0]` (RO)
    #[inline(always)]
    pub fn rxf3(&self) -> u32 {
        (self.value.read_bits() & Self::RXF3_MASK) >> Self::RXF3_OFFSET
    }

}

/// RXF3 value type for batch read/modify/write operations
///
/// Use `rxf3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxf3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Rxf3Val {
    bits: u32,
}

impl Rxf3Val {
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

    /// Read **RXF3** `[31:0]`
    #[inline(always)]
    pub const fn rxf3(&self) -> u32 {
        (self.bits & Rxf3::RXF3_MASK) >> Rxf3::RXF3_OFFSET
    }

}

impl Default for Rxf3Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Rxf3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Rxf3Val {
        Rxf3Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Rxf3Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Rxf3Val) -> Rxf3Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// IRQ Register
///
/// State machine IRQ flags register. Write 1 to clear. There are eight state machine IRQ flags, which can be set, cleared, and waited on by the state machines. There's no fixed association between flags and state machines -- any state machine can use any flag.
/// Any of the eight flags can be used for timing synchronisation between state machines, using IRQ and WAIT instructions. Any combination of the eight flags can also routed out to either of the two system-level interrupt requests, alongside FIFO status interrupts -- see e.g. IRQ0_INTE.
///
/// Address offset: 0x0030
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **IRQ** `[7:0]` (RW)
#[repr(transparent)]
pub struct Irq {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Irq {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Irq {
    // ======== Field Constants ========

    /// **IRQ**
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const IRQ_OFFSET: u32 = 0;
    pub const IRQ_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **IRQ** `[7:0]` (RW)
    #[inline(always)]
    pub fn irq(&self) -> u32 {
        (self.value.read_bits() & Self::IRQ_MASK) >> Self::IRQ_OFFSET
    }

    /// Write **IRQ** `[7:0]` (RW)
    #[inline(always)]
    pub fn write_irq(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::IRQ_MASK) | ((value << Self::IRQ_OFFSET) & Self::IRQ_MASK));
    }

}

/// IRQ value type for batch read/modify/write operations
///
/// Use `irq.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `irq.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IrqVal {
    bits: u32,
}

impl IrqVal {
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

    /// Read **IRQ** `[7:0]`
    #[inline(always)]
    pub const fn irq(&self) -> u32 {
        (self.bits & Irq::IRQ_MASK) >> Irq::IRQ_OFFSET
    }

    /// Set **IRQ** `[7:0]`
    #[inline(always)]
    pub const fn with_irq(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Irq::IRQ_MASK) | ((value << Irq::IRQ_OFFSET) & Irq::IRQ_MASK)
        }
    }

}

impl Default for IrqVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Irq {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IrqVal {
        IrqVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IrqVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IrqVal) -> IrqVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// IRQ_FORCE Register
///
/// Writing a 1 to each of these bits will forcibly assert the corresponding IRQ. Note this is different to the INTF register: writing here affects PIO internal state. INTF just asserts the processor-facing IRQ signal for testing ISRs, and is not visible to the state machines.
///
/// Address offset: 0x0034
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **IRQ_FORCE** `[7:0]` (WO)
#[repr(transparent)]
pub struct IrqForce {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for IrqForce {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IrqForce {
    // ======== Field Constants ========

    /// **IRQ_FORCE**
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const IRQ_FORCE_OFFSET: u32 = 0;
    pub const IRQ_FORCE_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// Write **IRQ_FORCE** `[7:0]` (WO)
    #[inline(always)]
    pub fn write_irq_force(&self, value: u32) {
        self.value.write_bits((value << Self::IRQ_FORCE_OFFSET) & Self::IRQ_FORCE_MASK);
    }

}

/// IRQ_FORCE value type for batch read/modify/write operations
///
/// Use `irqforce.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `irqforce.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IrqForceVal {
    bits: u32,
}

impl IrqForceVal {
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

    /// Set **IRQ_FORCE** `[7:0]`
    #[inline(always)]
    pub const fn with_irq_force(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !IrqForce::IRQ_FORCE_MASK) | ((value << IrqForce::IRQ_FORCE_OFFSET) & IrqForce::IRQ_FORCE_MASK)
        }
    }

}

impl Default for IrqForceVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl IrqForce {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IrqForceVal {
        IrqForceVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IrqForceVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IrqForceVal) -> IrqForceVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// INPUT_SYNC_BYPASS Register
///
/// There is a 2-flipflop synchronizer on each GPIO input, which protects PIO logic from metastabilities. This increases input delay, and for fast synchronous IO (e.g. SPI) these synchronizers may need to be bypassed. Each bit in this register corresponds to one GPIO.
/// 0 -> input is synchronized (default)
/// 1 -> synchronizer is bypassed
/// If in doubt, leave this register as all zeroes.
///
/// Address offset: 0x0038
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **INPUT_SYNC_BYPASS** `[31:0]` (RW)
#[repr(transparent)]
pub struct InputSyncBypass {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for InputSyncBypass {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InputSyncBypass {
    // ======== Field Constants ========

    /// **INPUT_SYNC_BYPASS**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const INPUT_SYNC_BYPASS_OFFSET: u32 = 0;
    pub const INPUT_SYNC_BYPASS_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **INPUT_SYNC_BYPASS** `[31:0]` (RW)
    #[inline(always)]
    pub fn input_sync_bypass(&self) -> u32 {
        (self.value.read_bits() & Self::INPUT_SYNC_BYPASS_MASK) >> Self::INPUT_SYNC_BYPASS_OFFSET
    }

    /// Write **INPUT_SYNC_BYPASS** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_input_sync_bypass(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::INPUT_SYNC_BYPASS_MASK) | ((value << Self::INPUT_SYNC_BYPASS_OFFSET) & Self::INPUT_SYNC_BYPASS_MASK));
    }

}

/// INPUT_SYNC_BYPASS value type for batch read/modify/write operations
///
/// Use `inputsyncbypass.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `inputsyncbypass.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InputSyncBypassVal {
    bits: u32,
}

impl InputSyncBypassVal {
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

    /// Read **INPUT_SYNC_BYPASS** `[31:0]`
    #[inline(always)]
    pub const fn input_sync_bypass(&self) -> u32 {
        (self.bits & InputSyncBypass::INPUT_SYNC_BYPASS_MASK) >> InputSyncBypass::INPUT_SYNC_BYPASS_OFFSET
    }

    /// Set **INPUT_SYNC_BYPASS** `[31:0]`
    #[inline(always)]
    pub const fn with_input_sync_bypass(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !InputSyncBypass::INPUT_SYNC_BYPASS_MASK) | ((value << InputSyncBypass::INPUT_SYNC_BYPASS_OFFSET) & InputSyncBypass::INPUT_SYNC_BYPASS_MASK)
        }
    }

}

impl Default for InputSyncBypassVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl InputSyncBypass {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InputSyncBypassVal {
        InputSyncBypassVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: InputSyncBypassVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(InputSyncBypassVal) -> InputSyncBypassVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// DBG_PADOUT Register
///
/// Read to sample the pad output values PIO is currently driving to the GPIOs. On RP2040 there are 30 GPIOs, so the two most significant bits are hardwired to 0.
///
/// Address offset: 0x003c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **DBG_PADOUT** `[31:0]` (RO)
#[repr(transparent)]
pub struct DbgPadout {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for DbgPadout {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DbgPadout {
    // ======== Field Constants ========

    /// **DBG_PADOUT**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const DBG_PADOUT_OFFSET: u32 = 0;
    pub const DBG_PADOUT_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **DBG_PADOUT** `[31:0]` (RO)
    #[inline(always)]
    pub fn dbg_padout(&self) -> u32 {
        (self.value.read_bits() & Self::DBG_PADOUT_MASK) >> Self::DBG_PADOUT_OFFSET
    }

}

/// DBG_PADOUT value type for batch read/modify/write operations
///
/// Use `dbgpadout.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dbgpadout.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DbgPadoutVal {
    bits: u32,
}

impl DbgPadoutVal {
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

    /// Read **DBG_PADOUT** `[31:0]`
    #[inline(always)]
    pub const fn dbg_padout(&self) -> u32 {
        (self.bits & DbgPadout::DBG_PADOUT_MASK) >> DbgPadout::DBG_PADOUT_OFFSET
    }

}

impl Default for DbgPadoutVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl DbgPadout {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DbgPadoutVal {
        DbgPadoutVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DbgPadoutVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DbgPadoutVal) -> DbgPadoutVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// DBG_PADOE Register
///
/// Read to sample the pad output enables (direction) PIO is currently driving to the GPIOs. On RP2040 there are 30 GPIOs, so the two most significant bits are hardwired to 0.
///
/// Address offset: 0x0040
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **DBG_PADOE** `[31:0]` (RO)
#[repr(transparent)]
pub struct DbgPadoe {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for DbgPadoe {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DbgPadoe {
    // ======== Field Constants ========

    /// **DBG_PADOE**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const DBG_PADOE_OFFSET: u32 = 0;
    pub const DBG_PADOE_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **DBG_PADOE** `[31:0]` (RO)
    #[inline(always)]
    pub fn dbg_padoe(&self) -> u32 {
        (self.value.read_bits() & Self::DBG_PADOE_MASK) >> Self::DBG_PADOE_OFFSET
    }

}

/// DBG_PADOE value type for batch read/modify/write operations
///
/// Use `dbgpadoe.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dbgpadoe.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DbgPadoeVal {
    bits: u32,
}

impl DbgPadoeVal {
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

    /// Read **DBG_PADOE** `[31:0]`
    #[inline(always)]
    pub const fn dbg_padoe(&self) -> u32 {
        (self.bits & DbgPadoe::DBG_PADOE_MASK) >> DbgPadoe::DBG_PADOE_OFFSET
    }

}

impl Default for DbgPadoeVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl DbgPadoe {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DbgPadoeVal {
        DbgPadoeVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DbgPadoeVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DbgPadoeVal) -> DbgPadoeVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// DBG_CFGINFO Register
///
/// The PIO hardware has some free parameters that may vary between chip products.
/// These should be provided in the chip datasheet, but are also exposed here.
///
/// Address offset: 0x0044
/// Reset value: 0x10000000
///
/// ## Fields
///
/// - **VERSION** `[31:28]` (RO)
///
///   Version of the core PIO hardware.
///
/// - **IMEM_SIZE** `[21:16]` (RO)
///
///   The size of the instruction memory, measured in units of one instruction
///
/// - **SM_COUNT** `[11:8]` (RO)
///
///   The number of state machines this PIO instance is equipped with.
///
/// - **FIFO_DEPTH** `[5:0]` (RO)
///
///   The depth of the state machine TX/RX FIFOs, measured in words.
///   Joining fifos via SHIFTCTRL_FJOIN gives one FIFO with double
///   this depth.
#[repr(transparent)]
pub struct DbgCfginfo {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for DbgCfginfo {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DbgCfginfo {
    // ======== Field Constants ========

    /// **VERSION**
    ///
    /// Version of the core PIO hardware.
    ///
    /// Bits `[31:28]`, mask `0xf0000000`
    pub const VERSION_OFFSET: u32 = 28;
    pub const VERSION_MASK: u32 = 0xf0000000;
    /// v0 = 0
    /// Version 0 (RP2040)
    pub const VERSION_V0: u32 = 0 << 28;
    pub const VERSION_V0_VAL: u32 = 0;
    /// v1 = 1
    /// Version 1 (RP2350)
    pub const VERSION_V1: u32 = 1 << 28;
    pub const VERSION_V1_VAL: u32 = 1;

    /// **IMEM_SIZE**
    ///
    /// The size of the instruction memory, measured in units of one instruction
    ///
    /// Bits `[21:16]`, mask `0x003f0000`
    pub const IMEM_SIZE_OFFSET: u32 = 16;
    pub const IMEM_SIZE_MASK: u32 = 0x003f0000;

    /// **SM_COUNT**
    ///
    /// The number of state machines this PIO instance is equipped with.
    ///
    /// Bits `[11:8]`, mask `0x00000f00`
    pub const SM_COUNT_OFFSET: u32 = 8;
    pub const SM_COUNT_MASK: u32 = 0x00000f00;

    /// **FIFO_DEPTH**
    ///
    /// The depth of the state machine TX/RX FIFOs, measured in words.
    /// Joining fifos via SHIFTCTRL_FJOIN gives one FIFO with double
    /// this depth.
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const FIFO_DEPTH_OFFSET: u32 = 0;
    pub const FIFO_DEPTH_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **VERSION** `[31:28]` (RO)
    ///
    /// Version of the core PIO hardware.
    #[inline(always)]
    pub fn version(&self) -> u32 {
        (self.value.read_bits() & Self::VERSION_MASK) >> Self::VERSION_OFFSET
    }

    /// **IMEM_SIZE** `[21:16]` (RO)
    ///
    /// The size of the instruction memory, measured in units of one instruction
    #[inline(always)]
    pub fn imem_size(&self) -> u32 {
        (self.value.read_bits() & Self::IMEM_SIZE_MASK) >> Self::IMEM_SIZE_OFFSET
    }

    /// **SM_COUNT** `[11:8]` (RO)
    ///
    /// The number of state machines this PIO instance is equipped with.
    #[inline(always)]
    pub fn sm_count(&self) -> u32 {
        (self.value.read_bits() & Self::SM_COUNT_MASK) >> Self::SM_COUNT_OFFSET
    }

    /// **FIFO_DEPTH** `[5:0]` (RO)
    ///
    /// The depth of the state machine TX/RX FIFOs, measured in words.
    /// Joining fifos via SHIFTCTRL_FJOIN gives one FIFO with double
    /// this depth.
    #[inline(always)]
    pub fn fifo_depth(&self) -> u32 {
        (self.value.read_bits() & Self::FIFO_DEPTH_MASK) >> Self::FIFO_DEPTH_OFFSET
    }

}

/// DBG_CFGINFO value type for batch read/modify/write operations
///
/// Use `dbgcfginfo.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dbgcfginfo.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DbgCfginfoVal {
    bits: u32,
}

impl DbgCfginfoVal {
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
        Self { bits: 0x10000000 }
    }

    /// Read **VERSION** `[31:28]`
    ///
    /// Version of the core PIO hardware.
    #[inline(always)]
    pub const fn version(&self) -> u32 {
        (self.bits & DbgCfginfo::VERSION_MASK) >> DbgCfginfo::VERSION_OFFSET
    }

    /// Read **IMEM_SIZE** `[21:16]`
    ///
    /// The size of the instruction memory, measured in units of one instruction
    #[inline(always)]
    pub const fn imem_size(&self) -> u32 {
        (self.bits & DbgCfginfo::IMEM_SIZE_MASK) >> DbgCfginfo::IMEM_SIZE_OFFSET
    }

    /// Read **SM_COUNT** `[11:8]`
    ///
    /// The number of state machines this PIO instance is equipped with.
    #[inline(always)]
    pub const fn sm_count(&self) -> u32 {
        (self.bits & DbgCfginfo::SM_COUNT_MASK) >> DbgCfginfo::SM_COUNT_OFFSET
    }

    /// Read **FIFO_DEPTH** `[5:0]`
    ///
    /// The depth of the state machine TX/RX FIFOs, measured in words.
    /// Joining fifos via SHIFTCTRL_FJOIN gives one FIFO with double
    /// this depth.
    #[inline(always)]
    pub const fn fifo_depth(&self) -> u32 {
        (self.bits & DbgCfginfo::FIFO_DEPTH_MASK) >> DbgCfginfo::FIFO_DEPTH_OFFSET
    }

}

impl Default for DbgCfginfoVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl DbgCfginfo {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DbgCfginfoVal {
        DbgCfginfoVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DbgCfginfoVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DbgCfginfoVal) -> DbgCfginfoVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// INSTR_MEM0 Register
///
/// Write-only access to instruction memory location 0
///
/// Address offset: 0x0048
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **INSTR_MEM0** `[15:0]` (WO)
#[repr(transparent)]
pub struct InstrMem0 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for InstrMem0 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InstrMem0 {
    // ======== Field Constants ========

    /// **INSTR_MEM0**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const INSTR_MEM0_OFFSET: u32 = 0;
    pub const INSTR_MEM0_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// Write **INSTR_MEM0** `[15:0]` (WO)
    #[inline(always)]
    pub fn write_instr_mem0(&self, value: u32) {
        self.value.write_bits((value << Self::INSTR_MEM0_OFFSET) & Self::INSTR_MEM0_MASK);
    }

}

/// INSTR_MEM0 value type for batch read/modify/write operations
///
/// Use `instrmem0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `instrmem0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InstrMem0Val {
    bits: u32,
}

impl InstrMem0Val {
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

    /// Set **INSTR_MEM0** `[15:0]`
    #[inline(always)]
    pub const fn with_instr_mem0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !InstrMem0::INSTR_MEM0_MASK) | ((value << InstrMem0::INSTR_MEM0_OFFSET) & InstrMem0::INSTR_MEM0_MASK)
        }
    }

}

impl Default for InstrMem0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl InstrMem0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InstrMem0Val {
        InstrMem0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: InstrMem0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(InstrMem0Val) -> InstrMem0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// INSTR_MEM1 Register
///
/// Write-only access to instruction memory location 1
///
/// Address offset: 0x004c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **INSTR_MEM1** `[15:0]` (WO)
#[repr(transparent)]
pub struct InstrMem1 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for InstrMem1 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InstrMem1 {
    // ======== Field Constants ========

    /// **INSTR_MEM1**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const INSTR_MEM1_OFFSET: u32 = 0;
    pub const INSTR_MEM1_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// Write **INSTR_MEM1** `[15:0]` (WO)
    #[inline(always)]
    pub fn write_instr_mem1(&self, value: u32) {
        self.value.write_bits((value << Self::INSTR_MEM1_OFFSET) & Self::INSTR_MEM1_MASK);
    }

}

/// INSTR_MEM1 value type for batch read/modify/write operations
///
/// Use `instrmem1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `instrmem1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InstrMem1Val {
    bits: u32,
}

impl InstrMem1Val {
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

    /// Set **INSTR_MEM1** `[15:0]`
    #[inline(always)]
    pub const fn with_instr_mem1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !InstrMem1::INSTR_MEM1_MASK) | ((value << InstrMem1::INSTR_MEM1_OFFSET) & InstrMem1::INSTR_MEM1_MASK)
        }
    }

}

impl Default for InstrMem1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl InstrMem1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InstrMem1Val {
        InstrMem1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: InstrMem1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(InstrMem1Val) -> InstrMem1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// INSTR_MEM2 Register
///
/// Write-only access to instruction memory location 2
///
/// Address offset: 0x0050
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **INSTR_MEM2** `[15:0]` (WO)
#[repr(transparent)]
pub struct InstrMem2 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for InstrMem2 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InstrMem2 {
    // ======== Field Constants ========

    /// **INSTR_MEM2**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const INSTR_MEM2_OFFSET: u32 = 0;
    pub const INSTR_MEM2_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// Write **INSTR_MEM2** `[15:0]` (WO)
    #[inline(always)]
    pub fn write_instr_mem2(&self, value: u32) {
        self.value.write_bits((value << Self::INSTR_MEM2_OFFSET) & Self::INSTR_MEM2_MASK);
    }

}

/// INSTR_MEM2 value type for batch read/modify/write operations
///
/// Use `instrmem2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `instrmem2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InstrMem2Val {
    bits: u32,
}

impl InstrMem2Val {
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

    /// Set **INSTR_MEM2** `[15:0]`
    #[inline(always)]
    pub const fn with_instr_mem2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !InstrMem2::INSTR_MEM2_MASK) | ((value << InstrMem2::INSTR_MEM2_OFFSET) & InstrMem2::INSTR_MEM2_MASK)
        }
    }

}

impl Default for InstrMem2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl InstrMem2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InstrMem2Val {
        InstrMem2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: InstrMem2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(InstrMem2Val) -> InstrMem2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// INSTR_MEM3 Register
///
/// Write-only access to instruction memory location 3
///
/// Address offset: 0x0054
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **INSTR_MEM3** `[15:0]` (WO)
#[repr(transparent)]
pub struct InstrMem3 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for InstrMem3 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InstrMem3 {
    // ======== Field Constants ========

    /// **INSTR_MEM3**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const INSTR_MEM3_OFFSET: u32 = 0;
    pub const INSTR_MEM3_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// Write **INSTR_MEM3** `[15:0]` (WO)
    #[inline(always)]
    pub fn write_instr_mem3(&self, value: u32) {
        self.value.write_bits((value << Self::INSTR_MEM3_OFFSET) & Self::INSTR_MEM3_MASK);
    }

}

/// INSTR_MEM3 value type for batch read/modify/write operations
///
/// Use `instrmem3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `instrmem3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InstrMem3Val {
    bits: u32,
}

impl InstrMem3Val {
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

    /// Set **INSTR_MEM3** `[15:0]`
    #[inline(always)]
    pub const fn with_instr_mem3(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !InstrMem3::INSTR_MEM3_MASK) | ((value << InstrMem3::INSTR_MEM3_OFFSET) & InstrMem3::INSTR_MEM3_MASK)
        }
    }

}

impl Default for InstrMem3Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl InstrMem3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InstrMem3Val {
        InstrMem3Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: InstrMem3Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(InstrMem3Val) -> InstrMem3Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// INSTR_MEM4 Register
///
/// Write-only access to instruction memory location 4
///
/// Address offset: 0x0058
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **INSTR_MEM4** `[15:0]` (WO)
#[repr(transparent)]
pub struct InstrMem4 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for InstrMem4 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InstrMem4 {
    // ======== Field Constants ========

    /// **INSTR_MEM4**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const INSTR_MEM4_OFFSET: u32 = 0;
    pub const INSTR_MEM4_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// Write **INSTR_MEM4** `[15:0]` (WO)
    #[inline(always)]
    pub fn write_instr_mem4(&self, value: u32) {
        self.value.write_bits((value << Self::INSTR_MEM4_OFFSET) & Self::INSTR_MEM4_MASK);
    }

}

/// INSTR_MEM4 value type for batch read/modify/write operations
///
/// Use `instrmem4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `instrmem4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InstrMem4Val {
    bits: u32,
}

impl InstrMem4Val {
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

    /// Set **INSTR_MEM4** `[15:0]`
    #[inline(always)]
    pub const fn with_instr_mem4(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !InstrMem4::INSTR_MEM4_MASK) | ((value << InstrMem4::INSTR_MEM4_OFFSET) & InstrMem4::INSTR_MEM4_MASK)
        }
    }

}

impl Default for InstrMem4Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl InstrMem4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InstrMem4Val {
        InstrMem4Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: InstrMem4Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(InstrMem4Val) -> InstrMem4Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// INSTR_MEM5 Register
///
/// Write-only access to instruction memory location 5
///
/// Address offset: 0x005c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **INSTR_MEM5** `[15:0]` (WO)
#[repr(transparent)]
pub struct InstrMem5 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for InstrMem5 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InstrMem5 {
    // ======== Field Constants ========

    /// **INSTR_MEM5**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const INSTR_MEM5_OFFSET: u32 = 0;
    pub const INSTR_MEM5_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// Write **INSTR_MEM5** `[15:0]` (WO)
    #[inline(always)]
    pub fn write_instr_mem5(&self, value: u32) {
        self.value.write_bits((value << Self::INSTR_MEM5_OFFSET) & Self::INSTR_MEM5_MASK);
    }

}

/// INSTR_MEM5 value type for batch read/modify/write operations
///
/// Use `instrmem5.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `instrmem5.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InstrMem5Val {
    bits: u32,
}

impl InstrMem5Val {
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

    /// Set **INSTR_MEM5** `[15:0]`
    #[inline(always)]
    pub const fn with_instr_mem5(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !InstrMem5::INSTR_MEM5_MASK) | ((value << InstrMem5::INSTR_MEM5_OFFSET) & InstrMem5::INSTR_MEM5_MASK)
        }
    }

}

impl Default for InstrMem5Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl InstrMem5 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InstrMem5Val {
        InstrMem5Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: InstrMem5Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(InstrMem5Val) -> InstrMem5Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// INSTR_MEM6 Register
///
/// Write-only access to instruction memory location 6
///
/// Address offset: 0x0060
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **INSTR_MEM6** `[15:0]` (WO)
#[repr(transparent)]
pub struct InstrMem6 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for InstrMem6 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InstrMem6 {
    // ======== Field Constants ========

    /// **INSTR_MEM6**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const INSTR_MEM6_OFFSET: u32 = 0;
    pub const INSTR_MEM6_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// Write **INSTR_MEM6** `[15:0]` (WO)
    #[inline(always)]
    pub fn write_instr_mem6(&self, value: u32) {
        self.value.write_bits((value << Self::INSTR_MEM6_OFFSET) & Self::INSTR_MEM6_MASK);
    }

}

/// INSTR_MEM6 value type for batch read/modify/write operations
///
/// Use `instrmem6.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `instrmem6.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InstrMem6Val {
    bits: u32,
}

impl InstrMem6Val {
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

    /// Set **INSTR_MEM6** `[15:0]`
    #[inline(always)]
    pub const fn with_instr_mem6(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !InstrMem6::INSTR_MEM6_MASK) | ((value << InstrMem6::INSTR_MEM6_OFFSET) & InstrMem6::INSTR_MEM6_MASK)
        }
    }

}

impl Default for InstrMem6Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl InstrMem6 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InstrMem6Val {
        InstrMem6Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: InstrMem6Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(InstrMem6Val) -> InstrMem6Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// INSTR_MEM7 Register
///
/// Write-only access to instruction memory location 7
///
/// Address offset: 0x0064
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **INSTR_MEM7** `[15:0]` (WO)
#[repr(transparent)]
pub struct InstrMem7 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for InstrMem7 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InstrMem7 {
    // ======== Field Constants ========

    /// **INSTR_MEM7**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const INSTR_MEM7_OFFSET: u32 = 0;
    pub const INSTR_MEM7_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// Write **INSTR_MEM7** `[15:0]` (WO)
    #[inline(always)]
    pub fn write_instr_mem7(&self, value: u32) {
        self.value.write_bits((value << Self::INSTR_MEM7_OFFSET) & Self::INSTR_MEM7_MASK);
    }

}

/// INSTR_MEM7 value type for batch read/modify/write operations
///
/// Use `instrmem7.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `instrmem7.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InstrMem7Val {
    bits: u32,
}

impl InstrMem7Val {
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

    /// Set **INSTR_MEM7** `[15:0]`
    #[inline(always)]
    pub const fn with_instr_mem7(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !InstrMem7::INSTR_MEM7_MASK) | ((value << InstrMem7::INSTR_MEM7_OFFSET) & InstrMem7::INSTR_MEM7_MASK)
        }
    }

}

impl Default for InstrMem7Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl InstrMem7 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InstrMem7Val {
        InstrMem7Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: InstrMem7Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(InstrMem7Val) -> InstrMem7Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// INSTR_MEM8 Register
///
/// Write-only access to instruction memory location 8
///
/// Address offset: 0x0068
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **INSTR_MEM8** `[15:0]` (WO)
#[repr(transparent)]
pub struct InstrMem8 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for InstrMem8 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InstrMem8 {
    // ======== Field Constants ========

    /// **INSTR_MEM8**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const INSTR_MEM8_OFFSET: u32 = 0;
    pub const INSTR_MEM8_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// Write **INSTR_MEM8** `[15:0]` (WO)
    #[inline(always)]
    pub fn write_instr_mem8(&self, value: u32) {
        self.value.write_bits((value << Self::INSTR_MEM8_OFFSET) & Self::INSTR_MEM8_MASK);
    }

}

/// INSTR_MEM8 value type for batch read/modify/write operations
///
/// Use `instrmem8.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `instrmem8.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InstrMem8Val {
    bits: u32,
}

impl InstrMem8Val {
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

    /// Set **INSTR_MEM8** `[15:0]`
    #[inline(always)]
    pub const fn with_instr_mem8(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !InstrMem8::INSTR_MEM8_MASK) | ((value << InstrMem8::INSTR_MEM8_OFFSET) & InstrMem8::INSTR_MEM8_MASK)
        }
    }

}

impl Default for InstrMem8Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl InstrMem8 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InstrMem8Val {
        InstrMem8Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: InstrMem8Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(InstrMem8Val) -> InstrMem8Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// INSTR_MEM9 Register
///
/// Write-only access to instruction memory location 9
///
/// Address offset: 0x006c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **INSTR_MEM9** `[15:0]` (WO)
#[repr(transparent)]
pub struct InstrMem9 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for InstrMem9 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InstrMem9 {
    // ======== Field Constants ========

    /// **INSTR_MEM9**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const INSTR_MEM9_OFFSET: u32 = 0;
    pub const INSTR_MEM9_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// Write **INSTR_MEM9** `[15:0]` (WO)
    #[inline(always)]
    pub fn write_instr_mem9(&self, value: u32) {
        self.value.write_bits((value << Self::INSTR_MEM9_OFFSET) & Self::INSTR_MEM9_MASK);
    }

}

/// INSTR_MEM9 value type for batch read/modify/write operations
///
/// Use `instrmem9.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `instrmem9.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InstrMem9Val {
    bits: u32,
}

impl InstrMem9Val {
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

    /// Set **INSTR_MEM9** `[15:0]`
    #[inline(always)]
    pub const fn with_instr_mem9(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !InstrMem9::INSTR_MEM9_MASK) | ((value << InstrMem9::INSTR_MEM9_OFFSET) & InstrMem9::INSTR_MEM9_MASK)
        }
    }

}

impl Default for InstrMem9Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl InstrMem9 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InstrMem9Val {
        InstrMem9Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: InstrMem9Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(InstrMem9Val) -> InstrMem9Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// INSTR_MEM10 Register
///
/// Write-only access to instruction memory location 10
///
/// Address offset: 0x0070
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **INSTR_MEM10** `[15:0]` (WO)
#[repr(transparent)]
pub struct InstrMem10 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for InstrMem10 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InstrMem10 {
    // ======== Field Constants ========

    /// **INSTR_MEM10**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const INSTR_MEM10_OFFSET: u32 = 0;
    pub const INSTR_MEM10_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// Write **INSTR_MEM10** `[15:0]` (WO)
    #[inline(always)]
    pub fn write_instr_mem10(&self, value: u32) {
        self.value.write_bits((value << Self::INSTR_MEM10_OFFSET) & Self::INSTR_MEM10_MASK);
    }

}

/// INSTR_MEM10 value type for batch read/modify/write operations
///
/// Use `instrmem10.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `instrmem10.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InstrMem10Val {
    bits: u32,
}

impl InstrMem10Val {
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

    /// Set **INSTR_MEM10** `[15:0]`
    #[inline(always)]
    pub const fn with_instr_mem10(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !InstrMem10::INSTR_MEM10_MASK) | ((value << InstrMem10::INSTR_MEM10_OFFSET) & InstrMem10::INSTR_MEM10_MASK)
        }
    }

}

impl Default for InstrMem10Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl InstrMem10 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InstrMem10Val {
        InstrMem10Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: InstrMem10Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(InstrMem10Val) -> InstrMem10Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// INSTR_MEM11 Register
///
/// Write-only access to instruction memory location 11
///
/// Address offset: 0x0074
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **INSTR_MEM11** `[15:0]` (WO)
#[repr(transparent)]
pub struct InstrMem11 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for InstrMem11 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InstrMem11 {
    // ======== Field Constants ========

    /// **INSTR_MEM11**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const INSTR_MEM11_OFFSET: u32 = 0;
    pub const INSTR_MEM11_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// Write **INSTR_MEM11** `[15:0]` (WO)
    #[inline(always)]
    pub fn write_instr_mem11(&self, value: u32) {
        self.value.write_bits((value << Self::INSTR_MEM11_OFFSET) & Self::INSTR_MEM11_MASK);
    }

}

/// INSTR_MEM11 value type for batch read/modify/write operations
///
/// Use `instrmem11.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `instrmem11.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InstrMem11Val {
    bits: u32,
}

impl InstrMem11Val {
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

    /// Set **INSTR_MEM11** `[15:0]`
    #[inline(always)]
    pub const fn with_instr_mem11(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !InstrMem11::INSTR_MEM11_MASK) | ((value << InstrMem11::INSTR_MEM11_OFFSET) & InstrMem11::INSTR_MEM11_MASK)
        }
    }

}

impl Default for InstrMem11Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl InstrMem11 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InstrMem11Val {
        InstrMem11Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: InstrMem11Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(InstrMem11Val) -> InstrMem11Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// INSTR_MEM12 Register
///
/// Write-only access to instruction memory location 12
///
/// Address offset: 0x0078
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **INSTR_MEM12** `[15:0]` (WO)
#[repr(transparent)]
pub struct InstrMem12 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for InstrMem12 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InstrMem12 {
    // ======== Field Constants ========

    /// **INSTR_MEM12**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const INSTR_MEM12_OFFSET: u32 = 0;
    pub const INSTR_MEM12_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// Write **INSTR_MEM12** `[15:0]` (WO)
    #[inline(always)]
    pub fn write_instr_mem12(&self, value: u32) {
        self.value.write_bits((value << Self::INSTR_MEM12_OFFSET) & Self::INSTR_MEM12_MASK);
    }

}

/// INSTR_MEM12 value type for batch read/modify/write operations
///
/// Use `instrmem12.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `instrmem12.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InstrMem12Val {
    bits: u32,
}

impl InstrMem12Val {
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

    /// Set **INSTR_MEM12** `[15:0]`
    #[inline(always)]
    pub const fn with_instr_mem12(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !InstrMem12::INSTR_MEM12_MASK) | ((value << InstrMem12::INSTR_MEM12_OFFSET) & InstrMem12::INSTR_MEM12_MASK)
        }
    }

}

impl Default for InstrMem12Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl InstrMem12 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InstrMem12Val {
        InstrMem12Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: InstrMem12Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(InstrMem12Val) -> InstrMem12Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// INSTR_MEM13 Register
///
/// Write-only access to instruction memory location 13
///
/// Address offset: 0x007c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **INSTR_MEM13** `[15:0]` (WO)
#[repr(transparent)]
pub struct InstrMem13 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for InstrMem13 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InstrMem13 {
    // ======== Field Constants ========

    /// **INSTR_MEM13**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const INSTR_MEM13_OFFSET: u32 = 0;
    pub const INSTR_MEM13_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// Write **INSTR_MEM13** `[15:0]` (WO)
    #[inline(always)]
    pub fn write_instr_mem13(&self, value: u32) {
        self.value.write_bits((value << Self::INSTR_MEM13_OFFSET) & Self::INSTR_MEM13_MASK);
    }

}

/// INSTR_MEM13 value type for batch read/modify/write operations
///
/// Use `instrmem13.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `instrmem13.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InstrMem13Val {
    bits: u32,
}

impl InstrMem13Val {
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

    /// Set **INSTR_MEM13** `[15:0]`
    #[inline(always)]
    pub const fn with_instr_mem13(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !InstrMem13::INSTR_MEM13_MASK) | ((value << InstrMem13::INSTR_MEM13_OFFSET) & InstrMem13::INSTR_MEM13_MASK)
        }
    }

}

impl Default for InstrMem13Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl InstrMem13 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InstrMem13Val {
        InstrMem13Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: InstrMem13Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(InstrMem13Val) -> InstrMem13Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// INSTR_MEM14 Register
///
/// Write-only access to instruction memory location 14
///
/// Address offset: 0x0080
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **INSTR_MEM14** `[15:0]` (WO)
#[repr(transparent)]
pub struct InstrMem14 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for InstrMem14 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InstrMem14 {
    // ======== Field Constants ========

    /// **INSTR_MEM14**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const INSTR_MEM14_OFFSET: u32 = 0;
    pub const INSTR_MEM14_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// Write **INSTR_MEM14** `[15:0]` (WO)
    #[inline(always)]
    pub fn write_instr_mem14(&self, value: u32) {
        self.value.write_bits((value << Self::INSTR_MEM14_OFFSET) & Self::INSTR_MEM14_MASK);
    }

}

/// INSTR_MEM14 value type for batch read/modify/write operations
///
/// Use `instrmem14.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `instrmem14.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InstrMem14Val {
    bits: u32,
}

impl InstrMem14Val {
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

    /// Set **INSTR_MEM14** `[15:0]`
    #[inline(always)]
    pub const fn with_instr_mem14(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !InstrMem14::INSTR_MEM14_MASK) | ((value << InstrMem14::INSTR_MEM14_OFFSET) & InstrMem14::INSTR_MEM14_MASK)
        }
    }

}

impl Default for InstrMem14Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl InstrMem14 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InstrMem14Val {
        InstrMem14Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: InstrMem14Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(InstrMem14Val) -> InstrMem14Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// INSTR_MEM15 Register
///
/// Write-only access to instruction memory location 15
///
/// Address offset: 0x0084
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **INSTR_MEM15** `[15:0]` (WO)
#[repr(transparent)]
pub struct InstrMem15 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for InstrMem15 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InstrMem15 {
    // ======== Field Constants ========

    /// **INSTR_MEM15**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const INSTR_MEM15_OFFSET: u32 = 0;
    pub const INSTR_MEM15_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// Write **INSTR_MEM15** `[15:0]` (WO)
    #[inline(always)]
    pub fn write_instr_mem15(&self, value: u32) {
        self.value.write_bits((value << Self::INSTR_MEM15_OFFSET) & Self::INSTR_MEM15_MASK);
    }

}

/// INSTR_MEM15 value type for batch read/modify/write operations
///
/// Use `instrmem15.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `instrmem15.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InstrMem15Val {
    bits: u32,
}

impl InstrMem15Val {
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

    /// Set **INSTR_MEM15** `[15:0]`
    #[inline(always)]
    pub const fn with_instr_mem15(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !InstrMem15::INSTR_MEM15_MASK) | ((value << InstrMem15::INSTR_MEM15_OFFSET) & InstrMem15::INSTR_MEM15_MASK)
        }
    }

}

impl Default for InstrMem15Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl InstrMem15 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InstrMem15Val {
        InstrMem15Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: InstrMem15Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(InstrMem15Val) -> InstrMem15Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// INSTR_MEM16 Register
///
/// Write-only access to instruction memory location 16
///
/// Address offset: 0x0088
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **INSTR_MEM16** `[15:0]` (WO)
#[repr(transparent)]
pub struct InstrMem16 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for InstrMem16 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InstrMem16 {
    // ======== Field Constants ========

    /// **INSTR_MEM16**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const INSTR_MEM16_OFFSET: u32 = 0;
    pub const INSTR_MEM16_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// Write **INSTR_MEM16** `[15:0]` (WO)
    #[inline(always)]
    pub fn write_instr_mem16(&self, value: u32) {
        self.value.write_bits((value << Self::INSTR_MEM16_OFFSET) & Self::INSTR_MEM16_MASK);
    }

}

/// INSTR_MEM16 value type for batch read/modify/write operations
///
/// Use `instrmem16.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `instrmem16.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InstrMem16Val {
    bits: u32,
}

impl InstrMem16Val {
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

    /// Set **INSTR_MEM16** `[15:0]`
    #[inline(always)]
    pub const fn with_instr_mem16(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !InstrMem16::INSTR_MEM16_MASK) | ((value << InstrMem16::INSTR_MEM16_OFFSET) & InstrMem16::INSTR_MEM16_MASK)
        }
    }

}

impl Default for InstrMem16Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl InstrMem16 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InstrMem16Val {
        InstrMem16Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: InstrMem16Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(InstrMem16Val) -> InstrMem16Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// INSTR_MEM17 Register
///
/// Write-only access to instruction memory location 17
///
/// Address offset: 0x008c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **INSTR_MEM17** `[15:0]` (WO)
#[repr(transparent)]
pub struct InstrMem17 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for InstrMem17 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InstrMem17 {
    // ======== Field Constants ========

    /// **INSTR_MEM17**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const INSTR_MEM17_OFFSET: u32 = 0;
    pub const INSTR_MEM17_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// Write **INSTR_MEM17** `[15:0]` (WO)
    #[inline(always)]
    pub fn write_instr_mem17(&self, value: u32) {
        self.value.write_bits((value << Self::INSTR_MEM17_OFFSET) & Self::INSTR_MEM17_MASK);
    }

}

/// INSTR_MEM17 value type for batch read/modify/write operations
///
/// Use `instrmem17.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `instrmem17.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InstrMem17Val {
    bits: u32,
}

impl InstrMem17Val {
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

    /// Set **INSTR_MEM17** `[15:0]`
    #[inline(always)]
    pub const fn with_instr_mem17(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !InstrMem17::INSTR_MEM17_MASK) | ((value << InstrMem17::INSTR_MEM17_OFFSET) & InstrMem17::INSTR_MEM17_MASK)
        }
    }

}

impl Default for InstrMem17Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl InstrMem17 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InstrMem17Val {
        InstrMem17Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: InstrMem17Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(InstrMem17Val) -> InstrMem17Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// INSTR_MEM18 Register
///
/// Write-only access to instruction memory location 18
///
/// Address offset: 0x0090
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **INSTR_MEM18** `[15:0]` (WO)
#[repr(transparent)]
pub struct InstrMem18 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for InstrMem18 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InstrMem18 {
    // ======== Field Constants ========

    /// **INSTR_MEM18**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const INSTR_MEM18_OFFSET: u32 = 0;
    pub const INSTR_MEM18_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// Write **INSTR_MEM18** `[15:0]` (WO)
    #[inline(always)]
    pub fn write_instr_mem18(&self, value: u32) {
        self.value.write_bits((value << Self::INSTR_MEM18_OFFSET) & Self::INSTR_MEM18_MASK);
    }

}

/// INSTR_MEM18 value type for batch read/modify/write operations
///
/// Use `instrmem18.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `instrmem18.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InstrMem18Val {
    bits: u32,
}

impl InstrMem18Val {
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

    /// Set **INSTR_MEM18** `[15:0]`
    #[inline(always)]
    pub const fn with_instr_mem18(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !InstrMem18::INSTR_MEM18_MASK) | ((value << InstrMem18::INSTR_MEM18_OFFSET) & InstrMem18::INSTR_MEM18_MASK)
        }
    }

}

impl Default for InstrMem18Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl InstrMem18 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InstrMem18Val {
        InstrMem18Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: InstrMem18Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(InstrMem18Val) -> InstrMem18Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// INSTR_MEM19 Register
///
/// Write-only access to instruction memory location 19
///
/// Address offset: 0x0094
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **INSTR_MEM19** `[15:0]` (WO)
#[repr(transparent)]
pub struct InstrMem19 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for InstrMem19 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InstrMem19 {
    // ======== Field Constants ========

    /// **INSTR_MEM19**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const INSTR_MEM19_OFFSET: u32 = 0;
    pub const INSTR_MEM19_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// Write **INSTR_MEM19** `[15:0]` (WO)
    #[inline(always)]
    pub fn write_instr_mem19(&self, value: u32) {
        self.value.write_bits((value << Self::INSTR_MEM19_OFFSET) & Self::INSTR_MEM19_MASK);
    }

}

/// INSTR_MEM19 value type for batch read/modify/write operations
///
/// Use `instrmem19.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `instrmem19.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InstrMem19Val {
    bits: u32,
}

impl InstrMem19Val {
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

    /// Set **INSTR_MEM19** `[15:0]`
    #[inline(always)]
    pub const fn with_instr_mem19(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !InstrMem19::INSTR_MEM19_MASK) | ((value << InstrMem19::INSTR_MEM19_OFFSET) & InstrMem19::INSTR_MEM19_MASK)
        }
    }

}

impl Default for InstrMem19Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl InstrMem19 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InstrMem19Val {
        InstrMem19Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: InstrMem19Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(InstrMem19Val) -> InstrMem19Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// INSTR_MEM20 Register
///
/// Write-only access to instruction memory location 20
///
/// Address offset: 0x0098
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **INSTR_MEM20** `[15:0]` (WO)
#[repr(transparent)]
pub struct InstrMem20 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for InstrMem20 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InstrMem20 {
    // ======== Field Constants ========

    /// **INSTR_MEM20**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const INSTR_MEM20_OFFSET: u32 = 0;
    pub const INSTR_MEM20_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// Write **INSTR_MEM20** `[15:0]` (WO)
    #[inline(always)]
    pub fn write_instr_mem20(&self, value: u32) {
        self.value.write_bits((value << Self::INSTR_MEM20_OFFSET) & Self::INSTR_MEM20_MASK);
    }

}

/// INSTR_MEM20 value type for batch read/modify/write operations
///
/// Use `instrmem20.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `instrmem20.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InstrMem20Val {
    bits: u32,
}

impl InstrMem20Val {
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

    /// Set **INSTR_MEM20** `[15:0]`
    #[inline(always)]
    pub const fn with_instr_mem20(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !InstrMem20::INSTR_MEM20_MASK) | ((value << InstrMem20::INSTR_MEM20_OFFSET) & InstrMem20::INSTR_MEM20_MASK)
        }
    }

}

impl Default for InstrMem20Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl InstrMem20 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InstrMem20Val {
        InstrMem20Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: InstrMem20Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(InstrMem20Val) -> InstrMem20Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// INSTR_MEM21 Register
///
/// Write-only access to instruction memory location 21
///
/// Address offset: 0x009c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **INSTR_MEM21** `[15:0]` (WO)
#[repr(transparent)]
pub struct InstrMem21 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for InstrMem21 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InstrMem21 {
    // ======== Field Constants ========

    /// **INSTR_MEM21**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const INSTR_MEM21_OFFSET: u32 = 0;
    pub const INSTR_MEM21_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// Write **INSTR_MEM21** `[15:0]` (WO)
    #[inline(always)]
    pub fn write_instr_mem21(&self, value: u32) {
        self.value.write_bits((value << Self::INSTR_MEM21_OFFSET) & Self::INSTR_MEM21_MASK);
    }

}

/// INSTR_MEM21 value type for batch read/modify/write operations
///
/// Use `instrmem21.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `instrmem21.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InstrMem21Val {
    bits: u32,
}

impl InstrMem21Val {
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

    /// Set **INSTR_MEM21** `[15:0]`
    #[inline(always)]
    pub const fn with_instr_mem21(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !InstrMem21::INSTR_MEM21_MASK) | ((value << InstrMem21::INSTR_MEM21_OFFSET) & InstrMem21::INSTR_MEM21_MASK)
        }
    }

}

impl Default for InstrMem21Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl InstrMem21 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InstrMem21Val {
        InstrMem21Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: InstrMem21Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(InstrMem21Val) -> InstrMem21Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// INSTR_MEM22 Register
///
/// Write-only access to instruction memory location 22
///
/// Address offset: 0x00a0
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **INSTR_MEM22** `[15:0]` (WO)
#[repr(transparent)]
pub struct InstrMem22 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for InstrMem22 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InstrMem22 {
    // ======== Field Constants ========

    /// **INSTR_MEM22**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const INSTR_MEM22_OFFSET: u32 = 0;
    pub const INSTR_MEM22_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// Write **INSTR_MEM22** `[15:0]` (WO)
    #[inline(always)]
    pub fn write_instr_mem22(&self, value: u32) {
        self.value.write_bits((value << Self::INSTR_MEM22_OFFSET) & Self::INSTR_MEM22_MASK);
    }

}

/// INSTR_MEM22 value type for batch read/modify/write operations
///
/// Use `instrmem22.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `instrmem22.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InstrMem22Val {
    bits: u32,
}

impl InstrMem22Val {
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

    /// Set **INSTR_MEM22** `[15:0]`
    #[inline(always)]
    pub const fn with_instr_mem22(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !InstrMem22::INSTR_MEM22_MASK) | ((value << InstrMem22::INSTR_MEM22_OFFSET) & InstrMem22::INSTR_MEM22_MASK)
        }
    }

}

impl Default for InstrMem22Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl InstrMem22 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InstrMem22Val {
        InstrMem22Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: InstrMem22Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(InstrMem22Val) -> InstrMem22Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// INSTR_MEM23 Register
///
/// Write-only access to instruction memory location 23
///
/// Address offset: 0x00a4
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **INSTR_MEM23** `[15:0]` (WO)
#[repr(transparent)]
pub struct InstrMem23 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for InstrMem23 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InstrMem23 {
    // ======== Field Constants ========

    /// **INSTR_MEM23**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const INSTR_MEM23_OFFSET: u32 = 0;
    pub const INSTR_MEM23_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// Write **INSTR_MEM23** `[15:0]` (WO)
    #[inline(always)]
    pub fn write_instr_mem23(&self, value: u32) {
        self.value.write_bits((value << Self::INSTR_MEM23_OFFSET) & Self::INSTR_MEM23_MASK);
    }

}

/// INSTR_MEM23 value type for batch read/modify/write operations
///
/// Use `instrmem23.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `instrmem23.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InstrMem23Val {
    bits: u32,
}

impl InstrMem23Val {
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

    /// Set **INSTR_MEM23** `[15:0]`
    #[inline(always)]
    pub const fn with_instr_mem23(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !InstrMem23::INSTR_MEM23_MASK) | ((value << InstrMem23::INSTR_MEM23_OFFSET) & InstrMem23::INSTR_MEM23_MASK)
        }
    }

}

impl Default for InstrMem23Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl InstrMem23 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InstrMem23Val {
        InstrMem23Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: InstrMem23Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(InstrMem23Val) -> InstrMem23Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// INSTR_MEM24 Register
///
/// Write-only access to instruction memory location 24
///
/// Address offset: 0x00a8
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **INSTR_MEM24** `[15:0]` (WO)
#[repr(transparent)]
pub struct InstrMem24 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for InstrMem24 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InstrMem24 {
    // ======== Field Constants ========

    /// **INSTR_MEM24**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const INSTR_MEM24_OFFSET: u32 = 0;
    pub const INSTR_MEM24_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// Write **INSTR_MEM24** `[15:0]` (WO)
    #[inline(always)]
    pub fn write_instr_mem24(&self, value: u32) {
        self.value.write_bits((value << Self::INSTR_MEM24_OFFSET) & Self::INSTR_MEM24_MASK);
    }

}

/// INSTR_MEM24 value type for batch read/modify/write operations
///
/// Use `instrmem24.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `instrmem24.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InstrMem24Val {
    bits: u32,
}

impl InstrMem24Val {
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

    /// Set **INSTR_MEM24** `[15:0]`
    #[inline(always)]
    pub const fn with_instr_mem24(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !InstrMem24::INSTR_MEM24_MASK) | ((value << InstrMem24::INSTR_MEM24_OFFSET) & InstrMem24::INSTR_MEM24_MASK)
        }
    }

}

impl Default for InstrMem24Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl InstrMem24 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InstrMem24Val {
        InstrMem24Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: InstrMem24Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(InstrMem24Val) -> InstrMem24Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// INSTR_MEM25 Register
///
/// Write-only access to instruction memory location 25
///
/// Address offset: 0x00ac
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **INSTR_MEM25** `[15:0]` (WO)
#[repr(transparent)]
pub struct InstrMem25 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for InstrMem25 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InstrMem25 {
    // ======== Field Constants ========

    /// **INSTR_MEM25**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const INSTR_MEM25_OFFSET: u32 = 0;
    pub const INSTR_MEM25_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// Write **INSTR_MEM25** `[15:0]` (WO)
    #[inline(always)]
    pub fn write_instr_mem25(&self, value: u32) {
        self.value.write_bits((value << Self::INSTR_MEM25_OFFSET) & Self::INSTR_MEM25_MASK);
    }

}

/// INSTR_MEM25 value type for batch read/modify/write operations
///
/// Use `instrmem25.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `instrmem25.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InstrMem25Val {
    bits: u32,
}

impl InstrMem25Val {
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

    /// Set **INSTR_MEM25** `[15:0]`
    #[inline(always)]
    pub const fn with_instr_mem25(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !InstrMem25::INSTR_MEM25_MASK) | ((value << InstrMem25::INSTR_MEM25_OFFSET) & InstrMem25::INSTR_MEM25_MASK)
        }
    }

}

impl Default for InstrMem25Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl InstrMem25 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InstrMem25Val {
        InstrMem25Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: InstrMem25Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(InstrMem25Val) -> InstrMem25Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// INSTR_MEM26 Register
///
/// Write-only access to instruction memory location 26
///
/// Address offset: 0x00b0
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **INSTR_MEM26** `[15:0]` (WO)
#[repr(transparent)]
pub struct InstrMem26 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for InstrMem26 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InstrMem26 {
    // ======== Field Constants ========

    /// **INSTR_MEM26**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const INSTR_MEM26_OFFSET: u32 = 0;
    pub const INSTR_MEM26_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// Write **INSTR_MEM26** `[15:0]` (WO)
    #[inline(always)]
    pub fn write_instr_mem26(&self, value: u32) {
        self.value.write_bits((value << Self::INSTR_MEM26_OFFSET) & Self::INSTR_MEM26_MASK);
    }

}

/// INSTR_MEM26 value type for batch read/modify/write operations
///
/// Use `instrmem26.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `instrmem26.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InstrMem26Val {
    bits: u32,
}

impl InstrMem26Val {
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

    /// Set **INSTR_MEM26** `[15:0]`
    #[inline(always)]
    pub const fn with_instr_mem26(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !InstrMem26::INSTR_MEM26_MASK) | ((value << InstrMem26::INSTR_MEM26_OFFSET) & InstrMem26::INSTR_MEM26_MASK)
        }
    }

}

impl Default for InstrMem26Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl InstrMem26 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InstrMem26Val {
        InstrMem26Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: InstrMem26Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(InstrMem26Val) -> InstrMem26Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// INSTR_MEM27 Register
///
/// Write-only access to instruction memory location 27
///
/// Address offset: 0x00b4
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **INSTR_MEM27** `[15:0]` (WO)
#[repr(transparent)]
pub struct InstrMem27 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for InstrMem27 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InstrMem27 {
    // ======== Field Constants ========

    /// **INSTR_MEM27**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const INSTR_MEM27_OFFSET: u32 = 0;
    pub const INSTR_MEM27_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// Write **INSTR_MEM27** `[15:0]` (WO)
    #[inline(always)]
    pub fn write_instr_mem27(&self, value: u32) {
        self.value.write_bits((value << Self::INSTR_MEM27_OFFSET) & Self::INSTR_MEM27_MASK);
    }

}

/// INSTR_MEM27 value type for batch read/modify/write operations
///
/// Use `instrmem27.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `instrmem27.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InstrMem27Val {
    bits: u32,
}

impl InstrMem27Val {
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

    /// Set **INSTR_MEM27** `[15:0]`
    #[inline(always)]
    pub const fn with_instr_mem27(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !InstrMem27::INSTR_MEM27_MASK) | ((value << InstrMem27::INSTR_MEM27_OFFSET) & InstrMem27::INSTR_MEM27_MASK)
        }
    }

}

impl Default for InstrMem27Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl InstrMem27 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InstrMem27Val {
        InstrMem27Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: InstrMem27Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(InstrMem27Val) -> InstrMem27Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// INSTR_MEM28 Register
///
/// Write-only access to instruction memory location 28
///
/// Address offset: 0x00b8
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **INSTR_MEM28** `[15:0]` (WO)
#[repr(transparent)]
pub struct InstrMem28 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for InstrMem28 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InstrMem28 {
    // ======== Field Constants ========

    /// **INSTR_MEM28**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const INSTR_MEM28_OFFSET: u32 = 0;
    pub const INSTR_MEM28_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// Write **INSTR_MEM28** `[15:0]` (WO)
    #[inline(always)]
    pub fn write_instr_mem28(&self, value: u32) {
        self.value.write_bits((value << Self::INSTR_MEM28_OFFSET) & Self::INSTR_MEM28_MASK);
    }

}

/// INSTR_MEM28 value type for batch read/modify/write operations
///
/// Use `instrmem28.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `instrmem28.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InstrMem28Val {
    bits: u32,
}

impl InstrMem28Val {
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

    /// Set **INSTR_MEM28** `[15:0]`
    #[inline(always)]
    pub const fn with_instr_mem28(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !InstrMem28::INSTR_MEM28_MASK) | ((value << InstrMem28::INSTR_MEM28_OFFSET) & InstrMem28::INSTR_MEM28_MASK)
        }
    }

}

impl Default for InstrMem28Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl InstrMem28 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InstrMem28Val {
        InstrMem28Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: InstrMem28Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(InstrMem28Val) -> InstrMem28Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// INSTR_MEM29 Register
///
/// Write-only access to instruction memory location 29
///
/// Address offset: 0x00bc
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **INSTR_MEM29** `[15:0]` (WO)
#[repr(transparent)]
pub struct InstrMem29 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for InstrMem29 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InstrMem29 {
    // ======== Field Constants ========

    /// **INSTR_MEM29**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const INSTR_MEM29_OFFSET: u32 = 0;
    pub const INSTR_MEM29_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// Write **INSTR_MEM29** `[15:0]` (WO)
    #[inline(always)]
    pub fn write_instr_mem29(&self, value: u32) {
        self.value.write_bits((value << Self::INSTR_MEM29_OFFSET) & Self::INSTR_MEM29_MASK);
    }

}

/// INSTR_MEM29 value type for batch read/modify/write operations
///
/// Use `instrmem29.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `instrmem29.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InstrMem29Val {
    bits: u32,
}

impl InstrMem29Val {
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

    /// Set **INSTR_MEM29** `[15:0]`
    #[inline(always)]
    pub const fn with_instr_mem29(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !InstrMem29::INSTR_MEM29_MASK) | ((value << InstrMem29::INSTR_MEM29_OFFSET) & InstrMem29::INSTR_MEM29_MASK)
        }
    }

}

impl Default for InstrMem29Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl InstrMem29 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InstrMem29Val {
        InstrMem29Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: InstrMem29Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(InstrMem29Val) -> InstrMem29Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// INSTR_MEM30 Register
///
/// Write-only access to instruction memory location 30
///
/// Address offset: 0x00c0
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **INSTR_MEM30** `[15:0]` (WO)
#[repr(transparent)]
pub struct InstrMem30 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for InstrMem30 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InstrMem30 {
    // ======== Field Constants ========

    /// **INSTR_MEM30**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const INSTR_MEM30_OFFSET: u32 = 0;
    pub const INSTR_MEM30_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// Write **INSTR_MEM30** `[15:0]` (WO)
    #[inline(always)]
    pub fn write_instr_mem30(&self, value: u32) {
        self.value.write_bits((value << Self::INSTR_MEM30_OFFSET) & Self::INSTR_MEM30_MASK);
    }

}

/// INSTR_MEM30 value type for batch read/modify/write operations
///
/// Use `instrmem30.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `instrmem30.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InstrMem30Val {
    bits: u32,
}

impl InstrMem30Val {
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

    /// Set **INSTR_MEM30** `[15:0]`
    #[inline(always)]
    pub const fn with_instr_mem30(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !InstrMem30::INSTR_MEM30_MASK) | ((value << InstrMem30::INSTR_MEM30_OFFSET) & InstrMem30::INSTR_MEM30_MASK)
        }
    }

}

impl Default for InstrMem30Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl InstrMem30 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InstrMem30Val {
        InstrMem30Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: InstrMem30Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(InstrMem30Val) -> InstrMem30Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// INSTR_MEM31 Register
///
/// Write-only access to instruction memory location 31
///
/// Address offset: 0x00c4
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **INSTR_MEM31** `[15:0]` (WO)
#[repr(transparent)]
pub struct InstrMem31 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for InstrMem31 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InstrMem31 {
    // ======== Field Constants ========

    /// **INSTR_MEM31**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const INSTR_MEM31_OFFSET: u32 = 0;
    pub const INSTR_MEM31_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// Write **INSTR_MEM31** `[15:0]` (WO)
    #[inline(always)]
    pub fn write_instr_mem31(&self, value: u32) {
        self.value.write_bits((value << Self::INSTR_MEM31_OFFSET) & Self::INSTR_MEM31_MASK);
    }

}

/// INSTR_MEM31 value type for batch read/modify/write operations
///
/// Use `instrmem31.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `instrmem31.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InstrMem31Val {
    bits: u32,
}

impl InstrMem31Val {
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

    /// Set **INSTR_MEM31** `[15:0]`
    #[inline(always)]
    pub const fn with_instr_mem31(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !InstrMem31::INSTR_MEM31_MASK) | ((value << InstrMem31::INSTR_MEM31_OFFSET) & InstrMem31::INSTR_MEM31_MASK)
        }
    }

}

impl Default for InstrMem31Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl InstrMem31 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InstrMem31Val {
        InstrMem31Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: InstrMem31Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(InstrMem31Val) -> InstrMem31Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SM0_CLKDIV Register
///
/// Clock divisor register for state machine 0
/// Frequency = clock freq / (CLKDIV_INT + CLKDIV_FRAC / 256)
///
/// Address offset: 0x00c8
/// Reset value: 0x00010000
///
/// ## Fields
///
/// - **INT** `[31:16]` (RW)
///
///   Effective frequency is sysclk/(int + frac/256).
///   Value of 0 is interpreted as 65536. If INT is 0, FRAC must also be 0.
///
/// - **FRAC** `[15:8]` (RW)
///
///   Fractional part of clock divisor
#[repr(transparent)]
pub struct Sm0Clkdiv {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Sm0Clkdiv {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sm0Clkdiv {
    // ======== Field Constants ========

    /// **INT**
    ///
    /// Effective frequency is sysclk/(int + frac/256).
    /// Value of 0 is interpreted as 65536. If INT is 0, FRAC must also be 0.
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const INT_OFFSET: u32 = 16;
    pub const INT_MASK: u32 = 0xffff0000;

    /// **FRAC**
    ///
    /// Fractional part of clock divisor
    ///
    /// Bits `[15:8]`, mask `0x0000ff00`
    pub const FRAC_OFFSET: u32 = 8;
    pub const FRAC_MASK: u32 = 0x0000ff00;

    // ======== Field Accessors ========

    /// **INT** `[31:16]` (RW)
    ///
    /// Effective frequency is sysclk/(int + frac/256).
    /// Value of 0 is interpreted as 65536. If INT is 0, FRAC must also be 0.
    #[inline(always)]
    pub fn int(&self) -> u32 {
        (self.value.read_bits() & Self::INT_MASK) >> Self::INT_OFFSET
    }

    /// Write **INT** `[31:16]` (RW)
    ///
    /// Effective frequency is sysclk/(int + frac/256).
    /// Value of 0 is interpreted as 65536. If INT is 0, FRAC must also be 0.
    #[inline(always)]
    pub fn write_int(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::INT_MASK) | ((value << Self::INT_OFFSET) & Self::INT_MASK));
    }

    /// **FRAC** `[15:8]` (RW)
    ///
    /// Fractional part of clock divisor
    #[inline(always)]
    pub fn frac(&self) -> u32 {
        (self.value.read_bits() & Self::FRAC_MASK) >> Self::FRAC_OFFSET
    }

    /// Write **FRAC** `[15:8]` (RW)
    ///
    /// Fractional part of clock divisor
    #[inline(always)]
    pub fn write_frac(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::FRAC_MASK) | ((value << Self::FRAC_OFFSET) & Self::FRAC_MASK));
    }

}

/// SM0_CLKDIV value type for batch read/modify/write operations
///
/// Use `sm0clkdiv.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sm0clkdiv.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Sm0ClkdivVal {
    bits: u32,
}

impl Sm0ClkdivVal {
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
        Self { bits: 0x00010000 }
    }

    /// Read **INT** `[31:16]`
    ///
    /// Effective frequency is sysclk/(int + frac/256).
    /// Value of 0 is interpreted as 65536. If INT is 0, FRAC must also be 0.
    #[inline(always)]
    pub const fn int(&self) -> u32 {
        (self.bits & Sm0Clkdiv::INT_MASK) >> Sm0Clkdiv::INT_OFFSET
    }

    /// Read **FRAC** `[15:8]`
    ///
    /// Fractional part of clock divisor
    #[inline(always)]
    pub const fn frac(&self) -> u32 {
        (self.bits & Sm0Clkdiv::FRAC_MASK) >> Sm0Clkdiv::FRAC_OFFSET
    }

    /// Set **INT** `[31:16]`
    ///
    /// Effective frequency is sysclk/(int + frac/256).
    /// Value of 0 is interpreted as 65536. If INT is 0, FRAC must also be 0.
    #[inline(always)]
    pub const fn with_int(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sm0Clkdiv::INT_MASK) | ((value << Sm0Clkdiv::INT_OFFSET) & Sm0Clkdiv::INT_MASK)
        }
    }

    /// Set **FRAC** `[15:8]`
    ///
    /// Fractional part of clock divisor
    #[inline(always)]
    pub const fn with_frac(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sm0Clkdiv::FRAC_MASK) | ((value << Sm0Clkdiv::FRAC_OFFSET) & Sm0Clkdiv::FRAC_MASK)
        }
    }

}

impl Default for Sm0ClkdivVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Sm0Clkdiv {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Sm0ClkdivVal {
        Sm0ClkdivVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Sm0ClkdivVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Sm0ClkdivVal) -> Sm0ClkdivVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SM0_EXECCTRL Register
///
/// Execution/behavioural settings for state machine 0
///
/// Address offset: 0x00cc
/// Reset value: 0x0001f000
///
/// ## Fields
///
/// - **EXEC_STALLED** `[31]` (RO)
///
///   If 1, an instruction written to SMx_INSTR is stalled, and latched by the state machine. Will clear to 0 once this instruction completes.
///
/// - **SIDE_EN** `[30]` (RW)
///
///   If 1, the MSB of the Delay/Side-set instruction field is used as side-set enable, rather than a side-set data bit. This allows instructions to perform side-set optionally, rather than on every instruction, but the maximum possible side-set width is reduced from 5 to 4. Note that the value of PINCTRL_SIDESET_COUNT is inclusive of this enable bit.
///
/// - **SIDE_PINDIR** `[29]` (RW)
///
///   If 1, side-set data is asserted to pin directions, instead of pin values
///
/// - **JMP_PIN** `[28:24]` (RW)
///
///   The GPIO number to use as condition for JMP PIN. Unaffected by input mapping.
///
/// - **OUT_EN_SEL** `[23:19]` (RW)
///
///   Which data bit to use for inline OUT enable
///
/// - **INLINE_OUT_EN** `[18]` (RW)
///
///   If 1, use a bit of OUT data as an auxiliary write enable
///   When used in conjunction with OUT_STICKY, writes with an enable of 0 will
///   deassert the latest pin write. This can create useful masking/override behaviour
///   due to the priority ordering of state machine pin writes (SM0 < SM1 < ...)
///
/// - **OUT_STICKY** `[17]` (RW)
///
///   Continuously assert the most recent OUT/SET to the pins
///
/// - **WRAP_TOP** `[16:12]` (RW)
///
///   After reaching this address, execution is wrapped to wrap_bottom.
///   If the instruction is a jump, and the jump condition is true, the jump takes priority.
///
/// - **WRAP_BOTTOM** `[11:7]` (RW)
///
///   After reaching wrap_top, execution is wrapped to this address.
///
/// - **STATUS_SEL** `[6:5]` (RW)
///
///   Comparison used for the MOV x, STATUS instruction.
///
/// - **STATUS_N** `[4:0]` (RW)
///
///   Comparison level or IRQ index for the MOV x, STATUS instruction.
///   If STATUS_SEL is TXLEVEL or RXLEVEL, then values of STATUS_N greater than the current FIFO depth are reserved, and have undefined behaviour.
#[repr(transparent)]
pub struct Sm0Execctrl {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Sm0Execctrl {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sm0Execctrl {
    // ======== Field Constants ========

    /// **EXEC_STALLED**
    ///
    /// If 1, an instruction written to SMx_INSTR is stalled, and latched by the state machine. Will clear to 0 once this instruction completes.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const EXEC_STALLED_OFFSET: u32 = 31;
    pub const EXEC_STALLED_MASK: u32 = 0x80000000;
    pub const EXEC_STALLED: u32 = 1 << 31;

    /// **SIDE_EN**
    ///
    /// If 1, the MSB of the Delay/Side-set instruction field is used as side-set enable, rather than a side-set data bit. This allows instructions to perform side-set optionally, rather than on every instruction, but the maximum possible side-set width is reduced from 5 to 4. Note that the value of PINCTRL_SIDESET_COUNT is inclusive of this enable bit.
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const SIDE_EN_OFFSET: u32 = 30;
    pub const SIDE_EN_MASK: u32 = 0x40000000;
    pub const SIDE_EN: u32 = 1 << 30;

    /// **SIDE_PINDIR**
    ///
    /// If 1, side-set data is asserted to pin directions, instead of pin values
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const SIDE_PINDIR_OFFSET: u32 = 29;
    pub const SIDE_PINDIR_MASK: u32 = 0x20000000;
    pub const SIDE_PINDIR: u32 = 1 << 29;

    /// **JMP_PIN**
    ///
    /// The GPIO number to use as condition for JMP PIN. Unaffected by input mapping.
    ///
    /// Bits `[28:24]`, mask `0x1f000000`
    pub const JMP_PIN_OFFSET: u32 = 24;
    pub const JMP_PIN_MASK: u32 = 0x1f000000;

    /// **OUT_EN_SEL**
    ///
    /// Which data bit to use for inline OUT enable
    ///
    /// Bits `[23:19]`, mask `0x00f80000`
    pub const OUT_EN_SEL_OFFSET: u32 = 19;
    pub const OUT_EN_SEL_MASK: u32 = 0x00f80000;

    /// **INLINE_OUT_EN**
    ///
    /// If 1, use a bit of OUT data as an auxiliary write enable
    /// When used in conjunction with OUT_STICKY, writes with an enable of 0 will
    /// deassert the latest pin write. This can create useful masking/override behaviour
    /// due to the priority ordering of state machine pin writes (SM0 < SM1 < ...)
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const INLINE_OUT_EN_OFFSET: u32 = 18;
    pub const INLINE_OUT_EN_MASK: u32 = 0x00040000;
    pub const INLINE_OUT_EN: u32 = 1 << 18;

    /// **OUT_STICKY**
    ///
    /// Continuously assert the most recent OUT/SET to the pins
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const OUT_STICKY_OFFSET: u32 = 17;
    pub const OUT_STICKY_MASK: u32 = 0x00020000;
    pub const OUT_STICKY: u32 = 1 << 17;

    /// **WRAP_TOP**
    ///
    /// After reaching this address, execution is wrapped to wrap_bottom.
    /// If the instruction is a jump, and the jump condition is true, the jump takes priority.
    ///
    /// Bits `[16:12]`, mask `0x0001f000`
    pub const WRAP_TOP_OFFSET: u32 = 12;
    pub const WRAP_TOP_MASK: u32 = 0x0001f000;

    /// **WRAP_BOTTOM**
    ///
    /// After reaching wrap_top, execution is wrapped to this address.
    ///
    /// Bits `[11:7]`, mask `0x00000f80`
    pub const WRAP_BOTTOM_OFFSET: u32 = 7;
    pub const WRAP_BOTTOM_MASK: u32 = 0x00000f80;

    /// **STATUS_SEL**
    ///
    /// Comparison used for the MOV x, STATUS instruction.
    ///
    /// Bits `[6:5]`, mask `0x00000060`
    pub const STATUS_SEL_OFFSET: u32 = 5;
    pub const STATUS_SEL_MASK: u32 = 0x00000060;
    /// TXLEVEL = 0
    /// All-ones if TX FIFO level < N, otherwise all-zeroes
    pub const STATUS_SEL_TXLEVEL: u32 = 0 << 5;
    pub const STATUS_SEL_TXLEVEL_VAL: u32 = 0;
    /// RXLEVEL = 1
    /// All-ones if RX FIFO level < N, otherwise all-zeroes
    pub const STATUS_SEL_RXLEVEL: u32 = 1 << 5;
    pub const STATUS_SEL_RXLEVEL_VAL: u32 = 1;
    /// IRQ = 2
    /// All-ones if the indexed IRQ flag is raised, otherwise all-zeroes
    pub const STATUS_SEL_IRQ: u32 = 2 << 5;
    pub const STATUS_SEL_IRQ_VAL: u32 = 2;

    /// **STATUS_N**
    ///
    /// Comparison level or IRQ index for the MOV x, STATUS instruction.
    /// If STATUS_SEL is TXLEVEL or RXLEVEL, then values of STATUS_N greater than the current FIFO depth are reserved, and have undefined behaviour.
    ///
    /// Bits `[4:0]`, mask `0x0000001f`
    pub const STATUS_N_OFFSET: u32 = 0;
    pub const STATUS_N_MASK: u32 = 0x0000001f;
    /// IRQ = 0
    /// Index 0-7 of an IRQ flag in this PIO block
    pub const STATUS_N_IRQ: u32 = 0 << 0;
    pub const STATUS_N_IRQ_VAL: u32 = 0;
    /// IRQ_PREVPIO = 8
    /// Index 0-7 of an IRQ flag in the next lower-numbered PIO block
    pub const STATUS_N_IRQ_PREVPIO: u32 = 8 << 0;
    pub const STATUS_N_IRQ_PREVPIO_VAL: u32 = 8;
    /// IRQ_NEXTPIO = 16
    /// Index 0-7 of an IRQ flag in the next higher-numbered PIO block
    pub const STATUS_N_IRQ_NEXTPIO: u32 = 16 << 0;
    pub const STATUS_N_IRQ_NEXTPIO_VAL: u32 = 16;

    // ======== Field Accessors ========

    /// **EXEC_STALLED** `[31]` (RO)
    ///
    /// If 1, an instruction written to SMx_INSTR is stalled, and latched by the state machine. Will clear to 0 once this instruction completes.
    #[inline(always)]
    pub fn exec_stalled(&self) -> bool {
        (self.value.read_bits() & Self::EXEC_STALLED_MASK) != 0
    }

    /// **SIDE_EN** `[30]` (RW)
    ///
    /// If 1, the MSB of the Delay/Side-set instruction field is used as side-set enable, rather than a side-set data bit. This allows instructions to perform side-set optionally, rather than on every instruction, but the maximum possible side-set width is reduced from 5 to 4. Note that the value of PINCTRL_SIDESET_COUNT is inclusive of this enable bit.
    #[inline(always)]
    pub fn side_en(&self) -> bool {
        (self.value.read_bits() & Self::SIDE_EN_MASK) != 0
    }

    /// Set SIDE_EN bit atomically (RW)
    #[inline(always)]
    pub fn set_side_en(&self) {
        self.set_bits(Self::SIDE_EN_MASK);
    }

    /// Clear SIDE_EN bit atomically (RW)
    #[inline(always)]
    pub fn clr_side_en(&self) {
        self.clr_bits(Self::SIDE_EN_MASK);
    }

    /// **SIDE_PINDIR** `[29]` (RW)
    ///
    /// If 1, side-set data is asserted to pin directions, instead of pin values
    #[inline(always)]
    pub fn side_pindir(&self) -> bool {
        (self.value.read_bits() & Self::SIDE_PINDIR_MASK) != 0
    }

    /// Set SIDE_PINDIR bit atomically (RW)
    #[inline(always)]
    pub fn set_side_pindir(&self) {
        self.set_bits(Self::SIDE_PINDIR_MASK);
    }

    /// Clear SIDE_PINDIR bit atomically (RW)
    #[inline(always)]
    pub fn clr_side_pindir(&self) {
        self.clr_bits(Self::SIDE_PINDIR_MASK);
    }

    /// **JMP_PIN** `[28:24]` (RW)
    ///
    /// The GPIO number to use as condition for JMP PIN. Unaffected by input mapping.
    #[inline(always)]
    pub fn jmp_pin(&self) -> u32 {
        (self.value.read_bits() & Self::JMP_PIN_MASK) >> Self::JMP_PIN_OFFSET
    }

    /// Write **JMP_PIN** `[28:24]` (RW)
    ///
    /// The GPIO number to use as condition for JMP PIN. Unaffected by input mapping.
    #[inline(always)]
    pub fn write_jmp_pin(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::JMP_PIN_MASK) | ((value << Self::JMP_PIN_OFFSET) & Self::JMP_PIN_MASK));
    }

    /// **OUT_EN_SEL** `[23:19]` (RW)
    ///
    /// Which data bit to use for inline OUT enable
    #[inline(always)]
    pub fn out_en_sel(&self) -> u32 {
        (self.value.read_bits() & Self::OUT_EN_SEL_MASK) >> Self::OUT_EN_SEL_OFFSET
    }

    /// Write **OUT_EN_SEL** `[23:19]` (RW)
    ///
    /// Which data bit to use for inline OUT enable
    #[inline(always)]
    pub fn write_out_en_sel(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::OUT_EN_SEL_MASK) | ((value << Self::OUT_EN_SEL_OFFSET) & Self::OUT_EN_SEL_MASK));
    }

    /// **INLINE_OUT_EN** `[18]` (RW)
    ///
    /// If 1, use a bit of OUT data as an auxiliary write enable
    /// When used in conjunction with OUT_STICKY, writes with an enable of 0 will
    /// deassert the latest pin write. This can create useful masking/override behaviour
    /// due to the priority ordering of state machine pin writes (SM0 < SM1 < ...)
    #[inline(always)]
    pub fn inline_out_en(&self) -> bool {
        (self.value.read_bits() & Self::INLINE_OUT_EN_MASK) != 0
    }

    /// Set INLINE_OUT_EN bit atomically (RW)
    #[inline(always)]
    pub fn set_inline_out_en(&self) {
        self.set_bits(Self::INLINE_OUT_EN_MASK);
    }

    /// Clear INLINE_OUT_EN bit atomically (RW)
    #[inline(always)]
    pub fn clr_inline_out_en(&self) {
        self.clr_bits(Self::INLINE_OUT_EN_MASK);
    }

    /// **OUT_STICKY** `[17]` (RW)
    ///
    /// Continuously assert the most recent OUT/SET to the pins
    #[inline(always)]
    pub fn out_sticky(&self) -> bool {
        (self.value.read_bits() & Self::OUT_STICKY_MASK) != 0
    }

    /// Set OUT_STICKY bit atomically (RW)
    #[inline(always)]
    pub fn set_out_sticky(&self) {
        self.set_bits(Self::OUT_STICKY_MASK);
    }

    /// Clear OUT_STICKY bit atomically (RW)
    #[inline(always)]
    pub fn clr_out_sticky(&self) {
        self.clr_bits(Self::OUT_STICKY_MASK);
    }

    /// **WRAP_TOP** `[16:12]` (RW)
    ///
    /// After reaching this address, execution is wrapped to wrap_bottom.
    /// If the instruction is a jump, and the jump condition is true, the jump takes priority.
    #[inline(always)]
    pub fn wrap_top(&self) -> u32 {
        (self.value.read_bits() & Self::WRAP_TOP_MASK) >> Self::WRAP_TOP_OFFSET
    }

    /// Write **WRAP_TOP** `[16:12]` (RW)
    ///
    /// After reaching this address, execution is wrapped to wrap_bottom.
    /// If the instruction is a jump, and the jump condition is true, the jump takes priority.
    #[inline(always)]
    pub fn write_wrap_top(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::WRAP_TOP_MASK) | ((value << Self::WRAP_TOP_OFFSET) & Self::WRAP_TOP_MASK));
    }

    /// **WRAP_BOTTOM** `[11:7]` (RW)
    ///
    /// After reaching wrap_top, execution is wrapped to this address.
    #[inline(always)]
    pub fn wrap_bottom(&self) -> u32 {
        (self.value.read_bits() & Self::WRAP_BOTTOM_MASK) >> Self::WRAP_BOTTOM_OFFSET
    }

    /// Write **WRAP_BOTTOM** `[11:7]` (RW)
    ///
    /// After reaching wrap_top, execution is wrapped to this address.
    #[inline(always)]
    pub fn write_wrap_bottom(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::WRAP_BOTTOM_MASK) | ((value << Self::WRAP_BOTTOM_OFFSET) & Self::WRAP_BOTTOM_MASK));
    }

    /// **STATUS_SEL** `[6:5]` (RW)
    ///
    /// Comparison used for the MOV x, STATUS instruction.
    #[inline(always)]
    pub fn status_sel(&self) -> u32 {
        (self.value.read_bits() & Self::STATUS_SEL_MASK) >> Self::STATUS_SEL_OFFSET
    }

    /// Write **STATUS_SEL** `[6:5]` (RW)
    ///
    /// Comparison used for the MOV x, STATUS instruction.
    #[inline(always)]
    pub fn write_status_sel(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::STATUS_SEL_MASK) | ((value << Self::STATUS_SEL_OFFSET) & Self::STATUS_SEL_MASK));
    }

    /// **STATUS_N** `[4:0]` (RW)
    ///
    /// Comparison level or IRQ index for the MOV x, STATUS instruction.
    /// If STATUS_SEL is TXLEVEL or RXLEVEL, then values of STATUS_N greater than the current FIFO depth are reserved, and have undefined behaviour.
    #[inline(always)]
    pub fn status_n(&self) -> u32 {
        (self.value.read_bits() & Self::STATUS_N_MASK) >> Self::STATUS_N_OFFSET
    }

    /// Write **STATUS_N** `[4:0]` (RW)
    ///
    /// Comparison level or IRQ index for the MOV x, STATUS instruction.
    /// If STATUS_SEL is TXLEVEL or RXLEVEL, then values of STATUS_N greater than the current FIFO depth are reserved, and have undefined behaviour.
    #[inline(always)]
    pub fn write_status_n(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::STATUS_N_MASK) | ((value << Self::STATUS_N_OFFSET) & Self::STATUS_N_MASK));
    }

}

/// SM0_EXECCTRL value type for batch read/modify/write operations
///
/// Use `sm0execctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sm0execctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Sm0ExecctrlVal {
    bits: u32,
}

impl Sm0ExecctrlVal {
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
        Self { bits: 0x0001f000 }
    }

    /// Read **EXEC_STALLED** `[31:31]`
    ///
    /// If 1, an instruction written to SMx_INSTR is stalled, and latched by the state machine. Will clear to 0 once this instruction completes.
    #[inline(always)]
    pub const fn exec_stalled(&self) -> bool {
        (self.bits & Sm0Execctrl::EXEC_STALLED_MASK) != 0
    }

    /// Read **SIDE_EN** `[30:30]`
    ///
    /// If 1, the MSB of the Delay/Side-set instruction field is used as side-set enable, rather than a side-set data bit. This allows instructions to perform side-set optionally, rather than on every instruction, but the maximum possible side-set width is reduced from 5 to 4. Note that the value of PINCTRL_SIDESET_COUNT is inclusive of this enable bit.
    #[inline(always)]
    pub const fn side_en(&self) -> bool {
        (self.bits & Sm0Execctrl::SIDE_EN_MASK) != 0
    }

    /// Read **SIDE_PINDIR** `[29:29]`
    ///
    /// If 1, side-set data is asserted to pin directions, instead of pin values
    #[inline(always)]
    pub const fn side_pindir(&self) -> bool {
        (self.bits & Sm0Execctrl::SIDE_PINDIR_MASK) != 0
    }

    /// Read **JMP_PIN** `[28:24]`
    ///
    /// The GPIO number to use as condition for JMP PIN. Unaffected by input mapping.
    #[inline(always)]
    pub const fn jmp_pin(&self) -> u32 {
        (self.bits & Sm0Execctrl::JMP_PIN_MASK) >> Sm0Execctrl::JMP_PIN_OFFSET
    }

    /// Read **OUT_EN_SEL** `[23:19]`
    ///
    /// Which data bit to use for inline OUT enable
    #[inline(always)]
    pub const fn out_en_sel(&self) -> u32 {
        (self.bits & Sm0Execctrl::OUT_EN_SEL_MASK) >> Sm0Execctrl::OUT_EN_SEL_OFFSET
    }

    /// Read **INLINE_OUT_EN** `[18:18]`
    ///
    /// If 1, use a bit of OUT data as an auxiliary write enable
    /// When used in conjunction with OUT_STICKY, writes with an enable of 0 will
    /// deassert the latest pin write. This can create useful masking/override behaviour
    /// due to the priority ordering of state machine pin writes (SM0 < SM1 < ...)
    #[inline(always)]
    pub const fn inline_out_en(&self) -> bool {
        (self.bits & Sm0Execctrl::INLINE_OUT_EN_MASK) != 0
    }

    /// Read **OUT_STICKY** `[17:17]`
    ///
    /// Continuously assert the most recent OUT/SET to the pins
    #[inline(always)]
    pub const fn out_sticky(&self) -> bool {
        (self.bits & Sm0Execctrl::OUT_STICKY_MASK) != 0
    }

    /// Read **WRAP_TOP** `[16:12]`
    ///
    /// After reaching this address, execution is wrapped to wrap_bottom.
    /// If the instruction is a jump, and the jump condition is true, the jump takes priority.
    #[inline(always)]
    pub const fn wrap_top(&self) -> u32 {
        (self.bits & Sm0Execctrl::WRAP_TOP_MASK) >> Sm0Execctrl::WRAP_TOP_OFFSET
    }

    /// Read **WRAP_BOTTOM** `[11:7]`
    ///
    /// After reaching wrap_top, execution is wrapped to this address.
    #[inline(always)]
    pub const fn wrap_bottom(&self) -> u32 {
        (self.bits & Sm0Execctrl::WRAP_BOTTOM_MASK) >> Sm0Execctrl::WRAP_BOTTOM_OFFSET
    }

    /// Read **STATUS_SEL** `[6:5]`
    ///
    /// Comparison used for the MOV x, STATUS instruction.
    #[inline(always)]
    pub const fn status_sel(&self) -> u32 {
        (self.bits & Sm0Execctrl::STATUS_SEL_MASK) >> Sm0Execctrl::STATUS_SEL_OFFSET
    }

    /// Read **STATUS_N** `[4:0]`
    ///
    /// Comparison level or IRQ index for the MOV x, STATUS instruction.
    /// If STATUS_SEL is TXLEVEL or RXLEVEL, then values of STATUS_N greater than the current FIFO depth are reserved, and have undefined behaviour.
    #[inline(always)]
    pub const fn status_n(&self) -> u32 {
        (self.bits & Sm0Execctrl::STATUS_N_MASK) >> Sm0Execctrl::STATUS_N_OFFSET
    }

    /// Set **SIDE_EN** `[30:30]`
    ///
    /// If 1, the MSB of the Delay/Side-set instruction field is used as side-set enable, rather than a side-set data bit. This allows instructions to perform side-set optionally, rather than on every instruction, but the maximum possible side-set width is reduced from 5 to 4. Note that the value of PINCTRL_SIDESET_COUNT is inclusive of this enable bit.
    #[inline(always)]
    pub const fn with_side_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sm0Execctrl::SIDE_EN_MASK
            } else {
                self.bits & !Sm0Execctrl::SIDE_EN_MASK
            }
        }
    }

    /// Set **SIDE_PINDIR** `[29:29]`
    ///
    /// If 1, side-set data is asserted to pin directions, instead of pin values
    #[inline(always)]
    pub const fn with_side_pindir(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sm0Execctrl::SIDE_PINDIR_MASK
            } else {
                self.bits & !Sm0Execctrl::SIDE_PINDIR_MASK
            }
        }
    }

    /// Set **JMP_PIN** `[28:24]`
    ///
    /// The GPIO number to use as condition for JMP PIN. Unaffected by input mapping.
    #[inline(always)]
    pub const fn with_jmp_pin(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sm0Execctrl::JMP_PIN_MASK) | ((value << Sm0Execctrl::JMP_PIN_OFFSET) & Sm0Execctrl::JMP_PIN_MASK)
        }
    }

    /// Set **OUT_EN_SEL** `[23:19]`
    ///
    /// Which data bit to use for inline OUT enable
    #[inline(always)]
    pub const fn with_out_en_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sm0Execctrl::OUT_EN_SEL_MASK) | ((value << Sm0Execctrl::OUT_EN_SEL_OFFSET) & Sm0Execctrl::OUT_EN_SEL_MASK)
        }
    }

    /// Set **INLINE_OUT_EN** `[18:18]`
    ///
    /// If 1, use a bit of OUT data as an auxiliary write enable
    /// When used in conjunction with OUT_STICKY, writes with an enable of 0 will
    /// deassert the latest pin write. This can create useful masking/override behaviour
    /// due to the priority ordering of state machine pin writes (SM0 < SM1 < ...)
    #[inline(always)]
    pub const fn with_inline_out_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sm0Execctrl::INLINE_OUT_EN_MASK
            } else {
                self.bits & !Sm0Execctrl::INLINE_OUT_EN_MASK
            }
        }
    }

    /// Set **OUT_STICKY** `[17:17]`
    ///
    /// Continuously assert the most recent OUT/SET to the pins
    #[inline(always)]
    pub const fn with_out_sticky(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sm0Execctrl::OUT_STICKY_MASK
            } else {
                self.bits & !Sm0Execctrl::OUT_STICKY_MASK
            }
        }
    }

    /// Set **WRAP_TOP** `[16:12]`
    ///
    /// After reaching this address, execution is wrapped to wrap_bottom.
    /// If the instruction is a jump, and the jump condition is true, the jump takes priority.
    #[inline(always)]
    pub const fn with_wrap_top(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sm0Execctrl::WRAP_TOP_MASK) | ((value << Sm0Execctrl::WRAP_TOP_OFFSET) & Sm0Execctrl::WRAP_TOP_MASK)
        }
    }

    /// Set **WRAP_BOTTOM** `[11:7]`
    ///
    /// After reaching wrap_top, execution is wrapped to this address.
    #[inline(always)]
    pub const fn with_wrap_bottom(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sm0Execctrl::WRAP_BOTTOM_MASK) | ((value << Sm0Execctrl::WRAP_BOTTOM_OFFSET) & Sm0Execctrl::WRAP_BOTTOM_MASK)
        }
    }

    /// Set **STATUS_SEL** `[6:5]`
    ///
    /// Comparison used for the MOV x, STATUS instruction.
    #[inline(always)]
    pub const fn with_status_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sm0Execctrl::STATUS_SEL_MASK) | ((value << Sm0Execctrl::STATUS_SEL_OFFSET) & Sm0Execctrl::STATUS_SEL_MASK)
        }
    }

    /// Set **STATUS_N** `[4:0]`
    ///
    /// Comparison level or IRQ index for the MOV x, STATUS instruction.
    /// If STATUS_SEL is TXLEVEL or RXLEVEL, then values of STATUS_N greater than the current FIFO depth are reserved, and have undefined behaviour.
    #[inline(always)]
    pub const fn with_status_n(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sm0Execctrl::STATUS_N_MASK) | ((value << Sm0Execctrl::STATUS_N_OFFSET) & Sm0Execctrl::STATUS_N_MASK)
        }
    }

}

impl Default for Sm0ExecctrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Sm0Execctrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Sm0ExecctrlVal {
        Sm0ExecctrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Sm0ExecctrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Sm0ExecctrlVal) -> Sm0ExecctrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SM0_SHIFTCTRL Register
///
/// Control behaviour of the input/output shift registers for state machine 0
///
/// Address offset: 0x00d0
/// Reset value: 0x000c0000
///
/// ## Fields
///
/// - **FJOIN_RX** `[31]` (RW)
///
///   When 1, RX FIFO steals the TX FIFO's storage, and becomes twice as deep.
///   TX FIFO is disabled as a result (always reads as both full and empty).
///   FIFOs are flushed when this bit is changed.
///
/// - **FJOIN_TX** `[30]` (RW)
///
///   When 1, TX FIFO steals the RX FIFO's storage, and becomes twice as deep.
///   RX FIFO is disabled as a result (always reads as both full and empty).
///   FIFOs are flushed when this bit is changed.
///
/// - **PULL_THRESH** `[29:25]` (RW)
///
///   Number of bits shifted out of OSR before autopull, or conditional pull (PULL IFEMPTY), will take place.
///   Write 0 for value of 32.
///
/// - **PUSH_THRESH** `[24:20]` (RW)
///
///   Number of bits shifted into ISR before autopush, or conditional push (PUSH IFFULL), will take place.
///   Write 0 for value of 32.
///
/// - **OUT_SHIFTDIR** `[19]` (RW)
///
///   1 = shift out of output shift register to right. 0 = to left.
///
/// - **IN_SHIFTDIR** `[18]` (RW)
///
///   1 = shift input shift register to right (data enters from left). 0 = to left.
///
/// - **AUTOPULL** `[17]` (RW)
///
///   Pull automatically when the output shift register is emptied, i.e. on or following an OUT instruction which causes the output shift counter to reach or exceed PULL_THRESH.
///
/// - **AUTOPUSH** `[16]` (RW)
///
///   Push automatically when the input shift register is filled, i.e. on an IN instruction which causes the input shift counter to reach or exceed PUSH_THRESH.
///
/// - **FJOIN_RX_PUT** `[15]` (RW)
///
///   If 1, disable this state machine's RX FIFO, make its storage available for random write access by the state machine (using the `put` instruction) and, unless FJOIN_RX_GET is also set, random read access by the processor (through the RXFx_PUTGETy registers).
///   If FJOIN_RX_PUT and FJOIN_RX_GET are both set, then the RX FIFO's registers can be randomly read/written by the state machine, but are completely inaccessible to the processor.
///   Setting this bit will clear the FJOIN_TX and FJOIN_RX bits.
///
/// - **FJOIN_RX_GET** `[14]` (RW)
///
///   If 1, disable this state machine's RX FIFO, make its storage available for random read access by the state machine (using the `get` instruction) and, unless FJOIN_RX_PUT is also set, random write access by the processor (through the RXFx_PUTGETy registers).
///   If FJOIN_RX_PUT and FJOIN_RX_GET are both set, then the RX FIFO's registers can be randomly read/written by the state machine, but are completely inaccessible to the processor.
///   Setting this bit will clear the FJOIN_TX and FJOIN_RX bits.
///
/// - **IN_COUNT** `[4:0]` (RW)
///
///   Set the number of pins which are not masked to 0 when read by an IN PINS, WAIT PIN or MOV x, PINS instruction.
///   For example, an IN_COUNT of 5 means that the 5 LSBs of the IN pin group are visible (bits 4:0), but the remaining 27 MSBs are masked to 0. A count of 32 is encoded with a field value of 0, so the default behaviour is to not perform any masking.
///   Note this masking is applied in addition to the masking usually performed by the IN instruction. This is mainly useful for the MOV x, PINS instruction, which otherwise has no way of masking pins.
#[repr(transparent)]
pub struct Sm0Shiftctrl {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Sm0Shiftctrl {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sm0Shiftctrl {
    // ======== Field Constants ========

    /// **FJOIN_RX**
    ///
    /// When 1, RX FIFO steals the TX FIFO's storage, and becomes twice as deep.
    /// TX FIFO is disabled as a result (always reads as both full and empty).
    /// FIFOs are flushed when this bit is changed.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const FJOIN_RX_OFFSET: u32 = 31;
    pub const FJOIN_RX_MASK: u32 = 0x80000000;
    pub const FJOIN_RX: u32 = 1 << 31;

    /// **FJOIN_TX**
    ///
    /// When 1, TX FIFO steals the RX FIFO's storage, and becomes twice as deep.
    /// RX FIFO is disabled as a result (always reads as both full and empty).
    /// FIFOs are flushed when this bit is changed.
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const FJOIN_TX_OFFSET: u32 = 30;
    pub const FJOIN_TX_MASK: u32 = 0x40000000;
    pub const FJOIN_TX: u32 = 1 << 30;

    /// **PULL_THRESH**
    ///
    /// Number of bits shifted out of OSR before autopull, or conditional pull (PULL IFEMPTY), will take place.
    /// Write 0 for value of 32.
    ///
    /// Bits `[29:25]`, mask `0x3e000000`
    pub const PULL_THRESH_OFFSET: u32 = 25;
    pub const PULL_THRESH_MASK: u32 = 0x3e000000;

    /// **PUSH_THRESH**
    ///
    /// Number of bits shifted into ISR before autopush, or conditional push (PUSH IFFULL), will take place.
    /// Write 0 for value of 32.
    ///
    /// Bits `[24:20]`, mask `0x01f00000`
    pub const PUSH_THRESH_OFFSET: u32 = 20;
    pub const PUSH_THRESH_MASK: u32 = 0x01f00000;

    /// **OUT_SHIFTDIR**
    ///
    /// 1 = shift out of output shift register to right. 0 = to left.
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const OUT_SHIFTDIR_OFFSET: u32 = 19;
    pub const OUT_SHIFTDIR_MASK: u32 = 0x00080000;
    pub const OUT_SHIFTDIR: u32 = 1 << 19;

    /// **IN_SHIFTDIR**
    ///
    /// 1 = shift input shift register to right (data enters from left). 0 = to left.
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const IN_SHIFTDIR_OFFSET: u32 = 18;
    pub const IN_SHIFTDIR_MASK: u32 = 0x00040000;
    pub const IN_SHIFTDIR: u32 = 1 << 18;

    /// **AUTOPULL**
    ///
    /// Pull automatically when the output shift register is emptied, i.e. on or following an OUT instruction which causes the output shift counter to reach or exceed PULL_THRESH.
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const AUTOPULL_OFFSET: u32 = 17;
    pub const AUTOPULL_MASK: u32 = 0x00020000;
    pub const AUTOPULL: u32 = 1 << 17;

    /// **AUTOPUSH**
    ///
    /// Push automatically when the input shift register is filled, i.e. on an IN instruction which causes the input shift counter to reach or exceed PUSH_THRESH.
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const AUTOPUSH_OFFSET: u32 = 16;
    pub const AUTOPUSH_MASK: u32 = 0x00010000;
    pub const AUTOPUSH: u32 = 1 << 16;

    /// **FJOIN_RX_PUT**
    ///
    /// If 1, disable this state machine's RX FIFO, make its storage available for random write access by the state machine (using the `put` instruction) and, unless FJOIN_RX_GET is also set, random read access by the processor (through the RXFx_PUTGETy registers).
    /// If FJOIN_RX_PUT and FJOIN_RX_GET are both set, then the RX FIFO's registers can be randomly read/written by the state machine, but are completely inaccessible to the processor.
    /// Setting this bit will clear the FJOIN_TX and FJOIN_RX bits.
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const FJOIN_RX_PUT_OFFSET: u32 = 15;
    pub const FJOIN_RX_PUT_MASK: u32 = 0x00008000;
    pub const FJOIN_RX_PUT: u32 = 1 << 15;

    /// **FJOIN_RX_GET**
    ///
    /// If 1, disable this state machine's RX FIFO, make its storage available for random read access by the state machine (using the `get` instruction) and, unless FJOIN_RX_PUT is also set, random write access by the processor (through the RXFx_PUTGETy registers).
    /// If FJOIN_RX_PUT and FJOIN_RX_GET are both set, then the RX FIFO's registers can be randomly read/written by the state machine, but are completely inaccessible to the processor.
    /// Setting this bit will clear the FJOIN_TX and FJOIN_RX bits.
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const FJOIN_RX_GET_OFFSET: u32 = 14;
    pub const FJOIN_RX_GET_MASK: u32 = 0x00004000;
    pub const FJOIN_RX_GET: u32 = 1 << 14;

    /// **IN_COUNT**
    ///
    /// Set the number of pins which are not masked to 0 when read by an IN PINS, WAIT PIN or MOV x, PINS instruction.
    /// For example, an IN_COUNT of 5 means that the 5 LSBs of the IN pin group are visible (bits 4:0), but the remaining 27 MSBs are masked to 0. A count of 32 is encoded with a field value of 0, so the default behaviour is to not perform any masking.
    /// Note this masking is applied in addition to the masking usually performed by the IN instruction. This is mainly useful for the MOV x, PINS instruction, which otherwise has no way of masking pins.
    ///
    /// Bits `[4:0]`, mask `0x0000001f`
    pub const IN_COUNT_OFFSET: u32 = 0;
    pub const IN_COUNT_MASK: u32 = 0x0000001f;

    // ======== Field Accessors ========

    /// **FJOIN_RX** `[31]` (RW)
    ///
    /// When 1, RX FIFO steals the TX FIFO's storage, and becomes twice as deep.
    /// TX FIFO is disabled as a result (always reads as both full and empty).
    /// FIFOs are flushed when this bit is changed.
    #[inline(always)]
    pub fn fjoin_rx(&self) -> bool {
        (self.value.read_bits() & Self::FJOIN_RX_MASK) != 0
    }

    /// Set FJOIN_RX bit atomically (RW)
    #[inline(always)]
    pub fn set_fjoin_rx(&self) {
        self.set_bits(Self::FJOIN_RX_MASK);
    }

    /// Clear FJOIN_RX bit atomically (RW)
    #[inline(always)]
    pub fn clr_fjoin_rx(&self) {
        self.clr_bits(Self::FJOIN_RX_MASK);
    }

    /// **FJOIN_TX** `[30]` (RW)
    ///
    /// When 1, TX FIFO steals the RX FIFO's storage, and becomes twice as deep.
    /// RX FIFO is disabled as a result (always reads as both full and empty).
    /// FIFOs are flushed when this bit is changed.
    #[inline(always)]
    pub fn fjoin_tx(&self) -> bool {
        (self.value.read_bits() & Self::FJOIN_TX_MASK) != 0
    }

    /// Set FJOIN_TX bit atomically (RW)
    #[inline(always)]
    pub fn set_fjoin_tx(&self) {
        self.set_bits(Self::FJOIN_TX_MASK);
    }

    /// Clear FJOIN_TX bit atomically (RW)
    #[inline(always)]
    pub fn clr_fjoin_tx(&self) {
        self.clr_bits(Self::FJOIN_TX_MASK);
    }

    /// **PULL_THRESH** `[29:25]` (RW)
    ///
    /// Number of bits shifted out of OSR before autopull, or conditional pull (PULL IFEMPTY), will take place.
    /// Write 0 for value of 32.
    #[inline(always)]
    pub fn pull_thresh(&self) -> u32 {
        (self.value.read_bits() & Self::PULL_THRESH_MASK) >> Self::PULL_THRESH_OFFSET
    }

    /// Write **PULL_THRESH** `[29:25]` (RW)
    ///
    /// Number of bits shifted out of OSR before autopull, or conditional pull (PULL IFEMPTY), will take place.
    /// Write 0 for value of 32.
    #[inline(always)]
    pub fn write_pull_thresh(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::PULL_THRESH_MASK) | ((value << Self::PULL_THRESH_OFFSET) & Self::PULL_THRESH_MASK));
    }

    /// **PUSH_THRESH** `[24:20]` (RW)
    ///
    /// Number of bits shifted into ISR before autopush, or conditional push (PUSH IFFULL), will take place.
    /// Write 0 for value of 32.
    #[inline(always)]
    pub fn push_thresh(&self) -> u32 {
        (self.value.read_bits() & Self::PUSH_THRESH_MASK) >> Self::PUSH_THRESH_OFFSET
    }

    /// Write **PUSH_THRESH** `[24:20]` (RW)
    ///
    /// Number of bits shifted into ISR before autopush, or conditional push (PUSH IFFULL), will take place.
    /// Write 0 for value of 32.
    #[inline(always)]
    pub fn write_push_thresh(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::PUSH_THRESH_MASK) | ((value << Self::PUSH_THRESH_OFFSET) & Self::PUSH_THRESH_MASK));
    }

    /// **OUT_SHIFTDIR** `[19]` (RW)
    ///
    /// 1 = shift out of output shift register to right. 0 = to left.
    #[inline(always)]
    pub fn out_shiftdir(&self) -> bool {
        (self.value.read_bits() & Self::OUT_SHIFTDIR_MASK) != 0
    }

    /// Set OUT_SHIFTDIR bit atomically (RW)
    #[inline(always)]
    pub fn set_out_shiftdir(&self) {
        self.set_bits(Self::OUT_SHIFTDIR_MASK);
    }

    /// Clear OUT_SHIFTDIR bit atomically (RW)
    #[inline(always)]
    pub fn clr_out_shiftdir(&self) {
        self.clr_bits(Self::OUT_SHIFTDIR_MASK);
    }

    /// **IN_SHIFTDIR** `[18]` (RW)
    ///
    /// 1 = shift input shift register to right (data enters from left). 0 = to left.
    #[inline(always)]
    pub fn in_shiftdir(&self) -> bool {
        (self.value.read_bits() & Self::IN_SHIFTDIR_MASK) != 0
    }

    /// Set IN_SHIFTDIR bit atomically (RW)
    #[inline(always)]
    pub fn set_in_shiftdir(&self) {
        self.set_bits(Self::IN_SHIFTDIR_MASK);
    }

    /// Clear IN_SHIFTDIR bit atomically (RW)
    #[inline(always)]
    pub fn clr_in_shiftdir(&self) {
        self.clr_bits(Self::IN_SHIFTDIR_MASK);
    }

    /// **AUTOPULL** `[17]` (RW)
    ///
    /// Pull automatically when the output shift register is emptied, i.e. on or following an OUT instruction which causes the output shift counter to reach or exceed PULL_THRESH.
    #[inline(always)]
    pub fn autopull(&self) -> bool {
        (self.value.read_bits() & Self::AUTOPULL_MASK) != 0
    }

    /// Set AUTOPULL bit atomically (RW)
    #[inline(always)]
    pub fn set_autopull(&self) {
        self.set_bits(Self::AUTOPULL_MASK);
    }

    /// Clear AUTOPULL bit atomically (RW)
    #[inline(always)]
    pub fn clr_autopull(&self) {
        self.clr_bits(Self::AUTOPULL_MASK);
    }

    /// **AUTOPUSH** `[16]` (RW)
    ///
    /// Push automatically when the input shift register is filled, i.e. on an IN instruction which causes the input shift counter to reach or exceed PUSH_THRESH.
    #[inline(always)]
    pub fn autopush(&self) -> bool {
        (self.value.read_bits() & Self::AUTOPUSH_MASK) != 0
    }

    /// Set AUTOPUSH bit atomically (RW)
    #[inline(always)]
    pub fn set_autopush(&self) {
        self.set_bits(Self::AUTOPUSH_MASK);
    }

    /// Clear AUTOPUSH bit atomically (RW)
    #[inline(always)]
    pub fn clr_autopush(&self) {
        self.clr_bits(Self::AUTOPUSH_MASK);
    }

    /// **FJOIN_RX_PUT** `[15]` (RW)
    ///
    /// If 1, disable this state machine's RX FIFO, make its storage available for random write access by the state machine (using the `put` instruction) and, unless FJOIN_RX_GET is also set, random read access by the processor (through the RXFx_PUTGETy registers).
    /// If FJOIN_RX_PUT and FJOIN_RX_GET are both set, then the RX FIFO's registers can be randomly read/written by the state machine, but are completely inaccessible to the processor.
    /// Setting this bit will clear the FJOIN_TX and FJOIN_RX bits.
    #[inline(always)]
    pub fn fjoin_rx_put(&self) -> bool {
        (self.value.read_bits() & Self::FJOIN_RX_PUT_MASK) != 0
    }

    /// Set FJOIN_RX_PUT bit atomically (RW)
    #[inline(always)]
    pub fn set_fjoin_rx_put(&self) {
        self.set_bits(Self::FJOIN_RX_PUT_MASK);
    }

    /// Clear FJOIN_RX_PUT bit atomically (RW)
    #[inline(always)]
    pub fn clr_fjoin_rx_put(&self) {
        self.clr_bits(Self::FJOIN_RX_PUT_MASK);
    }

    /// **FJOIN_RX_GET** `[14]` (RW)
    ///
    /// If 1, disable this state machine's RX FIFO, make its storage available for random read access by the state machine (using the `get` instruction) and, unless FJOIN_RX_PUT is also set, random write access by the processor (through the RXFx_PUTGETy registers).
    /// If FJOIN_RX_PUT and FJOIN_RX_GET are both set, then the RX FIFO's registers can be randomly read/written by the state machine, but are completely inaccessible to the processor.
    /// Setting this bit will clear the FJOIN_TX and FJOIN_RX bits.
    #[inline(always)]
    pub fn fjoin_rx_get(&self) -> bool {
        (self.value.read_bits() & Self::FJOIN_RX_GET_MASK) != 0
    }

    /// Set FJOIN_RX_GET bit atomically (RW)
    #[inline(always)]
    pub fn set_fjoin_rx_get(&self) {
        self.set_bits(Self::FJOIN_RX_GET_MASK);
    }

    /// Clear FJOIN_RX_GET bit atomically (RW)
    #[inline(always)]
    pub fn clr_fjoin_rx_get(&self) {
        self.clr_bits(Self::FJOIN_RX_GET_MASK);
    }

    /// **IN_COUNT** `[4:0]` (RW)
    ///
    /// Set the number of pins which are not masked to 0 when read by an IN PINS, WAIT PIN or MOV x, PINS instruction.
    /// For example, an IN_COUNT of 5 means that the 5 LSBs of the IN pin group are visible (bits 4:0), but the remaining 27 MSBs are masked to 0. A count of 32 is encoded with a field value of 0, so the default behaviour is to not perform any masking.
    /// Note this masking is applied in addition to the masking usually performed by the IN instruction. This is mainly useful for the MOV x, PINS instruction, which otherwise has no way of masking pins.
    #[inline(always)]
    pub fn in_count(&self) -> u32 {
        (self.value.read_bits() & Self::IN_COUNT_MASK) >> Self::IN_COUNT_OFFSET
    }

    /// Write **IN_COUNT** `[4:0]` (RW)
    ///
    /// Set the number of pins which are not masked to 0 when read by an IN PINS, WAIT PIN or MOV x, PINS instruction.
    /// For example, an IN_COUNT of 5 means that the 5 LSBs of the IN pin group are visible (bits 4:0), but the remaining 27 MSBs are masked to 0. A count of 32 is encoded with a field value of 0, so the default behaviour is to not perform any masking.
    /// Note this masking is applied in addition to the masking usually performed by the IN instruction. This is mainly useful for the MOV x, PINS instruction, which otherwise has no way of masking pins.
    #[inline(always)]
    pub fn write_in_count(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::IN_COUNT_MASK) | ((value << Self::IN_COUNT_OFFSET) & Self::IN_COUNT_MASK));
    }

}

/// SM0_SHIFTCTRL value type for batch read/modify/write operations
///
/// Use `sm0shiftctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sm0shiftctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Sm0ShiftctrlVal {
    bits: u32,
}

impl Sm0ShiftctrlVal {
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
        Self { bits: 0x000c0000 }
    }

    /// Read **FJOIN_RX** `[31:31]`
    ///
    /// When 1, RX FIFO steals the TX FIFO's storage, and becomes twice as deep.
    /// TX FIFO is disabled as a result (always reads as both full and empty).
    /// FIFOs are flushed when this bit is changed.
    #[inline(always)]
    pub const fn fjoin_rx(&self) -> bool {
        (self.bits & Sm0Shiftctrl::FJOIN_RX_MASK) != 0
    }

    /// Read **FJOIN_TX** `[30:30]`
    ///
    /// When 1, TX FIFO steals the RX FIFO's storage, and becomes twice as deep.
    /// RX FIFO is disabled as a result (always reads as both full and empty).
    /// FIFOs are flushed when this bit is changed.
    #[inline(always)]
    pub const fn fjoin_tx(&self) -> bool {
        (self.bits & Sm0Shiftctrl::FJOIN_TX_MASK) != 0
    }

    /// Read **PULL_THRESH** `[29:25]`
    ///
    /// Number of bits shifted out of OSR before autopull, or conditional pull (PULL IFEMPTY), will take place.
    /// Write 0 for value of 32.
    #[inline(always)]
    pub const fn pull_thresh(&self) -> u32 {
        (self.bits & Sm0Shiftctrl::PULL_THRESH_MASK) >> Sm0Shiftctrl::PULL_THRESH_OFFSET
    }

    /// Read **PUSH_THRESH** `[24:20]`
    ///
    /// Number of bits shifted into ISR before autopush, or conditional push (PUSH IFFULL), will take place.
    /// Write 0 for value of 32.
    #[inline(always)]
    pub const fn push_thresh(&self) -> u32 {
        (self.bits & Sm0Shiftctrl::PUSH_THRESH_MASK) >> Sm0Shiftctrl::PUSH_THRESH_OFFSET
    }

    /// Read **OUT_SHIFTDIR** `[19:19]`
    ///
    /// 1 = shift out of output shift register to right. 0 = to left.
    #[inline(always)]
    pub const fn out_shiftdir(&self) -> bool {
        (self.bits & Sm0Shiftctrl::OUT_SHIFTDIR_MASK) != 0
    }

    /// Read **IN_SHIFTDIR** `[18:18]`
    ///
    /// 1 = shift input shift register to right (data enters from left). 0 = to left.
    #[inline(always)]
    pub const fn in_shiftdir(&self) -> bool {
        (self.bits & Sm0Shiftctrl::IN_SHIFTDIR_MASK) != 0
    }

    /// Read **AUTOPULL** `[17:17]`
    ///
    /// Pull automatically when the output shift register is emptied, i.e. on or following an OUT instruction which causes the output shift counter to reach or exceed PULL_THRESH.
    #[inline(always)]
    pub const fn autopull(&self) -> bool {
        (self.bits & Sm0Shiftctrl::AUTOPULL_MASK) != 0
    }

    /// Read **AUTOPUSH** `[16:16]`
    ///
    /// Push automatically when the input shift register is filled, i.e. on an IN instruction which causes the input shift counter to reach or exceed PUSH_THRESH.
    #[inline(always)]
    pub const fn autopush(&self) -> bool {
        (self.bits & Sm0Shiftctrl::AUTOPUSH_MASK) != 0
    }

    /// Read **FJOIN_RX_PUT** `[15:15]`
    ///
    /// If 1, disable this state machine's RX FIFO, make its storage available for random write access by the state machine (using the `put` instruction) and, unless FJOIN_RX_GET is also set, random read access by the processor (through the RXFx_PUTGETy registers).
    /// If FJOIN_RX_PUT and FJOIN_RX_GET are both set, then the RX FIFO's registers can be randomly read/written by the state machine, but are completely inaccessible to the processor.
    /// Setting this bit will clear the FJOIN_TX and FJOIN_RX bits.
    #[inline(always)]
    pub const fn fjoin_rx_put(&self) -> bool {
        (self.bits & Sm0Shiftctrl::FJOIN_RX_PUT_MASK) != 0
    }

    /// Read **FJOIN_RX_GET** `[14:14]`
    ///
    /// If 1, disable this state machine's RX FIFO, make its storage available for random read access by the state machine (using the `get` instruction) and, unless FJOIN_RX_PUT is also set, random write access by the processor (through the RXFx_PUTGETy registers).
    /// If FJOIN_RX_PUT and FJOIN_RX_GET are both set, then the RX FIFO's registers can be randomly read/written by the state machine, but are completely inaccessible to the processor.
    /// Setting this bit will clear the FJOIN_TX and FJOIN_RX bits.
    #[inline(always)]
    pub const fn fjoin_rx_get(&self) -> bool {
        (self.bits & Sm0Shiftctrl::FJOIN_RX_GET_MASK) != 0
    }

    /// Read **IN_COUNT** `[4:0]`
    ///
    /// Set the number of pins which are not masked to 0 when read by an IN PINS, WAIT PIN or MOV x, PINS instruction.
    /// For example, an IN_COUNT of 5 means that the 5 LSBs of the IN pin group are visible (bits 4:0), but the remaining 27 MSBs are masked to 0. A count of 32 is encoded with a field value of 0, so the default behaviour is to not perform any masking.
    /// Note this masking is applied in addition to the masking usually performed by the IN instruction. This is mainly useful for the MOV x, PINS instruction, which otherwise has no way of masking pins.
    #[inline(always)]
    pub const fn in_count(&self) -> u32 {
        (self.bits & Sm0Shiftctrl::IN_COUNT_MASK) >> Sm0Shiftctrl::IN_COUNT_OFFSET
    }

    /// Set **FJOIN_RX** `[31:31]`
    ///
    /// When 1, RX FIFO steals the TX FIFO's storage, and becomes twice as deep.
    /// TX FIFO is disabled as a result (always reads as both full and empty).
    /// FIFOs are flushed when this bit is changed.
    #[inline(always)]
    pub const fn with_fjoin_rx(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sm0Shiftctrl::FJOIN_RX_MASK
            } else {
                self.bits & !Sm0Shiftctrl::FJOIN_RX_MASK
            }
        }
    }

    /// Set **FJOIN_TX** `[30:30]`
    ///
    /// When 1, TX FIFO steals the RX FIFO's storage, and becomes twice as deep.
    /// RX FIFO is disabled as a result (always reads as both full and empty).
    /// FIFOs are flushed when this bit is changed.
    #[inline(always)]
    pub const fn with_fjoin_tx(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sm0Shiftctrl::FJOIN_TX_MASK
            } else {
                self.bits & !Sm0Shiftctrl::FJOIN_TX_MASK
            }
        }
    }

    /// Set **PULL_THRESH** `[29:25]`
    ///
    /// Number of bits shifted out of OSR before autopull, or conditional pull (PULL IFEMPTY), will take place.
    /// Write 0 for value of 32.
    #[inline(always)]
    pub const fn with_pull_thresh(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sm0Shiftctrl::PULL_THRESH_MASK) | ((value << Sm0Shiftctrl::PULL_THRESH_OFFSET) & Sm0Shiftctrl::PULL_THRESH_MASK)
        }
    }

    /// Set **PUSH_THRESH** `[24:20]`
    ///
    /// Number of bits shifted into ISR before autopush, or conditional push (PUSH IFFULL), will take place.
    /// Write 0 for value of 32.
    #[inline(always)]
    pub const fn with_push_thresh(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sm0Shiftctrl::PUSH_THRESH_MASK) | ((value << Sm0Shiftctrl::PUSH_THRESH_OFFSET) & Sm0Shiftctrl::PUSH_THRESH_MASK)
        }
    }

    /// Set **OUT_SHIFTDIR** `[19:19]`
    ///
    /// 1 = shift out of output shift register to right. 0 = to left.
    #[inline(always)]
    pub const fn with_out_shiftdir(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sm0Shiftctrl::OUT_SHIFTDIR_MASK
            } else {
                self.bits & !Sm0Shiftctrl::OUT_SHIFTDIR_MASK
            }
        }
    }

    /// Set **IN_SHIFTDIR** `[18:18]`
    ///
    /// 1 = shift input shift register to right (data enters from left). 0 = to left.
    #[inline(always)]
    pub const fn with_in_shiftdir(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sm0Shiftctrl::IN_SHIFTDIR_MASK
            } else {
                self.bits & !Sm0Shiftctrl::IN_SHIFTDIR_MASK
            }
        }
    }

    /// Set **AUTOPULL** `[17:17]`
    ///
    /// Pull automatically when the output shift register is emptied, i.e. on or following an OUT instruction which causes the output shift counter to reach or exceed PULL_THRESH.
    #[inline(always)]
    pub const fn with_autopull(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sm0Shiftctrl::AUTOPULL_MASK
            } else {
                self.bits & !Sm0Shiftctrl::AUTOPULL_MASK
            }
        }
    }

    /// Set **AUTOPUSH** `[16:16]`
    ///
    /// Push automatically when the input shift register is filled, i.e. on an IN instruction which causes the input shift counter to reach or exceed PUSH_THRESH.
    #[inline(always)]
    pub const fn with_autopush(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sm0Shiftctrl::AUTOPUSH_MASK
            } else {
                self.bits & !Sm0Shiftctrl::AUTOPUSH_MASK
            }
        }
    }

    /// Set **FJOIN_RX_PUT** `[15:15]`
    ///
    /// If 1, disable this state machine's RX FIFO, make its storage available for random write access by the state machine (using the `put` instruction) and, unless FJOIN_RX_GET is also set, random read access by the processor (through the RXFx_PUTGETy registers).
    /// If FJOIN_RX_PUT and FJOIN_RX_GET are both set, then the RX FIFO's registers can be randomly read/written by the state machine, but are completely inaccessible to the processor.
    /// Setting this bit will clear the FJOIN_TX and FJOIN_RX bits.
    #[inline(always)]
    pub const fn with_fjoin_rx_put(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sm0Shiftctrl::FJOIN_RX_PUT_MASK
            } else {
                self.bits & !Sm0Shiftctrl::FJOIN_RX_PUT_MASK
            }
        }
    }

    /// Set **FJOIN_RX_GET** `[14:14]`
    ///
    /// If 1, disable this state machine's RX FIFO, make its storage available for random read access by the state machine (using the `get` instruction) and, unless FJOIN_RX_PUT is also set, random write access by the processor (through the RXFx_PUTGETy registers).
    /// If FJOIN_RX_PUT and FJOIN_RX_GET are both set, then the RX FIFO's registers can be randomly read/written by the state machine, but are completely inaccessible to the processor.
    /// Setting this bit will clear the FJOIN_TX and FJOIN_RX bits.
    #[inline(always)]
    pub const fn with_fjoin_rx_get(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sm0Shiftctrl::FJOIN_RX_GET_MASK
            } else {
                self.bits & !Sm0Shiftctrl::FJOIN_RX_GET_MASK
            }
        }
    }

    /// Set **IN_COUNT** `[4:0]`
    ///
    /// Set the number of pins which are not masked to 0 when read by an IN PINS, WAIT PIN or MOV x, PINS instruction.
    /// For example, an IN_COUNT of 5 means that the 5 LSBs of the IN pin group are visible (bits 4:0), but the remaining 27 MSBs are masked to 0. A count of 32 is encoded with a field value of 0, so the default behaviour is to not perform any masking.
    /// Note this masking is applied in addition to the masking usually performed by the IN instruction. This is mainly useful for the MOV x, PINS instruction, which otherwise has no way of masking pins.
    #[inline(always)]
    pub const fn with_in_count(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sm0Shiftctrl::IN_COUNT_MASK) | ((value << Sm0Shiftctrl::IN_COUNT_OFFSET) & Sm0Shiftctrl::IN_COUNT_MASK)
        }
    }

}

impl Default for Sm0ShiftctrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Sm0Shiftctrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Sm0ShiftctrlVal {
        Sm0ShiftctrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Sm0ShiftctrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Sm0ShiftctrlVal) -> Sm0ShiftctrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SM0_ADDR Register
///
/// Current instruction address of state machine 0
///
/// Address offset: 0x00d4
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SM0_ADDR** `[4:0]` (RO)
#[repr(transparent)]
pub struct Sm0Addr {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Sm0Addr {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sm0Addr {
    // ======== Field Constants ========

    /// **SM0_ADDR**
    /// Bits `[4:0]`, mask `0x0000001f`
    pub const SM0_ADDR_OFFSET: u32 = 0;
    pub const SM0_ADDR_MASK: u32 = 0x0000001f;

    // ======== Field Accessors ========

    /// **SM0_ADDR** `[4:0]` (RO)
    #[inline(always)]
    pub fn sm0_addr(&self) -> u32 {
        (self.value.read_bits() & Self::SM0_ADDR_MASK) >> Self::SM0_ADDR_OFFSET
    }

}

/// SM0_ADDR value type for batch read/modify/write operations
///
/// Use `sm0addr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sm0addr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Sm0AddrVal {
    bits: u32,
}

impl Sm0AddrVal {
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

    /// Read **SM0_ADDR** `[4:0]`
    #[inline(always)]
    pub const fn sm0_addr(&self) -> u32 {
        (self.bits & Sm0Addr::SM0_ADDR_MASK) >> Sm0Addr::SM0_ADDR_OFFSET
    }

}

impl Default for Sm0AddrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Sm0Addr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Sm0AddrVal {
        Sm0AddrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Sm0AddrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Sm0AddrVal) -> Sm0AddrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SM0_INSTR Register
///
/// Read to see the instruction currently addressed by state machine 0's program counter
/// Write to execute an instruction immediately (including jumps) and then resume execution.
///
/// Address offset: 0x00d8
///
/// ## Fields
///
/// - **SM0_INSTR** `[15:0]` (RW)
#[repr(transparent)]
pub struct Sm0Instr {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Sm0Instr {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sm0Instr {
    // ======== Field Constants ========

    /// **SM0_INSTR**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const SM0_INSTR_OFFSET: u32 = 0;
    pub const SM0_INSTR_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **SM0_INSTR** `[15:0]` (RW)
    #[inline(always)]
    pub fn sm0_instr(&self) -> u32 {
        (self.value.read_bits() & Self::SM0_INSTR_MASK) >> Self::SM0_INSTR_OFFSET
    }

    /// Write **SM0_INSTR** `[15:0]` (RW)
    #[inline(always)]
    pub fn write_sm0_instr(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SM0_INSTR_MASK) | ((value << Self::SM0_INSTR_OFFSET) & Self::SM0_INSTR_MASK));
    }

}

/// SM0_INSTR value type for batch read/modify/write operations
///
/// Use `sm0instr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sm0instr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Sm0InstrVal {
    bits: u32,
}

impl Sm0InstrVal {
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

    /// Read **SM0_INSTR** `[15:0]`
    #[inline(always)]
    pub const fn sm0_instr(&self) -> u32 {
        (self.bits & Sm0Instr::SM0_INSTR_MASK) >> Sm0Instr::SM0_INSTR_OFFSET
    }

    /// Set **SM0_INSTR** `[15:0]`
    #[inline(always)]
    pub const fn with_sm0_instr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sm0Instr::SM0_INSTR_MASK) | ((value << Sm0Instr::SM0_INSTR_OFFSET) & Sm0Instr::SM0_INSTR_MASK)
        }
    }

}

impl Default for Sm0InstrVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Sm0Instr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Sm0InstrVal {
        Sm0InstrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Sm0InstrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Sm0InstrVal) -> Sm0InstrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SM0_PINCTRL Register
///
/// State machine pin control
///
/// Address offset: 0x00dc
/// Reset value: 0x14000000
///
/// ## Fields
///
/// - **SIDESET_COUNT** `[31:29]` (RW)
///
///   The number of MSBs of the Delay/Side-set instruction field which are used for side-set. Inclusive of the enable bit, if present. Minimum of 0 (all delay bits, no side-set) and maximum of 5 (all side-set, no delay).
///
/// - **SET_COUNT** `[28:26]` (RW)
///
///   The number of pins asserted by a SET. In the range 0 to 5 inclusive.
///
/// - **OUT_COUNT** `[25:20]` (RW)
///
///   The number of pins asserted by an OUT PINS, OUT PINDIRS or MOV PINS instruction. In the range 0 to 32 inclusive.
///
/// - **IN_BASE** `[19:15]` (RW)
///
///   The pin which is mapped to the least-significant bit of a state machine's IN data bus. Higher-numbered pins are mapped to consecutively more-significant data bits, with a modulo of 32 applied to pin number.
///
/// - **SIDESET_BASE** `[14:10]` (RW)
///
///   The lowest-numbered pin that will be affected by a side-set operation. The MSBs of an instruction's side-set/delay field (up to 5, determined by SIDESET_COUNT) are used for side-set data, with the remaining LSBs used for delay. The least-significant bit of the side-set portion is the bit written to this pin, with more-significant bits written to higher-numbered pins.
///
/// - **SET_BASE** `[9:5]` (RW)
///
///   The lowest-numbered pin that will be affected by a SET PINS or SET PINDIRS instruction. The data written to this pin is the least-significant bit of the SET data.
///
/// - **OUT_BASE** `[4:0]` (RW)
///
///   The lowest-numbered pin that will be affected by an OUT PINS, OUT PINDIRS or MOV PINS instruction. The data written to this pin will always be the least-significant bit of the OUT or MOV data.
#[repr(transparent)]
pub struct Sm0Pinctrl {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Sm0Pinctrl {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sm0Pinctrl {
    // ======== Field Constants ========

    /// **SIDESET_COUNT**
    ///
    /// The number of MSBs of the Delay/Side-set instruction field which are used for side-set. Inclusive of the enable bit, if present. Minimum of 0 (all delay bits, no side-set) and maximum of 5 (all side-set, no delay).
    ///
    /// Bits `[31:29]`, mask `0xe0000000`
    pub const SIDESET_COUNT_OFFSET: u32 = 29;
    pub const SIDESET_COUNT_MASK: u32 = 0xe0000000;

    /// **SET_COUNT**
    ///
    /// The number of pins asserted by a SET. In the range 0 to 5 inclusive.
    ///
    /// Bits `[28:26]`, mask `0x1c000000`
    pub const SET_COUNT_OFFSET: u32 = 26;
    pub const SET_COUNT_MASK: u32 = 0x1c000000;

    /// **OUT_COUNT**
    ///
    /// The number of pins asserted by an OUT PINS, OUT PINDIRS or MOV PINS instruction. In the range 0 to 32 inclusive.
    ///
    /// Bits `[25:20]`, mask `0x03f00000`
    pub const OUT_COUNT_OFFSET: u32 = 20;
    pub const OUT_COUNT_MASK: u32 = 0x03f00000;

    /// **IN_BASE**
    ///
    /// The pin which is mapped to the least-significant bit of a state machine's IN data bus. Higher-numbered pins are mapped to consecutively more-significant data bits, with a modulo of 32 applied to pin number.
    ///
    /// Bits `[19:15]`, mask `0x000f8000`
    pub const IN_BASE_OFFSET: u32 = 15;
    pub const IN_BASE_MASK: u32 = 0x000f8000;

    /// **SIDESET_BASE**
    ///
    /// The lowest-numbered pin that will be affected by a side-set operation. The MSBs of an instruction's side-set/delay field (up to 5, determined by SIDESET_COUNT) are used for side-set data, with the remaining LSBs used for delay. The least-significant bit of the side-set portion is the bit written to this pin, with more-significant bits written to higher-numbered pins.
    ///
    /// Bits `[14:10]`, mask `0x00007c00`
    pub const SIDESET_BASE_OFFSET: u32 = 10;
    pub const SIDESET_BASE_MASK: u32 = 0x00007c00;

    /// **SET_BASE**
    ///
    /// The lowest-numbered pin that will be affected by a SET PINS or SET PINDIRS instruction. The data written to this pin is the least-significant bit of the SET data.
    ///
    /// Bits `[9:5]`, mask `0x000003e0`
    pub const SET_BASE_OFFSET: u32 = 5;
    pub const SET_BASE_MASK: u32 = 0x000003e0;

    /// **OUT_BASE**
    ///
    /// The lowest-numbered pin that will be affected by an OUT PINS, OUT PINDIRS or MOV PINS instruction. The data written to this pin will always be the least-significant bit of the OUT or MOV data.
    ///
    /// Bits `[4:0]`, mask `0x0000001f`
    pub const OUT_BASE_OFFSET: u32 = 0;
    pub const OUT_BASE_MASK: u32 = 0x0000001f;

    // ======== Field Accessors ========

    /// **SIDESET_COUNT** `[31:29]` (RW)
    ///
    /// The number of MSBs of the Delay/Side-set instruction field which are used for side-set. Inclusive of the enable bit, if present. Minimum of 0 (all delay bits, no side-set) and maximum of 5 (all side-set, no delay).
    #[inline(always)]
    pub fn sideset_count(&self) -> u32 {
        (self.value.read_bits() & Self::SIDESET_COUNT_MASK) >> Self::SIDESET_COUNT_OFFSET
    }

    /// Write **SIDESET_COUNT** `[31:29]` (RW)
    ///
    /// The number of MSBs of the Delay/Side-set instruction field which are used for side-set. Inclusive of the enable bit, if present. Minimum of 0 (all delay bits, no side-set) and maximum of 5 (all side-set, no delay).
    #[inline(always)]
    pub fn write_sideset_count(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SIDESET_COUNT_MASK) | ((value << Self::SIDESET_COUNT_OFFSET) & Self::SIDESET_COUNT_MASK));
    }

    /// **SET_COUNT** `[28:26]` (RW)
    ///
    /// The number of pins asserted by a SET. In the range 0 to 5 inclusive.
    #[inline(always)]
    pub fn set_count(&self) -> u32 {
        (self.value.read_bits() & Self::SET_COUNT_MASK) >> Self::SET_COUNT_OFFSET
    }

    /// Write **SET_COUNT** `[28:26]` (RW)
    ///
    /// The number of pins asserted by a SET. In the range 0 to 5 inclusive.
    #[inline(always)]
    pub fn write_set_count(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SET_COUNT_MASK) | ((value << Self::SET_COUNT_OFFSET) & Self::SET_COUNT_MASK));
    }

    /// **OUT_COUNT** `[25:20]` (RW)
    ///
    /// The number of pins asserted by an OUT PINS, OUT PINDIRS or MOV PINS instruction. In the range 0 to 32 inclusive.
    #[inline(always)]
    pub fn out_count(&self) -> u32 {
        (self.value.read_bits() & Self::OUT_COUNT_MASK) >> Self::OUT_COUNT_OFFSET
    }

    /// Write **OUT_COUNT** `[25:20]` (RW)
    ///
    /// The number of pins asserted by an OUT PINS, OUT PINDIRS or MOV PINS instruction. In the range 0 to 32 inclusive.
    #[inline(always)]
    pub fn write_out_count(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::OUT_COUNT_MASK) | ((value << Self::OUT_COUNT_OFFSET) & Self::OUT_COUNT_MASK));
    }

    /// **IN_BASE** `[19:15]` (RW)
    ///
    /// The pin which is mapped to the least-significant bit of a state machine's IN data bus. Higher-numbered pins are mapped to consecutively more-significant data bits, with a modulo of 32 applied to pin number.
    #[inline(always)]
    pub fn in_base(&self) -> u32 {
        (self.value.read_bits() & Self::IN_BASE_MASK) >> Self::IN_BASE_OFFSET
    }

    /// Write **IN_BASE** `[19:15]` (RW)
    ///
    /// The pin which is mapped to the least-significant bit of a state machine's IN data bus. Higher-numbered pins are mapped to consecutively more-significant data bits, with a modulo of 32 applied to pin number.
    #[inline(always)]
    pub fn write_in_base(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::IN_BASE_MASK) | ((value << Self::IN_BASE_OFFSET) & Self::IN_BASE_MASK));
    }

    /// **SIDESET_BASE** `[14:10]` (RW)
    ///
    /// The lowest-numbered pin that will be affected by a side-set operation. The MSBs of an instruction's side-set/delay field (up to 5, determined by SIDESET_COUNT) are used for side-set data, with the remaining LSBs used for delay. The least-significant bit of the side-set portion is the bit written to this pin, with more-significant bits written to higher-numbered pins.
    #[inline(always)]
    pub fn sideset_base(&self) -> u32 {
        (self.value.read_bits() & Self::SIDESET_BASE_MASK) >> Self::SIDESET_BASE_OFFSET
    }

    /// Write **SIDESET_BASE** `[14:10]` (RW)
    ///
    /// The lowest-numbered pin that will be affected by a side-set operation. The MSBs of an instruction's side-set/delay field (up to 5, determined by SIDESET_COUNT) are used for side-set data, with the remaining LSBs used for delay. The least-significant bit of the side-set portion is the bit written to this pin, with more-significant bits written to higher-numbered pins.
    #[inline(always)]
    pub fn write_sideset_base(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SIDESET_BASE_MASK) | ((value << Self::SIDESET_BASE_OFFSET) & Self::SIDESET_BASE_MASK));
    }

    /// **SET_BASE** `[9:5]` (RW)
    ///
    /// The lowest-numbered pin that will be affected by a SET PINS or SET PINDIRS instruction. The data written to this pin is the least-significant bit of the SET data.
    #[inline(always)]
    pub fn set_base(&self) -> u32 {
        (self.value.read_bits() & Self::SET_BASE_MASK) >> Self::SET_BASE_OFFSET
    }

    /// Write **SET_BASE** `[9:5]` (RW)
    ///
    /// The lowest-numbered pin that will be affected by a SET PINS or SET PINDIRS instruction. The data written to this pin is the least-significant bit of the SET data.
    #[inline(always)]
    pub fn write_set_base(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SET_BASE_MASK) | ((value << Self::SET_BASE_OFFSET) & Self::SET_BASE_MASK));
    }

    /// **OUT_BASE** `[4:0]` (RW)
    ///
    /// The lowest-numbered pin that will be affected by an OUT PINS, OUT PINDIRS or MOV PINS instruction. The data written to this pin will always be the least-significant bit of the OUT or MOV data.
    #[inline(always)]
    pub fn out_base(&self) -> u32 {
        (self.value.read_bits() & Self::OUT_BASE_MASK) >> Self::OUT_BASE_OFFSET
    }

    /// Write **OUT_BASE** `[4:0]` (RW)
    ///
    /// The lowest-numbered pin that will be affected by an OUT PINS, OUT PINDIRS or MOV PINS instruction. The data written to this pin will always be the least-significant bit of the OUT or MOV data.
    #[inline(always)]
    pub fn write_out_base(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::OUT_BASE_MASK) | ((value << Self::OUT_BASE_OFFSET) & Self::OUT_BASE_MASK));
    }

}

/// SM0_PINCTRL value type for batch read/modify/write operations
///
/// Use `sm0pinctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sm0pinctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Sm0PinctrlVal {
    bits: u32,
}

impl Sm0PinctrlVal {
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
        Self { bits: 0x14000000 }
    }

    /// Read **SIDESET_COUNT** `[31:29]`
    ///
    /// The number of MSBs of the Delay/Side-set instruction field which are used for side-set. Inclusive of the enable bit, if present. Minimum of 0 (all delay bits, no side-set) and maximum of 5 (all side-set, no delay).
    #[inline(always)]
    pub const fn sideset_count(&self) -> u32 {
        (self.bits & Sm0Pinctrl::SIDESET_COUNT_MASK) >> Sm0Pinctrl::SIDESET_COUNT_OFFSET
    }

    /// Read **SET_COUNT** `[28:26]`
    ///
    /// The number of pins asserted by a SET. In the range 0 to 5 inclusive.
    #[inline(always)]
    pub const fn set_count(&self) -> u32 {
        (self.bits & Sm0Pinctrl::SET_COUNT_MASK) >> Sm0Pinctrl::SET_COUNT_OFFSET
    }

    /// Read **OUT_COUNT** `[25:20]`
    ///
    /// The number of pins asserted by an OUT PINS, OUT PINDIRS or MOV PINS instruction. In the range 0 to 32 inclusive.
    #[inline(always)]
    pub const fn out_count(&self) -> u32 {
        (self.bits & Sm0Pinctrl::OUT_COUNT_MASK) >> Sm0Pinctrl::OUT_COUNT_OFFSET
    }

    /// Read **IN_BASE** `[19:15]`
    ///
    /// The pin which is mapped to the least-significant bit of a state machine's IN data bus. Higher-numbered pins are mapped to consecutively more-significant data bits, with a modulo of 32 applied to pin number.
    #[inline(always)]
    pub const fn in_base(&self) -> u32 {
        (self.bits & Sm0Pinctrl::IN_BASE_MASK) >> Sm0Pinctrl::IN_BASE_OFFSET
    }

    /// Read **SIDESET_BASE** `[14:10]`
    ///
    /// The lowest-numbered pin that will be affected by a side-set operation. The MSBs of an instruction's side-set/delay field (up to 5, determined by SIDESET_COUNT) are used for side-set data, with the remaining LSBs used for delay. The least-significant bit of the side-set portion is the bit written to this pin, with more-significant bits written to higher-numbered pins.
    #[inline(always)]
    pub const fn sideset_base(&self) -> u32 {
        (self.bits & Sm0Pinctrl::SIDESET_BASE_MASK) >> Sm0Pinctrl::SIDESET_BASE_OFFSET
    }

    /// Read **SET_BASE** `[9:5]`
    ///
    /// The lowest-numbered pin that will be affected by a SET PINS or SET PINDIRS instruction. The data written to this pin is the least-significant bit of the SET data.
    #[inline(always)]
    pub const fn set_base(&self) -> u32 {
        (self.bits & Sm0Pinctrl::SET_BASE_MASK) >> Sm0Pinctrl::SET_BASE_OFFSET
    }

    /// Read **OUT_BASE** `[4:0]`
    ///
    /// The lowest-numbered pin that will be affected by an OUT PINS, OUT PINDIRS or MOV PINS instruction. The data written to this pin will always be the least-significant bit of the OUT or MOV data.
    #[inline(always)]
    pub const fn out_base(&self) -> u32 {
        (self.bits & Sm0Pinctrl::OUT_BASE_MASK) >> Sm0Pinctrl::OUT_BASE_OFFSET
    }

    /// Set **SIDESET_COUNT** `[31:29]`
    ///
    /// The number of MSBs of the Delay/Side-set instruction field which are used for side-set. Inclusive of the enable bit, if present. Minimum of 0 (all delay bits, no side-set) and maximum of 5 (all side-set, no delay).
    #[inline(always)]
    pub const fn with_sideset_count(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sm0Pinctrl::SIDESET_COUNT_MASK) | ((value << Sm0Pinctrl::SIDESET_COUNT_OFFSET) & Sm0Pinctrl::SIDESET_COUNT_MASK)
        }
    }

    /// Set **SET_COUNT** `[28:26]`
    ///
    /// The number of pins asserted by a SET. In the range 0 to 5 inclusive.
    #[inline(always)]
    pub const fn with_set_count(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sm0Pinctrl::SET_COUNT_MASK) | ((value << Sm0Pinctrl::SET_COUNT_OFFSET) & Sm0Pinctrl::SET_COUNT_MASK)
        }
    }

    /// Set **OUT_COUNT** `[25:20]`
    ///
    /// The number of pins asserted by an OUT PINS, OUT PINDIRS or MOV PINS instruction. In the range 0 to 32 inclusive.
    #[inline(always)]
    pub const fn with_out_count(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sm0Pinctrl::OUT_COUNT_MASK) | ((value << Sm0Pinctrl::OUT_COUNT_OFFSET) & Sm0Pinctrl::OUT_COUNT_MASK)
        }
    }

    /// Set **IN_BASE** `[19:15]`
    ///
    /// The pin which is mapped to the least-significant bit of a state machine's IN data bus. Higher-numbered pins are mapped to consecutively more-significant data bits, with a modulo of 32 applied to pin number.
    #[inline(always)]
    pub const fn with_in_base(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sm0Pinctrl::IN_BASE_MASK) | ((value << Sm0Pinctrl::IN_BASE_OFFSET) & Sm0Pinctrl::IN_BASE_MASK)
        }
    }

    /// Set **SIDESET_BASE** `[14:10]`
    ///
    /// The lowest-numbered pin that will be affected by a side-set operation. The MSBs of an instruction's side-set/delay field (up to 5, determined by SIDESET_COUNT) are used for side-set data, with the remaining LSBs used for delay. The least-significant bit of the side-set portion is the bit written to this pin, with more-significant bits written to higher-numbered pins.
    #[inline(always)]
    pub const fn with_sideset_base(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sm0Pinctrl::SIDESET_BASE_MASK) | ((value << Sm0Pinctrl::SIDESET_BASE_OFFSET) & Sm0Pinctrl::SIDESET_BASE_MASK)
        }
    }

    /// Set **SET_BASE** `[9:5]`
    ///
    /// The lowest-numbered pin that will be affected by a SET PINS or SET PINDIRS instruction. The data written to this pin is the least-significant bit of the SET data.
    #[inline(always)]
    pub const fn with_set_base(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sm0Pinctrl::SET_BASE_MASK) | ((value << Sm0Pinctrl::SET_BASE_OFFSET) & Sm0Pinctrl::SET_BASE_MASK)
        }
    }

    /// Set **OUT_BASE** `[4:0]`
    ///
    /// The lowest-numbered pin that will be affected by an OUT PINS, OUT PINDIRS or MOV PINS instruction. The data written to this pin will always be the least-significant bit of the OUT or MOV data.
    #[inline(always)]
    pub const fn with_out_base(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sm0Pinctrl::OUT_BASE_MASK) | ((value << Sm0Pinctrl::OUT_BASE_OFFSET) & Sm0Pinctrl::OUT_BASE_MASK)
        }
    }

}

impl Default for Sm0PinctrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Sm0Pinctrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Sm0PinctrlVal {
        Sm0PinctrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Sm0PinctrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Sm0PinctrlVal) -> Sm0PinctrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SM1_CLKDIV Register
///
/// Clock divisor register for state machine 1
/// Frequency = clock freq / (CLKDIV_INT + CLKDIV_FRAC / 256)
///
/// Address offset: 0x00e0
/// Reset value: 0x00010000
///
/// ## Fields
///
/// - **INT** `[31:16]` (RW)
///
///   Effective frequency is sysclk/(int + frac/256).
///   Value of 0 is interpreted as 65536. If INT is 0, FRAC must also be 0.
///
/// - **FRAC** `[15:8]` (RW)
///
///   Fractional part of clock divisor
#[repr(transparent)]
pub struct Sm1Clkdiv {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Sm1Clkdiv {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sm1Clkdiv {
    // ======== Field Constants ========

    /// **INT**
    ///
    /// Effective frequency is sysclk/(int + frac/256).
    /// Value of 0 is interpreted as 65536. If INT is 0, FRAC must also be 0.
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const INT_OFFSET: u32 = 16;
    pub const INT_MASK: u32 = 0xffff0000;

    /// **FRAC**
    ///
    /// Fractional part of clock divisor
    ///
    /// Bits `[15:8]`, mask `0x0000ff00`
    pub const FRAC_OFFSET: u32 = 8;
    pub const FRAC_MASK: u32 = 0x0000ff00;

    // ======== Field Accessors ========

    /// **INT** `[31:16]` (RW)
    ///
    /// Effective frequency is sysclk/(int + frac/256).
    /// Value of 0 is interpreted as 65536. If INT is 0, FRAC must also be 0.
    #[inline(always)]
    pub fn int(&self) -> u32 {
        (self.value.read_bits() & Self::INT_MASK) >> Self::INT_OFFSET
    }

    /// Write **INT** `[31:16]` (RW)
    ///
    /// Effective frequency is sysclk/(int + frac/256).
    /// Value of 0 is interpreted as 65536. If INT is 0, FRAC must also be 0.
    #[inline(always)]
    pub fn write_int(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::INT_MASK) | ((value << Self::INT_OFFSET) & Self::INT_MASK));
    }

    /// **FRAC** `[15:8]` (RW)
    ///
    /// Fractional part of clock divisor
    #[inline(always)]
    pub fn frac(&self) -> u32 {
        (self.value.read_bits() & Self::FRAC_MASK) >> Self::FRAC_OFFSET
    }

    /// Write **FRAC** `[15:8]` (RW)
    ///
    /// Fractional part of clock divisor
    #[inline(always)]
    pub fn write_frac(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::FRAC_MASK) | ((value << Self::FRAC_OFFSET) & Self::FRAC_MASK));
    }

}

/// SM1_CLKDIV value type for batch read/modify/write operations
///
/// Use `sm1clkdiv.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sm1clkdiv.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Sm1ClkdivVal {
    bits: u32,
}

impl Sm1ClkdivVal {
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
        Self { bits: 0x00010000 }
    }

    /// Read **INT** `[31:16]`
    ///
    /// Effective frequency is sysclk/(int + frac/256).
    /// Value of 0 is interpreted as 65536. If INT is 0, FRAC must also be 0.
    #[inline(always)]
    pub const fn int(&self) -> u32 {
        (self.bits & Sm1Clkdiv::INT_MASK) >> Sm1Clkdiv::INT_OFFSET
    }

    /// Read **FRAC** `[15:8]`
    ///
    /// Fractional part of clock divisor
    #[inline(always)]
    pub const fn frac(&self) -> u32 {
        (self.bits & Sm1Clkdiv::FRAC_MASK) >> Sm1Clkdiv::FRAC_OFFSET
    }

    /// Set **INT** `[31:16]`
    ///
    /// Effective frequency is sysclk/(int + frac/256).
    /// Value of 0 is interpreted as 65536. If INT is 0, FRAC must also be 0.
    #[inline(always)]
    pub const fn with_int(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sm1Clkdiv::INT_MASK) | ((value << Sm1Clkdiv::INT_OFFSET) & Sm1Clkdiv::INT_MASK)
        }
    }

    /// Set **FRAC** `[15:8]`
    ///
    /// Fractional part of clock divisor
    #[inline(always)]
    pub const fn with_frac(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sm1Clkdiv::FRAC_MASK) | ((value << Sm1Clkdiv::FRAC_OFFSET) & Sm1Clkdiv::FRAC_MASK)
        }
    }

}

impl Default for Sm1ClkdivVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Sm1Clkdiv {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Sm1ClkdivVal {
        Sm1ClkdivVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Sm1ClkdivVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Sm1ClkdivVal) -> Sm1ClkdivVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SM1_EXECCTRL Register
///
/// Execution/behavioural settings for state machine 1
///
/// Address offset: 0x00e4
/// Reset value: 0x0001f000
///
/// ## Fields
///
/// - **EXEC_STALLED** `[31]` (RO)
///
///   If 1, an instruction written to SMx_INSTR is stalled, and latched by the state machine. Will clear to 0 once this instruction completes.
///
/// - **SIDE_EN** `[30]` (RW)
///
///   If 1, the MSB of the Delay/Side-set instruction field is used as side-set enable, rather than a side-set data bit. This allows instructions to perform side-set optionally, rather than on every instruction, but the maximum possible side-set width is reduced from 5 to 4. Note that the value of PINCTRL_SIDESET_COUNT is inclusive of this enable bit.
///
/// - **SIDE_PINDIR** `[29]` (RW)
///
///   If 1, side-set data is asserted to pin directions, instead of pin values
///
/// - **JMP_PIN** `[28:24]` (RW)
///
///   The GPIO number to use as condition for JMP PIN. Unaffected by input mapping.
///
/// - **OUT_EN_SEL** `[23:19]` (RW)
///
///   Which data bit to use for inline OUT enable
///
/// - **INLINE_OUT_EN** `[18]` (RW)
///
///   If 1, use a bit of OUT data as an auxiliary write enable
///   When used in conjunction with OUT_STICKY, writes with an enable of 0 will
///   deassert the latest pin write. This can create useful masking/override behaviour
///   due to the priority ordering of state machine pin writes (SM0 < SM1 < ...)
///
/// - **OUT_STICKY** `[17]` (RW)
///
///   Continuously assert the most recent OUT/SET to the pins
///
/// - **WRAP_TOP** `[16:12]` (RW)
///
///   After reaching this address, execution is wrapped to wrap_bottom.
///   If the instruction is a jump, and the jump condition is true, the jump takes priority.
///
/// - **WRAP_BOTTOM** `[11:7]` (RW)
///
///   After reaching wrap_top, execution is wrapped to this address.
///
/// - **STATUS_SEL** `[6:5]` (RW)
///
///   Comparison used for the MOV x, STATUS instruction.
///
/// - **STATUS_N** `[4:0]` (RW)
///
///   Comparison level or IRQ index for the MOV x, STATUS instruction.
///   If STATUS_SEL is TXLEVEL or RXLEVEL, then values of STATUS_N greater than the current FIFO depth are reserved, and have undefined behaviour.
#[repr(transparent)]
pub struct Sm1Execctrl {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Sm1Execctrl {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sm1Execctrl {
    // ======== Field Constants ========

    /// **EXEC_STALLED**
    ///
    /// If 1, an instruction written to SMx_INSTR is stalled, and latched by the state machine. Will clear to 0 once this instruction completes.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const EXEC_STALLED_OFFSET: u32 = 31;
    pub const EXEC_STALLED_MASK: u32 = 0x80000000;
    pub const EXEC_STALLED: u32 = 1 << 31;

    /// **SIDE_EN**
    ///
    /// If 1, the MSB of the Delay/Side-set instruction field is used as side-set enable, rather than a side-set data bit. This allows instructions to perform side-set optionally, rather than on every instruction, but the maximum possible side-set width is reduced from 5 to 4. Note that the value of PINCTRL_SIDESET_COUNT is inclusive of this enable bit.
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const SIDE_EN_OFFSET: u32 = 30;
    pub const SIDE_EN_MASK: u32 = 0x40000000;
    pub const SIDE_EN: u32 = 1 << 30;

    /// **SIDE_PINDIR**
    ///
    /// If 1, side-set data is asserted to pin directions, instead of pin values
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const SIDE_PINDIR_OFFSET: u32 = 29;
    pub const SIDE_PINDIR_MASK: u32 = 0x20000000;
    pub const SIDE_PINDIR: u32 = 1 << 29;

    /// **JMP_PIN**
    ///
    /// The GPIO number to use as condition for JMP PIN. Unaffected by input mapping.
    ///
    /// Bits `[28:24]`, mask `0x1f000000`
    pub const JMP_PIN_OFFSET: u32 = 24;
    pub const JMP_PIN_MASK: u32 = 0x1f000000;

    /// **OUT_EN_SEL**
    ///
    /// Which data bit to use for inline OUT enable
    ///
    /// Bits `[23:19]`, mask `0x00f80000`
    pub const OUT_EN_SEL_OFFSET: u32 = 19;
    pub const OUT_EN_SEL_MASK: u32 = 0x00f80000;

    /// **INLINE_OUT_EN**
    ///
    /// If 1, use a bit of OUT data as an auxiliary write enable
    /// When used in conjunction with OUT_STICKY, writes with an enable of 0 will
    /// deassert the latest pin write. This can create useful masking/override behaviour
    /// due to the priority ordering of state machine pin writes (SM0 < SM1 < ...)
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const INLINE_OUT_EN_OFFSET: u32 = 18;
    pub const INLINE_OUT_EN_MASK: u32 = 0x00040000;
    pub const INLINE_OUT_EN: u32 = 1 << 18;

    /// **OUT_STICKY**
    ///
    /// Continuously assert the most recent OUT/SET to the pins
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const OUT_STICKY_OFFSET: u32 = 17;
    pub const OUT_STICKY_MASK: u32 = 0x00020000;
    pub const OUT_STICKY: u32 = 1 << 17;

    /// **WRAP_TOP**
    ///
    /// After reaching this address, execution is wrapped to wrap_bottom.
    /// If the instruction is a jump, and the jump condition is true, the jump takes priority.
    ///
    /// Bits `[16:12]`, mask `0x0001f000`
    pub const WRAP_TOP_OFFSET: u32 = 12;
    pub const WRAP_TOP_MASK: u32 = 0x0001f000;

    /// **WRAP_BOTTOM**
    ///
    /// After reaching wrap_top, execution is wrapped to this address.
    ///
    /// Bits `[11:7]`, mask `0x00000f80`
    pub const WRAP_BOTTOM_OFFSET: u32 = 7;
    pub const WRAP_BOTTOM_MASK: u32 = 0x00000f80;

    /// **STATUS_SEL**
    ///
    /// Comparison used for the MOV x, STATUS instruction.
    ///
    /// Bits `[6:5]`, mask `0x00000060`
    pub const STATUS_SEL_OFFSET: u32 = 5;
    pub const STATUS_SEL_MASK: u32 = 0x00000060;
    /// TXLEVEL = 0
    /// All-ones if TX FIFO level < N, otherwise all-zeroes
    pub const STATUS_SEL_TXLEVEL: u32 = 0 << 5;
    pub const STATUS_SEL_TXLEVEL_VAL: u32 = 0;
    /// RXLEVEL = 1
    /// All-ones if RX FIFO level < N, otherwise all-zeroes
    pub const STATUS_SEL_RXLEVEL: u32 = 1 << 5;
    pub const STATUS_SEL_RXLEVEL_VAL: u32 = 1;
    /// IRQ = 2
    /// All-ones if the indexed IRQ flag is raised, otherwise all-zeroes
    pub const STATUS_SEL_IRQ: u32 = 2 << 5;
    pub const STATUS_SEL_IRQ_VAL: u32 = 2;

    /// **STATUS_N**
    ///
    /// Comparison level or IRQ index for the MOV x, STATUS instruction.
    /// If STATUS_SEL is TXLEVEL or RXLEVEL, then values of STATUS_N greater than the current FIFO depth are reserved, and have undefined behaviour.
    ///
    /// Bits `[4:0]`, mask `0x0000001f`
    pub const STATUS_N_OFFSET: u32 = 0;
    pub const STATUS_N_MASK: u32 = 0x0000001f;
    /// IRQ = 0
    /// Index 0-7 of an IRQ flag in this PIO block
    pub const STATUS_N_IRQ: u32 = 0 << 0;
    pub const STATUS_N_IRQ_VAL: u32 = 0;
    /// IRQ_PREVPIO = 8
    /// Index 0-7 of an IRQ flag in the next lower-numbered PIO block
    pub const STATUS_N_IRQ_PREVPIO: u32 = 8 << 0;
    pub const STATUS_N_IRQ_PREVPIO_VAL: u32 = 8;
    /// IRQ_NEXTPIO = 16
    /// Index 0-7 of an IRQ flag in the next higher-numbered PIO block
    pub const STATUS_N_IRQ_NEXTPIO: u32 = 16 << 0;
    pub const STATUS_N_IRQ_NEXTPIO_VAL: u32 = 16;

    // ======== Field Accessors ========

    /// **EXEC_STALLED** `[31]` (RO)
    ///
    /// If 1, an instruction written to SMx_INSTR is stalled, and latched by the state machine. Will clear to 0 once this instruction completes.
    #[inline(always)]
    pub fn exec_stalled(&self) -> bool {
        (self.value.read_bits() & Self::EXEC_STALLED_MASK) != 0
    }

    /// **SIDE_EN** `[30]` (RW)
    ///
    /// If 1, the MSB of the Delay/Side-set instruction field is used as side-set enable, rather than a side-set data bit. This allows instructions to perform side-set optionally, rather than on every instruction, but the maximum possible side-set width is reduced from 5 to 4. Note that the value of PINCTRL_SIDESET_COUNT is inclusive of this enable bit.
    #[inline(always)]
    pub fn side_en(&self) -> bool {
        (self.value.read_bits() & Self::SIDE_EN_MASK) != 0
    }

    /// Set SIDE_EN bit atomically (RW)
    #[inline(always)]
    pub fn set_side_en(&self) {
        self.set_bits(Self::SIDE_EN_MASK);
    }

    /// Clear SIDE_EN bit atomically (RW)
    #[inline(always)]
    pub fn clr_side_en(&self) {
        self.clr_bits(Self::SIDE_EN_MASK);
    }

    /// **SIDE_PINDIR** `[29]` (RW)
    ///
    /// If 1, side-set data is asserted to pin directions, instead of pin values
    #[inline(always)]
    pub fn side_pindir(&self) -> bool {
        (self.value.read_bits() & Self::SIDE_PINDIR_MASK) != 0
    }

    /// Set SIDE_PINDIR bit atomically (RW)
    #[inline(always)]
    pub fn set_side_pindir(&self) {
        self.set_bits(Self::SIDE_PINDIR_MASK);
    }

    /// Clear SIDE_PINDIR bit atomically (RW)
    #[inline(always)]
    pub fn clr_side_pindir(&self) {
        self.clr_bits(Self::SIDE_PINDIR_MASK);
    }

    /// **JMP_PIN** `[28:24]` (RW)
    ///
    /// The GPIO number to use as condition for JMP PIN. Unaffected by input mapping.
    #[inline(always)]
    pub fn jmp_pin(&self) -> u32 {
        (self.value.read_bits() & Self::JMP_PIN_MASK) >> Self::JMP_PIN_OFFSET
    }

    /// Write **JMP_PIN** `[28:24]` (RW)
    ///
    /// The GPIO number to use as condition for JMP PIN. Unaffected by input mapping.
    #[inline(always)]
    pub fn write_jmp_pin(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::JMP_PIN_MASK) | ((value << Self::JMP_PIN_OFFSET) & Self::JMP_PIN_MASK));
    }

    /// **OUT_EN_SEL** `[23:19]` (RW)
    ///
    /// Which data bit to use for inline OUT enable
    #[inline(always)]
    pub fn out_en_sel(&self) -> u32 {
        (self.value.read_bits() & Self::OUT_EN_SEL_MASK) >> Self::OUT_EN_SEL_OFFSET
    }

    /// Write **OUT_EN_SEL** `[23:19]` (RW)
    ///
    /// Which data bit to use for inline OUT enable
    #[inline(always)]
    pub fn write_out_en_sel(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::OUT_EN_SEL_MASK) | ((value << Self::OUT_EN_SEL_OFFSET) & Self::OUT_EN_SEL_MASK));
    }

    /// **INLINE_OUT_EN** `[18]` (RW)
    ///
    /// If 1, use a bit of OUT data as an auxiliary write enable
    /// When used in conjunction with OUT_STICKY, writes with an enable of 0 will
    /// deassert the latest pin write. This can create useful masking/override behaviour
    /// due to the priority ordering of state machine pin writes (SM0 < SM1 < ...)
    #[inline(always)]
    pub fn inline_out_en(&self) -> bool {
        (self.value.read_bits() & Self::INLINE_OUT_EN_MASK) != 0
    }

    /// Set INLINE_OUT_EN bit atomically (RW)
    #[inline(always)]
    pub fn set_inline_out_en(&self) {
        self.set_bits(Self::INLINE_OUT_EN_MASK);
    }

    /// Clear INLINE_OUT_EN bit atomically (RW)
    #[inline(always)]
    pub fn clr_inline_out_en(&self) {
        self.clr_bits(Self::INLINE_OUT_EN_MASK);
    }

    /// **OUT_STICKY** `[17]` (RW)
    ///
    /// Continuously assert the most recent OUT/SET to the pins
    #[inline(always)]
    pub fn out_sticky(&self) -> bool {
        (self.value.read_bits() & Self::OUT_STICKY_MASK) != 0
    }

    /// Set OUT_STICKY bit atomically (RW)
    #[inline(always)]
    pub fn set_out_sticky(&self) {
        self.set_bits(Self::OUT_STICKY_MASK);
    }

    /// Clear OUT_STICKY bit atomically (RW)
    #[inline(always)]
    pub fn clr_out_sticky(&self) {
        self.clr_bits(Self::OUT_STICKY_MASK);
    }

    /// **WRAP_TOP** `[16:12]` (RW)
    ///
    /// After reaching this address, execution is wrapped to wrap_bottom.
    /// If the instruction is a jump, and the jump condition is true, the jump takes priority.
    #[inline(always)]
    pub fn wrap_top(&self) -> u32 {
        (self.value.read_bits() & Self::WRAP_TOP_MASK) >> Self::WRAP_TOP_OFFSET
    }

    /// Write **WRAP_TOP** `[16:12]` (RW)
    ///
    /// After reaching this address, execution is wrapped to wrap_bottom.
    /// If the instruction is a jump, and the jump condition is true, the jump takes priority.
    #[inline(always)]
    pub fn write_wrap_top(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::WRAP_TOP_MASK) | ((value << Self::WRAP_TOP_OFFSET) & Self::WRAP_TOP_MASK));
    }

    /// **WRAP_BOTTOM** `[11:7]` (RW)
    ///
    /// After reaching wrap_top, execution is wrapped to this address.
    #[inline(always)]
    pub fn wrap_bottom(&self) -> u32 {
        (self.value.read_bits() & Self::WRAP_BOTTOM_MASK) >> Self::WRAP_BOTTOM_OFFSET
    }

    /// Write **WRAP_BOTTOM** `[11:7]` (RW)
    ///
    /// After reaching wrap_top, execution is wrapped to this address.
    #[inline(always)]
    pub fn write_wrap_bottom(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::WRAP_BOTTOM_MASK) | ((value << Self::WRAP_BOTTOM_OFFSET) & Self::WRAP_BOTTOM_MASK));
    }

    /// **STATUS_SEL** `[6:5]` (RW)
    ///
    /// Comparison used for the MOV x, STATUS instruction.
    #[inline(always)]
    pub fn status_sel(&self) -> u32 {
        (self.value.read_bits() & Self::STATUS_SEL_MASK) >> Self::STATUS_SEL_OFFSET
    }

    /// Write **STATUS_SEL** `[6:5]` (RW)
    ///
    /// Comparison used for the MOV x, STATUS instruction.
    #[inline(always)]
    pub fn write_status_sel(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::STATUS_SEL_MASK) | ((value << Self::STATUS_SEL_OFFSET) & Self::STATUS_SEL_MASK));
    }

    /// **STATUS_N** `[4:0]` (RW)
    ///
    /// Comparison level or IRQ index for the MOV x, STATUS instruction.
    /// If STATUS_SEL is TXLEVEL or RXLEVEL, then values of STATUS_N greater than the current FIFO depth are reserved, and have undefined behaviour.
    #[inline(always)]
    pub fn status_n(&self) -> u32 {
        (self.value.read_bits() & Self::STATUS_N_MASK) >> Self::STATUS_N_OFFSET
    }

    /// Write **STATUS_N** `[4:0]` (RW)
    ///
    /// Comparison level or IRQ index for the MOV x, STATUS instruction.
    /// If STATUS_SEL is TXLEVEL or RXLEVEL, then values of STATUS_N greater than the current FIFO depth are reserved, and have undefined behaviour.
    #[inline(always)]
    pub fn write_status_n(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::STATUS_N_MASK) | ((value << Self::STATUS_N_OFFSET) & Self::STATUS_N_MASK));
    }

}

/// SM1_EXECCTRL value type for batch read/modify/write operations
///
/// Use `sm1execctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sm1execctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Sm1ExecctrlVal {
    bits: u32,
}

impl Sm1ExecctrlVal {
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
        Self { bits: 0x0001f000 }
    }

    /// Read **EXEC_STALLED** `[31:31]`
    ///
    /// If 1, an instruction written to SMx_INSTR is stalled, and latched by the state machine. Will clear to 0 once this instruction completes.
    #[inline(always)]
    pub const fn exec_stalled(&self) -> bool {
        (self.bits & Sm1Execctrl::EXEC_STALLED_MASK) != 0
    }

    /// Read **SIDE_EN** `[30:30]`
    ///
    /// If 1, the MSB of the Delay/Side-set instruction field is used as side-set enable, rather than a side-set data bit. This allows instructions to perform side-set optionally, rather than on every instruction, but the maximum possible side-set width is reduced from 5 to 4. Note that the value of PINCTRL_SIDESET_COUNT is inclusive of this enable bit.
    #[inline(always)]
    pub const fn side_en(&self) -> bool {
        (self.bits & Sm1Execctrl::SIDE_EN_MASK) != 0
    }

    /// Read **SIDE_PINDIR** `[29:29]`
    ///
    /// If 1, side-set data is asserted to pin directions, instead of pin values
    #[inline(always)]
    pub const fn side_pindir(&self) -> bool {
        (self.bits & Sm1Execctrl::SIDE_PINDIR_MASK) != 0
    }

    /// Read **JMP_PIN** `[28:24]`
    ///
    /// The GPIO number to use as condition for JMP PIN. Unaffected by input mapping.
    #[inline(always)]
    pub const fn jmp_pin(&self) -> u32 {
        (self.bits & Sm1Execctrl::JMP_PIN_MASK) >> Sm1Execctrl::JMP_PIN_OFFSET
    }

    /// Read **OUT_EN_SEL** `[23:19]`
    ///
    /// Which data bit to use for inline OUT enable
    #[inline(always)]
    pub const fn out_en_sel(&self) -> u32 {
        (self.bits & Sm1Execctrl::OUT_EN_SEL_MASK) >> Sm1Execctrl::OUT_EN_SEL_OFFSET
    }

    /// Read **INLINE_OUT_EN** `[18:18]`
    ///
    /// If 1, use a bit of OUT data as an auxiliary write enable
    /// When used in conjunction with OUT_STICKY, writes with an enable of 0 will
    /// deassert the latest pin write. This can create useful masking/override behaviour
    /// due to the priority ordering of state machine pin writes (SM0 < SM1 < ...)
    #[inline(always)]
    pub const fn inline_out_en(&self) -> bool {
        (self.bits & Sm1Execctrl::INLINE_OUT_EN_MASK) != 0
    }

    /// Read **OUT_STICKY** `[17:17]`
    ///
    /// Continuously assert the most recent OUT/SET to the pins
    #[inline(always)]
    pub const fn out_sticky(&self) -> bool {
        (self.bits & Sm1Execctrl::OUT_STICKY_MASK) != 0
    }

    /// Read **WRAP_TOP** `[16:12]`
    ///
    /// After reaching this address, execution is wrapped to wrap_bottom.
    /// If the instruction is a jump, and the jump condition is true, the jump takes priority.
    #[inline(always)]
    pub const fn wrap_top(&self) -> u32 {
        (self.bits & Sm1Execctrl::WRAP_TOP_MASK) >> Sm1Execctrl::WRAP_TOP_OFFSET
    }

    /// Read **WRAP_BOTTOM** `[11:7]`
    ///
    /// After reaching wrap_top, execution is wrapped to this address.
    #[inline(always)]
    pub const fn wrap_bottom(&self) -> u32 {
        (self.bits & Sm1Execctrl::WRAP_BOTTOM_MASK) >> Sm1Execctrl::WRAP_BOTTOM_OFFSET
    }

    /// Read **STATUS_SEL** `[6:5]`
    ///
    /// Comparison used for the MOV x, STATUS instruction.
    #[inline(always)]
    pub const fn status_sel(&self) -> u32 {
        (self.bits & Sm1Execctrl::STATUS_SEL_MASK) >> Sm1Execctrl::STATUS_SEL_OFFSET
    }

    /// Read **STATUS_N** `[4:0]`
    ///
    /// Comparison level or IRQ index for the MOV x, STATUS instruction.
    /// If STATUS_SEL is TXLEVEL or RXLEVEL, then values of STATUS_N greater than the current FIFO depth are reserved, and have undefined behaviour.
    #[inline(always)]
    pub const fn status_n(&self) -> u32 {
        (self.bits & Sm1Execctrl::STATUS_N_MASK) >> Sm1Execctrl::STATUS_N_OFFSET
    }

    /// Set **SIDE_EN** `[30:30]`
    ///
    /// If 1, the MSB of the Delay/Side-set instruction field is used as side-set enable, rather than a side-set data bit. This allows instructions to perform side-set optionally, rather than on every instruction, but the maximum possible side-set width is reduced from 5 to 4. Note that the value of PINCTRL_SIDESET_COUNT is inclusive of this enable bit.
    #[inline(always)]
    pub const fn with_side_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sm1Execctrl::SIDE_EN_MASK
            } else {
                self.bits & !Sm1Execctrl::SIDE_EN_MASK
            }
        }
    }

    /// Set **SIDE_PINDIR** `[29:29]`
    ///
    /// If 1, side-set data is asserted to pin directions, instead of pin values
    #[inline(always)]
    pub const fn with_side_pindir(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sm1Execctrl::SIDE_PINDIR_MASK
            } else {
                self.bits & !Sm1Execctrl::SIDE_PINDIR_MASK
            }
        }
    }

    /// Set **JMP_PIN** `[28:24]`
    ///
    /// The GPIO number to use as condition for JMP PIN. Unaffected by input mapping.
    #[inline(always)]
    pub const fn with_jmp_pin(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sm1Execctrl::JMP_PIN_MASK) | ((value << Sm1Execctrl::JMP_PIN_OFFSET) & Sm1Execctrl::JMP_PIN_MASK)
        }
    }

    /// Set **OUT_EN_SEL** `[23:19]`
    ///
    /// Which data bit to use for inline OUT enable
    #[inline(always)]
    pub const fn with_out_en_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sm1Execctrl::OUT_EN_SEL_MASK) | ((value << Sm1Execctrl::OUT_EN_SEL_OFFSET) & Sm1Execctrl::OUT_EN_SEL_MASK)
        }
    }

    /// Set **INLINE_OUT_EN** `[18:18]`
    ///
    /// If 1, use a bit of OUT data as an auxiliary write enable
    /// When used in conjunction with OUT_STICKY, writes with an enable of 0 will
    /// deassert the latest pin write. This can create useful masking/override behaviour
    /// due to the priority ordering of state machine pin writes (SM0 < SM1 < ...)
    #[inline(always)]
    pub const fn with_inline_out_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sm1Execctrl::INLINE_OUT_EN_MASK
            } else {
                self.bits & !Sm1Execctrl::INLINE_OUT_EN_MASK
            }
        }
    }

    /// Set **OUT_STICKY** `[17:17]`
    ///
    /// Continuously assert the most recent OUT/SET to the pins
    #[inline(always)]
    pub const fn with_out_sticky(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sm1Execctrl::OUT_STICKY_MASK
            } else {
                self.bits & !Sm1Execctrl::OUT_STICKY_MASK
            }
        }
    }

    /// Set **WRAP_TOP** `[16:12]`
    ///
    /// After reaching this address, execution is wrapped to wrap_bottom.
    /// If the instruction is a jump, and the jump condition is true, the jump takes priority.
    #[inline(always)]
    pub const fn with_wrap_top(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sm1Execctrl::WRAP_TOP_MASK) | ((value << Sm1Execctrl::WRAP_TOP_OFFSET) & Sm1Execctrl::WRAP_TOP_MASK)
        }
    }

    /// Set **WRAP_BOTTOM** `[11:7]`
    ///
    /// After reaching wrap_top, execution is wrapped to this address.
    #[inline(always)]
    pub const fn with_wrap_bottom(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sm1Execctrl::WRAP_BOTTOM_MASK) | ((value << Sm1Execctrl::WRAP_BOTTOM_OFFSET) & Sm1Execctrl::WRAP_BOTTOM_MASK)
        }
    }

    /// Set **STATUS_SEL** `[6:5]`
    ///
    /// Comparison used for the MOV x, STATUS instruction.
    #[inline(always)]
    pub const fn with_status_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sm1Execctrl::STATUS_SEL_MASK) | ((value << Sm1Execctrl::STATUS_SEL_OFFSET) & Sm1Execctrl::STATUS_SEL_MASK)
        }
    }

    /// Set **STATUS_N** `[4:0]`
    ///
    /// Comparison level or IRQ index for the MOV x, STATUS instruction.
    /// If STATUS_SEL is TXLEVEL or RXLEVEL, then values of STATUS_N greater than the current FIFO depth are reserved, and have undefined behaviour.
    #[inline(always)]
    pub const fn with_status_n(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sm1Execctrl::STATUS_N_MASK) | ((value << Sm1Execctrl::STATUS_N_OFFSET) & Sm1Execctrl::STATUS_N_MASK)
        }
    }

}

impl Default for Sm1ExecctrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Sm1Execctrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Sm1ExecctrlVal {
        Sm1ExecctrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Sm1ExecctrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Sm1ExecctrlVal) -> Sm1ExecctrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SM1_SHIFTCTRL Register
///
/// Control behaviour of the input/output shift registers for state machine 1
///
/// Address offset: 0x00e8
/// Reset value: 0x000c0000
///
/// ## Fields
///
/// - **FJOIN_RX** `[31]` (RW)
///
///   When 1, RX FIFO steals the TX FIFO's storage, and becomes twice as deep.
///   TX FIFO is disabled as a result (always reads as both full and empty).
///   FIFOs are flushed when this bit is changed.
///
/// - **FJOIN_TX** `[30]` (RW)
///
///   When 1, TX FIFO steals the RX FIFO's storage, and becomes twice as deep.
///   RX FIFO is disabled as a result (always reads as both full and empty).
///   FIFOs are flushed when this bit is changed.
///
/// - **PULL_THRESH** `[29:25]` (RW)
///
///   Number of bits shifted out of OSR before autopull, or conditional pull (PULL IFEMPTY), will take place.
///   Write 0 for value of 32.
///
/// - **PUSH_THRESH** `[24:20]` (RW)
///
///   Number of bits shifted into ISR before autopush, or conditional push (PUSH IFFULL), will take place.
///   Write 0 for value of 32.
///
/// - **OUT_SHIFTDIR** `[19]` (RW)
///
///   1 = shift out of output shift register to right. 0 = to left.
///
/// - **IN_SHIFTDIR** `[18]` (RW)
///
///   1 = shift input shift register to right (data enters from left). 0 = to left.
///
/// - **AUTOPULL** `[17]` (RW)
///
///   Pull automatically when the output shift register is emptied, i.e. on or following an OUT instruction which causes the output shift counter to reach or exceed PULL_THRESH.
///
/// - **AUTOPUSH** `[16]` (RW)
///
///   Push automatically when the input shift register is filled, i.e. on an IN instruction which causes the input shift counter to reach or exceed PUSH_THRESH.
///
/// - **FJOIN_RX_PUT** `[15]` (RW)
///
///   If 1, disable this state machine's RX FIFO, make its storage available for random write access by the state machine (using the `put` instruction) and, unless FJOIN_RX_GET is also set, random read access by the processor (through the RXFx_PUTGETy registers).
///   If FJOIN_RX_PUT and FJOIN_RX_GET are both set, then the RX FIFO's registers can be randomly read/written by the state machine, but are completely inaccessible to the processor.
///   Setting this bit will clear the FJOIN_TX and FJOIN_RX bits.
///
/// - **FJOIN_RX_GET** `[14]` (RW)
///
///   If 1, disable this state machine's RX FIFO, make its storage available for random read access by the state machine (using the `get` instruction) and, unless FJOIN_RX_PUT is also set, random write access by the processor (through the RXFx_PUTGETy registers).
///   If FJOIN_RX_PUT and FJOIN_RX_GET are both set, then the RX FIFO's registers can be randomly read/written by the state machine, but are completely inaccessible to the processor.
///   Setting this bit will clear the FJOIN_TX and FJOIN_RX bits.
///
/// - **IN_COUNT** `[4:0]` (RW)
///
///   Set the number of pins which are not masked to 0 when read by an IN PINS, WAIT PIN or MOV x, PINS instruction.
///   For example, an IN_COUNT of 5 means that the 5 LSBs of the IN pin group are visible (bits 4:0), but the remaining 27 MSBs are masked to 0. A count of 32 is encoded with a field value of 0, so the default behaviour is to not perform any masking.
///   Note this masking is applied in addition to the masking usually performed by the IN instruction. This is mainly useful for the MOV x, PINS instruction, which otherwise has no way of masking pins.
#[repr(transparent)]
pub struct Sm1Shiftctrl {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Sm1Shiftctrl {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sm1Shiftctrl {
    // ======== Field Constants ========

    /// **FJOIN_RX**
    ///
    /// When 1, RX FIFO steals the TX FIFO's storage, and becomes twice as deep.
    /// TX FIFO is disabled as a result (always reads as both full and empty).
    /// FIFOs are flushed when this bit is changed.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const FJOIN_RX_OFFSET: u32 = 31;
    pub const FJOIN_RX_MASK: u32 = 0x80000000;
    pub const FJOIN_RX: u32 = 1 << 31;

    /// **FJOIN_TX**
    ///
    /// When 1, TX FIFO steals the RX FIFO's storage, and becomes twice as deep.
    /// RX FIFO is disabled as a result (always reads as both full and empty).
    /// FIFOs are flushed when this bit is changed.
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const FJOIN_TX_OFFSET: u32 = 30;
    pub const FJOIN_TX_MASK: u32 = 0x40000000;
    pub const FJOIN_TX: u32 = 1 << 30;

    /// **PULL_THRESH**
    ///
    /// Number of bits shifted out of OSR before autopull, or conditional pull (PULL IFEMPTY), will take place.
    /// Write 0 for value of 32.
    ///
    /// Bits `[29:25]`, mask `0x3e000000`
    pub const PULL_THRESH_OFFSET: u32 = 25;
    pub const PULL_THRESH_MASK: u32 = 0x3e000000;

    /// **PUSH_THRESH**
    ///
    /// Number of bits shifted into ISR before autopush, or conditional push (PUSH IFFULL), will take place.
    /// Write 0 for value of 32.
    ///
    /// Bits `[24:20]`, mask `0x01f00000`
    pub const PUSH_THRESH_OFFSET: u32 = 20;
    pub const PUSH_THRESH_MASK: u32 = 0x01f00000;

    /// **OUT_SHIFTDIR**
    ///
    /// 1 = shift out of output shift register to right. 0 = to left.
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const OUT_SHIFTDIR_OFFSET: u32 = 19;
    pub const OUT_SHIFTDIR_MASK: u32 = 0x00080000;
    pub const OUT_SHIFTDIR: u32 = 1 << 19;

    /// **IN_SHIFTDIR**
    ///
    /// 1 = shift input shift register to right (data enters from left). 0 = to left.
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const IN_SHIFTDIR_OFFSET: u32 = 18;
    pub const IN_SHIFTDIR_MASK: u32 = 0x00040000;
    pub const IN_SHIFTDIR: u32 = 1 << 18;

    /// **AUTOPULL**
    ///
    /// Pull automatically when the output shift register is emptied, i.e. on or following an OUT instruction which causes the output shift counter to reach or exceed PULL_THRESH.
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const AUTOPULL_OFFSET: u32 = 17;
    pub const AUTOPULL_MASK: u32 = 0x00020000;
    pub const AUTOPULL: u32 = 1 << 17;

    /// **AUTOPUSH**
    ///
    /// Push automatically when the input shift register is filled, i.e. on an IN instruction which causes the input shift counter to reach or exceed PUSH_THRESH.
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const AUTOPUSH_OFFSET: u32 = 16;
    pub const AUTOPUSH_MASK: u32 = 0x00010000;
    pub const AUTOPUSH: u32 = 1 << 16;

    /// **FJOIN_RX_PUT**
    ///
    /// If 1, disable this state machine's RX FIFO, make its storage available for random write access by the state machine (using the `put` instruction) and, unless FJOIN_RX_GET is also set, random read access by the processor (through the RXFx_PUTGETy registers).
    /// If FJOIN_RX_PUT and FJOIN_RX_GET are both set, then the RX FIFO's registers can be randomly read/written by the state machine, but are completely inaccessible to the processor.
    /// Setting this bit will clear the FJOIN_TX and FJOIN_RX bits.
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const FJOIN_RX_PUT_OFFSET: u32 = 15;
    pub const FJOIN_RX_PUT_MASK: u32 = 0x00008000;
    pub const FJOIN_RX_PUT: u32 = 1 << 15;

    /// **FJOIN_RX_GET**
    ///
    /// If 1, disable this state machine's RX FIFO, make its storage available for random read access by the state machine (using the `get` instruction) and, unless FJOIN_RX_PUT is also set, random write access by the processor (through the RXFx_PUTGETy registers).
    /// If FJOIN_RX_PUT and FJOIN_RX_GET are both set, then the RX FIFO's registers can be randomly read/written by the state machine, but are completely inaccessible to the processor.
    /// Setting this bit will clear the FJOIN_TX and FJOIN_RX bits.
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const FJOIN_RX_GET_OFFSET: u32 = 14;
    pub const FJOIN_RX_GET_MASK: u32 = 0x00004000;
    pub const FJOIN_RX_GET: u32 = 1 << 14;

    /// **IN_COUNT**
    ///
    /// Set the number of pins which are not masked to 0 when read by an IN PINS, WAIT PIN or MOV x, PINS instruction.
    /// For example, an IN_COUNT of 5 means that the 5 LSBs of the IN pin group are visible (bits 4:0), but the remaining 27 MSBs are masked to 0. A count of 32 is encoded with a field value of 0, so the default behaviour is to not perform any masking.
    /// Note this masking is applied in addition to the masking usually performed by the IN instruction. This is mainly useful for the MOV x, PINS instruction, which otherwise has no way of masking pins.
    ///
    /// Bits `[4:0]`, mask `0x0000001f`
    pub const IN_COUNT_OFFSET: u32 = 0;
    pub const IN_COUNT_MASK: u32 = 0x0000001f;

    // ======== Field Accessors ========

    /// **FJOIN_RX** `[31]` (RW)
    ///
    /// When 1, RX FIFO steals the TX FIFO's storage, and becomes twice as deep.
    /// TX FIFO is disabled as a result (always reads as both full and empty).
    /// FIFOs are flushed when this bit is changed.
    #[inline(always)]
    pub fn fjoin_rx(&self) -> bool {
        (self.value.read_bits() & Self::FJOIN_RX_MASK) != 0
    }

    /// Set FJOIN_RX bit atomically (RW)
    #[inline(always)]
    pub fn set_fjoin_rx(&self) {
        self.set_bits(Self::FJOIN_RX_MASK);
    }

    /// Clear FJOIN_RX bit atomically (RW)
    #[inline(always)]
    pub fn clr_fjoin_rx(&self) {
        self.clr_bits(Self::FJOIN_RX_MASK);
    }

    /// **FJOIN_TX** `[30]` (RW)
    ///
    /// When 1, TX FIFO steals the RX FIFO's storage, and becomes twice as deep.
    /// RX FIFO is disabled as a result (always reads as both full and empty).
    /// FIFOs are flushed when this bit is changed.
    #[inline(always)]
    pub fn fjoin_tx(&self) -> bool {
        (self.value.read_bits() & Self::FJOIN_TX_MASK) != 0
    }

    /// Set FJOIN_TX bit atomically (RW)
    #[inline(always)]
    pub fn set_fjoin_tx(&self) {
        self.set_bits(Self::FJOIN_TX_MASK);
    }

    /// Clear FJOIN_TX bit atomically (RW)
    #[inline(always)]
    pub fn clr_fjoin_tx(&self) {
        self.clr_bits(Self::FJOIN_TX_MASK);
    }

    /// **PULL_THRESH** `[29:25]` (RW)
    ///
    /// Number of bits shifted out of OSR before autopull, or conditional pull (PULL IFEMPTY), will take place.
    /// Write 0 for value of 32.
    #[inline(always)]
    pub fn pull_thresh(&self) -> u32 {
        (self.value.read_bits() & Self::PULL_THRESH_MASK) >> Self::PULL_THRESH_OFFSET
    }

    /// Write **PULL_THRESH** `[29:25]` (RW)
    ///
    /// Number of bits shifted out of OSR before autopull, or conditional pull (PULL IFEMPTY), will take place.
    /// Write 0 for value of 32.
    #[inline(always)]
    pub fn write_pull_thresh(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::PULL_THRESH_MASK) | ((value << Self::PULL_THRESH_OFFSET) & Self::PULL_THRESH_MASK));
    }

    /// **PUSH_THRESH** `[24:20]` (RW)
    ///
    /// Number of bits shifted into ISR before autopush, or conditional push (PUSH IFFULL), will take place.
    /// Write 0 for value of 32.
    #[inline(always)]
    pub fn push_thresh(&self) -> u32 {
        (self.value.read_bits() & Self::PUSH_THRESH_MASK) >> Self::PUSH_THRESH_OFFSET
    }

    /// Write **PUSH_THRESH** `[24:20]` (RW)
    ///
    /// Number of bits shifted into ISR before autopush, or conditional push (PUSH IFFULL), will take place.
    /// Write 0 for value of 32.
    #[inline(always)]
    pub fn write_push_thresh(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::PUSH_THRESH_MASK) | ((value << Self::PUSH_THRESH_OFFSET) & Self::PUSH_THRESH_MASK));
    }

    /// **OUT_SHIFTDIR** `[19]` (RW)
    ///
    /// 1 = shift out of output shift register to right. 0 = to left.
    #[inline(always)]
    pub fn out_shiftdir(&self) -> bool {
        (self.value.read_bits() & Self::OUT_SHIFTDIR_MASK) != 0
    }

    /// Set OUT_SHIFTDIR bit atomically (RW)
    #[inline(always)]
    pub fn set_out_shiftdir(&self) {
        self.set_bits(Self::OUT_SHIFTDIR_MASK);
    }

    /// Clear OUT_SHIFTDIR bit atomically (RW)
    #[inline(always)]
    pub fn clr_out_shiftdir(&self) {
        self.clr_bits(Self::OUT_SHIFTDIR_MASK);
    }

    /// **IN_SHIFTDIR** `[18]` (RW)
    ///
    /// 1 = shift input shift register to right (data enters from left). 0 = to left.
    #[inline(always)]
    pub fn in_shiftdir(&self) -> bool {
        (self.value.read_bits() & Self::IN_SHIFTDIR_MASK) != 0
    }

    /// Set IN_SHIFTDIR bit atomically (RW)
    #[inline(always)]
    pub fn set_in_shiftdir(&self) {
        self.set_bits(Self::IN_SHIFTDIR_MASK);
    }

    /// Clear IN_SHIFTDIR bit atomically (RW)
    #[inline(always)]
    pub fn clr_in_shiftdir(&self) {
        self.clr_bits(Self::IN_SHIFTDIR_MASK);
    }

    /// **AUTOPULL** `[17]` (RW)
    ///
    /// Pull automatically when the output shift register is emptied, i.e. on or following an OUT instruction which causes the output shift counter to reach or exceed PULL_THRESH.
    #[inline(always)]
    pub fn autopull(&self) -> bool {
        (self.value.read_bits() & Self::AUTOPULL_MASK) != 0
    }

    /// Set AUTOPULL bit atomically (RW)
    #[inline(always)]
    pub fn set_autopull(&self) {
        self.set_bits(Self::AUTOPULL_MASK);
    }

    /// Clear AUTOPULL bit atomically (RW)
    #[inline(always)]
    pub fn clr_autopull(&self) {
        self.clr_bits(Self::AUTOPULL_MASK);
    }

    /// **AUTOPUSH** `[16]` (RW)
    ///
    /// Push automatically when the input shift register is filled, i.e. on an IN instruction which causes the input shift counter to reach or exceed PUSH_THRESH.
    #[inline(always)]
    pub fn autopush(&self) -> bool {
        (self.value.read_bits() & Self::AUTOPUSH_MASK) != 0
    }

    /// Set AUTOPUSH bit atomically (RW)
    #[inline(always)]
    pub fn set_autopush(&self) {
        self.set_bits(Self::AUTOPUSH_MASK);
    }

    /// Clear AUTOPUSH bit atomically (RW)
    #[inline(always)]
    pub fn clr_autopush(&self) {
        self.clr_bits(Self::AUTOPUSH_MASK);
    }

    /// **FJOIN_RX_PUT** `[15]` (RW)
    ///
    /// If 1, disable this state machine's RX FIFO, make its storage available for random write access by the state machine (using the `put` instruction) and, unless FJOIN_RX_GET is also set, random read access by the processor (through the RXFx_PUTGETy registers).
    /// If FJOIN_RX_PUT and FJOIN_RX_GET are both set, then the RX FIFO's registers can be randomly read/written by the state machine, but are completely inaccessible to the processor.
    /// Setting this bit will clear the FJOIN_TX and FJOIN_RX bits.
    #[inline(always)]
    pub fn fjoin_rx_put(&self) -> bool {
        (self.value.read_bits() & Self::FJOIN_RX_PUT_MASK) != 0
    }

    /// Set FJOIN_RX_PUT bit atomically (RW)
    #[inline(always)]
    pub fn set_fjoin_rx_put(&self) {
        self.set_bits(Self::FJOIN_RX_PUT_MASK);
    }

    /// Clear FJOIN_RX_PUT bit atomically (RW)
    #[inline(always)]
    pub fn clr_fjoin_rx_put(&self) {
        self.clr_bits(Self::FJOIN_RX_PUT_MASK);
    }

    /// **FJOIN_RX_GET** `[14]` (RW)
    ///
    /// If 1, disable this state machine's RX FIFO, make its storage available for random read access by the state machine (using the `get` instruction) and, unless FJOIN_RX_PUT is also set, random write access by the processor (through the RXFx_PUTGETy registers).
    /// If FJOIN_RX_PUT and FJOIN_RX_GET are both set, then the RX FIFO's registers can be randomly read/written by the state machine, but are completely inaccessible to the processor.
    /// Setting this bit will clear the FJOIN_TX and FJOIN_RX bits.
    #[inline(always)]
    pub fn fjoin_rx_get(&self) -> bool {
        (self.value.read_bits() & Self::FJOIN_RX_GET_MASK) != 0
    }

    /// Set FJOIN_RX_GET bit atomically (RW)
    #[inline(always)]
    pub fn set_fjoin_rx_get(&self) {
        self.set_bits(Self::FJOIN_RX_GET_MASK);
    }

    /// Clear FJOIN_RX_GET bit atomically (RW)
    #[inline(always)]
    pub fn clr_fjoin_rx_get(&self) {
        self.clr_bits(Self::FJOIN_RX_GET_MASK);
    }

    /// **IN_COUNT** `[4:0]` (RW)
    ///
    /// Set the number of pins which are not masked to 0 when read by an IN PINS, WAIT PIN or MOV x, PINS instruction.
    /// For example, an IN_COUNT of 5 means that the 5 LSBs of the IN pin group are visible (bits 4:0), but the remaining 27 MSBs are masked to 0. A count of 32 is encoded with a field value of 0, so the default behaviour is to not perform any masking.
    /// Note this masking is applied in addition to the masking usually performed by the IN instruction. This is mainly useful for the MOV x, PINS instruction, which otherwise has no way of masking pins.
    #[inline(always)]
    pub fn in_count(&self) -> u32 {
        (self.value.read_bits() & Self::IN_COUNT_MASK) >> Self::IN_COUNT_OFFSET
    }

    /// Write **IN_COUNT** `[4:0]` (RW)
    ///
    /// Set the number of pins which are not masked to 0 when read by an IN PINS, WAIT PIN or MOV x, PINS instruction.
    /// For example, an IN_COUNT of 5 means that the 5 LSBs of the IN pin group are visible (bits 4:0), but the remaining 27 MSBs are masked to 0. A count of 32 is encoded with a field value of 0, so the default behaviour is to not perform any masking.
    /// Note this masking is applied in addition to the masking usually performed by the IN instruction. This is mainly useful for the MOV x, PINS instruction, which otherwise has no way of masking pins.
    #[inline(always)]
    pub fn write_in_count(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::IN_COUNT_MASK) | ((value << Self::IN_COUNT_OFFSET) & Self::IN_COUNT_MASK));
    }

}

/// SM1_SHIFTCTRL value type for batch read/modify/write operations
///
/// Use `sm1shiftctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sm1shiftctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Sm1ShiftctrlVal {
    bits: u32,
}

impl Sm1ShiftctrlVal {
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
        Self { bits: 0x000c0000 }
    }

    /// Read **FJOIN_RX** `[31:31]`
    ///
    /// When 1, RX FIFO steals the TX FIFO's storage, and becomes twice as deep.
    /// TX FIFO is disabled as a result (always reads as both full and empty).
    /// FIFOs are flushed when this bit is changed.
    #[inline(always)]
    pub const fn fjoin_rx(&self) -> bool {
        (self.bits & Sm1Shiftctrl::FJOIN_RX_MASK) != 0
    }

    /// Read **FJOIN_TX** `[30:30]`
    ///
    /// When 1, TX FIFO steals the RX FIFO's storage, and becomes twice as deep.
    /// RX FIFO is disabled as a result (always reads as both full and empty).
    /// FIFOs are flushed when this bit is changed.
    #[inline(always)]
    pub const fn fjoin_tx(&self) -> bool {
        (self.bits & Sm1Shiftctrl::FJOIN_TX_MASK) != 0
    }

    /// Read **PULL_THRESH** `[29:25]`
    ///
    /// Number of bits shifted out of OSR before autopull, or conditional pull (PULL IFEMPTY), will take place.
    /// Write 0 for value of 32.
    #[inline(always)]
    pub const fn pull_thresh(&self) -> u32 {
        (self.bits & Sm1Shiftctrl::PULL_THRESH_MASK) >> Sm1Shiftctrl::PULL_THRESH_OFFSET
    }

    /// Read **PUSH_THRESH** `[24:20]`
    ///
    /// Number of bits shifted into ISR before autopush, or conditional push (PUSH IFFULL), will take place.
    /// Write 0 for value of 32.
    #[inline(always)]
    pub const fn push_thresh(&self) -> u32 {
        (self.bits & Sm1Shiftctrl::PUSH_THRESH_MASK) >> Sm1Shiftctrl::PUSH_THRESH_OFFSET
    }

    /// Read **OUT_SHIFTDIR** `[19:19]`
    ///
    /// 1 = shift out of output shift register to right. 0 = to left.
    #[inline(always)]
    pub const fn out_shiftdir(&self) -> bool {
        (self.bits & Sm1Shiftctrl::OUT_SHIFTDIR_MASK) != 0
    }

    /// Read **IN_SHIFTDIR** `[18:18]`
    ///
    /// 1 = shift input shift register to right (data enters from left). 0 = to left.
    #[inline(always)]
    pub const fn in_shiftdir(&self) -> bool {
        (self.bits & Sm1Shiftctrl::IN_SHIFTDIR_MASK) != 0
    }

    /// Read **AUTOPULL** `[17:17]`
    ///
    /// Pull automatically when the output shift register is emptied, i.e. on or following an OUT instruction which causes the output shift counter to reach or exceed PULL_THRESH.
    #[inline(always)]
    pub const fn autopull(&self) -> bool {
        (self.bits & Sm1Shiftctrl::AUTOPULL_MASK) != 0
    }

    /// Read **AUTOPUSH** `[16:16]`
    ///
    /// Push automatically when the input shift register is filled, i.e. on an IN instruction which causes the input shift counter to reach or exceed PUSH_THRESH.
    #[inline(always)]
    pub const fn autopush(&self) -> bool {
        (self.bits & Sm1Shiftctrl::AUTOPUSH_MASK) != 0
    }

    /// Read **FJOIN_RX_PUT** `[15:15]`
    ///
    /// If 1, disable this state machine's RX FIFO, make its storage available for random write access by the state machine (using the `put` instruction) and, unless FJOIN_RX_GET is also set, random read access by the processor (through the RXFx_PUTGETy registers).
    /// If FJOIN_RX_PUT and FJOIN_RX_GET are both set, then the RX FIFO's registers can be randomly read/written by the state machine, but are completely inaccessible to the processor.
    /// Setting this bit will clear the FJOIN_TX and FJOIN_RX bits.
    #[inline(always)]
    pub const fn fjoin_rx_put(&self) -> bool {
        (self.bits & Sm1Shiftctrl::FJOIN_RX_PUT_MASK) != 0
    }

    /// Read **FJOIN_RX_GET** `[14:14]`
    ///
    /// If 1, disable this state machine's RX FIFO, make its storage available for random read access by the state machine (using the `get` instruction) and, unless FJOIN_RX_PUT is also set, random write access by the processor (through the RXFx_PUTGETy registers).
    /// If FJOIN_RX_PUT and FJOIN_RX_GET are both set, then the RX FIFO's registers can be randomly read/written by the state machine, but are completely inaccessible to the processor.
    /// Setting this bit will clear the FJOIN_TX and FJOIN_RX bits.
    #[inline(always)]
    pub const fn fjoin_rx_get(&self) -> bool {
        (self.bits & Sm1Shiftctrl::FJOIN_RX_GET_MASK) != 0
    }

    /// Read **IN_COUNT** `[4:0]`
    ///
    /// Set the number of pins which are not masked to 0 when read by an IN PINS, WAIT PIN or MOV x, PINS instruction.
    /// For example, an IN_COUNT of 5 means that the 5 LSBs of the IN pin group are visible (bits 4:0), but the remaining 27 MSBs are masked to 0. A count of 32 is encoded with a field value of 0, so the default behaviour is to not perform any masking.
    /// Note this masking is applied in addition to the masking usually performed by the IN instruction. This is mainly useful for the MOV x, PINS instruction, which otherwise has no way of masking pins.
    #[inline(always)]
    pub const fn in_count(&self) -> u32 {
        (self.bits & Sm1Shiftctrl::IN_COUNT_MASK) >> Sm1Shiftctrl::IN_COUNT_OFFSET
    }

    /// Set **FJOIN_RX** `[31:31]`
    ///
    /// When 1, RX FIFO steals the TX FIFO's storage, and becomes twice as deep.
    /// TX FIFO is disabled as a result (always reads as both full and empty).
    /// FIFOs are flushed when this bit is changed.
    #[inline(always)]
    pub const fn with_fjoin_rx(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sm1Shiftctrl::FJOIN_RX_MASK
            } else {
                self.bits & !Sm1Shiftctrl::FJOIN_RX_MASK
            }
        }
    }

    /// Set **FJOIN_TX** `[30:30]`
    ///
    /// When 1, TX FIFO steals the RX FIFO's storage, and becomes twice as deep.
    /// RX FIFO is disabled as a result (always reads as both full and empty).
    /// FIFOs are flushed when this bit is changed.
    #[inline(always)]
    pub const fn with_fjoin_tx(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sm1Shiftctrl::FJOIN_TX_MASK
            } else {
                self.bits & !Sm1Shiftctrl::FJOIN_TX_MASK
            }
        }
    }

    /// Set **PULL_THRESH** `[29:25]`
    ///
    /// Number of bits shifted out of OSR before autopull, or conditional pull (PULL IFEMPTY), will take place.
    /// Write 0 for value of 32.
    #[inline(always)]
    pub const fn with_pull_thresh(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sm1Shiftctrl::PULL_THRESH_MASK) | ((value << Sm1Shiftctrl::PULL_THRESH_OFFSET) & Sm1Shiftctrl::PULL_THRESH_MASK)
        }
    }

    /// Set **PUSH_THRESH** `[24:20]`
    ///
    /// Number of bits shifted into ISR before autopush, or conditional push (PUSH IFFULL), will take place.
    /// Write 0 for value of 32.
    #[inline(always)]
    pub const fn with_push_thresh(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sm1Shiftctrl::PUSH_THRESH_MASK) | ((value << Sm1Shiftctrl::PUSH_THRESH_OFFSET) & Sm1Shiftctrl::PUSH_THRESH_MASK)
        }
    }

    /// Set **OUT_SHIFTDIR** `[19:19]`
    ///
    /// 1 = shift out of output shift register to right. 0 = to left.
    #[inline(always)]
    pub const fn with_out_shiftdir(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sm1Shiftctrl::OUT_SHIFTDIR_MASK
            } else {
                self.bits & !Sm1Shiftctrl::OUT_SHIFTDIR_MASK
            }
        }
    }

    /// Set **IN_SHIFTDIR** `[18:18]`
    ///
    /// 1 = shift input shift register to right (data enters from left). 0 = to left.
    #[inline(always)]
    pub const fn with_in_shiftdir(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sm1Shiftctrl::IN_SHIFTDIR_MASK
            } else {
                self.bits & !Sm1Shiftctrl::IN_SHIFTDIR_MASK
            }
        }
    }

    /// Set **AUTOPULL** `[17:17]`
    ///
    /// Pull automatically when the output shift register is emptied, i.e. on or following an OUT instruction which causes the output shift counter to reach or exceed PULL_THRESH.
    #[inline(always)]
    pub const fn with_autopull(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sm1Shiftctrl::AUTOPULL_MASK
            } else {
                self.bits & !Sm1Shiftctrl::AUTOPULL_MASK
            }
        }
    }

    /// Set **AUTOPUSH** `[16:16]`
    ///
    /// Push automatically when the input shift register is filled, i.e. on an IN instruction which causes the input shift counter to reach or exceed PUSH_THRESH.
    #[inline(always)]
    pub const fn with_autopush(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sm1Shiftctrl::AUTOPUSH_MASK
            } else {
                self.bits & !Sm1Shiftctrl::AUTOPUSH_MASK
            }
        }
    }

    /// Set **FJOIN_RX_PUT** `[15:15]`
    ///
    /// If 1, disable this state machine's RX FIFO, make its storage available for random write access by the state machine (using the `put` instruction) and, unless FJOIN_RX_GET is also set, random read access by the processor (through the RXFx_PUTGETy registers).
    /// If FJOIN_RX_PUT and FJOIN_RX_GET are both set, then the RX FIFO's registers can be randomly read/written by the state machine, but are completely inaccessible to the processor.
    /// Setting this bit will clear the FJOIN_TX and FJOIN_RX bits.
    #[inline(always)]
    pub const fn with_fjoin_rx_put(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sm1Shiftctrl::FJOIN_RX_PUT_MASK
            } else {
                self.bits & !Sm1Shiftctrl::FJOIN_RX_PUT_MASK
            }
        }
    }

    /// Set **FJOIN_RX_GET** `[14:14]`
    ///
    /// If 1, disable this state machine's RX FIFO, make its storage available for random read access by the state machine (using the `get` instruction) and, unless FJOIN_RX_PUT is also set, random write access by the processor (through the RXFx_PUTGETy registers).
    /// If FJOIN_RX_PUT and FJOIN_RX_GET are both set, then the RX FIFO's registers can be randomly read/written by the state machine, but are completely inaccessible to the processor.
    /// Setting this bit will clear the FJOIN_TX and FJOIN_RX bits.
    #[inline(always)]
    pub const fn with_fjoin_rx_get(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sm1Shiftctrl::FJOIN_RX_GET_MASK
            } else {
                self.bits & !Sm1Shiftctrl::FJOIN_RX_GET_MASK
            }
        }
    }

    /// Set **IN_COUNT** `[4:0]`
    ///
    /// Set the number of pins which are not masked to 0 when read by an IN PINS, WAIT PIN or MOV x, PINS instruction.
    /// For example, an IN_COUNT of 5 means that the 5 LSBs of the IN pin group are visible (bits 4:0), but the remaining 27 MSBs are masked to 0. A count of 32 is encoded with a field value of 0, so the default behaviour is to not perform any masking.
    /// Note this masking is applied in addition to the masking usually performed by the IN instruction. This is mainly useful for the MOV x, PINS instruction, which otherwise has no way of masking pins.
    #[inline(always)]
    pub const fn with_in_count(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sm1Shiftctrl::IN_COUNT_MASK) | ((value << Sm1Shiftctrl::IN_COUNT_OFFSET) & Sm1Shiftctrl::IN_COUNT_MASK)
        }
    }

}

impl Default for Sm1ShiftctrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Sm1Shiftctrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Sm1ShiftctrlVal {
        Sm1ShiftctrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Sm1ShiftctrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Sm1ShiftctrlVal) -> Sm1ShiftctrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SM1_ADDR Register
///
/// Current instruction address of state machine 1
///
/// Address offset: 0x00ec
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SM1_ADDR** `[4:0]` (RO)
#[repr(transparent)]
pub struct Sm1Addr {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Sm1Addr {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sm1Addr {
    // ======== Field Constants ========

    /// **SM1_ADDR**
    /// Bits `[4:0]`, mask `0x0000001f`
    pub const SM1_ADDR_OFFSET: u32 = 0;
    pub const SM1_ADDR_MASK: u32 = 0x0000001f;

    // ======== Field Accessors ========

    /// **SM1_ADDR** `[4:0]` (RO)
    #[inline(always)]
    pub fn sm1_addr(&self) -> u32 {
        (self.value.read_bits() & Self::SM1_ADDR_MASK) >> Self::SM1_ADDR_OFFSET
    }

}

/// SM1_ADDR value type for batch read/modify/write operations
///
/// Use `sm1addr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sm1addr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Sm1AddrVal {
    bits: u32,
}

impl Sm1AddrVal {
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

    /// Read **SM1_ADDR** `[4:0]`
    #[inline(always)]
    pub const fn sm1_addr(&self) -> u32 {
        (self.bits & Sm1Addr::SM1_ADDR_MASK) >> Sm1Addr::SM1_ADDR_OFFSET
    }

}

impl Default for Sm1AddrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Sm1Addr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Sm1AddrVal {
        Sm1AddrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Sm1AddrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Sm1AddrVal) -> Sm1AddrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SM1_INSTR Register
///
/// Read to see the instruction currently addressed by state machine 1's program counter
/// Write to execute an instruction immediately (including jumps) and then resume execution.
///
/// Address offset: 0x00f0
///
/// ## Fields
///
/// - **SM1_INSTR** `[15:0]` (RW)
#[repr(transparent)]
pub struct Sm1Instr {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Sm1Instr {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sm1Instr {
    // ======== Field Constants ========

    /// **SM1_INSTR**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const SM1_INSTR_OFFSET: u32 = 0;
    pub const SM1_INSTR_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **SM1_INSTR** `[15:0]` (RW)
    #[inline(always)]
    pub fn sm1_instr(&self) -> u32 {
        (self.value.read_bits() & Self::SM1_INSTR_MASK) >> Self::SM1_INSTR_OFFSET
    }

    /// Write **SM1_INSTR** `[15:0]` (RW)
    #[inline(always)]
    pub fn write_sm1_instr(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SM1_INSTR_MASK) | ((value << Self::SM1_INSTR_OFFSET) & Self::SM1_INSTR_MASK));
    }

}

/// SM1_INSTR value type for batch read/modify/write operations
///
/// Use `sm1instr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sm1instr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Sm1InstrVal {
    bits: u32,
}

impl Sm1InstrVal {
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

    /// Read **SM1_INSTR** `[15:0]`
    #[inline(always)]
    pub const fn sm1_instr(&self) -> u32 {
        (self.bits & Sm1Instr::SM1_INSTR_MASK) >> Sm1Instr::SM1_INSTR_OFFSET
    }

    /// Set **SM1_INSTR** `[15:0]`
    #[inline(always)]
    pub const fn with_sm1_instr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sm1Instr::SM1_INSTR_MASK) | ((value << Sm1Instr::SM1_INSTR_OFFSET) & Sm1Instr::SM1_INSTR_MASK)
        }
    }

}

impl Default for Sm1InstrVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Sm1Instr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Sm1InstrVal {
        Sm1InstrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Sm1InstrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Sm1InstrVal) -> Sm1InstrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SM1_PINCTRL Register
///
/// State machine pin control
///
/// Address offset: 0x00f4
/// Reset value: 0x14000000
///
/// ## Fields
///
/// - **SIDESET_COUNT** `[31:29]` (RW)
///
///   The number of MSBs of the Delay/Side-set instruction field which are used for side-set. Inclusive of the enable bit, if present. Minimum of 0 (all delay bits, no side-set) and maximum of 5 (all side-set, no delay).
///
/// - **SET_COUNT** `[28:26]` (RW)
///
///   The number of pins asserted by a SET. In the range 0 to 5 inclusive.
///
/// - **OUT_COUNT** `[25:20]` (RW)
///
///   The number of pins asserted by an OUT PINS, OUT PINDIRS or MOV PINS instruction. In the range 0 to 32 inclusive.
///
/// - **IN_BASE** `[19:15]` (RW)
///
///   The pin which is mapped to the least-significant bit of a state machine's IN data bus. Higher-numbered pins are mapped to consecutively more-significant data bits, with a modulo of 32 applied to pin number.
///
/// - **SIDESET_BASE** `[14:10]` (RW)
///
///   The lowest-numbered pin that will be affected by a side-set operation. The MSBs of an instruction's side-set/delay field (up to 5, determined by SIDESET_COUNT) are used for side-set data, with the remaining LSBs used for delay. The least-significant bit of the side-set portion is the bit written to this pin, with more-significant bits written to higher-numbered pins.
///
/// - **SET_BASE** `[9:5]` (RW)
///
///   The lowest-numbered pin that will be affected by a SET PINS or SET PINDIRS instruction. The data written to this pin is the least-significant bit of the SET data.
///
/// - **OUT_BASE** `[4:0]` (RW)
///
///   The lowest-numbered pin that will be affected by an OUT PINS, OUT PINDIRS or MOV PINS instruction. The data written to this pin will always be the least-significant bit of the OUT or MOV data.
#[repr(transparent)]
pub struct Sm1Pinctrl {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Sm1Pinctrl {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sm1Pinctrl {
    // ======== Field Constants ========

    /// **SIDESET_COUNT**
    ///
    /// The number of MSBs of the Delay/Side-set instruction field which are used for side-set. Inclusive of the enable bit, if present. Minimum of 0 (all delay bits, no side-set) and maximum of 5 (all side-set, no delay).
    ///
    /// Bits `[31:29]`, mask `0xe0000000`
    pub const SIDESET_COUNT_OFFSET: u32 = 29;
    pub const SIDESET_COUNT_MASK: u32 = 0xe0000000;

    /// **SET_COUNT**
    ///
    /// The number of pins asserted by a SET. In the range 0 to 5 inclusive.
    ///
    /// Bits `[28:26]`, mask `0x1c000000`
    pub const SET_COUNT_OFFSET: u32 = 26;
    pub const SET_COUNT_MASK: u32 = 0x1c000000;

    /// **OUT_COUNT**
    ///
    /// The number of pins asserted by an OUT PINS, OUT PINDIRS or MOV PINS instruction. In the range 0 to 32 inclusive.
    ///
    /// Bits `[25:20]`, mask `0x03f00000`
    pub const OUT_COUNT_OFFSET: u32 = 20;
    pub const OUT_COUNT_MASK: u32 = 0x03f00000;

    /// **IN_BASE**
    ///
    /// The pin which is mapped to the least-significant bit of a state machine's IN data bus. Higher-numbered pins are mapped to consecutively more-significant data bits, with a modulo of 32 applied to pin number.
    ///
    /// Bits `[19:15]`, mask `0x000f8000`
    pub const IN_BASE_OFFSET: u32 = 15;
    pub const IN_BASE_MASK: u32 = 0x000f8000;

    /// **SIDESET_BASE**
    ///
    /// The lowest-numbered pin that will be affected by a side-set operation. The MSBs of an instruction's side-set/delay field (up to 5, determined by SIDESET_COUNT) are used for side-set data, with the remaining LSBs used for delay. The least-significant bit of the side-set portion is the bit written to this pin, with more-significant bits written to higher-numbered pins.
    ///
    /// Bits `[14:10]`, mask `0x00007c00`
    pub const SIDESET_BASE_OFFSET: u32 = 10;
    pub const SIDESET_BASE_MASK: u32 = 0x00007c00;

    /// **SET_BASE**
    ///
    /// The lowest-numbered pin that will be affected by a SET PINS or SET PINDIRS instruction. The data written to this pin is the least-significant bit of the SET data.
    ///
    /// Bits `[9:5]`, mask `0x000003e0`
    pub const SET_BASE_OFFSET: u32 = 5;
    pub const SET_BASE_MASK: u32 = 0x000003e0;

    /// **OUT_BASE**
    ///
    /// The lowest-numbered pin that will be affected by an OUT PINS, OUT PINDIRS or MOV PINS instruction. The data written to this pin will always be the least-significant bit of the OUT or MOV data.
    ///
    /// Bits `[4:0]`, mask `0x0000001f`
    pub const OUT_BASE_OFFSET: u32 = 0;
    pub const OUT_BASE_MASK: u32 = 0x0000001f;

    // ======== Field Accessors ========

    /// **SIDESET_COUNT** `[31:29]` (RW)
    ///
    /// The number of MSBs of the Delay/Side-set instruction field which are used for side-set. Inclusive of the enable bit, if present. Minimum of 0 (all delay bits, no side-set) and maximum of 5 (all side-set, no delay).
    #[inline(always)]
    pub fn sideset_count(&self) -> u32 {
        (self.value.read_bits() & Self::SIDESET_COUNT_MASK) >> Self::SIDESET_COUNT_OFFSET
    }

    /// Write **SIDESET_COUNT** `[31:29]` (RW)
    ///
    /// The number of MSBs of the Delay/Side-set instruction field which are used for side-set. Inclusive of the enable bit, if present. Minimum of 0 (all delay bits, no side-set) and maximum of 5 (all side-set, no delay).
    #[inline(always)]
    pub fn write_sideset_count(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SIDESET_COUNT_MASK) | ((value << Self::SIDESET_COUNT_OFFSET) & Self::SIDESET_COUNT_MASK));
    }

    /// **SET_COUNT** `[28:26]` (RW)
    ///
    /// The number of pins asserted by a SET. In the range 0 to 5 inclusive.
    #[inline(always)]
    pub fn set_count(&self) -> u32 {
        (self.value.read_bits() & Self::SET_COUNT_MASK) >> Self::SET_COUNT_OFFSET
    }

    /// Write **SET_COUNT** `[28:26]` (RW)
    ///
    /// The number of pins asserted by a SET. In the range 0 to 5 inclusive.
    #[inline(always)]
    pub fn write_set_count(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SET_COUNT_MASK) | ((value << Self::SET_COUNT_OFFSET) & Self::SET_COUNT_MASK));
    }

    /// **OUT_COUNT** `[25:20]` (RW)
    ///
    /// The number of pins asserted by an OUT PINS, OUT PINDIRS or MOV PINS instruction. In the range 0 to 32 inclusive.
    #[inline(always)]
    pub fn out_count(&self) -> u32 {
        (self.value.read_bits() & Self::OUT_COUNT_MASK) >> Self::OUT_COUNT_OFFSET
    }

    /// Write **OUT_COUNT** `[25:20]` (RW)
    ///
    /// The number of pins asserted by an OUT PINS, OUT PINDIRS or MOV PINS instruction. In the range 0 to 32 inclusive.
    #[inline(always)]
    pub fn write_out_count(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::OUT_COUNT_MASK) | ((value << Self::OUT_COUNT_OFFSET) & Self::OUT_COUNT_MASK));
    }

    /// **IN_BASE** `[19:15]` (RW)
    ///
    /// The pin which is mapped to the least-significant bit of a state machine's IN data bus. Higher-numbered pins are mapped to consecutively more-significant data bits, with a modulo of 32 applied to pin number.
    #[inline(always)]
    pub fn in_base(&self) -> u32 {
        (self.value.read_bits() & Self::IN_BASE_MASK) >> Self::IN_BASE_OFFSET
    }

    /// Write **IN_BASE** `[19:15]` (RW)
    ///
    /// The pin which is mapped to the least-significant bit of a state machine's IN data bus. Higher-numbered pins are mapped to consecutively more-significant data bits, with a modulo of 32 applied to pin number.
    #[inline(always)]
    pub fn write_in_base(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::IN_BASE_MASK) | ((value << Self::IN_BASE_OFFSET) & Self::IN_BASE_MASK));
    }

    /// **SIDESET_BASE** `[14:10]` (RW)
    ///
    /// The lowest-numbered pin that will be affected by a side-set operation. The MSBs of an instruction's side-set/delay field (up to 5, determined by SIDESET_COUNT) are used for side-set data, with the remaining LSBs used for delay. The least-significant bit of the side-set portion is the bit written to this pin, with more-significant bits written to higher-numbered pins.
    #[inline(always)]
    pub fn sideset_base(&self) -> u32 {
        (self.value.read_bits() & Self::SIDESET_BASE_MASK) >> Self::SIDESET_BASE_OFFSET
    }

    /// Write **SIDESET_BASE** `[14:10]` (RW)
    ///
    /// The lowest-numbered pin that will be affected by a side-set operation. The MSBs of an instruction's side-set/delay field (up to 5, determined by SIDESET_COUNT) are used for side-set data, with the remaining LSBs used for delay. The least-significant bit of the side-set portion is the bit written to this pin, with more-significant bits written to higher-numbered pins.
    #[inline(always)]
    pub fn write_sideset_base(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SIDESET_BASE_MASK) | ((value << Self::SIDESET_BASE_OFFSET) & Self::SIDESET_BASE_MASK));
    }

    /// **SET_BASE** `[9:5]` (RW)
    ///
    /// The lowest-numbered pin that will be affected by a SET PINS or SET PINDIRS instruction. The data written to this pin is the least-significant bit of the SET data.
    #[inline(always)]
    pub fn set_base(&self) -> u32 {
        (self.value.read_bits() & Self::SET_BASE_MASK) >> Self::SET_BASE_OFFSET
    }

    /// Write **SET_BASE** `[9:5]` (RW)
    ///
    /// The lowest-numbered pin that will be affected by a SET PINS or SET PINDIRS instruction. The data written to this pin is the least-significant bit of the SET data.
    #[inline(always)]
    pub fn write_set_base(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SET_BASE_MASK) | ((value << Self::SET_BASE_OFFSET) & Self::SET_BASE_MASK));
    }

    /// **OUT_BASE** `[4:0]` (RW)
    ///
    /// The lowest-numbered pin that will be affected by an OUT PINS, OUT PINDIRS or MOV PINS instruction. The data written to this pin will always be the least-significant bit of the OUT or MOV data.
    #[inline(always)]
    pub fn out_base(&self) -> u32 {
        (self.value.read_bits() & Self::OUT_BASE_MASK) >> Self::OUT_BASE_OFFSET
    }

    /// Write **OUT_BASE** `[4:0]` (RW)
    ///
    /// The lowest-numbered pin that will be affected by an OUT PINS, OUT PINDIRS or MOV PINS instruction. The data written to this pin will always be the least-significant bit of the OUT or MOV data.
    #[inline(always)]
    pub fn write_out_base(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::OUT_BASE_MASK) | ((value << Self::OUT_BASE_OFFSET) & Self::OUT_BASE_MASK));
    }

}

/// SM1_PINCTRL value type for batch read/modify/write operations
///
/// Use `sm1pinctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sm1pinctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Sm1PinctrlVal {
    bits: u32,
}

impl Sm1PinctrlVal {
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
        Self { bits: 0x14000000 }
    }

    /// Read **SIDESET_COUNT** `[31:29]`
    ///
    /// The number of MSBs of the Delay/Side-set instruction field which are used for side-set. Inclusive of the enable bit, if present. Minimum of 0 (all delay bits, no side-set) and maximum of 5 (all side-set, no delay).
    #[inline(always)]
    pub const fn sideset_count(&self) -> u32 {
        (self.bits & Sm1Pinctrl::SIDESET_COUNT_MASK) >> Sm1Pinctrl::SIDESET_COUNT_OFFSET
    }

    /// Read **SET_COUNT** `[28:26]`
    ///
    /// The number of pins asserted by a SET. In the range 0 to 5 inclusive.
    #[inline(always)]
    pub const fn set_count(&self) -> u32 {
        (self.bits & Sm1Pinctrl::SET_COUNT_MASK) >> Sm1Pinctrl::SET_COUNT_OFFSET
    }

    /// Read **OUT_COUNT** `[25:20]`
    ///
    /// The number of pins asserted by an OUT PINS, OUT PINDIRS or MOV PINS instruction. In the range 0 to 32 inclusive.
    #[inline(always)]
    pub const fn out_count(&self) -> u32 {
        (self.bits & Sm1Pinctrl::OUT_COUNT_MASK) >> Sm1Pinctrl::OUT_COUNT_OFFSET
    }

    /// Read **IN_BASE** `[19:15]`
    ///
    /// The pin which is mapped to the least-significant bit of a state machine's IN data bus. Higher-numbered pins are mapped to consecutively more-significant data bits, with a modulo of 32 applied to pin number.
    #[inline(always)]
    pub const fn in_base(&self) -> u32 {
        (self.bits & Sm1Pinctrl::IN_BASE_MASK) >> Sm1Pinctrl::IN_BASE_OFFSET
    }

    /// Read **SIDESET_BASE** `[14:10]`
    ///
    /// The lowest-numbered pin that will be affected by a side-set operation. The MSBs of an instruction's side-set/delay field (up to 5, determined by SIDESET_COUNT) are used for side-set data, with the remaining LSBs used for delay. The least-significant bit of the side-set portion is the bit written to this pin, with more-significant bits written to higher-numbered pins.
    #[inline(always)]
    pub const fn sideset_base(&self) -> u32 {
        (self.bits & Sm1Pinctrl::SIDESET_BASE_MASK) >> Sm1Pinctrl::SIDESET_BASE_OFFSET
    }

    /// Read **SET_BASE** `[9:5]`
    ///
    /// The lowest-numbered pin that will be affected by a SET PINS or SET PINDIRS instruction. The data written to this pin is the least-significant bit of the SET data.
    #[inline(always)]
    pub const fn set_base(&self) -> u32 {
        (self.bits & Sm1Pinctrl::SET_BASE_MASK) >> Sm1Pinctrl::SET_BASE_OFFSET
    }

    /// Read **OUT_BASE** `[4:0]`
    ///
    /// The lowest-numbered pin that will be affected by an OUT PINS, OUT PINDIRS or MOV PINS instruction. The data written to this pin will always be the least-significant bit of the OUT or MOV data.
    #[inline(always)]
    pub const fn out_base(&self) -> u32 {
        (self.bits & Sm1Pinctrl::OUT_BASE_MASK) >> Sm1Pinctrl::OUT_BASE_OFFSET
    }

    /// Set **SIDESET_COUNT** `[31:29]`
    ///
    /// The number of MSBs of the Delay/Side-set instruction field which are used for side-set. Inclusive of the enable bit, if present. Minimum of 0 (all delay bits, no side-set) and maximum of 5 (all side-set, no delay).
    #[inline(always)]
    pub const fn with_sideset_count(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sm1Pinctrl::SIDESET_COUNT_MASK) | ((value << Sm1Pinctrl::SIDESET_COUNT_OFFSET) & Sm1Pinctrl::SIDESET_COUNT_MASK)
        }
    }

    /// Set **SET_COUNT** `[28:26]`
    ///
    /// The number of pins asserted by a SET. In the range 0 to 5 inclusive.
    #[inline(always)]
    pub const fn with_set_count(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sm1Pinctrl::SET_COUNT_MASK) | ((value << Sm1Pinctrl::SET_COUNT_OFFSET) & Sm1Pinctrl::SET_COUNT_MASK)
        }
    }

    /// Set **OUT_COUNT** `[25:20]`
    ///
    /// The number of pins asserted by an OUT PINS, OUT PINDIRS or MOV PINS instruction. In the range 0 to 32 inclusive.
    #[inline(always)]
    pub const fn with_out_count(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sm1Pinctrl::OUT_COUNT_MASK) | ((value << Sm1Pinctrl::OUT_COUNT_OFFSET) & Sm1Pinctrl::OUT_COUNT_MASK)
        }
    }

    /// Set **IN_BASE** `[19:15]`
    ///
    /// The pin which is mapped to the least-significant bit of a state machine's IN data bus. Higher-numbered pins are mapped to consecutively more-significant data bits, with a modulo of 32 applied to pin number.
    #[inline(always)]
    pub const fn with_in_base(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sm1Pinctrl::IN_BASE_MASK) | ((value << Sm1Pinctrl::IN_BASE_OFFSET) & Sm1Pinctrl::IN_BASE_MASK)
        }
    }

    /// Set **SIDESET_BASE** `[14:10]`
    ///
    /// The lowest-numbered pin that will be affected by a side-set operation. The MSBs of an instruction's side-set/delay field (up to 5, determined by SIDESET_COUNT) are used for side-set data, with the remaining LSBs used for delay. The least-significant bit of the side-set portion is the bit written to this pin, with more-significant bits written to higher-numbered pins.
    #[inline(always)]
    pub const fn with_sideset_base(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sm1Pinctrl::SIDESET_BASE_MASK) | ((value << Sm1Pinctrl::SIDESET_BASE_OFFSET) & Sm1Pinctrl::SIDESET_BASE_MASK)
        }
    }

    /// Set **SET_BASE** `[9:5]`
    ///
    /// The lowest-numbered pin that will be affected by a SET PINS or SET PINDIRS instruction. The data written to this pin is the least-significant bit of the SET data.
    #[inline(always)]
    pub const fn with_set_base(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sm1Pinctrl::SET_BASE_MASK) | ((value << Sm1Pinctrl::SET_BASE_OFFSET) & Sm1Pinctrl::SET_BASE_MASK)
        }
    }

    /// Set **OUT_BASE** `[4:0]`
    ///
    /// The lowest-numbered pin that will be affected by an OUT PINS, OUT PINDIRS or MOV PINS instruction. The data written to this pin will always be the least-significant bit of the OUT or MOV data.
    #[inline(always)]
    pub const fn with_out_base(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sm1Pinctrl::OUT_BASE_MASK) | ((value << Sm1Pinctrl::OUT_BASE_OFFSET) & Sm1Pinctrl::OUT_BASE_MASK)
        }
    }

}

impl Default for Sm1PinctrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Sm1Pinctrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Sm1PinctrlVal {
        Sm1PinctrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Sm1PinctrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Sm1PinctrlVal) -> Sm1PinctrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SM2_CLKDIV Register
///
/// Clock divisor register for state machine 2
/// Frequency = clock freq / (CLKDIV_INT + CLKDIV_FRAC / 256)
///
/// Address offset: 0x00f8
/// Reset value: 0x00010000
///
/// ## Fields
///
/// - **INT** `[31:16]` (RW)
///
///   Effective frequency is sysclk/(int + frac/256).
///   Value of 0 is interpreted as 65536. If INT is 0, FRAC must also be 0.
///
/// - **FRAC** `[15:8]` (RW)
///
///   Fractional part of clock divisor
#[repr(transparent)]
pub struct Sm2Clkdiv {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Sm2Clkdiv {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sm2Clkdiv {
    // ======== Field Constants ========

    /// **INT**
    ///
    /// Effective frequency is sysclk/(int + frac/256).
    /// Value of 0 is interpreted as 65536. If INT is 0, FRAC must also be 0.
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const INT_OFFSET: u32 = 16;
    pub const INT_MASK: u32 = 0xffff0000;

    /// **FRAC**
    ///
    /// Fractional part of clock divisor
    ///
    /// Bits `[15:8]`, mask `0x0000ff00`
    pub const FRAC_OFFSET: u32 = 8;
    pub const FRAC_MASK: u32 = 0x0000ff00;

    // ======== Field Accessors ========

    /// **INT** `[31:16]` (RW)
    ///
    /// Effective frequency is sysclk/(int + frac/256).
    /// Value of 0 is interpreted as 65536. If INT is 0, FRAC must also be 0.
    #[inline(always)]
    pub fn int(&self) -> u32 {
        (self.value.read_bits() & Self::INT_MASK) >> Self::INT_OFFSET
    }

    /// Write **INT** `[31:16]` (RW)
    ///
    /// Effective frequency is sysclk/(int + frac/256).
    /// Value of 0 is interpreted as 65536. If INT is 0, FRAC must also be 0.
    #[inline(always)]
    pub fn write_int(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::INT_MASK) | ((value << Self::INT_OFFSET) & Self::INT_MASK));
    }

    /// **FRAC** `[15:8]` (RW)
    ///
    /// Fractional part of clock divisor
    #[inline(always)]
    pub fn frac(&self) -> u32 {
        (self.value.read_bits() & Self::FRAC_MASK) >> Self::FRAC_OFFSET
    }

    /// Write **FRAC** `[15:8]` (RW)
    ///
    /// Fractional part of clock divisor
    #[inline(always)]
    pub fn write_frac(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::FRAC_MASK) | ((value << Self::FRAC_OFFSET) & Self::FRAC_MASK));
    }

}

/// SM2_CLKDIV value type for batch read/modify/write operations
///
/// Use `sm2clkdiv.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sm2clkdiv.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Sm2ClkdivVal {
    bits: u32,
}

impl Sm2ClkdivVal {
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
        Self { bits: 0x00010000 }
    }

    /// Read **INT** `[31:16]`
    ///
    /// Effective frequency is sysclk/(int + frac/256).
    /// Value of 0 is interpreted as 65536. If INT is 0, FRAC must also be 0.
    #[inline(always)]
    pub const fn int(&self) -> u32 {
        (self.bits & Sm2Clkdiv::INT_MASK) >> Sm2Clkdiv::INT_OFFSET
    }

    /// Read **FRAC** `[15:8]`
    ///
    /// Fractional part of clock divisor
    #[inline(always)]
    pub const fn frac(&self) -> u32 {
        (self.bits & Sm2Clkdiv::FRAC_MASK) >> Sm2Clkdiv::FRAC_OFFSET
    }

    /// Set **INT** `[31:16]`
    ///
    /// Effective frequency is sysclk/(int + frac/256).
    /// Value of 0 is interpreted as 65536. If INT is 0, FRAC must also be 0.
    #[inline(always)]
    pub const fn with_int(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sm2Clkdiv::INT_MASK) | ((value << Sm2Clkdiv::INT_OFFSET) & Sm2Clkdiv::INT_MASK)
        }
    }

    /// Set **FRAC** `[15:8]`
    ///
    /// Fractional part of clock divisor
    #[inline(always)]
    pub const fn with_frac(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sm2Clkdiv::FRAC_MASK) | ((value << Sm2Clkdiv::FRAC_OFFSET) & Sm2Clkdiv::FRAC_MASK)
        }
    }

}

impl Default for Sm2ClkdivVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Sm2Clkdiv {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Sm2ClkdivVal {
        Sm2ClkdivVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Sm2ClkdivVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Sm2ClkdivVal) -> Sm2ClkdivVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SM2_EXECCTRL Register
///
/// Execution/behavioural settings for state machine 2
///
/// Address offset: 0x00fc
/// Reset value: 0x0001f000
///
/// ## Fields
///
/// - **EXEC_STALLED** `[31]` (RO)
///
///   If 1, an instruction written to SMx_INSTR is stalled, and latched by the state machine. Will clear to 0 once this instruction completes.
///
/// - **SIDE_EN** `[30]` (RW)
///
///   If 1, the MSB of the Delay/Side-set instruction field is used as side-set enable, rather than a side-set data bit. This allows instructions to perform side-set optionally, rather than on every instruction, but the maximum possible side-set width is reduced from 5 to 4. Note that the value of PINCTRL_SIDESET_COUNT is inclusive of this enable bit.
///
/// - **SIDE_PINDIR** `[29]` (RW)
///
///   If 1, side-set data is asserted to pin directions, instead of pin values
///
/// - **JMP_PIN** `[28:24]` (RW)
///
///   The GPIO number to use as condition for JMP PIN. Unaffected by input mapping.
///
/// - **OUT_EN_SEL** `[23:19]` (RW)
///
///   Which data bit to use for inline OUT enable
///
/// - **INLINE_OUT_EN** `[18]` (RW)
///
///   If 1, use a bit of OUT data as an auxiliary write enable
///   When used in conjunction with OUT_STICKY, writes with an enable of 0 will
///   deassert the latest pin write. This can create useful masking/override behaviour
///   due to the priority ordering of state machine pin writes (SM0 < SM1 < ...)
///
/// - **OUT_STICKY** `[17]` (RW)
///
///   Continuously assert the most recent OUT/SET to the pins
///
/// - **WRAP_TOP** `[16:12]` (RW)
///
///   After reaching this address, execution is wrapped to wrap_bottom.
///   If the instruction is a jump, and the jump condition is true, the jump takes priority.
///
/// - **WRAP_BOTTOM** `[11:7]` (RW)
///
///   After reaching wrap_top, execution is wrapped to this address.
///
/// - **STATUS_SEL** `[6:5]` (RW)
///
///   Comparison used for the MOV x, STATUS instruction.
///
/// - **STATUS_N** `[4:0]` (RW)
///
///   Comparison level or IRQ index for the MOV x, STATUS instruction.
///   If STATUS_SEL is TXLEVEL or RXLEVEL, then values of STATUS_N greater than the current FIFO depth are reserved, and have undefined behaviour.
#[repr(transparent)]
pub struct Sm2Execctrl {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Sm2Execctrl {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sm2Execctrl {
    // ======== Field Constants ========

    /// **EXEC_STALLED**
    ///
    /// If 1, an instruction written to SMx_INSTR is stalled, and latched by the state machine. Will clear to 0 once this instruction completes.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const EXEC_STALLED_OFFSET: u32 = 31;
    pub const EXEC_STALLED_MASK: u32 = 0x80000000;
    pub const EXEC_STALLED: u32 = 1 << 31;

    /// **SIDE_EN**
    ///
    /// If 1, the MSB of the Delay/Side-set instruction field is used as side-set enable, rather than a side-set data bit. This allows instructions to perform side-set optionally, rather than on every instruction, but the maximum possible side-set width is reduced from 5 to 4. Note that the value of PINCTRL_SIDESET_COUNT is inclusive of this enable bit.
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const SIDE_EN_OFFSET: u32 = 30;
    pub const SIDE_EN_MASK: u32 = 0x40000000;
    pub const SIDE_EN: u32 = 1 << 30;

    /// **SIDE_PINDIR**
    ///
    /// If 1, side-set data is asserted to pin directions, instead of pin values
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const SIDE_PINDIR_OFFSET: u32 = 29;
    pub const SIDE_PINDIR_MASK: u32 = 0x20000000;
    pub const SIDE_PINDIR: u32 = 1 << 29;

    /// **JMP_PIN**
    ///
    /// The GPIO number to use as condition for JMP PIN. Unaffected by input mapping.
    ///
    /// Bits `[28:24]`, mask `0x1f000000`
    pub const JMP_PIN_OFFSET: u32 = 24;
    pub const JMP_PIN_MASK: u32 = 0x1f000000;

    /// **OUT_EN_SEL**
    ///
    /// Which data bit to use for inline OUT enable
    ///
    /// Bits `[23:19]`, mask `0x00f80000`
    pub const OUT_EN_SEL_OFFSET: u32 = 19;
    pub const OUT_EN_SEL_MASK: u32 = 0x00f80000;

    /// **INLINE_OUT_EN**
    ///
    /// If 1, use a bit of OUT data as an auxiliary write enable
    /// When used in conjunction with OUT_STICKY, writes with an enable of 0 will
    /// deassert the latest pin write. This can create useful masking/override behaviour
    /// due to the priority ordering of state machine pin writes (SM0 < SM1 < ...)
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const INLINE_OUT_EN_OFFSET: u32 = 18;
    pub const INLINE_OUT_EN_MASK: u32 = 0x00040000;
    pub const INLINE_OUT_EN: u32 = 1 << 18;

    /// **OUT_STICKY**
    ///
    /// Continuously assert the most recent OUT/SET to the pins
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const OUT_STICKY_OFFSET: u32 = 17;
    pub const OUT_STICKY_MASK: u32 = 0x00020000;
    pub const OUT_STICKY: u32 = 1 << 17;

    /// **WRAP_TOP**
    ///
    /// After reaching this address, execution is wrapped to wrap_bottom.
    /// If the instruction is a jump, and the jump condition is true, the jump takes priority.
    ///
    /// Bits `[16:12]`, mask `0x0001f000`
    pub const WRAP_TOP_OFFSET: u32 = 12;
    pub const WRAP_TOP_MASK: u32 = 0x0001f000;

    /// **WRAP_BOTTOM**
    ///
    /// After reaching wrap_top, execution is wrapped to this address.
    ///
    /// Bits `[11:7]`, mask `0x00000f80`
    pub const WRAP_BOTTOM_OFFSET: u32 = 7;
    pub const WRAP_BOTTOM_MASK: u32 = 0x00000f80;

    /// **STATUS_SEL**
    ///
    /// Comparison used for the MOV x, STATUS instruction.
    ///
    /// Bits `[6:5]`, mask `0x00000060`
    pub const STATUS_SEL_OFFSET: u32 = 5;
    pub const STATUS_SEL_MASK: u32 = 0x00000060;
    /// TXLEVEL = 0
    /// All-ones if TX FIFO level < N, otherwise all-zeroes
    pub const STATUS_SEL_TXLEVEL: u32 = 0 << 5;
    pub const STATUS_SEL_TXLEVEL_VAL: u32 = 0;
    /// RXLEVEL = 1
    /// All-ones if RX FIFO level < N, otherwise all-zeroes
    pub const STATUS_SEL_RXLEVEL: u32 = 1 << 5;
    pub const STATUS_SEL_RXLEVEL_VAL: u32 = 1;
    /// IRQ = 2
    /// All-ones if the indexed IRQ flag is raised, otherwise all-zeroes
    pub const STATUS_SEL_IRQ: u32 = 2 << 5;
    pub const STATUS_SEL_IRQ_VAL: u32 = 2;

    /// **STATUS_N**
    ///
    /// Comparison level or IRQ index for the MOV x, STATUS instruction.
    /// If STATUS_SEL is TXLEVEL or RXLEVEL, then values of STATUS_N greater than the current FIFO depth are reserved, and have undefined behaviour.
    ///
    /// Bits `[4:0]`, mask `0x0000001f`
    pub const STATUS_N_OFFSET: u32 = 0;
    pub const STATUS_N_MASK: u32 = 0x0000001f;
    /// IRQ = 0
    /// Index 0-7 of an IRQ flag in this PIO block
    pub const STATUS_N_IRQ: u32 = 0 << 0;
    pub const STATUS_N_IRQ_VAL: u32 = 0;
    /// IRQ_PREVPIO = 8
    /// Index 0-7 of an IRQ flag in the next lower-numbered PIO block
    pub const STATUS_N_IRQ_PREVPIO: u32 = 8 << 0;
    pub const STATUS_N_IRQ_PREVPIO_VAL: u32 = 8;
    /// IRQ_NEXTPIO = 16
    /// Index 0-7 of an IRQ flag in the next higher-numbered PIO block
    pub const STATUS_N_IRQ_NEXTPIO: u32 = 16 << 0;
    pub const STATUS_N_IRQ_NEXTPIO_VAL: u32 = 16;

    // ======== Field Accessors ========

    /// **EXEC_STALLED** `[31]` (RO)
    ///
    /// If 1, an instruction written to SMx_INSTR is stalled, and latched by the state machine. Will clear to 0 once this instruction completes.
    #[inline(always)]
    pub fn exec_stalled(&self) -> bool {
        (self.value.read_bits() & Self::EXEC_STALLED_MASK) != 0
    }

    /// **SIDE_EN** `[30]` (RW)
    ///
    /// If 1, the MSB of the Delay/Side-set instruction field is used as side-set enable, rather than a side-set data bit. This allows instructions to perform side-set optionally, rather than on every instruction, but the maximum possible side-set width is reduced from 5 to 4. Note that the value of PINCTRL_SIDESET_COUNT is inclusive of this enable bit.
    #[inline(always)]
    pub fn side_en(&self) -> bool {
        (self.value.read_bits() & Self::SIDE_EN_MASK) != 0
    }

    /// Set SIDE_EN bit atomically (RW)
    #[inline(always)]
    pub fn set_side_en(&self) {
        self.set_bits(Self::SIDE_EN_MASK);
    }

    /// Clear SIDE_EN bit atomically (RW)
    #[inline(always)]
    pub fn clr_side_en(&self) {
        self.clr_bits(Self::SIDE_EN_MASK);
    }

    /// **SIDE_PINDIR** `[29]` (RW)
    ///
    /// If 1, side-set data is asserted to pin directions, instead of pin values
    #[inline(always)]
    pub fn side_pindir(&self) -> bool {
        (self.value.read_bits() & Self::SIDE_PINDIR_MASK) != 0
    }

    /// Set SIDE_PINDIR bit atomically (RW)
    #[inline(always)]
    pub fn set_side_pindir(&self) {
        self.set_bits(Self::SIDE_PINDIR_MASK);
    }

    /// Clear SIDE_PINDIR bit atomically (RW)
    #[inline(always)]
    pub fn clr_side_pindir(&self) {
        self.clr_bits(Self::SIDE_PINDIR_MASK);
    }

    /// **JMP_PIN** `[28:24]` (RW)
    ///
    /// The GPIO number to use as condition for JMP PIN. Unaffected by input mapping.
    #[inline(always)]
    pub fn jmp_pin(&self) -> u32 {
        (self.value.read_bits() & Self::JMP_PIN_MASK) >> Self::JMP_PIN_OFFSET
    }

    /// Write **JMP_PIN** `[28:24]` (RW)
    ///
    /// The GPIO number to use as condition for JMP PIN. Unaffected by input mapping.
    #[inline(always)]
    pub fn write_jmp_pin(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::JMP_PIN_MASK) | ((value << Self::JMP_PIN_OFFSET) & Self::JMP_PIN_MASK));
    }

    /// **OUT_EN_SEL** `[23:19]` (RW)
    ///
    /// Which data bit to use for inline OUT enable
    #[inline(always)]
    pub fn out_en_sel(&self) -> u32 {
        (self.value.read_bits() & Self::OUT_EN_SEL_MASK) >> Self::OUT_EN_SEL_OFFSET
    }

    /// Write **OUT_EN_SEL** `[23:19]` (RW)
    ///
    /// Which data bit to use for inline OUT enable
    #[inline(always)]
    pub fn write_out_en_sel(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::OUT_EN_SEL_MASK) | ((value << Self::OUT_EN_SEL_OFFSET) & Self::OUT_EN_SEL_MASK));
    }

    /// **INLINE_OUT_EN** `[18]` (RW)
    ///
    /// If 1, use a bit of OUT data as an auxiliary write enable
    /// When used in conjunction with OUT_STICKY, writes with an enable of 0 will
    /// deassert the latest pin write. This can create useful masking/override behaviour
    /// due to the priority ordering of state machine pin writes (SM0 < SM1 < ...)
    #[inline(always)]
    pub fn inline_out_en(&self) -> bool {
        (self.value.read_bits() & Self::INLINE_OUT_EN_MASK) != 0
    }

    /// Set INLINE_OUT_EN bit atomically (RW)
    #[inline(always)]
    pub fn set_inline_out_en(&self) {
        self.set_bits(Self::INLINE_OUT_EN_MASK);
    }

    /// Clear INLINE_OUT_EN bit atomically (RW)
    #[inline(always)]
    pub fn clr_inline_out_en(&self) {
        self.clr_bits(Self::INLINE_OUT_EN_MASK);
    }

    /// **OUT_STICKY** `[17]` (RW)
    ///
    /// Continuously assert the most recent OUT/SET to the pins
    #[inline(always)]
    pub fn out_sticky(&self) -> bool {
        (self.value.read_bits() & Self::OUT_STICKY_MASK) != 0
    }

    /// Set OUT_STICKY bit atomically (RW)
    #[inline(always)]
    pub fn set_out_sticky(&self) {
        self.set_bits(Self::OUT_STICKY_MASK);
    }

    /// Clear OUT_STICKY bit atomically (RW)
    #[inline(always)]
    pub fn clr_out_sticky(&self) {
        self.clr_bits(Self::OUT_STICKY_MASK);
    }

    /// **WRAP_TOP** `[16:12]` (RW)
    ///
    /// After reaching this address, execution is wrapped to wrap_bottom.
    /// If the instruction is a jump, and the jump condition is true, the jump takes priority.
    #[inline(always)]
    pub fn wrap_top(&self) -> u32 {
        (self.value.read_bits() & Self::WRAP_TOP_MASK) >> Self::WRAP_TOP_OFFSET
    }

    /// Write **WRAP_TOP** `[16:12]` (RW)
    ///
    /// After reaching this address, execution is wrapped to wrap_bottom.
    /// If the instruction is a jump, and the jump condition is true, the jump takes priority.
    #[inline(always)]
    pub fn write_wrap_top(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::WRAP_TOP_MASK) | ((value << Self::WRAP_TOP_OFFSET) & Self::WRAP_TOP_MASK));
    }

    /// **WRAP_BOTTOM** `[11:7]` (RW)
    ///
    /// After reaching wrap_top, execution is wrapped to this address.
    #[inline(always)]
    pub fn wrap_bottom(&self) -> u32 {
        (self.value.read_bits() & Self::WRAP_BOTTOM_MASK) >> Self::WRAP_BOTTOM_OFFSET
    }

    /// Write **WRAP_BOTTOM** `[11:7]` (RW)
    ///
    /// After reaching wrap_top, execution is wrapped to this address.
    #[inline(always)]
    pub fn write_wrap_bottom(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::WRAP_BOTTOM_MASK) | ((value << Self::WRAP_BOTTOM_OFFSET) & Self::WRAP_BOTTOM_MASK));
    }

    /// **STATUS_SEL** `[6:5]` (RW)
    ///
    /// Comparison used for the MOV x, STATUS instruction.
    #[inline(always)]
    pub fn status_sel(&self) -> u32 {
        (self.value.read_bits() & Self::STATUS_SEL_MASK) >> Self::STATUS_SEL_OFFSET
    }

    /// Write **STATUS_SEL** `[6:5]` (RW)
    ///
    /// Comparison used for the MOV x, STATUS instruction.
    #[inline(always)]
    pub fn write_status_sel(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::STATUS_SEL_MASK) | ((value << Self::STATUS_SEL_OFFSET) & Self::STATUS_SEL_MASK));
    }

    /// **STATUS_N** `[4:0]` (RW)
    ///
    /// Comparison level or IRQ index for the MOV x, STATUS instruction.
    /// If STATUS_SEL is TXLEVEL or RXLEVEL, then values of STATUS_N greater than the current FIFO depth are reserved, and have undefined behaviour.
    #[inline(always)]
    pub fn status_n(&self) -> u32 {
        (self.value.read_bits() & Self::STATUS_N_MASK) >> Self::STATUS_N_OFFSET
    }

    /// Write **STATUS_N** `[4:0]` (RW)
    ///
    /// Comparison level or IRQ index for the MOV x, STATUS instruction.
    /// If STATUS_SEL is TXLEVEL or RXLEVEL, then values of STATUS_N greater than the current FIFO depth are reserved, and have undefined behaviour.
    #[inline(always)]
    pub fn write_status_n(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::STATUS_N_MASK) | ((value << Self::STATUS_N_OFFSET) & Self::STATUS_N_MASK));
    }

}

/// SM2_EXECCTRL value type for batch read/modify/write operations
///
/// Use `sm2execctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sm2execctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Sm2ExecctrlVal {
    bits: u32,
}

impl Sm2ExecctrlVal {
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
        Self { bits: 0x0001f000 }
    }

    /// Read **EXEC_STALLED** `[31:31]`
    ///
    /// If 1, an instruction written to SMx_INSTR is stalled, and latched by the state machine. Will clear to 0 once this instruction completes.
    #[inline(always)]
    pub const fn exec_stalled(&self) -> bool {
        (self.bits & Sm2Execctrl::EXEC_STALLED_MASK) != 0
    }

    /// Read **SIDE_EN** `[30:30]`
    ///
    /// If 1, the MSB of the Delay/Side-set instruction field is used as side-set enable, rather than a side-set data bit. This allows instructions to perform side-set optionally, rather than on every instruction, but the maximum possible side-set width is reduced from 5 to 4. Note that the value of PINCTRL_SIDESET_COUNT is inclusive of this enable bit.
    #[inline(always)]
    pub const fn side_en(&self) -> bool {
        (self.bits & Sm2Execctrl::SIDE_EN_MASK) != 0
    }

    /// Read **SIDE_PINDIR** `[29:29]`
    ///
    /// If 1, side-set data is asserted to pin directions, instead of pin values
    #[inline(always)]
    pub const fn side_pindir(&self) -> bool {
        (self.bits & Sm2Execctrl::SIDE_PINDIR_MASK) != 0
    }

    /// Read **JMP_PIN** `[28:24]`
    ///
    /// The GPIO number to use as condition for JMP PIN. Unaffected by input mapping.
    #[inline(always)]
    pub const fn jmp_pin(&self) -> u32 {
        (self.bits & Sm2Execctrl::JMP_PIN_MASK) >> Sm2Execctrl::JMP_PIN_OFFSET
    }

    /// Read **OUT_EN_SEL** `[23:19]`
    ///
    /// Which data bit to use for inline OUT enable
    #[inline(always)]
    pub const fn out_en_sel(&self) -> u32 {
        (self.bits & Sm2Execctrl::OUT_EN_SEL_MASK) >> Sm2Execctrl::OUT_EN_SEL_OFFSET
    }

    /// Read **INLINE_OUT_EN** `[18:18]`
    ///
    /// If 1, use a bit of OUT data as an auxiliary write enable
    /// When used in conjunction with OUT_STICKY, writes with an enable of 0 will
    /// deassert the latest pin write. This can create useful masking/override behaviour
    /// due to the priority ordering of state machine pin writes (SM0 < SM1 < ...)
    #[inline(always)]
    pub const fn inline_out_en(&self) -> bool {
        (self.bits & Sm2Execctrl::INLINE_OUT_EN_MASK) != 0
    }

    /// Read **OUT_STICKY** `[17:17]`
    ///
    /// Continuously assert the most recent OUT/SET to the pins
    #[inline(always)]
    pub const fn out_sticky(&self) -> bool {
        (self.bits & Sm2Execctrl::OUT_STICKY_MASK) != 0
    }

    /// Read **WRAP_TOP** `[16:12]`
    ///
    /// After reaching this address, execution is wrapped to wrap_bottom.
    /// If the instruction is a jump, and the jump condition is true, the jump takes priority.
    #[inline(always)]
    pub const fn wrap_top(&self) -> u32 {
        (self.bits & Sm2Execctrl::WRAP_TOP_MASK) >> Sm2Execctrl::WRAP_TOP_OFFSET
    }

    /// Read **WRAP_BOTTOM** `[11:7]`
    ///
    /// After reaching wrap_top, execution is wrapped to this address.
    #[inline(always)]
    pub const fn wrap_bottom(&self) -> u32 {
        (self.bits & Sm2Execctrl::WRAP_BOTTOM_MASK) >> Sm2Execctrl::WRAP_BOTTOM_OFFSET
    }

    /// Read **STATUS_SEL** `[6:5]`
    ///
    /// Comparison used for the MOV x, STATUS instruction.
    #[inline(always)]
    pub const fn status_sel(&self) -> u32 {
        (self.bits & Sm2Execctrl::STATUS_SEL_MASK) >> Sm2Execctrl::STATUS_SEL_OFFSET
    }

    /// Read **STATUS_N** `[4:0]`
    ///
    /// Comparison level or IRQ index for the MOV x, STATUS instruction.
    /// If STATUS_SEL is TXLEVEL or RXLEVEL, then values of STATUS_N greater than the current FIFO depth are reserved, and have undefined behaviour.
    #[inline(always)]
    pub const fn status_n(&self) -> u32 {
        (self.bits & Sm2Execctrl::STATUS_N_MASK) >> Sm2Execctrl::STATUS_N_OFFSET
    }

    /// Set **SIDE_EN** `[30:30]`
    ///
    /// If 1, the MSB of the Delay/Side-set instruction field is used as side-set enable, rather than a side-set data bit. This allows instructions to perform side-set optionally, rather than on every instruction, but the maximum possible side-set width is reduced from 5 to 4. Note that the value of PINCTRL_SIDESET_COUNT is inclusive of this enable bit.
    #[inline(always)]
    pub const fn with_side_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sm2Execctrl::SIDE_EN_MASK
            } else {
                self.bits & !Sm2Execctrl::SIDE_EN_MASK
            }
        }
    }

    /// Set **SIDE_PINDIR** `[29:29]`
    ///
    /// If 1, side-set data is asserted to pin directions, instead of pin values
    #[inline(always)]
    pub const fn with_side_pindir(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sm2Execctrl::SIDE_PINDIR_MASK
            } else {
                self.bits & !Sm2Execctrl::SIDE_PINDIR_MASK
            }
        }
    }

    /// Set **JMP_PIN** `[28:24]`
    ///
    /// The GPIO number to use as condition for JMP PIN. Unaffected by input mapping.
    #[inline(always)]
    pub const fn with_jmp_pin(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sm2Execctrl::JMP_PIN_MASK) | ((value << Sm2Execctrl::JMP_PIN_OFFSET) & Sm2Execctrl::JMP_PIN_MASK)
        }
    }

    /// Set **OUT_EN_SEL** `[23:19]`
    ///
    /// Which data bit to use for inline OUT enable
    #[inline(always)]
    pub const fn with_out_en_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sm2Execctrl::OUT_EN_SEL_MASK) | ((value << Sm2Execctrl::OUT_EN_SEL_OFFSET) & Sm2Execctrl::OUT_EN_SEL_MASK)
        }
    }

    /// Set **INLINE_OUT_EN** `[18:18]`
    ///
    /// If 1, use a bit of OUT data as an auxiliary write enable
    /// When used in conjunction with OUT_STICKY, writes with an enable of 0 will
    /// deassert the latest pin write. This can create useful masking/override behaviour
    /// due to the priority ordering of state machine pin writes (SM0 < SM1 < ...)
    #[inline(always)]
    pub const fn with_inline_out_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sm2Execctrl::INLINE_OUT_EN_MASK
            } else {
                self.bits & !Sm2Execctrl::INLINE_OUT_EN_MASK
            }
        }
    }

    /// Set **OUT_STICKY** `[17:17]`
    ///
    /// Continuously assert the most recent OUT/SET to the pins
    #[inline(always)]
    pub const fn with_out_sticky(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sm2Execctrl::OUT_STICKY_MASK
            } else {
                self.bits & !Sm2Execctrl::OUT_STICKY_MASK
            }
        }
    }

    /// Set **WRAP_TOP** `[16:12]`
    ///
    /// After reaching this address, execution is wrapped to wrap_bottom.
    /// If the instruction is a jump, and the jump condition is true, the jump takes priority.
    #[inline(always)]
    pub const fn with_wrap_top(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sm2Execctrl::WRAP_TOP_MASK) | ((value << Sm2Execctrl::WRAP_TOP_OFFSET) & Sm2Execctrl::WRAP_TOP_MASK)
        }
    }

    /// Set **WRAP_BOTTOM** `[11:7]`
    ///
    /// After reaching wrap_top, execution is wrapped to this address.
    #[inline(always)]
    pub const fn with_wrap_bottom(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sm2Execctrl::WRAP_BOTTOM_MASK) | ((value << Sm2Execctrl::WRAP_BOTTOM_OFFSET) & Sm2Execctrl::WRAP_BOTTOM_MASK)
        }
    }

    /// Set **STATUS_SEL** `[6:5]`
    ///
    /// Comparison used for the MOV x, STATUS instruction.
    #[inline(always)]
    pub const fn with_status_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sm2Execctrl::STATUS_SEL_MASK) | ((value << Sm2Execctrl::STATUS_SEL_OFFSET) & Sm2Execctrl::STATUS_SEL_MASK)
        }
    }

    /// Set **STATUS_N** `[4:0]`
    ///
    /// Comparison level or IRQ index for the MOV x, STATUS instruction.
    /// If STATUS_SEL is TXLEVEL or RXLEVEL, then values of STATUS_N greater than the current FIFO depth are reserved, and have undefined behaviour.
    #[inline(always)]
    pub const fn with_status_n(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sm2Execctrl::STATUS_N_MASK) | ((value << Sm2Execctrl::STATUS_N_OFFSET) & Sm2Execctrl::STATUS_N_MASK)
        }
    }

}

impl Default for Sm2ExecctrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Sm2Execctrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Sm2ExecctrlVal {
        Sm2ExecctrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Sm2ExecctrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Sm2ExecctrlVal) -> Sm2ExecctrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SM2_SHIFTCTRL Register
///
/// Control behaviour of the input/output shift registers for state machine 2
///
/// Address offset: 0x0100
/// Reset value: 0x000c0000
///
/// ## Fields
///
/// - **FJOIN_RX** `[31]` (RW)
///
///   When 1, RX FIFO steals the TX FIFO's storage, and becomes twice as deep.
///   TX FIFO is disabled as a result (always reads as both full and empty).
///   FIFOs are flushed when this bit is changed.
///
/// - **FJOIN_TX** `[30]` (RW)
///
///   When 1, TX FIFO steals the RX FIFO's storage, and becomes twice as deep.
///   RX FIFO is disabled as a result (always reads as both full and empty).
///   FIFOs are flushed when this bit is changed.
///
/// - **PULL_THRESH** `[29:25]` (RW)
///
///   Number of bits shifted out of OSR before autopull, or conditional pull (PULL IFEMPTY), will take place.
///   Write 0 for value of 32.
///
/// - **PUSH_THRESH** `[24:20]` (RW)
///
///   Number of bits shifted into ISR before autopush, or conditional push (PUSH IFFULL), will take place.
///   Write 0 for value of 32.
///
/// - **OUT_SHIFTDIR** `[19]` (RW)
///
///   1 = shift out of output shift register to right. 0 = to left.
///
/// - **IN_SHIFTDIR** `[18]` (RW)
///
///   1 = shift input shift register to right (data enters from left). 0 = to left.
///
/// - **AUTOPULL** `[17]` (RW)
///
///   Pull automatically when the output shift register is emptied, i.e. on or following an OUT instruction which causes the output shift counter to reach or exceed PULL_THRESH.
///
/// - **AUTOPUSH** `[16]` (RW)
///
///   Push automatically when the input shift register is filled, i.e. on an IN instruction which causes the input shift counter to reach or exceed PUSH_THRESH.
///
/// - **FJOIN_RX_PUT** `[15]` (RW)
///
///   If 1, disable this state machine's RX FIFO, make its storage available for random write access by the state machine (using the `put` instruction) and, unless FJOIN_RX_GET is also set, random read access by the processor (through the RXFx_PUTGETy registers).
///   If FJOIN_RX_PUT and FJOIN_RX_GET are both set, then the RX FIFO's registers can be randomly read/written by the state machine, but are completely inaccessible to the processor.
///   Setting this bit will clear the FJOIN_TX and FJOIN_RX bits.
///
/// - **FJOIN_RX_GET** `[14]` (RW)
///
///   If 1, disable this state machine's RX FIFO, make its storage available for random read access by the state machine (using the `get` instruction) and, unless FJOIN_RX_PUT is also set, random write access by the processor (through the RXFx_PUTGETy registers).
///   If FJOIN_RX_PUT and FJOIN_RX_GET are both set, then the RX FIFO's registers can be randomly read/written by the state machine, but are completely inaccessible to the processor.
///   Setting this bit will clear the FJOIN_TX and FJOIN_RX bits.
///
/// - **IN_COUNT** `[4:0]` (RW)
///
///   Set the number of pins which are not masked to 0 when read by an IN PINS, WAIT PIN or MOV x, PINS instruction.
///   For example, an IN_COUNT of 5 means that the 5 LSBs of the IN pin group are visible (bits 4:0), but the remaining 27 MSBs are masked to 0. A count of 32 is encoded with a field value of 0, so the default behaviour is to not perform any masking.
///   Note this masking is applied in addition to the masking usually performed by the IN instruction. This is mainly useful for the MOV x, PINS instruction, which otherwise has no way of masking pins.
#[repr(transparent)]
pub struct Sm2Shiftctrl {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Sm2Shiftctrl {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sm2Shiftctrl {
    // ======== Field Constants ========

    /// **FJOIN_RX**
    ///
    /// When 1, RX FIFO steals the TX FIFO's storage, and becomes twice as deep.
    /// TX FIFO is disabled as a result (always reads as both full and empty).
    /// FIFOs are flushed when this bit is changed.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const FJOIN_RX_OFFSET: u32 = 31;
    pub const FJOIN_RX_MASK: u32 = 0x80000000;
    pub const FJOIN_RX: u32 = 1 << 31;

    /// **FJOIN_TX**
    ///
    /// When 1, TX FIFO steals the RX FIFO's storage, and becomes twice as deep.
    /// RX FIFO is disabled as a result (always reads as both full and empty).
    /// FIFOs are flushed when this bit is changed.
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const FJOIN_TX_OFFSET: u32 = 30;
    pub const FJOIN_TX_MASK: u32 = 0x40000000;
    pub const FJOIN_TX: u32 = 1 << 30;

    /// **PULL_THRESH**
    ///
    /// Number of bits shifted out of OSR before autopull, or conditional pull (PULL IFEMPTY), will take place.
    /// Write 0 for value of 32.
    ///
    /// Bits `[29:25]`, mask `0x3e000000`
    pub const PULL_THRESH_OFFSET: u32 = 25;
    pub const PULL_THRESH_MASK: u32 = 0x3e000000;

    /// **PUSH_THRESH**
    ///
    /// Number of bits shifted into ISR before autopush, or conditional push (PUSH IFFULL), will take place.
    /// Write 0 for value of 32.
    ///
    /// Bits `[24:20]`, mask `0x01f00000`
    pub const PUSH_THRESH_OFFSET: u32 = 20;
    pub const PUSH_THRESH_MASK: u32 = 0x01f00000;

    /// **OUT_SHIFTDIR**
    ///
    /// 1 = shift out of output shift register to right. 0 = to left.
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const OUT_SHIFTDIR_OFFSET: u32 = 19;
    pub const OUT_SHIFTDIR_MASK: u32 = 0x00080000;
    pub const OUT_SHIFTDIR: u32 = 1 << 19;

    /// **IN_SHIFTDIR**
    ///
    /// 1 = shift input shift register to right (data enters from left). 0 = to left.
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const IN_SHIFTDIR_OFFSET: u32 = 18;
    pub const IN_SHIFTDIR_MASK: u32 = 0x00040000;
    pub const IN_SHIFTDIR: u32 = 1 << 18;

    /// **AUTOPULL**
    ///
    /// Pull automatically when the output shift register is emptied, i.e. on or following an OUT instruction which causes the output shift counter to reach or exceed PULL_THRESH.
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const AUTOPULL_OFFSET: u32 = 17;
    pub const AUTOPULL_MASK: u32 = 0x00020000;
    pub const AUTOPULL: u32 = 1 << 17;

    /// **AUTOPUSH**
    ///
    /// Push automatically when the input shift register is filled, i.e. on an IN instruction which causes the input shift counter to reach or exceed PUSH_THRESH.
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const AUTOPUSH_OFFSET: u32 = 16;
    pub const AUTOPUSH_MASK: u32 = 0x00010000;
    pub const AUTOPUSH: u32 = 1 << 16;

    /// **FJOIN_RX_PUT**
    ///
    /// If 1, disable this state machine's RX FIFO, make its storage available for random write access by the state machine (using the `put` instruction) and, unless FJOIN_RX_GET is also set, random read access by the processor (through the RXFx_PUTGETy registers).
    /// If FJOIN_RX_PUT and FJOIN_RX_GET are both set, then the RX FIFO's registers can be randomly read/written by the state machine, but are completely inaccessible to the processor.
    /// Setting this bit will clear the FJOIN_TX and FJOIN_RX bits.
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const FJOIN_RX_PUT_OFFSET: u32 = 15;
    pub const FJOIN_RX_PUT_MASK: u32 = 0x00008000;
    pub const FJOIN_RX_PUT: u32 = 1 << 15;

    /// **FJOIN_RX_GET**
    ///
    /// If 1, disable this state machine's RX FIFO, make its storage available for random read access by the state machine (using the `get` instruction) and, unless FJOIN_RX_PUT is also set, random write access by the processor (through the RXFx_PUTGETy registers).
    /// If FJOIN_RX_PUT and FJOIN_RX_GET are both set, then the RX FIFO's registers can be randomly read/written by the state machine, but are completely inaccessible to the processor.
    /// Setting this bit will clear the FJOIN_TX and FJOIN_RX bits.
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const FJOIN_RX_GET_OFFSET: u32 = 14;
    pub const FJOIN_RX_GET_MASK: u32 = 0x00004000;
    pub const FJOIN_RX_GET: u32 = 1 << 14;

    /// **IN_COUNT**
    ///
    /// Set the number of pins which are not masked to 0 when read by an IN PINS, WAIT PIN or MOV x, PINS instruction.
    /// For example, an IN_COUNT of 5 means that the 5 LSBs of the IN pin group are visible (bits 4:0), but the remaining 27 MSBs are masked to 0. A count of 32 is encoded with a field value of 0, so the default behaviour is to not perform any masking.
    /// Note this masking is applied in addition to the masking usually performed by the IN instruction. This is mainly useful for the MOV x, PINS instruction, which otherwise has no way of masking pins.
    ///
    /// Bits `[4:0]`, mask `0x0000001f`
    pub const IN_COUNT_OFFSET: u32 = 0;
    pub const IN_COUNT_MASK: u32 = 0x0000001f;

    // ======== Field Accessors ========

    /// **FJOIN_RX** `[31]` (RW)
    ///
    /// When 1, RX FIFO steals the TX FIFO's storage, and becomes twice as deep.
    /// TX FIFO is disabled as a result (always reads as both full and empty).
    /// FIFOs are flushed when this bit is changed.
    #[inline(always)]
    pub fn fjoin_rx(&self) -> bool {
        (self.value.read_bits() & Self::FJOIN_RX_MASK) != 0
    }

    /// Set FJOIN_RX bit atomically (RW)
    #[inline(always)]
    pub fn set_fjoin_rx(&self) {
        self.set_bits(Self::FJOIN_RX_MASK);
    }

    /// Clear FJOIN_RX bit atomically (RW)
    #[inline(always)]
    pub fn clr_fjoin_rx(&self) {
        self.clr_bits(Self::FJOIN_RX_MASK);
    }

    /// **FJOIN_TX** `[30]` (RW)
    ///
    /// When 1, TX FIFO steals the RX FIFO's storage, and becomes twice as deep.
    /// RX FIFO is disabled as a result (always reads as both full and empty).
    /// FIFOs are flushed when this bit is changed.
    #[inline(always)]
    pub fn fjoin_tx(&self) -> bool {
        (self.value.read_bits() & Self::FJOIN_TX_MASK) != 0
    }

    /// Set FJOIN_TX bit atomically (RW)
    #[inline(always)]
    pub fn set_fjoin_tx(&self) {
        self.set_bits(Self::FJOIN_TX_MASK);
    }

    /// Clear FJOIN_TX bit atomically (RW)
    #[inline(always)]
    pub fn clr_fjoin_tx(&self) {
        self.clr_bits(Self::FJOIN_TX_MASK);
    }

    /// **PULL_THRESH** `[29:25]` (RW)
    ///
    /// Number of bits shifted out of OSR before autopull, or conditional pull (PULL IFEMPTY), will take place.
    /// Write 0 for value of 32.
    #[inline(always)]
    pub fn pull_thresh(&self) -> u32 {
        (self.value.read_bits() & Self::PULL_THRESH_MASK) >> Self::PULL_THRESH_OFFSET
    }

    /// Write **PULL_THRESH** `[29:25]` (RW)
    ///
    /// Number of bits shifted out of OSR before autopull, or conditional pull (PULL IFEMPTY), will take place.
    /// Write 0 for value of 32.
    #[inline(always)]
    pub fn write_pull_thresh(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::PULL_THRESH_MASK) | ((value << Self::PULL_THRESH_OFFSET) & Self::PULL_THRESH_MASK));
    }

    /// **PUSH_THRESH** `[24:20]` (RW)
    ///
    /// Number of bits shifted into ISR before autopush, or conditional push (PUSH IFFULL), will take place.
    /// Write 0 for value of 32.
    #[inline(always)]
    pub fn push_thresh(&self) -> u32 {
        (self.value.read_bits() & Self::PUSH_THRESH_MASK) >> Self::PUSH_THRESH_OFFSET
    }

    /// Write **PUSH_THRESH** `[24:20]` (RW)
    ///
    /// Number of bits shifted into ISR before autopush, or conditional push (PUSH IFFULL), will take place.
    /// Write 0 for value of 32.
    #[inline(always)]
    pub fn write_push_thresh(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::PUSH_THRESH_MASK) | ((value << Self::PUSH_THRESH_OFFSET) & Self::PUSH_THRESH_MASK));
    }

    /// **OUT_SHIFTDIR** `[19]` (RW)
    ///
    /// 1 = shift out of output shift register to right. 0 = to left.
    #[inline(always)]
    pub fn out_shiftdir(&self) -> bool {
        (self.value.read_bits() & Self::OUT_SHIFTDIR_MASK) != 0
    }

    /// Set OUT_SHIFTDIR bit atomically (RW)
    #[inline(always)]
    pub fn set_out_shiftdir(&self) {
        self.set_bits(Self::OUT_SHIFTDIR_MASK);
    }

    /// Clear OUT_SHIFTDIR bit atomically (RW)
    #[inline(always)]
    pub fn clr_out_shiftdir(&self) {
        self.clr_bits(Self::OUT_SHIFTDIR_MASK);
    }

    /// **IN_SHIFTDIR** `[18]` (RW)
    ///
    /// 1 = shift input shift register to right (data enters from left). 0 = to left.
    #[inline(always)]
    pub fn in_shiftdir(&self) -> bool {
        (self.value.read_bits() & Self::IN_SHIFTDIR_MASK) != 0
    }

    /// Set IN_SHIFTDIR bit atomically (RW)
    #[inline(always)]
    pub fn set_in_shiftdir(&self) {
        self.set_bits(Self::IN_SHIFTDIR_MASK);
    }

    /// Clear IN_SHIFTDIR bit atomically (RW)
    #[inline(always)]
    pub fn clr_in_shiftdir(&self) {
        self.clr_bits(Self::IN_SHIFTDIR_MASK);
    }

    /// **AUTOPULL** `[17]` (RW)
    ///
    /// Pull automatically when the output shift register is emptied, i.e. on or following an OUT instruction which causes the output shift counter to reach or exceed PULL_THRESH.
    #[inline(always)]
    pub fn autopull(&self) -> bool {
        (self.value.read_bits() & Self::AUTOPULL_MASK) != 0
    }

    /// Set AUTOPULL bit atomically (RW)
    #[inline(always)]
    pub fn set_autopull(&self) {
        self.set_bits(Self::AUTOPULL_MASK);
    }

    /// Clear AUTOPULL bit atomically (RW)
    #[inline(always)]
    pub fn clr_autopull(&self) {
        self.clr_bits(Self::AUTOPULL_MASK);
    }

    /// **AUTOPUSH** `[16]` (RW)
    ///
    /// Push automatically when the input shift register is filled, i.e. on an IN instruction which causes the input shift counter to reach or exceed PUSH_THRESH.
    #[inline(always)]
    pub fn autopush(&self) -> bool {
        (self.value.read_bits() & Self::AUTOPUSH_MASK) != 0
    }

    /// Set AUTOPUSH bit atomically (RW)
    #[inline(always)]
    pub fn set_autopush(&self) {
        self.set_bits(Self::AUTOPUSH_MASK);
    }

    /// Clear AUTOPUSH bit atomically (RW)
    #[inline(always)]
    pub fn clr_autopush(&self) {
        self.clr_bits(Self::AUTOPUSH_MASK);
    }

    /// **FJOIN_RX_PUT** `[15]` (RW)
    ///
    /// If 1, disable this state machine's RX FIFO, make its storage available for random write access by the state machine (using the `put` instruction) and, unless FJOIN_RX_GET is also set, random read access by the processor (through the RXFx_PUTGETy registers).
    /// If FJOIN_RX_PUT and FJOIN_RX_GET are both set, then the RX FIFO's registers can be randomly read/written by the state machine, but are completely inaccessible to the processor.
    /// Setting this bit will clear the FJOIN_TX and FJOIN_RX bits.
    #[inline(always)]
    pub fn fjoin_rx_put(&self) -> bool {
        (self.value.read_bits() & Self::FJOIN_RX_PUT_MASK) != 0
    }

    /// Set FJOIN_RX_PUT bit atomically (RW)
    #[inline(always)]
    pub fn set_fjoin_rx_put(&self) {
        self.set_bits(Self::FJOIN_RX_PUT_MASK);
    }

    /// Clear FJOIN_RX_PUT bit atomically (RW)
    #[inline(always)]
    pub fn clr_fjoin_rx_put(&self) {
        self.clr_bits(Self::FJOIN_RX_PUT_MASK);
    }

    /// **FJOIN_RX_GET** `[14]` (RW)
    ///
    /// If 1, disable this state machine's RX FIFO, make its storage available for random read access by the state machine (using the `get` instruction) and, unless FJOIN_RX_PUT is also set, random write access by the processor (through the RXFx_PUTGETy registers).
    /// If FJOIN_RX_PUT and FJOIN_RX_GET are both set, then the RX FIFO's registers can be randomly read/written by the state machine, but are completely inaccessible to the processor.
    /// Setting this bit will clear the FJOIN_TX and FJOIN_RX bits.
    #[inline(always)]
    pub fn fjoin_rx_get(&self) -> bool {
        (self.value.read_bits() & Self::FJOIN_RX_GET_MASK) != 0
    }

    /// Set FJOIN_RX_GET bit atomically (RW)
    #[inline(always)]
    pub fn set_fjoin_rx_get(&self) {
        self.set_bits(Self::FJOIN_RX_GET_MASK);
    }

    /// Clear FJOIN_RX_GET bit atomically (RW)
    #[inline(always)]
    pub fn clr_fjoin_rx_get(&self) {
        self.clr_bits(Self::FJOIN_RX_GET_MASK);
    }

    /// **IN_COUNT** `[4:0]` (RW)
    ///
    /// Set the number of pins which are not masked to 0 when read by an IN PINS, WAIT PIN or MOV x, PINS instruction.
    /// For example, an IN_COUNT of 5 means that the 5 LSBs of the IN pin group are visible (bits 4:0), but the remaining 27 MSBs are masked to 0. A count of 32 is encoded with a field value of 0, so the default behaviour is to not perform any masking.
    /// Note this masking is applied in addition to the masking usually performed by the IN instruction. This is mainly useful for the MOV x, PINS instruction, which otherwise has no way of masking pins.
    #[inline(always)]
    pub fn in_count(&self) -> u32 {
        (self.value.read_bits() & Self::IN_COUNT_MASK) >> Self::IN_COUNT_OFFSET
    }

    /// Write **IN_COUNT** `[4:0]` (RW)
    ///
    /// Set the number of pins which are not masked to 0 when read by an IN PINS, WAIT PIN or MOV x, PINS instruction.
    /// For example, an IN_COUNT of 5 means that the 5 LSBs of the IN pin group are visible (bits 4:0), but the remaining 27 MSBs are masked to 0. A count of 32 is encoded with a field value of 0, so the default behaviour is to not perform any masking.
    /// Note this masking is applied in addition to the masking usually performed by the IN instruction. This is mainly useful for the MOV x, PINS instruction, which otherwise has no way of masking pins.
    #[inline(always)]
    pub fn write_in_count(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::IN_COUNT_MASK) | ((value << Self::IN_COUNT_OFFSET) & Self::IN_COUNT_MASK));
    }

}

/// SM2_SHIFTCTRL value type for batch read/modify/write operations
///
/// Use `sm2shiftctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sm2shiftctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Sm2ShiftctrlVal {
    bits: u32,
}

impl Sm2ShiftctrlVal {
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
        Self { bits: 0x000c0000 }
    }

    /// Read **FJOIN_RX** `[31:31]`
    ///
    /// When 1, RX FIFO steals the TX FIFO's storage, and becomes twice as deep.
    /// TX FIFO is disabled as a result (always reads as both full and empty).
    /// FIFOs are flushed when this bit is changed.
    #[inline(always)]
    pub const fn fjoin_rx(&self) -> bool {
        (self.bits & Sm2Shiftctrl::FJOIN_RX_MASK) != 0
    }

    /// Read **FJOIN_TX** `[30:30]`
    ///
    /// When 1, TX FIFO steals the RX FIFO's storage, and becomes twice as deep.
    /// RX FIFO is disabled as a result (always reads as both full and empty).
    /// FIFOs are flushed when this bit is changed.
    #[inline(always)]
    pub const fn fjoin_tx(&self) -> bool {
        (self.bits & Sm2Shiftctrl::FJOIN_TX_MASK) != 0
    }

    /// Read **PULL_THRESH** `[29:25]`
    ///
    /// Number of bits shifted out of OSR before autopull, or conditional pull (PULL IFEMPTY), will take place.
    /// Write 0 for value of 32.
    #[inline(always)]
    pub const fn pull_thresh(&self) -> u32 {
        (self.bits & Sm2Shiftctrl::PULL_THRESH_MASK) >> Sm2Shiftctrl::PULL_THRESH_OFFSET
    }

    /// Read **PUSH_THRESH** `[24:20]`
    ///
    /// Number of bits shifted into ISR before autopush, or conditional push (PUSH IFFULL), will take place.
    /// Write 0 for value of 32.
    #[inline(always)]
    pub const fn push_thresh(&self) -> u32 {
        (self.bits & Sm2Shiftctrl::PUSH_THRESH_MASK) >> Sm2Shiftctrl::PUSH_THRESH_OFFSET
    }

    /// Read **OUT_SHIFTDIR** `[19:19]`
    ///
    /// 1 = shift out of output shift register to right. 0 = to left.
    #[inline(always)]
    pub const fn out_shiftdir(&self) -> bool {
        (self.bits & Sm2Shiftctrl::OUT_SHIFTDIR_MASK) != 0
    }

    /// Read **IN_SHIFTDIR** `[18:18]`
    ///
    /// 1 = shift input shift register to right (data enters from left). 0 = to left.
    #[inline(always)]
    pub const fn in_shiftdir(&self) -> bool {
        (self.bits & Sm2Shiftctrl::IN_SHIFTDIR_MASK) != 0
    }

    /// Read **AUTOPULL** `[17:17]`
    ///
    /// Pull automatically when the output shift register is emptied, i.e. on or following an OUT instruction which causes the output shift counter to reach or exceed PULL_THRESH.
    #[inline(always)]
    pub const fn autopull(&self) -> bool {
        (self.bits & Sm2Shiftctrl::AUTOPULL_MASK) != 0
    }

    /// Read **AUTOPUSH** `[16:16]`
    ///
    /// Push automatically when the input shift register is filled, i.e. on an IN instruction which causes the input shift counter to reach or exceed PUSH_THRESH.
    #[inline(always)]
    pub const fn autopush(&self) -> bool {
        (self.bits & Sm2Shiftctrl::AUTOPUSH_MASK) != 0
    }

    /// Read **FJOIN_RX_PUT** `[15:15]`
    ///
    /// If 1, disable this state machine's RX FIFO, make its storage available for random write access by the state machine (using the `put` instruction) and, unless FJOIN_RX_GET is also set, random read access by the processor (through the RXFx_PUTGETy registers).
    /// If FJOIN_RX_PUT and FJOIN_RX_GET are both set, then the RX FIFO's registers can be randomly read/written by the state machine, but are completely inaccessible to the processor.
    /// Setting this bit will clear the FJOIN_TX and FJOIN_RX bits.
    #[inline(always)]
    pub const fn fjoin_rx_put(&self) -> bool {
        (self.bits & Sm2Shiftctrl::FJOIN_RX_PUT_MASK) != 0
    }

    /// Read **FJOIN_RX_GET** `[14:14]`
    ///
    /// If 1, disable this state machine's RX FIFO, make its storage available for random read access by the state machine (using the `get` instruction) and, unless FJOIN_RX_PUT is also set, random write access by the processor (through the RXFx_PUTGETy registers).
    /// If FJOIN_RX_PUT and FJOIN_RX_GET are both set, then the RX FIFO's registers can be randomly read/written by the state machine, but are completely inaccessible to the processor.
    /// Setting this bit will clear the FJOIN_TX and FJOIN_RX bits.
    #[inline(always)]
    pub const fn fjoin_rx_get(&self) -> bool {
        (self.bits & Sm2Shiftctrl::FJOIN_RX_GET_MASK) != 0
    }

    /// Read **IN_COUNT** `[4:0]`
    ///
    /// Set the number of pins which are not masked to 0 when read by an IN PINS, WAIT PIN or MOV x, PINS instruction.
    /// For example, an IN_COUNT of 5 means that the 5 LSBs of the IN pin group are visible (bits 4:0), but the remaining 27 MSBs are masked to 0. A count of 32 is encoded with a field value of 0, so the default behaviour is to not perform any masking.
    /// Note this masking is applied in addition to the masking usually performed by the IN instruction. This is mainly useful for the MOV x, PINS instruction, which otherwise has no way of masking pins.
    #[inline(always)]
    pub const fn in_count(&self) -> u32 {
        (self.bits & Sm2Shiftctrl::IN_COUNT_MASK) >> Sm2Shiftctrl::IN_COUNT_OFFSET
    }

    /// Set **FJOIN_RX** `[31:31]`
    ///
    /// When 1, RX FIFO steals the TX FIFO's storage, and becomes twice as deep.
    /// TX FIFO is disabled as a result (always reads as both full and empty).
    /// FIFOs are flushed when this bit is changed.
    #[inline(always)]
    pub const fn with_fjoin_rx(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sm2Shiftctrl::FJOIN_RX_MASK
            } else {
                self.bits & !Sm2Shiftctrl::FJOIN_RX_MASK
            }
        }
    }

    /// Set **FJOIN_TX** `[30:30]`
    ///
    /// When 1, TX FIFO steals the RX FIFO's storage, and becomes twice as deep.
    /// RX FIFO is disabled as a result (always reads as both full and empty).
    /// FIFOs are flushed when this bit is changed.
    #[inline(always)]
    pub const fn with_fjoin_tx(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sm2Shiftctrl::FJOIN_TX_MASK
            } else {
                self.bits & !Sm2Shiftctrl::FJOIN_TX_MASK
            }
        }
    }

    /// Set **PULL_THRESH** `[29:25]`
    ///
    /// Number of bits shifted out of OSR before autopull, or conditional pull (PULL IFEMPTY), will take place.
    /// Write 0 for value of 32.
    #[inline(always)]
    pub const fn with_pull_thresh(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sm2Shiftctrl::PULL_THRESH_MASK) | ((value << Sm2Shiftctrl::PULL_THRESH_OFFSET) & Sm2Shiftctrl::PULL_THRESH_MASK)
        }
    }

    /// Set **PUSH_THRESH** `[24:20]`
    ///
    /// Number of bits shifted into ISR before autopush, or conditional push (PUSH IFFULL), will take place.
    /// Write 0 for value of 32.
    #[inline(always)]
    pub const fn with_push_thresh(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sm2Shiftctrl::PUSH_THRESH_MASK) | ((value << Sm2Shiftctrl::PUSH_THRESH_OFFSET) & Sm2Shiftctrl::PUSH_THRESH_MASK)
        }
    }

    /// Set **OUT_SHIFTDIR** `[19:19]`
    ///
    /// 1 = shift out of output shift register to right. 0 = to left.
    #[inline(always)]
    pub const fn with_out_shiftdir(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sm2Shiftctrl::OUT_SHIFTDIR_MASK
            } else {
                self.bits & !Sm2Shiftctrl::OUT_SHIFTDIR_MASK
            }
        }
    }

    /// Set **IN_SHIFTDIR** `[18:18]`
    ///
    /// 1 = shift input shift register to right (data enters from left). 0 = to left.
    #[inline(always)]
    pub const fn with_in_shiftdir(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sm2Shiftctrl::IN_SHIFTDIR_MASK
            } else {
                self.bits & !Sm2Shiftctrl::IN_SHIFTDIR_MASK
            }
        }
    }

    /// Set **AUTOPULL** `[17:17]`
    ///
    /// Pull automatically when the output shift register is emptied, i.e. on or following an OUT instruction which causes the output shift counter to reach or exceed PULL_THRESH.
    #[inline(always)]
    pub const fn with_autopull(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sm2Shiftctrl::AUTOPULL_MASK
            } else {
                self.bits & !Sm2Shiftctrl::AUTOPULL_MASK
            }
        }
    }

    /// Set **AUTOPUSH** `[16:16]`
    ///
    /// Push automatically when the input shift register is filled, i.e. on an IN instruction which causes the input shift counter to reach or exceed PUSH_THRESH.
    #[inline(always)]
    pub const fn with_autopush(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sm2Shiftctrl::AUTOPUSH_MASK
            } else {
                self.bits & !Sm2Shiftctrl::AUTOPUSH_MASK
            }
        }
    }

    /// Set **FJOIN_RX_PUT** `[15:15]`
    ///
    /// If 1, disable this state machine's RX FIFO, make its storage available for random write access by the state machine (using the `put` instruction) and, unless FJOIN_RX_GET is also set, random read access by the processor (through the RXFx_PUTGETy registers).
    /// If FJOIN_RX_PUT and FJOIN_RX_GET are both set, then the RX FIFO's registers can be randomly read/written by the state machine, but are completely inaccessible to the processor.
    /// Setting this bit will clear the FJOIN_TX and FJOIN_RX bits.
    #[inline(always)]
    pub const fn with_fjoin_rx_put(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sm2Shiftctrl::FJOIN_RX_PUT_MASK
            } else {
                self.bits & !Sm2Shiftctrl::FJOIN_RX_PUT_MASK
            }
        }
    }

    /// Set **FJOIN_RX_GET** `[14:14]`
    ///
    /// If 1, disable this state machine's RX FIFO, make its storage available for random read access by the state machine (using the `get` instruction) and, unless FJOIN_RX_PUT is also set, random write access by the processor (through the RXFx_PUTGETy registers).
    /// If FJOIN_RX_PUT and FJOIN_RX_GET are both set, then the RX FIFO's registers can be randomly read/written by the state machine, but are completely inaccessible to the processor.
    /// Setting this bit will clear the FJOIN_TX and FJOIN_RX bits.
    #[inline(always)]
    pub const fn with_fjoin_rx_get(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sm2Shiftctrl::FJOIN_RX_GET_MASK
            } else {
                self.bits & !Sm2Shiftctrl::FJOIN_RX_GET_MASK
            }
        }
    }

    /// Set **IN_COUNT** `[4:0]`
    ///
    /// Set the number of pins which are not masked to 0 when read by an IN PINS, WAIT PIN or MOV x, PINS instruction.
    /// For example, an IN_COUNT of 5 means that the 5 LSBs of the IN pin group are visible (bits 4:0), but the remaining 27 MSBs are masked to 0. A count of 32 is encoded with a field value of 0, so the default behaviour is to not perform any masking.
    /// Note this masking is applied in addition to the masking usually performed by the IN instruction. This is mainly useful for the MOV x, PINS instruction, which otherwise has no way of masking pins.
    #[inline(always)]
    pub const fn with_in_count(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sm2Shiftctrl::IN_COUNT_MASK) | ((value << Sm2Shiftctrl::IN_COUNT_OFFSET) & Sm2Shiftctrl::IN_COUNT_MASK)
        }
    }

}

impl Default for Sm2ShiftctrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Sm2Shiftctrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Sm2ShiftctrlVal {
        Sm2ShiftctrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Sm2ShiftctrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Sm2ShiftctrlVal) -> Sm2ShiftctrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SM2_ADDR Register
///
/// Current instruction address of state machine 2
///
/// Address offset: 0x0104
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SM2_ADDR** `[4:0]` (RO)
#[repr(transparent)]
pub struct Sm2Addr {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Sm2Addr {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sm2Addr {
    // ======== Field Constants ========

    /// **SM2_ADDR**
    /// Bits `[4:0]`, mask `0x0000001f`
    pub const SM2_ADDR_OFFSET: u32 = 0;
    pub const SM2_ADDR_MASK: u32 = 0x0000001f;

    // ======== Field Accessors ========

    /// **SM2_ADDR** `[4:0]` (RO)
    #[inline(always)]
    pub fn sm2_addr(&self) -> u32 {
        (self.value.read_bits() & Self::SM2_ADDR_MASK) >> Self::SM2_ADDR_OFFSET
    }

}

/// SM2_ADDR value type for batch read/modify/write operations
///
/// Use `sm2addr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sm2addr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Sm2AddrVal {
    bits: u32,
}

impl Sm2AddrVal {
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

    /// Read **SM2_ADDR** `[4:0]`
    #[inline(always)]
    pub const fn sm2_addr(&self) -> u32 {
        (self.bits & Sm2Addr::SM2_ADDR_MASK) >> Sm2Addr::SM2_ADDR_OFFSET
    }

}

impl Default for Sm2AddrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Sm2Addr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Sm2AddrVal {
        Sm2AddrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Sm2AddrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Sm2AddrVal) -> Sm2AddrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SM2_INSTR Register
///
/// Read to see the instruction currently addressed by state machine 2's program counter
/// Write to execute an instruction immediately (including jumps) and then resume execution.
///
/// Address offset: 0x0108
///
/// ## Fields
///
/// - **SM2_INSTR** `[15:0]` (RW)
#[repr(transparent)]
pub struct Sm2Instr {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Sm2Instr {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sm2Instr {
    // ======== Field Constants ========

    /// **SM2_INSTR**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const SM2_INSTR_OFFSET: u32 = 0;
    pub const SM2_INSTR_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **SM2_INSTR** `[15:0]` (RW)
    #[inline(always)]
    pub fn sm2_instr(&self) -> u32 {
        (self.value.read_bits() & Self::SM2_INSTR_MASK) >> Self::SM2_INSTR_OFFSET
    }

    /// Write **SM2_INSTR** `[15:0]` (RW)
    #[inline(always)]
    pub fn write_sm2_instr(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SM2_INSTR_MASK) | ((value << Self::SM2_INSTR_OFFSET) & Self::SM2_INSTR_MASK));
    }

}

/// SM2_INSTR value type for batch read/modify/write operations
///
/// Use `sm2instr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sm2instr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Sm2InstrVal {
    bits: u32,
}

impl Sm2InstrVal {
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

    /// Read **SM2_INSTR** `[15:0]`
    #[inline(always)]
    pub const fn sm2_instr(&self) -> u32 {
        (self.bits & Sm2Instr::SM2_INSTR_MASK) >> Sm2Instr::SM2_INSTR_OFFSET
    }

    /// Set **SM2_INSTR** `[15:0]`
    #[inline(always)]
    pub const fn with_sm2_instr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sm2Instr::SM2_INSTR_MASK) | ((value << Sm2Instr::SM2_INSTR_OFFSET) & Sm2Instr::SM2_INSTR_MASK)
        }
    }

}

impl Default for Sm2InstrVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Sm2Instr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Sm2InstrVal {
        Sm2InstrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Sm2InstrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Sm2InstrVal) -> Sm2InstrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SM2_PINCTRL Register
///
/// State machine pin control
///
/// Address offset: 0x010c
/// Reset value: 0x14000000
///
/// ## Fields
///
/// - **SIDESET_COUNT** `[31:29]` (RW)
///
///   The number of MSBs of the Delay/Side-set instruction field which are used for side-set. Inclusive of the enable bit, if present. Minimum of 0 (all delay bits, no side-set) and maximum of 5 (all side-set, no delay).
///
/// - **SET_COUNT** `[28:26]` (RW)
///
///   The number of pins asserted by a SET. In the range 0 to 5 inclusive.
///
/// - **OUT_COUNT** `[25:20]` (RW)
///
///   The number of pins asserted by an OUT PINS, OUT PINDIRS or MOV PINS instruction. In the range 0 to 32 inclusive.
///
/// - **IN_BASE** `[19:15]` (RW)
///
///   The pin which is mapped to the least-significant bit of a state machine's IN data bus. Higher-numbered pins are mapped to consecutively more-significant data bits, with a modulo of 32 applied to pin number.
///
/// - **SIDESET_BASE** `[14:10]` (RW)
///
///   The lowest-numbered pin that will be affected by a side-set operation. The MSBs of an instruction's side-set/delay field (up to 5, determined by SIDESET_COUNT) are used for side-set data, with the remaining LSBs used for delay. The least-significant bit of the side-set portion is the bit written to this pin, with more-significant bits written to higher-numbered pins.
///
/// - **SET_BASE** `[9:5]` (RW)
///
///   The lowest-numbered pin that will be affected by a SET PINS or SET PINDIRS instruction. The data written to this pin is the least-significant bit of the SET data.
///
/// - **OUT_BASE** `[4:0]` (RW)
///
///   The lowest-numbered pin that will be affected by an OUT PINS, OUT PINDIRS or MOV PINS instruction. The data written to this pin will always be the least-significant bit of the OUT or MOV data.
#[repr(transparent)]
pub struct Sm2Pinctrl {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Sm2Pinctrl {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sm2Pinctrl {
    // ======== Field Constants ========

    /// **SIDESET_COUNT**
    ///
    /// The number of MSBs of the Delay/Side-set instruction field which are used for side-set. Inclusive of the enable bit, if present. Minimum of 0 (all delay bits, no side-set) and maximum of 5 (all side-set, no delay).
    ///
    /// Bits `[31:29]`, mask `0xe0000000`
    pub const SIDESET_COUNT_OFFSET: u32 = 29;
    pub const SIDESET_COUNT_MASK: u32 = 0xe0000000;

    /// **SET_COUNT**
    ///
    /// The number of pins asserted by a SET. In the range 0 to 5 inclusive.
    ///
    /// Bits `[28:26]`, mask `0x1c000000`
    pub const SET_COUNT_OFFSET: u32 = 26;
    pub const SET_COUNT_MASK: u32 = 0x1c000000;

    /// **OUT_COUNT**
    ///
    /// The number of pins asserted by an OUT PINS, OUT PINDIRS or MOV PINS instruction. In the range 0 to 32 inclusive.
    ///
    /// Bits `[25:20]`, mask `0x03f00000`
    pub const OUT_COUNT_OFFSET: u32 = 20;
    pub const OUT_COUNT_MASK: u32 = 0x03f00000;

    /// **IN_BASE**
    ///
    /// The pin which is mapped to the least-significant bit of a state machine's IN data bus. Higher-numbered pins are mapped to consecutively more-significant data bits, with a modulo of 32 applied to pin number.
    ///
    /// Bits `[19:15]`, mask `0x000f8000`
    pub const IN_BASE_OFFSET: u32 = 15;
    pub const IN_BASE_MASK: u32 = 0x000f8000;

    /// **SIDESET_BASE**
    ///
    /// The lowest-numbered pin that will be affected by a side-set operation. The MSBs of an instruction's side-set/delay field (up to 5, determined by SIDESET_COUNT) are used for side-set data, with the remaining LSBs used for delay. The least-significant bit of the side-set portion is the bit written to this pin, with more-significant bits written to higher-numbered pins.
    ///
    /// Bits `[14:10]`, mask `0x00007c00`
    pub const SIDESET_BASE_OFFSET: u32 = 10;
    pub const SIDESET_BASE_MASK: u32 = 0x00007c00;

    /// **SET_BASE**
    ///
    /// The lowest-numbered pin that will be affected by a SET PINS or SET PINDIRS instruction. The data written to this pin is the least-significant bit of the SET data.
    ///
    /// Bits `[9:5]`, mask `0x000003e0`
    pub const SET_BASE_OFFSET: u32 = 5;
    pub const SET_BASE_MASK: u32 = 0x000003e0;

    /// **OUT_BASE**
    ///
    /// The lowest-numbered pin that will be affected by an OUT PINS, OUT PINDIRS or MOV PINS instruction. The data written to this pin will always be the least-significant bit of the OUT or MOV data.
    ///
    /// Bits `[4:0]`, mask `0x0000001f`
    pub const OUT_BASE_OFFSET: u32 = 0;
    pub const OUT_BASE_MASK: u32 = 0x0000001f;

    // ======== Field Accessors ========

    /// **SIDESET_COUNT** `[31:29]` (RW)
    ///
    /// The number of MSBs of the Delay/Side-set instruction field which are used for side-set. Inclusive of the enable bit, if present. Minimum of 0 (all delay bits, no side-set) and maximum of 5 (all side-set, no delay).
    #[inline(always)]
    pub fn sideset_count(&self) -> u32 {
        (self.value.read_bits() & Self::SIDESET_COUNT_MASK) >> Self::SIDESET_COUNT_OFFSET
    }

    /// Write **SIDESET_COUNT** `[31:29]` (RW)
    ///
    /// The number of MSBs of the Delay/Side-set instruction field which are used for side-set. Inclusive of the enable bit, if present. Minimum of 0 (all delay bits, no side-set) and maximum of 5 (all side-set, no delay).
    #[inline(always)]
    pub fn write_sideset_count(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SIDESET_COUNT_MASK) | ((value << Self::SIDESET_COUNT_OFFSET) & Self::SIDESET_COUNT_MASK));
    }

    /// **SET_COUNT** `[28:26]` (RW)
    ///
    /// The number of pins asserted by a SET. In the range 0 to 5 inclusive.
    #[inline(always)]
    pub fn set_count(&self) -> u32 {
        (self.value.read_bits() & Self::SET_COUNT_MASK) >> Self::SET_COUNT_OFFSET
    }

    /// Write **SET_COUNT** `[28:26]` (RW)
    ///
    /// The number of pins asserted by a SET. In the range 0 to 5 inclusive.
    #[inline(always)]
    pub fn write_set_count(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SET_COUNT_MASK) | ((value << Self::SET_COUNT_OFFSET) & Self::SET_COUNT_MASK));
    }

    /// **OUT_COUNT** `[25:20]` (RW)
    ///
    /// The number of pins asserted by an OUT PINS, OUT PINDIRS or MOV PINS instruction. In the range 0 to 32 inclusive.
    #[inline(always)]
    pub fn out_count(&self) -> u32 {
        (self.value.read_bits() & Self::OUT_COUNT_MASK) >> Self::OUT_COUNT_OFFSET
    }

    /// Write **OUT_COUNT** `[25:20]` (RW)
    ///
    /// The number of pins asserted by an OUT PINS, OUT PINDIRS or MOV PINS instruction. In the range 0 to 32 inclusive.
    #[inline(always)]
    pub fn write_out_count(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::OUT_COUNT_MASK) | ((value << Self::OUT_COUNT_OFFSET) & Self::OUT_COUNT_MASK));
    }

    /// **IN_BASE** `[19:15]` (RW)
    ///
    /// The pin which is mapped to the least-significant bit of a state machine's IN data bus. Higher-numbered pins are mapped to consecutively more-significant data bits, with a modulo of 32 applied to pin number.
    #[inline(always)]
    pub fn in_base(&self) -> u32 {
        (self.value.read_bits() & Self::IN_BASE_MASK) >> Self::IN_BASE_OFFSET
    }

    /// Write **IN_BASE** `[19:15]` (RW)
    ///
    /// The pin which is mapped to the least-significant bit of a state machine's IN data bus. Higher-numbered pins are mapped to consecutively more-significant data bits, with a modulo of 32 applied to pin number.
    #[inline(always)]
    pub fn write_in_base(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::IN_BASE_MASK) | ((value << Self::IN_BASE_OFFSET) & Self::IN_BASE_MASK));
    }

    /// **SIDESET_BASE** `[14:10]` (RW)
    ///
    /// The lowest-numbered pin that will be affected by a side-set operation. The MSBs of an instruction's side-set/delay field (up to 5, determined by SIDESET_COUNT) are used for side-set data, with the remaining LSBs used for delay. The least-significant bit of the side-set portion is the bit written to this pin, with more-significant bits written to higher-numbered pins.
    #[inline(always)]
    pub fn sideset_base(&self) -> u32 {
        (self.value.read_bits() & Self::SIDESET_BASE_MASK) >> Self::SIDESET_BASE_OFFSET
    }

    /// Write **SIDESET_BASE** `[14:10]` (RW)
    ///
    /// The lowest-numbered pin that will be affected by a side-set operation. The MSBs of an instruction's side-set/delay field (up to 5, determined by SIDESET_COUNT) are used for side-set data, with the remaining LSBs used for delay. The least-significant bit of the side-set portion is the bit written to this pin, with more-significant bits written to higher-numbered pins.
    #[inline(always)]
    pub fn write_sideset_base(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SIDESET_BASE_MASK) | ((value << Self::SIDESET_BASE_OFFSET) & Self::SIDESET_BASE_MASK));
    }

    /// **SET_BASE** `[9:5]` (RW)
    ///
    /// The lowest-numbered pin that will be affected by a SET PINS or SET PINDIRS instruction. The data written to this pin is the least-significant bit of the SET data.
    #[inline(always)]
    pub fn set_base(&self) -> u32 {
        (self.value.read_bits() & Self::SET_BASE_MASK) >> Self::SET_BASE_OFFSET
    }

    /// Write **SET_BASE** `[9:5]` (RW)
    ///
    /// The lowest-numbered pin that will be affected by a SET PINS or SET PINDIRS instruction. The data written to this pin is the least-significant bit of the SET data.
    #[inline(always)]
    pub fn write_set_base(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SET_BASE_MASK) | ((value << Self::SET_BASE_OFFSET) & Self::SET_BASE_MASK));
    }

    /// **OUT_BASE** `[4:0]` (RW)
    ///
    /// The lowest-numbered pin that will be affected by an OUT PINS, OUT PINDIRS or MOV PINS instruction. The data written to this pin will always be the least-significant bit of the OUT or MOV data.
    #[inline(always)]
    pub fn out_base(&self) -> u32 {
        (self.value.read_bits() & Self::OUT_BASE_MASK) >> Self::OUT_BASE_OFFSET
    }

    /// Write **OUT_BASE** `[4:0]` (RW)
    ///
    /// The lowest-numbered pin that will be affected by an OUT PINS, OUT PINDIRS or MOV PINS instruction. The data written to this pin will always be the least-significant bit of the OUT or MOV data.
    #[inline(always)]
    pub fn write_out_base(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::OUT_BASE_MASK) | ((value << Self::OUT_BASE_OFFSET) & Self::OUT_BASE_MASK));
    }

}

/// SM2_PINCTRL value type for batch read/modify/write operations
///
/// Use `sm2pinctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sm2pinctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Sm2PinctrlVal {
    bits: u32,
}

impl Sm2PinctrlVal {
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
        Self { bits: 0x14000000 }
    }

    /// Read **SIDESET_COUNT** `[31:29]`
    ///
    /// The number of MSBs of the Delay/Side-set instruction field which are used for side-set. Inclusive of the enable bit, if present. Minimum of 0 (all delay bits, no side-set) and maximum of 5 (all side-set, no delay).
    #[inline(always)]
    pub const fn sideset_count(&self) -> u32 {
        (self.bits & Sm2Pinctrl::SIDESET_COUNT_MASK) >> Sm2Pinctrl::SIDESET_COUNT_OFFSET
    }

    /// Read **SET_COUNT** `[28:26]`
    ///
    /// The number of pins asserted by a SET. In the range 0 to 5 inclusive.
    #[inline(always)]
    pub const fn set_count(&self) -> u32 {
        (self.bits & Sm2Pinctrl::SET_COUNT_MASK) >> Sm2Pinctrl::SET_COUNT_OFFSET
    }

    /// Read **OUT_COUNT** `[25:20]`
    ///
    /// The number of pins asserted by an OUT PINS, OUT PINDIRS or MOV PINS instruction. In the range 0 to 32 inclusive.
    #[inline(always)]
    pub const fn out_count(&self) -> u32 {
        (self.bits & Sm2Pinctrl::OUT_COUNT_MASK) >> Sm2Pinctrl::OUT_COUNT_OFFSET
    }

    /// Read **IN_BASE** `[19:15]`
    ///
    /// The pin which is mapped to the least-significant bit of a state machine's IN data bus. Higher-numbered pins are mapped to consecutively more-significant data bits, with a modulo of 32 applied to pin number.
    #[inline(always)]
    pub const fn in_base(&self) -> u32 {
        (self.bits & Sm2Pinctrl::IN_BASE_MASK) >> Sm2Pinctrl::IN_BASE_OFFSET
    }

    /// Read **SIDESET_BASE** `[14:10]`
    ///
    /// The lowest-numbered pin that will be affected by a side-set operation. The MSBs of an instruction's side-set/delay field (up to 5, determined by SIDESET_COUNT) are used for side-set data, with the remaining LSBs used for delay. The least-significant bit of the side-set portion is the bit written to this pin, with more-significant bits written to higher-numbered pins.
    #[inline(always)]
    pub const fn sideset_base(&self) -> u32 {
        (self.bits & Sm2Pinctrl::SIDESET_BASE_MASK) >> Sm2Pinctrl::SIDESET_BASE_OFFSET
    }

    /// Read **SET_BASE** `[9:5]`
    ///
    /// The lowest-numbered pin that will be affected by a SET PINS or SET PINDIRS instruction. The data written to this pin is the least-significant bit of the SET data.
    #[inline(always)]
    pub const fn set_base(&self) -> u32 {
        (self.bits & Sm2Pinctrl::SET_BASE_MASK) >> Sm2Pinctrl::SET_BASE_OFFSET
    }

    /// Read **OUT_BASE** `[4:0]`
    ///
    /// The lowest-numbered pin that will be affected by an OUT PINS, OUT PINDIRS or MOV PINS instruction. The data written to this pin will always be the least-significant bit of the OUT or MOV data.
    #[inline(always)]
    pub const fn out_base(&self) -> u32 {
        (self.bits & Sm2Pinctrl::OUT_BASE_MASK) >> Sm2Pinctrl::OUT_BASE_OFFSET
    }

    /// Set **SIDESET_COUNT** `[31:29]`
    ///
    /// The number of MSBs of the Delay/Side-set instruction field which are used for side-set. Inclusive of the enable bit, if present. Minimum of 0 (all delay bits, no side-set) and maximum of 5 (all side-set, no delay).
    #[inline(always)]
    pub const fn with_sideset_count(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sm2Pinctrl::SIDESET_COUNT_MASK) | ((value << Sm2Pinctrl::SIDESET_COUNT_OFFSET) & Sm2Pinctrl::SIDESET_COUNT_MASK)
        }
    }

    /// Set **SET_COUNT** `[28:26]`
    ///
    /// The number of pins asserted by a SET. In the range 0 to 5 inclusive.
    #[inline(always)]
    pub const fn with_set_count(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sm2Pinctrl::SET_COUNT_MASK) | ((value << Sm2Pinctrl::SET_COUNT_OFFSET) & Sm2Pinctrl::SET_COUNT_MASK)
        }
    }

    /// Set **OUT_COUNT** `[25:20]`
    ///
    /// The number of pins asserted by an OUT PINS, OUT PINDIRS or MOV PINS instruction. In the range 0 to 32 inclusive.
    #[inline(always)]
    pub const fn with_out_count(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sm2Pinctrl::OUT_COUNT_MASK) | ((value << Sm2Pinctrl::OUT_COUNT_OFFSET) & Sm2Pinctrl::OUT_COUNT_MASK)
        }
    }

    /// Set **IN_BASE** `[19:15]`
    ///
    /// The pin which is mapped to the least-significant bit of a state machine's IN data bus. Higher-numbered pins are mapped to consecutively more-significant data bits, with a modulo of 32 applied to pin number.
    #[inline(always)]
    pub const fn with_in_base(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sm2Pinctrl::IN_BASE_MASK) | ((value << Sm2Pinctrl::IN_BASE_OFFSET) & Sm2Pinctrl::IN_BASE_MASK)
        }
    }

    /// Set **SIDESET_BASE** `[14:10]`
    ///
    /// The lowest-numbered pin that will be affected by a side-set operation. The MSBs of an instruction's side-set/delay field (up to 5, determined by SIDESET_COUNT) are used for side-set data, with the remaining LSBs used for delay. The least-significant bit of the side-set portion is the bit written to this pin, with more-significant bits written to higher-numbered pins.
    #[inline(always)]
    pub const fn with_sideset_base(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sm2Pinctrl::SIDESET_BASE_MASK) | ((value << Sm2Pinctrl::SIDESET_BASE_OFFSET) & Sm2Pinctrl::SIDESET_BASE_MASK)
        }
    }

    /// Set **SET_BASE** `[9:5]`
    ///
    /// The lowest-numbered pin that will be affected by a SET PINS or SET PINDIRS instruction. The data written to this pin is the least-significant bit of the SET data.
    #[inline(always)]
    pub const fn with_set_base(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sm2Pinctrl::SET_BASE_MASK) | ((value << Sm2Pinctrl::SET_BASE_OFFSET) & Sm2Pinctrl::SET_BASE_MASK)
        }
    }

    /// Set **OUT_BASE** `[4:0]`
    ///
    /// The lowest-numbered pin that will be affected by an OUT PINS, OUT PINDIRS or MOV PINS instruction. The data written to this pin will always be the least-significant bit of the OUT or MOV data.
    #[inline(always)]
    pub const fn with_out_base(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sm2Pinctrl::OUT_BASE_MASK) | ((value << Sm2Pinctrl::OUT_BASE_OFFSET) & Sm2Pinctrl::OUT_BASE_MASK)
        }
    }

}

impl Default for Sm2PinctrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Sm2Pinctrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Sm2PinctrlVal {
        Sm2PinctrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Sm2PinctrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Sm2PinctrlVal) -> Sm2PinctrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SM3_CLKDIV Register
///
/// Clock divisor register for state machine 3
/// Frequency = clock freq / (CLKDIV_INT + CLKDIV_FRAC / 256)
///
/// Address offset: 0x0110
/// Reset value: 0x00010000
///
/// ## Fields
///
/// - **INT** `[31:16]` (RW)
///
///   Effective frequency is sysclk/(int + frac/256).
///   Value of 0 is interpreted as 65536. If INT is 0, FRAC must also be 0.
///
/// - **FRAC** `[15:8]` (RW)
///
///   Fractional part of clock divisor
#[repr(transparent)]
pub struct Sm3Clkdiv {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Sm3Clkdiv {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sm3Clkdiv {
    // ======== Field Constants ========

    /// **INT**
    ///
    /// Effective frequency is sysclk/(int + frac/256).
    /// Value of 0 is interpreted as 65536. If INT is 0, FRAC must also be 0.
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const INT_OFFSET: u32 = 16;
    pub const INT_MASK: u32 = 0xffff0000;

    /// **FRAC**
    ///
    /// Fractional part of clock divisor
    ///
    /// Bits `[15:8]`, mask `0x0000ff00`
    pub const FRAC_OFFSET: u32 = 8;
    pub const FRAC_MASK: u32 = 0x0000ff00;

    // ======== Field Accessors ========

    /// **INT** `[31:16]` (RW)
    ///
    /// Effective frequency is sysclk/(int + frac/256).
    /// Value of 0 is interpreted as 65536. If INT is 0, FRAC must also be 0.
    #[inline(always)]
    pub fn int(&self) -> u32 {
        (self.value.read_bits() & Self::INT_MASK) >> Self::INT_OFFSET
    }

    /// Write **INT** `[31:16]` (RW)
    ///
    /// Effective frequency is sysclk/(int + frac/256).
    /// Value of 0 is interpreted as 65536. If INT is 0, FRAC must also be 0.
    #[inline(always)]
    pub fn write_int(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::INT_MASK) | ((value << Self::INT_OFFSET) & Self::INT_MASK));
    }

    /// **FRAC** `[15:8]` (RW)
    ///
    /// Fractional part of clock divisor
    #[inline(always)]
    pub fn frac(&self) -> u32 {
        (self.value.read_bits() & Self::FRAC_MASK) >> Self::FRAC_OFFSET
    }

    /// Write **FRAC** `[15:8]` (RW)
    ///
    /// Fractional part of clock divisor
    #[inline(always)]
    pub fn write_frac(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::FRAC_MASK) | ((value << Self::FRAC_OFFSET) & Self::FRAC_MASK));
    }

}

/// SM3_CLKDIV value type for batch read/modify/write operations
///
/// Use `sm3clkdiv.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sm3clkdiv.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Sm3ClkdivVal {
    bits: u32,
}

impl Sm3ClkdivVal {
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
        Self { bits: 0x00010000 }
    }

    /// Read **INT** `[31:16]`
    ///
    /// Effective frequency is sysclk/(int + frac/256).
    /// Value of 0 is interpreted as 65536. If INT is 0, FRAC must also be 0.
    #[inline(always)]
    pub const fn int(&self) -> u32 {
        (self.bits & Sm3Clkdiv::INT_MASK) >> Sm3Clkdiv::INT_OFFSET
    }

    /// Read **FRAC** `[15:8]`
    ///
    /// Fractional part of clock divisor
    #[inline(always)]
    pub const fn frac(&self) -> u32 {
        (self.bits & Sm3Clkdiv::FRAC_MASK) >> Sm3Clkdiv::FRAC_OFFSET
    }

    /// Set **INT** `[31:16]`
    ///
    /// Effective frequency is sysclk/(int + frac/256).
    /// Value of 0 is interpreted as 65536. If INT is 0, FRAC must also be 0.
    #[inline(always)]
    pub const fn with_int(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sm3Clkdiv::INT_MASK) | ((value << Sm3Clkdiv::INT_OFFSET) & Sm3Clkdiv::INT_MASK)
        }
    }

    /// Set **FRAC** `[15:8]`
    ///
    /// Fractional part of clock divisor
    #[inline(always)]
    pub const fn with_frac(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sm3Clkdiv::FRAC_MASK) | ((value << Sm3Clkdiv::FRAC_OFFSET) & Sm3Clkdiv::FRAC_MASK)
        }
    }

}

impl Default for Sm3ClkdivVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Sm3Clkdiv {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Sm3ClkdivVal {
        Sm3ClkdivVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Sm3ClkdivVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Sm3ClkdivVal) -> Sm3ClkdivVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SM3_EXECCTRL Register
///
/// Execution/behavioural settings for state machine 3
///
/// Address offset: 0x0114
/// Reset value: 0x0001f000
///
/// ## Fields
///
/// - **EXEC_STALLED** `[31]` (RO)
///
///   If 1, an instruction written to SMx_INSTR is stalled, and latched by the state machine. Will clear to 0 once this instruction completes.
///
/// - **SIDE_EN** `[30]` (RW)
///
///   If 1, the MSB of the Delay/Side-set instruction field is used as side-set enable, rather than a side-set data bit. This allows instructions to perform side-set optionally, rather than on every instruction, but the maximum possible side-set width is reduced from 5 to 4. Note that the value of PINCTRL_SIDESET_COUNT is inclusive of this enable bit.
///
/// - **SIDE_PINDIR** `[29]` (RW)
///
///   If 1, side-set data is asserted to pin directions, instead of pin values
///
/// - **JMP_PIN** `[28:24]` (RW)
///
///   The GPIO number to use as condition for JMP PIN. Unaffected by input mapping.
///
/// - **OUT_EN_SEL** `[23:19]` (RW)
///
///   Which data bit to use for inline OUT enable
///
/// - **INLINE_OUT_EN** `[18]` (RW)
///
///   If 1, use a bit of OUT data as an auxiliary write enable
///   When used in conjunction with OUT_STICKY, writes with an enable of 0 will
///   deassert the latest pin write. This can create useful masking/override behaviour
///   due to the priority ordering of state machine pin writes (SM0 < SM1 < ...)
///
/// - **OUT_STICKY** `[17]` (RW)
///
///   Continuously assert the most recent OUT/SET to the pins
///
/// - **WRAP_TOP** `[16:12]` (RW)
///
///   After reaching this address, execution is wrapped to wrap_bottom.
///   If the instruction is a jump, and the jump condition is true, the jump takes priority.
///
/// - **WRAP_BOTTOM** `[11:7]` (RW)
///
///   After reaching wrap_top, execution is wrapped to this address.
///
/// - **STATUS_SEL** `[6:5]` (RW)
///
///   Comparison used for the MOV x, STATUS instruction.
///
/// - **STATUS_N** `[4:0]` (RW)
///
///   Comparison level or IRQ index for the MOV x, STATUS instruction.
///   If STATUS_SEL is TXLEVEL or RXLEVEL, then values of STATUS_N greater than the current FIFO depth are reserved, and have undefined behaviour.
#[repr(transparent)]
pub struct Sm3Execctrl {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Sm3Execctrl {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sm3Execctrl {
    // ======== Field Constants ========

    /// **EXEC_STALLED**
    ///
    /// If 1, an instruction written to SMx_INSTR is stalled, and latched by the state machine. Will clear to 0 once this instruction completes.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const EXEC_STALLED_OFFSET: u32 = 31;
    pub const EXEC_STALLED_MASK: u32 = 0x80000000;
    pub const EXEC_STALLED: u32 = 1 << 31;

    /// **SIDE_EN**
    ///
    /// If 1, the MSB of the Delay/Side-set instruction field is used as side-set enable, rather than a side-set data bit. This allows instructions to perform side-set optionally, rather than on every instruction, but the maximum possible side-set width is reduced from 5 to 4. Note that the value of PINCTRL_SIDESET_COUNT is inclusive of this enable bit.
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const SIDE_EN_OFFSET: u32 = 30;
    pub const SIDE_EN_MASK: u32 = 0x40000000;
    pub const SIDE_EN: u32 = 1 << 30;

    /// **SIDE_PINDIR**
    ///
    /// If 1, side-set data is asserted to pin directions, instead of pin values
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const SIDE_PINDIR_OFFSET: u32 = 29;
    pub const SIDE_PINDIR_MASK: u32 = 0x20000000;
    pub const SIDE_PINDIR: u32 = 1 << 29;

    /// **JMP_PIN**
    ///
    /// The GPIO number to use as condition for JMP PIN. Unaffected by input mapping.
    ///
    /// Bits `[28:24]`, mask `0x1f000000`
    pub const JMP_PIN_OFFSET: u32 = 24;
    pub const JMP_PIN_MASK: u32 = 0x1f000000;

    /// **OUT_EN_SEL**
    ///
    /// Which data bit to use for inline OUT enable
    ///
    /// Bits `[23:19]`, mask `0x00f80000`
    pub const OUT_EN_SEL_OFFSET: u32 = 19;
    pub const OUT_EN_SEL_MASK: u32 = 0x00f80000;

    /// **INLINE_OUT_EN**
    ///
    /// If 1, use a bit of OUT data as an auxiliary write enable
    /// When used in conjunction with OUT_STICKY, writes with an enable of 0 will
    /// deassert the latest pin write. This can create useful masking/override behaviour
    /// due to the priority ordering of state machine pin writes (SM0 < SM1 < ...)
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const INLINE_OUT_EN_OFFSET: u32 = 18;
    pub const INLINE_OUT_EN_MASK: u32 = 0x00040000;
    pub const INLINE_OUT_EN: u32 = 1 << 18;

    /// **OUT_STICKY**
    ///
    /// Continuously assert the most recent OUT/SET to the pins
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const OUT_STICKY_OFFSET: u32 = 17;
    pub const OUT_STICKY_MASK: u32 = 0x00020000;
    pub const OUT_STICKY: u32 = 1 << 17;

    /// **WRAP_TOP**
    ///
    /// After reaching this address, execution is wrapped to wrap_bottom.
    /// If the instruction is a jump, and the jump condition is true, the jump takes priority.
    ///
    /// Bits `[16:12]`, mask `0x0001f000`
    pub const WRAP_TOP_OFFSET: u32 = 12;
    pub const WRAP_TOP_MASK: u32 = 0x0001f000;

    /// **WRAP_BOTTOM**
    ///
    /// After reaching wrap_top, execution is wrapped to this address.
    ///
    /// Bits `[11:7]`, mask `0x00000f80`
    pub const WRAP_BOTTOM_OFFSET: u32 = 7;
    pub const WRAP_BOTTOM_MASK: u32 = 0x00000f80;

    /// **STATUS_SEL**
    ///
    /// Comparison used for the MOV x, STATUS instruction.
    ///
    /// Bits `[6:5]`, mask `0x00000060`
    pub const STATUS_SEL_OFFSET: u32 = 5;
    pub const STATUS_SEL_MASK: u32 = 0x00000060;
    /// TXLEVEL = 0
    /// All-ones if TX FIFO level < N, otherwise all-zeroes
    pub const STATUS_SEL_TXLEVEL: u32 = 0 << 5;
    pub const STATUS_SEL_TXLEVEL_VAL: u32 = 0;
    /// RXLEVEL = 1
    /// All-ones if RX FIFO level < N, otherwise all-zeroes
    pub const STATUS_SEL_RXLEVEL: u32 = 1 << 5;
    pub const STATUS_SEL_RXLEVEL_VAL: u32 = 1;
    /// IRQ = 2
    /// All-ones if the indexed IRQ flag is raised, otherwise all-zeroes
    pub const STATUS_SEL_IRQ: u32 = 2 << 5;
    pub const STATUS_SEL_IRQ_VAL: u32 = 2;

    /// **STATUS_N**
    ///
    /// Comparison level or IRQ index for the MOV x, STATUS instruction.
    /// If STATUS_SEL is TXLEVEL or RXLEVEL, then values of STATUS_N greater than the current FIFO depth are reserved, and have undefined behaviour.
    ///
    /// Bits `[4:0]`, mask `0x0000001f`
    pub const STATUS_N_OFFSET: u32 = 0;
    pub const STATUS_N_MASK: u32 = 0x0000001f;
    /// IRQ = 0
    /// Index 0-7 of an IRQ flag in this PIO block
    pub const STATUS_N_IRQ: u32 = 0 << 0;
    pub const STATUS_N_IRQ_VAL: u32 = 0;
    /// IRQ_PREVPIO = 8
    /// Index 0-7 of an IRQ flag in the next lower-numbered PIO block
    pub const STATUS_N_IRQ_PREVPIO: u32 = 8 << 0;
    pub const STATUS_N_IRQ_PREVPIO_VAL: u32 = 8;
    /// IRQ_NEXTPIO = 16
    /// Index 0-7 of an IRQ flag in the next higher-numbered PIO block
    pub const STATUS_N_IRQ_NEXTPIO: u32 = 16 << 0;
    pub const STATUS_N_IRQ_NEXTPIO_VAL: u32 = 16;

    // ======== Field Accessors ========

    /// **EXEC_STALLED** `[31]` (RO)
    ///
    /// If 1, an instruction written to SMx_INSTR is stalled, and latched by the state machine. Will clear to 0 once this instruction completes.
    #[inline(always)]
    pub fn exec_stalled(&self) -> bool {
        (self.value.read_bits() & Self::EXEC_STALLED_MASK) != 0
    }

    /// **SIDE_EN** `[30]` (RW)
    ///
    /// If 1, the MSB of the Delay/Side-set instruction field is used as side-set enable, rather than a side-set data bit. This allows instructions to perform side-set optionally, rather than on every instruction, but the maximum possible side-set width is reduced from 5 to 4. Note that the value of PINCTRL_SIDESET_COUNT is inclusive of this enable bit.
    #[inline(always)]
    pub fn side_en(&self) -> bool {
        (self.value.read_bits() & Self::SIDE_EN_MASK) != 0
    }

    /// Set SIDE_EN bit atomically (RW)
    #[inline(always)]
    pub fn set_side_en(&self) {
        self.set_bits(Self::SIDE_EN_MASK);
    }

    /// Clear SIDE_EN bit atomically (RW)
    #[inline(always)]
    pub fn clr_side_en(&self) {
        self.clr_bits(Self::SIDE_EN_MASK);
    }

    /// **SIDE_PINDIR** `[29]` (RW)
    ///
    /// If 1, side-set data is asserted to pin directions, instead of pin values
    #[inline(always)]
    pub fn side_pindir(&self) -> bool {
        (self.value.read_bits() & Self::SIDE_PINDIR_MASK) != 0
    }

    /// Set SIDE_PINDIR bit atomically (RW)
    #[inline(always)]
    pub fn set_side_pindir(&self) {
        self.set_bits(Self::SIDE_PINDIR_MASK);
    }

    /// Clear SIDE_PINDIR bit atomically (RW)
    #[inline(always)]
    pub fn clr_side_pindir(&self) {
        self.clr_bits(Self::SIDE_PINDIR_MASK);
    }

    /// **JMP_PIN** `[28:24]` (RW)
    ///
    /// The GPIO number to use as condition for JMP PIN. Unaffected by input mapping.
    #[inline(always)]
    pub fn jmp_pin(&self) -> u32 {
        (self.value.read_bits() & Self::JMP_PIN_MASK) >> Self::JMP_PIN_OFFSET
    }

    /// Write **JMP_PIN** `[28:24]` (RW)
    ///
    /// The GPIO number to use as condition for JMP PIN. Unaffected by input mapping.
    #[inline(always)]
    pub fn write_jmp_pin(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::JMP_PIN_MASK) | ((value << Self::JMP_PIN_OFFSET) & Self::JMP_PIN_MASK));
    }

    /// **OUT_EN_SEL** `[23:19]` (RW)
    ///
    /// Which data bit to use for inline OUT enable
    #[inline(always)]
    pub fn out_en_sel(&self) -> u32 {
        (self.value.read_bits() & Self::OUT_EN_SEL_MASK) >> Self::OUT_EN_SEL_OFFSET
    }

    /// Write **OUT_EN_SEL** `[23:19]` (RW)
    ///
    /// Which data bit to use for inline OUT enable
    #[inline(always)]
    pub fn write_out_en_sel(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::OUT_EN_SEL_MASK) | ((value << Self::OUT_EN_SEL_OFFSET) & Self::OUT_EN_SEL_MASK));
    }

    /// **INLINE_OUT_EN** `[18]` (RW)
    ///
    /// If 1, use a bit of OUT data as an auxiliary write enable
    /// When used in conjunction with OUT_STICKY, writes with an enable of 0 will
    /// deassert the latest pin write. This can create useful masking/override behaviour
    /// due to the priority ordering of state machine pin writes (SM0 < SM1 < ...)
    #[inline(always)]
    pub fn inline_out_en(&self) -> bool {
        (self.value.read_bits() & Self::INLINE_OUT_EN_MASK) != 0
    }

    /// Set INLINE_OUT_EN bit atomically (RW)
    #[inline(always)]
    pub fn set_inline_out_en(&self) {
        self.set_bits(Self::INLINE_OUT_EN_MASK);
    }

    /// Clear INLINE_OUT_EN bit atomically (RW)
    #[inline(always)]
    pub fn clr_inline_out_en(&self) {
        self.clr_bits(Self::INLINE_OUT_EN_MASK);
    }

    /// **OUT_STICKY** `[17]` (RW)
    ///
    /// Continuously assert the most recent OUT/SET to the pins
    #[inline(always)]
    pub fn out_sticky(&self) -> bool {
        (self.value.read_bits() & Self::OUT_STICKY_MASK) != 0
    }

    /// Set OUT_STICKY bit atomically (RW)
    #[inline(always)]
    pub fn set_out_sticky(&self) {
        self.set_bits(Self::OUT_STICKY_MASK);
    }

    /// Clear OUT_STICKY bit atomically (RW)
    #[inline(always)]
    pub fn clr_out_sticky(&self) {
        self.clr_bits(Self::OUT_STICKY_MASK);
    }

    /// **WRAP_TOP** `[16:12]` (RW)
    ///
    /// After reaching this address, execution is wrapped to wrap_bottom.
    /// If the instruction is a jump, and the jump condition is true, the jump takes priority.
    #[inline(always)]
    pub fn wrap_top(&self) -> u32 {
        (self.value.read_bits() & Self::WRAP_TOP_MASK) >> Self::WRAP_TOP_OFFSET
    }

    /// Write **WRAP_TOP** `[16:12]` (RW)
    ///
    /// After reaching this address, execution is wrapped to wrap_bottom.
    /// If the instruction is a jump, and the jump condition is true, the jump takes priority.
    #[inline(always)]
    pub fn write_wrap_top(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::WRAP_TOP_MASK) | ((value << Self::WRAP_TOP_OFFSET) & Self::WRAP_TOP_MASK));
    }

    /// **WRAP_BOTTOM** `[11:7]` (RW)
    ///
    /// After reaching wrap_top, execution is wrapped to this address.
    #[inline(always)]
    pub fn wrap_bottom(&self) -> u32 {
        (self.value.read_bits() & Self::WRAP_BOTTOM_MASK) >> Self::WRAP_BOTTOM_OFFSET
    }

    /// Write **WRAP_BOTTOM** `[11:7]` (RW)
    ///
    /// After reaching wrap_top, execution is wrapped to this address.
    #[inline(always)]
    pub fn write_wrap_bottom(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::WRAP_BOTTOM_MASK) | ((value << Self::WRAP_BOTTOM_OFFSET) & Self::WRAP_BOTTOM_MASK));
    }

    /// **STATUS_SEL** `[6:5]` (RW)
    ///
    /// Comparison used for the MOV x, STATUS instruction.
    #[inline(always)]
    pub fn status_sel(&self) -> u32 {
        (self.value.read_bits() & Self::STATUS_SEL_MASK) >> Self::STATUS_SEL_OFFSET
    }

    /// Write **STATUS_SEL** `[6:5]` (RW)
    ///
    /// Comparison used for the MOV x, STATUS instruction.
    #[inline(always)]
    pub fn write_status_sel(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::STATUS_SEL_MASK) | ((value << Self::STATUS_SEL_OFFSET) & Self::STATUS_SEL_MASK));
    }

    /// **STATUS_N** `[4:0]` (RW)
    ///
    /// Comparison level or IRQ index for the MOV x, STATUS instruction.
    /// If STATUS_SEL is TXLEVEL or RXLEVEL, then values of STATUS_N greater than the current FIFO depth are reserved, and have undefined behaviour.
    #[inline(always)]
    pub fn status_n(&self) -> u32 {
        (self.value.read_bits() & Self::STATUS_N_MASK) >> Self::STATUS_N_OFFSET
    }

    /// Write **STATUS_N** `[4:0]` (RW)
    ///
    /// Comparison level or IRQ index for the MOV x, STATUS instruction.
    /// If STATUS_SEL is TXLEVEL or RXLEVEL, then values of STATUS_N greater than the current FIFO depth are reserved, and have undefined behaviour.
    #[inline(always)]
    pub fn write_status_n(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::STATUS_N_MASK) | ((value << Self::STATUS_N_OFFSET) & Self::STATUS_N_MASK));
    }

}

/// SM3_EXECCTRL value type for batch read/modify/write operations
///
/// Use `sm3execctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sm3execctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Sm3ExecctrlVal {
    bits: u32,
}

impl Sm3ExecctrlVal {
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
        Self { bits: 0x0001f000 }
    }

    /// Read **EXEC_STALLED** `[31:31]`
    ///
    /// If 1, an instruction written to SMx_INSTR is stalled, and latched by the state machine. Will clear to 0 once this instruction completes.
    #[inline(always)]
    pub const fn exec_stalled(&self) -> bool {
        (self.bits & Sm3Execctrl::EXEC_STALLED_MASK) != 0
    }

    /// Read **SIDE_EN** `[30:30]`
    ///
    /// If 1, the MSB of the Delay/Side-set instruction field is used as side-set enable, rather than a side-set data bit. This allows instructions to perform side-set optionally, rather than on every instruction, but the maximum possible side-set width is reduced from 5 to 4. Note that the value of PINCTRL_SIDESET_COUNT is inclusive of this enable bit.
    #[inline(always)]
    pub const fn side_en(&self) -> bool {
        (self.bits & Sm3Execctrl::SIDE_EN_MASK) != 0
    }

    /// Read **SIDE_PINDIR** `[29:29]`
    ///
    /// If 1, side-set data is asserted to pin directions, instead of pin values
    #[inline(always)]
    pub const fn side_pindir(&self) -> bool {
        (self.bits & Sm3Execctrl::SIDE_PINDIR_MASK) != 0
    }

    /// Read **JMP_PIN** `[28:24]`
    ///
    /// The GPIO number to use as condition for JMP PIN. Unaffected by input mapping.
    #[inline(always)]
    pub const fn jmp_pin(&self) -> u32 {
        (self.bits & Sm3Execctrl::JMP_PIN_MASK) >> Sm3Execctrl::JMP_PIN_OFFSET
    }

    /// Read **OUT_EN_SEL** `[23:19]`
    ///
    /// Which data bit to use for inline OUT enable
    #[inline(always)]
    pub const fn out_en_sel(&self) -> u32 {
        (self.bits & Sm3Execctrl::OUT_EN_SEL_MASK) >> Sm3Execctrl::OUT_EN_SEL_OFFSET
    }

    /// Read **INLINE_OUT_EN** `[18:18]`
    ///
    /// If 1, use a bit of OUT data as an auxiliary write enable
    /// When used in conjunction with OUT_STICKY, writes with an enable of 0 will
    /// deassert the latest pin write. This can create useful masking/override behaviour
    /// due to the priority ordering of state machine pin writes (SM0 < SM1 < ...)
    #[inline(always)]
    pub const fn inline_out_en(&self) -> bool {
        (self.bits & Sm3Execctrl::INLINE_OUT_EN_MASK) != 0
    }

    /// Read **OUT_STICKY** `[17:17]`
    ///
    /// Continuously assert the most recent OUT/SET to the pins
    #[inline(always)]
    pub const fn out_sticky(&self) -> bool {
        (self.bits & Sm3Execctrl::OUT_STICKY_MASK) != 0
    }

    /// Read **WRAP_TOP** `[16:12]`
    ///
    /// After reaching this address, execution is wrapped to wrap_bottom.
    /// If the instruction is a jump, and the jump condition is true, the jump takes priority.
    #[inline(always)]
    pub const fn wrap_top(&self) -> u32 {
        (self.bits & Sm3Execctrl::WRAP_TOP_MASK) >> Sm3Execctrl::WRAP_TOP_OFFSET
    }

    /// Read **WRAP_BOTTOM** `[11:7]`
    ///
    /// After reaching wrap_top, execution is wrapped to this address.
    #[inline(always)]
    pub const fn wrap_bottom(&self) -> u32 {
        (self.bits & Sm3Execctrl::WRAP_BOTTOM_MASK) >> Sm3Execctrl::WRAP_BOTTOM_OFFSET
    }

    /// Read **STATUS_SEL** `[6:5]`
    ///
    /// Comparison used for the MOV x, STATUS instruction.
    #[inline(always)]
    pub const fn status_sel(&self) -> u32 {
        (self.bits & Sm3Execctrl::STATUS_SEL_MASK) >> Sm3Execctrl::STATUS_SEL_OFFSET
    }

    /// Read **STATUS_N** `[4:0]`
    ///
    /// Comparison level or IRQ index for the MOV x, STATUS instruction.
    /// If STATUS_SEL is TXLEVEL or RXLEVEL, then values of STATUS_N greater than the current FIFO depth are reserved, and have undefined behaviour.
    #[inline(always)]
    pub const fn status_n(&self) -> u32 {
        (self.bits & Sm3Execctrl::STATUS_N_MASK) >> Sm3Execctrl::STATUS_N_OFFSET
    }

    /// Set **SIDE_EN** `[30:30]`
    ///
    /// If 1, the MSB of the Delay/Side-set instruction field is used as side-set enable, rather than a side-set data bit. This allows instructions to perform side-set optionally, rather than on every instruction, but the maximum possible side-set width is reduced from 5 to 4. Note that the value of PINCTRL_SIDESET_COUNT is inclusive of this enable bit.
    #[inline(always)]
    pub const fn with_side_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sm3Execctrl::SIDE_EN_MASK
            } else {
                self.bits & !Sm3Execctrl::SIDE_EN_MASK
            }
        }
    }

    /// Set **SIDE_PINDIR** `[29:29]`
    ///
    /// If 1, side-set data is asserted to pin directions, instead of pin values
    #[inline(always)]
    pub const fn with_side_pindir(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sm3Execctrl::SIDE_PINDIR_MASK
            } else {
                self.bits & !Sm3Execctrl::SIDE_PINDIR_MASK
            }
        }
    }

    /// Set **JMP_PIN** `[28:24]`
    ///
    /// The GPIO number to use as condition for JMP PIN. Unaffected by input mapping.
    #[inline(always)]
    pub const fn with_jmp_pin(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sm3Execctrl::JMP_PIN_MASK) | ((value << Sm3Execctrl::JMP_PIN_OFFSET) & Sm3Execctrl::JMP_PIN_MASK)
        }
    }

    /// Set **OUT_EN_SEL** `[23:19]`
    ///
    /// Which data bit to use for inline OUT enable
    #[inline(always)]
    pub const fn with_out_en_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sm3Execctrl::OUT_EN_SEL_MASK) | ((value << Sm3Execctrl::OUT_EN_SEL_OFFSET) & Sm3Execctrl::OUT_EN_SEL_MASK)
        }
    }

    /// Set **INLINE_OUT_EN** `[18:18]`
    ///
    /// If 1, use a bit of OUT data as an auxiliary write enable
    /// When used in conjunction with OUT_STICKY, writes with an enable of 0 will
    /// deassert the latest pin write. This can create useful masking/override behaviour
    /// due to the priority ordering of state machine pin writes (SM0 < SM1 < ...)
    #[inline(always)]
    pub const fn with_inline_out_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sm3Execctrl::INLINE_OUT_EN_MASK
            } else {
                self.bits & !Sm3Execctrl::INLINE_OUT_EN_MASK
            }
        }
    }

    /// Set **OUT_STICKY** `[17:17]`
    ///
    /// Continuously assert the most recent OUT/SET to the pins
    #[inline(always)]
    pub const fn with_out_sticky(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sm3Execctrl::OUT_STICKY_MASK
            } else {
                self.bits & !Sm3Execctrl::OUT_STICKY_MASK
            }
        }
    }

    /// Set **WRAP_TOP** `[16:12]`
    ///
    /// After reaching this address, execution is wrapped to wrap_bottom.
    /// If the instruction is a jump, and the jump condition is true, the jump takes priority.
    #[inline(always)]
    pub const fn with_wrap_top(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sm3Execctrl::WRAP_TOP_MASK) | ((value << Sm3Execctrl::WRAP_TOP_OFFSET) & Sm3Execctrl::WRAP_TOP_MASK)
        }
    }

    /// Set **WRAP_BOTTOM** `[11:7]`
    ///
    /// After reaching wrap_top, execution is wrapped to this address.
    #[inline(always)]
    pub const fn with_wrap_bottom(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sm3Execctrl::WRAP_BOTTOM_MASK) | ((value << Sm3Execctrl::WRAP_BOTTOM_OFFSET) & Sm3Execctrl::WRAP_BOTTOM_MASK)
        }
    }

    /// Set **STATUS_SEL** `[6:5]`
    ///
    /// Comparison used for the MOV x, STATUS instruction.
    #[inline(always)]
    pub const fn with_status_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sm3Execctrl::STATUS_SEL_MASK) | ((value << Sm3Execctrl::STATUS_SEL_OFFSET) & Sm3Execctrl::STATUS_SEL_MASK)
        }
    }

    /// Set **STATUS_N** `[4:0]`
    ///
    /// Comparison level or IRQ index for the MOV x, STATUS instruction.
    /// If STATUS_SEL is TXLEVEL or RXLEVEL, then values of STATUS_N greater than the current FIFO depth are reserved, and have undefined behaviour.
    #[inline(always)]
    pub const fn with_status_n(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sm3Execctrl::STATUS_N_MASK) | ((value << Sm3Execctrl::STATUS_N_OFFSET) & Sm3Execctrl::STATUS_N_MASK)
        }
    }

}

impl Default for Sm3ExecctrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Sm3Execctrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Sm3ExecctrlVal {
        Sm3ExecctrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Sm3ExecctrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Sm3ExecctrlVal) -> Sm3ExecctrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SM3_SHIFTCTRL Register
///
/// Control behaviour of the input/output shift registers for state machine 3
///
/// Address offset: 0x0118
/// Reset value: 0x000c0000
///
/// ## Fields
///
/// - **FJOIN_RX** `[31]` (RW)
///
///   When 1, RX FIFO steals the TX FIFO's storage, and becomes twice as deep.
///   TX FIFO is disabled as a result (always reads as both full and empty).
///   FIFOs are flushed when this bit is changed.
///
/// - **FJOIN_TX** `[30]` (RW)
///
///   When 1, TX FIFO steals the RX FIFO's storage, and becomes twice as deep.
///   RX FIFO is disabled as a result (always reads as both full and empty).
///   FIFOs are flushed when this bit is changed.
///
/// - **PULL_THRESH** `[29:25]` (RW)
///
///   Number of bits shifted out of OSR before autopull, or conditional pull (PULL IFEMPTY), will take place.
///   Write 0 for value of 32.
///
/// - **PUSH_THRESH** `[24:20]` (RW)
///
///   Number of bits shifted into ISR before autopush, or conditional push (PUSH IFFULL), will take place.
///   Write 0 for value of 32.
///
/// - **OUT_SHIFTDIR** `[19]` (RW)
///
///   1 = shift out of output shift register to right. 0 = to left.
///
/// - **IN_SHIFTDIR** `[18]` (RW)
///
///   1 = shift input shift register to right (data enters from left). 0 = to left.
///
/// - **AUTOPULL** `[17]` (RW)
///
///   Pull automatically when the output shift register is emptied, i.e. on or following an OUT instruction which causes the output shift counter to reach or exceed PULL_THRESH.
///
/// - **AUTOPUSH** `[16]` (RW)
///
///   Push automatically when the input shift register is filled, i.e. on an IN instruction which causes the input shift counter to reach or exceed PUSH_THRESH.
///
/// - **FJOIN_RX_PUT** `[15]` (RW)
///
///   If 1, disable this state machine's RX FIFO, make its storage available for random write access by the state machine (using the `put` instruction) and, unless FJOIN_RX_GET is also set, random read access by the processor (through the RXFx_PUTGETy registers).
///   If FJOIN_RX_PUT and FJOIN_RX_GET are both set, then the RX FIFO's registers can be randomly read/written by the state machine, but are completely inaccessible to the processor.
///   Setting this bit will clear the FJOIN_TX and FJOIN_RX bits.
///
/// - **FJOIN_RX_GET** `[14]` (RW)
///
///   If 1, disable this state machine's RX FIFO, make its storage available for random read access by the state machine (using the `get` instruction) and, unless FJOIN_RX_PUT is also set, random write access by the processor (through the RXFx_PUTGETy registers).
///   If FJOIN_RX_PUT and FJOIN_RX_GET are both set, then the RX FIFO's registers can be randomly read/written by the state machine, but are completely inaccessible to the processor.
///   Setting this bit will clear the FJOIN_TX and FJOIN_RX bits.
///
/// - **IN_COUNT** `[4:0]` (RW)
///
///   Set the number of pins which are not masked to 0 when read by an IN PINS, WAIT PIN or MOV x, PINS instruction.
///   For example, an IN_COUNT of 5 means that the 5 LSBs of the IN pin group are visible (bits 4:0), but the remaining 27 MSBs are masked to 0. A count of 32 is encoded with a field value of 0, so the default behaviour is to not perform any masking.
///   Note this masking is applied in addition to the masking usually performed by the IN instruction. This is mainly useful for the MOV x, PINS instruction, which otherwise has no way of masking pins.
#[repr(transparent)]
pub struct Sm3Shiftctrl {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Sm3Shiftctrl {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sm3Shiftctrl {
    // ======== Field Constants ========

    /// **FJOIN_RX**
    ///
    /// When 1, RX FIFO steals the TX FIFO's storage, and becomes twice as deep.
    /// TX FIFO is disabled as a result (always reads as both full and empty).
    /// FIFOs are flushed when this bit is changed.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const FJOIN_RX_OFFSET: u32 = 31;
    pub const FJOIN_RX_MASK: u32 = 0x80000000;
    pub const FJOIN_RX: u32 = 1 << 31;

    /// **FJOIN_TX**
    ///
    /// When 1, TX FIFO steals the RX FIFO's storage, and becomes twice as deep.
    /// RX FIFO is disabled as a result (always reads as both full and empty).
    /// FIFOs are flushed when this bit is changed.
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const FJOIN_TX_OFFSET: u32 = 30;
    pub const FJOIN_TX_MASK: u32 = 0x40000000;
    pub const FJOIN_TX: u32 = 1 << 30;

    /// **PULL_THRESH**
    ///
    /// Number of bits shifted out of OSR before autopull, or conditional pull (PULL IFEMPTY), will take place.
    /// Write 0 for value of 32.
    ///
    /// Bits `[29:25]`, mask `0x3e000000`
    pub const PULL_THRESH_OFFSET: u32 = 25;
    pub const PULL_THRESH_MASK: u32 = 0x3e000000;

    /// **PUSH_THRESH**
    ///
    /// Number of bits shifted into ISR before autopush, or conditional push (PUSH IFFULL), will take place.
    /// Write 0 for value of 32.
    ///
    /// Bits `[24:20]`, mask `0x01f00000`
    pub const PUSH_THRESH_OFFSET: u32 = 20;
    pub const PUSH_THRESH_MASK: u32 = 0x01f00000;

    /// **OUT_SHIFTDIR**
    ///
    /// 1 = shift out of output shift register to right. 0 = to left.
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const OUT_SHIFTDIR_OFFSET: u32 = 19;
    pub const OUT_SHIFTDIR_MASK: u32 = 0x00080000;
    pub const OUT_SHIFTDIR: u32 = 1 << 19;

    /// **IN_SHIFTDIR**
    ///
    /// 1 = shift input shift register to right (data enters from left). 0 = to left.
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const IN_SHIFTDIR_OFFSET: u32 = 18;
    pub const IN_SHIFTDIR_MASK: u32 = 0x00040000;
    pub const IN_SHIFTDIR: u32 = 1 << 18;

    /// **AUTOPULL**
    ///
    /// Pull automatically when the output shift register is emptied, i.e. on or following an OUT instruction which causes the output shift counter to reach or exceed PULL_THRESH.
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const AUTOPULL_OFFSET: u32 = 17;
    pub const AUTOPULL_MASK: u32 = 0x00020000;
    pub const AUTOPULL: u32 = 1 << 17;

    /// **AUTOPUSH**
    ///
    /// Push automatically when the input shift register is filled, i.e. on an IN instruction which causes the input shift counter to reach or exceed PUSH_THRESH.
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const AUTOPUSH_OFFSET: u32 = 16;
    pub const AUTOPUSH_MASK: u32 = 0x00010000;
    pub const AUTOPUSH: u32 = 1 << 16;

    /// **FJOIN_RX_PUT**
    ///
    /// If 1, disable this state machine's RX FIFO, make its storage available for random write access by the state machine (using the `put` instruction) and, unless FJOIN_RX_GET is also set, random read access by the processor (through the RXFx_PUTGETy registers).
    /// If FJOIN_RX_PUT and FJOIN_RX_GET are both set, then the RX FIFO's registers can be randomly read/written by the state machine, but are completely inaccessible to the processor.
    /// Setting this bit will clear the FJOIN_TX and FJOIN_RX bits.
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const FJOIN_RX_PUT_OFFSET: u32 = 15;
    pub const FJOIN_RX_PUT_MASK: u32 = 0x00008000;
    pub const FJOIN_RX_PUT: u32 = 1 << 15;

    /// **FJOIN_RX_GET**
    ///
    /// If 1, disable this state machine's RX FIFO, make its storage available for random read access by the state machine (using the `get` instruction) and, unless FJOIN_RX_PUT is also set, random write access by the processor (through the RXFx_PUTGETy registers).
    /// If FJOIN_RX_PUT and FJOIN_RX_GET are both set, then the RX FIFO's registers can be randomly read/written by the state machine, but are completely inaccessible to the processor.
    /// Setting this bit will clear the FJOIN_TX and FJOIN_RX bits.
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const FJOIN_RX_GET_OFFSET: u32 = 14;
    pub const FJOIN_RX_GET_MASK: u32 = 0x00004000;
    pub const FJOIN_RX_GET: u32 = 1 << 14;

    /// **IN_COUNT**
    ///
    /// Set the number of pins which are not masked to 0 when read by an IN PINS, WAIT PIN or MOV x, PINS instruction.
    /// For example, an IN_COUNT of 5 means that the 5 LSBs of the IN pin group are visible (bits 4:0), but the remaining 27 MSBs are masked to 0. A count of 32 is encoded with a field value of 0, so the default behaviour is to not perform any masking.
    /// Note this masking is applied in addition to the masking usually performed by the IN instruction. This is mainly useful for the MOV x, PINS instruction, which otherwise has no way of masking pins.
    ///
    /// Bits `[4:0]`, mask `0x0000001f`
    pub const IN_COUNT_OFFSET: u32 = 0;
    pub const IN_COUNT_MASK: u32 = 0x0000001f;

    // ======== Field Accessors ========

    /// **FJOIN_RX** `[31]` (RW)
    ///
    /// When 1, RX FIFO steals the TX FIFO's storage, and becomes twice as deep.
    /// TX FIFO is disabled as a result (always reads as both full and empty).
    /// FIFOs are flushed when this bit is changed.
    #[inline(always)]
    pub fn fjoin_rx(&self) -> bool {
        (self.value.read_bits() & Self::FJOIN_RX_MASK) != 0
    }

    /// Set FJOIN_RX bit atomically (RW)
    #[inline(always)]
    pub fn set_fjoin_rx(&self) {
        self.set_bits(Self::FJOIN_RX_MASK);
    }

    /// Clear FJOIN_RX bit atomically (RW)
    #[inline(always)]
    pub fn clr_fjoin_rx(&self) {
        self.clr_bits(Self::FJOIN_RX_MASK);
    }

    /// **FJOIN_TX** `[30]` (RW)
    ///
    /// When 1, TX FIFO steals the RX FIFO's storage, and becomes twice as deep.
    /// RX FIFO is disabled as a result (always reads as both full and empty).
    /// FIFOs are flushed when this bit is changed.
    #[inline(always)]
    pub fn fjoin_tx(&self) -> bool {
        (self.value.read_bits() & Self::FJOIN_TX_MASK) != 0
    }

    /// Set FJOIN_TX bit atomically (RW)
    #[inline(always)]
    pub fn set_fjoin_tx(&self) {
        self.set_bits(Self::FJOIN_TX_MASK);
    }

    /// Clear FJOIN_TX bit atomically (RW)
    #[inline(always)]
    pub fn clr_fjoin_tx(&self) {
        self.clr_bits(Self::FJOIN_TX_MASK);
    }

    /// **PULL_THRESH** `[29:25]` (RW)
    ///
    /// Number of bits shifted out of OSR before autopull, or conditional pull (PULL IFEMPTY), will take place.
    /// Write 0 for value of 32.
    #[inline(always)]
    pub fn pull_thresh(&self) -> u32 {
        (self.value.read_bits() & Self::PULL_THRESH_MASK) >> Self::PULL_THRESH_OFFSET
    }

    /// Write **PULL_THRESH** `[29:25]` (RW)
    ///
    /// Number of bits shifted out of OSR before autopull, or conditional pull (PULL IFEMPTY), will take place.
    /// Write 0 for value of 32.
    #[inline(always)]
    pub fn write_pull_thresh(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::PULL_THRESH_MASK) | ((value << Self::PULL_THRESH_OFFSET) & Self::PULL_THRESH_MASK));
    }

    /// **PUSH_THRESH** `[24:20]` (RW)
    ///
    /// Number of bits shifted into ISR before autopush, or conditional push (PUSH IFFULL), will take place.
    /// Write 0 for value of 32.
    #[inline(always)]
    pub fn push_thresh(&self) -> u32 {
        (self.value.read_bits() & Self::PUSH_THRESH_MASK) >> Self::PUSH_THRESH_OFFSET
    }

    /// Write **PUSH_THRESH** `[24:20]` (RW)
    ///
    /// Number of bits shifted into ISR before autopush, or conditional push (PUSH IFFULL), will take place.
    /// Write 0 for value of 32.
    #[inline(always)]
    pub fn write_push_thresh(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::PUSH_THRESH_MASK) | ((value << Self::PUSH_THRESH_OFFSET) & Self::PUSH_THRESH_MASK));
    }

    /// **OUT_SHIFTDIR** `[19]` (RW)
    ///
    /// 1 = shift out of output shift register to right. 0 = to left.
    #[inline(always)]
    pub fn out_shiftdir(&self) -> bool {
        (self.value.read_bits() & Self::OUT_SHIFTDIR_MASK) != 0
    }

    /// Set OUT_SHIFTDIR bit atomically (RW)
    #[inline(always)]
    pub fn set_out_shiftdir(&self) {
        self.set_bits(Self::OUT_SHIFTDIR_MASK);
    }

    /// Clear OUT_SHIFTDIR bit atomically (RW)
    #[inline(always)]
    pub fn clr_out_shiftdir(&self) {
        self.clr_bits(Self::OUT_SHIFTDIR_MASK);
    }

    /// **IN_SHIFTDIR** `[18]` (RW)
    ///
    /// 1 = shift input shift register to right (data enters from left). 0 = to left.
    #[inline(always)]
    pub fn in_shiftdir(&self) -> bool {
        (self.value.read_bits() & Self::IN_SHIFTDIR_MASK) != 0
    }

    /// Set IN_SHIFTDIR bit atomically (RW)
    #[inline(always)]
    pub fn set_in_shiftdir(&self) {
        self.set_bits(Self::IN_SHIFTDIR_MASK);
    }

    /// Clear IN_SHIFTDIR bit atomically (RW)
    #[inline(always)]
    pub fn clr_in_shiftdir(&self) {
        self.clr_bits(Self::IN_SHIFTDIR_MASK);
    }

    /// **AUTOPULL** `[17]` (RW)
    ///
    /// Pull automatically when the output shift register is emptied, i.e. on or following an OUT instruction which causes the output shift counter to reach or exceed PULL_THRESH.
    #[inline(always)]
    pub fn autopull(&self) -> bool {
        (self.value.read_bits() & Self::AUTOPULL_MASK) != 0
    }

    /// Set AUTOPULL bit atomically (RW)
    #[inline(always)]
    pub fn set_autopull(&self) {
        self.set_bits(Self::AUTOPULL_MASK);
    }

    /// Clear AUTOPULL bit atomically (RW)
    #[inline(always)]
    pub fn clr_autopull(&self) {
        self.clr_bits(Self::AUTOPULL_MASK);
    }

    /// **AUTOPUSH** `[16]` (RW)
    ///
    /// Push automatically when the input shift register is filled, i.e. on an IN instruction which causes the input shift counter to reach or exceed PUSH_THRESH.
    #[inline(always)]
    pub fn autopush(&self) -> bool {
        (self.value.read_bits() & Self::AUTOPUSH_MASK) != 0
    }

    /// Set AUTOPUSH bit atomically (RW)
    #[inline(always)]
    pub fn set_autopush(&self) {
        self.set_bits(Self::AUTOPUSH_MASK);
    }

    /// Clear AUTOPUSH bit atomically (RW)
    #[inline(always)]
    pub fn clr_autopush(&self) {
        self.clr_bits(Self::AUTOPUSH_MASK);
    }

    /// **FJOIN_RX_PUT** `[15]` (RW)
    ///
    /// If 1, disable this state machine's RX FIFO, make its storage available for random write access by the state machine (using the `put` instruction) and, unless FJOIN_RX_GET is also set, random read access by the processor (through the RXFx_PUTGETy registers).
    /// If FJOIN_RX_PUT and FJOIN_RX_GET are both set, then the RX FIFO's registers can be randomly read/written by the state machine, but are completely inaccessible to the processor.
    /// Setting this bit will clear the FJOIN_TX and FJOIN_RX bits.
    #[inline(always)]
    pub fn fjoin_rx_put(&self) -> bool {
        (self.value.read_bits() & Self::FJOIN_RX_PUT_MASK) != 0
    }

    /// Set FJOIN_RX_PUT bit atomically (RW)
    #[inline(always)]
    pub fn set_fjoin_rx_put(&self) {
        self.set_bits(Self::FJOIN_RX_PUT_MASK);
    }

    /// Clear FJOIN_RX_PUT bit atomically (RW)
    #[inline(always)]
    pub fn clr_fjoin_rx_put(&self) {
        self.clr_bits(Self::FJOIN_RX_PUT_MASK);
    }

    /// **FJOIN_RX_GET** `[14]` (RW)
    ///
    /// If 1, disable this state machine's RX FIFO, make its storage available for random read access by the state machine (using the `get` instruction) and, unless FJOIN_RX_PUT is also set, random write access by the processor (through the RXFx_PUTGETy registers).
    /// If FJOIN_RX_PUT and FJOIN_RX_GET are both set, then the RX FIFO's registers can be randomly read/written by the state machine, but are completely inaccessible to the processor.
    /// Setting this bit will clear the FJOIN_TX and FJOIN_RX bits.
    #[inline(always)]
    pub fn fjoin_rx_get(&self) -> bool {
        (self.value.read_bits() & Self::FJOIN_RX_GET_MASK) != 0
    }

    /// Set FJOIN_RX_GET bit atomically (RW)
    #[inline(always)]
    pub fn set_fjoin_rx_get(&self) {
        self.set_bits(Self::FJOIN_RX_GET_MASK);
    }

    /// Clear FJOIN_RX_GET bit atomically (RW)
    #[inline(always)]
    pub fn clr_fjoin_rx_get(&self) {
        self.clr_bits(Self::FJOIN_RX_GET_MASK);
    }

    /// **IN_COUNT** `[4:0]` (RW)
    ///
    /// Set the number of pins which are not masked to 0 when read by an IN PINS, WAIT PIN or MOV x, PINS instruction.
    /// For example, an IN_COUNT of 5 means that the 5 LSBs of the IN pin group are visible (bits 4:0), but the remaining 27 MSBs are masked to 0. A count of 32 is encoded with a field value of 0, so the default behaviour is to not perform any masking.
    /// Note this masking is applied in addition to the masking usually performed by the IN instruction. This is mainly useful for the MOV x, PINS instruction, which otherwise has no way of masking pins.
    #[inline(always)]
    pub fn in_count(&self) -> u32 {
        (self.value.read_bits() & Self::IN_COUNT_MASK) >> Self::IN_COUNT_OFFSET
    }

    /// Write **IN_COUNT** `[4:0]` (RW)
    ///
    /// Set the number of pins which are not masked to 0 when read by an IN PINS, WAIT PIN or MOV x, PINS instruction.
    /// For example, an IN_COUNT of 5 means that the 5 LSBs of the IN pin group are visible (bits 4:0), but the remaining 27 MSBs are masked to 0. A count of 32 is encoded with a field value of 0, so the default behaviour is to not perform any masking.
    /// Note this masking is applied in addition to the masking usually performed by the IN instruction. This is mainly useful for the MOV x, PINS instruction, which otherwise has no way of masking pins.
    #[inline(always)]
    pub fn write_in_count(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::IN_COUNT_MASK) | ((value << Self::IN_COUNT_OFFSET) & Self::IN_COUNT_MASK));
    }

}

/// SM3_SHIFTCTRL value type for batch read/modify/write operations
///
/// Use `sm3shiftctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sm3shiftctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Sm3ShiftctrlVal {
    bits: u32,
}

impl Sm3ShiftctrlVal {
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
        Self { bits: 0x000c0000 }
    }

    /// Read **FJOIN_RX** `[31:31]`
    ///
    /// When 1, RX FIFO steals the TX FIFO's storage, and becomes twice as deep.
    /// TX FIFO is disabled as a result (always reads as both full and empty).
    /// FIFOs are flushed when this bit is changed.
    #[inline(always)]
    pub const fn fjoin_rx(&self) -> bool {
        (self.bits & Sm3Shiftctrl::FJOIN_RX_MASK) != 0
    }

    /// Read **FJOIN_TX** `[30:30]`
    ///
    /// When 1, TX FIFO steals the RX FIFO's storage, and becomes twice as deep.
    /// RX FIFO is disabled as a result (always reads as both full and empty).
    /// FIFOs are flushed when this bit is changed.
    #[inline(always)]
    pub const fn fjoin_tx(&self) -> bool {
        (self.bits & Sm3Shiftctrl::FJOIN_TX_MASK) != 0
    }

    /// Read **PULL_THRESH** `[29:25]`
    ///
    /// Number of bits shifted out of OSR before autopull, or conditional pull (PULL IFEMPTY), will take place.
    /// Write 0 for value of 32.
    #[inline(always)]
    pub const fn pull_thresh(&self) -> u32 {
        (self.bits & Sm3Shiftctrl::PULL_THRESH_MASK) >> Sm3Shiftctrl::PULL_THRESH_OFFSET
    }

    /// Read **PUSH_THRESH** `[24:20]`
    ///
    /// Number of bits shifted into ISR before autopush, or conditional push (PUSH IFFULL), will take place.
    /// Write 0 for value of 32.
    #[inline(always)]
    pub const fn push_thresh(&self) -> u32 {
        (self.bits & Sm3Shiftctrl::PUSH_THRESH_MASK) >> Sm3Shiftctrl::PUSH_THRESH_OFFSET
    }

    /// Read **OUT_SHIFTDIR** `[19:19]`
    ///
    /// 1 = shift out of output shift register to right. 0 = to left.
    #[inline(always)]
    pub const fn out_shiftdir(&self) -> bool {
        (self.bits & Sm3Shiftctrl::OUT_SHIFTDIR_MASK) != 0
    }

    /// Read **IN_SHIFTDIR** `[18:18]`
    ///
    /// 1 = shift input shift register to right (data enters from left). 0 = to left.
    #[inline(always)]
    pub const fn in_shiftdir(&self) -> bool {
        (self.bits & Sm3Shiftctrl::IN_SHIFTDIR_MASK) != 0
    }

    /// Read **AUTOPULL** `[17:17]`
    ///
    /// Pull automatically when the output shift register is emptied, i.e. on or following an OUT instruction which causes the output shift counter to reach or exceed PULL_THRESH.
    #[inline(always)]
    pub const fn autopull(&self) -> bool {
        (self.bits & Sm3Shiftctrl::AUTOPULL_MASK) != 0
    }

    /// Read **AUTOPUSH** `[16:16]`
    ///
    /// Push automatically when the input shift register is filled, i.e. on an IN instruction which causes the input shift counter to reach or exceed PUSH_THRESH.
    #[inline(always)]
    pub const fn autopush(&self) -> bool {
        (self.bits & Sm3Shiftctrl::AUTOPUSH_MASK) != 0
    }

    /// Read **FJOIN_RX_PUT** `[15:15]`
    ///
    /// If 1, disable this state machine's RX FIFO, make its storage available for random write access by the state machine (using the `put` instruction) and, unless FJOIN_RX_GET is also set, random read access by the processor (through the RXFx_PUTGETy registers).
    /// If FJOIN_RX_PUT and FJOIN_RX_GET are both set, then the RX FIFO's registers can be randomly read/written by the state machine, but are completely inaccessible to the processor.
    /// Setting this bit will clear the FJOIN_TX and FJOIN_RX bits.
    #[inline(always)]
    pub const fn fjoin_rx_put(&self) -> bool {
        (self.bits & Sm3Shiftctrl::FJOIN_RX_PUT_MASK) != 0
    }

    /// Read **FJOIN_RX_GET** `[14:14]`
    ///
    /// If 1, disable this state machine's RX FIFO, make its storage available for random read access by the state machine (using the `get` instruction) and, unless FJOIN_RX_PUT is also set, random write access by the processor (through the RXFx_PUTGETy registers).
    /// If FJOIN_RX_PUT and FJOIN_RX_GET are both set, then the RX FIFO's registers can be randomly read/written by the state machine, but are completely inaccessible to the processor.
    /// Setting this bit will clear the FJOIN_TX and FJOIN_RX bits.
    #[inline(always)]
    pub const fn fjoin_rx_get(&self) -> bool {
        (self.bits & Sm3Shiftctrl::FJOIN_RX_GET_MASK) != 0
    }

    /// Read **IN_COUNT** `[4:0]`
    ///
    /// Set the number of pins which are not masked to 0 when read by an IN PINS, WAIT PIN or MOV x, PINS instruction.
    /// For example, an IN_COUNT of 5 means that the 5 LSBs of the IN pin group are visible (bits 4:0), but the remaining 27 MSBs are masked to 0. A count of 32 is encoded with a field value of 0, so the default behaviour is to not perform any masking.
    /// Note this masking is applied in addition to the masking usually performed by the IN instruction. This is mainly useful for the MOV x, PINS instruction, which otherwise has no way of masking pins.
    #[inline(always)]
    pub const fn in_count(&self) -> u32 {
        (self.bits & Sm3Shiftctrl::IN_COUNT_MASK) >> Sm3Shiftctrl::IN_COUNT_OFFSET
    }

    /// Set **FJOIN_RX** `[31:31]`
    ///
    /// When 1, RX FIFO steals the TX FIFO's storage, and becomes twice as deep.
    /// TX FIFO is disabled as a result (always reads as both full and empty).
    /// FIFOs are flushed when this bit is changed.
    #[inline(always)]
    pub const fn with_fjoin_rx(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sm3Shiftctrl::FJOIN_RX_MASK
            } else {
                self.bits & !Sm3Shiftctrl::FJOIN_RX_MASK
            }
        }
    }

    /// Set **FJOIN_TX** `[30:30]`
    ///
    /// When 1, TX FIFO steals the RX FIFO's storage, and becomes twice as deep.
    /// RX FIFO is disabled as a result (always reads as both full and empty).
    /// FIFOs are flushed when this bit is changed.
    #[inline(always)]
    pub const fn with_fjoin_tx(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sm3Shiftctrl::FJOIN_TX_MASK
            } else {
                self.bits & !Sm3Shiftctrl::FJOIN_TX_MASK
            }
        }
    }

    /// Set **PULL_THRESH** `[29:25]`
    ///
    /// Number of bits shifted out of OSR before autopull, or conditional pull (PULL IFEMPTY), will take place.
    /// Write 0 for value of 32.
    #[inline(always)]
    pub const fn with_pull_thresh(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sm3Shiftctrl::PULL_THRESH_MASK) | ((value << Sm3Shiftctrl::PULL_THRESH_OFFSET) & Sm3Shiftctrl::PULL_THRESH_MASK)
        }
    }

    /// Set **PUSH_THRESH** `[24:20]`
    ///
    /// Number of bits shifted into ISR before autopush, or conditional push (PUSH IFFULL), will take place.
    /// Write 0 for value of 32.
    #[inline(always)]
    pub const fn with_push_thresh(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sm3Shiftctrl::PUSH_THRESH_MASK) | ((value << Sm3Shiftctrl::PUSH_THRESH_OFFSET) & Sm3Shiftctrl::PUSH_THRESH_MASK)
        }
    }

    /// Set **OUT_SHIFTDIR** `[19:19]`
    ///
    /// 1 = shift out of output shift register to right. 0 = to left.
    #[inline(always)]
    pub const fn with_out_shiftdir(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sm3Shiftctrl::OUT_SHIFTDIR_MASK
            } else {
                self.bits & !Sm3Shiftctrl::OUT_SHIFTDIR_MASK
            }
        }
    }

    /// Set **IN_SHIFTDIR** `[18:18]`
    ///
    /// 1 = shift input shift register to right (data enters from left). 0 = to left.
    #[inline(always)]
    pub const fn with_in_shiftdir(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sm3Shiftctrl::IN_SHIFTDIR_MASK
            } else {
                self.bits & !Sm3Shiftctrl::IN_SHIFTDIR_MASK
            }
        }
    }

    /// Set **AUTOPULL** `[17:17]`
    ///
    /// Pull automatically when the output shift register is emptied, i.e. on or following an OUT instruction which causes the output shift counter to reach or exceed PULL_THRESH.
    #[inline(always)]
    pub const fn with_autopull(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sm3Shiftctrl::AUTOPULL_MASK
            } else {
                self.bits & !Sm3Shiftctrl::AUTOPULL_MASK
            }
        }
    }

    /// Set **AUTOPUSH** `[16:16]`
    ///
    /// Push automatically when the input shift register is filled, i.e. on an IN instruction which causes the input shift counter to reach or exceed PUSH_THRESH.
    #[inline(always)]
    pub const fn with_autopush(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sm3Shiftctrl::AUTOPUSH_MASK
            } else {
                self.bits & !Sm3Shiftctrl::AUTOPUSH_MASK
            }
        }
    }

    /// Set **FJOIN_RX_PUT** `[15:15]`
    ///
    /// If 1, disable this state machine's RX FIFO, make its storage available for random write access by the state machine (using the `put` instruction) and, unless FJOIN_RX_GET is also set, random read access by the processor (through the RXFx_PUTGETy registers).
    /// If FJOIN_RX_PUT and FJOIN_RX_GET are both set, then the RX FIFO's registers can be randomly read/written by the state machine, but are completely inaccessible to the processor.
    /// Setting this bit will clear the FJOIN_TX and FJOIN_RX bits.
    #[inline(always)]
    pub const fn with_fjoin_rx_put(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sm3Shiftctrl::FJOIN_RX_PUT_MASK
            } else {
                self.bits & !Sm3Shiftctrl::FJOIN_RX_PUT_MASK
            }
        }
    }

    /// Set **FJOIN_RX_GET** `[14:14]`
    ///
    /// If 1, disable this state machine's RX FIFO, make its storage available for random read access by the state machine (using the `get` instruction) and, unless FJOIN_RX_PUT is also set, random write access by the processor (through the RXFx_PUTGETy registers).
    /// If FJOIN_RX_PUT and FJOIN_RX_GET are both set, then the RX FIFO's registers can be randomly read/written by the state machine, but are completely inaccessible to the processor.
    /// Setting this bit will clear the FJOIN_TX and FJOIN_RX bits.
    #[inline(always)]
    pub const fn with_fjoin_rx_get(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sm3Shiftctrl::FJOIN_RX_GET_MASK
            } else {
                self.bits & !Sm3Shiftctrl::FJOIN_RX_GET_MASK
            }
        }
    }

    /// Set **IN_COUNT** `[4:0]`
    ///
    /// Set the number of pins which are not masked to 0 when read by an IN PINS, WAIT PIN or MOV x, PINS instruction.
    /// For example, an IN_COUNT of 5 means that the 5 LSBs of the IN pin group are visible (bits 4:0), but the remaining 27 MSBs are masked to 0. A count of 32 is encoded with a field value of 0, so the default behaviour is to not perform any masking.
    /// Note this masking is applied in addition to the masking usually performed by the IN instruction. This is mainly useful for the MOV x, PINS instruction, which otherwise has no way of masking pins.
    #[inline(always)]
    pub const fn with_in_count(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sm3Shiftctrl::IN_COUNT_MASK) | ((value << Sm3Shiftctrl::IN_COUNT_OFFSET) & Sm3Shiftctrl::IN_COUNT_MASK)
        }
    }

}

impl Default for Sm3ShiftctrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Sm3Shiftctrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Sm3ShiftctrlVal {
        Sm3ShiftctrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Sm3ShiftctrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Sm3ShiftctrlVal) -> Sm3ShiftctrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SM3_ADDR Register
///
/// Current instruction address of state machine 3
///
/// Address offset: 0x011c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SM3_ADDR** `[4:0]` (RO)
#[repr(transparent)]
pub struct Sm3Addr {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Sm3Addr {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sm3Addr {
    // ======== Field Constants ========

    /// **SM3_ADDR**
    /// Bits `[4:0]`, mask `0x0000001f`
    pub const SM3_ADDR_OFFSET: u32 = 0;
    pub const SM3_ADDR_MASK: u32 = 0x0000001f;

    // ======== Field Accessors ========

    /// **SM3_ADDR** `[4:0]` (RO)
    #[inline(always)]
    pub fn sm3_addr(&self) -> u32 {
        (self.value.read_bits() & Self::SM3_ADDR_MASK) >> Self::SM3_ADDR_OFFSET
    }

}

/// SM3_ADDR value type for batch read/modify/write operations
///
/// Use `sm3addr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sm3addr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Sm3AddrVal {
    bits: u32,
}

impl Sm3AddrVal {
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

    /// Read **SM3_ADDR** `[4:0]`
    #[inline(always)]
    pub const fn sm3_addr(&self) -> u32 {
        (self.bits & Sm3Addr::SM3_ADDR_MASK) >> Sm3Addr::SM3_ADDR_OFFSET
    }

}

impl Default for Sm3AddrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Sm3Addr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Sm3AddrVal {
        Sm3AddrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Sm3AddrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Sm3AddrVal) -> Sm3AddrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SM3_INSTR Register
///
/// Read to see the instruction currently addressed by state machine 3's program counter
/// Write to execute an instruction immediately (including jumps) and then resume execution.
///
/// Address offset: 0x0120
///
/// ## Fields
///
/// - **SM3_INSTR** `[15:0]` (RW)
#[repr(transparent)]
pub struct Sm3Instr {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Sm3Instr {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sm3Instr {
    // ======== Field Constants ========

    /// **SM3_INSTR**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const SM3_INSTR_OFFSET: u32 = 0;
    pub const SM3_INSTR_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **SM3_INSTR** `[15:0]` (RW)
    #[inline(always)]
    pub fn sm3_instr(&self) -> u32 {
        (self.value.read_bits() & Self::SM3_INSTR_MASK) >> Self::SM3_INSTR_OFFSET
    }

    /// Write **SM3_INSTR** `[15:0]` (RW)
    #[inline(always)]
    pub fn write_sm3_instr(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SM3_INSTR_MASK) | ((value << Self::SM3_INSTR_OFFSET) & Self::SM3_INSTR_MASK));
    }

}

/// SM3_INSTR value type for batch read/modify/write operations
///
/// Use `sm3instr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sm3instr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Sm3InstrVal {
    bits: u32,
}

impl Sm3InstrVal {
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

    /// Read **SM3_INSTR** `[15:0]`
    #[inline(always)]
    pub const fn sm3_instr(&self) -> u32 {
        (self.bits & Sm3Instr::SM3_INSTR_MASK) >> Sm3Instr::SM3_INSTR_OFFSET
    }

    /// Set **SM3_INSTR** `[15:0]`
    #[inline(always)]
    pub const fn with_sm3_instr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sm3Instr::SM3_INSTR_MASK) | ((value << Sm3Instr::SM3_INSTR_OFFSET) & Sm3Instr::SM3_INSTR_MASK)
        }
    }

}

impl Default for Sm3InstrVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Sm3Instr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Sm3InstrVal {
        Sm3InstrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Sm3InstrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Sm3InstrVal) -> Sm3InstrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SM3_PINCTRL Register
///
/// State machine pin control
///
/// Address offset: 0x0124
/// Reset value: 0x14000000
///
/// ## Fields
///
/// - **SIDESET_COUNT** `[31:29]` (RW)
///
///   The number of MSBs of the Delay/Side-set instruction field which are used for side-set. Inclusive of the enable bit, if present. Minimum of 0 (all delay bits, no side-set) and maximum of 5 (all side-set, no delay).
///
/// - **SET_COUNT** `[28:26]` (RW)
///
///   The number of pins asserted by a SET. In the range 0 to 5 inclusive.
///
/// - **OUT_COUNT** `[25:20]` (RW)
///
///   The number of pins asserted by an OUT PINS, OUT PINDIRS or MOV PINS instruction. In the range 0 to 32 inclusive.
///
/// - **IN_BASE** `[19:15]` (RW)
///
///   The pin which is mapped to the least-significant bit of a state machine's IN data bus. Higher-numbered pins are mapped to consecutively more-significant data bits, with a modulo of 32 applied to pin number.
///
/// - **SIDESET_BASE** `[14:10]` (RW)
///
///   The lowest-numbered pin that will be affected by a side-set operation. The MSBs of an instruction's side-set/delay field (up to 5, determined by SIDESET_COUNT) are used for side-set data, with the remaining LSBs used for delay. The least-significant bit of the side-set portion is the bit written to this pin, with more-significant bits written to higher-numbered pins.
///
/// - **SET_BASE** `[9:5]` (RW)
///
///   The lowest-numbered pin that will be affected by a SET PINS or SET PINDIRS instruction. The data written to this pin is the least-significant bit of the SET data.
///
/// - **OUT_BASE** `[4:0]` (RW)
///
///   The lowest-numbered pin that will be affected by an OUT PINS, OUT PINDIRS or MOV PINS instruction. The data written to this pin will always be the least-significant bit of the OUT or MOV data.
#[repr(transparent)]
pub struct Sm3Pinctrl {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Sm3Pinctrl {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sm3Pinctrl {
    // ======== Field Constants ========

    /// **SIDESET_COUNT**
    ///
    /// The number of MSBs of the Delay/Side-set instruction field which are used for side-set. Inclusive of the enable bit, if present. Minimum of 0 (all delay bits, no side-set) and maximum of 5 (all side-set, no delay).
    ///
    /// Bits `[31:29]`, mask `0xe0000000`
    pub const SIDESET_COUNT_OFFSET: u32 = 29;
    pub const SIDESET_COUNT_MASK: u32 = 0xe0000000;

    /// **SET_COUNT**
    ///
    /// The number of pins asserted by a SET. In the range 0 to 5 inclusive.
    ///
    /// Bits `[28:26]`, mask `0x1c000000`
    pub const SET_COUNT_OFFSET: u32 = 26;
    pub const SET_COUNT_MASK: u32 = 0x1c000000;

    /// **OUT_COUNT**
    ///
    /// The number of pins asserted by an OUT PINS, OUT PINDIRS or MOV PINS instruction. In the range 0 to 32 inclusive.
    ///
    /// Bits `[25:20]`, mask `0x03f00000`
    pub const OUT_COUNT_OFFSET: u32 = 20;
    pub const OUT_COUNT_MASK: u32 = 0x03f00000;

    /// **IN_BASE**
    ///
    /// The pin which is mapped to the least-significant bit of a state machine's IN data bus. Higher-numbered pins are mapped to consecutively more-significant data bits, with a modulo of 32 applied to pin number.
    ///
    /// Bits `[19:15]`, mask `0x000f8000`
    pub const IN_BASE_OFFSET: u32 = 15;
    pub const IN_BASE_MASK: u32 = 0x000f8000;

    /// **SIDESET_BASE**
    ///
    /// The lowest-numbered pin that will be affected by a side-set operation. The MSBs of an instruction's side-set/delay field (up to 5, determined by SIDESET_COUNT) are used for side-set data, with the remaining LSBs used for delay. The least-significant bit of the side-set portion is the bit written to this pin, with more-significant bits written to higher-numbered pins.
    ///
    /// Bits `[14:10]`, mask `0x00007c00`
    pub const SIDESET_BASE_OFFSET: u32 = 10;
    pub const SIDESET_BASE_MASK: u32 = 0x00007c00;

    /// **SET_BASE**
    ///
    /// The lowest-numbered pin that will be affected by a SET PINS or SET PINDIRS instruction. The data written to this pin is the least-significant bit of the SET data.
    ///
    /// Bits `[9:5]`, mask `0x000003e0`
    pub const SET_BASE_OFFSET: u32 = 5;
    pub const SET_BASE_MASK: u32 = 0x000003e0;

    /// **OUT_BASE**
    ///
    /// The lowest-numbered pin that will be affected by an OUT PINS, OUT PINDIRS or MOV PINS instruction. The data written to this pin will always be the least-significant bit of the OUT or MOV data.
    ///
    /// Bits `[4:0]`, mask `0x0000001f`
    pub const OUT_BASE_OFFSET: u32 = 0;
    pub const OUT_BASE_MASK: u32 = 0x0000001f;

    // ======== Field Accessors ========

    /// **SIDESET_COUNT** `[31:29]` (RW)
    ///
    /// The number of MSBs of the Delay/Side-set instruction field which are used for side-set. Inclusive of the enable bit, if present. Minimum of 0 (all delay bits, no side-set) and maximum of 5 (all side-set, no delay).
    #[inline(always)]
    pub fn sideset_count(&self) -> u32 {
        (self.value.read_bits() & Self::SIDESET_COUNT_MASK) >> Self::SIDESET_COUNT_OFFSET
    }

    /// Write **SIDESET_COUNT** `[31:29]` (RW)
    ///
    /// The number of MSBs of the Delay/Side-set instruction field which are used for side-set. Inclusive of the enable bit, if present. Minimum of 0 (all delay bits, no side-set) and maximum of 5 (all side-set, no delay).
    #[inline(always)]
    pub fn write_sideset_count(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SIDESET_COUNT_MASK) | ((value << Self::SIDESET_COUNT_OFFSET) & Self::SIDESET_COUNT_MASK));
    }

    /// **SET_COUNT** `[28:26]` (RW)
    ///
    /// The number of pins asserted by a SET. In the range 0 to 5 inclusive.
    #[inline(always)]
    pub fn set_count(&self) -> u32 {
        (self.value.read_bits() & Self::SET_COUNT_MASK) >> Self::SET_COUNT_OFFSET
    }

    /// Write **SET_COUNT** `[28:26]` (RW)
    ///
    /// The number of pins asserted by a SET. In the range 0 to 5 inclusive.
    #[inline(always)]
    pub fn write_set_count(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SET_COUNT_MASK) | ((value << Self::SET_COUNT_OFFSET) & Self::SET_COUNT_MASK));
    }

    /// **OUT_COUNT** `[25:20]` (RW)
    ///
    /// The number of pins asserted by an OUT PINS, OUT PINDIRS or MOV PINS instruction. In the range 0 to 32 inclusive.
    #[inline(always)]
    pub fn out_count(&self) -> u32 {
        (self.value.read_bits() & Self::OUT_COUNT_MASK) >> Self::OUT_COUNT_OFFSET
    }

    /// Write **OUT_COUNT** `[25:20]` (RW)
    ///
    /// The number of pins asserted by an OUT PINS, OUT PINDIRS or MOV PINS instruction. In the range 0 to 32 inclusive.
    #[inline(always)]
    pub fn write_out_count(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::OUT_COUNT_MASK) | ((value << Self::OUT_COUNT_OFFSET) & Self::OUT_COUNT_MASK));
    }

    /// **IN_BASE** `[19:15]` (RW)
    ///
    /// The pin which is mapped to the least-significant bit of a state machine's IN data bus. Higher-numbered pins are mapped to consecutively more-significant data bits, with a modulo of 32 applied to pin number.
    #[inline(always)]
    pub fn in_base(&self) -> u32 {
        (self.value.read_bits() & Self::IN_BASE_MASK) >> Self::IN_BASE_OFFSET
    }

    /// Write **IN_BASE** `[19:15]` (RW)
    ///
    /// The pin which is mapped to the least-significant bit of a state machine's IN data bus. Higher-numbered pins are mapped to consecutively more-significant data bits, with a modulo of 32 applied to pin number.
    #[inline(always)]
    pub fn write_in_base(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::IN_BASE_MASK) | ((value << Self::IN_BASE_OFFSET) & Self::IN_BASE_MASK));
    }

    /// **SIDESET_BASE** `[14:10]` (RW)
    ///
    /// The lowest-numbered pin that will be affected by a side-set operation. The MSBs of an instruction's side-set/delay field (up to 5, determined by SIDESET_COUNT) are used for side-set data, with the remaining LSBs used for delay. The least-significant bit of the side-set portion is the bit written to this pin, with more-significant bits written to higher-numbered pins.
    #[inline(always)]
    pub fn sideset_base(&self) -> u32 {
        (self.value.read_bits() & Self::SIDESET_BASE_MASK) >> Self::SIDESET_BASE_OFFSET
    }

    /// Write **SIDESET_BASE** `[14:10]` (RW)
    ///
    /// The lowest-numbered pin that will be affected by a side-set operation. The MSBs of an instruction's side-set/delay field (up to 5, determined by SIDESET_COUNT) are used for side-set data, with the remaining LSBs used for delay. The least-significant bit of the side-set portion is the bit written to this pin, with more-significant bits written to higher-numbered pins.
    #[inline(always)]
    pub fn write_sideset_base(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SIDESET_BASE_MASK) | ((value << Self::SIDESET_BASE_OFFSET) & Self::SIDESET_BASE_MASK));
    }

    /// **SET_BASE** `[9:5]` (RW)
    ///
    /// The lowest-numbered pin that will be affected by a SET PINS or SET PINDIRS instruction. The data written to this pin is the least-significant bit of the SET data.
    #[inline(always)]
    pub fn set_base(&self) -> u32 {
        (self.value.read_bits() & Self::SET_BASE_MASK) >> Self::SET_BASE_OFFSET
    }

    /// Write **SET_BASE** `[9:5]` (RW)
    ///
    /// The lowest-numbered pin that will be affected by a SET PINS or SET PINDIRS instruction. The data written to this pin is the least-significant bit of the SET data.
    #[inline(always)]
    pub fn write_set_base(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SET_BASE_MASK) | ((value << Self::SET_BASE_OFFSET) & Self::SET_BASE_MASK));
    }

    /// **OUT_BASE** `[4:0]` (RW)
    ///
    /// The lowest-numbered pin that will be affected by an OUT PINS, OUT PINDIRS or MOV PINS instruction. The data written to this pin will always be the least-significant bit of the OUT or MOV data.
    #[inline(always)]
    pub fn out_base(&self) -> u32 {
        (self.value.read_bits() & Self::OUT_BASE_MASK) >> Self::OUT_BASE_OFFSET
    }

    /// Write **OUT_BASE** `[4:0]` (RW)
    ///
    /// The lowest-numbered pin that will be affected by an OUT PINS, OUT PINDIRS or MOV PINS instruction. The data written to this pin will always be the least-significant bit of the OUT or MOV data.
    #[inline(always)]
    pub fn write_out_base(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::OUT_BASE_MASK) | ((value << Self::OUT_BASE_OFFSET) & Self::OUT_BASE_MASK));
    }

}

/// SM3_PINCTRL value type for batch read/modify/write operations
///
/// Use `sm3pinctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sm3pinctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Sm3PinctrlVal {
    bits: u32,
}

impl Sm3PinctrlVal {
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
        Self { bits: 0x14000000 }
    }

    /// Read **SIDESET_COUNT** `[31:29]`
    ///
    /// The number of MSBs of the Delay/Side-set instruction field which are used for side-set. Inclusive of the enable bit, if present. Minimum of 0 (all delay bits, no side-set) and maximum of 5 (all side-set, no delay).
    #[inline(always)]
    pub const fn sideset_count(&self) -> u32 {
        (self.bits & Sm3Pinctrl::SIDESET_COUNT_MASK) >> Sm3Pinctrl::SIDESET_COUNT_OFFSET
    }

    /// Read **SET_COUNT** `[28:26]`
    ///
    /// The number of pins asserted by a SET. In the range 0 to 5 inclusive.
    #[inline(always)]
    pub const fn set_count(&self) -> u32 {
        (self.bits & Sm3Pinctrl::SET_COUNT_MASK) >> Sm3Pinctrl::SET_COUNT_OFFSET
    }

    /// Read **OUT_COUNT** `[25:20]`
    ///
    /// The number of pins asserted by an OUT PINS, OUT PINDIRS or MOV PINS instruction. In the range 0 to 32 inclusive.
    #[inline(always)]
    pub const fn out_count(&self) -> u32 {
        (self.bits & Sm3Pinctrl::OUT_COUNT_MASK) >> Sm3Pinctrl::OUT_COUNT_OFFSET
    }

    /// Read **IN_BASE** `[19:15]`
    ///
    /// The pin which is mapped to the least-significant bit of a state machine's IN data bus. Higher-numbered pins are mapped to consecutively more-significant data bits, with a modulo of 32 applied to pin number.
    #[inline(always)]
    pub const fn in_base(&self) -> u32 {
        (self.bits & Sm3Pinctrl::IN_BASE_MASK) >> Sm3Pinctrl::IN_BASE_OFFSET
    }

    /// Read **SIDESET_BASE** `[14:10]`
    ///
    /// The lowest-numbered pin that will be affected by a side-set operation. The MSBs of an instruction's side-set/delay field (up to 5, determined by SIDESET_COUNT) are used for side-set data, with the remaining LSBs used for delay. The least-significant bit of the side-set portion is the bit written to this pin, with more-significant bits written to higher-numbered pins.
    #[inline(always)]
    pub const fn sideset_base(&self) -> u32 {
        (self.bits & Sm3Pinctrl::SIDESET_BASE_MASK) >> Sm3Pinctrl::SIDESET_BASE_OFFSET
    }

    /// Read **SET_BASE** `[9:5]`
    ///
    /// The lowest-numbered pin that will be affected by a SET PINS or SET PINDIRS instruction. The data written to this pin is the least-significant bit of the SET data.
    #[inline(always)]
    pub const fn set_base(&self) -> u32 {
        (self.bits & Sm3Pinctrl::SET_BASE_MASK) >> Sm3Pinctrl::SET_BASE_OFFSET
    }

    /// Read **OUT_BASE** `[4:0]`
    ///
    /// The lowest-numbered pin that will be affected by an OUT PINS, OUT PINDIRS or MOV PINS instruction. The data written to this pin will always be the least-significant bit of the OUT or MOV data.
    #[inline(always)]
    pub const fn out_base(&self) -> u32 {
        (self.bits & Sm3Pinctrl::OUT_BASE_MASK) >> Sm3Pinctrl::OUT_BASE_OFFSET
    }

    /// Set **SIDESET_COUNT** `[31:29]`
    ///
    /// The number of MSBs of the Delay/Side-set instruction field which are used for side-set. Inclusive of the enable bit, if present. Minimum of 0 (all delay bits, no side-set) and maximum of 5 (all side-set, no delay).
    #[inline(always)]
    pub const fn with_sideset_count(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sm3Pinctrl::SIDESET_COUNT_MASK) | ((value << Sm3Pinctrl::SIDESET_COUNT_OFFSET) & Sm3Pinctrl::SIDESET_COUNT_MASK)
        }
    }

    /// Set **SET_COUNT** `[28:26]`
    ///
    /// The number of pins asserted by a SET. In the range 0 to 5 inclusive.
    #[inline(always)]
    pub const fn with_set_count(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sm3Pinctrl::SET_COUNT_MASK) | ((value << Sm3Pinctrl::SET_COUNT_OFFSET) & Sm3Pinctrl::SET_COUNT_MASK)
        }
    }

    /// Set **OUT_COUNT** `[25:20]`
    ///
    /// The number of pins asserted by an OUT PINS, OUT PINDIRS or MOV PINS instruction. In the range 0 to 32 inclusive.
    #[inline(always)]
    pub const fn with_out_count(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sm3Pinctrl::OUT_COUNT_MASK) | ((value << Sm3Pinctrl::OUT_COUNT_OFFSET) & Sm3Pinctrl::OUT_COUNT_MASK)
        }
    }

    /// Set **IN_BASE** `[19:15]`
    ///
    /// The pin which is mapped to the least-significant bit of a state machine's IN data bus. Higher-numbered pins are mapped to consecutively more-significant data bits, with a modulo of 32 applied to pin number.
    #[inline(always)]
    pub const fn with_in_base(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sm3Pinctrl::IN_BASE_MASK) | ((value << Sm3Pinctrl::IN_BASE_OFFSET) & Sm3Pinctrl::IN_BASE_MASK)
        }
    }

    /// Set **SIDESET_BASE** `[14:10]`
    ///
    /// The lowest-numbered pin that will be affected by a side-set operation. The MSBs of an instruction's side-set/delay field (up to 5, determined by SIDESET_COUNT) are used for side-set data, with the remaining LSBs used for delay. The least-significant bit of the side-set portion is the bit written to this pin, with more-significant bits written to higher-numbered pins.
    #[inline(always)]
    pub const fn with_sideset_base(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sm3Pinctrl::SIDESET_BASE_MASK) | ((value << Sm3Pinctrl::SIDESET_BASE_OFFSET) & Sm3Pinctrl::SIDESET_BASE_MASK)
        }
    }

    /// Set **SET_BASE** `[9:5]`
    ///
    /// The lowest-numbered pin that will be affected by a SET PINS or SET PINDIRS instruction. The data written to this pin is the least-significant bit of the SET data.
    #[inline(always)]
    pub const fn with_set_base(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sm3Pinctrl::SET_BASE_MASK) | ((value << Sm3Pinctrl::SET_BASE_OFFSET) & Sm3Pinctrl::SET_BASE_MASK)
        }
    }

    /// Set **OUT_BASE** `[4:0]`
    ///
    /// The lowest-numbered pin that will be affected by an OUT PINS, OUT PINDIRS or MOV PINS instruction. The data written to this pin will always be the least-significant bit of the OUT or MOV data.
    #[inline(always)]
    pub const fn with_out_base(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sm3Pinctrl::OUT_BASE_MASK) | ((value << Sm3Pinctrl::OUT_BASE_OFFSET) & Sm3Pinctrl::OUT_BASE_MASK)
        }
    }

}

impl Default for Sm3PinctrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Sm3Pinctrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Sm3PinctrlVal {
        Sm3PinctrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Sm3PinctrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Sm3PinctrlVal) -> Sm3PinctrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// RXF0_PUTGET0 Register
///
/// Direct read/write access to entry 0 of SM0's RX FIFO, if SHIFTCTRL_FJOIN_RX_PUT xor SHIFTCTRL_FJOIN_RX_GET is set.
///
/// Address offset: 0x0128
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **RXF0_PUTGET0** `[31:0]` (RW)
#[repr(transparent)]
pub struct Rxf0Putget0 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Rxf0Putget0 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Rxf0Putget0 {
    // ======== Field Constants ========

    /// **RXF0_PUTGET0**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const RXF0_PUTGET0_OFFSET: u32 = 0;
    pub const RXF0_PUTGET0_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **RXF0_PUTGET0** `[31:0]` (RW)
    #[inline(always)]
    pub fn rxf0_putget0(&self) -> u32 {
        (self.value.read_bits() & Self::RXF0_PUTGET0_MASK) >> Self::RXF0_PUTGET0_OFFSET
    }

    /// Write **RXF0_PUTGET0** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_rxf0_putget0(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::RXF0_PUTGET0_MASK) | ((value << Self::RXF0_PUTGET0_OFFSET) & Self::RXF0_PUTGET0_MASK));
    }

}

/// RXF0_PUTGET0 value type for batch read/modify/write operations
///
/// Use `rxf0putget0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxf0putget0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Rxf0Putget0Val {
    bits: u32,
}

impl Rxf0Putget0Val {
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

    /// Read **RXF0_PUTGET0** `[31:0]`
    #[inline(always)]
    pub const fn rxf0_putget0(&self) -> u32 {
        (self.bits & Rxf0Putget0::RXF0_PUTGET0_MASK) >> Rxf0Putget0::RXF0_PUTGET0_OFFSET
    }

    /// Set **RXF0_PUTGET0** `[31:0]`
    #[inline(always)]
    pub const fn with_rxf0_putget0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Rxf0Putget0::RXF0_PUTGET0_MASK) | ((value << Rxf0Putget0::RXF0_PUTGET0_OFFSET) & Rxf0Putget0::RXF0_PUTGET0_MASK)
        }
    }

}

impl Default for Rxf0Putget0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Rxf0Putget0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Rxf0Putget0Val {
        Rxf0Putget0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Rxf0Putget0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Rxf0Putget0Val) -> Rxf0Putget0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// RXF0_PUTGET1 Register
///
/// Direct read/write access to entry 1 of SM0's RX FIFO, if SHIFTCTRL_FJOIN_RX_PUT xor SHIFTCTRL_FJOIN_RX_GET is set.
///
/// Address offset: 0x012c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **RXF0_PUTGET1** `[31:0]` (RW)
#[repr(transparent)]
pub struct Rxf0Putget1 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Rxf0Putget1 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Rxf0Putget1 {
    // ======== Field Constants ========

    /// **RXF0_PUTGET1**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const RXF0_PUTGET1_OFFSET: u32 = 0;
    pub const RXF0_PUTGET1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **RXF0_PUTGET1** `[31:0]` (RW)
    #[inline(always)]
    pub fn rxf0_putget1(&self) -> u32 {
        (self.value.read_bits() & Self::RXF0_PUTGET1_MASK) >> Self::RXF0_PUTGET1_OFFSET
    }

    /// Write **RXF0_PUTGET1** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_rxf0_putget1(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::RXF0_PUTGET1_MASK) | ((value << Self::RXF0_PUTGET1_OFFSET) & Self::RXF0_PUTGET1_MASK));
    }

}

/// RXF0_PUTGET1 value type for batch read/modify/write operations
///
/// Use `rxf0putget1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxf0putget1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Rxf0Putget1Val {
    bits: u32,
}

impl Rxf0Putget1Val {
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

    /// Read **RXF0_PUTGET1** `[31:0]`
    #[inline(always)]
    pub const fn rxf0_putget1(&self) -> u32 {
        (self.bits & Rxf0Putget1::RXF0_PUTGET1_MASK) >> Rxf0Putget1::RXF0_PUTGET1_OFFSET
    }

    /// Set **RXF0_PUTGET1** `[31:0]`
    #[inline(always)]
    pub const fn with_rxf0_putget1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Rxf0Putget1::RXF0_PUTGET1_MASK) | ((value << Rxf0Putget1::RXF0_PUTGET1_OFFSET) & Rxf0Putget1::RXF0_PUTGET1_MASK)
        }
    }

}

impl Default for Rxf0Putget1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Rxf0Putget1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Rxf0Putget1Val {
        Rxf0Putget1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Rxf0Putget1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Rxf0Putget1Val) -> Rxf0Putget1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// RXF0_PUTGET2 Register
///
/// Direct read/write access to entry 2 of SM0's RX FIFO, if SHIFTCTRL_FJOIN_RX_PUT xor SHIFTCTRL_FJOIN_RX_GET is set.
///
/// Address offset: 0x0130
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **RXF0_PUTGET2** `[31:0]` (RW)
#[repr(transparent)]
pub struct Rxf0Putget2 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Rxf0Putget2 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Rxf0Putget2 {
    // ======== Field Constants ========

    /// **RXF0_PUTGET2**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const RXF0_PUTGET2_OFFSET: u32 = 0;
    pub const RXF0_PUTGET2_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **RXF0_PUTGET2** `[31:0]` (RW)
    #[inline(always)]
    pub fn rxf0_putget2(&self) -> u32 {
        (self.value.read_bits() & Self::RXF0_PUTGET2_MASK) >> Self::RXF0_PUTGET2_OFFSET
    }

    /// Write **RXF0_PUTGET2** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_rxf0_putget2(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::RXF0_PUTGET2_MASK) | ((value << Self::RXF0_PUTGET2_OFFSET) & Self::RXF0_PUTGET2_MASK));
    }

}

/// RXF0_PUTGET2 value type for batch read/modify/write operations
///
/// Use `rxf0putget2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxf0putget2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Rxf0Putget2Val {
    bits: u32,
}

impl Rxf0Putget2Val {
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

    /// Read **RXF0_PUTGET2** `[31:0]`
    #[inline(always)]
    pub const fn rxf0_putget2(&self) -> u32 {
        (self.bits & Rxf0Putget2::RXF0_PUTGET2_MASK) >> Rxf0Putget2::RXF0_PUTGET2_OFFSET
    }

    /// Set **RXF0_PUTGET2** `[31:0]`
    #[inline(always)]
    pub const fn with_rxf0_putget2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Rxf0Putget2::RXF0_PUTGET2_MASK) | ((value << Rxf0Putget2::RXF0_PUTGET2_OFFSET) & Rxf0Putget2::RXF0_PUTGET2_MASK)
        }
    }

}

impl Default for Rxf0Putget2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Rxf0Putget2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Rxf0Putget2Val {
        Rxf0Putget2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Rxf0Putget2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Rxf0Putget2Val) -> Rxf0Putget2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// RXF0_PUTGET3 Register
///
/// Direct read/write access to entry 3 of SM0's RX FIFO, if SHIFTCTRL_FJOIN_RX_PUT xor SHIFTCTRL_FJOIN_RX_GET is set.
///
/// Address offset: 0x0134
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **RXF0_PUTGET3** `[31:0]` (RW)
#[repr(transparent)]
pub struct Rxf0Putget3 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Rxf0Putget3 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Rxf0Putget3 {
    // ======== Field Constants ========

    /// **RXF0_PUTGET3**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const RXF0_PUTGET3_OFFSET: u32 = 0;
    pub const RXF0_PUTGET3_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **RXF0_PUTGET3** `[31:0]` (RW)
    #[inline(always)]
    pub fn rxf0_putget3(&self) -> u32 {
        (self.value.read_bits() & Self::RXF0_PUTGET3_MASK) >> Self::RXF0_PUTGET3_OFFSET
    }

    /// Write **RXF0_PUTGET3** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_rxf0_putget3(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::RXF0_PUTGET3_MASK) | ((value << Self::RXF0_PUTGET3_OFFSET) & Self::RXF0_PUTGET3_MASK));
    }

}

/// RXF0_PUTGET3 value type for batch read/modify/write operations
///
/// Use `rxf0putget3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxf0putget3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Rxf0Putget3Val {
    bits: u32,
}

impl Rxf0Putget3Val {
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

    /// Read **RXF0_PUTGET3** `[31:0]`
    #[inline(always)]
    pub const fn rxf0_putget3(&self) -> u32 {
        (self.bits & Rxf0Putget3::RXF0_PUTGET3_MASK) >> Rxf0Putget3::RXF0_PUTGET3_OFFSET
    }

    /// Set **RXF0_PUTGET3** `[31:0]`
    #[inline(always)]
    pub const fn with_rxf0_putget3(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Rxf0Putget3::RXF0_PUTGET3_MASK) | ((value << Rxf0Putget3::RXF0_PUTGET3_OFFSET) & Rxf0Putget3::RXF0_PUTGET3_MASK)
        }
    }

}

impl Default for Rxf0Putget3Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Rxf0Putget3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Rxf0Putget3Val {
        Rxf0Putget3Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Rxf0Putget3Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Rxf0Putget3Val) -> Rxf0Putget3Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// RXF1_PUTGET0 Register
///
/// Direct read/write access to entry 0 of SM1's RX FIFO, if SHIFTCTRL_FJOIN_RX_PUT xor SHIFTCTRL_FJOIN_RX_GET is set.
///
/// Address offset: 0x0138
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **RXF1_PUTGET0** `[31:0]` (RW)
#[repr(transparent)]
pub struct Rxf1Putget0 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Rxf1Putget0 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Rxf1Putget0 {
    // ======== Field Constants ========

    /// **RXF1_PUTGET0**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const RXF1_PUTGET0_OFFSET: u32 = 0;
    pub const RXF1_PUTGET0_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **RXF1_PUTGET0** `[31:0]` (RW)
    #[inline(always)]
    pub fn rxf1_putget0(&self) -> u32 {
        (self.value.read_bits() & Self::RXF1_PUTGET0_MASK) >> Self::RXF1_PUTGET0_OFFSET
    }

    /// Write **RXF1_PUTGET0** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_rxf1_putget0(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::RXF1_PUTGET0_MASK) | ((value << Self::RXF1_PUTGET0_OFFSET) & Self::RXF1_PUTGET0_MASK));
    }

}

/// RXF1_PUTGET0 value type for batch read/modify/write operations
///
/// Use `rxf1putget0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxf1putget0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Rxf1Putget0Val {
    bits: u32,
}

impl Rxf1Putget0Val {
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

    /// Read **RXF1_PUTGET0** `[31:0]`
    #[inline(always)]
    pub const fn rxf1_putget0(&self) -> u32 {
        (self.bits & Rxf1Putget0::RXF1_PUTGET0_MASK) >> Rxf1Putget0::RXF1_PUTGET0_OFFSET
    }

    /// Set **RXF1_PUTGET0** `[31:0]`
    #[inline(always)]
    pub const fn with_rxf1_putget0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Rxf1Putget0::RXF1_PUTGET0_MASK) | ((value << Rxf1Putget0::RXF1_PUTGET0_OFFSET) & Rxf1Putget0::RXF1_PUTGET0_MASK)
        }
    }

}

impl Default for Rxf1Putget0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Rxf1Putget0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Rxf1Putget0Val {
        Rxf1Putget0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Rxf1Putget0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Rxf1Putget0Val) -> Rxf1Putget0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// RXF1_PUTGET1 Register
///
/// Direct read/write access to entry 1 of SM1's RX FIFO, if SHIFTCTRL_FJOIN_RX_PUT xor SHIFTCTRL_FJOIN_RX_GET is set.
///
/// Address offset: 0x013c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **RXF1_PUTGET1** `[31:0]` (RW)
#[repr(transparent)]
pub struct Rxf1Putget1 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Rxf1Putget1 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Rxf1Putget1 {
    // ======== Field Constants ========

    /// **RXF1_PUTGET1**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const RXF1_PUTGET1_OFFSET: u32 = 0;
    pub const RXF1_PUTGET1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **RXF1_PUTGET1** `[31:0]` (RW)
    #[inline(always)]
    pub fn rxf1_putget1(&self) -> u32 {
        (self.value.read_bits() & Self::RXF1_PUTGET1_MASK) >> Self::RXF1_PUTGET1_OFFSET
    }

    /// Write **RXF1_PUTGET1** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_rxf1_putget1(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::RXF1_PUTGET1_MASK) | ((value << Self::RXF1_PUTGET1_OFFSET) & Self::RXF1_PUTGET1_MASK));
    }

}

/// RXF1_PUTGET1 value type for batch read/modify/write operations
///
/// Use `rxf1putget1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxf1putget1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Rxf1Putget1Val {
    bits: u32,
}

impl Rxf1Putget1Val {
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

    /// Read **RXF1_PUTGET1** `[31:0]`
    #[inline(always)]
    pub const fn rxf1_putget1(&self) -> u32 {
        (self.bits & Rxf1Putget1::RXF1_PUTGET1_MASK) >> Rxf1Putget1::RXF1_PUTGET1_OFFSET
    }

    /// Set **RXF1_PUTGET1** `[31:0]`
    #[inline(always)]
    pub const fn with_rxf1_putget1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Rxf1Putget1::RXF1_PUTGET1_MASK) | ((value << Rxf1Putget1::RXF1_PUTGET1_OFFSET) & Rxf1Putget1::RXF1_PUTGET1_MASK)
        }
    }

}

impl Default for Rxf1Putget1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Rxf1Putget1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Rxf1Putget1Val {
        Rxf1Putget1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Rxf1Putget1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Rxf1Putget1Val) -> Rxf1Putget1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// RXF1_PUTGET2 Register
///
/// Direct read/write access to entry 2 of SM1's RX FIFO, if SHIFTCTRL_FJOIN_RX_PUT xor SHIFTCTRL_FJOIN_RX_GET is set.
///
/// Address offset: 0x0140
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **RXF1_PUTGET2** `[31:0]` (RW)
#[repr(transparent)]
pub struct Rxf1Putget2 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Rxf1Putget2 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Rxf1Putget2 {
    // ======== Field Constants ========

    /// **RXF1_PUTGET2**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const RXF1_PUTGET2_OFFSET: u32 = 0;
    pub const RXF1_PUTGET2_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **RXF1_PUTGET2** `[31:0]` (RW)
    #[inline(always)]
    pub fn rxf1_putget2(&self) -> u32 {
        (self.value.read_bits() & Self::RXF1_PUTGET2_MASK) >> Self::RXF1_PUTGET2_OFFSET
    }

    /// Write **RXF1_PUTGET2** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_rxf1_putget2(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::RXF1_PUTGET2_MASK) | ((value << Self::RXF1_PUTGET2_OFFSET) & Self::RXF1_PUTGET2_MASK));
    }

}

/// RXF1_PUTGET2 value type for batch read/modify/write operations
///
/// Use `rxf1putget2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxf1putget2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Rxf1Putget2Val {
    bits: u32,
}

impl Rxf1Putget2Val {
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

    /// Read **RXF1_PUTGET2** `[31:0]`
    #[inline(always)]
    pub const fn rxf1_putget2(&self) -> u32 {
        (self.bits & Rxf1Putget2::RXF1_PUTGET2_MASK) >> Rxf1Putget2::RXF1_PUTGET2_OFFSET
    }

    /// Set **RXF1_PUTGET2** `[31:0]`
    #[inline(always)]
    pub const fn with_rxf1_putget2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Rxf1Putget2::RXF1_PUTGET2_MASK) | ((value << Rxf1Putget2::RXF1_PUTGET2_OFFSET) & Rxf1Putget2::RXF1_PUTGET2_MASK)
        }
    }

}

impl Default for Rxf1Putget2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Rxf1Putget2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Rxf1Putget2Val {
        Rxf1Putget2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Rxf1Putget2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Rxf1Putget2Val) -> Rxf1Putget2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// RXF1_PUTGET3 Register
///
/// Direct read/write access to entry 3 of SM1's RX FIFO, if SHIFTCTRL_FJOIN_RX_PUT xor SHIFTCTRL_FJOIN_RX_GET is set.
///
/// Address offset: 0x0144
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **RXF1_PUTGET3** `[31:0]` (RW)
#[repr(transparent)]
pub struct Rxf1Putget3 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Rxf1Putget3 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Rxf1Putget3 {
    // ======== Field Constants ========

    /// **RXF1_PUTGET3**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const RXF1_PUTGET3_OFFSET: u32 = 0;
    pub const RXF1_PUTGET3_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **RXF1_PUTGET3** `[31:0]` (RW)
    #[inline(always)]
    pub fn rxf1_putget3(&self) -> u32 {
        (self.value.read_bits() & Self::RXF1_PUTGET3_MASK) >> Self::RXF1_PUTGET3_OFFSET
    }

    /// Write **RXF1_PUTGET3** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_rxf1_putget3(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::RXF1_PUTGET3_MASK) | ((value << Self::RXF1_PUTGET3_OFFSET) & Self::RXF1_PUTGET3_MASK));
    }

}

/// RXF1_PUTGET3 value type for batch read/modify/write operations
///
/// Use `rxf1putget3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxf1putget3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Rxf1Putget3Val {
    bits: u32,
}

impl Rxf1Putget3Val {
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

    /// Read **RXF1_PUTGET3** `[31:0]`
    #[inline(always)]
    pub const fn rxf1_putget3(&self) -> u32 {
        (self.bits & Rxf1Putget3::RXF1_PUTGET3_MASK) >> Rxf1Putget3::RXF1_PUTGET3_OFFSET
    }

    /// Set **RXF1_PUTGET3** `[31:0]`
    #[inline(always)]
    pub const fn with_rxf1_putget3(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Rxf1Putget3::RXF1_PUTGET3_MASK) | ((value << Rxf1Putget3::RXF1_PUTGET3_OFFSET) & Rxf1Putget3::RXF1_PUTGET3_MASK)
        }
    }

}

impl Default for Rxf1Putget3Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Rxf1Putget3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Rxf1Putget3Val {
        Rxf1Putget3Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Rxf1Putget3Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Rxf1Putget3Val) -> Rxf1Putget3Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// RXF2_PUTGET0 Register
///
/// Direct read/write access to entry 0 of SM2's RX FIFO, if SHIFTCTRL_FJOIN_RX_PUT xor SHIFTCTRL_FJOIN_RX_GET is set.
///
/// Address offset: 0x0148
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **RXF2_PUTGET0** `[31:0]` (RW)
#[repr(transparent)]
pub struct Rxf2Putget0 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Rxf2Putget0 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Rxf2Putget0 {
    // ======== Field Constants ========

    /// **RXF2_PUTGET0**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const RXF2_PUTGET0_OFFSET: u32 = 0;
    pub const RXF2_PUTGET0_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **RXF2_PUTGET0** `[31:0]` (RW)
    #[inline(always)]
    pub fn rxf2_putget0(&self) -> u32 {
        (self.value.read_bits() & Self::RXF2_PUTGET0_MASK) >> Self::RXF2_PUTGET0_OFFSET
    }

    /// Write **RXF2_PUTGET0** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_rxf2_putget0(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::RXF2_PUTGET0_MASK) | ((value << Self::RXF2_PUTGET0_OFFSET) & Self::RXF2_PUTGET0_MASK));
    }

}

/// RXF2_PUTGET0 value type for batch read/modify/write operations
///
/// Use `rxf2putget0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxf2putget0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Rxf2Putget0Val {
    bits: u32,
}

impl Rxf2Putget0Val {
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

    /// Read **RXF2_PUTGET0** `[31:0]`
    #[inline(always)]
    pub const fn rxf2_putget0(&self) -> u32 {
        (self.bits & Rxf2Putget0::RXF2_PUTGET0_MASK) >> Rxf2Putget0::RXF2_PUTGET0_OFFSET
    }

    /// Set **RXF2_PUTGET0** `[31:0]`
    #[inline(always)]
    pub const fn with_rxf2_putget0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Rxf2Putget0::RXF2_PUTGET0_MASK) | ((value << Rxf2Putget0::RXF2_PUTGET0_OFFSET) & Rxf2Putget0::RXF2_PUTGET0_MASK)
        }
    }

}

impl Default for Rxf2Putget0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Rxf2Putget0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Rxf2Putget0Val {
        Rxf2Putget0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Rxf2Putget0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Rxf2Putget0Val) -> Rxf2Putget0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// RXF2_PUTGET1 Register
///
/// Direct read/write access to entry 1 of SM2's RX FIFO, if SHIFTCTRL_FJOIN_RX_PUT xor SHIFTCTRL_FJOIN_RX_GET is set.
///
/// Address offset: 0x014c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **RXF2_PUTGET1** `[31:0]` (RW)
#[repr(transparent)]
pub struct Rxf2Putget1 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Rxf2Putget1 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Rxf2Putget1 {
    // ======== Field Constants ========

    /// **RXF2_PUTGET1**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const RXF2_PUTGET1_OFFSET: u32 = 0;
    pub const RXF2_PUTGET1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **RXF2_PUTGET1** `[31:0]` (RW)
    #[inline(always)]
    pub fn rxf2_putget1(&self) -> u32 {
        (self.value.read_bits() & Self::RXF2_PUTGET1_MASK) >> Self::RXF2_PUTGET1_OFFSET
    }

    /// Write **RXF2_PUTGET1** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_rxf2_putget1(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::RXF2_PUTGET1_MASK) | ((value << Self::RXF2_PUTGET1_OFFSET) & Self::RXF2_PUTGET1_MASK));
    }

}

/// RXF2_PUTGET1 value type for batch read/modify/write operations
///
/// Use `rxf2putget1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxf2putget1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Rxf2Putget1Val {
    bits: u32,
}

impl Rxf2Putget1Val {
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

    /// Read **RXF2_PUTGET1** `[31:0]`
    #[inline(always)]
    pub const fn rxf2_putget1(&self) -> u32 {
        (self.bits & Rxf2Putget1::RXF2_PUTGET1_MASK) >> Rxf2Putget1::RXF2_PUTGET1_OFFSET
    }

    /// Set **RXF2_PUTGET1** `[31:0]`
    #[inline(always)]
    pub const fn with_rxf2_putget1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Rxf2Putget1::RXF2_PUTGET1_MASK) | ((value << Rxf2Putget1::RXF2_PUTGET1_OFFSET) & Rxf2Putget1::RXF2_PUTGET1_MASK)
        }
    }

}

impl Default for Rxf2Putget1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Rxf2Putget1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Rxf2Putget1Val {
        Rxf2Putget1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Rxf2Putget1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Rxf2Putget1Val) -> Rxf2Putget1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// RXF2_PUTGET2 Register
///
/// Direct read/write access to entry 2 of SM2's RX FIFO, if SHIFTCTRL_FJOIN_RX_PUT xor SHIFTCTRL_FJOIN_RX_GET is set.
///
/// Address offset: 0x0150
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **RXF2_PUTGET2** `[31:0]` (RW)
#[repr(transparent)]
pub struct Rxf2Putget2 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Rxf2Putget2 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Rxf2Putget2 {
    // ======== Field Constants ========

    /// **RXF2_PUTGET2**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const RXF2_PUTGET2_OFFSET: u32 = 0;
    pub const RXF2_PUTGET2_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **RXF2_PUTGET2** `[31:0]` (RW)
    #[inline(always)]
    pub fn rxf2_putget2(&self) -> u32 {
        (self.value.read_bits() & Self::RXF2_PUTGET2_MASK) >> Self::RXF2_PUTGET2_OFFSET
    }

    /// Write **RXF2_PUTGET2** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_rxf2_putget2(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::RXF2_PUTGET2_MASK) | ((value << Self::RXF2_PUTGET2_OFFSET) & Self::RXF2_PUTGET2_MASK));
    }

}

/// RXF2_PUTGET2 value type for batch read/modify/write operations
///
/// Use `rxf2putget2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxf2putget2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Rxf2Putget2Val {
    bits: u32,
}

impl Rxf2Putget2Val {
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

    /// Read **RXF2_PUTGET2** `[31:0]`
    #[inline(always)]
    pub const fn rxf2_putget2(&self) -> u32 {
        (self.bits & Rxf2Putget2::RXF2_PUTGET2_MASK) >> Rxf2Putget2::RXF2_PUTGET2_OFFSET
    }

    /// Set **RXF2_PUTGET2** `[31:0]`
    #[inline(always)]
    pub const fn with_rxf2_putget2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Rxf2Putget2::RXF2_PUTGET2_MASK) | ((value << Rxf2Putget2::RXF2_PUTGET2_OFFSET) & Rxf2Putget2::RXF2_PUTGET2_MASK)
        }
    }

}

impl Default for Rxf2Putget2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Rxf2Putget2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Rxf2Putget2Val {
        Rxf2Putget2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Rxf2Putget2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Rxf2Putget2Val) -> Rxf2Putget2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// RXF2_PUTGET3 Register
///
/// Direct read/write access to entry 3 of SM2's RX FIFO, if SHIFTCTRL_FJOIN_RX_PUT xor SHIFTCTRL_FJOIN_RX_GET is set.
///
/// Address offset: 0x0154
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **RXF2_PUTGET3** `[31:0]` (RW)
#[repr(transparent)]
pub struct Rxf2Putget3 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Rxf2Putget3 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Rxf2Putget3 {
    // ======== Field Constants ========

    /// **RXF2_PUTGET3**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const RXF2_PUTGET3_OFFSET: u32 = 0;
    pub const RXF2_PUTGET3_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **RXF2_PUTGET3** `[31:0]` (RW)
    #[inline(always)]
    pub fn rxf2_putget3(&self) -> u32 {
        (self.value.read_bits() & Self::RXF2_PUTGET3_MASK) >> Self::RXF2_PUTGET3_OFFSET
    }

    /// Write **RXF2_PUTGET3** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_rxf2_putget3(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::RXF2_PUTGET3_MASK) | ((value << Self::RXF2_PUTGET3_OFFSET) & Self::RXF2_PUTGET3_MASK));
    }

}

/// RXF2_PUTGET3 value type for batch read/modify/write operations
///
/// Use `rxf2putget3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxf2putget3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Rxf2Putget3Val {
    bits: u32,
}

impl Rxf2Putget3Val {
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

    /// Read **RXF2_PUTGET3** `[31:0]`
    #[inline(always)]
    pub const fn rxf2_putget3(&self) -> u32 {
        (self.bits & Rxf2Putget3::RXF2_PUTGET3_MASK) >> Rxf2Putget3::RXF2_PUTGET3_OFFSET
    }

    /// Set **RXF2_PUTGET3** `[31:0]`
    #[inline(always)]
    pub const fn with_rxf2_putget3(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Rxf2Putget3::RXF2_PUTGET3_MASK) | ((value << Rxf2Putget3::RXF2_PUTGET3_OFFSET) & Rxf2Putget3::RXF2_PUTGET3_MASK)
        }
    }

}

impl Default for Rxf2Putget3Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Rxf2Putget3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Rxf2Putget3Val {
        Rxf2Putget3Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Rxf2Putget3Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Rxf2Putget3Val) -> Rxf2Putget3Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// RXF3_PUTGET0 Register
///
/// Direct read/write access to entry 0 of SM3's RX FIFO, if SHIFTCTRL_FJOIN_RX_PUT xor SHIFTCTRL_FJOIN_RX_GET is set.
///
/// Address offset: 0x0158
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **RXF3_PUTGET0** `[31:0]` (RW)
#[repr(transparent)]
pub struct Rxf3Putget0 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Rxf3Putget0 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Rxf3Putget0 {
    // ======== Field Constants ========

    /// **RXF3_PUTGET0**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const RXF3_PUTGET0_OFFSET: u32 = 0;
    pub const RXF3_PUTGET0_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **RXF3_PUTGET0** `[31:0]` (RW)
    #[inline(always)]
    pub fn rxf3_putget0(&self) -> u32 {
        (self.value.read_bits() & Self::RXF3_PUTGET0_MASK) >> Self::RXF3_PUTGET0_OFFSET
    }

    /// Write **RXF3_PUTGET0** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_rxf3_putget0(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::RXF3_PUTGET0_MASK) | ((value << Self::RXF3_PUTGET0_OFFSET) & Self::RXF3_PUTGET0_MASK));
    }

}

/// RXF3_PUTGET0 value type for batch read/modify/write operations
///
/// Use `rxf3putget0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxf3putget0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Rxf3Putget0Val {
    bits: u32,
}

impl Rxf3Putget0Val {
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

    /// Read **RXF3_PUTGET0** `[31:0]`
    #[inline(always)]
    pub const fn rxf3_putget0(&self) -> u32 {
        (self.bits & Rxf3Putget0::RXF3_PUTGET0_MASK) >> Rxf3Putget0::RXF3_PUTGET0_OFFSET
    }

    /// Set **RXF3_PUTGET0** `[31:0]`
    #[inline(always)]
    pub const fn with_rxf3_putget0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Rxf3Putget0::RXF3_PUTGET0_MASK) | ((value << Rxf3Putget0::RXF3_PUTGET0_OFFSET) & Rxf3Putget0::RXF3_PUTGET0_MASK)
        }
    }

}

impl Default for Rxf3Putget0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Rxf3Putget0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Rxf3Putget0Val {
        Rxf3Putget0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Rxf3Putget0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Rxf3Putget0Val) -> Rxf3Putget0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// RXF3_PUTGET1 Register
///
/// Direct read/write access to entry 1 of SM3's RX FIFO, if SHIFTCTRL_FJOIN_RX_PUT xor SHIFTCTRL_FJOIN_RX_GET is set.
///
/// Address offset: 0x015c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **RXF3_PUTGET1** `[31:0]` (RW)
#[repr(transparent)]
pub struct Rxf3Putget1 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Rxf3Putget1 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Rxf3Putget1 {
    // ======== Field Constants ========

    /// **RXF3_PUTGET1**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const RXF3_PUTGET1_OFFSET: u32 = 0;
    pub const RXF3_PUTGET1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **RXF3_PUTGET1** `[31:0]` (RW)
    #[inline(always)]
    pub fn rxf3_putget1(&self) -> u32 {
        (self.value.read_bits() & Self::RXF3_PUTGET1_MASK) >> Self::RXF3_PUTGET1_OFFSET
    }

    /// Write **RXF3_PUTGET1** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_rxf3_putget1(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::RXF3_PUTGET1_MASK) | ((value << Self::RXF3_PUTGET1_OFFSET) & Self::RXF3_PUTGET1_MASK));
    }

}

/// RXF3_PUTGET1 value type for batch read/modify/write operations
///
/// Use `rxf3putget1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxf3putget1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Rxf3Putget1Val {
    bits: u32,
}

impl Rxf3Putget1Val {
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

    /// Read **RXF3_PUTGET1** `[31:0]`
    #[inline(always)]
    pub const fn rxf3_putget1(&self) -> u32 {
        (self.bits & Rxf3Putget1::RXF3_PUTGET1_MASK) >> Rxf3Putget1::RXF3_PUTGET1_OFFSET
    }

    /// Set **RXF3_PUTGET1** `[31:0]`
    #[inline(always)]
    pub const fn with_rxf3_putget1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Rxf3Putget1::RXF3_PUTGET1_MASK) | ((value << Rxf3Putget1::RXF3_PUTGET1_OFFSET) & Rxf3Putget1::RXF3_PUTGET1_MASK)
        }
    }

}

impl Default for Rxf3Putget1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Rxf3Putget1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Rxf3Putget1Val {
        Rxf3Putget1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Rxf3Putget1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Rxf3Putget1Val) -> Rxf3Putget1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// RXF3_PUTGET2 Register
///
/// Direct read/write access to entry 2 of SM3's RX FIFO, if SHIFTCTRL_FJOIN_RX_PUT xor SHIFTCTRL_FJOIN_RX_GET is set.
///
/// Address offset: 0x0160
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **RXF3_PUTGET2** `[31:0]` (RW)
#[repr(transparent)]
pub struct Rxf3Putget2 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Rxf3Putget2 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Rxf3Putget2 {
    // ======== Field Constants ========

    /// **RXF3_PUTGET2**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const RXF3_PUTGET2_OFFSET: u32 = 0;
    pub const RXF3_PUTGET2_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **RXF3_PUTGET2** `[31:0]` (RW)
    #[inline(always)]
    pub fn rxf3_putget2(&self) -> u32 {
        (self.value.read_bits() & Self::RXF3_PUTGET2_MASK) >> Self::RXF3_PUTGET2_OFFSET
    }

    /// Write **RXF3_PUTGET2** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_rxf3_putget2(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::RXF3_PUTGET2_MASK) | ((value << Self::RXF3_PUTGET2_OFFSET) & Self::RXF3_PUTGET2_MASK));
    }

}

/// RXF3_PUTGET2 value type for batch read/modify/write operations
///
/// Use `rxf3putget2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxf3putget2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Rxf3Putget2Val {
    bits: u32,
}

impl Rxf3Putget2Val {
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

    /// Read **RXF3_PUTGET2** `[31:0]`
    #[inline(always)]
    pub const fn rxf3_putget2(&self) -> u32 {
        (self.bits & Rxf3Putget2::RXF3_PUTGET2_MASK) >> Rxf3Putget2::RXF3_PUTGET2_OFFSET
    }

    /// Set **RXF3_PUTGET2** `[31:0]`
    #[inline(always)]
    pub const fn with_rxf3_putget2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Rxf3Putget2::RXF3_PUTGET2_MASK) | ((value << Rxf3Putget2::RXF3_PUTGET2_OFFSET) & Rxf3Putget2::RXF3_PUTGET2_MASK)
        }
    }

}

impl Default for Rxf3Putget2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Rxf3Putget2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Rxf3Putget2Val {
        Rxf3Putget2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Rxf3Putget2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Rxf3Putget2Val) -> Rxf3Putget2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// RXF3_PUTGET3 Register
///
/// Direct read/write access to entry 3 of SM3's RX FIFO, if SHIFTCTRL_FJOIN_RX_PUT xor SHIFTCTRL_FJOIN_RX_GET is set.
///
/// Address offset: 0x0164
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **RXF3_PUTGET3** `[31:0]` (RW)
#[repr(transparent)]
pub struct Rxf3Putget3 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Rxf3Putget3 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Rxf3Putget3 {
    // ======== Field Constants ========

    /// **RXF3_PUTGET3**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const RXF3_PUTGET3_OFFSET: u32 = 0;
    pub const RXF3_PUTGET3_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **RXF3_PUTGET3** `[31:0]` (RW)
    #[inline(always)]
    pub fn rxf3_putget3(&self) -> u32 {
        (self.value.read_bits() & Self::RXF3_PUTGET3_MASK) >> Self::RXF3_PUTGET3_OFFSET
    }

    /// Write **RXF3_PUTGET3** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_rxf3_putget3(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::RXF3_PUTGET3_MASK) | ((value << Self::RXF3_PUTGET3_OFFSET) & Self::RXF3_PUTGET3_MASK));
    }

}

/// RXF3_PUTGET3 value type for batch read/modify/write operations
///
/// Use `rxf3putget3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rxf3putget3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Rxf3Putget3Val {
    bits: u32,
}

impl Rxf3Putget3Val {
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

    /// Read **RXF3_PUTGET3** `[31:0]`
    #[inline(always)]
    pub const fn rxf3_putget3(&self) -> u32 {
        (self.bits & Rxf3Putget3::RXF3_PUTGET3_MASK) >> Rxf3Putget3::RXF3_PUTGET3_OFFSET
    }

    /// Set **RXF3_PUTGET3** `[31:0]`
    #[inline(always)]
    pub const fn with_rxf3_putget3(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Rxf3Putget3::RXF3_PUTGET3_MASK) | ((value << Rxf3Putget3::RXF3_PUTGET3_OFFSET) & Rxf3Putget3::RXF3_PUTGET3_MASK)
        }
    }

}

impl Default for Rxf3Putget3Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Rxf3Putget3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Rxf3Putget3Val {
        Rxf3Putget3Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Rxf3Putget3Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Rxf3Putget3Val) -> Rxf3Putget3Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// GPIOBASE Register
///
/// Relocate GPIO 0 (from PIO's point of view) in the system GPIO numbering, to access more than 32 GPIOs from PIO.
/// Only the values 0 and 16 are supported (only bit 4 is writable).
///
/// Address offset: 0x0168
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **GPIOBASE** `[4]` (RW)
#[repr(transparent)]
pub struct Gpiobase {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Gpiobase {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Gpiobase {
    // ======== Field Constants ========

    /// **GPIOBASE**
    /// Bits `[4:4]`, mask `0x00000010`
    pub const GPIOBASE_OFFSET: u32 = 4;
    pub const GPIOBASE_MASK: u32 = 0x00000010;
    pub const GPIOBASE: u32 = 1 << 4;

    // ======== Field Accessors ========

    /// **GPIOBASE** `[4]` (RW)
    #[inline(always)]
    pub fn gpiobase(&self) -> bool {
        (self.value.read_bits() & Self::GPIOBASE_MASK) != 0
    }

    /// Set GPIOBASE bit atomically (RW)
    #[inline(always)]
    pub fn set_gpiobase(&self) {
        self.set_bits(Self::GPIOBASE_MASK);
    }

    /// Clear GPIOBASE bit atomically (RW)
    #[inline(always)]
    pub fn clr_gpiobase(&self) {
        self.clr_bits(Self::GPIOBASE_MASK);
    }

}

/// GPIOBASE value type for batch read/modify/write operations
///
/// Use `gpiobase.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpiobase.write_val(val)`.
#[derive(Clone, Copy)]
pub struct GpiobaseVal {
    bits: u32,
}

impl GpiobaseVal {
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

    /// Read **GPIOBASE** `[4:4]`
    #[inline(always)]
    pub const fn gpiobase(&self) -> bool {
        (self.bits & Gpiobase::GPIOBASE_MASK) != 0
    }

    /// Set **GPIOBASE** `[4:4]`
    #[inline(always)]
    pub const fn with_gpiobase(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Gpiobase::GPIOBASE_MASK
            } else {
                self.bits & !Gpiobase::GPIOBASE_MASK
            }
        }
    }

}

impl Default for GpiobaseVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Gpiobase {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> GpiobaseVal {
        GpiobaseVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: GpiobaseVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(GpiobaseVal) -> GpiobaseVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// INTR Register
///
/// Raw Interrupts
///
/// Address offset: 0x016c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SM7** `[15]` (RO)
///
/// - **SM6** `[14]` (RO)
///
/// - **SM5** `[13]` (RO)
///
/// - **SM4** `[12]` (RO)
///
/// - **SM3** `[11]` (RO)
///
/// - **SM2** `[10]` (RO)
///
/// - **SM1** `[9]` (RO)
///
/// - **SM0** `[8]` (RO)
///
/// - **SM3_TXNFULL** `[7]` (RO)
///
/// - **SM2_TXNFULL** `[6]` (RO)
///
/// - **SM1_TXNFULL** `[5]` (RO)
///
/// - **SM0_TXNFULL** `[4]` (RO)
///
/// - **SM3_RXNEMPTY** `[3]` (RO)
///
/// - **SM2_RXNEMPTY** `[2]` (RO)
///
/// - **SM1_RXNEMPTY** `[1]` (RO)
///
/// - **SM0_RXNEMPTY** `[0]` (RO)
#[repr(transparent)]
pub struct Intr {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Intr {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Intr {
    // ======== Field Constants ========

    /// **SM7**
    /// Bits `[15:15]`, mask `0x00008000`
    pub const SM7_OFFSET: u32 = 15;
    pub const SM7_MASK: u32 = 0x00008000;
    pub const SM7: u32 = 1 << 15;

    /// **SM6**
    /// Bits `[14:14]`, mask `0x00004000`
    pub const SM6_OFFSET: u32 = 14;
    pub const SM6_MASK: u32 = 0x00004000;
    pub const SM6: u32 = 1 << 14;

    /// **SM5**
    /// Bits `[13:13]`, mask `0x00002000`
    pub const SM5_OFFSET: u32 = 13;
    pub const SM5_MASK: u32 = 0x00002000;
    pub const SM5: u32 = 1 << 13;

    /// **SM4**
    /// Bits `[12:12]`, mask `0x00001000`
    pub const SM4_OFFSET: u32 = 12;
    pub const SM4_MASK: u32 = 0x00001000;
    pub const SM4: u32 = 1 << 12;

    /// **SM3**
    /// Bits `[11:11]`, mask `0x00000800`
    pub const SM3_OFFSET: u32 = 11;
    pub const SM3_MASK: u32 = 0x00000800;
    pub const SM3: u32 = 1 << 11;

    /// **SM2**
    /// Bits `[10:10]`, mask `0x00000400`
    pub const SM2_OFFSET: u32 = 10;
    pub const SM2_MASK: u32 = 0x00000400;
    pub const SM2: u32 = 1 << 10;

    /// **SM1**
    /// Bits `[9:9]`, mask `0x00000200`
    pub const SM1_OFFSET: u32 = 9;
    pub const SM1_MASK: u32 = 0x00000200;
    pub const SM1: u32 = 1 << 9;

    /// **SM0**
    /// Bits `[8:8]`, mask `0x00000100`
    pub const SM0_OFFSET: u32 = 8;
    pub const SM0_MASK: u32 = 0x00000100;
    pub const SM0: u32 = 1 << 8;

    /// **SM3_TXNFULL**
    /// Bits `[7:7]`, mask `0x00000080`
    pub const SM3_TXNFULL_OFFSET: u32 = 7;
    pub const SM3_TXNFULL_MASK: u32 = 0x00000080;
    pub const SM3_TXNFULL: u32 = 1 << 7;

    /// **SM2_TXNFULL**
    /// Bits `[6:6]`, mask `0x00000040`
    pub const SM2_TXNFULL_OFFSET: u32 = 6;
    pub const SM2_TXNFULL_MASK: u32 = 0x00000040;
    pub const SM2_TXNFULL: u32 = 1 << 6;

    /// **SM1_TXNFULL**
    /// Bits `[5:5]`, mask `0x00000020`
    pub const SM1_TXNFULL_OFFSET: u32 = 5;
    pub const SM1_TXNFULL_MASK: u32 = 0x00000020;
    pub const SM1_TXNFULL: u32 = 1 << 5;

    /// **SM0_TXNFULL**
    /// Bits `[4:4]`, mask `0x00000010`
    pub const SM0_TXNFULL_OFFSET: u32 = 4;
    pub const SM0_TXNFULL_MASK: u32 = 0x00000010;
    pub const SM0_TXNFULL: u32 = 1 << 4;

    /// **SM3_RXNEMPTY**
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SM3_RXNEMPTY_OFFSET: u32 = 3;
    pub const SM3_RXNEMPTY_MASK: u32 = 0x00000008;
    pub const SM3_RXNEMPTY: u32 = 1 << 3;

    /// **SM2_RXNEMPTY**
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SM2_RXNEMPTY_OFFSET: u32 = 2;
    pub const SM2_RXNEMPTY_MASK: u32 = 0x00000004;
    pub const SM2_RXNEMPTY: u32 = 1 << 2;

    /// **SM1_RXNEMPTY**
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SM1_RXNEMPTY_OFFSET: u32 = 1;
    pub const SM1_RXNEMPTY_MASK: u32 = 0x00000002;
    pub const SM1_RXNEMPTY: u32 = 1 << 1;

    /// **SM0_RXNEMPTY**
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SM0_RXNEMPTY_OFFSET: u32 = 0;
    pub const SM0_RXNEMPTY_MASK: u32 = 0x00000001;
    pub const SM0_RXNEMPTY: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **SM7** `[15]` (RO)
    #[inline(always)]
    pub fn sm7(&self) -> bool {
        (self.value.read_bits() & Self::SM7_MASK) != 0
    }

    /// **SM6** `[14]` (RO)
    #[inline(always)]
    pub fn sm6(&self) -> bool {
        (self.value.read_bits() & Self::SM6_MASK) != 0
    }

    /// **SM5** `[13]` (RO)
    #[inline(always)]
    pub fn sm5(&self) -> bool {
        (self.value.read_bits() & Self::SM5_MASK) != 0
    }

    /// **SM4** `[12]` (RO)
    #[inline(always)]
    pub fn sm4(&self) -> bool {
        (self.value.read_bits() & Self::SM4_MASK) != 0
    }

    /// **SM3** `[11]` (RO)
    #[inline(always)]
    pub fn sm3(&self) -> bool {
        (self.value.read_bits() & Self::SM3_MASK) != 0
    }

    /// **SM2** `[10]` (RO)
    #[inline(always)]
    pub fn sm2(&self) -> bool {
        (self.value.read_bits() & Self::SM2_MASK) != 0
    }

    /// **SM1** `[9]` (RO)
    #[inline(always)]
    pub fn sm1(&self) -> bool {
        (self.value.read_bits() & Self::SM1_MASK) != 0
    }

    /// **SM0** `[8]` (RO)
    #[inline(always)]
    pub fn sm0(&self) -> bool {
        (self.value.read_bits() & Self::SM0_MASK) != 0
    }

    /// **SM3_TXNFULL** `[7]` (RO)
    #[inline(always)]
    pub fn sm3_txnfull(&self) -> bool {
        (self.value.read_bits() & Self::SM3_TXNFULL_MASK) != 0
    }

    /// **SM2_TXNFULL** `[6]` (RO)
    #[inline(always)]
    pub fn sm2_txnfull(&self) -> bool {
        (self.value.read_bits() & Self::SM2_TXNFULL_MASK) != 0
    }

    /// **SM1_TXNFULL** `[5]` (RO)
    #[inline(always)]
    pub fn sm1_txnfull(&self) -> bool {
        (self.value.read_bits() & Self::SM1_TXNFULL_MASK) != 0
    }

    /// **SM0_TXNFULL** `[4]` (RO)
    #[inline(always)]
    pub fn sm0_txnfull(&self) -> bool {
        (self.value.read_bits() & Self::SM0_TXNFULL_MASK) != 0
    }

    /// **SM3_RXNEMPTY** `[3]` (RO)
    #[inline(always)]
    pub fn sm3_rxnempty(&self) -> bool {
        (self.value.read_bits() & Self::SM3_RXNEMPTY_MASK) != 0
    }

    /// **SM2_RXNEMPTY** `[2]` (RO)
    #[inline(always)]
    pub fn sm2_rxnempty(&self) -> bool {
        (self.value.read_bits() & Self::SM2_RXNEMPTY_MASK) != 0
    }

    /// **SM1_RXNEMPTY** `[1]` (RO)
    #[inline(always)]
    pub fn sm1_rxnempty(&self) -> bool {
        (self.value.read_bits() & Self::SM1_RXNEMPTY_MASK) != 0
    }

    /// **SM0_RXNEMPTY** `[0]` (RO)
    #[inline(always)]
    pub fn sm0_rxnempty(&self) -> bool {
        (self.value.read_bits() & Self::SM0_RXNEMPTY_MASK) != 0
    }

}

/// INTR value type for batch read/modify/write operations
///
/// Use `intr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `intr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntrVal {
    bits: u32,
}

impl IntrVal {
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

    /// Read **SM7** `[15:15]`
    #[inline(always)]
    pub const fn sm7(&self) -> bool {
        (self.bits & Intr::SM7_MASK) != 0
    }

    /// Read **SM6** `[14:14]`
    #[inline(always)]
    pub const fn sm6(&self) -> bool {
        (self.bits & Intr::SM6_MASK) != 0
    }

    /// Read **SM5** `[13:13]`
    #[inline(always)]
    pub const fn sm5(&self) -> bool {
        (self.bits & Intr::SM5_MASK) != 0
    }

    /// Read **SM4** `[12:12]`
    #[inline(always)]
    pub const fn sm4(&self) -> bool {
        (self.bits & Intr::SM4_MASK) != 0
    }

    /// Read **SM3** `[11:11]`
    #[inline(always)]
    pub const fn sm3(&self) -> bool {
        (self.bits & Intr::SM3_MASK) != 0
    }

    /// Read **SM2** `[10:10]`
    #[inline(always)]
    pub const fn sm2(&self) -> bool {
        (self.bits & Intr::SM2_MASK) != 0
    }

    /// Read **SM1** `[9:9]`
    #[inline(always)]
    pub const fn sm1(&self) -> bool {
        (self.bits & Intr::SM1_MASK) != 0
    }

    /// Read **SM0** `[8:8]`
    #[inline(always)]
    pub const fn sm0(&self) -> bool {
        (self.bits & Intr::SM0_MASK) != 0
    }

    /// Read **SM3_TXNFULL** `[7:7]`
    #[inline(always)]
    pub const fn sm3_txnfull(&self) -> bool {
        (self.bits & Intr::SM3_TXNFULL_MASK) != 0
    }

    /// Read **SM2_TXNFULL** `[6:6]`
    #[inline(always)]
    pub const fn sm2_txnfull(&self) -> bool {
        (self.bits & Intr::SM2_TXNFULL_MASK) != 0
    }

    /// Read **SM1_TXNFULL** `[5:5]`
    #[inline(always)]
    pub const fn sm1_txnfull(&self) -> bool {
        (self.bits & Intr::SM1_TXNFULL_MASK) != 0
    }

    /// Read **SM0_TXNFULL** `[4:4]`
    #[inline(always)]
    pub const fn sm0_txnfull(&self) -> bool {
        (self.bits & Intr::SM0_TXNFULL_MASK) != 0
    }

    /// Read **SM3_RXNEMPTY** `[3:3]`
    #[inline(always)]
    pub const fn sm3_rxnempty(&self) -> bool {
        (self.bits & Intr::SM3_RXNEMPTY_MASK) != 0
    }

    /// Read **SM2_RXNEMPTY** `[2:2]`
    #[inline(always)]
    pub const fn sm2_rxnempty(&self) -> bool {
        (self.bits & Intr::SM2_RXNEMPTY_MASK) != 0
    }

    /// Read **SM1_RXNEMPTY** `[1:1]`
    #[inline(always)]
    pub const fn sm1_rxnempty(&self) -> bool {
        (self.bits & Intr::SM1_RXNEMPTY_MASK) != 0
    }

    /// Read **SM0_RXNEMPTY** `[0:0]`
    #[inline(always)]
    pub const fn sm0_rxnempty(&self) -> bool {
        (self.bits & Intr::SM0_RXNEMPTY_MASK) != 0
    }

}

impl Default for IntrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Intr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IntrVal {
        IntrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IntrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IntrVal) -> IntrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// IRQ0_INTE Register
///
/// Interrupt Enable for irq0
///
/// Address offset: 0x0170
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SM7** `[15]` (RW)
///
/// - **SM6** `[14]` (RW)
///
/// - **SM5** `[13]` (RW)
///
/// - **SM4** `[12]` (RW)
///
/// - **SM3** `[11]` (RW)
///
/// - **SM2** `[10]` (RW)
///
/// - **SM1** `[9]` (RW)
///
/// - **SM0** `[8]` (RW)
///
/// - **SM3_TXNFULL** `[7]` (RW)
///
/// - **SM2_TXNFULL** `[6]` (RW)
///
/// - **SM1_TXNFULL** `[5]` (RW)
///
/// - **SM0_TXNFULL** `[4]` (RW)
///
/// - **SM3_RXNEMPTY** `[3]` (RW)
///
/// - **SM2_RXNEMPTY** `[2]` (RW)
///
/// - **SM1_RXNEMPTY** `[1]` (RW)
///
/// - **SM0_RXNEMPTY** `[0]` (RW)
#[repr(transparent)]
pub struct Irq0Inte {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Irq0Inte {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Irq0Inte {
    // ======== Field Constants ========

    /// **SM7**
    /// Bits `[15:15]`, mask `0x00008000`
    pub const SM7_OFFSET: u32 = 15;
    pub const SM7_MASK: u32 = 0x00008000;
    pub const SM7: u32 = 1 << 15;

    /// **SM6**
    /// Bits `[14:14]`, mask `0x00004000`
    pub const SM6_OFFSET: u32 = 14;
    pub const SM6_MASK: u32 = 0x00004000;
    pub const SM6: u32 = 1 << 14;

    /// **SM5**
    /// Bits `[13:13]`, mask `0x00002000`
    pub const SM5_OFFSET: u32 = 13;
    pub const SM5_MASK: u32 = 0x00002000;
    pub const SM5: u32 = 1 << 13;

    /// **SM4**
    /// Bits `[12:12]`, mask `0x00001000`
    pub const SM4_OFFSET: u32 = 12;
    pub const SM4_MASK: u32 = 0x00001000;
    pub const SM4: u32 = 1 << 12;

    /// **SM3**
    /// Bits `[11:11]`, mask `0x00000800`
    pub const SM3_OFFSET: u32 = 11;
    pub const SM3_MASK: u32 = 0x00000800;
    pub const SM3: u32 = 1 << 11;

    /// **SM2**
    /// Bits `[10:10]`, mask `0x00000400`
    pub const SM2_OFFSET: u32 = 10;
    pub const SM2_MASK: u32 = 0x00000400;
    pub const SM2: u32 = 1 << 10;

    /// **SM1**
    /// Bits `[9:9]`, mask `0x00000200`
    pub const SM1_OFFSET: u32 = 9;
    pub const SM1_MASK: u32 = 0x00000200;
    pub const SM1: u32 = 1 << 9;

    /// **SM0**
    /// Bits `[8:8]`, mask `0x00000100`
    pub const SM0_OFFSET: u32 = 8;
    pub const SM0_MASK: u32 = 0x00000100;
    pub const SM0: u32 = 1 << 8;

    /// **SM3_TXNFULL**
    /// Bits `[7:7]`, mask `0x00000080`
    pub const SM3_TXNFULL_OFFSET: u32 = 7;
    pub const SM3_TXNFULL_MASK: u32 = 0x00000080;
    pub const SM3_TXNFULL: u32 = 1 << 7;

    /// **SM2_TXNFULL**
    /// Bits `[6:6]`, mask `0x00000040`
    pub const SM2_TXNFULL_OFFSET: u32 = 6;
    pub const SM2_TXNFULL_MASK: u32 = 0x00000040;
    pub const SM2_TXNFULL: u32 = 1 << 6;

    /// **SM1_TXNFULL**
    /// Bits `[5:5]`, mask `0x00000020`
    pub const SM1_TXNFULL_OFFSET: u32 = 5;
    pub const SM1_TXNFULL_MASK: u32 = 0x00000020;
    pub const SM1_TXNFULL: u32 = 1 << 5;

    /// **SM0_TXNFULL**
    /// Bits `[4:4]`, mask `0x00000010`
    pub const SM0_TXNFULL_OFFSET: u32 = 4;
    pub const SM0_TXNFULL_MASK: u32 = 0x00000010;
    pub const SM0_TXNFULL: u32 = 1 << 4;

    /// **SM3_RXNEMPTY**
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SM3_RXNEMPTY_OFFSET: u32 = 3;
    pub const SM3_RXNEMPTY_MASK: u32 = 0x00000008;
    pub const SM3_RXNEMPTY: u32 = 1 << 3;

    /// **SM2_RXNEMPTY**
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SM2_RXNEMPTY_OFFSET: u32 = 2;
    pub const SM2_RXNEMPTY_MASK: u32 = 0x00000004;
    pub const SM2_RXNEMPTY: u32 = 1 << 2;

    /// **SM1_RXNEMPTY**
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SM1_RXNEMPTY_OFFSET: u32 = 1;
    pub const SM1_RXNEMPTY_MASK: u32 = 0x00000002;
    pub const SM1_RXNEMPTY: u32 = 1 << 1;

    /// **SM0_RXNEMPTY**
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SM0_RXNEMPTY_OFFSET: u32 = 0;
    pub const SM0_RXNEMPTY_MASK: u32 = 0x00000001;
    pub const SM0_RXNEMPTY: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **SM7** `[15]` (RW)
    #[inline(always)]
    pub fn sm7(&self) -> bool {
        (self.value.read_bits() & Self::SM7_MASK) != 0
    }

    /// Set SM7 bit atomically (RW)
    #[inline(always)]
    pub fn set_sm7(&self) {
        self.set_bits(Self::SM7_MASK);
    }

    /// Clear SM7 bit atomically (RW)
    #[inline(always)]
    pub fn clr_sm7(&self) {
        self.clr_bits(Self::SM7_MASK);
    }

    /// **SM6** `[14]` (RW)
    #[inline(always)]
    pub fn sm6(&self) -> bool {
        (self.value.read_bits() & Self::SM6_MASK) != 0
    }

    /// Set SM6 bit atomically (RW)
    #[inline(always)]
    pub fn set_sm6(&self) {
        self.set_bits(Self::SM6_MASK);
    }

    /// Clear SM6 bit atomically (RW)
    #[inline(always)]
    pub fn clr_sm6(&self) {
        self.clr_bits(Self::SM6_MASK);
    }

    /// **SM5** `[13]` (RW)
    #[inline(always)]
    pub fn sm5(&self) -> bool {
        (self.value.read_bits() & Self::SM5_MASK) != 0
    }

    /// Set SM5 bit atomically (RW)
    #[inline(always)]
    pub fn set_sm5(&self) {
        self.set_bits(Self::SM5_MASK);
    }

    /// Clear SM5 bit atomically (RW)
    #[inline(always)]
    pub fn clr_sm5(&self) {
        self.clr_bits(Self::SM5_MASK);
    }

    /// **SM4** `[12]` (RW)
    #[inline(always)]
    pub fn sm4(&self) -> bool {
        (self.value.read_bits() & Self::SM4_MASK) != 0
    }

    /// Set SM4 bit atomically (RW)
    #[inline(always)]
    pub fn set_sm4(&self) {
        self.set_bits(Self::SM4_MASK);
    }

    /// Clear SM4 bit atomically (RW)
    #[inline(always)]
    pub fn clr_sm4(&self) {
        self.clr_bits(Self::SM4_MASK);
    }

    /// **SM3** `[11]` (RW)
    #[inline(always)]
    pub fn sm3(&self) -> bool {
        (self.value.read_bits() & Self::SM3_MASK) != 0
    }

    /// Set SM3 bit atomically (RW)
    #[inline(always)]
    pub fn set_sm3(&self) {
        self.set_bits(Self::SM3_MASK);
    }

    /// Clear SM3 bit atomically (RW)
    #[inline(always)]
    pub fn clr_sm3(&self) {
        self.clr_bits(Self::SM3_MASK);
    }

    /// **SM2** `[10]` (RW)
    #[inline(always)]
    pub fn sm2(&self) -> bool {
        (self.value.read_bits() & Self::SM2_MASK) != 0
    }

    /// Set SM2 bit atomically (RW)
    #[inline(always)]
    pub fn set_sm2(&self) {
        self.set_bits(Self::SM2_MASK);
    }

    /// Clear SM2 bit atomically (RW)
    #[inline(always)]
    pub fn clr_sm2(&self) {
        self.clr_bits(Self::SM2_MASK);
    }

    /// **SM1** `[9]` (RW)
    #[inline(always)]
    pub fn sm1(&self) -> bool {
        (self.value.read_bits() & Self::SM1_MASK) != 0
    }

    /// Set SM1 bit atomically (RW)
    #[inline(always)]
    pub fn set_sm1(&self) {
        self.set_bits(Self::SM1_MASK);
    }

    /// Clear SM1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_sm1(&self) {
        self.clr_bits(Self::SM1_MASK);
    }

    /// **SM0** `[8]` (RW)
    #[inline(always)]
    pub fn sm0(&self) -> bool {
        (self.value.read_bits() & Self::SM0_MASK) != 0
    }

    /// Set SM0 bit atomically (RW)
    #[inline(always)]
    pub fn set_sm0(&self) {
        self.set_bits(Self::SM0_MASK);
    }

    /// Clear SM0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_sm0(&self) {
        self.clr_bits(Self::SM0_MASK);
    }

    /// **SM3_TXNFULL** `[7]` (RW)
    #[inline(always)]
    pub fn sm3_txnfull(&self) -> bool {
        (self.value.read_bits() & Self::SM3_TXNFULL_MASK) != 0
    }

    /// Set SM3_TXNFULL bit atomically (RW)
    #[inline(always)]
    pub fn set_sm3_txnfull(&self) {
        self.set_bits(Self::SM3_TXNFULL_MASK);
    }

    /// Clear SM3_TXNFULL bit atomically (RW)
    #[inline(always)]
    pub fn clr_sm3_txnfull(&self) {
        self.clr_bits(Self::SM3_TXNFULL_MASK);
    }

    /// **SM2_TXNFULL** `[6]` (RW)
    #[inline(always)]
    pub fn sm2_txnfull(&self) -> bool {
        (self.value.read_bits() & Self::SM2_TXNFULL_MASK) != 0
    }

    /// Set SM2_TXNFULL bit atomically (RW)
    #[inline(always)]
    pub fn set_sm2_txnfull(&self) {
        self.set_bits(Self::SM2_TXNFULL_MASK);
    }

    /// Clear SM2_TXNFULL bit atomically (RW)
    #[inline(always)]
    pub fn clr_sm2_txnfull(&self) {
        self.clr_bits(Self::SM2_TXNFULL_MASK);
    }

    /// **SM1_TXNFULL** `[5]` (RW)
    #[inline(always)]
    pub fn sm1_txnfull(&self) -> bool {
        (self.value.read_bits() & Self::SM1_TXNFULL_MASK) != 0
    }

    /// Set SM1_TXNFULL bit atomically (RW)
    #[inline(always)]
    pub fn set_sm1_txnfull(&self) {
        self.set_bits(Self::SM1_TXNFULL_MASK);
    }

    /// Clear SM1_TXNFULL bit atomically (RW)
    #[inline(always)]
    pub fn clr_sm1_txnfull(&self) {
        self.clr_bits(Self::SM1_TXNFULL_MASK);
    }

    /// **SM0_TXNFULL** `[4]` (RW)
    #[inline(always)]
    pub fn sm0_txnfull(&self) -> bool {
        (self.value.read_bits() & Self::SM0_TXNFULL_MASK) != 0
    }

    /// Set SM0_TXNFULL bit atomically (RW)
    #[inline(always)]
    pub fn set_sm0_txnfull(&self) {
        self.set_bits(Self::SM0_TXNFULL_MASK);
    }

    /// Clear SM0_TXNFULL bit atomically (RW)
    #[inline(always)]
    pub fn clr_sm0_txnfull(&self) {
        self.clr_bits(Self::SM0_TXNFULL_MASK);
    }

    /// **SM3_RXNEMPTY** `[3]` (RW)
    #[inline(always)]
    pub fn sm3_rxnempty(&self) -> bool {
        (self.value.read_bits() & Self::SM3_RXNEMPTY_MASK) != 0
    }

    /// Set SM3_RXNEMPTY bit atomically (RW)
    #[inline(always)]
    pub fn set_sm3_rxnempty(&self) {
        self.set_bits(Self::SM3_RXNEMPTY_MASK);
    }

    /// Clear SM3_RXNEMPTY bit atomically (RW)
    #[inline(always)]
    pub fn clr_sm3_rxnempty(&self) {
        self.clr_bits(Self::SM3_RXNEMPTY_MASK);
    }

    /// **SM2_RXNEMPTY** `[2]` (RW)
    #[inline(always)]
    pub fn sm2_rxnempty(&self) -> bool {
        (self.value.read_bits() & Self::SM2_RXNEMPTY_MASK) != 0
    }

    /// Set SM2_RXNEMPTY bit atomically (RW)
    #[inline(always)]
    pub fn set_sm2_rxnempty(&self) {
        self.set_bits(Self::SM2_RXNEMPTY_MASK);
    }

    /// Clear SM2_RXNEMPTY bit atomically (RW)
    #[inline(always)]
    pub fn clr_sm2_rxnempty(&self) {
        self.clr_bits(Self::SM2_RXNEMPTY_MASK);
    }

    /// **SM1_RXNEMPTY** `[1]` (RW)
    #[inline(always)]
    pub fn sm1_rxnempty(&self) -> bool {
        (self.value.read_bits() & Self::SM1_RXNEMPTY_MASK) != 0
    }

    /// Set SM1_RXNEMPTY bit atomically (RW)
    #[inline(always)]
    pub fn set_sm1_rxnempty(&self) {
        self.set_bits(Self::SM1_RXNEMPTY_MASK);
    }

    /// Clear SM1_RXNEMPTY bit atomically (RW)
    #[inline(always)]
    pub fn clr_sm1_rxnempty(&self) {
        self.clr_bits(Self::SM1_RXNEMPTY_MASK);
    }

    /// **SM0_RXNEMPTY** `[0]` (RW)
    #[inline(always)]
    pub fn sm0_rxnempty(&self) -> bool {
        (self.value.read_bits() & Self::SM0_RXNEMPTY_MASK) != 0
    }

    /// Set SM0_RXNEMPTY bit atomically (RW)
    #[inline(always)]
    pub fn set_sm0_rxnempty(&self) {
        self.set_bits(Self::SM0_RXNEMPTY_MASK);
    }

    /// Clear SM0_RXNEMPTY bit atomically (RW)
    #[inline(always)]
    pub fn clr_sm0_rxnempty(&self) {
        self.clr_bits(Self::SM0_RXNEMPTY_MASK);
    }

}

/// IRQ0_INTE value type for batch read/modify/write operations
///
/// Use `irq0inte.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `irq0inte.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Irq0InteVal {
    bits: u32,
}

impl Irq0InteVal {
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

    /// Read **SM7** `[15:15]`
    #[inline(always)]
    pub const fn sm7(&self) -> bool {
        (self.bits & Irq0Inte::SM7_MASK) != 0
    }

    /// Read **SM6** `[14:14]`
    #[inline(always)]
    pub const fn sm6(&self) -> bool {
        (self.bits & Irq0Inte::SM6_MASK) != 0
    }

    /// Read **SM5** `[13:13]`
    #[inline(always)]
    pub const fn sm5(&self) -> bool {
        (self.bits & Irq0Inte::SM5_MASK) != 0
    }

    /// Read **SM4** `[12:12]`
    #[inline(always)]
    pub const fn sm4(&self) -> bool {
        (self.bits & Irq0Inte::SM4_MASK) != 0
    }

    /// Read **SM3** `[11:11]`
    #[inline(always)]
    pub const fn sm3(&self) -> bool {
        (self.bits & Irq0Inte::SM3_MASK) != 0
    }

    /// Read **SM2** `[10:10]`
    #[inline(always)]
    pub const fn sm2(&self) -> bool {
        (self.bits & Irq0Inte::SM2_MASK) != 0
    }

    /// Read **SM1** `[9:9]`
    #[inline(always)]
    pub const fn sm1(&self) -> bool {
        (self.bits & Irq0Inte::SM1_MASK) != 0
    }

    /// Read **SM0** `[8:8]`
    #[inline(always)]
    pub const fn sm0(&self) -> bool {
        (self.bits & Irq0Inte::SM0_MASK) != 0
    }

    /// Read **SM3_TXNFULL** `[7:7]`
    #[inline(always)]
    pub const fn sm3_txnfull(&self) -> bool {
        (self.bits & Irq0Inte::SM3_TXNFULL_MASK) != 0
    }

    /// Read **SM2_TXNFULL** `[6:6]`
    #[inline(always)]
    pub const fn sm2_txnfull(&self) -> bool {
        (self.bits & Irq0Inte::SM2_TXNFULL_MASK) != 0
    }

    /// Read **SM1_TXNFULL** `[5:5]`
    #[inline(always)]
    pub const fn sm1_txnfull(&self) -> bool {
        (self.bits & Irq0Inte::SM1_TXNFULL_MASK) != 0
    }

    /// Read **SM0_TXNFULL** `[4:4]`
    #[inline(always)]
    pub const fn sm0_txnfull(&self) -> bool {
        (self.bits & Irq0Inte::SM0_TXNFULL_MASK) != 0
    }

    /// Read **SM3_RXNEMPTY** `[3:3]`
    #[inline(always)]
    pub const fn sm3_rxnempty(&self) -> bool {
        (self.bits & Irq0Inte::SM3_RXNEMPTY_MASK) != 0
    }

    /// Read **SM2_RXNEMPTY** `[2:2]`
    #[inline(always)]
    pub const fn sm2_rxnempty(&self) -> bool {
        (self.bits & Irq0Inte::SM2_RXNEMPTY_MASK) != 0
    }

    /// Read **SM1_RXNEMPTY** `[1:1]`
    #[inline(always)]
    pub const fn sm1_rxnempty(&self) -> bool {
        (self.bits & Irq0Inte::SM1_RXNEMPTY_MASK) != 0
    }

    /// Read **SM0_RXNEMPTY** `[0:0]`
    #[inline(always)]
    pub const fn sm0_rxnempty(&self) -> bool {
        (self.bits & Irq0Inte::SM0_RXNEMPTY_MASK) != 0
    }

    /// Set **SM7** `[15:15]`
    #[inline(always)]
    pub const fn with_sm7(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq0Inte::SM7_MASK
            } else {
                self.bits & !Irq0Inte::SM7_MASK
            }
        }
    }

    /// Set **SM6** `[14:14]`
    #[inline(always)]
    pub const fn with_sm6(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq0Inte::SM6_MASK
            } else {
                self.bits & !Irq0Inte::SM6_MASK
            }
        }
    }

    /// Set **SM5** `[13:13]`
    #[inline(always)]
    pub const fn with_sm5(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq0Inte::SM5_MASK
            } else {
                self.bits & !Irq0Inte::SM5_MASK
            }
        }
    }

    /// Set **SM4** `[12:12]`
    #[inline(always)]
    pub const fn with_sm4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq0Inte::SM4_MASK
            } else {
                self.bits & !Irq0Inte::SM4_MASK
            }
        }
    }

    /// Set **SM3** `[11:11]`
    #[inline(always)]
    pub const fn with_sm3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq0Inte::SM3_MASK
            } else {
                self.bits & !Irq0Inte::SM3_MASK
            }
        }
    }

    /// Set **SM2** `[10:10]`
    #[inline(always)]
    pub const fn with_sm2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq0Inte::SM2_MASK
            } else {
                self.bits & !Irq0Inte::SM2_MASK
            }
        }
    }

    /// Set **SM1** `[9:9]`
    #[inline(always)]
    pub const fn with_sm1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq0Inte::SM1_MASK
            } else {
                self.bits & !Irq0Inte::SM1_MASK
            }
        }
    }

    /// Set **SM0** `[8:8]`
    #[inline(always)]
    pub const fn with_sm0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq0Inte::SM0_MASK
            } else {
                self.bits & !Irq0Inte::SM0_MASK
            }
        }
    }

    /// Set **SM3_TXNFULL** `[7:7]`
    #[inline(always)]
    pub const fn with_sm3_txnfull(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq0Inte::SM3_TXNFULL_MASK
            } else {
                self.bits & !Irq0Inte::SM3_TXNFULL_MASK
            }
        }
    }

    /// Set **SM2_TXNFULL** `[6:6]`
    #[inline(always)]
    pub const fn with_sm2_txnfull(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq0Inte::SM2_TXNFULL_MASK
            } else {
                self.bits & !Irq0Inte::SM2_TXNFULL_MASK
            }
        }
    }

    /// Set **SM1_TXNFULL** `[5:5]`
    #[inline(always)]
    pub const fn with_sm1_txnfull(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq0Inte::SM1_TXNFULL_MASK
            } else {
                self.bits & !Irq0Inte::SM1_TXNFULL_MASK
            }
        }
    }

    /// Set **SM0_TXNFULL** `[4:4]`
    #[inline(always)]
    pub const fn with_sm0_txnfull(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq0Inte::SM0_TXNFULL_MASK
            } else {
                self.bits & !Irq0Inte::SM0_TXNFULL_MASK
            }
        }
    }

    /// Set **SM3_RXNEMPTY** `[3:3]`
    #[inline(always)]
    pub const fn with_sm3_rxnempty(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq0Inte::SM3_RXNEMPTY_MASK
            } else {
                self.bits & !Irq0Inte::SM3_RXNEMPTY_MASK
            }
        }
    }

    /// Set **SM2_RXNEMPTY** `[2:2]`
    #[inline(always)]
    pub const fn with_sm2_rxnempty(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq0Inte::SM2_RXNEMPTY_MASK
            } else {
                self.bits & !Irq0Inte::SM2_RXNEMPTY_MASK
            }
        }
    }

    /// Set **SM1_RXNEMPTY** `[1:1]`
    #[inline(always)]
    pub const fn with_sm1_rxnempty(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq0Inte::SM1_RXNEMPTY_MASK
            } else {
                self.bits & !Irq0Inte::SM1_RXNEMPTY_MASK
            }
        }
    }

    /// Set **SM0_RXNEMPTY** `[0:0]`
    #[inline(always)]
    pub const fn with_sm0_rxnempty(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq0Inte::SM0_RXNEMPTY_MASK
            } else {
                self.bits & !Irq0Inte::SM0_RXNEMPTY_MASK
            }
        }
    }

}

impl Default for Irq0InteVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Irq0Inte {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Irq0InteVal {
        Irq0InteVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Irq0InteVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Irq0InteVal) -> Irq0InteVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// IRQ0_INTF Register
///
/// Interrupt Force for irq0
///
/// Address offset: 0x0174
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SM7** `[15]` (RW)
///
/// - **SM6** `[14]` (RW)
///
/// - **SM5** `[13]` (RW)
///
/// - **SM4** `[12]` (RW)
///
/// - **SM3** `[11]` (RW)
///
/// - **SM2** `[10]` (RW)
///
/// - **SM1** `[9]` (RW)
///
/// - **SM0** `[8]` (RW)
///
/// - **SM3_TXNFULL** `[7]` (RW)
///
/// - **SM2_TXNFULL** `[6]` (RW)
///
/// - **SM1_TXNFULL** `[5]` (RW)
///
/// - **SM0_TXNFULL** `[4]` (RW)
///
/// - **SM3_RXNEMPTY** `[3]` (RW)
///
/// - **SM2_RXNEMPTY** `[2]` (RW)
///
/// - **SM1_RXNEMPTY** `[1]` (RW)
///
/// - **SM0_RXNEMPTY** `[0]` (RW)
#[repr(transparent)]
pub struct Irq0Intf {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Irq0Intf {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Irq0Intf {
    // ======== Field Constants ========

    /// **SM7**
    /// Bits `[15:15]`, mask `0x00008000`
    pub const SM7_OFFSET: u32 = 15;
    pub const SM7_MASK: u32 = 0x00008000;
    pub const SM7: u32 = 1 << 15;

    /// **SM6**
    /// Bits `[14:14]`, mask `0x00004000`
    pub const SM6_OFFSET: u32 = 14;
    pub const SM6_MASK: u32 = 0x00004000;
    pub const SM6: u32 = 1 << 14;

    /// **SM5**
    /// Bits `[13:13]`, mask `0x00002000`
    pub const SM5_OFFSET: u32 = 13;
    pub const SM5_MASK: u32 = 0x00002000;
    pub const SM5: u32 = 1 << 13;

    /// **SM4**
    /// Bits `[12:12]`, mask `0x00001000`
    pub const SM4_OFFSET: u32 = 12;
    pub const SM4_MASK: u32 = 0x00001000;
    pub const SM4: u32 = 1 << 12;

    /// **SM3**
    /// Bits `[11:11]`, mask `0x00000800`
    pub const SM3_OFFSET: u32 = 11;
    pub const SM3_MASK: u32 = 0x00000800;
    pub const SM3: u32 = 1 << 11;

    /// **SM2**
    /// Bits `[10:10]`, mask `0x00000400`
    pub const SM2_OFFSET: u32 = 10;
    pub const SM2_MASK: u32 = 0x00000400;
    pub const SM2: u32 = 1 << 10;

    /// **SM1**
    /// Bits `[9:9]`, mask `0x00000200`
    pub const SM1_OFFSET: u32 = 9;
    pub const SM1_MASK: u32 = 0x00000200;
    pub const SM1: u32 = 1 << 9;

    /// **SM0**
    /// Bits `[8:8]`, mask `0x00000100`
    pub const SM0_OFFSET: u32 = 8;
    pub const SM0_MASK: u32 = 0x00000100;
    pub const SM0: u32 = 1 << 8;

    /// **SM3_TXNFULL**
    /// Bits `[7:7]`, mask `0x00000080`
    pub const SM3_TXNFULL_OFFSET: u32 = 7;
    pub const SM3_TXNFULL_MASK: u32 = 0x00000080;
    pub const SM3_TXNFULL: u32 = 1 << 7;

    /// **SM2_TXNFULL**
    /// Bits `[6:6]`, mask `0x00000040`
    pub const SM2_TXNFULL_OFFSET: u32 = 6;
    pub const SM2_TXNFULL_MASK: u32 = 0x00000040;
    pub const SM2_TXNFULL: u32 = 1 << 6;

    /// **SM1_TXNFULL**
    /// Bits `[5:5]`, mask `0x00000020`
    pub const SM1_TXNFULL_OFFSET: u32 = 5;
    pub const SM1_TXNFULL_MASK: u32 = 0x00000020;
    pub const SM1_TXNFULL: u32 = 1 << 5;

    /// **SM0_TXNFULL**
    /// Bits `[4:4]`, mask `0x00000010`
    pub const SM0_TXNFULL_OFFSET: u32 = 4;
    pub const SM0_TXNFULL_MASK: u32 = 0x00000010;
    pub const SM0_TXNFULL: u32 = 1 << 4;

    /// **SM3_RXNEMPTY**
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SM3_RXNEMPTY_OFFSET: u32 = 3;
    pub const SM3_RXNEMPTY_MASK: u32 = 0x00000008;
    pub const SM3_RXNEMPTY: u32 = 1 << 3;

    /// **SM2_RXNEMPTY**
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SM2_RXNEMPTY_OFFSET: u32 = 2;
    pub const SM2_RXNEMPTY_MASK: u32 = 0x00000004;
    pub const SM2_RXNEMPTY: u32 = 1 << 2;

    /// **SM1_RXNEMPTY**
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SM1_RXNEMPTY_OFFSET: u32 = 1;
    pub const SM1_RXNEMPTY_MASK: u32 = 0x00000002;
    pub const SM1_RXNEMPTY: u32 = 1 << 1;

    /// **SM0_RXNEMPTY**
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SM0_RXNEMPTY_OFFSET: u32 = 0;
    pub const SM0_RXNEMPTY_MASK: u32 = 0x00000001;
    pub const SM0_RXNEMPTY: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **SM7** `[15]` (RW)
    #[inline(always)]
    pub fn sm7(&self) -> bool {
        (self.value.read_bits() & Self::SM7_MASK) != 0
    }

    /// Set SM7 bit atomically (RW)
    #[inline(always)]
    pub fn set_sm7(&self) {
        self.set_bits(Self::SM7_MASK);
    }

    /// Clear SM7 bit atomically (RW)
    #[inline(always)]
    pub fn clr_sm7(&self) {
        self.clr_bits(Self::SM7_MASK);
    }

    /// **SM6** `[14]` (RW)
    #[inline(always)]
    pub fn sm6(&self) -> bool {
        (self.value.read_bits() & Self::SM6_MASK) != 0
    }

    /// Set SM6 bit atomically (RW)
    #[inline(always)]
    pub fn set_sm6(&self) {
        self.set_bits(Self::SM6_MASK);
    }

    /// Clear SM6 bit atomically (RW)
    #[inline(always)]
    pub fn clr_sm6(&self) {
        self.clr_bits(Self::SM6_MASK);
    }

    /// **SM5** `[13]` (RW)
    #[inline(always)]
    pub fn sm5(&self) -> bool {
        (self.value.read_bits() & Self::SM5_MASK) != 0
    }

    /// Set SM5 bit atomically (RW)
    #[inline(always)]
    pub fn set_sm5(&self) {
        self.set_bits(Self::SM5_MASK);
    }

    /// Clear SM5 bit atomically (RW)
    #[inline(always)]
    pub fn clr_sm5(&self) {
        self.clr_bits(Self::SM5_MASK);
    }

    /// **SM4** `[12]` (RW)
    #[inline(always)]
    pub fn sm4(&self) -> bool {
        (self.value.read_bits() & Self::SM4_MASK) != 0
    }

    /// Set SM4 bit atomically (RW)
    #[inline(always)]
    pub fn set_sm4(&self) {
        self.set_bits(Self::SM4_MASK);
    }

    /// Clear SM4 bit atomically (RW)
    #[inline(always)]
    pub fn clr_sm4(&self) {
        self.clr_bits(Self::SM4_MASK);
    }

    /// **SM3** `[11]` (RW)
    #[inline(always)]
    pub fn sm3(&self) -> bool {
        (self.value.read_bits() & Self::SM3_MASK) != 0
    }

    /// Set SM3 bit atomically (RW)
    #[inline(always)]
    pub fn set_sm3(&self) {
        self.set_bits(Self::SM3_MASK);
    }

    /// Clear SM3 bit atomically (RW)
    #[inline(always)]
    pub fn clr_sm3(&self) {
        self.clr_bits(Self::SM3_MASK);
    }

    /// **SM2** `[10]` (RW)
    #[inline(always)]
    pub fn sm2(&self) -> bool {
        (self.value.read_bits() & Self::SM2_MASK) != 0
    }

    /// Set SM2 bit atomically (RW)
    #[inline(always)]
    pub fn set_sm2(&self) {
        self.set_bits(Self::SM2_MASK);
    }

    /// Clear SM2 bit atomically (RW)
    #[inline(always)]
    pub fn clr_sm2(&self) {
        self.clr_bits(Self::SM2_MASK);
    }

    /// **SM1** `[9]` (RW)
    #[inline(always)]
    pub fn sm1(&self) -> bool {
        (self.value.read_bits() & Self::SM1_MASK) != 0
    }

    /// Set SM1 bit atomically (RW)
    #[inline(always)]
    pub fn set_sm1(&self) {
        self.set_bits(Self::SM1_MASK);
    }

    /// Clear SM1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_sm1(&self) {
        self.clr_bits(Self::SM1_MASK);
    }

    /// **SM0** `[8]` (RW)
    #[inline(always)]
    pub fn sm0(&self) -> bool {
        (self.value.read_bits() & Self::SM0_MASK) != 0
    }

    /// Set SM0 bit atomically (RW)
    #[inline(always)]
    pub fn set_sm0(&self) {
        self.set_bits(Self::SM0_MASK);
    }

    /// Clear SM0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_sm0(&self) {
        self.clr_bits(Self::SM0_MASK);
    }

    /// **SM3_TXNFULL** `[7]` (RW)
    #[inline(always)]
    pub fn sm3_txnfull(&self) -> bool {
        (self.value.read_bits() & Self::SM3_TXNFULL_MASK) != 0
    }

    /// Set SM3_TXNFULL bit atomically (RW)
    #[inline(always)]
    pub fn set_sm3_txnfull(&self) {
        self.set_bits(Self::SM3_TXNFULL_MASK);
    }

    /// Clear SM3_TXNFULL bit atomically (RW)
    #[inline(always)]
    pub fn clr_sm3_txnfull(&self) {
        self.clr_bits(Self::SM3_TXNFULL_MASK);
    }

    /// **SM2_TXNFULL** `[6]` (RW)
    #[inline(always)]
    pub fn sm2_txnfull(&self) -> bool {
        (self.value.read_bits() & Self::SM2_TXNFULL_MASK) != 0
    }

    /// Set SM2_TXNFULL bit atomically (RW)
    #[inline(always)]
    pub fn set_sm2_txnfull(&self) {
        self.set_bits(Self::SM2_TXNFULL_MASK);
    }

    /// Clear SM2_TXNFULL bit atomically (RW)
    #[inline(always)]
    pub fn clr_sm2_txnfull(&self) {
        self.clr_bits(Self::SM2_TXNFULL_MASK);
    }

    /// **SM1_TXNFULL** `[5]` (RW)
    #[inline(always)]
    pub fn sm1_txnfull(&self) -> bool {
        (self.value.read_bits() & Self::SM1_TXNFULL_MASK) != 0
    }

    /// Set SM1_TXNFULL bit atomically (RW)
    #[inline(always)]
    pub fn set_sm1_txnfull(&self) {
        self.set_bits(Self::SM1_TXNFULL_MASK);
    }

    /// Clear SM1_TXNFULL bit atomically (RW)
    #[inline(always)]
    pub fn clr_sm1_txnfull(&self) {
        self.clr_bits(Self::SM1_TXNFULL_MASK);
    }

    /// **SM0_TXNFULL** `[4]` (RW)
    #[inline(always)]
    pub fn sm0_txnfull(&self) -> bool {
        (self.value.read_bits() & Self::SM0_TXNFULL_MASK) != 0
    }

    /// Set SM0_TXNFULL bit atomically (RW)
    #[inline(always)]
    pub fn set_sm0_txnfull(&self) {
        self.set_bits(Self::SM0_TXNFULL_MASK);
    }

    /// Clear SM0_TXNFULL bit atomically (RW)
    #[inline(always)]
    pub fn clr_sm0_txnfull(&self) {
        self.clr_bits(Self::SM0_TXNFULL_MASK);
    }

    /// **SM3_RXNEMPTY** `[3]` (RW)
    #[inline(always)]
    pub fn sm3_rxnempty(&self) -> bool {
        (self.value.read_bits() & Self::SM3_RXNEMPTY_MASK) != 0
    }

    /// Set SM3_RXNEMPTY bit atomically (RW)
    #[inline(always)]
    pub fn set_sm3_rxnempty(&self) {
        self.set_bits(Self::SM3_RXNEMPTY_MASK);
    }

    /// Clear SM3_RXNEMPTY bit atomically (RW)
    #[inline(always)]
    pub fn clr_sm3_rxnempty(&self) {
        self.clr_bits(Self::SM3_RXNEMPTY_MASK);
    }

    /// **SM2_RXNEMPTY** `[2]` (RW)
    #[inline(always)]
    pub fn sm2_rxnempty(&self) -> bool {
        (self.value.read_bits() & Self::SM2_RXNEMPTY_MASK) != 0
    }

    /// Set SM2_RXNEMPTY bit atomically (RW)
    #[inline(always)]
    pub fn set_sm2_rxnempty(&self) {
        self.set_bits(Self::SM2_RXNEMPTY_MASK);
    }

    /// Clear SM2_RXNEMPTY bit atomically (RW)
    #[inline(always)]
    pub fn clr_sm2_rxnempty(&self) {
        self.clr_bits(Self::SM2_RXNEMPTY_MASK);
    }

    /// **SM1_RXNEMPTY** `[1]` (RW)
    #[inline(always)]
    pub fn sm1_rxnempty(&self) -> bool {
        (self.value.read_bits() & Self::SM1_RXNEMPTY_MASK) != 0
    }

    /// Set SM1_RXNEMPTY bit atomically (RW)
    #[inline(always)]
    pub fn set_sm1_rxnempty(&self) {
        self.set_bits(Self::SM1_RXNEMPTY_MASK);
    }

    /// Clear SM1_RXNEMPTY bit atomically (RW)
    #[inline(always)]
    pub fn clr_sm1_rxnempty(&self) {
        self.clr_bits(Self::SM1_RXNEMPTY_MASK);
    }

    /// **SM0_RXNEMPTY** `[0]` (RW)
    #[inline(always)]
    pub fn sm0_rxnempty(&self) -> bool {
        (self.value.read_bits() & Self::SM0_RXNEMPTY_MASK) != 0
    }

    /// Set SM0_RXNEMPTY bit atomically (RW)
    #[inline(always)]
    pub fn set_sm0_rxnempty(&self) {
        self.set_bits(Self::SM0_RXNEMPTY_MASK);
    }

    /// Clear SM0_RXNEMPTY bit atomically (RW)
    #[inline(always)]
    pub fn clr_sm0_rxnempty(&self) {
        self.clr_bits(Self::SM0_RXNEMPTY_MASK);
    }

}

/// IRQ0_INTF value type for batch read/modify/write operations
///
/// Use `irq0intf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `irq0intf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Irq0IntfVal {
    bits: u32,
}

impl Irq0IntfVal {
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

    /// Read **SM7** `[15:15]`
    #[inline(always)]
    pub const fn sm7(&self) -> bool {
        (self.bits & Irq0Intf::SM7_MASK) != 0
    }

    /// Read **SM6** `[14:14]`
    #[inline(always)]
    pub const fn sm6(&self) -> bool {
        (self.bits & Irq0Intf::SM6_MASK) != 0
    }

    /// Read **SM5** `[13:13]`
    #[inline(always)]
    pub const fn sm5(&self) -> bool {
        (self.bits & Irq0Intf::SM5_MASK) != 0
    }

    /// Read **SM4** `[12:12]`
    #[inline(always)]
    pub const fn sm4(&self) -> bool {
        (self.bits & Irq0Intf::SM4_MASK) != 0
    }

    /// Read **SM3** `[11:11]`
    #[inline(always)]
    pub const fn sm3(&self) -> bool {
        (self.bits & Irq0Intf::SM3_MASK) != 0
    }

    /// Read **SM2** `[10:10]`
    #[inline(always)]
    pub const fn sm2(&self) -> bool {
        (self.bits & Irq0Intf::SM2_MASK) != 0
    }

    /// Read **SM1** `[9:9]`
    #[inline(always)]
    pub const fn sm1(&self) -> bool {
        (self.bits & Irq0Intf::SM1_MASK) != 0
    }

    /// Read **SM0** `[8:8]`
    #[inline(always)]
    pub const fn sm0(&self) -> bool {
        (self.bits & Irq0Intf::SM0_MASK) != 0
    }

    /// Read **SM3_TXNFULL** `[7:7]`
    #[inline(always)]
    pub const fn sm3_txnfull(&self) -> bool {
        (self.bits & Irq0Intf::SM3_TXNFULL_MASK) != 0
    }

    /// Read **SM2_TXNFULL** `[6:6]`
    #[inline(always)]
    pub const fn sm2_txnfull(&self) -> bool {
        (self.bits & Irq0Intf::SM2_TXNFULL_MASK) != 0
    }

    /// Read **SM1_TXNFULL** `[5:5]`
    #[inline(always)]
    pub const fn sm1_txnfull(&self) -> bool {
        (self.bits & Irq0Intf::SM1_TXNFULL_MASK) != 0
    }

    /// Read **SM0_TXNFULL** `[4:4]`
    #[inline(always)]
    pub const fn sm0_txnfull(&self) -> bool {
        (self.bits & Irq0Intf::SM0_TXNFULL_MASK) != 0
    }

    /// Read **SM3_RXNEMPTY** `[3:3]`
    #[inline(always)]
    pub const fn sm3_rxnempty(&self) -> bool {
        (self.bits & Irq0Intf::SM3_RXNEMPTY_MASK) != 0
    }

    /// Read **SM2_RXNEMPTY** `[2:2]`
    #[inline(always)]
    pub const fn sm2_rxnempty(&self) -> bool {
        (self.bits & Irq0Intf::SM2_RXNEMPTY_MASK) != 0
    }

    /// Read **SM1_RXNEMPTY** `[1:1]`
    #[inline(always)]
    pub const fn sm1_rxnempty(&self) -> bool {
        (self.bits & Irq0Intf::SM1_RXNEMPTY_MASK) != 0
    }

    /// Read **SM0_RXNEMPTY** `[0:0]`
    #[inline(always)]
    pub const fn sm0_rxnempty(&self) -> bool {
        (self.bits & Irq0Intf::SM0_RXNEMPTY_MASK) != 0
    }

    /// Set **SM7** `[15:15]`
    #[inline(always)]
    pub const fn with_sm7(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq0Intf::SM7_MASK
            } else {
                self.bits & !Irq0Intf::SM7_MASK
            }
        }
    }

    /// Set **SM6** `[14:14]`
    #[inline(always)]
    pub const fn with_sm6(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq0Intf::SM6_MASK
            } else {
                self.bits & !Irq0Intf::SM6_MASK
            }
        }
    }

    /// Set **SM5** `[13:13]`
    #[inline(always)]
    pub const fn with_sm5(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq0Intf::SM5_MASK
            } else {
                self.bits & !Irq0Intf::SM5_MASK
            }
        }
    }

    /// Set **SM4** `[12:12]`
    #[inline(always)]
    pub const fn with_sm4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq0Intf::SM4_MASK
            } else {
                self.bits & !Irq0Intf::SM4_MASK
            }
        }
    }

    /// Set **SM3** `[11:11]`
    #[inline(always)]
    pub const fn with_sm3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq0Intf::SM3_MASK
            } else {
                self.bits & !Irq0Intf::SM3_MASK
            }
        }
    }

    /// Set **SM2** `[10:10]`
    #[inline(always)]
    pub const fn with_sm2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq0Intf::SM2_MASK
            } else {
                self.bits & !Irq0Intf::SM2_MASK
            }
        }
    }

    /// Set **SM1** `[9:9]`
    #[inline(always)]
    pub const fn with_sm1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq0Intf::SM1_MASK
            } else {
                self.bits & !Irq0Intf::SM1_MASK
            }
        }
    }

    /// Set **SM0** `[8:8]`
    #[inline(always)]
    pub const fn with_sm0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq0Intf::SM0_MASK
            } else {
                self.bits & !Irq0Intf::SM0_MASK
            }
        }
    }

    /// Set **SM3_TXNFULL** `[7:7]`
    #[inline(always)]
    pub const fn with_sm3_txnfull(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq0Intf::SM3_TXNFULL_MASK
            } else {
                self.bits & !Irq0Intf::SM3_TXNFULL_MASK
            }
        }
    }

    /// Set **SM2_TXNFULL** `[6:6]`
    #[inline(always)]
    pub const fn with_sm2_txnfull(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq0Intf::SM2_TXNFULL_MASK
            } else {
                self.bits & !Irq0Intf::SM2_TXNFULL_MASK
            }
        }
    }

    /// Set **SM1_TXNFULL** `[5:5]`
    #[inline(always)]
    pub const fn with_sm1_txnfull(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq0Intf::SM1_TXNFULL_MASK
            } else {
                self.bits & !Irq0Intf::SM1_TXNFULL_MASK
            }
        }
    }

    /// Set **SM0_TXNFULL** `[4:4]`
    #[inline(always)]
    pub const fn with_sm0_txnfull(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq0Intf::SM0_TXNFULL_MASK
            } else {
                self.bits & !Irq0Intf::SM0_TXNFULL_MASK
            }
        }
    }

    /// Set **SM3_RXNEMPTY** `[3:3]`
    #[inline(always)]
    pub const fn with_sm3_rxnempty(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq0Intf::SM3_RXNEMPTY_MASK
            } else {
                self.bits & !Irq0Intf::SM3_RXNEMPTY_MASK
            }
        }
    }

    /// Set **SM2_RXNEMPTY** `[2:2]`
    #[inline(always)]
    pub const fn with_sm2_rxnempty(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq0Intf::SM2_RXNEMPTY_MASK
            } else {
                self.bits & !Irq0Intf::SM2_RXNEMPTY_MASK
            }
        }
    }

    /// Set **SM1_RXNEMPTY** `[1:1]`
    #[inline(always)]
    pub const fn with_sm1_rxnempty(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq0Intf::SM1_RXNEMPTY_MASK
            } else {
                self.bits & !Irq0Intf::SM1_RXNEMPTY_MASK
            }
        }
    }

    /// Set **SM0_RXNEMPTY** `[0:0]`
    #[inline(always)]
    pub const fn with_sm0_rxnempty(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq0Intf::SM0_RXNEMPTY_MASK
            } else {
                self.bits & !Irq0Intf::SM0_RXNEMPTY_MASK
            }
        }
    }

}

impl Default for Irq0IntfVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Irq0Intf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Irq0IntfVal {
        Irq0IntfVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Irq0IntfVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Irq0IntfVal) -> Irq0IntfVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// IRQ0_INTS Register
///
/// Interrupt status after masking & forcing for irq0
///
/// Address offset: 0x0178
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SM7** `[15]` (RO)
///
/// - **SM6** `[14]` (RO)
///
/// - **SM5** `[13]` (RO)
///
/// - **SM4** `[12]` (RO)
///
/// - **SM3** `[11]` (RO)
///
/// - **SM2** `[10]` (RO)
///
/// - **SM1** `[9]` (RO)
///
/// - **SM0** `[8]` (RO)
///
/// - **SM3_TXNFULL** `[7]` (RO)
///
/// - **SM2_TXNFULL** `[6]` (RO)
///
/// - **SM1_TXNFULL** `[5]` (RO)
///
/// - **SM0_TXNFULL** `[4]` (RO)
///
/// - **SM3_RXNEMPTY** `[3]` (RO)
///
/// - **SM2_RXNEMPTY** `[2]` (RO)
///
/// - **SM1_RXNEMPTY** `[1]` (RO)
///
/// - **SM0_RXNEMPTY** `[0]` (RO)
#[repr(transparent)]
pub struct Irq0Ints {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Irq0Ints {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Irq0Ints {
    // ======== Field Constants ========

    /// **SM7**
    /// Bits `[15:15]`, mask `0x00008000`
    pub const SM7_OFFSET: u32 = 15;
    pub const SM7_MASK: u32 = 0x00008000;
    pub const SM7: u32 = 1 << 15;

    /// **SM6**
    /// Bits `[14:14]`, mask `0x00004000`
    pub const SM6_OFFSET: u32 = 14;
    pub const SM6_MASK: u32 = 0x00004000;
    pub const SM6: u32 = 1 << 14;

    /// **SM5**
    /// Bits `[13:13]`, mask `0x00002000`
    pub const SM5_OFFSET: u32 = 13;
    pub const SM5_MASK: u32 = 0x00002000;
    pub const SM5: u32 = 1 << 13;

    /// **SM4**
    /// Bits `[12:12]`, mask `0x00001000`
    pub const SM4_OFFSET: u32 = 12;
    pub const SM4_MASK: u32 = 0x00001000;
    pub const SM4: u32 = 1 << 12;

    /// **SM3**
    /// Bits `[11:11]`, mask `0x00000800`
    pub const SM3_OFFSET: u32 = 11;
    pub const SM3_MASK: u32 = 0x00000800;
    pub const SM3: u32 = 1 << 11;

    /// **SM2**
    /// Bits `[10:10]`, mask `0x00000400`
    pub const SM2_OFFSET: u32 = 10;
    pub const SM2_MASK: u32 = 0x00000400;
    pub const SM2: u32 = 1 << 10;

    /// **SM1**
    /// Bits `[9:9]`, mask `0x00000200`
    pub const SM1_OFFSET: u32 = 9;
    pub const SM1_MASK: u32 = 0x00000200;
    pub const SM1: u32 = 1 << 9;

    /// **SM0**
    /// Bits `[8:8]`, mask `0x00000100`
    pub const SM0_OFFSET: u32 = 8;
    pub const SM0_MASK: u32 = 0x00000100;
    pub const SM0: u32 = 1 << 8;

    /// **SM3_TXNFULL**
    /// Bits `[7:7]`, mask `0x00000080`
    pub const SM3_TXNFULL_OFFSET: u32 = 7;
    pub const SM3_TXNFULL_MASK: u32 = 0x00000080;
    pub const SM3_TXNFULL: u32 = 1 << 7;

    /// **SM2_TXNFULL**
    /// Bits `[6:6]`, mask `0x00000040`
    pub const SM2_TXNFULL_OFFSET: u32 = 6;
    pub const SM2_TXNFULL_MASK: u32 = 0x00000040;
    pub const SM2_TXNFULL: u32 = 1 << 6;

    /// **SM1_TXNFULL**
    /// Bits `[5:5]`, mask `0x00000020`
    pub const SM1_TXNFULL_OFFSET: u32 = 5;
    pub const SM1_TXNFULL_MASK: u32 = 0x00000020;
    pub const SM1_TXNFULL: u32 = 1 << 5;

    /// **SM0_TXNFULL**
    /// Bits `[4:4]`, mask `0x00000010`
    pub const SM0_TXNFULL_OFFSET: u32 = 4;
    pub const SM0_TXNFULL_MASK: u32 = 0x00000010;
    pub const SM0_TXNFULL: u32 = 1 << 4;

    /// **SM3_RXNEMPTY**
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SM3_RXNEMPTY_OFFSET: u32 = 3;
    pub const SM3_RXNEMPTY_MASK: u32 = 0x00000008;
    pub const SM3_RXNEMPTY: u32 = 1 << 3;

    /// **SM2_RXNEMPTY**
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SM2_RXNEMPTY_OFFSET: u32 = 2;
    pub const SM2_RXNEMPTY_MASK: u32 = 0x00000004;
    pub const SM2_RXNEMPTY: u32 = 1 << 2;

    /// **SM1_RXNEMPTY**
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SM1_RXNEMPTY_OFFSET: u32 = 1;
    pub const SM1_RXNEMPTY_MASK: u32 = 0x00000002;
    pub const SM1_RXNEMPTY: u32 = 1 << 1;

    /// **SM0_RXNEMPTY**
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SM0_RXNEMPTY_OFFSET: u32 = 0;
    pub const SM0_RXNEMPTY_MASK: u32 = 0x00000001;
    pub const SM0_RXNEMPTY: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **SM7** `[15]` (RO)
    #[inline(always)]
    pub fn sm7(&self) -> bool {
        (self.value.read_bits() & Self::SM7_MASK) != 0
    }

    /// **SM6** `[14]` (RO)
    #[inline(always)]
    pub fn sm6(&self) -> bool {
        (self.value.read_bits() & Self::SM6_MASK) != 0
    }

    /// **SM5** `[13]` (RO)
    #[inline(always)]
    pub fn sm5(&self) -> bool {
        (self.value.read_bits() & Self::SM5_MASK) != 0
    }

    /// **SM4** `[12]` (RO)
    #[inline(always)]
    pub fn sm4(&self) -> bool {
        (self.value.read_bits() & Self::SM4_MASK) != 0
    }

    /// **SM3** `[11]` (RO)
    #[inline(always)]
    pub fn sm3(&self) -> bool {
        (self.value.read_bits() & Self::SM3_MASK) != 0
    }

    /// **SM2** `[10]` (RO)
    #[inline(always)]
    pub fn sm2(&self) -> bool {
        (self.value.read_bits() & Self::SM2_MASK) != 0
    }

    /// **SM1** `[9]` (RO)
    #[inline(always)]
    pub fn sm1(&self) -> bool {
        (self.value.read_bits() & Self::SM1_MASK) != 0
    }

    /// **SM0** `[8]` (RO)
    #[inline(always)]
    pub fn sm0(&self) -> bool {
        (self.value.read_bits() & Self::SM0_MASK) != 0
    }

    /// **SM3_TXNFULL** `[7]` (RO)
    #[inline(always)]
    pub fn sm3_txnfull(&self) -> bool {
        (self.value.read_bits() & Self::SM3_TXNFULL_MASK) != 0
    }

    /// **SM2_TXNFULL** `[6]` (RO)
    #[inline(always)]
    pub fn sm2_txnfull(&self) -> bool {
        (self.value.read_bits() & Self::SM2_TXNFULL_MASK) != 0
    }

    /// **SM1_TXNFULL** `[5]` (RO)
    #[inline(always)]
    pub fn sm1_txnfull(&self) -> bool {
        (self.value.read_bits() & Self::SM1_TXNFULL_MASK) != 0
    }

    /// **SM0_TXNFULL** `[4]` (RO)
    #[inline(always)]
    pub fn sm0_txnfull(&self) -> bool {
        (self.value.read_bits() & Self::SM0_TXNFULL_MASK) != 0
    }

    /// **SM3_RXNEMPTY** `[3]` (RO)
    #[inline(always)]
    pub fn sm3_rxnempty(&self) -> bool {
        (self.value.read_bits() & Self::SM3_RXNEMPTY_MASK) != 0
    }

    /// **SM2_RXNEMPTY** `[2]` (RO)
    #[inline(always)]
    pub fn sm2_rxnempty(&self) -> bool {
        (self.value.read_bits() & Self::SM2_RXNEMPTY_MASK) != 0
    }

    /// **SM1_RXNEMPTY** `[1]` (RO)
    #[inline(always)]
    pub fn sm1_rxnempty(&self) -> bool {
        (self.value.read_bits() & Self::SM1_RXNEMPTY_MASK) != 0
    }

    /// **SM0_RXNEMPTY** `[0]` (RO)
    #[inline(always)]
    pub fn sm0_rxnempty(&self) -> bool {
        (self.value.read_bits() & Self::SM0_RXNEMPTY_MASK) != 0
    }

}

/// IRQ0_INTS value type for batch read/modify/write operations
///
/// Use `irq0ints.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `irq0ints.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Irq0IntsVal {
    bits: u32,
}

impl Irq0IntsVal {
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

    /// Read **SM7** `[15:15]`
    #[inline(always)]
    pub const fn sm7(&self) -> bool {
        (self.bits & Irq0Ints::SM7_MASK) != 0
    }

    /// Read **SM6** `[14:14]`
    #[inline(always)]
    pub const fn sm6(&self) -> bool {
        (self.bits & Irq0Ints::SM6_MASK) != 0
    }

    /// Read **SM5** `[13:13]`
    #[inline(always)]
    pub const fn sm5(&self) -> bool {
        (self.bits & Irq0Ints::SM5_MASK) != 0
    }

    /// Read **SM4** `[12:12]`
    #[inline(always)]
    pub const fn sm4(&self) -> bool {
        (self.bits & Irq0Ints::SM4_MASK) != 0
    }

    /// Read **SM3** `[11:11]`
    #[inline(always)]
    pub const fn sm3(&self) -> bool {
        (self.bits & Irq0Ints::SM3_MASK) != 0
    }

    /// Read **SM2** `[10:10]`
    #[inline(always)]
    pub const fn sm2(&self) -> bool {
        (self.bits & Irq0Ints::SM2_MASK) != 0
    }

    /// Read **SM1** `[9:9]`
    #[inline(always)]
    pub const fn sm1(&self) -> bool {
        (self.bits & Irq0Ints::SM1_MASK) != 0
    }

    /// Read **SM0** `[8:8]`
    #[inline(always)]
    pub const fn sm0(&self) -> bool {
        (self.bits & Irq0Ints::SM0_MASK) != 0
    }

    /// Read **SM3_TXNFULL** `[7:7]`
    #[inline(always)]
    pub const fn sm3_txnfull(&self) -> bool {
        (self.bits & Irq0Ints::SM3_TXNFULL_MASK) != 0
    }

    /// Read **SM2_TXNFULL** `[6:6]`
    #[inline(always)]
    pub const fn sm2_txnfull(&self) -> bool {
        (self.bits & Irq0Ints::SM2_TXNFULL_MASK) != 0
    }

    /// Read **SM1_TXNFULL** `[5:5]`
    #[inline(always)]
    pub const fn sm1_txnfull(&self) -> bool {
        (self.bits & Irq0Ints::SM1_TXNFULL_MASK) != 0
    }

    /// Read **SM0_TXNFULL** `[4:4]`
    #[inline(always)]
    pub const fn sm0_txnfull(&self) -> bool {
        (self.bits & Irq0Ints::SM0_TXNFULL_MASK) != 0
    }

    /// Read **SM3_RXNEMPTY** `[3:3]`
    #[inline(always)]
    pub const fn sm3_rxnempty(&self) -> bool {
        (self.bits & Irq0Ints::SM3_RXNEMPTY_MASK) != 0
    }

    /// Read **SM2_RXNEMPTY** `[2:2]`
    #[inline(always)]
    pub const fn sm2_rxnempty(&self) -> bool {
        (self.bits & Irq0Ints::SM2_RXNEMPTY_MASK) != 0
    }

    /// Read **SM1_RXNEMPTY** `[1:1]`
    #[inline(always)]
    pub const fn sm1_rxnempty(&self) -> bool {
        (self.bits & Irq0Ints::SM1_RXNEMPTY_MASK) != 0
    }

    /// Read **SM0_RXNEMPTY** `[0:0]`
    #[inline(always)]
    pub const fn sm0_rxnempty(&self) -> bool {
        (self.bits & Irq0Ints::SM0_RXNEMPTY_MASK) != 0
    }

}

impl Default for Irq0IntsVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Irq0Ints {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Irq0IntsVal {
        Irq0IntsVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Irq0IntsVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Irq0IntsVal) -> Irq0IntsVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// IRQ1_INTE Register
///
/// Interrupt Enable for irq1
///
/// Address offset: 0x017c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SM7** `[15]` (RW)
///
/// - **SM6** `[14]` (RW)
///
/// - **SM5** `[13]` (RW)
///
/// - **SM4** `[12]` (RW)
///
/// - **SM3** `[11]` (RW)
///
/// - **SM2** `[10]` (RW)
///
/// - **SM1** `[9]` (RW)
///
/// - **SM0** `[8]` (RW)
///
/// - **SM3_TXNFULL** `[7]` (RW)
///
/// - **SM2_TXNFULL** `[6]` (RW)
///
/// - **SM1_TXNFULL** `[5]` (RW)
///
/// - **SM0_TXNFULL** `[4]` (RW)
///
/// - **SM3_RXNEMPTY** `[3]` (RW)
///
/// - **SM2_RXNEMPTY** `[2]` (RW)
///
/// - **SM1_RXNEMPTY** `[1]` (RW)
///
/// - **SM0_RXNEMPTY** `[0]` (RW)
#[repr(transparent)]
pub struct Irq1Inte {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Irq1Inte {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Irq1Inte {
    // ======== Field Constants ========

    /// **SM7**
    /// Bits `[15:15]`, mask `0x00008000`
    pub const SM7_OFFSET: u32 = 15;
    pub const SM7_MASK: u32 = 0x00008000;
    pub const SM7: u32 = 1 << 15;

    /// **SM6**
    /// Bits `[14:14]`, mask `0x00004000`
    pub const SM6_OFFSET: u32 = 14;
    pub const SM6_MASK: u32 = 0x00004000;
    pub const SM6: u32 = 1 << 14;

    /// **SM5**
    /// Bits `[13:13]`, mask `0x00002000`
    pub const SM5_OFFSET: u32 = 13;
    pub const SM5_MASK: u32 = 0x00002000;
    pub const SM5: u32 = 1 << 13;

    /// **SM4**
    /// Bits `[12:12]`, mask `0x00001000`
    pub const SM4_OFFSET: u32 = 12;
    pub const SM4_MASK: u32 = 0x00001000;
    pub const SM4: u32 = 1 << 12;

    /// **SM3**
    /// Bits `[11:11]`, mask `0x00000800`
    pub const SM3_OFFSET: u32 = 11;
    pub const SM3_MASK: u32 = 0x00000800;
    pub const SM3: u32 = 1 << 11;

    /// **SM2**
    /// Bits `[10:10]`, mask `0x00000400`
    pub const SM2_OFFSET: u32 = 10;
    pub const SM2_MASK: u32 = 0x00000400;
    pub const SM2: u32 = 1 << 10;

    /// **SM1**
    /// Bits `[9:9]`, mask `0x00000200`
    pub const SM1_OFFSET: u32 = 9;
    pub const SM1_MASK: u32 = 0x00000200;
    pub const SM1: u32 = 1 << 9;

    /// **SM0**
    /// Bits `[8:8]`, mask `0x00000100`
    pub const SM0_OFFSET: u32 = 8;
    pub const SM0_MASK: u32 = 0x00000100;
    pub const SM0: u32 = 1 << 8;

    /// **SM3_TXNFULL**
    /// Bits `[7:7]`, mask `0x00000080`
    pub const SM3_TXNFULL_OFFSET: u32 = 7;
    pub const SM3_TXNFULL_MASK: u32 = 0x00000080;
    pub const SM3_TXNFULL: u32 = 1 << 7;

    /// **SM2_TXNFULL**
    /// Bits `[6:6]`, mask `0x00000040`
    pub const SM2_TXNFULL_OFFSET: u32 = 6;
    pub const SM2_TXNFULL_MASK: u32 = 0x00000040;
    pub const SM2_TXNFULL: u32 = 1 << 6;

    /// **SM1_TXNFULL**
    /// Bits `[5:5]`, mask `0x00000020`
    pub const SM1_TXNFULL_OFFSET: u32 = 5;
    pub const SM1_TXNFULL_MASK: u32 = 0x00000020;
    pub const SM1_TXNFULL: u32 = 1 << 5;

    /// **SM0_TXNFULL**
    /// Bits `[4:4]`, mask `0x00000010`
    pub const SM0_TXNFULL_OFFSET: u32 = 4;
    pub const SM0_TXNFULL_MASK: u32 = 0x00000010;
    pub const SM0_TXNFULL: u32 = 1 << 4;

    /// **SM3_RXNEMPTY**
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SM3_RXNEMPTY_OFFSET: u32 = 3;
    pub const SM3_RXNEMPTY_MASK: u32 = 0x00000008;
    pub const SM3_RXNEMPTY: u32 = 1 << 3;

    /// **SM2_RXNEMPTY**
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SM2_RXNEMPTY_OFFSET: u32 = 2;
    pub const SM2_RXNEMPTY_MASK: u32 = 0x00000004;
    pub const SM2_RXNEMPTY: u32 = 1 << 2;

    /// **SM1_RXNEMPTY**
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SM1_RXNEMPTY_OFFSET: u32 = 1;
    pub const SM1_RXNEMPTY_MASK: u32 = 0x00000002;
    pub const SM1_RXNEMPTY: u32 = 1 << 1;

    /// **SM0_RXNEMPTY**
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SM0_RXNEMPTY_OFFSET: u32 = 0;
    pub const SM0_RXNEMPTY_MASK: u32 = 0x00000001;
    pub const SM0_RXNEMPTY: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **SM7** `[15]` (RW)
    #[inline(always)]
    pub fn sm7(&self) -> bool {
        (self.value.read_bits() & Self::SM7_MASK) != 0
    }

    /// Set SM7 bit atomically (RW)
    #[inline(always)]
    pub fn set_sm7(&self) {
        self.set_bits(Self::SM7_MASK);
    }

    /// Clear SM7 bit atomically (RW)
    #[inline(always)]
    pub fn clr_sm7(&self) {
        self.clr_bits(Self::SM7_MASK);
    }

    /// **SM6** `[14]` (RW)
    #[inline(always)]
    pub fn sm6(&self) -> bool {
        (self.value.read_bits() & Self::SM6_MASK) != 0
    }

    /// Set SM6 bit atomically (RW)
    #[inline(always)]
    pub fn set_sm6(&self) {
        self.set_bits(Self::SM6_MASK);
    }

    /// Clear SM6 bit atomically (RW)
    #[inline(always)]
    pub fn clr_sm6(&self) {
        self.clr_bits(Self::SM6_MASK);
    }

    /// **SM5** `[13]` (RW)
    #[inline(always)]
    pub fn sm5(&self) -> bool {
        (self.value.read_bits() & Self::SM5_MASK) != 0
    }

    /// Set SM5 bit atomically (RW)
    #[inline(always)]
    pub fn set_sm5(&self) {
        self.set_bits(Self::SM5_MASK);
    }

    /// Clear SM5 bit atomically (RW)
    #[inline(always)]
    pub fn clr_sm5(&self) {
        self.clr_bits(Self::SM5_MASK);
    }

    /// **SM4** `[12]` (RW)
    #[inline(always)]
    pub fn sm4(&self) -> bool {
        (self.value.read_bits() & Self::SM4_MASK) != 0
    }

    /// Set SM4 bit atomically (RW)
    #[inline(always)]
    pub fn set_sm4(&self) {
        self.set_bits(Self::SM4_MASK);
    }

    /// Clear SM4 bit atomically (RW)
    #[inline(always)]
    pub fn clr_sm4(&self) {
        self.clr_bits(Self::SM4_MASK);
    }

    /// **SM3** `[11]` (RW)
    #[inline(always)]
    pub fn sm3(&self) -> bool {
        (self.value.read_bits() & Self::SM3_MASK) != 0
    }

    /// Set SM3 bit atomically (RW)
    #[inline(always)]
    pub fn set_sm3(&self) {
        self.set_bits(Self::SM3_MASK);
    }

    /// Clear SM3 bit atomically (RW)
    #[inline(always)]
    pub fn clr_sm3(&self) {
        self.clr_bits(Self::SM3_MASK);
    }

    /// **SM2** `[10]` (RW)
    #[inline(always)]
    pub fn sm2(&self) -> bool {
        (self.value.read_bits() & Self::SM2_MASK) != 0
    }

    /// Set SM2 bit atomically (RW)
    #[inline(always)]
    pub fn set_sm2(&self) {
        self.set_bits(Self::SM2_MASK);
    }

    /// Clear SM2 bit atomically (RW)
    #[inline(always)]
    pub fn clr_sm2(&self) {
        self.clr_bits(Self::SM2_MASK);
    }

    /// **SM1** `[9]` (RW)
    #[inline(always)]
    pub fn sm1(&self) -> bool {
        (self.value.read_bits() & Self::SM1_MASK) != 0
    }

    /// Set SM1 bit atomically (RW)
    #[inline(always)]
    pub fn set_sm1(&self) {
        self.set_bits(Self::SM1_MASK);
    }

    /// Clear SM1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_sm1(&self) {
        self.clr_bits(Self::SM1_MASK);
    }

    /// **SM0** `[8]` (RW)
    #[inline(always)]
    pub fn sm0(&self) -> bool {
        (self.value.read_bits() & Self::SM0_MASK) != 0
    }

    /// Set SM0 bit atomically (RW)
    #[inline(always)]
    pub fn set_sm0(&self) {
        self.set_bits(Self::SM0_MASK);
    }

    /// Clear SM0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_sm0(&self) {
        self.clr_bits(Self::SM0_MASK);
    }

    /// **SM3_TXNFULL** `[7]` (RW)
    #[inline(always)]
    pub fn sm3_txnfull(&self) -> bool {
        (self.value.read_bits() & Self::SM3_TXNFULL_MASK) != 0
    }

    /// Set SM3_TXNFULL bit atomically (RW)
    #[inline(always)]
    pub fn set_sm3_txnfull(&self) {
        self.set_bits(Self::SM3_TXNFULL_MASK);
    }

    /// Clear SM3_TXNFULL bit atomically (RW)
    #[inline(always)]
    pub fn clr_sm3_txnfull(&self) {
        self.clr_bits(Self::SM3_TXNFULL_MASK);
    }

    /// **SM2_TXNFULL** `[6]` (RW)
    #[inline(always)]
    pub fn sm2_txnfull(&self) -> bool {
        (self.value.read_bits() & Self::SM2_TXNFULL_MASK) != 0
    }

    /// Set SM2_TXNFULL bit atomically (RW)
    #[inline(always)]
    pub fn set_sm2_txnfull(&self) {
        self.set_bits(Self::SM2_TXNFULL_MASK);
    }

    /// Clear SM2_TXNFULL bit atomically (RW)
    #[inline(always)]
    pub fn clr_sm2_txnfull(&self) {
        self.clr_bits(Self::SM2_TXNFULL_MASK);
    }

    /// **SM1_TXNFULL** `[5]` (RW)
    #[inline(always)]
    pub fn sm1_txnfull(&self) -> bool {
        (self.value.read_bits() & Self::SM1_TXNFULL_MASK) != 0
    }

    /// Set SM1_TXNFULL bit atomically (RW)
    #[inline(always)]
    pub fn set_sm1_txnfull(&self) {
        self.set_bits(Self::SM1_TXNFULL_MASK);
    }

    /// Clear SM1_TXNFULL bit atomically (RW)
    #[inline(always)]
    pub fn clr_sm1_txnfull(&self) {
        self.clr_bits(Self::SM1_TXNFULL_MASK);
    }

    /// **SM0_TXNFULL** `[4]` (RW)
    #[inline(always)]
    pub fn sm0_txnfull(&self) -> bool {
        (self.value.read_bits() & Self::SM0_TXNFULL_MASK) != 0
    }

    /// Set SM0_TXNFULL bit atomically (RW)
    #[inline(always)]
    pub fn set_sm0_txnfull(&self) {
        self.set_bits(Self::SM0_TXNFULL_MASK);
    }

    /// Clear SM0_TXNFULL bit atomically (RW)
    #[inline(always)]
    pub fn clr_sm0_txnfull(&self) {
        self.clr_bits(Self::SM0_TXNFULL_MASK);
    }

    /// **SM3_RXNEMPTY** `[3]` (RW)
    #[inline(always)]
    pub fn sm3_rxnempty(&self) -> bool {
        (self.value.read_bits() & Self::SM3_RXNEMPTY_MASK) != 0
    }

    /// Set SM3_RXNEMPTY bit atomically (RW)
    #[inline(always)]
    pub fn set_sm3_rxnempty(&self) {
        self.set_bits(Self::SM3_RXNEMPTY_MASK);
    }

    /// Clear SM3_RXNEMPTY bit atomically (RW)
    #[inline(always)]
    pub fn clr_sm3_rxnempty(&self) {
        self.clr_bits(Self::SM3_RXNEMPTY_MASK);
    }

    /// **SM2_RXNEMPTY** `[2]` (RW)
    #[inline(always)]
    pub fn sm2_rxnempty(&self) -> bool {
        (self.value.read_bits() & Self::SM2_RXNEMPTY_MASK) != 0
    }

    /// Set SM2_RXNEMPTY bit atomically (RW)
    #[inline(always)]
    pub fn set_sm2_rxnempty(&self) {
        self.set_bits(Self::SM2_RXNEMPTY_MASK);
    }

    /// Clear SM2_RXNEMPTY bit atomically (RW)
    #[inline(always)]
    pub fn clr_sm2_rxnempty(&self) {
        self.clr_bits(Self::SM2_RXNEMPTY_MASK);
    }

    /// **SM1_RXNEMPTY** `[1]` (RW)
    #[inline(always)]
    pub fn sm1_rxnempty(&self) -> bool {
        (self.value.read_bits() & Self::SM1_RXNEMPTY_MASK) != 0
    }

    /// Set SM1_RXNEMPTY bit atomically (RW)
    #[inline(always)]
    pub fn set_sm1_rxnempty(&self) {
        self.set_bits(Self::SM1_RXNEMPTY_MASK);
    }

    /// Clear SM1_RXNEMPTY bit atomically (RW)
    #[inline(always)]
    pub fn clr_sm1_rxnempty(&self) {
        self.clr_bits(Self::SM1_RXNEMPTY_MASK);
    }

    /// **SM0_RXNEMPTY** `[0]` (RW)
    #[inline(always)]
    pub fn sm0_rxnempty(&self) -> bool {
        (self.value.read_bits() & Self::SM0_RXNEMPTY_MASK) != 0
    }

    /// Set SM0_RXNEMPTY bit atomically (RW)
    #[inline(always)]
    pub fn set_sm0_rxnempty(&self) {
        self.set_bits(Self::SM0_RXNEMPTY_MASK);
    }

    /// Clear SM0_RXNEMPTY bit atomically (RW)
    #[inline(always)]
    pub fn clr_sm0_rxnempty(&self) {
        self.clr_bits(Self::SM0_RXNEMPTY_MASK);
    }

}

/// IRQ1_INTE value type for batch read/modify/write operations
///
/// Use `irq1inte.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `irq1inte.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Irq1InteVal {
    bits: u32,
}

impl Irq1InteVal {
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

    /// Read **SM7** `[15:15]`
    #[inline(always)]
    pub const fn sm7(&self) -> bool {
        (self.bits & Irq1Inte::SM7_MASK) != 0
    }

    /// Read **SM6** `[14:14]`
    #[inline(always)]
    pub const fn sm6(&self) -> bool {
        (self.bits & Irq1Inte::SM6_MASK) != 0
    }

    /// Read **SM5** `[13:13]`
    #[inline(always)]
    pub const fn sm5(&self) -> bool {
        (self.bits & Irq1Inte::SM5_MASK) != 0
    }

    /// Read **SM4** `[12:12]`
    #[inline(always)]
    pub const fn sm4(&self) -> bool {
        (self.bits & Irq1Inte::SM4_MASK) != 0
    }

    /// Read **SM3** `[11:11]`
    #[inline(always)]
    pub const fn sm3(&self) -> bool {
        (self.bits & Irq1Inte::SM3_MASK) != 0
    }

    /// Read **SM2** `[10:10]`
    #[inline(always)]
    pub const fn sm2(&self) -> bool {
        (self.bits & Irq1Inte::SM2_MASK) != 0
    }

    /// Read **SM1** `[9:9]`
    #[inline(always)]
    pub const fn sm1(&self) -> bool {
        (self.bits & Irq1Inte::SM1_MASK) != 0
    }

    /// Read **SM0** `[8:8]`
    #[inline(always)]
    pub const fn sm0(&self) -> bool {
        (self.bits & Irq1Inte::SM0_MASK) != 0
    }

    /// Read **SM3_TXNFULL** `[7:7]`
    #[inline(always)]
    pub const fn sm3_txnfull(&self) -> bool {
        (self.bits & Irq1Inte::SM3_TXNFULL_MASK) != 0
    }

    /// Read **SM2_TXNFULL** `[6:6]`
    #[inline(always)]
    pub const fn sm2_txnfull(&self) -> bool {
        (self.bits & Irq1Inte::SM2_TXNFULL_MASK) != 0
    }

    /// Read **SM1_TXNFULL** `[5:5]`
    #[inline(always)]
    pub const fn sm1_txnfull(&self) -> bool {
        (self.bits & Irq1Inte::SM1_TXNFULL_MASK) != 0
    }

    /// Read **SM0_TXNFULL** `[4:4]`
    #[inline(always)]
    pub const fn sm0_txnfull(&self) -> bool {
        (self.bits & Irq1Inte::SM0_TXNFULL_MASK) != 0
    }

    /// Read **SM3_RXNEMPTY** `[3:3]`
    #[inline(always)]
    pub const fn sm3_rxnempty(&self) -> bool {
        (self.bits & Irq1Inte::SM3_RXNEMPTY_MASK) != 0
    }

    /// Read **SM2_RXNEMPTY** `[2:2]`
    #[inline(always)]
    pub const fn sm2_rxnempty(&self) -> bool {
        (self.bits & Irq1Inte::SM2_RXNEMPTY_MASK) != 0
    }

    /// Read **SM1_RXNEMPTY** `[1:1]`
    #[inline(always)]
    pub const fn sm1_rxnempty(&self) -> bool {
        (self.bits & Irq1Inte::SM1_RXNEMPTY_MASK) != 0
    }

    /// Read **SM0_RXNEMPTY** `[0:0]`
    #[inline(always)]
    pub const fn sm0_rxnempty(&self) -> bool {
        (self.bits & Irq1Inte::SM0_RXNEMPTY_MASK) != 0
    }

    /// Set **SM7** `[15:15]`
    #[inline(always)]
    pub const fn with_sm7(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq1Inte::SM7_MASK
            } else {
                self.bits & !Irq1Inte::SM7_MASK
            }
        }
    }

    /// Set **SM6** `[14:14]`
    #[inline(always)]
    pub const fn with_sm6(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq1Inte::SM6_MASK
            } else {
                self.bits & !Irq1Inte::SM6_MASK
            }
        }
    }

    /// Set **SM5** `[13:13]`
    #[inline(always)]
    pub const fn with_sm5(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq1Inte::SM5_MASK
            } else {
                self.bits & !Irq1Inte::SM5_MASK
            }
        }
    }

    /// Set **SM4** `[12:12]`
    #[inline(always)]
    pub const fn with_sm4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq1Inte::SM4_MASK
            } else {
                self.bits & !Irq1Inte::SM4_MASK
            }
        }
    }

    /// Set **SM3** `[11:11]`
    #[inline(always)]
    pub const fn with_sm3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq1Inte::SM3_MASK
            } else {
                self.bits & !Irq1Inte::SM3_MASK
            }
        }
    }

    /// Set **SM2** `[10:10]`
    #[inline(always)]
    pub const fn with_sm2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq1Inte::SM2_MASK
            } else {
                self.bits & !Irq1Inte::SM2_MASK
            }
        }
    }

    /// Set **SM1** `[9:9]`
    #[inline(always)]
    pub const fn with_sm1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq1Inte::SM1_MASK
            } else {
                self.bits & !Irq1Inte::SM1_MASK
            }
        }
    }

    /// Set **SM0** `[8:8]`
    #[inline(always)]
    pub const fn with_sm0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq1Inte::SM0_MASK
            } else {
                self.bits & !Irq1Inte::SM0_MASK
            }
        }
    }

    /// Set **SM3_TXNFULL** `[7:7]`
    #[inline(always)]
    pub const fn with_sm3_txnfull(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq1Inte::SM3_TXNFULL_MASK
            } else {
                self.bits & !Irq1Inte::SM3_TXNFULL_MASK
            }
        }
    }

    /// Set **SM2_TXNFULL** `[6:6]`
    #[inline(always)]
    pub const fn with_sm2_txnfull(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq1Inte::SM2_TXNFULL_MASK
            } else {
                self.bits & !Irq1Inte::SM2_TXNFULL_MASK
            }
        }
    }

    /// Set **SM1_TXNFULL** `[5:5]`
    #[inline(always)]
    pub const fn with_sm1_txnfull(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq1Inte::SM1_TXNFULL_MASK
            } else {
                self.bits & !Irq1Inte::SM1_TXNFULL_MASK
            }
        }
    }

    /// Set **SM0_TXNFULL** `[4:4]`
    #[inline(always)]
    pub const fn with_sm0_txnfull(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq1Inte::SM0_TXNFULL_MASK
            } else {
                self.bits & !Irq1Inte::SM0_TXNFULL_MASK
            }
        }
    }

    /// Set **SM3_RXNEMPTY** `[3:3]`
    #[inline(always)]
    pub const fn with_sm3_rxnempty(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq1Inte::SM3_RXNEMPTY_MASK
            } else {
                self.bits & !Irq1Inte::SM3_RXNEMPTY_MASK
            }
        }
    }

    /// Set **SM2_RXNEMPTY** `[2:2]`
    #[inline(always)]
    pub const fn with_sm2_rxnempty(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq1Inte::SM2_RXNEMPTY_MASK
            } else {
                self.bits & !Irq1Inte::SM2_RXNEMPTY_MASK
            }
        }
    }

    /// Set **SM1_RXNEMPTY** `[1:1]`
    #[inline(always)]
    pub const fn with_sm1_rxnempty(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq1Inte::SM1_RXNEMPTY_MASK
            } else {
                self.bits & !Irq1Inte::SM1_RXNEMPTY_MASK
            }
        }
    }

    /// Set **SM0_RXNEMPTY** `[0:0]`
    #[inline(always)]
    pub const fn with_sm0_rxnempty(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq1Inte::SM0_RXNEMPTY_MASK
            } else {
                self.bits & !Irq1Inte::SM0_RXNEMPTY_MASK
            }
        }
    }

}

impl Default for Irq1InteVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Irq1Inte {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Irq1InteVal {
        Irq1InteVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Irq1InteVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Irq1InteVal) -> Irq1InteVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// IRQ1_INTF Register
///
/// Interrupt Force for irq1
///
/// Address offset: 0x0180
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SM7** `[15]` (RW)
///
/// - **SM6** `[14]` (RW)
///
/// - **SM5** `[13]` (RW)
///
/// - **SM4** `[12]` (RW)
///
/// - **SM3** `[11]` (RW)
///
/// - **SM2** `[10]` (RW)
///
/// - **SM1** `[9]` (RW)
///
/// - **SM0** `[8]` (RW)
///
/// - **SM3_TXNFULL** `[7]` (RW)
///
/// - **SM2_TXNFULL** `[6]` (RW)
///
/// - **SM1_TXNFULL** `[5]` (RW)
///
/// - **SM0_TXNFULL** `[4]` (RW)
///
/// - **SM3_RXNEMPTY** `[3]` (RW)
///
/// - **SM2_RXNEMPTY** `[2]` (RW)
///
/// - **SM1_RXNEMPTY** `[1]` (RW)
///
/// - **SM0_RXNEMPTY** `[0]` (RW)
#[repr(transparent)]
pub struct Irq1Intf {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Irq1Intf {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Irq1Intf {
    // ======== Field Constants ========

    /// **SM7**
    /// Bits `[15:15]`, mask `0x00008000`
    pub const SM7_OFFSET: u32 = 15;
    pub const SM7_MASK: u32 = 0x00008000;
    pub const SM7: u32 = 1 << 15;

    /// **SM6**
    /// Bits `[14:14]`, mask `0x00004000`
    pub const SM6_OFFSET: u32 = 14;
    pub const SM6_MASK: u32 = 0x00004000;
    pub const SM6: u32 = 1 << 14;

    /// **SM5**
    /// Bits `[13:13]`, mask `0x00002000`
    pub const SM5_OFFSET: u32 = 13;
    pub const SM5_MASK: u32 = 0x00002000;
    pub const SM5: u32 = 1 << 13;

    /// **SM4**
    /// Bits `[12:12]`, mask `0x00001000`
    pub const SM4_OFFSET: u32 = 12;
    pub const SM4_MASK: u32 = 0x00001000;
    pub const SM4: u32 = 1 << 12;

    /// **SM3**
    /// Bits `[11:11]`, mask `0x00000800`
    pub const SM3_OFFSET: u32 = 11;
    pub const SM3_MASK: u32 = 0x00000800;
    pub const SM3: u32 = 1 << 11;

    /// **SM2**
    /// Bits `[10:10]`, mask `0x00000400`
    pub const SM2_OFFSET: u32 = 10;
    pub const SM2_MASK: u32 = 0x00000400;
    pub const SM2: u32 = 1 << 10;

    /// **SM1**
    /// Bits `[9:9]`, mask `0x00000200`
    pub const SM1_OFFSET: u32 = 9;
    pub const SM1_MASK: u32 = 0x00000200;
    pub const SM1: u32 = 1 << 9;

    /// **SM0**
    /// Bits `[8:8]`, mask `0x00000100`
    pub const SM0_OFFSET: u32 = 8;
    pub const SM0_MASK: u32 = 0x00000100;
    pub const SM0: u32 = 1 << 8;

    /// **SM3_TXNFULL**
    /// Bits `[7:7]`, mask `0x00000080`
    pub const SM3_TXNFULL_OFFSET: u32 = 7;
    pub const SM3_TXNFULL_MASK: u32 = 0x00000080;
    pub const SM3_TXNFULL: u32 = 1 << 7;

    /// **SM2_TXNFULL**
    /// Bits `[6:6]`, mask `0x00000040`
    pub const SM2_TXNFULL_OFFSET: u32 = 6;
    pub const SM2_TXNFULL_MASK: u32 = 0x00000040;
    pub const SM2_TXNFULL: u32 = 1 << 6;

    /// **SM1_TXNFULL**
    /// Bits `[5:5]`, mask `0x00000020`
    pub const SM1_TXNFULL_OFFSET: u32 = 5;
    pub const SM1_TXNFULL_MASK: u32 = 0x00000020;
    pub const SM1_TXNFULL: u32 = 1 << 5;

    /// **SM0_TXNFULL**
    /// Bits `[4:4]`, mask `0x00000010`
    pub const SM0_TXNFULL_OFFSET: u32 = 4;
    pub const SM0_TXNFULL_MASK: u32 = 0x00000010;
    pub const SM0_TXNFULL: u32 = 1 << 4;

    /// **SM3_RXNEMPTY**
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SM3_RXNEMPTY_OFFSET: u32 = 3;
    pub const SM3_RXNEMPTY_MASK: u32 = 0x00000008;
    pub const SM3_RXNEMPTY: u32 = 1 << 3;

    /// **SM2_RXNEMPTY**
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SM2_RXNEMPTY_OFFSET: u32 = 2;
    pub const SM2_RXNEMPTY_MASK: u32 = 0x00000004;
    pub const SM2_RXNEMPTY: u32 = 1 << 2;

    /// **SM1_RXNEMPTY**
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SM1_RXNEMPTY_OFFSET: u32 = 1;
    pub const SM1_RXNEMPTY_MASK: u32 = 0x00000002;
    pub const SM1_RXNEMPTY: u32 = 1 << 1;

    /// **SM0_RXNEMPTY**
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SM0_RXNEMPTY_OFFSET: u32 = 0;
    pub const SM0_RXNEMPTY_MASK: u32 = 0x00000001;
    pub const SM0_RXNEMPTY: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **SM7** `[15]` (RW)
    #[inline(always)]
    pub fn sm7(&self) -> bool {
        (self.value.read_bits() & Self::SM7_MASK) != 0
    }

    /// Set SM7 bit atomically (RW)
    #[inline(always)]
    pub fn set_sm7(&self) {
        self.set_bits(Self::SM7_MASK);
    }

    /// Clear SM7 bit atomically (RW)
    #[inline(always)]
    pub fn clr_sm7(&self) {
        self.clr_bits(Self::SM7_MASK);
    }

    /// **SM6** `[14]` (RW)
    #[inline(always)]
    pub fn sm6(&self) -> bool {
        (self.value.read_bits() & Self::SM6_MASK) != 0
    }

    /// Set SM6 bit atomically (RW)
    #[inline(always)]
    pub fn set_sm6(&self) {
        self.set_bits(Self::SM6_MASK);
    }

    /// Clear SM6 bit atomically (RW)
    #[inline(always)]
    pub fn clr_sm6(&self) {
        self.clr_bits(Self::SM6_MASK);
    }

    /// **SM5** `[13]` (RW)
    #[inline(always)]
    pub fn sm5(&self) -> bool {
        (self.value.read_bits() & Self::SM5_MASK) != 0
    }

    /// Set SM5 bit atomically (RW)
    #[inline(always)]
    pub fn set_sm5(&self) {
        self.set_bits(Self::SM5_MASK);
    }

    /// Clear SM5 bit atomically (RW)
    #[inline(always)]
    pub fn clr_sm5(&self) {
        self.clr_bits(Self::SM5_MASK);
    }

    /// **SM4** `[12]` (RW)
    #[inline(always)]
    pub fn sm4(&self) -> bool {
        (self.value.read_bits() & Self::SM4_MASK) != 0
    }

    /// Set SM4 bit atomically (RW)
    #[inline(always)]
    pub fn set_sm4(&self) {
        self.set_bits(Self::SM4_MASK);
    }

    /// Clear SM4 bit atomically (RW)
    #[inline(always)]
    pub fn clr_sm4(&self) {
        self.clr_bits(Self::SM4_MASK);
    }

    /// **SM3** `[11]` (RW)
    #[inline(always)]
    pub fn sm3(&self) -> bool {
        (self.value.read_bits() & Self::SM3_MASK) != 0
    }

    /// Set SM3 bit atomically (RW)
    #[inline(always)]
    pub fn set_sm3(&self) {
        self.set_bits(Self::SM3_MASK);
    }

    /// Clear SM3 bit atomically (RW)
    #[inline(always)]
    pub fn clr_sm3(&self) {
        self.clr_bits(Self::SM3_MASK);
    }

    /// **SM2** `[10]` (RW)
    #[inline(always)]
    pub fn sm2(&self) -> bool {
        (self.value.read_bits() & Self::SM2_MASK) != 0
    }

    /// Set SM2 bit atomically (RW)
    #[inline(always)]
    pub fn set_sm2(&self) {
        self.set_bits(Self::SM2_MASK);
    }

    /// Clear SM2 bit atomically (RW)
    #[inline(always)]
    pub fn clr_sm2(&self) {
        self.clr_bits(Self::SM2_MASK);
    }

    /// **SM1** `[9]` (RW)
    #[inline(always)]
    pub fn sm1(&self) -> bool {
        (self.value.read_bits() & Self::SM1_MASK) != 0
    }

    /// Set SM1 bit atomically (RW)
    #[inline(always)]
    pub fn set_sm1(&self) {
        self.set_bits(Self::SM1_MASK);
    }

    /// Clear SM1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_sm1(&self) {
        self.clr_bits(Self::SM1_MASK);
    }

    /// **SM0** `[8]` (RW)
    #[inline(always)]
    pub fn sm0(&self) -> bool {
        (self.value.read_bits() & Self::SM0_MASK) != 0
    }

    /// Set SM0 bit atomically (RW)
    #[inline(always)]
    pub fn set_sm0(&self) {
        self.set_bits(Self::SM0_MASK);
    }

    /// Clear SM0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_sm0(&self) {
        self.clr_bits(Self::SM0_MASK);
    }

    /// **SM3_TXNFULL** `[7]` (RW)
    #[inline(always)]
    pub fn sm3_txnfull(&self) -> bool {
        (self.value.read_bits() & Self::SM3_TXNFULL_MASK) != 0
    }

    /// Set SM3_TXNFULL bit atomically (RW)
    #[inline(always)]
    pub fn set_sm3_txnfull(&self) {
        self.set_bits(Self::SM3_TXNFULL_MASK);
    }

    /// Clear SM3_TXNFULL bit atomically (RW)
    #[inline(always)]
    pub fn clr_sm3_txnfull(&self) {
        self.clr_bits(Self::SM3_TXNFULL_MASK);
    }

    /// **SM2_TXNFULL** `[6]` (RW)
    #[inline(always)]
    pub fn sm2_txnfull(&self) -> bool {
        (self.value.read_bits() & Self::SM2_TXNFULL_MASK) != 0
    }

    /// Set SM2_TXNFULL bit atomically (RW)
    #[inline(always)]
    pub fn set_sm2_txnfull(&self) {
        self.set_bits(Self::SM2_TXNFULL_MASK);
    }

    /// Clear SM2_TXNFULL bit atomically (RW)
    #[inline(always)]
    pub fn clr_sm2_txnfull(&self) {
        self.clr_bits(Self::SM2_TXNFULL_MASK);
    }

    /// **SM1_TXNFULL** `[5]` (RW)
    #[inline(always)]
    pub fn sm1_txnfull(&self) -> bool {
        (self.value.read_bits() & Self::SM1_TXNFULL_MASK) != 0
    }

    /// Set SM1_TXNFULL bit atomically (RW)
    #[inline(always)]
    pub fn set_sm1_txnfull(&self) {
        self.set_bits(Self::SM1_TXNFULL_MASK);
    }

    /// Clear SM1_TXNFULL bit atomically (RW)
    #[inline(always)]
    pub fn clr_sm1_txnfull(&self) {
        self.clr_bits(Self::SM1_TXNFULL_MASK);
    }

    /// **SM0_TXNFULL** `[4]` (RW)
    #[inline(always)]
    pub fn sm0_txnfull(&self) -> bool {
        (self.value.read_bits() & Self::SM0_TXNFULL_MASK) != 0
    }

    /// Set SM0_TXNFULL bit atomically (RW)
    #[inline(always)]
    pub fn set_sm0_txnfull(&self) {
        self.set_bits(Self::SM0_TXNFULL_MASK);
    }

    /// Clear SM0_TXNFULL bit atomically (RW)
    #[inline(always)]
    pub fn clr_sm0_txnfull(&self) {
        self.clr_bits(Self::SM0_TXNFULL_MASK);
    }

    /// **SM3_RXNEMPTY** `[3]` (RW)
    #[inline(always)]
    pub fn sm3_rxnempty(&self) -> bool {
        (self.value.read_bits() & Self::SM3_RXNEMPTY_MASK) != 0
    }

    /// Set SM3_RXNEMPTY bit atomically (RW)
    #[inline(always)]
    pub fn set_sm3_rxnempty(&self) {
        self.set_bits(Self::SM3_RXNEMPTY_MASK);
    }

    /// Clear SM3_RXNEMPTY bit atomically (RW)
    #[inline(always)]
    pub fn clr_sm3_rxnempty(&self) {
        self.clr_bits(Self::SM3_RXNEMPTY_MASK);
    }

    /// **SM2_RXNEMPTY** `[2]` (RW)
    #[inline(always)]
    pub fn sm2_rxnempty(&self) -> bool {
        (self.value.read_bits() & Self::SM2_RXNEMPTY_MASK) != 0
    }

    /// Set SM2_RXNEMPTY bit atomically (RW)
    #[inline(always)]
    pub fn set_sm2_rxnempty(&self) {
        self.set_bits(Self::SM2_RXNEMPTY_MASK);
    }

    /// Clear SM2_RXNEMPTY bit atomically (RW)
    #[inline(always)]
    pub fn clr_sm2_rxnempty(&self) {
        self.clr_bits(Self::SM2_RXNEMPTY_MASK);
    }

    /// **SM1_RXNEMPTY** `[1]` (RW)
    #[inline(always)]
    pub fn sm1_rxnempty(&self) -> bool {
        (self.value.read_bits() & Self::SM1_RXNEMPTY_MASK) != 0
    }

    /// Set SM1_RXNEMPTY bit atomically (RW)
    #[inline(always)]
    pub fn set_sm1_rxnempty(&self) {
        self.set_bits(Self::SM1_RXNEMPTY_MASK);
    }

    /// Clear SM1_RXNEMPTY bit atomically (RW)
    #[inline(always)]
    pub fn clr_sm1_rxnempty(&self) {
        self.clr_bits(Self::SM1_RXNEMPTY_MASK);
    }

    /// **SM0_RXNEMPTY** `[0]` (RW)
    #[inline(always)]
    pub fn sm0_rxnempty(&self) -> bool {
        (self.value.read_bits() & Self::SM0_RXNEMPTY_MASK) != 0
    }

    /// Set SM0_RXNEMPTY bit atomically (RW)
    #[inline(always)]
    pub fn set_sm0_rxnempty(&self) {
        self.set_bits(Self::SM0_RXNEMPTY_MASK);
    }

    /// Clear SM0_RXNEMPTY bit atomically (RW)
    #[inline(always)]
    pub fn clr_sm0_rxnempty(&self) {
        self.clr_bits(Self::SM0_RXNEMPTY_MASK);
    }

}

/// IRQ1_INTF value type for batch read/modify/write operations
///
/// Use `irq1intf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `irq1intf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Irq1IntfVal {
    bits: u32,
}

impl Irq1IntfVal {
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

    /// Read **SM7** `[15:15]`
    #[inline(always)]
    pub const fn sm7(&self) -> bool {
        (self.bits & Irq1Intf::SM7_MASK) != 0
    }

    /// Read **SM6** `[14:14]`
    #[inline(always)]
    pub const fn sm6(&self) -> bool {
        (self.bits & Irq1Intf::SM6_MASK) != 0
    }

    /// Read **SM5** `[13:13]`
    #[inline(always)]
    pub const fn sm5(&self) -> bool {
        (self.bits & Irq1Intf::SM5_MASK) != 0
    }

    /// Read **SM4** `[12:12]`
    #[inline(always)]
    pub const fn sm4(&self) -> bool {
        (self.bits & Irq1Intf::SM4_MASK) != 0
    }

    /// Read **SM3** `[11:11]`
    #[inline(always)]
    pub const fn sm3(&self) -> bool {
        (self.bits & Irq1Intf::SM3_MASK) != 0
    }

    /// Read **SM2** `[10:10]`
    #[inline(always)]
    pub const fn sm2(&self) -> bool {
        (self.bits & Irq1Intf::SM2_MASK) != 0
    }

    /// Read **SM1** `[9:9]`
    #[inline(always)]
    pub const fn sm1(&self) -> bool {
        (self.bits & Irq1Intf::SM1_MASK) != 0
    }

    /// Read **SM0** `[8:8]`
    #[inline(always)]
    pub const fn sm0(&self) -> bool {
        (self.bits & Irq1Intf::SM0_MASK) != 0
    }

    /// Read **SM3_TXNFULL** `[7:7]`
    #[inline(always)]
    pub const fn sm3_txnfull(&self) -> bool {
        (self.bits & Irq1Intf::SM3_TXNFULL_MASK) != 0
    }

    /// Read **SM2_TXNFULL** `[6:6]`
    #[inline(always)]
    pub const fn sm2_txnfull(&self) -> bool {
        (self.bits & Irq1Intf::SM2_TXNFULL_MASK) != 0
    }

    /// Read **SM1_TXNFULL** `[5:5]`
    #[inline(always)]
    pub const fn sm1_txnfull(&self) -> bool {
        (self.bits & Irq1Intf::SM1_TXNFULL_MASK) != 0
    }

    /// Read **SM0_TXNFULL** `[4:4]`
    #[inline(always)]
    pub const fn sm0_txnfull(&self) -> bool {
        (self.bits & Irq1Intf::SM0_TXNFULL_MASK) != 0
    }

    /// Read **SM3_RXNEMPTY** `[3:3]`
    #[inline(always)]
    pub const fn sm3_rxnempty(&self) -> bool {
        (self.bits & Irq1Intf::SM3_RXNEMPTY_MASK) != 0
    }

    /// Read **SM2_RXNEMPTY** `[2:2]`
    #[inline(always)]
    pub const fn sm2_rxnempty(&self) -> bool {
        (self.bits & Irq1Intf::SM2_RXNEMPTY_MASK) != 0
    }

    /// Read **SM1_RXNEMPTY** `[1:1]`
    #[inline(always)]
    pub const fn sm1_rxnempty(&self) -> bool {
        (self.bits & Irq1Intf::SM1_RXNEMPTY_MASK) != 0
    }

    /// Read **SM0_RXNEMPTY** `[0:0]`
    #[inline(always)]
    pub const fn sm0_rxnempty(&self) -> bool {
        (self.bits & Irq1Intf::SM0_RXNEMPTY_MASK) != 0
    }

    /// Set **SM7** `[15:15]`
    #[inline(always)]
    pub const fn with_sm7(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq1Intf::SM7_MASK
            } else {
                self.bits & !Irq1Intf::SM7_MASK
            }
        }
    }

    /// Set **SM6** `[14:14]`
    #[inline(always)]
    pub const fn with_sm6(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq1Intf::SM6_MASK
            } else {
                self.bits & !Irq1Intf::SM6_MASK
            }
        }
    }

    /// Set **SM5** `[13:13]`
    #[inline(always)]
    pub const fn with_sm5(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq1Intf::SM5_MASK
            } else {
                self.bits & !Irq1Intf::SM5_MASK
            }
        }
    }

    /// Set **SM4** `[12:12]`
    #[inline(always)]
    pub const fn with_sm4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq1Intf::SM4_MASK
            } else {
                self.bits & !Irq1Intf::SM4_MASK
            }
        }
    }

    /// Set **SM3** `[11:11]`
    #[inline(always)]
    pub const fn with_sm3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq1Intf::SM3_MASK
            } else {
                self.bits & !Irq1Intf::SM3_MASK
            }
        }
    }

    /// Set **SM2** `[10:10]`
    #[inline(always)]
    pub const fn with_sm2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq1Intf::SM2_MASK
            } else {
                self.bits & !Irq1Intf::SM2_MASK
            }
        }
    }

    /// Set **SM1** `[9:9]`
    #[inline(always)]
    pub const fn with_sm1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq1Intf::SM1_MASK
            } else {
                self.bits & !Irq1Intf::SM1_MASK
            }
        }
    }

    /// Set **SM0** `[8:8]`
    #[inline(always)]
    pub const fn with_sm0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq1Intf::SM0_MASK
            } else {
                self.bits & !Irq1Intf::SM0_MASK
            }
        }
    }

    /// Set **SM3_TXNFULL** `[7:7]`
    #[inline(always)]
    pub const fn with_sm3_txnfull(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq1Intf::SM3_TXNFULL_MASK
            } else {
                self.bits & !Irq1Intf::SM3_TXNFULL_MASK
            }
        }
    }

    /// Set **SM2_TXNFULL** `[6:6]`
    #[inline(always)]
    pub const fn with_sm2_txnfull(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq1Intf::SM2_TXNFULL_MASK
            } else {
                self.bits & !Irq1Intf::SM2_TXNFULL_MASK
            }
        }
    }

    /// Set **SM1_TXNFULL** `[5:5]`
    #[inline(always)]
    pub const fn with_sm1_txnfull(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq1Intf::SM1_TXNFULL_MASK
            } else {
                self.bits & !Irq1Intf::SM1_TXNFULL_MASK
            }
        }
    }

    /// Set **SM0_TXNFULL** `[4:4]`
    #[inline(always)]
    pub const fn with_sm0_txnfull(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq1Intf::SM0_TXNFULL_MASK
            } else {
                self.bits & !Irq1Intf::SM0_TXNFULL_MASK
            }
        }
    }

    /// Set **SM3_RXNEMPTY** `[3:3]`
    #[inline(always)]
    pub const fn with_sm3_rxnempty(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq1Intf::SM3_RXNEMPTY_MASK
            } else {
                self.bits & !Irq1Intf::SM3_RXNEMPTY_MASK
            }
        }
    }

    /// Set **SM2_RXNEMPTY** `[2:2]`
    #[inline(always)]
    pub const fn with_sm2_rxnempty(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq1Intf::SM2_RXNEMPTY_MASK
            } else {
                self.bits & !Irq1Intf::SM2_RXNEMPTY_MASK
            }
        }
    }

    /// Set **SM1_RXNEMPTY** `[1:1]`
    #[inline(always)]
    pub const fn with_sm1_rxnempty(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq1Intf::SM1_RXNEMPTY_MASK
            } else {
                self.bits & !Irq1Intf::SM1_RXNEMPTY_MASK
            }
        }
    }

    /// Set **SM0_RXNEMPTY** `[0:0]`
    #[inline(always)]
    pub const fn with_sm0_rxnempty(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Irq1Intf::SM0_RXNEMPTY_MASK
            } else {
                self.bits & !Irq1Intf::SM0_RXNEMPTY_MASK
            }
        }
    }

}

impl Default for Irq1IntfVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Irq1Intf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Irq1IntfVal {
        Irq1IntfVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Irq1IntfVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Irq1IntfVal) -> Irq1IntfVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// IRQ1_INTS Register
///
/// Interrupt status after masking & forcing for irq1
///
/// Address offset: 0x0184
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SM7** `[15]` (RO)
///
/// - **SM6** `[14]` (RO)
///
/// - **SM5** `[13]` (RO)
///
/// - **SM4** `[12]` (RO)
///
/// - **SM3** `[11]` (RO)
///
/// - **SM2** `[10]` (RO)
///
/// - **SM1** `[9]` (RO)
///
/// - **SM0** `[8]` (RO)
///
/// - **SM3_TXNFULL** `[7]` (RO)
///
/// - **SM2_TXNFULL** `[6]` (RO)
///
/// - **SM1_TXNFULL** `[5]` (RO)
///
/// - **SM0_TXNFULL** `[4]` (RO)
///
/// - **SM3_RXNEMPTY** `[3]` (RO)
///
/// - **SM2_RXNEMPTY** `[2]` (RO)
///
/// - **SM1_RXNEMPTY** `[1]` (RO)
///
/// - **SM0_RXNEMPTY** `[0]` (RO)
#[repr(transparent)]
pub struct Irq1Ints {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Irq1Ints {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Irq1Ints {
    // ======== Field Constants ========

    /// **SM7**
    /// Bits `[15:15]`, mask `0x00008000`
    pub const SM7_OFFSET: u32 = 15;
    pub const SM7_MASK: u32 = 0x00008000;
    pub const SM7: u32 = 1 << 15;

    /// **SM6**
    /// Bits `[14:14]`, mask `0x00004000`
    pub const SM6_OFFSET: u32 = 14;
    pub const SM6_MASK: u32 = 0x00004000;
    pub const SM6: u32 = 1 << 14;

    /// **SM5**
    /// Bits `[13:13]`, mask `0x00002000`
    pub const SM5_OFFSET: u32 = 13;
    pub const SM5_MASK: u32 = 0x00002000;
    pub const SM5: u32 = 1 << 13;

    /// **SM4**
    /// Bits `[12:12]`, mask `0x00001000`
    pub const SM4_OFFSET: u32 = 12;
    pub const SM4_MASK: u32 = 0x00001000;
    pub const SM4: u32 = 1 << 12;

    /// **SM3**
    /// Bits `[11:11]`, mask `0x00000800`
    pub const SM3_OFFSET: u32 = 11;
    pub const SM3_MASK: u32 = 0x00000800;
    pub const SM3: u32 = 1 << 11;

    /// **SM2**
    /// Bits `[10:10]`, mask `0x00000400`
    pub const SM2_OFFSET: u32 = 10;
    pub const SM2_MASK: u32 = 0x00000400;
    pub const SM2: u32 = 1 << 10;

    /// **SM1**
    /// Bits `[9:9]`, mask `0x00000200`
    pub const SM1_OFFSET: u32 = 9;
    pub const SM1_MASK: u32 = 0x00000200;
    pub const SM1: u32 = 1 << 9;

    /// **SM0**
    /// Bits `[8:8]`, mask `0x00000100`
    pub const SM0_OFFSET: u32 = 8;
    pub const SM0_MASK: u32 = 0x00000100;
    pub const SM0: u32 = 1 << 8;

    /// **SM3_TXNFULL**
    /// Bits `[7:7]`, mask `0x00000080`
    pub const SM3_TXNFULL_OFFSET: u32 = 7;
    pub const SM3_TXNFULL_MASK: u32 = 0x00000080;
    pub const SM3_TXNFULL: u32 = 1 << 7;

    /// **SM2_TXNFULL**
    /// Bits `[6:6]`, mask `0x00000040`
    pub const SM2_TXNFULL_OFFSET: u32 = 6;
    pub const SM2_TXNFULL_MASK: u32 = 0x00000040;
    pub const SM2_TXNFULL: u32 = 1 << 6;

    /// **SM1_TXNFULL**
    /// Bits `[5:5]`, mask `0x00000020`
    pub const SM1_TXNFULL_OFFSET: u32 = 5;
    pub const SM1_TXNFULL_MASK: u32 = 0x00000020;
    pub const SM1_TXNFULL: u32 = 1 << 5;

    /// **SM0_TXNFULL**
    /// Bits `[4:4]`, mask `0x00000010`
    pub const SM0_TXNFULL_OFFSET: u32 = 4;
    pub const SM0_TXNFULL_MASK: u32 = 0x00000010;
    pub const SM0_TXNFULL: u32 = 1 << 4;

    /// **SM3_RXNEMPTY**
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SM3_RXNEMPTY_OFFSET: u32 = 3;
    pub const SM3_RXNEMPTY_MASK: u32 = 0x00000008;
    pub const SM3_RXNEMPTY: u32 = 1 << 3;

    /// **SM2_RXNEMPTY**
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SM2_RXNEMPTY_OFFSET: u32 = 2;
    pub const SM2_RXNEMPTY_MASK: u32 = 0x00000004;
    pub const SM2_RXNEMPTY: u32 = 1 << 2;

    /// **SM1_RXNEMPTY**
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SM1_RXNEMPTY_OFFSET: u32 = 1;
    pub const SM1_RXNEMPTY_MASK: u32 = 0x00000002;
    pub const SM1_RXNEMPTY: u32 = 1 << 1;

    /// **SM0_RXNEMPTY**
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SM0_RXNEMPTY_OFFSET: u32 = 0;
    pub const SM0_RXNEMPTY_MASK: u32 = 0x00000001;
    pub const SM0_RXNEMPTY: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **SM7** `[15]` (RO)
    #[inline(always)]
    pub fn sm7(&self) -> bool {
        (self.value.read_bits() & Self::SM7_MASK) != 0
    }

    /// **SM6** `[14]` (RO)
    #[inline(always)]
    pub fn sm6(&self) -> bool {
        (self.value.read_bits() & Self::SM6_MASK) != 0
    }

    /// **SM5** `[13]` (RO)
    #[inline(always)]
    pub fn sm5(&self) -> bool {
        (self.value.read_bits() & Self::SM5_MASK) != 0
    }

    /// **SM4** `[12]` (RO)
    #[inline(always)]
    pub fn sm4(&self) -> bool {
        (self.value.read_bits() & Self::SM4_MASK) != 0
    }

    /// **SM3** `[11]` (RO)
    #[inline(always)]
    pub fn sm3(&self) -> bool {
        (self.value.read_bits() & Self::SM3_MASK) != 0
    }

    /// **SM2** `[10]` (RO)
    #[inline(always)]
    pub fn sm2(&self) -> bool {
        (self.value.read_bits() & Self::SM2_MASK) != 0
    }

    /// **SM1** `[9]` (RO)
    #[inline(always)]
    pub fn sm1(&self) -> bool {
        (self.value.read_bits() & Self::SM1_MASK) != 0
    }

    /// **SM0** `[8]` (RO)
    #[inline(always)]
    pub fn sm0(&self) -> bool {
        (self.value.read_bits() & Self::SM0_MASK) != 0
    }

    /// **SM3_TXNFULL** `[7]` (RO)
    #[inline(always)]
    pub fn sm3_txnfull(&self) -> bool {
        (self.value.read_bits() & Self::SM3_TXNFULL_MASK) != 0
    }

    /// **SM2_TXNFULL** `[6]` (RO)
    #[inline(always)]
    pub fn sm2_txnfull(&self) -> bool {
        (self.value.read_bits() & Self::SM2_TXNFULL_MASK) != 0
    }

    /// **SM1_TXNFULL** `[5]` (RO)
    #[inline(always)]
    pub fn sm1_txnfull(&self) -> bool {
        (self.value.read_bits() & Self::SM1_TXNFULL_MASK) != 0
    }

    /// **SM0_TXNFULL** `[4]` (RO)
    #[inline(always)]
    pub fn sm0_txnfull(&self) -> bool {
        (self.value.read_bits() & Self::SM0_TXNFULL_MASK) != 0
    }

    /// **SM3_RXNEMPTY** `[3]` (RO)
    #[inline(always)]
    pub fn sm3_rxnempty(&self) -> bool {
        (self.value.read_bits() & Self::SM3_RXNEMPTY_MASK) != 0
    }

    /// **SM2_RXNEMPTY** `[2]` (RO)
    #[inline(always)]
    pub fn sm2_rxnempty(&self) -> bool {
        (self.value.read_bits() & Self::SM2_RXNEMPTY_MASK) != 0
    }

    /// **SM1_RXNEMPTY** `[1]` (RO)
    #[inline(always)]
    pub fn sm1_rxnempty(&self) -> bool {
        (self.value.read_bits() & Self::SM1_RXNEMPTY_MASK) != 0
    }

    /// **SM0_RXNEMPTY** `[0]` (RO)
    #[inline(always)]
    pub fn sm0_rxnempty(&self) -> bool {
        (self.value.read_bits() & Self::SM0_RXNEMPTY_MASK) != 0
    }

}

/// IRQ1_INTS value type for batch read/modify/write operations
///
/// Use `irq1ints.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `irq1ints.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Irq1IntsVal {
    bits: u32,
}

impl Irq1IntsVal {
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

    /// Read **SM7** `[15:15]`
    #[inline(always)]
    pub const fn sm7(&self) -> bool {
        (self.bits & Irq1Ints::SM7_MASK) != 0
    }

    /// Read **SM6** `[14:14]`
    #[inline(always)]
    pub const fn sm6(&self) -> bool {
        (self.bits & Irq1Ints::SM6_MASK) != 0
    }

    /// Read **SM5** `[13:13]`
    #[inline(always)]
    pub const fn sm5(&self) -> bool {
        (self.bits & Irq1Ints::SM5_MASK) != 0
    }

    /// Read **SM4** `[12:12]`
    #[inline(always)]
    pub const fn sm4(&self) -> bool {
        (self.bits & Irq1Ints::SM4_MASK) != 0
    }

    /// Read **SM3** `[11:11]`
    #[inline(always)]
    pub const fn sm3(&self) -> bool {
        (self.bits & Irq1Ints::SM3_MASK) != 0
    }

    /// Read **SM2** `[10:10]`
    #[inline(always)]
    pub const fn sm2(&self) -> bool {
        (self.bits & Irq1Ints::SM2_MASK) != 0
    }

    /// Read **SM1** `[9:9]`
    #[inline(always)]
    pub const fn sm1(&self) -> bool {
        (self.bits & Irq1Ints::SM1_MASK) != 0
    }

    /// Read **SM0** `[8:8]`
    #[inline(always)]
    pub const fn sm0(&self) -> bool {
        (self.bits & Irq1Ints::SM0_MASK) != 0
    }

    /// Read **SM3_TXNFULL** `[7:7]`
    #[inline(always)]
    pub const fn sm3_txnfull(&self) -> bool {
        (self.bits & Irq1Ints::SM3_TXNFULL_MASK) != 0
    }

    /// Read **SM2_TXNFULL** `[6:6]`
    #[inline(always)]
    pub const fn sm2_txnfull(&self) -> bool {
        (self.bits & Irq1Ints::SM2_TXNFULL_MASK) != 0
    }

    /// Read **SM1_TXNFULL** `[5:5]`
    #[inline(always)]
    pub const fn sm1_txnfull(&self) -> bool {
        (self.bits & Irq1Ints::SM1_TXNFULL_MASK) != 0
    }

    /// Read **SM0_TXNFULL** `[4:4]`
    #[inline(always)]
    pub const fn sm0_txnfull(&self) -> bool {
        (self.bits & Irq1Ints::SM0_TXNFULL_MASK) != 0
    }

    /// Read **SM3_RXNEMPTY** `[3:3]`
    #[inline(always)]
    pub const fn sm3_rxnempty(&self) -> bool {
        (self.bits & Irq1Ints::SM3_RXNEMPTY_MASK) != 0
    }

    /// Read **SM2_RXNEMPTY** `[2:2]`
    #[inline(always)]
    pub const fn sm2_rxnempty(&self) -> bool {
        (self.bits & Irq1Ints::SM2_RXNEMPTY_MASK) != 0
    }

    /// Read **SM1_RXNEMPTY** `[1:1]`
    #[inline(always)]
    pub const fn sm1_rxnempty(&self) -> bool {
        (self.bits & Irq1Ints::SM1_RXNEMPTY_MASK) != 0
    }

    /// Read **SM0_RXNEMPTY** `[0:0]`
    #[inline(always)]
    pub const fn sm0_rxnempty(&self) -> bool {
        (self.bits & Irq1Ints::SM0_RXNEMPTY_MASK) != 0
    }

}

impl Default for Irq1IntsVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Irq1Ints {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Irq1IntsVal {
        Irq1IntsVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Irq1IntsVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Irq1IntsVal) -> Irq1IntsVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// PIO control register
    pub ctrl: Ctrl,
    /// 0x0004
    /// FIFO status register
    pub fstat: Fstat,
    /// 0x0008
    /// FIFO debug register
    pub fdebug: Fdebug,
    /// 0x000c
    /// FIFO levels
    pub flevel: Flevel,
    /// 0x0010
    /// Direct write access to the TX FIFO for this state machine. Each write pushes one word to the FIFO. Attempting to write to a full FIFO has no effect on the FIFO state or contents, and sets the sticky FDEBUG_TXOVER error flag for this FIFO.
    pub txf0: Txf0,
    /// 0x0014
    /// Direct write access to the TX FIFO for this state machine. Each write pushes one word to the FIFO. Attempting to write to a full FIFO has no effect on the FIFO state or contents, and sets the sticky FDEBUG_TXOVER error flag for this FIFO.
    pub txf1: Txf1,
    /// 0x0018
    /// Direct write access to the TX FIFO for this state machine. Each write pushes one word to the FIFO. Attempting to write to a full FIFO has no effect on the FIFO state or contents, and sets the sticky FDEBUG_TXOVER error flag for this FIFO.
    pub txf2: Txf2,
    /// 0x001c
    /// Direct write access to the TX FIFO for this state machine. Each write pushes one word to the FIFO. Attempting to write to a full FIFO has no effect on the FIFO state or contents, and sets the sticky FDEBUG_TXOVER error flag for this FIFO.
    pub txf3: Txf3,
    /// 0x0020
    /// Direct read access to the RX FIFO for this state machine. Each read pops one word from the FIFO. Attempting to read from an empty FIFO has no effect on the FIFO state, and sets the sticky FDEBUG_RXUNDER error flag for this FIFO. The data returned to the system on a read from an empty FIFO is undefined.
    pub rxf0: Rxf0,
    /// 0x0024
    /// Direct read access to the RX FIFO for this state machine. Each read pops one word from the FIFO. Attempting to read from an empty FIFO has no effect on the FIFO state, and sets the sticky FDEBUG_RXUNDER error flag for this FIFO. The data returned to the system on a read from an empty FIFO is undefined.
    pub rxf1: Rxf1,
    /// 0x0028
    /// Direct read access to the RX FIFO for this state machine. Each read pops one word from the FIFO. Attempting to read from an empty FIFO has no effect on the FIFO state, and sets the sticky FDEBUG_RXUNDER error flag for this FIFO. The data returned to the system on a read from an empty FIFO is undefined.
    pub rxf2: Rxf2,
    /// 0x002c
    /// Direct read access to the RX FIFO for this state machine. Each read pops one word from the FIFO. Attempting to read from an empty FIFO has no effect on the FIFO state, and sets the sticky FDEBUG_RXUNDER error flag for this FIFO. The data returned to the system on a read from an empty FIFO is undefined.
    pub rxf3: Rxf3,
    /// 0x0030
    /// State machine IRQ flags register. Write 1 to clear. There are eight state machine IRQ flags, which can be set, cleared, and waited on by the state machines. There's no fixed association between flags and state machines -- any state machine can use any flag.
    /// Any of the eight flags can be used for timing synchronisation between state machines, using IRQ and WAIT instructions. Any combination of the eight flags can also routed out to either of the two system-level interrupt requests, alongside FIFO status interrupts -- see e.g. IRQ0_INTE.
    pub irq: Irq,
    /// 0x0034
    /// Writing a 1 to each of these bits will forcibly assert the corresponding IRQ. Note this is different to the INTF register: writing here affects PIO internal state. INTF just asserts the processor-facing IRQ signal for testing ISRs, and is not visible to the state machines.
    pub irq_force: IrqForce,
    /// 0x0038
    /// There is a 2-flipflop synchronizer on each GPIO input, which protects PIO logic from metastabilities. This increases input delay, and for fast synchronous IO (e.g. SPI) these synchronizers may need to be bypassed. Each bit in this register corresponds to one GPIO.
    /// 0 -> input is synchronized (default)
    /// 1 -> synchronizer is bypassed
    /// If in doubt, leave this register as all zeroes.
    pub input_sync_bypass: InputSyncBypass,
    /// 0x003c
    /// Read to sample the pad output values PIO is currently driving to the GPIOs. On RP2040 there are 30 GPIOs, so the two most significant bits are hardwired to 0.
    pub dbg_padout: DbgPadout,
    /// 0x0040
    /// Read to sample the pad output enables (direction) PIO is currently driving to the GPIOs. On RP2040 there are 30 GPIOs, so the two most significant bits are hardwired to 0.
    pub dbg_padoe: DbgPadoe,
    /// 0x0044
    /// The PIO hardware has some free parameters that may vary between chip products.
    /// These should be provided in the chip datasheet, but are also exposed here.
    pub dbg_cfginfo: DbgCfginfo,
    /// 0x0048
    /// Write-only access to instruction memory location 0
    pub instr_mem0: InstrMem0,
    /// 0x004c
    /// Write-only access to instruction memory location 1
    pub instr_mem1: InstrMem1,
    /// 0x0050
    /// Write-only access to instruction memory location 2
    pub instr_mem2: InstrMem2,
    /// 0x0054
    /// Write-only access to instruction memory location 3
    pub instr_mem3: InstrMem3,
    /// 0x0058
    /// Write-only access to instruction memory location 4
    pub instr_mem4: InstrMem4,
    /// 0x005c
    /// Write-only access to instruction memory location 5
    pub instr_mem5: InstrMem5,
    /// 0x0060
    /// Write-only access to instruction memory location 6
    pub instr_mem6: InstrMem6,
    /// 0x0064
    /// Write-only access to instruction memory location 7
    pub instr_mem7: InstrMem7,
    /// 0x0068
    /// Write-only access to instruction memory location 8
    pub instr_mem8: InstrMem8,
    /// 0x006c
    /// Write-only access to instruction memory location 9
    pub instr_mem9: InstrMem9,
    /// 0x0070
    /// Write-only access to instruction memory location 10
    pub instr_mem10: InstrMem10,
    /// 0x0074
    /// Write-only access to instruction memory location 11
    pub instr_mem11: InstrMem11,
    /// 0x0078
    /// Write-only access to instruction memory location 12
    pub instr_mem12: InstrMem12,
    /// 0x007c
    /// Write-only access to instruction memory location 13
    pub instr_mem13: InstrMem13,
    /// 0x0080
    /// Write-only access to instruction memory location 14
    pub instr_mem14: InstrMem14,
    /// 0x0084
    /// Write-only access to instruction memory location 15
    pub instr_mem15: InstrMem15,
    /// 0x0088
    /// Write-only access to instruction memory location 16
    pub instr_mem16: InstrMem16,
    /// 0x008c
    /// Write-only access to instruction memory location 17
    pub instr_mem17: InstrMem17,
    /// 0x0090
    /// Write-only access to instruction memory location 18
    pub instr_mem18: InstrMem18,
    /// 0x0094
    /// Write-only access to instruction memory location 19
    pub instr_mem19: InstrMem19,
    /// 0x0098
    /// Write-only access to instruction memory location 20
    pub instr_mem20: InstrMem20,
    /// 0x009c
    /// Write-only access to instruction memory location 21
    pub instr_mem21: InstrMem21,
    /// 0x00a0
    /// Write-only access to instruction memory location 22
    pub instr_mem22: InstrMem22,
    /// 0x00a4
    /// Write-only access to instruction memory location 23
    pub instr_mem23: InstrMem23,
    /// 0x00a8
    /// Write-only access to instruction memory location 24
    pub instr_mem24: InstrMem24,
    /// 0x00ac
    /// Write-only access to instruction memory location 25
    pub instr_mem25: InstrMem25,
    /// 0x00b0
    /// Write-only access to instruction memory location 26
    pub instr_mem26: InstrMem26,
    /// 0x00b4
    /// Write-only access to instruction memory location 27
    pub instr_mem27: InstrMem27,
    /// 0x00b8
    /// Write-only access to instruction memory location 28
    pub instr_mem28: InstrMem28,
    /// 0x00bc
    /// Write-only access to instruction memory location 29
    pub instr_mem29: InstrMem29,
    /// 0x00c0
    /// Write-only access to instruction memory location 30
    pub instr_mem30: InstrMem30,
    /// 0x00c4
    /// Write-only access to instruction memory location 31
    pub instr_mem31: InstrMem31,
    /// 0x00c8
    /// Clock divisor register for state machine 0
    /// Frequency = clock freq / (CLKDIV_INT + CLKDIV_FRAC / 256)
    pub sm0_clkdiv: Sm0Clkdiv,
    /// 0x00cc
    /// Execution/behavioural settings for state machine 0
    pub sm0_execctrl: Sm0Execctrl,
    /// 0x00d0
    /// Control behaviour of the input/output shift registers for state machine 0
    pub sm0_shiftctrl: Sm0Shiftctrl,
    /// 0x00d4
    /// Current instruction address of state machine 0
    pub sm0_addr: Sm0Addr,
    /// 0x00d8
    /// Read to see the instruction currently addressed by state machine 0's program counter
    /// Write to execute an instruction immediately (including jumps) and then resume execution.
    pub sm0_instr: Sm0Instr,
    /// 0x00dc
    /// State machine pin control
    pub sm0_pinctrl: Sm0Pinctrl,
    /// 0x00e0
    /// Clock divisor register for state machine 1
    /// Frequency = clock freq / (CLKDIV_INT + CLKDIV_FRAC / 256)
    pub sm1_clkdiv: Sm1Clkdiv,
    /// 0x00e4
    /// Execution/behavioural settings for state machine 1
    pub sm1_execctrl: Sm1Execctrl,
    /// 0x00e8
    /// Control behaviour of the input/output shift registers for state machine 1
    pub sm1_shiftctrl: Sm1Shiftctrl,
    /// 0x00ec
    /// Current instruction address of state machine 1
    pub sm1_addr: Sm1Addr,
    /// 0x00f0
    /// Read to see the instruction currently addressed by state machine 1's program counter
    /// Write to execute an instruction immediately (including jumps) and then resume execution.
    pub sm1_instr: Sm1Instr,
    /// 0x00f4
    /// State machine pin control
    pub sm1_pinctrl: Sm1Pinctrl,
    /// 0x00f8
    /// Clock divisor register for state machine 2
    /// Frequency = clock freq / (CLKDIV_INT + CLKDIV_FRAC / 256)
    pub sm2_clkdiv: Sm2Clkdiv,
    /// 0x00fc
    /// Execution/behavioural settings for state machine 2
    pub sm2_execctrl: Sm2Execctrl,
    /// 0x0100
    /// Control behaviour of the input/output shift registers for state machine 2
    pub sm2_shiftctrl: Sm2Shiftctrl,
    /// 0x0104
    /// Current instruction address of state machine 2
    pub sm2_addr: Sm2Addr,
    /// 0x0108
    /// Read to see the instruction currently addressed by state machine 2's program counter
    /// Write to execute an instruction immediately (including jumps) and then resume execution.
    pub sm2_instr: Sm2Instr,
    /// 0x010c
    /// State machine pin control
    pub sm2_pinctrl: Sm2Pinctrl,
    /// 0x0110
    /// Clock divisor register for state machine 3
    /// Frequency = clock freq / (CLKDIV_INT + CLKDIV_FRAC / 256)
    pub sm3_clkdiv: Sm3Clkdiv,
    /// 0x0114
    /// Execution/behavioural settings for state machine 3
    pub sm3_execctrl: Sm3Execctrl,
    /// 0x0118
    /// Control behaviour of the input/output shift registers for state machine 3
    pub sm3_shiftctrl: Sm3Shiftctrl,
    /// 0x011c
    /// Current instruction address of state machine 3
    pub sm3_addr: Sm3Addr,
    /// 0x0120
    /// Read to see the instruction currently addressed by state machine 3's program counter
    /// Write to execute an instruction immediately (including jumps) and then resume execution.
    pub sm3_instr: Sm3Instr,
    /// 0x0124
    /// State machine pin control
    pub sm3_pinctrl: Sm3Pinctrl,
    /// 0x0128
    /// Direct read/write access to entry 0 of SM0's RX FIFO, if SHIFTCTRL_FJOIN_RX_PUT xor SHIFTCTRL_FJOIN_RX_GET is set.
    pub rxf0_putget0: Rxf0Putget0,
    /// 0x012c
    /// Direct read/write access to entry 1 of SM0's RX FIFO, if SHIFTCTRL_FJOIN_RX_PUT xor SHIFTCTRL_FJOIN_RX_GET is set.
    pub rxf0_putget1: Rxf0Putget1,
    /// 0x0130
    /// Direct read/write access to entry 2 of SM0's RX FIFO, if SHIFTCTRL_FJOIN_RX_PUT xor SHIFTCTRL_FJOIN_RX_GET is set.
    pub rxf0_putget2: Rxf0Putget2,
    /// 0x0134
    /// Direct read/write access to entry 3 of SM0's RX FIFO, if SHIFTCTRL_FJOIN_RX_PUT xor SHIFTCTRL_FJOIN_RX_GET is set.
    pub rxf0_putget3: Rxf0Putget3,
    /// 0x0138
    /// Direct read/write access to entry 0 of SM1's RX FIFO, if SHIFTCTRL_FJOIN_RX_PUT xor SHIFTCTRL_FJOIN_RX_GET is set.
    pub rxf1_putget0: Rxf1Putget0,
    /// 0x013c
    /// Direct read/write access to entry 1 of SM1's RX FIFO, if SHIFTCTRL_FJOIN_RX_PUT xor SHIFTCTRL_FJOIN_RX_GET is set.
    pub rxf1_putget1: Rxf1Putget1,
    /// 0x0140
    /// Direct read/write access to entry 2 of SM1's RX FIFO, if SHIFTCTRL_FJOIN_RX_PUT xor SHIFTCTRL_FJOIN_RX_GET is set.
    pub rxf1_putget2: Rxf1Putget2,
    /// 0x0144
    /// Direct read/write access to entry 3 of SM1's RX FIFO, if SHIFTCTRL_FJOIN_RX_PUT xor SHIFTCTRL_FJOIN_RX_GET is set.
    pub rxf1_putget3: Rxf1Putget3,
    /// 0x0148
    /// Direct read/write access to entry 0 of SM2's RX FIFO, if SHIFTCTRL_FJOIN_RX_PUT xor SHIFTCTRL_FJOIN_RX_GET is set.
    pub rxf2_putget0: Rxf2Putget0,
    /// 0x014c
    /// Direct read/write access to entry 1 of SM2's RX FIFO, if SHIFTCTRL_FJOIN_RX_PUT xor SHIFTCTRL_FJOIN_RX_GET is set.
    pub rxf2_putget1: Rxf2Putget1,
    /// 0x0150
    /// Direct read/write access to entry 2 of SM2's RX FIFO, if SHIFTCTRL_FJOIN_RX_PUT xor SHIFTCTRL_FJOIN_RX_GET is set.
    pub rxf2_putget2: Rxf2Putget2,
    /// 0x0154
    /// Direct read/write access to entry 3 of SM2's RX FIFO, if SHIFTCTRL_FJOIN_RX_PUT xor SHIFTCTRL_FJOIN_RX_GET is set.
    pub rxf2_putget3: Rxf2Putget3,
    /// 0x0158
    /// Direct read/write access to entry 0 of SM3's RX FIFO, if SHIFTCTRL_FJOIN_RX_PUT xor SHIFTCTRL_FJOIN_RX_GET is set.
    pub rxf3_putget0: Rxf3Putget0,
    /// 0x015c
    /// Direct read/write access to entry 1 of SM3's RX FIFO, if SHIFTCTRL_FJOIN_RX_PUT xor SHIFTCTRL_FJOIN_RX_GET is set.
    pub rxf3_putget1: Rxf3Putget1,
    /// 0x0160
    /// Direct read/write access to entry 2 of SM3's RX FIFO, if SHIFTCTRL_FJOIN_RX_PUT xor SHIFTCTRL_FJOIN_RX_GET is set.
    pub rxf3_putget2: Rxf3Putget2,
    /// 0x0164
    /// Direct read/write access to entry 3 of SM3's RX FIFO, if SHIFTCTRL_FJOIN_RX_PUT xor SHIFTCTRL_FJOIN_RX_GET is set.
    pub rxf3_putget3: Rxf3Putget3,
    /// 0x0168
    /// Relocate GPIO 0 (from PIO's point of view) in the system GPIO numbering, to access more than 32 GPIOs from PIO.
    /// Only the values 0 and 16 are supported (only bit 4 is writable).
    pub gpiobase: Gpiobase,
    /// 0x016c
    /// Raw Interrupts
    pub intr: Intr,
    /// 0x0170
    /// Interrupt Enable for irq0
    pub irq0_inte: Irq0Inte,
    /// 0x0174
    /// Interrupt Force for irq0
    pub irq0_intf: Irq0Intf,
    /// 0x0178
    /// Interrupt status after masking & forcing for irq0
    pub irq0_ints: Irq0Ints,
    /// 0x017c
    /// Interrupt Enable for irq1
    pub irq1_inte: Irq1Inte,
    /// 0x0180
    /// Interrupt Force for irq1
    pub irq1_intf: Irq1Intf,
    /// 0x0184
    /// Interrupt status after masking & forcing for irq1
    pub irq1_ints: Irq1Ints,
}

