//! IO_QSPI - 

#![allow(dead_code)]

pub const BASE: u32 = 0x40030000;

/// USBPHY_DP_STATUS Register
///
/// Address offset: 0x0000
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **IRQTOPROC** [26] (RO): interrupt to processors, after override is applied
/// - **INFROMPAD** [17] (RO): input signal from pad, before filtering and override are applied
/// - **OETOPAD** [13] (RO): output enable to pad after register override is applied
/// - **OUTTOPAD** [9] (RO): output signal to pad after register override is applied
#[repr(transparent)]
pub struct UsbphyDpStatus {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for UsbphyDpStatus {}
unsafe impl Sync for UsbphyDpStatus {}

impl UsbphyDpStatus {
    // ======== Field Constants ========

    /// IRQTOPROC - interrupt to processors, after override is applied
    /// Bits [26:26], mask 0x04000000
    pub const IRQTOPROC_OFFSET: u32 = 26;
    pub const IRQTOPROC_MASK: u32 = 0x04000000;
    pub const IRQTOPROC: u32 = 1 << 26;

    /// INFROMPAD - input signal from pad, before filtering and override are applied
    /// Bits [17:17], mask 0x00020000
    pub const INFROMPAD_OFFSET: u32 = 17;
    pub const INFROMPAD_MASK: u32 = 0x00020000;
    pub const INFROMPAD: u32 = 1 << 17;

    /// OETOPAD - output enable to pad after register override is applied
    /// Bits [13:13], mask 0x00002000
    pub const OETOPAD_OFFSET: u32 = 13;
    pub const OETOPAD_MASK: u32 = 0x00002000;
    pub const OETOPAD: u32 = 1 << 13;

    /// OUTTOPAD - output signal to pad after register override is applied
    /// Bits [9:9], mask 0x00000200
    pub const OUTTOPAD_OFFSET: u32 = 9;
    pub const OUTTOPAD_MASK: u32 = 0x00000200;
    pub const OUTTOPAD: u32 = 1 << 9;

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

    /// **IRQTOPROC** [26:26]: interrupt to processors, after override is applied
    #[inline(always)]
    pub fn irqtoproc(&self) -> bool {
        (self.read_bits() & Self::IRQTOPROC_MASK) != 0
    }

    /// **INFROMPAD** [17:17]: input signal from pad, before filtering and override are applied
    #[inline(always)]
    pub fn infrompad(&self) -> bool {
        (self.read_bits() & Self::INFROMPAD_MASK) != 0
    }

    /// **OETOPAD** [13:13]: output enable to pad after register override is applied
    #[inline(always)]
    pub fn oetopad(&self) -> bool {
        (self.read_bits() & Self::OETOPAD_MASK) != 0
    }

    /// **OUTTOPAD** [9:9]: output signal to pad after register override is applied
    #[inline(always)]
    pub fn outtopad(&self) -> bool {
        (self.read_bits() & Self::OUTTOPAD_MASK) != 0
    }

}

/// USBPHY_DP_STATUS value type for batch read/modify/write operations
///
/// Use `usbphydpstatus.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `usbphydpstatus.write_val(val)`.
#[derive(Clone, Copy)]
pub struct UsbphyDpStatusVal {
    bits: u32,
}

impl UsbphyDpStatusVal {
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

    /// Read IRQTOPROC [26:26]: interrupt to processors, after override is applied
    #[inline(always)]
    pub const fn irqtoproc(&self) -> bool {
        (self.bits & UsbphyDpStatus::IRQTOPROC_MASK) != 0
    }

    /// Read INFROMPAD [17:17]: input signal from pad, before filtering and override are applied
    #[inline(always)]
    pub const fn infrompad(&self) -> bool {
        (self.bits & UsbphyDpStatus::INFROMPAD_MASK) != 0
    }

    /// Read OETOPAD [13:13]: output enable to pad after register override is applied
    #[inline(always)]
    pub const fn oetopad(&self) -> bool {
        (self.bits & UsbphyDpStatus::OETOPAD_MASK) != 0
    }

    /// Read OUTTOPAD [9:9]: output signal to pad after register override is applied
    #[inline(always)]
    pub const fn outtopad(&self) -> bool {
        (self.bits & UsbphyDpStatus::OUTTOPAD_MASK) != 0
    }

}

impl Default for UsbphyDpStatusVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl UsbphyDpStatus {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> UsbphyDpStatusVal {
        UsbphyDpStatusVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: UsbphyDpStatusVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(UsbphyDpStatusVal) -> UsbphyDpStatusVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// USBPHY_DP_CTRL Register
///
/// Address offset: 0x0004
/// Reset value: 0x0000001f
///
/// ## Fields
///
/// - **IRQOVER** [29:28] (RW): 
/// - **INOVER** [17:16] (RW): 
/// - **OEOVER** [15:14] (RW): 
/// - **OUTOVER** [13:12] (RW): 
/// - **FUNCSEL** [4:0] (RW): 0-31 -> selects pin function according to the gpio table 31 == NULL
#[repr(transparent)]
pub struct UsbphyDpCtrl {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for UsbphyDpCtrl {}
unsafe impl Sync for UsbphyDpCtrl {}

impl UsbphyDpCtrl {
    // ======== Field Constants ========

    /// Bits [29:28], mask 0x30000000
    pub const IRQOVER_OFFSET: u32 = 28;
    pub const IRQOVER_MASK: u32 = 0x30000000;
    /// NORMAL = 0: don't invert the interrupt
    pub const IRQOVER_NORMAL: u32 = 0 << 28;
    /// INVERT = 1: invert the interrupt
    pub const IRQOVER_INVERT: u32 = 1 << 28;
    /// LOW = 2: drive interrupt low
    pub const IRQOVER_LOW: u32 = 2 << 28;
    /// HIGH = 3: drive interrupt high
    pub const IRQOVER_HIGH: u32 = 3 << 28;

    /// Bits [17:16], mask 0x00030000
    pub const INOVER_OFFSET: u32 = 16;
    pub const INOVER_MASK: u32 = 0x00030000;
    /// NORMAL = 0: don't invert the peri input
    pub const INOVER_NORMAL: u32 = 0 << 16;
    /// INVERT = 1: invert the peri input
    pub const INOVER_INVERT: u32 = 1 << 16;
    /// LOW = 2: drive peri input low
    pub const INOVER_LOW: u32 = 2 << 16;
    /// HIGH = 3: drive peri input high
    pub const INOVER_HIGH: u32 = 3 << 16;

    /// Bits [15:14], mask 0x0000c000
    pub const OEOVER_OFFSET: u32 = 14;
    pub const OEOVER_MASK: u32 = 0x0000c000;
    /// NORMAL = 0: drive output enable from peripheral signal selected by funcsel
    pub const OEOVER_NORMAL: u32 = 0 << 14;
    /// INVERT = 1: drive output enable from inverse of peripheral signal selected by funcsel
    pub const OEOVER_INVERT: u32 = 1 << 14;
    /// DISABLE = 2: disable output
    pub const OEOVER_DISABLE: u32 = 2 << 14;
    /// ENABLE = 3: enable output
    pub const OEOVER_ENABLE: u32 = 3 << 14;

    /// Bits [13:12], mask 0x00003000
    pub const OUTOVER_OFFSET: u32 = 12;
    pub const OUTOVER_MASK: u32 = 0x00003000;
    /// NORMAL = 0: drive output from peripheral signal selected by funcsel
    pub const OUTOVER_NORMAL: u32 = 0 << 12;
    /// INVERT = 1: drive output from inverse of peripheral signal selected by funcsel
    pub const OUTOVER_INVERT: u32 = 1 << 12;
    /// LOW = 2: drive output low
    pub const OUTOVER_LOW: u32 = 2 << 12;
    /// HIGH = 3: drive output high
    pub const OUTOVER_HIGH: u32 = 3 << 12;

    /// FUNCSEL - 0-31 -> selects pin function according to the gpio table 31 == NULL
    /// Bits [4:0], mask 0x0000001f
    pub const FUNCSEL_OFFSET: u32 = 0;
    pub const FUNCSEL_MASK: u32 = 0x0000001f;
    pub const FUNCSEL_UART1_TX: u32 = 2 << 0;
    pub const FUNCSEL_I2C0_SDA: u32 = 3 << 0;
    pub const FUNCSEL_SIOB_PROC_56: u32 = 5 << 0;
    pub const FUNCSEL_NULL: u32 = 31 << 0;

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

    /// **IRQOVER** [29:28]
    #[inline(always)]
    pub fn irqover(&self) -> u32 {
        (self.read_bits() & Self::IRQOVER_MASK) >> Self::IRQOVER_OFFSET
    }

    /// Write IRQOVER field
    #[inline(always)]
    pub fn write_irqover(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::IRQOVER_MASK) | ((value << Self::IRQOVER_OFFSET) & Self::IRQOVER_MASK));
    }

    /// **INOVER** [17:16]
    #[inline(always)]
    pub fn inover(&self) -> u32 {
        (self.read_bits() & Self::INOVER_MASK) >> Self::INOVER_OFFSET
    }

    /// Write INOVER field
    #[inline(always)]
    pub fn write_inover(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::INOVER_MASK) | ((value << Self::INOVER_OFFSET) & Self::INOVER_MASK));
    }

    /// **OEOVER** [15:14]
    #[inline(always)]
    pub fn oeover(&self) -> u32 {
        (self.read_bits() & Self::OEOVER_MASK) >> Self::OEOVER_OFFSET
    }

    /// Write OEOVER field
    #[inline(always)]
    pub fn write_oeover(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::OEOVER_MASK) | ((value << Self::OEOVER_OFFSET) & Self::OEOVER_MASK));
    }

    /// **OUTOVER** [13:12]
    #[inline(always)]
    pub fn outover(&self) -> u32 {
        (self.read_bits() & Self::OUTOVER_MASK) >> Self::OUTOVER_OFFSET
    }

    /// Write OUTOVER field
    #[inline(always)]
    pub fn write_outover(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::OUTOVER_MASK) | ((value << Self::OUTOVER_OFFSET) & Self::OUTOVER_MASK));
    }

    /// **FUNCSEL** [4:0]: 0-31 -> selects pin function according to the gpio table 31 == NULL
    #[inline(always)]
    pub fn funcsel(&self) -> u32 {
        (self.read_bits() & Self::FUNCSEL_MASK) >> Self::FUNCSEL_OFFSET
    }

    /// Write FUNCSEL field
    #[inline(always)]
    pub fn write_funcsel(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::FUNCSEL_MASK) | ((value << Self::FUNCSEL_OFFSET) & Self::FUNCSEL_MASK));
    }

}

/// USBPHY_DP_CTRL value type for batch read/modify/write operations
///
/// Use `usbphydpctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `usbphydpctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct UsbphyDpCtrlVal {
    bits: u32,
}

impl UsbphyDpCtrlVal {
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
        Self { bits: 0x0000001f }
    }

    /// Read IRQOVER [29:28]
    #[inline(always)]
    pub const fn irqover(&self) -> u32 {
        (self.bits & UsbphyDpCtrl::IRQOVER_MASK) >> UsbphyDpCtrl::IRQOVER_OFFSET
    }

    /// Read INOVER [17:16]
    #[inline(always)]
    pub const fn inover(&self) -> u32 {
        (self.bits & UsbphyDpCtrl::INOVER_MASK) >> UsbphyDpCtrl::INOVER_OFFSET
    }

    /// Read OEOVER [15:14]
    #[inline(always)]
    pub const fn oeover(&self) -> u32 {
        (self.bits & UsbphyDpCtrl::OEOVER_MASK) >> UsbphyDpCtrl::OEOVER_OFFSET
    }

    /// Read OUTOVER [13:12]
    #[inline(always)]
    pub const fn outover(&self) -> u32 {
        (self.bits & UsbphyDpCtrl::OUTOVER_MASK) >> UsbphyDpCtrl::OUTOVER_OFFSET
    }

    /// Read FUNCSEL [4:0]: 0-31 -> selects pin function according to the gpio table 31 == NULL
    #[inline(always)]
    pub const fn funcsel(&self) -> u32 {
        (self.bits & UsbphyDpCtrl::FUNCSEL_MASK) >> UsbphyDpCtrl::FUNCSEL_OFFSET
    }

    /// Set IRQOVER [29:28]
    #[inline(always)]
    pub const fn with_irqover(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !UsbphyDpCtrl::IRQOVER_MASK) | ((value << UsbphyDpCtrl::IRQOVER_OFFSET) & UsbphyDpCtrl::IRQOVER_MASK)
        }
    }

    /// Set INOVER [17:16]
    #[inline(always)]
    pub const fn with_inover(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !UsbphyDpCtrl::INOVER_MASK) | ((value << UsbphyDpCtrl::INOVER_OFFSET) & UsbphyDpCtrl::INOVER_MASK)
        }
    }

    /// Set OEOVER [15:14]
    #[inline(always)]
    pub const fn with_oeover(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !UsbphyDpCtrl::OEOVER_MASK) | ((value << UsbphyDpCtrl::OEOVER_OFFSET) & UsbphyDpCtrl::OEOVER_MASK)
        }
    }

    /// Set OUTOVER [13:12]
    #[inline(always)]
    pub const fn with_outover(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !UsbphyDpCtrl::OUTOVER_MASK) | ((value << UsbphyDpCtrl::OUTOVER_OFFSET) & UsbphyDpCtrl::OUTOVER_MASK)
        }
    }

    /// Set FUNCSEL [4:0]: 0-31 -> selects pin function according to the gpio table 31 == NULL
    #[inline(always)]
    pub const fn with_funcsel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !UsbphyDpCtrl::FUNCSEL_MASK) | ((value << UsbphyDpCtrl::FUNCSEL_OFFSET) & UsbphyDpCtrl::FUNCSEL_MASK)
        }
    }

}

impl Default for UsbphyDpCtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl UsbphyDpCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> UsbphyDpCtrlVal {
        UsbphyDpCtrlVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: UsbphyDpCtrlVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(UsbphyDpCtrlVal) -> UsbphyDpCtrlVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// USBPHY_DM_STATUS Register
///
/// Address offset: 0x0008
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **IRQTOPROC** [26] (RO): interrupt to processors, after override is applied
/// - **INFROMPAD** [17] (RO): input signal from pad, before filtering and override are applied
/// - **OETOPAD** [13] (RO): output enable to pad after register override is applied
/// - **OUTTOPAD** [9] (RO): output signal to pad after register override is applied
#[repr(transparent)]
pub struct UsbphyDmStatus {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for UsbphyDmStatus {}
unsafe impl Sync for UsbphyDmStatus {}

impl UsbphyDmStatus {
    // ======== Field Constants ========

    /// IRQTOPROC - interrupt to processors, after override is applied
    /// Bits [26:26], mask 0x04000000
    pub const IRQTOPROC_OFFSET: u32 = 26;
    pub const IRQTOPROC_MASK: u32 = 0x04000000;
    pub const IRQTOPROC: u32 = 1 << 26;

    /// INFROMPAD - input signal from pad, before filtering and override are applied
    /// Bits [17:17], mask 0x00020000
    pub const INFROMPAD_OFFSET: u32 = 17;
    pub const INFROMPAD_MASK: u32 = 0x00020000;
    pub const INFROMPAD: u32 = 1 << 17;

    /// OETOPAD - output enable to pad after register override is applied
    /// Bits [13:13], mask 0x00002000
    pub const OETOPAD_OFFSET: u32 = 13;
    pub const OETOPAD_MASK: u32 = 0x00002000;
    pub const OETOPAD: u32 = 1 << 13;

    /// OUTTOPAD - output signal to pad after register override is applied
    /// Bits [9:9], mask 0x00000200
    pub const OUTTOPAD_OFFSET: u32 = 9;
    pub const OUTTOPAD_MASK: u32 = 0x00000200;
    pub const OUTTOPAD: u32 = 1 << 9;

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

    /// **IRQTOPROC** [26:26]: interrupt to processors, after override is applied
    #[inline(always)]
    pub fn irqtoproc(&self) -> bool {
        (self.read_bits() & Self::IRQTOPROC_MASK) != 0
    }

    /// **INFROMPAD** [17:17]: input signal from pad, before filtering and override are applied
    #[inline(always)]
    pub fn infrompad(&self) -> bool {
        (self.read_bits() & Self::INFROMPAD_MASK) != 0
    }

    /// **OETOPAD** [13:13]: output enable to pad after register override is applied
    #[inline(always)]
    pub fn oetopad(&self) -> bool {
        (self.read_bits() & Self::OETOPAD_MASK) != 0
    }

    /// **OUTTOPAD** [9:9]: output signal to pad after register override is applied
    #[inline(always)]
    pub fn outtopad(&self) -> bool {
        (self.read_bits() & Self::OUTTOPAD_MASK) != 0
    }

}

/// USBPHY_DM_STATUS value type for batch read/modify/write operations
///
/// Use `usbphydmstatus.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `usbphydmstatus.write_val(val)`.
#[derive(Clone, Copy)]
pub struct UsbphyDmStatusVal {
    bits: u32,
}

impl UsbphyDmStatusVal {
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

    /// Read IRQTOPROC [26:26]: interrupt to processors, after override is applied
    #[inline(always)]
    pub const fn irqtoproc(&self) -> bool {
        (self.bits & UsbphyDmStatus::IRQTOPROC_MASK) != 0
    }

    /// Read INFROMPAD [17:17]: input signal from pad, before filtering and override are applied
    #[inline(always)]
    pub const fn infrompad(&self) -> bool {
        (self.bits & UsbphyDmStatus::INFROMPAD_MASK) != 0
    }

    /// Read OETOPAD [13:13]: output enable to pad after register override is applied
    #[inline(always)]
    pub const fn oetopad(&self) -> bool {
        (self.bits & UsbphyDmStatus::OETOPAD_MASK) != 0
    }

    /// Read OUTTOPAD [9:9]: output signal to pad after register override is applied
    #[inline(always)]
    pub const fn outtopad(&self) -> bool {
        (self.bits & UsbphyDmStatus::OUTTOPAD_MASK) != 0
    }

}

impl Default for UsbphyDmStatusVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl UsbphyDmStatus {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> UsbphyDmStatusVal {
        UsbphyDmStatusVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: UsbphyDmStatusVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(UsbphyDmStatusVal) -> UsbphyDmStatusVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// USBPHY_DM_CTRL Register
///
/// Address offset: 0x000c
/// Reset value: 0x0000001f
///
/// ## Fields
///
/// - **IRQOVER** [29:28] (RW): 
/// - **INOVER** [17:16] (RW): 
/// - **OEOVER** [15:14] (RW): 
/// - **OUTOVER** [13:12] (RW): 
/// - **FUNCSEL** [4:0] (RW): 0-31 -> selects pin function according to the gpio table 31 == NULL
#[repr(transparent)]
pub struct UsbphyDmCtrl {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for UsbphyDmCtrl {}
unsafe impl Sync for UsbphyDmCtrl {}

impl UsbphyDmCtrl {
    // ======== Field Constants ========

    /// Bits [29:28], mask 0x30000000
    pub const IRQOVER_OFFSET: u32 = 28;
    pub const IRQOVER_MASK: u32 = 0x30000000;
    /// NORMAL = 0: don't invert the interrupt
    pub const IRQOVER_NORMAL: u32 = 0 << 28;
    /// INVERT = 1: invert the interrupt
    pub const IRQOVER_INVERT: u32 = 1 << 28;
    /// LOW = 2: drive interrupt low
    pub const IRQOVER_LOW: u32 = 2 << 28;
    /// HIGH = 3: drive interrupt high
    pub const IRQOVER_HIGH: u32 = 3 << 28;

    /// Bits [17:16], mask 0x00030000
    pub const INOVER_OFFSET: u32 = 16;
    pub const INOVER_MASK: u32 = 0x00030000;
    /// NORMAL = 0: don't invert the peri input
    pub const INOVER_NORMAL: u32 = 0 << 16;
    /// INVERT = 1: invert the peri input
    pub const INOVER_INVERT: u32 = 1 << 16;
    /// LOW = 2: drive peri input low
    pub const INOVER_LOW: u32 = 2 << 16;
    /// HIGH = 3: drive peri input high
    pub const INOVER_HIGH: u32 = 3 << 16;

    /// Bits [15:14], mask 0x0000c000
    pub const OEOVER_OFFSET: u32 = 14;
    pub const OEOVER_MASK: u32 = 0x0000c000;
    /// NORMAL = 0: drive output enable from peripheral signal selected by funcsel
    pub const OEOVER_NORMAL: u32 = 0 << 14;
    /// INVERT = 1: drive output enable from inverse of peripheral signal selected by funcsel
    pub const OEOVER_INVERT: u32 = 1 << 14;
    /// DISABLE = 2: disable output
    pub const OEOVER_DISABLE: u32 = 2 << 14;
    /// ENABLE = 3: enable output
    pub const OEOVER_ENABLE: u32 = 3 << 14;

    /// Bits [13:12], mask 0x00003000
    pub const OUTOVER_OFFSET: u32 = 12;
    pub const OUTOVER_MASK: u32 = 0x00003000;
    /// NORMAL = 0: drive output from peripheral signal selected by funcsel
    pub const OUTOVER_NORMAL: u32 = 0 << 12;
    /// INVERT = 1: drive output from inverse of peripheral signal selected by funcsel
    pub const OUTOVER_INVERT: u32 = 1 << 12;
    /// LOW = 2: drive output low
    pub const OUTOVER_LOW: u32 = 2 << 12;
    /// HIGH = 3: drive output high
    pub const OUTOVER_HIGH: u32 = 3 << 12;

    /// FUNCSEL - 0-31 -> selects pin function according to the gpio table 31 == NULL
    /// Bits [4:0], mask 0x0000001f
    pub const FUNCSEL_OFFSET: u32 = 0;
    pub const FUNCSEL_MASK: u32 = 0x0000001f;
    pub const FUNCSEL_UART1_RX: u32 = 2 << 0;
    pub const FUNCSEL_I2C0_SCL: u32 = 3 << 0;
    pub const FUNCSEL_SIOB_PROC_57: u32 = 5 << 0;
    pub const FUNCSEL_NULL: u32 = 31 << 0;

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

    /// **IRQOVER** [29:28]
    #[inline(always)]
    pub fn irqover(&self) -> u32 {
        (self.read_bits() & Self::IRQOVER_MASK) >> Self::IRQOVER_OFFSET
    }

    /// Write IRQOVER field
    #[inline(always)]
    pub fn write_irqover(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::IRQOVER_MASK) | ((value << Self::IRQOVER_OFFSET) & Self::IRQOVER_MASK));
    }

    /// **INOVER** [17:16]
    #[inline(always)]
    pub fn inover(&self) -> u32 {
        (self.read_bits() & Self::INOVER_MASK) >> Self::INOVER_OFFSET
    }

    /// Write INOVER field
    #[inline(always)]
    pub fn write_inover(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::INOVER_MASK) | ((value << Self::INOVER_OFFSET) & Self::INOVER_MASK));
    }

    /// **OEOVER** [15:14]
    #[inline(always)]
    pub fn oeover(&self) -> u32 {
        (self.read_bits() & Self::OEOVER_MASK) >> Self::OEOVER_OFFSET
    }

    /// Write OEOVER field
    #[inline(always)]
    pub fn write_oeover(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::OEOVER_MASK) | ((value << Self::OEOVER_OFFSET) & Self::OEOVER_MASK));
    }

    /// **OUTOVER** [13:12]
    #[inline(always)]
    pub fn outover(&self) -> u32 {
        (self.read_bits() & Self::OUTOVER_MASK) >> Self::OUTOVER_OFFSET
    }

    /// Write OUTOVER field
    #[inline(always)]
    pub fn write_outover(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::OUTOVER_MASK) | ((value << Self::OUTOVER_OFFSET) & Self::OUTOVER_MASK));
    }

    /// **FUNCSEL** [4:0]: 0-31 -> selects pin function according to the gpio table 31 == NULL
    #[inline(always)]
    pub fn funcsel(&self) -> u32 {
        (self.read_bits() & Self::FUNCSEL_MASK) >> Self::FUNCSEL_OFFSET
    }

    /// Write FUNCSEL field
    #[inline(always)]
    pub fn write_funcsel(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::FUNCSEL_MASK) | ((value << Self::FUNCSEL_OFFSET) & Self::FUNCSEL_MASK));
    }

}

/// USBPHY_DM_CTRL value type for batch read/modify/write operations
///
/// Use `usbphydmctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `usbphydmctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct UsbphyDmCtrlVal {
    bits: u32,
}

impl UsbphyDmCtrlVal {
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
        Self { bits: 0x0000001f }
    }

    /// Read IRQOVER [29:28]
    #[inline(always)]
    pub const fn irqover(&self) -> u32 {
        (self.bits & UsbphyDmCtrl::IRQOVER_MASK) >> UsbphyDmCtrl::IRQOVER_OFFSET
    }

    /// Read INOVER [17:16]
    #[inline(always)]
    pub const fn inover(&self) -> u32 {
        (self.bits & UsbphyDmCtrl::INOVER_MASK) >> UsbphyDmCtrl::INOVER_OFFSET
    }

    /// Read OEOVER [15:14]
    #[inline(always)]
    pub const fn oeover(&self) -> u32 {
        (self.bits & UsbphyDmCtrl::OEOVER_MASK) >> UsbphyDmCtrl::OEOVER_OFFSET
    }

    /// Read OUTOVER [13:12]
    #[inline(always)]
    pub const fn outover(&self) -> u32 {
        (self.bits & UsbphyDmCtrl::OUTOVER_MASK) >> UsbphyDmCtrl::OUTOVER_OFFSET
    }

    /// Read FUNCSEL [4:0]: 0-31 -> selects pin function according to the gpio table 31 == NULL
    #[inline(always)]
    pub const fn funcsel(&self) -> u32 {
        (self.bits & UsbphyDmCtrl::FUNCSEL_MASK) >> UsbphyDmCtrl::FUNCSEL_OFFSET
    }

    /// Set IRQOVER [29:28]
    #[inline(always)]
    pub const fn with_irqover(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !UsbphyDmCtrl::IRQOVER_MASK) | ((value << UsbphyDmCtrl::IRQOVER_OFFSET) & UsbphyDmCtrl::IRQOVER_MASK)
        }
    }

    /// Set INOVER [17:16]
    #[inline(always)]
    pub const fn with_inover(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !UsbphyDmCtrl::INOVER_MASK) | ((value << UsbphyDmCtrl::INOVER_OFFSET) & UsbphyDmCtrl::INOVER_MASK)
        }
    }

    /// Set OEOVER [15:14]
    #[inline(always)]
    pub const fn with_oeover(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !UsbphyDmCtrl::OEOVER_MASK) | ((value << UsbphyDmCtrl::OEOVER_OFFSET) & UsbphyDmCtrl::OEOVER_MASK)
        }
    }

    /// Set OUTOVER [13:12]
    #[inline(always)]
    pub const fn with_outover(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !UsbphyDmCtrl::OUTOVER_MASK) | ((value << UsbphyDmCtrl::OUTOVER_OFFSET) & UsbphyDmCtrl::OUTOVER_MASK)
        }
    }

    /// Set FUNCSEL [4:0]: 0-31 -> selects pin function according to the gpio table 31 == NULL
    #[inline(always)]
    pub const fn with_funcsel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !UsbphyDmCtrl::FUNCSEL_MASK) | ((value << UsbphyDmCtrl::FUNCSEL_OFFSET) & UsbphyDmCtrl::FUNCSEL_MASK)
        }
    }

}

impl Default for UsbphyDmCtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl UsbphyDmCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> UsbphyDmCtrlVal {
        UsbphyDmCtrlVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: UsbphyDmCtrlVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(UsbphyDmCtrlVal) -> UsbphyDmCtrlVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// GPIO_QSPI_SCLK_STATUS Register
///
/// Address offset: 0x0010
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **IRQTOPROC** [26] (RO): interrupt to processors, after override is applied
/// - **INFROMPAD** [17] (RO): input signal from pad, before filtering and override are applied
/// - **OETOPAD** [13] (RO): output enable to pad after register override is applied
/// - **OUTTOPAD** [9] (RO): output signal to pad after register override is applied
#[repr(transparent)]
pub struct GpioQspiSclkStatus {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for GpioQspiSclkStatus {}
unsafe impl Sync for GpioQspiSclkStatus {}

impl GpioQspiSclkStatus {
    // ======== Field Constants ========

    /// IRQTOPROC - interrupt to processors, after override is applied
    /// Bits [26:26], mask 0x04000000
    pub const IRQTOPROC_OFFSET: u32 = 26;
    pub const IRQTOPROC_MASK: u32 = 0x04000000;
    pub const IRQTOPROC: u32 = 1 << 26;

    /// INFROMPAD - input signal from pad, before filtering and override are applied
    /// Bits [17:17], mask 0x00020000
    pub const INFROMPAD_OFFSET: u32 = 17;
    pub const INFROMPAD_MASK: u32 = 0x00020000;
    pub const INFROMPAD: u32 = 1 << 17;

    /// OETOPAD - output enable to pad after register override is applied
    /// Bits [13:13], mask 0x00002000
    pub const OETOPAD_OFFSET: u32 = 13;
    pub const OETOPAD_MASK: u32 = 0x00002000;
    pub const OETOPAD: u32 = 1 << 13;

    /// OUTTOPAD - output signal to pad after register override is applied
    /// Bits [9:9], mask 0x00000200
    pub const OUTTOPAD_OFFSET: u32 = 9;
    pub const OUTTOPAD_MASK: u32 = 0x00000200;
    pub const OUTTOPAD: u32 = 1 << 9;

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

    /// **IRQTOPROC** [26:26]: interrupt to processors, after override is applied
    #[inline(always)]
    pub fn irqtoproc(&self) -> bool {
        (self.read_bits() & Self::IRQTOPROC_MASK) != 0
    }

    /// **INFROMPAD** [17:17]: input signal from pad, before filtering and override are applied
    #[inline(always)]
    pub fn infrompad(&self) -> bool {
        (self.read_bits() & Self::INFROMPAD_MASK) != 0
    }

    /// **OETOPAD** [13:13]: output enable to pad after register override is applied
    #[inline(always)]
    pub fn oetopad(&self) -> bool {
        (self.read_bits() & Self::OETOPAD_MASK) != 0
    }

    /// **OUTTOPAD** [9:9]: output signal to pad after register override is applied
    #[inline(always)]
    pub fn outtopad(&self) -> bool {
        (self.read_bits() & Self::OUTTOPAD_MASK) != 0
    }

}

/// GPIO_QSPI_SCLK_STATUS value type for batch read/modify/write operations
///
/// Use `gpioqspisclkstatus.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpioqspisclkstatus.write_val(val)`.
#[derive(Clone, Copy)]
pub struct GpioQspiSclkStatusVal {
    bits: u32,
}

impl GpioQspiSclkStatusVal {
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

    /// Read IRQTOPROC [26:26]: interrupt to processors, after override is applied
    #[inline(always)]
    pub const fn irqtoproc(&self) -> bool {
        (self.bits & GpioQspiSclkStatus::IRQTOPROC_MASK) != 0
    }

    /// Read INFROMPAD [17:17]: input signal from pad, before filtering and override are applied
    #[inline(always)]
    pub const fn infrompad(&self) -> bool {
        (self.bits & GpioQspiSclkStatus::INFROMPAD_MASK) != 0
    }

    /// Read OETOPAD [13:13]: output enable to pad after register override is applied
    #[inline(always)]
    pub const fn oetopad(&self) -> bool {
        (self.bits & GpioQspiSclkStatus::OETOPAD_MASK) != 0
    }

    /// Read OUTTOPAD [9:9]: output signal to pad after register override is applied
    #[inline(always)]
    pub const fn outtopad(&self) -> bool {
        (self.bits & GpioQspiSclkStatus::OUTTOPAD_MASK) != 0
    }

}

impl Default for GpioQspiSclkStatusVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl GpioQspiSclkStatus {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> GpioQspiSclkStatusVal {
        GpioQspiSclkStatusVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: GpioQspiSclkStatusVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(GpioQspiSclkStatusVal) -> GpioQspiSclkStatusVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// GPIO_QSPI_SCLK_CTRL Register
///
/// Address offset: 0x0014
/// Reset value: 0x0000001f
///
/// ## Fields
///
/// - **IRQOVER** [29:28] (RW): 
/// - **INOVER** [17:16] (RW): 
/// - **OEOVER** [15:14] (RW): 
/// - **OUTOVER** [13:12] (RW): 
/// - **FUNCSEL** [4:0] (RW): 0-31 -> selects pin function according to the gpio table 31 == NULL
#[repr(transparent)]
pub struct GpioQspiSclkCtrl {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for GpioQspiSclkCtrl {}
unsafe impl Sync for GpioQspiSclkCtrl {}

impl GpioQspiSclkCtrl {
    // ======== Field Constants ========

    /// Bits [29:28], mask 0x30000000
    pub const IRQOVER_OFFSET: u32 = 28;
    pub const IRQOVER_MASK: u32 = 0x30000000;
    /// NORMAL = 0: don't invert the interrupt
    pub const IRQOVER_NORMAL: u32 = 0 << 28;
    /// INVERT = 1: invert the interrupt
    pub const IRQOVER_INVERT: u32 = 1 << 28;
    /// LOW = 2: drive interrupt low
    pub const IRQOVER_LOW: u32 = 2 << 28;
    /// HIGH = 3: drive interrupt high
    pub const IRQOVER_HIGH: u32 = 3 << 28;

    /// Bits [17:16], mask 0x00030000
    pub const INOVER_OFFSET: u32 = 16;
    pub const INOVER_MASK: u32 = 0x00030000;
    /// NORMAL = 0: don't invert the peri input
    pub const INOVER_NORMAL: u32 = 0 << 16;
    /// INVERT = 1: invert the peri input
    pub const INOVER_INVERT: u32 = 1 << 16;
    /// LOW = 2: drive peri input low
    pub const INOVER_LOW: u32 = 2 << 16;
    /// HIGH = 3: drive peri input high
    pub const INOVER_HIGH: u32 = 3 << 16;

    /// Bits [15:14], mask 0x0000c000
    pub const OEOVER_OFFSET: u32 = 14;
    pub const OEOVER_MASK: u32 = 0x0000c000;
    /// NORMAL = 0: drive output enable from peripheral signal selected by funcsel
    pub const OEOVER_NORMAL: u32 = 0 << 14;
    /// INVERT = 1: drive output enable from inverse of peripheral signal selected by funcsel
    pub const OEOVER_INVERT: u32 = 1 << 14;
    /// DISABLE = 2: disable output
    pub const OEOVER_DISABLE: u32 = 2 << 14;
    /// ENABLE = 3: enable output
    pub const OEOVER_ENABLE: u32 = 3 << 14;

    /// Bits [13:12], mask 0x00003000
    pub const OUTOVER_OFFSET: u32 = 12;
    pub const OUTOVER_MASK: u32 = 0x00003000;
    /// NORMAL = 0: drive output from peripheral signal selected by funcsel
    pub const OUTOVER_NORMAL: u32 = 0 << 12;
    /// INVERT = 1: drive output from inverse of peripheral signal selected by funcsel
    pub const OUTOVER_INVERT: u32 = 1 << 12;
    /// LOW = 2: drive output low
    pub const OUTOVER_LOW: u32 = 2 << 12;
    /// HIGH = 3: drive output high
    pub const OUTOVER_HIGH: u32 = 3 << 12;

    /// FUNCSEL - 0-31 -> selects pin function according to the gpio table 31 == NULL
    /// Bits [4:0], mask 0x0000001f
    pub const FUNCSEL_OFFSET: u32 = 0;
    pub const FUNCSEL_MASK: u32 = 0x0000001f;
    pub const FUNCSEL_XIP_SCLK: u32 = 0 << 0;
    pub const FUNCSEL_UART1_CTS: u32 = 2 << 0;
    pub const FUNCSEL_I2C1_SDA: u32 = 3 << 0;
    pub const FUNCSEL_SIOB_PROC_58: u32 = 5 << 0;
    pub const FUNCSEL_UART1_TX: u32 = 11 << 0;
    pub const FUNCSEL_NULL: u32 = 31 << 0;

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

    /// **IRQOVER** [29:28]
    #[inline(always)]
    pub fn irqover(&self) -> u32 {
        (self.read_bits() & Self::IRQOVER_MASK) >> Self::IRQOVER_OFFSET
    }

    /// Write IRQOVER field
    #[inline(always)]
    pub fn write_irqover(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::IRQOVER_MASK) | ((value << Self::IRQOVER_OFFSET) & Self::IRQOVER_MASK));
    }

    /// **INOVER** [17:16]
    #[inline(always)]
    pub fn inover(&self) -> u32 {
        (self.read_bits() & Self::INOVER_MASK) >> Self::INOVER_OFFSET
    }

    /// Write INOVER field
    #[inline(always)]
    pub fn write_inover(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::INOVER_MASK) | ((value << Self::INOVER_OFFSET) & Self::INOVER_MASK));
    }

    /// **OEOVER** [15:14]
    #[inline(always)]
    pub fn oeover(&self) -> u32 {
        (self.read_bits() & Self::OEOVER_MASK) >> Self::OEOVER_OFFSET
    }

    /// Write OEOVER field
    #[inline(always)]
    pub fn write_oeover(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::OEOVER_MASK) | ((value << Self::OEOVER_OFFSET) & Self::OEOVER_MASK));
    }

    /// **OUTOVER** [13:12]
    #[inline(always)]
    pub fn outover(&self) -> u32 {
        (self.read_bits() & Self::OUTOVER_MASK) >> Self::OUTOVER_OFFSET
    }

    /// Write OUTOVER field
    #[inline(always)]
    pub fn write_outover(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::OUTOVER_MASK) | ((value << Self::OUTOVER_OFFSET) & Self::OUTOVER_MASK));
    }

    /// **FUNCSEL** [4:0]: 0-31 -> selects pin function according to the gpio table 31 == NULL
    #[inline(always)]
    pub fn funcsel(&self) -> u32 {
        (self.read_bits() & Self::FUNCSEL_MASK) >> Self::FUNCSEL_OFFSET
    }

    /// Write FUNCSEL field
    #[inline(always)]
    pub fn write_funcsel(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::FUNCSEL_MASK) | ((value << Self::FUNCSEL_OFFSET) & Self::FUNCSEL_MASK));
    }

}

/// GPIO_QSPI_SCLK_CTRL value type for batch read/modify/write operations
///
/// Use `gpioqspisclkctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpioqspisclkctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct GpioQspiSclkCtrlVal {
    bits: u32,
}

impl GpioQspiSclkCtrlVal {
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
        Self { bits: 0x0000001f }
    }

    /// Read IRQOVER [29:28]
    #[inline(always)]
    pub const fn irqover(&self) -> u32 {
        (self.bits & GpioQspiSclkCtrl::IRQOVER_MASK) >> GpioQspiSclkCtrl::IRQOVER_OFFSET
    }

    /// Read INOVER [17:16]
    #[inline(always)]
    pub const fn inover(&self) -> u32 {
        (self.bits & GpioQspiSclkCtrl::INOVER_MASK) >> GpioQspiSclkCtrl::INOVER_OFFSET
    }

    /// Read OEOVER [15:14]
    #[inline(always)]
    pub const fn oeover(&self) -> u32 {
        (self.bits & GpioQspiSclkCtrl::OEOVER_MASK) >> GpioQspiSclkCtrl::OEOVER_OFFSET
    }

    /// Read OUTOVER [13:12]
    #[inline(always)]
    pub const fn outover(&self) -> u32 {
        (self.bits & GpioQspiSclkCtrl::OUTOVER_MASK) >> GpioQspiSclkCtrl::OUTOVER_OFFSET
    }

    /// Read FUNCSEL [4:0]: 0-31 -> selects pin function according to the gpio table 31 == NULL
    #[inline(always)]
    pub const fn funcsel(&self) -> u32 {
        (self.bits & GpioQspiSclkCtrl::FUNCSEL_MASK) >> GpioQspiSclkCtrl::FUNCSEL_OFFSET
    }

    /// Set IRQOVER [29:28]
    #[inline(always)]
    pub const fn with_irqover(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GpioQspiSclkCtrl::IRQOVER_MASK) | ((value << GpioQspiSclkCtrl::IRQOVER_OFFSET) & GpioQspiSclkCtrl::IRQOVER_MASK)
        }
    }

    /// Set INOVER [17:16]
    #[inline(always)]
    pub const fn with_inover(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GpioQspiSclkCtrl::INOVER_MASK) | ((value << GpioQspiSclkCtrl::INOVER_OFFSET) & GpioQspiSclkCtrl::INOVER_MASK)
        }
    }

    /// Set OEOVER [15:14]
    #[inline(always)]
    pub const fn with_oeover(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GpioQspiSclkCtrl::OEOVER_MASK) | ((value << GpioQspiSclkCtrl::OEOVER_OFFSET) & GpioQspiSclkCtrl::OEOVER_MASK)
        }
    }

    /// Set OUTOVER [13:12]
    #[inline(always)]
    pub const fn with_outover(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GpioQspiSclkCtrl::OUTOVER_MASK) | ((value << GpioQspiSclkCtrl::OUTOVER_OFFSET) & GpioQspiSclkCtrl::OUTOVER_MASK)
        }
    }

    /// Set FUNCSEL [4:0]: 0-31 -> selects pin function according to the gpio table 31 == NULL
    #[inline(always)]
    pub const fn with_funcsel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GpioQspiSclkCtrl::FUNCSEL_MASK) | ((value << GpioQspiSclkCtrl::FUNCSEL_OFFSET) & GpioQspiSclkCtrl::FUNCSEL_MASK)
        }
    }

}

impl Default for GpioQspiSclkCtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl GpioQspiSclkCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> GpioQspiSclkCtrlVal {
        GpioQspiSclkCtrlVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: GpioQspiSclkCtrlVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(GpioQspiSclkCtrlVal) -> GpioQspiSclkCtrlVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// GPIO_QSPI_SS_STATUS Register
///
/// Address offset: 0x0018
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **IRQTOPROC** [26] (RO): interrupt to processors, after override is applied
/// - **INFROMPAD** [17] (RO): input signal from pad, before filtering and override are applied
/// - **OETOPAD** [13] (RO): output enable to pad after register override is applied
/// - **OUTTOPAD** [9] (RO): output signal to pad after register override is applied
#[repr(transparent)]
pub struct GpioQspiSsStatus {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for GpioQspiSsStatus {}
unsafe impl Sync for GpioQspiSsStatus {}

impl GpioQspiSsStatus {
    // ======== Field Constants ========

    /// IRQTOPROC - interrupt to processors, after override is applied
    /// Bits [26:26], mask 0x04000000
    pub const IRQTOPROC_OFFSET: u32 = 26;
    pub const IRQTOPROC_MASK: u32 = 0x04000000;
    pub const IRQTOPROC: u32 = 1 << 26;

    /// INFROMPAD - input signal from pad, before filtering and override are applied
    /// Bits [17:17], mask 0x00020000
    pub const INFROMPAD_OFFSET: u32 = 17;
    pub const INFROMPAD_MASK: u32 = 0x00020000;
    pub const INFROMPAD: u32 = 1 << 17;

    /// OETOPAD - output enable to pad after register override is applied
    /// Bits [13:13], mask 0x00002000
    pub const OETOPAD_OFFSET: u32 = 13;
    pub const OETOPAD_MASK: u32 = 0x00002000;
    pub const OETOPAD: u32 = 1 << 13;

    /// OUTTOPAD - output signal to pad after register override is applied
    /// Bits [9:9], mask 0x00000200
    pub const OUTTOPAD_OFFSET: u32 = 9;
    pub const OUTTOPAD_MASK: u32 = 0x00000200;
    pub const OUTTOPAD: u32 = 1 << 9;

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

    /// **IRQTOPROC** [26:26]: interrupt to processors, after override is applied
    #[inline(always)]
    pub fn irqtoproc(&self) -> bool {
        (self.read_bits() & Self::IRQTOPROC_MASK) != 0
    }

    /// **INFROMPAD** [17:17]: input signal from pad, before filtering and override are applied
    #[inline(always)]
    pub fn infrompad(&self) -> bool {
        (self.read_bits() & Self::INFROMPAD_MASK) != 0
    }

    /// **OETOPAD** [13:13]: output enable to pad after register override is applied
    #[inline(always)]
    pub fn oetopad(&self) -> bool {
        (self.read_bits() & Self::OETOPAD_MASK) != 0
    }

    /// **OUTTOPAD** [9:9]: output signal to pad after register override is applied
    #[inline(always)]
    pub fn outtopad(&self) -> bool {
        (self.read_bits() & Self::OUTTOPAD_MASK) != 0
    }

}

/// GPIO_QSPI_SS_STATUS value type for batch read/modify/write operations
///
/// Use `gpioqspissstatus.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpioqspissstatus.write_val(val)`.
#[derive(Clone, Copy)]
pub struct GpioQspiSsStatusVal {
    bits: u32,
}

impl GpioQspiSsStatusVal {
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

    /// Read IRQTOPROC [26:26]: interrupt to processors, after override is applied
    #[inline(always)]
    pub const fn irqtoproc(&self) -> bool {
        (self.bits & GpioQspiSsStatus::IRQTOPROC_MASK) != 0
    }

    /// Read INFROMPAD [17:17]: input signal from pad, before filtering and override are applied
    #[inline(always)]
    pub const fn infrompad(&self) -> bool {
        (self.bits & GpioQspiSsStatus::INFROMPAD_MASK) != 0
    }

    /// Read OETOPAD [13:13]: output enable to pad after register override is applied
    #[inline(always)]
    pub const fn oetopad(&self) -> bool {
        (self.bits & GpioQspiSsStatus::OETOPAD_MASK) != 0
    }

    /// Read OUTTOPAD [9:9]: output signal to pad after register override is applied
    #[inline(always)]
    pub const fn outtopad(&self) -> bool {
        (self.bits & GpioQspiSsStatus::OUTTOPAD_MASK) != 0
    }

}

impl Default for GpioQspiSsStatusVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl GpioQspiSsStatus {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> GpioQspiSsStatusVal {
        GpioQspiSsStatusVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: GpioQspiSsStatusVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(GpioQspiSsStatusVal) -> GpioQspiSsStatusVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// GPIO_QSPI_SS_CTRL Register
///
/// Address offset: 0x001c
/// Reset value: 0x0000001f
///
/// ## Fields
///
/// - **IRQOVER** [29:28] (RW): 
/// - **INOVER** [17:16] (RW): 
/// - **OEOVER** [15:14] (RW): 
/// - **OUTOVER** [13:12] (RW): 
/// - **FUNCSEL** [4:0] (RW): 0-31 -> selects pin function according to the gpio table 31 == NULL
#[repr(transparent)]
pub struct GpioQspiSsCtrl {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for GpioQspiSsCtrl {}
unsafe impl Sync for GpioQspiSsCtrl {}

impl GpioQspiSsCtrl {
    // ======== Field Constants ========

    /// Bits [29:28], mask 0x30000000
    pub const IRQOVER_OFFSET: u32 = 28;
    pub const IRQOVER_MASK: u32 = 0x30000000;
    /// NORMAL = 0: don't invert the interrupt
    pub const IRQOVER_NORMAL: u32 = 0 << 28;
    /// INVERT = 1: invert the interrupt
    pub const IRQOVER_INVERT: u32 = 1 << 28;
    /// LOW = 2: drive interrupt low
    pub const IRQOVER_LOW: u32 = 2 << 28;
    /// HIGH = 3: drive interrupt high
    pub const IRQOVER_HIGH: u32 = 3 << 28;

    /// Bits [17:16], mask 0x00030000
    pub const INOVER_OFFSET: u32 = 16;
    pub const INOVER_MASK: u32 = 0x00030000;
    /// NORMAL = 0: don't invert the peri input
    pub const INOVER_NORMAL: u32 = 0 << 16;
    /// INVERT = 1: invert the peri input
    pub const INOVER_INVERT: u32 = 1 << 16;
    /// LOW = 2: drive peri input low
    pub const INOVER_LOW: u32 = 2 << 16;
    /// HIGH = 3: drive peri input high
    pub const INOVER_HIGH: u32 = 3 << 16;

    /// Bits [15:14], mask 0x0000c000
    pub const OEOVER_OFFSET: u32 = 14;
    pub const OEOVER_MASK: u32 = 0x0000c000;
    /// NORMAL = 0: drive output enable from peripheral signal selected by funcsel
    pub const OEOVER_NORMAL: u32 = 0 << 14;
    /// INVERT = 1: drive output enable from inverse of peripheral signal selected by funcsel
    pub const OEOVER_INVERT: u32 = 1 << 14;
    /// DISABLE = 2: disable output
    pub const OEOVER_DISABLE: u32 = 2 << 14;
    /// ENABLE = 3: enable output
    pub const OEOVER_ENABLE: u32 = 3 << 14;

    /// Bits [13:12], mask 0x00003000
    pub const OUTOVER_OFFSET: u32 = 12;
    pub const OUTOVER_MASK: u32 = 0x00003000;
    /// NORMAL = 0: drive output from peripheral signal selected by funcsel
    pub const OUTOVER_NORMAL: u32 = 0 << 12;
    /// INVERT = 1: drive output from inverse of peripheral signal selected by funcsel
    pub const OUTOVER_INVERT: u32 = 1 << 12;
    /// LOW = 2: drive output low
    pub const OUTOVER_LOW: u32 = 2 << 12;
    /// HIGH = 3: drive output high
    pub const OUTOVER_HIGH: u32 = 3 << 12;

    /// FUNCSEL - 0-31 -> selects pin function according to the gpio table 31 == NULL
    /// Bits [4:0], mask 0x0000001f
    pub const FUNCSEL_OFFSET: u32 = 0;
    pub const FUNCSEL_MASK: u32 = 0x0000001f;
    pub const FUNCSEL_XIP_SS_N_0: u32 = 0 << 0;
    pub const FUNCSEL_UART1_RTS: u32 = 2 << 0;
    pub const FUNCSEL_I2C1_SCL: u32 = 3 << 0;
    pub const FUNCSEL_SIOB_PROC_59: u32 = 5 << 0;
    pub const FUNCSEL_UART1_RX: u32 = 11 << 0;
    pub const FUNCSEL_NULL: u32 = 31 << 0;

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

    /// **IRQOVER** [29:28]
    #[inline(always)]
    pub fn irqover(&self) -> u32 {
        (self.read_bits() & Self::IRQOVER_MASK) >> Self::IRQOVER_OFFSET
    }

    /// Write IRQOVER field
    #[inline(always)]
    pub fn write_irqover(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::IRQOVER_MASK) | ((value << Self::IRQOVER_OFFSET) & Self::IRQOVER_MASK));
    }

    /// **INOVER** [17:16]
    #[inline(always)]
    pub fn inover(&self) -> u32 {
        (self.read_bits() & Self::INOVER_MASK) >> Self::INOVER_OFFSET
    }

    /// Write INOVER field
    #[inline(always)]
    pub fn write_inover(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::INOVER_MASK) | ((value << Self::INOVER_OFFSET) & Self::INOVER_MASK));
    }

    /// **OEOVER** [15:14]
    #[inline(always)]
    pub fn oeover(&self) -> u32 {
        (self.read_bits() & Self::OEOVER_MASK) >> Self::OEOVER_OFFSET
    }

    /// Write OEOVER field
    #[inline(always)]
    pub fn write_oeover(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::OEOVER_MASK) | ((value << Self::OEOVER_OFFSET) & Self::OEOVER_MASK));
    }

    /// **OUTOVER** [13:12]
    #[inline(always)]
    pub fn outover(&self) -> u32 {
        (self.read_bits() & Self::OUTOVER_MASK) >> Self::OUTOVER_OFFSET
    }

    /// Write OUTOVER field
    #[inline(always)]
    pub fn write_outover(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::OUTOVER_MASK) | ((value << Self::OUTOVER_OFFSET) & Self::OUTOVER_MASK));
    }

    /// **FUNCSEL** [4:0]: 0-31 -> selects pin function according to the gpio table 31 == NULL
    #[inline(always)]
    pub fn funcsel(&self) -> u32 {
        (self.read_bits() & Self::FUNCSEL_MASK) >> Self::FUNCSEL_OFFSET
    }

    /// Write FUNCSEL field
    #[inline(always)]
    pub fn write_funcsel(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::FUNCSEL_MASK) | ((value << Self::FUNCSEL_OFFSET) & Self::FUNCSEL_MASK));
    }

}

/// GPIO_QSPI_SS_CTRL value type for batch read/modify/write operations
///
/// Use `gpioqspissctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpioqspissctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct GpioQspiSsCtrlVal {
    bits: u32,
}

impl GpioQspiSsCtrlVal {
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
        Self { bits: 0x0000001f }
    }

    /// Read IRQOVER [29:28]
    #[inline(always)]
    pub const fn irqover(&self) -> u32 {
        (self.bits & GpioQspiSsCtrl::IRQOVER_MASK) >> GpioQspiSsCtrl::IRQOVER_OFFSET
    }

    /// Read INOVER [17:16]
    #[inline(always)]
    pub const fn inover(&self) -> u32 {
        (self.bits & GpioQspiSsCtrl::INOVER_MASK) >> GpioQspiSsCtrl::INOVER_OFFSET
    }

    /// Read OEOVER [15:14]
    #[inline(always)]
    pub const fn oeover(&self) -> u32 {
        (self.bits & GpioQspiSsCtrl::OEOVER_MASK) >> GpioQspiSsCtrl::OEOVER_OFFSET
    }

    /// Read OUTOVER [13:12]
    #[inline(always)]
    pub const fn outover(&self) -> u32 {
        (self.bits & GpioQspiSsCtrl::OUTOVER_MASK) >> GpioQspiSsCtrl::OUTOVER_OFFSET
    }

    /// Read FUNCSEL [4:0]: 0-31 -> selects pin function according to the gpio table 31 == NULL
    #[inline(always)]
    pub const fn funcsel(&self) -> u32 {
        (self.bits & GpioQspiSsCtrl::FUNCSEL_MASK) >> GpioQspiSsCtrl::FUNCSEL_OFFSET
    }

    /// Set IRQOVER [29:28]
    #[inline(always)]
    pub const fn with_irqover(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GpioQspiSsCtrl::IRQOVER_MASK) | ((value << GpioQspiSsCtrl::IRQOVER_OFFSET) & GpioQspiSsCtrl::IRQOVER_MASK)
        }
    }

    /// Set INOVER [17:16]
    #[inline(always)]
    pub const fn with_inover(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GpioQspiSsCtrl::INOVER_MASK) | ((value << GpioQspiSsCtrl::INOVER_OFFSET) & GpioQspiSsCtrl::INOVER_MASK)
        }
    }

    /// Set OEOVER [15:14]
    #[inline(always)]
    pub const fn with_oeover(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GpioQspiSsCtrl::OEOVER_MASK) | ((value << GpioQspiSsCtrl::OEOVER_OFFSET) & GpioQspiSsCtrl::OEOVER_MASK)
        }
    }

    /// Set OUTOVER [13:12]
    #[inline(always)]
    pub const fn with_outover(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GpioQspiSsCtrl::OUTOVER_MASK) | ((value << GpioQspiSsCtrl::OUTOVER_OFFSET) & GpioQspiSsCtrl::OUTOVER_MASK)
        }
    }

    /// Set FUNCSEL [4:0]: 0-31 -> selects pin function according to the gpio table 31 == NULL
    #[inline(always)]
    pub const fn with_funcsel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GpioQspiSsCtrl::FUNCSEL_MASK) | ((value << GpioQspiSsCtrl::FUNCSEL_OFFSET) & GpioQspiSsCtrl::FUNCSEL_MASK)
        }
    }

}

impl Default for GpioQspiSsCtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl GpioQspiSsCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> GpioQspiSsCtrlVal {
        GpioQspiSsCtrlVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: GpioQspiSsCtrlVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(GpioQspiSsCtrlVal) -> GpioQspiSsCtrlVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// GPIO_QSPI_SD0_STATUS Register
///
/// Address offset: 0x0020
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **IRQTOPROC** [26] (RO): interrupt to processors, after override is applied
/// - **INFROMPAD** [17] (RO): input signal from pad, before filtering and override are applied
/// - **OETOPAD** [13] (RO): output enable to pad after register override is applied
/// - **OUTTOPAD** [9] (RO): output signal to pad after register override is applied
#[repr(transparent)]
pub struct GpioQspiSd0Status {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for GpioQspiSd0Status {}
unsafe impl Sync for GpioQspiSd0Status {}

impl GpioQspiSd0Status {
    // ======== Field Constants ========

    /// IRQTOPROC - interrupt to processors, after override is applied
    /// Bits [26:26], mask 0x04000000
    pub const IRQTOPROC_OFFSET: u32 = 26;
    pub const IRQTOPROC_MASK: u32 = 0x04000000;
    pub const IRQTOPROC: u32 = 1 << 26;

    /// INFROMPAD - input signal from pad, before filtering and override are applied
    /// Bits [17:17], mask 0x00020000
    pub const INFROMPAD_OFFSET: u32 = 17;
    pub const INFROMPAD_MASK: u32 = 0x00020000;
    pub const INFROMPAD: u32 = 1 << 17;

    /// OETOPAD - output enable to pad after register override is applied
    /// Bits [13:13], mask 0x00002000
    pub const OETOPAD_OFFSET: u32 = 13;
    pub const OETOPAD_MASK: u32 = 0x00002000;
    pub const OETOPAD: u32 = 1 << 13;

    /// OUTTOPAD - output signal to pad after register override is applied
    /// Bits [9:9], mask 0x00000200
    pub const OUTTOPAD_OFFSET: u32 = 9;
    pub const OUTTOPAD_MASK: u32 = 0x00000200;
    pub const OUTTOPAD: u32 = 1 << 9;

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

    /// **IRQTOPROC** [26:26]: interrupt to processors, after override is applied
    #[inline(always)]
    pub fn irqtoproc(&self) -> bool {
        (self.read_bits() & Self::IRQTOPROC_MASK) != 0
    }

    /// **INFROMPAD** [17:17]: input signal from pad, before filtering and override are applied
    #[inline(always)]
    pub fn infrompad(&self) -> bool {
        (self.read_bits() & Self::INFROMPAD_MASK) != 0
    }

    /// **OETOPAD** [13:13]: output enable to pad after register override is applied
    #[inline(always)]
    pub fn oetopad(&self) -> bool {
        (self.read_bits() & Self::OETOPAD_MASK) != 0
    }

    /// **OUTTOPAD** [9:9]: output signal to pad after register override is applied
    #[inline(always)]
    pub fn outtopad(&self) -> bool {
        (self.read_bits() & Self::OUTTOPAD_MASK) != 0
    }

}

/// GPIO_QSPI_SD0_STATUS value type for batch read/modify/write operations
///
/// Use `gpioqspisd0status.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpioqspisd0status.write_val(val)`.
#[derive(Clone, Copy)]
pub struct GpioQspiSd0StatusVal {
    bits: u32,
}

impl GpioQspiSd0StatusVal {
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

    /// Read IRQTOPROC [26:26]: interrupt to processors, after override is applied
    #[inline(always)]
    pub const fn irqtoproc(&self) -> bool {
        (self.bits & GpioQspiSd0Status::IRQTOPROC_MASK) != 0
    }

    /// Read INFROMPAD [17:17]: input signal from pad, before filtering and override are applied
    #[inline(always)]
    pub const fn infrompad(&self) -> bool {
        (self.bits & GpioQspiSd0Status::INFROMPAD_MASK) != 0
    }

    /// Read OETOPAD [13:13]: output enable to pad after register override is applied
    #[inline(always)]
    pub const fn oetopad(&self) -> bool {
        (self.bits & GpioQspiSd0Status::OETOPAD_MASK) != 0
    }

    /// Read OUTTOPAD [9:9]: output signal to pad after register override is applied
    #[inline(always)]
    pub const fn outtopad(&self) -> bool {
        (self.bits & GpioQspiSd0Status::OUTTOPAD_MASK) != 0
    }

}

impl Default for GpioQspiSd0StatusVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl GpioQspiSd0Status {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> GpioQspiSd0StatusVal {
        GpioQspiSd0StatusVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: GpioQspiSd0StatusVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(GpioQspiSd0StatusVal) -> GpioQspiSd0StatusVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// GPIO_QSPI_SD0_CTRL Register
///
/// Address offset: 0x0024
/// Reset value: 0x0000001f
///
/// ## Fields
///
/// - **IRQOVER** [29:28] (RW): 
/// - **INOVER** [17:16] (RW): 
/// - **OEOVER** [15:14] (RW): 
/// - **OUTOVER** [13:12] (RW): 
/// - **FUNCSEL** [4:0] (RW): 0-31 -> selects pin function according to the gpio table 31 == NULL
#[repr(transparent)]
pub struct GpioQspiSd0Ctrl {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for GpioQspiSd0Ctrl {}
unsafe impl Sync for GpioQspiSd0Ctrl {}

impl GpioQspiSd0Ctrl {
    // ======== Field Constants ========

    /// Bits [29:28], mask 0x30000000
    pub const IRQOVER_OFFSET: u32 = 28;
    pub const IRQOVER_MASK: u32 = 0x30000000;
    /// NORMAL = 0: don't invert the interrupt
    pub const IRQOVER_NORMAL: u32 = 0 << 28;
    /// INVERT = 1: invert the interrupt
    pub const IRQOVER_INVERT: u32 = 1 << 28;
    /// LOW = 2: drive interrupt low
    pub const IRQOVER_LOW: u32 = 2 << 28;
    /// HIGH = 3: drive interrupt high
    pub const IRQOVER_HIGH: u32 = 3 << 28;

    /// Bits [17:16], mask 0x00030000
    pub const INOVER_OFFSET: u32 = 16;
    pub const INOVER_MASK: u32 = 0x00030000;
    /// NORMAL = 0: don't invert the peri input
    pub const INOVER_NORMAL: u32 = 0 << 16;
    /// INVERT = 1: invert the peri input
    pub const INOVER_INVERT: u32 = 1 << 16;
    /// LOW = 2: drive peri input low
    pub const INOVER_LOW: u32 = 2 << 16;
    /// HIGH = 3: drive peri input high
    pub const INOVER_HIGH: u32 = 3 << 16;

    /// Bits [15:14], mask 0x0000c000
    pub const OEOVER_OFFSET: u32 = 14;
    pub const OEOVER_MASK: u32 = 0x0000c000;
    /// NORMAL = 0: drive output enable from peripheral signal selected by funcsel
    pub const OEOVER_NORMAL: u32 = 0 << 14;
    /// INVERT = 1: drive output enable from inverse of peripheral signal selected by funcsel
    pub const OEOVER_INVERT: u32 = 1 << 14;
    /// DISABLE = 2: disable output
    pub const OEOVER_DISABLE: u32 = 2 << 14;
    /// ENABLE = 3: enable output
    pub const OEOVER_ENABLE: u32 = 3 << 14;

    /// Bits [13:12], mask 0x00003000
    pub const OUTOVER_OFFSET: u32 = 12;
    pub const OUTOVER_MASK: u32 = 0x00003000;
    /// NORMAL = 0: drive output from peripheral signal selected by funcsel
    pub const OUTOVER_NORMAL: u32 = 0 << 12;
    /// INVERT = 1: drive output from inverse of peripheral signal selected by funcsel
    pub const OUTOVER_INVERT: u32 = 1 << 12;
    /// LOW = 2: drive output low
    pub const OUTOVER_LOW: u32 = 2 << 12;
    /// HIGH = 3: drive output high
    pub const OUTOVER_HIGH: u32 = 3 << 12;

    /// FUNCSEL - 0-31 -> selects pin function according to the gpio table 31 == NULL
    /// Bits [4:0], mask 0x0000001f
    pub const FUNCSEL_OFFSET: u32 = 0;
    pub const FUNCSEL_MASK: u32 = 0x0000001f;
    pub const FUNCSEL_XIP_SD0: u32 = 0 << 0;
    pub const FUNCSEL_UART0_TX: u32 = 2 << 0;
    pub const FUNCSEL_I2C0_SDA: u32 = 3 << 0;
    pub const FUNCSEL_SIOB_PROC_60: u32 = 5 << 0;
    pub const FUNCSEL_NULL: u32 = 31 << 0;

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

    /// **IRQOVER** [29:28]
    #[inline(always)]
    pub fn irqover(&self) -> u32 {
        (self.read_bits() & Self::IRQOVER_MASK) >> Self::IRQOVER_OFFSET
    }

    /// Write IRQOVER field
    #[inline(always)]
    pub fn write_irqover(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::IRQOVER_MASK) | ((value << Self::IRQOVER_OFFSET) & Self::IRQOVER_MASK));
    }

    /// **INOVER** [17:16]
    #[inline(always)]
    pub fn inover(&self) -> u32 {
        (self.read_bits() & Self::INOVER_MASK) >> Self::INOVER_OFFSET
    }

    /// Write INOVER field
    #[inline(always)]
    pub fn write_inover(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::INOVER_MASK) | ((value << Self::INOVER_OFFSET) & Self::INOVER_MASK));
    }

    /// **OEOVER** [15:14]
    #[inline(always)]
    pub fn oeover(&self) -> u32 {
        (self.read_bits() & Self::OEOVER_MASK) >> Self::OEOVER_OFFSET
    }

    /// Write OEOVER field
    #[inline(always)]
    pub fn write_oeover(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::OEOVER_MASK) | ((value << Self::OEOVER_OFFSET) & Self::OEOVER_MASK));
    }

    /// **OUTOVER** [13:12]
    #[inline(always)]
    pub fn outover(&self) -> u32 {
        (self.read_bits() & Self::OUTOVER_MASK) >> Self::OUTOVER_OFFSET
    }

    /// Write OUTOVER field
    #[inline(always)]
    pub fn write_outover(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::OUTOVER_MASK) | ((value << Self::OUTOVER_OFFSET) & Self::OUTOVER_MASK));
    }

    /// **FUNCSEL** [4:0]: 0-31 -> selects pin function according to the gpio table 31 == NULL
    #[inline(always)]
    pub fn funcsel(&self) -> u32 {
        (self.read_bits() & Self::FUNCSEL_MASK) >> Self::FUNCSEL_OFFSET
    }

    /// Write FUNCSEL field
    #[inline(always)]
    pub fn write_funcsel(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::FUNCSEL_MASK) | ((value << Self::FUNCSEL_OFFSET) & Self::FUNCSEL_MASK));
    }

}

/// GPIO_QSPI_SD0_CTRL value type for batch read/modify/write operations
///
/// Use `gpioqspisd0ctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpioqspisd0ctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct GpioQspiSd0CtrlVal {
    bits: u32,
}

impl GpioQspiSd0CtrlVal {
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
        Self { bits: 0x0000001f }
    }

    /// Read IRQOVER [29:28]
    #[inline(always)]
    pub const fn irqover(&self) -> u32 {
        (self.bits & GpioQspiSd0Ctrl::IRQOVER_MASK) >> GpioQspiSd0Ctrl::IRQOVER_OFFSET
    }

    /// Read INOVER [17:16]
    #[inline(always)]
    pub const fn inover(&self) -> u32 {
        (self.bits & GpioQspiSd0Ctrl::INOVER_MASK) >> GpioQspiSd0Ctrl::INOVER_OFFSET
    }

    /// Read OEOVER [15:14]
    #[inline(always)]
    pub const fn oeover(&self) -> u32 {
        (self.bits & GpioQspiSd0Ctrl::OEOVER_MASK) >> GpioQspiSd0Ctrl::OEOVER_OFFSET
    }

    /// Read OUTOVER [13:12]
    #[inline(always)]
    pub const fn outover(&self) -> u32 {
        (self.bits & GpioQspiSd0Ctrl::OUTOVER_MASK) >> GpioQspiSd0Ctrl::OUTOVER_OFFSET
    }

    /// Read FUNCSEL [4:0]: 0-31 -> selects pin function according to the gpio table 31 == NULL
    #[inline(always)]
    pub const fn funcsel(&self) -> u32 {
        (self.bits & GpioQspiSd0Ctrl::FUNCSEL_MASK) >> GpioQspiSd0Ctrl::FUNCSEL_OFFSET
    }

    /// Set IRQOVER [29:28]
    #[inline(always)]
    pub const fn with_irqover(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GpioQspiSd0Ctrl::IRQOVER_MASK) | ((value << GpioQspiSd0Ctrl::IRQOVER_OFFSET) & GpioQspiSd0Ctrl::IRQOVER_MASK)
        }
    }

    /// Set INOVER [17:16]
    #[inline(always)]
    pub const fn with_inover(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GpioQspiSd0Ctrl::INOVER_MASK) | ((value << GpioQspiSd0Ctrl::INOVER_OFFSET) & GpioQspiSd0Ctrl::INOVER_MASK)
        }
    }

    /// Set OEOVER [15:14]
    #[inline(always)]
    pub const fn with_oeover(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GpioQspiSd0Ctrl::OEOVER_MASK) | ((value << GpioQspiSd0Ctrl::OEOVER_OFFSET) & GpioQspiSd0Ctrl::OEOVER_MASK)
        }
    }

    /// Set OUTOVER [13:12]
    #[inline(always)]
    pub const fn with_outover(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GpioQspiSd0Ctrl::OUTOVER_MASK) | ((value << GpioQspiSd0Ctrl::OUTOVER_OFFSET) & GpioQspiSd0Ctrl::OUTOVER_MASK)
        }
    }

    /// Set FUNCSEL [4:0]: 0-31 -> selects pin function according to the gpio table 31 == NULL
    #[inline(always)]
    pub const fn with_funcsel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GpioQspiSd0Ctrl::FUNCSEL_MASK) | ((value << GpioQspiSd0Ctrl::FUNCSEL_OFFSET) & GpioQspiSd0Ctrl::FUNCSEL_MASK)
        }
    }

}

impl Default for GpioQspiSd0CtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl GpioQspiSd0Ctrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> GpioQspiSd0CtrlVal {
        GpioQspiSd0CtrlVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: GpioQspiSd0CtrlVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(GpioQspiSd0CtrlVal) -> GpioQspiSd0CtrlVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// GPIO_QSPI_SD1_STATUS Register
///
/// Address offset: 0x0028
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **IRQTOPROC** [26] (RO): interrupt to processors, after override is applied
/// - **INFROMPAD** [17] (RO): input signal from pad, before filtering and override are applied
/// - **OETOPAD** [13] (RO): output enable to pad after register override is applied
/// - **OUTTOPAD** [9] (RO): output signal to pad after register override is applied
#[repr(transparent)]
pub struct GpioQspiSd1Status {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for GpioQspiSd1Status {}
unsafe impl Sync for GpioQspiSd1Status {}

impl GpioQspiSd1Status {
    // ======== Field Constants ========

    /// IRQTOPROC - interrupt to processors, after override is applied
    /// Bits [26:26], mask 0x04000000
    pub const IRQTOPROC_OFFSET: u32 = 26;
    pub const IRQTOPROC_MASK: u32 = 0x04000000;
    pub const IRQTOPROC: u32 = 1 << 26;

    /// INFROMPAD - input signal from pad, before filtering and override are applied
    /// Bits [17:17], mask 0x00020000
    pub const INFROMPAD_OFFSET: u32 = 17;
    pub const INFROMPAD_MASK: u32 = 0x00020000;
    pub const INFROMPAD: u32 = 1 << 17;

    /// OETOPAD - output enable to pad after register override is applied
    /// Bits [13:13], mask 0x00002000
    pub const OETOPAD_OFFSET: u32 = 13;
    pub const OETOPAD_MASK: u32 = 0x00002000;
    pub const OETOPAD: u32 = 1 << 13;

    /// OUTTOPAD - output signal to pad after register override is applied
    /// Bits [9:9], mask 0x00000200
    pub const OUTTOPAD_OFFSET: u32 = 9;
    pub const OUTTOPAD_MASK: u32 = 0x00000200;
    pub const OUTTOPAD: u32 = 1 << 9;

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

    /// **IRQTOPROC** [26:26]: interrupt to processors, after override is applied
    #[inline(always)]
    pub fn irqtoproc(&self) -> bool {
        (self.read_bits() & Self::IRQTOPROC_MASK) != 0
    }

    /// **INFROMPAD** [17:17]: input signal from pad, before filtering and override are applied
    #[inline(always)]
    pub fn infrompad(&self) -> bool {
        (self.read_bits() & Self::INFROMPAD_MASK) != 0
    }

    /// **OETOPAD** [13:13]: output enable to pad after register override is applied
    #[inline(always)]
    pub fn oetopad(&self) -> bool {
        (self.read_bits() & Self::OETOPAD_MASK) != 0
    }

    /// **OUTTOPAD** [9:9]: output signal to pad after register override is applied
    #[inline(always)]
    pub fn outtopad(&self) -> bool {
        (self.read_bits() & Self::OUTTOPAD_MASK) != 0
    }

}

/// GPIO_QSPI_SD1_STATUS value type for batch read/modify/write operations
///
/// Use `gpioqspisd1status.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpioqspisd1status.write_val(val)`.
#[derive(Clone, Copy)]
pub struct GpioQspiSd1StatusVal {
    bits: u32,
}

impl GpioQspiSd1StatusVal {
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

    /// Read IRQTOPROC [26:26]: interrupt to processors, after override is applied
    #[inline(always)]
    pub const fn irqtoproc(&self) -> bool {
        (self.bits & GpioQspiSd1Status::IRQTOPROC_MASK) != 0
    }

    /// Read INFROMPAD [17:17]: input signal from pad, before filtering and override are applied
    #[inline(always)]
    pub const fn infrompad(&self) -> bool {
        (self.bits & GpioQspiSd1Status::INFROMPAD_MASK) != 0
    }

    /// Read OETOPAD [13:13]: output enable to pad after register override is applied
    #[inline(always)]
    pub const fn oetopad(&self) -> bool {
        (self.bits & GpioQspiSd1Status::OETOPAD_MASK) != 0
    }

    /// Read OUTTOPAD [9:9]: output signal to pad after register override is applied
    #[inline(always)]
    pub const fn outtopad(&self) -> bool {
        (self.bits & GpioQspiSd1Status::OUTTOPAD_MASK) != 0
    }

}

impl Default for GpioQspiSd1StatusVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl GpioQspiSd1Status {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> GpioQspiSd1StatusVal {
        GpioQspiSd1StatusVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: GpioQspiSd1StatusVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(GpioQspiSd1StatusVal) -> GpioQspiSd1StatusVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// GPIO_QSPI_SD1_CTRL Register
///
/// Address offset: 0x002c
/// Reset value: 0x0000001f
///
/// ## Fields
///
/// - **IRQOVER** [29:28] (RW): 
/// - **INOVER** [17:16] (RW): 
/// - **OEOVER** [15:14] (RW): 
/// - **OUTOVER** [13:12] (RW): 
/// - **FUNCSEL** [4:0] (RW): 0-31 -> selects pin function according to the gpio table 31 == NULL
#[repr(transparent)]
pub struct GpioQspiSd1Ctrl {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for GpioQspiSd1Ctrl {}
unsafe impl Sync for GpioQspiSd1Ctrl {}

impl GpioQspiSd1Ctrl {
    // ======== Field Constants ========

    /// Bits [29:28], mask 0x30000000
    pub const IRQOVER_OFFSET: u32 = 28;
    pub const IRQOVER_MASK: u32 = 0x30000000;
    /// NORMAL = 0: don't invert the interrupt
    pub const IRQOVER_NORMAL: u32 = 0 << 28;
    /// INVERT = 1: invert the interrupt
    pub const IRQOVER_INVERT: u32 = 1 << 28;
    /// LOW = 2: drive interrupt low
    pub const IRQOVER_LOW: u32 = 2 << 28;
    /// HIGH = 3: drive interrupt high
    pub const IRQOVER_HIGH: u32 = 3 << 28;

    /// Bits [17:16], mask 0x00030000
    pub const INOVER_OFFSET: u32 = 16;
    pub const INOVER_MASK: u32 = 0x00030000;
    /// NORMAL = 0: don't invert the peri input
    pub const INOVER_NORMAL: u32 = 0 << 16;
    /// INVERT = 1: invert the peri input
    pub const INOVER_INVERT: u32 = 1 << 16;
    /// LOW = 2: drive peri input low
    pub const INOVER_LOW: u32 = 2 << 16;
    /// HIGH = 3: drive peri input high
    pub const INOVER_HIGH: u32 = 3 << 16;

    /// Bits [15:14], mask 0x0000c000
    pub const OEOVER_OFFSET: u32 = 14;
    pub const OEOVER_MASK: u32 = 0x0000c000;
    /// NORMAL = 0: drive output enable from peripheral signal selected by funcsel
    pub const OEOVER_NORMAL: u32 = 0 << 14;
    /// INVERT = 1: drive output enable from inverse of peripheral signal selected by funcsel
    pub const OEOVER_INVERT: u32 = 1 << 14;
    /// DISABLE = 2: disable output
    pub const OEOVER_DISABLE: u32 = 2 << 14;
    /// ENABLE = 3: enable output
    pub const OEOVER_ENABLE: u32 = 3 << 14;

    /// Bits [13:12], mask 0x00003000
    pub const OUTOVER_OFFSET: u32 = 12;
    pub const OUTOVER_MASK: u32 = 0x00003000;
    /// NORMAL = 0: drive output from peripheral signal selected by funcsel
    pub const OUTOVER_NORMAL: u32 = 0 << 12;
    /// INVERT = 1: drive output from inverse of peripheral signal selected by funcsel
    pub const OUTOVER_INVERT: u32 = 1 << 12;
    /// LOW = 2: drive output low
    pub const OUTOVER_LOW: u32 = 2 << 12;
    /// HIGH = 3: drive output high
    pub const OUTOVER_HIGH: u32 = 3 << 12;

    /// FUNCSEL - 0-31 -> selects pin function according to the gpio table 31 == NULL
    /// Bits [4:0], mask 0x0000001f
    pub const FUNCSEL_OFFSET: u32 = 0;
    pub const FUNCSEL_MASK: u32 = 0x0000001f;
    pub const FUNCSEL_XIP_SD1: u32 = 0 << 0;
    pub const FUNCSEL_UART0_RX: u32 = 2 << 0;
    pub const FUNCSEL_I2C0_SCL: u32 = 3 << 0;
    pub const FUNCSEL_SIOB_PROC_61: u32 = 5 << 0;
    pub const FUNCSEL_NULL: u32 = 31 << 0;

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

    /// **IRQOVER** [29:28]
    #[inline(always)]
    pub fn irqover(&self) -> u32 {
        (self.read_bits() & Self::IRQOVER_MASK) >> Self::IRQOVER_OFFSET
    }

    /// Write IRQOVER field
    #[inline(always)]
    pub fn write_irqover(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::IRQOVER_MASK) | ((value << Self::IRQOVER_OFFSET) & Self::IRQOVER_MASK));
    }

    /// **INOVER** [17:16]
    #[inline(always)]
    pub fn inover(&self) -> u32 {
        (self.read_bits() & Self::INOVER_MASK) >> Self::INOVER_OFFSET
    }

    /// Write INOVER field
    #[inline(always)]
    pub fn write_inover(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::INOVER_MASK) | ((value << Self::INOVER_OFFSET) & Self::INOVER_MASK));
    }

    /// **OEOVER** [15:14]
    #[inline(always)]
    pub fn oeover(&self) -> u32 {
        (self.read_bits() & Self::OEOVER_MASK) >> Self::OEOVER_OFFSET
    }

    /// Write OEOVER field
    #[inline(always)]
    pub fn write_oeover(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::OEOVER_MASK) | ((value << Self::OEOVER_OFFSET) & Self::OEOVER_MASK));
    }

    /// **OUTOVER** [13:12]
    #[inline(always)]
    pub fn outover(&self) -> u32 {
        (self.read_bits() & Self::OUTOVER_MASK) >> Self::OUTOVER_OFFSET
    }

    /// Write OUTOVER field
    #[inline(always)]
    pub fn write_outover(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::OUTOVER_MASK) | ((value << Self::OUTOVER_OFFSET) & Self::OUTOVER_MASK));
    }

    /// **FUNCSEL** [4:0]: 0-31 -> selects pin function according to the gpio table 31 == NULL
    #[inline(always)]
    pub fn funcsel(&self) -> u32 {
        (self.read_bits() & Self::FUNCSEL_MASK) >> Self::FUNCSEL_OFFSET
    }

    /// Write FUNCSEL field
    #[inline(always)]
    pub fn write_funcsel(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::FUNCSEL_MASK) | ((value << Self::FUNCSEL_OFFSET) & Self::FUNCSEL_MASK));
    }

}

/// GPIO_QSPI_SD1_CTRL value type for batch read/modify/write operations
///
/// Use `gpioqspisd1ctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpioqspisd1ctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct GpioQspiSd1CtrlVal {
    bits: u32,
}

impl GpioQspiSd1CtrlVal {
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
        Self { bits: 0x0000001f }
    }

    /// Read IRQOVER [29:28]
    #[inline(always)]
    pub const fn irqover(&self) -> u32 {
        (self.bits & GpioQspiSd1Ctrl::IRQOVER_MASK) >> GpioQspiSd1Ctrl::IRQOVER_OFFSET
    }

    /// Read INOVER [17:16]
    #[inline(always)]
    pub const fn inover(&self) -> u32 {
        (self.bits & GpioQspiSd1Ctrl::INOVER_MASK) >> GpioQspiSd1Ctrl::INOVER_OFFSET
    }

    /// Read OEOVER [15:14]
    #[inline(always)]
    pub const fn oeover(&self) -> u32 {
        (self.bits & GpioQspiSd1Ctrl::OEOVER_MASK) >> GpioQspiSd1Ctrl::OEOVER_OFFSET
    }

    /// Read OUTOVER [13:12]
    #[inline(always)]
    pub const fn outover(&self) -> u32 {
        (self.bits & GpioQspiSd1Ctrl::OUTOVER_MASK) >> GpioQspiSd1Ctrl::OUTOVER_OFFSET
    }

    /// Read FUNCSEL [4:0]: 0-31 -> selects pin function according to the gpio table 31 == NULL
    #[inline(always)]
    pub const fn funcsel(&self) -> u32 {
        (self.bits & GpioQspiSd1Ctrl::FUNCSEL_MASK) >> GpioQspiSd1Ctrl::FUNCSEL_OFFSET
    }

    /// Set IRQOVER [29:28]
    #[inline(always)]
    pub const fn with_irqover(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GpioQspiSd1Ctrl::IRQOVER_MASK) | ((value << GpioQspiSd1Ctrl::IRQOVER_OFFSET) & GpioQspiSd1Ctrl::IRQOVER_MASK)
        }
    }

    /// Set INOVER [17:16]
    #[inline(always)]
    pub const fn with_inover(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GpioQspiSd1Ctrl::INOVER_MASK) | ((value << GpioQspiSd1Ctrl::INOVER_OFFSET) & GpioQspiSd1Ctrl::INOVER_MASK)
        }
    }

    /// Set OEOVER [15:14]
    #[inline(always)]
    pub const fn with_oeover(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GpioQspiSd1Ctrl::OEOVER_MASK) | ((value << GpioQspiSd1Ctrl::OEOVER_OFFSET) & GpioQspiSd1Ctrl::OEOVER_MASK)
        }
    }

    /// Set OUTOVER [13:12]
    #[inline(always)]
    pub const fn with_outover(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GpioQspiSd1Ctrl::OUTOVER_MASK) | ((value << GpioQspiSd1Ctrl::OUTOVER_OFFSET) & GpioQspiSd1Ctrl::OUTOVER_MASK)
        }
    }

    /// Set FUNCSEL [4:0]: 0-31 -> selects pin function according to the gpio table 31 == NULL
    #[inline(always)]
    pub const fn with_funcsel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GpioQspiSd1Ctrl::FUNCSEL_MASK) | ((value << GpioQspiSd1Ctrl::FUNCSEL_OFFSET) & GpioQspiSd1Ctrl::FUNCSEL_MASK)
        }
    }

}

impl Default for GpioQspiSd1CtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl GpioQspiSd1Ctrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> GpioQspiSd1CtrlVal {
        GpioQspiSd1CtrlVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: GpioQspiSd1CtrlVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(GpioQspiSd1CtrlVal) -> GpioQspiSd1CtrlVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// GPIO_QSPI_SD2_STATUS Register
///
/// Address offset: 0x0030
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **IRQTOPROC** [26] (RO): interrupt to processors, after override is applied
/// - **INFROMPAD** [17] (RO): input signal from pad, before filtering and override are applied
/// - **OETOPAD** [13] (RO): output enable to pad after register override is applied
/// - **OUTTOPAD** [9] (RO): output signal to pad after register override is applied
#[repr(transparent)]
pub struct GpioQspiSd2Status {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for GpioQspiSd2Status {}
unsafe impl Sync for GpioQspiSd2Status {}

impl GpioQspiSd2Status {
    // ======== Field Constants ========

    /// IRQTOPROC - interrupt to processors, after override is applied
    /// Bits [26:26], mask 0x04000000
    pub const IRQTOPROC_OFFSET: u32 = 26;
    pub const IRQTOPROC_MASK: u32 = 0x04000000;
    pub const IRQTOPROC: u32 = 1 << 26;

    /// INFROMPAD - input signal from pad, before filtering and override are applied
    /// Bits [17:17], mask 0x00020000
    pub const INFROMPAD_OFFSET: u32 = 17;
    pub const INFROMPAD_MASK: u32 = 0x00020000;
    pub const INFROMPAD: u32 = 1 << 17;

    /// OETOPAD - output enable to pad after register override is applied
    /// Bits [13:13], mask 0x00002000
    pub const OETOPAD_OFFSET: u32 = 13;
    pub const OETOPAD_MASK: u32 = 0x00002000;
    pub const OETOPAD: u32 = 1 << 13;

    /// OUTTOPAD - output signal to pad after register override is applied
    /// Bits [9:9], mask 0x00000200
    pub const OUTTOPAD_OFFSET: u32 = 9;
    pub const OUTTOPAD_MASK: u32 = 0x00000200;
    pub const OUTTOPAD: u32 = 1 << 9;

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

    /// **IRQTOPROC** [26:26]: interrupt to processors, after override is applied
    #[inline(always)]
    pub fn irqtoproc(&self) -> bool {
        (self.read_bits() & Self::IRQTOPROC_MASK) != 0
    }

    /// **INFROMPAD** [17:17]: input signal from pad, before filtering and override are applied
    #[inline(always)]
    pub fn infrompad(&self) -> bool {
        (self.read_bits() & Self::INFROMPAD_MASK) != 0
    }

    /// **OETOPAD** [13:13]: output enable to pad after register override is applied
    #[inline(always)]
    pub fn oetopad(&self) -> bool {
        (self.read_bits() & Self::OETOPAD_MASK) != 0
    }

    /// **OUTTOPAD** [9:9]: output signal to pad after register override is applied
    #[inline(always)]
    pub fn outtopad(&self) -> bool {
        (self.read_bits() & Self::OUTTOPAD_MASK) != 0
    }

}

/// GPIO_QSPI_SD2_STATUS value type for batch read/modify/write operations
///
/// Use `gpioqspisd2status.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpioqspisd2status.write_val(val)`.
#[derive(Clone, Copy)]
pub struct GpioQspiSd2StatusVal {
    bits: u32,
}

impl GpioQspiSd2StatusVal {
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

    /// Read IRQTOPROC [26:26]: interrupt to processors, after override is applied
    #[inline(always)]
    pub const fn irqtoproc(&self) -> bool {
        (self.bits & GpioQspiSd2Status::IRQTOPROC_MASK) != 0
    }

    /// Read INFROMPAD [17:17]: input signal from pad, before filtering and override are applied
    #[inline(always)]
    pub const fn infrompad(&self) -> bool {
        (self.bits & GpioQspiSd2Status::INFROMPAD_MASK) != 0
    }

    /// Read OETOPAD [13:13]: output enable to pad after register override is applied
    #[inline(always)]
    pub const fn oetopad(&self) -> bool {
        (self.bits & GpioQspiSd2Status::OETOPAD_MASK) != 0
    }

    /// Read OUTTOPAD [9:9]: output signal to pad after register override is applied
    #[inline(always)]
    pub const fn outtopad(&self) -> bool {
        (self.bits & GpioQspiSd2Status::OUTTOPAD_MASK) != 0
    }

}

impl Default for GpioQspiSd2StatusVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl GpioQspiSd2Status {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> GpioQspiSd2StatusVal {
        GpioQspiSd2StatusVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: GpioQspiSd2StatusVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(GpioQspiSd2StatusVal) -> GpioQspiSd2StatusVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// GPIO_QSPI_SD2_CTRL Register
///
/// Address offset: 0x0034
/// Reset value: 0x0000001f
///
/// ## Fields
///
/// - **IRQOVER** [29:28] (RW): 
/// - **INOVER** [17:16] (RW): 
/// - **OEOVER** [15:14] (RW): 
/// - **OUTOVER** [13:12] (RW): 
/// - **FUNCSEL** [4:0] (RW): 0-31 -> selects pin function according to the gpio table 31 == NULL
#[repr(transparent)]
pub struct GpioQspiSd2Ctrl {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for GpioQspiSd2Ctrl {}
unsafe impl Sync for GpioQspiSd2Ctrl {}

impl GpioQspiSd2Ctrl {
    // ======== Field Constants ========

    /// Bits [29:28], mask 0x30000000
    pub const IRQOVER_OFFSET: u32 = 28;
    pub const IRQOVER_MASK: u32 = 0x30000000;
    /// NORMAL = 0: don't invert the interrupt
    pub const IRQOVER_NORMAL: u32 = 0 << 28;
    /// INVERT = 1: invert the interrupt
    pub const IRQOVER_INVERT: u32 = 1 << 28;
    /// LOW = 2: drive interrupt low
    pub const IRQOVER_LOW: u32 = 2 << 28;
    /// HIGH = 3: drive interrupt high
    pub const IRQOVER_HIGH: u32 = 3 << 28;

    /// Bits [17:16], mask 0x00030000
    pub const INOVER_OFFSET: u32 = 16;
    pub const INOVER_MASK: u32 = 0x00030000;
    /// NORMAL = 0: don't invert the peri input
    pub const INOVER_NORMAL: u32 = 0 << 16;
    /// INVERT = 1: invert the peri input
    pub const INOVER_INVERT: u32 = 1 << 16;
    /// LOW = 2: drive peri input low
    pub const INOVER_LOW: u32 = 2 << 16;
    /// HIGH = 3: drive peri input high
    pub const INOVER_HIGH: u32 = 3 << 16;

    /// Bits [15:14], mask 0x0000c000
    pub const OEOVER_OFFSET: u32 = 14;
    pub const OEOVER_MASK: u32 = 0x0000c000;
    /// NORMAL = 0: drive output enable from peripheral signal selected by funcsel
    pub const OEOVER_NORMAL: u32 = 0 << 14;
    /// INVERT = 1: drive output enable from inverse of peripheral signal selected by funcsel
    pub const OEOVER_INVERT: u32 = 1 << 14;
    /// DISABLE = 2: disable output
    pub const OEOVER_DISABLE: u32 = 2 << 14;
    /// ENABLE = 3: enable output
    pub const OEOVER_ENABLE: u32 = 3 << 14;

    /// Bits [13:12], mask 0x00003000
    pub const OUTOVER_OFFSET: u32 = 12;
    pub const OUTOVER_MASK: u32 = 0x00003000;
    /// NORMAL = 0: drive output from peripheral signal selected by funcsel
    pub const OUTOVER_NORMAL: u32 = 0 << 12;
    /// INVERT = 1: drive output from inverse of peripheral signal selected by funcsel
    pub const OUTOVER_INVERT: u32 = 1 << 12;
    /// LOW = 2: drive output low
    pub const OUTOVER_LOW: u32 = 2 << 12;
    /// HIGH = 3: drive output high
    pub const OUTOVER_HIGH: u32 = 3 << 12;

    /// FUNCSEL - 0-31 -> selects pin function according to the gpio table 31 == NULL
    /// Bits [4:0], mask 0x0000001f
    pub const FUNCSEL_OFFSET: u32 = 0;
    pub const FUNCSEL_MASK: u32 = 0x0000001f;
    pub const FUNCSEL_XIP_SD2: u32 = 0 << 0;
    pub const FUNCSEL_UART0_CTS: u32 = 2 << 0;
    pub const FUNCSEL_I2C1_SDA: u32 = 3 << 0;
    pub const FUNCSEL_SIOB_PROC_62: u32 = 5 << 0;
    pub const FUNCSEL_UART0_TX: u32 = 11 << 0;
    pub const FUNCSEL_NULL: u32 = 31 << 0;

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

    /// **IRQOVER** [29:28]
    #[inline(always)]
    pub fn irqover(&self) -> u32 {
        (self.read_bits() & Self::IRQOVER_MASK) >> Self::IRQOVER_OFFSET
    }

    /// Write IRQOVER field
    #[inline(always)]
    pub fn write_irqover(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::IRQOVER_MASK) | ((value << Self::IRQOVER_OFFSET) & Self::IRQOVER_MASK));
    }

    /// **INOVER** [17:16]
    #[inline(always)]
    pub fn inover(&self) -> u32 {
        (self.read_bits() & Self::INOVER_MASK) >> Self::INOVER_OFFSET
    }

    /// Write INOVER field
    #[inline(always)]
    pub fn write_inover(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::INOVER_MASK) | ((value << Self::INOVER_OFFSET) & Self::INOVER_MASK));
    }

    /// **OEOVER** [15:14]
    #[inline(always)]
    pub fn oeover(&self) -> u32 {
        (self.read_bits() & Self::OEOVER_MASK) >> Self::OEOVER_OFFSET
    }

    /// Write OEOVER field
    #[inline(always)]
    pub fn write_oeover(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::OEOVER_MASK) | ((value << Self::OEOVER_OFFSET) & Self::OEOVER_MASK));
    }

    /// **OUTOVER** [13:12]
    #[inline(always)]
    pub fn outover(&self) -> u32 {
        (self.read_bits() & Self::OUTOVER_MASK) >> Self::OUTOVER_OFFSET
    }

    /// Write OUTOVER field
    #[inline(always)]
    pub fn write_outover(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::OUTOVER_MASK) | ((value << Self::OUTOVER_OFFSET) & Self::OUTOVER_MASK));
    }

    /// **FUNCSEL** [4:0]: 0-31 -> selects pin function according to the gpio table 31 == NULL
    #[inline(always)]
    pub fn funcsel(&self) -> u32 {
        (self.read_bits() & Self::FUNCSEL_MASK) >> Self::FUNCSEL_OFFSET
    }

    /// Write FUNCSEL field
    #[inline(always)]
    pub fn write_funcsel(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::FUNCSEL_MASK) | ((value << Self::FUNCSEL_OFFSET) & Self::FUNCSEL_MASK));
    }

}

/// GPIO_QSPI_SD2_CTRL value type for batch read/modify/write operations
///
/// Use `gpioqspisd2ctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpioqspisd2ctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct GpioQspiSd2CtrlVal {
    bits: u32,
}

impl GpioQspiSd2CtrlVal {
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
        Self { bits: 0x0000001f }
    }

    /// Read IRQOVER [29:28]
    #[inline(always)]
    pub const fn irqover(&self) -> u32 {
        (self.bits & GpioQspiSd2Ctrl::IRQOVER_MASK) >> GpioQspiSd2Ctrl::IRQOVER_OFFSET
    }

    /// Read INOVER [17:16]
    #[inline(always)]
    pub const fn inover(&self) -> u32 {
        (self.bits & GpioQspiSd2Ctrl::INOVER_MASK) >> GpioQspiSd2Ctrl::INOVER_OFFSET
    }

    /// Read OEOVER [15:14]
    #[inline(always)]
    pub const fn oeover(&self) -> u32 {
        (self.bits & GpioQspiSd2Ctrl::OEOVER_MASK) >> GpioQspiSd2Ctrl::OEOVER_OFFSET
    }

    /// Read OUTOVER [13:12]
    #[inline(always)]
    pub const fn outover(&self) -> u32 {
        (self.bits & GpioQspiSd2Ctrl::OUTOVER_MASK) >> GpioQspiSd2Ctrl::OUTOVER_OFFSET
    }

    /// Read FUNCSEL [4:0]: 0-31 -> selects pin function according to the gpio table 31 == NULL
    #[inline(always)]
    pub const fn funcsel(&self) -> u32 {
        (self.bits & GpioQspiSd2Ctrl::FUNCSEL_MASK) >> GpioQspiSd2Ctrl::FUNCSEL_OFFSET
    }

    /// Set IRQOVER [29:28]
    #[inline(always)]
    pub const fn with_irqover(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GpioQspiSd2Ctrl::IRQOVER_MASK) | ((value << GpioQspiSd2Ctrl::IRQOVER_OFFSET) & GpioQspiSd2Ctrl::IRQOVER_MASK)
        }
    }

    /// Set INOVER [17:16]
    #[inline(always)]
    pub const fn with_inover(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GpioQspiSd2Ctrl::INOVER_MASK) | ((value << GpioQspiSd2Ctrl::INOVER_OFFSET) & GpioQspiSd2Ctrl::INOVER_MASK)
        }
    }

    /// Set OEOVER [15:14]
    #[inline(always)]
    pub const fn with_oeover(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GpioQspiSd2Ctrl::OEOVER_MASK) | ((value << GpioQspiSd2Ctrl::OEOVER_OFFSET) & GpioQspiSd2Ctrl::OEOVER_MASK)
        }
    }

    /// Set OUTOVER [13:12]
    #[inline(always)]
    pub const fn with_outover(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GpioQspiSd2Ctrl::OUTOVER_MASK) | ((value << GpioQspiSd2Ctrl::OUTOVER_OFFSET) & GpioQspiSd2Ctrl::OUTOVER_MASK)
        }
    }

    /// Set FUNCSEL [4:0]: 0-31 -> selects pin function according to the gpio table 31 == NULL
    #[inline(always)]
    pub const fn with_funcsel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GpioQspiSd2Ctrl::FUNCSEL_MASK) | ((value << GpioQspiSd2Ctrl::FUNCSEL_OFFSET) & GpioQspiSd2Ctrl::FUNCSEL_MASK)
        }
    }

}

impl Default for GpioQspiSd2CtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl GpioQspiSd2Ctrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> GpioQspiSd2CtrlVal {
        GpioQspiSd2CtrlVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: GpioQspiSd2CtrlVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(GpioQspiSd2CtrlVal) -> GpioQspiSd2CtrlVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// GPIO_QSPI_SD3_STATUS Register
///
/// Address offset: 0x0038
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **IRQTOPROC** [26] (RO): interrupt to processors, after override is applied
/// - **INFROMPAD** [17] (RO): input signal from pad, before filtering and override are applied
/// - **OETOPAD** [13] (RO): output enable to pad after register override is applied
/// - **OUTTOPAD** [9] (RO): output signal to pad after register override is applied
#[repr(transparent)]
pub struct GpioQspiSd3Status {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for GpioQspiSd3Status {}
unsafe impl Sync for GpioQspiSd3Status {}

impl GpioQspiSd3Status {
    // ======== Field Constants ========

    /// IRQTOPROC - interrupt to processors, after override is applied
    /// Bits [26:26], mask 0x04000000
    pub const IRQTOPROC_OFFSET: u32 = 26;
    pub const IRQTOPROC_MASK: u32 = 0x04000000;
    pub const IRQTOPROC: u32 = 1 << 26;

    /// INFROMPAD - input signal from pad, before filtering and override are applied
    /// Bits [17:17], mask 0x00020000
    pub const INFROMPAD_OFFSET: u32 = 17;
    pub const INFROMPAD_MASK: u32 = 0x00020000;
    pub const INFROMPAD: u32 = 1 << 17;

    /// OETOPAD - output enable to pad after register override is applied
    /// Bits [13:13], mask 0x00002000
    pub const OETOPAD_OFFSET: u32 = 13;
    pub const OETOPAD_MASK: u32 = 0x00002000;
    pub const OETOPAD: u32 = 1 << 13;

    /// OUTTOPAD - output signal to pad after register override is applied
    /// Bits [9:9], mask 0x00000200
    pub const OUTTOPAD_OFFSET: u32 = 9;
    pub const OUTTOPAD_MASK: u32 = 0x00000200;
    pub const OUTTOPAD: u32 = 1 << 9;

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

    /// **IRQTOPROC** [26:26]: interrupt to processors, after override is applied
    #[inline(always)]
    pub fn irqtoproc(&self) -> bool {
        (self.read_bits() & Self::IRQTOPROC_MASK) != 0
    }

    /// **INFROMPAD** [17:17]: input signal from pad, before filtering and override are applied
    #[inline(always)]
    pub fn infrompad(&self) -> bool {
        (self.read_bits() & Self::INFROMPAD_MASK) != 0
    }

    /// **OETOPAD** [13:13]: output enable to pad after register override is applied
    #[inline(always)]
    pub fn oetopad(&self) -> bool {
        (self.read_bits() & Self::OETOPAD_MASK) != 0
    }

    /// **OUTTOPAD** [9:9]: output signal to pad after register override is applied
    #[inline(always)]
    pub fn outtopad(&self) -> bool {
        (self.read_bits() & Self::OUTTOPAD_MASK) != 0
    }

}

/// GPIO_QSPI_SD3_STATUS value type for batch read/modify/write operations
///
/// Use `gpioqspisd3status.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpioqspisd3status.write_val(val)`.
#[derive(Clone, Copy)]
pub struct GpioQspiSd3StatusVal {
    bits: u32,
}

impl GpioQspiSd3StatusVal {
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

    /// Read IRQTOPROC [26:26]: interrupt to processors, after override is applied
    #[inline(always)]
    pub const fn irqtoproc(&self) -> bool {
        (self.bits & GpioQspiSd3Status::IRQTOPROC_MASK) != 0
    }

    /// Read INFROMPAD [17:17]: input signal from pad, before filtering and override are applied
    #[inline(always)]
    pub const fn infrompad(&self) -> bool {
        (self.bits & GpioQspiSd3Status::INFROMPAD_MASK) != 0
    }

    /// Read OETOPAD [13:13]: output enable to pad after register override is applied
    #[inline(always)]
    pub const fn oetopad(&self) -> bool {
        (self.bits & GpioQspiSd3Status::OETOPAD_MASK) != 0
    }

    /// Read OUTTOPAD [9:9]: output signal to pad after register override is applied
    #[inline(always)]
    pub const fn outtopad(&self) -> bool {
        (self.bits & GpioQspiSd3Status::OUTTOPAD_MASK) != 0
    }

}

impl Default for GpioQspiSd3StatusVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl GpioQspiSd3Status {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> GpioQspiSd3StatusVal {
        GpioQspiSd3StatusVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: GpioQspiSd3StatusVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(GpioQspiSd3StatusVal) -> GpioQspiSd3StatusVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// GPIO_QSPI_SD3_CTRL Register
///
/// Address offset: 0x003c
/// Reset value: 0x0000001f
///
/// ## Fields
///
/// - **IRQOVER** [29:28] (RW): 
/// - **INOVER** [17:16] (RW): 
/// - **OEOVER** [15:14] (RW): 
/// - **OUTOVER** [13:12] (RW): 
/// - **FUNCSEL** [4:0] (RW): 0-31 -> selects pin function according to the gpio table 31 == NULL
#[repr(transparent)]
pub struct GpioQspiSd3Ctrl {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for GpioQspiSd3Ctrl {}
unsafe impl Sync for GpioQspiSd3Ctrl {}

impl GpioQspiSd3Ctrl {
    // ======== Field Constants ========

    /// Bits [29:28], mask 0x30000000
    pub const IRQOVER_OFFSET: u32 = 28;
    pub const IRQOVER_MASK: u32 = 0x30000000;
    /// NORMAL = 0: don't invert the interrupt
    pub const IRQOVER_NORMAL: u32 = 0 << 28;
    /// INVERT = 1: invert the interrupt
    pub const IRQOVER_INVERT: u32 = 1 << 28;
    /// LOW = 2: drive interrupt low
    pub const IRQOVER_LOW: u32 = 2 << 28;
    /// HIGH = 3: drive interrupt high
    pub const IRQOVER_HIGH: u32 = 3 << 28;

    /// Bits [17:16], mask 0x00030000
    pub const INOVER_OFFSET: u32 = 16;
    pub const INOVER_MASK: u32 = 0x00030000;
    /// NORMAL = 0: don't invert the peri input
    pub const INOVER_NORMAL: u32 = 0 << 16;
    /// INVERT = 1: invert the peri input
    pub const INOVER_INVERT: u32 = 1 << 16;
    /// LOW = 2: drive peri input low
    pub const INOVER_LOW: u32 = 2 << 16;
    /// HIGH = 3: drive peri input high
    pub const INOVER_HIGH: u32 = 3 << 16;

    /// Bits [15:14], mask 0x0000c000
    pub const OEOVER_OFFSET: u32 = 14;
    pub const OEOVER_MASK: u32 = 0x0000c000;
    /// NORMAL = 0: drive output enable from peripheral signal selected by funcsel
    pub const OEOVER_NORMAL: u32 = 0 << 14;
    /// INVERT = 1: drive output enable from inverse of peripheral signal selected by funcsel
    pub const OEOVER_INVERT: u32 = 1 << 14;
    /// DISABLE = 2: disable output
    pub const OEOVER_DISABLE: u32 = 2 << 14;
    /// ENABLE = 3: enable output
    pub const OEOVER_ENABLE: u32 = 3 << 14;

    /// Bits [13:12], mask 0x00003000
    pub const OUTOVER_OFFSET: u32 = 12;
    pub const OUTOVER_MASK: u32 = 0x00003000;
    /// NORMAL = 0: drive output from peripheral signal selected by funcsel
    pub const OUTOVER_NORMAL: u32 = 0 << 12;
    /// INVERT = 1: drive output from inverse of peripheral signal selected by funcsel
    pub const OUTOVER_INVERT: u32 = 1 << 12;
    /// LOW = 2: drive output low
    pub const OUTOVER_LOW: u32 = 2 << 12;
    /// HIGH = 3: drive output high
    pub const OUTOVER_HIGH: u32 = 3 << 12;

    /// FUNCSEL - 0-31 -> selects pin function according to the gpio table 31 == NULL
    /// Bits [4:0], mask 0x0000001f
    pub const FUNCSEL_OFFSET: u32 = 0;
    pub const FUNCSEL_MASK: u32 = 0x0000001f;
    pub const FUNCSEL_XIP_SD3: u32 = 0 << 0;
    pub const FUNCSEL_UART0_RTS: u32 = 2 << 0;
    pub const FUNCSEL_I2C1_SCL: u32 = 3 << 0;
    pub const FUNCSEL_SIOB_PROC_63: u32 = 5 << 0;
    pub const FUNCSEL_UART0_RX: u32 = 11 << 0;
    pub const FUNCSEL_NULL: u32 = 31 << 0;

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

    /// **IRQOVER** [29:28]
    #[inline(always)]
    pub fn irqover(&self) -> u32 {
        (self.read_bits() & Self::IRQOVER_MASK) >> Self::IRQOVER_OFFSET
    }

    /// Write IRQOVER field
    #[inline(always)]
    pub fn write_irqover(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::IRQOVER_MASK) | ((value << Self::IRQOVER_OFFSET) & Self::IRQOVER_MASK));
    }

    /// **INOVER** [17:16]
    #[inline(always)]
    pub fn inover(&self) -> u32 {
        (self.read_bits() & Self::INOVER_MASK) >> Self::INOVER_OFFSET
    }

    /// Write INOVER field
    #[inline(always)]
    pub fn write_inover(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::INOVER_MASK) | ((value << Self::INOVER_OFFSET) & Self::INOVER_MASK));
    }

    /// **OEOVER** [15:14]
    #[inline(always)]
    pub fn oeover(&self) -> u32 {
        (self.read_bits() & Self::OEOVER_MASK) >> Self::OEOVER_OFFSET
    }

    /// Write OEOVER field
    #[inline(always)]
    pub fn write_oeover(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::OEOVER_MASK) | ((value << Self::OEOVER_OFFSET) & Self::OEOVER_MASK));
    }

    /// **OUTOVER** [13:12]
    #[inline(always)]
    pub fn outover(&self) -> u32 {
        (self.read_bits() & Self::OUTOVER_MASK) >> Self::OUTOVER_OFFSET
    }

    /// Write OUTOVER field
    #[inline(always)]
    pub fn write_outover(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::OUTOVER_MASK) | ((value << Self::OUTOVER_OFFSET) & Self::OUTOVER_MASK));
    }

    /// **FUNCSEL** [4:0]: 0-31 -> selects pin function according to the gpio table 31 == NULL
    #[inline(always)]
    pub fn funcsel(&self) -> u32 {
        (self.read_bits() & Self::FUNCSEL_MASK) >> Self::FUNCSEL_OFFSET
    }

    /// Write FUNCSEL field
    #[inline(always)]
    pub fn write_funcsel(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::FUNCSEL_MASK) | ((value << Self::FUNCSEL_OFFSET) & Self::FUNCSEL_MASK));
    }

}

/// GPIO_QSPI_SD3_CTRL value type for batch read/modify/write operations
///
/// Use `gpioqspisd3ctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpioqspisd3ctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct GpioQspiSd3CtrlVal {
    bits: u32,
}

impl GpioQspiSd3CtrlVal {
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
        Self { bits: 0x0000001f }
    }

    /// Read IRQOVER [29:28]
    #[inline(always)]
    pub const fn irqover(&self) -> u32 {
        (self.bits & GpioQspiSd3Ctrl::IRQOVER_MASK) >> GpioQspiSd3Ctrl::IRQOVER_OFFSET
    }

    /// Read INOVER [17:16]
    #[inline(always)]
    pub const fn inover(&self) -> u32 {
        (self.bits & GpioQspiSd3Ctrl::INOVER_MASK) >> GpioQspiSd3Ctrl::INOVER_OFFSET
    }

    /// Read OEOVER [15:14]
    #[inline(always)]
    pub const fn oeover(&self) -> u32 {
        (self.bits & GpioQspiSd3Ctrl::OEOVER_MASK) >> GpioQspiSd3Ctrl::OEOVER_OFFSET
    }

    /// Read OUTOVER [13:12]
    #[inline(always)]
    pub const fn outover(&self) -> u32 {
        (self.bits & GpioQspiSd3Ctrl::OUTOVER_MASK) >> GpioQspiSd3Ctrl::OUTOVER_OFFSET
    }

    /// Read FUNCSEL [4:0]: 0-31 -> selects pin function according to the gpio table 31 == NULL
    #[inline(always)]
    pub const fn funcsel(&self) -> u32 {
        (self.bits & GpioQspiSd3Ctrl::FUNCSEL_MASK) >> GpioQspiSd3Ctrl::FUNCSEL_OFFSET
    }

    /// Set IRQOVER [29:28]
    #[inline(always)]
    pub const fn with_irqover(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GpioQspiSd3Ctrl::IRQOVER_MASK) | ((value << GpioQspiSd3Ctrl::IRQOVER_OFFSET) & GpioQspiSd3Ctrl::IRQOVER_MASK)
        }
    }

    /// Set INOVER [17:16]
    #[inline(always)]
    pub const fn with_inover(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GpioQspiSd3Ctrl::INOVER_MASK) | ((value << GpioQspiSd3Ctrl::INOVER_OFFSET) & GpioQspiSd3Ctrl::INOVER_MASK)
        }
    }

    /// Set OEOVER [15:14]
    #[inline(always)]
    pub const fn with_oeover(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GpioQspiSd3Ctrl::OEOVER_MASK) | ((value << GpioQspiSd3Ctrl::OEOVER_OFFSET) & GpioQspiSd3Ctrl::OEOVER_MASK)
        }
    }

    /// Set OUTOVER [13:12]
    #[inline(always)]
    pub const fn with_outover(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GpioQspiSd3Ctrl::OUTOVER_MASK) | ((value << GpioQspiSd3Ctrl::OUTOVER_OFFSET) & GpioQspiSd3Ctrl::OUTOVER_MASK)
        }
    }

    /// Set FUNCSEL [4:0]: 0-31 -> selects pin function according to the gpio table 31 == NULL
    #[inline(always)]
    pub const fn with_funcsel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GpioQspiSd3Ctrl::FUNCSEL_MASK) | ((value << GpioQspiSd3Ctrl::FUNCSEL_OFFSET) & GpioQspiSd3Ctrl::FUNCSEL_MASK)
        }
    }

}

impl Default for GpioQspiSd3CtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl GpioQspiSd3Ctrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> GpioQspiSd3CtrlVal {
        GpioQspiSd3CtrlVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: GpioQspiSd3CtrlVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(GpioQspiSd3CtrlVal) -> GpioQspiSd3CtrlVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// IRQSUMMARY_PROC0_SECURE Register
///
/// Address offset: 0x0200
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **GPIO_QSPI_SD3** [7] (RO): 
/// - **GPIO_QSPI_SD2** [6] (RO): 
/// - **GPIO_QSPI_SD1** [5] (RO): 
/// - **GPIO_QSPI_SD0** [4] (RO): 
/// - **GPIO_QSPI_SS** [3] (RO): 
/// - **GPIO_QSPI_SCLK** [2] (RO): 
/// - **USBPHY_DM** [1] (RO): 
/// - **USBPHY_DP** [0] (RO): 
#[repr(transparent)]
pub struct IrqsummaryProc0Secure {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for IrqsummaryProc0Secure {}
unsafe impl Sync for IrqsummaryProc0Secure {}

impl IrqsummaryProc0Secure {
    // ======== Field Constants ========

    /// Bits [7:7], mask 0x00000080
    pub const GPIO_QSPI_SD3_OFFSET: u32 = 7;
    pub const GPIO_QSPI_SD3_MASK: u32 = 0x00000080;
    pub const GPIO_QSPI_SD3: u32 = 1 << 7;

    /// Bits [6:6], mask 0x00000040
    pub const GPIO_QSPI_SD2_OFFSET: u32 = 6;
    pub const GPIO_QSPI_SD2_MASK: u32 = 0x00000040;
    pub const GPIO_QSPI_SD2: u32 = 1 << 6;

    /// Bits [5:5], mask 0x00000020
    pub const GPIO_QSPI_SD1_OFFSET: u32 = 5;
    pub const GPIO_QSPI_SD1_MASK: u32 = 0x00000020;
    pub const GPIO_QSPI_SD1: u32 = 1 << 5;

    /// Bits [4:4], mask 0x00000010
    pub const GPIO_QSPI_SD0_OFFSET: u32 = 4;
    pub const GPIO_QSPI_SD0_MASK: u32 = 0x00000010;
    pub const GPIO_QSPI_SD0: u32 = 1 << 4;

    /// Bits [3:3], mask 0x00000008
    pub const GPIO_QSPI_SS_OFFSET: u32 = 3;
    pub const GPIO_QSPI_SS_MASK: u32 = 0x00000008;
    pub const GPIO_QSPI_SS: u32 = 1 << 3;

    /// Bits [2:2], mask 0x00000004
    pub const GPIO_QSPI_SCLK_OFFSET: u32 = 2;
    pub const GPIO_QSPI_SCLK_MASK: u32 = 0x00000004;
    pub const GPIO_QSPI_SCLK: u32 = 1 << 2;

    /// Bits [1:1], mask 0x00000002
    pub const USBPHY_DM_OFFSET: u32 = 1;
    pub const USBPHY_DM_MASK: u32 = 0x00000002;
    pub const USBPHY_DM: u32 = 1 << 1;

    /// Bits [0:0], mask 0x00000001
    pub const USBPHY_DP_OFFSET: u32 = 0;
    pub const USBPHY_DP_MASK: u32 = 0x00000001;
    pub const USBPHY_DP: u32 = 1 << 0;

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

    /// **GPIO_QSPI_SD3** [7:7]
    #[inline(always)]
    pub fn gpio_qspi_sd3(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD3_MASK) != 0
    }

    /// **GPIO_QSPI_SD2** [6:6]
    #[inline(always)]
    pub fn gpio_qspi_sd2(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD2_MASK) != 0
    }

    /// **GPIO_QSPI_SD1** [5:5]
    #[inline(always)]
    pub fn gpio_qspi_sd1(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD1_MASK) != 0
    }

    /// **GPIO_QSPI_SD0** [4:4]
    #[inline(always)]
    pub fn gpio_qspi_sd0(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD0_MASK) != 0
    }

    /// **GPIO_QSPI_SS** [3:3]
    #[inline(always)]
    pub fn gpio_qspi_ss(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SS_MASK) != 0
    }

    /// **GPIO_QSPI_SCLK** [2:2]
    #[inline(always)]
    pub fn gpio_qspi_sclk(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SCLK_MASK) != 0
    }

    /// **USBPHY_DM** [1:1]
    #[inline(always)]
    pub fn usbphy_dm(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DM_MASK) != 0
    }

    /// **USBPHY_DP** [0:0]
    #[inline(always)]
    pub fn usbphy_dp(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DP_MASK) != 0
    }

}

/// IRQSUMMARY_PROC0_SECURE value type for batch read/modify/write operations
///
/// Use `irqsummaryproc0secure.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `irqsummaryproc0secure.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IrqsummaryProc0SecureVal {
    bits: u32,
}

impl IrqsummaryProc0SecureVal {
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

    /// Read GPIO_QSPI_SD3 [7:7]
    #[inline(always)]
    pub const fn gpio_qspi_sd3(&self) -> bool {
        (self.bits & IrqsummaryProc0Secure::GPIO_QSPI_SD3_MASK) != 0
    }

    /// Read GPIO_QSPI_SD2 [6:6]
    #[inline(always)]
    pub const fn gpio_qspi_sd2(&self) -> bool {
        (self.bits & IrqsummaryProc0Secure::GPIO_QSPI_SD2_MASK) != 0
    }

    /// Read GPIO_QSPI_SD1 [5:5]
    #[inline(always)]
    pub const fn gpio_qspi_sd1(&self) -> bool {
        (self.bits & IrqsummaryProc0Secure::GPIO_QSPI_SD1_MASK) != 0
    }

    /// Read GPIO_QSPI_SD0 [4:4]
    #[inline(always)]
    pub const fn gpio_qspi_sd0(&self) -> bool {
        (self.bits & IrqsummaryProc0Secure::GPIO_QSPI_SD0_MASK) != 0
    }

    /// Read GPIO_QSPI_SS [3:3]
    #[inline(always)]
    pub const fn gpio_qspi_ss(&self) -> bool {
        (self.bits & IrqsummaryProc0Secure::GPIO_QSPI_SS_MASK) != 0
    }

    /// Read GPIO_QSPI_SCLK [2:2]
    #[inline(always)]
    pub const fn gpio_qspi_sclk(&self) -> bool {
        (self.bits & IrqsummaryProc0Secure::GPIO_QSPI_SCLK_MASK) != 0
    }

    /// Read USBPHY_DM [1:1]
    #[inline(always)]
    pub const fn usbphy_dm(&self) -> bool {
        (self.bits & IrqsummaryProc0Secure::USBPHY_DM_MASK) != 0
    }

    /// Read USBPHY_DP [0:0]
    #[inline(always)]
    pub const fn usbphy_dp(&self) -> bool {
        (self.bits & IrqsummaryProc0Secure::USBPHY_DP_MASK) != 0
    }

}

impl Default for IrqsummaryProc0SecureVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl IrqsummaryProc0Secure {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> IrqsummaryProc0SecureVal {
        IrqsummaryProc0SecureVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: IrqsummaryProc0SecureVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(IrqsummaryProc0SecureVal) -> IrqsummaryProc0SecureVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// IRQSUMMARY_PROC0_NONSECURE Register
///
/// Address offset: 0x0204
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **GPIO_QSPI_SD3** [7] (RO): 
/// - **GPIO_QSPI_SD2** [6] (RO): 
/// - **GPIO_QSPI_SD1** [5] (RO): 
/// - **GPIO_QSPI_SD0** [4] (RO): 
/// - **GPIO_QSPI_SS** [3] (RO): 
/// - **GPIO_QSPI_SCLK** [2] (RO): 
/// - **USBPHY_DM** [1] (RO): 
/// - **USBPHY_DP** [0] (RO): 
#[repr(transparent)]
pub struct IrqsummaryProc0Nonsecure {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for IrqsummaryProc0Nonsecure {}
unsafe impl Sync for IrqsummaryProc0Nonsecure {}

impl IrqsummaryProc0Nonsecure {
    // ======== Field Constants ========

    /// Bits [7:7], mask 0x00000080
    pub const GPIO_QSPI_SD3_OFFSET: u32 = 7;
    pub const GPIO_QSPI_SD3_MASK: u32 = 0x00000080;
    pub const GPIO_QSPI_SD3: u32 = 1 << 7;

    /// Bits [6:6], mask 0x00000040
    pub const GPIO_QSPI_SD2_OFFSET: u32 = 6;
    pub const GPIO_QSPI_SD2_MASK: u32 = 0x00000040;
    pub const GPIO_QSPI_SD2: u32 = 1 << 6;

    /// Bits [5:5], mask 0x00000020
    pub const GPIO_QSPI_SD1_OFFSET: u32 = 5;
    pub const GPIO_QSPI_SD1_MASK: u32 = 0x00000020;
    pub const GPIO_QSPI_SD1: u32 = 1 << 5;

    /// Bits [4:4], mask 0x00000010
    pub const GPIO_QSPI_SD0_OFFSET: u32 = 4;
    pub const GPIO_QSPI_SD0_MASK: u32 = 0x00000010;
    pub const GPIO_QSPI_SD0: u32 = 1 << 4;

    /// Bits [3:3], mask 0x00000008
    pub const GPIO_QSPI_SS_OFFSET: u32 = 3;
    pub const GPIO_QSPI_SS_MASK: u32 = 0x00000008;
    pub const GPIO_QSPI_SS: u32 = 1 << 3;

    /// Bits [2:2], mask 0x00000004
    pub const GPIO_QSPI_SCLK_OFFSET: u32 = 2;
    pub const GPIO_QSPI_SCLK_MASK: u32 = 0x00000004;
    pub const GPIO_QSPI_SCLK: u32 = 1 << 2;

    /// Bits [1:1], mask 0x00000002
    pub const USBPHY_DM_OFFSET: u32 = 1;
    pub const USBPHY_DM_MASK: u32 = 0x00000002;
    pub const USBPHY_DM: u32 = 1 << 1;

    /// Bits [0:0], mask 0x00000001
    pub const USBPHY_DP_OFFSET: u32 = 0;
    pub const USBPHY_DP_MASK: u32 = 0x00000001;
    pub const USBPHY_DP: u32 = 1 << 0;

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

    /// **GPIO_QSPI_SD3** [7:7]
    #[inline(always)]
    pub fn gpio_qspi_sd3(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD3_MASK) != 0
    }

    /// **GPIO_QSPI_SD2** [6:6]
    #[inline(always)]
    pub fn gpio_qspi_sd2(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD2_MASK) != 0
    }

    /// **GPIO_QSPI_SD1** [5:5]
    #[inline(always)]
    pub fn gpio_qspi_sd1(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD1_MASK) != 0
    }

    /// **GPIO_QSPI_SD0** [4:4]
    #[inline(always)]
    pub fn gpio_qspi_sd0(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD0_MASK) != 0
    }

    /// **GPIO_QSPI_SS** [3:3]
    #[inline(always)]
    pub fn gpio_qspi_ss(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SS_MASK) != 0
    }

    /// **GPIO_QSPI_SCLK** [2:2]
    #[inline(always)]
    pub fn gpio_qspi_sclk(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SCLK_MASK) != 0
    }

    /// **USBPHY_DM** [1:1]
    #[inline(always)]
    pub fn usbphy_dm(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DM_MASK) != 0
    }

    /// **USBPHY_DP** [0:0]
    #[inline(always)]
    pub fn usbphy_dp(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DP_MASK) != 0
    }

}

/// IRQSUMMARY_PROC0_NONSECURE value type for batch read/modify/write operations
///
/// Use `irqsummaryproc0nonsecure.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `irqsummaryproc0nonsecure.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IrqsummaryProc0NonsecureVal {
    bits: u32,
}

impl IrqsummaryProc0NonsecureVal {
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

    /// Read GPIO_QSPI_SD3 [7:7]
    #[inline(always)]
    pub const fn gpio_qspi_sd3(&self) -> bool {
        (self.bits & IrqsummaryProc0Nonsecure::GPIO_QSPI_SD3_MASK) != 0
    }

    /// Read GPIO_QSPI_SD2 [6:6]
    #[inline(always)]
    pub const fn gpio_qspi_sd2(&self) -> bool {
        (self.bits & IrqsummaryProc0Nonsecure::GPIO_QSPI_SD2_MASK) != 0
    }

    /// Read GPIO_QSPI_SD1 [5:5]
    #[inline(always)]
    pub const fn gpio_qspi_sd1(&self) -> bool {
        (self.bits & IrqsummaryProc0Nonsecure::GPIO_QSPI_SD1_MASK) != 0
    }

    /// Read GPIO_QSPI_SD0 [4:4]
    #[inline(always)]
    pub const fn gpio_qspi_sd0(&self) -> bool {
        (self.bits & IrqsummaryProc0Nonsecure::GPIO_QSPI_SD0_MASK) != 0
    }

    /// Read GPIO_QSPI_SS [3:3]
    #[inline(always)]
    pub const fn gpio_qspi_ss(&self) -> bool {
        (self.bits & IrqsummaryProc0Nonsecure::GPIO_QSPI_SS_MASK) != 0
    }

    /// Read GPIO_QSPI_SCLK [2:2]
    #[inline(always)]
    pub const fn gpio_qspi_sclk(&self) -> bool {
        (self.bits & IrqsummaryProc0Nonsecure::GPIO_QSPI_SCLK_MASK) != 0
    }

    /// Read USBPHY_DM [1:1]
    #[inline(always)]
    pub const fn usbphy_dm(&self) -> bool {
        (self.bits & IrqsummaryProc0Nonsecure::USBPHY_DM_MASK) != 0
    }

    /// Read USBPHY_DP [0:0]
    #[inline(always)]
    pub const fn usbphy_dp(&self) -> bool {
        (self.bits & IrqsummaryProc0Nonsecure::USBPHY_DP_MASK) != 0
    }

}

impl Default for IrqsummaryProc0NonsecureVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl IrqsummaryProc0Nonsecure {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> IrqsummaryProc0NonsecureVal {
        IrqsummaryProc0NonsecureVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: IrqsummaryProc0NonsecureVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(IrqsummaryProc0NonsecureVal) -> IrqsummaryProc0NonsecureVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// IRQSUMMARY_PROC1_SECURE Register
///
/// Address offset: 0x0208
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **GPIO_QSPI_SD3** [7] (RO): 
/// - **GPIO_QSPI_SD2** [6] (RO): 
/// - **GPIO_QSPI_SD1** [5] (RO): 
/// - **GPIO_QSPI_SD0** [4] (RO): 
/// - **GPIO_QSPI_SS** [3] (RO): 
/// - **GPIO_QSPI_SCLK** [2] (RO): 
/// - **USBPHY_DM** [1] (RO): 
/// - **USBPHY_DP** [0] (RO): 
#[repr(transparent)]
pub struct IrqsummaryProc1Secure {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for IrqsummaryProc1Secure {}
unsafe impl Sync for IrqsummaryProc1Secure {}

impl IrqsummaryProc1Secure {
    // ======== Field Constants ========

    /// Bits [7:7], mask 0x00000080
    pub const GPIO_QSPI_SD3_OFFSET: u32 = 7;
    pub const GPIO_QSPI_SD3_MASK: u32 = 0x00000080;
    pub const GPIO_QSPI_SD3: u32 = 1 << 7;

    /// Bits [6:6], mask 0x00000040
    pub const GPIO_QSPI_SD2_OFFSET: u32 = 6;
    pub const GPIO_QSPI_SD2_MASK: u32 = 0x00000040;
    pub const GPIO_QSPI_SD2: u32 = 1 << 6;

    /// Bits [5:5], mask 0x00000020
    pub const GPIO_QSPI_SD1_OFFSET: u32 = 5;
    pub const GPIO_QSPI_SD1_MASK: u32 = 0x00000020;
    pub const GPIO_QSPI_SD1: u32 = 1 << 5;

    /// Bits [4:4], mask 0x00000010
    pub const GPIO_QSPI_SD0_OFFSET: u32 = 4;
    pub const GPIO_QSPI_SD0_MASK: u32 = 0x00000010;
    pub const GPIO_QSPI_SD0: u32 = 1 << 4;

    /// Bits [3:3], mask 0x00000008
    pub const GPIO_QSPI_SS_OFFSET: u32 = 3;
    pub const GPIO_QSPI_SS_MASK: u32 = 0x00000008;
    pub const GPIO_QSPI_SS: u32 = 1 << 3;

    /// Bits [2:2], mask 0x00000004
    pub const GPIO_QSPI_SCLK_OFFSET: u32 = 2;
    pub const GPIO_QSPI_SCLK_MASK: u32 = 0x00000004;
    pub const GPIO_QSPI_SCLK: u32 = 1 << 2;

    /// Bits [1:1], mask 0x00000002
    pub const USBPHY_DM_OFFSET: u32 = 1;
    pub const USBPHY_DM_MASK: u32 = 0x00000002;
    pub const USBPHY_DM: u32 = 1 << 1;

    /// Bits [0:0], mask 0x00000001
    pub const USBPHY_DP_OFFSET: u32 = 0;
    pub const USBPHY_DP_MASK: u32 = 0x00000001;
    pub const USBPHY_DP: u32 = 1 << 0;

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

    /// **GPIO_QSPI_SD3** [7:7]
    #[inline(always)]
    pub fn gpio_qspi_sd3(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD3_MASK) != 0
    }

    /// **GPIO_QSPI_SD2** [6:6]
    #[inline(always)]
    pub fn gpio_qspi_sd2(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD2_MASK) != 0
    }

    /// **GPIO_QSPI_SD1** [5:5]
    #[inline(always)]
    pub fn gpio_qspi_sd1(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD1_MASK) != 0
    }

    /// **GPIO_QSPI_SD0** [4:4]
    #[inline(always)]
    pub fn gpio_qspi_sd0(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD0_MASK) != 0
    }

    /// **GPIO_QSPI_SS** [3:3]
    #[inline(always)]
    pub fn gpio_qspi_ss(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SS_MASK) != 0
    }

    /// **GPIO_QSPI_SCLK** [2:2]
    #[inline(always)]
    pub fn gpio_qspi_sclk(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SCLK_MASK) != 0
    }

    /// **USBPHY_DM** [1:1]
    #[inline(always)]
    pub fn usbphy_dm(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DM_MASK) != 0
    }

    /// **USBPHY_DP** [0:0]
    #[inline(always)]
    pub fn usbphy_dp(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DP_MASK) != 0
    }

}

/// IRQSUMMARY_PROC1_SECURE value type for batch read/modify/write operations
///
/// Use `irqsummaryproc1secure.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `irqsummaryproc1secure.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IrqsummaryProc1SecureVal {
    bits: u32,
}

impl IrqsummaryProc1SecureVal {
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

    /// Read GPIO_QSPI_SD3 [7:7]
    #[inline(always)]
    pub const fn gpio_qspi_sd3(&self) -> bool {
        (self.bits & IrqsummaryProc1Secure::GPIO_QSPI_SD3_MASK) != 0
    }

    /// Read GPIO_QSPI_SD2 [6:6]
    #[inline(always)]
    pub const fn gpio_qspi_sd2(&self) -> bool {
        (self.bits & IrqsummaryProc1Secure::GPIO_QSPI_SD2_MASK) != 0
    }

    /// Read GPIO_QSPI_SD1 [5:5]
    #[inline(always)]
    pub const fn gpio_qspi_sd1(&self) -> bool {
        (self.bits & IrqsummaryProc1Secure::GPIO_QSPI_SD1_MASK) != 0
    }

    /// Read GPIO_QSPI_SD0 [4:4]
    #[inline(always)]
    pub const fn gpio_qspi_sd0(&self) -> bool {
        (self.bits & IrqsummaryProc1Secure::GPIO_QSPI_SD0_MASK) != 0
    }

    /// Read GPIO_QSPI_SS [3:3]
    #[inline(always)]
    pub const fn gpio_qspi_ss(&self) -> bool {
        (self.bits & IrqsummaryProc1Secure::GPIO_QSPI_SS_MASK) != 0
    }

    /// Read GPIO_QSPI_SCLK [2:2]
    #[inline(always)]
    pub const fn gpio_qspi_sclk(&self) -> bool {
        (self.bits & IrqsummaryProc1Secure::GPIO_QSPI_SCLK_MASK) != 0
    }

    /// Read USBPHY_DM [1:1]
    #[inline(always)]
    pub const fn usbphy_dm(&self) -> bool {
        (self.bits & IrqsummaryProc1Secure::USBPHY_DM_MASK) != 0
    }

    /// Read USBPHY_DP [0:0]
    #[inline(always)]
    pub const fn usbphy_dp(&self) -> bool {
        (self.bits & IrqsummaryProc1Secure::USBPHY_DP_MASK) != 0
    }

}

impl Default for IrqsummaryProc1SecureVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl IrqsummaryProc1Secure {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> IrqsummaryProc1SecureVal {
        IrqsummaryProc1SecureVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: IrqsummaryProc1SecureVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(IrqsummaryProc1SecureVal) -> IrqsummaryProc1SecureVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// IRQSUMMARY_PROC1_NONSECURE Register
///
/// Address offset: 0x020c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **GPIO_QSPI_SD3** [7] (RO): 
/// - **GPIO_QSPI_SD2** [6] (RO): 
/// - **GPIO_QSPI_SD1** [5] (RO): 
/// - **GPIO_QSPI_SD0** [4] (RO): 
/// - **GPIO_QSPI_SS** [3] (RO): 
/// - **GPIO_QSPI_SCLK** [2] (RO): 
/// - **USBPHY_DM** [1] (RO): 
/// - **USBPHY_DP** [0] (RO): 
#[repr(transparent)]
pub struct IrqsummaryProc1Nonsecure {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for IrqsummaryProc1Nonsecure {}
unsafe impl Sync for IrqsummaryProc1Nonsecure {}

impl IrqsummaryProc1Nonsecure {
    // ======== Field Constants ========

    /// Bits [7:7], mask 0x00000080
    pub const GPIO_QSPI_SD3_OFFSET: u32 = 7;
    pub const GPIO_QSPI_SD3_MASK: u32 = 0x00000080;
    pub const GPIO_QSPI_SD3: u32 = 1 << 7;

    /// Bits [6:6], mask 0x00000040
    pub const GPIO_QSPI_SD2_OFFSET: u32 = 6;
    pub const GPIO_QSPI_SD2_MASK: u32 = 0x00000040;
    pub const GPIO_QSPI_SD2: u32 = 1 << 6;

    /// Bits [5:5], mask 0x00000020
    pub const GPIO_QSPI_SD1_OFFSET: u32 = 5;
    pub const GPIO_QSPI_SD1_MASK: u32 = 0x00000020;
    pub const GPIO_QSPI_SD1: u32 = 1 << 5;

    /// Bits [4:4], mask 0x00000010
    pub const GPIO_QSPI_SD0_OFFSET: u32 = 4;
    pub const GPIO_QSPI_SD0_MASK: u32 = 0x00000010;
    pub const GPIO_QSPI_SD0: u32 = 1 << 4;

    /// Bits [3:3], mask 0x00000008
    pub const GPIO_QSPI_SS_OFFSET: u32 = 3;
    pub const GPIO_QSPI_SS_MASK: u32 = 0x00000008;
    pub const GPIO_QSPI_SS: u32 = 1 << 3;

    /// Bits [2:2], mask 0x00000004
    pub const GPIO_QSPI_SCLK_OFFSET: u32 = 2;
    pub const GPIO_QSPI_SCLK_MASK: u32 = 0x00000004;
    pub const GPIO_QSPI_SCLK: u32 = 1 << 2;

    /// Bits [1:1], mask 0x00000002
    pub const USBPHY_DM_OFFSET: u32 = 1;
    pub const USBPHY_DM_MASK: u32 = 0x00000002;
    pub const USBPHY_DM: u32 = 1 << 1;

    /// Bits [0:0], mask 0x00000001
    pub const USBPHY_DP_OFFSET: u32 = 0;
    pub const USBPHY_DP_MASK: u32 = 0x00000001;
    pub const USBPHY_DP: u32 = 1 << 0;

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

    /// **GPIO_QSPI_SD3** [7:7]
    #[inline(always)]
    pub fn gpio_qspi_sd3(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD3_MASK) != 0
    }

    /// **GPIO_QSPI_SD2** [6:6]
    #[inline(always)]
    pub fn gpio_qspi_sd2(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD2_MASK) != 0
    }

    /// **GPIO_QSPI_SD1** [5:5]
    #[inline(always)]
    pub fn gpio_qspi_sd1(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD1_MASK) != 0
    }

    /// **GPIO_QSPI_SD0** [4:4]
    #[inline(always)]
    pub fn gpio_qspi_sd0(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD0_MASK) != 0
    }

    /// **GPIO_QSPI_SS** [3:3]
    #[inline(always)]
    pub fn gpio_qspi_ss(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SS_MASK) != 0
    }

    /// **GPIO_QSPI_SCLK** [2:2]
    #[inline(always)]
    pub fn gpio_qspi_sclk(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SCLK_MASK) != 0
    }

    /// **USBPHY_DM** [1:1]
    #[inline(always)]
    pub fn usbphy_dm(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DM_MASK) != 0
    }

    /// **USBPHY_DP** [0:0]
    #[inline(always)]
    pub fn usbphy_dp(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DP_MASK) != 0
    }

}

/// IRQSUMMARY_PROC1_NONSECURE value type for batch read/modify/write operations
///
/// Use `irqsummaryproc1nonsecure.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `irqsummaryproc1nonsecure.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IrqsummaryProc1NonsecureVal {
    bits: u32,
}

impl IrqsummaryProc1NonsecureVal {
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

    /// Read GPIO_QSPI_SD3 [7:7]
    #[inline(always)]
    pub const fn gpio_qspi_sd3(&self) -> bool {
        (self.bits & IrqsummaryProc1Nonsecure::GPIO_QSPI_SD3_MASK) != 0
    }

    /// Read GPIO_QSPI_SD2 [6:6]
    #[inline(always)]
    pub const fn gpio_qspi_sd2(&self) -> bool {
        (self.bits & IrqsummaryProc1Nonsecure::GPIO_QSPI_SD2_MASK) != 0
    }

    /// Read GPIO_QSPI_SD1 [5:5]
    #[inline(always)]
    pub const fn gpio_qspi_sd1(&self) -> bool {
        (self.bits & IrqsummaryProc1Nonsecure::GPIO_QSPI_SD1_MASK) != 0
    }

    /// Read GPIO_QSPI_SD0 [4:4]
    #[inline(always)]
    pub const fn gpio_qspi_sd0(&self) -> bool {
        (self.bits & IrqsummaryProc1Nonsecure::GPIO_QSPI_SD0_MASK) != 0
    }

    /// Read GPIO_QSPI_SS [3:3]
    #[inline(always)]
    pub const fn gpio_qspi_ss(&self) -> bool {
        (self.bits & IrqsummaryProc1Nonsecure::GPIO_QSPI_SS_MASK) != 0
    }

    /// Read GPIO_QSPI_SCLK [2:2]
    #[inline(always)]
    pub const fn gpio_qspi_sclk(&self) -> bool {
        (self.bits & IrqsummaryProc1Nonsecure::GPIO_QSPI_SCLK_MASK) != 0
    }

    /// Read USBPHY_DM [1:1]
    #[inline(always)]
    pub const fn usbphy_dm(&self) -> bool {
        (self.bits & IrqsummaryProc1Nonsecure::USBPHY_DM_MASK) != 0
    }

    /// Read USBPHY_DP [0:0]
    #[inline(always)]
    pub const fn usbphy_dp(&self) -> bool {
        (self.bits & IrqsummaryProc1Nonsecure::USBPHY_DP_MASK) != 0
    }

}

impl Default for IrqsummaryProc1NonsecureVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl IrqsummaryProc1Nonsecure {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> IrqsummaryProc1NonsecureVal {
        IrqsummaryProc1NonsecureVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: IrqsummaryProc1NonsecureVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(IrqsummaryProc1NonsecureVal) -> IrqsummaryProc1NonsecureVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// IRQSUMMARY_DORMANT_WAKE_SECURE Register
///
/// Address offset: 0x0210
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **GPIO_QSPI_SD3** [7] (RO): 
/// - **GPIO_QSPI_SD2** [6] (RO): 
/// - **GPIO_QSPI_SD1** [5] (RO): 
/// - **GPIO_QSPI_SD0** [4] (RO): 
/// - **GPIO_QSPI_SS** [3] (RO): 
/// - **GPIO_QSPI_SCLK** [2] (RO): 
/// - **USBPHY_DM** [1] (RO): 
/// - **USBPHY_DP** [0] (RO): 
#[repr(transparent)]
pub struct IrqsummaryDormantWakeSecure {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for IrqsummaryDormantWakeSecure {}
unsafe impl Sync for IrqsummaryDormantWakeSecure {}

impl IrqsummaryDormantWakeSecure {
    // ======== Field Constants ========

    /// Bits [7:7], mask 0x00000080
    pub const GPIO_QSPI_SD3_OFFSET: u32 = 7;
    pub const GPIO_QSPI_SD3_MASK: u32 = 0x00000080;
    pub const GPIO_QSPI_SD3: u32 = 1 << 7;

    /// Bits [6:6], mask 0x00000040
    pub const GPIO_QSPI_SD2_OFFSET: u32 = 6;
    pub const GPIO_QSPI_SD2_MASK: u32 = 0x00000040;
    pub const GPIO_QSPI_SD2: u32 = 1 << 6;

    /// Bits [5:5], mask 0x00000020
    pub const GPIO_QSPI_SD1_OFFSET: u32 = 5;
    pub const GPIO_QSPI_SD1_MASK: u32 = 0x00000020;
    pub const GPIO_QSPI_SD1: u32 = 1 << 5;

    /// Bits [4:4], mask 0x00000010
    pub const GPIO_QSPI_SD0_OFFSET: u32 = 4;
    pub const GPIO_QSPI_SD0_MASK: u32 = 0x00000010;
    pub const GPIO_QSPI_SD0: u32 = 1 << 4;

    /// Bits [3:3], mask 0x00000008
    pub const GPIO_QSPI_SS_OFFSET: u32 = 3;
    pub const GPIO_QSPI_SS_MASK: u32 = 0x00000008;
    pub const GPIO_QSPI_SS: u32 = 1 << 3;

    /// Bits [2:2], mask 0x00000004
    pub const GPIO_QSPI_SCLK_OFFSET: u32 = 2;
    pub const GPIO_QSPI_SCLK_MASK: u32 = 0x00000004;
    pub const GPIO_QSPI_SCLK: u32 = 1 << 2;

    /// Bits [1:1], mask 0x00000002
    pub const USBPHY_DM_OFFSET: u32 = 1;
    pub const USBPHY_DM_MASK: u32 = 0x00000002;
    pub const USBPHY_DM: u32 = 1 << 1;

    /// Bits [0:0], mask 0x00000001
    pub const USBPHY_DP_OFFSET: u32 = 0;
    pub const USBPHY_DP_MASK: u32 = 0x00000001;
    pub const USBPHY_DP: u32 = 1 << 0;

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

    /// **GPIO_QSPI_SD3** [7:7]
    #[inline(always)]
    pub fn gpio_qspi_sd3(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD3_MASK) != 0
    }

    /// **GPIO_QSPI_SD2** [6:6]
    #[inline(always)]
    pub fn gpio_qspi_sd2(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD2_MASK) != 0
    }

    /// **GPIO_QSPI_SD1** [5:5]
    #[inline(always)]
    pub fn gpio_qspi_sd1(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD1_MASK) != 0
    }

    /// **GPIO_QSPI_SD0** [4:4]
    #[inline(always)]
    pub fn gpio_qspi_sd0(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD0_MASK) != 0
    }

    /// **GPIO_QSPI_SS** [3:3]
    #[inline(always)]
    pub fn gpio_qspi_ss(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SS_MASK) != 0
    }

    /// **GPIO_QSPI_SCLK** [2:2]
    #[inline(always)]
    pub fn gpio_qspi_sclk(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SCLK_MASK) != 0
    }

    /// **USBPHY_DM** [1:1]
    #[inline(always)]
    pub fn usbphy_dm(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DM_MASK) != 0
    }

    /// **USBPHY_DP** [0:0]
    #[inline(always)]
    pub fn usbphy_dp(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DP_MASK) != 0
    }

}

/// IRQSUMMARY_DORMANT_WAKE_SECURE value type for batch read/modify/write operations
///
/// Use `irqsummarydormantwakesecure.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `irqsummarydormantwakesecure.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IrqsummaryDormantWakeSecureVal {
    bits: u32,
}

impl IrqsummaryDormantWakeSecureVal {
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

    /// Read GPIO_QSPI_SD3 [7:7]
    #[inline(always)]
    pub const fn gpio_qspi_sd3(&self) -> bool {
        (self.bits & IrqsummaryDormantWakeSecure::GPIO_QSPI_SD3_MASK) != 0
    }

    /// Read GPIO_QSPI_SD2 [6:6]
    #[inline(always)]
    pub const fn gpio_qspi_sd2(&self) -> bool {
        (self.bits & IrqsummaryDormantWakeSecure::GPIO_QSPI_SD2_MASK) != 0
    }

    /// Read GPIO_QSPI_SD1 [5:5]
    #[inline(always)]
    pub const fn gpio_qspi_sd1(&self) -> bool {
        (self.bits & IrqsummaryDormantWakeSecure::GPIO_QSPI_SD1_MASK) != 0
    }

    /// Read GPIO_QSPI_SD0 [4:4]
    #[inline(always)]
    pub const fn gpio_qspi_sd0(&self) -> bool {
        (self.bits & IrqsummaryDormantWakeSecure::GPIO_QSPI_SD0_MASK) != 0
    }

    /// Read GPIO_QSPI_SS [3:3]
    #[inline(always)]
    pub const fn gpio_qspi_ss(&self) -> bool {
        (self.bits & IrqsummaryDormantWakeSecure::GPIO_QSPI_SS_MASK) != 0
    }

    /// Read GPIO_QSPI_SCLK [2:2]
    #[inline(always)]
    pub const fn gpio_qspi_sclk(&self) -> bool {
        (self.bits & IrqsummaryDormantWakeSecure::GPIO_QSPI_SCLK_MASK) != 0
    }

    /// Read USBPHY_DM [1:1]
    #[inline(always)]
    pub const fn usbphy_dm(&self) -> bool {
        (self.bits & IrqsummaryDormantWakeSecure::USBPHY_DM_MASK) != 0
    }

    /// Read USBPHY_DP [0:0]
    #[inline(always)]
    pub const fn usbphy_dp(&self) -> bool {
        (self.bits & IrqsummaryDormantWakeSecure::USBPHY_DP_MASK) != 0
    }

}

impl Default for IrqsummaryDormantWakeSecureVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl IrqsummaryDormantWakeSecure {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> IrqsummaryDormantWakeSecureVal {
        IrqsummaryDormantWakeSecureVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: IrqsummaryDormantWakeSecureVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(IrqsummaryDormantWakeSecureVal) -> IrqsummaryDormantWakeSecureVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// IRQSUMMARY_DORMANT_WAKE_NONSECURE Register
///
/// Address offset: 0x0214
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **GPIO_QSPI_SD3** [7] (RO): 
/// - **GPIO_QSPI_SD2** [6] (RO): 
/// - **GPIO_QSPI_SD1** [5] (RO): 
/// - **GPIO_QSPI_SD0** [4] (RO): 
/// - **GPIO_QSPI_SS** [3] (RO): 
/// - **GPIO_QSPI_SCLK** [2] (RO): 
/// - **USBPHY_DM** [1] (RO): 
/// - **USBPHY_DP** [0] (RO): 
#[repr(transparent)]
pub struct IrqsummaryDormantWakeNonsecure {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for IrqsummaryDormantWakeNonsecure {}
unsafe impl Sync for IrqsummaryDormantWakeNonsecure {}

impl IrqsummaryDormantWakeNonsecure {
    // ======== Field Constants ========

    /// Bits [7:7], mask 0x00000080
    pub const GPIO_QSPI_SD3_OFFSET: u32 = 7;
    pub const GPIO_QSPI_SD3_MASK: u32 = 0x00000080;
    pub const GPIO_QSPI_SD3: u32 = 1 << 7;

    /// Bits [6:6], mask 0x00000040
    pub const GPIO_QSPI_SD2_OFFSET: u32 = 6;
    pub const GPIO_QSPI_SD2_MASK: u32 = 0x00000040;
    pub const GPIO_QSPI_SD2: u32 = 1 << 6;

    /// Bits [5:5], mask 0x00000020
    pub const GPIO_QSPI_SD1_OFFSET: u32 = 5;
    pub const GPIO_QSPI_SD1_MASK: u32 = 0x00000020;
    pub const GPIO_QSPI_SD1: u32 = 1 << 5;

    /// Bits [4:4], mask 0x00000010
    pub const GPIO_QSPI_SD0_OFFSET: u32 = 4;
    pub const GPIO_QSPI_SD0_MASK: u32 = 0x00000010;
    pub const GPIO_QSPI_SD0: u32 = 1 << 4;

    /// Bits [3:3], mask 0x00000008
    pub const GPIO_QSPI_SS_OFFSET: u32 = 3;
    pub const GPIO_QSPI_SS_MASK: u32 = 0x00000008;
    pub const GPIO_QSPI_SS: u32 = 1 << 3;

    /// Bits [2:2], mask 0x00000004
    pub const GPIO_QSPI_SCLK_OFFSET: u32 = 2;
    pub const GPIO_QSPI_SCLK_MASK: u32 = 0x00000004;
    pub const GPIO_QSPI_SCLK: u32 = 1 << 2;

    /// Bits [1:1], mask 0x00000002
    pub const USBPHY_DM_OFFSET: u32 = 1;
    pub const USBPHY_DM_MASK: u32 = 0x00000002;
    pub const USBPHY_DM: u32 = 1 << 1;

    /// Bits [0:0], mask 0x00000001
    pub const USBPHY_DP_OFFSET: u32 = 0;
    pub const USBPHY_DP_MASK: u32 = 0x00000001;
    pub const USBPHY_DP: u32 = 1 << 0;

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

    /// **GPIO_QSPI_SD3** [7:7]
    #[inline(always)]
    pub fn gpio_qspi_sd3(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD3_MASK) != 0
    }

    /// **GPIO_QSPI_SD2** [6:6]
    #[inline(always)]
    pub fn gpio_qspi_sd2(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD2_MASK) != 0
    }

    /// **GPIO_QSPI_SD1** [5:5]
    #[inline(always)]
    pub fn gpio_qspi_sd1(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD1_MASK) != 0
    }

    /// **GPIO_QSPI_SD0** [4:4]
    #[inline(always)]
    pub fn gpio_qspi_sd0(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD0_MASK) != 0
    }

    /// **GPIO_QSPI_SS** [3:3]
    #[inline(always)]
    pub fn gpio_qspi_ss(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SS_MASK) != 0
    }

    /// **GPIO_QSPI_SCLK** [2:2]
    #[inline(always)]
    pub fn gpio_qspi_sclk(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SCLK_MASK) != 0
    }

    /// **USBPHY_DM** [1:1]
    #[inline(always)]
    pub fn usbphy_dm(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DM_MASK) != 0
    }

    /// **USBPHY_DP** [0:0]
    #[inline(always)]
    pub fn usbphy_dp(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DP_MASK) != 0
    }

}

/// IRQSUMMARY_DORMANT_WAKE_NONSECURE value type for batch read/modify/write operations
///
/// Use `irqsummarydormantwakenonsecure.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `irqsummarydormantwakenonsecure.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IrqsummaryDormantWakeNonsecureVal {
    bits: u32,
}

impl IrqsummaryDormantWakeNonsecureVal {
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

    /// Read GPIO_QSPI_SD3 [7:7]
    #[inline(always)]
    pub const fn gpio_qspi_sd3(&self) -> bool {
        (self.bits & IrqsummaryDormantWakeNonsecure::GPIO_QSPI_SD3_MASK) != 0
    }

    /// Read GPIO_QSPI_SD2 [6:6]
    #[inline(always)]
    pub const fn gpio_qspi_sd2(&self) -> bool {
        (self.bits & IrqsummaryDormantWakeNonsecure::GPIO_QSPI_SD2_MASK) != 0
    }

    /// Read GPIO_QSPI_SD1 [5:5]
    #[inline(always)]
    pub const fn gpio_qspi_sd1(&self) -> bool {
        (self.bits & IrqsummaryDormantWakeNonsecure::GPIO_QSPI_SD1_MASK) != 0
    }

    /// Read GPIO_QSPI_SD0 [4:4]
    #[inline(always)]
    pub const fn gpio_qspi_sd0(&self) -> bool {
        (self.bits & IrqsummaryDormantWakeNonsecure::GPIO_QSPI_SD0_MASK) != 0
    }

    /// Read GPIO_QSPI_SS [3:3]
    #[inline(always)]
    pub const fn gpio_qspi_ss(&self) -> bool {
        (self.bits & IrqsummaryDormantWakeNonsecure::GPIO_QSPI_SS_MASK) != 0
    }

    /// Read GPIO_QSPI_SCLK [2:2]
    #[inline(always)]
    pub const fn gpio_qspi_sclk(&self) -> bool {
        (self.bits & IrqsummaryDormantWakeNonsecure::GPIO_QSPI_SCLK_MASK) != 0
    }

    /// Read USBPHY_DM [1:1]
    #[inline(always)]
    pub const fn usbphy_dm(&self) -> bool {
        (self.bits & IrqsummaryDormantWakeNonsecure::USBPHY_DM_MASK) != 0
    }

    /// Read USBPHY_DP [0:0]
    #[inline(always)]
    pub const fn usbphy_dp(&self) -> bool {
        (self.bits & IrqsummaryDormantWakeNonsecure::USBPHY_DP_MASK) != 0
    }

}

impl Default for IrqsummaryDormantWakeNonsecureVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl IrqsummaryDormantWakeNonsecure {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> IrqsummaryDormantWakeNonsecureVal {
        IrqsummaryDormantWakeNonsecureVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: IrqsummaryDormantWakeNonsecureVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(IrqsummaryDormantWakeNonsecureVal) -> IrqsummaryDormantWakeNonsecureVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// INTR Register
///
/// Raw Interrupts
///
/// Address offset: 0x0218
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **GPIO_QSPI_SD3_EDGE_HIGH** [31] (RW): 
/// - **GPIO_QSPI_SD3_EDGE_LOW** [30] (RW): 
/// - **GPIO_QSPI_SD3_LEVEL_HIGH** [29] (RO): 
/// - **GPIO_QSPI_SD3_LEVEL_LOW** [28] (RO): 
/// - **GPIO_QSPI_SD2_EDGE_HIGH** [27] (RW): 
/// - **GPIO_QSPI_SD2_EDGE_LOW** [26] (RW): 
/// - **GPIO_QSPI_SD2_LEVEL_HIGH** [25] (RO): 
/// - **GPIO_QSPI_SD2_LEVEL_LOW** [24] (RO): 
/// - **GPIO_QSPI_SD1_EDGE_HIGH** [23] (RW): 
/// - **GPIO_QSPI_SD1_EDGE_LOW** [22] (RW): 
/// - **GPIO_QSPI_SD1_LEVEL_HIGH** [21] (RO): 
/// - **GPIO_QSPI_SD1_LEVEL_LOW** [20] (RO): 
/// - **GPIO_QSPI_SD0_EDGE_HIGH** [19] (RW): 
/// - **GPIO_QSPI_SD0_EDGE_LOW** [18] (RW): 
/// - **GPIO_QSPI_SD0_LEVEL_HIGH** [17] (RO): 
/// - **GPIO_QSPI_SD0_LEVEL_LOW** [16] (RO): 
/// - **GPIO_QSPI_SS_EDGE_HIGH** [15] (RW): 
/// - **GPIO_QSPI_SS_EDGE_LOW** [14] (RW): 
/// - **GPIO_QSPI_SS_LEVEL_HIGH** [13] (RO): 
/// - **GPIO_QSPI_SS_LEVEL_LOW** [12] (RO): 
/// - **GPIO_QSPI_SCLK_EDGE_HIGH** [11] (RW): 
/// - **GPIO_QSPI_SCLK_EDGE_LOW** [10] (RW): 
/// - **GPIO_QSPI_SCLK_LEVEL_HIGH** [9] (RO): 
/// - **GPIO_QSPI_SCLK_LEVEL_LOW** [8] (RO): 
/// - **USBPHY_DM_EDGE_HIGH** [7] (RW): 
/// - **USBPHY_DM_EDGE_LOW** [6] (RW): 
/// - **USBPHY_DM_LEVEL_HIGH** [5] (RO): 
/// - **USBPHY_DM_LEVEL_LOW** [4] (RO): 
/// - **USBPHY_DP_EDGE_HIGH** [3] (RW): 
/// - **USBPHY_DP_EDGE_LOW** [2] (RW): 
/// - **USBPHY_DP_LEVEL_HIGH** [1] (RO): 
/// - **USBPHY_DP_LEVEL_LOW** [0] (RO): 
#[repr(transparent)]
pub struct Intr {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for Intr {}
unsafe impl Sync for Intr {}

impl Intr {
    // ======== Field Constants ========

    /// Bits [31:31], mask 0x80000000
    pub const GPIO_QSPI_SD3_EDGE_HIGH_OFFSET: u32 = 31;
    pub const GPIO_QSPI_SD3_EDGE_HIGH_MASK: u32 = 0x80000000;
    pub const GPIO_QSPI_SD3_EDGE_HIGH: u32 = 1 << 31;

    /// Bits [30:30], mask 0x40000000
    pub const GPIO_QSPI_SD3_EDGE_LOW_OFFSET: u32 = 30;
    pub const GPIO_QSPI_SD3_EDGE_LOW_MASK: u32 = 0x40000000;
    pub const GPIO_QSPI_SD3_EDGE_LOW: u32 = 1 << 30;

    /// Bits [29:29], mask 0x20000000
    pub const GPIO_QSPI_SD3_LEVEL_HIGH_OFFSET: u32 = 29;
    pub const GPIO_QSPI_SD3_LEVEL_HIGH_MASK: u32 = 0x20000000;
    pub const GPIO_QSPI_SD3_LEVEL_HIGH: u32 = 1 << 29;

    /// Bits [28:28], mask 0x10000000
    pub const GPIO_QSPI_SD3_LEVEL_LOW_OFFSET: u32 = 28;
    pub const GPIO_QSPI_SD3_LEVEL_LOW_MASK: u32 = 0x10000000;
    pub const GPIO_QSPI_SD3_LEVEL_LOW: u32 = 1 << 28;

    /// Bits [27:27], mask 0x08000000
    pub const GPIO_QSPI_SD2_EDGE_HIGH_OFFSET: u32 = 27;
    pub const GPIO_QSPI_SD2_EDGE_HIGH_MASK: u32 = 0x08000000;
    pub const GPIO_QSPI_SD2_EDGE_HIGH: u32 = 1 << 27;

    /// Bits [26:26], mask 0x04000000
    pub const GPIO_QSPI_SD2_EDGE_LOW_OFFSET: u32 = 26;
    pub const GPIO_QSPI_SD2_EDGE_LOW_MASK: u32 = 0x04000000;
    pub const GPIO_QSPI_SD2_EDGE_LOW: u32 = 1 << 26;

    /// Bits [25:25], mask 0x02000000
    pub const GPIO_QSPI_SD2_LEVEL_HIGH_OFFSET: u32 = 25;
    pub const GPIO_QSPI_SD2_LEVEL_HIGH_MASK: u32 = 0x02000000;
    pub const GPIO_QSPI_SD2_LEVEL_HIGH: u32 = 1 << 25;

    /// Bits [24:24], mask 0x01000000
    pub const GPIO_QSPI_SD2_LEVEL_LOW_OFFSET: u32 = 24;
    pub const GPIO_QSPI_SD2_LEVEL_LOW_MASK: u32 = 0x01000000;
    pub const GPIO_QSPI_SD2_LEVEL_LOW: u32 = 1 << 24;

    /// Bits [23:23], mask 0x00800000
    pub const GPIO_QSPI_SD1_EDGE_HIGH_OFFSET: u32 = 23;
    pub const GPIO_QSPI_SD1_EDGE_HIGH_MASK: u32 = 0x00800000;
    pub const GPIO_QSPI_SD1_EDGE_HIGH: u32 = 1 << 23;

    /// Bits [22:22], mask 0x00400000
    pub const GPIO_QSPI_SD1_EDGE_LOW_OFFSET: u32 = 22;
    pub const GPIO_QSPI_SD1_EDGE_LOW_MASK: u32 = 0x00400000;
    pub const GPIO_QSPI_SD1_EDGE_LOW: u32 = 1 << 22;

    /// Bits [21:21], mask 0x00200000
    pub const GPIO_QSPI_SD1_LEVEL_HIGH_OFFSET: u32 = 21;
    pub const GPIO_QSPI_SD1_LEVEL_HIGH_MASK: u32 = 0x00200000;
    pub const GPIO_QSPI_SD1_LEVEL_HIGH: u32 = 1 << 21;

    /// Bits [20:20], mask 0x00100000
    pub const GPIO_QSPI_SD1_LEVEL_LOW_OFFSET: u32 = 20;
    pub const GPIO_QSPI_SD1_LEVEL_LOW_MASK: u32 = 0x00100000;
    pub const GPIO_QSPI_SD1_LEVEL_LOW: u32 = 1 << 20;

    /// Bits [19:19], mask 0x00080000
    pub const GPIO_QSPI_SD0_EDGE_HIGH_OFFSET: u32 = 19;
    pub const GPIO_QSPI_SD0_EDGE_HIGH_MASK: u32 = 0x00080000;
    pub const GPIO_QSPI_SD0_EDGE_HIGH: u32 = 1 << 19;

    /// Bits [18:18], mask 0x00040000
    pub const GPIO_QSPI_SD0_EDGE_LOW_OFFSET: u32 = 18;
    pub const GPIO_QSPI_SD0_EDGE_LOW_MASK: u32 = 0x00040000;
    pub const GPIO_QSPI_SD0_EDGE_LOW: u32 = 1 << 18;

    /// Bits [17:17], mask 0x00020000
    pub const GPIO_QSPI_SD0_LEVEL_HIGH_OFFSET: u32 = 17;
    pub const GPIO_QSPI_SD0_LEVEL_HIGH_MASK: u32 = 0x00020000;
    pub const GPIO_QSPI_SD0_LEVEL_HIGH: u32 = 1 << 17;

    /// Bits [16:16], mask 0x00010000
    pub const GPIO_QSPI_SD0_LEVEL_LOW_OFFSET: u32 = 16;
    pub const GPIO_QSPI_SD0_LEVEL_LOW_MASK: u32 = 0x00010000;
    pub const GPIO_QSPI_SD0_LEVEL_LOW: u32 = 1 << 16;

    /// Bits [15:15], mask 0x00008000
    pub const GPIO_QSPI_SS_EDGE_HIGH_OFFSET: u32 = 15;
    pub const GPIO_QSPI_SS_EDGE_HIGH_MASK: u32 = 0x00008000;
    pub const GPIO_QSPI_SS_EDGE_HIGH: u32 = 1 << 15;

    /// Bits [14:14], mask 0x00004000
    pub const GPIO_QSPI_SS_EDGE_LOW_OFFSET: u32 = 14;
    pub const GPIO_QSPI_SS_EDGE_LOW_MASK: u32 = 0x00004000;
    pub const GPIO_QSPI_SS_EDGE_LOW: u32 = 1 << 14;

    /// Bits [13:13], mask 0x00002000
    pub const GPIO_QSPI_SS_LEVEL_HIGH_OFFSET: u32 = 13;
    pub const GPIO_QSPI_SS_LEVEL_HIGH_MASK: u32 = 0x00002000;
    pub const GPIO_QSPI_SS_LEVEL_HIGH: u32 = 1 << 13;

    /// Bits [12:12], mask 0x00001000
    pub const GPIO_QSPI_SS_LEVEL_LOW_OFFSET: u32 = 12;
    pub const GPIO_QSPI_SS_LEVEL_LOW_MASK: u32 = 0x00001000;
    pub const GPIO_QSPI_SS_LEVEL_LOW: u32 = 1 << 12;

    /// Bits [11:11], mask 0x00000800
    pub const GPIO_QSPI_SCLK_EDGE_HIGH_OFFSET: u32 = 11;
    pub const GPIO_QSPI_SCLK_EDGE_HIGH_MASK: u32 = 0x00000800;
    pub const GPIO_QSPI_SCLK_EDGE_HIGH: u32 = 1 << 11;

    /// Bits [10:10], mask 0x00000400
    pub const GPIO_QSPI_SCLK_EDGE_LOW_OFFSET: u32 = 10;
    pub const GPIO_QSPI_SCLK_EDGE_LOW_MASK: u32 = 0x00000400;
    pub const GPIO_QSPI_SCLK_EDGE_LOW: u32 = 1 << 10;

    /// Bits [9:9], mask 0x00000200
    pub const GPIO_QSPI_SCLK_LEVEL_HIGH_OFFSET: u32 = 9;
    pub const GPIO_QSPI_SCLK_LEVEL_HIGH_MASK: u32 = 0x00000200;
    pub const GPIO_QSPI_SCLK_LEVEL_HIGH: u32 = 1 << 9;

    /// Bits [8:8], mask 0x00000100
    pub const GPIO_QSPI_SCLK_LEVEL_LOW_OFFSET: u32 = 8;
    pub const GPIO_QSPI_SCLK_LEVEL_LOW_MASK: u32 = 0x00000100;
    pub const GPIO_QSPI_SCLK_LEVEL_LOW: u32 = 1 << 8;

    /// Bits [7:7], mask 0x00000080
    pub const USBPHY_DM_EDGE_HIGH_OFFSET: u32 = 7;
    pub const USBPHY_DM_EDGE_HIGH_MASK: u32 = 0x00000080;
    pub const USBPHY_DM_EDGE_HIGH: u32 = 1 << 7;

    /// Bits [6:6], mask 0x00000040
    pub const USBPHY_DM_EDGE_LOW_OFFSET: u32 = 6;
    pub const USBPHY_DM_EDGE_LOW_MASK: u32 = 0x00000040;
    pub const USBPHY_DM_EDGE_LOW: u32 = 1 << 6;

    /// Bits [5:5], mask 0x00000020
    pub const USBPHY_DM_LEVEL_HIGH_OFFSET: u32 = 5;
    pub const USBPHY_DM_LEVEL_HIGH_MASK: u32 = 0x00000020;
    pub const USBPHY_DM_LEVEL_HIGH: u32 = 1 << 5;

    /// Bits [4:4], mask 0x00000010
    pub const USBPHY_DM_LEVEL_LOW_OFFSET: u32 = 4;
    pub const USBPHY_DM_LEVEL_LOW_MASK: u32 = 0x00000010;
    pub const USBPHY_DM_LEVEL_LOW: u32 = 1 << 4;

    /// Bits [3:3], mask 0x00000008
    pub const USBPHY_DP_EDGE_HIGH_OFFSET: u32 = 3;
    pub const USBPHY_DP_EDGE_HIGH_MASK: u32 = 0x00000008;
    pub const USBPHY_DP_EDGE_HIGH: u32 = 1 << 3;

    /// Bits [2:2], mask 0x00000004
    pub const USBPHY_DP_EDGE_LOW_OFFSET: u32 = 2;
    pub const USBPHY_DP_EDGE_LOW_MASK: u32 = 0x00000004;
    pub const USBPHY_DP_EDGE_LOW: u32 = 1 << 2;

    /// Bits [1:1], mask 0x00000002
    pub const USBPHY_DP_LEVEL_HIGH_OFFSET: u32 = 1;
    pub const USBPHY_DP_LEVEL_HIGH_MASK: u32 = 0x00000002;
    pub const USBPHY_DP_LEVEL_HIGH: u32 = 1 << 1;

    /// Bits [0:0], mask 0x00000001
    pub const USBPHY_DP_LEVEL_LOW_OFFSET: u32 = 0;
    pub const USBPHY_DP_LEVEL_LOW_MASK: u32 = 0x00000001;
    pub const USBPHY_DP_LEVEL_LOW: u32 = 1 << 0;

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

    /// **GPIO_QSPI_SD3_EDGE_HIGH** [31:31]
    #[inline(always)]
    pub fn gpio_qspi_sd3_edge_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD3_EDGE_HIGH_MASK) != 0
    }

    /// Set GPIO_QSPI_SD3_EDGE_HIGH bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd3_edge_high(&self) {
        self.set_bits(Self::GPIO_QSPI_SD3_EDGE_HIGH_MASK);
    }

    /// Clear GPIO_QSPI_SD3_EDGE_HIGH bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd3_edge_high(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD3_EDGE_HIGH_MASK);
    }

    /// **GPIO_QSPI_SD3_EDGE_LOW** [30:30]
    #[inline(always)]
    pub fn gpio_qspi_sd3_edge_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD3_EDGE_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SD3_EDGE_LOW bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd3_edge_low(&self) {
        self.set_bits(Self::GPIO_QSPI_SD3_EDGE_LOW_MASK);
    }

    /// Clear GPIO_QSPI_SD3_EDGE_LOW bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd3_edge_low(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD3_EDGE_LOW_MASK);
    }

    /// **GPIO_QSPI_SD3_LEVEL_HIGH** [29:29]
    #[inline(always)]
    pub fn gpio_qspi_sd3_level_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD3_LEVEL_HIGH_MASK) != 0
    }

    /// **GPIO_QSPI_SD3_LEVEL_LOW** [28:28]
    #[inline(always)]
    pub fn gpio_qspi_sd3_level_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD3_LEVEL_LOW_MASK) != 0
    }

    /// **GPIO_QSPI_SD2_EDGE_HIGH** [27:27]
    #[inline(always)]
    pub fn gpio_qspi_sd2_edge_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD2_EDGE_HIGH_MASK) != 0
    }

    /// Set GPIO_QSPI_SD2_EDGE_HIGH bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd2_edge_high(&self) {
        self.set_bits(Self::GPIO_QSPI_SD2_EDGE_HIGH_MASK);
    }

    /// Clear GPIO_QSPI_SD2_EDGE_HIGH bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd2_edge_high(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD2_EDGE_HIGH_MASK);
    }

    /// **GPIO_QSPI_SD2_EDGE_LOW** [26:26]
    #[inline(always)]
    pub fn gpio_qspi_sd2_edge_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD2_EDGE_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SD2_EDGE_LOW bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd2_edge_low(&self) {
        self.set_bits(Self::GPIO_QSPI_SD2_EDGE_LOW_MASK);
    }

    /// Clear GPIO_QSPI_SD2_EDGE_LOW bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd2_edge_low(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD2_EDGE_LOW_MASK);
    }

    /// **GPIO_QSPI_SD2_LEVEL_HIGH** [25:25]
    #[inline(always)]
    pub fn gpio_qspi_sd2_level_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD2_LEVEL_HIGH_MASK) != 0
    }

    /// **GPIO_QSPI_SD2_LEVEL_LOW** [24:24]
    #[inline(always)]
    pub fn gpio_qspi_sd2_level_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD2_LEVEL_LOW_MASK) != 0
    }

    /// **GPIO_QSPI_SD1_EDGE_HIGH** [23:23]
    #[inline(always)]
    pub fn gpio_qspi_sd1_edge_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD1_EDGE_HIGH_MASK) != 0
    }

    /// Set GPIO_QSPI_SD1_EDGE_HIGH bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd1_edge_high(&self) {
        self.set_bits(Self::GPIO_QSPI_SD1_EDGE_HIGH_MASK);
    }

    /// Clear GPIO_QSPI_SD1_EDGE_HIGH bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd1_edge_high(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD1_EDGE_HIGH_MASK);
    }

    /// **GPIO_QSPI_SD1_EDGE_LOW** [22:22]
    #[inline(always)]
    pub fn gpio_qspi_sd1_edge_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD1_EDGE_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SD1_EDGE_LOW bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd1_edge_low(&self) {
        self.set_bits(Self::GPIO_QSPI_SD1_EDGE_LOW_MASK);
    }

    /// Clear GPIO_QSPI_SD1_EDGE_LOW bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd1_edge_low(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD1_EDGE_LOW_MASK);
    }

    /// **GPIO_QSPI_SD1_LEVEL_HIGH** [21:21]
    #[inline(always)]
    pub fn gpio_qspi_sd1_level_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD1_LEVEL_HIGH_MASK) != 0
    }

    /// **GPIO_QSPI_SD1_LEVEL_LOW** [20:20]
    #[inline(always)]
    pub fn gpio_qspi_sd1_level_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD1_LEVEL_LOW_MASK) != 0
    }

    /// **GPIO_QSPI_SD0_EDGE_HIGH** [19:19]
    #[inline(always)]
    pub fn gpio_qspi_sd0_edge_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD0_EDGE_HIGH_MASK) != 0
    }

    /// Set GPIO_QSPI_SD0_EDGE_HIGH bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd0_edge_high(&self) {
        self.set_bits(Self::GPIO_QSPI_SD0_EDGE_HIGH_MASK);
    }

    /// Clear GPIO_QSPI_SD0_EDGE_HIGH bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd0_edge_high(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD0_EDGE_HIGH_MASK);
    }

    /// **GPIO_QSPI_SD0_EDGE_LOW** [18:18]
    #[inline(always)]
    pub fn gpio_qspi_sd0_edge_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD0_EDGE_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SD0_EDGE_LOW bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd0_edge_low(&self) {
        self.set_bits(Self::GPIO_QSPI_SD0_EDGE_LOW_MASK);
    }

    /// Clear GPIO_QSPI_SD0_EDGE_LOW bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd0_edge_low(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD0_EDGE_LOW_MASK);
    }

    /// **GPIO_QSPI_SD0_LEVEL_HIGH** [17:17]
    #[inline(always)]
    pub fn gpio_qspi_sd0_level_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD0_LEVEL_HIGH_MASK) != 0
    }

    /// **GPIO_QSPI_SD0_LEVEL_LOW** [16:16]
    #[inline(always)]
    pub fn gpio_qspi_sd0_level_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD0_LEVEL_LOW_MASK) != 0
    }

    /// **GPIO_QSPI_SS_EDGE_HIGH** [15:15]
    #[inline(always)]
    pub fn gpio_qspi_ss_edge_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SS_EDGE_HIGH_MASK) != 0
    }

    /// Set GPIO_QSPI_SS_EDGE_HIGH bit
    #[inline(always)]
    pub fn set_gpio_qspi_ss_edge_high(&self) {
        self.set_bits(Self::GPIO_QSPI_SS_EDGE_HIGH_MASK);
    }

    /// Clear GPIO_QSPI_SS_EDGE_HIGH bit
    #[inline(always)]
    pub fn clr_gpio_qspi_ss_edge_high(&self) {
        self.clr_bits(Self::GPIO_QSPI_SS_EDGE_HIGH_MASK);
    }

    /// **GPIO_QSPI_SS_EDGE_LOW** [14:14]
    #[inline(always)]
    pub fn gpio_qspi_ss_edge_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SS_EDGE_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SS_EDGE_LOW bit
    #[inline(always)]
    pub fn set_gpio_qspi_ss_edge_low(&self) {
        self.set_bits(Self::GPIO_QSPI_SS_EDGE_LOW_MASK);
    }

    /// Clear GPIO_QSPI_SS_EDGE_LOW bit
    #[inline(always)]
    pub fn clr_gpio_qspi_ss_edge_low(&self) {
        self.clr_bits(Self::GPIO_QSPI_SS_EDGE_LOW_MASK);
    }

    /// **GPIO_QSPI_SS_LEVEL_HIGH** [13:13]
    #[inline(always)]
    pub fn gpio_qspi_ss_level_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SS_LEVEL_HIGH_MASK) != 0
    }

    /// **GPIO_QSPI_SS_LEVEL_LOW** [12:12]
    #[inline(always)]
    pub fn gpio_qspi_ss_level_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SS_LEVEL_LOW_MASK) != 0
    }

    /// **GPIO_QSPI_SCLK_EDGE_HIGH** [11:11]
    #[inline(always)]
    pub fn gpio_qspi_sclk_edge_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SCLK_EDGE_HIGH_MASK) != 0
    }

    /// Set GPIO_QSPI_SCLK_EDGE_HIGH bit
    #[inline(always)]
    pub fn set_gpio_qspi_sclk_edge_high(&self) {
        self.set_bits(Self::GPIO_QSPI_SCLK_EDGE_HIGH_MASK);
    }

    /// Clear GPIO_QSPI_SCLK_EDGE_HIGH bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sclk_edge_high(&self) {
        self.clr_bits(Self::GPIO_QSPI_SCLK_EDGE_HIGH_MASK);
    }

    /// **GPIO_QSPI_SCLK_EDGE_LOW** [10:10]
    #[inline(always)]
    pub fn gpio_qspi_sclk_edge_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SCLK_EDGE_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SCLK_EDGE_LOW bit
    #[inline(always)]
    pub fn set_gpio_qspi_sclk_edge_low(&self) {
        self.set_bits(Self::GPIO_QSPI_SCLK_EDGE_LOW_MASK);
    }

    /// Clear GPIO_QSPI_SCLK_EDGE_LOW bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sclk_edge_low(&self) {
        self.clr_bits(Self::GPIO_QSPI_SCLK_EDGE_LOW_MASK);
    }

    /// **GPIO_QSPI_SCLK_LEVEL_HIGH** [9:9]
    #[inline(always)]
    pub fn gpio_qspi_sclk_level_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SCLK_LEVEL_HIGH_MASK) != 0
    }

    /// **GPIO_QSPI_SCLK_LEVEL_LOW** [8:8]
    #[inline(always)]
    pub fn gpio_qspi_sclk_level_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SCLK_LEVEL_LOW_MASK) != 0
    }

    /// **USBPHY_DM_EDGE_HIGH** [7:7]
    #[inline(always)]
    pub fn usbphy_dm_edge_high(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DM_EDGE_HIGH_MASK) != 0
    }

    /// Set USBPHY_DM_EDGE_HIGH bit
    #[inline(always)]
    pub fn set_usbphy_dm_edge_high(&self) {
        self.set_bits(Self::USBPHY_DM_EDGE_HIGH_MASK);
    }

    /// Clear USBPHY_DM_EDGE_HIGH bit
    #[inline(always)]
    pub fn clr_usbphy_dm_edge_high(&self) {
        self.clr_bits(Self::USBPHY_DM_EDGE_HIGH_MASK);
    }

    /// **USBPHY_DM_EDGE_LOW** [6:6]
    #[inline(always)]
    pub fn usbphy_dm_edge_low(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DM_EDGE_LOW_MASK) != 0
    }

    /// Set USBPHY_DM_EDGE_LOW bit
    #[inline(always)]
    pub fn set_usbphy_dm_edge_low(&self) {
        self.set_bits(Self::USBPHY_DM_EDGE_LOW_MASK);
    }

    /// Clear USBPHY_DM_EDGE_LOW bit
    #[inline(always)]
    pub fn clr_usbphy_dm_edge_low(&self) {
        self.clr_bits(Self::USBPHY_DM_EDGE_LOW_MASK);
    }

    /// **USBPHY_DM_LEVEL_HIGH** [5:5]
    #[inline(always)]
    pub fn usbphy_dm_level_high(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DM_LEVEL_HIGH_MASK) != 0
    }

    /// **USBPHY_DM_LEVEL_LOW** [4:4]
    #[inline(always)]
    pub fn usbphy_dm_level_low(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DM_LEVEL_LOW_MASK) != 0
    }

    /// **USBPHY_DP_EDGE_HIGH** [3:3]
    #[inline(always)]
    pub fn usbphy_dp_edge_high(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DP_EDGE_HIGH_MASK) != 0
    }

    /// Set USBPHY_DP_EDGE_HIGH bit
    #[inline(always)]
    pub fn set_usbphy_dp_edge_high(&self) {
        self.set_bits(Self::USBPHY_DP_EDGE_HIGH_MASK);
    }

    /// Clear USBPHY_DP_EDGE_HIGH bit
    #[inline(always)]
    pub fn clr_usbphy_dp_edge_high(&self) {
        self.clr_bits(Self::USBPHY_DP_EDGE_HIGH_MASK);
    }

    /// **USBPHY_DP_EDGE_LOW** [2:2]
    #[inline(always)]
    pub fn usbphy_dp_edge_low(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DP_EDGE_LOW_MASK) != 0
    }

    /// Set USBPHY_DP_EDGE_LOW bit
    #[inline(always)]
    pub fn set_usbphy_dp_edge_low(&self) {
        self.set_bits(Self::USBPHY_DP_EDGE_LOW_MASK);
    }

    /// Clear USBPHY_DP_EDGE_LOW bit
    #[inline(always)]
    pub fn clr_usbphy_dp_edge_low(&self) {
        self.clr_bits(Self::USBPHY_DP_EDGE_LOW_MASK);
    }

    /// **USBPHY_DP_LEVEL_HIGH** [1:1]
    #[inline(always)]
    pub fn usbphy_dp_level_high(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DP_LEVEL_HIGH_MASK) != 0
    }

    /// **USBPHY_DP_LEVEL_LOW** [0:0]
    #[inline(always)]
    pub fn usbphy_dp_level_low(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DP_LEVEL_LOW_MASK) != 0
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

    /// Read GPIO_QSPI_SD3_EDGE_HIGH [31:31]
    #[inline(always)]
    pub const fn gpio_qspi_sd3_edge_high(&self) -> bool {
        (self.bits & Intr::GPIO_QSPI_SD3_EDGE_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD3_EDGE_LOW [30:30]
    #[inline(always)]
    pub const fn gpio_qspi_sd3_edge_low(&self) -> bool {
        (self.bits & Intr::GPIO_QSPI_SD3_EDGE_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SD3_LEVEL_HIGH [29:29]
    #[inline(always)]
    pub const fn gpio_qspi_sd3_level_high(&self) -> bool {
        (self.bits & Intr::GPIO_QSPI_SD3_LEVEL_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD3_LEVEL_LOW [28:28]
    #[inline(always)]
    pub const fn gpio_qspi_sd3_level_low(&self) -> bool {
        (self.bits & Intr::GPIO_QSPI_SD3_LEVEL_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SD2_EDGE_HIGH [27:27]
    #[inline(always)]
    pub const fn gpio_qspi_sd2_edge_high(&self) -> bool {
        (self.bits & Intr::GPIO_QSPI_SD2_EDGE_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD2_EDGE_LOW [26:26]
    #[inline(always)]
    pub const fn gpio_qspi_sd2_edge_low(&self) -> bool {
        (self.bits & Intr::GPIO_QSPI_SD2_EDGE_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SD2_LEVEL_HIGH [25:25]
    #[inline(always)]
    pub const fn gpio_qspi_sd2_level_high(&self) -> bool {
        (self.bits & Intr::GPIO_QSPI_SD2_LEVEL_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD2_LEVEL_LOW [24:24]
    #[inline(always)]
    pub const fn gpio_qspi_sd2_level_low(&self) -> bool {
        (self.bits & Intr::GPIO_QSPI_SD2_LEVEL_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SD1_EDGE_HIGH [23:23]
    #[inline(always)]
    pub const fn gpio_qspi_sd1_edge_high(&self) -> bool {
        (self.bits & Intr::GPIO_QSPI_SD1_EDGE_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD1_EDGE_LOW [22:22]
    #[inline(always)]
    pub const fn gpio_qspi_sd1_edge_low(&self) -> bool {
        (self.bits & Intr::GPIO_QSPI_SD1_EDGE_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SD1_LEVEL_HIGH [21:21]
    #[inline(always)]
    pub const fn gpio_qspi_sd1_level_high(&self) -> bool {
        (self.bits & Intr::GPIO_QSPI_SD1_LEVEL_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD1_LEVEL_LOW [20:20]
    #[inline(always)]
    pub const fn gpio_qspi_sd1_level_low(&self) -> bool {
        (self.bits & Intr::GPIO_QSPI_SD1_LEVEL_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SD0_EDGE_HIGH [19:19]
    #[inline(always)]
    pub const fn gpio_qspi_sd0_edge_high(&self) -> bool {
        (self.bits & Intr::GPIO_QSPI_SD0_EDGE_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD0_EDGE_LOW [18:18]
    #[inline(always)]
    pub const fn gpio_qspi_sd0_edge_low(&self) -> bool {
        (self.bits & Intr::GPIO_QSPI_SD0_EDGE_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SD0_LEVEL_HIGH [17:17]
    #[inline(always)]
    pub const fn gpio_qspi_sd0_level_high(&self) -> bool {
        (self.bits & Intr::GPIO_QSPI_SD0_LEVEL_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD0_LEVEL_LOW [16:16]
    #[inline(always)]
    pub const fn gpio_qspi_sd0_level_low(&self) -> bool {
        (self.bits & Intr::GPIO_QSPI_SD0_LEVEL_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SS_EDGE_HIGH [15:15]
    #[inline(always)]
    pub const fn gpio_qspi_ss_edge_high(&self) -> bool {
        (self.bits & Intr::GPIO_QSPI_SS_EDGE_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SS_EDGE_LOW [14:14]
    #[inline(always)]
    pub const fn gpio_qspi_ss_edge_low(&self) -> bool {
        (self.bits & Intr::GPIO_QSPI_SS_EDGE_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SS_LEVEL_HIGH [13:13]
    #[inline(always)]
    pub const fn gpio_qspi_ss_level_high(&self) -> bool {
        (self.bits & Intr::GPIO_QSPI_SS_LEVEL_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SS_LEVEL_LOW [12:12]
    #[inline(always)]
    pub const fn gpio_qspi_ss_level_low(&self) -> bool {
        (self.bits & Intr::GPIO_QSPI_SS_LEVEL_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SCLK_EDGE_HIGH [11:11]
    #[inline(always)]
    pub const fn gpio_qspi_sclk_edge_high(&self) -> bool {
        (self.bits & Intr::GPIO_QSPI_SCLK_EDGE_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SCLK_EDGE_LOW [10:10]
    #[inline(always)]
    pub const fn gpio_qspi_sclk_edge_low(&self) -> bool {
        (self.bits & Intr::GPIO_QSPI_SCLK_EDGE_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SCLK_LEVEL_HIGH [9:9]
    #[inline(always)]
    pub const fn gpio_qspi_sclk_level_high(&self) -> bool {
        (self.bits & Intr::GPIO_QSPI_SCLK_LEVEL_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SCLK_LEVEL_LOW [8:8]
    #[inline(always)]
    pub const fn gpio_qspi_sclk_level_low(&self) -> bool {
        (self.bits & Intr::GPIO_QSPI_SCLK_LEVEL_LOW_MASK) != 0
    }

    /// Read USBPHY_DM_EDGE_HIGH [7:7]
    #[inline(always)]
    pub const fn usbphy_dm_edge_high(&self) -> bool {
        (self.bits & Intr::USBPHY_DM_EDGE_HIGH_MASK) != 0
    }

    /// Read USBPHY_DM_EDGE_LOW [6:6]
    #[inline(always)]
    pub const fn usbphy_dm_edge_low(&self) -> bool {
        (self.bits & Intr::USBPHY_DM_EDGE_LOW_MASK) != 0
    }

    /// Read USBPHY_DM_LEVEL_HIGH [5:5]
    #[inline(always)]
    pub const fn usbphy_dm_level_high(&self) -> bool {
        (self.bits & Intr::USBPHY_DM_LEVEL_HIGH_MASK) != 0
    }

    /// Read USBPHY_DM_LEVEL_LOW [4:4]
    #[inline(always)]
    pub const fn usbphy_dm_level_low(&self) -> bool {
        (self.bits & Intr::USBPHY_DM_LEVEL_LOW_MASK) != 0
    }

    /// Read USBPHY_DP_EDGE_HIGH [3:3]
    #[inline(always)]
    pub const fn usbphy_dp_edge_high(&self) -> bool {
        (self.bits & Intr::USBPHY_DP_EDGE_HIGH_MASK) != 0
    }

    /// Read USBPHY_DP_EDGE_LOW [2:2]
    #[inline(always)]
    pub const fn usbphy_dp_edge_low(&self) -> bool {
        (self.bits & Intr::USBPHY_DP_EDGE_LOW_MASK) != 0
    }

    /// Read USBPHY_DP_LEVEL_HIGH [1:1]
    #[inline(always)]
    pub const fn usbphy_dp_level_high(&self) -> bool {
        (self.bits & Intr::USBPHY_DP_LEVEL_HIGH_MASK) != 0
    }

    /// Read USBPHY_DP_LEVEL_LOW [0:0]
    #[inline(always)]
    pub const fn usbphy_dp_level_low(&self) -> bool {
        (self.bits & Intr::USBPHY_DP_LEVEL_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SD3_EDGE_HIGH [31:31]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd3_edge_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intr::GPIO_QSPI_SD3_EDGE_HIGH_MASK
            } else {
                self.bits & !Intr::GPIO_QSPI_SD3_EDGE_HIGH_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD3_EDGE_LOW [30:30]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd3_edge_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intr::GPIO_QSPI_SD3_EDGE_LOW_MASK
            } else {
                self.bits & !Intr::GPIO_QSPI_SD3_EDGE_LOW_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD2_EDGE_HIGH [27:27]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd2_edge_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intr::GPIO_QSPI_SD2_EDGE_HIGH_MASK
            } else {
                self.bits & !Intr::GPIO_QSPI_SD2_EDGE_HIGH_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD2_EDGE_LOW [26:26]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd2_edge_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intr::GPIO_QSPI_SD2_EDGE_LOW_MASK
            } else {
                self.bits & !Intr::GPIO_QSPI_SD2_EDGE_LOW_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD1_EDGE_HIGH [23:23]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd1_edge_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intr::GPIO_QSPI_SD1_EDGE_HIGH_MASK
            } else {
                self.bits & !Intr::GPIO_QSPI_SD1_EDGE_HIGH_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD1_EDGE_LOW [22:22]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd1_edge_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intr::GPIO_QSPI_SD1_EDGE_LOW_MASK
            } else {
                self.bits & !Intr::GPIO_QSPI_SD1_EDGE_LOW_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD0_EDGE_HIGH [19:19]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd0_edge_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intr::GPIO_QSPI_SD0_EDGE_HIGH_MASK
            } else {
                self.bits & !Intr::GPIO_QSPI_SD0_EDGE_HIGH_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD0_EDGE_LOW [18:18]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd0_edge_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intr::GPIO_QSPI_SD0_EDGE_LOW_MASK
            } else {
                self.bits & !Intr::GPIO_QSPI_SD0_EDGE_LOW_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SS_EDGE_HIGH [15:15]
    #[inline(always)]
    pub const fn with_gpio_qspi_ss_edge_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intr::GPIO_QSPI_SS_EDGE_HIGH_MASK
            } else {
                self.bits & !Intr::GPIO_QSPI_SS_EDGE_HIGH_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SS_EDGE_LOW [14:14]
    #[inline(always)]
    pub const fn with_gpio_qspi_ss_edge_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intr::GPIO_QSPI_SS_EDGE_LOW_MASK
            } else {
                self.bits & !Intr::GPIO_QSPI_SS_EDGE_LOW_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SCLK_EDGE_HIGH [11:11]
    #[inline(always)]
    pub const fn with_gpio_qspi_sclk_edge_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intr::GPIO_QSPI_SCLK_EDGE_HIGH_MASK
            } else {
                self.bits & !Intr::GPIO_QSPI_SCLK_EDGE_HIGH_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SCLK_EDGE_LOW [10:10]
    #[inline(always)]
    pub const fn with_gpio_qspi_sclk_edge_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intr::GPIO_QSPI_SCLK_EDGE_LOW_MASK
            } else {
                self.bits & !Intr::GPIO_QSPI_SCLK_EDGE_LOW_MASK
            }
        }
    }

    /// Set USBPHY_DM_EDGE_HIGH [7:7]
    #[inline(always)]
    pub const fn with_usbphy_dm_edge_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intr::USBPHY_DM_EDGE_HIGH_MASK
            } else {
                self.bits & !Intr::USBPHY_DM_EDGE_HIGH_MASK
            }
        }
    }

    /// Set USBPHY_DM_EDGE_LOW [6:6]
    #[inline(always)]
    pub const fn with_usbphy_dm_edge_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intr::USBPHY_DM_EDGE_LOW_MASK
            } else {
                self.bits & !Intr::USBPHY_DM_EDGE_LOW_MASK
            }
        }
    }

    /// Set USBPHY_DP_EDGE_HIGH [3:3]
    #[inline(always)]
    pub const fn with_usbphy_dp_edge_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intr::USBPHY_DP_EDGE_HIGH_MASK
            } else {
                self.bits & !Intr::USBPHY_DP_EDGE_HIGH_MASK
            }
        }
    }

    /// Set USBPHY_DP_EDGE_LOW [2:2]
    #[inline(always)]
    pub const fn with_usbphy_dp_edge_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intr::USBPHY_DP_EDGE_LOW_MASK
            } else {
                self.bits & !Intr::USBPHY_DP_EDGE_LOW_MASK
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
    pub fn read(&self) -> IntrVal {
        IntrVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: IntrVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(IntrVal) -> IntrVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// PROC0_INTE Register
///
/// Interrupt Enable for proc0
///
/// Address offset: 0x021c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **GPIO_QSPI_SD3_EDGE_HIGH** [31] (RW): 
/// - **GPIO_QSPI_SD3_EDGE_LOW** [30] (RW): 
/// - **GPIO_QSPI_SD3_LEVEL_HIGH** [29] (RW): 
/// - **GPIO_QSPI_SD3_LEVEL_LOW** [28] (RW): 
/// - **GPIO_QSPI_SD2_EDGE_HIGH** [27] (RW): 
/// - **GPIO_QSPI_SD2_EDGE_LOW** [26] (RW): 
/// - **GPIO_QSPI_SD2_LEVEL_HIGH** [25] (RW): 
/// - **GPIO_QSPI_SD2_LEVEL_LOW** [24] (RW): 
/// - **GPIO_QSPI_SD1_EDGE_HIGH** [23] (RW): 
/// - **GPIO_QSPI_SD1_EDGE_LOW** [22] (RW): 
/// - **GPIO_QSPI_SD1_LEVEL_HIGH** [21] (RW): 
/// - **GPIO_QSPI_SD1_LEVEL_LOW** [20] (RW): 
/// - **GPIO_QSPI_SD0_EDGE_HIGH** [19] (RW): 
/// - **GPIO_QSPI_SD0_EDGE_LOW** [18] (RW): 
/// - **GPIO_QSPI_SD0_LEVEL_HIGH** [17] (RW): 
/// - **GPIO_QSPI_SD0_LEVEL_LOW** [16] (RW): 
/// - **GPIO_QSPI_SS_EDGE_HIGH** [15] (RW): 
/// - **GPIO_QSPI_SS_EDGE_LOW** [14] (RW): 
/// - **GPIO_QSPI_SS_LEVEL_HIGH** [13] (RW): 
/// - **GPIO_QSPI_SS_LEVEL_LOW** [12] (RW): 
/// - **GPIO_QSPI_SCLK_EDGE_HIGH** [11] (RW): 
/// - **GPIO_QSPI_SCLK_EDGE_LOW** [10] (RW): 
/// - **GPIO_QSPI_SCLK_LEVEL_HIGH** [9] (RW): 
/// - **GPIO_QSPI_SCLK_LEVEL_LOW** [8] (RW): 
/// - **USBPHY_DM_EDGE_HIGH** [7] (RW): 
/// - **USBPHY_DM_EDGE_LOW** [6] (RW): 
/// - **USBPHY_DM_LEVEL_HIGH** [5] (RW): 
/// - **USBPHY_DM_LEVEL_LOW** [4] (RW): 
/// - **USBPHY_DP_EDGE_HIGH** [3] (RW): 
/// - **USBPHY_DP_EDGE_LOW** [2] (RW): 
/// - **USBPHY_DP_LEVEL_HIGH** [1] (RW): 
/// - **USBPHY_DP_LEVEL_LOW** [0] (RW): 
#[repr(transparent)]
pub struct Proc0Inte {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for Proc0Inte {}
unsafe impl Sync for Proc0Inte {}

impl Proc0Inte {
    // ======== Field Constants ========

    /// Bits [31:31], mask 0x80000000
    pub const GPIO_QSPI_SD3_EDGE_HIGH_OFFSET: u32 = 31;
    pub const GPIO_QSPI_SD3_EDGE_HIGH_MASK: u32 = 0x80000000;
    pub const GPIO_QSPI_SD3_EDGE_HIGH: u32 = 1 << 31;

    /// Bits [30:30], mask 0x40000000
    pub const GPIO_QSPI_SD3_EDGE_LOW_OFFSET: u32 = 30;
    pub const GPIO_QSPI_SD3_EDGE_LOW_MASK: u32 = 0x40000000;
    pub const GPIO_QSPI_SD3_EDGE_LOW: u32 = 1 << 30;

    /// Bits [29:29], mask 0x20000000
    pub const GPIO_QSPI_SD3_LEVEL_HIGH_OFFSET: u32 = 29;
    pub const GPIO_QSPI_SD3_LEVEL_HIGH_MASK: u32 = 0x20000000;
    pub const GPIO_QSPI_SD3_LEVEL_HIGH: u32 = 1 << 29;

    /// Bits [28:28], mask 0x10000000
    pub const GPIO_QSPI_SD3_LEVEL_LOW_OFFSET: u32 = 28;
    pub const GPIO_QSPI_SD3_LEVEL_LOW_MASK: u32 = 0x10000000;
    pub const GPIO_QSPI_SD3_LEVEL_LOW: u32 = 1 << 28;

    /// Bits [27:27], mask 0x08000000
    pub const GPIO_QSPI_SD2_EDGE_HIGH_OFFSET: u32 = 27;
    pub const GPIO_QSPI_SD2_EDGE_HIGH_MASK: u32 = 0x08000000;
    pub const GPIO_QSPI_SD2_EDGE_HIGH: u32 = 1 << 27;

    /// Bits [26:26], mask 0x04000000
    pub const GPIO_QSPI_SD2_EDGE_LOW_OFFSET: u32 = 26;
    pub const GPIO_QSPI_SD2_EDGE_LOW_MASK: u32 = 0x04000000;
    pub const GPIO_QSPI_SD2_EDGE_LOW: u32 = 1 << 26;

    /// Bits [25:25], mask 0x02000000
    pub const GPIO_QSPI_SD2_LEVEL_HIGH_OFFSET: u32 = 25;
    pub const GPIO_QSPI_SD2_LEVEL_HIGH_MASK: u32 = 0x02000000;
    pub const GPIO_QSPI_SD2_LEVEL_HIGH: u32 = 1 << 25;

    /// Bits [24:24], mask 0x01000000
    pub const GPIO_QSPI_SD2_LEVEL_LOW_OFFSET: u32 = 24;
    pub const GPIO_QSPI_SD2_LEVEL_LOW_MASK: u32 = 0x01000000;
    pub const GPIO_QSPI_SD2_LEVEL_LOW: u32 = 1 << 24;

    /// Bits [23:23], mask 0x00800000
    pub const GPIO_QSPI_SD1_EDGE_HIGH_OFFSET: u32 = 23;
    pub const GPIO_QSPI_SD1_EDGE_HIGH_MASK: u32 = 0x00800000;
    pub const GPIO_QSPI_SD1_EDGE_HIGH: u32 = 1 << 23;

    /// Bits [22:22], mask 0x00400000
    pub const GPIO_QSPI_SD1_EDGE_LOW_OFFSET: u32 = 22;
    pub const GPIO_QSPI_SD1_EDGE_LOW_MASK: u32 = 0x00400000;
    pub const GPIO_QSPI_SD1_EDGE_LOW: u32 = 1 << 22;

    /// Bits [21:21], mask 0x00200000
    pub const GPIO_QSPI_SD1_LEVEL_HIGH_OFFSET: u32 = 21;
    pub const GPIO_QSPI_SD1_LEVEL_HIGH_MASK: u32 = 0x00200000;
    pub const GPIO_QSPI_SD1_LEVEL_HIGH: u32 = 1 << 21;

    /// Bits [20:20], mask 0x00100000
    pub const GPIO_QSPI_SD1_LEVEL_LOW_OFFSET: u32 = 20;
    pub const GPIO_QSPI_SD1_LEVEL_LOW_MASK: u32 = 0x00100000;
    pub const GPIO_QSPI_SD1_LEVEL_LOW: u32 = 1 << 20;

    /// Bits [19:19], mask 0x00080000
    pub const GPIO_QSPI_SD0_EDGE_HIGH_OFFSET: u32 = 19;
    pub const GPIO_QSPI_SD0_EDGE_HIGH_MASK: u32 = 0x00080000;
    pub const GPIO_QSPI_SD0_EDGE_HIGH: u32 = 1 << 19;

    /// Bits [18:18], mask 0x00040000
    pub const GPIO_QSPI_SD0_EDGE_LOW_OFFSET: u32 = 18;
    pub const GPIO_QSPI_SD0_EDGE_LOW_MASK: u32 = 0x00040000;
    pub const GPIO_QSPI_SD0_EDGE_LOW: u32 = 1 << 18;

    /// Bits [17:17], mask 0x00020000
    pub const GPIO_QSPI_SD0_LEVEL_HIGH_OFFSET: u32 = 17;
    pub const GPIO_QSPI_SD0_LEVEL_HIGH_MASK: u32 = 0x00020000;
    pub const GPIO_QSPI_SD0_LEVEL_HIGH: u32 = 1 << 17;

    /// Bits [16:16], mask 0x00010000
    pub const GPIO_QSPI_SD0_LEVEL_LOW_OFFSET: u32 = 16;
    pub const GPIO_QSPI_SD0_LEVEL_LOW_MASK: u32 = 0x00010000;
    pub const GPIO_QSPI_SD0_LEVEL_LOW: u32 = 1 << 16;

    /// Bits [15:15], mask 0x00008000
    pub const GPIO_QSPI_SS_EDGE_HIGH_OFFSET: u32 = 15;
    pub const GPIO_QSPI_SS_EDGE_HIGH_MASK: u32 = 0x00008000;
    pub const GPIO_QSPI_SS_EDGE_HIGH: u32 = 1 << 15;

    /// Bits [14:14], mask 0x00004000
    pub const GPIO_QSPI_SS_EDGE_LOW_OFFSET: u32 = 14;
    pub const GPIO_QSPI_SS_EDGE_LOW_MASK: u32 = 0x00004000;
    pub const GPIO_QSPI_SS_EDGE_LOW: u32 = 1 << 14;

    /// Bits [13:13], mask 0x00002000
    pub const GPIO_QSPI_SS_LEVEL_HIGH_OFFSET: u32 = 13;
    pub const GPIO_QSPI_SS_LEVEL_HIGH_MASK: u32 = 0x00002000;
    pub const GPIO_QSPI_SS_LEVEL_HIGH: u32 = 1 << 13;

    /// Bits [12:12], mask 0x00001000
    pub const GPIO_QSPI_SS_LEVEL_LOW_OFFSET: u32 = 12;
    pub const GPIO_QSPI_SS_LEVEL_LOW_MASK: u32 = 0x00001000;
    pub const GPIO_QSPI_SS_LEVEL_LOW: u32 = 1 << 12;

    /// Bits [11:11], mask 0x00000800
    pub const GPIO_QSPI_SCLK_EDGE_HIGH_OFFSET: u32 = 11;
    pub const GPIO_QSPI_SCLK_EDGE_HIGH_MASK: u32 = 0x00000800;
    pub const GPIO_QSPI_SCLK_EDGE_HIGH: u32 = 1 << 11;

    /// Bits [10:10], mask 0x00000400
    pub const GPIO_QSPI_SCLK_EDGE_LOW_OFFSET: u32 = 10;
    pub const GPIO_QSPI_SCLK_EDGE_LOW_MASK: u32 = 0x00000400;
    pub const GPIO_QSPI_SCLK_EDGE_LOW: u32 = 1 << 10;

    /// Bits [9:9], mask 0x00000200
    pub const GPIO_QSPI_SCLK_LEVEL_HIGH_OFFSET: u32 = 9;
    pub const GPIO_QSPI_SCLK_LEVEL_HIGH_MASK: u32 = 0x00000200;
    pub const GPIO_QSPI_SCLK_LEVEL_HIGH: u32 = 1 << 9;

    /// Bits [8:8], mask 0x00000100
    pub const GPIO_QSPI_SCLK_LEVEL_LOW_OFFSET: u32 = 8;
    pub const GPIO_QSPI_SCLK_LEVEL_LOW_MASK: u32 = 0x00000100;
    pub const GPIO_QSPI_SCLK_LEVEL_LOW: u32 = 1 << 8;

    /// Bits [7:7], mask 0x00000080
    pub const USBPHY_DM_EDGE_HIGH_OFFSET: u32 = 7;
    pub const USBPHY_DM_EDGE_HIGH_MASK: u32 = 0x00000080;
    pub const USBPHY_DM_EDGE_HIGH: u32 = 1 << 7;

    /// Bits [6:6], mask 0x00000040
    pub const USBPHY_DM_EDGE_LOW_OFFSET: u32 = 6;
    pub const USBPHY_DM_EDGE_LOW_MASK: u32 = 0x00000040;
    pub const USBPHY_DM_EDGE_LOW: u32 = 1 << 6;

    /// Bits [5:5], mask 0x00000020
    pub const USBPHY_DM_LEVEL_HIGH_OFFSET: u32 = 5;
    pub const USBPHY_DM_LEVEL_HIGH_MASK: u32 = 0x00000020;
    pub const USBPHY_DM_LEVEL_HIGH: u32 = 1 << 5;

    /// Bits [4:4], mask 0x00000010
    pub const USBPHY_DM_LEVEL_LOW_OFFSET: u32 = 4;
    pub const USBPHY_DM_LEVEL_LOW_MASK: u32 = 0x00000010;
    pub const USBPHY_DM_LEVEL_LOW: u32 = 1 << 4;

    /// Bits [3:3], mask 0x00000008
    pub const USBPHY_DP_EDGE_HIGH_OFFSET: u32 = 3;
    pub const USBPHY_DP_EDGE_HIGH_MASK: u32 = 0x00000008;
    pub const USBPHY_DP_EDGE_HIGH: u32 = 1 << 3;

    /// Bits [2:2], mask 0x00000004
    pub const USBPHY_DP_EDGE_LOW_OFFSET: u32 = 2;
    pub const USBPHY_DP_EDGE_LOW_MASK: u32 = 0x00000004;
    pub const USBPHY_DP_EDGE_LOW: u32 = 1 << 2;

    /// Bits [1:1], mask 0x00000002
    pub const USBPHY_DP_LEVEL_HIGH_OFFSET: u32 = 1;
    pub const USBPHY_DP_LEVEL_HIGH_MASK: u32 = 0x00000002;
    pub const USBPHY_DP_LEVEL_HIGH: u32 = 1 << 1;

    /// Bits [0:0], mask 0x00000001
    pub const USBPHY_DP_LEVEL_LOW_OFFSET: u32 = 0;
    pub const USBPHY_DP_LEVEL_LOW_MASK: u32 = 0x00000001;
    pub const USBPHY_DP_LEVEL_LOW: u32 = 1 << 0;

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

    /// **GPIO_QSPI_SD3_EDGE_HIGH** [31:31]
    #[inline(always)]
    pub fn gpio_qspi_sd3_edge_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD3_EDGE_HIGH_MASK) != 0
    }

    /// Set GPIO_QSPI_SD3_EDGE_HIGH bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd3_edge_high(&self) {
        self.set_bits(Self::GPIO_QSPI_SD3_EDGE_HIGH_MASK);
    }

    /// Clear GPIO_QSPI_SD3_EDGE_HIGH bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd3_edge_high(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD3_EDGE_HIGH_MASK);
    }

    /// **GPIO_QSPI_SD3_EDGE_LOW** [30:30]
    #[inline(always)]
    pub fn gpio_qspi_sd3_edge_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD3_EDGE_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SD3_EDGE_LOW bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd3_edge_low(&self) {
        self.set_bits(Self::GPIO_QSPI_SD3_EDGE_LOW_MASK);
    }

    /// Clear GPIO_QSPI_SD3_EDGE_LOW bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd3_edge_low(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD3_EDGE_LOW_MASK);
    }

    /// **GPIO_QSPI_SD3_LEVEL_HIGH** [29:29]
    #[inline(always)]
    pub fn gpio_qspi_sd3_level_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD3_LEVEL_HIGH_MASK) != 0
    }

    /// Set GPIO_QSPI_SD3_LEVEL_HIGH bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd3_level_high(&self) {
        self.set_bits(Self::GPIO_QSPI_SD3_LEVEL_HIGH_MASK);
    }

    /// Clear GPIO_QSPI_SD3_LEVEL_HIGH bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd3_level_high(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD3_LEVEL_HIGH_MASK);
    }

    /// **GPIO_QSPI_SD3_LEVEL_LOW** [28:28]
    #[inline(always)]
    pub fn gpio_qspi_sd3_level_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD3_LEVEL_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SD3_LEVEL_LOW bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd3_level_low(&self) {
        self.set_bits(Self::GPIO_QSPI_SD3_LEVEL_LOW_MASK);
    }

    /// Clear GPIO_QSPI_SD3_LEVEL_LOW bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd3_level_low(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD3_LEVEL_LOW_MASK);
    }

    /// **GPIO_QSPI_SD2_EDGE_HIGH** [27:27]
    #[inline(always)]
    pub fn gpio_qspi_sd2_edge_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD2_EDGE_HIGH_MASK) != 0
    }

    /// Set GPIO_QSPI_SD2_EDGE_HIGH bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd2_edge_high(&self) {
        self.set_bits(Self::GPIO_QSPI_SD2_EDGE_HIGH_MASK);
    }

    /// Clear GPIO_QSPI_SD2_EDGE_HIGH bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd2_edge_high(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD2_EDGE_HIGH_MASK);
    }

    /// **GPIO_QSPI_SD2_EDGE_LOW** [26:26]
    #[inline(always)]
    pub fn gpio_qspi_sd2_edge_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD2_EDGE_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SD2_EDGE_LOW bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd2_edge_low(&self) {
        self.set_bits(Self::GPIO_QSPI_SD2_EDGE_LOW_MASK);
    }

    /// Clear GPIO_QSPI_SD2_EDGE_LOW bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd2_edge_low(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD2_EDGE_LOW_MASK);
    }

    /// **GPIO_QSPI_SD2_LEVEL_HIGH** [25:25]
    #[inline(always)]
    pub fn gpio_qspi_sd2_level_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD2_LEVEL_HIGH_MASK) != 0
    }

    /// Set GPIO_QSPI_SD2_LEVEL_HIGH bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd2_level_high(&self) {
        self.set_bits(Self::GPIO_QSPI_SD2_LEVEL_HIGH_MASK);
    }

    /// Clear GPIO_QSPI_SD2_LEVEL_HIGH bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd2_level_high(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD2_LEVEL_HIGH_MASK);
    }

    /// **GPIO_QSPI_SD2_LEVEL_LOW** [24:24]
    #[inline(always)]
    pub fn gpio_qspi_sd2_level_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD2_LEVEL_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SD2_LEVEL_LOW bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd2_level_low(&self) {
        self.set_bits(Self::GPIO_QSPI_SD2_LEVEL_LOW_MASK);
    }

    /// Clear GPIO_QSPI_SD2_LEVEL_LOW bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd2_level_low(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD2_LEVEL_LOW_MASK);
    }

    /// **GPIO_QSPI_SD1_EDGE_HIGH** [23:23]
    #[inline(always)]
    pub fn gpio_qspi_sd1_edge_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD1_EDGE_HIGH_MASK) != 0
    }

    /// Set GPIO_QSPI_SD1_EDGE_HIGH bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd1_edge_high(&self) {
        self.set_bits(Self::GPIO_QSPI_SD1_EDGE_HIGH_MASK);
    }

    /// Clear GPIO_QSPI_SD1_EDGE_HIGH bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd1_edge_high(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD1_EDGE_HIGH_MASK);
    }

    /// **GPIO_QSPI_SD1_EDGE_LOW** [22:22]
    #[inline(always)]
    pub fn gpio_qspi_sd1_edge_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD1_EDGE_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SD1_EDGE_LOW bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd1_edge_low(&self) {
        self.set_bits(Self::GPIO_QSPI_SD1_EDGE_LOW_MASK);
    }

    /// Clear GPIO_QSPI_SD1_EDGE_LOW bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd1_edge_low(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD1_EDGE_LOW_MASK);
    }

    /// **GPIO_QSPI_SD1_LEVEL_HIGH** [21:21]
    #[inline(always)]
    pub fn gpio_qspi_sd1_level_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD1_LEVEL_HIGH_MASK) != 0
    }

    /// Set GPIO_QSPI_SD1_LEVEL_HIGH bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd1_level_high(&self) {
        self.set_bits(Self::GPIO_QSPI_SD1_LEVEL_HIGH_MASK);
    }

    /// Clear GPIO_QSPI_SD1_LEVEL_HIGH bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd1_level_high(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD1_LEVEL_HIGH_MASK);
    }

    /// **GPIO_QSPI_SD1_LEVEL_LOW** [20:20]
    #[inline(always)]
    pub fn gpio_qspi_sd1_level_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD1_LEVEL_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SD1_LEVEL_LOW bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd1_level_low(&self) {
        self.set_bits(Self::GPIO_QSPI_SD1_LEVEL_LOW_MASK);
    }

    /// Clear GPIO_QSPI_SD1_LEVEL_LOW bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd1_level_low(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD1_LEVEL_LOW_MASK);
    }

    /// **GPIO_QSPI_SD0_EDGE_HIGH** [19:19]
    #[inline(always)]
    pub fn gpio_qspi_sd0_edge_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD0_EDGE_HIGH_MASK) != 0
    }

    /// Set GPIO_QSPI_SD0_EDGE_HIGH bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd0_edge_high(&self) {
        self.set_bits(Self::GPIO_QSPI_SD0_EDGE_HIGH_MASK);
    }

    /// Clear GPIO_QSPI_SD0_EDGE_HIGH bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd0_edge_high(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD0_EDGE_HIGH_MASK);
    }

    /// **GPIO_QSPI_SD0_EDGE_LOW** [18:18]
    #[inline(always)]
    pub fn gpio_qspi_sd0_edge_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD0_EDGE_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SD0_EDGE_LOW bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd0_edge_low(&self) {
        self.set_bits(Self::GPIO_QSPI_SD0_EDGE_LOW_MASK);
    }

    /// Clear GPIO_QSPI_SD0_EDGE_LOW bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd0_edge_low(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD0_EDGE_LOW_MASK);
    }

    /// **GPIO_QSPI_SD0_LEVEL_HIGH** [17:17]
    #[inline(always)]
    pub fn gpio_qspi_sd0_level_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD0_LEVEL_HIGH_MASK) != 0
    }

    /// Set GPIO_QSPI_SD0_LEVEL_HIGH bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd0_level_high(&self) {
        self.set_bits(Self::GPIO_QSPI_SD0_LEVEL_HIGH_MASK);
    }

    /// Clear GPIO_QSPI_SD0_LEVEL_HIGH bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd0_level_high(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD0_LEVEL_HIGH_MASK);
    }

    /// **GPIO_QSPI_SD0_LEVEL_LOW** [16:16]
    #[inline(always)]
    pub fn gpio_qspi_sd0_level_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD0_LEVEL_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SD0_LEVEL_LOW bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd0_level_low(&self) {
        self.set_bits(Self::GPIO_QSPI_SD0_LEVEL_LOW_MASK);
    }

    /// Clear GPIO_QSPI_SD0_LEVEL_LOW bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd0_level_low(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD0_LEVEL_LOW_MASK);
    }

    /// **GPIO_QSPI_SS_EDGE_HIGH** [15:15]
    #[inline(always)]
    pub fn gpio_qspi_ss_edge_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SS_EDGE_HIGH_MASK) != 0
    }

    /// Set GPIO_QSPI_SS_EDGE_HIGH bit
    #[inline(always)]
    pub fn set_gpio_qspi_ss_edge_high(&self) {
        self.set_bits(Self::GPIO_QSPI_SS_EDGE_HIGH_MASK);
    }

    /// Clear GPIO_QSPI_SS_EDGE_HIGH bit
    #[inline(always)]
    pub fn clr_gpio_qspi_ss_edge_high(&self) {
        self.clr_bits(Self::GPIO_QSPI_SS_EDGE_HIGH_MASK);
    }

    /// **GPIO_QSPI_SS_EDGE_LOW** [14:14]
    #[inline(always)]
    pub fn gpio_qspi_ss_edge_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SS_EDGE_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SS_EDGE_LOW bit
    #[inline(always)]
    pub fn set_gpio_qspi_ss_edge_low(&self) {
        self.set_bits(Self::GPIO_QSPI_SS_EDGE_LOW_MASK);
    }

    /// Clear GPIO_QSPI_SS_EDGE_LOW bit
    #[inline(always)]
    pub fn clr_gpio_qspi_ss_edge_low(&self) {
        self.clr_bits(Self::GPIO_QSPI_SS_EDGE_LOW_MASK);
    }

    /// **GPIO_QSPI_SS_LEVEL_HIGH** [13:13]
    #[inline(always)]
    pub fn gpio_qspi_ss_level_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SS_LEVEL_HIGH_MASK) != 0
    }

    /// Set GPIO_QSPI_SS_LEVEL_HIGH bit
    #[inline(always)]
    pub fn set_gpio_qspi_ss_level_high(&self) {
        self.set_bits(Self::GPIO_QSPI_SS_LEVEL_HIGH_MASK);
    }

    /// Clear GPIO_QSPI_SS_LEVEL_HIGH bit
    #[inline(always)]
    pub fn clr_gpio_qspi_ss_level_high(&self) {
        self.clr_bits(Self::GPIO_QSPI_SS_LEVEL_HIGH_MASK);
    }

    /// **GPIO_QSPI_SS_LEVEL_LOW** [12:12]
    #[inline(always)]
    pub fn gpio_qspi_ss_level_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SS_LEVEL_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SS_LEVEL_LOW bit
    #[inline(always)]
    pub fn set_gpio_qspi_ss_level_low(&self) {
        self.set_bits(Self::GPIO_QSPI_SS_LEVEL_LOW_MASK);
    }

    /// Clear GPIO_QSPI_SS_LEVEL_LOW bit
    #[inline(always)]
    pub fn clr_gpio_qspi_ss_level_low(&self) {
        self.clr_bits(Self::GPIO_QSPI_SS_LEVEL_LOW_MASK);
    }

    /// **GPIO_QSPI_SCLK_EDGE_HIGH** [11:11]
    #[inline(always)]
    pub fn gpio_qspi_sclk_edge_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SCLK_EDGE_HIGH_MASK) != 0
    }

    /// Set GPIO_QSPI_SCLK_EDGE_HIGH bit
    #[inline(always)]
    pub fn set_gpio_qspi_sclk_edge_high(&self) {
        self.set_bits(Self::GPIO_QSPI_SCLK_EDGE_HIGH_MASK);
    }

    /// Clear GPIO_QSPI_SCLK_EDGE_HIGH bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sclk_edge_high(&self) {
        self.clr_bits(Self::GPIO_QSPI_SCLK_EDGE_HIGH_MASK);
    }

    /// **GPIO_QSPI_SCLK_EDGE_LOW** [10:10]
    #[inline(always)]
    pub fn gpio_qspi_sclk_edge_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SCLK_EDGE_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SCLK_EDGE_LOW bit
    #[inline(always)]
    pub fn set_gpio_qspi_sclk_edge_low(&self) {
        self.set_bits(Self::GPIO_QSPI_SCLK_EDGE_LOW_MASK);
    }

    /// Clear GPIO_QSPI_SCLK_EDGE_LOW bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sclk_edge_low(&self) {
        self.clr_bits(Self::GPIO_QSPI_SCLK_EDGE_LOW_MASK);
    }

    /// **GPIO_QSPI_SCLK_LEVEL_HIGH** [9:9]
    #[inline(always)]
    pub fn gpio_qspi_sclk_level_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SCLK_LEVEL_HIGH_MASK) != 0
    }

    /// Set GPIO_QSPI_SCLK_LEVEL_HIGH bit
    #[inline(always)]
    pub fn set_gpio_qspi_sclk_level_high(&self) {
        self.set_bits(Self::GPIO_QSPI_SCLK_LEVEL_HIGH_MASK);
    }

    /// Clear GPIO_QSPI_SCLK_LEVEL_HIGH bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sclk_level_high(&self) {
        self.clr_bits(Self::GPIO_QSPI_SCLK_LEVEL_HIGH_MASK);
    }

    /// **GPIO_QSPI_SCLK_LEVEL_LOW** [8:8]
    #[inline(always)]
    pub fn gpio_qspi_sclk_level_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SCLK_LEVEL_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SCLK_LEVEL_LOW bit
    #[inline(always)]
    pub fn set_gpio_qspi_sclk_level_low(&self) {
        self.set_bits(Self::GPIO_QSPI_SCLK_LEVEL_LOW_MASK);
    }

    /// Clear GPIO_QSPI_SCLK_LEVEL_LOW bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sclk_level_low(&self) {
        self.clr_bits(Self::GPIO_QSPI_SCLK_LEVEL_LOW_MASK);
    }

    /// **USBPHY_DM_EDGE_HIGH** [7:7]
    #[inline(always)]
    pub fn usbphy_dm_edge_high(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DM_EDGE_HIGH_MASK) != 0
    }

    /// Set USBPHY_DM_EDGE_HIGH bit
    #[inline(always)]
    pub fn set_usbphy_dm_edge_high(&self) {
        self.set_bits(Self::USBPHY_DM_EDGE_HIGH_MASK);
    }

    /// Clear USBPHY_DM_EDGE_HIGH bit
    #[inline(always)]
    pub fn clr_usbphy_dm_edge_high(&self) {
        self.clr_bits(Self::USBPHY_DM_EDGE_HIGH_MASK);
    }

    /// **USBPHY_DM_EDGE_LOW** [6:6]
    #[inline(always)]
    pub fn usbphy_dm_edge_low(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DM_EDGE_LOW_MASK) != 0
    }

    /// Set USBPHY_DM_EDGE_LOW bit
    #[inline(always)]
    pub fn set_usbphy_dm_edge_low(&self) {
        self.set_bits(Self::USBPHY_DM_EDGE_LOW_MASK);
    }

    /// Clear USBPHY_DM_EDGE_LOW bit
    #[inline(always)]
    pub fn clr_usbphy_dm_edge_low(&self) {
        self.clr_bits(Self::USBPHY_DM_EDGE_LOW_MASK);
    }

    /// **USBPHY_DM_LEVEL_HIGH** [5:5]
    #[inline(always)]
    pub fn usbphy_dm_level_high(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DM_LEVEL_HIGH_MASK) != 0
    }

    /// Set USBPHY_DM_LEVEL_HIGH bit
    #[inline(always)]
    pub fn set_usbphy_dm_level_high(&self) {
        self.set_bits(Self::USBPHY_DM_LEVEL_HIGH_MASK);
    }

    /// Clear USBPHY_DM_LEVEL_HIGH bit
    #[inline(always)]
    pub fn clr_usbphy_dm_level_high(&self) {
        self.clr_bits(Self::USBPHY_DM_LEVEL_HIGH_MASK);
    }

    /// **USBPHY_DM_LEVEL_LOW** [4:4]
    #[inline(always)]
    pub fn usbphy_dm_level_low(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DM_LEVEL_LOW_MASK) != 0
    }

    /// Set USBPHY_DM_LEVEL_LOW bit
    #[inline(always)]
    pub fn set_usbphy_dm_level_low(&self) {
        self.set_bits(Self::USBPHY_DM_LEVEL_LOW_MASK);
    }

    /// Clear USBPHY_DM_LEVEL_LOW bit
    #[inline(always)]
    pub fn clr_usbphy_dm_level_low(&self) {
        self.clr_bits(Self::USBPHY_DM_LEVEL_LOW_MASK);
    }

    /// **USBPHY_DP_EDGE_HIGH** [3:3]
    #[inline(always)]
    pub fn usbphy_dp_edge_high(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DP_EDGE_HIGH_MASK) != 0
    }

    /// Set USBPHY_DP_EDGE_HIGH bit
    #[inline(always)]
    pub fn set_usbphy_dp_edge_high(&self) {
        self.set_bits(Self::USBPHY_DP_EDGE_HIGH_MASK);
    }

    /// Clear USBPHY_DP_EDGE_HIGH bit
    #[inline(always)]
    pub fn clr_usbphy_dp_edge_high(&self) {
        self.clr_bits(Self::USBPHY_DP_EDGE_HIGH_MASK);
    }

    /// **USBPHY_DP_EDGE_LOW** [2:2]
    #[inline(always)]
    pub fn usbphy_dp_edge_low(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DP_EDGE_LOW_MASK) != 0
    }

    /// Set USBPHY_DP_EDGE_LOW bit
    #[inline(always)]
    pub fn set_usbphy_dp_edge_low(&self) {
        self.set_bits(Self::USBPHY_DP_EDGE_LOW_MASK);
    }

    /// Clear USBPHY_DP_EDGE_LOW bit
    #[inline(always)]
    pub fn clr_usbphy_dp_edge_low(&self) {
        self.clr_bits(Self::USBPHY_DP_EDGE_LOW_MASK);
    }

    /// **USBPHY_DP_LEVEL_HIGH** [1:1]
    #[inline(always)]
    pub fn usbphy_dp_level_high(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DP_LEVEL_HIGH_MASK) != 0
    }

    /// Set USBPHY_DP_LEVEL_HIGH bit
    #[inline(always)]
    pub fn set_usbphy_dp_level_high(&self) {
        self.set_bits(Self::USBPHY_DP_LEVEL_HIGH_MASK);
    }

    /// Clear USBPHY_DP_LEVEL_HIGH bit
    #[inline(always)]
    pub fn clr_usbphy_dp_level_high(&self) {
        self.clr_bits(Self::USBPHY_DP_LEVEL_HIGH_MASK);
    }

    /// **USBPHY_DP_LEVEL_LOW** [0:0]
    #[inline(always)]
    pub fn usbphy_dp_level_low(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DP_LEVEL_LOW_MASK) != 0
    }

    /// Set USBPHY_DP_LEVEL_LOW bit
    #[inline(always)]
    pub fn set_usbphy_dp_level_low(&self) {
        self.set_bits(Self::USBPHY_DP_LEVEL_LOW_MASK);
    }

    /// Clear USBPHY_DP_LEVEL_LOW bit
    #[inline(always)]
    pub fn clr_usbphy_dp_level_low(&self) {
        self.clr_bits(Self::USBPHY_DP_LEVEL_LOW_MASK);
    }

}

/// PROC0_INTE value type for batch read/modify/write operations
///
/// Use `proc0inte.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `proc0inte.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Proc0InteVal {
    bits: u32,
}

impl Proc0InteVal {
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

    /// Read GPIO_QSPI_SD3_EDGE_HIGH [31:31]
    #[inline(always)]
    pub const fn gpio_qspi_sd3_edge_high(&self) -> bool {
        (self.bits & Proc0Inte::GPIO_QSPI_SD3_EDGE_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD3_EDGE_LOW [30:30]
    #[inline(always)]
    pub const fn gpio_qspi_sd3_edge_low(&self) -> bool {
        (self.bits & Proc0Inte::GPIO_QSPI_SD3_EDGE_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SD3_LEVEL_HIGH [29:29]
    #[inline(always)]
    pub const fn gpio_qspi_sd3_level_high(&self) -> bool {
        (self.bits & Proc0Inte::GPIO_QSPI_SD3_LEVEL_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD3_LEVEL_LOW [28:28]
    #[inline(always)]
    pub const fn gpio_qspi_sd3_level_low(&self) -> bool {
        (self.bits & Proc0Inte::GPIO_QSPI_SD3_LEVEL_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SD2_EDGE_HIGH [27:27]
    #[inline(always)]
    pub const fn gpio_qspi_sd2_edge_high(&self) -> bool {
        (self.bits & Proc0Inte::GPIO_QSPI_SD2_EDGE_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD2_EDGE_LOW [26:26]
    #[inline(always)]
    pub const fn gpio_qspi_sd2_edge_low(&self) -> bool {
        (self.bits & Proc0Inte::GPIO_QSPI_SD2_EDGE_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SD2_LEVEL_HIGH [25:25]
    #[inline(always)]
    pub const fn gpio_qspi_sd2_level_high(&self) -> bool {
        (self.bits & Proc0Inte::GPIO_QSPI_SD2_LEVEL_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD2_LEVEL_LOW [24:24]
    #[inline(always)]
    pub const fn gpio_qspi_sd2_level_low(&self) -> bool {
        (self.bits & Proc0Inte::GPIO_QSPI_SD2_LEVEL_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SD1_EDGE_HIGH [23:23]
    #[inline(always)]
    pub const fn gpio_qspi_sd1_edge_high(&self) -> bool {
        (self.bits & Proc0Inte::GPIO_QSPI_SD1_EDGE_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD1_EDGE_LOW [22:22]
    #[inline(always)]
    pub const fn gpio_qspi_sd1_edge_low(&self) -> bool {
        (self.bits & Proc0Inte::GPIO_QSPI_SD1_EDGE_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SD1_LEVEL_HIGH [21:21]
    #[inline(always)]
    pub const fn gpio_qspi_sd1_level_high(&self) -> bool {
        (self.bits & Proc0Inte::GPIO_QSPI_SD1_LEVEL_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD1_LEVEL_LOW [20:20]
    #[inline(always)]
    pub const fn gpio_qspi_sd1_level_low(&self) -> bool {
        (self.bits & Proc0Inte::GPIO_QSPI_SD1_LEVEL_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SD0_EDGE_HIGH [19:19]
    #[inline(always)]
    pub const fn gpio_qspi_sd0_edge_high(&self) -> bool {
        (self.bits & Proc0Inte::GPIO_QSPI_SD0_EDGE_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD0_EDGE_LOW [18:18]
    #[inline(always)]
    pub const fn gpio_qspi_sd0_edge_low(&self) -> bool {
        (self.bits & Proc0Inte::GPIO_QSPI_SD0_EDGE_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SD0_LEVEL_HIGH [17:17]
    #[inline(always)]
    pub const fn gpio_qspi_sd0_level_high(&self) -> bool {
        (self.bits & Proc0Inte::GPIO_QSPI_SD0_LEVEL_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD0_LEVEL_LOW [16:16]
    #[inline(always)]
    pub const fn gpio_qspi_sd0_level_low(&self) -> bool {
        (self.bits & Proc0Inte::GPIO_QSPI_SD0_LEVEL_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SS_EDGE_HIGH [15:15]
    #[inline(always)]
    pub const fn gpio_qspi_ss_edge_high(&self) -> bool {
        (self.bits & Proc0Inte::GPIO_QSPI_SS_EDGE_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SS_EDGE_LOW [14:14]
    #[inline(always)]
    pub const fn gpio_qspi_ss_edge_low(&self) -> bool {
        (self.bits & Proc0Inte::GPIO_QSPI_SS_EDGE_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SS_LEVEL_HIGH [13:13]
    #[inline(always)]
    pub const fn gpio_qspi_ss_level_high(&self) -> bool {
        (self.bits & Proc0Inte::GPIO_QSPI_SS_LEVEL_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SS_LEVEL_LOW [12:12]
    #[inline(always)]
    pub const fn gpio_qspi_ss_level_low(&self) -> bool {
        (self.bits & Proc0Inte::GPIO_QSPI_SS_LEVEL_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SCLK_EDGE_HIGH [11:11]
    #[inline(always)]
    pub const fn gpio_qspi_sclk_edge_high(&self) -> bool {
        (self.bits & Proc0Inte::GPIO_QSPI_SCLK_EDGE_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SCLK_EDGE_LOW [10:10]
    #[inline(always)]
    pub const fn gpio_qspi_sclk_edge_low(&self) -> bool {
        (self.bits & Proc0Inte::GPIO_QSPI_SCLK_EDGE_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SCLK_LEVEL_HIGH [9:9]
    #[inline(always)]
    pub const fn gpio_qspi_sclk_level_high(&self) -> bool {
        (self.bits & Proc0Inte::GPIO_QSPI_SCLK_LEVEL_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SCLK_LEVEL_LOW [8:8]
    #[inline(always)]
    pub const fn gpio_qspi_sclk_level_low(&self) -> bool {
        (self.bits & Proc0Inte::GPIO_QSPI_SCLK_LEVEL_LOW_MASK) != 0
    }

    /// Read USBPHY_DM_EDGE_HIGH [7:7]
    #[inline(always)]
    pub const fn usbphy_dm_edge_high(&self) -> bool {
        (self.bits & Proc0Inte::USBPHY_DM_EDGE_HIGH_MASK) != 0
    }

    /// Read USBPHY_DM_EDGE_LOW [6:6]
    #[inline(always)]
    pub const fn usbphy_dm_edge_low(&self) -> bool {
        (self.bits & Proc0Inte::USBPHY_DM_EDGE_LOW_MASK) != 0
    }

    /// Read USBPHY_DM_LEVEL_HIGH [5:5]
    #[inline(always)]
    pub const fn usbphy_dm_level_high(&self) -> bool {
        (self.bits & Proc0Inte::USBPHY_DM_LEVEL_HIGH_MASK) != 0
    }

    /// Read USBPHY_DM_LEVEL_LOW [4:4]
    #[inline(always)]
    pub const fn usbphy_dm_level_low(&self) -> bool {
        (self.bits & Proc0Inte::USBPHY_DM_LEVEL_LOW_MASK) != 0
    }

    /// Read USBPHY_DP_EDGE_HIGH [3:3]
    #[inline(always)]
    pub const fn usbphy_dp_edge_high(&self) -> bool {
        (self.bits & Proc0Inte::USBPHY_DP_EDGE_HIGH_MASK) != 0
    }

    /// Read USBPHY_DP_EDGE_LOW [2:2]
    #[inline(always)]
    pub const fn usbphy_dp_edge_low(&self) -> bool {
        (self.bits & Proc0Inte::USBPHY_DP_EDGE_LOW_MASK) != 0
    }

    /// Read USBPHY_DP_LEVEL_HIGH [1:1]
    #[inline(always)]
    pub const fn usbphy_dp_level_high(&self) -> bool {
        (self.bits & Proc0Inte::USBPHY_DP_LEVEL_HIGH_MASK) != 0
    }

    /// Read USBPHY_DP_LEVEL_LOW [0:0]
    #[inline(always)]
    pub const fn usbphy_dp_level_low(&self) -> bool {
        (self.bits & Proc0Inte::USBPHY_DP_LEVEL_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SD3_EDGE_HIGH [31:31]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd3_edge_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc0Inte::GPIO_QSPI_SD3_EDGE_HIGH_MASK
            } else {
                self.bits & !Proc0Inte::GPIO_QSPI_SD3_EDGE_HIGH_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD3_EDGE_LOW [30:30]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd3_edge_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc0Inte::GPIO_QSPI_SD3_EDGE_LOW_MASK
            } else {
                self.bits & !Proc0Inte::GPIO_QSPI_SD3_EDGE_LOW_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD3_LEVEL_HIGH [29:29]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd3_level_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc0Inte::GPIO_QSPI_SD3_LEVEL_HIGH_MASK
            } else {
                self.bits & !Proc0Inte::GPIO_QSPI_SD3_LEVEL_HIGH_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD3_LEVEL_LOW [28:28]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd3_level_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc0Inte::GPIO_QSPI_SD3_LEVEL_LOW_MASK
            } else {
                self.bits & !Proc0Inte::GPIO_QSPI_SD3_LEVEL_LOW_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD2_EDGE_HIGH [27:27]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd2_edge_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc0Inte::GPIO_QSPI_SD2_EDGE_HIGH_MASK
            } else {
                self.bits & !Proc0Inte::GPIO_QSPI_SD2_EDGE_HIGH_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD2_EDGE_LOW [26:26]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd2_edge_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc0Inte::GPIO_QSPI_SD2_EDGE_LOW_MASK
            } else {
                self.bits & !Proc0Inte::GPIO_QSPI_SD2_EDGE_LOW_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD2_LEVEL_HIGH [25:25]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd2_level_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc0Inte::GPIO_QSPI_SD2_LEVEL_HIGH_MASK
            } else {
                self.bits & !Proc0Inte::GPIO_QSPI_SD2_LEVEL_HIGH_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD2_LEVEL_LOW [24:24]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd2_level_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc0Inte::GPIO_QSPI_SD2_LEVEL_LOW_MASK
            } else {
                self.bits & !Proc0Inte::GPIO_QSPI_SD2_LEVEL_LOW_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD1_EDGE_HIGH [23:23]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd1_edge_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc0Inte::GPIO_QSPI_SD1_EDGE_HIGH_MASK
            } else {
                self.bits & !Proc0Inte::GPIO_QSPI_SD1_EDGE_HIGH_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD1_EDGE_LOW [22:22]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd1_edge_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc0Inte::GPIO_QSPI_SD1_EDGE_LOW_MASK
            } else {
                self.bits & !Proc0Inte::GPIO_QSPI_SD1_EDGE_LOW_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD1_LEVEL_HIGH [21:21]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd1_level_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc0Inte::GPIO_QSPI_SD1_LEVEL_HIGH_MASK
            } else {
                self.bits & !Proc0Inte::GPIO_QSPI_SD1_LEVEL_HIGH_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD1_LEVEL_LOW [20:20]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd1_level_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc0Inte::GPIO_QSPI_SD1_LEVEL_LOW_MASK
            } else {
                self.bits & !Proc0Inte::GPIO_QSPI_SD1_LEVEL_LOW_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD0_EDGE_HIGH [19:19]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd0_edge_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc0Inte::GPIO_QSPI_SD0_EDGE_HIGH_MASK
            } else {
                self.bits & !Proc0Inte::GPIO_QSPI_SD0_EDGE_HIGH_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD0_EDGE_LOW [18:18]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd0_edge_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc0Inte::GPIO_QSPI_SD0_EDGE_LOW_MASK
            } else {
                self.bits & !Proc0Inte::GPIO_QSPI_SD0_EDGE_LOW_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD0_LEVEL_HIGH [17:17]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd0_level_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc0Inte::GPIO_QSPI_SD0_LEVEL_HIGH_MASK
            } else {
                self.bits & !Proc0Inte::GPIO_QSPI_SD0_LEVEL_HIGH_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD0_LEVEL_LOW [16:16]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd0_level_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc0Inte::GPIO_QSPI_SD0_LEVEL_LOW_MASK
            } else {
                self.bits & !Proc0Inte::GPIO_QSPI_SD0_LEVEL_LOW_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SS_EDGE_HIGH [15:15]
    #[inline(always)]
    pub const fn with_gpio_qspi_ss_edge_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc0Inte::GPIO_QSPI_SS_EDGE_HIGH_MASK
            } else {
                self.bits & !Proc0Inte::GPIO_QSPI_SS_EDGE_HIGH_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SS_EDGE_LOW [14:14]
    #[inline(always)]
    pub const fn with_gpio_qspi_ss_edge_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc0Inte::GPIO_QSPI_SS_EDGE_LOW_MASK
            } else {
                self.bits & !Proc0Inte::GPIO_QSPI_SS_EDGE_LOW_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SS_LEVEL_HIGH [13:13]
    #[inline(always)]
    pub const fn with_gpio_qspi_ss_level_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc0Inte::GPIO_QSPI_SS_LEVEL_HIGH_MASK
            } else {
                self.bits & !Proc0Inte::GPIO_QSPI_SS_LEVEL_HIGH_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SS_LEVEL_LOW [12:12]
    #[inline(always)]
    pub const fn with_gpio_qspi_ss_level_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc0Inte::GPIO_QSPI_SS_LEVEL_LOW_MASK
            } else {
                self.bits & !Proc0Inte::GPIO_QSPI_SS_LEVEL_LOW_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SCLK_EDGE_HIGH [11:11]
    #[inline(always)]
    pub const fn with_gpio_qspi_sclk_edge_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc0Inte::GPIO_QSPI_SCLK_EDGE_HIGH_MASK
            } else {
                self.bits & !Proc0Inte::GPIO_QSPI_SCLK_EDGE_HIGH_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SCLK_EDGE_LOW [10:10]
    #[inline(always)]
    pub const fn with_gpio_qspi_sclk_edge_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc0Inte::GPIO_QSPI_SCLK_EDGE_LOW_MASK
            } else {
                self.bits & !Proc0Inte::GPIO_QSPI_SCLK_EDGE_LOW_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SCLK_LEVEL_HIGH [9:9]
    #[inline(always)]
    pub const fn with_gpio_qspi_sclk_level_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc0Inte::GPIO_QSPI_SCLK_LEVEL_HIGH_MASK
            } else {
                self.bits & !Proc0Inte::GPIO_QSPI_SCLK_LEVEL_HIGH_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SCLK_LEVEL_LOW [8:8]
    #[inline(always)]
    pub const fn with_gpio_qspi_sclk_level_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc0Inte::GPIO_QSPI_SCLK_LEVEL_LOW_MASK
            } else {
                self.bits & !Proc0Inte::GPIO_QSPI_SCLK_LEVEL_LOW_MASK
            }
        }
    }

    /// Set USBPHY_DM_EDGE_HIGH [7:7]
    #[inline(always)]
    pub const fn with_usbphy_dm_edge_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc0Inte::USBPHY_DM_EDGE_HIGH_MASK
            } else {
                self.bits & !Proc0Inte::USBPHY_DM_EDGE_HIGH_MASK
            }
        }
    }

    /// Set USBPHY_DM_EDGE_LOW [6:6]
    #[inline(always)]
    pub const fn with_usbphy_dm_edge_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc0Inte::USBPHY_DM_EDGE_LOW_MASK
            } else {
                self.bits & !Proc0Inte::USBPHY_DM_EDGE_LOW_MASK
            }
        }
    }

    /// Set USBPHY_DM_LEVEL_HIGH [5:5]
    #[inline(always)]
    pub const fn with_usbphy_dm_level_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc0Inte::USBPHY_DM_LEVEL_HIGH_MASK
            } else {
                self.bits & !Proc0Inte::USBPHY_DM_LEVEL_HIGH_MASK
            }
        }
    }

    /// Set USBPHY_DM_LEVEL_LOW [4:4]
    #[inline(always)]
    pub const fn with_usbphy_dm_level_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc0Inte::USBPHY_DM_LEVEL_LOW_MASK
            } else {
                self.bits & !Proc0Inte::USBPHY_DM_LEVEL_LOW_MASK
            }
        }
    }

    /// Set USBPHY_DP_EDGE_HIGH [3:3]
    #[inline(always)]
    pub const fn with_usbphy_dp_edge_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc0Inte::USBPHY_DP_EDGE_HIGH_MASK
            } else {
                self.bits & !Proc0Inte::USBPHY_DP_EDGE_HIGH_MASK
            }
        }
    }

    /// Set USBPHY_DP_EDGE_LOW [2:2]
    #[inline(always)]
    pub const fn with_usbphy_dp_edge_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc0Inte::USBPHY_DP_EDGE_LOW_MASK
            } else {
                self.bits & !Proc0Inte::USBPHY_DP_EDGE_LOW_MASK
            }
        }
    }

    /// Set USBPHY_DP_LEVEL_HIGH [1:1]
    #[inline(always)]
    pub const fn with_usbphy_dp_level_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc0Inte::USBPHY_DP_LEVEL_HIGH_MASK
            } else {
                self.bits & !Proc0Inte::USBPHY_DP_LEVEL_HIGH_MASK
            }
        }
    }

    /// Set USBPHY_DP_LEVEL_LOW [0:0]
    #[inline(always)]
    pub const fn with_usbphy_dp_level_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc0Inte::USBPHY_DP_LEVEL_LOW_MASK
            } else {
                self.bits & !Proc0Inte::USBPHY_DP_LEVEL_LOW_MASK
            }
        }
    }

}

impl Default for Proc0InteVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Proc0Inte {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> Proc0InteVal {
        Proc0InteVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: Proc0InteVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(Proc0InteVal) -> Proc0InteVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// PROC0_INTF Register
///
/// Interrupt Force for proc0
///
/// Address offset: 0x0220
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **GPIO_QSPI_SD3_EDGE_HIGH** [31] (RW): 
/// - **GPIO_QSPI_SD3_EDGE_LOW** [30] (RW): 
/// - **GPIO_QSPI_SD3_LEVEL_HIGH** [29] (RW): 
/// - **GPIO_QSPI_SD3_LEVEL_LOW** [28] (RW): 
/// - **GPIO_QSPI_SD2_EDGE_HIGH** [27] (RW): 
/// - **GPIO_QSPI_SD2_EDGE_LOW** [26] (RW): 
/// - **GPIO_QSPI_SD2_LEVEL_HIGH** [25] (RW): 
/// - **GPIO_QSPI_SD2_LEVEL_LOW** [24] (RW): 
/// - **GPIO_QSPI_SD1_EDGE_HIGH** [23] (RW): 
/// - **GPIO_QSPI_SD1_EDGE_LOW** [22] (RW): 
/// - **GPIO_QSPI_SD1_LEVEL_HIGH** [21] (RW): 
/// - **GPIO_QSPI_SD1_LEVEL_LOW** [20] (RW): 
/// - **GPIO_QSPI_SD0_EDGE_HIGH** [19] (RW): 
/// - **GPIO_QSPI_SD0_EDGE_LOW** [18] (RW): 
/// - **GPIO_QSPI_SD0_LEVEL_HIGH** [17] (RW): 
/// - **GPIO_QSPI_SD0_LEVEL_LOW** [16] (RW): 
/// - **GPIO_QSPI_SS_EDGE_HIGH** [15] (RW): 
/// - **GPIO_QSPI_SS_EDGE_LOW** [14] (RW): 
/// - **GPIO_QSPI_SS_LEVEL_HIGH** [13] (RW): 
/// - **GPIO_QSPI_SS_LEVEL_LOW** [12] (RW): 
/// - **GPIO_QSPI_SCLK_EDGE_HIGH** [11] (RW): 
/// - **GPIO_QSPI_SCLK_EDGE_LOW** [10] (RW): 
/// - **GPIO_QSPI_SCLK_LEVEL_HIGH** [9] (RW): 
/// - **GPIO_QSPI_SCLK_LEVEL_LOW** [8] (RW): 
/// - **USBPHY_DM_EDGE_HIGH** [7] (RW): 
/// - **USBPHY_DM_EDGE_LOW** [6] (RW): 
/// - **USBPHY_DM_LEVEL_HIGH** [5] (RW): 
/// - **USBPHY_DM_LEVEL_LOW** [4] (RW): 
/// - **USBPHY_DP_EDGE_HIGH** [3] (RW): 
/// - **USBPHY_DP_EDGE_LOW** [2] (RW): 
/// - **USBPHY_DP_LEVEL_HIGH** [1] (RW): 
/// - **USBPHY_DP_LEVEL_LOW** [0] (RW): 
#[repr(transparent)]
pub struct Proc0Intf {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for Proc0Intf {}
unsafe impl Sync for Proc0Intf {}

impl Proc0Intf {
    // ======== Field Constants ========

    /// Bits [31:31], mask 0x80000000
    pub const GPIO_QSPI_SD3_EDGE_HIGH_OFFSET: u32 = 31;
    pub const GPIO_QSPI_SD3_EDGE_HIGH_MASK: u32 = 0x80000000;
    pub const GPIO_QSPI_SD3_EDGE_HIGH: u32 = 1 << 31;

    /// Bits [30:30], mask 0x40000000
    pub const GPIO_QSPI_SD3_EDGE_LOW_OFFSET: u32 = 30;
    pub const GPIO_QSPI_SD3_EDGE_LOW_MASK: u32 = 0x40000000;
    pub const GPIO_QSPI_SD3_EDGE_LOW: u32 = 1 << 30;

    /// Bits [29:29], mask 0x20000000
    pub const GPIO_QSPI_SD3_LEVEL_HIGH_OFFSET: u32 = 29;
    pub const GPIO_QSPI_SD3_LEVEL_HIGH_MASK: u32 = 0x20000000;
    pub const GPIO_QSPI_SD3_LEVEL_HIGH: u32 = 1 << 29;

    /// Bits [28:28], mask 0x10000000
    pub const GPIO_QSPI_SD3_LEVEL_LOW_OFFSET: u32 = 28;
    pub const GPIO_QSPI_SD3_LEVEL_LOW_MASK: u32 = 0x10000000;
    pub const GPIO_QSPI_SD3_LEVEL_LOW: u32 = 1 << 28;

    /// Bits [27:27], mask 0x08000000
    pub const GPIO_QSPI_SD2_EDGE_HIGH_OFFSET: u32 = 27;
    pub const GPIO_QSPI_SD2_EDGE_HIGH_MASK: u32 = 0x08000000;
    pub const GPIO_QSPI_SD2_EDGE_HIGH: u32 = 1 << 27;

    /// Bits [26:26], mask 0x04000000
    pub const GPIO_QSPI_SD2_EDGE_LOW_OFFSET: u32 = 26;
    pub const GPIO_QSPI_SD2_EDGE_LOW_MASK: u32 = 0x04000000;
    pub const GPIO_QSPI_SD2_EDGE_LOW: u32 = 1 << 26;

    /// Bits [25:25], mask 0x02000000
    pub const GPIO_QSPI_SD2_LEVEL_HIGH_OFFSET: u32 = 25;
    pub const GPIO_QSPI_SD2_LEVEL_HIGH_MASK: u32 = 0x02000000;
    pub const GPIO_QSPI_SD2_LEVEL_HIGH: u32 = 1 << 25;

    /// Bits [24:24], mask 0x01000000
    pub const GPIO_QSPI_SD2_LEVEL_LOW_OFFSET: u32 = 24;
    pub const GPIO_QSPI_SD2_LEVEL_LOW_MASK: u32 = 0x01000000;
    pub const GPIO_QSPI_SD2_LEVEL_LOW: u32 = 1 << 24;

    /// Bits [23:23], mask 0x00800000
    pub const GPIO_QSPI_SD1_EDGE_HIGH_OFFSET: u32 = 23;
    pub const GPIO_QSPI_SD1_EDGE_HIGH_MASK: u32 = 0x00800000;
    pub const GPIO_QSPI_SD1_EDGE_HIGH: u32 = 1 << 23;

    /// Bits [22:22], mask 0x00400000
    pub const GPIO_QSPI_SD1_EDGE_LOW_OFFSET: u32 = 22;
    pub const GPIO_QSPI_SD1_EDGE_LOW_MASK: u32 = 0x00400000;
    pub const GPIO_QSPI_SD1_EDGE_LOW: u32 = 1 << 22;

    /// Bits [21:21], mask 0x00200000
    pub const GPIO_QSPI_SD1_LEVEL_HIGH_OFFSET: u32 = 21;
    pub const GPIO_QSPI_SD1_LEVEL_HIGH_MASK: u32 = 0x00200000;
    pub const GPIO_QSPI_SD1_LEVEL_HIGH: u32 = 1 << 21;

    /// Bits [20:20], mask 0x00100000
    pub const GPIO_QSPI_SD1_LEVEL_LOW_OFFSET: u32 = 20;
    pub const GPIO_QSPI_SD1_LEVEL_LOW_MASK: u32 = 0x00100000;
    pub const GPIO_QSPI_SD1_LEVEL_LOW: u32 = 1 << 20;

    /// Bits [19:19], mask 0x00080000
    pub const GPIO_QSPI_SD0_EDGE_HIGH_OFFSET: u32 = 19;
    pub const GPIO_QSPI_SD0_EDGE_HIGH_MASK: u32 = 0x00080000;
    pub const GPIO_QSPI_SD0_EDGE_HIGH: u32 = 1 << 19;

    /// Bits [18:18], mask 0x00040000
    pub const GPIO_QSPI_SD0_EDGE_LOW_OFFSET: u32 = 18;
    pub const GPIO_QSPI_SD0_EDGE_LOW_MASK: u32 = 0x00040000;
    pub const GPIO_QSPI_SD0_EDGE_LOW: u32 = 1 << 18;

    /// Bits [17:17], mask 0x00020000
    pub const GPIO_QSPI_SD0_LEVEL_HIGH_OFFSET: u32 = 17;
    pub const GPIO_QSPI_SD0_LEVEL_HIGH_MASK: u32 = 0x00020000;
    pub const GPIO_QSPI_SD0_LEVEL_HIGH: u32 = 1 << 17;

    /// Bits [16:16], mask 0x00010000
    pub const GPIO_QSPI_SD0_LEVEL_LOW_OFFSET: u32 = 16;
    pub const GPIO_QSPI_SD0_LEVEL_LOW_MASK: u32 = 0x00010000;
    pub const GPIO_QSPI_SD0_LEVEL_LOW: u32 = 1 << 16;

    /// Bits [15:15], mask 0x00008000
    pub const GPIO_QSPI_SS_EDGE_HIGH_OFFSET: u32 = 15;
    pub const GPIO_QSPI_SS_EDGE_HIGH_MASK: u32 = 0x00008000;
    pub const GPIO_QSPI_SS_EDGE_HIGH: u32 = 1 << 15;

    /// Bits [14:14], mask 0x00004000
    pub const GPIO_QSPI_SS_EDGE_LOW_OFFSET: u32 = 14;
    pub const GPIO_QSPI_SS_EDGE_LOW_MASK: u32 = 0x00004000;
    pub const GPIO_QSPI_SS_EDGE_LOW: u32 = 1 << 14;

    /// Bits [13:13], mask 0x00002000
    pub const GPIO_QSPI_SS_LEVEL_HIGH_OFFSET: u32 = 13;
    pub const GPIO_QSPI_SS_LEVEL_HIGH_MASK: u32 = 0x00002000;
    pub const GPIO_QSPI_SS_LEVEL_HIGH: u32 = 1 << 13;

    /// Bits [12:12], mask 0x00001000
    pub const GPIO_QSPI_SS_LEVEL_LOW_OFFSET: u32 = 12;
    pub const GPIO_QSPI_SS_LEVEL_LOW_MASK: u32 = 0x00001000;
    pub const GPIO_QSPI_SS_LEVEL_LOW: u32 = 1 << 12;

    /// Bits [11:11], mask 0x00000800
    pub const GPIO_QSPI_SCLK_EDGE_HIGH_OFFSET: u32 = 11;
    pub const GPIO_QSPI_SCLK_EDGE_HIGH_MASK: u32 = 0x00000800;
    pub const GPIO_QSPI_SCLK_EDGE_HIGH: u32 = 1 << 11;

    /// Bits [10:10], mask 0x00000400
    pub const GPIO_QSPI_SCLK_EDGE_LOW_OFFSET: u32 = 10;
    pub const GPIO_QSPI_SCLK_EDGE_LOW_MASK: u32 = 0x00000400;
    pub const GPIO_QSPI_SCLK_EDGE_LOW: u32 = 1 << 10;

    /// Bits [9:9], mask 0x00000200
    pub const GPIO_QSPI_SCLK_LEVEL_HIGH_OFFSET: u32 = 9;
    pub const GPIO_QSPI_SCLK_LEVEL_HIGH_MASK: u32 = 0x00000200;
    pub const GPIO_QSPI_SCLK_LEVEL_HIGH: u32 = 1 << 9;

    /// Bits [8:8], mask 0x00000100
    pub const GPIO_QSPI_SCLK_LEVEL_LOW_OFFSET: u32 = 8;
    pub const GPIO_QSPI_SCLK_LEVEL_LOW_MASK: u32 = 0x00000100;
    pub const GPIO_QSPI_SCLK_LEVEL_LOW: u32 = 1 << 8;

    /// Bits [7:7], mask 0x00000080
    pub const USBPHY_DM_EDGE_HIGH_OFFSET: u32 = 7;
    pub const USBPHY_DM_EDGE_HIGH_MASK: u32 = 0x00000080;
    pub const USBPHY_DM_EDGE_HIGH: u32 = 1 << 7;

    /// Bits [6:6], mask 0x00000040
    pub const USBPHY_DM_EDGE_LOW_OFFSET: u32 = 6;
    pub const USBPHY_DM_EDGE_LOW_MASK: u32 = 0x00000040;
    pub const USBPHY_DM_EDGE_LOW: u32 = 1 << 6;

    /// Bits [5:5], mask 0x00000020
    pub const USBPHY_DM_LEVEL_HIGH_OFFSET: u32 = 5;
    pub const USBPHY_DM_LEVEL_HIGH_MASK: u32 = 0x00000020;
    pub const USBPHY_DM_LEVEL_HIGH: u32 = 1 << 5;

    /// Bits [4:4], mask 0x00000010
    pub const USBPHY_DM_LEVEL_LOW_OFFSET: u32 = 4;
    pub const USBPHY_DM_LEVEL_LOW_MASK: u32 = 0x00000010;
    pub const USBPHY_DM_LEVEL_LOW: u32 = 1 << 4;

    /// Bits [3:3], mask 0x00000008
    pub const USBPHY_DP_EDGE_HIGH_OFFSET: u32 = 3;
    pub const USBPHY_DP_EDGE_HIGH_MASK: u32 = 0x00000008;
    pub const USBPHY_DP_EDGE_HIGH: u32 = 1 << 3;

    /// Bits [2:2], mask 0x00000004
    pub const USBPHY_DP_EDGE_LOW_OFFSET: u32 = 2;
    pub const USBPHY_DP_EDGE_LOW_MASK: u32 = 0x00000004;
    pub const USBPHY_DP_EDGE_LOW: u32 = 1 << 2;

    /// Bits [1:1], mask 0x00000002
    pub const USBPHY_DP_LEVEL_HIGH_OFFSET: u32 = 1;
    pub const USBPHY_DP_LEVEL_HIGH_MASK: u32 = 0x00000002;
    pub const USBPHY_DP_LEVEL_HIGH: u32 = 1 << 1;

    /// Bits [0:0], mask 0x00000001
    pub const USBPHY_DP_LEVEL_LOW_OFFSET: u32 = 0;
    pub const USBPHY_DP_LEVEL_LOW_MASK: u32 = 0x00000001;
    pub const USBPHY_DP_LEVEL_LOW: u32 = 1 << 0;

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

    /// **GPIO_QSPI_SD3_EDGE_HIGH** [31:31]
    #[inline(always)]
    pub fn gpio_qspi_sd3_edge_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD3_EDGE_HIGH_MASK) != 0
    }

    /// Set GPIO_QSPI_SD3_EDGE_HIGH bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd3_edge_high(&self) {
        self.set_bits(Self::GPIO_QSPI_SD3_EDGE_HIGH_MASK);
    }

    /// Clear GPIO_QSPI_SD3_EDGE_HIGH bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd3_edge_high(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD3_EDGE_HIGH_MASK);
    }

    /// **GPIO_QSPI_SD3_EDGE_LOW** [30:30]
    #[inline(always)]
    pub fn gpio_qspi_sd3_edge_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD3_EDGE_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SD3_EDGE_LOW bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd3_edge_low(&self) {
        self.set_bits(Self::GPIO_QSPI_SD3_EDGE_LOW_MASK);
    }

    /// Clear GPIO_QSPI_SD3_EDGE_LOW bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd3_edge_low(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD3_EDGE_LOW_MASK);
    }

    /// **GPIO_QSPI_SD3_LEVEL_HIGH** [29:29]
    #[inline(always)]
    pub fn gpio_qspi_sd3_level_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD3_LEVEL_HIGH_MASK) != 0
    }

    /// Set GPIO_QSPI_SD3_LEVEL_HIGH bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd3_level_high(&self) {
        self.set_bits(Self::GPIO_QSPI_SD3_LEVEL_HIGH_MASK);
    }

    /// Clear GPIO_QSPI_SD3_LEVEL_HIGH bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd3_level_high(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD3_LEVEL_HIGH_MASK);
    }

    /// **GPIO_QSPI_SD3_LEVEL_LOW** [28:28]
    #[inline(always)]
    pub fn gpio_qspi_sd3_level_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD3_LEVEL_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SD3_LEVEL_LOW bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd3_level_low(&self) {
        self.set_bits(Self::GPIO_QSPI_SD3_LEVEL_LOW_MASK);
    }

    /// Clear GPIO_QSPI_SD3_LEVEL_LOW bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd3_level_low(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD3_LEVEL_LOW_MASK);
    }

    /// **GPIO_QSPI_SD2_EDGE_HIGH** [27:27]
    #[inline(always)]
    pub fn gpio_qspi_sd2_edge_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD2_EDGE_HIGH_MASK) != 0
    }

    /// Set GPIO_QSPI_SD2_EDGE_HIGH bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd2_edge_high(&self) {
        self.set_bits(Self::GPIO_QSPI_SD2_EDGE_HIGH_MASK);
    }

    /// Clear GPIO_QSPI_SD2_EDGE_HIGH bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd2_edge_high(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD2_EDGE_HIGH_MASK);
    }

    /// **GPIO_QSPI_SD2_EDGE_LOW** [26:26]
    #[inline(always)]
    pub fn gpio_qspi_sd2_edge_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD2_EDGE_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SD2_EDGE_LOW bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd2_edge_low(&self) {
        self.set_bits(Self::GPIO_QSPI_SD2_EDGE_LOW_MASK);
    }

    /// Clear GPIO_QSPI_SD2_EDGE_LOW bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd2_edge_low(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD2_EDGE_LOW_MASK);
    }

    /// **GPIO_QSPI_SD2_LEVEL_HIGH** [25:25]
    #[inline(always)]
    pub fn gpio_qspi_sd2_level_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD2_LEVEL_HIGH_MASK) != 0
    }

    /// Set GPIO_QSPI_SD2_LEVEL_HIGH bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd2_level_high(&self) {
        self.set_bits(Self::GPIO_QSPI_SD2_LEVEL_HIGH_MASK);
    }

    /// Clear GPIO_QSPI_SD2_LEVEL_HIGH bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd2_level_high(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD2_LEVEL_HIGH_MASK);
    }

    /// **GPIO_QSPI_SD2_LEVEL_LOW** [24:24]
    #[inline(always)]
    pub fn gpio_qspi_sd2_level_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD2_LEVEL_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SD2_LEVEL_LOW bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd2_level_low(&self) {
        self.set_bits(Self::GPIO_QSPI_SD2_LEVEL_LOW_MASK);
    }

    /// Clear GPIO_QSPI_SD2_LEVEL_LOW bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd2_level_low(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD2_LEVEL_LOW_MASK);
    }

    /// **GPIO_QSPI_SD1_EDGE_HIGH** [23:23]
    #[inline(always)]
    pub fn gpio_qspi_sd1_edge_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD1_EDGE_HIGH_MASK) != 0
    }

    /// Set GPIO_QSPI_SD1_EDGE_HIGH bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd1_edge_high(&self) {
        self.set_bits(Self::GPIO_QSPI_SD1_EDGE_HIGH_MASK);
    }

    /// Clear GPIO_QSPI_SD1_EDGE_HIGH bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd1_edge_high(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD1_EDGE_HIGH_MASK);
    }

    /// **GPIO_QSPI_SD1_EDGE_LOW** [22:22]
    #[inline(always)]
    pub fn gpio_qspi_sd1_edge_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD1_EDGE_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SD1_EDGE_LOW bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd1_edge_low(&self) {
        self.set_bits(Self::GPIO_QSPI_SD1_EDGE_LOW_MASK);
    }

    /// Clear GPIO_QSPI_SD1_EDGE_LOW bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd1_edge_low(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD1_EDGE_LOW_MASK);
    }

    /// **GPIO_QSPI_SD1_LEVEL_HIGH** [21:21]
    #[inline(always)]
    pub fn gpio_qspi_sd1_level_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD1_LEVEL_HIGH_MASK) != 0
    }

    /// Set GPIO_QSPI_SD1_LEVEL_HIGH bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd1_level_high(&self) {
        self.set_bits(Self::GPIO_QSPI_SD1_LEVEL_HIGH_MASK);
    }

    /// Clear GPIO_QSPI_SD1_LEVEL_HIGH bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd1_level_high(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD1_LEVEL_HIGH_MASK);
    }

    /// **GPIO_QSPI_SD1_LEVEL_LOW** [20:20]
    #[inline(always)]
    pub fn gpio_qspi_sd1_level_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD1_LEVEL_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SD1_LEVEL_LOW bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd1_level_low(&self) {
        self.set_bits(Self::GPIO_QSPI_SD1_LEVEL_LOW_MASK);
    }

    /// Clear GPIO_QSPI_SD1_LEVEL_LOW bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd1_level_low(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD1_LEVEL_LOW_MASK);
    }

    /// **GPIO_QSPI_SD0_EDGE_HIGH** [19:19]
    #[inline(always)]
    pub fn gpio_qspi_sd0_edge_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD0_EDGE_HIGH_MASK) != 0
    }

    /// Set GPIO_QSPI_SD0_EDGE_HIGH bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd0_edge_high(&self) {
        self.set_bits(Self::GPIO_QSPI_SD0_EDGE_HIGH_MASK);
    }

    /// Clear GPIO_QSPI_SD0_EDGE_HIGH bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd0_edge_high(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD0_EDGE_HIGH_MASK);
    }

    /// **GPIO_QSPI_SD0_EDGE_LOW** [18:18]
    #[inline(always)]
    pub fn gpio_qspi_sd0_edge_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD0_EDGE_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SD0_EDGE_LOW bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd0_edge_low(&self) {
        self.set_bits(Self::GPIO_QSPI_SD0_EDGE_LOW_MASK);
    }

    /// Clear GPIO_QSPI_SD0_EDGE_LOW bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd0_edge_low(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD0_EDGE_LOW_MASK);
    }

    /// **GPIO_QSPI_SD0_LEVEL_HIGH** [17:17]
    #[inline(always)]
    pub fn gpio_qspi_sd0_level_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD0_LEVEL_HIGH_MASK) != 0
    }

    /// Set GPIO_QSPI_SD0_LEVEL_HIGH bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd0_level_high(&self) {
        self.set_bits(Self::GPIO_QSPI_SD0_LEVEL_HIGH_MASK);
    }

    /// Clear GPIO_QSPI_SD0_LEVEL_HIGH bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd0_level_high(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD0_LEVEL_HIGH_MASK);
    }

    /// **GPIO_QSPI_SD0_LEVEL_LOW** [16:16]
    #[inline(always)]
    pub fn gpio_qspi_sd0_level_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD0_LEVEL_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SD0_LEVEL_LOW bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd0_level_low(&self) {
        self.set_bits(Self::GPIO_QSPI_SD0_LEVEL_LOW_MASK);
    }

    /// Clear GPIO_QSPI_SD0_LEVEL_LOW bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd0_level_low(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD0_LEVEL_LOW_MASK);
    }

    /// **GPIO_QSPI_SS_EDGE_HIGH** [15:15]
    #[inline(always)]
    pub fn gpio_qspi_ss_edge_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SS_EDGE_HIGH_MASK) != 0
    }

    /// Set GPIO_QSPI_SS_EDGE_HIGH bit
    #[inline(always)]
    pub fn set_gpio_qspi_ss_edge_high(&self) {
        self.set_bits(Self::GPIO_QSPI_SS_EDGE_HIGH_MASK);
    }

    /// Clear GPIO_QSPI_SS_EDGE_HIGH bit
    #[inline(always)]
    pub fn clr_gpio_qspi_ss_edge_high(&self) {
        self.clr_bits(Self::GPIO_QSPI_SS_EDGE_HIGH_MASK);
    }

    /// **GPIO_QSPI_SS_EDGE_LOW** [14:14]
    #[inline(always)]
    pub fn gpio_qspi_ss_edge_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SS_EDGE_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SS_EDGE_LOW bit
    #[inline(always)]
    pub fn set_gpio_qspi_ss_edge_low(&self) {
        self.set_bits(Self::GPIO_QSPI_SS_EDGE_LOW_MASK);
    }

    /// Clear GPIO_QSPI_SS_EDGE_LOW bit
    #[inline(always)]
    pub fn clr_gpio_qspi_ss_edge_low(&self) {
        self.clr_bits(Self::GPIO_QSPI_SS_EDGE_LOW_MASK);
    }

    /// **GPIO_QSPI_SS_LEVEL_HIGH** [13:13]
    #[inline(always)]
    pub fn gpio_qspi_ss_level_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SS_LEVEL_HIGH_MASK) != 0
    }

    /// Set GPIO_QSPI_SS_LEVEL_HIGH bit
    #[inline(always)]
    pub fn set_gpio_qspi_ss_level_high(&self) {
        self.set_bits(Self::GPIO_QSPI_SS_LEVEL_HIGH_MASK);
    }

    /// Clear GPIO_QSPI_SS_LEVEL_HIGH bit
    #[inline(always)]
    pub fn clr_gpio_qspi_ss_level_high(&self) {
        self.clr_bits(Self::GPIO_QSPI_SS_LEVEL_HIGH_MASK);
    }

    /// **GPIO_QSPI_SS_LEVEL_LOW** [12:12]
    #[inline(always)]
    pub fn gpio_qspi_ss_level_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SS_LEVEL_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SS_LEVEL_LOW bit
    #[inline(always)]
    pub fn set_gpio_qspi_ss_level_low(&self) {
        self.set_bits(Self::GPIO_QSPI_SS_LEVEL_LOW_MASK);
    }

    /// Clear GPIO_QSPI_SS_LEVEL_LOW bit
    #[inline(always)]
    pub fn clr_gpio_qspi_ss_level_low(&self) {
        self.clr_bits(Self::GPIO_QSPI_SS_LEVEL_LOW_MASK);
    }

    /// **GPIO_QSPI_SCLK_EDGE_HIGH** [11:11]
    #[inline(always)]
    pub fn gpio_qspi_sclk_edge_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SCLK_EDGE_HIGH_MASK) != 0
    }

    /// Set GPIO_QSPI_SCLK_EDGE_HIGH bit
    #[inline(always)]
    pub fn set_gpio_qspi_sclk_edge_high(&self) {
        self.set_bits(Self::GPIO_QSPI_SCLK_EDGE_HIGH_MASK);
    }

    /// Clear GPIO_QSPI_SCLK_EDGE_HIGH bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sclk_edge_high(&self) {
        self.clr_bits(Self::GPIO_QSPI_SCLK_EDGE_HIGH_MASK);
    }

    /// **GPIO_QSPI_SCLK_EDGE_LOW** [10:10]
    #[inline(always)]
    pub fn gpio_qspi_sclk_edge_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SCLK_EDGE_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SCLK_EDGE_LOW bit
    #[inline(always)]
    pub fn set_gpio_qspi_sclk_edge_low(&self) {
        self.set_bits(Self::GPIO_QSPI_SCLK_EDGE_LOW_MASK);
    }

    /// Clear GPIO_QSPI_SCLK_EDGE_LOW bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sclk_edge_low(&self) {
        self.clr_bits(Self::GPIO_QSPI_SCLK_EDGE_LOW_MASK);
    }

    /// **GPIO_QSPI_SCLK_LEVEL_HIGH** [9:9]
    #[inline(always)]
    pub fn gpio_qspi_sclk_level_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SCLK_LEVEL_HIGH_MASK) != 0
    }

    /// Set GPIO_QSPI_SCLK_LEVEL_HIGH bit
    #[inline(always)]
    pub fn set_gpio_qspi_sclk_level_high(&self) {
        self.set_bits(Self::GPIO_QSPI_SCLK_LEVEL_HIGH_MASK);
    }

    /// Clear GPIO_QSPI_SCLK_LEVEL_HIGH bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sclk_level_high(&self) {
        self.clr_bits(Self::GPIO_QSPI_SCLK_LEVEL_HIGH_MASK);
    }

    /// **GPIO_QSPI_SCLK_LEVEL_LOW** [8:8]
    #[inline(always)]
    pub fn gpio_qspi_sclk_level_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SCLK_LEVEL_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SCLK_LEVEL_LOW bit
    #[inline(always)]
    pub fn set_gpio_qspi_sclk_level_low(&self) {
        self.set_bits(Self::GPIO_QSPI_SCLK_LEVEL_LOW_MASK);
    }

    /// Clear GPIO_QSPI_SCLK_LEVEL_LOW bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sclk_level_low(&self) {
        self.clr_bits(Self::GPIO_QSPI_SCLK_LEVEL_LOW_MASK);
    }

    /// **USBPHY_DM_EDGE_HIGH** [7:7]
    #[inline(always)]
    pub fn usbphy_dm_edge_high(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DM_EDGE_HIGH_MASK) != 0
    }

    /// Set USBPHY_DM_EDGE_HIGH bit
    #[inline(always)]
    pub fn set_usbphy_dm_edge_high(&self) {
        self.set_bits(Self::USBPHY_DM_EDGE_HIGH_MASK);
    }

    /// Clear USBPHY_DM_EDGE_HIGH bit
    #[inline(always)]
    pub fn clr_usbphy_dm_edge_high(&self) {
        self.clr_bits(Self::USBPHY_DM_EDGE_HIGH_MASK);
    }

    /// **USBPHY_DM_EDGE_LOW** [6:6]
    #[inline(always)]
    pub fn usbphy_dm_edge_low(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DM_EDGE_LOW_MASK) != 0
    }

    /// Set USBPHY_DM_EDGE_LOW bit
    #[inline(always)]
    pub fn set_usbphy_dm_edge_low(&self) {
        self.set_bits(Self::USBPHY_DM_EDGE_LOW_MASK);
    }

    /// Clear USBPHY_DM_EDGE_LOW bit
    #[inline(always)]
    pub fn clr_usbphy_dm_edge_low(&self) {
        self.clr_bits(Self::USBPHY_DM_EDGE_LOW_MASK);
    }

    /// **USBPHY_DM_LEVEL_HIGH** [5:5]
    #[inline(always)]
    pub fn usbphy_dm_level_high(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DM_LEVEL_HIGH_MASK) != 0
    }

    /// Set USBPHY_DM_LEVEL_HIGH bit
    #[inline(always)]
    pub fn set_usbphy_dm_level_high(&self) {
        self.set_bits(Self::USBPHY_DM_LEVEL_HIGH_MASK);
    }

    /// Clear USBPHY_DM_LEVEL_HIGH bit
    #[inline(always)]
    pub fn clr_usbphy_dm_level_high(&self) {
        self.clr_bits(Self::USBPHY_DM_LEVEL_HIGH_MASK);
    }

    /// **USBPHY_DM_LEVEL_LOW** [4:4]
    #[inline(always)]
    pub fn usbphy_dm_level_low(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DM_LEVEL_LOW_MASK) != 0
    }

    /// Set USBPHY_DM_LEVEL_LOW bit
    #[inline(always)]
    pub fn set_usbphy_dm_level_low(&self) {
        self.set_bits(Self::USBPHY_DM_LEVEL_LOW_MASK);
    }

    /// Clear USBPHY_DM_LEVEL_LOW bit
    #[inline(always)]
    pub fn clr_usbphy_dm_level_low(&self) {
        self.clr_bits(Self::USBPHY_DM_LEVEL_LOW_MASK);
    }

    /// **USBPHY_DP_EDGE_HIGH** [3:3]
    #[inline(always)]
    pub fn usbphy_dp_edge_high(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DP_EDGE_HIGH_MASK) != 0
    }

    /// Set USBPHY_DP_EDGE_HIGH bit
    #[inline(always)]
    pub fn set_usbphy_dp_edge_high(&self) {
        self.set_bits(Self::USBPHY_DP_EDGE_HIGH_MASK);
    }

    /// Clear USBPHY_DP_EDGE_HIGH bit
    #[inline(always)]
    pub fn clr_usbphy_dp_edge_high(&self) {
        self.clr_bits(Self::USBPHY_DP_EDGE_HIGH_MASK);
    }

    /// **USBPHY_DP_EDGE_LOW** [2:2]
    #[inline(always)]
    pub fn usbphy_dp_edge_low(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DP_EDGE_LOW_MASK) != 0
    }

    /// Set USBPHY_DP_EDGE_LOW bit
    #[inline(always)]
    pub fn set_usbphy_dp_edge_low(&self) {
        self.set_bits(Self::USBPHY_DP_EDGE_LOW_MASK);
    }

    /// Clear USBPHY_DP_EDGE_LOW bit
    #[inline(always)]
    pub fn clr_usbphy_dp_edge_low(&self) {
        self.clr_bits(Self::USBPHY_DP_EDGE_LOW_MASK);
    }

    /// **USBPHY_DP_LEVEL_HIGH** [1:1]
    #[inline(always)]
    pub fn usbphy_dp_level_high(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DP_LEVEL_HIGH_MASK) != 0
    }

    /// Set USBPHY_DP_LEVEL_HIGH bit
    #[inline(always)]
    pub fn set_usbphy_dp_level_high(&self) {
        self.set_bits(Self::USBPHY_DP_LEVEL_HIGH_MASK);
    }

    /// Clear USBPHY_DP_LEVEL_HIGH bit
    #[inline(always)]
    pub fn clr_usbphy_dp_level_high(&self) {
        self.clr_bits(Self::USBPHY_DP_LEVEL_HIGH_MASK);
    }

    /// **USBPHY_DP_LEVEL_LOW** [0:0]
    #[inline(always)]
    pub fn usbphy_dp_level_low(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DP_LEVEL_LOW_MASK) != 0
    }

    /// Set USBPHY_DP_LEVEL_LOW bit
    #[inline(always)]
    pub fn set_usbphy_dp_level_low(&self) {
        self.set_bits(Self::USBPHY_DP_LEVEL_LOW_MASK);
    }

    /// Clear USBPHY_DP_LEVEL_LOW bit
    #[inline(always)]
    pub fn clr_usbphy_dp_level_low(&self) {
        self.clr_bits(Self::USBPHY_DP_LEVEL_LOW_MASK);
    }

}

/// PROC0_INTF value type for batch read/modify/write operations
///
/// Use `proc0intf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `proc0intf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Proc0IntfVal {
    bits: u32,
}

impl Proc0IntfVal {
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

    /// Read GPIO_QSPI_SD3_EDGE_HIGH [31:31]
    #[inline(always)]
    pub const fn gpio_qspi_sd3_edge_high(&self) -> bool {
        (self.bits & Proc0Intf::GPIO_QSPI_SD3_EDGE_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD3_EDGE_LOW [30:30]
    #[inline(always)]
    pub const fn gpio_qspi_sd3_edge_low(&self) -> bool {
        (self.bits & Proc0Intf::GPIO_QSPI_SD3_EDGE_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SD3_LEVEL_HIGH [29:29]
    #[inline(always)]
    pub const fn gpio_qspi_sd3_level_high(&self) -> bool {
        (self.bits & Proc0Intf::GPIO_QSPI_SD3_LEVEL_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD3_LEVEL_LOW [28:28]
    #[inline(always)]
    pub const fn gpio_qspi_sd3_level_low(&self) -> bool {
        (self.bits & Proc0Intf::GPIO_QSPI_SD3_LEVEL_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SD2_EDGE_HIGH [27:27]
    #[inline(always)]
    pub const fn gpio_qspi_sd2_edge_high(&self) -> bool {
        (self.bits & Proc0Intf::GPIO_QSPI_SD2_EDGE_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD2_EDGE_LOW [26:26]
    #[inline(always)]
    pub const fn gpio_qspi_sd2_edge_low(&self) -> bool {
        (self.bits & Proc0Intf::GPIO_QSPI_SD2_EDGE_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SD2_LEVEL_HIGH [25:25]
    #[inline(always)]
    pub const fn gpio_qspi_sd2_level_high(&self) -> bool {
        (self.bits & Proc0Intf::GPIO_QSPI_SD2_LEVEL_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD2_LEVEL_LOW [24:24]
    #[inline(always)]
    pub const fn gpio_qspi_sd2_level_low(&self) -> bool {
        (self.bits & Proc0Intf::GPIO_QSPI_SD2_LEVEL_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SD1_EDGE_HIGH [23:23]
    #[inline(always)]
    pub const fn gpio_qspi_sd1_edge_high(&self) -> bool {
        (self.bits & Proc0Intf::GPIO_QSPI_SD1_EDGE_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD1_EDGE_LOW [22:22]
    #[inline(always)]
    pub const fn gpio_qspi_sd1_edge_low(&self) -> bool {
        (self.bits & Proc0Intf::GPIO_QSPI_SD1_EDGE_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SD1_LEVEL_HIGH [21:21]
    #[inline(always)]
    pub const fn gpio_qspi_sd1_level_high(&self) -> bool {
        (self.bits & Proc0Intf::GPIO_QSPI_SD1_LEVEL_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD1_LEVEL_LOW [20:20]
    #[inline(always)]
    pub const fn gpio_qspi_sd1_level_low(&self) -> bool {
        (self.bits & Proc0Intf::GPIO_QSPI_SD1_LEVEL_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SD0_EDGE_HIGH [19:19]
    #[inline(always)]
    pub const fn gpio_qspi_sd0_edge_high(&self) -> bool {
        (self.bits & Proc0Intf::GPIO_QSPI_SD0_EDGE_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD0_EDGE_LOW [18:18]
    #[inline(always)]
    pub const fn gpio_qspi_sd0_edge_low(&self) -> bool {
        (self.bits & Proc0Intf::GPIO_QSPI_SD0_EDGE_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SD0_LEVEL_HIGH [17:17]
    #[inline(always)]
    pub const fn gpio_qspi_sd0_level_high(&self) -> bool {
        (self.bits & Proc0Intf::GPIO_QSPI_SD0_LEVEL_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD0_LEVEL_LOW [16:16]
    #[inline(always)]
    pub const fn gpio_qspi_sd0_level_low(&self) -> bool {
        (self.bits & Proc0Intf::GPIO_QSPI_SD0_LEVEL_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SS_EDGE_HIGH [15:15]
    #[inline(always)]
    pub const fn gpio_qspi_ss_edge_high(&self) -> bool {
        (self.bits & Proc0Intf::GPIO_QSPI_SS_EDGE_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SS_EDGE_LOW [14:14]
    #[inline(always)]
    pub const fn gpio_qspi_ss_edge_low(&self) -> bool {
        (self.bits & Proc0Intf::GPIO_QSPI_SS_EDGE_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SS_LEVEL_HIGH [13:13]
    #[inline(always)]
    pub const fn gpio_qspi_ss_level_high(&self) -> bool {
        (self.bits & Proc0Intf::GPIO_QSPI_SS_LEVEL_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SS_LEVEL_LOW [12:12]
    #[inline(always)]
    pub const fn gpio_qspi_ss_level_low(&self) -> bool {
        (self.bits & Proc0Intf::GPIO_QSPI_SS_LEVEL_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SCLK_EDGE_HIGH [11:11]
    #[inline(always)]
    pub const fn gpio_qspi_sclk_edge_high(&self) -> bool {
        (self.bits & Proc0Intf::GPIO_QSPI_SCLK_EDGE_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SCLK_EDGE_LOW [10:10]
    #[inline(always)]
    pub const fn gpio_qspi_sclk_edge_low(&self) -> bool {
        (self.bits & Proc0Intf::GPIO_QSPI_SCLK_EDGE_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SCLK_LEVEL_HIGH [9:9]
    #[inline(always)]
    pub const fn gpio_qspi_sclk_level_high(&self) -> bool {
        (self.bits & Proc0Intf::GPIO_QSPI_SCLK_LEVEL_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SCLK_LEVEL_LOW [8:8]
    #[inline(always)]
    pub const fn gpio_qspi_sclk_level_low(&self) -> bool {
        (self.bits & Proc0Intf::GPIO_QSPI_SCLK_LEVEL_LOW_MASK) != 0
    }

    /// Read USBPHY_DM_EDGE_HIGH [7:7]
    #[inline(always)]
    pub const fn usbphy_dm_edge_high(&self) -> bool {
        (self.bits & Proc0Intf::USBPHY_DM_EDGE_HIGH_MASK) != 0
    }

    /// Read USBPHY_DM_EDGE_LOW [6:6]
    #[inline(always)]
    pub const fn usbphy_dm_edge_low(&self) -> bool {
        (self.bits & Proc0Intf::USBPHY_DM_EDGE_LOW_MASK) != 0
    }

    /// Read USBPHY_DM_LEVEL_HIGH [5:5]
    #[inline(always)]
    pub const fn usbphy_dm_level_high(&self) -> bool {
        (self.bits & Proc0Intf::USBPHY_DM_LEVEL_HIGH_MASK) != 0
    }

    /// Read USBPHY_DM_LEVEL_LOW [4:4]
    #[inline(always)]
    pub const fn usbphy_dm_level_low(&self) -> bool {
        (self.bits & Proc0Intf::USBPHY_DM_LEVEL_LOW_MASK) != 0
    }

    /// Read USBPHY_DP_EDGE_HIGH [3:3]
    #[inline(always)]
    pub const fn usbphy_dp_edge_high(&self) -> bool {
        (self.bits & Proc0Intf::USBPHY_DP_EDGE_HIGH_MASK) != 0
    }

    /// Read USBPHY_DP_EDGE_LOW [2:2]
    #[inline(always)]
    pub const fn usbphy_dp_edge_low(&self) -> bool {
        (self.bits & Proc0Intf::USBPHY_DP_EDGE_LOW_MASK) != 0
    }

    /// Read USBPHY_DP_LEVEL_HIGH [1:1]
    #[inline(always)]
    pub const fn usbphy_dp_level_high(&self) -> bool {
        (self.bits & Proc0Intf::USBPHY_DP_LEVEL_HIGH_MASK) != 0
    }

    /// Read USBPHY_DP_LEVEL_LOW [0:0]
    #[inline(always)]
    pub const fn usbphy_dp_level_low(&self) -> bool {
        (self.bits & Proc0Intf::USBPHY_DP_LEVEL_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SD3_EDGE_HIGH [31:31]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd3_edge_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc0Intf::GPIO_QSPI_SD3_EDGE_HIGH_MASK
            } else {
                self.bits & !Proc0Intf::GPIO_QSPI_SD3_EDGE_HIGH_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD3_EDGE_LOW [30:30]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd3_edge_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc0Intf::GPIO_QSPI_SD3_EDGE_LOW_MASK
            } else {
                self.bits & !Proc0Intf::GPIO_QSPI_SD3_EDGE_LOW_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD3_LEVEL_HIGH [29:29]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd3_level_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc0Intf::GPIO_QSPI_SD3_LEVEL_HIGH_MASK
            } else {
                self.bits & !Proc0Intf::GPIO_QSPI_SD3_LEVEL_HIGH_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD3_LEVEL_LOW [28:28]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd3_level_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc0Intf::GPIO_QSPI_SD3_LEVEL_LOW_MASK
            } else {
                self.bits & !Proc0Intf::GPIO_QSPI_SD3_LEVEL_LOW_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD2_EDGE_HIGH [27:27]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd2_edge_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc0Intf::GPIO_QSPI_SD2_EDGE_HIGH_MASK
            } else {
                self.bits & !Proc0Intf::GPIO_QSPI_SD2_EDGE_HIGH_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD2_EDGE_LOW [26:26]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd2_edge_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc0Intf::GPIO_QSPI_SD2_EDGE_LOW_MASK
            } else {
                self.bits & !Proc0Intf::GPIO_QSPI_SD2_EDGE_LOW_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD2_LEVEL_HIGH [25:25]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd2_level_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc0Intf::GPIO_QSPI_SD2_LEVEL_HIGH_MASK
            } else {
                self.bits & !Proc0Intf::GPIO_QSPI_SD2_LEVEL_HIGH_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD2_LEVEL_LOW [24:24]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd2_level_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc0Intf::GPIO_QSPI_SD2_LEVEL_LOW_MASK
            } else {
                self.bits & !Proc0Intf::GPIO_QSPI_SD2_LEVEL_LOW_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD1_EDGE_HIGH [23:23]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd1_edge_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc0Intf::GPIO_QSPI_SD1_EDGE_HIGH_MASK
            } else {
                self.bits & !Proc0Intf::GPIO_QSPI_SD1_EDGE_HIGH_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD1_EDGE_LOW [22:22]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd1_edge_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc0Intf::GPIO_QSPI_SD1_EDGE_LOW_MASK
            } else {
                self.bits & !Proc0Intf::GPIO_QSPI_SD1_EDGE_LOW_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD1_LEVEL_HIGH [21:21]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd1_level_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc0Intf::GPIO_QSPI_SD1_LEVEL_HIGH_MASK
            } else {
                self.bits & !Proc0Intf::GPIO_QSPI_SD1_LEVEL_HIGH_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD1_LEVEL_LOW [20:20]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd1_level_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc0Intf::GPIO_QSPI_SD1_LEVEL_LOW_MASK
            } else {
                self.bits & !Proc0Intf::GPIO_QSPI_SD1_LEVEL_LOW_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD0_EDGE_HIGH [19:19]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd0_edge_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc0Intf::GPIO_QSPI_SD0_EDGE_HIGH_MASK
            } else {
                self.bits & !Proc0Intf::GPIO_QSPI_SD0_EDGE_HIGH_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD0_EDGE_LOW [18:18]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd0_edge_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc0Intf::GPIO_QSPI_SD0_EDGE_LOW_MASK
            } else {
                self.bits & !Proc0Intf::GPIO_QSPI_SD0_EDGE_LOW_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD0_LEVEL_HIGH [17:17]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd0_level_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc0Intf::GPIO_QSPI_SD0_LEVEL_HIGH_MASK
            } else {
                self.bits & !Proc0Intf::GPIO_QSPI_SD0_LEVEL_HIGH_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD0_LEVEL_LOW [16:16]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd0_level_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc0Intf::GPIO_QSPI_SD0_LEVEL_LOW_MASK
            } else {
                self.bits & !Proc0Intf::GPIO_QSPI_SD0_LEVEL_LOW_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SS_EDGE_HIGH [15:15]
    #[inline(always)]
    pub const fn with_gpio_qspi_ss_edge_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc0Intf::GPIO_QSPI_SS_EDGE_HIGH_MASK
            } else {
                self.bits & !Proc0Intf::GPIO_QSPI_SS_EDGE_HIGH_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SS_EDGE_LOW [14:14]
    #[inline(always)]
    pub const fn with_gpio_qspi_ss_edge_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc0Intf::GPIO_QSPI_SS_EDGE_LOW_MASK
            } else {
                self.bits & !Proc0Intf::GPIO_QSPI_SS_EDGE_LOW_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SS_LEVEL_HIGH [13:13]
    #[inline(always)]
    pub const fn with_gpio_qspi_ss_level_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc0Intf::GPIO_QSPI_SS_LEVEL_HIGH_MASK
            } else {
                self.bits & !Proc0Intf::GPIO_QSPI_SS_LEVEL_HIGH_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SS_LEVEL_LOW [12:12]
    #[inline(always)]
    pub const fn with_gpio_qspi_ss_level_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc0Intf::GPIO_QSPI_SS_LEVEL_LOW_MASK
            } else {
                self.bits & !Proc0Intf::GPIO_QSPI_SS_LEVEL_LOW_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SCLK_EDGE_HIGH [11:11]
    #[inline(always)]
    pub const fn with_gpio_qspi_sclk_edge_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc0Intf::GPIO_QSPI_SCLK_EDGE_HIGH_MASK
            } else {
                self.bits & !Proc0Intf::GPIO_QSPI_SCLK_EDGE_HIGH_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SCLK_EDGE_LOW [10:10]
    #[inline(always)]
    pub const fn with_gpio_qspi_sclk_edge_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc0Intf::GPIO_QSPI_SCLK_EDGE_LOW_MASK
            } else {
                self.bits & !Proc0Intf::GPIO_QSPI_SCLK_EDGE_LOW_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SCLK_LEVEL_HIGH [9:9]
    #[inline(always)]
    pub const fn with_gpio_qspi_sclk_level_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc0Intf::GPIO_QSPI_SCLK_LEVEL_HIGH_MASK
            } else {
                self.bits & !Proc0Intf::GPIO_QSPI_SCLK_LEVEL_HIGH_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SCLK_LEVEL_LOW [8:8]
    #[inline(always)]
    pub const fn with_gpio_qspi_sclk_level_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc0Intf::GPIO_QSPI_SCLK_LEVEL_LOW_MASK
            } else {
                self.bits & !Proc0Intf::GPIO_QSPI_SCLK_LEVEL_LOW_MASK
            }
        }
    }

    /// Set USBPHY_DM_EDGE_HIGH [7:7]
    #[inline(always)]
    pub const fn with_usbphy_dm_edge_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc0Intf::USBPHY_DM_EDGE_HIGH_MASK
            } else {
                self.bits & !Proc0Intf::USBPHY_DM_EDGE_HIGH_MASK
            }
        }
    }

    /// Set USBPHY_DM_EDGE_LOW [6:6]
    #[inline(always)]
    pub const fn with_usbphy_dm_edge_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc0Intf::USBPHY_DM_EDGE_LOW_MASK
            } else {
                self.bits & !Proc0Intf::USBPHY_DM_EDGE_LOW_MASK
            }
        }
    }

    /// Set USBPHY_DM_LEVEL_HIGH [5:5]
    #[inline(always)]
    pub const fn with_usbphy_dm_level_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc0Intf::USBPHY_DM_LEVEL_HIGH_MASK
            } else {
                self.bits & !Proc0Intf::USBPHY_DM_LEVEL_HIGH_MASK
            }
        }
    }

    /// Set USBPHY_DM_LEVEL_LOW [4:4]
    #[inline(always)]
    pub const fn with_usbphy_dm_level_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc0Intf::USBPHY_DM_LEVEL_LOW_MASK
            } else {
                self.bits & !Proc0Intf::USBPHY_DM_LEVEL_LOW_MASK
            }
        }
    }

    /// Set USBPHY_DP_EDGE_HIGH [3:3]
    #[inline(always)]
    pub const fn with_usbphy_dp_edge_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc0Intf::USBPHY_DP_EDGE_HIGH_MASK
            } else {
                self.bits & !Proc0Intf::USBPHY_DP_EDGE_HIGH_MASK
            }
        }
    }

    /// Set USBPHY_DP_EDGE_LOW [2:2]
    #[inline(always)]
    pub const fn with_usbphy_dp_edge_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc0Intf::USBPHY_DP_EDGE_LOW_MASK
            } else {
                self.bits & !Proc0Intf::USBPHY_DP_EDGE_LOW_MASK
            }
        }
    }

    /// Set USBPHY_DP_LEVEL_HIGH [1:1]
    #[inline(always)]
    pub const fn with_usbphy_dp_level_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc0Intf::USBPHY_DP_LEVEL_HIGH_MASK
            } else {
                self.bits & !Proc0Intf::USBPHY_DP_LEVEL_HIGH_MASK
            }
        }
    }

    /// Set USBPHY_DP_LEVEL_LOW [0:0]
    #[inline(always)]
    pub const fn with_usbphy_dp_level_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc0Intf::USBPHY_DP_LEVEL_LOW_MASK
            } else {
                self.bits & !Proc0Intf::USBPHY_DP_LEVEL_LOW_MASK
            }
        }
    }

}

impl Default for Proc0IntfVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Proc0Intf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> Proc0IntfVal {
        Proc0IntfVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: Proc0IntfVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(Proc0IntfVal) -> Proc0IntfVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// PROC0_INTS Register
///
/// Interrupt status after masking & forcing for proc0
///
/// Address offset: 0x0224
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **GPIO_QSPI_SD3_EDGE_HIGH** [31] (RO): 
/// - **GPIO_QSPI_SD3_EDGE_LOW** [30] (RO): 
/// - **GPIO_QSPI_SD3_LEVEL_HIGH** [29] (RO): 
/// - **GPIO_QSPI_SD3_LEVEL_LOW** [28] (RO): 
/// - **GPIO_QSPI_SD2_EDGE_HIGH** [27] (RO): 
/// - **GPIO_QSPI_SD2_EDGE_LOW** [26] (RO): 
/// - **GPIO_QSPI_SD2_LEVEL_HIGH** [25] (RO): 
/// - **GPIO_QSPI_SD2_LEVEL_LOW** [24] (RO): 
/// - **GPIO_QSPI_SD1_EDGE_HIGH** [23] (RO): 
/// - **GPIO_QSPI_SD1_EDGE_LOW** [22] (RO): 
/// - **GPIO_QSPI_SD1_LEVEL_HIGH** [21] (RO): 
/// - **GPIO_QSPI_SD1_LEVEL_LOW** [20] (RO): 
/// - **GPIO_QSPI_SD0_EDGE_HIGH** [19] (RO): 
/// - **GPIO_QSPI_SD0_EDGE_LOW** [18] (RO): 
/// - **GPIO_QSPI_SD0_LEVEL_HIGH** [17] (RO): 
/// - **GPIO_QSPI_SD0_LEVEL_LOW** [16] (RO): 
/// - **GPIO_QSPI_SS_EDGE_HIGH** [15] (RO): 
/// - **GPIO_QSPI_SS_EDGE_LOW** [14] (RO): 
/// - **GPIO_QSPI_SS_LEVEL_HIGH** [13] (RO): 
/// - **GPIO_QSPI_SS_LEVEL_LOW** [12] (RO): 
/// - **GPIO_QSPI_SCLK_EDGE_HIGH** [11] (RO): 
/// - **GPIO_QSPI_SCLK_EDGE_LOW** [10] (RO): 
/// - **GPIO_QSPI_SCLK_LEVEL_HIGH** [9] (RO): 
/// - **GPIO_QSPI_SCLK_LEVEL_LOW** [8] (RO): 
/// - **USBPHY_DM_EDGE_HIGH** [7] (RO): 
/// - **USBPHY_DM_EDGE_LOW** [6] (RO): 
/// - **USBPHY_DM_LEVEL_HIGH** [5] (RO): 
/// - **USBPHY_DM_LEVEL_LOW** [4] (RO): 
/// - **USBPHY_DP_EDGE_HIGH** [3] (RO): 
/// - **USBPHY_DP_EDGE_LOW** [2] (RO): 
/// - **USBPHY_DP_LEVEL_HIGH** [1] (RO): 
/// - **USBPHY_DP_LEVEL_LOW** [0] (RO): 
#[repr(transparent)]
pub struct Proc0Ints {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for Proc0Ints {}
unsafe impl Sync for Proc0Ints {}

impl Proc0Ints {
    // ======== Field Constants ========

    /// Bits [31:31], mask 0x80000000
    pub const GPIO_QSPI_SD3_EDGE_HIGH_OFFSET: u32 = 31;
    pub const GPIO_QSPI_SD3_EDGE_HIGH_MASK: u32 = 0x80000000;
    pub const GPIO_QSPI_SD3_EDGE_HIGH: u32 = 1 << 31;

    /// Bits [30:30], mask 0x40000000
    pub const GPIO_QSPI_SD3_EDGE_LOW_OFFSET: u32 = 30;
    pub const GPIO_QSPI_SD3_EDGE_LOW_MASK: u32 = 0x40000000;
    pub const GPIO_QSPI_SD3_EDGE_LOW: u32 = 1 << 30;

    /// Bits [29:29], mask 0x20000000
    pub const GPIO_QSPI_SD3_LEVEL_HIGH_OFFSET: u32 = 29;
    pub const GPIO_QSPI_SD3_LEVEL_HIGH_MASK: u32 = 0x20000000;
    pub const GPIO_QSPI_SD3_LEVEL_HIGH: u32 = 1 << 29;

    /// Bits [28:28], mask 0x10000000
    pub const GPIO_QSPI_SD3_LEVEL_LOW_OFFSET: u32 = 28;
    pub const GPIO_QSPI_SD3_LEVEL_LOW_MASK: u32 = 0x10000000;
    pub const GPIO_QSPI_SD3_LEVEL_LOW: u32 = 1 << 28;

    /// Bits [27:27], mask 0x08000000
    pub const GPIO_QSPI_SD2_EDGE_HIGH_OFFSET: u32 = 27;
    pub const GPIO_QSPI_SD2_EDGE_HIGH_MASK: u32 = 0x08000000;
    pub const GPIO_QSPI_SD2_EDGE_HIGH: u32 = 1 << 27;

    /// Bits [26:26], mask 0x04000000
    pub const GPIO_QSPI_SD2_EDGE_LOW_OFFSET: u32 = 26;
    pub const GPIO_QSPI_SD2_EDGE_LOW_MASK: u32 = 0x04000000;
    pub const GPIO_QSPI_SD2_EDGE_LOW: u32 = 1 << 26;

    /// Bits [25:25], mask 0x02000000
    pub const GPIO_QSPI_SD2_LEVEL_HIGH_OFFSET: u32 = 25;
    pub const GPIO_QSPI_SD2_LEVEL_HIGH_MASK: u32 = 0x02000000;
    pub const GPIO_QSPI_SD2_LEVEL_HIGH: u32 = 1 << 25;

    /// Bits [24:24], mask 0x01000000
    pub const GPIO_QSPI_SD2_LEVEL_LOW_OFFSET: u32 = 24;
    pub const GPIO_QSPI_SD2_LEVEL_LOW_MASK: u32 = 0x01000000;
    pub const GPIO_QSPI_SD2_LEVEL_LOW: u32 = 1 << 24;

    /// Bits [23:23], mask 0x00800000
    pub const GPIO_QSPI_SD1_EDGE_HIGH_OFFSET: u32 = 23;
    pub const GPIO_QSPI_SD1_EDGE_HIGH_MASK: u32 = 0x00800000;
    pub const GPIO_QSPI_SD1_EDGE_HIGH: u32 = 1 << 23;

    /// Bits [22:22], mask 0x00400000
    pub const GPIO_QSPI_SD1_EDGE_LOW_OFFSET: u32 = 22;
    pub const GPIO_QSPI_SD1_EDGE_LOW_MASK: u32 = 0x00400000;
    pub const GPIO_QSPI_SD1_EDGE_LOW: u32 = 1 << 22;

    /// Bits [21:21], mask 0x00200000
    pub const GPIO_QSPI_SD1_LEVEL_HIGH_OFFSET: u32 = 21;
    pub const GPIO_QSPI_SD1_LEVEL_HIGH_MASK: u32 = 0x00200000;
    pub const GPIO_QSPI_SD1_LEVEL_HIGH: u32 = 1 << 21;

    /// Bits [20:20], mask 0x00100000
    pub const GPIO_QSPI_SD1_LEVEL_LOW_OFFSET: u32 = 20;
    pub const GPIO_QSPI_SD1_LEVEL_LOW_MASK: u32 = 0x00100000;
    pub const GPIO_QSPI_SD1_LEVEL_LOW: u32 = 1 << 20;

    /// Bits [19:19], mask 0x00080000
    pub const GPIO_QSPI_SD0_EDGE_HIGH_OFFSET: u32 = 19;
    pub const GPIO_QSPI_SD0_EDGE_HIGH_MASK: u32 = 0x00080000;
    pub const GPIO_QSPI_SD0_EDGE_HIGH: u32 = 1 << 19;

    /// Bits [18:18], mask 0x00040000
    pub const GPIO_QSPI_SD0_EDGE_LOW_OFFSET: u32 = 18;
    pub const GPIO_QSPI_SD0_EDGE_LOW_MASK: u32 = 0x00040000;
    pub const GPIO_QSPI_SD0_EDGE_LOW: u32 = 1 << 18;

    /// Bits [17:17], mask 0x00020000
    pub const GPIO_QSPI_SD0_LEVEL_HIGH_OFFSET: u32 = 17;
    pub const GPIO_QSPI_SD0_LEVEL_HIGH_MASK: u32 = 0x00020000;
    pub const GPIO_QSPI_SD0_LEVEL_HIGH: u32 = 1 << 17;

    /// Bits [16:16], mask 0x00010000
    pub const GPIO_QSPI_SD0_LEVEL_LOW_OFFSET: u32 = 16;
    pub const GPIO_QSPI_SD0_LEVEL_LOW_MASK: u32 = 0x00010000;
    pub const GPIO_QSPI_SD0_LEVEL_LOW: u32 = 1 << 16;

    /// Bits [15:15], mask 0x00008000
    pub const GPIO_QSPI_SS_EDGE_HIGH_OFFSET: u32 = 15;
    pub const GPIO_QSPI_SS_EDGE_HIGH_MASK: u32 = 0x00008000;
    pub const GPIO_QSPI_SS_EDGE_HIGH: u32 = 1 << 15;

    /// Bits [14:14], mask 0x00004000
    pub const GPIO_QSPI_SS_EDGE_LOW_OFFSET: u32 = 14;
    pub const GPIO_QSPI_SS_EDGE_LOW_MASK: u32 = 0x00004000;
    pub const GPIO_QSPI_SS_EDGE_LOW: u32 = 1 << 14;

    /// Bits [13:13], mask 0x00002000
    pub const GPIO_QSPI_SS_LEVEL_HIGH_OFFSET: u32 = 13;
    pub const GPIO_QSPI_SS_LEVEL_HIGH_MASK: u32 = 0x00002000;
    pub const GPIO_QSPI_SS_LEVEL_HIGH: u32 = 1 << 13;

    /// Bits [12:12], mask 0x00001000
    pub const GPIO_QSPI_SS_LEVEL_LOW_OFFSET: u32 = 12;
    pub const GPIO_QSPI_SS_LEVEL_LOW_MASK: u32 = 0x00001000;
    pub const GPIO_QSPI_SS_LEVEL_LOW: u32 = 1 << 12;

    /// Bits [11:11], mask 0x00000800
    pub const GPIO_QSPI_SCLK_EDGE_HIGH_OFFSET: u32 = 11;
    pub const GPIO_QSPI_SCLK_EDGE_HIGH_MASK: u32 = 0x00000800;
    pub const GPIO_QSPI_SCLK_EDGE_HIGH: u32 = 1 << 11;

    /// Bits [10:10], mask 0x00000400
    pub const GPIO_QSPI_SCLK_EDGE_LOW_OFFSET: u32 = 10;
    pub const GPIO_QSPI_SCLK_EDGE_LOW_MASK: u32 = 0x00000400;
    pub const GPIO_QSPI_SCLK_EDGE_LOW: u32 = 1 << 10;

    /// Bits [9:9], mask 0x00000200
    pub const GPIO_QSPI_SCLK_LEVEL_HIGH_OFFSET: u32 = 9;
    pub const GPIO_QSPI_SCLK_LEVEL_HIGH_MASK: u32 = 0x00000200;
    pub const GPIO_QSPI_SCLK_LEVEL_HIGH: u32 = 1 << 9;

    /// Bits [8:8], mask 0x00000100
    pub const GPIO_QSPI_SCLK_LEVEL_LOW_OFFSET: u32 = 8;
    pub const GPIO_QSPI_SCLK_LEVEL_LOW_MASK: u32 = 0x00000100;
    pub const GPIO_QSPI_SCLK_LEVEL_LOW: u32 = 1 << 8;

    /// Bits [7:7], mask 0x00000080
    pub const USBPHY_DM_EDGE_HIGH_OFFSET: u32 = 7;
    pub const USBPHY_DM_EDGE_HIGH_MASK: u32 = 0x00000080;
    pub const USBPHY_DM_EDGE_HIGH: u32 = 1 << 7;

    /// Bits [6:6], mask 0x00000040
    pub const USBPHY_DM_EDGE_LOW_OFFSET: u32 = 6;
    pub const USBPHY_DM_EDGE_LOW_MASK: u32 = 0x00000040;
    pub const USBPHY_DM_EDGE_LOW: u32 = 1 << 6;

    /// Bits [5:5], mask 0x00000020
    pub const USBPHY_DM_LEVEL_HIGH_OFFSET: u32 = 5;
    pub const USBPHY_DM_LEVEL_HIGH_MASK: u32 = 0x00000020;
    pub const USBPHY_DM_LEVEL_HIGH: u32 = 1 << 5;

    /// Bits [4:4], mask 0x00000010
    pub const USBPHY_DM_LEVEL_LOW_OFFSET: u32 = 4;
    pub const USBPHY_DM_LEVEL_LOW_MASK: u32 = 0x00000010;
    pub const USBPHY_DM_LEVEL_LOW: u32 = 1 << 4;

    /// Bits [3:3], mask 0x00000008
    pub const USBPHY_DP_EDGE_HIGH_OFFSET: u32 = 3;
    pub const USBPHY_DP_EDGE_HIGH_MASK: u32 = 0x00000008;
    pub const USBPHY_DP_EDGE_HIGH: u32 = 1 << 3;

    /// Bits [2:2], mask 0x00000004
    pub const USBPHY_DP_EDGE_LOW_OFFSET: u32 = 2;
    pub const USBPHY_DP_EDGE_LOW_MASK: u32 = 0x00000004;
    pub const USBPHY_DP_EDGE_LOW: u32 = 1 << 2;

    /// Bits [1:1], mask 0x00000002
    pub const USBPHY_DP_LEVEL_HIGH_OFFSET: u32 = 1;
    pub const USBPHY_DP_LEVEL_HIGH_MASK: u32 = 0x00000002;
    pub const USBPHY_DP_LEVEL_HIGH: u32 = 1 << 1;

    /// Bits [0:0], mask 0x00000001
    pub const USBPHY_DP_LEVEL_LOW_OFFSET: u32 = 0;
    pub const USBPHY_DP_LEVEL_LOW_MASK: u32 = 0x00000001;
    pub const USBPHY_DP_LEVEL_LOW: u32 = 1 << 0;

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

    /// **GPIO_QSPI_SD3_EDGE_HIGH** [31:31]
    #[inline(always)]
    pub fn gpio_qspi_sd3_edge_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD3_EDGE_HIGH_MASK) != 0
    }

    /// **GPIO_QSPI_SD3_EDGE_LOW** [30:30]
    #[inline(always)]
    pub fn gpio_qspi_sd3_edge_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD3_EDGE_LOW_MASK) != 0
    }

    /// **GPIO_QSPI_SD3_LEVEL_HIGH** [29:29]
    #[inline(always)]
    pub fn gpio_qspi_sd3_level_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD3_LEVEL_HIGH_MASK) != 0
    }

    /// **GPIO_QSPI_SD3_LEVEL_LOW** [28:28]
    #[inline(always)]
    pub fn gpio_qspi_sd3_level_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD3_LEVEL_LOW_MASK) != 0
    }

    /// **GPIO_QSPI_SD2_EDGE_HIGH** [27:27]
    #[inline(always)]
    pub fn gpio_qspi_sd2_edge_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD2_EDGE_HIGH_MASK) != 0
    }

    /// **GPIO_QSPI_SD2_EDGE_LOW** [26:26]
    #[inline(always)]
    pub fn gpio_qspi_sd2_edge_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD2_EDGE_LOW_MASK) != 0
    }

    /// **GPIO_QSPI_SD2_LEVEL_HIGH** [25:25]
    #[inline(always)]
    pub fn gpio_qspi_sd2_level_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD2_LEVEL_HIGH_MASK) != 0
    }

    /// **GPIO_QSPI_SD2_LEVEL_LOW** [24:24]
    #[inline(always)]
    pub fn gpio_qspi_sd2_level_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD2_LEVEL_LOW_MASK) != 0
    }

    /// **GPIO_QSPI_SD1_EDGE_HIGH** [23:23]
    #[inline(always)]
    pub fn gpio_qspi_sd1_edge_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD1_EDGE_HIGH_MASK) != 0
    }

    /// **GPIO_QSPI_SD1_EDGE_LOW** [22:22]
    #[inline(always)]
    pub fn gpio_qspi_sd1_edge_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD1_EDGE_LOW_MASK) != 0
    }

    /// **GPIO_QSPI_SD1_LEVEL_HIGH** [21:21]
    #[inline(always)]
    pub fn gpio_qspi_sd1_level_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD1_LEVEL_HIGH_MASK) != 0
    }

    /// **GPIO_QSPI_SD1_LEVEL_LOW** [20:20]
    #[inline(always)]
    pub fn gpio_qspi_sd1_level_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD1_LEVEL_LOW_MASK) != 0
    }

    /// **GPIO_QSPI_SD0_EDGE_HIGH** [19:19]
    #[inline(always)]
    pub fn gpio_qspi_sd0_edge_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD0_EDGE_HIGH_MASK) != 0
    }

    /// **GPIO_QSPI_SD0_EDGE_LOW** [18:18]
    #[inline(always)]
    pub fn gpio_qspi_sd0_edge_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD0_EDGE_LOW_MASK) != 0
    }

    /// **GPIO_QSPI_SD0_LEVEL_HIGH** [17:17]
    #[inline(always)]
    pub fn gpio_qspi_sd0_level_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD0_LEVEL_HIGH_MASK) != 0
    }

    /// **GPIO_QSPI_SD0_LEVEL_LOW** [16:16]
    #[inline(always)]
    pub fn gpio_qspi_sd0_level_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD0_LEVEL_LOW_MASK) != 0
    }

    /// **GPIO_QSPI_SS_EDGE_HIGH** [15:15]
    #[inline(always)]
    pub fn gpio_qspi_ss_edge_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SS_EDGE_HIGH_MASK) != 0
    }

    /// **GPIO_QSPI_SS_EDGE_LOW** [14:14]
    #[inline(always)]
    pub fn gpio_qspi_ss_edge_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SS_EDGE_LOW_MASK) != 0
    }

    /// **GPIO_QSPI_SS_LEVEL_HIGH** [13:13]
    #[inline(always)]
    pub fn gpio_qspi_ss_level_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SS_LEVEL_HIGH_MASK) != 0
    }

    /// **GPIO_QSPI_SS_LEVEL_LOW** [12:12]
    #[inline(always)]
    pub fn gpio_qspi_ss_level_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SS_LEVEL_LOW_MASK) != 0
    }

    /// **GPIO_QSPI_SCLK_EDGE_HIGH** [11:11]
    #[inline(always)]
    pub fn gpio_qspi_sclk_edge_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SCLK_EDGE_HIGH_MASK) != 0
    }

    /// **GPIO_QSPI_SCLK_EDGE_LOW** [10:10]
    #[inline(always)]
    pub fn gpio_qspi_sclk_edge_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SCLK_EDGE_LOW_MASK) != 0
    }

    /// **GPIO_QSPI_SCLK_LEVEL_HIGH** [9:9]
    #[inline(always)]
    pub fn gpio_qspi_sclk_level_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SCLK_LEVEL_HIGH_MASK) != 0
    }

    /// **GPIO_QSPI_SCLK_LEVEL_LOW** [8:8]
    #[inline(always)]
    pub fn gpio_qspi_sclk_level_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SCLK_LEVEL_LOW_MASK) != 0
    }

    /// **USBPHY_DM_EDGE_HIGH** [7:7]
    #[inline(always)]
    pub fn usbphy_dm_edge_high(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DM_EDGE_HIGH_MASK) != 0
    }

    /// **USBPHY_DM_EDGE_LOW** [6:6]
    #[inline(always)]
    pub fn usbphy_dm_edge_low(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DM_EDGE_LOW_MASK) != 0
    }

    /// **USBPHY_DM_LEVEL_HIGH** [5:5]
    #[inline(always)]
    pub fn usbphy_dm_level_high(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DM_LEVEL_HIGH_MASK) != 0
    }

    /// **USBPHY_DM_LEVEL_LOW** [4:4]
    #[inline(always)]
    pub fn usbphy_dm_level_low(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DM_LEVEL_LOW_MASK) != 0
    }

    /// **USBPHY_DP_EDGE_HIGH** [3:3]
    #[inline(always)]
    pub fn usbphy_dp_edge_high(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DP_EDGE_HIGH_MASK) != 0
    }

    /// **USBPHY_DP_EDGE_LOW** [2:2]
    #[inline(always)]
    pub fn usbphy_dp_edge_low(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DP_EDGE_LOW_MASK) != 0
    }

    /// **USBPHY_DP_LEVEL_HIGH** [1:1]
    #[inline(always)]
    pub fn usbphy_dp_level_high(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DP_LEVEL_HIGH_MASK) != 0
    }

    /// **USBPHY_DP_LEVEL_LOW** [0:0]
    #[inline(always)]
    pub fn usbphy_dp_level_low(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DP_LEVEL_LOW_MASK) != 0
    }

}

/// PROC0_INTS value type for batch read/modify/write operations
///
/// Use `proc0ints.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `proc0ints.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Proc0IntsVal {
    bits: u32,
}

impl Proc0IntsVal {
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

    /// Read GPIO_QSPI_SD3_EDGE_HIGH [31:31]
    #[inline(always)]
    pub const fn gpio_qspi_sd3_edge_high(&self) -> bool {
        (self.bits & Proc0Ints::GPIO_QSPI_SD3_EDGE_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD3_EDGE_LOW [30:30]
    #[inline(always)]
    pub const fn gpio_qspi_sd3_edge_low(&self) -> bool {
        (self.bits & Proc0Ints::GPIO_QSPI_SD3_EDGE_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SD3_LEVEL_HIGH [29:29]
    #[inline(always)]
    pub const fn gpio_qspi_sd3_level_high(&self) -> bool {
        (self.bits & Proc0Ints::GPIO_QSPI_SD3_LEVEL_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD3_LEVEL_LOW [28:28]
    #[inline(always)]
    pub const fn gpio_qspi_sd3_level_low(&self) -> bool {
        (self.bits & Proc0Ints::GPIO_QSPI_SD3_LEVEL_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SD2_EDGE_HIGH [27:27]
    #[inline(always)]
    pub const fn gpio_qspi_sd2_edge_high(&self) -> bool {
        (self.bits & Proc0Ints::GPIO_QSPI_SD2_EDGE_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD2_EDGE_LOW [26:26]
    #[inline(always)]
    pub const fn gpio_qspi_sd2_edge_low(&self) -> bool {
        (self.bits & Proc0Ints::GPIO_QSPI_SD2_EDGE_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SD2_LEVEL_HIGH [25:25]
    #[inline(always)]
    pub const fn gpio_qspi_sd2_level_high(&self) -> bool {
        (self.bits & Proc0Ints::GPIO_QSPI_SD2_LEVEL_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD2_LEVEL_LOW [24:24]
    #[inline(always)]
    pub const fn gpio_qspi_sd2_level_low(&self) -> bool {
        (self.bits & Proc0Ints::GPIO_QSPI_SD2_LEVEL_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SD1_EDGE_HIGH [23:23]
    #[inline(always)]
    pub const fn gpio_qspi_sd1_edge_high(&self) -> bool {
        (self.bits & Proc0Ints::GPIO_QSPI_SD1_EDGE_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD1_EDGE_LOW [22:22]
    #[inline(always)]
    pub const fn gpio_qspi_sd1_edge_low(&self) -> bool {
        (self.bits & Proc0Ints::GPIO_QSPI_SD1_EDGE_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SD1_LEVEL_HIGH [21:21]
    #[inline(always)]
    pub const fn gpio_qspi_sd1_level_high(&self) -> bool {
        (self.bits & Proc0Ints::GPIO_QSPI_SD1_LEVEL_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD1_LEVEL_LOW [20:20]
    #[inline(always)]
    pub const fn gpio_qspi_sd1_level_low(&self) -> bool {
        (self.bits & Proc0Ints::GPIO_QSPI_SD1_LEVEL_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SD0_EDGE_HIGH [19:19]
    #[inline(always)]
    pub const fn gpio_qspi_sd0_edge_high(&self) -> bool {
        (self.bits & Proc0Ints::GPIO_QSPI_SD0_EDGE_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD0_EDGE_LOW [18:18]
    #[inline(always)]
    pub const fn gpio_qspi_sd0_edge_low(&self) -> bool {
        (self.bits & Proc0Ints::GPIO_QSPI_SD0_EDGE_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SD0_LEVEL_HIGH [17:17]
    #[inline(always)]
    pub const fn gpio_qspi_sd0_level_high(&self) -> bool {
        (self.bits & Proc0Ints::GPIO_QSPI_SD0_LEVEL_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD0_LEVEL_LOW [16:16]
    #[inline(always)]
    pub const fn gpio_qspi_sd0_level_low(&self) -> bool {
        (self.bits & Proc0Ints::GPIO_QSPI_SD0_LEVEL_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SS_EDGE_HIGH [15:15]
    #[inline(always)]
    pub const fn gpio_qspi_ss_edge_high(&self) -> bool {
        (self.bits & Proc0Ints::GPIO_QSPI_SS_EDGE_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SS_EDGE_LOW [14:14]
    #[inline(always)]
    pub const fn gpio_qspi_ss_edge_low(&self) -> bool {
        (self.bits & Proc0Ints::GPIO_QSPI_SS_EDGE_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SS_LEVEL_HIGH [13:13]
    #[inline(always)]
    pub const fn gpio_qspi_ss_level_high(&self) -> bool {
        (self.bits & Proc0Ints::GPIO_QSPI_SS_LEVEL_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SS_LEVEL_LOW [12:12]
    #[inline(always)]
    pub const fn gpio_qspi_ss_level_low(&self) -> bool {
        (self.bits & Proc0Ints::GPIO_QSPI_SS_LEVEL_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SCLK_EDGE_HIGH [11:11]
    #[inline(always)]
    pub const fn gpio_qspi_sclk_edge_high(&self) -> bool {
        (self.bits & Proc0Ints::GPIO_QSPI_SCLK_EDGE_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SCLK_EDGE_LOW [10:10]
    #[inline(always)]
    pub const fn gpio_qspi_sclk_edge_low(&self) -> bool {
        (self.bits & Proc0Ints::GPIO_QSPI_SCLK_EDGE_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SCLK_LEVEL_HIGH [9:9]
    #[inline(always)]
    pub const fn gpio_qspi_sclk_level_high(&self) -> bool {
        (self.bits & Proc0Ints::GPIO_QSPI_SCLK_LEVEL_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SCLK_LEVEL_LOW [8:8]
    #[inline(always)]
    pub const fn gpio_qspi_sclk_level_low(&self) -> bool {
        (self.bits & Proc0Ints::GPIO_QSPI_SCLK_LEVEL_LOW_MASK) != 0
    }

    /// Read USBPHY_DM_EDGE_HIGH [7:7]
    #[inline(always)]
    pub const fn usbphy_dm_edge_high(&self) -> bool {
        (self.bits & Proc0Ints::USBPHY_DM_EDGE_HIGH_MASK) != 0
    }

    /// Read USBPHY_DM_EDGE_LOW [6:6]
    #[inline(always)]
    pub const fn usbphy_dm_edge_low(&self) -> bool {
        (self.bits & Proc0Ints::USBPHY_DM_EDGE_LOW_MASK) != 0
    }

    /// Read USBPHY_DM_LEVEL_HIGH [5:5]
    #[inline(always)]
    pub const fn usbphy_dm_level_high(&self) -> bool {
        (self.bits & Proc0Ints::USBPHY_DM_LEVEL_HIGH_MASK) != 0
    }

    /// Read USBPHY_DM_LEVEL_LOW [4:4]
    #[inline(always)]
    pub const fn usbphy_dm_level_low(&self) -> bool {
        (self.bits & Proc0Ints::USBPHY_DM_LEVEL_LOW_MASK) != 0
    }

    /// Read USBPHY_DP_EDGE_HIGH [3:3]
    #[inline(always)]
    pub const fn usbphy_dp_edge_high(&self) -> bool {
        (self.bits & Proc0Ints::USBPHY_DP_EDGE_HIGH_MASK) != 0
    }

    /// Read USBPHY_DP_EDGE_LOW [2:2]
    #[inline(always)]
    pub const fn usbphy_dp_edge_low(&self) -> bool {
        (self.bits & Proc0Ints::USBPHY_DP_EDGE_LOW_MASK) != 0
    }

    /// Read USBPHY_DP_LEVEL_HIGH [1:1]
    #[inline(always)]
    pub const fn usbphy_dp_level_high(&self) -> bool {
        (self.bits & Proc0Ints::USBPHY_DP_LEVEL_HIGH_MASK) != 0
    }

    /// Read USBPHY_DP_LEVEL_LOW [0:0]
    #[inline(always)]
    pub const fn usbphy_dp_level_low(&self) -> bool {
        (self.bits & Proc0Ints::USBPHY_DP_LEVEL_LOW_MASK) != 0
    }

}

impl Default for Proc0IntsVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Proc0Ints {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> Proc0IntsVal {
        Proc0IntsVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: Proc0IntsVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(Proc0IntsVal) -> Proc0IntsVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// PROC1_INTE Register
///
/// Interrupt Enable for proc1
///
/// Address offset: 0x0228
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **GPIO_QSPI_SD3_EDGE_HIGH** [31] (RW): 
/// - **GPIO_QSPI_SD3_EDGE_LOW** [30] (RW): 
/// - **GPIO_QSPI_SD3_LEVEL_HIGH** [29] (RW): 
/// - **GPIO_QSPI_SD3_LEVEL_LOW** [28] (RW): 
/// - **GPIO_QSPI_SD2_EDGE_HIGH** [27] (RW): 
/// - **GPIO_QSPI_SD2_EDGE_LOW** [26] (RW): 
/// - **GPIO_QSPI_SD2_LEVEL_HIGH** [25] (RW): 
/// - **GPIO_QSPI_SD2_LEVEL_LOW** [24] (RW): 
/// - **GPIO_QSPI_SD1_EDGE_HIGH** [23] (RW): 
/// - **GPIO_QSPI_SD1_EDGE_LOW** [22] (RW): 
/// - **GPIO_QSPI_SD1_LEVEL_HIGH** [21] (RW): 
/// - **GPIO_QSPI_SD1_LEVEL_LOW** [20] (RW): 
/// - **GPIO_QSPI_SD0_EDGE_HIGH** [19] (RW): 
/// - **GPIO_QSPI_SD0_EDGE_LOW** [18] (RW): 
/// - **GPIO_QSPI_SD0_LEVEL_HIGH** [17] (RW): 
/// - **GPIO_QSPI_SD0_LEVEL_LOW** [16] (RW): 
/// - **GPIO_QSPI_SS_EDGE_HIGH** [15] (RW): 
/// - **GPIO_QSPI_SS_EDGE_LOW** [14] (RW): 
/// - **GPIO_QSPI_SS_LEVEL_HIGH** [13] (RW): 
/// - **GPIO_QSPI_SS_LEVEL_LOW** [12] (RW): 
/// - **GPIO_QSPI_SCLK_EDGE_HIGH** [11] (RW): 
/// - **GPIO_QSPI_SCLK_EDGE_LOW** [10] (RW): 
/// - **GPIO_QSPI_SCLK_LEVEL_HIGH** [9] (RW): 
/// - **GPIO_QSPI_SCLK_LEVEL_LOW** [8] (RW): 
/// - **USBPHY_DM_EDGE_HIGH** [7] (RW): 
/// - **USBPHY_DM_EDGE_LOW** [6] (RW): 
/// - **USBPHY_DM_LEVEL_HIGH** [5] (RW): 
/// - **USBPHY_DM_LEVEL_LOW** [4] (RW): 
/// - **USBPHY_DP_EDGE_HIGH** [3] (RW): 
/// - **USBPHY_DP_EDGE_LOW** [2] (RW): 
/// - **USBPHY_DP_LEVEL_HIGH** [1] (RW): 
/// - **USBPHY_DP_LEVEL_LOW** [0] (RW): 
#[repr(transparent)]
pub struct Proc1Inte {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for Proc1Inte {}
unsafe impl Sync for Proc1Inte {}

impl Proc1Inte {
    // ======== Field Constants ========

    /// Bits [31:31], mask 0x80000000
    pub const GPIO_QSPI_SD3_EDGE_HIGH_OFFSET: u32 = 31;
    pub const GPIO_QSPI_SD3_EDGE_HIGH_MASK: u32 = 0x80000000;
    pub const GPIO_QSPI_SD3_EDGE_HIGH: u32 = 1 << 31;

    /// Bits [30:30], mask 0x40000000
    pub const GPIO_QSPI_SD3_EDGE_LOW_OFFSET: u32 = 30;
    pub const GPIO_QSPI_SD3_EDGE_LOW_MASK: u32 = 0x40000000;
    pub const GPIO_QSPI_SD3_EDGE_LOW: u32 = 1 << 30;

    /// Bits [29:29], mask 0x20000000
    pub const GPIO_QSPI_SD3_LEVEL_HIGH_OFFSET: u32 = 29;
    pub const GPIO_QSPI_SD3_LEVEL_HIGH_MASK: u32 = 0x20000000;
    pub const GPIO_QSPI_SD3_LEVEL_HIGH: u32 = 1 << 29;

    /// Bits [28:28], mask 0x10000000
    pub const GPIO_QSPI_SD3_LEVEL_LOW_OFFSET: u32 = 28;
    pub const GPIO_QSPI_SD3_LEVEL_LOW_MASK: u32 = 0x10000000;
    pub const GPIO_QSPI_SD3_LEVEL_LOW: u32 = 1 << 28;

    /// Bits [27:27], mask 0x08000000
    pub const GPIO_QSPI_SD2_EDGE_HIGH_OFFSET: u32 = 27;
    pub const GPIO_QSPI_SD2_EDGE_HIGH_MASK: u32 = 0x08000000;
    pub const GPIO_QSPI_SD2_EDGE_HIGH: u32 = 1 << 27;

    /// Bits [26:26], mask 0x04000000
    pub const GPIO_QSPI_SD2_EDGE_LOW_OFFSET: u32 = 26;
    pub const GPIO_QSPI_SD2_EDGE_LOW_MASK: u32 = 0x04000000;
    pub const GPIO_QSPI_SD2_EDGE_LOW: u32 = 1 << 26;

    /// Bits [25:25], mask 0x02000000
    pub const GPIO_QSPI_SD2_LEVEL_HIGH_OFFSET: u32 = 25;
    pub const GPIO_QSPI_SD2_LEVEL_HIGH_MASK: u32 = 0x02000000;
    pub const GPIO_QSPI_SD2_LEVEL_HIGH: u32 = 1 << 25;

    /// Bits [24:24], mask 0x01000000
    pub const GPIO_QSPI_SD2_LEVEL_LOW_OFFSET: u32 = 24;
    pub const GPIO_QSPI_SD2_LEVEL_LOW_MASK: u32 = 0x01000000;
    pub const GPIO_QSPI_SD2_LEVEL_LOW: u32 = 1 << 24;

    /// Bits [23:23], mask 0x00800000
    pub const GPIO_QSPI_SD1_EDGE_HIGH_OFFSET: u32 = 23;
    pub const GPIO_QSPI_SD1_EDGE_HIGH_MASK: u32 = 0x00800000;
    pub const GPIO_QSPI_SD1_EDGE_HIGH: u32 = 1 << 23;

    /// Bits [22:22], mask 0x00400000
    pub const GPIO_QSPI_SD1_EDGE_LOW_OFFSET: u32 = 22;
    pub const GPIO_QSPI_SD1_EDGE_LOW_MASK: u32 = 0x00400000;
    pub const GPIO_QSPI_SD1_EDGE_LOW: u32 = 1 << 22;

    /// Bits [21:21], mask 0x00200000
    pub const GPIO_QSPI_SD1_LEVEL_HIGH_OFFSET: u32 = 21;
    pub const GPIO_QSPI_SD1_LEVEL_HIGH_MASK: u32 = 0x00200000;
    pub const GPIO_QSPI_SD1_LEVEL_HIGH: u32 = 1 << 21;

    /// Bits [20:20], mask 0x00100000
    pub const GPIO_QSPI_SD1_LEVEL_LOW_OFFSET: u32 = 20;
    pub const GPIO_QSPI_SD1_LEVEL_LOW_MASK: u32 = 0x00100000;
    pub const GPIO_QSPI_SD1_LEVEL_LOW: u32 = 1 << 20;

    /// Bits [19:19], mask 0x00080000
    pub const GPIO_QSPI_SD0_EDGE_HIGH_OFFSET: u32 = 19;
    pub const GPIO_QSPI_SD0_EDGE_HIGH_MASK: u32 = 0x00080000;
    pub const GPIO_QSPI_SD0_EDGE_HIGH: u32 = 1 << 19;

    /// Bits [18:18], mask 0x00040000
    pub const GPIO_QSPI_SD0_EDGE_LOW_OFFSET: u32 = 18;
    pub const GPIO_QSPI_SD0_EDGE_LOW_MASK: u32 = 0x00040000;
    pub const GPIO_QSPI_SD0_EDGE_LOW: u32 = 1 << 18;

    /// Bits [17:17], mask 0x00020000
    pub const GPIO_QSPI_SD0_LEVEL_HIGH_OFFSET: u32 = 17;
    pub const GPIO_QSPI_SD0_LEVEL_HIGH_MASK: u32 = 0x00020000;
    pub const GPIO_QSPI_SD0_LEVEL_HIGH: u32 = 1 << 17;

    /// Bits [16:16], mask 0x00010000
    pub const GPIO_QSPI_SD0_LEVEL_LOW_OFFSET: u32 = 16;
    pub const GPIO_QSPI_SD0_LEVEL_LOW_MASK: u32 = 0x00010000;
    pub const GPIO_QSPI_SD0_LEVEL_LOW: u32 = 1 << 16;

    /// Bits [15:15], mask 0x00008000
    pub const GPIO_QSPI_SS_EDGE_HIGH_OFFSET: u32 = 15;
    pub const GPIO_QSPI_SS_EDGE_HIGH_MASK: u32 = 0x00008000;
    pub const GPIO_QSPI_SS_EDGE_HIGH: u32 = 1 << 15;

    /// Bits [14:14], mask 0x00004000
    pub const GPIO_QSPI_SS_EDGE_LOW_OFFSET: u32 = 14;
    pub const GPIO_QSPI_SS_EDGE_LOW_MASK: u32 = 0x00004000;
    pub const GPIO_QSPI_SS_EDGE_LOW: u32 = 1 << 14;

    /// Bits [13:13], mask 0x00002000
    pub const GPIO_QSPI_SS_LEVEL_HIGH_OFFSET: u32 = 13;
    pub const GPIO_QSPI_SS_LEVEL_HIGH_MASK: u32 = 0x00002000;
    pub const GPIO_QSPI_SS_LEVEL_HIGH: u32 = 1 << 13;

    /// Bits [12:12], mask 0x00001000
    pub const GPIO_QSPI_SS_LEVEL_LOW_OFFSET: u32 = 12;
    pub const GPIO_QSPI_SS_LEVEL_LOW_MASK: u32 = 0x00001000;
    pub const GPIO_QSPI_SS_LEVEL_LOW: u32 = 1 << 12;

    /// Bits [11:11], mask 0x00000800
    pub const GPIO_QSPI_SCLK_EDGE_HIGH_OFFSET: u32 = 11;
    pub const GPIO_QSPI_SCLK_EDGE_HIGH_MASK: u32 = 0x00000800;
    pub const GPIO_QSPI_SCLK_EDGE_HIGH: u32 = 1 << 11;

    /// Bits [10:10], mask 0x00000400
    pub const GPIO_QSPI_SCLK_EDGE_LOW_OFFSET: u32 = 10;
    pub const GPIO_QSPI_SCLK_EDGE_LOW_MASK: u32 = 0x00000400;
    pub const GPIO_QSPI_SCLK_EDGE_LOW: u32 = 1 << 10;

    /// Bits [9:9], mask 0x00000200
    pub const GPIO_QSPI_SCLK_LEVEL_HIGH_OFFSET: u32 = 9;
    pub const GPIO_QSPI_SCLK_LEVEL_HIGH_MASK: u32 = 0x00000200;
    pub const GPIO_QSPI_SCLK_LEVEL_HIGH: u32 = 1 << 9;

    /// Bits [8:8], mask 0x00000100
    pub const GPIO_QSPI_SCLK_LEVEL_LOW_OFFSET: u32 = 8;
    pub const GPIO_QSPI_SCLK_LEVEL_LOW_MASK: u32 = 0x00000100;
    pub const GPIO_QSPI_SCLK_LEVEL_LOW: u32 = 1 << 8;

    /// Bits [7:7], mask 0x00000080
    pub const USBPHY_DM_EDGE_HIGH_OFFSET: u32 = 7;
    pub const USBPHY_DM_EDGE_HIGH_MASK: u32 = 0x00000080;
    pub const USBPHY_DM_EDGE_HIGH: u32 = 1 << 7;

    /// Bits [6:6], mask 0x00000040
    pub const USBPHY_DM_EDGE_LOW_OFFSET: u32 = 6;
    pub const USBPHY_DM_EDGE_LOW_MASK: u32 = 0x00000040;
    pub const USBPHY_DM_EDGE_LOW: u32 = 1 << 6;

    /// Bits [5:5], mask 0x00000020
    pub const USBPHY_DM_LEVEL_HIGH_OFFSET: u32 = 5;
    pub const USBPHY_DM_LEVEL_HIGH_MASK: u32 = 0x00000020;
    pub const USBPHY_DM_LEVEL_HIGH: u32 = 1 << 5;

    /// Bits [4:4], mask 0x00000010
    pub const USBPHY_DM_LEVEL_LOW_OFFSET: u32 = 4;
    pub const USBPHY_DM_LEVEL_LOW_MASK: u32 = 0x00000010;
    pub const USBPHY_DM_LEVEL_LOW: u32 = 1 << 4;

    /// Bits [3:3], mask 0x00000008
    pub const USBPHY_DP_EDGE_HIGH_OFFSET: u32 = 3;
    pub const USBPHY_DP_EDGE_HIGH_MASK: u32 = 0x00000008;
    pub const USBPHY_DP_EDGE_HIGH: u32 = 1 << 3;

    /// Bits [2:2], mask 0x00000004
    pub const USBPHY_DP_EDGE_LOW_OFFSET: u32 = 2;
    pub const USBPHY_DP_EDGE_LOW_MASK: u32 = 0x00000004;
    pub const USBPHY_DP_EDGE_LOW: u32 = 1 << 2;

    /// Bits [1:1], mask 0x00000002
    pub const USBPHY_DP_LEVEL_HIGH_OFFSET: u32 = 1;
    pub const USBPHY_DP_LEVEL_HIGH_MASK: u32 = 0x00000002;
    pub const USBPHY_DP_LEVEL_HIGH: u32 = 1 << 1;

    /// Bits [0:0], mask 0x00000001
    pub const USBPHY_DP_LEVEL_LOW_OFFSET: u32 = 0;
    pub const USBPHY_DP_LEVEL_LOW_MASK: u32 = 0x00000001;
    pub const USBPHY_DP_LEVEL_LOW: u32 = 1 << 0;

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

    /// **GPIO_QSPI_SD3_EDGE_HIGH** [31:31]
    #[inline(always)]
    pub fn gpio_qspi_sd3_edge_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD3_EDGE_HIGH_MASK) != 0
    }

    /// Set GPIO_QSPI_SD3_EDGE_HIGH bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd3_edge_high(&self) {
        self.set_bits(Self::GPIO_QSPI_SD3_EDGE_HIGH_MASK);
    }

    /// Clear GPIO_QSPI_SD3_EDGE_HIGH bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd3_edge_high(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD3_EDGE_HIGH_MASK);
    }

    /// **GPIO_QSPI_SD3_EDGE_LOW** [30:30]
    #[inline(always)]
    pub fn gpio_qspi_sd3_edge_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD3_EDGE_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SD3_EDGE_LOW bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd3_edge_low(&self) {
        self.set_bits(Self::GPIO_QSPI_SD3_EDGE_LOW_MASK);
    }

    /// Clear GPIO_QSPI_SD3_EDGE_LOW bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd3_edge_low(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD3_EDGE_LOW_MASK);
    }

    /// **GPIO_QSPI_SD3_LEVEL_HIGH** [29:29]
    #[inline(always)]
    pub fn gpio_qspi_sd3_level_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD3_LEVEL_HIGH_MASK) != 0
    }

    /// Set GPIO_QSPI_SD3_LEVEL_HIGH bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd3_level_high(&self) {
        self.set_bits(Self::GPIO_QSPI_SD3_LEVEL_HIGH_MASK);
    }

    /// Clear GPIO_QSPI_SD3_LEVEL_HIGH bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd3_level_high(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD3_LEVEL_HIGH_MASK);
    }

    /// **GPIO_QSPI_SD3_LEVEL_LOW** [28:28]
    #[inline(always)]
    pub fn gpio_qspi_sd3_level_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD3_LEVEL_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SD3_LEVEL_LOW bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd3_level_low(&self) {
        self.set_bits(Self::GPIO_QSPI_SD3_LEVEL_LOW_MASK);
    }

    /// Clear GPIO_QSPI_SD3_LEVEL_LOW bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd3_level_low(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD3_LEVEL_LOW_MASK);
    }

    /// **GPIO_QSPI_SD2_EDGE_HIGH** [27:27]
    #[inline(always)]
    pub fn gpio_qspi_sd2_edge_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD2_EDGE_HIGH_MASK) != 0
    }

    /// Set GPIO_QSPI_SD2_EDGE_HIGH bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd2_edge_high(&self) {
        self.set_bits(Self::GPIO_QSPI_SD2_EDGE_HIGH_MASK);
    }

    /// Clear GPIO_QSPI_SD2_EDGE_HIGH bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd2_edge_high(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD2_EDGE_HIGH_MASK);
    }

    /// **GPIO_QSPI_SD2_EDGE_LOW** [26:26]
    #[inline(always)]
    pub fn gpio_qspi_sd2_edge_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD2_EDGE_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SD2_EDGE_LOW bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd2_edge_low(&self) {
        self.set_bits(Self::GPIO_QSPI_SD2_EDGE_LOW_MASK);
    }

    /// Clear GPIO_QSPI_SD2_EDGE_LOW bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd2_edge_low(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD2_EDGE_LOW_MASK);
    }

    /// **GPIO_QSPI_SD2_LEVEL_HIGH** [25:25]
    #[inline(always)]
    pub fn gpio_qspi_sd2_level_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD2_LEVEL_HIGH_MASK) != 0
    }

    /// Set GPIO_QSPI_SD2_LEVEL_HIGH bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd2_level_high(&self) {
        self.set_bits(Self::GPIO_QSPI_SD2_LEVEL_HIGH_MASK);
    }

    /// Clear GPIO_QSPI_SD2_LEVEL_HIGH bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd2_level_high(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD2_LEVEL_HIGH_MASK);
    }

    /// **GPIO_QSPI_SD2_LEVEL_LOW** [24:24]
    #[inline(always)]
    pub fn gpio_qspi_sd2_level_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD2_LEVEL_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SD2_LEVEL_LOW bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd2_level_low(&self) {
        self.set_bits(Self::GPIO_QSPI_SD2_LEVEL_LOW_MASK);
    }

    /// Clear GPIO_QSPI_SD2_LEVEL_LOW bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd2_level_low(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD2_LEVEL_LOW_MASK);
    }

    /// **GPIO_QSPI_SD1_EDGE_HIGH** [23:23]
    #[inline(always)]
    pub fn gpio_qspi_sd1_edge_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD1_EDGE_HIGH_MASK) != 0
    }

    /// Set GPIO_QSPI_SD1_EDGE_HIGH bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd1_edge_high(&self) {
        self.set_bits(Self::GPIO_QSPI_SD1_EDGE_HIGH_MASK);
    }

    /// Clear GPIO_QSPI_SD1_EDGE_HIGH bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd1_edge_high(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD1_EDGE_HIGH_MASK);
    }

    /// **GPIO_QSPI_SD1_EDGE_LOW** [22:22]
    #[inline(always)]
    pub fn gpio_qspi_sd1_edge_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD1_EDGE_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SD1_EDGE_LOW bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd1_edge_low(&self) {
        self.set_bits(Self::GPIO_QSPI_SD1_EDGE_LOW_MASK);
    }

    /// Clear GPIO_QSPI_SD1_EDGE_LOW bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd1_edge_low(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD1_EDGE_LOW_MASK);
    }

    /// **GPIO_QSPI_SD1_LEVEL_HIGH** [21:21]
    #[inline(always)]
    pub fn gpio_qspi_sd1_level_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD1_LEVEL_HIGH_MASK) != 0
    }

    /// Set GPIO_QSPI_SD1_LEVEL_HIGH bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd1_level_high(&self) {
        self.set_bits(Self::GPIO_QSPI_SD1_LEVEL_HIGH_MASK);
    }

    /// Clear GPIO_QSPI_SD1_LEVEL_HIGH bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd1_level_high(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD1_LEVEL_HIGH_MASK);
    }

    /// **GPIO_QSPI_SD1_LEVEL_LOW** [20:20]
    #[inline(always)]
    pub fn gpio_qspi_sd1_level_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD1_LEVEL_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SD1_LEVEL_LOW bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd1_level_low(&self) {
        self.set_bits(Self::GPIO_QSPI_SD1_LEVEL_LOW_MASK);
    }

    /// Clear GPIO_QSPI_SD1_LEVEL_LOW bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd1_level_low(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD1_LEVEL_LOW_MASK);
    }

    /// **GPIO_QSPI_SD0_EDGE_HIGH** [19:19]
    #[inline(always)]
    pub fn gpio_qspi_sd0_edge_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD0_EDGE_HIGH_MASK) != 0
    }

    /// Set GPIO_QSPI_SD0_EDGE_HIGH bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd0_edge_high(&self) {
        self.set_bits(Self::GPIO_QSPI_SD0_EDGE_HIGH_MASK);
    }

    /// Clear GPIO_QSPI_SD0_EDGE_HIGH bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd0_edge_high(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD0_EDGE_HIGH_MASK);
    }

    /// **GPIO_QSPI_SD0_EDGE_LOW** [18:18]
    #[inline(always)]
    pub fn gpio_qspi_sd0_edge_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD0_EDGE_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SD0_EDGE_LOW bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd0_edge_low(&self) {
        self.set_bits(Self::GPIO_QSPI_SD0_EDGE_LOW_MASK);
    }

    /// Clear GPIO_QSPI_SD0_EDGE_LOW bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd0_edge_low(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD0_EDGE_LOW_MASK);
    }

    /// **GPIO_QSPI_SD0_LEVEL_HIGH** [17:17]
    #[inline(always)]
    pub fn gpio_qspi_sd0_level_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD0_LEVEL_HIGH_MASK) != 0
    }

    /// Set GPIO_QSPI_SD0_LEVEL_HIGH bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd0_level_high(&self) {
        self.set_bits(Self::GPIO_QSPI_SD0_LEVEL_HIGH_MASK);
    }

    /// Clear GPIO_QSPI_SD0_LEVEL_HIGH bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd0_level_high(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD0_LEVEL_HIGH_MASK);
    }

    /// **GPIO_QSPI_SD0_LEVEL_LOW** [16:16]
    #[inline(always)]
    pub fn gpio_qspi_sd0_level_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD0_LEVEL_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SD0_LEVEL_LOW bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd0_level_low(&self) {
        self.set_bits(Self::GPIO_QSPI_SD0_LEVEL_LOW_MASK);
    }

    /// Clear GPIO_QSPI_SD0_LEVEL_LOW bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd0_level_low(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD0_LEVEL_LOW_MASK);
    }

    /// **GPIO_QSPI_SS_EDGE_HIGH** [15:15]
    #[inline(always)]
    pub fn gpio_qspi_ss_edge_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SS_EDGE_HIGH_MASK) != 0
    }

    /// Set GPIO_QSPI_SS_EDGE_HIGH bit
    #[inline(always)]
    pub fn set_gpio_qspi_ss_edge_high(&self) {
        self.set_bits(Self::GPIO_QSPI_SS_EDGE_HIGH_MASK);
    }

    /// Clear GPIO_QSPI_SS_EDGE_HIGH bit
    #[inline(always)]
    pub fn clr_gpio_qspi_ss_edge_high(&self) {
        self.clr_bits(Self::GPIO_QSPI_SS_EDGE_HIGH_MASK);
    }

    /// **GPIO_QSPI_SS_EDGE_LOW** [14:14]
    #[inline(always)]
    pub fn gpio_qspi_ss_edge_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SS_EDGE_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SS_EDGE_LOW bit
    #[inline(always)]
    pub fn set_gpio_qspi_ss_edge_low(&self) {
        self.set_bits(Self::GPIO_QSPI_SS_EDGE_LOW_MASK);
    }

    /// Clear GPIO_QSPI_SS_EDGE_LOW bit
    #[inline(always)]
    pub fn clr_gpio_qspi_ss_edge_low(&self) {
        self.clr_bits(Self::GPIO_QSPI_SS_EDGE_LOW_MASK);
    }

    /// **GPIO_QSPI_SS_LEVEL_HIGH** [13:13]
    #[inline(always)]
    pub fn gpio_qspi_ss_level_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SS_LEVEL_HIGH_MASK) != 0
    }

    /// Set GPIO_QSPI_SS_LEVEL_HIGH bit
    #[inline(always)]
    pub fn set_gpio_qspi_ss_level_high(&self) {
        self.set_bits(Self::GPIO_QSPI_SS_LEVEL_HIGH_MASK);
    }

    /// Clear GPIO_QSPI_SS_LEVEL_HIGH bit
    #[inline(always)]
    pub fn clr_gpio_qspi_ss_level_high(&self) {
        self.clr_bits(Self::GPIO_QSPI_SS_LEVEL_HIGH_MASK);
    }

    /// **GPIO_QSPI_SS_LEVEL_LOW** [12:12]
    #[inline(always)]
    pub fn gpio_qspi_ss_level_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SS_LEVEL_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SS_LEVEL_LOW bit
    #[inline(always)]
    pub fn set_gpio_qspi_ss_level_low(&self) {
        self.set_bits(Self::GPIO_QSPI_SS_LEVEL_LOW_MASK);
    }

    /// Clear GPIO_QSPI_SS_LEVEL_LOW bit
    #[inline(always)]
    pub fn clr_gpio_qspi_ss_level_low(&self) {
        self.clr_bits(Self::GPIO_QSPI_SS_LEVEL_LOW_MASK);
    }

    /// **GPIO_QSPI_SCLK_EDGE_HIGH** [11:11]
    #[inline(always)]
    pub fn gpio_qspi_sclk_edge_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SCLK_EDGE_HIGH_MASK) != 0
    }

    /// Set GPIO_QSPI_SCLK_EDGE_HIGH bit
    #[inline(always)]
    pub fn set_gpio_qspi_sclk_edge_high(&self) {
        self.set_bits(Self::GPIO_QSPI_SCLK_EDGE_HIGH_MASK);
    }

    /// Clear GPIO_QSPI_SCLK_EDGE_HIGH bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sclk_edge_high(&self) {
        self.clr_bits(Self::GPIO_QSPI_SCLK_EDGE_HIGH_MASK);
    }

    /// **GPIO_QSPI_SCLK_EDGE_LOW** [10:10]
    #[inline(always)]
    pub fn gpio_qspi_sclk_edge_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SCLK_EDGE_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SCLK_EDGE_LOW bit
    #[inline(always)]
    pub fn set_gpio_qspi_sclk_edge_low(&self) {
        self.set_bits(Self::GPIO_QSPI_SCLK_EDGE_LOW_MASK);
    }

    /// Clear GPIO_QSPI_SCLK_EDGE_LOW bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sclk_edge_low(&self) {
        self.clr_bits(Self::GPIO_QSPI_SCLK_EDGE_LOW_MASK);
    }

    /// **GPIO_QSPI_SCLK_LEVEL_HIGH** [9:9]
    #[inline(always)]
    pub fn gpio_qspi_sclk_level_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SCLK_LEVEL_HIGH_MASK) != 0
    }

    /// Set GPIO_QSPI_SCLK_LEVEL_HIGH bit
    #[inline(always)]
    pub fn set_gpio_qspi_sclk_level_high(&self) {
        self.set_bits(Self::GPIO_QSPI_SCLK_LEVEL_HIGH_MASK);
    }

    /// Clear GPIO_QSPI_SCLK_LEVEL_HIGH bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sclk_level_high(&self) {
        self.clr_bits(Self::GPIO_QSPI_SCLK_LEVEL_HIGH_MASK);
    }

    /// **GPIO_QSPI_SCLK_LEVEL_LOW** [8:8]
    #[inline(always)]
    pub fn gpio_qspi_sclk_level_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SCLK_LEVEL_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SCLK_LEVEL_LOW bit
    #[inline(always)]
    pub fn set_gpio_qspi_sclk_level_low(&self) {
        self.set_bits(Self::GPIO_QSPI_SCLK_LEVEL_LOW_MASK);
    }

    /// Clear GPIO_QSPI_SCLK_LEVEL_LOW bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sclk_level_low(&self) {
        self.clr_bits(Self::GPIO_QSPI_SCLK_LEVEL_LOW_MASK);
    }

    /// **USBPHY_DM_EDGE_HIGH** [7:7]
    #[inline(always)]
    pub fn usbphy_dm_edge_high(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DM_EDGE_HIGH_MASK) != 0
    }

    /// Set USBPHY_DM_EDGE_HIGH bit
    #[inline(always)]
    pub fn set_usbphy_dm_edge_high(&self) {
        self.set_bits(Self::USBPHY_DM_EDGE_HIGH_MASK);
    }

    /// Clear USBPHY_DM_EDGE_HIGH bit
    #[inline(always)]
    pub fn clr_usbphy_dm_edge_high(&self) {
        self.clr_bits(Self::USBPHY_DM_EDGE_HIGH_MASK);
    }

    /// **USBPHY_DM_EDGE_LOW** [6:6]
    #[inline(always)]
    pub fn usbphy_dm_edge_low(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DM_EDGE_LOW_MASK) != 0
    }

    /// Set USBPHY_DM_EDGE_LOW bit
    #[inline(always)]
    pub fn set_usbphy_dm_edge_low(&self) {
        self.set_bits(Self::USBPHY_DM_EDGE_LOW_MASK);
    }

    /// Clear USBPHY_DM_EDGE_LOW bit
    #[inline(always)]
    pub fn clr_usbphy_dm_edge_low(&self) {
        self.clr_bits(Self::USBPHY_DM_EDGE_LOW_MASK);
    }

    /// **USBPHY_DM_LEVEL_HIGH** [5:5]
    #[inline(always)]
    pub fn usbphy_dm_level_high(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DM_LEVEL_HIGH_MASK) != 0
    }

    /// Set USBPHY_DM_LEVEL_HIGH bit
    #[inline(always)]
    pub fn set_usbphy_dm_level_high(&self) {
        self.set_bits(Self::USBPHY_DM_LEVEL_HIGH_MASK);
    }

    /// Clear USBPHY_DM_LEVEL_HIGH bit
    #[inline(always)]
    pub fn clr_usbphy_dm_level_high(&self) {
        self.clr_bits(Self::USBPHY_DM_LEVEL_HIGH_MASK);
    }

    /// **USBPHY_DM_LEVEL_LOW** [4:4]
    #[inline(always)]
    pub fn usbphy_dm_level_low(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DM_LEVEL_LOW_MASK) != 0
    }

    /// Set USBPHY_DM_LEVEL_LOW bit
    #[inline(always)]
    pub fn set_usbphy_dm_level_low(&self) {
        self.set_bits(Self::USBPHY_DM_LEVEL_LOW_MASK);
    }

    /// Clear USBPHY_DM_LEVEL_LOW bit
    #[inline(always)]
    pub fn clr_usbphy_dm_level_low(&self) {
        self.clr_bits(Self::USBPHY_DM_LEVEL_LOW_MASK);
    }

    /// **USBPHY_DP_EDGE_HIGH** [3:3]
    #[inline(always)]
    pub fn usbphy_dp_edge_high(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DP_EDGE_HIGH_MASK) != 0
    }

    /// Set USBPHY_DP_EDGE_HIGH bit
    #[inline(always)]
    pub fn set_usbphy_dp_edge_high(&self) {
        self.set_bits(Self::USBPHY_DP_EDGE_HIGH_MASK);
    }

    /// Clear USBPHY_DP_EDGE_HIGH bit
    #[inline(always)]
    pub fn clr_usbphy_dp_edge_high(&self) {
        self.clr_bits(Self::USBPHY_DP_EDGE_HIGH_MASK);
    }

    /// **USBPHY_DP_EDGE_LOW** [2:2]
    #[inline(always)]
    pub fn usbphy_dp_edge_low(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DP_EDGE_LOW_MASK) != 0
    }

    /// Set USBPHY_DP_EDGE_LOW bit
    #[inline(always)]
    pub fn set_usbphy_dp_edge_low(&self) {
        self.set_bits(Self::USBPHY_DP_EDGE_LOW_MASK);
    }

    /// Clear USBPHY_DP_EDGE_LOW bit
    #[inline(always)]
    pub fn clr_usbphy_dp_edge_low(&self) {
        self.clr_bits(Self::USBPHY_DP_EDGE_LOW_MASK);
    }

    /// **USBPHY_DP_LEVEL_HIGH** [1:1]
    #[inline(always)]
    pub fn usbphy_dp_level_high(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DP_LEVEL_HIGH_MASK) != 0
    }

    /// Set USBPHY_DP_LEVEL_HIGH bit
    #[inline(always)]
    pub fn set_usbphy_dp_level_high(&self) {
        self.set_bits(Self::USBPHY_DP_LEVEL_HIGH_MASK);
    }

    /// Clear USBPHY_DP_LEVEL_HIGH bit
    #[inline(always)]
    pub fn clr_usbphy_dp_level_high(&self) {
        self.clr_bits(Self::USBPHY_DP_LEVEL_HIGH_MASK);
    }

    /// **USBPHY_DP_LEVEL_LOW** [0:0]
    #[inline(always)]
    pub fn usbphy_dp_level_low(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DP_LEVEL_LOW_MASK) != 0
    }

    /// Set USBPHY_DP_LEVEL_LOW bit
    #[inline(always)]
    pub fn set_usbphy_dp_level_low(&self) {
        self.set_bits(Self::USBPHY_DP_LEVEL_LOW_MASK);
    }

    /// Clear USBPHY_DP_LEVEL_LOW bit
    #[inline(always)]
    pub fn clr_usbphy_dp_level_low(&self) {
        self.clr_bits(Self::USBPHY_DP_LEVEL_LOW_MASK);
    }

}

/// PROC1_INTE value type for batch read/modify/write operations
///
/// Use `proc1inte.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `proc1inte.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Proc1InteVal {
    bits: u32,
}

impl Proc1InteVal {
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

    /// Read GPIO_QSPI_SD3_EDGE_HIGH [31:31]
    #[inline(always)]
    pub const fn gpio_qspi_sd3_edge_high(&self) -> bool {
        (self.bits & Proc1Inte::GPIO_QSPI_SD3_EDGE_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD3_EDGE_LOW [30:30]
    #[inline(always)]
    pub const fn gpio_qspi_sd3_edge_low(&self) -> bool {
        (self.bits & Proc1Inte::GPIO_QSPI_SD3_EDGE_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SD3_LEVEL_HIGH [29:29]
    #[inline(always)]
    pub const fn gpio_qspi_sd3_level_high(&self) -> bool {
        (self.bits & Proc1Inte::GPIO_QSPI_SD3_LEVEL_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD3_LEVEL_LOW [28:28]
    #[inline(always)]
    pub const fn gpio_qspi_sd3_level_low(&self) -> bool {
        (self.bits & Proc1Inte::GPIO_QSPI_SD3_LEVEL_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SD2_EDGE_HIGH [27:27]
    #[inline(always)]
    pub const fn gpio_qspi_sd2_edge_high(&self) -> bool {
        (self.bits & Proc1Inte::GPIO_QSPI_SD2_EDGE_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD2_EDGE_LOW [26:26]
    #[inline(always)]
    pub const fn gpio_qspi_sd2_edge_low(&self) -> bool {
        (self.bits & Proc1Inte::GPIO_QSPI_SD2_EDGE_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SD2_LEVEL_HIGH [25:25]
    #[inline(always)]
    pub const fn gpio_qspi_sd2_level_high(&self) -> bool {
        (self.bits & Proc1Inte::GPIO_QSPI_SD2_LEVEL_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD2_LEVEL_LOW [24:24]
    #[inline(always)]
    pub const fn gpio_qspi_sd2_level_low(&self) -> bool {
        (self.bits & Proc1Inte::GPIO_QSPI_SD2_LEVEL_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SD1_EDGE_HIGH [23:23]
    #[inline(always)]
    pub const fn gpio_qspi_sd1_edge_high(&self) -> bool {
        (self.bits & Proc1Inte::GPIO_QSPI_SD1_EDGE_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD1_EDGE_LOW [22:22]
    #[inline(always)]
    pub const fn gpio_qspi_sd1_edge_low(&self) -> bool {
        (self.bits & Proc1Inte::GPIO_QSPI_SD1_EDGE_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SD1_LEVEL_HIGH [21:21]
    #[inline(always)]
    pub const fn gpio_qspi_sd1_level_high(&self) -> bool {
        (self.bits & Proc1Inte::GPIO_QSPI_SD1_LEVEL_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD1_LEVEL_LOW [20:20]
    #[inline(always)]
    pub const fn gpio_qspi_sd1_level_low(&self) -> bool {
        (self.bits & Proc1Inte::GPIO_QSPI_SD1_LEVEL_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SD0_EDGE_HIGH [19:19]
    #[inline(always)]
    pub const fn gpio_qspi_sd0_edge_high(&self) -> bool {
        (self.bits & Proc1Inte::GPIO_QSPI_SD0_EDGE_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD0_EDGE_LOW [18:18]
    #[inline(always)]
    pub const fn gpio_qspi_sd0_edge_low(&self) -> bool {
        (self.bits & Proc1Inte::GPIO_QSPI_SD0_EDGE_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SD0_LEVEL_HIGH [17:17]
    #[inline(always)]
    pub const fn gpio_qspi_sd0_level_high(&self) -> bool {
        (self.bits & Proc1Inte::GPIO_QSPI_SD0_LEVEL_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD0_LEVEL_LOW [16:16]
    #[inline(always)]
    pub const fn gpio_qspi_sd0_level_low(&self) -> bool {
        (self.bits & Proc1Inte::GPIO_QSPI_SD0_LEVEL_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SS_EDGE_HIGH [15:15]
    #[inline(always)]
    pub const fn gpio_qspi_ss_edge_high(&self) -> bool {
        (self.bits & Proc1Inte::GPIO_QSPI_SS_EDGE_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SS_EDGE_LOW [14:14]
    #[inline(always)]
    pub const fn gpio_qspi_ss_edge_low(&self) -> bool {
        (self.bits & Proc1Inte::GPIO_QSPI_SS_EDGE_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SS_LEVEL_HIGH [13:13]
    #[inline(always)]
    pub const fn gpio_qspi_ss_level_high(&self) -> bool {
        (self.bits & Proc1Inte::GPIO_QSPI_SS_LEVEL_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SS_LEVEL_LOW [12:12]
    #[inline(always)]
    pub const fn gpio_qspi_ss_level_low(&self) -> bool {
        (self.bits & Proc1Inte::GPIO_QSPI_SS_LEVEL_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SCLK_EDGE_HIGH [11:11]
    #[inline(always)]
    pub const fn gpio_qspi_sclk_edge_high(&self) -> bool {
        (self.bits & Proc1Inte::GPIO_QSPI_SCLK_EDGE_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SCLK_EDGE_LOW [10:10]
    #[inline(always)]
    pub const fn gpio_qspi_sclk_edge_low(&self) -> bool {
        (self.bits & Proc1Inte::GPIO_QSPI_SCLK_EDGE_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SCLK_LEVEL_HIGH [9:9]
    #[inline(always)]
    pub const fn gpio_qspi_sclk_level_high(&self) -> bool {
        (self.bits & Proc1Inte::GPIO_QSPI_SCLK_LEVEL_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SCLK_LEVEL_LOW [8:8]
    #[inline(always)]
    pub const fn gpio_qspi_sclk_level_low(&self) -> bool {
        (self.bits & Proc1Inte::GPIO_QSPI_SCLK_LEVEL_LOW_MASK) != 0
    }

    /// Read USBPHY_DM_EDGE_HIGH [7:7]
    #[inline(always)]
    pub const fn usbphy_dm_edge_high(&self) -> bool {
        (self.bits & Proc1Inte::USBPHY_DM_EDGE_HIGH_MASK) != 0
    }

    /// Read USBPHY_DM_EDGE_LOW [6:6]
    #[inline(always)]
    pub const fn usbphy_dm_edge_low(&self) -> bool {
        (self.bits & Proc1Inte::USBPHY_DM_EDGE_LOW_MASK) != 0
    }

    /// Read USBPHY_DM_LEVEL_HIGH [5:5]
    #[inline(always)]
    pub const fn usbphy_dm_level_high(&self) -> bool {
        (self.bits & Proc1Inte::USBPHY_DM_LEVEL_HIGH_MASK) != 0
    }

    /// Read USBPHY_DM_LEVEL_LOW [4:4]
    #[inline(always)]
    pub const fn usbphy_dm_level_low(&self) -> bool {
        (self.bits & Proc1Inte::USBPHY_DM_LEVEL_LOW_MASK) != 0
    }

    /// Read USBPHY_DP_EDGE_HIGH [3:3]
    #[inline(always)]
    pub const fn usbphy_dp_edge_high(&self) -> bool {
        (self.bits & Proc1Inte::USBPHY_DP_EDGE_HIGH_MASK) != 0
    }

    /// Read USBPHY_DP_EDGE_LOW [2:2]
    #[inline(always)]
    pub const fn usbphy_dp_edge_low(&self) -> bool {
        (self.bits & Proc1Inte::USBPHY_DP_EDGE_LOW_MASK) != 0
    }

    /// Read USBPHY_DP_LEVEL_HIGH [1:1]
    #[inline(always)]
    pub const fn usbphy_dp_level_high(&self) -> bool {
        (self.bits & Proc1Inte::USBPHY_DP_LEVEL_HIGH_MASK) != 0
    }

    /// Read USBPHY_DP_LEVEL_LOW [0:0]
    #[inline(always)]
    pub const fn usbphy_dp_level_low(&self) -> bool {
        (self.bits & Proc1Inte::USBPHY_DP_LEVEL_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SD3_EDGE_HIGH [31:31]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd3_edge_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc1Inte::GPIO_QSPI_SD3_EDGE_HIGH_MASK
            } else {
                self.bits & !Proc1Inte::GPIO_QSPI_SD3_EDGE_HIGH_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD3_EDGE_LOW [30:30]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd3_edge_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc1Inte::GPIO_QSPI_SD3_EDGE_LOW_MASK
            } else {
                self.bits & !Proc1Inte::GPIO_QSPI_SD3_EDGE_LOW_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD3_LEVEL_HIGH [29:29]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd3_level_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc1Inte::GPIO_QSPI_SD3_LEVEL_HIGH_MASK
            } else {
                self.bits & !Proc1Inte::GPIO_QSPI_SD3_LEVEL_HIGH_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD3_LEVEL_LOW [28:28]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd3_level_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc1Inte::GPIO_QSPI_SD3_LEVEL_LOW_MASK
            } else {
                self.bits & !Proc1Inte::GPIO_QSPI_SD3_LEVEL_LOW_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD2_EDGE_HIGH [27:27]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd2_edge_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc1Inte::GPIO_QSPI_SD2_EDGE_HIGH_MASK
            } else {
                self.bits & !Proc1Inte::GPIO_QSPI_SD2_EDGE_HIGH_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD2_EDGE_LOW [26:26]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd2_edge_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc1Inte::GPIO_QSPI_SD2_EDGE_LOW_MASK
            } else {
                self.bits & !Proc1Inte::GPIO_QSPI_SD2_EDGE_LOW_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD2_LEVEL_HIGH [25:25]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd2_level_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc1Inte::GPIO_QSPI_SD2_LEVEL_HIGH_MASK
            } else {
                self.bits & !Proc1Inte::GPIO_QSPI_SD2_LEVEL_HIGH_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD2_LEVEL_LOW [24:24]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd2_level_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc1Inte::GPIO_QSPI_SD2_LEVEL_LOW_MASK
            } else {
                self.bits & !Proc1Inte::GPIO_QSPI_SD2_LEVEL_LOW_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD1_EDGE_HIGH [23:23]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd1_edge_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc1Inte::GPIO_QSPI_SD1_EDGE_HIGH_MASK
            } else {
                self.bits & !Proc1Inte::GPIO_QSPI_SD1_EDGE_HIGH_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD1_EDGE_LOW [22:22]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd1_edge_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc1Inte::GPIO_QSPI_SD1_EDGE_LOW_MASK
            } else {
                self.bits & !Proc1Inte::GPIO_QSPI_SD1_EDGE_LOW_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD1_LEVEL_HIGH [21:21]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd1_level_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc1Inte::GPIO_QSPI_SD1_LEVEL_HIGH_MASK
            } else {
                self.bits & !Proc1Inte::GPIO_QSPI_SD1_LEVEL_HIGH_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD1_LEVEL_LOW [20:20]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd1_level_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc1Inte::GPIO_QSPI_SD1_LEVEL_LOW_MASK
            } else {
                self.bits & !Proc1Inte::GPIO_QSPI_SD1_LEVEL_LOW_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD0_EDGE_HIGH [19:19]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd0_edge_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc1Inte::GPIO_QSPI_SD0_EDGE_HIGH_MASK
            } else {
                self.bits & !Proc1Inte::GPIO_QSPI_SD0_EDGE_HIGH_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD0_EDGE_LOW [18:18]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd0_edge_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc1Inte::GPIO_QSPI_SD0_EDGE_LOW_MASK
            } else {
                self.bits & !Proc1Inte::GPIO_QSPI_SD0_EDGE_LOW_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD0_LEVEL_HIGH [17:17]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd0_level_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc1Inte::GPIO_QSPI_SD0_LEVEL_HIGH_MASK
            } else {
                self.bits & !Proc1Inte::GPIO_QSPI_SD0_LEVEL_HIGH_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD0_LEVEL_LOW [16:16]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd0_level_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc1Inte::GPIO_QSPI_SD0_LEVEL_LOW_MASK
            } else {
                self.bits & !Proc1Inte::GPIO_QSPI_SD0_LEVEL_LOW_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SS_EDGE_HIGH [15:15]
    #[inline(always)]
    pub const fn with_gpio_qspi_ss_edge_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc1Inte::GPIO_QSPI_SS_EDGE_HIGH_MASK
            } else {
                self.bits & !Proc1Inte::GPIO_QSPI_SS_EDGE_HIGH_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SS_EDGE_LOW [14:14]
    #[inline(always)]
    pub const fn with_gpio_qspi_ss_edge_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc1Inte::GPIO_QSPI_SS_EDGE_LOW_MASK
            } else {
                self.bits & !Proc1Inte::GPIO_QSPI_SS_EDGE_LOW_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SS_LEVEL_HIGH [13:13]
    #[inline(always)]
    pub const fn with_gpio_qspi_ss_level_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc1Inte::GPIO_QSPI_SS_LEVEL_HIGH_MASK
            } else {
                self.bits & !Proc1Inte::GPIO_QSPI_SS_LEVEL_HIGH_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SS_LEVEL_LOW [12:12]
    #[inline(always)]
    pub const fn with_gpio_qspi_ss_level_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc1Inte::GPIO_QSPI_SS_LEVEL_LOW_MASK
            } else {
                self.bits & !Proc1Inte::GPIO_QSPI_SS_LEVEL_LOW_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SCLK_EDGE_HIGH [11:11]
    #[inline(always)]
    pub const fn with_gpio_qspi_sclk_edge_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc1Inte::GPIO_QSPI_SCLK_EDGE_HIGH_MASK
            } else {
                self.bits & !Proc1Inte::GPIO_QSPI_SCLK_EDGE_HIGH_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SCLK_EDGE_LOW [10:10]
    #[inline(always)]
    pub const fn with_gpio_qspi_sclk_edge_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc1Inte::GPIO_QSPI_SCLK_EDGE_LOW_MASK
            } else {
                self.bits & !Proc1Inte::GPIO_QSPI_SCLK_EDGE_LOW_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SCLK_LEVEL_HIGH [9:9]
    #[inline(always)]
    pub const fn with_gpio_qspi_sclk_level_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc1Inte::GPIO_QSPI_SCLK_LEVEL_HIGH_MASK
            } else {
                self.bits & !Proc1Inte::GPIO_QSPI_SCLK_LEVEL_HIGH_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SCLK_LEVEL_LOW [8:8]
    #[inline(always)]
    pub const fn with_gpio_qspi_sclk_level_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc1Inte::GPIO_QSPI_SCLK_LEVEL_LOW_MASK
            } else {
                self.bits & !Proc1Inte::GPIO_QSPI_SCLK_LEVEL_LOW_MASK
            }
        }
    }

    /// Set USBPHY_DM_EDGE_HIGH [7:7]
    #[inline(always)]
    pub const fn with_usbphy_dm_edge_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc1Inte::USBPHY_DM_EDGE_HIGH_MASK
            } else {
                self.bits & !Proc1Inte::USBPHY_DM_EDGE_HIGH_MASK
            }
        }
    }

    /// Set USBPHY_DM_EDGE_LOW [6:6]
    #[inline(always)]
    pub const fn with_usbphy_dm_edge_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc1Inte::USBPHY_DM_EDGE_LOW_MASK
            } else {
                self.bits & !Proc1Inte::USBPHY_DM_EDGE_LOW_MASK
            }
        }
    }

    /// Set USBPHY_DM_LEVEL_HIGH [5:5]
    #[inline(always)]
    pub const fn with_usbphy_dm_level_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc1Inte::USBPHY_DM_LEVEL_HIGH_MASK
            } else {
                self.bits & !Proc1Inte::USBPHY_DM_LEVEL_HIGH_MASK
            }
        }
    }

    /// Set USBPHY_DM_LEVEL_LOW [4:4]
    #[inline(always)]
    pub const fn with_usbphy_dm_level_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc1Inte::USBPHY_DM_LEVEL_LOW_MASK
            } else {
                self.bits & !Proc1Inte::USBPHY_DM_LEVEL_LOW_MASK
            }
        }
    }

    /// Set USBPHY_DP_EDGE_HIGH [3:3]
    #[inline(always)]
    pub const fn with_usbphy_dp_edge_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc1Inte::USBPHY_DP_EDGE_HIGH_MASK
            } else {
                self.bits & !Proc1Inte::USBPHY_DP_EDGE_HIGH_MASK
            }
        }
    }

    /// Set USBPHY_DP_EDGE_LOW [2:2]
    #[inline(always)]
    pub const fn with_usbphy_dp_edge_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc1Inte::USBPHY_DP_EDGE_LOW_MASK
            } else {
                self.bits & !Proc1Inte::USBPHY_DP_EDGE_LOW_MASK
            }
        }
    }

    /// Set USBPHY_DP_LEVEL_HIGH [1:1]
    #[inline(always)]
    pub const fn with_usbphy_dp_level_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc1Inte::USBPHY_DP_LEVEL_HIGH_MASK
            } else {
                self.bits & !Proc1Inte::USBPHY_DP_LEVEL_HIGH_MASK
            }
        }
    }

    /// Set USBPHY_DP_LEVEL_LOW [0:0]
    #[inline(always)]
    pub const fn with_usbphy_dp_level_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc1Inte::USBPHY_DP_LEVEL_LOW_MASK
            } else {
                self.bits & !Proc1Inte::USBPHY_DP_LEVEL_LOW_MASK
            }
        }
    }

}

impl Default for Proc1InteVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Proc1Inte {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> Proc1InteVal {
        Proc1InteVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: Proc1InteVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(Proc1InteVal) -> Proc1InteVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// PROC1_INTF Register
///
/// Interrupt Force for proc1
///
/// Address offset: 0x022c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **GPIO_QSPI_SD3_EDGE_HIGH** [31] (RW): 
/// - **GPIO_QSPI_SD3_EDGE_LOW** [30] (RW): 
/// - **GPIO_QSPI_SD3_LEVEL_HIGH** [29] (RW): 
/// - **GPIO_QSPI_SD3_LEVEL_LOW** [28] (RW): 
/// - **GPIO_QSPI_SD2_EDGE_HIGH** [27] (RW): 
/// - **GPIO_QSPI_SD2_EDGE_LOW** [26] (RW): 
/// - **GPIO_QSPI_SD2_LEVEL_HIGH** [25] (RW): 
/// - **GPIO_QSPI_SD2_LEVEL_LOW** [24] (RW): 
/// - **GPIO_QSPI_SD1_EDGE_HIGH** [23] (RW): 
/// - **GPIO_QSPI_SD1_EDGE_LOW** [22] (RW): 
/// - **GPIO_QSPI_SD1_LEVEL_HIGH** [21] (RW): 
/// - **GPIO_QSPI_SD1_LEVEL_LOW** [20] (RW): 
/// - **GPIO_QSPI_SD0_EDGE_HIGH** [19] (RW): 
/// - **GPIO_QSPI_SD0_EDGE_LOW** [18] (RW): 
/// - **GPIO_QSPI_SD0_LEVEL_HIGH** [17] (RW): 
/// - **GPIO_QSPI_SD0_LEVEL_LOW** [16] (RW): 
/// - **GPIO_QSPI_SS_EDGE_HIGH** [15] (RW): 
/// - **GPIO_QSPI_SS_EDGE_LOW** [14] (RW): 
/// - **GPIO_QSPI_SS_LEVEL_HIGH** [13] (RW): 
/// - **GPIO_QSPI_SS_LEVEL_LOW** [12] (RW): 
/// - **GPIO_QSPI_SCLK_EDGE_HIGH** [11] (RW): 
/// - **GPIO_QSPI_SCLK_EDGE_LOW** [10] (RW): 
/// - **GPIO_QSPI_SCLK_LEVEL_HIGH** [9] (RW): 
/// - **GPIO_QSPI_SCLK_LEVEL_LOW** [8] (RW): 
/// - **USBPHY_DM_EDGE_HIGH** [7] (RW): 
/// - **USBPHY_DM_EDGE_LOW** [6] (RW): 
/// - **USBPHY_DM_LEVEL_HIGH** [5] (RW): 
/// - **USBPHY_DM_LEVEL_LOW** [4] (RW): 
/// - **USBPHY_DP_EDGE_HIGH** [3] (RW): 
/// - **USBPHY_DP_EDGE_LOW** [2] (RW): 
/// - **USBPHY_DP_LEVEL_HIGH** [1] (RW): 
/// - **USBPHY_DP_LEVEL_LOW** [0] (RW): 
#[repr(transparent)]
pub struct Proc1Intf {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for Proc1Intf {}
unsafe impl Sync for Proc1Intf {}

impl Proc1Intf {
    // ======== Field Constants ========

    /// Bits [31:31], mask 0x80000000
    pub const GPIO_QSPI_SD3_EDGE_HIGH_OFFSET: u32 = 31;
    pub const GPIO_QSPI_SD3_EDGE_HIGH_MASK: u32 = 0x80000000;
    pub const GPIO_QSPI_SD3_EDGE_HIGH: u32 = 1 << 31;

    /// Bits [30:30], mask 0x40000000
    pub const GPIO_QSPI_SD3_EDGE_LOW_OFFSET: u32 = 30;
    pub const GPIO_QSPI_SD3_EDGE_LOW_MASK: u32 = 0x40000000;
    pub const GPIO_QSPI_SD3_EDGE_LOW: u32 = 1 << 30;

    /// Bits [29:29], mask 0x20000000
    pub const GPIO_QSPI_SD3_LEVEL_HIGH_OFFSET: u32 = 29;
    pub const GPIO_QSPI_SD3_LEVEL_HIGH_MASK: u32 = 0x20000000;
    pub const GPIO_QSPI_SD3_LEVEL_HIGH: u32 = 1 << 29;

    /// Bits [28:28], mask 0x10000000
    pub const GPIO_QSPI_SD3_LEVEL_LOW_OFFSET: u32 = 28;
    pub const GPIO_QSPI_SD3_LEVEL_LOW_MASK: u32 = 0x10000000;
    pub const GPIO_QSPI_SD3_LEVEL_LOW: u32 = 1 << 28;

    /// Bits [27:27], mask 0x08000000
    pub const GPIO_QSPI_SD2_EDGE_HIGH_OFFSET: u32 = 27;
    pub const GPIO_QSPI_SD2_EDGE_HIGH_MASK: u32 = 0x08000000;
    pub const GPIO_QSPI_SD2_EDGE_HIGH: u32 = 1 << 27;

    /// Bits [26:26], mask 0x04000000
    pub const GPIO_QSPI_SD2_EDGE_LOW_OFFSET: u32 = 26;
    pub const GPIO_QSPI_SD2_EDGE_LOW_MASK: u32 = 0x04000000;
    pub const GPIO_QSPI_SD2_EDGE_LOW: u32 = 1 << 26;

    /// Bits [25:25], mask 0x02000000
    pub const GPIO_QSPI_SD2_LEVEL_HIGH_OFFSET: u32 = 25;
    pub const GPIO_QSPI_SD2_LEVEL_HIGH_MASK: u32 = 0x02000000;
    pub const GPIO_QSPI_SD2_LEVEL_HIGH: u32 = 1 << 25;

    /// Bits [24:24], mask 0x01000000
    pub const GPIO_QSPI_SD2_LEVEL_LOW_OFFSET: u32 = 24;
    pub const GPIO_QSPI_SD2_LEVEL_LOW_MASK: u32 = 0x01000000;
    pub const GPIO_QSPI_SD2_LEVEL_LOW: u32 = 1 << 24;

    /// Bits [23:23], mask 0x00800000
    pub const GPIO_QSPI_SD1_EDGE_HIGH_OFFSET: u32 = 23;
    pub const GPIO_QSPI_SD1_EDGE_HIGH_MASK: u32 = 0x00800000;
    pub const GPIO_QSPI_SD1_EDGE_HIGH: u32 = 1 << 23;

    /// Bits [22:22], mask 0x00400000
    pub const GPIO_QSPI_SD1_EDGE_LOW_OFFSET: u32 = 22;
    pub const GPIO_QSPI_SD1_EDGE_LOW_MASK: u32 = 0x00400000;
    pub const GPIO_QSPI_SD1_EDGE_LOW: u32 = 1 << 22;

    /// Bits [21:21], mask 0x00200000
    pub const GPIO_QSPI_SD1_LEVEL_HIGH_OFFSET: u32 = 21;
    pub const GPIO_QSPI_SD1_LEVEL_HIGH_MASK: u32 = 0x00200000;
    pub const GPIO_QSPI_SD1_LEVEL_HIGH: u32 = 1 << 21;

    /// Bits [20:20], mask 0x00100000
    pub const GPIO_QSPI_SD1_LEVEL_LOW_OFFSET: u32 = 20;
    pub const GPIO_QSPI_SD1_LEVEL_LOW_MASK: u32 = 0x00100000;
    pub const GPIO_QSPI_SD1_LEVEL_LOW: u32 = 1 << 20;

    /// Bits [19:19], mask 0x00080000
    pub const GPIO_QSPI_SD0_EDGE_HIGH_OFFSET: u32 = 19;
    pub const GPIO_QSPI_SD0_EDGE_HIGH_MASK: u32 = 0x00080000;
    pub const GPIO_QSPI_SD0_EDGE_HIGH: u32 = 1 << 19;

    /// Bits [18:18], mask 0x00040000
    pub const GPIO_QSPI_SD0_EDGE_LOW_OFFSET: u32 = 18;
    pub const GPIO_QSPI_SD0_EDGE_LOW_MASK: u32 = 0x00040000;
    pub const GPIO_QSPI_SD0_EDGE_LOW: u32 = 1 << 18;

    /// Bits [17:17], mask 0x00020000
    pub const GPIO_QSPI_SD0_LEVEL_HIGH_OFFSET: u32 = 17;
    pub const GPIO_QSPI_SD0_LEVEL_HIGH_MASK: u32 = 0x00020000;
    pub const GPIO_QSPI_SD0_LEVEL_HIGH: u32 = 1 << 17;

    /// Bits [16:16], mask 0x00010000
    pub const GPIO_QSPI_SD0_LEVEL_LOW_OFFSET: u32 = 16;
    pub const GPIO_QSPI_SD0_LEVEL_LOW_MASK: u32 = 0x00010000;
    pub const GPIO_QSPI_SD0_LEVEL_LOW: u32 = 1 << 16;

    /// Bits [15:15], mask 0x00008000
    pub const GPIO_QSPI_SS_EDGE_HIGH_OFFSET: u32 = 15;
    pub const GPIO_QSPI_SS_EDGE_HIGH_MASK: u32 = 0x00008000;
    pub const GPIO_QSPI_SS_EDGE_HIGH: u32 = 1 << 15;

    /// Bits [14:14], mask 0x00004000
    pub const GPIO_QSPI_SS_EDGE_LOW_OFFSET: u32 = 14;
    pub const GPIO_QSPI_SS_EDGE_LOW_MASK: u32 = 0x00004000;
    pub const GPIO_QSPI_SS_EDGE_LOW: u32 = 1 << 14;

    /// Bits [13:13], mask 0x00002000
    pub const GPIO_QSPI_SS_LEVEL_HIGH_OFFSET: u32 = 13;
    pub const GPIO_QSPI_SS_LEVEL_HIGH_MASK: u32 = 0x00002000;
    pub const GPIO_QSPI_SS_LEVEL_HIGH: u32 = 1 << 13;

    /// Bits [12:12], mask 0x00001000
    pub const GPIO_QSPI_SS_LEVEL_LOW_OFFSET: u32 = 12;
    pub const GPIO_QSPI_SS_LEVEL_LOW_MASK: u32 = 0x00001000;
    pub const GPIO_QSPI_SS_LEVEL_LOW: u32 = 1 << 12;

    /// Bits [11:11], mask 0x00000800
    pub const GPIO_QSPI_SCLK_EDGE_HIGH_OFFSET: u32 = 11;
    pub const GPIO_QSPI_SCLK_EDGE_HIGH_MASK: u32 = 0x00000800;
    pub const GPIO_QSPI_SCLK_EDGE_HIGH: u32 = 1 << 11;

    /// Bits [10:10], mask 0x00000400
    pub const GPIO_QSPI_SCLK_EDGE_LOW_OFFSET: u32 = 10;
    pub const GPIO_QSPI_SCLK_EDGE_LOW_MASK: u32 = 0x00000400;
    pub const GPIO_QSPI_SCLK_EDGE_LOW: u32 = 1 << 10;

    /// Bits [9:9], mask 0x00000200
    pub const GPIO_QSPI_SCLK_LEVEL_HIGH_OFFSET: u32 = 9;
    pub const GPIO_QSPI_SCLK_LEVEL_HIGH_MASK: u32 = 0x00000200;
    pub const GPIO_QSPI_SCLK_LEVEL_HIGH: u32 = 1 << 9;

    /// Bits [8:8], mask 0x00000100
    pub const GPIO_QSPI_SCLK_LEVEL_LOW_OFFSET: u32 = 8;
    pub const GPIO_QSPI_SCLK_LEVEL_LOW_MASK: u32 = 0x00000100;
    pub const GPIO_QSPI_SCLK_LEVEL_LOW: u32 = 1 << 8;

    /// Bits [7:7], mask 0x00000080
    pub const USBPHY_DM_EDGE_HIGH_OFFSET: u32 = 7;
    pub const USBPHY_DM_EDGE_HIGH_MASK: u32 = 0x00000080;
    pub const USBPHY_DM_EDGE_HIGH: u32 = 1 << 7;

    /// Bits [6:6], mask 0x00000040
    pub const USBPHY_DM_EDGE_LOW_OFFSET: u32 = 6;
    pub const USBPHY_DM_EDGE_LOW_MASK: u32 = 0x00000040;
    pub const USBPHY_DM_EDGE_LOW: u32 = 1 << 6;

    /// Bits [5:5], mask 0x00000020
    pub const USBPHY_DM_LEVEL_HIGH_OFFSET: u32 = 5;
    pub const USBPHY_DM_LEVEL_HIGH_MASK: u32 = 0x00000020;
    pub const USBPHY_DM_LEVEL_HIGH: u32 = 1 << 5;

    /// Bits [4:4], mask 0x00000010
    pub const USBPHY_DM_LEVEL_LOW_OFFSET: u32 = 4;
    pub const USBPHY_DM_LEVEL_LOW_MASK: u32 = 0x00000010;
    pub const USBPHY_DM_LEVEL_LOW: u32 = 1 << 4;

    /// Bits [3:3], mask 0x00000008
    pub const USBPHY_DP_EDGE_HIGH_OFFSET: u32 = 3;
    pub const USBPHY_DP_EDGE_HIGH_MASK: u32 = 0x00000008;
    pub const USBPHY_DP_EDGE_HIGH: u32 = 1 << 3;

    /// Bits [2:2], mask 0x00000004
    pub const USBPHY_DP_EDGE_LOW_OFFSET: u32 = 2;
    pub const USBPHY_DP_EDGE_LOW_MASK: u32 = 0x00000004;
    pub const USBPHY_DP_EDGE_LOW: u32 = 1 << 2;

    /// Bits [1:1], mask 0x00000002
    pub const USBPHY_DP_LEVEL_HIGH_OFFSET: u32 = 1;
    pub const USBPHY_DP_LEVEL_HIGH_MASK: u32 = 0x00000002;
    pub const USBPHY_DP_LEVEL_HIGH: u32 = 1 << 1;

    /// Bits [0:0], mask 0x00000001
    pub const USBPHY_DP_LEVEL_LOW_OFFSET: u32 = 0;
    pub const USBPHY_DP_LEVEL_LOW_MASK: u32 = 0x00000001;
    pub const USBPHY_DP_LEVEL_LOW: u32 = 1 << 0;

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

    /// **GPIO_QSPI_SD3_EDGE_HIGH** [31:31]
    #[inline(always)]
    pub fn gpio_qspi_sd3_edge_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD3_EDGE_HIGH_MASK) != 0
    }

    /// Set GPIO_QSPI_SD3_EDGE_HIGH bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd3_edge_high(&self) {
        self.set_bits(Self::GPIO_QSPI_SD3_EDGE_HIGH_MASK);
    }

    /// Clear GPIO_QSPI_SD3_EDGE_HIGH bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd3_edge_high(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD3_EDGE_HIGH_MASK);
    }

    /// **GPIO_QSPI_SD3_EDGE_LOW** [30:30]
    #[inline(always)]
    pub fn gpio_qspi_sd3_edge_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD3_EDGE_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SD3_EDGE_LOW bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd3_edge_low(&self) {
        self.set_bits(Self::GPIO_QSPI_SD3_EDGE_LOW_MASK);
    }

    /// Clear GPIO_QSPI_SD3_EDGE_LOW bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd3_edge_low(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD3_EDGE_LOW_MASK);
    }

    /// **GPIO_QSPI_SD3_LEVEL_HIGH** [29:29]
    #[inline(always)]
    pub fn gpio_qspi_sd3_level_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD3_LEVEL_HIGH_MASK) != 0
    }

    /// Set GPIO_QSPI_SD3_LEVEL_HIGH bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd3_level_high(&self) {
        self.set_bits(Self::GPIO_QSPI_SD3_LEVEL_HIGH_MASK);
    }

    /// Clear GPIO_QSPI_SD3_LEVEL_HIGH bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd3_level_high(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD3_LEVEL_HIGH_MASK);
    }

    /// **GPIO_QSPI_SD3_LEVEL_LOW** [28:28]
    #[inline(always)]
    pub fn gpio_qspi_sd3_level_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD3_LEVEL_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SD3_LEVEL_LOW bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd3_level_low(&self) {
        self.set_bits(Self::GPIO_QSPI_SD3_LEVEL_LOW_MASK);
    }

    /// Clear GPIO_QSPI_SD3_LEVEL_LOW bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd3_level_low(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD3_LEVEL_LOW_MASK);
    }

    /// **GPIO_QSPI_SD2_EDGE_HIGH** [27:27]
    #[inline(always)]
    pub fn gpio_qspi_sd2_edge_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD2_EDGE_HIGH_MASK) != 0
    }

    /// Set GPIO_QSPI_SD2_EDGE_HIGH bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd2_edge_high(&self) {
        self.set_bits(Self::GPIO_QSPI_SD2_EDGE_HIGH_MASK);
    }

    /// Clear GPIO_QSPI_SD2_EDGE_HIGH bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd2_edge_high(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD2_EDGE_HIGH_MASK);
    }

    /// **GPIO_QSPI_SD2_EDGE_LOW** [26:26]
    #[inline(always)]
    pub fn gpio_qspi_sd2_edge_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD2_EDGE_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SD2_EDGE_LOW bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd2_edge_low(&self) {
        self.set_bits(Self::GPIO_QSPI_SD2_EDGE_LOW_MASK);
    }

    /// Clear GPIO_QSPI_SD2_EDGE_LOW bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd2_edge_low(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD2_EDGE_LOW_MASK);
    }

    /// **GPIO_QSPI_SD2_LEVEL_HIGH** [25:25]
    #[inline(always)]
    pub fn gpio_qspi_sd2_level_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD2_LEVEL_HIGH_MASK) != 0
    }

    /// Set GPIO_QSPI_SD2_LEVEL_HIGH bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd2_level_high(&self) {
        self.set_bits(Self::GPIO_QSPI_SD2_LEVEL_HIGH_MASK);
    }

    /// Clear GPIO_QSPI_SD2_LEVEL_HIGH bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd2_level_high(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD2_LEVEL_HIGH_MASK);
    }

    /// **GPIO_QSPI_SD2_LEVEL_LOW** [24:24]
    #[inline(always)]
    pub fn gpio_qspi_sd2_level_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD2_LEVEL_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SD2_LEVEL_LOW bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd2_level_low(&self) {
        self.set_bits(Self::GPIO_QSPI_SD2_LEVEL_LOW_MASK);
    }

    /// Clear GPIO_QSPI_SD2_LEVEL_LOW bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd2_level_low(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD2_LEVEL_LOW_MASK);
    }

    /// **GPIO_QSPI_SD1_EDGE_HIGH** [23:23]
    #[inline(always)]
    pub fn gpio_qspi_sd1_edge_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD1_EDGE_HIGH_MASK) != 0
    }

    /// Set GPIO_QSPI_SD1_EDGE_HIGH bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd1_edge_high(&self) {
        self.set_bits(Self::GPIO_QSPI_SD1_EDGE_HIGH_MASK);
    }

    /// Clear GPIO_QSPI_SD1_EDGE_HIGH bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd1_edge_high(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD1_EDGE_HIGH_MASK);
    }

    /// **GPIO_QSPI_SD1_EDGE_LOW** [22:22]
    #[inline(always)]
    pub fn gpio_qspi_sd1_edge_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD1_EDGE_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SD1_EDGE_LOW bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd1_edge_low(&self) {
        self.set_bits(Self::GPIO_QSPI_SD1_EDGE_LOW_MASK);
    }

    /// Clear GPIO_QSPI_SD1_EDGE_LOW bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd1_edge_low(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD1_EDGE_LOW_MASK);
    }

    /// **GPIO_QSPI_SD1_LEVEL_HIGH** [21:21]
    #[inline(always)]
    pub fn gpio_qspi_sd1_level_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD1_LEVEL_HIGH_MASK) != 0
    }

    /// Set GPIO_QSPI_SD1_LEVEL_HIGH bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd1_level_high(&self) {
        self.set_bits(Self::GPIO_QSPI_SD1_LEVEL_HIGH_MASK);
    }

    /// Clear GPIO_QSPI_SD1_LEVEL_HIGH bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd1_level_high(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD1_LEVEL_HIGH_MASK);
    }

    /// **GPIO_QSPI_SD1_LEVEL_LOW** [20:20]
    #[inline(always)]
    pub fn gpio_qspi_sd1_level_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD1_LEVEL_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SD1_LEVEL_LOW bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd1_level_low(&self) {
        self.set_bits(Self::GPIO_QSPI_SD1_LEVEL_LOW_MASK);
    }

    /// Clear GPIO_QSPI_SD1_LEVEL_LOW bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd1_level_low(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD1_LEVEL_LOW_MASK);
    }

    /// **GPIO_QSPI_SD0_EDGE_HIGH** [19:19]
    #[inline(always)]
    pub fn gpio_qspi_sd0_edge_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD0_EDGE_HIGH_MASK) != 0
    }

    /// Set GPIO_QSPI_SD0_EDGE_HIGH bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd0_edge_high(&self) {
        self.set_bits(Self::GPIO_QSPI_SD0_EDGE_HIGH_MASK);
    }

    /// Clear GPIO_QSPI_SD0_EDGE_HIGH bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd0_edge_high(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD0_EDGE_HIGH_MASK);
    }

    /// **GPIO_QSPI_SD0_EDGE_LOW** [18:18]
    #[inline(always)]
    pub fn gpio_qspi_sd0_edge_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD0_EDGE_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SD0_EDGE_LOW bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd0_edge_low(&self) {
        self.set_bits(Self::GPIO_QSPI_SD0_EDGE_LOW_MASK);
    }

    /// Clear GPIO_QSPI_SD0_EDGE_LOW bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd0_edge_low(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD0_EDGE_LOW_MASK);
    }

    /// **GPIO_QSPI_SD0_LEVEL_HIGH** [17:17]
    #[inline(always)]
    pub fn gpio_qspi_sd0_level_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD0_LEVEL_HIGH_MASK) != 0
    }

    /// Set GPIO_QSPI_SD0_LEVEL_HIGH bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd0_level_high(&self) {
        self.set_bits(Self::GPIO_QSPI_SD0_LEVEL_HIGH_MASK);
    }

    /// Clear GPIO_QSPI_SD0_LEVEL_HIGH bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd0_level_high(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD0_LEVEL_HIGH_MASK);
    }

    /// **GPIO_QSPI_SD0_LEVEL_LOW** [16:16]
    #[inline(always)]
    pub fn gpio_qspi_sd0_level_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD0_LEVEL_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SD0_LEVEL_LOW bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd0_level_low(&self) {
        self.set_bits(Self::GPIO_QSPI_SD0_LEVEL_LOW_MASK);
    }

    /// Clear GPIO_QSPI_SD0_LEVEL_LOW bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd0_level_low(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD0_LEVEL_LOW_MASK);
    }

    /// **GPIO_QSPI_SS_EDGE_HIGH** [15:15]
    #[inline(always)]
    pub fn gpio_qspi_ss_edge_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SS_EDGE_HIGH_MASK) != 0
    }

    /// Set GPIO_QSPI_SS_EDGE_HIGH bit
    #[inline(always)]
    pub fn set_gpio_qspi_ss_edge_high(&self) {
        self.set_bits(Self::GPIO_QSPI_SS_EDGE_HIGH_MASK);
    }

    /// Clear GPIO_QSPI_SS_EDGE_HIGH bit
    #[inline(always)]
    pub fn clr_gpio_qspi_ss_edge_high(&self) {
        self.clr_bits(Self::GPIO_QSPI_SS_EDGE_HIGH_MASK);
    }

    /// **GPIO_QSPI_SS_EDGE_LOW** [14:14]
    #[inline(always)]
    pub fn gpio_qspi_ss_edge_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SS_EDGE_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SS_EDGE_LOW bit
    #[inline(always)]
    pub fn set_gpio_qspi_ss_edge_low(&self) {
        self.set_bits(Self::GPIO_QSPI_SS_EDGE_LOW_MASK);
    }

    /// Clear GPIO_QSPI_SS_EDGE_LOW bit
    #[inline(always)]
    pub fn clr_gpio_qspi_ss_edge_low(&self) {
        self.clr_bits(Self::GPIO_QSPI_SS_EDGE_LOW_MASK);
    }

    /// **GPIO_QSPI_SS_LEVEL_HIGH** [13:13]
    #[inline(always)]
    pub fn gpio_qspi_ss_level_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SS_LEVEL_HIGH_MASK) != 0
    }

    /// Set GPIO_QSPI_SS_LEVEL_HIGH bit
    #[inline(always)]
    pub fn set_gpio_qspi_ss_level_high(&self) {
        self.set_bits(Self::GPIO_QSPI_SS_LEVEL_HIGH_MASK);
    }

    /// Clear GPIO_QSPI_SS_LEVEL_HIGH bit
    #[inline(always)]
    pub fn clr_gpio_qspi_ss_level_high(&self) {
        self.clr_bits(Self::GPIO_QSPI_SS_LEVEL_HIGH_MASK);
    }

    /// **GPIO_QSPI_SS_LEVEL_LOW** [12:12]
    #[inline(always)]
    pub fn gpio_qspi_ss_level_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SS_LEVEL_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SS_LEVEL_LOW bit
    #[inline(always)]
    pub fn set_gpio_qspi_ss_level_low(&self) {
        self.set_bits(Self::GPIO_QSPI_SS_LEVEL_LOW_MASK);
    }

    /// Clear GPIO_QSPI_SS_LEVEL_LOW bit
    #[inline(always)]
    pub fn clr_gpio_qspi_ss_level_low(&self) {
        self.clr_bits(Self::GPIO_QSPI_SS_LEVEL_LOW_MASK);
    }

    /// **GPIO_QSPI_SCLK_EDGE_HIGH** [11:11]
    #[inline(always)]
    pub fn gpio_qspi_sclk_edge_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SCLK_EDGE_HIGH_MASK) != 0
    }

    /// Set GPIO_QSPI_SCLK_EDGE_HIGH bit
    #[inline(always)]
    pub fn set_gpio_qspi_sclk_edge_high(&self) {
        self.set_bits(Self::GPIO_QSPI_SCLK_EDGE_HIGH_MASK);
    }

    /// Clear GPIO_QSPI_SCLK_EDGE_HIGH bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sclk_edge_high(&self) {
        self.clr_bits(Self::GPIO_QSPI_SCLK_EDGE_HIGH_MASK);
    }

    /// **GPIO_QSPI_SCLK_EDGE_LOW** [10:10]
    #[inline(always)]
    pub fn gpio_qspi_sclk_edge_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SCLK_EDGE_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SCLK_EDGE_LOW bit
    #[inline(always)]
    pub fn set_gpio_qspi_sclk_edge_low(&self) {
        self.set_bits(Self::GPIO_QSPI_SCLK_EDGE_LOW_MASK);
    }

    /// Clear GPIO_QSPI_SCLK_EDGE_LOW bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sclk_edge_low(&self) {
        self.clr_bits(Self::GPIO_QSPI_SCLK_EDGE_LOW_MASK);
    }

    /// **GPIO_QSPI_SCLK_LEVEL_HIGH** [9:9]
    #[inline(always)]
    pub fn gpio_qspi_sclk_level_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SCLK_LEVEL_HIGH_MASK) != 0
    }

    /// Set GPIO_QSPI_SCLK_LEVEL_HIGH bit
    #[inline(always)]
    pub fn set_gpio_qspi_sclk_level_high(&self) {
        self.set_bits(Self::GPIO_QSPI_SCLK_LEVEL_HIGH_MASK);
    }

    /// Clear GPIO_QSPI_SCLK_LEVEL_HIGH bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sclk_level_high(&self) {
        self.clr_bits(Self::GPIO_QSPI_SCLK_LEVEL_HIGH_MASK);
    }

    /// **GPIO_QSPI_SCLK_LEVEL_LOW** [8:8]
    #[inline(always)]
    pub fn gpio_qspi_sclk_level_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SCLK_LEVEL_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SCLK_LEVEL_LOW bit
    #[inline(always)]
    pub fn set_gpio_qspi_sclk_level_low(&self) {
        self.set_bits(Self::GPIO_QSPI_SCLK_LEVEL_LOW_MASK);
    }

    /// Clear GPIO_QSPI_SCLK_LEVEL_LOW bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sclk_level_low(&self) {
        self.clr_bits(Self::GPIO_QSPI_SCLK_LEVEL_LOW_MASK);
    }

    /// **USBPHY_DM_EDGE_HIGH** [7:7]
    #[inline(always)]
    pub fn usbphy_dm_edge_high(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DM_EDGE_HIGH_MASK) != 0
    }

    /// Set USBPHY_DM_EDGE_HIGH bit
    #[inline(always)]
    pub fn set_usbphy_dm_edge_high(&self) {
        self.set_bits(Self::USBPHY_DM_EDGE_HIGH_MASK);
    }

    /// Clear USBPHY_DM_EDGE_HIGH bit
    #[inline(always)]
    pub fn clr_usbphy_dm_edge_high(&self) {
        self.clr_bits(Self::USBPHY_DM_EDGE_HIGH_MASK);
    }

    /// **USBPHY_DM_EDGE_LOW** [6:6]
    #[inline(always)]
    pub fn usbphy_dm_edge_low(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DM_EDGE_LOW_MASK) != 0
    }

    /// Set USBPHY_DM_EDGE_LOW bit
    #[inline(always)]
    pub fn set_usbphy_dm_edge_low(&self) {
        self.set_bits(Self::USBPHY_DM_EDGE_LOW_MASK);
    }

    /// Clear USBPHY_DM_EDGE_LOW bit
    #[inline(always)]
    pub fn clr_usbphy_dm_edge_low(&self) {
        self.clr_bits(Self::USBPHY_DM_EDGE_LOW_MASK);
    }

    /// **USBPHY_DM_LEVEL_HIGH** [5:5]
    #[inline(always)]
    pub fn usbphy_dm_level_high(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DM_LEVEL_HIGH_MASK) != 0
    }

    /// Set USBPHY_DM_LEVEL_HIGH bit
    #[inline(always)]
    pub fn set_usbphy_dm_level_high(&self) {
        self.set_bits(Self::USBPHY_DM_LEVEL_HIGH_MASK);
    }

    /// Clear USBPHY_DM_LEVEL_HIGH bit
    #[inline(always)]
    pub fn clr_usbphy_dm_level_high(&self) {
        self.clr_bits(Self::USBPHY_DM_LEVEL_HIGH_MASK);
    }

    /// **USBPHY_DM_LEVEL_LOW** [4:4]
    #[inline(always)]
    pub fn usbphy_dm_level_low(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DM_LEVEL_LOW_MASK) != 0
    }

    /// Set USBPHY_DM_LEVEL_LOW bit
    #[inline(always)]
    pub fn set_usbphy_dm_level_low(&self) {
        self.set_bits(Self::USBPHY_DM_LEVEL_LOW_MASK);
    }

    /// Clear USBPHY_DM_LEVEL_LOW bit
    #[inline(always)]
    pub fn clr_usbphy_dm_level_low(&self) {
        self.clr_bits(Self::USBPHY_DM_LEVEL_LOW_MASK);
    }

    /// **USBPHY_DP_EDGE_HIGH** [3:3]
    #[inline(always)]
    pub fn usbphy_dp_edge_high(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DP_EDGE_HIGH_MASK) != 0
    }

    /// Set USBPHY_DP_EDGE_HIGH bit
    #[inline(always)]
    pub fn set_usbphy_dp_edge_high(&self) {
        self.set_bits(Self::USBPHY_DP_EDGE_HIGH_MASK);
    }

    /// Clear USBPHY_DP_EDGE_HIGH bit
    #[inline(always)]
    pub fn clr_usbphy_dp_edge_high(&self) {
        self.clr_bits(Self::USBPHY_DP_EDGE_HIGH_MASK);
    }

    /// **USBPHY_DP_EDGE_LOW** [2:2]
    #[inline(always)]
    pub fn usbphy_dp_edge_low(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DP_EDGE_LOW_MASK) != 0
    }

    /// Set USBPHY_DP_EDGE_LOW bit
    #[inline(always)]
    pub fn set_usbphy_dp_edge_low(&self) {
        self.set_bits(Self::USBPHY_DP_EDGE_LOW_MASK);
    }

    /// Clear USBPHY_DP_EDGE_LOW bit
    #[inline(always)]
    pub fn clr_usbphy_dp_edge_low(&self) {
        self.clr_bits(Self::USBPHY_DP_EDGE_LOW_MASK);
    }

    /// **USBPHY_DP_LEVEL_HIGH** [1:1]
    #[inline(always)]
    pub fn usbphy_dp_level_high(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DP_LEVEL_HIGH_MASK) != 0
    }

    /// Set USBPHY_DP_LEVEL_HIGH bit
    #[inline(always)]
    pub fn set_usbphy_dp_level_high(&self) {
        self.set_bits(Self::USBPHY_DP_LEVEL_HIGH_MASK);
    }

    /// Clear USBPHY_DP_LEVEL_HIGH bit
    #[inline(always)]
    pub fn clr_usbphy_dp_level_high(&self) {
        self.clr_bits(Self::USBPHY_DP_LEVEL_HIGH_MASK);
    }

    /// **USBPHY_DP_LEVEL_LOW** [0:0]
    #[inline(always)]
    pub fn usbphy_dp_level_low(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DP_LEVEL_LOW_MASK) != 0
    }

    /// Set USBPHY_DP_LEVEL_LOW bit
    #[inline(always)]
    pub fn set_usbphy_dp_level_low(&self) {
        self.set_bits(Self::USBPHY_DP_LEVEL_LOW_MASK);
    }

    /// Clear USBPHY_DP_LEVEL_LOW bit
    #[inline(always)]
    pub fn clr_usbphy_dp_level_low(&self) {
        self.clr_bits(Self::USBPHY_DP_LEVEL_LOW_MASK);
    }

}

/// PROC1_INTF value type for batch read/modify/write operations
///
/// Use `proc1intf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `proc1intf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Proc1IntfVal {
    bits: u32,
}

impl Proc1IntfVal {
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

    /// Read GPIO_QSPI_SD3_EDGE_HIGH [31:31]
    #[inline(always)]
    pub const fn gpio_qspi_sd3_edge_high(&self) -> bool {
        (self.bits & Proc1Intf::GPIO_QSPI_SD3_EDGE_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD3_EDGE_LOW [30:30]
    #[inline(always)]
    pub const fn gpio_qspi_sd3_edge_low(&self) -> bool {
        (self.bits & Proc1Intf::GPIO_QSPI_SD3_EDGE_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SD3_LEVEL_HIGH [29:29]
    #[inline(always)]
    pub const fn gpio_qspi_sd3_level_high(&self) -> bool {
        (self.bits & Proc1Intf::GPIO_QSPI_SD3_LEVEL_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD3_LEVEL_LOW [28:28]
    #[inline(always)]
    pub const fn gpio_qspi_sd3_level_low(&self) -> bool {
        (self.bits & Proc1Intf::GPIO_QSPI_SD3_LEVEL_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SD2_EDGE_HIGH [27:27]
    #[inline(always)]
    pub const fn gpio_qspi_sd2_edge_high(&self) -> bool {
        (self.bits & Proc1Intf::GPIO_QSPI_SD2_EDGE_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD2_EDGE_LOW [26:26]
    #[inline(always)]
    pub const fn gpio_qspi_sd2_edge_low(&self) -> bool {
        (self.bits & Proc1Intf::GPIO_QSPI_SD2_EDGE_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SD2_LEVEL_HIGH [25:25]
    #[inline(always)]
    pub const fn gpio_qspi_sd2_level_high(&self) -> bool {
        (self.bits & Proc1Intf::GPIO_QSPI_SD2_LEVEL_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD2_LEVEL_LOW [24:24]
    #[inline(always)]
    pub const fn gpio_qspi_sd2_level_low(&self) -> bool {
        (self.bits & Proc1Intf::GPIO_QSPI_SD2_LEVEL_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SD1_EDGE_HIGH [23:23]
    #[inline(always)]
    pub const fn gpio_qspi_sd1_edge_high(&self) -> bool {
        (self.bits & Proc1Intf::GPIO_QSPI_SD1_EDGE_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD1_EDGE_LOW [22:22]
    #[inline(always)]
    pub const fn gpio_qspi_sd1_edge_low(&self) -> bool {
        (self.bits & Proc1Intf::GPIO_QSPI_SD1_EDGE_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SD1_LEVEL_HIGH [21:21]
    #[inline(always)]
    pub const fn gpio_qspi_sd1_level_high(&self) -> bool {
        (self.bits & Proc1Intf::GPIO_QSPI_SD1_LEVEL_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD1_LEVEL_LOW [20:20]
    #[inline(always)]
    pub const fn gpio_qspi_sd1_level_low(&self) -> bool {
        (self.bits & Proc1Intf::GPIO_QSPI_SD1_LEVEL_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SD0_EDGE_HIGH [19:19]
    #[inline(always)]
    pub const fn gpio_qspi_sd0_edge_high(&self) -> bool {
        (self.bits & Proc1Intf::GPIO_QSPI_SD0_EDGE_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD0_EDGE_LOW [18:18]
    #[inline(always)]
    pub const fn gpio_qspi_sd0_edge_low(&self) -> bool {
        (self.bits & Proc1Intf::GPIO_QSPI_SD0_EDGE_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SD0_LEVEL_HIGH [17:17]
    #[inline(always)]
    pub const fn gpio_qspi_sd0_level_high(&self) -> bool {
        (self.bits & Proc1Intf::GPIO_QSPI_SD0_LEVEL_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD0_LEVEL_LOW [16:16]
    #[inline(always)]
    pub const fn gpio_qspi_sd0_level_low(&self) -> bool {
        (self.bits & Proc1Intf::GPIO_QSPI_SD0_LEVEL_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SS_EDGE_HIGH [15:15]
    #[inline(always)]
    pub const fn gpio_qspi_ss_edge_high(&self) -> bool {
        (self.bits & Proc1Intf::GPIO_QSPI_SS_EDGE_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SS_EDGE_LOW [14:14]
    #[inline(always)]
    pub const fn gpio_qspi_ss_edge_low(&self) -> bool {
        (self.bits & Proc1Intf::GPIO_QSPI_SS_EDGE_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SS_LEVEL_HIGH [13:13]
    #[inline(always)]
    pub const fn gpio_qspi_ss_level_high(&self) -> bool {
        (self.bits & Proc1Intf::GPIO_QSPI_SS_LEVEL_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SS_LEVEL_LOW [12:12]
    #[inline(always)]
    pub const fn gpio_qspi_ss_level_low(&self) -> bool {
        (self.bits & Proc1Intf::GPIO_QSPI_SS_LEVEL_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SCLK_EDGE_HIGH [11:11]
    #[inline(always)]
    pub const fn gpio_qspi_sclk_edge_high(&self) -> bool {
        (self.bits & Proc1Intf::GPIO_QSPI_SCLK_EDGE_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SCLK_EDGE_LOW [10:10]
    #[inline(always)]
    pub const fn gpio_qspi_sclk_edge_low(&self) -> bool {
        (self.bits & Proc1Intf::GPIO_QSPI_SCLK_EDGE_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SCLK_LEVEL_HIGH [9:9]
    #[inline(always)]
    pub const fn gpio_qspi_sclk_level_high(&self) -> bool {
        (self.bits & Proc1Intf::GPIO_QSPI_SCLK_LEVEL_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SCLK_LEVEL_LOW [8:8]
    #[inline(always)]
    pub const fn gpio_qspi_sclk_level_low(&self) -> bool {
        (self.bits & Proc1Intf::GPIO_QSPI_SCLK_LEVEL_LOW_MASK) != 0
    }

    /// Read USBPHY_DM_EDGE_HIGH [7:7]
    #[inline(always)]
    pub const fn usbphy_dm_edge_high(&self) -> bool {
        (self.bits & Proc1Intf::USBPHY_DM_EDGE_HIGH_MASK) != 0
    }

    /// Read USBPHY_DM_EDGE_LOW [6:6]
    #[inline(always)]
    pub const fn usbphy_dm_edge_low(&self) -> bool {
        (self.bits & Proc1Intf::USBPHY_DM_EDGE_LOW_MASK) != 0
    }

    /// Read USBPHY_DM_LEVEL_HIGH [5:5]
    #[inline(always)]
    pub const fn usbphy_dm_level_high(&self) -> bool {
        (self.bits & Proc1Intf::USBPHY_DM_LEVEL_HIGH_MASK) != 0
    }

    /// Read USBPHY_DM_LEVEL_LOW [4:4]
    #[inline(always)]
    pub const fn usbphy_dm_level_low(&self) -> bool {
        (self.bits & Proc1Intf::USBPHY_DM_LEVEL_LOW_MASK) != 0
    }

    /// Read USBPHY_DP_EDGE_HIGH [3:3]
    #[inline(always)]
    pub const fn usbphy_dp_edge_high(&self) -> bool {
        (self.bits & Proc1Intf::USBPHY_DP_EDGE_HIGH_MASK) != 0
    }

    /// Read USBPHY_DP_EDGE_LOW [2:2]
    #[inline(always)]
    pub const fn usbphy_dp_edge_low(&self) -> bool {
        (self.bits & Proc1Intf::USBPHY_DP_EDGE_LOW_MASK) != 0
    }

    /// Read USBPHY_DP_LEVEL_HIGH [1:1]
    #[inline(always)]
    pub const fn usbphy_dp_level_high(&self) -> bool {
        (self.bits & Proc1Intf::USBPHY_DP_LEVEL_HIGH_MASK) != 0
    }

    /// Read USBPHY_DP_LEVEL_LOW [0:0]
    #[inline(always)]
    pub const fn usbphy_dp_level_low(&self) -> bool {
        (self.bits & Proc1Intf::USBPHY_DP_LEVEL_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SD3_EDGE_HIGH [31:31]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd3_edge_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc1Intf::GPIO_QSPI_SD3_EDGE_HIGH_MASK
            } else {
                self.bits & !Proc1Intf::GPIO_QSPI_SD3_EDGE_HIGH_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD3_EDGE_LOW [30:30]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd3_edge_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc1Intf::GPIO_QSPI_SD3_EDGE_LOW_MASK
            } else {
                self.bits & !Proc1Intf::GPIO_QSPI_SD3_EDGE_LOW_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD3_LEVEL_HIGH [29:29]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd3_level_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc1Intf::GPIO_QSPI_SD3_LEVEL_HIGH_MASK
            } else {
                self.bits & !Proc1Intf::GPIO_QSPI_SD3_LEVEL_HIGH_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD3_LEVEL_LOW [28:28]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd3_level_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc1Intf::GPIO_QSPI_SD3_LEVEL_LOW_MASK
            } else {
                self.bits & !Proc1Intf::GPIO_QSPI_SD3_LEVEL_LOW_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD2_EDGE_HIGH [27:27]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd2_edge_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc1Intf::GPIO_QSPI_SD2_EDGE_HIGH_MASK
            } else {
                self.bits & !Proc1Intf::GPIO_QSPI_SD2_EDGE_HIGH_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD2_EDGE_LOW [26:26]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd2_edge_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc1Intf::GPIO_QSPI_SD2_EDGE_LOW_MASK
            } else {
                self.bits & !Proc1Intf::GPIO_QSPI_SD2_EDGE_LOW_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD2_LEVEL_HIGH [25:25]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd2_level_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc1Intf::GPIO_QSPI_SD2_LEVEL_HIGH_MASK
            } else {
                self.bits & !Proc1Intf::GPIO_QSPI_SD2_LEVEL_HIGH_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD2_LEVEL_LOW [24:24]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd2_level_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc1Intf::GPIO_QSPI_SD2_LEVEL_LOW_MASK
            } else {
                self.bits & !Proc1Intf::GPIO_QSPI_SD2_LEVEL_LOW_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD1_EDGE_HIGH [23:23]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd1_edge_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc1Intf::GPIO_QSPI_SD1_EDGE_HIGH_MASK
            } else {
                self.bits & !Proc1Intf::GPIO_QSPI_SD1_EDGE_HIGH_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD1_EDGE_LOW [22:22]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd1_edge_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc1Intf::GPIO_QSPI_SD1_EDGE_LOW_MASK
            } else {
                self.bits & !Proc1Intf::GPIO_QSPI_SD1_EDGE_LOW_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD1_LEVEL_HIGH [21:21]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd1_level_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc1Intf::GPIO_QSPI_SD1_LEVEL_HIGH_MASK
            } else {
                self.bits & !Proc1Intf::GPIO_QSPI_SD1_LEVEL_HIGH_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD1_LEVEL_LOW [20:20]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd1_level_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc1Intf::GPIO_QSPI_SD1_LEVEL_LOW_MASK
            } else {
                self.bits & !Proc1Intf::GPIO_QSPI_SD1_LEVEL_LOW_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD0_EDGE_HIGH [19:19]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd0_edge_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc1Intf::GPIO_QSPI_SD0_EDGE_HIGH_MASK
            } else {
                self.bits & !Proc1Intf::GPIO_QSPI_SD0_EDGE_HIGH_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD0_EDGE_LOW [18:18]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd0_edge_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc1Intf::GPIO_QSPI_SD0_EDGE_LOW_MASK
            } else {
                self.bits & !Proc1Intf::GPIO_QSPI_SD0_EDGE_LOW_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD0_LEVEL_HIGH [17:17]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd0_level_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc1Intf::GPIO_QSPI_SD0_LEVEL_HIGH_MASK
            } else {
                self.bits & !Proc1Intf::GPIO_QSPI_SD0_LEVEL_HIGH_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD0_LEVEL_LOW [16:16]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd0_level_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc1Intf::GPIO_QSPI_SD0_LEVEL_LOW_MASK
            } else {
                self.bits & !Proc1Intf::GPIO_QSPI_SD0_LEVEL_LOW_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SS_EDGE_HIGH [15:15]
    #[inline(always)]
    pub const fn with_gpio_qspi_ss_edge_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc1Intf::GPIO_QSPI_SS_EDGE_HIGH_MASK
            } else {
                self.bits & !Proc1Intf::GPIO_QSPI_SS_EDGE_HIGH_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SS_EDGE_LOW [14:14]
    #[inline(always)]
    pub const fn with_gpio_qspi_ss_edge_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc1Intf::GPIO_QSPI_SS_EDGE_LOW_MASK
            } else {
                self.bits & !Proc1Intf::GPIO_QSPI_SS_EDGE_LOW_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SS_LEVEL_HIGH [13:13]
    #[inline(always)]
    pub const fn with_gpio_qspi_ss_level_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc1Intf::GPIO_QSPI_SS_LEVEL_HIGH_MASK
            } else {
                self.bits & !Proc1Intf::GPIO_QSPI_SS_LEVEL_HIGH_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SS_LEVEL_LOW [12:12]
    #[inline(always)]
    pub const fn with_gpio_qspi_ss_level_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc1Intf::GPIO_QSPI_SS_LEVEL_LOW_MASK
            } else {
                self.bits & !Proc1Intf::GPIO_QSPI_SS_LEVEL_LOW_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SCLK_EDGE_HIGH [11:11]
    #[inline(always)]
    pub const fn with_gpio_qspi_sclk_edge_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc1Intf::GPIO_QSPI_SCLK_EDGE_HIGH_MASK
            } else {
                self.bits & !Proc1Intf::GPIO_QSPI_SCLK_EDGE_HIGH_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SCLK_EDGE_LOW [10:10]
    #[inline(always)]
    pub const fn with_gpio_qspi_sclk_edge_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc1Intf::GPIO_QSPI_SCLK_EDGE_LOW_MASK
            } else {
                self.bits & !Proc1Intf::GPIO_QSPI_SCLK_EDGE_LOW_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SCLK_LEVEL_HIGH [9:9]
    #[inline(always)]
    pub const fn with_gpio_qspi_sclk_level_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc1Intf::GPIO_QSPI_SCLK_LEVEL_HIGH_MASK
            } else {
                self.bits & !Proc1Intf::GPIO_QSPI_SCLK_LEVEL_HIGH_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SCLK_LEVEL_LOW [8:8]
    #[inline(always)]
    pub const fn with_gpio_qspi_sclk_level_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc1Intf::GPIO_QSPI_SCLK_LEVEL_LOW_MASK
            } else {
                self.bits & !Proc1Intf::GPIO_QSPI_SCLK_LEVEL_LOW_MASK
            }
        }
    }

    /// Set USBPHY_DM_EDGE_HIGH [7:7]
    #[inline(always)]
    pub const fn with_usbphy_dm_edge_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc1Intf::USBPHY_DM_EDGE_HIGH_MASK
            } else {
                self.bits & !Proc1Intf::USBPHY_DM_EDGE_HIGH_MASK
            }
        }
    }

    /// Set USBPHY_DM_EDGE_LOW [6:6]
    #[inline(always)]
    pub const fn with_usbphy_dm_edge_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc1Intf::USBPHY_DM_EDGE_LOW_MASK
            } else {
                self.bits & !Proc1Intf::USBPHY_DM_EDGE_LOW_MASK
            }
        }
    }

    /// Set USBPHY_DM_LEVEL_HIGH [5:5]
    #[inline(always)]
    pub const fn with_usbphy_dm_level_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc1Intf::USBPHY_DM_LEVEL_HIGH_MASK
            } else {
                self.bits & !Proc1Intf::USBPHY_DM_LEVEL_HIGH_MASK
            }
        }
    }

    /// Set USBPHY_DM_LEVEL_LOW [4:4]
    #[inline(always)]
    pub const fn with_usbphy_dm_level_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc1Intf::USBPHY_DM_LEVEL_LOW_MASK
            } else {
                self.bits & !Proc1Intf::USBPHY_DM_LEVEL_LOW_MASK
            }
        }
    }

    /// Set USBPHY_DP_EDGE_HIGH [3:3]
    #[inline(always)]
    pub const fn with_usbphy_dp_edge_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc1Intf::USBPHY_DP_EDGE_HIGH_MASK
            } else {
                self.bits & !Proc1Intf::USBPHY_DP_EDGE_HIGH_MASK
            }
        }
    }

    /// Set USBPHY_DP_EDGE_LOW [2:2]
    #[inline(always)]
    pub const fn with_usbphy_dp_edge_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc1Intf::USBPHY_DP_EDGE_LOW_MASK
            } else {
                self.bits & !Proc1Intf::USBPHY_DP_EDGE_LOW_MASK
            }
        }
    }

    /// Set USBPHY_DP_LEVEL_HIGH [1:1]
    #[inline(always)]
    pub const fn with_usbphy_dp_level_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc1Intf::USBPHY_DP_LEVEL_HIGH_MASK
            } else {
                self.bits & !Proc1Intf::USBPHY_DP_LEVEL_HIGH_MASK
            }
        }
    }

    /// Set USBPHY_DP_LEVEL_LOW [0:0]
    #[inline(always)]
    pub const fn with_usbphy_dp_level_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Proc1Intf::USBPHY_DP_LEVEL_LOW_MASK
            } else {
                self.bits & !Proc1Intf::USBPHY_DP_LEVEL_LOW_MASK
            }
        }
    }

}

impl Default for Proc1IntfVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Proc1Intf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> Proc1IntfVal {
        Proc1IntfVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: Proc1IntfVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(Proc1IntfVal) -> Proc1IntfVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// PROC1_INTS Register
///
/// Interrupt status after masking & forcing for proc1
///
/// Address offset: 0x0230
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **GPIO_QSPI_SD3_EDGE_HIGH** [31] (RO): 
/// - **GPIO_QSPI_SD3_EDGE_LOW** [30] (RO): 
/// - **GPIO_QSPI_SD3_LEVEL_HIGH** [29] (RO): 
/// - **GPIO_QSPI_SD3_LEVEL_LOW** [28] (RO): 
/// - **GPIO_QSPI_SD2_EDGE_HIGH** [27] (RO): 
/// - **GPIO_QSPI_SD2_EDGE_LOW** [26] (RO): 
/// - **GPIO_QSPI_SD2_LEVEL_HIGH** [25] (RO): 
/// - **GPIO_QSPI_SD2_LEVEL_LOW** [24] (RO): 
/// - **GPIO_QSPI_SD1_EDGE_HIGH** [23] (RO): 
/// - **GPIO_QSPI_SD1_EDGE_LOW** [22] (RO): 
/// - **GPIO_QSPI_SD1_LEVEL_HIGH** [21] (RO): 
/// - **GPIO_QSPI_SD1_LEVEL_LOW** [20] (RO): 
/// - **GPIO_QSPI_SD0_EDGE_HIGH** [19] (RO): 
/// - **GPIO_QSPI_SD0_EDGE_LOW** [18] (RO): 
/// - **GPIO_QSPI_SD0_LEVEL_HIGH** [17] (RO): 
/// - **GPIO_QSPI_SD0_LEVEL_LOW** [16] (RO): 
/// - **GPIO_QSPI_SS_EDGE_HIGH** [15] (RO): 
/// - **GPIO_QSPI_SS_EDGE_LOW** [14] (RO): 
/// - **GPIO_QSPI_SS_LEVEL_HIGH** [13] (RO): 
/// - **GPIO_QSPI_SS_LEVEL_LOW** [12] (RO): 
/// - **GPIO_QSPI_SCLK_EDGE_HIGH** [11] (RO): 
/// - **GPIO_QSPI_SCLK_EDGE_LOW** [10] (RO): 
/// - **GPIO_QSPI_SCLK_LEVEL_HIGH** [9] (RO): 
/// - **GPIO_QSPI_SCLK_LEVEL_LOW** [8] (RO): 
/// - **USBPHY_DM_EDGE_HIGH** [7] (RO): 
/// - **USBPHY_DM_EDGE_LOW** [6] (RO): 
/// - **USBPHY_DM_LEVEL_HIGH** [5] (RO): 
/// - **USBPHY_DM_LEVEL_LOW** [4] (RO): 
/// - **USBPHY_DP_EDGE_HIGH** [3] (RO): 
/// - **USBPHY_DP_EDGE_LOW** [2] (RO): 
/// - **USBPHY_DP_LEVEL_HIGH** [1] (RO): 
/// - **USBPHY_DP_LEVEL_LOW** [0] (RO): 
#[repr(transparent)]
pub struct Proc1Ints {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for Proc1Ints {}
unsafe impl Sync for Proc1Ints {}

impl Proc1Ints {
    // ======== Field Constants ========

    /// Bits [31:31], mask 0x80000000
    pub const GPIO_QSPI_SD3_EDGE_HIGH_OFFSET: u32 = 31;
    pub const GPIO_QSPI_SD3_EDGE_HIGH_MASK: u32 = 0x80000000;
    pub const GPIO_QSPI_SD3_EDGE_HIGH: u32 = 1 << 31;

    /// Bits [30:30], mask 0x40000000
    pub const GPIO_QSPI_SD3_EDGE_LOW_OFFSET: u32 = 30;
    pub const GPIO_QSPI_SD3_EDGE_LOW_MASK: u32 = 0x40000000;
    pub const GPIO_QSPI_SD3_EDGE_LOW: u32 = 1 << 30;

    /// Bits [29:29], mask 0x20000000
    pub const GPIO_QSPI_SD3_LEVEL_HIGH_OFFSET: u32 = 29;
    pub const GPIO_QSPI_SD3_LEVEL_HIGH_MASK: u32 = 0x20000000;
    pub const GPIO_QSPI_SD3_LEVEL_HIGH: u32 = 1 << 29;

    /// Bits [28:28], mask 0x10000000
    pub const GPIO_QSPI_SD3_LEVEL_LOW_OFFSET: u32 = 28;
    pub const GPIO_QSPI_SD3_LEVEL_LOW_MASK: u32 = 0x10000000;
    pub const GPIO_QSPI_SD3_LEVEL_LOW: u32 = 1 << 28;

    /// Bits [27:27], mask 0x08000000
    pub const GPIO_QSPI_SD2_EDGE_HIGH_OFFSET: u32 = 27;
    pub const GPIO_QSPI_SD2_EDGE_HIGH_MASK: u32 = 0x08000000;
    pub const GPIO_QSPI_SD2_EDGE_HIGH: u32 = 1 << 27;

    /// Bits [26:26], mask 0x04000000
    pub const GPIO_QSPI_SD2_EDGE_LOW_OFFSET: u32 = 26;
    pub const GPIO_QSPI_SD2_EDGE_LOW_MASK: u32 = 0x04000000;
    pub const GPIO_QSPI_SD2_EDGE_LOW: u32 = 1 << 26;

    /// Bits [25:25], mask 0x02000000
    pub const GPIO_QSPI_SD2_LEVEL_HIGH_OFFSET: u32 = 25;
    pub const GPIO_QSPI_SD2_LEVEL_HIGH_MASK: u32 = 0x02000000;
    pub const GPIO_QSPI_SD2_LEVEL_HIGH: u32 = 1 << 25;

    /// Bits [24:24], mask 0x01000000
    pub const GPIO_QSPI_SD2_LEVEL_LOW_OFFSET: u32 = 24;
    pub const GPIO_QSPI_SD2_LEVEL_LOW_MASK: u32 = 0x01000000;
    pub const GPIO_QSPI_SD2_LEVEL_LOW: u32 = 1 << 24;

    /// Bits [23:23], mask 0x00800000
    pub const GPIO_QSPI_SD1_EDGE_HIGH_OFFSET: u32 = 23;
    pub const GPIO_QSPI_SD1_EDGE_HIGH_MASK: u32 = 0x00800000;
    pub const GPIO_QSPI_SD1_EDGE_HIGH: u32 = 1 << 23;

    /// Bits [22:22], mask 0x00400000
    pub const GPIO_QSPI_SD1_EDGE_LOW_OFFSET: u32 = 22;
    pub const GPIO_QSPI_SD1_EDGE_LOW_MASK: u32 = 0x00400000;
    pub const GPIO_QSPI_SD1_EDGE_LOW: u32 = 1 << 22;

    /// Bits [21:21], mask 0x00200000
    pub const GPIO_QSPI_SD1_LEVEL_HIGH_OFFSET: u32 = 21;
    pub const GPIO_QSPI_SD1_LEVEL_HIGH_MASK: u32 = 0x00200000;
    pub const GPIO_QSPI_SD1_LEVEL_HIGH: u32 = 1 << 21;

    /// Bits [20:20], mask 0x00100000
    pub const GPIO_QSPI_SD1_LEVEL_LOW_OFFSET: u32 = 20;
    pub const GPIO_QSPI_SD1_LEVEL_LOW_MASK: u32 = 0x00100000;
    pub const GPIO_QSPI_SD1_LEVEL_LOW: u32 = 1 << 20;

    /// Bits [19:19], mask 0x00080000
    pub const GPIO_QSPI_SD0_EDGE_HIGH_OFFSET: u32 = 19;
    pub const GPIO_QSPI_SD0_EDGE_HIGH_MASK: u32 = 0x00080000;
    pub const GPIO_QSPI_SD0_EDGE_HIGH: u32 = 1 << 19;

    /// Bits [18:18], mask 0x00040000
    pub const GPIO_QSPI_SD0_EDGE_LOW_OFFSET: u32 = 18;
    pub const GPIO_QSPI_SD0_EDGE_LOW_MASK: u32 = 0x00040000;
    pub const GPIO_QSPI_SD0_EDGE_LOW: u32 = 1 << 18;

    /// Bits [17:17], mask 0x00020000
    pub const GPIO_QSPI_SD0_LEVEL_HIGH_OFFSET: u32 = 17;
    pub const GPIO_QSPI_SD0_LEVEL_HIGH_MASK: u32 = 0x00020000;
    pub const GPIO_QSPI_SD0_LEVEL_HIGH: u32 = 1 << 17;

    /// Bits [16:16], mask 0x00010000
    pub const GPIO_QSPI_SD0_LEVEL_LOW_OFFSET: u32 = 16;
    pub const GPIO_QSPI_SD0_LEVEL_LOW_MASK: u32 = 0x00010000;
    pub const GPIO_QSPI_SD0_LEVEL_LOW: u32 = 1 << 16;

    /// Bits [15:15], mask 0x00008000
    pub const GPIO_QSPI_SS_EDGE_HIGH_OFFSET: u32 = 15;
    pub const GPIO_QSPI_SS_EDGE_HIGH_MASK: u32 = 0x00008000;
    pub const GPIO_QSPI_SS_EDGE_HIGH: u32 = 1 << 15;

    /// Bits [14:14], mask 0x00004000
    pub const GPIO_QSPI_SS_EDGE_LOW_OFFSET: u32 = 14;
    pub const GPIO_QSPI_SS_EDGE_LOW_MASK: u32 = 0x00004000;
    pub const GPIO_QSPI_SS_EDGE_LOW: u32 = 1 << 14;

    /// Bits [13:13], mask 0x00002000
    pub const GPIO_QSPI_SS_LEVEL_HIGH_OFFSET: u32 = 13;
    pub const GPIO_QSPI_SS_LEVEL_HIGH_MASK: u32 = 0x00002000;
    pub const GPIO_QSPI_SS_LEVEL_HIGH: u32 = 1 << 13;

    /// Bits [12:12], mask 0x00001000
    pub const GPIO_QSPI_SS_LEVEL_LOW_OFFSET: u32 = 12;
    pub const GPIO_QSPI_SS_LEVEL_LOW_MASK: u32 = 0x00001000;
    pub const GPIO_QSPI_SS_LEVEL_LOW: u32 = 1 << 12;

    /// Bits [11:11], mask 0x00000800
    pub const GPIO_QSPI_SCLK_EDGE_HIGH_OFFSET: u32 = 11;
    pub const GPIO_QSPI_SCLK_EDGE_HIGH_MASK: u32 = 0x00000800;
    pub const GPIO_QSPI_SCLK_EDGE_HIGH: u32 = 1 << 11;

    /// Bits [10:10], mask 0x00000400
    pub const GPIO_QSPI_SCLK_EDGE_LOW_OFFSET: u32 = 10;
    pub const GPIO_QSPI_SCLK_EDGE_LOW_MASK: u32 = 0x00000400;
    pub const GPIO_QSPI_SCLK_EDGE_LOW: u32 = 1 << 10;

    /// Bits [9:9], mask 0x00000200
    pub const GPIO_QSPI_SCLK_LEVEL_HIGH_OFFSET: u32 = 9;
    pub const GPIO_QSPI_SCLK_LEVEL_HIGH_MASK: u32 = 0x00000200;
    pub const GPIO_QSPI_SCLK_LEVEL_HIGH: u32 = 1 << 9;

    /// Bits [8:8], mask 0x00000100
    pub const GPIO_QSPI_SCLK_LEVEL_LOW_OFFSET: u32 = 8;
    pub const GPIO_QSPI_SCLK_LEVEL_LOW_MASK: u32 = 0x00000100;
    pub const GPIO_QSPI_SCLK_LEVEL_LOW: u32 = 1 << 8;

    /// Bits [7:7], mask 0x00000080
    pub const USBPHY_DM_EDGE_HIGH_OFFSET: u32 = 7;
    pub const USBPHY_DM_EDGE_HIGH_MASK: u32 = 0x00000080;
    pub const USBPHY_DM_EDGE_HIGH: u32 = 1 << 7;

    /// Bits [6:6], mask 0x00000040
    pub const USBPHY_DM_EDGE_LOW_OFFSET: u32 = 6;
    pub const USBPHY_DM_EDGE_LOW_MASK: u32 = 0x00000040;
    pub const USBPHY_DM_EDGE_LOW: u32 = 1 << 6;

    /// Bits [5:5], mask 0x00000020
    pub const USBPHY_DM_LEVEL_HIGH_OFFSET: u32 = 5;
    pub const USBPHY_DM_LEVEL_HIGH_MASK: u32 = 0x00000020;
    pub const USBPHY_DM_LEVEL_HIGH: u32 = 1 << 5;

    /// Bits [4:4], mask 0x00000010
    pub const USBPHY_DM_LEVEL_LOW_OFFSET: u32 = 4;
    pub const USBPHY_DM_LEVEL_LOW_MASK: u32 = 0x00000010;
    pub const USBPHY_DM_LEVEL_LOW: u32 = 1 << 4;

    /// Bits [3:3], mask 0x00000008
    pub const USBPHY_DP_EDGE_HIGH_OFFSET: u32 = 3;
    pub const USBPHY_DP_EDGE_HIGH_MASK: u32 = 0x00000008;
    pub const USBPHY_DP_EDGE_HIGH: u32 = 1 << 3;

    /// Bits [2:2], mask 0x00000004
    pub const USBPHY_DP_EDGE_LOW_OFFSET: u32 = 2;
    pub const USBPHY_DP_EDGE_LOW_MASK: u32 = 0x00000004;
    pub const USBPHY_DP_EDGE_LOW: u32 = 1 << 2;

    /// Bits [1:1], mask 0x00000002
    pub const USBPHY_DP_LEVEL_HIGH_OFFSET: u32 = 1;
    pub const USBPHY_DP_LEVEL_HIGH_MASK: u32 = 0x00000002;
    pub const USBPHY_DP_LEVEL_HIGH: u32 = 1 << 1;

    /// Bits [0:0], mask 0x00000001
    pub const USBPHY_DP_LEVEL_LOW_OFFSET: u32 = 0;
    pub const USBPHY_DP_LEVEL_LOW_MASK: u32 = 0x00000001;
    pub const USBPHY_DP_LEVEL_LOW: u32 = 1 << 0;

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

    /// **GPIO_QSPI_SD3_EDGE_HIGH** [31:31]
    #[inline(always)]
    pub fn gpio_qspi_sd3_edge_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD3_EDGE_HIGH_MASK) != 0
    }

    /// **GPIO_QSPI_SD3_EDGE_LOW** [30:30]
    #[inline(always)]
    pub fn gpio_qspi_sd3_edge_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD3_EDGE_LOW_MASK) != 0
    }

    /// **GPIO_QSPI_SD3_LEVEL_HIGH** [29:29]
    #[inline(always)]
    pub fn gpio_qspi_sd3_level_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD3_LEVEL_HIGH_MASK) != 0
    }

    /// **GPIO_QSPI_SD3_LEVEL_LOW** [28:28]
    #[inline(always)]
    pub fn gpio_qspi_sd3_level_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD3_LEVEL_LOW_MASK) != 0
    }

    /// **GPIO_QSPI_SD2_EDGE_HIGH** [27:27]
    #[inline(always)]
    pub fn gpio_qspi_sd2_edge_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD2_EDGE_HIGH_MASK) != 0
    }

    /// **GPIO_QSPI_SD2_EDGE_LOW** [26:26]
    #[inline(always)]
    pub fn gpio_qspi_sd2_edge_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD2_EDGE_LOW_MASK) != 0
    }

    /// **GPIO_QSPI_SD2_LEVEL_HIGH** [25:25]
    #[inline(always)]
    pub fn gpio_qspi_sd2_level_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD2_LEVEL_HIGH_MASK) != 0
    }

    /// **GPIO_QSPI_SD2_LEVEL_LOW** [24:24]
    #[inline(always)]
    pub fn gpio_qspi_sd2_level_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD2_LEVEL_LOW_MASK) != 0
    }

    /// **GPIO_QSPI_SD1_EDGE_HIGH** [23:23]
    #[inline(always)]
    pub fn gpio_qspi_sd1_edge_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD1_EDGE_HIGH_MASK) != 0
    }

    /// **GPIO_QSPI_SD1_EDGE_LOW** [22:22]
    #[inline(always)]
    pub fn gpio_qspi_sd1_edge_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD1_EDGE_LOW_MASK) != 0
    }

    /// **GPIO_QSPI_SD1_LEVEL_HIGH** [21:21]
    #[inline(always)]
    pub fn gpio_qspi_sd1_level_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD1_LEVEL_HIGH_MASK) != 0
    }

    /// **GPIO_QSPI_SD1_LEVEL_LOW** [20:20]
    #[inline(always)]
    pub fn gpio_qspi_sd1_level_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD1_LEVEL_LOW_MASK) != 0
    }

    /// **GPIO_QSPI_SD0_EDGE_HIGH** [19:19]
    #[inline(always)]
    pub fn gpio_qspi_sd0_edge_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD0_EDGE_HIGH_MASK) != 0
    }

    /// **GPIO_QSPI_SD0_EDGE_LOW** [18:18]
    #[inline(always)]
    pub fn gpio_qspi_sd0_edge_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD0_EDGE_LOW_MASK) != 0
    }

    /// **GPIO_QSPI_SD0_LEVEL_HIGH** [17:17]
    #[inline(always)]
    pub fn gpio_qspi_sd0_level_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD0_LEVEL_HIGH_MASK) != 0
    }

    /// **GPIO_QSPI_SD0_LEVEL_LOW** [16:16]
    #[inline(always)]
    pub fn gpio_qspi_sd0_level_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD0_LEVEL_LOW_MASK) != 0
    }

    /// **GPIO_QSPI_SS_EDGE_HIGH** [15:15]
    #[inline(always)]
    pub fn gpio_qspi_ss_edge_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SS_EDGE_HIGH_MASK) != 0
    }

    /// **GPIO_QSPI_SS_EDGE_LOW** [14:14]
    #[inline(always)]
    pub fn gpio_qspi_ss_edge_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SS_EDGE_LOW_MASK) != 0
    }

    /// **GPIO_QSPI_SS_LEVEL_HIGH** [13:13]
    #[inline(always)]
    pub fn gpio_qspi_ss_level_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SS_LEVEL_HIGH_MASK) != 0
    }

    /// **GPIO_QSPI_SS_LEVEL_LOW** [12:12]
    #[inline(always)]
    pub fn gpio_qspi_ss_level_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SS_LEVEL_LOW_MASK) != 0
    }

    /// **GPIO_QSPI_SCLK_EDGE_HIGH** [11:11]
    #[inline(always)]
    pub fn gpio_qspi_sclk_edge_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SCLK_EDGE_HIGH_MASK) != 0
    }

    /// **GPIO_QSPI_SCLK_EDGE_LOW** [10:10]
    #[inline(always)]
    pub fn gpio_qspi_sclk_edge_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SCLK_EDGE_LOW_MASK) != 0
    }

    /// **GPIO_QSPI_SCLK_LEVEL_HIGH** [9:9]
    #[inline(always)]
    pub fn gpio_qspi_sclk_level_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SCLK_LEVEL_HIGH_MASK) != 0
    }

    /// **GPIO_QSPI_SCLK_LEVEL_LOW** [8:8]
    #[inline(always)]
    pub fn gpio_qspi_sclk_level_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SCLK_LEVEL_LOW_MASK) != 0
    }

    /// **USBPHY_DM_EDGE_HIGH** [7:7]
    #[inline(always)]
    pub fn usbphy_dm_edge_high(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DM_EDGE_HIGH_MASK) != 0
    }

    /// **USBPHY_DM_EDGE_LOW** [6:6]
    #[inline(always)]
    pub fn usbphy_dm_edge_low(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DM_EDGE_LOW_MASK) != 0
    }

    /// **USBPHY_DM_LEVEL_HIGH** [5:5]
    #[inline(always)]
    pub fn usbphy_dm_level_high(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DM_LEVEL_HIGH_MASK) != 0
    }

    /// **USBPHY_DM_LEVEL_LOW** [4:4]
    #[inline(always)]
    pub fn usbphy_dm_level_low(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DM_LEVEL_LOW_MASK) != 0
    }

    /// **USBPHY_DP_EDGE_HIGH** [3:3]
    #[inline(always)]
    pub fn usbphy_dp_edge_high(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DP_EDGE_HIGH_MASK) != 0
    }

    /// **USBPHY_DP_EDGE_LOW** [2:2]
    #[inline(always)]
    pub fn usbphy_dp_edge_low(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DP_EDGE_LOW_MASK) != 0
    }

    /// **USBPHY_DP_LEVEL_HIGH** [1:1]
    #[inline(always)]
    pub fn usbphy_dp_level_high(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DP_LEVEL_HIGH_MASK) != 0
    }

    /// **USBPHY_DP_LEVEL_LOW** [0:0]
    #[inline(always)]
    pub fn usbphy_dp_level_low(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DP_LEVEL_LOW_MASK) != 0
    }

}

/// PROC1_INTS value type for batch read/modify/write operations
///
/// Use `proc1ints.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `proc1ints.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Proc1IntsVal {
    bits: u32,
}

impl Proc1IntsVal {
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

    /// Read GPIO_QSPI_SD3_EDGE_HIGH [31:31]
    #[inline(always)]
    pub const fn gpio_qspi_sd3_edge_high(&self) -> bool {
        (self.bits & Proc1Ints::GPIO_QSPI_SD3_EDGE_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD3_EDGE_LOW [30:30]
    #[inline(always)]
    pub const fn gpio_qspi_sd3_edge_low(&self) -> bool {
        (self.bits & Proc1Ints::GPIO_QSPI_SD3_EDGE_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SD3_LEVEL_HIGH [29:29]
    #[inline(always)]
    pub const fn gpio_qspi_sd3_level_high(&self) -> bool {
        (self.bits & Proc1Ints::GPIO_QSPI_SD3_LEVEL_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD3_LEVEL_LOW [28:28]
    #[inline(always)]
    pub const fn gpio_qspi_sd3_level_low(&self) -> bool {
        (self.bits & Proc1Ints::GPIO_QSPI_SD3_LEVEL_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SD2_EDGE_HIGH [27:27]
    #[inline(always)]
    pub const fn gpio_qspi_sd2_edge_high(&self) -> bool {
        (self.bits & Proc1Ints::GPIO_QSPI_SD2_EDGE_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD2_EDGE_LOW [26:26]
    #[inline(always)]
    pub const fn gpio_qspi_sd2_edge_low(&self) -> bool {
        (self.bits & Proc1Ints::GPIO_QSPI_SD2_EDGE_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SD2_LEVEL_HIGH [25:25]
    #[inline(always)]
    pub const fn gpio_qspi_sd2_level_high(&self) -> bool {
        (self.bits & Proc1Ints::GPIO_QSPI_SD2_LEVEL_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD2_LEVEL_LOW [24:24]
    #[inline(always)]
    pub const fn gpio_qspi_sd2_level_low(&self) -> bool {
        (self.bits & Proc1Ints::GPIO_QSPI_SD2_LEVEL_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SD1_EDGE_HIGH [23:23]
    #[inline(always)]
    pub const fn gpio_qspi_sd1_edge_high(&self) -> bool {
        (self.bits & Proc1Ints::GPIO_QSPI_SD1_EDGE_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD1_EDGE_LOW [22:22]
    #[inline(always)]
    pub const fn gpio_qspi_sd1_edge_low(&self) -> bool {
        (self.bits & Proc1Ints::GPIO_QSPI_SD1_EDGE_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SD1_LEVEL_HIGH [21:21]
    #[inline(always)]
    pub const fn gpio_qspi_sd1_level_high(&self) -> bool {
        (self.bits & Proc1Ints::GPIO_QSPI_SD1_LEVEL_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD1_LEVEL_LOW [20:20]
    #[inline(always)]
    pub const fn gpio_qspi_sd1_level_low(&self) -> bool {
        (self.bits & Proc1Ints::GPIO_QSPI_SD1_LEVEL_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SD0_EDGE_HIGH [19:19]
    #[inline(always)]
    pub const fn gpio_qspi_sd0_edge_high(&self) -> bool {
        (self.bits & Proc1Ints::GPIO_QSPI_SD0_EDGE_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD0_EDGE_LOW [18:18]
    #[inline(always)]
    pub const fn gpio_qspi_sd0_edge_low(&self) -> bool {
        (self.bits & Proc1Ints::GPIO_QSPI_SD0_EDGE_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SD0_LEVEL_HIGH [17:17]
    #[inline(always)]
    pub const fn gpio_qspi_sd0_level_high(&self) -> bool {
        (self.bits & Proc1Ints::GPIO_QSPI_SD0_LEVEL_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD0_LEVEL_LOW [16:16]
    #[inline(always)]
    pub const fn gpio_qspi_sd0_level_low(&self) -> bool {
        (self.bits & Proc1Ints::GPIO_QSPI_SD0_LEVEL_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SS_EDGE_HIGH [15:15]
    #[inline(always)]
    pub const fn gpio_qspi_ss_edge_high(&self) -> bool {
        (self.bits & Proc1Ints::GPIO_QSPI_SS_EDGE_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SS_EDGE_LOW [14:14]
    #[inline(always)]
    pub const fn gpio_qspi_ss_edge_low(&self) -> bool {
        (self.bits & Proc1Ints::GPIO_QSPI_SS_EDGE_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SS_LEVEL_HIGH [13:13]
    #[inline(always)]
    pub const fn gpio_qspi_ss_level_high(&self) -> bool {
        (self.bits & Proc1Ints::GPIO_QSPI_SS_LEVEL_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SS_LEVEL_LOW [12:12]
    #[inline(always)]
    pub const fn gpio_qspi_ss_level_low(&self) -> bool {
        (self.bits & Proc1Ints::GPIO_QSPI_SS_LEVEL_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SCLK_EDGE_HIGH [11:11]
    #[inline(always)]
    pub const fn gpio_qspi_sclk_edge_high(&self) -> bool {
        (self.bits & Proc1Ints::GPIO_QSPI_SCLK_EDGE_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SCLK_EDGE_LOW [10:10]
    #[inline(always)]
    pub const fn gpio_qspi_sclk_edge_low(&self) -> bool {
        (self.bits & Proc1Ints::GPIO_QSPI_SCLK_EDGE_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SCLK_LEVEL_HIGH [9:9]
    #[inline(always)]
    pub const fn gpio_qspi_sclk_level_high(&self) -> bool {
        (self.bits & Proc1Ints::GPIO_QSPI_SCLK_LEVEL_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SCLK_LEVEL_LOW [8:8]
    #[inline(always)]
    pub const fn gpio_qspi_sclk_level_low(&self) -> bool {
        (self.bits & Proc1Ints::GPIO_QSPI_SCLK_LEVEL_LOW_MASK) != 0
    }

    /// Read USBPHY_DM_EDGE_HIGH [7:7]
    #[inline(always)]
    pub const fn usbphy_dm_edge_high(&self) -> bool {
        (self.bits & Proc1Ints::USBPHY_DM_EDGE_HIGH_MASK) != 0
    }

    /// Read USBPHY_DM_EDGE_LOW [6:6]
    #[inline(always)]
    pub const fn usbphy_dm_edge_low(&self) -> bool {
        (self.bits & Proc1Ints::USBPHY_DM_EDGE_LOW_MASK) != 0
    }

    /// Read USBPHY_DM_LEVEL_HIGH [5:5]
    #[inline(always)]
    pub const fn usbphy_dm_level_high(&self) -> bool {
        (self.bits & Proc1Ints::USBPHY_DM_LEVEL_HIGH_MASK) != 0
    }

    /// Read USBPHY_DM_LEVEL_LOW [4:4]
    #[inline(always)]
    pub const fn usbphy_dm_level_low(&self) -> bool {
        (self.bits & Proc1Ints::USBPHY_DM_LEVEL_LOW_MASK) != 0
    }

    /// Read USBPHY_DP_EDGE_HIGH [3:3]
    #[inline(always)]
    pub const fn usbphy_dp_edge_high(&self) -> bool {
        (self.bits & Proc1Ints::USBPHY_DP_EDGE_HIGH_MASK) != 0
    }

    /// Read USBPHY_DP_EDGE_LOW [2:2]
    #[inline(always)]
    pub const fn usbphy_dp_edge_low(&self) -> bool {
        (self.bits & Proc1Ints::USBPHY_DP_EDGE_LOW_MASK) != 0
    }

    /// Read USBPHY_DP_LEVEL_HIGH [1:1]
    #[inline(always)]
    pub const fn usbphy_dp_level_high(&self) -> bool {
        (self.bits & Proc1Ints::USBPHY_DP_LEVEL_HIGH_MASK) != 0
    }

    /// Read USBPHY_DP_LEVEL_LOW [0:0]
    #[inline(always)]
    pub const fn usbphy_dp_level_low(&self) -> bool {
        (self.bits & Proc1Ints::USBPHY_DP_LEVEL_LOW_MASK) != 0
    }

}

impl Default for Proc1IntsVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Proc1Ints {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> Proc1IntsVal {
        Proc1IntsVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: Proc1IntsVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(Proc1IntsVal) -> Proc1IntsVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// DORMANT_WAKE_INTE Register
///
/// Interrupt Enable for dormant_wake
///
/// Address offset: 0x0234
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **GPIO_QSPI_SD3_EDGE_HIGH** [31] (RW): 
/// - **GPIO_QSPI_SD3_EDGE_LOW** [30] (RW): 
/// - **GPIO_QSPI_SD3_LEVEL_HIGH** [29] (RW): 
/// - **GPIO_QSPI_SD3_LEVEL_LOW** [28] (RW): 
/// - **GPIO_QSPI_SD2_EDGE_HIGH** [27] (RW): 
/// - **GPIO_QSPI_SD2_EDGE_LOW** [26] (RW): 
/// - **GPIO_QSPI_SD2_LEVEL_HIGH** [25] (RW): 
/// - **GPIO_QSPI_SD2_LEVEL_LOW** [24] (RW): 
/// - **GPIO_QSPI_SD1_EDGE_HIGH** [23] (RW): 
/// - **GPIO_QSPI_SD1_EDGE_LOW** [22] (RW): 
/// - **GPIO_QSPI_SD1_LEVEL_HIGH** [21] (RW): 
/// - **GPIO_QSPI_SD1_LEVEL_LOW** [20] (RW): 
/// - **GPIO_QSPI_SD0_EDGE_HIGH** [19] (RW): 
/// - **GPIO_QSPI_SD0_EDGE_LOW** [18] (RW): 
/// - **GPIO_QSPI_SD0_LEVEL_HIGH** [17] (RW): 
/// - **GPIO_QSPI_SD0_LEVEL_LOW** [16] (RW): 
/// - **GPIO_QSPI_SS_EDGE_HIGH** [15] (RW): 
/// - **GPIO_QSPI_SS_EDGE_LOW** [14] (RW): 
/// - **GPIO_QSPI_SS_LEVEL_HIGH** [13] (RW): 
/// - **GPIO_QSPI_SS_LEVEL_LOW** [12] (RW): 
/// - **GPIO_QSPI_SCLK_EDGE_HIGH** [11] (RW): 
/// - **GPIO_QSPI_SCLK_EDGE_LOW** [10] (RW): 
/// - **GPIO_QSPI_SCLK_LEVEL_HIGH** [9] (RW): 
/// - **GPIO_QSPI_SCLK_LEVEL_LOW** [8] (RW): 
/// - **USBPHY_DM_EDGE_HIGH** [7] (RW): 
/// - **USBPHY_DM_EDGE_LOW** [6] (RW): 
/// - **USBPHY_DM_LEVEL_HIGH** [5] (RW): 
/// - **USBPHY_DM_LEVEL_LOW** [4] (RW): 
/// - **USBPHY_DP_EDGE_HIGH** [3] (RW): 
/// - **USBPHY_DP_EDGE_LOW** [2] (RW): 
/// - **USBPHY_DP_LEVEL_HIGH** [1] (RW): 
/// - **USBPHY_DP_LEVEL_LOW** [0] (RW): 
#[repr(transparent)]
pub struct DormantWakeInte {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for DormantWakeInte {}
unsafe impl Sync for DormantWakeInte {}

impl DormantWakeInte {
    // ======== Field Constants ========

    /// Bits [31:31], mask 0x80000000
    pub const GPIO_QSPI_SD3_EDGE_HIGH_OFFSET: u32 = 31;
    pub const GPIO_QSPI_SD3_EDGE_HIGH_MASK: u32 = 0x80000000;
    pub const GPIO_QSPI_SD3_EDGE_HIGH: u32 = 1 << 31;

    /// Bits [30:30], mask 0x40000000
    pub const GPIO_QSPI_SD3_EDGE_LOW_OFFSET: u32 = 30;
    pub const GPIO_QSPI_SD3_EDGE_LOW_MASK: u32 = 0x40000000;
    pub const GPIO_QSPI_SD3_EDGE_LOW: u32 = 1 << 30;

    /// Bits [29:29], mask 0x20000000
    pub const GPIO_QSPI_SD3_LEVEL_HIGH_OFFSET: u32 = 29;
    pub const GPIO_QSPI_SD3_LEVEL_HIGH_MASK: u32 = 0x20000000;
    pub const GPIO_QSPI_SD3_LEVEL_HIGH: u32 = 1 << 29;

    /// Bits [28:28], mask 0x10000000
    pub const GPIO_QSPI_SD3_LEVEL_LOW_OFFSET: u32 = 28;
    pub const GPIO_QSPI_SD3_LEVEL_LOW_MASK: u32 = 0x10000000;
    pub const GPIO_QSPI_SD3_LEVEL_LOW: u32 = 1 << 28;

    /// Bits [27:27], mask 0x08000000
    pub const GPIO_QSPI_SD2_EDGE_HIGH_OFFSET: u32 = 27;
    pub const GPIO_QSPI_SD2_EDGE_HIGH_MASK: u32 = 0x08000000;
    pub const GPIO_QSPI_SD2_EDGE_HIGH: u32 = 1 << 27;

    /// Bits [26:26], mask 0x04000000
    pub const GPIO_QSPI_SD2_EDGE_LOW_OFFSET: u32 = 26;
    pub const GPIO_QSPI_SD2_EDGE_LOW_MASK: u32 = 0x04000000;
    pub const GPIO_QSPI_SD2_EDGE_LOW: u32 = 1 << 26;

    /// Bits [25:25], mask 0x02000000
    pub const GPIO_QSPI_SD2_LEVEL_HIGH_OFFSET: u32 = 25;
    pub const GPIO_QSPI_SD2_LEVEL_HIGH_MASK: u32 = 0x02000000;
    pub const GPIO_QSPI_SD2_LEVEL_HIGH: u32 = 1 << 25;

    /// Bits [24:24], mask 0x01000000
    pub const GPIO_QSPI_SD2_LEVEL_LOW_OFFSET: u32 = 24;
    pub const GPIO_QSPI_SD2_LEVEL_LOW_MASK: u32 = 0x01000000;
    pub const GPIO_QSPI_SD2_LEVEL_LOW: u32 = 1 << 24;

    /// Bits [23:23], mask 0x00800000
    pub const GPIO_QSPI_SD1_EDGE_HIGH_OFFSET: u32 = 23;
    pub const GPIO_QSPI_SD1_EDGE_HIGH_MASK: u32 = 0x00800000;
    pub const GPIO_QSPI_SD1_EDGE_HIGH: u32 = 1 << 23;

    /// Bits [22:22], mask 0x00400000
    pub const GPIO_QSPI_SD1_EDGE_LOW_OFFSET: u32 = 22;
    pub const GPIO_QSPI_SD1_EDGE_LOW_MASK: u32 = 0x00400000;
    pub const GPIO_QSPI_SD1_EDGE_LOW: u32 = 1 << 22;

    /// Bits [21:21], mask 0x00200000
    pub const GPIO_QSPI_SD1_LEVEL_HIGH_OFFSET: u32 = 21;
    pub const GPIO_QSPI_SD1_LEVEL_HIGH_MASK: u32 = 0x00200000;
    pub const GPIO_QSPI_SD1_LEVEL_HIGH: u32 = 1 << 21;

    /// Bits [20:20], mask 0x00100000
    pub const GPIO_QSPI_SD1_LEVEL_LOW_OFFSET: u32 = 20;
    pub const GPIO_QSPI_SD1_LEVEL_LOW_MASK: u32 = 0x00100000;
    pub const GPIO_QSPI_SD1_LEVEL_LOW: u32 = 1 << 20;

    /// Bits [19:19], mask 0x00080000
    pub const GPIO_QSPI_SD0_EDGE_HIGH_OFFSET: u32 = 19;
    pub const GPIO_QSPI_SD0_EDGE_HIGH_MASK: u32 = 0x00080000;
    pub const GPIO_QSPI_SD0_EDGE_HIGH: u32 = 1 << 19;

    /// Bits [18:18], mask 0x00040000
    pub const GPIO_QSPI_SD0_EDGE_LOW_OFFSET: u32 = 18;
    pub const GPIO_QSPI_SD0_EDGE_LOW_MASK: u32 = 0x00040000;
    pub const GPIO_QSPI_SD0_EDGE_LOW: u32 = 1 << 18;

    /// Bits [17:17], mask 0x00020000
    pub const GPIO_QSPI_SD0_LEVEL_HIGH_OFFSET: u32 = 17;
    pub const GPIO_QSPI_SD0_LEVEL_HIGH_MASK: u32 = 0x00020000;
    pub const GPIO_QSPI_SD0_LEVEL_HIGH: u32 = 1 << 17;

    /// Bits [16:16], mask 0x00010000
    pub const GPIO_QSPI_SD0_LEVEL_LOW_OFFSET: u32 = 16;
    pub const GPIO_QSPI_SD0_LEVEL_LOW_MASK: u32 = 0x00010000;
    pub const GPIO_QSPI_SD0_LEVEL_LOW: u32 = 1 << 16;

    /// Bits [15:15], mask 0x00008000
    pub const GPIO_QSPI_SS_EDGE_HIGH_OFFSET: u32 = 15;
    pub const GPIO_QSPI_SS_EDGE_HIGH_MASK: u32 = 0x00008000;
    pub const GPIO_QSPI_SS_EDGE_HIGH: u32 = 1 << 15;

    /// Bits [14:14], mask 0x00004000
    pub const GPIO_QSPI_SS_EDGE_LOW_OFFSET: u32 = 14;
    pub const GPIO_QSPI_SS_EDGE_LOW_MASK: u32 = 0x00004000;
    pub const GPIO_QSPI_SS_EDGE_LOW: u32 = 1 << 14;

    /// Bits [13:13], mask 0x00002000
    pub const GPIO_QSPI_SS_LEVEL_HIGH_OFFSET: u32 = 13;
    pub const GPIO_QSPI_SS_LEVEL_HIGH_MASK: u32 = 0x00002000;
    pub const GPIO_QSPI_SS_LEVEL_HIGH: u32 = 1 << 13;

    /// Bits [12:12], mask 0x00001000
    pub const GPIO_QSPI_SS_LEVEL_LOW_OFFSET: u32 = 12;
    pub const GPIO_QSPI_SS_LEVEL_LOW_MASK: u32 = 0x00001000;
    pub const GPIO_QSPI_SS_LEVEL_LOW: u32 = 1 << 12;

    /// Bits [11:11], mask 0x00000800
    pub const GPIO_QSPI_SCLK_EDGE_HIGH_OFFSET: u32 = 11;
    pub const GPIO_QSPI_SCLK_EDGE_HIGH_MASK: u32 = 0x00000800;
    pub const GPIO_QSPI_SCLK_EDGE_HIGH: u32 = 1 << 11;

    /// Bits [10:10], mask 0x00000400
    pub const GPIO_QSPI_SCLK_EDGE_LOW_OFFSET: u32 = 10;
    pub const GPIO_QSPI_SCLK_EDGE_LOW_MASK: u32 = 0x00000400;
    pub const GPIO_QSPI_SCLK_EDGE_LOW: u32 = 1 << 10;

    /// Bits [9:9], mask 0x00000200
    pub const GPIO_QSPI_SCLK_LEVEL_HIGH_OFFSET: u32 = 9;
    pub const GPIO_QSPI_SCLK_LEVEL_HIGH_MASK: u32 = 0x00000200;
    pub const GPIO_QSPI_SCLK_LEVEL_HIGH: u32 = 1 << 9;

    /// Bits [8:8], mask 0x00000100
    pub const GPIO_QSPI_SCLK_LEVEL_LOW_OFFSET: u32 = 8;
    pub const GPIO_QSPI_SCLK_LEVEL_LOW_MASK: u32 = 0x00000100;
    pub const GPIO_QSPI_SCLK_LEVEL_LOW: u32 = 1 << 8;

    /// Bits [7:7], mask 0x00000080
    pub const USBPHY_DM_EDGE_HIGH_OFFSET: u32 = 7;
    pub const USBPHY_DM_EDGE_HIGH_MASK: u32 = 0x00000080;
    pub const USBPHY_DM_EDGE_HIGH: u32 = 1 << 7;

    /// Bits [6:6], mask 0x00000040
    pub const USBPHY_DM_EDGE_LOW_OFFSET: u32 = 6;
    pub const USBPHY_DM_EDGE_LOW_MASK: u32 = 0x00000040;
    pub const USBPHY_DM_EDGE_LOW: u32 = 1 << 6;

    /// Bits [5:5], mask 0x00000020
    pub const USBPHY_DM_LEVEL_HIGH_OFFSET: u32 = 5;
    pub const USBPHY_DM_LEVEL_HIGH_MASK: u32 = 0x00000020;
    pub const USBPHY_DM_LEVEL_HIGH: u32 = 1 << 5;

    /// Bits [4:4], mask 0x00000010
    pub const USBPHY_DM_LEVEL_LOW_OFFSET: u32 = 4;
    pub const USBPHY_DM_LEVEL_LOW_MASK: u32 = 0x00000010;
    pub const USBPHY_DM_LEVEL_LOW: u32 = 1 << 4;

    /// Bits [3:3], mask 0x00000008
    pub const USBPHY_DP_EDGE_HIGH_OFFSET: u32 = 3;
    pub const USBPHY_DP_EDGE_HIGH_MASK: u32 = 0x00000008;
    pub const USBPHY_DP_EDGE_HIGH: u32 = 1 << 3;

    /// Bits [2:2], mask 0x00000004
    pub const USBPHY_DP_EDGE_LOW_OFFSET: u32 = 2;
    pub const USBPHY_DP_EDGE_LOW_MASK: u32 = 0x00000004;
    pub const USBPHY_DP_EDGE_LOW: u32 = 1 << 2;

    /// Bits [1:1], mask 0x00000002
    pub const USBPHY_DP_LEVEL_HIGH_OFFSET: u32 = 1;
    pub const USBPHY_DP_LEVEL_HIGH_MASK: u32 = 0x00000002;
    pub const USBPHY_DP_LEVEL_HIGH: u32 = 1 << 1;

    /// Bits [0:0], mask 0x00000001
    pub const USBPHY_DP_LEVEL_LOW_OFFSET: u32 = 0;
    pub const USBPHY_DP_LEVEL_LOW_MASK: u32 = 0x00000001;
    pub const USBPHY_DP_LEVEL_LOW: u32 = 1 << 0;

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

    /// **GPIO_QSPI_SD3_EDGE_HIGH** [31:31]
    #[inline(always)]
    pub fn gpio_qspi_sd3_edge_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD3_EDGE_HIGH_MASK) != 0
    }

    /// Set GPIO_QSPI_SD3_EDGE_HIGH bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd3_edge_high(&self) {
        self.set_bits(Self::GPIO_QSPI_SD3_EDGE_HIGH_MASK);
    }

    /// Clear GPIO_QSPI_SD3_EDGE_HIGH bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd3_edge_high(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD3_EDGE_HIGH_MASK);
    }

    /// **GPIO_QSPI_SD3_EDGE_LOW** [30:30]
    #[inline(always)]
    pub fn gpio_qspi_sd3_edge_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD3_EDGE_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SD3_EDGE_LOW bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd3_edge_low(&self) {
        self.set_bits(Self::GPIO_QSPI_SD3_EDGE_LOW_MASK);
    }

    /// Clear GPIO_QSPI_SD3_EDGE_LOW bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd3_edge_low(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD3_EDGE_LOW_MASK);
    }

    /// **GPIO_QSPI_SD3_LEVEL_HIGH** [29:29]
    #[inline(always)]
    pub fn gpio_qspi_sd3_level_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD3_LEVEL_HIGH_MASK) != 0
    }

    /// Set GPIO_QSPI_SD3_LEVEL_HIGH bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd3_level_high(&self) {
        self.set_bits(Self::GPIO_QSPI_SD3_LEVEL_HIGH_MASK);
    }

    /// Clear GPIO_QSPI_SD3_LEVEL_HIGH bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd3_level_high(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD3_LEVEL_HIGH_MASK);
    }

    /// **GPIO_QSPI_SD3_LEVEL_LOW** [28:28]
    #[inline(always)]
    pub fn gpio_qspi_sd3_level_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD3_LEVEL_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SD3_LEVEL_LOW bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd3_level_low(&self) {
        self.set_bits(Self::GPIO_QSPI_SD3_LEVEL_LOW_MASK);
    }

    /// Clear GPIO_QSPI_SD3_LEVEL_LOW bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd3_level_low(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD3_LEVEL_LOW_MASK);
    }

    /// **GPIO_QSPI_SD2_EDGE_HIGH** [27:27]
    #[inline(always)]
    pub fn gpio_qspi_sd2_edge_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD2_EDGE_HIGH_MASK) != 0
    }

    /// Set GPIO_QSPI_SD2_EDGE_HIGH bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd2_edge_high(&self) {
        self.set_bits(Self::GPIO_QSPI_SD2_EDGE_HIGH_MASK);
    }

    /// Clear GPIO_QSPI_SD2_EDGE_HIGH bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd2_edge_high(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD2_EDGE_HIGH_MASK);
    }

    /// **GPIO_QSPI_SD2_EDGE_LOW** [26:26]
    #[inline(always)]
    pub fn gpio_qspi_sd2_edge_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD2_EDGE_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SD2_EDGE_LOW bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd2_edge_low(&self) {
        self.set_bits(Self::GPIO_QSPI_SD2_EDGE_LOW_MASK);
    }

    /// Clear GPIO_QSPI_SD2_EDGE_LOW bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd2_edge_low(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD2_EDGE_LOW_MASK);
    }

    /// **GPIO_QSPI_SD2_LEVEL_HIGH** [25:25]
    #[inline(always)]
    pub fn gpio_qspi_sd2_level_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD2_LEVEL_HIGH_MASK) != 0
    }

    /// Set GPIO_QSPI_SD2_LEVEL_HIGH bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd2_level_high(&self) {
        self.set_bits(Self::GPIO_QSPI_SD2_LEVEL_HIGH_MASK);
    }

    /// Clear GPIO_QSPI_SD2_LEVEL_HIGH bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd2_level_high(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD2_LEVEL_HIGH_MASK);
    }

    /// **GPIO_QSPI_SD2_LEVEL_LOW** [24:24]
    #[inline(always)]
    pub fn gpio_qspi_sd2_level_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD2_LEVEL_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SD2_LEVEL_LOW bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd2_level_low(&self) {
        self.set_bits(Self::GPIO_QSPI_SD2_LEVEL_LOW_MASK);
    }

    /// Clear GPIO_QSPI_SD2_LEVEL_LOW bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd2_level_low(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD2_LEVEL_LOW_MASK);
    }

    /// **GPIO_QSPI_SD1_EDGE_HIGH** [23:23]
    #[inline(always)]
    pub fn gpio_qspi_sd1_edge_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD1_EDGE_HIGH_MASK) != 0
    }

    /// Set GPIO_QSPI_SD1_EDGE_HIGH bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd1_edge_high(&self) {
        self.set_bits(Self::GPIO_QSPI_SD1_EDGE_HIGH_MASK);
    }

    /// Clear GPIO_QSPI_SD1_EDGE_HIGH bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd1_edge_high(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD1_EDGE_HIGH_MASK);
    }

    /// **GPIO_QSPI_SD1_EDGE_LOW** [22:22]
    #[inline(always)]
    pub fn gpio_qspi_sd1_edge_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD1_EDGE_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SD1_EDGE_LOW bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd1_edge_low(&self) {
        self.set_bits(Self::GPIO_QSPI_SD1_EDGE_LOW_MASK);
    }

    /// Clear GPIO_QSPI_SD1_EDGE_LOW bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd1_edge_low(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD1_EDGE_LOW_MASK);
    }

    /// **GPIO_QSPI_SD1_LEVEL_HIGH** [21:21]
    #[inline(always)]
    pub fn gpio_qspi_sd1_level_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD1_LEVEL_HIGH_MASK) != 0
    }

    /// Set GPIO_QSPI_SD1_LEVEL_HIGH bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd1_level_high(&self) {
        self.set_bits(Self::GPIO_QSPI_SD1_LEVEL_HIGH_MASK);
    }

    /// Clear GPIO_QSPI_SD1_LEVEL_HIGH bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd1_level_high(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD1_LEVEL_HIGH_MASK);
    }

    /// **GPIO_QSPI_SD1_LEVEL_LOW** [20:20]
    #[inline(always)]
    pub fn gpio_qspi_sd1_level_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD1_LEVEL_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SD1_LEVEL_LOW bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd1_level_low(&self) {
        self.set_bits(Self::GPIO_QSPI_SD1_LEVEL_LOW_MASK);
    }

    /// Clear GPIO_QSPI_SD1_LEVEL_LOW bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd1_level_low(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD1_LEVEL_LOW_MASK);
    }

    /// **GPIO_QSPI_SD0_EDGE_HIGH** [19:19]
    #[inline(always)]
    pub fn gpio_qspi_sd0_edge_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD0_EDGE_HIGH_MASK) != 0
    }

    /// Set GPIO_QSPI_SD0_EDGE_HIGH bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd0_edge_high(&self) {
        self.set_bits(Self::GPIO_QSPI_SD0_EDGE_HIGH_MASK);
    }

    /// Clear GPIO_QSPI_SD0_EDGE_HIGH bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd0_edge_high(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD0_EDGE_HIGH_MASK);
    }

    /// **GPIO_QSPI_SD0_EDGE_LOW** [18:18]
    #[inline(always)]
    pub fn gpio_qspi_sd0_edge_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD0_EDGE_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SD0_EDGE_LOW bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd0_edge_low(&self) {
        self.set_bits(Self::GPIO_QSPI_SD0_EDGE_LOW_MASK);
    }

    /// Clear GPIO_QSPI_SD0_EDGE_LOW bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd0_edge_low(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD0_EDGE_LOW_MASK);
    }

    /// **GPIO_QSPI_SD0_LEVEL_HIGH** [17:17]
    #[inline(always)]
    pub fn gpio_qspi_sd0_level_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD0_LEVEL_HIGH_MASK) != 0
    }

    /// Set GPIO_QSPI_SD0_LEVEL_HIGH bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd0_level_high(&self) {
        self.set_bits(Self::GPIO_QSPI_SD0_LEVEL_HIGH_MASK);
    }

    /// Clear GPIO_QSPI_SD0_LEVEL_HIGH bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd0_level_high(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD0_LEVEL_HIGH_MASK);
    }

    /// **GPIO_QSPI_SD0_LEVEL_LOW** [16:16]
    #[inline(always)]
    pub fn gpio_qspi_sd0_level_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD0_LEVEL_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SD0_LEVEL_LOW bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd0_level_low(&self) {
        self.set_bits(Self::GPIO_QSPI_SD0_LEVEL_LOW_MASK);
    }

    /// Clear GPIO_QSPI_SD0_LEVEL_LOW bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd0_level_low(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD0_LEVEL_LOW_MASK);
    }

    /// **GPIO_QSPI_SS_EDGE_HIGH** [15:15]
    #[inline(always)]
    pub fn gpio_qspi_ss_edge_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SS_EDGE_HIGH_MASK) != 0
    }

    /// Set GPIO_QSPI_SS_EDGE_HIGH bit
    #[inline(always)]
    pub fn set_gpio_qspi_ss_edge_high(&self) {
        self.set_bits(Self::GPIO_QSPI_SS_EDGE_HIGH_MASK);
    }

    /// Clear GPIO_QSPI_SS_EDGE_HIGH bit
    #[inline(always)]
    pub fn clr_gpio_qspi_ss_edge_high(&self) {
        self.clr_bits(Self::GPIO_QSPI_SS_EDGE_HIGH_MASK);
    }

    /// **GPIO_QSPI_SS_EDGE_LOW** [14:14]
    #[inline(always)]
    pub fn gpio_qspi_ss_edge_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SS_EDGE_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SS_EDGE_LOW bit
    #[inline(always)]
    pub fn set_gpio_qspi_ss_edge_low(&self) {
        self.set_bits(Self::GPIO_QSPI_SS_EDGE_LOW_MASK);
    }

    /// Clear GPIO_QSPI_SS_EDGE_LOW bit
    #[inline(always)]
    pub fn clr_gpio_qspi_ss_edge_low(&self) {
        self.clr_bits(Self::GPIO_QSPI_SS_EDGE_LOW_MASK);
    }

    /// **GPIO_QSPI_SS_LEVEL_HIGH** [13:13]
    #[inline(always)]
    pub fn gpio_qspi_ss_level_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SS_LEVEL_HIGH_MASK) != 0
    }

    /// Set GPIO_QSPI_SS_LEVEL_HIGH bit
    #[inline(always)]
    pub fn set_gpio_qspi_ss_level_high(&self) {
        self.set_bits(Self::GPIO_QSPI_SS_LEVEL_HIGH_MASK);
    }

    /// Clear GPIO_QSPI_SS_LEVEL_HIGH bit
    #[inline(always)]
    pub fn clr_gpio_qspi_ss_level_high(&self) {
        self.clr_bits(Self::GPIO_QSPI_SS_LEVEL_HIGH_MASK);
    }

    /// **GPIO_QSPI_SS_LEVEL_LOW** [12:12]
    #[inline(always)]
    pub fn gpio_qspi_ss_level_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SS_LEVEL_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SS_LEVEL_LOW bit
    #[inline(always)]
    pub fn set_gpio_qspi_ss_level_low(&self) {
        self.set_bits(Self::GPIO_QSPI_SS_LEVEL_LOW_MASK);
    }

    /// Clear GPIO_QSPI_SS_LEVEL_LOW bit
    #[inline(always)]
    pub fn clr_gpio_qspi_ss_level_low(&self) {
        self.clr_bits(Self::GPIO_QSPI_SS_LEVEL_LOW_MASK);
    }

    /// **GPIO_QSPI_SCLK_EDGE_HIGH** [11:11]
    #[inline(always)]
    pub fn gpio_qspi_sclk_edge_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SCLK_EDGE_HIGH_MASK) != 0
    }

    /// Set GPIO_QSPI_SCLK_EDGE_HIGH bit
    #[inline(always)]
    pub fn set_gpio_qspi_sclk_edge_high(&self) {
        self.set_bits(Self::GPIO_QSPI_SCLK_EDGE_HIGH_MASK);
    }

    /// Clear GPIO_QSPI_SCLK_EDGE_HIGH bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sclk_edge_high(&self) {
        self.clr_bits(Self::GPIO_QSPI_SCLK_EDGE_HIGH_MASK);
    }

    /// **GPIO_QSPI_SCLK_EDGE_LOW** [10:10]
    #[inline(always)]
    pub fn gpio_qspi_sclk_edge_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SCLK_EDGE_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SCLK_EDGE_LOW bit
    #[inline(always)]
    pub fn set_gpio_qspi_sclk_edge_low(&self) {
        self.set_bits(Self::GPIO_QSPI_SCLK_EDGE_LOW_MASK);
    }

    /// Clear GPIO_QSPI_SCLK_EDGE_LOW bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sclk_edge_low(&self) {
        self.clr_bits(Self::GPIO_QSPI_SCLK_EDGE_LOW_MASK);
    }

    /// **GPIO_QSPI_SCLK_LEVEL_HIGH** [9:9]
    #[inline(always)]
    pub fn gpio_qspi_sclk_level_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SCLK_LEVEL_HIGH_MASK) != 0
    }

    /// Set GPIO_QSPI_SCLK_LEVEL_HIGH bit
    #[inline(always)]
    pub fn set_gpio_qspi_sclk_level_high(&self) {
        self.set_bits(Self::GPIO_QSPI_SCLK_LEVEL_HIGH_MASK);
    }

    /// Clear GPIO_QSPI_SCLK_LEVEL_HIGH bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sclk_level_high(&self) {
        self.clr_bits(Self::GPIO_QSPI_SCLK_LEVEL_HIGH_MASK);
    }

    /// **GPIO_QSPI_SCLK_LEVEL_LOW** [8:8]
    #[inline(always)]
    pub fn gpio_qspi_sclk_level_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SCLK_LEVEL_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SCLK_LEVEL_LOW bit
    #[inline(always)]
    pub fn set_gpio_qspi_sclk_level_low(&self) {
        self.set_bits(Self::GPIO_QSPI_SCLK_LEVEL_LOW_MASK);
    }

    /// Clear GPIO_QSPI_SCLK_LEVEL_LOW bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sclk_level_low(&self) {
        self.clr_bits(Self::GPIO_QSPI_SCLK_LEVEL_LOW_MASK);
    }

    /// **USBPHY_DM_EDGE_HIGH** [7:7]
    #[inline(always)]
    pub fn usbphy_dm_edge_high(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DM_EDGE_HIGH_MASK) != 0
    }

    /// Set USBPHY_DM_EDGE_HIGH bit
    #[inline(always)]
    pub fn set_usbphy_dm_edge_high(&self) {
        self.set_bits(Self::USBPHY_DM_EDGE_HIGH_MASK);
    }

    /// Clear USBPHY_DM_EDGE_HIGH bit
    #[inline(always)]
    pub fn clr_usbphy_dm_edge_high(&self) {
        self.clr_bits(Self::USBPHY_DM_EDGE_HIGH_MASK);
    }

    /// **USBPHY_DM_EDGE_LOW** [6:6]
    #[inline(always)]
    pub fn usbphy_dm_edge_low(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DM_EDGE_LOW_MASK) != 0
    }

    /// Set USBPHY_DM_EDGE_LOW bit
    #[inline(always)]
    pub fn set_usbphy_dm_edge_low(&self) {
        self.set_bits(Self::USBPHY_DM_EDGE_LOW_MASK);
    }

    /// Clear USBPHY_DM_EDGE_LOW bit
    #[inline(always)]
    pub fn clr_usbphy_dm_edge_low(&self) {
        self.clr_bits(Self::USBPHY_DM_EDGE_LOW_MASK);
    }

    /// **USBPHY_DM_LEVEL_HIGH** [5:5]
    #[inline(always)]
    pub fn usbphy_dm_level_high(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DM_LEVEL_HIGH_MASK) != 0
    }

    /// Set USBPHY_DM_LEVEL_HIGH bit
    #[inline(always)]
    pub fn set_usbphy_dm_level_high(&self) {
        self.set_bits(Self::USBPHY_DM_LEVEL_HIGH_MASK);
    }

    /// Clear USBPHY_DM_LEVEL_HIGH bit
    #[inline(always)]
    pub fn clr_usbphy_dm_level_high(&self) {
        self.clr_bits(Self::USBPHY_DM_LEVEL_HIGH_MASK);
    }

    /// **USBPHY_DM_LEVEL_LOW** [4:4]
    #[inline(always)]
    pub fn usbphy_dm_level_low(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DM_LEVEL_LOW_MASK) != 0
    }

    /// Set USBPHY_DM_LEVEL_LOW bit
    #[inline(always)]
    pub fn set_usbphy_dm_level_low(&self) {
        self.set_bits(Self::USBPHY_DM_LEVEL_LOW_MASK);
    }

    /// Clear USBPHY_DM_LEVEL_LOW bit
    #[inline(always)]
    pub fn clr_usbphy_dm_level_low(&self) {
        self.clr_bits(Self::USBPHY_DM_LEVEL_LOW_MASK);
    }

    /// **USBPHY_DP_EDGE_HIGH** [3:3]
    #[inline(always)]
    pub fn usbphy_dp_edge_high(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DP_EDGE_HIGH_MASK) != 0
    }

    /// Set USBPHY_DP_EDGE_HIGH bit
    #[inline(always)]
    pub fn set_usbphy_dp_edge_high(&self) {
        self.set_bits(Self::USBPHY_DP_EDGE_HIGH_MASK);
    }

    /// Clear USBPHY_DP_EDGE_HIGH bit
    #[inline(always)]
    pub fn clr_usbphy_dp_edge_high(&self) {
        self.clr_bits(Self::USBPHY_DP_EDGE_HIGH_MASK);
    }

    /// **USBPHY_DP_EDGE_LOW** [2:2]
    #[inline(always)]
    pub fn usbphy_dp_edge_low(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DP_EDGE_LOW_MASK) != 0
    }

    /// Set USBPHY_DP_EDGE_LOW bit
    #[inline(always)]
    pub fn set_usbphy_dp_edge_low(&self) {
        self.set_bits(Self::USBPHY_DP_EDGE_LOW_MASK);
    }

    /// Clear USBPHY_DP_EDGE_LOW bit
    #[inline(always)]
    pub fn clr_usbphy_dp_edge_low(&self) {
        self.clr_bits(Self::USBPHY_DP_EDGE_LOW_MASK);
    }

    /// **USBPHY_DP_LEVEL_HIGH** [1:1]
    #[inline(always)]
    pub fn usbphy_dp_level_high(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DP_LEVEL_HIGH_MASK) != 0
    }

    /// Set USBPHY_DP_LEVEL_HIGH bit
    #[inline(always)]
    pub fn set_usbphy_dp_level_high(&self) {
        self.set_bits(Self::USBPHY_DP_LEVEL_HIGH_MASK);
    }

    /// Clear USBPHY_DP_LEVEL_HIGH bit
    #[inline(always)]
    pub fn clr_usbphy_dp_level_high(&self) {
        self.clr_bits(Self::USBPHY_DP_LEVEL_HIGH_MASK);
    }

    /// **USBPHY_DP_LEVEL_LOW** [0:0]
    #[inline(always)]
    pub fn usbphy_dp_level_low(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DP_LEVEL_LOW_MASK) != 0
    }

    /// Set USBPHY_DP_LEVEL_LOW bit
    #[inline(always)]
    pub fn set_usbphy_dp_level_low(&self) {
        self.set_bits(Self::USBPHY_DP_LEVEL_LOW_MASK);
    }

    /// Clear USBPHY_DP_LEVEL_LOW bit
    #[inline(always)]
    pub fn clr_usbphy_dp_level_low(&self) {
        self.clr_bits(Self::USBPHY_DP_LEVEL_LOW_MASK);
    }

}

/// DORMANT_WAKE_INTE value type for batch read/modify/write operations
///
/// Use `dormantwakeinte.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dormantwakeinte.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DormantWakeInteVal {
    bits: u32,
}

impl DormantWakeInteVal {
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

    /// Read GPIO_QSPI_SD3_EDGE_HIGH [31:31]
    #[inline(always)]
    pub const fn gpio_qspi_sd3_edge_high(&self) -> bool {
        (self.bits & DormantWakeInte::GPIO_QSPI_SD3_EDGE_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD3_EDGE_LOW [30:30]
    #[inline(always)]
    pub const fn gpio_qspi_sd3_edge_low(&self) -> bool {
        (self.bits & DormantWakeInte::GPIO_QSPI_SD3_EDGE_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SD3_LEVEL_HIGH [29:29]
    #[inline(always)]
    pub const fn gpio_qspi_sd3_level_high(&self) -> bool {
        (self.bits & DormantWakeInte::GPIO_QSPI_SD3_LEVEL_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD3_LEVEL_LOW [28:28]
    #[inline(always)]
    pub const fn gpio_qspi_sd3_level_low(&self) -> bool {
        (self.bits & DormantWakeInte::GPIO_QSPI_SD3_LEVEL_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SD2_EDGE_HIGH [27:27]
    #[inline(always)]
    pub const fn gpio_qspi_sd2_edge_high(&self) -> bool {
        (self.bits & DormantWakeInte::GPIO_QSPI_SD2_EDGE_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD2_EDGE_LOW [26:26]
    #[inline(always)]
    pub const fn gpio_qspi_sd2_edge_low(&self) -> bool {
        (self.bits & DormantWakeInte::GPIO_QSPI_SD2_EDGE_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SD2_LEVEL_HIGH [25:25]
    #[inline(always)]
    pub const fn gpio_qspi_sd2_level_high(&self) -> bool {
        (self.bits & DormantWakeInte::GPIO_QSPI_SD2_LEVEL_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD2_LEVEL_LOW [24:24]
    #[inline(always)]
    pub const fn gpio_qspi_sd2_level_low(&self) -> bool {
        (self.bits & DormantWakeInte::GPIO_QSPI_SD2_LEVEL_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SD1_EDGE_HIGH [23:23]
    #[inline(always)]
    pub const fn gpio_qspi_sd1_edge_high(&self) -> bool {
        (self.bits & DormantWakeInte::GPIO_QSPI_SD1_EDGE_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD1_EDGE_LOW [22:22]
    #[inline(always)]
    pub const fn gpio_qspi_sd1_edge_low(&self) -> bool {
        (self.bits & DormantWakeInte::GPIO_QSPI_SD1_EDGE_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SD1_LEVEL_HIGH [21:21]
    #[inline(always)]
    pub const fn gpio_qspi_sd1_level_high(&self) -> bool {
        (self.bits & DormantWakeInte::GPIO_QSPI_SD1_LEVEL_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD1_LEVEL_LOW [20:20]
    #[inline(always)]
    pub const fn gpio_qspi_sd1_level_low(&self) -> bool {
        (self.bits & DormantWakeInte::GPIO_QSPI_SD1_LEVEL_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SD0_EDGE_HIGH [19:19]
    #[inline(always)]
    pub const fn gpio_qspi_sd0_edge_high(&self) -> bool {
        (self.bits & DormantWakeInte::GPIO_QSPI_SD0_EDGE_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD0_EDGE_LOW [18:18]
    #[inline(always)]
    pub const fn gpio_qspi_sd0_edge_low(&self) -> bool {
        (self.bits & DormantWakeInte::GPIO_QSPI_SD0_EDGE_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SD0_LEVEL_HIGH [17:17]
    #[inline(always)]
    pub const fn gpio_qspi_sd0_level_high(&self) -> bool {
        (self.bits & DormantWakeInte::GPIO_QSPI_SD0_LEVEL_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD0_LEVEL_LOW [16:16]
    #[inline(always)]
    pub const fn gpio_qspi_sd0_level_low(&self) -> bool {
        (self.bits & DormantWakeInte::GPIO_QSPI_SD0_LEVEL_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SS_EDGE_HIGH [15:15]
    #[inline(always)]
    pub const fn gpio_qspi_ss_edge_high(&self) -> bool {
        (self.bits & DormantWakeInte::GPIO_QSPI_SS_EDGE_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SS_EDGE_LOW [14:14]
    #[inline(always)]
    pub const fn gpio_qspi_ss_edge_low(&self) -> bool {
        (self.bits & DormantWakeInte::GPIO_QSPI_SS_EDGE_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SS_LEVEL_HIGH [13:13]
    #[inline(always)]
    pub const fn gpio_qspi_ss_level_high(&self) -> bool {
        (self.bits & DormantWakeInte::GPIO_QSPI_SS_LEVEL_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SS_LEVEL_LOW [12:12]
    #[inline(always)]
    pub const fn gpio_qspi_ss_level_low(&self) -> bool {
        (self.bits & DormantWakeInte::GPIO_QSPI_SS_LEVEL_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SCLK_EDGE_HIGH [11:11]
    #[inline(always)]
    pub const fn gpio_qspi_sclk_edge_high(&self) -> bool {
        (self.bits & DormantWakeInte::GPIO_QSPI_SCLK_EDGE_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SCLK_EDGE_LOW [10:10]
    #[inline(always)]
    pub const fn gpio_qspi_sclk_edge_low(&self) -> bool {
        (self.bits & DormantWakeInte::GPIO_QSPI_SCLK_EDGE_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SCLK_LEVEL_HIGH [9:9]
    #[inline(always)]
    pub const fn gpio_qspi_sclk_level_high(&self) -> bool {
        (self.bits & DormantWakeInte::GPIO_QSPI_SCLK_LEVEL_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SCLK_LEVEL_LOW [8:8]
    #[inline(always)]
    pub const fn gpio_qspi_sclk_level_low(&self) -> bool {
        (self.bits & DormantWakeInte::GPIO_QSPI_SCLK_LEVEL_LOW_MASK) != 0
    }

    /// Read USBPHY_DM_EDGE_HIGH [7:7]
    #[inline(always)]
    pub const fn usbphy_dm_edge_high(&self) -> bool {
        (self.bits & DormantWakeInte::USBPHY_DM_EDGE_HIGH_MASK) != 0
    }

    /// Read USBPHY_DM_EDGE_LOW [6:6]
    #[inline(always)]
    pub const fn usbphy_dm_edge_low(&self) -> bool {
        (self.bits & DormantWakeInte::USBPHY_DM_EDGE_LOW_MASK) != 0
    }

    /// Read USBPHY_DM_LEVEL_HIGH [5:5]
    #[inline(always)]
    pub const fn usbphy_dm_level_high(&self) -> bool {
        (self.bits & DormantWakeInte::USBPHY_DM_LEVEL_HIGH_MASK) != 0
    }

    /// Read USBPHY_DM_LEVEL_LOW [4:4]
    #[inline(always)]
    pub const fn usbphy_dm_level_low(&self) -> bool {
        (self.bits & DormantWakeInte::USBPHY_DM_LEVEL_LOW_MASK) != 0
    }

    /// Read USBPHY_DP_EDGE_HIGH [3:3]
    #[inline(always)]
    pub const fn usbphy_dp_edge_high(&self) -> bool {
        (self.bits & DormantWakeInte::USBPHY_DP_EDGE_HIGH_MASK) != 0
    }

    /// Read USBPHY_DP_EDGE_LOW [2:2]
    #[inline(always)]
    pub const fn usbphy_dp_edge_low(&self) -> bool {
        (self.bits & DormantWakeInte::USBPHY_DP_EDGE_LOW_MASK) != 0
    }

    /// Read USBPHY_DP_LEVEL_HIGH [1:1]
    #[inline(always)]
    pub const fn usbphy_dp_level_high(&self) -> bool {
        (self.bits & DormantWakeInte::USBPHY_DP_LEVEL_HIGH_MASK) != 0
    }

    /// Read USBPHY_DP_LEVEL_LOW [0:0]
    #[inline(always)]
    pub const fn usbphy_dp_level_low(&self) -> bool {
        (self.bits & DormantWakeInte::USBPHY_DP_LEVEL_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SD3_EDGE_HIGH [31:31]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd3_edge_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DormantWakeInte::GPIO_QSPI_SD3_EDGE_HIGH_MASK
            } else {
                self.bits & !DormantWakeInte::GPIO_QSPI_SD3_EDGE_HIGH_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD3_EDGE_LOW [30:30]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd3_edge_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DormantWakeInte::GPIO_QSPI_SD3_EDGE_LOW_MASK
            } else {
                self.bits & !DormantWakeInte::GPIO_QSPI_SD3_EDGE_LOW_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD3_LEVEL_HIGH [29:29]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd3_level_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DormantWakeInte::GPIO_QSPI_SD3_LEVEL_HIGH_MASK
            } else {
                self.bits & !DormantWakeInte::GPIO_QSPI_SD3_LEVEL_HIGH_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD3_LEVEL_LOW [28:28]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd3_level_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DormantWakeInte::GPIO_QSPI_SD3_LEVEL_LOW_MASK
            } else {
                self.bits & !DormantWakeInte::GPIO_QSPI_SD3_LEVEL_LOW_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD2_EDGE_HIGH [27:27]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd2_edge_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DormantWakeInte::GPIO_QSPI_SD2_EDGE_HIGH_MASK
            } else {
                self.bits & !DormantWakeInte::GPIO_QSPI_SD2_EDGE_HIGH_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD2_EDGE_LOW [26:26]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd2_edge_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DormantWakeInte::GPIO_QSPI_SD2_EDGE_LOW_MASK
            } else {
                self.bits & !DormantWakeInte::GPIO_QSPI_SD2_EDGE_LOW_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD2_LEVEL_HIGH [25:25]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd2_level_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DormantWakeInte::GPIO_QSPI_SD2_LEVEL_HIGH_MASK
            } else {
                self.bits & !DormantWakeInte::GPIO_QSPI_SD2_LEVEL_HIGH_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD2_LEVEL_LOW [24:24]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd2_level_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DormantWakeInte::GPIO_QSPI_SD2_LEVEL_LOW_MASK
            } else {
                self.bits & !DormantWakeInte::GPIO_QSPI_SD2_LEVEL_LOW_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD1_EDGE_HIGH [23:23]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd1_edge_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DormantWakeInte::GPIO_QSPI_SD1_EDGE_HIGH_MASK
            } else {
                self.bits & !DormantWakeInte::GPIO_QSPI_SD1_EDGE_HIGH_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD1_EDGE_LOW [22:22]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd1_edge_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DormantWakeInte::GPIO_QSPI_SD1_EDGE_LOW_MASK
            } else {
                self.bits & !DormantWakeInte::GPIO_QSPI_SD1_EDGE_LOW_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD1_LEVEL_HIGH [21:21]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd1_level_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DormantWakeInte::GPIO_QSPI_SD1_LEVEL_HIGH_MASK
            } else {
                self.bits & !DormantWakeInte::GPIO_QSPI_SD1_LEVEL_HIGH_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD1_LEVEL_LOW [20:20]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd1_level_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DormantWakeInte::GPIO_QSPI_SD1_LEVEL_LOW_MASK
            } else {
                self.bits & !DormantWakeInte::GPIO_QSPI_SD1_LEVEL_LOW_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD0_EDGE_HIGH [19:19]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd0_edge_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DormantWakeInte::GPIO_QSPI_SD0_EDGE_HIGH_MASK
            } else {
                self.bits & !DormantWakeInte::GPIO_QSPI_SD0_EDGE_HIGH_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD0_EDGE_LOW [18:18]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd0_edge_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DormantWakeInte::GPIO_QSPI_SD0_EDGE_LOW_MASK
            } else {
                self.bits & !DormantWakeInte::GPIO_QSPI_SD0_EDGE_LOW_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD0_LEVEL_HIGH [17:17]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd0_level_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DormantWakeInte::GPIO_QSPI_SD0_LEVEL_HIGH_MASK
            } else {
                self.bits & !DormantWakeInte::GPIO_QSPI_SD0_LEVEL_HIGH_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD0_LEVEL_LOW [16:16]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd0_level_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DormantWakeInte::GPIO_QSPI_SD0_LEVEL_LOW_MASK
            } else {
                self.bits & !DormantWakeInte::GPIO_QSPI_SD0_LEVEL_LOW_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SS_EDGE_HIGH [15:15]
    #[inline(always)]
    pub const fn with_gpio_qspi_ss_edge_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DormantWakeInte::GPIO_QSPI_SS_EDGE_HIGH_MASK
            } else {
                self.bits & !DormantWakeInte::GPIO_QSPI_SS_EDGE_HIGH_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SS_EDGE_LOW [14:14]
    #[inline(always)]
    pub const fn with_gpio_qspi_ss_edge_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DormantWakeInte::GPIO_QSPI_SS_EDGE_LOW_MASK
            } else {
                self.bits & !DormantWakeInte::GPIO_QSPI_SS_EDGE_LOW_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SS_LEVEL_HIGH [13:13]
    #[inline(always)]
    pub const fn with_gpio_qspi_ss_level_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DormantWakeInte::GPIO_QSPI_SS_LEVEL_HIGH_MASK
            } else {
                self.bits & !DormantWakeInte::GPIO_QSPI_SS_LEVEL_HIGH_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SS_LEVEL_LOW [12:12]
    #[inline(always)]
    pub const fn with_gpio_qspi_ss_level_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DormantWakeInte::GPIO_QSPI_SS_LEVEL_LOW_MASK
            } else {
                self.bits & !DormantWakeInte::GPIO_QSPI_SS_LEVEL_LOW_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SCLK_EDGE_HIGH [11:11]
    #[inline(always)]
    pub const fn with_gpio_qspi_sclk_edge_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DormantWakeInte::GPIO_QSPI_SCLK_EDGE_HIGH_MASK
            } else {
                self.bits & !DormantWakeInte::GPIO_QSPI_SCLK_EDGE_HIGH_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SCLK_EDGE_LOW [10:10]
    #[inline(always)]
    pub const fn with_gpio_qspi_sclk_edge_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DormantWakeInte::GPIO_QSPI_SCLK_EDGE_LOW_MASK
            } else {
                self.bits & !DormantWakeInte::GPIO_QSPI_SCLK_EDGE_LOW_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SCLK_LEVEL_HIGH [9:9]
    #[inline(always)]
    pub const fn with_gpio_qspi_sclk_level_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DormantWakeInte::GPIO_QSPI_SCLK_LEVEL_HIGH_MASK
            } else {
                self.bits & !DormantWakeInte::GPIO_QSPI_SCLK_LEVEL_HIGH_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SCLK_LEVEL_LOW [8:8]
    #[inline(always)]
    pub const fn with_gpio_qspi_sclk_level_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DormantWakeInte::GPIO_QSPI_SCLK_LEVEL_LOW_MASK
            } else {
                self.bits & !DormantWakeInte::GPIO_QSPI_SCLK_LEVEL_LOW_MASK
            }
        }
    }

    /// Set USBPHY_DM_EDGE_HIGH [7:7]
    #[inline(always)]
    pub const fn with_usbphy_dm_edge_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DormantWakeInte::USBPHY_DM_EDGE_HIGH_MASK
            } else {
                self.bits & !DormantWakeInte::USBPHY_DM_EDGE_HIGH_MASK
            }
        }
    }

    /// Set USBPHY_DM_EDGE_LOW [6:6]
    #[inline(always)]
    pub const fn with_usbphy_dm_edge_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DormantWakeInte::USBPHY_DM_EDGE_LOW_MASK
            } else {
                self.bits & !DormantWakeInte::USBPHY_DM_EDGE_LOW_MASK
            }
        }
    }

    /// Set USBPHY_DM_LEVEL_HIGH [5:5]
    #[inline(always)]
    pub const fn with_usbphy_dm_level_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DormantWakeInte::USBPHY_DM_LEVEL_HIGH_MASK
            } else {
                self.bits & !DormantWakeInte::USBPHY_DM_LEVEL_HIGH_MASK
            }
        }
    }

    /// Set USBPHY_DM_LEVEL_LOW [4:4]
    #[inline(always)]
    pub const fn with_usbphy_dm_level_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DormantWakeInte::USBPHY_DM_LEVEL_LOW_MASK
            } else {
                self.bits & !DormantWakeInte::USBPHY_DM_LEVEL_LOW_MASK
            }
        }
    }

    /// Set USBPHY_DP_EDGE_HIGH [3:3]
    #[inline(always)]
    pub const fn with_usbphy_dp_edge_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DormantWakeInte::USBPHY_DP_EDGE_HIGH_MASK
            } else {
                self.bits & !DormantWakeInte::USBPHY_DP_EDGE_HIGH_MASK
            }
        }
    }

    /// Set USBPHY_DP_EDGE_LOW [2:2]
    #[inline(always)]
    pub const fn with_usbphy_dp_edge_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DormantWakeInte::USBPHY_DP_EDGE_LOW_MASK
            } else {
                self.bits & !DormantWakeInte::USBPHY_DP_EDGE_LOW_MASK
            }
        }
    }

    /// Set USBPHY_DP_LEVEL_HIGH [1:1]
    #[inline(always)]
    pub const fn with_usbphy_dp_level_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DormantWakeInte::USBPHY_DP_LEVEL_HIGH_MASK
            } else {
                self.bits & !DormantWakeInte::USBPHY_DP_LEVEL_HIGH_MASK
            }
        }
    }

    /// Set USBPHY_DP_LEVEL_LOW [0:0]
    #[inline(always)]
    pub const fn with_usbphy_dp_level_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DormantWakeInte::USBPHY_DP_LEVEL_LOW_MASK
            } else {
                self.bits & !DormantWakeInte::USBPHY_DP_LEVEL_LOW_MASK
            }
        }
    }

}

impl Default for DormantWakeInteVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl DormantWakeInte {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> DormantWakeInteVal {
        DormantWakeInteVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: DormantWakeInteVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(DormantWakeInteVal) -> DormantWakeInteVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// DORMANT_WAKE_INTF Register
///
/// Interrupt Force for dormant_wake
///
/// Address offset: 0x0238
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **GPIO_QSPI_SD3_EDGE_HIGH** [31] (RW): 
/// - **GPIO_QSPI_SD3_EDGE_LOW** [30] (RW): 
/// - **GPIO_QSPI_SD3_LEVEL_HIGH** [29] (RW): 
/// - **GPIO_QSPI_SD3_LEVEL_LOW** [28] (RW): 
/// - **GPIO_QSPI_SD2_EDGE_HIGH** [27] (RW): 
/// - **GPIO_QSPI_SD2_EDGE_LOW** [26] (RW): 
/// - **GPIO_QSPI_SD2_LEVEL_HIGH** [25] (RW): 
/// - **GPIO_QSPI_SD2_LEVEL_LOW** [24] (RW): 
/// - **GPIO_QSPI_SD1_EDGE_HIGH** [23] (RW): 
/// - **GPIO_QSPI_SD1_EDGE_LOW** [22] (RW): 
/// - **GPIO_QSPI_SD1_LEVEL_HIGH** [21] (RW): 
/// - **GPIO_QSPI_SD1_LEVEL_LOW** [20] (RW): 
/// - **GPIO_QSPI_SD0_EDGE_HIGH** [19] (RW): 
/// - **GPIO_QSPI_SD0_EDGE_LOW** [18] (RW): 
/// - **GPIO_QSPI_SD0_LEVEL_HIGH** [17] (RW): 
/// - **GPIO_QSPI_SD0_LEVEL_LOW** [16] (RW): 
/// - **GPIO_QSPI_SS_EDGE_HIGH** [15] (RW): 
/// - **GPIO_QSPI_SS_EDGE_LOW** [14] (RW): 
/// - **GPIO_QSPI_SS_LEVEL_HIGH** [13] (RW): 
/// - **GPIO_QSPI_SS_LEVEL_LOW** [12] (RW): 
/// - **GPIO_QSPI_SCLK_EDGE_HIGH** [11] (RW): 
/// - **GPIO_QSPI_SCLK_EDGE_LOW** [10] (RW): 
/// - **GPIO_QSPI_SCLK_LEVEL_HIGH** [9] (RW): 
/// - **GPIO_QSPI_SCLK_LEVEL_LOW** [8] (RW): 
/// - **USBPHY_DM_EDGE_HIGH** [7] (RW): 
/// - **USBPHY_DM_EDGE_LOW** [6] (RW): 
/// - **USBPHY_DM_LEVEL_HIGH** [5] (RW): 
/// - **USBPHY_DM_LEVEL_LOW** [4] (RW): 
/// - **USBPHY_DP_EDGE_HIGH** [3] (RW): 
/// - **USBPHY_DP_EDGE_LOW** [2] (RW): 
/// - **USBPHY_DP_LEVEL_HIGH** [1] (RW): 
/// - **USBPHY_DP_LEVEL_LOW** [0] (RW): 
#[repr(transparent)]
pub struct DormantWakeIntf {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for DormantWakeIntf {}
unsafe impl Sync for DormantWakeIntf {}

impl DormantWakeIntf {
    // ======== Field Constants ========

    /// Bits [31:31], mask 0x80000000
    pub const GPIO_QSPI_SD3_EDGE_HIGH_OFFSET: u32 = 31;
    pub const GPIO_QSPI_SD3_EDGE_HIGH_MASK: u32 = 0x80000000;
    pub const GPIO_QSPI_SD3_EDGE_HIGH: u32 = 1 << 31;

    /// Bits [30:30], mask 0x40000000
    pub const GPIO_QSPI_SD3_EDGE_LOW_OFFSET: u32 = 30;
    pub const GPIO_QSPI_SD3_EDGE_LOW_MASK: u32 = 0x40000000;
    pub const GPIO_QSPI_SD3_EDGE_LOW: u32 = 1 << 30;

    /// Bits [29:29], mask 0x20000000
    pub const GPIO_QSPI_SD3_LEVEL_HIGH_OFFSET: u32 = 29;
    pub const GPIO_QSPI_SD3_LEVEL_HIGH_MASK: u32 = 0x20000000;
    pub const GPIO_QSPI_SD3_LEVEL_HIGH: u32 = 1 << 29;

    /// Bits [28:28], mask 0x10000000
    pub const GPIO_QSPI_SD3_LEVEL_LOW_OFFSET: u32 = 28;
    pub const GPIO_QSPI_SD3_LEVEL_LOW_MASK: u32 = 0x10000000;
    pub const GPIO_QSPI_SD3_LEVEL_LOW: u32 = 1 << 28;

    /// Bits [27:27], mask 0x08000000
    pub const GPIO_QSPI_SD2_EDGE_HIGH_OFFSET: u32 = 27;
    pub const GPIO_QSPI_SD2_EDGE_HIGH_MASK: u32 = 0x08000000;
    pub const GPIO_QSPI_SD2_EDGE_HIGH: u32 = 1 << 27;

    /// Bits [26:26], mask 0x04000000
    pub const GPIO_QSPI_SD2_EDGE_LOW_OFFSET: u32 = 26;
    pub const GPIO_QSPI_SD2_EDGE_LOW_MASK: u32 = 0x04000000;
    pub const GPIO_QSPI_SD2_EDGE_LOW: u32 = 1 << 26;

    /// Bits [25:25], mask 0x02000000
    pub const GPIO_QSPI_SD2_LEVEL_HIGH_OFFSET: u32 = 25;
    pub const GPIO_QSPI_SD2_LEVEL_HIGH_MASK: u32 = 0x02000000;
    pub const GPIO_QSPI_SD2_LEVEL_HIGH: u32 = 1 << 25;

    /// Bits [24:24], mask 0x01000000
    pub const GPIO_QSPI_SD2_LEVEL_LOW_OFFSET: u32 = 24;
    pub const GPIO_QSPI_SD2_LEVEL_LOW_MASK: u32 = 0x01000000;
    pub const GPIO_QSPI_SD2_LEVEL_LOW: u32 = 1 << 24;

    /// Bits [23:23], mask 0x00800000
    pub const GPIO_QSPI_SD1_EDGE_HIGH_OFFSET: u32 = 23;
    pub const GPIO_QSPI_SD1_EDGE_HIGH_MASK: u32 = 0x00800000;
    pub const GPIO_QSPI_SD1_EDGE_HIGH: u32 = 1 << 23;

    /// Bits [22:22], mask 0x00400000
    pub const GPIO_QSPI_SD1_EDGE_LOW_OFFSET: u32 = 22;
    pub const GPIO_QSPI_SD1_EDGE_LOW_MASK: u32 = 0x00400000;
    pub const GPIO_QSPI_SD1_EDGE_LOW: u32 = 1 << 22;

    /// Bits [21:21], mask 0x00200000
    pub const GPIO_QSPI_SD1_LEVEL_HIGH_OFFSET: u32 = 21;
    pub const GPIO_QSPI_SD1_LEVEL_HIGH_MASK: u32 = 0x00200000;
    pub const GPIO_QSPI_SD1_LEVEL_HIGH: u32 = 1 << 21;

    /// Bits [20:20], mask 0x00100000
    pub const GPIO_QSPI_SD1_LEVEL_LOW_OFFSET: u32 = 20;
    pub const GPIO_QSPI_SD1_LEVEL_LOW_MASK: u32 = 0x00100000;
    pub const GPIO_QSPI_SD1_LEVEL_LOW: u32 = 1 << 20;

    /// Bits [19:19], mask 0x00080000
    pub const GPIO_QSPI_SD0_EDGE_HIGH_OFFSET: u32 = 19;
    pub const GPIO_QSPI_SD0_EDGE_HIGH_MASK: u32 = 0x00080000;
    pub const GPIO_QSPI_SD0_EDGE_HIGH: u32 = 1 << 19;

    /// Bits [18:18], mask 0x00040000
    pub const GPIO_QSPI_SD0_EDGE_LOW_OFFSET: u32 = 18;
    pub const GPIO_QSPI_SD0_EDGE_LOW_MASK: u32 = 0x00040000;
    pub const GPIO_QSPI_SD0_EDGE_LOW: u32 = 1 << 18;

    /// Bits [17:17], mask 0x00020000
    pub const GPIO_QSPI_SD0_LEVEL_HIGH_OFFSET: u32 = 17;
    pub const GPIO_QSPI_SD0_LEVEL_HIGH_MASK: u32 = 0x00020000;
    pub const GPIO_QSPI_SD0_LEVEL_HIGH: u32 = 1 << 17;

    /// Bits [16:16], mask 0x00010000
    pub const GPIO_QSPI_SD0_LEVEL_LOW_OFFSET: u32 = 16;
    pub const GPIO_QSPI_SD0_LEVEL_LOW_MASK: u32 = 0x00010000;
    pub const GPIO_QSPI_SD0_LEVEL_LOW: u32 = 1 << 16;

    /// Bits [15:15], mask 0x00008000
    pub const GPIO_QSPI_SS_EDGE_HIGH_OFFSET: u32 = 15;
    pub const GPIO_QSPI_SS_EDGE_HIGH_MASK: u32 = 0x00008000;
    pub const GPIO_QSPI_SS_EDGE_HIGH: u32 = 1 << 15;

    /// Bits [14:14], mask 0x00004000
    pub const GPIO_QSPI_SS_EDGE_LOW_OFFSET: u32 = 14;
    pub const GPIO_QSPI_SS_EDGE_LOW_MASK: u32 = 0x00004000;
    pub const GPIO_QSPI_SS_EDGE_LOW: u32 = 1 << 14;

    /// Bits [13:13], mask 0x00002000
    pub const GPIO_QSPI_SS_LEVEL_HIGH_OFFSET: u32 = 13;
    pub const GPIO_QSPI_SS_LEVEL_HIGH_MASK: u32 = 0x00002000;
    pub const GPIO_QSPI_SS_LEVEL_HIGH: u32 = 1 << 13;

    /// Bits [12:12], mask 0x00001000
    pub const GPIO_QSPI_SS_LEVEL_LOW_OFFSET: u32 = 12;
    pub const GPIO_QSPI_SS_LEVEL_LOW_MASK: u32 = 0x00001000;
    pub const GPIO_QSPI_SS_LEVEL_LOW: u32 = 1 << 12;

    /// Bits [11:11], mask 0x00000800
    pub const GPIO_QSPI_SCLK_EDGE_HIGH_OFFSET: u32 = 11;
    pub const GPIO_QSPI_SCLK_EDGE_HIGH_MASK: u32 = 0x00000800;
    pub const GPIO_QSPI_SCLK_EDGE_HIGH: u32 = 1 << 11;

    /// Bits [10:10], mask 0x00000400
    pub const GPIO_QSPI_SCLK_EDGE_LOW_OFFSET: u32 = 10;
    pub const GPIO_QSPI_SCLK_EDGE_LOW_MASK: u32 = 0x00000400;
    pub const GPIO_QSPI_SCLK_EDGE_LOW: u32 = 1 << 10;

    /// Bits [9:9], mask 0x00000200
    pub const GPIO_QSPI_SCLK_LEVEL_HIGH_OFFSET: u32 = 9;
    pub const GPIO_QSPI_SCLK_LEVEL_HIGH_MASK: u32 = 0x00000200;
    pub const GPIO_QSPI_SCLK_LEVEL_HIGH: u32 = 1 << 9;

    /// Bits [8:8], mask 0x00000100
    pub const GPIO_QSPI_SCLK_LEVEL_LOW_OFFSET: u32 = 8;
    pub const GPIO_QSPI_SCLK_LEVEL_LOW_MASK: u32 = 0x00000100;
    pub const GPIO_QSPI_SCLK_LEVEL_LOW: u32 = 1 << 8;

    /// Bits [7:7], mask 0x00000080
    pub const USBPHY_DM_EDGE_HIGH_OFFSET: u32 = 7;
    pub const USBPHY_DM_EDGE_HIGH_MASK: u32 = 0x00000080;
    pub const USBPHY_DM_EDGE_HIGH: u32 = 1 << 7;

    /// Bits [6:6], mask 0x00000040
    pub const USBPHY_DM_EDGE_LOW_OFFSET: u32 = 6;
    pub const USBPHY_DM_EDGE_LOW_MASK: u32 = 0x00000040;
    pub const USBPHY_DM_EDGE_LOW: u32 = 1 << 6;

    /// Bits [5:5], mask 0x00000020
    pub const USBPHY_DM_LEVEL_HIGH_OFFSET: u32 = 5;
    pub const USBPHY_DM_LEVEL_HIGH_MASK: u32 = 0x00000020;
    pub const USBPHY_DM_LEVEL_HIGH: u32 = 1 << 5;

    /// Bits [4:4], mask 0x00000010
    pub const USBPHY_DM_LEVEL_LOW_OFFSET: u32 = 4;
    pub const USBPHY_DM_LEVEL_LOW_MASK: u32 = 0x00000010;
    pub const USBPHY_DM_LEVEL_LOW: u32 = 1 << 4;

    /// Bits [3:3], mask 0x00000008
    pub const USBPHY_DP_EDGE_HIGH_OFFSET: u32 = 3;
    pub const USBPHY_DP_EDGE_HIGH_MASK: u32 = 0x00000008;
    pub const USBPHY_DP_EDGE_HIGH: u32 = 1 << 3;

    /// Bits [2:2], mask 0x00000004
    pub const USBPHY_DP_EDGE_LOW_OFFSET: u32 = 2;
    pub const USBPHY_DP_EDGE_LOW_MASK: u32 = 0x00000004;
    pub const USBPHY_DP_EDGE_LOW: u32 = 1 << 2;

    /// Bits [1:1], mask 0x00000002
    pub const USBPHY_DP_LEVEL_HIGH_OFFSET: u32 = 1;
    pub const USBPHY_DP_LEVEL_HIGH_MASK: u32 = 0x00000002;
    pub const USBPHY_DP_LEVEL_HIGH: u32 = 1 << 1;

    /// Bits [0:0], mask 0x00000001
    pub const USBPHY_DP_LEVEL_LOW_OFFSET: u32 = 0;
    pub const USBPHY_DP_LEVEL_LOW_MASK: u32 = 0x00000001;
    pub const USBPHY_DP_LEVEL_LOW: u32 = 1 << 0;

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

    /// **GPIO_QSPI_SD3_EDGE_HIGH** [31:31]
    #[inline(always)]
    pub fn gpio_qspi_sd3_edge_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD3_EDGE_HIGH_MASK) != 0
    }

    /// Set GPIO_QSPI_SD3_EDGE_HIGH bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd3_edge_high(&self) {
        self.set_bits(Self::GPIO_QSPI_SD3_EDGE_HIGH_MASK);
    }

    /// Clear GPIO_QSPI_SD3_EDGE_HIGH bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd3_edge_high(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD3_EDGE_HIGH_MASK);
    }

    /// **GPIO_QSPI_SD3_EDGE_LOW** [30:30]
    #[inline(always)]
    pub fn gpio_qspi_sd3_edge_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD3_EDGE_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SD3_EDGE_LOW bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd3_edge_low(&self) {
        self.set_bits(Self::GPIO_QSPI_SD3_EDGE_LOW_MASK);
    }

    /// Clear GPIO_QSPI_SD3_EDGE_LOW bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd3_edge_low(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD3_EDGE_LOW_MASK);
    }

    /// **GPIO_QSPI_SD3_LEVEL_HIGH** [29:29]
    #[inline(always)]
    pub fn gpio_qspi_sd3_level_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD3_LEVEL_HIGH_MASK) != 0
    }

    /// Set GPIO_QSPI_SD3_LEVEL_HIGH bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd3_level_high(&self) {
        self.set_bits(Self::GPIO_QSPI_SD3_LEVEL_HIGH_MASK);
    }

    /// Clear GPIO_QSPI_SD3_LEVEL_HIGH bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd3_level_high(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD3_LEVEL_HIGH_MASK);
    }

    /// **GPIO_QSPI_SD3_LEVEL_LOW** [28:28]
    #[inline(always)]
    pub fn gpio_qspi_sd3_level_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD3_LEVEL_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SD3_LEVEL_LOW bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd3_level_low(&self) {
        self.set_bits(Self::GPIO_QSPI_SD3_LEVEL_LOW_MASK);
    }

    /// Clear GPIO_QSPI_SD3_LEVEL_LOW bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd3_level_low(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD3_LEVEL_LOW_MASK);
    }

    /// **GPIO_QSPI_SD2_EDGE_HIGH** [27:27]
    #[inline(always)]
    pub fn gpio_qspi_sd2_edge_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD2_EDGE_HIGH_MASK) != 0
    }

    /// Set GPIO_QSPI_SD2_EDGE_HIGH bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd2_edge_high(&self) {
        self.set_bits(Self::GPIO_QSPI_SD2_EDGE_HIGH_MASK);
    }

    /// Clear GPIO_QSPI_SD2_EDGE_HIGH bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd2_edge_high(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD2_EDGE_HIGH_MASK);
    }

    /// **GPIO_QSPI_SD2_EDGE_LOW** [26:26]
    #[inline(always)]
    pub fn gpio_qspi_sd2_edge_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD2_EDGE_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SD2_EDGE_LOW bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd2_edge_low(&self) {
        self.set_bits(Self::GPIO_QSPI_SD2_EDGE_LOW_MASK);
    }

    /// Clear GPIO_QSPI_SD2_EDGE_LOW bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd2_edge_low(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD2_EDGE_LOW_MASK);
    }

    /// **GPIO_QSPI_SD2_LEVEL_HIGH** [25:25]
    #[inline(always)]
    pub fn gpio_qspi_sd2_level_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD2_LEVEL_HIGH_MASK) != 0
    }

    /// Set GPIO_QSPI_SD2_LEVEL_HIGH bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd2_level_high(&self) {
        self.set_bits(Self::GPIO_QSPI_SD2_LEVEL_HIGH_MASK);
    }

    /// Clear GPIO_QSPI_SD2_LEVEL_HIGH bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd2_level_high(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD2_LEVEL_HIGH_MASK);
    }

    /// **GPIO_QSPI_SD2_LEVEL_LOW** [24:24]
    #[inline(always)]
    pub fn gpio_qspi_sd2_level_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD2_LEVEL_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SD2_LEVEL_LOW bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd2_level_low(&self) {
        self.set_bits(Self::GPIO_QSPI_SD2_LEVEL_LOW_MASK);
    }

    /// Clear GPIO_QSPI_SD2_LEVEL_LOW bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd2_level_low(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD2_LEVEL_LOW_MASK);
    }

    /// **GPIO_QSPI_SD1_EDGE_HIGH** [23:23]
    #[inline(always)]
    pub fn gpio_qspi_sd1_edge_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD1_EDGE_HIGH_MASK) != 0
    }

    /// Set GPIO_QSPI_SD1_EDGE_HIGH bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd1_edge_high(&self) {
        self.set_bits(Self::GPIO_QSPI_SD1_EDGE_HIGH_MASK);
    }

    /// Clear GPIO_QSPI_SD1_EDGE_HIGH bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd1_edge_high(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD1_EDGE_HIGH_MASK);
    }

    /// **GPIO_QSPI_SD1_EDGE_LOW** [22:22]
    #[inline(always)]
    pub fn gpio_qspi_sd1_edge_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD1_EDGE_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SD1_EDGE_LOW bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd1_edge_low(&self) {
        self.set_bits(Self::GPIO_QSPI_SD1_EDGE_LOW_MASK);
    }

    /// Clear GPIO_QSPI_SD1_EDGE_LOW bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd1_edge_low(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD1_EDGE_LOW_MASK);
    }

    /// **GPIO_QSPI_SD1_LEVEL_HIGH** [21:21]
    #[inline(always)]
    pub fn gpio_qspi_sd1_level_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD1_LEVEL_HIGH_MASK) != 0
    }

    /// Set GPIO_QSPI_SD1_LEVEL_HIGH bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd1_level_high(&self) {
        self.set_bits(Self::GPIO_QSPI_SD1_LEVEL_HIGH_MASK);
    }

    /// Clear GPIO_QSPI_SD1_LEVEL_HIGH bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd1_level_high(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD1_LEVEL_HIGH_MASK);
    }

    /// **GPIO_QSPI_SD1_LEVEL_LOW** [20:20]
    #[inline(always)]
    pub fn gpio_qspi_sd1_level_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD1_LEVEL_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SD1_LEVEL_LOW bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd1_level_low(&self) {
        self.set_bits(Self::GPIO_QSPI_SD1_LEVEL_LOW_MASK);
    }

    /// Clear GPIO_QSPI_SD1_LEVEL_LOW bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd1_level_low(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD1_LEVEL_LOW_MASK);
    }

    /// **GPIO_QSPI_SD0_EDGE_HIGH** [19:19]
    #[inline(always)]
    pub fn gpio_qspi_sd0_edge_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD0_EDGE_HIGH_MASK) != 0
    }

    /// Set GPIO_QSPI_SD0_EDGE_HIGH bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd0_edge_high(&self) {
        self.set_bits(Self::GPIO_QSPI_SD0_EDGE_HIGH_MASK);
    }

    /// Clear GPIO_QSPI_SD0_EDGE_HIGH bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd0_edge_high(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD0_EDGE_HIGH_MASK);
    }

    /// **GPIO_QSPI_SD0_EDGE_LOW** [18:18]
    #[inline(always)]
    pub fn gpio_qspi_sd0_edge_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD0_EDGE_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SD0_EDGE_LOW bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd0_edge_low(&self) {
        self.set_bits(Self::GPIO_QSPI_SD0_EDGE_LOW_MASK);
    }

    /// Clear GPIO_QSPI_SD0_EDGE_LOW bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd0_edge_low(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD0_EDGE_LOW_MASK);
    }

    /// **GPIO_QSPI_SD0_LEVEL_HIGH** [17:17]
    #[inline(always)]
    pub fn gpio_qspi_sd0_level_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD0_LEVEL_HIGH_MASK) != 0
    }

    /// Set GPIO_QSPI_SD0_LEVEL_HIGH bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd0_level_high(&self) {
        self.set_bits(Self::GPIO_QSPI_SD0_LEVEL_HIGH_MASK);
    }

    /// Clear GPIO_QSPI_SD0_LEVEL_HIGH bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd0_level_high(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD0_LEVEL_HIGH_MASK);
    }

    /// **GPIO_QSPI_SD0_LEVEL_LOW** [16:16]
    #[inline(always)]
    pub fn gpio_qspi_sd0_level_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD0_LEVEL_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SD0_LEVEL_LOW bit
    #[inline(always)]
    pub fn set_gpio_qspi_sd0_level_low(&self) {
        self.set_bits(Self::GPIO_QSPI_SD0_LEVEL_LOW_MASK);
    }

    /// Clear GPIO_QSPI_SD0_LEVEL_LOW bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sd0_level_low(&self) {
        self.clr_bits(Self::GPIO_QSPI_SD0_LEVEL_LOW_MASK);
    }

    /// **GPIO_QSPI_SS_EDGE_HIGH** [15:15]
    #[inline(always)]
    pub fn gpio_qspi_ss_edge_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SS_EDGE_HIGH_MASK) != 0
    }

    /// Set GPIO_QSPI_SS_EDGE_HIGH bit
    #[inline(always)]
    pub fn set_gpio_qspi_ss_edge_high(&self) {
        self.set_bits(Self::GPIO_QSPI_SS_EDGE_HIGH_MASK);
    }

    /// Clear GPIO_QSPI_SS_EDGE_HIGH bit
    #[inline(always)]
    pub fn clr_gpio_qspi_ss_edge_high(&self) {
        self.clr_bits(Self::GPIO_QSPI_SS_EDGE_HIGH_MASK);
    }

    /// **GPIO_QSPI_SS_EDGE_LOW** [14:14]
    #[inline(always)]
    pub fn gpio_qspi_ss_edge_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SS_EDGE_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SS_EDGE_LOW bit
    #[inline(always)]
    pub fn set_gpio_qspi_ss_edge_low(&self) {
        self.set_bits(Self::GPIO_QSPI_SS_EDGE_LOW_MASK);
    }

    /// Clear GPIO_QSPI_SS_EDGE_LOW bit
    #[inline(always)]
    pub fn clr_gpio_qspi_ss_edge_low(&self) {
        self.clr_bits(Self::GPIO_QSPI_SS_EDGE_LOW_MASK);
    }

    /// **GPIO_QSPI_SS_LEVEL_HIGH** [13:13]
    #[inline(always)]
    pub fn gpio_qspi_ss_level_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SS_LEVEL_HIGH_MASK) != 0
    }

    /// Set GPIO_QSPI_SS_LEVEL_HIGH bit
    #[inline(always)]
    pub fn set_gpio_qspi_ss_level_high(&self) {
        self.set_bits(Self::GPIO_QSPI_SS_LEVEL_HIGH_MASK);
    }

    /// Clear GPIO_QSPI_SS_LEVEL_HIGH bit
    #[inline(always)]
    pub fn clr_gpio_qspi_ss_level_high(&self) {
        self.clr_bits(Self::GPIO_QSPI_SS_LEVEL_HIGH_MASK);
    }

    /// **GPIO_QSPI_SS_LEVEL_LOW** [12:12]
    #[inline(always)]
    pub fn gpio_qspi_ss_level_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SS_LEVEL_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SS_LEVEL_LOW bit
    #[inline(always)]
    pub fn set_gpio_qspi_ss_level_low(&self) {
        self.set_bits(Self::GPIO_QSPI_SS_LEVEL_LOW_MASK);
    }

    /// Clear GPIO_QSPI_SS_LEVEL_LOW bit
    #[inline(always)]
    pub fn clr_gpio_qspi_ss_level_low(&self) {
        self.clr_bits(Self::GPIO_QSPI_SS_LEVEL_LOW_MASK);
    }

    /// **GPIO_QSPI_SCLK_EDGE_HIGH** [11:11]
    #[inline(always)]
    pub fn gpio_qspi_sclk_edge_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SCLK_EDGE_HIGH_MASK) != 0
    }

    /// Set GPIO_QSPI_SCLK_EDGE_HIGH bit
    #[inline(always)]
    pub fn set_gpio_qspi_sclk_edge_high(&self) {
        self.set_bits(Self::GPIO_QSPI_SCLK_EDGE_HIGH_MASK);
    }

    /// Clear GPIO_QSPI_SCLK_EDGE_HIGH bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sclk_edge_high(&self) {
        self.clr_bits(Self::GPIO_QSPI_SCLK_EDGE_HIGH_MASK);
    }

    /// **GPIO_QSPI_SCLK_EDGE_LOW** [10:10]
    #[inline(always)]
    pub fn gpio_qspi_sclk_edge_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SCLK_EDGE_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SCLK_EDGE_LOW bit
    #[inline(always)]
    pub fn set_gpio_qspi_sclk_edge_low(&self) {
        self.set_bits(Self::GPIO_QSPI_SCLK_EDGE_LOW_MASK);
    }

    /// Clear GPIO_QSPI_SCLK_EDGE_LOW bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sclk_edge_low(&self) {
        self.clr_bits(Self::GPIO_QSPI_SCLK_EDGE_LOW_MASK);
    }

    /// **GPIO_QSPI_SCLK_LEVEL_HIGH** [9:9]
    #[inline(always)]
    pub fn gpio_qspi_sclk_level_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SCLK_LEVEL_HIGH_MASK) != 0
    }

    /// Set GPIO_QSPI_SCLK_LEVEL_HIGH bit
    #[inline(always)]
    pub fn set_gpio_qspi_sclk_level_high(&self) {
        self.set_bits(Self::GPIO_QSPI_SCLK_LEVEL_HIGH_MASK);
    }

    /// Clear GPIO_QSPI_SCLK_LEVEL_HIGH bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sclk_level_high(&self) {
        self.clr_bits(Self::GPIO_QSPI_SCLK_LEVEL_HIGH_MASK);
    }

    /// **GPIO_QSPI_SCLK_LEVEL_LOW** [8:8]
    #[inline(always)]
    pub fn gpio_qspi_sclk_level_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SCLK_LEVEL_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SCLK_LEVEL_LOW bit
    #[inline(always)]
    pub fn set_gpio_qspi_sclk_level_low(&self) {
        self.set_bits(Self::GPIO_QSPI_SCLK_LEVEL_LOW_MASK);
    }

    /// Clear GPIO_QSPI_SCLK_LEVEL_LOW bit
    #[inline(always)]
    pub fn clr_gpio_qspi_sclk_level_low(&self) {
        self.clr_bits(Self::GPIO_QSPI_SCLK_LEVEL_LOW_MASK);
    }

    /// **USBPHY_DM_EDGE_HIGH** [7:7]
    #[inline(always)]
    pub fn usbphy_dm_edge_high(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DM_EDGE_HIGH_MASK) != 0
    }

    /// Set USBPHY_DM_EDGE_HIGH bit
    #[inline(always)]
    pub fn set_usbphy_dm_edge_high(&self) {
        self.set_bits(Self::USBPHY_DM_EDGE_HIGH_MASK);
    }

    /// Clear USBPHY_DM_EDGE_HIGH bit
    #[inline(always)]
    pub fn clr_usbphy_dm_edge_high(&self) {
        self.clr_bits(Self::USBPHY_DM_EDGE_HIGH_MASK);
    }

    /// **USBPHY_DM_EDGE_LOW** [6:6]
    #[inline(always)]
    pub fn usbphy_dm_edge_low(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DM_EDGE_LOW_MASK) != 0
    }

    /// Set USBPHY_DM_EDGE_LOW bit
    #[inline(always)]
    pub fn set_usbphy_dm_edge_low(&self) {
        self.set_bits(Self::USBPHY_DM_EDGE_LOW_MASK);
    }

    /// Clear USBPHY_DM_EDGE_LOW bit
    #[inline(always)]
    pub fn clr_usbphy_dm_edge_low(&self) {
        self.clr_bits(Self::USBPHY_DM_EDGE_LOW_MASK);
    }

    /// **USBPHY_DM_LEVEL_HIGH** [5:5]
    #[inline(always)]
    pub fn usbphy_dm_level_high(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DM_LEVEL_HIGH_MASK) != 0
    }

    /// Set USBPHY_DM_LEVEL_HIGH bit
    #[inline(always)]
    pub fn set_usbphy_dm_level_high(&self) {
        self.set_bits(Self::USBPHY_DM_LEVEL_HIGH_MASK);
    }

    /// Clear USBPHY_DM_LEVEL_HIGH bit
    #[inline(always)]
    pub fn clr_usbphy_dm_level_high(&self) {
        self.clr_bits(Self::USBPHY_DM_LEVEL_HIGH_MASK);
    }

    /// **USBPHY_DM_LEVEL_LOW** [4:4]
    #[inline(always)]
    pub fn usbphy_dm_level_low(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DM_LEVEL_LOW_MASK) != 0
    }

    /// Set USBPHY_DM_LEVEL_LOW bit
    #[inline(always)]
    pub fn set_usbphy_dm_level_low(&self) {
        self.set_bits(Self::USBPHY_DM_LEVEL_LOW_MASK);
    }

    /// Clear USBPHY_DM_LEVEL_LOW bit
    #[inline(always)]
    pub fn clr_usbphy_dm_level_low(&self) {
        self.clr_bits(Self::USBPHY_DM_LEVEL_LOW_MASK);
    }

    /// **USBPHY_DP_EDGE_HIGH** [3:3]
    #[inline(always)]
    pub fn usbphy_dp_edge_high(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DP_EDGE_HIGH_MASK) != 0
    }

    /// Set USBPHY_DP_EDGE_HIGH bit
    #[inline(always)]
    pub fn set_usbphy_dp_edge_high(&self) {
        self.set_bits(Self::USBPHY_DP_EDGE_HIGH_MASK);
    }

    /// Clear USBPHY_DP_EDGE_HIGH bit
    #[inline(always)]
    pub fn clr_usbphy_dp_edge_high(&self) {
        self.clr_bits(Self::USBPHY_DP_EDGE_HIGH_MASK);
    }

    /// **USBPHY_DP_EDGE_LOW** [2:2]
    #[inline(always)]
    pub fn usbphy_dp_edge_low(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DP_EDGE_LOW_MASK) != 0
    }

    /// Set USBPHY_DP_EDGE_LOW bit
    #[inline(always)]
    pub fn set_usbphy_dp_edge_low(&self) {
        self.set_bits(Self::USBPHY_DP_EDGE_LOW_MASK);
    }

    /// Clear USBPHY_DP_EDGE_LOW bit
    #[inline(always)]
    pub fn clr_usbphy_dp_edge_low(&self) {
        self.clr_bits(Self::USBPHY_DP_EDGE_LOW_MASK);
    }

    /// **USBPHY_DP_LEVEL_HIGH** [1:1]
    #[inline(always)]
    pub fn usbphy_dp_level_high(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DP_LEVEL_HIGH_MASK) != 0
    }

    /// Set USBPHY_DP_LEVEL_HIGH bit
    #[inline(always)]
    pub fn set_usbphy_dp_level_high(&self) {
        self.set_bits(Self::USBPHY_DP_LEVEL_HIGH_MASK);
    }

    /// Clear USBPHY_DP_LEVEL_HIGH bit
    #[inline(always)]
    pub fn clr_usbphy_dp_level_high(&self) {
        self.clr_bits(Self::USBPHY_DP_LEVEL_HIGH_MASK);
    }

    /// **USBPHY_DP_LEVEL_LOW** [0:0]
    #[inline(always)]
    pub fn usbphy_dp_level_low(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DP_LEVEL_LOW_MASK) != 0
    }

    /// Set USBPHY_DP_LEVEL_LOW bit
    #[inline(always)]
    pub fn set_usbphy_dp_level_low(&self) {
        self.set_bits(Self::USBPHY_DP_LEVEL_LOW_MASK);
    }

    /// Clear USBPHY_DP_LEVEL_LOW bit
    #[inline(always)]
    pub fn clr_usbphy_dp_level_low(&self) {
        self.clr_bits(Self::USBPHY_DP_LEVEL_LOW_MASK);
    }

}

/// DORMANT_WAKE_INTF value type for batch read/modify/write operations
///
/// Use `dormantwakeintf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dormantwakeintf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DormantWakeIntfVal {
    bits: u32,
}

impl DormantWakeIntfVal {
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

    /// Read GPIO_QSPI_SD3_EDGE_HIGH [31:31]
    #[inline(always)]
    pub const fn gpio_qspi_sd3_edge_high(&self) -> bool {
        (self.bits & DormantWakeIntf::GPIO_QSPI_SD3_EDGE_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD3_EDGE_LOW [30:30]
    #[inline(always)]
    pub const fn gpio_qspi_sd3_edge_low(&self) -> bool {
        (self.bits & DormantWakeIntf::GPIO_QSPI_SD3_EDGE_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SD3_LEVEL_HIGH [29:29]
    #[inline(always)]
    pub const fn gpio_qspi_sd3_level_high(&self) -> bool {
        (self.bits & DormantWakeIntf::GPIO_QSPI_SD3_LEVEL_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD3_LEVEL_LOW [28:28]
    #[inline(always)]
    pub const fn gpio_qspi_sd3_level_low(&self) -> bool {
        (self.bits & DormantWakeIntf::GPIO_QSPI_SD3_LEVEL_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SD2_EDGE_HIGH [27:27]
    #[inline(always)]
    pub const fn gpio_qspi_sd2_edge_high(&self) -> bool {
        (self.bits & DormantWakeIntf::GPIO_QSPI_SD2_EDGE_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD2_EDGE_LOW [26:26]
    #[inline(always)]
    pub const fn gpio_qspi_sd2_edge_low(&self) -> bool {
        (self.bits & DormantWakeIntf::GPIO_QSPI_SD2_EDGE_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SD2_LEVEL_HIGH [25:25]
    #[inline(always)]
    pub const fn gpio_qspi_sd2_level_high(&self) -> bool {
        (self.bits & DormantWakeIntf::GPIO_QSPI_SD2_LEVEL_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD2_LEVEL_LOW [24:24]
    #[inline(always)]
    pub const fn gpio_qspi_sd2_level_low(&self) -> bool {
        (self.bits & DormantWakeIntf::GPIO_QSPI_SD2_LEVEL_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SD1_EDGE_HIGH [23:23]
    #[inline(always)]
    pub const fn gpio_qspi_sd1_edge_high(&self) -> bool {
        (self.bits & DormantWakeIntf::GPIO_QSPI_SD1_EDGE_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD1_EDGE_LOW [22:22]
    #[inline(always)]
    pub const fn gpio_qspi_sd1_edge_low(&self) -> bool {
        (self.bits & DormantWakeIntf::GPIO_QSPI_SD1_EDGE_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SD1_LEVEL_HIGH [21:21]
    #[inline(always)]
    pub const fn gpio_qspi_sd1_level_high(&self) -> bool {
        (self.bits & DormantWakeIntf::GPIO_QSPI_SD1_LEVEL_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD1_LEVEL_LOW [20:20]
    #[inline(always)]
    pub const fn gpio_qspi_sd1_level_low(&self) -> bool {
        (self.bits & DormantWakeIntf::GPIO_QSPI_SD1_LEVEL_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SD0_EDGE_HIGH [19:19]
    #[inline(always)]
    pub const fn gpio_qspi_sd0_edge_high(&self) -> bool {
        (self.bits & DormantWakeIntf::GPIO_QSPI_SD0_EDGE_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD0_EDGE_LOW [18:18]
    #[inline(always)]
    pub const fn gpio_qspi_sd0_edge_low(&self) -> bool {
        (self.bits & DormantWakeIntf::GPIO_QSPI_SD0_EDGE_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SD0_LEVEL_HIGH [17:17]
    #[inline(always)]
    pub const fn gpio_qspi_sd0_level_high(&self) -> bool {
        (self.bits & DormantWakeIntf::GPIO_QSPI_SD0_LEVEL_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD0_LEVEL_LOW [16:16]
    #[inline(always)]
    pub const fn gpio_qspi_sd0_level_low(&self) -> bool {
        (self.bits & DormantWakeIntf::GPIO_QSPI_SD0_LEVEL_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SS_EDGE_HIGH [15:15]
    #[inline(always)]
    pub const fn gpio_qspi_ss_edge_high(&self) -> bool {
        (self.bits & DormantWakeIntf::GPIO_QSPI_SS_EDGE_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SS_EDGE_LOW [14:14]
    #[inline(always)]
    pub const fn gpio_qspi_ss_edge_low(&self) -> bool {
        (self.bits & DormantWakeIntf::GPIO_QSPI_SS_EDGE_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SS_LEVEL_HIGH [13:13]
    #[inline(always)]
    pub const fn gpio_qspi_ss_level_high(&self) -> bool {
        (self.bits & DormantWakeIntf::GPIO_QSPI_SS_LEVEL_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SS_LEVEL_LOW [12:12]
    #[inline(always)]
    pub const fn gpio_qspi_ss_level_low(&self) -> bool {
        (self.bits & DormantWakeIntf::GPIO_QSPI_SS_LEVEL_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SCLK_EDGE_HIGH [11:11]
    #[inline(always)]
    pub const fn gpio_qspi_sclk_edge_high(&self) -> bool {
        (self.bits & DormantWakeIntf::GPIO_QSPI_SCLK_EDGE_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SCLK_EDGE_LOW [10:10]
    #[inline(always)]
    pub const fn gpio_qspi_sclk_edge_low(&self) -> bool {
        (self.bits & DormantWakeIntf::GPIO_QSPI_SCLK_EDGE_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SCLK_LEVEL_HIGH [9:9]
    #[inline(always)]
    pub const fn gpio_qspi_sclk_level_high(&self) -> bool {
        (self.bits & DormantWakeIntf::GPIO_QSPI_SCLK_LEVEL_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SCLK_LEVEL_LOW [8:8]
    #[inline(always)]
    pub const fn gpio_qspi_sclk_level_low(&self) -> bool {
        (self.bits & DormantWakeIntf::GPIO_QSPI_SCLK_LEVEL_LOW_MASK) != 0
    }

    /// Read USBPHY_DM_EDGE_HIGH [7:7]
    #[inline(always)]
    pub const fn usbphy_dm_edge_high(&self) -> bool {
        (self.bits & DormantWakeIntf::USBPHY_DM_EDGE_HIGH_MASK) != 0
    }

    /// Read USBPHY_DM_EDGE_LOW [6:6]
    #[inline(always)]
    pub const fn usbphy_dm_edge_low(&self) -> bool {
        (self.bits & DormantWakeIntf::USBPHY_DM_EDGE_LOW_MASK) != 0
    }

    /// Read USBPHY_DM_LEVEL_HIGH [5:5]
    #[inline(always)]
    pub const fn usbphy_dm_level_high(&self) -> bool {
        (self.bits & DormantWakeIntf::USBPHY_DM_LEVEL_HIGH_MASK) != 0
    }

    /// Read USBPHY_DM_LEVEL_LOW [4:4]
    #[inline(always)]
    pub const fn usbphy_dm_level_low(&self) -> bool {
        (self.bits & DormantWakeIntf::USBPHY_DM_LEVEL_LOW_MASK) != 0
    }

    /// Read USBPHY_DP_EDGE_HIGH [3:3]
    #[inline(always)]
    pub const fn usbphy_dp_edge_high(&self) -> bool {
        (self.bits & DormantWakeIntf::USBPHY_DP_EDGE_HIGH_MASK) != 0
    }

    /// Read USBPHY_DP_EDGE_LOW [2:2]
    #[inline(always)]
    pub const fn usbphy_dp_edge_low(&self) -> bool {
        (self.bits & DormantWakeIntf::USBPHY_DP_EDGE_LOW_MASK) != 0
    }

    /// Read USBPHY_DP_LEVEL_HIGH [1:1]
    #[inline(always)]
    pub const fn usbphy_dp_level_high(&self) -> bool {
        (self.bits & DormantWakeIntf::USBPHY_DP_LEVEL_HIGH_MASK) != 0
    }

    /// Read USBPHY_DP_LEVEL_LOW [0:0]
    #[inline(always)]
    pub const fn usbphy_dp_level_low(&self) -> bool {
        (self.bits & DormantWakeIntf::USBPHY_DP_LEVEL_LOW_MASK) != 0
    }

    /// Set GPIO_QSPI_SD3_EDGE_HIGH [31:31]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd3_edge_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DormantWakeIntf::GPIO_QSPI_SD3_EDGE_HIGH_MASK
            } else {
                self.bits & !DormantWakeIntf::GPIO_QSPI_SD3_EDGE_HIGH_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD3_EDGE_LOW [30:30]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd3_edge_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DormantWakeIntf::GPIO_QSPI_SD3_EDGE_LOW_MASK
            } else {
                self.bits & !DormantWakeIntf::GPIO_QSPI_SD3_EDGE_LOW_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD3_LEVEL_HIGH [29:29]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd3_level_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DormantWakeIntf::GPIO_QSPI_SD3_LEVEL_HIGH_MASK
            } else {
                self.bits & !DormantWakeIntf::GPIO_QSPI_SD3_LEVEL_HIGH_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD3_LEVEL_LOW [28:28]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd3_level_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DormantWakeIntf::GPIO_QSPI_SD3_LEVEL_LOW_MASK
            } else {
                self.bits & !DormantWakeIntf::GPIO_QSPI_SD3_LEVEL_LOW_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD2_EDGE_HIGH [27:27]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd2_edge_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DormantWakeIntf::GPIO_QSPI_SD2_EDGE_HIGH_MASK
            } else {
                self.bits & !DormantWakeIntf::GPIO_QSPI_SD2_EDGE_HIGH_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD2_EDGE_LOW [26:26]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd2_edge_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DormantWakeIntf::GPIO_QSPI_SD2_EDGE_LOW_MASK
            } else {
                self.bits & !DormantWakeIntf::GPIO_QSPI_SD2_EDGE_LOW_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD2_LEVEL_HIGH [25:25]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd2_level_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DormantWakeIntf::GPIO_QSPI_SD2_LEVEL_HIGH_MASK
            } else {
                self.bits & !DormantWakeIntf::GPIO_QSPI_SD2_LEVEL_HIGH_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD2_LEVEL_LOW [24:24]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd2_level_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DormantWakeIntf::GPIO_QSPI_SD2_LEVEL_LOW_MASK
            } else {
                self.bits & !DormantWakeIntf::GPIO_QSPI_SD2_LEVEL_LOW_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD1_EDGE_HIGH [23:23]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd1_edge_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DormantWakeIntf::GPIO_QSPI_SD1_EDGE_HIGH_MASK
            } else {
                self.bits & !DormantWakeIntf::GPIO_QSPI_SD1_EDGE_HIGH_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD1_EDGE_LOW [22:22]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd1_edge_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DormantWakeIntf::GPIO_QSPI_SD1_EDGE_LOW_MASK
            } else {
                self.bits & !DormantWakeIntf::GPIO_QSPI_SD1_EDGE_LOW_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD1_LEVEL_HIGH [21:21]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd1_level_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DormantWakeIntf::GPIO_QSPI_SD1_LEVEL_HIGH_MASK
            } else {
                self.bits & !DormantWakeIntf::GPIO_QSPI_SD1_LEVEL_HIGH_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD1_LEVEL_LOW [20:20]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd1_level_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DormantWakeIntf::GPIO_QSPI_SD1_LEVEL_LOW_MASK
            } else {
                self.bits & !DormantWakeIntf::GPIO_QSPI_SD1_LEVEL_LOW_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD0_EDGE_HIGH [19:19]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd0_edge_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DormantWakeIntf::GPIO_QSPI_SD0_EDGE_HIGH_MASK
            } else {
                self.bits & !DormantWakeIntf::GPIO_QSPI_SD0_EDGE_HIGH_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD0_EDGE_LOW [18:18]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd0_edge_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DormantWakeIntf::GPIO_QSPI_SD0_EDGE_LOW_MASK
            } else {
                self.bits & !DormantWakeIntf::GPIO_QSPI_SD0_EDGE_LOW_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD0_LEVEL_HIGH [17:17]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd0_level_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DormantWakeIntf::GPIO_QSPI_SD0_LEVEL_HIGH_MASK
            } else {
                self.bits & !DormantWakeIntf::GPIO_QSPI_SD0_LEVEL_HIGH_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SD0_LEVEL_LOW [16:16]
    #[inline(always)]
    pub const fn with_gpio_qspi_sd0_level_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DormantWakeIntf::GPIO_QSPI_SD0_LEVEL_LOW_MASK
            } else {
                self.bits & !DormantWakeIntf::GPIO_QSPI_SD0_LEVEL_LOW_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SS_EDGE_HIGH [15:15]
    #[inline(always)]
    pub const fn with_gpio_qspi_ss_edge_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DormantWakeIntf::GPIO_QSPI_SS_EDGE_HIGH_MASK
            } else {
                self.bits & !DormantWakeIntf::GPIO_QSPI_SS_EDGE_HIGH_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SS_EDGE_LOW [14:14]
    #[inline(always)]
    pub const fn with_gpio_qspi_ss_edge_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DormantWakeIntf::GPIO_QSPI_SS_EDGE_LOW_MASK
            } else {
                self.bits & !DormantWakeIntf::GPIO_QSPI_SS_EDGE_LOW_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SS_LEVEL_HIGH [13:13]
    #[inline(always)]
    pub const fn with_gpio_qspi_ss_level_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DormantWakeIntf::GPIO_QSPI_SS_LEVEL_HIGH_MASK
            } else {
                self.bits & !DormantWakeIntf::GPIO_QSPI_SS_LEVEL_HIGH_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SS_LEVEL_LOW [12:12]
    #[inline(always)]
    pub const fn with_gpio_qspi_ss_level_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DormantWakeIntf::GPIO_QSPI_SS_LEVEL_LOW_MASK
            } else {
                self.bits & !DormantWakeIntf::GPIO_QSPI_SS_LEVEL_LOW_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SCLK_EDGE_HIGH [11:11]
    #[inline(always)]
    pub const fn with_gpio_qspi_sclk_edge_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DormantWakeIntf::GPIO_QSPI_SCLK_EDGE_HIGH_MASK
            } else {
                self.bits & !DormantWakeIntf::GPIO_QSPI_SCLK_EDGE_HIGH_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SCLK_EDGE_LOW [10:10]
    #[inline(always)]
    pub const fn with_gpio_qspi_sclk_edge_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DormantWakeIntf::GPIO_QSPI_SCLK_EDGE_LOW_MASK
            } else {
                self.bits & !DormantWakeIntf::GPIO_QSPI_SCLK_EDGE_LOW_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SCLK_LEVEL_HIGH [9:9]
    #[inline(always)]
    pub const fn with_gpio_qspi_sclk_level_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DormantWakeIntf::GPIO_QSPI_SCLK_LEVEL_HIGH_MASK
            } else {
                self.bits & !DormantWakeIntf::GPIO_QSPI_SCLK_LEVEL_HIGH_MASK
            }
        }
    }

    /// Set GPIO_QSPI_SCLK_LEVEL_LOW [8:8]
    #[inline(always)]
    pub const fn with_gpio_qspi_sclk_level_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DormantWakeIntf::GPIO_QSPI_SCLK_LEVEL_LOW_MASK
            } else {
                self.bits & !DormantWakeIntf::GPIO_QSPI_SCLK_LEVEL_LOW_MASK
            }
        }
    }

    /// Set USBPHY_DM_EDGE_HIGH [7:7]
    #[inline(always)]
    pub const fn with_usbphy_dm_edge_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DormantWakeIntf::USBPHY_DM_EDGE_HIGH_MASK
            } else {
                self.bits & !DormantWakeIntf::USBPHY_DM_EDGE_HIGH_MASK
            }
        }
    }

    /// Set USBPHY_DM_EDGE_LOW [6:6]
    #[inline(always)]
    pub const fn with_usbphy_dm_edge_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DormantWakeIntf::USBPHY_DM_EDGE_LOW_MASK
            } else {
                self.bits & !DormantWakeIntf::USBPHY_DM_EDGE_LOW_MASK
            }
        }
    }

    /// Set USBPHY_DM_LEVEL_HIGH [5:5]
    #[inline(always)]
    pub const fn with_usbphy_dm_level_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DormantWakeIntf::USBPHY_DM_LEVEL_HIGH_MASK
            } else {
                self.bits & !DormantWakeIntf::USBPHY_DM_LEVEL_HIGH_MASK
            }
        }
    }

    /// Set USBPHY_DM_LEVEL_LOW [4:4]
    #[inline(always)]
    pub const fn with_usbphy_dm_level_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DormantWakeIntf::USBPHY_DM_LEVEL_LOW_MASK
            } else {
                self.bits & !DormantWakeIntf::USBPHY_DM_LEVEL_LOW_MASK
            }
        }
    }

    /// Set USBPHY_DP_EDGE_HIGH [3:3]
    #[inline(always)]
    pub const fn with_usbphy_dp_edge_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DormantWakeIntf::USBPHY_DP_EDGE_HIGH_MASK
            } else {
                self.bits & !DormantWakeIntf::USBPHY_DP_EDGE_HIGH_MASK
            }
        }
    }

    /// Set USBPHY_DP_EDGE_LOW [2:2]
    #[inline(always)]
    pub const fn with_usbphy_dp_edge_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DormantWakeIntf::USBPHY_DP_EDGE_LOW_MASK
            } else {
                self.bits & !DormantWakeIntf::USBPHY_DP_EDGE_LOW_MASK
            }
        }
    }

    /// Set USBPHY_DP_LEVEL_HIGH [1:1]
    #[inline(always)]
    pub const fn with_usbphy_dp_level_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DormantWakeIntf::USBPHY_DP_LEVEL_HIGH_MASK
            } else {
                self.bits & !DormantWakeIntf::USBPHY_DP_LEVEL_HIGH_MASK
            }
        }
    }

    /// Set USBPHY_DP_LEVEL_LOW [0:0]
    #[inline(always)]
    pub const fn with_usbphy_dp_level_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DormantWakeIntf::USBPHY_DP_LEVEL_LOW_MASK
            } else {
                self.bits & !DormantWakeIntf::USBPHY_DP_LEVEL_LOW_MASK
            }
        }
    }

}

impl Default for DormantWakeIntfVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl DormantWakeIntf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> DormantWakeIntfVal {
        DormantWakeIntfVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: DormantWakeIntfVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(DormantWakeIntfVal) -> DormantWakeIntfVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// DORMANT_WAKE_INTS Register
///
/// Interrupt status after masking & forcing for dormant_wake
///
/// Address offset: 0x023c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **GPIO_QSPI_SD3_EDGE_HIGH** [31] (RO): 
/// - **GPIO_QSPI_SD3_EDGE_LOW** [30] (RO): 
/// - **GPIO_QSPI_SD3_LEVEL_HIGH** [29] (RO): 
/// - **GPIO_QSPI_SD3_LEVEL_LOW** [28] (RO): 
/// - **GPIO_QSPI_SD2_EDGE_HIGH** [27] (RO): 
/// - **GPIO_QSPI_SD2_EDGE_LOW** [26] (RO): 
/// - **GPIO_QSPI_SD2_LEVEL_HIGH** [25] (RO): 
/// - **GPIO_QSPI_SD2_LEVEL_LOW** [24] (RO): 
/// - **GPIO_QSPI_SD1_EDGE_HIGH** [23] (RO): 
/// - **GPIO_QSPI_SD1_EDGE_LOW** [22] (RO): 
/// - **GPIO_QSPI_SD1_LEVEL_HIGH** [21] (RO): 
/// - **GPIO_QSPI_SD1_LEVEL_LOW** [20] (RO): 
/// - **GPIO_QSPI_SD0_EDGE_HIGH** [19] (RO): 
/// - **GPIO_QSPI_SD0_EDGE_LOW** [18] (RO): 
/// - **GPIO_QSPI_SD0_LEVEL_HIGH** [17] (RO): 
/// - **GPIO_QSPI_SD0_LEVEL_LOW** [16] (RO): 
/// - **GPIO_QSPI_SS_EDGE_HIGH** [15] (RO): 
/// - **GPIO_QSPI_SS_EDGE_LOW** [14] (RO): 
/// - **GPIO_QSPI_SS_LEVEL_HIGH** [13] (RO): 
/// - **GPIO_QSPI_SS_LEVEL_LOW** [12] (RO): 
/// - **GPIO_QSPI_SCLK_EDGE_HIGH** [11] (RO): 
/// - **GPIO_QSPI_SCLK_EDGE_LOW** [10] (RO): 
/// - **GPIO_QSPI_SCLK_LEVEL_HIGH** [9] (RO): 
/// - **GPIO_QSPI_SCLK_LEVEL_LOW** [8] (RO): 
/// - **USBPHY_DM_EDGE_HIGH** [7] (RO): 
/// - **USBPHY_DM_EDGE_LOW** [6] (RO): 
/// - **USBPHY_DM_LEVEL_HIGH** [5] (RO): 
/// - **USBPHY_DM_LEVEL_LOW** [4] (RO): 
/// - **USBPHY_DP_EDGE_HIGH** [3] (RO): 
/// - **USBPHY_DP_EDGE_LOW** [2] (RO): 
/// - **USBPHY_DP_LEVEL_HIGH** [1] (RO): 
/// - **USBPHY_DP_LEVEL_LOW** [0] (RO): 
#[repr(transparent)]
pub struct DormantWakeInts {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for DormantWakeInts {}
unsafe impl Sync for DormantWakeInts {}

impl DormantWakeInts {
    // ======== Field Constants ========

    /// Bits [31:31], mask 0x80000000
    pub const GPIO_QSPI_SD3_EDGE_HIGH_OFFSET: u32 = 31;
    pub const GPIO_QSPI_SD3_EDGE_HIGH_MASK: u32 = 0x80000000;
    pub const GPIO_QSPI_SD3_EDGE_HIGH: u32 = 1 << 31;

    /// Bits [30:30], mask 0x40000000
    pub const GPIO_QSPI_SD3_EDGE_LOW_OFFSET: u32 = 30;
    pub const GPIO_QSPI_SD3_EDGE_LOW_MASK: u32 = 0x40000000;
    pub const GPIO_QSPI_SD3_EDGE_LOW: u32 = 1 << 30;

    /// Bits [29:29], mask 0x20000000
    pub const GPIO_QSPI_SD3_LEVEL_HIGH_OFFSET: u32 = 29;
    pub const GPIO_QSPI_SD3_LEVEL_HIGH_MASK: u32 = 0x20000000;
    pub const GPIO_QSPI_SD3_LEVEL_HIGH: u32 = 1 << 29;

    /// Bits [28:28], mask 0x10000000
    pub const GPIO_QSPI_SD3_LEVEL_LOW_OFFSET: u32 = 28;
    pub const GPIO_QSPI_SD3_LEVEL_LOW_MASK: u32 = 0x10000000;
    pub const GPIO_QSPI_SD3_LEVEL_LOW: u32 = 1 << 28;

    /// Bits [27:27], mask 0x08000000
    pub const GPIO_QSPI_SD2_EDGE_HIGH_OFFSET: u32 = 27;
    pub const GPIO_QSPI_SD2_EDGE_HIGH_MASK: u32 = 0x08000000;
    pub const GPIO_QSPI_SD2_EDGE_HIGH: u32 = 1 << 27;

    /// Bits [26:26], mask 0x04000000
    pub const GPIO_QSPI_SD2_EDGE_LOW_OFFSET: u32 = 26;
    pub const GPIO_QSPI_SD2_EDGE_LOW_MASK: u32 = 0x04000000;
    pub const GPIO_QSPI_SD2_EDGE_LOW: u32 = 1 << 26;

    /// Bits [25:25], mask 0x02000000
    pub const GPIO_QSPI_SD2_LEVEL_HIGH_OFFSET: u32 = 25;
    pub const GPIO_QSPI_SD2_LEVEL_HIGH_MASK: u32 = 0x02000000;
    pub const GPIO_QSPI_SD2_LEVEL_HIGH: u32 = 1 << 25;

    /// Bits [24:24], mask 0x01000000
    pub const GPIO_QSPI_SD2_LEVEL_LOW_OFFSET: u32 = 24;
    pub const GPIO_QSPI_SD2_LEVEL_LOW_MASK: u32 = 0x01000000;
    pub const GPIO_QSPI_SD2_LEVEL_LOW: u32 = 1 << 24;

    /// Bits [23:23], mask 0x00800000
    pub const GPIO_QSPI_SD1_EDGE_HIGH_OFFSET: u32 = 23;
    pub const GPIO_QSPI_SD1_EDGE_HIGH_MASK: u32 = 0x00800000;
    pub const GPIO_QSPI_SD1_EDGE_HIGH: u32 = 1 << 23;

    /// Bits [22:22], mask 0x00400000
    pub const GPIO_QSPI_SD1_EDGE_LOW_OFFSET: u32 = 22;
    pub const GPIO_QSPI_SD1_EDGE_LOW_MASK: u32 = 0x00400000;
    pub const GPIO_QSPI_SD1_EDGE_LOW: u32 = 1 << 22;

    /// Bits [21:21], mask 0x00200000
    pub const GPIO_QSPI_SD1_LEVEL_HIGH_OFFSET: u32 = 21;
    pub const GPIO_QSPI_SD1_LEVEL_HIGH_MASK: u32 = 0x00200000;
    pub const GPIO_QSPI_SD1_LEVEL_HIGH: u32 = 1 << 21;

    /// Bits [20:20], mask 0x00100000
    pub const GPIO_QSPI_SD1_LEVEL_LOW_OFFSET: u32 = 20;
    pub const GPIO_QSPI_SD1_LEVEL_LOW_MASK: u32 = 0x00100000;
    pub const GPIO_QSPI_SD1_LEVEL_LOW: u32 = 1 << 20;

    /// Bits [19:19], mask 0x00080000
    pub const GPIO_QSPI_SD0_EDGE_HIGH_OFFSET: u32 = 19;
    pub const GPIO_QSPI_SD0_EDGE_HIGH_MASK: u32 = 0x00080000;
    pub const GPIO_QSPI_SD0_EDGE_HIGH: u32 = 1 << 19;

    /// Bits [18:18], mask 0x00040000
    pub const GPIO_QSPI_SD0_EDGE_LOW_OFFSET: u32 = 18;
    pub const GPIO_QSPI_SD0_EDGE_LOW_MASK: u32 = 0x00040000;
    pub const GPIO_QSPI_SD0_EDGE_LOW: u32 = 1 << 18;

    /// Bits [17:17], mask 0x00020000
    pub const GPIO_QSPI_SD0_LEVEL_HIGH_OFFSET: u32 = 17;
    pub const GPIO_QSPI_SD0_LEVEL_HIGH_MASK: u32 = 0x00020000;
    pub const GPIO_QSPI_SD0_LEVEL_HIGH: u32 = 1 << 17;

    /// Bits [16:16], mask 0x00010000
    pub const GPIO_QSPI_SD0_LEVEL_LOW_OFFSET: u32 = 16;
    pub const GPIO_QSPI_SD0_LEVEL_LOW_MASK: u32 = 0x00010000;
    pub const GPIO_QSPI_SD0_LEVEL_LOW: u32 = 1 << 16;

    /// Bits [15:15], mask 0x00008000
    pub const GPIO_QSPI_SS_EDGE_HIGH_OFFSET: u32 = 15;
    pub const GPIO_QSPI_SS_EDGE_HIGH_MASK: u32 = 0x00008000;
    pub const GPIO_QSPI_SS_EDGE_HIGH: u32 = 1 << 15;

    /// Bits [14:14], mask 0x00004000
    pub const GPIO_QSPI_SS_EDGE_LOW_OFFSET: u32 = 14;
    pub const GPIO_QSPI_SS_EDGE_LOW_MASK: u32 = 0x00004000;
    pub const GPIO_QSPI_SS_EDGE_LOW: u32 = 1 << 14;

    /// Bits [13:13], mask 0x00002000
    pub const GPIO_QSPI_SS_LEVEL_HIGH_OFFSET: u32 = 13;
    pub const GPIO_QSPI_SS_LEVEL_HIGH_MASK: u32 = 0x00002000;
    pub const GPIO_QSPI_SS_LEVEL_HIGH: u32 = 1 << 13;

    /// Bits [12:12], mask 0x00001000
    pub const GPIO_QSPI_SS_LEVEL_LOW_OFFSET: u32 = 12;
    pub const GPIO_QSPI_SS_LEVEL_LOW_MASK: u32 = 0x00001000;
    pub const GPIO_QSPI_SS_LEVEL_LOW: u32 = 1 << 12;

    /// Bits [11:11], mask 0x00000800
    pub const GPIO_QSPI_SCLK_EDGE_HIGH_OFFSET: u32 = 11;
    pub const GPIO_QSPI_SCLK_EDGE_HIGH_MASK: u32 = 0x00000800;
    pub const GPIO_QSPI_SCLK_EDGE_HIGH: u32 = 1 << 11;

    /// Bits [10:10], mask 0x00000400
    pub const GPIO_QSPI_SCLK_EDGE_LOW_OFFSET: u32 = 10;
    pub const GPIO_QSPI_SCLK_EDGE_LOW_MASK: u32 = 0x00000400;
    pub const GPIO_QSPI_SCLK_EDGE_LOW: u32 = 1 << 10;

    /// Bits [9:9], mask 0x00000200
    pub const GPIO_QSPI_SCLK_LEVEL_HIGH_OFFSET: u32 = 9;
    pub const GPIO_QSPI_SCLK_LEVEL_HIGH_MASK: u32 = 0x00000200;
    pub const GPIO_QSPI_SCLK_LEVEL_HIGH: u32 = 1 << 9;

    /// Bits [8:8], mask 0x00000100
    pub const GPIO_QSPI_SCLK_LEVEL_LOW_OFFSET: u32 = 8;
    pub const GPIO_QSPI_SCLK_LEVEL_LOW_MASK: u32 = 0x00000100;
    pub const GPIO_QSPI_SCLK_LEVEL_LOW: u32 = 1 << 8;

    /// Bits [7:7], mask 0x00000080
    pub const USBPHY_DM_EDGE_HIGH_OFFSET: u32 = 7;
    pub const USBPHY_DM_EDGE_HIGH_MASK: u32 = 0x00000080;
    pub const USBPHY_DM_EDGE_HIGH: u32 = 1 << 7;

    /// Bits [6:6], mask 0x00000040
    pub const USBPHY_DM_EDGE_LOW_OFFSET: u32 = 6;
    pub const USBPHY_DM_EDGE_LOW_MASK: u32 = 0x00000040;
    pub const USBPHY_DM_EDGE_LOW: u32 = 1 << 6;

    /// Bits [5:5], mask 0x00000020
    pub const USBPHY_DM_LEVEL_HIGH_OFFSET: u32 = 5;
    pub const USBPHY_DM_LEVEL_HIGH_MASK: u32 = 0x00000020;
    pub const USBPHY_DM_LEVEL_HIGH: u32 = 1 << 5;

    /// Bits [4:4], mask 0x00000010
    pub const USBPHY_DM_LEVEL_LOW_OFFSET: u32 = 4;
    pub const USBPHY_DM_LEVEL_LOW_MASK: u32 = 0x00000010;
    pub const USBPHY_DM_LEVEL_LOW: u32 = 1 << 4;

    /// Bits [3:3], mask 0x00000008
    pub const USBPHY_DP_EDGE_HIGH_OFFSET: u32 = 3;
    pub const USBPHY_DP_EDGE_HIGH_MASK: u32 = 0x00000008;
    pub const USBPHY_DP_EDGE_HIGH: u32 = 1 << 3;

    /// Bits [2:2], mask 0x00000004
    pub const USBPHY_DP_EDGE_LOW_OFFSET: u32 = 2;
    pub const USBPHY_DP_EDGE_LOW_MASK: u32 = 0x00000004;
    pub const USBPHY_DP_EDGE_LOW: u32 = 1 << 2;

    /// Bits [1:1], mask 0x00000002
    pub const USBPHY_DP_LEVEL_HIGH_OFFSET: u32 = 1;
    pub const USBPHY_DP_LEVEL_HIGH_MASK: u32 = 0x00000002;
    pub const USBPHY_DP_LEVEL_HIGH: u32 = 1 << 1;

    /// Bits [0:0], mask 0x00000001
    pub const USBPHY_DP_LEVEL_LOW_OFFSET: u32 = 0;
    pub const USBPHY_DP_LEVEL_LOW_MASK: u32 = 0x00000001;
    pub const USBPHY_DP_LEVEL_LOW: u32 = 1 << 0;

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

    /// **GPIO_QSPI_SD3_EDGE_HIGH** [31:31]
    #[inline(always)]
    pub fn gpio_qspi_sd3_edge_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD3_EDGE_HIGH_MASK) != 0
    }

    /// **GPIO_QSPI_SD3_EDGE_LOW** [30:30]
    #[inline(always)]
    pub fn gpio_qspi_sd3_edge_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD3_EDGE_LOW_MASK) != 0
    }

    /// **GPIO_QSPI_SD3_LEVEL_HIGH** [29:29]
    #[inline(always)]
    pub fn gpio_qspi_sd3_level_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD3_LEVEL_HIGH_MASK) != 0
    }

    /// **GPIO_QSPI_SD3_LEVEL_LOW** [28:28]
    #[inline(always)]
    pub fn gpio_qspi_sd3_level_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD3_LEVEL_LOW_MASK) != 0
    }

    /// **GPIO_QSPI_SD2_EDGE_HIGH** [27:27]
    #[inline(always)]
    pub fn gpio_qspi_sd2_edge_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD2_EDGE_HIGH_MASK) != 0
    }

    /// **GPIO_QSPI_SD2_EDGE_LOW** [26:26]
    #[inline(always)]
    pub fn gpio_qspi_sd2_edge_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD2_EDGE_LOW_MASK) != 0
    }

    /// **GPIO_QSPI_SD2_LEVEL_HIGH** [25:25]
    #[inline(always)]
    pub fn gpio_qspi_sd2_level_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD2_LEVEL_HIGH_MASK) != 0
    }

    /// **GPIO_QSPI_SD2_LEVEL_LOW** [24:24]
    #[inline(always)]
    pub fn gpio_qspi_sd2_level_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD2_LEVEL_LOW_MASK) != 0
    }

    /// **GPIO_QSPI_SD1_EDGE_HIGH** [23:23]
    #[inline(always)]
    pub fn gpio_qspi_sd1_edge_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD1_EDGE_HIGH_MASK) != 0
    }

    /// **GPIO_QSPI_SD1_EDGE_LOW** [22:22]
    #[inline(always)]
    pub fn gpio_qspi_sd1_edge_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD1_EDGE_LOW_MASK) != 0
    }

    /// **GPIO_QSPI_SD1_LEVEL_HIGH** [21:21]
    #[inline(always)]
    pub fn gpio_qspi_sd1_level_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD1_LEVEL_HIGH_MASK) != 0
    }

    /// **GPIO_QSPI_SD1_LEVEL_LOW** [20:20]
    #[inline(always)]
    pub fn gpio_qspi_sd1_level_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD1_LEVEL_LOW_MASK) != 0
    }

    /// **GPIO_QSPI_SD0_EDGE_HIGH** [19:19]
    #[inline(always)]
    pub fn gpio_qspi_sd0_edge_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD0_EDGE_HIGH_MASK) != 0
    }

    /// **GPIO_QSPI_SD0_EDGE_LOW** [18:18]
    #[inline(always)]
    pub fn gpio_qspi_sd0_edge_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD0_EDGE_LOW_MASK) != 0
    }

    /// **GPIO_QSPI_SD0_LEVEL_HIGH** [17:17]
    #[inline(always)]
    pub fn gpio_qspi_sd0_level_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD0_LEVEL_HIGH_MASK) != 0
    }

    /// **GPIO_QSPI_SD0_LEVEL_LOW** [16:16]
    #[inline(always)]
    pub fn gpio_qspi_sd0_level_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SD0_LEVEL_LOW_MASK) != 0
    }

    /// **GPIO_QSPI_SS_EDGE_HIGH** [15:15]
    #[inline(always)]
    pub fn gpio_qspi_ss_edge_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SS_EDGE_HIGH_MASK) != 0
    }

    /// **GPIO_QSPI_SS_EDGE_LOW** [14:14]
    #[inline(always)]
    pub fn gpio_qspi_ss_edge_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SS_EDGE_LOW_MASK) != 0
    }

    /// **GPIO_QSPI_SS_LEVEL_HIGH** [13:13]
    #[inline(always)]
    pub fn gpio_qspi_ss_level_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SS_LEVEL_HIGH_MASK) != 0
    }

    /// **GPIO_QSPI_SS_LEVEL_LOW** [12:12]
    #[inline(always)]
    pub fn gpio_qspi_ss_level_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SS_LEVEL_LOW_MASK) != 0
    }

    /// **GPIO_QSPI_SCLK_EDGE_HIGH** [11:11]
    #[inline(always)]
    pub fn gpio_qspi_sclk_edge_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SCLK_EDGE_HIGH_MASK) != 0
    }

    /// **GPIO_QSPI_SCLK_EDGE_LOW** [10:10]
    #[inline(always)]
    pub fn gpio_qspi_sclk_edge_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SCLK_EDGE_LOW_MASK) != 0
    }

    /// **GPIO_QSPI_SCLK_LEVEL_HIGH** [9:9]
    #[inline(always)]
    pub fn gpio_qspi_sclk_level_high(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SCLK_LEVEL_HIGH_MASK) != 0
    }

    /// **GPIO_QSPI_SCLK_LEVEL_LOW** [8:8]
    #[inline(always)]
    pub fn gpio_qspi_sclk_level_low(&self) -> bool {
        (self.read_bits() & Self::GPIO_QSPI_SCLK_LEVEL_LOW_MASK) != 0
    }

    /// **USBPHY_DM_EDGE_HIGH** [7:7]
    #[inline(always)]
    pub fn usbphy_dm_edge_high(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DM_EDGE_HIGH_MASK) != 0
    }

    /// **USBPHY_DM_EDGE_LOW** [6:6]
    #[inline(always)]
    pub fn usbphy_dm_edge_low(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DM_EDGE_LOW_MASK) != 0
    }

    /// **USBPHY_DM_LEVEL_HIGH** [5:5]
    #[inline(always)]
    pub fn usbphy_dm_level_high(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DM_LEVEL_HIGH_MASK) != 0
    }

    /// **USBPHY_DM_LEVEL_LOW** [4:4]
    #[inline(always)]
    pub fn usbphy_dm_level_low(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DM_LEVEL_LOW_MASK) != 0
    }

    /// **USBPHY_DP_EDGE_HIGH** [3:3]
    #[inline(always)]
    pub fn usbphy_dp_edge_high(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DP_EDGE_HIGH_MASK) != 0
    }

    /// **USBPHY_DP_EDGE_LOW** [2:2]
    #[inline(always)]
    pub fn usbphy_dp_edge_low(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DP_EDGE_LOW_MASK) != 0
    }

    /// **USBPHY_DP_LEVEL_HIGH** [1:1]
    #[inline(always)]
    pub fn usbphy_dp_level_high(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DP_LEVEL_HIGH_MASK) != 0
    }

    /// **USBPHY_DP_LEVEL_LOW** [0:0]
    #[inline(always)]
    pub fn usbphy_dp_level_low(&self) -> bool {
        (self.read_bits() & Self::USBPHY_DP_LEVEL_LOW_MASK) != 0
    }

}

/// DORMANT_WAKE_INTS value type for batch read/modify/write operations
///
/// Use `dormantwakeints.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dormantwakeints.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DormantWakeIntsVal {
    bits: u32,
}

impl DormantWakeIntsVal {
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

    /// Read GPIO_QSPI_SD3_EDGE_HIGH [31:31]
    #[inline(always)]
    pub const fn gpio_qspi_sd3_edge_high(&self) -> bool {
        (self.bits & DormantWakeInts::GPIO_QSPI_SD3_EDGE_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD3_EDGE_LOW [30:30]
    #[inline(always)]
    pub const fn gpio_qspi_sd3_edge_low(&self) -> bool {
        (self.bits & DormantWakeInts::GPIO_QSPI_SD3_EDGE_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SD3_LEVEL_HIGH [29:29]
    #[inline(always)]
    pub const fn gpio_qspi_sd3_level_high(&self) -> bool {
        (self.bits & DormantWakeInts::GPIO_QSPI_SD3_LEVEL_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD3_LEVEL_LOW [28:28]
    #[inline(always)]
    pub const fn gpio_qspi_sd3_level_low(&self) -> bool {
        (self.bits & DormantWakeInts::GPIO_QSPI_SD3_LEVEL_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SD2_EDGE_HIGH [27:27]
    #[inline(always)]
    pub const fn gpio_qspi_sd2_edge_high(&self) -> bool {
        (self.bits & DormantWakeInts::GPIO_QSPI_SD2_EDGE_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD2_EDGE_LOW [26:26]
    #[inline(always)]
    pub const fn gpio_qspi_sd2_edge_low(&self) -> bool {
        (self.bits & DormantWakeInts::GPIO_QSPI_SD2_EDGE_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SD2_LEVEL_HIGH [25:25]
    #[inline(always)]
    pub const fn gpio_qspi_sd2_level_high(&self) -> bool {
        (self.bits & DormantWakeInts::GPIO_QSPI_SD2_LEVEL_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD2_LEVEL_LOW [24:24]
    #[inline(always)]
    pub const fn gpio_qspi_sd2_level_low(&self) -> bool {
        (self.bits & DormantWakeInts::GPIO_QSPI_SD2_LEVEL_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SD1_EDGE_HIGH [23:23]
    #[inline(always)]
    pub const fn gpio_qspi_sd1_edge_high(&self) -> bool {
        (self.bits & DormantWakeInts::GPIO_QSPI_SD1_EDGE_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD1_EDGE_LOW [22:22]
    #[inline(always)]
    pub const fn gpio_qspi_sd1_edge_low(&self) -> bool {
        (self.bits & DormantWakeInts::GPIO_QSPI_SD1_EDGE_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SD1_LEVEL_HIGH [21:21]
    #[inline(always)]
    pub const fn gpio_qspi_sd1_level_high(&self) -> bool {
        (self.bits & DormantWakeInts::GPIO_QSPI_SD1_LEVEL_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD1_LEVEL_LOW [20:20]
    #[inline(always)]
    pub const fn gpio_qspi_sd1_level_low(&self) -> bool {
        (self.bits & DormantWakeInts::GPIO_QSPI_SD1_LEVEL_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SD0_EDGE_HIGH [19:19]
    #[inline(always)]
    pub const fn gpio_qspi_sd0_edge_high(&self) -> bool {
        (self.bits & DormantWakeInts::GPIO_QSPI_SD0_EDGE_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD0_EDGE_LOW [18:18]
    #[inline(always)]
    pub const fn gpio_qspi_sd0_edge_low(&self) -> bool {
        (self.bits & DormantWakeInts::GPIO_QSPI_SD0_EDGE_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SD0_LEVEL_HIGH [17:17]
    #[inline(always)]
    pub const fn gpio_qspi_sd0_level_high(&self) -> bool {
        (self.bits & DormantWakeInts::GPIO_QSPI_SD0_LEVEL_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SD0_LEVEL_LOW [16:16]
    #[inline(always)]
    pub const fn gpio_qspi_sd0_level_low(&self) -> bool {
        (self.bits & DormantWakeInts::GPIO_QSPI_SD0_LEVEL_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SS_EDGE_HIGH [15:15]
    #[inline(always)]
    pub const fn gpio_qspi_ss_edge_high(&self) -> bool {
        (self.bits & DormantWakeInts::GPIO_QSPI_SS_EDGE_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SS_EDGE_LOW [14:14]
    #[inline(always)]
    pub const fn gpio_qspi_ss_edge_low(&self) -> bool {
        (self.bits & DormantWakeInts::GPIO_QSPI_SS_EDGE_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SS_LEVEL_HIGH [13:13]
    #[inline(always)]
    pub const fn gpio_qspi_ss_level_high(&self) -> bool {
        (self.bits & DormantWakeInts::GPIO_QSPI_SS_LEVEL_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SS_LEVEL_LOW [12:12]
    #[inline(always)]
    pub const fn gpio_qspi_ss_level_low(&self) -> bool {
        (self.bits & DormantWakeInts::GPIO_QSPI_SS_LEVEL_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SCLK_EDGE_HIGH [11:11]
    #[inline(always)]
    pub const fn gpio_qspi_sclk_edge_high(&self) -> bool {
        (self.bits & DormantWakeInts::GPIO_QSPI_SCLK_EDGE_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SCLK_EDGE_LOW [10:10]
    #[inline(always)]
    pub const fn gpio_qspi_sclk_edge_low(&self) -> bool {
        (self.bits & DormantWakeInts::GPIO_QSPI_SCLK_EDGE_LOW_MASK) != 0
    }

    /// Read GPIO_QSPI_SCLK_LEVEL_HIGH [9:9]
    #[inline(always)]
    pub const fn gpio_qspi_sclk_level_high(&self) -> bool {
        (self.bits & DormantWakeInts::GPIO_QSPI_SCLK_LEVEL_HIGH_MASK) != 0
    }

    /// Read GPIO_QSPI_SCLK_LEVEL_LOW [8:8]
    #[inline(always)]
    pub const fn gpio_qspi_sclk_level_low(&self) -> bool {
        (self.bits & DormantWakeInts::GPIO_QSPI_SCLK_LEVEL_LOW_MASK) != 0
    }

    /// Read USBPHY_DM_EDGE_HIGH [7:7]
    #[inline(always)]
    pub const fn usbphy_dm_edge_high(&self) -> bool {
        (self.bits & DormantWakeInts::USBPHY_DM_EDGE_HIGH_MASK) != 0
    }

    /// Read USBPHY_DM_EDGE_LOW [6:6]
    #[inline(always)]
    pub const fn usbphy_dm_edge_low(&self) -> bool {
        (self.bits & DormantWakeInts::USBPHY_DM_EDGE_LOW_MASK) != 0
    }

    /// Read USBPHY_DM_LEVEL_HIGH [5:5]
    #[inline(always)]
    pub const fn usbphy_dm_level_high(&self) -> bool {
        (self.bits & DormantWakeInts::USBPHY_DM_LEVEL_HIGH_MASK) != 0
    }

    /// Read USBPHY_DM_LEVEL_LOW [4:4]
    #[inline(always)]
    pub const fn usbphy_dm_level_low(&self) -> bool {
        (self.bits & DormantWakeInts::USBPHY_DM_LEVEL_LOW_MASK) != 0
    }

    /// Read USBPHY_DP_EDGE_HIGH [3:3]
    #[inline(always)]
    pub const fn usbphy_dp_edge_high(&self) -> bool {
        (self.bits & DormantWakeInts::USBPHY_DP_EDGE_HIGH_MASK) != 0
    }

    /// Read USBPHY_DP_EDGE_LOW [2:2]
    #[inline(always)]
    pub const fn usbphy_dp_edge_low(&self) -> bool {
        (self.bits & DormantWakeInts::USBPHY_DP_EDGE_LOW_MASK) != 0
    }

    /// Read USBPHY_DP_LEVEL_HIGH [1:1]
    #[inline(always)]
    pub const fn usbphy_dp_level_high(&self) -> bool {
        (self.bits & DormantWakeInts::USBPHY_DP_LEVEL_HIGH_MASK) != 0
    }

    /// Read USBPHY_DP_LEVEL_LOW [0:0]
    #[inline(always)]
    pub const fn usbphy_dp_level_low(&self) -> bool {
        (self.bits & DormantWakeInts::USBPHY_DP_LEVEL_LOW_MASK) != 0
    }

}

impl Default for DormantWakeIntsVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl DormantWakeInts {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> DormantWakeIntsVal {
        DormantWakeIntsVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: DormantWakeIntsVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(DormantWakeIntsVal) -> DormantWakeIntsVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    pub usbphy_dp_status: UsbphyDpStatus,
    /// 0x0004
    pub usbphy_dp_ctrl: UsbphyDpCtrl,
    /// 0x0008
    pub usbphy_dm_status: UsbphyDmStatus,
    /// 0x000c
    pub usbphy_dm_ctrl: UsbphyDmCtrl,
    /// 0x0010
    pub gpio_qspi_sclk_status: GpioQspiSclkStatus,
    /// 0x0014
    pub gpio_qspi_sclk_ctrl: GpioQspiSclkCtrl,
    /// 0x0018
    pub gpio_qspi_ss_status: GpioQspiSsStatus,
    /// 0x001c
    pub gpio_qspi_ss_ctrl: GpioQspiSsCtrl,
    /// 0x0020
    pub gpio_qspi_sd0_status: GpioQspiSd0Status,
    /// 0x0024
    pub gpio_qspi_sd0_ctrl: GpioQspiSd0Ctrl,
    /// 0x0028
    pub gpio_qspi_sd1_status: GpioQspiSd1Status,
    /// 0x002c
    pub gpio_qspi_sd1_ctrl: GpioQspiSd1Ctrl,
    /// 0x0030
    pub gpio_qspi_sd2_status: GpioQspiSd2Status,
    /// 0x0034
    pub gpio_qspi_sd2_ctrl: GpioQspiSd2Ctrl,
    /// 0x0038
    pub gpio_qspi_sd3_status: GpioQspiSd3Status,
    /// 0x003c
    pub gpio_qspi_sd3_ctrl: GpioQspiSd3Ctrl,
    _reserved0: [u8; 448],
    /// 0x0200
    pub irqsummary_proc0_secure: IrqsummaryProc0Secure,
    /// 0x0204
    pub irqsummary_proc0_nonsecure: IrqsummaryProc0Nonsecure,
    /// 0x0208
    pub irqsummary_proc1_secure: IrqsummaryProc1Secure,
    /// 0x020c
    pub irqsummary_proc1_nonsecure: IrqsummaryProc1Nonsecure,
    /// 0x0210
    pub irqsummary_dormant_wake_secure: IrqsummaryDormantWakeSecure,
    /// 0x0214
    pub irqsummary_dormant_wake_nonsecure: IrqsummaryDormantWakeNonsecure,
    /// 0x0218 - Raw Interrupts
    pub intr: Intr,
    /// 0x021c - Interrupt Enable for proc0
    pub proc0_inte: Proc0Inte,
    /// 0x0220 - Interrupt Force for proc0
    pub proc0_intf: Proc0Intf,
    /// 0x0224 - Interrupt status after masking & forcing for proc0
    pub proc0_ints: Proc0Ints,
    /// 0x0228 - Interrupt Enable for proc1
    pub proc1_inte: Proc1Inte,
    /// 0x022c - Interrupt Force for proc1
    pub proc1_intf: Proc1Intf,
    /// 0x0230 - Interrupt status after masking & forcing for proc1
    pub proc1_ints: Proc1Ints,
    /// 0x0234 - Interrupt Enable for dormant_wake
    pub dormant_wake_inte: DormantWakeInte,
    /// 0x0238 - Interrupt Force for dormant_wake
    pub dormant_wake_intf: DormantWakeIntf,
    /// 0x023c - Interrupt status after masking & forcing for dormant_wake
    pub dormant_wake_ints: DormantWakeInts,
}

