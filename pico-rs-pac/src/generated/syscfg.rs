//! SYSCFG - Register block for various chip control signals

#![allow(dead_code)]

pub const BASE: u32 = 0x40008000;

/// PROC_CONFIG Register
///
/// Configuration for processors
///
/// Address offset: 0x0000
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **PROC1_HALTED** [1] (RO): Indication that proc1 has halted
/// - **PROC0_HALTED** [0] (RO): Indication that proc0 has halted
#[repr(transparent)]
pub struct ProcConfig {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for ProcConfig {}
unsafe impl Sync for ProcConfig {}

impl ProcConfig {
    // ======== Field Constants ========

    /// PROC1_HALTED - Indication that proc1 has halted
    /// Bits [1:1], mask 0x00000002
    pub const PROC1_HALTED_OFFSET: u32 = 1;
    pub const PROC1_HALTED_MASK: u32 = 0x00000002;
    pub const PROC1_HALTED: u32 = 1 << 1;

    /// PROC0_HALTED - Indication that proc0 has halted
    /// Bits [0:0], mask 0x00000001
    pub const PROC0_HALTED_OFFSET: u32 = 0;
    pub const PROC0_HALTED_MASK: u32 = 0x00000001;
    pub const PROC0_HALTED: u32 = 1 << 0;

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

    /// **PROC1_HALTED** [1:1]: Indication that proc1 has halted
    #[inline(always)]
    pub fn proc1_halted(&self) -> bool {
        (self.read_bits() & Self::PROC1_HALTED_MASK) != 0
    }

    /// **PROC0_HALTED** [0:0]: Indication that proc0 has halted
    #[inline(always)]
    pub fn proc0_halted(&self) -> bool {
        (self.read_bits() & Self::PROC0_HALTED_MASK) != 0
    }

}

/// PROC_CONFIG value type for batch read/modify/write operations
///
/// Use `procconfig.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `procconfig.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ProcConfigVal {
    bits: u32,
}

impl ProcConfigVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
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

    /// Read PROC1_HALTED [1:1]: Indication that proc1 has halted
    #[inline(always)]
    pub const fn proc1_halted(&self) -> bool {
        (self.bits & ProcConfig::PROC1_HALTED_MASK) != 0
    }

    /// Read PROC0_HALTED [0:0]: Indication that proc0 has halted
    #[inline(always)]
    pub const fn proc0_halted(&self) -> bool {
        (self.bits & ProcConfig::PROC0_HALTED_MASK) != 0
    }

}

impl Default for ProcConfigVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl ProcConfig {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> ProcConfigVal {
        ProcConfigVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: ProcConfigVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(ProcConfigVal) -> ProcConfigVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// PROC_IN_SYNC_BYPASS Register
///
/// For each bit, if 1, bypass the input synchronizer between that GPIO and the
/// GPIO input register in the SIO. The input synchronizers should generally be
/// unbypassed, to avoid injecting metastabilities into processors. If you're
/// feeling brave, you can bypass to save two cycles of input latency. This
/// register applies to GPIO 0...31.
///
/// Address offset: 0x0004
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **GPIO** [31:0] (RW): 
#[repr(transparent)]
pub struct ProcInSyncBypass {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for ProcInSyncBypass {}
unsafe impl Sync for ProcInSyncBypass {}

impl ProcInSyncBypass {
    // ======== Field Constants ========

    /// Bits [31:0], mask 0xffffffff
    pub const GPIO_OFFSET: u32 = 0;
    pub const GPIO_MASK: u32 = 0xffffffff;

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

    /// **GPIO** [31:0]
    #[inline(always)]
    pub fn gpio(&self) -> u32 {
        (self.read_bits() & Self::GPIO_MASK) >> Self::GPIO_OFFSET
    }

    /// Write GPIO field
    #[inline(always)]
    pub fn write_gpio(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::GPIO_MASK) | ((value << Self::GPIO_OFFSET) & Self::GPIO_MASK));
    }

}

/// PROC_IN_SYNC_BYPASS value type for batch read/modify/write operations
///
/// Use `procinsyncbypass.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `procinsyncbypass.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ProcInSyncBypassVal {
    bits: u32,
}

impl ProcInSyncBypassVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
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

    /// Read GPIO [31:0]
    #[inline(always)]
    pub const fn gpio(&self) -> u32 {
        (self.bits & ProcInSyncBypass::GPIO_MASK) >> ProcInSyncBypass::GPIO_OFFSET
    }

    /// Set GPIO [31:0]
    #[inline(always)]
    pub const fn with_gpio(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ProcInSyncBypass::GPIO_MASK) | ((value << ProcInSyncBypass::GPIO_OFFSET) & ProcInSyncBypass::GPIO_MASK)
        }
    }

}

impl Default for ProcInSyncBypassVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl ProcInSyncBypass {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> ProcInSyncBypassVal {
        ProcInSyncBypassVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: ProcInSyncBypassVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(ProcInSyncBypassVal) -> ProcInSyncBypassVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// PROC_IN_SYNC_BYPASS_HI Register
///
/// For each bit, if 1, bypass the input synchronizer between that GPIO and the
/// GPIO input register in the SIO. The input synchronizers should generally be
/// unbypassed, to avoid injecting metastabilities into processors. If you're
/// feeling brave, you can bypass to save two cycles of input latency. This
/// register applies to GPIO 32...47. USB GPIO 56..57 QSPI GPIO 58..63
///
/// Address offset: 0x0008
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **QSPI_SD** [31:28] (RW): 
/// - **QSPI_CSN** [27] (RW): 
/// - **QSPI_SCK** [26] (RW): 
/// - **USB_DM** [25] (RW): 
/// - **USB_DP** [24] (RW): 
/// - **GPIO** [15:0] (RW): 
#[repr(transparent)]
pub struct ProcInSyncBypassHi {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for ProcInSyncBypassHi {}
unsafe impl Sync for ProcInSyncBypassHi {}

impl ProcInSyncBypassHi {
    // ======== Field Constants ========

    /// Bits [31:28], mask 0xf0000000
    pub const QSPI_SD_OFFSET: u32 = 28;
    pub const QSPI_SD_MASK: u32 = 0xf0000000;

    /// Bits [27:27], mask 0x08000000
    pub const QSPI_CSN_OFFSET: u32 = 27;
    pub const QSPI_CSN_MASK: u32 = 0x08000000;
    pub const QSPI_CSN: u32 = 1 << 27;

    /// Bits [26:26], mask 0x04000000
    pub const QSPI_SCK_OFFSET: u32 = 26;
    pub const QSPI_SCK_MASK: u32 = 0x04000000;
    pub const QSPI_SCK: u32 = 1 << 26;

    /// Bits [25:25], mask 0x02000000
    pub const USB_DM_OFFSET: u32 = 25;
    pub const USB_DM_MASK: u32 = 0x02000000;
    pub const USB_DM: u32 = 1 << 25;

    /// Bits [24:24], mask 0x01000000
    pub const USB_DP_OFFSET: u32 = 24;
    pub const USB_DP_MASK: u32 = 0x01000000;
    pub const USB_DP: u32 = 1 << 24;

    /// Bits [15:0], mask 0x0000ffff
    pub const GPIO_OFFSET: u32 = 0;
    pub const GPIO_MASK: u32 = 0x0000ffff;

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

    /// **QSPI_SD** [31:28]
    #[inline(always)]
    pub fn qspi_sd(&self) -> u32 {
        (self.read_bits() & Self::QSPI_SD_MASK) >> Self::QSPI_SD_OFFSET
    }

    /// Write QSPI_SD field
    #[inline(always)]
    pub fn write_qspi_sd(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::QSPI_SD_MASK) | ((value << Self::QSPI_SD_OFFSET) & Self::QSPI_SD_MASK));
    }

    /// **QSPI_CSN** [27:27]
    #[inline(always)]
    pub fn qspi_csn(&self) -> bool {
        (self.read_bits() & Self::QSPI_CSN_MASK) != 0
    }

    /// Set QSPI_CSN bit
    #[inline(always)]
    pub fn set_qspi_csn(&self) {
        self.set_bits(Self::QSPI_CSN_MASK);
    }

    /// Clear QSPI_CSN bit
    #[inline(always)]
    pub fn clr_qspi_csn(&self) {
        self.clr_bits(Self::QSPI_CSN_MASK);
    }

    /// **QSPI_SCK** [26:26]
    #[inline(always)]
    pub fn qspi_sck(&self) -> bool {
        (self.read_bits() & Self::QSPI_SCK_MASK) != 0
    }

    /// Set QSPI_SCK bit
    #[inline(always)]
    pub fn set_qspi_sck(&self) {
        self.set_bits(Self::QSPI_SCK_MASK);
    }

    /// Clear QSPI_SCK bit
    #[inline(always)]
    pub fn clr_qspi_sck(&self) {
        self.clr_bits(Self::QSPI_SCK_MASK);
    }

    /// **USB_DM** [25:25]
    #[inline(always)]
    pub fn usb_dm(&self) -> bool {
        (self.read_bits() & Self::USB_DM_MASK) != 0
    }

    /// Set USB_DM bit
    #[inline(always)]
    pub fn set_usb_dm(&self) {
        self.set_bits(Self::USB_DM_MASK);
    }

    /// Clear USB_DM bit
    #[inline(always)]
    pub fn clr_usb_dm(&self) {
        self.clr_bits(Self::USB_DM_MASK);
    }

    /// **USB_DP** [24:24]
    #[inline(always)]
    pub fn usb_dp(&self) -> bool {
        (self.read_bits() & Self::USB_DP_MASK) != 0
    }

    /// Set USB_DP bit
    #[inline(always)]
    pub fn set_usb_dp(&self) {
        self.set_bits(Self::USB_DP_MASK);
    }

    /// Clear USB_DP bit
    #[inline(always)]
    pub fn clr_usb_dp(&self) {
        self.clr_bits(Self::USB_DP_MASK);
    }

    /// **GPIO** [15:0]
    #[inline(always)]
    pub fn gpio(&self) -> u32 {
        (self.read_bits() & Self::GPIO_MASK) >> Self::GPIO_OFFSET
    }

    /// Write GPIO field
    #[inline(always)]
    pub fn write_gpio(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::GPIO_MASK) | ((value << Self::GPIO_OFFSET) & Self::GPIO_MASK));
    }

}

/// PROC_IN_SYNC_BYPASS_HI value type for batch read/modify/write operations
///
/// Use `procinsyncbypasshi.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `procinsyncbypasshi.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ProcInSyncBypassHiVal {
    bits: u32,
}

impl ProcInSyncBypassHiVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
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

    /// Read QSPI_SD [31:28]
    #[inline(always)]
    pub const fn qspi_sd(&self) -> u32 {
        (self.bits & ProcInSyncBypassHi::QSPI_SD_MASK) >> ProcInSyncBypassHi::QSPI_SD_OFFSET
    }

    /// Read QSPI_CSN [27:27]
    #[inline(always)]
    pub const fn qspi_csn(&self) -> bool {
        (self.bits & ProcInSyncBypassHi::QSPI_CSN_MASK) != 0
    }

    /// Read QSPI_SCK [26:26]
    #[inline(always)]
    pub const fn qspi_sck(&self) -> bool {
        (self.bits & ProcInSyncBypassHi::QSPI_SCK_MASK) != 0
    }

    /// Read USB_DM [25:25]
    #[inline(always)]
    pub const fn usb_dm(&self) -> bool {
        (self.bits & ProcInSyncBypassHi::USB_DM_MASK) != 0
    }

    /// Read USB_DP [24:24]
    #[inline(always)]
    pub const fn usb_dp(&self) -> bool {
        (self.bits & ProcInSyncBypassHi::USB_DP_MASK) != 0
    }

    /// Read GPIO [15:0]
    #[inline(always)]
    pub const fn gpio(&self) -> u32 {
        (self.bits & ProcInSyncBypassHi::GPIO_MASK) >> ProcInSyncBypassHi::GPIO_OFFSET
    }

    /// Set QSPI_SD [31:28]
    #[inline(always)]
    pub const fn with_qspi_sd(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ProcInSyncBypassHi::QSPI_SD_MASK) | ((value << ProcInSyncBypassHi::QSPI_SD_OFFSET) & ProcInSyncBypassHi::QSPI_SD_MASK)
        }
    }

    /// Set QSPI_CSN [27:27]
    #[inline(always)]
    pub const fn with_qspi_csn(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ProcInSyncBypassHi::QSPI_CSN_MASK
            } else {
                self.bits & !ProcInSyncBypassHi::QSPI_CSN_MASK
            }
        }
    }

    /// Set QSPI_SCK [26:26]
    #[inline(always)]
    pub const fn with_qspi_sck(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ProcInSyncBypassHi::QSPI_SCK_MASK
            } else {
                self.bits & !ProcInSyncBypassHi::QSPI_SCK_MASK
            }
        }
    }

    /// Set USB_DM [25:25]
    #[inline(always)]
    pub const fn with_usb_dm(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ProcInSyncBypassHi::USB_DM_MASK
            } else {
                self.bits & !ProcInSyncBypassHi::USB_DM_MASK
            }
        }
    }

    /// Set USB_DP [24:24]
    #[inline(always)]
    pub const fn with_usb_dp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ProcInSyncBypassHi::USB_DP_MASK
            } else {
                self.bits & !ProcInSyncBypassHi::USB_DP_MASK
            }
        }
    }

    /// Set GPIO [15:0]
    #[inline(always)]
    pub const fn with_gpio(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ProcInSyncBypassHi::GPIO_MASK) | ((value << ProcInSyncBypassHi::GPIO_OFFSET) & ProcInSyncBypassHi::GPIO_MASK)
        }
    }

}

impl Default for ProcInSyncBypassHiVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl ProcInSyncBypassHi {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> ProcInSyncBypassHiVal {
        ProcInSyncBypassHiVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: ProcInSyncBypassHiVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(ProcInSyncBypassHiVal) -> ProcInSyncBypassHiVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// DBGFORCE Register
///
/// Directly control the chip SWD debug port
///
/// Address offset: 0x000c
/// Reset value: 0x00000006
///
/// ## Fields
///
/// - **ATTACH** [3] (RW): Attach chip debug port to syscfg controls, and disconnect it from external ...
/// - **SWCLK** [2] (RW): Directly drive SWCLK, if ATTACH is set
/// - **SWDI** [1] (RW): Directly drive SWDIO input, if ATTACH is set
/// - **SWDO** [0] (RO): Observe the value of SWDIO output.
#[repr(transparent)]
pub struct Dbgforce {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for Dbgforce {}
unsafe impl Sync for Dbgforce {}

impl Dbgforce {
    // ======== Field Constants ========

    /// ATTACH - Attach chip debug port to syscfg controls, and disconnect it from external ...
    /// Bits [3:3], mask 0x00000008
    pub const ATTACH_OFFSET: u32 = 3;
    pub const ATTACH_MASK: u32 = 0x00000008;
    pub const ATTACH: u32 = 1 << 3;

    /// SWCLK - Directly drive SWCLK, if ATTACH is set
    /// Bits [2:2], mask 0x00000004
    pub const SWCLK_OFFSET: u32 = 2;
    pub const SWCLK_MASK: u32 = 0x00000004;
    pub const SWCLK: u32 = 1 << 2;

    /// SWDI - Directly drive SWDIO input, if ATTACH is set
    /// Bits [1:1], mask 0x00000002
    pub const SWDI_OFFSET: u32 = 1;
    pub const SWDI_MASK: u32 = 0x00000002;
    pub const SWDI: u32 = 1 << 1;

    /// SWDO - Observe the value of SWDIO output.
    /// Bits [0:0], mask 0x00000001
    pub const SWDO_OFFSET: u32 = 0;
    pub const SWDO_MASK: u32 = 0x00000001;
    pub const SWDO: u32 = 1 << 0;

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

    /// **ATTACH** [3:3]: Attach chip debug port to syscfg controls, and disconnect it from external ...
    #[inline(always)]
    pub fn attach(&self) -> bool {
        (self.read_bits() & Self::ATTACH_MASK) != 0
    }

    /// Set ATTACH bit
    #[inline(always)]
    pub fn set_attach(&self) {
        self.set_bits(Self::ATTACH_MASK);
    }

    /// Clear ATTACH bit
    #[inline(always)]
    pub fn clr_attach(&self) {
        self.clr_bits(Self::ATTACH_MASK);
    }

    /// **SWCLK** [2:2]: Directly drive SWCLK, if ATTACH is set
    #[inline(always)]
    pub fn swclk(&self) -> bool {
        (self.read_bits() & Self::SWCLK_MASK) != 0
    }

    /// Set SWCLK bit
    #[inline(always)]
    pub fn set_swclk(&self) {
        self.set_bits(Self::SWCLK_MASK);
    }

    /// Clear SWCLK bit
    #[inline(always)]
    pub fn clr_swclk(&self) {
        self.clr_bits(Self::SWCLK_MASK);
    }

    /// **SWDI** [1:1]: Directly drive SWDIO input, if ATTACH is set
    #[inline(always)]
    pub fn swdi(&self) -> bool {
        (self.read_bits() & Self::SWDI_MASK) != 0
    }

    /// Set SWDI bit
    #[inline(always)]
    pub fn set_swdi(&self) {
        self.set_bits(Self::SWDI_MASK);
    }

    /// Clear SWDI bit
    #[inline(always)]
    pub fn clr_swdi(&self) {
        self.clr_bits(Self::SWDI_MASK);
    }

    /// **SWDO** [0:0]: Observe the value of SWDIO output.
    #[inline(always)]
    pub fn swdo(&self) -> bool {
        (self.read_bits() & Self::SWDO_MASK) != 0
    }

}

/// DBGFORCE value type for batch read/modify/write operations
///
/// Use `dbgforce.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dbgforce.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DbgforceVal {
    bits: u32,
}

impl DbgforceVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000006 }
    }

    /// Read ATTACH [3:3]: Attach chip debug port to syscfg controls, and disconnect it from external ...
    #[inline(always)]
    pub const fn attach(&self) -> bool {
        (self.bits & Dbgforce::ATTACH_MASK) != 0
    }

    /// Read SWCLK [2:2]: Directly drive SWCLK, if ATTACH is set
    #[inline(always)]
    pub const fn swclk(&self) -> bool {
        (self.bits & Dbgforce::SWCLK_MASK) != 0
    }

    /// Read SWDI [1:1]: Directly drive SWDIO input, if ATTACH is set
    #[inline(always)]
    pub const fn swdi(&self) -> bool {
        (self.bits & Dbgforce::SWDI_MASK) != 0
    }

    /// Read SWDO [0:0]: Observe the value of SWDIO output.
    #[inline(always)]
    pub const fn swdo(&self) -> bool {
        (self.bits & Dbgforce::SWDO_MASK) != 0
    }

    /// Set ATTACH [3:3]: Attach chip debug port to syscfg controls, and disconnect it from external ...
    #[inline(always)]
    pub const fn with_attach(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Dbgforce::ATTACH_MASK
            } else {
                self.bits & !Dbgforce::ATTACH_MASK
            }
        }
    }

    /// Set SWCLK [2:2]: Directly drive SWCLK, if ATTACH is set
    #[inline(always)]
    pub const fn with_swclk(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Dbgforce::SWCLK_MASK
            } else {
                self.bits & !Dbgforce::SWCLK_MASK
            }
        }
    }

    /// Set SWDI [1:1]: Directly drive SWDIO input, if ATTACH is set
    #[inline(always)]
    pub const fn with_swdi(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Dbgforce::SWDI_MASK
            } else {
                self.bits & !Dbgforce::SWDI_MASK
            }
        }
    }

}

impl Default for DbgforceVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Dbgforce {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> DbgforceVal {
        DbgforceVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: DbgforceVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(DbgforceVal) -> DbgforceVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// MEMPOWERDOWN Register
///
/// Control PD pins to memories. Set high to put memories to a low power state.
/// In this state the memories will retain contents but not be accessible Use
/// with caution
///
/// Address offset: 0x0010
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **BOOTRAM** [12] (RW): 
/// - **ROM** [11] (RW): 
/// - **USB** [10] (RW): 
/// - **SRAM9** [9] (RW): 
/// - **SRAM8** [8] (RW): 
/// - **SRAM7** [7] (RW): 
/// - **SRAM6** [6] (RW): 
/// - **SRAM5** [5] (RW): 
/// - **SRAM4** [4] (RW): 
/// - **SRAM3** [3] (RW): 
/// - **SRAM2** [2] (RW): 
/// - **SRAM1** [1] (RW): 
/// - **SRAM0** [0] (RW): 
#[repr(transparent)]
pub struct Mempowerdown {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for Mempowerdown {}
unsafe impl Sync for Mempowerdown {}

impl Mempowerdown {
    // ======== Field Constants ========

    /// Bits [12:12], mask 0x00001000
    pub const BOOTRAM_OFFSET: u32 = 12;
    pub const BOOTRAM_MASK: u32 = 0x00001000;
    pub const BOOTRAM: u32 = 1 << 12;

    /// Bits [11:11], mask 0x00000800
    pub const ROM_OFFSET: u32 = 11;
    pub const ROM_MASK: u32 = 0x00000800;
    pub const ROM: u32 = 1 << 11;

    /// Bits [10:10], mask 0x00000400
    pub const USB_OFFSET: u32 = 10;
    pub const USB_MASK: u32 = 0x00000400;
    pub const USB: u32 = 1 << 10;

    /// Bits [9:9], mask 0x00000200
    pub const SRAM9_OFFSET: u32 = 9;
    pub const SRAM9_MASK: u32 = 0x00000200;
    pub const SRAM9: u32 = 1 << 9;

    /// Bits [8:8], mask 0x00000100
    pub const SRAM8_OFFSET: u32 = 8;
    pub const SRAM8_MASK: u32 = 0x00000100;
    pub const SRAM8: u32 = 1 << 8;

    /// Bits [7:7], mask 0x00000080
    pub const SRAM7_OFFSET: u32 = 7;
    pub const SRAM7_MASK: u32 = 0x00000080;
    pub const SRAM7: u32 = 1 << 7;

    /// Bits [6:6], mask 0x00000040
    pub const SRAM6_OFFSET: u32 = 6;
    pub const SRAM6_MASK: u32 = 0x00000040;
    pub const SRAM6: u32 = 1 << 6;

    /// Bits [5:5], mask 0x00000020
    pub const SRAM5_OFFSET: u32 = 5;
    pub const SRAM5_MASK: u32 = 0x00000020;
    pub const SRAM5: u32 = 1 << 5;

    /// Bits [4:4], mask 0x00000010
    pub const SRAM4_OFFSET: u32 = 4;
    pub const SRAM4_MASK: u32 = 0x00000010;
    pub const SRAM4: u32 = 1 << 4;

    /// Bits [3:3], mask 0x00000008
    pub const SRAM3_OFFSET: u32 = 3;
    pub const SRAM3_MASK: u32 = 0x00000008;
    pub const SRAM3: u32 = 1 << 3;

    /// Bits [2:2], mask 0x00000004
    pub const SRAM2_OFFSET: u32 = 2;
    pub const SRAM2_MASK: u32 = 0x00000004;
    pub const SRAM2: u32 = 1 << 2;

    /// Bits [1:1], mask 0x00000002
    pub const SRAM1_OFFSET: u32 = 1;
    pub const SRAM1_MASK: u32 = 0x00000002;
    pub const SRAM1: u32 = 1 << 1;

    /// Bits [0:0], mask 0x00000001
    pub const SRAM0_OFFSET: u32 = 0;
    pub const SRAM0_MASK: u32 = 0x00000001;
    pub const SRAM0: u32 = 1 << 0;

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

    /// **BOOTRAM** [12:12]
    #[inline(always)]
    pub fn bootram(&self) -> bool {
        (self.read_bits() & Self::BOOTRAM_MASK) != 0
    }

    /// Set BOOTRAM bit
    #[inline(always)]
    pub fn set_bootram(&self) {
        self.set_bits(Self::BOOTRAM_MASK);
    }

    /// Clear BOOTRAM bit
    #[inline(always)]
    pub fn clr_bootram(&self) {
        self.clr_bits(Self::BOOTRAM_MASK);
    }

    /// **ROM** [11:11]
    #[inline(always)]
    pub fn rom(&self) -> bool {
        (self.read_bits() & Self::ROM_MASK) != 0
    }

    /// Set ROM bit
    #[inline(always)]
    pub fn set_rom(&self) {
        self.set_bits(Self::ROM_MASK);
    }

    /// Clear ROM bit
    #[inline(always)]
    pub fn clr_rom(&self) {
        self.clr_bits(Self::ROM_MASK);
    }

    /// **USB** [10:10]
    #[inline(always)]
    pub fn usb(&self) -> bool {
        (self.read_bits() & Self::USB_MASK) != 0
    }

    /// Set USB bit
    #[inline(always)]
    pub fn set_usb(&self) {
        self.set_bits(Self::USB_MASK);
    }

    /// Clear USB bit
    #[inline(always)]
    pub fn clr_usb(&self) {
        self.clr_bits(Self::USB_MASK);
    }

    /// **SRAM9** [9:9]
    #[inline(always)]
    pub fn sram9(&self) -> bool {
        (self.read_bits() & Self::SRAM9_MASK) != 0
    }

    /// Set SRAM9 bit
    #[inline(always)]
    pub fn set_sram9(&self) {
        self.set_bits(Self::SRAM9_MASK);
    }

    /// Clear SRAM9 bit
    #[inline(always)]
    pub fn clr_sram9(&self) {
        self.clr_bits(Self::SRAM9_MASK);
    }

    /// **SRAM8** [8:8]
    #[inline(always)]
    pub fn sram8(&self) -> bool {
        (self.read_bits() & Self::SRAM8_MASK) != 0
    }

    /// Set SRAM8 bit
    #[inline(always)]
    pub fn set_sram8(&self) {
        self.set_bits(Self::SRAM8_MASK);
    }

    /// Clear SRAM8 bit
    #[inline(always)]
    pub fn clr_sram8(&self) {
        self.clr_bits(Self::SRAM8_MASK);
    }

    /// **SRAM7** [7:7]
    #[inline(always)]
    pub fn sram7(&self) -> bool {
        (self.read_bits() & Self::SRAM7_MASK) != 0
    }

    /// Set SRAM7 bit
    #[inline(always)]
    pub fn set_sram7(&self) {
        self.set_bits(Self::SRAM7_MASK);
    }

    /// Clear SRAM7 bit
    #[inline(always)]
    pub fn clr_sram7(&self) {
        self.clr_bits(Self::SRAM7_MASK);
    }

    /// **SRAM6** [6:6]
    #[inline(always)]
    pub fn sram6(&self) -> bool {
        (self.read_bits() & Self::SRAM6_MASK) != 0
    }

    /// Set SRAM6 bit
    #[inline(always)]
    pub fn set_sram6(&self) {
        self.set_bits(Self::SRAM6_MASK);
    }

    /// Clear SRAM6 bit
    #[inline(always)]
    pub fn clr_sram6(&self) {
        self.clr_bits(Self::SRAM6_MASK);
    }

    /// **SRAM5** [5:5]
    #[inline(always)]
    pub fn sram5(&self) -> bool {
        (self.read_bits() & Self::SRAM5_MASK) != 0
    }

    /// Set SRAM5 bit
    #[inline(always)]
    pub fn set_sram5(&self) {
        self.set_bits(Self::SRAM5_MASK);
    }

    /// Clear SRAM5 bit
    #[inline(always)]
    pub fn clr_sram5(&self) {
        self.clr_bits(Self::SRAM5_MASK);
    }

    /// **SRAM4** [4:4]
    #[inline(always)]
    pub fn sram4(&self) -> bool {
        (self.read_bits() & Self::SRAM4_MASK) != 0
    }

    /// Set SRAM4 bit
    #[inline(always)]
    pub fn set_sram4(&self) {
        self.set_bits(Self::SRAM4_MASK);
    }

    /// Clear SRAM4 bit
    #[inline(always)]
    pub fn clr_sram4(&self) {
        self.clr_bits(Self::SRAM4_MASK);
    }

    /// **SRAM3** [3:3]
    #[inline(always)]
    pub fn sram3(&self) -> bool {
        (self.read_bits() & Self::SRAM3_MASK) != 0
    }

    /// Set SRAM3 bit
    #[inline(always)]
    pub fn set_sram3(&self) {
        self.set_bits(Self::SRAM3_MASK);
    }

    /// Clear SRAM3 bit
    #[inline(always)]
    pub fn clr_sram3(&self) {
        self.clr_bits(Self::SRAM3_MASK);
    }

    /// **SRAM2** [2:2]
    #[inline(always)]
    pub fn sram2(&self) -> bool {
        (self.read_bits() & Self::SRAM2_MASK) != 0
    }

    /// Set SRAM2 bit
    #[inline(always)]
    pub fn set_sram2(&self) {
        self.set_bits(Self::SRAM2_MASK);
    }

    /// Clear SRAM2 bit
    #[inline(always)]
    pub fn clr_sram2(&self) {
        self.clr_bits(Self::SRAM2_MASK);
    }

    /// **SRAM1** [1:1]
    #[inline(always)]
    pub fn sram1(&self) -> bool {
        (self.read_bits() & Self::SRAM1_MASK) != 0
    }

    /// Set SRAM1 bit
    #[inline(always)]
    pub fn set_sram1(&self) {
        self.set_bits(Self::SRAM1_MASK);
    }

    /// Clear SRAM1 bit
    #[inline(always)]
    pub fn clr_sram1(&self) {
        self.clr_bits(Self::SRAM1_MASK);
    }

    /// **SRAM0** [0:0]
    #[inline(always)]
    pub fn sram0(&self) -> bool {
        (self.read_bits() & Self::SRAM0_MASK) != 0
    }

    /// Set SRAM0 bit
    #[inline(always)]
    pub fn set_sram0(&self) {
        self.set_bits(Self::SRAM0_MASK);
    }

    /// Clear SRAM0 bit
    #[inline(always)]
    pub fn clr_sram0(&self) {
        self.clr_bits(Self::SRAM0_MASK);
    }

}

/// MEMPOWERDOWN value type for batch read/modify/write operations
///
/// Use `mempowerdown.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `mempowerdown.write_val(val)`.
#[derive(Clone, Copy)]
pub struct MempowerdownVal {
    bits: u32,
}

impl MempowerdownVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
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

    /// Read BOOTRAM [12:12]
    #[inline(always)]
    pub const fn bootram(&self) -> bool {
        (self.bits & Mempowerdown::BOOTRAM_MASK) != 0
    }

    /// Read ROM [11:11]
    #[inline(always)]
    pub const fn rom(&self) -> bool {
        (self.bits & Mempowerdown::ROM_MASK) != 0
    }

    /// Read USB [10:10]
    #[inline(always)]
    pub const fn usb(&self) -> bool {
        (self.bits & Mempowerdown::USB_MASK) != 0
    }

    /// Read SRAM9 [9:9]
    #[inline(always)]
    pub const fn sram9(&self) -> bool {
        (self.bits & Mempowerdown::SRAM9_MASK) != 0
    }

    /// Read SRAM8 [8:8]
    #[inline(always)]
    pub const fn sram8(&self) -> bool {
        (self.bits & Mempowerdown::SRAM8_MASK) != 0
    }

    /// Read SRAM7 [7:7]
    #[inline(always)]
    pub const fn sram7(&self) -> bool {
        (self.bits & Mempowerdown::SRAM7_MASK) != 0
    }

    /// Read SRAM6 [6:6]
    #[inline(always)]
    pub const fn sram6(&self) -> bool {
        (self.bits & Mempowerdown::SRAM6_MASK) != 0
    }

    /// Read SRAM5 [5:5]
    #[inline(always)]
    pub const fn sram5(&self) -> bool {
        (self.bits & Mempowerdown::SRAM5_MASK) != 0
    }

    /// Read SRAM4 [4:4]
    #[inline(always)]
    pub const fn sram4(&self) -> bool {
        (self.bits & Mempowerdown::SRAM4_MASK) != 0
    }

    /// Read SRAM3 [3:3]
    #[inline(always)]
    pub const fn sram3(&self) -> bool {
        (self.bits & Mempowerdown::SRAM3_MASK) != 0
    }

    /// Read SRAM2 [2:2]
    #[inline(always)]
    pub const fn sram2(&self) -> bool {
        (self.bits & Mempowerdown::SRAM2_MASK) != 0
    }

    /// Read SRAM1 [1:1]
    #[inline(always)]
    pub const fn sram1(&self) -> bool {
        (self.bits & Mempowerdown::SRAM1_MASK) != 0
    }

    /// Read SRAM0 [0:0]
    #[inline(always)]
    pub const fn sram0(&self) -> bool {
        (self.bits & Mempowerdown::SRAM0_MASK) != 0
    }

    /// Set BOOTRAM [12:12]
    #[inline(always)]
    pub const fn with_bootram(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Mempowerdown::BOOTRAM_MASK
            } else {
                self.bits & !Mempowerdown::BOOTRAM_MASK
            }
        }
    }

    /// Set ROM [11:11]
    #[inline(always)]
    pub const fn with_rom(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Mempowerdown::ROM_MASK
            } else {
                self.bits & !Mempowerdown::ROM_MASK
            }
        }
    }

    /// Set USB [10:10]
    #[inline(always)]
    pub const fn with_usb(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Mempowerdown::USB_MASK
            } else {
                self.bits & !Mempowerdown::USB_MASK
            }
        }
    }

    /// Set SRAM9 [9:9]
    #[inline(always)]
    pub const fn with_sram9(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Mempowerdown::SRAM9_MASK
            } else {
                self.bits & !Mempowerdown::SRAM9_MASK
            }
        }
    }

    /// Set SRAM8 [8:8]
    #[inline(always)]
    pub const fn with_sram8(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Mempowerdown::SRAM8_MASK
            } else {
                self.bits & !Mempowerdown::SRAM8_MASK
            }
        }
    }

    /// Set SRAM7 [7:7]
    #[inline(always)]
    pub const fn with_sram7(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Mempowerdown::SRAM7_MASK
            } else {
                self.bits & !Mempowerdown::SRAM7_MASK
            }
        }
    }

    /// Set SRAM6 [6:6]
    #[inline(always)]
    pub const fn with_sram6(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Mempowerdown::SRAM6_MASK
            } else {
                self.bits & !Mempowerdown::SRAM6_MASK
            }
        }
    }

    /// Set SRAM5 [5:5]
    #[inline(always)]
    pub const fn with_sram5(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Mempowerdown::SRAM5_MASK
            } else {
                self.bits & !Mempowerdown::SRAM5_MASK
            }
        }
    }

    /// Set SRAM4 [4:4]
    #[inline(always)]
    pub const fn with_sram4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Mempowerdown::SRAM4_MASK
            } else {
                self.bits & !Mempowerdown::SRAM4_MASK
            }
        }
    }

    /// Set SRAM3 [3:3]
    #[inline(always)]
    pub const fn with_sram3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Mempowerdown::SRAM3_MASK
            } else {
                self.bits & !Mempowerdown::SRAM3_MASK
            }
        }
    }

    /// Set SRAM2 [2:2]
    #[inline(always)]
    pub const fn with_sram2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Mempowerdown::SRAM2_MASK
            } else {
                self.bits & !Mempowerdown::SRAM2_MASK
            }
        }
    }

    /// Set SRAM1 [1:1]
    #[inline(always)]
    pub const fn with_sram1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Mempowerdown::SRAM1_MASK
            } else {
                self.bits & !Mempowerdown::SRAM1_MASK
            }
        }
    }

    /// Set SRAM0 [0:0]
    #[inline(always)]
    pub const fn with_sram0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Mempowerdown::SRAM0_MASK
            } else {
                self.bits & !Mempowerdown::SRAM0_MASK
            }
        }
    }

}

impl Default for MempowerdownVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Mempowerdown {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> MempowerdownVal {
        MempowerdownVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: MempowerdownVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(MempowerdownVal) -> MempowerdownVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// AUXCTRL Register
///
/// Auxiliary system control register
///
/// Address offset: 0x0014
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **AUXCTRL** [7:0] (RW): * Bits 7:3: Reserved * Bit 2: Set to mask OTP power analogue power supply d...
#[repr(transparent)]
pub struct Auxctrl {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for Auxctrl {}
unsafe impl Sync for Auxctrl {}

impl Auxctrl {
    // ======== Field Constants ========

    /// AUXCTRL - * Bits 7:3: Reserved * Bit 2: Set to mask OTP power analogue power supply d...
    /// Bits [7:0], mask 0x000000ff
    pub const AUXCTRL_OFFSET: u32 = 0;
    pub const AUXCTRL_MASK: u32 = 0x000000ff;

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

    /// **AUXCTRL** [7:0]: * Bits 7:3: Reserved * Bit 2: Set to mask OTP power analogue power supply d...
    #[inline(always)]
    pub fn auxctrl(&self) -> u32 {
        (self.read_bits() & Self::AUXCTRL_MASK) >> Self::AUXCTRL_OFFSET
    }

    /// Write AUXCTRL field
    #[inline(always)]
    pub fn write_auxctrl(&self, value: u32) {
        self.modify_bits(|r| (r & !Self::AUXCTRL_MASK) | ((value << Self::AUXCTRL_OFFSET) & Self::AUXCTRL_MASK));
    }

}

/// AUXCTRL value type for batch read/modify/write operations
///
/// Use `auxctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `auxctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct AuxctrlVal {
    bits: u32,
}

impl AuxctrlVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
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

    /// Read AUXCTRL [7:0]: * Bits 7:3: Reserved * Bit 2: Set to mask OTP power analogue power supply d...
    #[inline(always)]
    pub const fn auxctrl(&self) -> u32 {
        (self.bits & Auxctrl::AUXCTRL_MASK) >> Auxctrl::AUXCTRL_OFFSET
    }

    /// Set AUXCTRL [7:0]: * Bits 7:3: Reserved * Bit 2: Set to mask OTP power analogue power supply d...
    #[inline(always)]
    pub const fn with_auxctrl(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Auxctrl::AUXCTRL_MASK) | ((value << Auxctrl::AUXCTRL_OFFSET) & Auxctrl::AUXCTRL_MASK)
        }
    }

}

impl Default for AuxctrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Auxctrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> AuxctrlVal {
        AuxctrlVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: AuxctrlVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(AuxctrlVal) -> AuxctrlVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000 - Configuration for processors
    pub proc_config: ProcConfig,
    /// 0x0004 - For each bit, if 1, bypass the input synchronizer between that GPIO and the...
    pub proc_in_sync_bypass: ProcInSyncBypass,
    /// 0x0008 - For each bit, if 1, bypass the input synchronizer between that GPIO and the...
    pub proc_in_sync_bypass_hi: ProcInSyncBypassHi,
    /// 0x000c - Directly control the chip SWD debug port
    pub dbgforce: Dbgforce,
    /// 0x0010 - Control PD pins to memories. Set high to put memories to a low power state....
    pub mempowerdown: Mempowerdown,
    /// 0x0014 - Auxiliary system control register
    pub auxctrl: Auxctrl,
}

