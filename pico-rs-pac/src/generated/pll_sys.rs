//! PLL_SYS
//!
//! This peripheral supports atomic SET/CLR/XOR register aliases.

#![allow(dead_code)]

pub const BASE: u32 = 0x40050000;

/// CS Register
///
/// Control and Status
/// GENERAL CONSTRAINTS:
/// Reference clock frequency min=5MHz, max=800MHz
/// Feedback divider min=16, max=320
/// VCO frequency min=750MHz, max=1600MHz
///
/// Address offset: 0x0000
/// Reset value: 0x00000001
///
/// ## Fields
///
/// - **LOCK** `[31]` (RO)
///
///   PLL is locked
///
/// - **LOCK_N** `[30]` (RW)
///
///   PLL is not locked
///   Ideally this is cleared when PLL lock is seen and this should never normally be set
///
/// - **BYPASS** `[8]` (RW)
///
///   Passes the reference clock to the output instead of the divided VCO. The VCO continues to run so the user can switch between the reference clock and the divided VCO but the output will glitch when doing so.
///
/// - **REFDIV** `[5:0]` (RW)
///
///   Divides the PLL input reference clock.
///   Behaviour is undefined for div=0.
///   PLL output will be unpredictable during refdiv changes, wait for lock=1 before using it.
#[repr(transparent)]
pub struct Cs {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Cs {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Cs {
    // ======== Field Constants ========

    /// **LOCK**
    ///
    /// PLL is locked
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const LOCK_OFFSET: u32 = 31;
    pub const LOCK_MASK: u32 = 0x80000000;
    pub const LOCK: u32 = 1 << 31;

    /// **LOCK_N**
    ///
    /// PLL is not locked
    /// Ideally this is cleared when PLL lock is seen and this should never normally be set
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const LOCK_N_OFFSET: u32 = 30;
    pub const LOCK_N_MASK: u32 = 0x40000000;
    pub const LOCK_N: u32 = 1 << 30;

    /// **BYPASS**
    ///
    /// Passes the reference clock to the output instead of the divided VCO. The VCO continues to run so the user can switch between the reference clock and the divided VCO but the output will glitch when doing so.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const BYPASS_OFFSET: u32 = 8;
    pub const BYPASS_MASK: u32 = 0x00000100;
    pub const BYPASS: u32 = 1 << 8;

    /// **REFDIV**
    ///
    /// Divides the PLL input reference clock.
    /// Behaviour is undefined for div=0.
    /// PLL output will be unpredictable during refdiv changes, wait for lock=1 before using it.
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const REFDIV_OFFSET: u32 = 0;
    pub const REFDIV_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **LOCK** `[31]` (RO)
    ///
    /// PLL is locked
    #[inline(always)]
    pub fn lock(&self) -> bool {
        (self.value.read_bits() & Self::LOCK_MASK) != 0
    }

    /// **LOCK_N** `[30]` (RW)
    ///
    /// PLL is not locked
    /// Ideally this is cleared when PLL lock is seen and this should never normally be set
    #[inline(always)]
    pub fn lock_n(&self) -> bool {
        (self.value.read_bits() & Self::LOCK_N_MASK) != 0
    }

    /// Set LOCK_N bit atomically (RW)
    #[inline(always)]
    pub fn set_lock_n(&self) {
        self.set_bits(Self::LOCK_N_MASK);
    }

    /// Clear LOCK_N bit atomically (RW)
    #[inline(always)]
    pub fn clr_lock_n(&self) {
        self.clr_bits(Self::LOCK_N_MASK);
    }

    /// **BYPASS** `[8]` (RW)
    ///
    /// Passes the reference clock to the output instead of the divided VCO. The VCO continues to run so the user can switch between the reference clock and the divided VCO but the output will glitch when doing so.
    #[inline(always)]
    pub fn bypass(&self) -> bool {
        (self.value.read_bits() & Self::BYPASS_MASK) != 0
    }

    /// Set BYPASS bit atomically (RW)
    #[inline(always)]
    pub fn set_bypass(&self) {
        self.set_bits(Self::BYPASS_MASK);
    }

    /// Clear BYPASS bit atomically (RW)
    #[inline(always)]
    pub fn clr_bypass(&self) {
        self.clr_bits(Self::BYPASS_MASK);
    }

    /// **REFDIV** `[5:0]` (RW)
    ///
    /// Divides the PLL input reference clock.
    /// Behaviour is undefined for div=0.
    /// PLL output will be unpredictable during refdiv changes, wait for lock=1 before using it.
    #[inline(always)]
    pub fn refdiv(&self) -> u32 {
        (self.value.read_bits() & Self::REFDIV_MASK) >> Self::REFDIV_OFFSET
    }

    /// Write **REFDIV** `[5:0]` (RW)
    ///
    /// Divides the PLL input reference clock.
    /// Behaviour is undefined for div=0.
    /// PLL output will be unpredictable during refdiv changes, wait for lock=1 before using it.
    #[inline(always)]
    pub fn write_refdiv(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REFDIV_MASK) | ((value << Self::REFDIV_OFFSET) & Self::REFDIV_MASK)
        });
    }
}

/// CS value type for batch read/modify/write operations
///
/// Use `cs.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `cs.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CsVal {
    bits: u32,
}

impl CsVal {
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

    /// Read **LOCK** `[31:31]`
    ///
    /// PLL is locked
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        (self.bits & Cs::LOCK_MASK) != 0
    }

    /// Read **LOCK_N** `[30:30]`
    ///
    /// PLL is not locked
    /// Ideally this is cleared when PLL lock is seen and this should never normally be set
    #[inline(always)]
    pub const fn lock_n(&self) -> bool {
        (self.bits & Cs::LOCK_N_MASK) != 0
    }

    /// Read **BYPASS** `[8:8]`
    ///
    /// Passes the reference clock to the output instead of the divided VCO. The VCO continues to run so the user can switch between the reference clock and the divided VCO but the output will glitch when doing so.
    #[inline(always)]
    pub const fn bypass(&self) -> bool {
        (self.bits & Cs::BYPASS_MASK) != 0
    }

    /// Read **REFDIV** `[5:0]`
    ///
    /// Divides the PLL input reference clock.
    /// Behaviour is undefined for div=0.
    /// PLL output will be unpredictable during refdiv changes, wait for lock=1 before using it.
    #[inline(always)]
    pub const fn refdiv(&self) -> u32 {
        (self.bits & Cs::REFDIV_MASK) >> Cs::REFDIV_OFFSET
    }

    /// Set **LOCK_N** `[30:30]`
    ///
    /// PLL is not locked
    /// Ideally this is cleared when PLL lock is seen and this should never normally be set
    #[inline(always)]
    pub const fn with_lock_n(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cs::LOCK_N_MASK
            } else {
                self.bits & !Cs::LOCK_N_MASK
            },
        }
    }

    /// Set **BYPASS** `[8:8]`
    ///
    /// Passes the reference clock to the output instead of the divided VCO. The VCO continues to run so the user can switch between the reference clock and the divided VCO but the output will glitch when doing so.
    #[inline(always)]
    pub const fn with_bypass(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cs::BYPASS_MASK
            } else {
                self.bits & !Cs::BYPASS_MASK
            },
        }
    }

    /// Set **REFDIV** `[5:0]`
    ///
    /// Divides the PLL input reference clock.
    /// Behaviour is undefined for div=0.
    /// PLL output will be unpredictable during refdiv changes, wait for lock=1 before using it.
    #[inline(always)]
    pub const fn with_refdiv(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cs::REFDIV_MASK) | ((value << Cs::REFDIV_OFFSET) & Cs::REFDIV_MASK),
        }
    }
}

impl Default for CsVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Cs {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CsVal {
        CsVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CsVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CsVal) -> CsVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PWR Register
///
/// Controls the PLL power modes.
///
/// Address offset: 0x0004
/// Reset value: 0x0000002d
///
/// ## Fields
///
/// - **VCOPD** `[5]` (RW)
///
///   PLL VCO powerdown
///   To save power set high when PLL output not required or bypass=1.
///
/// - **POSTDIVPD** `[3]` (RW)
///
///   PLL post divider powerdown
///   To save power set high when PLL output not required or bypass=1.
///
/// - **DSMPD** `[2]` (RW)
///
///   PLL DSM powerdown
///   Nothing is achieved by setting this low.
///
/// - **PD** `[0]` (RW)
///
///   PLL powerdown
///   To save power set high when PLL output not required.
#[repr(transparent)]
pub struct Pwr {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Pwr {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Pwr {
    // ======== Field Constants ========

    /// **VCOPD**
    ///
    /// PLL VCO powerdown
    /// To save power set high when PLL output not required or bypass=1.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const VCOPD_OFFSET: u32 = 5;
    pub const VCOPD_MASK: u32 = 0x00000020;
    pub const VCOPD: u32 = 1 << 5;

    /// **POSTDIVPD**
    ///
    /// PLL post divider powerdown
    /// To save power set high when PLL output not required or bypass=1.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const POSTDIVPD_OFFSET: u32 = 3;
    pub const POSTDIVPD_MASK: u32 = 0x00000008;
    pub const POSTDIVPD: u32 = 1 << 3;

    /// **DSMPD**
    ///
    /// PLL DSM powerdown
    /// Nothing is achieved by setting this low.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const DSMPD_OFFSET: u32 = 2;
    pub const DSMPD_MASK: u32 = 0x00000004;
    pub const DSMPD: u32 = 1 << 2;

    /// **PD**
    ///
    /// PLL powerdown
    /// To save power set high when PLL output not required.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const PD_OFFSET: u32 = 0;
    pub const PD_MASK: u32 = 0x00000001;
    pub const PD: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **VCOPD** `[5]` (RW)
    ///
    /// PLL VCO powerdown
    /// To save power set high when PLL output not required or bypass=1.
    #[inline(always)]
    pub fn vcopd(&self) -> bool {
        (self.value.read_bits() & Self::VCOPD_MASK) != 0
    }

    /// Set VCOPD bit atomically (RW)
    #[inline(always)]
    pub fn set_vcopd(&self) {
        self.set_bits(Self::VCOPD_MASK);
    }

    /// Clear VCOPD bit atomically (RW)
    #[inline(always)]
    pub fn clr_vcopd(&self) {
        self.clr_bits(Self::VCOPD_MASK);
    }

    /// **POSTDIVPD** `[3]` (RW)
    ///
    /// PLL post divider powerdown
    /// To save power set high when PLL output not required or bypass=1.
    #[inline(always)]
    pub fn postdivpd(&self) -> bool {
        (self.value.read_bits() & Self::POSTDIVPD_MASK) != 0
    }

    /// Set POSTDIVPD bit atomically (RW)
    #[inline(always)]
    pub fn set_postdivpd(&self) {
        self.set_bits(Self::POSTDIVPD_MASK);
    }

    /// Clear POSTDIVPD bit atomically (RW)
    #[inline(always)]
    pub fn clr_postdivpd(&self) {
        self.clr_bits(Self::POSTDIVPD_MASK);
    }

    /// **DSMPD** `[2]` (RW)
    ///
    /// PLL DSM powerdown
    /// Nothing is achieved by setting this low.
    #[inline(always)]
    pub fn dsmpd(&self) -> bool {
        (self.value.read_bits() & Self::DSMPD_MASK) != 0
    }

    /// Set DSMPD bit atomically (RW)
    #[inline(always)]
    pub fn set_dsmpd(&self) {
        self.set_bits(Self::DSMPD_MASK);
    }

    /// Clear DSMPD bit atomically (RW)
    #[inline(always)]
    pub fn clr_dsmpd(&self) {
        self.clr_bits(Self::DSMPD_MASK);
    }

    /// **PD** `[0]` (RW)
    ///
    /// PLL powerdown
    /// To save power set high when PLL output not required.
    #[inline(always)]
    pub fn pd(&self) -> bool {
        (self.value.read_bits() & Self::PD_MASK) != 0
    }

    /// Set PD bit atomically (RW)
    #[inline(always)]
    pub fn set_pd(&self) {
        self.set_bits(Self::PD_MASK);
    }

    /// Clear PD bit atomically (RW)
    #[inline(always)]
    pub fn clr_pd(&self) {
        self.clr_bits(Self::PD_MASK);
    }
}

/// PWR value type for batch read/modify/write operations
///
/// Use `pwr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pwr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PwrVal {
    bits: u32,
}

impl PwrVal {
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
        Self { bits: 0x0000002d }
    }

    /// Read **VCOPD** `[5:5]`
    ///
    /// PLL VCO powerdown
    /// To save power set high when PLL output not required or bypass=1.
    #[inline(always)]
    pub const fn vcopd(&self) -> bool {
        (self.bits & Pwr::VCOPD_MASK) != 0
    }

    /// Read **POSTDIVPD** `[3:3]`
    ///
    /// PLL post divider powerdown
    /// To save power set high when PLL output not required or bypass=1.
    #[inline(always)]
    pub const fn postdivpd(&self) -> bool {
        (self.bits & Pwr::POSTDIVPD_MASK) != 0
    }

    /// Read **DSMPD** `[2:2]`
    ///
    /// PLL DSM powerdown
    /// Nothing is achieved by setting this low.
    #[inline(always)]
    pub const fn dsmpd(&self) -> bool {
        (self.bits & Pwr::DSMPD_MASK) != 0
    }

    /// Read **PD** `[0:0]`
    ///
    /// PLL powerdown
    /// To save power set high when PLL output not required.
    #[inline(always)]
    pub const fn pd(&self) -> bool {
        (self.bits & Pwr::PD_MASK) != 0
    }

    /// Set **VCOPD** `[5:5]`
    ///
    /// PLL VCO powerdown
    /// To save power set high when PLL output not required or bypass=1.
    #[inline(always)]
    pub const fn with_vcopd(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pwr::VCOPD_MASK
            } else {
                self.bits & !Pwr::VCOPD_MASK
            },
        }
    }

    /// Set **POSTDIVPD** `[3:3]`
    ///
    /// PLL post divider powerdown
    /// To save power set high when PLL output not required or bypass=1.
    #[inline(always)]
    pub const fn with_postdivpd(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pwr::POSTDIVPD_MASK
            } else {
                self.bits & !Pwr::POSTDIVPD_MASK
            },
        }
    }

    /// Set **DSMPD** `[2:2]`
    ///
    /// PLL DSM powerdown
    /// Nothing is achieved by setting this low.
    #[inline(always)]
    pub const fn with_dsmpd(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pwr::DSMPD_MASK
            } else {
                self.bits & !Pwr::DSMPD_MASK
            },
        }
    }

    /// Set **PD** `[0:0]`
    ///
    /// PLL powerdown
    /// To save power set high when PLL output not required.
    #[inline(always)]
    pub const fn with_pd(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pwr::PD_MASK
            } else {
                self.bits & !Pwr::PD_MASK
            },
        }
    }
}

impl Default for PwrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Pwr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PwrVal {
        PwrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: PwrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(PwrVal) -> PwrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// FBDIV_INT Register
///
/// Feedback divisor
/// (note: this PLL does not support fractional division)
///
/// Address offset: 0x0008
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **FBDIV_INT** `[11:0]` (RW)
///
///   see ctrl reg description for constraints
#[repr(transparent)]
pub struct FbdivInt {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for FbdivInt {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl FbdivInt {
    // ======== Field Constants ========

    /// **FBDIV_INT**
    ///
    /// see ctrl reg description for constraints
    ///
    /// Bits `[11:0]`, mask `0x00000fff`
    pub const FBDIV_INT_OFFSET: u32 = 0;
    pub const FBDIV_INT_MASK: u32 = 0x00000fff;

    // ======== Field Accessors ========

    /// **FBDIV_INT** `[11:0]` (RW)
    ///
    /// see ctrl reg description for constraints
    #[inline(always)]
    pub fn fbdiv_int(&self) -> u32 {
        (self.value.read_bits() & Self::FBDIV_INT_MASK) >> Self::FBDIV_INT_OFFSET
    }

    /// Write **FBDIV_INT** `[11:0]` (RW)
    ///
    /// see ctrl reg description for constraints
    #[inline(always)]
    pub fn write_fbdiv_int(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::FBDIV_INT_MASK) | ((value << Self::FBDIV_INT_OFFSET) & Self::FBDIV_INT_MASK)
        });
    }
}

/// FBDIV_INT value type for batch read/modify/write operations
///
/// Use `fbdivint.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `fbdivint.write_val(val)`.
#[derive(Clone, Copy)]
pub struct FbdivIntVal {
    bits: u32,
}

impl FbdivIntVal {
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

    /// Read **FBDIV_INT** `[11:0]`
    ///
    /// see ctrl reg description for constraints
    #[inline(always)]
    pub const fn fbdiv_int(&self) -> u32 {
        (self.bits & FbdivInt::FBDIV_INT_MASK) >> FbdivInt::FBDIV_INT_OFFSET
    }

    /// Set **FBDIV_INT** `[11:0]`
    ///
    /// see ctrl reg description for constraints
    #[inline(always)]
    pub const fn with_fbdiv_int(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !FbdivInt::FBDIV_INT_MASK)
                | ((value << FbdivInt::FBDIV_INT_OFFSET) & FbdivInt::FBDIV_INT_MASK),
        }
    }
}

impl Default for FbdivIntVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl FbdivInt {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> FbdivIntVal {
        FbdivIntVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: FbdivIntVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(FbdivIntVal) -> FbdivIntVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PRIM Register
///
/// Controls the PLL post dividers for the primary output
/// (note: this PLL does not have a secondary output)
/// the primary output is driven from VCO divided by postdiv1*postdiv2
///
/// Address offset: 0x000c
/// Reset value: 0x00077000
///
/// ## Fields
///
/// - **POSTDIV1** `[18:16]` (RW)
///
///   divide by 1-7
///
/// - **POSTDIV2** `[14:12]` (RW)
///
///   divide by 1-7
#[repr(transparent)]
pub struct Prim {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Prim {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Prim {
    // ======== Field Constants ========

    /// **POSTDIV1**
    ///
    /// divide by 1-7
    ///
    /// Bits `[18:16]`, mask `0x00070000`
    pub const POSTDIV1_OFFSET: u32 = 16;
    pub const POSTDIV1_MASK: u32 = 0x00070000;

    /// **POSTDIV2**
    ///
    /// divide by 1-7
    ///
    /// Bits `[14:12]`, mask `0x00007000`
    pub const POSTDIV2_OFFSET: u32 = 12;
    pub const POSTDIV2_MASK: u32 = 0x00007000;

    // ======== Field Accessors ========

    /// **POSTDIV1** `[18:16]` (RW)
    ///
    /// divide by 1-7
    #[inline(always)]
    pub fn postdiv1(&self) -> u32 {
        (self.value.read_bits() & Self::POSTDIV1_MASK) >> Self::POSTDIV1_OFFSET
    }

    /// Write **POSTDIV1** `[18:16]` (RW)
    ///
    /// divide by 1-7
    #[inline(always)]
    pub fn write_postdiv1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::POSTDIV1_MASK) | ((value << Self::POSTDIV1_OFFSET) & Self::POSTDIV1_MASK)
        });
    }

    /// **POSTDIV2** `[14:12]` (RW)
    ///
    /// divide by 1-7
    #[inline(always)]
    pub fn postdiv2(&self) -> u32 {
        (self.value.read_bits() & Self::POSTDIV2_MASK) >> Self::POSTDIV2_OFFSET
    }

    /// Write **POSTDIV2** `[14:12]` (RW)
    ///
    /// divide by 1-7
    #[inline(always)]
    pub fn write_postdiv2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::POSTDIV2_MASK) | ((value << Self::POSTDIV2_OFFSET) & Self::POSTDIV2_MASK)
        });
    }
}

/// PRIM value type for batch read/modify/write operations
///
/// Use `prim.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `prim.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PrimVal {
    bits: u32,
}

impl PrimVal {
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
        Self { bits: 0x00077000 }
    }

    /// Read **POSTDIV1** `[18:16]`
    ///
    /// divide by 1-7
    #[inline(always)]
    pub const fn postdiv1(&self) -> u32 {
        (self.bits & Prim::POSTDIV1_MASK) >> Prim::POSTDIV1_OFFSET
    }

    /// Read **POSTDIV2** `[14:12]`
    ///
    /// divide by 1-7
    #[inline(always)]
    pub const fn postdiv2(&self) -> u32 {
        (self.bits & Prim::POSTDIV2_MASK) >> Prim::POSTDIV2_OFFSET
    }

    /// Set **POSTDIV1** `[18:16]`
    ///
    /// divide by 1-7
    #[inline(always)]
    pub const fn with_postdiv1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Prim::POSTDIV1_MASK)
                | ((value << Prim::POSTDIV1_OFFSET) & Prim::POSTDIV1_MASK),
        }
    }

    /// Set **POSTDIV2** `[14:12]`
    ///
    /// divide by 1-7
    #[inline(always)]
    pub const fn with_postdiv2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Prim::POSTDIV2_MASK)
                | ((value << Prim::POSTDIV2_OFFSET) & Prim::POSTDIV2_MASK),
        }
    }
}

impl Default for PrimVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Prim {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PrimVal {
        PrimVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: PrimVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(PrimVal) -> PrimVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// INTR Register
///
/// Raw Interrupts
///
/// Address offset: 0x0010
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **LOCK_N_STICKY** `[0]` (RW)
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

    /// **LOCK_N_STICKY**
    /// Bits `[0:0]`, mask `0x00000001`
    pub const LOCK_N_STICKY_OFFSET: u32 = 0;
    pub const LOCK_N_STICKY_MASK: u32 = 0x00000001;
    pub const LOCK_N_STICKY: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **LOCK_N_STICKY** `[0]` (RW)
    #[inline(always)]
    pub fn lock_n_sticky(&self) -> bool {
        (self.value.read_bits() & Self::LOCK_N_STICKY_MASK) != 0
    }

    /// Set LOCK_N_STICKY bit atomically (RW)
    #[inline(always)]
    pub fn set_lock_n_sticky(&self) {
        self.set_bits(Self::LOCK_N_STICKY_MASK);
    }

    /// Clear LOCK_N_STICKY bit atomically (RW)
    #[inline(always)]
    pub fn clr_lock_n_sticky(&self) {
        self.clr_bits(Self::LOCK_N_STICKY_MASK);
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

    /// Read **LOCK_N_STICKY** `[0:0]`
    #[inline(always)]
    pub const fn lock_n_sticky(&self) -> bool {
        (self.bits & Intr::LOCK_N_STICKY_MASK) != 0
    }

    /// Set **LOCK_N_STICKY** `[0:0]`
    #[inline(always)]
    pub const fn with_lock_n_sticky(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intr::LOCK_N_STICKY_MASK
            } else {
                self.bits & !Intr::LOCK_N_STICKY_MASK
            },
        }
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

/// INTE Register
///
/// Interrupt Enable
///
/// Address offset: 0x0014
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **LOCK_N_STICKY** `[0]` (RW)
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

    /// **LOCK_N_STICKY**
    /// Bits `[0:0]`, mask `0x00000001`
    pub const LOCK_N_STICKY_OFFSET: u32 = 0;
    pub const LOCK_N_STICKY_MASK: u32 = 0x00000001;
    pub const LOCK_N_STICKY: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **LOCK_N_STICKY** `[0]` (RW)
    #[inline(always)]
    pub fn lock_n_sticky(&self) -> bool {
        (self.value.read_bits() & Self::LOCK_N_STICKY_MASK) != 0
    }

    /// Set LOCK_N_STICKY bit atomically (RW)
    #[inline(always)]
    pub fn set_lock_n_sticky(&self) {
        self.set_bits(Self::LOCK_N_STICKY_MASK);
    }

    /// Clear LOCK_N_STICKY bit atomically (RW)
    #[inline(always)]
    pub fn clr_lock_n_sticky(&self) {
        self.clr_bits(Self::LOCK_N_STICKY_MASK);
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

    /// Read **LOCK_N_STICKY** `[0:0]`
    #[inline(always)]
    pub const fn lock_n_sticky(&self) -> bool {
        (self.bits & Inte::LOCK_N_STICKY_MASK) != 0
    }

    /// Set **LOCK_N_STICKY** `[0:0]`
    #[inline(always)]
    pub const fn with_lock_n_sticky(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Inte::LOCK_N_STICKY_MASK
            } else {
                self.bits & !Inte::LOCK_N_STICKY_MASK
            },
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
/// Address offset: 0x0018
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **LOCK_N_STICKY** `[0]` (RW)
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

    /// **LOCK_N_STICKY**
    /// Bits `[0:0]`, mask `0x00000001`
    pub const LOCK_N_STICKY_OFFSET: u32 = 0;
    pub const LOCK_N_STICKY_MASK: u32 = 0x00000001;
    pub const LOCK_N_STICKY: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **LOCK_N_STICKY** `[0]` (RW)
    #[inline(always)]
    pub fn lock_n_sticky(&self) -> bool {
        (self.value.read_bits() & Self::LOCK_N_STICKY_MASK) != 0
    }

    /// Set LOCK_N_STICKY bit atomically (RW)
    #[inline(always)]
    pub fn set_lock_n_sticky(&self) {
        self.set_bits(Self::LOCK_N_STICKY_MASK);
    }

    /// Clear LOCK_N_STICKY bit atomically (RW)
    #[inline(always)]
    pub fn clr_lock_n_sticky(&self) {
        self.clr_bits(Self::LOCK_N_STICKY_MASK);
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

    /// Read **LOCK_N_STICKY** `[0:0]`
    #[inline(always)]
    pub const fn lock_n_sticky(&self) -> bool {
        (self.bits & Intf::LOCK_N_STICKY_MASK) != 0
    }

    /// Set **LOCK_N_STICKY** `[0:0]`
    #[inline(always)]
    pub const fn with_lock_n_sticky(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intf::LOCK_N_STICKY_MASK
            } else {
                self.bits & !Intf::LOCK_N_STICKY_MASK
            },
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
/// Address offset: 0x001c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **LOCK_N_STICKY** `[0]` (RO)
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

    /// **LOCK_N_STICKY**
    /// Bits `[0:0]`, mask `0x00000001`
    pub const LOCK_N_STICKY_OFFSET: u32 = 0;
    pub const LOCK_N_STICKY_MASK: u32 = 0x00000001;
    pub const LOCK_N_STICKY: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **LOCK_N_STICKY** `[0]` (RO)
    #[inline(always)]
    pub fn lock_n_sticky(&self) -> bool {
        (self.value.read_bits() & Self::LOCK_N_STICKY_MASK) != 0
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

    /// Read **LOCK_N_STICKY** `[0:0]`
    #[inline(always)]
    pub const fn lock_n_sticky(&self) -> bool {
        (self.bits & Ints::LOCK_N_STICKY_MASK) != 0
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
    /// Control and Status
    /// GENERAL CONSTRAINTS:
    /// Reference clock frequency min=5MHz, max=800MHz
    /// Feedback divider min=16, max=320
    /// VCO frequency min=750MHz, max=1600MHz
    pub cs: Cs,
    /// 0x0004
    /// Controls the PLL power modes.
    pub pwr: Pwr,
    /// 0x0008
    /// Feedback divisor
    /// (note: this PLL does not support fractional division)
    pub fbdiv_int: FbdivInt,
    /// 0x000c
    /// Controls the PLL post dividers for the primary output
    /// (note: this PLL does not have a secondary output)
    /// the primary output is driven from VCO divided by postdiv1*postdiv2
    pub prim: Prim,
    /// 0x0010
    /// Raw Interrupts
    pub intr: Intr,
    /// 0x0014
    /// Interrupt Enable
    pub inte: Inte,
    /// 0x0018
    /// Interrupt Force
    pub intf: Intf,
    /// 0x001c
    /// Interrupt status after masking & forcing
    pub ints: Ints,
}
