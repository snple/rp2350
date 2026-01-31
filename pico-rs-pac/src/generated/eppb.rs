//! EPPB
//! Cortex-M33 EPPB vendor register block for RP2350

#![allow(dead_code)]

pub const BASE: u32 = 0xe0080000;

/// NMI_MASK0 Register
///
/// NMI mask for IRQs 0 through 31. This register is core-local, and is reset by a processor warm reset.
///
/// Address offset: 0x0000
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NMI_MASK0** `[31:0]` (RW)
#[repr(transparent)]
pub struct NmiMask0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for NmiMask0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl NmiMask0 {
    // ======== Field Constants ========

    /// **NMI_MASK0**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const NMI_MASK0_OFFSET: u32 = 0;
    pub const NMI_MASK0_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **NMI_MASK0** `[31:0]` (RW)
    #[inline(always)]
    pub fn nmi_mask0(&self) -> u32 {
        (self.value.read_bits() & Self::NMI_MASK0_MASK) >> Self::NMI_MASK0_OFFSET
    }

    /// Write **NMI_MASK0** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_nmi_mask0(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::NMI_MASK0_MASK) | ((value << Self::NMI_MASK0_OFFSET) & Self::NMI_MASK0_MASK));
    }

}

/// NMI_MASK0 value type for batch read/modify/write operations
///
/// Use `nmimask0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `nmimask0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct NmiMask0Val {
    bits: u32,
}

impl NmiMask0Val {
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

    /// Read **NMI_MASK0** `[31:0]`
    #[inline(always)]
    pub const fn nmi_mask0(&self) -> u32 {
        (self.bits & NmiMask0::NMI_MASK0_MASK) >> NmiMask0::NMI_MASK0_OFFSET
    }

    /// Set **NMI_MASK0** `[31:0]`
    #[inline(always)]
    pub const fn with_nmi_mask0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !NmiMask0::NMI_MASK0_MASK) | ((value << NmiMask0::NMI_MASK0_OFFSET) & NmiMask0::NMI_MASK0_MASK)
        }
    }

}

impl Default for NmiMask0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl NmiMask0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> NmiMask0Val {
        NmiMask0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: NmiMask0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(NmiMask0Val) -> NmiMask0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// NMI_MASK1 Register
///
/// NMI mask for IRQs 0 though 51. This register is core-local, and is reset by a processor warm reset.
///
/// Address offset: 0x0004
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NMI_MASK1** `[19:0]` (RW)
#[repr(transparent)]
pub struct NmiMask1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for NmiMask1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl NmiMask1 {
    // ======== Field Constants ========

    /// **NMI_MASK1**
    /// Bits `[19:0]`, mask `0x000fffff`
    pub const NMI_MASK1_OFFSET: u32 = 0;
    pub const NMI_MASK1_MASK: u32 = 0x000fffff;

    // ======== Field Accessors ========

    /// **NMI_MASK1** `[19:0]` (RW)
    #[inline(always)]
    pub fn nmi_mask1(&self) -> u32 {
        (self.value.read_bits() & Self::NMI_MASK1_MASK) >> Self::NMI_MASK1_OFFSET
    }

    /// Write **NMI_MASK1** `[19:0]` (RW)
    #[inline(always)]
    pub fn write_nmi_mask1(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::NMI_MASK1_MASK) | ((value << Self::NMI_MASK1_OFFSET) & Self::NMI_MASK1_MASK));
    }

}

/// NMI_MASK1 value type for batch read/modify/write operations
///
/// Use `nmimask1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `nmimask1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct NmiMask1Val {
    bits: u32,
}

impl NmiMask1Val {
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

    /// Read **NMI_MASK1** `[19:0]`
    #[inline(always)]
    pub const fn nmi_mask1(&self) -> u32 {
        (self.bits & NmiMask1::NMI_MASK1_MASK) >> NmiMask1::NMI_MASK1_OFFSET
    }

    /// Set **NMI_MASK1** `[19:0]`
    #[inline(always)]
    pub const fn with_nmi_mask1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !NmiMask1::NMI_MASK1_MASK) | ((value << NmiMask1::NMI_MASK1_OFFSET) & NmiMask1::NMI_MASK1_MASK)
        }
    }

}

impl Default for NmiMask1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl NmiMask1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> NmiMask1Val {
        NmiMask1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: NmiMask1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(NmiMask1Val) -> NmiMask1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// SLEEPCTRL Register
///
/// Nonstandard sleep control register
///
/// Address offset: 0x0008
/// Reset value: 0x00000002
///
/// ## Fields
///
/// - **WICENACK** `[2]` (RO)
///
///   Status signal from the processor's interrupt controller. Changes to WICENREQ are eventually reflected in WICENACK.
///
/// - **WICENREQ** `[1]` (RW)
///
///   Request that the next processor deep sleep is a WIC sleep. After setting this bit, before sleeping, poll WICENACK to ensure the processor interrupt controller has acknowledged the change.
///
/// - **LIGHT_SLEEP** `[0]` (RW)
///
///   By default, any processor sleep will deassert the system-level clock request. Reenabling the clocks incurs 5 cycles of additional latency on wakeup.
///   Setting LIGHT_SLEEP to 1 keeps the clock request asserted during a normal sleep (Arm SCR.SLEEPDEEP = 0), for faster wakeup. Processor deep sleep (Arm SCR.SLEEPDEEP = 1) is not affected, and will always deassert the system-level clock request.
#[repr(transparent)]
pub struct Sleepctrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Sleepctrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sleepctrl {
    // ======== Field Constants ========

    /// **WICENACK**
    ///
    /// Status signal from the processor's interrupt controller. Changes to WICENREQ are eventually reflected in WICENACK.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const WICENACK_OFFSET: u32 = 2;
    pub const WICENACK_MASK: u32 = 0x00000004;
    pub const WICENACK: u32 = 1 << 2;

    /// **WICENREQ**
    ///
    /// Request that the next processor deep sleep is a WIC sleep. After setting this bit, before sleeping, poll WICENACK to ensure the processor interrupt controller has acknowledged the change.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const WICENREQ_OFFSET: u32 = 1;
    pub const WICENREQ_MASK: u32 = 0x00000002;
    pub const WICENREQ: u32 = 1 << 1;

    /// **LIGHT_SLEEP**
    ///
    /// By default, any processor sleep will deassert the system-level clock request. Reenabling the clocks incurs 5 cycles of additional latency on wakeup.
    /// Setting LIGHT_SLEEP to 1 keeps the clock request asserted during a normal sleep (Arm SCR.SLEEPDEEP = 0), for faster wakeup. Processor deep sleep (Arm SCR.SLEEPDEEP = 1) is not affected, and will always deassert the system-level clock request.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const LIGHT_SLEEP_OFFSET: u32 = 0;
    pub const LIGHT_SLEEP_MASK: u32 = 0x00000001;
    pub const LIGHT_SLEEP: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **WICENACK** `[2]` (RO)
    ///
    /// Status signal from the processor's interrupt controller. Changes to WICENREQ are eventually reflected in WICENACK.
    #[inline(always)]
    pub fn wicenack(&self) -> bool {
        (self.value.read_bits() & Self::WICENACK_MASK) != 0
    }

    /// **WICENREQ** `[1]` (RW)
    ///
    /// Request that the next processor deep sleep is a WIC sleep. After setting this bit, before sleeping, poll WICENACK to ensure the processor interrupt controller has acknowledged the change.
    #[inline(always)]
    pub fn wicenreq(&self) -> bool {
        (self.value.read_bits() & Self::WICENREQ_MASK) != 0
    }

    /// Set WICENREQ bit atomically (RW)
    #[inline(always)]
    pub fn set_wicenreq(&self) {
        self.value.set_bits(Self::WICENREQ_MASK);
    }

    /// Clear WICENREQ bit atomically (RW)
    #[inline(always)]
    pub fn clr_wicenreq(&self) {
        self.value.clr_bits(Self::WICENREQ_MASK);
    }

    /// **LIGHT_SLEEP** `[0]` (RW)
    ///
    /// By default, any processor sleep will deassert the system-level clock request. Reenabling the clocks incurs 5 cycles of additional latency on wakeup.
    /// Setting LIGHT_SLEEP to 1 keeps the clock request asserted during a normal sleep (Arm SCR.SLEEPDEEP = 0), for faster wakeup. Processor deep sleep (Arm SCR.SLEEPDEEP = 1) is not affected, and will always deassert the system-level clock request.
    #[inline(always)]
    pub fn light_sleep(&self) -> bool {
        (self.value.read_bits() & Self::LIGHT_SLEEP_MASK) != 0
    }

    /// Set LIGHT_SLEEP bit atomically (RW)
    #[inline(always)]
    pub fn set_light_sleep(&self) {
        self.value.set_bits(Self::LIGHT_SLEEP_MASK);
    }

    /// Clear LIGHT_SLEEP bit atomically (RW)
    #[inline(always)]
    pub fn clr_light_sleep(&self) {
        self.value.clr_bits(Self::LIGHT_SLEEP_MASK);
    }

}

/// SLEEPCTRL value type for batch read/modify/write operations
///
/// Use `sleepctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sleepctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SleepctrlVal {
    bits: u32,
}

impl SleepctrlVal {
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
        Self { bits: 0x00000002 }
    }

    /// Read **WICENACK** `[2:2]`
    ///
    /// Status signal from the processor's interrupt controller. Changes to WICENREQ are eventually reflected in WICENACK.
    #[inline(always)]
    pub const fn wicenack(&self) -> bool {
        (self.bits & Sleepctrl::WICENACK_MASK) != 0
    }

    /// Read **WICENREQ** `[1:1]`
    ///
    /// Request that the next processor deep sleep is a WIC sleep. After setting this bit, before sleeping, poll WICENACK to ensure the processor interrupt controller has acknowledged the change.
    #[inline(always)]
    pub const fn wicenreq(&self) -> bool {
        (self.bits & Sleepctrl::WICENREQ_MASK) != 0
    }

    /// Read **LIGHT_SLEEP** `[0:0]`
    ///
    /// By default, any processor sleep will deassert the system-level clock request. Reenabling the clocks incurs 5 cycles of additional latency on wakeup.
    /// Setting LIGHT_SLEEP to 1 keeps the clock request asserted during a normal sleep (Arm SCR.SLEEPDEEP = 0), for faster wakeup. Processor deep sleep (Arm SCR.SLEEPDEEP = 1) is not affected, and will always deassert the system-level clock request.
    #[inline(always)]
    pub const fn light_sleep(&self) -> bool {
        (self.bits & Sleepctrl::LIGHT_SLEEP_MASK) != 0
    }

    /// Set **WICENREQ** `[1:1]`
    ///
    /// Request that the next processor deep sleep is a WIC sleep. After setting this bit, before sleeping, poll WICENACK to ensure the processor interrupt controller has acknowledged the change.
    #[inline(always)]
    pub const fn with_wicenreq(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sleepctrl::WICENREQ_MASK
            } else {
                self.bits & !Sleepctrl::WICENREQ_MASK
            }
        }
    }

    /// Set **LIGHT_SLEEP** `[0:0]`
    ///
    /// By default, any processor sleep will deassert the system-level clock request. Reenabling the clocks incurs 5 cycles of additional latency on wakeup.
    /// Setting LIGHT_SLEEP to 1 keeps the clock request asserted during a normal sleep (Arm SCR.SLEEPDEEP = 0), for faster wakeup. Processor deep sleep (Arm SCR.SLEEPDEEP = 1) is not affected, and will always deassert the system-level clock request.
    #[inline(always)]
    pub const fn with_light_sleep(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sleepctrl::LIGHT_SLEEP_MASK
            } else {
                self.bits & !Sleepctrl::LIGHT_SLEEP_MASK
            }
        }
    }

}

impl Default for SleepctrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Sleepctrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SleepctrlVal {
        SleepctrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SleepctrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SleepctrlVal) -> SleepctrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// NMI mask for IRQs 0 through 31. This register is core-local, and is reset by a processor warm reset.
    pub nmi_mask0: NmiMask0,
    /// 0x0004
    /// NMI mask for IRQs 0 though 51. This register is core-local, and is reset by a processor warm reset.
    pub nmi_mask1: NmiMask1,
    /// 0x0008
    /// Nonstandard sleep control register
    pub sleepctrl: Sleepctrl,
}

