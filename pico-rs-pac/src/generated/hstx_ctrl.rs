//! HSTX_CTRL - Control interface to HSTX. For FIFO write access and status, see the HSTX_F...
//!
//! Control interface to HSTX. For FIFO write access and status, see the
//! HSTX_FIFO register block.

#![allow(dead_code)]

pub const BASE: u32 = 0x400c0000;

/// CSR Register
///
/// Address offset: 0x0000
/// Reset value: 0x10050600
///
/// ## Fields
///
/// - **CLKDIV** [31:28] (RW): Clock period of the generated clock, measured in HSTX clock cycles. Can be ...
/// - **CLKPHASE** [27:24] (RW): Set the initial phase of the generated clock. A CLKPHASE of 0 means the clo...
/// - **N_SHIFTS** [20:16] (RW): Number of times to shift the shift register before refilling it from the FI...
/// - **SHIFT** [12:8] (RW): How many bits to right-rotate the shift register by each cycle. The use of ...
/// - **COUPLED_SEL** [6:5] (RW): Select which PIO to use for coupled mode operation.
/// - **COUPLED_MODE** [4] (RW): Enable the PIO-to-HSTX 1:1 connection. The HSTX must be clocked *directly* ...
/// - **EXPAND_EN** [1] (RW): Enable the command expander. When 0, raw FIFO data is passed directly to th...
/// - **EN** [0] (RW): When EN is 1, the HSTX will shift out data as it appears in the FIFO. As lo...
#[repr(transparent)]
pub struct Csr {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for Csr {}
unsafe impl Sync for Csr {}

impl Csr {
    // ======== Field Constants ========

    /// CLKDIV - Clock period of the generated clock, measured in HSTX clock cycles. Can be ...
    /// Bits [31:28], mask 0xf0000000
    pub const CLKDIV_OFFSET: u32 = 28;
    pub const CLKDIV_MASK: u32 = 0xf0000000;

    /// CLKPHASE - Set the initial phase of the generated clock. A CLKPHASE of 0 means the clo...
    /// Bits [27:24], mask 0x0f000000
    pub const CLKPHASE_OFFSET: u32 = 24;
    pub const CLKPHASE_MASK: u32 = 0x0f000000;

    /// N_SHIFTS - Number of times to shift the shift register before refilling it from the FI...
    /// Bits [20:16], mask 0x001f0000
    pub const N_SHIFTS_OFFSET: u32 = 16;
    pub const N_SHIFTS_MASK: u32 = 0x001f0000;

    /// SHIFT - How many bits to right-rotate the shift register by each cycle. The use of ...
    /// Bits [12:8], mask 0x00001f00
    pub const SHIFT_OFFSET: u32 = 8;
    pub const SHIFT_MASK: u32 = 0x00001f00;

    /// COUPLED_SEL - Select which PIO to use for coupled mode operation.
    /// Bits [6:5], mask 0x00000060
    pub const COUPLED_SEL_OFFSET: u32 = 5;
    pub const COUPLED_SEL_MASK: u32 = 0x00000060;

    /// COUPLED_MODE - Enable the PIO-to-HSTX 1:1 connection. The HSTX must be clocked *directly* ...
    /// Bits [4:4], mask 0x00000010
    pub const COUPLED_MODE_OFFSET: u32 = 4;
    pub const COUPLED_MODE_MASK: u32 = 0x00000010;
    pub const COUPLED_MODE: u32 = 1 << 4;

    /// EXPAND_EN - Enable the command expander. When 0, raw FIFO data is passed directly to th...
    /// Bits [1:1], mask 0x00000002
    pub const EXPAND_EN_OFFSET: u32 = 1;
    pub const EXPAND_EN_MASK: u32 = 0x00000002;
    pub const EXPAND_EN: u32 = 1 << 1;

    /// EN - When EN is 1, the HSTX will shift out data as it appears in the FIFO. As lo...
    /// Bits [0:0], mask 0x00000001
    pub const EN_OFFSET: u32 = 0;
    pub const EN_MASK: u32 = 0x00000001;
    pub const EN: u32 = 1 << 0;

    // ======== Basic Access Methods ========

    /// Read the raw register value as u32
    #[inline(always)]
    pub fn read_bits(&self) -> u32 {
        unsafe { core::ptr::read_volatile(self.value.get()) }
    }

    /// Write a raw u32 value to the register
    #[inline(always)]
    pub fn write_bits(&self, value: u32) {
        unsafe { core::ptr::write_volatile(self.value.get(), value) }
    }

    /// Modify the raw register value
    #[inline(always)]
    pub fn modify_bits<F: FnOnce(u32) -> u32>(&self, f: F) {
        self.write_bits(f(self.read_bits()));
    }

    /// Atomic SET - sets bits that are 1 (hardware alias at +0x2000)
    #[inline(always)]
    pub fn set_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x2000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic CLR - clears bits that are 1 (hardware alias at +0x3000)
    #[inline(always)]
    pub fn clr_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x3000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic XOR - toggles bits that are 1 (hardware alias at +0x1000)
    #[inline(always)]
    pub fn xor_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x1000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    // ======== Field Accessors ========

    /// **CLKDIV** [31:28]: Clock period of the generated clock, measured in HSTX clock cycles. Can be ...
    #[inline(always)]
    pub fn clkdiv(&self) -> u32 {
        (self.read_bits() & Self::CLKDIV_MASK) >> Self::CLKDIV_OFFSET
    }

    /// Write CLKDIV field
    #[inline(always)]
    pub fn write_clkdiv(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::CLKDIV_MASK) | ((value << Self::CLKDIV_OFFSET) & Self::CLKDIV_MASK));
    }

    /// **CLKPHASE** [27:24]: Set the initial phase of the generated clock. A CLKPHASE of 0 means the clo...
    #[inline(always)]
    pub fn clkphase(&self) -> u32 {
        (self.read_bits() & Self::CLKPHASE_MASK) >> Self::CLKPHASE_OFFSET
    }

    /// Write CLKPHASE field
    #[inline(always)]
    pub fn write_clkphase(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::CLKPHASE_MASK) | ((value << Self::CLKPHASE_OFFSET) & Self::CLKPHASE_MASK));
    }

    /// **N_SHIFTS** [20:16]: Number of times to shift the shift register before refilling it from the FI...
    #[inline(always)]
    pub fn n_shifts(&self) -> u32 {
        (self.read_bits() & Self::N_SHIFTS_MASK) >> Self::N_SHIFTS_OFFSET
    }

    /// Write N_SHIFTS field
    #[inline(always)]
    pub fn write_n_shifts(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::N_SHIFTS_MASK) | ((value << Self::N_SHIFTS_OFFSET) & Self::N_SHIFTS_MASK));
    }

    /// **SHIFT** [12:8]: How many bits to right-rotate the shift register by each cycle. The use of ...
    #[inline(always)]
    pub fn shift(&self) -> u32 {
        (self.read_bits() & Self::SHIFT_MASK) >> Self::SHIFT_OFFSET
    }

    /// Write SHIFT field
    #[inline(always)]
    pub fn write_shift(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SHIFT_MASK) | ((value << Self::SHIFT_OFFSET) & Self::SHIFT_MASK));
    }

    /// **COUPLED_SEL** [6:5]: Select which PIO to use for coupled mode operation.
    #[inline(always)]
    pub fn coupled_sel(&self) -> u32 {
        (self.read_bits() & Self::COUPLED_SEL_MASK) >> Self::COUPLED_SEL_OFFSET
    }

    /// Write COUPLED_SEL field
    #[inline(always)]
    pub fn write_coupled_sel(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::COUPLED_SEL_MASK) | ((value << Self::COUPLED_SEL_OFFSET) & Self::COUPLED_SEL_MASK));
    }

    /// **COUPLED_MODE** [4:4]: Enable the PIO-to-HSTX 1:1 connection. The HSTX must be clocked *directly* ...
    #[inline(always)]
    pub fn coupled_mode(&self) -> bool {
        (self.read_bits() & Self::COUPLED_MODE_MASK) != 0
    }

    /// Set COUPLED_MODE bit
    #[inline(always)]
    pub fn set_coupled_mode(&self) {
        self.set_bits(Self::COUPLED_MODE_MASK);
    }

    /// Clear COUPLED_MODE bit
    #[inline(always)]
    pub fn clr_coupled_mode(&self) {
        self.clr_bits(Self::COUPLED_MODE_MASK);
    }

    /// **EXPAND_EN** [1:1]: Enable the command expander. When 0, raw FIFO data is passed directly to th...
    #[inline(always)]
    pub fn expand_en(&self) -> bool {
        (self.read_bits() & Self::EXPAND_EN_MASK) != 0
    }

    /// Set EXPAND_EN bit
    #[inline(always)]
    pub fn set_expand_en(&self) {
        self.set_bits(Self::EXPAND_EN_MASK);
    }

    /// Clear EXPAND_EN bit
    #[inline(always)]
    pub fn clr_expand_en(&self) {
        self.clr_bits(Self::EXPAND_EN_MASK);
    }

    /// **EN** [0:0]: When EN is 1, the HSTX will shift out data as it appears in the FIFO. As lo...
    #[inline(always)]
    pub fn en(&self) -> bool {
        (self.read_bits() & Self::EN_MASK) != 0
    }

    /// Set EN bit
    #[inline(always)]
    pub fn set_en(&self) {
        self.set_bits(Self::EN_MASK);
    }

    /// Clear EN bit
    #[inline(always)]
    pub fn clr_en(&self) {
        self.clr_bits(Self::EN_MASK);
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
        Self { bits: 0x10050600 }
    }

    /// Read CLKDIV [31:28]: Clock period of the generated clock, measured in HSTX clock cycles. Can be ...
    #[inline(always)]
    pub const fn clkdiv(&self) -> u32 {
        (self.bits & Csr::CLKDIV_MASK) >> Csr::CLKDIV_OFFSET
    }

    /// Read CLKPHASE [27:24]: Set the initial phase of the generated clock. A CLKPHASE of 0 means the clo...
    #[inline(always)]
    pub const fn clkphase(&self) -> u32 {
        (self.bits & Csr::CLKPHASE_MASK) >> Csr::CLKPHASE_OFFSET
    }

    /// Read N_SHIFTS [20:16]: Number of times to shift the shift register before refilling it from the FI...
    #[inline(always)]
    pub const fn n_shifts(&self) -> u32 {
        (self.bits & Csr::N_SHIFTS_MASK) >> Csr::N_SHIFTS_OFFSET
    }

    /// Read SHIFT [12:8]: How many bits to right-rotate the shift register by each cycle. The use of ...
    #[inline(always)]
    pub const fn shift(&self) -> u32 {
        (self.bits & Csr::SHIFT_MASK) >> Csr::SHIFT_OFFSET
    }

    /// Read COUPLED_SEL [6:5]: Select which PIO to use for coupled mode operation.
    #[inline(always)]
    pub const fn coupled_sel(&self) -> u32 {
        (self.bits & Csr::COUPLED_SEL_MASK) >> Csr::COUPLED_SEL_OFFSET
    }

    /// Read COUPLED_MODE [4:4]: Enable the PIO-to-HSTX 1:1 connection. The HSTX must be clocked *directly* ...
    #[inline(always)]
    pub const fn coupled_mode(&self) -> bool {
        (self.bits & Csr::COUPLED_MODE_MASK) != 0
    }

    /// Read EXPAND_EN [1:1]: Enable the command expander. When 0, raw FIFO data is passed directly to th...
    #[inline(always)]
    pub const fn expand_en(&self) -> bool {
        (self.bits & Csr::EXPAND_EN_MASK) != 0
    }

    /// Read EN [0:0]: When EN is 1, the HSTX will shift out data as it appears in the FIFO. As lo...
    #[inline(always)]
    pub const fn en(&self) -> bool {
        (self.bits & Csr::EN_MASK) != 0
    }

    /// Set CLKDIV [31:28]: Clock period of the generated clock, measured in HSTX clock cycles. Can be ...
    #[inline(always)]
    pub const fn with_clkdiv(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Csr::CLKDIV_MASK) | ((value << Csr::CLKDIV_OFFSET) & Csr::CLKDIV_MASK)
        }
    }

    /// Set CLKPHASE [27:24]: Set the initial phase of the generated clock. A CLKPHASE of 0 means the clo...
    #[inline(always)]
    pub const fn with_clkphase(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Csr::CLKPHASE_MASK) | ((value << Csr::CLKPHASE_OFFSET) & Csr::CLKPHASE_MASK)
        }
    }

    /// Set N_SHIFTS [20:16]: Number of times to shift the shift register before refilling it from the FI...
    #[inline(always)]
    pub const fn with_n_shifts(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Csr::N_SHIFTS_MASK) | ((value << Csr::N_SHIFTS_OFFSET) & Csr::N_SHIFTS_MASK)
        }
    }

    /// Set SHIFT [12:8]: How many bits to right-rotate the shift register by each cycle. The use of ...
    #[inline(always)]
    pub const fn with_shift(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Csr::SHIFT_MASK) | ((value << Csr::SHIFT_OFFSET) & Csr::SHIFT_MASK)
        }
    }

    /// Set COUPLED_SEL [6:5]: Select which PIO to use for coupled mode operation.
    #[inline(always)]
    pub const fn with_coupled_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Csr::COUPLED_SEL_MASK) | ((value << Csr::COUPLED_SEL_OFFSET) & Csr::COUPLED_SEL_MASK)
        }
    }

    /// Set COUPLED_MODE [4:4]: Enable the PIO-to-HSTX 1:1 connection. The HSTX must be clocked *directly* ...
    #[inline(always)]
    pub const fn with_coupled_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Csr::COUPLED_MODE_MASK
            } else {
                self.bits & !Csr::COUPLED_MODE_MASK
            }
        }
    }

    /// Set EXPAND_EN [1:1]: Enable the command expander. When 0, raw FIFO data is passed directly to th...
    #[inline(always)]
    pub const fn with_expand_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Csr::EXPAND_EN_MASK
            } else {
                self.bits & !Csr::EXPAND_EN_MASK
            }
        }
    }

    /// Set EN [0:0]: When EN is 1, the HSTX will shift out data as it appears in the FIFO. As lo...
    #[inline(always)]
    pub const fn with_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Csr::EN_MASK
            } else {
                self.bits & !Csr::EN_MASK
            }
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
    pub fn read(&self) -> CsrVal {
        CsrVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: CsrVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(CsrVal) -> CsrVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// BIT0 Register
///
/// Data control register for output bit 0
///
/// Address offset: 0x0004
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CLK** [17] (RW): Connect this output to the generated clock, rather than the data shift regi...
/// - **INV** [16] (RW): Invert this data output (logical NOT)
/// - **SEL_N** [12:8] (RW): Shift register data bit select for the second half of the HSTX clock cycle
/// - **SEL_P** [4:0] (RW): Shift register data bit select for the first half of the HSTX clock cycle
#[repr(transparent)]
pub struct Bit0 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for Bit0 {}
unsafe impl Sync for Bit0 {}

impl Bit0 {
    // ======== Field Constants ========

    /// CLK - Connect this output to the generated clock, rather than the data shift regi...
    /// Bits [17:17], mask 0x00020000
    pub const CLK_OFFSET: u32 = 17;
    pub const CLK_MASK: u32 = 0x00020000;
    pub const CLK: u32 = 1 << 17;

    /// INV - Invert this data output (logical NOT)
    /// Bits [16:16], mask 0x00010000
    pub const INV_OFFSET: u32 = 16;
    pub const INV_MASK: u32 = 0x00010000;
    pub const INV: u32 = 1 << 16;

    /// SEL_N - Shift register data bit select for the second half of the HSTX clock cycle
    /// Bits [12:8], mask 0x00001f00
    pub const SEL_N_OFFSET: u32 = 8;
    pub const SEL_N_MASK: u32 = 0x00001f00;

    /// SEL_P - Shift register data bit select for the first half of the HSTX clock cycle
    /// Bits [4:0], mask 0x0000001f
    pub const SEL_P_OFFSET: u32 = 0;
    pub const SEL_P_MASK: u32 = 0x0000001f;

    // ======== Basic Access Methods ========

    /// Read the raw register value as u32
    #[inline(always)]
    pub fn read_bits(&self) -> u32 {
        unsafe { core::ptr::read_volatile(self.value.get()) }
    }

    /// Write a raw u32 value to the register
    #[inline(always)]
    pub fn write_bits(&self, value: u32) {
        unsafe { core::ptr::write_volatile(self.value.get(), value) }
    }

    /// Modify the raw register value
    #[inline(always)]
    pub fn modify_bits<F: FnOnce(u32) -> u32>(&self, f: F) {
        self.write_bits(f(self.read_bits()));
    }

    /// Atomic SET - sets bits that are 1 (hardware alias at +0x2000)
    #[inline(always)]
    pub fn set_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x2000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic CLR - clears bits that are 1 (hardware alias at +0x3000)
    #[inline(always)]
    pub fn clr_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x3000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic XOR - toggles bits that are 1 (hardware alias at +0x1000)
    #[inline(always)]
    pub fn xor_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x1000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    // ======== Field Accessors ========

    /// **CLK** [17:17]: Connect this output to the generated clock, rather than the data shift regi...
    #[inline(always)]
    pub fn clk(&self) -> bool {
        (self.read_bits() & Self::CLK_MASK) != 0
    }

    /// Set CLK bit
    #[inline(always)]
    pub fn set_clk(&self) {
        self.set_bits(Self::CLK_MASK);
    }

    /// Clear CLK bit
    #[inline(always)]
    pub fn clr_clk(&self) {
        self.clr_bits(Self::CLK_MASK);
    }

    /// **INV** [16:16]: Invert this data output (logical NOT)
    #[inline(always)]
    pub fn inv(&self) -> bool {
        (self.read_bits() & Self::INV_MASK) != 0
    }

    /// Set INV bit
    #[inline(always)]
    pub fn set_inv(&self) {
        self.set_bits(Self::INV_MASK);
    }

    /// Clear INV bit
    #[inline(always)]
    pub fn clr_inv(&self) {
        self.clr_bits(Self::INV_MASK);
    }

    /// **SEL_N** [12:8]: Shift register data bit select for the second half of the HSTX clock cycle
    #[inline(always)]
    pub fn sel_n(&self) -> u32 {
        (self.read_bits() & Self::SEL_N_MASK) >> Self::SEL_N_OFFSET
    }

    /// Write SEL_N field
    #[inline(always)]
    pub fn write_sel_n(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEL_N_MASK) | ((value << Self::SEL_N_OFFSET) & Self::SEL_N_MASK));
    }

    /// **SEL_P** [4:0]: Shift register data bit select for the first half of the HSTX clock cycle
    #[inline(always)]
    pub fn sel_p(&self) -> u32 {
        (self.read_bits() & Self::SEL_P_MASK) >> Self::SEL_P_OFFSET
    }

    /// Write SEL_P field
    #[inline(always)]
    pub fn write_sel_p(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEL_P_MASK) | ((value << Self::SEL_P_OFFSET) & Self::SEL_P_MASK));
    }

}

/// BIT0 value type for batch read/modify/write operations
///
/// Use `bit0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bit0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bit0Val {
    bits: u32,
}

impl Bit0Val {
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

    /// Read CLK [17:17]: Connect this output to the generated clock, rather than the data shift regi...
    #[inline(always)]
    pub const fn clk(&self) -> bool {
        (self.bits & Bit0::CLK_MASK) != 0
    }

    /// Read INV [16:16]: Invert this data output (logical NOT)
    #[inline(always)]
    pub const fn inv(&self) -> bool {
        (self.bits & Bit0::INV_MASK) != 0
    }

    /// Read SEL_N [12:8]: Shift register data bit select for the second half of the HSTX clock cycle
    #[inline(always)]
    pub const fn sel_n(&self) -> u32 {
        (self.bits & Bit0::SEL_N_MASK) >> Bit0::SEL_N_OFFSET
    }

    /// Read SEL_P [4:0]: Shift register data bit select for the first half of the HSTX clock cycle
    #[inline(always)]
    pub const fn sel_p(&self) -> u32 {
        (self.bits & Bit0::SEL_P_MASK) >> Bit0::SEL_P_OFFSET
    }

    /// Set CLK [17:17]: Connect this output to the generated clock, rather than the data shift regi...
    #[inline(always)]
    pub const fn with_clk(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bit0::CLK_MASK
            } else {
                self.bits & !Bit0::CLK_MASK
            }
        }
    }

    /// Set INV [16:16]: Invert this data output (logical NOT)
    #[inline(always)]
    pub const fn with_inv(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bit0::INV_MASK
            } else {
                self.bits & !Bit0::INV_MASK
            }
        }
    }

    /// Set SEL_N [12:8]: Shift register data bit select for the second half of the HSTX clock cycle
    #[inline(always)]
    pub const fn with_sel_n(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Bit0::SEL_N_MASK) | ((value << Bit0::SEL_N_OFFSET) & Bit0::SEL_N_MASK)
        }
    }

    /// Set SEL_P [4:0]: Shift register data bit select for the first half of the HSTX clock cycle
    #[inline(always)]
    pub const fn with_sel_p(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Bit0::SEL_P_MASK) | ((value << Bit0::SEL_P_OFFSET) & Bit0::SEL_P_MASK)
        }
    }

}

impl Default for Bit0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Bit0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> Bit0Val {
        Bit0Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: Bit0Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(Bit0Val) -> Bit0Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// BIT1 Register
///
/// Data control register for output bit 1
///
/// Address offset: 0x0008
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CLK** [17] (RW): Connect this output to the generated clock, rather than the data shift regi...
/// - **INV** [16] (RW): Invert this data output (logical NOT)
/// - **SEL_N** [12:8] (RW): Shift register data bit select for the second half of the HSTX clock cycle
/// - **SEL_P** [4:0] (RW): Shift register data bit select for the first half of the HSTX clock cycle
#[repr(transparent)]
pub struct Bit1 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for Bit1 {}
unsafe impl Sync for Bit1 {}

impl Bit1 {
    // ======== Field Constants ========

    /// CLK - Connect this output to the generated clock, rather than the data shift regi...
    /// Bits [17:17], mask 0x00020000
    pub const CLK_OFFSET: u32 = 17;
    pub const CLK_MASK: u32 = 0x00020000;
    pub const CLK: u32 = 1 << 17;

    /// INV - Invert this data output (logical NOT)
    /// Bits [16:16], mask 0x00010000
    pub const INV_OFFSET: u32 = 16;
    pub const INV_MASK: u32 = 0x00010000;
    pub const INV: u32 = 1 << 16;

    /// SEL_N - Shift register data bit select for the second half of the HSTX clock cycle
    /// Bits [12:8], mask 0x00001f00
    pub const SEL_N_OFFSET: u32 = 8;
    pub const SEL_N_MASK: u32 = 0x00001f00;

    /// SEL_P - Shift register data bit select for the first half of the HSTX clock cycle
    /// Bits [4:0], mask 0x0000001f
    pub const SEL_P_OFFSET: u32 = 0;
    pub const SEL_P_MASK: u32 = 0x0000001f;

    // ======== Basic Access Methods ========

    /// Read the raw register value as u32
    #[inline(always)]
    pub fn read_bits(&self) -> u32 {
        unsafe { core::ptr::read_volatile(self.value.get()) }
    }

    /// Write a raw u32 value to the register
    #[inline(always)]
    pub fn write_bits(&self, value: u32) {
        unsafe { core::ptr::write_volatile(self.value.get(), value) }
    }

    /// Modify the raw register value
    #[inline(always)]
    pub fn modify_bits<F: FnOnce(u32) -> u32>(&self, f: F) {
        self.write_bits(f(self.read_bits()));
    }

    /// Atomic SET - sets bits that are 1 (hardware alias at +0x2000)
    #[inline(always)]
    pub fn set_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x2000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic CLR - clears bits that are 1 (hardware alias at +0x3000)
    #[inline(always)]
    pub fn clr_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x3000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic XOR - toggles bits that are 1 (hardware alias at +0x1000)
    #[inline(always)]
    pub fn xor_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x1000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    // ======== Field Accessors ========

    /// **CLK** [17:17]: Connect this output to the generated clock, rather than the data shift regi...
    #[inline(always)]
    pub fn clk(&self) -> bool {
        (self.read_bits() & Self::CLK_MASK) != 0
    }

    /// Set CLK bit
    #[inline(always)]
    pub fn set_clk(&self) {
        self.set_bits(Self::CLK_MASK);
    }

    /// Clear CLK bit
    #[inline(always)]
    pub fn clr_clk(&self) {
        self.clr_bits(Self::CLK_MASK);
    }

    /// **INV** [16:16]: Invert this data output (logical NOT)
    #[inline(always)]
    pub fn inv(&self) -> bool {
        (self.read_bits() & Self::INV_MASK) != 0
    }

    /// Set INV bit
    #[inline(always)]
    pub fn set_inv(&self) {
        self.set_bits(Self::INV_MASK);
    }

    /// Clear INV bit
    #[inline(always)]
    pub fn clr_inv(&self) {
        self.clr_bits(Self::INV_MASK);
    }

    /// **SEL_N** [12:8]: Shift register data bit select for the second half of the HSTX clock cycle
    #[inline(always)]
    pub fn sel_n(&self) -> u32 {
        (self.read_bits() & Self::SEL_N_MASK) >> Self::SEL_N_OFFSET
    }

    /// Write SEL_N field
    #[inline(always)]
    pub fn write_sel_n(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEL_N_MASK) | ((value << Self::SEL_N_OFFSET) & Self::SEL_N_MASK));
    }

    /// **SEL_P** [4:0]: Shift register data bit select for the first half of the HSTX clock cycle
    #[inline(always)]
    pub fn sel_p(&self) -> u32 {
        (self.read_bits() & Self::SEL_P_MASK) >> Self::SEL_P_OFFSET
    }

    /// Write SEL_P field
    #[inline(always)]
    pub fn write_sel_p(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEL_P_MASK) | ((value << Self::SEL_P_OFFSET) & Self::SEL_P_MASK));
    }

}

/// BIT1 value type for batch read/modify/write operations
///
/// Use `bit1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bit1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bit1Val {
    bits: u32,
}

impl Bit1Val {
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

    /// Read CLK [17:17]: Connect this output to the generated clock, rather than the data shift regi...
    #[inline(always)]
    pub const fn clk(&self) -> bool {
        (self.bits & Bit1::CLK_MASK) != 0
    }

    /// Read INV [16:16]: Invert this data output (logical NOT)
    #[inline(always)]
    pub const fn inv(&self) -> bool {
        (self.bits & Bit1::INV_MASK) != 0
    }

    /// Read SEL_N [12:8]: Shift register data bit select for the second half of the HSTX clock cycle
    #[inline(always)]
    pub const fn sel_n(&self) -> u32 {
        (self.bits & Bit1::SEL_N_MASK) >> Bit1::SEL_N_OFFSET
    }

    /// Read SEL_P [4:0]: Shift register data bit select for the first half of the HSTX clock cycle
    #[inline(always)]
    pub const fn sel_p(&self) -> u32 {
        (self.bits & Bit1::SEL_P_MASK) >> Bit1::SEL_P_OFFSET
    }

    /// Set CLK [17:17]: Connect this output to the generated clock, rather than the data shift regi...
    #[inline(always)]
    pub const fn with_clk(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bit1::CLK_MASK
            } else {
                self.bits & !Bit1::CLK_MASK
            }
        }
    }

    /// Set INV [16:16]: Invert this data output (logical NOT)
    #[inline(always)]
    pub const fn with_inv(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bit1::INV_MASK
            } else {
                self.bits & !Bit1::INV_MASK
            }
        }
    }

    /// Set SEL_N [12:8]: Shift register data bit select for the second half of the HSTX clock cycle
    #[inline(always)]
    pub const fn with_sel_n(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Bit1::SEL_N_MASK) | ((value << Bit1::SEL_N_OFFSET) & Bit1::SEL_N_MASK)
        }
    }

    /// Set SEL_P [4:0]: Shift register data bit select for the first half of the HSTX clock cycle
    #[inline(always)]
    pub const fn with_sel_p(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Bit1::SEL_P_MASK) | ((value << Bit1::SEL_P_OFFSET) & Bit1::SEL_P_MASK)
        }
    }

}

impl Default for Bit1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Bit1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> Bit1Val {
        Bit1Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: Bit1Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(Bit1Val) -> Bit1Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// BIT2 Register
///
/// Data control register for output bit 2
///
/// Address offset: 0x000c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CLK** [17] (RW): Connect this output to the generated clock, rather than the data shift regi...
/// - **INV** [16] (RW): Invert this data output (logical NOT)
/// - **SEL_N** [12:8] (RW): Shift register data bit select for the second half of the HSTX clock cycle
/// - **SEL_P** [4:0] (RW): Shift register data bit select for the first half of the HSTX clock cycle
#[repr(transparent)]
pub struct Bit2 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for Bit2 {}
unsafe impl Sync for Bit2 {}

impl Bit2 {
    // ======== Field Constants ========

    /// CLK - Connect this output to the generated clock, rather than the data shift regi...
    /// Bits [17:17], mask 0x00020000
    pub const CLK_OFFSET: u32 = 17;
    pub const CLK_MASK: u32 = 0x00020000;
    pub const CLK: u32 = 1 << 17;

    /// INV - Invert this data output (logical NOT)
    /// Bits [16:16], mask 0x00010000
    pub const INV_OFFSET: u32 = 16;
    pub const INV_MASK: u32 = 0x00010000;
    pub const INV: u32 = 1 << 16;

    /// SEL_N - Shift register data bit select for the second half of the HSTX clock cycle
    /// Bits [12:8], mask 0x00001f00
    pub const SEL_N_OFFSET: u32 = 8;
    pub const SEL_N_MASK: u32 = 0x00001f00;

    /// SEL_P - Shift register data bit select for the first half of the HSTX clock cycle
    /// Bits [4:0], mask 0x0000001f
    pub const SEL_P_OFFSET: u32 = 0;
    pub const SEL_P_MASK: u32 = 0x0000001f;

    // ======== Basic Access Methods ========

    /// Read the raw register value as u32
    #[inline(always)]
    pub fn read_bits(&self) -> u32 {
        unsafe { core::ptr::read_volatile(self.value.get()) }
    }

    /// Write a raw u32 value to the register
    #[inline(always)]
    pub fn write_bits(&self, value: u32) {
        unsafe { core::ptr::write_volatile(self.value.get(), value) }
    }

    /// Modify the raw register value
    #[inline(always)]
    pub fn modify_bits<F: FnOnce(u32) -> u32>(&self, f: F) {
        self.write_bits(f(self.read_bits()));
    }

    /// Atomic SET - sets bits that are 1 (hardware alias at +0x2000)
    #[inline(always)]
    pub fn set_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x2000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic CLR - clears bits that are 1 (hardware alias at +0x3000)
    #[inline(always)]
    pub fn clr_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x3000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic XOR - toggles bits that are 1 (hardware alias at +0x1000)
    #[inline(always)]
    pub fn xor_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x1000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    // ======== Field Accessors ========

    /// **CLK** [17:17]: Connect this output to the generated clock, rather than the data shift regi...
    #[inline(always)]
    pub fn clk(&self) -> bool {
        (self.read_bits() & Self::CLK_MASK) != 0
    }

    /// Set CLK bit
    #[inline(always)]
    pub fn set_clk(&self) {
        self.set_bits(Self::CLK_MASK);
    }

    /// Clear CLK bit
    #[inline(always)]
    pub fn clr_clk(&self) {
        self.clr_bits(Self::CLK_MASK);
    }

    /// **INV** [16:16]: Invert this data output (logical NOT)
    #[inline(always)]
    pub fn inv(&self) -> bool {
        (self.read_bits() & Self::INV_MASK) != 0
    }

    /// Set INV bit
    #[inline(always)]
    pub fn set_inv(&self) {
        self.set_bits(Self::INV_MASK);
    }

    /// Clear INV bit
    #[inline(always)]
    pub fn clr_inv(&self) {
        self.clr_bits(Self::INV_MASK);
    }

    /// **SEL_N** [12:8]: Shift register data bit select for the second half of the HSTX clock cycle
    #[inline(always)]
    pub fn sel_n(&self) -> u32 {
        (self.read_bits() & Self::SEL_N_MASK) >> Self::SEL_N_OFFSET
    }

    /// Write SEL_N field
    #[inline(always)]
    pub fn write_sel_n(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEL_N_MASK) | ((value << Self::SEL_N_OFFSET) & Self::SEL_N_MASK));
    }

    /// **SEL_P** [4:0]: Shift register data bit select for the first half of the HSTX clock cycle
    #[inline(always)]
    pub fn sel_p(&self) -> u32 {
        (self.read_bits() & Self::SEL_P_MASK) >> Self::SEL_P_OFFSET
    }

    /// Write SEL_P field
    #[inline(always)]
    pub fn write_sel_p(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEL_P_MASK) | ((value << Self::SEL_P_OFFSET) & Self::SEL_P_MASK));
    }

}

/// BIT2 value type for batch read/modify/write operations
///
/// Use `bit2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bit2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bit2Val {
    bits: u32,
}

impl Bit2Val {
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

    /// Read CLK [17:17]: Connect this output to the generated clock, rather than the data shift regi...
    #[inline(always)]
    pub const fn clk(&self) -> bool {
        (self.bits & Bit2::CLK_MASK) != 0
    }

    /// Read INV [16:16]: Invert this data output (logical NOT)
    #[inline(always)]
    pub const fn inv(&self) -> bool {
        (self.bits & Bit2::INV_MASK) != 0
    }

    /// Read SEL_N [12:8]: Shift register data bit select for the second half of the HSTX clock cycle
    #[inline(always)]
    pub const fn sel_n(&self) -> u32 {
        (self.bits & Bit2::SEL_N_MASK) >> Bit2::SEL_N_OFFSET
    }

    /// Read SEL_P [4:0]: Shift register data bit select for the first half of the HSTX clock cycle
    #[inline(always)]
    pub const fn sel_p(&self) -> u32 {
        (self.bits & Bit2::SEL_P_MASK) >> Bit2::SEL_P_OFFSET
    }

    /// Set CLK [17:17]: Connect this output to the generated clock, rather than the data shift regi...
    #[inline(always)]
    pub const fn with_clk(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bit2::CLK_MASK
            } else {
                self.bits & !Bit2::CLK_MASK
            }
        }
    }

    /// Set INV [16:16]: Invert this data output (logical NOT)
    #[inline(always)]
    pub const fn with_inv(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bit2::INV_MASK
            } else {
                self.bits & !Bit2::INV_MASK
            }
        }
    }

    /// Set SEL_N [12:8]: Shift register data bit select for the second half of the HSTX clock cycle
    #[inline(always)]
    pub const fn with_sel_n(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Bit2::SEL_N_MASK) | ((value << Bit2::SEL_N_OFFSET) & Bit2::SEL_N_MASK)
        }
    }

    /// Set SEL_P [4:0]: Shift register data bit select for the first half of the HSTX clock cycle
    #[inline(always)]
    pub const fn with_sel_p(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Bit2::SEL_P_MASK) | ((value << Bit2::SEL_P_OFFSET) & Bit2::SEL_P_MASK)
        }
    }

}

impl Default for Bit2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Bit2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> Bit2Val {
        Bit2Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: Bit2Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(Bit2Val) -> Bit2Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// BIT3 Register
///
/// Data control register for output bit 3
///
/// Address offset: 0x0010
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CLK** [17] (RW): Connect this output to the generated clock, rather than the data shift regi...
/// - **INV** [16] (RW): Invert this data output (logical NOT)
/// - **SEL_N** [12:8] (RW): Shift register data bit select for the second half of the HSTX clock cycle
/// - **SEL_P** [4:0] (RW): Shift register data bit select for the first half of the HSTX clock cycle
#[repr(transparent)]
pub struct Bit3 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for Bit3 {}
unsafe impl Sync for Bit3 {}

impl Bit3 {
    // ======== Field Constants ========

    /// CLK - Connect this output to the generated clock, rather than the data shift regi...
    /// Bits [17:17], mask 0x00020000
    pub const CLK_OFFSET: u32 = 17;
    pub const CLK_MASK: u32 = 0x00020000;
    pub const CLK: u32 = 1 << 17;

    /// INV - Invert this data output (logical NOT)
    /// Bits [16:16], mask 0x00010000
    pub const INV_OFFSET: u32 = 16;
    pub const INV_MASK: u32 = 0x00010000;
    pub const INV: u32 = 1 << 16;

    /// SEL_N - Shift register data bit select for the second half of the HSTX clock cycle
    /// Bits [12:8], mask 0x00001f00
    pub const SEL_N_OFFSET: u32 = 8;
    pub const SEL_N_MASK: u32 = 0x00001f00;

    /// SEL_P - Shift register data bit select for the first half of the HSTX clock cycle
    /// Bits [4:0], mask 0x0000001f
    pub const SEL_P_OFFSET: u32 = 0;
    pub const SEL_P_MASK: u32 = 0x0000001f;

    // ======== Basic Access Methods ========

    /// Read the raw register value as u32
    #[inline(always)]
    pub fn read_bits(&self) -> u32 {
        unsafe { core::ptr::read_volatile(self.value.get()) }
    }

    /// Write a raw u32 value to the register
    #[inline(always)]
    pub fn write_bits(&self, value: u32) {
        unsafe { core::ptr::write_volatile(self.value.get(), value) }
    }

    /// Modify the raw register value
    #[inline(always)]
    pub fn modify_bits<F: FnOnce(u32) -> u32>(&self, f: F) {
        self.write_bits(f(self.read_bits()));
    }

    /// Atomic SET - sets bits that are 1 (hardware alias at +0x2000)
    #[inline(always)]
    pub fn set_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x2000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic CLR - clears bits that are 1 (hardware alias at +0x3000)
    #[inline(always)]
    pub fn clr_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x3000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic XOR - toggles bits that are 1 (hardware alias at +0x1000)
    #[inline(always)]
    pub fn xor_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x1000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    // ======== Field Accessors ========

    /// **CLK** [17:17]: Connect this output to the generated clock, rather than the data shift regi...
    #[inline(always)]
    pub fn clk(&self) -> bool {
        (self.read_bits() & Self::CLK_MASK) != 0
    }

    /// Set CLK bit
    #[inline(always)]
    pub fn set_clk(&self) {
        self.set_bits(Self::CLK_MASK);
    }

    /// Clear CLK bit
    #[inline(always)]
    pub fn clr_clk(&self) {
        self.clr_bits(Self::CLK_MASK);
    }

    /// **INV** [16:16]: Invert this data output (logical NOT)
    #[inline(always)]
    pub fn inv(&self) -> bool {
        (self.read_bits() & Self::INV_MASK) != 0
    }

    /// Set INV bit
    #[inline(always)]
    pub fn set_inv(&self) {
        self.set_bits(Self::INV_MASK);
    }

    /// Clear INV bit
    #[inline(always)]
    pub fn clr_inv(&self) {
        self.clr_bits(Self::INV_MASK);
    }

    /// **SEL_N** [12:8]: Shift register data bit select for the second half of the HSTX clock cycle
    #[inline(always)]
    pub fn sel_n(&self) -> u32 {
        (self.read_bits() & Self::SEL_N_MASK) >> Self::SEL_N_OFFSET
    }

    /// Write SEL_N field
    #[inline(always)]
    pub fn write_sel_n(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEL_N_MASK) | ((value << Self::SEL_N_OFFSET) & Self::SEL_N_MASK));
    }

    /// **SEL_P** [4:0]: Shift register data bit select for the first half of the HSTX clock cycle
    #[inline(always)]
    pub fn sel_p(&self) -> u32 {
        (self.read_bits() & Self::SEL_P_MASK) >> Self::SEL_P_OFFSET
    }

    /// Write SEL_P field
    #[inline(always)]
    pub fn write_sel_p(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEL_P_MASK) | ((value << Self::SEL_P_OFFSET) & Self::SEL_P_MASK));
    }

}

/// BIT3 value type for batch read/modify/write operations
///
/// Use `bit3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bit3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bit3Val {
    bits: u32,
}

impl Bit3Val {
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

    /// Read CLK [17:17]: Connect this output to the generated clock, rather than the data shift regi...
    #[inline(always)]
    pub const fn clk(&self) -> bool {
        (self.bits & Bit3::CLK_MASK) != 0
    }

    /// Read INV [16:16]: Invert this data output (logical NOT)
    #[inline(always)]
    pub const fn inv(&self) -> bool {
        (self.bits & Bit3::INV_MASK) != 0
    }

    /// Read SEL_N [12:8]: Shift register data bit select for the second half of the HSTX clock cycle
    #[inline(always)]
    pub const fn sel_n(&self) -> u32 {
        (self.bits & Bit3::SEL_N_MASK) >> Bit3::SEL_N_OFFSET
    }

    /// Read SEL_P [4:0]: Shift register data bit select for the first half of the HSTX clock cycle
    #[inline(always)]
    pub const fn sel_p(&self) -> u32 {
        (self.bits & Bit3::SEL_P_MASK) >> Bit3::SEL_P_OFFSET
    }

    /// Set CLK [17:17]: Connect this output to the generated clock, rather than the data shift regi...
    #[inline(always)]
    pub const fn with_clk(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bit3::CLK_MASK
            } else {
                self.bits & !Bit3::CLK_MASK
            }
        }
    }

    /// Set INV [16:16]: Invert this data output (logical NOT)
    #[inline(always)]
    pub const fn with_inv(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bit3::INV_MASK
            } else {
                self.bits & !Bit3::INV_MASK
            }
        }
    }

    /// Set SEL_N [12:8]: Shift register data bit select for the second half of the HSTX clock cycle
    #[inline(always)]
    pub const fn with_sel_n(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Bit3::SEL_N_MASK) | ((value << Bit3::SEL_N_OFFSET) & Bit3::SEL_N_MASK)
        }
    }

    /// Set SEL_P [4:0]: Shift register data bit select for the first half of the HSTX clock cycle
    #[inline(always)]
    pub const fn with_sel_p(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Bit3::SEL_P_MASK) | ((value << Bit3::SEL_P_OFFSET) & Bit3::SEL_P_MASK)
        }
    }

}

impl Default for Bit3Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Bit3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> Bit3Val {
        Bit3Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: Bit3Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(Bit3Val) -> Bit3Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// BIT4 Register
///
/// Data control register for output bit 4
///
/// Address offset: 0x0014
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CLK** [17] (RW): Connect this output to the generated clock, rather than the data shift regi...
/// - **INV** [16] (RW): Invert this data output (logical NOT)
/// - **SEL_N** [12:8] (RW): Shift register data bit select for the second half of the HSTX clock cycle
/// - **SEL_P** [4:0] (RW): Shift register data bit select for the first half of the HSTX clock cycle
#[repr(transparent)]
pub struct Bit4 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for Bit4 {}
unsafe impl Sync for Bit4 {}

impl Bit4 {
    // ======== Field Constants ========

    /// CLK - Connect this output to the generated clock, rather than the data shift regi...
    /// Bits [17:17], mask 0x00020000
    pub const CLK_OFFSET: u32 = 17;
    pub const CLK_MASK: u32 = 0x00020000;
    pub const CLK: u32 = 1 << 17;

    /// INV - Invert this data output (logical NOT)
    /// Bits [16:16], mask 0x00010000
    pub const INV_OFFSET: u32 = 16;
    pub const INV_MASK: u32 = 0x00010000;
    pub const INV: u32 = 1 << 16;

    /// SEL_N - Shift register data bit select for the second half of the HSTX clock cycle
    /// Bits [12:8], mask 0x00001f00
    pub const SEL_N_OFFSET: u32 = 8;
    pub const SEL_N_MASK: u32 = 0x00001f00;

    /// SEL_P - Shift register data bit select for the first half of the HSTX clock cycle
    /// Bits [4:0], mask 0x0000001f
    pub const SEL_P_OFFSET: u32 = 0;
    pub const SEL_P_MASK: u32 = 0x0000001f;

    // ======== Basic Access Methods ========

    /// Read the raw register value as u32
    #[inline(always)]
    pub fn read_bits(&self) -> u32 {
        unsafe { core::ptr::read_volatile(self.value.get()) }
    }

    /// Write a raw u32 value to the register
    #[inline(always)]
    pub fn write_bits(&self, value: u32) {
        unsafe { core::ptr::write_volatile(self.value.get(), value) }
    }

    /// Modify the raw register value
    #[inline(always)]
    pub fn modify_bits<F: FnOnce(u32) -> u32>(&self, f: F) {
        self.write_bits(f(self.read_bits()));
    }

    /// Atomic SET - sets bits that are 1 (hardware alias at +0x2000)
    #[inline(always)]
    pub fn set_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x2000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic CLR - clears bits that are 1 (hardware alias at +0x3000)
    #[inline(always)]
    pub fn clr_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x3000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic XOR - toggles bits that are 1 (hardware alias at +0x1000)
    #[inline(always)]
    pub fn xor_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x1000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    // ======== Field Accessors ========

    /// **CLK** [17:17]: Connect this output to the generated clock, rather than the data shift regi...
    #[inline(always)]
    pub fn clk(&self) -> bool {
        (self.read_bits() & Self::CLK_MASK) != 0
    }

    /// Set CLK bit
    #[inline(always)]
    pub fn set_clk(&self) {
        self.set_bits(Self::CLK_MASK);
    }

    /// Clear CLK bit
    #[inline(always)]
    pub fn clr_clk(&self) {
        self.clr_bits(Self::CLK_MASK);
    }

    /// **INV** [16:16]: Invert this data output (logical NOT)
    #[inline(always)]
    pub fn inv(&self) -> bool {
        (self.read_bits() & Self::INV_MASK) != 0
    }

    /// Set INV bit
    #[inline(always)]
    pub fn set_inv(&self) {
        self.set_bits(Self::INV_MASK);
    }

    /// Clear INV bit
    #[inline(always)]
    pub fn clr_inv(&self) {
        self.clr_bits(Self::INV_MASK);
    }

    /// **SEL_N** [12:8]: Shift register data bit select for the second half of the HSTX clock cycle
    #[inline(always)]
    pub fn sel_n(&self) -> u32 {
        (self.read_bits() & Self::SEL_N_MASK) >> Self::SEL_N_OFFSET
    }

    /// Write SEL_N field
    #[inline(always)]
    pub fn write_sel_n(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEL_N_MASK) | ((value << Self::SEL_N_OFFSET) & Self::SEL_N_MASK));
    }

    /// **SEL_P** [4:0]: Shift register data bit select for the first half of the HSTX clock cycle
    #[inline(always)]
    pub fn sel_p(&self) -> u32 {
        (self.read_bits() & Self::SEL_P_MASK) >> Self::SEL_P_OFFSET
    }

    /// Write SEL_P field
    #[inline(always)]
    pub fn write_sel_p(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEL_P_MASK) | ((value << Self::SEL_P_OFFSET) & Self::SEL_P_MASK));
    }

}

/// BIT4 value type for batch read/modify/write operations
///
/// Use `bit4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bit4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bit4Val {
    bits: u32,
}

impl Bit4Val {
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

    /// Read CLK [17:17]: Connect this output to the generated clock, rather than the data shift regi...
    #[inline(always)]
    pub const fn clk(&self) -> bool {
        (self.bits & Bit4::CLK_MASK) != 0
    }

    /// Read INV [16:16]: Invert this data output (logical NOT)
    #[inline(always)]
    pub const fn inv(&self) -> bool {
        (self.bits & Bit4::INV_MASK) != 0
    }

    /// Read SEL_N [12:8]: Shift register data bit select for the second half of the HSTX clock cycle
    #[inline(always)]
    pub const fn sel_n(&self) -> u32 {
        (self.bits & Bit4::SEL_N_MASK) >> Bit4::SEL_N_OFFSET
    }

    /// Read SEL_P [4:0]: Shift register data bit select for the first half of the HSTX clock cycle
    #[inline(always)]
    pub const fn sel_p(&self) -> u32 {
        (self.bits & Bit4::SEL_P_MASK) >> Bit4::SEL_P_OFFSET
    }

    /// Set CLK [17:17]: Connect this output to the generated clock, rather than the data shift regi...
    #[inline(always)]
    pub const fn with_clk(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bit4::CLK_MASK
            } else {
                self.bits & !Bit4::CLK_MASK
            }
        }
    }

    /// Set INV [16:16]: Invert this data output (logical NOT)
    #[inline(always)]
    pub const fn with_inv(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bit4::INV_MASK
            } else {
                self.bits & !Bit4::INV_MASK
            }
        }
    }

    /// Set SEL_N [12:8]: Shift register data bit select for the second half of the HSTX clock cycle
    #[inline(always)]
    pub const fn with_sel_n(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Bit4::SEL_N_MASK) | ((value << Bit4::SEL_N_OFFSET) & Bit4::SEL_N_MASK)
        }
    }

    /// Set SEL_P [4:0]: Shift register data bit select for the first half of the HSTX clock cycle
    #[inline(always)]
    pub const fn with_sel_p(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Bit4::SEL_P_MASK) | ((value << Bit4::SEL_P_OFFSET) & Bit4::SEL_P_MASK)
        }
    }

}

impl Default for Bit4Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Bit4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> Bit4Val {
        Bit4Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: Bit4Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(Bit4Val) -> Bit4Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// BIT5 Register
///
/// Data control register for output bit 5
///
/// Address offset: 0x0018
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CLK** [17] (RW): Connect this output to the generated clock, rather than the data shift regi...
/// - **INV** [16] (RW): Invert this data output (logical NOT)
/// - **SEL_N** [12:8] (RW): Shift register data bit select for the second half of the HSTX clock cycle
/// - **SEL_P** [4:0] (RW): Shift register data bit select for the first half of the HSTX clock cycle
#[repr(transparent)]
pub struct Bit5 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for Bit5 {}
unsafe impl Sync for Bit5 {}

impl Bit5 {
    // ======== Field Constants ========

    /// CLK - Connect this output to the generated clock, rather than the data shift regi...
    /// Bits [17:17], mask 0x00020000
    pub const CLK_OFFSET: u32 = 17;
    pub const CLK_MASK: u32 = 0x00020000;
    pub const CLK: u32 = 1 << 17;

    /// INV - Invert this data output (logical NOT)
    /// Bits [16:16], mask 0x00010000
    pub const INV_OFFSET: u32 = 16;
    pub const INV_MASK: u32 = 0x00010000;
    pub const INV: u32 = 1 << 16;

    /// SEL_N - Shift register data bit select for the second half of the HSTX clock cycle
    /// Bits [12:8], mask 0x00001f00
    pub const SEL_N_OFFSET: u32 = 8;
    pub const SEL_N_MASK: u32 = 0x00001f00;

    /// SEL_P - Shift register data bit select for the first half of the HSTX clock cycle
    /// Bits [4:0], mask 0x0000001f
    pub const SEL_P_OFFSET: u32 = 0;
    pub const SEL_P_MASK: u32 = 0x0000001f;

    // ======== Basic Access Methods ========

    /// Read the raw register value as u32
    #[inline(always)]
    pub fn read_bits(&self) -> u32 {
        unsafe { core::ptr::read_volatile(self.value.get()) }
    }

    /// Write a raw u32 value to the register
    #[inline(always)]
    pub fn write_bits(&self, value: u32) {
        unsafe { core::ptr::write_volatile(self.value.get(), value) }
    }

    /// Modify the raw register value
    #[inline(always)]
    pub fn modify_bits<F: FnOnce(u32) -> u32>(&self, f: F) {
        self.write_bits(f(self.read_bits()));
    }

    /// Atomic SET - sets bits that are 1 (hardware alias at +0x2000)
    #[inline(always)]
    pub fn set_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x2000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic CLR - clears bits that are 1 (hardware alias at +0x3000)
    #[inline(always)]
    pub fn clr_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x3000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic XOR - toggles bits that are 1 (hardware alias at +0x1000)
    #[inline(always)]
    pub fn xor_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x1000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    // ======== Field Accessors ========

    /// **CLK** [17:17]: Connect this output to the generated clock, rather than the data shift regi...
    #[inline(always)]
    pub fn clk(&self) -> bool {
        (self.read_bits() & Self::CLK_MASK) != 0
    }

    /// Set CLK bit
    #[inline(always)]
    pub fn set_clk(&self) {
        self.set_bits(Self::CLK_MASK);
    }

    /// Clear CLK bit
    #[inline(always)]
    pub fn clr_clk(&self) {
        self.clr_bits(Self::CLK_MASK);
    }

    /// **INV** [16:16]: Invert this data output (logical NOT)
    #[inline(always)]
    pub fn inv(&self) -> bool {
        (self.read_bits() & Self::INV_MASK) != 0
    }

    /// Set INV bit
    #[inline(always)]
    pub fn set_inv(&self) {
        self.set_bits(Self::INV_MASK);
    }

    /// Clear INV bit
    #[inline(always)]
    pub fn clr_inv(&self) {
        self.clr_bits(Self::INV_MASK);
    }

    /// **SEL_N** [12:8]: Shift register data bit select for the second half of the HSTX clock cycle
    #[inline(always)]
    pub fn sel_n(&self) -> u32 {
        (self.read_bits() & Self::SEL_N_MASK) >> Self::SEL_N_OFFSET
    }

    /// Write SEL_N field
    #[inline(always)]
    pub fn write_sel_n(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEL_N_MASK) | ((value << Self::SEL_N_OFFSET) & Self::SEL_N_MASK));
    }

    /// **SEL_P** [4:0]: Shift register data bit select for the first half of the HSTX clock cycle
    #[inline(always)]
    pub fn sel_p(&self) -> u32 {
        (self.read_bits() & Self::SEL_P_MASK) >> Self::SEL_P_OFFSET
    }

    /// Write SEL_P field
    #[inline(always)]
    pub fn write_sel_p(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEL_P_MASK) | ((value << Self::SEL_P_OFFSET) & Self::SEL_P_MASK));
    }

}

/// BIT5 value type for batch read/modify/write operations
///
/// Use `bit5.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bit5.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bit5Val {
    bits: u32,
}

impl Bit5Val {
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

    /// Read CLK [17:17]: Connect this output to the generated clock, rather than the data shift regi...
    #[inline(always)]
    pub const fn clk(&self) -> bool {
        (self.bits & Bit5::CLK_MASK) != 0
    }

    /// Read INV [16:16]: Invert this data output (logical NOT)
    #[inline(always)]
    pub const fn inv(&self) -> bool {
        (self.bits & Bit5::INV_MASK) != 0
    }

    /// Read SEL_N [12:8]: Shift register data bit select for the second half of the HSTX clock cycle
    #[inline(always)]
    pub const fn sel_n(&self) -> u32 {
        (self.bits & Bit5::SEL_N_MASK) >> Bit5::SEL_N_OFFSET
    }

    /// Read SEL_P [4:0]: Shift register data bit select for the first half of the HSTX clock cycle
    #[inline(always)]
    pub const fn sel_p(&self) -> u32 {
        (self.bits & Bit5::SEL_P_MASK) >> Bit5::SEL_P_OFFSET
    }

    /// Set CLK [17:17]: Connect this output to the generated clock, rather than the data shift regi...
    #[inline(always)]
    pub const fn with_clk(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bit5::CLK_MASK
            } else {
                self.bits & !Bit5::CLK_MASK
            }
        }
    }

    /// Set INV [16:16]: Invert this data output (logical NOT)
    #[inline(always)]
    pub const fn with_inv(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bit5::INV_MASK
            } else {
                self.bits & !Bit5::INV_MASK
            }
        }
    }

    /// Set SEL_N [12:8]: Shift register data bit select for the second half of the HSTX clock cycle
    #[inline(always)]
    pub const fn with_sel_n(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Bit5::SEL_N_MASK) | ((value << Bit5::SEL_N_OFFSET) & Bit5::SEL_N_MASK)
        }
    }

    /// Set SEL_P [4:0]: Shift register data bit select for the first half of the HSTX clock cycle
    #[inline(always)]
    pub const fn with_sel_p(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Bit5::SEL_P_MASK) | ((value << Bit5::SEL_P_OFFSET) & Bit5::SEL_P_MASK)
        }
    }

}

impl Default for Bit5Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Bit5 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> Bit5Val {
        Bit5Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: Bit5Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(Bit5Val) -> Bit5Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// BIT6 Register
///
/// Data control register for output bit 6
///
/// Address offset: 0x001c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CLK** [17] (RW): Connect this output to the generated clock, rather than the data shift regi...
/// - **INV** [16] (RW): Invert this data output (logical NOT)
/// - **SEL_N** [12:8] (RW): Shift register data bit select for the second half of the HSTX clock cycle
/// - **SEL_P** [4:0] (RW): Shift register data bit select for the first half of the HSTX clock cycle
#[repr(transparent)]
pub struct Bit6 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for Bit6 {}
unsafe impl Sync for Bit6 {}

impl Bit6 {
    // ======== Field Constants ========

    /// CLK - Connect this output to the generated clock, rather than the data shift regi...
    /// Bits [17:17], mask 0x00020000
    pub const CLK_OFFSET: u32 = 17;
    pub const CLK_MASK: u32 = 0x00020000;
    pub const CLK: u32 = 1 << 17;

    /// INV - Invert this data output (logical NOT)
    /// Bits [16:16], mask 0x00010000
    pub const INV_OFFSET: u32 = 16;
    pub const INV_MASK: u32 = 0x00010000;
    pub const INV: u32 = 1 << 16;

    /// SEL_N - Shift register data bit select for the second half of the HSTX clock cycle
    /// Bits [12:8], mask 0x00001f00
    pub const SEL_N_OFFSET: u32 = 8;
    pub const SEL_N_MASK: u32 = 0x00001f00;

    /// SEL_P - Shift register data bit select for the first half of the HSTX clock cycle
    /// Bits [4:0], mask 0x0000001f
    pub const SEL_P_OFFSET: u32 = 0;
    pub const SEL_P_MASK: u32 = 0x0000001f;

    // ======== Basic Access Methods ========

    /// Read the raw register value as u32
    #[inline(always)]
    pub fn read_bits(&self) -> u32 {
        unsafe { core::ptr::read_volatile(self.value.get()) }
    }

    /// Write a raw u32 value to the register
    #[inline(always)]
    pub fn write_bits(&self, value: u32) {
        unsafe { core::ptr::write_volatile(self.value.get(), value) }
    }

    /// Modify the raw register value
    #[inline(always)]
    pub fn modify_bits<F: FnOnce(u32) -> u32>(&self, f: F) {
        self.write_bits(f(self.read_bits()));
    }

    /// Atomic SET - sets bits that are 1 (hardware alias at +0x2000)
    #[inline(always)]
    pub fn set_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x2000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic CLR - clears bits that are 1 (hardware alias at +0x3000)
    #[inline(always)]
    pub fn clr_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x3000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic XOR - toggles bits that are 1 (hardware alias at +0x1000)
    #[inline(always)]
    pub fn xor_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x1000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    // ======== Field Accessors ========

    /// **CLK** [17:17]: Connect this output to the generated clock, rather than the data shift regi...
    #[inline(always)]
    pub fn clk(&self) -> bool {
        (self.read_bits() & Self::CLK_MASK) != 0
    }

    /// Set CLK bit
    #[inline(always)]
    pub fn set_clk(&self) {
        self.set_bits(Self::CLK_MASK);
    }

    /// Clear CLK bit
    #[inline(always)]
    pub fn clr_clk(&self) {
        self.clr_bits(Self::CLK_MASK);
    }

    /// **INV** [16:16]: Invert this data output (logical NOT)
    #[inline(always)]
    pub fn inv(&self) -> bool {
        (self.read_bits() & Self::INV_MASK) != 0
    }

    /// Set INV bit
    #[inline(always)]
    pub fn set_inv(&self) {
        self.set_bits(Self::INV_MASK);
    }

    /// Clear INV bit
    #[inline(always)]
    pub fn clr_inv(&self) {
        self.clr_bits(Self::INV_MASK);
    }

    /// **SEL_N** [12:8]: Shift register data bit select for the second half of the HSTX clock cycle
    #[inline(always)]
    pub fn sel_n(&self) -> u32 {
        (self.read_bits() & Self::SEL_N_MASK) >> Self::SEL_N_OFFSET
    }

    /// Write SEL_N field
    #[inline(always)]
    pub fn write_sel_n(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEL_N_MASK) | ((value << Self::SEL_N_OFFSET) & Self::SEL_N_MASK));
    }

    /// **SEL_P** [4:0]: Shift register data bit select for the first half of the HSTX clock cycle
    #[inline(always)]
    pub fn sel_p(&self) -> u32 {
        (self.read_bits() & Self::SEL_P_MASK) >> Self::SEL_P_OFFSET
    }

    /// Write SEL_P field
    #[inline(always)]
    pub fn write_sel_p(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEL_P_MASK) | ((value << Self::SEL_P_OFFSET) & Self::SEL_P_MASK));
    }

}

/// BIT6 value type for batch read/modify/write operations
///
/// Use `bit6.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bit6.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bit6Val {
    bits: u32,
}

impl Bit6Val {
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

    /// Read CLK [17:17]: Connect this output to the generated clock, rather than the data shift regi...
    #[inline(always)]
    pub const fn clk(&self) -> bool {
        (self.bits & Bit6::CLK_MASK) != 0
    }

    /// Read INV [16:16]: Invert this data output (logical NOT)
    #[inline(always)]
    pub const fn inv(&self) -> bool {
        (self.bits & Bit6::INV_MASK) != 0
    }

    /// Read SEL_N [12:8]: Shift register data bit select for the second half of the HSTX clock cycle
    #[inline(always)]
    pub const fn sel_n(&self) -> u32 {
        (self.bits & Bit6::SEL_N_MASK) >> Bit6::SEL_N_OFFSET
    }

    /// Read SEL_P [4:0]: Shift register data bit select for the first half of the HSTX clock cycle
    #[inline(always)]
    pub const fn sel_p(&self) -> u32 {
        (self.bits & Bit6::SEL_P_MASK) >> Bit6::SEL_P_OFFSET
    }

    /// Set CLK [17:17]: Connect this output to the generated clock, rather than the data shift regi...
    #[inline(always)]
    pub const fn with_clk(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bit6::CLK_MASK
            } else {
                self.bits & !Bit6::CLK_MASK
            }
        }
    }

    /// Set INV [16:16]: Invert this data output (logical NOT)
    #[inline(always)]
    pub const fn with_inv(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bit6::INV_MASK
            } else {
                self.bits & !Bit6::INV_MASK
            }
        }
    }

    /// Set SEL_N [12:8]: Shift register data bit select for the second half of the HSTX clock cycle
    #[inline(always)]
    pub const fn with_sel_n(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Bit6::SEL_N_MASK) | ((value << Bit6::SEL_N_OFFSET) & Bit6::SEL_N_MASK)
        }
    }

    /// Set SEL_P [4:0]: Shift register data bit select for the first half of the HSTX clock cycle
    #[inline(always)]
    pub const fn with_sel_p(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Bit6::SEL_P_MASK) | ((value << Bit6::SEL_P_OFFSET) & Bit6::SEL_P_MASK)
        }
    }

}

impl Default for Bit6Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Bit6 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> Bit6Val {
        Bit6Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: Bit6Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(Bit6Val) -> Bit6Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// BIT7 Register
///
/// Data control register for output bit 7
///
/// Address offset: 0x0020
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CLK** [17] (RW): Connect this output to the generated clock, rather than the data shift regi...
/// - **INV** [16] (RW): Invert this data output (logical NOT)
/// - **SEL_N** [12:8] (RW): Shift register data bit select for the second half of the HSTX clock cycle
/// - **SEL_P** [4:0] (RW): Shift register data bit select for the first half of the HSTX clock cycle
#[repr(transparent)]
pub struct Bit7 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for Bit7 {}
unsafe impl Sync for Bit7 {}

impl Bit7 {
    // ======== Field Constants ========

    /// CLK - Connect this output to the generated clock, rather than the data shift regi...
    /// Bits [17:17], mask 0x00020000
    pub const CLK_OFFSET: u32 = 17;
    pub const CLK_MASK: u32 = 0x00020000;
    pub const CLK: u32 = 1 << 17;

    /// INV - Invert this data output (logical NOT)
    /// Bits [16:16], mask 0x00010000
    pub const INV_OFFSET: u32 = 16;
    pub const INV_MASK: u32 = 0x00010000;
    pub const INV: u32 = 1 << 16;

    /// SEL_N - Shift register data bit select for the second half of the HSTX clock cycle
    /// Bits [12:8], mask 0x00001f00
    pub const SEL_N_OFFSET: u32 = 8;
    pub const SEL_N_MASK: u32 = 0x00001f00;

    /// SEL_P - Shift register data bit select for the first half of the HSTX clock cycle
    /// Bits [4:0], mask 0x0000001f
    pub const SEL_P_OFFSET: u32 = 0;
    pub const SEL_P_MASK: u32 = 0x0000001f;

    // ======== Basic Access Methods ========

    /// Read the raw register value as u32
    #[inline(always)]
    pub fn read_bits(&self) -> u32 {
        unsafe { core::ptr::read_volatile(self.value.get()) }
    }

    /// Write a raw u32 value to the register
    #[inline(always)]
    pub fn write_bits(&self, value: u32) {
        unsafe { core::ptr::write_volatile(self.value.get(), value) }
    }

    /// Modify the raw register value
    #[inline(always)]
    pub fn modify_bits<F: FnOnce(u32) -> u32>(&self, f: F) {
        self.write_bits(f(self.read_bits()));
    }

    /// Atomic SET - sets bits that are 1 (hardware alias at +0x2000)
    #[inline(always)]
    pub fn set_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x2000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic CLR - clears bits that are 1 (hardware alias at +0x3000)
    #[inline(always)]
    pub fn clr_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x3000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic XOR - toggles bits that are 1 (hardware alias at +0x1000)
    #[inline(always)]
    pub fn xor_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x1000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    // ======== Field Accessors ========

    /// **CLK** [17:17]: Connect this output to the generated clock, rather than the data shift regi...
    #[inline(always)]
    pub fn clk(&self) -> bool {
        (self.read_bits() & Self::CLK_MASK) != 0
    }

    /// Set CLK bit
    #[inline(always)]
    pub fn set_clk(&self) {
        self.set_bits(Self::CLK_MASK);
    }

    /// Clear CLK bit
    #[inline(always)]
    pub fn clr_clk(&self) {
        self.clr_bits(Self::CLK_MASK);
    }

    /// **INV** [16:16]: Invert this data output (logical NOT)
    #[inline(always)]
    pub fn inv(&self) -> bool {
        (self.read_bits() & Self::INV_MASK) != 0
    }

    /// Set INV bit
    #[inline(always)]
    pub fn set_inv(&self) {
        self.set_bits(Self::INV_MASK);
    }

    /// Clear INV bit
    #[inline(always)]
    pub fn clr_inv(&self) {
        self.clr_bits(Self::INV_MASK);
    }

    /// **SEL_N** [12:8]: Shift register data bit select for the second half of the HSTX clock cycle
    #[inline(always)]
    pub fn sel_n(&self) -> u32 {
        (self.read_bits() & Self::SEL_N_MASK) >> Self::SEL_N_OFFSET
    }

    /// Write SEL_N field
    #[inline(always)]
    pub fn write_sel_n(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEL_N_MASK) | ((value << Self::SEL_N_OFFSET) & Self::SEL_N_MASK));
    }

    /// **SEL_P** [4:0]: Shift register data bit select for the first half of the HSTX clock cycle
    #[inline(always)]
    pub fn sel_p(&self) -> u32 {
        (self.read_bits() & Self::SEL_P_MASK) >> Self::SEL_P_OFFSET
    }

    /// Write SEL_P field
    #[inline(always)]
    pub fn write_sel_p(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::SEL_P_MASK) | ((value << Self::SEL_P_OFFSET) & Self::SEL_P_MASK));
    }

}

/// BIT7 value type for batch read/modify/write operations
///
/// Use `bit7.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bit7.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bit7Val {
    bits: u32,
}

impl Bit7Val {
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

    /// Read CLK [17:17]: Connect this output to the generated clock, rather than the data shift regi...
    #[inline(always)]
    pub const fn clk(&self) -> bool {
        (self.bits & Bit7::CLK_MASK) != 0
    }

    /// Read INV [16:16]: Invert this data output (logical NOT)
    #[inline(always)]
    pub const fn inv(&self) -> bool {
        (self.bits & Bit7::INV_MASK) != 0
    }

    /// Read SEL_N [12:8]: Shift register data bit select for the second half of the HSTX clock cycle
    #[inline(always)]
    pub const fn sel_n(&self) -> u32 {
        (self.bits & Bit7::SEL_N_MASK) >> Bit7::SEL_N_OFFSET
    }

    /// Read SEL_P [4:0]: Shift register data bit select for the first half of the HSTX clock cycle
    #[inline(always)]
    pub const fn sel_p(&self) -> u32 {
        (self.bits & Bit7::SEL_P_MASK) >> Bit7::SEL_P_OFFSET
    }

    /// Set CLK [17:17]: Connect this output to the generated clock, rather than the data shift regi...
    #[inline(always)]
    pub const fn with_clk(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bit7::CLK_MASK
            } else {
                self.bits & !Bit7::CLK_MASK
            }
        }
    }

    /// Set INV [16:16]: Invert this data output (logical NOT)
    #[inline(always)]
    pub const fn with_inv(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bit7::INV_MASK
            } else {
                self.bits & !Bit7::INV_MASK
            }
        }
    }

    /// Set SEL_N [12:8]: Shift register data bit select for the second half of the HSTX clock cycle
    #[inline(always)]
    pub const fn with_sel_n(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Bit7::SEL_N_MASK) | ((value << Bit7::SEL_N_OFFSET) & Bit7::SEL_N_MASK)
        }
    }

    /// Set SEL_P [4:0]: Shift register data bit select for the first half of the HSTX clock cycle
    #[inline(always)]
    pub const fn with_sel_p(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Bit7::SEL_P_MASK) | ((value << Bit7::SEL_P_OFFSET) & Bit7::SEL_P_MASK)
        }
    }

}

impl Default for Bit7Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Bit7 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> Bit7Val {
        Bit7Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: Bit7Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(Bit7Val) -> Bit7Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// EXPAND_SHIFT Register
///
/// Configure the optional shifter inside the command expander
///
/// Address offset: 0x0024
/// Reset value: 0x01000100
///
/// ## Fields
///
/// - **ENC_N_SHIFTS** [28:24] (RW): Number of times to consume from the shift register before refilling it from...
/// - **ENC_SHIFT** [20:16] (RW): How many bits to right-rotate the shift register by each time data is pushe...
/// - **RAW_N_SHIFTS** [12:8] (RW): Number of times to consume from the shift register before refilling it from...
/// - **RAW_SHIFT** [4:0] (RW): How many bits to right-rotate the shift register by each time data is pushe...
#[repr(transparent)]
pub struct ExpandShift {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for ExpandShift {}
unsafe impl Sync for ExpandShift {}

impl ExpandShift {
    // ======== Field Constants ========

    /// ENC_N_SHIFTS - Number of times to consume from the shift register before refilling it from...
    /// Bits [28:24], mask 0x1f000000
    pub const ENC_N_SHIFTS_OFFSET: u32 = 24;
    pub const ENC_N_SHIFTS_MASK: u32 = 0x1f000000;

    /// ENC_SHIFT - How many bits to right-rotate the shift register by each time data is pushe...
    /// Bits [20:16], mask 0x001f0000
    pub const ENC_SHIFT_OFFSET: u32 = 16;
    pub const ENC_SHIFT_MASK: u32 = 0x001f0000;

    /// RAW_N_SHIFTS - Number of times to consume from the shift register before refilling it from...
    /// Bits [12:8], mask 0x00001f00
    pub const RAW_N_SHIFTS_OFFSET: u32 = 8;
    pub const RAW_N_SHIFTS_MASK: u32 = 0x00001f00;

    /// RAW_SHIFT - How many bits to right-rotate the shift register by each time data is pushe...
    /// Bits [4:0], mask 0x0000001f
    pub const RAW_SHIFT_OFFSET: u32 = 0;
    pub const RAW_SHIFT_MASK: u32 = 0x0000001f;

    // ======== Basic Access Methods ========

    /// Read the raw register value as u32
    #[inline(always)]
    pub fn read_bits(&self) -> u32 {
        unsafe { core::ptr::read_volatile(self.value.get()) }
    }

    /// Write a raw u32 value to the register
    #[inline(always)]
    pub fn write_bits(&self, value: u32) {
        unsafe { core::ptr::write_volatile(self.value.get(), value) }
    }

    /// Modify the raw register value
    #[inline(always)]
    pub fn modify_bits<F: FnOnce(u32) -> u32>(&self, f: F) {
        self.write_bits(f(self.read_bits()));
    }

    /// Atomic SET - sets bits that are 1 (hardware alias at +0x2000)
    #[inline(always)]
    pub fn set_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x2000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic CLR - clears bits that are 1 (hardware alias at +0x3000)
    #[inline(always)]
    pub fn clr_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x3000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic XOR - toggles bits that are 1 (hardware alias at +0x1000)
    #[inline(always)]
    pub fn xor_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x1000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    // ======== Field Accessors ========

    /// **ENC_N_SHIFTS** [28:24]: Number of times to consume from the shift register before refilling it from...
    #[inline(always)]
    pub fn enc_n_shifts(&self) -> u32 {
        (self.read_bits() & Self::ENC_N_SHIFTS_MASK) >> Self::ENC_N_SHIFTS_OFFSET
    }

    /// Write ENC_N_SHIFTS field
    #[inline(always)]
    pub fn write_enc_n_shifts(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::ENC_N_SHIFTS_MASK) | ((value << Self::ENC_N_SHIFTS_OFFSET) & Self::ENC_N_SHIFTS_MASK));
    }

    /// **ENC_SHIFT** [20:16]: How many bits to right-rotate the shift register by each time data is pushe...
    #[inline(always)]
    pub fn enc_shift(&self) -> u32 {
        (self.read_bits() & Self::ENC_SHIFT_MASK) >> Self::ENC_SHIFT_OFFSET
    }

    /// Write ENC_SHIFT field
    #[inline(always)]
    pub fn write_enc_shift(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::ENC_SHIFT_MASK) | ((value << Self::ENC_SHIFT_OFFSET) & Self::ENC_SHIFT_MASK));
    }

    /// **RAW_N_SHIFTS** [12:8]: Number of times to consume from the shift register before refilling it from...
    #[inline(always)]
    pub fn raw_n_shifts(&self) -> u32 {
        (self.read_bits() & Self::RAW_N_SHIFTS_MASK) >> Self::RAW_N_SHIFTS_OFFSET
    }

    /// Write RAW_N_SHIFTS field
    #[inline(always)]
    pub fn write_raw_n_shifts(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::RAW_N_SHIFTS_MASK) | ((value << Self::RAW_N_SHIFTS_OFFSET) & Self::RAW_N_SHIFTS_MASK));
    }

    /// **RAW_SHIFT** [4:0]: How many bits to right-rotate the shift register by each time data is pushe...
    #[inline(always)]
    pub fn raw_shift(&self) -> u32 {
        (self.read_bits() & Self::RAW_SHIFT_MASK) >> Self::RAW_SHIFT_OFFSET
    }

    /// Write RAW_SHIFT field
    #[inline(always)]
    pub fn write_raw_shift(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::RAW_SHIFT_MASK) | ((value << Self::RAW_SHIFT_OFFSET) & Self::RAW_SHIFT_MASK));
    }

}

/// EXPAND_SHIFT value type for batch read/modify/write operations
///
/// Use `expandshift.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `expandshift.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ExpandShiftVal {
    bits: u32,
}

impl ExpandShiftVal {
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
        Self { bits: 0x01000100 }
    }

    /// Read ENC_N_SHIFTS [28:24]: Number of times to consume from the shift register before refilling it from...
    #[inline(always)]
    pub const fn enc_n_shifts(&self) -> u32 {
        (self.bits & ExpandShift::ENC_N_SHIFTS_MASK) >> ExpandShift::ENC_N_SHIFTS_OFFSET
    }

    /// Read ENC_SHIFT [20:16]: How many bits to right-rotate the shift register by each time data is pushe...
    #[inline(always)]
    pub const fn enc_shift(&self) -> u32 {
        (self.bits & ExpandShift::ENC_SHIFT_MASK) >> ExpandShift::ENC_SHIFT_OFFSET
    }

    /// Read RAW_N_SHIFTS [12:8]: Number of times to consume from the shift register before refilling it from...
    #[inline(always)]
    pub const fn raw_n_shifts(&self) -> u32 {
        (self.bits & ExpandShift::RAW_N_SHIFTS_MASK) >> ExpandShift::RAW_N_SHIFTS_OFFSET
    }

    /// Read RAW_SHIFT [4:0]: How many bits to right-rotate the shift register by each time data is pushe...
    #[inline(always)]
    pub const fn raw_shift(&self) -> u32 {
        (self.bits & ExpandShift::RAW_SHIFT_MASK) >> ExpandShift::RAW_SHIFT_OFFSET
    }

    /// Set ENC_N_SHIFTS [28:24]: Number of times to consume from the shift register before refilling it from...
    #[inline(always)]
    pub const fn with_enc_n_shifts(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ExpandShift::ENC_N_SHIFTS_MASK) | ((value << ExpandShift::ENC_N_SHIFTS_OFFSET) & ExpandShift::ENC_N_SHIFTS_MASK)
        }
    }

    /// Set ENC_SHIFT [20:16]: How many bits to right-rotate the shift register by each time data is pushe...
    #[inline(always)]
    pub const fn with_enc_shift(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ExpandShift::ENC_SHIFT_MASK) | ((value << ExpandShift::ENC_SHIFT_OFFSET) & ExpandShift::ENC_SHIFT_MASK)
        }
    }

    /// Set RAW_N_SHIFTS [12:8]: Number of times to consume from the shift register before refilling it from...
    #[inline(always)]
    pub const fn with_raw_n_shifts(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ExpandShift::RAW_N_SHIFTS_MASK) | ((value << ExpandShift::RAW_N_SHIFTS_OFFSET) & ExpandShift::RAW_N_SHIFTS_MASK)
        }
    }

    /// Set RAW_SHIFT [4:0]: How many bits to right-rotate the shift register by each time data is pushe...
    #[inline(always)]
    pub const fn with_raw_shift(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ExpandShift::RAW_SHIFT_MASK) | ((value << ExpandShift::RAW_SHIFT_OFFSET) & ExpandShift::RAW_SHIFT_MASK)
        }
    }

}

impl Default for ExpandShiftVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl ExpandShift {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> ExpandShiftVal {
        ExpandShiftVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: ExpandShiftVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(ExpandShiftVal) -> ExpandShiftVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// EXPAND_TMDS Register
///
/// Configure the optional TMDS encoder inside the command expander
///
/// Address offset: 0x0028
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **L2_NBITS** [23:21] (RW): Number of valid data bits for the lane 2 TMDS encoder, starting from bit 7 ...
/// - **L2_ROT** [20:16] (RW): Right-rotate applied to the current shifter data before the lane 2 TMDS enc...
/// - **L1_NBITS** [15:13] (RW): Number of valid data bits for the lane 1 TMDS encoder, starting from bit 7 ...
/// - **L1_ROT** [12:8] (RW): Right-rotate applied to the current shifter data before the lane 1 TMDS enc...
/// - **L0_NBITS** [7:5] (RW): Number of valid data bits for the lane 0 TMDS encoder, starting from bit 7 ...
/// - **L0_ROT** [4:0] (RW): Right-rotate applied to the current shifter data before the lane 0 TMDS enc...
#[repr(transparent)]
pub struct ExpandTmds {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for ExpandTmds {}
unsafe impl Sync for ExpandTmds {}

impl ExpandTmds {
    // ======== Field Constants ========

    /// L2_NBITS - Number of valid data bits for the lane 2 TMDS encoder, starting from bit 7 ...
    /// Bits [23:21], mask 0x00e00000
    pub const L2_NBITS_OFFSET: u32 = 21;
    pub const L2_NBITS_MASK: u32 = 0x00e00000;

    /// L2_ROT - Right-rotate applied to the current shifter data before the lane 2 TMDS enc...
    /// Bits [20:16], mask 0x001f0000
    pub const L2_ROT_OFFSET: u32 = 16;
    pub const L2_ROT_MASK: u32 = 0x001f0000;

    /// L1_NBITS - Number of valid data bits for the lane 1 TMDS encoder, starting from bit 7 ...
    /// Bits [15:13], mask 0x0000e000
    pub const L1_NBITS_OFFSET: u32 = 13;
    pub const L1_NBITS_MASK: u32 = 0x0000e000;

    /// L1_ROT - Right-rotate applied to the current shifter data before the lane 1 TMDS enc...
    /// Bits [12:8], mask 0x00001f00
    pub const L1_ROT_OFFSET: u32 = 8;
    pub const L1_ROT_MASK: u32 = 0x00001f00;

    /// L0_NBITS - Number of valid data bits for the lane 0 TMDS encoder, starting from bit 7 ...
    /// Bits [7:5], mask 0x000000e0
    pub const L0_NBITS_OFFSET: u32 = 5;
    pub const L0_NBITS_MASK: u32 = 0x000000e0;

    /// L0_ROT - Right-rotate applied to the current shifter data before the lane 0 TMDS enc...
    /// Bits [4:0], mask 0x0000001f
    pub const L0_ROT_OFFSET: u32 = 0;
    pub const L0_ROT_MASK: u32 = 0x0000001f;

    // ======== Basic Access Methods ========

    /// Read the raw register value as u32
    #[inline(always)]
    pub fn read_bits(&self) -> u32 {
        unsafe { core::ptr::read_volatile(self.value.get()) }
    }

    /// Write a raw u32 value to the register
    #[inline(always)]
    pub fn write_bits(&self, value: u32) {
        unsafe { core::ptr::write_volatile(self.value.get(), value) }
    }

    /// Modify the raw register value
    #[inline(always)]
    pub fn modify_bits<F: FnOnce(u32) -> u32>(&self, f: F) {
        self.write_bits(f(self.read_bits()));
    }

    /// Atomic SET - sets bits that are 1 (hardware alias at +0x2000)
    #[inline(always)]
    pub fn set_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x2000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic CLR - clears bits that are 1 (hardware alias at +0x3000)
    #[inline(always)]
    pub fn clr_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x3000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    /// Atomic XOR - toggles bits that are 1 (hardware alias at +0x1000)
    #[inline(always)]
    pub fn xor_bits(&self, bits: u32) {
        unsafe {
            let addr = (self.value.get() as usize + 0x1000) as *mut u32;
            core::ptr::write_volatile(addr, bits);
        }
    }

    // ======== Field Accessors ========

    /// **L2_NBITS** [23:21]: Number of valid data bits for the lane 2 TMDS encoder, starting from bit 7 ...
    #[inline(always)]
    pub fn l2_nbits(&self) -> u32 {
        (self.read_bits() & Self::L2_NBITS_MASK) >> Self::L2_NBITS_OFFSET
    }

    /// Write L2_NBITS field
    #[inline(always)]
    pub fn write_l2_nbits(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::L2_NBITS_MASK) | ((value << Self::L2_NBITS_OFFSET) & Self::L2_NBITS_MASK));
    }

    /// **L2_ROT** [20:16]: Right-rotate applied to the current shifter data before the lane 2 TMDS enc...
    #[inline(always)]
    pub fn l2_rot(&self) -> u32 {
        (self.read_bits() & Self::L2_ROT_MASK) >> Self::L2_ROT_OFFSET
    }

    /// Write L2_ROT field
    #[inline(always)]
    pub fn write_l2_rot(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::L2_ROT_MASK) | ((value << Self::L2_ROT_OFFSET) & Self::L2_ROT_MASK));
    }

    /// **L1_NBITS** [15:13]: Number of valid data bits for the lane 1 TMDS encoder, starting from bit 7 ...
    #[inline(always)]
    pub fn l1_nbits(&self) -> u32 {
        (self.read_bits() & Self::L1_NBITS_MASK) >> Self::L1_NBITS_OFFSET
    }

    /// Write L1_NBITS field
    #[inline(always)]
    pub fn write_l1_nbits(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::L1_NBITS_MASK) | ((value << Self::L1_NBITS_OFFSET) & Self::L1_NBITS_MASK));
    }

    /// **L1_ROT** [12:8]: Right-rotate applied to the current shifter data before the lane 1 TMDS enc...
    #[inline(always)]
    pub fn l1_rot(&self) -> u32 {
        (self.read_bits() & Self::L1_ROT_MASK) >> Self::L1_ROT_OFFSET
    }

    /// Write L1_ROT field
    #[inline(always)]
    pub fn write_l1_rot(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::L1_ROT_MASK) | ((value << Self::L1_ROT_OFFSET) & Self::L1_ROT_MASK));
    }

    /// **L0_NBITS** [7:5]: Number of valid data bits for the lane 0 TMDS encoder, starting from bit 7 ...
    #[inline(always)]
    pub fn l0_nbits(&self) -> u32 {
        (self.read_bits() & Self::L0_NBITS_MASK) >> Self::L0_NBITS_OFFSET
    }

    /// Write L0_NBITS field
    #[inline(always)]
    pub fn write_l0_nbits(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::L0_NBITS_MASK) | ((value << Self::L0_NBITS_OFFSET) & Self::L0_NBITS_MASK));
    }

    /// **L0_ROT** [4:0]: Right-rotate applied to the current shifter data before the lane 0 TMDS enc...
    #[inline(always)]
    pub fn l0_rot(&self) -> u32 {
        (self.read_bits() & Self::L0_ROT_MASK) >> Self::L0_ROT_OFFSET
    }

    /// Write L0_ROT field
    #[inline(always)]
    pub fn write_l0_rot(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::L0_ROT_MASK) | ((value << Self::L0_ROT_OFFSET) & Self::L0_ROT_MASK));
    }

}

/// EXPAND_TMDS value type for batch read/modify/write operations
///
/// Use `expandtmds.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `expandtmds.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ExpandTmdsVal {
    bits: u32,
}

impl ExpandTmdsVal {
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

    /// Read L2_NBITS [23:21]: Number of valid data bits for the lane 2 TMDS encoder, starting from bit 7 ...
    #[inline(always)]
    pub const fn l2_nbits(&self) -> u32 {
        (self.bits & ExpandTmds::L2_NBITS_MASK) >> ExpandTmds::L2_NBITS_OFFSET
    }

    /// Read L2_ROT [20:16]: Right-rotate applied to the current shifter data before the lane 2 TMDS enc...
    #[inline(always)]
    pub const fn l2_rot(&self) -> u32 {
        (self.bits & ExpandTmds::L2_ROT_MASK) >> ExpandTmds::L2_ROT_OFFSET
    }

    /// Read L1_NBITS [15:13]: Number of valid data bits for the lane 1 TMDS encoder, starting from bit 7 ...
    #[inline(always)]
    pub const fn l1_nbits(&self) -> u32 {
        (self.bits & ExpandTmds::L1_NBITS_MASK) >> ExpandTmds::L1_NBITS_OFFSET
    }

    /// Read L1_ROT [12:8]: Right-rotate applied to the current shifter data before the lane 1 TMDS enc...
    #[inline(always)]
    pub const fn l1_rot(&self) -> u32 {
        (self.bits & ExpandTmds::L1_ROT_MASK) >> ExpandTmds::L1_ROT_OFFSET
    }

    /// Read L0_NBITS [7:5]: Number of valid data bits for the lane 0 TMDS encoder, starting from bit 7 ...
    #[inline(always)]
    pub const fn l0_nbits(&self) -> u32 {
        (self.bits & ExpandTmds::L0_NBITS_MASK) >> ExpandTmds::L0_NBITS_OFFSET
    }

    /// Read L0_ROT [4:0]: Right-rotate applied to the current shifter data before the lane 0 TMDS enc...
    #[inline(always)]
    pub const fn l0_rot(&self) -> u32 {
        (self.bits & ExpandTmds::L0_ROT_MASK) >> ExpandTmds::L0_ROT_OFFSET
    }

    /// Set L2_NBITS [23:21]: Number of valid data bits for the lane 2 TMDS encoder, starting from bit 7 ...
    #[inline(always)]
    pub const fn with_l2_nbits(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ExpandTmds::L2_NBITS_MASK) | ((value << ExpandTmds::L2_NBITS_OFFSET) & ExpandTmds::L2_NBITS_MASK)
        }
    }

    /// Set L2_ROT [20:16]: Right-rotate applied to the current shifter data before the lane 2 TMDS enc...
    #[inline(always)]
    pub const fn with_l2_rot(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ExpandTmds::L2_ROT_MASK) | ((value << ExpandTmds::L2_ROT_OFFSET) & ExpandTmds::L2_ROT_MASK)
        }
    }

    /// Set L1_NBITS [15:13]: Number of valid data bits for the lane 1 TMDS encoder, starting from bit 7 ...
    #[inline(always)]
    pub const fn with_l1_nbits(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ExpandTmds::L1_NBITS_MASK) | ((value << ExpandTmds::L1_NBITS_OFFSET) & ExpandTmds::L1_NBITS_MASK)
        }
    }

    /// Set L1_ROT [12:8]: Right-rotate applied to the current shifter data before the lane 1 TMDS enc...
    #[inline(always)]
    pub const fn with_l1_rot(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ExpandTmds::L1_ROT_MASK) | ((value << ExpandTmds::L1_ROT_OFFSET) & ExpandTmds::L1_ROT_MASK)
        }
    }

    /// Set L0_NBITS [7:5]: Number of valid data bits for the lane 0 TMDS encoder, starting from bit 7 ...
    #[inline(always)]
    pub const fn with_l0_nbits(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ExpandTmds::L0_NBITS_MASK) | ((value << ExpandTmds::L0_NBITS_OFFSET) & ExpandTmds::L0_NBITS_MASK)
        }
    }

    /// Set L0_ROT [4:0]: Right-rotate applied to the current shifter data before the lane 0 TMDS enc...
    #[inline(always)]
    pub const fn with_l0_rot(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ExpandTmds::L0_ROT_MASK) | ((value << ExpandTmds::L0_ROT_OFFSET) & ExpandTmds::L0_ROT_MASK)
        }
    }

}

impl Default for ExpandTmdsVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl ExpandTmds {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> ExpandTmdsVal {
        ExpandTmdsVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: ExpandTmdsVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(ExpandTmdsVal) -> ExpandTmdsVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    pub csr: Csr,
    /// 0x0004 - Data control register for output bit 0
    pub bit0: Bit0,
    /// 0x0008 - Data control register for output bit 1
    pub bit1: Bit1,
    /// 0x000c - Data control register for output bit 2
    pub bit2: Bit2,
    /// 0x0010 - Data control register for output bit 3
    pub bit3: Bit3,
    /// 0x0014 - Data control register for output bit 4
    pub bit4: Bit4,
    /// 0x0018 - Data control register for output bit 5
    pub bit5: Bit5,
    /// 0x001c - Data control register for output bit 6
    pub bit6: Bit6,
    /// 0x0020 - Data control register for output bit 7
    pub bit7: Bit7,
    /// 0x0024 - Configure the optional shifter inside the command expander
    pub expand_shift: ExpandShift,
    /// 0x0028 - Configure the optional TMDS encoder inside the command expander
    pub expand_tmds: ExpandTmds,
}

