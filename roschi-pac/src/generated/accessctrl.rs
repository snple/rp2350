//! ACCESSCTRL
//! Hardware access control registers
//!
//! This peripheral supports atomic SET/CLR/XOR register aliases.

#![allow(dead_code)]

pub const BASE: u32 = 0x40060000;

/// LOCK Register
///
/// Once a LOCK bit is written to 1, ACCESSCTRL silently ignores writes from that master. LOCK is writable only by a Secure, Privileged processor or debugger.
/// LOCK bits are only writable when their value is zero. Once set, they can never be cleared, except by a full reset of ACCESSCTRL
/// Setting the LOCK bit does not affect whether an access raises a bus error. Unprivileged writes, or writes from the DMA, will continue to raise bus errors. All other accesses will continue not to.
///
/// Address offset: 0x0000
/// Reset value: 0x00000004
///
/// ## Fields
///
/// - **DEBUG** `[3]` (RW)
///
/// - **DMA** `[2]` (RO)
///
/// - **CORE1** `[1]` (RW)
///
/// - **CORE0** `[0]` (RW)
#[repr(transparent)]
pub struct Lock {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Lock {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Lock {
    // ======== Field Constants ========

    /// **DEBUG**
    /// Bits `[3:3]`, mask `0x00000008`
    pub const DEBUG_OFFSET: u32 = 3;
    pub const DEBUG_MASK: u32 = 0x00000008;
    pub const DEBUG: u32 = 1 << 3;

    /// **DMA**
    /// Bits `[2:2]`, mask `0x00000004`
    pub const DMA_OFFSET: u32 = 2;
    pub const DMA_MASK: u32 = 0x00000004;
    pub const DMA: u32 = 1 << 2;

    /// **CORE1**
    /// Bits `[1:1]`, mask `0x00000002`
    pub const CORE1_OFFSET: u32 = 1;
    pub const CORE1_MASK: u32 = 0x00000002;
    pub const CORE1: u32 = 1 << 1;

    /// **CORE0**
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CORE0_OFFSET: u32 = 0;
    pub const CORE0_MASK: u32 = 0x00000001;
    pub const CORE0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DEBUG** `[3]` (RW)
    #[inline(always)]
    pub fn debug(&self) -> bool {
        (self.value.read_bits() & Self::DEBUG_MASK) != 0
    }

    /// Set DEBUG bit atomically (RW)
    #[inline(always)]
    pub fn set_debug(&self) {
        self.set_bits(Self::DEBUG_MASK);
    }

    /// Clear DEBUG bit atomically (RW)
    #[inline(always)]
    pub fn clr_debug(&self) {
        self.clr_bits(Self::DEBUG_MASK);
    }

    /// **DMA** `[2]` (RO)
    #[inline(always)]
    pub fn dma(&self) -> bool {
        (self.value.read_bits() & Self::DMA_MASK) != 0
    }

    /// **CORE1** `[1]` (RW)
    #[inline(always)]
    pub fn core1(&self) -> bool {
        (self.value.read_bits() & Self::CORE1_MASK) != 0
    }

    /// Set CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn set_core1(&self) {
        self.set_bits(Self::CORE1_MASK);
    }

    /// Clear CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core1(&self) {
        self.clr_bits(Self::CORE1_MASK);
    }

    /// **CORE0** `[0]` (RW)
    #[inline(always)]
    pub fn core0(&self) -> bool {
        (self.value.read_bits() & Self::CORE0_MASK) != 0
    }

    /// Set CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn set_core0(&self) {
        self.set_bits(Self::CORE0_MASK);
    }

    /// Clear CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core0(&self) {
        self.clr_bits(Self::CORE0_MASK);
    }
}

/// LOCK value type for batch read/modify/write operations
///
/// Use `lock.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lock.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LockVal {
    bits: u32,
}

impl LockVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000004 }
    }

    /// Read **DEBUG** `[3:3]`
    #[inline(always)]
    pub const fn debug(&self) -> bool {
        (self.bits & Lock::DEBUG_MASK) != 0
    }

    /// Read **DMA** `[2:2]`
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        (self.bits & Lock::DMA_MASK) != 0
    }

    /// Read **CORE1** `[1:1]`
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        (self.bits & Lock::CORE1_MASK) != 0
    }

    /// Read **CORE0** `[0:0]`
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        (self.bits & Lock::CORE0_MASK) != 0
    }

    /// Set **DEBUG** `[3:3]`
    #[inline(always)]
    pub const fn with_debug(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Lock::DEBUG_MASK
            } else {
                self.bits & !Lock::DEBUG_MASK
            }
        }
    }

    /// Set **CORE1** `[1:1]`
    #[inline(always)]
    pub const fn with_core1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Lock::CORE1_MASK
            } else {
                self.bits & !Lock::CORE1_MASK
            }
        }
    }

    /// Set **CORE0** `[0:0]`
    #[inline(always)]
    pub const fn with_core0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Lock::CORE0_MASK
            } else {
                self.bits & !Lock::CORE0_MASK
            }
        }
    }
}

impl Default for LockVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Lock {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LockVal {
        LockVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LockVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LockVal) -> LockVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// FORCE_CORE_NS Register
///
/// Force core 1's bus accesses to always be Non-secure, no matter the core's internal state.
/// Useful for schemes where one core is designated as the Non-secure core, since some peripherals may filter individual registers internally based on security state but not on master ID.
///
/// Address offset: 0x0004
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CORE1** `[1]` (RW)
#[repr(transparent)]
pub struct ForceCoreNs {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for ForceCoreNs {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ForceCoreNs {
    // ======== Field Constants ========

    /// **CORE1**
    /// Bits `[1:1]`, mask `0x00000002`
    pub const CORE1_OFFSET: u32 = 1;
    pub const CORE1_MASK: u32 = 0x00000002;
    pub const CORE1: u32 = 1 << 1;

    // ======== Field Accessors ========

    /// **CORE1** `[1]` (RW)
    #[inline(always)]
    pub fn core1(&self) -> bool {
        (self.value.read_bits() & Self::CORE1_MASK) != 0
    }

    /// Set CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn set_core1(&self) {
        self.set_bits(Self::CORE1_MASK);
    }

    /// Clear CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core1(&self) {
        self.clr_bits(Self::CORE1_MASK);
    }
}

/// FORCE_CORE_NS value type for batch read/modify/write operations
///
/// Use `forcecorens.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `forcecorens.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ForceCoreNsVal {
    bits: u32,
}

impl ForceCoreNsVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
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

    /// Read **CORE1** `[1:1]`
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        (self.bits & ForceCoreNs::CORE1_MASK) != 0
    }

    /// Set **CORE1** `[1:1]`
    #[inline(always)]
    pub const fn with_core1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ForceCoreNs::CORE1_MASK
            } else {
                self.bits & !ForceCoreNs::CORE1_MASK
            }
        }
    }
}

impl Default for ForceCoreNsVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl ForceCoreNs {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ForceCoreNsVal {
        ForceCoreNsVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ForceCoreNsVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ForceCoreNsVal) -> ForceCoreNsVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CFGRESET Register
///
/// Write 1 to reset all ACCESSCTRL configuration, except for the LOCK and FORCE_CORE_NS registers.
/// This bit is used in the RP2350 bootrom to quickly restore ACCESSCTRL to a known state during the boot path.
/// Note that, like all registers in ACCESSCTRL, this register is not writable when the writer's corresponding LOCK bit is set, therefore a master which has been locked out of ACCESSCTRL can not use the CFGRESET register to disturb its contents.
///
/// Address offset: 0x0008
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CFGRESET** `[0]` (WO)
#[repr(transparent)]
pub struct Cfgreset {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Cfgreset {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Cfgreset {
    // ======== Field Constants ========

    /// **CFGRESET**
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CFGRESET_OFFSET: u32 = 0;
    pub const CFGRESET_MASK: u32 = 0x00000001;
    pub const CFGRESET: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **CFGRESET** (WO)
    #[inline(always)]
    pub fn trigger_cfgreset(&self) {
        self.set_bits(Self::CFGRESET_MASK);
    }
}

/// CFGRESET value type for batch read/modify/write operations
///
/// Use `cfgreset.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `cfgreset.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CfgresetVal {
    bits: u32,
}

impl CfgresetVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
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

    /// Set **CFGRESET** `[0:0]`
    #[inline(always)]
    pub const fn with_cfgreset(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cfgreset::CFGRESET_MASK
            } else {
                self.bits & !Cfgreset::CFGRESET_MASK
            }
        }
    }
}

impl Default for CfgresetVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Cfgreset {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CfgresetVal) {
        self.value.write_bits(val.bits())
    }
}

/// GPIO_NSMASK0 Register
///
/// Control whether GPIO0...31 are accessible to Non-secure code. Writable only by a Secure, Privileged processor or debugger.
/// 0 -> Secure access only
/// 1 -> Secure + Non-secure access
///
/// Address offset: 0x000c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **GPIO_NSMASK0** `[31:0]` (RW)
#[repr(transparent)]
pub struct GpioNsmask0 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for GpioNsmask0 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl GpioNsmask0 {
    // ======== Field Constants ========

    /// **GPIO_NSMASK0**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const GPIO_NSMASK0_OFFSET: u32 = 0;
    pub const GPIO_NSMASK0_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **GPIO_NSMASK0** `[31:0]` (RW)
    #[inline(always)]
    pub fn gpio_nsmask0(&self) -> u32 {
        (self.value.read_bits() & Self::GPIO_NSMASK0_MASK) >> Self::GPIO_NSMASK0_OFFSET
    }

    /// Write **GPIO_NSMASK0** `[31:0]` (RW)
    #[inline(always)]
    pub fn write_gpio_nsmask0(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::GPIO_NSMASK0_MASK) | ((value << Self::GPIO_NSMASK0_OFFSET) & Self::GPIO_NSMASK0_MASK));
    }
}

/// GPIO_NSMASK0 value type for batch read/modify/write operations
///
/// Use `gpionsmask0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpionsmask0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct GpioNsmask0Val {
    bits: u32,
}

impl GpioNsmask0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
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

    /// Read **GPIO_NSMASK0** `[31:0]`
    #[inline(always)]
    pub const fn gpio_nsmask0(&self) -> u32 {
        (self.bits & GpioNsmask0::GPIO_NSMASK0_MASK) >> GpioNsmask0::GPIO_NSMASK0_OFFSET
    }

    /// Set **GPIO_NSMASK0** `[31:0]`
    #[inline(always)]
    pub const fn with_gpio_nsmask0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GpioNsmask0::GPIO_NSMASK0_MASK) | ((value << GpioNsmask0::GPIO_NSMASK0_OFFSET) & GpioNsmask0::GPIO_NSMASK0_MASK)
        }
    }
}

impl Default for GpioNsmask0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl GpioNsmask0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> GpioNsmask0Val {
        GpioNsmask0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: GpioNsmask0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(GpioNsmask0Val) -> GpioNsmask0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// GPIO_NSMASK1 Register
///
/// Control whether GPIO32..47 are accessible to Non-secure code, and whether QSPI and USB bitbang are accessible through the Non-secure SIO. Writable only by a Secure, Privileged processor or debugger.
///
/// Address offset: 0x0010
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **QSPI_SD** `[31:28]` (RW)
///
/// - **QSPI_CSN** `[27]` (RW)
///
/// - **QSPI_SCK** `[26]` (RW)
///
/// - **USB_DM** `[25]` (RW)
///
/// - **USB_DP** `[24]` (RW)
///
/// - **GPIO** `[15:0]` (RW)
#[repr(transparent)]
pub struct GpioNsmask1 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for GpioNsmask1 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl GpioNsmask1 {
    // ======== Field Constants ========

    /// **QSPI_SD**
    /// Bits `[31:28]`, mask `0xf0000000`
    pub const QSPI_SD_OFFSET: u32 = 28;
    pub const QSPI_SD_MASK: u32 = 0xf0000000;

    /// **QSPI_CSN**
    /// Bits `[27:27]`, mask `0x08000000`
    pub const QSPI_CSN_OFFSET: u32 = 27;
    pub const QSPI_CSN_MASK: u32 = 0x08000000;
    pub const QSPI_CSN: u32 = 1 << 27;

    /// **QSPI_SCK**
    /// Bits `[26:26]`, mask `0x04000000`
    pub const QSPI_SCK_OFFSET: u32 = 26;
    pub const QSPI_SCK_MASK: u32 = 0x04000000;
    pub const QSPI_SCK: u32 = 1 << 26;

    /// **USB_DM**
    /// Bits `[25:25]`, mask `0x02000000`
    pub const USB_DM_OFFSET: u32 = 25;
    pub const USB_DM_MASK: u32 = 0x02000000;
    pub const USB_DM: u32 = 1 << 25;

    /// **USB_DP**
    /// Bits `[24:24]`, mask `0x01000000`
    pub const USB_DP_OFFSET: u32 = 24;
    pub const USB_DP_MASK: u32 = 0x01000000;
    pub const USB_DP: u32 = 1 << 24;

    /// **GPIO**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const GPIO_OFFSET: u32 = 0;
    pub const GPIO_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **QSPI_SD** `[31:28]` (RW)
    #[inline(always)]
    pub fn qspi_sd(&self) -> u32 {
        (self.value.read_bits() & Self::QSPI_SD_MASK) >> Self::QSPI_SD_OFFSET
    }

    /// Write **QSPI_SD** `[31:28]` (RW)
    #[inline(always)]
    pub fn write_qspi_sd(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::QSPI_SD_MASK) | ((value << Self::QSPI_SD_OFFSET) & Self::QSPI_SD_MASK));
    }

    /// **QSPI_CSN** `[27]` (RW)
    #[inline(always)]
    pub fn qspi_csn(&self) -> bool {
        (self.value.read_bits() & Self::QSPI_CSN_MASK) != 0
    }

    /// Set QSPI_CSN bit atomically (RW)
    #[inline(always)]
    pub fn set_qspi_csn(&self) {
        self.set_bits(Self::QSPI_CSN_MASK);
    }

    /// Clear QSPI_CSN bit atomically (RW)
    #[inline(always)]
    pub fn clr_qspi_csn(&self) {
        self.clr_bits(Self::QSPI_CSN_MASK);
    }

    /// **QSPI_SCK** `[26]` (RW)
    #[inline(always)]
    pub fn qspi_sck(&self) -> bool {
        (self.value.read_bits() & Self::QSPI_SCK_MASK) != 0
    }

    /// Set QSPI_SCK bit atomically (RW)
    #[inline(always)]
    pub fn set_qspi_sck(&self) {
        self.set_bits(Self::QSPI_SCK_MASK);
    }

    /// Clear QSPI_SCK bit atomically (RW)
    #[inline(always)]
    pub fn clr_qspi_sck(&self) {
        self.clr_bits(Self::QSPI_SCK_MASK);
    }

    /// **USB_DM** `[25]` (RW)
    #[inline(always)]
    pub fn usb_dm(&self) -> bool {
        (self.value.read_bits() & Self::USB_DM_MASK) != 0
    }

    /// Set USB_DM bit atomically (RW)
    #[inline(always)]
    pub fn set_usb_dm(&self) {
        self.set_bits(Self::USB_DM_MASK);
    }

    /// Clear USB_DM bit atomically (RW)
    #[inline(always)]
    pub fn clr_usb_dm(&self) {
        self.clr_bits(Self::USB_DM_MASK);
    }

    /// **USB_DP** `[24]` (RW)
    #[inline(always)]
    pub fn usb_dp(&self) -> bool {
        (self.value.read_bits() & Self::USB_DP_MASK) != 0
    }

    /// Set USB_DP bit atomically (RW)
    #[inline(always)]
    pub fn set_usb_dp(&self) {
        self.set_bits(Self::USB_DP_MASK);
    }

    /// Clear USB_DP bit atomically (RW)
    #[inline(always)]
    pub fn clr_usb_dp(&self) {
        self.clr_bits(Self::USB_DP_MASK);
    }

    /// **GPIO** `[15:0]` (RW)
    #[inline(always)]
    pub fn gpio(&self) -> u32 {
        (self.value.read_bits() & Self::GPIO_MASK) >> Self::GPIO_OFFSET
    }

    /// Write **GPIO** `[15:0]` (RW)
    #[inline(always)]
    pub fn write_gpio(&self, value: u32) {
        self.value.modify_bits(|r| (r & !Self::GPIO_MASK) | ((value << Self::GPIO_OFFSET) & Self::GPIO_MASK));
    }
}

/// GPIO_NSMASK1 value type for batch read/modify/write operations
///
/// Use `gpionsmask1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpionsmask1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct GpioNsmask1Val {
    bits: u32,
}

impl GpioNsmask1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
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

    /// Read **QSPI_SD** `[31:28]`
    #[inline(always)]
    pub const fn qspi_sd(&self) -> u32 {
        (self.bits & GpioNsmask1::QSPI_SD_MASK) >> GpioNsmask1::QSPI_SD_OFFSET
    }

    /// Read **QSPI_CSN** `[27:27]`
    #[inline(always)]
    pub const fn qspi_csn(&self) -> bool {
        (self.bits & GpioNsmask1::QSPI_CSN_MASK) != 0
    }

    /// Read **QSPI_SCK** `[26:26]`
    #[inline(always)]
    pub const fn qspi_sck(&self) -> bool {
        (self.bits & GpioNsmask1::QSPI_SCK_MASK) != 0
    }

    /// Read **USB_DM** `[25:25]`
    #[inline(always)]
    pub const fn usb_dm(&self) -> bool {
        (self.bits & GpioNsmask1::USB_DM_MASK) != 0
    }

    /// Read **USB_DP** `[24:24]`
    #[inline(always)]
    pub const fn usb_dp(&self) -> bool {
        (self.bits & GpioNsmask1::USB_DP_MASK) != 0
    }

    /// Read **GPIO** `[15:0]`
    #[inline(always)]
    pub const fn gpio(&self) -> u32 {
        (self.bits & GpioNsmask1::GPIO_MASK) >> GpioNsmask1::GPIO_OFFSET
    }

    /// Set **QSPI_SD** `[31:28]`
    #[inline(always)]
    pub const fn with_qspi_sd(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GpioNsmask1::QSPI_SD_MASK) | ((value << GpioNsmask1::QSPI_SD_OFFSET) & GpioNsmask1::QSPI_SD_MASK)
        }
    }

    /// Set **QSPI_CSN** `[27:27]`
    #[inline(always)]
    pub const fn with_qspi_csn(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | GpioNsmask1::QSPI_CSN_MASK
            } else {
                self.bits & !GpioNsmask1::QSPI_CSN_MASK
            }
        }
    }

    /// Set **QSPI_SCK** `[26:26]`
    #[inline(always)]
    pub const fn with_qspi_sck(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | GpioNsmask1::QSPI_SCK_MASK
            } else {
                self.bits & !GpioNsmask1::QSPI_SCK_MASK
            }
        }
    }

    /// Set **USB_DM** `[25:25]`
    #[inline(always)]
    pub const fn with_usb_dm(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | GpioNsmask1::USB_DM_MASK
            } else {
                self.bits & !GpioNsmask1::USB_DM_MASK
            }
        }
    }

    /// Set **USB_DP** `[24:24]`
    #[inline(always)]
    pub const fn with_usb_dp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | GpioNsmask1::USB_DP_MASK
            } else {
                self.bits & !GpioNsmask1::USB_DP_MASK
            }
        }
    }

    /// Set **GPIO** `[15:0]`
    #[inline(always)]
    pub const fn with_gpio(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GpioNsmask1::GPIO_MASK) | ((value << GpioNsmask1::GPIO_OFFSET) & GpioNsmask1::GPIO_MASK)
        }
    }
}

impl Default for GpioNsmask1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl GpioNsmask1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> GpioNsmask1Val {
        GpioNsmask1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: GpioNsmask1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(GpioNsmask1Val) -> GpioNsmask1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ROM Register
///
/// Control whether debugger, DMA, core 0 and core 1 can access ROM, and at what security/privilege levels they can do so.
/// Defaults to fully open access.
/// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
///
/// Address offset: 0x0014
/// Reset value: 0x000000ff
///
/// ## Fields
///
/// - **DBG** `[7]` (RW)
///
///   If 1, ROM can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **DMA** `[6]` (RW)
///
///   If 1, ROM can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE1** `[5]` (RW)
///
///   If 1, ROM can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE0** `[4]` (RW)
///
///   If 1, ROM can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **SP** `[3]` (RW)
///
///   If 1, ROM can be accessed from a Secure, Privileged context.
///
/// - **SU** `[2]` (RW)
///
///   If 1, and SP is also set, ROM can be accessed from a Secure, Unprivileged context.
///
/// - **NSP** `[1]` (RW)
///
///   If 1, ROM can be accessed from a Non-secure, Privileged context.
///
/// - **NSU** `[0]` (RW)
///
///   If 1, and NSP is also set, ROM can be accessed from a Non-secure, Unprivileged context.
///   This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
#[repr(transparent)]
pub struct Rom {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Rom {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Rom {
    // ======== Field Constants ========

    /// **DBG**
    ///
    /// If 1, ROM can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const DBG_OFFSET: u32 = 7;
    pub const DBG_MASK: u32 = 0x00000080;
    pub const DBG: u32 = 1 << 7;

    /// **DMA**
    ///
    /// If 1, ROM can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const DMA_OFFSET: u32 = 6;
    pub const DMA_MASK: u32 = 0x00000040;
    pub const DMA: u32 = 1 << 6;

    /// **CORE1**
    ///
    /// If 1, ROM can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CORE1_OFFSET: u32 = 5;
    pub const CORE1_MASK: u32 = 0x00000020;
    pub const CORE1: u32 = 1 << 5;

    /// **CORE0**
    ///
    /// If 1, ROM can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CORE0_OFFSET: u32 = 4;
    pub const CORE0_MASK: u32 = 0x00000010;
    pub const CORE0: u32 = 1 << 4;

    /// **SP**
    ///
    /// If 1, ROM can be accessed from a Secure, Privileged context.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SP_OFFSET: u32 = 3;
    pub const SP_MASK: u32 = 0x00000008;
    pub const SP: u32 = 1 << 3;

    /// **SU**
    ///
    /// If 1, and SP is also set, ROM can be accessed from a Secure, Unprivileged context.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SU_OFFSET: u32 = 2;
    pub const SU_MASK: u32 = 0x00000004;
    pub const SU: u32 = 1 << 2;

    /// **NSP**
    ///
    /// If 1, ROM can be accessed from a Non-secure, Privileged context.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const NSP_OFFSET: u32 = 1;
    pub const NSP_MASK: u32 = 0x00000002;
    pub const NSP: u32 = 1 << 1;

    /// **NSU**
    ///
    /// If 1, and NSP is also set, ROM can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const NSU_OFFSET: u32 = 0;
    pub const NSU_MASK: u32 = 0x00000001;
    pub const NSU: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DBG** `[7]` (RW)
    ///
    /// If 1, ROM can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dbg(&self) -> bool {
        (self.value.read_bits() & Self::DBG_MASK) != 0
    }

    /// Set DBG bit atomically (RW)
    #[inline(always)]
    pub fn set_dbg(&self) {
        self.set_bits(Self::DBG_MASK);
    }

    /// Clear DBG bit atomically (RW)
    #[inline(always)]
    pub fn clr_dbg(&self) {
        self.clr_bits(Self::DBG_MASK);
    }

    /// **DMA** `[6]` (RW)
    ///
    /// If 1, ROM can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dma(&self) -> bool {
        (self.value.read_bits() & Self::DMA_MASK) != 0
    }

    /// Set DMA bit atomically (RW)
    #[inline(always)]
    pub fn set_dma(&self) {
        self.set_bits(Self::DMA_MASK);
    }

    /// Clear DMA bit atomically (RW)
    #[inline(always)]
    pub fn clr_dma(&self) {
        self.clr_bits(Self::DMA_MASK);
    }

    /// **CORE1** `[5]` (RW)
    ///
    /// If 1, ROM can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core1(&self) -> bool {
        (self.value.read_bits() & Self::CORE1_MASK) != 0
    }

    /// Set CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn set_core1(&self) {
        self.set_bits(Self::CORE1_MASK);
    }

    /// Clear CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core1(&self) {
        self.clr_bits(Self::CORE1_MASK);
    }

    /// **CORE0** `[4]` (RW)
    ///
    /// If 1, ROM can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core0(&self) -> bool {
        (self.value.read_bits() & Self::CORE0_MASK) != 0
    }

    /// Set CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn set_core0(&self) {
        self.set_bits(Self::CORE0_MASK);
    }

    /// Clear CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core0(&self) {
        self.clr_bits(Self::CORE0_MASK);
    }

    /// **SP** `[3]` (RW)
    ///
    /// If 1, ROM can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub fn sp(&self) -> bool {
        (self.value.read_bits() & Self::SP_MASK) != 0
    }

    /// Set SP bit atomically (RW)
    #[inline(always)]
    pub fn set_sp(&self) {
        self.set_bits(Self::SP_MASK);
    }

    /// Clear SP bit atomically (RW)
    #[inline(always)]
    pub fn clr_sp(&self) {
        self.clr_bits(Self::SP_MASK);
    }

    /// **SU** `[2]` (RW)
    ///
    /// If 1, and SP is also set, ROM can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub fn su(&self) -> bool {
        (self.value.read_bits() & Self::SU_MASK) != 0
    }

    /// Set SU bit atomically (RW)
    #[inline(always)]
    pub fn set_su(&self) {
        self.set_bits(Self::SU_MASK);
    }

    /// Clear SU bit atomically (RW)
    #[inline(always)]
    pub fn clr_su(&self) {
        self.clr_bits(Self::SU_MASK);
    }

    /// **NSP** `[1]` (RW)
    ///
    /// If 1, ROM can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub fn nsp(&self) -> bool {
        (self.value.read_bits() & Self::NSP_MASK) != 0
    }

    /// Set NSP bit atomically (RW)
    #[inline(always)]
    pub fn set_nsp(&self) {
        self.set_bits(Self::NSP_MASK);
    }

    /// Clear NSP bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsp(&self) {
        self.clr_bits(Self::NSP_MASK);
    }

    /// **NSU** `[0]` (RW)
    ///
    /// If 1, and NSP is also set, ROM can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub fn nsu(&self) -> bool {
        (self.value.read_bits() & Self::NSU_MASK) != 0
    }

    /// Set NSU bit atomically (RW)
    #[inline(always)]
    pub fn set_nsu(&self) {
        self.set_bits(Self::NSU_MASK);
    }

    /// Clear NSU bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsu(&self) {
        self.clr_bits(Self::NSU_MASK);
    }
}

/// ROM value type for batch read/modify/write operations
///
/// Use `rom.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rom.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RomVal {
    bits: u32,
}

impl RomVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
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

    /// Read **DBG** `[7:7]`
    ///
    /// If 1, ROM can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        (self.bits & Rom::DBG_MASK) != 0
    }

    /// Read **DMA** `[6:6]`
    ///
    /// If 1, ROM can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        (self.bits & Rom::DMA_MASK) != 0
    }

    /// Read **CORE1** `[5:5]`
    ///
    /// If 1, ROM can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        (self.bits & Rom::CORE1_MASK) != 0
    }

    /// Read **CORE0** `[4:4]`
    ///
    /// If 1, ROM can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        (self.bits & Rom::CORE0_MASK) != 0
    }

    /// Read **SP** `[3:3]`
    ///
    /// If 1, ROM can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        (self.bits & Rom::SP_MASK) != 0
    }

    /// Read **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, ROM can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn su(&self) -> bool {
        (self.bits & Rom::SU_MASK) != 0
    }

    /// Read **NSP** `[1:1]`
    ///
    /// If 1, ROM can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        (self.bits & Rom::NSP_MASK) != 0
    }

    /// Read **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, ROM can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        (self.bits & Rom::NSU_MASK) != 0
    }

    /// Set **DBG** `[7:7]`
    ///
    /// If 1, ROM can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dbg(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Rom::DBG_MASK
            } else {
                self.bits & !Rom::DBG_MASK
            }
        }
    }

    /// Set **DMA** `[6:6]`
    ///
    /// If 1, ROM can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dma(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Rom::DMA_MASK
            } else {
                self.bits & !Rom::DMA_MASK
            }
        }
    }

    /// Set **CORE1** `[5:5]`
    ///
    /// If 1, ROM can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Rom::CORE1_MASK
            } else {
                self.bits & !Rom::CORE1_MASK
            }
        }
    }

    /// Set **CORE0** `[4:4]`
    ///
    /// If 1, ROM can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Rom::CORE0_MASK
            } else {
                self.bits & !Rom::CORE0_MASK
            }
        }
    }

    /// Set **SP** `[3:3]`
    ///
    /// If 1, ROM can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn with_sp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Rom::SP_MASK
            } else {
                self.bits & !Rom::SP_MASK
            }
        }
    }

    /// Set **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, ROM can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn with_su(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Rom::SU_MASK
            } else {
                self.bits & !Rom::SU_MASK
            }
        }
    }

    /// Set **NSP** `[1:1]`
    ///
    /// If 1, ROM can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn with_nsp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Rom::NSP_MASK
            } else {
                self.bits & !Rom::NSP_MASK
            }
        }
    }

    /// Set **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, ROM can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn with_nsu(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Rom::NSU_MASK
            } else {
                self.bits & !Rom::NSU_MASK
            }
        }
    }
}

impl Default for RomVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Rom {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RomVal {
        RomVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RomVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RomVal) -> RomVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// XIP_MAIN Register
///
/// Control whether debugger, DMA, core 0 and core 1 can access XIP_MAIN, and at what security/privilege levels they can do so.
/// Defaults to fully open access.
/// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
///
/// Address offset: 0x0018
/// Reset value: 0x000000ff
///
/// ## Fields
///
/// - **DBG** `[7]` (RW)
///
///   If 1, XIP_MAIN can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **DMA** `[6]` (RW)
///
///   If 1, XIP_MAIN can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE1** `[5]` (RW)
///
///   If 1, XIP_MAIN can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE0** `[4]` (RW)
///
///   If 1, XIP_MAIN can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **SP** `[3]` (RW)
///
///   If 1, XIP_MAIN can be accessed from a Secure, Privileged context.
///
/// - **SU** `[2]` (RW)
///
///   If 1, and SP is also set, XIP_MAIN can be accessed from a Secure, Unprivileged context.
///
/// - **NSP** `[1]` (RW)
///
///   If 1, XIP_MAIN can be accessed from a Non-secure, Privileged context.
///
/// - **NSU** `[0]` (RW)
///
///   If 1, and NSP is also set, XIP_MAIN can be accessed from a Non-secure, Unprivileged context.
///   This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
#[repr(transparent)]
pub struct XipMain {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for XipMain {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XipMain {
    // ======== Field Constants ========

    /// **DBG**
    ///
    /// If 1, XIP_MAIN can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const DBG_OFFSET: u32 = 7;
    pub const DBG_MASK: u32 = 0x00000080;
    pub const DBG: u32 = 1 << 7;

    /// **DMA**
    ///
    /// If 1, XIP_MAIN can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const DMA_OFFSET: u32 = 6;
    pub const DMA_MASK: u32 = 0x00000040;
    pub const DMA: u32 = 1 << 6;

    /// **CORE1**
    ///
    /// If 1, XIP_MAIN can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CORE1_OFFSET: u32 = 5;
    pub const CORE1_MASK: u32 = 0x00000020;
    pub const CORE1: u32 = 1 << 5;

    /// **CORE0**
    ///
    /// If 1, XIP_MAIN can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CORE0_OFFSET: u32 = 4;
    pub const CORE0_MASK: u32 = 0x00000010;
    pub const CORE0: u32 = 1 << 4;

    /// **SP**
    ///
    /// If 1, XIP_MAIN can be accessed from a Secure, Privileged context.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SP_OFFSET: u32 = 3;
    pub const SP_MASK: u32 = 0x00000008;
    pub const SP: u32 = 1 << 3;

    /// **SU**
    ///
    /// If 1, and SP is also set, XIP_MAIN can be accessed from a Secure, Unprivileged context.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SU_OFFSET: u32 = 2;
    pub const SU_MASK: u32 = 0x00000004;
    pub const SU: u32 = 1 << 2;

    /// **NSP**
    ///
    /// If 1, XIP_MAIN can be accessed from a Non-secure, Privileged context.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const NSP_OFFSET: u32 = 1;
    pub const NSP_MASK: u32 = 0x00000002;
    pub const NSP: u32 = 1 << 1;

    /// **NSU**
    ///
    /// If 1, and NSP is also set, XIP_MAIN can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const NSU_OFFSET: u32 = 0;
    pub const NSU_MASK: u32 = 0x00000001;
    pub const NSU: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DBG** `[7]` (RW)
    ///
    /// If 1, XIP_MAIN can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dbg(&self) -> bool {
        (self.value.read_bits() & Self::DBG_MASK) != 0
    }

    /// Set DBG bit atomically (RW)
    #[inline(always)]
    pub fn set_dbg(&self) {
        self.set_bits(Self::DBG_MASK);
    }

    /// Clear DBG bit atomically (RW)
    #[inline(always)]
    pub fn clr_dbg(&self) {
        self.clr_bits(Self::DBG_MASK);
    }

    /// **DMA** `[6]` (RW)
    ///
    /// If 1, XIP_MAIN can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dma(&self) -> bool {
        (self.value.read_bits() & Self::DMA_MASK) != 0
    }

    /// Set DMA bit atomically (RW)
    #[inline(always)]
    pub fn set_dma(&self) {
        self.set_bits(Self::DMA_MASK);
    }

    /// Clear DMA bit atomically (RW)
    #[inline(always)]
    pub fn clr_dma(&self) {
        self.clr_bits(Self::DMA_MASK);
    }

    /// **CORE1** `[5]` (RW)
    ///
    /// If 1, XIP_MAIN can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core1(&self) -> bool {
        (self.value.read_bits() & Self::CORE1_MASK) != 0
    }

    /// Set CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn set_core1(&self) {
        self.set_bits(Self::CORE1_MASK);
    }

    /// Clear CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core1(&self) {
        self.clr_bits(Self::CORE1_MASK);
    }

    /// **CORE0** `[4]` (RW)
    ///
    /// If 1, XIP_MAIN can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core0(&self) -> bool {
        (self.value.read_bits() & Self::CORE0_MASK) != 0
    }

    /// Set CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn set_core0(&self) {
        self.set_bits(Self::CORE0_MASK);
    }

    /// Clear CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core0(&self) {
        self.clr_bits(Self::CORE0_MASK);
    }

    /// **SP** `[3]` (RW)
    ///
    /// If 1, XIP_MAIN can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub fn sp(&self) -> bool {
        (self.value.read_bits() & Self::SP_MASK) != 0
    }

    /// Set SP bit atomically (RW)
    #[inline(always)]
    pub fn set_sp(&self) {
        self.set_bits(Self::SP_MASK);
    }

    /// Clear SP bit atomically (RW)
    #[inline(always)]
    pub fn clr_sp(&self) {
        self.clr_bits(Self::SP_MASK);
    }

    /// **SU** `[2]` (RW)
    ///
    /// If 1, and SP is also set, XIP_MAIN can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub fn su(&self) -> bool {
        (self.value.read_bits() & Self::SU_MASK) != 0
    }

    /// Set SU bit atomically (RW)
    #[inline(always)]
    pub fn set_su(&self) {
        self.set_bits(Self::SU_MASK);
    }

    /// Clear SU bit atomically (RW)
    #[inline(always)]
    pub fn clr_su(&self) {
        self.clr_bits(Self::SU_MASK);
    }

    /// **NSP** `[1]` (RW)
    ///
    /// If 1, XIP_MAIN can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub fn nsp(&self) -> bool {
        (self.value.read_bits() & Self::NSP_MASK) != 0
    }

    /// Set NSP bit atomically (RW)
    #[inline(always)]
    pub fn set_nsp(&self) {
        self.set_bits(Self::NSP_MASK);
    }

    /// Clear NSP bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsp(&self) {
        self.clr_bits(Self::NSP_MASK);
    }

    /// **NSU** `[0]` (RW)
    ///
    /// If 1, and NSP is also set, XIP_MAIN can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub fn nsu(&self) -> bool {
        (self.value.read_bits() & Self::NSU_MASK) != 0
    }

    /// Set NSU bit atomically (RW)
    #[inline(always)]
    pub fn set_nsu(&self) {
        self.set_bits(Self::NSU_MASK);
    }

    /// Clear NSU bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsu(&self) {
        self.clr_bits(Self::NSU_MASK);
    }
}

/// XIP_MAIN value type for batch read/modify/write operations
///
/// Use `xipmain.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `xipmain.write_val(val)`.
#[derive(Clone, Copy)]
pub struct XipMainVal {
    bits: u32,
}

impl XipMainVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
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

    /// Read **DBG** `[7:7]`
    ///
    /// If 1, XIP_MAIN can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        (self.bits & XipMain::DBG_MASK) != 0
    }

    /// Read **DMA** `[6:6]`
    ///
    /// If 1, XIP_MAIN can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        (self.bits & XipMain::DMA_MASK) != 0
    }

    /// Read **CORE1** `[5:5]`
    ///
    /// If 1, XIP_MAIN can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        (self.bits & XipMain::CORE1_MASK) != 0
    }

    /// Read **CORE0** `[4:4]`
    ///
    /// If 1, XIP_MAIN can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        (self.bits & XipMain::CORE0_MASK) != 0
    }

    /// Read **SP** `[3:3]`
    ///
    /// If 1, XIP_MAIN can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        (self.bits & XipMain::SP_MASK) != 0
    }

    /// Read **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, XIP_MAIN can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn su(&self) -> bool {
        (self.bits & XipMain::SU_MASK) != 0
    }

    /// Read **NSP** `[1:1]`
    ///
    /// If 1, XIP_MAIN can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        (self.bits & XipMain::NSP_MASK) != 0
    }

    /// Read **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, XIP_MAIN can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        (self.bits & XipMain::NSU_MASK) != 0
    }

    /// Set **DBG** `[7:7]`
    ///
    /// If 1, XIP_MAIN can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dbg(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | XipMain::DBG_MASK
            } else {
                self.bits & !XipMain::DBG_MASK
            }
        }
    }

    /// Set **DMA** `[6:6]`
    ///
    /// If 1, XIP_MAIN can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dma(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | XipMain::DMA_MASK
            } else {
                self.bits & !XipMain::DMA_MASK
            }
        }
    }

    /// Set **CORE1** `[5:5]`
    ///
    /// If 1, XIP_MAIN can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | XipMain::CORE1_MASK
            } else {
                self.bits & !XipMain::CORE1_MASK
            }
        }
    }

    /// Set **CORE0** `[4:4]`
    ///
    /// If 1, XIP_MAIN can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | XipMain::CORE0_MASK
            } else {
                self.bits & !XipMain::CORE0_MASK
            }
        }
    }

    /// Set **SP** `[3:3]`
    ///
    /// If 1, XIP_MAIN can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn with_sp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | XipMain::SP_MASK
            } else {
                self.bits & !XipMain::SP_MASK
            }
        }
    }

    /// Set **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, XIP_MAIN can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn with_su(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | XipMain::SU_MASK
            } else {
                self.bits & !XipMain::SU_MASK
            }
        }
    }

    /// Set **NSP** `[1:1]`
    ///
    /// If 1, XIP_MAIN can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn with_nsp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | XipMain::NSP_MASK
            } else {
                self.bits & !XipMain::NSP_MASK
            }
        }
    }

    /// Set **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, XIP_MAIN can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn with_nsu(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | XipMain::NSU_MASK
            } else {
                self.bits & !XipMain::NSU_MASK
            }
        }
    }
}

impl Default for XipMainVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl XipMain {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> XipMainVal {
        XipMainVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: XipMainVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(XipMainVal) -> XipMainVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SRAM0 Register
///
/// Control whether debugger, DMA, core 0 and core 1 can access SRAM0, and at what security/privilege levels they can do so.
/// Defaults to fully open access.
/// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
///
/// Address offset: 0x001c
/// Reset value: 0x000000ff
///
/// ## Fields
///
/// - **DBG** `[7]` (RW)
///
///   If 1, SRAM0 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **DMA** `[6]` (RW)
///
///   If 1, SRAM0 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE1** `[5]` (RW)
///
///   If 1, SRAM0 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE0** `[4]` (RW)
///
///   If 1, SRAM0 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **SP** `[3]` (RW)
///
///   If 1, SRAM0 can be accessed from a Secure, Privileged context.
///
/// - **SU** `[2]` (RW)
///
///   If 1, and SP is also set, SRAM0 can be accessed from a Secure, Unprivileged context.
///
/// - **NSP** `[1]` (RW)
///
///   If 1, SRAM0 can be accessed from a Non-secure, Privileged context.
///
/// - **NSU** `[0]` (RW)
///
///   If 1, and NSP is also set, SRAM0 can be accessed from a Non-secure, Unprivileged context.
///   This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
#[repr(transparent)]
pub struct Sram0 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Sram0 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sram0 {
    // ======== Field Constants ========

    /// **DBG**
    ///
    /// If 1, SRAM0 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const DBG_OFFSET: u32 = 7;
    pub const DBG_MASK: u32 = 0x00000080;
    pub const DBG: u32 = 1 << 7;

    /// **DMA**
    ///
    /// If 1, SRAM0 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const DMA_OFFSET: u32 = 6;
    pub const DMA_MASK: u32 = 0x00000040;
    pub const DMA: u32 = 1 << 6;

    /// **CORE1**
    ///
    /// If 1, SRAM0 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CORE1_OFFSET: u32 = 5;
    pub const CORE1_MASK: u32 = 0x00000020;
    pub const CORE1: u32 = 1 << 5;

    /// **CORE0**
    ///
    /// If 1, SRAM0 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CORE0_OFFSET: u32 = 4;
    pub const CORE0_MASK: u32 = 0x00000010;
    pub const CORE0: u32 = 1 << 4;

    /// **SP**
    ///
    /// If 1, SRAM0 can be accessed from a Secure, Privileged context.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SP_OFFSET: u32 = 3;
    pub const SP_MASK: u32 = 0x00000008;
    pub const SP: u32 = 1 << 3;

    /// **SU**
    ///
    /// If 1, and SP is also set, SRAM0 can be accessed from a Secure, Unprivileged context.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SU_OFFSET: u32 = 2;
    pub const SU_MASK: u32 = 0x00000004;
    pub const SU: u32 = 1 << 2;

    /// **NSP**
    ///
    /// If 1, SRAM0 can be accessed from a Non-secure, Privileged context.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const NSP_OFFSET: u32 = 1;
    pub const NSP_MASK: u32 = 0x00000002;
    pub const NSP: u32 = 1 << 1;

    /// **NSU**
    ///
    /// If 1, and NSP is also set, SRAM0 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const NSU_OFFSET: u32 = 0;
    pub const NSU_MASK: u32 = 0x00000001;
    pub const NSU: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DBG** `[7]` (RW)
    ///
    /// If 1, SRAM0 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dbg(&self) -> bool {
        (self.value.read_bits() & Self::DBG_MASK) != 0
    }

    /// Set DBG bit atomically (RW)
    #[inline(always)]
    pub fn set_dbg(&self) {
        self.set_bits(Self::DBG_MASK);
    }

    /// Clear DBG bit atomically (RW)
    #[inline(always)]
    pub fn clr_dbg(&self) {
        self.clr_bits(Self::DBG_MASK);
    }

    /// **DMA** `[6]` (RW)
    ///
    /// If 1, SRAM0 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dma(&self) -> bool {
        (self.value.read_bits() & Self::DMA_MASK) != 0
    }

    /// Set DMA bit atomically (RW)
    #[inline(always)]
    pub fn set_dma(&self) {
        self.set_bits(Self::DMA_MASK);
    }

    /// Clear DMA bit atomically (RW)
    #[inline(always)]
    pub fn clr_dma(&self) {
        self.clr_bits(Self::DMA_MASK);
    }

    /// **CORE1** `[5]` (RW)
    ///
    /// If 1, SRAM0 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core1(&self) -> bool {
        (self.value.read_bits() & Self::CORE1_MASK) != 0
    }

    /// Set CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn set_core1(&self) {
        self.set_bits(Self::CORE1_MASK);
    }

    /// Clear CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core1(&self) {
        self.clr_bits(Self::CORE1_MASK);
    }

    /// **CORE0** `[4]` (RW)
    ///
    /// If 1, SRAM0 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core0(&self) -> bool {
        (self.value.read_bits() & Self::CORE0_MASK) != 0
    }

    /// Set CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn set_core0(&self) {
        self.set_bits(Self::CORE0_MASK);
    }

    /// Clear CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core0(&self) {
        self.clr_bits(Self::CORE0_MASK);
    }

    /// **SP** `[3]` (RW)
    ///
    /// If 1, SRAM0 can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub fn sp(&self) -> bool {
        (self.value.read_bits() & Self::SP_MASK) != 0
    }

    /// Set SP bit atomically (RW)
    #[inline(always)]
    pub fn set_sp(&self) {
        self.set_bits(Self::SP_MASK);
    }

    /// Clear SP bit atomically (RW)
    #[inline(always)]
    pub fn clr_sp(&self) {
        self.clr_bits(Self::SP_MASK);
    }

    /// **SU** `[2]` (RW)
    ///
    /// If 1, and SP is also set, SRAM0 can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub fn su(&self) -> bool {
        (self.value.read_bits() & Self::SU_MASK) != 0
    }

    /// Set SU bit atomically (RW)
    #[inline(always)]
    pub fn set_su(&self) {
        self.set_bits(Self::SU_MASK);
    }

    /// Clear SU bit atomically (RW)
    #[inline(always)]
    pub fn clr_su(&self) {
        self.clr_bits(Self::SU_MASK);
    }

    /// **NSP** `[1]` (RW)
    ///
    /// If 1, SRAM0 can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub fn nsp(&self) -> bool {
        (self.value.read_bits() & Self::NSP_MASK) != 0
    }

    /// Set NSP bit atomically (RW)
    #[inline(always)]
    pub fn set_nsp(&self) {
        self.set_bits(Self::NSP_MASK);
    }

    /// Clear NSP bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsp(&self) {
        self.clr_bits(Self::NSP_MASK);
    }

    /// **NSU** `[0]` (RW)
    ///
    /// If 1, and NSP is also set, SRAM0 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub fn nsu(&self) -> bool {
        (self.value.read_bits() & Self::NSU_MASK) != 0
    }

    /// Set NSU bit atomically (RW)
    #[inline(always)]
    pub fn set_nsu(&self) {
        self.set_bits(Self::NSU_MASK);
    }

    /// Clear NSU bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsu(&self) {
        self.clr_bits(Self::NSU_MASK);
    }
}

/// SRAM0 value type for batch read/modify/write operations
///
/// Use `sram0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sram0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Sram0Val {
    bits: u32,
}

impl Sram0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
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

    /// Read **DBG** `[7:7]`
    ///
    /// If 1, SRAM0 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        (self.bits & Sram0::DBG_MASK) != 0
    }

    /// Read **DMA** `[6:6]`
    ///
    /// If 1, SRAM0 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        (self.bits & Sram0::DMA_MASK) != 0
    }

    /// Read **CORE1** `[5:5]`
    ///
    /// If 1, SRAM0 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        (self.bits & Sram0::CORE1_MASK) != 0
    }

    /// Read **CORE0** `[4:4]`
    ///
    /// If 1, SRAM0 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        (self.bits & Sram0::CORE0_MASK) != 0
    }

    /// Read **SP** `[3:3]`
    ///
    /// If 1, SRAM0 can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        (self.bits & Sram0::SP_MASK) != 0
    }

    /// Read **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, SRAM0 can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn su(&self) -> bool {
        (self.bits & Sram0::SU_MASK) != 0
    }

    /// Read **NSP** `[1:1]`
    ///
    /// If 1, SRAM0 can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        (self.bits & Sram0::NSP_MASK) != 0
    }

    /// Read **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, SRAM0 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        (self.bits & Sram0::NSU_MASK) != 0
    }

    /// Set **DBG** `[7:7]`
    ///
    /// If 1, SRAM0 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dbg(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram0::DBG_MASK
            } else {
                self.bits & !Sram0::DBG_MASK
            }
        }
    }

    /// Set **DMA** `[6:6]`
    ///
    /// If 1, SRAM0 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dma(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram0::DMA_MASK
            } else {
                self.bits & !Sram0::DMA_MASK
            }
        }
    }

    /// Set **CORE1** `[5:5]`
    ///
    /// If 1, SRAM0 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram0::CORE1_MASK
            } else {
                self.bits & !Sram0::CORE1_MASK
            }
        }
    }

    /// Set **CORE0** `[4:4]`
    ///
    /// If 1, SRAM0 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram0::CORE0_MASK
            } else {
                self.bits & !Sram0::CORE0_MASK
            }
        }
    }

    /// Set **SP** `[3:3]`
    ///
    /// If 1, SRAM0 can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn with_sp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram0::SP_MASK
            } else {
                self.bits & !Sram0::SP_MASK
            }
        }
    }

    /// Set **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, SRAM0 can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn with_su(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram0::SU_MASK
            } else {
                self.bits & !Sram0::SU_MASK
            }
        }
    }

    /// Set **NSP** `[1:1]`
    ///
    /// If 1, SRAM0 can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn with_nsp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram0::NSP_MASK
            } else {
                self.bits & !Sram0::NSP_MASK
            }
        }
    }

    /// Set **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, SRAM0 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn with_nsu(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram0::NSU_MASK
            } else {
                self.bits & !Sram0::NSU_MASK
            }
        }
    }
}

impl Default for Sram0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Sram0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Sram0Val {
        Sram0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Sram0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Sram0Val) -> Sram0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SRAM1 Register
///
/// Control whether debugger, DMA, core 0 and core 1 can access SRAM1, and at what security/privilege levels they can do so.
/// Defaults to fully open access.
/// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
///
/// Address offset: 0x0020
/// Reset value: 0x000000ff
///
/// ## Fields
///
/// - **DBG** `[7]` (RW)
///
///   If 1, SRAM1 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **DMA** `[6]` (RW)
///
///   If 1, SRAM1 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE1** `[5]` (RW)
///
///   If 1, SRAM1 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE0** `[4]` (RW)
///
///   If 1, SRAM1 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **SP** `[3]` (RW)
///
///   If 1, SRAM1 can be accessed from a Secure, Privileged context.
///
/// - **SU** `[2]` (RW)
///
///   If 1, and SP is also set, SRAM1 can be accessed from a Secure, Unprivileged context.
///
/// - **NSP** `[1]` (RW)
///
///   If 1, SRAM1 can be accessed from a Non-secure, Privileged context.
///
/// - **NSU** `[0]` (RW)
///
///   If 1, and NSP is also set, SRAM1 can be accessed from a Non-secure, Unprivileged context.
///   This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
#[repr(transparent)]
pub struct Sram1 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Sram1 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sram1 {
    // ======== Field Constants ========

    /// **DBG**
    ///
    /// If 1, SRAM1 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const DBG_OFFSET: u32 = 7;
    pub const DBG_MASK: u32 = 0x00000080;
    pub const DBG: u32 = 1 << 7;

    /// **DMA**
    ///
    /// If 1, SRAM1 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const DMA_OFFSET: u32 = 6;
    pub const DMA_MASK: u32 = 0x00000040;
    pub const DMA: u32 = 1 << 6;

    /// **CORE1**
    ///
    /// If 1, SRAM1 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CORE1_OFFSET: u32 = 5;
    pub const CORE1_MASK: u32 = 0x00000020;
    pub const CORE1: u32 = 1 << 5;

    /// **CORE0**
    ///
    /// If 1, SRAM1 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CORE0_OFFSET: u32 = 4;
    pub const CORE0_MASK: u32 = 0x00000010;
    pub const CORE0: u32 = 1 << 4;

    /// **SP**
    ///
    /// If 1, SRAM1 can be accessed from a Secure, Privileged context.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SP_OFFSET: u32 = 3;
    pub const SP_MASK: u32 = 0x00000008;
    pub const SP: u32 = 1 << 3;

    /// **SU**
    ///
    /// If 1, and SP is also set, SRAM1 can be accessed from a Secure, Unprivileged context.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SU_OFFSET: u32 = 2;
    pub const SU_MASK: u32 = 0x00000004;
    pub const SU: u32 = 1 << 2;

    /// **NSP**
    ///
    /// If 1, SRAM1 can be accessed from a Non-secure, Privileged context.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const NSP_OFFSET: u32 = 1;
    pub const NSP_MASK: u32 = 0x00000002;
    pub const NSP: u32 = 1 << 1;

    /// **NSU**
    ///
    /// If 1, and NSP is also set, SRAM1 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const NSU_OFFSET: u32 = 0;
    pub const NSU_MASK: u32 = 0x00000001;
    pub const NSU: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DBG** `[7]` (RW)
    ///
    /// If 1, SRAM1 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dbg(&self) -> bool {
        (self.value.read_bits() & Self::DBG_MASK) != 0
    }

    /// Set DBG bit atomically (RW)
    #[inline(always)]
    pub fn set_dbg(&self) {
        self.set_bits(Self::DBG_MASK);
    }

    /// Clear DBG bit atomically (RW)
    #[inline(always)]
    pub fn clr_dbg(&self) {
        self.clr_bits(Self::DBG_MASK);
    }

    /// **DMA** `[6]` (RW)
    ///
    /// If 1, SRAM1 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dma(&self) -> bool {
        (self.value.read_bits() & Self::DMA_MASK) != 0
    }

    /// Set DMA bit atomically (RW)
    #[inline(always)]
    pub fn set_dma(&self) {
        self.set_bits(Self::DMA_MASK);
    }

    /// Clear DMA bit atomically (RW)
    #[inline(always)]
    pub fn clr_dma(&self) {
        self.clr_bits(Self::DMA_MASK);
    }

    /// **CORE1** `[5]` (RW)
    ///
    /// If 1, SRAM1 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core1(&self) -> bool {
        (self.value.read_bits() & Self::CORE1_MASK) != 0
    }

    /// Set CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn set_core1(&self) {
        self.set_bits(Self::CORE1_MASK);
    }

    /// Clear CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core1(&self) {
        self.clr_bits(Self::CORE1_MASK);
    }

    /// **CORE0** `[4]` (RW)
    ///
    /// If 1, SRAM1 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core0(&self) -> bool {
        (self.value.read_bits() & Self::CORE0_MASK) != 0
    }

    /// Set CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn set_core0(&self) {
        self.set_bits(Self::CORE0_MASK);
    }

    /// Clear CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core0(&self) {
        self.clr_bits(Self::CORE0_MASK);
    }

    /// **SP** `[3]` (RW)
    ///
    /// If 1, SRAM1 can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub fn sp(&self) -> bool {
        (self.value.read_bits() & Self::SP_MASK) != 0
    }

    /// Set SP bit atomically (RW)
    #[inline(always)]
    pub fn set_sp(&self) {
        self.set_bits(Self::SP_MASK);
    }

    /// Clear SP bit atomically (RW)
    #[inline(always)]
    pub fn clr_sp(&self) {
        self.clr_bits(Self::SP_MASK);
    }

    /// **SU** `[2]` (RW)
    ///
    /// If 1, and SP is also set, SRAM1 can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub fn su(&self) -> bool {
        (self.value.read_bits() & Self::SU_MASK) != 0
    }

    /// Set SU bit atomically (RW)
    #[inline(always)]
    pub fn set_su(&self) {
        self.set_bits(Self::SU_MASK);
    }

    /// Clear SU bit atomically (RW)
    #[inline(always)]
    pub fn clr_su(&self) {
        self.clr_bits(Self::SU_MASK);
    }

    /// **NSP** `[1]` (RW)
    ///
    /// If 1, SRAM1 can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub fn nsp(&self) -> bool {
        (self.value.read_bits() & Self::NSP_MASK) != 0
    }

    /// Set NSP bit atomically (RW)
    #[inline(always)]
    pub fn set_nsp(&self) {
        self.set_bits(Self::NSP_MASK);
    }

    /// Clear NSP bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsp(&self) {
        self.clr_bits(Self::NSP_MASK);
    }

    /// **NSU** `[0]` (RW)
    ///
    /// If 1, and NSP is also set, SRAM1 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub fn nsu(&self) -> bool {
        (self.value.read_bits() & Self::NSU_MASK) != 0
    }

    /// Set NSU bit atomically (RW)
    #[inline(always)]
    pub fn set_nsu(&self) {
        self.set_bits(Self::NSU_MASK);
    }

    /// Clear NSU bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsu(&self) {
        self.clr_bits(Self::NSU_MASK);
    }
}

/// SRAM1 value type for batch read/modify/write operations
///
/// Use `sram1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sram1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Sram1Val {
    bits: u32,
}

impl Sram1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
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

    /// Read **DBG** `[7:7]`
    ///
    /// If 1, SRAM1 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        (self.bits & Sram1::DBG_MASK) != 0
    }

    /// Read **DMA** `[6:6]`
    ///
    /// If 1, SRAM1 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        (self.bits & Sram1::DMA_MASK) != 0
    }

    /// Read **CORE1** `[5:5]`
    ///
    /// If 1, SRAM1 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        (self.bits & Sram1::CORE1_MASK) != 0
    }

    /// Read **CORE0** `[4:4]`
    ///
    /// If 1, SRAM1 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        (self.bits & Sram1::CORE0_MASK) != 0
    }

    /// Read **SP** `[3:3]`
    ///
    /// If 1, SRAM1 can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        (self.bits & Sram1::SP_MASK) != 0
    }

    /// Read **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, SRAM1 can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn su(&self) -> bool {
        (self.bits & Sram1::SU_MASK) != 0
    }

    /// Read **NSP** `[1:1]`
    ///
    /// If 1, SRAM1 can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        (self.bits & Sram1::NSP_MASK) != 0
    }

    /// Read **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, SRAM1 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        (self.bits & Sram1::NSU_MASK) != 0
    }

    /// Set **DBG** `[7:7]`
    ///
    /// If 1, SRAM1 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dbg(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram1::DBG_MASK
            } else {
                self.bits & !Sram1::DBG_MASK
            }
        }
    }

    /// Set **DMA** `[6:6]`
    ///
    /// If 1, SRAM1 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dma(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram1::DMA_MASK
            } else {
                self.bits & !Sram1::DMA_MASK
            }
        }
    }

    /// Set **CORE1** `[5:5]`
    ///
    /// If 1, SRAM1 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram1::CORE1_MASK
            } else {
                self.bits & !Sram1::CORE1_MASK
            }
        }
    }

    /// Set **CORE0** `[4:4]`
    ///
    /// If 1, SRAM1 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram1::CORE0_MASK
            } else {
                self.bits & !Sram1::CORE0_MASK
            }
        }
    }

    /// Set **SP** `[3:3]`
    ///
    /// If 1, SRAM1 can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn with_sp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram1::SP_MASK
            } else {
                self.bits & !Sram1::SP_MASK
            }
        }
    }

    /// Set **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, SRAM1 can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn with_su(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram1::SU_MASK
            } else {
                self.bits & !Sram1::SU_MASK
            }
        }
    }

    /// Set **NSP** `[1:1]`
    ///
    /// If 1, SRAM1 can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn with_nsp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram1::NSP_MASK
            } else {
                self.bits & !Sram1::NSP_MASK
            }
        }
    }

    /// Set **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, SRAM1 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn with_nsu(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram1::NSU_MASK
            } else {
                self.bits & !Sram1::NSU_MASK
            }
        }
    }
}

impl Default for Sram1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Sram1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Sram1Val {
        Sram1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Sram1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Sram1Val) -> Sram1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SRAM2 Register
///
/// Control whether debugger, DMA, core 0 and core 1 can access SRAM2, and at what security/privilege levels they can do so.
/// Defaults to fully open access.
/// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
///
/// Address offset: 0x0024
/// Reset value: 0x000000ff
///
/// ## Fields
///
/// - **DBG** `[7]` (RW)
///
///   If 1, SRAM2 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **DMA** `[6]` (RW)
///
///   If 1, SRAM2 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE1** `[5]` (RW)
///
///   If 1, SRAM2 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE0** `[4]` (RW)
///
///   If 1, SRAM2 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **SP** `[3]` (RW)
///
///   If 1, SRAM2 can be accessed from a Secure, Privileged context.
///
/// - **SU** `[2]` (RW)
///
///   If 1, and SP is also set, SRAM2 can be accessed from a Secure, Unprivileged context.
///
/// - **NSP** `[1]` (RW)
///
///   If 1, SRAM2 can be accessed from a Non-secure, Privileged context.
///
/// - **NSU** `[0]` (RW)
///
///   If 1, and NSP is also set, SRAM2 can be accessed from a Non-secure, Unprivileged context.
///   This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
#[repr(transparent)]
pub struct Sram2 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Sram2 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sram2 {
    // ======== Field Constants ========

    /// **DBG**
    ///
    /// If 1, SRAM2 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const DBG_OFFSET: u32 = 7;
    pub const DBG_MASK: u32 = 0x00000080;
    pub const DBG: u32 = 1 << 7;

    /// **DMA**
    ///
    /// If 1, SRAM2 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const DMA_OFFSET: u32 = 6;
    pub const DMA_MASK: u32 = 0x00000040;
    pub const DMA: u32 = 1 << 6;

    /// **CORE1**
    ///
    /// If 1, SRAM2 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CORE1_OFFSET: u32 = 5;
    pub const CORE1_MASK: u32 = 0x00000020;
    pub const CORE1: u32 = 1 << 5;

    /// **CORE0**
    ///
    /// If 1, SRAM2 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CORE0_OFFSET: u32 = 4;
    pub const CORE0_MASK: u32 = 0x00000010;
    pub const CORE0: u32 = 1 << 4;

    /// **SP**
    ///
    /// If 1, SRAM2 can be accessed from a Secure, Privileged context.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SP_OFFSET: u32 = 3;
    pub const SP_MASK: u32 = 0x00000008;
    pub const SP: u32 = 1 << 3;

    /// **SU**
    ///
    /// If 1, and SP is also set, SRAM2 can be accessed from a Secure, Unprivileged context.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SU_OFFSET: u32 = 2;
    pub const SU_MASK: u32 = 0x00000004;
    pub const SU: u32 = 1 << 2;

    /// **NSP**
    ///
    /// If 1, SRAM2 can be accessed from a Non-secure, Privileged context.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const NSP_OFFSET: u32 = 1;
    pub const NSP_MASK: u32 = 0x00000002;
    pub const NSP: u32 = 1 << 1;

    /// **NSU**
    ///
    /// If 1, and NSP is also set, SRAM2 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const NSU_OFFSET: u32 = 0;
    pub const NSU_MASK: u32 = 0x00000001;
    pub const NSU: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DBG** `[7]` (RW)
    ///
    /// If 1, SRAM2 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dbg(&self) -> bool {
        (self.value.read_bits() & Self::DBG_MASK) != 0
    }

    /// Set DBG bit atomically (RW)
    #[inline(always)]
    pub fn set_dbg(&self) {
        self.set_bits(Self::DBG_MASK);
    }

    /// Clear DBG bit atomically (RW)
    #[inline(always)]
    pub fn clr_dbg(&self) {
        self.clr_bits(Self::DBG_MASK);
    }

    /// **DMA** `[6]` (RW)
    ///
    /// If 1, SRAM2 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dma(&self) -> bool {
        (self.value.read_bits() & Self::DMA_MASK) != 0
    }

    /// Set DMA bit atomically (RW)
    #[inline(always)]
    pub fn set_dma(&self) {
        self.set_bits(Self::DMA_MASK);
    }

    /// Clear DMA bit atomically (RW)
    #[inline(always)]
    pub fn clr_dma(&self) {
        self.clr_bits(Self::DMA_MASK);
    }

    /// **CORE1** `[5]` (RW)
    ///
    /// If 1, SRAM2 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core1(&self) -> bool {
        (self.value.read_bits() & Self::CORE1_MASK) != 0
    }

    /// Set CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn set_core1(&self) {
        self.set_bits(Self::CORE1_MASK);
    }

    /// Clear CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core1(&self) {
        self.clr_bits(Self::CORE1_MASK);
    }

    /// **CORE0** `[4]` (RW)
    ///
    /// If 1, SRAM2 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core0(&self) -> bool {
        (self.value.read_bits() & Self::CORE0_MASK) != 0
    }

    /// Set CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn set_core0(&self) {
        self.set_bits(Self::CORE0_MASK);
    }

    /// Clear CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core0(&self) {
        self.clr_bits(Self::CORE0_MASK);
    }

    /// **SP** `[3]` (RW)
    ///
    /// If 1, SRAM2 can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub fn sp(&self) -> bool {
        (self.value.read_bits() & Self::SP_MASK) != 0
    }

    /// Set SP bit atomically (RW)
    #[inline(always)]
    pub fn set_sp(&self) {
        self.set_bits(Self::SP_MASK);
    }

    /// Clear SP bit atomically (RW)
    #[inline(always)]
    pub fn clr_sp(&self) {
        self.clr_bits(Self::SP_MASK);
    }

    /// **SU** `[2]` (RW)
    ///
    /// If 1, and SP is also set, SRAM2 can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub fn su(&self) -> bool {
        (self.value.read_bits() & Self::SU_MASK) != 0
    }

    /// Set SU bit atomically (RW)
    #[inline(always)]
    pub fn set_su(&self) {
        self.set_bits(Self::SU_MASK);
    }

    /// Clear SU bit atomically (RW)
    #[inline(always)]
    pub fn clr_su(&self) {
        self.clr_bits(Self::SU_MASK);
    }

    /// **NSP** `[1]` (RW)
    ///
    /// If 1, SRAM2 can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub fn nsp(&self) -> bool {
        (self.value.read_bits() & Self::NSP_MASK) != 0
    }

    /// Set NSP bit atomically (RW)
    #[inline(always)]
    pub fn set_nsp(&self) {
        self.set_bits(Self::NSP_MASK);
    }

    /// Clear NSP bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsp(&self) {
        self.clr_bits(Self::NSP_MASK);
    }

    /// **NSU** `[0]` (RW)
    ///
    /// If 1, and NSP is also set, SRAM2 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub fn nsu(&self) -> bool {
        (self.value.read_bits() & Self::NSU_MASK) != 0
    }

    /// Set NSU bit atomically (RW)
    #[inline(always)]
    pub fn set_nsu(&self) {
        self.set_bits(Self::NSU_MASK);
    }

    /// Clear NSU bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsu(&self) {
        self.clr_bits(Self::NSU_MASK);
    }
}

/// SRAM2 value type for batch read/modify/write operations
///
/// Use `sram2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sram2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Sram2Val {
    bits: u32,
}

impl Sram2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
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

    /// Read **DBG** `[7:7]`
    ///
    /// If 1, SRAM2 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        (self.bits & Sram2::DBG_MASK) != 0
    }

    /// Read **DMA** `[6:6]`
    ///
    /// If 1, SRAM2 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        (self.bits & Sram2::DMA_MASK) != 0
    }

    /// Read **CORE1** `[5:5]`
    ///
    /// If 1, SRAM2 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        (self.bits & Sram2::CORE1_MASK) != 0
    }

    /// Read **CORE0** `[4:4]`
    ///
    /// If 1, SRAM2 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        (self.bits & Sram2::CORE0_MASK) != 0
    }

    /// Read **SP** `[3:3]`
    ///
    /// If 1, SRAM2 can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        (self.bits & Sram2::SP_MASK) != 0
    }

    /// Read **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, SRAM2 can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn su(&self) -> bool {
        (self.bits & Sram2::SU_MASK) != 0
    }

    /// Read **NSP** `[1:1]`
    ///
    /// If 1, SRAM2 can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        (self.bits & Sram2::NSP_MASK) != 0
    }

    /// Read **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, SRAM2 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        (self.bits & Sram2::NSU_MASK) != 0
    }

    /// Set **DBG** `[7:7]`
    ///
    /// If 1, SRAM2 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dbg(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram2::DBG_MASK
            } else {
                self.bits & !Sram2::DBG_MASK
            }
        }
    }

    /// Set **DMA** `[6:6]`
    ///
    /// If 1, SRAM2 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dma(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram2::DMA_MASK
            } else {
                self.bits & !Sram2::DMA_MASK
            }
        }
    }

    /// Set **CORE1** `[5:5]`
    ///
    /// If 1, SRAM2 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram2::CORE1_MASK
            } else {
                self.bits & !Sram2::CORE1_MASK
            }
        }
    }

    /// Set **CORE0** `[4:4]`
    ///
    /// If 1, SRAM2 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram2::CORE0_MASK
            } else {
                self.bits & !Sram2::CORE0_MASK
            }
        }
    }

    /// Set **SP** `[3:3]`
    ///
    /// If 1, SRAM2 can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn with_sp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram2::SP_MASK
            } else {
                self.bits & !Sram2::SP_MASK
            }
        }
    }

    /// Set **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, SRAM2 can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn with_su(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram2::SU_MASK
            } else {
                self.bits & !Sram2::SU_MASK
            }
        }
    }

    /// Set **NSP** `[1:1]`
    ///
    /// If 1, SRAM2 can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn with_nsp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram2::NSP_MASK
            } else {
                self.bits & !Sram2::NSP_MASK
            }
        }
    }

    /// Set **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, SRAM2 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn with_nsu(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram2::NSU_MASK
            } else {
                self.bits & !Sram2::NSU_MASK
            }
        }
    }
}

impl Default for Sram2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Sram2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Sram2Val {
        Sram2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Sram2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Sram2Val) -> Sram2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SRAM3 Register
///
/// Control whether debugger, DMA, core 0 and core 1 can access SRAM3, and at what security/privilege levels they can do so.
/// Defaults to fully open access.
/// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
///
/// Address offset: 0x0028
/// Reset value: 0x000000ff
///
/// ## Fields
///
/// - **DBG** `[7]` (RW)
///
///   If 1, SRAM3 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **DMA** `[6]` (RW)
///
///   If 1, SRAM3 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE1** `[5]` (RW)
///
///   If 1, SRAM3 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE0** `[4]` (RW)
///
///   If 1, SRAM3 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **SP** `[3]` (RW)
///
///   If 1, SRAM3 can be accessed from a Secure, Privileged context.
///
/// - **SU** `[2]` (RW)
///
///   If 1, and SP is also set, SRAM3 can be accessed from a Secure, Unprivileged context.
///
/// - **NSP** `[1]` (RW)
///
///   If 1, SRAM3 can be accessed from a Non-secure, Privileged context.
///
/// - **NSU** `[0]` (RW)
///
///   If 1, and NSP is also set, SRAM3 can be accessed from a Non-secure, Unprivileged context.
///   This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
#[repr(transparent)]
pub struct Sram3 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Sram3 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sram3 {
    // ======== Field Constants ========

    /// **DBG**
    ///
    /// If 1, SRAM3 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const DBG_OFFSET: u32 = 7;
    pub const DBG_MASK: u32 = 0x00000080;
    pub const DBG: u32 = 1 << 7;

    /// **DMA**
    ///
    /// If 1, SRAM3 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const DMA_OFFSET: u32 = 6;
    pub const DMA_MASK: u32 = 0x00000040;
    pub const DMA: u32 = 1 << 6;

    /// **CORE1**
    ///
    /// If 1, SRAM3 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CORE1_OFFSET: u32 = 5;
    pub const CORE1_MASK: u32 = 0x00000020;
    pub const CORE1: u32 = 1 << 5;

    /// **CORE0**
    ///
    /// If 1, SRAM3 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CORE0_OFFSET: u32 = 4;
    pub const CORE0_MASK: u32 = 0x00000010;
    pub const CORE0: u32 = 1 << 4;

    /// **SP**
    ///
    /// If 1, SRAM3 can be accessed from a Secure, Privileged context.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SP_OFFSET: u32 = 3;
    pub const SP_MASK: u32 = 0x00000008;
    pub const SP: u32 = 1 << 3;

    /// **SU**
    ///
    /// If 1, and SP is also set, SRAM3 can be accessed from a Secure, Unprivileged context.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SU_OFFSET: u32 = 2;
    pub const SU_MASK: u32 = 0x00000004;
    pub const SU: u32 = 1 << 2;

    /// **NSP**
    ///
    /// If 1, SRAM3 can be accessed from a Non-secure, Privileged context.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const NSP_OFFSET: u32 = 1;
    pub const NSP_MASK: u32 = 0x00000002;
    pub const NSP: u32 = 1 << 1;

    /// **NSU**
    ///
    /// If 1, and NSP is also set, SRAM3 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const NSU_OFFSET: u32 = 0;
    pub const NSU_MASK: u32 = 0x00000001;
    pub const NSU: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DBG** `[7]` (RW)
    ///
    /// If 1, SRAM3 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dbg(&self) -> bool {
        (self.value.read_bits() & Self::DBG_MASK) != 0
    }

    /// Set DBG bit atomically (RW)
    #[inline(always)]
    pub fn set_dbg(&self) {
        self.set_bits(Self::DBG_MASK);
    }

    /// Clear DBG bit atomically (RW)
    #[inline(always)]
    pub fn clr_dbg(&self) {
        self.clr_bits(Self::DBG_MASK);
    }

    /// **DMA** `[6]` (RW)
    ///
    /// If 1, SRAM3 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dma(&self) -> bool {
        (self.value.read_bits() & Self::DMA_MASK) != 0
    }

    /// Set DMA bit atomically (RW)
    #[inline(always)]
    pub fn set_dma(&self) {
        self.set_bits(Self::DMA_MASK);
    }

    /// Clear DMA bit atomically (RW)
    #[inline(always)]
    pub fn clr_dma(&self) {
        self.clr_bits(Self::DMA_MASK);
    }

    /// **CORE1** `[5]` (RW)
    ///
    /// If 1, SRAM3 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core1(&self) -> bool {
        (self.value.read_bits() & Self::CORE1_MASK) != 0
    }

    /// Set CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn set_core1(&self) {
        self.set_bits(Self::CORE1_MASK);
    }

    /// Clear CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core1(&self) {
        self.clr_bits(Self::CORE1_MASK);
    }

    /// **CORE0** `[4]` (RW)
    ///
    /// If 1, SRAM3 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core0(&self) -> bool {
        (self.value.read_bits() & Self::CORE0_MASK) != 0
    }

    /// Set CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn set_core0(&self) {
        self.set_bits(Self::CORE0_MASK);
    }

    /// Clear CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core0(&self) {
        self.clr_bits(Self::CORE0_MASK);
    }

    /// **SP** `[3]` (RW)
    ///
    /// If 1, SRAM3 can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub fn sp(&self) -> bool {
        (self.value.read_bits() & Self::SP_MASK) != 0
    }

    /// Set SP bit atomically (RW)
    #[inline(always)]
    pub fn set_sp(&self) {
        self.set_bits(Self::SP_MASK);
    }

    /// Clear SP bit atomically (RW)
    #[inline(always)]
    pub fn clr_sp(&self) {
        self.clr_bits(Self::SP_MASK);
    }

    /// **SU** `[2]` (RW)
    ///
    /// If 1, and SP is also set, SRAM3 can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub fn su(&self) -> bool {
        (self.value.read_bits() & Self::SU_MASK) != 0
    }

    /// Set SU bit atomically (RW)
    #[inline(always)]
    pub fn set_su(&self) {
        self.set_bits(Self::SU_MASK);
    }

    /// Clear SU bit atomically (RW)
    #[inline(always)]
    pub fn clr_su(&self) {
        self.clr_bits(Self::SU_MASK);
    }

    /// **NSP** `[1]` (RW)
    ///
    /// If 1, SRAM3 can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub fn nsp(&self) -> bool {
        (self.value.read_bits() & Self::NSP_MASK) != 0
    }

    /// Set NSP bit atomically (RW)
    #[inline(always)]
    pub fn set_nsp(&self) {
        self.set_bits(Self::NSP_MASK);
    }

    /// Clear NSP bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsp(&self) {
        self.clr_bits(Self::NSP_MASK);
    }

    /// **NSU** `[0]` (RW)
    ///
    /// If 1, and NSP is also set, SRAM3 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub fn nsu(&self) -> bool {
        (self.value.read_bits() & Self::NSU_MASK) != 0
    }

    /// Set NSU bit atomically (RW)
    #[inline(always)]
    pub fn set_nsu(&self) {
        self.set_bits(Self::NSU_MASK);
    }

    /// Clear NSU bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsu(&self) {
        self.clr_bits(Self::NSU_MASK);
    }
}

/// SRAM3 value type for batch read/modify/write operations
///
/// Use `sram3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sram3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Sram3Val {
    bits: u32,
}

impl Sram3Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
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

    /// Read **DBG** `[7:7]`
    ///
    /// If 1, SRAM3 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        (self.bits & Sram3::DBG_MASK) != 0
    }

    /// Read **DMA** `[6:6]`
    ///
    /// If 1, SRAM3 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        (self.bits & Sram3::DMA_MASK) != 0
    }

    /// Read **CORE1** `[5:5]`
    ///
    /// If 1, SRAM3 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        (self.bits & Sram3::CORE1_MASK) != 0
    }

    /// Read **CORE0** `[4:4]`
    ///
    /// If 1, SRAM3 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        (self.bits & Sram3::CORE0_MASK) != 0
    }

    /// Read **SP** `[3:3]`
    ///
    /// If 1, SRAM3 can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        (self.bits & Sram3::SP_MASK) != 0
    }

    /// Read **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, SRAM3 can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn su(&self) -> bool {
        (self.bits & Sram3::SU_MASK) != 0
    }

    /// Read **NSP** `[1:1]`
    ///
    /// If 1, SRAM3 can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        (self.bits & Sram3::NSP_MASK) != 0
    }

    /// Read **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, SRAM3 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        (self.bits & Sram3::NSU_MASK) != 0
    }

    /// Set **DBG** `[7:7]`
    ///
    /// If 1, SRAM3 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dbg(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram3::DBG_MASK
            } else {
                self.bits & !Sram3::DBG_MASK
            }
        }
    }

    /// Set **DMA** `[6:6]`
    ///
    /// If 1, SRAM3 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dma(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram3::DMA_MASK
            } else {
                self.bits & !Sram3::DMA_MASK
            }
        }
    }

    /// Set **CORE1** `[5:5]`
    ///
    /// If 1, SRAM3 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram3::CORE1_MASK
            } else {
                self.bits & !Sram3::CORE1_MASK
            }
        }
    }

    /// Set **CORE0** `[4:4]`
    ///
    /// If 1, SRAM3 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram3::CORE0_MASK
            } else {
                self.bits & !Sram3::CORE0_MASK
            }
        }
    }

    /// Set **SP** `[3:3]`
    ///
    /// If 1, SRAM3 can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn with_sp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram3::SP_MASK
            } else {
                self.bits & !Sram3::SP_MASK
            }
        }
    }

    /// Set **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, SRAM3 can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn with_su(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram3::SU_MASK
            } else {
                self.bits & !Sram3::SU_MASK
            }
        }
    }

    /// Set **NSP** `[1:1]`
    ///
    /// If 1, SRAM3 can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn with_nsp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram3::NSP_MASK
            } else {
                self.bits & !Sram3::NSP_MASK
            }
        }
    }

    /// Set **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, SRAM3 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn with_nsu(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram3::NSU_MASK
            } else {
                self.bits & !Sram3::NSU_MASK
            }
        }
    }
}

impl Default for Sram3Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Sram3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Sram3Val {
        Sram3Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Sram3Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Sram3Val) -> Sram3Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SRAM4 Register
///
/// Control whether debugger, DMA, core 0 and core 1 can access SRAM4, and at what security/privilege levels they can do so.
/// Defaults to fully open access.
/// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
///
/// Address offset: 0x002c
/// Reset value: 0x000000ff
///
/// ## Fields
///
/// - **DBG** `[7]` (RW)
///
///   If 1, SRAM4 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **DMA** `[6]` (RW)
///
///   If 1, SRAM4 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE1** `[5]` (RW)
///
///   If 1, SRAM4 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE0** `[4]` (RW)
///
///   If 1, SRAM4 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **SP** `[3]` (RW)
///
///   If 1, SRAM4 can be accessed from a Secure, Privileged context.
///
/// - **SU** `[2]` (RW)
///
///   If 1, and SP is also set, SRAM4 can be accessed from a Secure, Unprivileged context.
///
/// - **NSP** `[1]` (RW)
///
///   If 1, SRAM4 can be accessed from a Non-secure, Privileged context.
///
/// - **NSU** `[0]` (RW)
///
///   If 1, and NSP is also set, SRAM4 can be accessed from a Non-secure, Unprivileged context.
///   This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
#[repr(transparent)]
pub struct Sram4 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Sram4 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sram4 {
    // ======== Field Constants ========

    /// **DBG**
    ///
    /// If 1, SRAM4 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const DBG_OFFSET: u32 = 7;
    pub const DBG_MASK: u32 = 0x00000080;
    pub const DBG: u32 = 1 << 7;

    /// **DMA**
    ///
    /// If 1, SRAM4 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const DMA_OFFSET: u32 = 6;
    pub const DMA_MASK: u32 = 0x00000040;
    pub const DMA: u32 = 1 << 6;

    /// **CORE1**
    ///
    /// If 1, SRAM4 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CORE1_OFFSET: u32 = 5;
    pub const CORE1_MASK: u32 = 0x00000020;
    pub const CORE1: u32 = 1 << 5;

    /// **CORE0**
    ///
    /// If 1, SRAM4 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CORE0_OFFSET: u32 = 4;
    pub const CORE0_MASK: u32 = 0x00000010;
    pub const CORE0: u32 = 1 << 4;

    /// **SP**
    ///
    /// If 1, SRAM4 can be accessed from a Secure, Privileged context.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SP_OFFSET: u32 = 3;
    pub const SP_MASK: u32 = 0x00000008;
    pub const SP: u32 = 1 << 3;

    /// **SU**
    ///
    /// If 1, and SP is also set, SRAM4 can be accessed from a Secure, Unprivileged context.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SU_OFFSET: u32 = 2;
    pub const SU_MASK: u32 = 0x00000004;
    pub const SU: u32 = 1 << 2;

    /// **NSP**
    ///
    /// If 1, SRAM4 can be accessed from a Non-secure, Privileged context.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const NSP_OFFSET: u32 = 1;
    pub const NSP_MASK: u32 = 0x00000002;
    pub const NSP: u32 = 1 << 1;

    /// **NSU**
    ///
    /// If 1, and NSP is also set, SRAM4 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const NSU_OFFSET: u32 = 0;
    pub const NSU_MASK: u32 = 0x00000001;
    pub const NSU: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DBG** `[7]` (RW)
    ///
    /// If 1, SRAM4 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dbg(&self) -> bool {
        (self.value.read_bits() & Self::DBG_MASK) != 0
    }

    /// Set DBG bit atomically (RW)
    #[inline(always)]
    pub fn set_dbg(&self) {
        self.set_bits(Self::DBG_MASK);
    }

    /// Clear DBG bit atomically (RW)
    #[inline(always)]
    pub fn clr_dbg(&self) {
        self.clr_bits(Self::DBG_MASK);
    }

    /// **DMA** `[6]` (RW)
    ///
    /// If 1, SRAM4 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dma(&self) -> bool {
        (self.value.read_bits() & Self::DMA_MASK) != 0
    }

    /// Set DMA bit atomically (RW)
    #[inline(always)]
    pub fn set_dma(&self) {
        self.set_bits(Self::DMA_MASK);
    }

    /// Clear DMA bit atomically (RW)
    #[inline(always)]
    pub fn clr_dma(&self) {
        self.clr_bits(Self::DMA_MASK);
    }

    /// **CORE1** `[5]` (RW)
    ///
    /// If 1, SRAM4 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core1(&self) -> bool {
        (self.value.read_bits() & Self::CORE1_MASK) != 0
    }

    /// Set CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn set_core1(&self) {
        self.set_bits(Self::CORE1_MASK);
    }

    /// Clear CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core1(&self) {
        self.clr_bits(Self::CORE1_MASK);
    }

    /// **CORE0** `[4]` (RW)
    ///
    /// If 1, SRAM4 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core0(&self) -> bool {
        (self.value.read_bits() & Self::CORE0_MASK) != 0
    }

    /// Set CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn set_core0(&self) {
        self.set_bits(Self::CORE0_MASK);
    }

    /// Clear CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core0(&self) {
        self.clr_bits(Self::CORE0_MASK);
    }

    /// **SP** `[3]` (RW)
    ///
    /// If 1, SRAM4 can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub fn sp(&self) -> bool {
        (self.value.read_bits() & Self::SP_MASK) != 0
    }

    /// Set SP bit atomically (RW)
    #[inline(always)]
    pub fn set_sp(&self) {
        self.set_bits(Self::SP_MASK);
    }

    /// Clear SP bit atomically (RW)
    #[inline(always)]
    pub fn clr_sp(&self) {
        self.clr_bits(Self::SP_MASK);
    }

    /// **SU** `[2]` (RW)
    ///
    /// If 1, and SP is also set, SRAM4 can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub fn su(&self) -> bool {
        (self.value.read_bits() & Self::SU_MASK) != 0
    }

    /// Set SU bit atomically (RW)
    #[inline(always)]
    pub fn set_su(&self) {
        self.set_bits(Self::SU_MASK);
    }

    /// Clear SU bit atomically (RW)
    #[inline(always)]
    pub fn clr_su(&self) {
        self.clr_bits(Self::SU_MASK);
    }

    /// **NSP** `[1]` (RW)
    ///
    /// If 1, SRAM4 can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub fn nsp(&self) -> bool {
        (self.value.read_bits() & Self::NSP_MASK) != 0
    }

    /// Set NSP bit atomically (RW)
    #[inline(always)]
    pub fn set_nsp(&self) {
        self.set_bits(Self::NSP_MASK);
    }

    /// Clear NSP bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsp(&self) {
        self.clr_bits(Self::NSP_MASK);
    }

    /// **NSU** `[0]` (RW)
    ///
    /// If 1, and NSP is also set, SRAM4 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub fn nsu(&self) -> bool {
        (self.value.read_bits() & Self::NSU_MASK) != 0
    }

    /// Set NSU bit atomically (RW)
    #[inline(always)]
    pub fn set_nsu(&self) {
        self.set_bits(Self::NSU_MASK);
    }

    /// Clear NSU bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsu(&self) {
        self.clr_bits(Self::NSU_MASK);
    }
}

/// SRAM4 value type for batch read/modify/write operations
///
/// Use `sram4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sram4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Sram4Val {
    bits: u32,
}

impl Sram4Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
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

    /// Read **DBG** `[7:7]`
    ///
    /// If 1, SRAM4 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        (self.bits & Sram4::DBG_MASK) != 0
    }

    /// Read **DMA** `[6:6]`
    ///
    /// If 1, SRAM4 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        (self.bits & Sram4::DMA_MASK) != 0
    }

    /// Read **CORE1** `[5:5]`
    ///
    /// If 1, SRAM4 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        (self.bits & Sram4::CORE1_MASK) != 0
    }

    /// Read **CORE0** `[4:4]`
    ///
    /// If 1, SRAM4 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        (self.bits & Sram4::CORE0_MASK) != 0
    }

    /// Read **SP** `[3:3]`
    ///
    /// If 1, SRAM4 can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        (self.bits & Sram4::SP_MASK) != 0
    }

    /// Read **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, SRAM4 can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn su(&self) -> bool {
        (self.bits & Sram4::SU_MASK) != 0
    }

    /// Read **NSP** `[1:1]`
    ///
    /// If 1, SRAM4 can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        (self.bits & Sram4::NSP_MASK) != 0
    }

    /// Read **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, SRAM4 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        (self.bits & Sram4::NSU_MASK) != 0
    }

    /// Set **DBG** `[7:7]`
    ///
    /// If 1, SRAM4 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dbg(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram4::DBG_MASK
            } else {
                self.bits & !Sram4::DBG_MASK
            }
        }
    }

    /// Set **DMA** `[6:6]`
    ///
    /// If 1, SRAM4 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dma(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram4::DMA_MASK
            } else {
                self.bits & !Sram4::DMA_MASK
            }
        }
    }

    /// Set **CORE1** `[5:5]`
    ///
    /// If 1, SRAM4 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram4::CORE1_MASK
            } else {
                self.bits & !Sram4::CORE1_MASK
            }
        }
    }

    /// Set **CORE0** `[4:4]`
    ///
    /// If 1, SRAM4 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram4::CORE0_MASK
            } else {
                self.bits & !Sram4::CORE0_MASK
            }
        }
    }

    /// Set **SP** `[3:3]`
    ///
    /// If 1, SRAM4 can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn with_sp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram4::SP_MASK
            } else {
                self.bits & !Sram4::SP_MASK
            }
        }
    }

    /// Set **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, SRAM4 can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn with_su(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram4::SU_MASK
            } else {
                self.bits & !Sram4::SU_MASK
            }
        }
    }

    /// Set **NSP** `[1:1]`
    ///
    /// If 1, SRAM4 can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn with_nsp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram4::NSP_MASK
            } else {
                self.bits & !Sram4::NSP_MASK
            }
        }
    }

    /// Set **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, SRAM4 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn with_nsu(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram4::NSU_MASK
            } else {
                self.bits & !Sram4::NSU_MASK
            }
        }
    }
}

impl Default for Sram4Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Sram4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Sram4Val {
        Sram4Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Sram4Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Sram4Val) -> Sram4Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SRAM5 Register
///
/// Control whether debugger, DMA, core 0 and core 1 can access SRAM5, and at what security/privilege levels they can do so.
/// Defaults to fully open access.
/// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
///
/// Address offset: 0x0030
/// Reset value: 0x000000ff
///
/// ## Fields
///
/// - **DBG** `[7]` (RW)
///
///   If 1, SRAM5 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **DMA** `[6]` (RW)
///
///   If 1, SRAM5 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE1** `[5]` (RW)
///
///   If 1, SRAM5 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE0** `[4]` (RW)
///
///   If 1, SRAM5 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **SP** `[3]` (RW)
///
///   If 1, SRAM5 can be accessed from a Secure, Privileged context.
///
/// - **SU** `[2]` (RW)
///
///   If 1, and SP is also set, SRAM5 can be accessed from a Secure, Unprivileged context.
///
/// - **NSP** `[1]` (RW)
///
///   If 1, SRAM5 can be accessed from a Non-secure, Privileged context.
///
/// - **NSU** `[0]` (RW)
///
///   If 1, and NSP is also set, SRAM5 can be accessed from a Non-secure, Unprivileged context.
///   This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
#[repr(transparent)]
pub struct Sram5 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Sram5 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sram5 {
    // ======== Field Constants ========

    /// **DBG**
    ///
    /// If 1, SRAM5 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const DBG_OFFSET: u32 = 7;
    pub const DBG_MASK: u32 = 0x00000080;
    pub const DBG: u32 = 1 << 7;

    /// **DMA**
    ///
    /// If 1, SRAM5 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const DMA_OFFSET: u32 = 6;
    pub const DMA_MASK: u32 = 0x00000040;
    pub const DMA: u32 = 1 << 6;

    /// **CORE1**
    ///
    /// If 1, SRAM5 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CORE1_OFFSET: u32 = 5;
    pub const CORE1_MASK: u32 = 0x00000020;
    pub const CORE1: u32 = 1 << 5;

    /// **CORE0**
    ///
    /// If 1, SRAM5 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CORE0_OFFSET: u32 = 4;
    pub const CORE0_MASK: u32 = 0x00000010;
    pub const CORE0: u32 = 1 << 4;

    /// **SP**
    ///
    /// If 1, SRAM5 can be accessed from a Secure, Privileged context.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SP_OFFSET: u32 = 3;
    pub const SP_MASK: u32 = 0x00000008;
    pub const SP: u32 = 1 << 3;

    /// **SU**
    ///
    /// If 1, and SP is also set, SRAM5 can be accessed from a Secure, Unprivileged context.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SU_OFFSET: u32 = 2;
    pub const SU_MASK: u32 = 0x00000004;
    pub const SU: u32 = 1 << 2;

    /// **NSP**
    ///
    /// If 1, SRAM5 can be accessed from a Non-secure, Privileged context.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const NSP_OFFSET: u32 = 1;
    pub const NSP_MASK: u32 = 0x00000002;
    pub const NSP: u32 = 1 << 1;

    /// **NSU**
    ///
    /// If 1, and NSP is also set, SRAM5 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const NSU_OFFSET: u32 = 0;
    pub const NSU_MASK: u32 = 0x00000001;
    pub const NSU: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DBG** `[7]` (RW)
    ///
    /// If 1, SRAM5 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dbg(&self) -> bool {
        (self.value.read_bits() & Self::DBG_MASK) != 0
    }

    /// Set DBG bit atomically (RW)
    #[inline(always)]
    pub fn set_dbg(&self) {
        self.set_bits(Self::DBG_MASK);
    }

    /// Clear DBG bit atomically (RW)
    #[inline(always)]
    pub fn clr_dbg(&self) {
        self.clr_bits(Self::DBG_MASK);
    }

    /// **DMA** `[6]` (RW)
    ///
    /// If 1, SRAM5 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dma(&self) -> bool {
        (self.value.read_bits() & Self::DMA_MASK) != 0
    }

    /// Set DMA bit atomically (RW)
    #[inline(always)]
    pub fn set_dma(&self) {
        self.set_bits(Self::DMA_MASK);
    }

    /// Clear DMA bit atomically (RW)
    #[inline(always)]
    pub fn clr_dma(&self) {
        self.clr_bits(Self::DMA_MASK);
    }

    /// **CORE1** `[5]` (RW)
    ///
    /// If 1, SRAM5 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core1(&self) -> bool {
        (self.value.read_bits() & Self::CORE1_MASK) != 0
    }

    /// Set CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn set_core1(&self) {
        self.set_bits(Self::CORE1_MASK);
    }

    /// Clear CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core1(&self) {
        self.clr_bits(Self::CORE1_MASK);
    }

    /// **CORE0** `[4]` (RW)
    ///
    /// If 1, SRAM5 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core0(&self) -> bool {
        (self.value.read_bits() & Self::CORE0_MASK) != 0
    }

    /// Set CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn set_core0(&self) {
        self.set_bits(Self::CORE0_MASK);
    }

    /// Clear CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core0(&self) {
        self.clr_bits(Self::CORE0_MASK);
    }

    /// **SP** `[3]` (RW)
    ///
    /// If 1, SRAM5 can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub fn sp(&self) -> bool {
        (self.value.read_bits() & Self::SP_MASK) != 0
    }

    /// Set SP bit atomically (RW)
    #[inline(always)]
    pub fn set_sp(&self) {
        self.set_bits(Self::SP_MASK);
    }

    /// Clear SP bit atomically (RW)
    #[inline(always)]
    pub fn clr_sp(&self) {
        self.clr_bits(Self::SP_MASK);
    }

    /// **SU** `[2]` (RW)
    ///
    /// If 1, and SP is also set, SRAM5 can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub fn su(&self) -> bool {
        (self.value.read_bits() & Self::SU_MASK) != 0
    }

    /// Set SU bit atomically (RW)
    #[inline(always)]
    pub fn set_su(&self) {
        self.set_bits(Self::SU_MASK);
    }

    /// Clear SU bit atomically (RW)
    #[inline(always)]
    pub fn clr_su(&self) {
        self.clr_bits(Self::SU_MASK);
    }

    /// **NSP** `[1]` (RW)
    ///
    /// If 1, SRAM5 can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub fn nsp(&self) -> bool {
        (self.value.read_bits() & Self::NSP_MASK) != 0
    }

    /// Set NSP bit atomically (RW)
    #[inline(always)]
    pub fn set_nsp(&self) {
        self.set_bits(Self::NSP_MASK);
    }

    /// Clear NSP bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsp(&self) {
        self.clr_bits(Self::NSP_MASK);
    }

    /// **NSU** `[0]` (RW)
    ///
    /// If 1, and NSP is also set, SRAM5 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub fn nsu(&self) -> bool {
        (self.value.read_bits() & Self::NSU_MASK) != 0
    }

    /// Set NSU bit atomically (RW)
    #[inline(always)]
    pub fn set_nsu(&self) {
        self.set_bits(Self::NSU_MASK);
    }

    /// Clear NSU bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsu(&self) {
        self.clr_bits(Self::NSU_MASK);
    }
}

/// SRAM5 value type for batch read/modify/write operations
///
/// Use `sram5.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sram5.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Sram5Val {
    bits: u32,
}

impl Sram5Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
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

    /// Read **DBG** `[7:7]`
    ///
    /// If 1, SRAM5 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        (self.bits & Sram5::DBG_MASK) != 0
    }

    /// Read **DMA** `[6:6]`
    ///
    /// If 1, SRAM5 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        (self.bits & Sram5::DMA_MASK) != 0
    }

    /// Read **CORE1** `[5:5]`
    ///
    /// If 1, SRAM5 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        (self.bits & Sram5::CORE1_MASK) != 0
    }

    /// Read **CORE0** `[4:4]`
    ///
    /// If 1, SRAM5 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        (self.bits & Sram5::CORE0_MASK) != 0
    }

    /// Read **SP** `[3:3]`
    ///
    /// If 1, SRAM5 can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        (self.bits & Sram5::SP_MASK) != 0
    }

    /// Read **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, SRAM5 can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn su(&self) -> bool {
        (self.bits & Sram5::SU_MASK) != 0
    }

    /// Read **NSP** `[1:1]`
    ///
    /// If 1, SRAM5 can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        (self.bits & Sram5::NSP_MASK) != 0
    }

    /// Read **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, SRAM5 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        (self.bits & Sram5::NSU_MASK) != 0
    }

    /// Set **DBG** `[7:7]`
    ///
    /// If 1, SRAM5 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dbg(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram5::DBG_MASK
            } else {
                self.bits & !Sram5::DBG_MASK
            }
        }
    }

    /// Set **DMA** `[6:6]`
    ///
    /// If 1, SRAM5 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dma(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram5::DMA_MASK
            } else {
                self.bits & !Sram5::DMA_MASK
            }
        }
    }

    /// Set **CORE1** `[5:5]`
    ///
    /// If 1, SRAM5 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram5::CORE1_MASK
            } else {
                self.bits & !Sram5::CORE1_MASK
            }
        }
    }

    /// Set **CORE0** `[4:4]`
    ///
    /// If 1, SRAM5 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram5::CORE0_MASK
            } else {
                self.bits & !Sram5::CORE0_MASK
            }
        }
    }

    /// Set **SP** `[3:3]`
    ///
    /// If 1, SRAM5 can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn with_sp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram5::SP_MASK
            } else {
                self.bits & !Sram5::SP_MASK
            }
        }
    }

    /// Set **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, SRAM5 can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn with_su(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram5::SU_MASK
            } else {
                self.bits & !Sram5::SU_MASK
            }
        }
    }

    /// Set **NSP** `[1:1]`
    ///
    /// If 1, SRAM5 can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn with_nsp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram5::NSP_MASK
            } else {
                self.bits & !Sram5::NSP_MASK
            }
        }
    }

    /// Set **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, SRAM5 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn with_nsu(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram5::NSU_MASK
            } else {
                self.bits & !Sram5::NSU_MASK
            }
        }
    }
}

impl Default for Sram5Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Sram5 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Sram5Val {
        Sram5Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Sram5Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Sram5Val) -> Sram5Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SRAM6 Register
///
/// Control whether debugger, DMA, core 0 and core 1 can access SRAM6, and at what security/privilege levels they can do so.
/// Defaults to fully open access.
/// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
///
/// Address offset: 0x0034
/// Reset value: 0x000000ff
///
/// ## Fields
///
/// - **DBG** `[7]` (RW)
///
///   If 1, SRAM6 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **DMA** `[6]` (RW)
///
///   If 1, SRAM6 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE1** `[5]` (RW)
///
///   If 1, SRAM6 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE0** `[4]` (RW)
///
///   If 1, SRAM6 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **SP** `[3]` (RW)
///
///   If 1, SRAM6 can be accessed from a Secure, Privileged context.
///
/// - **SU** `[2]` (RW)
///
///   If 1, and SP is also set, SRAM6 can be accessed from a Secure, Unprivileged context.
///
/// - **NSP** `[1]` (RW)
///
///   If 1, SRAM6 can be accessed from a Non-secure, Privileged context.
///
/// - **NSU** `[0]` (RW)
///
///   If 1, and NSP is also set, SRAM6 can be accessed from a Non-secure, Unprivileged context.
///   This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
#[repr(transparent)]
pub struct Sram6 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Sram6 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sram6 {
    // ======== Field Constants ========

    /// **DBG**
    ///
    /// If 1, SRAM6 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const DBG_OFFSET: u32 = 7;
    pub const DBG_MASK: u32 = 0x00000080;
    pub const DBG: u32 = 1 << 7;

    /// **DMA**
    ///
    /// If 1, SRAM6 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const DMA_OFFSET: u32 = 6;
    pub const DMA_MASK: u32 = 0x00000040;
    pub const DMA: u32 = 1 << 6;

    /// **CORE1**
    ///
    /// If 1, SRAM6 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CORE1_OFFSET: u32 = 5;
    pub const CORE1_MASK: u32 = 0x00000020;
    pub const CORE1: u32 = 1 << 5;

    /// **CORE0**
    ///
    /// If 1, SRAM6 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CORE0_OFFSET: u32 = 4;
    pub const CORE0_MASK: u32 = 0x00000010;
    pub const CORE0: u32 = 1 << 4;

    /// **SP**
    ///
    /// If 1, SRAM6 can be accessed from a Secure, Privileged context.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SP_OFFSET: u32 = 3;
    pub const SP_MASK: u32 = 0x00000008;
    pub const SP: u32 = 1 << 3;

    /// **SU**
    ///
    /// If 1, and SP is also set, SRAM6 can be accessed from a Secure, Unprivileged context.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SU_OFFSET: u32 = 2;
    pub const SU_MASK: u32 = 0x00000004;
    pub const SU: u32 = 1 << 2;

    /// **NSP**
    ///
    /// If 1, SRAM6 can be accessed from a Non-secure, Privileged context.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const NSP_OFFSET: u32 = 1;
    pub const NSP_MASK: u32 = 0x00000002;
    pub const NSP: u32 = 1 << 1;

    /// **NSU**
    ///
    /// If 1, and NSP is also set, SRAM6 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const NSU_OFFSET: u32 = 0;
    pub const NSU_MASK: u32 = 0x00000001;
    pub const NSU: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DBG** `[7]` (RW)
    ///
    /// If 1, SRAM6 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dbg(&self) -> bool {
        (self.value.read_bits() & Self::DBG_MASK) != 0
    }

    /// Set DBG bit atomically (RW)
    #[inline(always)]
    pub fn set_dbg(&self) {
        self.set_bits(Self::DBG_MASK);
    }

    /// Clear DBG bit atomically (RW)
    #[inline(always)]
    pub fn clr_dbg(&self) {
        self.clr_bits(Self::DBG_MASK);
    }

    /// **DMA** `[6]` (RW)
    ///
    /// If 1, SRAM6 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dma(&self) -> bool {
        (self.value.read_bits() & Self::DMA_MASK) != 0
    }

    /// Set DMA bit atomically (RW)
    #[inline(always)]
    pub fn set_dma(&self) {
        self.set_bits(Self::DMA_MASK);
    }

    /// Clear DMA bit atomically (RW)
    #[inline(always)]
    pub fn clr_dma(&self) {
        self.clr_bits(Self::DMA_MASK);
    }

    /// **CORE1** `[5]` (RW)
    ///
    /// If 1, SRAM6 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core1(&self) -> bool {
        (self.value.read_bits() & Self::CORE1_MASK) != 0
    }

    /// Set CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn set_core1(&self) {
        self.set_bits(Self::CORE1_MASK);
    }

    /// Clear CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core1(&self) {
        self.clr_bits(Self::CORE1_MASK);
    }

    /// **CORE0** `[4]` (RW)
    ///
    /// If 1, SRAM6 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core0(&self) -> bool {
        (self.value.read_bits() & Self::CORE0_MASK) != 0
    }

    /// Set CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn set_core0(&self) {
        self.set_bits(Self::CORE0_MASK);
    }

    /// Clear CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core0(&self) {
        self.clr_bits(Self::CORE0_MASK);
    }

    /// **SP** `[3]` (RW)
    ///
    /// If 1, SRAM6 can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub fn sp(&self) -> bool {
        (self.value.read_bits() & Self::SP_MASK) != 0
    }

    /// Set SP bit atomically (RW)
    #[inline(always)]
    pub fn set_sp(&self) {
        self.set_bits(Self::SP_MASK);
    }

    /// Clear SP bit atomically (RW)
    #[inline(always)]
    pub fn clr_sp(&self) {
        self.clr_bits(Self::SP_MASK);
    }

    /// **SU** `[2]` (RW)
    ///
    /// If 1, and SP is also set, SRAM6 can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub fn su(&self) -> bool {
        (self.value.read_bits() & Self::SU_MASK) != 0
    }

    /// Set SU bit atomically (RW)
    #[inline(always)]
    pub fn set_su(&self) {
        self.set_bits(Self::SU_MASK);
    }

    /// Clear SU bit atomically (RW)
    #[inline(always)]
    pub fn clr_su(&self) {
        self.clr_bits(Self::SU_MASK);
    }

    /// **NSP** `[1]` (RW)
    ///
    /// If 1, SRAM6 can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub fn nsp(&self) -> bool {
        (self.value.read_bits() & Self::NSP_MASK) != 0
    }

    /// Set NSP bit atomically (RW)
    #[inline(always)]
    pub fn set_nsp(&self) {
        self.set_bits(Self::NSP_MASK);
    }

    /// Clear NSP bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsp(&self) {
        self.clr_bits(Self::NSP_MASK);
    }

    /// **NSU** `[0]` (RW)
    ///
    /// If 1, and NSP is also set, SRAM6 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub fn nsu(&self) -> bool {
        (self.value.read_bits() & Self::NSU_MASK) != 0
    }

    /// Set NSU bit atomically (RW)
    #[inline(always)]
    pub fn set_nsu(&self) {
        self.set_bits(Self::NSU_MASK);
    }

    /// Clear NSU bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsu(&self) {
        self.clr_bits(Self::NSU_MASK);
    }
}

/// SRAM6 value type for batch read/modify/write operations
///
/// Use `sram6.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sram6.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Sram6Val {
    bits: u32,
}

impl Sram6Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
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

    /// Read **DBG** `[7:7]`
    ///
    /// If 1, SRAM6 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        (self.bits & Sram6::DBG_MASK) != 0
    }

    /// Read **DMA** `[6:6]`
    ///
    /// If 1, SRAM6 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        (self.bits & Sram6::DMA_MASK) != 0
    }

    /// Read **CORE1** `[5:5]`
    ///
    /// If 1, SRAM6 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        (self.bits & Sram6::CORE1_MASK) != 0
    }

    /// Read **CORE0** `[4:4]`
    ///
    /// If 1, SRAM6 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        (self.bits & Sram6::CORE0_MASK) != 0
    }

    /// Read **SP** `[3:3]`
    ///
    /// If 1, SRAM6 can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        (self.bits & Sram6::SP_MASK) != 0
    }

    /// Read **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, SRAM6 can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn su(&self) -> bool {
        (self.bits & Sram6::SU_MASK) != 0
    }

    /// Read **NSP** `[1:1]`
    ///
    /// If 1, SRAM6 can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        (self.bits & Sram6::NSP_MASK) != 0
    }

    /// Read **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, SRAM6 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        (self.bits & Sram6::NSU_MASK) != 0
    }

    /// Set **DBG** `[7:7]`
    ///
    /// If 1, SRAM6 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dbg(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram6::DBG_MASK
            } else {
                self.bits & !Sram6::DBG_MASK
            }
        }
    }

    /// Set **DMA** `[6:6]`
    ///
    /// If 1, SRAM6 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dma(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram6::DMA_MASK
            } else {
                self.bits & !Sram6::DMA_MASK
            }
        }
    }

    /// Set **CORE1** `[5:5]`
    ///
    /// If 1, SRAM6 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram6::CORE1_MASK
            } else {
                self.bits & !Sram6::CORE1_MASK
            }
        }
    }

    /// Set **CORE0** `[4:4]`
    ///
    /// If 1, SRAM6 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram6::CORE0_MASK
            } else {
                self.bits & !Sram6::CORE0_MASK
            }
        }
    }

    /// Set **SP** `[3:3]`
    ///
    /// If 1, SRAM6 can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn with_sp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram6::SP_MASK
            } else {
                self.bits & !Sram6::SP_MASK
            }
        }
    }

    /// Set **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, SRAM6 can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn with_su(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram6::SU_MASK
            } else {
                self.bits & !Sram6::SU_MASK
            }
        }
    }

    /// Set **NSP** `[1:1]`
    ///
    /// If 1, SRAM6 can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn with_nsp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram6::NSP_MASK
            } else {
                self.bits & !Sram6::NSP_MASK
            }
        }
    }

    /// Set **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, SRAM6 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn with_nsu(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram6::NSU_MASK
            } else {
                self.bits & !Sram6::NSU_MASK
            }
        }
    }
}

impl Default for Sram6Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Sram6 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Sram6Val {
        Sram6Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Sram6Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Sram6Val) -> Sram6Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SRAM7 Register
///
/// Control whether debugger, DMA, core 0 and core 1 can access SRAM7, and at what security/privilege levels they can do so.
/// Defaults to fully open access.
/// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
///
/// Address offset: 0x0038
/// Reset value: 0x000000ff
///
/// ## Fields
///
/// - **DBG** `[7]` (RW)
///
///   If 1, SRAM7 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **DMA** `[6]` (RW)
///
///   If 1, SRAM7 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE1** `[5]` (RW)
///
///   If 1, SRAM7 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE0** `[4]` (RW)
///
///   If 1, SRAM7 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **SP** `[3]` (RW)
///
///   If 1, SRAM7 can be accessed from a Secure, Privileged context.
///
/// - **SU** `[2]` (RW)
///
///   If 1, and SP is also set, SRAM7 can be accessed from a Secure, Unprivileged context.
///
/// - **NSP** `[1]` (RW)
///
///   If 1, SRAM7 can be accessed from a Non-secure, Privileged context.
///
/// - **NSU** `[0]` (RW)
///
///   If 1, and NSP is also set, SRAM7 can be accessed from a Non-secure, Unprivileged context.
///   This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
#[repr(transparent)]
pub struct Sram7 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Sram7 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sram7 {
    // ======== Field Constants ========

    /// **DBG**
    ///
    /// If 1, SRAM7 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const DBG_OFFSET: u32 = 7;
    pub const DBG_MASK: u32 = 0x00000080;
    pub const DBG: u32 = 1 << 7;

    /// **DMA**
    ///
    /// If 1, SRAM7 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const DMA_OFFSET: u32 = 6;
    pub const DMA_MASK: u32 = 0x00000040;
    pub const DMA: u32 = 1 << 6;

    /// **CORE1**
    ///
    /// If 1, SRAM7 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CORE1_OFFSET: u32 = 5;
    pub const CORE1_MASK: u32 = 0x00000020;
    pub const CORE1: u32 = 1 << 5;

    /// **CORE0**
    ///
    /// If 1, SRAM7 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CORE0_OFFSET: u32 = 4;
    pub const CORE0_MASK: u32 = 0x00000010;
    pub const CORE0: u32 = 1 << 4;

    /// **SP**
    ///
    /// If 1, SRAM7 can be accessed from a Secure, Privileged context.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SP_OFFSET: u32 = 3;
    pub const SP_MASK: u32 = 0x00000008;
    pub const SP: u32 = 1 << 3;

    /// **SU**
    ///
    /// If 1, and SP is also set, SRAM7 can be accessed from a Secure, Unprivileged context.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SU_OFFSET: u32 = 2;
    pub const SU_MASK: u32 = 0x00000004;
    pub const SU: u32 = 1 << 2;

    /// **NSP**
    ///
    /// If 1, SRAM7 can be accessed from a Non-secure, Privileged context.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const NSP_OFFSET: u32 = 1;
    pub const NSP_MASK: u32 = 0x00000002;
    pub const NSP: u32 = 1 << 1;

    /// **NSU**
    ///
    /// If 1, and NSP is also set, SRAM7 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const NSU_OFFSET: u32 = 0;
    pub const NSU_MASK: u32 = 0x00000001;
    pub const NSU: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DBG** `[7]` (RW)
    ///
    /// If 1, SRAM7 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dbg(&self) -> bool {
        (self.value.read_bits() & Self::DBG_MASK) != 0
    }

    /// Set DBG bit atomically (RW)
    #[inline(always)]
    pub fn set_dbg(&self) {
        self.set_bits(Self::DBG_MASK);
    }

    /// Clear DBG bit atomically (RW)
    #[inline(always)]
    pub fn clr_dbg(&self) {
        self.clr_bits(Self::DBG_MASK);
    }

    /// **DMA** `[6]` (RW)
    ///
    /// If 1, SRAM7 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dma(&self) -> bool {
        (self.value.read_bits() & Self::DMA_MASK) != 0
    }

    /// Set DMA bit atomically (RW)
    #[inline(always)]
    pub fn set_dma(&self) {
        self.set_bits(Self::DMA_MASK);
    }

    /// Clear DMA bit atomically (RW)
    #[inline(always)]
    pub fn clr_dma(&self) {
        self.clr_bits(Self::DMA_MASK);
    }

    /// **CORE1** `[5]` (RW)
    ///
    /// If 1, SRAM7 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core1(&self) -> bool {
        (self.value.read_bits() & Self::CORE1_MASK) != 0
    }

    /// Set CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn set_core1(&self) {
        self.set_bits(Self::CORE1_MASK);
    }

    /// Clear CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core1(&self) {
        self.clr_bits(Self::CORE1_MASK);
    }

    /// **CORE0** `[4]` (RW)
    ///
    /// If 1, SRAM7 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core0(&self) -> bool {
        (self.value.read_bits() & Self::CORE0_MASK) != 0
    }

    /// Set CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn set_core0(&self) {
        self.set_bits(Self::CORE0_MASK);
    }

    /// Clear CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core0(&self) {
        self.clr_bits(Self::CORE0_MASK);
    }

    /// **SP** `[3]` (RW)
    ///
    /// If 1, SRAM7 can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub fn sp(&self) -> bool {
        (self.value.read_bits() & Self::SP_MASK) != 0
    }

    /// Set SP bit atomically (RW)
    #[inline(always)]
    pub fn set_sp(&self) {
        self.set_bits(Self::SP_MASK);
    }

    /// Clear SP bit atomically (RW)
    #[inline(always)]
    pub fn clr_sp(&self) {
        self.clr_bits(Self::SP_MASK);
    }

    /// **SU** `[2]` (RW)
    ///
    /// If 1, and SP is also set, SRAM7 can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub fn su(&self) -> bool {
        (self.value.read_bits() & Self::SU_MASK) != 0
    }

    /// Set SU bit atomically (RW)
    #[inline(always)]
    pub fn set_su(&self) {
        self.set_bits(Self::SU_MASK);
    }

    /// Clear SU bit atomically (RW)
    #[inline(always)]
    pub fn clr_su(&self) {
        self.clr_bits(Self::SU_MASK);
    }

    /// **NSP** `[1]` (RW)
    ///
    /// If 1, SRAM7 can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub fn nsp(&self) -> bool {
        (self.value.read_bits() & Self::NSP_MASK) != 0
    }

    /// Set NSP bit atomically (RW)
    #[inline(always)]
    pub fn set_nsp(&self) {
        self.set_bits(Self::NSP_MASK);
    }

    /// Clear NSP bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsp(&self) {
        self.clr_bits(Self::NSP_MASK);
    }

    /// **NSU** `[0]` (RW)
    ///
    /// If 1, and NSP is also set, SRAM7 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub fn nsu(&self) -> bool {
        (self.value.read_bits() & Self::NSU_MASK) != 0
    }

    /// Set NSU bit atomically (RW)
    #[inline(always)]
    pub fn set_nsu(&self) {
        self.set_bits(Self::NSU_MASK);
    }

    /// Clear NSU bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsu(&self) {
        self.clr_bits(Self::NSU_MASK);
    }
}

/// SRAM7 value type for batch read/modify/write operations
///
/// Use `sram7.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sram7.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Sram7Val {
    bits: u32,
}

impl Sram7Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
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

    /// Read **DBG** `[7:7]`
    ///
    /// If 1, SRAM7 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        (self.bits & Sram7::DBG_MASK) != 0
    }

    /// Read **DMA** `[6:6]`
    ///
    /// If 1, SRAM7 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        (self.bits & Sram7::DMA_MASK) != 0
    }

    /// Read **CORE1** `[5:5]`
    ///
    /// If 1, SRAM7 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        (self.bits & Sram7::CORE1_MASK) != 0
    }

    /// Read **CORE0** `[4:4]`
    ///
    /// If 1, SRAM7 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        (self.bits & Sram7::CORE0_MASK) != 0
    }

    /// Read **SP** `[3:3]`
    ///
    /// If 1, SRAM7 can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        (self.bits & Sram7::SP_MASK) != 0
    }

    /// Read **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, SRAM7 can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn su(&self) -> bool {
        (self.bits & Sram7::SU_MASK) != 0
    }

    /// Read **NSP** `[1:1]`
    ///
    /// If 1, SRAM7 can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        (self.bits & Sram7::NSP_MASK) != 0
    }

    /// Read **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, SRAM7 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        (self.bits & Sram7::NSU_MASK) != 0
    }

    /// Set **DBG** `[7:7]`
    ///
    /// If 1, SRAM7 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dbg(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram7::DBG_MASK
            } else {
                self.bits & !Sram7::DBG_MASK
            }
        }
    }

    /// Set **DMA** `[6:6]`
    ///
    /// If 1, SRAM7 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dma(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram7::DMA_MASK
            } else {
                self.bits & !Sram7::DMA_MASK
            }
        }
    }

    /// Set **CORE1** `[5:5]`
    ///
    /// If 1, SRAM7 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram7::CORE1_MASK
            } else {
                self.bits & !Sram7::CORE1_MASK
            }
        }
    }

    /// Set **CORE0** `[4:4]`
    ///
    /// If 1, SRAM7 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram7::CORE0_MASK
            } else {
                self.bits & !Sram7::CORE0_MASK
            }
        }
    }

    /// Set **SP** `[3:3]`
    ///
    /// If 1, SRAM7 can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn with_sp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram7::SP_MASK
            } else {
                self.bits & !Sram7::SP_MASK
            }
        }
    }

    /// Set **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, SRAM7 can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn with_su(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram7::SU_MASK
            } else {
                self.bits & !Sram7::SU_MASK
            }
        }
    }

    /// Set **NSP** `[1:1]`
    ///
    /// If 1, SRAM7 can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn with_nsp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram7::NSP_MASK
            } else {
                self.bits & !Sram7::NSP_MASK
            }
        }
    }

    /// Set **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, SRAM7 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn with_nsu(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram7::NSU_MASK
            } else {
                self.bits & !Sram7::NSU_MASK
            }
        }
    }
}

impl Default for Sram7Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Sram7 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Sram7Val {
        Sram7Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Sram7Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Sram7Val) -> Sram7Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SRAM8 Register
///
/// Control whether debugger, DMA, core 0 and core 1 can access SRAM8, and at what security/privilege levels they can do so.
/// Defaults to fully open access.
/// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
///
/// Address offset: 0x003c
/// Reset value: 0x000000ff
///
/// ## Fields
///
/// - **DBG** `[7]` (RW)
///
///   If 1, SRAM8 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **DMA** `[6]` (RW)
///
///   If 1, SRAM8 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE1** `[5]` (RW)
///
///   If 1, SRAM8 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE0** `[4]` (RW)
///
///   If 1, SRAM8 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **SP** `[3]` (RW)
///
///   If 1, SRAM8 can be accessed from a Secure, Privileged context.
///
/// - **SU** `[2]` (RW)
///
///   If 1, and SP is also set, SRAM8 can be accessed from a Secure, Unprivileged context.
///
/// - **NSP** `[1]` (RW)
///
///   If 1, SRAM8 can be accessed from a Non-secure, Privileged context.
///
/// - **NSU** `[0]` (RW)
///
///   If 1, and NSP is also set, SRAM8 can be accessed from a Non-secure, Unprivileged context.
///   This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
#[repr(transparent)]
pub struct Sram8 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Sram8 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sram8 {
    // ======== Field Constants ========

    /// **DBG**
    ///
    /// If 1, SRAM8 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const DBG_OFFSET: u32 = 7;
    pub const DBG_MASK: u32 = 0x00000080;
    pub const DBG: u32 = 1 << 7;

    /// **DMA**
    ///
    /// If 1, SRAM8 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const DMA_OFFSET: u32 = 6;
    pub const DMA_MASK: u32 = 0x00000040;
    pub const DMA: u32 = 1 << 6;

    /// **CORE1**
    ///
    /// If 1, SRAM8 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CORE1_OFFSET: u32 = 5;
    pub const CORE1_MASK: u32 = 0x00000020;
    pub const CORE1: u32 = 1 << 5;

    /// **CORE0**
    ///
    /// If 1, SRAM8 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CORE0_OFFSET: u32 = 4;
    pub const CORE0_MASK: u32 = 0x00000010;
    pub const CORE0: u32 = 1 << 4;

    /// **SP**
    ///
    /// If 1, SRAM8 can be accessed from a Secure, Privileged context.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SP_OFFSET: u32 = 3;
    pub const SP_MASK: u32 = 0x00000008;
    pub const SP: u32 = 1 << 3;

    /// **SU**
    ///
    /// If 1, and SP is also set, SRAM8 can be accessed from a Secure, Unprivileged context.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SU_OFFSET: u32 = 2;
    pub const SU_MASK: u32 = 0x00000004;
    pub const SU: u32 = 1 << 2;

    /// **NSP**
    ///
    /// If 1, SRAM8 can be accessed from a Non-secure, Privileged context.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const NSP_OFFSET: u32 = 1;
    pub const NSP_MASK: u32 = 0x00000002;
    pub const NSP: u32 = 1 << 1;

    /// **NSU**
    ///
    /// If 1, and NSP is also set, SRAM8 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const NSU_OFFSET: u32 = 0;
    pub const NSU_MASK: u32 = 0x00000001;
    pub const NSU: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DBG** `[7]` (RW)
    ///
    /// If 1, SRAM8 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dbg(&self) -> bool {
        (self.value.read_bits() & Self::DBG_MASK) != 0
    }

    /// Set DBG bit atomically (RW)
    #[inline(always)]
    pub fn set_dbg(&self) {
        self.set_bits(Self::DBG_MASK);
    }

    /// Clear DBG bit atomically (RW)
    #[inline(always)]
    pub fn clr_dbg(&self) {
        self.clr_bits(Self::DBG_MASK);
    }

    /// **DMA** `[6]` (RW)
    ///
    /// If 1, SRAM8 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dma(&self) -> bool {
        (self.value.read_bits() & Self::DMA_MASK) != 0
    }

    /// Set DMA bit atomically (RW)
    #[inline(always)]
    pub fn set_dma(&self) {
        self.set_bits(Self::DMA_MASK);
    }

    /// Clear DMA bit atomically (RW)
    #[inline(always)]
    pub fn clr_dma(&self) {
        self.clr_bits(Self::DMA_MASK);
    }

    /// **CORE1** `[5]` (RW)
    ///
    /// If 1, SRAM8 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core1(&self) -> bool {
        (self.value.read_bits() & Self::CORE1_MASK) != 0
    }

    /// Set CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn set_core1(&self) {
        self.set_bits(Self::CORE1_MASK);
    }

    /// Clear CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core1(&self) {
        self.clr_bits(Self::CORE1_MASK);
    }

    /// **CORE0** `[4]` (RW)
    ///
    /// If 1, SRAM8 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core0(&self) -> bool {
        (self.value.read_bits() & Self::CORE0_MASK) != 0
    }

    /// Set CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn set_core0(&self) {
        self.set_bits(Self::CORE0_MASK);
    }

    /// Clear CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core0(&self) {
        self.clr_bits(Self::CORE0_MASK);
    }

    /// **SP** `[3]` (RW)
    ///
    /// If 1, SRAM8 can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub fn sp(&self) -> bool {
        (self.value.read_bits() & Self::SP_MASK) != 0
    }

    /// Set SP bit atomically (RW)
    #[inline(always)]
    pub fn set_sp(&self) {
        self.set_bits(Self::SP_MASK);
    }

    /// Clear SP bit atomically (RW)
    #[inline(always)]
    pub fn clr_sp(&self) {
        self.clr_bits(Self::SP_MASK);
    }

    /// **SU** `[2]` (RW)
    ///
    /// If 1, and SP is also set, SRAM8 can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub fn su(&self) -> bool {
        (self.value.read_bits() & Self::SU_MASK) != 0
    }

    /// Set SU bit atomically (RW)
    #[inline(always)]
    pub fn set_su(&self) {
        self.set_bits(Self::SU_MASK);
    }

    /// Clear SU bit atomically (RW)
    #[inline(always)]
    pub fn clr_su(&self) {
        self.clr_bits(Self::SU_MASK);
    }

    /// **NSP** `[1]` (RW)
    ///
    /// If 1, SRAM8 can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub fn nsp(&self) -> bool {
        (self.value.read_bits() & Self::NSP_MASK) != 0
    }

    /// Set NSP bit atomically (RW)
    #[inline(always)]
    pub fn set_nsp(&self) {
        self.set_bits(Self::NSP_MASK);
    }

    /// Clear NSP bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsp(&self) {
        self.clr_bits(Self::NSP_MASK);
    }

    /// **NSU** `[0]` (RW)
    ///
    /// If 1, and NSP is also set, SRAM8 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub fn nsu(&self) -> bool {
        (self.value.read_bits() & Self::NSU_MASK) != 0
    }

    /// Set NSU bit atomically (RW)
    #[inline(always)]
    pub fn set_nsu(&self) {
        self.set_bits(Self::NSU_MASK);
    }

    /// Clear NSU bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsu(&self) {
        self.clr_bits(Self::NSU_MASK);
    }
}

/// SRAM8 value type for batch read/modify/write operations
///
/// Use `sram8.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sram8.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Sram8Val {
    bits: u32,
}

impl Sram8Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
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

    /// Read **DBG** `[7:7]`
    ///
    /// If 1, SRAM8 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        (self.bits & Sram8::DBG_MASK) != 0
    }

    /// Read **DMA** `[6:6]`
    ///
    /// If 1, SRAM8 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        (self.bits & Sram8::DMA_MASK) != 0
    }

    /// Read **CORE1** `[5:5]`
    ///
    /// If 1, SRAM8 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        (self.bits & Sram8::CORE1_MASK) != 0
    }

    /// Read **CORE0** `[4:4]`
    ///
    /// If 1, SRAM8 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        (self.bits & Sram8::CORE0_MASK) != 0
    }

    /// Read **SP** `[3:3]`
    ///
    /// If 1, SRAM8 can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        (self.bits & Sram8::SP_MASK) != 0
    }

    /// Read **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, SRAM8 can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn su(&self) -> bool {
        (self.bits & Sram8::SU_MASK) != 0
    }

    /// Read **NSP** `[1:1]`
    ///
    /// If 1, SRAM8 can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        (self.bits & Sram8::NSP_MASK) != 0
    }

    /// Read **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, SRAM8 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        (self.bits & Sram8::NSU_MASK) != 0
    }

    /// Set **DBG** `[7:7]`
    ///
    /// If 1, SRAM8 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dbg(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram8::DBG_MASK
            } else {
                self.bits & !Sram8::DBG_MASK
            }
        }
    }

    /// Set **DMA** `[6:6]`
    ///
    /// If 1, SRAM8 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dma(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram8::DMA_MASK
            } else {
                self.bits & !Sram8::DMA_MASK
            }
        }
    }

    /// Set **CORE1** `[5:5]`
    ///
    /// If 1, SRAM8 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram8::CORE1_MASK
            } else {
                self.bits & !Sram8::CORE1_MASK
            }
        }
    }

    /// Set **CORE0** `[4:4]`
    ///
    /// If 1, SRAM8 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram8::CORE0_MASK
            } else {
                self.bits & !Sram8::CORE0_MASK
            }
        }
    }

    /// Set **SP** `[3:3]`
    ///
    /// If 1, SRAM8 can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn with_sp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram8::SP_MASK
            } else {
                self.bits & !Sram8::SP_MASK
            }
        }
    }

    /// Set **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, SRAM8 can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn with_su(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram8::SU_MASK
            } else {
                self.bits & !Sram8::SU_MASK
            }
        }
    }

    /// Set **NSP** `[1:1]`
    ///
    /// If 1, SRAM8 can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn with_nsp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram8::NSP_MASK
            } else {
                self.bits & !Sram8::NSP_MASK
            }
        }
    }

    /// Set **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, SRAM8 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn with_nsu(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram8::NSU_MASK
            } else {
                self.bits & !Sram8::NSU_MASK
            }
        }
    }
}

impl Default for Sram8Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Sram8 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Sram8Val {
        Sram8Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Sram8Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Sram8Val) -> Sram8Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SRAM9 Register
///
/// Control whether debugger, DMA, core 0 and core 1 can access SRAM9, and at what security/privilege levels they can do so.
/// Defaults to fully open access.
/// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
///
/// Address offset: 0x0040
/// Reset value: 0x000000ff
///
/// ## Fields
///
/// - **DBG** `[7]` (RW)
///
///   If 1, SRAM9 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **DMA** `[6]` (RW)
///
///   If 1, SRAM9 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE1** `[5]` (RW)
///
///   If 1, SRAM9 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE0** `[4]` (RW)
///
///   If 1, SRAM9 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **SP** `[3]` (RW)
///
///   If 1, SRAM9 can be accessed from a Secure, Privileged context.
///
/// - **SU** `[2]` (RW)
///
///   If 1, and SP is also set, SRAM9 can be accessed from a Secure, Unprivileged context.
///
/// - **NSP** `[1]` (RW)
///
///   If 1, SRAM9 can be accessed from a Non-secure, Privileged context.
///
/// - **NSU** `[0]` (RW)
///
///   If 1, and NSP is also set, SRAM9 can be accessed from a Non-secure, Unprivileged context.
///   This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
#[repr(transparent)]
pub struct Sram9 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Sram9 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sram9 {
    // ======== Field Constants ========

    /// **DBG**
    ///
    /// If 1, SRAM9 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const DBG_OFFSET: u32 = 7;
    pub const DBG_MASK: u32 = 0x00000080;
    pub const DBG: u32 = 1 << 7;

    /// **DMA**
    ///
    /// If 1, SRAM9 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const DMA_OFFSET: u32 = 6;
    pub const DMA_MASK: u32 = 0x00000040;
    pub const DMA: u32 = 1 << 6;

    /// **CORE1**
    ///
    /// If 1, SRAM9 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CORE1_OFFSET: u32 = 5;
    pub const CORE1_MASK: u32 = 0x00000020;
    pub const CORE1: u32 = 1 << 5;

    /// **CORE0**
    ///
    /// If 1, SRAM9 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CORE0_OFFSET: u32 = 4;
    pub const CORE0_MASK: u32 = 0x00000010;
    pub const CORE0: u32 = 1 << 4;

    /// **SP**
    ///
    /// If 1, SRAM9 can be accessed from a Secure, Privileged context.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SP_OFFSET: u32 = 3;
    pub const SP_MASK: u32 = 0x00000008;
    pub const SP: u32 = 1 << 3;

    /// **SU**
    ///
    /// If 1, and SP is also set, SRAM9 can be accessed from a Secure, Unprivileged context.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SU_OFFSET: u32 = 2;
    pub const SU_MASK: u32 = 0x00000004;
    pub const SU: u32 = 1 << 2;

    /// **NSP**
    ///
    /// If 1, SRAM9 can be accessed from a Non-secure, Privileged context.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const NSP_OFFSET: u32 = 1;
    pub const NSP_MASK: u32 = 0x00000002;
    pub const NSP: u32 = 1 << 1;

    /// **NSU**
    ///
    /// If 1, and NSP is also set, SRAM9 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const NSU_OFFSET: u32 = 0;
    pub const NSU_MASK: u32 = 0x00000001;
    pub const NSU: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DBG** `[7]` (RW)
    ///
    /// If 1, SRAM9 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dbg(&self) -> bool {
        (self.value.read_bits() & Self::DBG_MASK) != 0
    }

    /// Set DBG bit atomically (RW)
    #[inline(always)]
    pub fn set_dbg(&self) {
        self.set_bits(Self::DBG_MASK);
    }

    /// Clear DBG bit atomically (RW)
    #[inline(always)]
    pub fn clr_dbg(&self) {
        self.clr_bits(Self::DBG_MASK);
    }

    /// **DMA** `[6]` (RW)
    ///
    /// If 1, SRAM9 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dma(&self) -> bool {
        (self.value.read_bits() & Self::DMA_MASK) != 0
    }

    /// Set DMA bit atomically (RW)
    #[inline(always)]
    pub fn set_dma(&self) {
        self.set_bits(Self::DMA_MASK);
    }

    /// Clear DMA bit atomically (RW)
    #[inline(always)]
    pub fn clr_dma(&self) {
        self.clr_bits(Self::DMA_MASK);
    }

    /// **CORE1** `[5]` (RW)
    ///
    /// If 1, SRAM9 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core1(&self) -> bool {
        (self.value.read_bits() & Self::CORE1_MASK) != 0
    }

    /// Set CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn set_core1(&self) {
        self.set_bits(Self::CORE1_MASK);
    }

    /// Clear CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core1(&self) {
        self.clr_bits(Self::CORE1_MASK);
    }

    /// **CORE0** `[4]` (RW)
    ///
    /// If 1, SRAM9 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core0(&self) -> bool {
        (self.value.read_bits() & Self::CORE0_MASK) != 0
    }

    /// Set CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn set_core0(&self) {
        self.set_bits(Self::CORE0_MASK);
    }

    /// Clear CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core0(&self) {
        self.clr_bits(Self::CORE0_MASK);
    }

    /// **SP** `[3]` (RW)
    ///
    /// If 1, SRAM9 can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub fn sp(&self) -> bool {
        (self.value.read_bits() & Self::SP_MASK) != 0
    }

    /// Set SP bit atomically (RW)
    #[inline(always)]
    pub fn set_sp(&self) {
        self.set_bits(Self::SP_MASK);
    }

    /// Clear SP bit atomically (RW)
    #[inline(always)]
    pub fn clr_sp(&self) {
        self.clr_bits(Self::SP_MASK);
    }

    /// **SU** `[2]` (RW)
    ///
    /// If 1, and SP is also set, SRAM9 can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub fn su(&self) -> bool {
        (self.value.read_bits() & Self::SU_MASK) != 0
    }

    /// Set SU bit atomically (RW)
    #[inline(always)]
    pub fn set_su(&self) {
        self.set_bits(Self::SU_MASK);
    }

    /// Clear SU bit atomically (RW)
    #[inline(always)]
    pub fn clr_su(&self) {
        self.clr_bits(Self::SU_MASK);
    }

    /// **NSP** `[1]` (RW)
    ///
    /// If 1, SRAM9 can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub fn nsp(&self) -> bool {
        (self.value.read_bits() & Self::NSP_MASK) != 0
    }

    /// Set NSP bit atomically (RW)
    #[inline(always)]
    pub fn set_nsp(&self) {
        self.set_bits(Self::NSP_MASK);
    }

    /// Clear NSP bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsp(&self) {
        self.clr_bits(Self::NSP_MASK);
    }

    /// **NSU** `[0]` (RW)
    ///
    /// If 1, and NSP is also set, SRAM9 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub fn nsu(&self) -> bool {
        (self.value.read_bits() & Self::NSU_MASK) != 0
    }

    /// Set NSU bit atomically (RW)
    #[inline(always)]
    pub fn set_nsu(&self) {
        self.set_bits(Self::NSU_MASK);
    }

    /// Clear NSU bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsu(&self) {
        self.clr_bits(Self::NSU_MASK);
    }
}

/// SRAM9 value type for batch read/modify/write operations
///
/// Use `sram9.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sram9.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Sram9Val {
    bits: u32,
}

impl Sram9Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
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

    /// Read **DBG** `[7:7]`
    ///
    /// If 1, SRAM9 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        (self.bits & Sram9::DBG_MASK) != 0
    }

    /// Read **DMA** `[6:6]`
    ///
    /// If 1, SRAM9 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        (self.bits & Sram9::DMA_MASK) != 0
    }

    /// Read **CORE1** `[5:5]`
    ///
    /// If 1, SRAM9 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        (self.bits & Sram9::CORE1_MASK) != 0
    }

    /// Read **CORE0** `[4:4]`
    ///
    /// If 1, SRAM9 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        (self.bits & Sram9::CORE0_MASK) != 0
    }

    /// Read **SP** `[3:3]`
    ///
    /// If 1, SRAM9 can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        (self.bits & Sram9::SP_MASK) != 0
    }

    /// Read **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, SRAM9 can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn su(&self) -> bool {
        (self.bits & Sram9::SU_MASK) != 0
    }

    /// Read **NSP** `[1:1]`
    ///
    /// If 1, SRAM9 can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        (self.bits & Sram9::NSP_MASK) != 0
    }

    /// Read **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, SRAM9 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        (self.bits & Sram9::NSU_MASK) != 0
    }

    /// Set **DBG** `[7:7]`
    ///
    /// If 1, SRAM9 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dbg(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram9::DBG_MASK
            } else {
                self.bits & !Sram9::DBG_MASK
            }
        }
    }

    /// Set **DMA** `[6:6]`
    ///
    /// If 1, SRAM9 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dma(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram9::DMA_MASK
            } else {
                self.bits & !Sram9::DMA_MASK
            }
        }
    }

    /// Set **CORE1** `[5:5]`
    ///
    /// If 1, SRAM9 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram9::CORE1_MASK
            } else {
                self.bits & !Sram9::CORE1_MASK
            }
        }
    }

    /// Set **CORE0** `[4:4]`
    ///
    /// If 1, SRAM9 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram9::CORE0_MASK
            } else {
                self.bits & !Sram9::CORE0_MASK
            }
        }
    }

    /// Set **SP** `[3:3]`
    ///
    /// If 1, SRAM9 can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn with_sp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram9::SP_MASK
            } else {
                self.bits & !Sram9::SP_MASK
            }
        }
    }

    /// Set **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, SRAM9 can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn with_su(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram9::SU_MASK
            } else {
                self.bits & !Sram9::SU_MASK
            }
        }
    }

    /// Set **NSP** `[1:1]`
    ///
    /// If 1, SRAM9 can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn with_nsp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram9::NSP_MASK
            } else {
                self.bits & !Sram9::NSP_MASK
            }
        }
    }

    /// Set **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, SRAM9 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn with_nsu(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sram9::NSU_MASK
            } else {
                self.bits & !Sram9::NSU_MASK
            }
        }
    }
}

impl Default for Sram9Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Sram9 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Sram9Val {
        Sram9Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Sram9Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Sram9Val) -> Sram9Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DMA Register
///
/// Control whether debugger, DMA, core 0 and core 1 can access DMA, and at what security/privilege levels they can do so.
/// Defaults to Secure access from any master.
/// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
///
/// Address offset: 0x0044
/// Reset value: 0x000000fc
///
/// ## Fields
///
/// - **DBG** `[7]` (RW)
///
///   If 1, DMA can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **DMA** `[6]` (RW)
///
///   If 1, DMA can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE1** `[5]` (RW)
///
///   If 1, DMA can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE0** `[4]` (RW)
///
///   If 1, DMA can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **SP** `[3]` (RW)
///
///   If 1, DMA can be accessed from a Secure, Privileged context.
///
/// - **SU** `[2]` (RW)
///
///   If 1, and SP is also set, DMA can be accessed from a Secure, Unprivileged context.
///
/// - **NSP** `[1]` (RW)
///
///   If 1, DMA can be accessed from a Non-secure, Privileged context.
///
/// - **NSU** `[0]` (RW)
///
///   If 1, and NSP is also set, DMA can be accessed from a Non-secure, Unprivileged context.
///   This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
#[repr(transparent)]
pub struct Dma {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Dma {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Dma {
    // ======== Field Constants ========

    /// **DBG**
    ///
    /// If 1, DMA can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const DBG_OFFSET: u32 = 7;
    pub const DBG_MASK: u32 = 0x00000080;
    pub const DBG: u32 = 1 << 7;

    /// **DMA**
    ///
    /// If 1, DMA can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const DMA_OFFSET: u32 = 6;
    pub const DMA_MASK: u32 = 0x00000040;
    pub const DMA: u32 = 1 << 6;

    /// **CORE1**
    ///
    /// If 1, DMA can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CORE1_OFFSET: u32 = 5;
    pub const CORE1_MASK: u32 = 0x00000020;
    pub const CORE1: u32 = 1 << 5;

    /// **CORE0**
    ///
    /// If 1, DMA can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CORE0_OFFSET: u32 = 4;
    pub const CORE0_MASK: u32 = 0x00000010;
    pub const CORE0: u32 = 1 << 4;

    /// **SP**
    ///
    /// If 1, DMA can be accessed from a Secure, Privileged context.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SP_OFFSET: u32 = 3;
    pub const SP_MASK: u32 = 0x00000008;
    pub const SP: u32 = 1 << 3;

    /// **SU**
    ///
    /// If 1, and SP is also set, DMA can be accessed from a Secure, Unprivileged context.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SU_OFFSET: u32 = 2;
    pub const SU_MASK: u32 = 0x00000004;
    pub const SU: u32 = 1 << 2;

    /// **NSP**
    ///
    /// If 1, DMA can be accessed from a Non-secure, Privileged context.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const NSP_OFFSET: u32 = 1;
    pub const NSP_MASK: u32 = 0x00000002;
    pub const NSP: u32 = 1 << 1;

    /// **NSU**
    ///
    /// If 1, and NSP is also set, DMA can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const NSU_OFFSET: u32 = 0;
    pub const NSU_MASK: u32 = 0x00000001;
    pub const NSU: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DBG** `[7]` (RW)
    ///
    /// If 1, DMA can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dbg(&self) -> bool {
        (self.value.read_bits() & Self::DBG_MASK) != 0
    }

    /// Set DBG bit atomically (RW)
    #[inline(always)]
    pub fn set_dbg(&self) {
        self.set_bits(Self::DBG_MASK);
    }

    /// Clear DBG bit atomically (RW)
    #[inline(always)]
    pub fn clr_dbg(&self) {
        self.clr_bits(Self::DBG_MASK);
    }

    /// **DMA** `[6]` (RW)
    ///
    /// If 1, DMA can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dma(&self) -> bool {
        (self.value.read_bits() & Self::DMA_MASK) != 0
    }

    /// Set DMA bit atomically (RW)
    #[inline(always)]
    pub fn set_dma(&self) {
        self.set_bits(Self::DMA_MASK);
    }

    /// Clear DMA bit atomically (RW)
    #[inline(always)]
    pub fn clr_dma(&self) {
        self.clr_bits(Self::DMA_MASK);
    }

    /// **CORE1** `[5]` (RW)
    ///
    /// If 1, DMA can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core1(&self) -> bool {
        (self.value.read_bits() & Self::CORE1_MASK) != 0
    }

    /// Set CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn set_core1(&self) {
        self.set_bits(Self::CORE1_MASK);
    }

    /// Clear CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core1(&self) {
        self.clr_bits(Self::CORE1_MASK);
    }

    /// **CORE0** `[4]` (RW)
    ///
    /// If 1, DMA can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core0(&self) -> bool {
        (self.value.read_bits() & Self::CORE0_MASK) != 0
    }

    /// Set CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn set_core0(&self) {
        self.set_bits(Self::CORE0_MASK);
    }

    /// Clear CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core0(&self) {
        self.clr_bits(Self::CORE0_MASK);
    }

    /// **SP** `[3]` (RW)
    ///
    /// If 1, DMA can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub fn sp(&self) -> bool {
        (self.value.read_bits() & Self::SP_MASK) != 0
    }

    /// Set SP bit atomically (RW)
    #[inline(always)]
    pub fn set_sp(&self) {
        self.set_bits(Self::SP_MASK);
    }

    /// Clear SP bit atomically (RW)
    #[inline(always)]
    pub fn clr_sp(&self) {
        self.clr_bits(Self::SP_MASK);
    }

    /// **SU** `[2]` (RW)
    ///
    /// If 1, and SP is also set, DMA can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub fn su(&self) -> bool {
        (self.value.read_bits() & Self::SU_MASK) != 0
    }

    /// Set SU bit atomically (RW)
    #[inline(always)]
    pub fn set_su(&self) {
        self.set_bits(Self::SU_MASK);
    }

    /// Clear SU bit atomically (RW)
    #[inline(always)]
    pub fn clr_su(&self) {
        self.clr_bits(Self::SU_MASK);
    }

    /// **NSP** `[1]` (RW)
    ///
    /// If 1, DMA can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub fn nsp(&self) -> bool {
        (self.value.read_bits() & Self::NSP_MASK) != 0
    }

    /// Set NSP bit atomically (RW)
    #[inline(always)]
    pub fn set_nsp(&self) {
        self.set_bits(Self::NSP_MASK);
    }

    /// Clear NSP bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsp(&self) {
        self.clr_bits(Self::NSP_MASK);
    }

    /// **NSU** `[0]` (RW)
    ///
    /// If 1, and NSP is also set, DMA can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub fn nsu(&self) -> bool {
        (self.value.read_bits() & Self::NSU_MASK) != 0
    }

    /// Set NSU bit atomically (RW)
    #[inline(always)]
    pub fn set_nsu(&self) {
        self.set_bits(Self::NSU_MASK);
    }

    /// Clear NSU bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsu(&self) {
        self.clr_bits(Self::NSU_MASK);
    }
}

/// DMA value type for batch read/modify/write operations
///
/// Use `dma.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dma.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DmaVal {
    bits: u32,
}

impl DmaVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x000000fc }
    }

    /// Read **DBG** `[7:7]`
    ///
    /// If 1, DMA can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        (self.bits & Dma::DBG_MASK) != 0
    }

    /// Read **DMA** `[6:6]`
    ///
    /// If 1, DMA can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        (self.bits & Dma::DMA_MASK) != 0
    }

    /// Read **CORE1** `[5:5]`
    ///
    /// If 1, DMA can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        (self.bits & Dma::CORE1_MASK) != 0
    }

    /// Read **CORE0** `[4:4]`
    ///
    /// If 1, DMA can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        (self.bits & Dma::CORE0_MASK) != 0
    }

    /// Read **SP** `[3:3]`
    ///
    /// If 1, DMA can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        (self.bits & Dma::SP_MASK) != 0
    }

    /// Read **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, DMA can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn su(&self) -> bool {
        (self.bits & Dma::SU_MASK) != 0
    }

    /// Read **NSP** `[1:1]`
    ///
    /// If 1, DMA can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        (self.bits & Dma::NSP_MASK) != 0
    }

    /// Read **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, DMA can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        (self.bits & Dma::NSU_MASK) != 0
    }

    /// Set **DBG** `[7:7]`
    ///
    /// If 1, DMA can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dbg(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Dma::DBG_MASK
            } else {
                self.bits & !Dma::DBG_MASK
            }
        }
    }

    /// Set **DMA** `[6:6]`
    ///
    /// If 1, DMA can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dma(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Dma::DMA_MASK
            } else {
                self.bits & !Dma::DMA_MASK
            }
        }
    }

    /// Set **CORE1** `[5:5]`
    ///
    /// If 1, DMA can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Dma::CORE1_MASK
            } else {
                self.bits & !Dma::CORE1_MASK
            }
        }
    }

    /// Set **CORE0** `[4:4]`
    ///
    /// If 1, DMA can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Dma::CORE0_MASK
            } else {
                self.bits & !Dma::CORE0_MASK
            }
        }
    }

    /// Set **SP** `[3:3]`
    ///
    /// If 1, DMA can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn with_sp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Dma::SP_MASK
            } else {
                self.bits & !Dma::SP_MASK
            }
        }
    }

    /// Set **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, DMA can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn with_su(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Dma::SU_MASK
            } else {
                self.bits & !Dma::SU_MASK
            }
        }
    }

    /// Set **NSP** `[1:1]`
    ///
    /// If 1, DMA can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn with_nsp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Dma::NSP_MASK
            } else {
                self.bits & !Dma::NSP_MASK
            }
        }
    }

    /// Set **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, DMA can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn with_nsu(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Dma::NSU_MASK
            } else {
                self.bits & !Dma::NSU_MASK
            }
        }
    }
}

impl Default for DmaVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Dma {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DmaVal {
        DmaVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DmaVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DmaVal) -> DmaVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// USBCTRL Register
///
/// Control whether debugger, DMA, core 0 and core 1 can access USBCTRL, and at what security/privilege levels they can do so.
/// Defaults to Secure access from any master.
/// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
///
/// Address offset: 0x0048
/// Reset value: 0x000000fc
///
/// ## Fields
///
/// - **DBG** `[7]` (RW)
///
///   If 1, USBCTRL can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **DMA** `[6]` (RW)
///
///   If 1, USBCTRL can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE1** `[5]` (RW)
///
///   If 1, USBCTRL can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE0** `[4]` (RW)
///
///   If 1, USBCTRL can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **SP** `[3]` (RW)
///
///   If 1, USBCTRL can be accessed from a Secure, Privileged context.
///
/// - **SU** `[2]` (RW)
///
///   If 1, and SP is also set, USBCTRL can be accessed from a Secure, Unprivileged context.
///
/// - **NSP** `[1]` (RW)
///
///   If 1, USBCTRL can be accessed from a Non-secure, Privileged context.
///
/// - **NSU** `[0]` (RW)
///
///   If 1, and NSP is also set, USBCTRL can be accessed from a Non-secure, Unprivileged context.
///   This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
#[repr(transparent)]
pub struct Usbctrl {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Usbctrl {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Usbctrl {
    // ======== Field Constants ========

    /// **DBG**
    ///
    /// If 1, USBCTRL can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const DBG_OFFSET: u32 = 7;
    pub const DBG_MASK: u32 = 0x00000080;
    pub const DBG: u32 = 1 << 7;

    /// **DMA**
    ///
    /// If 1, USBCTRL can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const DMA_OFFSET: u32 = 6;
    pub const DMA_MASK: u32 = 0x00000040;
    pub const DMA: u32 = 1 << 6;

    /// **CORE1**
    ///
    /// If 1, USBCTRL can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CORE1_OFFSET: u32 = 5;
    pub const CORE1_MASK: u32 = 0x00000020;
    pub const CORE1: u32 = 1 << 5;

    /// **CORE0**
    ///
    /// If 1, USBCTRL can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CORE0_OFFSET: u32 = 4;
    pub const CORE0_MASK: u32 = 0x00000010;
    pub const CORE0: u32 = 1 << 4;

    /// **SP**
    ///
    /// If 1, USBCTRL can be accessed from a Secure, Privileged context.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SP_OFFSET: u32 = 3;
    pub const SP_MASK: u32 = 0x00000008;
    pub const SP: u32 = 1 << 3;

    /// **SU**
    ///
    /// If 1, and SP is also set, USBCTRL can be accessed from a Secure, Unprivileged context.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SU_OFFSET: u32 = 2;
    pub const SU_MASK: u32 = 0x00000004;
    pub const SU: u32 = 1 << 2;

    /// **NSP**
    ///
    /// If 1, USBCTRL can be accessed from a Non-secure, Privileged context.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const NSP_OFFSET: u32 = 1;
    pub const NSP_MASK: u32 = 0x00000002;
    pub const NSP: u32 = 1 << 1;

    /// **NSU**
    ///
    /// If 1, and NSP is also set, USBCTRL can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const NSU_OFFSET: u32 = 0;
    pub const NSU_MASK: u32 = 0x00000001;
    pub const NSU: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DBG** `[7]` (RW)
    ///
    /// If 1, USBCTRL can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dbg(&self) -> bool {
        (self.value.read_bits() & Self::DBG_MASK) != 0
    }

    /// Set DBG bit atomically (RW)
    #[inline(always)]
    pub fn set_dbg(&self) {
        self.set_bits(Self::DBG_MASK);
    }

    /// Clear DBG bit atomically (RW)
    #[inline(always)]
    pub fn clr_dbg(&self) {
        self.clr_bits(Self::DBG_MASK);
    }

    /// **DMA** `[6]` (RW)
    ///
    /// If 1, USBCTRL can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dma(&self) -> bool {
        (self.value.read_bits() & Self::DMA_MASK) != 0
    }

    /// Set DMA bit atomically (RW)
    #[inline(always)]
    pub fn set_dma(&self) {
        self.set_bits(Self::DMA_MASK);
    }

    /// Clear DMA bit atomically (RW)
    #[inline(always)]
    pub fn clr_dma(&self) {
        self.clr_bits(Self::DMA_MASK);
    }

    /// **CORE1** `[5]` (RW)
    ///
    /// If 1, USBCTRL can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core1(&self) -> bool {
        (self.value.read_bits() & Self::CORE1_MASK) != 0
    }

    /// Set CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn set_core1(&self) {
        self.set_bits(Self::CORE1_MASK);
    }

    /// Clear CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core1(&self) {
        self.clr_bits(Self::CORE1_MASK);
    }

    /// **CORE0** `[4]` (RW)
    ///
    /// If 1, USBCTRL can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core0(&self) -> bool {
        (self.value.read_bits() & Self::CORE0_MASK) != 0
    }

    /// Set CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn set_core0(&self) {
        self.set_bits(Self::CORE0_MASK);
    }

    /// Clear CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core0(&self) {
        self.clr_bits(Self::CORE0_MASK);
    }

    /// **SP** `[3]` (RW)
    ///
    /// If 1, USBCTRL can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub fn sp(&self) -> bool {
        (self.value.read_bits() & Self::SP_MASK) != 0
    }

    /// Set SP bit atomically (RW)
    #[inline(always)]
    pub fn set_sp(&self) {
        self.set_bits(Self::SP_MASK);
    }

    /// Clear SP bit atomically (RW)
    #[inline(always)]
    pub fn clr_sp(&self) {
        self.clr_bits(Self::SP_MASK);
    }

    /// **SU** `[2]` (RW)
    ///
    /// If 1, and SP is also set, USBCTRL can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub fn su(&self) -> bool {
        (self.value.read_bits() & Self::SU_MASK) != 0
    }

    /// Set SU bit atomically (RW)
    #[inline(always)]
    pub fn set_su(&self) {
        self.set_bits(Self::SU_MASK);
    }

    /// Clear SU bit atomically (RW)
    #[inline(always)]
    pub fn clr_su(&self) {
        self.clr_bits(Self::SU_MASK);
    }

    /// **NSP** `[1]` (RW)
    ///
    /// If 1, USBCTRL can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub fn nsp(&self) -> bool {
        (self.value.read_bits() & Self::NSP_MASK) != 0
    }

    /// Set NSP bit atomically (RW)
    #[inline(always)]
    pub fn set_nsp(&self) {
        self.set_bits(Self::NSP_MASK);
    }

    /// Clear NSP bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsp(&self) {
        self.clr_bits(Self::NSP_MASK);
    }

    /// **NSU** `[0]` (RW)
    ///
    /// If 1, and NSP is also set, USBCTRL can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub fn nsu(&self) -> bool {
        (self.value.read_bits() & Self::NSU_MASK) != 0
    }

    /// Set NSU bit atomically (RW)
    #[inline(always)]
    pub fn set_nsu(&self) {
        self.set_bits(Self::NSU_MASK);
    }

    /// Clear NSU bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsu(&self) {
        self.clr_bits(Self::NSU_MASK);
    }
}

/// USBCTRL value type for batch read/modify/write operations
///
/// Use `usbctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `usbctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct UsbctrlVal {
    bits: u32,
}

impl UsbctrlVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x000000fc }
    }

    /// Read **DBG** `[7:7]`
    ///
    /// If 1, USBCTRL can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        (self.bits & Usbctrl::DBG_MASK) != 0
    }

    /// Read **DMA** `[6:6]`
    ///
    /// If 1, USBCTRL can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        (self.bits & Usbctrl::DMA_MASK) != 0
    }

    /// Read **CORE1** `[5:5]`
    ///
    /// If 1, USBCTRL can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        (self.bits & Usbctrl::CORE1_MASK) != 0
    }

    /// Read **CORE0** `[4:4]`
    ///
    /// If 1, USBCTRL can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        (self.bits & Usbctrl::CORE0_MASK) != 0
    }

    /// Read **SP** `[3:3]`
    ///
    /// If 1, USBCTRL can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        (self.bits & Usbctrl::SP_MASK) != 0
    }

    /// Read **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, USBCTRL can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn su(&self) -> bool {
        (self.bits & Usbctrl::SU_MASK) != 0
    }

    /// Read **NSP** `[1:1]`
    ///
    /// If 1, USBCTRL can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        (self.bits & Usbctrl::NSP_MASK) != 0
    }

    /// Read **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, USBCTRL can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        (self.bits & Usbctrl::NSU_MASK) != 0
    }

    /// Set **DBG** `[7:7]`
    ///
    /// If 1, USBCTRL can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dbg(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Usbctrl::DBG_MASK
            } else {
                self.bits & !Usbctrl::DBG_MASK
            }
        }
    }

    /// Set **DMA** `[6:6]`
    ///
    /// If 1, USBCTRL can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dma(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Usbctrl::DMA_MASK
            } else {
                self.bits & !Usbctrl::DMA_MASK
            }
        }
    }

    /// Set **CORE1** `[5:5]`
    ///
    /// If 1, USBCTRL can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Usbctrl::CORE1_MASK
            } else {
                self.bits & !Usbctrl::CORE1_MASK
            }
        }
    }

    /// Set **CORE0** `[4:4]`
    ///
    /// If 1, USBCTRL can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Usbctrl::CORE0_MASK
            } else {
                self.bits & !Usbctrl::CORE0_MASK
            }
        }
    }

    /// Set **SP** `[3:3]`
    ///
    /// If 1, USBCTRL can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn with_sp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Usbctrl::SP_MASK
            } else {
                self.bits & !Usbctrl::SP_MASK
            }
        }
    }

    /// Set **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, USBCTRL can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn with_su(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Usbctrl::SU_MASK
            } else {
                self.bits & !Usbctrl::SU_MASK
            }
        }
    }

    /// Set **NSP** `[1:1]`
    ///
    /// If 1, USBCTRL can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn with_nsp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Usbctrl::NSP_MASK
            } else {
                self.bits & !Usbctrl::NSP_MASK
            }
        }
    }

    /// Set **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, USBCTRL can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn with_nsu(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Usbctrl::NSU_MASK
            } else {
                self.bits & !Usbctrl::NSU_MASK
            }
        }
    }
}

impl Default for UsbctrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Usbctrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> UsbctrlVal {
        UsbctrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: UsbctrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(UsbctrlVal) -> UsbctrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PIO0 Register
///
/// Control whether debugger, DMA, core 0 and core 1 can access PIO0, and at what security/privilege levels they can do so.
/// Defaults to Secure access from any master.
/// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
///
/// Address offset: 0x004c
/// Reset value: 0x000000fc
///
/// ## Fields
///
/// - **DBG** `[7]` (RW)
///
///   If 1, PIO0 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **DMA** `[6]` (RW)
///
///   If 1, PIO0 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE1** `[5]` (RW)
///
///   If 1, PIO0 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE0** `[4]` (RW)
///
///   If 1, PIO0 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **SP** `[3]` (RW)
///
///   If 1, PIO0 can be accessed from a Secure, Privileged context.
///
/// - **SU** `[2]` (RW)
///
///   If 1, and SP is also set, PIO0 can be accessed from a Secure, Unprivileged context.
///
/// - **NSP** `[1]` (RW)
///
///   If 1, PIO0 can be accessed from a Non-secure, Privileged context.
///
/// - **NSU** `[0]` (RW)
///
///   If 1, and NSP is also set, PIO0 can be accessed from a Non-secure, Unprivileged context.
///   This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
#[repr(transparent)]
pub struct Pio0 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Pio0 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Pio0 {
    // ======== Field Constants ========

    /// **DBG**
    ///
    /// If 1, PIO0 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const DBG_OFFSET: u32 = 7;
    pub const DBG_MASK: u32 = 0x00000080;
    pub const DBG: u32 = 1 << 7;

    /// **DMA**
    ///
    /// If 1, PIO0 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const DMA_OFFSET: u32 = 6;
    pub const DMA_MASK: u32 = 0x00000040;
    pub const DMA: u32 = 1 << 6;

    /// **CORE1**
    ///
    /// If 1, PIO0 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CORE1_OFFSET: u32 = 5;
    pub const CORE1_MASK: u32 = 0x00000020;
    pub const CORE1: u32 = 1 << 5;

    /// **CORE0**
    ///
    /// If 1, PIO0 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CORE0_OFFSET: u32 = 4;
    pub const CORE0_MASK: u32 = 0x00000010;
    pub const CORE0: u32 = 1 << 4;

    /// **SP**
    ///
    /// If 1, PIO0 can be accessed from a Secure, Privileged context.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SP_OFFSET: u32 = 3;
    pub const SP_MASK: u32 = 0x00000008;
    pub const SP: u32 = 1 << 3;

    /// **SU**
    ///
    /// If 1, and SP is also set, PIO0 can be accessed from a Secure, Unprivileged context.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SU_OFFSET: u32 = 2;
    pub const SU_MASK: u32 = 0x00000004;
    pub const SU: u32 = 1 << 2;

    /// **NSP**
    ///
    /// If 1, PIO0 can be accessed from a Non-secure, Privileged context.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const NSP_OFFSET: u32 = 1;
    pub const NSP_MASK: u32 = 0x00000002;
    pub const NSP: u32 = 1 << 1;

    /// **NSU**
    ///
    /// If 1, and NSP is also set, PIO0 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const NSU_OFFSET: u32 = 0;
    pub const NSU_MASK: u32 = 0x00000001;
    pub const NSU: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DBG** `[7]` (RW)
    ///
    /// If 1, PIO0 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dbg(&self) -> bool {
        (self.value.read_bits() & Self::DBG_MASK) != 0
    }

    /// Set DBG bit atomically (RW)
    #[inline(always)]
    pub fn set_dbg(&self) {
        self.set_bits(Self::DBG_MASK);
    }

    /// Clear DBG bit atomically (RW)
    #[inline(always)]
    pub fn clr_dbg(&self) {
        self.clr_bits(Self::DBG_MASK);
    }

    /// **DMA** `[6]` (RW)
    ///
    /// If 1, PIO0 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dma(&self) -> bool {
        (self.value.read_bits() & Self::DMA_MASK) != 0
    }

    /// Set DMA bit atomically (RW)
    #[inline(always)]
    pub fn set_dma(&self) {
        self.set_bits(Self::DMA_MASK);
    }

    /// Clear DMA bit atomically (RW)
    #[inline(always)]
    pub fn clr_dma(&self) {
        self.clr_bits(Self::DMA_MASK);
    }

    /// **CORE1** `[5]` (RW)
    ///
    /// If 1, PIO0 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core1(&self) -> bool {
        (self.value.read_bits() & Self::CORE1_MASK) != 0
    }

    /// Set CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn set_core1(&self) {
        self.set_bits(Self::CORE1_MASK);
    }

    /// Clear CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core1(&self) {
        self.clr_bits(Self::CORE1_MASK);
    }

    /// **CORE0** `[4]` (RW)
    ///
    /// If 1, PIO0 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core0(&self) -> bool {
        (self.value.read_bits() & Self::CORE0_MASK) != 0
    }

    /// Set CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn set_core0(&self) {
        self.set_bits(Self::CORE0_MASK);
    }

    /// Clear CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core0(&self) {
        self.clr_bits(Self::CORE0_MASK);
    }

    /// **SP** `[3]` (RW)
    ///
    /// If 1, PIO0 can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub fn sp(&self) -> bool {
        (self.value.read_bits() & Self::SP_MASK) != 0
    }

    /// Set SP bit atomically (RW)
    #[inline(always)]
    pub fn set_sp(&self) {
        self.set_bits(Self::SP_MASK);
    }

    /// Clear SP bit atomically (RW)
    #[inline(always)]
    pub fn clr_sp(&self) {
        self.clr_bits(Self::SP_MASK);
    }

    /// **SU** `[2]` (RW)
    ///
    /// If 1, and SP is also set, PIO0 can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub fn su(&self) -> bool {
        (self.value.read_bits() & Self::SU_MASK) != 0
    }

    /// Set SU bit atomically (RW)
    #[inline(always)]
    pub fn set_su(&self) {
        self.set_bits(Self::SU_MASK);
    }

    /// Clear SU bit atomically (RW)
    #[inline(always)]
    pub fn clr_su(&self) {
        self.clr_bits(Self::SU_MASK);
    }

    /// **NSP** `[1]` (RW)
    ///
    /// If 1, PIO0 can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub fn nsp(&self) -> bool {
        (self.value.read_bits() & Self::NSP_MASK) != 0
    }

    /// Set NSP bit atomically (RW)
    #[inline(always)]
    pub fn set_nsp(&self) {
        self.set_bits(Self::NSP_MASK);
    }

    /// Clear NSP bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsp(&self) {
        self.clr_bits(Self::NSP_MASK);
    }

    /// **NSU** `[0]` (RW)
    ///
    /// If 1, and NSP is also set, PIO0 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub fn nsu(&self) -> bool {
        (self.value.read_bits() & Self::NSU_MASK) != 0
    }

    /// Set NSU bit atomically (RW)
    #[inline(always)]
    pub fn set_nsu(&self) {
        self.set_bits(Self::NSU_MASK);
    }

    /// Clear NSU bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsu(&self) {
        self.clr_bits(Self::NSU_MASK);
    }
}

/// PIO0 value type for batch read/modify/write operations
///
/// Use `pio0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pio0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Pio0Val {
    bits: u32,
}

impl Pio0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x000000fc }
    }

    /// Read **DBG** `[7:7]`
    ///
    /// If 1, PIO0 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        (self.bits & Pio0::DBG_MASK) != 0
    }

    /// Read **DMA** `[6:6]`
    ///
    /// If 1, PIO0 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        (self.bits & Pio0::DMA_MASK) != 0
    }

    /// Read **CORE1** `[5:5]`
    ///
    /// If 1, PIO0 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        (self.bits & Pio0::CORE1_MASK) != 0
    }

    /// Read **CORE0** `[4:4]`
    ///
    /// If 1, PIO0 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        (self.bits & Pio0::CORE0_MASK) != 0
    }

    /// Read **SP** `[3:3]`
    ///
    /// If 1, PIO0 can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        (self.bits & Pio0::SP_MASK) != 0
    }

    /// Read **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, PIO0 can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn su(&self) -> bool {
        (self.bits & Pio0::SU_MASK) != 0
    }

    /// Read **NSP** `[1:1]`
    ///
    /// If 1, PIO0 can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        (self.bits & Pio0::NSP_MASK) != 0
    }

    /// Read **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, PIO0 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        (self.bits & Pio0::NSU_MASK) != 0
    }

    /// Set **DBG** `[7:7]`
    ///
    /// If 1, PIO0 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dbg(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pio0::DBG_MASK
            } else {
                self.bits & !Pio0::DBG_MASK
            }
        }
    }

    /// Set **DMA** `[6:6]`
    ///
    /// If 1, PIO0 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dma(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pio0::DMA_MASK
            } else {
                self.bits & !Pio0::DMA_MASK
            }
        }
    }

    /// Set **CORE1** `[5:5]`
    ///
    /// If 1, PIO0 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pio0::CORE1_MASK
            } else {
                self.bits & !Pio0::CORE1_MASK
            }
        }
    }

    /// Set **CORE0** `[4:4]`
    ///
    /// If 1, PIO0 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pio0::CORE0_MASK
            } else {
                self.bits & !Pio0::CORE0_MASK
            }
        }
    }

    /// Set **SP** `[3:3]`
    ///
    /// If 1, PIO0 can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn with_sp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pio0::SP_MASK
            } else {
                self.bits & !Pio0::SP_MASK
            }
        }
    }

    /// Set **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, PIO0 can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn with_su(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pio0::SU_MASK
            } else {
                self.bits & !Pio0::SU_MASK
            }
        }
    }

    /// Set **NSP** `[1:1]`
    ///
    /// If 1, PIO0 can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn with_nsp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pio0::NSP_MASK
            } else {
                self.bits & !Pio0::NSP_MASK
            }
        }
    }

    /// Set **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, PIO0 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn with_nsu(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pio0::NSU_MASK
            } else {
                self.bits & !Pio0::NSU_MASK
            }
        }
    }
}

impl Default for Pio0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Pio0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Pio0Val {
        Pio0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Pio0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Pio0Val) -> Pio0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PIO1 Register
///
/// Control whether debugger, DMA, core 0 and core 1 can access PIO1, and at what security/privilege levels they can do so.
/// Defaults to Secure access from any master.
/// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
///
/// Address offset: 0x0050
/// Reset value: 0x000000fc
///
/// ## Fields
///
/// - **DBG** `[7]` (RW)
///
///   If 1, PIO1 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **DMA** `[6]` (RW)
///
///   If 1, PIO1 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE1** `[5]` (RW)
///
///   If 1, PIO1 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE0** `[4]` (RW)
///
///   If 1, PIO1 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **SP** `[3]` (RW)
///
///   If 1, PIO1 can be accessed from a Secure, Privileged context.
///
/// - **SU** `[2]` (RW)
///
///   If 1, and SP is also set, PIO1 can be accessed from a Secure, Unprivileged context.
///
/// - **NSP** `[1]` (RW)
///
///   If 1, PIO1 can be accessed from a Non-secure, Privileged context.
///
/// - **NSU** `[0]` (RW)
///
///   If 1, and NSP is also set, PIO1 can be accessed from a Non-secure, Unprivileged context.
///   This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
#[repr(transparent)]
pub struct Pio1 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Pio1 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Pio1 {
    // ======== Field Constants ========

    /// **DBG**
    ///
    /// If 1, PIO1 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const DBG_OFFSET: u32 = 7;
    pub const DBG_MASK: u32 = 0x00000080;
    pub const DBG: u32 = 1 << 7;

    /// **DMA**
    ///
    /// If 1, PIO1 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const DMA_OFFSET: u32 = 6;
    pub const DMA_MASK: u32 = 0x00000040;
    pub const DMA: u32 = 1 << 6;

    /// **CORE1**
    ///
    /// If 1, PIO1 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CORE1_OFFSET: u32 = 5;
    pub const CORE1_MASK: u32 = 0x00000020;
    pub const CORE1: u32 = 1 << 5;

    /// **CORE0**
    ///
    /// If 1, PIO1 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CORE0_OFFSET: u32 = 4;
    pub const CORE0_MASK: u32 = 0x00000010;
    pub const CORE0: u32 = 1 << 4;

    /// **SP**
    ///
    /// If 1, PIO1 can be accessed from a Secure, Privileged context.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SP_OFFSET: u32 = 3;
    pub const SP_MASK: u32 = 0x00000008;
    pub const SP: u32 = 1 << 3;

    /// **SU**
    ///
    /// If 1, and SP is also set, PIO1 can be accessed from a Secure, Unprivileged context.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SU_OFFSET: u32 = 2;
    pub const SU_MASK: u32 = 0x00000004;
    pub const SU: u32 = 1 << 2;

    /// **NSP**
    ///
    /// If 1, PIO1 can be accessed from a Non-secure, Privileged context.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const NSP_OFFSET: u32 = 1;
    pub const NSP_MASK: u32 = 0x00000002;
    pub const NSP: u32 = 1 << 1;

    /// **NSU**
    ///
    /// If 1, and NSP is also set, PIO1 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const NSU_OFFSET: u32 = 0;
    pub const NSU_MASK: u32 = 0x00000001;
    pub const NSU: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DBG** `[7]` (RW)
    ///
    /// If 1, PIO1 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dbg(&self) -> bool {
        (self.value.read_bits() & Self::DBG_MASK) != 0
    }

    /// Set DBG bit atomically (RW)
    #[inline(always)]
    pub fn set_dbg(&self) {
        self.set_bits(Self::DBG_MASK);
    }

    /// Clear DBG bit atomically (RW)
    #[inline(always)]
    pub fn clr_dbg(&self) {
        self.clr_bits(Self::DBG_MASK);
    }

    /// **DMA** `[6]` (RW)
    ///
    /// If 1, PIO1 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dma(&self) -> bool {
        (self.value.read_bits() & Self::DMA_MASK) != 0
    }

    /// Set DMA bit atomically (RW)
    #[inline(always)]
    pub fn set_dma(&self) {
        self.set_bits(Self::DMA_MASK);
    }

    /// Clear DMA bit atomically (RW)
    #[inline(always)]
    pub fn clr_dma(&self) {
        self.clr_bits(Self::DMA_MASK);
    }

    /// **CORE1** `[5]` (RW)
    ///
    /// If 1, PIO1 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core1(&self) -> bool {
        (self.value.read_bits() & Self::CORE1_MASK) != 0
    }

    /// Set CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn set_core1(&self) {
        self.set_bits(Self::CORE1_MASK);
    }

    /// Clear CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core1(&self) {
        self.clr_bits(Self::CORE1_MASK);
    }

    /// **CORE0** `[4]` (RW)
    ///
    /// If 1, PIO1 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core0(&self) -> bool {
        (self.value.read_bits() & Self::CORE0_MASK) != 0
    }

    /// Set CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn set_core0(&self) {
        self.set_bits(Self::CORE0_MASK);
    }

    /// Clear CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core0(&self) {
        self.clr_bits(Self::CORE0_MASK);
    }

    /// **SP** `[3]` (RW)
    ///
    /// If 1, PIO1 can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub fn sp(&self) -> bool {
        (self.value.read_bits() & Self::SP_MASK) != 0
    }

    /// Set SP bit atomically (RW)
    #[inline(always)]
    pub fn set_sp(&self) {
        self.set_bits(Self::SP_MASK);
    }

    /// Clear SP bit atomically (RW)
    #[inline(always)]
    pub fn clr_sp(&self) {
        self.clr_bits(Self::SP_MASK);
    }

    /// **SU** `[2]` (RW)
    ///
    /// If 1, and SP is also set, PIO1 can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub fn su(&self) -> bool {
        (self.value.read_bits() & Self::SU_MASK) != 0
    }

    /// Set SU bit atomically (RW)
    #[inline(always)]
    pub fn set_su(&self) {
        self.set_bits(Self::SU_MASK);
    }

    /// Clear SU bit atomically (RW)
    #[inline(always)]
    pub fn clr_su(&self) {
        self.clr_bits(Self::SU_MASK);
    }

    /// **NSP** `[1]` (RW)
    ///
    /// If 1, PIO1 can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub fn nsp(&self) -> bool {
        (self.value.read_bits() & Self::NSP_MASK) != 0
    }

    /// Set NSP bit atomically (RW)
    #[inline(always)]
    pub fn set_nsp(&self) {
        self.set_bits(Self::NSP_MASK);
    }

    /// Clear NSP bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsp(&self) {
        self.clr_bits(Self::NSP_MASK);
    }

    /// **NSU** `[0]` (RW)
    ///
    /// If 1, and NSP is also set, PIO1 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub fn nsu(&self) -> bool {
        (self.value.read_bits() & Self::NSU_MASK) != 0
    }

    /// Set NSU bit atomically (RW)
    #[inline(always)]
    pub fn set_nsu(&self) {
        self.set_bits(Self::NSU_MASK);
    }

    /// Clear NSU bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsu(&self) {
        self.clr_bits(Self::NSU_MASK);
    }
}

/// PIO1 value type for batch read/modify/write operations
///
/// Use `pio1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pio1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Pio1Val {
    bits: u32,
}

impl Pio1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x000000fc }
    }

    /// Read **DBG** `[7:7]`
    ///
    /// If 1, PIO1 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        (self.bits & Pio1::DBG_MASK) != 0
    }

    /// Read **DMA** `[6:6]`
    ///
    /// If 1, PIO1 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        (self.bits & Pio1::DMA_MASK) != 0
    }

    /// Read **CORE1** `[5:5]`
    ///
    /// If 1, PIO1 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        (self.bits & Pio1::CORE1_MASK) != 0
    }

    /// Read **CORE0** `[4:4]`
    ///
    /// If 1, PIO1 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        (self.bits & Pio1::CORE0_MASK) != 0
    }

    /// Read **SP** `[3:3]`
    ///
    /// If 1, PIO1 can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        (self.bits & Pio1::SP_MASK) != 0
    }

    /// Read **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, PIO1 can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn su(&self) -> bool {
        (self.bits & Pio1::SU_MASK) != 0
    }

    /// Read **NSP** `[1:1]`
    ///
    /// If 1, PIO1 can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        (self.bits & Pio1::NSP_MASK) != 0
    }

    /// Read **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, PIO1 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        (self.bits & Pio1::NSU_MASK) != 0
    }

    /// Set **DBG** `[7:7]`
    ///
    /// If 1, PIO1 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dbg(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pio1::DBG_MASK
            } else {
                self.bits & !Pio1::DBG_MASK
            }
        }
    }

    /// Set **DMA** `[6:6]`
    ///
    /// If 1, PIO1 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dma(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pio1::DMA_MASK
            } else {
                self.bits & !Pio1::DMA_MASK
            }
        }
    }

    /// Set **CORE1** `[5:5]`
    ///
    /// If 1, PIO1 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pio1::CORE1_MASK
            } else {
                self.bits & !Pio1::CORE1_MASK
            }
        }
    }

    /// Set **CORE0** `[4:4]`
    ///
    /// If 1, PIO1 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pio1::CORE0_MASK
            } else {
                self.bits & !Pio1::CORE0_MASK
            }
        }
    }

    /// Set **SP** `[3:3]`
    ///
    /// If 1, PIO1 can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn with_sp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pio1::SP_MASK
            } else {
                self.bits & !Pio1::SP_MASK
            }
        }
    }

    /// Set **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, PIO1 can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn with_su(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pio1::SU_MASK
            } else {
                self.bits & !Pio1::SU_MASK
            }
        }
    }

    /// Set **NSP** `[1:1]`
    ///
    /// If 1, PIO1 can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn with_nsp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pio1::NSP_MASK
            } else {
                self.bits & !Pio1::NSP_MASK
            }
        }
    }

    /// Set **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, PIO1 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn with_nsu(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pio1::NSU_MASK
            } else {
                self.bits & !Pio1::NSU_MASK
            }
        }
    }
}

impl Default for Pio1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Pio1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Pio1Val {
        Pio1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Pio1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Pio1Val) -> Pio1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PIO2 Register
///
/// Control whether debugger, DMA, core 0 and core 1 can access PIO2, and at what security/privilege levels they can do so.
/// Defaults to Secure access from any master.
/// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
///
/// Address offset: 0x0054
/// Reset value: 0x000000fc
///
/// ## Fields
///
/// - **DBG** `[7]` (RW)
///
///   If 1, PIO2 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **DMA** `[6]` (RW)
///
///   If 1, PIO2 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE1** `[5]` (RW)
///
///   If 1, PIO2 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE0** `[4]` (RW)
///
///   If 1, PIO2 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **SP** `[3]` (RW)
///
///   If 1, PIO2 can be accessed from a Secure, Privileged context.
///
/// - **SU** `[2]` (RW)
///
///   If 1, and SP is also set, PIO2 can be accessed from a Secure, Unprivileged context.
///
/// - **NSP** `[1]` (RW)
///
///   If 1, PIO2 can be accessed from a Non-secure, Privileged context.
///
/// - **NSU** `[0]` (RW)
///
///   If 1, and NSP is also set, PIO2 can be accessed from a Non-secure, Unprivileged context.
///   This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
#[repr(transparent)]
pub struct Pio2 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Pio2 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Pio2 {
    // ======== Field Constants ========

    /// **DBG**
    ///
    /// If 1, PIO2 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const DBG_OFFSET: u32 = 7;
    pub const DBG_MASK: u32 = 0x00000080;
    pub const DBG: u32 = 1 << 7;

    /// **DMA**
    ///
    /// If 1, PIO2 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const DMA_OFFSET: u32 = 6;
    pub const DMA_MASK: u32 = 0x00000040;
    pub const DMA: u32 = 1 << 6;

    /// **CORE1**
    ///
    /// If 1, PIO2 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CORE1_OFFSET: u32 = 5;
    pub const CORE1_MASK: u32 = 0x00000020;
    pub const CORE1: u32 = 1 << 5;

    /// **CORE0**
    ///
    /// If 1, PIO2 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CORE0_OFFSET: u32 = 4;
    pub const CORE0_MASK: u32 = 0x00000010;
    pub const CORE0: u32 = 1 << 4;

    /// **SP**
    ///
    /// If 1, PIO2 can be accessed from a Secure, Privileged context.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SP_OFFSET: u32 = 3;
    pub const SP_MASK: u32 = 0x00000008;
    pub const SP: u32 = 1 << 3;

    /// **SU**
    ///
    /// If 1, and SP is also set, PIO2 can be accessed from a Secure, Unprivileged context.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SU_OFFSET: u32 = 2;
    pub const SU_MASK: u32 = 0x00000004;
    pub const SU: u32 = 1 << 2;

    /// **NSP**
    ///
    /// If 1, PIO2 can be accessed from a Non-secure, Privileged context.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const NSP_OFFSET: u32 = 1;
    pub const NSP_MASK: u32 = 0x00000002;
    pub const NSP: u32 = 1 << 1;

    /// **NSU**
    ///
    /// If 1, and NSP is also set, PIO2 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const NSU_OFFSET: u32 = 0;
    pub const NSU_MASK: u32 = 0x00000001;
    pub const NSU: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DBG** `[7]` (RW)
    ///
    /// If 1, PIO2 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dbg(&self) -> bool {
        (self.value.read_bits() & Self::DBG_MASK) != 0
    }

    /// Set DBG bit atomically (RW)
    #[inline(always)]
    pub fn set_dbg(&self) {
        self.set_bits(Self::DBG_MASK);
    }

    /// Clear DBG bit atomically (RW)
    #[inline(always)]
    pub fn clr_dbg(&self) {
        self.clr_bits(Self::DBG_MASK);
    }

    /// **DMA** `[6]` (RW)
    ///
    /// If 1, PIO2 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dma(&self) -> bool {
        (self.value.read_bits() & Self::DMA_MASK) != 0
    }

    /// Set DMA bit atomically (RW)
    #[inline(always)]
    pub fn set_dma(&self) {
        self.set_bits(Self::DMA_MASK);
    }

    /// Clear DMA bit atomically (RW)
    #[inline(always)]
    pub fn clr_dma(&self) {
        self.clr_bits(Self::DMA_MASK);
    }

    /// **CORE1** `[5]` (RW)
    ///
    /// If 1, PIO2 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core1(&self) -> bool {
        (self.value.read_bits() & Self::CORE1_MASK) != 0
    }

    /// Set CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn set_core1(&self) {
        self.set_bits(Self::CORE1_MASK);
    }

    /// Clear CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core1(&self) {
        self.clr_bits(Self::CORE1_MASK);
    }

    /// **CORE0** `[4]` (RW)
    ///
    /// If 1, PIO2 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core0(&self) -> bool {
        (self.value.read_bits() & Self::CORE0_MASK) != 0
    }

    /// Set CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn set_core0(&self) {
        self.set_bits(Self::CORE0_MASK);
    }

    /// Clear CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core0(&self) {
        self.clr_bits(Self::CORE0_MASK);
    }

    /// **SP** `[3]` (RW)
    ///
    /// If 1, PIO2 can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub fn sp(&self) -> bool {
        (self.value.read_bits() & Self::SP_MASK) != 0
    }

    /// Set SP bit atomically (RW)
    #[inline(always)]
    pub fn set_sp(&self) {
        self.set_bits(Self::SP_MASK);
    }

    /// Clear SP bit atomically (RW)
    #[inline(always)]
    pub fn clr_sp(&self) {
        self.clr_bits(Self::SP_MASK);
    }

    /// **SU** `[2]` (RW)
    ///
    /// If 1, and SP is also set, PIO2 can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub fn su(&self) -> bool {
        (self.value.read_bits() & Self::SU_MASK) != 0
    }

    /// Set SU bit atomically (RW)
    #[inline(always)]
    pub fn set_su(&self) {
        self.set_bits(Self::SU_MASK);
    }

    /// Clear SU bit atomically (RW)
    #[inline(always)]
    pub fn clr_su(&self) {
        self.clr_bits(Self::SU_MASK);
    }

    /// **NSP** `[1]` (RW)
    ///
    /// If 1, PIO2 can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub fn nsp(&self) -> bool {
        (self.value.read_bits() & Self::NSP_MASK) != 0
    }

    /// Set NSP bit atomically (RW)
    #[inline(always)]
    pub fn set_nsp(&self) {
        self.set_bits(Self::NSP_MASK);
    }

    /// Clear NSP bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsp(&self) {
        self.clr_bits(Self::NSP_MASK);
    }

    /// **NSU** `[0]` (RW)
    ///
    /// If 1, and NSP is also set, PIO2 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub fn nsu(&self) -> bool {
        (self.value.read_bits() & Self::NSU_MASK) != 0
    }

    /// Set NSU bit atomically (RW)
    #[inline(always)]
    pub fn set_nsu(&self) {
        self.set_bits(Self::NSU_MASK);
    }

    /// Clear NSU bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsu(&self) {
        self.clr_bits(Self::NSU_MASK);
    }
}

/// PIO2 value type for batch read/modify/write operations
///
/// Use `pio2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pio2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Pio2Val {
    bits: u32,
}

impl Pio2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x000000fc }
    }

    /// Read **DBG** `[7:7]`
    ///
    /// If 1, PIO2 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        (self.bits & Pio2::DBG_MASK) != 0
    }

    /// Read **DMA** `[6:6]`
    ///
    /// If 1, PIO2 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        (self.bits & Pio2::DMA_MASK) != 0
    }

    /// Read **CORE1** `[5:5]`
    ///
    /// If 1, PIO2 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        (self.bits & Pio2::CORE1_MASK) != 0
    }

    /// Read **CORE0** `[4:4]`
    ///
    /// If 1, PIO2 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        (self.bits & Pio2::CORE0_MASK) != 0
    }

    /// Read **SP** `[3:3]`
    ///
    /// If 1, PIO2 can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        (self.bits & Pio2::SP_MASK) != 0
    }

    /// Read **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, PIO2 can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn su(&self) -> bool {
        (self.bits & Pio2::SU_MASK) != 0
    }

    /// Read **NSP** `[1:1]`
    ///
    /// If 1, PIO2 can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        (self.bits & Pio2::NSP_MASK) != 0
    }

    /// Read **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, PIO2 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        (self.bits & Pio2::NSU_MASK) != 0
    }

    /// Set **DBG** `[7:7]`
    ///
    /// If 1, PIO2 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dbg(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pio2::DBG_MASK
            } else {
                self.bits & !Pio2::DBG_MASK
            }
        }
    }

    /// Set **DMA** `[6:6]`
    ///
    /// If 1, PIO2 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dma(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pio2::DMA_MASK
            } else {
                self.bits & !Pio2::DMA_MASK
            }
        }
    }

    /// Set **CORE1** `[5:5]`
    ///
    /// If 1, PIO2 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pio2::CORE1_MASK
            } else {
                self.bits & !Pio2::CORE1_MASK
            }
        }
    }

    /// Set **CORE0** `[4:4]`
    ///
    /// If 1, PIO2 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pio2::CORE0_MASK
            } else {
                self.bits & !Pio2::CORE0_MASK
            }
        }
    }

    /// Set **SP** `[3:3]`
    ///
    /// If 1, PIO2 can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn with_sp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pio2::SP_MASK
            } else {
                self.bits & !Pio2::SP_MASK
            }
        }
    }

    /// Set **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, PIO2 can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn with_su(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pio2::SU_MASK
            } else {
                self.bits & !Pio2::SU_MASK
            }
        }
    }

    /// Set **NSP** `[1:1]`
    ///
    /// If 1, PIO2 can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn with_nsp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pio2::NSP_MASK
            } else {
                self.bits & !Pio2::NSP_MASK
            }
        }
    }

    /// Set **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, PIO2 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn with_nsu(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pio2::NSU_MASK
            } else {
                self.bits & !Pio2::NSU_MASK
            }
        }
    }
}

impl Default for Pio2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Pio2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Pio2Val {
        Pio2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Pio2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Pio2Val) -> Pio2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CORESIGHT_TRACE Register
///
/// Control whether debugger, DMA, core 0 and core 1 can access CORESIGHT_TRACE, and at what security/privilege levels they can do so.
/// Defaults to Secure, Privileged processor or debug access only.
/// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
///
/// Address offset: 0x0058
/// Reset value: 0x000000b8
///
/// ## Fields
///
/// - **DBG** `[7]` (RW)
///
///   If 1, CORESIGHT_TRACE can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **DMA** `[6]` (RW)
///
///   If 1, CORESIGHT_TRACE can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE1** `[5]` (RW)
///
///   If 1, CORESIGHT_TRACE can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE0** `[4]` (RW)
///
///   If 1, CORESIGHT_TRACE can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **SP** `[3]` (RW)
///
///   If 1, CORESIGHT_TRACE can be accessed from a Secure, Privileged context.
///
/// - **SU** `[2]` (RW)
///
///   If 1, and SP is also set, CORESIGHT_TRACE can be accessed from a Secure, Unprivileged context.
///
/// - **NSP** `[1]` (RW)
///
///   If 1, CORESIGHT_TRACE can be accessed from a Non-secure, Privileged context.
///
/// - **NSU** `[0]` (RW)
///
///   If 1, and NSP is also set, CORESIGHT_TRACE can be accessed from a Non-secure, Unprivileged context.
///   This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
#[repr(transparent)]
pub struct CoresightTrace {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for CoresightTrace {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CoresightTrace {
    // ======== Field Constants ========

    /// **DBG**
    ///
    /// If 1, CORESIGHT_TRACE can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const DBG_OFFSET: u32 = 7;
    pub const DBG_MASK: u32 = 0x00000080;
    pub const DBG: u32 = 1 << 7;

    /// **DMA**
    ///
    /// If 1, CORESIGHT_TRACE can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const DMA_OFFSET: u32 = 6;
    pub const DMA_MASK: u32 = 0x00000040;
    pub const DMA: u32 = 1 << 6;

    /// **CORE1**
    ///
    /// If 1, CORESIGHT_TRACE can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CORE1_OFFSET: u32 = 5;
    pub const CORE1_MASK: u32 = 0x00000020;
    pub const CORE1: u32 = 1 << 5;

    /// **CORE0**
    ///
    /// If 1, CORESIGHT_TRACE can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CORE0_OFFSET: u32 = 4;
    pub const CORE0_MASK: u32 = 0x00000010;
    pub const CORE0: u32 = 1 << 4;

    /// **SP**
    ///
    /// If 1, CORESIGHT_TRACE can be accessed from a Secure, Privileged context.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SP_OFFSET: u32 = 3;
    pub const SP_MASK: u32 = 0x00000008;
    pub const SP: u32 = 1 << 3;

    /// **SU**
    ///
    /// If 1, and SP is also set, CORESIGHT_TRACE can be accessed from a Secure, Unprivileged context.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SU_OFFSET: u32 = 2;
    pub const SU_MASK: u32 = 0x00000004;
    pub const SU: u32 = 1 << 2;

    /// **NSP**
    ///
    /// If 1, CORESIGHT_TRACE can be accessed from a Non-secure, Privileged context.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const NSP_OFFSET: u32 = 1;
    pub const NSP_MASK: u32 = 0x00000002;
    pub const NSP: u32 = 1 << 1;

    /// **NSU**
    ///
    /// If 1, and NSP is also set, CORESIGHT_TRACE can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const NSU_OFFSET: u32 = 0;
    pub const NSU_MASK: u32 = 0x00000001;
    pub const NSU: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DBG** `[7]` (RW)
    ///
    /// If 1, CORESIGHT_TRACE can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dbg(&self) -> bool {
        (self.value.read_bits() & Self::DBG_MASK) != 0
    }

    /// Set DBG bit atomically (RW)
    #[inline(always)]
    pub fn set_dbg(&self) {
        self.set_bits(Self::DBG_MASK);
    }

    /// Clear DBG bit atomically (RW)
    #[inline(always)]
    pub fn clr_dbg(&self) {
        self.clr_bits(Self::DBG_MASK);
    }

    /// **DMA** `[6]` (RW)
    ///
    /// If 1, CORESIGHT_TRACE can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dma(&self) -> bool {
        (self.value.read_bits() & Self::DMA_MASK) != 0
    }

    /// Set DMA bit atomically (RW)
    #[inline(always)]
    pub fn set_dma(&self) {
        self.set_bits(Self::DMA_MASK);
    }

    /// Clear DMA bit atomically (RW)
    #[inline(always)]
    pub fn clr_dma(&self) {
        self.clr_bits(Self::DMA_MASK);
    }

    /// **CORE1** `[5]` (RW)
    ///
    /// If 1, CORESIGHT_TRACE can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core1(&self) -> bool {
        (self.value.read_bits() & Self::CORE1_MASK) != 0
    }

    /// Set CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn set_core1(&self) {
        self.set_bits(Self::CORE1_MASK);
    }

    /// Clear CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core1(&self) {
        self.clr_bits(Self::CORE1_MASK);
    }

    /// **CORE0** `[4]` (RW)
    ///
    /// If 1, CORESIGHT_TRACE can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core0(&self) -> bool {
        (self.value.read_bits() & Self::CORE0_MASK) != 0
    }

    /// Set CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn set_core0(&self) {
        self.set_bits(Self::CORE0_MASK);
    }

    /// Clear CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core0(&self) {
        self.clr_bits(Self::CORE0_MASK);
    }

    /// **SP** `[3]` (RW)
    ///
    /// If 1, CORESIGHT_TRACE can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub fn sp(&self) -> bool {
        (self.value.read_bits() & Self::SP_MASK) != 0
    }

    /// Set SP bit atomically (RW)
    #[inline(always)]
    pub fn set_sp(&self) {
        self.set_bits(Self::SP_MASK);
    }

    /// Clear SP bit atomically (RW)
    #[inline(always)]
    pub fn clr_sp(&self) {
        self.clr_bits(Self::SP_MASK);
    }

    /// **SU** `[2]` (RW)
    ///
    /// If 1, and SP is also set, CORESIGHT_TRACE can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub fn su(&self) -> bool {
        (self.value.read_bits() & Self::SU_MASK) != 0
    }

    /// Set SU bit atomically (RW)
    #[inline(always)]
    pub fn set_su(&self) {
        self.set_bits(Self::SU_MASK);
    }

    /// Clear SU bit atomically (RW)
    #[inline(always)]
    pub fn clr_su(&self) {
        self.clr_bits(Self::SU_MASK);
    }

    /// **NSP** `[1]` (RW)
    ///
    /// If 1, CORESIGHT_TRACE can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub fn nsp(&self) -> bool {
        (self.value.read_bits() & Self::NSP_MASK) != 0
    }

    /// Set NSP bit atomically (RW)
    #[inline(always)]
    pub fn set_nsp(&self) {
        self.set_bits(Self::NSP_MASK);
    }

    /// Clear NSP bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsp(&self) {
        self.clr_bits(Self::NSP_MASK);
    }

    /// **NSU** `[0]` (RW)
    ///
    /// If 1, and NSP is also set, CORESIGHT_TRACE can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub fn nsu(&self) -> bool {
        (self.value.read_bits() & Self::NSU_MASK) != 0
    }

    /// Set NSU bit atomically (RW)
    #[inline(always)]
    pub fn set_nsu(&self) {
        self.set_bits(Self::NSU_MASK);
    }

    /// Clear NSU bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsu(&self) {
        self.clr_bits(Self::NSU_MASK);
    }
}

/// CORESIGHT_TRACE value type for batch read/modify/write operations
///
/// Use `coresighttrace.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `coresighttrace.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CoresightTraceVal {
    bits: u32,
}

impl CoresightTraceVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x000000b8 }
    }

    /// Read **DBG** `[7:7]`
    ///
    /// If 1, CORESIGHT_TRACE can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        (self.bits & CoresightTrace::DBG_MASK) != 0
    }

    /// Read **DMA** `[6:6]`
    ///
    /// If 1, CORESIGHT_TRACE can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        (self.bits & CoresightTrace::DMA_MASK) != 0
    }

    /// Read **CORE1** `[5:5]`
    ///
    /// If 1, CORESIGHT_TRACE can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        (self.bits & CoresightTrace::CORE1_MASK) != 0
    }

    /// Read **CORE0** `[4:4]`
    ///
    /// If 1, CORESIGHT_TRACE can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        (self.bits & CoresightTrace::CORE0_MASK) != 0
    }

    /// Read **SP** `[3:3]`
    ///
    /// If 1, CORESIGHT_TRACE can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        (self.bits & CoresightTrace::SP_MASK) != 0
    }

    /// Read **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, CORESIGHT_TRACE can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn su(&self) -> bool {
        (self.bits & CoresightTrace::SU_MASK) != 0
    }

    /// Read **NSP** `[1:1]`
    ///
    /// If 1, CORESIGHT_TRACE can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        (self.bits & CoresightTrace::NSP_MASK) != 0
    }

    /// Read **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, CORESIGHT_TRACE can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        (self.bits & CoresightTrace::NSU_MASK) != 0
    }

    /// Set **DBG** `[7:7]`
    ///
    /// If 1, CORESIGHT_TRACE can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dbg(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CoresightTrace::DBG_MASK
            } else {
                self.bits & !CoresightTrace::DBG_MASK
            }
        }
    }

    /// Set **DMA** `[6:6]`
    ///
    /// If 1, CORESIGHT_TRACE can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dma(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CoresightTrace::DMA_MASK
            } else {
                self.bits & !CoresightTrace::DMA_MASK
            }
        }
    }

    /// Set **CORE1** `[5:5]`
    ///
    /// If 1, CORESIGHT_TRACE can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CoresightTrace::CORE1_MASK
            } else {
                self.bits & !CoresightTrace::CORE1_MASK
            }
        }
    }

    /// Set **CORE0** `[4:4]`
    ///
    /// If 1, CORESIGHT_TRACE can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CoresightTrace::CORE0_MASK
            } else {
                self.bits & !CoresightTrace::CORE0_MASK
            }
        }
    }

    /// Set **SP** `[3:3]`
    ///
    /// If 1, CORESIGHT_TRACE can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn with_sp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CoresightTrace::SP_MASK
            } else {
                self.bits & !CoresightTrace::SP_MASK
            }
        }
    }

    /// Set **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, CORESIGHT_TRACE can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn with_su(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CoresightTrace::SU_MASK
            } else {
                self.bits & !CoresightTrace::SU_MASK
            }
        }
    }

    /// Set **NSP** `[1:1]`
    ///
    /// If 1, CORESIGHT_TRACE can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn with_nsp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CoresightTrace::NSP_MASK
            } else {
                self.bits & !CoresightTrace::NSP_MASK
            }
        }
    }

    /// Set **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, CORESIGHT_TRACE can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn with_nsu(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CoresightTrace::NSU_MASK
            } else {
                self.bits & !CoresightTrace::NSU_MASK
            }
        }
    }
}

impl Default for CoresightTraceVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl CoresightTrace {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CoresightTraceVal {
        CoresightTraceVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CoresightTraceVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CoresightTraceVal) -> CoresightTraceVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CORESIGHT_PERIPH Register
///
/// Control whether debugger, DMA, core 0 and core 1 can access CORESIGHT_PERIPH, and at what security/privilege levels they can do so.
/// Defaults to Secure, Privileged processor or debug access only.
/// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
///
/// Address offset: 0x005c
/// Reset value: 0x000000b8
///
/// ## Fields
///
/// - **DBG** `[7]` (RW)
///
///   If 1, CORESIGHT_PERIPH can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **DMA** `[6]` (RW)
///
///   If 1, CORESIGHT_PERIPH can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE1** `[5]` (RW)
///
///   If 1, CORESIGHT_PERIPH can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE0** `[4]` (RW)
///
///   If 1, CORESIGHT_PERIPH can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **SP** `[3]` (RW)
///
///   If 1, CORESIGHT_PERIPH can be accessed from a Secure, Privileged context.
///
/// - **SU** `[2]` (RW)
///
///   If 1, and SP is also set, CORESIGHT_PERIPH can be accessed from a Secure, Unprivileged context.
///
/// - **NSP** `[1]` (RW)
///
///   If 1, CORESIGHT_PERIPH can be accessed from a Non-secure, Privileged context.
///
/// - **NSU** `[0]` (RW)
///
///   If 1, and NSP is also set, CORESIGHT_PERIPH can be accessed from a Non-secure, Unprivileged context.
///   This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
#[repr(transparent)]
pub struct CoresightPeriph {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for CoresightPeriph {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CoresightPeriph {
    // ======== Field Constants ========

    /// **DBG**
    ///
    /// If 1, CORESIGHT_PERIPH can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const DBG_OFFSET: u32 = 7;
    pub const DBG_MASK: u32 = 0x00000080;
    pub const DBG: u32 = 1 << 7;

    /// **DMA**
    ///
    /// If 1, CORESIGHT_PERIPH can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const DMA_OFFSET: u32 = 6;
    pub const DMA_MASK: u32 = 0x00000040;
    pub const DMA: u32 = 1 << 6;

    /// **CORE1**
    ///
    /// If 1, CORESIGHT_PERIPH can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CORE1_OFFSET: u32 = 5;
    pub const CORE1_MASK: u32 = 0x00000020;
    pub const CORE1: u32 = 1 << 5;

    /// **CORE0**
    ///
    /// If 1, CORESIGHT_PERIPH can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CORE0_OFFSET: u32 = 4;
    pub const CORE0_MASK: u32 = 0x00000010;
    pub const CORE0: u32 = 1 << 4;

    /// **SP**
    ///
    /// If 1, CORESIGHT_PERIPH can be accessed from a Secure, Privileged context.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SP_OFFSET: u32 = 3;
    pub const SP_MASK: u32 = 0x00000008;
    pub const SP: u32 = 1 << 3;

    /// **SU**
    ///
    /// If 1, and SP is also set, CORESIGHT_PERIPH can be accessed from a Secure, Unprivileged context.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SU_OFFSET: u32 = 2;
    pub const SU_MASK: u32 = 0x00000004;
    pub const SU: u32 = 1 << 2;

    /// **NSP**
    ///
    /// If 1, CORESIGHT_PERIPH can be accessed from a Non-secure, Privileged context.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const NSP_OFFSET: u32 = 1;
    pub const NSP_MASK: u32 = 0x00000002;
    pub const NSP: u32 = 1 << 1;

    /// **NSU**
    ///
    /// If 1, and NSP is also set, CORESIGHT_PERIPH can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const NSU_OFFSET: u32 = 0;
    pub const NSU_MASK: u32 = 0x00000001;
    pub const NSU: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DBG** `[7]` (RW)
    ///
    /// If 1, CORESIGHT_PERIPH can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dbg(&self) -> bool {
        (self.value.read_bits() & Self::DBG_MASK) != 0
    }

    /// Set DBG bit atomically (RW)
    #[inline(always)]
    pub fn set_dbg(&self) {
        self.set_bits(Self::DBG_MASK);
    }

    /// Clear DBG bit atomically (RW)
    #[inline(always)]
    pub fn clr_dbg(&self) {
        self.clr_bits(Self::DBG_MASK);
    }

    /// **DMA** `[6]` (RW)
    ///
    /// If 1, CORESIGHT_PERIPH can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dma(&self) -> bool {
        (self.value.read_bits() & Self::DMA_MASK) != 0
    }

    /// Set DMA bit atomically (RW)
    #[inline(always)]
    pub fn set_dma(&self) {
        self.set_bits(Self::DMA_MASK);
    }

    /// Clear DMA bit atomically (RW)
    #[inline(always)]
    pub fn clr_dma(&self) {
        self.clr_bits(Self::DMA_MASK);
    }

    /// **CORE1** `[5]` (RW)
    ///
    /// If 1, CORESIGHT_PERIPH can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core1(&self) -> bool {
        (self.value.read_bits() & Self::CORE1_MASK) != 0
    }

    /// Set CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn set_core1(&self) {
        self.set_bits(Self::CORE1_MASK);
    }

    /// Clear CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core1(&self) {
        self.clr_bits(Self::CORE1_MASK);
    }

    /// **CORE0** `[4]` (RW)
    ///
    /// If 1, CORESIGHT_PERIPH can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core0(&self) -> bool {
        (self.value.read_bits() & Self::CORE0_MASK) != 0
    }

    /// Set CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn set_core0(&self) {
        self.set_bits(Self::CORE0_MASK);
    }

    /// Clear CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core0(&self) {
        self.clr_bits(Self::CORE0_MASK);
    }

    /// **SP** `[3]` (RW)
    ///
    /// If 1, CORESIGHT_PERIPH can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub fn sp(&self) -> bool {
        (self.value.read_bits() & Self::SP_MASK) != 0
    }

    /// Set SP bit atomically (RW)
    #[inline(always)]
    pub fn set_sp(&self) {
        self.set_bits(Self::SP_MASK);
    }

    /// Clear SP bit atomically (RW)
    #[inline(always)]
    pub fn clr_sp(&self) {
        self.clr_bits(Self::SP_MASK);
    }

    /// **SU** `[2]` (RW)
    ///
    /// If 1, and SP is also set, CORESIGHT_PERIPH can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub fn su(&self) -> bool {
        (self.value.read_bits() & Self::SU_MASK) != 0
    }

    /// Set SU bit atomically (RW)
    #[inline(always)]
    pub fn set_su(&self) {
        self.set_bits(Self::SU_MASK);
    }

    /// Clear SU bit atomically (RW)
    #[inline(always)]
    pub fn clr_su(&self) {
        self.clr_bits(Self::SU_MASK);
    }

    /// **NSP** `[1]` (RW)
    ///
    /// If 1, CORESIGHT_PERIPH can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub fn nsp(&self) -> bool {
        (self.value.read_bits() & Self::NSP_MASK) != 0
    }

    /// Set NSP bit atomically (RW)
    #[inline(always)]
    pub fn set_nsp(&self) {
        self.set_bits(Self::NSP_MASK);
    }

    /// Clear NSP bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsp(&self) {
        self.clr_bits(Self::NSP_MASK);
    }

    /// **NSU** `[0]` (RW)
    ///
    /// If 1, and NSP is also set, CORESIGHT_PERIPH can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub fn nsu(&self) -> bool {
        (self.value.read_bits() & Self::NSU_MASK) != 0
    }

    /// Set NSU bit atomically (RW)
    #[inline(always)]
    pub fn set_nsu(&self) {
        self.set_bits(Self::NSU_MASK);
    }

    /// Clear NSU bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsu(&self) {
        self.clr_bits(Self::NSU_MASK);
    }
}

/// CORESIGHT_PERIPH value type for batch read/modify/write operations
///
/// Use `coresightperiph.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `coresightperiph.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CoresightPeriphVal {
    bits: u32,
}

impl CoresightPeriphVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x000000b8 }
    }

    /// Read **DBG** `[7:7]`
    ///
    /// If 1, CORESIGHT_PERIPH can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        (self.bits & CoresightPeriph::DBG_MASK) != 0
    }

    /// Read **DMA** `[6:6]`
    ///
    /// If 1, CORESIGHT_PERIPH can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        (self.bits & CoresightPeriph::DMA_MASK) != 0
    }

    /// Read **CORE1** `[5:5]`
    ///
    /// If 1, CORESIGHT_PERIPH can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        (self.bits & CoresightPeriph::CORE1_MASK) != 0
    }

    /// Read **CORE0** `[4:4]`
    ///
    /// If 1, CORESIGHT_PERIPH can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        (self.bits & CoresightPeriph::CORE0_MASK) != 0
    }

    /// Read **SP** `[3:3]`
    ///
    /// If 1, CORESIGHT_PERIPH can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        (self.bits & CoresightPeriph::SP_MASK) != 0
    }

    /// Read **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, CORESIGHT_PERIPH can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn su(&self) -> bool {
        (self.bits & CoresightPeriph::SU_MASK) != 0
    }

    /// Read **NSP** `[1:1]`
    ///
    /// If 1, CORESIGHT_PERIPH can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        (self.bits & CoresightPeriph::NSP_MASK) != 0
    }

    /// Read **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, CORESIGHT_PERIPH can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        (self.bits & CoresightPeriph::NSU_MASK) != 0
    }

    /// Set **DBG** `[7:7]`
    ///
    /// If 1, CORESIGHT_PERIPH can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dbg(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CoresightPeriph::DBG_MASK
            } else {
                self.bits & !CoresightPeriph::DBG_MASK
            }
        }
    }

    /// Set **DMA** `[6:6]`
    ///
    /// If 1, CORESIGHT_PERIPH can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dma(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CoresightPeriph::DMA_MASK
            } else {
                self.bits & !CoresightPeriph::DMA_MASK
            }
        }
    }

    /// Set **CORE1** `[5:5]`
    ///
    /// If 1, CORESIGHT_PERIPH can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CoresightPeriph::CORE1_MASK
            } else {
                self.bits & !CoresightPeriph::CORE1_MASK
            }
        }
    }

    /// Set **CORE0** `[4:4]`
    ///
    /// If 1, CORESIGHT_PERIPH can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CoresightPeriph::CORE0_MASK
            } else {
                self.bits & !CoresightPeriph::CORE0_MASK
            }
        }
    }

    /// Set **SP** `[3:3]`
    ///
    /// If 1, CORESIGHT_PERIPH can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn with_sp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CoresightPeriph::SP_MASK
            } else {
                self.bits & !CoresightPeriph::SP_MASK
            }
        }
    }

    /// Set **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, CORESIGHT_PERIPH can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn with_su(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CoresightPeriph::SU_MASK
            } else {
                self.bits & !CoresightPeriph::SU_MASK
            }
        }
    }

    /// Set **NSP** `[1:1]`
    ///
    /// If 1, CORESIGHT_PERIPH can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn with_nsp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CoresightPeriph::NSP_MASK
            } else {
                self.bits & !CoresightPeriph::NSP_MASK
            }
        }
    }

    /// Set **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, CORESIGHT_PERIPH can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn with_nsu(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CoresightPeriph::NSU_MASK
            } else {
                self.bits & !CoresightPeriph::NSU_MASK
            }
        }
    }
}

impl Default for CoresightPeriphVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl CoresightPeriph {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CoresightPeriphVal {
        CoresightPeriphVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CoresightPeriphVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CoresightPeriphVal) -> CoresightPeriphVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SYSINFO Register
///
/// Control whether debugger, DMA, core 0 and core 1 can access SYSINFO, and at what security/privilege levels they can do so.
/// Defaults to fully open access.
/// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
///
/// Address offset: 0x0060
/// Reset value: 0x000000ff
///
/// ## Fields
///
/// - **DBG** `[7]` (RW)
///
///   If 1, SYSINFO can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **DMA** `[6]` (RW)
///
///   If 1, SYSINFO can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE1** `[5]` (RW)
///
///   If 1, SYSINFO can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE0** `[4]` (RW)
///
///   If 1, SYSINFO can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **SP** `[3]` (RW)
///
///   If 1, SYSINFO can be accessed from a Secure, Privileged context.
///
/// - **SU** `[2]` (RW)
///
///   If 1, and SP is also set, SYSINFO can be accessed from a Secure, Unprivileged context.
///
/// - **NSP** `[1]` (RW)
///
///   If 1, SYSINFO can be accessed from a Non-secure, Privileged context.
///
/// - **NSU** `[0]` (RW)
///
///   If 1, and NSP is also set, SYSINFO can be accessed from a Non-secure, Unprivileged context.
///   This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
#[repr(transparent)]
pub struct Sysinfo {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Sysinfo {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sysinfo {
    // ======== Field Constants ========

    /// **DBG**
    ///
    /// If 1, SYSINFO can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const DBG_OFFSET: u32 = 7;
    pub const DBG_MASK: u32 = 0x00000080;
    pub const DBG: u32 = 1 << 7;

    /// **DMA**
    ///
    /// If 1, SYSINFO can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const DMA_OFFSET: u32 = 6;
    pub const DMA_MASK: u32 = 0x00000040;
    pub const DMA: u32 = 1 << 6;

    /// **CORE1**
    ///
    /// If 1, SYSINFO can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CORE1_OFFSET: u32 = 5;
    pub const CORE1_MASK: u32 = 0x00000020;
    pub const CORE1: u32 = 1 << 5;

    /// **CORE0**
    ///
    /// If 1, SYSINFO can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CORE0_OFFSET: u32 = 4;
    pub const CORE0_MASK: u32 = 0x00000010;
    pub const CORE0: u32 = 1 << 4;

    /// **SP**
    ///
    /// If 1, SYSINFO can be accessed from a Secure, Privileged context.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SP_OFFSET: u32 = 3;
    pub const SP_MASK: u32 = 0x00000008;
    pub const SP: u32 = 1 << 3;

    /// **SU**
    ///
    /// If 1, and SP is also set, SYSINFO can be accessed from a Secure, Unprivileged context.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SU_OFFSET: u32 = 2;
    pub const SU_MASK: u32 = 0x00000004;
    pub const SU: u32 = 1 << 2;

    /// **NSP**
    ///
    /// If 1, SYSINFO can be accessed from a Non-secure, Privileged context.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const NSP_OFFSET: u32 = 1;
    pub const NSP_MASK: u32 = 0x00000002;
    pub const NSP: u32 = 1 << 1;

    /// **NSU**
    ///
    /// If 1, and NSP is also set, SYSINFO can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const NSU_OFFSET: u32 = 0;
    pub const NSU_MASK: u32 = 0x00000001;
    pub const NSU: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DBG** `[7]` (RW)
    ///
    /// If 1, SYSINFO can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dbg(&self) -> bool {
        (self.value.read_bits() & Self::DBG_MASK) != 0
    }

    /// Set DBG bit atomically (RW)
    #[inline(always)]
    pub fn set_dbg(&self) {
        self.set_bits(Self::DBG_MASK);
    }

    /// Clear DBG bit atomically (RW)
    #[inline(always)]
    pub fn clr_dbg(&self) {
        self.clr_bits(Self::DBG_MASK);
    }

    /// **DMA** `[6]` (RW)
    ///
    /// If 1, SYSINFO can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dma(&self) -> bool {
        (self.value.read_bits() & Self::DMA_MASK) != 0
    }

    /// Set DMA bit atomically (RW)
    #[inline(always)]
    pub fn set_dma(&self) {
        self.set_bits(Self::DMA_MASK);
    }

    /// Clear DMA bit atomically (RW)
    #[inline(always)]
    pub fn clr_dma(&self) {
        self.clr_bits(Self::DMA_MASK);
    }

    /// **CORE1** `[5]` (RW)
    ///
    /// If 1, SYSINFO can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core1(&self) -> bool {
        (self.value.read_bits() & Self::CORE1_MASK) != 0
    }

    /// Set CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn set_core1(&self) {
        self.set_bits(Self::CORE1_MASK);
    }

    /// Clear CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core1(&self) {
        self.clr_bits(Self::CORE1_MASK);
    }

    /// **CORE0** `[4]` (RW)
    ///
    /// If 1, SYSINFO can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core0(&self) -> bool {
        (self.value.read_bits() & Self::CORE0_MASK) != 0
    }

    /// Set CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn set_core0(&self) {
        self.set_bits(Self::CORE0_MASK);
    }

    /// Clear CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core0(&self) {
        self.clr_bits(Self::CORE0_MASK);
    }

    /// **SP** `[3]` (RW)
    ///
    /// If 1, SYSINFO can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub fn sp(&self) -> bool {
        (self.value.read_bits() & Self::SP_MASK) != 0
    }

    /// Set SP bit atomically (RW)
    #[inline(always)]
    pub fn set_sp(&self) {
        self.set_bits(Self::SP_MASK);
    }

    /// Clear SP bit atomically (RW)
    #[inline(always)]
    pub fn clr_sp(&self) {
        self.clr_bits(Self::SP_MASK);
    }

    /// **SU** `[2]` (RW)
    ///
    /// If 1, and SP is also set, SYSINFO can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub fn su(&self) -> bool {
        (self.value.read_bits() & Self::SU_MASK) != 0
    }

    /// Set SU bit atomically (RW)
    #[inline(always)]
    pub fn set_su(&self) {
        self.set_bits(Self::SU_MASK);
    }

    /// Clear SU bit atomically (RW)
    #[inline(always)]
    pub fn clr_su(&self) {
        self.clr_bits(Self::SU_MASK);
    }

    /// **NSP** `[1]` (RW)
    ///
    /// If 1, SYSINFO can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub fn nsp(&self) -> bool {
        (self.value.read_bits() & Self::NSP_MASK) != 0
    }

    /// Set NSP bit atomically (RW)
    #[inline(always)]
    pub fn set_nsp(&self) {
        self.set_bits(Self::NSP_MASK);
    }

    /// Clear NSP bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsp(&self) {
        self.clr_bits(Self::NSP_MASK);
    }

    /// **NSU** `[0]` (RW)
    ///
    /// If 1, and NSP is also set, SYSINFO can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub fn nsu(&self) -> bool {
        (self.value.read_bits() & Self::NSU_MASK) != 0
    }

    /// Set NSU bit atomically (RW)
    #[inline(always)]
    pub fn set_nsu(&self) {
        self.set_bits(Self::NSU_MASK);
    }

    /// Clear NSU bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsu(&self) {
        self.clr_bits(Self::NSU_MASK);
    }
}

/// SYSINFO value type for batch read/modify/write operations
///
/// Use `sysinfo.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sysinfo.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SysinfoVal {
    bits: u32,
}

impl SysinfoVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
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

    /// Read **DBG** `[7:7]`
    ///
    /// If 1, SYSINFO can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        (self.bits & Sysinfo::DBG_MASK) != 0
    }

    /// Read **DMA** `[6:6]`
    ///
    /// If 1, SYSINFO can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        (self.bits & Sysinfo::DMA_MASK) != 0
    }

    /// Read **CORE1** `[5:5]`
    ///
    /// If 1, SYSINFO can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        (self.bits & Sysinfo::CORE1_MASK) != 0
    }

    /// Read **CORE0** `[4:4]`
    ///
    /// If 1, SYSINFO can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        (self.bits & Sysinfo::CORE0_MASK) != 0
    }

    /// Read **SP** `[3:3]`
    ///
    /// If 1, SYSINFO can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        (self.bits & Sysinfo::SP_MASK) != 0
    }

    /// Read **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, SYSINFO can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn su(&self) -> bool {
        (self.bits & Sysinfo::SU_MASK) != 0
    }

    /// Read **NSP** `[1:1]`
    ///
    /// If 1, SYSINFO can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        (self.bits & Sysinfo::NSP_MASK) != 0
    }

    /// Read **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, SYSINFO can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        (self.bits & Sysinfo::NSU_MASK) != 0
    }

    /// Set **DBG** `[7:7]`
    ///
    /// If 1, SYSINFO can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dbg(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sysinfo::DBG_MASK
            } else {
                self.bits & !Sysinfo::DBG_MASK
            }
        }
    }

    /// Set **DMA** `[6:6]`
    ///
    /// If 1, SYSINFO can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dma(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sysinfo::DMA_MASK
            } else {
                self.bits & !Sysinfo::DMA_MASK
            }
        }
    }

    /// Set **CORE1** `[5:5]`
    ///
    /// If 1, SYSINFO can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sysinfo::CORE1_MASK
            } else {
                self.bits & !Sysinfo::CORE1_MASK
            }
        }
    }

    /// Set **CORE0** `[4:4]`
    ///
    /// If 1, SYSINFO can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sysinfo::CORE0_MASK
            } else {
                self.bits & !Sysinfo::CORE0_MASK
            }
        }
    }

    /// Set **SP** `[3:3]`
    ///
    /// If 1, SYSINFO can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn with_sp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sysinfo::SP_MASK
            } else {
                self.bits & !Sysinfo::SP_MASK
            }
        }
    }

    /// Set **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, SYSINFO can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn with_su(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sysinfo::SU_MASK
            } else {
                self.bits & !Sysinfo::SU_MASK
            }
        }
    }

    /// Set **NSP** `[1:1]`
    ///
    /// If 1, SYSINFO can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn with_nsp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sysinfo::NSP_MASK
            } else {
                self.bits & !Sysinfo::NSP_MASK
            }
        }
    }

    /// Set **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, SYSINFO can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn with_nsu(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sysinfo::NSU_MASK
            } else {
                self.bits & !Sysinfo::NSU_MASK
            }
        }
    }
}

impl Default for SysinfoVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Sysinfo {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SysinfoVal {
        SysinfoVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SysinfoVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SysinfoVal) -> SysinfoVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RESETS Register
///
/// Control whether debugger, DMA, core 0 and core 1 can access RESETS, and at what security/privilege levels they can do so.
/// Defaults to Secure access from any master.
/// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
///
/// Address offset: 0x0064
/// Reset value: 0x000000fc
///
/// ## Fields
///
/// - **DBG** `[7]` (RW)
///
///   If 1, RESETS can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **DMA** `[6]` (RW)
///
///   If 1, RESETS can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE1** `[5]` (RW)
///
///   If 1, RESETS can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE0** `[4]` (RW)
///
///   If 1, RESETS can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **SP** `[3]` (RW)
///
///   If 1, RESETS can be accessed from a Secure, Privileged context.
///
/// - **SU** `[2]` (RW)
///
///   If 1, and SP is also set, RESETS can be accessed from a Secure, Unprivileged context.
///
/// - **NSP** `[1]` (RW)
///
///   If 1, RESETS can be accessed from a Non-secure, Privileged context.
///
/// - **NSU** `[0]` (RW)
///
///   If 1, and NSP is also set, RESETS can be accessed from a Non-secure, Unprivileged context.
///   This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
#[repr(transparent)]
pub struct Resets {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Resets {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Resets {
    // ======== Field Constants ========

    /// **DBG**
    ///
    /// If 1, RESETS can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const DBG_OFFSET: u32 = 7;
    pub const DBG_MASK: u32 = 0x00000080;
    pub const DBG: u32 = 1 << 7;

    /// **DMA**
    ///
    /// If 1, RESETS can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const DMA_OFFSET: u32 = 6;
    pub const DMA_MASK: u32 = 0x00000040;
    pub const DMA: u32 = 1 << 6;

    /// **CORE1**
    ///
    /// If 1, RESETS can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CORE1_OFFSET: u32 = 5;
    pub const CORE1_MASK: u32 = 0x00000020;
    pub const CORE1: u32 = 1 << 5;

    /// **CORE0**
    ///
    /// If 1, RESETS can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CORE0_OFFSET: u32 = 4;
    pub const CORE0_MASK: u32 = 0x00000010;
    pub const CORE0: u32 = 1 << 4;

    /// **SP**
    ///
    /// If 1, RESETS can be accessed from a Secure, Privileged context.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SP_OFFSET: u32 = 3;
    pub const SP_MASK: u32 = 0x00000008;
    pub const SP: u32 = 1 << 3;

    /// **SU**
    ///
    /// If 1, and SP is also set, RESETS can be accessed from a Secure, Unprivileged context.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SU_OFFSET: u32 = 2;
    pub const SU_MASK: u32 = 0x00000004;
    pub const SU: u32 = 1 << 2;

    /// **NSP**
    ///
    /// If 1, RESETS can be accessed from a Non-secure, Privileged context.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const NSP_OFFSET: u32 = 1;
    pub const NSP_MASK: u32 = 0x00000002;
    pub const NSP: u32 = 1 << 1;

    /// **NSU**
    ///
    /// If 1, and NSP is also set, RESETS can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const NSU_OFFSET: u32 = 0;
    pub const NSU_MASK: u32 = 0x00000001;
    pub const NSU: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DBG** `[7]` (RW)
    ///
    /// If 1, RESETS can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dbg(&self) -> bool {
        (self.value.read_bits() & Self::DBG_MASK) != 0
    }

    /// Set DBG bit atomically (RW)
    #[inline(always)]
    pub fn set_dbg(&self) {
        self.set_bits(Self::DBG_MASK);
    }

    /// Clear DBG bit atomically (RW)
    #[inline(always)]
    pub fn clr_dbg(&self) {
        self.clr_bits(Self::DBG_MASK);
    }

    /// **DMA** `[6]` (RW)
    ///
    /// If 1, RESETS can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dma(&self) -> bool {
        (self.value.read_bits() & Self::DMA_MASK) != 0
    }

    /// Set DMA bit atomically (RW)
    #[inline(always)]
    pub fn set_dma(&self) {
        self.set_bits(Self::DMA_MASK);
    }

    /// Clear DMA bit atomically (RW)
    #[inline(always)]
    pub fn clr_dma(&self) {
        self.clr_bits(Self::DMA_MASK);
    }

    /// **CORE1** `[5]` (RW)
    ///
    /// If 1, RESETS can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core1(&self) -> bool {
        (self.value.read_bits() & Self::CORE1_MASK) != 0
    }

    /// Set CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn set_core1(&self) {
        self.set_bits(Self::CORE1_MASK);
    }

    /// Clear CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core1(&self) {
        self.clr_bits(Self::CORE1_MASK);
    }

    /// **CORE0** `[4]` (RW)
    ///
    /// If 1, RESETS can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core0(&self) -> bool {
        (self.value.read_bits() & Self::CORE0_MASK) != 0
    }

    /// Set CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn set_core0(&self) {
        self.set_bits(Self::CORE0_MASK);
    }

    /// Clear CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core0(&self) {
        self.clr_bits(Self::CORE0_MASK);
    }

    /// **SP** `[3]` (RW)
    ///
    /// If 1, RESETS can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub fn sp(&self) -> bool {
        (self.value.read_bits() & Self::SP_MASK) != 0
    }

    /// Set SP bit atomically (RW)
    #[inline(always)]
    pub fn set_sp(&self) {
        self.set_bits(Self::SP_MASK);
    }

    /// Clear SP bit atomically (RW)
    #[inline(always)]
    pub fn clr_sp(&self) {
        self.clr_bits(Self::SP_MASK);
    }

    /// **SU** `[2]` (RW)
    ///
    /// If 1, and SP is also set, RESETS can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub fn su(&self) -> bool {
        (self.value.read_bits() & Self::SU_MASK) != 0
    }

    /// Set SU bit atomically (RW)
    #[inline(always)]
    pub fn set_su(&self) {
        self.set_bits(Self::SU_MASK);
    }

    /// Clear SU bit atomically (RW)
    #[inline(always)]
    pub fn clr_su(&self) {
        self.clr_bits(Self::SU_MASK);
    }

    /// **NSP** `[1]` (RW)
    ///
    /// If 1, RESETS can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub fn nsp(&self) -> bool {
        (self.value.read_bits() & Self::NSP_MASK) != 0
    }

    /// Set NSP bit atomically (RW)
    #[inline(always)]
    pub fn set_nsp(&self) {
        self.set_bits(Self::NSP_MASK);
    }

    /// Clear NSP bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsp(&self) {
        self.clr_bits(Self::NSP_MASK);
    }

    /// **NSU** `[0]` (RW)
    ///
    /// If 1, and NSP is also set, RESETS can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub fn nsu(&self) -> bool {
        (self.value.read_bits() & Self::NSU_MASK) != 0
    }

    /// Set NSU bit atomically (RW)
    #[inline(always)]
    pub fn set_nsu(&self) {
        self.set_bits(Self::NSU_MASK);
    }

    /// Clear NSU bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsu(&self) {
        self.clr_bits(Self::NSU_MASK);
    }
}

/// RESETS value type for batch read/modify/write operations
///
/// Use `resets.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `resets.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ResetsVal {
    bits: u32,
}

impl ResetsVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x000000fc }
    }

    /// Read **DBG** `[7:7]`
    ///
    /// If 1, RESETS can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        (self.bits & Resets::DBG_MASK) != 0
    }

    /// Read **DMA** `[6:6]`
    ///
    /// If 1, RESETS can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        (self.bits & Resets::DMA_MASK) != 0
    }

    /// Read **CORE1** `[5:5]`
    ///
    /// If 1, RESETS can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        (self.bits & Resets::CORE1_MASK) != 0
    }

    /// Read **CORE0** `[4:4]`
    ///
    /// If 1, RESETS can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        (self.bits & Resets::CORE0_MASK) != 0
    }

    /// Read **SP** `[3:3]`
    ///
    /// If 1, RESETS can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        (self.bits & Resets::SP_MASK) != 0
    }

    /// Read **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, RESETS can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn su(&self) -> bool {
        (self.bits & Resets::SU_MASK) != 0
    }

    /// Read **NSP** `[1:1]`
    ///
    /// If 1, RESETS can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        (self.bits & Resets::NSP_MASK) != 0
    }

    /// Read **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, RESETS can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        (self.bits & Resets::NSU_MASK) != 0
    }

    /// Set **DBG** `[7:7]`
    ///
    /// If 1, RESETS can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dbg(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Resets::DBG_MASK
            } else {
                self.bits & !Resets::DBG_MASK
            }
        }
    }

    /// Set **DMA** `[6:6]`
    ///
    /// If 1, RESETS can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dma(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Resets::DMA_MASK
            } else {
                self.bits & !Resets::DMA_MASK
            }
        }
    }

    /// Set **CORE1** `[5:5]`
    ///
    /// If 1, RESETS can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Resets::CORE1_MASK
            } else {
                self.bits & !Resets::CORE1_MASK
            }
        }
    }

    /// Set **CORE0** `[4:4]`
    ///
    /// If 1, RESETS can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Resets::CORE0_MASK
            } else {
                self.bits & !Resets::CORE0_MASK
            }
        }
    }

    /// Set **SP** `[3:3]`
    ///
    /// If 1, RESETS can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn with_sp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Resets::SP_MASK
            } else {
                self.bits & !Resets::SP_MASK
            }
        }
    }

    /// Set **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, RESETS can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn with_su(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Resets::SU_MASK
            } else {
                self.bits & !Resets::SU_MASK
            }
        }
    }

    /// Set **NSP** `[1:1]`
    ///
    /// If 1, RESETS can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn with_nsp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Resets::NSP_MASK
            } else {
                self.bits & !Resets::NSP_MASK
            }
        }
    }

    /// Set **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, RESETS can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn with_nsu(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Resets::NSU_MASK
            } else {
                self.bits & !Resets::NSU_MASK
            }
        }
    }
}

impl Default for ResetsVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Resets {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ResetsVal {
        ResetsVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ResetsVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ResetsVal) -> ResetsVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// IO_BANK0 Register
///
/// Control whether debugger, DMA, core 0 and core 1 can access IO_BANK0, and at what security/privilege levels they can do so.
/// Defaults to Secure access from any master.
/// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
///
/// Address offset: 0x0068
/// Reset value: 0x000000fc
///
/// ## Fields
///
/// - **DBG** `[7]` (RW)
///
///   If 1, IO_BANK0 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **DMA** `[6]` (RW)
///
///   If 1, IO_BANK0 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE1** `[5]` (RW)
///
///   If 1, IO_BANK0 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE0** `[4]` (RW)
///
///   If 1, IO_BANK0 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **SP** `[3]` (RW)
///
///   If 1, IO_BANK0 can be accessed from a Secure, Privileged context.
///
/// - **SU** `[2]` (RW)
///
///   If 1, and SP is also set, IO_BANK0 can be accessed from a Secure, Unprivileged context.
///
/// - **NSP** `[1]` (RW)
///
///   If 1, IO_BANK0 can be accessed from a Non-secure, Privileged context.
///
/// - **NSU** `[0]` (RW)
///
///   If 1, and NSP is also set, IO_BANK0 can be accessed from a Non-secure, Unprivileged context.
///   This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
#[repr(transparent)]
pub struct IoBank0 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for IoBank0 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IoBank0 {
    // ======== Field Constants ========

    /// **DBG**
    ///
    /// If 1, IO_BANK0 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const DBG_OFFSET: u32 = 7;
    pub const DBG_MASK: u32 = 0x00000080;
    pub const DBG: u32 = 1 << 7;

    /// **DMA**
    ///
    /// If 1, IO_BANK0 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const DMA_OFFSET: u32 = 6;
    pub const DMA_MASK: u32 = 0x00000040;
    pub const DMA: u32 = 1 << 6;

    /// **CORE1**
    ///
    /// If 1, IO_BANK0 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CORE1_OFFSET: u32 = 5;
    pub const CORE1_MASK: u32 = 0x00000020;
    pub const CORE1: u32 = 1 << 5;

    /// **CORE0**
    ///
    /// If 1, IO_BANK0 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CORE0_OFFSET: u32 = 4;
    pub const CORE0_MASK: u32 = 0x00000010;
    pub const CORE0: u32 = 1 << 4;

    /// **SP**
    ///
    /// If 1, IO_BANK0 can be accessed from a Secure, Privileged context.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SP_OFFSET: u32 = 3;
    pub const SP_MASK: u32 = 0x00000008;
    pub const SP: u32 = 1 << 3;

    /// **SU**
    ///
    /// If 1, and SP is also set, IO_BANK0 can be accessed from a Secure, Unprivileged context.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SU_OFFSET: u32 = 2;
    pub const SU_MASK: u32 = 0x00000004;
    pub const SU: u32 = 1 << 2;

    /// **NSP**
    ///
    /// If 1, IO_BANK0 can be accessed from a Non-secure, Privileged context.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const NSP_OFFSET: u32 = 1;
    pub const NSP_MASK: u32 = 0x00000002;
    pub const NSP: u32 = 1 << 1;

    /// **NSU**
    ///
    /// If 1, and NSP is also set, IO_BANK0 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const NSU_OFFSET: u32 = 0;
    pub const NSU_MASK: u32 = 0x00000001;
    pub const NSU: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DBG** `[7]` (RW)
    ///
    /// If 1, IO_BANK0 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dbg(&self) -> bool {
        (self.value.read_bits() & Self::DBG_MASK) != 0
    }

    /// Set DBG bit atomically (RW)
    #[inline(always)]
    pub fn set_dbg(&self) {
        self.set_bits(Self::DBG_MASK);
    }

    /// Clear DBG bit atomically (RW)
    #[inline(always)]
    pub fn clr_dbg(&self) {
        self.clr_bits(Self::DBG_MASK);
    }

    /// **DMA** `[6]` (RW)
    ///
    /// If 1, IO_BANK0 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dma(&self) -> bool {
        (self.value.read_bits() & Self::DMA_MASK) != 0
    }

    /// Set DMA bit atomically (RW)
    #[inline(always)]
    pub fn set_dma(&self) {
        self.set_bits(Self::DMA_MASK);
    }

    /// Clear DMA bit atomically (RW)
    #[inline(always)]
    pub fn clr_dma(&self) {
        self.clr_bits(Self::DMA_MASK);
    }

    /// **CORE1** `[5]` (RW)
    ///
    /// If 1, IO_BANK0 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core1(&self) -> bool {
        (self.value.read_bits() & Self::CORE1_MASK) != 0
    }

    /// Set CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn set_core1(&self) {
        self.set_bits(Self::CORE1_MASK);
    }

    /// Clear CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core1(&self) {
        self.clr_bits(Self::CORE1_MASK);
    }

    /// **CORE0** `[4]` (RW)
    ///
    /// If 1, IO_BANK0 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core0(&self) -> bool {
        (self.value.read_bits() & Self::CORE0_MASK) != 0
    }

    /// Set CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn set_core0(&self) {
        self.set_bits(Self::CORE0_MASK);
    }

    /// Clear CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core0(&self) {
        self.clr_bits(Self::CORE0_MASK);
    }

    /// **SP** `[3]` (RW)
    ///
    /// If 1, IO_BANK0 can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub fn sp(&self) -> bool {
        (self.value.read_bits() & Self::SP_MASK) != 0
    }

    /// Set SP bit atomically (RW)
    #[inline(always)]
    pub fn set_sp(&self) {
        self.set_bits(Self::SP_MASK);
    }

    /// Clear SP bit atomically (RW)
    #[inline(always)]
    pub fn clr_sp(&self) {
        self.clr_bits(Self::SP_MASK);
    }

    /// **SU** `[2]` (RW)
    ///
    /// If 1, and SP is also set, IO_BANK0 can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub fn su(&self) -> bool {
        (self.value.read_bits() & Self::SU_MASK) != 0
    }

    /// Set SU bit atomically (RW)
    #[inline(always)]
    pub fn set_su(&self) {
        self.set_bits(Self::SU_MASK);
    }

    /// Clear SU bit atomically (RW)
    #[inline(always)]
    pub fn clr_su(&self) {
        self.clr_bits(Self::SU_MASK);
    }

    /// **NSP** `[1]` (RW)
    ///
    /// If 1, IO_BANK0 can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub fn nsp(&self) -> bool {
        (self.value.read_bits() & Self::NSP_MASK) != 0
    }

    /// Set NSP bit atomically (RW)
    #[inline(always)]
    pub fn set_nsp(&self) {
        self.set_bits(Self::NSP_MASK);
    }

    /// Clear NSP bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsp(&self) {
        self.clr_bits(Self::NSP_MASK);
    }

    /// **NSU** `[0]` (RW)
    ///
    /// If 1, and NSP is also set, IO_BANK0 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub fn nsu(&self) -> bool {
        (self.value.read_bits() & Self::NSU_MASK) != 0
    }

    /// Set NSU bit atomically (RW)
    #[inline(always)]
    pub fn set_nsu(&self) {
        self.set_bits(Self::NSU_MASK);
    }

    /// Clear NSU bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsu(&self) {
        self.clr_bits(Self::NSU_MASK);
    }
}

/// IO_BANK0 value type for batch read/modify/write operations
///
/// Use `iobank0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `iobank0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IoBank0Val {
    bits: u32,
}

impl IoBank0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x000000fc }
    }

    /// Read **DBG** `[7:7]`
    ///
    /// If 1, IO_BANK0 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        (self.bits & IoBank0::DBG_MASK) != 0
    }

    /// Read **DMA** `[6:6]`
    ///
    /// If 1, IO_BANK0 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        (self.bits & IoBank0::DMA_MASK) != 0
    }

    /// Read **CORE1** `[5:5]`
    ///
    /// If 1, IO_BANK0 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        (self.bits & IoBank0::CORE1_MASK) != 0
    }

    /// Read **CORE0** `[4:4]`
    ///
    /// If 1, IO_BANK0 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        (self.bits & IoBank0::CORE0_MASK) != 0
    }

    /// Read **SP** `[3:3]`
    ///
    /// If 1, IO_BANK0 can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        (self.bits & IoBank0::SP_MASK) != 0
    }

    /// Read **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, IO_BANK0 can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn su(&self) -> bool {
        (self.bits & IoBank0::SU_MASK) != 0
    }

    /// Read **NSP** `[1:1]`
    ///
    /// If 1, IO_BANK0 can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        (self.bits & IoBank0::NSP_MASK) != 0
    }

    /// Read **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, IO_BANK0 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        (self.bits & IoBank0::NSU_MASK) != 0
    }

    /// Set **DBG** `[7:7]`
    ///
    /// If 1, IO_BANK0 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dbg(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IoBank0::DBG_MASK
            } else {
                self.bits & !IoBank0::DBG_MASK
            }
        }
    }

    /// Set **DMA** `[6:6]`
    ///
    /// If 1, IO_BANK0 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dma(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IoBank0::DMA_MASK
            } else {
                self.bits & !IoBank0::DMA_MASK
            }
        }
    }

    /// Set **CORE1** `[5:5]`
    ///
    /// If 1, IO_BANK0 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IoBank0::CORE1_MASK
            } else {
                self.bits & !IoBank0::CORE1_MASK
            }
        }
    }

    /// Set **CORE0** `[4:4]`
    ///
    /// If 1, IO_BANK0 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IoBank0::CORE0_MASK
            } else {
                self.bits & !IoBank0::CORE0_MASK
            }
        }
    }

    /// Set **SP** `[3:3]`
    ///
    /// If 1, IO_BANK0 can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn with_sp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IoBank0::SP_MASK
            } else {
                self.bits & !IoBank0::SP_MASK
            }
        }
    }

    /// Set **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, IO_BANK0 can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn with_su(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IoBank0::SU_MASK
            } else {
                self.bits & !IoBank0::SU_MASK
            }
        }
    }

    /// Set **NSP** `[1:1]`
    ///
    /// If 1, IO_BANK0 can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn with_nsp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IoBank0::NSP_MASK
            } else {
                self.bits & !IoBank0::NSP_MASK
            }
        }
    }

    /// Set **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, IO_BANK0 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn with_nsu(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IoBank0::NSU_MASK
            } else {
                self.bits & !IoBank0::NSU_MASK
            }
        }
    }
}

impl Default for IoBank0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl IoBank0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IoBank0Val {
        IoBank0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IoBank0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IoBank0Val) -> IoBank0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// IO_BANK1 Register
///
/// Control whether debugger, DMA, core 0 and core 1 can access IO_BANK1, and at what security/privilege levels they can do so.
/// Defaults to Secure access from any master.
/// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
///
/// Address offset: 0x006c
/// Reset value: 0x000000fc
///
/// ## Fields
///
/// - **DBG** `[7]` (RW)
///
///   If 1, IO_BANK1 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **DMA** `[6]` (RW)
///
///   If 1, IO_BANK1 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE1** `[5]` (RW)
///
///   If 1, IO_BANK1 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE0** `[4]` (RW)
///
///   If 1, IO_BANK1 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **SP** `[3]` (RW)
///
///   If 1, IO_BANK1 can be accessed from a Secure, Privileged context.
///
/// - **SU** `[2]` (RW)
///
///   If 1, and SP is also set, IO_BANK1 can be accessed from a Secure, Unprivileged context.
///
/// - **NSP** `[1]` (RW)
///
///   If 1, IO_BANK1 can be accessed from a Non-secure, Privileged context.
///
/// - **NSU** `[0]` (RW)
///
///   If 1, and NSP is also set, IO_BANK1 can be accessed from a Non-secure, Unprivileged context.
///   This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
#[repr(transparent)]
pub struct IoBank1 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for IoBank1 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IoBank1 {
    // ======== Field Constants ========

    /// **DBG**
    ///
    /// If 1, IO_BANK1 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const DBG_OFFSET: u32 = 7;
    pub const DBG_MASK: u32 = 0x00000080;
    pub const DBG: u32 = 1 << 7;

    /// **DMA**
    ///
    /// If 1, IO_BANK1 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const DMA_OFFSET: u32 = 6;
    pub const DMA_MASK: u32 = 0x00000040;
    pub const DMA: u32 = 1 << 6;

    /// **CORE1**
    ///
    /// If 1, IO_BANK1 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CORE1_OFFSET: u32 = 5;
    pub const CORE1_MASK: u32 = 0x00000020;
    pub const CORE1: u32 = 1 << 5;

    /// **CORE0**
    ///
    /// If 1, IO_BANK1 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CORE0_OFFSET: u32 = 4;
    pub const CORE0_MASK: u32 = 0x00000010;
    pub const CORE0: u32 = 1 << 4;

    /// **SP**
    ///
    /// If 1, IO_BANK1 can be accessed from a Secure, Privileged context.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SP_OFFSET: u32 = 3;
    pub const SP_MASK: u32 = 0x00000008;
    pub const SP: u32 = 1 << 3;

    /// **SU**
    ///
    /// If 1, and SP is also set, IO_BANK1 can be accessed from a Secure, Unprivileged context.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SU_OFFSET: u32 = 2;
    pub const SU_MASK: u32 = 0x00000004;
    pub const SU: u32 = 1 << 2;

    /// **NSP**
    ///
    /// If 1, IO_BANK1 can be accessed from a Non-secure, Privileged context.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const NSP_OFFSET: u32 = 1;
    pub const NSP_MASK: u32 = 0x00000002;
    pub const NSP: u32 = 1 << 1;

    /// **NSU**
    ///
    /// If 1, and NSP is also set, IO_BANK1 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const NSU_OFFSET: u32 = 0;
    pub const NSU_MASK: u32 = 0x00000001;
    pub const NSU: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DBG** `[7]` (RW)
    ///
    /// If 1, IO_BANK1 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dbg(&self) -> bool {
        (self.value.read_bits() & Self::DBG_MASK) != 0
    }

    /// Set DBG bit atomically (RW)
    #[inline(always)]
    pub fn set_dbg(&self) {
        self.set_bits(Self::DBG_MASK);
    }

    /// Clear DBG bit atomically (RW)
    #[inline(always)]
    pub fn clr_dbg(&self) {
        self.clr_bits(Self::DBG_MASK);
    }

    /// **DMA** `[6]` (RW)
    ///
    /// If 1, IO_BANK1 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dma(&self) -> bool {
        (self.value.read_bits() & Self::DMA_MASK) != 0
    }

    /// Set DMA bit atomically (RW)
    #[inline(always)]
    pub fn set_dma(&self) {
        self.set_bits(Self::DMA_MASK);
    }

    /// Clear DMA bit atomically (RW)
    #[inline(always)]
    pub fn clr_dma(&self) {
        self.clr_bits(Self::DMA_MASK);
    }

    /// **CORE1** `[5]` (RW)
    ///
    /// If 1, IO_BANK1 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core1(&self) -> bool {
        (self.value.read_bits() & Self::CORE1_MASK) != 0
    }

    /// Set CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn set_core1(&self) {
        self.set_bits(Self::CORE1_MASK);
    }

    /// Clear CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core1(&self) {
        self.clr_bits(Self::CORE1_MASK);
    }

    /// **CORE0** `[4]` (RW)
    ///
    /// If 1, IO_BANK1 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core0(&self) -> bool {
        (self.value.read_bits() & Self::CORE0_MASK) != 0
    }

    /// Set CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn set_core0(&self) {
        self.set_bits(Self::CORE0_MASK);
    }

    /// Clear CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core0(&self) {
        self.clr_bits(Self::CORE0_MASK);
    }

    /// **SP** `[3]` (RW)
    ///
    /// If 1, IO_BANK1 can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub fn sp(&self) -> bool {
        (self.value.read_bits() & Self::SP_MASK) != 0
    }

    /// Set SP bit atomically (RW)
    #[inline(always)]
    pub fn set_sp(&self) {
        self.set_bits(Self::SP_MASK);
    }

    /// Clear SP bit atomically (RW)
    #[inline(always)]
    pub fn clr_sp(&self) {
        self.clr_bits(Self::SP_MASK);
    }

    /// **SU** `[2]` (RW)
    ///
    /// If 1, and SP is also set, IO_BANK1 can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub fn su(&self) -> bool {
        (self.value.read_bits() & Self::SU_MASK) != 0
    }

    /// Set SU bit atomically (RW)
    #[inline(always)]
    pub fn set_su(&self) {
        self.set_bits(Self::SU_MASK);
    }

    /// Clear SU bit atomically (RW)
    #[inline(always)]
    pub fn clr_su(&self) {
        self.clr_bits(Self::SU_MASK);
    }

    /// **NSP** `[1]` (RW)
    ///
    /// If 1, IO_BANK1 can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub fn nsp(&self) -> bool {
        (self.value.read_bits() & Self::NSP_MASK) != 0
    }

    /// Set NSP bit atomically (RW)
    #[inline(always)]
    pub fn set_nsp(&self) {
        self.set_bits(Self::NSP_MASK);
    }

    /// Clear NSP bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsp(&self) {
        self.clr_bits(Self::NSP_MASK);
    }

    /// **NSU** `[0]` (RW)
    ///
    /// If 1, and NSP is also set, IO_BANK1 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub fn nsu(&self) -> bool {
        (self.value.read_bits() & Self::NSU_MASK) != 0
    }

    /// Set NSU bit atomically (RW)
    #[inline(always)]
    pub fn set_nsu(&self) {
        self.set_bits(Self::NSU_MASK);
    }

    /// Clear NSU bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsu(&self) {
        self.clr_bits(Self::NSU_MASK);
    }
}

/// IO_BANK1 value type for batch read/modify/write operations
///
/// Use `iobank1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `iobank1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IoBank1Val {
    bits: u32,
}

impl IoBank1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x000000fc }
    }

    /// Read **DBG** `[7:7]`
    ///
    /// If 1, IO_BANK1 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        (self.bits & IoBank1::DBG_MASK) != 0
    }

    /// Read **DMA** `[6:6]`
    ///
    /// If 1, IO_BANK1 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        (self.bits & IoBank1::DMA_MASK) != 0
    }

    /// Read **CORE1** `[5:5]`
    ///
    /// If 1, IO_BANK1 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        (self.bits & IoBank1::CORE1_MASK) != 0
    }

    /// Read **CORE0** `[4:4]`
    ///
    /// If 1, IO_BANK1 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        (self.bits & IoBank1::CORE0_MASK) != 0
    }

    /// Read **SP** `[3:3]`
    ///
    /// If 1, IO_BANK1 can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        (self.bits & IoBank1::SP_MASK) != 0
    }

    /// Read **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, IO_BANK1 can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn su(&self) -> bool {
        (self.bits & IoBank1::SU_MASK) != 0
    }

    /// Read **NSP** `[1:1]`
    ///
    /// If 1, IO_BANK1 can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        (self.bits & IoBank1::NSP_MASK) != 0
    }

    /// Read **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, IO_BANK1 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        (self.bits & IoBank1::NSU_MASK) != 0
    }

    /// Set **DBG** `[7:7]`
    ///
    /// If 1, IO_BANK1 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dbg(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IoBank1::DBG_MASK
            } else {
                self.bits & !IoBank1::DBG_MASK
            }
        }
    }

    /// Set **DMA** `[6:6]`
    ///
    /// If 1, IO_BANK1 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dma(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IoBank1::DMA_MASK
            } else {
                self.bits & !IoBank1::DMA_MASK
            }
        }
    }

    /// Set **CORE1** `[5:5]`
    ///
    /// If 1, IO_BANK1 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IoBank1::CORE1_MASK
            } else {
                self.bits & !IoBank1::CORE1_MASK
            }
        }
    }

    /// Set **CORE0** `[4:4]`
    ///
    /// If 1, IO_BANK1 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IoBank1::CORE0_MASK
            } else {
                self.bits & !IoBank1::CORE0_MASK
            }
        }
    }

    /// Set **SP** `[3:3]`
    ///
    /// If 1, IO_BANK1 can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn with_sp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IoBank1::SP_MASK
            } else {
                self.bits & !IoBank1::SP_MASK
            }
        }
    }

    /// Set **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, IO_BANK1 can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn with_su(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IoBank1::SU_MASK
            } else {
                self.bits & !IoBank1::SU_MASK
            }
        }
    }

    /// Set **NSP** `[1:1]`
    ///
    /// If 1, IO_BANK1 can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn with_nsp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IoBank1::NSP_MASK
            } else {
                self.bits & !IoBank1::NSP_MASK
            }
        }
    }

    /// Set **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, IO_BANK1 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn with_nsu(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IoBank1::NSU_MASK
            } else {
                self.bits & !IoBank1::NSU_MASK
            }
        }
    }
}

impl Default for IoBank1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl IoBank1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IoBank1Val {
        IoBank1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IoBank1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IoBank1Val) -> IoBank1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PADS_BANK0 Register
///
/// Control whether debugger, DMA, core 0 and core 1 can access PADS_BANK0, and at what security/privilege levels they can do so.
/// Defaults to Secure access from any master.
/// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
///
/// Address offset: 0x0070
/// Reset value: 0x000000fc
///
/// ## Fields
///
/// - **DBG** `[7]` (RW)
///
///   If 1, PADS_BANK0 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **DMA** `[6]` (RW)
///
///   If 1, PADS_BANK0 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE1** `[5]` (RW)
///
///   If 1, PADS_BANK0 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE0** `[4]` (RW)
///
///   If 1, PADS_BANK0 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **SP** `[3]` (RW)
///
///   If 1, PADS_BANK0 can be accessed from a Secure, Privileged context.
///
/// - **SU** `[2]` (RW)
///
///   If 1, and SP is also set, PADS_BANK0 can be accessed from a Secure, Unprivileged context.
///
/// - **NSP** `[1]` (RW)
///
///   If 1, PADS_BANK0 can be accessed from a Non-secure, Privileged context.
///
/// - **NSU** `[0]` (RW)
///
///   If 1, and NSP is also set, PADS_BANK0 can be accessed from a Non-secure, Unprivileged context.
///   This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
#[repr(transparent)]
pub struct PadsBank0 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for PadsBank0 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PadsBank0 {
    // ======== Field Constants ========

    /// **DBG**
    ///
    /// If 1, PADS_BANK0 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const DBG_OFFSET: u32 = 7;
    pub const DBG_MASK: u32 = 0x00000080;
    pub const DBG: u32 = 1 << 7;

    /// **DMA**
    ///
    /// If 1, PADS_BANK0 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const DMA_OFFSET: u32 = 6;
    pub const DMA_MASK: u32 = 0x00000040;
    pub const DMA: u32 = 1 << 6;

    /// **CORE1**
    ///
    /// If 1, PADS_BANK0 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CORE1_OFFSET: u32 = 5;
    pub const CORE1_MASK: u32 = 0x00000020;
    pub const CORE1: u32 = 1 << 5;

    /// **CORE0**
    ///
    /// If 1, PADS_BANK0 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CORE0_OFFSET: u32 = 4;
    pub const CORE0_MASK: u32 = 0x00000010;
    pub const CORE0: u32 = 1 << 4;

    /// **SP**
    ///
    /// If 1, PADS_BANK0 can be accessed from a Secure, Privileged context.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SP_OFFSET: u32 = 3;
    pub const SP_MASK: u32 = 0x00000008;
    pub const SP: u32 = 1 << 3;

    /// **SU**
    ///
    /// If 1, and SP is also set, PADS_BANK0 can be accessed from a Secure, Unprivileged context.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SU_OFFSET: u32 = 2;
    pub const SU_MASK: u32 = 0x00000004;
    pub const SU: u32 = 1 << 2;

    /// **NSP**
    ///
    /// If 1, PADS_BANK0 can be accessed from a Non-secure, Privileged context.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const NSP_OFFSET: u32 = 1;
    pub const NSP_MASK: u32 = 0x00000002;
    pub const NSP: u32 = 1 << 1;

    /// **NSU**
    ///
    /// If 1, and NSP is also set, PADS_BANK0 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const NSU_OFFSET: u32 = 0;
    pub const NSU_MASK: u32 = 0x00000001;
    pub const NSU: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DBG** `[7]` (RW)
    ///
    /// If 1, PADS_BANK0 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dbg(&self) -> bool {
        (self.value.read_bits() & Self::DBG_MASK) != 0
    }

    /// Set DBG bit atomically (RW)
    #[inline(always)]
    pub fn set_dbg(&self) {
        self.set_bits(Self::DBG_MASK);
    }

    /// Clear DBG bit atomically (RW)
    #[inline(always)]
    pub fn clr_dbg(&self) {
        self.clr_bits(Self::DBG_MASK);
    }

    /// **DMA** `[6]` (RW)
    ///
    /// If 1, PADS_BANK0 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dma(&self) -> bool {
        (self.value.read_bits() & Self::DMA_MASK) != 0
    }

    /// Set DMA bit atomically (RW)
    #[inline(always)]
    pub fn set_dma(&self) {
        self.set_bits(Self::DMA_MASK);
    }

    /// Clear DMA bit atomically (RW)
    #[inline(always)]
    pub fn clr_dma(&self) {
        self.clr_bits(Self::DMA_MASK);
    }

    /// **CORE1** `[5]` (RW)
    ///
    /// If 1, PADS_BANK0 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core1(&self) -> bool {
        (self.value.read_bits() & Self::CORE1_MASK) != 0
    }

    /// Set CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn set_core1(&self) {
        self.set_bits(Self::CORE1_MASK);
    }

    /// Clear CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core1(&self) {
        self.clr_bits(Self::CORE1_MASK);
    }

    /// **CORE0** `[4]` (RW)
    ///
    /// If 1, PADS_BANK0 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core0(&self) -> bool {
        (self.value.read_bits() & Self::CORE0_MASK) != 0
    }

    /// Set CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn set_core0(&self) {
        self.set_bits(Self::CORE0_MASK);
    }

    /// Clear CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core0(&self) {
        self.clr_bits(Self::CORE0_MASK);
    }

    /// **SP** `[3]` (RW)
    ///
    /// If 1, PADS_BANK0 can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub fn sp(&self) -> bool {
        (self.value.read_bits() & Self::SP_MASK) != 0
    }

    /// Set SP bit atomically (RW)
    #[inline(always)]
    pub fn set_sp(&self) {
        self.set_bits(Self::SP_MASK);
    }

    /// Clear SP bit atomically (RW)
    #[inline(always)]
    pub fn clr_sp(&self) {
        self.clr_bits(Self::SP_MASK);
    }

    /// **SU** `[2]` (RW)
    ///
    /// If 1, and SP is also set, PADS_BANK0 can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub fn su(&self) -> bool {
        (self.value.read_bits() & Self::SU_MASK) != 0
    }

    /// Set SU bit atomically (RW)
    #[inline(always)]
    pub fn set_su(&self) {
        self.set_bits(Self::SU_MASK);
    }

    /// Clear SU bit atomically (RW)
    #[inline(always)]
    pub fn clr_su(&self) {
        self.clr_bits(Self::SU_MASK);
    }

    /// **NSP** `[1]` (RW)
    ///
    /// If 1, PADS_BANK0 can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub fn nsp(&self) -> bool {
        (self.value.read_bits() & Self::NSP_MASK) != 0
    }

    /// Set NSP bit atomically (RW)
    #[inline(always)]
    pub fn set_nsp(&self) {
        self.set_bits(Self::NSP_MASK);
    }

    /// Clear NSP bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsp(&self) {
        self.clr_bits(Self::NSP_MASK);
    }

    /// **NSU** `[0]` (RW)
    ///
    /// If 1, and NSP is also set, PADS_BANK0 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub fn nsu(&self) -> bool {
        (self.value.read_bits() & Self::NSU_MASK) != 0
    }

    /// Set NSU bit atomically (RW)
    #[inline(always)]
    pub fn set_nsu(&self) {
        self.set_bits(Self::NSU_MASK);
    }

    /// Clear NSU bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsu(&self) {
        self.clr_bits(Self::NSU_MASK);
    }
}

/// PADS_BANK0 value type for batch read/modify/write operations
///
/// Use `padsbank0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `padsbank0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PadsBank0Val {
    bits: u32,
}

impl PadsBank0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x000000fc }
    }

    /// Read **DBG** `[7:7]`
    ///
    /// If 1, PADS_BANK0 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        (self.bits & PadsBank0::DBG_MASK) != 0
    }

    /// Read **DMA** `[6:6]`
    ///
    /// If 1, PADS_BANK0 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        (self.bits & PadsBank0::DMA_MASK) != 0
    }

    /// Read **CORE1** `[5:5]`
    ///
    /// If 1, PADS_BANK0 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        (self.bits & PadsBank0::CORE1_MASK) != 0
    }

    /// Read **CORE0** `[4:4]`
    ///
    /// If 1, PADS_BANK0 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        (self.bits & PadsBank0::CORE0_MASK) != 0
    }

    /// Read **SP** `[3:3]`
    ///
    /// If 1, PADS_BANK0 can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        (self.bits & PadsBank0::SP_MASK) != 0
    }

    /// Read **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, PADS_BANK0 can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn su(&self) -> bool {
        (self.bits & PadsBank0::SU_MASK) != 0
    }

    /// Read **NSP** `[1:1]`
    ///
    /// If 1, PADS_BANK0 can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        (self.bits & PadsBank0::NSP_MASK) != 0
    }

    /// Read **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, PADS_BANK0 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        (self.bits & PadsBank0::NSU_MASK) != 0
    }

    /// Set **DBG** `[7:7]`
    ///
    /// If 1, PADS_BANK0 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dbg(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PadsBank0::DBG_MASK
            } else {
                self.bits & !PadsBank0::DBG_MASK
            }
        }
    }

    /// Set **DMA** `[6:6]`
    ///
    /// If 1, PADS_BANK0 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dma(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PadsBank0::DMA_MASK
            } else {
                self.bits & !PadsBank0::DMA_MASK
            }
        }
    }

    /// Set **CORE1** `[5:5]`
    ///
    /// If 1, PADS_BANK0 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PadsBank0::CORE1_MASK
            } else {
                self.bits & !PadsBank0::CORE1_MASK
            }
        }
    }

    /// Set **CORE0** `[4:4]`
    ///
    /// If 1, PADS_BANK0 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PadsBank0::CORE0_MASK
            } else {
                self.bits & !PadsBank0::CORE0_MASK
            }
        }
    }

    /// Set **SP** `[3:3]`
    ///
    /// If 1, PADS_BANK0 can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn with_sp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PadsBank0::SP_MASK
            } else {
                self.bits & !PadsBank0::SP_MASK
            }
        }
    }

    /// Set **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, PADS_BANK0 can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn with_su(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PadsBank0::SU_MASK
            } else {
                self.bits & !PadsBank0::SU_MASK
            }
        }
    }

    /// Set **NSP** `[1:1]`
    ///
    /// If 1, PADS_BANK0 can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn with_nsp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PadsBank0::NSP_MASK
            } else {
                self.bits & !PadsBank0::NSP_MASK
            }
        }
    }

    /// Set **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, PADS_BANK0 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn with_nsu(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PadsBank0::NSU_MASK
            } else {
                self.bits & !PadsBank0::NSU_MASK
            }
        }
    }
}

impl Default for PadsBank0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl PadsBank0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PadsBank0Val {
        PadsBank0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: PadsBank0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(PadsBank0Val) -> PadsBank0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PADS_QSPI Register
///
/// Control whether debugger, DMA, core 0 and core 1 can access PADS_QSPI, and at what security/privilege levels they can do so.
/// Defaults to Secure access from any master.
/// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
///
/// Address offset: 0x0074
/// Reset value: 0x000000fc
///
/// ## Fields
///
/// - **DBG** `[7]` (RW)
///
///   If 1, PADS_QSPI can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **DMA** `[6]` (RW)
///
///   If 1, PADS_QSPI can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE1** `[5]` (RW)
///
///   If 1, PADS_QSPI can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE0** `[4]` (RW)
///
///   If 1, PADS_QSPI can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **SP** `[3]` (RW)
///
///   If 1, PADS_QSPI can be accessed from a Secure, Privileged context.
///
/// - **SU** `[2]` (RW)
///
///   If 1, and SP is also set, PADS_QSPI can be accessed from a Secure, Unprivileged context.
///
/// - **NSP** `[1]` (RW)
///
///   If 1, PADS_QSPI can be accessed from a Non-secure, Privileged context.
///
/// - **NSU** `[0]` (RW)
///
///   If 1, and NSP is also set, PADS_QSPI can be accessed from a Non-secure, Unprivileged context.
///   This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
#[repr(transparent)]
pub struct PadsQspi {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for PadsQspi {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PadsQspi {
    // ======== Field Constants ========

    /// **DBG**
    ///
    /// If 1, PADS_QSPI can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const DBG_OFFSET: u32 = 7;
    pub const DBG_MASK: u32 = 0x00000080;
    pub const DBG: u32 = 1 << 7;

    /// **DMA**
    ///
    /// If 1, PADS_QSPI can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const DMA_OFFSET: u32 = 6;
    pub const DMA_MASK: u32 = 0x00000040;
    pub const DMA: u32 = 1 << 6;

    /// **CORE1**
    ///
    /// If 1, PADS_QSPI can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CORE1_OFFSET: u32 = 5;
    pub const CORE1_MASK: u32 = 0x00000020;
    pub const CORE1: u32 = 1 << 5;

    /// **CORE0**
    ///
    /// If 1, PADS_QSPI can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CORE0_OFFSET: u32 = 4;
    pub const CORE0_MASK: u32 = 0x00000010;
    pub const CORE0: u32 = 1 << 4;

    /// **SP**
    ///
    /// If 1, PADS_QSPI can be accessed from a Secure, Privileged context.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SP_OFFSET: u32 = 3;
    pub const SP_MASK: u32 = 0x00000008;
    pub const SP: u32 = 1 << 3;

    /// **SU**
    ///
    /// If 1, and SP is also set, PADS_QSPI can be accessed from a Secure, Unprivileged context.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SU_OFFSET: u32 = 2;
    pub const SU_MASK: u32 = 0x00000004;
    pub const SU: u32 = 1 << 2;

    /// **NSP**
    ///
    /// If 1, PADS_QSPI can be accessed from a Non-secure, Privileged context.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const NSP_OFFSET: u32 = 1;
    pub const NSP_MASK: u32 = 0x00000002;
    pub const NSP: u32 = 1 << 1;

    /// **NSU**
    ///
    /// If 1, and NSP is also set, PADS_QSPI can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const NSU_OFFSET: u32 = 0;
    pub const NSU_MASK: u32 = 0x00000001;
    pub const NSU: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DBG** `[7]` (RW)
    ///
    /// If 1, PADS_QSPI can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dbg(&self) -> bool {
        (self.value.read_bits() & Self::DBG_MASK) != 0
    }

    /// Set DBG bit atomically (RW)
    #[inline(always)]
    pub fn set_dbg(&self) {
        self.set_bits(Self::DBG_MASK);
    }

    /// Clear DBG bit atomically (RW)
    #[inline(always)]
    pub fn clr_dbg(&self) {
        self.clr_bits(Self::DBG_MASK);
    }

    /// **DMA** `[6]` (RW)
    ///
    /// If 1, PADS_QSPI can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dma(&self) -> bool {
        (self.value.read_bits() & Self::DMA_MASK) != 0
    }

    /// Set DMA bit atomically (RW)
    #[inline(always)]
    pub fn set_dma(&self) {
        self.set_bits(Self::DMA_MASK);
    }

    /// Clear DMA bit atomically (RW)
    #[inline(always)]
    pub fn clr_dma(&self) {
        self.clr_bits(Self::DMA_MASK);
    }

    /// **CORE1** `[5]` (RW)
    ///
    /// If 1, PADS_QSPI can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core1(&self) -> bool {
        (self.value.read_bits() & Self::CORE1_MASK) != 0
    }

    /// Set CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn set_core1(&self) {
        self.set_bits(Self::CORE1_MASK);
    }

    /// Clear CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core1(&self) {
        self.clr_bits(Self::CORE1_MASK);
    }

    /// **CORE0** `[4]` (RW)
    ///
    /// If 1, PADS_QSPI can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core0(&self) -> bool {
        (self.value.read_bits() & Self::CORE0_MASK) != 0
    }

    /// Set CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn set_core0(&self) {
        self.set_bits(Self::CORE0_MASK);
    }

    /// Clear CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core0(&self) {
        self.clr_bits(Self::CORE0_MASK);
    }

    /// **SP** `[3]` (RW)
    ///
    /// If 1, PADS_QSPI can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub fn sp(&self) -> bool {
        (self.value.read_bits() & Self::SP_MASK) != 0
    }

    /// Set SP bit atomically (RW)
    #[inline(always)]
    pub fn set_sp(&self) {
        self.set_bits(Self::SP_MASK);
    }

    /// Clear SP bit atomically (RW)
    #[inline(always)]
    pub fn clr_sp(&self) {
        self.clr_bits(Self::SP_MASK);
    }

    /// **SU** `[2]` (RW)
    ///
    /// If 1, and SP is also set, PADS_QSPI can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub fn su(&self) -> bool {
        (self.value.read_bits() & Self::SU_MASK) != 0
    }

    /// Set SU bit atomically (RW)
    #[inline(always)]
    pub fn set_su(&self) {
        self.set_bits(Self::SU_MASK);
    }

    /// Clear SU bit atomically (RW)
    #[inline(always)]
    pub fn clr_su(&self) {
        self.clr_bits(Self::SU_MASK);
    }

    /// **NSP** `[1]` (RW)
    ///
    /// If 1, PADS_QSPI can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub fn nsp(&self) -> bool {
        (self.value.read_bits() & Self::NSP_MASK) != 0
    }

    /// Set NSP bit atomically (RW)
    #[inline(always)]
    pub fn set_nsp(&self) {
        self.set_bits(Self::NSP_MASK);
    }

    /// Clear NSP bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsp(&self) {
        self.clr_bits(Self::NSP_MASK);
    }

    /// **NSU** `[0]` (RW)
    ///
    /// If 1, and NSP is also set, PADS_QSPI can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub fn nsu(&self) -> bool {
        (self.value.read_bits() & Self::NSU_MASK) != 0
    }

    /// Set NSU bit atomically (RW)
    #[inline(always)]
    pub fn set_nsu(&self) {
        self.set_bits(Self::NSU_MASK);
    }

    /// Clear NSU bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsu(&self) {
        self.clr_bits(Self::NSU_MASK);
    }
}

/// PADS_QSPI value type for batch read/modify/write operations
///
/// Use `padsqspi.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `padsqspi.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PadsQspiVal {
    bits: u32,
}

impl PadsQspiVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x000000fc }
    }

    /// Read **DBG** `[7:7]`
    ///
    /// If 1, PADS_QSPI can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        (self.bits & PadsQspi::DBG_MASK) != 0
    }

    /// Read **DMA** `[6:6]`
    ///
    /// If 1, PADS_QSPI can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        (self.bits & PadsQspi::DMA_MASK) != 0
    }

    /// Read **CORE1** `[5:5]`
    ///
    /// If 1, PADS_QSPI can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        (self.bits & PadsQspi::CORE1_MASK) != 0
    }

    /// Read **CORE0** `[4:4]`
    ///
    /// If 1, PADS_QSPI can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        (self.bits & PadsQspi::CORE0_MASK) != 0
    }

    /// Read **SP** `[3:3]`
    ///
    /// If 1, PADS_QSPI can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        (self.bits & PadsQspi::SP_MASK) != 0
    }

    /// Read **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, PADS_QSPI can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn su(&self) -> bool {
        (self.bits & PadsQspi::SU_MASK) != 0
    }

    /// Read **NSP** `[1:1]`
    ///
    /// If 1, PADS_QSPI can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        (self.bits & PadsQspi::NSP_MASK) != 0
    }

    /// Read **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, PADS_QSPI can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        (self.bits & PadsQspi::NSU_MASK) != 0
    }

    /// Set **DBG** `[7:7]`
    ///
    /// If 1, PADS_QSPI can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dbg(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PadsQspi::DBG_MASK
            } else {
                self.bits & !PadsQspi::DBG_MASK
            }
        }
    }

    /// Set **DMA** `[6:6]`
    ///
    /// If 1, PADS_QSPI can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dma(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PadsQspi::DMA_MASK
            } else {
                self.bits & !PadsQspi::DMA_MASK
            }
        }
    }

    /// Set **CORE1** `[5:5]`
    ///
    /// If 1, PADS_QSPI can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PadsQspi::CORE1_MASK
            } else {
                self.bits & !PadsQspi::CORE1_MASK
            }
        }
    }

    /// Set **CORE0** `[4:4]`
    ///
    /// If 1, PADS_QSPI can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PadsQspi::CORE0_MASK
            } else {
                self.bits & !PadsQspi::CORE0_MASK
            }
        }
    }

    /// Set **SP** `[3:3]`
    ///
    /// If 1, PADS_QSPI can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn with_sp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PadsQspi::SP_MASK
            } else {
                self.bits & !PadsQspi::SP_MASK
            }
        }
    }

    /// Set **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, PADS_QSPI can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn with_su(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PadsQspi::SU_MASK
            } else {
                self.bits & !PadsQspi::SU_MASK
            }
        }
    }

    /// Set **NSP** `[1:1]`
    ///
    /// If 1, PADS_QSPI can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn with_nsp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PadsQspi::NSP_MASK
            } else {
                self.bits & !PadsQspi::NSP_MASK
            }
        }
    }

    /// Set **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, PADS_QSPI can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn with_nsu(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PadsQspi::NSU_MASK
            } else {
                self.bits & !PadsQspi::NSU_MASK
            }
        }
    }
}

impl Default for PadsQspiVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl PadsQspi {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PadsQspiVal {
        PadsQspiVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: PadsQspiVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(PadsQspiVal) -> PadsQspiVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// BUSCTRL Register
///
/// Control whether debugger, DMA, core 0 and core 1 can access BUSCTRL, and at what security/privilege levels they can do so.
/// Defaults to Secure access from any master.
/// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
///
/// Address offset: 0x0078
/// Reset value: 0x000000fc
///
/// ## Fields
///
/// - **DBG** `[7]` (RW)
///
///   If 1, BUSCTRL can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **DMA** `[6]` (RW)
///
///   If 1, BUSCTRL can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE1** `[5]` (RW)
///
///   If 1, BUSCTRL can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE0** `[4]` (RW)
///
///   If 1, BUSCTRL can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **SP** `[3]` (RW)
///
///   If 1, BUSCTRL can be accessed from a Secure, Privileged context.
///
/// - **SU** `[2]` (RW)
///
///   If 1, and SP is also set, BUSCTRL can be accessed from a Secure, Unprivileged context.
///
/// - **NSP** `[1]` (RW)
///
///   If 1, BUSCTRL can be accessed from a Non-secure, Privileged context.
///
/// - **NSU** `[0]` (RW)
///
///   If 1, and NSP is also set, BUSCTRL can be accessed from a Non-secure, Unprivileged context.
///   This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
#[repr(transparent)]
pub struct Busctrl {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Busctrl {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Busctrl {
    // ======== Field Constants ========

    /// **DBG**
    ///
    /// If 1, BUSCTRL can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const DBG_OFFSET: u32 = 7;
    pub const DBG_MASK: u32 = 0x00000080;
    pub const DBG: u32 = 1 << 7;

    /// **DMA**
    ///
    /// If 1, BUSCTRL can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const DMA_OFFSET: u32 = 6;
    pub const DMA_MASK: u32 = 0x00000040;
    pub const DMA: u32 = 1 << 6;

    /// **CORE1**
    ///
    /// If 1, BUSCTRL can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CORE1_OFFSET: u32 = 5;
    pub const CORE1_MASK: u32 = 0x00000020;
    pub const CORE1: u32 = 1 << 5;

    /// **CORE0**
    ///
    /// If 1, BUSCTRL can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CORE0_OFFSET: u32 = 4;
    pub const CORE0_MASK: u32 = 0x00000010;
    pub const CORE0: u32 = 1 << 4;

    /// **SP**
    ///
    /// If 1, BUSCTRL can be accessed from a Secure, Privileged context.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SP_OFFSET: u32 = 3;
    pub const SP_MASK: u32 = 0x00000008;
    pub const SP: u32 = 1 << 3;

    /// **SU**
    ///
    /// If 1, and SP is also set, BUSCTRL can be accessed from a Secure, Unprivileged context.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SU_OFFSET: u32 = 2;
    pub const SU_MASK: u32 = 0x00000004;
    pub const SU: u32 = 1 << 2;

    /// **NSP**
    ///
    /// If 1, BUSCTRL can be accessed from a Non-secure, Privileged context.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const NSP_OFFSET: u32 = 1;
    pub const NSP_MASK: u32 = 0x00000002;
    pub const NSP: u32 = 1 << 1;

    /// **NSU**
    ///
    /// If 1, and NSP is also set, BUSCTRL can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const NSU_OFFSET: u32 = 0;
    pub const NSU_MASK: u32 = 0x00000001;
    pub const NSU: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DBG** `[7]` (RW)
    ///
    /// If 1, BUSCTRL can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dbg(&self) -> bool {
        (self.value.read_bits() & Self::DBG_MASK) != 0
    }

    /// Set DBG bit atomically (RW)
    #[inline(always)]
    pub fn set_dbg(&self) {
        self.set_bits(Self::DBG_MASK);
    }

    /// Clear DBG bit atomically (RW)
    #[inline(always)]
    pub fn clr_dbg(&self) {
        self.clr_bits(Self::DBG_MASK);
    }

    /// **DMA** `[6]` (RW)
    ///
    /// If 1, BUSCTRL can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dma(&self) -> bool {
        (self.value.read_bits() & Self::DMA_MASK) != 0
    }

    /// Set DMA bit atomically (RW)
    #[inline(always)]
    pub fn set_dma(&self) {
        self.set_bits(Self::DMA_MASK);
    }

    /// Clear DMA bit atomically (RW)
    #[inline(always)]
    pub fn clr_dma(&self) {
        self.clr_bits(Self::DMA_MASK);
    }

    /// **CORE1** `[5]` (RW)
    ///
    /// If 1, BUSCTRL can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core1(&self) -> bool {
        (self.value.read_bits() & Self::CORE1_MASK) != 0
    }

    /// Set CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn set_core1(&self) {
        self.set_bits(Self::CORE1_MASK);
    }

    /// Clear CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core1(&self) {
        self.clr_bits(Self::CORE1_MASK);
    }

    /// **CORE0** `[4]` (RW)
    ///
    /// If 1, BUSCTRL can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core0(&self) -> bool {
        (self.value.read_bits() & Self::CORE0_MASK) != 0
    }

    /// Set CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn set_core0(&self) {
        self.set_bits(Self::CORE0_MASK);
    }

    /// Clear CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core0(&self) {
        self.clr_bits(Self::CORE0_MASK);
    }

    /// **SP** `[3]` (RW)
    ///
    /// If 1, BUSCTRL can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub fn sp(&self) -> bool {
        (self.value.read_bits() & Self::SP_MASK) != 0
    }

    /// Set SP bit atomically (RW)
    #[inline(always)]
    pub fn set_sp(&self) {
        self.set_bits(Self::SP_MASK);
    }

    /// Clear SP bit atomically (RW)
    #[inline(always)]
    pub fn clr_sp(&self) {
        self.clr_bits(Self::SP_MASK);
    }

    /// **SU** `[2]` (RW)
    ///
    /// If 1, and SP is also set, BUSCTRL can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub fn su(&self) -> bool {
        (self.value.read_bits() & Self::SU_MASK) != 0
    }

    /// Set SU bit atomically (RW)
    #[inline(always)]
    pub fn set_su(&self) {
        self.set_bits(Self::SU_MASK);
    }

    /// Clear SU bit atomically (RW)
    #[inline(always)]
    pub fn clr_su(&self) {
        self.clr_bits(Self::SU_MASK);
    }

    /// **NSP** `[1]` (RW)
    ///
    /// If 1, BUSCTRL can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub fn nsp(&self) -> bool {
        (self.value.read_bits() & Self::NSP_MASK) != 0
    }

    /// Set NSP bit atomically (RW)
    #[inline(always)]
    pub fn set_nsp(&self) {
        self.set_bits(Self::NSP_MASK);
    }

    /// Clear NSP bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsp(&self) {
        self.clr_bits(Self::NSP_MASK);
    }

    /// **NSU** `[0]` (RW)
    ///
    /// If 1, and NSP is also set, BUSCTRL can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub fn nsu(&self) -> bool {
        (self.value.read_bits() & Self::NSU_MASK) != 0
    }

    /// Set NSU bit atomically (RW)
    #[inline(always)]
    pub fn set_nsu(&self) {
        self.set_bits(Self::NSU_MASK);
    }

    /// Clear NSU bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsu(&self) {
        self.clr_bits(Self::NSU_MASK);
    }
}

/// BUSCTRL value type for batch read/modify/write operations
///
/// Use `busctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `busctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BusctrlVal {
    bits: u32,
}

impl BusctrlVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x000000fc }
    }

    /// Read **DBG** `[7:7]`
    ///
    /// If 1, BUSCTRL can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        (self.bits & Busctrl::DBG_MASK) != 0
    }

    /// Read **DMA** `[6:6]`
    ///
    /// If 1, BUSCTRL can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        (self.bits & Busctrl::DMA_MASK) != 0
    }

    /// Read **CORE1** `[5:5]`
    ///
    /// If 1, BUSCTRL can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        (self.bits & Busctrl::CORE1_MASK) != 0
    }

    /// Read **CORE0** `[4:4]`
    ///
    /// If 1, BUSCTRL can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        (self.bits & Busctrl::CORE0_MASK) != 0
    }

    /// Read **SP** `[3:3]`
    ///
    /// If 1, BUSCTRL can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        (self.bits & Busctrl::SP_MASK) != 0
    }

    /// Read **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, BUSCTRL can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn su(&self) -> bool {
        (self.bits & Busctrl::SU_MASK) != 0
    }

    /// Read **NSP** `[1:1]`
    ///
    /// If 1, BUSCTRL can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        (self.bits & Busctrl::NSP_MASK) != 0
    }

    /// Read **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, BUSCTRL can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        (self.bits & Busctrl::NSU_MASK) != 0
    }

    /// Set **DBG** `[7:7]`
    ///
    /// If 1, BUSCTRL can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dbg(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Busctrl::DBG_MASK
            } else {
                self.bits & !Busctrl::DBG_MASK
            }
        }
    }

    /// Set **DMA** `[6:6]`
    ///
    /// If 1, BUSCTRL can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dma(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Busctrl::DMA_MASK
            } else {
                self.bits & !Busctrl::DMA_MASK
            }
        }
    }

    /// Set **CORE1** `[5:5]`
    ///
    /// If 1, BUSCTRL can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Busctrl::CORE1_MASK
            } else {
                self.bits & !Busctrl::CORE1_MASK
            }
        }
    }

    /// Set **CORE0** `[4:4]`
    ///
    /// If 1, BUSCTRL can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Busctrl::CORE0_MASK
            } else {
                self.bits & !Busctrl::CORE0_MASK
            }
        }
    }

    /// Set **SP** `[3:3]`
    ///
    /// If 1, BUSCTRL can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn with_sp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Busctrl::SP_MASK
            } else {
                self.bits & !Busctrl::SP_MASK
            }
        }
    }

    /// Set **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, BUSCTRL can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn with_su(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Busctrl::SU_MASK
            } else {
                self.bits & !Busctrl::SU_MASK
            }
        }
    }

    /// Set **NSP** `[1:1]`
    ///
    /// If 1, BUSCTRL can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn with_nsp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Busctrl::NSP_MASK
            } else {
                self.bits & !Busctrl::NSP_MASK
            }
        }
    }

    /// Set **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, BUSCTRL can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn with_nsu(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Busctrl::NSU_MASK
            } else {
                self.bits & !Busctrl::NSU_MASK
            }
        }
    }
}

impl Default for BusctrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Busctrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> BusctrlVal {
        BusctrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: BusctrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(BusctrlVal) -> BusctrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ADC0 Register
///
/// Control whether debugger, DMA, core 0 and core 1 can access ADC0, and at what security/privilege levels they can do so.
/// Defaults to Secure access from any master.
/// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
///
/// Address offset: 0x007c
/// Reset value: 0x000000fc
///
/// ## Fields
///
/// - **DBG** `[7]` (RW)
///
///   If 1, ADC0 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **DMA** `[6]` (RW)
///
///   If 1, ADC0 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE1** `[5]` (RW)
///
///   If 1, ADC0 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE0** `[4]` (RW)
///
///   If 1, ADC0 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **SP** `[3]` (RW)
///
///   If 1, ADC0 can be accessed from a Secure, Privileged context.
///
/// - **SU** `[2]` (RW)
///
///   If 1, and SP is also set, ADC0 can be accessed from a Secure, Unprivileged context.
///
/// - **NSP** `[1]` (RW)
///
///   If 1, ADC0 can be accessed from a Non-secure, Privileged context.
///
/// - **NSU** `[0]` (RW)
///
///   If 1, and NSP is also set, ADC0 can be accessed from a Non-secure, Unprivileged context.
///   This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
#[repr(transparent)]
pub struct Adc0 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Adc0 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Adc0 {
    // ======== Field Constants ========

    /// **DBG**
    ///
    /// If 1, ADC0 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const DBG_OFFSET: u32 = 7;
    pub const DBG_MASK: u32 = 0x00000080;
    pub const DBG: u32 = 1 << 7;

    /// **DMA**
    ///
    /// If 1, ADC0 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const DMA_OFFSET: u32 = 6;
    pub const DMA_MASK: u32 = 0x00000040;
    pub const DMA: u32 = 1 << 6;

    /// **CORE1**
    ///
    /// If 1, ADC0 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CORE1_OFFSET: u32 = 5;
    pub const CORE1_MASK: u32 = 0x00000020;
    pub const CORE1: u32 = 1 << 5;

    /// **CORE0**
    ///
    /// If 1, ADC0 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CORE0_OFFSET: u32 = 4;
    pub const CORE0_MASK: u32 = 0x00000010;
    pub const CORE0: u32 = 1 << 4;

    /// **SP**
    ///
    /// If 1, ADC0 can be accessed from a Secure, Privileged context.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SP_OFFSET: u32 = 3;
    pub const SP_MASK: u32 = 0x00000008;
    pub const SP: u32 = 1 << 3;

    /// **SU**
    ///
    /// If 1, and SP is also set, ADC0 can be accessed from a Secure, Unprivileged context.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SU_OFFSET: u32 = 2;
    pub const SU_MASK: u32 = 0x00000004;
    pub const SU: u32 = 1 << 2;

    /// **NSP**
    ///
    /// If 1, ADC0 can be accessed from a Non-secure, Privileged context.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const NSP_OFFSET: u32 = 1;
    pub const NSP_MASK: u32 = 0x00000002;
    pub const NSP: u32 = 1 << 1;

    /// **NSU**
    ///
    /// If 1, and NSP is also set, ADC0 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const NSU_OFFSET: u32 = 0;
    pub const NSU_MASK: u32 = 0x00000001;
    pub const NSU: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DBG** `[7]` (RW)
    ///
    /// If 1, ADC0 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dbg(&self) -> bool {
        (self.value.read_bits() & Self::DBG_MASK) != 0
    }

    /// Set DBG bit atomically (RW)
    #[inline(always)]
    pub fn set_dbg(&self) {
        self.set_bits(Self::DBG_MASK);
    }

    /// Clear DBG bit atomically (RW)
    #[inline(always)]
    pub fn clr_dbg(&self) {
        self.clr_bits(Self::DBG_MASK);
    }

    /// **DMA** `[6]` (RW)
    ///
    /// If 1, ADC0 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dma(&self) -> bool {
        (self.value.read_bits() & Self::DMA_MASK) != 0
    }

    /// Set DMA bit atomically (RW)
    #[inline(always)]
    pub fn set_dma(&self) {
        self.set_bits(Self::DMA_MASK);
    }

    /// Clear DMA bit atomically (RW)
    #[inline(always)]
    pub fn clr_dma(&self) {
        self.clr_bits(Self::DMA_MASK);
    }

    /// **CORE1** `[5]` (RW)
    ///
    /// If 1, ADC0 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core1(&self) -> bool {
        (self.value.read_bits() & Self::CORE1_MASK) != 0
    }

    /// Set CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn set_core1(&self) {
        self.set_bits(Self::CORE1_MASK);
    }

    /// Clear CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core1(&self) {
        self.clr_bits(Self::CORE1_MASK);
    }

    /// **CORE0** `[4]` (RW)
    ///
    /// If 1, ADC0 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core0(&self) -> bool {
        (self.value.read_bits() & Self::CORE0_MASK) != 0
    }

    /// Set CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn set_core0(&self) {
        self.set_bits(Self::CORE0_MASK);
    }

    /// Clear CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core0(&self) {
        self.clr_bits(Self::CORE0_MASK);
    }

    /// **SP** `[3]` (RW)
    ///
    /// If 1, ADC0 can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub fn sp(&self) -> bool {
        (self.value.read_bits() & Self::SP_MASK) != 0
    }

    /// Set SP bit atomically (RW)
    #[inline(always)]
    pub fn set_sp(&self) {
        self.set_bits(Self::SP_MASK);
    }

    /// Clear SP bit atomically (RW)
    #[inline(always)]
    pub fn clr_sp(&self) {
        self.clr_bits(Self::SP_MASK);
    }

    /// **SU** `[2]` (RW)
    ///
    /// If 1, and SP is also set, ADC0 can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub fn su(&self) -> bool {
        (self.value.read_bits() & Self::SU_MASK) != 0
    }

    /// Set SU bit atomically (RW)
    #[inline(always)]
    pub fn set_su(&self) {
        self.set_bits(Self::SU_MASK);
    }

    /// Clear SU bit atomically (RW)
    #[inline(always)]
    pub fn clr_su(&self) {
        self.clr_bits(Self::SU_MASK);
    }

    /// **NSP** `[1]` (RW)
    ///
    /// If 1, ADC0 can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub fn nsp(&self) -> bool {
        (self.value.read_bits() & Self::NSP_MASK) != 0
    }

    /// Set NSP bit atomically (RW)
    #[inline(always)]
    pub fn set_nsp(&self) {
        self.set_bits(Self::NSP_MASK);
    }

    /// Clear NSP bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsp(&self) {
        self.clr_bits(Self::NSP_MASK);
    }

    /// **NSU** `[0]` (RW)
    ///
    /// If 1, and NSP is also set, ADC0 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub fn nsu(&self) -> bool {
        (self.value.read_bits() & Self::NSU_MASK) != 0
    }

    /// Set NSU bit atomically (RW)
    #[inline(always)]
    pub fn set_nsu(&self) {
        self.set_bits(Self::NSU_MASK);
    }

    /// Clear NSU bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsu(&self) {
        self.clr_bits(Self::NSU_MASK);
    }
}

/// ADC0 value type for batch read/modify/write operations
///
/// Use `adc0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `adc0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Adc0Val {
    bits: u32,
}

impl Adc0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x000000fc }
    }

    /// Read **DBG** `[7:7]`
    ///
    /// If 1, ADC0 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        (self.bits & Adc0::DBG_MASK) != 0
    }

    /// Read **DMA** `[6:6]`
    ///
    /// If 1, ADC0 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        (self.bits & Adc0::DMA_MASK) != 0
    }

    /// Read **CORE1** `[5:5]`
    ///
    /// If 1, ADC0 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        (self.bits & Adc0::CORE1_MASK) != 0
    }

    /// Read **CORE0** `[4:4]`
    ///
    /// If 1, ADC0 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        (self.bits & Adc0::CORE0_MASK) != 0
    }

    /// Read **SP** `[3:3]`
    ///
    /// If 1, ADC0 can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        (self.bits & Adc0::SP_MASK) != 0
    }

    /// Read **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, ADC0 can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn su(&self) -> bool {
        (self.bits & Adc0::SU_MASK) != 0
    }

    /// Read **NSP** `[1:1]`
    ///
    /// If 1, ADC0 can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        (self.bits & Adc0::NSP_MASK) != 0
    }

    /// Read **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, ADC0 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        (self.bits & Adc0::NSU_MASK) != 0
    }

    /// Set **DBG** `[7:7]`
    ///
    /// If 1, ADC0 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dbg(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Adc0::DBG_MASK
            } else {
                self.bits & !Adc0::DBG_MASK
            }
        }
    }

    /// Set **DMA** `[6:6]`
    ///
    /// If 1, ADC0 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dma(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Adc0::DMA_MASK
            } else {
                self.bits & !Adc0::DMA_MASK
            }
        }
    }

    /// Set **CORE1** `[5:5]`
    ///
    /// If 1, ADC0 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Adc0::CORE1_MASK
            } else {
                self.bits & !Adc0::CORE1_MASK
            }
        }
    }

    /// Set **CORE0** `[4:4]`
    ///
    /// If 1, ADC0 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Adc0::CORE0_MASK
            } else {
                self.bits & !Adc0::CORE0_MASK
            }
        }
    }

    /// Set **SP** `[3:3]`
    ///
    /// If 1, ADC0 can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn with_sp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Adc0::SP_MASK
            } else {
                self.bits & !Adc0::SP_MASK
            }
        }
    }

    /// Set **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, ADC0 can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn with_su(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Adc0::SU_MASK
            } else {
                self.bits & !Adc0::SU_MASK
            }
        }
    }

    /// Set **NSP** `[1:1]`
    ///
    /// If 1, ADC0 can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn with_nsp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Adc0::NSP_MASK
            } else {
                self.bits & !Adc0::NSP_MASK
            }
        }
    }

    /// Set **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, ADC0 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn with_nsu(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Adc0::NSU_MASK
            } else {
                self.bits & !Adc0::NSU_MASK
            }
        }
    }
}

impl Default for Adc0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Adc0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Adc0Val {
        Adc0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Adc0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Adc0Val) -> Adc0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// HSTX Register
///
/// Control whether debugger, DMA, core 0 and core 1 can access HSTX, and at what security/privilege levels they can do so.
/// Defaults to Secure access from any master.
/// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
///
/// Address offset: 0x0080
/// Reset value: 0x000000fc
///
/// ## Fields
///
/// - **DBG** `[7]` (RW)
///
///   If 1, HSTX can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **DMA** `[6]` (RW)
///
///   If 1, HSTX can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE1** `[5]` (RW)
///
///   If 1, HSTX can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE0** `[4]` (RW)
///
///   If 1, HSTX can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **SP** `[3]` (RW)
///
///   If 1, HSTX can be accessed from a Secure, Privileged context.
///
/// - **SU** `[2]` (RW)
///
///   If 1, and SP is also set, HSTX can be accessed from a Secure, Unprivileged context.
///
/// - **NSP** `[1]` (RW)
///
///   If 1, HSTX can be accessed from a Non-secure, Privileged context.
///
/// - **NSU** `[0]` (RW)
///
///   If 1, and NSP is also set, HSTX can be accessed from a Non-secure, Unprivileged context.
///   This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
#[repr(transparent)]
pub struct Hstx {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Hstx {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Hstx {
    // ======== Field Constants ========

    /// **DBG**
    ///
    /// If 1, HSTX can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const DBG_OFFSET: u32 = 7;
    pub const DBG_MASK: u32 = 0x00000080;
    pub const DBG: u32 = 1 << 7;

    /// **DMA**
    ///
    /// If 1, HSTX can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const DMA_OFFSET: u32 = 6;
    pub const DMA_MASK: u32 = 0x00000040;
    pub const DMA: u32 = 1 << 6;

    /// **CORE1**
    ///
    /// If 1, HSTX can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CORE1_OFFSET: u32 = 5;
    pub const CORE1_MASK: u32 = 0x00000020;
    pub const CORE1: u32 = 1 << 5;

    /// **CORE0**
    ///
    /// If 1, HSTX can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CORE0_OFFSET: u32 = 4;
    pub const CORE0_MASK: u32 = 0x00000010;
    pub const CORE0: u32 = 1 << 4;

    /// **SP**
    ///
    /// If 1, HSTX can be accessed from a Secure, Privileged context.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SP_OFFSET: u32 = 3;
    pub const SP_MASK: u32 = 0x00000008;
    pub const SP: u32 = 1 << 3;

    /// **SU**
    ///
    /// If 1, and SP is also set, HSTX can be accessed from a Secure, Unprivileged context.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SU_OFFSET: u32 = 2;
    pub const SU_MASK: u32 = 0x00000004;
    pub const SU: u32 = 1 << 2;

    /// **NSP**
    ///
    /// If 1, HSTX can be accessed from a Non-secure, Privileged context.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const NSP_OFFSET: u32 = 1;
    pub const NSP_MASK: u32 = 0x00000002;
    pub const NSP: u32 = 1 << 1;

    /// **NSU**
    ///
    /// If 1, and NSP is also set, HSTX can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const NSU_OFFSET: u32 = 0;
    pub const NSU_MASK: u32 = 0x00000001;
    pub const NSU: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DBG** `[7]` (RW)
    ///
    /// If 1, HSTX can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dbg(&self) -> bool {
        (self.value.read_bits() & Self::DBG_MASK) != 0
    }

    /// Set DBG bit atomically (RW)
    #[inline(always)]
    pub fn set_dbg(&self) {
        self.set_bits(Self::DBG_MASK);
    }

    /// Clear DBG bit atomically (RW)
    #[inline(always)]
    pub fn clr_dbg(&self) {
        self.clr_bits(Self::DBG_MASK);
    }

    /// **DMA** `[6]` (RW)
    ///
    /// If 1, HSTX can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dma(&self) -> bool {
        (self.value.read_bits() & Self::DMA_MASK) != 0
    }

    /// Set DMA bit atomically (RW)
    #[inline(always)]
    pub fn set_dma(&self) {
        self.set_bits(Self::DMA_MASK);
    }

    /// Clear DMA bit atomically (RW)
    #[inline(always)]
    pub fn clr_dma(&self) {
        self.clr_bits(Self::DMA_MASK);
    }

    /// **CORE1** `[5]` (RW)
    ///
    /// If 1, HSTX can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core1(&self) -> bool {
        (self.value.read_bits() & Self::CORE1_MASK) != 0
    }

    /// Set CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn set_core1(&self) {
        self.set_bits(Self::CORE1_MASK);
    }

    /// Clear CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core1(&self) {
        self.clr_bits(Self::CORE1_MASK);
    }

    /// **CORE0** `[4]` (RW)
    ///
    /// If 1, HSTX can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core0(&self) -> bool {
        (self.value.read_bits() & Self::CORE0_MASK) != 0
    }

    /// Set CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn set_core0(&self) {
        self.set_bits(Self::CORE0_MASK);
    }

    /// Clear CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core0(&self) {
        self.clr_bits(Self::CORE0_MASK);
    }

    /// **SP** `[3]` (RW)
    ///
    /// If 1, HSTX can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub fn sp(&self) -> bool {
        (self.value.read_bits() & Self::SP_MASK) != 0
    }

    /// Set SP bit atomically (RW)
    #[inline(always)]
    pub fn set_sp(&self) {
        self.set_bits(Self::SP_MASK);
    }

    /// Clear SP bit atomically (RW)
    #[inline(always)]
    pub fn clr_sp(&self) {
        self.clr_bits(Self::SP_MASK);
    }

    /// **SU** `[2]` (RW)
    ///
    /// If 1, and SP is also set, HSTX can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub fn su(&self) -> bool {
        (self.value.read_bits() & Self::SU_MASK) != 0
    }

    /// Set SU bit atomically (RW)
    #[inline(always)]
    pub fn set_su(&self) {
        self.set_bits(Self::SU_MASK);
    }

    /// Clear SU bit atomically (RW)
    #[inline(always)]
    pub fn clr_su(&self) {
        self.clr_bits(Self::SU_MASK);
    }

    /// **NSP** `[1]` (RW)
    ///
    /// If 1, HSTX can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub fn nsp(&self) -> bool {
        (self.value.read_bits() & Self::NSP_MASK) != 0
    }

    /// Set NSP bit atomically (RW)
    #[inline(always)]
    pub fn set_nsp(&self) {
        self.set_bits(Self::NSP_MASK);
    }

    /// Clear NSP bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsp(&self) {
        self.clr_bits(Self::NSP_MASK);
    }

    /// **NSU** `[0]` (RW)
    ///
    /// If 1, and NSP is also set, HSTX can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub fn nsu(&self) -> bool {
        (self.value.read_bits() & Self::NSU_MASK) != 0
    }

    /// Set NSU bit atomically (RW)
    #[inline(always)]
    pub fn set_nsu(&self) {
        self.set_bits(Self::NSU_MASK);
    }

    /// Clear NSU bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsu(&self) {
        self.clr_bits(Self::NSU_MASK);
    }
}

/// HSTX value type for batch read/modify/write operations
///
/// Use `hstx.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `hstx.write_val(val)`.
#[derive(Clone, Copy)]
pub struct HstxVal {
    bits: u32,
}

impl HstxVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x000000fc }
    }

    /// Read **DBG** `[7:7]`
    ///
    /// If 1, HSTX can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        (self.bits & Hstx::DBG_MASK) != 0
    }

    /// Read **DMA** `[6:6]`
    ///
    /// If 1, HSTX can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        (self.bits & Hstx::DMA_MASK) != 0
    }

    /// Read **CORE1** `[5:5]`
    ///
    /// If 1, HSTX can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        (self.bits & Hstx::CORE1_MASK) != 0
    }

    /// Read **CORE0** `[4:4]`
    ///
    /// If 1, HSTX can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        (self.bits & Hstx::CORE0_MASK) != 0
    }

    /// Read **SP** `[3:3]`
    ///
    /// If 1, HSTX can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        (self.bits & Hstx::SP_MASK) != 0
    }

    /// Read **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, HSTX can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn su(&self) -> bool {
        (self.bits & Hstx::SU_MASK) != 0
    }

    /// Read **NSP** `[1:1]`
    ///
    /// If 1, HSTX can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        (self.bits & Hstx::NSP_MASK) != 0
    }

    /// Read **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, HSTX can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        (self.bits & Hstx::NSU_MASK) != 0
    }

    /// Set **DBG** `[7:7]`
    ///
    /// If 1, HSTX can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dbg(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Hstx::DBG_MASK
            } else {
                self.bits & !Hstx::DBG_MASK
            }
        }
    }

    /// Set **DMA** `[6:6]`
    ///
    /// If 1, HSTX can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dma(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Hstx::DMA_MASK
            } else {
                self.bits & !Hstx::DMA_MASK
            }
        }
    }

    /// Set **CORE1** `[5:5]`
    ///
    /// If 1, HSTX can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Hstx::CORE1_MASK
            } else {
                self.bits & !Hstx::CORE1_MASK
            }
        }
    }

    /// Set **CORE0** `[4:4]`
    ///
    /// If 1, HSTX can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Hstx::CORE0_MASK
            } else {
                self.bits & !Hstx::CORE0_MASK
            }
        }
    }

    /// Set **SP** `[3:3]`
    ///
    /// If 1, HSTX can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn with_sp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Hstx::SP_MASK
            } else {
                self.bits & !Hstx::SP_MASK
            }
        }
    }

    /// Set **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, HSTX can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn with_su(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Hstx::SU_MASK
            } else {
                self.bits & !Hstx::SU_MASK
            }
        }
    }

    /// Set **NSP** `[1:1]`
    ///
    /// If 1, HSTX can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn with_nsp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Hstx::NSP_MASK
            } else {
                self.bits & !Hstx::NSP_MASK
            }
        }
    }

    /// Set **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, HSTX can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn with_nsu(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Hstx::NSU_MASK
            } else {
                self.bits & !Hstx::NSU_MASK
            }
        }
    }
}

impl Default for HstxVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Hstx {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> HstxVal {
        HstxVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: HstxVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(HstxVal) -> HstxVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// I2C0 Register
///
/// Control whether debugger, DMA, core 0 and core 1 can access I2C0, and at what security/privilege levels they can do so.
/// Defaults to Secure access from any master.
/// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
///
/// Address offset: 0x0084
/// Reset value: 0x000000fc
///
/// ## Fields
///
/// - **DBG** `[7]` (RW)
///
///   If 1, I2C0 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **DMA** `[6]` (RW)
///
///   If 1, I2C0 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE1** `[5]` (RW)
///
///   If 1, I2C0 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE0** `[4]` (RW)
///
///   If 1, I2C0 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **SP** `[3]` (RW)
///
///   If 1, I2C0 can be accessed from a Secure, Privileged context.
///
/// - **SU** `[2]` (RW)
///
///   If 1, and SP is also set, I2C0 can be accessed from a Secure, Unprivileged context.
///
/// - **NSP** `[1]` (RW)
///
///   If 1, I2C0 can be accessed from a Non-secure, Privileged context.
///
/// - **NSU** `[0]` (RW)
///
///   If 1, and NSP is also set, I2C0 can be accessed from a Non-secure, Unprivileged context.
///   This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
#[repr(transparent)]
pub struct I2c0 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for I2c0 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl I2c0 {
    // ======== Field Constants ========

    /// **DBG**
    ///
    /// If 1, I2C0 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const DBG_OFFSET: u32 = 7;
    pub const DBG_MASK: u32 = 0x00000080;
    pub const DBG: u32 = 1 << 7;

    /// **DMA**
    ///
    /// If 1, I2C0 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const DMA_OFFSET: u32 = 6;
    pub const DMA_MASK: u32 = 0x00000040;
    pub const DMA: u32 = 1 << 6;

    /// **CORE1**
    ///
    /// If 1, I2C0 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CORE1_OFFSET: u32 = 5;
    pub const CORE1_MASK: u32 = 0x00000020;
    pub const CORE1: u32 = 1 << 5;

    /// **CORE0**
    ///
    /// If 1, I2C0 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CORE0_OFFSET: u32 = 4;
    pub const CORE0_MASK: u32 = 0x00000010;
    pub const CORE0: u32 = 1 << 4;

    /// **SP**
    ///
    /// If 1, I2C0 can be accessed from a Secure, Privileged context.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SP_OFFSET: u32 = 3;
    pub const SP_MASK: u32 = 0x00000008;
    pub const SP: u32 = 1 << 3;

    /// **SU**
    ///
    /// If 1, and SP is also set, I2C0 can be accessed from a Secure, Unprivileged context.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SU_OFFSET: u32 = 2;
    pub const SU_MASK: u32 = 0x00000004;
    pub const SU: u32 = 1 << 2;

    /// **NSP**
    ///
    /// If 1, I2C0 can be accessed from a Non-secure, Privileged context.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const NSP_OFFSET: u32 = 1;
    pub const NSP_MASK: u32 = 0x00000002;
    pub const NSP: u32 = 1 << 1;

    /// **NSU**
    ///
    /// If 1, and NSP is also set, I2C0 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const NSU_OFFSET: u32 = 0;
    pub const NSU_MASK: u32 = 0x00000001;
    pub const NSU: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DBG** `[7]` (RW)
    ///
    /// If 1, I2C0 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dbg(&self) -> bool {
        (self.value.read_bits() & Self::DBG_MASK) != 0
    }

    /// Set DBG bit atomically (RW)
    #[inline(always)]
    pub fn set_dbg(&self) {
        self.set_bits(Self::DBG_MASK);
    }

    /// Clear DBG bit atomically (RW)
    #[inline(always)]
    pub fn clr_dbg(&self) {
        self.clr_bits(Self::DBG_MASK);
    }

    /// **DMA** `[6]` (RW)
    ///
    /// If 1, I2C0 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dma(&self) -> bool {
        (self.value.read_bits() & Self::DMA_MASK) != 0
    }

    /// Set DMA bit atomically (RW)
    #[inline(always)]
    pub fn set_dma(&self) {
        self.set_bits(Self::DMA_MASK);
    }

    /// Clear DMA bit atomically (RW)
    #[inline(always)]
    pub fn clr_dma(&self) {
        self.clr_bits(Self::DMA_MASK);
    }

    /// **CORE1** `[5]` (RW)
    ///
    /// If 1, I2C0 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core1(&self) -> bool {
        (self.value.read_bits() & Self::CORE1_MASK) != 0
    }

    /// Set CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn set_core1(&self) {
        self.set_bits(Self::CORE1_MASK);
    }

    /// Clear CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core1(&self) {
        self.clr_bits(Self::CORE1_MASK);
    }

    /// **CORE0** `[4]` (RW)
    ///
    /// If 1, I2C0 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core0(&self) -> bool {
        (self.value.read_bits() & Self::CORE0_MASK) != 0
    }

    /// Set CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn set_core0(&self) {
        self.set_bits(Self::CORE0_MASK);
    }

    /// Clear CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core0(&self) {
        self.clr_bits(Self::CORE0_MASK);
    }

    /// **SP** `[3]` (RW)
    ///
    /// If 1, I2C0 can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub fn sp(&self) -> bool {
        (self.value.read_bits() & Self::SP_MASK) != 0
    }

    /// Set SP bit atomically (RW)
    #[inline(always)]
    pub fn set_sp(&self) {
        self.set_bits(Self::SP_MASK);
    }

    /// Clear SP bit atomically (RW)
    #[inline(always)]
    pub fn clr_sp(&self) {
        self.clr_bits(Self::SP_MASK);
    }

    /// **SU** `[2]` (RW)
    ///
    /// If 1, and SP is also set, I2C0 can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub fn su(&self) -> bool {
        (self.value.read_bits() & Self::SU_MASK) != 0
    }

    /// Set SU bit atomically (RW)
    #[inline(always)]
    pub fn set_su(&self) {
        self.set_bits(Self::SU_MASK);
    }

    /// Clear SU bit atomically (RW)
    #[inline(always)]
    pub fn clr_su(&self) {
        self.clr_bits(Self::SU_MASK);
    }

    /// **NSP** `[1]` (RW)
    ///
    /// If 1, I2C0 can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub fn nsp(&self) -> bool {
        (self.value.read_bits() & Self::NSP_MASK) != 0
    }

    /// Set NSP bit atomically (RW)
    #[inline(always)]
    pub fn set_nsp(&self) {
        self.set_bits(Self::NSP_MASK);
    }

    /// Clear NSP bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsp(&self) {
        self.clr_bits(Self::NSP_MASK);
    }

    /// **NSU** `[0]` (RW)
    ///
    /// If 1, and NSP is also set, I2C0 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub fn nsu(&self) -> bool {
        (self.value.read_bits() & Self::NSU_MASK) != 0
    }

    /// Set NSU bit atomically (RW)
    #[inline(always)]
    pub fn set_nsu(&self) {
        self.set_bits(Self::NSU_MASK);
    }

    /// Clear NSU bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsu(&self) {
        self.clr_bits(Self::NSU_MASK);
    }
}

/// I2C0 value type for batch read/modify/write operations
///
/// Use `i2c0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `i2c0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct I2c0Val {
    bits: u32,
}

impl I2c0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x000000fc }
    }

    /// Read **DBG** `[7:7]`
    ///
    /// If 1, I2C0 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        (self.bits & I2c0::DBG_MASK) != 0
    }

    /// Read **DMA** `[6:6]`
    ///
    /// If 1, I2C0 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        (self.bits & I2c0::DMA_MASK) != 0
    }

    /// Read **CORE1** `[5:5]`
    ///
    /// If 1, I2C0 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        (self.bits & I2c0::CORE1_MASK) != 0
    }

    /// Read **CORE0** `[4:4]`
    ///
    /// If 1, I2C0 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        (self.bits & I2c0::CORE0_MASK) != 0
    }

    /// Read **SP** `[3:3]`
    ///
    /// If 1, I2C0 can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        (self.bits & I2c0::SP_MASK) != 0
    }

    /// Read **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, I2C0 can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn su(&self) -> bool {
        (self.bits & I2c0::SU_MASK) != 0
    }

    /// Read **NSP** `[1:1]`
    ///
    /// If 1, I2C0 can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        (self.bits & I2c0::NSP_MASK) != 0
    }

    /// Read **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, I2C0 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        (self.bits & I2c0::NSU_MASK) != 0
    }

    /// Set **DBG** `[7:7]`
    ///
    /// If 1, I2C0 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dbg(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | I2c0::DBG_MASK
            } else {
                self.bits & !I2c0::DBG_MASK
            }
        }
    }

    /// Set **DMA** `[6:6]`
    ///
    /// If 1, I2C0 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dma(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | I2c0::DMA_MASK
            } else {
                self.bits & !I2c0::DMA_MASK
            }
        }
    }

    /// Set **CORE1** `[5:5]`
    ///
    /// If 1, I2C0 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | I2c0::CORE1_MASK
            } else {
                self.bits & !I2c0::CORE1_MASK
            }
        }
    }

    /// Set **CORE0** `[4:4]`
    ///
    /// If 1, I2C0 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | I2c0::CORE0_MASK
            } else {
                self.bits & !I2c0::CORE0_MASK
            }
        }
    }

    /// Set **SP** `[3:3]`
    ///
    /// If 1, I2C0 can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn with_sp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | I2c0::SP_MASK
            } else {
                self.bits & !I2c0::SP_MASK
            }
        }
    }

    /// Set **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, I2C0 can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn with_su(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | I2c0::SU_MASK
            } else {
                self.bits & !I2c0::SU_MASK
            }
        }
    }

    /// Set **NSP** `[1:1]`
    ///
    /// If 1, I2C0 can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn with_nsp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | I2c0::NSP_MASK
            } else {
                self.bits & !I2c0::NSP_MASK
            }
        }
    }

    /// Set **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, I2C0 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn with_nsu(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | I2c0::NSU_MASK
            } else {
                self.bits & !I2c0::NSU_MASK
            }
        }
    }
}

impl Default for I2c0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl I2c0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> I2c0Val {
        I2c0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: I2c0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(I2c0Val) -> I2c0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// I2C1 Register
///
/// Control whether debugger, DMA, core 0 and core 1 can access I2C1, and at what security/privilege levels they can do so.
/// Defaults to Secure access from any master.
/// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
///
/// Address offset: 0x0088
/// Reset value: 0x000000fc
///
/// ## Fields
///
/// - **DBG** `[7]` (RW)
///
///   If 1, I2C1 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **DMA** `[6]` (RW)
///
///   If 1, I2C1 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE1** `[5]` (RW)
///
///   If 1, I2C1 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE0** `[4]` (RW)
///
///   If 1, I2C1 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **SP** `[3]` (RW)
///
///   If 1, I2C1 can be accessed from a Secure, Privileged context.
///
/// - **SU** `[2]` (RW)
///
///   If 1, and SP is also set, I2C1 can be accessed from a Secure, Unprivileged context.
///
/// - **NSP** `[1]` (RW)
///
///   If 1, I2C1 can be accessed from a Non-secure, Privileged context.
///
/// - **NSU** `[0]` (RW)
///
///   If 1, and NSP is also set, I2C1 can be accessed from a Non-secure, Unprivileged context.
///   This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
#[repr(transparent)]
pub struct I2c1 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for I2c1 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl I2c1 {
    // ======== Field Constants ========

    /// **DBG**
    ///
    /// If 1, I2C1 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const DBG_OFFSET: u32 = 7;
    pub const DBG_MASK: u32 = 0x00000080;
    pub const DBG: u32 = 1 << 7;

    /// **DMA**
    ///
    /// If 1, I2C1 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const DMA_OFFSET: u32 = 6;
    pub const DMA_MASK: u32 = 0x00000040;
    pub const DMA: u32 = 1 << 6;

    /// **CORE1**
    ///
    /// If 1, I2C1 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CORE1_OFFSET: u32 = 5;
    pub const CORE1_MASK: u32 = 0x00000020;
    pub const CORE1: u32 = 1 << 5;

    /// **CORE0**
    ///
    /// If 1, I2C1 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CORE0_OFFSET: u32 = 4;
    pub const CORE0_MASK: u32 = 0x00000010;
    pub const CORE0: u32 = 1 << 4;

    /// **SP**
    ///
    /// If 1, I2C1 can be accessed from a Secure, Privileged context.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SP_OFFSET: u32 = 3;
    pub const SP_MASK: u32 = 0x00000008;
    pub const SP: u32 = 1 << 3;

    /// **SU**
    ///
    /// If 1, and SP is also set, I2C1 can be accessed from a Secure, Unprivileged context.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SU_OFFSET: u32 = 2;
    pub const SU_MASK: u32 = 0x00000004;
    pub const SU: u32 = 1 << 2;

    /// **NSP**
    ///
    /// If 1, I2C1 can be accessed from a Non-secure, Privileged context.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const NSP_OFFSET: u32 = 1;
    pub const NSP_MASK: u32 = 0x00000002;
    pub const NSP: u32 = 1 << 1;

    /// **NSU**
    ///
    /// If 1, and NSP is also set, I2C1 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const NSU_OFFSET: u32 = 0;
    pub const NSU_MASK: u32 = 0x00000001;
    pub const NSU: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DBG** `[7]` (RW)
    ///
    /// If 1, I2C1 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dbg(&self) -> bool {
        (self.value.read_bits() & Self::DBG_MASK) != 0
    }

    /// Set DBG bit atomically (RW)
    #[inline(always)]
    pub fn set_dbg(&self) {
        self.set_bits(Self::DBG_MASK);
    }

    /// Clear DBG bit atomically (RW)
    #[inline(always)]
    pub fn clr_dbg(&self) {
        self.clr_bits(Self::DBG_MASK);
    }

    /// **DMA** `[6]` (RW)
    ///
    /// If 1, I2C1 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dma(&self) -> bool {
        (self.value.read_bits() & Self::DMA_MASK) != 0
    }

    /// Set DMA bit atomically (RW)
    #[inline(always)]
    pub fn set_dma(&self) {
        self.set_bits(Self::DMA_MASK);
    }

    /// Clear DMA bit atomically (RW)
    #[inline(always)]
    pub fn clr_dma(&self) {
        self.clr_bits(Self::DMA_MASK);
    }

    /// **CORE1** `[5]` (RW)
    ///
    /// If 1, I2C1 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core1(&self) -> bool {
        (self.value.read_bits() & Self::CORE1_MASK) != 0
    }

    /// Set CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn set_core1(&self) {
        self.set_bits(Self::CORE1_MASK);
    }

    /// Clear CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core1(&self) {
        self.clr_bits(Self::CORE1_MASK);
    }

    /// **CORE0** `[4]` (RW)
    ///
    /// If 1, I2C1 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core0(&self) -> bool {
        (self.value.read_bits() & Self::CORE0_MASK) != 0
    }

    /// Set CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn set_core0(&self) {
        self.set_bits(Self::CORE0_MASK);
    }

    /// Clear CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core0(&self) {
        self.clr_bits(Self::CORE0_MASK);
    }

    /// **SP** `[3]` (RW)
    ///
    /// If 1, I2C1 can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub fn sp(&self) -> bool {
        (self.value.read_bits() & Self::SP_MASK) != 0
    }

    /// Set SP bit atomically (RW)
    #[inline(always)]
    pub fn set_sp(&self) {
        self.set_bits(Self::SP_MASK);
    }

    /// Clear SP bit atomically (RW)
    #[inline(always)]
    pub fn clr_sp(&self) {
        self.clr_bits(Self::SP_MASK);
    }

    /// **SU** `[2]` (RW)
    ///
    /// If 1, and SP is also set, I2C1 can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub fn su(&self) -> bool {
        (self.value.read_bits() & Self::SU_MASK) != 0
    }

    /// Set SU bit atomically (RW)
    #[inline(always)]
    pub fn set_su(&self) {
        self.set_bits(Self::SU_MASK);
    }

    /// Clear SU bit atomically (RW)
    #[inline(always)]
    pub fn clr_su(&self) {
        self.clr_bits(Self::SU_MASK);
    }

    /// **NSP** `[1]` (RW)
    ///
    /// If 1, I2C1 can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub fn nsp(&self) -> bool {
        (self.value.read_bits() & Self::NSP_MASK) != 0
    }

    /// Set NSP bit atomically (RW)
    #[inline(always)]
    pub fn set_nsp(&self) {
        self.set_bits(Self::NSP_MASK);
    }

    /// Clear NSP bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsp(&self) {
        self.clr_bits(Self::NSP_MASK);
    }

    /// **NSU** `[0]` (RW)
    ///
    /// If 1, and NSP is also set, I2C1 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub fn nsu(&self) -> bool {
        (self.value.read_bits() & Self::NSU_MASK) != 0
    }

    /// Set NSU bit atomically (RW)
    #[inline(always)]
    pub fn set_nsu(&self) {
        self.set_bits(Self::NSU_MASK);
    }

    /// Clear NSU bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsu(&self) {
        self.clr_bits(Self::NSU_MASK);
    }
}

/// I2C1 value type for batch read/modify/write operations
///
/// Use `i2c1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `i2c1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct I2c1Val {
    bits: u32,
}

impl I2c1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x000000fc }
    }

    /// Read **DBG** `[7:7]`
    ///
    /// If 1, I2C1 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        (self.bits & I2c1::DBG_MASK) != 0
    }

    /// Read **DMA** `[6:6]`
    ///
    /// If 1, I2C1 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        (self.bits & I2c1::DMA_MASK) != 0
    }

    /// Read **CORE1** `[5:5]`
    ///
    /// If 1, I2C1 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        (self.bits & I2c1::CORE1_MASK) != 0
    }

    /// Read **CORE0** `[4:4]`
    ///
    /// If 1, I2C1 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        (self.bits & I2c1::CORE0_MASK) != 0
    }

    /// Read **SP** `[3:3]`
    ///
    /// If 1, I2C1 can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        (self.bits & I2c1::SP_MASK) != 0
    }

    /// Read **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, I2C1 can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn su(&self) -> bool {
        (self.bits & I2c1::SU_MASK) != 0
    }

    /// Read **NSP** `[1:1]`
    ///
    /// If 1, I2C1 can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        (self.bits & I2c1::NSP_MASK) != 0
    }

    /// Read **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, I2C1 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        (self.bits & I2c1::NSU_MASK) != 0
    }

    /// Set **DBG** `[7:7]`
    ///
    /// If 1, I2C1 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dbg(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | I2c1::DBG_MASK
            } else {
                self.bits & !I2c1::DBG_MASK
            }
        }
    }

    /// Set **DMA** `[6:6]`
    ///
    /// If 1, I2C1 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dma(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | I2c1::DMA_MASK
            } else {
                self.bits & !I2c1::DMA_MASK
            }
        }
    }

    /// Set **CORE1** `[5:5]`
    ///
    /// If 1, I2C1 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | I2c1::CORE1_MASK
            } else {
                self.bits & !I2c1::CORE1_MASK
            }
        }
    }

    /// Set **CORE0** `[4:4]`
    ///
    /// If 1, I2C1 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | I2c1::CORE0_MASK
            } else {
                self.bits & !I2c1::CORE0_MASK
            }
        }
    }

    /// Set **SP** `[3:3]`
    ///
    /// If 1, I2C1 can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn with_sp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | I2c1::SP_MASK
            } else {
                self.bits & !I2c1::SP_MASK
            }
        }
    }

    /// Set **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, I2C1 can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn with_su(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | I2c1::SU_MASK
            } else {
                self.bits & !I2c1::SU_MASK
            }
        }
    }

    /// Set **NSP** `[1:1]`
    ///
    /// If 1, I2C1 can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn with_nsp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | I2c1::NSP_MASK
            } else {
                self.bits & !I2c1::NSP_MASK
            }
        }
    }

    /// Set **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, I2C1 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn with_nsu(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | I2c1::NSU_MASK
            } else {
                self.bits & !I2c1::NSU_MASK
            }
        }
    }
}

impl Default for I2c1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl I2c1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> I2c1Val {
        I2c1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: I2c1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(I2c1Val) -> I2c1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PWM Register
///
/// Control whether debugger, DMA, core 0 and core 1 can access PWM, and at what security/privilege levels they can do so.
/// Defaults to Secure access from any master.
/// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
///
/// Address offset: 0x008c
/// Reset value: 0x000000fc
///
/// ## Fields
///
/// - **DBG** `[7]` (RW)
///
///   If 1, PWM can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **DMA** `[6]` (RW)
///
///   If 1, PWM can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE1** `[5]` (RW)
///
///   If 1, PWM can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE0** `[4]` (RW)
///
///   If 1, PWM can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **SP** `[3]` (RW)
///
///   If 1, PWM can be accessed from a Secure, Privileged context.
///
/// - **SU** `[2]` (RW)
///
///   If 1, and SP is also set, PWM can be accessed from a Secure, Unprivileged context.
///
/// - **NSP** `[1]` (RW)
///
///   If 1, PWM can be accessed from a Non-secure, Privileged context.
///
/// - **NSU** `[0]` (RW)
///
///   If 1, and NSP is also set, PWM can be accessed from a Non-secure, Unprivileged context.
///   This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
#[repr(transparent)]
pub struct Pwm {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Pwm {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Pwm {
    // ======== Field Constants ========

    /// **DBG**
    ///
    /// If 1, PWM can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const DBG_OFFSET: u32 = 7;
    pub const DBG_MASK: u32 = 0x00000080;
    pub const DBG: u32 = 1 << 7;

    /// **DMA**
    ///
    /// If 1, PWM can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const DMA_OFFSET: u32 = 6;
    pub const DMA_MASK: u32 = 0x00000040;
    pub const DMA: u32 = 1 << 6;

    /// **CORE1**
    ///
    /// If 1, PWM can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CORE1_OFFSET: u32 = 5;
    pub const CORE1_MASK: u32 = 0x00000020;
    pub const CORE1: u32 = 1 << 5;

    /// **CORE0**
    ///
    /// If 1, PWM can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CORE0_OFFSET: u32 = 4;
    pub const CORE0_MASK: u32 = 0x00000010;
    pub const CORE0: u32 = 1 << 4;

    /// **SP**
    ///
    /// If 1, PWM can be accessed from a Secure, Privileged context.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SP_OFFSET: u32 = 3;
    pub const SP_MASK: u32 = 0x00000008;
    pub const SP: u32 = 1 << 3;

    /// **SU**
    ///
    /// If 1, and SP is also set, PWM can be accessed from a Secure, Unprivileged context.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SU_OFFSET: u32 = 2;
    pub const SU_MASK: u32 = 0x00000004;
    pub const SU: u32 = 1 << 2;

    /// **NSP**
    ///
    /// If 1, PWM can be accessed from a Non-secure, Privileged context.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const NSP_OFFSET: u32 = 1;
    pub const NSP_MASK: u32 = 0x00000002;
    pub const NSP: u32 = 1 << 1;

    /// **NSU**
    ///
    /// If 1, and NSP is also set, PWM can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const NSU_OFFSET: u32 = 0;
    pub const NSU_MASK: u32 = 0x00000001;
    pub const NSU: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DBG** `[7]` (RW)
    ///
    /// If 1, PWM can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dbg(&self) -> bool {
        (self.value.read_bits() & Self::DBG_MASK) != 0
    }

    /// Set DBG bit atomically (RW)
    #[inline(always)]
    pub fn set_dbg(&self) {
        self.set_bits(Self::DBG_MASK);
    }

    /// Clear DBG bit atomically (RW)
    #[inline(always)]
    pub fn clr_dbg(&self) {
        self.clr_bits(Self::DBG_MASK);
    }

    /// **DMA** `[6]` (RW)
    ///
    /// If 1, PWM can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dma(&self) -> bool {
        (self.value.read_bits() & Self::DMA_MASK) != 0
    }

    /// Set DMA bit atomically (RW)
    #[inline(always)]
    pub fn set_dma(&self) {
        self.set_bits(Self::DMA_MASK);
    }

    /// Clear DMA bit atomically (RW)
    #[inline(always)]
    pub fn clr_dma(&self) {
        self.clr_bits(Self::DMA_MASK);
    }

    /// **CORE1** `[5]` (RW)
    ///
    /// If 1, PWM can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core1(&self) -> bool {
        (self.value.read_bits() & Self::CORE1_MASK) != 0
    }

    /// Set CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn set_core1(&self) {
        self.set_bits(Self::CORE1_MASK);
    }

    /// Clear CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core1(&self) {
        self.clr_bits(Self::CORE1_MASK);
    }

    /// **CORE0** `[4]` (RW)
    ///
    /// If 1, PWM can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core0(&self) -> bool {
        (self.value.read_bits() & Self::CORE0_MASK) != 0
    }

    /// Set CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn set_core0(&self) {
        self.set_bits(Self::CORE0_MASK);
    }

    /// Clear CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core0(&self) {
        self.clr_bits(Self::CORE0_MASK);
    }

    /// **SP** `[3]` (RW)
    ///
    /// If 1, PWM can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub fn sp(&self) -> bool {
        (self.value.read_bits() & Self::SP_MASK) != 0
    }

    /// Set SP bit atomically (RW)
    #[inline(always)]
    pub fn set_sp(&self) {
        self.set_bits(Self::SP_MASK);
    }

    /// Clear SP bit atomically (RW)
    #[inline(always)]
    pub fn clr_sp(&self) {
        self.clr_bits(Self::SP_MASK);
    }

    /// **SU** `[2]` (RW)
    ///
    /// If 1, and SP is also set, PWM can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub fn su(&self) -> bool {
        (self.value.read_bits() & Self::SU_MASK) != 0
    }

    /// Set SU bit atomically (RW)
    #[inline(always)]
    pub fn set_su(&self) {
        self.set_bits(Self::SU_MASK);
    }

    /// Clear SU bit atomically (RW)
    #[inline(always)]
    pub fn clr_su(&self) {
        self.clr_bits(Self::SU_MASK);
    }

    /// **NSP** `[1]` (RW)
    ///
    /// If 1, PWM can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub fn nsp(&self) -> bool {
        (self.value.read_bits() & Self::NSP_MASK) != 0
    }

    /// Set NSP bit atomically (RW)
    #[inline(always)]
    pub fn set_nsp(&self) {
        self.set_bits(Self::NSP_MASK);
    }

    /// Clear NSP bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsp(&self) {
        self.clr_bits(Self::NSP_MASK);
    }

    /// **NSU** `[0]` (RW)
    ///
    /// If 1, and NSP is also set, PWM can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub fn nsu(&self) -> bool {
        (self.value.read_bits() & Self::NSU_MASK) != 0
    }

    /// Set NSU bit atomically (RW)
    #[inline(always)]
    pub fn set_nsu(&self) {
        self.set_bits(Self::NSU_MASK);
    }

    /// Clear NSU bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsu(&self) {
        self.clr_bits(Self::NSU_MASK);
    }
}

/// PWM value type for batch read/modify/write operations
///
/// Use `pwm.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pwm.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PwmVal {
    bits: u32,
}

impl PwmVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x000000fc }
    }

    /// Read **DBG** `[7:7]`
    ///
    /// If 1, PWM can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        (self.bits & Pwm::DBG_MASK) != 0
    }

    /// Read **DMA** `[6:6]`
    ///
    /// If 1, PWM can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        (self.bits & Pwm::DMA_MASK) != 0
    }

    /// Read **CORE1** `[5:5]`
    ///
    /// If 1, PWM can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        (self.bits & Pwm::CORE1_MASK) != 0
    }

    /// Read **CORE0** `[4:4]`
    ///
    /// If 1, PWM can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        (self.bits & Pwm::CORE0_MASK) != 0
    }

    /// Read **SP** `[3:3]`
    ///
    /// If 1, PWM can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        (self.bits & Pwm::SP_MASK) != 0
    }

    /// Read **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, PWM can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn su(&self) -> bool {
        (self.bits & Pwm::SU_MASK) != 0
    }

    /// Read **NSP** `[1:1]`
    ///
    /// If 1, PWM can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        (self.bits & Pwm::NSP_MASK) != 0
    }

    /// Read **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, PWM can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        (self.bits & Pwm::NSU_MASK) != 0
    }

    /// Set **DBG** `[7:7]`
    ///
    /// If 1, PWM can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dbg(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pwm::DBG_MASK
            } else {
                self.bits & !Pwm::DBG_MASK
            }
        }
    }

    /// Set **DMA** `[6:6]`
    ///
    /// If 1, PWM can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dma(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pwm::DMA_MASK
            } else {
                self.bits & !Pwm::DMA_MASK
            }
        }
    }

    /// Set **CORE1** `[5:5]`
    ///
    /// If 1, PWM can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pwm::CORE1_MASK
            } else {
                self.bits & !Pwm::CORE1_MASK
            }
        }
    }

    /// Set **CORE0** `[4:4]`
    ///
    /// If 1, PWM can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pwm::CORE0_MASK
            } else {
                self.bits & !Pwm::CORE0_MASK
            }
        }
    }

    /// Set **SP** `[3:3]`
    ///
    /// If 1, PWM can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn with_sp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pwm::SP_MASK
            } else {
                self.bits & !Pwm::SP_MASK
            }
        }
    }

    /// Set **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, PWM can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn with_su(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pwm::SU_MASK
            } else {
                self.bits & !Pwm::SU_MASK
            }
        }
    }

    /// Set **NSP** `[1:1]`
    ///
    /// If 1, PWM can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn with_nsp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pwm::NSP_MASK
            } else {
                self.bits & !Pwm::NSP_MASK
            }
        }
    }

    /// Set **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, PWM can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn with_nsu(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pwm::NSU_MASK
            } else {
                self.bits & !Pwm::NSU_MASK
            }
        }
    }
}

impl Default for PwmVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Pwm {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PwmVal {
        PwmVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: PwmVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(PwmVal) -> PwmVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI0 Register
///
/// Control whether debugger, DMA, core 0 and core 1 can access SPI0, and at what security/privilege levels they can do so.
/// Defaults to Secure access from any master.
/// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
///
/// Address offset: 0x0090
/// Reset value: 0x000000fc
///
/// ## Fields
///
/// - **DBG** `[7]` (RW)
///
///   If 1, SPI0 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **DMA** `[6]` (RW)
///
///   If 1, SPI0 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE1** `[5]` (RW)
///
///   If 1, SPI0 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE0** `[4]` (RW)
///
///   If 1, SPI0 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **SP** `[3]` (RW)
///
///   If 1, SPI0 can be accessed from a Secure, Privileged context.
///
/// - **SU** `[2]` (RW)
///
///   If 1, and SP is also set, SPI0 can be accessed from a Secure, Unprivileged context.
///
/// - **NSP** `[1]` (RW)
///
///   If 1, SPI0 can be accessed from a Non-secure, Privileged context.
///
/// - **NSU** `[0]` (RW)
///
///   If 1, and NSP is also set, SPI0 can be accessed from a Non-secure, Unprivileged context.
///   This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
#[repr(transparent)]
pub struct Spi0 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Spi0 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Spi0 {
    // ======== Field Constants ========

    /// **DBG**
    ///
    /// If 1, SPI0 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const DBG_OFFSET: u32 = 7;
    pub const DBG_MASK: u32 = 0x00000080;
    pub const DBG: u32 = 1 << 7;

    /// **DMA**
    ///
    /// If 1, SPI0 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const DMA_OFFSET: u32 = 6;
    pub const DMA_MASK: u32 = 0x00000040;
    pub const DMA: u32 = 1 << 6;

    /// **CORE1**
    ///
    /// If 1, SPI0 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CORE1_OFFSET: u32 = 5;
    pub const CORE1_MASK: u32 = 0x00000020;
    pub const CORE1: u32 = 1 << 5;

    /// **CORE0**
    ///
    /// If 1, SPI0 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CORE0_OFFSET: u32 = 4;
    pub const CORE0_MASK: u32 = 0x00000010;
    pub const CORE0: u32 = 1 << 4;

    /// **SP**
    ///
    /// If 1, SPI0 can be accessed from a Secure, Privileged context.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SP_OFFSET: u32 = 3;
    pub const SP_MASK: u32 = 0x00000008;
    pub const SP: u32 = 1 << 3;

    /// **SU**
    ///
    /// If 1, and SP is also set, SPI0 can be accessed from a Secure, Unprivileged context.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SU_OFFSET: u32 = 2;
    pub const SU_MASK: u32 = 0x00000004;
    pub const SU: u32 = 1 << 2;

    /// **NSP**
    ///
    /// If 1, SPI0 can be accessed from a Non-secure, Privileged context.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const NSP_OFFSET: u32 = 1;
    pub const NSP_MASK: u32 = 0x00000002;
    pub const NSP: u32 = 1 << 1;

    /// **NSU**
    ///
    /// If 1, and NSP is also set, SPI0 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const NSU_OFFSET: u32 = 0;
    pub const NSU_MASK: u32 = 0x00000001;
    pub const NSU: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DBG** `[7]` (RW)
    ///
    /// If 1, SPI0 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dbg(&self) -> bool {
        (self.value.read_bits() & Self::DBG_MASK) != 0
    }

    /// Set DBG bit atomically (RW)
    #[inline(always)]
    pub fn set_dbg(&self) {
        self.set_bits(Self::DBG_MASK);
    }

    /// Clear DBG bit atomically (RW)
    #[inline(always)]
    pub fn clr_dbg(&self) {
        self.clr_bits(Self::DBG_MASK);
    }

    /// **DMA** `[6]` (RW)
    ///
    /// If 1, SPI0 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dma(&self) -> bool {
        (self.value.read_bits() & Self::DMA_MASK) != 0
    }

    /// Set DMA bit atomically (RW)
    #[inline(always)]
    pub fn set_dma(&self) {
        self.set_bits(Self::DMA_MASK);
    }

    /// Clear DMA bit atomically (RW)
    #[inline(always)]
    pub fn clr_dma(&self) {
        self.clr_bits(Self::DMA_MASK);
    }

    /// **CORE1** `[5]` (RW)
    ///
    /// If 1, SPI0 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core1(&self) -> bool {
        (self.value.read_bits() & Self::CORE1_MASK) != 0
    }

    /// Set CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn set_core1(&self) {
        self.set_bits(Self::CORE1_MASK);
    }

    /// Clear CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core1(&self) {
        self.clr_bits(Self::CORE1_MASK);
    }

    /// **CORE0** `[4]` (RW)
    ///
    /// If 1, SPI0 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core0(&self) -> bool {
        (self.value.read_bits() & Self::CORE0_MASK) != 0
    }

    /// Set CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn set_core0(&self) {
        self.set_bits(Self::CORE0_MASK);
    }

    /// Clear CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core0(&self) {
        self.clr_bits(Self::CORE0_MASK);
    }

    /// **SP** `[3]` (RW)
    ///
    /// If 1, SPI0 can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub fn sp(&self) -> bool {
        (self.value.read_bits() & Self::SP_MASK) != 0
    }

    /// Set SP bit atomically (RW)
    #[inline(always)]
    pub fn set_sp(&self) {
        self.set_bits(Self::SP_MASK);
    }

    /// Clear SP bit atomically (RW)
    #[inline(always)]
    pub fn clr_sp(&self) {
        self.clr_bits(Self::SP_MASK);
    }

    /// **SU** `[2]` (RW)
    ///
    /// If 1, and SP is also set, SPI0 can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub fn su(&self) -> bool {
        (self.value.read_bits() & Self::SU_MASK) != 0
    }

    /// Set SU bit atomically (RW)
    #[inline(always)]
    pub fn set_su(&self) {
        self.set_bits(Self::SU_MASK);
    }

    /// Clear SU bit atomically (RW)
    #[inline(always)]
    pub fn clr_su(&self) {
        self.clr_bits(Self::SU_MASK);
    }

    /// **NSP** `[1]` (RW)
    ///
    /// If 1, SPI0 can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub fn nsp(&self) -> bool {
        (self.value.read_bits() & Self::NSP_MASK) != 0
    }

    /// Set NSP bit atomically (RW)
    #[inline(always)]
    pub fn set_nsp(&self) {
        self.set_bits(Self::NSP_MASK);
    }

    /// Clear NSP bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsp(&self) {
        self.clr_bits(Self::NSP_MASK);
    }

    /// **NSU** `[0]` (RW)
    ///
    /// If 1, and NSP is also set, SPI0 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub fn nsu(&self) -> bool {
        (self.value.read_bits() & Self::NSU_MASK) != 0
    }

    /// Set NSU bit atomically (RW)
    #[inline(always)]
    pub fn set_nsu(&self) {
        self.set_bits(Self::NSU_MASK);
    }

    /// Clear NSU bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsu(&self) {
        self.clr_bits(Self::NSU_MASK);
    }
}

/// SPI0 value type for batch read/modify/write operations
///
/// Use `spi0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spi0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Spi0Val {
    bits: u32,
}

impl Spi0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x000000fc }
    }

    /// Read **DBG** `[7:7]`
    ///
    /// If 1, SPI0 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        (self.bits & Spi0::DBG_MASK) != 0
    }

    /// Read **DMA** `[6:6]`
    ///
    /// If 1, SPI0 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        (self.bits & Spi0::DMA_MASK) != 0
    }

    /// Read **CORE1** `[5:5]`
    ///
    /// If 1, SPI0 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        (self.bits & Spi0::CORE1_MASK) != 0
    }

    /// Read **CORE0** `[4:4]`
    ///
    /// If 1, SPI0 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        (self.bits & Spi0::CORE0_MASK) != 0
    }

    /// Read **SP** `[3:3]`
    ///
    /// If 1, SPI0 can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        (self.bits & Spi0::SP_MASK) != 0
    }

    /// Read **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, SPI0 can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn su(&self) -> bool {
        (self.bits & Spi0::SU_MASK) != 0
    }

    /// Read **NSP** `[1:1]`
    ///
    /// If 1, SPI0 can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        (self.bits & Spi0::NSP_MASK) != 0
    }

    /// Read **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, SPI0 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        (self.bits & Spi0::NSU_MASK) != 0
    }

    /// Set **DBG** `[7:7]`
    ///
    /// If 1, SPI0 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dbg(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Spi0::DBG_MASK
            } else {
                self.bits & !Spi0::DBG_MASK
            }
        }
    }

    /// Set **DMA** `[6:6]`
    ///
    /// If 1, SPI0 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dma(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Spi0::DMA_MASK
            } else {
                self.bits & !Spi0::DMA_MASK
            }
        }
    }

    /// Set **CORE1** `[5:5]`
    ///
    /// If 1, SPI0 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Spi0::CORE1_MASK
            } else {
                self.bits & !Spi0::CORE1_MASK
            }
        }
    }

    /// Set **CORE0** `[4:4]`
    ///
    /// If 1, SPI0 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Spi0::CORE0_MASK
            } else {
                self.bits & !Spi0::CORE0_MASK
            }
        }
    }

    /// Set **SP** `[3:3]`
    ///
    /// If 1, SPI0 can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn with_sp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Spi0::SP_MASK
            } else {
                self.bits & !Spi0::SP_MASK
            }
        }
    }

    /// Set **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, SPI0 can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn with_su(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Spi0::SU_MASK
            } else {
                self.bits & !Spi0::SU_MASK
            }
        }
    }

    /// Set **NSP** `[1:1]`
    ///
    /// If 1, SPI0 can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn with_nsp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Spi0::NSP_MASK
            } else {
                self.bits & !Spi0::NSP_MASK
            }
        }
    }

    /// Set **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, SPI0 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn with_nsu(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Spi0::NSU_MASK
            } else {
                self.bits & !Spi0::NSU_MASK
            }
        }
    }
}

impl Default for Spi0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Spi0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Spi0Val {
        Spi0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Spi0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Spi0Val) -> Spi0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI1 Register
///
/// Control whether debugger, DMA, core 0 and core 1 can access SPI1, and at what security/privilege levels they can do so.
/// Defaults to Secure access from any master.
/// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
///
/// Address offset: 0x0094
/// Reset value: 0x000000fc
///
/// ## Fields
///
/// - **DBG** `[7]` (RW)
///
///   If 1, SPI1 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **DMA** `[6]` (RW)
///
///   If 1, SPI1 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE1** `[5]` (RW)
///
///   If 1, SPI1 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE0** `[4]` (RW)
///
///   If 1, SPI1 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **SP** `[3]` (RW)
///
///   If 1, SPI1 can be accessed from a Secure, Privileged context.
///
/// - **SU** `[2]` (RW)
///
///   If 1, and SP is also set, SPI1 can be accessed from a Secure, Unprivileged context.
///
/// - **NSP** `[1]` (RW)
///
///   If 1, SPI1 can be accessed from a Non-secure, Privileged context.
///
/// - **NSU** `[0]` (RW)
///
///   If 1, and NSP is also set, SPI1 can be accessed from a Non-secure, Unprivileged context.
///   This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
#[repr(transparent)]
pub struct Spi1 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Spi1 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Spi1 {
    // ======== Field Constants ========

    /// **DBG**
    ///
    /// If 1, SPI1 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const DBG_OFFSET: u32 = 7;
    pub const DBG_MASK: u32 = 0x00000080;
    pub const DBG: u32 = 1 << 7;

    /// **DMA**
    ///
    /// If 1, SPI1 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const DMA_OFFSET: u32 = 6;
    pub const DMA_MASK: u32 = 0x00000040;
    pub const DMA: u32 = 1 << 6;

    /// **CORE1**
    ///
    /// If 1, SPI1 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CORE1_OFFSET: u32 = 5;
    pub const CORE1_MASK: u32 = 0x00000020;
    pub const CORE1: u32 = 1 << 5;

    /// **CORE0**
    ///
    /// If 1, SPI1 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CORE0_OFFSET: u32 = 4;
    pub const CORE0_MASK: u32 = 0x00000010;
    pub const CORE0: u32 = 1 << 4;

    /// **SP**
    ///
    /// If 1, SPI1 can be accessed from a Secure, Privileged context.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SP_OFFSET: u32 = 3;
    pub const SP_MASK: u32 = 0x00000008;
    pub const SP: u32 = 1 << 3;

    /// **SU**
    ///
    /// If 1, and SP is also set, SPI1 can be accessed from a Secure, Unprivileged context.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SU_OFFSET: u32 = 2;
    pub const SU_MASK: u32 = 0x00000004;
    pub const SU: u32 = 1 << 2;

    /// **NSP**
    ///
    /// If 1, SPI1 can be accessed from a Non-secure, Privileged context.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const NSP_OFFSET: u32 = 1;
    pub const NSP_MASK: u32 = 0x00000002;
    pub const NSP: u32 = 1 << 1;

    /// **NSU**
    ///
    /// If 1, and NSP is also set, SPI1 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const NSU_OFFSET: u32 = 0;
    pub const NSU_MASK: u32 = 0x00000001;
    pub const NSU: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DBG** `[7]` (RW)
    ///
    /// If 1, SPI1 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dbg(&self) -> bool {
        (self.value.read_bits() & Self::DBG_MASK) != 0
    }

    /// Set DBG bit atomically (RW)
    #[inline(always)]
    pub fn set_dbg(&self) {
        self.set_bits(Self::DBG_MASK);
    }

    /// Clear DBG bit atomically (RW)
    #[inline(always)]
    pub fn clr_dbg(&self) {
        self.clr_bits(Self::DBG_MASK);
    }

    /// **DMA** `[6]` (RW)
    ///
    /// If 1, SPI1 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dma(&self) -> bool {
        (self.value.read_bits() & Self::DMA_MASK) != 0
    }

    /// Set DMA bit atomically (RW)
    #[inline(always)]
    pub fn set_dma(&self) {
        self.set_bits(Self::DMA_MASK);
    }

    /// Clear DMA bit atomically (RW)
    #[inline(always)]
    pub fn clr_dma(&self) {
        self.clr_bits(Self::DMA_MASK);
    }

    /// **CORE1** `[5]` (RW)
    ///
    /// If 1, SPI1 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core1(&self) -> bool {
        (self.value.read_bits() & Self::CORE1_MASK) != 0
    }

    /// Set CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn set_core1(&self) {
        self.set_bits(Self::CORE1_MASK);
    }

    /// Clear CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core1(&self) {
        self.clr_bits(Self::CORE1_MASK);
    }

    /// **CORE0** `[4]` (RW)
    ///
    /// If 1, SPI1 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core0(&self) -> bool {
        (self.value.read_bits() & Self::CORE0_MASK) != 0
    }

    /// Set CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn set_core0(&self) {
        self.set_bits(Self::CORE0_MASK);
    }

    /// Clear CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core0(&self) {
        self.clr_bits(Self::CORE0_MASK);
    }

    /// **SP** `[3]` (RW)
    ///
    /// If 1, SPI1 can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub fn sp(&self) -> bool {
        (self.value.read_bits() & Self::SP_MASK) != 0
    }

    /// Set SP bit atomically (RW)
    #[inline(always)]
    pub fn set_sp(&self) {
        self.set_bits(Self::SP_MASK);
    }

    /// Clear SP bit atomically (RW)
    #[inline(always)]
    pub fn clr_sp(&self) {
        self.clr_bits(Self::SP_MASK);
    }

    /// **SU** `[2]` (RW)
    ///
    /// If 1, and SP is also set, SPI1 can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub fn su(&self) -> bool {
        (self.value.read_bits() & Self::SU_MASK) != 0
    }

    /// Set SU bit atomically (RW)
    #[inline(always)]
    pub fn set_su(&self) {
        self.set_bits(Self::SU_MASK);
    }

    /// Clear SU bit atomically (RW)
    #[inline(always)]
    pub fn clr_su(&self) {
        self.clr_bits(Self::SU_MASK);
    }

    /// **NSP** `[1]` (RW)
    ///
    /// If 1, SPI1 can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub fn nsp(&self) -> bool {
        (self.value.read_bits() & Self::NSP_MASK) != 0
    }

    /// Set NSP bit atomically (RW)
    #[inline(always)]
    pub fn set_nsp(&self) {
        self.set_bits(Self::NSP_MASK);
    }

    /// Clear NSP bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsp(&self) {
        self.clr_bits(Self::NSP_MASK);
    }

    /// **NSU** `[0]` (RW)
    ///
    /// If 1, and NSP is also set, SPI1 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub fn nsu(&self) -> bool {
        (self.value.read_bits() & Self::NSU_MASK) != 0
    }

    /// Set NSU bit atomically (RW)
    #[inline(always)]
    pub fn set_nsu(&self) {
        self.set_bits(Self::NSU_MASK);
    }

    /// Clear NSU bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsu(&self) {
        self.clr_bits(Self::NSU_MASK);
    }
}

/// SPI1 value type for batch read/modify/write operations
///
/// Use `spi1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spi1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Spi1Val {
    bits: u32,
}

impl Spi1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x000000fc }
    }

    /// Read **DBG** `[7:7]`
    ///
    /// If 1, SPI1 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        (self.bits & Spi1::DBG_MASK) != 0
    }

    /// Read **DMA** `[6:6]`
    ///
    /// If 1, SPI1 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        (self.bits & Spi1::DMA_MASK) != 0
    }

    /// Read **CORE1** `[5:5]`
    ///
    /// If 1, SPI1 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        (self.bits & Spi1::CORE1_MASK) != 0
    }

    /// Read **CORE0** `[4:4]`
    ///
    /// If 1, SPI1 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        (self.bits & Spi1::CORE0_MASK) != 0
    }

    /// Read **SP** `[3:3]`
    ///
    /// If 1, SPI1 can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        (self.bits & Spi1::SP_MASK) != 0
    }

    /// Read **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, SPI1 can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn su(&self) -> bool {
        (self.bits & Spi1::SU_MASK) != 0
    }

    /// Read **NSP** `[1:1]`
    ///
    /// If 1, SPI1 can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        (self.bits & Spi1::NSP_MASK) != 0
    }

    /// Read **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, SPI1 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        (self.bits & Spi1::NSU_MASK) != 0
    }

    /// Set **DBG** `[7:7]`
    ///
    /// If 1, SPI1 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dbg(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Spi1::DBG_MASK
            } else {
                self.bits & !Spi1::DBG_MASK
            }
        }
    }

    /// Set **DMA** `[6:6]`
    ///
    /// If 1, SPI1 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dma(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Spi1::DMA_MASK
            } else {
                self.bits & !Spi1::DMA_MASK
            }
        }
    }

    /// Set **CORE1** `[5:5]`
    ///
    /// If 1, SPI1 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Spi1::CORE1_MASK
            } else {
                self.bits & !Spi1::CORE1_MASK
            }
        }
    }

    /// Set **CORE0** `[4:4]`
    ///
    /// If 1, SPI1 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Spi1::CORE0_MASK
            } else {
                self.bits & !Spi1::CORE0_MASK
            }
        }
    }

    /// Set **SP** `[3:3]`
    ///
    /// If 1, SPI1 can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn with_sp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Spi1::SP_MASK
            } else {
                self.bits & !Spi1::SP_MASK
            }
        }
    }

    /// Set **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, SPI1 can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn with_su(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Spi1::SU_MASK
            } else {
                self.bits & !Spi1::SU_MASK
            }
        }
    }

    /// Set **NSP** `[1:1]`
    ///
    /// If 1, SPI1 can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn with_nsp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Spi1::NSP_MASK
            } else {
                self.bits & !Spi1::NSP_MASK
            }
        }
    }

    /// Set **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, SPI1 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn with_nsu(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Spi1::NSU_MASK
            } else {
                self.bits & !Spi1::NSU_MASK
            }
        }
    }
}

impl Default for Spi1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Spi1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Spi1Val {
        Spi1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Spi1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Spi1Val) -> Spi1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TIMER0 Register
///
/// Control whether debugger, DMA, core 0 and core 1 can access TIMER0, and at what security/privilege levels they can do so.
/// Defaults to Secure access from any master.
/// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
///
/// Address offset: 0x0098
/// Reset value: 0x000000fc
///
/// ## Fields
///
/// - **DBG** `[7]` (RW)
///
///   If 1, TIMER0 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **DMA** `[6]` (RW)
///
///   If 1, TIMER0 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE1** `[5]` (RW)
///
///   If 1, TIMER0 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE0** `[4]` (RW)
///
///   If 1, TIMER0 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **SP** `[3]` (RW)
///
///   If 1, TIMER0 can be accessed from a Secure, Privileged context.
///
/// - **SU** `[2]` (RW)
///
///   If 1, and SP is also set, TIMER0 can be accessed from a Secure, Unprivileged context.
///
/// - **NSP** `[1]` (RW)
///
///   If 1, TIMER0 can be accessed from a Non-secure, Privileged context.
///
/// - **NSU** `[0]` (RW)
///
///   If 1, and NSP is also set, TIMER0 can be accessed from a Non-secure, Unprivileged context.
///   This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
#[repr(transparent)]
pub struct Timer0 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Timer0 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Timer0 {
    // ======== Field Constants ========

    /// **DBG**
    ///
    /// If 1, TIMER0 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const DBG_OFFSET: u32 = 7;
    pub const DBG_MASK: u32 = 0x00000080;
    pub const DBG: u32 = 1 << 7;

    /// **DMA**
    ///
    /// If 1, TIMER0 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const DMA_OFFSET: u32 = 6;
    pub const DMA_MASK: u32 = 0x00000040;
    pub const DMA: u32 = 1 << 6;

    /// **CORE1**
    ///
    /// If 1, TIMER0 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CORE1_OFFSET: u32 = 5;
    pub const CORE1_MASK: u32 = 0x00000020;
    pub const CORE1: u32 = 1 << 5;

    /// **CORE0**
    ///
    /// If 1, TIMER0 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CORE0_OFFSET: u32 = 4;
    pub const CORE0_MASK: u32 = 0x00000010;
    pub const CORE0: u32 = 1 << 4;

    /// **SP**
    ///
    /// If 1, TIMER0 can be accessed from a Secure, Privileged context.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SP_OFFSET: u32 = 3;
    pub const SP_MASK: u32 = 0x00000008;
    pub const SP: u32 = 1 << 3;

    /// **SU**
    ///
    /// If 1, and SP is also set, TIMER0 can be accessed from a Secure, Unprivileged context.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SU_OFFSET: u32 = 2;
    pub const SU_MASK: u32 = 0x00000004;
    pub const SU: u32 = 1 << 2;

    /// **NSP**
    ///
    /// If 1, TIMER0 can be accessed from a Non-secure, Privileged context.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const NSP_OFFSET: u32 = 1;
    pub const NSP_MASK: u32 = 0x00000002;
    pub const NSP: u32 = 1 << 1;

    /// **NSU**
    ///
    /// If 1, and NSP is also set, TIMER0 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const NSU_OFFSET: u32 = 0;
    pub const NSU_MASK: u32 = 0x00000001;
    pub const NSU: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DBG** `[7]` (RW)
    ///
    /// If 1, TIMER0 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dbg(&self) -> bool {
        (self.value.read_bits() & Self::DBG_MASK) != 0
    }

    /// Set DBG bit atomically (RW)
    #[inline(always)]
    pub fn set_dbg(&self) {
        self.set_bits(Self::DBG_MASK);
    }

    /// Clear DBG bit atomically (RW)
    #[inline(always)]
    pub fn clr_dbg(&self) {
        self.clr_bits(Self::DBG_MASK);
    }

    /// **DMA** `[6]` (RW)
    ///
    /// If 1, TIMER0 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dma(&self) -> bool {
        (self.value.read_bits() & Self::DMA_MASK) != 0
    }

    /// Set DMA bit atomically (RW)
    #[inline(always)]
    pub fn set_dma(&self) {
        self.set_bits(Self::DMA_MASK);
    }

    /// Clear DMA bit atomically (RW)
    #[inline(always)]
    pub fn clr_dma(&self) {
        self.clr_bits(Self::DMA_MASK);
    }

    /// **CORE1** `[5]` (RW)
    ///
    /// If 1, TIMER0 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core1(&self) -> bool {
        (self.value.read_bits() & Self::CORE1_MASK) != 0
    }

    /// Set CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn set_core1(&self) {
        self.set_bits(Self::CORE1_MASK);
    }

    /// Clear CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core1(&self) {
        self.clr_bits(Self::CORE1_MASK);
    }

    /// **CORE0** `[4]` (RW)
    ///
    /// If 1, TIMER0 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core0(&self) -> bool {
        (self.value.read_bits() & Self::CORE0_MASK) != 0
    }

    /// Set CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn set_core0(&self) {
        self.set_bits(Self::CORE0_MASK);
    }

    /// Clear CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core0(&self) {
        self.clr_bits(Self::CORE0_MASK);
    }

    /// **SP** `[3]` (RW)
    ///
    /// If 1, TIMER0 can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub fn sp(&self) -> bool {
        (self.value.read_bits() & Self::SP_MASK) != 0
    }

    /// Set SP bit atomically (RW)
    #[inline(always)]
    pub fn set_sp(&self) {
        self.set_bits(Self::SP_MASK);
    }

    /// Clear SP bit atomically (RW)
    #[inline(always)]
    pub fn clr_sp(&self) {
        self.clr_bits(Self::SP_MASK);
    }

    /// **SU** `[2]` (RW)
    ///
    /// If 1, and SP is also set, TIMER0 can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub fn su(&self) -> bool {
        (self.value.read_bits() & Self::SU_MASK) != 0
    }

    /// Set SU bit atomically (RW)
    #[inline(always)]
    pub fn set_su(&self) {
        self.set_bits(Self::SU_MASK);
    }

    /// Clear SU bit atomically (RW)
    #[inline(always)]
    pub fn clr_su(&self) {
        self.clr_bits(Self::SU_MASK);
    }

    /// **NSP** `[1]` (RW)
    ///
    /// If 1, TIMER0 can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub fn nsp(&self) -> bool {
        (self.value.read_bits() & Self::NSP_MASK) != 0
    }

    /// Set NSP bit atomically (RW)
    #[inline(always)]
    pub fn set_nsp(&self) {
        self.set_bits(Self::NSP_MASK);
    }

    /// Clear NSP bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsp(&self) {
        self.clr_bits(Self::NSP_MASK);
    }

    /// **NSU** `[0]` (RW)
    ///
    /// If 1, and NSP is also set, TIMER0 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub fn nsu(&self) -> bool {
        (self.value.read_bits() & Self::NSU_MASK) != 0
    }

    /// Set NSU bit atomically (RW)
    #[inline(always)]
    pub fn set_nsu(&self) {
        self.set_bits(Self::NSU_MASK);
    }

    /// Clear NSU bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsu(&self) {
        self.clr_bits(Self::NSU_MASK);
    }
}

/// TIMER0 value type for batch read/modify/write operations
///
/// Use `timer0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `timer0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Timer0Val {
    bits: u32,
}

impl Timer0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x000000fc }
    }

    /// Read **DBG** `[7:7]`
    ///
    /// If 1, TIMER0 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        (self.bits & Timer0::DBG_MASK) != 0
    }

    /// Read **DMA** `[6:6]`
    ///
    /// If 1, TIMER0 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        (self.bits & Timer0::DMA_MASK) != 0
    }

    /// Read **CORE1** `[5:5]`
    ///
    /// If 1, TIMER0 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        (self.bits & Timer0::CORE1_MASK) != 0
    }

    /// Read **CORE0** `[4:4]`
    ///
    /// If 1, TIMER0 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        (self.bits & Timer0::CORE0_MASK) != 0
    }

    /// Read **SP** `[3:3]`
    ///
    /// If 1, TIMER0 can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        (self.bits & Timer0::SP_MASK) != 0
    }

    /// Read **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, TIMER0 can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn su(&self) -> bool {
        (self.bits & Timer0::SU_MASK) != 0
    }

    /// Read **NSP** `[1:1]`
    ///
    /// If 1, TIMER0 can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        (self.bits & Timer0::NSP_MASK) != 0
    }

    /// Read **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, TIMER0 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        (self.bits & Timer0::NSU_MASK) != 0
    }

    /// Set **DBG** `[7:7]`
    ///
    /// If 1, TIMER0 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dbg(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Timer0::DBG_MASK
            } else {
                self.bits & !Timer0::DBG_MASK
            }
        }
    }

    /// Set **DMA** `[6:6]`
    ///
    /// If 1, TIMER0 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dma(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Timer0::DMA_MASK
            } else {
                self.bits & !Timer0::DMA_MASK
            }
        }
    }

    /// Set **CORE1** `[5:5]`
    ///
    /// If 1, TIMER0 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Timer0::CORE1_MASK
            } else {
                self.bits & !Timer0::CORE1_MASK
            }
        }
    }

    /// Set **CORE0** `[4:4]`
    ///
    /// If 1, TIMER0 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Timer0::CORE0_MASK
            } else {
                self.bits & !Timer0::CORE0_MASK
            }
        }
    }

    /// Set **SP** `[3:3]`
    ///
    /// If 1, TIMER0 can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn with_sp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Timer0::SP_MASK
            } else {
                self.bits & !Timer0::SP_MASK
            }
        }
    }

    /// Set **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, TIMER0 can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn with_su(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Timer0::SU_MASK
            } else {
                self.bits & !Timer0::SU_MASK
            }
        }
    }

    /// Set **NSP** `[1:1]`
    ///
    /// If 1, TIMER0 can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn with_nsp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Timer0::NSP_MASK
            } else {
                self.bits & !Timer0::NSP_MASK
            }
        }
    }

    /// Set **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, TIMER0 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn with_nsu(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Timer0::NSU_MASK
            } else {
                self.bits & !Timer0::NSU_MASK
            }
        }
    }
}

impl Default for Timer0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Timer0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Timer0Val {
        Timer0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Timer0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Timer0Val) -> Timer0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TIMER1 Register
///
/// Control whether debugger, DMA, core 0 and core 1 can access TIMER1, and at what security/privilege levels they can do so.
/// Defaults to Secure access from any master.
/// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
///
/// Address offset: 0x009c
/// Reset value: 0x000000fc
///
/// ## Fields
///
/// - **DBG** `[7]` (RW)
///
///   If 1, TIMER1 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **DMA** `[6]` (RW)
///
///   If 1, TIMER1 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE1** `[5]` (RW)
///
///   If 1, TIMER1 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE0** `[4]` (RW)
///
///   If 1, TIMER1 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **SP** `[3]` (RW)
///
///   If 1, TIMER1 can be accessed from a Secure, Privileged context.
///
/// - **SU** `[2]` (RW)
///
///   If 1, and SP is also set, TIMER1 can be accessed from a Secure, Unprivileged context.
///
/// - **NSP** `[1]` (RW)
///
///   If 1, TIMER1 can be accessed from a Non-secure, Privileged context.
///
/// - **NSU** `[0]` (RW)
///
///   If 1, and NSP is also set, TIMER1 can be accessed from a Non-secure, Unprivileged context.
///   This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
#[repr(transparent)]
pub struct Timer1 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Timer1 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Timer1 {
    // ======== Field Constants ========

    /// **DBG**
    ///
    /// If 1, TIMER1 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const DBG_OFFSET: u32 = 7;
    pub const DBG_MASK: u32 = 0x00000080;
    pub const DBG: u32 = 1 << 7;

    /// **DMA**
    ///
    /// If 1, TIMER1 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const DMA_OFFSET: u32 = 6;
    pub const DMA_MASK: u32 = 0x00000040;
    pub const DMA: u32 = 1 << 6;

    /// **CORE1**
    ///
    /// If 1, TIMER1 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CORE1_OFFSET: u32 = 5;
    pub const CORE1_MASK: u32 = 0x00000020;
    pub const CORE1: u32 = 1 << 5;

    /// **CORE0**
    ///
    /// If 1, TIMER1 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CORE0_OFFSET: u32 = 4;
    pub const CORE0_MASK: u32 = 0x00000010;
    pub const CORE0: u32 = 1 << 4;

    /// **SP**
    ///
    /// If 1, TIMER1 can be accessed from a Secure, Privileged context.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SP_OFFSET: u32 = 3;
    pub const SP_MASK: u32 = 0x00000008;
    pub const SP: u32 = 1 << 3;

    /// **SU**
    ///
    /// If 1, and SP is also set, TIMER1 can be accessed from a Secure, Unprivileged context.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SU_OFFSET: u32 = 2;
    pub const SU_MASK: u32 = 0x00000004;
    pub const SU: u32 = 1 << 2;

    /// **NSP**
    ///
    /// If 1, TIMER1 can be accessed from a Non-secure, Privileged context.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const NSP_OFFSET: u32 = 1;
    pub const NSP_MASK: u32 = 0x00000002;
    pub const NSP: u32 = 1 << 1;

    /// **NSU**
    ///
    /// If 1, and NSP is also set, TIMER1 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const NSU_OFFSET: u32 = 0;
    pub const NSU_MASK: u32 = 0x00000001;
    pub const NSU: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DBG** `[7]` (RW)
    ///
    /// If 1, TIMER1 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dbg(&self) -> bool {
        (self.value.read_bits() & Self::DBG_MASK) != 0
    }

    /// Set DBG bit atomically (RW)
    #[inline(always)]
    pub fn set_dbg(&self) {
        self.set_bits(Self::DBG_MASK);
    }

    /// Clear DBG bit atomically (RW)
    #[inline(always)]
    pub fn clr_dbg(&self) {
        self.clr_bits(Self::DBG_MASK);
    }

    /// **DMA** `[6]` (RW)
    ///
    /// If 1, TIMER1 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dma(&self) -> bool {
        (self.value.read_bits() & Self::DMA_MASK) != 0
    }

    /// Set DMA bit atomically (RW)
    #[inline(always)]
    pub fn set_dma(&self) {
        self.set_bits(Self::DMA_MASK);
    }

    /// Clear DMA bit atomically (RW)
    #[inline(always)]
    pub fn clr_dma(&self) {
        self.clr_bits(Self::DMA_MASK);
    }

    /// **CORE1** `[5]` (RW)
    ///
    /// If 1, TIMER1 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core1(&self) -> bool {
        (self.value.read_bits() & Self::CORE1_MASK) != 0
    }

    /// Set CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn set_core1(&self) {
        self.set_bits(Self::CORE1_MASK);
    }

    /// Clear CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core1(&self) {
        self.clr_bits(Self::CORE1_MASK);
    }

    /// **CORE0** `[4]` (RW)
    ///
    /// If 1, TIMER1 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core0(&self) -> bool {
        (self.value.read_bits() & Self::CORE0_MASK) != 0
    }

    /// Set CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn set_core0(&self) {
        self.set_bits(Self::CORE0_MASK);
    }

    /// Clear CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core0(&self) {
        self.clr_bits(Self::CORE0_MASK);
    }

    /// **SP** `[3]` (RW)
    ///
    /// If 1, TIMER1 can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub fn sp(&self) -> bool {
        (self.value.read_bits() & Self::SP_MASK) != 0
    }

    /// Set SP bit atomically (RW)
    #[inline(always)]
    pub fn set_sp(&self) {
        self.set_bits(Self::SP_MASK);
    }

    /// Clear SP bit atomically (RW)
    #[inline(always)]
    pub fn clr_sp(&self) {
        self.clr_bits(Self::SP_MASK);
    }

    /// **SU** `[2]` (RW)
    ///
    /// If 1, and SP is also set, TIMER1 can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub fn su(&self) -> bool {
        (self.value.read_bits() & Self::SU_MASK) != 0
    }

    /// Set SU bit atomically (RW)
    #[inline(always)]
    pub fn set_su(&self) {
        self.set_bits(Self::SU_MASK);
    }

    /// Clear SU bit atomically (RW)
    #[inline(always)]
    pub fn clr_su(&self) {
        self.clr_bits(Self::SU_MASK);
    }

    /// **NSP** `[1]` (RW)
    ///
    /// If 1, TIMER1 can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub fn nsp(&self) -> bool {
        (self.value.read_bits() & Self::NSP_MASK) != 0
    }

    /// Set NSP bit atomically (RW)
    #[inline(always)]
    pub fn set_nsp(&self) {
        self.set_bits(Self::NSP_MASK);
    }

    /// Clear NSP bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsp(&self) {
        self.clr_bits(Self::NSP_MASK);
    }

    /// **NSU** `[0]` (RW)
    ///
    /// If 1, and NSP is also set, TIMER1 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub fn nsu(&self) -> bool {
        (self.value.read_bits() & Self::NSU_MASK) != 0
    }

    /// Set NSU bit atomically (RW)
    #[inline(always)]
    pub fn set_nsu(&self) {
        self.set_bits(Self::NSU_MASK);
    }

    /// Clear NSU bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsu(&self) {
        self.clr_bits(Self::NSU_MASK);
    }
}

/// TIMER1 value type for batch read/modify/write operations
///
/// Use `timer1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `timer1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Timer1Val {
    bits: u32,
}

impl Timer1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x000000fc }
    }

    /// Read **DBG** `[7:7]`
    ///
    /// If 1, TIMER1 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        (self.bits & Timer1::DBG_MASK) != 0
    }

    /// Read **DMA** `[6:6]`
    ///
    /// If 1, TIMER1 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        (self.bits & Timer1::DMA_MASK) != 0
    }

    /// Read **CORE1** `[5:5]`
    ///
    /// If 1, TIMER1 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        (self.bits & Timer1::CORE1_MASK) != 0
    }

    /// Read **CORE0** `[4:4]`
    ///
    /// If 1, TIMER1 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        (self.bits & Timer1::CORE0_MASK) != 0
    }

    /// Read **SP** `[3:3]`
    ///
    /// If 1, TIMER1 can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        (self.bits & Timer1::SP_MASK) != 0
    }

    /// Read **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, TIMER1 can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn su(&self) -> bool {
        (self.bits & Timer1::SU_MASK) != 0
    }

    /// Read **NSP** `[1:1]`
    ///
    /// If 1, TIMER1 can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        (self.bits & Timer1::NSP_MASK) != 0
    }

    /// Read **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, TIMER1 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        (self.bits & Timer1::NSU_MASK) != 0
    }

    /// Set **DBG** `[7:7]`
    ///
    /// If 1, TIMER1 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dbg(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Timer1::DBG_MASK
            } else {
                self.bits & !Timer1::DBG_MASK
            }
        }
    }

    /// Set **DMA** `[6:6]`
    ///
    /// If 1, TIMER1 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dma(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Timer1::DMA_MASK
            } else {
                self.bits & !Timer1::DMA_MASK
            }
        }
    }

    /// Set **CORE1** `[5:5]`
    ///
    /// If 1, TIMER1 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Timer1::CORE1_MASK
            } else {
                self.bits & !Timer1::CORE1_MASK
            }
        }
    }

    /// Set **CORE0** `[4:4]`
    ///
    /// If 1, TIMER1 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Timer1::CORE0_MASK
            } else {
                self.bits & !Timer1::CORE0_MASK
            }
        }
    }

    /// Set **SP** `[3:3]`
    ///
    /// If 1, TIMER1 can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn with_sp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Timer1::SP_MASK
            } else {
                self.bits & !Timer1::SP_MASK
            }
        }
    }

    /// Set **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, TIMER1 can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn with_su(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Timer1::SU_MASK
            } else {
                self.bits & !Timer1::SU_MASK
            }
        }
    }

    /// Set **NSP** `[1:1]`
    ///
    /// If 1, TIMER1 can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn with_nsp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Timer1::NSP_MASK
            } else {
                self.bits & !Timer1::NSP_MASK
            }
        }
    }

    /// Set **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, TIMER1 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn with_nsu(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Timer1::NSU_MASK
            } else {
                self.bits & !Timer1::NSU_MASK
            }
        }
    }
}

impl Default for Timer1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Timer1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Timer1Val {
        Timer1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Timer1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Timer1Val) -> Timer1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// UART0 Register
///
/// Control whether debugger, DMA, core 0 and core 1 can access UART0, and at what security/privilege levels they can do so.
/// Defaults to Secure access from any master.
/// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
///
/// Address offset: 0x00a0
/// Reset value: 0x000000fc
///
/// ## Fields
///
/// - **DBG** `[7]` (RW)
///
///   If 1, UART0 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **DMA** `[6]` (RW)
///
///   If 1, UART0 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE1** `[5]` (RW)
///
///   If 1, UART0 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE0** `[4]` (RW)
///
///   If 1, UART0 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **SP** `[3]` (RW)
///
///   If 1, UART0 can be accessed from a Secure, Privileged context.
///
/// - **SU** `[2]` (RW)
///
///   If 1, and SP is also set, UART0 can be accessed from a Secure, Unprivileged context.
///
/// - **NSP** `[1]` (RW)
///
///   If 1, UART0 can be accessed from a Non-secure, Privileged context.
///
/// - **NSU** `[0]` (RW)
///
///   If 1, and NSP is also set, UART0 can be accessed from a Non-secure, Unprivileged context.
///   This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
#[repr(transparent)]
pub struct Uart0 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Uart0 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Uart0 {
    // ======== Field Constants ========

    /// **DBG**
    ///
    /// If 1, UART0 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const DBG_OFFSET: u32 = 7;
    pub const DBG_MASK: u32 = 0x00000080;
    pub const DBG: u32 = 1 << 7;

    /// **DMA**
    ///
    /// If 1, UART0 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const DMA_OFFSET: u32 = 6;
    pub const DMA_MASK: u32 = 0x00000040;
    pub const DMA: u32 = 1 << 6;

    /// **CORE1**
    ///
    /// If 1, UART0 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CORE1_OFFSET: u32 = 5;
    pub const CORE1_MASK: u32 = 0x00000020;
    pub const CORE1: u32 = 1 << 5;

    /// **CORE0**
    ///
    /// If 1, UART0 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CORE0_OFFSET: u32 = 4;
    pub const CORE0_MASK: u32 = 0x00000010;
    pub const CORE0: u32 = 1 << 4;

    /// **SP**
    ///
    /// If 1, UART0 can be accessed from a Secure, Privileged context.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SP_OFFSET: u32 = 3;
    pub const SP_MASK: u32 = 0x00000008;
    pub const SP: u32 = 1 << 3;

    /// **SU**
    ///
    /// If 1, and SP is also set, UART0 can be accessed from a Secure, Unprivileged context.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SU_OFFSET: u32 = 2;
    pub const SU_MASK: u32 = 0x00000004;
    pub const SU: u32 = 1 << 2;

    /// **NSP**
    ///
    /// If 1, UART0 can be accessed from a Non-secure, Privileged context.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const NSP_OFFSET: u32 = 1;
    pub const NSP_MASK: u32 = 0x00000002;
    pub const NSP: u32 = 1 << 1;

    /// **NSU**
    ///
    /// If 1, and NSP is also set, UART0 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const NSU_OFFSET: u32 = 0;
    pub const NSU_MASK: u32 = 0x00000001;
    pub const NSU: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DBG** `[7]` (RW)
    ///
    /// If 1, UART0 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dbg(&self) -> bool {
        (self.value.read_bits() & Self::DBG_MASK) != 0
    }

    /// Set DBG bit atomically (RW)
    #[inline(always)]
    pub fn set_dbg(&self) {
        self.set_bits(Self::DBG_MASK);
    }

    /// Clear DBG bit atomically (RW)
    #[inline(always)]
    pub fn clr_dbg(&self) {
        self.clr_bits(Self::DBG_MASK);
    }

    /// **DMA** `[6]` (RW)
    ///
    /// If 1, UART0 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dma(&self) -> bool {
        (self.value.read_bits() & Self::DMA_MASK) != 0
    }

    /// Set DMA bit atomically (RW)
    #[inline(always)]
    pub fn set_dma(&self) {
        self.set_bits(Self::DMA_MASK);
    }

    /// Clear DMA bit atomically (RW)
    #[inline(always)]
    pub fn clr_dma(&self) {
        self.clr_bits(Self::DMA_MASK);
    }

    /// **CORE1** `[5]` (RW)
    ///
    /// If 1, UART0 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core1(&self) -> bool {
        (self.value.read_bits() & Self::CORE1_MASK) != 0
    }

    /// Set CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn set_core1(&self) {
        self.set_bits(Self::CORE1_MASK);
    }

    /// Clear CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core1(&self) {
        self.clr_bits(Self::CORE1_MASK);
    }

    /// **CORE0** `[4]` (RW)
    ///
    /// If 1, UART0 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core0(&self) -> bool {
        (self.value.read_bits() & Self::CORE0_MASK) != 0
    }

    /// Set CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn set_core0(&self) {
        self.set_bits(Self::CORE0_MASK);
    }

    /// Clear CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core0(&self) {
        self.clr_bits(Self::CORE0_MASK);
    }

    /// **SP** `[3]` (RW)
    ///
    /// If 1, UART0 can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub fn sp(&self) -> bool {
        (self.value.read_bits() & Self::SP_MASK) != 0
    }

    /// Set SP bit atomically (RW)
    #[inline(always)]
    pub fn set_sp(&self) {
        self.set_bits(Self::SP_MASK);
    }

    /// Clear SP bit atomically (RW)
    #[inline(always)]
    pub fn clr_sp(&self) {
        self.clr_bits(Self::SP_MASK);
    }

    /// **SU** `[2]` (RW)
    ///
    /// If 1, and SP is also set, UART0 can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub fn su(&self) -> bool {
        (self.value.read_bits() & Self::SU_MASK) != 0
    }

    /// Set SU bit atomically (RW)
    #[inline(always)]
    pub fn set_su(&self) {
        self.set_bits(Self::SU_MASK);
    }

    /// Clear SU bit atomically (RW)
    #[inline(always)]
    pub fn clr_su(&self) {
        self.clr_bits(Self::SU_MASK);
    }

    /// **NSP** `[1]` (RW)
    ///
    /// If 1, UART0 can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub fn nsp(&self) -> bool {
        (self.value.read_bits() & Self::NSP_MASK) != 0
    }

    /// Set NSP bit atomically (RW)
    #[inline(always)]
    pub fn set_nsp(&self) {
        self.set_bits(Self::NSP_MASK);
    }

    /// Clear NSP bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsp(&self) {
        self.clr_bits(Self::NSP_MASK);
    }

    /// **NSU** `[0]` (RW)
    ///
    /// If 1, and NSP is also set, UART0 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub fn nsu(&self) -> bool {
        (self.value.read_bits() & Self::NSU_MASK) != 0
    }

    /// Set NSU bit atomically (RW)
    #[inline(always)]
    pub fn set_nsu(&self) {
        self.set_bits(Self::NSU_MASK);
    }

    /// Clear NSU bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsu(&self) {
        self.clr_bits(Self::NSU_MASK);
    }
}

/// UART0 value type for batch read/modify/write operations
///
/// Use `uart0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `uart0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Uart0Val {
    bits: u32,
}

impl Uart0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x000000fc }
    }

    /// Read **DBG** `[7:7]`
    ///
    /// If 1, UART0 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        (self.bits & Uart0::DBG_MASK) != 0
    }

    /// Read **DMA** `[6:6]`
    ///
    /// If 1, UART0 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        (self.bits & Uart0::DMA_MASK) != 0
    }

    /// Read **CORE1** `[5:5]`
    ///
    /// If 1, UART0 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        (self.bits & Uart0::CORE1_MASK) != 0
    }

    /// Read **CORE0** `[4:4]`
    ///
    /// If 1, UART0 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        (self.bits & Uart0::CORE0_MASK) != 0
    }

    /// Read **SP** `[3:3]`
    ///
    /// If 1, UART0 can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        (self.bits & Uart0::SP_MASK) != 0
    }

    /// Read **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, UART0 can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn su(&self) -> bool {
        (self.bits & Uart0::SU_MASK) != 0
    }

    /// Read **NSP** `[1:1]`
    ///
    /// If 1, UART0 can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        (self.bits & Uart0::NSP_MASK) != 0
    }

    /// Read **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, UART0 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        (self.bits & Uart0::NSU_MASK) != 0
    }

    /// Set **DBG** `[7:7]`
    ///
    /// If 1, UART0 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dbg(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uart0::DBG_MASK
            } else {
                self.bits & !Uart0::DBG_MASK
            }
        }
    }

    /// Set **DMA** `[6:6]`
    ///
    /// If 1, UART0 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dma(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uart0::DMA_MASK
            } else {
                self.bits & !Uart0::DMA_MASK
            }
        }
    }

    /// Set **CORE1** `[5:5]`
    ///
    /// If 1, UART0 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uart0::CORE1_MASK
            } else {
                self.bits & !Uart0::CORE1_MASK
            }
        }
    }

    /// Set **CORE0** `[4:4]`
    ///
    /// If 1, UART0 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uart0::CORE0_MASK
            } else {
                self.bits & !Uart0::CORE0_MASK
            }
        }
    }

    /// Set **SP** `[3:3]`
    ///
    /// If 1, UART0 can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn with_sp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uart0::SP_MASK
            } else {
                self.bits & !Uart0::SP_MASK
            }
        }
    }

    /// Set **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, UART0 can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn with_su(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uart0::SU_MASK
            } else {
                self.bits & !Uart0::SU_MASK
            }
        }
    }

    /// Set **NSP** `[1:1]`
    ///
    /// If 1, UART0 can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn with_nsp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uart0::NSP_MASK
            } else {
                self.bits & !Uart0::NSP_MASK
            }
        }
    }

    /// Set **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, UART0 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn with_nsu(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uart0::NSU_MASK
            } else {
                self.bits & !Uart0::NSU_MASK
            }
        }
    }
}

impl Default for Uart0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Uart0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Uart0Val {
        Uart0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Uart0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Uart0Val) -> Uart0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// UART1 Register
///
/// Control whether debugger, DMA, core 0 and core 1 can access UART1, and at what security/privilege levels they can do so.
/// Defaults to Secure access from any master.
/// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
///
/// Address offset: 0x00a4
/// Reset value: 0x000000fc
///
/// ## Fields
///
/// - **DBG** `[7]` (RW)
///
///   If 1, UART1 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **DMA** `[6]` (RW)
///
///   If 1, UART1 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE1** `[5]` (RW)
///
///   If 1, UART1 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE0** `[4]` (RW)
///
///   If 1, UART1 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **SP** `[3]` (RW)
///
///   If 1, UART1 can be accessed from a Secure, Privileged context.
///
/// - **SU** `[2]` (RW)
///
///   If 1, and SP is also set, UART1 can be accessed from a Secure, Unprivileged context.
///
/// - **NSP** `[1]` (RW)
///
///   If 1, UART1 can be accessed from a Non-secure, Privileged context.
///
/// - **NSU** `[0]` (RW)
///
///   If 1, and NSP is also set, UART1 can be accessed from a Non-secure, Unprivileged context.
///   This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
#[repr(transparent)]
pub struct Uart1 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Uart1 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Uart1 {
    // ======== Field Constants ========

    /// **DBG**
    ///
    /// If 1, UART1 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const DBG_OFFSET: u32 = 7;
    pub const DBG_MASK: u32 = 0x00000080;
    pub const DBG: u32 = 1 << 7;

    /// **DMA**
    ///
    /// If 1, UART1 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const DMA_OFFSET: u32 = 6;
    pub const DMA_MASK: u32 = 0x00000040;
    pub const DMA: u32 = 1 << 6;

    /// **CORE1**
    ///
    /// If 1, UART1 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CORE1_OFFSET: u32 = 5;
    pub const CORE1_MASK: u32 = 0x00000020;
    pub const CORE1: u32 = 1 << 5;

    /// **CORE0**
    ///
    /// If 1, UART1 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CORE0_OFFSET: u32 = 4;
    pub const CORE0_MASK: u32 = 0x00000010;
    pub const CORE0: u32 = 1 << 4;

    /// **SP**
    ///
    /// If 1, UART1 can be accessed from a Secure, Privileged context.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SP_OFFSET: u32 = 3;
    pub const SP_MASK: u32 = 0x00000008;
    pub const SP: u32 = 1 << 3;

    /// **SU**
    ///
    /// If 1, and SP is also set, UART1 can be accessed from a Secure, Unprivileged context.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SU_OFFSET: u32 = 2;
    pub const SU_MASK: u32 = 0x00000004;
    pub const SU: u32 = 1 << 2;

    /// **NSP**
    ///
    /// If 1, UART1 can be accessed from a Non-secure, Privileged context.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const NSP_OFFSET: u32 = 1;
    pub const NSP_MASK: u32 = 0x00000002;
    pub const NSP: u32 = 1 << 1;

    /// **NSU**
    ///
    /// If 1, and NSP is also set, UART1 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const NSU_OFFSET: u32 = 0;
    pub const NSU_MASK: u32 = 0x00000001;
    pub const NSU: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DBG** `[7]` (RW)
    ///
    /// If 1, UART1 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dbg(&self) -> bool {
        (self.value.read_bits() & Self::DBG_MASK) != 0
    }

    /// Set DBG bit atomically (RW)
    #[inline(always)]
    pub fn set_dbg(&self) {
        self.set_bits(Self::DBG_MASK);
    }

    /// Clear DBG bit atomically (RW)
    #[inline(always)]
    pub fn clr_dbg(&self) {
        self.clr_bits(Self::DBG_MASK);
    }

    /// **DMA** `[6]` (RW)
    ///
    /// If 1, UART1 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dma(&self) -> bool {
        (self.value.read_bits() & Self::DMA_MASK) != 0
    }

    /// Set DMA bit atomically (RW)
    #[inline(always)]
    pub fn set_dma(&self) {
        self.set_bits(Self::DMA_MASK);
    }

    /// Clear DMA bit atomically (RW)
    #[inline(always)]
    pub fn clr_dma(&self) {
        self.clr_bits(Self::DMA_MASK);
    }

    /// **CORE1** `[5]` (RW)
    ///
    /// If 1, UART1 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core1(&self) -> bool {
        (self.value.read_bits() & Self::CORE1_MASK) != 0
    }

    /// Set CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn set_core1(&self) {
        self.set_bits(Self::CORE1_MASK);
    }

    /// Clear CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core1(&self) {
        self.clr_bits(Self::CORE1_MASK);
    }

    /// **CORE0** `[4]` (RW)
    ///
    /// If 1, UART1 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core0(&self) -> bool {
        (self.value.read_bits() & Self::CORE0_MASK) != 0
    }

    /// Set CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn set_core0(&self) {
        self.set_bits(Self::CORE0_MASK);
    }

    /// Clear CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core0(&self) {
        self.clr_bits(Self::CORE0_MASK);
    }

    /// **SP** `[3]` (RW)
    ///
    /// If 1, UART1 can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub fn sp(&self) -> bool {
        (self.value.read_bits() & Self::SP_MASK) != 0
    }

    /// Set SP bit atomically (RW)
    #[inline(always)]
    pub fn set_sp(&self) {
        self.set_bits(Self::SP_MASK);
    }

    /// Clear SP bit atomically (RW)
    #[inline(always)]
    pub fn clr_sp(&self) {
        self.clr_bits(Self::SP_MASK);
    }

    /// **SU** `[2]` (RW)
    ///
    /// If 1, and SP is also set, UART1 can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub fn su(&self) -> bool {
        (self.value.read_bits() & Self::SU_MASK) != 0
    }

    /// Set SU bit atomically (RW)
    #[inline(always)]
    pub fn set_su(&self) {
        self.set_bits(Self::SU_MASK);
    }

    /// Clear SU bit atomically (RW)
    #[inline(always)]
    pub fn clr_su(&self) {
        self.clr_bits(Self::SU_MASK);
    }

    /// **NSP** `[1]` (RW)
    ///
    /// If 1, UART1 can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub fn nsp(&self) -> bool {
        (self.value.read_bits() & Self::NSP_MASK) != 0
    }

    /// Set NSP bit atomically (RW)
    #[inline(always)]
    pub fn set_nsp(&self) {
        self.set_bits(Self::NSP_MASK);
    }

    /// Clear NSP bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsp(&self) {
        self.clr_bits(Self::NSP_MASK);
    }

    /// **NSU** `[0]` (RW)
    ///
    /// If 1, and NSP is also set, UART1 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub fn nsu(&self) -> bool {
        (self.value.read_bits() & Self::NSU_MASK) != 0
    }

    /// Set NSU bit atomically (RW)
    #[inline(always)]
    pub fn set_nsu(&self) {
        self.set_bits(Self::NSU_MASK);
    }

    /// Clear NSU bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsu(&self) {
        self.clr_bits(Self::NSU_MASK);
    }
}

/// UART1 value type for batch read/modify/write operations
///
/// Use `uart1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `uart1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Uart1Val {
    bits: u32,
}

impl Uart1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x000000fc }
    }

    /// Read **DBG** `[7:7]`
    ///
    /// If 1, UART1 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        (self.bits & Uart1::DBG_MASK) != 0
    }

    /// Read **DMA** `[6:6]`
    ///
    /// If 1, UART1 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        (self.bits & Uart1::DMA_MASK) != 0
    }

    /// Read **CORE1** `[5:5]`
    ///
    /// If 1, UART1 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        (self.bits & Uart1::CORE1_MASK) != 0
    }

    /// Read **CORE0** `[4:4]`
    ///
    /// If 1, UART1 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        (self.bits & Uart1::CORE0_MASK) != 0
    }

    /// Read **SP** `[3:3]`
    ///
    /// If 1, UART1 can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        (self.bits & Uart1::SP_MASK) != 0
    }

    /// Read **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, UART1 can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn su(&self) -> bool {
        (self.bits & Uart1::SU_MASK) != 0
    }

    /// Read **NSP** `[1:1]`
    ///
    /// If 1, UART1 can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        (self.bits & Uart1::NSP_MASK) != 0
    }

    /// Read **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, UART1 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        (self.bits & Uart1::NSU_MASK) != 0
    }

    /// Set **DBG** `[7:7]`
    ///
    /// If 1, UART1 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dbg(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uart1::DBG_MASK
            } else {
                self.bits & !Uart1::DBG_MASK
            }
        }
    }

    /// Set **DMA** `[6:6]`
    ///
    /// If 1, UART1 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dma(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uart1::DMA_MASK
            } else {
                self.bits & !Uart1::DMA_MASK
            }
        }
    }

    /// Set **CORE1** `[5:5]`
    ///
    /// If 1, UART1 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uart1::CORE1_MASK
            } else {
                self.bits & !Uart1::CORE1_MASK
            }
        }
    }

    /// Set **CORE0** `[4:4]`
    ///
    /// If 1, UART1 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uart1::CORE0_MASK
            } else {
                self.bits & !Uart1::CORE0_MASK
            }
        }
    }

    /// Set **SP** `[3:3]`
    ///
    /// If 1, UART1 can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn with_sp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uart1::SP_MASK
            } else {
                self.bits & !Uart1::SP_MASK
            }
        }
    }

    /// Set **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, UART1 can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn with_su(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uart1::SU_MASK
            } else {
                self.bits & !Uart1::SU_MASK
            }
        }
    }

    /// Set **NSP** `[1:1]`
    ///
    /// If 1, UART1 can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn with_nsp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uart1::NSP_MASK
            } else {
                self.bits & !Uart1::NSP_MASK
            }
        }
    }

    /// Set **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, UART1 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn with_nsu(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Uart1::NSU_MASK
            } else {
                self.bits & !Uart1::NSU_MASK
            }
        }
    }
}

impl Default for Uart1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Uart1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Uart1Val {
        Uart1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Uart1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Uart1Val) -> Uart1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// OTP Register
///
/// Control whether debugger, DMA, core 0 and core 1 can access OTP, and at what security/privilege levels they can do so.
/// Defaults to Secure access from any master.
/// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
///
/// Address offset: 0x00a8
/// Reset value: 0x000000fc
///
/// ## Fields
///
/// - **DBG** `[7]` (RW)
///
///   If 1, OTP can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **DMA** `[6]` (RW)
///
///   If 1, OTP can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE1** `[5]` (RW)
///
///   If 1, OTP can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE0** `[4]` (RW)
///
///   If 1, OTP can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **SP** `[3]` (RW)
///
///   If 1, OTP can be accessed from a Secure, Privileged context.
///
/// - **SU** `[2]` (RW)
///
///   If 1, and SP is also set, OTP can be accessed from a Secure, Unprivileged context.
///
/// - **NSP** `[1]` (RW)
///
///   If 1, OTP can be accessed from a Non-secure, Privileged context.
///
/// - **NSU** `[0]` (RW)
///
///   If 1, and NSP is also set, OTP can be accessed from a Non-secure, Unprivileged context.
///   This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
#[repr(transparent)]
pub struct Otp {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Otp {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Otp {
    // ======== Field Constants ========

    /// **DBG**
    ///
    /// If 1, OTP can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const DBG_OFFSET: u32 = 7;
    pub const DBG_MASK: u32 = 0x00000080;
    pub const DBG: u32 = 1 << 7;

    /// **DMA**
    ///
    /// If 1, OTP can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const DMA_OFFSET: u32 = 6;
    pub const DMA_MASK: u32 = 0x00000040;
    pub const DMA: u32 = 1 << 6;

    /// **CORE1**
    ///
    /// If 1, OTP can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CORE1_OFFSET: u32 = 5;
    pub const CORE1_MASK: u32 = 0x00000020;
    pub const CORE1: u32 = 1 << 5;

    /// **CORE0**
    ///
    /// If 1, OTP can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CORE0_OFFSET: u32 = 4;
    pub const CORE0_MASK: u32 = 0x00000010;
    pub const CORE0: u32 = 1 << 4;

    /// **SP**
    ///
    /// If 1, OTP can be accessed from a Secure, Privileged context.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SP_OFFSET: u32 = 3;
    pub const SP_MASK: u32 = 0x00000008;
    pub const SP: u32 = 1 << 3;

    /// **SU**
    ///
    /// If 1, and SP is also set, OTP can be accessed from a Secure, Unprivileged context.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SU_OFFSET: u32 = 2;
    pub const SU_MASK: u32 = 0x00000004;
    pub const SU: u32 = 1 << 2;

    /// **NSP**
    ///
    /// If 1, OTP can be accessed from a Non-secure, Privileged context.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const NSP_OFFSET: u32 = 1;
    pub const NSP_MASK: u32 = 0x00000002;
    pub const NSP: u32 = 1 << 1;

    /// **NSU**
    ///
    /// If 1, and NSP is also set, OTP can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const NSU_OFFSET: u32 = 0;
    pub const NSU_MASK: u32 = 0x00000001;
    pub const NSU: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DBG** `[7]` (RW)
    ///
    /// If 1, OTP can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dbg(&self) -> bool {
        (self.value.read_bits() & Self::DBG_MASK) != 0
    }

    /// Set DBG bit atomically (RW)
    #[inline(always)]
    pub fn set_dbg(&self) {
        self.set_bits(Self::DBG_MASK);
    }

    /// Clear DBG bit atomically (RW)
    #[inline(always)]
    pub fn clr_dbg(&self) {
        self.clr_bits(Self::DBG_MASK);
    }

    /// **DMA** `[6]` (RW)
    ///
    /// If 1, OTP can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dma(&self) -> bool {
        (self.value.read_bits() & Self::DMA_MASK) != 0
    }

    /// Set DMA bit atomically (RW)
    #[inline(always)]
    pub fn set_dma(&self) {
        self.set_bits(Self::DMA_MASK);
    }

    /// Clear DMA bit atomically (RW)
    #[inline(always)]
    pub fn clr_dma(&self) {
        self.clr_bits(Self::DMA_MASK);
    }

    /// **CORE1** `[5]` (RW)
    ///
    /// If 1, OTP can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core1(&self) -> bool {
        (self.value.read_bits() & Self::CORE1_MASK) != 0
    }

    /// Set CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn set_core1(&self) {
        self.set_bits(Self::CORE1_MASK);
    }

    /// Clear CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core1(&self) {
        self.clr_bits(Self::CORE1_MASK);
    }

    /// **CORE0** `[4]` (RW)
    ///
    /// If 1, OTP can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core0(&self) -> bool {
        (self.value.read_bits() & Self::CORE0_MASK) != 0
    }

    /// Set CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn set_core0(&self) {
        self.set_bits(Self::CORE0_MASK);
    }

    /// Clear CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core0(&self) {
        self.clr_bits(Self::CORE0_MASK);
    }

    /// **SP** `[3]` (RW)
    ///
    /// If 1, OTP can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub fn sp(&self) -> bool {
        (self.value.read_bits() & Self::SP_MASK) != 0
    }

    /// Set SP bit atomically (RW)
    #[inline(always)]
    pub fn set_sp(&self) {
        self.set_bits(Self::SP_MASK);
    }

    /// Clear SP bit atomically (RW)
    #[inline(always)]
    pub fn clr_sp(&self) {
        self.clr_bits(Self::SP_MASK);
    }

    /// **SU** `[2]` (RW)
    ///
    /// If 1, and SP is also set, OTP can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub fn su(&self) -> bool {
        (self.value.read_bits() & Self::SU_MASK) != 0
    }

    /// Set SU bit atomically (RW)
    #[inline(always)]
    pub fn set_su(&self) {
        self.set_bits(Self::SU_MASK);
    }

    /// Clear SU bit atomically (RW)
    #[inline(always)]
    pub fn clr_su(&self) {
        self.clr_bits(Self::SU_MASK);
    }

    /// **NSP** `[1]` (RW)
    ///
    /// If 1, OTP can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub fn nsp(&self) -> bool {
        (self.value.read_bits() & Self::NSP_MASK) != 0
    }

    /// Set NSP bit atomically (RW)
    #[inline(always)]
    pub fn set_nsp(&self) {
        self.set_bits(Self::NSP_MASK);
    }

    /// Clear NSP bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsp(&self) {
        self.clr_bits(Self::NSP_MASK);
    }

    /// **NSU** `[0]` (RW)
    ///
    /// If 1, and NSP is also set, OTP can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub fn nsu(&self) -> bool {
        (self.value.read_bits() & Self::NSU_MASK) != 0
    }

    /// Set NSU bit atomically (RW)
    #[inline(always)]
    pub fn set_nsu(&self) {
        self.set_bits(Self::NSU_MASK);
    }

    /// Clear NSU bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsu(&self) {
        self.clr_bits(Self::NSU_MASK);
    }
}

/// OTP value type for batch read/modify/write operations
///
/// Use `otp.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `otp.write_val(val)`.
#[derive(Clone, Copy)]
pub struct OtpVal {
    bits: u32,
}

impl OtpVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x000000fc }
    }

    /// Read **DBG** `[7:7]`
    ///
    /// If 1, OTP can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        (self.bits & Otp::DBG_MASK) != 0
    }

    /// Read **DMA** `[6:6]`
    ///
    /// If 1, OTP can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        (self.bits & Otp::DMA_MASK) != 0
    }

    /// Read **CORE1** `[5:5]`
    ///
    /// If 1, OTP can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        (self.bits & Otp::CORE1_MASK) != 0
    }

    /// Read **CORE0** `[4:4]`
    ///
    /// If 1, OTP can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        (self.bits & Otp::CORE0_MASK) != 0
    }

    /// Read **SP** `[3:3]`
    ///
    /// If 1, OTP can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        (self.bits & Otp::SP_MASK) != 0
    }

    /// Read **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, OTP can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn su(&self) -> bool {
        (self.bits & Otp::SU_MASK) != 0
    }

    /// Read **NSP** `[1:1]`
    ///
    /// If 1, OTP can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        (self.bits & Otp::NSP_MASK) != 0
    }

    /// Read **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, OTP can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        (self.bits & Otp::NSU_MASK) != 0
    }

    /// Set **DBG** `[7:7]`
    ///
    /// If 1, OTP can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dbg(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Otp::DBG_MASK
            } else {
                self.bits & !Otp::DBG_MASK
            }
        }
    }

    /// Set **DMA** `[6:6]`
    ///
    /// If 1, OTP can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dma(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Otp::DMA_MASK
            } else {
                self.bits & !Otp::DMA_MASK
            }
        }
    }

    /// Set **CORE1** `[5:5]`
    ///
    /// If 1, OTP can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Otp::CORE1_MASK
            } else {
                self.bits & !Otp::CORE1_MASK
            }
        }
    }

    /// Set **CORE0** `[4:4]`
    ///
    /// If 1, OTP can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Otp::CORE0_MASK
            } else {
                self.bits & !Otp::CORE0_MASK
            }
        }
    }

    /// Set **SP** `[3:3]`
    ///
    /// If 1, OTP can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn with_sp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Otp::SP_MASK
            } else {
                self.bits & !Otp::SP_MASK
            }
        }
    }

    /// Set **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, OTP can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn with_su(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Otp::SU_MASK
            } else {
                self.bits & !Otp::SU_MASK
            }
        }
    }

    /// Set **NSP** `[1:1]`
    ///
    /// If 1, OTP can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn with_nsp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Otp::NSP_MASK
            } else {
                self.bits & !Otp::NSP_MASK
            }
        }
    }

    /// Set **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, OTP can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn with_nsu(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Otp::NSU_MASK
            } else {
                self.bits & !Otp::NSU_MASK
            }
        }
    }
}

impl Default for OtpVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Otp {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> OtpVal {
        OtpVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: OtpVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(OtpVal) -> OtpVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TBMAN Register
///
/// Control whether debugger, DMA, core 0 and core 1 can access TBMAN, and at what security/privilege levels they can do so.
/// Defaults to Secure access from any master.
/// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
///
/// Address offset: 0x00ac
/// Reset value: 0x000000fc
///
/// ## Fields
///
/// - **DBG** `[7]` (RW)
///
///   If 1, TBMAN can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **DMA** `[6]` (RW)
///
///   If 1, TBMAN can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE1** `[5]` (RW)
///
///   If 1, TBMAN can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE0** `[4]` (RW)
///
///   If 1, TBMAN can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **SP** `[3]` (RW)
///
///   If 1, TBMAN can be accessed from a Secure, Privileged context.
///
/// - **SU** `[2]` (RW)
///
///   If 1, and SP is also set, TBMAN can be accessed from a Secure, Unprivileged context.
///
/// - **NSP** `[1]` (RW)
///
///   If 1, TBMAN can be accessed from a Non-secure, Privileged context.
///
/// - **NSU** `[0]` (RW)
///
///   If 1, and NSP is also set, TBMAN can be accessed from a Non-secure, Unprivileged context.
///   This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
#[repr(transparent)]
pub struct Tbman {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Tbman {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Tbman {
    // ======== Field Constants ========

    /// **DBG**
    ///
    /// If 1, TBMAN can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const DBG_OFFSET: u32 = 7;
    pub const DBG_MASK: u32 = 0x00000080;
    pub const DBG: u32 = 1 << 7;

    /// **DMA**
    ///
    /// If 1, TBMAN can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const DMA_OFFSET: u32 = 6;
    pub const DMA_MASK: u32 = 0x00000040;
    pub const DMA: u32 = 1 << 6;

    /// **CORE1**
    ///
    /// If 1, TBMAN can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CORE1_OFFSET: u32 = 5;
    pub const CORE1_MASK: u32 = 0x00000020;
    pub const CORE1: u32 = 1 << 5;

    /// **CORE0**
    ///
    /// If 1, TBMAN can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CORE0_OFFSET: u32 = 4;
    pub const CORE0_MASK: u32 = 0x00000010;
    pub const CORE0: u32 = 1 << 4;

    /// **SP**
    ///
    /// If 1, TBMAN can be accessed from a Secure, Privileged context.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SP_OFFSET: u32 = 3;
    pub const SP_MASK: u32 = 0x00000008;
    pub const SP: u32 = 1 << 3;

    /// **SU**
    ///
    /// If 1, and SP is also set, TBMAN can be accessed from a Secure, Unprivileged context.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SU_OFFSET: u32 = 2;
    pub const SU_MASK: u32 = 0x00000004;
    pub const SU: u32 = 1 << 2;

    /// **NSP**
    ///
    /// If 1, TBMAN can be accessed from a Non-secure, Privileged context.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const NSP_OFFSET: u32 = 1;
    pub const NSP_MASK: u32 = 0x00000002;
    pub const NSP: u32 = 1 << 1;

    /// **NSU**
    ///
    /// If 1, and NSP is also set, TBMAN can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const NSU_OFFSET: u32 = 0;
    pub const NSU_MASK: u32 = 0x00000001;
    pub const NSU: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DBG** `[7]` (RW)
    ///
    /// If 1, TBMAN can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dbg(&self) -> bool {
        (self.value.read_bits() & Self::DBG_MASK) != 0
    }

    /// Set DBG bit atomically (RW)
    #[inline(always)]
    pub fn set_dbg(&self) {
        self.set_bits(Self::DBG_MASK);
    }

    /// Clear DBG bit atomically (RW)
    #[inline(always)]
    pub fn clr_dbg(&self) {
        self.clr_bits(Self::DBG_MASK);
    }

    /// **DMA** `[6]` (RW)
    ///
    /// If 1, TBMAN can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dma(&self) -> bool {
        (self.value.read_bits() & Self::DMA_MASK) != 0
    }

    /// Set DMA bit atomically (RW)
    #[inline(always)]
    pub fn set_dma(&self) {
        self.set_bits(Self::DMA_MASK);
    }

    /// Clear DMA bit atomically (RW)
    #[inline(always)]
    pub fn clr_dma(&self) {
        self.clr_bits(Self::DMA_MASK);
    }

    /// **CORE1** `[5]` (RW)
    ///
    /// If 1, TBMAN can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core1(&self) -> bool {
        (self.value.read_bits() & Self::CORE1_MASK) != 0
    }

    /// Set CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn set_core1(&self) {
        self.set_bits(Self::CORE1_MASK);
    }

    /// Clear CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core1(&self) {
        self.clr_bits(Self::CORE1_MASK);
    }

    /// **CORE0** `[4]` (RW)
    ///
    /// If 1, TBMAN can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core0(&self) -> bool {
        (self.value.read_bits() & Self::CORE0_MASK) != 0
    }

    /// Set CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn set_core0(&self) {
        self.set_bits(Self::CORE0_MASK);
    }

    /// Clear CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core0(&self) {
        self.clr_bits(Self::CORE0_MASK);
    }

    /// **SP** `[3]` (RW)
    ///
    /// If 1, TBMAN can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub fn sp(&self) -> bool {
        (self.value.read_bits() & Self::SP_MASK) != 0
    }

    /// Set SP bit atomically (RW)
    #[inline(always)]
    pub fn set_sp(&self) {
        self.set_bits(Self::SP_MASK);
    }

    /// Clear SP bit atomically (RW)
    #[inline(always)]
    pub fn clr_sp(&self) {
        self.clr_bits(Self::SP_MASK);
    }

    /// **SU** `[2]` (RW)
    ///
    /// If 1, and SP is also set, TBMAN can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub fn su(&self) -> bool {
        (self.value.read_bits() & Self::SU_MASK) != 0
    }

    /// Set SU bit atomically (RW)
    #[inline(always)]
    pub fn set_su(&self) {
        self.set_bits(Self::SU_MASK);
    }

    /// Clear SU bit atomically (RW)
    #[inline(always)]
    pub fn clr_su(&self) {
        self.clr_bits(Self::SU_MASK);
    }

    /// **NSP** `[1]` (RW)
    ///
    /// If 1, TBMAN can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub fn nsp(&self) -> bool {
        (self.value.read_bits() & Self::NSP_MASK) != 0
    }

    /// Set NSP bit atomically (RW)
    #[inline(always)]
    pub fn set_nsp(&self) {
        self.set_bits(Self::NSP_MASK);
    }

    /// Clear NSP bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsp(&self) {
        self.clr_bits(Self::NSP_MASK);
    }

    /// **NSU** `[0]` (RW)
    ///
    /// If 1, and NSP is also set, TBMAN can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub fn nsu(&self) -> bool {
        (self.value.read_bits() & Self::NSU_MASK) != 0
    }

    /// Set NSU bit atomically (RW)
    #[inline(always)]
    pub fn set_nsu(&self) {
        self.set_bits(Self::NSU_MASK);
    }

    /// Clear NSU bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsu(&self) {
        self.clr_bits(Self::NSU_MASK);
    }
}

/// TBMAN value type for batch read/modify/write operations
///
/// Use `tbman.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `tbman.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TbmanVal {
    bits: u32,
}

impl TbmanVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x000000fc }
    }

    /// Read **DBG** `[7:7]`
    ///
    /// If 1, TBMAN can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        (self.bits & Tbman::DBG_MASK) != 0
    }

    /// Read **DMA** `[6:6]`
    ///
    /// If 1, TBMAN can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        (self.bits & Tbman::DMA_MASK) != 0
    }

    /// Read **CORE1** `[5:5]`
    ///
    /// If 1, TBMAN can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        (self.bits & Tbman::CORE1_MASK) != 0
    }

    /// Read **CORE0** `[4:4]`
    ///
    /// If 1, TBMAN can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        (self.bits & Tbman::CORE0_MASK) != 0
    }

    /// Read **SP** `[3:3]`
    ///
    /// If 1, TBMAN can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        (self.bits & Tbman::SP_MASK) != 0
    }

    /// Read **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, TBMAN can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn su(&self) -> bool {
        (self.bits & Tbman::SU_MASK) != 0
    }

    /// Read **NSP** `[1:1]`
    ///
    /// If 1, TBMAN can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        (self.bits & Tbman::NSP_MASK) != 0
    }

    /// Read **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, TBMAN can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        (self.bits & Tbman::NSU_MASK) != 0
    }

    /// Set **DBG** `[7:7]`
    ///
    /// If 1, TBMAN can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dbg(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Tbman::DBG_MASK
            } else {
                self.bits & !Tbman::DBG_MASK
            }
        }
    }

    /// Set **DMA** `[6:6]`
    ///
    /// If 1, TBMAN can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dma(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Tbman::DMA_MASK
            } else {
                self.bits & !Tbman::DMA_MASK
            }
        }
    }

    /// Set **CORE1** `[5:5]`
    ///
    /// If 1, TBMAN can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Tbman::CORE1_MASK
            } else {
                self.bits & !Tbman::CORE1_MASK
            }
        }
    }

    /// Set **CORE0** `[4:4]`
    ///
    /// If 1, TBMAN can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Tbman::CORE0_MASK
            } else {
                self.bits & !Tbman::CORE0_MASK
            }
        }
    }

    /// Set **SP** `[3:3]`
    ///
    /// If 1, TBMAN can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn with_sp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Tbman::SP_MASK
            } else {
                self.bits & !Tbman::SP_MASK
            }
        }
    }

    /// Set **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, TBMAN can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn with_su(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Tbman::SU_MASK
            } else {
                self.bits & !Tbman::SU_MASK
            }
        }
    }

    /// Set **NSP** `[1:1]`
    ///
    /// If 1, TBMAN can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn with_nsp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Tbman::NSP_MASK
            } else {
                self.bits & !Tbman::NSP_MASK
            }
        }
    }

    /// Set **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, TBMAN can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn with_nsu(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Tbman::NSU_MASK
            } else {
                self.bits & !Tbman::NSU_MASK
            }
        }
    }
}

impl Default for TbmanVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Tbman {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TbmanVal {
        TbmanVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TbmanVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TbmanVal) -> TbmanVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// POWMAN Register
///
/// Control whether debugger, DMA, core 0 and core 1 can access POWMAN, and at what security/privilege levels they can do so.
/// Defaults to Secure, Privileged processor or debug access only.
/// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
///
/// Address offset: 0x00b0
/// Reset value: 0x000000b8
///
/// ## Fields
///
/// - **DBG** `[7]` (RW)
///
///   If 1, POWMAN can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **DMA** `[6]` (RW)
///
///   If 1, POWMAN can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE1** `[5]` (RW)
///
///   If 1, POWMAN can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE0** `[4]` (RW)
///
///   If 1, POWMAN can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **SP** `[3]` (RW)
///
///   If 1, POWMAN can be accessed from a Secure, Privileged context.
///
/// - **SU** `[2]` (RW)
///
///   If 1, and SP is also set, POWMAN can be accessed from a Secure, Unprivileged context.
///
/// - **NSP** `[1]` (RW)
///
///   If 1, POWMAN can be accessed from a Non-secure, Privileged context.
///
/// - **NSU** `[0]` (RW)
///
///   If 1, and NSP is also set, POWMAN can be accessed from a Non-secure, Unprivileged context.
///   This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
#[repr(transparent)]
pub struct Powman {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Powman {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Powman {
    // ======== Field Constants ========

    /// **DBG**
    ///
    /// If 1, POWMAN can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const DBG_OFFSET: u32 = 7;
    pub const DBG_MASK: u32 = 0x00000080;
    pub const DBG: u32 = 1 << 7;

    /// **DMA**
    ///
    /// If 1, POWMAN can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const DMA_OFFSET: u32 = 6;
    pub const DMA_MASK: u32 = 0x00000040;
    pub const DMA: u32 = 1 << 6;

    /// **CORE1**
    ///
    /// If 1, POWMAN can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CORE1_OFFSET: u32 = 5;
    pub const CORE1_MASK: u32 = 0x00000020;
    pub const CORE1: u32 = 1 << 5;

    /// **CORE0**
    ///
    /// If 1, POWMAN can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CORE0_OFFSET: u32 = 4;
    pub const CORE0_MASK: u32 = 0x00000010;
    pub const CORE0: u32 = 1 << 4;

    /// **SP**
    ///
    /// If 1, POWMAN can be accessed from a Secure, Privileged context.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SP_OFFSET: u32 = 3;
    pub const SP_MASK: u32 = 0x00000008;
    pub const SP: u32 = 1 << 3;

    /// **SU**
    ///
    /// If 1, and SP is also set, POWMAN can be accessed from a Secure, Unprivileged context.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SU_OFFSET: u32 = 2;
    pub const SU_MASK: u32 = 0x00000004;
    pub const SU: u32 = 1 << 2;

    /// **NSP**
    ///
    /// If 1, POWMAN can be accessed from a Non-secure, Privileged context.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const NSP_OFFSET: u32 = 1;
    pub const NSP_MASK: u32 = 0x00000002;
    pub const NSP: u32 = 1 << 1;

    /// **NSU**
    ///
    /// If 1, and NSP is also set, POWMAN can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const NSU_OFFSET: u32 = 0;
    pub const NSU_MASK: u32 = 0x00000001;
    pub const NSU: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DBG** `[7]` (RW)
    ///
    /// If 1, POWMAN can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dbg(&self) -> bool {
        (self.value.read_bits() & Self::DBG_MASK) != 0
    }

    /// Set DBG bit atomically (RW)
    #[inline(always)]
    pub fn set_dbg(&self) {
        self.set_bits(Self::DBG_MASK);
    }

    /// Clear DBG bit atomically (RW)
    #[inline(always)]
    pub fn clr_dbg(&self) {
        self.clr_bits(Self::DBG_MASK);
    }

    /// **DMA** `[6]` (RW)
    ///
    /// If 1, POWMAN can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dma(&self) -> bool {
        (self.value.read_bits() & Self::DMA_MASK) != 0
    }

    /// Set DMA bit atomically (RW)
    #[inline(always)]
    pub fn set_dma(&self) {
        self.set_bits(Self::DMA_MASK);
    }

    /// Clear DMA bit atomically (RW)
    #[inline(always)]
    pub fn clr_dma(&self) {
        self.clr_bits(Self::DMA_MASK);
    }

    /// **CORE1** `[5]` (RW)
    ///
    /// If 1, POWMAN can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core1(&self) -> bool {
        (self.value.read_bits() & Self::CORE1_MASK) != 0
    }

    /// Set CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn set_core1(&self) {
        self.set_bits(Self::CORE1_MASK);
    }

    /// Clear CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core1(&self) {
        self.clr_bits(Self::CORE1_MASK);
    }

    /// **CORE0** `[4]` (RW)
    ///
    /// If 1, POWMAN can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core0(&self) -> bool {
        (self.value.read_bits() & Self::CORE0_MASK) != 0
    }

    /// Set CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn set_core0(&self) {
        self.set_bits(Self::CORE0_MASK);
    }

    /// Clear CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core0(&self) {
        self.clr_bits(Self::CORE0_MASK);
    }

    /// **SP** `[3]` (RW)
    ///
    /// If 1, POWMAN can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub fn sp(&self) -> bool {
        (self.value.read_bits() & Self::SP_MASK) != 0
    }

    /// Set SP bit atomically (RW)
    #[inline(always)]
    pub fn set_sp(&self) {
        self.set_bits(Self::SP_MASK);
    }

    /// Clear SP bit atomically (RW)
    #[inline(always)]
    pub fn clr_sp(&self) {
        self.clr_bits(Self::SP_MASK);
    }

    /// **SU** `[2]` (RW)
    ///
    /// If 1, and SP is also set, POWMAN can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub fn su(&self) -> bool {
        (self.value.read_bits() & Self::SU_MASK) != 0
    }

    /// Set SU bit atomically (RW)
    #[inline(always)]
    pub fn set_su(&self) {
        self.set_bits(Self::SU_MASK);
    }

    /// Clear SU bit atomically (RW)
    #[inline(always)]
    pub fn clr_su(&self) {
        self.clr_bits(Self::SU_MASK);
    }

    /// **NSP** `[1]` (RW)
    ///
    /// If 1, POWMAN can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub fn nsp(&self) -> bool {
        (self.value.read_bits() & Self::NSP_MASK) != 0
    }

    /// Set NSP bit atomically (RW)
    #[inline(always)]
    pub fn set_nsp(&self) {
        self.set_bits(Self::NSP_MASK);
    }

    /// Clear NSP bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsp(&self) {
        self.clr_bits(Self::NSP_MASK);
    }

    /// **NSU** `[0]` (RW)
    ///
    /// If 1, and NSP is also set, POWMAN can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub fn nsu(&self) -> bool {
        (self.value.read_bits() & Self::NSU_MASK) != 0
    }

    /// Set NSU bit atomically (RW)
    #[inline(always)]
    pub fn set_nsu(&self) {
        self.set_bits(Self::NSU_MASK);
    }

    /// Clear NSU bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsu(&self) {
        self.clr_bits(Self::NSU_MASK);
    }
}

/// POWMAN value type for batch read/modify/write operations
///
/// Use `powman.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `powman.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PowmanVal {
    bits: u32,
}

impl PowmanVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x000000b8 }
    }

    /// Read **DBG** `[7:7]`
    ///
    /// If 1, POWMAN can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        (self.bits & Powman::DBG_MASK) != 0
    }

    /// Read **DMA** `[6:6]`
    ///
    /// If 1, POWMAN can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        (self.bits & Powman::DMA_MASK) != 0
    }

    /// Read **CORE1** `[5:5]`
    ///
    /// If 1, POWMAN can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        (self.bits & Powman::CORE1_MASK) != 0
    }

    /// Read **CORE0** `[4:4]`
    ///
    /// If 1, POWMAN can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        (self.bits & Powman::CORE0_MASK) != 0
    }

    /// Read **SP** `[3:3]`
    ///
    /// If 1, POWMAN can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        (self.bits & Powman::SP_MASK) != 0
    }

    /// Read **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, POWMAN can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn su(&self) -> bool {
        (self.bits & Powman::SU_MASK) != 0
    }

    /// Read **NSP** `[1:1]`
    ///
    /// If 1, POWMAN can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        (self.bits & Powman::NSP_MASK) != 0
    }

    /// Read **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, POWMAN can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        (self.bits & Powman::NSU_MASK) != 0
    }

    /// Set **DBG** `[7:7]`
    ///
    /// If 1, POWMAN can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dbg(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Powman::DBG_MASK
            } else {
                self.bits & !Powman::DBG_MASK
            }
        }
    }

    /// Set **DMA** `[6:6]`
    ///
    /// If 1, POWMAN can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dma(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Powman::DMA_MASK
            } else {
                self.bits & !Powman::DMA_MASK
            }
        }
    }

    /// Set **CORE1** `[5:5]`
    ///
    /// If 1, POWMAN can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Powman::CORE1_MASK
            } else {
                self.bits & !Powman::CORE1_MASK
            }
        }
    }

    /// Set **CORE0** `[4:4]`
    ///
    /// If 1, POWMAN can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Powman::CORE0_MASK
            } else {
                self.bits & !Powman::CORE0_MASK
            }
        }
    }

    /// Set **SP** `[3:3]`
    ///
    /// If 1, POWMAN can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn with_sp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Powman::SP_MASK
            } else {
                self.bits & !Powman::SP_MASK
            }
        }
    }

    /// Set **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, POWMAN can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn with_su(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Powman::SU_MASK
            } else {
                self.bits & !Powman::SU_MASK
            }
        }
    }

    /// Set **NSP** `[1:1]`
    ///
    /// If 1, POWMAN can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn with_nsp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Powman::NSP_MASK
            } else {
                self.bits & !Powman::NSP_MASK
            }
        }
    }

    /// Set **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, POWMAN can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn with_nsu(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Powman::NSU_MASK
            } else {
                self.bits & !Powman::NSU_MASK
            }
        }
    }
}

impl Default for PowmanVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Powman {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PowmanVal {
        PowmanVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: PowmanVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(PowmanVal) -> PowmanVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TRNG Register
///
/// Control whether debugger, DMA, core 0 and core 1 can access TRNG, and at what security/privilege levels they can do so.
/// Defaults to Secure, Privileged processor or debug access only.
/// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
///
/// Address offset: 0x00b4
/// Reset value: 0x000000b8
///
/// ## Fields
///
/// - **DBG** `[7]` (RW)
///
///   If 1, TRNG can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **DMA** `[6]` (RW)
///
///   If 1, TRNG can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE1** `[5]` (RW)
///
///   If 1, TRNG can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE0** `[4]` (RW)
///
///   If 1, TRNG can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **SP** `[3]` (RW)
///
///   If 1, TRNG can be accessed from a Secure, Privileged context.
///
/// - **SU** `[2]` (RW)
///
///   If 1, and SP is also set, TRNG can be accessed from a Secure, Unprivileged context.
///
/// - **NSP** `[1]` (RW)
///
///   If 1, TRNG can be accessed from a Non-secure, Privileged context.
///
/// - **NSU** `[0]` (RW)
///
///   If 1, and NSP is also set, TRNG can be accessed from a Non-secure, Unprivileged context.
///   This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
#[repr(transparent)]
pub struct Trng {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Trng {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Trng {
    // ======== Field Constants ========

    /// **DBG**
    ///
    /// If 1, TRNG can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const DBG_OFFSET: u32 = 7;
    pub const DBG_MASK: u32 = 0x00000080;
    pub const DBG: u32 = 1 << 7;

    /// **DMA**
    ///
    /// If 1, TRNG can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const DMA_OFFSET: u32 = 6;
    pub const DMA_MASK: u32 = 0x00000040;
    pub const DMA: u32 = 1 << 6;

    /// **CORE1**
    ///
    /// If 1, TRNG can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CORE1_OFFSET: u32 = 5;
    pub const CORE1_MASK: u32 = 0x00000020;
    pub const CORE1: u32 = 1 << 5;

    /// **CORE0**
    ///
    /// If 1, TRNG can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CORE0_OFFSET: u32 = 4;
    pub const CORE0_MASK: u32 = 0x00000010;
    pub const CORE0: u32 = 1 << 4;

    /// **SP**
    ///
    /// If 1, TRNG can be accessed from a Secure, Privileged context.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SP_OFFSET: u32 = 3;
    pub const SP_MASK: u32 = 0x00000008;
    pub const SP: u32 = 1 << 3;

    /// **SU**
    ///
    /// If 1, and SP is also set, TRNG can be accessed from a Secure, Unprivileged context.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SU_OFFSET: u32 = 2;
    pub const SU_MASK: u32 = 0x00000004;
    pub const SU: u32 = 1 << 2;

    /// **NSP**
    ///
    /// If 1, TRNG can be accessed from a Non-secure, Privileged context.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const NSP_OFFSET: u32 = 1;
    pub const NSP_MASK: u32 = 0x00000002;
    pub const NSP: u32 = 1 << 1;

    /// **NSU**
    ///
    /// If 1, and NSP is also set, TRNG can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const NSU_OFFSET: u32 = 0;
    pub const NSU_MASK: u32 = 0x00000001;
    pub const NSU: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DBG** `[7]` (RW)
    ///
    /// If 1, TRNG can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dbg(&self) -> bool {
        (self.value.read_bits() & Self::DBG_MASK) != 0
    }

    /// Set DBG bit atomically (RW)
    #[inline(always)]
    pub fn set_dbg(&self) {
        self.set_bits(Self::DBG_MASK);
    }

    /// Clear DBG bit atomically (RW)
    #[inline(always)]
    pub fn clr_dbg(&self) {
        self.clr_bits(Self::DBG_MASK);
    }

    /// **DMA** `[6]` (RW)
    ///
    /// If 1, TRNG can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dma(&self) -> bool {
        (self.value.read_bits() & Self::DMA_MASK) != 0
    }

    /// Set DMA bit atomically (RW)
    #[inline(always)]
    pub fn set_dma(&self) {
        self.set_bits(Self::DMA_MASK);
    }

    /// Clear DMA bit atomically (RW)
    #[inline(always)]
    pub fn clr_dma(&self) {
        self.clr_bits(Self::DMA_MASK);
    }

    /// **CORE1** `[5]` (RW)
    ///
    /// If 1, TRNG can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core1(&self) -> bool {
        (self.value.read_bits() & Self::CORE1_MASK) != 0
    }

    /// Set CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn set_core1(&self) {
        self.set_bits(Self::CORE1_MASK);
    }

    /// Clear CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core1(&self) {
        self.clr_bits(Self::CORE1_MASK);
    }

    /// **CORE0** `[4]` (RW)
    ///
    /// If 1, TRNG can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core0(&self) -> bool {
        (self.value.read_bits() & Self::CORE0_MASK) != 0
    }

    /// Set CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn set_core0(&self) {
        self.set_bits(Self::CORE0_MASK);
    }

    /// Clear CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core0(&self) {
        self.clr_bits(Self::CORE0_MASK);
    }

    /// **SP** `[3]` (RW)
    ///
    /// If 1, TRNG can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub fn sp(&self) -> bool {
        (self.value.read_bits() & Self::SP_MASK) != 0
    }

    /// Set SP bit atomically (RW)
    #[inline(always)]
    pub fn set_sp(&self) {
        self.set_bits(Self::SP_MASK);
    }

    /// Clear SP bit atomically (RW)
    #[inline(always)]
    pub fn clr_sp(&self) {
        self.clr_bits(Self::SP_MASK);
    }

    /// **SU** `[2]` (RW)
    ///
    /// If 1, and SP is also set, TRNG can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub fn su(&self) -> bool {
        (self.value.read_bits() & Self::SU_MASK) != 0
    }

    /// Set SU bit atomically (RW)
    #[inline(always)]
    pub fn set_su(&self) {
        self.set_bits(Self::SU_MASK);
    }

    /// Clear SU bit atomically (RW)
    #[inline(always)]
    pub fn clr_su(&self) {
        self.clr_bits(Self::SU_MASK);
    }

    /// **NSP** `[1]` (RW)
    ///
    /// If 1, TRNG can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub fn nsp(&self) -> bool {
        (self.value.read_bits() & Self::NSP_MASK) != 0
    }

    /// Set NSP bit atomically (RW)
    #[inline(always)]
    pub fn set_nsp(&self) {
        self.set_bits(Self::NSP_MASK);
    }

    /// Clear NSP bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsp(&self) {
        self.clr_bits(Self::NSP_MASK);
    }

    /// **NSU** `[0]` (RW)
    ///
    /// If 1, and NSP is also set, TRNG can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub fn nsu(&self) -> bool {
        (self.value.read_bits() & Self::NSU_MASK) != 0
    }

    /// Set NSU bit atomically (RW)
    #[inline(always)]
    pub fn set_nsu(&self) {
        self.set_bits(Self::NSU_MASK);
    }

    /// Clear NSU bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsu(&self) {
        self.clr_bits(Self::NSU_MASK);
    }
}

/// TRNG value type for batch read/modify/write operations
///
/// Use `trng.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `trng.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TrngVal {
    bits: u32,
}

impl TrngVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x000000b8 }
    }

    /// Read **DBG** `[7:7]`
    ///
    /// If 1, TRNG can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        (self.bits & Trng::DBG_MASK) != 0
    }

    /// Read **DMA** `[6:6]`
    ///
    /// If 1, TRNG can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        (self.bits & Trng::DMA_MASK) != 0
    }

    /// Read **CORE1** `[5:5]`
    ///
    /// If 1, TRNG can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        (self.bits & Trng::CORE1_MASK) != 0
    }

    /// Read **CORE0** `[4:4]`
    ///
    /// If 1, TRNG can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        (self.bits & Trng::CORE0_MASK) != 0
    }

    /// Read **SP** `[3:3]`
    ///
    /// If 1, TRNG can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        (self.bits & Trng::SP_MASK) != 0
    }

    /// Read **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, TRNG can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn su(&self) -> bool {
        (self.bits & Trng::SU_MASK) != 0
    }

    /// Read **NSP** `[1:1]`
    ///
    /// If 1, TRNG can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        (self.bits & Trng::NSP_MASK) != 0
    }

    /// Read **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, TRNG can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        (self.bits & Trng::NSU_MASK) != 0
    }

    /// Set **DBG** `[7:7]`
    ///
    /// If 1, TRNG can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dbg(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Trng::DBG_MASK
            } else {
                self.bits & !Trng::DBG_MASK
            }
        }
    }

    /// Set **DMA** `[6:6]`
    ///
    /// If 1, TRNG can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dma(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Trng::DMA_MASK
            } else {
                self.bits & !Trng::DMA_MASK
            }
        }
    }

    /// Set **CORE1** `[5:5]`
    ///
    /// If 1, TRNG can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Trng::CORE1_MASK
            } else {
                self.bits & !Trng::CORE1_MASK
            }
        }
    }

    /// Set **CORE0** `[4:4]`
    ///
    /// If 1, TRNG can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Trng::CORE0_MASK
            } else {
                self.bits & !Trng::CORE0_MASK
            }
        }
    }

    /// Set **SP** `[3:3]`
    ///
    /// If 1, TRNG can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn with_sp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Trng::SP_MASK
            } else {
                self.bits & !Trng::SP_MASK
            }
        }
    }

    /// Set **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, TRNG can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn with_su(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Trng::SU_MASK
            } else {
                self.bits & !Trng::SU_MASK
            }
        }
    }

    /// Set **NSP** `[1:1]`
    ///
    /// If 1, TRNG can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn with_nsp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Trng::NSP_MASK
            } else {
                self.bits & !Trng::NSP_MASK
            }
        }
    }

    /// Set **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, TRNG can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn with_nsu(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Trng::NSU_MASK
            } else {
                self.bits & !Trng::NSU_MASK
            }
        }
    }
}

impl Default for TrngVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Trng {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TrngVal {
        TrngVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TrngVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TrngVal) -> TrngVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SHA256 Register
///
/// Control whether debugger, DMA, core 0 and core 1 can access SHA256, and at what security/privilege levels they can do so.
/// Defaults to Secure, Privileged access only.
/// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
///
/// Address offset: 0x00b8
/// Reset value: 0x000000f8
///
/// ## Fields
///
/// - **DBG** `[7]` (RW)
///
///   If 1, SHA256 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **DMA** `[6]` (RW)
///
///   If 1, SHA256 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE1** `[5]` (RW)
///
///   If 1, SHA256 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE0** `[4]` (RW)
///
///   If 1, SHA256 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **SP** `[3]` (RW)
///
///   If 1, SHA256 can be accessed from a Secure, Privileged context.
///
/// - **SU** `[2]` (RW)
///
///   If 1, and SP is also set, SHA256 can be accessed from a Secure, Unprivileged context.
///
/// - **NSP** `[1]` (RW)
///
///   If 1, SHA256 can be accessed from a Non-secure, Privileged context.
///
/// - **NSU** `[0]` (RW)
///
///   If 1, and NSP is also set, SHA256 can be accessed from a Non-secure, Unprivileged context.
///   This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
#[repr(transparent)]
pub struct Sha256 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Sha256 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sha256 {
    // ======== Field Constants ========

    /// **DBG**
    ///
    /// If 1, SHA256 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const DBG_OFFSET: u32 = 7;
    pub const DBG_MASK: u32 = 0x00000080;
    pub const DBG: u32 = 1 << 7;

    /// **DMA**
    ///
    /// If 1, SHA256 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const DMA_OFFSET: u32 = 6;
    pub const DMA_MASK: u32 = 0x00000040;
    pub const DMA: u32 = 1 << 6;

    /// **CORE1**
    ///
    /// If 1, SHA256 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CORE1_OFFSET: u32 = 5;
    pub const CORE1_MASK: u32 = 0x00000020;
    pub const CORE1: u32 = 1 << 5;

    /// **CORE0**
    ///
    /// If 1, SHA256 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CORE0_OFFSET: u32 = 4;
    pub const CORE0_MASK: u32 = 0x00000010;
    pub const CORE0: u32 = 1 << 4;

    /// **SP**
    ///
    /// If 1, SHA256 can be accessed from a Secure, Privileged context.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SP_OFFSET: u32 = 3;
    pub const SP_MASK: u32 = 0x00000008;
    pub const SP: u32 = 1 << 3;

    /// **SU**
    ///
    /// If 1, and SP is also set, SHA256 can be accessed from a Secure, Unprivileged context.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SU_OFFSET: u32 = 2;
    pub const SU_MASK: u32 = 0x00000004;
    pub const SU: u32 = 1 << 2;

    /// **NSP**
    ///
    /// If 1, SHA256 can be accessed from a Non-secure, Privileged context.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const NSP_OFFSET: u32 = 1;
    pub const NSP_MASK: u32 = 0x00000002;
    pub const NSP: u32 = 1 << 1;

    /// **NSU**
    ///
    /// If 1, and NSP is also set, SHA256 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const NSU_OFFSET: u32 = 0;
    pub const NSU_MASK: u32 = 0x00000001;
    pub const NSU: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DBG** `[7]` (RW)
    ///
    /// If 1, SHA256 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dbg(&self) -> bool {
        (self.value.read_bits() & Self::DBG_MASK) != 0
    }

    /// Set DBG bit atomically (RW)
    #[inline(always)]
    pub fn set_dbg(&self) {
        self.set_bits(Self::DBG_MASK);
    }

    /// Clear DBG bit atomically (RW)
    #[inline(always)]
    pub fn clr_dbg(&self) {
        self.clr_bits(Self::DBG_MASK);
    }

    /// **DMA** `[6]` (RW)
    ///
    /// If 1, SHA256 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dma(&self) -> bool {
        (self.value.read_bits() & Self::DMA_MASK) != 0
    }

    /// Set DMA bit atomically (RW)
    #[inline(always)]
    pub fn set_dma(&self) {
        self.set_bits(Self::DMA_MASK);
    }

    /// Clear DMA bit atomically (RW)
    #[inline(always)]
    pub fn clr_dma(&self) {
        self.clr_bits(Self::DMA_MASK);
    }

    /// **CORE1** `[5]` (RW)
    ///
    /// If 1, SHA256 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core1(&self) -> bool {
        (self.value.read_bits() & Self::CORE1_MASK) != 0
    }

    /// Set CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn set_core1(&self) {
        self.set_bits(Self::CORE1_MASK);
    }

    /// Clear CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core1(&self) {
        self.clr_bits(Self::CORE1_MASK);
    }

    /// **CORE0** `[4]` (RW)
    ///
    /// If 1, SHA256 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core0(&self) -> bool {
        (self.value.read_bits() & Self::CORE0_MASK) != 0
    }

    /// Set CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn set_core0(&self) {
        self.set_bits(Self::CORE0_MASK);
    }

    /// Clear CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core0(&self) {
        self.clr_bits(Self::CORE0_MASK);
    }

    /// **SP** `[3]` (RW)
    ///
    /// If 1, SHA256 can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub fn sp(&self) -> bool {
        (self.value.read_bits() & Self::SP_MASK) != 0
    }

    /// Set SP bit atomically (RW)
    #[inline(always)]
    pub fn set_sp(&self) {
        self.set_bits(Self::SP_MASK);
    }

    /// Clear SP bit atomically (RW)
    #[inline(always)]
    pub fn clr_sp(&self) {
        self.clr_bits(Self::SP_MASK);
    }

    /// **SU** `[2]` (RW)
    ///
    /// If 1, and SP is also set, SHA256 can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub fn su(&self) -> bool {
        (self.value.read_bits() & Self::SU_MASK) != 0
    }

    /// Set SU bit atomically (RW)
    #[inline(always)]
    pub fn set_su(&self) {
        self.set_bits(Self::SU_MASK);
    }

    /// Clear SU bit atomically (RW)
    #[inline(always)]
    pub fn clr_su(&self) {
        self.clr_bits(Self::SU_MASK);
    }

    /// **NSP** `[1]` (RW)
    ///
    /// If 1, SHA256 can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub fn nsp(&self) -> bool {
        (self.value.read_bits() & Self::NSP_MASK) != 0
    }

    /// Set NSP bit atomically (RW)
    #[inline(always)]
    pub fn set_nsp(&self) {
        self.set_bits(Self::NSP_MASK);
    }

    /// Clear NSP bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsp(&self) {
        self.clr_bits(Self::NSP_MASK);
    }

    /// **NSU** `[0]` (RW)
    ///
    /// If 1, and NSP is also set, SHA256 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub fn nsu(&self) -> bool {
        (self.value.read_bits() & Self::NSU_MASK) != 0
    }

    /// Set NSU bit atomically (RW)
    #[inline(always)]
    pub fn set_nsu(&self) {
        self.set_bits(Self::NSU_MASK);
    }

    /// Clear NSU bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsu(&self) {
        self.clr_bits(Self::NSU_MASK);
    }
}

/// SHA256 value type for batch read/modify/write operations
///
/// Use `sha256.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sha256.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Sha256Val {
    bits: u32,
}

impl Sha256Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x000000f8 }
    }

    /// Read **DBG** `[7:7]`
    ///
    /// If 1, SHA256 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        (self.bits & Sha256::DBG_MASK) != 0
    }

    /// Read **DMA** `[6:6]`
    ///
    /// If 1, SHA256 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        (self.bits & Sha256::DMA_MASK) != 0
    }

    /// Read **CORE1** `[5:5]`
    ///
    /// If 1, SHA256 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        (self.bits & Sha256::CORE1_MASK) != 0
    }

    /// Read **CORE0** `[4:4]`
    ///
    /// If 1, SHA256 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        (self.bits & Sha256::CORE0_MASK) != 0
    }

    /// Read **SP** `[3:3]`
    ///
    /// If 1, SHA256 can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        (self.bits & Sha256::SP_MASK) != 0
    }

    /// Read **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, SHA256 can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn su(&self) -> bool {
        (self.bits & Sha256::SU_MASK) != 0
    }

    /// Read **NSP** `[1:1]`
    ///
    /// If 1, SHA256 can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        (self.bits & Sha256::NSP_MASK) != 0
    }

    /// Read **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, SHA256 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        (self.bits & Sha256::NSU_MASK) != 0
    }

    /// Set **DBG** `[7:7]`
    ///
    /// If 1, SHA256 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dbg(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sha256::DBG_MASK
            } else {
                self.bits & !Sha256::DBG_MASK
            }
        }
    }

    /// Set **DMA** `[6:6]`
    ///
    /// If 1, SHA256 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dma(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sha256::DMA_MASK
            } else {
                self.bits & !Sha256::DMA_MASK
            }
        }
    }

    /// Set **CORE1** `[5:5]`
    ///
    /// If 1, SHA256 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sha256::CORE1_MASK
            } else {
                self.bits & !Sha256::CORE1_MASK
            }
        }
    }

    /// Set **CORE0** `[4:4]`
    ///
    /// If 1, SHA256 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sha256::CORE0_MASK
            } else {
                self.bits & !Sha256::CORE0_MASK
            }
        }
    }

    /// Set **SP** `[3:3]`
    ///
    /// If 1, SHA256 can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn with_sp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sha256::SP_MASK
            } else {
                self.bits & !Sha256::SP_MASK
            }
        }
    }

    /// Set **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, SHA256 can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn with_su(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sha256::SU_MASK
            } else {
                self.bits & !Sha256::SU_MASK
            }
        }
    }

    /// Set **NSP** `[1:1]`
    ///
    /// If 1, SHA256 can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn with_nsp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sha256::NSP_MASK
            } else {
                self.bits & !Sha256::NSP_MASK
            }
        }
    }

    /// Set **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, SHA256 can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn with_nsu(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sha256::NSU_MASK
            } else {
                self.bits & !Sha256::NSU_MASK
            }
        }
    }
}

impl Default for Sha256Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Sha256 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Sha256Val {
        Sha256Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Sha256Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Sha256Val) -> Sha256Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SYSCFG Register
///
/// Control whether debugger, DMA, core 0 and core 1 can access SYSCFG, and at what security/privilege levels they can do so.
/// Defaults to Secure, Privileged processor or debug access only.
/// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
///
/// Address offset: 0x00bc
/// Reset value: 0x000000b8
///
/// ## Fields
///
/// - **DBG** `[7]` (RW)
///
///   If 1, SYSCFG can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **DMA** `[6]` (RW)
///
///   If 1, SYSCFG can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE1** `[5]` (RW)
///
///   If 1, SYSCFG can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE0** `[4]` (RW)
///
///   If 1, SYSCFG can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **SP** `[3]` (RW)
///
///   If 1, SYSCFG can be accessed from a Secure, Privileged context.
///
/// - **SU** `[2]` (RW)
///
///   If 1, and SP is also set, SYSCFG can be accessed from a Secure, Unprivileged context.
///
/// - **NSP** `[1]` (RW)
///
///   If 1, SYSCFG can be accessed from a Non-secure, Privileged context.
///
/// - **NSU** `[0]` (RW)
///
///   If 1, and NSP is also set, SYSCFG can be accessed from a Non-secure, Unprivileged context.
///   This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
#[repr(transparent)]
pub struct Syscfg {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Syscfg {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Syscfg {
    // ======== Field Constants ========

    /// **DBG**
    ///
    /// If 1, SYSCFG can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const DBG_OFFSET: u32 = 7;
    pub const DBG_MASK: u32 = 0x00000080;
    pub const DBG: u32 = 1 << 7;

    /// **DMA**
    ///
    /// If 1, SYSCFG can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const DMA_OFFSET: u32 = 6;
    pub const DMA_MASK: u32 = 0x00000040;
    pub const DMA: u32 = 1 << 6;

    /// **CORE1**
    ///
    /// If 1, SYSCFG can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CORE1_OFFSET: u32 = 5;
    pub const CORE1_MASK: u32 = 0x00000020;
    pub const CORE1: u32 = 1 << 5;

    /// **CORE0**
    ///
    /// If 1, SYSCFG can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CORE0_OFFSET: u32 = 4;
    pub const CORE0_MASK: u32 = 0x00000010;
    pub const CORE0: u32 = 1 << 4;

    /// **SP**
    ///
    /// If 1, SYSCFG can be accessed from a Secure, Privileged context.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SP_OFFSET: u32 = 3;
    pub const SP_MASK: u32 = 0x00000008;
    pub const SP: u32 = 1 << 3;

    /// **SU**
    ///
    /// If 1, and SP is also set, SYSCFG can be accessed from a Secure, Unprivileged context.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SU_OFFSET: u32 = 2;
    pub const SU_MASK: u32 = 0x00000004;
    pub const SU: u32 = 1 << 2;

    /// **NSP**
    ///
    /// If 1, SYSCFG can be accessed from a Non-secure, Privileged context.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const NSP_OFFSET: u32 = 1;
    pub const NSP_MASK: u32 = 0x00000002;
    pub const NSP: u32 = 1 << 1;

    /// **NSU**
    ///
    /// If 1, and NSP is also set, SYSCFG can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const NSU_OFFSET: u32 = 0;
    pub const NSU_MASK: u32 = 0x00000001;
    pub const NSU: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DBG** `[7]` (RW)
    ///
    /// If 1, SYSCFG can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dbg(&self) -> bool {
        (self.value.read_bits() & Self::DBG_MASK) != 0
    }

    /// Set DBG bit atomically (RW)
    #[inline(always)]
    pub fn set_dbg(&self) {
        self.set_bits(Self::DBG_MASK);
    }

    /// Clear DBG bit atomically (RW)
    #[inline(always)]
    pub fn clr_dbg(&self) {
        self.clr_bits(Self::DBG_MASK);
    }

    /// **DMA** `[6]` (RW)
    ///
    /// If 1, SYSCFG can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dma(&self) -> bool {
        (self.value.read_bits() & Self::DMA_MASK) != 0
    }

    /// Set DMA bit atomically (RW)
    #[inline(always)]
    pub fn set_dma(&self) {
        self.set_bits(Self::DMA_MASK);
    }

    /// Clear DMA bit atomically (RW)
    #[inline(always)]
    pub fn clr_dma(&self) {
        self.clr_bits(Self::DMA_MASK);
    }

    /// **CORE1** `[5]` (RW)
    ///
    /// If 1, SYSCFG can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core1(&self) -> bool {
        (self.value.read_bits() & Self::CORE1_MASK) != 0
    }

    /// Set CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn set_core1(&self) {
        self.set_bits(Self::CORE1_MASK);
    }

    /// Clear CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core1(&self) {
        self.clr_bits(Self::CORE1_MASK);
    }

    /// **CORE0** `[4]` (RW)
    ///
    /// If 1, SYSCFG can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core0(&self) -> bool {
        (self.value.read_bits() & Self::CORE0_MASK) != 0
    }

    /// Set CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn set_core0(&self) {
        self.set_bits(Self::CORE0_MASK);
    }

    /// Clear CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core0(&self) {
        self.clr_bits(Self::CORE0_MASK);
    }

    /// **SP** `[3]` (RW)
    ///
    /// If 1, SYSCFG can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub fn sp(&self) -> bool {
        (self.value.read_bits() & Self::SP_MASK) != 0
    }

    /// Set SP bit atomically (RW)
    #[inline(always)]
    pub fn set_sp(&self) {
        self.set_bits(Self::SP_MASK);
    }

    /// Clear SP bit atomically (RW)
    #[inline(always)]
    pub fn clr_sp(&self) {
        self.clr_bits(Self::SP_MASK);
    }

    /// **SU** `[2]` (RW)
    ///
    /// If 1, and SP is also set, SYSCFG can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub fn su(&self) -> bool {
        (self.value.read_bits() & Self::SU_MASK) != 0
    }

    /// Set SU bit atomically (RW)
    #[inline(always)]
    pub fn set_su(&self) {
        self.set_bits(Self::SU_MASK);
    }

    /// Clear SU bit atomically (RW)
    #[inline(always)]
    pub fn clr_su(&self) {
        self.clr_bits(Self::SU_MASK);
    }

    /// **NSP** `[1]` (RW)
    ///
    /// If 1, SYSCFG can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub fn nsp(&self) -> bool {
        (self.value.read_bits() & Self::NSP_MASK) != 0
    }

    /// Set NSP bit atomically (RW)
    #[inline(always)]
    pub fn set_nsp(&self) {
        self.set_bits(Self::NSP_MASK);
    }

    /// Clear NSP bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsp(&self) {
        self.clr_bits(Self::NSP_MASK);
    }

    /// **NSU** `[0]` (RW)
    ///
    /// If 1, and NSP is also set, SYSCFG can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub fn nsu(&self) -> bool {
        (self.value.read_bits() & Self::NSU_MASK) != 0
    }

    /// Set NSU bit atomically (RW)
    #[inline(always)]
    pub fn set_nsu(&self) {
        self.set_bits(Self::NSU_MASK);
    }

    /// Clear NSU bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsu(&self) {
        self.clr_bits(Self::NSU_MASK);
    }
}

/// SYSCFG value type for batch read/modify/write operations
///
/// Use `syscfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `syscfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SyscfgVal {
    bits: u32,
}

impl SyscfgVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x000000b8 }
    }

    /// Read **DBG** `[7:7]`
    ///
    /// If 1, SYSCFG can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        (self.bits & Syscfg::DBG_MASK) != 0
    }

    /// Read **DMA** `[6:6]`
    ///
    /// If 1, SYSCFG can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        (self.bits & Syscfg::DMA_MASK) != 0
    }

    /// Read **CORE1** `[5:5]`
    ///
    /// If 1, SYSCFG can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        (self.bits & Syscfg::CORE1_MASK) != 0
    }

    /// Read **CORE0** `[4:4]`
    ///
    /// If 1, SYSCFG can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        (self.bits & Syscfg::CORE0_MASK) != 0
    }

    /// Read **SP** `[3:3]`
    ///
    /// If 1, SYSCFG can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        (self.bits & Syscfg::SP_MASK) != 0
    }

    /// Read **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, SYSCFG can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn su(&self) -> bool {
        (self.bits & Syscfg::SU_MASK) != 0
    }

    /// Read **NSP** `[1:1]`
    ///
    /// If 1, SYSCFG can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        (self.bits & Syscfg::NSP_MASK) != 0
    }

    /// Read **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, SYSCFG can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        (self.bits & Syscfg::NSU_MASK) != 0
    }

    /// Set **DBG** `[7:7]`
    ///
    /// If 1, SYSCFG can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dbg(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Syscfg::DBG_MASK
            } else {
                self.bits & !Syscfg::DBG_MASK
            }
        }
    }

    /// Set **DMA** `[6:6]`
    ///
    /// If 1, SYSCFG can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dma(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Syscfg::DMA_MASK
            } else {
                self.bits & !Syscfg::DMA_MASK
            }
        }
    }

    /// Set **CORE1** `[5:5]`
    ///
    /// If 1, SYSCFG can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Syscfg::CORE1_MASK
            } else {
                self.bits & !Syscfg::CORE1_MASK
            }
        }
    }

    /// Set **CORE0** `[4:4]`
    ///
    /// If 1, SYSCFG can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Syscfg::CORE0_MASK
            } else {
                self.bits & !Syscfg::CORE0_MASK
            }
        }
    }

    /// Set **SP** `[3:3]`
    ///
    /// If 1, SYSCFG can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn with_sp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Syscfg::SP_MASK
            } else {
                self.bits & !Syscfg::SP_MASK
            }
        }
    }

    /// Set **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, SYSCFG can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn with_su(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Syscfg::SU_MASK
            } else {
                self.bits & !Syscfg::SU_MASK
            }
        }
    }

    /// Set **NSP** `[1:1]`
    ///
    /// If 1, SYSCFG can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn with_nsp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Syscfg::NSP_MASK
            } else {
                self.bits & !Syscfg::NSP_MASK
            }
        }
    }

    /// Set **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, SYSCFG can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn with_nsu(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Syscfg::NSU_MASK
            } else {
                self.bits & !Syscfg::NSU_MASK
            }
        }
    }
}

impl Default for SyscfgVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Syscfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SyscfgVal {
        SyscfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SyscfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SyscfgVal) -> SyscfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CLOCKS Register
///
/// Control whether debugger, DMA, core 0 and core 1 can access CLOCKS, and at what security/privilege levels they can do so.
/// Defaults to Secure, Privileged processor or debug access only.
/// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
///
/// Address offset: 0x00c0
/// Reset value: 0x000000b8
///
/// ## Fields
///
/// - **DBG** `[7]` (RW)
///
///   If 1, CLOCKS can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **DMA** `[6]` (RW)
///
///   If 1, CLOCKS can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE1** `[5]` (RW)
///
///   If 1, CLOCKS can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE0** `[4]` (RW)
///
///   If 1, CLOCKS can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **SP** `[3]` (RW)
///
///   If 1, CLOCKS can be accessed from a Secure, Privileged context.
///
/// - **SU** `[2]` (RW)
///
///   If 1, and SP is also set, CLOCKS can be accessed from a Secure, Unprivileged context.
///
/// - **NSP** `[1]` (RW)
///
///   If 1, CLOCKS can be accessed from a Non-secure, Privileged context.
///
/// - **NSU** `[0]` (RW)
///
///   If 1, and NSP is also set, CLOCKS can be accessed from a Non-secure, Unprivileged context.
///   This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
#[repr(transparent)]
pub struct Clocks {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Clocks {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Clocks {
    // ======== Field Constants ========

    /// **DBG**
    ///
    /// If 1, CLOCKS can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const DBG_OFFSET: u32 = 7;
    pub const DBG_MASK: u32 = 0x00000080;
    pub const DBG: u32 = 1 << 7;

    /// **DMA**
    ///
    /// If 1, CLOCKS can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const DMA_OFFSET: u32 = 6;
    pub const DMA_MASK: u32 = 0x00000040;
    pub const DMA: u32 = 1 << 6;

    /// **CORE1**
    ///
    /// If 1, CLOCKS can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CORE1_OFFSET: u32 = 5;
    pub const CORE1_MASK: u32 = 0x00000020;
    pub const CORE1: u32 = 1 << 5;

    /// **CORE0**
    ///
    /// If 1, CLOCKS can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CORE0_OFFSET: u32 = 4;
    pub const CORE0_MASK: u32 = 0x00000010;
    pub const CORE0: u32 = 1 << 4;

    /// **SP**
    ///
    /// If 1, CLOCKS can be accessed from a Secure, Privileged context.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SP_OFFSET: u32 = 3;
    pub const SP_MASK: u32 = 0x00000008;
    pub const SP: u32 = 1 << 3;

    /// **SU**
    ///
    /// If 1, and SP is also set, CLOCKS can be accessed from a Secure, Unprivileged context.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SU_OFFSET: u32 = 2;
    pub const SU_MASK: u32 = 0x00000004;
    pub const SU: u32 = 1 << 2;

    /// **NSP**
    ///
    /// If 1, CLOCKS can be accessed from a Non-secure, Privileged context.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const NSP_OFFSET: u32 = 1;
    pub const NSP_MASK: u32 = 0x00000002;
    pub const NSP: u32 = 1 << 1;

    /// **NSU**
    ///
    /// If 1, and NSP is also set, CLOCKS can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const NSU_OFFSET: u32 = 0;
    pub const NSU_MASK: u32 = 0x00000001;
    pub const NSU: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DBG** `[7]` (RW)
    ///
    /// If 1, CLOCKS can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dbg(&self) -> bool {
        (self.value.read_bits() & Self::DBG_MASK) != 0
    }

    /// Set DBG bit atomically (RW)
    #[inline(always)]
    pub fn set_dbg(&self) {
        self.set_bits(Self::DBG_MASK);
    }

    /// Clear DBG bit atomically (RW)
    #[inline(always)]
    pub fn clr_dbg(&self) {
        self.clr_bits(Self::DBG_MASK);
    }

    /// **DMA** `[6]` (RW)
    ///
    /// If 1, CLOCKS can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dma(&self) -> bool {
        (self.value.read_bits() & Self::DMA_MASK) != 0
    }

    /// Set DMA bit atomically (RW)
    #[inline(always)]
    pub fn set_dma(&self) {
        self.set_bits(Self::DMA_MASK);
    }

    /// Clear DMA bit atomically (RW)
    #[inline(always)]
    pub fn clr_dma(&self) {
        self.clr_bits(Self::DMA_MASK);
    }

    /// **CORE1** `[5]` (RW)
    ///
    /// If 1, CLOCKS can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core1(&self) -> bool {
        (self.value.read_bits() & Self::CORE1_MASK) != 0
    }

    /// Set CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn set_core1(&self) {
        self.set_bits(Self::CORE1_MASK);
    }

    /// Clear CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core1(&self) {
        self.clr_bits(Self::CORE1_MASK);
    }

    /// **CORE0** `[4]` (RW)
    ///
    /// If 1, CLOCKS can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core0(&self) -> bool {
        (self.value.read_bits() & Self::CORE0_MASK) != 0
    }

    /// Set CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn set_core0(&self) {
        self.set_bits(Self::CORE0_MASK);
    }

    /// Clear CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core0(&self) {
        self.clr_bits(Self::CORE0_MASK);
    }

    /// **SP** `[3]` (RW)
    ///
    /// If 1, CLOCKS can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub fn sp(&self) -> bool {
        (self.value.read_bits() & Self::SP_MASK) != 0
    }

    /// Set SP bit atomically (RW)
    #[inline(always)]
    pub fn set_sp(&self) {
        self.set_bits(Self::SP_MASK);
    }

    /// Clear SP bit atomically (RW)
    #[inline(always)]
    pub fn clr_sp(&self) {
        self.clr_bits(Self::SP_MASK);
    }

    /// **SU** `[2]` (RW)
    ///
    /// If 1, and SP is also set, CLOCKS can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub fn su(&self) -> bool {
        (self.value.read_bits() & Self::SU_MASK) != 0
    }

    /// Set SU bit atomically (RW)
    #[inline(always)]
    pub fn set_su(&self) {
        self.set_bits(Self::SU_MASK);
    }

    /// Clear SU bit atomically (RW)
    #[inline(always)]
    pub fn clr_su(&self) {
        self.clr_bits(Self::SU_MASK);
    }

    /// **NSP** `[1]` (RW)
    ///
    /// If 1, CLOCKS can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub fn nsp(&self) -> bool {
        (self.value.read_bits() & Self::NSP_MASK) != 0
    }

    /// Set NSP bit atomically (RW)
    #[inline(always)]
    pub fn set_nsp(&self) {
        self.set_bits(Self::NSP_MASK);
    }

    /// Clear NSP bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsp(&self) {
        self.clr_bits(Self::NSP_MASK);
    }

    /// **NSU** `[0]` (RW)
    ///
    /// If 1, and NSP is also set, CLOCKS can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub fn nsu(&self) -> bool {
        (self.value.read_bits() & Self::NSU_MASK) != 0
    }

    /// Set NSU bit atomically (RW)
    #[inline(always)]
    pub fn set_nsu(&self) {
        self.set_bits(Self::NSU_MASK);
    }

    /// Clear NSU bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsu(&self) {
        self.clr_bits(Self::NSU_MASK);
    }
}

/// CLOCKS value type for batch read/modify/write operations
///
/// Use `clocks.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `clocks.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ClocksVal {
    bits: u32,
}

impl ClocksVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x000000b8 }
    }

    /// Read **DBG** `[7:7]`
    ///
    /// If 1, CLOCKS can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        (self.bits & Clocks::DBG_MASK) != 0
    }

    /// Read **DMA** `[6:6]`
    ///
    /// If 1, CLOCKS can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        (self.bits & Clocks::DMA_MASK) != 0
    }

    /// Read **CORE1** `[5:5]`
    ///
    /// If 1, CLOCKS can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        (self.bits & Clocks::CORE1_MASK) != 0
    }

    /// Read **CORE0** `[4:4]`
    ///
    /// If 1, CLOCKS can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        (self.bits & Clocks::CORE0_MASK) != 0
    }

    /// Read **SP** `[3:3]`
    ///
    /// If 1, CLOCKS can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        (self.bits & Clocks::SP_MASK) != 0
    }

    /// Read **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, CLOCKS can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn su(&self) -> bool {
        (self.bits & Clocks::SU_MASK) != 0
    }

    /// Read **NSP** `[1:1]`
    ///
    /// If 1, CLOCKS can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        (self.bits & Clocks::NSP_MASK) != 0
    }

    /// Read **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, CLOCKS can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        (self.bits & Clocks::NSU_MASK) != 0
    }

    /// Set **DBG** `[7:7]`
    ///
    /// If 1, CLOCKS can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dbg(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Clocks::DBG_MASK
            } else {
                self.bits & !Clocks::DBG_MASK
            }
        }
    }

    /// Set **DMA** `[6:6]`
    ///
    /// If 1, CLOCKS can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dma(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Clocks::DMA_MASK
            } else {
                self.bits & !Clocks::DMA_MASK
            }
        }
    }

    /// Set **CORE1** `[5:5]`
    ///
    /// If 1, CLOCKS can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Clocks::CORE1_MASK
            } else {
                self.bits & !Clocks::CORE1_MASK
            }
        }
    }

    /// Set **CORE0** `[4:4]`
    ///
    /// If 1, CLOCKS can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Clocks::CORE0_MASK
            } else {
                self.bits & !Clocks::CORE0_MASK
            }
        }
    }

    /// Set **SP** `[3:3]`
    ///
    /// If 1, CLOCKS can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn with_sp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Clocks::SP_MASK
            } else {
                self.bits & !Clocks::SP_MASK
            }
        }
    }

    /// Set **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, CLOCKS can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn with_su(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Clocks::SU_MASK
            } else {
                self.bits & !Clocks::SU_MASK
            }
        }
    }

    /// Set **NSP** `[1:1]`
    ///
    /// If 1, CLOCKS can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn with_nsp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Clocks::NSP_MASK
            } else {
                self.bits & !Clocks::NSP_MASK
            }
        }
    }

    /// Set **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, CLOCKS can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn with_nsu(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Clocks::NSU_MASK
            } else {
                self.bits & !Clocks::NSU_MASK
            }
        }
    }
}

impl Default for ClocksVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Clocks {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ClocksVal {
        ClocksVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ClocksVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ClocksVal) -> ClocksVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// XOSC Register
///
/// Control whether debugger, DMA, core 0 and core 1 can access XOSC, and at what security/privilege levels they can do so.
/// Defaults to Secure, Privileged processor or debug access only.
/// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
///
/// Address offset: 0x00c4
/// Reset value: 0x000000b8
///
/// ## Fields
///
/// - **DBG** `[7]` (RW)
///
///   If 1, XOSC can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **DMA** `[6]` (RW)
///
///   If 1, XOSC can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE1** `[5]` (RW)
///
///   If 1, XOSC can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE0** `[4]` (RW)
///
///   If 1, XOSC can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **SP** `[3]` (RW)
///
///   If 1, XOSC can be accessed from a Secure, Privileged context.
///
/// - **SU** `[2]` (RW)
///
///   If 1, and SP is also set, XOSC can be accessed from a Secure, Unprivileged context.
///
/// - **NSP** `[1]` (RW)
///
///   If 1, XOSC can be accessed from a Non-secure, Privileged context.
///
/// - **NSU** `[0]` (RW)
///
///   If 1, and NSP is also set, XOSC can be accessed from a Non-secure, Unprivileged context.
///   This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
#[repr(transparent)]
pub struct Xosc {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Xosc {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Xosc {
    // ======== Field Constants ========

    /// **DBG**
    ///
    /// If 1, XOSC can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const DBG_OFFSET: u32 = 7;
    pub const DBG_MASK: u32 = 0x00000080;
    pub const DBG: u32 = 1 << 7;

    /// **DMA**
    ///
    /// If 1, XOSC can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const DMA_OFFSET: u32 = 6;
    pub const DMA_MASK: u32 = 0x00000040;
    pub const DMA: u32 = 1 << 6;

    /// **CORE1**
    ///
    /// If 1, XOSC can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CORE1_OFFSET: u32 = 5;
    pub const CORE1_MASK: u32 = 0x00000020;
    pub const CORE1: u32 = 1 << 5;

    /// **CORE0**
    ///
    /// If 1, XOSC can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CORE0_OFFSET: u32 = 4;
    pub const CORE0_MASK: u32 = 0x00000010;
    pub const CORE0: u32 = 1 << 4;

    /// **SP**
    ///
    /// If 1, XOSC can be accessed from a Secure, Privileged context.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SP_OFFSET: u32 = 3;
    pub const SP_MASK: u32 = 0x00000008;
    pub const SP: u32 = 1 << 3;

    /// **SU**
    ///
    /// If 1, and SP is also set, XOSC can be accessed from a Secure, Unprivileged context.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SU_OFFSET: u32 = 2;
    pub const SU_MASK: u32 = 0x00000004;
    pub const SU: u32 = 1 << 2;

    /// **NSP**
    ///
    /// If 1, XOSC can be accessed from a Non-secure, Privileged context.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const NSP_OFFSET: u32 = 1;
    pub const NSP_MASK: u32 = 0x00000002;
    pub const NSP: u32 = 1 << 1;

    /// **NSU**
    ///
    /// If 1, and NSP is also set, XOSC can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const NSU_OFFSET: u32 = 0;
    pub const NSU_MASK: u32 = 0x00000001;
    pub const NSU: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DBG** `[7]` (RW)
    ///
    /// If 1, XOSC can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dbg(&self) -> bool {
        (self.value.read_bits() & Self::DBG_MASK) != 0
    }

    /// Set DBG bit atomically (RW)
    #[inline(always)]
    pub fn set_dbg(&self) {
        self.set_bits(Self::DBG_MASK);
    }

    /// Clear DBG bit atomically (RW)
    #[inline(always)]
    pub fn clr_dbg(&self) {
        self.clr_bits(Self::DBG_MASK);
    }

    /// **DMA** `[6]` (RW)
    ///
    /// If 1, XOSC can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dma(&self) -> bool {
        (self.value.read_bits() & Self::DMA_MASK) != 0
    }

    /// Set DMA bit atomically (RW)
    #[inline(always)]
    pub fn set_dma(&self) {
        self.set_bits(Self::DMA_MASK);
    }

    /// Clear DMA bit atomically (RW)
    #[inline(always)]
    pub fn clr_dma(&self) {
        self.clr_bits(Self::DMA_MASK);
    }

    /// **CORE1** `[5]` (RW)
    ///
    /// If 1, XOSC can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core1(&self) -> bool {
        (self.value.read_bits() & Self::CORE1_MASK) != 0
    }

    /// Set CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn set_core1(&self) {
        self.set_bits(Self::CORE1_MASK);
    }

    /// Clear CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core1(&self) {
        self.clr_bits(Self::CORE1_MASK);
    }

    /// **CORE0** `[4]` (RW)
    ///
    /// If 1, XOSC can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core0(&self) -> bool {
        (self.value.read_bits() & Self::CORE0_MASK) != 0
    }

    /// Set CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn set_core0(&self) {
        self.set_bits(Self::CORE0_MASK);
    }

    /// Clear CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core0(&self) {
        self.clr_bits(Self::CORE0_MASK);
    }

    /// **SP** `[3]` (RW)
    ///
    /// If 1, XOSC can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub fn sp(&self) -> bool {
        (self.value.read_bits() & Self::SP_MASK) != 0
    }

    /// Set SP bit atomically (RW)
    #[inline(always)]
    pub fn set_sp(&self) {
        self.set_bits(Self::SP_MASK);
    }

    /// Clear SP bit atomically (RW)
    #[inline(always)]
    pub fn clr_sp(&self) {
        self.clr_bits(Self::SP_MASK);
    }

    /// **SU** `[2]` (RW)
    ///
    /// If 1, and SP is also set, XOSC can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub fn su(&self) -> bool {
        (self.value.read_bits() & Self::SU_MASK) != 0
    }

    /// Set SU bit atomically (RW)
    #[inline(always)]
    pub fn set_su(&self) {
        self.set_bits(Self::SU_MASK);
    }

    /// Clear SU bit atomically (RW)
    #[inline(always)]
    pub fn clr_su(&self) {
        self.clr_bits(Self::SU_MASK);
    }

    /// **NSP** `[1]` (RW)
    ///
    /// If 1, XOSC can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub fn nsp(&self) -> bool {
        (self.value.read_bits() & Self::NSP_MASK) != 0
    }

    /// Set NSP bit atomically (RW)
    #[inline(always)]
    pub fn set_nsp(&self) {
        self.set_bits(Self::NSP_MASK);
    }

    /// Clear NSP bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsp(&self) {
        self.clr_bits(Self::NSP_MASK);
    }

    /// **NSU** `[0]` (RW)
    ///
    /// If 1, and NSP is also set, XOSC can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub fn nsu(&self) -> bool {
        (self.value.read_bits() & Self::NSU_MASK) != 0
    }

    /// Set NSU bit atomically (RW)
    #[inline(always)]
    pub fn set_nsu(&self) {
        self.set_bits(Self::NSU_MASK);
    }

    /// Clear NSU bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsu(&self) {
        self.clr_bits(Self::NSU_MASK);
    }
}

/// XOSC value type for batch read/modify/write operations
///
/// Use `xosc.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `xosc.write_val(val)`.
#[derive(Clone, Copy)]
pub struct XoscVal {
    bits: u32,
}

impl XoscVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x000000b8 }
    }

    /// Read **DBG** `[7:7]`
    ///
    /// If 1, XOSC can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        (self.bits & Xosc::DBG_MASK) != 0
    }

    /// Read **DMA** `[6:6]`
    ///
    /// If 1, XOSC can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        (self.bits & Xosc::DMA_MASK) != 0
    }

    /// Read **CORE1** `[5:5]`
    ///
    /// If 1, XOSC can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        (self.bits & Xosc::CORE1_MASK) != 0
    }

    /// Read **CORE0** `[4:4]`
    ///
    /// If 1, XOSC can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        (self.bits & Xosc::CORE0_MASK) != 0
    }

    /// Read **SP** `[3:3]`
    ///
    /// If 1, XOSC can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        (self.bits & Xosc::SP_MASK) != 0
    }

    /// Read **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, XOSC can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn su(&self) -> bool {
        (self.bits & Xosc::SU_MASK) != 0
    }

    /// Read **NSP** `[1:1]`
    ///
    /// If 1, XOSC can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        (self.bits & Xosc::NSP_MASK) != 0
    }

    /// Read **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, XOSC can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        (self.bits & Xosc::NSU_MASK) != 0
    }

    /// Set **DBG** `[7:7]`
    ///
    /// If 1, XOSC can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dbg(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Xosc::DBG_MASK
            } else {
                self.bits & !Xosc::DBG_MASK
            }
        }
    }

    /// Set **DMA** `[6:6]`
    ///
    /// If 1, XOSC can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dma(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Xosc::DMA_MASK
            } else {
                self.bits & !Xosc::DMA_MASK
            }
        }
    }

    /// Set **CORE1** `[5:5]`
    ///
    /// If 1, XOSC can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Xosc::CORE1_MASK
            } else {
                self.bits & !Xosc::CORE1_MASK
            }
        }
    }

    /// Set **CORE0** `[4:4]`
    ///
    /// If 1, XOSC can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Xosc::CORE0_MASK
            } else {
                self.bits & !Xosc::CORE0_MASK
            }
        }
    }

    /// Set **SP** `[3:3]`
    ///
    /// If 1, XOSC can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn with_sp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Xosc::SP_MASK
            } else {
                self.bits & !Xosc::SP_MASK
            }
        }
    }

    /// Set **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, XOSC can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn with_su(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Xosc::SU_MASK
            } else {
                self.bits & !Xosc::SU_MASK
            }
        }
    }

    /// Set **NSP** `[1:1]`
    ///
    /// If 1, XOSC can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn with_nsp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Xosc::NSP_MASK
            } else {
                self.bits & !Xosc::NSP_MASK
            }
        }
    }

    /// Set **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, XOSC can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn with_nsu(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Xosc::NSU_MASK
            } else {
                self.bits & !Xosc::NSU_MASK
            }
        }
    }
}

impl Default for XoscVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Xosc {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> XoscVal {
        XoscVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: XoscVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(XoscVal) -> XoscVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ROSC Register
///
/// Control whether debugger, DMA, core 0 and core 1 can access ROSC, and at what security/privilege levels they can do so.
/// Defaults to Secure, Privileged processor or debug access only.
/// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
///
/// Address offset: 0x00c8
/// Reset value: 0x000000b8
///
/// ## Fields
///
/// - **DBG** `[7]` (RW)
///
///   If 1, ROSC can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **DMA** `[6]` (RW)
///
///   If 1, ROSC can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE1** `[5]` (RW)
///
///   If 1, ROSC can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE0** `[4]` (RW)
///
///   If 1, ROSC can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **SP** `[3]` (RW)
///
///   If 1, ROSC can be accessed from a Secure, Privileged context.
///
/// - **SU** `[2]` (RW)
///
///   If 1, and SP is also set, ROSC can be accessed from a Secure, Unprivileged context.
///
/// - **NSP** `[1]` (RW)
///
///   If 1, ROSC can be accessed from a Non-secure, Privileged context.
///
/// - **NSU** `[0]` (RW)
///
///   If 1, and NSP is also set, ROSC can be accessed from a Non-secure, Unprivileged context.
///   This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
#[repr(transparent)]
pub struct Rosc {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Rosc {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Rosc {
    // ======== Field Constants ========

    /// **DBG**
    ///
    /// If 1, ROSC can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const DBG_OFFSET: u32 = 7;
    pub const DBG_MASK: u32 = 0x00000080;
    pub const DBG: u32 = 1 << 7;

    /// **DMA**
    ///
    /// If 1, ROSC can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const DMA_OFFSET: u32 = 6;
    pub const DMA_MASK: u32 = 0x00000040;
    pub const DMA: u32 = 1 << 6;

    /// **CORE1**
    ///
    /// If 1, ROSC can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CORE1_OFFSET: u32 = 5;
    pub const CORE1_MASK: u32 = 0x00000020;
    pub const CORE1: u32 = 1 << 5;

    /// **CORE0**
    ///
    /// If 1, ROSC can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CORE0_OFFSET: u32 = 4;
    pub const CORE0_MASK: u32 = 0x00000010;
    pub const CORE0: u32 = 1 << 4;

    /// **SP**
    ///
    /// If 1, ROSC can be accessed from a Secure, Privileged context.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SP_OFFSET: u32 = 3;
    pub const SP_MASK: u32 = 0x00000008;
    pub const SP: u32 = 1 << 3;

    /// **SU**
    ///
    /// If 1, and SP is also set, ROSC can be accessed from a Secure, Unprivileged context.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SU_OFFSET: u32 = 2;
    pub const SU_MASK: u32 = 0x00000004;
    pub const SU: u32 = 1 << 2;

    /// **NSP**
    ///
    /// If 1, ROSC can be accessed from a Non-secure, Privileged context.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const NSP_OFFSET: u32 = 1;
    pub const NSP_MASK: u32 = 0x00000002;
    pub const NSP: u32 = 1 << 1;

    /// **NSU**
    ///
    /// If 1, and NSP is also set, ROSC can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const NSU_OFFSET: u32 = 0;
    pub const NSU_MASK: u32 = 0x00000001;
    pub const NSU: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DBG** `[7]` (RW)
    ///
    /// If 1, ROSC can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dbg(&self) -> bool {
        (self.value.read_bits() & Self::DBG_MASK) != 0
    }

    /// Set DBG bit atomically (RW)
    #[inline(always)]
    pub fn set_dbg(&self) {
        self.set_bits(Self::DBG_MASK);
    }

    /// Clear DBG bit atomically (RW)
    #[inline(always)]
    pub fn clr_dbg(&self) {
        self.clr_bits(Self::DBG_MASK);
    }

    /// **DMA** `[6]` (RW)
    ///
    /// If 1, ROSC can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dma(&self) -> bool {
        (self.value.read_bits() & Self::DMA_MASK) != 0
    }

    /// Set DMA bit atomically (RW)
    #[inline(always)]
    pub fn set_dma(&self) {
        self.set_bits(Self::DMA_MASK);
    }

    /// Clear DMA bit atomically (RW)
    #[inline(always)]
    pub fn clr_dma(&self) {
        self.clr_bits(Self::DMA_MASK);
    }

    /// **CORE1** `[5]` (RW)
    ///
    /// If 1, ROSC can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core1(&self) -> bool {
        (self.value.read_bits() & Self::CORE1_MASK) != 0
    }

    /// Set CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn set_core1(&self) {
        self.set_bits(Self::CORE1_MASK);
    }

    /// Clear CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core1(&self) {
        self.clr_bits(Self::CORE1_MASK);
    }

    /// **CORE0** `[4]` (RW)
    ///
    /// If 1, ROSC can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core0(&self) -> bool {
        (self.value.read_bits() & Self::CORE0_MASK) != 0
    }

    /// Set CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn set_core0(&self) {
        self.set_bits(Self::CORE0_MASK);
    }

    /// Clear CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core0(&self) {
        self.clr_bits(Self::CORE0_MASK);
    }

    /// **SP** `[3]` (RW)
    ///
    /// If 1, ROSC can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub fn sp(&self) -> bool {
        (self.value.read_bits() & Self::SP_MASK) != 0
    }

    /// Set SP bit atomically (RW)
    #[inline(always)]
    pub fn set_sp(&self) {
        self.set_bits(Self::SP_MASK);
    }

    /// Clear SP bit atomically (RW)
    #[inline(always)]
    pub fn clr_sp(&self) {
        self.clr_bits(Self::SP_MASK);
    }

    /// **SU** `[2]` (RW)
    ///
    /// If 1, and SP is also set, ROSC can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub fn su(&self) -> bool {
        (self.value.read_bits() & Self::SU_MASK) != 0
    }

    /// Set SU bit atomically (RW)
    #[inline(always)]
    pub fn set_su(&self) {
        self.set_bits(Self::SU_MASK);
    }

    /// Clear SU bit atomically (RW)
    #[inline(always)]
    pub fn clr_su(&self) {
        self.clr_bits(Self::SU_MASK);
    }

    /// **NSP** `[1]` (RW)
    ///
    /// If 1, ROSC can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub fn nsp(&self) -> bool {
        (self.value.read_bits() & Self::NSP_MASK) != 0
    }

    /// Set NSP bit atomically (RW)
    #[inline(always)]
    pub fn set_nsp(&self) {
        self.set_bits(Self::NSP_MASK);
    }

    /// Clear NSP bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsp(&self) {
        self.clr_bits(Self::NSP_MASK);
    }

    /// **NSU** `[0]` (RW)
    ///
    /// If 1, and NSP is also set, ROSC can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub fn nsu(&self) -> bool {
        (self.value.read_bits() & Self::NSU_MASK) != 0
    }

    /// Set NSU bit atomically (RW)
    #[inline(always)]
    pub fn set_nsu(&self) {
        self.set_bits(Self::NSU_MASK);
    }

    /// Clear NSU bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsu(&self) {
        self.clr_bits(Self::NSU_MASK);
    }
}

/// ROSC value type for batch read/modify/write operations
///
/// Use `rosc.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rosc.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RoscVal {
    bits: u32,
}

impl RoscVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x000000b8 }
    }

    /// Read **DBG** `[7:7]`
    ///
    /// If 1, ROSC can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        (self.bits & Rosc::DBG_MASK) != 0
    }

    /// Read **DMA** `[6:6]`
    ///
    /// If 1, ROSC can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        (self.bits & Rosc::DMA_MASK) != 0
    }

    /// Read **CORE1** `[5:5]`
    ///
    /// If 1, ROSC can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        (self.bits & Rosc::CORE1_MASK) != 0
    }

    /// Read **CORE0** `[4:4]`
    ///
    /// If 1, ROSC can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        (self.bits & Rosc::CORE0_MASK) != 0
    }

    /// Read **SP** `[3:3]`
    ///
    /// If 1, ROSC can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        (self.bits & Rosc::SP_MASK) != 0
    }

    /// Read **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, ROSC can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn su(&self) -> bool {
        (self.bits & Rosc::SU_MASK) != 0
    }

    /// Read **NSP** `[1:1]`
    ///
    /// If 1, ROSC can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        (self.bits & Rosc::NSP_MASK) != 0
    }

    /// Read **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, ROSC can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        (self.bits & Rosc::NSU_MASK) != 0
    }

    /// Set **DBG** `[7:7]`
    ///
    /// If 1, ROSC can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dbg(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Rosc::DBG_MASK
            } else {
                self.bits & !Rosc::DBG_MASK
            }
        }
    }

    /// Set **DMA** `[6:6]`
    ///
    /// If 1, ROSC can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dma(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Rosc::DMA_MASK
            } else {
                self.bits & !Rosc::DMA_MASK
            }
        }
    }

    /// Set **CORE1** `[5:5]`
    ///
    /// If 1, ROSC can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Rosc::CORE1_MASK
            } else {
                self.bits & !Rosc::CORE1_MASK
            }
        }
    }

    /// Set **CORE0** `[4:4]`
    ///
    /// If 1, ROSC can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Rosc::CORE0_MASK
            } else {
                self.bits & !Rosc::CORE0_MASK
            }
        }
    }

    /// Set **SP** `[3:3]`
    ///
    /// If 1, ROSC can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn with_sp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Rosc::SP_MASK
            } else {
                self.bits & !Rosc::SP_MASK
            }
        }
    }

    /// Set **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, ROSC can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn with_su(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Rosc::SU_MASK
            } else {
                self.bits & !Rosc::SU_MASK
            }
        }
    }

    /// Set **NSP** `[1:1]`
    ///
    /// If 1, ROSC can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn with_nsp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Rosc::NSP_MASK
            } else {
                self.bits & !Rosc::NSP_MASK
            }
        }
    }

    /// Set **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, ROSC can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn with_nsu(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Rosc::NSU_MASK
            } else {
                self.bits & !Rosc::NSU_MASK
            }
        }
    }
}

impl Default for RoscVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Rosc {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RoscVal {
        RoscVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RoscVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RoscVal) -> RoscVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PLL_SYS Register
///
/// Control whether debugger, DMA, core 0 and core 1 can access PLL_SYS, and at what security/privilege levels they can do so.
/// Defaults to Secure, Privileged processor or debug access only.
/// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
///
/// Address offset: 0x00cc
/// Reset value: 0x000000b8
///
/// ## Fields
///
/// - **DBG** `[7]` (RW)
///
///   If 1, PLL_SYS can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **DMA** `[6]` (RW)
///
///   If 1, PLL_SYS can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE1** `[5]` (RW)
///
///   If 1, PLL_SYS can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE0** `[4]` (RW)
///
///   If 1, PLL_SYS can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **SP** `[3]` (RW)
///
///   If 1, PLL_SYS can be accessed from a Secure, Privileged context.
///
/// - **SU** `[2]` (RW)
///
///   If 1, and SP is also set, PLL_SYS can be accessed from a Secure, Unprivileged context.
///
/// - **NSP** `[1]` (RW)
///
///   If 1, PLL_SYS can be accessed from a Non-secure, Privileged context.
///
/// - **NSU** `[0]` (RW)
///
///   If 1, and NSP is also set, PLL_SYS can be accessed from a Non-secure, Unprivileged context.
///   This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
#[repr(transparent)]
pub struct PllSys {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for PllSys {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PllSys {
    // ======== Field Constants ========

    /// **DBG**
    ///
    /// If 1, PLL_SYS can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const DBG_OFFSET: u32 = 7;
    pub const DBG_MASK: u32 = 0x00000080;
    pub const DBG: u32 = 1 << 7;

    /// **DMA**
    ///
    /// If 1, PLL_SYS can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const DMA_OFFSET: u32 = 6;
    pub const DMA_MASK: u32 = 0x00000040;
    pub const DMA: u32 = 1 << 6;

    /// **CORE1**
    ///
    /// If 1, PLL_SYS can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CORE1_OFFSET: u32 = 5;
    pub const CORE1_MASK: u32 = 0x00000020;
    pub const CORE1: u32 = 1 << 5;

    /// **CORE0**
    ///
    /// If 1, PLL_SYS can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CORE0_OFFSET: u32 = 4;
    pub const CORE0_MASK: u32 = 0x00000010;
    pub const CORE0: u32 = 1 << 4;

    /// **SP**
    ///
    /// If 1, PLL_SYS can be accessed from a Secure, Privileged context.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SP_OFFSET: u32 = 3;
    pub const SP_MASK: u32 = 0x00000008;
    pub const SP: u32 = 1 << 3;

    /// **SU**
    ///
    /// If 1, and SP is also set, PLL_SYS can be accessed from a Secure, Unprivileged context.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SU_OFFSET: u32 = 2;
    pub const SU_MASK: u32 = 0x00000004;
    pub const SU: u32 = 1 << 2;

    /// **NSP**
    ///
    /// If 1, PLL_SYS can be accessed from a Non-secure, Privileged context.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const NSP_OFFSET: u32 = 1;
    pub const NSP_MASK: u32 = 0x00000002;
    pub const NSP: u32 = 1 << 1;

    /// **NSU**
    ///
    /// If 1, and NSP is also set, PLL_SYS can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const NSU_OFFSET: u32 = 0;
    pub const NSU_MASK: u32 = 0x00000001;
    pub const NSU: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DBG** `[7]` (RW)
    ///
    /// If 1, PLL_SYS can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dbg(&self) -> bool {
        (self.value.read_bits() & Self::DBG_MASK) != 0
    }

    /// Set DBG bit atomically (RW)
    #[inline(always)]
    pub fn set_dbg(&self) {
        self.set_bits(Self::DBG_MASK);
    }

    /// Clear DBG bit atomically (RW)
    #[inline(always)]
    pub fn clr_dbg(&self) {
        self.clr_bits(Self::DBG_MASK);
    }

    /// **DMA** `[6]` (RW)
    ///
    /// If 1, PLL_SYS can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dma(&self) -> bool {
        (self.value.read_bits() & Self::DMA_MASK) != 0
    }

    /// Set DMA bit atomically (RW)
    #[inline(always)]
    pub fn set_dma(&self) {
        self.set_bits(Self::DMA_MASK);
    }

    /// Clear DMA bit atomically (RW)
    #[inline(always)]
    pub fn clr_dma(&self) {
        self.clr_bits(Self::DMA_MASK);
    }

    /// **CORE1** `[5]` (RW)
    ///
    /// If 1, PLL_SYS can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core1(&self) -> bool {
        (self.value.read_bits() & Self::CORE1_MASK) != 0
    }

    /// Set CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn set_core1(&self) {
        self.set_bits(Self::CORE1_MASK);
    }

    /// Clear CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core1(&self) {
        self.clr_bits(Self::CORE1_MASK);
    }

    /// **CORE0** `[4]` (RW)
    ///
    /// If 1, PLL_SYS can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core0(&self) -> bool {
        (self.value.read_bits() & Self::CORE0_MASK) != 0
    }

    /// Set CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn set_core0(&self) {
        self.set_bits(Self::CORE0_MASK);
    }

    /// Clear CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core0(&self) {
        self.clr_bits(Self::CORE0_MASK);
    }

    /// **SP** `[3]` (RW)
    ///
    /// If 1, PLL_SYS can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub fn sp(&self) -> bool {
        (self.value.read_bits() & Self::SP_MASK) != 0
    }

    /// Set SP bit atomically (RW)
    #[inline(always)]
    pub fn set_sp(&self) {
        self.set_bits(Self::SP_MASK);
    }

    /// Clear SP bit atomically (RW)
    #[inline(always)]
    pub fn clr_sp(&self) {
        self.clr_bits(Self::SP_MASK);
    }

    /// **SU** `[2]` (RW)
    ///
    /// If 1, and SP is also set, PLL_SYS can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub fn su(&self) -> bool {
        (self.value.read_bits() & Self::SU_MASK) != 0
    }

    /// Set SU bit atomically (RW)
    #[inline(always)]
    pub fn set_su(&self) {
        self.set_bits(Self::SU_MASK);
    }

    /// Clear SU bit atomically (RW)
    #[inline(always)]
    pub fn clr_su(&self) {
        self.clr_bits(Self::SU_MASK);
    }

    /// **NSP** `[1]` (RW)
    ///
    /// If 1, PLL_SYS can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub fn nsp(&self) -> bool {
        (self.value.read_bits() & Self::NSP_MASK) != 0
    }

    /// Set NSP bit atomically (RW)
    #[inline(always)]
    pub fn set_nsp(&self) {
        self.set_bits(Self::NSP_MASK);
    }

    /// Clear NSP bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsp(&self) {
        self.clr_bits(Self::NSP_MASK);
    }

    /// **NSU** `[0]` (RW)
    ///
    /// If 1, and NSP is also set, PLL_SYS can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub fn nsu(&self) -> bool {
        (self.value.read_bits() & Self::NSU_MASK) != 0
    }

    /// Set NSU bit atomically (RW)
    #[inline(always)]
    pub fn set_nsu(&self) {
        self.set_bits(Self::NSU_MASK);
    }

    /// Clear NSU bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsu(&self) {
        self.clr_bits(Self::NSU_MASK);
    }
}

/// PLL_SYS value type for batch read/modify/write operations
///
/// Use `pllsys.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pllsys.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PllSysVal {
    bits: u32,
}

impl PllSysVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x000000b8 }
    }

    /// Read **DBG** `[7:7]`
    ///
    /// If 1, PLL_SYS can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        (self.bits & PllSys::DBG_MASK) != 0
    }

    /// Read **DMA** `[6:6]`
    ///
    /// If 1, PLL_SYS can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        (self.bits & PllSys::DMA_MASK) != 0
    }

    /// Read **CORE1** `[5:5]`
    ///
    /// If 1, PLL_SYS can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        (self.bits & PllSys::CORE1_MASK) != 0
    }

    /// Read **CORE0** `[4:4]`
    ///
    /// If 1, PLL_SYS can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        (self.bits & PllSys::CORE0_MASK) != 0
    }

    /// Read **SP** `[3:3]`
    ///
    /// If 1, PLL_SYS can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        (self.bits & PllSys::SP_MASK) != 0
    }

    /// Read **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, PLL_SYS can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn su(&self) -> bool {
        (self.bits & PllSys::SU_MASK) != 0
    }

    /// Read **NSP** `[1:1]`
    ///
    /// If 1, PLL_SYS can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        (self.bits & PllSys::NSP_MASK) != 0
    }

    /// Read **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, PLL_SYS can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        (self.bits & PllSys::NSU_MASK) != 0
    }

    /// Set **DBG** `[7:7]`
    ///
    /// If 1, PLL_SYS can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dbg(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PllSys::DBG_MASK
            } else {
                self.bits & !PllSys::DBG_MASK
            }
        }
    }

    /// Set **DMA** `[6:6]`
    ///
    /// If 1, PLL_SYS can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dma(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PllSys::DMA_MASK
            } else {
                self.bits & !PllSys::DMA_MASK
            }
        }
    }

    /// Set **CORE1** `[5:5]`
    ///
    /// If 1, PLL_SYS can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PllSys::CORE1_MASK
            } else {
                self.bits & !PllSys::CORE1_MASK
            }
        }
    }

    /// Set **CORE0** `[4:4]`
    ///
    /// If 1, PLL_SYS can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PllSys::CORE0_MASK
            } else {
                self.bits & !PllSys::CORE0_MASK
            }
        }
    }

    /// Set **SP** `[3:3]`
    ///
    /// If 1, PLL_SYS can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn with_sp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PllSys::SP_MASK
            } else {
                self.bits & !PllSys::SP_MASK
            }
        }
    }

    /// Set **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, PLL_SYS can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn with_su(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PllSys::SU_MASK
            } else {
                self.bits & !PllSys::SU_MASK
            }
        }
    }

    /// Set **NSP** `[1:1]`
    ///
    /// If 1, PLL_SYS can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn with_nsp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PllSys::NSP_MASK
            } else {
                self.bits & !PllSys::NSP_MASK
            }
        }
    }

    /// Set **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, PLL_SYS can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn with_nsu(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PllSys::NSU_MASK
            } else {
                self.bits & !PllSys::NSU_MASK
            }
        }
    }
}

impl Default for PllSysVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl PllSys {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PllSysVal {
        PllSysVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: PllSysVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(PllSysVal) -> PllSysVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PLL_USB Register
///
/// Control whether debugger, DMA, core 0 and core 1 can access PLL_USB, and at what security/privilege levels they can do so.
/// Defaults to Secure, Privileged processor or debug access only.
/// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
///
/// Address offset: 0x00d0
/// Reset value: 0x000000b8
///
/// ## Fields
///
/// - **DBG** `[7]` (RW)
///
///   If 1, PLL_USB can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **DMA** `[6]` (RW)
///
///   If 1, PLL_USB can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE1** `[5]` (RW)
///
///   If 1, PLL_USB can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE0** `[4]` (RW)
///
///   If 1, PLL_USB can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **SP** `[3]` (RW)
///
///   If 1, PLL_USB can be accessed from a Secure, Privileged context.
///
/// - **SU** `[2]` (RW)
///
///   If 1, and SP is also set, PLL_USB can be accessed from a Secure, Unprivileged context.
///
/// - **NSP** `[1]` (RW)
///
///   If 1, PLL_USB can be accessed from a Non-secure, Privileged context.
///
/// - **NSU** `[0]` (RW)
///
///   If 1, and NSP is also set, PLL_USB can be accessed from a Non-secure, Unprivileged context.
///   This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
#[repr(transparent)]
pub struct PllUsb {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for PllUsb {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PllUsb {
    // ======== Field Constants ========

    /// **DBG**
    ///
    /// If 1, PLL_USB can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const DBG_OFFSET: u32 = 7;
    pub const DBG_MASK: u32 = 0x00000080;
    pub const DBG: u32 = 1 << 7;

    /// **DMA**
    ///
    /// If 1, PLL_USB can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const DMA_OFFSET: u32 = 6;
    pub const DMA_MASK: u32 = 0x00000040;
    pub const DMA: u32 = 1 << 6;

    /// **CORE1**
    ///
    /// If 1, PLL_USB can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CORE1_OFFSET: u32 = 5;
    pub const CORE1_MASK: u32 = 0x00000020;
    pub const CORE1: u32 = 1 << 5;

    /// **CORE0**
    ///
    /// If 1, PLL_USB can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CORE0_OFFSET: u32 = 4;
    pub const CORE0_MASK: u32 = 0x00000010;
    pub const CORE0: u32 = 1 << 4;

    /// **SP**
    ///
    /// If 1, PLL_USB can be accessed from a Secure, Privileged context.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SP_OFFSET: u32 = 3;
    pub const SP_MASK: u32 = 0x00000008;
    pub const SP: u32 = 1 << 3;

    /// **SU**
    ///
    /// If 1, and SP is also set, PLL_USB can be accessed from a Secure, Unprivileged context.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SU_OFFSET: u32 = 2;
    pub const SU_MASK: u32 = 0x00000004;
    pub const SU: u32 = 1 << 2;

    /// **NSP**
    ///
    /// If 1, PLL_USB can be accessed from a Non-secure, Privileged context.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const NSP_OFFSET: u32 = 1;
    pub const NSP_MASK: u32 = 0x00000002;
    pub const NSP: u32 = 1 << 1;

    /// **NSU**
    ///
    /// If 1, and NSP is also set, PLL_USB can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const NSU_OFFSET: u32 = 0;
    pub const NSU_MASK: u32 = 0x00000001;
    pub const NSU: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DBG** `[7]` (RW)
    ///
    /// If 1, PLL_USB can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dbg(&self) -> bool {
        (self.value.read_bits() & Self::DBG_MASK) != 0
    }

    /// Set DBG bit atomically (RW)
    #[inline(always)]
    pub fn set_dbg(&self) {
        self.set_bits(Self::DBG_MASK);
    }

    /// Clear DBG bit atomically (RW)
    #[inline(always)]
    pub fn clr_dbg(&self) {
        self.clr_bits(Self::DBG_MASK);
    }

    /// **DMA** `[6]` (RW)
    ///
    /// If 1, PLL_USB can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dma(&self) -> bool {
        (self.value.read_bits() & Self::DMA_MASK) != 0
    }

    /// Set DMA bit atomically (RW)
    #[inline(always)]
    pub fn set_dma(&self) {
        self.set_bits(Self::DMA_MASK);
    }

    /// Clear DMA bit atomically (RW)
    #[inline(always)]
    pub fn clr_dma(&self) {
        self.clr_bits(Self::DMA_MASK);
    }

    /// **CORE1** `[5]` (RW)
    ///
    /// If 1, PLL_USB can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core1(&self) -> bool {
        (self.value.read_bits() & Self::CORE1_MASK) != 0
    }

    /// Set CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn set_core1(&self) {
        self.set_bits(Self::CORE1_MASK);
    }

    /// Clear CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core1(&self) {
        self.clr_bits(Self::CORE1_MASK);
    }

    /// **CORE0** `[4]` (RW)
    ///
    /// If 1, PLL_USB can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core0(&self) -> bool {
        (self.value.read_bits() & Self::CORE0_MASK) != 0
    }

    /// Set CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn set_core0(&self) {
        self.set_bits(Self::CORE0_MASK);
    }

    /// Clear CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core0(&self) {
        self.clr_bits(Self::CORE0_MASK);
    }

    /// **SP** `[3]` (RW)
    ///
    /// If 1, PLL_USB can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub fn sp(&self) -> bool {
        (self.value.read_bits() & Self::SP_MASK) != 0
    }

    /// Set SP bit atomically (RW)
    #[inline(always)]
    pub fn set_sp(&self) {
        self.set_bits(Self::SP_MASK);
    }

    /// Clear SP bit atomically (RW)
    #[inline(always)]
    pub fn clr_sp(&self) {
        self.clr_bits(Self::SP_MASK);
    }

    /// **SU** `[2]` (RW)
    ///
    /// If 1, and SP is also set, PLL_USB can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub fn su(&self) -> bool {
        (self.value.read_bits() & Self::SU_MASK) != 0
    }

    /// Set SU bit atomically (RW)
    #[inline(always)]
    pub fn set_su(&self) {
        self.set_bits(Self::SU_MASK);
    }

    /// Clear SU bit atomically (RW)
    #[inline(always)]
    pub fn clr_su(&self) {
        self.clr_bits(Self::SU_MASK);
    }

    /// **NSP** `[1]` (RW)
    ///
    /// If 1, PLL_USB can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub fn nsp(&self) -> bool {
        (self.value.read_bits() & Self::NSP_MASK) != 0
    }

    /// Set NSP bit atomically (RW)
    #[inline(always)]
    pub fn set_nsp(&self) {
        self.set_bits(Self::NSP_MASK);
    }

    /// Clear NSP bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsp(&self) {
        self.clr_bits(Self::NSP_MASK);
    }

    /// **NSU** `[0]` (RW)
    ///
    /// If 1, and NSP is also set, PLL_USB can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub fn nsu(&self) -> bool {
        (self.value.read_bits() & Self::NSU_MASK) != 0
    }

    /// Set NSU bit atomically (RW)
    #[inline(always)]
    pub fn set_nsu(&self) {
        self.set_bits(Self::NSU_MASK);
    }

    /// Clear NSU bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsu(&self) {
        self.clr_bits(Self::NSU_MASK);
    }
}

/// PLL_USB value type for batch read/modify/write operations
///
/// Use `pllusb.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pllusb.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PllUsbVal {
    bits: u32,
}

impl PllUsbVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x000000b8 }
    }

    /// Read **DBG** `[7:7]`
    ///
    /// If 1, PLL_USB can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        (self.bits & PllUsb::DBG_MASK) != 0
    }

    /// Read **DMA** `[6:6]`
    ///
    /// If 1, PLL_USB can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        (self.bits & PllUsb::DMA_MASK) != 0
    }

    /// Read **CORE1** `[5:5]`
    ///
    /// If 1, PLL_USB can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        (self.bits & PllUsb::CORE1_MASK) != 0
    }

    /// Read **CORE0** `[4:4]`
    ///
    /// If 1, PLL_USB can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        (self.bits & PllUsb::CORE0_MASK) != 0
    }

    /// Read **SP** `[3:3]`
    ///
    /// If 1, PLL_USB can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        (self.bits & PllUsb::SP_MASK) != 0
    }

    /// Read **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, PLL_USB can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn su(&self) -> bool {
        (self.bits & PllUsb::SU_MASK) != 0
    }

    /// Read **NSP** `[1:1]`
    ///
    /// If 1, PLL_USB can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        (self.bits & PllUsb::NSP_MASK) != 0
    }

    /// Read **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, PLL_USB can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        (self.bits & PllUsb::NSU_MASK) != 0
    }

    /// Set **DBG** `[7:7]`
    ///
    /// If 1, PLL_USB can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dbg(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PllUsb::DBG_MASK
            } else {
                self.bits & !PllUsb::DBG_MASK
            }
        }
    }

    /// Set **DMA** `[6:6]`
    ///
    /// If 1, PLL_USB can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dma(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PllUsb::DMA_MASK
            } else {
                self.bits & !PllUsb::DMA_MASK
            }
        }
    }

    /// Set **CORE1** `[5:5]`
    ///
    /// If 1, PLL_USB can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PllUsb::CORE1_MASK
            } else {
                self.bits & !PllUsb::CORE1_MASK
            }
        }
    }

    /// Set **CORE0** `[4:4]`
    ///
    /// If 1, PLL_USB can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PllUsb::CORE0_MASK
            } else {
                self.bits & !PllUsb::CORE0_MASK
            }
        }
    }

    /// Set **SP** `[3:3]`
    ///
    /// If 1, PLL_USB can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn with_sp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PllUsb::SP_MASK
            } else {
                self.bits & !PllUsb::SP_MASK
            }
        }
    }

    /// Set **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, PLL_USB can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn with_su(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PllUsb::SU_MASK
            } else {
                self.bits & !PllUsb::SU_MASK
            }
        }
    }

    /// Set **NSP** `[1:1]`
    ///
    /// If 1, PLL_USB can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn with_nsp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PllUsb::NSP_MASK
            } else {
                self.bits & !PllUsb::NSP_MASK
            }
        }
    }

    /// Set **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, PLL_USB can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn with_nsu(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PllUsb::NSU_MASK
            } else {
                self.bits & !PllUsb::NSU_MASK
            }
        }
    }
}

impl Default for PllUsbVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl PllUsb {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PllUsbVal {
        PllUsbVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: PllUsbVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(PllUsbVal) -> PllUsbVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TICKS Register
///
/// Control whether debugger, DMA, core 0 and core 1 can access TICKS, and at what security/privilege levels they can do so.
/// Defaults to Secure, Privileged processor or debug access only.
/// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
///
/// Address offset: 0x00d4
/// Reset value: 0x000000b8
///
/// ## Fields
///
/// - **DBG** `[7]` (RW)
///
///   If 1, TICKS can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **DMA** `[6]` (RW)
///
///   If 1, TICKS can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE1** `[5]` (RW)
///
///   If 1, TICKS can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE0** `[4]` (RW)
///
///   If 1, TICKS can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **SP** `[3]` (RW)
///
///   If 1, TICKS can be accessed from a Secure, Privileged context.
///
/// - **SU** `[2]` (RW)
///
///   If 1, and SP is also set, TICKS can be accessed from a Secure, Unprivileged context.
///
/// - **NSP** `[1]` (RW)
///
///   If 1, TICKS can be accessed from a Non-secure, Privileged context.
///
/// - **NSU** `[0]` (RW)
///
///   If 1, and NSP is also set, TICKS can be accessed from a Non-secure, Unprivileged context.
///   This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
#[repr(transparent)]
pub struct Ticks {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Ticks {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ticks {
    // ======== Field Constants ========

    /// **DBG**
    ///
    /// If 1, TICKS can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const DBG_OFFSET: u32 = 7;
    pub const DBG_MASK: u32 = 0x00000080;
    pub const DBG: u32 = 1 << 7;

    /// **DMA**
    ///
    /// If 1, TICKS can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const DMA_OFFSET: u32 = 6;
    pub const DMA_MASK: u32 = 0x00000040;
    pub const DMA: u32 = 1 << 6;

    /// **CORE1**
    ///
    /// If 1, TICKS can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CORE1_OFFSET: u32 = 5;
    pub const CORE1_MASK: u32 = 0x00000020;
    pub const CORE1: u32 = 1 << 5;

    /// **CORE0**
    ///
    /// If 1, TICKS can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CORE0_OFFSET: u32 = 4;
    pub const CORE0_MASK: u32 = 0x00000010;
    pub const CORE0: u32 = 1 << 4;

    /// **SP**
    ///
    /// If 1, TICKS can be accessed from a Secure, Privileged context.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SP_OFFSET: u32 = 3;
    pub const SP_MASK: u32 = 0x00000008;
    pub const SP: u32 = 1 << 3;

    /// **SU**
    ///
    /// If 1, and SP is also set, TICKS can be accessed from a Secure, Unprivileged context.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SU_OFFSET: u32 = 2;
    pub const SU_MASK: u32 = 0x00000004;
    pub const SU: u32 = 1 << 2;

    /// **NSP**
    ///
    /// If 1, TICKS can be accessed from a Non-secure, Privileged context.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const NSP_OFFSET: u32 = 1;
    pub const NSP_MASK: u32 = 0x00000002;
    pub const NSP: u32 = 1 << 1;

    /// **NSU**
    ///
    /// If 1, and NSP is also set, TICKS can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const NSU_OFFSET: u32 = 0;
    pub const NSU_MASK: u32 = 0x00000001;
    pub const NSU: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DBG** `[7]` (RW)
    ///
    /// If 1, TICKS can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dbg(&self) -> bool {
        (self.value.read_bits() & Self::DBG_MASK) != 0
    }

    /// Set DBG bit atomically (RW)
    #[inline(always)]
    pub fn set_dbg(&self) {
        self.set_bits(Self::DBG_MASK);
    }

    /// Clear DBG bit atomically (RW)
    #[inline(always)]
    pub fn clr_dbg(&self) {
        self.clr_bits(Self::DBG_MASK);
    }

    /// **DMA** `[6]` (RW)
    ///
    /// If 1, TICKS can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dma(&self) -> bool {
        (self.value.read_bits() & Self::DMA_MASK) != 0
    }

    /// Set DMA bit atomically (RW)
    #[inline(always)]
    pub fn set_dma(&self) {
        self.set_bits(Self::DMA_MASK);
    }

    /// Clear DMA bit atomically (RW)
    #[inline(always)]
    pub fn clr_dma(&self) {
        self.clr_bits(Self::DMA_MASK);
    }

    /// **CORE1** `[5]` (RW)
    ///
    /// If 1, TICKS can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core1(&self) -> bool {
        (self.value.read_bits() & Self::CORE1_MASK) != 0
    }

    /// Set CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn set_core1(&self) {
        self.set_bits(Self::CORE1_MASK);
    }

    /// Clear CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core1(&self) {
        self.clr_bits(Self::CORE1_MASK);
    }

    /// **CORE0** `[4]` (RW)
    ///
    /// If 1, TICKS can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core0(&self) -> bool {
        (self.value.read_bits() & Self::CORE0_MASK) != 0
    }

    /// Set CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn set_core0(&self) {
        self.set_bits(Self::CORE0_MASK);
    }

    /// Clear CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core0(&self) {
        self.clr_bits(Self::CORE0_MASK);
    }

    /// **SP** `[3]` (RW)
    ///
    /// If 1, TICKS can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub fn sp(&self) -> bool {
        (self.value.read_bits() & Self::SP_MASK) != 0
    }

    /// Set SP bit atomically (RW)
    #[inline(always)]
    pub fn set_sp(&self) {
        self.set_bits(Self::SP_MASK);
    }

    /// Clear SP bit atomically (RW)
    #[inline(always)]
    pub fn clr_sp(&self) {
        self.clr_bits(Self::SP_MASK);
    }

    /// **SU** `[2]` (RW)
    ///
    /// If 1, and SP is also set, TICKS can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub fn su(&self) -> bool {
        (self.value.read_bits() & Self::SU_MASK) != 0
    }

    /// Set SU bit atomically (RW)
    #[inline(always)]
    pub fn set_su(&self) {
        self.set_bits(Self::SU_MASK);
    }

    /// Clear SU bit atomically (RW)
    #[inline(always)]
    pub fn clr_su(&self) {
        self.clr_bits(Self::SU_MASK);
    }

    /// **NSP** `[1]` (RW)
    ///
    /// If 1, TICKS can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub fn nsp(&self) -> bool {
        (self.value.read_bits() & Self::NSP_MASK) != 0
    }

    /// Set NSP bit atomically (RW)
    #[inline(always)]
    pub fn set_nsp(&self) {
        self.set_bits(Self::NSP_MASK);
    }

    /// Clear NSP bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsp(&self) {
        self.clr_bits(Self::NSP_MASK);
    }

    /// **NSU** `[0]` (RW)
    ///
    /// If 1, and NSP is also set, TICKS can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub fn nsu(&self) -> bool {
        (self.value.read_bits() & Self::NSU_MASK) != 0
    }

    /// Set NSU bit atomically (RW)
    #[inline(always)]
    pub fn set_nsu(&self) {
        self.set_bits(Self::NSU_MASK);
    }

    /// Clear NSU bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsu(&self) {
        self.clr_bits(Self::NSU_MASK);
    }
}

/// TICKS value type for batch read/modify/write operations
///
/// Use `ticks.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ticks.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TicksVal {
    bits: u32,
}

impl TicksVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x000000b8 }
    }

    /// Read **DBG** `[7:7]`
    ///
    /// If 1, TICKS can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        (self.bits & Ticks::DBG_MASK) != 0
    }

    /// Read **DMA** `[6:6]`
    ///
    /// If 1, TICKS can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        (self.bits & Ticks::DMA_MASK) != 0
    }

    /// Read **CORE1** `[5:5]`
    ///
    /// If 1, TICKS can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        (self.bits & Ticks::CORE1_MASK) != 0
    }

    /// Read **CORE0** `[4:4]`
    ///
    /// If 1, TICKS can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        (self.bits & Ticks::CORE0_MASK) != 0
    }

    /// Read **SP** `[3:3]`
    ///
    /// If 1, TICKS can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        (self.bits & Ticks::SP_MASK) != 0
    }

    /// Read **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, TICKS can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn su(&self) -> bool {
        (self.bits & Ticks::SU_MASK) != 0
    }

    /// Read **NSP** `[1:1]`
    ///
    /// If 1, TICKS can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        (self.bits & Ticks::NSP_MASK) != 0
    }

    /// Read **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, TICKS can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        (self.bits & Ticks::NSU_MASK) != 0
    }

    /// Set **DBG** `[7:7]`
    ///
    /// If 1, TICKS can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dbg(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ticks::DBG_MASK
            } else {
                self.bits & !Ticks::DBG_MASK
            }
        }
    }

    /// Set **DMA** `[6:6]`
    ///
    /// If 1, TICKS can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dma(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ticks::DMA_MASK
            } else {
                self.bits & !Ticks::DMA_MASK
            }
        }
    }

    /// Set **CORE1** `[5:5]`
    ///
    /// If 1, TICKS can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ticks::CORE1_MASK
            } else {
                self.bits & !Ticks::CORE1_MASK
            }
        }
    }

    /// Set **CORE0** `[4:4]`
    ///
    /// If 1, TICKS can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ticks::CORE0_MASK
            } else {
                self.bits & !Ticks::CORE0_MASK
            }
        }
    }

    /// Set **SP** `[3:3]`
    ///
    /// If 1, TICKS can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn with_sp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ticks::SP_MASK
            } else {
                self.bits & !Ticks::SP_MASK
            }
        }
    }

    /// Set **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, TICKS can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn with_su(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ticks::SU_MASK
            } else {
                self.bits & !Ticks::SU_MASK
            }
        }
    }

    /// Set **NSP** `[1:1]`
    ///
    /// If 1, TICKS can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn with_nsp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ticks::NSP_MASK
            } else {
                self.bits & !Ticks::NSP_MASK
            }
        }
    }

    /// Set **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, TICKS can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn with_nsu(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ticks::NSU_MASK
            } else {
                self.bits & !Ticks::NSU_MASK
            }
        }
    }
}

impl Default for TicksVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ticks {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TicksVal {
        TicksVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TicksVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TicksVal) -> TicksVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// WATCHDOG Register
///
/// Control whether debugger, DMA, core 0 and core 1 can access WATCHDOG, and at what security/privilege levels they can do so.
/// Defaults to Secure, Privileged processor or debug access only.
/// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
///
/// Address offset: 0x00d8
/// Reset value: 0x000000b8
///
/// ## Fields
///
/// - **DBG** `[7]` (RW)
///
///   If 1, WATCHDOG can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **DMA** `[6]` (RW)
///
///   If 1, WATCHDOG can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE1** `[5]` (RW)
///
///   If 1, WATCHDOG can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE0** `[4]` (RW)
///
///   If 1, WATCHDOG can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **SP** `[3]` (RW)
///
///   If 1, WATCHDOG can be accessed from a Secure, Privileged context.
///
/// - **SU** `[2]` (RW)
///
///   If 1, and SP is also set, WATCHDOG can be accessed from a Secure, Unprivileged context.
///
/// - **NSP** `[1]` (RW)
///
///   If 1, WATCHDOG can be accessed from a Non-secure, Privileged context.
///
/// - **NSU** `[0]` (RW)
///
///   If 1, and NSP is also set, WATCHDOG can be accessed from a Non-secure, Unprivileged context.
///   This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
#[repr(transparent)]
pub struct Watchdog {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Watchdog {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Watchdog {
    // ======== Field Constants ========

    /// **DBG**
    ///
    /// If 1, WATCHDOG can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const DBG_OFFSET: u32 = 7;
    pub const DBG_MASK: u32 = 0x00000080;
    pub const DBG: u32 = 1 << 7;

    /// **DMA**
    ///
    /// If 1, WATCHDOG can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const DMA_OFFSET: u32 = 6;
    pub const DMA_MASK: u32 = 0x00000040;
    pub const DMA: u32 = 1 << 6;

    /// **CORE1**
    ///
    /// If 1, WATCHDOG can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CORE1_OFFSET: u32 = 5;
    pub const CORE1_MASK: u32 = 0x00000020;
    pub const CORE1: u32 = 1 << 5;

    /// **CORE0**
    ///
    /// If 1, WATCHDOG can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CORE0_OFFSET: u32 = 4;
    pub const CORE0_MASK: u32 = 0x00000010;
    pub const CORE0: u32 = 1 << 4;

    /// **SP**
    ///
    /// If 1, WATCHDOG can be accessed from a Secure, Privileged context.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SP_OFFSET: u32 = 3;
    pub const SP_MASK: u32 = 0x00000008;
    pub const SP: u32 = 1 << 3;

    /// **SU**
    ///
    /// If 1, and SP is also set, WATCHDOG can be accessed from a Secure, Unprivileged context.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SU_OFFSET: u32 = 2;
    pub const SU_MASK: u32 = 0x00000004;
    pub const SU: u32 = 1 << 2;

    /// **NSP**
    ///
    /// If 1, WATCHDOG can be accessed from a Non-secure, Privileged context.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const NSP_OFFSET: u32 = 1;
    pub const NSP_MASK: u32 = 0x00000002;
    pub const NSP: u32 = 1 << 1;

    /// **NSU**
    ///
    /// If 1, and NSP is also set, WATCHDOG can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const NSU_OFFSET: u32 = 0;
    pub const NSU_MASK: u32 = 0x00000001;
    pub const NSU: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DBG** `[7]` (RW)
    ///
    /// If 1, WATCHDOG can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dbg(&self) -> bool {
        (self.value.read_bits() & Self::DBG_MASK) != 0
    }

    /// Set DBG bit atomically (RW)
    #[inline(always)]
    pub fn set_dbg(&self) {
        self.set_bits(Self::DBG_MASK);
    }

    /// Clear DBG bit atomically (RW)
    #[inline(always)]
    pub fn clr_dbg(&self) {
        self.clr_bits(Self::DBG_MASK);
    }

    /// **DMA** `[6]` (RW)
    ///
    /// If 1, WATCHDOG can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dma(&self) -> bool {
        (self.value.read_bits() & Self::DMA_MASK) != 0
    }

    /// Set DMA bit atomically (RW)
    #[inline(always)]
    pub fn set_dma(&self) {
        self.set_bits(Self::DMA_MASK);
    }

    /// Clear DMA bit atomically (RW)
    #[inline(always)]
    pub fn clr_dma(&self) {
        self.clr_bits(Self::DMA_MASK);
    }

    /// **CORE1** `[5]` (RW)
    ///
    /// If 1, WATCHDOG can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core1(&self) -> bool {
        (self.value.read_bits() & Self::CORE1_MASK) != 0
    }

    /// Set CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn set_core1(&self) {
        self.set_bits(Self::CORE1_MASK);
    }

    /// Clear CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core1(&self) {
        self.clr_bits(Self::CORE1_MASK);
    }

    /// **CORE0** `[4]` (RW)
    ///
    /// If 1, WATCHDOG can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core0(&self) -> bool {
        (self.value.read_bits() & Self::CORE0_MASK) != 0
    }

    /// Set CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn set_core0(&self) {
        self.set_bits(Self::CORE0_MASK);
    }

    /// Clear CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core0(&self) {
        self.clr_bits(Self::CORE0_MASK);
    }

    /// **SP** `[3]` (RW)
    ///
    /// If 1, WATCHDOG can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub fn sp(&self) -> bool {
        (self.value.read_bits() & Self::SP_MASK) != 0
    }

    /// Set SP bit atomically (RW)
    #[inline(always)]
    pub fn set_sp(&self) {
        self.set_bits(Self::SP_MASK);
    }

    /// Clear SP bit atomically (RW)
    #[inline(always)]
    pub fn clr_sp(&self) {
        self.clr_bits(Self::SP_MASK);
    }

    /// **SU** `[2]` (RW)
    ///
    /// If 1, and SP is also set, WATCHDOG can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub fn su(&self) -> bool {
        (self.value.read_bits() & Self::SU_MASK) != 0
    }

    /// Set SU bit atomically (RW)
    #[inline(always)]
    pub fn set_su(&self) {
        self.set_bits(Self::SU_MASK);
    }

    /// Clear SU bit atomically (RW)
    #[inline(always)]
    pub fn clr_su(&self) {
        self.clr_bits(Self::SU_MASK);
    }

    /// **NSP** `[1]` (RW)
    ///
    /// If 1, WATCHDOG can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub fn nsp(&self) -> bool {
        (self.value.read_bits() & Self::NSP_MASK) != 0
    }

    /// Set NSP bit atomically (RW)
    #[inline(always)]
    pub fn set_nsp(&self) {
        self.set_bits(Self::NSP_MASK);
    }

    /// Clear NSP bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsp(&self) {
        self.clr_bits(Self::NSP_MASK);
    }

    /// **NSU** `[0]` (RW)
    ///
    /// If 1, and NSP is also set, WATCHDOG can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub fn nsu(&self) -> bool {
        (self.value.read_bits() & Self::NSU_MASK) != 0
    }

    /// Set NSU bit atomically (RW)
    #[inline(always)]
    pub fn set_nsu(&self) {
        self.set_bits(Self::NSU_MASK);
    }

    /// Clear NSU bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsu(&self) {
        self.clr_bits(Self::NSU_MASK);
    }
}

/// WATCHDOG value type for batch read/modify/write operations
///
/// Use `watchdog.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `watchdog.write_val(val)`.
#[derive(Clone, Copy)]
pub struct WatchdogVal {
    bits: u32,
}

impl WatchdogVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x000000b8 }
    }

    /// Read **DBG** `[7:7]`
    ///
    /// If 1, WATCHDOG can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        (self.bits & Watchdog::DBG_MASK) != 0
    }

    /// Read **DMA** `[6:6]`
    ///
    /// If 1, WATCHDOG can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        (self.bits & Watchdog::DMA_MASK) != 0
    }

    /// Read **CORE1** `[5:5]`
    ///
    /// If 1, WATCHDOG can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        (self.bits & Watchdog::CORE1_MASK) != 0
    }

    /// Read **CORE0** `[4:4]`
    ///
    /// If 1, WATCHDOG can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        (self.bits & Watchdog::CORE0_MASK) != 0
    }

    /// Read **SP** `[3:3]`
    ///
    /// If 1, WATCHDOG can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        (self.bits & Watchdog::SP_MASK) != 0
    }

    /// Read **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, WATCHDOG can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn su(&self) -> bool {
        (self.bits & Watchdog::SU_MASK) != 0
    }

    /// Read **NSP** `[1:1]`
    ///
    /// If 1, WATCHDOG can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        (self.bits & Watchdog::NSP_MASK) != 0
    }

    /// Read **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, WATCHDOG can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        (self.bits & Watchdog::NSU_MASK) != 0
    }

    /// Set **DBG** `[7:7]`
    ///
    /// If 1, WATCHDOG can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dbg(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Watchdog::DBG_MASK
            } else {
                self.bits & !Watchdog::DBG_MASK
            }
        }
    }

    /// Set **DMA** `[6:6]`
    ///
    /// If 1, WATCHDOG can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dma(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Watchdog::DMA_MASK
            } else {
                self.bits & !Watchdog::DMA_MASK
            }
        }
    }

    /// Set **CORE1** `[5:5]`
    ///
    /// If 1, WATCHDOG can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Watchdog::CORE1_MASK
            } else {
                self.bits & !Watchdog::CORE1_MASK
            }
        }
    }

    /// Set **CORE0** `[4:4]`
    ///
    /// If 1, WATCHDOG can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Watchdog::CORE0_MASK
            } else {
                self.bits & !Watchdog::CORE0_MASK
            }
        }
    }

    /// Set **SP** `[3:3]`
    ///
    /// If 1, WATCHDOG can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn with_sp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Watchdog::SP_MASK
            } else {
                self.bits & !Watchdog::SP_MASK
            }
        }
    }

    /// Set **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, WATCHDOG can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn with_su(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Watchdog::SU_MASK
            } else {
                self.bits & !Watchdog::SU_MASK
            }
        }
    }

    /// Set **NSP** `[1:1]`
    ///
    /// If 1, WATCHDOG can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn with_nsp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Watchdog::NSP_MASK
            } else {
                self.bits & !Watchdog::NSP_MASK
            }
        }
    }

    /// Set **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, WATCHDOG can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn with_nsu(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Watchdog::NSU_MASK
            } else {
                self.bits & !Watchdog::NSU_MASK
            }
        }
    }
}

impl Default for WatchdogVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Watchdog {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> WatchdogVal {
        WatchdogVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: WatchdogVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(WatchdogVal) -> WatchdogVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RSM Register
///
/// Control whether debugger, DMA, core 0 and core 1 can access RSM, and at what security/privilege levels they can do so.
/// Defaults to Secure, Privileged processor or debug access only.
/// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
///
/// Address offset: 0x00dc
/// Reset value: 0x000000b8
///
/// ## Fields
///
/// - **DBG** `[7]` (RW)
///
///   If 1, RSM can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **DMA** `[6]` (RW)
///
///   If 1, RSM can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE1** `[5]` (RW)
///
///   If 1, RSM can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE0** `[4]` (RW)
///
///   If 1, RSM can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **SP** `[3]` (RW)
///
///   If 1, RSM can be accessed from a Secure, Privileged context.
///
/// - **SU** `[2]` (RW)
///
///   If 1, and SP is also set, RSM can be accessed from a Secure, Unprivileged context.
///
/// - **NSP** `[1]` (RW)
///
///   If 1, RSM can be accessed from a Non-secure, Privileged context.
///
/// - **NSU** `[0]` (RW)
///
///   If 1, and NSP is also set, RSM can be accessed from a Non-secure, Unprivileged context.
///   This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
#[repr(transparent)]
pub struct Rsm {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Rsm {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Rsm {
    // ======== Field Constants ========

    /// **DBG**
    ///
    /// If 1, RSM can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const DBG_OFFSET: u32 = 7;
    pub const DBG_MASK: u32 = 0x00000080;
    pub const DBG: u32 = 1 << 7;

    /// **DMA**
    ///
    /// If 1, RSM can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const DMA_OFFSET: u32 = 6;
    pub const DMA_MASK: u32 = 0x00000040;
    pub const DMA: u32 = 1 << 6;

    /// **CORE1**
    ///
    /// If 1, RSM can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CORE1_OFFSET: u32 = 5;
    pub const CORE1_MASK: u32 = 0x00000020;
    pub const CORE1: u32 = 1 << 5;

    /// **CORE0**
    ///
    /// If 1, RSM can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CORE0_OFFSET: u32 = 4;
    pub const CORE0_MASK: u32 = 0x00000010;
    pub const CORE0: u32 = 1 << 4;

    /// **SP**
    ///
    /// If 1, RSM can be accessed from a Secure, Privileged context.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SP_OFFSET: u32 = 3;
    pub const SP_MASK: u32 = 0x00000008;
    pub const SP: u32 = 1 << 3;

    /// **SU**
    ///
    /// If 1, and SP is also set, RSM can be accessed from a Secure, Unprivileged context.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SU_OFFSET: u32 = 2;
    pub const SU_MASK: u32 = 0x00000004;
    pub const SU: u32 = 1 << 2;

    /// **NSP**
    ///
    /// If 1, RSM can be accessed from a Non-secure, Privileged context.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const NSP_OFFSET: u32 = 1;
    pub const NSP_MASK: u32 = 0x00000002;
    pub const NSP: u32 = 1 << 1;

    /// **NSU**
    ///
    /// If 1, and NSP is also set, RSM can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const NSU_OFFSET: u32 = 0;
    pub const NSU_MASK: u32 = 0x00000001;
    pub const NSU: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DBG** `[7]` (RW)
    ///
    /// If 1, RSM can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dbg(&self) -> bool {
        (self.value.read_bits() & Self::DBG_MASK) != 0
    }

    /// Set DBG bit atomically (RW)
    #[inline(always)]
    pub fn set_dbg(&self) {
        self.set_bits(Self::DBG_MASK);
    }

    /// Clear DBG bit atomically (RW)
    #[inline(always)]
    pub fn clr_dbg(&self) {
        self.clr_bits(Self::DBG_MASK);
    }

    /// **DMA** `[6]` (RW)
    ///
    /// If 1, RSM can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dma(&self) -> bool {
        (self.value.read_bits() & Self::DMA_MASK) != 0
    }

    /// Set DMA bit atomically (RW)
    #[inline(always)]
    pub fn set_dma(&self) {
        self.set_bits(Self::DMA_MASK);
    }

    /// Clear DMA bit atomically (RW)
    #[inline(always)]
    pub fn clr_dma(&self) {
        self.clr_bits(Self::DMA_MASK);
    }

    /// **CORE1** `[5]` (RW)
    ///
    /// If 1, RSM can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core1(&self) -> bool {
        (self.value.read_bits() & Self::CORE1_MASK) != 0
    }

    /// Set CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn set_core1(&self) {
        self.set_bits(Self::CORE1_MASK);
    }

    /// Clear CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core1(&self) {
        self.clr_bits(Self::CORE1_MASK);
    }

    /// **CORE0** `[4]` (RW)
    ///
    /// If 1, RSM can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core0(&self) -> bool {
        (self.value.read_bits() & Self::CORE0_MASK) != 0
    }

    /// Set CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn set_core0(&self) {
        self.set_bits(Self::CORE0_MASK);
    }

    /// Clear CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core0(&self) {
        self.clr_bits(Self::CORE0_MASK);
    }

    /// **SP** `[3]` (RW)
    ///
    /// If 1, RSM can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub fn sp(&self) -> bool {
        (self.value.read_bits() & Self::SP_MASK) != 0
    }

    /// Set SP bit atomically (RW)
    #[inline(always)]
    pub fn set_sp(&self) {
        self.set_bits(Self::SP_MASK);
    }

    /// Clear SP bit atomically (RW)
    #[inline(always)]
    pub fn clr_sp(&self) {
        self.clr_bits(Self::SP_MASK);
    }

    /// **SU** `[2]` (RW)
    ///
    /// If 1, and SP is also set, RSM can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub fn su(&self) -> bool {
        (self.value.read_bits() & Self::SU_MASK) != 0
    }

    /// Set SU bit atomically (RW)
    #[inline(always)]
    pub fn set_su(&self) {
        self.set_bits(Self::SU_MASK);
    }

    /// Clear SU bit atomically (RW)
    #[inline(always)]
    pub fn clr_su(&self) {
        self.clr_bits(Self::SU_MASK);
    }

    /// **NSP** `[1]` (RW)
    ///
    /// If 1, RSM can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub fn nsp(&self) -> bool {
        (self.value.read_bits() & Self::NSP_MASK) != 0
    }

    /// Set NSP bit atomically (RW)
    #[inline(always)]
    pub fn set_nsp(&self) {
        self.set_bits(Self::NSP_MASK);
    }

    /// Clear NSP bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsp(&self) {
        self.clr_bits(Self::NSP_MASK);
    }

    /// **NSU** `[0]` (RW)
    ///
    /// If 1, and NSP is also set, RSM can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub fn nsu(&self) -> bool {
        (self.value.read_bits() & Self::NSU_MASK) != 0
    }

    /// Set NSU bit atomically (RW)
    #[inline(always)]
    pub fn set_nsu(&self) {
        self.set_bits(Self::NSU_MASK);
    }

    /// Clear NSU bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsu(&self) {
        self.clr_bits(Self::NSU_MASK);
    }
}

/// RSM value type for batch read/modify/write operations
///
/// Use `rsm.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rsm.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RsmVal {
    bits: u32,
}

impl RsmVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x000000b8 }
    }

    /// Read **DBG** `[7:7]`
    ///
    /// If 1, RSM can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        (self.bits & Rsm::DBG_MASK) != 0
    }

    /// Read **DMA** `[6:6]`
    ///
    /// If 1, RSM can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        (self.bits & Rsm::DMA_MASK) != 0
    }

    /// Read **CORE1** `[5:5]`
    ///
    /// If 1, RSM can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        (self.bits & Rsm::CORE1_MASK) != 0
    }

    /// Read **CORE0** `[4:4]`
    ///
    /// If 1, RSM can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        (self.bits & Rsm::CORE0_MASK) != 0
    }

    /// Read **SP** `[3:3]`
    ///
    /// If 1, RSM can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        (self.bits & Rsm::SP_MASK) != 0
    }

    /// Read **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, RSM can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn su(&self) -> bool {
        (self.bits & Rsm::SU_MASK) != 0
    }

    /// Read **NSP** `[1:1]`
    ///
    /// If 1, RSM can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        (self.bits & Rsm::NSP_MASK) != 0
    }

    /// Read **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, RSM can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        (self.bits & Rsm::NSU_MASK) != 0
    }

    /// Set **DBG** `[7:7]`
    ///
    /// If 1, RSM can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dbg(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Rsm::DBG_MASK
            } else {
                self.bits & !Rsm::DBG_MASK
            }
        }
    }

    /// Set **DMA** `[6:6]`
    ///
    /// If 1, RSM can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dma(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Rsm::DMA_MASK
            } else {
                self.bits & !Rsm::DMA_MASK
            }
        }
    }

    /// Set **CORE1** `[5:5]`
    ///
    /// If 1, RSM can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Rsm::CORE1_MASK
            } else {
                self.bits & !Rsm::CORE1_MASK
            }
        }
    }

    /// Set **CORE0** `[4:4]`
    ///
    /// If 1, RSM can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Rsm::CORE0_MASK
            } else {
                self.bits & !Rsm::CORE0_MASK
            }
        }
    }

    /// Set **SP** `[3:3]`
    ///
    /// If 1, RSM can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn with_sp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Rsm::SP_MASK
            } else {
                self.bits & !Rsm::SP_MASK
            }
        }
    }

    /// Set **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, RSM can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn with_su(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Rsm::SU_MASK
            } else {
                self.bits & !Rsm::SU_MASK
            }
        }
    }

    /// Set **NSP** `[1:1]`
    ///
    /// If 1, RSM can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn with_nsp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Rsm::NSP_MASK
            } else {
                self.bits & !Rsm::NSP_MASK
            }
        }
    }

    /// Set **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, RSM can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn with_nsu(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Rsm::NSU_MASK
            } else {
                self.bits & !Rsm::NSU_MASK
            }
        }
    }
}

impl Default for RsmVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Rsm {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RsmVal {
        RsmVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RsmVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RsmVal) -> RsmVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// XIP_CTRL Register
///
/// Control whether debugger, DMA, core 0 and core 1 can access XIP_CTRL, and at what security/privilege levels they can do so.
/// Defaults to Secure, Privileged processor or debug access only.
/// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
///
/// Address offset: 0x00e0
/// Reset value: 0x000000b8
///
/// ## Fields
///
/// - **DBG** `[7]` (RW)
///
///   If 1, XIP_CTRL can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **DMA** `[6]` (RW)
///
///   If 1, XIP_CTRL can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE1** `[5]` (RW)
///
///   If 1, XIP_CTRL can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE0** `[4]` (RW)
///
///   If 1, XIP_CTRL can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **SP** `[3]` (RW)
///
///   If 1, XIP_CTRL can be accessed from a Secure, Privileged context.
///
/// - **SU** `[2]` (RW)
///
///   If 1, and SP is also set, XIP_CTRL can be accessed from a Secure, Unprivileged context.
///
/// - **NSP** `[1]` (RW)
///
///   If 1, XIP_CTRL can be accessed from a Non-secure, Privileged context.
///
/// - **NSU** `[0]` (RW)
///
///   If 1, and NSP is also set, XIP_CTRL can be accessed from a Non-secure, Unprivileged context.
///   This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
#[repr(transparent)]
pub struct XipCtrl {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for XipCtrl {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XipCtrl {
    // ======== Field Constants ========

    /// **DBG**
    ///
    /// If 1, XIP_CTRL can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const DBG_OFFSET: u32 = 7;
    pub const DBG_MASK: u32 = 0x00000080;
    pub const DBG: u32 = 1 << 7;

    /// **DMA**
    ///
    /// If 1, XIP_CTRL can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const DMA_OFFSET: u32 = 6;
    pub const DMA_MASK: u32 = 0x00000040;
    pub const DMA: u32 = 1 << 6;

    /// **CORE1**
    ///
    /// If 1, XIP_CTRL can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CORE1_OFFSET: u32 = 5;
    pub const CORE1_MASK: u32 = 0x00000020;
    pub const CORE1: u32 = 1 << 5;

    /// **CORE0**
    ///
    /// If 1, XIP_CTRL can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CORE0_OFFSET: u32 = 4;
    pub const CORE0_MASK: u32 = 0x00000010;
    pub const CORE0: u32 = 1 << 4;

    /// **SP**
    ///
    /// If 1, XIP_CTRL can be accessed from a Secure, Privileged context.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SP_OFFSET: u32 = 3;
    pub const SP_MASK: u32 = 0x00000008;
    pub const SP: u32 = 1 << 3;

    /// **SU**
    ///
    /// If 1, and SP is also set, XIP_CTRL can be accessed from a Secure, Unprivileged context.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SU_OFFSET: u32 = 2;
    pub const SU_MASK: u32 = 0x00000004;
    pub const SU: u32 = 1 << 2;

    /// **NSP**
    ///
    /// If 1, XIP_CTRL can be accessed from a Non-secure, Privileged context.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const NSP_OFFSET: u32 = 1;
    pub const NSP_MASK: u32 = 0x00000002;
    pub const NSP: u32 = 1 << 1;

    /// **NSU**
    ///
    /// If 1, and NSP is also set, XIP_CTRL can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const NSU_OFFSET: u32 = 0;
    pub const NSU_MASK: u32 = 0x00000001;
    pub const NSU: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DBG** `[7]` (RW)
    ///
    /// If 1, XIP_CTRL can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dbg(&self) -> bool {
        (self.value.read_bits() & Self::DBG_MASK) != 0
    }

    /// Set DBG bit atomically (RW)
    #[inline(always)]
    pub fn set_dbg(&self) {
        self.set_bits(Self::DBG_MASK);
    }

    /// Clear DBG bit atomically (RW)
    #[inline(always)]
    pub fn clr_dbg(&self) {
        self.clr_bits(Self::DBG_MASK);
    }

    /// **DMA** `[6]` (RW)
    ///
    /// If 1, XIP_CTRL can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dma(&self) -> bool {
        (self.value.read_bits() & Self::DMA_MASK) != 0
    }

    /// Set DMA bit atomically (RW)
    #[inline(always)]
    pub fn set_dma(&self) {
        self.set_bits(Self::DMA_MASK);
    }

    /// Clear DMA bit atomically (RW)
    #[inline(always)]
    pub fn clr_dma(&self) {
        self.clr_bits(Self::DMA_MASK);
    }

    /// **CORE1** `[5]` (RW)
    ///
    /// If 1, XIP_CTRL can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core1(&self) -> bool {
        (self.value.read_bits() & Self::CORE1_MASK) != 0
    }

    /// Set CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn set_core1(&self) {
        self.set_bits(Self::CORE1_MASK);
    }

    /// Clear CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core1(&self) {
        self.clr_bits(Self::CORE1_MASK);
    }

    /// **CORE0** `[4]` (RW)
    ///
    /// If 1, XIP_CTRL can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core0(&self) -> bool {
        (self.value.read_bits() & Self::CORE0_MASK) != 0
    }

    /// Set CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn set_core0(&self) {
        self.set_bits(Self::CORE0_MASK);
    }

    /// Clear CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core0(&self) {
        self.clr_bits(Self::CORE0_MASK);
    }

    /// **SP** `[3]` (RW)
    ///
    /// If 1, XIP_CTRL can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub fn sp(&self) -> bool {
        (self.value.read_bits() & Self::SP_MASK) != 0
    }

    /// Set SP bit atomically (RW)
    #[inline(always)]
    pub fn set_sp(&self) {
        self.set_bits(Self::SP_MASK);
    }

    /// Clear SP bit atomically (RW)
    #[inline(always)]
    pub fn clr_sp(&self) {
        self.clr_bits(Self::SP_MASK);
    }

    /// **SU** `[2]` (RW)
    ///
    /// If 1, and SP is also set, XIP_CTRL can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub fn su(&self) -> bool {
        (self.value.read_bits() & Self::SU_MASK) != 0
    }

    /// Set SU bit atomically (RW)
    #[inline(always)]
    pub fn set_su(&self) {
        self.set_bits(Self::SU_MASK);
    }

    /// Clear SU bit atomically (RW)
    #[inline(always)]
    pub fn clr_su(&self) {
        self.clr_bits(Self::SU_MASK);
    }

    /// **NSP** `[1]` (RW)
    ///
    /// If 1, XIP_CTRL can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub fn nsp(&self) -> bool {
        (self.value.read_bits() & Self::NSP_MASK) != 0
    }

    /// Set NSP bit atomically (RW)
    #[inline(always)]
    pub fn set_nsp(&self) {
        self.set_bits(Self::NSP_MASK);
    }

    /// Clear NSP bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsp(&self) {
        self.clr_bits(Self::NSP_MASK);
    }

    /// **NSU** `[0]` (RW)
    ///
    /// If 1, and NSP is also set, XIP_CTRL can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub fn nsu(&self) -> bool {
        (self.value.read_bits() & Self::NSU_MASK) != 0
    }

    /// Set NSU bit atomically (RW)
    #[inline(always)]
    pub fn set_nsu(&self) {
        self.set_bits(Self::NSU_MASK);
    }

    /// Clear NSU bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsu(&self) {
        self.clr_bits(Self::NSU_MASK);
    }
}

/// XIP_CTRL value type for batch read/modify/write operations
///
/// Use `xipctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `xipctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct XipCtrlVal {
    bits: u32,
}

impl XipCtrlVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x000000b8 }
    }

    /// Read **DBG** `[7:7]`
    ///
    /// If 1, XIP_CTRL can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        (self.bits & XipCtrl::DBG_MASK) != 0
    }

    /// Read **DMA** `[6:6]`
    ///
    /// If 1, XIP_CTRL can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        (self.bits & XipCtrl::DMA_MASK) != 0
    }

    /// Read **CORE1** `[5:5]`
    ///
    /// If 1, XIP_CTRL can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        (self.bits & XipCtrl::CORE1_MASK) != 0
    }

    /// Read **CORE0** `[4:4]`
    ///
    /// If 1, XIP_CTRL can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        (self.bits & XipCtrl::CORE0_MASK) != 0
    }

    /// Read **SP** `[3:3]`
    ///
    /// If 1, XIP_CTRL can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        (self.bits & XipCtrl::SP_MASK) != 0
    }

    /// Read **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, XIP_CTRL can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn su(&self) -> bool {
        (self.bits & XipCtrl::SU_MASK) != 0
    }

    /// Read **NSP** `[1:1]`
    ///
    /// If 1, XIP_CTRL can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        (self.bits & XipCtrl::NSP_MASK) != 0
    }

    /// Read **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, XIP_CTRL can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        (self.bits & XipCtrl::NSU_MASK) != 0
    }

    /// Set **DBG** `[7:7]`
    ///
    /// If 1, XIP_CTRL can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dbg(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | XipCtrl::DBG_MASK
            } else {
                self.bits & !XipCtrl::DBG_MASK
            }
        }
    }

    /// Set **DMA** `[6:6]`
    ///
    /// If 1, XIP_CTRL can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dma(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | XipCtrl::DMA_MASK
            } else {
                self.bits & !XipCtrl::DMA_MASK
            }
        }
    }

    /// Set **CORE1** `[5:5]`
    ///
    /// If 1, XIP_CTRL can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | XipCtrl::CORE1_MASK
            } else {
                self.bits & !XipCtrl::CORE1_MASK
            }
        }
    }

    /// Set **CORE0** `[4:4]`
    ///
    /// If 1, XIP_CTRL can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | XipCtrl::CORE0_MASK
            } else {
                self.bits & !XipCtrl::CORE0_MASK
            }
        }
    }

    /// Set **SP** `[3:3]`
    ///
    /// If 1, XIP_CTRL can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn with_sp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | XipCtrl::SP_MASK
            } else {
                self.bits & !XipCtrl::SP_MASK
            }
        }
    }

    /// Set **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, XIP_CTRL can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn with_su(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | XipCtrl::SU_MASK
            } else {
                self.bits & !XipCtrl::SU_MASK
            }
        }
    }

    /// Set **NSP** `[1:1]`
    ///
    /// If 1, XIP_CTRL can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn with_nsp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | XipCtrl::NSP_MASK
            } else {
                self.bits & !XipCtrl::NSP_MASK
            }
        }
    }

    /// Set **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, XIP_CTRL can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn with_nsu(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | XipCtrl::NSU_MASK
            } else {
                self.bits & !XipCtrl::NSU_MASK
            }
        }
    }
}

impl Default for XipCtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl XipCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> XipCtrlVal {
        XipCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: XipCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(XipCtrlVal) -> XipCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// XIP_QMI Register
///
/// Control whether debugger, DMA, core 0 and core 1 can access XIP_QMI, and at what security/privilege levels they can do so.
/// Defaults to Secure, Privileged processor or debug access only.
/// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
///
/// Address offset: 0x00e4
/// Reset value: 0x000000b8
///
/// ## Fields
///
/// - **DBG** `[7]` (RW)
///
///   If 1, XIP_QMI can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **DMA** `[6]` (RW)
///
///   If 1, XIP_QMI can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE1** `[5]` (RW)
///
///   If 1, XIP_QMI can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE0** `[4]` (RW)
///
///   If 1, XIP_QMI can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **SP** `[3]` (RW)
///
///   If 1, XIP_QMI can be accessed from a Secure, Privileged context.
///
/// - **SU** `[2]` (RW)
///
///   If 1, and SP is also set, XIP_QMI can be accessed from a Secure, Unprivileged context.
///
/// - **NSP** `[1]` (RW)
///
///   If 1, XIP_QMI can be accessed from a Non-secure, Privileged context.
///
/// - **NSU** `[0]` (RW)
///
///   If 1, and NSP is also set, XIP_QMI can be accessed from a Non-secure, Unprivileged context.
///   This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
#[repr(transparent)]
pub struct XipQmi {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for XipQmi {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XipQmi {
    // ======== Field Constants ========

    /// **DBG**
    ///
    /// If 1, XIP_QMI can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const DBG_OFFSET: u32 = 7;
    pub const DBG_MASK: u32 = 0x00000080;
    pub const DBG: u32 = 1 << 7;

    /// **DMA**
    ///
    /// If 1, XIP_QMI can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const DMA_OFFSET: u32 = 6;
    pub const DMA_MASK: u32 = 0x00000040;
    pub const DMA: u32 = 1 << 6;

    /// **CORE1**
    ///
    /// If 1, XIP_QMI can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CORE1_OFFSET: u32 = 5;
    pub const CORE1_MASK: u32 = 0x00000020;
    pub const CORE1: u32 = 1 << 5;

    /// **CORE0**
    ///
    /// If 1, XIP_QMI can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CORE0_OFFSET: u32 = 4;
    pub const CORE0_MASK: u32 = 0x00000010;
    pub const CORE0: u32 = 1 << 4;

    /// **SP**
    ///
    /// If 1, XIP_QMI can be accessed from a Secure, Privileged context.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SP_OFFSET: u32 = 3;
    pub const SP_MASK: u32 = 0x00000008;
    pub const SP: u32 = 1 << 3;

    /// **SU**
    ///
    /// If 1, and SP is also set, XIP_QMI can be accessed from a Secure, Unprivileged context.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SU_OFFSET: u32 = 2;
    pub const SU_MASK: u32 = 0x00000004;
    pub const SU: u32 = 1 << 2;

    /// **NSP**
    ///
    /// If 1, XIP_QMI can be accessed from a Non-secure, Privileged context.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const NSP_OFFSET: u32 = 1;
    pub const NSP_MASK: u32 = 0x00000002;
    pub const NSP: u32 = 1 << 1;

    /// **NSU**
    ///
    /// If 1, and NSP is also set, XIP_QMI can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const NSU_OFFSET: u32 = 0;
    pub const NSU_MASK: u32 = 0x00000001;
    pub const NSU: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DBG** `[7]` (RW)
    ///
    /// If 1, XIP_QMI can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dbg(&self) -> bool {
        (self.value.read_bits() & Self::DBG_MASK) != 0
    }

    /// Set DBG bit atomically (RW)
    #[inline(always)]
    pub fn set_dbg(&self) {
        self.set_bits(Self::DBG_MASK);
    }

    /// Clear DBG bit atomically (RW)
    #[inline(always)]
    pub fn clr_dbg(&self) {
        self.clr_bits(Self::DBG_MASK);
    }

    /// **DMA** `[6]` (RW)
    ///
    /// If 1, XIP_QMI can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dma(&self) -> bool {
        (self.value.read_bits() & Self::DMA_MASK) != 0
    }

    /// Set DMA bit atomically (RW)
    #[inline(always)]
    pub fn set_dma(&self) {
        self.set_bits(Self::DMA_MASK);
    }

    /// Clear DMA bit atomically (RW)
    #[inline(always)]
    pub fn clr_dma(&self) {
        self.clr_bits(Self::DMA_MASK);
    }

    /// **CORE1** `[5]` (RW)
    ///
    /// If 1, XIP_QMI can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core1(&self) -> bool {
        (self.value.read_bits() & Self::CORE1_MASK) != 0
    }

    /// Set CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn set_core1(&self) {
        self.set_bits(Self::CORE1_MASK);
    }

    /// Clear CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core1(&self) {
        self.clr_bits(Self::CORE1_MASK);
    }

    /// **CORE0** `[4]` (RW)
    ///
    /// If 1, XIP_QMI can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core0(&self) -> bool {
        (self.value.read_bits() & Self::CORE0_MASK) != 0
    }

    /// Set CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn set_core0(&self) {
        self.set_bits(Self::CORE0_MASK);
    }

    /// Clear CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core0(&self) {
        self.clr_bits(Self::CORE0_MASK);
    }

    /// **SP** `[3]` (RW)
    ///
    /// If 1, XIP_QMI can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub fn sp(&self) -> bool {
        (self.value.read_bits() & Self::SP_MASK) != 0
    }

    /// Set SP bit atomically (RW)
    #[inline(always)]
    pub fn set_sp(&self) {
        self.set_bits(Self::SP_MASK);
    }

    /// Clear SP bit atomically (RW)
    #[inline(always)]
    pub fn clr_sp(&self) {
        self.clr_bits(Self::SP_MASK);
    }

    /// **SU** `[2]` (RW)
    ///
    /// If 1, and SP is also set, XIP_QMI can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub fn su(&self) -> bool {
        (self.value.read_bits() & Self::SU_MASK) != 0
    }

    /// Set SU bit atomically (RW)
    #[inline(always)]
    pub fn set_su(&self) {
        self.set_bits(Self::SU_MASK);
    }

    /// Clear SU bit atomically (RW)
    #[inline(always)]
    pub fn clr_su(&self) {
        self.clr_bits(Self::SU_MASK);
    }

    /// **NSP** `[1]` (RW)
    ///
    /// If 1, XIP_QMI can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub fn nsp(&self) -> bool {
        (self.value.read_bits() & Self::NSP_MASK) != 0
    }

    /// Set NSP bit atomically (RW)
    #[inline(always)]
    pub fn set_nsp(&self) {
        self.set_bits(Self::NSP_MASK);
    }

    /// Clear NSP bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsp(&self) {
        self.clr_bits(Self::NSP_MASK);
    }

    /// **NSU** `[0]` (RW)
    ///
    /// If 1, and NSP is also set, XIP_QMI can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub fn nsu(&self) -> bool {
        (self.value.read_bits() & Self::NSU_MASK) != 0
    }

    /// Set NSU bit atomically (RW)
    #[inline(always)]
    pub fn set_nsu(&self) {
        self.set_bits(Self::NSU_MASK);
    }

    /// Clear NSU bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsu(&self) {
        self.clr_bits(Self::NSU_MASK);
    }
}

/// XIP_QMI value type for batch read/modify/write operations
///
/// Use `xipqmi.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `xipqmi.write_val(val)`.
#[derive(Clone, Copy)]
pub struct XipQmiVal {
    bits: u32,
}

impl XipQmiVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x000000b8 }
    }

    /// Read **DBG** `[7:7]`
    ///
    /// If 1, XIP_QMI can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        (self.bits & XipQmi::DBG_MASK) != 0
    }

    /// Read **DMA** `[6:6]`
    ///
    /// If 1, XIP_QMI can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        (self.bits & XipQmi::DMA_MASK) != 0
    }

    /// Read **CORE1** `[5:5]`
    ///
    /// If 1, XIP_QMI can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        (self.bits & XipQmi::CORE1_MASK) != 0
    }

    /// Read **CORE0** `[4:4]`
    ///
    /// If 1, XIP_QMI can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        (self.bits & XipQmi::CORE0_MASK) != 0
    }

    /// Read **SP** `[3:3]`
    ///
    /// If 1, XIP_QMI can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        (self.bits & XipQmi::SP_MASK) != 0
    }

    /// Read **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, XIP_QMI can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn su(&self) -> bool {
        (self.bits & XipQmi::SU_MASK) != 0
    }

    /// Read **NSP** `[1:1]`
    ///
    /// If 1, XIP_QMI can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        (self.bits & XipQmi::NSP_MASK) != 0
    }

    /// Read **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, XIP_QMI can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        (self.bits & XipQmi::NSU_MASK) != 0
    }

    /// Set **DBG** `[7:7]`
    ///
    /// If 1, XIP_QMI can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dbg(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | XipQmi::DBG_MASK
            } else {
                self.bits & !XipQmi::DBG_MASK
            }
        }
    }

    /// Set **DMA** `[6:6]`
    ///
    /// If 1, XIP_QMI can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dma(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | XipQmi::DMA_MASK
            } else {
                self.bits & !XipQmi::DMA_MASK
            }
        }
    }

    /// Set **CORE1** `[5:5]`
    ///
    /// If 1, XIP_QMI can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | XipQmi::CORE1_MASK
            } else {
                self.bits & !XipQmi::CORE1_MASK
            }
        }
    }

    /// Set **CORE0** `[4:4]`
    ///
    /// If 1, XIP_QMI can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | XipQmi::CORE0_MASK
            } else {
                self.bits & !XipQmi::CORE0_MASK
            }
        }
    }

    /// Set **SP** `[3:3]`
    ///
    /// If 1, XIP_QMI can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn with_sp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | XipQmi::SP_MASK
            } else {
                self.bits & !XipQmi::SP_MASK
            }
        }
    }

    /// Set **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, XIP_QMI can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn with_su(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | XipQmi::SU_MASK
            } else {
                self.bits & !XipQmi::SU_MASK
            }
        }
    }

    /// Set **NSP** `[1:1]`
    ///
    /// If 1, XIP_QMI can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn with_nsp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | XipQmi::NSP_MASK
            } else {
                self.bits & !XipQmi::NSP_MASK
            }
        }
    }

    /// Set **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, XIP_QMI can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn with_nsu(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | XipQmi::NSU_MASK
            } else {
                self.bits & !XipQmi::NSU_MASK
            }
        }
    }
}

impl Default for XipQmiVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl XipQmi {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> XipQmiVal {
        XipQmiVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: XipQmiVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(XipQmiVal) -> XipQmiVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// XIP_AUX Register
///
/// Control whether debugger, DMA, core 0 and core 1 can access XIP_AUX, and at what security/privilege levels they can do so.
/// Defaults to Secure, Privileged access only.
/// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
///
/// Address offset: 0x00e8
/// Reset value: 0x000000f8
///
/// ## Fields
///
/// - **DBG** `[7]` (RW)
///
///   If 1, XIP_AUX can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **DMA** `[6]` (RW)
///
///   If 1, XIP_AUX can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE1** `[5]` (RW)
///
///   If 1, XIP_AUX can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **CORE0** `[4]` (RW)
///
///   If 1, XIP_AUX can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
///
/// - **SP** `[3]` (RW)
///
///   If 1, XIP_AUX can be accessed from a Secure, Privileged context.
///
/// - **SU** `[2]` (RW)
///
///   If 1, and SP is also set, XIP_AUX can be accessed from a Secure, Unprivileged context.
///
/// - **NSP** `[1]` (RW)
///
///   If 1, XIP_AUX can be accessed from a Non-secure, Privileged context.
///
/// - **NSU** `[0]` (RW)
///
///   If 1, and NSP is also set, XIP_AUX can be accessed from a Non-secure, Unprivileged context.
///   This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
#[repr(transparent)]
pub struct XipAux {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for XipAux {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XipAux {
    // ======== Field Constants ========

    /// **DBG**
    ///
    /// If 1, XIP_AUX can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const DBG_OFFSET: u32 = 7;
    pub const DBG_MASK: u32 = 0x00000080;
    pub const DBG: u32 = 1 << 7;

    /// **DMA**
    ///
    /// If 1, XIP_AUX can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const DMA_OFFSET: u32 = 6;
    pub const DMA_MASK: u32 = 0x00000040;
    pub const DMA: u32 = 1 << 6;

    /// **CORE1**
    ///
    /// If 1, XIP_AUX can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CORE1_OFFSET: u32 = 5;
    pub const CORE1_MASK: u32 = 0x00000020;
    pub const CORE1: u32 = 1 << 5;

    /// **CORE0**
    ///
    /// If 1, XIP_AUX can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CORE0_OFFSET: u32 = 4;
    pub const CORE0_MASK: u32 = 0x00000010;
    pub const CORE0: u32 = 1 << 4;

    /// **SP**
    ///
    /// If 1, XIP_AUX can be accessed from a Secure, Privileged context.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SP_OFFSET: u32 = 3;
    pub const SP_MASK: u32 = 0x00000008;
    pub const SP: u32 = 1 << 3;

    /// **SU**
    ///
    /// If 1, and SP is also set, XIP_AUX can be accessed from a Secure, Unprivileged context.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SU_OFFSET: u32 = 2;
    pub const SU_MASK: u32 = 0x00000004;
    pub const SU: u32 = 1 << 2;

    /// **NSP**
    ///
    /// If 1, XIP_AUX can be accessed from a Non-secure, Privileged context.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const NSP_OFFSET: u32 = 1;
    pub const NSP_MASK: u32 = 0x00000002;
    pub const NSP: u32 = 1 << 1;

    /// **NSU**
    ///
    /// If 1, and NSP is also set, XIP_AUX can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const NSU_OFFSET: u32 = 0;
    pub const NSU_MASK: u32 = 0x00000001;
    pub const NSU: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DBG** `[7]` (RW)
    ///
    /// If 1, XIP_AUX can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dbg(&self) -> bool {
        (self.value.read_bits() & Self::DBG_MASK) != 0
    }

    /// Set DBG bit atomically (RW)
    #[inline(always)]
    pub fn set_dbg(&self) {
        self.set_bits(Self::DBG_MASK);
    }

    /// Clear DBG bit atomically (RW)
    #[inline(always)]
    pub fn clr_dbg(&self) {
        self.clr_bits(Self::DBG_MASK);
    }

    /// **DMA** `[6]` (RW)
    ///
    /// If 1, XIP_AUX can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn dma(&self) -> bool {
        (self.value.read_bits() & Self::DMA_MASK) != 0
    }

    /// Set DMA bit atomically (RW)
    #[inline(always)]
    pub fn set_dma(&self) {
        self.set_bits(Self::DMA_MASK);
    }

    /// Clear DMA bit atomically (RW)
    #[inline(always)]
    pub fn clr_dma(&self) {
        self.clr_bits(Self::DMA_MASK);
    }

    /// **CORE1** `[5]` (RW)
    ///
    /// If 1, XIP_AUX can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core1(&self) -> bool {
        (self.value.read_bits() & Self::CORE1_MASK) != 0
    }

    /// Set CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn set_core1(&self) {
        self.set_bits(Self::CORE1_MASK);
    }

    /// Clear CORE1 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core1(&self) {
        self.clr_bits(Self::CORE1_MASK);
    }

    /// **CORE0** `[4]` (RW)
    ///
    /// If 1, XIP_AUX can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub fn core0(&self) -> bool {
        (self.value.read_bits() & Self::CORE0_MASK) != 0
    }

    /// Set CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn set_core0(&self) {
        self.set_bits(Self::CORE0_MASK);
    }

    /// Clear CORE0 bit atomically (RW)
    #[inline(always)]
    pub fn clr_core0(&self) {
        self.clr_bits(Self::CORE0_MASK);
    }

    /// **SP** `[3]` (RW)
    ///
    /// If 1, XIP_AUX can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub fn sp(&self) -> bool {
        (self.value.read_bits() & Self::SP_MASK) != 0
    }

    /// Set SP bit atomically (RW)
    #[inline(always)]
    pub fn set_sp(&self) {
        self.set_bits(Self::SP_MASK);
    }

    /// Clear SP bit atomically (RW)
    #[inline(always)]
    pub fn clr_sp(&self) {
        self.clr_bits(Self::SP_MASK);
    }

    /// **SU** `[2]` (RW)
    ///
    /// If 1, and SP is also set, XIP_AUX can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub fn su(&self) -> bool {
        (self.value.read_bits() & Self::SU_MASK) != 0
    }

    /// Set SU bit atomically (RW)
    #[inline(always)]
    pub fn set_su(&self) {
        self.set_bits(Self::SU_MASK);
    }

    /// Clear SU bit atomically (RW)
    #[inline(always)]
    pub fn clr_su(&self) {
        self.clr_bits(Self::SU_MASK);
    }

    /// **NSP** `[1]` (RW)
    ///
    /// If 1, XIP_AUX can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub fn nsp(&self) -> bool {
        (self.value.read_bits() & Self::NSP_MASK) != 0
    }

    /// Set NSP bit atomically (RW)
    #[inline(always)]
    pub fn set_nsp(&self) {
        self.set_bits(Self::NSP_MASK);
    }

    /// Clear NSP bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsp(&self) {
        self.clr_bits(Self::NSP_MASK);
    }

    /// **NSU** `[0]` (RW)
    ///
    /// If 1, and NSP is also set, XIP_AUX can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub fn nsu(&self) -> bool {
        (self.value.read_bits() & Self::NSU_MASK) != 0
    }

    /// Set NSU bit atomically (RW)
    #[inline(always)]
    pub fn set_nsu(&self) {
        self.set_bits(Self::NSU_MASK);
    }

    /// Clear NSU bit atomically (RW)
    #[inline(always)]
    pub fn clr_nsu(&self) {
        self.clr_bits(Self::NSU_MASK);
    }
}

/// XIP_AUX value type for batch read/modify/write operations
///
/// Use `xipaux.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `xipaux.write_val(val)`.
#[derive(Clone, Copy)]
pub struct XipAuxVal {
    bits: u32,
}

impl XipAuxVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x000000f8 }
    }

    /// Read **DBG** `[7:7]`
    ///
    /// If 1, XIP_AUX can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        (self.bits & XipAux::DBG_MASK) != 0
    }

    /// Read **DMA** `[6:6]`
    ///
    /// If 1, XIP_AUX can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        (self.bits & XipAux::DMA_MASK) != 0
    }

    /// Read **CORE1** `[5:5]`
    ///
    /// If 1, XIP_AUX can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        (self.bits & XipAux::CORE1_MASK) != 0
    }

    /// Read **CORE0** `[4:4]`
    ///
    /// If 1, XIP_AUX can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        (self.bits & XipAux::CORE0_MASK) != 0
    }

    /// Read **SP** `[3:3]`
    ///
    /// If 1, XIP_AUX can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        (self.bits & XipAux::SP_MASK) != 0
    }

    /// Read **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, XIP_AUX can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn su(&self) -> bool {
        (self.bits & XipAux::SU_MASK) != 0
    }

    /// Read **NSP** `[1:1]`
    ///
    /// If 1, XIP_AUX can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        (self.bits & XipAux::NSP_MASK) != 0
    }

    /// Read **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, XIP_AUX can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        (self.bits & XipAux::NSU_MASK) != 0
    }

    /// Set **DBG** `[7:7]`
    ///
    /// If 1, XIP_AUX can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dbg(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | XipAux::DBG_MASK
            } else {
                self.bits & !XipAux::DBG_MASK
            }
        }
    }

    /// Set **DMA** `[6:6]`
    ///
    /// If 1, XIP_AUX can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_dma(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | XipAux::DMA_MASK
            } else {
                self.bits & !XipAux::DMA_MASK
            }
        }
    }

    /// Set **CORE1** `[5:5]`
    ///
    /// If 1, XIP_AUX can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | XipAux::CORE1_MASK
            } else {
                self.bits & !XipAux::CORE1_MASK
            }
        }
    }

    /// Set **CORE0** `[4:4]`
    ///
    /// If 1, XIP_AUX can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register.
    #[inline(always)]
    pub const fn with_core0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | XipAux::CORE0_MASK
            } else {
                self.bits & !XipAux::CORE0_MASK
            }
        }
    }

    /// Set **SP** `[3:3]`
    ///
    /// If 1, XIP_AUX can be accessed from a Secure, Privileged context.
    #[inline(always)]
    pub const fn with_sp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | XipAux::SP_MASK
            } else {
                self.bits & !XipAux::SP_MASK
            }
        }
    }

    /// Set **SU** `[2:2]`
    ///
    /// If 1, and SP is also set, XIP_AUX can be accessed from a Secure, Unprivileged context.
    #[inline(always)]
    pub const fn with_su(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | XipAux::SU_MASK
            } else {
                self.bits & !XipAux::SU_MASK
            }
        }
    }

    /// Set **NSP** `[1:1]`
    ///
    /// If 1, XIP_AUX can be accessed from a Non-secure, Privileged context.
    #[inline(always)]
    pub const fn with_nsp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | XipAux::NSP_MASK
            } else {
                self.bits & !XipAux::NSP_MASK
            }
        }
    }

    /// Set **NSU** `[0:0]`
    ///
    /// If 1, and NSP is also set, XIP_AUX can be accessed from a Non-secure, Unprivileged context.
    /// This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set.
    #[inline(always)]
    pub const fn with_nsu(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | XipAux::NSU_MASK
            } else {
                self.bits & !XipAux::NSU_MASK
            }
        }
    }
}

impl Default for XipAuxVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl XipAux {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> XipAuxVal {
        XipAuxVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: XipAuxVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(XipAuxVal) -> XipAuxVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// Once a LOCK bit is written to 1, ACCESSCTRL silently ignores writes from that master. LOCK is writable only by a Secure, Privileged processor or debugger.
    /// LOCK bits are only writable when their value is zero. Once set, they can never be cleared, except by a full reset of ACCESSCTRL
    /// Setting the LOCK bit does not affect whether an access raises a bus error. Unprivileged writes, or writes from the DMA, will continue to raise bus errors. All other accesses will continue not to.
    pub lock: Lock,
    /// 0x0004
    /// Force core 1's bus accesses to always be Non-secure, no matter the core's internal state.
    /// Useful for schemes where one core is designated as the Non-secure core, since some peripherals may filter individual registers internally based on security state but not on master ID.
    pub force_core_ns: ForceCoreNs,
    /// 0x0008
    /// Write 1 to reset all ACCESSCTRL configuration, except for the LOCK and FORCE_CORE_NS registers.
    /// This bit is used in the RP2350 bootrom to quickly restore ACCESSCTRL to a known state during the boot path.
    /// Note that, like all registers in ACCESSCTRL, this register is not writable when the writer's corresponding LOCK bit is set, therefore a master which has been locked out of ACCESSCTRL can not use the CFGRESET register to disturb its contents.
    pub cfgreset: Cfgreset,
    /// 0x000c
    /// Control whether GPIO0...31 are accessible to Non-secure code. Writable only by a Secure, Privileged processor or debugger.
    /// 0 -> Secure access only
    /// 1 -> Secure + Non-secure access
    pub gpio_nsmask0: GpioNsmask0,
    /// 0x0010
    /// Control whether GPIO32..47 are accessible to Non-secure code, and whether QSPI and USB bitbang are accessible through the Non-secure SIO. Writable only by a Secure, Privileged processor or debugger.
    pub gpio_nsmask1: GpioNsmask1,
    /// 0x0014
    /// Control whether debugger, DMA, core 0 and core 1 can access ROM, and at what security/privilege levels they can do so.
    /// Defaults to fully open access.
    /// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
    pub rom: Rom,
    /// 0x0018
    /// Control whether debugger, DMA, core 0 and core 1 can access XIP_MAIN, and at what security/privilege levels they can do so.
    /// Defaults to fully open access.
    /// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
    pub xip_main: XipMain,
    /// 0x001c
    /// Control whether debugger, DMA, core 0 and core 1 can access SRAM0, and at what security/privilege levels they can do so.
    /// Defaults to fully open access.
    /// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
    pub sram0: Sram0,
    /// 0x0020
    /// Control whether debugger, DMA, core 0 and core 1 can access SRAM1, and at what security/privilege levels they can do so.
    /// Defaults to fully open access.
    /// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
    pub sram1: Sram1,
    /// 0x0024
    /// Control whether debugger, DMA, core 0 and core 1 can access SRAM2, and at what security/privilege levels they can do so.
    /// Defaults to fully open access.
    /// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
    pub sram2: Sram2,
    /// 0x0028
    /// Control whether debugger, DMA, core 0 and core 1 can access SRAM3, and at what security/privilege levels they can do so.
    /// Defaults to fully open access.
    /// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
    pub sram3: Sram3,
    /// 0x002c
    /// Control whether debugger, DMA, core 0 and core 1 can access SRAM4, and at what security/privilege levels they can do so.
    /// Defaults to fully open access.
    /// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
    pub sram4: Sram4,
    /// 0x0030
    /// Control whether debugger, DMA, core 0 and core 1 can access SRAM5, and at what security/privilege levels they can do so.
    /// Defaults to fully open access.
    /// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
    pub sram5: Sram5,
    /// 0x0034
    /// Control whether debugger, DMA, core 0 and core 1 can access SRAM6, and at what security/privilege levels they can do so.
    /// Defaults to fully open access.
    /// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
    pub sram6: Sram6,
    /// 0x0038
    /// Control whether debugger, DMA, core 0 and core 1 can access SRAM7, and at what security/privilege levels they can do so.
    /// Defaults to fully open access.
    /// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
    pub sram7: Sram7,
    /// 0x003c
    /// Control whether debugger, DMA, core 0 and core 1 can access SRAM8, and at what security/privilege levels they can do so.
    /// Defaults to fully open access.
    /// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
    pub sram8: Sram8,
    /// 0x0040
    /// Control whether debugger, DMA, core 0 and core 1 can access SRAM9, and at what security/privilege levels they can do so.
    /// Defaults to fully open access.
    /// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
    pub sram9: Sram9,
    /// 0x0044
    /// Control whether debugger, DMA, core 0 and core 1 can access DMA, and at what security/privilege levels they can do so.
    /// Defaults to Secure access from any master.
    /// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
    pub dma: Dma,
    /// 0x0048
    /// Control whether debugger, DMA, core 0 and core 1 can access USBCTRL, and at what security/privilege levels they can do so.
    /// Defaults to Secure access from any master.
    /// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
    pub usbctrl: Usbctrl,
    /// 0x004c
    /// Control whether debugger, DMA, core 0 and core 1 can access PIO0, and at what security/privilege levels they can do so.
    /// Defaults to Secure access from any master.
    /// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
    pub pio0: Pio0,
    /// 0x0050
    /// Control whether debugger, DMA, core 0 and core 1 can access PIO1, and at what security/privilege levels they can do so.
    /// Defaults to Secure access from any master.
    /// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
    pub pio1: Pio1,
    /// 0x0054
    /// Control whether debugger, DMA, core 0 and core 1 can access PIO2, and at what security/privilege levels they can do so.
    /// Defaults to Secure access from any master.
    /// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
    pub pio2: Pio2,
    /// 0x0058
    /// Control whether debugger, DMA, core 0 and core 1 can access CORESIGHT_TRACE, and at what security/privilege levels they can do so.
    /// Defaults to Secure, Privileged processor or debug access only.
    /// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
    pub coresight_trace: CoresightTrace,
    /// 0x005c
    /// Control whether debugger, DMA, core 0 and core 1 can access CORESIGHT_PERIPH, and at what security/privilege levels they can do so.
    /// Defaults to Secure, Privileged processor or debug access only.
    /// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
    pub coresight_periph: CoresightPeriph,
    /// 0x0060
    /// Control whether debugger, DMA, core 0 and core 1 can access SYSINFO, and at what security/privilege levels they can do so.
    /// Defaults to fully open access.
    /// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
    pub sysinfo: Sysinfo,
    /// 0x0064
    /// Control whether debugger, DMA, core 0 and core 1 can access RESETS, and at what security/privilege levels they can do so.
    /// Defaults to Secure access from any master.
    /// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
    pub resets: Resets,
    /// 0x0068
    /// Control whether debugger, DMA, core 0 and core 1 can access IO_BANK0, and at what security/privilege levels they can do so.
    /// Defaults to Secure access from any master.
    /// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
    pub io_bank0: IoBank0,
    /// 0x006c
    /// Control whether debugger, DMA, core 0 and core 1 can access IO_BANK1, and at what security/privilege levels they can do so.
    /// Defaults to Secure access from any master.
    /// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
    pub io_bank1: IoBank1,
    /// 0x0070
    /// Control whether debugger, DMA, core 0 and core 1 can access PADS_BANK0, and at what security/privilege levels they can do so.
    /// Defaults to Secure access from any master.
    /// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
    pub pads_bank0: PadsBank0,
    /// 0x0074
    /// Control whether debugger, DMA, core 0 and core 1 can access PADS_QSPI, and at what security/privilege levels they can do so.
    /// Defaults to Secure access from any master.
    /// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
    pub pads_qspi: PadsQspi,
    /// 0x0078
    /// Control whether debugger, DMA, core 0 and core 1 can access BUSCTRL, and at what security/privilege levels they can do so.
    /// Defaults to Secure access from any master.
    /// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
    pub busctrl: Busctrl,
    /// 0x007c
    /// Control whether debugger, DMA, core 0 and core 1 can access ADC0, and at what security/privilege levels they can do so.
    /// Defaults to Secure access from any master.
    /// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
    pub adc0: Adc0,
    /// 0x0080
    /// Control whether debugger, DMA, core 0 and core 1 can access HSTX, and at what security/privilege levels they can do so.
    /// Defaults to Secure access from any master.
    /// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
    pub hstx: Hstx,
    /// 0x0084
    /// Control whether debugger, DMA, core 0 and core 1 can access I2C0, and at what security/privilege levels they can do so.
    /// Defaults to Secure access from any master.
    /// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
    pub i2c0: I2c0,
    /// 0x0088
    /// Control whether debugger, DMA, core 0 and core 1 can access I2C1, and at what security/privilege levels they can do so.
    /// Defaults to Secure access from any master.
    /// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
    pub i2c1: I2c1,
    /// 0x008c
    /// Control whether debugger, DMA, core 0 and core 1 can access PWM, and at what security/privilege levels they can do so.
    /// Defaults to Secure access from any master.
    /// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
    pub pwm: Pwm,
    /// 0x0090
    /// Control whether debugger, DMA, core 0 and core 1 can access SPI0, and at what security/privilege levels they can do so.
    /// Defaults to Secure access from any master.
    /// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
    pub spi0: Spi0,
    /// 0x0094
    /// Control whether debugger, DMA, core 0 and core 1 can access SPI1, and at what security/privilege levels they can do so.
    /// Defaults to Secure access from any master.
    /// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
    pub spi1: Spi1,
    /// 0x0098
    /// Control whether debugger, DMA, core 0 and core 1 can access TIMER0, and at what security/privilege levels they can do so.
    /// Defaults to Secure access from any master.
    /// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
    pub timer0: Timer0,
    /// 0x009c
    /// Control whether debugger, DMA, core 0 and core 1 can access TIMER1, and at what security/privilege levels they can do so.
    /// Defaults to Secure access from any master.
    /// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
    pub timer1: Timer1,
    /// 0x00a0
    /// Control whether debugger, DMA, core 0 and core 1 can access UART0, and at what security/privilege levels they can do so.
    /// Defaults to Secure access from any master.
    /// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
    pub uart0: Uart0,
    /// 0x00a4
    /// Control whether debugger, DMA, core 0 and core 1 can access UART1, and at what security/privilege levels they can do so.
    /// Defaults to Secure access from any master.
    /// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
    pub uart1: Uart1,
    /// 0x00a8
    /// Control whether debugger, DMA, core 0 and core 1 can access OTP, and at what security/privilege levels they can do so.
    /// Defaults to Secure access from any master.
    /// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
    pub otp: Otp,
    /// 0x00ac
    /// Control whether debugger, DMA, core 0 and core 1 can access TBMAN, and at what security/privilege levels they can do so.
    /// Defaults to Secure access from any master.
    /// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
    pub tbman: Tbman,
    /// 0x00b0
    /// Control whether debugger, DMA, core 0 and core 1 can access POWMAN, and at what security/privilege levels they can do so.
    /// Defaults to Secure, Privileged processor or debug access only.
    /// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
    pub powman: Powman,
    /// 0x00b4
    /// Control whether debugger, DMA, core 0 and core 1 can access TRNG, and at what security/privilege levels they can do so.
    /// Defaults to Secure, Privileged processor or debug access only.
    /// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
    pub trng: Trng,
    /// 0x00b8
    /// Control whether debugger, DMA, core 0 and core 1 can access SHA256, and at what security/privilege levels they can do so.
    /// Defaults to Secure, Privileged access only.
    /// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
    pub sha256: Sha256,
    /// 0x00bc
    /// Control whether debugger, DMA, core 0 and core 1 can access SYSCFG, and at what security/privilege levels they can do so.
    /// Defaults to Secure, Privileged processor or debug access only.
    /// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
    pub syscfg: Syscfg,
    /// 0x00c0
    /// Control whether debugger, DMA, core 0 and core 1 can access CLOCKS, and at what security/privilege levels they can do so.
    /// Defaults to Secure, Privileged processor or debug access only.
    /// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
    pub clocks: Clocks,
    /// 0x00c4
    /// Control whether debugger, DMA, core 0 and core 1 can access XOSC, and at what security/privilege levels they can do so.
    /// Defaults to Secure, Privileged processor or debug access only.
    /// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
    pub xosc: Xosc,
    /// 0x00c8
    /// Control whether debugger, DMA, core 0 and core 1 can access ROSC, and at what security/privilege levels they can do so.
    /// Defaults to Secure, Privileged processor or debug access only.
    /// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
    pub rosc: Rosc,
    /// 0x00cc
    /// Control whether debugger, DMA, core 0 and core 1 can access PLL_SYS, and at what security/privilege levels they can do so.
    /// Defaults to Secure, Privileged processor or debug access only.
    /// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
    pub pll_sys: PllSys,
    /// 0x00d0
    /// Control whether debugger, DMA, core 0 and core 1 can access PLL_USB, and at what security/privilege levels they can do so.
    /// Defaults to Secure, Privileged processor or debug access only.
    /// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
    pub pll_usb: PllUsb,
    /// 0x00d4
    /// Control whether debugger, DMA, core 0 and core 1 can access TICKS, and at what security/privilege levels they can do so.
    /// Defaults to Secure, Privileged processor or debug access only.
    /// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
    pub ticks: Ticks,
    /// 0x00d8
    /// Control whether debugger, DMA, core 0 and core 1 can access WATCHDOG, and at what security/privilege levels they can do so.
    /// Defaults to Secure, Privileged processor or debug access only.
    /// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
    pub watchdog: Watchdog,
    /// 0x00dc
    /// Control whether debugger, DMA, core 0 and core 1 can access RSM, and at what security/privilege levels they can do so.
    /// Defaults to Secure, Privileged processor or debug access only.
    /// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
    pub rsm: Rsm,
    /// 0x00e0
    /// Control whether debugger, DMA, core 0 and core 1 can access XIP_CTRL, and at what security/privilege levels they can do so.
    /// Defaults to Secure, Privileged processor or debug access only.
    /// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
    pub xip_ctrl: XipCtrl,
    /// 0x00e4
    /// Control whether debugger, DMA, core 0 and core 1 can access XIP_QMI, and at what security/privilege levels they can do so.
    /// Defaults to Secure, Privileged processor or debug access only.
    /// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
    pub xip_qmi: XipQmi,
    /// 0x00e8
    /// Control whether debugger, DMA, core 0 and core 1 can access XIP_AUX, and at what security/privilege levels they can do so.
    /// Defaults to Secure, Privileged access only.
    /// This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set.
    pub xip_aux: XipAux,
}

