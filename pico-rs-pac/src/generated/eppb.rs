//! EPPB - Cortex-M33 EPPB vendor register block for RP2350

#![allow(dead_code)]

pub const BASE: u32 = 0xe0080000;

/// NMI_MASK0 Register
///
/// NMI mask for IRQs 0 through 31. This register is core-local, and is reset by
/// a processor warm reset.
///
/// Address offset: 0x0000
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NMI_MASK0** [31:0] (RW): 
#[repr(transparent)]
pub struct NmiMask0 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for NmiMask0 {}
unsafe impl Sync for NmiMask0 {}

impl NmiMask0 {
    // ======== Field Constants ========

    /// Bits [31:0], mask 0xffffffff
    pub const NMI_MASK0_OFFSET: u32 = 0;
    pub const NMI_MASK0_MASK: u32 = 0xffffffff;

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

    /// **NMI_MASK0** [31:0]
    #[inline(always)]
    pub fn nmi_mask0(&self) -> u32 {
        (self.read_bits() & Self::NMI_MASK0_MASK) >> Self::NMI_MASK0_OFFSET
    }

    /// Write NMI_MASK0 field
    #[inline(always)]
    pub fn write_nmi_mask0(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::NMI_MASK0_MASK) | ((value << Self::NMI_MASK0_OFFSET) & Self::NMI_MASK0_MASK));
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

    /// Read NMI_MASK0 [31:0]
    #[inline(always)]
    pub const fn nmi_mask0(&self) -> u32 {
        (self.bits & NmiMask0::NMI_MASK0_MASK) >> NmiMask0::NMI_MASK0_OFFSET
    }

    /// Set NMI_MASK0 [31:0]
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
    pub fn read(&self) -> NmiMask0Val {
        NmiMask0Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: NmiMask0Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(NmiMask0Val) -> NmiMask0Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// NMI_MASK1 Register
///
/// NMI mask for IRQs 0 though 51. This register is core-local, and is reset by
/// a processor warm reset.
///
/// Address offset: 0x0004
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **NMI_MASK1** [19:0] (RW): 
#[repr(transparent)]
pub struct NmiMask1 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for NmiMask1 {}
unsafe impl Sync for NmiMask1 {}

impl NmiMask1 {
    // ======== Field Constants ========

    /// Bits [19:0], mask 0x000fffff
    pub const NMI_MASK1_OFFSET: u32 = 0;
    pub const NMI_MASK1_MASK: u32 = 0x000fffff;

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

    /// **NMI_MASK1** [19:0]
    #[inline(always)]
    pub fn nmi_mask1(&self) -> u32 {
        (self.read_bits() & Self::NMI_MASK1_MASK) >> Self::NMI_MASK1_OFFSET
    }

    /// Write NMI_MASK1 field
    #[inline(always)]
    pub fn write_nmi_mask1(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::NMI_MASK1_MASK) | ((value << Self::NMI_MASK1_OFFSET) & Self::NMI_MASK1_MASK));
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

    /// Read NMI_MASK1 [19:0]
    #[inline(always)]
    pub const fn nmi_mask1(&self) -> u32 {
        (self.bits & NmiMask1::NMI_MASK1_MASK) >> NmiMask1::NMI_MASK1_OFFSET
    }

    /// Set NMI_MASK1 [19:0]
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
    pub fn read(&self) -> NmiMask1Val {
        NmiMask1Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: NmiMask1Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(NmiMask1Val) -> NmiMask1Val>(&self, f: F) {
        self.write(f(self.read()))
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
/// - **WICENACK** [2] (RO): Status signal from the processor's interrupt controller. Changes to WICENRE...
/// - **WICENREQ** [1] (RW): Request that the next processor deep sleep is a WIC sleep. After setting th...
/// - **LIGHT_SLEEP** [0] (RW): By default, any processor sleep will deassert the system-level clock reques...
#[repr(transparent)]
pub struct Sleepctrl {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for Sleepctrl {}
unsafe impl Sync for Sleepctrl {}

impl Sleepctrl {
    // ======== Field Constants ========

    /// WICENACK - Status signal from the processor's interrupt controller. Changes to WICENRE...
    /// Bits [2:2], mask 0x00000004
    pub const WICENACK_OFFSET: u32 = 2;
    pub const WICENACK_MASK: u32 = 0x00000004;
    pub const WICENACK: u32 = 1 << 2;

    /// WICENREQ - Request that the next processor deep sleep is a WIC sleep. After setting th...
    /// Bits [1:1], mask 0x00000002
    pub const WICENREQ_OFFSET: u32 = 1;
    pub const WICENREQ_MASK: u32 = 0x00000002;
    pub const WICENREQ: u32 = 1 << 1;

    /// LIGHT_SLEEP - By default, any processor sleep will deassert the system-level clock reques...
    /// Bits [0:0], mask 0x00000001
    pub const LIGHT_SLEEP_OFFSET: u32 = 0;
    pub const LIGHT_SLEEP_MASK: u32 = 0x00000001;
    pub const LIGHT_SLEEP: u32 = 1 << 0;

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

    /// **WICENACK** [2:2]: Status signal from the processor's interrupt controller. Changes to WICENRE...
    #[inline(always)]
    pub fn wicenack(&self) -> bool {
        (self.read_bits() & Self::WICENACK_MASK) != 0
    }

    /// **WICENREQ** [1:1]: Request that the next processor deep sleep is a WIC sleep. After setting th...
    #[inline(always)]
    pub fn wicenreq(&self) -> bool {
        (self.read_bits() & Self::WICENREQ_MASK) != 0
    }

    /// Set WICENREQ bit
    #[inline(always)]
    pub fn set_wicenreq(&self) {
        self.set_bits(Self::WICENREQ_MASK);
    }

    /// Clear WICENREQ bit
    #[inline(always)]
    pub fn clr_wicenreq(&self) {
        self.clr_bits(Self::WICENREQ_MASK);
    }

    /// **LIGHT_SLEEP** [0:0]: By default, any processor sleep will deassert the system-level clock reques...
    #[inline(always)]
    pub fn light_sleep(&self) -> bool {
        (self.read_bits() & Self::LIGHT_SLEEP_MASK) != 0
    }

    /// Set LIGHT_SLEEP bit
    #[inline(always)]
    pub fn set_light_sleep(&self) {
        self.set_bits(Self::LIGHT_SLEEP_MASK);
    }

    /// Clear LIGHT_SLEEP bit
    #[inline(always)]
    pub fn clr_light_sleep(&self) {
        self.clr_bits(Self::LIGHT_SLEEP_MASK);
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

    /// Read WICENACK [2:2]: Status signal from the processor's interrupt controller. Changes to WICENRE...
    #[inline(always)]
    pub const fn wicenack(&self) -> bool {
        (self.bits & Sleepctrl::WICENACK_MASK) != 0
    }

    /// Read WICENREQ [1:1]: Request that the next processor deep sleep is a WIC sleep. After setting th...
    #[inline(always)]
    pub const fn wicenreq(&self) -> bool {
        (self.bits & Sleepctrl::WICENREQ_MASK) != 0
    }

    /// Read LIGHT_SLEEP [0:0]: By default, any processor sleep will deassert the system-level clock reques...
    #[inline(always)]
    pub const fn light_sleep(&self) -> bool {
        (self.bits & Sleepctrl::LIGHT_SLEEP_MASK) != 0
    }

    /// Set WICENREQ [1:1]: Request that the next processor deep sleep is a WIC sleep. After setting th...
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

    /// Set LIGHT_SLEEP [0:0]: By default, any processor sleep will deassert the system-level clock reques...
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
    pub fn read(&self) -> SleepctrlVal {
        SleepctrlVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: SleepctrlVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(SleepctrlVal) -> SleepctrlVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000 - NMI mask for IRQs 0 through 31. This register is core-local, and is reset b...
    pub nmi_mask0: NmiMask0,
    /// 0x0004 - NMI mask for IRQs 0 though 51. This register is core-local, and is reset by...
    pub nmi_mask1: NmiMask1,
    /// 0x0008 - Nonstandard sleep control register
    pub sleepctrl: Sleepctrl,
}

