//! CLOCKS
//!
//! This peripheral supports atomic SET/CLR/XOR register aliases.

#![allow(dead_code)]

pub const BASE: u32 = 0x40010000;

/// CLK_GPOUT0_CTRL Register
///
/// Clock control, can be changed on-the-fly (except for auxsrc)
///
/// Address offset: 0x0000
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **ENABLED** `[28]` (RO)
///
///   clock generator is enabled
///
/// - **NUDGE** `[20]` (RW)
///
///   An edge on this signal shifts the phase of the output by 1 cycle of the input clock
///   This can be done at any time
///
/// - **PHASE** `[17:16]` (RW)
///
///   This delays the enable signal by up to 3 cycles of the input clock
///   This must be set before the clock is enabled to have any effect
///
/// - **DC50** `[12]` (RW)
///
///   Enables duty cycle correction for odd divisors, can be changed on-the-fly
///
/// - **ENABLE** `[11]` (RW)
///
///   Starts and stops the clock generator cleanly
///
/// - **KILL** `[10]` (RW)
///
///   Asynchronously kills the clock generator, enable must be set low before deasserting kill
///
/// - **AUXSRC** `[8:5]` (RW)
///
///   Selects the auxiliary clock source, will glitch when switching
#[repr(transparent)]
pub struct ClkGpout0Ctrl {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for ClkGpout0Ctrl {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ClkGpout0Ctrl {
    // ======== Field Constants ========

    /// **ENABLED**
    ///
    /// clock generator is enabled
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const ENABLED_OFFSET: u32 = 28;
    pub const ENABLED_MASK: u32 = 0x10000000;
    pub const ENABLED: u32 = 1 << 28;

    /// **NUDGE**
    ///
    /// An edge on this signal shifts the phase of the output by 1 cycle of the input clock
    /// This can be done at any time
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const NUDGE_OFFSET: u32 = 20;
    pub const NUDGE_MASK: u32 = 0x00100000;
    pub const NUDGE: u32 = 1 << 20;

    /// **PHASE**
    ///
    /// This delays the enable signal by up to 3 cycles of the input clock
    /// This must be set before the clock is enabled to have any effect
    ///
    /// Bits `[17:16]`, mask `0x00030000`
    pub const PHASE_OFFSET: u32 = 16;
    pub const PHASE_MASK: u32 = 0x00030000;

    /// **DC50**
    ///
    /// Enables duty cycle correction for odd divisors, can be changed on-the-fly
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const DC50_OFFSET: u32 = 12;
    pub const DC50_MASK: u32 = 0x00001000;
    pub const DC50: u32 = 1 << 12;

    /// **ENABLE**
    ///
    /// Starts and stops the clock generator cleanly
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const ENABLE_OFFSET: u32 = 11;
    pub const ENABLE_MASK: u32 = 0x00000800;
    pub const ENABLE: u32 = 1 << 11;

    /// **KILL**
    ///
    /// Asynchronously kills the clock generator, enable must be set low before deasserting kill
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const KILL_OFFSET: u32 = 10;
    pub const KILL_MASK: u32 = 0x00000400;
    pub const KILL: u32 = 1 << 10;

    /// **AUXSRC**
    ///
    /// Selects the auxiliary clock source, will glitch when switching
    ///
    /// Bits `[8:5]`, mask `0x000001e0`
    pub const AUXSRC_OFFSET: u32 = 5;
    pub const AUXSRC_MASK: u32 = 0x000001e0;
    pub const AUXSRC_CLKSRC_PLL_SYS: u32 = 0 << 5;
    pub const AUXSRC_CLKSRC_PLL_SYS_VAL: u32 = 0;
    pub const AUXSRC_CLKSRC_GPIN0: u32 = 1 << 5;
    pub const AUXSRC_CLKSRC_GPIN0_VAL: u32 = 1;
    pub const AUXSRC_CLKSRC_GPIN1: u32 = 2 << 5;
    pub const AUXSRC_CLKSRC_GPIN1_VAL: u32 = 2;
    pub const AUXSRC_CLKSRC_PLL_USB: u32 = 3 << 5;
    pub const AUXSRC_CLKSRC_PLL_USB_VAL: u32 = 3;
    pub const AUXSRC_CLKSRC_PLL_USB_PRIMARY_REF_OPCG: u32 = 4 << 5;
    pub const AUXSRC_CLKSRC_PLL_USB_PRIMARY_REF_OPCG_VAL: u32 = 4;
    pub const AUXSRC_ROSC_CLKSRC: u32 = 5 << 5;
    pub const AUXSRC_ROSC_CLKSRC_VAL: u32 = 5;
    pub const AUXSRC_XOSC_CLKSRC: u32 = 6 << 5;
    pub const AUXSRC_XOSC_CLKSRC_VAL: u32 = 6;
    pub const AUXSRC_LPOSC_CLKSRC: u32 = 7 << 5;
    pub const AUXSRC_LPOSC_CLKSRC_VAL: u32 = 7;
    pub const AUXSRC_CLK_SYS: u32 = 8 << 5;
    pub const AUXSRC_CLK_SYS_VAL: u32 = 8;
    pub const AUXSRC_CLK_USB: u32 = 9 << 5;
    pub const AUXSRC_CLK_USB_VAL: u32 = 9;
    pub const AUXSRC_CLK_ADC: u32 = 10 << 5;
    pub const AUXSRC_CLK_ADC_VAL: u32 = 10;
    pub const AUXSRC_CLK_REF: u32 = 11 << 5;
    pub const AUXSRC_CLK_REF_VAL: u32 = 11;
    pub const AUXSRC_CLK_PERI: u32 = 12 << 5;
    pub const AUXSRC_CLK_PERI_VAL: u32 = 12;
    pub const AUXSRC_CLK_HSTX: u32 = 13 << 5;
    pub const AUXSRC_CLK_HSTX_VAL: u32 = 13;
    pub const AUXSRC_OTP_CLK2FC: u32 = 14 << 5;
    pub const AUXSRC_OTP_CLK2FC_VAL: u32 = 14;

    // ======== Field Accessors ========

    /// **ENABLED** `[28]` (RO)
    ///
    /// clock generator is enabled
    #[inline(always)]
    pub fn enabled(&self) -> bool {
        (self.value.read_bits() & Self::ENABLED_MASK) != 0
    }

    /// **NUDGE** `[20]` (RW)
    ///
    /// An edge on this signal shifts the phase of the output by 1 cycle of the input clock
    /// This can be done at any time
    #[inline(always)]
    pub fn nudge(&self) -> bool {
        (self.value.read_bits() & Self::NUDGE_MASK) != 0
    }

    /// Set NUDGE bit atomically (RW)
    #[inline(always)]
    pub fn set_nudge(&self) {
        self.set_bits(Self::NUDGE_MASK);
    }

    /// Clear NUDGE bit atomically (RW)
    #[inline(always)]
    pub fn clr_nudge(&self) {
        self.clr_bits(Self::NUDGE_MASK);
    }

    /// **PHASE** `[17:16]` (RW)
    ///
    /// This delays the enable signal by up to 3 cycles of the input clock
    /// This must be set before the clock is enabled to have any effect
    #[inline(always)]
    pub fn phase(&self) -> u32 {
        (self.value.read_bits() & Self::PHASE_MASK) >> Self::PHASE_OFFSET
    }

    /// Write **PHASE** `[17:16]` (RW)
    ///
    /// This delays the enable signal by up to 3 cycles of the input clock
    /// This must be set before the clock is enabled to have any effect
    #[inline(always)]
    pub fn write_phase(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::PHASE_MASK) | ((value << Self::PHASE_OFFSET) & Self::PHASE_MASK));
    }

    /// **DC50** `[12]` (RW)
    ///
    /// Enables duty cycle correction for odd divisors, can be changed on-the-fly
    #[inline(always)]
    pub fn dc50(&self) -> bool {
        (self.value.read_bits() & Self::DC50_MASK) != 0
    }

    /// Set DC50 bit atomically (RW)
    #[inline(always)]
    pub fn set_dc50(&self) {
        self.set_bits(Self::DC50_MASK);
    }

    /// Clear DC50 bit atomically (RW)
    #[inline(always)]
    pub fn clr_dc50(&self) {
        self.clr_bits(Self::DC50_MASK);
    }

    /// **ENABLE** `[11]` (RW)
    ///
    /// Starts and stops the clock generator cleanly
    #[inline(always)]
    pub fn enable(&self) -> bool {
        (self.value.read_bits() & Self::ENABLE_MASK) != 0
    }

    /// Set ENABLE bit atomically (RW)
    #[inline(always)]
    pub fn set_enable(&self) {
        self.set_bits(Self::ENABLE_MASK);
    }

    /// Clear ENABLE bit atomically (RW)
    #[inline(always)]
    pub fn clr_enable(&self) {
        self.clr_bits(Self::ENABLE_MASK);
    }

    /// **KILL** `[10]` (RW)
    ///
    /// Asynchronously kills the clock generator, enable must be set low before deasserting kill
    #[inline(always)]
    pub fn kill(&self) -> bool {
        (self.value.read_bits() & Self::KILL_MASK) != 0
    }

    /// Set KILL bit atomically (RW)
    #[inline(always)]
    pub fn set_kill(&self) {
        self.set_bits(Self::KILL_MASK);
    }

    /// Clear KILL bit atomically (RW)
    #[inline(always)]
    pub fn clr_kill(&self) {
        self.clr_bits(Self::KILL_MASK);
    }

    /// **AUXSRC** `[8:5]` (RW)
    ///
    /// Selects the auxiliary clock source, will glitch when switching
    #[inline(always)]
    pub fn auxsrc(&self) -> u32 {
        (self.value.read_bits() & Self::AUXSRC_MASK) >> Self::AUXSRC_OFFSET
    }

    /// Write **AUXSRC** `[8:5]` (RW)
    ///
    /// Selects the auxiliary clock source, will glitch when switching
    #[inline(always)]
    pub fn write_auxsrc(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::AUXSRC_MASK) | ((value << Self::AUXSRC_OFFSET) & Self::AUXSRC_MASK));
    }
}

/// CLK_GPOUT0_CTRL value type for batch read/modify/write operations
///
/// Use `clkgpout0ctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `clkgpout0ctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ClkGpout0CtrlVal {
    bits: u32,
}

impl ClkGpout0CtrlVal {
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

    /// Read **ENABLED** `[28:28]`
    ///
    /// clock generator is enabled
    #[inline(always)]
    pub const fn enabled(&self) -> bool {
        (self.bits & ClkGpout0Ctrl::ENABLED_MASK) != 0
    }

    /// Read **NUDGE** `[20:20]`
    ///
    /// An edge on this signal shifts the phase of the output by 1 cycle of the input clock
    /// This can be done at any time
    #[inline(always)]
    pub const fn nudge(&self) -> bool {
        (self.bits & ClkGpout0Ctrl::NUDGE_MASK) != 0
    }

    /// Read **PHASE** `[17:16]`
    ///
    /// This delays the enable signal by up to 3 cycles of the input clock
    /// This must be set before the clock is enabled to have any effect
    #[inline(always)]
    pub const fn phase(&self) -> u32 {
        (self.bits & ClkGpout0Ctrl::PHASE_MASK) >> ClkGpout0Ctrl::PHASE_OFFSET
    }

    /// Read **DC50** `[12:12]`
    ///
    /// Enables duty cycle correction for odd divisors, can be changed on-the-fly
    #[inline(always)]
    pub const fn dc50(&self) -> bool {
        (self.bits & ClkGpout0Ctrl::DC50_MASK) != 0
    }

    /// Read **ENABLE** `[11:11]`
    ///
    /// Starts and stops the clock generator cleanly
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        (self.bits & ClkGpout0Ctrl::ENABLE_MASK) != 0
    }

    /// Read **KILL** `[10:10]`
    ///
    /// Asynchronously kills the clock generator, enable must be set low before deasserting kill
    #[inline(always)]
    pub const fn kill(&self) -> bool {
        (self.bits & ClkGpout0Ctrl::KILL_MASK) != 0
    }

    /// Read **AUXSRC** `[8:5]`
    ///
    /// Selects the auxiliary clock source, will glitch when switching
    #[inline(always)]
    pub const fn auxsrc(&self) -> u32 {
        (self.bits & ClkGpout0Ctrl::AUXSRC_MASK) >> ClkGpout0Ctrl::AUXSRC_OFFSET
    }

    /// Set **NUDGE** `[20:20]`
    ///
    /// An edge on this signal shifts the phase of the output by 1 cycle of the input clock
    /// This can be done at any time
    #[inline(always)]
    pub const fn with_nudge(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClkGpout0Ctrl::NUDGE_MASK
            } else {
                self.bits & !ClkGpout0Ctrl::NUDGE_MASK
            }
        }
    }

    /// Set **PHASE** `[17:16]`
    ///
    /// This delays the enable signal by up to 3 cycles of the input clock
    /// This must be set before the clock is enabled to have any effect
    #[inline(always)]
    pub const fn with_phase(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ClkGpout0Ctrl::PHASE_MASK) | ((value << ClkGpout0Ctrl::PHASE_OFFSET) & ClkGpout0Ctrl::PHASE_MASK)
        }
    }

    /// Set **DC50** `[12:12]`
    ///
    /// Enables duty cycle correction for odd divisors, can be changed on-the-fly
    #[inline(always)]
    pub const fn with_dc50(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClkGpout0Ctrl::DC50_MASK
            } else {
                self.bits & !ClkGpout0Ctrl::DC50_MASK
            }
        }
    }

    /// Set **ENABLE** `[11:11]`
    ///
    /// Starts and stops the clock generator cleanly
    #[inline(always)]
    pub const fn with_enable(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClkGpout0Ctrl::ENABLE_MASK
            } else {
                self.bits & !ClkGpout0Ctrl::ENABLE_MASK
            }
        }
    }

    /// Set **KILL** `[10:10]`
    ///
    /// Asynchronously kills the clock generator, enable must be set low before deasserting kill
    #[inline(always)]
    pub const fn with_kill(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClkGpout0Ctrl::KILL_MASK
            } else {
                self.bits & !ClkGpout0Ctrl::KILL_MASK
            }
        }
    }

    /// Set **AUXSRC** `[8:5]`
    ///
    /// Selects the auxiliary clock source, will glitch when switching
    #[inline(always)]
    pub const fn with_auxsrc(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ClkGpout0Ctrl::AUXSRC_MASK) | ((value << ClkGpout0Ctrl::AUXSRC_OFFSET) & ClkGpout0Ctrl::AUXSRC_MASK)
        }
    }
}

impl Default for ClkGpout0CtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl ClkGpout0Ctrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ClkGpout0CtrlVal {
        ClkGpout0CtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ClkGpout0CtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ClkGpout0CtrlVal) -> ClkGpout0CtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CLK_GPOUT0_DIV Register
///
/// Address offset: 0x0004
/// Reset value: 0x00010000
///
/// ## Fields
///
/// - **INT** `[31:16]` (RW)
///
///   Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly
///
/// - **FRAC** `[15:0]` (RW)
///
///   Fractional component of the divisor, can be changed on-the-fly
#[repr(transparent)]
pub struct ClkGpout0Div {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for ClkGpout0Div {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ClkGpout0Div {
    // ======== Field Constants ========

    /// **INT**
    ///
    /// Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const INT_OFFSET: u32 = 16;
    pub const INT_MASK: u32 = 0xffff0000;

    /// **FRAC**
    ///
    /// Fractional component of the divisor, can be changed on-the-fly
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const FRAC_OFFSET: u32 = 0;
    pub const FRAC_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **INT** `[31:16]` (RW)
    ///
    /// Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly
    #[inline(always)]
    pub fn int(&self) -> u32 {
        (self.value.read_bits() & Self::INT_MASK) >> Self::INT_OFFSET
    }

    /// Write **INT** `[31:16]` (RW)
    ///
    /// Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly
    #[inline(always)]
    pub fn write_int(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::INT_MASK) | ((value << Self::INT_OFFSET) & Self::INT_MASK));
    }

    /// **FRAC** `[15:0]` (RW)
    ///
    /// Fractional component of the divisor, can be changed on-the-fly
    #[inline(always)]
    pub fn frac(&self) -> u32 {
        (self.value.read_bits() & Self::FRAC_MASK) >> Self::FRAC_OFFSET
    }

    /// Write **FRAC** `[15:0]` (RW)
    ///
    /// Fractional component of the divisor, can be changed on-the-fly
    #[inline(always)]
    pub fn write_frac(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::FRAC_MASK) | ((value << Self::FRAC_OFFSET) & Self::FRAC_MASK));
    }
}

/// CLK_GPOUT0_DIV value type for batch read/modify/write operations
///
/// Use `clkgpout0div.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `clkgpout0div.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ClkGpout0DivVal {
    bits: u32,
}

impl ClkGpout0DivVal {
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
    /// Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly
    #[inline(always)]
    pub const fn int(&self) -> u32 {
        (self.bits & ClkGpout0Div::INT_MASK) >> ClkGpout0Div::INT_OFFSET
    }

    /// Read **FRAC** `[15:0]`
    ///
    /// Fractional component of the divisor, can be changed on-the-fly
    #[inline(always)]
    pub const fn frac(&self) -> u32 {
        (self.bits & ClkGpout0Div::FRAC_MASK) >> ClkGpout0Div::FRAC_OFFSET
    }

    /// Set **INT** `[31:16]`
    ///
    /// Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly
    #[inline(always)]
    pub const fn with_int(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ClkGpout0Div::INT_MASK) | ((value << ClkGpout0Div::INT_OFFSET) & ClkGpout0Div::INT_MASK)
        }
    }

    /// Set **FRAC** `[15:0]`
    ///
    /// Fractional component of the divisor, can be changed on-the-fly
    #[inline(always)]
    pub const fn with_frac(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ClkGpout0Div::FRAC_MASK) | ((value << ClkGpout0Div::FRAC_OFFSET) & ClkGpout0Div::FRAC_MASK)
        }
    }
}

impl Default for ClkGpout0DivVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl ClkGpout0Div {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ClkGpout0DivVal {
        ClkGpout0DivVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ClkGpout0DivVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ClkGpout0DivVal) -> ClkGpout0DivVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CLK_GPOUT0_SELECTED Register
///
/// Indicates which src is currently selected (one-hot)
///
/// Address offset: 0x0008
/// Reset value: 0x00000001
///
/// ## Fields
///
/// - **CLK_GPOUT0_SELECTED** `[0]` (RO)
///
///   This slice does not have a glitchless mux (only the AUX_SRC field is present, not SRC) so this register is hardwired to 0x1.
#[repr(transparent)]
pub struct ClkGpout0Selected {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for ClkGpout0Selected {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ClkGpout0Selected {
    // ======== Field Constants ========

    /// **CLK_GPOUT0_SELECTED**
    ///
    /// This slice does not have a glitchless mux (only the AUX_SRC field is present, not SRC) so this register is hardwired to 0x1.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CLK_GPOUT0_SELECTED_OFFSET: u32 = 0;
    pub const CLK_GPOUT0_SELECTED_MASK: u32 = 0x00000001;
    pub const CLK_GPOUT0_SELECTED: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CLK_GPOUT0_SELECTED** `[0]` (RO)
    ///
    /// This slice does not have a glitchless mux (only the AUX_SRC field is present, not SRC) so this register is hardwired to 0x1.
    #[inline(always)]
    pub fn clk_gpout0_selected(&self) -> bool {
        (self.value.read_bits() & Self::CLK_GPOUT0_SELECTED_MASK) != 0
    }
}

/// CLK_GPOUT0_SELECTED value type for batch read/modify/write operations
///
/// Use `clkgpout0selected.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `clkgpout0selected.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ClkGpout0SelectedVal {
    bits: u32,
}

impl ClkGpout0SelectedVal {
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
        Self { bits: 0x00000001 }
    }

    /// Read **CLK_GPOUT0_SELECTED** `[0:0]`
    ///
    /// This slice does not have a glitchless mux (only the AUX_SRC field is present, not SRC) so this register is hardwired to 0x1.
    #[inline(always)]
    pub const fn clk_gpout0_selected(&self) -> bool {
        (self.bits & ClkGpout0Selected::CLK_GPOUT0_SELECTED_MASK) != 0
    }
}

impl Default for ClkGpout0SelectedVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl ClkGpout0Selected {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ClkGpout0SelectedVal {
        ClkGpout0SelectedVal::from_bits(self.value.read_bits())
    }
}

/// CLK_GPOUT1_CTRL Register
///
/// Clock control, can be changed on-the-fly (except for auxsrc)
///
/// Address offset: 0x000c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **ENABLED** `[28]` (RO)
///
///   clock generator is enabled
///
/// - **NUDGE** `[20]` (RW)
///
///   An edge on this signal shifts the phase of the output by 1 cycle of the input clock
///   This can be done at any time
///
/// - **PHASE** `[17:16]` (RW)
///
///   This delays the enable signal by up to 3 cycles of the input clock
///   This must be set before the clock is enabled to have any effect
///
/// - **DC50** `[12]` (RW)
///
///   Enables duty cycle correction for odd divisors, can be changed on-the-fly
///
/// - **ENABLE** `[11]` (RW)
///
///   Starts and stops the clock generator cleanly
///
/// - **KILL** `[10]` (RW)
///
///   Asynchronously kills the clock generator, enable must be set low before deasserting kill
///
/// - **AUXSRC** `[8:5]` (RW)
///
///   Selects the auxiliary clock source, will glitch when switching
#[repr(transparent)]
pub struct ClkGpout1Ctrl {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for ClkGpout1Ctrl {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ClkGpout1Ctrl {
    // ======== Field Constants ========

    /// **ENABLED**
    ///
    /// clock generator is enabled
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const ENABLED_OFFSET: u32 = 28;
    pub const ENABLED_MASK: u32 = 0x10000000;
    pub const ENABLED: u32 = 1 << 28;

    /// **NUDGE**
    ///
    /// An edge on this signal shifts the phase of the output by 1 cycle of the input clock
    /// This can be done at any time
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const NUDGE_OFFSET: u32 = 20;
    pub const NUDGE_MASK: u32 = 0x00100000;
    pub const NUDGE: u32 = 1 << 20;

    /// **PHASE**
    ///
    /// This delays the enable signal by up to 3 cycles of the input clock
    /// This must be set before the clock is enabled to have any effect
    ///
    /// Bits `[17:16]`, mask `0x00030000`
    pub const PHASE_OFFSET: u32 = 16;
    pub const PHASE_MASK: u32 = 0x00030000;

    /// **DC50**
    ///
    /// Enables duty cycle correction for odd divisors, can be changed on-the-fly
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const DC50_OFFSET: u32 = 12;
    pub const DC50_MASK: u32 = 0x00001000;
    pub const DC50: u32 = 1 << 12;

    /// **ENABLE**
    ///
    /// Starts and stops the clock generator cleanly
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const ENABLE_OFFSET: u32 = 11;
    pub const ENABLE_MASK: u32 = 0x00000800;
    pub const ENABLE: u32 = 1 << 11;

    /// **KILL**
    ///
    /// Asynchronously kills the clock generator, enable must be set low before deasserting kill
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const KILL_OFFSET: u32 = 10;
    pub const KILL_MASK: u32 = 0x00000400;
    pub const KILL: u32 = 1 << 10;

    /// **AUXSRC**
    ///
    /// Selects the auxiliary clock source, will glitch when switching
    ///
    /// Bits `[8:5]`, mask `0x000001e0`
    pub const AUXSRC_OFFSET: u32 = 5;
    pub const AUXSRC_MASK: u32 = 0x000001e0;
    pub const AUXSRC_CLKSRC_PLL_SYS: u32 = 0 << 5;
    pub const AUXSRC_CLKSRC_PLL_SYS_VAL: u32 = 0;
    pub const AUXSRC_CLKSRC_GPIN0: u32 = 1 << 5;
    pub const AUXSRC_CLKSRC_GPIN0_VAL: u32 = 1;
    pub const AUXSRC_CLKSRC_GPIN1: u32 = 2 << 5;
    pub const AUXSRC_CLKSRC_GPIN1_VAL: u32 = 2;
    pub const AUXSRC_CLKSRC_PLL_USB: u32 = 3 << 5;
    pub const AUXSRC_CLKSRC_PLL_USB_VAL: u32 = 3;
    pub const AUXSRC_CLKSRC_PLL_USB_PRIMARY_REF_OPCG: u32 = 4 << 5;
    pub const AUXSRC_CLKSRC_PLL_USB_PRIMARY_REF_OPCG_VAL: u32 = 4;
    pub const AUXSRC_ROSC_CLKSRC: u32 = 5 << 5;
    pub const AUXSRC_ROSC_CLKSRC_VAL: u32 = 5;
    pub const AUXSRC_XOSC_CLKSRC: u32 = 6 << 5;
    pub const AUXSRC_XOSC_CLKSRC_VAL: u32 = 6;
    pub const AUXSRC_LPOSC_CLKSRC: u32 = 7 << 5;
    pub const AUXSRC_LPOSC_CLKSRC_VAL: u32 = 7;
    pub const AUXSRC_CLK_SYS: u32 = 8 << 5;
    pub const AUXSRC_CLK_SYS_VAL: u32 = 8;
    pub const AUXSRC_CLK_USB: u32 = 9 << 5;
    pub const AUXSRC_CLK_USB_VAL: u32 = 9;
    pub const AUXSRC_CLK_ADC: u32 = 10 << 5;
    pub const AUXSRC_CLK_ADC_VAL: u32 = 10;
    pub const AUXSRC_CLK_REF: u32 = 11 << 5;
    pub const AUXSRC_CLK_REF_VAL: u32 = 11;
    pub const AUXSRC_CLK_PERI: u32 = 12 << 5;
    pub const AUXSRC_CLK_PERI_VAL: u32 = 12;
    pub const AUXSRC_CLK_HSTX: u32 = 13 << 5;
    pub const AUXSRC_CLK_HSTX_VAL: u32 = 13;
    pub const AUXSRC_OTP_CLK2FC: u32 = 14 << 5;
    pub const AUXSRC_OTP_CLK2FC_VAL: u32 = 14;

    // ======== Field Accessors ========

    /// **ENABLED** `[28]` (RO)
    ///
    /// clock generator is enabled
    #[inline(always)]
    pub fn enabled(&self) -> bool {
        (self.value.read_bits() & Self::ENABLED_MASK) != 0
    }

    /// **NUDGE** `[20]` (RW)
    ///
    /// An edge on this signal shifts the phase of the output by 1 cycle of the input clock
    /// This can be done at any time
    #[inline(always)]
    pub fn nudge(&self) -> bool {
        (self.value.read_bits() & Self::NUDGE_MASK) != 0
    }

    /// Set NUDGE bit atomically (RW)
    #[inline(always)]
    pub fn set_nudge(&self) {
        self.set_bits(Self::NUDGE_MASK);
    }

    /// Clear NUDGE bit atomically (RW)
    #[inline(always)]
    pub fn clr_nudge(&self) {
        self.clr_bits(Self::NUDGE_MASK);
    }

    /// **PHASE** `[17:16]` (RW)
    ///
    /// This delays the enable signal by up to 3 cycles of the input clock
    /// This must be set before the clock is enabled to have any effect
    #[inline(always)]
    pub fn phase(&self) -> u32 {
        (self.value.read_bits() & Self::PHASE_MASK) >> Self::PHASE_OFFSET
    }

    /// Write **PHASE** `[17:16]` (RW)
    ///
    /// This delays the enable signal by up to 3 cycles of the input clock
    /// This must be set before the clock is enabled to have any effect
    #[inline(always)]
    pub fn write_phase(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::PHASE_MASK) | ((value << Self::PHASE_OFFSET) & Self::PHASE_MASK));
    }

    /// **DC50** `[12]` (RW)
    ///
    /// Enables duty cycle correction for odd divisors, can be changed on-the-fly
    #[inline(always)]
    pub fn dc50(&self) -> bool {
        (self.value.read_bits() & Self::DC50_MASK) != 0
    }

    /// Set DC50 bit atomically (RW)
    #[inline(always)]
    pub fn set_dc50(&self) {
        self.set_bits(Self::DC50_MASK);
    }

    /// Clear DC50 bit atomically (RW)
    #[inline(always)]
    pub fn clr_dc50(&self) {
        self.clr_bits(Self::DC50_MASK);
    }

    /// **ENABLE** `[11]` (RW)
    ///
    /// Starts and stops the clock generator cleanly
    #[inline(always)]
    pub fn enable(&self) -> bool {
        (self.value.read_bits() & Self::ENABLE_MASK) != 0
    }

    /// Set ENABLE bit atomically (RW)
    #[inline(always)]
    pub fn set_enable(&self) {
        self.set_bits(Self::ENABLE_MASK);
    }

    /// Clear ENABLE bit atomically (RW)
    #[inline(always)]
    pub fn clr_enable(&self) {
        self.clr_bits(Self::ENABLE_MASK);
    }

    /// **KILL** `[10]` (RW)
    ///
    /// Asynchronously kills the clock generator, enable must be set low before deasserting kill
    #[inline(always)]
    pub fn kill(&self) -> bool {
        (self.value.read_bits() & Self::KILL_MASK) != 0
    }

    /// Set KILL bit atomically (RW)
    #[inline(always)]
    pub fn set_kill(&self) {
        self.set_bits(Self::KILL_MASK);
    }

    /// Clear KILL bit atomically (RW)
    #[inline(always)]
    pub fn clr_kill(&self) {
        self.clr_bits(Self::KILL_MASK);
    }

    /// **AUXSRC** `[8:5]` (RW)
    ///
    /// Selects the auxiliary clock source, will glitch when switching
    #[inline(always)]
    pub fn auxsrc(&self) -> u32 {
        (self.value.read_bits() & Self::AUXSRC_MASK) >> Self::AUXSRC_OFFSET
    }

    /// Write **AUXSRC** `[8:5]` (RW)
    ///
    /// Selects the auxiliary clock source, will glitch when switching
    #[inline(always)]
    pub fn write_auxsrc(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::AUXSRC_MASK) | ((value << Self::AUXSRC_OFFSET) & Self::AUXSRC_MASK));
    }
}

/// CLK_GPOUT1_CTRL value type for batch read/modify/write operations
///
/// Use `clkgpout1ctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `clkgpout1ctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ClkGpout1CtrlVal {
    bits: u32,
}

impl ClkGpout1CtrlVal {
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

    /// Read **ENABLED** `[28:28]`
    ///
    /// clock generator is enabled
    #[inline(always)]
    pub const fn enabled(&self) -> bool {
        (self.bits & ClkGpout1Ctrl::ENABLED_MASK) != 0
    }

    /// Read **NUDGE** `[20:20]`
    ///
    /// An edge on this signal shifts the phase of the output by 1 cycle of the input clock
    /// This can be done at any time
    #[inline(always)]
    pub const fn nudge(&self) -> bool {
        (self.bits & ClkGpout1Ctrl::NUDGE_MASK) != 0
    }

    /// Read **PHASE** `[17:16]`
    ///
    /// This delays the enable signal by up to 3 cycles of the input clock
    /// This must be set before the clock is enabled to have any effect
    #[inline(always)]
    pub const fn phase(&self) -> u32 {
        (self.bits & ClkGpout1Ctrl::PHASE_MASK) >> ClkGpout1Ctrl::PHASE_OFFSET
    }

    /// Read **DC50** `[12:12]`
    ///
    /// Enables duty cycle correction for odd divisors, can be changed on-the-fly
    #[inline(always)]
    pub const fn dc50(&self) -> bool {
        (self.bits & ClkGpout1Ctrl::DC50_MASK) != 0
    }

    /// Read **ENABLE** `[11:11]`
    ///
    /// Starts and stops the clock generator cleanly
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        (self.bits & ClkGpout1Ctrl::ENABLE_MASK) != 0
    }

    /// Read **KILL** `[10:10]`
    ///
    /// Asynchronously kills the clock generator, enable must be set low before deasserting kill
    #[inline(always)]
    pub const fn kill(&self) -> bool {
        (self.bits & ClkGpout1Ctrl::KILL_MASK) != 0
    }

    /// Read **AUXSRC** `[8:5]`
    ///
    /// Selects the auxiliary clock source, will glitch when switching
    #[inline(always)]
    pub const fn auxsrc(&self) -> u32 {
        (self.bits & ClkGpout1Ctrl::AUXSRC_MASK) >> ClkGpout1Ctrl::AUXSRC_OFFSET
    }

    /// Set **NUDGE** `[20:20]`
    ///
    /// An edge on this signal shifts the phase of the output by 1 cycle of the input clock
    /// This can be done at any time
    #[inline(always)]
    pub const fn with_nudge(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClkGpout1Ctrl::NUDGE_MASK
            } else {
                self.bits & !ClkGpout1Ctrl::NUDGE_MASK
            }
        }
    }

    /// Set **PHASE** `[17:16]`
    ///
    /// This delays the enable signal by up to 3 cycles of the input clock
    /// This must be set before the clock is enabled to have any effect
    #[inline(always)]
    pub const fn with_phase(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ClkGpout1Ctrl::PHASE_MASK) | ((value << ClkGpout1Ctrl::PHASE_OFFSET) & ClkGpout1Ctrl::PHASE_MASK)
        }
    }

    /// Set **DC50** `[12:12]`
    ///
    /// Enables duty cycle correction for odd divisors, can be changed on-the-fly
    #[inline(always)]
    pub const fn with_dc50(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClkGpout1Ctrl::DC50_MASK
            } else {
                self.bits & !ClkGpout1Ctrl::DC50_MASK
            }
        }
    }

    /// Set **ENABLE** `[11:11]`
    ///
    /// Starts and stops the clock generator cleanly
    #[inline(always)]
    pub const fn with_enable(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClkGpout1Ctrl::ENABLE_MASK
            } else {
                self.bits & !ClkGpout1Ctrl::ENABLE_MASK
            }
        }
    }

    /// Set **KILL** `[10:10]`
    ///
    /// Asynchronously kills the clock generator, enable must be set low before deasserting kill
    #[inline(always)]
    pub const fn with_kill(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClkGpout1Ctrl::KILL_MASK
            } else {
                self.bits & !ClkGpout1Ctrl::KILL_MASK
            }
        }
    }

    /// Set **AUXSRC** `[8:5]`
    ///
    /// Selects the auxiliary clock source, will glitch when switching
    #[inline(always)]
    pub const fn with_auxsrc(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ClkGpout1Ctrl::AUXSRC_MASK) | ((value << ClkGpout1Ctrl::AUXSRC_OFFSET) & ClkGpout1Ctrl::AUXSRC_MASK)
        }
    }
}

impl Default for ClkGpout1CtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl ClkGpout1Ctrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ClkGpout1CtrlVal {
        ClkGpout1CtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ClkGpout1CtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ClkGpout1CtrlVal) -> ClkGpout1CtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CLK_GPOUT1_DIV Register
///
/// Address offset: 0x0010
/// Reset value: 0x00010000
///
/// ## Fields
///
/// - **INT** `[31:16]` (RW)
///
///   Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly
///
/// - **FRAC** `[15:0]` (RW)
///
///   Fractional component of the divisor, can be changed on-the-fly
#[repr(transparent)]
pub struct ClkGpout1Div {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for ClkGpout1Div {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ClkGpout1Div {
    // ======== Field Constants ========

    /// **INT**
    ///
    /// Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const INT_OFFSET: u32 = 16;
    pub const INT_MASK: u32 = 0xffff0000;

    /// **FRAC**
    ///
    /// Fractional component of the divisor, can be changed on-the-fly
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const FRAC_OFFSET: u32 = 0;
    pub const FRAC_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **INT** `[31:16]` (RW)
    ///
    /// Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly
    #[inline(always)]
    pub fn int(&self) -> u32 {
        (self.value.read_bits() & Self::INT_MASK) >> Self::INT_OFFSET
    }

    /// Write **INT** `[31:16]` (RW)
    ///
    /// Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly
    #[inline(always)]
    pub fn write_int(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::INT_MASK) | ((value << Self::INT_OFFSET) & Self::INT_MASK));
    }

    /// **FRAC** `[15:0]` (RW)
    ///
    /// Fractional component of the divisor, can be changed on-the-fly
    #[inline(always)]
    pub fn frac(&self) -> u32 {
        (self.value.read_bits() & Self::FRAC_MASK) >> Self::FRAC_OFFSET
    }

    /// Write **FRAC** `[15:0]` (RW)
    ///
    /// Fractional component of the divisor, can be changed on-the-fly
    #[inline(always)]
    pub fn write_frac(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::FRAC_MASK) | ((value << Self::FRAC_OFFSET) & Self::FRAC_MASK));
    }
}

/// CLK_GPOUT1_DIV value type for batch read/modify/write operations
///
/// Use `clkgpout1div.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `clkgpout1div.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ClkGpout1DivVal {
    bits: u32,
}

impl ClkGpout1DivVal {
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
    /// Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly
    #[inline(always)]
    pub const fn int(&self) -> u32 {
        (self.bits & ClkGpout1Div::INT_MASK) >> ClkGpout1Div::INT_OFFSET
    }

    /// Read **FRAC** `[15:0]`
    ///
    /// Fractional component of the divisor, can be changed on-the-fly
    #[inline(always)]
    pub const fn frac(&self) -> u32 {
        (self.bits & ClkGpout1Div::FRAC_MASK) >> ClkGpout1Div::FRAC_OFFSET
    }

    /// Set **INT** `[31:16]`
    ///
    /// Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly
    #[inline(always)]
    pub const fn with_int(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ClkGpout1Div::INT_MASK) | ((value << ClkGpout1Div::INT_OFFSET) & ClkGpout1Div::INT_MASK)
        }
    }

    /// Set **FRAC** `[15:0]`
    ///
    /// Fractional component of the divisor, can be changed on-the-fly
    #[inline(always)]
    pub const fn with_frac(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ClkGpout1Div::FRAC_MASK) | ((value << ClkGpout1Div::FRAC_OFFSET) & ClkGpout1Div::FRAC_MASK)
        }
    }
}

impl Default for ClkGpout1DivVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl ClkGpout1Div {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ClkGpout1DivVal {
        ClkGpout1DivVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ClkGpout1DivVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ClkGpout1DivVal) -> ClkGpout1DivVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CLK_GPOUT1_SELECTED Register
///
/// Indicates which src is currently selected (one-hot)
///
/// Address offset: 0x0014
/// Reset value: 0x00000001
///
/// ## Fields
///
/// - **CLK_GPOUT1_SELECTED** `[0]` (RO)
///
///   This slice does not have a glitchless mux (only the AUX_SRC field is present, not SRC) so this register is hardwired to 0x1.
#[repr(transparent)]
pub struct ClkGpout1Selected {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for ClkGpout1Selected {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ClkGpout1Selected {
    // ======== Field Constants ========

    /// **CLK_GPOUT1_SELECTED**
    ///
    /// This slice does not have a glitchless mux (only the AUX_SRC field is present, not SRC) so this register is hardwired to 0x1.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CLK_GPOUT1_SELECTED_OFFSET: u32 = 0;
    pub const CLK_GPOUT1_SELECTED_MASK: u32 = 0x00000001;
    pub const CLK_GPOUT1_SELECTED: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CLK_GPOUT1_SELECTED** `[0]` (RO)
    ///
    /// This slice does not have a glitchless mux (only the AUX_SRC field is present, not SRC) so this register is hardwired to 0x1.
    #[inline(always)]
    pub fn clk_gpout1_selected(&self) -> bool {
        (self.value.read_bits() & Self::CLK_GPOUT1_SELECTED_MASK) != 0
    }
}

/// CLK_GPOUT1_SELECTED value type for batch read/modify/write operations
///
/// Use `clkgpout1selected.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `clkgpout1selected.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ClkGpout1SelectedVal {
    bits: u32,
}

impl ClkGpout1SelectedVal {
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
        Self { bits: 0x00000001 }
    }

    /// Read **CLK_GPOUT1_SELECTED** `[0:0]`
    ///
    /// This slice does not have a glitchless mux (only the AUX_SRC field is present, not SRC) so this register is hardwired to 0x1.
    #[inline(always)]
    pub const fn clk_gpout1_selected(&self) -> bool {
        (self.bits & ClkGpout1Selected::CLK_GPOUT1_SELECTED_MASK) != 0
    }
}

impl Default for ClkGpout1SelectedVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl ClkGpout1Selected {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ClkGpout1SelectedVal {
        ClkGpout1SelectedVal::from_bits(self.value.read_bits())
    }
}

/// CLK_GPOUT2_CTRL Register
///
/// Clock control, can be changed on-the-fly (except for auxsrc)
///
/// Address offset: 0x0018
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **ENABLED** `[28]` (RO)
///
///   clock generator is enabled
///
/// - **NUDGE** `[20]` (RW)
///
///   An edge on this signal shifts the phase of the output by 1 cycle of the input clock
///   This can be done at any time
///
/// - **PHASE** `[17:16]` (RW)
///
///   This delays the enable signal by up to 3 cycles of the input clock
///   This must be set before the clock is enabled to have any effect
///
/// - **DC50** `[12]` (RW)
///
///   Enables duty cycle correction for odd divisors, can be changed on-the-fly
///
/// - **ENABLE** `[11]` (RW)
///
///   Starts and stops the clock generator cleanly
///
/// - **KILL** `[10]` (RW)
///
///   Asynchronously kills the clock generator, enable must be set low before deasserting kill
///
/// - **AUXSRC** `[8:5]` (RW)
///
///   Selects the auxiliary clock source, will glitch when switching
#[repr(transparent)]
pub struct ClkGpout2Ctrl {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for ClkGpout2Ctrl {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ClkGpout2Ctrl {
    // ======== Field Constants ========

    /// **ENABLED**
    ///
    /// clock generator is enabled
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const ENABLED_OFFSET: u32 = 28;
    pub const ENABLED_MASK: u32 = 0x10000000;
    pub const ENABLED: u32 = 1 << 28;

    /// **NUDGE**
    ///
    /// An edge on this signal shifts the phase of the output by 1 cycle of the input clock
    /// This can be done at any time
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const NUDGE_OFFSET: u32 = 20;
    pub const NUDGE_MASK: u32 = 0x00100000;
    pub const NUDGE: u32 = 1 << 20;

    /// **PHASE**
    ///
    /// This delays the enable signal by up to 3 cycles of the input clock
    /// This must be set before the clock is enabled to have any effect
    ///
    /// Bits `[17:16]`, mask `0x00030000`
    pub const PHASE_OFFSET: u32 = 16;
    pub const PHASE_MASK: u32 = 0x00030000;

    /// **DC50**
    ///
    /// Enables duty cycle correction for odd divisors, can be changed on-the-fly
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const DC50_OFFSET: u32 = 12;
    pub const DC50_MASK: u32 = 0x00001000;
    pub const DC50: u32 = 1 << 12;

    /// **ENABLE**
    ///
    /// Starts and stops the clock generator cleanly
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const ENABLE_OFFSET: u32 = 11;
    pub const ENABLE_MASK: u32 = 0x00000800;
    pub const ENABLE: u32 = 1 << 11;

    /// **KILL**
    ///
    /// Asynchronously kills the clock generator, enable must be set low before deasserting kill
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const KILL_OFFSET: u32 = 10;
    pub const KILL_MASK: u32 = 0x00000400;
    pub const KILL: u32 = 1 << 10;

    /// **AUXSRC**
    ///
    /// Selects the auxiliary clock source, will glitch when switching
    ///
    /// Bits `[8:5]`, mask `0x000001e0`
    pub const AUXSRC_OFFSET: u32 = 5;
    pub const AUXSRC_MASK: u32 = 0x000001e0;
    pub const AUXSRC_CLKSRC_PLL_SYS: u32 = 0 << 5;
    pub const AUXSRC_CLKSRC_PLL_SYS_VAL: u32 = 0;
    pub const AUXSRC_CLKSRC_GPIN0: u32 = 1 << 5;
    pub const AUXSRC_CLKSRC_GPIN0_VAL: u32 = 1;
    pub const AUXSRC_CLKSRC_GPIN1: u32 = 2 << 5;
    pub const AUXSRC_CLKSRC_GPIN1_VAL: u32 = 2;
    pub const AUXSRC_CLKSRC_PLL_USB: u32 = 3 << 5;
    pub const AUXSRC_CLKSRC_PLL_USB_VAL: u32 = 3;
    pub const AUXSRC_CLKSRC_PLL_USB_PRIMARY_REF_OPCG: u32 = 4 << 5;
    pub const AUXSRC_CLKSRC_PLL_USB_PRIMARY_REF_OPCG_VAL: u32 = 4;
    pub const AUXSRC_ROSC_CLKSRC_PH: u32 = 5 << 5;
    pub const AUXSRC_ROSC_CLKSRC_PH_VAL: u32 = 5;
    pub const AUXSRC_XOSC_CLKSRC: u32 = 6 << 5;
    pub const AUXSRC_XOSC_CLKSRC_VAL: u32 = 6;
    pub const AUXSRC_LPOSC_CLKSRC: u32 = 7 << 5;
    pub const AUXSRC_LPOSC_CLKSRC_VAL: u32 = 7;
    pub const AUXSRC_CLK_SYS: u32 = 8 << 5;
    pub const AUXSRC_CLK_SYS_VAL: u32 = 8;
    pub const AUXSRC_CLK_USB: u32 = 9 << 5;
    pub const AUXSRC_CLK_USB_VAL: u32 = 9;
    pub const AUXSRC_CLK_ADC: u32 = 10 << 5;
    pub const AUXSRC_CLK_ADC_VAL: u32 = 10;
    pub const AUXSRC_CLK_REF: u32 = 11 << 5;
    pub const AUXSRC_CLK_REF_VAL: u32 = 11;
    pub const AUXSRC_CLK_PERI: u32 = 12 << 5;
    pub const AUXSRC_CLK_PERI_VAL: u32 = 12;
    pub const AUXSRC_CLK_HSTX: u32 = 13 << 5;
    pub const AUXSRC_CLK_HSTX_VAL: u32 = 13;
    pub const AUXSRC_OTP_CLK2FC: u32 = 14 << 5;
    pub const AUXSRC_OTP_CLK2FC_VAL: u32 = 14;

    // ======== Field Accessors ========

    /// **ENABLED** `[28]` (RO)
    ///
    /// clock generator is enabled
    #[inline(always)]
    pub fn enabled(&self) -> bool {
        (self.value.read_bits() & Self::ENABLED_MASK) != 0
    }

    /// **NUDGE** `[20]` (RW)
    ///
    /// An edge on this signal shifts the phase of the output by 1 cycle of the input clock
    /// This can be done at any time
    #[inline(always)]
    pub fn nudge(&self) -> bool {
        (self.value.read_bits() & Self::NUDGE_MASK) != 0
    }

    /// Set NUDGE bit atomically (RW)
    #[inline(always)]
    pub fn set_nudge(&self) {
        self.set_bits(Self::NUDGE_MASK);
    }

    /// Clear NUDGE bit atomically (RW)
    #[inline(always)]
    pub fn clr_nudge(&self) {
        self.clr_bits(Self::NUDGE_MASK);
    }

    /// **PHASE** `[17:16]` (RW)
    ///
    /// This delays the enable signal by up to 3 cycles of the input clock
    /// This must be set before the clock is enabled to have any effect
    #[inline(always)]
    pub fn phase(&self) -> u32 {
        (self.value.read_bits() & Self::PHASE_MASK) >> Self::PHASE_OFFSET
    }

    /// Write **PHASE** `[17:16]` (RW)
    ///
    /// This delays the enable signal by up to 3 cycles of the input clock
    /// This must be set before the clock is enabled to have any effect
    #[inline(always)]
    pub fn write_phase(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::PHASE_MASK) | ((value << Self::PHASE_OFFSET) & Self::PHASE_MASK));
    }

    /// **DC50** `[12]` (RW)
    ///
    /// Enables duty cycle correction for odd divisors, can be changed on-the-fly
    #[inline(always)]
    pub fn dc50(&self) -> bool {
        (self.value.read_bits() & Self::DC50_MASK) != 0
    }

    /// Set DC50 bit atomically (RW)
    #[inline(always)]
    pub fn set_dc50(&self) {
        self.set_bits(Self::DC50_MASK);
    }

    /// Clear DC50 bit atomically (RW)
    #[inline(always)]
    pub fn clr_dc50(&self) {
        self.clr_bits(Self::DC50_MASK);
    }

    /// **ENABLE** `[11]` (RW)
    ///
    /// Starts and stops the clock generator cleanly
    #[inline(always)]
    pub fn enable(&self) -> bool {
        (self.value.read_bits() & Self::ENABLE_MASK) != 0
    }

    /// Set ENABLE bit atomically (RW)
    #[inline(always)]
    pub fn set_enable(&self) {
        self.set_bits(Self::ENABLE_MASK);
    }

    /// Clear ENABLE bit atomically (RW)
    #[inline(always)]
    pub fn clr_enable(&self) {
        self.clr_bits(Self::ENABLE_MASK);
    }

    /// **KILL** `[10]` (RW)
    ///
    /// Asynchronously kills the clock generator, enable must be set low before deasserting kill
    #[inline(always)]
    pub fn kill(&self) -> bool {
        (self.value.read_bits() & Self::KILL_MASK) != 0
    }

    /// Set KILL bit atomically (RW)
    #[inline(always)]
    pub fn set_kill(&self) {
        self.set_bits(Self::KILL_MASK);
    }

    /// Clear KILL bit atomically (RW)
    #[inline(always)]
    pub fn clr_kill(&self) {
        self.clr_bits(Self::KILL_MASK);
    }

    /// **AUXSRC** `[8:5]` (RW)
    ///
    /// Selects the auxiliary clock source, will glitch when switching
    #[inline(always)]
    pub fn auxsrc(&self) -> u32 {
        (self.value.read_bits() & Self::AUXSRC_MASK) >> Self::AUXSRC_OFFSET
    }

    /// Write **AUXSRC** `[8:5]` (RW)
    ///
    /// Selects the auxiliary clock source, will glitch when switching
    #[inline(always)]
    pub fn write_auxsrc(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::AUXSRC_MASK) | ((value << Self::AUXSRC_OFFSET) & Self::AUXSRC_MASK));
    }
}

/// CLK_GPOUT2_CTRL value type for batch read/modify/write operations
///
/// Use `clkgpout2ctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `clkgpout2ctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ClkGpout2CtrlVal {
    bits: u32,
}

impl ClkGpout2CtrlVal {
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

    /// Read **ENABLED** `[28:28]`
    ///
    /// clock generator is enabled
    #[inline(always)]
    pub const fn enabled(&self) -> bool {
        (self.bits & ClkGpout2Ctrl::ENABLED_MASK) != 0
    }

    /// Read **NUDGE** `[20:20]`
    ///
    /// An edge on this signal shifts the phase of the output by 1 cycle of the input clock
    /// This can be done at any time
    #[inline(always)]
    pub const fn nudge(&self) -> bool {
        (self.bits & ClkGpout2Ctrl::NUDGE_MASK) != 0
    }

    /// Read **PHASE** `[17:16]`
    ///
    /// This delays the enable signal by up to 3 cycles of the input clock
    /// This must be set before the clock is enabled to have any effect
    #[inline(always)]
    pub const fn phase(&self) -> u32 {
        (self.bits & ClkGpout2Ctrl::PHASE_MASK) >> ClkGpout2Ctrl::PHASE_OFFSET
    }

    /// Read **DC50** `[12:12]`
    ///
    /// Enables duty cycle correction for odd divisors, can be changed on-the-fly
    #[inline(always)]
    pub const fn dc50(&self) -> bool {
        (self.bits & ClkGpout2Ctrl::DC50_MASK) != 0
    }

    /// Read **ENABLE** `[11:11]`
    ///
    /// Starts and stops the clock generator cleanly
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        (self.bits & ClkGpout2Ctrl::ENABLE_MASK) != 0
    }

    /// Read **KILL** `[10:10]`
    ///
    /// Asynchronously kills the clock generator, enable must be set low before deasserting kill
    #[inline(always)]
    pub const fn kill(&self) -> bool {
        (self.bits & ClkGpout2Ctrl::KILL_MASK) != 0
    }

    /// Read **AUXSRC** `[8:5]`
    ///
    /// Selects the auxiliary clock source, will glitch when switching
    #[inline(always)]
    pub const fn auxsrc(&self) -> u32 {
        (self.bits & ClkGpout2Ctrl::AUXSRC_MASK) >> ClkGpout2Ctrl::AUXSRC_OFFSET
    }

    /// Set **NUDGE** `[20:20]`
    ///
    /// An edge on this signal shifts the phase of the output by 1 cycle of the input clock
    /// This can be done at any time
    #[inline(always)]
    pub const fn with_nudge(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClkGpout2Ctrl::NUDGE_MASK
            } else {
                self.bits & !ClkGpout2Ctrl::NUDGE_MASK
            }
        }
    }

    /// Set **PHASE** `[17:16]`
    ///
    /// This delays the enable signal by up to 3 cycles of the input clock
    /// This must be set before the clock is enabled to have any effect
    #[inline(always)]
    pub const fn with_phase(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ClkGpout2Ctrl::PHASE_MASK) | ((value << ClkGpout2Ctrl::PHASE_OFFSET) & ClkGpout2Ctrl::PHASE_MASK)
        }
    }

    /// Set **DC50** `[12:12]`
    ///
    /// Enables duty cycle correction for odd divisors, can be changed on-the-fly
    #[inline(always)]
    pub const fn with_dc50(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClkGpout2Ctrl::DC50_MASK
            } else {
                self.bits & !ClkGpout2Ctrl::DC50_MASK
            }
        }
    }

    /// Set **ENABLE** `[11:11]`
    ///
    /// Starts and stops the clock generator cleanly
    #[inline(always)]
    pub const fn with_enable(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClkGpout2Ctrl::ENABLE_MASK
            } else {
                self.bits & !ClkGpout2Ctrl::ENABLE_MASK
            }
        }
    }

    /// Set **KILL** `[10:10]`
    ///
    /// Asynchronously kills the clock generator, enable must be set low before deasserting kill
    #[inline(always)]
    pub const fn with_kill(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClkGpout2Ctrl::KILL_MASK
            } else {
                self.bits & !ClkGpout2Ctrl::KILL_MASK
            }
        }
    }

    /// Set **AUXSRC** `[8:5]`
    ///
    /// Selects the auxiliary clock source, will glitch when switching
    #[inline(always)]
    pub const fn with_auxsrc(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ClkGpout2Ctrl::AUXSRC_MASK) | ((value << ClkGpout2Ctrl::AUXSRC_OFFSET) & ClkGpout2Ctrl::AUXSRC_MASK)
        }
    }
}

impl Default for ClkGpout2CtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl ClkGpout2Ctrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ClkGpout2CtrlVal {
        ClkGpout2CtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ClkGpout2CtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ClkGpout2CtrlVal) -> ClkGpout2CtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CLK_GPOUT2_DIV Register
///
/// Address offset: 0x001c
/// Reset value: 0x00010000
///
/// ## Fields
///
/// - **INT** `[31:16]` (RW)
///
///   Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly
///
/// - **FRAC** `[15:0]` (RW)
///
///   Fractional component of the divisor, can be changed on-the-fly
#[repr(transparent)]
pub struct ClkGpout2Div {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for ClkGpout2Div {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ClkGpout2Div {
    // ======== Field Constants ========

    /// **INT**
    ///
    /// Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const INT_OFFSET: u32 = 16;
    pub const INT_MASK: u32 = 0xffff0000;

    /// **FRAC**
    ///
    /// Fractional component of the divisor, can be changed on-the-fly
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const FRAC_OFFSET: u32 = 0;
    pub const FRAC_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **INT** `[31:16]` (RW)
    ///
    /// Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly
    #[inline(always)]
    pub fn int(&self) -> u32 {
        (self.value.read_bits() & Self::INT_MASK) >> Self::INT_OFFSET
    }

    /// Write **INT** `[31:16]` (RW)
    ///
    /// Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly
    #[inline(always)]
    pub fn write_int(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::INT_MASK) | ((value << Self::INT_OFFSET) & Self::INT_MASK));
    }

    /// **FRAC** `[15:0]` (RW)
    ///
    /// Fractional component of the divisor, can be changed on-the-fly
    #[inline(always)]
    pub fn frac(&self) -> u32 {
        (self.value.read_bits() & Self::FRAC_MASK) >> Self::FRAC_OFFSET
    }

    /// Write **FRAC** `[15:0]` (RW)
    ///
    /// Fractional component of the divisor, can be changed on-the-fly
    #[inline(always)]
    pub fn write_frac(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::FRAC_MASK) | ((value << Self::FRAC_OFFSET) & Self::FRAC_MASK));
    }
}

/// CLK_GPOUT2_DIV value type for batch read/modify/write operations
///
/// Use `clkgpout2div.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `clkgpout2div.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ClkGpout2DivVal {
    bits: u32,
}

impl ClkGpout2DivVal {
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
    /// Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly
    #[inline(always)]
    pub const fn int(&self) -> u32 {
        (self.bits & ClkGpout2Div::INT_MASK) >> ClkGpout2Div::INT_OFFSET
    }

    /// Read **FRAC** `[15:0]`
    ///
    /// Fractional component of the divisor, can be changed on-the-fly
    #[inline(always)]
    pub const fn frac(&self) -> u32 {
        (self.bits & ClkGpout2Div::FRAC_MASK) >> ClkGpout2Div::FRAC_OFFSET
    }

    /// Set **INT** `[31:16]`
    ///
    /// Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly
    #[inline(always)]
    pub const fn with_int(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ClkGpout2Div::INT_MASK) | ((value << ClkGpout2Div::INT_OFFSET) & ClkGpout2Div::INT_MASK)
        }
    }

    /// Set **FRAC** `[15:0]`
    ///
    /// Fractional component of the divisor, can be changed on-the-fly
    #[inline(always)]
    pub const fn with_frac(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ClkGpout2Div::FRAC_MASK) | ((value << ClkGpout2Div::FRAC_OFFSET) & ClkGpout2Div::FRAC_MASK)
        }
    }
}

impl Default for ClkGpout2DivVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl ClkGpout2Div {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ClkGpout2DivVal {
        ClkGpout2DivVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ClkGpout2DivVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ClkGpout2DivVal) -> ClkGpout2DivVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CLK_GPOUT2_SELECTED Register
///
/// Indicates which src is currently selected (one-hot)
///
/// Address offset: 0x0020
/// Reset value: 0x00000001
///
/// ## Fields
///
/// - **CLK_GPOUT2_SELECTED** `[0]` (RO)
///
///   This slice does not have a glitchless mux (only the AUX_SRC field is present, not SRC) so this register is hardwired to 0x1.
#[repr(transparent)]
pub struct ClkGpout2Selected {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for ClkGpout2Selected {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ClkGpout2Selected {
    // ======== Field Constants ========

    /// **CLK_GPOUT2_SELECTED**
    ///
    /// This slice does not have a glitchless mux (only the AUX_SRC field is present, not SRC) so this register is hardwired to 0x1.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CLK_GPOUT2_SELECTED_OFFSET: u32 = 0;
    pub const CLK_GPOUT2_SELECTED_MASK: u32 = 0x00000001;
    pub const CLK_GPOUT2_SELECTED: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CLK_GPOUT2_SELECTED** `[0]` (RO)
    ///
    /// This slice does not have a glitchless mux (only the AUX_SRC field is present, not SRC) so this register is hardwired to 0x1.
    #[inline(always)]
    pub fn clk_gpout2_selected(&self) -> bool {
        (self.value.read_bits() & Self::CLK_GPOUT2_SELECTED_MASK) != 0
    }
}

/// CLK_GPOUT2_SELECTED value type for batch read/modify/write operations
///
/// Use `clkgpout2selected.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `clkgpout2selected.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ClkGpout2SelectedVal {
    bits: u32,
}

impl ClkGpout2SelectedVal {
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
        Self { bits: 0x00000001 }
    }

    /// Read **CLK_GPOUT2_SELECTED** `[0:0]`
    ///
    /// This slice does not have a glitchless mux (only the AUX_SRC field is present, not SRC) so this register is hardwired to 0x1.
    #[inline(always)]
    pub const fn clk_gpout2_selected(&self) -> bool {
        (self.bits & ClkGpout2Selected::CLK_GPOUT2_SELECTED_MASK) != 0
    }
}

impl Default for ClkGpout2SelectedVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl ClkGpout2Selected {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ClkGpout2SelectedVal {
        ClkGpout2SelectedVal::from_bits(self.value.read_bits())
    }
}

/// CLK_GPOUT3_CTRL Register
///
/// Clock control, can be changed on-the-fly (except for auxsrc)
///
/// Address offset: 0x0024
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **ENABLED** `[28]` (RO)
///
///   clock generator is enabled
///
/// - **NUDGE** `[20]` (RW)
///
///   An edge on this signal shifts the phase of the output by 1 cycle of the input clock
///   This can be done at any time
///
/// - **PHASE** `[17:16]` (RW)
///
///   This delays the enable signal by up to 3 cycles of the input clock
///   This must be set before the clock is enabled to have any effect
///
/// - **DC50** `[12]` (RW)
///
///   Enables duty cycle correction for odd divisors, can be changed on-the-fly
///
/// - **ENABLE** `[11]` (RW)
///
///   Starts and stops the clock generator cleanly
///
/// - **KILL** `[10]` (RW)
///
///   Asynchronously kills the clock generator, enable must be set low before deasserting kill
///
/// - **AUXSRC** `[8:5]` (RW)
///
///   Selects the auxiliary clock source, will glitch when switching
#[repr(transparent)]
pub struct ClkGpout3Ctrl {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for ClkGpout3Ctrl {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ClkGpout3Ctrl {
    // ======== Field Constants ========

    /// **ENABLED**
    ///
    /// clock generator is enabled
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const ENABLED_OFFSET: u32 = 28;
    pub const ENABLED_MASK: u32 = 0x10000000;
    pub const ENABLED: u32 = 1 << 28;

    /// **NUDGE**
    ///
    /// An edge on this signal shifts the phase of the output by 1 cycle of the input clock
    /// This can be done at any time
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const NUDGE_OFFSET: u32 = 20;
    pub const NUDGE_MASK: u32 = 0x00100000;
    pub const NUDGE: u32 = 1 << 20;

    /// **PHASE**
    ///
    /// This delays the enable signal by up to 3 cycles of the input clock
    /// This must be set before the clock is enabled to have any effect
    ///
    /// Bits `[17:16]`, mask `0x00030000`
    pub const PHASE_OFFSET: u32 = 16;
    pub const PHASE_MASK: u32 = 0x00030000;

    /// **DC50**
    ///
    /// Enables duty cycle correction for odd divisors, can be changed on-the-fly
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const DC50_OFFSET: u32 = 12;
    pub const DC50_MASK: u32 = 0x00001000;
    pub const DC50: u32 = 1 << 12;

    /// **ENABLE**
    ///
    /// Starts and stops the clock generator cleanly
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const ENABLE_OFFSET: u32 = 11;
    pub const ENABLE_MASK: u32 = 0x00000800;
    pub const ENABLE: u32 = 1 << 11;

    /// **KILL**
    ///
    /// Asynchronously kills the clock generator, enable must be set low before deasserting kill
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const KILL_OFFSET: u32 = 10;
    pub const KILL_MASK: u32 = 0x00000400;
    pub const KILL: u32 = 1 << 10;

    /// **AUXSRC**
    ///
    /// Selects the auxiliary clock source, will glitch when switching
    ///
    /// Bits `[8:5]`, mask `0x000001e0`
    pub const AUXSRC_OFFSET: u32 = 5;
    pub const AUXSRC_MASK: u32 = 0x000001e0;
    pub const AUXSRC_CLKSRC_PLL_SYS: u32 = 0 << 5;
    pub const AUXSRC_CLKSRC_PLL_SYS_VAL: u32 = 0;
    pub const AUXSRC_CLKSRC_GPIN0: u32 = 1 << 5;
    pub const AUXSRC_CLKSRC_GPIN0_VAL: u32 = 1;
    pub const AUXSRC_CLKSRC_GPIN1: u32 = 2 << 5;
    pub const AUXSRC_CLKSRC_GPIN1_VAL: u32 = 2;
    pub const AUXSRC_CLKSRC_PLL_USB: u32 = 3 << 5;
    pub const AUXSRC_CLKSRC_PLL_USB_VAL: u32 = 3;
    pub const AUXSRC_CLKSRC_PLL_USB_PRIMARY_REF_OPCG: u32 = 4 << 5;
    pub const AUXSRC_CLKSRC_PLL_USB_PRIMARY_REF_OPCG_VAL: u32 = 4;
    pub const AUXSRC_ROSC_CLKSRC_PH: u32 = 5 << 5;
    pub const AUXSRC_ROSC_CLKSRC_PH_VAL: u32 = 5;
    pub const AUXSRC_XOSC_CLKSRC: u32 = 6 << 5;
    pub const AUXSRC_XOSC_CLKSRC_VAL: u32 = 6;
    pub const AUXSRC_LPOSC_CLKSRC: u32 = 7 << 5;
    pub const AUXSRC_LPOSC_CLKSRC_VAL: u32 = 7;
    pub const AUXSRC_CLK_SYS: u32 = 8 << 5;
    pub const AUXSRC_CLK_SYS_VAL: u32 = 8;
    pub const AUXSRC_CLK_USB: u32 = 9 << 5;
    pub const AUXSRC_CLK_USB_VAL: u32 = 9;
    pub const AUXSRC_CLK_ADC: u32 = 10 << 5;
    pub const AUXSRC_CLK_ADC_VAL: u32 = 10;
    pub const AUXSRC_CLK_REF: u32 = 11 << 5;
    pub const AUXSRC_CLK_REF_VAL: u32 = 11;
    pub const AUXSRC_CLK_PERI: u32 = 12 << 5;
    pub const AUXSRC_CLK_PERI_VAL: u32 = 12;
    pub const AUXSRC_CLK_HSTX: u32 = 13 << 5;
    pub const AUXSRC_CLK_HSTX_VAL: u32 = 13;
    pub const AUXSRC_OTP_CLK2FC: u32 = 14 << 5;
    pub const AUXSRC_OTP_CLK2FC_VAL: u32 = 14;

    // ======== Field Accessors ========

    /// **ENABLED** `[28]` (RO)
    ///
    /// clock generator is enabled
    #[inline(always)]
    pub fn enabled(&self) -> bool {
        (self.value.read_bits() & Self::ENABLED_MASK) != 0
    }

    /// **NUDGE** `[20]` (RW)
    ///
    /// An edge on this signal shifts the phase of the output by 1 cycle of the input clock
    /// This can be done at any time
    #[inline(always)]
    pub fn nudge(&self) -> bool {
        (self.value.read_bits() & Self::NUDGE_MASK) != 0
    }

    /// Set NUDGE bit atomically (RW)
    #[inline(always)]
    pub fn set_nudge(&self) {
        self.set_bits(Self::NUDGE_MASK);
    }

    /// Clear NUDGE bit atomically (RW)
    #[inline(always)]
    pub fn clr_nudge(&self) {
        self.clr_bits(Self::NUDGE_MASK);
    }

    /// **PHASE** `[17:16]` (RW)
    ///
    /// This delays the enable signal by up to 3 cycles of the input clock
    /// This must be set before the clock is enabled to have any effect
    #[inline(always)]
    pub fn phase(&self) -> u32 {
        (self.value.read_bits() & Self::PHASE_MASK) >> Self::PHASE_OFFSET
    }

    /// Write **PHASE** `[17:16]` (RW)
    ///
    /// This delays the enable signal by up to 3 cycles of the input clock
    /// This must be set before the clock is enabled to have any effect
    #[inline(always)]
    pub fn write_phase(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::PHASE_MASK) | ((value << Self::PHASE_OFFSET) & Self::PHASE_MASK));
    }

    /// **DC50** `[12]` (RW)
    ///
    /// Enables duty cycle correction for odd divisors, can be changed on-the-fly
    #[inline(always)]
    pub fn dc50(&self) -> bool {
        (self.value.read_bits() & Self::DC50_MASK) != 0
    }

    /// Set DC50 bit atomically (RW)
    #[inline(always)]
    pub fn set_dc50(&self) {
        self.set_bits(Self::DC50_MASK);
    }

    /// Clear DC50 bit atomically (RW)
    #[inline(always)]
    pub fn clr_dc50(&self) {
        self.clr_bits(Self::DC50_MASK);
    }

    /// **ENABLE** `[11]` (RW)
    ///
    /// Starts and stops the clock generator cleanly
    #[inline(always)]
    pub fn enable(&self) -> bool {
        (self.value.read_bits() & Self::ENABLE_MASK) != 0
    }

    /// Set ENABLE bit atomically (RW)
    #[inline(always)]
    pub fn set_enable(&self) {
        self.set_bits(Self::ENABLE_MASK);
    }

    /// Clear ENABLE bit atomically (RW)
    #[inline(always)]
    pub fn clr_enable(&self) {
        self.clr_bits(Self::ENABLE_MASK);
    }

    /// **KILL** `[10]` (RW)
    ///
    /// Asynchronously kills the clock generator, enable must be set low before deasserting kill
    #[inline(always)]
    pub fn kill(&self) -> bool {
        (self.value.read_bits() & Self::KILL_MASK) != 0
    }

    /// Set KILL bit atomically (RW)
    #[inline(always)]
    pub fn set_kill(&self) {
        self.set_bits(Self::KILL_MASK);
    }

    /// Clear KILL bit atomically (RW)
    #[inline(always)]
    pub fn clr_kill(&self) {
        self.clr_bits(Self::KILL_MASK);
    }

    /// **AUXSRC** `[8:5]` (RW)
    ///
    /// Selects the auxiliary clock source, will glitch when switching
    #[inline(always)]
    pub fn auxsrc(&self) -> u32 {
        (self.value.read_bits() & Self::AUXSRC_MASK) >> Self::AUXSRC_OFFSET
    }

    /// Write **AUXSRC** `[8:5]` (RW)
    ///
    /// Selects the auxiliary clock source, will glitch when switching
    #[inline(always)]
    pub fn write_auxsrc(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::AUXSRC_MASK) | ((value << Self::AUXSRC_OFFSET) & Self::AUXSRC_MASK));
    }
}

/// CLK_GPOUT3_CTRL value type for batch read/modify/write operations
///
/// Use `clkgpout3ctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `clkgpout3ctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ClkGpout3CtrlVal {
    bits: u32,
}

impl ClkGpout3CtrlVal {
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

    /// Read **ENABLED** `[28:28]`
    ///
    /// clock generator is enabled
    #[inline(always)]
    pub const fn enabled(&self) -> bool {
        (self.bits & ClkGpout3Ctrl::ENABLED_MASK) != 0
    }

    /// Read **NUDGE** `[20:20]`
    ///
    /// An edge on this signal shifts the phase of the output by 1 cycle of the input clock
    /// This can be done at any time
    #[inline(always)]
    pub const fn nudge(&self) -> bool {
        (self.bits & ClkGpout3Ctrl::NUDGE_MASK) != 0
    }

    /// Read **PHASE** `[17:16]`
    ///
    /// This delays the enable signal by up to 3 cycles of the input clock
    /// This must be set before the clock is enabled to have any effect
    #[inline(always)]
    pub const fn phase(&self) -> u32 {
        (self.bits & ClkGpout3Ctrl::PHASE_MASK) >> ClkGpout3Ctrl::PHASE_OFFSET
    }

    /// Read **DC50** `[12:12]`
    ///
    /// Enables duty cycle correction for odd divisors, can be changed on-the-fly
    #[inline(always)]
    pub const fn dc50(&self) -> bool {
        (self.bits & ClkGpout3Ctrl::DC50_MASK) != 0
    }

    /// Read **ENABLE** `[11:11]`
    ///
    /// Starts and stops the clock generator cleanly
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        (self.bits & ClkGpout3Ctrl::ENABLE_MASK) != 0
    }

    /// Read **KILL** `[10:10]`
    ///
    /// Asynchronously kills the clock generator, enable must be set low before deasserting kill
    #[inline(always)]
    pub const fn kill(&self) -> bool {
        (self.bits & ClkGpout3Ctrl::KILL_MASK) != 0
    }

    /// Read **AUXSRC** `[8:5]`
    ///
    /// Selects the auxiliary clock source, will glitch when switching
    #[inline(always)]
    pub const fn auxsrc(&self) -> u32 {
        (self.bits & ClkGpout3Ctrl::AUXSRC_MASK) >> ClkGpout3Ctrl::AUXSRC_OFFSET
    }

    /// Set **NUDGE** `[20:20]`
    ///
    /// An edge on this signal shifts the phase of the output by 1 cycle of the input clock
    /// This can be done at any time
    #[inline(always)]
    pub const fn with_nudge(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClkGpout3Ctrl::NUDGE_MASK
            } else {
                self.bits & !ClkGpout3Ctrl::NUDGE_MASK
            }
        }
    }

    /// Set **PHASE** `[17:16]`
    ///
    /// This delays the enable signal by up to 3 cycles of the input clock
    /// This must be set before the clock is enabled to have any effect
    #[inline(always)]
    pub const fn with_phase(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ClkGpout3Ctrl::PHASE_MASK) | ((value << ClkGpout3Ctrl::PHASE_OFFSET) & ClkGpout3Ctrl::PHASE_MASK)
        }
    }

    /// Set **DC50** `[12:12]`
    ///
    /// Enables duty cycle correction for odd divisors, can be changed on-the-fly
    #[inline(always)]
    pub const fn with_dc50(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClkGpout3Ctrl::DC50_MASK
            } else {
                self.bits & !ClkGpout3Ctrl::DC50_MASK
            }
        }
    }

    /// Set **ENABLE** `[11:11]`
    ///
    /// Starts and stops the clock generator cleanly
    #[inline(always)]
    pub const fn with_enable(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClkGpout3Ctrl::ENABLE_MASK
            } else {
                self.bits & !ClkGpout3Ctrl::ENABLE_MASK
            }
        }
    }

    /// Set **KILL** `[10:10]`
    ///
    /// Asynchronously kills the clock generator, enable must be set low before deasserting kill
    #[inline(always)]
    pub const fn with_kill(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClkGpout3Ctrl::KILL_MASK
            } else {
                self.bits & !ClkGpout3Ctrl::KILL_MASK
            }
        }
    }

    /// Set **AUXSRC** `[8:5]`
    ///
    /// Selects the auxiliary clock source, will glitch when switching
    #[inline(always)]
    pub const fn with_auxsrc(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ClkGpout3Ctrl::AUXSRC_MASK) | ((value << ClkGpout3Ctrl::AUXSRC_OFFSET) & ClkGpout3Ctrl::AUXSRC_MASK)
        }
    }
}

impl Default for ClkGpout3CtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl ClkGpout3Ctrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ClkGpout3CtrlVal {
        ClkGpout3CtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ClkGpout3CtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ClkGpout3CtrlVal) -> ClkGpout3CtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CLK_GPOUT3_DIV Register
///
/// Address offset: 0x0028
/// Reset value: 0x00010000
///
/// ## Fields
///
/// - **INT** `[31:16]` (RW)
///
///   Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly
///
/// - **FRAC** `[15:0]` (RW)
///
///   Fractional component of the divisor, can be changed on-the-fly
#[repr(transparent)]
pub struct ClkGpout3Div {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for ClkGpout3Div {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ClkGpout3Div {
    // ======== Field Constants ========

    /// **INT**
    ///
    /// Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const INT_OFFSET: u32 = 16;
    pub const INT_MASK: u32 = 0xffff0000;

    /// **FRAC**
    ///
    /// Fractional component of the divisor, can be changed on-the-fly
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const FRAC_OFFSET: u32 = 0;
    pub const FRAC_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **INT** `[31:16]` (RW)
    ///
    /// Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly
    #[inline(always)]
    pub fn int(&self) -> u32 {
        (self.value.read_bits() & Self::INT_MASK) >> Self::INT_OFFSET
    }

    /// Write **INT** `[31:16]` (RW)
    ///
    /// Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly
    #[inline(always)]
    pub fn write_int(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::INT_MASK) | ((value << Self::INT_OFFSET) & Self::INT_MASK));
    }

    /// **FRAC** `[15:0]` (RW)
    ///
    /// Fractional component of the divisor, can be changed on-the-fly
    #[inline(always)]
    pub fn frac(&self) -> u32 {
        (self.value.read_bits() & Self::FRAC_MASK) >> Self::FRAC_OFFSET
    }

    /// Write **FRAC** `[15:0]` (RW)
    ///
    /// Fractional component of the divisor, can be changed on-the-fly
    #[inline(always)]
    pub fn write_frac(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::FRAC_MASK) | ((value << Self::FRAC_OFFSET) & Self::FRAC_MASK));
    }
}

/// CLK_GPOUT3_DIV value type for batch read/modify/write operations
///
/// Use `clkgpout3div.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `clkgpout3div.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ClkGpout3DivVal {
    bits: u32,
}

impl ClkGpout3DivVal {
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
    /// Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly
    #[inline(always)]
    pub const fn int(&self) -> u32 {
        (self.bits & ClkGpout3Div::INT_MASK) >> ClkGpout3Div::INT_OFFSET
    }

    /// Read **FRAC** `[15:0]`
    ///
    /// Fractional component of the divisor, can be changed on-the-fly
    #[inline(always)]
    pub const fn frac(&self) -> u32 {
        (self.bits & ClkGpout3Div::FRAC_MASK) >> ClkGpout3Div::FRAC_OFFSET
    }

    /// Set **INT** `[31:16]`
    ///
    /// Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly
    #[inline(always)]
    pub const fn with_int(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ClkGpout3Div::INT_MASK) | ((value << ClkGpout3Div::INT_OFFSET) & ClkGpout3Div::INT_MASK)
        }
    }

    /// Set **FRAC** `[15:0]`
    ///
    /// Fractional component of the divisor, can be changed on-the-fly
    #[inline(always)]
    pub const fn with_frac(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ClkGpout3Div::FRAC_MASK) | ((value << ClkGpout3Div::FRAC_OFFSET) & ClkGpout3Div::FRAC_MASK)
        }
    }
}

impl Default for ClkGpout3DivVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl ClkGpout3Div {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ClkGpout3DivVal {
        ClkGpout3DivVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ClkGpout3DivVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ClkGpout3DivVal) -> ClkGpout3DivVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CLK_GPOUT3_SELECTED Register
///
/// Indicates which src is currently selected (one-hot)
///
/// Address offset: 0x002c
/// Reset value: 0x00000001
///
/// ## Fields
///
/// - **CLK_GPOUT3_SELECTED** `[0]` (RO)
///
///   This slice does not have a glitchless mux (only the AUX_SRC field is present, not SRC) so this register is hardwired to 0x1.
#[repr(transparent)]
pub struct ClkGpout3Selected {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for ClkGpout3Selected {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ClkGpout3Selected {
    // ======== Field Constants ========

    /// **CLK_GPOUT3_SELECTED**
    ///
    /// This slice does not have a glitchless mux (only the AUX_SRC field is present, not SRC) so this register is hardwired to 0x1.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CLK_GPOUT3_SELECTED_OFFSET: u32 = 0;
    pub const CLK_GPOUT3_SELECTED_MASK: u32 = 0x00000001;
    pub const CLK_GPOUT3_SELECTED: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CLK_GPOUT3_SELECTED** `[0]` (RO)
    ///
    /// This slice does not have a glitchless mux (only the AUX_SRC field is present, not SRC) so this register is hardwired to 0x1.
    #[inline(always)]
    pub fn clk_gpout3_selected(&self) -> bool {
        (self.value.read_bits() & Self::CLK_GPOUT3_SELECTED_MASK) != 0
    }
}

/// CLK_GPOUT3_SELECTED value type for batch read/modify/write operations
///
/// Use `clkgpout3selected.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `clkgpout3selected.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ClkGpout3SelectedVal {
    bits: u32,
}

impl ClkGpout3SelectedVal {
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
        Self { bits: 0x00000001 }
    }

    /// Read **CLK_GPOUT3_SELECTED** `[0:0]`
    ///
    /// This slice does not have a glitchless mux (only the AUX_SRC field is present, not SRC) so this register is hardwired to 0x1.
    #[inline(always)]
    pub const fn clk_gpout3_selected(&self) -> bool {
        (self.bits & ClkGpout3Selected::CLK_GPOUT3_SELECTED_MASK) != 0
    }
}

impl Default for ClkGpout3SelectedVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl ClkGpout3Selected {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ClkGpout3SelectedVal {
        ClkGpout3SelectedVal::from_bits(self.value.read_bits())
    }
}

/// CLK_REF_CTRL Register
///
/// Clock control, can be changed on-the-fly (except for auxsrc)
///
/// Address offset: 0x0030
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **AUXSRC** `[6:5]` (RW)
///
///   Selects the auxiliary clock source, will glitch when switching
///
/// - **SRC** `[1:0]` (RW)
///
///   Selects the clock source glitchlessly, can be changed on-the-fly
#[repr(transparent)]
pub struct ClkRefCtrl {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for ClkRefCtrl {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ClkRefCtrl {
    // ======== Field Constants ========

    /// **AUXSRC**
    ///
    /// Selects the auxiliary clock source, will glitch when switching
    ///
    /// Bits `[6:5]`, mask `0x00000060`
    pub const AUXSRC_OFFSET: u32 = 5;
    pub const AUXSRC_MASK: u32 = 0x00000060;
    pub const AUXSRC_CLKSRC_PLL_USB: u32 = 0 << 5;
    pub const AUXSRC_CLKSRC_PLL_USB_VAL: u32 = 0;
    pub const AUXSRC_CLKSRC_GPIN0: u32 = 1 << 5;
    pub const AUXSRC_CLKSRC_GPIN0_VAL: u32 = 1;
    pub const AUXSRC_CLKSRC_GPIN1: u32 = 2 << 5;
    pub const AUXSRC_CLKSRC_GPIN1_VAL: u32 = 2;
    pub const AUXSRC_CLKSRC_PLL_USB_PRIMARY_REF_OPCG: u32 = 3 << 5;
    pub const AUXSRC_CLKSRC_PLL_USB_PRIMARY_REF_OPCG_VAL: u32 = 3;

    /// **SRC**
    ///
    /// Selects the clock source glitchlessly, can be changed on-the-fly
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const SRC_OFFSET: u32 = 0;
    pub const SRC_MASK: u32 = 0x00000003;
    pub const SRC_ROSC_CLKSRC_PH: u32 = 0 << 0;
    pub const SRC_ROSC_CLKSRC_PH_VAL: u32 = 0;
    pub const SRC_CLKSRC_CLK_REF_AUX: u32 = 1 << 0;
    pub const SRC_CLKSRC_CLK_REF_AUX_VAL: u32 = 1;
    pub const SRC_XOSC_CLKSRC: u32 = 2 << 0;
    pub const SRC_XOSC_CLKSRC_VAL: u32 = 2;
    pub const SRC_LPOSC_CLKSRC: u32 = 3 << 0;
    pub const SRC_LPOSC_CLKSRC_VAL: u32 = 3;

    // ======== Field Accessors ========

    /// **AUXSRC** `[6:5]` (RW)
    ///
    /// Selects the auxiliary clock source, will glitch when switching
    #[inline(always)]
    pub fn auxsrc(&self) -> u32 {
        (self.value.read_bits() & Self::AUXSRC_MASK) >> Self::AUXSRC_OFFSET
    }

    /// Write **AUXSRC** `[6:5]` (RW)
    ///
    /// Selects the auxiliary clock source, will glitch when switching
    #[inline(always)]
    pub fn write_auxsrc(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::AUXSRC_MASK) | ((value << Self::AUXSRC_OFFSET) & Self::AUXSRC_MASK));
    }

    /// **SRC** `[1:0]` (RW)
    ///
    /// Selects the clock source glitchlessly, can be changed on-the-fly
    #[inline(always)]
    pub fn src(&self) -> u32 {
        (self.value.read_bits() & Self::SRC_MASK) >> Self::SRC_OFFSET
    }

    /// Write **SRC** `[1:0]` (RW)
    ///
    /// Selects the clock source glitchlessly, can be changed on-the-fly
    #[inline(always)]
    pub fn write_src(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SRC_MASK) | ((value << Self::SRC_OFFSET) & Self::SRC_MASK));
    }
}

/// CLK_REF_CTRL value type for batch read/modify/write operations
///
/// Use `clkrefctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `clkrefctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ClkRefCtrlVal {
    bits: u32,
}

impl ClkRefCtrlVal {
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

    /// Read **AUXSRC** `[6:5]`
    ///
    /// Selects the auxiliary clock source, will glitch when switching
    #[inline(always)]
    pub const fn auxsrc(&self) -> u32 {
        (self.bits & ClkRefCtrl::AUXSRC_MASK) >> ClkRefCtrl::AUXSRC_OFFSET
    }

    /// Read **SRC** `[1:0]`
    ///
    /// Selects the clock source glitchlessly, can be changed on-the-fly
    #[inline(always)]
    pub const fn src(&self) -> u32 {
        (self.bits & ClkRefCtrl::SRC_MASK) >> ClkRefCtrl::SRC_OFFSET
    }

    /// Set **AUXSRC** `[6:5]`
    ///
    /// Selects the auxiliary clock source, will glitch when switching
    #[inline(always)]
    pub const fn with_auxsrc(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ClkRefCtrl::AUXSRC_MASK) | ((value << ClkRefCtrl::AUXSRC_OFFSET) & ClkRefCtrl::AUXSRC_MASK)
        }
    }

    /// Set **SRC** `[1:0]`
    ///
    /// Selects the clock source glitchlessly, can be changed on-the-fly
    #[inline(always)]
    pub const fn with_src(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ClkRefCtrl::SRC_MASK) | ((value << ClkRefCtrl::SRC_OFFSET) & ClkRefCtrl::SRC_MASK)
        }
    }
}

impl Default for ClkRefCtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl ClkRefCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ClkRefCtrlVal {
        ClkRefCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ClkRefCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ClkRefCtrlVal) -> ClkRefCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CLK_REF_DIV Register
///
/// Address offset: 0x0034
/// Reset value: 0x00010000
///
/// ## Fields
///
/// - **INT** `[23:16]` (RW)
///
///   Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly
#[repr(transparent)]
pub struct ClkRefDiv {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for ClkRefDiv {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ClkRefDiv {
    // ======== Field Constants ========

    /// **INT**
    ///
    /// Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly
    ///
    /// Bits `[23:16]`, mask `0x00ff0000`
    pub const INT_OFFSET: u32 = 16;
    pub const INT_MASK: u32 = 0x00ff0000;

    // ======== Field Accessors ========

    /// **INT** `[23:16]` (RW)
    ///
    /// Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly
    #[inline(always)]
    pub fn int(&self) -> u32 {
        (self.value.read_bits() & Self::INT_MASK) >> Self::INT_OFFSET
    }

    /// Write **INT** `[23:16]` (RW)
    ///
    /// Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly
    #[inline(always)]
    pub fn write_int(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::INT_MASK) | ((value << Self::INT_OFFSET) & Self::INT_MASK));
    }
}

/// CLK_REF_DIV value type for batch read/modify/write operations
///
/// Use `clkrefdiv.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `clkrefdiv.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ClkRefDivVal {
    bits: u32,
}

impl ClkRefDivVal {
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

    /// Read **INT** `[23:16]`
    ///
    /// Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly
    #[inline(always)]
    pub const fn int(&self) -> u32 {
        (self.bits & ClkRefDiv::INT_MASK) >> ClkRefDiv::INT_OFFSET
    }

    /// Set **INT** `[23:16]`
    ///
    /// Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly
    #[inline(always)]
    pub const fn with_int(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ClkRefDiv::INT_MASK) | ((value << ClkRefDiv::INT_OFFSET) & ClkRefDiv::INT_MASK)
        }
    }
}

impl Default for ClkRefDivVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl ClkRefDiv {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ClkRefDivVal {
        ClkRefDivVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ClkRefDivVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ClkRefDivVal) -> ClkRefDivVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CLK_REF_SELECTED Register
///
/// Indicates which src is currently selected (one-hot)
///
/// Address offset: 0x0038
/// Reset value: 0x00000001
///
/// ## Fields
///
/// - **CLK_REF_SELECTED** `[3:0]` (RO)
///
///   The glitchless multiplexer does not switch instantaneously (to avoid glitches), so software should poll this register to wait for the switch to complete. This register contains one decoded bit for each of the clock sources enumerated in the CTRL SRC field. At most one of these bits will be set at any time, indicating that clock is currently present at the output of the glitchless mux. Whilst switching is in progress, this register may briefly show all-0s.
#[repr(transparent)]
pub struct ClkRefSelected {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for ClkRefSelected {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ClkRefSelected {
    // ======== Field Constants ========

    /// **CLK_REF_SELECTED**
    ///
    /// The glitchless multiplexer does not switch instantaneously (to avoid glitches), so software should poll this register to wait for the switch to complete. This register contains one decoded bit for each of the clock sources enumerated in the CTRL SRC field. At most one of these bits will be set at any time, indicating that clock is currently present at the output of the glitchless mux. Whilst switching is in progress, this register may briefly show all-0s.
    ///
    /// Bits `[3:0]`, mask `0x0000000f`
    pub const CLK_REF_SELECTED_OFFSET: u32 = 0;
    pub const CLK_REF_SELECTED_MASK: u32 = 0x0000000f;

    // ======== Field Accessors ========

    /// **CLK_REF_SELECTED** `[3:0]` (RO)
    ///
    /// The glitchless multiplexer does not switch instantaneously (to avoid glitches), so software should poll this register to wait for the switch to complete. This register contains one decoded bit for each of the clock sources enumerated in the CTRL SRC field. At most one of these bits will be set at any time, indicating that clock is currently present at the output of the glitchless mux. Whilst switching is in progress, this register may briefly show all-0s.
    #[inline(always)]
    pub fn clk_ref_selected(&self) -> u32 {
        (self.value.read_bits() & Self::CLK_REF_SELECTED_MASK) >> Self::CLK_REF_SELECTED_OFFSET
    }
}

/// CLK_REF_SELECTED value type for batch read/modify/write operations
///
/// Use `clkrefselected.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `clkrefselected.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ClkRefSelectedVal {
    bits: u32,
}

impl ClkRefSelectedVal {
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
        Self { bits: 0x00000001 }
    }

    /// Read **CLK_REF_SELECTED** `[3:0]`
    ///
    /// The glitchless multiplexer does not switch instantaneously (to avoid glitches), so software should poll this register to wait for the switch to complete. This register contains one decoded bit for each of the clock sources enumerated in the CTRL SRC field. At most one of these bits will be set at any time, indicating that clock is currently present at the output of the glitchless mux. Whilst switching is in progress, this register may briefly show all-0s.
    #[inline(always)]
    pub const fn clk_ref_selected(&self) -> u32 {
        (self.bits & ClkRefSelected::CLK_REF_SELECTED_MASK) >> ClkRefSelected::CLK_REF_SELECTED_OFFSET
    }
}

impl Default for ClkRefSelectedVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl ClkRefSelected {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ClkRefSelectedVal {
        ClkRefSelectedVal::from_bits(self.value.read_bits())
    }
}

/// CLK_SYS_CTRL Register
///
/// Clock control, can be changed on-the-fly (except for auxsrc)
///
/// Address offset: 0x003c
/// Reset value: 0x00000041
///
/// ## Fields
///
/// - **AUXSRC** `[7:5]` (RW)
///
///   Selects the auxiliary clock source, will glitch when switching
///
/// - **SRC** `[0]` (RW)
///
///   Selects the clock source glitchlessly, can be changed on-the-fly
#[repr(transparent)]
pub struct ClkSysCtrl {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for ClkSysCtrl {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ClkSysCtrl {
    // ======== Field Constants ========

    /// **AUXSRC**
    ///
    /// Selects the auxiliary clock source, will glitch when switching
    ///
    /// Bits `[7:5]`, mask `0x000000e0`
    pub const AUXSRC_OFFSET: u32 = 5;
    pub const AUXSRC_MASK: u32 = 0x000000e0;
    pub const AUXSRC_CLKSRC_PLL_SYS: u32 = 0 << 5;
    pub const AUXSRC_CLKSRC_PLL_SYS_VAL: u32 = 0;
    pub const AUXSRC_CLKSRC_PLL_USB: u32 = 1 << 5;
    pub const AUXSRC_CLKSRC_PLL_USB_VAL: u32 = 1;
    pub const AUXSRC_ROSC_CLKSRC: u32 = 2 << 5;
    pub const AUXSRC_ROSC_CLKSRC_VAL: u32 = 2;
    pub const AUXSRC_XOSC_CLKSRC: u32 = 3 << 5;
    pub const AUXSRC_XOSC_CLKSRC_VAL: u32 = 3;
    pub const AUXSRC_CLKSRC_GPIN0: u32 = 4 << 5;
    pub const AUXSRC_CLKSRC_GPIN0_VAL: u32 = 4;
    pub const AUXSRC_CLKSRC_GPIN1: u32 = 5 << 5;
    pub const AUXSRC_CLKSRC_GPIN1_VAL: u32 = 5;

    /// **SRC**
    ///
    /// Selects the clock source glitchlessly, can be changed on-the-fly
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SRC_OFFSET: u32 = 0;
    pub const SRC_MASK: u32 = 0x00000001;
    pub const SRC: u32 = 1 << 0;
    pub const SRC_CLK_REF: u32 = 0 << 0;
    pub const SRC_CLK_REF_VAL: u32 = 0;
    pub const SRC_CLKSRC_CLK_SYS_AUX: u32 = 1 << 0;
    pub const SRC_CLKSRC_CLK_SYS_AUX_VAL: u32 = 1;

    // ======== Field Accessors ========

    /// **AUXSRC** `[7:5]` (RW)
    ///
    /// Selects the auxiliary clock source, will glitch when switching
    #[inline(always)]
    pub fn auxsrc(&self) -> u32 {
        (self.value.read_bits() & Self::AUXSRC_MASK) >> Self::AUXSRC_OFFSET
    }

    /// Write **AUXSRC** `[7:5]` (RW)
    ///
    /// Selects the auxiliary clock source, will glitch when switching
    #[inline(always)]
    pub fn write_auxsrc(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::AUXSRC_MASK) | ((value << Self::AUXSRC_OFFSET) & Self::AUXSRC_MASK));
    }

    /// **SRC** `[0]` (RW)
    ///
    /// Selects the clock source glitchlessly, can be changed on-the-fly
    #[inline(always)]
    pub fn src(&self) -> bool {
        (self.value.read_bits() & Self::SRC_MASK) != 0
    }

    /// Set SRC bit atomically (RW)
    #[inline(always)]
    pub fn set_src(&self) {
        self.set_bits(Self::SRC_MASK);
    }

    /// Clear SRC bit atomically (RW)
    #[inline(always)]
    pub fn clr_src(&self) {
        self.clr_bits(Self::SRC_MASK);
    }
}

/// CLK_SYS_CTRL value type for batch read/modify/write operations
///
/// Use `clksysctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `clksysctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ClkSysCtrlVal {
    bits: u32,
}

impl ClkSysCtrlVal {
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
        Self { bits: 0x00000041 }
    }

    /// Read **AUXSRC** `[7:5]`
    ///
    /// Selects the auxiliary clock source, will glitch when switching
    #[inline(always)]
    pub const fn auxsrc(&self) -> u32 {
        (self.bits & ClkSysCtrl::AUXSRC_MASK) >> ClkSysCtrl::AUXSRC_OFFSET
    }

    /// Read **SRC** `[0:0]`
    ///
    /// Selects the clock source glitchlessly, can be changed on-the-fly
    #[inline(always)]
    pub const fn src(&self) -> bool {
        (self.bits & ClkSysCtrl::SRC_MASK) != 0
    }

    /// Set **AUXSRC** `[7:5]`
    ///
    /// Selects the auxiliary clock source, will glitch when switching
    #[inline(always)]
    pub const fn with_auxsrc(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ClkSysCtrl::AUXSRC_MASK) | ((value << ClkSysCtrl::AUXSRC_OFFSET) & ClkSysCtrl::AUXSRC_MASK)
        }
    }

    /// Set **SRC** `[0:0]`
    ///
    /// Selects the clock source glitchlessly, can be changed on-the-fly
    #[inline(always)]
    pub const fn with_src(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClkSysCtrl::SRC_MASK
            } else {
                self.bits & !ClkSysCtrl::SRC_MASK
            }
        }
    }
}

impl Default for ClkSysCtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl ClkSysCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ClkSysCtrlVal {
        ClkSysCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ClkSysCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ClkSysCtrlVal) -> ClkSysCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CLK_SYS_DIV Register
///
/// Address offset: 0x0040
/// Reset value: 0x00010000
///
/// ## Fields
///
/// - **INT** `[31:16]` (RW)
///
///   Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly
///
/// - **FRAC** `[15:0]` (RW)
///
///   Fractional component of the divisor, can be changed on-the-fly
#[repr(transparent)]
pub struct ClkSysDiv {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for ClkSysDiv {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ClkSysDiv {
    // ======== Field Constants ========

    /// **INT**
    ///
    /// Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const INT_OFFSET: u32 = 16;
    pub const INT_MASK: u32 = 0xffff0000;

    /// **FRAC**
    ///
    /// Fractional component of the divisor, can be changed on-the-fly
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const FRAC_OFFSET: u32 = 0;
    pub const FRAC_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **INT** `[31:16]` (RW)
    ///
    /// Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly
    #[inline(always)]
    pub fn int(&self) -> u32 {
        (self.value.read_bits() & Self::INT_MASK) >> Self::INT_OFFSET
    }

    /// Write **INT** `[31:16]` (RW)
    ///
    /// Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly
    #[inline(always)]
    pub fn write_int(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::INT_MASK) | ((value << Self::INT_OFFSET) & Self::INT_MASK));
    }

    /// **FRAC** `[15:0]` (RW)
    ///
    /// Fractional component of the divisor, can be changed on-the-fly
    #[inline(always)]
    pub fn frac(&self) -> u32 {
        (self.value.read_bits() & Self::FRAC_MASK) >> Self::FRAC_OFFSET
    }

    /// Write **FRAC** `[15:0]` (RW)
    ///
    /// Fractional component of the divisor, can be changed on-the-fly
    #[inline(always)]
    pub fn write_frac(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::FRAC_MASK) | ((value << Self::FRAC_OFFSET) & Self::FRAC_MASK));
    }
}

/// CLK_SYS_DIV value type for batch read/modify/write operations
///
/// Use `clksysdiv.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `clksysdiv.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ClkSysDivVal {
    bits: u32,
}

impl ClkSysDivVal {
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
    /// Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly
    #[inline(always)]
    pub const fn int(&self) -> u32 {
        (self.bits & ClkSysDiv::INT_MASK) >> ClkSysDiv::INT_OFFSET
    }

    /// Read **FRAC** `[15:0]`
    ///
    /// Fractional component of the divisor, can be changed on-the-fly
    #[inline(always)]
    pub const fn frac(&self) -> u32 {
        (self.bits & ClkSysDiv::FRAC_MASK) >> ClkSysDiv::FRAC_OFFSET
    }

    /// Set **INT** `[31:16]`
    ///
    /// Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly
    #[inline(always)]
    pub const fn with_int(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ClkSysDiv::INT_MASK) | ((value << ClkSysDiv::INT_OFFSET) & ClkSysDiv::INT_MASK)
        }
    }

    /// Set **FRAC** `[15:0]`
    ///
    /// Fractional component of the divisor, can be changed on-the-fly
    #[inline(always)]
    pub const fn with_frac(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ClkSysDiv::FRAC_MASK) | ((value << ClkSysDiv::FRAC_OFFSET) & ClkSysDiv::FRAC_MASK)
        }
    }
}

impl Default for ClkSysDivVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl ClkSysDiv {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ClkSysDivVal {
        ClkSysDivVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ClkSysDivVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ClkSysDivVal) -> ClkSysDivVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CLK_SYS_SELECTED Register
///
/// Indicates which src is currently selected (one-hot)
///
/// Address offset: 0x0044
/// Reset value: 0x00000001
///
/// ## Fields
///
/// - **CLK_SYS_SELECTED** `[1:0]` (RO)
///
///   The glitchless multiplexer does not switch instantaneously (to avoid glitches), so software should poll this register to wait for the switch to complete. This register contains one decoded bit for each of the clock sources enumerated in the CTRL SRC field. At most one of these bits will be set at any time, indicating that clock is currently present at the output of the glitchless mux. Whilst switching is in progress, this register may briefly show all-0s.
#[repr(transparent)]
pub struct ClkSysSelected {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for ClkSysSelected {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ClkSysSelected {
    // ======== Field Constants ========

    /// **CLK_SYS_SELECTED**
    ///
    /// The glitchless multiplexer does not switch instantaneously (to avoid glitches), so software should poll this register to wait for the switch to complete. This register contains one decoded bit for each of the clock sources enumerated in the CTRL SRC field. At most one of these bits will be set at any time, indicating that clock is currently present at the output of the glitchless mux. Whilst switching is in progress, this register may briefly show all-0s.
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const CLK_SYS_SELECTED_OFFSET: u32 = 0;
    pub const CLK_SYS_SELECTED_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **CLK_SYS_SELECTED** `[1:0]` (RO)
    ///
    /// The glitchless multiplexer does not switch instantaneously (to avoid glitches), so software should poll this register to wait for the switch to complete. This register contains one decoded bit for each of the clock sources enumerated in the CTRL SRC field. At most one of these bits will be set at any time, indicating that clock is currently present at the output of the glitchless mux. Whilst switching is in progress, this register may briefly show all-0s.
    #[inline(always)]
    pub fn clk_sys_selected(&self) -> u32 {
        (self.value.read_bits() & Self::CLK_SYS_SELECTED_MASK) >> Self::CLK_SYS_SELECTED_OFFSET
    }
}

/// CLK_SYS_SELECTED value type for batch read/modify/write operations
///
/// Use `clksysselected.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `clksysselected.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ClkSysSelectedVal {
    bits: u32,
}

impl ClkSysSelectedVal {
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
        Self { bits: 0x00000001 }
    }

    /// Read **CLK_SYS_SELECTED** `[1:0]`
    ///
    /// The glitchless multiplexer does not switch instantaneously (to avoid glitches), so software should poll this register to wait for the switch to complete. This register contains one decoded bit for each of the clock sources enumerated in the CTRL SRC field. At most one of these bits will be set at any time, indicating that clock is currently present at the output of the glitchless mux. Whilst switching is in progress, this register may briefly show all-0s.
    #[inline(always)]
    pub const fn clk_sys_selected(&self) -> u32 {
        (self.bits & ClkSysSelected::CLK_SYS_SELECTED_MASK) >> ClkSysSelected::CLK_SYS_SELECTED_OFFSET
    }
}

impl Default for ClkSysSelectedVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl ClkSysSelected {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ClkSysSelectedVal {
        ClkSysSelectedVal::from_bits(self.value.read_bits())
    }
}

/// CLK_PERI_CTRL Register
///
/// Clock control, can be changed on-the-fly (except for auxsrc)
///
/// Address offset: 0x0048
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **ENABLED** `[28]` (RO)
///
///   clock generator is enabled
///
/// - **ENABLE** `[11]` (RW)
///
///   Starts and stops the clock generator cleanly
///
/// - **KILL** `[10]` (RW)
///
///   Asynchronously kills the clock generator, enable must be set low before deasserting kill
///
/// - **AUXSRC** `[7:5]` (RW)
///
///   Selects the auxiliary clock source, will glitch when switching
#[repr(transparent)]
pub struct ClkPeriCtrl {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for ClkPeriCtrl {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ClkPeriCtrl {
    // ======== Field Constants ========

    /// **ENABLED**
    ///
    /// clock generator is enabled
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const ENABLED_OFFSET: u32 = 28;
    pub const ENABLED_MASK: u32 = 0x10000000;
    pub const ENABLED: u32 = 1 << 28;

    /// **ENABLE**
    ///
    /// Starts and stops the clock generator cleanly
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const ENABLE_OFFSET: u32 = 11;
    pub const ENABLE_MASK: u32 = 0x00000800;
    pub const ENABLE: u32 = 1 << 11;

    /// **KILL**
    ///
    /// Asynchronously kills the clock generator, enable must be set low before deasserting kill
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const KILL_OFFSET: u32 = 10;
    pub const KILL_MASK: u32 = 0x00000400;
    pub const KILL: u32 = 1 << 10;

    /// **AUXSRC**
    ///
    /// Selects the auxiliary clock source, will glitch when switching
    ///
    /// Bits `[7:5]`, mask `0x000000e0`
    pub const AUXSRC_OFFSET: u32 = 5;
    pub const AUXSRC_MASK: u32 = 0x000000e0;
    pub const AUXSRC_CLK_SYS: u32 = 0 << 5;
    pub const AUXSRC_CLK_SYS_VAL: u32 = 0;
    pub const AUXSRC_CLKSRC_PLL_SYS: u32 = 1 << 5;
    pub const AUXSRC_CLKSRC_PLL_SYS_VAL: u32 = 1;
    pub const AUXSRC_CLKSRC_PLL_USB: u32 = 2 << 5;
    pub const AUXSRC_CLKSRC_PLL_USB_VAL: u32 = 2;
    pub const AUXSRC_ROSC_CLKSRC_PH: u32 = 3 << 5;
    pub const AUXSRC_ROSC_CLKSRC_PH_VAL: u32 = 3;
    pub const AUXSRC_XOSC_CLKSRC: u32 = 4 << 5;
    pub const AUXSRC_XOSC_CLKSRC_VAL: u32 = 4;
    pub const AUXSRC_CLKSRC_GPIN0: u32 = 5 << 5;
    pub const AUXSRC_CLKSRC_GPIN0_VAL: u32 = 5;
    pub const AUXSRC_CLKSRC_GPIN1: u32 = 6 << 5;
    pub const AUXSRC_CLKSRC_GPIN1_VAL: u32 = 6;

    // ======== Field Accessors ========

    /// **ENABLED** `[28]` (RO)
    ///
    /// clock generator is enabled
    #[inline(always)]
    pub fn enabled(&self) -> bool {
        (self.value.read_bits() & Self::ENABLED_MASK) != 0
    }

    /// **ENABLE** `[11]` (RW)
    ///
    /// Starts and stops the clock generator cleanly
    #[inline(always)]
    pub fn enable(&self) -> bool {
        (self.value.read_bits() & Self::ENABLE_MASK) != 0
    }

    /// Set ENABLE bit atomically (RW)
    #[inline(always)]
    pub fn set_enable(&self) {
        self.set_bits(Self::ENABLE_MASK);
    }

    /// Clear ENABLE bit atomically (RW)
    #[inline(always)]
    pub fn clr_enable(&self) {
        self.clr_bits(Self::ENABLE_MASK);
    }

    /// **KILL** `[10]` (RW)
    ///
    /// Asynchronously kills the clock generator, enable must be set low before deasserting kill
    #[inline(always)]
    pub fn kill(&self) -> bool {
        (self.value.read_bits() & Self::KILL_MASK) != 0
    }

    /// Set KILL bit atomically (RW)
    #[inline(always)]
    pub fn set_kill(&self) {
        self.set_bits(Self::KILL_MASK);
    }

    /// Clear KILL bit atomically (RW)
    #[inline(always)]
    pub fn clr_kill(&self) {
        self.clr_bits(Self::KILL_MASK);
    }

    /// **AUXSRC** `[7:5]` (RW)
    ///
    /// Selects the auxiliary clock source, will glitch when switching
    #[inline(always)]
    pub fn auxsrc(&self) -> u32 {
        (self.value.read_bits() & Self::AUXSRC_MASK) >> Self::AUXSRC_OFFSET
    }

    /// Write **AUXSRC** `[7:5]` (RW)
    ///
    /// Selects the auxiliary clock source, will glitch when switching
    #[inline(always)]
    pub fn write_auxsrc(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::AUXSRC_MASK) | ((value << Self::AUXSRC_OFFSET) & Self::AUXSRC_MASK));
    }
}

/// CLK_PERI_CTRL value type for batch read/modify/write operations
///
/// Use `clkperictrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `clkperictrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ClkPeriCtrlVal {
    bits: u32,
}

impl ClkPeriCtrlVal {
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

    /// Read **ENABLED** `[28:28]`
    ///
    /// clock generator is enabled
    #[inline(always)]
    pub const fn enabled(&self) -> bool {
        (self.bits & ClkPeriCtrl::ENABLED_MASK) != 0
    }

    /// Read **ENABLE** `[11:11]`
    ///
    /// Starts and stops the clock generator cleanly
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        (self.bits & ClkPeriCtrl::ENABLE_MASK) != 0
    }

    /// Read **KILL** `[10:10]`
    ///
    /// Asynchronously kills the clock generator, enable must be set low before deasserting kill
    #[inline(always)]
    pub const fn kill(&self) -> bool {
        (self.bits & ClkPeriCtrl::KILL_MASK) != 0
    }

    /// Read **AUXSRC** `[7:5]`
    ///
    /// Selects the auxiliary clock source, will glitch when switching
    #[inline(always)]
    pub const fn auxsrc(&self) -> u32 {
        (self.bits & ClkPeriCtrl::AUXSRC_MASK) >> ClkPeriCtrl::AUXSRC_OFFSET
    }

    /// Set **ENABLE** `[11:11]`
    ///
    /// Starts and stops the clock generator cleanly
    #[inline(always)]
    pub const fn with_enable(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClkPeriCtrl::ENABLE_MASK
            } else {
                self.bits & !ClkPeriCtrl::ENABLE_MASK
            }
        }
    }

    /// Set **KILL** `[10:10]`
    ///
    /// Asynchronously kills the clock generator, enable must be set low before deasserting kill
    #[inline(always)]
    pub const fn with_kill(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClkPeriCtrl::KILL_MASK
            } else {
                self.bits & !ClkPeriCtrl::KILL_MASK
            }
        }
    }

    /// Set **AUXSRC** `[7:5]`
    ///
    /// Selects the auxiliary clock source, will glitch when switching
    #[inline(always)]
    pub const fn with_auxsrc(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ClkPeriCtrl::AUXSRC_MASK) | ((value << ClkPeriCtrl::AUXSRC_OFFSET) & ClkPeriCtrl::AUXSRC_MASK)
        }
    }
}

impl Default for ClkPeriCtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl ClkPeriCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ClkPeriCtrlVal {
        ClkPeriCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ClkPeriCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ClkPeriCtrlVal) -> ClkPeriCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CLK_PERI_DIV Register
///
/// Address offset: 0x004c
/// Reset value: 0x00010000
///
/// ## Fields
///
/// - **INT** `[17:16]` (RW)
///
///   Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly
#[repr(transparent)]
pub struct ClkPeriDiv {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for ClkPeriDiv {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ClkPeriDiv {
    // ======== Field Constants ========

    /// **INT**
    ///
    /// Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly
    ///
    /// Bits `[17:16]`, mask `0x00030000`
    pub const INT_OFFSET: u32 = 16;
    pub const INT_MASK: u32 = 0x00030000;

    // ======== Field Accessors ========

    /// **INT** `[17:16]` (RW)
    ///
    /// Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly
    #[inline(always)]
    pub fn int(&self) -> u32 {
        (self.value.read_bits() & Self::INT_MASK) >> Self::INT_OFFSET
    }

    /// Write **INT** `[17:16]` (RW)
    ///
    /// Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly
    #[inline(always)]
    pub fn write_int(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::INT_MASK) | ((value << Self::INT_OFFSET) & Self::INT_MASK));
    }
}

/// CLK_PERI_DIV value type for batch read/modify/write operations
///
/// Use `clkperidiv.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `clkperidiv.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ClkPeriDivVal {
    bits: u32,
}

impl ClkPeriDivVal {
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

    /// Read **INT** `[17:16]`
    ///
    /// Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly
    #[inline(always)]
    pub const fn int(&self) -> u32 {
        (self.bits & ClkPeriDiv::INT_MASK) >> ClkPeriDiv::INT_OFFSET
    }

    /// Set **INT** `[17:16]`
    ///
    /// Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly
    #[inline(always)]
    pub const fn with_int(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ClkPeriDiv::INT_MASK) | ((value << ClkPeriDiv::INT_OFFSET) & ClkPeriDiv::INT_MASK)
        }
    }
}

impl Default for ClkPeriDivVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl ClkPeriDiv {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ClkPeriDivVal {
        ClkPeriDivVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ClkPeriDivVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ClkPeriDivVal) -> ClkPeriDivVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CLK_PERI_SELECTED Register
///
/// Indicates which src is currently selected (one-hot)
///
/// Address offset: 0x0050
/// Reset value: 0x00000001
///
/// ## Fields
///
/// - **CLK_PERI_SELECTED** `[0]` (RO)
///
///   This slice does not have a glitchless mux (only the AUX_SRC field is present, not SRC) so this register is hardwired to 0x1.
#[repr(transparent)]
pub struct ClkPeriSelected {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for ClkPeriSelected {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ClkPeriSelected {
    // ======== Field Constants ========

    /// **CLK_PERI_SELECTED**
    ///
    /// This slice does not have a glitchless mux (only the AUX_SRC field is present, not SRC) so this register is hardwired to 0x1.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CLK_PERI_SELECTED_OFFSET: u32 = 0;
    pub const CLK_PERI_SELECTED_MASK: u32 = 0x00000001;
    pub const CLK_PERI_SELECTED: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CLK_PERI_SELECTED** `[0]` (RO)
    ///
    /// This slice does not have a glitchless mux (only the AUX_SRC field is present, not SRC) so this register is hardwired to 0x1.
    #[inline(always)]
    pub fn clk_peri_selected(&self) -> bool {
        (self.value.read_bits() & Self::CLK_PERI_SELECTED_MASK) != 0
    }
}

/// CLK_PERI_SELECTED value type for batch read/modify/write operations
///
/// Use `clkperiselected.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `clkperiselected.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ClkPeriSelectedVal {
    bits: u32,
}

impl ClkPeriSelectedVal {
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
        Self { bits: 0x00000001 }
    }

    /// Read **CLK_PERI_SELECTED** `[0:0]`
    ///
    /// This slice does not have a glitchless mux (only the AUX_SRC field is present, not SRC) so this register is hardwired to 0x1.
    #[inline(always)]
    pub const fn clk_peri_selected(&self) -> bool {
        (self.bits & ClkPeriSelected::CLK_PERI_SELECTED_MASK) != 0
    }
}

impl Default for ClkPeriSelectedVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl ClkPeriSelected {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ClkPeriSelectedVal {
        ClkPeriSelectedVal::from_bits(self.value.read_bits())
    }
}

/// CLK_HSTX_CTRL Register
///
/// Clock control, can be changed on-the-fly (except for auxsrc)
///
/// Address offset: 0x0054
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **ENABLED** `[28]` (RO)
///
///   clock generator is enabled
///
/// - **NUDGE** `[20]` (RW)
///
///   An edge on this signal shifts the phase of the output by 1 cycle of the input clock
///   This can be done at any time
///
/// - **PHASE** `[17:16]` (RW)
///
///   This delays the enable signal by up to 3 cycles of the input clock
///   This must be set before the clock is enabled to have any effect
///
/// - **ENABLE** `[11]` (RW)
///
///   Starts and stops the clock generator cleanly
///
/// - **KILL** `[10]` (RW)
///
///   Asynchronously kills the clock generator, enable must be set low before deasserting kill
///
/// - **AUXSRC** `[7:5]` (RW)
///
///   Selects the auxiliary clock source, will glitch when switching
#[repr(transparent)]
pub struct ClkHstxCtrl {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for ClkHstxCtrl {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ClkHstxCtrl {
    // ======== Field Constants ========

    /// **ENABLED**
    ///
    /// clock generator is enabled
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const ENABLED_OFFSET: u32 = 28;
    pub const ENABLED_MASK: u32 = 0x10000000;
    pub const ENABLED: u32 = 1 << 28;

    /// **NUDGE**
    ///
    /// An edge on this signal shifts the phase of the output by 1 cycle of the input clock
    /// This can be done at any time
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const NUDGE_OFFSET: u32 = 20;
    pub const NUDGE_MASK: u32 = 0x00100000;
    pub const NUDGE: u32 = 1 << 20;

    /// **PHASE**
    ///
    /// This delays the enable signal by up to 3 cycles of the input clock
    /// This must be set before the clock is enabled to have any effect
    ///
    /// Bits `[17:16]`, mask `0x00030000`
    pub const PHASE_OFFSET: u32 = 16;
    pub const PHASE_MASK: u32 = 0x00030000;

    /// **ENABLE**
    ///
    /// Starts and stops the clock generator cleanly
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const ENABLE_OFFSET: u32 = 11;
    pub const ENABLE_MASK: u32 = 0x00000800;
    pub const ENABLE: u32 = 1 << 11;

    /// **KILL**
    ///
    /// Asynchronously kills the clock generator, enable must be set low before deasserting kill
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const KILL_OFFSET: u32 = 10;
    pub const KILL_MASK: u32 = 0x00000400;
    pub const KILL: u32 = 1 << 10;

    /// **AUXSRC**
    ///
    /// Selects the auxiliary clock source, will glitch when switching
    ///
    /// Bits `[7:5]`, mask `0x000000e0`
    pub const AUXSRC_OFFSET: u32 = 5;
    pub const AUXSRC_MASK: u32 = 0x000000e0;
    pub const AUXSRC_CLK_SYS: u32 = 0 << 5;
    pub const AUXSRC_CLK_SYS_VAL: u32 = 0;
    pub const AUXSRC_CLKSRC_PLL_SYS: u32 = 1 << 5;
    pub const AUXSRC_CLKSRC_PLL_SYS_VAL: u32 = 1;
    pub const AUXSRC_CLKSRC_PLL_USB: u32 = 2 << 5;
    pub const AUXSRC_CLKSRC_PLL_USB_VAL: u32 = 2;
    pub const AUXSRC_CLKSRC_GPIN0: u32 = 3 << 5;
    pub const AUXSRC_CLKSRC_GPIN0_VAL: u32 = 3;
    pub const AUXSRC_CLKSRC_GPIN1: u32 = 4 << 5;
    pub const AUXSRC_CLKSRC_GPIN1_VAL: u32 = 4;

    // ======== Field Accessors ========

    /// **ENABLED** `[28]` (RO)
    ///
    /// clock generator is enabled
    #[inline(always)]
    pub fn enabled(&self) -> bool {
        (self.value.read_bits() & Self::ENABLED_MASK) != 0
    }

    /// **NUDGE** `[20]` (RW)
    ///
    /// An edge on this signal shifts the phase of the output by 1 cycle of the input clock
    /// This can be done at any time
    #[inline(always)]
    pub fn nudge(&self) -> bool {
        (self.value.read_bits() & Self::NUDGE_MASK) != 0
    }

    /// Set NUDGE bit atomically (RW)
    #[inline(always)]
    pub fn set_nudge(&self) {
        self.set_bits(Self::NUDGE_MASK);
    }

    /// Clear NUDGE bit atomically (RW)
    #[inline(always)]
    pub fn clr_nudge(&self) {
        self.clr_bits(Self::NUDGE_MASK);
    }

    /// **PHASE** `[17:16]` (RW)
    ///
    /// This delays the enable signal by up to 3 cycles of the input clock
    /// This must be set before the clock is enabled to have any effect
    #[inline(always)]
    pub fn phase(&self) -> u32 {
        (self.value.read_bits() & Self::PHASE_MASK) >> Self::PHASE_OFFSET
    }

    /// Write **PHASE** `[17:16]` (RW)
    ///
    /// This delays the enable signal by up to 3 cycles of the input clock
    /// This must be set before the clock is enabled to have any effect
    #[inline(always)]
    pub fn write_phase(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::PHASE_MASK) | ((value << Self::PHASE_OFFSET) & Self::PHASE_MASK));
    }

    /// **ENABLE** `[11]` (RW)
    ///
    /// Starts and stops the clock generator cleanly
    #[inline(always)]
    pub fn enable(&self) -> bool {
        (self.value.read_bits() & Self::ENABLE_MASK) != 0
    }

    /// Set ENABLE bit atomically (RW)
    #[inline(always)]
    pub fn set_enable(&self) {
        self.set_bits(Self::ENABLE_MASK);
    }

    /// Clear ENABLE bit atomically (RW)
    #[inline(always)]
    pub fn clr_enable(&self) {
        self.clr_bits(Self::ENABLE_MASK);
    }

    /// **KILL** `[10]` (RW)
    ///
    /// Asynchronously kills the clock generator, enable must be set low before deasserting kill
    #[inline(always)]
    pub fn kill(&self) -> bool {
        (self.value.read_bits() & Self::KILL_MASK) != 0
    }

    /// Set KILL bit atomically (RW)
    #[inline(always)]
    pub fn set_kill(&self) {
        self.set_bits(Self::KILL_MASK);
    }

    /// Clear KILL bit atomically (RW)
    #[inline(always)]
    pub fn clr_kill(&self) {
        self.clr_bits(Self::KILL_MASK);
    }

    /// **AUXSRC** `[7:5]` (RW)
    ///
    /// Selects the auxiliary clock source, will glitch when switching
    #[inline(always)]
    pub fn auxsrc(&self) -> u32 {
        (self.value.read_bits() & Self::AUXSRC_MASK) >> Self::AUXSRC_OFFSET
    }

    /// Write **AUXSRC** `[7:5]` (RW)
    ///
    /// Selects the auxiliary clock source, will glitch when switching
    #[inline(always)]
    pub fn write_auxsrc(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::AUXSRC_MASK) | ((value << Self::AUXSRC_OFFSET) & Self::AUXSRC_MASK));
    }
}

/// CLK_HSTX_CTRL value type for batch read/modify/write operations
///
/// Use `clkhstxctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `clkhstxctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ClkHstxCtrlVal {
    bits: u32,
}

impl ClkHstxCtrlVal {
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

    /// Read **ENABLED** `[28:28]`
    ///
    /// clock generator is enabled
    #[inline(always)]
    pub const fn enabled(&self) -> bool {
        (self.bits & ClkHstxCtrl::ENABLED_MASK) != 0
    }

    /// Read **NUDGE** `[20:20]`
    ///
    /// An edge on this signal shifts the phase of the output by 1 cycle of the input clock
    /// This can be done at any time
    #[inline(always)]
    pub const fn nudge(&self) -> bool {
        (self.bits & ClkHstxCtrl::NUDGE_MASK) != 0
    }

    /// Read **PHASE** `[17:16]`
    ///
    /// This delays the enable signal by up to 3 cycles of the input clock
    /// This must be set before the clock is enabled to have any effect
    #[inline(always)]
    pub const fn phase(&self) -> u32 {
        (self.bits & ClkHstxCtrl::PHASE_MASK) >> ClkHstxCtrl::PHASE_OFFSET
    }

    /// Read **ENABLE** `[11:11]`
    ///
    /// Starts and stops the clock generator cleanly
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        (self.bits & ClkHstxCtrl::ENABLE_MASK) != 0
    }

    /// Read **KILL** `[10:10]`
    ///
    /// Asynchronously kills the clock generator, enable must be set low before deasserting kill
    #[inline(always)]
    pub const fn kill(&self) -> bool {
        (self.bits & ClkHstxCtrl::KILL_MASK) != 0
    }

    /// Read **AUXSRC** `[7:5]`
    ///
    /// Selects the auxiliary clock source, will glitch when switching
    #[inline(always)]
    pub const fn auxsrc(&self) -> u32 {
        (self.bits & ClkHstxCtrl::AUXSRC_MASK) >> ClkHstxCtrl::AUXSRC_OFFSET
    }

    /// Set **NUDGE** `[20:20]`
    ///
    /// An edge on this signal shifts the phase of the output by 1 cycle of the input clock
    /// This can be done at any time
    #[inline(always)]
    pub const fn with_nudge(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClkHstxCtrl::NUDGE_MASK
            } else {
                self.bits & !ClkHstxCtrl::NUDGE_MASK
            }
        }
    }

    /// Set **PHASE** `[17:16]`
    ///
    /// This delays the enable signal by up to 3 cycles of the input clock
    /// This must be set before the clock is enabled to have any effect
    #[inline(always)]
    pub const fn with_phase(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ClkHstxCtrl::PHASE_MASK) | ((value << ClkHstxCtrl::PHASE_OFFSET) & ClkHstxCtrl::PHASE_MASK)
        }
    }

    /// Set **ENABLE** `[11:11]`
    ///
    /// Starts and stops the clock generator cleanly
    #[inline(always)]
    pub const fn with_enable(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClkHstxCtrl::ENABLE_MASK
            } else {
                self.bits & !ClkHstxCtrl::ENABLE_MASK
            }
        }
    }

    /// Set **KILL** `[10:10]`
    ///
    /// Asynchronously kills the clock generator, enable must be set low before deasserting kill
    #[inline(always)]
    pub const fn with_kill(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClkHstxCtrl::KILL_MASK
            } else {
                self.bits & !ClkHstxCtrl::KILL_MASK
            }
        }
    }

    /// Set **AUXSRC** `[7:5]`
    ///
    /// Selects the auxiliary clock source, will glitch when switching
    #[inline(always)]
    pub const fn with_auxsrc(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ClkHstxCtrl::AUXSRC_MASK) | ((value << ClkHstxCtrl::AUXSRC_OFFSET) & ClkHstxCtrl::AUXSRC_MASK)
        }
    }
}

impl Default for ClkHstxCtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl ClkHstxCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ClkHstxCtrlVal {
        ClkHstxCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ClkHstxCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ClkHstxCtrlVal) -> ClkHstxCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CLK_HSTX_DIV Register
///
/// Address offset: 0x0058
/// Reset value: 0x00010000
///
/// ## Fields
///
/// - **INT** `[17:16]` (RW)
///
///   Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly
#[repr(transparent)]
pub struct ClkHstxDiv {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for ClkHstxDiv {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ClkHstxDiv {
    // ======== Field Constants ========

    /// **INT**
    ///
    /// Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly
    ///
    /// Bits `[17:16]`, mask `0x00030000`
    pub const INT_OFFSET: u32 = 16;
    pub const INT_MASK: u32 = 0x00030000;

    // ======== Field Accessors ========

    /// **INT** `[17:16]` (RW)
    ///
    /// Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly
    #[inline(always)]
    pub fn int(&self) -> u32 {
        (self.value.read_bits() & Self::INT_MASK) >> Self::INT_OFFSET
    }

    /// Write **INT** `[17:16]` (RW)
    ///
    /// Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly
    #[inline(always)]
    pub fn write_int(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::INT_MASK) | ((value << Self::INT_OFFSET) & Self::INT_MASK));
    }
}

/// CLK_HSTX_DIV value type for batch read/modify/write operations
///
/// Use `clkhstxdiv.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `clkhstxdiv.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ClkHstxDivVal {
    bits: u32,
}

impl ClkHstxDivVal {
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

    /// Read **INT** `[17:16]`
    ///
    /// Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly
    #[inline(always)]
    pub const fn int(&self) -> u32 {
        (self.bits & ClkHstxDiv::INT_MASK) >> ClkHstxDiv::INT_OFFSET
    }

    /// Set **INT** `[17:16]`
    ///
    /// Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly
    #[inline(always)]
    pub const fn with_int(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ClkHstxDiv::INT_MASK) | ((value << ClkHstxDiv::INT_OFFSET) & ClkHstxDiv::INT_MASK)
        }
    }
}

impl Default for ClkHstxDivVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl ClkHstxDiv {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ClkHstxDivVal {
        ClkHstxDivVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ClkHstxDivVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ClkHstxDivVal) -> ClkHstxDivVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CLK_HSTX_SELECTED Register
///
/// Indicates which src is currently selected (one-hot)
///
/// Address offset: 0x005c
/// Reset value: 0x00000001
///
/// ## Fields
///
/// - **CLK_HSTX_SELECTED** `[0]` (RO)
///
///   This slice does not have a glitchless mux (only the AUX_SRC field is present, not SRC) so this register is hardwired to 0x1.
#[repr(transparent)]
pub struct ClkHstxSelected {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for ClkHstxSelected {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ClkHstxSelected {
    // ======== Field Constants ========

    /// **CLK_HSTX_SELECTED**
    ///
    /// This slice does not have a glitchless mux (only the AUX_SRC field is present, not SRC) so this register is hardwired to 0x1.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CLK_HSTX_SELECTED_OFFSET: u32 = 0;
    pub const CLK_HSTX_SELECTED_MASK: u32 = 0x00000001;
    pub const CLK_HSTX_SELECTED: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CLK_HSTX_SELECTED** `[0]` (RO)
    ///
    /// This slice does not have a glitchless mux (only the AUX_SRC field is present, not SRC) so this register is hardwired to 0x1.
    #[inline(always)]
    pub fn clk_hstx_selected(&self) -> bool {
        (self.value.read_bits() & Self::CLK_HSTX_SELECTED_MASK) != 0
    }
}

/// CLK_HSTX_SELECTED value type for batch read/modify/write operations
///
/// Use `clkhstxselected.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `clkhstxselected.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ClkHstxSelectedVal {
    bits: u32,
}

impl ClkHstxSelectedVal {
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
        Self { bits: 0x00000001 }
    }

    /// Read **CLK_HSTX_SELECTED** `[0:0]`
    ///
    /// This slice does not have a glitchless mux (only the AUX_SRC field is present, not SRC) so this register is hardwired to 0x1.
    #[inline(always)]
    pub const fn clk_hstx_selected(&self) -> bool {
        (self.bits & ClkHstxSelected::CLK_HSTX_SELECTED_MASK) != 0
    }
}

impl Default for ClkHstxSelectedVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl ClkHstxSelected {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ClkHstxSelectedVal {
        ClkHstxSelectedVal::from_bits(self.value.read_bits())
    }
}

/// CLK_USB_CTRL Register
///
/// Clock control, can be changed on-the-fly (except for auxsrc)
///
/// Address offset: 0x0060
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **ENABLED** `[28]` (RO)
///
///   clock generator is enabled
///
/// - **NUDGE** `[20]` (RW)
///
///   An edge on this signal shifts the phase of the output by 1 cycle of the input clock
///   This can be done at any time
///
/// - **PHASE** `[17:16]` (RW)
///
///   This delays the enable signal by up to 3 cycles of the input clock
///   This must be set before the clock is enabled to have any effect
///
/// - **ENABLE** `[11]` (RW)
///
///   Starts and stops the clock generator cleanly
///
/// - **KILL** `[10]` (RW)
///
///   Asynchronously kills the clock generator, enable must be set low before deasserting kill
///
/// - **AUXSRC** `[7:5]` (RW)
///
///   Selects the auxiliary clock source, will glitch when switching
#[repr(transparent)]
pub struct ClkUsbCtrl {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for ClkUsbCtrl {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ClkUsbCtrl {
    // ======== Field Constants ========

    /// **ENABLED**
    ///
    /// clock generator is enabled
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const ENABLED_OFFSET: u32 = 28;
    pub const ENABLED_MASK: u32 = 0x10000000;
    pub const ENABLED: u32 = 1 << 28;

    /// **NUDGE**
    ///
    /// An edge on this signal shifts the phase of the output by 1 cycle of the input clock
    /// This can be done at any time
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const NUDGE_OFFSET: u32 = 20;
    pub const NUDGE_MASK: u32 = 0x00100000;
    pub const NUDGE: u32 = 1 << 20;

    /// **PHASE**
    ///
    /// This delays the enable signal by up to 3 cycles of the input clock
    /// This must be set before the clock is enabled to have any effect
    ///
    /// Bits `[17:16]`, mask `0x00030000`
    pub const PHASE_OFFSET: u32 = 16;
    pub const PHASE_MASK: u32 = 0x00030000;

    /// **ENABLE**
    ///
    /// Starts and stops the clock generator cleanly
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const ENABLE_OFFSET: u32 = 11;
    pub const ENABLE_MASK: u32 = 0x00000800;
    pub const ENABLE: u32 = 1 << 11;

    /// **KILL**
    ///
    /// Asynchronously kills the clock generator, enable must be set low before deasserting kill
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const KILL_OFFSET: u32 = 10;
    pub const KILL_MASK: u32 = 0x00000400;
    pub const KILL: u32 = 1 << 10;

    /// **AUXSRC**
    ///
    /// Selects the auxiliary clock source, will glitch when switching
    ///
    /// Bits `[7:5]`, mask `0x000000e0`
    pub const AUXSRC_OFFSET: u32 = 5;
    pub const AUXSRC_MASK: u32 = 0x000000e0;
    pub const AUXSRC_CLKSRC_PLL_USB: u32 = 0 << 5;
    pub const AUXSRC_CLKSRC_PLL_USB_VAL: u32 = 0;
    pub const AUXSRC_CLKSRC_PLL_SYS: u32 = 1 << 5;
    pub const AUXSRC_CLKSRC_PLL_SYS_VAL: u32 = 1;
    pub const AUXSRC_ROSC_CLKSRC_PH: u32 = 2 << 5;
    pub const AUXSRC_ROSC_CLKSRC_PH_VAL: u32 = 2;
    pub const AUXSRC_XOSC_CLKSRC: u32 = 3 << 5;
    pub const AUXSRC_XOSC_CLKSRC_VAL: u32 = 3;
    pub const AUXSRC_CLKSRC_GPIN0: u32 = 4 << 5;
    pub const AUXSRC_CLKSRC_GPIN0_VAL: u32 = 4;
    pub const AUXSRC_CLKSRC_GPIN1: u32 = 5 << 5;
    pub const AUXSRC_CLKSRC_GPIN1_VAL: u32 = 5;

    // ======== Field Accessors ========

    /// **ENABLED** `[28]` (RO)
    ///
    /// clock generator is enabled
    #[inline(always)]
    pub fn enabled(&self) -> bool {
        (self.value.read_bits() & Self::ENABLED_MASK) != 0
    }

    /// **NUDGE** `[20]` (RW)
    ///
    /// An edge on this signal shifts the phase of the output by 1 cycle of the input clock
    /// This can be done at any time
    #[inline(always)]
    pub fn nudge(&self) -> bool {
        (self.value.read_bits() & Self::NUDGE_MASK) != 0
    }

    /// Set NUDGE bit atomically (RW)
    #[inline(always)]
    pub fn set_nudge(&self) {
        self.set_bits(Self::NUDGE_MASK);
    }

    /// Clear NUDGE bit atomically (RW)
    #[inline(always)]
    pub fn clr_nudge(&self) {
        self.clr_bits(Self::NUDGE_MASK);
    }

    /// **PHASE** `[17:16]` (RW)
    ///
    /// This delays the enable signal by up to 3 cycles of the input clock
    /// This must be set before the clock is enabled to have any effect
    #[inline(always)]
    pub fn phase(&self) -> u32 {
        (self.value.read_bits() & Self::PHASE_MASK) >> Self::PHASE_OFFSET
    }

    /// Write **PHASE** `[17:16]` (RW)
    ///
    /// This delays the enable signal by up to 3 cycles of the input clock
    /// This must be set before the clock is enabled to have any effect
    #[inline(always)]
    pub fn write_phase(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::PHASE_MASK) | ((value << Self::PHASE_OFFSET) & Self::PHASE_MASK));
    }

    /// **ENABLE** `[11]` (RW)
    ///
    /// Starts and stops the clock generator cleanly
    #[inline(always)]
    pub fn enable(&self) -> bool {
        (self.value.read_bits() & Self::ENABLE_MASK) != 0
    }

    /// Set ENABLE bit atomically (RW)
    #[inline(always)]
    pub fn set_enable(&self) {
        self.set_bits(Self::ENABLE_MASK);
    }

    /// Clear ENABLE bit atomically (RW)
    #[inline(always)]
    pub fn clr_enable(&self) {
        self.clr_bits(Self::ENABLE_MASK);
    }

    /// **KILL** `[10]` (RW)
    ///
    /// Asynchronously kills the clock generator, enable must be set low before deasserting kill
    #[inline(always)]
    pub fn kill(&self) -> bool {
        (self.value.read_bits() & Self::KILL_MASK) != 0
    }

    /// Set KILL bit atomically (RW)
    #[inline(always)]
    pub fn set_kill(&self) {
        self.set_bits(Self::KILL_MASK);
    }

    /// Clear KILL bit atomically (RW)
    #[inline(always)]
    pub fn clr_kill(&self) {
        self.clr_bits(Self::KILL_MASK);
    }

    /// **AUXSRC** `[7:5]` (RW)
    ///
    /// Selects the auxiliary clock source, will glitch when switching
    #[inline(always)]
    pub fn auxsrc(&self) -> u32 {
        (self.value.read_bits() & Self::AUXSRC_MASK) >> Self::AUXSRC_OFFSET
    }

    /// Write **AUXSRC** `[7:5]` (RW)
    ///
    /// Selects the auxiliary clock source, will glitch when switching
    #[inline(always)]
    pub fn write_auxsrc(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::AUXSRC_MASK) | ((value << Self::AUXSRC_OFFSET) & Self::AUXSRC_MASK));
    }
}

/// CLK_USB_CTRL value type for batch read/modify/write operations
///
/// Use `clkusbctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `clkusbctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ClkUsbCtrlVal {
    bits: u32,
}

impl ClkUsbCtrlVal {
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

    /// Read **ENABLED** `[28:28]`
    ///
    /// clock generator is enabled
    #[inline(always)]
    pub const fn enabled(&self) -> bool {
        (self.bits & ClkUsbCtrl::ENABLED_MASK) != 0
    }

    /// Read **NUDGE** `[20:20]`
    ///
    /// An edge on this signal shifts the phase of the output by 1 cycle of the input clock
    /// This can be done at any time
    #[inline(always)]
    pub const fn nudge(&self) -> bool {
        (self.bits & ClkUsbCtrl::NUDGE_MASK) != 0
    }

    /// Read **PHASE** `[17:16]`
    ///
    /// This delays the enable signal by up to 3 cycles of the input clock
    /// This must be set before the clock is enabled to have any effect
    #[inline(always)]
    pub const fn phase(&self) -> u32 {
        (self.bits & ClkUsbCtrl::PHASE_MASK) >> ClkUsbCtrl::PHASE_OFFSET
    }

    /// Read **ENABLE** `[11:11]`
    ///
    /// Starts and stops the clock generator cleanly
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        (self.bits & ClkUsbCtrl::ENABLE_MASK) != 0
    }

    /// Read **KILL** `[10:10]`
    ///
    /// Asynchronously kills the clock generator, enable must be set low before deasserting kill
    #[inline(always)]
    pub const fn kill(&self) -> bool {
        (self.bits & ClkUsbCtrl::KILL_MASK) != 0
    }

    /// Read **AUXSRC** `[7:5]`
    ///
    /// Selects the auxiliary clock source, will glitch when switching
    #[inline(always)]
    pub const fn auxsrc(&self) -> u32 {
        (self.bits & ClkUsbCtrl::AUXSRC_MASK) >> ClkUsbCtrl::AUXSRC_OFFSET
    }

    /// Set **NUDGE** `[20:20]`
    ///
    /// An edge on this signal shifts the phase of the output by 1 cycle of the input clock
    /// This can be done at any time
    #[inline(always)]
    pub const fn with_nudge(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClkUsbCtrl::NUDGE_MASK
            } else {
                self.bits & !ClkUsbCtrl::NUDGE_MASK
            }
        }
    }

    /// Set **PHASE** `[17:16]`
    ///
    /// This delays the enable signal by up to 3 cycles of the input clock
    /// This must be set before the clock is enabled to have any effect
    #[inline(always)]
    pub const fn with_phase(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ClkUsbCtrl::PHASE_MASK) | ((value << ClkUsbCtrl::PHASE_OFFSET) & ClkUsbCtrl::PHASE_MASK)
        }
    }

    /// Set **ENABLE** `[11:11]`
    ///
    /// Starts and stops the clock generator cleanly
    #[inline(always)]
    pub const fn with_enable(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClkUsbCtrl::ENABLE_MASK
            } else {
                self.bits & !ClkUsbCtrl::ENABLE_MASK
            }
        }
    }

    /// Set **KILL** `[10:10]`
    ///
    /// Asynchronously kills the clock generator, enable must be set low before deasserting kill
    #[inline(always)]
    pub const fn with_kill(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClkUsbCtrl::KILL_MASK
            } else {
                self.bits & !ClkUsbCtrl::KILL_MASK
            }
        }
    }

    /// Set **AUXSRC** `[7:5]`
    ///
    /// Selects the auxiliary clock source, will glitch when switching
    #[inline(always)]
    pub const fn with_auxsrc(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ClkUsbCtrl::AUXSRC_MASK) | ((value << ClkUsbCtrl::AUXSRC_OFFSET) & ClkUsbCtrl::AUXSRC_MASK)
        }
    }
}

impl Default for ClkUsbCtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl ClkUsbCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ClkUsbCtrlVal {
        ClkUsbCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ClkUsbCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ClkUsbCtrlVal) -> ClkUsbCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CLK_USB_DIV Register
///
/// Address offset: 0x0064
/// Reset value: 0x00010000
///
/// ## Fields
///
/// - **INT** `[19:16]` (RW)
///
///   Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly
#[repr(transparent)]
pub struct ClkUsbDiv {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for ClkUsbDiv {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ClkUsbDiv {
    // ======== Field Constants ========

    /// **INT**
    ///
    /// Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly
    ///
    /// Bits `[19:16]`, mask `0x000f0000`
    pub const INT_OFFSET: u32 = 16;
    pub const INT_MASK: u32 = 0x000f0000;

    // ======== Field Accessors ========

    /// **INT** `[19:16]` (RW)
    ///
    /// Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly
    #[inline(always)]
    pub fn int(&self) -> u32 {
        (self.value.read_bits() & Self::INT_MASK) >> Self::INT_OFFSET
    }

    /// Write **INT** `[19:16]` (RW)
    ///
    /// Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly
    #[inline(always)]
    pub fn write_int(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::INT_MASK) | ((value << Self::INT_OFFSET) & Self::INT_MASK));
    }
}

/// CLK_USB_DIV value type for batch read/modify/write operations
///
/// Use `clkusbdiv.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `clkusbdiv.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ClkUsbDivVal {
    bits: u32,
}

impl ClkUsbDivVal {
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

    /// Read **INT** `[19:16]`
    ///
    /// Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly
    #[inline(always)]
    pub const fn int(&self) -> u32 {
        (self.bits & ClkUsbDiv::INT_MASK) >> ClkUsbDiv::INT_OFFSET
    }

    /// Set **INT** `[19:16]`
    ///
    /// Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly
    #[inline(always)]
    pub const fn with_int(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ClkUsbDiv::INT_MASK) | ((value << ClkUsbDiv::INT_OFFSET) & ClkUsbDiv::INT_MASK)
        }
    }
}

impl Default for ClkUsbDivVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl ClkUsbDiv {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ClkUsbDivVal {
        ClkUsbDivVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ClkUsbDivVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ClkUsbDivVal) -> ClkUsbDivVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CLK_USB_SELECTED Register
///
/// Indicates which src is currently selected (one-hot)
///
/// Address offset: 0x0068
/// Reset value: 0x00000001
///
/// ## Fields
///
/// - **CLK_USB_SELECTED** `[0]` (RO)
///
///   This slice does not have a glitchless mux (only the AUX_SRC field is present, not SRC) so this register is hardwired to 0x1.
#[repr(transparent)]
pub struct ClkUsbSelected {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for ClkUsbSelected {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ClkUsbSelected {
    // ======== Field Constants ========

    /// **CLK_USB_SELECTED**
    ///
    /// This slice does not have a glitchless mux (only the AUX_SRC field is present, not SRC) so this register is hardwired to 0x1.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CLK_USB_SELECTED_OFFSET: u32 = 0;
    pub const CLK_USB_SELECTED_MASK: u32 = 0x00000001;
    pub const CLK_USB_SELECTED: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CLK_USB_SELECTED** `[0]` (RO)
    ///
    /// This slice does not have a glitchless mux (only the AUX_SRC field is present, not SRC) so this register is hardwired to 0x1.
    #[inline(always)]
    pub fn clk_usb_selected(&self) -> bool {
        (self.value.read_bits() & Self::CLK_USB_SELECTED_MASK) != 0
    }
}

/// CLK_USB_SELECTED value type for batch read/modify/write operations
///
/// Use `clkusbselected.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `clkusbselected.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ClkUsbSelectedVal {
    bits: u32,
}

impl ClkUsbSelectedVal {
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
        Self { bits: 0x00000001 }
    }

    /// Read **CLK_USB_SELECTED** `[0:0]`
    ///
    /// This slice does not have a glitchless mux (only the AUX_SRC field is present, not SRC) so this register is hardwired to 0x1.
    #[inline(always)]
    pub const fn clk_usb_selected(&self) -> bool {
        (self.bits & ClkUsbSelected::CLK_USB_SELECTED_MASK) != 0
    }
}

impl Default for ClkUsbSelectedVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl ClkUsbSelected {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ClkUsbSelectedVal {
        ClkUsbSelectedVal::from_bits(self.value.read_bits())
    }
}

/// CLK_ADC_CTRL Register
///
/// Clock control, can be changed on-the-fly (except for auxsrc)
///
/// Address offset: 0x006c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **ENABLED** `[28]` (RO)
///
///   clock generator is enabled
///
/// - **NUDGE** `[20]` (RW)
///
///   An edge on this signal shifts the phase of the output by 1 cycle of the input clock
///   This can be done at any time
///
/// - **PHASE** `[17:16]` (RW)
///
///   This delays the enable signal by up to 3 cycles of the input clock
///   This must be set before the clock is enabled to have any effect
///
/// - **ENABLE** `[11]` (RW)
///
///   Starts and stops the clock generator cleanly
///
/// - **KILL** `[10]` (RW)
///
///   Asynchronously kills the clock generator, enable must be set low before deasserting kill
///
/// - **AUXSRC** `[7:5]` (RW)
///
///   Selects the auxiliary clock source, will glitch when switching
#[repr(transparent)]
pub struct ClkAdcCtrl {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for ClkAdcCtrl {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ClkAdcCtrl {
    // ======== Field Constants ========

    /// **ENABLED**
    ///
    /// clock generator is enabled
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const ENABLED_OFFSET: u32 = 28;
    pub const ENABLED_MASK: u32 = 0x10000000;
    pub const ENABLED: u32 = 1 << 28;

    /// **NUDGE**
    ///
    /// An edge on this signal shifts the phase of the output by 1 cycle of the input clock
    /// This can be done at any time
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const NUDGE_OFFSET: u32 = 20;
    pub const NUDGE_MASK: u32 = 0x00100000;
    pub const NUDGE: u32 = 1 << 20;

    /// **PHASE**
    ///
    /// This delays the enable signal by up to 3 cycles of the input clock
    /// This must be set before the clock is enabled to have any effect
    ///
    /// Bits `[17:16]`, mask `0x00030000`
    pub const PHASE_OFFSET: u32 = 16;
    pub const PHASE_MASK: u32 = 0x00030000;

    /// **ENABLE**
    ///
    /// Starts and stops the clock generator cleanly
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const ENABLE_OFFSET: u32 = 11;
    pub const ENABLE_MASK: u32 = 0x00000800;
    pub const ENABLE: u32 = 1 << 11;

    /// **KILL**
    ///
    /// Asynchronously kills the clock generator, enable must be set low before deasserting kill
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const KILL_OFFSET: u32 = 10;
    pub const KILL_MASK: u32 = 0x00000400;
    pub const KILL: u32 = 1 << 10;

    /// **AUXSRC**
    ///
    /// Selects the auxiliary clock source, will glitch when switching
    ///
    /// Bits `[7:5]`, mask `0x000000e0`
    pub const AUXSRC_OFFSET: u32 = 5;
    pub const AUXSRC_MASK: u32 = 0x000000e0;
    pub const AUXSRC_CLKSRC_PLL_USB: u32 = 0 << 5;
    pub const AUXSRC_CLKSRC_PLL_USB_VAL: u32 = 0;
    pub const AUXSRC_CLKSRC_PLL_SYS: u32 = 1 << 5;
    pub const AUXSRC_CLKSRC_PLL_SYS_VAL: u32 = 1;
    pub const AUXSRC_ROSC_CLKSRC_PH: u32 = 2 << 5;
    pub const AUXSRC_ROSC_CLKSRC_PH_VAL: u32 = 2;
    pub const AUXSRC_XOSC_CLKSRC: u32 = 3 << 5;
    pub const AUXSRC_XOSC_CLKSRC_VAL: u32 = 3;
    pub const AUXSRC_CLKSRC_GPIN0: u32 = 4 << 5;
    pub const AUXSRC_CLKSRC_GPIN0_VAL: u32 = 4;
    pub const AUXSRC_CLKSRC_GPIN1: u32 = 5 << 5;
    pub const AUXSRC_CLKSRC_GPIN1_VAL: u32 = 5;

    // ======== Field Accessors ========

    /// **ENABLED** `[28]` (RO)
    ///
    /// clock generator is enabled
    #[inline(always)]
    pub fn enabled(&self) -> bool {
        (self.value.read_bits() & Self::ENABLED_MASK) != 0
    }

    /// **NUDGE** `[20]` (RW)
    ///
    /// An edge on this signal shifts the phase of the output by 1 cycle of the input clock
    /// This can be done at any time
    #[inline(always)]
    pub fn nudge(&self) -> bool {
        (self.value.read_bits() & Self::NUDGE_MASK) != 0
    }

    /// Set NUDGE bit atomically (RW)
    #[inline(always)]
    pub fn set_nudge(&self) {
        self.set_bits(Self::NUDGE_MASK);
    }

    /// Clear NUDGE bit atomically (RW)
    #[inline(always)]
    pub fn clr_nudge(&self) {
        self.clr_bits(Self::NUDGE_MASK);
    }

    /// **PHASE** `[17:16]` (RW)
    ///
    /// This delays the enable signal by up to 3 cycles of the input clock
    /// This must be set before the clock is enabled to have any effect
    #[inline(always)]
    pub fn phase(&self) -> u32 {
        (self.value.read_bits() & Self::PHASE_MASK) >> Self::PHASE_OFFSET
    }

    /// Write **PHASE** `[17:16]` (RW)
    ///
    /// This delays the enable signal by up to 3 cycles of the input clock
    /// This must be set before the clock is enabled to have any effect
    #[inline(always)]
    pub fn write_phase(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::PHASE_MASK) | ((value << Self::PHASE_OFFSET) & Self::PHASE_MASK));
    }

    /// **ENABLE** `[11]` (RW)
    ///
    /// Starts and stops the clock generator cleanly
    #[inline(always)]
    pub fn enable(&self) -> bool {
        (self.value.read_bits() & Self::ENABLE_MASK) != 0
    }

    /// Set ENABLE bit atomically (RW)
    #[inline(always)]
    pub fn set_enable(&self) {
        self.set_bits(Self::ENABLE_MASK);
    }

    /// Clear ENABLE bit atomically (RW)
    #[inline(always)]
    pub fn clr_enable(&self) {
        self.clr_bits(Self::ENABLE_MASK);
    }

    /// **KILL** `[10]` (RW)
    ///
    /// Asynchronously kills the clock generator, enable must be set low before deasserting kill
    #[inline(always)]
    pub fn kill(&self) -> bool {
        (self.value.read_bits() & Self::KILL_MASK) != 0
    }

    /// Set KILL bit atomically (RW)
    #[inline(always)]
    pub fn set_kill(&self) {
        self.set_bits(Self::KILL_MASK);
    }

    /// Clear KILL bit atomically (RW)
    #[inline(always)]
    pub fn clr_kill(&self) {
        self.clr_bits(Self::KILL_MASK);
    }

    /// **AUXSRC** `[7:5]` (RW)
    ///
    /// Selects the auxiliary clock source, will glitch when switching
    #[inline(always)]
    pub fn auxsrc(&self) -> u32 {
        (self.value.read_bits() & Self::AUXSRC_MASK) >> Self::AUXSRC_OFFSET
    }

    /// Write **AUXSRC** `[7:5]` (RW)
    ///
    /// Selects the auxiliary clock source, will glitch when switching
    #[inline(always)]
    pub fn write_auxsrc(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::AUXSRC_MASK) | ((value << Self::AUXSRC_OFFSET) & Self::AUXSRC_MASK));
    }
}

/// CLK_ADC_CTRL value type for batch read/modify/write operations
///
/// Use `clkadcctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `clkadcctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ClkAdcCtrlVal {
    bits: u32,
}

impl ClkAdcCtrlVal {
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

    /// Read **ENABLED** `[28:28]`
    ///
    /// clock generator is enabled
    #[inline(always)]
    pub const fn enabled(&self) -> bool {
        (self.bits & ClkAdcCtrl::ENABLED_MASK) != 0
    }

    /// Read **NUDGE** `[20:20]`
    ///
    /// An edge on this signal shifts the phase of the output by 1 cycle of the input clock
    /// This can be done at any time
    #[inline(always)]
    pub const fn nudge(&self) -> bool {
        (self.bits & ClkAdcCtrl::NUDGE_MASK) != 0
    }

    /// Read **PHASE** `[17:16]`
    ///
    /// This delays the enable signal by up to 3 cycles of the input clock
    /// This must be set before the clock is enabled to have any effect
    #[inline(always)]
    pub const fn phase(&self) -> u32 {
        (self.bits & ClkAdcCtrl::PHASE_MASK) >> ClkAdcCtrl::PHASE_OFFSET
    }

    /// Read **ENABLE** `[11:11]`
    ///
    /// Starts and stops the clock generator cleanly
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        (self.bits & ClkAdcCtrl::ENABLE_MASK) != 0
    }

    /// Read **KILL** `[10:10]`
    ///
    /// Asynchronously kills the clock generator, enable must be set low before deasserting kill
    #[inline(always)]
    pub const fn kill(&self) -> bool {
        (self.bits & ClkAdcCtrl::KILL_MASK) != 0
    }

    /// Read **AUXSRC** `[7:5]`
    ///
    /// Selects the auxiliary clock source, will glitch when switching
    #[inline(always)]
    pub const fn auxsrc(&self) -> u32 {
        (self.bits & ClkAdcCtrl::AUXSRC_MASK) >> ClkAdcCtrl::AUXSRC_OFFSET
    }

    /// Set **NUDGE** `[20:20]`
    ///
    /// An edge on this signal shifts the phase of the output by 1 cycle of the input clock
    /// This can be done at any time
    #[inline(always)]
    pub const fn with_nudge(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClkAdcCtrl::NUDGE_MASK
            } else {
                self.bits & !ClkAdcCtrl::NUDGE_MASK
            }
        }
    }

    /// Set **PHASE** `[17:16]`
    ///
    /// This delays the enable signal by up to 3 cycles of the input clock
    /// This must be set before the clock is enabled to have any effect
    #[inline(always)]
    pub const fn with_phase(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ClkAdcCtrl::PHASE_MASK) | ((value << ClkAdcCtrl::PHASE_OFFSET) & ClkAdcCtrl::PHASE_MASK)
        }
    }

    /// Set **ENABLE** `[11:11]`
    ///
    /// Starts and stops the clock generator cleanly
    #[inline(always)]
    pub const fn with_enable(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClkAdcCtrl::ENABLE_MASK
            } else {
                self.bits & !ClkAdcCtrl::ENABLE_MASK
            }
        }
    }

    /// Set **KILL** `[10:10]`
    ///
    /// Asynchronously kills the clock generator, enable must be set low before deasserting kill
    #[inline(always)]
    pub const fn with_kill(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClkAdcCtrl::KILL_MASK
            } else {
                self.bits & !ClkAdcCtrl::KILL_MASK
            }
        }
    }

    /// Set **AUXSRC** `[7:5]`
    ///
    /// Selects the auxiliary clock source, will glitch when switching
    #[inline(always)]
    pub const fn with_auxsrc(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ClkAdcCtrl::AUXSRC_MASK) | ((value << ClkAdcCtrl::AUXSRC_OFFSET) & ClkAdcCtrl::AUXSRC_MASK)
        }
    }
}

impl Default for ClkAdcCtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl ClkAdcCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ClkAdcCtrlVal {
        ClkAdcCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ClkAdcCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ClkAdcCtrlVal) -> ClkAdcCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CLK_ADC_DIV Register
///
/// Address offset: 0x0070
/// Reset value: 0x00010000
///
/// ## Fields
///
/// - **INT** `[19:16]` (RW)
///
///   Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly
#[repr(transparent)]
pub struct ClkAdcDiv {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for ClkAdcDiv {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ClkAdcDiv {
    // ======== Field Constants ========

    /// **INT**
    ///
    /// Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly
    ///
    /// Bits `[19:16]`, mask `0x000f0000`
    pub const INT_OFFSET: u32 = 16;
    pub const INT_MASK: u32 = 0x000f0000;

    // ======== Field Accessors ========

    /// **INT** `[19:16]` (RW)
    ///
    /// Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly
    #[inline(always)]
    pub fn int(&self) -> u32 {
        (self.value.read_bits() & Self::INT_MASK) >> Self::INT_OFFSET
    }

    /// Write **INT** `[19:16]` (RW)
    ///
    /// Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly
    #[inline(always)]
    pub fn write_int(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::INT_MASK) | ((value << Self::INT_OFFSET) & Self::INT_MASK));
    }
}

/// CLK_ADC_DIV value type for batch read/modify/write operations
///
/// Use `clkadcdiv.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `clkadcdiv.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ClkAdcDivVal {
    bits: u32,
}

impl ClkAdcDivVal {
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

    /// Read **INT** `[19:16]`
    ///
    /// Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly
    #[inline(always)]
    pub const fn int(&self) -> u32 {
        (self.bits & ClkAdcDiv::INT_MASK) >> ClkAdcDiv::INT_OFFSET
    }

    /// Set **INT** `[19:16]`
    ///
    /// Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly
    #[inline(always)]
    pub const fn with_int(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ClkAdcDiv::INT_MASK) | ((value << ClkAdcDiv::INT_OFFSET) & ClkAdcDiv::INT_MASK)
        }
    }
}

impl Default for ClkAdcDivVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl ClkAdcDiv {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ClkAdcDivVal {
        ClkAdcDivVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ClkAdcDivVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ClkAdcDivVal) -> ClkAdcDivVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CLK_ADC_SELECTED Register
///
/// Indicates which src is currently selected (one-hot)
///
/// Address offset: 0x0074
/// Reset value: 0x00000001
///
/// ## Fields
///
/// - **CLK_ADC_SELECTED** `[0]` (RO)
///
///   This slice does not have a glitchless mux (only the AUX_SRC field is present, not SRC) so this register is hardwired to 0x1.
#[repr(transparent)]
pub struct ClkAdcSelected {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for ClkAdcSelected {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ClkAdcSelected {
    // ======== Field Constants ========

    /// **CLK_ADC_SELECTED**
    ///
    /// This slice does not have a glitchless mux (only the AUX_SRC field is present, not SRC) so this register is hardwired to 0x1.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CLK_ADC_SELECTED_OFFSET: u32 = 0;
    pub const CLK_ADC_SELECTED_MASK: u32 = 0x00000001;
    pub const CLK_ADC_SELECTED: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CLK_ADC_SELECTED** `[0]` (RO)
    ///
    /// This slice does not have a glitchless mux (only the AUX_SRC field is present, not SRC) so this register is hardwired to 0x1.
    #[inline(always)]
    pub fn clk_adc_selected(&self) -> bool {
        (self.value.read_bits() & Self::CLK_ADC_SELECTED_MASK) != 0
    }
}

/// CLK_ADC_SELECTED value type for batch read/modify/write operations
///
/// Use `clkadcselected.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `clkadcselected.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ClkAdcSelectedVal {
    bits: u32,
}

impl ClkAdcSelectedVal {
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
        Self { bits: 0x00000001 }
    }

    /// Read **CLK_ADC_SELECTED** `[0:0]`
    ///
    /// This slice does not have a glitchless mux (only the AUX_SRC field is present, not SRC) so this register is hardwired to 0x1.
    #[inline(always)]
    pub const fn clk_adc_selected(&self) -> bool {
        (self.bits & ClkAdcSelected::CLK_ADC_SELECTED_MASK) != 0
    }
}

impl Default for ClkAdcSelectedVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl ClkAdcSelected {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ClkAdcSelectedVal {
        ClkAdcSelectedVal::from_bits(self.value.read_bits())
    }
}

/// DFTCLK_XOSC_CTRL Register
///
/// Address offset: 0x0078
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SRC** `[1:0]` (RW)
#[repr(transparent)]
pub struct DftclkXoscCtrl {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for DftclkXoscCtrl {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DftclkXoscCtrl {
    // ======== Field Constants ========

    /// **SRC**
    /// Bits `[1:0]`, mask `0x00000003`
    pub const SRC_OFFSET: u32 = 0;
    pub const SRC_MASK: u32 = 0x00000003;
    pub const SRC_NULL: u32 = 0 << 0;
    pub const SRC_NULL_VAL: u32 = 0;
    pub const SRC_CLKSRC_PLL_USB_PRIMARY: u32 = 1 << 0;
    pub const SRC_CLKSRC_PLL_USB_PRIMARY_VAL: u32 = 1;
    pub const SRC_CLKSRC_GPIN0: u32 = 2 << 0;
    pub const SRC_CLKSRC_GPIN0_VAL: u32 = 2;

    // ======== Field Accessors ========

    /// **SRC** `[1:0]` (RW)
    #[inline(always)]
    pub fn src(&self) -> u32 {
        (self.value.read_bits() & Self::SRC_MASK) >> Self::SRC_OFFSET
    }

    /// Write **SRC** `[1:0]` (RW)
    #[inline(always)]
    pub fn write_src(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SRC_MASK) | ((value << Self::SRC_OFFSET) & Self::SRC_MASK));
    }
}

/// DFTCLK_XOSC_CTRL value type for batch read/modify/write operations
///
/// Use `dftclkxoscctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dftclkxoscctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DftclkXoscCtrlVal {
    bits: u32,
}

impl DftclkXoscCtrlVal {
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

    /// Read **SRC** `[1:0]`
    #[inline(always)]
    pub const fn src(&self) -> u32 {
        (self.bits & DftclkXoscCtrl::SRC_MASK) >> DftclkXoscCtrl::SRC_OFFSET
    }

    /// Set **SRC** `[1:0]`
    #[inline(always)]
    pub const fn with_src(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DftclkXoscCtrl::SRC_MASK) | ((value << DftclkXoscCtrl::SRC_OFFSET) & DftclkXoscCtrl::SRC_MASK)
        }
    }
}

impl Default for DftclkXoscCtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl DftclkXoscCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DftclkXoscCtrlVal {
        DftclkXoscCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DftclkXoscCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DftclkXoscCtrlVal) -> DftclkXoscCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DFTCLK_ROSC_CTRL Register
///
/// Address offset: 0x007c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SRC** `[1:0]` (RW)
#[repr(transparent)]
pub struct DftclkRoscCtrl {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for DftclkRoscCtrl {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DftclkRoscCtrl {
    // ======== Field Constants ========

    /// **SRC**
    /// Bits `[1:0]`, mask `0x00000003`
    pub const SRC_OFFSET: u32 = 0;
    pub const SRC_MASK: u32 = 0x00000003;
    pub const SRC_NULL: u32 = 0 << 0;
    pub const SRC_NULL_VAL: u32 = 0;
    pub const SRC_CLKSRC_PLL_SYS_PRIMARY_ROSC: u32 = 1 << 0;
    pub const SRC_CLKSRC_PLL_SYS_PRIMARY_ROSC_VAL: u32 = 1;
    pub const SRC_CLKSRC_GPIN1: u32 = 2 << 0;
    pub const SRC_CLKSRC_GPIN1_VAL: u32 = 2;

    // ======== Field Accessors ========

    /// **SRC** `[1:0]` (RW)
    #[inline(always)]
    pub fn src(&self) -> u32 {
        (self.value.read_bits() & Self::SRC_MASK) >> Self::SRC_OFFSET
    }

    /// Write **SRC** `[1:0]` (RW)
    #[inline(always)]
    pub fn write_src(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SRC_MASK) | ((value << Self::SRC_OFFSET) & Self::SRC_MASK));
    }
}

/// DFTCLK_ROSC_CTRL value type for batch read/modify/write operations
///
/// Use `dftclkroscctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dftclkroscctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DftclkRoscCtrlVal {
    bits: u32,
}

impl DftclkRoscCtrlVal {
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

    /// Read **SRC** `[1:0]`
    #[inline(always)]
    pub const fn src(&self) -> u32 {
        (self.bits & DftclkRoscCtrl::SRC_MASK) >> DftclkRoscCtrl::SRC_OFFSET
    }

    /// Set **SRC** `[1:0]`
    #[inline(always)]
    pub const fn with_src(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DftclkRoscCtrl::SRC_MASK) | ((value << DftclkRoscCtrl::SRC_OFFSET) & DftclkRoscCtrl::SRC_MASK)
        }
    }
}

impl Default for DftclkRoscCtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl DftclkRoscCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DftclkRoscCtrlVal {
        DftclkRoscCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DftclkRoscCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DftclkRoscCtrlVal) -> DftclkRoscCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DFTCLK_LPOSC_CTRL Register
///
/// Address offset: 0x0080
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SRC** `[1:0]` (RW)
#[repr(transparent)]
pub struct DftclkLposcCtrl {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for DftclkLposcCtrl {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DftclkLposcCtrl {
    // ======== Field Constants ========

    /// **SRC**
    /// Bits `[1:0]`, mask `0x00000003`
    pub const SRC_OFFSET: u32 = 0;
    pub const SRC_MASK: u32 = 0x00000003;
    pub const SRC_NULL: u32 = 0 << 0;
    pub const SRC_NULL_VAL: u32 = 0;
    pub const SRC_CLKSRC_PLL_USB_PRIMARY_LPOSC: u32 = 1 << 0;
    pub const SRC_CLKSRC_PLL_USB_PRIMARY_LPOSC_VAL: u32 = 1;
    pub const SRC_CLKSRC_GPIN1: u32 = 2 << 0;
    pub const SRC_CLKSRC_GPIN1_VAL: u32 = 2;

    // ======== Field Accessors ========

    /// **SRC** `[1:0]` (RW)
    #[inline(always)]
    pub fn src(&self) -> u32 {
        (self.value.read_bits() & Self::SRC_MASK) >> Self::SRC_OFFSET
    }

    /// Write **SRC** `[1:0]` (RW)
    #[inline(always)]
    pub fn write_src(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SRC_MASK) | ((value << Self::SRC_OFFSET) & Self::SRC_MASK));
    }
}

/// DFTCLK_LPOSC_CTRL value type for batch read/modify/write operations
///
/// Use `dftclklposcctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dftclklposcctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DftclkLposcCtrlVal {
    bits: u32,
}

impl DftclkLposcCtrlVal {
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

    /// Read **SRC** `[1:0]`
    #[inline(always)]
    pub const fn src(&self) -> u32 {
        (self.bits & DftclkLposcCtrl::SRC_MASK) >> DftclkLposcCtrl::SRC_OFFSET
    }

    /// Set **SRC** `[1:0]`
    #[inline(always)]
    pub const fn with_src(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DftclkLposcCtrl::SRC_MASK) | ((value << DftclkLposcCtrl::SRC_OFFSET) & DftclkLposcCtrl::SRC_MASK)
        }
    }
}

impl Default for DftclkLposcCtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl DftclkLposcCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DftclkLposcCtrlVal {
        DftclkLposcCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DftclkLposcCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DftclkLposcCtrlVal) -> DftclkLposcCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CLK_SYS_RESUS_CTRL Register
///
/// Address offset: 0x0084
/// Reset value: 0x000000ff
///
/// ## Fields
///
/// - **CLEAR** `[16]` (RW)
///
///   For clearing the resus after the fault that triggered it has been corrected
///
/// - **FRCE** `[12]` (RW)
///
///   Force a resus, for test purposes only
///
/// - **ENABLE** `[8]` (RW)
///
///   Enable resus
///
/// - **TIMEOUT** `[7:0]` (RW)
///
///   This is expressed as a number of clk_ref cycles
///   and must be >= 2x clk_ref_freq/min_clk_tst_freq
#[repr(transparent)]
pub struct ClkSysResusCtrl {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for ClkSysResusCtrl {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ClkSysResusCtrl {
    // ======== Field Constants ========

    /// **CLEAR**
    ///
    /// For clearing the resus after the fault that triggered it has been corrected
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const CLEAR_OFFSET: u32 = 16;
    pub const CLEAR_MASK: u32 = 0x00010000;
    pub const CLEAR: u32 = 1 << 16;

    /// **FRCE**
    ///
    /// Force a resus, for test purposes only
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const FRCE_OFFSET: u32 = 12;
    pub const FRCE_MASK: u32 = 0x00001000;
    pub const FRCE: u32 = 1 << 12;

    /// **ENABLE**
    ///
    /// Enable resus
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ENABLE_OFFSET: u32 = 8;
    pub const ENABLE_MASK: u32 = 0x00000100;
    pub const ENABLE: u32 = 1 << 8;

    /// **TIMEOUT**
    ///
    /// This is expressed as a number of clk_ref cycles
    /// and must be >= 2x clk_ref_freq/min_clk_tst_freq
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const TIMEOUT_OFFSET: u32 = 0;
    pub const TIMEOUT_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **CLEAR** `[16]` (RW)
    ///
    /// For clearing the resus after the fault that triggered it has been corrected
    #[inline(always)]
    pub fn clear(&self) -> bool {
        (self.value.read_bits() & Self::CLEAR_MASK) != 0
    }

    /// Set CLEAR bit atomically (RW)
    #[inline(always)]
    pub fn set_clear(&self) {
        self.set_bits(Self::CLEAR_MASK);
    }

    /// Clear CLEAR bit atomically (RW)
    #[inline(always)]
    pub fn clr_clear(&self) {
        self.clr_bits(Self::CLEAR_MASK);
    }

    /// **FRCE** `[12]` (RW)
    ///
    /// Force a resus, for test purposes only
    #[inline(always)]
    pub fn frce(&self) -> bool {
        (self.value.read_bits() & Self::FRCE_MASK) != 0
    }

    /// Set FRCE bit atomically (RW)
    #[inline(always)]
    pub fn set_frce(&self) {
        self.set_bits(Self::FRCE_MASK);
    }

    /// Clear FRCE bit atomically (RW)
    #[inline(always)]
    pub fn clr_frce(&self) {
        self.clr_bits(Self::FRCE_MASK);
    }

    /// **ENABLE** `[8]` (RW)
    ///
    /// Enable resus
    #[inline(always)]
    pub fn enable(&self) -> bool {
        (self.value.read_bits() & Self::ENABLE_MASK) != 0
    }

    /// Set ENABLE bit atomically (RW)
    #[inline(always)]
    pub fn set_enable(&self) {
        self.set_bits(Self::ENABLE_MASK);
    }

    /// Clear ENABLE bit atomically (RW)
    #[inline(always)]
    pub fn clr_enable(&self) {
        self.clr_bits(Self::ENABLE_MASK);
    }

    /// **TIMEOUT** `[7:0]` (RW)
    ///
    /// This is expressed as a number of clk_ref cycles
    /// and must be >= 2x clk_ref_freq/min_clk_tst_freq
    #[inline(always)]
    pub fn timeout(&self) -> u32 {
        (self.value.read_bits() & Self::TIMEOUT_MASK) >> Self::TIMEOUT_OFFSET
    }

    /// Write **TIMEOUT** `[7:0]` (RW)
    ///
    /// This is expressed as a number of clk_ref cycles
    /// and must be >= 2x clk_ref_freq/min_clk_tst_freq
    #[inline(always)]
    pub fn write_timeout(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::TIMEOUT_MASK) | ((value << Self::TIMEOUT_OFFSET) & Self::TIMEOUT_MASK));
    }
}

/// CLK_SYS_RESUS_CTRL value type for batch read/modify/write operations
///
/// Use `clksysresusctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `clksysresusctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ClkSysResusCtrlVal {
    bits: u32,
}

impl ClkSysResusCtrlVal {
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
        Self { bits: 0x000000ff }
    }

    /// Read **CLEAR** `[16:16]`
    ///
    /// For clearing the resus after the fault that triggered it has been corrected
    #[inline(always)]
    pub const fn clear(&self) -> bool {
        (self.bits & ClkSysResusCtrl::CLEAR_MASK) != 0
    }

    /// Read **FRCE** `[12:12]`
    ///
    /// Force a resus, for test purposes only
    #[inline(always)]
    pub const fn frce(&self) -> bool {
        (self.bits & ClkSysResusCtrl::FRCE_MASK) != 0
    }

    /// Read **ENABLE** `[8:8]`
    ///
    /// Enable resus
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        (self.bits & ClkSysResusCtrl::ENABLE_MASK) != 0
    }

    /// Read **TIMEOUT** `[7:0]`
    ///
    /// This is expressed as a number of clk_ref cycles
    /// and must be >= 2x clk_ref_freq/min_clk_tst_freq
    #[inline(always)]
    pub const fn timeout(&self) -> u32 {
        (self.bits & ClkSysResusCtrl::TIMEOUT_MASK) >> ClkSysResusCtrl::TIMEOUT_OFFSET
    }

    /// Set **CLEAR** `[16:16]`
    ///
    /// For clearing the resus after the fault that triggered it has been corrected
    #[inline(always)]
    pub const fn with_clear(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClkSysResusCtrl::CLEAR_MASK
            } else {
                self.bits & !ClkSysResusCtrl::CLEAR_MASK
            }
        }
    }

    /// Set **FRCE** `[12:12]`
    ///
    /// Force a resus, for test purposes only
    #[inline(always)]
    pub const fn with_frce(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClkSysResusCtrl::FRCE_MASK
            } else {
                self.bits & !ClkSysResusCtrl::FRCE_MASK
            }
        }
    }

    /// Set **ENABLE** `[8:8]`
    ///
    /// Enable resus
    #[inline(always)]
    pub const fn with_enable(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClkSysResusCtrl::ENABLE_MASK
            } else {
                self.bits & !ClkSysResusCtrl::ENABLE_MASK
            }
        }
    }

    /// Set **TIMEOUT** `[7:0]`
    ///
    /// This is expressed as a number of clk_ref cycles
    /// and must be >= 2x clk_ref_freq/min_clk_tst_freq
    #[inline(always)]
    pub const fn with_timeout(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ClkSysResusCtrl::TIMEOUT_MASK) | ((value << ClkSysResusCtrl::TIMEOUT_OFFSET) & ClkSysResusCtrl::TIMEOUT_MASK)
        }
    }
}

impl Default for ClkSysResusCtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl ClkSysResusCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ClkSysResusCtrlVal {
        ClkSysResusCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ClkSysResusCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ClkSysResusCtrlVal) -> ClkSysResusCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CLK_SYS_RESUS_STATUS Register
///
/// Address offset: 0x0088
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **RESUSSED** `[0]` (RO)
///
///   Clock has been resuscitated, correct the error then send ctrl_clear=1
#[repr(transparent)]
pub struct ClkSysResusStatus {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for ClkSysResusStatus {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ClkSysResusStatus {
    // ======== Field Constants ========

    /// **RESUSSED**
    ///
    /// Clock has been resuscitated, correct the error then send ctrl_clear=1
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const RESUSSED_OFFSET: u32 = 0;
    pub const RESUSSED_MASK: u32 = 0x00000001;
    pub const RESUSSED: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **RESUSSED** `[0]` (RO)
    ///
    /// Clock has been resuscitated, correct the error then send ctrl_clear=1
    #[inline(always)]
    pub fn resussed(&self) -> bool {
        (self.value.read_bits() & Self::RESUSSED_MASK) != 0
    }
}

/// CLK_SYS_RESUS_STATUS value type for batch read/modify/write operations
///
/// Use `clksysresusstatus.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `clksysresusstatus.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ClkSysResusStatusVal {
    bits: u32,
}

impl ClkSysResusStatusVal {
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

    /// Read **RESUSSED** `[0:0]`
    ///
    /// Clock has been resuscitated, correct the error then send ctrl_clear=1
    #[inline(always)]
    pub const fn resussed(&self) -> bool {
        (self.bits & ClkSysResusStatus::RESUSSED_MASK) != 0
    }
}

impl Default for ClkSysResusStatusVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl ClkSysResusStatus {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ClkSysResusStatusVal {
        ClkSysResusStatusVal::from_bits(self.value.read_bits())
    }
}

/// FC0_REF_KHZ Register
///
/// Reference clock frequency in kHz
///
/// Address offset: 0x008c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **FC0_REF_KHZ** `[19:0]` (RW)
#[repr(transparent)]
pub struct Fc0RefKhz {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Fc0RefKhz {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Fc0RefKhz {
    // ======== Field Constants ========

    /// **FC0_REF_KHZ**
    /// Bits `[19:0]`, mask `0x000fffff`
    pub const FC0_REF_KHZ_OFFSET: u32 = 0;
    pub const FC0_REF_KHZ_MASK: u32 = 0x000fffff;

    // ======== Field Accessors ========

    /// **FC0_REF_KHZ** `[19:0]` (RW)
    #[inline(always)]
    pub fn fc0_ref_khz(&self) -> u32 {
        (self.value.read_bits() & Self::FC0_REF_KHZ_MASK) >> Self::FC0_REF_KHZ_OFFSET
    }

    /// Write **FC0_REF_KHZ** `[19:0]` (RW)
    #[inline(always)]
    pub fn write_fc0_ref_khz(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::FC0_REF_KHZ_MASK) | ((value << Self::FC0_REF_KHZ_OFFSET) & Self::FC0_REF_KHZ_MASK));
    }
}

/// FC0_REF_KHZ value type for batch read/modify/write operations
///
/// Use `fc0refkhz.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `fc0refkhz.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Fc0RefKhzVal {
    bits: u32,
}

impl Fc0RefKhzVal {
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

    /// Read **FC0_REF_KHZ** `[19:0]`
    #[inline(always)]
    pub const fn fc0_ref_khz(&self) -> u32 {
        (self.bits & Fc0RefKhz::FC0_REF_KHZ_MASK) >> Fc0RefKhz::FC0_REF_KHZ_OFFSET
    }

    /// Set **FC0_REF_KHZ** `[19:0]`
    #[inline(always)]
    pub const fn with_fc0_ref_khz(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Fc0RefKhz::FC0_REF_KHZ_MASK) | ((value << Fc0RefKhz::FC0_REF_KHZ_OFFSET) & Fc0RefKhz::FC0_REF_KHZ_MASK)
        }
    }
}

impl Default for Fc0RefKhzVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Fc0RefKhz {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Fc0RefKhzVal {
        Fc0RefKhzVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Fc0RefKhzVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Fc0RefKhzVal) -> Fc0RefKhzVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// FC0_MIN_KHZ Register
///
/// Minimum pass frequency in kHz. This is optional. Set to 0 if you are not using the pass/fail flags
///
/// Address offset: 0x0090
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **FC0_MIN_KHZ** `[24:0]` (RW)
#[repr(transparent)]
pub struct Fc0MinKhz {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Fc0MinKhz {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Fc0MinKhz {
    // ======== Field Constants ========

    /// **FC0_MIN_KHZ**
    /// Bits `[24:0]`, mask `0x01ffffff`
    pub const FC0_MIN_KHZ_OFFSET: u32 = 0;
    pub const FC0_MIN_KHZ_MASK: u32 = 0x01ffffff;

    // ======== Field Accessors ========

    /// **FC0_MIN_KHZ** `[24:0]` (RW)
    #[inline(always)]
    pub fn fc0_min_khz(&self) -> u32 {
        (self.value.read_bits() & Self::FC0_MIN_KHZ_MASK) >> Self::FC0_MIN_KHZ_OFFSET
    }

    /// Write **FC0_MIN_KHZ** `[24:0]` (RW)
    #[inline(always)]
    pub fn write_fc0_min_khz(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::FC0_MIN_KHZ_MASK) | ((value << Self::FC0_MIN_KHZ_OFFSET) & Self::FC0_MIN_KHZ_MASK));
    }
}

/// FC0_MIN_KHZ value type for batch read/modify/write operations
///
/// Use `fc0minkhz.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `fc0minkhz.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Fc0MinKhzVal {
    bits: u32,
}

impl Fc0MinKhzVal {
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

    /// Read **FC0_MIN_KHZ** `[24:0]`
    #[inline(always)]
    pub const fn fc0_min_khz(&self) -> u32 {
        (self.bits & Fc0MinKhz::FC0_MIN_KHZ_MASK) >> Fc0MinKhz::FC0_MIN_KHZ_OFFSET
    }

    /// Set **FC0_MIN_KHZ** `[24:0]`
    #[inline(always)]
    pub const fn with_fc0_min_khz(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Fc0MinKhz::FC0_MIN_KHZ_MASK) | ((value << Fc0MinKhz::FC0_MIN_KHZ_OFFSET) & Fc0MinKhz::FC0_MIN_KHZ_MASK)
        }
    }
}

impl Default for Fc0MinKhzVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Fc0MinKhz {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Fc0MinKhzVal {
        Fc0MinKhzVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Fc0MinKhzVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Fc0MinKhzVal) -> Fc0MinKhzVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// FC0_MAX_KHZ Register
///
/// Maximum pass frequency in kHz. This is optional. Set to 0x1ffffff if you are not using the pass/fail flags
///
/// Address offset: 0x0094
/// Reset value: 0x01ffffff
///
/// ## Fields
///
/// - **FC0_MAX_KHZ** `[24:0]` (RW)
#[repr(transparent)]
pub struct Fc0MaxKhz {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Fc0MaxKhz {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Fc0MaxKhz {
    // ======== Field Constants ========

    /// **FC0_MAX_KHZ**
    /// Bits `[24:0]`, mask `0x01ffffff`
    pub const FC0_MAX_KHZ_OFFSET: u32 = 0;
    pub const FC0_MAX_KHZ_MASK: u32 = 0x01ffffff;

    // ======== Field Accessors ========

    /// **FC0_MAX_KHZ** `[24:0]` (RW)
    #[inline(always)]
    pub fn fc0_max_khz(&self) -> u32 {
        (self.value.read_bits() & Self::FC0_MAX_KHZ_MASK) >> Self::FC0_MAX_KHZ_OFFSET
    }

    /// Write **FC0_MAX_KHZ** `[24:0]` (RW)
    #[inline(always)]
    pub fn write_fc0_max_khz(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::FC0_MAX_KHZ_MASK) | ((value << Self::FC0_MAX_KHZ_OFFSET) & Self::FC0_MAX_KHZ_MASK));
    }
}

/// FC0_MAX_KHZ value type for batch read/modify/write operations
///
/// Use `fc0maxkhz.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `fc0maxkhz.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Fc0MaxKhzVal {
    bits: u32,
}

impl Fc0MaxKhzVal {
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
        Self { bits: 0x01ffffff }
    }

    /// Read **FC0_MAX_KHZ** `[24:0]`
    #[inline(always)]
    pub const fn fc0_max_khz(&self) -> u32 {
        (self.bits & Fc0MaxKhz::FC0_MAX_KHZ_MASK) >> Fc0MaxKhz::FC0_MAX_KHZ_OFFSET
    }

    /// Set **FC0_MAX_KHZ** `[24:0]`
    #[inline(always)]
    pub const fn with_fc0_max_khz(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Fc0MaxKhz::FC0_MAX_KHZ_MASK) | ((value << Fc0MaxKhz::FC0_MAX_KHZ_OFFSET) & Fc0MaxKhz::FC0_MAX_KHZ_MASK)
        }
    }
}

impl Default for Fc0MaxKhzVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Fc0MaxKhz {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Fc0MaxKhzVal {
        Fc0MaxKhzVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Fc0MaxKhzVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Fc0MaxKhzVal) -> Fc0MaxKhzVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// FC0_DELAY Register
///
/// Delays the start of frequency counting to allow the mux to settle
/// Delay is measured in multiples of the reference clock period
///
/// Address offset: 0x0098
/// Reset value: 0x00000001
///
/// ## Fields
///
/// - **FC0_DELAY** `[2:0]` (RW)
#[repr(transparent)]
pub struct Fc0Delay {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Fc0Delay {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Fc0Delay {
    // ======== Field Constants ========

    /// **FC0_DELAY**
    /// Bits `[2:0]`, mask `0x00000007`
    pub const FC0_DELAY_OFFSET: u32 = 0;
    pub const FC0_DELAY_MASK: u32 = 0x00000007;

    // ======== Field Accessors ========

    /// **FC0_DELAY** `[2:0]` (RW)
    #[inline(always)]
    pub fn fc0_delay(&self) -> u32 {
        (self.value.read_bits() & Self::FC0_DELAY_MASK) >> Self::FC0_DELAY_OFFSET
    }

    /// Write **FC0_DELAY** `[2:0]` (RW)
    #[inline(always)]
    pub fn write_fc0_delay(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::FC0_DELAY_MASK) | ((value << Self::FC0_DELAY_OFFSET) & Self::FC0_DELAY_MASK));
    }
}

/// FC0_DELAY value type for batch read/modify/write operations
///
/// Use `fc0delay.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `fc0delay.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Fc0DelayVal {
    bits: u32,
}

impl Fc0DelayVal {
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
        Self { bits: 0x00000001 }
    }

    /// Read **FC0_DELAY** `[2:0]`
    #[inline(always)]
    pub const fn fc0_delay(&self) -> u32 {
        (self.bits & Fc0Delay::FC0_DELAY_MASK) >> Fc0Delay::FC0_DELAY_OFFSET
    }

    /// Set **FC0_DELAY** `[2:0]`
    #[inline(always)]
    pub const fn with_fc0_delay(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Fc0Delay::FC0_DELAY_MASK) | ((value << Fc0Delay::FC0_DELAY_OFFSET) & Fc0Delay::FC0_DELAY_MASK)
        }
    }
}

impl Default for Fc0DelayVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Fc0Delay {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Fc0DelayVal {
        Fc0DelayVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Fc0DelayVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Fc0DelayVal) -> Fc0DelayVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// FC0_INTERVAL Register
///
/// The test interval is 0.98us * 2**interval, but let's call it 1us * 2**interval
/// The default gives a test interval of 250us
///
/// Address offset: 0x009c
/// Reset value: 0x00000008
///
/// ## Fields
///
/// - **FC0_INTERVAL** `[3:0]` (RW)
#[repr(transparent)]
pub struct Fc0Interval {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Fc0Interval {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Fc0Interval {
    // ======== Field Constants ========

    /// **FC0_INTERVAL**
    /// Bits `[3:0]`, mask `0x0000000f`
    pub const FC0_INTERVAL_OFFSET: u32 = 0;
    pub const FC0_INTERVAL_MASK: u32 = 0x0000000f;

    // ======== Field Accessors ========

    /// **FC0_INTERVAL** `[3:0]` (RW)
    #[inline(always)]
    pub fn fc0_interval(&self) -> u32 {
        (self.value.read_bits() & Self::FC0_INTERVAL_MASK) >> Self::FC0_INTERVAL_OFFSET
    }

    /// Write **FC0_INTERVAL** `[3:0]` (RW)
    #[inline(always)]
    pub fn write_fc0_interval(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::FC0_INTERVAL_MASK) | ((value << Self::FC0_INTERVAL_OFFSET) & Self::FC0_INTERVAL_MASK));
    }
}

/// FC0_INTERVAL value type for batch read/modify/write operations
///
/// Use `fc0interval.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `fc0interval.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Fc0IntervalVal {
    bits: u32,
}

impl Fc0IntervalVal {
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
        Self { bits: 0x00000008 }
    }

    /// Read **FC0_INTERVAL** `[3:0]`
    #[inline(always)]
    pub const fn fc0_interval(&self) -> u32 {
        (self.bits & Fc0Interval::FC0_INTERVAL_MASK) >> Fc0Interval::FC0_INTERVAL_OFFSET
    }

    /// Set **FC0_INTERVAL** `[3:0]`
    #[inline(always)]
    pub const fn with_fc0_interval(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Fc0Interval::FC0_INTERVAL_MASK) | ((value << Fc0Interval::FC0_INTERVAL_OFFSET) & Fc0Interval::FC0_INTERVAL_MASK)
        }
    }
}

impl Default for Fc0IntervalVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Fc0Interval {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Fc0IntervalVal {
        Fc0IntervalVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Fc0IntervalVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Fc0IntervalVal) -> Fc0IntervalVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// FC0_SRC Register
///
/// Clock sent to frequency counter, set to 0 when not required
/// Writing to this register initiates the frequency count
///
/// Address offset: 0x00a0
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **FC0_SRC** `[7:0]` (RW)
#[repr(transparent)]
pub struct Fc0Src {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Fc0Src {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Fc0Src {
    // ======== Field Constants ========

    /// **FC0_SRC**
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const FC0_SRC_OFFSET: u32 = 0;
    pub const FC0_SRC_MASK: u32 = 0x000000ff;
    pub const FC0_SRC_NULL: u32 = 0 << 0;
    pub const FC0_SRC_NULL_VAL: u32 = 0;
    pub const FC0_SRC_PLL_SYS_CLKSRC_PRIMARY: u32 = 1 << 0;
    pub const FC0_SRC_PLL_SYS_CLKSRC_PRIMARY_VAL: u32 = 1;
    pub const FC0_SRC_PLL_USB_CLKSRC_PRIMARY: u32 = 2 << 0;
    pub const FC0_SRC_PLL_USB_CLKSRC_PRIMARY_VAL: u32 = 2;
    pub const FC0_SRC_ROSC_CLKSRC: u32 = 3 << 0;
    pub const FC0_SRC_ROSC_CLKSRC_VAL: u32 = 3;
    pub const FC0_SRC_ROSC_CLKSRC_PH: u32 = 4 << 0;
    pub const FC0_SRC_ROSC_CLKSRC_PH_VAL: u32 = 4;
    pub const FC0_SRC_XOSC_CLKSRC: u32 = 5 << 0;
    pub const FC0_SRC_XOSC_CLKSRC_VAL: u32 = 5;
    pub const FC0_SRC_CLKSRC_GPIN0: u32 = 6 << 0;
    pub const FC0_SRC_CLKSRC_GPIN0_VAL: u32 = 6;
    pub const FC0_SRC_CLKSRC_GPIN1: u32 = 7 << 0;
    pub const FC0_SRC_CLKSRC_GPIN1_VAL: u32 = 7;
    pub const FC0_SRC_CLK_REF: u32 = 8 << 0;
    pub const FC0_SRC_CLK_REF_VAL: u32 = 8;
    pub const FC0_SRC_CLK_SYS: u32 = 9 << 0;
    pub const FC0_SRC_CLK_SYS_VAL: u32 = 9;
    pub const FC0_SRC_CLK_PERI: u32 = 10 << 0;
    pub const FC0_SRC_CLK_PERI_VAL: u32 = 10;
    pub const FC0_SRC_CLK_USB: u32 = 11 << 0;
    pub const FC0_SRC_CLK_USB_VAL: u32 = 11;
    pub const FC0_SRC_CLK_ADC: u32 = 12 << 0;
    pub const FC0_SRC_CLK_ADC_VAL: u32 = 12;
    pub const FC0_SRC_CLK_HSTX: u32 = 13 << 0;
    pub const FC0_SRC_CLK_HSTX_VAL: u32 = 13;
    pub const FC0_SRC_LPOSC_CLKSRC: u32 = 14 << 0;
    pub const FC0_SRC_LPOSC_CLKSRC_VAL: u32 = 14;
    pub const FC0_SRC_OTP_CLK2FC: u32 = 15 << 0;
    pub const FC0_SRC_OTP_CLK2FC_VAL: u32 = 15;
    pub const FC0_SRC_PLL_USB_CLKSRC_PRIMARY_DFT: u32 = 16 << 0;
    pub const FC0_SRC_PLL_USB_CLKSRC_PRIMARY_DFT_VAL: u32 = 16;

    // ======== Field Accessors ========

    /// **FC0_SRC** `[7:0]` (RW)
    #[inline(always)]
    pub fn fc0_src(&self) -> u32 {
        (self.value.read_bits() & Self::FC0_SRC_MASK) >> Self::FC0_SRC_OFFSET
    }

    /// Write **FC0_SRC** `[7:0]` (RW)
    #[inline(always)]
    pub fn write_fc0_src(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::FC0_SRC_MASK) | ((value << Self::FC0_SRC_OFFSET) & Self::FC0_SRC_MASK));
    }
}

/// FC0_SRC value type for batch read/modify/write operations
///
/// Use `fc0src.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `fc0src.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Fc0SrcVal {
    bits: u32,
}

impl Fc0SrcVal {
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

    /// Read **FC0_SRC** `[7:0]`
    #[inline(always)]
    pub const fn fc0_src(&self) -> u32 {
        (self.bits & Fc0Src::FC0_SRC_MASK) >> Fc0Src::FC0_SRC_OFFSET
    }

    /// Set **FC0_SRC** `[7:0]`
    #[inline(always)]
    pub const fn with_fc0_src(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Fc0Src::FC0_SRC_MASK) | ((value << Fc0Src::FC0_SRC_OFFSET) & Fc0Src::FC0_SRC_MASK)
        }
    }
}

impl Default for Fc0SrcVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Fc0Src {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Fc0SrcVal {
        Fc0SrcVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Fc0SrcVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Fc0SrcVal) -> Fc0SrcVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// FC0_STATUS Register
///
/// Frequency counter status
///
/// Address offset: 0x00a4
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **DIED** `[28]` (RO)
///
///   Test clock stopped during test
///
/// - **FAST** `[24]` (RO)
///
///   Test clock faster than expected, only valid when status_done=1
///
/// - **SLOW** `[20]` (RO)
///
///   Test clock slower than expected, only valid when status_done=1
///
/// - **FAIL** `[16]` (RO)
///
///   Test failed
///
/// - **WAITING** `[12]` (RO)
///
///   Waiting for test clock to start
///
/// - **RUNNING** `[8]` (RO)
///
///   Test running
///
/// - **DONE** `[4]` (RO)
///
///   Test complete
///
/// - **PASS** `[0]` (RO)
///
///   Test passed
#[repr(transparent)]
pub struct Fc0Status {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Fc0Status {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Fc0Status {
    // ======== Field Constants ========

    /// **DIED**
    ///
    /// Test clock stopped during test
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const DIED_OFFSET: u32 = 28;
    pub const DIED_MASK: u32 = 0x10000000;
    pub const DIED: u32 = 1 << 28;

    /// **FAST**
    ///
    /// Test clock faster than expected, only valid when status_done=1
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const FAST_OFFSET: u32 = 24;
    pub const FAST_MASK: u32 = 0x01000000;
    pub const FAST: u32 = 1 << 24;

    /// **SLOW**
    ///
    /// Test clock slower than expected, only valid when status_done=1
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const SLOW_OFFSET: u32 = 20;
    pub const SLOW_MASK: u32 = 0x00100000;
    pub const SLOW: u32 = 1 << 20;

    /// **FAIL**
    ///
    /// Test failed
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const FAIL_OFFSET: u32 = 16;
    pub const FAIL_MASK: u32 = 0x00010000;
    pub const FAIL: u32 = 1 << 16;

    /// **WAITING**
    ///
    /// Waiting for test clock to start
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const WAITING_OFFSET: u32 = 12;
    pub const WAITING_MASK: u32 = 0x00001000;
    pub const WAITING: u32 = 1 << 12;

    /// **RUNNING**
    ///
    /// Test running
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const RUNNING_OFFSET: u32 = 8;
    pub const RUNNING_MASK: u32 = 0x00000100;
    pub const RUNNING: u32 = 1 << 8;

    /// **DONE**
    ///
    /// Test complete
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const DONE_OFFSET: u32 = 4;
    pub const DONE_MASK: u32 = 0x00000010;
    pub const DONE: u32 = 1 << 4;

    /// **PASS**
    ///
    /// Test passed
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const PASS_OFFSET: u32 = 0;
    pub const PASS_MASK: u32 = 0x00000001;
    pub const PASS: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DIED** `[28]` (RO)
    ///
    /// Test clock stopped during test
    #[inline(always)]
    pub fn died(&self) -> bool {
        (self.value.read_bits() & Self::DIED_MASK) != 0
    }

    /// **FAST** `[24]` (RO)
    ///
    /// Test clock faster than expected, only valid when status_done=1
    #[inline(always)]
    pub fn fast(&self) -> bool {
        (self.value.read_bits() & Self::FAST_MASK) != 0
    }

    /// **SLOW** `[20]` (RO)
    ///
    /// Test clock slower than expected, only valid when status_done=1
    #[inline(always)]
    pub fn slow(&self) -> bool {
        (self.value.read_bits() & Self::SLOW_MASK) != 0
    }

    /// **FAIL** `[16]` (RO)
    ///
    /// Test failed
    #[inline(always)]
    pub fn fail(&self) -> bool {
        (self.value.read_bits() & Self::FAIL_MASK) != 0
    }

    /// **WAITING** `[12]` (RO)
    ///
    /// Waiting for test clock to start
    #[inline(always)]
    pub fn waiting(&self) -> bool {
        (self.value.read_bits() & Self::WAITING_MASK) != 0
    }

    /// **RUNNING** `[8]` (RO)
    ///
    /// Test running
    #[inline(always)]
    pub fn running(&self) -> bool {
        (self.value.read_bits() & Self::RUNNING_MASK) != 0
    }

    /// **DONE** `[4]` (RO)
    ///
    /// Test complete
    #[inline(always)]
    pub fn done(&self) -> bool {
        (self.value.read_bits() & Self::DONE_MASK) != 0
    }

    /// **PASS** `[0]` (RO)
    ///
    /// Test passed
    #[inline(always)]
    pub fn pass(&self) -> bool {
        (self.value.read_bits() & Self::PASS_MASK) != 0
    }
}

/// FC0_STATUS value type for batch read/modify/write operations
///
/// Use `fc0status.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `fc0status.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Fc0StatusVal {
    bits: u32,
}

impl Fc0StatusVal {
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

    /// Read **DIED** `[28:28]`
    ///
    /// Test clock stopped during test
    #[inline(always)]
    pub const fn died(&self) -> bool {
        (self.bits & Fc0Status::DIED_MASK) != 0
    }

    /// Read **FAST** `[24:24]`
    ///
    /// Test clock faster than expected, only valid when status_done=1
    #[inline(always)]
    pub const fn fast(&self) -> bool {
        (self.bits & Fc0Status::FAST_MASK) != 0
    }

    /// Read **SLOW** `[20:20]`
    ///
    /// Test clock slower than expected, only valid when status_done=1
    #[inline(always)]
    pub const fn slow(&self) -> bool {
        (self.bits & Fc0Status::SLOW_MASK) != 0
    }

    /// Read **FAIL** `[16:16]`
    ///
    /// Test failed
    #[inline(always)]
    pub const fn fail(&self) -> bool {
        (self.bits & Fc0Status::FAIL_MASK) != 0
    }

    /// Read **WAITING** `[12:12]`
    ///
    /// Waiting for test clock to start
    #[inline(always)]
    pub const fn waiting(&self) -> bool {
        (self.bits & Fc0Status::WAITING_MASK) != 0
    }

    /// Read **RUNNING** `[8:8]`
    ///
    /// Test running
    #[inline(always)]
    pub const fn running(&self) -> bool {
        (self.bits & Fc0Status::RUNNING_MASK) != 0
    }

    /// Read **DONE** `[4:4]`
    ///
    /// Test complete
    #[inline(always)]
    pub const fn done(&self) -> bool {
        (self.bits & Fc0Status::DONE_MASK) != 0
    }

    /// Read **PASS** `[0:0]`
    ///
    /// Test passed
    #[inline(always)]
    pub const fn pass(&self) -> bool {
        (self.bits & Fc0Status::PASS_MASK) != 0
    }
}

impl Default for Fc0StatusVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Fc0Status {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Fc0StatusVal {
        Fc0StatusVal::from_bits(self.value.read_bits())
    }
}

/// FC0_RESULT Register
///
/// Result of frequency measurement, only valid when status_done=1
///
/// Address offset: 0x00a8
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **KHZ** `[29:5]` (RO)
///
/// - **FRAC** `[4:0]` (RO)
#[repr(transparent)]
pub struct Fc0Result {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Fc0Result {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Fc0Result {
    // ======== Field Constants ========

    /// **KHZ**
    /// Bits `[29:5]`, mask `0x3fffffe0`
    pub const KHZ_OFFSET: u32 = 5;
    pub const KHZ_MASK: u32 = 0x3fffffe0;

    /// **FRAC**
    /// Bits `[4:0]`, mask `0x0000001f`
    pub const FRAC_OFFSET: u32 = 0;
    pub const FRAC_MASK: u32 = 0x0000001f;

    // ======== Field Accessors ========

    /// **KHZ** `[29:5]` (RO)
    #[inline(always)]
    pub fn khz(&self) -> u32 {
        (self.value.read_bits() & Self::KHZ_MASK) >> Self::KHZ_OFFSET
    }

    /// **FRAC** `[4:0]` (RO)
    #[inline(always)]
    pub fn frac(&self) -> u32 {
        (self.value.read_bits() & Self::FRAC_MASK) >> Self::FRAC_OFFSET
    }
}

/// FC0_RESULT value type for batch read/modify/write operations
///
/// Use `fc0result.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `fc0result.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Fc0ResultVal {
    bits: u32,
}

impl Fc0ResultVal {
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

    /// Read **KHZ** `[29:5]`
    #[inline(always)]
    pub const fn khz(&self) -> u32 {
        (self.bits & Fc0Result::KHZ_MASK) >> Fc0Result::KHZ_OFFSET
    }

    /// Read **FRAC** `[4:0]`
    #[inline(always)]
    pub const fn frac(&self) -> u32 {
        (self.bits & Fc0Result::FRAC_MASK) >> Fc0Result::FRAC_OFFSET
    }
}

impl Default for Fc0ResultVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Fc0Result {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Fc0ResultVal {
        Fc0ResultVal::from_bits(self.value.read_bits())
    }
}

/// WAKE_EN0 Register
///
/// enable clock in wake mode
///
/// Address offset: 0x00ac
/// Reset value: 0xffffffff
///
/// ## Fields
///
/// - **CLK_SYS_SIO** `[31]` (RW)
///
/// - **CLK_SYS_SHA256** `[30]` (RW)
///
/// - **CLK_SYS_PSM** `[29]` (RW)
///
/// - **CLK_SYS_ROSC** `[28]` (RW)
///
/// - **CLK_SYS_ROM** `[27]` (RW)
///
/// - **CLK_SYS_RESETS** `[26]` (RW)
///
/// - **CLK_SYS_PWM** `[25]` (RW)
///
/// - **CLK_SYS_POWMAN** `[24]` (RW)
///
/// - **CLK_REF_POWMAN** `[23]` (RW)
///
/// - **CLK_SYS_PLL_USB** `[22]` (RW)
///
/// - **CLK_SYS_PLL_SYS** `[21]` (RW)
///
/// - **CLK_SYS_PIO2** `[20]` (RW)
///
/// - **CLK_SYS_PIO1** `[19]` (RW)
///
/// - **CLK_SYS_PIO0** `[18]` (RW)
///
/// - **CLK_SYS_PADS** `[17]` (RW)
///
/// - **CLK_SYS_OTP** `[16]` (RW)
///
/// - **CLK_REF_OTP** `[15]` (RW)
///
/// - **CLK_SYS_JTAG** `[14]` (RW)
///
/// - **CLK_SYS_IO** `[13]` (RW)
///
/// - **CLK_SYS_I2C1** `[12]` (RW)
///
/// - **CLK_SYS_I2C0** `[11]` (RW)
///
/// - **CLK_SYS_HSTX** `[10]` (RW)
///
/// - **CLK_HSTX** `[9]` (RW)
///
/// - **CLK_SYS_GLITCH_DETECTOR** `[8]` (RW)
///
/// - **CLK_SYS_DMA** `[7]` (RW)
///
/// - **CLK_SYS_BUSFABRIC** `[6]` (RW)
///
/// - **CLK_SYS_BUSCTRL** `[5]` (RW)
///
/// - **CLK_SYS_BOOTRAM** `[4]` (RW)
///
/// - **CLK_SYS_ADC** `[3]` (RW)
///
/// - **CLK_ADC** `[2]` (RW)
///
/// - **CLK_SYS_ACCESSCTRL** `[1]` (RW)
///
/// - **CLK_SYS_CLOCKS** `[0]` (RW)
#[repr(transparent)]
pub struct WakeEn0 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for WakeEn0 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl WakeEn0 {
    // ======== Field Constants ========

    /// **CLK_SYS_SIO**
    /// Bits `[31:31]`, mask `0x80000000`
    pub const CLK_SYS_SIO_OFFSET: u32 = 31;
    pub const CLK_SYS_SIO_MASK: u32 = 0x80000000;
    pub const CLK_SYS_SIO: u32 = 1 << 31;

    /// **CLK_SYS_SHA256**
    /// Bits `[30:30]`, mask `0x40000000`
    pub const CLK_SYS_SHA256_OFFSET: u32 = 30;
    pub const CLK_SYS_SHA256_MASK: u32 = 0x40000000;
    pub const CLK_SYS_SHA256: u32 = 1 << 30;

    /// **CLK_SYS_PSM**
    /// Bits `[29:29]`, mask `0x20000000`
    pub const CLK_SYS_PSM_OFFSET: u32 = 29;
    pub const CLK_SYS_PSM_MASK: u32 = 0x20000000;
    pub const CLK_SYS_PSM: u32 = 1 << 29;

    /// **CLK_SYS_ROSC**
    /// Bits `[28:28]`, mask `0x10000000`
    pub const CLK_SYS_ROSC_OFFSET: u32 = 28;
    pub const CLK_SYS_ROSC_MASK: u32 = 0x10000000;
    pub const CLK_SYS_ROSC: u32 = 1 << 28;

    /// **CLK_SYS_ROM**
    /// Bits `[27:27]`, mask `0x08000000`
    pub const CLK_SYS_ROM_OFFSET: u32 = 27;
    pub const CLK_SYS_ROM_MASK: u32 = 0x08000000;
    pub const CLK_SYS_ROM: u32 = 1 << 27;

    /// **CLK_SYS_RESETS**
    /// Bits `[26:26]`, mask `0x04000000`
    pub const CLK_SYS_RESETS_OFFSET: u32 = 26;
    pub const CLK_SYS_RESETS_MASK: u32 = 0x04000000;
    pub const CLK_SYS_RESETS: u32 = 1 << 26;

    /// **CLK_SYS_PWM**
    /// Bits `[25:25]`, mask `0x02000000`
    pub const CLK_SYS_PWM_OFFSET: u32 = 25;
    pub const CLK_SYS_PWM_MASK: u32 = 0x02000000;
    pub const CLK_SYS_PWM: u32 = 1 << 25;

    /// **CLK_SYS_POWMAN**
    /// Bits `[24:24]`, mask `0x01000000`
    pub const CLK_SYS_POWMAN_OFFSET: u32 = 24;
    pub const CLK_SYS_POWMAN_MASK: u32 = 0x01000000;
    pub const CLK_SYS_POWMAN: u32 = 1 << 24;

    /// **CLK_REF_POWMAN**
    /// Bits `[23:23]`, mask `0x00800000`
    pub const CLK_REF_POWMAN_OFFSET: u32 = 23;
    pub const CLK_REF_POWMAN_MASK: u32 = 0x00800000;
    pub const CLK_REF_POWMAN: u32 = 1 << 23;

    /// **CLK_SYS_PLL_USB**
    /// Bits `[22:22]`, mask `0x00400000`
    pub const CLK_SYS_PLL_USB_OFFSET: u32 = 22;
    pub const CLK_SYS_PLL_USB_MASK: u32 = 0x00400000;
    pub const CLK_SYS_PLL_USB: u32 = 1 << 22;

    /// **CLK_SYS_PLL_SYS**
    /// Bits `[21:21]`, mask `0x00200000`
    pub const CLK_SYS_PLL_SYS_OFFSET: u32 = 21;
    pub const CLK_SYS_PLL_SYS_MASK: u32 = 0x00200000;
    pub const CLK_SYS_PLL_SYS: u32 = 1 << 21;

    /// **CLK_SYS_PIO2**
    /// Bits `[20:20]`, mask `0x00100000`
    pub const CLK_SYS_PIO2_OFFSET: u32 = 20;
    pub const CLK_SYS_PIO2_MASK: u32 = 0x00100000;
    pub const CLK_SYS_PIO2: u32 = 1 << 20;

    /// **CLK_SYS_PIO1**
    /// Bits `[19:19]`, mask `0x00080000`
    pub const CLK_SYS_PIO1_OFFSET: u32 = 19;
    pub const CLK_SYS_PIO1_MASK: u32 = 0x00080000;
    pub const CLK_SYS_PIO1: u32 = 1 << 19;

    /// **CLK_SYS_PIO0**
    /// Bits `[18:18]`, mask `0x00040000`
    pub const CLK_SYS_PIO0_OFFSET: u32 = 18;
    pub const CLK_SYS_PIO0_MASK: u32 = 0x00040000;
    pub const CLK_SYS_PIO0: u32 = 1 << 18;

    /// **CLK_SYS_PADS**
    /// Bits `[17:17]`, mask `0x00020000`
    pub const CLK_SYS_PADS_OFFSET: u32 = 17;
    pub const CLK_SYS_PADS_MASK: u32 = 0x00020000;
    pub const CLK_SYS_PADS: u32 = 1 << 17;

    /// **CLK_SYS_OTP**
    /// Bits `[16:16]`, mask `0x00010000`
    pub const CLK_SYS_OTP_OFFSET: u32 = 16;
    pub const CLK_SYS_OTP_MASK: u32 = 0x00010000;
    pub const CLK_SYS_OTP: u32 = 1 << 16;

    /// **CLK_REF_OTP**
    /// Bits `[15:15]`, mask `0x00008000`
    pub const CLK_REF_OTP_OFFSET: u32 = 15;
    pub const CLK_REF_OTP_MASK: u32 = 0x00008000;
    pub const CLK_REF_OTP: u32 = 1 << 15;

    /// **CLK_SYS_JTAG**
    /// Bits `[14:14]`, mask `0x00004000`
    pub const CLK_SYS_JTAG_OFFSET: u32 = 14;
    pub const CLK_SYS_JTAG_MASK: u32 = 0x00004000;
    pub const CLK_SYS_JTAG: u32 = 1 << 14;

    /// **CLK_SYS_IO**
    /// Bits `[13:13]`, mask `0x00002000`
    pub const CLK_SYS_IO_OFFSET: u32 = 13;
    pub const CLK_SYS_IO_MASK: u32 = 0x00002000;
    pub const CLK_SYS_IO: u32 = 1 << 13;

    /// **CLK_SYS_I2C1**
    /// Bits `[12:12]`, mask `0x00001000`
    pub const CLK_SYS_I2C1_OFFSET: u32 = 12;
    pub const CLK_SYS_I2C1_MASK: u32 = 0x00001000;
    pub const CLK_SYS_I2C1: u32 = 1 << 12;

    /// **CLK_SYS_I2C0**
    /// Bits `[11:11]`, mask `0x00000800`
    pub const CLK_SYS_I2C0_OFFSET: u32 = 11;
    pub const CLK_SYS_I2C0_MASK: u32 = 0x00000800;
    pub const CLK_SYS_I2C0: u32 = 1 << 11;

    /// **CLK_SYS_HSTX**
    /// Bits `[10:10]`, mask `0x00000400`
    pub const CLK_SYS_HSTX_OFFSET: u32 = 10;
    pub const CLK_SYS_HSTX_MASK: u32 = 0x00000400;
    pub const CLK_SYS_HSTX: u32 = 1 << 10;

    /// **CLK_HSTX**
    /// Bits `[9:9]`, mask `0x00000200`
    pub const CLK_HSTX_OFFSET: u32 = 9;
    pub const CLK_HSTX_MASK: u32 = 0x00000200;
    pub const CLK_HSTX: u32 = 1 << 9;

    /// **CLK_SYS_GLITCH_DETECTOR**
    /// Bits `[8:8]`, mask `0x00000100`
    pub const CLK_SYS_GLITCH_DETECTOR_OFFSET: u32 = 8;
    pub const CLK_SYS_GLITCH_DETECTOR_MASK: u32 = 0x00000100;
    pub const CLK_SYS_GLITCH_DETECTOR: u32 = 1 << 8;

    /// **CLK_SYS_DMA**
    /// Bits `[7:7]`, mask `0x00000080`
    pub const CLK_SYS_DMA_OFFSET: u32 = 7;
    pub const CLK_SYS_DMA_MASK: u32 = 0x00000080;
    pub const CLK_SYS_DMA: u32 = 1 << 7;

    /// **CLK_SYS_BUSFABRIC**
    /// Bits `[6:6]`, mask `0x00000040`
    pub const CLK_SYS_BUSFABRIC_OFFSET: u32 = 6;
    pub const CLK_SYS_BUSFABRIC_MASK: u32 = 0x00000040;
    pub const CLK_SYS_BUSFABRIC: u32 = 1 << 6;

    /// **CLK_SYS_BUSCTRL**
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CLK_SYS_BUSCTRL_OFFSET: u32 = 5;
    pub const CLK_SYS_BUSCTRL_MASK: u32 = 0x00000020;
    pub const CLK_SYS_BUSCTRL: u32 = 1 << 5;

    /// **CLK_SYS_BOOTRAM**
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CLK_SYS_BOOTRAM_OFFSET: u32 = 4;
    pub const CLK_SYS_BOOTRAM_MASK: u32 = 0x00000010;
    pub const CLK_SYS_BOOTRAM: u32 = 1 << 4;

    /// **CLK_SYS_ADC**
    /// Bits `[3:3]`, mask `0x00000008`
    pub const CLK_SYS_ADC_OFFSET: u32 = 3;
    pub const CLK_SYS_ADC_MASK: u32 = 0x00000008;
    pub const CLK_SYS_ADC: u32 = 1 << 3;

    /// **CLK_ADC**
    /// Bits `[2:2]`, mask `0x00000004`
    pub const CLK_ADC_OFFSET: u32 = 2;
    pub const CLK_ADC_MASK: u32 = 0x00000004;
    pub const CLK_ADC: u32 = 1 << 2;

    /// **CLK_SYS_ACCESSCTRL**
    /// Bits `[1:1]`, mask `0x00000002`
    pub const CLK_SYS_ACCESSCTRL_OFFSET: u32 = 1;
    pub const CLK_SYS_ACCESSCTRL_MASK: u32 = 0x00000002;
    pub const CLK_SYS_ACCESSCTRL: u32 = 1 << 1;

    /// **CLK_SYS_CLOCKS**
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CLK_SYS_CLOCKS_OFFSET: u32 = 0;
    pub const CLK_SYS_CLOCKS_MASK: u32 = 0x00000001;
    pub const CLK_SYS_CLOCKS: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CLK_SYS_SIO** `[31]` (RW)
    #[inline(always)]
    pub fn clk_sys_sio(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_SIO_MASK) != 0
    }

    /// Set CLK_SYS_SIO bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_sio(&self) {
        self.set_bits(Self::CLK_SYS_SIO_MASK);
    }

    /// Clear CLK_SYS_SIO bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_sio(&self) {
        self.clr_bits(Self::CLK_SYS_SIO_MASK);
    }

    /// **CLK_SYS_SHA256** `[30]` (RW)
    #[inline(always)]
    pub fn clk_sys_sha256(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_SHA256_MASK) != 0
    }

    /// Set CLK_SYS_SHA256 bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_sha256(&self) {
        self.set_bits(Self::CLK_SYS_SHA256_MASK);
    }

    /// Clear CLK_SYS_SHA256 bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_sha256(&self) {
        self.clr_bits(Self::CLK_SYS_SHA256_MASK);
    }

    /// **CLK_SYS_PSM** `[29]` (RW)
    #[inline(always)]
    pub fn clk_sys_psm(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_PSM_MASK) != 0
    }

    /// Set CLK_SYS_PSM bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_psm(&self) {
        self.set_bits(Self::CLK_SYS_PSM_MASK);
    }

    /// Clear CLK_SYS_PSM bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_psm(&self) {
        self.clr_bits(Self::CLK_SYS_PSM_MASK);
    }

    /// **CLK_SYS_ROSC** `[28]` (RW)
    #[inline(always)]
    pub fn clk_sys_rosc(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_ROSC_MASK) != 0
    }

    /// Set CLK_SYS_ROSC bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_rosc(&self) {
        self.set_bits(Self::CLK_SYS_ROSC_MASK);
    }

    /// Clear CLK_SYS_ROSC bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_rosc(&self) {
        self.clr_bits(Self::CLK_SYS_ROSC_MASK);
    }

    /// **CLK_SYS_ROM** `[27]` (RW)
    #[inline(always)]
    pub fn clk_sys_rom(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_ROM_MASK) != 0
    }

    /// Set CLK_SYS_ROM bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_rom(&self) {
        self.set_bits(Self::CLK_SYS_ROM_MASK);
    }

    /// Clear CLK_SYS_ROM bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_rom(&self) {
        self.clr_bits(Self::CLK_SYS_ROM_MASK);
    }

    /// **CLK_SYS_RESETS** `[26]` (RW)
    #[inline(always)]
    pub fn clk_sys_resets(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_RESETS_MASK) != 0
    }

    /// Set CLK_SYS_RESETS bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_resets(&self) {
        self.set_bits(Self::CLK_SYS_RESETS_MASK);
    }

    /// Clear CLK_SYS_RESETS bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_resets(&self) {
        self.clr_bits(Self::CLK_SYS_RESETS_MASK);
    }

    /// **CLK_SYS_PWM** `[25]` (RW)
    #[inline(always)]
    pub fn clk_sys_pwm(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_PWM_MASK) != 0
    }

    /// Set CLK_SYS_PWM bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_pwm(&self) {
        self.set_bits(Self::CLK_SYS_PWM_MASK);
    }

    /// Clear CLK_SYS_PWM bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_pwm(&self) {
        self.clr_bits(Self::CLK_SYS_PWM_MASK);
    }

    /// **CLK_SYS_POWMAN** `[24]` (RW)
    #[inline(always)]
    pub fn clk_sys_powman(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_POWMAN_MASK) != 0
    }

    /// Set CLK_SYS_POWMAN bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_powman(&self) {
        self.set_bits(Self::CLK_SYS_POWMAN_MASK);
    }

    /// Clear CLK_SYS_POWMAN bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_powman(&self) {
        self.clr_bits(Self::CLK_SYS_POWMAN_MASK);
    }

    /// **CLK_REF_POWMAN** `[23]` (RW)
    #[inline(always)]
    pub fn clk_ref_powman(&self) -> bool {
        (self.value.read_bits() & Self::CLK_REF_POWMAN_MASK) != 0
    }

    /// Set CLK_REF_POWMAN bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_ref_powman(&self) {
        self.set_bits(Self::CLK_REF_POWMAN_MASK);
    }

    /// Clear CLK_REF_POWMAN bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_ref_powman(&self) {
        self.clr_bits(Self::CLK_REF_POWMAN_MASK);
    }

    /// **CLK_SYS_PLL_USB** `[22]` (RW)
    #[inline(always)]
    pub fn clk_sys_pll_usb(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_PLL_USB_MASK) != 0
    }

    /// Set CLK_SYS_PLL_USB bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_pll_usb(&self) {
        self.set_bits(Self::CLK_SYS_PLL_USB_MASK);
    }

    /// Clear CLK_SYS_PLL_USB bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_pll_usb(&self) {
        self.clr_bits(Self::CLK_SYS_PLL_USB_MASK);
    }

    /// **CLK_SYS_PLL_SYS** `[21]` (RW)
    #[inline(always)]
    pub fn clk_sys_pll_sys(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_PLL_SYS_MASK) != 0
    }

    /// Set CLK_SYS_PLL_SYS bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_pll_sys(&self) {
        self.set_bits(Self::CLK_SYS_PLL_SYS_MASK);
    }

    /// Clear CLK_SYS_PLL_SYS bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_pll_sys(&self) {
        self.clr_bits(Self::CLK_SYS_PLL_SYS_MASK);
    }

    /// **CLK_SYS_PIO2** `[20]` (RW)
    #[inline(always)]
    pub fn clk_sys_pio2(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_PIO2_MASK) != 0
    }

    /// Set CLK_SYS_PIO2 bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_pio2(&self) {
        self.set_bits(Self::CLK_SYS_PIO2_MASK);
    }

    /// Clear CLK_SYS_PIO2 bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_pio2(&self) {
        self.clr_bits(Self::CLK_SYS_PIO2_MASK);
    }

    /// **CLK_SYS_PIO1** `[19]` (RW)
    #[inline(always)]
    pub fn clk_sys_pio1(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_PIO1_MASK) != 0
    }

    /// Set CLK_SYS_PIO1 bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_pio1(&self) {
        self.set_bits(Self::CLK_SYS_PIO1_MASK);
    }

    /// Clear CLK_SYS_PIO1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_pio1(&self) {
        self.clr_bits(Self::CLK_SYS_PIO1_MASK);
    }

    /// **CLK_SYS_PIO0** `[18]` (RW)
    #[inline(always)]
    pub fn clk_sys_pio0(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_PIO0_MASK) != 0
    }

    /// Set CLK_SYS_PIO0 bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_pio0(&self) {
        self.set_bits(Self::CLK_SYS_PIO0_MASK);
    }

    /// Clear CLK_SYS_PIO0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_pio0(&self) {
        self.clr_bits(Self::CLK_SYS_PIO0_MASK);
    }

    /// **CLK_SYS_PADS** `[17]` (RW)
    #[inline(always)]
    pub fn clk_sys_pads(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_PADS_MASK) != 0
    }

    /// Set CLK_SYS_PADS bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_pads(&self) {
        self.set_bits(Self::CLK_SYS_PADS_MASK);
    }

    /// Clear CLK_SYS_PADS bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_pads(&self) {
        self.clr_bits(Self::CLK_SYS_PADS_MASK);
    }

    /// **CLK_SYS_OTP** `[16]` (RW)
    #[inline(always)]
    pub fn clk_sys_otp(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_OTP_MASK) != 0
    }

    /// Set CLK_SYS_OTP bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_otp(&self) {
        self.set_bits(Self::CLK_SYS_OTP_MASK);
    }

    /// Clear CLK_SYS_OTP bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_otp(&self) {
        self.clr_bits(Self::CLK_SYS_OTP_MASK);
    }

    /// **CLK_REF_OTP** `[15]` (RW)
    #[inline(always)]
    pub fn clk_ref_otp(&self) -> bool {
        (self.value.read_bits() & Self::CLK_REF_OTP_MASK) != 0
    }

    /// Set CLK_REF_OTP bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_ref_otp(&self) {
        self.set_bits(Self::CLK_REF_OTP_MASK);
    }

    /// Clear CLK_REF_OTP bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_ref_otp(&self) {
        self.clr_bits(Self::CLK_REF_OTP_MASK);
    }

    /// **CLK_SYS_JTAG** `[14]` (RW)
    #[inline(always)]
    pub fn clk_sys_jtag(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_JTAG_MASK) != 0
    }

    /// Set CLK_SYS_JTAG bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_jtag(&self) {
        self.set_bits(Self::CLK_SYS_JTAG_MASK);
    }

    /// Clear CLK_SYS_JTAG bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_jtag(&self) {
        self.clr_bits(Self::CLK_SYS_JTAG_MASK);
    }

    /// **CLK_SYS_IO** `[13]` (RW)
    #[inline(always)]
    pub fn clk_sys_io(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_IO_MASK) != 0
    }

    /// Set CLK_SYS_IO bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_io(&self) {
        self.set_bits(Self::CLK_SYS_IO_MASK);
    }

    /// Clear CLK_SYS_IO bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_io(&self) {
        self.clr_bits(Self::CLK_SYS_IO_MASK);
    }

    /// **CLK_SYS_I2C1** `[12]` (RW)
    #[inline(always)]
    pub fn clk_sys_i2c1(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_I2C1_MASK) != 0
    }

    /// Set CLK_SYS_I2C1 bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_i2c1(&self) {
        self.set_bits(Self::CLK_SYS_I2C1_MASK);
    }

    /// Clear CLK_SYS_I2C1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_i2c1(&self) {
        self.clr_bits(Self::CLK_SYS_I2C1_MASK);
    }

    /// **CLK_SYS_I2C0** `[11]` (RW)
    #[inline(always)]
    pub fn clk_sys_i2c0(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_I2C0_MASK) != 0
    }

    /// Set CLK_SYS_I2C0 bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_i2c0(&self) {
        self.set_bits(Self::CLK_SYS_I2C0_MASK);
    }

    /// Clear CLK_SYS_I2C0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_i2c0(&self) {
        self.clr_bits(Self::CLK_SYS_I2C0_MASK);
    }

    /// **CLK_SYS_HSTX** `[10]` (RW)
    #[inline(always)]
    pub fn clk_sys_hstx(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_HSTX_MASK) != 0
    }

    /// Set CLK_SYS_HSTX bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_hstx(&self) {
        self.set_bits(Self::CLK_SYS_HSTX_MASK);
    }

    /// Clear CLK_SYS_HSTX bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_hstx(&self) {
        self.clr_bits(Self::CLK_SYS_HSTX_MASK);
    }

    /// **CLK_HSTX** `[9]` (RW)
    #[inline(always)]
    pub fn clk_hstx(&self) -> bool {
        (self.value.read_bits() & Self::CLK_HSTX_MASK) != 0
    }

    /// Set CLK_HSTX bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_hstx(&self) {
        self.set_bits(Self::CLK_HSTX_MASK);
    }

    /// Clear CLK_HSTX bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_hstx(&self) {
        self.clr_bits(Self::CLK_HSTX_MASK);
    }

    /// **CLK_SYS_GLITCH_DETECTOR** `[8]` (RW)
    #[inline(always)]
    pub fn clk_sys_glitch_detector(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_GLITCH_DETECTOR_MASK) != 0
    }

    /// Set CLK_SYS_GLITCH_DETECTOR bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_glitch_detector(&self) {
        self.set_bits(Self::CLK_SYS_GLITCH_DETECTOR_MASK);
    }

    /// Clear CLK_SYS_GLITCH_DETECTOR bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_glitch_detector(&self) {
        self.clr_bits(Self::CLK_SYS_GLITCH_DETECTOR_MASK);
    }

    /// **CLK_SYS_DMA** `[7]` (RW)
    #[inline(always)]
    pub fn clk_sys_dma(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_DMA_MASK) != 0
    }

    /// Set CLK_SYS_DMA bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_dma(&self) {
        self.set_bits(Self::CLK_SYS_DMA_MASK);
    }

    /// Clear CLK_SYS_DMA bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_dma(&self) {
        self.clr_bits(Self::CLK_SYS_DMA_MASK);
    }

    /// **CLK_SYS_BUSFABRIC** `[6]` (RW)
    #[inline(always)]
    pub fn clk_sys_busfabric(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_BUSFABRIC_MASK) != 0
    }

    /// Set CLK_SYS_BUSFABRIC bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_busfabric(&self) {
        self.set_bits(Self::CLK_SYS_BUSFABRIC_MASK);
    }

    /// Clear CLK_SYS_BUSFABRIC bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_busfabric(&self) {
        self.clr_bits(Self::CLK_SYS_BUSFABRIC_MASK);
    }

    /// **CLK_SYS_BUSCTRL** `[5]` (RW)
    #[inline(always)]
    pub fn clk_sys_busctrl(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_BUSCTRL_MASK) != 0
    }

    /// Set CLK_SYS_BUSCTRL bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_busctrl(&self) {
        self.set_bits(Self::CLK_SYS_BUSCTRL_MASK);
    }

    /// Clear CLK_SYS_BUSCTRL bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_busctrl(&self) {
        self.clr_bits(Self::CLK_SYS_BUSCTRL_MASK);
    }

    /// **CLK_SYS_BOOTRAM** `[4]` (RW)
    #[inline(always)]
    pub fn clk_sys_bootram(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_BOOTRAM_MASK) != 0
    }

    /// Set CLK_SYS_BOOTRAM bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_bootram(&self) {
        self.set_bits(Self::CLK_SYS_BOOTRAM_MASK);
    }

    /// Clear CLK_SYS_BOOTRAM bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_bootram(&self) {
        self.clr_bits(Self::CLK_SYS_BOOTRAM_MASK);
    }

    /// **CLK_SYS_ADC** `[3]` (RW)
    #[inline(always)]
    pub fn clk_sys_adc(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_ADC_MASK) != 0
    }

    /// Set CLK_SYS_ADC bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_adc(&self) {
        self.set_bits(Self::CLK_SYS_ADC_MASK);
    }

    /// Clear CLK_SYS_ADC bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_adc(&self) {
        self.clr_bits(Self::CLK_SYS_ADC_MASK);
    }

    /// **CLK_ADC** `[2]` (RW)
    #[inline(always)]
    pub fn clk_adc(&self) -> bool {
        (self.value.read_bits() & Self::CLK_ADC_MASK) != 0
    }

    /// Set CLK_ADC bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_adc(&self) {
        self.set_bits(Self::CLK_ADC_MASK);
    }

    /// Clear CLK_ADC bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_adc(&self) {
        self.clr_bits(Self::CLK_ADC_MASK);
    }

    /// **CLK_SYS_ACCESSCTRL** `[1]` (RW)
    #[inline(always)]
    pub fn clk_sys_accessctrl(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_ACCESSCTRL_MASK) != 0
    }

    /// Set CLK_SYS_ACCESSCTRL bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_accessctrl(&self) {
        self.set_bits(Self::CLK_SYS_ACCESSCTRL_MASK);
    }

    /// Clear CLK_SYS_ACCESSCTRL bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_accessctrl(&self) {
        self.clr_bits(Self::CLK_SYS_ACCESSCTRL_MASK);
    }

    /// **CLK_SYS_CLOCKS** `[0]` (RW)
    #[inline(always)]
    pub fn clk_sys_clocks(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_CLOCKS_MASK) != 0
    }

    /// Set CLK_SYS_CLOCKS bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_clocks(&self) {
        self.set_bits(Self::CLK_SYS_CLOCKS_MASK);
    }

    /// Clear CLK_SYS_CLOCKS bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_clocks(&self) {
        self.clr_bits(Self::CLK_SYS_CLOCKS_MASK);
    }
}

/// WAKE_EN0 value type for batch read/modify/write operations
///
/// Use `wakeen0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `wakeen0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct WakeEn0Val {
    bits: u32,
}

impl WakeEn0Val {
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
        Self { bits: 0xffffffff }
    }

    /// Read **CLK_SYS_SIO** `[31:31]`
    #[inline(always)]
    pub const fn clk_sys_sio(&self) -> bool {
        (self.bits & WakeEn0::CLK_SYS_SIO_MASK) != 0
    }

    /// Read **CLK_SYS_SHA256** `[30:30]`
    #[inline(always)]
    pub const fn clk_sys_sha256(&self) -> bool {
        (self.bits & WakeEn0::CLK_SYS_SHA256_MASK) != 0
    }

    /// Read **CLK_SYS_PSM** `[29:29]`
    #[inline(always)]
    pub const fn clk_sys_psm(&self) -> bool {
        (self.bits & WakeEn0::CLK_SYS_PSM_MASK) != 0
    }

    /// Read **CLK_SYS_ROSC** `[28:28]`
    #[inline(always)]
    pub const fn clk_sys_rosc(&self) -> bool {
        (self.bits & WakeEn0::CLK_SYS_ROSC_MASK) != 0
    }

    /// Read **CLK_SYS_ROM** `[27:27]`
    #[inline(always)]
    pub const fn clk_sys_rom(&self) -> bool {
        (self.bits & WakeEn0::CLK_SYS_ROM_MASK) != 0
    }

    /// Read **CLK_SYS_RESETS** `[26:26]`
    #[inline(always)]
    pub const fn clk_sys_resets(&self) -> bool {
        (self.bits & WakeEn0::CLK_SYS_RESETS_MASK) != 0
    }

    /// Read **CLK_SYS_PWM** `[25:25]`
    #[inline(always)]
    pub const fn clk_sys_pwm(&self) -> bool {
        (self.bits & WakeEn0::CLK_SYS_PWM_MASK) != 0
    }

    /// Read **CLK_SYS_POWMAN** `[24:24]`
    #[inline(always)]
    pub const fn clk_sys_powman(&self) -> bool {
        (self.bits & WakeEn0::CLK_SYS_POWMAN_MASK) != 0
    }

    /// Read **CLK_REF_POWMAN** `[23:23]`
    #[inline(always)]
    pub const fn clk_ref_powman(&self) -> bool {
        (self.bits & WakeEn0::CLK_REF_POWMAN_MASK) != 0
    }

    /// Read **CLK_SYS_PLL_USB** `[22:22]`
    #[inline(always)]
    pub const fn clk_sys_pll_usb(&self) -> bool {
        (self.bits & WakeEn0::CLK_SYS_PLL_USB_MASK) != 0
    }

    /// Read **CLK_SYS_PLL_SYS** `[21:21]`
    #[inline(always)]
    pub const fn clk_sys_pll_sys(&self) -> bool {
        (self.bits & WakeEn0::CLK_SYS_PLL_SYS_MASK) != 0
    }

    /// Read **CLK_SYS_PIO2** `[20:20]`
    #[inline(always)]
    pub const fn clk_sys_pio2(&self) -> bool {
        (self.bits & WakeEn0::CLK_SYS_PIO2_MASK) != 0
    }

    /// Read **CLK_SYS_PIO1** `[19:19]`
    #[inline(always)]
    pub const fn clk_sys_pio1(&self) -> bool {
        (self.bits & WakeEn0::CLK_SYS_PIO1_MASK) != 0
    }

    /// Read **CLK_SYS_PIO0** `[18:18]`
    #[inline(always)]
    pub const fn clk_sys_pio0(&self) -> bool {
        (self.bits & WakeEn0::CLK_SYS_PIO0_MASK) != 0
    }

    /// Read **CLK_SYS_PADS** `[17:17]`
    #[inline(always)]
    pub const fn clk_sys_pads(&self) -> bool {
        (self.bits & WakeEn0::CLK_SYS_PADS_MASK) != 0
    }

    /// Read **CLK_SYS_OTP** `[16:16]`
    #[inline(always)]
    pub const fn clk_sys_otp(&self) -> bool {
        (self.bits & WakeEn0::CLK_SYS_OTP_MASK) != 0
    }

    /// Read **CLK_REF_OTP** `[15:15]`
    #[inline(always)]
    pub const fn clk_ref_otp(&self) -> bool {
        (self.bits & WakeEn0::CLK_REF_OTP_MASK) != 0
    }

    /// Read **CLK_SYS_JTAG** `[14:14]`
    #[inline(always)]
    pub const fn clk_sys_jtag(&self) -> bool {
        (self.bits & WakeEn0::CLK_SYS_JTAG_MASK) != 0
    }

    /// Read **CLK_SYS_IO** `[13:13]`
    #[inline(always)]
    pub const fn clk_sys_io(&self) -> bool {
        (self.bits & WakeEn0::CLK_SYS_IO_MASK) != 0
    }

    /// Read **CLK_SYS_I2C1** `[12:12]`
    #[inline(always)]
    pub const fn clk_sys_i2c1(&self) -> bool {
        (self.bits & WakeEn0::CLK_SYS_I2C1_MASK) != 0
    }

    /// Read **CLK_SYS_I2C0** `[11:11]`
    #[inline(always)]
    pub const fn clk_sys_i2c0(&self) -> bool {
        (self.bits & WakeEn0::CLK_SYS_I2C0_MASK) != 0
    }

    /// Read **CLK_SYS_HSTX** `[10:10]`
    #[inline(always)]
    pub const fn clk_sys_hstx(&self) -> bool {
        (self.bits & WakeEn0::CLK_SYS_HSTX_MASK) != 0
    }

    /// Read **CLK_HSTX** `[9:9]`
    #[inline(always)]
    pub const fn clk_hstx(&self) -> bool {
        (self.bits & WakeEn0::CLK_HSTX_MASK) != 0
    }

    /// Read **CLK_SYS_GLITCH_DETECTOR** `[8:8]`
    #[inline(always)]
    pub const fn clk_sys_glitch_detector(&self) -> bool {
        (self.bits & WakeEn0::CLK_SYS_GLITCH_DETECTOR_MASK) != 0
    }

    /// Read **CLK_SYS_DMA** `[7:7]`
    #[inline(always)]
    pub const fn clk_sys_dma(&self) -> bool {
        (self.bits & WakeEn0::CLK_SYS_DMA_MASK) != 0
    }

    /// Read **CLK_SYS_BUSFABRIC** `[6:6]`
    #[inline(always)]
    pub const fn clk_sys_busfabric(&self) -> bool {
        (self.bits & WakeEn0::CLK_SYS_BUSFABRIC_MASK) != 0
    }

    /// Read **CLK_SYS_BUSCTRL** `[5:5]`
    #[inline(always)]
    pub const fn clk_sys_busctrl(&self) -> bool {
        (self.bits & WakeEn0::CLK_SYS_BUSCTRL_MASK) != 0
    }

    /// Read **CLK_SYS_BOOTRAM** `[4:4]`
    #[inline(always)]
    pub const fn clk_sys_bootram(&self) -> bool {
        (self.bits & WakeEn0::CLK_SYS_BOOTRAM_MASK) != 0
    }

    /// Read **CLK_SYS_ADC** `[3:3]`
    #[inline(always)]
    pub const fn clk_sys_adc(&self) -> bool {
        (self.bits & WakeEn0::CLK_SYS_ADC_MASK) != 0
    }

    /// Read **CLK_ADC** `[2:2]`
    #[inline(always)]
    pub const fn clk_adc(&self) -> bool {
        (self.bits & WakeEn0::CLK_ADC_MASK) != 0
    }

    /// Read **CLK_SYS_ACCESSCTRL** `[1:1]`
    #[inline(always)]
    pub const fn clk_sys_accessctrl(&self) -> bool {
        (self.bits & WakeEn0::CLK_SYS_ACCESSCTRL_MASK) != 0
    }

    /// Read **CLK_SYS_CLOCKS** `[0:0]`
    #[inline(always)]
    pub const fn clk_sys_clocks(&self) -> bool {
        (self.bits & WakeEn0::CLK_SYS_CLOCKS_MASK) != 0
    }

    /// Set **CLK_SYS_SIO** `[31:31]`
    #[inline(always)]
    pub const fn with_clk_sys_sio(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | WakeEn0::CLK_SYS_SIO_MASK
            } else {
                self.bits & !WakeEn0::CLK_SYS_SIO_MASK
            }
        }
    }

    /// Set **CLK_SYS_SHA256** `[30:30]`
    #[inline(always)]
    pub const fn with_clk_sys_sha256(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | WakeEn0::CLK_SYS_SHA256_MASK
            } else {
                self.bits & !WakeEn0::CLK_SYS_SHA256_MASK
            }
        }
    }

    /// Set **CLK_SYS_PSM** `[29:29]`
    #[inline(always)]
    pub const fn with_clk_sys_psm(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | WakeEn0::CLK_SYS_PSM_MASK
            } else {
                self.bits & !WakeEn0::CLK_SYS_PSM_MASK
            }
        }
    }

    /// Set **CLK_SYS_ROSC** `[28:28]`
    #[inline(always)]
    pub const fn with_clk_sys_rosc(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | WakeEn0::CLK_SYS_ROSC_MASK
            } else {
                self.bits & !WakeEn0::CLK_SYS_ROSC_MASK
            }
        }
    }

    /// Set **CLK_SYS_ROM** `[27:27]`
    #[inline(always)]
    pub const fn with_clk_sys_rom(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | WakeEn0::CLK_SYS_ROM_MASK
            } else {
                self.bits & !WakeEn0::CLK_SYS_ROM_MASK
            }
        }
    }

    /// Set **CLK_SYS_RESETS** `[26:26]`
    #[inline(always)]
    pub const fn with_clk_sys_resets(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | WakeEn0::CLK_SYS_RESETS_MASK
            } else {
                self.bits & !WakeEn0::CLK_SYS_RESETS_MASK
            }
        }
    }

    /// Set **CLK_SYS_PWM** `[25:25]`
    #[inline(always)]
    pub const fn with_clk_sys_pwm(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | WakeEn0::CLK_SYS_PWM_MASK
            } else {
                self.bits & !WakeEn0::CLK_SYS_PWM_MASK
            }
        }
    }

    /// Set **CLK_SYS_POWMAN** `[24:24]`
    #[inline(always)]
    pub const fn with_clk_sys_powman(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | WakeEn0::CLK_SYS_POWMAN_MASK
            } else {
                self.bits & !WakeEn0::CLK_SYS_POWMAN_MASK
            }
        }
    }

    /// Set **CLK_REF_POWMAN** `[23:23]`
    #[inline(always)]
    pub const fn with_clk_ref_powman(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | WakeEn0::CLK_REF_POWMAN_MASK
            } else {
                self.bits & !WakeEn0::CLK_REF_POWMAN_MASK
            }
        }
    }

    /// Set **CLK_SYS_PLL_USB** `[22:22]`
    #[inline(always)]
    pub const fn with_clk_sys_pll_usb(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | WakeEn0::CLK_SYS_PLL_USB_MASK
            } else {
                self.bits & !WakeEn0::CLK_SYS_PLL_USB_MASK
            }
        }
    }

    /// Set **CLK_SYS_PLL_SYS** `[21:21]`
    #[inline(always)]
    pub const fn with_clk_sys_pll_sys(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | WakeEn0::CLK_SYS_PLL_SYS_MASK
            } else {
                self.bits & !WakeEn0::CLK_SYS_PLL_SYS_MASK
            }
        }
    }

    /// Set **CLK_SYS_PIO2** `[20:20]`
    #[inline(always)]
    pub const fn with_clk_sys_pio2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | WakeEn0::CLK_SYS_PIO2_MASK
            } else {
                self.bits & !WakeEn0::CLK_SYS_PIO2_MASK
            }
        }
    }

    /// Set **CLK_SYS_PIO1** `[19:19]`
    #[inline(always)]
    pub const fn with_clk_sys_pio1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | WakeEn0::CLK_SYS_PIO1_MASK
            } else {
                self.bits & !WakeEn0::CLK_SYS_PIO1_MASK
            }
        }
    }

    /// Set **CLK_SYS_PIO0** `[18:18]`
    #[inline(always)]
    pub const fn with_clk_sys_pio0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | WakeEn0::CLK_SYS_PIO0_MASK
            } else {
                self.bits & !WakeEn0::CLK_SYS_PIO0_MASK
            }
        }
    }

    /// Set **CLK_SYS_PADS** `[17:17]`
    #[inline(always)]
    pub const fn with_clk_sys_pads(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | WakeEn0::CLK_SYS_PADS_MASK
            } else {
                self.bits & !WakeEn0::CLK_SYS_PADS_MASK
            }
        }
    }

    /// Set **CLK_SYS_OTP** `[16:16]`
    #[inline(always)]
    pub const fn with_clk_sys_otp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | WakeEn0::CLK_SYS_OTP_MASK
            } else {
                self.bits & !WakeEn0::CLK_SYS_OTP_MASK
            }
        }
    }

    /// Set **CLK_REF_OTP** `[15:15]`
    #[inline(always)]
    pub const fn with_clk_ref_otp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | WakeEn0::CLK_REF_OTP_MASK
            } else {
                self.bits & !WakeEn0::CLK_REF_OTP_MASK
            }
        }
    }

    /// Set **CLK_SYS_JTAG** `[14:14]`
    #[inline(always)]
    pub const fn with_clk_sys_jtag(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | WakeEn0::CLK_SYS_JTAG_MASK
            } else {
                self.bits & !WakeEn0::CLK_SYS_JTAG_MASK
            }
        }
    }

    /// Set **CLK_SYS_IO** `[13:13]`
    #[inline(always)]
    pub const fn with_clk_sys_io(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | WakeEn0::CLK_SYS_IO_MASK
            } else {
                self.bits & !WakeEn0::CLK_SYS_IO_MASK
            }
        }
    }

    /// Set **CLK_SYS_I2C1** `[12:12]`
    #[inline(always)]
    pub const fn with_clk_sys_i2c1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | WakeEn0::CLK_SYS_I2C1_MASK
            } else {
                self.bits & !WakeEn0::CLK_SYS_I2C1_MASK
            }
        }
    }

    /// Set **CLK_SYS_I2C0** `[11:11]`
    #[inline(always)]
    pub const fn with_clk_sys_i2c0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | WakeEn0::CLK_SYS_I2C0_MASK
            } else {
                self.bits & !WakeEn0::CLK_SYS_I2C0_MASK
            }
        }
    }

    /// Set **CLK_SYS_HSTX** `[10:10]`
    #[inline(always)]
    pub const fn with_clk_sys_hstx(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | WakeEn0::CLK_SYS_HSTX_MASK
            } else {
                self.bits & !WakeEn0::CLK_SYS_HSTX_MASK
            }
        }
    }

    /// Set **CLK_HSTX** `[9:9]`
    #[inline(always)]
    pub const fn with_clk_hstx(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | WakeEn0::CLK_HSTX_MASK
            } else {
                self.bits & !WakeEn0::CLK_HSTX_MASK
            }
        }
    }

    /// Set **CLK_SYS_GLITCH_DETECTOR** `[8:8]`
    #[inline(always)]
    pub const fn with_clk_sys_glitch_detector(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | WakeEn0::CLK_SYS_GLITCH_DETECTOR_MASK
            } else {
                self.bits & !WakeEn0::CLK_SYS_GLITCH_DETECTOR_MASK
            }
        }
    }

    /// Set **CLK_SYS_DMA** `[7:7]`
    #[inline(always)]
    pub const fn with_clk_sys_dma(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | WakeEn0::CLK_SYS_DMA_MASK
            } else {
                self.bits & !WakeEn0::CLK_SYS_DMA_MASK
            }
        }
    }

    /// Set **CLK_SYS_BUSFABRIC** `[6:6]`
    #[inline(always)]
    pub const fn with_clk_sys_busfabric(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | WakeEn0::CLK_SYS_BUSFABRIC_MASK
            } else {
                self.bits & !WakeEn0::CLK_SYS_BUSFABRIC_MASK
            }
        }
    }

    /// Set **CLK_SYS_BUSCTRL** `[5:5]`
    #[inline(always)]
    pub const fn with_clk_sys_busctrl(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | WakeEn0::CLK_SYS_BUSCTRL_MASK
            } else {
                self.bits & !WakeEn0::CLK_SYS_BUSCTRL_MASK
            }
        }
    }

    /// Set **CLK_SYS_BOOTRAM** `[4:4]`
    #[inline(always)]
    pub const fn with_clk_sys_bootram(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | WakeEn0::CLK_SYS_BOOTRAM_MASK
            } else {
                self.bits & !WakeEn0::CLK_SYS_BOOTRAM_MASK
            }
        }
    }

    /// Set **CLK_SYS_ADC** `[3:3]`
    #[inline(always)]
    pub const fn with_clk_sys_adc(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | WakeEn0::CLK_SYS_ADC_MASK
            } else {
                self.bits & !WakeEn0::CLK_SYS_ADC_MASK
            }
        }
    }

    /// Set **CLK_ADC** `[2:2]`
    #[inline(always)]
    pub const fn with_clk_adc(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | WakeEn0::CLK_ADC_MASK
            } else {
                self.bits & !WakeEn0::CLK_ADC_MASK
            }
        }
    }

    /// Set **CLK_SYS_ACCESSCTRL** `[1:1]`
    #[inline(always)]
    pub const fn with_clk_sys_accessctrl(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | WakeEn0::CLK_SYS_ACCESSCTRL_MASK
            } else {
                self.bits & !WakeEn0::CLK_SYS_ACCESSCTRL_MASK
            }
        }
    }

    /// Set **CLK_SYS_CLOCKS** `[0:0]`
    #[inline(always)]
    pub const fn with_clk_sys_clocks(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | WakeEn0::CLK_SYS_CLOCKS_MASK
            } else {
                self.bits & !WakeEn0::CLK_SYS_CLOCKS_MASK
            }
        }
    }
}

impl Default for WakeEn0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl WakeEn0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> WakeEn0Val {
        WakeEn0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: WakeEn0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(WakeEn0Val) -> WakeEn0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// WAKE_EN1 Register
///
/// enable clock in wake mode
///
/// Address offset: 0x00b0
/// Reset value: 0x7fffffff
///
/// ## Fields
///
/// - **CLK_SYS_XOSC** `[30]` (RW)
///
/// - **CLK_SYS_XIP** `[29]` (RW)
///
/// - **CLK_SYS_WATCHDOG** `[28]` (RW)
///
/// - **CLK_USB** `[27]` (RW)
///
/// - **CLK_SYS_USBCTRL** `[26]` (RW)
///
/// - **CLK_SYS_UART1** `[25]` (RW)
///
/// - **CLK_PERI_UART1** `[24]` (RW)
///
/// - **CLK_SYS_UART0** `[23]` (RW)
///
/// - **CLK_PERI_UART0** `[22]` (RW)
///
/// - **CLK_SYS_TRNG** `[21]` (RW)
///
/// - **CLK_SYS_TIMER1** `[20]` (RW)
///
/// - **CLK_SYS_TIMER0** `[19]` (RW)
///
/// - **CLK_SYS_TICKS** `[18]` (RW)
///
/// - **CLK_REF_TICKS** `[17]` (RW)
///
/// - **CLK_SYS_TBMAN** `[16]` (RW)
///
/// - **CLK_SYS_SYSINFO** `[15]` (RW)
///
/// - **CLK_SYS_SYSCFG** `[14]` (RW)
///
/// - **CLK_SYS_SRAM9** `[13]` (RW)
///
/// - **CLK_SYS_SRAM8** `[12]` (RW)
///
/// - **CLK_SYS_SRAM7** `[11]` (RW)
///
/// - **CLK_SYS_SRAM6** `[10]` (RW)
///
/// - **CLK_SYS_SRAM5** `[9]` (RW)
///
/// - **CLK_SYS_SRAM4** `[8]` (RW)
///
/// - **CLK_SYS_SRAM3** `[7]` (RW)
///
/// - **CLK_SYS_SRAM2** `[6]` (RW)
///
/// - **CLK_SYS_SRAM1** `[5]` (RW)
///
/// - **CLK_SYS_SRAM0** `[4]` (RW)
///
/// - **CLK_SYS_SPI1** `[3]` (RW)
///
/// - **CLK_PERI_SPI1** `[2]` (RW)
///
/// - **CLK_SYS_SPI0** `[1]` (RW)
///
/// - **CLK_PERI_SPI0** `[0]` (RW)
#[repr(transparent)]
pub struct WakeEn1 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for WakeEn1 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl WakeEn1 {
    // ======== Field Constants ========

    /// **CLK_SYS_XOSC**
    /// Bits `[30:30]`, mask `0x40000000`
    pub const CLK_SYS_XOSC_OFFSET: u32 = 30;
    pub const CLK_SYS_XOSC_MASK: u32 = 0x40000000;
    pub const CLK_SYS_XOSC: u32 = 1 << 30;

    /// **CLK_SYS_XIP**
    /// Bits `[29:29]`, mask `0x20000000`
    pub const CLK_SYS_XIP_OFFSET: u32 = 29;
    pub const CLK_SYS_XIP_MASK: u32 = 0x20000000;
    pub const CLK_SYS_XIP: u32 = 1 << 29;

    /// **CLK_SYS_WATCHDOG**
    /// Bits `[28:28]`, mask `0x10000000`
    pub const CLK_SYS_WATCHDOG_OFFSET: u32 = 28;
    pub const CLK_SYS_WATCHDOG_MASK: u32 = 0x10000000;
    pub const CLK_SYS_WATCHDOG: u32 = 1 << 28;

    /// **CLK_USB**
    /// Bits `[27:27]`, mask `0x08000000`
    pub const CLK_USB_OFFSET: u32 = 27;
    pub const CLK_USB_MASK: u32 = 0x08000000;
    pub const CLK_USB: u32 = 1 << 27;

    /// **CLK_SYS_USBCTRL**
    /// Bits `[26:26]`, mask `0x04000000`
    pub const CLK_SYS_USBCTRL_OFFSET: u32 = 26;
    pub const CLK_SYS_USBCTRL_MASK: u32 = 0x04000000;
    pub const CLK_SYS_USBCTRL: u32 = 1 << 26;

    /// **CLK_SYS_UART1**
    /// Bits `[25:25]`, mask `0x02000000`
    pub const CLK_SYS_UART1_OFFSET: u32 = 25;
    pub const CLK_SYS_UART1_MASK: u32 = 0x02000000;
    pub const CLK_SYS_UART1: u32 = 1 << 25;

    /// **CLK_PERI_UART1**
    /// Bits `[24:24]`, mask `0x01000000`
    pub const CLK_PERI_UART1_OFFSET: u32 = 24;
    pub const CLK_PERI_UART1_MASK: u32 = 0x01000000;
    pub const CLK_PERI_UART1: u32 = 1 << 24;

    /// **CLK_SYS_UART0**
    /// Bits `[23:23]`, mask `0x00800000`
    pub const CLK_SYS_UART0_OFFSET: u32 = 23;
    pub const CLK_SYS_UART0_MASK: u32 = 0x00800000;
    pub const CLK_SYS_UART0: u32 = 1 << 23;

    /// **CLK_PERI_UART0**
    /// Bits `[22:22]`, mask `0x00400000`
    pub const CLK_PERI_UART0_OFFSET: u32 = 22;
    pub const CLK_PERI_UART0_MASK: u32 = 0x00400000;
    pub const CLK_PERI_UART0: u32 = 1 << 22;

    /// **CLK_SYS_TRNG**
    /// Bits `[21:21]`, mask `0x00200000`
    pub const CLK_SYS_TRNG_OFFSET: u32 = 21;
    pub const CLK_SYS_TRNG_MASK: u32 = 0x00200000;
    pub const CLK_SYS_TRNG: u32 = 1 << 21;

    /// **CLK_SYS_TIMER1**
    /// Bits `[20:20]`, mask `0x00100000`
    pub const CLK_SYS_TIMER1_OFFSET: u32 = 20;
    pub const CLK_SYS_TIMER1_MASK: u32 = 0x00100000;
    pub const CLK_SYS_TIMER1: u32 = 1 << 20;

    /// **CLK_SYS_TIMER0**
    /// Bits `[19:19]`, mask `0x00080000`
    pub const CLK_SYS_TIMER0_OFFSET: u32 = 19;
    pub const CLK_SYS_TIMER0_MASK: u32 = 0x00080000;
    pub const CLK_SYS_TIMER0: u32 = 1 << 19;

    /// **CLK_SYS_TICKS**
    /// Bits `[18:18]`, mask `0x00040000`
    pub const CLK_SYS_TICKS_OFFSET: u32 = 18;
    pub const CLK_SYS_TICKS_MASK: u32 = 0x00040000;
    pub const CLK_SYS_TICKS: u32 = 1 << 18;

    /// **CLK_REF_TICKS**
    /// Bits `[17:17]`, mask `0x00020000`
    pub const CLK_REF_TICKS_OFFSET: u32 = 17;
    pub const CLK_REF_TICKS_MASK: u32 = 0x00020000;
    pub const CLK_REF_TICKS: u32 = 1 << 17;

    /// **CLK_SYS_TBMAN**
    /// Bits `[16:16]`, mask `0x00010000`
    pub const CLK_SYS_TBMAN_OFFSET: u32 = 16;
    pub const CLK_SYS_TBMAN_MASK: u32 = 0x00010000;
    pub const CLK_SYS_TBMAN: u32 = 1 << 16;

    /// **CLK_SYS_SYSINFO**
    /// Bits `[15:15]`, mask `0x00008000`
    pub const CLK_SYS_SYSINFO_OFFSET: u32 = 15;
    pub const CLK_SYS_SYSINFO_MASK: u32 = 0x00008000;
    pub const CLK_SYS_SYSINFO: u32 = 1 << 15;

    /// **CLK_SYS_SYSCFG**
    /// Bits `[14:14]`, mask `0x00004000`
    pub const CLK_SYS_SYSCFG_OFFSET: u32 = 14;
    pub const CLK_SYS_SYSCFG_MASK: u32 = 0x00004000;
    pub const CLK_SYS_SYSCFG: u32 = 1 << 14;

    /// **CLK_SYS_SRAM9**
    /// Bits `[13:13]`, mask `0x00002000`
    pub const CLK_SYS_SRAM9_OFFSET: u32 = 13;
    pub const CLK_SYS_SRAM9_MASK: u32 = 0x00002000;
    pub const CLK_SYS_SRAM9: u32 = 1 << 13;

    /// **CLK_SYS_SRAM8**
    /// Bits `[12:12]`, mask `0x00001000`
    pub const CLK_SYS_SRAM8_OFFSET: u32 = 12;
    pub const CLK_SYS_SRAM8_MASK: u32 = 0x00001000;
    pub const CLK_SYS_SRAM8: u32 = 1 << 12;

    /// **CLK_SYS_SRAM7**
    /// Bits `[11:11]`, mask `0x00000800`
    pub const CLK_SYS_SRAM7_OFFSET: u32 = 11;
    pub const CLK_SYS_SRAM7_MASK: u32 = 0x00000800;
    pub const CLK_SYS_SRAM7: u32 = 1 << 11;

    /// **CLK_SYS_SRAM6**
    /// Bits `[10:10]`, mask `0x00000400`
    pub const CLK_SYS_SRAM6_OFFSET: u32 = 10;
    pub const CLK_SYS_SRAM6_MASK: u32 = 0x00000400;
    pub const CLK_SYS_SRAM6: u32 = 1 << 10;

    /// **CLK_SYS_SRAM5**
    /// Bits `[9:9]`, mask `0x00000200`
    pub const CLK_SYS_SRAM5_OFFSET: u32 = 9;
    pub const CLK_SYS_SRAM5_MASK: u32 = 0x00000200;
    pub const CLK_SYS_SRAM5: u32 = 1 << 9;

    /// **CLK_SYS_SRAM4**
    /// Bits `[8:8]`, mask `0x00000100`
    pub const CLK_SYS_SRAM4_OFFSET: u32 = 8;
    pub const CLK_SYS_SRAM4_MASK: u32 = 0x00000100;
    pub const CLK_SYS_SRAM4: u32 = 1 << 8;

    /// **CLK_SYS_SRAM3**
    /// Bits `[7:7]`, mask `0x00000080`
    pub const CLK_SYS_SRAM3_OFFSET: u32 = 7;
    pub const CLK_SYS_SRAM3_MASK: u32 = 0x00000080;
    pub const CLK_SYS_SRAM3: u32 = 1 << 7;

    /// **CLK_SYS_SRAM2**
    /// Bits `[6:6]`, mask `0x00000040`
    pub const CLK_SYS_SRAM2_OFFSET: u32 = 6;
    pub const CLK_SYS_SRAM2_MASK: u32 = 0x00000040;
    pub const CLK_SYS_SRAM2: u32 = 1 << 6;

    /// **CLK_SYS_SRAM1**
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CLK_SYS_SRAM1_OFFSET: u32 = 5;
    pub const CLK_SYS_SRAM1_MASK: u32 = 0x00000020;
    pub const CLK_SYS_SRAM1: u32 = 1 << 5;

    /// **CLK_SYS_SRAM0**
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CLK_SYS_SRAM0_OFFSET: u32 = 4;
    pub const CLK_SYS_SRAM0_MASK: u32 = 0x00000010;
    pub const CLK_SYS_SRAM0: u32 = 1 << 4;

    /// **CLK_SYS_SPI1**
    /// Bits `[3:3]`, mask `0x00000008`
    pub const CLK_SYS_SPI1_OFFSET: u32 = 3;
    pub const CLK_SYS_SPI1_MASK: u32 = 0x00000008;
    pub const CLK_SYS_SPI1: u32 = 1 << 3;

    /// **CLK_PERI_SPI1**
    /// Bits `[2:2]`, mask `0x00000004`
    pub const CLK_PERI_SPI1_OFFSET: u32 = 2;
    pub const CLK_PERI_SPI1_MASK: u32 = 0x00000004;
    pub const CLK_PERI_SPI1: u32 = 1 << 2;

    /// **CLK_SYS_SPI0**
    /// Bits `[1:1]`, mask `0x00000002`
    pub const CLK_SYS_SPI0_OFFSET: u32 = 1;
    pub const CLK_SYS_SPI0_MASK: u32 = 0x00000002;
    pub const CLK_SYS_SPI0: u32 = 1 << 1;

    /// **CLK_PERI_SPI0**
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CLK_PERI_SPI0_OFFSET: u32 = 0;
    pub const CLK_PERI_SPI0_MASK: u32 = 0x00000001;
    pub const CLK_PERI_SPI0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CLK_SYS_XOSC** `[30]` (RW)
    #[inline(always)]
    pub fn clk_sys_xosc(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_XOSC_MASK) != 0
    }

    /// Set CLK_SYS_XOSC bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_xosc(&self) {
        self.set_bits(Self::CLK_SYS_XOSC_MASK);
    }

    /// Clear CLK_SYS_XOSC bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_xosc(&self) {
        self.clr_bits(Self::CLK_SYS_XOSC_MASK);
    }

    /// **CLK_SYS_XIP** `[29]` (RW)
    #[inline(always)]
    pub fn clk_sys_xip(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_XIP_MASK) != 0
    }

    /// Set CLK_SYS_XIP bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_xip(&self) {
        self.set_bits(Self::CLK_SYS_XIP_MASK);
    }

    /// Clear CLK_SYS_XIP bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_xip(&self) {
        self.clr_bits(Self::CLK_SYS_XIP_MASK);
    }

    /// **CLK_SYS_WATCHDOG** `[28]` (RW)
    #[inline(always)]
    pub fn clk_sys_watchdog(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_WATCHDOG_MASK) != 0
    }

    /// Set CLK_SYS_WATCHDOG bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_watchdog(&self) {
        self.set_bits(Self::CLK_SYS_WATCHDOG_MASK);
    }

    /// Clear CLK_SYS_WATCHDOG bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_watchdog(&self) {
        self.clr_bits(Self::CLK_SYS_WATCHDOG_MASK);
    }

    /// **CLK_USB** `[27]` (RW)
    #[inline(always)]
    pub fn clk_usb(&self) -> bool {
        (self.value.read_bits() & Self::CLK_USB_MASK) != 0
    }

    /// Set CLK_USB bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_usb(&self) {
        self.set_bits(Self::CLK_USB_MASK);
    }

    /// Clear CLK_USB bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_usb(&self) {
        self.clr_bits(Self::CLK_USB_MASK);
    }

    /// **CLK_SYS_USBCTRL** `[26]` (RW)
    #[inline(always)]
    pub fn clk_sys_usbctrl(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_USBCTRL_MASK) != 0
    }

    /// Set CLK_SYS_USBCTRL bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_usbctrl(&self) {
        self.set_bits(Self::CLK_SYS_USBCTRL_MASK);
    }

    /// Clear CLK_SYS_USBCTRL bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_usbctrl(&self) {
        self.clr_bits(Self::CLK_SYS_USBCTRL_MASK);
    }

    /// **CLK_SYS_UART1** `[25]` (RW)
    #[inline(always)]
    pub fn clk_sys_uart1(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_UART1_MASK) != 0
    }

    /// Set CLK_SYS_UART1 bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_uart1(&self) {
        self.set_bits(Self::CLK_SYS_UART1_MASK);
    }

    /// Clear CLK_SYS_UART1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_uart1(&self) {
        self.clr_bits(Self::CLK_SYS_UART1_MASK);
    }

    /// **CLK_PERI_UART1** `[24]` (RW)
    #[inline(always)]
    pub fn clk_peri_uart1(&self) -> bool {
        (self.value.read_bits() & Self::CLK_PERI_UART1_MASK) != 0
    }

    /// Set CLK_PERI_UART1 bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_peri_uart1(&self) {
        self.set_bits(Self::CLK_PERI_UART1_MASK);
    }

    /// Clear CLK_PERI_UART1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_peri_uart1(&self) {
        self.clr_bits(Self::CLK_PERI_UART1_MASK);
    }

    /// **CLK_SYS_UART0** `[23]` (RW)
    #[inline(always)]
    pub fn clk_sys_uart0(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_UART0_MASK) != 0
    }

    /// Set CLK_SYS_UART0 bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_uart0(&self) {
        self.set_bits(Self::CLK_SYS_UART0_MASK);
    }

    /// Clear CLK_SYS_UART0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_uart0(&self) {
        self.clr_bits(Self::CLK_SYS_UART0_MASK);
    }

    /// **CLK_PERI_UART0** `[22]` (RW)
    #[inline(always)]
    pub fn clk_peri_uart0(&self) -> bool {
        (self.value.read_bits() & Self::CLK_PERI_UART0_MASK) != 0
    }

    /// Set CLK_PERI_UART0 bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_peri_uart0(&self) {
        self.set_bits(Self::CLK_PERI_UART0_MASK);
    }

    /// Clear CLK_PERI_UART0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_peri_uart0(&self) {
        self.clr_bits(Self::CLK_PERI_UART0_MASK);
    }

    /// **CLK_SYS_TRNG** `[21]` (RW)
    #[inline(always)]
    pub fn clk_sys_trng(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_TRNG_MASK) != 0
    }

    /// Set CLK_SYS_TRNG bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_trng(&self) {
        self.set_bits(Self::CLK_SYS_TRNG_MASK);
    }

    /// Clear CLK_SYS_TRNG bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_trng(&self) {
        self.clr_bits(Self::CLK_SYS_TRNG_MASK);
    }

    /// **CLK_SYS_TIMER1** `[20]` (RW)
    #[inline(always)]
    pub fn clk_sys_timer1(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_TIMER1_MASK) != 0
    }

    /// Set CLK_SYS_TIMER1 bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_timer1(&self) {
        self.set_bits(Self::CLK_SYS_TIMER1_MASK);
    }

    /// Clear CLK_SYS_TIMER1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_timer1(&self) {
        self.clr_bits(Self::CLK_SYS_TIMER1_MASK);
    }

    /// **CLK_SYS_TIMER0** `[19]` (RW)
    #[inline(always)]
    pub fn clk_sys_timer0(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_TIMER0_MASK) != 0
    }

    /// Set CLK_SYS_TIMER0 bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_timer0(&self) {
        self.set_bits(Self::CLK_SYS_TIMER0_MASK);
    }

    /// Clear CLK_SYS_TIMER0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_timer0(&self) {
        self.clr_bits(Self::CLK_SYS_TIMER0_MASK);
    }

    /// **CLK_SYS_TICKS** `[18]` (RW)
    #[inline(always)]
    pub fn clk_sys_ticks(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_TICKS_MASK) != 0
    }

    /// Set CLK_SYS_TICKS bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_ticks(&self) {
        self.set_bits(Self::CLK_SYS_TICKS_MASK);
    }

    /// Clear CLK_SYS_TICKS bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_ticks(&self) {
        self.clr_bits(Self::CLK_SYS_TICKS_MASK);
    }

    /// **CLK_REF_TICKS** `[17]` (RW)
    #[inline(always)]
    pub fn clk_ref_ticks(&self) -> bool {
        (self.value.read_bits() & Self::CLK_REF_TICKS_MASK) != 0
    }

    /// Set CLK_REF_TICKS bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_ref_ticks(&self) {
        self.set_bits(Self::CLK_REF_TICKS_MASK);
    }

    /// Clear CLK_REF_TICKS bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_ref_ticks(&self) {
        self.clr_bits(Self::CLK_REF_TICKS_MASK);
    }

    /// **CLK_SYS_TBMAN** `[16]` (RW)
    #[inline(always)]
    pub fn clk_sys_tbman(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_TBMAN_MASK) != 0
    }

    /// Set CLK_SYS_TBMAN bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_tbman(&self) {
        self.set_bits(Self::CLK_SYS_TBMAN_MASK);
    }

    /// Clear CLK_SYS_TBMAN bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_tbman(&self) {
        self.clr_bits(Self::CLK_SYS_TBMAN_MASK);
    }

    /// **CLK_SYS_SYSINFO** `[15]` (RW)
    #[inline(always)]
    pub fn clk_sys_sysinfo(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_SYSINFO_MASK) != 0
    }

    /// Set CLK_SYS_SYSINFO bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_sysinfo(&self) {
        self.set_bits(Self::CLK_SYS_SYSINFO_MASK);
    }

    /// Clear CLK_SYS_SYSINFO bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_sysinfo(&self) {
        self.clr_bits(Self::CLK_SYS_SYSINFO_MASK);
    }

    /// **CLK_SYS_SYSCFG** `[14]` (RW)
    #[inline(always)]
    pub fn clk_sys_syscfg(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_SYSCFG_MASK) != 0
    }

    /// Set CLK_SYS_SYSCFG bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_syscfg(&self) {
        self.set_bits(Self::CLK_SYS_SYSCFG_MASK);
    }

    /// Clear CLK_SYS_SYSCFG bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_syscfg(&self) {
        self.clr_bits(Self::CLK_SYS_SYSCFG_MASK);
    }

    /// **CLK_SYS_SRAM9** `[13]` (RW)
    #[inline(always)]
    pub fn clk_sys_sram9(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_SRAM9_MASK) != 0
    }

    /// Set CLK_SYS_SRAM9 bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_sram9(&self) {
        self.set_bits(Self::CLK_SYS_SRAM9_MASK);
    }

    /// Clear CLK_SYS_SRAM9 bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_sram9(&self) {
        self.clr_bits(Self::CLK_SYS_SRAM9_MASK);
    }

    /// **CLK_SYS_SRAM8** `[12]` (RW)
    #[inline(always)]
    pub fn clk_sys_sram8(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_SRAM8_MASK) != 0
    }

    /// Set CLK_SYS_SRAM8 bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_sram8(&self) {
        self.set_bits(Self::CLK_SYS_SRAM8_MASK);
    }

    /// Clear CLK_SYS_SRAM8 bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_sram8(&self) {
        self.clr_bits(Self::CLK_SYS_SRAM8_MASK);
    }

    /// **CLK_SYS_SRAM7** `[11]` (RW)
    #[inline(always)]
    pub fn clk_sys_sram7(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_SRAM7_MASK) != 0
    }

    /// Set CLK_SYS_SRAM7 bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_sram7(&self) {
        self.set_bits(Self::CLK_SYS_SRAM7_MASK);
    }

    /// Clear CLK_SYS_SRAM7 bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_sram7(&self) {
        self.clr_bits(Self::CLK_SYS_SRAM7_MASK);
    }

    /// **CLK_SYS_SRAM6** `[10]` (RW)
    #[inline(always)]
    pub fn clk_sys_sram6(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_SRAM6_MASK) != 0
    }

    /// Set CLK_SYS_SRAM6 bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_sram6(&self) {
        self.set_bits(Self::CLK_SYS_SRAM6_MASK);
    }

    /// Clear CLK_SYS_SRAM6 bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_sram6(&self) {
        self.clr_bits(Self::CLK_SYS_SRAM6_MASK);
    }

    /// **CLK_SYS_SRAM5** `[9]` (RW)
    #[inline(always)]
    pub fn clk_sys_sram5(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_SRAM5_MASK) != 0
    }

    /// Set CLK_SYS_SRAM5 bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_sram5(&self) {
        self.set_bits(Self::CLK_SYS_SRAM5_MASK);
    }

    /// Clear CLK_SYS_SRAM5 bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_sram5(&self) {
        self.clr_bits(Self::CLK_SYS_SRAM5_MASK);
    }

    /// **CLK_SYS_SRAM4** `[8]` (RW)
    #[inline(always)]
    pub fn clk_sys_sram4(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_SRAM4_MASK) != 0
    }

    /// Set CLK_SYS_SRAM4 bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_sram4(&self) {
        self.set_bits(Self::CLK_SYS_SRAM4_MASK);
    }

    /// Clear CLK_SYS_SRAM4 bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_sram4(&self) {
        self.clr_bits(Self::CLK_SYS_SRAM4_MASK);
    }

    /// **CLK_SYS_SRAM3** `[7]` (RW)
    #[inline(always)]
    pub fn clk_sys_sram3(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_SRAM3_MASK) != 0
    }

    /// Set CLK_SYS_SRAM3 bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_sram3(&self) {
        self.set_bits(Self::CLK_SYS_SRAM3_MASK);
    }

    /// Clear CLK_SYS_SRAM3 bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_sram3(&self) {
        self.clr_bits(Self::CLK_SYS_SRAM3_MASK);
    }

    /// **CLK_SYS_SRAM2** `[6]` (RW)
    #[inline(always)]
    pub fn clk_sys_sram2(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_SRAM2_MASK) != 0
    }

    /// Set CLK_SYS_SRAM2 bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_sram2(&self) {
        self.set_bits(Self::CLK_SYS_SRAM2_MASK);
    }

    /// Clear CLK_SYS_SRAM2 bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_sram2(&self) {
        self.clr_bits(Self::CLK_SYS_SRAM2_MASK);
    }

    /// **CLK_SYS_SRAM1** `[5]` (RW)
    #[inline(always)]
    pub fn clk_sys_sram1(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_SRAM1_MASK) != 0
    }

    /// Set CLK_SYS_SRAM1 bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_sram1(&self) {
        self.set_bits(Self::CLK_SYS_SRAM1_MASK);
    }

    /// Clear CLK_SYS_SRAM1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_sram1(&self) {
        self.clr_bits(Self::CLK_SYS_SRAM1_MASK);
    }

    /// **CLK_SYS_SRAM0** `[4]` (RW)
    #[inline(always)]
    pub fn clk_sys_sram0(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_SRAM0_MASK) != 0
    }

    /// Set CLK_SYS_SRAM0 bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_sram0(&self) {
        self.set_bits(Self::CLK_SYS_SRAM0_MASK);
    }

    /// Clear CLK_SYS_SRAM0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_sram0(&self) {
        self.clr_bits(Self::CLK_SYS_SRAM0_MASK);
    }

    /// **CLK_SYS_SPI1** `[3]` (RW)
    #[inline(always)]
    pub fn clk_sys_spi1(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_SPI1_MASK) != 0
    }

    /// Set CLK_SYS_SPI1 bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_spi1(&self) {
        self.set_bits(Self::CLK_SYS_SPI1_MASK);
    }

    /// Clear CLK_SYS_SPI1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_spi1(&self) {
        self.clr_bits(Self::CLK_SYS_SPI1_MASK);
    }

    /// **CLK_PERI_SPI1** `[2]` (RW)
    #[inline(always)]
    pub fn clk_peri_spi1(&self) -> bool {
        (self.value.read_bits() & Self::CLK_PERI_SPI1_MASK) != 0
    }

    /// Set CLK_PERI_SPI1 bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_peri_spi1(&self) {
        self.set_bits(Self::CLK_PERI_SPI1_MASK);
    }

    /// Clear CLK_PERI_SPI1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_peri_spi1(&self) {
        self.clr_bits(Self::CLK_PERI_SPI1_MASK);
    }

    /// **CLK_SYS_SPI0** `[1]` (RW)
    #[inline(always)]
    pub fn clk_sys_spi0(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_SPI0_MASK) != 0
    }

    /// Set CLK_SYS_SPI0 bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_spi0(&self) {
        self.set_bits(Self::CLK_SYS_SPI0_MASK);
    }

    /// Clear CLK_SYS_SPI0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_spi0(&self) {
        self.clr_bits(Self::CLK_SYS_SPI0_MASK);
    }

    /// **CLK_PERI_SPI0** `[0]` (RW)
    #[inline(always)]
    pub fn clk_peri_spi0(&self) -> bool {
        (self.value.read_bits() & Self::CLK_PERI_SPI0_MASK) != 0
    }

    /// Set CLK_PERI_SPI0 bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_peri_spi0(&self) {
        self.set_bits(Self::CLK_PERI_SPI0_MASK);
    }

    /// Clear CLK_PERI_SPI0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_peri_spi0(&self) {
        self.clr_bits(Self::CLK_PERI_SPI0_MASK);
    }
}

/// WAKE_EN1 value type for batch read/modify/write operations
///
/// Use `wakeen1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `wakeen1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct WakeEn1Val {
    bits: u32,
}

impl WakeEn1Val {
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
        Self { bits: 0x7fffffff }
    }

    /// Read **CLK_SYS_XOSC** `[30:30]`
    #[inline(always)]
    pub const fn clk_sys_xosc(&self) -> bool {
        (self.bits & WakeEn1::CLK_SYS_XOSC_MASK) != 0
    }

    /// Read **CLK_SYS_XIP** `[29:29]`
    #[inline(always)]
    pub const fn clk_sys_xip(&self) -> bool {
        (self.bits & WakeEn1::CLK_SYS_XIP_MASK) != 0
    }

    /// Read **CLK_SYS_WATCHDOG** `[28:28]`
    #[inline(always)]
    pub const fn clk_sys_watchdog(&self) -> bool {
        (self.bits & WakeEn1::CLK_SYS_WATCHDOG_MASK) != 0
    }

    /// Read **CLK_USB** `[27:27]`
    #[inline(always)]
    pub const fn clk_usb(&self) -> bool {
        (self.bits & WakeEn1::CLK_USB_MASK) != 0
    }

    /// Read **CLK_SYS_USBCTRL** `[26:26]`
    #[inline(always)]
    pub const fn clk_sys_usbctrl(&self) -> bool {
        (self.bits & WakeEn1::CLK_SYS_USBCTRL_MASK) != 0
    }

    /// Read **CLK_SYS_UART1** `[25:25]`
    #[inline(always)]
    pub const fn clk_sys_uart1(&self) -> bool {
        (self.bits & WakeEn1::CLK_SYS_UART1_MASK) != 0
    }

    /// Read **CLK_PERI_UART1** `[24:24]`
    #[inline(always)]
    pub const fn clk_peri_uart1(&self) -> bool {
        (self.bits & WakeEn1::CLK_PERI_UART1_MASK) != 0
    }

    /// Read **CLK_SYS_UART0** `[23:23]`
    #[inline(always)]
    pub const fn clk_sys_uart0(&self) -> bool {
        (self.bits & WakeEn1::CLK_SYS_UART0_MASK) != 0
    }

    /// Read **CLK_PERI_UART0** `[22:22]`
    #[inline(always)]
    pub const fn clk_peri_uart0(&self) -> bool {
        (self.bits & WakeEn1::CLK_PERI_UART0_MASK) != 0
    }

    /// Read **CLK_SYS_TRNG** `[21:21]`
    #[inline(always)]
    pub const fn clk_sys_trng(&self) -> bool {
        (self.bits & WakeEn1::CLK_SYS_TRNG_MASK) != 0
    }

    /// Read **CLK_SYS_TIMER1** `[20:20]`
    #[inline(always)]
    pub const fn clk_sys_timer1(&self) -> bool {
        (self.bits & WakeEn1::CLK_SYS_TIMER1_MASK) != 0
    }

    /// Read **CLK_SYS_TIMER0** `[19:19]`
    #[inline(always)]
    pub const fn clk_sys_timer0(&self) -> bool {
        (self.bits & WakeEn1::CLK_SYS_TIMER0_MASK) != 0
    }

    /// Read **CLK_SYS_TICKS** `[18:18]`
    #[inline(always)]
    pub const fn clk_sys_ticks(&self) -> bool {
        (self.bits & WakeEn1::CLK_SYS_TICKS_MASK) != 0
    }

    /// Read **CLK_REF_TICKS** `[17:17]`
    #[inline(always)]
    pub const fn clk_ref_ticks(&self) -> bool {
        (self.bits & WakeEn1::CLK_REF_TICKS_MASK) != 0
    }

    /// Read **CLK_SYS_TBMAN** `[16:16]`
    #[inline(always)]
    pub const fn clk_sys_tbman(&self) -> bool {
        (self.bits & WakeEn1::CLK_SYS_TBMAN_MASK) != 0
    }

    /// Read **CLK_SYS_SYSINFO** `[15:15]`
    #[inline(always)]
    pub const fn clk_sys_sysinfo(&self) -> bool {
        (self.bits & WakeEn1::CLK_SYS_SYSINFO_MASK) != 0
    }

    /// Read **CLK_SYS_SYSCFG** `[14:14]`
    #[inline(always)]
    pub const fn clk_sys_syscfg(&self) -> bool {
        (self.bits & WakeEn1::CLK_SYS_SYSCFG_MASK) != 0
    }

    /// Read **CLK_SYS_SRAM9** `[13:13]`
    #[inline(always)]
    pub const fn clk_sys_sram9(&self) -> bool {
        (self.bits & WakeEn1::CLK_SYS_SRAM9_MASK) != 0
    }

    /// Read **CLK_SYS_SRAM8** `[12:12]`
    #[inline(always)]
    pub const fn clk_sys_sram8(&self) -> bool {
        (self.bits & WakeEn1::CLK_SYS_SRAM8_MASK) != 0
    }

    /// Read **CLK_SYS_SRAM7** `[11:11]`
    #[inline(always)]
    pub const fn clk_sys_sram7(&self) -> bool {
        (self.bits & WakeEn1::CLK_SYS_SRAM7_MASK) != 0
    }

    /// Read **CLK_SYS_SRAM6** `[10:10]`
    #[inline(always)]
    pub const fn clk_sys_sram6(&self) -> bool {
        (self.bits & WakeEn1::CLK_SYS_SRAM6_MASK) != 0
    }

    /// Read **CLK_SYS_SRAM5** `[9:9]`
    #[inline(always)]
    pub const fn clk_sys_sram5(&self) -> bool {
        (self.bits & WakeEn1::CLK_SYS_SRAM5_MASK) != 0
    }

    /// Read **CLK_SYS_SRAM4** `[8:8]`
    #[inline(always)]
    pub const fn clk_sys_sram4(&self) -> bool {
        (self.bits & WakeEn1::CLK_SYS_SRAM4_MASK) != 0
    }

    /// Read **CLK_SYS_SRAM3** `[7:7]`
    #[inline(always)]
    pub const fn clk_sys_sram3(&self) -> bool {
        (self.bits & WakeEn1::CLK_SYS_SRAM3_MASK) != 0
    }

    /// Read **CLK_SYS_SRAM2** `[6:6]`
    #[inline(always)]
    pub const fn clk_sys_sram2(&self) -> bool {
        (self.bits & WakeEn1::CLK_SYS_SRAM2_MASK) != 0
    }

    /// Read **CLK_SYS_SRAM1** `[5:5]`
    #[inline(always)]
    pub const fn clk_sys_sram1(&self) -> bool {
        (self.bits & WakeEn1::CLK_SYS_SRAM1_MASK) != 0
    }

    /// Read **CLK_SYS_SRAM0** `[4:4]`
    #[inline(always)]
    pub const fn clk_sys_sram0(&self) -> bool {
        (self.bits & WakeEn1::CLK_SYS_SRAM0_MASK) != 0
    }

    /// Read **CLK_SYS_SPI1** `[3:3]`
    #[inline(always)]
    pub const fn clk_sys_spi1(&self) -> bool {
        (self.bits & WakeEn1::CLK_SYS_SPI1_MASK) != 0
    }

    /// Read **CLK_PERI_SPI1** `[2:2]`
    #[inline(always)]
    pub const fn clk_peri_spi1(&self) -> bool {
        (self.bits & WakeEn1::CLK_PERI_SPI1_MASK) != 0
    }

    /// Read **CLK_SYS_SPI0** `[1:1]`
    #[inline(always)]
    pub const fn clk_sys_spi0(&self) -> bool {
        (self.bits & WakeEn1::CLK_SYS_SPI0_MASK) != 0
    }

    /// Read **CLK_PERI_SPI0** `[0:0]`
    #[inline(always)]
    pub const fn clk_peri_spi0(&self) -> bool {
        (self.bits & WakeEn1::CLK_PERI_SPI0_MASK) != 0
    }

    /// Set **CLK_SYS_XOSC** `[30:30]`
    #[inline(always)]
    pub const fn with_clk_sys_xosc(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | WakeEn1::CLK_SYS_XOSC_MASK
            } else {
                self.bits & !WakeEn1::CLK_SYS_XOSC_MASK
            }
        }
    }

    /// Set **CLK_SYS_XIP** `[29:29]`
    #[inline(always)]
    pub const fn with_clk_sys_xip(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | WakeEn1::CLK_SYS_XIP_MASK
            } else {
                self.bits & !WakeEn1::CLK_SYS_XIP_MASK
            }
        }
    }

    /// Set **CLK_SYS_WATCHDOG** `[28:28]`
    #[inline(always)]
    pub const fn with_clk_sys_watchdog(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | WakeEn1::CLK_SYS_WATCHDOG_MASK
            } else {
                self.bits & !WakeEn1::CLK_SYS_WATCHDOG_MASK
            }
        }
    }

    /// Set **CLK_USB** `[27:27]`
    #[inline(always)]
    pub const fn with_clk_usb(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | WakeEn1::CLK_USB_MASK
            } else {
                self.bits & !WakeEn1::CLK_USB_MASK
            }
        }
    }

    /// Set **CLK_SYS_USBCTRL** `[26:26]`
    #[inline(always)]
    pub const fn with_clk_sys_usbctrl(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | WakeEn1::CLK_SYS_USBCTRL_MASK
            } else {
                self.bits & !WakeEn1::CLK_SYS_USBCTRL_MASK
            }
        }
    }

    /// Set **CLK_SYS_UART1** `[25:25]`
    #[inline(always)]
    pub const fn with_clk_sys_uart1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | WakeEn1::CLK_SYS_UART1_MASK
            } else {
                self.bits & !WakeEn1::CLK_SYS_UART1_MASK
            }
        }
    }

    /// Set **CLK_PERI_UART1** `[24:24]`
    #[inline(always)]
    pub const fn with_clk_peri_uart1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | WakeEn1::CLK_PERI_UART1_MASK
            } else {
                self.bits & !WakeEn1::CLK_PERI_UART1_MASK
            }
        }
    }

    /// Set **CLK_SYS_UART0** `[23:23]`
    #[inline(always)]
    pub const fn with_clk_sys_uart0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | WakeEn1::CLK_SYS_UART0_MASK
            } else {
                self.bits & !WakeEn1::CLK_SYS_UART0_MASK
            }
        }
    }

    /// Set **CLK_PERI_UART0** `[22:22]`
    #[inline(always)]
    pub const fn with_clk_peri_uart0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | WakeEn1::CLK_PERI_UART0_MASK
            } else {
                self.bits & !WakeEn1::CLK_PERI_UART0_MASK
            }
        }
    }

    /// Set **CLK_SYS_TRNG** `[21:21]`
    #[inline(always)]
    pub const fn with_clk_sys_trng(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | WakeEn1::CLK_SYS_TRNG_MASK
            } else {
                self.bits & !WakeEn1::CLK_SYS_TRNG_MASK
            }
        }
    }

    /// Set **CLK_SYS_TIMER1** `[20:20]`
    #[inline(always)]
    pub const fn with_clk_sys_timer1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | WakeEn1::CLK_SYS_TIMER1_MASK
            } else {
                self.bits & !WakeEn1::CLK_SYS_TIMER1_MASK
            }
        }
    }

    /// Set **CLK_SYS_TIMER0** `[19:19]`
    #[inline(always)]
    pub const fn with_clk_sys_timer0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | WakeEn1::CLK_SYS_TIMER0_MASK
            } else {
                self.bits & !WakeEn1::CLK_SYS_TIMER0_MASK
            }
        }
    }

    /// Set **CLK_SYS_TICKS** `[18:18]`
    #[inline(always)]
    pub const fn with_clk_sys_ticks(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | WakeEn1::CLK_SYS_TICKS_MASK
            } else {
                self.bits & !WakeEn1::CLK_SYS_TICKS_MASK
            }
        }
    }

    /// Set **CLK_REF_TICKS** `[17:17]`
    #[inline(always)]
    pub const fn with_clk_ref_ticks(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | WakeEn1::CLK_REF_TICKS_MASK
            } else {
                self.bits & !WakeEn1::CLK_REF_TICKS_MASK
            }
        }
    }

    /// Set **CLK_SYS_TBMAN** `[16:16]`
    #[inline(always)]
    pub const fn with_clk_sys_tbman(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | WakeEn1::CLK_SYS_TBMAN_MASK
            } else {
                self.bits & !WakeEn1::CLK_SYS_TBMAN_MASK
            }
        }
    }

    /// Set **CLK_SYS_SYSINFO** `[15:15]`
    #[inline(always)]
    pub const fn with_clk_sys_sysinfo(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | WakeEn1::CLK_SYS_SYSINFO_MASK
            } else {
                self.bits & !WakeEn1::CLK_SYS_SYSINFO_MASK
            }
        }
    }

    /// Set **CLK_SYS_SYSCFG** `[14:14]`
    #[inline(always)]
    pub const fn with_clk_sys_syscfg(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | WakeEn1::CLK_SYS_SYSCFG_MASK
            } else {
                self.bits & !WakeEn1::CLK_SYS_SYSCFG_MASK
            }
        }
    }

    /// Set **CLK_SYS_SRAM9** `[13:13]`
    #[inline(always)]
    pub const fn with_clk_sys_sram9(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | WakeEn1::CLK_SYS_SRAM9_MASK
            } else {
                self.bits & !WakeEn1::CLK_SYS_SRAM9_MASK
            }
        }
    }

    /// Set **CLK_SYS_SRAM8** `[12:12]`
    #[inline(always)]
    pub const fn with_clk_sys_sram8(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | WakeEn1::CLK_SYS_SRAM8_MASK
            } else {
                self.bits & !WakeEn1::CLK_SYS_SRAM8_MASK
            }
        }
    }

    /// Set **CLK_SYS_SRAM7** `[11:11]`
    #[inline(always)]
    pub const fn with_clk_sys_sram7(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | WakeEn1::CLK_SYS_SRAM7_MASK
            } else {
                self.bits & !WakeEn1::CLK_SYS_SRAM7_MASK
            }
        }
    }

    /// Set **CLK_SYS_SRAM6** `[10:10]`
    #[inline(always)]
    pub const fn with_clk_sys_sram6(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | WakeEn1::CLK_SYS_SRAM6_MASK
            } else {
                self.bits & !WakeEn1::CLK_SYS_SRAM6_MASK
            }
        }
    }

    /// Set **CLK_SYS_SRAM5** `[9:9]`
    #[inline(always)]
    pub const fn with_clk_sys_sram5(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | WakeEn1::CLK_SYS_SRAM5_MASK
            } else {
                self.bits & !WakeEn1::CLK_SYS_SRAM5_MASK
            }
        }
    }

    /// Set **CLK_SYS_SRAM4** `[8:8]`
    #[inline(always)]
    pub const fn with_clk_sys_sram4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | WakeEn1::CLK_SYS_SRAM4_MASK
            } else {
                self.bits & !WakeEn1::CLK_SYS_SRAM4_MASK
            }
        }
    }

    /// Set **CLK_SYS_SRAM3** `[7:7]`
    #[inline(always)]
    pub const fn with_clk_sys_sram3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | WakeEn1::CLK_SYS_SRAM3_MASK
            } else {
                self.bits & !WakeEn1::CLK_SYS_SRAM3_MASK
            }
        }
    }

    /// Set **CLK_SYS_SRAM2** `[6:6]`
    #[inline(always)]
    pub const fn with_clk_sys_sram2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | WakeEn1::CLK_SYS_SRAM2_MASK
            } else {
                self.bits & !WakeEn1::CLK_SYS_SRAM2_MASK
            }
        }
    }

    /// Set **CLK_SYS_SRAM1** `[5:5]`
    #[inline(always)]
    pub const fn with_clk_sys_sram1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | WakeEn1::CLK_SYS_SRAM1_MASK
            } else {
                self.bits & !WakeEn1::CLK_SYS_SRAM1_MASK
            }
        }
    }

    /// Set **CLK_SYS_SRAM0** `[4:4]`
    #[inline(always)]
    pub const fn with_clk_sys_sram0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | WakeEn1::CLK_SYS_SRAM0_MASK
            } else {
                self.bits & !WakeEn1::CLK_SYS_SRAM0_MASK
            }
        }
    }

    /// Set **CLK_SYS_SPI1** `[3:3]`
    #[inline(always)]
    pub const fn with_clk_sys_spi1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | WakeEn1::CLK_SYS_SPI1_MASK
            } else {
                self.bits & !WakeEn1::CLK_SYS_SPI1_MASK
            }
        }
    }

    /// Set **CLK_PERI_SPI1** `[2:2]`
    #[inline(always)]
    pub const fn with_clk_peri_spi1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | WakeEn1::CLK_PERI_SPI1_MASK
            } else {
                self.bits & !WakeEn1::CLK_PERI_SPI1_MASK
            }
        }
    }

    /// Set **CLK_SYS_SPI0** `[1:1]`
    #[inline(always)]
    pub const fn with_clk_sys_spi0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | WakeEn1::CLK_SYS_SPI0_MASK
            } else {
                self.bits & !WakeEn1::CLK_SYS_SPI0_MASK
            }
        }
    }

    /// Set **CLK_PERI_SPI0** `[0:0]`
    #[inline(always)]
    pub const fn with_clk_peri_spi0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | WakeEn1::CLK_PERI_SPI0_MASK
            } else {
                self.bits & !WakeEn1::CLK_PERI_SPI0_MASK
            }
        }
    }
}

impl Default for WakeEn1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl WakeEn1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> WakeEn1Val {
        WakeEn1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: WakeEn1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(WakeEn1Val) -> WakeEn1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SLEEP_EN0 Register
///
/// enable clock in sleep mode
///
/// Address offset: 0x00b4
/// Reset value: 0xffffffff
///
/// ## Fields
///
/// - **CLK_SYS_SIO** `[31]` (RW)
///
/// - **CLK_SYS_SHA256** `[30]` (RW)
///
/// - **CLK_SYS_PSM** `[29]` (RW)
///
/// - **CLK_SYS_ROSC** `[28]` (RW)
///
/// - **CLK_SYS_ROM** `[27]` (RW)
///
/// - **CLK_SYS_RESETS** `[26]` (RW)
///
/// - **CLK_SYS_PWM** `[25]` (RW)
///
/// - **CLK_SYS_POWMAN** `[24]` (RW)
///
/// - **CLK_REF_POWMAN** `[23]` (RW)
///
/// - **CLK_SYS_PLL_USB** `[22]` (RW)
///
/// - **CLK_SYS_PLL_SYS** `[21]` (RW)
///
/// - **CLK_SYS_PIO2** `[20]` (RW)
///
/// - **CLK_SYS_PIO1** `[19]` (RW)
///
/// - **CLK_SYS_PIO0** `[18]` (RW)
///
/// - **CLK_SYS_PADS** `[17]` (RW)
///
/// - **CLK_SYS_OTP** `[16]` (RW)
///
/// - **CLK_REF_OTP** `[15]` (RW)
///
/// - **CLK_SYS_JTAG** `[14]` (RW)
///
/// - **CLK_SYS_IO** `[13]` (RW)
///
/// - **CLK_SYS_I2C1** `[12]` (RW)
///
/// - **CLK_SYS_I2C0** `[11]` (RW)
///
/// - **CLK_SYS_HSTX** `[10]` (RW)
///
/// - **CLK_HSTX** `[9]` (RW)
///
/// - **CLK_SYS_GLITCH_DETECTOR** `[8]` (RW)
///
/// - **CLK_SYS_DMA** `[7]` (RW)
///
/// - **CLK_SYS_BUSFABRIC** `[6]` (RW)
///
/// - **CLK_SYS_BUSCTRL** `[5]` (RW)
///
/// - **CLK_SYS_BOOTRAM** `[4]` (RW)
///
/// - **CLK_SYS_ADC** `[3]` (RW)
///
/// - **CLK_ADC** `[2]` (RW)
///
/// - **CLK_SYS_ACCESSCTRL** `[1]` (RW)
///
/// - **CLK_SYS_CLOCKS** `[0]` (RW)
#[repr(transparent)]
pub struct SleepEn0 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for SleepEn0 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SleepEn0 {
    // ======== Field Constants ========

    /// **CLK_SYS_SIO**
    /// Bits `[31:31]`, mask `0x80000000`
    pub const CLK_SYS_SIO_OFFSET: u32 = 31;
    pub const CLK_SYS_SIO_MASK: u32 = 0x80000000;
    pub const CLK_SYS_SIO: u32 = 1 << 31;

    /// **CLK_SYS_SHA256**
    /// Bits `[30:30]`, mask `0x40000000`
    pub const CLK_SYS_SHA256_OFFSET: u32 = 30;
    pub const CLK_SYS_SHA256_MASK: u32 = 0x40000000;
    pub const CLK_SYS_SHA256: u32 = 1 << 30;

    /// **CLK_SYS_PSM**
    /// Bits `[29:29]`, mask `0x20000000`
    pub const CLK_SYS_PSM_OFFSET: u32 = 29;
    pub const CLK_SYS_PSM_MASK: u32 = 0x20000000;
    pub const CLK_SYS_PSM: u32 = 1 << 29;

    /// **CLK_SYS_ROSC**
    /// Bits `[28:28]`, mask `0x10000000`
    pub const CLK_SYS_ROSC_OFFSET: u32 = 28;
    pub const CLK_SYS_ROSC_MASK: u32 = 0x10000000;
    pub const CLK_SYS_ROSC: u32 = 1 << 28;

    /// **CLK_SYS_ROM**
    /// Bits `[27:27]`, mask `0x08000000`
    pub const CLK_SYS_ROM_OFFSET: u32 = 27;
    pub const CLK_SYS_ROM_MASK: u32 = 0x08000000;
    pub const CLK_SYS_ROM: u32 = 1 << 27;

    /// **CLK_SYS_RESETS**
    /// Bits `[26:26]`, mask `0x04000000`
    pub const CLK_SYS_RESETS_OFFSET: u32 = 26;
    pub const CLK_SYS_RESETS_MASK: u32 = 0x04000000;
    pub const CLK_SYS_RESETS: u32 = 1 << 26;

    /// **CLK_SYS_PWM**
    /// Bits `[25:25]`, mask `0x02000000`
    pub const CLK_SYS_PWM_OFFSET: u32 = 25;
    pub const CLK_SYS_PWM_MASK: u32 = 0x02000000;
    pub const CLK_SYS_PWM: u32 = 1 << 25;

    /// **CLK_SYS_POWMAN**
    /// Bits `[24:24]`, mask `0x01000000`
    pub const CLK_SYS_POWMAN_OFFSET: u32 = 24;
    pub const CLK_SYS_POWMAN_MASK: u32 = 0x01000000;
    pub const CLK_SYS_POWMAN: u32 = 1 << 24;

    /// **CLK_REF_POWMAN**
    /// Bits `[23:23]`, mask `0x00800000`
    pub const CLK_REF_POWMAN_OFFSET: u32 = 23;
    pub const CLK_REF_POWMAN_MASK: u32 = 0x00800000;
    pub const CLK_REF_POWMAN: u32 = 1 << 23;

    /// **CLK_SYS_PLL_USB**
    /// Bits `[22:22]`, mask `0x00400000`
    pub const CLK_SYS_PLL_USB_OFFSET: u32 = 22;
    pub const CLK_SYS_PLL_USB_MASK: u32 = 0x00400000;
    pub const CLK_SYS_PLL_USB: u32 = 1 << 22;

    /// **CLK_SYS_PLL_SYS**
    /// Bits `[21:21]`, mask `0x00200000`
    pub const CLK_SYS_PLL_SYS_OFFSET: u32 = 21;
    pub const CLK_SYS_PLL_SYS_MASK: u32 = 0x00200000;
    pub const CLK_SYS_PLL_SYS: u32 = 1 << 21;

    /// **CLK_SYS_PIO2**
    /// Bits `[20:20]`, mask `0x00100000`
    pub const CLK_SYS_PIO2_OFFSET: u32 = 20;
    pub const CLK_SYS_PIO2_MASK: u32 = 0x00100000;
    pub const CLK_SYS_PIO2: u32 = 1 << 20;

    /// **CLK_SYS_PIO1**
    /// Bits `[19:19]`, mask `0x00080000`
    pub const CLK_SYS_PIO1_OFFSET: u32 = 19;
    pub const CLK_SYS_PIO1_MASK: u32 = 0x00080000;
    pub const CLK_SYS_PIO1: u32 = 1 << 19;

    /// **CLK_SYS_PIO0**
    /// Bits `[18:18]`, mask `0x00040000`
    pub const CLK_SYS_PIO0_OFFSET: u32 = 18;
    pub const CLK_SYS_PIO0_MASK: u32 = 0x00040000;
    pub const CLK_SYS_PIO0: u32 = 1 << 18;

    /// **CLK_SYS_PADS**
    /// Bits `[17:17]`, mask `0x00020000`
    pub const CLK_SYS_PADS_OFFSET: u32 = 17;
    pub const CLK_SYS_PADS_MASK: u32 = 0x00020000;
    pub const CLK_SYS_PADS: u32 = 1 << 17;

    /// **CLK_SYS_OTP**
    /// Bits `[16:16]`, mask `0x00010000`
    pub const CLK_SYS_OTP_OFFSET: u32 = 16;
    pub const CLK_SYS_OTP_MASK: u32 = 0x00010000;
    pub const CLK_SYS_OTP: u32 = 1 << 16;

    /// **CLK_REF_OTP**
    /// Bits `[15:15]`, mask `0x00008000`
    pub const CLK_REF_OTP_OFFSET: u32 = 15;
    pub const CLK_REF_OTP_MASK: u32 = 0x00008000;
    pub const CLK_REF_OTP: u32 = 1 << 15;

    /// **CLK_SYS_JTAG**
    /// Bits `[14:14]`, mask `0x00004000`
    pub const CLK_SYS_JTAG_OFFSET: u32 = 14;
    pub const CLK_SYS_JTAG_MASK: u32 = 0x00004000;
    pub const CLK_SYS_JTAG: u32 = 1 << 14;

    /// **CLK_SYS_IO**
    /// Bits `[13:13]`, mask `0x00002000`
    pub const CLK_SYS_IO_OFFSET: u32 = 13;
    pub const CLK_SYS_IO_MASK: u32 = 0x00002000;
    pub const CLK_SYS_IO: u32 = 1 << 13;

    /// **CLK_SYS_I2C1**
    /// Bits `[12:12]`, mask `0x00001000`
    pub const CLK_SYS_I2C1_OFFSET: u32 = 12;
    pub const CLK_SYS_I2C1_MASK: u32 = 0x00001000;
    pub const CLK_SYS_I2C1: u32 = 1 << 12;

    /// **CLK_SYS_I2C0**
    /// Bits `[11:11]`, mask `0x00000800`
    pub const CLK_SYS_I2C0_OFFSET: u32 = 11;
    pub const CLK_SYS_I2C0_MASK: u32 = 0x00000800;
    pub const CLK_SYS_I2C0: u32 = 1 << 11;

    /// **CLK_SYS_HSTX**
    /// Bits `[10:10]`, mask `0x00000400`
    pub const CLK_SYS_HSTX_OFFSET: u32 = 10;
    pub const CLK_SYS_HSTX_MASK: u32 = 0x00000400;
    pub const CLK_SYS_HSTX: u32 = 1 << 10;

    /// **CLK_HSTX**
    /// Bits `[9:9]`, mask `0x00000200`
    pub const CLK_HSTX_OFFSET: u32 = 9;
    pub const CLK_HSTX_MASK: u32 = 0x00000200;
    pub const CLK_HSTX: u32 = 1 << 9;

    /// **CLK_SYS_GLITCH_DETECTOR**
    /// Bits `[8:8]`, mask `0x00000100`
    pub const CLK_SYS_GLITCH_DETECTOR_OFFSET: u32 = 8;
    pub const CLK_SYS_GLITCH_DETECTOR_MASK: u32 = 0x00000100;
    pub const CLK_SYS_GLITCH_DETECTOR: u32 = 1 << 8;

    /// **CLK_SYS_DMA**
    /// Bits `[7:7]`, mask `0x00000080`
    pub const CLK_SYS_DMA_OFFSET: u32 = 7;
    pub const CLK_SYS_DMA_MASK: u32 = 0x00000080;
    pub const CLK_SYS_DMA: u32 = 1 << 7;

    /// **CLK_SYS_BUSFABRIC**
    /// Bits `[6:6]`, mask `0x00000040`
    pub const CLK_SYS_BUSFABRIC_OFFSET: u32 = 6;
    pub const CLK_SYS_BUSFABRIC_MASK: u32 = 0x00000040;
    pub const CLK_SYS_BUSFABRIC: u32 = 1 << 6;

    /// **CLK_SYS_BUSCTRL**
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CLK_SYS_BUSCTRL_OFFSET: u32 = 5;
    pub const CLK_SYS_BUSCTRL_MASK: u32 = 0x00000020;
    pub const CLK_SYS_BUSCTRL: u32 = 1 << 5;

    /// **CLK_SYS_BOOTRAM**
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CLK_SYS_BOOTRAM_OFFSET: u32 = 4;
    pub const CLK_SYS_BOOTRAM_MASK: u32 = 0x00000010;
    pub const CLK_SYS_BOOTRAM: u32 = 1 << 4;

    /// **CLK_SYS_ADC**
    /// Bits `[3:3]`, mask `0x00000008`
    pub const CLK_SYS_ADC_OFFSET: u32 = 3;
    pub const CLK_SYS_ADC_MASK: u32 = 0x00000008;
    pub const CLK_SYS_ADC: u32 = 1 << 3;

    /// **CLK_ADC**
    /// Bits `[2:2]`, mask `0x00000004`
    pub const CLK_ADC_OFFSET: u32 = 2;
    pub const CLK_ADC_MASK: u32 = 0x00000004;
    pub const CLK_ADC: u32 = 1 << 2;

    /// **CLK_SYS_ACCESSCTRL**
    /// Bits `[1:1]`, mask `0x00000002`
    pub const CLK_SYS_ACCESSCTRL_OFFSET: u32 = 1;
    pub const CLK_SYS_ACCESSCTRL_MASK: u32 = 0x00000002;
    pub const CLK_SYS_ACCESSCTRL: u32 = 1 << 1;

    /// **CLK_SYS_CLOCKS**
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CLK_SYS_CLOCKS_OFFSET: u32 = 0;
    pub const CLK_SYS_CLOCKS_MASK: u32 = 0x00000001;
    pub const CLK_SYS_CLOCKS: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CLK_SYS_SIO** `[31]` (RW)
    #[inline(always)]
    pub fn clk_sys_sio(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_SIO_MASK) != 0
    }

    /// Set CLK_SYS_SIO bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_sio(&self) {
        self.set_bits(Self::CLK_SYS_SIO_MASK);
    }

    /// Clear CLK_SYS_SIO bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_sio(&self) {
        self.clr_bits(Self::CLK_SYS_SIO_MASK);
    }

    /// **CLK_SYS_SHA256** `[30]` (RW)
    #[inline(always)]
    pub fn clk_sys_sha256(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_SHA256_MASK) != 0
    }

    /// Set CLK_SYS_SHA256 bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_sha256(&self) {
        self.set_bits(Self::CLK_SYS_SHA256_MASK);
    }

    /// Clear CLK_SYS_SHA256 bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_sha256(&self) {
        self.clr_bits(Self::CLK_SYS_SHA256_MASK);
    }

    /// **CLK_SYS_PSM** `[29]` (RW)
    #[inline(always)]
    pub fn clk_sys_psm(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_PSM_MASK) != 0
    }

    /// Set CLK_SYS_PSM bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_psm(&self) {
        self.set_bits(Self::CLK_SYS_PSM_MASK);
    }

    /// Clear CLK_SYS_PSM bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_psm(&self) {
        self.clr_bits(Self::CLK_SYS_PSM_MASK);
    }

    /// **CLK_SYS_ROSC** `[28]` (RW)
    #[inline(always)]
    pub fn clk_sys_rosc(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_ROSC_MASK) != 0
    }

    /// Set CLK_SYS_ROSC bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_rosc(&self) {
        self.set_bits(Self::CLK_SYS_ROSC_MASK);
    }

    /// Clear CLK_SYS_ROSC bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_rosc(&self) {
        self.clr_bits(Self::CLK_SYS_ROSC_MASK);
    }

    /// **CLK_SYS_ROM** `[27]` (RW)
    #[inline(always)]
    pub fn clk_sys_rom(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_ROM_MASK) != 0
    }

    /// Set CLK_SYS_ROM bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_rom(&self) {
        self.set_bits(Self::CLK_SYS_ROM_MASK);
    }

    /// Clear CLK_SYS_ROM bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_rom(&self) {
        self.clr_bits(Self::CLK_SYS_ROM_MASK);
    }

    /// **CLK_SYS_RESETS** `[26]` (RW)
    #[inline(always)]
    pub fn clk_sys_resets(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_RESETS_MASK) != 0
    }

    /// Set CLK_SYS_RESETS bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_resets(&self) {
        self.set_bits(Self::CLK_SYS_RESETS_MASK);
    }

    /// Clear CLK_SYS_RESETS bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_resets(&self) {
        self.clr_bits(Self::CLK_SYS_RESETS_MASK);
    }

    /// **CLK_SYS_PWM** `[25]` (RW)
    #[inline(always)]
    pub fn clk_sys_pwm(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_PWM_MASK) != 0
    }

    /// Set CLK_SYS_PWM bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_pwm(&self) {
        self.set_bits(Self::CLK_SYS_PWM_MASK);
    }

    /// Clear CLK_SYS_PWM bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_pwm(&self) {
        self.clr_bits(Self::CLK_SYS_PWM_MASK);
    }

    /// **CLK_SYS_POWMAN** `[24]` (RW)
    #[inline(always)]
    pub fn clk_sys_powman(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_POWMAN_MASK) != 0
    }

    /// Set CLK_SYS_POWMAN bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_powman(&self) {
        self.set_bits(Self::CLK_SYS_POWMAN_MASK);
    }

    /// Clear CLK_SYS_POWMAN bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_powman(&self) {
        self.clr_bits(Self::CLK_SYS_POWMAN_MASK);
    }

    /// **CLK_REF_POWMAN** `[23]` (RW)
    #[inline(always)]
    pub fn clk_ref_powman(&self) -> bool {
        (self.value.read_bits() & Self::CLK_REF_POWMAN_MASK) != 0
    }

    /// Set CLK_REF_POWMAN bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_ref_powman(&self) {
        self.set_bits(Self::CLK_REF_POWMAN_MASK);
    }

    /// Clear CLK_REF_POWMAN bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_ref_powman(&self) {
        self.clr_bits(Self::CLK_REF_POWMAN_MASK);
    }

    /// **CLK_SYS_PLL_USB** `[22]` (RW)
    #[inline(always)]
    pub fn clk_sys_pll_usb(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_PLL_USB_MASK) != 0
    }

    /// Set CLK_SYS_PLL_USB bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_pll_usb(&self) {
        self.set_bits(Self::CLK_SYS_PLL_USB_MASK);
    }

    /// Clear CLK_SYS_PLL_USB bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_pll_usb(&self) {
        self.clr_bits(Self::CLK_SYS_PLL_USB_MASK);
    }

    /// **CLK_SYS_PLL_SYS** `[21]` (RW)
    #[inline(always)]
    pub fn clk_sys_pll_sys(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_PLL_SYS_MASK) != 0
    }

    /// Set CLK_SYS_PLL_SYS bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_pll_sys(&self) {
        self.set_bits(Self::CLK_SYS_PLL_SYS_MASK);
    }

    /// Clear CLK_SYS_PLL_SYS bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_pll_sys(&self) {
        self.clr_bits(Self::CLK_SYS_PLL_SYS_MASK);
    }

    /// **CLK_SYS_PIO2** `[20]` (RW)
    #[inline(always)]
    pub fn clk_sys_pio2(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_PIO2_MASK) != 0
    }

    /// Set CLK_SYS_PIO2 bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_pio2(&self) {
        self.set_bits(Self::CLK_SYS_PIO2_MASK);
    }

    /// Clear CLK_SYS_PIO2 bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_pio2(&self) {
        self.clr_bits(Self::CLK_SYS_PIO2_MASK);
    }

    /// **CLK_SYS_PIO1** `[19]` (RW)
    #[inline(always)]
    pub fn clk_sys_pio1(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_PIO1_MASK) != 0
    }

    /// Set CLK_SYS_PIO1 bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_pio1(&self) {
        self.set_bits(Self::CLK_SYS_PIO1_MASK);
    }

    /// Clear CLK_SYS_PIO1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_pio1(&self) {
        self.clr_bits(Self::CLK_SYS_PIO1_MASK);
    }

    /// **CLK_SYS_PIO0** `[18]` (RW)
    #[inline(always)]
    pub fn clk_sys_pio0(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_PIO0_MASK) != 0
    }

    /// Set CLK_SYS_PIO0 bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_pio0(&self) {
        self.set_bits(Self::CLK_SYS_PIO0_MASK);
    }

    /// Clear CLK_SYS_PIO0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_pio0(&self) {
        self.clr_bits(Self::CLK_SYS_PIO0_MASK);
    }

    /// **CLK_SYS_PADS** `[17]` (RW)
    #[inline(always)]
    pub fn clk_sys_pads(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_PADS_MASK) != 0
    }

    /// Set CLK_SYS_PADS bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_pads(&self) {
        self.set_bits(Self::CLK_SYS_PADS_MASK);
    }

    /// Clear CLK_SYS_PADS bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_pads(&self) {
        self.clr_bits(Self::CLK_SYS_PADS_MASK);
    }

    /// **CLK_SYS_OTP** `[16]` (RW)
    #[inline(always)]
    pub fn clk_sys_otp(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_OTP_MASK) != 0
    }

    /// Set CLK_SYS_OTP bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_otp(&self) {
        self.set_bits(Self::CLK_SYS_OTP_MASK);
    }

    /// Clear CLK_SYS_OTP bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_otp(&self) {
        self.clr_bits(Self::CLK_SYS_OTP_MASK);
    }

    /// **CLK_REF_OTP** `[15]` (RW)
    #[inline(always)]
    pub fn clk_ref_otp(&self) -> bool {
        (self.value.read_bits() & Self::CLK_REF_OTP_MASK) != 0
    }

    /// Set CLK_REF_OTP bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_ref_otp(&self) {
        self.set_bits(Self::CLK_REF_OTP_MASK);
    }

    /// Clear CLK_REF_OTP bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_ref_otp(&self) {
        self.clr_bits(Self::CLK_REF_OTP_MASK);
    }

    /// **CLK_SYS_JTAG** `[14]` (RW)
    #[inline(always)]
    pub fn clk_sys_jtag(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_JTAG_MASK) != 0
    }

    /// Set CLK_SYS_JTAG bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_jtag(&self) {
        self.set_bits(Self::CLK_SYS_JTAG_MASK);
    }

    /// Clear CLK_SYS_JTAG bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_jtag(&self) {
        self.clr_bits(Self::CLK_SYS_JTAG_MASK);
    }

    /// **CLK_SYS_IO** `[13]` (RW)
    #[inline(always)]
    pub fn clk_sys_io(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_IO_MASK) != 0
    }

    /// Set CLK_SYS_IO bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_io(&self) {
        self.set_bits(Self::CLK_SYS_IO_MASK);
    }

    /// Clear CLK_SYS_IO bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_io(&self) {
        self.clr_bits(Self::CLK_SYS_IO_MASK);
    }

    /// **CLK_SYS_I2C1** `[12]` (RW)
    #[inline(always)]
    pub fn clk_sys_i2c1(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_I2C1_MASK) != 0
    }

    /// Set CLK_SYS_I2C1 bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_i2c1(&self) {
        self.set_bits(Self::CLK_SYS_I2C1_MASK);
    }

    /// Clear CLK_SYS_I2C1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_i2c1(&self) {
        self.clr_bits(Self::CLK_SYS_I2C1_MASK);
    }

    /// **CLK_SYS_I2C0** `[11]` (RW)
    #[inline(always)]
    pub fn clk_sys_i2c0(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_I2C0_MASK) != 0
    }

    /// Set CLK_SYS_I2C0 bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_i2c0(&self) {
        self.set_bits(Self::CLK_SYS_I2C0_MASK);
    }

    /// Clear CLK_SYS_I2C0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_i2c0(&self) {
        self.clr_bits(Self::CLK_SYS_I2C0_MASK);
    }

    /// **CLK_SYS_HSTX** `[10]` (RW)
    #[inline(always)]
    pub fn clk_sys_hstx(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_HSTX_MASK) != 0
    }

    /// Set CLK_SYS_HSTX bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_hstx(&self) {
        self.set_bits(Self::CLK_SYS_HSTX_MASK);
    }

    /// Clear CLK_SYS_HSTX bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_hstx(&self) {
        self.clr_bits(Self::CLK_SYS_HSTX_MASK);
    }

    /// **CLK_HSTX** `[9]` (RW)
    #[inline(always)]
    pub fn clk_hstx(&self) -> bool {
        (self.value.read_bits() & Self::CLK_HSTX_MASK) != 0
    }

    /// Set CLK_HSTX bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_hstx(&self) {
        self.set_bits(Self::CLK_HSTX_MASK);
    }

    /// Clear CLK_HSTX bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_hstx(&self) {
        self.clr_bits(Self::CLK_HSTX_MASK);
    }

    /// **CLK_SYS_GLITCH_DETECTOR** `[8]` (RW)
    #[inline(always)]
    pub fn clk_sys_glitch_detector(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_GLITCH_DETECTOR_MASK) != 0
    }

    /// Set CLK_SYS_GLITCH_DETECTOR bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_glitch_detector(&self) {
        self.set_bits(Self::CLK_SYS_GLITCH_DETECTOR_MASK);
    }

    /// Clear CLK_SYS_GLITCH_DETECTOR bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_glitch_detector(&self) {
        self.clr_bits(Self::CLK_SYS_GLITCH_DETECTOR_MASK);
    }

    /// **CLK_SYS_DMA** `[7]` (RW)
    #[inline(always)]
    pub fn clk_sys_dma(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_DMA_MASK) != 0
    }

    /// Set CLK_SYS_DMA bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_dma(&self) {
        self.set_bits(Self::CLK_SYS_DMA_MASK);
    }

    /// Clear CLK_SYS_DMA bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_dma(&self) {
        self.clr_bits(Self::CLK_SYS_DMA_MASK);
    }

    /// **CLK_SYS_BUSFABRIC** `[6]` (RW)
    #[inline(always)]
    pub fn clk_sys_busfabric(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_BUSFABRIC_MASK) != 0
    }

    /// Set CLK_SYS_BUSFABRIC bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_busfabric(&self) {
        self.set_bits(Self::CLK_SYS_BUSFABRIC_MASK);
    }

    /// Clear CLK_SYS_BUSFABRIC bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_busfabric(&self) {
        self.clr_bits(Self::CLK_SYS_BUSFABRIC_MASK);
    }

    /// **CLK_SYS_BUSCTRL** `[5]` (RW)
    #[inline(always)]
    pub fn clk_sys_busctrl(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_BUSCTRL_MASK) != 0
    }

    /// Set CLK_SYS_BUSCTRL bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_busctrl(&self) {
        self.set_bits(Self::CLK_SYS_BUSCTRL_MASK);
    }

    /// Clear CLK_SYS_BUSCTRL bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_busctrl(&self) {
        self.clr_bits(Self::CLK_SYS_BUSCTRL_MASK);
    }

    /// **CLK_SYS_BOOTRAM** `[4]` (RW)
    #[inline(always)]
    pub fn clk_sys_bootram(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_BOOTRAM_MASK) != 0
    }

    /// Set CLK_SYS_BOOTRAM bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_bootram(&self) {
        self.set_bits(Self::CLK_SYS_BOOTRAM_MASK);
    }

    /// Clear CLK_SYS_BOOTRAM bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_bootram(&self) {
        self.clr_bits(Self::CLK_SYS_BOOTRAM_MASK);
    }

    /// **CLK_SYS_ADC** `[3]` (RW)
    #[inline(always)]
    pub fn clk_sys_adc(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_ADC_MASK) != 0
    }

    /// Set CLK_SYS_ADC bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_adc(&self) {
        self.set_bits(Self::CLK_SYS_ADC_MASK);
    }

    /// Clear CLK_SYS_ADC bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_adc(&self) {
        self.clr_bits(Self::CLK_SYS_ADC_MASK);
    }

    /// **CLK_ADC** `[2]` (RW)
    #[inline(always)]
    pub fn clk_adc(&self) -> bool {
        (self.value.read_bits() & Self::CLK_ADC_MASK) != 0
    }

    /// Set CLK_ADC bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_adc(&self) {
        self.set_bits(Self::CLK_ADC_MASK);
    }

    /// Clear CLK_ADC bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_adc(&self) {
        self.clr_bits(Self::CLK_ADC_MASK);
    }

    /// **CLK_SYS_ACCESSCTRL** `[1]` (RW)
    #[inline(always)]
    pub fn clk_sys_accessctrl(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_ACCESSCTRL_MASK) != 0
    }

    /// Set CLK_SYS_ACCESSCTRL bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_accessctrl(&self) {
        self.set_bits(Self::CLK_SYS_ACCESSCTRL_MASK);
    }

    /// Clear CLK_SYS_ACCESSCTRL bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_accessctrl(&self) {
        self.clr_bits(Self::CLK_SYS_ACCESSCTRL_MASK);
    }

    /// **CLK_SYS_CLOCKS** `[0]` (RW)
    #[inline(always)]
    pub fn clk_sys_clocks(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_CLOCKS_MASK) != 0
    }

    /// Set CLK_SYS_CLOCKS bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_clocks(&self) {
        self.set_bits(Self::CLK_SYS_CLOCKS_MASK);
    }

    /// Clear CLK_SYS_CLOCKS bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_clocks(&self) {
        self.clr_bits(Self::CLK_SYS_CLOCKS_MASK);
    }
}

/// SLEEP_EN0 value type for batch read/modify/write operations
///
/// Use `sleepen0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sleepen0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SleepEn0Val {
    bits: u32,
}

impl SleepEn0Val {
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
        Self { bits: 0xffffffff }
    }

    /// Read **CLK_SYS_SIO** `[31:31]`
    #[inline(always)]
    pub const fn clk_sys_sio(&self) -> bool {
        (self.bits & SleepEn0::CLK_SYS_SIO_MASK) != 0
    }

    /// Read **CLK_SYS_SHA256** `[30:30]`
    #[inline(always)]
    pub const fn clk_sys_sha256(&self) -> bool {
        (self.bits & SleepEn0::CLK_SYS_SHA256_MASK) != 0
    }

    /// Read **CLK_SYS_PSM** `[29:29]`
    #[inline(always)]
    pub const fn clk_sys_psm(&self) -> bool {
        (self.bits & SleepEn0::CLK_SYS_PSM_MASK) != 0
    }

    /// Read **CLK_SYS_ROSC** `[28:28]`
    #[inline(always)]
    pub const fn clk_sys_rosc(&self) -> bool {
        (self.bits & SleepEn0::CLK_SYS_ROSC_MASK) != 0
    }

    /// Read **CLK_SYS_ROM** `[27:27]`
    #[inline(always)]
    pub const fn clk_sys_rom(&self) -> bool {
        (self.bits & SleepEn0::CLK_SYS_ROM_MASK) != 0
    }

    /// Read **CLK_SYS_RESETS** `[26:26]`
    #[inline(always)]
    pub const fn clk_sys_resets(&self) -> bool {
        (self.bits & SleepEn0::CLK_SYS_RESETS_MASK) != 0
    }

    /// Read **CLK_SYS_PWM** `[25:25]`
    #[inline(always)]
    pub const fn clk_sys_pwm(&self) -> bool {
        (self.bits & SleepEn0::CLK_SYS_PWM_MASK) != 0
    }

    /// Read **CLK_SYS_POWMAN** `[24:24]`
    #[inline(always)]
    pub const fn clk_sys_powman(&self) -> bool {
        (self.bits & SleepEn0::CLK_SYS_POWMAN_MASK) != 0
    }

    /// Read **CLK_REF_POWMAN** `[23:23]`
    #[inline(always)]
    pub const fn clk_ref_powman(&self) -> bool {
        (self.bits & SleepEn0::CLK_REF_POWMAN_MASK) != 0
    }

    /// Read **CLK_SYS_PLL_USB** `[22:22]`
    #[inline(always)]
    pub const fn clk_sys_pll_usb(&self) -> bool {
        (self.bits & SleepEn0::CLK_SYS_PLL_USB_MASK) != 0
    }

    /// Read **CLK_SYS_PLL_SYS** `[21:21]`
    #[inline(always)]
    pub const fn clk_sys_pll_sys(&self) -> bool {
        (self.bits & SleepEn0::CLK_SYS_PLL_SYS_MASK) != 0
    }

    /// Read **CLK_SYS_PIO2** `[20:20]`
    #[inline(always)]
    pub const fn clk_sys_pio2(&self) -> bool {
        (self.bits & SleepEn0::CLK_SYS_PIO2_MASK) != 0
    }

    /// Read **CLK_SYS_PIO1** `[19:19]`
    #[inline(always)]
    pub const fn clk_sys_pio1(&self) -> bool {
        (self.bits & SleepEn0::CLK_SYS_PIO1_MASK) != 0
    }

    /// Read **CLK_SYS_PIO0** `[18:18]`
    #[inline(always)]
    pub const fn clk_sys_pio0(&self) -> bool {
        (self.bits & SleepEn0::CLK_SYS_PIO0_MASK) != 0
    }

    /// Read **CLK_SYS_PADS** `[17:17]`
    #[inline(always)]
    pub const fn clk_sys_pads(&self) -> bool {
        (self.bits & SleepEn0::CLK_SYS_PADS_MASK) != 0
    }

    /// Read **CLK_SYS_OTP** `[16:16]`
    #[inline(always)]
    pub const fn clk_sys_otp(&self) -> bool {
        (self.bits & SleepEn0::CLK_SYS_OTP_MASK) != 0
    }

    /// Read **CLK_REF_OTP** `[15:15]`
    #[inline(always)]
    pub const fn clk_ref_otp(&self) -> bool {
        (self.bits & SleepEn0::CLK_REF_OTP_MASK) != 0
    }

    /// Read **CLK_SYS_JTAG** `[14:14]`
    #[inline(always)]
    pub const fn clk_sys_jtag(&self) -> bool {
        (self.bits & SleepEn0::CLK_SYS_JTAG_MASK) != 0
    }

    /// Read **CLK_SYS_IO** `[13:13]`
    #[inline(always)]
    pub const fn clk_sys_io(&self) -> bool {
        (self.bits & SleepEn0::CLK_SYS_IO_MASK) != 0
    }

    /// Read **CLK_SYS_I2C1** `[12:12]`
    #[inline(always)]
    pub const fn clk_sys_i2c1(&self) -> bool {
        (self.bits & SleepEn0::CLK_SYS_I2C1_MASK) != 0
    }

    /// Read **CLK_SYS_I2C0** `[11:11]`
    #[inline(always)]
    pub const fn clk_sys_i2c0(&self) -> bool {
        (self.bits & SleepEn0::CLK_SYS_I2C0_MASK) != 0
    }

    /// Read **CLK_SYS_HSTX** `[10:10]`
    #[inline(always)]
    pub const fn clk_sys_hstx(&self) -> bool {
        (self.bits & SleepEn0::CLK_SYS_HSTX_MASK) != 0
    }

    /// Read **CLK_HSTX** `[9:9]`
    #[inline(always)]
    pub const fn clk_hstx(&self) -> bool {
        (self.bits & SleepEn0::CLK_HSTX_MASK) != 0
    }

    /// Read **CLK_SYS_GLITCH_DETECTOR** `[8:8]`
    #[inline(always)]
    pub const fn clk_sys_glitch_detector(&self) -> bool {
        (self.bits & SleepEn0::CLK_SYS_GLITCH_DETECTOR_MASK) != 0
    }

    /// Read **CLK_SYS_DMA** `[7:7]`
    #[inline(always)]
    pub const fn clk_sys_dma(&self) -> bool {
        (self.bits & SleepEn0::CLK_SYS_DMA_MASK) != 0
    }

    /// Read **CLK_SYS_BUSFABRIC** `[6:6]`
    #[inline(always)]
    pub const fn clk_sys_busfabric(&self) -> bool {
        (self.bits & SleepEn0::CLK_SYS_BUSFABRIC_MASK) != 0
    }

    /// Read **CLK_SYS_BUSCTRL** `[5:5]`
    #[inline(always)]
    pub const fn clk_sys_busctrl(&self) -> bool {
        (self.bits & SleepEn0::CLK_SYS_BUSCTRL_MASK) != 0
    }

    /// Read **CLK_SYS_BOOTRAM** `[4:4]`
    #[inline(always)]
    pub const fn clk_sys_bootram(&self) -> bool {
        (self.bits & SleepEn0::CLK_SYS_BOOTRAM_MASK) != 0
    }

    /// Read **CLK_SYS_ADC** `[3:3]`
    #[inline(always)]
    pub const fn clk_sys_adc(&self) -> bool {
        (self.bits & SleepEn0::CLK_SYS_ADC_MASK) != 0
    }

    /// Read **CLK_ADC** `[2:2]`
    #[inline(always)]
    pub const fn clk_adc(&self) -> bool {
        (self.bits & SleepEn0::CLK_ADC_MASK) != 0
    }

    /// Read **CLK_SYS_ACCESSCTRL** `[1:1]`
    #[inline(always)]
    pub const fn clk_sys_accessctrl(&self) -> bool {
        (self.bits & SleepEn0::CLK_SYS_ACCESSCTRL_MASK) != 0
    }

    /// Read **CLK_SYS_CLOCKS** `[0:0]`
    #[inline(always)]
    pub const fn clk_sys_clocks(&self) -> bool {
        (self.bits & SleepEn0::CLK_SYS_CLOCKS_MASK) != 0
    }

    /// Set **CLK_SYS_SIO** `[31:31]`
    #[inline(always)]
    pub const fn with_clk_sys_sio(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SleepEn0::CLK_SYS_SIO_MASK
            } else {
                self.bits & !SleepEn0::CLK_SYS_SIO_MASK
            }
        }
    }

    /// Set **CLK_SYS_SHA256** `[30:30]`
    #[inline(always)]
    pub const fn with_clk_sys_sha256(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SleepEn0::CLK_SYS_SHA256_MASK
            } else {
                self.bits & !SleepEn0::CLK_SYS_SHA256_MASK
            }
        }
    }

    /// Set **CLK_SYS_PSM** `[29:29]`
    #[inline(always)]
    pub const fn with_clk_sys_psm(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SleepEn0::CLK_SYS_PSM_MASK
            } else {
                self.bits & !SleepEn0::CLK_SYS_PSM_MASK
            }
        }
    }

    /// Set **CLK_SYS_ROSC** `[28:28]`
    #[inline(always)]
    pub const fn with_clk_sys_rosc(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SleepEn0::CLK_SYS_ROSC_MASK
            } else {
                self.bits & !SleepEn0::CLK_SYS_ROSC_MASK
            }
        }
    }

    /// Set **CLK_SYS_ROM** `[27:27]`
    #[inline(always)]
    pub const fn with_clk_sys_rom(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SleepEn0::CLK_SYS_ROM_MASK
            } else {
                self.bits & !SleepEn0::CLK_SYS_ROM_MASK
            }
        }
    }

    /// Set **CLK_SYS_RESETS** `[26:26]`
    #[inline(always)]
    pub const fn with_clk_sys_resets(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SleepEn0::CLK_SYS_RESETS_MASK
            } else {
                self.bits & !SleepEn0::CLK_SYS_RESETS_MASK
            }
        }
    }

    /// Set **CLK_SYS_PWM** `[25:25]`
    #[inline(always)]
    pub const fn with_clk_sys_pwm(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SleepEn0::CLK_SYS_PWM_MASK
            } else {
                self.bits & !SleepEn0::CLK_SYS_PWM_MASK
            }
        }
    }

    /// Set **CLK_SYS_POWMAN** `[24:24]`
    #[inline(always)]
    pub const fn with_clk_sys_powman(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SleepEn0::CLK_SYS_POWMAN_MASK
            } else {
                self.bits & !SleepEn0::CLK_SYS_POWMAN_MASK
            }
        }
    }

    /// Set **CLK_REF_POWMAN** `[23:23]`
    #[inline(always)]
    pub const fn with_clk_ref_powman(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SleepEn0::CLK_REF_POWMAN_MASK
            } else {
                self.bits & !SleepEn0::CLK_REF_POWMAN_MASK
            }
        }
    }

    /// Set **CLK_SYS_PLL_USB** `[22:22]`
    #[inline(always)]
    pub const fn with_clk_sys_pll_usb(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SleepEn0::CLK_SYS_PLL_USB_MASK
            } else {
                self.bits & !SleepEn0::CLK_SYS_PLL_USB_MASK
            }
        }
    }

    /// Set **CLK_SYS_PLL_SYS** `[21:21]`
    #[inline(always)]
    pub const fn with_clk_sys_pll_sys(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SleepEn0::CLK_SYS_PLL_SYS_MASK
            } else {
                self.bits & !SleepEn0::CLK_SYS_PLL_SYS_MASK
            }
        }
    }

    /// Set **CLK_SYS_PIO2** `[20:20]`
    #[inline(always)]
    pub const fn with_clk_sys_pio2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SleepEn0::CLK_SYS_PIO2_MASK
            } else {
                self.bits & !SleepEn0::CLK_SYS_PIO2_MASK
            }
        }
    }

    /// Set **CLK_SYS_PIO1** `[19:19]`
    #[inline(always)]
    pub const fn with_clk_sys_pio1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SleepEn0::CLK_SYS_PIO1_MASK
            } else {
                self.bits & !SleepEn0::CLK_SYS_PIO1_MASK
            }
        }
    }

    /// Set **CLK_SYS_PIO0** `[18:18]`
    #[inline(always)]
    pub const fn with_clk_sys_pio0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SleepEn0::CLK_SYS_PIO0_MASK
            } else {
                self.bits & !SleepEn0::CLK_SYS_PIO0_MASK
            }
        }
    }

    /// Set **CLK_SYS_PADS** `[17:17]`
    #[inline(always)]
    pub const fn with_clk_sys_pads(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SleepEn0::CLK_SYS_PADS_MASK
            } else {
                self.bits & !SleepEn0::CLK_SYS_PADS_MASK
            }
        }
    }

    /// Set **CLK_SYS_OTP** `[16:16]`
    #[inline(always)]
    pub const fn with_clk_sys_otp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SleepEn0::CLK_SYS_OTP_MASK
            } else {
                self.bits & !SleepEn0::CLK_SYS_OTP_MASK
            }
        }
    }

    /// Set **CLK_REF_OTP** `[15:15]`
    #[inline(always)]
    pub const fn with_clk_ref_otp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SleepEn0::CLK_REF_OTP_MASK
            } else {
                self.bits & !SleepEn0::CLK_REF_OTP_MASK
            }
        }
    }

    /// Set **CLK_SYS_JTAG** `[14:14]`
    #[inline(always)]
    pub const fn with_clk_sys_jtag(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SleepEn0::CLK_SYS_JTAG_MASK
            } else {
                self.bits & !SleepEn0::CLK_SYS_JTAG_MASK
            }
        }
    }

    /// Set **CLK_SYS_IO** `[13:13]`
    #[inline(always)]
    pub const fn with_clk_sys_io(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SleepEn0::CLK_SYS_IO_MASK
            } else {
                self.bits & !SleepEn0::CLK_SYS_IO_MASK
            }
        }
    }

    /// Set **CLK_SYS_I2C1** `[12:12]`
    #[inline(always)]
    pub const fn with_clk_sys_i2c1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SleepEn0::CLK_SYS_I2C1_MASK
            } else {
                self.bits & !SleepEn0::CLK_SYS_I2C1_MASK
            }
        }
    }

    /// Set **CLK_SYS_I2C0** `[11:11]`
    #[inline(always)]
    pub const fn with_clk_sys_i2c0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SleepEn0::CLK_SYS_I2C0_MASK
            } else {
                self.bits & !SleepEn0::CLK_SYS_I2C0_MASK
            }
        }
    }

    /// Set **CLK_SYS_HSTX** `[10:10]`
    #[inline(always)]
    pub const fn with_clk_sys_hstx(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SleepEn0::CLK_SYS_HSTX_MASK
            } else {
                self.bits & !SleepEn0::CLK_SYS_HSTX_MASK
            }
        }
    }

    /// Set **CLK_HSTX** `[9:9]`
    #[inline(always)]
    pub const fn with_clk_hstx(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SleepEn0::CLK_HSTX_MASK
            } else {
                self.bits & !SleepEn0::CLK_HSTX_MASK
            }
        }
    }

    /// Set **CLK_SYS_GLITCH_DETECTOR** `[8:8]`
    #[inline(always)]
    pub const fn with_clk_sys_glitch_detector(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SleepEn0::CLK_SYS_GLITCH_DETECTOR_MASK
            } else {
                self.bits & !SleepEn0::CLK_SYS_GLITCH_DETECTOR_MASK
            }
        }
    }

    /// Set **CLK_SYS_DMA** `[7:7]`
    #[inline(always)]
    pub const fn with_clk_sys_dma(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SleepEn0::CLK_SYS_DMA_MASK
            } else {
                self.bits & !SleepEn0::CLK_SYS_DMA_MASK
            }
        }
    }

    /// Set **CLK_SYS_BUSFABRIC** `[6:6]`
    #[inline(always)]
    pub const fn with_clk_sys_busfabric(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SleepEn0::CLK_SYS_BUSFABRIC_MASK
            } else {
                self.bits & !SleepEn0::CLK_SYS_BUSFABRIC_MASK
            }
        }
    }

    /// Set **CLK_SYS_BUSCTRL** `[5:5]`
    #[inline(always)]
    pub const fn with_clk_sys_busctrl(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SleepEn0::CLK_SYS_BUSCTRL_MASK
            } else {
                self.bits & !SleepEn0::CLK_SYS_BUSCTRL_MASK
            }
        }
    }

    /// Set **CLK_SYS_BOOTRAM** `[4:4]`
    #[inline(always)]
    pub const fn with_clk_sys_bootram(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SleepEn0::CLK_SYS_BOOTRAM_MASK
            } else {
                self.bits & !SleepEn0::CLK_SYS_BOOTRAM_MASK
            }
        }
    }

    /// Set **CLK_SYS_ADC** `[3:3]`
    #[inline(always)]
    pub const fn with_clk_sys_adc(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SleepEn0::CLK_SYS_ADC_MASK
            } else {
                self.bits & !SleepEn0::CLK_SYS_ADC_MASK
            }
        }
    }

    /// Set **CLK_ADC** `[2:2]`
    #[inline(always)]
    pub const fn with_clk_adc(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SleepEn0::CLK_ADC_MASK
            } else {
                self.bits & !SleepEn0::CLK_ADC_MASK
            }
        }
    }

    /// Set **CLK_SYS_ACCESSCTRL** `[1:1]`
    #[inline(always)]
    pub const fn with_clk_sys_accessctrl(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SleepEn0::CLK_SYS_ACCESSCTRL_MASK
            } else {
                self.bits & !SleepEn0::CLK_SYS_ACCESSCTRL_MASK
            }
        }
    }

    /// Set **CLK_SYS_CLOCKS** `[0:0]`
    #[inline(always)]
    pub const fn with_clk_sys_clocks(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SleepEn0::CLK_SYS_CLOCKS_MASK
            } else {
                self.bits & !SleepEn0::CLK_SYS_CLOCKS_MASK
            }
        }
    }
}

impl Default for SleepEn0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SleepEn0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SleepEn0Val {
        SleepEn0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SleepEn0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SleepEn0Val) -> SleepEn0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SLEEP_EN1 Register
///
/// enable clock in sleep mode
///
/// Address offset: 0x00b8
/// Reset value: 0x7fffffff
///
/// ## Fields
///
/// - **CLK_SYS_XOSC** `[30]` (RW)
///
/// - **CLK_SYS_XIP** `[29]` (RW)
///
/// - **CLK_SYS_WATCHDOG** `[28]` (RW)
///
/// - **CLK_USB** `[27]` (RW)
///
/// - **CLK_SYS_USBCTRL** `[26]` (RW)
///
/// - **CLK_SYS_UART1** `[25]` (RW)
///
/// - **CLK_PERI_UART1** `[24]` (RW)
///
/// - **CLK_SYS_UART0** `[23]` (RW)
///
/// - **CLK_PERI_UART0** `[22]` (RW)
///
/// - **CLK_SYS_TRNG** `[21]` (RW)
///
/// - **CLK_SYS_TIMER1** `[20]` (RW)
///
/// - **CLK_SYS_TIMER0** `[19]` (RW)
///
/// - **CLK_SYS_TICKS** `[18]` (RW)
///
/// - **CLK_REF_TICKS** `[17]` (RW)
///
/// - **CLK_SYS_TBMAN** `[16]` (RW)
///
/// - **CLK_SYS_SYSINFO** `[15]` (RW)
///
/// - **CLK_SYS_SYSCFG** `[14]` (RW)
///
/// - **CLK_SYS_SRAM9** `[13]` (RW)
///
/// - **CLK_SYS_SRAM8** `[12]` (RW)
///
/// - **CLK_SYS_SRAM7** `[11]` (RW)
///
/// - **CLK_SYS_SRAM6** `[10]` (RW)
///
/// - **CLK_SYS_SRAM5** `[9]` (RW)
///
/// - **CLK_SYS_SRAM4** `[8]` (RW)
///
/// - **CLK_SYS_SRAM3** `[7]` (RW)
///
/// - **CLK_SYS_SRAM2** `[6]` (RW)
///
/// - **CLK_SYS_SRAM1** `[5]` (RW)
///
/// - **CLK_SYS_SRAM0** `[4]` (RW)
///
/// - **CLK_SYS_SPI1** `[3]` (RW)
///
/// - **CLK_PERI_SPI1** `[2]` (RW)
///
/// - **CLK_SYS_SPI0** `[1]` (RW)
///
/// - **CLK_PERI_SPI0** `[0]` (RW)
#[repr(transparent)]
pub struct SleepEn1 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for SleepEn1 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SleepEn1 {
    // ======== Field Constants ========

    /// **CLK_SYS_XOSC**
    /// Bits `[30:30]`, mask `0x40000000`
    pub const CLK_SYS_XOSC_OFFSET: u32 = 30;
    pub const CLK_SYS_XOSC_MASK: u32 = 0x40000000;
    pub const CLK_SYS_XOSC: u32 = 1 << 30;

    /// **CLK_SYS_XIP**
    /// Bits `[29:29]`, mask `0x20000000`
    pub const CLK_SYS_XIP_OFFSET: u32 = 29;
    pub const CLK_SYS_XIP_MASK: u32 = 0x20000000;
    pub const CLK_SYS_XIP: u32 = 1 << 29;

    /// **CLK_SYS_WATCHDOG**
    /// Bits `[28:28]`, mask `0x10000000`
    pub const CLK_SYS_WATCHDOG_OFFSET: u32 = 28;
    pub const CLK_SYS_WATCHDOG_MASK: u32 = 0x10000000;
    pub const CLK_SYS_WATCHDOG: u32 = 1 << 28;

    /// **CLK_USB**
    /// Bits `[27:27]`, mask `0x08000000`
    pub const CLK_USB_OFFSET: u32 = 27;
    pub const CLK_USB_MASK: u32 = 0x08000000;
    pub const CLK_USB: u32 = 1 << 27;

    /// **CLK_SYS_USBCTRL**
    /// Bits `[26:26]`, mask `0x04000000`
    pub const CLK_SYS_USBCTRL_OFFSET: u32 = 26;
    pub const CLK_SYS_USBCTRL_MASK: u32 = 0x04000000;
    pub const CLK_SYS_USBCTRL: u32 = 1 << 26;

    /// **CLK_SYS_UART1**
    /// Bits `[25:25]`, mask `0x02000000`
    pub const CLK_SYS_UART1_OFFSET: u32 = 25;
    pub const CLK_SYS_UART1_MASK: u32 = 0x02000000;
    pub const CLK_SYS_UART1: u32 = 1 << 25;

    /// **CLK_PERI_UART1**
    /// Bits `[24:24]`, mask `0x01000000`
    pub const CLK_PERI_UART1_OFFSET: u32 = 24;
    pub const CLK_PERI_UART1_MASK: u32 = 0x01000000;
    pub const CLK_PERI_UART1: u32 = 1 << 24;

    /// **CLK_SYS_UART0**
    /// Bits `[23:23]`, mask `0x00800000`
    pub const CLK_SYS_UART0_OFFSET: u32 = 23;
    pub const CLK_SYS_UART0_MASK: u32 = 0x00800000;
    pub const CLK_SYS_UART0: u32 = 1 << 23;

    /// **CLK_PERI_UART0**
    /// Bits `[22:22]`, mask `0x00400000`
    pub const CLK_PERI_UART0_OFFSET: u32 = 22;
    pub const CLK_PERI_UART0_MASK: u32 = 0x00400000;
    pub const CLK_PERI_UART0: u32 = 1 << 22;

    /// **CLK_SYS_TRNG**
    /// Bits `[21:21]`, mask `0x00200000`
    pub const CLK_SYS_TRNG_OFFSET: u32 = 21;
    pub const CLK_SYS_TRNG_MASK: u32 = 0x00200000;
    pub const CLK_SYS_TRNG: u32 = 1 << 21;

    /// **CLK_SYS_TIMER1**
    /// Bits `[20:20]`, mask `0x00100000`
    pub const CLK_SYS_TIMER1_OFFSET: u32 = 20;
    pub const CLK_SYS_TIMER1_MASK: u32 = 0x00100000;
    pub const CLK_SYS_TIMER1: u32 = 1 << 20;

    /// **CLK_SYS_TIMER0**
    /// Bits `[19:19]`, mask `0x00080000`
    pub const CLK_SYS_TIMER0_OFFSET: u32 = 19;
    pub const CLK_SYS_TIMER0_MASK: u32 = 0x00080000;
    pub const CLK_SYS_TIMER0: u32 = 1 << 19;

    /// **CLK_SYS_TICKS**
    /// Bits `[18:18]`, mask `0x00040000`
    pub const CLK_SYS_TICKS_OFFSET: u32 = 18;
    pub const CLK_SYS_TICKS_MASK: u32 = 0x00040000;
    pub const CLK_SYS_TICKS: u32 = 1 << 18;

    /// **CLK_REF_TICKS**
    /// Bits `[17:17]`, mask `0x00020000`
    pub const CLK_REF_TICKS_OFFSET: u32 = 17;
    pub const CLK_REF_TICKS_MASK: u32 = 0x00020000;
    pub const CLK_REF_TICKS: u32 = 1 << 17;

    /// **CLK_SYS_TBMAN**
    /// Bits `[16:16]`, mask `0x00010000`
    pub const CLK_SYS_TBMAN_OFFSET: u32 = 16;
    pub const CLK_SYS_TBMAN_MASK: u32 = 0x00010000;
    pub const CLK_SYS_TBMAN: u32 = 1 << 16;

    /// **CLK_SYS_SYSINFO**
    /// Bits `[15:15]`, mask `0x00008000`
    pub const CLK_SYS_SYSINFO_OFFSET: u32 = 15;
    pub const CLK_SYS_SYSINFO_MASK: u32 = 0x00008000;
    pub const CLK_SYS_SYSINFO: u32 = 1 << 15;

    /// **CLK_SYS_SYSCFG**
    /// Bits `[14:14]`, mask `0x00004000`
    pub const CLK_SYS_SYSCFG_OFFSET: u32 = 14;
    pub const CLK_SYS_SYSCFG_MASK: u32 = 0x00004000;
    pub const CLK_SYS_SYSCFG: u32 = 1 << 14;

    /// **CLK_SYS_SRAM9**
    /// Bits `[13:13]`, mask `0x00002000`
    pub const CLK_SYS_SRAM9_OFFSET: u32 = 13;
    pub const CLK_SYS_SRAM9_MASK: u32 = 0x00002000;
    pub const CLK_SYS_SRAM9: u32 = 1 << 13;

    /// **CLK_SYS_SRAM8**
    /// Bits `[12:12]`, mask `0x00001000`
    pub const CLK_SYS_SRAM8_OFFSET: u32 = 12;
    pub const CLK_SYS_SRAM8_MASK: u32 = 0x00001000;
    pub const CLK_SYS_SRAM8: u32 = 1 << 12;

    /// **CLK_SYS_SRAM7**
    /// Bits `[11:11]`, mask `0x00000800`
    pub const CLK_SYS_SRAM7_OFFSET: u32 = 11;
    pub const CLK_SYS_SRAM7_MASK: u32 = 0x00000800;
    pub const CLK_SYS_SRAM7: u32 = 1 << 11;

    /// **CLK_SYS_SRAM6**
    /// Bits `[10:10]`, mask `0x00000400`
    pub const CLK_SYS_SRAM6_OFFSET: u32 = 10;
    pub const CLK_SYS_SRAM6_MASK: u32 = 0x00000400;
    pub const CLK_SYS_SRAM6: u32 = 1 << 10;

    /// **CLK_SYS_SRAM5**
    /// Bits `[9:9]`, mask `0x00000200`
    pub const CLK_SYS_SRAM5_OFFSET: u32 = 9;
    pub const CLK_SYS_SRAM5_MASK: u32 = 0x00000200;
    pub const CLK_SYS_SRAM5: u32 = 1 << 9;

    /// **CLK_SYS_SRAM4**
    /// Bits `[8:8]`, mask `0x00000100`
    pub const CLK_SYS_SRAM4_OFFSET: u32 = 8;
    pub const CLK_SYS_SRAM4_MASK: u32 = 0x00000100;
    pub const CLK_SYS_SRAM4: u32 = 1 << 8;

    /// **CLK_SYS_SRAM3**
    /// Bits `[7:7]`, mask `0x00000080`
    pub const CLK_SYS_SRAM3_OFFSET: u32 = 7;
    pub const CLK_SYS_SRAM3_MASK: u32 = 0x00000080;
    pub const CLK_SYS_SRAM3: u32 = 1 << 7;

    /// **CLK_SYS_SRAM2**
    /// Bits `[6:6]`, mask `0x00000040`
    pub const CLK_SYS_SRAM2_OFFSET: u32 = 6;
    pub const CLK_SYS_SRAM2_MASK: u32 = 0x00000040;
    pub const CLK_SYS_SRAM2: u32 = 1 << 6;

    /// **CLK_SYS_SRAM1**
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CLK_SYS_SRAM1_OFFSET: u32 = 5;
    pub const CLK_SYS_SRAM1_MASK: u32 = 0x00000020;
    pub const CLK_SYS_SRAM1: u32 = 1 << 5;

    /// **CLK_SYS_SRAM0**
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CLK_SYS_SRAM0_OFFSET: u32 = 4;
    pub const CLK_SYS_SRAM0_MASK: u32 = 0x00000010;
    pub const CLK_SYS_SRAM0: u32 = 1 << 4;

    /// **CLK_SYS_SPI1**
    /// Bits `[3:3]`, mask `0x00000008`
    pub const CLK_SYS_SPI1_OFFSET: u32 = 3;
    pub const CLK_SYS_SPI1_MASK: u32 = 0x00000008;
    pub const CLK_SYS_SPI1: u32 = 1 << 3;

    /// **CLK_PERI_SPI1**
    /// Bits `[2:2]`, mask `0x00000004`
    pub const CLK_PERI_SPI1_OFFSET: u32 = 2;
    pub const CLK_PERI_SPI1_MASK: u32 = 0x00000004;
    pub const CLK_PERI_SPI1: u32 = 1 << 2;

    /// **CLK_SYS_SPI0**
    /// Bits `[1:1]`, mask `0x00000002`
    pub const CLK_SYS_SPI0_OFFSET: u32 = 1;
    pub const CLK_SYS_SPI0_MASK: u32 = 0x00000002;
    pub const CLK_SYS_SPI0: u32 = 1 << 1;

    /// **CLK_PERI_SPI0**
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CLK_PERI_SPI0_OFFSET: u32 = 0;
    pub const CLK_PERI_SPI0_MASK: u32 = 0x00000001;
    pub const CLK_PERI_SPI0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CLK_SYS_XOSC** `[30]` (RW)
    #[inline(always)]
    pub fn clk_sys_xosc(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_XOSC_MASK) != 0
    }

    /// Set CLK_SYS_XOSC bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_xosc(&self) {
        self.set_bits(Self::CLK_SYS_XOSC_MASK);
    }

    /// Clear CLK_SYS_XOSC bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_xosc(&self) {
        self.clr_bits(Self::CLK_SYS_XOSC_MASK);
    }

    /// **CLK_SYS_XIP** `[29]` (RW)
    #[inline(always)]
    pub fn clk_sys_xip(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_XIP_MASK) != 0
    }

    /// Set CLK_SYS_XIP bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_xip(&self) {
        self.set_bits(Self::CLK_SYS_XIP_MASK);
    }

    /// Clear CLK_SYS_XIP bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_xip(&self) {
        self.clr_bits(Self::CLK_SYS_XIP_MASK);
    }

    /// **CLK_SYS_WATCHDOG** `[28]` (RW)
    #[inline(always)]
    pub fn clk_sys_watchdog(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_WATCHDOG_MASK) != 0
    }

    /// Set CLK_SYS_WATCHDOG bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_watchdog(&self) {
        self.set_bits(Self::CLK_SYS_WATCHDOG_MASK);
    }

    /// Clear CLK_SYS_WATCHDOG bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_watchdog(&self) {
        self.clr_bits(Self::CLK_SYS_WATCHDOG_MASK);
    }

    /// **CLK_USB** `[27]` (RW)
    #[inline(always)]
    pub fn clk_usb(&self) -> bool {
        (self.value.read_bits() & Self::CLK_USB_MASK) != 0
    }

    /// Set CLK_USB bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_usb(&self) {
        self.set_bits(Self::CLK_USB_MASK);
    }

    /// Clear CLK_USB bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_usb(&self) {
        self.clr_bits(Self::CLK_USB_MASK);
    }

    /// **CLK_SYS_USBCTRL** `[26]` (RW)
    #[inline(always)]
    pub fn clk_sys_usbctrl(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_USBCTRL_MASK) != 0
    }

    /// Set CLK_SYS_USBCTRL bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_usbctrl(&self) {
        self.set_bits(Self::CLK_SYS_USBCTRL_MASK);
    }

    /// Clear CLK_SYS_USBCTRL bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_usbctrl(&self) {
        self.clr_bits(Self::CLK_SYS_USBCTRL_MASK);
    }

    /// **CLK_SYS_UART1** `[25]` (RW)
    #[inline(always)]
    pub fn clk_sys_uart1(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_UART1_MASK) != 0
    }

    /// Set CLK_SYS_UART1 bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_uart1(&self) {
        self.set_bits(Self::CLK_SYS_UART1_MASK);
    }

    /// Clear CLK_SYS_UART1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_uart1(&self) {
        self.clr_bits(Self::CLK_SYS_UART1_MASK);
    }

    /// **CLK_PERI_UART1** `[24]` (RW)
    #[inline(always)]
    pub fn clk_peri_uart1(&self) -> bool {
        (self.value.read_bits() & Self::CLK_PERI_UART1_MASK) != 0
    }

    /// Set CLK_PERI_UART1 bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_peri_uart1(&self) {
        self.set_bits(Self::CLK_PERI_UART1_MASK);
    }

    /// Clear CLK_PERI_UART1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_peri_uart1(&self) {
        self.clr_bits(Self::CLK_PERI_UART1_MASK);
    }

    /// **CLK_SYS_UART0** `[23]` (RW)
    #[inline(always)]
    pub fn clk_sys_uart0(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_UART0_MASK) != 0
    }

    /// Set CLK_SYS_UART0 bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_uart0(&self) {
        self.set_bits(Self::CLK_SYS_UART0_MASK);
    }

    /// Clear CLK_SYS_UART0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_uart0(&self) {
        self.clr_bits(Self::CLK_SYS_UART0_MASK);
    }

    /// **CLK_PERI_UART0** `[22]` (RW)
    #[inline(always)]
    pub fn clk_peri_uart0(&self) -> bool {
        (self.value.read_bits() & Self::CLK_PERI_UART0_MASK) != 0
    }

    /// Set CLK_PERI_UART0 bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_peri_uart0(&self) {
        self.set_bits(Self::CLK_PERI_UART0_MASK);
    }

    /// Clear CLK_PERI_UART0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_peri_uart0(&self) {
        self.clr_bits(Self::CLK_PERI_UART0_MASK);
    }

    /// **CLK_SYS_TRNG** `[21]` (RW)
    #[inline(always)]
    pub fn clk_sys_trng(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_TRNG_MASK) != 0
    }

    /// Set CLK_SYS_TRNG bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_trng(&self) {
        self.set_bits(Self::CLK_SYS_TRNG_MASK);
    }

    /// Clear CLK_SYS_TRNG bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_trng(&self) {
        self.clr_bits(Self::CLK_SYS_TRNG_MASK);
    }

    /// **CLK_SYS_TIMER1** `[20]` (RW)
    #[inline(always)]
    pub fn clk_sys_timer1(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_TIMER1_MASK) != 0
    }

    /// Set CLK_SYS_TIMER1 bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_timer1(&self) {
        self.set_bits(Self::CLK_SYS_TIMER1_MASK);
    }

    /// Clear CLK_SYS_TIMER1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_timer1(&self) {
        self.clr_bits(Self::CLK_SYS_TIMER1_MASK);
    }

    /// **CLK_SYS_TIMER0** `[19]` (RW)
    #[inline(always)]
    pub fn clk_sys_timer0(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_TIMER0_MASK) != 0
    }

    /// Set CLK_SYS_TIMER0 bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_timer0(&self) {
        self.set_bits(Self::CLK_SYS_TIMER0_MASK);
    }

    /// Clear CLK_SYS_TIMER0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_timer0(&self) {
        self.clr_bits(Self::CLK_SYS_TIMER0_MASK);
    }

    /// **CLK_SYS_TICKS** `[18]` (RW)
    #[inline(always)]
    pub fn clk_sys_ticks(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_TICKS_MASK) != 0
    }

    /// Set CLK_SYS_TICKS bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_ticks(&self) {
        self.set_bits(Self::CLK_SYS_TICKS_MASK);
    }

    /// Clear CLK_SYS_TICKS bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_ticks(&self) {
        self.clr_bits(Self::CLK_SYS_TICKS_MASK);
    }

    /// **CLK_REF_TICKS** `[17]` (RW)
    #[inline(always)]
    pub fn clk_ref_ticks(&self) -> bool {
        (self.value.read_bits() & Self::CLK_REF_TICKS_MASK) != 0
    }

    /// Set CLK_REF_TICKS bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_ref_ticks(&self) {
        self.set_bits(Self::CLK_REF_TICKS_MASK);
    }

    /// Clear CLK_REF_TICKS bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_ref_ticks(&self) {
        self.clr_bits(Self::CLK_REF_TICKS_MASK);
    }

    /// **CLK_SYS_TBMAN** `[16]` (RW)
    #[inline(always)]
    pub fn clk_sys_tbman(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_TBMAN_MASK) != 0
    }

    /// Set CLK_SYS_TBMAN bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_tbman(&self) {
        self.set_bits(Self::CLK_SYS_TBMAN_MASK);
    }

    /// Clear CLK_SYS_TBMAN bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_tbman(&self) {
        self.clr_bits(Self::CLK_SYS_TBMAN_MASK);
    }

    /// **CLK_SYS_SYSINFO** `[15]` (RW)
    #[inline(always)]
    pub fn clk_sys_sysinfo(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_SYSINFO_MASK) != 0
    }

    /// Set CLK_SYS_SYSINFO bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_sysinfo(&self) {
        self.set_bits(Self::CLK_SYS_SYSINFO_MASK);
    }

    /// Clear CLK_SYS_SYSINFO bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_sysinfo(&self) {
        self.clr_bits(Self::CLK_SYS_SYSINFO_MASK);
    }

    /// **CLK_SYS_SYSCFG** `[14]` (RW)
    #[inline(always)]
    pub fn clk_sys_syscfg(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_SYSCFG_MASK) != 0
    }

    /// Set CLK_SYS_SYSCFG bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_syscfg(&self) {
        self.set_bits(Self::CLK_SYS_SYSCFG_MASK);
    }

    /// Clear CLK_SYS_SYSCFG bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_syscfg(&self) {
        self.clr_bits(Self::CLK_SYS_SYSCFG_MASK);
    }

    /// **CLK_SYS_SRAM9** `[13]` (RW)
    #[inline(always)]
    pub fn clk_sys_sram9(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_SRAM9_MASK) != 0
    }

    /// Set CLK_SYS_SRAM9 bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_sram9(&self) {
        self.set_bits(Self::CLK_SYS_SRAM9_MASK);
    }

    /// Clear CLK_SYS_SRAM9 bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_sram9(&self) {
        self.clr_bits(Self::CLK_SYS_SRAM9_MASK);
    }

    /// **CLK_SYS_SRAM8** `[12]` (RW)
    #[inline(always)]
    pub fn clk_sys_sram8(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_SRAM8_MASK) != 0
    }

    /// Set CLK_SYS_SRAM8 bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_sram8(&self) {
        self.set_bits(Self::CLK_SYS_SRAM8_MASK);
    }

    /// Clear CLK_SYS_SRAM8 bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_sram8(&self) {
        self.clr_bits(Self::CLK_SYS_SRAM8_MASK);
    }

    /// **CLK_SYS_SRAM7** `[11]` (RW)
    #[inline(always)]
    pub fn clk_sys_sram7(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_SRAM7_MASK) != 0
    }

    /// Set CLK_SYS_SRAM7 bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_sram7(&self) {
        self.set_bits(Self::CLK_SYS_SRAM7_MASK);
    }

    /// Clear CLK_SYS_SRAM7 bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_sram7(&self) {
        self.clr_bits(Self::CLK_SYS_SRAM7_MASK);
    }

    /// **CLK_SYS_SRAM6** `[10]` (RW)
    #[inline(always)]
    pub fn clk_sys_sram6(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_SRAM6_MASK) != 0
    }

    /// Set CLK_SYS_SRAM6 bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_sram6(&self) {
        self.set_bits(Self::CLK_SYS_SRAM6_MASK);
    }

    /// Clear CLK_SYS_SRAM6 bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_sram6(&self) {
        self.clr_bits(Self::CLK_SYS_SRAM6_MASK);
    }

    /// **CLK_SYS_SRAM5** `[9]` (RW)
    #[inline(always)]
    pub fn clk_sys_sram5(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_SRAM5_MASK) != 0
    }

    /// Set CLK_SYS_SRAM5 bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_sram5(&self) {
        self.set_bits(Self::CLK_SYS_SRAM5_MASK);
    }

    /// Clear CLK_SYS_SRAM5 bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_sram5(&self) {
        self.clr_bits(Self::CLK_SYS_SRAM5_MASK);
    }

    /// **CLK_SYS_SRAM4** `[8]` (RW)
    #[inline(always)]
    pub fn clk_sys_sram4(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_SRAM4_MASK) != 0
    }

    /// Set CLK_SYS_SRAM4 bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_sram4(&self) {
        self.set_bits(Self::CLK_SYS_SRAM4_MASK);
    }

    /// Clear CLK_SYS_SRAM4 bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_sram4(&self) {
        self.clr_bits(Self::CLK_SYS_SRAM4_MASK);
    }

    /// **CLK_SYS_SRAM3** `[7]` (RW)
    #[inline(always)]
    pub fn clk_sys_sram3(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_SRAM3_MASK) != 0
    }

    /// Set CLK_SYS_SRAM3 bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_sram3(&self) {
        self.set_bits(Self::CLK_SYS_SRAM3_MASK);
    }

    /// Clear CLK_SYS_SRAM3 bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_sram3(&self) {
        self.clr_bits(Self::CLK_SYS_SRAM3_MASK);
    }

    /// **CLK_SYS_SRAM2** `[6]` (RW)
    #[inline(always)]
    pub fn clk_sys_sram2(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_SRAM2_MASK) != 0
    }

    /// Set CLK_SYS_SRAM2 bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_sram2(&self) {
        self.set_bits(Self::CLK_SYS_SRAM2_MASK);
    }

    /// Clear CLK_SYS_SRAM2 bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_sram2(&self) {
        self.clr_bits(Self::CLK_SYS_SRAM2_MASK);
    }

    /// **CLK_SYS_SRAM1** `[5]` (RW)
    #[inline(always)]
    pub fn clk_sys_sram1(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_SRAM1_MASK) != 0
    }

    /// Set CLK_SYS_SRAM1 bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_sram1(&self) {
        self.set_bits(Self::CLK_SYS_SRAM1_MASK);
    }

    /// Clear CLK_SYS_SRAM1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_sram1(&self) {
        self.clr_bits(Self::CLK_SYS_SRAM1_MASK);
    }

    /// **CLK_SYS_SRAM0** `[4]` (RW)
    #[inline(always)]
    pub fn clk_sys_sram0(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_SRAM0_MASK) != 0
    }

    /// Set CLK_SYS_SRAM0 bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_sram0(&self) {
        self.set_bits(Self::CLK_SYS_SRAM0_MASK);
    }

    /// Clear CLK_SYS_SRAM0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_sram0(&self) {
        self.clr_bits(Self::CLK_SYS_SRAM0_MASK);
    }

    /// **CLK_SYS_SPI1** `[3]` (RW)
    #[inline(always)]
    pub fn clk_sys_spi1(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_SPI1_MASK) != 0
    }

    /// Set CLK_SYS_SPI1 bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_spi1(&self) {
        self.set_bits(Self::CLK_SYS_SPI1_MASK);
    }

    /// Clear CLK_SYS_SPI1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_spi1(&self) {
        self.clr_bits(Self::CLK_SYS_SPI1_MASK);
    }

    /// **CLK_PERI_SPI1** `[2]` (RW)
    #[inline(always)]
    pub fn clk_peri_spi1(&self) -> bool {
        (self.value.read_bits() & Self::CLK_PERI_SPI1_MASK) != 0
    }

    /// Set CLK_PERI_SPI1 bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_peri_spi1(&self) {
        self.set_bits(Self::CLK_PERI_SPI1_MASK);
    }

    /// Clear CLK_PERI_SPI1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_peri_spi1(&self) {
        self.clr_bits(Self::CLK_PERI_SPI1_MASK);
    }

    /// **CLK_SYS_SPI0** `[1]` (RW)
    #[inline(always)]
    pub fn clk_sys_spi0(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_SPI0_MASK) != 0
    }

    /// Set CLK_SYS_SPI0 bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_spi0(&self) {
        self.set_bits(Self::CLK_SYS_SPI0_MASK);
    }

    /// Clear CLK_SYS_SPI0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_spi0(&self) {
        self.clr_bits(Self::CLK_SYS_SPI0_MASK);
    }

    /// **CLK_PERI_SPI0** `[0]` (RW)
    #[inline(always)]
    pub fn clk_peri_spi0(&self) -> bool {
        (self.value.read_bits() & Self::CLK_PERI_SPI0_MASK) != 0
    }

    /// Set CLK_PERI_SPI0 bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_peri_spi0(&self) {
        self.set_bits(Self::CLK_PERI_SPI0_MASK);
    }

    /// Clear CLK_PERI_SPI0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_peri_spi0(&self) {
        self.clr_bits(Self::CLK_PERI_SPI0_MASK);
    }
}

/// SLEEP_EN1 value type for batch read/modify/write operations
///
/// Use `sleepen1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sleepen1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SleepEn1Val {
    bits: u32,
}

impl SleepEn1Val {
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
        Self { bits: 0x7fffffff }
    }

    /// Read **CLK_SYS_XOSC** `[30:30]`
    #[inline(always)]
    pub const fn clk_sys_xosc(&self) -> bool {
        (self.bits & SleepEn1::CLK_SYS_XOSC_MASK) != 0
    }

    /// Read **CLK_SYS_XIP** `[29:29]`
    #[inline(always)]
    pub const fn clk_sys_xip(&self) -> bool {
        (self.bits & SleepEn1::CLK_SYS_XIP_MASK) != 0
    }

    /// Read **CLK_SYS_WATCHDOG** `[28:28]`
    #[inline(always)]
    pub const fn clk_sys_watchdog(&self) -> bool {
        (self.bits & SleepEn1::CLK_SYS_WATCHDOG_MASK) != 0
    }

    /// Read **CLK_USB** `[27:27]`
    #[inline(always)]
    pub const fn clk_usb(&self) -> bool {
        (self.bits & SleepEn1::CLK_USB_MASK) != 0
    }

    /// Read **CLK_SYS_USBCTRL** `[26:26]`
    #[inline(always)]
    pub const fn clk_sys_usbctrl(&self) -> bool {
        (self.bits & SleepEn1::CLK_SYS_USBCTRL_MASK) != 0
    }

    /// Read **CLK_SYS_UART1** `[25:25]`
    #[inline(always)]
    pub const fn clk_sys_uart1(&self) -> bool {
        (self.bits & SleepEn1::CLK_SYS_UART1_MASK) != 0
    }

    /// Read **CLK_PERI_UART1** `[24:24]`
    #[inline(always)]
    pub const fn clk_peri_uart1(&self) -> bool {
        (self.bits & SleepEn1::CLK_PERI_UART1_MASK) != 0
    }

    /// Read **CLK_SYS_UART0** `[23:23]`
    #[inline(always)]
    pub const fn clk_sys_uart0(&self) -> bool {
        (self.bits & SleepEn1::CLK_SYS_UART0_MASK) != 0
    }

    /// Read **CLK_PERI_UART0** `[22:22]`
    #[inline(always)]
    pub const fn clk_peri_uart0(&self) -> bool {
        (self.bits & SleepEn1::CLK_PERI_UART0_MASK) != 0
    }

    /// Read **CLK_SYS_TRNG** `[21:21]`
    #[inline(always)]
    pub const fn clk_sys_trng(&self) -> bool {
        (self.bits & SleepEn1::CLK_SYS_TRNG_MASK) != 0
    }

    /// Read **CLK_SYS_TIMER1** `[20:20]`
    #[inline(always)]
    pub const fn clk_sys_timer1(&self) -> bool {
        (self.bits & SleepEn1::CLK_SYS_TIMER1_MASK) != 0
    }

    /// Read **CLK_SYS_TIMER0** `[19:19]`
    #[inline(always)]
    pub const fn clk_sys_timer0(&self) -> bool {
        (self.bits & SleepEn1::CLK_SYS_TIMER0_MASK) != 0
    }

    /// Read **CLK_SYS_TICKS** `[18:18]`
    #[inline(always)]
    pub const fn clk_sys_ticks(&self) -> bool {
        (self.bits & SleepEn1::CLK_SYS_TICKS_MASK) != 0
    }

    /// Read **CLK_REF_TICKS** `[17:17]`
    #[inline(always)]
    pub const fn clk_ref_ticks(&self) -> bool {
        (self.bits & SleepEn1::CLK_REF_TICKS_MASK) != 0
    }

    /// Read **CLK_SYS_TBMAN** `[16:16]`
    #[inline(always)]
    pub const fn clk_sys_tbman(&self) -> bool {
        (self.bits & SleepEn1::CLK_SYS_TBMAN_MASK) != 0
    }

    /// Read **CLK_SYS_SYSINFO** `[15:15]`
    #[inline(always)]
    pub const fn clk_sys_sysinfo(&self) -> bool {
        (self.bits & SleepEn1::CLK_SYS_SYSINFO_MASK) != 0
    }

    /// Read **CLK_SYS_SYSCFG** `[14:14]`
    #[inline(always)]
    pub const fn clk_sys_syscfg(&self) -> bool {
        (self.bits & SleepEn1::CLK_SYS_SYSCFG_MASK) != 0
    }

    /// Read **CLK_SYS_SRAM9** `[13:13]`
    #[inline(always)]
    pub const fn clk_sys_sram9(&self) -> bool {
        (self.bits & SleepEn1::CLK_SYS_SRAM9_MASK) != 0
    }

    /// Read **CLK_SYS_SRAM8** `[12:12]`
    #[inline(always)]
    pub const fn clk_sys_sram8(&self) -> bool {
        (self.bits & SleepEn1::CLK_SYS_SRAM8_MASK) != 0
    }

    /// Read **CLK_SYS_SRAM7** `[11:11]`
    #[inline(always)]
    pub const fn clk_sys_sram7(&self) -> bool {
        (self.bits & SleepEn1::CLK_SYS_SRAM7_MASK) != 0
    }

    /// Read **CLK_SYS_SRAM6** `[10:10]`
    #[inline(always)]
    pub const fn clk_sys_sram6(&self) -> bool {
        (self.bits & SleepEn1::CLK_SYS_SRAM6_MASK) != 0
    }

    /// Read **CLK_SYS_SRAM5** `[9:9]`
    #[inline(always)]
    pub const fn clk_sys_sram5(&self) -> bool {
        (self.bits & SleepEn1::CLK_SYS_SRAM5_MASK) != 0
    }

    /// Read **CLK_SYS_SRAM4** `[8:8]`
    #[inline(always)]
    pub const fn clk_sys_sram4(&self) -> bool {
        (self.bits & SleepEn1::CLK_SYS_SRAM4_MASK) != 0
    }

    /// Read **CLK_SYS_SRAM3** `[7:7]`
    #[inline(always)]
    pub const fn clk_sys_sram3(&self) -> bool {
        (self.bits & SleepEn1::CLK_SYS_SRAM3_MASK) != 0
    }

    /// Read **CLK_SYS_SRAM2** `[6:6]`
    #[inline(always)]
    pub const fn clk_sys_sram2(&self) -> bool {
        (self.bits & SleepEn1::CLK_SYS_SRAM2_MASK) != 0
    }

    /// Read **CLK_SYS_SRAM1** `[5:5]`
    #[inline(always)]
    pub const fn clk_sys_sram1(&self) -> bool {
        (self.bits & SleepEn1::CLK_SYS_SRAM1_MASK) != 0
    }

    /// Read **CLK_SYS_SRAM0** `[4:4]`
    #[inline(always)]
    pub const fn clk_sys_sram0(&self) -> bool {
        (self.bits & SleepEn1::CLK_SYS_SRAM0_MASK) != 0
    }

    /// Read **CLK_SYS_SPI1** `[3:3]`
    #[inline(always)]
    pub const fn clk_sys_spi1(&self) -> bool {
        (self.bits & SleepEn1::CLK_SYS_SPI1_MASK) != 0
    }

    /// Read **CLK_PERI_SPI1** `[2:2]`
    #[inline(always)]
    pub const fn clk_peri_spi1(&self) -> bool {
        (self.bits & SleepEn1::CLK_PERI_SPI1_MASK) != 0
    }

    /// Read **CLK_SYS_SPI0** `[1:1]`
    #[inline(always)]
    pub const fn clk_sys_spi0(&self) -> bool {
        (self.bits & SleepEn1::CLK_SYS_SPI0_MASK) != 0
    }

    /// Read **CLK_PERI_SPI0** `[0:0]`
    #[inline(always)]
    pub const fn clk_peri_spi0(&self) -> bool {
        (self.bits & SleepEn1::CLK_PERI_SPI0_MASK) != 0
    }

    /// Set **CLK_SYS_XOSC** `[30:30]`
    #[inline(always)]
    pub const fn with_clk_sys_xosc(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SleepEn1::CLK_SYS_XOSC_MASK
            } else {
                self.bits & !SleepEn1::CLK_SYS_XOSC_MASK
            }
        }
    }

    /// Set **CLK_SYS_XIP** `[29:29]`
    #[inline(always)]
    pub const fn with_clk_sys_xip(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SleepEn1::CLK_SYS_XIP_MASK
            } else {
                self.bits & !SleepEn1::CLK_SYS_XIP_MASK
            }
        }
    }

    /// Set **CLK_SYS_WATCHDOG** `[28:28]`
    #[inline(always)]
    pub const fn with_clk_sys_watchdog(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SleepEn1::CLK_SYS_WATCHDOG_MASK
            } else {
                self.bits & !SleepEn1::CLK_SYS_WATCHDOG_MASK
            }
        }
    }

    /// Set **CLK_USB** `[27:27]`
    #[inline(always)]
    pub const fn with_clk_usb(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SleepEn1::CLK_USB_MASK
            } else {
                self.bits & !SleepEn1::CLK_USB_MASK
            }
        }
    }

    /// Set **CLK_SYS_USBCTRL** `[26:26]`
    #[inline(always)]
    pub const fn with_clk_sys_usbctrl(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SleepEn1::CLK_SYS_USBCTRL_MASK
            } else {
                self.bits & !SleepEn1::CLK_SYS_USBCTRL_MASK
            }
        }
    }

    /// Set **CLK_SYS_UART1** `[25:25]`
    #[inline(always)]
    pub const fn with_clk_sys_uart1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SleepEn1::CLK_SYS_UART1_MASK
            } else {
                self.bits & !SleepEn1::CLK_SYS_UART1_MASK
            }
        }
    }

    /// Set **CLK_PERI_UART1** `[24:24]`
    #[inline(always)]
    pub const fn with_clk_peri_uart1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SleepEn1::CLK_PERI_UART1_MASK
            } else {
                self.bits & !SleepEn1::CLK_PERI_UART1_MASK
            }
        }
    }

    /// Set **CLK_SYS_UART0** `[23:23]`
    #[inline(always)]
    pub const fn with_clk_sys_uart0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SleepEn1::CLK_SYS_UART0_MASK
            } else {
                self.bits & !SleepEn1::CLK_SYS_UART0_MASK
            }
        }
    }

    /// Set **CLK_PERI_UART0** `[22:22]`
    #[inline(always)]
    pub const fn with_clk_peri_uart0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SleepEn1::CLK_PERI_UART0_MASK
            } else {
                self.bits & !SleepEn1::CLK_PERI_UART0_MASK
            }
        }
    }

    /// Set **CLK_SYS_TRNG** `[21:21]`
    #[inline(always)]
    pub const fn with_clk_sys_trng(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SleepEn1::CLK_SYS_TRNG_MASK
            } else {
                self.bits & !SleepEn1::CLK_SYS_TRNG_MASK
            }
        }
    }

    /// Set **CLK_SYS_TIMER1** `[20:20]`
    #[inline(always)]
    pub const fn with_clk_sys_timer1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SleepEn1::CLK_SYS_TIMER1_MASK
            } else {
                self.bits & !SleepEn1::CLK_SYS_TIMER1_MASK
            }
        }
    }

    /// Set **CLK_SYS_TIMER0** `[19:19]`
    #[inline(always)]
    pub const fn with_clk_sys_timer0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SleepEn1::CLK_SYS_TIMER0_MASK
            } else {
                self.bits & !SleepEn1::CLK_SYS_TIMER0_MASK
            }
        }
    }

    /// Set **CLK_SYS_TICKS** `[18:18]`
    #[inline(always)]
    pub const fn with_clk_sys_ticks(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SleepEn1::CLK_SYS_TICKS_MASK
            } else {
                self.bits & !SleepEn1::CLK_SYS_TICKS_MASK
            }
        }
    }

    /// Set **CLK_REF_TICKS** `[17:17]`
    #[inline(always)]
    pub const fn with_clk_ref_ticks(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SleepEn1::CLK_REF_TICKS_MASK
            } else {
                self.bits & !SleepEn1::CLK_REF_TICKS_MASK
            }
        }
    }

    /// Set **CLK_SYS_TBMAN** `[16:16]`
    #[inline(always)]
    pub const fn with_clk_sys_tbman(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SleepEn1::CLK_SYS_TBMAN_MASK
            } else {
                self.bits & !SleepEn1::CLK_SYS_TBMAN_MASK
            }
        }
    }

    /// Set **CLK_SYS_SYSINFO** `[15:15]`
    #[inline(always)]
    pub const fn with_clk_sys_sysinfo(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SleepEn1::CLK_SYS_SYSINFO_MASK
            } else {
                self.bits & !SleepEn1::CLK_SYS_SYSINFO_MASK
            }
        }
    }

    /// Set **CLK_SYS_SYSCFG** `[14:14]`
    #[inline(always)]
    pub const fn with_clk_sys_syscfg(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SleepEn1::CLK_SYS_SYSCFG_MASK
            } else {
                self.bits & !SleepEn1::CLK_SYS_SYSCFG_MASK
            }
        }
    }

    /// Set **CLK_SYS_SRAM9** `[13:13]`
    #[inline(always)]
    pub const fn with_clk_sys_sram9(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SleepEn1::CLK_SYS_SRAM9_MASK
            } else {
                self.bits & !SleepEn1::CLK_SYS_SRAM9_MASK
            }
        }
    }

    /// Set **CLK_SYS_SRAM8** `[12:12]`
    #[inline(always)]
    pub const fn with_clk_sys_sram8(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SleepEn1::CLK_SYS_SRAM8_MASK
            } else {
                self.bits & !SleepEn1::CLK_SYS_SRAM8_MASK
            }
        }
    }

    /// Set **CLK_SYS_SRAM7** `[11:11]`
    #[inline(always)]
    pub const fn with_clk_sys_sram7(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SleepEn1::CLK_SYS_SRAM7_MASK
            } else {
                self.bits & !SleepEn1::CLK_SYS_SRAM7_MASK
            }
        }
    }

    /// Set **CLK_SYS_SRAM6** `[10:10]`
    #[inline(always)]
    pub const fn with_clk_sys_sram6(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SleepEn1::CLK_SYS_SRAM6_MASK
            } else {
                self.bits & !SleepEn1::CLK_SYS_SRAM6_MASK
            }
        }
    }

    /// Set **CLK_SYS_SRAM5** `[9:9]`
    #[inline(always)]
    pub const fn with_clk_sys_sram5(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SleepEn1::CLK_SYS_SRAM5_MASK
            } else {
                self.bits & !SleepEn1::CLK_SYS_SRAM5_MASK
            }
        }
    }

    /// Set **CLK_SYS_SRAM4** `[8:8]`
    #[inline(always)]
    pub const fn with_clk_sys_sram4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SleepEn1::CLK_SYS_SRAM4_MASK
            } else {
                self.bits & !SleepEn1::CLK_SYS_SRAM4_MASK
            }
        }
    }

    /// Set **CLK_SYS_SRAM3** `[7:7]`
    #[inline(always)]
    pub const fn with_clk_sys_sram3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SleepEn1::CLK_SYS_SRAM3_MASK
            } else {
                self.bits & !SleepEn1::CLK_SYS_SRAM3_MASK
            }
        }
    }

    /// Set **CLK_SYS_SRAM2** `[6:6]`
    #[inline(always)]
    pub const fn with_clk_sys_sram2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SleepEn1::CLK_SYS_SRAM2_MASK
            } else {
                self.bits & !SleepEn1::CLK_SYS_SRAM2_MASK
            }
        }
    }

    /// Set **CLK_SYS_SRAM1** `[5:5]`
    #[inline(always)]
    pub const fn with_clk_sys_sram1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SleepEn1::CLK_SYS_SRAM1_MASK
            } else {
                self.bits & !SleepEn1::CLK_SYS_SRAM1_MASK
            }
        }
    }

    /// Set **CLK_SYS_SRAM0** `[4:4]`
    #[inline(always)]
    pub const fn with_clk_sys_sram0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SleepEn1::CLK_SYS_SRAM0_MASK
            } else {
                self.bits & !SleepEn1::CLK_SYS_SRAM0_MASK
            }
        }
    }

    /// Set **CLK_SYS_SPI1** `[3:3]`
    #[inline(always)]
    pub const fn with_clk_sys_spi1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SleepEn1::CLK_SYS_SPI1_MASK
            } else {
                self.bits & !SleepEn1::CLK_SYS_SPI1_MASK
            }
        }
    }

    /// Set **CLK_PERI_SPI1** `[2:2]`
    #[inline(always)]
    pub const fn with_clk_peri_spi1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SleepEn1::CLK_PERI_SPI1_MASK
            } else {
                self.bits & !SleepEn1::CLK_PERI_SPI1_MASK
            }
        }
    }

    /// Set **CLK_SYS_SPI0** `[1:1]`
    #[inline(always)]
    pub const fn with_clk_sys_spi0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SleepEn1::CLK_SYS_SPI0_MASK
            } else {
                self.bits & !SleepEn1::CLK_SYS_SPI0_MASK
            }
        }
    }

    /// Set **CLK_PERI_SPI0** `[0:0]`
    #[inline(always)]
    pub const fn with_clk_peri_spi0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SleepEn1::CLK_PERI_SPI0_MASK
            } else {
                self.bits & !SleepEn1::CLK_PERI_SPI0_MASK
            }
        }
    }
}

impl Default for SleepEn1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SleepEn1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SleepEn1Val {
        SleepEn1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SleepEn1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SleepEn1Val) -> SleepEn1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ENABLED0 Register
///
/// indicates the state of the clock enable
///
/// Address offset: 0x00bc
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CLK_SYS_SIO** `[31]` (RO)
///
/// - **CLK_SYS_SHA256** `[30]` (RO)
///
/// - **CLK_SYS_PSM** `[29]` (RO)
///
/// - **CLK_SYS_ROSC** `[28]` (RO)
///
/// - **CLK_SYS_ROM** `[27]` (RO)
///
/// - **CLK_SYS_RESETS** `[26]` (RO)
///
/// - **CLK_SYS_PWM** `[25]` (RO)
///
/// - **CLK_SYS_POWMAN** `[24]` (RO)
///
/// - **CLK_REF_POWMAN** `[23]` (RO)
///
/// - **CLK_SYS_PLL_USB** `[22]` (RO)
///
/// - **CLK_SYS_PLL_SYS** `[21]` (RO)
///
/// - **CLK_SYS_PIO2** `[20]` (RO)
///
/// - **CLK_SYS_PIO1** `[19]` (RO)
///
/// - **CLK_SYS_PIO0** `[18]` (RO)
///
/// - **CLK_SYS_PADS** `[17]` (RO)
///
/// - **CLK_SYS_OTP** `[16]` (RO)
///
/// - **CLK_REF_OTP** `[15]` (RO)
///
/// - **CLK_SYS_JTAG** `[14]` (RO)
///
/// - **CLK_SYS_IO** `[13]` (RO)
///
/// - **CLK_SYS_I2C1** `[12]` (RO)
///
/// - **CLK_SYS_I2C0** `[11]` (RO)
///
/// - **CLK_SYS_HSTX** `[10]` (RO)
///
/// - **CLK_HSTX** `[9]` (RO)
///
/// - **CLK_SYS_GLITCH_DETECTOR** `[8]` (RO)
///
/// - **CLK_SYS_DMA** `[7]` (RO)
///
/// - **CLK_SYS_BUSFABRIC** `[6]` (RO)
///
/// - **CLK_SYS_BUSCTRL** `[5]` (RO)
///
/// - **CLK_SYS_BOOTRAM** `[4]` (RO)
///
/// - **CLK_SYS_ADC** `[3]` (RO)
///
/// - **CLK_ADC** `[2]` (RO)
///
/// - **CLK_SYS_ACCESSCTRL** `[1]` (RO)
///
/// - **CLK_SYS_CLOCKS** `[0]` (RO)
#[repr(transparent)]
pub struct Enabled0 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Enabled0 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Enabled0 {
    // ======== Field Constants ========

    /// **CLK_SYS_SIO**
    /// Bits `[31:31]`, mask `0x80000000`
    pub const CLK_SYS_SIO_OFFSET: u32 = 31;
    pub const CLK_SYS_SIO_MASK: u32 = 0x80000000;
    pub const CLK_SYS_SIO: u32 = 1 << 31;

    /// **CLK_SYS_SHA256**
    /// Bits `[30:30]`, mask `0x40000000`
    pub const CLK_SYS_SHA256_OFFSET: u32 = 30;
    pub const CLK_SYS_SHA256_MASK: u32 = 0x40000000;
    pub const CLK_SYS_SHA256: u32 = 1 << 30;

    /// **CLK_SYS_PSM**
    /// Bits `[29:29]`, mask `0x20000000`
    pub const CLK_SYS_PSM_OFFSET: u32 = 29;
    pub const CLK_SYS_PSM_MASK: u32 = 0x20000000;
    pub const CLK_SYS_PSM: u32 = 1 << 29;

    /// **CLK_SYS_ROSC**
    /// Bits `[28:28]`, mask `0x10000000`
    pub const CLK_SYS_ROSC_OFFSET: u32 = 28;
    pub const CLK_SYS_ROSC_MASK: u32 = 0x10000000;
    pub const CLK_SYS_ROSC: u32 = 1 << 28;

    /// **CLK_SYS_ROM**
    /// Bits `[27:27]`, mask `0x08000000`
    pub const CLK_SYS_ROM_OFFSET: u32 = 27;
    pub const CLK_SYS_ROM_MASK: u32 = 0x08000000;
    pub const CLK_SYS_ROM: u32 = 1 << 27;

    /// **CLK_SYS_RESETS**
    /// Bits `[26:26]`, mask `0x04000000`
    pub const CLK_SYS_RESETS_OFFSET: u32 = 26;
    pub const CLK_SYS_RESETS_MASK: u32 = 0x04000000;
    pub const CLK_SYS_RESETS: u32 = 1 << 26;

    /// **CLK_SYS_PWM**
    /// Bits `[25:25]`, mask `0x02000000`
    pub const CLK_SYS_PWM_OFFSET: u32 = 25;
    pub const CLK_SYS_PWM_MASK: u32 = 0x02000000;
    pub const CLK_SYS_PWM: u32 = 1 << 25;

    /// **CLK_SYS_POWMAN**
    /// Bits `[24:24]`, mask `0x01000000`
    pub const CLK_SYS_POWMAN_OFFSET: u32 = 24;
    pub const CLK_SYS_POWMAN_MASK: u32 = 0x01000000;
    pub const CLK_SYS_POWMAN: u32 = 1 << 24;

    /// **CLK_REF_POWMAN**
    /// Bits `[23:23]`, mask `0x00800000`
    pub const CLK_REF_POWMAN_OFFSET: u32 = 23;
    pub const CLK_REF_POWMAN_MASK: u32 = 0x00800000;
    pub const CLK_REF_POWMAN: u32 = 1 << 23;

    /// **CLK_SYS_PLL_USB**
    /// Bits `[22:22]`, mask `0x00400000`
    pub const CLK_SYS_PLL_USB_OFFSET: u32 = 22;
    pub const CLK_SYS_PLL_USB_MASK: u32 = 0x00400000;
    pub const CLK_SYS_PLL_USB: u32 = 1 << 22;

    /// **CLK_SYS_PLL_SYS**
    /// Bits `[21:21]`, mask `0x00200000`
    pub const CLK_SYS_PLL_SYS_OFFSET: u32 = 21;
    pub const CLK_SYS_PLL_SYS_MASK: u32 = 0x00200000;
    pub const CLK_SYS_PLL_SYS: u32 = 1 << 21;

    /// **CLK_SYS_PIO2**
    /// Bits `[20:20]`, mask `0x00100000`
    pub const CLK_SYS_PIO2_OFFSET: u32 = 20;
    pub const CLK_SYS_PIO2_MASK: u32 = 0x00100000;
    pub const CLK_SYS_PIO2: u32 = 1 << 20;

    /// **CLK_SYS_PIO1**
    /// Bits `[19:19]`, mask `0x00080000`
    pub const CLK_SYS_PIO1_OFFSET: u32 = 19;
    pub const CLK_SYS_PIO1_MASK: u32 = 0x00080000;
    pub const CLK_SYS_PIO1: u32 = 1 << 19;

    /// **CLK_SYS_PIO0**
    /// Bits `[18:18]`, mask `0x00040000`
    pub const CLK_SYS_PIO0_OFFSET: u32 = 18;
    pub const CLK_SYS_PIO0_MASK: u32 = 0x00040000;
    pub const CLK_SYS_PIO0: u32 = 1 << 18;

    /// **CLK_SYS_PADS**
    /// Bits `[17:17]`, mask `0x00020000`
    pub const CLK_SYS_PADS_OFFSET: u32 = 17;
    pub const CLK_SYS_PADS_MASK: u32 = 0x00020000;
    pub const CLK_SYS_PADS: u32 = 1 << 17;

    /// **CLK_SYS_OTP**
    /// Bits `[16:16]`, mask `0x00010000`
    pub const CLK_SYS_OTP_OFFSET: u32 = 16;
    pub const CLK_SYS_OTP_MASK: u32 = 0x00010000;
    pub const CLK_SYS_OTP: u32 = 1 << 16;

    /// **CLK_REF_OTP**
    /// Bits `[15:15]`, mask `0x00008000`
    pub const CLK_REF_OTP_OFFSET: u32 = 15;
    pub const CLK_REF_OTP_MASK: u32 = 0x00008000;
    pub const CLK_REF_OTP: u32 = 1 << 15;

    /// **CLK_SYS_JTAG**
    /// Bits `[14:14]`, mask `0x00004000`
    pub const CLK_SYS_JTAG_OFFSET: u32 = 14;
    pub const CLK_SYS_JTAG_MASK: u32 = 0x00004000;
    pub const CLK_SYS_JTAG: u32 = 1 << 14;

    /// **CLK_SYS_IO**
    /// Bits `[13:13]`, mask `0x00002000`
    pub const CLK_SYS_IO_OFFSET: u32 = 13;
    pub const CLK_SYS_IO_MASK: u32 = 0x00002000;
    pub const CLK_SYS_IO: u32 = 1 << 13;

    /// **CLK_SYS_I2C1**
    /// Bits `[12:12]`, mask `0x00001000`
    pub const CLK_SYS_I2C1_OFFSET: u32 = 12;
    pub const CLK_SYS_I2C1_MASK: u32 = 0x00001000;
    pub const CLK_SYS_I2C1: u32 = 1 << 12;

    /// **CLK_SYS_I2C0**
    /// Bits `[11:11]`, mask `0x00000800`
    pub const CLK_SYS_I2C0_OFFSET: u32 = 11;
    pub const CLK_SYS_I2C0_MASK: u32 = 0x00000800;
    pub const CLK_SYS_I2C0: u32 = 1 << 11;

    /// **CLK_SYS_HSTX**
    /// Bits `[10:10]`, mask `0x00000400`
    pub const CLK_SYS_HSTX_OFFSET: u32 = 10;
    pub const CLK_SYS_HSTX_MASK: u32 = 0x00000400;
    pub const CLK_SYS_HSTX: u32 = 1 << 10;

    /// **CLK_HSTX**
    /// Bits `[9:9]`, mask `0x00000200`
    pub const CLK_HSTX_OFFSET: u32 = 9;
    pub const CLK_HSTX_MASK: u32 = 0x00000200;
    pub const CLK_HSTX: u32 = 1 << 9;

    /// **CLK_SYS_GLITCH_DETECTOR**
    /// Bits `[8:8]`, mask `0x00000100`
    pub const CLK_SYS_GLITCH_DETECTOR_OFFSET: u32 = 8;
    pub const CLK_SYS_GLITCH_DETECTOR_MASK: u32 = 0x00000100;
    pub const CLK_SYS_GLITCH_DETECTOR: u32 = 1 << 8;

    /// **CLK_SYS_DMA**
    /// Bits `[7:7]`, mask `0x00000080`
    pub const CLK_SYS_DMA_OFFSET: u32 = 7;
    pub const CLK_SYS_DMA_MASK: u32 = 0x00000080;
    pub const CLK_SYS_DMA: u32 = 1 << 7;

    /// **CLK_SYS_BUSFABRIC**
    /// Bits `[6:6]`, mask `0x00000040`
    pub const CLK_SYS_BUSFABRIC_OFFSET: u32 = 6;
    pub const CLK_SYS_BUSFABRIC_MASK: u32 = 0x00000040;
    pub const CLK_SYS_BUSFABRIC: u32 = 1 << 6;

    /// **CLK_SYS_BUSCTRL**
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CLK_SYS_BUSCTRL_OFFSET: u32 = 5;
    pub const CLK_SYS_BUSCTRL_MASK: u32 = 0x00000020;
    pub const CLK_SYS_BUSCTRL: u32 = 1 << 5;

    /// **CLK_SYS_BOOTRAM**
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CLK_SYS_BOOTRAM_OFFSET: u32 = 4;
    pub const CLK_SYS_BOOTRAM_MASK: u32 = 0x00000010;
    pub const CLK_SYS_BOOTRAM: u32 = 1 << 4;

    /// **CLK_SYS_ADC**
    /// Bits `[3:3]`, mask `0x00000008`
    pub const CLK_SYS_ADC_OFFSET: u32 = 3;
    pub const CLK_SYS_ADC_MASK: u32 = 0x00000008;
    pub const CLK_SYS_ADC: u32 = 1 << 3;

    /// **CLK_ADC**
    /// Bits `[2:2]`, mask `0x00000004`
    pub const CLK_ADC_OFFSET: u32 = 2;
    pub const CLK_ADC_MASK: u32 = 0x00000004;
    pub const CLK_ADC: u32 = 1 << 2;

    /// **CLK_SYS_ACCESSCTRL**
    /// Bits `[1:1]`, mask `0x00000002`
    pub const CLK_SYS_ACCESSCTRL_OFFSET: u32 = 1;
    pub const CLK_SYS_ACCESSCTRL_MASK: u32 = 0x00000002;
    pub const CLK_SYS_ACCESSCTRL: u32 = 1 << 1;

    /// **CLK_SYS_CLOCKS**
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CLK_SYS_CLOCKS_OFFSET: u32 = 0;
    pub const CLK_SYS_CLOCKS_MASK: u32 = 0x00000001;
    pub const CLK_SYS_CLOCKS: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CLK_SYS_SIO** `[31]` (RO)
    #[inline(always)]
    pub fn clk_sys_sio(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_SIO_MASK) != 0
    }

    /// **CLK_SYS_SHA256** `[30]` (RO)
    #[inline(always)]
    pub fn clk_sys_sha256(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_SHA256_MASK) != 0
    }

    /// **CLK_SYS_PSM** `[29]` (RO)
    #[inline(always)]
    pub fn clk_sys_psm(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_PSM_MASK) != 0
    }

    /// **CLK_SYS_ROSC** `[28]` (RO)
    #[inline(always)]
    pub fn clk_sys_rosc(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_ROSC_MASK) != 0
    }

    /// **CLK_SYS_ROM** `[27]` (RO)
    #[inline(always)]
    pub fn clk_sys_rom(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_ROM_MASK) != 0
    }

    /// **CLK_SYS_RESETS** `[26]` (RO)
    #[inline(always)]
    pub fn clk_sys_resets(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_RESETS_MASK) != 0
    }

    /// **CLK_SYS_PWM** `[25]` (RO)
    #[inline(always)]
    pub fn clk_sys_pwm(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_PWM_MASK) != 0
    }

    /// **CLK_SYS_POWMAN** `[24]` (RO)
    #[inline(always)]
    pub fn clk_sys_powman(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_POWMAN_MASK) != 0
    }

    /// **CLK_REF_POWMAN** `[23]` (RO)
    #[inline(always)]
    pub fn clk_ref_powman(&self) -> bool {
        (self.value.read_bits() & Self::CLK_REF_POWMAN_MASK) != 0
    }

    /// **CLK_SYS_PLL_USB** `[22]` (RO)
    #[inline(always)]
    pub fn clk_sys_pll_usb(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_PLL_USB_MASK) != 0
    }

    /// **CLK_SYS_PLL_SYS** `[21]` (RO)
    #[inline(always)]
    pub fn clk_sys_pll_sys(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_PLL_SYS_MASK) != 0
    }

    /// **CLK_SYS_PIO2** `[20]` (RO)
    #[inline(always)]
    pub fn clk_sys_pio2(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_PIO2_MASK) != 0
    }

    /// **CLK_SYS_PIO1** `[19]` (RO)
    #[inline(always)]
    pub fn clk_sys_pio1(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_PIO1_MASK) != 0
    }

    /// **CLK_SYS_PIO0** `[18]` (RO)
    #[inline(always)]
    pub fn clk_sys_pio0(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_PIO0_MASK) != 0
    }

    /// **CLK_SYS_PADS** `[17]` (RO)
    #[inline(always)]
    pub fn clk_sys_pads(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_PADS_MASK) != 0
    }

    /// **CLK_SYS_OTP** `[16]` (RO)
    #[inline(always)]
    pub fn clk_sys_otp(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_OTP_MASK) != 0
    }

    /// **CLK_REF_OTP** `[15]` (RO)
    #[inline(always)]
    pub fn clk_ref_otp(&self) -> bool {
        (self.value.read_bits() & Self::CLK_REF_OTP_MASK) != 0
    }

    /// **CLK_SYS_JTAG** `[14]` (RO)
    #[inline(always)]
    pub fn clk_sys_jtag(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_JTAG_MASK) != 0
    }

    /// **CLK_SYS_IO** `[13]` (RO)
    #[inline(always)]
    pub fn clk_sys_io(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_IO_MASK) != 0
    }

    /// **CLK_SYS_I2C1** `[12]` (RO)
    #[inline(always)]
    pub fn clk_sys_i2c1(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_I2C1_MASK) != 0
    }

    /// **CLK_SYS_I2C0** `[11]` (RO)
    #[inline(always)]
    pub fn clk_sys_i2c0(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_I2C0_MASK) != 0
    }

    /// **CLK_SYS_HSTX** `[10]` (RO)
    #[inline(always)]
    pub fn clk_sys_hstx(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_HSTX_MASK) != 0
    }

    /// **CLK_HSTX** `[9]` (RO)
    #[inline(always)]
    pub fn clk_hstx(&self) -> bool {
        (self.value.read_bits() & Self::CLK_HSTX_MASK) != 0
    }

    /// **CLK_SYS_GLITCH_DETECTOR** `[8]` (RO)
    #[inline(always)]
    pub fn clk_sys_glitch_detector(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_GLITCH_DETECTOR_MASK) != 0
    }

    /// **CLK_SYS_DMA** `[7]` (RO)
    #[inline(always)]
    pub fn clk_sys_dma(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_DMA_MASK) != 0
    }

    /// **CLK_SYS_BUSFABRIC** `[6]` (RO)
    #[inline(always)]
    pub fn clk_sys_busfabric(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_BUSFABRIC_MASK) != 0
    }

    /// **CLK_SYS_BUSCTRL** `[5]` (RO)
    #[inline(always)]
    pub fn clk_sys_busctrl(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_BUSCTRL_MASK) != 0
    }

    /// **CLK_SYS_BOOTRAM** `[4]` (RO)
    #[inline(always)]
    pub fn clk_sys_bootram(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_BOOTRAM_MASK) != 0
    }

    /// **CLK_SYS_ADC** `[3]` (RO)
    #[inline(always)]
    pub fn clk_sys_adc(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_ADC_MASK) != 0
    }

    /// **CLK_ADC** `[2]` (RO)
    #[inline(always)]
    pub fn clk_adc(&self) -> bool {
        (self.value.read_bits() & Self::CLK_ADC_MASK) != 0
    }

    /// **CLK_SYS_ACCESSCTRL** `[1]` (RO)
    #[inline(always)]
    pub fn clk_sys_accessctrl(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_ACCESSCTRL_MASK) != 0
    }

    /// **CLK_SYS_CLOCKS** `[0]` (RO)
    #[inline(always)]
    pub fn clk_sys_clocks(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_CLOCKS_MASK) != 0
    }
}

/// ENABLED0 value type for batch read/modify/write operations
///
/// Use `enabled0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `enabled0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Enabled0Val {
    bits: u32,
}

impl Enabled0Val {
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

    /// Read **CLK_SYS_SIO** `[31:31]`
    #[inline(always)]
    pub const fn clk_sys_sio(&self) -> bool {
        (self.bits & Enabled0::CLK_SYS_SIO_MASK) != 0
    }

    /// Read **CLK_SYS_SHA256** `[30:30]`
    #[inline(always)]
    pub const fn clk_sys_sha256(&self) -> bool {
        (self.bits & Enabled0::CLK_SYS_SHA256_MASK) != 0
    }

    /// Read **CLK_SYS_PSM** `[29:29]`
    #[inline(always)]
    pub const fn clk_sys_psm(&self) -> bool {
        (self.bits & Enabled0::CLK_SYS_PSM_MASK) != 0
    }

    /// Read **CLK_SYS_ROSC** `[28:28]`
    #[inline(always)]
    pub const fn clk_sys_rosc(&self) -> bool {
        (self.bits & Enabled0::CLK_SYS_ROSC_MASK) != 0
    }

    /// Read **CLK_SYS_ROM** `[27:27]`
    #[inline(always)]
    pub const fn clk_sys_rom(&self) -> bool {
        (self.bits & Enabled0::CLK_SYS_ROM_MASK) != 0
    }

    /// Read **CLK_SYS_RESETS** `[26:26]`
    #[inline(always)]
    pub const fn clk_sys_resets(&self) -> bool {
        (self.bits & Enabled0::CLK_SYS_RESETS_MASK) != 0
    }

    /// Read **CLK_SYS_PWM** `[25:25]`
    #[inline(always)]
    pub const fn clk_sys_pwm(&self) -> bool {
        (self.bits & Enabled0::CLK_SYS_PWM_MASK) != 0
    }

    /// Read **CLK_SYS_POWMAN** `[24:24]`
    #[inline(always)]
    pub const fn clk_sys_powman(&self) -> bool {
        (self.bits & Enabled0::CLK_SYS_POWMAN_MASK) != 0
    }

    /// Read **CLK_REF_POWMAN** `[23:23]`
    #[inline(always)]
    pub const fn clk_ref_powman(&self) -> bool {
        (self.bits & Enabled0::CLK_REF_POWMAN_MASK) != 0
    }

    /// Read **CLK_SYS_PLL_USB** `[22:22]`
    #[inline(always)]
    pub const fn clk_sys_pll_usb(&self) -> bool {
        (self.bits & Enabled0::CLK_SYS_PLL_USB_MASK) != 0
    }

    /// Read **CLK_SYS_PLL_SYS** `[21:21]`
    #[inline(always)]
    pub const fn clk_sys_pll_sys(&self) -> bool {
        (self.bits & Enabled0::CLK_SYS_PLL_SYS_MASK) != 0
    }

    /// Read **CLK_SYS_PIO2** `[20:20]`
    #[inline(always)]
    pub const fn clk_sys_pio2(&self) -> bool {
        (self.bits & Enabled0::CLK_SYS_PIO2_MASK) != 0
    }

    /// Read **CLK_SYS_PIO1** `[19:19]`
    #[inline(always)]
    pub const fn clk_sys_pio1(&self) -> bool {
        (self.bits & Enabled0::CLK_SYS_PIO1_MASK) != 0
    }

    /// Read **CLK_SYS_PIO0** `[18:18]`
    #[inline(always)]
    pub const fn clk_sys_pio0(&self) -> bool {
        (self.bits & Enabled0::CLK_SYS_PIO0_MASK) != 0
    }

    /// Read **CLK_SYS_PADS** `[17:17]`
    #[inline(always)]
    pub const fn clk_sys_pads(&self) -> bool {
        (self.bits & Enabled0::CLK_SYS_PADS_MASK) != 0
    }

    /// Read **CLK_SYS_OTP** `[16:16]`
    #[inline(always)]
    pub const fn clk_sys_otp(&self) -> bool {
        (self.bits & Enabled0::CLK_SYS_OTP_MASK) != 0
    }

    /// Read **CLK_REF_OTP** `[15:15]`
    #[inline(always)]
    pub const fn clk_ref_otp(&self) -> bool {
        (self.bits & Enabled0::CLK_REF_OTP_MASK) != 0
    }

    /// Read **CLK_SYS_JTAG** `[14:14]`
    #[inline(always)]
    pub const fn clk_sys_jtag(&self) -> bool {
        (self.bits & Enabled0::CLK_SYS_JTAG_MASK) != 0
    }

    /// Read **CLK_SYS_IO** `[13:13]`
    #[inline(always)]
    pub const fn clk_sys_io(&self) -> bool {
        (self.bits & Enabled0::CLK_SYS_IO_MASK) != 0
    }

    /// Read **CLK_SYS_I2C1** `[12:12]`
    #[inline(always)]
    pub const fn clk_sys_i2c1(&self) -> bool {
        (self.bits & Enabled0::CLK_SYS_I2C1_MASK) != 0
    }

    /// Read **CLK_SYS_I2C0** `[11:11]`
    #[inline(always)]
    pub const fn clk_sys_i2c0(&self) -> bool {
        (self.bits & Enabled0::CLK_SYS_I2C0_MASK) != 0
    }

    /// Read **CLK_SYS_HSTX** `[10:10]`
    #[inline(always)]
    pub const fn clk_sys_hstx(&self) -> bool {
        (self.bits & Enabled0::CLK_SYS_HSTX_MASK) != 0
    }

    /// Read **CLK_HSTX** `[9:9]`
    #[inline(always)]
    pub const fn clk_hstx(&self) -> bool {
        (self.bits & Enabled0::CLK_HSTX_MASK) != 0
    }

    /// Read **CLK_SYS_GLITCH_DETECTOR** `[8:8]`
    #[inline(always)]
    pub const fn clk_sys_glitch_detector(&self) -> bool {
        (self.bits & Enabled0::CLK_SYS_GLITCH_DETECTOR_MASK) != 0
    }

    /// Read **CLK_SYS_DMA** `[7:7]`
    #[inline(always)]
    pub const fn clk_sys_dma(&self) -> bool {
        (self.bits & Enabled0::CLK_SYS_DMA_MASK) != 0
    }

    /// Read **CLK_SYS_BUSFABRIC** `[6:6]`
    #[inline(always)]
    pub const fn clk_sys_busfabric(&self) -> bool {
        (self.bits & Enabled0::CLK_SYS_BUSFABRIC_MASK) != 0
    }

    /// Read **CLK_SYS_BUSCTRL** `[5:5]`
    #[inline(always)]
    pub const fn clk_sys_busctrl(&self) -> bool {
        (self.bits & Enabled0::CLK_SYS_BUSCTRL_MASK) != 0
    }

    /// Read **CLK_SYS_BOOTRAM** `[4:4]`
    #[inline(always)]
    pub const fn clk_sys_bootram(&self) -> bool {
        (self.bits & Enabled0::CLK_SYS_BOOTRAM_MASK) != 0
    }

    /// Read **CLK_SYS_ADC** `[3:3]`
    #[inline(always)]
    pub const fn clk_sys_adc(&self) -> bool {
        (self.bits & Enabled0::CLK_SYS_ADC_MASK) != 0
    }

    /// Read **CLK_ADC** `[2:2]`
    #[inline(always)]
    pub const fn clk_adc(&self) -> bool {
        (self.bits & Enabled0::CLK_ADC_MASK) != 0
    }

    /// Read **CLK_SYS_ACCESSCTRL** `[1:1]`
    #[inline(always)]
    pub const fn clk_sys_accessctrl(&self) -> bool {
        (self.bits & Enabled0::CLK_SYS_ACCESSCTRL_MASK) != 0
    }

    /// Read **CLK_SYS_CLOCKS** `[0:0]`
    #[inline(always)]
    pub const fn clk_sys_clocks(&self) -> bool {
        (self.bits & Enabled0::CLK_SYS_CLOCKS_MASK) != 0
    }
}

impl Default for Enabled0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Enabled0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Enabled0Val {
        Enabled0Val::from_bits(self.value.read_bits())
    }
}

/// ENABLED1 Register
///
/// indicates the state of the clock enable
///
/// Address offset: 0x00c0
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CLK_SYS_XOSC** `[30]` (RO)
///
/// - **CLK_SYS_XIP** `[29]` (RO)
///
/// - **CLK_SYS_WATCHDOG** `[28]` (RO)
///
/// - **CLK_USB** `[27]` (RO)
///
/// - **CLK_SYS_USBCTRL** `[26]` (RO)
///
/// - **CLK_SYS_UART1** `[25]` (RO)
///
/// - **CLK_PERI_UART1** `[24]` (RO)
///
/// - **CLK_SYS_UART0** `[23]` (RO)
///
/// - **CLK_PERI_UART0** `[22]` (RO)
///
/// - **CLK_SYS_TRNG** `[21]` (RO)
///
/// - **CLK_SYS_TIMER1** `[20]` (RO)
///
/// - **CLK_SYS_TIMER0** `[19]` (RO)
///
/// - **CLK_SYS_TICKS** `[18]` (RO)
///
/// - **CLK_REF_TICKS** `[17]` (RO)
///
/// - **CLK_SYS_TBMAN** `[16]` (RO)
///
/// - **CLK_SYS_SYSINFO** `[15]` (RO)
///
/// - **CLK_SYS_SYSCFG** `[14]` (RO)
///
/// - **CLK_SYS_SRAM9** `[13]` (RO)
///
/// - **CLK_SYS_SRAM8** `[12]` (RO)
///
/// - **CLK_SYS_SRAM7** `[11]` (RO)
///
/// - **CLK_SYS_SRAM6** `[10]` (RO)
///
/// - **CLK_SYS_SRAM5** `[9]` (RO)
///
/// - **CLK_SYS_SRAM4** `[8]` (RO)
///
/// - **CLK_SYS_SRAM3** `[7]` (RO)
///
/// - **CLK_SYS_SRAM2** `[6]` (RO)
///
/// - **CLK_SYS_SRAM1** `[5]` (RO)
///
/// - **CLK_SYS_SRAM0** `[4]` (RO)
///
/// - **CLK_SYS_SPI1** `[3]` (RO)
///
/// - **CLK_PERI_SPI1** `[2]` (RO)
///
/// - **CLK_SYS_SPI0** `[1]` (RO)
///
/// - **CLK_PERI_SPI0** `[0]` (RO)
#[repr(transparent)]
pub struct Enabled1 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Enabled1 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Enabled1 {
    // ======== Field Constants ========

    /// **CLK_SYS_XOSC**
    /// Bits `[30:30]`, mask `0x40000000`
    pub const CLK_SYS_XOSC_OFFSET: u32 = 30;
    pub const CLK_SYS_XOSC_MASK: u32 = 0x40000000;
    pub const CLK_SYS_XOSC: u32 = 1 << 30;

    /// **CLK_SYS_XIP**
    /// Bits `[29:29]`, mask `0x20000000`
    pub const CLK_SYS_XIP_OFFSET: u32 = 29;
    pub const CLK_SYS_XIP_MASK: u32 = 0x20000000;
    pub const CLK_SYS_XIP: u32 = 1 << 29;

    /// **CLK_SYS_WATCHDOG**
    /// Bits `[28:28]`, mask `0x10000000`
    pub const CLK_SYS_WATCHDOG_OFFSET: u32 = 28;
    pub const CLK_SYS_WATCHDOG_MASK: u32 = 0x10000000;
    pub const CLK_SYS_WATCHDOG: u32 = 1 << 28;

    /// **CLK_USB**
    /// Bits `[27:27]`, mask `0x08000000`
    pub const CLK_USB_OFFSET: u32 = 27;
    pub const CLK_USB_MASK: u32 = 0x08000000;
    pub const CLK_USB: u32 = 1 << 27;

    /// **CLK_SYS_USBCTRL**
    /// Bits `[26:26]`, mask `0x04000000`
    pub const CLK_SYS_USBCTRL_OFFSET: u32 = 26;
    pub const CLK_SYS_USBCTRL_MASK: u32 = 0x04000000;
    pub const CLK_SYS_USBCTRL: u32 = 1 << 26;

    /// **CLK_SYS_UART1**
    /// Bits `[25:25]`, mask `0x02000000`
    pub const CLK_SYS_UART1_OFFSET: u32 = 25;
    pub const CLK_SYS_UART1_MASK: u32 = 0x02000000;
    pub const CLK_SYS_UART1: u32 = 1 << 25;

    /// **CLK_PERI_UART1**
    /// Bits `[24:24]`, mask `0x01000000`
    pub const CLK_PERI_UART1_OFFSET: u32 = 24;
    pub const CLK_PERI_UART1_MASK: u32 = 0x01000000;
    pub const CLK_PERI_UART1: u32 = 1 << 24;

    /// **CLK_SYS_UART0**
    /// Bits `[23:23]`, mask `0x00800000`
    pub const CLK_SYS_UART0_OFFSET: u32 = 23;
    pub const CLK_SYS_UART0_MASK: u32 = 0x00800000;
    pub const CLK_SYS_UART0: u32 = 1 << 23;

    /// **CLK_PERI_UART0**
    /// Bits `[22:22]`, mask `0x00400000`
    pub const CLK_PERI_UART0_OFFSET: u32 = 22;
    pub const CLK_PERI_UART0_MASK: u32 = 0x00400000;
    pub const CLK_PERI_UART0: u32 = 1 << 22;

    /// **CLK_SYS_TRNG**
    /// Bits `[21:21]`, mask `0x00200000`
    pub const CLK_SYS_TRNG_OFFSET: u32 = 21;
    pub const CLK_SYS_TRNG_MASK: u32 = 0x00200000;
    pub const CLK_SYS_TRNG: u32 = 1 << 21;

    /// **CLK_SYS_TIMER1**
    /// Bits `[20:20]`, mask `0x00100000`
    pub const CLK_SYS_TIMER1_OFFSET: u32 = 20;
    pub const CLK_SYS_TIMER1_MASK: u32 = 0x00100000;
    pub const CLK_SYS_TIMER1: u32 = 1 << 20;

    /// **CLK_SYS_TIMER0**
    /// Bits `[19:19]`, mask `0x00080000`
    pub const CLK_SYS_TIMER0_OFFSET: u32 = 19;
    pub const CLK_SYS_TIMER0_MASK: u32 = 0x00080000;
    pub const CLK_SYS_TIMER0: u32 = 1 << 19;

    /// **CLK_SYS_TICKS**
    /// Bits `[18:18]`, mask `0x00040000`
    pub const CLK_SYS_TICKS_OFFSET: u32 = 18;
    pub const CLK_SYS_TICKS_MASK: u32 = 0x00040000;
    pub const CLK_SYS_TICKS: u32 = 1 << 18;

    /// **CLK_REF_TICKS**
    /// Bits `[17:17]`, mask `0x00020000`
    pub const CLK_REF_TICKS_OFFSET: u32 = 17;
    pub const CLK_REF_TICKS_MASK: u32 = 0x00020000;
    pub const CLK_REF_TICKS: u32 = 1 << 17;

    /// **CLK_SYS_TBMAN**
    /// Bits `[16:16]`, mask `0x00010000`
    pub const CLK_SYS_TBMAN_OFFSET: u32 = 16;
    pub const CLK_SYS_TBMAN_MASK: u32 = 0x00010000;
    pub const CLK_SYS_TBMAN: u32 = 1 << 16;

    /// **CLK_SYS_SYSINFO**
    /// Bits `[15:15]`, mask `0x00008000`
    pub const CLK_SYS_SYSINFO_OFFSET: u32 = 15;
    pub const CLK_SYS_SYSINFO_MASK: u32 = 0x00008000;
    pub const CLK_SYS_SYSINFO: u32 = 1 << 15;

    /// **CLK_SYS_SYSCFG**
    /// Bits `[14:14]`, mask `0x00004000`
    pub const CLK_SYS_SYSCFG_OFFSET: u32 = 14;
    pub const CLK_SYS_SYSCFG_MASK: u32 = 0x00004000;
    pub const CLK_SYS_SYSCFG: u32 = 1 << 14;

    /// **CLK_SYS_SRAM9**
    /// Bits `[13:13]`, mask `0x00002000`
    pub const CLK_SYS_SRAM9_OFFSET: u32 = 13;
    pub const CLK_SYS_SRAM9_MASK: u32 = 0x00002000;
    pub const CLK_SYS_SRAM9: u32 = 1 << 13;

    /// **CLK_SYS_SRAM8**
    /// Bits `[12:12]`, mask `0x00001000`
    pub const CLK_SYS_SRAM8_OFFSET: u32 = 12;
    pub const CLK_SYS_SRAM8_MASK: u32 = 0x00001000;
    pub const CLK_SYS_SRAM8: u32 = 1 << 12;

    /// **CLK_SYS_SRAM7**
    /// Bits `[11:11]`, mask `0x00000800`
    pub const CLK_SYS_SRAM7_OFFSET: u32 = 11;
    pub const CLK_SYS_SRAM7_MASK: u32 = 0x00000800;
    pub const CLK_SYS_SRAM7: u32 = 1 << 11;

    /// **CLK_SYS_SRAM6**
    /// Bits `[10:10]`, mask `0x00000400`
    pub const CLK_SYS_SRAM6_OFFSET: u32 = 10;
    pub const CLK_SYS_SRAM6_MASK: u32 = 0x00000400;
    pub const CLK_SYS_SRAM6: u32 = 1 << 10;

    /// **CLK_SYS_SRAM5**
    /// Bits `[9:9]`, mask `0x00000200`
    pub const CLK_SYS_SRAM5_OFFSET: u32 = 9;
    pub const CLK_SYS_SRAM5_MASK: u32 = 0x00000200;
    pub const CLK_SYS_SRAM5: u32 = 1 << 9;

    /// **CLK_SYS_SRAM4**
    /// Bits `[8:8]`, mask `0x00000100`
    pub const CLK_SYS_SRAM4_OFFSET: u32 = 8;
    pub const CLK_SYS_SRAM4_MASK: u32 = 0x00000100;
    pub const CLK_SYS_SRAM4: u32 = 1 << 8;

    /// **CLK_SYS_SRAM3**
    /// Bits `[7:7]`, mask `0x00000080`
    pub const CLK_SYS_SRAM3_OFFSET: u32 = 7;
    pub const CLK_SYS_SRAM3_MASK: u32 = 0x00000080;
    pub const CLK_SYS_SRAM3: u32 = 1 << 7;

    /// **CLK_SYS_SRAM2**
    /// Bits `[6:6]`, mask `0x00000040`
    pub const CLK_SYS_SRAM2_OFFSET: u32 = 6;
    pub const CLK_SYS_SRAM2_MASK: u32 = 0x00000040;
    pub const CLK_SYS_SRAM2: u32 = 1 << 6;

    /// **CLK_SYS_SRAM1**
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CLK_SYS_SRAM1_OFFSET: u32 = 5;
    pub const CLK_SYS_SRAM1_MASK: u32 = 0x00000020;
    pub const CLK_SYS_SRAM1: u32 = 1 << 5;

    /// **CLK_SYS_SRAM0**
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CLK_SYS_SRAM0_OFFSET: u32 = 4;
    pub const CLK_SYS_SRAM0_MASK: u32 = 0x00000010;
    pub const CLK_SYS_SRAM0: u32 = 1 << 4;

    /// **CLK_SYS_SPI1**
    /// Bits `[3:3]`, mask `0x00000008`
    pub const CLK_SYS_SPI1_OFFSET: u32 = 3;
    pub const CLK_SYS_SPI1_MASK: u32 = 0x00000008;
    pub const CLK_SYS_SPI1: u32 = 1 << 3;

    /// **CLK_PERI_SPI1**
    /// Bits `[2:2]`, mask `0x00000004`
    pub const CLK_PERI_SPI1_OFFSET: u32 = 2;
    pub const CLK_PERI_SPI1_MASK: u32 = 0x00000004;
    pub const CLK_PERI_SPI1: u32 = 1 << 2;

    /// **CLK_SYS_SPI0**
    /// Bits `[1:1]`, mask `0x00000002`
    pub const CLK_SYS_SPI0_OFFSET: u32 = 1;
    pub const CLK_SYS_SPI0_MASK: u32 = 0x00000002;
    pub const CLK_SYS_SPI0: u32 = 1 << 1;

    /// **CLK_PERI_SPI0**
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CLK_PERI_SPI0_OFFSET: u32 = 0;
    pub const CLK_PERI_SPI0_MASK: u32 = 0x00000001;
    pub const CLK_PERI_SPI0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CLK_SYS_XOSC** `[30]` (RO)
    #[inline(always)]
    pub fn clk_sys_xosc(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_XOSC_MASK) != 0
    }

    /// **CLK_SYS_XIP** `[29]` (RO)
    #[inline(always)]
    pub fn clk_sys_xip(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_XIP_MASK) != 0
    }

    /// **CLK_SYS_WATCHDOG** `[28]` (RO)
    #[inline(always)]
    pub fn clk_sys_watchdog(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_WATCHDOG_MASK) != 0
    }

    /// **CLK_USB** `[27]` (RO)
    #[inline(always)]
    pub fn clk_usb(&self) -> bool {
        (self.value.read_bits() & Self::CLK_USB_MASK) != 0
    }

    /// **CLK_SYS_USBCTRL** `[26]` (RO)
    #[inline(always)]
    pub fn clk_sys_usbctrl(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_USBCTRL_MASK) != 0
    }

    /// **CLK_SYS_UART1** `[25]` (RO)
    #[inline(always)]
    pub fn clk_sys_uart1(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_UART1_MASK) != 0
    }

    /// **CLK_PERI_UART1** `[24]` (RO)
    #[inline(always)]
    pub fn clk_peri_uart1(&self) -> bool {
        (self.value.read_bits() & Self::CLK_PERI_UART1_MASK) != 0
    }

    /// **CLK_SYS_UART0** `[23]` (RO)
    #[inline(always)]
    pub fn clk_sys_uart0(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_UART0_MASK) != 0
    }

    /// **CLK_PERI_UART0** `[22]` (RO)
    #[inline(always)]
    pub fn clk_peri_uart0(&self) -> bool {
        (self.value.read_bits() & Self::CLK_PERI_UART0_MASK) != 0
    }

    /// **CLK_SYS_TRNG** `[21]` (RO)
    #[inline(always)]
    pub fn clk_sys_trng(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_TRNG_MASK) != 0
    }

    /// **CLK_SYS_TIMER1** `[20]` (RO)
    #[inline(always)]
    pub fn clk_sys_timer1(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_TIMER1_MASK) != 0
    }

    /// **CLK_SYS_TIMER0** `[19]` (RO)
    #[inline(always)]
    pub fn clk_sys_timer0(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_TIMER0_MASK) != 0
    }

    /// **CLK_SYS_TICKS** `[18]` (RO)
    #[inline(always)]
    pub fn clk_sys_ticks(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_TICKS_MASK) != 0
    }

    /// **CLK_REF_TICKS** `[17]` (RO)
    #[inline(always)]
    pub fn clk_ref_ticks(&self) -> bool {
        (self.value.read_bits() & Self::CLK_REF_TICKS_MASK) != 0
    }

    /// **CLK_SYS_TBMAN** `[16]` (RO)
    #[inline(always)]
    pub fn clk_sys_tbman(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_TBMAN_MASK) != 0
    }

    /// **CLK_SYS_SYSINFO** `[15]` (RO)
    #[inline(always)]
    pub fn clk_sys_sysinfo(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_SYSINFO_MASK) != 0
    }

    /// **CLK_SYS_SYSCFG** `[14]` (RO)
    #[inline(always)]
    pub fn clk_sys_syscfg(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_SYSCFG_MASK) != 0
    }

    /// **CLK_SYS_SRAM9** `[13]` (RO)
    #[inline(always)]
    pub fn clk_sys_sram9(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_SRAM9_MASK) != 0
    }

    /// **CLK_SYS_SRAM8** `[12]` (RO)
    #[inline(always)]
    pub fn clk_sys_sram8(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_SRAM8_MASK) != 0
    }

    /// **CLK_SYS_SRAM7** `[11]` (RO)
    #[inline(always)]
    pub fn clk_sys_sram7(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_SRAM7_MASK) != 0
    }

    /// **CLK_SYS_SRAM6** `[10]` (RO)
    #[inline(always)]
    pub fn clk_sys_sram6(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_SRAM6_MASK) != 0
    }

    /// **CLK_SYS_SRAM5** `[9]` (RO)
    #[inline(always)]
    pub fn clk_sys_sram5(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_SRAM5_MASK) != 0
    }

    /// **CLK_SYS_SRAM4** `[8]` (RO)
    #[inline(always)]
    pub fn clk_sys_sram4(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_SRAM4_MASK) != 0
    }

    /// **CLK_SYS_SRAM3** `[7]` (RO)
    #[inline(always)]
    pub fn clk_sys_sram3(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_SRAM3_MASK) != 0
    }

    /// **CLK_SYS_SRAM2** `[6]` (RO)
    #[inline(always)]
    pub fn clk_sys_sram2(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_SRAM2_MASK) != 0
    }

    /// **CLK_SYS_SRAM1** `[5]` (RO)
    #[inline(always)]
    pub fn clk_sys_sram1(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_SRAM1_MASK) != 0
    }

    /// **CLK_SYS_SRAM0** `[4]` (RO)
    #[inline(always)]
    pub fn clk_sys_sram0(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_SRAM0_MASK) != 0
    }

    /// **CLK_SYS_SPI1** `[3]` (RO)
    #[inline(always)]
    pub fn clk_sys_spi1(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_SPI1_MASK) != 0
    }

    /// **CLK_PERI_SPI1** `[2]` (RO)
    #[inline(always)]
    pub fn clk_peri_spi1(&self) -> bool {
        (self.value.read_bits() & Self::CLK_PERI_SPI1_MASK) != 0
    }

    /// **CLK_SYS_SPI0** `[1]` (RO)
    #[inline(always)]
    pub fn clk_sys_spi0(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_SPI0_MASK) != 0
    }

    /// **CLK_PERI_SPI0** `[0]` (RO)
    #[inline(always)]
    pub fn clk_peri_spi0(&self) -> bool {
        (self.value.read_bits() & Self::CLK_PERI_SPI0_MASK) != 0
    }
}

/// ENABLED1 value type for batch read/modify/write operations
///
/// Use `enabled1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `enabled1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Enabled1Val {
    bits: u32,
}

impl Enabled1Val {
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

    /// Read **CLK_SYS_XOSC** `[30:30]`
    #[inline(always)]
    pub const fn clk_sys_xosc(&self) -> bool {
        (self.bits & Enabled1::CLK_SYS_XOSC_MASK) != 0
    }

    /// Read **CLK_SYS_XIP** `[29:29]`
    #[inline(always)]
    pub const fn clk_sys_xip(&self) -> bool {
        (self.bits & Enabled1::CLK_SYS_XIP_MASK) != 0
    }

    /// Read **CLK_SYS_WATCHDOG** `[28:28]`
    #[inline(always)]
    pub const fn clk_sys_watchdog(&self) -> bool {
        (self.bits & Enabled1::CLK_SYS_WATCHDOG_MASK) != 0
    }

    /// Read **CLK_USB** `[27:27]`
    #[inline(always)]
    pub const fn clk_usb(&self) -> bool {
        (self.bits & Enabled1::CLK_USB_MASK) != 0
    }

    /// Read **CLK_SYS_USBCTRL** `[26:26]`
    #[inline(always)]
    pub const fn clk_sys_usbctrl(&self) -> bool {
        (self.bits & Enabled1::CLK_SYS_USBCTRL_MASK) != 0
    }

    /// Read **CLK_SYS_UART1** `[25:25]`
    #[inline(always)]
    pub const fn clk_sys_uart1(&self) -> bool {
        (self.bits & Enabled1::CLK_SYS_UART1_MASK) != 0
    }

    /// Read **CLK_PERI_UART1** `[24:24]`
    #[inline(always)]
    pub const fn clk_peri_uart1(&self) -> bool {
        (self.bits & Enabled1::CLK_PERI_UART1_MASK) != 0
    }

    /// Read **CLK_SYS_UART0** `[23:23]`
    #[inline(always)]
    pub const fn clk_sys_uart0(&self) -> bool {
        (self.bits & Enabled1::CLK_SYS_UART0_MASK) != 0
    }

    /// Read **CLK_PERI_UART0** `[22:22]`
    #[inline(always)]
    pub const fn clk_peri_uart0(&self) -> bool {
        (self.bits & Enabled1::CLK_PERI_UART0_MASK) != 0
    }

    /// Read **CLK_SYS_TRNG** `[21:21]`
    #[inline(always)]
    pub const fn clk_sys_trng(&self) -> bool {
        (self.bits & Enabled1::CLK_SYS_TRNG_MASK) != 0
    }

    /// Read **CLK_SYS_TIMER1** `[20:20]`
    #[inline(always)]
    pub const fn clk_sys_timer1(&self) -> bool {
        (self.bits & Enabled1::CLK_SYS_TIMER1_MASK) != 0
    }

    /// Read **CLK_SYS_TIMER0** `[19:19]`
    #[inline(always)]
    pub const fn clk_sys_timer0(&self) -> bool {
        (self.bits & Enabled1::CLK_SYS_TIMER0_MASK) != 0
    }

    /// Read **CLK_SYS_TICKS** `[18:18]`
    #[inline(always)]
    pub const fn clk_sys_ticks(&self) -> bool {
        (self.bits & Enabled1::CLK_SYS_TICKS_MASK) != 0
    }

    /// Read **CLK_REF_TICKS** `[17:17]`
    #[inline(always)]
    pub const fn clk_ref_ticks(&self) -> bool {
        (self.bits & Enabled1::CLK_REF_TICKS_MASK) != 0
    }

    /// Read **CLK_SYS_TBMAN** `[16:16]`
    #[inline(always)]
    pub const fn clk_sys_tbman(&self) -> bool {
        (self.bits & Enabled1::CLK_SYS_TBMAN_MASK) != 0
    }

    /// Read **CLK_SYS_SYSINFO** `[15:15]`
    #[inline(always)]
    pub const fn clk_sys_sysinfo(&self) -> bool {
        (self.bits & Enabled1::CLK_SYS_SYSINFO_MASK) != 0
    }

    /// Read **CLK_SYS_SYSCFG** `[14:14]`
    #[inline(always)]
    pub const fn clk_sys_syscfg(&self) -> bool {
        (self.bits & Enabled1::CLK_SYS_SYSCFG_MASK) != 0
    }

    /// Read **CLK_SYS_SRAM9** `[13:13]`
    #[inline(always)]
    pub const fn clk_sys_sram9(&self) -> bool {
        (self.bits & Enabled1::CLK_SYS_SRAM9_MASK) != 0
    }

    /// Read **CLK_SYS_SRAM8** `[12:12]`
    #[inline(always)]
    pub const fn clk_sys_sram8(&self) -> bool {
        (self.bits & Enabled1::CLK_SYS_SRAM8_MASK) != 0
    }

    /// Read **CLK_SYS_SRAM7** `[11:11]`
    #[inline(always)]
    pub const fn clk_sys_sram7(&self) -> bool {
        (self.bits & Enabled1::CLK_SYS_SRAM7_MASK) != 0
    }

    /// Read **CLK_SYS_SRAM6** `[10:10]`
    #[inline(always)]
    pub const fn clk_sys_sram6(&self) -> bool {
        (self.bits & Enabled1::CLK_SYS_SRAM6_MASK) != 0
    }

    /// Read **CLK_SYS_SRAM5** `[9:9]`
    #[inline(always)]
    pub const fn clk_sys_sram5(&self) -> bool {
        (self.bits & Enabled1::CLK_SYS_SRAM5_MASK) != 0
    }

    /// Read **CLK_SYS_SRAM4** `[8:8]`
    #[inline(always)]
    pub const fn clk_sys_sram4(&self) -> bool {
        (self.bits & Enabled1::CLK_SYS_SRAM4_MASK) != 0
    }

    /// Read **CLK_SYS_SRAM3** `[7:7]`
    #[inline(always)]
    pub const fn clk_sys_sram3(&self) -> bool {
        (self.bits & Enabled1::CLK_SYS_SRAM3_MASK) != 0
    }

    /// Read **CLK_SYS_SRAM2** `[6:6]`
    #[inline(always)]
    pub const fn clk_sys_sram2(&self) -> bool {
        (self.bits & Enabled1::CLK_SYS_SRAM2_MASK) != 0
    }

    /// Read **CLK_SYS_SRAM1** `[5:5]`
    #[inline(always)]
    pub const fn clk_sys_sram1(&self) -> bool {
        (self.bits & Enabled1::CLK_SYS_SRAM1_MASK) != 0
    }

    /// Read **CLK_SYS_SRAM0** `[4:4]`
    #[inline(always)]
    pub const fn clk_sys_sram0(&self) -> bool {
        (self.bits & Enabled1::CLK_SYS_SRAM0_MASK) != 0
    }

    /// Read **CLK_SYS_SPI1** `[3:3]`
    #[inline(always)]
    pub const fn clk_sys_spi1(&self) -> bool {
        (self.bits & Enabled1::CLK_SYS_SPI1_MASK) != 0
    }

    /// Read **CLK_PERI_SPI1** `[2:2]`
    #[inline(always)]
    pub const fn clk_peri_spi1(&self) -> bool {
        (self.bits & Enabled1::CLK_PERI_SPI1_MASK) != 0
    }

    /// Read **CLK_SYS_SPI0** `[1:1]`
    #[inline(always)]
    pub const fn clk_sys_spi0(&self) -> bool {
        (self.bits & Enabled1::CLK_SYS_SPI0_MASK) != 0
    }

    /// Read **CLK_PERI_SPI0** `[0:0]`
    #[inline(always)]
    pub const fn clk_peri_spi0(&self) -> bool {
        (self.bits & Enabled1::CLK_PERI_SPI0_MASK) != 0
    }
}

impl Default for Enabled1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Enabled1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Enabled1Val {
        Enabled1Val::from_bits(self.value.read_bits())
    }
}

/// INTR Register
///
/// Raw Interrupts
///
/// Address offset: 0x00c4
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CLK_SYS_RESUS** `[0]` (RO)
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

    /// **CLK_SYS_RESUS**
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CLK_SYS_RESUS_OFFSET: u32 = 0;
    pub const CLK_SYS_RESUS_MASK: u32 = 0x00000001;
    pub const CLK_SYS_RESUS: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CLK_SYS_RESUS** `[0]` (RO)
    #[inline(always)]
    pub fn clk_sys_resus(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_RESUS_MASK) != 0
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

    /// Read **CLK_SYS_RESUS** `[0:0]`
    #[inline(always)]
    pub const fn clk_sys_resus(&self) -> bool {
        (self.bits & Intr::CLK_SYS_RESUS_MASK) != 0
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
}

/// INTE Register
///
/// Interrupt Enable
///
/// Address offset: 0x00c8
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CLK_SYS_RESUS** `[0]` (RW)
#[repr(transparent)]
pub struct Inte {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Inte {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Inte {
    // ======== Field Constants ========

    /// **CLK_SYS_RESUS**
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CLK_SYS_RESUS_OFFSET: u32 = 0;
    pub const CLK_SYS_RESUS_MASK: u32 = 0x00000001;
    pub const CLK_SYS_RESUS: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CLK_SYS_RESUS** `[0]` (RW)
    #[inline(always)]
    pub fn clk_sys_resus(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_RESUS_MASK) != 0
    }

    /// Set CLK_SYS_RESUS bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_resus(&self) {
        self.set_bits(Self::CLK_SYS_RESUS_MASK);
    }

    /// Clear CLK_SYS_RESUS bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_resus(&self) {
        self.clr_bits(Self::CLK_SYS_RESUS_MASK);
    }
}

/// INTE value type for batch read/modify/write operations
///
/// Use `inte.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `inte.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InteVal {
    bits: u32,
}

impl InteVal {
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

    /// Read **CLK_SYS_RESUS** `[0:0]`
    #[inline(always)]
    pub const fn clk_sys_resus(&self) -> bool {
        (self.bits & Inte::CLK_SYS_RESUS_MASK) != 0
    }

    /// Set **CLK_SYS_RESUS** `[0:0]`
    #[inline(always)]
    pub const fn with_clk_sys_resus(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Inte::CLK_SYS_RESUS_MASK
            } else {
                self.bits & !Inte::CLK_SYS_RESUS_MASK
            }
        }
    }
}

impl Default for InteVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Inte {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InteVal {
        InteVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: InteVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(InteVal) -> InteVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// INTF Register
///
/// Interrupt Force
///
/// Address offset: 0x00cc
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CLK_SYS_RESUS** `[0]` (RW)
#[repr(transparent)]
pub struct Intf {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Intf {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Intf {
    // ======== Field Constants ========

    /// **CLK_SYS_RESUS**
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CLK_SYS_RESUS_OFFSET: u32 = 0;
    pub const CLK_SYS_RESUS_MASK: u32 = 0x00000001;
    pub const CLK_SYS_RESUS: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CLK_SYS_RESUS** `[0]` (RW)
    #[inline(always)]
    pub fn clk_sys_resus(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_RESUS_MASK) != 0
    }

    /// Set CLK_SYS_RESUS bit atomically (RW)
    #[inline(always)]
    pub fn set_clk_sys_resus(&self) {
        self.set_bits(Self::CLK_SYS_RESUS_MASK);
    }

    /// Clear CLK_SYS_RESUS bit atomically (RW)
    #[inline(always)]
    pub fn clr_clk_sys_resus(&self) {
        self.clr_bits(Self::CLK_SYS_RESUS_MASK);
    }
}

/// INTF value type for batch read/modify/write operations
///
/// Use `intf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `intf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntfVal {
    bits: u32,
}

impl IntfVal {
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

    /// Read **CLK_SYS_RESUS** `[0:0]`
    #[inline(always)]
    pub const fn clk_sys_resus(&self) -> bool {
        (self.bits & Intf::CLK_SYS_RESUS_MASK) != 0
    }

    /// Set **CLK_SYS_RESUS** `[0:0]`
    #[inline(always)]
    pub const fn with_clk_sys_resus(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intf::CLK_SYS_RESUS_MASK
            } else {
                self.bits & !Intf::CLK_SYS_RESUS_MASK
            }
        }
    }
}

impl Default for IntfVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Intf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IntfVal {
        IntfVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IntfVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IntfVal) -> IntfVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// INTS Register
///
/// Interrupt status after masking & forcing
///
/// Address offset: 0x00d0
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CLK_SYS_RESUS** `[0]` (RO)
#[repr(transparent)]
pub struct Ints {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Ints {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ints {
    // ======== Field Constants ========

    /// **CLK_SYS_RESUS**
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CLK_SYS_RESUS_OFFSET: u32 = 0;
    pub const CLK_SYS_RESUS_MASK: u32 = 0x00000001;
    pub const CLK_SYS_RESUS: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CLK_SYS_RESUS** `[0]` (RO)
    #[inline(always)]
    pub fn clk_sys_resus(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SYS_RESUS_MASK) != 0
    }
}

/// INTS value type for batch read/modify/write operations
///
/// Use `ints.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ints.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntsVal {
    bits: u32,
}

impl IntsVal {
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

    /// Read **CLK_SYS_RESUS** `[0:0]`
    #[inline(always)]
    pub const fn clk_sys_resus(&self) -> bool {
        (self.bits & Ints::CLK_SYS_RESUS_MASK) != 0
    }
}

impl Default for IntsVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ints {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IntsVal {
        IntsVal::from_bits(self.value.read_bits())
    }
}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// Clock control, can be changed on-the-fly (except for auxsrc)
    pub clk_gpout0_ctrl: ClkGpout0Ctrl,
    /// 0x0004
    pub clk_gpout0_div: ClkGpout0Div,
    /// 0x0008
    /// Indicates which src is currently selected (one-hot)
    pub clk_gpout0_selected: ClkGpout0Selected,
    /// 0x000c
    /// Clock control, can be changed on-the-fly (except for auxsrc)
    pub clk_gpout1_ctrl: ClkGpout1Ctrl,
    /// 0x0010
    pub clk_gpout1_div: ClkGpout1Div,
    /// 0x0014
    /// Indicates which src is currently selected (one-hot)
    pub clk_gpout1_selected: ClkGpout1Selected,
    /// 0x0018
    /// Clock control, can be changed on-the-fly (except for auxsrc)
    pub clk_gpout2_ctrl: ClkGpout2Ctrl,
    /// 0x001c
    pub clk_gpout2_div: ClkGpout2Div,
    /// 0x0020
    /// Indicates which src is currently selected (one-hot)
    pub clk_gpout2_selected: ClkGpout2Selected,
    /// 0x0024
    /// Clock control, can be changed on-the-fly (except for auxsrc)
    pub clk_gpout3_ctrl: ClkGpout3Ctrl,
    /// 0x0028
    pub clk_gpout3_div: ClkGpout3Div,
    /// 0x002c
    /// Indicates which src is currently selected (one-hot)
    pub clk_gpout3_selected: ClkGpout3Selected,
    /// 0x0030
    /// Clock control, can be changed on-the-fly (except for auxsrc)
    pub clk_ref_ctrl: ClkRefCtrl,
    /// 0x0034
    pub clk_ref_div: ClkRefDiv,
    /// 0x0038
    /// Indicates which src is currently selected (one-hot)
    pub clk_ref_selected: ClkRefSelected,
    /// 0x003c
    /// Clock control, can be changed on-the-fly (except for auxsrc)
    pub clk_sys_ctrl: ClkSysCtrl,
    /// 0x0040
    pub clk_sys_div: ClkSysDiv,
    /// 0x0044
    /// Indicates which src is currently selected (one-hot)
    pub clk_sys_selected: ClkSysSelected,
    /// 0x0048
    /// Clock control, can be changed on-the-fly (except for auxsrc)
    pub clk_peri_ctrl: ClkPeriCtrl,
    /// 0x004c
    pub clk_peri_div: ClkPeriDiv,
    /// 0x0050
    /// Indicates which src is currently selected (one-hot)
    pub clk_peri_selected: ClkPeriSelected,
    /// 0x0054
    /// Clock control, can be changed on-the-fly (except for auxsrc)
    pub clk_hstx_ctrl: ClkHstxCtrl,
    /// 0x0058
    pub clk_hstx_div: ClkHstxDiv,
    /// 0x005c
    /// Indicates which src is currently selected (one-hot)
    pub clk_hstx_selected: ClkHstxSelected,
    /// 0x0060
    /// Clock control, can be changed on-the-fly (except for auxsrc)
    pub clk_usb_ctrl: ClkUsbCtrl,
    /// 0x0064
    pub clk_usb_div: ClkUsbDiv,
    /// 0x0068
    /// Indicates which src is currently selected (one-hot)
    pub clk_usb_selected: ClkUsbSelected,
    /// 0x006c
    /// Clock control, can be changed on-the-fly (except for auxsrc)
    pub clk_adc_ctrl: ClkAdcCtrl,
    /// 0x0070
    pub clk_adc_div: ClkAdcDiv,
    /// 0x0074
    /// Indicates which src is currently selected (one-hot)
    pub clk_adc_selected: ClkAdcSelected,
    /// 0x0078
    pub dftclk_xosc_ctrl: DftclkXoscCtrl,
    /// 0x007c
    pub dftclk_rosc_ctrl: DftclkRoscCtrl,
    /// 0x0080
    pub dftclk_lposc_ctrl: DftclkLposcCtrl,
    /// 0x0084
    pub clk_sys_resus_ctrl: ClkSysResusCtrl,
    /// 0x0088
    pub clk_sys_resus_status: ClkSysResusStatus,
    /// 0x008c
    /// Reference clock frequency in kHz
    pub fc0_ref_khz: Fc0RefKhz,
    /// 0x0090
    /// Minimum pass frequency in kHz. This is optional. Set to 0 if you are not using the pass/fail flags
    pub fc0_min_khz: Fc0MinKhz,
    /// 0x0094
    /// Maximum pass frequency in kHz. This is optional. Set to 0x1ffffff if you are not using the pass/fail flags
    pub fc0_max_khz: Fc0MaxKhz,
    /// 0x0098
    /// Delays the start of frequency counting to allow the mux to settle
    /// Delay is measured in multiples of the reference clock period
    pub fc0_delay: Fc0Delay,
    /// 0x009c
    /// The test interval is 0.98us * 2**interval, but let's call it 1us * 2**interval
    /// The default gives a test interval of 250us
    pub fc0_interval: Fc0Interval,
    /// 0x00a0
    /// Clock sent to frequency counter, set to 0 when not required
    /// Writing to this register initiates the frequency count
    pub fc0_src: Fc0Src,
    /// 0x00a4
    /// Frequency counter status
    pub fc0_status: Fc0Status,
    /// 0x00a8
    /// Result of frequency measurement, only valid when status_done=1
    pub fc0_result: Fc0Result,
    /// 0x00ac
    /// enable clock in wake mode
    pub wake_en0: WakeEn0,
    /// 0x00b0
    /// enable clock in wake mode
    pub wake_en1: WakeEn1,
    /// 0x00b4
    /// enable clock in sleep mode
    pub sleep_en0: SleepEn0,
    /// 0x00b8
    /// enable clock in sleep mode
    pub sleep_en1: SleepEn1,
    /// 0x00bc
    /// indicates the state of the clock enable
    pub enabled0: Enabled0,
    /// 0x00c0
    /// indicates the state of the clock enable
    pub enabled1: Enabled1,
    /// 0x00c4
    /// Raw Interrupts
    pub intr: Intr,
    /// 0x00c8
    /// Interrupt Enable
    pub inte: Inte,
    /// 0x00cc
    /// Interrupt Force
    pub intf: Intf,
    /// 0x00d0
    /// Interrupt status after masking & forcing
    pub ints: Ints,
}

