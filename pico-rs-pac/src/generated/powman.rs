//! POWMAN
//! Controls vreg, bor, lposc, chip resets & xosc startup, powman and provides scratch register for general use and for bootcode use
//!
//! This peripheral supports atomic SET/CLR/XOR register aliases.

#![allow(dead_code)]

pub const BASE: u32 = 0x40100000;

/// BADPASSWD Register
///
/// Indicates a bad password has been used
///
/// Address offset: 0x0000
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **BADPASSWD** `[0]` (RW)
#[repr(transparent)]
pub struct Badpasswd {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Badpasswd {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Badpasswd {
    // ======== Field Constants ========

    /// **BADPASSWD**
    /// Bits `[0:0]`, mask `0x00000001`
    pub const BADPASSWD_OFFSET: u32 = 0;
    pub const BADPASSWD_MASK: u32 = 0x00000001;
    pub const BADPASSWD: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **BADPASSWD** `[0]` (RW)
    #[inline(always)]
    pub fn badpasswd(&self) -> bool {
        (self.value.read_bits() & Self::BADPASSWD_MASK) != 0
    }

    /// Set BADPASSWD bit atomically (RW)
    #[inline(always)]
    pub fn set_badpasswd(&self) {
        self.set_bits(Self::BADPASSWD_MASK);
    }

    /// Clear BADPASSWD bit atomically (RW)
    #[inline(always)]
    pub fn clr_badpasswd(&self) {
        self.clr_bits(Self::BADPASSWD_MASK);
    }

}

/// BADPASSWD value type for batch read/modify/write operations
///
/// Use `badpasswd.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `badpasswd.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BadpasswdVal {
    bits: u32,
}

impl BadpasswdVal {
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

    /// Read **BADPASSWD** `[0:0]`
    #[inline(always)]
    pub const fn badpasswd(&self) -> bool {
        (self.bits & Badpasswd::BADPASSWD_MASK) != 0
    }

    /// Set **BADPASSWD** `[0:0]`
    #[inline(always)]
    pub const fn with_badpasswd(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Badpasswd::BADPASSWD_MASK
            } else {
                self.bits & !Badpasswd::BADPASSWD_MASK
            }
        }
    }

}

impl Default for BadpasswdVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Badpasswd {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> BadpasswdVal {
        BadpasswdVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: BadpasswdVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(BadpasswdVal) -> BadpasswdVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// VREG_CTRL Register
///
/// Voltage Regulator Control
///
/// Address offset: 0x0004
/// Reset value: 0x00008050
///
/// ## Fields
///
/// - **RST_N** `[15]` (RW)
///
///   returns the regulator to its startup settings
///   0 - reset
///   1 - not reset (default)
///
/// - **UNLOCK** `[13]` (RW)
///
///   unlocks the VREG control interface after power up
///   0 - Locked (default)
///   1 - Unlocked
///   It cannot be relocked when it is unlocked.
///
/// - **ISOLATE** `[12]` (RW)
///
///   isolates the VREG control interface
///   0 - not isolated (default)
///   1 - isolated
///
/// - **DISABLE_VOLTAGE_LIMIT** `[8]` (RW)
///
///   0=not disabled, 1=enabled
///
/// - **HT_TH** `[6:4]` (RW)
///
///   high temperature protection threshold
///   regulator power transistors are disabled when junction temperature exceeds threshold
///   000 - 100C
///   001 - 105C
///   010 - 110C
///   011 - 115C
///   100 - 120C
///   101 - 125C
///   110 - 135C
///   111 - 150C
#[repr(transparent)]
pub struct VregCtrl {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for VregCtrl {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl VregCtrl {
    // ======== Field Constants ========

    /// **RST_N**
    ///
    /// returns the regulator to its startup settings
    /// 0 - reset
    /// 1 - not reset (default)
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const RST_N_OFFSET: u32 = 15;
    pub const RST_N_MASK: u32 = 0x00008000;
    pub const RST_N: u32 = 1 << 15;

    /// **UNLOCK**
    ///
    /// unlocks the VREG control interface after power up
    /// 0 - Locked (default)
    /// 1 - Unlocked
    /// It cannot be relocked when it is unlocked.
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const UNLOCK_OFFSET: u32 = 13;
    pub const UNLOCK_MASK: u32 = 0x00002000;
    pub const UNLOCK: u32 = 1 << 13;

    /// **ISOLATE**
    ///
    /// isolates the VREG control interface
    /// 0 - not isolated (default)
    /// 1 - isolated
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const ISOLATE_OFFSET: u32 = 12;
    pub const ISOLATE_MASK: u32 = 0x00001000;
    pub const ISOLATE: u32 = 1 << 12;

    /// **DISABLE_VOLTAGE_LIMIT**
    ///
    /// 0=not disabled, 1=enabled
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const DISABLE_VOLTAGE_LIMIT_OFFSET: u32 = 8;
    pub const DISABLE_VOLTAGE_LIMIT_MASK: u32 = 0x00000100;
    pub const DISABLE_VOLTAGE_LIMIT: u32 = 1 << 8;

    /// **HT_TH**
    ///
    /// high temperature protection threshold
    /// regulator power transistors are disabled when junction temperature exceeds threshold
    /// 000 - 100C
    /// 001 - 105C
    /// 010 - 110C
    /// 011 - 115C
    /// 100 - 120C
    /// 101 - 125C
    /// 110 - 135C
    /// 111 - 150C
    ///
    /// Bits `[6:4]`, mask `0x00000070`
    pub const HT_TH_OFFSET: u32 = 4;
    pub const HT_TH_MASK: u32 = 0x00000070;

    // ======== Field Accessors ========

    /// **RST_N** `[15]` (RW)
    ///
    /// returns the regulator to its startup settings
    /// 0 - reset
    /// 1 - not reset (default)
    #[inline(always)]
    pub fn rst_n(&self) -> bool {
        (self.value.read_bits() & Self::RST_N_MASK) != 0
    }

    /// Set RST_N bit atomically (RW)
    #[inline(always)]
    pub fn set_rst_n(&self) {
        self.set_bits(Self::RST_N_MASK);
    }

    /// Clear RST_N bit atomically (RW)
    #[inline(always)]
    pub fn clr_rst_n(&self) {
        self.clr_bits(Self::RST_N_MASK);
    }

    /// **UNLOCK** `[13]` (RW)
    ///
    /// unlocks the VREG control interface after power up
    /// 0 - Locked (default)
    /// 1 - Unlocked
    /// It cannot be relocked when it is unlocked.
    #[inline(always)]
    pub fn unlock(&self) -> bool {
        (self.value.read_bits() & Self::UNLOCK_MASK) != 0
    }

    /// Set UNLOCK bit atomically (RW)
    #[inline(always)]
    pub fn set_unlock(&self) {
        self.set_bits(Self::UNLOCK_MASK);
    }

    /// Clear UNLOCK bit atomically (RW)
    #[inline(always)]
    pub fn clr_unlock(&self) {
        self.clr_bits(Self::UNLOCK_MASK);
    }

    /// **ISOLATE** `[12]` (RW)
    ///
    /// isolates the VREG control interface
    /// 0 - not isolated (default)
    /// 1 - isolated
    #[inline(always)]
    pub fn isolate(&self) -> bool {
        (self.value.read_bits() & Self::ISOLATE_MASK) != 0
    }

    /// Set ISOLATE bit atomically (RW)
    #[inline(always)]
    pub fn set_isolate(&self) {
        self.set_bits(Self::ISOLATE_MASK);
    }

    /// Clear ISOLATE bit atomically (RW)
    #[inline(always)]
    pub fn clr_isolate(&self) {
        self.clr_bits(Self::ISOLATE_MASK);
    }

    /// **DISABLE_VOLTAGE_LIMIT** `[8]` (RW)
    ///
    /// 0=not disabled, 1=enabled
    #[inline(always)]
    pub fn disable_voltage_limit(&self) -> bool {
        (self.value.read_bits() & Self::DISABLE_VOLTAGE_LIMIT_MASK) != 0
    }

    /// Set DISABLE_VOLTAGE_LIMIT bit atomically (RW)
    #[inline(always)]
    pub fn set_disable_voltage_limit(&self) {
        self.set_bits(Self::DISABLE_VOLTAGE_LIMIT_MASK);
    }

    /// Clear DISABLE_VOLTAGE_LIMIT bit atomically (RW)
    #[inline(always)]
    pub fn clr_disable_voltage_limit(&self) {
        self.clr_bits(Self::DISABLE_VOLTAGE_LIMIT_MASK);
    }

    /// **HT_TH** `[6:4]` (RW)
    ///
    /// high temperature protection threshold
    /// regulator power transistors are disabled when junction temperature exceeds threshold
    /// 000 - 100C
    /// 001 - 105C
    /// 010 - 110C
    /// 011 - 115C
    /// 100 - 120C
    /// 101 - 125C
    /// 110 - 135C
    /// 111 - 150C
    #[inline(always)]
    pub fn ht_th(&self) -> u32 {
        (self.value.read_bits() & Self::HT_TH_MASK) >> Self::HT_TH_OFFSET
    }

    /// Write **HT_TH** `[6:4]` (RW)
    ///
    /// high temperature protection threshold
    /// regulator power transistors are disabled when junction temperature exceeds threshold
    /// 000 - 100C
    /// 001 - 105C
    /// 010 - 110C
    /// 011 - 115C
    /// 100 - 120C
    /// 101 - 125C
    /// 110 - 135C
    /// 111 - 150C
    #[inline(always)]
    pub fn write_ht_th(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::HT_TH_MASK) | ((value << Self::HT_TH_OFFSET) & Self::HT_TH_MASK));
    }

}

/// VREG_CTRL value type for batch read/modify/write operations
///
/// Use `vregctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `vregctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct VregCtrlVal {
    bits: u32,
}

impl VregCtrlVal {
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
        Self { bits: 0x00008050 }
    }

    /// Read **RST_N** `[15:15]`
    ///
    /// returns the regulator to its startup settings
    /// 0 - reset
    /// 1 - not reset (default)
    #[inline(always)]
    pub const fn rst_n(&self) -> bool {
        (self.bits & VregCtrl::RST_N_MASK) != 0
    }

    /// Read **UNLOCK** `[13:13]`
    ///
    /// unlocks the VREG control interface after power up
    /// 0 - Locked (default)
    /// 1 - Unlocked
    /// It cannot be relocked when it is unlocked.
    #[inline(always)]
    pub const fn unlock(&self) -> bool {
        (self.bits & VregCtrl::UNLOCK_MASK) != 0
    }

    /// Read **ISOLATE** `[12:12]`
    ///
    /// isolates the VREG control interface
    /// 0 - not isolated (default)
    /// 1 - isolated
    #[inline(always)]
    pub const fn isolate(&self) -> bool {
        (self.bits & VregCtrl::ISOLATE_MASK) != 0
    }

    /// Read **DISABLE_VOLTAGE_LIMIT** `[8:8]`
    ///
    /// 0=not disabled, 1=enabled
    #[inline(always)]
    pub const fn disable_voltage_limit(&self) -> bool {
        (self.bits & VregCtrl::DISABLE_VOLTAGE_LIMIT_MASK) != 0
    }

    /// Read **HT_TH** `[6:4]`
    ///
    /// high temperature protection threshold
    /// regulator power transistors are disabled when junction temperature exceeds threshold
    /// 000 - 100C
    /// 001 - 105C
    /// 010 - 110C
    /// 011 - 115C
    /// 100 - 120C
    /// 101 - 125C
    /// 110 - 135C
    /// 111 - 150C
    #[inline(always)]
    pub const fn ht_th(&self) -> u32 {
        (self.bits & VregCtrl::HT_TH_MASK) >> VregCtrl::HT_TH_OFFSET
    }

    /// Set **RST_N** `[15:15]`
    ///
    /// returns the regulator to its startup settings
    /// 0 - reset
    /// 1 - not reset (default)
    #[inline(always)]
    pub const fn with_rst_n(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | VregCtrl::RST_N_MASK
            } else {
                self.bits & !VregCtrl::RST_N_MASK
            }
        }
    }

    /// Set **UNLOCK** `[13:13]`
    ///
    /// unlocks the VREG control interface after power up
    /// 0 - Locked (default)
    /// 1 - Unlocked
    /// It cannot be relocked when it is unlocked.
    #[inline(always)]
    pub const fn with_unlock(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | VregCtrl::UNLOCK_MASK
            } else {
                self.bits & !VregCtrl::UNLOCK_MASK
            }
        }
    }

    /// Set **ISOLATE** `[12:12]`
    ///
    /// isolates the VREG control interface
    /// 0 - not isolated (default)
    /// 1 - isolated
    #[inline(always)]
    pub const fn with_isolate(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | VregCtrl::ISOLATE_MASK
            } else {
                self.bits & !VregCtrl::ISOLATE_MASK
            }
        }
    }

    /// Set **DISABLE_VOLTAGE_LIMIT** `[8:8]`
    ///
    /// 0=not disabled, 1=enabled
    #[inline(always)]
    pub const fn with_disable_voltage_limit(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | VregCtrl::DISABLE_VOLTAGE_LIMIT_MASK
            } else {
                self.bits & !VregCtrl::DISABLE_VOLTAGE_LIMIT_MASK
            }
        }
    }

    /// Set **HT_TH** `[6:4]`
    ///
    /// high temperature protection threshold
    /// regulator power transistors are disabled when junction temperature exceeds threshold
    /// 000 - 100C
    /// 001 - 105C
    /// 010 - 110C
    /// 011 - 115C
    /// 100 - 120C
    /// 101 - 125C
    /// 110 - 135C
    /// 111 - 150C
    #[inline(always)]
    pub const fn with_ht_th(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !VregCtrl::HT_TH_MASK) | ((value << VregCtrl::HT_TH_OFFSET) & VregCtrl::HT_TH_MASK)
        }
    }

}

impl Default for VregCtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl VregCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> VregCtrlVal {
        VregCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: VregCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(VregCtrlVal) -> VregCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// VREG_STS Register
///
/// Voltage Regulator Status
///
/// Address offset: 0x0008
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **VOUT_OK** `[4]` (RO)
///
///   output regulation status
///   0=not in regulation, 1=in regulation
///
/// - **STARTUP** `[0]` (RO)
///
///   startup status
///   0=startup complete, 1=starting up
#[repr(transparent)]
pub struct VregSts {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for VregSts {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl VregSts {
    // ======== Field Constants ========

    /// **VOUT_OK**
    ///
    /// output regulation status
    /// 0=not in regulation, 1=in regulation
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const VOUT_OK_OFFSET: u32 = 4;
    pub const VOUT_OK_MASK: u32 = 0x00000010;
    pub const VOUT_OK: u32 = 1 << 4;

    /// **STARTUP**
    ///
    /// startup status
    /// 0=startup complete, 1=starting up
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const STARTUP_OFFSET: u32 = 0;
    pub const STARTUP_MASK: u32 = 0x00000001;
    pub const STARTUP: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **VOUT_OK** `[4]` (RO)
    ///
    /// output regulation status
    /// 0=not in regulation, 1=in regulation
    #[inline(always)]
    pub fn vout_ok(&self) -> bool {
        (self.value.read_bits() & Self::VOUT_OK_MASK) != 0
    }

    /// **STARTUP** `[0]` (RO)
    ///
    /// startup status
    /// 0=startup complete, 1=starting up
    #[inline(always)]
    pub fn startup(&self) -> bool {
        (self.value.read_bits() & Self::STARTUP_MASK) != 0
    }

}

/// VREG_STS value type for batch read/modify/write operations
///
/// Use `vregsts.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `vregsts.write_val(val)`.
#[derive(Clone, Copy)]
pub struct VregStsVal {
    bits: u32,
}

impl VregStsVal {
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

    /// Read **VOUT_OK** `[4:4]`
    ///
    /// output regulation status
    /// 0=not in regulation, 1=in regulation
    #[inline(always)]
    pub const fn vout_ok(&self) -> bool {
        (self.bits & VregSts::VOUT_OK_MASK) != 0
    }

    /// Read **STARTUP** `[0:0]`
    ///
    /// startup status
    /// 0=startup complete, 1=starting up
    #[inline(always)]
    pub const fn startup(&self) -> bool {
        (self.bits & VregSts::STARTUP_MASK) != 0
    }

}

impl Default for VregStsVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl VregSts {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> VregStsVal {
        VregStsVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: VregStsVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(VregStsVal) -> VregStsVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// VREG Register
///
/// Voltage Regulator Settings
///
/// Address offset: 0x000c
/// Reset value: 0x000000b0
///
/// ## Fields
///
/// - **UPDATE_IN_PROGRESS** `[15]` (RO)
///
///   regulator state is being updated
///   writes to the vreg register will be ignored when this field is set
///
/// - **VSEL** `[8:4]` (RW)
///
///   output voltage select
///   the regulator output voltage is limited to 1.3V unless the voltage limit
///   is disabled using the disable_voltage_limit field in the vreg_ctrl register
///   00000 - 0.55V
///   00001 - 0.60V
///   00010 - 0.65V
///   00011 - 0.70V
///   00100 - 0.75V
///   00101 - 0.80V
///   00110 - 0.85V
///   00111 - 0.90V
///   01000 - 0.95V
///   01001 - 1.00V
///   01010 - 1.05V
///   01011 - 1.10V (default)
///   01100 - 1.15V
///   01101 - 1.20V
///   01110 - 1.25V
///   01111 - 1.30V
///   10000 - 1.35V
///   10001 - 1.40V
///   10010 - 1.50V
///   10011 - 1.60V
///   10100 - 1.65V
///   10101 - 1.70V
///   10110 - 1.80V
///   10111 - 1.90V
///   11000 - 2.00V
///   11001 - 2.35V
///   11010 - 2.50V
///   11011 - 2.65V
///   11100 - 2.80V
///   11101 - 3.00V
///   11110 - 3.15V
///   11111 - 3.30V
///
/// - **HIZ** `[1]` (RW)
///
///   high impedance mode select
///   0=not in high impedance mode, 1=in high impedance mode
#[repr(transparent)]
pub struct Vreg {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Vreg {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Vreg {
    // ======== Field Constants ========

    /// **UPDATE_IN_PROGRESS**
    ///
    /// regulator state is being updated
    /// writes to the vreg register will be ignored when this field is set
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const UPDATE_IN_PROGRESS_OFFSET: u32 = 15;
    pub const UPDATE_IN_PROGRESS_MASK: u32 = 0x00008000;
    pub const UPDATE_IN_PROGRESS: u32 = 1 << 15;

    /// **VSEL**
    ///
    /// output voltage select
    /// the regulator output voltage is limited to 1.3V unless the voltage limit
    /// is disabled using the disable_voltage_limit field in the vreg_ctrl register
    /// 00000 - 0.55V
    /// 00001 - 0.60V
    /// 00010 - 0.65V
    /// 00011 - 0.70V
    /// 00100 - 0.75V
    /// 00101 - 0.80V
    /// 00110 - 0.85V
    /// 00111 - 0.90V
    /// 01000 - 0.95V
    /// 01001 - 1.00V
    /// 01010 - 1.05V
    /// 01011 - 1.10V (default)
    /// 01100 - 1.15V
    /// 01101 - 1.20V
    /// 01110 - 1.25V
    /// 01111 - 1.30V
    /// 10000 - 1.35V
    /// 10001 - 1.40V
    /// 10010 - 1.50V
    /// 10011 - 1.60V
    /// 10100 - 1.65V
    /// 10101 - 1.70V
    /// 10110 - 1.80V
    /// 10111 - 1.90V
    /// 11000 - 2.00V
    /// 11001 - 2.35V
    /// 11010 - 2.50V
    /// 11011 - 2.65V
    /// 11100 - 2.80V
    /// 11101 - 3.00V
    /// 11110 - 3.15V
    /// 11111 - 3.30V
    ///
    /// Bits `[8:4]`, mask `0x000001f0`
    pub const VSEL_OFFSET: u32 = 4;
    pub const VSEL_MASK: u32 = 0x000001f0;

    /// **HIZ**
    ///
    /// high impedance mode select
    /// 0=not in high impedance mode, 1=in high impedance mode
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const HIZ_OFFSET: u32 = 1;
    pub const HIZ_MASK: u32 = 0x00000002;
    pub const HIZ: u32 = 1 << 1;

    // ======== Field Accessors ========

    /// **UPDATE_IN_PROGRESS** `[15]` (RO)
    ///
    /// regulator state is being updated
    /// writes to the vreg register will be ignored when this field is set
    #[inline(always)]
    pub fn update_in_progress(&self) -> bool {
        (self.value.read_bits() & Self::UPDATE_IN_PROGRESS_MASK) != 0
    }

    /// **VSEL** `[8:4]` (RW)
    ///
    /// output voltage select
    /// the regulator output voltage is limited to 1.3V unless the voltage limit
    /// is disabled using the disable_voltage_limit field in the vreg_ctrl register
    /// 00000 - 0.55V
    /// 00001 - 0.60V
    /// 00010 - 0.65V
    /// 00011 - 0.70V
    /// 00100 - 0.75V
    /// 00101 - 0.80V
    /// 00110 - 0.85V
    /// 00111 - 0.90V
    /// 01000 - 0.95V
    /// 01001 - 1.00V
    /// 01010 - 1.05V
    /// 01011 - 1.10V (default)
    /// 01100 - 1.15V
    /// 01101 - 1.20V
    /// 01110 - 1.25V
    /// 01111 - 1.30V
    /// 10000 - 1.35V
    /// 10001 - 1.40V
    /// 10010 - 1.50V
    /// 10011 - 1.60V
    /// 10100 - 1.65V
    /// 10101 - 1.70V
    /// 10110 - 1.80V
    /// 10111 - 1.90V
    /// 11000 - 2.00V
    /// 11001 - 2.35V
    /// 11010 - 2.50V
    /// 11011 - 2.65V
    /// 11100 - 2.80V
    /// 11101 - 3.00V
    /// 11110 - 3.15V
    /// 11111 - 3.30V
    #[inline(always)]
    pub fn vsel(&self) -> u32 {
        (self.value.read_bits() & Self::VSEL_MASK) >> Self::VSEL_OFFSET
    }

    /// Write **VSEL** `[8:4]` (RW)
    ///
    /// output voltage select
    /// the regulator output voltage is limited to 1.3V unless the voltage limit
    /// is disabled using the disable_voltage_limit field in the vreg_ctrl register
    /// 00000 - 0.55V
    /// 00001 - 0.60V
    /// 00010 - 0.65V
    /// 00011 - 0.70V
    /// 00100 - 0.75V
    /// 00101 - 0.80V
    /// 00110 - 0.85V
    /// 00111 - 0.90V
    /// 01000 - 0.95V
    /// 01001 - 1.00V
    /// 01010 - 1.05V
    /// 01011 - 1.10V (default)
    /// 01100 - 1.15V
    /// 01101 - 1.20V
    /// 01110 - 1.25V
    /// 01111 - 1.30V
    /// 10000 - 1.35V
    /// 10001 - 1.40V
    /// 10010 - 1.50V
    /// 10011 - 1.60V
    /// 10100 - 1.65V
    /// 10101 - 1.70V
    /// 10110 - 1.80V
    /// 10111 - 1.90V
    /// 11000 - 2.00V
    /// 11001 - 2.35V
    /// 11010 - 2.50V
    /// 11011 - 2.65V
    /// 11100 - 2.80V
    /// 11101 - 3.00V
    /// 11110 - 3.15V
    /// 11111 - 3.30V
    #[inline(always)]
    pub fn write_vsel(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::VSEL_MASK) | ((value << Self::VSEL_OFFSET) & Self::VSEL_MASK));
    }

    /// **HIZ** `[1]` (RW)
    ///
    /// high impedance mode select
    /// 0=not in high impedance mode, 1=in high impedance mode
    #[inline(always)]
    pub fn hiz(&self) -> bool {
        (self.value.read_bits() & Self::HIZ_MASK) != 0
    }

    /// Set HIZ bit atomically (RW)
    #[inline(always)]
    pub fn set_hiz(&self) {
        self.set_bits(Self::HIZ_MASK);
    }

    /// Clear HIZ bit atomically (RW)
    #[inline(always)]
    pub fn clr_hiz(&self) {
        self.clr_bits(Self::HIZ_MASK);
    }

}

/// VREG value type for batch read/modify/write operations
///
/// Use `vreg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `vreg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct VregVal {
    bits: u32,
}

impl VregVal {
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
        Self { bits: 0x000000b0 }
    }

    /// Read **UPDATE_IN_PROGRESS** `[15:15]`
    ///
    /// regulator state is being updated
    /// writes to the vreg register will be ignored when this field is set
    #[inline(always)]
    pub const fn update_in_progress(&self) -> bool {
        (self.bits & Vreg::UPDATE_IN_PROGRESS_MASK) != 0
    }

    /// Read **VSEL** `[8:4]`
    ///
    /// output voltage select
    /// the regulator output voltage is limited to 1.3V unless the voltage limit
    /// is disabled using the disable_voltage_limit field in the vreg_ctrl register
    /// 00000 - 0.55V
    /// 00001 - 0.60V
    /// 00010 - 0.65V
    /// 00011 - 0.70V
    /// 00100 - 0.75V
    /// 00101 - 0.80V
    /// 00110 - 0.85V
    /// 00111 - 0.90V
    /// 01000 - 0.95V
    /// 01001 - 1.00V
    /// 01010 - 1.05V
    /// 01011 - 1.10V (default)
    /// 01100 - 1.15V
    /// 01101 - 1.20V
    /// 01110 - 1.25V
    /// 01111 - 1.30V
    /// 10000 - 1.35V
    /// 10001 - 1.40V
    /// 10010 - 1.50V
    /// 10011 - 1.60V
    /// 10100 - 1.65V
    /// 10101 - 1.70V
    /// 10110 - 1.80V
    /// 10111 - 1.90V
    /// 11000 - 2.00V
    /// 11001 - 2.35V
    /// 11010 - 2.50V
    /// 11011 - 2.65V
    /// 11100 - 2.80V
    /// 11101 - 3.00V
    /// 11110 - 3.15V
    /// 11111 - 3.30V
    #[inline(always)]
    pub const fn vsel(&self) -> u32 {
        (self.bits & Vreg::VSEL_MASK) >> Vreg::VSEL_OFFSET
    }

    /// Read **HIZ** `[1:1]`
    ///
    /// high impedance mode select
    /// 0=not in high impedance mode, 1=in high impedance mode
    #[inline(always)]
    pub const fn hiz(&self) -> bool {
        (self.bits & Vreg::HIZ_MASK) != 0
    }

    /// Set **VSEL** `[8:4]`
    ///
    /// output voltage select
    /// the regulator output voltage is limited to 1.3V unless the voltage limit
    /// is disabled using the disable_voltage_limit field in the vreg_ctrl register
    /// 00000 - 0.55V
    /// 00001 - 0.60V
    /// 00010 - 0.65V
    /// 00011 - 0.70V
    /// 00100 - 0.75V
    /// 00101 - 0.80V
    /// 00110 - 0.85V
    /// 00111 - 0.90V
    /// 01000 - 0.95V
    /// 01001 - 1.00V
    /// 01010 - 1.05V
    /// 01011 - 1.10V (default)
    /// 01100 - 1.15V
    /// 01101 - 1.20V
    /// 01110 - 1.25V
    /// 01111 - 1.30V
    /// 10000 - 1.35V
    /// 10001 - 1.40V
    /// 10010 - 1.50V
    /// 10011 - 1.60V
    /// 10100 - 1.65V
    /// 10101 - 1.70V
    /// 10110 - 1.80V
    /// 10111 - 1.90V
    /// 11000 - 2.00V
    /// 11001 - 2.35V
    /// 11010 - 2.50V
    /// 11011 - 2.65V
    /// 11100 - 2.80V
    /// 11101 - 3.00V
    /// 11110 - 3.15V
    /// 11111 - 3.30V
    #[inline(always)]
    pub const fn with_vsel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Vreg::VSEL_MASK) | ((value << Vreg::VSEL_OFFSET) & Vreg::VSEL_MASK)
        }
    }

    /// Set **HIZ** `[1:1]`
    ///
    /// high impedance mode select
    /// 0=not in high impedance mode, 1=in high impedance mode
    #[inline(always)]
    pub const fn with_hiz(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Vreg::HIZ_MASK
            } else {
                self.bits & !Vreg::HIZ_MASK
            }
        }
    }

}

impl Default for VregVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Vreg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> VregVal {
        VregVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: VregVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(VregVal) -> VregVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// VREG_LP_ENTRY Register
///
/// Voltage Regulator Low Power Entry Settings
///
/// Address offset: 0x0010
/// Reset value: 0x000000b4
///
/// ## Fields
///
/// - **VSEL** `[8:4]` (RW)
///
///   output voltage select
///   the regulator output voltage is limited to 1.3V unless the voltage limit
///   is disabled using the disable_voltage_limit field in the vreg_ctrl register
///   00000 - 0.55V
///   00001 - 0.60V
///   00010 - 0.65V
///   00011 - 0.70V
///   00100 - 0.75V
///   00101 - 0.80V
///   00110 - 0.85V
///   00111 - 0.90V
///   01000 - 0.95V
///   01001 - 1.00V
///   01010 - 1.05V
///   01011 - 1.10V (default)
///   01100 - 1.15V
///   01101 - 1.20V
///   01110 - 1.25V
///   01111 - 1.30V
///   10000 - 1.35V
///   10001 - 1.40V
///   10010 - 1.50V
///   10011 - 1.60V
///   10100 - 1.65V
///   10101 - 1.70V
///   10110 - 1.80V
///   10111 - 1.90V
///   11000 - 2.00V
///   11001 - 2.35V
///   11010 - 2.50V
///   11011 - 2.65V
///   11100 - 2.80V
///   11101 - 3.00V
///   11110 - 3.15V
///   11111 - 3.30V
///
/// - **MODE** `[2]` (RW)
///
///   selects either normal (switching) mode or low power (linear) mode
///   low power mode can only be selected for output voltages up to 1.3V
///   0 = normal mode (switching)
///   1 = low power mode (linear)
///
/// - **HIZ** `[1]` (RW)
///
///   high impedance mode select
///   0=not in high impedance mode, 1=in high impedance mode
#[repr(transparent)]
pub struct VregLpEntry {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for VregLpEntry {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl VregLpEntry {
    // ======== Field Constants ========

    /// **VSEL**
    ///
    /// output voltage select
    /// the regulator output voltage is limited to 1.3V unless the voltage limit
    /// is disabled using the disable_voltage_limit field in the vreg_ctrl register
    /// 00000 - 0.55V
    /// 00001 - 0.60V
    /// 00010 - 0.65V
    /// 00011 - 0.70V
    /// 00100 - 0.75V
    /// 00101 - 0.80V
    /// 00110 - 0.85V
    /// 00111 - 0.90V
    /// 01000 - 0.95V
    /// 01001 - 1.00V
    /// 01010 - 1.05V
    /// 01011 - 1.10V (default)
    /// 01100 - 1.15V
    /// 01101 - 1.20V
    /// 01110 - 1.25V
    /// 01111 - 1.30V
    /// 10000 - 1.35V
    /// 10001 - 1.40V
    /// 10010 - 1.50V
    /// 10011 - 1.60V
    /// 10100 - 1.65V
    /// 10101 - 1.70V
    /// 10110 - 1.80V
    /// 10111 - 1.90V
    /// 11000 - 2.00V
    /// 11001 - 2.35V
    /// 11010 - 2.50V
    /// 11011 - 2.65V
    /// 11100 - 2.80V
    /// 11101 - 3.00V
    /// 11110 - 3.15V
    /// 11111 - 3.30V
    ///
    /// Bits `[8:4]`, mask `0x000001f0`
    pub const VSEL_OFFSET: u32 = 4;
    pub const VSEL_MASK: u32 = 0x000001f0;

    /// **MODE**
    ///
    /// selects either normal (switching) mode or low power (linear) mode
    /// low power mode can only be selected for output voltages up to 1.3V
    /// 0 = normal mode (switching)
    /// 1 = low power mode (linear)
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const MODE_OFFSET: u32 = 2;
    pub const MODE_MASK: u32 = 0x00000004;
    pub const MODE: u32 = 1 << 2;

    /// **HIZ**
    ///
    /// high impedance mode select
    /// 0=not in high impedance mode, 1=in high impedance mode
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const HIZ_OFFSET: u32 = 1;
    pub const HIZ_MASK: u32 = 0x00000002;
    pub const HIZ: u32 = 1 << 1;

    // ======== Field Accessors ========

    /// **VSEL** `[8:4]` (RW)
    ///
    /// output voltage select
    /// the regulator output voltage is limited to 1.3V unless the voltage limit
    /// is disabled using the disable_voltage_limit field in the vreg_ctrl register
    /// 00000 - 0.55V
    /// 00001 - 0.60V
    /// 00010 - 0.65V
    /// 00011 - 0.70V
    /// 00100 - 0.75V
    /// 00101 - 0.80V
    /// 00110 - 0.85V
    /// 00111 - 0.90V
    /// 01000 - 0.95V
    /// 01001 - 1.00V
    /// 01010 - 1.05V
    /// 01011 - 1.10V (default)
    /// 01100 - 1.15V
    /// 01101 - 1.20V
    /// 01110 - 1.25V
    /// 01111 - 1.30V
    /// 10000 - 1.35V
    /// 10001 - 1.40V
    /// 10010 - 1.50V
    /// 10011 - 1.60V
    /// 10100 - 1.65V
    /// 10101 - 1.70V
    /// 10110 - 1.80V
    /// 10111 - 1.90V
    /// 11000 - 2.00V
    /// 11001 - 2.35V
    /// 11010 - 2.50V
    /// 11011 - 2.65V
    /// 11100 - 2.80V
    /// 11101 - 3.00V
    /// 11110 - 3.15V
    /// 11111 - 3.30V
    #[inline(always)]
    pub fn vsel(&self) -> u32 {
        (self.value.read_bits() & Self::VSEL_MASK) >> Self::VSEL_OFFSET
    }

    /// Write **VSEL** `[8:4]` (RW)
    ///
    /// output voltage select
    /// the regulator output voltage is limited to 1.3V unless the voltage limit
    /// is disabled using the disable_voltage_limit field in the vreg_ctrl register
    /// 00000 - 0.55V
    /// 00001 - 0.60V
    /// 00010 - 0.65V
    /// 00011 - 0.70V
    /// 00100 - 0.75V
    /// 00101 - 0.80V
    /// 00110 - 0.85V
    /// 00111 - 0.90V
    /// 01000 - 0.95V
    /// 01001 - 1.00V
    /// 01010 - 1.05V
    /// 01011 - 1.10V (default)
    /// 01100 - 1.15V
    /// 01101 - 1.20V
    /// 01110 - 1.25V
    /// 01111 - 1.30V
    /// 10000 - 1.35V
    /// 10001 - 1.40V
    /// 10010 - 1.50V
    /// 10011 - 1.60V
    /// 10100 - 1.65V
    /// 10101 - 1.70V
    /// 10110 - 1.80V
    /// 10111 - 1.90V
    /// 11000 - 2.00V
    /// 11001 - 2.35V
    /// 11010 - 2.50V
    /// 11011 - 2.65V
    /// 11100 - 2.80V
    /// 11101 - 3.00V
    /// 11110 - 3.15V
    /// 11111 - 3.30V
    #[inline(always)]
    pub fn write_vsel(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::VSEL_MASK) | ((value << Self::VSEL_OFFSET) & Self::VSEL_MASK));
    }

    /// **MODE** `[2]` (RW)
    ///
    /// selects either normal (switching) mode or low power (linear) mode
    /// low power mode can only be selected for output voltages up to 1.3V
    /// 0 = normal mode (switching)
    /// 1 = low power mode (linear)
    #[inline(always)]
    pub fn mode(&self) -> bool {
        (self.value.read_bits() & Self::MODE_MASK) != 0
    }

    /// Set MODE bit atomically (RW)
    #[inline(always)]
    pub fn set_mode(&self) {
        self.set_bits(Self::MODE_MASK);
    }

    /// Clear MODE bit atomically (RW)
    #[inline(always)]
    pub fn clr_mode(&self) {
        self.clr_bits(Self::MODE_MASK);
    }

    /// **HIZ** `[1]` (RW)
    ///
    /// high impedance mode select
    /// 0=not in high impedance mode, 1=in high impedance mode
    #[inline(always)]
    pub fn hiz(&self) -> bool {
        (self.value.read_bits() & Self::HIZ_MASK) != 0
    }

    /// Set HIZ bit atomically (RW)
    #[inline(always)]
    pub fn set_hiz(&self) {
        self.set_bits(Self::HIZ_MASK);
    }

    /// Clear HIZ bit atomically (RW)
    #[inline(always)]
    pub fn clr_hiz(&self) {
        self.clr_bits(Self::HIZ_MASK);
    }

}

/// VREG_LP_ENTRY value type for batch read/modify/write operations
///
/// Use `vreglpentry.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `vreglpentry.write_val(val)`.
#[derive(Clone, Copy)]
pub struct VregLpEntryVal {
    bits: u32,
}

impl VregLpEntryVal {
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
        Self { bits: 0x000000b4 }
    }

    /// Read **VSEL** `[8:4]`
    ///
    /// output voltage select
    /// the regulator output voltage is limited to 1.3V unless the voltage limit
    /// is disabled using the disable_voltage_limit field in the vreg_ctrl register
    /// 00000 - 0.55V
    /// 00001 - 0.60V
    /// 00010 - 0.65V
    /// 00011 - 0.70V
    /// 00100 - 0.75V
    /// 00101 - 0.80V
    /// 00110 - 0.85V
    /// 00111 - 0.90V
    /// 01000 - 0.95V
    /// 01001 - 1.00V
    /// 01010 - 1.05V
    /// 01011 - 1.10V (default)
    /// 01100 - 1.15V
    /// 01101 - 1.20V
    /// 01110 - 1.25V
    /// 01111 - 1.30V
    /// 10000 - 1.35V
    /// 10001 - 1.40V
    /// 10010 - 1.50V
    /// 10011 - 1.60V
    /// 10100 - 1.65V
    /// 10101 - 1.70V
    /// 10110 - 1.80V
    /// 10111 - 1.90V
    /// 11000 - 2.00V
    /// 11001 - 2.35V
    /// 11010 - 2.50V
    /// 11011 - 2.65V
    /// 11100 - 2.80V
    /// 11101 - 3.00V
    /// 11110 - 3.15V
    /// 11111 - 3.30V
    #[inline(always)]
    pub const fn vsel(&self) -> u32 {
        (self.bits & VregLpEntry::VSEL_MASK) >> VregLpEntry::VSEL_OFFSET
    }

    /// Read **MODE** `[2:2]`
    ///
    /// selects either normal (switching) mode or low power (linear) mode
    /// low power mode can only be selected for output voltages up to 1.3V
    /// 0 = normal mode (switching)
    /// 1 = low power mode (linear)
    #[inline(always)]
    pub const fn mode(&self) -> bool {
        (self.bits & VregLpEntry::MODE_MASK) != 0
    }

    /// Read **HIZ** `[1:1]`
    ///
    /// high impedance mode select
    /// 0=not in high impedance mode, 1=in high impedance mode
    #[inline(always)]
    pub const fn hiz(&self) -> bool {
        (self.bits & VregLpEntry::HIZ_MASK) != 0
    }

    /// Set **VSEL** `[8:4]`
    ///
    /// output voltage select
    /// the regulator output voltage is limited to 1.3V unless the voltage limit
    /// is disabled using the disable_voltage_limit field in the vreg_ctrl register
    /// 00000 - 0.55V
    /// 00001 - 0.60V
    /// 00010 - 0.65V
    /// 00011 - 0.70V
    /// 00100 - 0.75V
    /// 00101 - 0.80V
    /// 00110 - 0.85V
    /// 00111 - 0.90V
    /// 01000 - 0.95V
    /// 01001 - 1.00V
    /// 01010 - 1.05V
    /// 01011 - 1.10V (default)
    /// 01100 - 1.15V
    /// 01101 - 1.20V
    /// 01110 - 1.25V
    /// 01111 - 1.30V
    /// 10000 - 1.35V
    /// 10001 - 1.40V
    /// 10010 - 1.50V
    /// 10011 - 1.60V
    /// 10100 - 1.65V
    /// 10101 - 1.70V
    /// 10110 - 1.80V
    /// 10111 - 1.90V
    /// 11000 - 2.00V
    /// 11001 - 2.35V
    /// 11010 - 2.50V
    /// 11011 - 2.65V
    /// 11100 - 2.80V
    /// 11101 - 3.00V
    /// 11110 - 3.15V
    /// 11111 - 3.30V
    #[inline(always)]
    pub const fn with_vsel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !VregLpEntry::VSEL_MASK) | ((value << VregLpEntry::VSEL_OFFSET) & VregLpEntry::VSEL_MASK)
        }
    }

    /// Set **MODE** `[2:2]`
    ///
    /// selects either normal (switching) mode or low power (linear) mode
    /// low power mode can only be selected for output voltages up to 1.3V
    /// 0 = normal mode (switching)
    /// 1 = low power mode (linear)
    #[inline(always)]
    pub const fn with_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | VregLpEntry::MODE_MASK
            } else {
                self.bits & !VregLpEntry::MODE_MASK
            }
        }
    }

    /// Set **HIZ** `[1:1]`
    ///
    /// high impedance mode select
    /// 0=not in high impedance mode, 1=in high impedance mode
    #[inline(always)]
    pub const fn with_hiz(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | VregLpEntry::HIZ_MASK
            } else {
                self.bits & !VregLpEntry::HIZ_MASK
            }
        }
    }

}

impl Default for VregLpEntryVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl VregLpEntry {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> VregLpEntryVal {
        VregLpEntryVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: VregLpEntryVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(VregLpEntryVal) -> VregLpEntryVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// VREG_LP_EXIT Register
///
/// Voltage Regulator Low Power Exit Settings
///
/// Address offset: 0x0014
/// Reset value: 0x000000b0
///
/// ## Fields
///
/// - **VSEL** `[8:4]` (RW)
///
///   output voltage select
///   the regulator output voltage is limited to 1.3V unless the voltage limit
///   is disabled using the disable_voltage_limit field in the vreg_ctrl register
///   00000 - 0.55V
///   00001 - 0.60V
///   00010 - 0.65V
///   00011 - 0.70V
///   00100 - 0.75V
///   00101 - 0.80V
///   00110 - 0.85V
///   00111 - 0.90V
///   01000 - 0.95V
///   01001 - 1.00V
///   01010 - 1.05V
///   01011 - 1.10V (default)
///   01100 - 1.15V
///   01101 - 1.20V
///   01110 - 1.25V
///   01111 - 1.30V
///   10000 - 1.35V
///   10001 - 1.40V
///   10010 - 1.50V
///   10011 - 1.60V
///   10100 - 1.65V
///   10101 - 1.70V
///   10110 - 1.80V
///   10111 - 1.90V
///   11000 - 2.00V
///   11001 - 2.35V
///   11010 - 2.50V
///   11011 - 2.65V
///   11100 - 2.80V
///   11101 - 3.00V
///   11110 - 3.15V
///   11111 - 3.30V
///
/// - **MODE** `[2]` (RW)
///
///   selects either normal (switching) mode or low power (linear) mode
///   low power mode can only be selected for output voltages up to 1.3V
///   0 = normal mode (switching)
///   1 = low power mode (linear)
///
/// - **HIZ** `[1]` (RW)
///
///   high impedance mode select
///   0=not in high impedance mode, 1=in high impedance mode
#[repr(transparent)]
pub struct VregLpExit {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for VregLpExit {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl VregLpExit {
    // ======== Field Constants ========

    /// **VSEL**
    ///
    /// output voltage select
    /// the regulator output voltage is limited to 1.3V unless the voltage limit
    /// is disabled using the disable_voltage_limit field in the vreg_ctrl register
    /// 00000 - 0.55V
    /// 00001 - 0.60V
    /// 00010 - 0.65V
    /// 00011 - 0.70V
    /// 00100 - 0.75V
    /// 00101 - 0.80V
    /// 00110 - 0.85V
    /// 00111 - 0.90V
    /// 01000 - 0.95V
    /// 01001 - 1.00V
    /// 01010 - 1.05V
    /// 01011 - 1.10V (default)
    /// 01100 - 1.15V
    /// 01101 - 1.20V
    /// 01110 - 1.25V
    /// 01111 - 1.30V
    /// 10000 - 1.35V
    /// 10001 - 1.40V
    /// 10010 - 1.50V
    /// 10011 - 1.60V
    /// 10100 - 1.65V
    /// 10101 - 1.70V
    /// 10110 - 1.80V
    /// 10111 - 1.90V
    /// 11000 - 2.00V
    /// 11001 - 2.35V
    /// 11010 - 2.50V
    /// 11011 - 2.65V
    /// 11100 - 2.80V
    /// 11101 - 3.00V
    /// 11110 - 3.15V
    /// 11111 - 3.30V
    ///
    /// Bits `[8:4]`, mask `0x000001f0`
    pub const VSEL_OFFSET: u32 = 4;
    pub const VSEL_MASK: u32 = 0x000001f0;

    /// **MODE**
    ///
    /// selects either normal (switching) mode or low power (linear) mode
    /// low power mode can only be selected for output voltages up to 1.3V
    /// 0 = normal mode (switching)
    /// 1 = low power mode (linear)
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const MODE_OFFSET: u32 = 2;
    pub const MODE_MASK: u32 = 0x00000004;
    pub const MODE: u32 = 1 << 2;

    /// **HIZ**
    ///
    /// high impedance mode select
    /// 0=not in high impedance mode, 1=in high impedance mode
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const HIZ_OFFSET: u32 = 1;
    pub const HIZ_MASK: u32 = 0x00000002;
    pub const HIZ: u32 = 1 << 1;

    // ======== Field Accessors ========

    /// **VSEL** `[8:4]` (RW)
    ///
    /// output voltage select
    /// the regulator output voltage is limited to 1.3V unless the voltage limit
    /// is disabled using the disable_voltage_limit field in the vreg_ctrl register
    /// 00000 - 0.55V
    /// 00001 - 0.60V
    /// 00010 - 0.65V
    /// 00011 - 0.70V
    /// 00100 - 0.75V
    /// 00101 - 0.80V
    /// 00110 - 0.85V
    /// 00111 - 0.90V
    /// 01000 - 0.95V
    /// 01001 - 1.00V
    /// 01010 - 1.05V
    /// 01011 - 1.10V (default)
    /// 01100 - 1.15V
    /// 01101 - 1.20V
    /// 01110 - 1.25V
    /// 01111 - 1.30V
    /// 10000 - 1.35V
    /// 10001 - 1.40V
    /// 10010 - 1.50V
    /// 10011 - 1.60V
    /// 10100 - 1.65V
    /// 10101 - 1.70V
    /// 10110 - 1.80V
    /// 10111 - 1.90V
    /// 11000 - 2.00V
    /// 11001 - 2.35V
    /// 11010 - 2.50V
    /// 11011 - 2.65V
    /// 11100 - 2.80V
    /// 11101 - 3.00V
    /// 11110 - 3.15V
    /// 11111 - 3.30V
    #[inline(always)]
    pub fn vsel(&self) -> u32 {
        (self.value.read_bits() & Self::VSEL_MASK) >> Self::VSEL_OFFSET
    }

    /// Write **VSEL** `[8:4]` (RW)
    ///
    /// output voltage select
    /// the regulator output voltage is limited to 1.3V unless the voltage limit
    /// is disabled using the disable_voltage_limit field in the vreg_ctrl register
    /// 00000 - 0.55V
    /// 00001 - 0.60V
    /// 00010 - 0.65V
    /// 00011 - 0.70V
    /// 00100 - 0.75V
    /// 00101 - 0.80V
    /// 00110 - 0.85V
    /// 00111 - 0.90V
    /// 01000 - 0.95V
    /// 01001 - 1.00V
    /// 01010 - 1.05V
    /// 01011 - 1.10V (default)
    /// 01100 - 1.15V
    /// 01101 - 1.20V
    /// 01110 - 1.25V
    /// 01111 - 1.30V
    /// 10000 - 1.35V
    /// 10001 - 1.40V
    /// 10010 - 1.50V
    /// 10011 - 1.60V
    /// 10100 - 1.65V
    /// 10101 - 1.70V
    /// 10110 - 1.80V
    /// 10111 - 1.90V
    /// 11000 - 2.00V
    /// 11001 - 2.35V
    /// 11010 - 2.50V
    /// 11011 - 2.65V
    /// 11100 - 2.80V
    /// 11101 - 3.00V
    /// 11110 - 3.15V
    /// 11111 - 3.30V
    #[inline(always)]
    pub fn write_vsel(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::VSEL_MASK) | ((value << Self::VSEL_OFFSET) & Self::VSEL_MASK));
    }

    /// **MODE** `[2]` (RW)
    ///
    /// selects either normal (switching) mode or low power (linear) mode
    /// low power mode can only be selected for output voltages up to 1.3V
    /// 0 = normal mode (switching)
    /// 1 = low power mode (linear)
    #[inline(always)]
    pub fn mode(&self) -> bool {
        (self.value.read_bits() & Self::MODE_MASK) != 0
    }

    /// Set MODE bit atomically (RW)
    #[inline(always)]
    pub fn set_mode(&self) {
        self.set_bits(Self::MODE_MASK);
    }

    /// Clear MODE bit atomically (RW)
    #[inline(always)]
    pub fn clr_mode(&self) {
        self.clr_bits(Self::MODE_MASK);
    }

    /// **HIZ** `[1]` (RW)
    ///
    /// high impedance mode select
    /// 0=not in high impedance mode, 1=in high impedance mode
    #[inline(always)]
    pub fn hiz(&self) -> bool {
        (self.value.read_bits() & Self::HIZ_MASK) != 0
    }

    /// Set HIZ bit atomically (RW)
    #[inline(always)]
    pub fn set_hiz(&self) {
        self.set_bits(Self::HIZ_MASK);
    }

    /// Clear HIZ bit atomically (RW)
    #[inline(always)]
    pub fn clr_hiz(&self) {
        self.clr_bits(Self::HIZ_MASK);
    }

}

/// VREG_LP_EXIT value type for batch read/modify/write operations
///
/// Use `vreglpexit.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `vreglpexit.write_val(val)`.
#[derive(Clone, Copy)]
pub struct VregLpExitVal {
    bits: u32,
}

impl VregLpExitVal {
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
        Self { bits: 0x000000b0 }
    }

    /// Read **VSEL** `[8:4]`
    ///
    /// output voltage select
    /// the regulator output voltage is limited to 1.3V unless the voltage limit
    /// is disabled using the disable_voltage_limit field in the vreg_ctrl register
    /// 00000 - 0.55V
    /// 00001 - 0.60V
    /// 00010 - 0.65V
    /// 00011 - 0.70V
    /// 00100 - 0.75V
    /// 00101 - 0.80V
    /// 00110 - 0.85V
    /// 00111 - 0.90V
    /// 01000 - 0.95V
    /// 01001 - 1.00V
    /// 01010 - 1.05V
    /// 01011 - 1.10V (default)
    /// 01100 - 1.15V
    /// 01101 - 1.20V
    /// 01110 - 1.25V
    /// 01111 - 1.30V
    /// 10000 - 1.35V
    /// 10001 - 1.40V
    /// 10010 - 1.50V
    /// 10011 - 1.60V
    /// 10100 - 1.65V
    /// 10101 - 1.70V
    /// 10110 - 1.80V
    /// 10111 - 1.90V
    /// 11000 - 2.00V
    /// 11001 - 2.35V
    /// 11010 - 2.50V
    /// 11011 - 2.65V
    /// 11100 - 2.80V
    /// 11101 - 3.00V
    /// 11110 - 3.15V
    /// 11111 - 3.30V
    #[inline(always)]
    pub const fn vsel(&self) -> u32 {
        (self.bits & VregLpExit::VSEL_MASK) >> VregLpExit::VSEL_OFFSET
    }

    /// Read **MODE** `[2:2]`
    ///
    /// selects either normal (switching) mode or low power (linear) mode
    /// low power mode can only be selected for output voltages up to 1.3V
    /// 0 = normal mode (switching)
    /// 1 = low power mode (linear)
    #[inline(always)]
    pub const fn mode(&self) -> bool {
        (self.bits & VregLpExit::MODE_MASK) != 0
    }

    /// Read **HIZ** `[1:1]`
    ///
    /// high impedance mode select
    /// 0=not in high impedance mode, 1=in high impedance mode
    #[inline(always)]
    pub const fn hiz(&self) -> bool {
        (self.bits & VregLpExit::HIZ_MASK) != 0
    }

    /// Set **VSEL** `[8:4]`
    ///
    /// output voltage select
    /// the regulator output voltage is limited to 1.3V unless the voltage limit
    /// is disabled using the disable_voltage_limit field in the vreg_ctrl register
    /// 00000 - 0.55V
    /// 00001 - 0.60V
    /// 00010 - 0.65V
    /// 00011 - 0.70V
    /// 00100 - 0.75V
    /// 00101 - 0.80V
    /// 00110 - 0.85V
    /// 00111 - 0.90V
    /// 01000 - 0.95V
    /// 01001 - 1.00V
    /// 01010 - 1.05V
    /// 01011 - 1.10V (default)
    /// 01100 - 1.15V
    /// 01101 - 1.20V
    /// 01110 - 1.25V
    /// 01111 - 1.30V
    /// 10000 - 1.35V
    /// 10001 - 1.40V
    /// 10010 - 1.50V
    /// 10011 - 1.60V
    /// 10100 - 1.65V
    /// 10101 - 1.70V
    /// 10110 - 1.80V
    /// 10111 - 1.90V
    /// 11000 - 2.00V
    /// 11001 - 2.35V
    /// 11010 - 2.50V
    /// 11011 - 2.65V
    /// 11100 - 2.80V
    /// 11101 - 3.00V
    /// 11110 - 3.15V
    /// 11111 - 3.30V
    #[inline(always)]
    pub const fn with_vsel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !VregLpExit::VSEL_MASK) | ((value << VregLpExit::VSEL_OFFSET) & VregLpExit::VSEL_MASK)
        }
    }

    /// Set **MODE** `[2:2]`
    ///
    /// selects either normal (switching) mode or low power (linear) mode
    /// low power mode can only be selected for output voltages up to 1.3V
    /// 0 = normal mode (switching)
    /// 1 = low power mode (linear)
    #[inline(always)]
    pub const fn with_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | VregLpExit::MODE_MASK
            } else {
                self.bits & !VregLpExit::MODE_MASK
            }
        }
    }

    /// Set **HIZ** `[1:1]`
    ///
    /// high impedance mode select
    /// 0=not in high impedance mode, 1=in high impedance mode
    #[inline(always)]
    pub const fn with_hiz(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | VregLpExit::HIZ_MASK
            } else {
                self.bits & !VregLpExit::HIZ_MASK
            }
        }
    }

}

impl Default for VregLpExitVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl VregLpExit {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> VregLpExitVal {
        VregLpExitVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: VregLpExitVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(VregLpExitVal) -> VregLpExitVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// BOD_CTRL Register
///
/// Brown-out Detection Control
///
/// Address offset: 0x0018
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **ISOLATE** `[12]` (RW)
///
///   isolates the brown-out detection control interface
///   0 - not isolated (default)
///   1 - isolated
#[repr(transparent)]
pub struct BodCtrl {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for BodCtrl {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl BodCtrl {
    // ======== Field Constants ========

    /// **ISOLATE**
    ///
    /// isolates the brown-out detection control interface
    /// 0 - not isolated (default)
    /// 1 - isolated
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const ISOLATE_OFFSET: u32 = 12;
    pub const ISOLATE_MASK: u32 = 0x00001000;
    pub const ISOLATE: u32 = 1 << 12;

    // ======== Field Accessors ========

    /// **ISOLATE** `[12]` (RW)
    ///
    /// isolates the brown-out detection control interface
    /// 0 - not isolated (default)
    /// 1 - isolated
    #[inline(always)]
    pub fn isolate(&self) -> bool {
        (self.value.read_bits() & Self::ISOLATE_MASK) != 0
    }

    /// Set ISOLATE bit atomically (RW)
    #[inline(always)]
    pub fn set_isolate(&self) {
        self.set_bits(Self::ISOLATE_MASK);
    }

    /// Clear ISOLATE bit atomically (RW)
    #[inline(always)]
    pub fn clr_isolate(&self) {
        self.clr_bits(Self::ISOLATE_MASK);
    }

}

/// BOD_CTRL value type for batch read/modify/write operations
///
/// Use `bodctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bodctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BodCtrlVal {
    bits: u32,
}

impl BodCtrlVal {
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

    /// Read **ISOLATE** `[12:12]`
    ///
    /// isolates the brown-out detection control interface
    /// 0 - not isolated (default)
    /// 1 - isolated
    #[inline(always)]
    pub const fn isolate(&self) -> bool {
        (self.bits & BodCtrl::ISOLATE_MASK) != 0
    }

    /// Set **ISOLATE** `[12:12]`
    ///
    /// isolates the brown-out detection control interface
    /// 0 - not isolated (default)
    /// 1 - isolated
    #[inline(always)]
    pub const fn with_isolate(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | BodCtrl::ISOLATE_MASK
            } else {
                self.bits & !BodCtrl::ISOLATE_MASK
            }
        }
    }

}

impl Default for BodCtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl BodCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> BodCtrlVal {
        BodCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: BodCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(BodCtrlVal) -> BodCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// BOD Register
///
/// Brown-out Detection Settings
///
/// Address offset: 0x001c
/// Reset value: 0x000000b1
///
/// ## Fields
///
/// - **VSEL** `[8:4]` (RW)
///
///   threshold select
///   00000 - 0.473V
///   00001 - 0.516V
///   00010 - 0.559V
///   00011 - 0.602V
///   00100 - 0.645VS
///   00101 - 0.688V
///   00110 - 0.731V
///   00111 - 0.774V
///   01000 - 0.817V
///   01001 - 0.860V (default)
///   01010 - 0.903V
///   01011 - 0.946V
///   01100 - 0.989V
///   01101 - 1.032V
///   01110 - 1.075V
///   01111 - 1.118V
///   10000 - 1.161
///   10001 - 1.204V
///
/// - **EN** `[0]` (RW)
///
///   enable brown-out detection
///   0=not enabled, 1=enabled
#[repr(transparent)]
pub struct Bod {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Bod {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Bod {
    // ======== Field Constants ========

    /// **VSEL**
    ///
    /// threshold select
    /// 00000 - 0.473V
    /// 00001 - 0.516V
    /// 00010 - 0.559V
    /// 00011 - 0.602V
    /// 00100 - 0.645VS
    /// 00101 - 0.688V
    /// 00110 - 0.731V
    /// 00111 - 0.774V
    /// 01000 - 0.817V
    /// 01001 - 0.860V (default)
    /// 01010 - 0.903V
    /// 01011 - 0.946V
    /// 01100 - 0.989V
    /// 01101 - 1.032V
    /// 01110 - 1.075V
    /// 01111 - 1.118V
    /// 10000 - 1.161
    /// 10001 - 1.204V
    ///
    /// Bits `[8:4]`, mask `0x000001f0`
    pub const VSEL_OFFSET: u32 = 4;
    pub const VSEL_MASK: u32 = 0x000001f0;

    /// **EN**
    ///
    /// enable brown-out detection
    /// 0=not enabled, 1=enabled
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const EN_OFFSET: u32 = 0;
    pub const EN_MASK: u32 = 0x00000001;
    pub const EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **VSEL** `[8:4]` (RW)
    ///
    /// threshold select
    /// 00000 - 0.473V
    /// 00001 - 0.516V
    /// 00010 - 0.559V
    /// 00011 - 0.602V
    /// 00100 - 0.645VS
    /// 00101 - 0.688V
    /// 00110 - 0.731V
    /// 00111 - 0.774V
    /// 01000 - 0.817V
    /// 01001 - 0.860V (default)
    /// 01010 - 0.903V
    /// 01011 - 0.946V
    /// 01100 - 0.989V
    /// 01101 - 1.032V
    /// 01110 - 1.075V
    /// 01111 - 1.118V
    /// 10000 - 1.161
    /// 10001 - 1.204V
    #[inline(always)]
    pub fn vsel(&self) -> u32 {
        (self.value.read_bits() & Self::VSEL_MASK) >> Self::VSEL_OFFSET
    }

    /// Write **VSEL** `[8:4]` (RW)
    ///
    /// threshold select
    /// 00000 - 0.473V
    /// 00001 - 0.516V
    /// 00010 - 0.559V
    /// 00011 - 0.602V
    /// 00100 - 0.645VS
    /// 00101 - 0.688V
    /// 00110 - 0.731V
    /// 00111 - 0.774V
    /// 01000 - 0.817V
    /// 01001 - 0.860V (default)
    /// 01010 - 0.903V
    /// 01011 - 0.946V
    /// 01100 - 0.989V
    /// 01101 - 1.032V
    /// 01110 - 1.075V
    /// 01111 - 1.118V
    /// 10000 - 1.161
    /// 10001 - 1.204V
    #[inline(always)]
    pub fn write_vsel(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::VSEL_MASK) | ((value << Self::VSEL_OFFSET) & Self::VSEL_MASK));
    }

    /// **EN** `[0]` (RW)
    ///
    /// enable brown-out detection
    /// 0=not enabled, 1=enabled
    #[inline(always)]
    pub fn en(&self) -> bool {
        (self.value.read_bits() & Self::EN_MASK) != 0
    }

    /// Set EN bit atomically (RW)
    #[inline(always)]
    pub fn set_en(&self) {
        self.set_bits(Self::EN_MASK);
    }

    /// Clear EN bit atomically (RW)
    #[inline(always)]
    pub fn clr_en(&self) {
        self.clr_bits(Self::EN_MASK);
    }

}

/// BOD value type for batch read/modify/write operations
///
/// Use `bod.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bod.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BodVal {
    bits: u32,
}

impl BodVal {
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
        Self { bits: 0x000000b1 }
    }

    /// Read **VSEL** `[8:4]`
    ///
    /// threshold select
    /// 00000 - 0.473V
    /// 00001 - 0.516V
    /// 00010 - 0.559V
    /// 00011 - 0.602V
    /// 00100 - 0.645VS
    /// 00101 - 0.688V
    /// 00110 - 0.731V
    /// 00111 - 0.774V
    /// 01000 - 0.817V
    /// 01001 - 0.860V (default)
    /// 01010 - 0.903V
    /// 01011 - 0.946V
    /// 01100 - 0.989V
    /// 01101 - 1.032V
    /// 01110 - 1.075V
    /// 01111 - 1.118V
    /// 10000 - 1.161
    /// 10001 - 1.204V
    #[inline(always)]
    pub const fn vsel(&self) -> u32 {
        (self.bits & Bod::VSEL_MASK) >> Bod::VSEL_OFFSET
    }

    /// Read **EN** `[0:0]`
    ///
    /// enable brown-out detection
    /// 0=not enabled, 1=enabled
    #[inline(always)]
    pub const fn en(&self) -> bool {
        (self.bits & Bod::EN_MASK) != 0
    }

    /// Set **VSEL** `[8:4]`
    ///
    /// threshold select
    /// 00000 - 0.473V
    /// 00001 - 0.516V
    /// 00010 - 0.559V
    /// 00011 - 0.602V
    /// 00100 - 0.645VS
    /// 00101 - 0.688V
    /// 00110 - 0.731V
    /// 00111 - 0.774V
    /// 01000 - 0.817V
    /// 01001 - 0.860V (default)
    /// 01010 - 0.903V
    /// 01011 - 0.946V
    /// 01100 - 0.989V
    /// 01101 - 1.032V
    /// 01110 - 1.075V
    /// 01111 - 1.118V
    /// 10000 - 1.161
    /// 10001 - 1.204V
    #[inline(always)]
    pub const fn with_vsel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Bod::VSEL_MASK) | ((value << Bod::VSEL_OFFSET) & Bod::VSEL_MASK)
        }
    }

    /// Set **EN** `[0:0]`
    ///
    /// enable brown-out detection
    /// 0=not enabled, 1=enabled
    #[inline(always)]
    pub const fn with_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bod::EN_MASK
            } else {
                self.bits & !Bod::EN_MASK
            }
        }
    }

}

impl Default for BodVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Bod {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> BodVal {
        BodVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: BodVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(BodVal) -> BodVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// BOD_LP_ENTRY Register
///
/// Brown-out Detection Low Power Entry Settings
///
/// Address offset: 0x0020
/// Reset value: 0x000000b0
///
/// ## Fields
///
/// - **VSEL** `[8:4]` (RW)
///
///   threshold select
///   00000 - 0.473V
///   00001 - 0.516V
///   00010 - 0.559V
///   00011 - 0.602V
///   00100 - 0.645VS
///   00101 - 0.688V
///   00110 - 0.731V
///   00111 - 0.774V
///   01000 - 0.817V
///   01001 - 0.860V (default)
///   01010 - 0.903V
///   01011 - 0.946V
///   01100 - 0.989V
///   01101 - 1.032V
///   01110 - 1.075V
///   01111 - 1.118V
///   10000 - 1.161
///   10001 - 1.204V
///
/// - **EN** `[0]` (RW)
///
///   enable brown-out detection
///   0=not enabled, 1=enabled
#[repr(transparent)]
pub struct BodLpEntry {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for BodLpEntry {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl BodLpEntry {
    // ======== Field Constants ========

    /// **VSEL**
    ///
    /// threshold select
    /// 00000 - 0.473V
    /// 00001 - 0.516V
    /// 00010 - 0.559V
    /// 00011 - 0.602V
    /// 00100 - 0.645VS
    /// 00101 - 0.688V
    /// 00110 - 0.731V
    /// 00111 - 0.774V
    /// 01000 - 0.817V
    /// 01001 - 0.860V (default)
    /// 01010 - 0.903V
    /// 01011 - 0.946V
    /// 01100 - 0.989V
    /// 01101 - 1.032V
    /// 01110 - 1.075V
    /// 01111 - 1.118V
    /// 10000 - 1.161
    /// 10001 - 1.204V
    ///
    /// Bits `[8:4]`, mask `0x000001f0`
    pub const VSEL_OFFSET: u32 = 4;
    pub const VSEL_MASK: u32 = 0x000001f0;

    /// **EN**
    ///
    /// enable brown-out detection
    /// 0=not enabled, 1=enabled
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const EN_OFFSET: u32 = 0;
    pub const EN_MASK: u32 = 0x00000001;
    pub const EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **VSEL** `[8:4]` (RW)
    ///
    /// threshold select
    /// 00000 - 0.473V
    /// 00001 - 0.516V
    /// 00010 - 0.559V
    /// 00011 - 0.602V
    /// 00100 - 0.645VS
    /// 00101 - 0.688V
    /// 00110 - 0.731V
    /// 00111 - 0.774V
    /// 01000 - 0.817V
    /// 01001 - 0.860V (default)
    /// 01010 - 0.903V
    /// 01011 - 0.946V
    /// 01100 - 0.989V
    /// 01101 - 1.032V
    /// 01110 - 1.075V
    /// 01111 - 1.118V
    /// 10000 - 1.161
    /// 10001 - 1.204V
    #[inline(always)]
    pub fn vsel(&self) -> u32 {
        (self.value.read_bits() & Self::VSEL_MASK) >> Self::VSEL_OFFSET
    }

    /// Write **VSEL** `[8:4]` (RW)
    ///
    /// threshold select
    /// 00000 - 0.473V
    /// 00001 - 0.516V
    /// 00010 - 0.559V
    /// 00011 - 0.602V
    /// 00100 - 0.645VS
    /// 00101 - 0.688V
    /// 00110 - 0.731V
    /// 00111 - 0.774V
    /// 01000 - 0.817V
    /// 01001 - 0.860V (default)
    /// 01010 - 0.903V
    /// 01011 - 0.946V
    /// 01100 - 0.989V
    /// 01101 - 1.032V
    /// 01110 - 1.075V
    /// 01111 - 1.118V
    /// 10000 - 1.161
    /// 10001 - 1.204V
    #[inline(always)]
    pub fn write_vsel(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::VSEL_MASK) | ((value << Self::VSEL_OFFSET) & Self::VSEL_MASK));
    }

    /// **EN** `[0]` (RW)
    ///
    /// enable brown-out detection
    /// 0=not enabled, 1=enabled
    #[inline(always)]
    pub fn en(&self) -> bool {
        (self.value.read_bits() & Self::EN_MASK) != 0
    }

    /// Set EN bit atomically (RW)
    #[inline(always)]
    pub fn set_en(&self) {
        self.set_bits(Self::EN_MASK);
    }

    /// Clear EN bit atomically (RW)
    #[inline(always)]
    pub fn clr_en(&self) {
        self.clr_bits(Self::EN_MASK);
    }

}

/// BOD_LP_ENTRY value type for batch read/modify/write operations
///
/// Use `bodlpentry.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bodlpentry.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BodLpEntryVal {
    bits: u32,
}

impl BodLpEntryVal {
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
        Self { bits: 0x000000b0 }
    }

    /// Read **VSEL** `[8:4]`
    ///
    /// threshold select
    /// 00000 - 0.473V
    /// 00001 - 0.516V
    /// 00010 - 0.559V
    /// 00011 - 0.602V
    /// 00100 - 0.645VS
    /// 00101 - 0.688V
    /// 00110 - 0.731V
    /// 00111 - 0.774V
    /// 01000 - 0.817V
    /// 01001 - 0.860V (default)
    /// 01010 - 0.903V
    /// 01011 - 0.946V
    /// 01100 - 0.989V
    /// 01101 - 1.032V
    /// 01110 - 1.075V
    /// 01111 - 1.118V
    /// 10000 - 1.161
    /// 10001 - 1.204V
    #[inline(always)]
    pub const fn vsel(&self) -> u32 {
        (self.bits & BodLpEntry::VSEL_MASK) >> BodLpEntry::VSEL_OFFSET
    }

    /// Read **EN** `[0:0]`
    ///
    /// enable brown-out detection
    /// 0=not enabled, 1=enabled
    #[inline(always)]
    pub const fn en(&self) -> bool {
        (self.bits & BodLpEntry::EN_MASK) != 0
    }

    /// Set **VSEL** `[8:4]`
    ///
    /// threshold select
    /// 00000 - 0.473V
    /// 00001 - 0.516V
    /// 00010 - 0.559V
    /// 00011 - 0.602V
    /// 00100 - 0.645VS
    /// 00101 - 0.688V
    /// 00110 - 0.731V
    /// 00111 - 0.774V
    /// 01000 - 0.817V
    /// 01001 - 0.860V (default)
    /// 01010 - 0.903V
    /// 01011 - 0.946V
    /// 01100 - 0.989V
    /// 01101 - 1.032V
    /// 01110 - 1.075V
    /// 01111 - 1.118V
    /// 10000 - 1.161
    /// 10001 - 1.204V
    #[inline(always)]
    pub const fn with_vsel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BodLpEntry::VSEL_MASK) | ((value << BodLpEntry::VSEL_OFFSET) & BodLpEntry::VSEL_MASK)
        }
    }

    /// Set **EN** `[0:0]`
    ///
    /// enable brown-out detection
    /// 0=not enabled, 1=enabled
    #[inline(always)]
    pub const fn with_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | BodLpEntry::EN_MASK
            } else {
                self.bits & !BodLpEntry::EN_MASK
            }
        }
    }

}

impl Default for BodLpEntryVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl BodLpEntry {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> BodLpEntryVal {
        BodLpEntryVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: BodLpEntryVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(BodLpEntryVal) -> BodLpEntryVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// BOD_LP_EXIT Register
///
/// Brown-out Detection Low Power Exit Settings
///
/// Address offset: 0x0024
/// Reset value: 0x000000b1
///
/// ## Fields
///
/// - **VSEL** `[8:4]` (RW)
///
///   threshold select
///   00000 - 0.473V
///   00001 - 0.516V
///   00010 - 0.559V
///   00011 - 0.602V
///   00100 - 0.645VS
///   00101 - 0.688V
///   00110 - 0.731V
///   00111 - 0.774V
///   01000 - 0.817V
///   01001 - 0.860V (default)
///   01010 - 0.903V
///   01011 - 0.946V
///   01100 - 0.989V
///   01101 - 1.032V
///   01110 - 1.075V
///   01111 - 1.118V
///   10000 - 1.161
///   10001 - 1.204V
///
/// - **EN** `[0]` (RW)
///
///   enable brown-out detection
///   0=not enabled, 1=enabled
#[repr(transparent)]
pub struct BodLpExit {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for BodLpExit {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl BodLpExit {
    // ======== Field Constants ========

    /// **VSEL**
    ///
    /// threshold select
    /// 00000 - 0.473V
    /// 00001 - 0.516V
    /// 00010 - 0.559V
    /// 00011 - 0.602V
    /// 00100 - 0.645VS
    /// 00101 - 0.688V
    /// 00110 - 0.731V
    /// 00111 - 0.774V
    /// 01000 - 0.817V
    /// 01001 - 0.860V (default)
    /// 01010 - 0.903V
    /// 01011 - 0.946V
    /// 01100 - 0.989V
    /// 01101 - 1.032V
    /// 01110 - 1.075V
    /// 01111 - 1.118V
    /// 10000 - 1.161
    /// 10001 - 1.204V
    ///
    /// Bits `[8:4]`, mask `0x000001f0`
    pub const VSEL_OFFSET: u32 = 4;
    pub const VSEL_MASK: u32 = 0x000001f0;

    /// **EN**
    ///
    /// enable brown-out detection
    /// 0=not enabled, 1=enabled
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const EN_OFFSET: u32 = 0;
    pub const EN_MASK: u32 = 0x00000001;
    pub const EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **VSEL** `[8:4]` (RW)
    ///
    /// threshold select
    /// 00000 - 0.473V
    /// 00001 - 0.516V
    /// 00010 - 0.559V
    /// 00011 - 0.602V
    /// 00100 - 0.645VS
    /// 00101 - 0.688V
    /// 00110 - 0.731V
    /// 00111 - 0.774V
    /// 01000 - 0.817V
    /// 01001 - 0.860V (default)
    /// 01010 - 0.903V
    /// 01011 - 0.946V
    /// 01100 - 0.989V
    /// 01101 - 1.032V
    /// 01110 - 1.075V
    /// 01111 - 1.118V
    /// 10000 - 1.161
    /// 10001 - 1.204V
    #[inline(always)]
    pub fn vsel(&self) -> u32 {
        (self.value.read_bits() & Self::VSEL_MASK) >> Self::VSEL_OFFSET
    }

    /// Write **VSEL** `[8:4]` (RW)
    ///
    /// threshold select
    /// 00000 - 0.473V
    /// 00001 - 0.516V
    /// 00010 - 0.559V
    /// 00011 - 0.602V
    /// 00100 - 0.645VS
    /// 00101 - 0.688V
    /// 00110 - 0.731V
    /// 00111 - 0.774V
    /// 01000 - 0.817V
    /// 01001 - 0.860V (default)
    /// 01010 - 0.903V
    /// 01011 - 0.946V
    /// 01100 - 0.989V
    /// 01101 - 1.032V
    /// 01110 - 1.075V
    /// 01111 - 1.118V
    /// 10000 - 1.161
    /// 10001 - 1.204V
    #[inline(always)]
    pub fn write_vsel(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::VSEL_MASK) | ((value << Self::VSEL_OFFSET) & Self::VSEL_MASK));
    }

    /// **EN** `[0]` (RW)
    ///
    /// enable brown-out detection
    /// 0=not enabled, 1=enabled
    #[inline(always)]
    pub fn en(&self) -> bool {
        (self.value.read_bits() & Self::EN_MASK) != 0
    }

    /// Set EN bit atomically (RW)
    #[inline(always)]
    pub fn set_en(&self) {
        self.set_bits(Self::EN_MASK);
    }

    /// Clear EN bit atomically (RW)
    #[inline(always)]
    pub fn clr_en(&self) {
        self.clr_bits(Self::EN_MASK);
    }

}

/// BOD_LP_EXIT value type for batch read/modify/write operations
///
/// Use `bodlpexit.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bodlpexit.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BodLpExitVal {
    bits: u32,
}

impl BodLpExitVal {
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
        Self { bits: 0x000000b1 }
    }

    /// Read **VSEL** `[8:4]`
    ///
    /// threshold select
    /// 00000 - 0.473V
    /// 00001 - 0.516V
    /// 00010 - 0.559V
    /// 00011 - 0.602V
    /// 00100 - 0.645VS
    /// 00101 - 0.688V
    /// 00110 - 0.731V
    /// 00111 - 0.774V
    /// 01000 - 0.817V
    /// 01001 - 0.860V (default)
    /// 01010 - 0.903V
    /// 01011 - 0.946V
    /// 01100 - 0.989V
    /// 01101 - 1.032V
    /// 01110 - 1.075V
    /// 01111 - 1.118V
    /// 10000 - 1.161
    /// 10001 - 1.204V
    #[inline(always)]
    pub const fn vsel(&self) -> u32 {
        (self.bits & BodLpExit::VSEL_MASK) >> BodLpExit::VSEL_OFFSET
    }

    /// Read **EN** `[0:0]`
    ///
    /// enable brown-out detection
    /// 0=not enabled, 1=enabled
    #[inline(always)]
    pub const fn en(&self) -> bool {
        (self.bits & BodLpExit::EN_MASK) != 0
    }

    /// Set **VSEL** `[8:4]`
    ///
    /// threshold select
    /// 00000 - 0.473V
    /// 00001 - 0.516V
    /// 00010 - 0.559V
    /// 00011 - 0.602V
    /// 00100 - 0.645VS
    /// 00101 - 0.688V
    /// 00110 - 0.731V
    /// 00111 - 0.774V
    /// 01000 - 0.817V
    /// 01001 - 0.860V (default)
    /// 01010 - 0.903V
    /// 01011 - 0.946V
    /// 01100 - 0.989V
    /// 01101 - 1.032V
    /// 01110 - 1.075V
    /// 01111 - 1.118V
    /// 10000 - 1.161
    /// 10001 - 1.204V
    #[inline(always)]
    pub const fn with_vsel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BodLpExit::VSEL_MASK) | ((value << BodLpExit::VSEL_OFFSET) & BodLpExit::VSEL_MASK)
        }
    }

    /// Set **EN** `[0:0]`
    ///
    /// enable brown-out detection
    /// 0=not enabled, 1=enabled
    #[inline(always)]
    pub const fn with_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | BodLpExit::EN_MASK
            } else {
                self.bits & !BodLpExit::EN_MASK
            }
        }
    }

}

impl Default for BodLpExitVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl BodLpExit {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> BodLpExitVal {
        BodLpExitVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: BodLpExitVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(BodLpExitVal) -> BodLpExitVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// LPOSC Register
///
/// Low power oscillator control register.
///
/// Address offset: 0x0028
/// Reset value: 0x00000203
///
/// ## Fields
///
/// - **TRIM** `[9:4]` (RW)
///
///   Frequency trim - the trim step is typically 1% of the reset frequency, but can be up to 3%
///
/// - **MODE** `[1:0]` (RW)
///
///   This feature has been removed
#[repr(transparent)]
pub struct Lposc {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Lposc {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Lposc {
    // ======== Field Constants ========

    /// **TRIM**
    ///
    /// Frequency trim - the trim step is typically 1% of the reset frequency, but can be up to 3%
    ///
    /// Bits `[9:4]`, mask `0x000003f0`
    pub const TRIM_OFFSET: u32 = 4;
    pub const TRIM_MASK: u32 = 0x000003f0;

    /// **MODE**
    ///
    /// This feature has been removed
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const MODE_OFFSET: u32 = 0;
    pub const MODE_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **TRIM** `[9:4]` (RW)
    ///
    /// Frequency trim - the trim step is typically 1% of the reset frequency, but can be up to 3%
    #[inline(always)]
    pub fn trim(&self) -> u32 {
        (self.value.read_bits() & Self::TRIM_MASK) >> Self::TRIM_OFFSET
    }

    /// Write **TRIM** `[9:4]` (RW)
    ///
    /// Frequency trim - the trim step is typically 1% of the reset frequency, but can be up to 3%
    #[inline(always)]
    pub fn write_trim(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::TRIM_MASK) | ((value << Self::TRIM_OFFSET) & Self::TRIM_MASK));
    }

    /// **MODE** `[1:0]` (RW)
    ///
    /// This feature has been removed
    #[inline(always)]
    pub fn mode(&self) -> u32 {
        (self.value.read_bits() & Self::MODE_MASK) >> Self::MODE_OFFSET
    }

    /// Write **MODE** `[1:0]` (RW)
    ///
    /// This feature has been removed
    #[inline(always)]
    pub fn write_mode(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::MODE_MASK) | ((value << Self::MODE_OFFSET) & Self::MODE_MASK));
    }

}

/// LPOSC value type for batch read/modify/write operations
///
/// Use `lposc.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lposc.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LposcVal {
    bits: u32,
}

impl LposcVal {
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
        Self { bits: 0x00000203 }
    }

    /// Read **TRIM** `[9:4]`
    ///
    /// Frequency trim - the trim step is typically 1% of the reset frequency, but can be up to 3%
    #[inline(always)]
    pub const fn trim(&self) -> u32 {
        (self.bits & Lposc::TRIM_MASK) >> Lposc::TRIM_OFFSET
    }

    /// Read **MODE** `[1:0]`
    ///
    /// This feature has been removed
    #[inline(always)]
    pub const fn mode(&self) -> u32 {
        (self.bits & Lposc::MODE_MASK) >> Lposc::MODE_OFFSET
    }

    /// Set **TRIM** `[9:4]`
    ///
    /// Frequency trim - the trim step is typically 1% of the reset frequency, but can be up to 3%
    #[inline(always)]
    pub const fn with_trim(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Lposc::TRIM_MASK) | ((value << Lposc::TRIM_OFFSET) & Lposc::TRIM_MASK)
        }
    }

    /// Set **MODE** `[1:0]`
    ///
    /// This feature has been removed
    #[inline(always)]
    pub const fn with_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Lposc::MODE_MASK) | ((value << Lposc::MODE_OFFSET) & Lposc::MODE_MASK)
        }
    }

}

impl Default for LposcVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Lposc {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LposcVal {
        LposcVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LposcVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LposcVal) -> LposcVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// CHIP_RESET Register
///
/// Chip reset control and status
///
/// Address offset: 0x002c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **HAD_WATCHDOG_RESET_RSM** `[28]` (RO)
///
///   Last reset was a watchdog timeout which was configured to reset the power-on state machine
///   This resets:
///   double_tap flag no
///   DP no
///   RPAP no
///   rescue_flag no
///   timer no
///   powman no
///   swcore no
///   psm yes
///   and does not change the power state
///
/// - **HAD_HZD_SYS_RESET_REQ** `[27]` (RO)
///
///   Last reset was a system reset from the hazard debugger
///   This resets:
///   double_tap flag no
///   DP no
///   RPAP no
///   rescue_flag no
///   timer no
///   powman no
///   swcore no
///   psm yes
///   and does not change the power state
///
/// - **HAD_GLITCH_DETECT** `[26]` (RO)
///
///   Last reset was due to a power supply glitch
///   This resets:
///   double_tap flag no
///   DP no
///   RPAP no
///   rescue_flag no
///   timer no
///   powman no
///   swcore no
///   psm yes
///   and does not change the power state
///
/// - **HAD_SWCORE_PD** `[25]` (RO)
///
///   Last reset was a switched core powerdown
///   This resets:
///   double_tap flag no
///   DP no
///   RPAP no
///   rescue_flag no
///   timer no
///   powman no
///   swcore yes
///   psm yes
///   then starts the power sequencer
///
/// - **HAD_WATCHDOG_RESET_SWCORE** `[24]` (RO)
///
///   Last reset was a watchdog timeout which was configured to reset the switched-core
///   This resets:
///   double_tap flag no
///   DP no
///   RPAP no
///   rescue_flag no
///   timer no
///   powman no
///   swcore yes
///   psm yes
///   then starts the power sequencer
///
/// - **HAD_WATCHDOG_RESET_POWMAN** `[23]` (RO)
///
///   Last reset was a watchdog timeout which was configured to reset the power manager
///   This resets:
///   double_tap flag no
///   DP no
///   RPAP no
///   rescue_flag no
///   timer yes
///   powman yes
///   swcore yes
///   psm yes
///   then starts the power sequencer
///
/// - **HAD_WATCHDOG_RESET_POWMAN_ASYNC** `[22]` (RO)
///
///   Last reset was a watchdog timeout which was configured to reset the power manager asynchronously
///   This resets:
///   double_tap flag no
///   DP no
///   RPAP no
///   rescue_flag no
///   timer yes
///   powman yes
///   swcore yes
///   psm yes
///   then starts the power sequencer
///
/// - **HAD_RESCUE** `[21]` (RO)
///
///   Last reset was a rescue reset from the debugger
///   This resets:
///   double_tap flag no
///   DP no
///   RPAP no
///   rescue_flag no, it sets this flag
///   timer yes
///   powman yes
///   swcore yes
///   psm yes
///   then starts the power sequencer
///
/// - **HAD_DP_RESET_REQ** `[19]` (RO)
///
///   Last reset was an reset request from the arm debugger
///   This resets:
///   double_tap flag no
///   DP no
///   RPAP no
///   rescue_flag yes
///   timer yes
///   powman yes
///   swcore yes
///   psm yes
///   then starts the power sequencer
///
/// - **HAD_RUN_LOW** `[18]` (RO)
///
///   Last reset was from the RUN pin
///   This resets:
///   double_tap flag no
///   DP yes
///   RPAP yes
///   rescue_flag yes
///   timer yes
///   powman yes
///   swcore yes
///   psm yes
///   then starts the power sequencer
///
/// - **HAD_BOR** `[17]` (RO)
///
///   Last reset was from the brown-out detection block
///   This resets:
///   double_tap flag yes
///   DP yes
///   RPAP yes
///   rescue_flag yes
///   timer yes
///   powman yes
///   swcore yes
///   psm yes
///   then starts the power sequencer
///
/// - **HAD_POR** `[16]` (RO)
///
///   Last reset was from the power-on reset
///   This resets:
///   double_tap flag yes
///   DP yes
///   RPAP yes
///   rescue_flag yes
///   timer yes
///   powman yes
///   swcore yes
///   psm yes
///   then starts the power sequencer
///
/// - **RESCUE_FLAG** `[4]` (RW)
///
///   This is set by a rescue reset from the RP-AP.
///   Its purpose is to halt before the bootrom before booting from flash in order to recover from a boot lock-up.
///   The debugger can then attach once the bootrom has been halted and flash some working code that does not lock up.
///
/// - **DOUBLE_TAP** `[0]` (RW)
///
///   This flag is set by double-tapping RUN. It tells bootcode to go into the bootloader.
#[repr(transparent)]
pub struct ChipReset {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for ChipReset {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ChipReset {
    // ======== Field Constants ========

    /// **HAD_WATCHDOG_RESET_RSM**
    ///
    /// Last reset was a watchdog timeout which was configured to reset the power-on state machine
    /// This resets:
    /// double_tap flag no
    /// DP no
    /// RPAP no
    /// rescue_flag no
    /// timer no
    /// powman no
    /// swcore no
    /// psm yes
    /// and does not change the power state
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const HAD_WATCHDOG_RESET_RSM_OFFSET: u32 = 28;
    pub const HAD_WATCHDOG_RESET_RSM_MASK: u32 = 0x10000000;
    pub const HAD_WATCHDOG_RESET_RSM: u32 = 1 << 28;

    /// **HAD_HZD_SYS_RESET_REQ**
    ///
    /// Last reset was a system reset from the hazard debugger
    /// This resets:
    /// double_tap flag no
    /// DP no
    /// RPAP no
    /// rescue_flag no
    /// timer no
    /// powman no
    /// swcore no
    /// psm yes
    /// and does not change the power state
    ///
    /// Bits `[27:27]`, mask `0x08000000`
    pub const HAD_HZD_SYS_RESET_REQ_OFFSET: u32 = 27;
    pub const HAD_HZD_SYS_RESET_REQ_MASK: u32 = 0x08000000;
    pub const HAD_HZD_SYS_RESET_REQ: u32 = 1 << 27;

    /// **HAD_GLITCH_DETECT**
    ///
    /// Last reset was due to a power supply glitch
    /// This resets:
    /// double_tap flag no
    /// DP no
    /// RPAP no
    /// rescue_flag no
    /// timer no
    /// powman no
    /// swcore no
    /// psm yes
    /// and does not change the power state
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const HAD_GLITCH_DETECT_OFFSET: u32 = 26;
    pub const HAD_GLITCH_DETECT_MASK: u32 = 0x04000000;
    pub const HAD_GLITCH_DETECT: u32 = 1 << 26;

    /// **HAD_SWCORE_PD**
    ///
    /// Last reset was a switched core powerdown
    /// This resets:
    /// double_tap flag no
    /// DP no
    /// RPAP no
    /// rescue_flag no
    /// timer no
    /// powman no
    /// swcore yes
    /// psm yes
    /// then starts the power sequencer
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const HAD_SWCORE_PD_OFFSET: u32 = 25;
    pub const HAD_SWCORE_PD_MASK: u32 = 0x02000000;
    pub const HAD_SWCORE_PD: u32 = 1 << 25;

    /// **HAD_WATCHDOG_RESET_SWCORE**
    ///
    /// Last reset was a watchdog timeout which was configured to reset the switched-core
    /// This resets:
    /// double_tap flag no
    /// DP no
    /// RPAP no
    /// rescue_flag no
    /// timer no
    /// powman no
    /// swcore yes
    /// psm yes
    /// then starts the power sequencer
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const HAD_WATCHDOG_RESET_SWCORE_OFFSET: u32 = 24;
    pub const HAD_WATCHDOG_RESET_SWCORE_MASK: u32 = 0x01000000;
    pub const HAD_WATCHDOG_RESET_SWCORE: u32 = 1 << 24;

    /// **HAD_WATCHDOG_RESET_POWMAN**
    ///
    /// Last reset was a watchdog timeout which was configured to reset the power manager
    /// This resets:
    /// double_tap flag no
    /// DP no
    /// RPAP no
    /// rescue_flag no
    /// timer yes
    /// powman yes
    /// swcore yes
    /// psm yes
    /// then starts the power sequencer
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const HAD_WATCHDOG_RESET_POWMAN_OFFSET: u32 = 23;
    pub const HAD_WATCHDOG_RESET_POWMAN_MASK: u32 = 0x00800000;
    pub const HAD_WATCHDOG_RESET_POWMAN: u32 = 1 << 23;

    /// **HAD_WATCHDOG_RESET_POWMAN_ASYNC**
    ///
    /// Last reset was a watchdog timeout which was configured to reset the power manager asynchronously
    /// This resets:
    /// double_tap flag no
    /// DP no
    /// RPAP no
    /// rescue_flag no
    /// timer yes
    /// powman yes
    /// swcore yes
    /// psm yes
    /// then starts the power sequencer
    ///
    /// Bits `[22:22]`, mask `0x00400000`
    pub const HAD_WATCHDOG_RESET_POWMAN_ASYNC_OFFSET: u32 = 22;
    pub const HAD_WATCHDOG_RESET_POWMAN_ASYNC_MASK: u32 = 0x00400000;
    pub const HAD_WATCHDOG_RESET_POWMAN_ASYNC: u32 = 1 << 22;

    /// **HAD_RESCUE**
    ///
    /// Last reset was a rescue reset from the debugger
    /// This resets:
    /// double_tap flag no
    /// DP no
    /// RPAP no
    /// rescue_flag no, it sets this flag
    /// timer yes
    /// powman yes
    /// swcore yes
    /// psm yes
    /// then starts the power sequencer
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const HAD_RESCUE_OFFSET: u32 = 21;
    pub const HAD_RESCUE_MASK: u32 = 0x00200000;
    pub const HAD_RESCUE: u32 = 1 << 21;

    /// **HAD_DP_RESET_REQ**
    ///
    /// Last reset was an reset request from the arm debugger
    /// This resets:
    /// double_tap flag no
    /// DP no
    /// RPAP no
    /// rescue_flag yes
    /// timer yes
    /// powman yes
    /// swcore yes
    /// psm yes
    /// then starts the power sequencer
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const HAD_DP_RESET_REQ_OFFSET: u32 = 19;
    pub const HAD_DP_RESET_REQ_MASK: u32 = 0x00080000;
    pub const HAD_DP_RESET_REQ: u32 = 1 << 19;

    /// **HAD_RUN_LOW**
    ///
    /// Last reset was from the RUN pin
    /// This resets:
    /// double_tap flag no
    /// DP yes
    /// RPAP yes
    /// rescue_flag yes
    /// timer yes
    /// powman yes
    /// swcore yes
    /// psm yes
    /// then starts the power sequencer
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const HAD_RUN_LOW_OFFSET: u32 = 18;
    pub const HAD_RUN_LOW_MASK: u32 = 0x00040000;
    pub const HAD_RUN_LOW: u32 = 1 << 18;

    /// **HAD_BOR**
    ///
    /// Last reset was from the brown-out detection block
    /// This resets:
    /// double_tap flag yes
    /// DP yes
    /// RPAP yes
    /// rescue_flag yes
    /// timer yes
    /// powman yes
    /// swcore yes
    /// psm yes
    /// then starts the power sequencer
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const HAD_BOR_OFFSET: u32 = 17;
    pub const HAD_BOR_MASK: u32 = 0x00020000;
    pub const HAD_BOR: u32 = 1 << 17;

    /// **HAD_POR**
    ///
    /// Last reset was from the power-on reset
    /// This resets:
    /// double_tap flag yes
    /// DP yes
    /// RPAP yes
    /// rescue_flag yes
    /// timer yes
    /// powman yes
    /// swcore yes
    /// psm yes
    /// then starts the power sequencer
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const HAD_POR_OFFSET: u32 = 16;
    pub const HAD_POR_MASK: u32 = 0x00010000;
    pub const HAD_POR: u32 = 1 << 16;

    /// **RESCUE_FLAG**
    ///
    /// This is set by a rescue reset from the RP-AP.
    /// Its purpose is to halt before the bootrom before booting from flash in order to recover from a boot lock-up.
    /// The debugger can then attach once the bootrom has been halted and flash some working code that does not lock up.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const RESCUE_FLAG_OFFSET: u32 = 4;
    pub const RESCUE_FLAG_MASK: u32 = 0x00000010;
    pub const RESCUE_FLAG: u32 = 1 << 4;

    /// **DOUBLE_TAP**
    ///
    /// This flag is set by double-tapping RUN. It tells bootcode to go into the bootloader.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const DOUBLE_TAP_OFFSET: u32 = 0;
    pub const DOUBLE_TAP_MASK: u32 = 0x00000001;
    pub const DOUBLE_TAP: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **HAD_WATCHDOG_RESET_RSM** `[28]` (RO)
    ///
    /// Last reset was a watchdog timeout which was configured to reset the power-on state machine
    /// This resets:
    /// double_tap flag no
    /// DP no
    /// RPAP no
    /// rescue_flag no
    /// timer no
    /// powman no
    /// swcore no
    /// psm yes
    /// and does not change the power state
    #[inline(always)]
    pub fn had_watchdog_reset_rsm(&self) -> bool {
        (self.value.read_bits() & Self::HAD_WATCHDOG_RESET_RSM_MASK) != 0
    }

    /// **HAD_HZD_SYS_RESET_REQ** `[27]` (RO)
    ///
    /// Last reset was a system reset from the hazard debugger
    /// This resets:
    /// double_tap flag no
    /// DP no
    /// RPAP no
    /// rescue_flag no
    /// timer no
    /// powman no
    /// swcore no
    /// psm yes
    /// and does not change the power state
    #[inline(always)]
    pub fn had_hzd_sys_reset_req(&self) -> bool {
        (self.value.read_bits() & Self::HAD_HZD_SYS_RESET_REQ_MASK) != 0
    }

    /// **HAD_GLITCH_DETECT** `[26]` (RO)
    ///
    /// Last reset was due to a power supply glitch
    /// This resets:
    /// double_tap flag no
    /// DP no
    /// RPAP no
    /// rescue_flag no
    /// timer no
    /// powman no
    /// swcore no
    /// psm yes
    /// and does not change the power state
    #[inline(always)]
    pub fn had_glitch_detect(&self) -> bool {
        (self.value.read_bits() & Self::HAD_GLITCH_DETECT_MASK) != 0
    }

    /// **HAD_SWCORE_PD** `[25]` (RO)
    ///
    /// Last reset was a switched core powerdown
    /// This resets:
    /// double_tap flag no
    /// DP no
    /// RPAP no
    /// rescue_flag no
    /// timer no
    /// powman no
    /// swcore yes
    /// psm yes
    /// then starts the power sequencer
    #[inline(always)]
    pub fn had_swcore_pd(&self) -> bool {
        (self.value.read_bits() & Self::HAD_SWCORE_PD_MASK) != 0
    }

    /// **HAD_WATCHDOG_RESET_SWCORE** `[24]` (RO)
    ///
    /// Last reset was a watchdog timeout which was configured to reset the switched-core
    /// This resets:
    /// double_tap flag no
    /// DP no
    /// RPAP no
    /// rescue_flag no
    /// timer no
    /// powman no
    /// swcore yes
    /// psm yes
    /// then starts the power sequencer
    #[inline(always)]
    pub fn had_watchdog_reset_swcore(&self) -> bool {
        (self.value.read_bits() & Self::HAD_WATCHDOG_RESET_SWCORE_MASK) != 0
    }

    /// **HAD_WATCHDOG_RESET_POWMAN** `[23]` (RO)
    ///
    /// Last reset was a watchdog timeout which was configured to reset the power manager
    /// This resets:
    /// double_tap flag no
    /// DP no
    /// RPAP no
    /// rescue_flag no
    /// timer yes
    /// powman yes
    /// swcore yes
    /// psm yes
    /// then starts the power sequencer
    #[inline(always)]
    pub fn had_watchdog_reset_powman(&self) -> bool {
        (self.value.read_bits() & Self::HAD_WATCHDOG_RESET_POWMAN_MASK) != 0
    }

    /// **HAD_WATCHDOG_RESET_POWMAN_ASYNC** `[22]` (RO)
    ///
    /// Last reset was a watchdog timeout which was configured to reset the power manager asynchronously
    /// This resets:
    /// double_tap flag no
    /// DP no
    /// RPAP no
    /// rescue_flag no
    /// timer yes
    /// powman yes
    /// swcore yes
    /// psm yes
    /// then starts the power sequencer
    #[inline(always)]
    pub fn had_watchdog_reset_powman_async(&self) -> bool {
        (self.value.read_bits() & Self::HAD_WATCHDOG_RESET_POWMAN_ASYNC_MASK) != 0
    }

    /// **HAD_RESCUE** `[21]` (RO)
    ///
    /// Last reset was a rescue reset from the debugger
    /// This resets:
    /// double_tap flag no
    /// DP no
    /// RPAP no
    /// rescue_flag no, it sets this flag
    /// timer yes
    /// powman yes
    /// swcore yes
    /// psm yes
    /// then starts the power sequencer
    #[inline(always)]
    pub fn had_rescue(&self) -> bool {
        (self.value.read_bits() & Self::HAD_RESCUE_MASK) != 0
    }

    /// **HAD_DP_RESET_REQ** `[19]` (RO)
    ///
    /// Last reset was an reset request from the arm debugger
    /// This resets:
    /// double_tap flag no
    /// DP no
    /// RPAP no
    /// rescue_flag yes
    /// timer yes
    /// powman yes
    /// swcore yes
    /// psm yes
    /// then starts the power sequencer
    #[inline(always)]
    pub fn had_dp_reset_req(&self) -> bool {
        (self.value.read_bits() & Self::HAD_DP_RESET_REQ_MASK) != 0
    }

    /// **HAD_RUN_LOW** `[18]` (RO)
    ///
    /// Last reset was from the RUN pin
    /// This resets:
    /// double_tap flag no
    /// DP yes
    /// RPAP yes
    /// rescue_flag yes
    /// timer yes
    /// powman yes
    /// swcore yes
    /// psm yes
    /// then starts the power sequencer
    #[inline(always)]
    pub fn had_run_low(&self) -> bool {
        (self.value.read_bits() & Self::HAD_RUN_LOW_MASK) != 0
    }

    /// **HAD_BOR** `[17]` (RO)
    ///
    /// Last reset was from the brown-out detection block
    /// This resets:
    /// double_tap flag yes
    /// DP yes
    /// RPAP yes
    /// rescue_flag yes
    /// timer yes
    /// powman yes
    /// swcore yes
    /// psm yes
    /// then starts the power sequencer
    #[inline(always)]
    pub fn had_bor(&self) -> bool {
        (self.value.read_bits() & Self::HAD_BOR_MASK) != 0
    }

    /// **HAD_POR** `[16]` (RO)
    ///
    /// Last reset was from the power-on reset
    /// This resets:
    /// double_tap flag yes
    /// DP yes
    /// RPAP yes
    /// rescue_flag yes
    /// timer yes
    /// powman yes
    /// swcore yes
    /// psm yes
    /// then starts the power sequencer
    #[inline(always)]
    pub fn had_por(&self) -> bool {
        (self.value.read_bits() & Self::HAD_POR_MASK) != 0
    }

    /// **RESCUE_FLAG** `[4]` (RW)
    ///
    /// This is set by a rescue reset from the RP-AP.
    /// Its purpose is to halt before the bootrom before booting from flash in order to recover from a boot lock-up.
    /// The debugger can then attach once the bootrom has been halted and flash some working code that does not lock up.
    #[inline(always)]
    pub fn rescue_flag(&self) -> bool {
        (self.value.read_bits() & Self::RESCUE_FLAG_MASK) != 0
    }

    /// Set RESCUE_FLAG bit atomically (RW)
    #[inline(always)]
    pub fn set_rescue_flag(&self) {
        self.set_bits(Self::RESCUE_FLAG_MASK);
    }

    /// Clear RESCUE_FLAG bit atomically (RW)
    #[inline(always)]
    pub fn clr_rescue_flag(&self) {
        self.clr_bits(Self::RESCUE_FLAG_MASK);
    }

    /// **DOUBLE_TAP** `[0]` (RW)
    ///
    /// This flag is set by double-tapping RUN. It tells bootcode to go into the bootloader.
    #[inline(always)]
    pub fn double_tap(&self) -> bool {
        (self.value.read_bits() & Self::DOUBLE_TAP_MASK) != 0
    }

    /// Set DOUBLE_TAP bit atomically (RW)
    #[inline(always)]
    pub fn set_double_tap(&self) {
        self.set_bits(Self::DOUBLE_TAP_MASK);
    }

    /// Clear DOUBLE_TAP bit atomically (RW)
    #[inline(always)]
    pub fn clr_double_tap(&self) {
        self.clr_bits(Self::DOUBLE_TAP_MASK);
    }

}

/// CHIP_RESET value type for batch read/modify/write operations
///
/// Use `chipreset.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `chipreset.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ChipResetVal {
    bits: u32,
}

impl ChipResetVal {
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

    /// Read **HAD_WATCHDOG_RESET_RSM** `[28:28]`
    ///
    /// Last reset was a watchdog timeout which was configured to reset the power-on state machine
    /// This resets:
    /// double_tap flag no
    /// DP no
    /// RPAP no
    /// rescue_flag no
    /// timer no
    /// powman no
    /// swcore no
    /// psm yes
    /// and does not change the power state
    #[inline(always)]
    pub const fn had_watchdog_reset_rsm(&self) -> bool {
        (self.bits & ChipReset::HAD_WATCHDOG_RESET_RSM_MASK) != 0
    }

    /// Read **HAD_HZD_SYS_RESET_REQ** `[27:27]`
    ///
    /// Last reset was a system reset from the hazard debugger
    /// This resets:
    /// double_tap flag no
    /// DP no
    /// RPAP no
    /// rescue_flag no
    /// timer no
    /// powman no
    /// swcore no
    /// psm yes
    /// and does not change the power state
    #[inline(always)]
    pub const fn had_hzd_sys_reset_req(&self) -> bool {
        (self.bits & ChipReset::HAD_HZD_SYS_RESET_REQ_MASK) != 0
    }

    /// Read **HAD_GLITCH_DETECT** `[26:26]`
    ///
    /// Last reset was due to a power supply glitch
    /// This resets:
    /// double_tap flag no
    /// DP no
    /// RPAP no
    /// rescue_flag no
    /// timer no
    /// powman no
    /// swcore no
    /// psm yes
    /// and does not change the power state
    #[inline(always)]
    pub const fn had_glitch_detect(&self) -> bool {
        (self.bits & ChipReset::HAD_GLITCH_DETECT_MASK) != 0
    }

    /// Read **HAD_SWCORE_PD** `[25:25]`
    ///
    /// Last reset was a switched core powerdown
    /// This resets:
    /// double_tap flag no
    /// DP no
    /// RPAP no
    /// rescue_flag no
    /// timer no
    /// powman no
    /// swcore yes
    /// psm yes
    /// then starts the power sequencer
    #[inline(always)]
    pub const fn had_swcore_pd(&self) -> bool {
        (self.bits & ChipReset::HAD_SWCORE_PD_MASK) != 0
    }

    /// Read **HAD_WATCHDOG_RESET_SWCORE** `[24:24]`
    ///
    /// Last reset was a watchdog timeout which was configured to reset the switched-core
    /// This resets:
    /// double_tap flag no
    /// DP no
    /// RPAP no
    /// rescue_flag no
    /// timer no
    /// powman no
    /// swcore yes
    /// psm yes
    /// then starts the power sequencer
    #[inline(always)]
    pub const fn had_watchdog_reset_swcore(&self) -> bool {
        (self.bits & ChipReset::HAD_WATCHDOG_RESET_SWCORE_MASK) != 0
    }

    /// Read **HAD_WATCHDOG_RESET_POWMAN** `[23:23]`
    ///
    /// Last reset was a watchdog timeout which was configured to reset the power manager
    /// This resets:
    /// double_tap flag no
    /// DP no
    /// RPAP no
    /// rescue_flag no
    /// timer yes
    /// powman yes
    /// swcore yes
    /// psm yes
    /// then starts the power sequencer
    #[inline(always)]
    pub const fn had_watchdog_reset_powman(&self) -> bool {
        (self.bits & ChipReset::HAD_WATCHDOG_RESET_POWMAN_MASK) != 0
    }

    /// Read **HAD_WATCHDOG_RESET_POWMAN_ASYNC** `[22:22]`
    ///
    /// Last reset was a watchdog timeout which was configured to reset the power manager asynchronously
    /// This resets:
    /// double_tap flag no
    /// DP no
    /// RPAP no
    /// rescue_flag no
    /// timer yes
    /// powman yes
    /// swcore yes
    /// psm yes
    /// then starts the power sequencer
    #[inline(always)]
    pub const fn had_watchdog_reset_powman_async(&self) -> bool {
        (self.bits & ChipReset::HAD_WATCHDOG_RESET_POWMAN_ASYNC_MASK) != 0
    }

    /// Read **HAD_RESCUE** `[21:21]`
    ///
    /// Last reset was a rescue reset from the debugger
    /// This resets:
    /// double_tap flag no
    /// DP no
    /// RPAP no
    /// rescue_flag no, it sets this flag
    /// timer yes
    /// powman yes
    /// swcore yes
    /// psm yes
    /// then starts the power sequencer
    #[inline(always)]
    pub const fn had_rescue(&self) -> bool {
        (self.bits & ChipReset::HAD_RESCUE_MASK) != 0
    }

    /// Read **HAD_DP_RESET_REQ** `[19:19]`
    ///
    /// Last reset was an reset request from the arm debugger
    /// This resets:
    /// double_tap flag no
    /// DP no
    /// RPAP no
    /// rescue_flag yes
    /// timer yes
    /// powman yes
    /// swcore yes
    /// psm yes
    /// then starts the power sequencer
    #[inline(always)]
    pub const fn had_dp_reset_req(&self) -> bool {
        (self.bits & ChipReset::HAD_DP_RESET_REQ_MASK) != 0
    }

    /// Read **HAD_RUN_LOW** `[18:18]`
    ///
    /// Last reset was from the RUN pin
    /// This resets:
    /// double_tap flag no
    /// DP yes
    /// RPAP yes
    /// rescue_flag yes
    /// timer yes
    /// powman yes
    /// swcore yes
    /// psm yes
    /// then starts the power sequencer
    #[inline(always)]
    pub const fn had_run_low(&self) -> bool {
        (self.bits & ChipReset::HAD_RUN_LOW_MASK) != 0
    }

    /// Read **HAD_BOR** `[17:17]`
    ///
    /// Last reset was from the brown-out detection block
    /// This resets:
    /// double_tap flag yes
    /// DP yes
    /// RPAP yes
    /// rescue_flag yes
    /// timer yes
    /// powman yes
    /// swcore yes
    /// psm yes
    /// then starts the power sequencer
    #[inline(always)]
    pub const fn had_bor(&self) -> bool {
        (self.bits & ChipReset::HAD_BOR_MASK) != 0
    }

    /// Read **HAD_POR** `[16:16]`
    ///
    /// Last reset was from the power-on reset
    /// This resets:
    /// double_tap flag yes
    /// DP yes
    /// RPAP yes
    /// rescue_flag yes
    /// timer yes
    /// powman yes
    /// swcore yes
    /// psm yes
    /// then starts the power sequencer
    #[inline(always)]
    pub const fn had_por(&self) -> bool {
        (self.bits & ChipReset::HAD_POR_MASK) != 0
    }

    /// Read **RESCUE_FLAG** `[4:4]`
    ///
    /// This is set by a rescue reset from the RP-AP.
    /// Its purpose is to halt before the bootrom before booting from flash in order to recover from a boot lock-up.
    /// The debugger can then attach once the bootrom has been halted and flash some working code that does not lock up.
    #[inline(always)]
    pub const fn rescue_flag(&self) -> bool {
        (self.bits & ChipReset::RESCUE_FLAG_MASK) != 0
    }

    /// Read **DOUBLE_TAP** `[0:0]`
    ///
    /// This flag is set by double-tapping RUN. It tells bootcode to go into the bootloader.
    #[inline(always)]
    pub const fn double_tap(&self) -> bool {
        (self.bits & ChipReset::DOUBLE_TAP_MASK) != 0
    }

    /// Set **RESCUE_FLAG** `[4:4]`
    ///
    /// This is set by a rescue reset from the RP-AP.
    /// Its purpose is to halt before the bootrom before booting from flash in order to recover from a boot lock-up.
    /// The debugger can then attach once the bootrom has been halted and flash some working code that does not lock up.
    #[inline(always)]
    pub const fn with_rescue_flag(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ChipReset::RESCUE_FLAG_MASK
            } else {
                self.bits & !ChipReset::RESCUE_FLAG_MASK
            }
        }
    }

    /// Set **DOUBLE_TAP** `[0:0]`
    ///
    /// This flag is set by double-tapping RUN. It tells bootcode to go into the bootloader.
    #[inline(always)]
    pub const fn with_double_tap(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ChipReset::DOUBLE_TAP_MASK
            } else {
                self.bits & !ChipReset::DOUBLE_TAP_MASK
            }
        }
    }

}

impl Default for ChipResetVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl ChipReset {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ChipResetVal {
        ChipResetVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ChipResetVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ChipResetVal) -> ChipResetVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// WDSEL Register
///
/// Allows a watchdog reset to reset the internal state of powman in addition to the power-on state machine (PSM).
/// Note that powman ignores watchdog resets that do not select at least the CLOCKS stage or earlier stages in the PSM. If using these bits, it's recommended to set PSM_WDSEL to all-ones in addition to the desired bits in this register. Failing to select CLOCKS or earlier will result in the POWMAN_WDSEL register having no effect.
///
/// Address offset: 0x0030
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **RESET_RSM** `[12]` (RW)
///
///   If set to 1, a watchdog reset will run the full power-on state machine (PSM) sequence
///   From a user perspective it is the same as setting RSM_WDSEL_PROC_COLD
///   From a hardware debug perspective it has the same effect as a reset from a glitch detector
///
/// - **RESET_SWCORE** `[8]` (RW)
///
///   If set to 1, a watchdog reset will reset the switched core power domain and run the full power-on state machine (PSM) sequence
///   From a user perspective it is the same as setting RSM_WDSEL_PROC_COLD
///   From a hardware debug perspective it has the same effect as a power-on reset for the switched core power domain
///
/// - **RESET_POWMAN** `[4]` (RW)
///
///   If set to 1, a watchdog reset will restore powman defaults, reset the timer, reset the switched core power domain
///   and run the full power-on state machine (PSM) sequence
///   This relies on clk_ref running. Use reset_powman_async if that may not be true
///
/// - **RESET_POWMAN_ASYNC** `[0]` (RW)
///
///   If set to 1, a watchdog reset will restore powman defaults, reset the timer,
///   reset the switched core domain and run the full power-on state machine (PSM) sequence
///   This does not rely on clk_ref running
#[repr(transparent)]
pub struct Wdsel {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Wdsel {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Wdsel {
    // ======== Field Constants ========

    /// **RESET_RSM**
    ///
    /// If set to 1, a watchdog reset will run the full power-on state machine (PSM) sequence
    /// From a user perspective it is the same as setting RSM_WDSEL_PROC_COLD
    /// From a hardware debug perspective it has the same effect as a reset from a glitch detector
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const RESET_RSM_OFFSET: u32 = 12;
    pub const RESET_RSM_MASK: u32 = 0x00001000;
    pub const RESET_RSM: u32 = 1 << 12;

    /// **RESET_SWCORE**
    ///
    /// If set to 1, a watchdog reset will reset the switched core power domain and run the full power-on state machine (PSM) sequence
    /// From a user perspective it is the same as setting RSM_WDSEL_PROC_COLD
    /// From a hardware debug perspective it has the same effect as a power-on reset for the switched core power domain
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const RESET_SWCORE_OFFSET: u32 = 8;
    pub const RESET_SWCORE_MASK: u32 = 0x00000100;
    pub const RESET_SWCORE: u32 = 1 << 8;

    /// **RESET_POWMAN**
    ///
    /// If set to 1, a watchdog reset will restore powman defaults, reset the timer, reset the switched core power domain
    /// and run the full power-on state machine (PSM) sequence
    /// This relies on clk_ref running. Use reset_powman_async if that may not be true
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const RESET_POWMAN_OFFSET: u32 = 4;
    pub const RESET_POWMAN_MASK: u32 = 0x00000010;
    pub const RESET_POWMAN: u32 = 1 << 4;

    /// **RESET_POWMAN_ASYNC**
    ///
    /// If set to 1, a watchdog reset will restore powman defaults, reset the timer,
    /// reset the switched core domain and run the full power-on state machine (PSM) sequence
    /// This does not rely on clk_ref running
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const RESET_POWMAN_ASYNC_OFFSET: u32 = 0;
    pub const RESET_POWMAN_ASYNC_MASK: u32 = 0x00000001;
    pub const RESET_POWMAN_ASYNC: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **RESET_RSM** `[12]` (RW)
    ///
    /// If set to 1, a watchdog reset will run the full power-on state machine (PSM) sequence
    /// From a user perspective it is the same as setting RSM_WDSEL_PROC_COLD
    /// From a hardware debug perspective it has the same effect as a reset from a glitch detector
    #[inline(always)]
    pub fn reset_rsm(&self) -> bool {
        (self.value.read_bits() & Self::RESET_RSM_MASK) != 0
    }

    /// Set RESET_RSM bit atomically (RW)
    #[inline(always)]
    pub fn set_reset_rsm(&self) {
        self.set_bits(Self::RESET_RSM_MASK);
    }

    /// Clear RESET_RSM bit atomically (RW)
    #[inline(always)]
    pub fn clr_reset_rsm(&self) {
        self.clr_bits(Self::RESET_RSM_MASK);
    }

    /// **RESET_SWCORE** `[8]` (RW)
    ///
    /// If set to 1, a watchdog reset will reset the switched core power domain and run the full power-on state machine (PSM) sequence
    /// From a user perspective it is the same as setting RSM_WDSEL_PROC_COLD
    /// From a hardware debug perspective it has the same effect as a power-on reset for the switched core power domain
    #[inline(always)]
    pub fn reset_swcore(&self) -> bool {
        (self.value.read_bits() & Self::RESET_SWCORE_MASK) != 0
    }

    /// Set RESET_SWCORE bit atomically (RW)
    #[inline(always)]
    pub fn set_reset_swcore(&self) {
        self.set_bits(Self::RESET_SWCORE_MASK);
    }

    /// Clear RESET_SWCORE bit atomically (RW)
    #[inline(always)]
    pub fn clr_reset_swcore(&self) {
        self.clr_bits(Self::RESET_SWCORE_MASK);
    }

    /// **RESET_POWMAN** `[4]` (RW)
    ///
    /// If set to 1, a watchdog reset will restore powman defaults, reset the timer, reset the switched core power domain
    /// and run the full power-on state machine (PSM) sequence
    /// This relies on clk_ref running. Use reset_powman_async if that may not be true
    #[inline(always)]
    pub fn reset_powman(&self) -> bool {
        (self.value.read_bits() & Self::RESET_POWMAN_MASK) != 0
    }

    /// Set RESET_POWMAN bit atomically (RW)
    #[inline(always)]
    pub fn set_reset_powman(&self) {
        self.set_bits(Self::RESET_POWMAN_MASK);
    }

    /// Clear RESET_POWMAN bit atomically (RW)
    #[inline(always)]
    pub fn clr_reset_powman(&self) {
        self.clr_bits(Self::RESET_POWMAN_MASK);
    }

    /// **RESET_POWMAN_ASYNC** `[0]` (RW)
    ///
    /// If set to 1, a watchdog reset will restore powman defaults, reset the timer,
    /// reset the switched core domain and run the full power-on state machine (PSM) sequence
    /// This does not rely on clk_ref running
    #[inline(always)]
    pub fn reset_powman_async(&self) -> bool {
        (self.value.read_bits() & Self::RESET_POWMAN_ASYNC_MASK) != 0
    }

    /// Set RESET_POWMAN_ASYNC bit atomically (RW)
    #[inline(always)]
    pub fn set_reset_powman_async(&self) {
        self.set_bits(Self::RESET_POWMAN_ASYNC_MASK);
    }

    /// Clear RESET_POWMAN_ASYNC bit atomically (RW)
    #[inline(always)]
    pub fn clr_reset_powman_async(&self) {
        self.clr_bits(Self::RESET_POWMAN_ASYNC_MASK);
    }

}

/// WDSEL value type for batch read/modify/write operations
///
/// Use `wdsel.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `wdsel.write_val(val)`.
#[derive(Clone, Copy)]
pub struct WdselVal {
    bits: u32,
}

impl WdselVal {
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

    /// Read **RESET_RSM** `[12:12]`
    ///
    /// If set to 1, a watchdog reset will run the full power-on state machine (PSM) sequence
    /// From a user perspective it is the same as setting RSM_WDSEL_PROC_COLD
    /// From a hardware debug perspective it has the same effect as a reset from a glitch detector
    #[inline(always)]
    pub const fn reset_rsm(&self) -> bool {
        (self.bits & Wdsel::RESET_RSM_MASK) != 0
    }

    /// Read **RESET_SWCORE** `[8:8]`
    ///
    /// If set to 1, a watchdog reset will reset the switched core power domain and run the full power-on state machine (PSM) sequence
    /// From a user perspective it is the same as setting RSM_WDSEL_PROC_COLD
    /// From a hardware debug perspective it has the same effect as a power-on reset for the switched core power domain
    #[inline(always)]
    pub const fn reset_swcore(&self) -> bool {
        (self.bits & Wdsel::RESET_SWCORE_MASK) != 0
    }

    /// Read **RESET_POWMAN** `[4:4]`
    ///
    /// If set to 1, a watchdog reset will restore powman defaults, reset the timer, reset the switched core power domain
    /// and run the full power-on state machine (PSM) sequence
    /// This relies on clk_ref running. Use reset_powman_async if that may not be true
    #[inline(always)]
    pub const fn reset_powman(&self) -> bool {
        (self.bits & Wdsel::RESET_POWMAN_MASK) != 0
    }

    /// Read **RESET_POWMAN_ASYNC** `[0:0]`
    ///
    /// If set to 1, a watchdog reset will restore powman defaults, reset the timer,
    /// reset the switched core domain and run the full power-on state machine (PSM) sequence
    /// This does not rely on clk_ref running
    #[inline(always)]
    pub const fn reset_powman_async(&self) -> bool {
        (self.bits & Wdsel::RESET_POWMAN_ASYNC_MASK) != 0
    }

    /// Set **RESET_RSM** `[12:12]`
    ///
    /// If set to 1, a watchdog reset will run the full power-on state machine (PSM) sequence
    /// From a user perspective it is the same as setting RSM_WDSEL_PROC_COLD
    /// From a hardware debug perspective it has the same effect as a reset from a glitch detector
    #[inline(always)]
    pub const fn with_reset_rsm(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Wdsel::RESET_RSM_MASK
            } else {
                self.bits & !Wdsel::RESET_RSM_MASK
            }
        }
    }

    /// Set **RESET_SWCORE** `[8:8]`
    ///
    /// If set to 1, a watchdog reset will reset the switched core power domain and run the full power-on state machine (PSM) sequence
    /// From a user perspective it is the same as setting RSM_WDSEL_PROC_COLD
    /// From a hardware debug perspective it has the same effect as a power-on reset for the switched core power domain
    #[inline(always)]
    pub const fn with_reset_swcore(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Wdsel::RESET_SWCORE_MASK
            } else {
                self.bits & !Wdsel::RESET_SWCORE_MASK
            }
        }
    }

    /// Set **RESET_POWMAN** `[4:4]`
    ///
    /// If set to 1, a watchdog reset will restore powman defaults, reset the timer, reset the switched core power domain
    /// and run the full power-on state machine (PSM) sequence
    /// This relies on clk_ref running. Use reset_powman_async if that may not be true
    #[inline(always)]
    pub const fn with_reset_powman(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Wdsel::RESET_POWMAN_MASK
            } else {
                self.bits & !Wdsel::RESET_POWMAN_MASK
            }
        }
    }

    /// Set **RESET_POWMAN_ASYNC** `[0:0]`
    ///
    /// If set to 1, a watchdog reset will restore powman defaults, reset the timer,
    /// reset the switched core domain and run the full power-on state machine (PSM) sequence
    /// This does not rely on clk_ref running
    #[inline(always)]
    pub const fn with_reset_powman_async(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Wdsel::RESET_POWMAN_ASYNC_MASK
            } else {
                self.bits & !Wdsel::RESET_POWMAN_ASYNC_MASK
            }
        }
    }

}

impl Default for WdselVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Wdsel {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> WdselVal {
        WdselVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: WdselVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(WdselVal) -> WdselVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SEQ_CFG Register
///
/// For configuration of the power sequencer
/// Writes are ignored while POWMAN_STATE_CHANGING=1
///
/// Address offset: 0x0034
/// Reset value: 0x001011f0
///
/// ## Fields
///
/// - **USING_FAST_POWCK** `[20]` (RO)
///
///   0 indicates the POWMAN clock is running from the low power oscillator (32kHz)
///   1 indicates the POWMAN clock is running from the reference clock (2-50MHz)
///
/// - **USING_BOD_LP** `[17]` (RO)
///
///   Indicates the brown-out detector (BOD) mode
///   0 = BOD high power mode which is the default
///   1 = BOD low power mode
///
/// - **USING_VREG_LP** `[16]` (RO)
///
///   Indicates the voltage regulator (VREG) mode
///   0 = VREG high power mode which is the default
///   1 = VREG low power mode
///
/// - **USE_FAST_POWCK** `[12]` (RW)
///
///   selects the reference clock (clk_ref) as the source of the POWMAN clock when switched-core is powered. The POWMAN clock always switches to the slow clock (lposc) when switched-core is powered down because the fast clock stops running.
///   0 always run the POWMAN clock from the slow clock (lposc)
///   1 run the POWMAN clock from the fast clock when available
///   This setting takes effect when a power up sequence is next run
///
/// - **RUN_LPOSC_IN_LP** `[8]` (RW)
///
///   Set to 0 to stop the low power osc when the switched-core is powered down, which is unwise if using it to clock the timer
///   This setting takes effect when the swcore is next powered down
///
/// - **USE_BOD_HP** `[7]` (RW)
///
///   Set to 0 to prevent automatic switching to bod high power mode when switched-core is powered up
///   This setting takes effect when the swcore is next powered up
///
/// - **USE_BOD_LP** `[6]` (RW)
///
///   Set to 0 to prevent automatic switching to bod low power mode when switched-core is powered down
///   This setting takes effect when the swcore is next powered down
///
/// - **USE_VREG_HP** `[5]` (RW)
///
///   Set to 0 to prevent automatic switching to vreg high power mode when switched-core is powered up
///   This setting takes effect when the swcore is next powered up
///
/// - **USE_VREG_LP** `[4]` (RW)
///
///   Set to 0 to prevent automatic switching to vreg low power mode when switched-core is powered down
///   This setting takes effect when the swcore is next powered down
///
/// - **HW_PWRUP_SRAM0** `[1]` (RW)
///
///   Specifies the power state of SRAM0 when powering up swcore from a low power state (P1.xxx) to a high power state (P0.0xx).
///   0=power-up
///   1=no change
///
/// - **HW_PWRUP_SRAM1** `[0]` (RW)
///
///   Specifies the power state of SRAM1 when powering up swcore from a low power state (P1.xxx) to a high power state (P0.0xx).
///   0=power-up
///   1=no change
#[repr(transparent)]
pub struct SeqCfg {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for SeqCfg {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SeqCfg {
    // ======== Field Constants ========

    /// **USING_FAST_POWCK**
    ///
    /// 0 indicates the POWMAN clock is running from the low power oscillator (32kHz)
    /// 1 indicates the POWMAN clock is running from the reference clock (2-50MHz)
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const USING_FAST_POWCK_OFFSET: u32 = 20;
    pub const USING_FAST_POWCK_MASK: u32 = 0x00100000;
    pub const USING_FAST_POWCK: u32 = 1 << 20;

    /// **USING_BOD_LP**
    ///
    /// Indicates the brown-out detector (BOD) mode
    /// 0 = BOD high power mode which is the default
    /// 1 = BOD low power mode
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const USING_BOD_LP_OFFSET: u32 = 17;
    pub const USING_BOD_LP_MASK: u32 = 0x00020000;
    pub const USING_BOD_LP: u32 = 1 << 17;

    /// **USING_VREG_LP**
    ///
    /// Indicates the voltage regulator (VREG) mode
    /// 0 = VREG high power mode which is the default
    /// 1 = VREG low power mode
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const USING_VREG_LP_OFFSET: u32 = 16;
    pub const USING_VREG_LP_MASK: u32 = 0x00010000;
    pub const USING_VREG_LP: u32 = 1 << 16;

    /// **USE_FAST_POWCK**
    ///
    /// selects the reference clock (clk_ref) as the source of the POWMAN clock when switched-core is powered. The POWMAN clock always switches to the slow clock (lposc) when switched-core is powered down because the fast clock stops running.
    /// 0 always run the POWMAN clock from the slow clock (lposc)
    /// 1 run the POWMAN clock from the fast clock when available
    /// This setting takes effect when a power up sequence is next run
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const USE_FAST_POWCK_OFFSET: u32 = 12;
    pub const USE_FAST_POWCK_MASK: u32 = 0x00001000;
    pub const USE_FAST_POWCK: u32 = 1 << 12;

    /// **RUN_LPOSC_IN_LP**
    ///
    /// Set to 0 to stop the low power osc when the switched-core is powered down, which is unwise if using it to clock the timer
    /// This setting takes effect when the swcore is next powered down
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const RUN_LPOSC_IN_LP_OFFSET: u32 = 8;
    pub const RUN_LPOSC_IN_LP_MASK: u32 = 0x00000100;
    pub const RUN_LPOSC_IN_LP: u32 = 1 << 8;

    /// **USE_BOD_HP**
    ///
    /// Set to 0 to prevent automatic switching to bod high power mode when switched-core is powered up
    /// This setting takes effect when the swcore is next powered up
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const USE_BOD_HP_OFFSET: u32 = 7;
    pub const USE_BOD_HP_MASK: u32 = 0x00000080;
    pub const USE_BOD_HP: u32 = 1 << 7;

    /// **USE_BOD_LP**
    ///
    /// Set to 0 to prevent automatic switching to bod low power mode when switched-core is powered down
    /// This setting takes effect when the swcore is next powered down
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const USE_BOD_LP_OFFSET: u32 = 6;
    pub const USE_BOD_LP_MASK: u32 = 0x00000040;
    pub const USE_BOD_LP: u32 = 1 << 6;

    /// **USE_VREG_HP**
    ///
    /// Set to 0 to prevent automatic switching to vreg high power mode when switched-core is powered up
    /// This setting takes effect when the swcore is next powered up
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const USE_VREG_HP_OFFSET: u32 = 5;
    pub const USE_VREG_HP_MASK: u32 = 0x00000020;
    pub const USE_VREG_HP: u32 = 1 << 5;

    /// **USE_VREG_LP**
    ///
    /// Set to 0 to prevent automatic switching to vreg low power mode when switched-core is powered down
    /// This setting takes effect when the swcore is next powered down
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const USE_VREG_LP_OFFSET: u32 = 4;
    pub const USE_VREG_LP_MASK: u32 = 0x00000010;
    pub const USE_VREG_LP: u32 = 1 << 4;

    /// **HW_PWRUP_SRAM0**
    ///
    /// Specifies the power state of SRAM0 when powering up swcore from a low power state (P1.xxx) to a high power state (P0.0xx).
    /// 0=power-up
    /// 1=no change
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const HW_PWRUP_SRAM0_OFFSET: u32 = 1;
    pub const HW_PWRUP_SRAM0_MASK: u32 = 0x00000002;
    pub const HW_PWRUP_SRAM0: u32 = 1 << 1;

    /// **HW_PWRUP_SRAM1**
    ///
    /// Specifies the power state of SRAM1 when powering up swcore from a low power state (P1.xxx) to a high power state (P0.0xx).
    /// 0=power-up
    /// 1=no change
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const HW_PWRUP_SRAM1_OFFSET: u32 = 0;
    pub const HW_PWRUP_SRAM1_MASK: u32 = 0x00000001;
    pub const HW_PWRUP_SRAM1: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **USING_FAST_POWCK** `[20]` (RO)
    ///
    /// 0 indicates the POWMAN clock is running from the low power oscillator (32kHz)
    /// 1 indicates the POWMAN clock is running from the reference clock (2-50MHz)
    #[inline(always)]
    pub fn using_fast_powck(&self) -> bool {
        (self.value.read_bits() & Self::USING_FAST_POWCK_MASK) != 0
    }

    /// **USING_BOD_LP** `[17]` (RO)
    ///
    /// Indicates the brown-out detector (BOD) mode
    /// 0 = BOD high power mode which is the default
    /// 1 = BOD low power mode
    #[inline(always)]
    pub fn using_bod_lp(&self) -> bool {
        (self.value.read_bits() & Self::USING_BOD_LP_MASK) != 0
    }

    /// **USING_VREG_LP** `[16]` (RO)
    ///
    /// Indicates the voltage regulator (VREG) mode
    /// 0 = VREG high power mode which is the default
    /// 1 = VREG low power mode
    #[inline(always)]
    pub fn using_vreg_lp(&self) -> bool {
        (self.value.read_bits() & Self::USING_VREG_LP_MASK) != 0
    }

    /// **USE_FAST_POWCK** `[12]` (RW)
    ///
    /// selects the reference clock (clk_ref) as the source of the POWMAN clock when switched-core is powered. The POWMAN clock always switches to the slow clock (lposc) when switched-core is powered down because the fast clock stops running.
    /// 0 always run the POWMAN clock from the slow clock (lposc)
    /// 1 run the POWMAN clock from the fast clock when available
    /// This setting takes effect when a power up sequence is next run
    #[inline(always)]
    pub fn use_fast_powck(&self) -> bool {
        (self.value.read_bits() & Self::USE_FAST_POWCK_MASK) != 0
    }

    /// Set USE_FAST_POWCK bit atomically (RW)
    #[inline(always)]
    pub fn set_use_fast_powck(&self) {
        self.set_bits(Self::USE_FAST_POWCK_MASK);
    }

    /// Clear USE_FAST_POWCK bit atomically (RW)
    #[inline(always)]
    pub fn clr_use_fast_powck(&self) {
        self.clr_bits(Self::USE_FAST_POWCK_MASK);
    }

    /// **RUN_LPOSC_IN_LP** `[8]` (RW)
    ///
    /// Set to 0 to stop the low power osc when the switched-core is powered down, which is unwise if using it to clock the timer
    /// This setting takes effect when the swcore is next powered down
    #[inline(always)]
    pub fn run_lposc_in_lp(&self) -> bool {
        (self.value.read_bits() & Self::RUN_LPOSC_IN_LP_MASK) != 0
    }

    /// Set RUN_LPOSC_IN_LP bit atomically (RW)
    #[inline(always)]
    pub fn set_run_lposc_in_lp(&self) {
        self.set_bits(Self::RUN_LPOSC_IN_LP_MASK);
    }

    /// Clear RUN_LPOSC_IN_LP bit atomically (RW)
    #[inline(always)]
    pub fn clr_run_lposc_in_lp(&self) {
        self.clr_bits(Self::RUN_LPOSC_IN_LP_MASK);
    }

    /// **USE_BOD_HP** `[7]` (RW)
    ///
    /// Set to 0 to prevent automatic switching to bod high power mode when switched-core is powered up
    /// This setting takes effect when the swcore is next powered up
    #[inline(always)]
    pub fn use_bod_hp(&self) -> bool {
        (self.value.read_bits() & Self::USE_BOD_HP_MASK) != 0
    }

    /// Set USE_BOD_HP bit atomically (RW)
    #[inline(always)]
    pub fn set_use_bod_hp(&self) {
        self.set_bits(Self::USE_BOD_HP_MASK);
    }

    /// Clear USE_BOD_HP bit atomically (RW)
    #[inline(always)]
    pub fn clr_use_bod_hp(&self) {
        self.clr_bits(Self::USE_BOD_HP_MASK);
    }

    /// **USE_BOD_LP** `[6]` (RW)
    ///
    /// Set to 0 to prevent automatic switching to bod low power mode when switched-core is powered down
    /// This setting takes effect when the swcore is next powered down
    #[inline(always)]
    pub fn use_bod_lp(&self) -> bool {
        (self.value.read_bits() & Self::USE_BOD_LP_MASK) != 0
    }

    /// Set USE_BOD_LP bit atomically (RW)
    #[inline(always)]
    pub fn set_use_bod_lp(&self) {
        self.set_bits(Self::USE_BOD_LP_MASK);
    }

    /// Clear USE_BOD_LP bit atomically (RW)
    #[inline(always)]
    pub fn clr_use_bod_lp(&self) {
        self.clr_bits(Self::USE_BOD_LP_MASK);
    }

    /// **USE_VREG_HP** `[5]` (RW)
    ///
    /// Set to 0 to prevent automatic switching to vreg high power mode when switched-core is powered up
    /// This setting takes effect when the swcore is next powered up
    #[inline(always)]
    pub fn use_vreg_hp(&self) -> bool {
        (self.value.read_bits() & Self::USE_VREG_HP_MASK) != 0
    }

    /// Set USE_VREG_HP bit atomically (RW)
    #[inline(always)]
    pub fn set_use_vreg_hp(&self) {
        self.set_bits(Self::USE_VREG_HP_MASK);
    }

    /// Clear USE_VREG_HP bit atomically (RW)
    #[inline(always)]
    pub fn clr_use_vreg_hp(&self) {
        self.clr_bits(Self::USE_VREG_HP_MASK);
    }

    /// **USE_VREG_LP** `[4]` (RW)
    ///
    /// Set to 0 to prevent automatic switching to vreg low power mode when switched-core is powered down
    /// This setting takes effect when the swcore is next powered down
    #[inline(always)]
    pub fn use_vreg_lp(&self) -> bool {
        (self.value.read_bits() & Self::USE_VREG_LP_MASK) != 0
    }

    /// Set USE_VREG_LP bit atomically (RW)
    #[inline(always)]
    pub fn set_use_vreg_lp(&self) {
        self.set_bits(Self::USE_VREG_LP_MASK);
    }

    /// Clear USE_VREG_LP bit atomically (RW)
    #[inline(always)]
    pub fn clr_use_vreg_lp(&self) {
        self.clr_bits(Self::USE_VREG_LP_MASK);
    }

    /// **HW_PWRUP_SRAM0** `[1]` (RW)
    ///
    /// Specifies the power state of SRAM0 when powering up swcore from a low power state (P1.xxx) to a high power state (P0.0xx).
    /// 0=power-up
    /// 1=no change
    #[inline(always)]
    pub fn hw_pwrup_sram0(&self) -> bool {
        (self.value.read_bits() & Self::HW_PWRUP_SRAM0_MASK) != 0
    }

    /// Set HW_PWRUP_SRAM0 bit atomically (RW)
    #[inline(always)]
    pub fn set_hw_pwrup_sram0(&self) {
        self.set_bits(Self::HW_PWRUP_SRAM0_MASK);
    }

    /// Clear HW_PWRUP_SRAM0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_hw_pwrup_sram0(&self) {
        self.clr_bits(Self::HW_PWRUP_SRAM0_MASK);
    }

    /// **HW_PWRUP_SRAM1** `[0]` (RW)
    ///
    /// Specifies the power state of SRAM1 when powering up swcore from a low power state (P1.xxx) to a high power state (P0.0xx).
    /// 0=power-up
    /// 1=no change
    #[inline(always)]
    pub fn hw_pwrup_sram1(&self) -> bool {
        (self.value.read_bits() & Self::HW_PWRUP_SRAM1_MASK) != 0
    }

    /// Set HW_PWRUP_SRAM1 bit atomically (RW)
    #[inline(always)]
    pub fn set_hw_pwrup_sram1(&self) {
        self.set_bits(Self::HW_PWRUP_SRAM1_MASK);
    }

    /// Clear HW_PWRUP_SRAM1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_hw_pwrup_sram1(&self) {
        self.clr_bits(Self::HW_PWRUP_SRAM1_MASK);
    }

}

/// SEQ_CFG value type for batch read/modify/write operations
///
/// Use `seqcfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `seqcfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SeqCfgVal {
    bits: u32,
}

impl SeqCfgVal {
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
        Self { bits: 0x001011f0 }
    }

    /// Read **USING_FAST_POWCK** `[20:20]`
    ///
    /// 0 indicates the POWMAN clock is running from the low power oscillator (32kHz)
    /// 1 indicates the POWMAN clock is running from the reference clock (2-50MHz)
    #[inline(always)]
    pub const fn using_fast_powck(&self) -> bool {
        (self.bits & SeqCfg::USING_FAST_POWCK_MASK) != 0
    }

    /// Read **USING_BOD_LP** `[17:17]`
    ///
    /// Indicates the brown-out detector (BOD) mode
    /// 0 = BOD high power mode which is the default
    /// 1 = BOD low power mode
    #[inline(always)]
    pub const fn using_bod_lp(&self) -> bool {
        (self.bits & SeqCfg::USING_BOD_LP_MASK) != 0
    }

    /// Read **USING_VREG_LP** `[16:16]`
    ///
    /// Indicates the voltage regulator (VREG) mode
    /// 0 = VREG high power mode which is the default
    /// 1 = VREG low power mode
    #[inline(always)]
    pub const fn using_vreg_lp(&self) -> bool {
        (self.bits & SeqCfg::USING_VREG_LP_MASK) != 0
    }

    /// Read **USE_FAST_POWCK** `[12:12]`
    ///
    /// selects the reference clock (clk_ref) as the source of the POWMAN clock when switched-core is powered. The POWMAN clock always switches to the slow clock (lposc) when switched-core is powered down because the fast clock stops running.
    /// 0 always run the POWMAN clock from the slow clock (lposc)
    /// 1 run the POWMAN clock from the fast clock when available
    /// This setting takes effect when a power up sequence is next run
    #[inline(always)]
    pub const fn use_fast_powck(&self) -> bool {
        (self.bits & SeqCfg::USE_FAST_POWCK_MASK) != 0
    }

    /// Read **RUN_LPOSC_IN_LP** `[8:8]`
    ///
    /// Set to 0 to stop the low power osc when the switched-core is powered down, which is unwise if using it to clock the timer
    /// This setting takes effect when the swcore is next powered down
    #[inline(always)]
    pub const fn run_lposc_in_lp(&self) -> bool {
        (self.bits & SeqCfg::RUN_LPOSC_IN_LP_MASK) != 0
    }

    /// Read **USE_BOD_HP** `[7:7]`
    ///
    /// Set to 0 to prevent automatic switching to bod high power mode when switched-core is powered up
    /// This setting takes effect when the swcore is next powered up
    #[inline(always)]
    pub const fn use_bod_hp(&self) -> bool {
        (self.bits & SeqCfg::USE_BOD_HP_MASK) != 0
    }

    /// Read **USE_BOD_LP** `[6:6]`
    ///
    /// Set to 0 to prevent automatic switching to bod low power mode when switched-core is powered down
    /// This setting takes effect when the swcore is next powered down
    #[inline(always)]
    pub const fn use_bod_lp(&self) -> bool {
        (self.bits & SeqCfg::USE_BOD_LP_MASK) != 0
    }

    /// Read **USE_VREG_HP** `[5:5]`
    ///
    /// Set to 0 to prevent automatic switching to vreg high power mode when switched-core is powered up
    /// This setting takes effect when the swcore is next powered up
    #[inline(always)]
    pub const fn use_vreg_hp(&self) -> bool {
        (self.bits & SeqCfg::USE_VREG_HP_MASK) != 0
    }

    /// Read **USE_VREG_LP** `[4:4]`
    ///
    /// Set to 0 to prevent automatic switching to vreg low power mode when switched-core is powered down
    /// This setting takes effect when the swcore is next powered down
    #[inline(always)]
    pub const fn use_vreg_lp(&self) -> bool {
        (self.bits & SeqCfg::USE_VREG_LP_MASK) != 0
    }

    /// Read **HW_PWRUP_SRAM0** `[1:1]`
    ///
    /// Specifies the power state of SRAM0 when powering up swcore from a low power state (P1.xxx) to a high power state (P0.0xx).
    /// 0=power-up
    /// 1=no change
    #[inline(always)]
    pub const fn hw_pwrup_sram0(&self) -> bool {
        (self.bits & SeqCfg::HW_PWRUP_SRAM0_MASK) != 0
    }

    /// Read **HW_PWRUP_SRAM1** `[0:0]`
    ///
    /// Specifies the power state of SRAM1 when powering up swcore from a low power state (P1.xxx) to a high power state (P0.0xx).
    /// 0=power-up
    /// 1=no change
    #[inline(always)]
    pub const fn hw_pwrup_sram1(&self) -> bool {
        (self.bits & SeqCfg::HW_PWRUP_SRAM1_MASK) != 0
    }

    /// Set **USE_FAST_POWCK** `[12:12]`
    ///
    /// selects the reference clock (clk_ref) as the source of the POWMAN clock when switched-core is powered. The POWMAN clock always switches to the slow clock (lposc) when switched-core is powered down because the fast clock stops running.
    /// 0 always run the POWMAN clock from the slow clock (lposc)
    /// 1 run the POWMAN clock from the fast clock when available
    /// This setting takes effect when a power up sequence is next run
    #[inline(always)]
    pub const fn with_use_fast_powck(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SeqCfg::USE_FAST_POWCK_MASK
            } else {
                self.bits & !SeqCfg::USE_FAST_POWCK_MASK
            }
        }
    }

    /// Set **RUN_LPOSC_IN_LP** `[8:8]`
    ///
    /// Set to 0 to stop the low power osc when the switched-core is powered down, which is unwise if using it to clock the timer
    /// This setting takes effect when the swcore is next powered down
    #[inline(always)]
    pub const fn with_run_lposc_in_lp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SeqCfg::RUN_LPOSC_IN_LP_MASK
            } else {
                self.bits & !SeqCfg::RUN_LPOSC_IN_LP_MASK
            }
        }
    }

    /// Set **USE_BOD_HP** `[7:7]`
    ///
    /// Set to 0 to prevent automatic switching to bod high power mode when switched-core is powered up
    /// This setting takes effect when the swcore is next powered up
    #[inline(always)]
    pub const fn with_use_bod_hp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SeqCfg::USE_BOD_HP_MASK
            } else {
                self.bits & !SeqCfg::USE_BOD_HP_MASK
            }
        }
    }

    /// Set **USE_BOD_LP** `[6:6]`
    ///
    /// Set to 0 to prevent automatic switching to bod low power mode when switched-core is powered down
    /// This setting takes effect when the swcore is next powered down
    #[inline(always)]
    pub const fn with_use_bod_lp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SeqCfg::USE_BOD_LP_MASK
            } else {
                self.bits & !SeqCfg::USE_BOD_LP_MASK
            }
        }
    }

    /// Set **USE_VREG_HP** `[5:5]`
    ///
    /// Set to 0 to prevent automatic switching to vreg high power mode when switched-core is powered up
    /// This setting takes effect when the swcore is next powered up
    #[inline(always)]
    pub const fn with_use_vreg_hp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SeqCfg::USE_VREG_HP_MASK
            } else {
                self.bits & !SeqCfg::USE_VREG_HP_MASK
            }
        }
    }

    /// Set **USE_VREG_LP** `[4:4]`
    ///
    /// Set to 0 to prevent automatic switching to vreg low power mode when switched-core is powered down
    /// This setting takes effect when the swcore is next powered down
    #[inline(always)]
    pub const fn with_use_vreg_lp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SeqCfg::USE_VREG_LP_MASK
            } else {
                self.bits & !SeqCfg::USE_VREG_LP_MASK
            }
        }
    }

    /// Set **HW_PWRUP_SRAM0** `[1:1]`
    ///
    /// Specifies the power state of SRAM0 when powering up swcore from a low power state (P1.xxx) to a high power state (P0.0xx).
    /// 0=power-up
    /// 1=no change
    #[inline(always)]
    pub const fn with_hw_pwrup_sram0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SeqCfg::HW_PWRUP_SRAM0_MASK
            } else {
                self.bits & !SeqCfg::HW_PWRUP_SRAM0_MASK
            }
        }
    }

    /// Set **HW_PWRUP_SRAM1** `[0:0]`
    ///
    /// Specifies the power state of SRAM1 when powering up swcore from a low power state (P1.xxx) to a high power state (P0.0xx).
    /// 0=power-up
    /// 1=no change
    #[inline(always)]
    pub const fn with_hw_pwrup_sram1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SeqCfg::HW_PWRUP_SRAM1_MASK
            } else {
                self.bits & !SeqCfg::HW_PWRUP_SRAM1_MASK
            }
        }
    }

}

impl Default for SeqCfgVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SeqCfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SeqCfgVal {
        SeqCfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SeqCfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SeqCfgVal) -> SeqCfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// STATE Register
///
/// This register controls the power state of the 4 power domains.
/// The current power state is indicated in POWMAN_STATE_CURRENT which is read-only.
/// To change the state, write to POWMAN_STATE_REQ.
/// The coding of POWMAN_STATE_CURRENT & POWMAN_STATE_REQ corresponds to the power states
/// defined in the datasheet:
/// bit 3 = SWCORE
/// bit 2 = XIP cache
/// bit 1 = SRAM0
/// bit 0 = SRAM1
/// 0 = powered up
/// 1 = powered down
/// When POWMAN_STATE_REQ is written, the POWMAN_STATE_WAITING flag is set while the Power Manager determines what is required. If an invalid transition is requested the Power Manager will still register the request in POWMAN_STATE_REQ but will also set the POWMAN_BAD_REQ flag. It will then implement the power-up requests and ignore the power down requests. To do nothing would risk entering an unrecoverable lock-up state. Invalid requests are: any combination of power up and power down requests any request that results in swcore being powered and xip unpowered If the request is to power down the switched-core domain then POWMAN_STATE_WAITING stays active until the processors halt. During this time the POWMAN_STATE_REQ field can be re-written to change or cancel the request. When the power state transition begins the POWMAN_STATE_WAITING_flag is cleared, the POWMAN_STATE_CHANGING flag is set and POWMAN register writes are ignored until the transition completes.
///
/// Address offset: 0x0038
/// Reset value: 0x0000000f
///
/// ## Fields
///
/// - **CHANGING** `[13]` (RO)
///
///   Indicates a power state change is in progress
///
/// - **WAITING** `[12]` (RO)
///
///   Indicates the power manager has received a state change request and is waiting for other actions to complete before executing it
///
/// - **BAD_HW_REQ** `[11]` (RO)
///
///   Invalid hardware initiated state request, power up requests actioned, power down requests ignored
///
/// - **BAD_SW_REQ** `[10]` (RO)
///
///   Invalid software initiated state request ignored
///
/// - **PWRUP_WHILE_WAITING** `[9]` (RW)
///
///   Indicates that a power state change request was ignored because of a pending power state change request
///
/// - **REQ_IGNORED** `[8]` (RW)
///
///   Indicates that a software state change request was ignored because it clashed with an ongoing hardware or debugger request
///
/// - **REQ** `[7:4]` (RW)
///
///   This is written by software or hardware to request a new power state
///
/// - **CURRENT** `[3:0]` (RO)
///
///   Indicates the current power state
#[repr(transparent)]
pub struct State {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for State {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl State {
    // ======== Field Constants ========

    /// **CHANGING**
    ///
    /// Indicates a power state change is in progress
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const CHANGING_OFFSET: u32 = 13;
    pub const CHANGING_MASK: u32 = 0x00002000;
    pub const CHANGING: u32 = 1 << 13;

    /// **WAITING**
    ///
    /// Indicates the power manager has received a state change request and is waiting for other actions to complete before executing it
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const WAITING_OFFSET: u32 = 12;
    pub const WAITING_MASK: u32 = 0x00001000;
    pub const WAITING: u32 = 1 << 12;

    /// **BAD_HW_REQ**
    ///
    /// Invalid hardware initiated state request, power up requests actioned, power down requests ignored
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const BAD_HW_REQ_OFFSET: u32 = 11;
    pub const BAD_HW_REQ_MASK: u32 = 0x00000800;
    pub const BAD_HW_REQ: u32 = 1 << 11;

    /// **BAD_SW_REQ**
    ///
    /// Invalid software initiated state request ignored
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const BAD_SW_REQ_OFFSET: u32 = 10;
    pub const BAD_SW_REQ_MASK: u32 = 0x00000400;
    pub const BAD_SW_REQ: u32 = 1 << 10;

    /// **PWRUP_WHILE_WAITING**
    ///
    /// Indicates that a power state change request was ignored because of a pending power state change request
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const PWRUP_WHILE_WAITING_OFFSET: u32 = 9;
    pub const PWRUP_WHILE_WAITING_MASK: u32 = 0x00000200;
    pub const PWRUP_WHILE_WAITING: u32 = 1 << 9;

    /// **REQ_IGNORED**
    ///
    /// Indicates that a software state change request was ignored because it clashed with an ongoing hardware or debugger request
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const REQ_IGNORED_OFFSET: u32 = 8;
    pub const REQ_IGNORED_MASK: u32 = 0x00000100;
    pub const REQ_IGNORED: u32 = 1 << 8;

    /// **REQ**
    ///
    /// This is written by software or hardware to request a new power state
    ///
    /// Bits `[7:4]`, mask `0x000000f0`
    pub const REQ_OFFSET: u32 = 4;
    pub const REQ_MASK: u32 = 0x000000f0;

    /// **CURRENT**
    ///
    /// Indicates the current power state
    ///
    /// Bits `[3:0]`, mask `0x0000000f`
    pub const CURRENT_OFFSET: u32 = 0;
    pub const CURRENT_MASK: u32 = 0x0000000f;

    // ======== Field Accessors ========

    /// **CHANGING** `[13]` (RO)
    ///
    /// Indicates a power state change is in progress
    #[inline(always)]
    pub fn changing(&self) -> bool {
        (self.value.read_bits() & Self::CHANGING_MASK) != 0
    }

    /// **WAITING** `[12]` (RO)
    ///
    /// Indicates the power manager has received a state change request and is waiting for other actions to complete before executing it
    #[inline(always)]
    pub fn waiting(&self) -> bool {
        (self.value.read_bits() & Self::WAITING_MASK) != 0
    }

    /// **BAD_HW_REQ** `[11]` (RO)
    ///
    /// Invalid hardware initiated state request, power up requests actioned, power down requests ignored
    #[inline(always)]
    pub fn bad_hw_req(&self) -> bool {
        (self.value.read_bits() & Self::BAD_HW_REQ_MASK) != 0
    }

    /// **BAD_SW_REQ** `[10]` (RO)
    ///
    /// Invalid software initiated state request ignored
    #[inline(always)]
    pub fn bad_sw_req(&self) -> bool {
        (self.value.read_bits() & Self::BAD_SW_REQ_MASK) != 0
    }

    /// **PWRUP_WHILE_WAITING** `[9]` (RW)
    ///
    /// Indicates that a power state change request was ignored because of a pending power state change request
    #[inline(always)]
    pub fn pwrup_while_waiting(&self) -> bool {
        (self.value.read_bits() & Self::PWRUP_WHILE_WAITING_MASK) != 0
    }

    /// Set PWRUP_WHILE_WAITING bit atomically (RW)
    #[inline(always)]
    pub fn set_pwrup_while_waiting(&self) {
        self.set_bits(Self::PWRUP_WHILE_WAITING_MASK);
    }

    /// Clear PWRUP_WHILE_WAITING bit atomically (RW)
    #[inline(always)]
    pub fn clr_pwrup_while_waiting(&self) {
        self.clr_bits(Self::PWRUP_WHILE_WAITING_MASK);
    }

    /// **REQ_IGNORED** `[8]` (RW)
    ///
    /// Indicates that a software state change request was ignored because it clashed with an ongoing hardware or debugger request
    #[inline(always)]
    pub fn req_ignored(&self) -> bool {
        (self.value.read_bits() & Self::REQ_IGNORED_MASK) != 0
    }

    /// Set REQ_IGNORED bit atomically (RW)
    #[inline(always)]
    pub fn set_req_ignored(&self) {
        self.set_bits(Self::REQ_IGNORED_MASK);
    }

    /// Clear REQ_IGNORED bit atomically (RW)
    #[inline(always)]
    pub fn clr_req_ignored(&self) {
        self.clr_bits(Self::REQ_IGNORED_MASK);
    }

    /// **REQ** `[7:4]` (RW)
    ///
    /// This is written by software or hardware to request a new power state
    #[inline(always)]
    pub fn req(&self) -> u32 {
        (self.value.read_bits() & Self::REQ_MASK) >> Self::REQ_OFFSET
    }

    /// Write **REQ** `[7:4]` (RW)
    ///
    /// This is written by software or hardware to request a new power state
    #[inline(always)]
    pub fn write_req(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::REQ_MASK) | ((value << Self::REQ_OFFSET) & Self::REQ_MASK));
    }

    /// **CURRENT** `[3:0]` (RO)
    ///
    /// Indicates the current power state
    #[inline(always)]
    pub fn current(&self) -> u32 {
        (self.value.read_bits() & Self::CURRENT_MASK) >> Self::CURRENT_OFFSET
    }

}

/// STATE value type for batch read/modify/write operations
///
/// Use `state.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `state.write_val(val)`.
#[derive(Clone, Copy)]
pub struct StateVal {
    bits: u32,
}

impl StateVal {
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
        Self { bits: 0x0000000f }
    }

    /// Read **CHANGING** `[13:13]`
    ///
    /// Indicates a power state change is in progress
    #[inline(always)]
    pub const fn changing(&self) -> bool {
        (self.bits & State::CHANGING_MASK) != 0
    }

    /// Read **WAITING** `[12:12]`
    ///
    /// Indicates the power manager has received a state change request and is waiting for other actions to complete before executing it
    #[inline(always)]
    pub const fn waiting(&self) -> bool {
        (self.bits & State::WAITING_MASK) != 0
    }

    /// Read **BAD_HW_REQ** `[11:11]`
    ///
    /// Invalid hardware initiated state request, power up requests actioned, power down requests ignored
    #[inline(always)]
    pub const fn bad_hw_req(&self) -> bool {
        (self.bits & State::BAD_HW_REQ_MASK) != 0
    }

    /// Read **BAD_SW_REQ** `[10:10]`
    ///
    /// Invalid software initiated state request ignored
    #[inline(always)]
    pub const fn bad_sw_req(&self) -> bool {
        (self.bits & State::BAD_SW_REQ_MASK) != 0
    }

    /// Read **PWRUP_WHILE_WAITING** `[9:9]`
    ///
    /// Indicates that a power state change request was ignored because of a pending power state change request
    #[inline(always)]
    pub const fn pwrup_while_waiting(&self) -> bool {
        (self.bits & State::PWRUP_WHILE_WAITING_MASK) != 0
    }

    /// Read **REQ_IGNORED** `[8:8]`
    ///
    /// Indicates that a software state change request was ignored because it clashed with an ongoing hardware or debugger request
    #[inline(always)]
    pub const fn req_ignored(&self) -> bool {
        (self.bits & State::REQ_IGNORED_MASK) != 0
    }

    /// Read **REQ** `[7:4]`
    ///
    /// This is written by software or hardware to request a new power state
    #[inline(always)]
    pub const fn req(&self) -> u32 {
        (self.bits & State::REQ_MASK) >> State::REQ_OFFSET
    }

    /// Read **CURRENT** `[3:0]`
    ///
    /// Indicates the current power state
    #[inline(always)]
    pub const fn current(&self) -> u32 {
        (self.bits & State::CURRENT_MASK) >> State::CURRENT_OFFSET
    }

    /// Set **PWRUP_WHILE_WAITING** `[9:9]`
    ///
    /// Indicates that a power state change request was ignored because of a pending power state change request
    #[inline(always)]
    pub const fn with_pwrup_while_waiting(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | State::PWRUP_WHILE_WAITING_MASK
            } else {
                self.bits & !State::PWRUP_WHILE_WAITING_MASK
            }
        }
    }

    /// Set **REQ_IGNORED** `[8:8]`
    ///
    /// Indicates that a software state change request was ignored because it clashed with an ongoing hardware or debugger request
    #[inline(always)]
    pub const fn with_req_ignored(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | State::REQ_IGNORED_MASK
            } else {
                self.bits & !State::REQ_IGNORED_MASK
            }
        }
    }

    /// Set **REQ** `[7:4]`
    ///
    /// This is written by software or hardware to request a new power state
    #[inline(always)]
    pub const fn with_req(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !State::REQ_MASK) | ((value << State::REQ_OFFSET) & State::REQ_MASK)
        }
    }

}

impl Default for StateVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl State {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> StateVal {
        StateVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: StateVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(StateVal) -> StateVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// POW_FASTDIV Register
///
/// Address offset: 0x003c
/// Reset value: 0x00000040
///
/// ## Fields
///
/// - **POW_FASTDIV** `[10:0]` (RW)
///
///   divides the POWMAN clock to provide a tick for the delay module and state machines
///   when clk_pow is running from the slow clock it is not divided
///   when clk_pow is running from the fast clock it is divided by tick_div
#[repr(transparent)]
pub struct PowFastdiv {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for PowFastdiv {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PowFastdiv {
    // ======== Field Constants ========

    /// **POW_FASTDIV**
    ///
    /// divides the POWMAN clock to provide a tick for the delay module and state machines
    /// when clk_pow is running from the slow clock it is not divided
    /// when clk_pow is running from the fast clock it is divided by tick_div
    ///
    /// Bits `[10:0]`, mask `0x000007ff`
    pub const POW_FASTDIV_OFFSET: u32 = 0;
    pub const POW_FASTDIV_MASK: u32 = 0x000007ff;

    // ======== Field Accessors ========

    /// **POW_FASTDIV** `[10:0]` (RW)
    ///
    /// divides the POWMAN clock to provide a tick for the delay module and state machines
    /// when clk_pow is running from the slow clock it is not divided
    /// when clk_pow is running from the fast clock it is divided by tick_div
    #[inline(always)]
    pub fn pow_fastdiv(&self) -> u32 {
        (self.value.read_bits() & Self::POW_FASTDIV_MASK) >> Self::POW_FASTDIV_OFFSET
    }

    /// Write **POW_FASTDIV** `[10:0]` (RW)
    ///
    /// divides the POWMAN clock to provide a tick for the delay module and state machines
    /// when clk_pow is running from the slow clock it is not divided
    /// when clk_pow is running from the fast clock it is divided by tick_div
    #[inline(always)]
    pub fn write_pow_fastdiv(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::POW_FASTDIV_MASK) | ((value << Self::POW_FASTDIV_OFFSET) & Self::POW_FASTDIV_MASK));
    }

}

/// POW_FASTDIV value type for batch read/modify/write operations
///
/// Use `powfastdiv.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `powfastdiv.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PowFastdivVal {
    bits: u32,
}

impl PowFastdivVal {
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
        Self { bits: 0x00000040 }
    }

    /// Read **POW_FASTDIV** `[10:0]`
    ///
    /// divides the POWMAN clock to provide a tick for the delay module and state machines
    /// when clk_pow is running from the slow clock it is not divided
    /// when clk_pow is running from the fast clock it is divided by tick_div
    #[inline(always)]
    pub const fn pow_fastdiv(&self) -> u32 {
        (self.bits & PowFastdiv::POW_FASTDIV_MASK) >> PowFastdiv::POW_FASTDIV_OFFSET
    }

    /// Set **POW_FASTDIV** `[10:0]`
    ///
    /// divides the POWMAN clock to provide a tick for the delay module and state machines
    /// when clk_pow is running from the slow clock it is not divided
    /// when clk_pow is running from the fast clock it is divided by tick_div
    #[inline(always)]
    pub const fn with_pow_fastdiv(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !PowFastdiv::POW_FASTDIV_MASK) | ((value << PowFastdiv::POW_FASTDIV_OFFSET) & PowFastdiv::POW_FASTDIV_MASK)
        }
    }

}

impl Default for PowFastdivVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl PowFastdiv {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PowFastdivVal {
        PowFastdivVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: PowFastdivVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(PowFastdivVal) -> PowFastdivVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// POW_DELAY Register
///
/// power state machine delays
///
/// Address offset: 0x0040
/// Reset value: 0x00002011
///
/// ## Fields
///
/// - **SRAM_STEP** `[15:8]` (RW)
///
///   timing between the sram0 and sram1 power state machine steps
///   measured in units of the powman tick period (>=1us), 0 gives a delay of 1 unit
///
/// - **XIP_STEP** `[7:4]` (RW)
///
///   timing between the xip power state machine steps
///   measured in units of the lposc period, 0 gives a delay of 1 unit
///
/// - **SWCORE_STEP** `[3:0]` (RW)
///
///   timing between the swcore power state machine steps
///   measured in units of the lposc period, 0 gives a delay of 1 unit
#[repr(transparent)]
pub struct PowDelay {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for PowDelay {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PowDelay {
    // ======== Field Constants ========

    /// **SRAM_STEP**
    ///
    /// timing between the sram0 and sram1 power state machine steps
    /// measured in units of the powman tick period (>=1us), 0 gives a delay of 1 unit
    ///
    /// Bits `[15:8]`, mask `0x0000ff00`
    pub const SRAM_STEP_OFFSET: u32 = 8;
    pub const SRAM_STEP_MASK: u32 = 0x0000ff00;

    /// **XIP_STEP**
    ///
    /// timing between the xip power state machine steps
    /// measured in units of the lposc period, 0 gives a delay of 1 unit
    ///
    /// Bits `[7:4]`, mask `0x000000f0`
    pub const XIP_STEP_OFFSET: u32 = 4;
    pub const XIP_STEP_MASK: u32 = 0x000000f0;

    /// **SWCORE_STEP**
    ///
    /// timing between the swcore power state machine steps
    /// measured in units of the lposc period, 0 gives a delay of 1 unit
    ///
    /// Bits `[3:0]`, mask `0x0000000f`
    pub const SWCORE_STEP_OFFSET: u32 = 0;
    pub const SWCORE_STEP_MASK: u32 = 0x0000000f;

    // ======== Field Accessors ========

    /// **SRAM_STEP** `[15:8]` (RW)
    ///
    /// timing between the sram0 and sram1 power state machine steps
    /// measured in units of the powman tick period (>=1us), 0 gives a delay of 1 unit
    #[inline(always)]
    pub fn sram_step(&self) -> u32 {
        (self.value.read_bits() & Self::SRAM_STEP_MASK) >> Self::SRAM_STEP_OFFSET
    }

    /// Write **SRAM_STEP** `[15:8]` (RW)
    ///
    /// timing between the sram0 and sram1 power state machine steps
    /// measured in units of the powman tick period (>=1us), 0 gives a delay of 1 unit
    #[inline(always)]
    pub fn write_sram_step(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SRAM_STEP_MASK) | ((value << Self::SRAM_STEP_OFFSET) & Self::SRAM_STEP_MASK));
    }

    /// **XIP_STEP** `[7:4]` (RW)
    ///
    /// timing between the xip power state machine steps
    /// measured in units of the lposc period, 0 gives a delay of 1 unit
    #[inline(always)]
    pub fn xip_step(&self) -> u32 {
        (self.value.read_bits() & Self::XIP_STEP_MASK) >> Self::XIP_STEP_OFFSET
    }

    /// Write **XIP_STEP** `[7:4]` (RW)
    ///
    /// timing between the xip power state machine steps
    /// measured in units of the lposc period, 0 gives a delay of 1 unit
    #[inline(always)]
    pub fn write_xip_step(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::XIP_STEP_MASK) | ((value << Self::XIP_STEP_OFFSET) & Self::XIP_STEP_MASK));
    }

    /// **SWCORE_STEP** `[3:0]` (RW)
    ///
    /// timing between the swcore power state machine steps
    /// measured in units of the lposc period, 0 gives a delay of 1 unit
    #[inline(always)]
    pub fn swcore_step(&self) -> u32 {
        (self.value.read_bits() & Self::SWCORE_STEP_MASK) >> Self::SWCORE_STEP_OFFSET
    }

    /// Write **SWCORE_STEP** `[3:0]` (RW)
    ///
    /// timing between the swcore power state machine steps
    /// measured in units of the lposc period, 0 gives a delay of 1 unit
    #[inline(always)]
    pub fn write_swcore_step(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SWCORE_STEP_MASK) | ((value << Self::SWCORE_STEP_OFFSET) & Self::SWCORE_STEP_MASK));
    }

}

/// POW_DELAY value type for batch read/modify/write operations
///
/// Use `powdelay.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `powdelay.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PowDelayVal {
    bits: u32,
}

impl PowDelayVal {
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
        Self { bits: 0x00002011 }
    }

    /// Read **SRAM_STEP** `[15:8]`
    ///
    /// timing between the sram0 and sram1 power state machine steps
    /// measured in units of the powman tick period (>=1us), 0 gives a delay of 1 unit
    #[inline(always)]
    pub const fn sram_step(&self) -> u32 {
        (self.bits & PowDelay::SRAM_STEP_MASK) >> PowDelay::SRAM_STEP_OFFSET
    }

    /// Read **XIP_STEP** `[7:4]`
    ///
    /// timing between the xip power state machine steps
    /// measured in units of the lposc period, 0 gives a delay of 1 unit
    #[inline(always)]
    pub const fn xip_step(&self) -> u32 {
        (self.bits & PowDelay::XIP_STEP_MASK) >> PowDelay::XIP_STEP_OFFSET
    }

    /// Read **SWCORE_STEP** `[3:0]`
    ///
    /// timing between the swcore power state machine steps
    /// measured in units of the lposc period, 0 gives a delay of 1 unit
    #[inline(always)]
    pub const fn swcore_step(&self) -> u32 {
        (self.bits & PowDelay::SWCORE_STEP_MASK) >> PowDelay::SWCORE_STEP_OFFSET
    }

    /// Set **SRAM_STEP** `[15:8]`
    ///
    /// timing between the sram0 and sram1 power state machine steps
    /// measured in units of the powman tick period (>=1us), 0 gives a delay of 1 unit
    #[inline(always)]
    pub const fn with_sram_step(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !PowDelay::SRAM_STEP_MASK) | ((value << PowDelay::SRAM_STEP_OFFSET) & PowDelay::SRAM_STEP_MASK)
        }
    }

    /// Set **XIP_STEP** `[7:4]`
    ///
    /// timing between the xip power state machine steps
    /// measured in units of the lposc period, 0 gives a delay of 1 unit
    #[inline(always)]
    pub const fn with_xip_step(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !PowDelay::XIP_STEP_MASK) | ((value << PowDelay::XIP_STEP_OFFSET) & PowDelay::XIP_STEP_MASK)
        }
    }

    /// Set **SWCORE_STEP** `[3:0]`
    ///
    /// timing between the swcore power state machine steps
    /// measured in units of the lposc period, 0 gives a delay of 1 unit
    #[inline(always)]
    pub const fn with_swcore_step(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !PowDelay::SWCORE_STEP_MASK) | ((value << PowDelay::SWCORE_STEP_OFFSET) & PowDelay::SWCORE_STEP_MASK)
        }
    }

}

impl Default for PowDelayVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl PowDelay {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PowDelayVal {
        PowDelayVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: PowDelayVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(PowDelayVal) -> PowDelayVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// EXT_CTRL0 Register
///
/// Configures a gpio as a power mode aware control output
///
/// Address offset: 0x0044
/// Reset value: 0x0000003f
///
/// ## Fields
///
/// - **LP_EXIT_STATE** `[14]` (RW)
///
///   output level when exiting the low power state
///
/// - **LP_ENTRY_STATE** `[13]` (RW)
///
///   output level when entering the low power state
///
/// - **INIT_STATE** `[12]` (RW)
///
/// - **INIT** `[8]` (RW)
///
/// - **GPIO_SELECT** `[5:0]` (RW)
///
///   selects from gpio 0->30
///   set to 31 to disable this feature
#[repr(transparent)]
pub struct ExtCtrl0 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for ExtCtrl0 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ExtCtrl0 {
    // ======== Field Constants ========

    /// **LP_EXIT_STATE**
    ///
    /// output level when exiting the low power state
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const LP_EXIT_STATE_OFFSET: u32 = 14;
    pub const LP_EXIT_STATE_MASK: u32 = 0x00004000;
    pub const LP_EXIT_STATE: u32 = 1 << 14;

    /// **LP_ENTRY_STATE**
    ///
    /// output level when entering the low power state
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const LP_ENTRY_STATE_OFFSET: u32 = 13;
    pub const LP_ENTRY_STATE_MASK: u32 = 0x00002000;
    pub const LP_ENTRY_STATE: u32 = 1 << 13;

    /// **INIT_STATE**
    /// Bits `[12:12]`, mask `0x00001000`
    pub const INIT_STATE_OFFSET: u32 = 12;
    pub const INIT_STATE_MASK: u32 = 0x00001000;
    pub const INIT_STATE: u32 = 1 << 12;

    /// **INIT**
    /// Bits `[8:8]`, mask `0x00000100`
    pub const INIT_OFFSET: u32 = 8;
    pub const INIT_MASK: u32 = 0x00000100;
    pub const INIT: u32 = 1 << 8;

    /// **GPIO_SELECT**
    ///
    /// selects from gpio 0->30
    /// set to 31 to disable this feature
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const GPIO_SELECT_OFFSET: u32 = 0;
    pub const GPIO_SELECT_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **LP_EXIT_STATE** `[14]` (RW)
    ///
    /// output level when exiting the low power state
    #[inline(always)]
    pub fn lp_exit_state(&self) -> bool {
        (self.value.read_bits() & Self::LP_EXIT_STATE_MASK) != 0
    }

    /// Set LP_EXIT_STATE bit atomically (RW)
    #[inline(always)]
    pub fn set_lp_exit_state(&self) {
        self.set_bits(Self::LP_EXIT_STATE_MASK);
    }

    /// Clear LP_EXIT_STATE bit atomically (RW)
    #[inline(always)]
    pub fn clr_lp_exit_state(&self) {
        self.clr_bits(Self::LP_EXIT_STATE_MASK);
    }

    /// **LP_ENTRY_STATE** `[13]` (RW)
    ///
    /// output level when entering the low power state
    #[inline(always)]
    pub fn lp_entry_state(&self) -> bool {
        (self.value.read_bits() & Self::LP_ENTRY_STATE_MASK) != 0
    }

    /// Set LP_ENTRY_STATE bit atomically (RW)
    #[inline(always)]
    pub fn set_lp_entry_state(&self) {
        self.set_bits(Self::LP_ENTRY_STATE_MASK);
    }

    /// Clear LP_ENTRY_STATE bit atomically (RW)
    #[inline(always)]
    pub fn clr_lp_entry_state(&self) {
        self.clr_bits(Self::LP_ENTRY_STATE_MASK);
    }

    /// **INIT_STATE** `[12]` (RW)
    #[inline(always)]
    pub fn init_state(&self) -> bool {
        (self.value.read_bits() & Self::INIT_STATE_MASK) != 0
    }

    /// Set INIT_STATE bit atomically (RW)
    #[inline(always)]
    pub fn set_init_state(&self) {
        self.set_bits(Self::INIT_STATE_MASK);
    }

    /// Clear INIT_STATE bit atomically (RW)
    #[inline(always)]
    pub fn clr_init_state(&self) {
        self.clr_bits(Self::INIT_STATE_MASK);
    }

    /// **INIT** `[8]` (RW)
    #[inline(always)]
    pub fn init(&self) -> bool {
        (self.value.read_bits() & Self::INIT_MASK) != 0
    }

    /// Set INIT bit atomically (RW)
    #[inline(always)]
    pub fn set_init(&self) {
        self.set_bits(Self::INIT_MASK);
    }

    /// Clear INIT bit atomically (RW)
    #[inline(always)]
    pub fn clr_init(&self) {
        self.clr_bits(Self::INIT_MASK);
    }

    /// **GPIO_SELECT** `[5:0]` (RW)
    ///
    /// selects from gpio 0->30
    /// set to 31 to disable this feature
    #[inline(always)]
    pub fn gpio_select(&self) -> u32 {
        (self.value.read_bits() & Self::GPIO_SELECT_MASK) >> Self::GPIO_SELECT_OFFSET
    }

    /// Write **GPIO_SELECT** `[5:0]` (RW)
    ///
    /// selects from gpio 0->30
    /// set to 31 to disable this feature
    #[inline(always)]
    pub fn write_gpio_select(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::GPIO_SELECT_MASK) | ((value << Self::GPIO_SELECT_OFFSET) & Self::GPIO_SELECT_MASK));
    }

}

/// EXT_CTRL0 value type for batch read/modify/write operations
///
/// Use `extctrl0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `extctrl0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ExtCtrl0Val {
    bits: u32,
}

impl ExtCtrl0Val {
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
        Self { bits: 0x0000003f }
    }

    /// Read **LP_EXIT_STATE** `[14:14]`
    ///
    /// output level when exiting the low power state
    #[inline(always)]
    pub const fn lp_exit_state(&self) -> bool {
        (self.bits & ExtCtrl0::LP_EXIT_STATE_MASK) != 0
    }

    /// Read **LP_ENTRY_STATE** `[13:13]`
    ///
    /// output level when entering the low power state
    #[inline(always)]
    pub const fn lp_entry_state(&self) -> bool {
        (self.bits & ExtCtrl0::LP_ENTRY_STATE_MASK) != 0
    }

    /// Read **INIT_STATE** `[12:12]`
    #[inline(always)]
    pub const fn init_state(&self) -> bool {
        (self.bits & ExtCtrl0::INIT_STATE_MASK) != 0
    }

    /// Read **INIT** `[8:8]`
    #[inline(always)]
    pub const fn init(&self) -> bool {
        (self.bits & ExtCtrl0::INIT_MASK) != 0
    }

    /// Read **GPIO_SELECT** `[5:0]`
    ///
    /// selects from gpio 0->30
    /// set to 31 to disable this feature
    #[inline(always)]
    pub const fn gpio_select(&self) -> u32 {
        (self.bits & ExtCtrl0::GPIO_SELECT_MASK) >> ExtCtrl0::GPIO_SELECT_OFFSET
    }

    /// Set **LP_EXIT_STATE** `[14:14]`
    ///
    /// output level when exiting the low power state
    #[inline(always)]
    pub const fn with_lp_exit_state(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ExtCtrl0::LP_EXIT_STATE_MASK
            } else {
                self.bits & !ExtCtrl0::LP_EXIT_STATE_MASK
            }
        }
    }

    /// Set **LP_ENTRY_STATE** `[13:13]`
    ///
    /// output level when entering the low power state
    #[inline(always)]
    pub const fn with_lp_entry_state(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ExtCtrl0::LP_ENTRY_STATE_MASK
            } else {
                self.bits & !ExtCtrl0::LP_ENTRY_STATE_MASK
            }
        }
    }

    /// Set **INIT_STATE** `[12:12]`
    #[inline(always)]
    pub const fn with_init_state(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ExtCtrl0::INIT_STATE_MASK
            } else {
                self.bits & !ExtCtrl0::INIT_STATE_MASK
            }
        }
    }

    /// Set **INIT** `[8:8]`
    #[inline(always)]
    pub const fn with_init(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ExtCtrl0::INIT_MASK
            } else {
                self.bits & !ExtCtrl0::INIT_MASK
            }
        }
    }

    /// Set **GPIO_SELECT** `[5:0]`
    ///
    /// selects from gpio 0->30
    /// set to 31 to disable this feature
    #[inline(always)]
    pub const fn with_gpio_select(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ExtCtrl0::GPIO_SELECT_MASK) | ((value << ExtCtrl0::GPIO_SELECT_OFFSET) & ExtCtrl0::GPIO_SELECT_MASK)
        }
    }

}

impl Default for ExtCtrl0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl ExtCtrl0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ExtCtrl0Val {
        ExtCtrl0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ExtCtrl0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ExtCtrl0Val) -> ExtCtrl0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// EXT_CTRL1 Register
///
/// Configures a gpio as a power mode aware control output
///
/// Address offset: 0x0048
/// Reset value: 0x0000003f
///
/// ## Fields
///
/// - **LP_EXIT_STATE** `[14]` (RW)
///
///   output level when exiting the low power state
///
/// - **LP_ENTRY_STATE** `[13]` (RW)
///
///   output level when entering the low power state
///
/// - **INIT_STATE** `[12]` (RW)
///
/// - **INIT** `[8]` (RW)
///
/// - **GPIO_SELECT** `[5:0]` (RW)
///
///   selects from gpio 0->30
///   set to 31 to disable this feature
#[repr(transparent)]
pub struct ExtCtrl1 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for ExtCtrl1 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ExtCtrl1 {
    // ======== Field Constants ========

    /// **LP_EXIT_STATE**
    ///
    /// output level when exiting the low power state
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const LP_EXIT_STATE_OFFSET: u32 = 14;
    pub const LP_EXIT_STATE_MASK: u32 = 0x00004000;
    pub const LP_EXIT_STATE: u32 = 1 << 14;

    /// **LP_ENTRY_STATE**
    ///
    /// output level when entering the low power state
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const LP_ENTRY_STATE_OFFSET: u32 = 13;
    pub const LP_ENTRY_STATE_MASK: u32 = 0x00002000;
    pub const LP_ENTRY_STATE: u32 = 1 << 13;

    /// **INIT_STATE**
    /// Bits `[12:12]`, mask `0x00001000`
    pub const INIT_STATE_OFFSET: u32 = 12;
    pub const INIT_STATE_MASK: u32 = 0x00001000;
    pub const INIT_STATE: u32 = 1 << 12;

    /// **INIT**
    /// Bits `[8:8]`, mask `0x00000100`
    pub const INIT_OFFSET: u32 = 8;
    pub const INIT_MASK: u32 = 0x00000100;
    pub const INIT: u32 = 1 << 8;

    /// **GPIO_SELECT**
    ///
    /// selects from gpio 0->30
    /// set to 31 to disable this feature
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const GPIO_SELECT_OFFSET: u32 = 0;
    pub const GPIO_SELECT_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **LP_EXIT_STATE** `[14]` (RW)
    ///
    /// output level when exiting the low power state
    #[inline(always)]
    pub fn lp_exit_state(&self) -> bool {
        (self.value.read_bits() & Self::LP_EXIT_STATE_MASK) != 0
    }

    /// Set LP_EXIT_STATE bit atomically (RW)
    #[inline(always)]
    pub fn set_lp_exit_state(&self) {
        self.set_bits(Self::LP_EXIT_STATE_MASK);
    }

    /// Clear LP_EXIT_STATE bit atomically (RW)
    #[inline(always)]
    pub fn clr_lp_exit_state(&self) {
        self.clr_bits(Self::LP_EXIT_STATE_MASK);
    }

    /// **LP_ENTRY_STATE** `[13]` (RW)
    ///
    /// output level when entering the low power state
    #[inline(always)]
    pub fn lp_entry_state(&self) -> bool {
        (self.value.read_bits() & Self::LP_ENTRY_STATE_MASK) != 0
    }

    /// Set LP_ENTRY_STATE bit atomically (RW)
    #[inline(always)]
    pub fn set_lp_entry_state(&self) {
        self.set_bits(Self::LP_ENTRY_STATE_MASK);
    }

    /// Clear LP_ENTRY_STATE bit atomically (RW)
    #[inline(always)]
    pub fn clr_lp_entry_state(&self) {
        self.clr_bits(Self::LP_ENTRY_STATE_MASK);
    }

    /// **INIT_STATE** `[12]` (RW)
    #[inline(always)]
    pub fn init_state(&self) -> bool {
        (self.value.read_bits() & Self::INIT_STATE_MASK) != 0
    }

    /// Set INIT_STATE bit atomically (RW)
    #[inline(always)]
    pub fn set_init_state(&self) {
        self.set_bits(Self::INIT_STATE_MASK);
    }

    /// Clear INIT_STATE bit atomically (RW)
    #[inline(always)]
    pub fn clr_init_state(&self) {
        self.clr_bits(Self::INIT_STATE_MASK);
    }

    /// **INIT** `[8]` (RW)
    #[inline(always)]
    pub fn init(&self) -> bool {
        (self.value.read_bits() & Self::INIT_MASK) != 0
    }

    /// Set INIT bit atomically (RW)
    #[inline(always)]
    pub fn set_init(&self) {
        self.set_bits(Self::INIT_MASK);
    }

    /// Clear INIT bit atomically (RW)
    #[inline(always)]
    pub fn clr_init(&self) {
        self.clr_bits(Self::INIT_MASK);
    }

    /// **GPIO_SELECT** `[5:0]` (RW)
    ///
    /// selects from gpio 0->30
    /// set to 31 to disable this feature
    #[inline(always)]
    pub fn gpio_select(&self) -> u32 {
        (self.value.read_bits() & Self::GPIO_SELECT_MASK) >> Self::GPIO_SELECT_OFFSET
    }

    /// Write **GPIO_SELECT** `[5:0]` (RW)
    ///
    /// selects from gpio 0->30
    /// set to 31 to disable this feature
    #[inline(always)]
    pub fn write_gpio_select(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::GPIO_SELECT_MASK) | ((value << Self::GPIO_SELECT_OFFSET) & Self::GPIO_SELECT_MASK));
    }

}

/// EXT_CTRL1 value type for batch read/modify/write operations
///
/// Use `extctrl1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `extctrl1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ExtCtrl1Val {
    bits: u32,
}

impl ExtCtrl1Val {
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
        Self { bits: 0x0000003f }
    }

    /// Read **LP_EXIT_STATE** `[14:14]`
    ///
    /// output level when exiting the low power state
    #[inline(always)]
    pub const fn lp_exit_state(&self) -> bool {
        (self.bits & ExtCtrl1::LP_EXIT_STATE_MASK) != 0
    }

    /// Read **LP_ENTRY_STATE** `[13:13]`
    ///
    /// output level when entering the low power state
    #[inline(always)]
    pub const fn lp_entry_state(&self) -> bool {
        (self.bits & ExtCtrl1::LP_ENTRY_STATE_MASK) != 0
    }

    /// Read **INIT_STATE** `[12:12]`
    #[inline(always)]
    pub const fn init_state(&self) -> bool {
        (self.bits & ExtCtrl1::INIT_STATE_MASK) != 0
    }

    /// Read **INIT** `[8:8]`
    #[inline(always)]
    pub const fn init(&self) -> bool {
        (self.bits & ExtCtrl1::INIT_MASK) != 0
    }

    /// Read **GPIO_SELECT** `[5:0]`
    ///
    /// selects from gpio 0->30
    /// set to 31 to disable this feature
    #[inline(always)]
    pub const fn gpio_select(&self) -> u32 {
        (self.bits & ExtCtrl1::GPIO_SELECT_MASK) >> ExtCtrl1::GPIO_SELECT_OFFSET
    }

    /// Set **LP_EXIT_STATE** `[14:14]`
    ///
    /// output level when exiting the low power state
    #[inline(always)]
    pub const fn with_lp_exit_state(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ExtCtrl1::LP_EXIT_STATE_MASK
            } else {
                self.bits & !ExtCtrl1::LP_EXIT_STATE_MASK
            }
        }
    }

    /// Set **LP_ENTRY_STATE** `[13:13]`
    ///
    /// output level when entering the low power state
    #[inline(always)]
    pub const fn with_lp_entry_state(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ExtCtrl1::LP_ENTRY_STATE_MASK
            } else {
                self.bits & !ExtCtrl1::LP_ENTRY_STATE_MASK
            }
        }
    }

    /// Set **INIT_STATE** `[12:12]`
    #[inline(always)]
    pub const fn with_init_state(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ExtCtrl1::INIT_STATE_MASK
            } else {
                self.bits & !ExtCtrl1::INIT_STATE_MASK
            }
        }
    }

    /// Set **INIT** `[8:8]`
    #[inline(always)]
    pub const fn with_init(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ExtCtrl1::INIT_MASK
            } else {
                self.bits & !ExtCtrl1::INIT_MASK
            }
        }
    }

    /// Set **GPIO_SELECT** `[5:0]`
    ///
    /// selects from gpio 0->30
    /// set to 31 to disable this feature
    #[inline(always)]
    pub const fn with_gpio_select(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ExtCtrl1::GPIO_SELECT_MASK) | ((value << ExtCtrl1::GPIO_SELECT_OFFSET) & ExtCtrl1::GPIO_SELECT_MASK)
        }
    }

}

impl Default for ExtCtrl1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl ExtCtrl1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ExtCtrl1Val {
        ExtCtrl1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ExtCtrl1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ExtCtrl1Val) -> ExtCtrl1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// EXT_TIME_REF Register
///
/// Select a GPIO to use as a time reference, the source can be used to drive the low power clock at 32kHz, or to provide a 1ms tick to the timer, or provide a 1Hz tick to the timer. The tick selection is controlled by the POWMAN_TIMER register.
///
/// Address offset: 0x004c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **DRIVE_LPCK** `[4]` (RW)
///
///   Use the selected GPIO to drive the 32kHz low power clock, in place of LPOSC. This field must only be written when POWMAN_TIMER_RUN=0
///
/// - **SOURCE_SEL** `[1:0]` (RW)
///
///   0 -> gpio12
///   1 -> gpio20
///   2 -> gpio14
///   3 -> gpio22
#[repr(transparent)]
pub struct ExtTimeRef {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for ExtTimeRef {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ExtTimeRef {
    // ======== Field Constants ========

    /// **DRIVE_LPCK**
    ///
    /// Use the selected GPIO to drive the 32kHz low power clock, in place of LPOSC. This field must only be written when POWMAN_TIMER_RUN=0
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const DRIVE_LPCK_OFFSET: u32 = 4;
    pub const DRIVE_LPCK_MASK: u32 = 0x00000010;
    pub const DRIVE_LPCK: u32 = 1 << 4;

    /// **SOURCE_SEL**
    ///
    /// 0 -> gpio12
    /// 1 -> gpio20
    /// 2 -> gpio14
    /// 3 -> gpio22
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const SOURCE_SEL_OFFSET: u32 = 0;
    pub const SOURCE_SEL_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **DRIVE_LPCK** `[4]` (RW)
    ///
    /// Use the selected GPIO to drive the 32kHz low power clock, in place of LPOSC. This field must only be written when POWMAN_TIMER_RUN=0
    #[inline(always)]
    pub fn drive_lpck(&self) -> bool {
        (self.value.read_bits() & Self::DRIVE_LPCK_MASK) != 0
    }

    /// Set DRIVE_LPCK bit atomically (RW)
    #[inline(always)]
    pub fn set_drive_lpck(&self) {
        self.set_bits(Self::DRIVE_LPCK_MASK);
    }

    /// Clear DRIVE_LPCK bit atomically (RW)
    #[inline(always)]
    pub fn clr_drive_lpck(&self) {
        self.clr_bits(Self::DRIVE_LPCK_MASK);
    }

    /// **SOURCE_SEL** `[1:0]` (RW)
    ///
    /// 0 -> gpio12
    /// 1 -> gpio20
    /// 2 -> gpio14
    /// 3 -> gpio22
    #[inline(always)]
    pub fn source_sel(&self) -> u32 {
        (self.value.read_bits() & Self::SOURCE_SEL_MASK) >> Self::SOURCE_SEL_OFFSET
    }

    /// Write **SOURCE_SEL** `[1:0]` (RW)
    ///
    /// 0 -> gpio12
    /// 1 -> gpio20
    /// 2 -> gpio14
    /// 3 -> gpio22
    #[inline(always)]
    pub fn write_source_sel(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SOURCE_SEL_MASK) | ((value << Self::SOURCE_SEL_OFFSET) & Self::SOURCE_SEL_MASK));
    }

}

/// EXT_TIME_REF value type for batch read/modify/write operations
///
/// Use `exttimeref.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `exttimeref.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ExtTimeRefVal {
    bits: u32,
}

impl ExtTimeRefVal {
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

    /// Read **DRIVE_LPCK** `[4:4]`
    ///
    /// Use the selected GPIO to drive the 32kHz low power clock, in place of LPOSC. This field must only be written when POWMAN_TIMER_RUN=0
    #[inline(always)]
    pub const fn drive_lpck(&self) -> bool {
        (self.bits & ExtTimeRef::DRIVE_LPCK_MASK) != 0
    }

    /// Read **SOURCE_SEL** `[1:0]`
    ///
    /// 0 -> gpio12
    /// 1 -> gpio20
    /// 2 -> gpio14
    /// 3 -> gpio22
    #[inline(always)]
    pub const fn source_sel(&self) -> u32 {
        (self.bits & ExtTimeRef::SOURCE_SEL_MASK) >> ExtTimeRef::SOURCE_SEL_OFFSET
    }

    /// Set **DRIVE_LPCK** `[4:4]`
    ///
    /// Use the selected GPIO to drive the 32kHz low power clock, in place of LPOSC. This field must only be written when POWMAN_TIMER_RUN=0
    #[inline(always)]
    pub const fn with_drive_lpck(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ExtTimeRef::DRIVE_LPCK_MASK
            } else {
                self.bits & !ExtTimeRef::DRIVE_LPCK_MASK
            }
        }
    }

    /// Set **SOURCE_SEL** `[1:0]`
    ///
    /// 0 -> gpio12
    /// 1 -> gpio20
    /// 2 -> gpio14
    /// 3 -> gpio22
    #[inline(always)]
    pub const fn with_source_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ExtTimeRef::SOURCE_SEL_MASK) | ((value << ExtTimeRef::SOURCE_SEL_OFFSET) & ExtTimeRef::SOURCE_SEL_MASK)
        }
    }

}

impl Default for ExtTimeRefVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl ExtTimeRef {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ExtTimeRefVal {
        ExtTimeRefVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ExtTimeRefVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ExtTimeRefVal) -> ExtTimeRefVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// LPOSC_FREQ_KHZ_INT Register
///
/// Informs the AON Timer of the integer component of the clock frequency when running off the LPOSC.
///
/// Address offset: 0x0050
/// Reset value: 0x00000020
///
/// ## Fields
///
/// - **LPOSC_FREQ_KHZ_INT** `[5:0]` (RW)
///
///   Integer component of the LPOSC or GPIO clock source frequency in kHz. Default = 32 This field must only be written when POWMAN_TIMER_RUN=0 or POWMAN_TIMER_USING_XOSC=1
#[repr(transparent)]
pub struct LposcFreqKhzInt {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for LposcFreqKhzInt {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LposcFreqKhzInt {
    // ======== Field Constants ========

    /// **LPOSC_FREQ_KHZ_INT**
    ///
    /// Integer component of the LPOSC or GPIO clock source frequency in kHz. Default = 32 This field must only be written when POWMAN_TIMER_RUN=0 or POWMAN_TIMER_USING_XOSC=1
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const LPOSC_FREQ_KHZ_INT_OFFSET: u32 = 0;
    pub const LPOSC_FREQ_KHZ_INT_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **LPOSC_FREQ_KHZ_INT** `[5:0]` (RW)
    ///
    /// Integer component of the LPOSC or GPIO clock source frequency in kHz. Default = 32 This field must only be written when POWMAN_TIMER_RUN=0 or POWMAN_TIMER_USING_XOSC=1
    #[inline(always)]
    pub fn lposc_freq_khz_int(&self) -> u32 {
        (self.value.read_bits() & Self::LPOSC_FREQ_KHZ_INT_MASK) >> Self::LPOSC_FREQ_KHZ_INT_OFFSET
    }

    /// Write **LPOSC_FREQ_KHZ_INT** `[5:0]` (RW)
    ///
    /// Integer component of the LPOSC or GPIO clock source frequency in kHz. Default = 32 This field must only be written when POWMAN_TIMER_RUN=0 or POWMAN_TIMER_USING_XOSC=1
    #[inline(always)]
    pub fn write_lposc_freq_khz_int(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::LPOSC_FREQ_KHZ_INT_MASK) | ((value << Self::LPOSC_FREQ_KHZ_INT_OFFSET) & Self::LPOSC_FREQ_KHZ_INT_MASK));
    }

}

/// LPOSC_FREQ_KHZ_INT value type for batch read/modify/write operations
///
/// Use `lposcfreqkhzint.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lposcfreqkhzint.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LposcFreqKhzIntVal {
    bits: u32,
}

impl LposcFreqKhzIntVal {
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
        Self { bits: 0x00000020 }
    }

    /// Read **LPOSC_FREQ_KHZ_INT** `[5:0]`
    ///
    /// Integer component of the LPOSC or GPIO clock source frequency in kHz. Default = 32 This field must only be written when POWMAN_TIMER_RUN=0 or POWMAN_TIMER_USING_XOSC=1
    #[inline(always)]
    pub const fn lposc_freq_khz_int(&self) -> u32 {
        (self.bits & LposcFreqKhzInt::LPOSC_FREQ_KHZ_INT_MASK) >> LposcFreqKhzInt::LPOSC_FREQ_KHZ_INT_OFFSET
    }

    /// Set **LPOSC_FREQ_KHZ_INT** `[5:0]`
    ///
    /// Integer component of the LPOSC or GPIO clock source frequency in kHz. Default = 32 This field must only be written when POWMAN_TIMER_RUN=0 or POWMAN_TIMER_USING_XOSC=1
    #[inline(always)]
    pub const fn with_lposc_freq_khz_int(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LposcFreqKhzInt::LPOSC_FREQ_KHZ_INT_MASK) | ((value << LposcFreqKhzInt::LPOSC_FREQ_KHZ_INT_OFFSET) & LposcFreqKhzInt::LPOSC_FREQ_KHZ_INT_MASK)
        }
    }

}

impl Default for LposcFreqKhzIntVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl LposcFreqKhzInt {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LposcFreqKhzIntVal {
        LposcFreqKhzIntVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LposcFreqKhzIntVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LposcFreqKhzIntVal) -> LposcFreqKhzIntVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// LPOSC_FREQ_KHZ_FRAC Register
///
/// Informs the AON Timer of the fractional component of the clock frequency when running off the LPOSC.
///
/// Address offset: 0x0054
/// Reset value: 0x0000c49c
///
/// ## Fields
///
/// - **LPOSC_FREQ_KHZ_FRAC** `[15:0]` (RW)
///
///   Fractional component of the LPOSC or GPIO clock source frequency in kHz. Default = 0.768 This field must only be written when POWMAN_TIMER_RUN=0 or POWMAN_TIMER_USING_XOSC=1
#[repr(transparent)]
pub struct LposcFreqKhzFrac {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for LposcFreqKhzFrac {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LposcFreqKhzFrac {
    // ======== Field Constants ========

    /// **LPOSC_FREQ_KHZ_FRAC**
    ///
    /// Fractional component of the LPOSC or GPIO clock source frequency in kHz. Default = 0.768 This field must only be written when POWMAN_TIMER_RUN=0 or POWMAN_TIMER_USING_XOSC=1
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const LPOSC_FREQ_KHZ_FRAC_OFFSET: u32 = 0;
    pub const LPOSC_FREQ_KHZ_FRAC_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **LPOSC_FREQ_KHZ_FRAC** `[15:0]` (RW)
    ///
    /// Fractional component of the LPOSC or GPIO clock source frequency in kHz. Default = 0.768 This field must only be written when POWMAN_TIMER_RUN=0 or POWMAN_TIMER_USING_XOSC=1
    #[inline(always)]
    pub fn lposc_freq_khz_frac(&self) -> u32 {
        (self.value.read_bits() & Self::LPOSC_FREQ_KHZ_FRAC_MASK) >> Self::LPOSC_FREQ_KHZ_FRAC_OFFSET
    }

    /// Write **LPOSC_FREQ_KHZ_FRAC** `[15:0]` (RW)
    ///
    /// Fractional component of the LPOSC or GPIO clock source frequency in kHz. Default = 0.768 This field must only be written when POWMAN_TIMER_RUN=0 or POWMAN_TIMER_USING_XOSC=1
    #[inline(always)]
    pub fn write_lposc_freq_khz_frac(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::LPOSC_FREQ_KHZ_FRAC_MASK) | ((value << Self::LPOSC_FREQ_KHZ_FRAC_OFFSET) & Self::LPOSC_FREQ_KHZ_FRAC_MASK));
    }

}

/// LPOSC_FREQ_KHZ_FRAC value type for batch read/modify/write operations
///
/// Use `lposcfreqkhzfrac.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lposcfreqkhzfrac.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LposcFreqKhzFracVal {
    bits: u32,
}

impl LposcFreqKhzFracVal {
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
        Self { bits: 0x0000c49c }
    }

    /// Read **LPOSC_FREQ_KHZ_FRAC** `[15:0]`
    ///
    /// Fractional component of the LPOSC or GPIO clock source frequency in kHz. Default = 0.768 This field must only be written when POWMAN_TIMER_RUN=0 or POWMAN_TIMER_USING_XOSC=1
    #[inline(always)]
    pub const fn lposc_freq_khz_frac(&self) -> u32 {
        (self.bits & LposcFreqKhzFrac::LPOSC_FREQ_KHZ_FRAC_MASK) >> LposcFreqKhzFrac::LPOSC_FREQ_KHZ_FRAC_OFFSET
    }

    /// Set **LPOSC_FREQ_KHZ_FRAC** `[15:0]`
    ///
    /// Fractional component of the LPOSC or GPIO clock source frequency in kHz. Default = 0.768 This field must only be written when POWMAN_TIMER_RUN=0 or POWMAN_TIMER_USING_XOSC=1
    #[inline(always)]
    pub const fn with_lposc_freq_khz_frac(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LposcFreqKhzFrac::LPOSC_FREQ_KHZ_FRAC_MASK) | ((value << LposcFreqKhzFrac::LPOSC_FREQ_KHZ_FRAC_OFFSET) & LposcFreqKhzFrac::LPOSC_FREQ_KHZ_FRAC_MASK)
        }
    }

}

impl Default for LposcFreqKhzFracVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl LposcFreqKhzFrac {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LposcFreqKhzFracVal {
        LposcFreqKhzFracVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LposcFreqKhzFracVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LposcFreqKhzFracVal) -> LposcFreqKhzFracVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// XOSC_FREQ_KHZ_INT Register
///
/// Informs the AON Timer of the integer component of the clock frequency when running off the XOSC.
///
/// Address offset: 0x0058
/// Reset value: 0x00002ee0
///
/// ## Fields
///
/// - **XOSC_FREQ_KHZ_INT** `[15:0]` (RW)
///
///   Integer component of the XOSC frequency in kHz. Default = 12000 Must be >1 This field must only be written when POWMAN_TIMER_RUN=0 or POWMAN_TIMER_USING_XOSC=0
#[repr(transparent)]
pub struct XoscFreqKhzInt {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for XoscFreqKhzInt {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XoscFreqKhzInt {
    // ======== Field Constants ========

    /// **XOSC_FREQ_KHZ_INT**
    ///
    /// Integer component of the XOSC frequency in kHz. Default = 12000 Must be >1 This field must only be written when POWMAN_TIMER_RUN=0 or POWMAN_TIMER_USING_XOSC=0
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const XOSC_FREQ_KHZ_INT_OFFSET: u32 = 0;
    pub const XOSC_FREQ_KHZ_INT_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **XOSC_FREQ_KHZ_INT** `[15:0]` (RW)
    ///
    /// Integer component of the XOSC frequency in kHz. Default = 12000 Must be >1 This field must only be written when POWMAN_TIMER_RUN=0 or POWMAN_TIMER_USING_XOSC=0
    #[inline(always)]
    pub fn xosc_freq_khz_int(&self) -> u32 {
        (self.value.read_bits() & Self::XOSC_FREQ_KHZ_INT_MASK) >> Self::XOSC_FREQ_KHZ_INT_OFFSET
    }

    /// Write **XOSC_FREQ_KHZ_INT** `[15:0]` (RW)
    ///
    /// Integer component of the XOSC frequency in kHz. Default = 12000 Must be >1 This field must only be written when POWMAN_TIMER_RUN=0 or POWMAN_TIMER_USING_XOSC=0
    #[inline(always)]
    pub fn write_xosc_freq_khz_int(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::XOSC_FREQ_KHZ_INT_MASK) | ((value << Self::XOSC_FREQ_KHZ_INT_OFFSET) & Self::XOSC_FREQ_KHZ_INT_MASK));
    }

}

/// XOSC_FREQ_KHZ_INT value type for batch read/modify/write operations
///
/// Use `xoscfreqkhzint.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `xoscfreqkhzint.write_val(val)`.
#[derive(Clone, Copy)]
pub struct XoscFreqKhzIntVal {
    bits: u32,
}

impl XoscFreqKhzIntVal {
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
        Self { bits: 0x00002ee0 }
    }

    /// Read **XOSC_FREQ_KHZ_INT** `[15:0]`
    ///
    /// Integer component of the XOSC frequency in kHz. Default = 12000 Must be >1 This field must only be written when POWMAN_TIMER_RUN=0 or POWMAN_TIMER_USING_XOSC=0
    #[inline(always)]
    pub const fn xosc_freq_khz_int(&self) -> u32 {
        (self.bits & XoscFreqKhzInt::XOSC_FREQ_KHZ_INT_MASK) >> XoscFreqKhzInt::XOSC_FREQ_KHZ_INT_OFFSET
    }

    /// Set **XOSC_FREQ_KHZ_INT** `[15:0]`
    ///
    /// Integer component of the XOSC frequency in kHz. Default = 12000 Must be >1 This field must only be written when POWMAN_TIMER_RUN=0 or POWMAN_TIMER_USING_XOSC=0
    #[inline(always)]
    pub const fn with_xosc_freq_khz_int(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !XoscFreqKhzInt::XOSC_FREQ_KHZ_INT_MASK) | ((value << XoscFreqKhzInt::XOSC_FREQ_KHZ_INT_OFFSET) & XoscFreqKhzInt::XOSC_FREQ_KHZ_INT_MASK)
        }
    }

}

impl Default for XoscFreqKhzIntVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl XoscFreqKhzInt {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> XoscFreqKhzIntVal {
        XoscFreqKhzIntVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: XoscFreqKhzIntVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(XoscFreqKhzIntVal) -> XoscFreqKhzIntVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// XOSC_FREQ_KHZ_FRAC Register
///
/// Informs the AON Timer of the fractional component of the clock frequency when running off the XOSC.
///
/// Address offset: 0x005c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **XOSC_FREQ_KHZ_FRAC** `[15:0]` (RW)
///
///   Fractional component of the XOSC frequency in kHz. This field must only be written when POWMAN_TIMER_RUN=0 or POWMAN_TIMER_USING_XOSC=0
#[repr(transparent)]
pub struct XoscFreqKhzFrac {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for XoscFreqKhzFrac {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XoscFreqKhzFrac {
    // ======== Field Constants ========

    /// **XOSC_FREQ_KHZ_FRAC**
    ///
    /// Fractional component of the XOSC frequency in kHz. This field must only be written when POWMAN_TIMER_RUN=0 or POWMAN_TIMER_USING_XOSC=0
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const XOSC_FREQ_KHZ_FRAC_OFFSET: u32 = 0;
    pub const XOSC_FREQ_KHZ_FRAC_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **XOSC_FREQ_KHZ_FRAC** `[15:0]` (RW)
    ///
    /// Fractional component of the XOSC frequency in kHz. This field must only be written when POWMAN_TIMER_RUN=0 or POWMAN_TIMER_USING_XOSC=0
    #[inline(always)]
    pub fn xosc_freq_khz_frac(&self) -> u32 {
        (self.value.read_bits() & Self::XOSC_FREQ_KHZ_FRAC_MASK) >> Self::XOSC_FREQ_KHZ_FRAC_OFFSET
    }

    /// Write **XOSC_FREQ_KHZ_FRAC** `[15:0]` (RW)
    ///
    /// Fractional component of the XOSC frequency in kHz. This field must only be written when POWMAN_TIMER_RUN=0 or POWMAN_TIMER_USING_XOSC=0
    #[inline(always)]
    pub fn write_xosc_freq_khz_frac(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::XOSC_FREQ_KHZ_FRAC_MASK) | ((value << Self::XOSC_FREQ_KHZ_FRAC_OFFSET) & Self::XOSC_FREQ_KHZ_FRAC_MASK));
    }

}

/// XOSC_FREQ_KHZ_FRAC value type for batch read/modify/write operations
///
/// Use `xoscfreqkhzfrac.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `xoscfreqkhzfrac.write_val(val)`.
#[derive(Clone, Copy)]
pub struct XoscFreqKhzFracVal {
    bits: u32,
}

impl XoscFreqKhzFracVal {
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

    /// Read **XOSC_FREQ_KHZ_FRAC** `[15:0]`
    ///
    /// Fractional component of the XOSC frequency in kHz. This field must only be written when POWMAN_TIMER_RUN=0 or POWMAN_TIMER_USING_XOSC=0
    #[inline(always)]
    pub const fn xosc_freq_khz_frac(&self) -> u32 {
        (self.bits & XoscFreqKhzFrac::XOSC_FREQ_KHZ_FRAC_MASK) >> XoscFreqKhzFrac::XOSC_FREQ_KHZ_FRAC_OFFSET
    }

    /// Set **XOSC_FREQ_KHZ_FRAC** `[15:0]`
    ///
    /// Fractional component of the XOSC frequency in kHz. This field must only be written when POWMAN_TIMER_RUN=0 or POWMAN_TIMER_USING_XOSC=0
    #[inline(always)]
    pub const fn with_xosc_freq_khz_frac(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !XoscFreqKhzFrac::XOSC_FREQ_KHZ_FRAC_MASK) | ((value << XoscFreqKhzFrac::XOSC_FREQ_KHZ_FRAC_OFFSET) & XoscFreqKhzFrac::XOSC_FREQ_KHZ_FRAC_MASK)
        }
    }

}

impl Default for XoscFreqKhzFracVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl XoscFreqKhzFrac {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> XoscFreqKhzFracVal {
        XoscFreqKhzFracVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: XoscFreqKhzFracVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(XoscFreqKhzFracVal) -> XoscFreqKhzFracVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SET_TIME_63TO48 Register
///
/// Address offset: 0x0060
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SET_TIME_63TO48** `[15:0]` (RW)
///
///   For setting the time, do not use for reading the time, use POWMAN_READ_TIME_UPPER and POWMAN_READ_TIME_LOWER. This field must only be written when POWMAN_TIMER_RUN=0
#[repr(transparent)]
pub struct SetTime63to48 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for SetTime63to48 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SetTime63to48 {
    // ======== Field Constants ========

    /// **SET_TIME_63TO48**
    ///
    /// For setting the time, do not use for reading the time, use POWMAN_READ_TIME_UPPER and POWMAN_READ_TIME_LOWER. This field must only be written when POWMAN_TIMER_RUN=0
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const SET_TIME_63TO48_OFFSET: u32 = 0;
    pub const SET_TIME_63TO48_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **SET_TIME_63TO48** `[15:0]` (RW)
    ///
    /// For setting the time, do not use for reading the time, use POWMAN_READ_TIME_UPPER and POWMAN_READ_TIME_LOWER. This field must only be written when POWMAN_TIMER_RUN=0
    #[inline(always)]
    pub fn set_time_63to48(&self) -> u32 {
        (self.value.read_bits() & Self::SET_TIME_63TO48_MASK) >> Self::SET_TIME_63TO48_OFFSET
    }

    /// Write **SET_TIME_63TO48** `[15:0]` (RW)
    ///
    /// For setting the time, do not use for reading the time, use POWMAN_READ_TIME_UPPER and POWMAN_READ_TIME_LOWER. This field must only be written when POWMAN_TIMER_RUN=0
    #[inline(always)]
    pub fn write_set_time_63to48(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SET_TIME_63TO48_MASK) | ((value << Self::SET_TIME_63TO48_OFFSET) & Self::SET_TIME_63TO48_MASK));
    }

}

/// SET_TIME_63TO48 value type for batch read/modify/write operations
///
/// Use `settime63to48.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `settime63to48.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SetTime63to48Val {
    bits: u32,
}

impl SetTime63to48Val {
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

    /// Read **SET_TIME_63TO48** `[15:0]`
    ///
    /// For setting the time, do not use for reading the time, use POWMAN_READ_TIME_UPPER and POWMAN_READ_TIME_LOWER. This field must only be written when POWMAN_TIMER_RUN=0
    #[inline(always)]
    pub const fn set_time_63to48(&self) -> u32 {
        (self.bits & SetTime63to48::SET_TIME_63TO48_MASK) >> SetTime63to48::SET_TIME_63TO48_OFFSET
    }

    /// Set **SET_TIME_63TO48** `[15:0]`
    ///
    /// For setting the time, do not use for reading the time, use POWMAN_READ_TIME_UPPER and POWMAN_READ_TIME_LOWER. This field must only be written when POWMAN_TIMER_RUN=0
    #[inline(always)]
    pub const fn with_set_time_63to48(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SetTime63to48::SET_TIME_63TO48_MASK) | ((value << SetTime63to48::SET_TIME_63TO48_OFFSET) & SetTime63to48::SET_TIME_63TO48_MASK)
        }
    }

}

impl Default for SetTime63to48Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SetTime63to48 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SetTime63to48Val {
        SetTime63to48Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SetTime63to48Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SetTime63to48Val) -> SetTime63to48Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SET_TIME_47TO32 Register
///
/// Address offset: 0x0064
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SET_TIME_47TO32** `[15:0]` (RW)
///
///   For setting the time, do not use for reading the time, use POWMAN_READ_TIME_UPPER and POWMAN_READ_TIME_LOWER. This field must only be written when POWMAN_TIMER_RUN=0
#[repr(transparent)]
pub struct SetTime47to32 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for SetTime47to32 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SetTime47to32 {
    // ======== Field Constants ========

    /// **SET_TIME_47TO32**
    ///
    /// For setting the time, do not use for reading the time, use POWMAN_READ_TIME_UPPER and POWMAN_READ_TIME_LOWER. This field must only be written when POWMAN_TIMER_RUN=0
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const SET_TIME_47TO32_OFFSET: u32 = 0;
    pub const SET_TIME_47TO32_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **SET_TIME_47TO32** `[15:0]` (RW)
    ///
    /// For setting the time, do not use for reading the time, use POWMAN_READ_TIME_UPPER and POWMAN_READ_TIME_LOWER. This field must only be written when POWMAN_TIMER_RUN=0
    #[inline(always)]
    pub fn set_time_47to32(&self) -> u32 {
        (self.value.read_bits() & Self::SET_TIME_47TO32_MASK) >> Self::SET_TIME_47TO32_OFFSET
    }

    /// Write **SET_TIME_47TO32** `[15:0]` (RW)
    ///
    /// For setting the time, do not use for reading the time, use POWMAN_READ_TIME_UPPER and POWMAN_READ_TIME_LOWER. This field must only be written when POWMAN_TIMER_RUN=0
    #[inline(always)]
    pub fn write_set_time_47to32(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SET_TIME_47TO32_MASK) | ((value << Self::SET_TIME_47TO32_OFFSET) & Self::SET_TIME_47TO32_MASK));
    }

}

/// SET_TIME_47TO32 value type for batch read/modify/write operations
///
/// Use `settime47to32.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `settime47to32.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SetTime47to32Val {
    bits: u32,
}

impl SetTime47to32Val {
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

    /// Read **SET_TIME_47TO32** `[15:0]`
    ///
    /// For setting the time, do not use for reading the time, use POWMAN_READ_TIME_UPPER and POWMAN_READ_TIME_LOWER. This field must only be written when POWMAN_TIMER_RUN=0
    #[inline(always)]
    pub const fn set_time_47to32(&self) -> u32 {
        (self.bits & SetTime47to32::SET_TIME_47TO32_MASK) >> SetTime47to32::SET_TIME_47TO32_OFFSET
    }

    /// Set **SET_TIME_47TO32** `[15:0]`
    ///
    /// For setting the time, do not use for reading the time, use POWMAN_READ_TIME_UPPER and POWMAN_READ_TIME_LOWER. This field must only be written when POWMAN_TIMER_RUN=0
    #[inline(always)]
    pub const fn with_set_time_47to32(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SetTime47to32::SET_TIME_47TO32_MASK) | ((value << SetTime47to32::SET_TIME_47TO32_OFFSET) & SetTime47to32::SET_TIME_47TO32_MASK)
        }
    }

}

impl Default for SetTime47to32Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SetTime47to32 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SetTime47to32Val {
        SetTime47to32Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SetTime47to32Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SetTime47to32Val) -> SetTime47to32Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SET_TIME_31TO16 Register
///
/// Address offset: 0x0068
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SET_TIME_31TO16** `[15:0]` (RW)
///
///   For setting the time, do not use for reading the time, use POWMAN_READ_TIME_UPPER and POWMAN_READ_TIME_LOWER. This field must only be written when POWMAN_TIMER_RUN=0
#[repr(transparent)]
pub struct SetTime31to16 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for SetTime31to16 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SetTime31to16 {
    // ======== Field Constants ========

    /// **SET_TIME_31TO16**
    ///
    /// For setting the time, do not use for reading the time, use POWMAN_READ_TIME_UPPER and POWMAN_READ_TIME_LOWER. This field must only be written when POWMAN_TIMER_RUN=0
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const SET_TIME_31TO16_OFFSET: u32 = 0;
    pub const SET_TIME_31TO16_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **SET_TIME_31TO16** `[15:0]` (RW)
    ///
    /// For setting the time, do not use for reading the time, use POWMAN_READ_TIME_UPPER and POWMAN_READ_TIME_LOWER. This field must only be written when POWMAN_TIMER_RUN=0
    #[inline(always)]
    pub fn set_time_31to16(&self) -> u32 {
        (self.value.read_bits() & Self::SET_TIME_31TO16_MASK) >> Self::SET_TIME_31TO16_OFFSET
    }

    /// Write **SET_TIME_31TO16** `[15:0]` (RW)
    ///
    /// For setting the time, do not use for reading the time, use POWMAN_READ_TIME_UPPER and POWMAN_READ_TIME_LOWER. This field must only be written when POWMAN_TIMER_RUN=0
    #[inline(always)]
    pub fn write_set_time_31to16(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SET_TIME_31TO16_MASK) | ((value << Self::SET_TIME_31TO16_OFFSET) & Self::SET_TIME_31TO16_MASK));
    }

}

/// SET_TIME_31TO16 value type for batch read/modify/write operations
///
/// Use `settime31to16.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `settime31to16.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SetTime31to16Val {
    bits: u32,
}

impl SetTime31to16Val {
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

    /// Read **SET_TIME_31TO16** `[15:0]`
    ///
    /// For setting the time, do not use for reading the time, use POWMAN_READ_TIME_UPPER and POWMAN_READ_TIME_LOWER. This field must only be written when POWMAN_TIMER_RUN=0
    #[inline(always)]
    pub const fn set_time_31to16(&self) -> u32 {
        (self.bits & SetTime31to16::SET_TIME_31TO16_MASK) >> SetTime31to16::SET_TIME_31TO16_OFFSET
    }

    /// Set **SET_TIME_31TO16** `[15:0]`
    ///
    /// For setting the time, do not use for reading the time, use POWMAN_READ_TIME_UPPER and POWMAN_READ_TIME_LOWER. This field must only be written when POWMAN_TIMER_RUN=0
    #[inline(always)]
    pub const fn with_set_time_31to16(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SetTime31to16::SET_TIME_31TO16_MASK) | ((value << SetTime31to16::SET_TIME_31TO16_OFFSET) & SetTime31to16::SET_TIME_31TO16_MASK)
        }
    }

}

impl Default for SetTime31to16Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SetTime31to16 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SetTime31to16Val {
        SetTime31to16Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SetTime31to16Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SetTime31to16Val) -> SetTime31to16Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SET_TIME_15TO0 Register
///
/// Address offset: 0x006c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SET_TIME_15TO0** `[15:0]` (RW)
///
///   For setting the time, do not use for reading the time, use POWMAN_READ_TIME_UPPER and POWMAN_READ_TIME_LOWER. This field must only be written when POWMAN_TIMER_RUN=0
#[repr(transparent)]
pub struct SetTime15to0 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for SetTime15to0 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SetTime15to0 {
    // ======== Field Constants ========

    /// **SET_TIME_15TO0**
    ///
    /// For setting the time, do not use for reading the time, use POWMAN_READ_TIME_UPPER and POWMAN_READ_TIME_LOWER. This field must only be written when POWMAN_TIMER_RUN=0
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const SET_TIME_15TO0_OFFSET: u32 = 0;
    pub const SET_TIME_15TO0_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **SET_TIME_15TO0** `[15:0]` (RW)
    ///
    /// For setting the time, do not use for reading the time, use POWMAN_READ_TIME_UPPER and POWMAN_READ_TIME_LOWER. This field must only be written when POWMAN_TIMER_RUN=0
    #[inline(always)]
    pub fn set_time_15to0(&self) -> u32 {
        (self.value.read_bits() & Self::SET_TIME_15TO0_MASK) >> Self::SET_TIME_15TO0_OFFSET
    }

    /// Write **SET_TIME_15TO0** `[15:0]` (RW)
    ///
    /// For setting the time, do not use for reading the time, use POWMAN_READ_TIME_UPPER and POWMAN_READ_TIME_LOWER. This field must only be written when POWMAN_TIMER_RUN=0
    #[inline(always)]
    pub fn write_set_time_15to0(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SET_TIME_15TO0_MASK) | ((value << Self::SET_TIME_15TO0_OFFSET) & Self::SET_TIME_15TO0_MASK));
    }

}

/// SET_TIME_15TO0 value type for batch read/modify/write operations
///
/// Use `settime15to0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `settime15to0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SetTime15to0Val {
    bits: u32,
}

impl SetTime15to0Val {
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

    /// Read **SET_TIME_15TO0** `[15:0]`
    ///
    /// For setting the time, do not use for reading the time, use POWMAN_READ_TIME_UPPER and POWMAN_READ_TIME_LOWER. This field must only be written when POWMAN_TIMER_RUN=0
    #[inline(always)]
    pub const fn set_time_15to0(&self) -> u32 {
        (self.bits & SetTime15to0::SET_TIME_15TO0_MASK) >> SetTime15to0::SET_TIME_15TO0_OFFSET
    }

    /// Set **SET_TIME_15TO0** `[15:0]`
    ///
    /// For setting the time, do not use for reading the time, use POWMAN_READ_TIME_UPPER and POWMAN_READ_TIME_LOWER. This field must only be written when POWMAN_TIMER_RUN=0
    #[inline(always)]
    pub const fn with_set_time_15to0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SetTime15to0::SET_TIME_15TO0_MASK) | ((value << SetTime15to0::SET_TIME_15TO0_OFFSET) & SetTime15to0::SET_TIME_15TO0_MASK)
        }
    }

}

impl Default for SetTime15to0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SetTime15to0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SetTime15to0Val {
        SetTime15to0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SetTime15to0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SetTime15to0Val) -> SetTime15to0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// READ_TIME_UPPER Register
///
/// Address offset: 0x0070
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **READ_TIME_UPPER** `[31:0]` (RO)
///
///   For reading bits 63:32 of the timer. When reading all 64 bits it is possible for the LOWER count to rollover during the read. It is recommended to read UPPER, then LOWER, then re-read UPPER and, if it has changed, re-read LOWER.
#[repr(transparent)]
pub struct ReadTimeUpper {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for ReadTimeUpper {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ReadTimeUpper {
    // ======== Field Constants ========

    /// **READ_TIME_UPPER**
    ///
    /// For reading bits 63:32 of the timer. When reading all 64 bits it is possible for the LOWER count to rollover during the read. It is recommended to read UPPER, then LOWER, then re-read UPPER and, if it has changed, re-read LOWER.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const READ_TIME_UPPER_OFFSET: u32 = 0;
    pub const READ_TIME_UPPER_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **READ_TIME_UPPER** `[31:0]` (RO)
    ///
    /// For reading bits 63:32 of the timer. When reading all 64 bits it is possible for the LOWER count to rollover during the read. It is recommended to read UPPER, then LOWER, then re-read UPPER and, if it has changed, re-read LOWER.
    #[inline(always)]
    pub fn read_time_upper(&self) -> u32 {
        (self.value.read_bits() & Self::READ_TIME_UPPER_MASK) >> Self::READ_TIME_UPPER_OFFSET
    }

}

/// READ_TIME_UPPER value type for batch read/modify/write operations
///
/// Use `readtimeupper.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `readtimeupper.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ReadTimeUpperVal {
    bits: u32,
}

impl ReadTimeUpperVal {
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

    /// Read **READ_TIME_UPPER** `[31:0]`
    ///
    /// For reading bits 63:32 of the timer. When reading all 64 bits it is possible for the LOWER count to rollover during the read. It is recommended to read UPPER, then LOWER, then re-read UPPER and, if it has changed, re-read LOWER.
    #[inline(always)]
    pub const fn read_time_upper(&self) -> u32 {
        (self.bits & ReadTimeUpper::READ_TIME_UPPER_MASK) >> ReadTimeUpper::READ_TIME_UPPER_OFFSET
    }

}

impl Default for ReadTimeUpperVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl ReadTimeUpper {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ReadTimeUpperVal {
        ReadTimeUpperVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ReadTimeUpperVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ReadTimeUpperVal) -> ReadTimeUpperVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// READ_TIME_LOWER Register
///
/// Address offset: 0x0074
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **READ_TIME_LOWER** `[31:0]` (RO)
///
///   For reading bits 31:0 of the timer.
#[repr(transparent)]
pub struct ReadTimeLower {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for ReadTimeLower {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ReadTimeLower {
    // ======== Field Constants ========

    /// **READ_TIME_LOWER**
    ///
    /// For reading bits 31:0 of the timer.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const READ_TIME_LOWER_OFFSET: u32 = 0;
    pub const READ_TIME_LOWER_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **READ_TIME_LOWER** `[31:0]` (RO)
    ///
    /// For reading bits 31:0 of the timer.
    #[inline(always)]
    pub fn read_time_lower(&self) -> u32 {
        (self.value.read_bits() & Self::READ_TIME_LOWER_MASK) >> Self::READ_TIME_LOWER_OFFSET
    }

}

/// READ_TIME_LOWER value type for batch read/modify/write operations
///
/// Use `readtimelower.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `readtimelower.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ReadTimeLowerVal {
    bits: u32,
}

impl ReadTimeLowerVal {
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

    /// Read **READ_TIME_LOWER** `[31:0]`
    ///
    /// For reading bits 31:0 of the timer.
    #[inline(always)]
    pub const fn read_time_lower(&self) -> u32 {
        (self.bits & ReadTimeLower::READ_TIME_LOWER_MASK) >> ReadTimeLower::READ_TIME_LOWER_OFFSET
    }

}

impl Default for ReadTimeLowerVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl ReadTimeLower {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ReadTimeLowerVal {
        ReadTimeLowerVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ReadTimeLowerVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ReadTimeLowerVal) -> ReadTimeLowerVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// ALARM_TIME_63TO48 Register
///
/// Address offset: 0x0078
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **ALARM_TIME_63TO48** `[15:0]` (RW)
///
///   This field must only be written when POWMAN_ALARM_ENAB=0
#[repr(transparent)]
pub struct AlarmTime63to48 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for AlarmTime63to48 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl AlarmTime63to48 {
    // ======== Field Constants ========

    /// **ALARM_TIME_63TO48**
    ///
    /// This field must only be written when POWMAN_ALARM_ENAB=0
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const ALARM_TIME_63TO48_OFFSET: u32 = 0;
    pub const ALARM_TIME_63TO48_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **ALARM_TIME_63TO48** `[15:0]` (RW)
    ///
    /// This field must only be written when POWMAN_ALARM_ENAB=0
    #[inline(always)]
    pub fn alarm_time_63to48(&self) -> u32 {
        (self.value.read_bits() & Self::ALARM_TIME_63TO48_MASK) >> Self::ALARM_TIME_63TO48_OFFSET
    }

    /// Write **ALARM_TIME_63TO48** `[15:0]` (RW)
    ///
    /// This field must only be written when POWMAN_ALARM_ENAB=0
    #[inline(always)]
    pub fn write_alarm_time_63to48(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::ALARM_TIME_63TO48_MASK) | ((value << Self::ALARM_TIME_63TO48_OFFSET) & Self::ALARM_TIME_63TO48_MASK));
    }

}

/// ALARM_TIME_63TO48 value type for batch read/modify/write operations
///
/// Use `alarmtime63to48.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `alarmtime63to48.write_val(val)`.
#[derive(Clone, Copy)]
pub struct AlarmTime63to48Val {
    bits: u32,
}

impl AlarmTime63to48Val {
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

    /// Read **ALARM_TIME_63TO48** `[15:0]`
    ///
    /// This field must only be written when POWMAN_ALARM_ENAB=0
    #[inline(always)]
    pub const fn alarm_time_63to48(&self) -> u32 {
        (self.bits & AlarmTime63to48::ALARM_TIME_63TO48_MASK) >> AlarmTime63to48::ALARM_TIME_63TO48_OFFSET
    }

    /// Set **ALARM_TIME_63TO48** `[15:0]`
    ///
    /// This field must only be written when POWMAN_ALARM_ENAB=0
    #[inline(always)]
    pub const fn with_alarm_time_63to48(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AlarmTime63to48::ALARM_TIME_63TO48_MASK) | ((value << AlarmTime63to48::ALARM_TIME_63TO48_OFFSET) & AlarmTime63to48::ALARM_TIME_63TO48_MASK)
        }
    }

}

impl Default for AlarmTime63to48Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl AlarmTime63to48 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> AlarmTime63to48Val {
        AlarmTime63to48Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: AlarmTime63to48Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(AlarmTime63to48Val) -> AlarmTime63to48Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// ALARM_TIME_47TO32 Register
///
/// Address offset: 0x007c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **ALARM_TIME_47TO32** `[15:0]` (RW)
///
///   This field must only be written when POWMAN_ALARM_ENAB=0
#[repr(transparent)]
pub struct AlarmTime47to32 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for AlarmTime47to32 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl AlarmTime47to32 {
    // ======== Field Constants ========

    /// **ALARM_TIME_47TO32**
    ///
    /// This field must only be written when POWMAN_ALARM_ENAB=0
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const ALARM_TIME_47TO32_OFFSET: u32 = 0;
    pub const ALARM_TIME_47TO32_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **ALARM_TIME_47TO32** `[15:0]` (RW)
    ///
    /// This field must only be written when POWMAN_ALARM_ENAB=0
    #[inline(always)]
    pub fn alarm_time_47to32(&self) -> u32 {
        (self.value.read_bits() & Self::ALARM_TIME_47TO32_MASK) >> Self::ALARM_TIME_47TO32_OFFSET
    }

    /// Write **ALARM_TIME_47TO32** `[15:0]` (RW)
    ///
    /// This field must only be written when POWMAN_ALARM_ENAB=0
    #[inline(always)]
    pub fn write_alarm_time_47to32(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::ALARM_TIME_47TO32_MASK) | ((value << Self::ALARM_TIME_47TO32_OFFSET) & Self::ALARM_TIME_47TO32_MASK));
    }

}

/// ALARM_TIME_47TO32 value type for batch read/modify/write operations
///
/// Use `alarmtime47to32.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `alarmtime47to32.write_val(val)`.
#[derive(Clone, Copy)]
pub struct AlarmTime47to32Val {
    bits: u32,
}

impl AlarmTime47to32Val {
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

    /// Read **ALARM_TIME_47TO32** `[15:0]`
    ///
    /// This field must only be written when POWMAN_ALARM_ENAB=0
    #[inline(always)]
    pub const fn alarm_time_47to32(&self) -> u32 {
        (self.bits & AlarmTime47to32::ALARM_TIME_47TO32_MASK) >> AlarmTime47to32::ALARM_TIME_47TO32_OFFSET
    }

    /// Set **ALARM_TIME_47TO32** `[15:0]`
    ///
    /// This field must only be written when POWMAN_ALARM_ENAB=0
    #[inline(always)]
    pub const fn with_alarm_time_47to32(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AlarmTime47to32::ALARM_TIME_47TO32_MASK) | ((value << AlarmTime47to32::ALARM_TIME_47TO32_OFFSET) & AlarmTime47to32::ALARM_TIME_47TO32_MASK)
        }
    }

}

impl Default for AlarmTime47to32Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl AlarmTime47to32 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> AlarmTime47to32Val {
        AlarmTime47to32Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: AlarmTime47to32Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(AlarmTime47to32Val) -> AlarmTime47to32Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// ALARM_TIME_31TO16 Register
///
/// Address offset: 0x0080
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **ALARM_TIME_31TO16** `[15:0]` (RW)
///
///   This field must only be written when POWMAN_ALARM_ENAB=0
#[repr(transparent)]
pub struct AlarmTime31to16 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for AlarmTime31to16 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl AlarmTime31to16 {
    // ======== Field Constants ========

    /// **ALARM_TIME_31TO16**
    ///
    /// This field must only be written when POWMAN_ALARM_ENAB=0
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const ALARM_TIME_31TO16_OFFSET: u32 = 0;
    pub const ALARM_TIME_31TO16_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **ALARM_TIME_31TO16** `[15:0]` (RW)
    ///
    /// This field must only be written when POWMAN_ALARM_ENAB=0
    #[inline(always)]
    pub fn alarm_time_31to16(&self) -> u32 {
        (self.value.read_bits() & Self::ALARM_TIME_31TO16_MASK) >> Self::ALARM_TIME_31TO16_OFFSET
    }

    /// Write **ALARM_TIME_31TO16** `[15:0]` (RW)
    ///
    /// This field must only be written when POWMAN_ALARM_ENAB=0
    #[inline(always)]
    pub fn write_alarm_time_31to16(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::ALARM_TIME_31TO16_MASK) | ((value << Self::ALARM_TIME_31TO16_OFFSET) & Self::ALARM_TIME_31TO16_MASK));
    }

}

/// ALARM_TIME_31TO16 value type for batch read/modify/write operations
///
/// Use `alarmtime31to16.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `alarmtime31to16.write_val(val)`.
#[derive(Clone, Copy)]
pub struct AlarmTime31to16Val {
    bits: u32,
}

impl AlarmTime31to16Val {
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

    /// Read **ALARM_TIME_31TO16** `[15:0]`
    ///
    /// This field must only be written when POWMAN_ALARM_ENAB=0
    #[inline(always)]
    pub const fn alarm_time_31to16(&self) -> u32 {
        (self.bits & AlarmTime31to16::ALARM_TIME_31TO16_MASK) >> AlarmTime31to16::ALARM_TIME_31TO16_OFFSET
    }

    /// Set **ALARM_TIME_31TO16** `[15:0]`
    ///
    /// This field must only be written when POWMAN_ALARM_ENAB=0
    #[inline(always)]
    pub const fn with_alarm_time_31to16(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AlarmTime31to16::ALARM_TIME_31TO16_MASK) | ((value << AlarmTime31to16::ALARM_TIME_31TO16_OFFSET) & AlarmTime31to16::ALARM_TIME_31TO16_MASK)
        }
    }

}

impl Default for AlarmTime31to16Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl AlarmTime31to16 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> AlarmTime31to16Val {
        AlarmTime31to16Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: AlarmTime31to16Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(AlarmTime31to16Val) -> AlarmTime31to16Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// ALARM_TIME_15TO0 Register
///
/// Address offset: 0x0084
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **ALARM_TIME_15TO0** `[15:0]` (RW)
///
///   This field must only be written when POWMAN_ALARM_ENAB=0
#[repr(transparent)]
pub struct AlarmTime15to0 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for AlarmTime15to0 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl AlarmTime15to0 {
    // ======== Field Constants ========

    /// **ALARM_TIME_15TO0**
    ///
    /// This field must only be written when POWMAN_ALARM_ENAB=0
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const ALARM_TIME_15TO0_OFFSET: u32 = 0;
    pub const ALARM_TIME_15TO0_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **ALARM_TIME_15TO0** `[15:0]` (RW)
    ///
    /// This field must only be written when POWMAN_ALARM_ENAB=0
    #[inline(always)]
    pub fn alarm_time_15to0(&self) -> u32 {
        (self.value.read_bits() & Self::ALARM_TIME_15TO0_MASK) >> Self::ALARM_TIME_15TO0_OFFSET
    }

    /// Write **ALARM_TIME_15TO0** `[15:0]` (RW)
    ///
    /// This field must only be written when POWMAN_ALARM_ENAB=0
    #[inline(always)]
    pub fn write_alarm_time_15to0(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::ALARM_TIME_15TO0_MASK) | ((value << Self::ALARM_TIME_15TO0_OFFSET) & Self::ALARM_TIME_15TO0_MASK));
    }

}

/// ALARM_TIME_15TO0 value type for batch read/modify/write operations
///
/// Use `alarmtime15to0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `alarmtime15to0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct AlarmTime15to0Val {
    bits: u32,
}

impl AlarmTime15to0Val {
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

    /// Read **ALARM_TIME_15TO0** `[15:0]`
    ///
    /// This field must only be written when POWMAN_ALARM_ENAB=0
    #[inline(always)]
    pub const fn alarm_time_15to0(&self) -> u32 {
        (self.bits & AlarmTime15to0::ALARM_TIME_15TO0_MASK) >> AlarmTime15to0::ALARM_TIME_15TO0_OFFSET
    }

    /// Set **ALARM_TIME_15TO0** `[15:0]`
    ///
    /// This field must only be written when POWMAN_ALARM_ENAB=0
    #[inline(always)]
    pub const fn with_alarm_time_15to0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AlarmTime15to0::ALARM_TIME_15TO0_MASK) | ((value << AlarmTime15to0::ALARM_TIME_15TO0_OFFSET) & AlarmTime15to0::ALARM_TIME_15TO0_MASK)
        }
    }

}

impl Default for AlarmTime15to0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl AlarmTime15to0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> AlarmTime15to0Val {
        AlarmTime15to0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: AlarmTime15to0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(AlarmTime15to0Val) -> AlarmTime15to0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// TIMER Register
///
/// Address offset: 0x0088
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **USING_GPIO_1HZ** `[19]` (RO)
///
///   Timer is synchronised to a 1hz gpio source
///
/// - **USING_GPIO_1KHZ** `[18]` (RO)
///
///   Timer is running from a 1khz gpio source
///
/// - **USING_LPOSC** `[17]` (RO)
///
///   Timer is running from lposc
///
/// - **USING_XOSC** `[16]` (RO)
///
///   Timer is running from xosc
///
/// - **USE_GPIO_1HZ** `[13]` (RW)
///
///   Selects the gpio source as the reference for the sec counter. The msec counter will continue to use the lposc or xosc reference.
///
/// - **USE_GPIO_1KHZ** `[10]` (WO)
///
///   switch to gpio as the source of the 1kHz timer tick
///
/// - **USE_XOSC** `[9]` (WO)
///
///   switch to xosc as the source of the 1kHz timer tick
///
/// - **USE_LPOSC** `[8]` (WO)
///
///   Switch to lposc as the source of the 1kHz timer tick
///
/// - **ALARM** `[6]` (RW)
///
///   Alarm has fired. Write to 1 to clear the alarm.
///
/// - **PWRUP_ON_ALARM** `[5]` (RW)
///
///   Alarm wakes the chip from low power mode
///
/// - **ALARM_ENAB** `[4]` (RW)
///
///   Enables the alarm. The alarm must be disabled while writing the alarm time.
///
/// - **CLEAR** `[2]` (WO)
///
///   Clears the timer, does not disable the timer and does not affect the alarm. This control can be written at any time.
///
/// - **RUN** `[1]` (RW)
///
///   Timer enable. Setting this bit causes the timer to begin counting up from its current value. Clearing this bit stops the timer from counting.
///   Before enabling the timer, set the POWMAN_LPOSC_FREQ* and POWMAN_XOSC_FREQ* registers to configure the count rate, and initialise the current time by writing to SET_TIME_63TO48 through SET_TIME_15TO0. You must not write to the SET_TIME_x registers when the timer is running.
///   Once configured, start the timer by setting POWMAN_TIMER_RUN=1. This will start the timer running from the LPOSC. When the XOSC is available switch the reference clock to XOSC then select it as the timer clock by setting POWMAN_TIMER_USE_XOSC=1
///
/// - **NONSEC_WRITE** `[0]` (RW)
///
///   Control whether Non-secure software can write to the timer registers. All other registers are hardwired to be inaccessible to Non-secure.
#[repr(transparent)]
pub struct Timer {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Timer {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Timer {
    // ======== Field Constants ========

    /// **USING_GPIO_1HZ**
    ///
    /// Timer is synchronised to a 1hz gpio source
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const USING_GPIO_1HZ_OFFSET: u32 = 19;
    pub const USING_GPIO_1HZ_MASK: u32 = 0x00080000;
    pub const USING_GPIO_1HZ: u32 = 1 << 19;

    /// **USING_GPIO_1KHZ**
    ///
    /// Timer is running from a 1khz gpio source
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const USING_GPIO_1KHZ_OFFSET: u32 = 18;
    pub const USING_GPIO_1KHZ_MASK: u32 = 0x00040000;
    pub const USING_GPIO_1KHZ: u32 = 1 << 18;

    /// **USING_LPOSC**
    ///
    /// Timer is running from lposc
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const USING_LPOSC_OFFSET: u32 = 17;
    pub const USING_LPOSC_MASK: u32 = 0x00020000;
    pub const USING_LPOSC: u32 = 1 << 17;

    /// **USING_XOSC**
    ///
    /// Timer is running from xosc
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const USING_XOSC_OFFSET: u32 = 16;
    pub const USING_XOSC_MASK: u32 = 0x00010000;
    pub const USING_XOSC: u32 = 1 << 16;

    /// **USE_GPIO_1HZ**
    ///
    /// Selects the gpio source as the reference for the sec counter. The msec counter will continue to use the lposc or xosc reference.
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const USE_GPIO_1HZ_OFFSET: u32 = 13;
    pub const USE_GPIO_1HZ_MASK: u32 = 0x00002000;
    pub const USE_GPIO_1HZ: u32 = 1 << 13;

    /// **USE_GPIO_1KHZ**
    ///
    /// switch to gpio as the source of the 1kHz timer tick
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const USE_GPIO_1KHZ_OFFSET: u32 = 10;
    pub const USE_GPIO_1KHZ_MASK: u32 = 0x00000400;
    pub const USE_GPIO_1KHZ: u32 = 1 << 10;

    /// **USE_XOSC**
    ///
    /// switch to xosc as the source of the 1kHz timer tick
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const USE_XOSC_OFFSET: u32 = 9;
    pub const USE_XOSC_MASK: u32 = 0x00000200;
    pub const USE_XOSC: u32 = 1 << 9;

    /// **USE_LPOSC**
    ///
    /// Switch to lposc as the source of the 1kHz timer tick
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const USE_LPOSC_OFFSET: u32 = 8;
    pub const USE_LPOSC_MASK: u32 = 0x00000100;
    pub const USE_LPOSC: u32 = 1 << 8;

    /// **ALARM**
    ///
    /// Alarm has fired. Write to 1 to clear the alarm.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const ALARM_OFFSET: u32 = 6;
    pub const ALARM_MASK: u32 = 0x00000040;
    pub const ALARM: u32 = 1 << 6;

    /// **PWRUP_ON_ALARM**
    ///
    /// Alarm wakes the chip from low power mode
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const PWRUP_ON_ALARM_OFFSET: u32 = 5;
    pub const PWRUP_ON_ALARM_MASK: u32 = 0x00000020;
    pub const PWRUP_ON_ALARM: u32 = 1 << 5;

    /// **ALARM_ENAB**
    ///
    /// Enables the alarm. The alarm must be disabled while writing the alarm time.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const ALARM_ENAB_OFFSET: u32 = 4;
    pub const ALARM_ENAB_MASK: u32 = 0x00000010;
    pub const ALARM_ENAB: u32 = 1 << 4;

    /// **CLEAR**
    ///
    /// Clears the timer, does not disable the timer and does not affect the alarm. This control can be written at any time.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const CLEAR_OFFSET: u32 = 2;
    pub const CLEAR_MASK: u32 = 0x00000004;
    pub const CLEAR: u32 = 1 << 2;

    /// **RUN**
    ///
    /// Timer enable. Setting this bit causes the timer to begin counting up from its current value. Clearing this bit stops the timer from counting.
    /// Before enabling the timer, set the POWMAN_LPOSC_FREQ* and POWMAN_XOSC_FREQ* registers to configure the count rate, and initialise the current time by writing to SET_TIME_63TO48 through SET_TIME_15TO0. You must not write to the SET_TIME_x registers when the timer is running.
    /// Once configured, start the timer by setting POWMAN_TIMER_RUN=1. This will start the timer running from the LPOSC. When the XOSC is available switch the reference clock to XOSC then select it as the timer clock by setting POWMAN_TIMER_USE_XOSC=1
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const RUN_OFFSET: u32 = 1;
    pub const RUN_MASK: u32 = 0x00000002;
    pub const RUN: u32 = 1 << 1;

    /// **NONSEC_WRITE**
    ///
    /// Control whether Non-secure software can write to the timer registers. All other registers are hardwired to be inaccessible to Non-secure.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const NONSEC_WRITE_OFFSET: u32 = 0;
    pub const NONSEC_WRITE_MASK: u32 = 0x00000001;
    pub const NONSEC_WRITE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **USING_GPIO_1HZ** `[19]` (RO)
    ///
    /// Timer is synchronised to a 1hz gpio source
    #[inline(always)]
    pub fn using_gpio_1hz(&self) -> bool {
        (self.value.read_bits() & Self::USING_GPIO_1HZ_MASK) != 0
    }

    /// **USING_GPIO_1KHZ** `[18]` (RO)
    ///
    /// Timer is running from a 1khz gpio source
    #[inline(always)]
    pub fn using_gpio_1khz(&self) -> bool {
        (self.value.read_bits() & Self::USING_GPIO_1KHZ_MASK) != 0
    }

    /// **USING_LPOSC** `[17]` (RO)
    ///
    /// Timer is running from lposc
    #[inline(always)]
    pub fn using_lposc(&self) -> bool {
        (self.value.read_bits() & Self::USING_LPOSC_MASK) != 0
    }

    /// **USING_XOSC** `[16]` (RO)
    ///
    /// Timer is running from xosc
    #[inline(always)]
    pub fn using_xosc(&self) -> bool {
        (self.value.read_bits() & Self::USING_XOSC_MASK) != 0
    }

    /// **USE_GPIO_1HZ** `[13]` (RW)
    ///
    /// Selects the gpio source as the reference for the sec counter. The msec counter will continue to use the lposc or xosc reference.
    #[inline(always)]
    pub fn use_gpio_1hz(&self) -> bool {
        (self.value.read_bits() & Self::USE_GPIO_1HZ_MASK) != 0
    }

    /// Set USE_GPIO_1HZ bit atomically (RW)
    #[inline(always)]
    pub fn set_use_gpio_1hz(&self) {
        self.set_bits(Self::USE_GPIO_1HZ_MASK);
    }

    /// Clear USE_GPIO_1HZ bit atomically (RW)
    #[inline(always)]
    pub fn clr_use_gpio_1hz(&self) {
        self.clr_bits(Self::USE_GPIO_1HZ_MASK);
    }

    /// Trigger **USE_GPIO_1KHZ** (WO)
    ///
    /// switch to gpio as the source of the 1kHz timer tick
    #[inline(always)]
    pub fn trigger_use_gpio_1khz(&self) {
        self.value.write_bits(Self::USE_GPIO_1KHZ_MASK);
    }

    /// Trigger **USE_XOSC** (WO)
    ///
    /// switch to xosc as the source of the 1kHz timer tick
    #[inline(always)]
    pub fn trigger_use_xosc(&self) {
        self.value.write_bits(Self::USE_XOSC_MASK);
    }

    /// Trigger **USE_LPOSC** (WO)
    ///
    /// Switch to lposc as the source of the 1kHz timer tick
    #[inline(always)]
    pub fn trigger_use_lposc(&self) {
        self.value.write_bits(Self::USE_LPOSC_MASK);
    }

    /// **ALARM** `[6]` (RW)
    ///
    /// Alarm has fired. Write to 1 to clear the alarm.
    #[inline(always)]
    pub fn alarm(&self) -> bool {
        (self.value.read_bits() & Self::ALARM_MASK) != 0
    }

    /// Set ALARM bit atomically (RW)
    #[inline(always)]
    pub fn set_alarm(&self) {
        self.set_bits(Self::ALARM_MASK);
    }

    /// Clear ALARM bit atomically (RW)
    #[inline(always)]
    pub fn clr_alarm(&self) {
        self.clr_bits(Self::ALARM_MASK);
    }

    /// **PWRUP_ON_ALARM** `[5]` (RW)
    ///
    /// Alarm wakes the chip from low power mode
    #[inline(always)]
    pub fn pwrup_on_alarm(&self) -> bool {
        (self.value.read_bits() & Self::PWRUP_ON_ALARM_MASK) != 0
    }

    /// Set PWRUP_ON_ALARM bit atomically (RW)
    #[inline(always)]
    pub fn set_pwrup_on_alarm(&self) {
        self.set_bits(Self::PWRUP_ON_ALARM_MASK);
    }

    /// Clear PWRUP_ON_ALARM bit atomically (RW)
    #[inline(always)]
    pub fn clr_pwrup_on_alarm(&self) {
        self.clr_bits(Self::PWRUP_ON_ALARM_MASK);
    }

    /// **ALARM_ENAB** `[4]` (RW)
    ///
    /// Enables the alarm. The alarm must be disabled while writing the alarm time.
    #[inline(always)]
    pub fn alarm_enab(&self) -> bool {
        (self.value.read_bits() & Self::ALARM_ENAB_MASK) != 0
    }

    /// Set ALARM_ENAB bit atomically (RW)
    #[inline(always)]
    pub fn set_alarm_enab(&self) {
        self.set_bits(Self::ALARM_ENAB_MASK);
    }

    /// Clear ALARM_ENAB bit atomically (RW)
    #[inline(always)]
    pub fn clr_alarm_enab(&self) {
        self.clr_bits(Self::ALARM_ENAB_MASK);
    }

    /// Trigger **CLEAR** (WO)
    ///
    /// Clears the timer, does not disable the timer and does not affect the alarm. This control can be written at any time.
    #[inline(always)]
    pub fn trigger_clear(&self) {
        self.value.write_bits(Self::CLEAR_MASK);
    }

    /// **RUN** `[1]` (RW)
    ///
    /// Timer enable. Setting this bit causes the timer to begin counting up from its current value. Clearing this bit stops the timer from counting.
    /// Before enabling the timer, set the POWMAN_LPOSC_FREQ* and POWMAN_XOSC_FREQ* registers to configure the count rate, and initialise the current time by writing to SET_TIME_63TO48 through SET_TIME_15TO0. You must not write to the SET_TIME_x registers when the timer is running.
    /// Once configured, start the timer by setting POWMAN_TIMER_RUN=1. This will start the timer running from the LPOSC. When the XOSC is available switch the reference clock to XOSC then select it as the timer clock by setting POWMAN_TIMER_USE_XOSC=1
    #[inline(always)]
    pub fn run(&self) -> bool {
        (self.value.read_bits() & Self::RUN_MASK) != 0
    }

    /// Set RUN bit atomically (RW)
    #[inline(always)]
    pub fn set_run(&self) {
        self.set_bits(Self::RUN_MASK);
    }

    /// Clear RUN bit atomically (RW)
    #[inline(always)]
    pub fn clr_run(&self) {
        self.clr_bits(Self::RUN_MASK);
    }

    /// **NONSEC_WRITE** `[0]` (RW)
    ///
    /// Control whether Non-secure software can write to the timer registers. All other registers are hardwired to be inaccessible to Non-secure.
    #[inline(always)]
    pub fn nonsec_write(&self) -> bool {
        (self.value.read_bits() & Self::NONSEC_WRITE_MASK) != 0
    }

    /// Set NONSEC_WRITE bit atomically (RW)
    #[inline(always)]
    pub fn set_nonsec_write(&self) {
        self.set_bits(Self::NONSEC_WRITE_MASK);
    }

    /// Clear NONSEC_WRITE bit atomically (RW)
    #[inline(always)]
    pub fn clr_nonsec_write(&self) {
        self.clr_bits(Self::NONSEC_WRITE_MASK);
    }

}

/// TIMER value type for batch read/modify/write operations
///
/// Use `timer.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `timer.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TimerVal {
    bits: u32,
}

impl TimerVal {
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

    /// Read **USING_GPIO_1HZ** `[19:19]`
    ///
    /// Timer is synchronised to a 1hz gpio source
    #[inline(always)]
    pub const fn using_gpio_1hz(&self) -> bool {
        (self.bits & Timer::USING_GPIO_1HZ_MASK) != 0
    }

    /// Read **USING_GPIO_1KHZ** `[18:18]`
    ///
    /// Timer is running from a 1khz gpio source
    #[inline(always)]
    pub const fn using_gpio_1khz(&self) -> bool {
        (self.bits & Timer::USING_GPIO_1KHZ_MASK) != 0
    }

    /// Read **USING_LPOSC** `[17:17]`
    ///
    /// Timer is running from lposc
    #[inline(always)]
    pub const fn using_lposc(&self) -> bool {
        (self.bits & Timer::USING_LPOSC_MASK) != 0
    }

    /// Read **USING_XOSC** `[16:16]`
    ///
    /// Timer is running from xosc
    #[inline(always)]
    pub const fn using_xosc(&self) -> bool {
        (self.bits & Timer::USING_XOSC_MASK) != 0
    }

    /// Read **USE_GPIO_1HZ** `[13:13]`
    ///
    /// Selects the gpio source as the reference for the sec counter. The msec counter will continue to use the lposc or xosc reference.
    #[inline(always)]
    pub const fn use_gpio_1hz(&self) -> bool {
        (self.bits & Timer::USE_GPIO_1HZ_MASK) != 0
    }

    /// Read **ALARM** `[6:6]`
    ///
    /// Alarm has fired. Write to 1 to clear the alarm.
    #[inline(always)]
    pub const fn alarm(&self) -> bool {
        (self.bits & Timer::ALARM_MASK) != 0
    }

    /// Read **PWRUP_ON_ALARM** `[5:5]`
    ///
    /// Alarm wakes the chip from low power mode
    #[inline(always)]
    pub const fn pwrup_on_alarm(&self) -> bool {
        (self.bits & Timer::PWRUP_ON_ALARM_MASK) != 0
    }

    /// Read **ALARM_ENAB** `[4:4]`
    ///
    /// Enables the alarm. The alarm must be disabled while writing the alarm time.
    #[inline(always)]
    pub const fn alarm_enab(&self) -> bool {
        (self.bits & Timer::ALARM_ENAB_MASK) != 0
    }

    /// Read **RUN** `[1:1]`
    ///
    /// Timer enable. Setting this bit causes the timer to begin counting up from its current value. Clearing this bit stops the timer from counting.
    /// Before enabling the timer, set the POWMAN_LPOSC_FREQ* and POWMAN_XOSC_FREQ* registers to configure the count rate, and initialise the current time by writing to SET_TIME_63TO48 through SET_TIME_15TO0. You must not write to the SET_TIME_x registers when the timer is running.
    /// Once configured, start the timer by setting POWMAN_TIMER_RUN=1. This will start the timer running from the LPOSC. When the XOSC is available switch the reference clock to XOSC then select it as the timer clock by setting POWMAN_TIMER_USE_XOSC=1
    #[inline(always)]
    pub const fn run(&self) -> bool {
        (self.bits & Timer::RUN_MASK) != 0
    }

    /// Read **NONSEC_WRITE** `[0:0]`
    ///
    /// Control whether Non-secure software can write to the timer registers. All other registers are hardwired to be inaccessible to Non-secure.
    #[inline(always)]
    pub const fn nonsec_write(&self) -> bool {
        (self.bits & Timer::NONSEC_WRITE_MASK) != 0
    }

    /// Set **USE_GPIO_1HZ** `[13:13]`
    ///
    /// Selects the gpio source as the reference for the sec counter. The msec counter will continue to use the lposc or xosc reference.
    #[inline(always)]
    pub const fn with_use_gpio_1hz(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Timer::USE_GPIO_1HZ_MASK
            } else {
                self.bits & !Timer::USE_GPIO_1HZ_MASK
            }
        }
    }

    /// Set **USE_GPIO_1KHZ** `[10:10]`
    ///
    /// switch to gpio as the source of the 1kHz timer tick
    #[inline(always)]
    pub const fn with_use_gpio_1khz(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Timer::USE_GPIO_1KHZ_MASK
            } else {
                self.bits & !Timer::USE_GPIO_1KHZ_MASK
            }
        }
    }

    /// Set **USE_XOSC** `[9:9]`
    ///
    /// switch to xosc as the source of the 1kHz timer tick
    #[inline(always)]
    pub const fn with_use_xosc(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Timer::USE_XOSC_MASK
            } else {
                self.bits & !Timer::USE_XOSC_MASK
            }
        }
    }

    /// Set **USE_LPOSC** `[8:8]`
    ///
    /// Switch to lposc as the source of the 1kHz timer tick
    #[inline(always)]
    pub const fn with_use_lposc(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Timer::USE_LPOSC_MASK
            } else {
                self.bits & !Timer::USE_LPOSC_MASK
            }
        }
    }

    /// Set **ALARM** `[6:6]`
    ///
    /// Alarm has fired. Write to 1 to clear the alarm.
    #[inline(always)]
    pub const fn with_alarm(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Timer::ALARM_MASK
            } else {
                self.bits & !Timer::ALARM_MASK
            }
        }
    }

    /// Set **PWRUP_ON_ALARM** `[5:5]`
    ///
    /// Alarm wakes the chip from low power mode
    #[inline(always)]
    pub const fn with_pwrup_on_alarm(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Timer::PWRUP_ON_ALARM_MASK
            } else {
                self.bits & !Timer::PWRUP_ON_ALARM_MASK
            }
        }
    }

    /// Set **ALARM_ENAB** `[4:4]`
    ///
    /// Enables the alarm. The alarm must be disabled while writing the alarm time.
    #[inline(always)]
    pub const fn with_alarm_enab(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Timer::ALARM_ENAB_MASK
            } else {
                self.bits & !Timer::ALARM_ENAB_MASK
            }
        }
    }

    /// Set **CLEAR** `[2:2]`
    ///
    /// Clears the timer, does not disable the timer and does not affect the alarm. This control can be written at any time.
    #[inline(always)]
    pub const fn with_clear(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Timer::CLEAR_MASK
            } else {
                self.bits & !Timer::CLEAR_MASK
            }
        }
    }

    /// Set **RUN** `[1:1]`
    ///
    /// Timer enable. Setting this bit causes the timer to begin counting up from its current value. Clearing this bit stops the timer from counting.
    /// Before enabling the timer, set the POWMAN_LPOSC_FREQ* and POWMAN_XOSC_FREQ* registers to configure the count rate, and initialise the current time by writing to SET_TIME_63TO48 through SET_TIME_15TO0. You must not write to the SET_TIME_x registers when the timer is running.
    /// Once configured, start the timer by setting POWMAN_TIMER_RUN=1. This will start the timer running from the LPOSC. When the XOSC is available switch the reference clock to XOSC then select it as the timer clock by setting POWMAN_TIMER_USE_XOSC=1
    #[inline(always)]
    pub const fn with_run(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Timer::RUN_MASK
            } else {
                self.bits & !Timer::RUN_MASK
            }
        }
    }

    /// Set **NONSEC_WRITE** `[0:0]`
    ///
    /// Control whether Non-secure software can write to the timer registers. All other registers are hardwired to be inaccessible to Non-secure.
    #[inline(always)]
    pub const fn with_nonsec_write(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Timer::NONSEC_WRITE_MASK
            } else {
                self.bits & !Timer::NONSEC_WRITE_MASK
            }
        }
    }

}

impl Default for TimerVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Timer {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TimerVal {
        TimerVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TimerVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TimerVal) -> TimerVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// PWRUP0 Register
///
/// 4 GPIO powerup events can be configured to wake the chip up from a low power state.
/// The pwrups are level/edge sensitive and can be set to trigger on a high/rising or low/falling event
/// The number of gpios available depends on the package option. An invalid selection will be ignored
/// source = 0 selects gpio0
/// .
/// .
/// source = 47 selects gpio47
/// source = 48 selects qspi_ss
/// source = 49 selects qspi_sd0
/// source = 50 selects qspi_sd1
/// source = 51 selects qspi_sd2
/// source = 52 selects qspi_sd3
/// source = 53 selects qspi_sclk
/// level = 0 triggers the pwrup when the source is low
/// level = 1 triggers the pwrup when the source is high
///
/// Address offset: 0x008c
/// Reset value: 0x0000003f
///
/// ## Fields
///
/// - **RAW_STATUS** `[10]` (RO)
///
///   Value of selected gpio pin (only if enable == 1)
///
/// - **STATUS** `[9]` (RW)
///
///   Status of gpio wakeup. Write to 1 to clear a latched edge detect.
///
/// - **MODE** `[8]` (RW)
///
///   Edge or level detect. Edge will detect a 0 to 1 transition (or 1 to 0 transition). Level will detect a 1 or 0. Both types of event get latched into the current_pwrup_req register.
///
/// - **DIRECTION** `[7]` (RW)
///
/// - **ENABLE** `[6]` (RW)
///
///   Set to 1 to enable the wakeup source. Set to 0 to disable the wakeup source and clear a pending wakeup event.
///   If using edge detect a latched edge needs to be cleared by writing 1 to the status register also.
///
/// - **SOURCE** `[5:0]` (RW)
#[repr(transparent)]
pub struct Pwrup0 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Pwrup0 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Pwrup0 {
    // ======== Field Constants ========

    /// **RAW_STATUS**
    ///
    /// Value of selected gpio pin (only if enable == 1)
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const RAW_STATUS_OFFSET: u32 = 10;
    pub const RAW_STATUS_MASK: u32 = 0x00000400;
    pub const RAW_STATUS: u32 = 1 << 10;

    /// **STATUS**
    ///
    /// Status of gpio wakeup. Write to 1 to clear a latched edge detect.
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const STATUS_OFFSET: u32 = 9;
    pub const STATUS_MASK: u32 = 0x00000200;
    pub const STATUS: u32 = 1 << 9;

    /// **MODE**
    ///
    /// Edge or level detect. Edge will detect a 0 to 1 transition (or 1 to 0 transition). Level will detect a 1 or 0. Both types of event get latched into the current_pwrup_req register.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const MODE_OFFSET: u32 = 8;
    pub const MODE_MASK: u32 = 0x00000100;
    pub const MODE: u32 = 1 << 8;
    pub const MODE_LEVEL: u32 = 0 << 8;
    pub const MODE_LEVEL_VAL: u32 = 0;
    pub const MODE_EDGE: u32 = 1 << 8;
    pub const MODE_EDGE_VAL: u32 = 1;

    /// **DIRECTION**
    /// Bits `[7:7]`, mask `0x00000080`
    pub const DIRECTION_OFFSET: u32 = 7;
    pub const DIRECTION_MASK: u32 = 0x00000080;
    pub const DIRECTION: u32 = 1 << 7;
    pub const DIRECTION_LOW_FALLING: u32 = 0 << 7;
    pub const DIRECTION_LOW_FALLING_VAL: u32 = 0;
    pub const DIRECTION_HIGH_RISING: u32 = 1 << 7;
    pub const DIRECTION_HIGH_RISING_VAL: u32 = 1;

    /// **ENABLE**
    ///
    /// Set to 1 to enable the wakeup source. Set to 0 to disable the wakeup source and clear a pending wakeup event.
    /// If using edge detect a latched edge needs to be cleared by writing 1 to the status register also.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const ENABLE_OFFSET: u32 = 6;
    pub const ENABLE_MASK: u32 = 0x00000040;
    pub const ENABLE: u32 = 1 << 6;

    /// **SOURCE**
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const SOURCE_OFFSET: u32 = 0;
    pub const SOURCE_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **RAW_STATUS** `[10]` (RO)
    ///
    /// Value of selected gpio pin (only if enable == 1)
    #[inline(always)]
    pub fn raw_status(&self) -> bool {
        (self.value.read_bits() & Self::RAW_STATUS_MASK) != 0
    }

    /// **STATUS** `[9]` (RW)
    ///
    /// Status of gpio wakeup. Write to 1 to clear a latched edge detect.
    #[inline(always)]
    pub fn status(&self) -> bool {
        (self.value.read_bits() & Self::STATUS_MASK) != 0
    }

    /// Set STATUS bit atomically (RW)
    #[inline(always)]
    pub fn set_status(&self) {
        self.set_bits(Self::STATUS_MASK);
    }

    /// Clear STATUS bit atomically (RW)
    #[inline(always)]
    pub fn clr_status(&self) {
        self.clr_bits(Self::STATUS_MASK);
    }

    /// **MODE** `[8]` (RW)
    ///
    /// Edge or level detect. Edge will detect a 0 to 1 transition (or 1 to 0 transition). Level will detect a 1 or 0. Both types of event get latched into the current_pwrup_req register.
    #[inline(always)]
    pub fn mode(&self) -> bool {
        (self.value.read_bits() & Self::MODE_MASK) != 0
    }

    /// Set MODE bit atomically (RW)
    #[inline(always)]
    pub fn set_mode(&self) {
        self.set_bits(Self::MODE_MASK);
    }

    /// Clear MODE bit atomically (RW)
    #[inline(always)]
    pub fn clr_mode(&self) {
        self.clr_bits(Self::MODE_MASK);
    }

    /// **DIRECTION** `[7]` (RW)
    #[inline(always)]
    pub fn direction(&self) -> bool {
        (self.value.read_bits() & Self::DIRECTION_MASK) != 0
    }

    /// Set DIRECTION bit atomically (RW)
    #[inline(always)]
    pub fn set_direction(&self) {
        self.set_bits(Self::DIRECTION_MASK);
    }

    /// Clear DIRECTION bit atomically (RW)
    #[inline(always)]
    pub fn clr_direction(&self) {
        self.clr_bits(Self::DIRECTION_MASK);
    }

    /// **ENABLE** `[6]` (RW)
    ///
    /// Set to 1 to enable the wakeup source. Set to 0 to disable the wakeup source and clear a pending wakeup event.
    /// If using edge detect a latched edge needs to be cleared by writing 1 to the status register also.
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

    /// **SOURCE** `[5:0]` (RW)
    #[inline(always)]
    pub fn source(&self) -> u32 {
        (self.value.read_bits() & Self::SOURCE_MASK) >> Self::SOURCE_OFFSET
    }

    /// Write **SOURCE** `[5:0]` (RW)
    #[inline(always)]
    pub fn write_source(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SOURCE_MASK) | ((value << Self::SOURCE_OFFSET) & Self::SOURCE_MASK));
    }

}

/// PWRUP0 value type for batch read/modify/write operations
///
/// Use `pwrup0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pwrup0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Pwrup0Val {
    bits: u32,
}

impl Pwrup0Val {
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
        Self { bits: 0x0000003f }
    }

    /// Read **RAW_STATUS** `[10:10]`
    ///
    /// Value of selected gpio pin (only if enable == 1)
    #[inline(always)]
    pub const fn raw_status(&self) -> bool {
        (self.bits & Pwrup0::RAW_STATUS_MASK) != 0
    }

    /// Read **STATUS** `[9:9]`
    ///
    /// Status of gpio wakeup. Write to 1 to clear a latched edge detect.
    #[inline(always)]
    pub const fn status(&self) -> bool {
        (self.bits & Pwrup0::STATUS_MASK) != 0
    }

    /// Read **MODE** `[8:8]`
    ///
    /// Edge or level detect. Edge will detect a 0 to 1 transition (or 1 to 0 transition). Level will detect a 1 or 0. Both types of event get latched into the current_pwrup_req register.
    #[inline(always)]
    pub const fn mode(&self) -> bool {
        (self.bits & Pwrup0::MODE_MASK) != 0
    }

    /// Read **DIRECTION** `[7:7]`
    #[inline(always)]
    pub const fn direction(&self) -> bool {
        (self.bits & Pwrup0::DIRECTION_MASK) != 0
    }

    /// Read **ENABLE** `[6:6]`
    ///
    /// Set to 1 to enable the wakeup source. Set to 0 to disable the wakeup source and clear a pending wakeup event.
    /// If using edge detect a latched edge needs to be cleared by writing 1 to the status register also.
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        (self.bits & Pwrup0::ENABLE_MASK) != 0
    }

    /// Read **SOURCE** `[5:0]`
    #[inline(always)]
    pub const fn source(&self) -> u32 {
        (self.bits & Pwrup0::SOURCE_MASK) >> Pwrup0::SOURCE_OFFSET
    }

    /// Set **STATUS** `[9:9]`
    ///
    /// Status of gpio wakeup. Write to 1 to clear a latched edge detect.
    #[inline(always)]
    pub const fn with_status(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pwrup0::STATUS_MASK
            } else {
                self.bits & !Pwrup0::STATUS_MASK
            }
        }
    }

    /// Set **MODE** `[8:8]`
    ///
    /// Edge or level detect. Edge will detect a 0 to 1 transition (or 1 to 0 transition). Level will detect a 1 or 0. Both types of event get latched into the current_pwrup_req register.
    #[inline(always)]
    pub const fn with_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pwrup0::MODE_MASK
            } else {
                self.bits & !Pwrup0::MODE_MASK
            }
        }
    }

    /// Set **DIRECTION** `[7:7]`
    #[inline(always)]
    pub const fn with_direction(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pwrup0::DIRECTION_MASK
            } else {
                self.bits & !Pwrup0::DIRECTION_MASK
            }
        }
    }

    /// Set **ENABLE** `[6:6]`
    ///
    /// Set to 1 to enable the wakeup source. Set to 0 to disable the wakeup source and clear a pending wakeup event.
    /// If using edge detect a latched edge needs to be cleared by writing 1 to the status register also.
    #[inline(always)]
    pub const fn with_enable(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pwrup0::ENABLE_MASK
            } else {
                self.bits & !Pwrup0::ENABLE_MASK
            }
        }
    }

    /// Set **SOURCE** `[5:0]`
    #[inline(always)]
    pub const fn with_source(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Pwrup0::SOURCE_MASK) | ((value << Pwrup0::SOURCE_OFFSET) & Pwrup0::SOURCE_MASK)
        }
    }

}

impl Default for Pwrup0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Pwrup0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Pwrup0Val {
        Pwrup0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Pwrup0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Pwrup0Val) -> Pwrup0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// PWRUP1 Register
///
/// 4 GPIO powerup events can be configured to wake the chip up from a low power state.
/// The pwrups are level/edge sensitive and can be set to trigger on a high/rising or low/falling event
/// The number of gpios available depends on the package option. An invalid selection will be ignored
/// source = 0 selects gpio0
/// .
/// .
/// source = 47 selects gpio47
/// source = 48 selects qspi_ss
/// source = 49 selects qspi_sd0
/// source = 50 selects qspi_sd1
/// source = 51 selects qspi_sd2
/// source = 52 selects qspi_sd3
/// source = 53 selects qspi_sclk
/// level = 0 triggers the pwrup when the source is low
/// level = 1 triggers the pwrup when the source is high
///
/// Address offset: 0x0090
/// Reset value: 0x0000003f
///
/// ## Fields
///
/// - **RAW_STATUS** `[10]` (RO)
///
///   Value of selected gpio pin (only if enable == 1)
///
/// - **STATUS** `[9]` (RW)
///
///   Status of gpio wakeup. Write to 1 to clear a latched edge detect.
///
/// - **MODE** `[8]` (RW)
///
///   Edge or level detect. Edge will detect a 0 to 1 transition (or 1 to 0 transition). Level will detect a 1 or 0. Both types of event get latched into the current_pwrup_req register.
///
/// - **DIRECTION** `[7]` (RW)
///
/// - **ENABLE** `[6]` (RW)
///
///   Set to 1 to enable the wakeup source. Set to 0 to disable the wakeup source and clear a pending wakeup event.
///   If using edge detect a latched edge needs to be cleared by writing 1 to the status register also.
///
/// - **SOURCE** `[5:0]` (RW)
#[repr(transparent)]
pub struct Pwrup1 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Pwrup1 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Pwrup1 {
    // ======== Field Constants ========

    /// **RAW_STATUS**
    ///
    /// Value of selected gpio pin (only if enable == 1)
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const RAW_STATUS_OFFSET: u32 = 10;
    pub const RAW_STATUS_MASK: u32 = 0x00000400;
    pub const RAW_STATUS: u32 = 1 << 10;

    /// **STATUS**
    ///
    /// Status of gpio wakeup. Write to 1 to clear a latched edge detect.
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const STATUS_OFFSET: u32 = 9;
    pub const STATUS_MASK: u32 = 0x00000200;
    pub const STATUS: u32 = 1 << 9;

    /// **MODE**
    ///
    /// Edge or level detect. Edge will detect a 0 to 1 transition (or 1 to 0 transition). Level will detect a 1 or 0. Both types of event get latched into the current_pwrup_req register.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const MODE_OFFSET: u32 = 8;
    pub const MODE_MASK: u32 = 0x00000100;
    pub const MODE: u32 = 1 << 8;
    pub const MODE_LEVEL: u32 = 0 << 8;
    pub const MODE_LEVEL_VAL: u32 = 0;
    pub const MODE_EDGE: u32 = 1 << 8;
    pub const MODE_EDGE_VAL: u32 = 1;

    /// **DIRECTION**
    /// Bits `[7:7]`, mask `0x00000080`
    pub const DIRECTION_OFFSET: u32 = 7;
    pub const DIRECTION_MASK: u32 = 0x00000080;
    pub const DIRECTION: u32 = 1 << 7;
    pub const DIRECTION_LOW_FALLING: u32 = 0 << 7;
    pub const DIRECTION_LOW_FALLING_VAL: u32 = 0;
    pub const DIRECTION_HIGH_RISING: u32 = 1 << 7;
    pub const DIRECTION_HIGH_RISING_VAL: u32 = 1;

    /// **ENABLE**
    ///
    /// Set to 1 to enable the wakeup source. Set to 0 to disable the wakeup source and clear a pending wakeup event.
    /// If using edge detect a latched edge needs to be cleared by writing 1 to the status register also.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const ENABLE_OFFSET: u32 = 6;
    pub const ENABLE_MASK: u32 = 0x00000040;
    pub const ENABLE: u32 = 1 << 6;

    /// **SOURCE**
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const SOURCE_OFFSET: u32 = 0;
    pub const SOURCE_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **RAW_STATUS** `[10]` (RO)
    ///
    /// Value of selected gpio pin (only if enable == 1)
    #[inline(always)]
    pub fn raw_status(&self) -> bool {
        (self.value.read_bits() & Self::RAW_STATUS_MASK) != 0
    }

    /// **STATUS** `[9]` (RW)
    ///
    /// Status of gpio wakeup. Write to 1 to clear a latched edge detect.
    #[inline(always)]
    pub fn status(&self) -> bool {
        (self.value.read_bits() & Self::STATUS_MASK) != 0
    }

    /// Set STATUS bit atomically (RW)
    #[inline(always)]
    pub fn set_status(&self) {
        self.set_bits(Self::STATUS_MASK);
    }

    /// Clear STATUS bit atomically (RW)
    #[inline(always)]
    pub fn clr_status(&self) {
        self.clr_bits(Self::STATUS_MASK);
    }

    /// **MODE** `[8]` (RW)
    ///
    /// Edge or level detect. Edge will detect a 0 to 1 transition (or 1 to 0 transition). Level will detect a 1 or 0. Both types of event get latched into the current_pwrup_req register.
    #[inline(always)]
    pub fn mode(&self) -> bool {
        (self.value.read_bits() & Self::MODE_MASK) != 0
    }

    /// Set MODE bit atomically (RW)
    #[inline(always)]
    pub fn set_mode(&self) {
        self.set_bits(Self::MODE_MASK);
    }

    /// Clear MODE bit atomically (RW)
    #[inline(always)]
    pub fn clr_mode(&self) {
        self.clr_bits(Self::MODE_MASK);
    }

    /// **DIRECTION** `[7]` (RW)
    #[inline(always)]
    pub fn direction(&self) -> bool {
        (self.value.read_bits() & Self::DIRECTION_MASK) != 0
    }

    /// Set DIRECTION bit atomically (RW)
    #[inline(always)]
    pub fn set_direction(&self) {
        self.set_bits(Self::DIRECTION_MASK);
    }

    /// Clear DIRECTION bit atomically (RW)
    #[inline(always)]
    pub fn clr_direction(&self) {
        self.clr_bits(Self::DIRECTION_MASK);
    }

    /// **ENABLE** `[6]` (RW)
    ///
    /// Set to 1 to enable the wakeup source. Set to 0 to disable the wakeup source and clear a pending wakeup event.
    /// If using edge detect a latched edge needs to be cleared by writing 1 to the status register also.
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

    /// **SOURCE** `[5:0]` (RW)
    #[inline(always)]
    pub fn source(&self) -> u32 {
        (self.value.read_bits() & Self::SOURCE_MASK) >> Self::SOURCE_OFFSET
    }

    /// Write **SOURCE** `[5:0]` (RW)
    #[inline(always)]
    pub fn write_source(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SOURCE_MASK) | ((value << Self::SOURCE_OFFSET) & Self::SOURCE_MASK));
    }

}

/// PWRUP1 value type for batch read/modify/write operations
///
/// Use `pwrup1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pwrup1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Pwrup1Val {
    bits: u32,
}

impl Pwrup1Val {
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
        Self { bits: 0x0000003f }
    }

    /// Read **RAW_STATUS** `[10:10]`
    ///
    /// Value of selected gpio pin (only if enable == 1)
    #[inline(always)]
    pub const fn raw_status(&self) -> bool {
        (self.bits & Pwrup1::RAW_STATUS_MASK) != 0
    }

    /// Read **STATUS** `[9:9]`
    ///
    /// Status of gpio wakeup. Write to 1 to clear a latched edge detect.
    #[inline(always)]
    pub const fn status(&self) -> bool {
        (self.bits & Pwrup1::STATUS_MASK) != 0
    }

    /// Read **MODE** `[8:8]`
    ///
    /// Edge or level detect. Edge will detect a 0 to 1 transition (or 1 to 0 transition). Level will detect a 1 or 0. Both types of event get latched into the current_pwrup_req register.
    #[inline(always)]
    pub const fn mode(&self) -> bool {
        (self.bits & Pwrup1::MODE_MASK) != 0
    }

    /// Read **DIRECTION** `[7:7]`
    #[inline(always)]
    pub const fn direction(&self) -> bool {
        (self.bits & Pwrup1::DIRECTION_MASK) != 0
    }

    /// Read **ENABLE** `[6:6]`
    ///
    /// Set to 1 to enable the wakeup source. Set to 0 to disable the wakeup source and clear a pending wakeup event.
    /// If using edge detect a latched edge needs to be cleared by writing 1 to the status register also.
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        (self.bits & Pwrup1::ENABLE_MASK) != 0
    }

    /// Read **SOURCE** `[5:0]`
    #[inline(always)]
    pub const fn source(&self) -> u32 {
        (self.bits & Pwrup1::SOURCE_MASK) >> Pwrup1::SOURCE_OFFSET
    }

    /// Set **STATUS** `[9:9]`
    ///
    /// Status of gpio wakeup. Write to 1 to clear a latched edge detect.
    #[inline(always)]
    pub const fn with_status(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pwrup1::STATUS_MASK
            } else {
                self.bits & !Pwrup1::STATUS_MASK
            }
        }
    }

    /// Set **MODE** `[8:8]`
    ///
    /// Edge or level detect. Edge will detect a 0 to 1 transition (or 1 to 0 transition). Level will detect a 1 or 0. Both types of event get latched into the current_pwrup_req register.
    #[inline(always)]
    pub const fn with_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pwrup1::MODE_MASK
            } else {
                self.bits & !Pwrup1::MODE_MASK
            }
        }
    }

    /// Set **DIRECTION** `[7:7]`
    #[inline(always)]
    pub const fn with_direction(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pwrup1::DIRECTION_MASK
            } else {
                self.bits & !Pwrup1::DIRECTION_MASK
            }
        }
    }

    /// Set **ENABLE** `[6:6]`
    ///
    /// Set to 1 to enable the wakeup source. Set to 0 to disable the wakeup source and clear a pending wakeup event.
    /// If using edge detect a latched edge needs to be cleared by writing 1 to the status register also.
    #[inline(always)]
    pub const fn with_enable(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pwrup1::ENABLE_MASK
            } else {
                self.bits & !Pwrup1::ENABLE_MASK
            }
        }
    }

    /// Set **SOURCE** `[5:0]`
    #[inline(always)]
    pub const fn with_source(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Pwrup1::SOURCE_MASK) | ((value << Pwrup1::SOURCE_OFFSET) & Pwrup1::SOURCE_MASK)
        }
    }

}

impl Default for Pwrup1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Pwrup1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Pwrup1Val {
        Pwrup1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Pwrup1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Pwrup1Val) -> Pwrup1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// PWRUP2 Register
///
/// 4 GPIO powerup events can be configured to wake the chip up from a low power state.
/// The pwrups are level/edge sensitive and can be set to trigger on a high/rising or low/falling event
/// The number of gpios available depends on the package option. An invalid selection will be ignored
/// source = 0 selects gpio0
/// .
/// .
/// source = 47 selects gpio47
/// source = 48 selects qspi_ss
/// source = 49 selects qspi_sd0
/// source = 50 selects qspi_sd1
/// source = 51 selects qspi_sd2
/// source = 52 selects qspi_sd3
/// source = 53 selects qspi_sclk
/// level = 0 triggers the pwrup when the source is low
/// level = 1 triggers the pwrup when the source is high
///
/// Address offset: 0x0094
/// Reset value: 0x0000003f
///
/// ## Fields
///
/// - **RAW_STATUS** `[10]` (RO)
///
///   Value of selected gpio pin (only if enable == 1)
///
/// - **STATUS** `[9]` (RW)
///
///   Status of gpio wakeup. Write to 1 to clear a latched edge detect.
///
/// - **MODE** `[8]` (RW)
///
///   Edge or level detect. Edge will detect a 0 to 1 transition (or 1 to 0 transition). Level will detect a 1 or 0. Both types of event get latched into the current_pwrup_req register.
///
/// - **DIRECTION** `[7]` (RW)
///
/// - **ENABLE** `[6]` (RW)
///
///   Set to 1 to enable the wakeup source. Set to 0 to disable the wakeup source and clear a pending wakeup event.
///   If using edge detect a latched edge needs to be cleared by writing 1 to the status register also.
///
/// - **SOURCE** `[5:0]` (RW)
#[repr(transparent)]
pub struct Pwrup2 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Pwrup2 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Pwrup2 {
    // ======== Field Constants ========

    /// **RAW_STATUS**
    ///
    /// Value of selected gpio pin (only if enable == 1)
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const RAW_STATUS_OFFSET: u32 = 10;
    pub const RAW_STATUS_MASK: u32 = 0x00000400;
    pub const RAW_STATUS: u32 = 1 << 10;

    /// **STATUS**
    ///
    /// Status of gpio wakeup. Write to 1 to clear a latched edge detect.
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const STATUS_OFFSET: u32 = 9;
    pub const STATUS_MASK: u32 = 0x00000200;
    pub const STATUS: u32 = 1 << 9;

    /// **MODE**
    ///
    /// Edge or level detect. Edge will detect a 0 to 1 transition (or 1 to 0 transition). Level will detect a 1 or 0. Both types of event get latched into the current_pwrup_req register.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const MODE_OFFSET: u32 = 8;
    pub const MODE_MASK: u32 = 0x00000100;
    pub const MODE: u32 = 1 << 8;
    pub const MODE_LEVEL: u32 = 0 << 8;
    pub const MODE_LEVEL_VAL: u32 = 0;
    pub const MODE_EDGE: u32 = 1 << 8;
    pub const MODE_EDGE_VAL: u32 = 1;

    /// **DIRECTION**
    /// Bits `[7:7]`, mask `0x00000080`
    pub const DIRECTION_OFFSET: u32 = 7;
    pub const DIRECTION_MASK: u32 = 0x00000080;
    pub const DIRECTION: u32 = 1 << 7;
    pub const DIRECTION_LOW_FALLING: u32 = 0 << 7;
    pub const DIRECTION_LOW_FALLING_VAL: u32 = 0;
    pub const DIRECTION_HIGH_RISING: u32 = 1 << 7;
    pub const DIRECTION_HIGH_RISING_VAL: u32 = 1;

    /// **ENABLE**
    ///
    /// Set to 1 to enable the wakeup source. Set to 0 to disable the wakeup source and clear a pending wakeup event.
    /// If using edge detect a latched edge needs to be cleared by writing 1 to the status register also.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const ENABLE_OFFSET: u32 = 6;
    pub const ENABLE_MASK: u32 = 0x00000040;
    pub const ENABLE: u32 = 1 << 6;

    /// **SOURCE**
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const SOURCE_OFFSET: u32 = 0;
    pub const SOURCE_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **RAW_STATUS** `[10]` (RO)
    ///
    /// Value of selected gpio pin (only if enable == 1)
    #[inline(always)]
    pub fn raw_status(&self) -> bool {
        (self.value.read_bits() & Self::RAW_STATUS_MASK) != 0
    }

    /// **STATUS** `[9]` (RW)
    ///
    /// Status of gpio wakeup. Write to 1 to clear a latched edge detect.
    #[inline(always)]
    pub fn status(&self) -> bool {
        (self.value.read_bits() & Self::STATUS_MASK) != 0
    }

    /// Set STATUS bit atomically (RW)
    #[inline(always)]
    pub fn set_status(&self) {
        self.set_bits(Self::STATUS_MASK);
    }

    /// Clear STATUS bit atomically (RW)
    #[inline(always)]
    pub fn clr_status(&self) {
        self.clr_bits(Self::STATUS_MASK);
    }

    /// **MODE** `[8]` (RW)
    ///
    /// Edge or level detect. Edge will detect a 0 to 1 transition (or 1 to 0 transition). Level will detect a 1 or 0. Both types of event get latched into the current_pwrup_req register.
    #[inline(always)]
    pub fn mode(&self) -> bool {
        (self.value.read_bits() & Self::MODE_MASK) != 0
    }

    /// Set MODE bit atomically (RW)
    #[inline(always)]
    pub fn set_mode(&self) {
        self.set_bits(Self::MODE_MASK);
    }

    /// Clear MODE bit atomically (RW)
    #[inline(always)]
    pub fn clr_mode(&self) {
        self.clr_bits(Self::MODE_MASK);
    }

    /// **DIRECTION** `[7]` (RW)
    #[inline(always)]
    pub fn direction(&self) -> bool {
        (self.value.read_bits() & Self::DIRECTION_MASK) != 0
    }

    /// Set DIRECTION bit atomically (RW)
    #[inline(always)]
    pub fn set_direction(&self) {
        self.set_bits(Self::DIRECTION_MASK);
    }

    /// Clear DIRECTION bit atomically (RW)
    #[inline(always)]
    pub fn clr_direction(&self) {
        self.clr_bits(Self::DIRECTION_MASK);
    }

    /// **ENABLE** `[6]` (RW)
    ///
    /// Set to 1 to enable the wakeup source. Set to 0 to disable the wakeup source and clear a pending wakeup event.
    /// If using edge detect a latched edge needs to be cleared by writing 1 to the status register also.
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

    /// **SOURCE** `[5:0]` (RW)
    #[inline(always)]
    pub fn source(&self) -> u32 {
        (self.value.read_bits() & Self::SOURCE_MASK) >> Self::SOURCE_OFFSET
    }

    /// Write **SOURCE** `[5:0]` (RW)
    #[inline(always)]
    pub fn write_source(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SOURCE_MASK) | ((value << Self::SOURCE_OFFSET) & Self::SOURCE_MASK));
    }

}

/// PWRUP2 value type for batch read/modify/write operations
///
/// Use `pwrup2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pwrup2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Pwrup2Val {
    bits: u32,
}

impl Pwrup2Val {
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
        Self { bits: 0x0000003f }
    }

    /// Read **RAW_STATUS** `[10:10]`
    ///
    /// Value of selected gpio pin (only if enable == 1)
    #[inline(always)]
    pub const fn raw_status(&self) -> bool {
        (self.bits & Pwrup2::RAW_STATUS_MASK) != 0
    }

    /// Read **STATUS** `[9:9]`
    ///
    /// Status of gpio wakeup. Write to 1 to clear a latched edge detect.
    #[inline(always)]
    pub const fn status(&self) -> bool {
        (self.bits & Pwrup2::STATUS_MASK) != 0
    }

    /// Read **MODE** `[8:8]`
    ///
    /// Edge or level detect. Edge will detect a 0 to 1 transition (or 1 to 0 transition). Level will detect a 1 or 0. Both types of event get latched into the current_pwrup_req register.
    #[inline(always)]
    pub const fn mode(&self) -> bool {
        (self.bits & Pwrup2::MODE_MASK) != 0
    }

    /// Read **DIRECTION** `[7:7]`
    #[inline(always)]
    pub const fn direction(&self) -> bool {
        (self.bits & Pwrup2::DIRECTION_MASK) != 0
    }

    /// Read **ENABLE** `[6:6]`
    ///
    /// Set to 1 to enable the wakeup source. Set to 0 to disable the wakeup source and clear a pending wakeup event.
    /// If using edge detect a latched edge needs to be cleared by writing 1 to the status register also.
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        (self.bits & Pwrup2::ENABLE_MASK) != 0
    }

    /// Read **SOURCE** `[5:0]`
    #[inline(always)]
    pub const fn source(&self) -> u32 {
        (self.bits & Pwrup2::SOURCE_MASK) >> Pwrup2::SOURCE_OFFSET
    }

    /// Set **STATUS** `[9:9]`
    ///
    /// Status of gpio wakeup. Write to 1 to clear a latched edge detect.
    #[inline(always)]
    pub const fn with_status(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pwrup2::STATUS_MASK
            } else {
                self.bits & !Pwrup2::STATUS_MASK
            }
        }
    }

    /// Set **MODE** `[8:8]`
    ///
    /// Edge or level detect. Edge will detect a 0 to 1 transition (or 1 to 0 transition). Level will detect a 1 or 0. Both types of event get latched into the current_pwrup_req register.
    #[inline(always)]
    pub const fn with_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pwrup2::MODE_MASK
            } else {
                self.bits & !Pwrup2::MODE_MASK
            }
        }
    }

    /// Set **DIRECTION** `[7:7]`
    #[inline(always)]
    pub const fn with_direction(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pwrup2::DIRECTION_MASK
            } else {
                self.bits & !Pwrup2::DIRECTION_MASK
            }
        }
    }

    /// Set **ENABLE** `[6:6]`
    ///
    /// Set to 1 to enable the wakeup source. Set to 0 to disable the wakeup source and clear a pending wakeup event.
    /// If using edge detect a latched edge needs to be cleared by writing 1 to the status register also.
    #[inline(always)]
    pub const fn with_enable(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pwrup2::ENABLE_MASK
            } else {
                self.bits & !Pwrup2::ENABLE_MASK
            }
        }
    }

    /// Set **SOURCE** `[5:0]`
    #[inline(always)]
    pub const fn with_source(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Pwrup2::SOURCE_MASK) | ((value << Pwrup2::SOURCE_OFFSET) & Pwrup2::SOURCE_MASK)
        }
    }

}

impl Default for Pwrup2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Pwrup2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Pwrup2Val {
        Pwrup2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Pwrup2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Pwrup2Val) -> Pwrup2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// PWRUP3 Register
///
/// 4 GPIO powerup events can be configured to wake the chip up from a low power state.
/// The pwrups are level/edge sensitive and can be set to trigger on a high/rising or low/falling event
/// The number of gpios available depends on the package option. An invalid selection will be ignored
/// source = 0 selects gpio0
/// .
/// .
/// source = 47 selects gpio47
/// source = 48 selects qspi_ss
/// source = 49 selects qspi_sd0
/// source = 50 selects qspi_sd1
/// source = 51 selects qspi_sd2
/// source = 52 selects qspi_sd3
/// source = 53 selects qspi_sclk
/// level = 0 triggers the pwrup when the source is low
/// level = 1 triggers the pwrup when the source is high
///
/// Address offset: 0x0098
/// Reset value: 0x0000003f
///
/// ## Fields
///
/// - **RAW_STATUS** `[10]` (RO)
///
///   Value of selected gpio pin (only if enable == 1)
///
/// - **STATUS** `[9]` (RW)
///
///   Status of gpio wakeup. Write to 1 to clear a latched edge detect.
///
/// - **MODE** `[8]` (RW)
///
///   Edge or level detect. Edge will detect a 0 to 1 transition (or 1 to 0 transition). Level will detect a 1 or 0. Both types of event get latched into the current_pwrup_req register.
///
/// - **DIRECTION** `[7]` (RW)
///
/// - **ENABLE** `[6]` (RW)
///
///   Set to 1 to enable the wakeup source. Set to 0 to disable the wakeup source and clear a pending wakeup event.
///   If using edge detect a latched edge needs to be cleared by writing 1 to the status register also.
///
/// - **SOURCE** `[5:0]` (RW)
#[repr(transparent)]
pub struct Pwrup3 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Pwrup3 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Pwrup3 {
    // ======== Field Constants ========

    /// **RAW_STATUS**
    ///
    /// Value of selected gpio pin (only if enable == 1)
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const RAW_STATUS_OFFSET: u32 = 10;
    pub const RAW_STATUS_MASK: u32 = 0x00000400;
    pub const RAW_STATUS: u32 = 1 << 10;

    /// **STATUS**
    ///
    /// Status of gpio wakeup. Write to 1 to clear a latched edge detect.
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const STATUS_OFFSET: u32 = 9;
    pub const STATUS_MASK: u32 = 0x00000200;
    pub const STATUS: u32 = 1 << 9;

    /// **MODE**
    ///
    /// Edge or level detect. Edge will detect a 0 to 1 transition (or 1 to 0 transition). Level will detect a 1 or 0. Both types of event get latched into the current_pwrup_req register.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const MODE_OFFSET: u32 = 8;
    pub const MODE_MASK: u32 = 0x00000100;
    pub const MODE: u32 = 1 << 8;
    pub const MODE_LEVEL: u32 = 0 << 8;
    pub const MODE_LEVEL_VAL: u32 = 0;
    pub const MODE_EDGE: u32 = 1 << 8;
    pub const MODE_EDGE_VAL: u32 = 1;

    /// **DIRECTION**
    /// Bits `[7:7]`, mask `0x00000080`
    pub const DIRECTION_OFFSET: u32 = 7;
    pub const DIRECTION_MASK: u32 = 0x00000080;
    pub const DIRECTION: u32 = 1 << 7;
    pub const DIRECTION_LOW_FALLING: u32 = 0 << 7;
    pub const DIRECTION_LOW_FALLING_VAL: u32 = 0;
    pub const DIRECTION_HIGH_RISING: u32 = 1 << 7;
    pub const DIRECTION_HIGH_RISING_VAL: u32 = 1;

    /// **ENABLE**
    ///
    /// Set to 1 to enable the wakeup source. Set to 0 to disable the wakeup source and clear a pending wakeup event.
    /// If using edge detect a latched edge needs to be cleared by writing 1 to the status register also.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const ENABLE_OFFSET: u32 = 6;
    pub const ENABLE_MASK: u32 = 0x00000040;
    pub const ENABLE: u32 = 1 << 6;

    /// **SOURCE**
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const SOURCE_OFFSET: u32 = 0;
    pub const SOURCE_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **RAW_STATUS** `[10]` (RO)
    ///
    /// Value of selected gpio pin (only if enable == 1)
    #[inline(always)]
    pub fn raw_status(&self) -> bool {
        (self.value.read_bits() & Self::RAW_STATUS_MASK) != 0
    }

    /// **STATUS** `[9]` (RW)
    ///
    /// Status of gpio wakeup. Write to 1 to clear a latched edge detect.
    #[inline(always)]
    pub fn status(&self) -> bool {
        (self.value.read_bits() & Self::STATUS_MASK) != 0
    }

    /// Set STATUS bit atomically (RW)
    #[inline(always)]
    pub fn set_status(&self) {
        self.set_bits(Self::STATUS_MASK);
    }

    /// Clear STATUS bit atomically (RW)
    #[inline(always)]
    pub fn clr_status(&self) {
        self.clr_bits(Self::STATUS_MASK);
    }

    /// **MODE** `[8]` (RW)
    ///
    /// Edge or level detect. Edge will detect a 0 to 1 transition (or 1 to 0 transition). Level will detect a 1 or 0. Both types of event get latched into the current_pwrup_req register.
    #[inline(always)]
    pub fn mode(&self) -> bool {
        (self.value.read_bits() & Self::MODE_MASK) != 0
    }

    /// Set MODE bit atomically (RW)
    #[inline(always)]
    pub fn set_mode(&self) {
        self.set_bits(Self::MODE_MASK);
    }

    /// Clear MODE bit atomically (RW)
    #[inline(always)]
    pub fn clr_mode(&self) {
        self.clr_bits(Self::MODE_MASK);
    }

    /// **DIRECTION** `[7]` (RW)
    #[inline(always)]
    pub fn direction(&self) -> bool {
        (self.value.read_bits() & Self::DIRECTION_MASK) != 0
    }

    /// Set DIRECTION bit atomically (RW)
    #[inline(always)]
    pub fn set_direction(&self) {
        self.set_bits(Self::DIRECTION_MASK);
    }

    /// Clear DIRECTION bit atomically (RW)
    #[inline(always)]
    pub fn clr_direction(&self) {
        self.clr_bits(Self::DIRECTION_MASK);
    }

    /// **ENABLE** `[6]` (RW)
    ///
    /// Set to 1 to enable the wakeup source. Set to 0 to disable the wakeup source and clear a pending wakeup event.
    /// If using edge detect a latched edge needs to be cleared by writing 1 to the status register also.
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

    /// **SOURCE** `[5:0]` (RW)
    #[inline(always)]
    pub fn source(&self) -> u32 {
        (self.value.read_bits() & Self::SOURCE_MASK) >> Self::SOURCE_OFFSET
    }

    /// Write **SOURCE** `[5:0]` (RW)
    #[inline(always)]
    pub fn write_source(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SOURCE_MASK) | ((value << Self::SOURCE_OFFSET) & Self::SOURCE_MASK));
    }

}

/// PWRUP3 value type for batch read/modify/write operations
///
/// Use `pwrup3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pwrup3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Pwrup3Val {
    bits: u32,
}

impl Pwrup3Val {
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
        Self { bits: 0x0000003f }
    }

    /// Read **RAW_STATUS** `[10:10]`
    ///
    /// Value of selected gpio pin (only if enable == 1)
    #[inline(always)]
    pub const fn raw_status(&self) -> bool {
        (self.bits & Pwrup3::RAW_STATUS_MASK) != 0
    }

    /// Read **STATUS** `[9:9]`
    ///
    /// Status of gpio wakeup. Write to 1 to clear a latched edge detect.
    #[inline(always)]
    pub const fn status(&self) -> bool {
        (self.bits & Pwrup3::STATUS_MASK) != 0
    }

    /// Read **MODE** `[8:8]`
    ///
    /// Edge or level detect. Edge will detect a 0 to 1 transition (or 1 to 0 transition). Level will detect a 1 or 0. Both types of event get latched into the current_pwrup_req register.
    #[inline(always)]
    pub const fn mode(&self) -> bool {
        (self.bits & Pwrup3::MODE_MASK) != 0
    }

    /// Read **DIRECTION** `[7:7]`
    #[inline(always)]
    pub const fn direction(&self) -> bool {
        (self.bits & Pwrup3::DIRECTION_MASK) != 0
    }

    /// Read **ENABLE** `[6:6]`
    ///
    /// Set to 1 to enable the wakeup source. Set to 0 to disable the wakeup source and clear a pending wakeup event.
    /// If using edge detect a latched edge needs to be cleared by writing 1 to the status register also.
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        (self.bits & Pwrup3::ENABLE_MASK) != 0
    }

    /// Read **SOURCE** `[5:0]`
    #[inline(always)]
    pub const fn source(&self) -> u32 {
        (self.bits & Pwrup3::SOURCE_MASK) >> Pwrup3::SOURCE_OFFSET
    }

    /// Set **STATUS** `[9:9]`
    ///
    /// Status of gpio wakeup. Write to 1 to clear a latched edge detect.
    #[inline(always)]
    pub const fn with_status(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pwrup3::STATUS_MASK
            } else {
                self.bits & !Pwrup3::STATUS_MASK
            }
        }
    }

    /// Set **MODE** `[8:8]`
    ///
    /// Edge or level detect. Edge will detect a 0 to 1 transition (or 1 to 0 transition). Level will detect a 1 or 0. Both types of event get latched into the current_pwrup_req register.
    #[inline(always)]
    pub const fn with_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pwrup3::MODE_MASK
            } else {
                self.bits & !Pwrup3::MODE_MASK
            }
        }
    }

    /// Set **DIRECTION** `[7:7]`
    #[inline(always)]
    pub const fn with_direction(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pwrup3::DIRECTION_MASK
            } else {
                self.bits & !Pwrup3::DIRECTION_MASK
            }
        }
    }

    /// Set **ENABLE** `[6:6]`
    ///
    /// Set to 1 to enable the wakeup source. Set to 0 to disable the wakeup source and clear a pending wakeup event.
    /// If using edge detect a latched edge needs to be cleared by writing 1 to the status register also.
    #[inline(always)]
    pub const fn with_enable(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pwrup3::ENABLE_MASK
            } else {
                self.bits & !Pwrup3::ENABLE_MASK
            }
        }
    }

    /// Set **SOURCE** `[5:0]`
    #[inline(always)]
    pub const fn with_source(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Pwrup3::SOURCE_MASK) | ((value << Pwrup3::SOURCE_OFFSET) & Pwrup3::SOURCE_MASK)
        }
    }

}

impl Default for Pwrup3Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Pwrup3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Pwrup3Val {
        Pwrup3Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Pwrup3Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Pwrup3Val) -> Pwrup3Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// CURRENT_PWRUP_REQ Register
///
/// Indicates current powerup request state
/// pwrup events can be cleared by removing the enable from the pwrup register. The alarm pwrup req can be cleared by clearing timer.alarm_enab
/// 0 = chip reset, for the source of the last reset see POWMAN_CHIP_RESET
/// 1 = pwrup0
/// 2 = pwrup1
/// 3 = pwrup2
/// 4 = pwrup3
/// 5 = coresight_pwrup
/// 6 = alarm_pwrup
///
/// Address offset: 0x009c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CURRENT_PWRUP_REQ** `[6:0]` (RO)
#[repr(transparent)]
pub struct CurrentPwrupReq {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for CurrentPwrupReq {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CurrentPwrupReq {
    // ======== Field Constants ========

    /// **CURRENT_PWRUP_REQ**
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const CURRENT_PWRUP_REQ_OFFSET: u32 = 0;
    pub const CURRENT_PWRUP_REQ_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **CURRENT_PWRUP_REQ** `[6:0]` (RO)
    #[inline(always)]
    pub fn current_pwrup_req(&self) -> u32 {
        (self.value.read_bits() & Self::CURRENT_PWRUP_REQ_MASK) >> Self::CURRENT_PWRUP_REQ_OFFSET
    }

}

/// CURRENT_PWRUP_REQ value type for batch read/modify/write operations
///
/// Use `currentpwrupreq.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `currentpwrupreq.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CurrentPwrupReqVal {
    bits: u32,
}

impl CurrentPwrupReqVal {
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

    /// Read **CURRENT_PWRUP_REQ** `[6:0]`
    #[inline(always)]
    pub const fn current_pwrup_req(&self) -> u32 {
        (self.bits & CurrentPwrupReq::CURRENT_PWRUP_REQ_MASK) >> CurrentPwrupReq::CURRENT_PWRUP_REQ_OFFSET
    }

}

impl Default for CurrentPwrupReqVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl CurrentPwrupReq {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CurrentPwrupReqVal {
        CurrentPwrupReqVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CurrentPwrupReqVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CurrentPwrupReqVal) -> CurrentPwrupReqVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// LAST_SWCORE_PWRUP Register
///
/// Indicates which pwrup source triggered the last switched-core power up
/// 0 = chip reset, for the source of the last reset see POWMAN_CHIP_RESET
/// 1 = pwrup0
/// 2 = pwrup1
/// 3 = pwrup2
/// 4 = pwrup3
/// 5 = coresight_pwrup
/// 6 = alarm_pwrup
///
/// Address offset: 0x00a0
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **LAST_SWCORE_PWRUP** `[6:0]` (RO)
#[repr(transparent)]
pub struct LastSwcorePwrup {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for LastSwcorePwrup {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LastSwcorePwrup {
    // ======== Field Constants ========

    /// **LAST_SWCORE_PWRUP**
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const LAST_SWCORE_PWRUP_OFFSET: u32 = 0;
    pub const LAST_SWCORE_PWRUP_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **LAST_SWCORE_PWRUP** `[6:0]` (RO)
    #[inline(always)]
    pub fn last_swcore_pwrup(&self) -> u32 {
        (self.value.read_bits() & Self::LAST_SWCORE_PWRUP_MASK) >> Self::LAST_SWCORE_PWRUP_OFFSET
    }

}

/// LAST_SWCORE_PWRUP value type for batch read/modify/write operations
///
/// Use `lastswcorepwrup.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lastswcorepwrup.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LastSwcorePwrupVal {
    bits: u32,
}

impl LastSwcorePwrupVal {
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

    /// Read **LAST_SWCORE_PWRUP** `[6:0]`
    #[inline(always)]
    pub const fn last_swcore_pwrup(&self) -> u32 {
        (self.bits & LastSwcorePwrup::LAST_SWCORE_PWRUP_MASK) >> LastSwcorePwrup::LAST_SWCORE_PWRUP_OFFSET
    }

}

impl Default for LastSwcorePwrupVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl LastSwcorePwrup {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LastSwcorePwrupVal {
        LastSwcorePwrupVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LastSwcorePwrupVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LastSwcorePwrupVal) -> LastSwcorePwrupVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// DBG_PWRCFG Register
///
/// Address offset: 0x00a4
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **IGNORE** `[0]` (RW)
///
///   Ignore pwrup req from debugger. If pwrup req is asserted then this will prevent power down and set powerdown blocked. Set ignore to stop paying attention to pwrup_req
#[repr(transparent)]
pub struct DbgPwrcfg {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for DbgPwrcfg {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DbgPwrcfg {
    // ======== Field Constants ========

    /// **IGNORE**
    ///
    /// Ignore pwrup req from debugger. If pwrup req is asserted then this will prevent power down and set powerdown blocked. Set ignore to stop paying attention to pwrup_req
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const IGNORE_OFFSET: u32 = 0;
    pub const IGNORE_MASK: u32 = 0x00000001;
    pub const IGNORE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **IGNORE** `[0]` (RW)
    ///
    /// Ignore pwrup req from debugger. If pwrup req is asserted then this will prevent power down and set powerdown blocked. Set ignore to stop paying attention to pwrup_req
    #[inline(always)]
    pub fn ignore(&self) -> bool {
        (self.value.read_bits() & Self::IGNORE_MASK) != 0
    }

    /// Set IGNORE bit atomically (RW)
    #[inline(always)]
    pub fn set_ignore(&self) {
        self.set_bits(Self::IGNORE_MASK);
    }

    /// Clear IGNORE bit atomically (RW)
    #[inline(always)]
    pub fn clr_ignore(&self) {
        self.clr_bits(Self::IGNORE_MASK);
    }

}

/// DBG_PWRCFG value type for batch read/modify/write operations
///
/// Use `dbgpwrcfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dbgpwrcfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DbgPwrcfgVal {
    bits: u32,
}

impl DbgPwrcfgVal {
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

    /// Read **IGNORE** `[0:0]`
    ///
    /// Ignore pwrup req from debugger. If pwrup req is asserted then this will prevent power down and set powerdown blocked. Set ignore to stop paying attention to pwrup_req
    #[inline(always)]
    pub const fn ignore(&self) -> bool {
        (self.bits & DbgPwrcfg::IGNORE_MASK) != 0
    }

    /// Set **IGNORE** `[0:0]`
    ///
    /// Ignore pwrup req from debugger. If pwrup req is asserted then this will prevent power down and set powerdown blocked. Set ignore to stop paying attention to pwrup_req
    #[inline(always)]
    pub const fn with_ignore(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DbgPwrcfg::IGNORE_MASK
            } else {
                self.bits & !DbgPwrcfg::IGNORE_MASK
            }
        }
    }

}

impl Default for DbgPwrcfgVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl DbgPwrcfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DbgPwrcfgVal {
        DbgPwrcfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DbgPwrcfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DbgPwrcfgVal) -> DbgPwrcfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// BOOTDIS Register
///
/// Tell the bootrom to ignore the BOOT0..3 registers following the next RSM reset (e.g. the next core power down/up).
/// If an early boot stage has soft-locked some OTP pages in order to protect their contents from later stages, there is a risk that Secure code running at a later stage can unlock the pages by powering the core up and down.
/// This register can be used to ensure that the bootloader runs as normal on the next power up, preventing Secure code at a later stage from accessing OTP in its unlocked state.
/// Should be used in conjunction with the OTP BOOTDIS register.
///
/// Address offset: 0x00a8
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NEXT** `[1]` (RW)
///
///   This flag always ORs writes into its current contents. It can be set but not cleared by software.
///   The BOOTDIS_NEXT bit is OR'd into the BOOTDIS_NOW bit when the core is powered down. Simultaneously, the BOOTDIS_NEXT bit is cleared. Setting this bit means that the BOOT0..3 registers will be ignored following the next reset of the RSM by powman.
///   This flag should be set by an early boot stage that has soft-locked OTP pages, to prevent later stages from unlocking it by power cycling.
///
/// - **NOW** `[0]` (RW)
///
///   When powman resets the RSM, the current value of BOOTDIS_NEXT is OR'd into BOOTDIS_NOW, and BOOTDIS_NEXT is cleared.
///   The bootrom checks this flag before reading the BOOT0..3 registers. If it is set, the bootrom clears it, and ignores the BOOT registers. This prevents Secure software from diverting the boot path before a bootloader has had the chance to soft lock OTP pages containing sensitive data.
#[repr(transparent)]
pub struct Bootdis {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Bootdis {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Bootdis {
    // ======== Field Constants ========

    /// **NEXT**
    ///
    /// This flag always ORs writes into its current contents. It can be set but not cleared by software.
    /// The BOOTDIS_NEXT bit is OR'd into the BOOTDIS_NOW bit when the core is powered down. Simultaneously, the BOOTDIS_NEXT bit is cleared. Setting this bit means that the BOOT0..3 registers will be ignored following the next reset of the RSM by powman.
    /// This flag should be set by an early boot stage that has soft-locked OTP pages, to prevent later stages from unlocking it by power cycling.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const NEXT_OFFSET: u32 = 1;
    pub const NEXT_MASK: u32 = 0x00000002;
    pub const NEXT: u32 = 1 << 1;

    /// **NOW**
    ///
    /// When powman resets the RSM, the current value of BOOTDIS_NEXT is OR'd into BOOTDIS_NOW, and BOOTDIS_NEXT is cleared.
    /// The bootrom checks this flag before reading the BOOT0..3 registers. If it is set, the bootrom clears it, and ignores the BOOT registers. This prevents Secure software from diverting the boot path before a bootloader has had the chance to soft lock OTP pages containing sensitive data.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const NOW_OFFSET: u32 = 0;
    pub const NOW_MASK: u32 = 0x00000001;
    pub const NOW: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **NEXT** `[1]` (RW)
    ///
    /// This flag always ORs writes into its current contents. It can be set but not cleared by software.
    /// The BOOTDIS_NEXT bit is OR'd into the BOOTDIS_NOW bit when the core is powered down. Simultaneously, the BOOTDIS_NEXT bit is cleared. Setting this bit means that the BOOT0..3 registers will be ignored following the next reset of the RSM by powman.
    /// This flag should be set by an early boot stage that has soft-locked OTP pages, to prevent later stages from unlocking it by power cycling.
    #[inline(always)]
    pub fn next(&self) -> bool {
        (self.value.read_bits() & Self::NEXT_MASK) != 0
    }

    /// Set NEXT bit atomically (RW)
    #[inline(always)]
    pub fn set_next(&self) {
        self.set_bits(Self::NEXT_MASK);
    }

    /// Clear NEXT bit atomically (RW)
    #[inline(always)]
    pub fn clr_next(&self) {
        self.clr_bits(Self::NEXT_MASK);
    }

    /// **NOW** `[0]` (RW)
    ///
    /// When powman resets the RSM, the current value of BOOTDIS_NEXT is OR'd into BOOTDIS_NOW, and BOOTDIS_NEXT is cleared.
    /// The bootrom checks this flag before reading the BOOT0..3 registers. If it is set, the bootrom clears it, and ignores the BOOT registers. This prevents Secure software from diverting the boot path before a bootloader has had the chance to soft lock OTP pages containing sensitive data.
    #[inline(always)]
    pub fn now(&self) -> bool {
        (self.value.read_bits() & Self::NOW_MASK) != 0
    }

    /// Set NOW bit atomically (RW)
    #[inline(always)]
    pub fn set_now(&self) {
        self.set_bits(Self::NOW_MASK);
    }

    /// Clear NOW bit atomically (RW)
    #[inline(always)]
    pub fn clr_now(&self) {
        self.clr_bits(Self::NOW_MASK);
    }

}

/// BOOTDIS value type for batch read/modify/write operations
///
/// Use `bootdis.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootdis.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BootdisVal {
    bits: u32,
}

impl BootdisVal {
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

    /// Read **NEXT** `[1:1]`
    ///
    /// This flag always ORs writes into its current contents. It can be set but not cleared by software.
    /// The BOOTDIS_NEXT bit is OR'd into the BOOTDIS_NOW bit when the core is powered down. Simultaneously, the BOOTDIS_NEXT bit is cleared. Setting this bit means that the BOOT0..3 registers will be ignored following the next reset of the RSM by powman.
    /// This flag should be set by an early boot stage that has soft-locked OTP pages, to prevent later stages from unlocking it by power cycling.
    #[inline(always)]
    pub const fn next(&self) -> bool {
        (self.bits & Bootdis::NEXT_MASK) != 0
    }

    /// Read **NOW** `[0:0]`
    ///
    /// When powman resets the RSM, the current value of BOOTDIS_NEXT is OR'd into BOOTDIS_NOW, and BOOTDIS_NEXT is cleared.
    /// The bootrom checks this flag before reading the BOOT0..3 registers. If it is set, the bootrom clears it, and ignores the BOOT registers. This prevents Secure software from diverting the boot path before a bootloader has had the chance to soft lock OTP pages containing sensitive data.
    #[inline(always)]
    pub const fn now(&self) -> bool {
        (self.bits & Bootdis::NOW_MASK) != 0
    }

    /// Set **NEXT** `[1:1]`
    ///
    /// This flag always ORs writes into its current contents. It can be set but not cleared by software.
    /// The BOOTDIS_NEXT bit is OR'd into the BOOTDIS_NOW bit when the core is powered down. Simultaneously, the BOOTDIS_NEXT bit is cleared. Setting this bit means that the BOOT0..3 registers will be ignored following the next reset of the RSM by powman.
    /// This flag should be set by an early boot stage that has soft-locked OTP pages, to prevent later stages from unlocking it by power cycling.
    #[inline(always)]
    pub const fn with_next(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bootdis::NEXT_MASK
            } else {
                self.bits & !Bootdis::NEXT_MASK
            }
        }
    }

    /// Set **NOW** `[0:0]`
    ///
    /// When powman resets the RSM, the current value of BOOTDIS_NEXT is OR'd into BOOTDIS_NOW, and BOOTDIS_NEXT is cleared.
    /// The bootrom checks this flag before reading the BOOT0..3 registers. If it is set, the bootrom clears it, and ignores the BOOT registers. This prevents Secure software from diverting the boot path before a bootloader has had the chance to soft lock OTP pages containing sensitive data.
    #[inline(always)]
    pub const fn with_now(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Bootdis::NOW_MASK
            } else {
                self.bits & !Bootdis::NOW_MASK
            }
        }
    }

}

impl Default for BootdisVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Bootdis {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> BootdisVal {
        BootdisVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: BootdisVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(BootdisVal) -> BootdisVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// DBGCONFIG Register
///
/// Address offset: 0x00ac
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **DP_INSTID** `[3:0]` (RW)
///
///   Configure DP instance ID for SWD multidrop selection.
///   Recommend that this is NOT changed until you require debug access in multi-chip environment
#[repr(transparent)]
pub struct Dbgconfig {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Dbgconfig {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Dbgconfig {
    // ======== Field Constants ========

    /// **DP_INSTID**
    ///
    /// Configure DP instance ID for SWD multidrop selection.
    /// Recommend that this is NOT changed until you require debug access in multi-chip environment
    ///
    /// Bits `[3:0]`, mask `0x0000000f`
    pub const DP_INSTID_OFFSET: u32 = 0;
    pub const DP_INSTID_MASK: u32 = 0x0000000f;

    // ======== Field Accessors ========

    /// **DP_INSTID** `[3:0]` (RW)
    ///
    /// Configure DP instance ID for SWD multidrop selection.
    /// Recommend that this is NOT changed until you require debug access in multi-chip environment
    #[inline(always)]
    pub fn dp_instid(&self) -> u32 {
        (self.value.read_bits() & Self::DP_INSTID_MASK) >> Self::DP_INSTID_OFFSET
    }

    /// Write **DP_INSTID** `[3:0]` (RW)
    ///
    /// Configure DP instance ID for SWD multidrop selection.
    /// Recommend that this is NOT changed until you require debug access in multi-chip environment
    #[inline(always)]
    pub fn write_dp_instid(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::DP_INSTID_MASK) | ((value << Self::DP_INSTID_OFFSET) & Self::DP_INSTID_MASK));
    }

}

/// DBGCONFIG value type for batch read/modify/write operations
///
/// Use `dbgconfig.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dbgconfig.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DbgconfigVal {
    bits: u32,
}

impl DbgconfigVal {
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

    /// Read **DP_INSTID** `[3:0]`
    ///
    /// Configure DP instance ID for SWD multidrop selection.
    /// Recommend that this is NOT changed until you require debug access in multi-chip environment
    #[inline(always)]
    pub const fn dp_instid(&self) -> u32 {
        (self.bits & Dbgconfig::DP_INSTID_MASK) >> Dbgconfig::DP_INSTID_OFFSET
    }

    /// Set **DP_INSTID** `[3:0]`
    ///
    /// Configure DP instance ID for SWD multidrop selection.
    /// Recommend that this is NOT changed until you require debug access in multi-chip environment
    #[inline(always)]
    pub const fn with_dp_instid(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Dbgconfig::DP_INSTID_MASK) | ((value << Dbgconfig::DP_INSTID_OFFSET) & Dbgconfig::DP_INSTID_MASK)
        }
    }

}

impl Default for DbgconfigVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Dbgconfig {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DbgconfigVal {
        DbgconfigVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DbgconfigVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DbgconfigVal) -> DbgconfigVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SCRATCH0 Register
///
/// Scratch register. Information persists in low power mode
///
/// Address offset: 0x00b0
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SCRATCH0** `[31:0]` (RW)
#[repr(transparent)]
pub struct Scratch0 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Scratch0 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Scratch0 {
    // ======== Field Constants ========

    /// **SCRATCH0**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SCRATCH0_OFFSET: u32 = 0;
    pub const SCRATCH0_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SCRATCH0** `[31:0]` (RW)
    #[inline(always)]
    pub fn scratch0(&self) -> u32 {
        (self.value.read_bits() & Self::SCRATCH0_MASK) >> Self::SCRATCH0_OFFSET
    }

    /// Write **SCRATCH0** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_scratch0(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SCRATCH0_MASK) | ((value << Self::SCRATCH0_OFFSET) & Self::SCRATCH0_MASK));
    }

}

/// SCRATCH0 value type for batch read/modify/write operations
///
/// Use `scratch0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `scratch0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Scratch0Val {
    bits: u32,
}

impl Scratch0Val {
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

    /// Read **SCRATCH0** `[31:0]`
    #[inline(always)]
    pub const fn scratch0(&self) -> u32 {
        (self.bits & Scratch0::SCRATCH0_MASK) >> Scratch0::SCRATCH0_OFFSET
    }

    /// Set **SCRATCH0** `[31:0]`
    #[inline(always)]
    pub const fn with_scratch0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Scratch0::SCRATCH0_MASK) | ((value << Scratch0::SCRATCH0_OFFSET) & Scratch0::SCRATCH0_MASK)
        }
    }

}

impl Default for Scratch0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Scratch0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Scratch0Val {
        Scratch0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Scratch0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Scratch0Val) -> Scratch0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SCRATCH1 Register
///
/// Scratch register. Information persists in low power mode
///
/// Address offset: 0x00b4
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SCRATCH1** `[31:0]` (RW)
#[repr(transparent)]
pub struct Scratch1 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Scratch1 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Scratch1 {
    // ======== Field Constants ========

    /// **SCRATCH1**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SCRATCH1_OFFSET: u32 = 0;
    pub const SCRATCH1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SCRATCH1** `[31:0]` (RW)
    #[inline(always)]
    pub fn scratch1(&self) -> u32 {
        (self.value.read_bits() & Self::SCRATCH1_MASK) >> Self::SCRATCH1_OFFSET
    }

    /// Write **SCRATCH1** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_scratch1(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SCRATCH1_MASK) | ((value << Self::SCRATCH1_OFFSET) & Self::SCRATCH1_MASK));
    }

}

/// SCRATCH1 value type for batch read/modify/write operations
///
/// Use `scratch1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `scratch1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Scratch1Val {
    bits: u32,
}

impl Scratch1Val {
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

    /// Read **SCRATCH1** `[31:0]`
    #[inline(always)]
    pub const fn scratch1(&self) -> u32 {
        (self.bits & Scratch1::SCRATCH1_MASK) >> Scratch1::SCRATCH1_OFFSET
    }

    /// Set **SCRATCH1** `[31:0]`
    #[inline(always)]
    pub const fn with_scratch1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Scratch1::SCRATCH1_MASK) | ((value << Scratch1::SCRATCH1_OFFSET) & Scratch1::SCRATCH1_MASK)
        }
    }

}

impl Default for Scratch1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Scratch1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Scratch1Val {
        Scratch1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Scratch1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Scratch1Val) -> Scratch1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SCRATCH2 Register
///
/// Scratch register. Information persists in low power mode
///
/// Address offset: 0x00b8
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SCRATCH2** `[31:0]` (RW)
#[repr(transparent)]
pub struct Scratch2 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Scratch2 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Scratch2 {
    // ======== Field Constants ========

    /// **SCRATCH2**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SCRATCH2_OFFSET: u32 = 0;
    pub const SCRATCH2_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SCRATCH2** `[31:0]` (RW)
    #[inline(always)]
    pub fn scratch2(&self) -> u32 {
        (self.value.read_bits() & Self::SCRATCH2_MASK) >> Self::SCRATCH2_OFFSET
    }

    /// Write **SCRATCH2** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_scratch2(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SCRATCH2_MASK) | ((value << Self::SCRATCH2_OFFSET) & Self::SCRATCH2_MASK));
    }

}

/// SCRATCH2 value type for batch read/modify/write operations
///
/// Use `scratch2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `scratch2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Scratch2Val {
    bits: u32,
}

impl Scratch2Val {
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

    /// Read **SCRATCH2** `[31:0]`
    #[inline(always)]
    pub const fn scratch2(&self) -> u32 {
        (self.bits & Scratch2::SCRATCH2_MASK) >> Scratch2::SCRATCH2_OFFSET
    }

    /// Set **SCRATCH2** `[31:0]`
    #[inline(always)]
    pub const fn with_scratch2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Scratch2::SCRATCH2_MASK) | ((value << Scratch2::SCRATCH2_OFFSET) & Scratch2::SCRATCH2_MASK)
        }
    }

}

impl Default for Scratch2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Scratch2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Scratch2Val {
        Scratch2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Scratch2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Scratch2Val) -> Scratch2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SCRATCH3 Register
///
/// Scratch register. Information persists in low power mode
///
/// Address offset: 0x00bc
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SCRATCH3** `[31:0]` (RW)
#[repr(transparent)]
pub struct Scratch3 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Scratch3 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Scratch3 {
    // ======== Field Constants ========

    /// **SCRATCH3**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SCRATCH3_OFFSET: u32 = 0;
    pub const SCRATCH3_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SCRATCH3** `[31:0]` (RW)
    #[inline(always)]
    pub fn scratch3(&self) -> u32 {
        (self.value.read_bits() & Self::SCRATCH3_MASK) >> Self::SCRATCH3_OFFSET
    }

    /// Write **SCRATCH3** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_scratch3(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SCRATCH3_MASK) | ((value << Self::SCRATCH3_OFFSET) & Self::SCRATCH3_MASK));
    }

}

/// SCRATCH3 value type for batch read/modify/write operations
///
/// Use `scratch3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `scratch3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Scratch3Val {
    bits: u32,
}

impl Scratch3Val {
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

    /// Read **SCRATCH3** `[31:0]`
    #[inline(always)]
    pub const fn scratch3(&self) -> u32 {
        (self.bits & Scratch3::SCRATCH3_MASK) >> Scratch3::SCRATCH3_OFFSET
    }

    /// Set **SCRATCH3** `[31:0]`
    #[inline(always)]
    pub const fn with_scratch3(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Scratch3::SCRATCH3_MASK) | ((value << Scratch3::SCRATCH3_OFFSET) & Scratch3::SCRATCH3_MASK)
        }
    }

}

impl Default for Scratch3Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Scratch3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Scratch3Val {
        Scratch3Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Scratch3Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Scratch3Val) -> Scratch3Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SCRATCH4 Register
///
/// Scratch register. Information persists in low power mode
///
/// Address offset: 0x00c0
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SCRATCH4** `[31:0]` (RW)
#[repr(transparent)]
pub struct Scratch4 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Scratch4 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Scratch4 {
    // ======== Field Constants ========

    /// **SCRATCH4**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SCRATCH4_OFFSET: u32 = 0;
    pub const SCRATCH4_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SCRATCH4** `[31:0]` (RW)
    #[inline(always)]
    pub fn scratch4(&self) -> u32 {
        (self.value.read_bits() & Self::SCRATCH4_MASK) >> Self::SCRATCH4_OFFSET
    }

    /// Write **SCRATCH4** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_scratch4(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SCRATCH4_MASK) | ((value << Self::SCRATCH4_OFFSET) & Self::SCRATCH4_MASK));
    }

}

/// SCRATCH4 value type for batch read/modify/write operations
///
/// Use `scratch4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `scratch4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Scratch4Val {
    bits: u32,
}

impl Scratch4Val {
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

    /// Read **SCRATCH4** `[31:0]`
    #[inline(always)]
    pub const fn scratch4(&self) -> u32 {
        (self.bits & Scratch4::SCRATCH4_MASK) >> Scratch4::SCRATCH4_OFFSET
    }

    /// Set **SCRATCH4** `[31:0]`
    #[inline(always)]
    pub const fn with_scratch4(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Scratch4::SCRATCH4_MASK) | ((value << Scratch4::SCRATCH4_OFFSET) & Scratch4::SCRATCH4_MASK)
        }
    }

}

impl Default for Scratch4Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Scratch4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Scratch4Val {
        Scratch4Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Scratch4Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Scratch4Val) -> Scratch4Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SCRATCH5 Register
///
/// Scratch register. Information persists in low power mode
///
/// Address offset: 0x00c4
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SCRATCH5** `[31:0]` (RW)
#[repr(transparent)]
pub struct Scratch5 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Scratch5 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Scratch5 {
    // ======== Field Constants ========

    /// **SCRATCH5**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SCRATCH5_OFFSET: u32 = 0;
    pub const SCRATCH5_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SCRATCH5** `[31:0]` (RW)
    #[inline(always)]
    pub fn scratch5(&self) -> u32 {
        (self.value.read_bits() & Self::SCRATCH5_MASK) >> Self::SCRATCH5_OFFSET
    }

    /// Write **SCRATCH5** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_scratch5(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SCRATCH5_MASK) | ((value << Self::SCRATCH5_OFFSET) & Self::SCRATCH5_MASK));
    }

}

/// SCRATCH5 value type for batch read/modify/write operations
///
/// Use `scratch5.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `scratch5.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Scratch5Val {
    bits: u32,
}

impl Scratch5Val {
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

    /// Read **SCRATCH5** `[31:0]`
    #[inline(always)]
    pub const fn scratch5(&self) -> u32 {
        (self.bits & Scratch5::SCRATCH5_MASK) >> Scratch5::SCRATCH5_OFFSET
    }

    /// Set **SCRATCH5** `[31:0]`
    #[inline(always)]
    pub const fn with_scratch5(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Scratch5::SCRATCH5_MASK) | ((value << Scratch5::SCRATCH5_OFFSET) & Scratch5::SCRATCH5_MASK)
        }
    }

}

impl Default for Scratch5Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Scratch5 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Scratch5Val {
        Scratch5Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Scratch5Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Scratch5Val) -> Scratch5Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SCRATCH6 Register
///
/// Scratch register. Information persists in low power mode
///
/// Address offset: 0x00c8
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SCRATCH6** `[31:0]` (RW)
#[repr(transparent)]
pub struct Scratch6 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Scratch6 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Scratch6 {
    // ======== Field Constants ========

    /// **SCRATCH6**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SCRATCH6_OFFSET: u32 = 0;
    pub const SCRATCH6_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SCRATCH6** `[31:0]` (RW)
    #[inline(always)]
    pub fn scratch6(&self) -> u32 {
        (self.value.read_bits() & Self::SCRATCH6_MASK) >> Self::SCRATCH6_OFFSET
    }

    /// Write **SCRATCH6** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_scratch6(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SCRATCH6_MASK) | ((value << Self::SCRATCH6_OFFSET) & Self::SCRATCH6_MASK));
    }

}

/// SCRATCH6 value type for batch read/modify/write operations
///
/// Use `scratch6.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `scratch6.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Scratch6Val {
    bits: u32,
}

impl Scratch6Val {
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

    /// Read **SCRATCH6** `[31:0]`
    #[inline(always)]
    pub const fn scratch6(&self) -> u32 {
        (self.bits & Scratch6::SCRATCH6_MASK) >> Scratch6::SCRATCH6_OFFSET
    }

    /// Set **SCRATCH6** `[31:0]`
    #[inline(always)]
    pub const fn with_scratch6(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Scratch6::SCRATCH6_MASK) | ((value << Scratch6::SCRATCH6_OFFSET) & Scratch6::SCRATCH6_MASK)
        }
    }

}

impl Default for Scratch6Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Scratch6 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Scratch6Val {
        Scratch6Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Scratch6Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Scratch6Val) -> Scratch6Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SCRATCH7 Register
///
/// Scratch register. Information persists in low power mode
///
/// Address offset: 0x00cc
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SCRATCH7** `[31:0]` (RW)
#[repr(transparent)]
pub struct Scratch7 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Scratch7 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Scratch7 {
    // ======== Field Constants ========

    /// **SCRATCH7**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SCRATCH7_OFFSET: u32 = 0;
    pub const SCRATCH7_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SCRATCH7** `[31:0]` (RW)
    #[inline(always)]
    pub fn scratch7(&self) -> u32 {
        (self.value.read_bits() & Self::SCRATCH7_MASK) >> Self::SCRATCH7_OFFSET
    }

    /// Write **SCRATCH7** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_scratch7(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::SCRATCH7_MASK) | ((value << Self::SCRATCH7_OFFSET) & Self::SCRATCH7_MASK));
    }

}

/// SCRATCH7 value type for batch read/modify/write operations
///
/// Use `scratch7.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `scratch7.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Scratch7Val {
    bits: u32,
}

impl Scratch7Val {
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

    /// Read **SCRATCH7** `[31:0]`
    #[inline(always)]
    pub const fn scratch7(&self) -> u32 {
        (self.bits & Scratch7::SCRATCH7_MASK) >> Scratch7::SCRATCH7_OFFSET
    }

    /// Set **SCRATCH7** `[31:0]`
    #[inline(always)]
    pub const fn with_scratch7(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Scratch7::SCRATCH7_MASK) | ((value << Scratch7::SCRATCH7_OFFSET) & Scratch7::SCRATCH7_MASK)
        }
    }

}

impl Default for Scratch7Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Scratch7 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Scratch7Val {
        Scratch7Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Scratch7Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Scratch7Val) -> Scratch7Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// BOOT0 Register
///
/// Scratch register. Information persists in low power mode
///
/// Address offset: 0x00d0
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **BOOT0** `[31:0]` (RW)
#[repr(transparent)]
pub struct Boot0 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Boot0 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Boot0 {
    // ======== Field Constants ========

    /// **BOOT0**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const BOOT0_OFFSET: u32 = 0;
    pub const BOOT0_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **BOOT0** `[31:0]` (RW)
    #[inline(always)]
    pub fn boot0(&self) -> u32 {
        (self.value.read_bits() & Self::BOOT0_MASK) >> Self::BOOT0_OFFSET
    }

    /// Write **BOOT0** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_boot0(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::BOOT0_MASK) | ((value << Self::BOOT0_OFFSET) & Self::BOOT0_MASK));
    }

}

/// BOOT0 value type for batch read/modify/write operations
///
/// Use `boot0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `boot0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Boot0Val {
    bits: u32,
}

impl Boot0Val {
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

    /// Read **BOOT0** `[31:0]`
    #[inline(always)]
    pub const fn boot0(&self) -> u32 {
        (self.bits & Boot0::BOOT0_MASK) >> Boot0::BOOT0_OFFSET
    }

    /// Set **BOOT0** `[31:0]`
    #[inline(always)]
    pub const fn with_boot0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Boot0::BOOT0_MASK) | ((value << Boot0::BOOT0_OFFSET) & Boot0::BOOT0_MASK)
        }
    }

}

impl Default for Boot0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Boot0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Boot0Val {
        Boot0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Boot0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Boot0Val) -> Boot0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// BOOT1 Register
///
/// Scratch register. Information persists in low power mode
///
/// Address offset: 0x00d4
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **BOOT1** `[31:0]` (RW)
#[repr(transparent)]
pub struct Boot1 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Boot1 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Boot1 {
    // ======== Field Constants ========

    /// **BOOT1**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const BOOT1_OFFSET: u32 = 0;
    pub const BOOT1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **BOOT1** `[31:0]` (RW)
    #[inline(always)]
    pub fn boot1(&self) -> u32 {
        (self.value.read_bits() & Self::BOOT1_MASK) >> Self::BOOT1_OFFSET
    }

    /// Write **BOOT1** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_boot1(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::BOOT1_MASK) | ((value << Self::BOOT1_OFFSET) & Self::BOOT1_MASK));
    }

}

/// BOOT1 value type for batch read/modify/write operations
///
/// Use `boot1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `boot1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Boot1Val {
    bits: u32,
}

impl Boot1Val {
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

    /// Read **BOOT1** `[31:0]`
    #[inline(always)]
    pub const fn boot1(&self) -> u32 {
        (self.bits & Boot1::BOOT1_MASK) >> Boot1::BOOT1_OFFSET
    }

    /// Set **BOOT1** `[31:0]`
    #[inline(always)]
    pub const fn with_boot1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Boot1::BOOT1_MASK) | ((value << Boot1::BOOT1_OFFSET) & Boot1::BOOT1_MASK)
        }
    }

}

impl Default for Boot1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Boot1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Boot1Val {
        Boot1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Boot1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Boot1Val) -> Boot1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// BOOT2 Register
///
/// Scratch register. Information persists in low power mode
///
/// Address offset: 0x00d8
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **BOOT2** `[31:0]` (RW)
#[repr(transparent)]
pub struct Boot2 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Boot2 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Boot2 {
    // ======== Field Constants ========

    /// **BOOT2**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const BOOT2_OFFSET: u32 = 0;
    pub const BOOT2_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **BOOT2** `[31:0]` (RW)
    #[inline(always)]
    pub fn boot2(&self) -> u32 {
        (self.value.read_bits() & Self::BOOT2_MASK) >> Self::BOOT2_OFFSET
    }

    /// Write **BOOT2** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_boot2(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::BOOT2_MASK) | ((value << Self::BOOT2_OFFSET) & Self::BOOT2_MASK));
    }

}

/// BOOT2 value type for batch read/modify/write operations
///
/// Use `boot2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `boot2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Boot2Val {
    bits: u32,
}

impl Boot2Val {
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

    /// Read **BOOT2** `[31:0]`
    #[inline(always)]
    pub const fn boot2(&self) -> u32 {
        (self.bits & Boot2::BOOT2_MASK) >> Boot2::BOOT2_OFFSET
    }

    /// Set **BOOT2** `[31:0]`
    #[inline(always)]
    pub const fn with_boot2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Boot2::BOOT2_MASK) | ((value << Boot2::BOOT2_OFFSET) & Boot2::BOOT2_MASK)
        }
    }

}

impl Default for Boot2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Boot2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Boot2Val {
        Boot2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Boot2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Boot2Val) -> Boot2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// BOOT3 Register
///
/// Scratch register. Information persists in low power mode
///
/// Address offset: 0x00dc
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **BOOT3** `[31:0]` (RW)
#[repr(transparent)]
pub struct Boot3 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Boot3 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Boot3 {
    // ======== Field Constants ========

    /// **BOOT3**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const BOOT3_OFFSET: u32 = 0;
    pub const BOOT3_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **BOOT3** `[31:0]` (RW)
    #[inline(always)]
    pub fn boot3(&self) -> u32 {
        (self.value.read_bits() & Self::BOOT3_MASK) >> Self::BOOT3_OFFSET
    }

    /// Write **BOOT3** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_boot3(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::BOOT3_MASK) | ((value << Self::BOOT3_OFFSET) & Self::BOOT3_MASK));
    }

}

/// BOOT3 value type for batch read/modify/write operations
///
/// Use `boot3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `boot3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Boot3Val {
    bits: u32,
}

impl Boot3Val {
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

    /// Read **BOOT3** `[31:0]`
    #[inline(always)]
    pub const fn boot3(&self) -> u32 {
        (self.bits & Boot3::BOOT3_MASK) >> Boot3::BOOT3_OFFSET
    }

    /// Set **BOOT3** `[31:0]`
    #[inline(always)]
    pub const fn with_boot3(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Boot3::BOOT3_MASK) | ((value << Boot3::BOOT3_OFFSET) & Boot3::BOOT3_MASK)
        }
    }

}

impl Default for Boot3Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Boot3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Boot3Val {
        Boot3Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Boot3Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Boot3Val) -> Boot3Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// INTR Register
///
/// Raw Interrupts
///
/// Address offset: 0x00e0
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **PWRUP_WHILE_WAITING** `[3]` (RO)
///
///   Source is state.pwrup_while_waiting
///
/// - **STATE_REQ_IGNORED** `[2]` (RO)
///
///   Source is state.req_ignored
///
/// - **TIMER** `[1]` (RO)
///
/// - **VREG_OUTPUT_LOW** `[0]` (RW)
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

    /// **PWRUP_WHILE_WAITING**
    ///
    /// Source is state.pwrup_while_waiting
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const PWRUP_WHILE_WAITING_OFFSET: u32 = 3;
    pub const PWRUP_WHILE_WAITING_MASK: u32 = 0x00000008;
    pub const PWRUP_WHILE_WAITING: u32 = 1 << 3;

    /// **STATE_REQ_IGNORED**
    ///
    /// Source is state.req_ignored
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const STATE_REQ_IGNORED_OFFSET: u32 = 2;
    pub const STATE_REQ_IGNORED_MASK: u32 = 0x00000004;
    pub const STATE_REQ_IGNORED: u32 = 1 << 2;

    /// **TIMER**
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TIMER_OFFSET: u32 = 1;
    pub const TIMER_MASK: u32 = 0x00000002;
    pub const TIMER: u32 = 1 << 1;

    /// **VREG_OUTPUT_LOW**
    /// Bits `[0:0]`, mask `0x00000001`
    pub const VREG_OUTPUT_LOW_OFFSET: u32 = 0;
    pub const VREG_OUTPUT_LOW_MASK: u32 = 0x00000001;
    pub const VREG_OUTPUT_LOW: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **PWRUP_WHILE_WAITING** `[3]` (RO)
    ///
    /// Source is state.pwrup_while_waiting
    #[inline(always)]
    pub fn pwrup_while_waiting(&self) -> bool {
        (self.value.read_bits() & Self::PWRUP_WHILE_WAITING_MASK) != 0
    }

    /// **STATE_REQ_IGNORED** `[2]` (RO)
    ///
    /// Source is state.req_ignored
    #[inline(always)]
    pub fn state_req_ignored(&self) -> bool {
        (self.value.read_bits() & Self::STATE_REQ_IGNORED_MASK) != 0
    }

    /// **TIMER** `[1]` (RO)
    #[inline(always)]
    pub fn timer(&self) -> bool {
        (self.value.read_bits() & Self::TIMER_MASK) != 0
    }

    /// **VREG_OUTPUT_LOW** `[0]` (RW)
    #[inline(always)]
    pub fn vreg_output_low(&self) -> bool {
        (self.value.read_bits() & Self::VREG_OUTPUT_LOW_MASK) != 0
    }

    /// Set VREG_OUTPUT_LOW bit atomically (RW)
    #[inline(always)]
    pub fn set_vreg_output_low(&self) {
        self.set_bits(Self::VREG_OUTPUT_LOW_MASK);
    }

    /// Clear VREG_OUTPUT_LOW bit atomically (RW)
    #[inline(always)]
    pub fn clr_vreg_output_low(&self) {
        self.clr_bits(Self::VREG_OUTPUT_LOW_MASK);
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

    /// Read **PWRUP_WHILE_WAITING** `[3:3]`
    ///
    /// Source is state.pwrup_while_waiting
    #[inline(always)]
    pub const fn pwrup_while_waiting(&self) -> bool {
        (self.bits & Intr::PWRUP_WHILE_WAITING_MASK) != 0
    }

    /// Read **STATE_REQ_IGNORED** `[2:2]`
    ///
    /// Source is state.req_ignored
    #[inline(always)]
    pub const fn state_req_ignored(&self) -> bool {
        (self.bits & Intr::STATE_REQ_IGNORED_MASK) != 0
    }

    /// Read **TIMER** `[1:1]`
    #[inline(always)]
    pub const fn timer(&self) -> bool {
        (self.bits & Intr::TIMER_MASK) != 0
    }

    /// Read **VREG_OUTPUT_LOW** `[0:0]`
    #[inline(always)]
    pub const fn vreg_output_low(&self) -> bool {
        (self.bits & Intr::VREG_OUTPUT_LOW_MASK) != 0
    }

    /// Set **VREG_OUTPUT_LOW** `[0:0]`
    #[inline(always)]
    pub const fn with_vreg_output_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intr::VREG_OUTPUT_LOW_MASK
            } else {
                self.bits & !Intr::VREG_OUTPUT_LOW_MASK
            }
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
/// Address offset: 0x00e4
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **PWRUP_WHILE_WAITING** `[3]` (RW)
///
///   Source is state.pwrup_while_waiting
///
/// - **STATE_REQ_IGNORED** `[2]` (RW)
///
///   Source is state.req_ignored
///
/// - **TIMER** `[1]` (RW)
///
/// - **VREG_OUTPUT_LOW** `[0]` (RW)
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

    /// **PWRUP_WHILE_WAITING**
    ///
    /// Source is state.pwrup_while_waiting
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const PWRUP_WHILE_WAITING_OFFSET: u32 = 3;
    pub const PWRUP_WHILE_WAITING_MASK: u32 = 0x00000008;
    pub const PWRUP_WHILE_WAITING: u32 = 1 << 3;

    /// **STATE_REQ_IGNORED**
    ///
    /// Source is state.req_ignored
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const STATE_REQ_IGNORED_OFFSET: u32 = 2;
    pub const STATE_REQ_IGNORED_MASK: u32 = 0x00000004;
    pub const STATE_REQ_IGNORED: u32 = 1 << 2;

    /// **TIMER**
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TIMER_OFFSET: u32 = 1;
    pub const TIMER_MASK: u32 = 0x00000002;
    pub const TIMER: u32 = 1 << 1;

    /// **VREG_OUTPUT_LOW**
    /// Bits `[0:0]`, mask `0x00000001`
    pub const VREG_OUTPUT_LOW_OFFSET: u32 = 0;
    pub const VREG_OUTPUT_LOW_MASK: u32 = 0x00000001;
    pub const VREG_OUTPUT_LOW: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **PWRUP_WHILE_WAITING** `[3]` (RW)
    ///
    /// Source is state.pwrup_while_waiting
    #[inline(always)]
    pub fn pwrup_while_waiting(&self) -> bool {
        (self.value.read_bits() & Self::PWRUP_WHILE_WAITING_MASK) != 0
    }

    /// Set PWRUP_WHILE_WAITING bit atomically (RW)
    #[inline(always)]
    pub fn set_pwrup_while_waiting(&self) {
        self.set_bits(Self::PWRUP_WHILE_WAITING_MASK);
    }

    /// Clear PWRUP_WHILE_WAITING bit atomically (RW)
    #[inline(always)]
    pub fn clr_pwrup_while_waiting(&self) {
        self.clr_bits(Self::PWRUP_WHILE_WAITING_MASK);
    }

    /// **STATE_REQ_IGNORED** `[2]` (RW)
    ///
    /// Source is state.req_ignored
    #[inline(always)]
    pub fn state_req_ignored(&self) -> bool {
        (self.value.read_bits() & Self::STATE_REQ_IGNORED_MASK) != 0
    }

    /// Set STATE_REQ_IGNORED bit atomically (RW)
    #[inline(always)]
    pub fn set_state_req_ignored(&self) {
        self.set_bits(Self::STATE_REQ_IGNORED_MASK);
    }

    /// Clear STATE_REQ_IGNORED bit atomically (RW)
    #[inline(always)]
    pub fn clr_state_req_ignored(&self) {
        self.clr_bits(Self::STATE_REQ_IGNORED_MASK);
    }

    /// **TIMER** `[1]` (RW)
    #[inline(always)]
    pub fn timer(&self) -> bool {
        (self.value.read_bits() & Self::TIMER_MASK) != 0
    }

    /// Set TIMER bit atomically (RW)
    #[inline(always)]
    pub fn set_timer(&self) {
        self.set_bits(Self::TIMER_MASK);
    }

    /// Clear TIMER bit atomically (RW)
    #[inline(always)]
    pub fn clr_timer(&self) {
        self.clr_bits(Self::TIMER_MASK);
    }

    /// **VREG_OUTPUT_LOW** `[0]` (RW)
    #[inline(always)]
    pub fn vreg_output_low(&self) -> bool {
        (self.value.read_bits() & Self::VREG_OUTPUT_LOW_MASK) != 0
    }

    /// Set VREG_OUTPUT_LOW bit atomically (RW)
    #[inline(always)]
    pub fn set_vreg_output_low(&self) {
        self.set_bits(Self::VREG_OUTPUT_LOW_MASK);
    }

    /// Clear VREG_OUTPUT_LOW bit atomically (RW)
    #[inline(always)]
    pub fn clr_vreg_output_low(&self) {
        self.clr_bits(Self::VREG_OUTPUT_LOW_MASK);
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

    /// Read **PWRUP_WHILE_WAITING** `[3:3]`
    ///
    /// Source is state.pwrup_while_waiting
    #[inline(always)]
    pub const fn pwrup_while_waiting(&self) -> bool {
        (self.bits & Inte::PWRUP_WHILE_WAITING_MASK) != 0
    }

    /// Read **STATE_REQ_IGNORED** `[2:2]`
    ///
    /// Source is state.req_ignored
    #[inline(always)]
    pub const fn state_req_ignored(&self) -> bool {
        (self.bits & Inte::STATE_REQ_IGNORED_MASK) != 0
    }

    /// Read **TIMER** `[1:1]`
    #[inline(always)]
    pub const fn timer(&self) -> bool {
        (self.bits & Inte::TIMER_MASK) != 0
    }

    /// Read **VREG_OUTPUT_LOW** `[0:0]`
    #[inline(always)]
    pub const fn vreg_output_low(&self) -> bool {
        (self.bits & Inte::VREG_OUTPUT_LOW_MASK) != 0
    }

    /// Set **PWRUP_WHILE_WAITING** `[3:3]`
    ///
    /// Source is state.pwrup_while_waiting
    #[inline(always)]
    pub const fn with_pwrup_while_waiting(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Inte::PWRUP_WHILE_WAITING_MASK
            } else {
                self.bits & !Inte::PWRUP_WHILE_WAITING_MASK
            }
        }
    }

    /// Set **STATE_REQ_IGNORED** `[2:2]`
    ///
    /// Source is state.req_ignored
    #[inline(always)]
    pub const fn with_state_req_ignored(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Inte::STATE_REQ_IGNORED_MASK
            } else {
                self.bits & !Inte::STATE_REQ_IGNORED_MASK
            }
        }
    }

    /// Set **TIMER** `[1:1]`
    #[inline(always)]
    pub const fn with_timer(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Inte::TIMER_MASK
            } else {
                self.bits & !Inte::TIMER_MASK
            }
        }
    }

    /// Set **VREG_OUTPUT_LOW** `[0:0]`
    #[inline(always)]
    pub const fn with_vreg_output_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Inte::VREG_OUTPUT_LOW_MASK
            } else {
                self.bits & !Inte::VREG_OUTPUT_LOW_MASK
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
/// Address offset: 0x00e8
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **PWRUP_WHILE_WAITING** `[3]` (RW)
///
///   Source is state.pwrup_while_waiting
///
/// - **STATE_REQ_IGNORED** `[2]` (RW)
///
///   Source is state.req_ignored
///
/// - **TIMER** `[1]` (RW)
///
/// - **VREG_OUTPUT_LOW** `[0]` (RW)
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

    /// **PWRUP_WHILE_WAITING**
    ///
    /// Source is state.pwrup_while_waiting
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const PWRUP_WHILE_WAITING_OFFSET: u32 = 3;
    pub const PWRUP_WHILE_WAITING_MASK: u32 = 0x00000008;
    pub const PWRUP_WHILE_WAITING: u32 = 1 << 3;

    /// **STATE_REQ_IGNORED**
    ///
    /// Source is state.req_ignored
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const STATE_REQ_IGNORED_OFFSET: u32 = 2;
    pub const STATE_REQ_IGNORED_MASK: u32 = 0x00000004;
    pub const STATE_REQ_IGNORED: u32 = 1 << 2;

    /// **TIMER**
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TIMER_OFFSET: u32 = 1;
    pub const TIMER_MASK: u32 = 0x00000002;
    pub const TIMER: u32 = 1 << 1;

    /// **VREG_OUTPUT_LOW**
    /// Bits `[0:0]`, mask `0x00000001`
    pub const VREG_OUTPUT_LOW_OFFSET: u32 = 0;
    pub const VREG_OUTPUT_LOW_MASK: u32 = 0x00000001;
    pub const VREG_OUTPUT_LOW: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **PWRUP_WHILE_WAITING** `[3]` (RW)
    ///
    /// Source is state.pwrup_while_waiting
    #[inline(always)]
    pub fn pwrup_while_waiting(&self) -> bool {
        (self.value.read_bits() & Self::PWRUP_WHILE_WAITING_MASK) != 0
    }

    /// Set PWRUP_WHILE_WAITING bit atomically (RW)
    #[inline(always)]
    pub fn set_pwrup_while_waiting(&self) {
        self.set_bits(Self::PWRUP_WHILE_WAITING_MASK);
    }

    /// Clear PWRUP_WHILE_WAITING bit atomically (RW)
    #[inline(always)]
    pub fn clr_pwrup_while_waiting(&self) {
        self.clr_bits(Self::PWRUP_WHILE_WAITING_MASK);
    }

    /// **STATE_REQ_IGNORED** `[2]` (RW)
    ///
    /// Source is state.req_ignored
    #[inline(always)]
    pub fn state_req_ignored(&self) -> bool {
        (self.value.read_bits() & Self::STATE_REQ_IGNORED_MASK) != 0
    }

    /// Set STATE_REQ_IGNORED bit atomically (RW)
    #[inline(always)]
    pub fn set_state_req_ignored(&self) {
        self.set_bits(Self::STATE_REQ_IGNORED_MASK);
    }

    /// Clear STATE_REQ_IGNORED bit atomically (RW)
    #[inline(always)]
    pub fn clr_state_req_ignored(&self) {
        self.clr_bits(Self::STATE_REQ_IGNORED_MASK);
    }

    /// **TIMER** `[1]` (RW)
    #[inline(always)]
    pub fn timer(&self) -> bool {
        (self.value.read_bits() & Self::TIMER_MASK) != 0
    }

    /// Set TIMER bit atomically (RW)
    #[inline(always)]
    pub fn set_timer(&self) {
        self.set_bits(Self::TIMER_MASK);
    }

    /// Clear TIMER bit atomically (RW)
    #[inline(always)]
    pub fn clr_timer(&self) {
        self.clr_bits(Self::TIMER_MASK);
    }

    /// **VREG_OUTPUT_LOW** `[0]` (RW)
    #[inline(always)]
    pub fn vreg_output_low(&self) -> bool {
        (self.value.read_bits() & Self::VREG_OUTPUT_LOW_MASK) != 0
    }

    /// Set VREG_OUTPUT_LOW bit atomically (RW)
    #[inline(always)]
    pub fn set_vreg_output_low(&self) {
        self.set_bits(Self::VREG_OUTPUT_LOW_MASK);
    }

    /// Clear VREG_OUTPUT_LOW bit atomically (RW)
    #[inline(always)]
    pub fn clr_vreg_output_low(&self) {
        self.clr_bits(Self::VREG_OUTPUT_LOW_MASK);
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

    /// Read **PWRUP_WHILE_WAITING** `[3:3]`
    ///
    /// Source is state.pwrup_while_waiting
    #[inline(always)]
    pub const fn pwrup_while_waiting(&self) -> bool {
        (self.bits & Intf::PWRUP_WHILE_WAITING_MASK) != 0
    }

    /// Read **STATE_REQ_IGNORED** `[2:2]`
    ///
    /// Source is state.req_ignored
    #[inline(always)]
    pub const fn state_req_ignored(&self) -> bool {
        (self.bits & Intf::STATE_REQ_IGNORED_MASK) != 0
    }

    /// Read **TIMER** `[1:1]`
    #[inline(always)]
    pub const fn timer(&self) -> bool {
        (self.bits & Intf::TIMER_MASK) != 0
    }

    /// Read **VREG_OUTPUT_LOW** `[0:0]`
    #[inline(always)]
    pub const fn vreg_output_low(&self) -> bool {
        (self.bits & Intf::VREG_OUTPUT_LOW_MASK) != 0
    }

    /// Set **PWRUP_WHILE_WAITING** `[3:3]`
    ///
    /// Source is state.pwrup_while_waiting
    #[inline(always)]
    pub const fn with_pwrup_while_waiting(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intf::PWRUP_WHILE_WAITING_MASK
            } else {
                self.bits & !Intf::PWRUP_WHILE_WAITING_MASK
            }
        }
    }

    /// Set **STATE_REQ_IGNORED** `[2:2]`
    ///
    /// Source is state.req_ignored
    #[inline(always)]
    pub const fn with_state_req_ignored(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intf::STATE_REQ_IGNORED_MASK
            } else {
                self.bits & !Intf::STATE_REQ_IGNORED_MASK
            }
        }
    }

    /// Set **TIMER** `[1:1]`
    #[inline(always)]
    pub const fn with_timer(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intf::TIMER_MASK
            } else {
                self.bits & !Intf::TIMER_MASK
            }
        }
    }

    /// Set **VREG_OUTPUT_LOW** `[0:0]`
    #[inline(always)]
    pub const fn with_vreg_output_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intf::VREG_OUTPUT_LOW_MASK
            } else {
                self.bits & !Intf::VREG_OUTPUT_LOW_MASK
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
/// Address offset: 0x00ec
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **PWRUP_WHILE_WAITING** `[3]` (RO)
///
///   Source is state.pwrup_while_waiting
///
/// - **STATE_REQ_IGNORED** `[2]` (RO)
///
///   Source is state.req_ignored
///
/// - **TIMER** `[1]` (RO)
///
/// - **VREG_OUTPUT_LOW** `[0]` (RO)
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

    /// **PWRUP_WHILE_WAITING**
    ///
    /// Source is state.pwrup_while_waiting
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const PWRUP_WHILE_WAITING_OFFSET: u32 = 3;
    pub const PWRUP_WHILE_WAITING_MASK: u32 = 0x00000008;
    pub const PWRUP_WHILE_WAITING: u32 = 1 << 3;

    /// **STATE_REQ_IGNORED**
    ///
    /// Source is state.req_ignored
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const STATE_REQ_IGNORED_OFFSET: u32 = 2;
    pub const STATE_REQ_IGNORED_MASK: u32 = 0x00000004;
    pub const STATE_REQ_IGNORED: u32 = 1 << 2;

    /// **TIMER**
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TIMER_OFFSET: u32 = 1;
    pub const TIMER_MASK: u32 = 0x00000002;
    pub const TIMER: u32 = 1 << 1;

    /// **VREG_OUTPUT_LOW**
    /// Bits `[0:0]`, mask `0x00000001`
    pub const VREG_OUTPUT_LOW_OFFSET: u32 = 0;
    pub const VREG_OUTPUT_LOW_MASK: u32 = 0x00000001;
    pub const VREG_OUTPUT_LOW: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **PWRUP_WHILE_WAITING** `[3]` (RO)
    ///
    /// Source is state.pwrup_while_waiting
    #[inline(always)]
    pub fn pwrup_while_waiting(&self) -> bool {
        (self.value.read_bits() & Self::PWRUP_WHILE_WAITING_MASK) != 0
    }

    /// **STATE_REQ_IGNORED** `[2]` (RO)
    ///
    /// Source is state.req_ignored
    #[inline(always)]
    pub fn state_req_ignored(&self) -> bool {
        (self.value.read_bits() & Self::STATE_REQ_IGNORED_MASK) != 0
    }

    /// **TIMER** `[1]` (RO)
    #[inline(always)]
    pub fn timer(&self) -> bool {
        (self.value.read_bits() & Self::TIMER_MASK) != 0
    }

    /// **VREG_OUTPUT_LOW** `[0]` (RO)
    #[inline(always)]
    pub fn vreg_output_low(&self) -> bool {
        (self.value.read_bits() & Self::VREG_OUTPUT_LOW_MASK) != 0
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

    /// Read **PWRUP_WHILE_WAITING** `[3:3]`
    ///
    /// Source is state.pwrup_while_waiting
    #[inline(always)]
    pub const fn pwrup_while_waiting(&self) -> bool {
        (self.bits & Ints::PWRUP_WHILE_WAITING_MASK) != 0
    }

    /// Read **STATE_REQ_IGNORED** `[2:2]`
    ///
    /// Source is state.req_ignored
    #[inline(always)]
    pub const fn state_req_ignored(&self) -> bool {
        (self.bits & Ints::STATE_REQ_IGNORED_MASK) != 0
    }

    /// Read **TIMER** `[1:1]`
    #[inline(always)]
    pub const fn timer(&self) -> bool {
        (self.bits & Ints::TIMER_MASK) != 0
    }

    /// Read **VREG_OUTPUT_LOW** `[0:0]`
    #[inline(always)]
    pub const fn vreg_output_low(&self) -> bool {
        (self.bits & Ints::VREG_OUTPUT_LOW_MASK) != 0
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

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IntsVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IntsVal) -> IntsVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// Indicates a bad password has been used
    pub badpasswd: Badpasswd,
    /// 0x0004
    /// Voltage Regulator Control
    pub vreg_ctrl: VregCtrl,
    /// 0x0008
    /// Voltage Regulator Status
    pub vreg_sts: VregSts,
    /// 0x000c
    /// Voltage Regulator Settings
    pub vreg: Vreg,
    /// 0x0010
    /// Voltage Regulator Low Power Entry Settings
    pub vreg_lp_entry: VregLpEntry,
    /// 0x0014
    /// Voltage Regulator Low Power Exit Settings
    pub vreg_lp_exit: VregLpExit,
    /// 0x0018
    /// Brown-out Detection Control
    pub bod_ctrl: BodCtrl,
    /// 0x001c
    /// Brown-out Detection Settings
    pub bod: Bod,
    /// 0x0020
    /// Brown-out Detection Low Power Entry Settings
    pub bod_lp_entry: BodLpEntry,
    /// 0x0024
    /// Brown-out Detection Low Power Exit Settings
    pub bod_lp_exit: BodLpExit,
    /// 0x0028
    /// Low power oscillator control register.
    pub lposc: Lposc,
    /// 0x002c
    /// Chip reset control and status
    pub chip_reset: ChipReset,
    /// 0x0030
    /// Allows a watchdog reset to reset the internal state of powman in addition to the power-on state machine (PSM).
    /// Note that powman ignores watchdog resets that do not select at least the CLOCKS stage or earlier stages in the PSM. If using these bits, it's recommended to set PSM_WDSEL to all-ones in addition to the desired bits in this register. Failing to select CLOCKS or earlier will result in the POWMAN_WDSEL register having no effect.
    pub wdsel: Wdsel,
    /// 0x0034
    /// For configuration of the power sequencer
    /// Writes are ignored while POWMAN_STATE_CHANGING=1
    pub seq_cfg: SeqCfg,
    /// 0x0038
    /// This register controls the power state of the 4 power domains.
    /// The current power state is indicated in POWMAN_STATE_CURRENT which is read-only.
    /// To change the state, write to POWMAN_STATE_REQ.
    /// The coding of POWMAN_STATE_CURRENT & POWMAN_STATE_REQ corresponds to the power states
    /// defined in the datasheet:
    /// bit 3 = SWCORE
    /// bit 2 = XIP cache
    /// bit 1 = SRAM0
    /// bit 0 = SRAM1
    /// 0 = powered up
    /// 1 = powered down
    /// When POWMAN_STATE_REQ is written, the POWMAN_STATE_WAITING flag is set while the Power Manager determines what is required. If an invalid transition is requested the Power Manager will still register the request in POWMAN_STATE_REQ but will also set the POWMAN_BAD_REQ flag. It will then implement the power-up requests and ignore the power down requests. To do nothing would risk entering an unrecoverable lock-up state. Invalid requests are: any combination of power up and power down requests any request that results in swcore being powered and xip unpowered If the request is to power down the switched-core domain then POWMAN_STATE_WAITING stays active until the processors halt. During this time the POWMAN_STATE_REQ field can be re-written to change or cancel the request. When the power state transition begins the POWMAN_STATE_WAITING_flag is cleared, the POWMAN_STATE_CHANGING flag is set and POWMAN register writes are ignored until the transition completes.
    pub state: State,
    /// 0x003c
    pub pow_fastdiv: PowFastdiv,
    /// 0x0040
    /// power state machine delays
    pub pow_delay: PowDelay,
    /// 0x0044
    /// Configures a gpio as a power mode aware control output
    pub ext_ctrl0: ExtCtrl0,
    /// 0x0048
    /// Configures a gpio as a power mode aware control output
    pub ext_ctrl1: ExtCtrl1,
    /// 0x004c
    /// Select a GPIO to use as a time reference, the source can be used to drive the low power clock at 32kHz, or to provide a 1ms tick to the timer, or provide a 1Hz tick to the timer. The tick selection is controlled by the POWMAN_TIMER register.
    pub ext_time_ref: ExtTimeRef,
    /// 0x0050
    /// Informs the AON Timer of the integer component of the clock frequency when running off the LPOSC.
    pub lposc_freq_khz_int: LposcFreqKhzInt,
    /// 0x0054
    /// Informs the AON Timer of the fractional component of the clock frequency when running off the LPOSC.
    pub lposc_freq_khz_frac: LposcFreqKhzFrac,
    /// 0x0058
    /// Informs the AON Timer of the integer component of the clock frequency when running off the XOSC.
    pub xosc_freq_khz_int: XoscFreqKhzInt,
    /// 0x005c
    /// Informs the AON Timer of the fractional component of the clock frequency when running off the XOSC.
    pub xosc_freq_khz_frac: XoscFreqKhzFrac,
    /// 0x0060
    pub set_time_63to48: SetTime63to48,
    /// 0x0064
    pub set_time_47to32: SetTime47to32,
    /// 0x0068
    pub set_time_31to16: SetTime31to16,
    /// 0x006c
    pub set_time_15to0: SetTime15to0,
    /// 0x0070
    pub read_time_upper: ReadTimeUpper,
    /// 0x0074
    pub read_time_lower: ReadTimeLower,
    /// 0x0078
    pub alarm_time_63to48: AlarmTime63to48,
    /// 0x007c
    pub alarm_time_47to32: AlarmTime47to32,
    /// 0x0080
    pub alarm_time_31to16: AlarmTime31to16,
    /// 0x0084
    pub alarm_time_15to0: AlarmTime15to0,
    /// 0x0088
    pub timer: Timer,
    /// 0x008c
    /// 4 GPIO powerup events can be configured to wake the chip up from a low power state.
    /// The pwrups are level/edge sensitive and can be set to trigger on a high/rising or low/falling event
    /// The number of gpios available depends on the package option. An invalid selection will be ignored
    /// source = 0 selects gpio0
    /// .
    /// .
    /// source = 47 selects gpio47
    /// source = 48 selects qspi_ss
    /// source = 49 selects qspi_sd0
    /// source = 50 selects qspi_sd1
    /// source = 51 selects qspi_sd2
    /// source = 52 selects qspi_sd3
    /// source = 53 selects qspi_sclk
    /// level = 0 triggers the pwrup when the source is low
    /// level = 1 triggers the pwrup when the source is high
    pub pwrup0: Pwrup0,
    /// 0x0090
    /// 4 GPIO powerup events can be configured to wake the chip up from a low power state.
    /// The pwrups are level/edge sensitive and can be set to trigger on a high/rising or low/falling event
    /// The number of gpios available depends on the package option. An invalid selection will be ignored
    /// source = 0 selects gpio0
    /// .
    /// .
    /// source = 47 selects gpio47
    /// source = 48 selects qspi_ss
    /// source = 49 selects qspi_sd0
    /// source = 50 selects qspi_sd1
    /// source = 51 selects qspi_sd2
    /// source = 52 selects qspi_sd3
    /// source = 53 selects qspi_sclk
    /// level = 0 triggers the pwrup when the source is low
    /// level = 1 triggers the pwrup when the source is high
    pub pwrup1: Pwrup1,
    /// 0x0094
    /// 4 GPIO powerup events can be configured to wake the chip up from a low power state.
    /// The pwrups are level/edge sensitive and can be set to trigger on a high/rising or low/falling event
    /// The number of gpios available depends on the package option. An invalid selection will be ignored
    /// source = 0 selects gpio0
    /// .
    /// .
    /// source = 47 selects gpio47
    /// source = 48 selects qspi_ss
    /// source = 49 selects qspi_sd0
    /// source = 50 selects qspi_sd1
    /// source = 51 selects qspi_sd2
    /// source = 52 selects qspi_sd3
    /// source = 53 selects qspi_sclk
    /// level = 0 triggers the pwrup when the source is low
    /// level = 1 triggers the pwrup when the source is high
    pub pwrup2: Pwrup2,
    /// 0x0098
    /// 4 GPIO powerup events can be configured to wake the chip up from a low power state.
    /// The pwrups are level/edge sensitive and can be set to trigger on a high/rising or low/falling event
    /// The number of gpios available depends on the package option. An invalid selection will be ignored
    /// source = 0 selects gpio0
    /// .
    /// .
    /// source = 47 selects gpio47
    /// source = 48 selects qspi_ss
    /// source = 49 selects qspi_sd0
    /// source = 50 selects qspi_sd1
    /// source = 51 selects qspi_sd2
    /// source = 52 selects qspi_sd3
    /// source = 53 selects qspi_sclk
    /// level = 0 triggers the pwrup when the source is low
    /// level = 1 triggers the pwrup when the source is high
    pub pwrup3: Pwrup3,
    /// 0x009c
    /// Indicates current powerup request state
    /// pwrup events can be cleared by removing the enable from the pwrup register. The alarm pwrup req can be cleared by clearing timer.alarm_enab
    /// 0 = chip reset, for the source of the last reset see POWMAN_CHIP_RESET
    /// 1 = pwrup0
    /// 2 = pwrup1
    /// 3 = pwrup2
    /// 4 = pwrup3
    /// 5 = coresight_pwrup
    /// 6 = alarm_pwrup
    pub current_pwrup_req: CurrentPwrupReq,
    /// 0x00a0
    /// Indicates which pwrup source triggered the last switched-core power up
    /// 0 = chip reset, for the source of the last reset see POWMAN_CHIP_RESET
    /// 1 = pwrup0
    /// 2 = pwrup1
    /// 3 = pwrup2
    /// 4 = pwrup3
    /// 5 = coresight_pwrup
    /// 6 = alarm_pwrup
    pub last_swcore_pwrup: LastSwcorePwrup,
    /// 0x00a4
    pub dbg_pwrcfg: DbgPwrcfg,
    /// 0x00a8
    /// Tell the bootrom to ignore the BOOT0..3 registers following the next RSM reset (e.g. the next core power down/up).
    /// If an early boot stage has soft-locked some OTP pages in order to protect their contents from later stages, there is a risk that Secure code running at a later stage can unlock the pages by powering the core up and down.
    /// This register can be used to ensure that the bootloader runs as normal on the next power up, preventing Secure code at a later stage from accessing OTP in its unlocked state.
    /// Should be used in conjunction with the OTP BOOTDIS register.
    pub bootdis: Bootdis,
    /// 0x00ac
    pub dbgconfig: Dbgconfig,
    /// 0x00b0
    /// Scratch register. Information persists in low power mode
    pub scratch0: Scratch0,
    /// 0x00b4
    /// Scratch register. Information persists in low power mode
    pub scratch1: Scratch1,
    /// 0x00b8
    /// Scratch register. Information persists in low power mode
    pub scratch2: Scratch2,
    /// 0x00bc
    /// Scratch register. Information persists in low power mode
    pub scratch3: Scratch3,
    /// 0x00c0
    /// Scratch register. Information persists in low power mode
    pub scratch4: Scratch4,
    /// 0x00c4
    /// Scratch register. Information persists in low power mode
    pub scratch5: Scratch5,
    /// 0x00c8
    /// Scratch register. Information persists in low power mode
    pub scratch6: Scratch6,
    /// 0x00cc
    /// Scratch register. Information persists in low power mode
    pub scratch7: Scratch7,
    /// 0x00d0
    /// Scratch register. Information persists in low power mode
    pub boot0: Boot0,
    /// 0x00d4
    /// Scratch register. Information persists in low power mode
    pub boot1: Boot1,
    /// 0x00d8
    /// Scratch register. Information persists in low power mode
    pub boot2: Boot2,
    /// 0x00dc
    /// Scratch register. Information persists in low power mode
    pub boot3: Boot3,
    /// 0x00e0
    /// Raw Interrupts
    pub intr: Intr,
    /// 0x00e4
    /// Interrupt Enable
    pub inte: Inte,
    /// 0x00e8
    /// Interrupt Force
    pub intf: Intf,
    /// 0x00ec
    /// Interrupt status after masking & forcing
    pub ints: Ints,
}

