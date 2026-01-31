//! OTP_DATA
//! Predefined OTP data layout for RP2350
//!
//! This peripheral supports atomic SET/CLR/XOR register aliases.

#![allow(dead_code)]

pub const BASE: u32 = 0x40130000;

/// CHIPID0 Register
///
/// Bits 15:0 of public device ID. (ECC)
/// The CHIPID0..3 rows contain a 64-bit random identifier for this chip, which can be read from the USB bootloader PICOBOOT interface or from the get_sys_info ROM API.
/// The number of random bits makes the occurrence of twins exceedingly unlikely: for example, a fleet of a hundred million devices has a 99.97% probability of no twinned IDs. This is estimated to be lower than the occurrence of process errors in the assignment of sequential random IDs, and for practical purposes CHIPID may be treated as unique.
///
/// Address offset: 0x0000
///
/// ## Fields
///
/// - **CHIPID0** `[15:0]` (RO)
#[repr(transparent)]
pub struct Chipid0 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Chipid0 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Chipid0 {
    // ======== Field Constants ========

    /// **CHIPID0**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const CHIPID0_OFFSET: u32 = 0;
    pub const CHIPID0_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **CHIPID0** `[15:0]` (RO)
    #[inline(always)]
    pub fn chipid0(&self) -> u32 {
        (self.value.read_bits() & Self::CHIPID0_MASK) >> Self::CHIPID0_OFFSET
    }

}

/// CHIPID0 value type for batch read/modify/write operations
///
/// Use `chipid0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `chipid0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Chipid0Val {
    bits: u32,
}

impl Chipid0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CHIPID0** `[15:0]`
    #[inline(always)]
    pub const fn chipid0(&self) -> u32 {
        (self.bits & Chipid0::CHIPID0_MASK) >> Chipid0::CHIPID0_OFFSET
    }

}

impl Default for Chipid0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Chipid0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Chipid0Val {
        Chipid0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Chipid0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Chipid0Val) -> Chipid0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// CHIPID1 Register
///
/// Bits 31:16 of public device ID (ECC)
///
/// Address offset: 0x0002
///
/// ## Fields
///
/// - **CHIPID1** `[15:0]` (RO)
#[repr(transparent)]
pub struct Chipid1 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Chipid1 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Chipid1 {
    // ======== Field Constants ========

    /// **CHIPID1**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const CHIPID1_OFFSET: u32 = 0;
    pub const CHIPID1_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **CHIPID1** `[15:0]` (RO)
    #[inline(always)]
    pub fn chipid1(&self) -> u32 {
        (self.value.read_bits() & Self::CHIPID1_MASK) >> Self::CHIPID1_OFFSET
    }

}

/// CHIPID1 value type for batch read/modify/write operations
///
/// Use `chipid1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `chipid1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Chipid1Val {
    bits: u32,
}

impl Chipid1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CHIPID1** `[15:0]`
    #[inline(always)]
    pub const fn chipid1(&self) -> u32 {
        (self.bits & Chipid1::CHIPID1_MASK) >> Chipid1::CHIPID1_OFFSET
    }

}

impl Default for Chipid1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Chipid1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Chipid1Val {
        Chipid1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Chipid1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Chipid1Val) -> Chipid1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// CHIPID2 Register
///
/// Bits 47:32 of public device ID (ECC)
///
/// Address offset: 0x0004
///
/// ## Fields
///
/// - **CHIPID2** `[15:0]` (RO)
#[repr(transparent)]
pub struct Chipid2 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Chipid2 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Chipid2 {
    // ======== Field Constants ========

    /// **CHIPID2**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const CHIPID2_OFFSET: u32 = 0;
    pub const CHIPID2_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **CHIPID2** `[15:0]` (RO)
    #[inline(always)]
    pub fn chipid2(&self) -> u32 {
        (self.value.read_bits() & Self::CHIPID2_MASK) >> Self::CHIPID2_OFFSET
    }

}

/// CHIPID2 value type for batch read/modify/write operations
///
/// Use `chipid2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `chipid2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Chipid2Val {
    bits: u32,
}

impl Chipid2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CHIPID2** `[15:0]`
    #[inline(always)]
    pub const fn chipid2(&self) -> u32 {
        (self.bits & Chipid2::CHIPID2_MASK) >> Chipid2::CHIPID2_OFFSET
    }

}

impl Default for Chipid2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Chipid2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Chipid2Val {
        Chipid2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Chipid2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Chipid2Val) -> Chipid2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// CHIPID3 Register
///
/// Bits 63:48 of public device ID (ECC)
///
/// Address offset: 0x0006
///
/// ## Fields
///
/// - **CHIPID3** `[15:0]` (RO)
#[repr(transparent)]
pub struct Chipid3 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Chipid3 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Chipid3 {
    // ======== Field Constants ========

    /// **CHIPID3**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const CHIPID3_OFFSET: u32 = 0;
    pub const CHIPID3_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **CHIPID3** `[15:0]` (RO)
    #[inline(always)]
    pub fn chipid3(&self) -> u32 {
        (self.value.read_bits() & Self::CHIPID3_MASK) >> Self::CHIPID3_OFFSET
    }

}

/// CHIPID3 value type for batch read/modify/write operations
///
/// Use `chipid3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `chipid3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Chipid3Val {
    bits: u32,
}

impl Chipid3Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CHIPID3** `[15:0]`
    #[inline(always)]
    pub const fn chipid3(&self) -> u32 {
        (self.bits & Chipid3::CHIPID3_MASK) >> Chipid3::CHIPID3_OFFSET
    }

}

impl Default for Chipid3Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Chipid3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Chipid3Val {
        Chipid3Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Chipid3Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Chipid3Val) -> Chipid3Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// RANDID0 Register
///
/// Bits 15:0 of private per-device random number (ECC)
/// The RANDID0..7 rows form a 128-bit random number generated during device test.
/// This ID is not exposed through the USB PICOBOOT GET_INFO command or the ROM `get_sys_info()` API. However note that the USB PICOBOOT OTP access point can read the entirety of page 0, so this value is not meaningfully private unless the USB PICOBOOT interface is disabled via the DISABLE_BOOTSEL_USB_PICOBOOT_IFC flag in BOOT_FLAGS0.
///
/// Address offset: 0x0008
///
/// ## Fields
///
/// - **RANDID0** `[15:0]` (RO)
#[repr(transparent)]
pub struct Randid0 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Randid0 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Randid0 {
    // ======== Field Constants ========

    /// **RANDID0**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const RANDID0_OFFSET: u32 = 0;
    pub const RANDID0_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **RANDID0** `[15:0]` (RO)
    #[inline(always)]
    pub fn randid0(&self) -> u32 {
        (self.value.read_bits() & Self::RANDID0_MASK) >> Self::RANDID0_OFFSET
    }

}

/// RANDID0 value type for batch read/modify/write operations
///
/// Use `randid0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `randid0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Randid0Val {
    bits: u32,
}

impl Randid0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **RANDID0** `[15:0]`
    #[inline(always)]
    pub const fn randid0(&self) -> u32 {
        (self.bits & Randid0::RANDID0_MASK) >> Randid0::RANDID0_OFFSET
    }

}

impl Default for Randid0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Randid0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Randid0Val {
        Randid0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Randid0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Randid0Val) -> Randid0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// RANDID1 Register
///
/// Bits 31:16 of private per-device random number (ECC)
///
/// Address offset: 0x000a
///
/// ## Fields
///
/// - **RANDID1** `[15:0]` (RO)
#[repr(transparent)]
pub struct Randid1 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Randid1 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Randid1 {
    // ======== Field Constants ========

    /// **RANDID1**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const RANDID1_OFFSET: u32 = 0;
    pub const RANDID1_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **RANDID1** `[15:0]` (RO)
    #[inline(always)]
    pub fn randid1(&self) -> u32 {
        (self.value.read_bits() & Self::RANDID1_MASK) >> Self::RANDID1_OFFSET
    }

}

/// RANDID1 value type for batch read/modify/write operations
///
/// Use `randid1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `randid1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Randid1Val {
    bits: u32,
}

impl Randid1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **RANDID1** `[15:0]`
    #[inline(always)]
    pub const fn randid1(&self) -> u32 {
        (self.bits & Randid1::RANDID1_MASK) >> Randid1::RANDID1_OFFSET
    }

}

impl Default for Randid1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Randid1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Randid1Val {
        Randid1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Randid1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Randid1Val) -> Randid1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// RANDID2 Register
///
/// Bits 47:32 of private per-device random number (ECC)
///
/// Address offset: 0x000c
///
/// ## Fields
///
/// - **RANDID2** `[15:0]` (RO)
#[repr(transparent)]
pub struct Randid2 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Randid2 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Randid2 {
    // ======== Field Constants ========

    /// **RANDID2**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const RANDID2_OFFSET: u32 = 0;
    pub const RANDID2_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **RANDID2** `[15:0]` (RO)
    #[inline(always)]
    pub fn randid2(&self) -> u32 {
        (self.value.read_bits() & Self::RANDID2_MASK) >> Self::RANDID2_OFFSET
    }

}

/// RANDID2 value type for batch read/modify/write operations
///
/// Use `randid2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `randid2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Randid2Val {
    bits: u32,
}

impl Randid2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **RANDID2** `[15:0]`
    #[inline(always)]
    pub const fn randid2(&self) -> u32 {
        (self.bits & Randid2::RANDID2_MASK) >> Randid2::RANDID2_OFFSET
    }

}

impl Default for Randid2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Randid2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Randid2Val {
        Randid2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Randid2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Randid2Val) -> Randid2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// RANDID3 Register
///
/// Bits 63:48 of private per-device random number (ECC)
///
/// Address offset: 0x000e
///
/// ## Fields
///
/// - **RANDID3** `[15:0]` (RO)
#[repr(transparent)]
pub struct Randid3 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Randid3 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Randid3 {
    // ======== Field Constants ========

    /// **RANDID3**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const RANDID3_OFFSET: u32 = 0;
    pub const RANDID3_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **RANDID3** `[15:0]` (RO)
    #[inline(always)]
    pub fn randid3(&self) -> u32 {
        (self.value.read_bits() & Self::RANDID3_MASK) >> Self::RANDID3_OFFSET
    }

}

/// RANDID3 value type for batch read/modify/write operations
///
/// Use `randid3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `randid3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Randid3Val {
    bits: u32,
}

impl Randid3Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **RANDID3** `[15:0]`
    #[inline(always)]
    pub const fn randid3(&self) -> u32 {
        (self.bits & Randid3::RANDID3_MASK) >> Randid3::RANDID3_OFFSET
    }

}

impl Default for Randid3Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Randid3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Randid3Val {
        Randid3Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Randid3Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Randid3Val) -> Randid3Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// RANDID4 Register
///
/// Bits 79:64 of private per-device random number (ECC)
///
/// Address offset: 0x0010
///
/// ## Fields
///
/// - **RANDID4** `[15:0]` (RO)
#[repr(transparent)]
pub struct Randid4 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Randid4 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Randid4 {
    // ======== Field Constants ========

    /// **RANDID4**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const RANDID4_OFFSET: u32 = 0;
    pub const RANDID4_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **RANDID4** `[15:0]` (RO)
    #[inline(always)]
    pub fn randid4(&self) -> u32 {
        (self.value.read_bits() & Self::RANDID4_MASK) >> Self::RANDID4_OFFSET
    }

}

/// RANDID4 value type for batch read/modify/write operations
///
/// Use `randid4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `randid4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Randid4Val {
    bits: u32,
}

impl Randid4Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **RANDID4** `[15:0]`
    #[inline(always)]
    pub const fn randid4(&self) -> u32 {
        (self.bits & Randid4::RANDID4_MASK) >> Randid4::RANDID4_OFFSET
    }

}

impl Default for Randid4Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Randid4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Randid4Val {
        Randid4Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Randid4Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Randid4Val) -> Randid4Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// RANDID5 Register
///
/// Bits 95:80 of private per-device random number (ECC)
///
/// Address offset: 0x0012
///
/// ## Fields
///
/// - **RANDID5** `[15:0]` (RO)
#[repr(transparent)]
pub struct Randid5 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Randid5 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Randid5 {
    // ======== Field Constants ========

    /// **RANDID5**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const RANDID5_OFFSET: u32 = 0;
    pub const RANDID5_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **RANDID5** `[15:0]` (RO)
    #[inline(always)]
    pub fn randid5(&self) -> u32 {
        (self.value.read_bits() & Self::RANDID5_MASK) >> Self::RANDID5_OFFSET
    }

}

/// RANDID5 value type for batch read/modify/write operations
///
/// Use `randid5.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `randid5.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Randid5Val {
    bits: u32,
}

impl Randid5Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **RANDID5** `[15:0]`
    #[inline(always)]
    pub const fn randid5(&self) -> u32 {
        (self.bits & Randid5::RANDID5_MASK) >> Randid5::RANDID5_OFFSET
    }

}

impl Default for Randid5Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Randid5 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Randid5Val {
        Randid5Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Randid5Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Randid5Val) -> Randid5Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// RANDID6 Register
///
/// Bits 111:96 of private per-device random number (ECC)
///
/// Address offset: 0x0014
///
/// ## Fields
///
/// - **RANDID6** `[15:0]` (RO)
#[repr(transparent)]
pub struct Randid6 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Randid6 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Randid6 {
    // ======== Field Constants ========

    /// **RANDID6**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const RANDID6_OFFSET: u32 = 0;
    pub const RANDID6_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **RANDID6** `[15:0]` (RO)
    #[inline(always)]
    pub fn randid6(&self) -> u32 {
        (self.value.read_bits() & Self::RANDID6_MASK) >> Self::RANDID6_OFFSET
    }

}

/// RANDID6 value type for batch read/modify/write operations
///
/// Use `randid6.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `randid6.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Randid6Val {
    bits: u32,
}

impl Randid6Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **RANDID6** `[15:0]`
    #[inline(always)]
    pub const fn randid6(&self) -> u32 {
        (self.bits & Randid6::RANDID6_MASK) >> Randid6::RANDID6_OFFSET
    }

}

impl Default for Randid6Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Randid6 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Randid6Val {
        Randid6Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Randid6Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Randid6Val) -> Randid6Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// RANDID7 Register
///
/// Bits 127:112 of private per-device random number (ECC)
///
/// Address offset: 0x0016
///
/// ## Fields
///
/// - **RANDID7** `[15:0]` (RO)
#[repr(transparent)]
pub struct Randid7 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Randid7 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Randid7 {
    // ======== Field Constants ========

    /// **RANDID7**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const RANDID7_OFFSET: u32 = 0;
    pub const RANDID7_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **RANDID7** `[15:0]` (RO)
    #[inline(always)]
    pub fn randid7(&self) -> u32 {
        (self.value.read_bits() & Self::RANDID7_MASK) >> Self::RANDID7_OFFSET
    }

}

/// RANDID7 value type for batch read/modify/write operations
///
/// Use `randid7.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `randid7.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Randid7Val {
    bits: u32,
}

impl Randid7Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **RANDID7** `[15:0]`
    #[inline(always)]
    pub const fn randid7(&self) -> u32 {
        (self.bits & Randid7::RANDID7_MASK) >> Randid7::RANDID7_OFFSET
    }

}

impl Default for Randid7Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Randid7 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Randid7Val {
        Randid7Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Randid7Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Randid7Val) -> Randid7Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// ROSC_CALIB Register
///
/// Ring oscillator frequency in kHz, measured during manufacturing (ECC)
/// This is measured at 1.1 V, at room temperature, with the ROSC configuration registers in their reset state.
///
/// Address offset: 0x0020
///
/// ## Fields
///
/// - **ROSC_CALIB** `[15:0]` (RO)
#[repr(transparent)]
pub struct RoscCalib {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for RoscCalib {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RoscCalib {
    // ======== Field Constants ========

    /// **ROSC_CALIB**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const ROSC_CALIB_OFFSET: u32 = 0;
    pub const ROSC_CALIB_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **ROSC_CALIB** `[15:0]` (RO)
    #[inline(always)]
    pub fn rosc_calib(&self) -> u32 {
        (self.value.read_bits() & Self::ROSC_CALIB_MASK) >> Self::ROSC_CALIB_OFFSET
    }

}

/// ROSC_CALIB value type for batch read/modify/write operations
///
/// Use `rosccalib.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rosccalib.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RoscCalibVal {
    bits: u32,
}

impl RoscCalibVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **ROSC_CALIB** `[15:0]`
    #[inline(always)]
    pub const fn rosc_calib(&self) -> u32 {
        (self.bits & RoscCalib::ROSC_CALIB_MASK) >> RoscCalib::ROSC_CALIB_OFFSET
    }

}

impl Default for RoscCalibVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RoscCalib {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RoscCalibVal {
        RoscCalibVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RoscCalibVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RoscCalibVal) -> RoscCalibVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// LPOSC_CALIB Register
///
/// Low-power oscillator frequency in Hz, measured during manufacturing (ECC)
/// This is measured at 1.1V, at room temperature, with the LPOSC trim register in its reset state.
///
/// Address offset: 0x0022
///
/// ## Fields
///
/// - **LPOSC_CALIB** `[15:0]` (RO)
#[repr(transparent)]
pub struct LposcCalib {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for LposcCalib {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LposcCalib {
    // ======== Field Constants ========

    /// **LPOSC_CALIB**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const LPOSC_CALIB_OFFSET: u32 = 0;
    pub const LPOSC_CALIB_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **LPOSC_CALIB** `[15:0]` (RO)
    #[inline(always)]
    pub fn lposc_calib(&self) -> u32 {
        (self.value.read_bits() & Self::LPOSC_CALIB_MASK) >> Self::LPOSC_CALIB_OFFSET
    }

}

/// LPOSC_CALIB value type for batch read/modify/write operations
///
/// Use `lposccalib.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lposccalib.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LposcCalibVal {
    bits: u32,
}

impl LposcCalibVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LPOSC_CALIB** `[15:0]`
    #[inline(always)]
    pub const fn lposc_calib(&self) -> u32 {
        (self.bits & LposcCalib::LPOSC_CALIB_MASK) >> LposcCalib::LPOSC_CALIB_OFFSET
    }

}

impl Default for LposcCalibVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LposcCalib {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LposcCalibVal {
        LposcCalibVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LposcCalibVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LposcCalibVal) -> LposcCalibVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// NUM_GPIOS Register
///
/// The number of main user GPIOs (bank 0). Should read 48 in the QFN80 package, and 30 in the QFN60 package. (ECC)
///
/// Address offset: 0x0030
///
/// ## Fields
///
/// - **NUM_GPIOS** `[7:0]` (RO)
#[repr(transparent)]
pub struct NumGpios {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for NumGpios {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl NumGpios {
    // ======== Field Constants ========

    /// **NUM_GPIOS**
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const NUM_GPIOS_OFFSET: u32 = 0;
    pub const NUM_GPIOS_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **NUM_GPIOS** `[7:0]` (RO)
    #[inline(always)]
    pub fn num_gpios(&self) -> u32 {
        (self.value.read_bits() & Self::NUM_GPIOS_MASK) >> Self::NUM_GPIOS_OFFSET
    }

}

/// NUM_GPIOS value type for batch read/modify/write operations
///
/// Use `numgpios.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `numgpios.write_val(val)`.
#[derive(Clone, Copy)]
pub struct NumGpiosVal {
    bits: u32,
}

impl NumGpiosVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **NUM_GPIOS** `[7:0]`
    #[inline(always)]
    pub const fn num_gpios(&self) -> u32 {
        (self.bits & NumGpios::NUM_GPIOS_MASK) >> NumGpios::NUM_GPIOS_OFFSET
    }

}

impl Default for NumGpiosVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl NumGpios {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> NumGpiosVal {
        NumGpiosVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: NumGpiosVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(NumGpiosVal) -> NumGpiosVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// INFO_CRC0 Register
///
/// Lower 16 bits of CRC32 of OTP addresses 0x00 through 0x6b (polynomial 0x4c11db7, input reflected, output reflected, seed all-ones, final XOR all-ones) (ECC)
///
/// Address offset: 0x006c
///
/// ## Fields
///
/// - **INFO_CRC0** `[15:0]` (RO)
#[repr(transparent)]
pub struct InfoCrc0 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for InfoCrc0 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoCrc0 {
    // ======== Field Constants ========

    /// **INFO_CRC0**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const INFO_CRC0_OFFSET: u32 = 0;
    pub const INFO_CRC0_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **INFO_CRC0** `[15:0]` (RO)
    #[inline(always)]
    pub fn info_crc0(&self) -> u32 {
        (self.value.read_bits() & Self::INFO_CRC0_MASK) >> Self::INFO_CRC0_OFFSET
    }

}

/// INFO_CRC0 value type for batch read/modify/write operations
///
/// Use `infocrc0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `infocrc0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InfoCrc0Val {
    bits: u32,
}

impl InfoCrc0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **INFO_CRC0** `[15:0]`
    #[inline(always)]
    pub const fn info_crc0(&self) -> u32 {
        (self.bits & InfoCrc0::INFO_CRC0_MASK) >> InfoCrc0::INFO_CRC0_OFFSET
    }

}

impl Default for InfoCrc0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl InfoCrc0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InfoCrc0Val {
        InfoCrc0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: InfoCrc0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(InfoCrc0Val) -> InfoCrc0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// INFO_CRC1 Register
///
/// Upper 16 bits of CRC32 of OTP addresses 0x00 through 0x6b (ECC)
///
/// Address offset: 0x006e
///
/// ## Fields
///
/// - **INFO_CRC1** `[15:0]` (RO)
#[repr(transparent)]
pub struct InfoCrc1 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for InfoCrc1 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoCrc1 {
    // ======== Field Constants ========

    /// **INFO_CRC1**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const INFO_CRC1_OFFSET: u32 = 0;
    pub const INFO_CRC1_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **INFO_CRC1** `[15:0]` (RO)
    #[inline(always)]
    pub fn info_crc1(&self) -> u32 {
        (self.value.read_bits() & Self::INFO_CRC1_MASK) >> Self::INFO_CRC1_OFFSET
    }

}

/// INFO_CRC1 value type for batch read/modify/write operations
///
/// Use `infocrc1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `infocrc1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InfoCrc1Val {
    bits: u32,
}

impl InfoCrc1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **INFO_CRC1** `[15:0]`
    #[inline(always)]
    pub const fn info_crc1(&self) -> u32 {
        (self.bits & InfoCrc1::INFO_CRC1_MASK) >> InfoCrc1::INFO_CRC1_OFFSET
    }

}

impl Default for InfoCrc1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl InfoCrc1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InfoCrc1Val {
        InfoCrc1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: InfoCrc1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(InfoCrc1Val) -> InfoCrc1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// FLASH_DEVINFO Register
///
/// Stores information about external flash device(s). (ECC)
/// Assumed to be valid if BOOT_FLAGS0_FLASH_DEVINFO_ENABLE is set.
///
/// Address offset: 0x00a8
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CS1_SIZE** `[15:12]` (RO)
///
///   The size of the flash/PSRAM device on chip select 1 (addressable at 0x11000000 through 0x11ffffff).
///   A value of zero is decoded as a size of zero (no device). Nonzero values are decoded as 4kiB << CS1_SIZE. For example, four megabytes is encoded with a CS1_SIZE value of 10, and 16 megabytes is encoded with a CS1_SIZE value of 12.
///   When BOOT_FLAGS0_FLASH_DEVINFO_ENABLE is not set, a default of zero is used.
///
/// - **CS0_SIZE** `[11:8]` (RO)
///
///   The size of the flash/PSRAM device on chip select 0 (addressable at 0x10000000 through 0x10ffffff).
///   A value of zero is decoded as a size of zero (no device). Nonzero values are decoded as 4kiB << CS0_SIZE. For example, four megabytes is encoded with a CS0_SIZE value of 10, and 16 megabytes is encoded with a CS0_SIZE value of 12.
///   When BOOT_FLAGS0_FLASH_DEVINFO_ENABLE is not set, a default of 12 (16 MiB) is used.
///
/// - **D8H_ERASE_SUPPORTED** `[7]` (RO)
///
///   If true, all attached devices are assumed to support (or ignore, in the case of PSRAM) a block erase command with a command prefix of D8h, an erase size of 64 kiB, and a 24-bit address. Almost all 25-series flash devices support this command.
///   If set, the bootrom will use the D8h erase command where it is able, to accelerate bulk erase operations. This makes flash programming faster.
///   When BOOT_FLAGS0_FLASH_DEVINFO_ENABLE is not set, this field defaults to false.
///
/// - **CS1_GPIO** `[5:0]` (RO)
///
///   Indicate a GPIO number to be used for the secondary flash chip select (CS1), which selects the external QSPI device mapped at system addresses 0x11000000 through 0x11ffffff. There is no such configuration for CS0, as the primary chip select has a dedicated pin.
///   On RP2350 the permissible GPIO numbers are 0, 8, 19 and 47.
///   Ignored if CS1_size is zero. If CS1_SIZE is nonzero, the bootrom will automatically configure this GPIO as a second chip select upon entering the flash boot path, or entering any other path that may use the QSPI flash interface, such as BOOTSEL mode (nsboot).
#[repr(transparent)]
pub struct FlashDevinfo {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for FlashDevinfo {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl FlashDevinfo {
    // ======== Field Constants ========

    /// **CS1_SIZE**
    ///
    /// The size of the flash/PSRAM device on chip select 1 (addressable at 0x11000000 through 0x11ffffff).
    /// A value of zero is decoded as a size of zero (no device). Nonzero values are decoded as 4kiB << CS1_SIZE. For example, four megabytes is encoded with a CS1_SIZE value of 10, and 16 megabytes is encoded with a CS1_SIZE value of 12.
    /// When BOOT_FLAGS0_FLASH_DEVINFO_ENABLE is not set, a default of zero is used.
    ///
    /// Bits `[15:12]`, mask `0x0000f000`
    pub const CS1_SIZE_OFFSET: u32 = 12;
    pub const CS1_SIZE_MASK: u32 = 0x0000f000;
    pub const CS1_SIZE_NONE: u32 = 0 << 12;
    pub const CS1_SIZE_NONE_VAL: u32 = 0;
    pub const CS1_SIZE_8K: u32 = 1 << 12;
    pub const CS1_SIZE_8K_VAL: u32 = 1;
    pub const CS1_SIZE_16K: u32 = 2 << 12;
    pub const CS1_SIZE_16K_VAL: u32 = 2;
    pub const CS1_SIZE_32K: u32 = 3 << 12;
    pub const CS1_SIZE_32K_VAL: u32 = 3;
    pub const CS1_SIZE_64K: u32 = 4 << 12;
    pub const CS1_SIZE_64K_VAL: u32 = 4;
    pub const CS1_SIZE_128K: u32 = 5 << 12;
    pub const CS1_SIZE_128K_VAL: u32 = 5;
    pub const CS1_SIZE_256K: u32 = 6 << 12;
    pub const CS1_SIZE_256K_VAL: u32 = 6;
    pub const CS1_SIZE_512K: u32 = 7 << 12;
    pub const CS1_SIZE_512K_VAL: u32 = 7;
    pub const CS1_SIZE_1M: u32 = 8 << 12;
    pub const CS1_SIZE_1M_VAL: u32 = 8;
    pub const CS1_SIZE_2M: u32 = 9 << 12;
    pub const CS1_SIZE_2M_VAL: u32 = 9;
    pub const CS1_SIZE_4M: u32 = 10 << 12;
    pub const CS1_SIZE_4M_VAL: u32 = 10;
    pub const CS1_SIZE_8M: u32 = 11 << 12;
    pub const CS1_SIZE_8M_VAL: u32 = 11;
    pub const CS1_SIZE_16M: u32 = 12 << 12;
    pub const CS1_SIZE_16M_VAL: u32 = 12;

    /// **CS0_SIZE**
    ///
    /// The size of the flash/PSRAM device on chip select 0 (addressable at 0x10000000 through 0x10ffffff).
    /// A value of zero is decoded as a size of zero (no device). Nonzero values are decoded as 4kiB << CS0_SIZE. For example, four megabytes is encoded with a CS0_SIZE value of 10, and 16 megabytes is encoded with a CS0_SIZE value of 12.
    /// When BOOT_FLAGS0_FLASH_DEVINFO_ENABLE is not set, a default of 12 (16 MiB) is used.
    ///
    /// Bits `[11:8]`, mask `0x00000f00`
    pub const CS0_SIZE_OFFSET: u32 = 8;
    pub const CS0_SIZE_MASK: u32 = 0x00000f00;
    pub const CS0_SIZE_NONE: u32 = 0 << 8;
    pub const CS0_SIZE_NONE_VAL: u32 = 0;
    pub const CS0_SIZE_8K: u32 = 1 << 8;
    pub const CS0_SIZE_8K_VAL: u32 = 1;
    pub const CS0_SIZE_16K: u32 = 2 << 8;
    pub const CS0_SIZE_16K_VAL: u32 = 2;
    pub const CS0_SIZE_32K: u32 = 3 << 8;
    pub const CS0_SIZE_32K_VAL: u32 = 3;
    pub const CS0_SIZE_64K: u32 = 4 << 8;
    pub const CS0_SIZE_64K_VAL: u32 = 4;
    pub const CS0_SIZE_128K: u32 = 5 << 8;
    pub const CS0_SIZE_128K_VAL: u32 = 5;
    pub const CS0_SIZE_256K: u32 = 6 << 8;
    pub const CS0_SIZE_256K_VAL: u32 = 6;
    pub const CS0_SIZE_512K: u32 = 7 << 8;
    pub const CS0_SIZE_512K_VAL: u32 = 7;
    pub const CS0_SIZE_1M: u32 = 8 << 8;
    pub const CS0_SIZE_1M_VAL: u32 = 8;
    pub const CS0_SIZE_2M: u32 = 9 << 8;
    pub const CS0_SIZE_2M_VAL: u32 = 9;
    pub const CS0_SIZE_4M: u32 = 10 << 8;
    pub const CS0_SIZE_4M_VAL: u32 = 10;
    pub const CS0_SIZE_8M: u32 = 11 << 8;
    pub const CS0_SIZE_8M_VAL: u32 = 11;
    pub const CS0_SIZE_16M: u32 = 12 << 8;
    pub const CS0_SIZE_16M_VAL: u32 = 12;

    /// **D8H_ERASE_SUPPORTED**
    ///
    /// If true, all attached devices are assumed to support (or ignore, in the case of PSRAM) a block erase command with a command prefix of D8h, an erase size of 64 kiB, and a 24-bit address. Almost all 25-series flash devices support this command.
    /// If set, the bootrom will use the D8h erase command where it is able, to accelerate bulk erase operations. This makes flash programming faster.
    /// When BOOT_FLAGS0_FLASH_DEVINFO_ENABLE is not set, this field defaults to false.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const D8H_ERASE_SUPPORTED_OFFSET: u32 = 7;
    pub const D8H_ERASE_SUPPORTED_MASK: u32 = 0x00000080;
    pub const D8H_ERASE_SUPPORTED: u32 = 1 << 7;

    /// **CS1_GPIO**
    ///
    /// Indicate a GPIO number to be used for the secondary flash chip select (CS1), which selects the external QSPI device mapped at system addresses 0x11000000 through 0x11ffffff. There is no such configuration for CS0, as the primary chip select has a dedicated pin.
    /// On RP2350 the permissible GPIO numbers are 0, 8, 19 and 47.
    /// Ignored if CS1_size is zero. If CS1_SIZE is nonzero, the bootrom will automatically configure this GPIO as a second chip select upon entering the flash boot path, or entering any other path that may use the QSPI flash interface, such as BOOTSEL mode (nsboot).
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CS1_GPIO_OFFSET: u32 = 0;
    pub const CS1_GPIO_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CS1_SIZE** `[15:12]` (RO)
    ///
    /// The size of the flash/PSRAM device on chip select 1 (addressable at 0x11000000 through 0x11ffffff).
    /// A value of zero is decoded as a size of zero (no device). Nonzero values are decoded as 4kiB << CS1_SIZE. For example, four megabytes is encoded with a CS1_SIZE value of 10, and 16 megabytes is encoded with a CS1_SIZE value of 12.
    /// When BOOT_FLAGS0_FLASH_DEVINFO_ENABLE is not set, a default of zero is used.
    #[inline(always)]
    pub fn cs1_size(&self) -> u32 {
        (self.value.read_bits() & Self::CS1_SIZE_MASK) >> Self::CS1_SIZE_OFFSET
    }

    /// **CS0_SIZE** `[11:8]` (RO)
    ///
    /// The size of the flash/PSRAM device on chip select 0 (addressable at 0x10000000 through 0x10ffffff).
    /// A value of zero is decoded as a size of zero (no device). Nonzero values are decoded as 4kiB << CS0_SIZE. For example, four megabytes is encoded with a CS0_SIZE value of 10, and 16 megabytes is encoded with a CS0_SIZE value of 12.
    /// When BOOT_FLAGS0_FLASH_DEVINFO_ENABLE is not set, a default of 12 (16 MiB) is used.
    #[inline(always)]
    pub fn cs0_size(&self) -> u32 {
        (self.value.read_bits() & Self::CS0_SIZE_MASK) >> Self::CS0_SIZE_OFFSET
    }

    /// **D8H_ERASE_SUPPORTED** `[7]` (RO)
    ///
    /// If true, all attached devices are assumed to support (or ignore, in the case of PSRAM) a block erase command with a command prefix of D8h, an erase size of 64 kiB, and a 24-bit address. Almost all 25-series flash devices support this command.
    /// If set, the bootrom will use the D8h erase command where it is able, to accelerate bulk erase operations. This makes flash programming faster.
    /// When BOOT_FLAGS0_FLASH_DEVINFO_ENABLE is not set, this field defaults to false.
    #[inline(always)]
    pub fn d8h_erase_supported(&self) -> bool {
        (self.value.read_bits() & Self::D8H_ERASE_SUPPORTED_MASK) != 0
    }

    /// **CS1_GPIO** `[5:0]` (RO)
    ///
    /// Indicate a GPIO number to be used for the secondary flash chip select (CS1), which selects the external QSPI device mapped at system addresses 0x11000000 through 0x11ffffff. There is no such configuration for CS0, as the primary chip select has a dedicated pin.
    /// On RP2350 the permissible GPIO numbers are 0, 8, 19 and 47.
    /// Ignored if CS1_size is zero. If CS1_SIZE is nonzero, the bootrom will automatically configure this GPIO as a second chip select upon entering the flash boot path, or entering any other path that may use the QSPI flash interface, such as BOOTSEL mode (nsboot).
    #[inline(always)]
    pub fn cs1_gpio(&self) -> u32 {
        (self.value.read_bits() & Self::CS1_GPIO_MASK) >> Self::CS1_GPIO_OFFSET
    }

}

/// FLASH_DEVINFO value type for batch read/modify/write operations
///
/// Use `flashdevinfo.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `flashdevinfo.write_val(val)`.
#[derive(Clone, Copy)]
pub struct FlashDevinfoVal {
    bits: u32,
}

impl FlashDevinfoVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
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

    /// Read **CS1_SIZE** `[15:12]`
    ///
    /// The size of the flash/PSRAM device on chip select 1 (addressable at 0x11000000 through 0x11ffffff).
    /// A value of zero is decoded as a size of zero (no device). Nonzero values are decoded as 4kiB << CS1_SIZE. For example, four megabytes is encoded with a CS1_SIZE value of 10, and 16 megabytes is encoded with a CS1_SIZE value of 12.
    /// When BOOT_FLAGS0_FLASH_DEVINFO_ENABLE is not set, a default of zero is used.
    #[inline(always)]
    pub const fn cs1_size(&self) -> u32 {
        (self.bits & FlashDevinfo::CS1_SIZE_MASK) >> FlashDevinfo::CS1_SIZE_OFFSET
    }

    /// Read **CS0_SIZE** `[11:8]`
    ///
    /// The size of the flash/PSRAM device on chip select 0 (addressable at 0x10000000 through 0x10ffffff).
    /// A value of zero is decoded as a size of zero (no device). Nonzero values are decoded as 4kiB << CS0_SIZE. For example, four megabytes is encoded with a CS0_SIZE value of 10, and 16 megabytes is encoded with a CS0_SIZE value of 12.
    /// When BOOT_FLAGS0_FLASH_DEVINFO_ENABLE is not set, a default of 12 (16 MiB) is used.
    #[inline(always)]
    pub const fn cs0_size(&self) -> u32 {
        (self.bits & FlashDevinfo::CS0_SIZE_MASK) >> FlashDevinfo::CS0_SIZE_OFFSET
    }

    /// Read **D8H_ERASE_SUPPORTED** `[7:7]`
    ///
    /// If true, all attached devices are assumed to support (or ignore, in the case of PSRAM) a block erase command with a command prefix of D8h, an erase size of 64 kiB, and a 24-bit address. Almost all 25-series flash devices support this command.
    /// If set, the bootrom will use the D8h erase command where it is able, to accelerate bulk erase operations. This makes flash programming faster.
    /// When BOOT_FLAGS0_FLASH_DEVINFO_ENABLE is not set, this field defaults to false.
    #[inline(always)]
    pub const fn d8h_erase_supported(&self) -> bool {
        (self.bits & FlashDevinfo::D8H_ERASE_SUPPORTED_MASK) != 0
    }

    /// Read **CS1_GPIO** `[5:0]`
    ///
    /// Indicate a GPIO number to be used for the secondary flash chip select (CS1), which selects the external QSPI device mapped at system addresses 0x11000000 through 0x11ffffff. There is no such configuration for CS0, as the primary chip select has a dedicated pin.
    /// On RP2350 the permissible GPIO numbers are 0, 8, 19 and 47.
    /// Ignored if CS1_size is zero. If CS1_SIZE is nonzero, the bootrom will automatically configure this GPIO as a second chip select upon entering the flash boot path, or entering any other path that may use the QSPI flash interface, such as BOOTSEL mode (nsboot).
    #[inline(always)]
    pub const fn cs1_gpio(&self) -> u32 {
        (self.bits & FlashDevinfo::CS1_GPIO_MASK) >> FlashDevinfo::CS1_GPIO_OFFSET
    }

}

impl Default for FlashDevinfoVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl FlashDevinfo {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> FlashDevinfoVal {
        FlashDevinfoVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: FlashDevinfoVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(FlashDevinfoVal) -> FlashDevinfoVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// FLASH_PARTITION_SLOT_SIZE Register
///
/// Gap between partition table slot 0 and slot 1 at the start of flash (the default size is 4096 bytes) (ECC) Enabled by the OVERRIDE_FLASH_PARTITION_SLOT_SIZE bit in BOOT_FLAGS, the size is 4096 * (value + 1)
///
/// Address offset: 0x00aa
///
/// ## Fields
///
/// - **FLASH_PARTITION_SLOT_SIZE** `[15:0]` (RO)
#[repr(transparent)]
pub struct FlashPartitionSlotSize {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for FlashPartitionSlotSize {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl FlashPartitionSlotSize {
    // ======== Field Constants ========

    /// **FLASH_PARTITION_SLOT_SIZE**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const FLASH_PARTITION_SLOT_SIZE_OFFSET: u32 = 0;
    pub const FLASH_PARTITION_SLOT_SIZE_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **FLASH_PARTITION_SLOT_SIZE** `[15:0]` (RO)
    #[inline(always)]
    pub fn flash_partition_slot_size(&self) -> u32 {
        (self.value.read_bits() & Self::FLASH_PARTITION_SLOT_SIZE_MASK) >> Self::FLASH_PARTITION_SLOT_SIZE_OFFSET
    }

}

/// FLASH_PARTITION_SLOT_SIZE value type for batch read/modify/write operations
///
/// Use `flashpartitionslotsize.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `flashpartitionslotsize.write_val(val)`.
#[derive(Clone, Copy)]
pub struct FlashPartitionSlotSizeVal {
    bits: u32,
}

impl FlashPartitionSlotSizeVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **FLASH_PARTITION_SLOT_SIZE** `[15:0]`
    #[inline(always)]
    pub const fn flash_partition_slot_size(&self) -> u32 {
        (self.bits & FlashPartitionSlotSize::FLASH_PARTITION_SLOT_SIZE_MASK) >> FlashPartitionSlotSize::FLASH_PARTITION_SLOT_SIZE_OFFSET
    }

}

impl Default for FlashPartitionSlotSizeVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl FlashPartitionSlotSize {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> FlashPartitionSlotSizeVal {
        FlashPartitionSlotSizeVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: FlashPartitionSlotSizeVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(FlashPartitionSlotSizeVal) -> FlashPartitionSlotSizeVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// BOOTSEL_LED_CFG Register
///
/// Pin configuration for LED status, used by USB bootloader. (ECC)
/// Must be valid if BOOT_FLAGS0_ENABLE_BOOTSEL_LED is set.
///
/// Address offset: 0x00ac
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **ACTIVELOW** `[8]` (RO)
///
///   LED is active-low. (Default: active-high.)
///
/// - **PIN** `[5:0]` (RO)
///
///   GPIO index to use for bootloader activity LED.
#[repr(transparent)]
pub struct BootselLedCfg {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for BootselLedCfg {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl BootselLedCfg {
    // ======== Field Constants ========

    /// **ACTIVELOW**
    ///
    /// LED is active-low. (Default: active-high.)
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ACTIVELOW_OFFSET: u32 = 8;
    pub const ACTIVELOW_MASK: u32 = 0x00000100;
    pub const ACTIVELOW: u32 = 1 << 8;

    /// **PIN**
    ///
    /// GPIO index to use for bootloader activity LED.
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const PIN_OFFSET: u32 = 0;
    pub const PIN_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **ACTIVELOW** `[8]` (RO)
    ///
    /// LED is active-low. (Default: active-high.)
    #[inline(always)]
    pub fn activelow(&self) -> bool {
        (self.value.read_bits() & Self::ACTIVELOW_MASK) != 0
    }

    /// **PIN** `[5:0]` (RO)
    ///
    /// GPIO index to use for bootloader activity LED.
    #[inline(always)]
    pub fn pin(&self) -> u32 {
        (self.value.read_bits() & Self::PIN_MASK) >> Self::PIN_OFFSET
    }

}

/// BOOTSEL_LED_CFG value type for batch read/modify/write operations
///
/// Use `bootselledcfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootselledcfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BootselLedCfgVal {
    bits: u32,
}

impl BootselLedCfgVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
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

    /// Read **ACTIVELOW** `[8:8]`
    ///
    /// LED is active-low. (Default: active-high.)
    #[inline(always)]
    pub const fn activelow(&self) -> bool {
        (self.bits & BootselLedCfg::ACTIVELOW_MASK) != 0
    }

    /// Read **PIN** `[5:0]`
    ///
    /// GPIO index to use for bootloader activity LED.
    #[inline(always)]
    pub const fn pin(&self) -> u32 {
        (self.bits & BootselLedCfg::PIN_MASK) >> BootselLedCfg::PIN_OFFSET
    }

}

impl Default for BootselLedCfgVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl BootselLedCfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> BootselLedCfgVal {
        BootselLedCfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: BootselLedCfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(BootselLedCfgVal) -> BootselLedCfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// BOOTSEL_PLL_CFG Register
///
/// Optional PLL configuration for BOOTSEL mode. (ECC)
/// This should be configured to produce an exact 48 MHz based on the crystal oscillator frequency. User mode software may also use this value to calculate the expected crystal frequency based on an assumed 48 MHz PLL output.
/// If no configuration is given, the crystal is assumed to be 12 MHz.
/// The PLL frequency can be calculated as:
/// PLL out = (XOSC frequency / (REFDIV+1)) x FBDIV / (POSTDIV1 x POSTDIV2)
/// Conversely the crystal frequency can be calculated as:
/// XOSC frequency = 48 MHz x (REFDIV+1) x (POSTDIV1 x POSTDIV2) / FBDIV
/// (Note the +1 on REFDIV is because the value stored in this OTP location is the actual divisor value minus one.)
/// Used if and only if ENABLE_BOOTSEL_NON_DEFAULT_PLL_XOSC_CFG is set in BOOT_FLAGS0. That bit should be set only after this row and BOOTSEL_XOSC_CFG are both correctly programmed.
///
/// Address offset: 0x00ae
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **REFDIV** `[15]` (RO)
///
///   PLL reference divisor, minus one.
///   Programming a value of 0 means a reference divisor of 1. Programming a value of 1 means a reference divisor of 2 (for exceptionally fast XIN inputs)
///
/// - **POSTDIV2** `[14:12]` (RO)
///
///   PLL post-divide 2 divisor, in the range 1..7 inclusive.
///
/// - **POSTDIV1** `[11:9]` (RO)
///
///   PLL post-divide 1 divisor, in the range 1..7 inclusive.
///
/// - **FBDIV** `[8:0]` (RO)
///
///   PLL feedback divisor, in the range 16..320 inclusive.
#[repr(transparent)]
pub struct BootselPllCfg {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for BootselPllCfg {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl BootselPllCfg {
    // ======== Field Constants ========

    /// **REFDIV**
    ///
    /// PLL reference divisor, minus one.
    /// Programming a value of 0 means a reference divisor of 1. Programming a value of 1 means a reference divisor of 2 (for exceptionally fast XIN inputs)
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const REFDIV_OFFSET: u32 = 15;
    pub const REFDIV_MASK: u32 = 0x00008000;
    pub const REFDIV: u32 = 1 << 15;

    /// **POSTDIV2**
    ///
    /// PLL post-divide 2 divisor, in the range 1..7 inclusive.
    ///
    /// Bits `[14:12]`, mask `0x00007000`
    pub const POSTDIV2_OFFSET: u32 = 12;
    pub const POSTDIV2_MASK: u32 = 0x00007000;

    /// **POSTDIV1**
    ///
    /// PLL post-divide 1 divisor, in the range 1..7 inclusive.
    ///
    /// Bits `[11:9]`, mask `0x00000e00`
    pub const POSTDIV1_OFFSET: u32 = 9;
    pub const POSTDIV1_MASK: u32 = 0x00000e00;

    /// **FBDIV**
    ///
    /// PLL feedback divisor, in the range 16..320 inclusive.
    ///
    /// Bits `[8:0]`, mask `0x000001ff`
    pub const FBDIV_OFFSET: u32 = 0;
    pub const FBDIV_MASK: u32 = 0x000001ff;

    // ======== Field Accessors ========

    /// **REFDIV** `[15]` (RO)
    ///
    /// PLL reference divisor, minus one.
    /// Programming a value of 0 means a reference divisor of 1. Programming a value of 1 means a reference divisor of 2 (for exceptionally fast XIN inputs)
    #[inline(always)]
    pub fn refdiv(&self) -> bool {
        (self.value.read_bits() & Self::REFDIV_MASK) != 0
    }

    /// **POSTDIV2** `[14:12]` (RO)
    ///
    /// PLL post-divide 2 divisor, in the range 1..7 inclusive.
    #[inline(always)]
    pub fn postdiv2(&self) -> u32 {
        (self.value.read_bits() & Self::POSTDIV2_MASK) >> Self::POSTDIV2_OFFSET
    }

    /// **POSTDIV1** `[11:9]` (RO)
    ///
    /// PLL post-divide 1 divisor, in the range 1..7 inclusive.
    #[inline(always)]
    pub fn postdiv1(&self) -> u32 {
        (self.value.read_bits() & Self::POSTDIV1_MASK) >> Self::POSTDIV1_OFFSET
    }

    /// **FBDIV** `[8:0]` (RO)
    ///
    /// PLL feedback divisor, in the range 16..320 inclusive.
    #[inline(always)]
    pub fn fbdiv(&self) -> u32 {
        (self.value.read_bits() & Self::FBDIV_MASK) >> Self::FBDIV_OFFSET
    }

}

/// BOOTSEL_PLL_CFG value type for batch read/modify/write operations
///
/// Use `bootselpllcfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootselpllcfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BootselPllCfgVal {
    bits: u32,
}

impl BootselPllCfgVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
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

    /// Read **REFDIV** `[15:15]`
    ///
    /// PLL reference divisor, minus one.
    /// Programming a value of 0 means a reference divisor of 1. Programming a value of 1 means a reference divisor of 2 (for exceptionally fast XIN inputs)
    #[inline(always)]
    pub const fn refdiv(&self) -> bool {
        (self.bits & BootselPllCfg::REFDIV_MASK) != 0
    }

    /// Read **POSTDIV2** `[14:12]`
    ///
    /// PLL post-divide 2 divisor, in the range 1..7 inclusive.
    #[inline(always)]
    pub const fn postdiv2(&self) -> u32 {
        (self.bits & BootselPllCfg::POSTDIV2_MASK) >> BootselPllCfg::POSTDIV2_OFFSET
    }

    /// Read **POSTDIV1** `[11:9]`
    ///
    /// PLL post-divide 1 divisor, in the range 1..7 inclusive.
    #[inline(always)]
    pub const fn postdiv1(&self) -> u32 {
        (self.bits & BootselPllCfg::POSTDIV1_MASK) >> BootselPllCfg::POSTDIV1_OFFSET
    }

    /// Read **FBDIV** `[8:0]`
    ///
    /// PLL feedback divisor, in the range 16..320 inclusive.
    #[inline(always)]
    pub const fn fbdiv(&self) -> u32 {
        (self.bits & BootselPllCfg::FBDIV_MASK) >> BootselPllCfg::FBDIV_OFFSET
    }

}

impl Default for BootselPllCfgVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl BootselPllCfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> BootselPllCfgVal {
        BootselPllCfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: BootselPllCfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(BootselPllCfgVal) -> BootselPllCfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// BOOTSEL_XOSC_CFG Register
///
/// Non-default crystal oscillator configuration for the USB bootloader. (ECC)
/// These values may also be used by user code configuring the crystal oscillator.
/// Used if and only if ENABLE_BOOTSEL_NON_DEFAULT_PLL_XOSC_CFG is set in BOOT_FLAGS0. That bit should be set only after this row and BOOTSEL_PLL_CFG are both correctly programmed.
///
/// Address offset: 0x00b0
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **RANGE** `[15:14]` (RO)
///
///   Value of the XOSC_CTRL_FREQ_RANGE register.
///
/// - **STARTUP** `[13:0]` (RO)
///
///   Value of the XOSC_STARTUP register
#[repr(transparent)]
pub struct BootselXoscCfg {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for BootselXoscCfg {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl BootselXoscCfg {
    // ======== Field Constants ========

    /// **RANGE**
    ///
    /// Value of the XOSC_CTRL_FREQ_RANGE register.
    ///
    /// Bits `[15:14]`, mask `0x0000c000`
    pub const RANGE_OFFSET: u32 = 14;
    pub const RANGE_MASK: u32 = 0x0000c000;
    pub const RANGE_1_15MHZ: u32 = 0 << 14;
    pub const RANGE_1_15MHZ_VAL: u32 = 0;
    pub const RANGE_10_30MHZ: u32 = 1 << 14;
    pub const RANGE_10_30MHZ_VAL: u32 = 1;
    pub const RANGE_25_60MHZ: u32 = 2 << 14;
    pub const RANGE_25_60MHZ_VAL: u32 = 2;
    pub const RANGE_40_100MHZ: u32 = 3 << 14;
    pub const RANGE_40_100MHZ_VAL: u32 = 3;

    /// **STARTUP**
    ///
    /// Value of the XOSC_STARTUP register
    ///
    /// Bits `[13:0]`, mask `0x00003fff`
    pub const STARTUP_OFFSET: u32 = 0;
    pub const STARTUP_MASK: u32 = 0x00003fff;

    // ======== Field Accessors ========

    /// **RANGE** `[15:14]` (RO)
    ///
    /// Value of the XOSC_CTRL_FREQ_RANGE register.
    #[inline(always)]
    pub fn range(&self) -> u32 {
        (self.value.read_bits() & Self::RANGE_MASK) >> Self::RANGE_OFFSET
    }

    /// **STARTUP** `[13:0]` (RO)
    ///
    /// Value of the XOSC_STARTUP register
    #[inline(always)]
    pub fn startup(&self) -> u32 {
        (self.value.read_bits() & Self::STARTUP_MASK) >> Self::STARTUP_OFFSET
    }

}

/// BOOTSEL_XOSC_CFG value type for batch read/modify/write operations
///
/// Use `bootselxosccfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootselxosccfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BootselXoscCfgVal {
    bits: u32,
}

impl BootselXoscCfgVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
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

    /// Read **RANGE** `[15:14]`
    ///
    /// Value of the XOSC_CTRL_FREQ_RANGE register.
    #[inline(always)]
    pub const fn range(&self) -> u32 {
        (self.bits & BootselXoscCfg::RANGE_MASK) >> BootselXoscCfg::RANGE_OFFSET
    }

    /// Read **STARTUP** `[13:0]`
    ///
    /// Value of the XOSC_STARTUP register
    #[inline(always)]
    pub const fn startup(&self) -> u32 {
        (self.bits & BootselXoscCfg::STARTUP_MASK) >> BootselXoscCfg::STARTUP_OFFSET
    }

}

impl Default for BootselXoscCfgVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl BootselXoscCfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> BootselXoscCfgVal {
        BootselXoscCfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: BootselXoscCfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(BootselXoscCfgVal) -> BootselXoscCfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// USB_WHITE_LABEL_ADDR Register
///
/// Row index of the USB_WHITE_LABEL structure within OTP (ECC)
/// The table has 16 rows, each of which are also ECC and marked valid by the corresponding valid bit in USB_BOOT_FLAGS (ECC).
/// The entries are either _VALUEs where the 16 bit value is used as is, or _STRDEFs which acts as a pointers to a string value.
/// The value stored in a _STRDEF is two separate bytes: The low seven bits of the first (LSB) byte indicates the number of characters in the string, and the top bit of the first (LSB) byte if set to indicate that each character in the string is two bytes (Unicode) versus one byte if unset. The second (MSB) byte represents the location of the string data, and is encoded as the number of rows from this USB_WHITE_LABEL_ADDR; i.e. the row of the start of the string is USB_WHITE_LABEL_ADDR value + msb_byte.
/// In each case, the corresponding valid bit enables replacing the default value for the corresponding item provided by the boot rom.
/// Note that Unicode _STRDEFs are only supported for USB_DEVICE_PRODUCT_STRDEF, USB_DEVICE_SERIAL_NUMBER_STRDEF and USB_DEVICE_MANUFACTURER_STRDEF. Unicode values will be ignored if specified for other fields, and non-unicode values for these three items will be converted to Unicode characters by setting the upper 8 bits to zero.
/// Note that if the USB_WHITE_LABEL structure or the corresponding strings are not readable by BOOTSEL mode based on OTP permissions, or if alignment requirements are not met, then the corresponding default values are used.
/// The index values indicate where each field is located (row USB_WHITE_LABEL_ADDR value + index):
///
/// Address offset: 0x00b8
///
/// ## Fields
///
/// - **USB_WHITE_LABEL_ADDR** `[15:0]` (RO)
#[repr(transparent)]
pub struct UsbWhiteLabelAddr {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for UsbWhiteLabelAddr {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl UsbWhiteLabelAddr {
    // ======== Field Constants ========

    /// **USB_WHITE_LABEL_ADDR**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const USB_WHITE_LABEL_ADDR_OFFSET: u32 = 0;
    pub const USB_WHITE_LABEL_ADDR_MASK: u32 = 0x0000ffff;
    pub const USB_WHITE_LABEL_ADDR_INDEX_USB_DEVICE_VID_VALUE: u32 = 0 << 0;
    pub const USB_WHITE_LABEL_ADDR_INDEX_USB_DEVICE_VID_VALUE_VAL: u32 = 0;
    pub const USB_WHITE_LABEL_ADDR_INDEX_USB_DEVICE_PID_VALUE: u32 = 1 << 0;
    pub const USB_WHITE_LABEL_ADDR_INDEX_USB_DEVICE_PID_VALUE_VAL: u32 = 1;
    pub const USB_WHITE_LABEL_ADDR_INDEX_USB_DEVICE_BCD_DEVICE_VALUE: u32 = 2 << 0;
    pub const USB_WHITE_LABEL_ADDR_INDEX_USB_DEVICE_BCD_DEVICE_VALUE_VAL: u32 = 2;
    pub const USB_WHITE_LABEL_ADDR_INDEX_USB_DEVICE_LANG_ID_VALUE: u32 = 3 << 0;
    pub const USB_WHITE_LABEL_ADDR_INDEX_USB_DEVICE_LANG_ID_VALUE_VAL: u32 = 3;
    pub const USB_WHITE_LABEL_ADDR_INDEX_USB_DEVICE_MANUFACTURER_STRDEF: u32 = 4 << 0;
    pub const USB_WHITE_LABEL_ADDR_INDEX_USB_DEVICE_MANUFACTURER_STRDEF_VAL: u32 = 4;
    pub const USB_WHITE_LABEL_ADDR_INDEX_USB_DEVICE_PRODUCT_STRDEF: u32 = 5 << 0;
    pub const USB_WHITE_LABEL_ADDR_INDEX_USB_DEVICE_PRODUCT_STRDEF_VAL: u32 = 5;
    pub const USB_WHITE_LABEL_ADDR_INDEX_USB_DEVICE_SERIAL_NUMBER_STRDEF: u32 = 6 << 0;
    pub const USB_WHITE_LABEL_ADDR_INDEX_USB_DEVICE_SERIAL_NUMBER_STRDEF_VAL: u32 = 6;
    pub const USB_WHITE_LABEL_ADDR_INDEX_USB_CONFIG_ATTRIBUTES_MAX_POWER_VALUES: u32 = 7 << 0;
    pub const USB_WHITE_LABEL_ADDR_INDEX_USB_CONFIG_ATTRIBUTES_MAX_POWER_VALUES_VAL: u32 = 7;
    pub const USB_WHITE_LABEL_ADDR_INDEX_VOLUME_LABEL_STRDEF: u32 = 8 << 0;
    pub const USB_WHITE_LABEL_ADDR_INDEX_VOLUME_LABEL_STRDEF_VAL: u32 = 8;
    pub const USB_WHITE_LABEL_ADDR_INDEX_SCSI_INQUIRY_VENDOR_STRDEF: u32 = 9 << 0;
    pub const USB_WHITE_LABEL_ADDR_INDEX_SCSI_INQUIRY_VENDOR_STRDEF_VAL: u32 = 9;
    pub const USB_WHITE_LABEL_ADDR_INDEX_SCSI_INQUIRY_PRODUCT_STRDEF: u32 = 10 << 0;
    pub const USB_WHITE_LABEL_ADDR_INDEX_SCSI_INQUIRY_PRODUCT_STRDEF_VAL: u32 = 10;
    pub const USB_WHITE_LABEL_ADDR_INDEX_SCSI_INQUIRY_VERSION_STRDEF: u32 = 11 << 0;
    pub const USB_WHITE_LABEL_ADDR_INDEX_SCSI_INQUIRY_VERSION_STRDEF_VAL: u32 = 11;
    pub const USB_WHITE_LABEL_ADDR_INDEX_INDEX_HTM_REDIRECT_URL_STRDEF: u32 = 12 << 0;
    pub const USB_WHITE_LABEL_ADDR_INDEX_INDEX_HTM_REDIRECT_URL_STRDEF_VAL: u32 = 12;
    pub const USB_WHITE_LABEL_ADDR_INDEX_INDEX_HTM_REDIRECT_NAME_STRDEF: u32 = 13 << 0;
    pub const USB_WHITE_LABEL_ADDR_INDEX_INDEX_HTM_REDIRECT_NAME_STRDEF_VAL: u32 = 13;
    pub const USB_WHITE_LABEL_ADDR_INDEX_INFO_UF2_TXT_MODEL_STRDEF: u32 = 14 << 0;
    pub const USB_WHITE_LABEL_ADDR_INDEX_INFO_UF2_TXT_MODEL_STRDEF_VAL: u32 = 14;
    pub const USB_WHITE_LABEL_ADDR_INDEX_INFO_UF2_TXT_BOARD_ID_STRDEF: u32 = 15 << 0;
    pub const USB_WHITE_LABEL_ADDR_INDEX_INFO_UF2_TXT_BOARD_ID_STRDEF_VAL: u32 = 15;

    // ======== Field Accessors ========

    /// **USB_WHITE_LABEL_ADDR** `[15:0]` (RO)
    #[inline(always)]
    pub fn usb_white_label_addr(&self) -> u32 {
        (self.value.read_bits() & Self::USB_WHITE_LABEL_ADDR_MASK) >> Self::USB_WHITE_LABEL_ADDR_OFFSET
    }

}

/// USB_WHITE_LABEL_ADDR value type for batch read/modify/write operations
///
/// Use `usbwhitelabeladdr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `usbwhitelabeladdr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct UsbWhiteLabelAddrVal {
    bits: u32,
}

impl UsbWhiteLabelAddrVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **USB_WHITE_LABEL_ADDR** `[15:0]`
    #[inline(always)]
    pub const fn usb_white_label_addr(&self) -> u32 {
        (self.bits & UsbWhiteLabelAddr::USB_WHITE_LABEL_ADDR_MASK) >> UsbWhiteLabelAddr::USB_WHITE_LABEL_ADDR_OFFSET
    }

}

impl Default for UsbWhiteLabelAddrVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl UsbWhiteLabelAddr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> UsbWhiteLabelAddrVal {
        UsbWhiteLabelAddrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: UsbWhiteLabelAddrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(UsbWhiteLabelAddrVal) -> UsbWhiteLabelAddrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// OTPBOOT_SRC Register
///
/// OTP start row for the OTP boot image. (ECC)
/// If OTP boot is enabled, the bootrom will load from this location into SRAM and then directly enter the loaded image. Note that the image must be signed if SECURE_BOOT_ENABLE is set. The image itself is assumed to be ECC-protected.
/// This must be an even number. Equivalently, the OTP boot image must start at a word-aligned location in the ECC read data address window.
///
/// Address offset: 0x00bc
///
/// ## Fields
///
/// - **OTPBOOT_SRC** `[15:0]` (RO)
#[repr(transparent)]
pub struct OtpbootSrc {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for OtpbootSrc {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl OtpbootSrc {
    // ======== Field Constants ========

    /// **OTPBOOT_SRC**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const OTPBOOT_SRC_OFFSET: u32 = 0;
    pub const OTPBOOT_SRC_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **OTPBOOT_SRC** `[15:0]` (RO)
    #[inline(always)]
    pub fn otpboot_src(&self) -> u32 {
        (self.value.read_bits() & Self::OTPBOOT_SRC_MASK) >> Self::OTPBOOT_SRC_OFFSET
    }

}

/// OTPBOOT_SRC value type for batch read/modify/write operations
///
/// Use `otpbootsrc.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `otpbootsrc.write_val(val)`.
#[derive(Clone, Copy)]
pub struct OtpbootSrcVal {
    bits: u32,
}

impl OtpbootSrcVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **OTPBOOT_SRC** `[15:0]`
    #[inline(always)]
    pub const fn otpboot_src(&self) -> u32 {
        (self.bits & OtpbootSrc::OTPBOOT_SRC_MASK) >> OtpbootSrc::OTPBOOT_SRC_OFFSET
    }

}

impl Default for OtpbootSrcVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl OtpbootSrc {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> OtpbootSrcVal {
        OtpbootSrcVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: OtpbootSrcVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(OtpbootSrcVal) -> OtpbootSrcVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// OTPBOOT_LEN Register
///
/// Length in rows of the OTP boot image. (ECC)
/// OTPBOOT_LEN must be even. The total image size must be a multiple of 4 bytes (32 bits).
///
/// Address offset: 0x00be
///
/// ## Fields
///
/// - **OTPBOOT_LEN** `[15:0]` (RO)
#[repr(transparent)]
pub struct OtpbootLen {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for OtpbootLen {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl OtpbootLen {
    // ======== Field Constants ========

    /// **OTPBOOT_LEN**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const OTPBOOT_LEN_OFFSET: u32 = 0;
    pub const OTPBOOT_LEN_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **OTPBOOT_LEN** `[15:0]` (RO)
    #[inline(always)]
    pub fn otpboot_len(&self) -> u32 {
        (self.value.read_bits() & Self::OTPBOOT_LEN_MASK) >> Self::OTPBOOT_LEN_OFFSET
    }

}

/// OTPBOOT_LEN value type for batch read/modify/write operations
///
/// Use `otpbootlen.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `otpbootlen.write_val(val)`.
#[derive(Clone, Copy)]
pub struct OtpbootLenVal {
    bits: u32,
}

impl OtpbootLenVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **OTPBOOT_LEN** `[15:0]`
    #[inline(always)]
    pub const fn otpboot_len(&self) -> u32 {
        (self.bits & OtpbootLen::OTPBOOT_LEN_MASK) >> OtpbootLen::OTPBOOT_LEN_OFFSET
    }

}

impl Default for OtpbootLenVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl OtpbootLen {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> OtpbootLenVal {
        OtpbootLenVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: OtpbootLenVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(OtpbootLenVal) -> OtpbootLenVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// OTPBOOT_DST0 Register
///
/// Bits 15:0 of the OTP boot image load destination (and entry point). (ECC)
/// This must be a location in main SRAM (main SRAM is addresses 0x20000000 through 0x20082000) and must be word-aligned.
///
/// Address offset: 0x00c0
///
/// ## Fields
///
/// - **OTPBOOT_DST0** `[15:0]` (RO)
#[repr(transparent)]
pub struct OtpbootDst0 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for OtpbootDst0 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl OtpbootDst0 {
    // ======== Field Constants ========

    /// **OTPBOOT_DST0**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const OTPBOOT_DST0_OFFSET: u32 = 0;
    pub const OTPBOOT_DST0_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **OTPBOOT_DST0** `[15:0]` (RO)
    #[inline(always)]
    pub fn otpboot_dst0(&self) -> u32 {
        (self.value.read_bits() & Self::OTPBOOT_DST0_MASK) >> Self::OTPBOOT_DST0_OFFSET
    }

}

/// OTPBOOT_DST0 value type for batch read/modify/write operations
///
/// Use `otpbootdst0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `otpbootdst0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct OtpbootDst0Val {
    bits: u32,
}

impl OtpbootDst0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **OTPBOOT_DST0** `[15:0]`
    #[inline(always)]
    pub const fn otpboot_dst0(&self) -> u32 {
        (self.bits & OtpbootDst0::OTPBOOT_DST0_MASK) >> OtpbootDst0::OTPBOOT_DST0_OFFSET
    }

}

impl Default for OtpbootDst0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl OtpbootDst0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> OtpbootDst0Val {
        OtpbootDst0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: OtpbootDst0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(OtpbootDst0Val) -> OtpbootDst0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// OTPBOOT_DST1 Register
///
/// Bits 31:16 of the OTP boot image load destination (and entry point). (ECC)
/// This must be a location in main SRAM (main SRAM is addresses 0x20000000 through 0x20082000) and must be word-aligned.
///
/// Address offset: 0x00c2
///
/// ## Fields
///
/// - **OTPBOOT_DST1** `[15:0]` (RO)
#[repr(transparent)]
pub struct OtpbootDst1 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for OtpbootDst1 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl OtpbootDst1 {
    // ======== Field Constants ========

    /// **OTPBOOT_DST1**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const OTPBOOT_DST1_OFFSET: u32 = 0;
    pub const OTPBOOT_DST1_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **OTPBOOT_DST1** `[15:0]` (RO)
    #[inline(always)]
    pub fn otpboot_dst1(&self) -> u32 {
        (self.value.read_bits() & Self::OTPBOOT_DST1_MASK) >> Self::OTPBOOT_DST1_OFFSET
    }

}

/// OTPBOOT_DST1 value type for batch read/modify/write operations
///
/// Use `otpbootdst1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `otpbootdst1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct OtpbootDst1Val {
    bits: u32,
}

impl OtpbootDst1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **OTPBOOT_DST1** `[15:0]`
    #[inline(always)]
    pub const fn otpboot_dst1(&self) -> u32 {
        (self.bits & OtpbootDst1::OTPBOOT_DST1_MASK) >> OtpbootDst1::OTPBOOT_DST1_OFFSET
    }

}

impl Default for OtpbootDst1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl OtpbootDst1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> OtpbootDst1Val {
        OtpbootDst1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: OtpbootDst1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(OtpbootDst1Val) -> OtpbootDst1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// BOOTKEY0_0 Register
///
/// Bits 15:0 of SHA-256 hash of boot key 0 (ECC)
///
/// Address offset: 0x0100
///
/// ## Fields
///
/// - **BOOTKEY0_0** `[15:0]` (RO)
#[repr(transparent)]
pub struct Bootkey00 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Bootkey00 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Bootkey00 {
    // ======== Field Constants ========

    /// **BOOTKEY0_0**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const BOOTKEY0_0_OFFSET: u32 = 0;
    pub const BOOTKEY0_0_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **BOOTKEY0_0** `[15:0]` (RO)
    #[inline(always)]
    pub fn bootkey0_0(&self) -> u32 {
        (self.value.read_bits() & Self::BOOTKEY0_0_MASK) >> Self::BOOTKEY0_0_OFFSET
    }

}

/// BOOTKEY0_0 value type for batch read/modify/write operations
///
/// Use `bootkey00.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootkey00.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bootkey00Val {
    bits: u32,
}

impl Bootkey00Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **BOOTKEY0_0** `[15:0]`
    #[inline(always)]
    pub const fn bootkey0_0(&self) -> u32 {
        (self.bits & Bootkey00::BOOTKEY0_0_MASK) >> Bootkey00::BOOTKEY0_0_OFFSET
    }

}

impl Default for Bootkey00Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Bootkey00 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Bootkey00Val {
        Bootkey00Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Bootkey00Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Bootkey00Val) -> Bootkey00Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// BOOTKEY0_1 Register
///
/// Bits 31:16 of SHA-256 hash of boot key 0 (ECC)
///
/// Address offset: 0x0102
///
/// ## Fields
///
/// - **BOOTKEY0_1** `[15:0]` (RO)
#[repr(transparent)]
pub struct Bootkey01 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Bootkey01 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Bootkey01 {
    // ======== Field Constants ========

    /// **BOOTKEY0_1**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const BOOTKEY0_1_OFFSET: u32 = 0;
    pub const BOOTKEY0_1_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **BOOTKEY0_1** `[15:0]` (RO)
    #[inline(always)]
    pub fn bootkey0_1(&self) -> u32 {
        (self.value.read_bits() & Self::BOOTKEY0_1_MASK) >> Self::BOOTKEY0_1_OFFSET
    }

}

/// BOOTKEY0_1 value type for batch read/modify/write operations
///
/// Use `bootkey01.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootkey01.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bootkey01Val {
    bits: u32,
}

impl Bootkey01Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **BOOTKEY0_1** `[15:0]`
    #[inline(always)]
    pub const fn bootkey0_1(&self) -> u32 {
        (self.bits & Bootkey01::BOOTKEY0_1_MASK) >> Bootkey01::BOOTKEY0_1_OFFSET
    }

}

impl Default for Bootkey01Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Bootkey01 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Bootkey01Val {
        Bootkey01Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Bootkey01Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Bootkey01Val) -> Bootkey01Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// BOOTKEY0_2 Register
///
/// Bits 47:32 of SHA-256 hash of boot key 0 (ECC)
///
/// Address offset: 0x0104
///
/// ## Fields
///
/// - **BOOTKEY0_2** `[15:0]` (RO)
#[repr(transparent)]
pub struct Bootkey02 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Bootkey02 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Bootkey02 {
    // ======== Field Constants ========

    /// **BOOTKEY0_2**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const BOOTKEY0_2_OFFSET: u32 = 0;
    pub const BOOTKEY0_2_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **BOOTKEY0_2** `[15:0]` (RO)
    #[inline(always)]
    pub fn bootkey0_2(&self) -> u32 {
        (self.value.read_bits() & Self::BOOTKEY0_2_MASK) >> Self::BOOTKEY0_2_OFFSET
    }

}

/// BOOTKEY0_2 value type for batch read/modify/write operations
///
/// Use `bootkey02.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootkey02.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bootkey02Val {
    bits: u32,
}

impl Bootkey02Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **BOOTKEY0_2** `[15:0]`
    #[inline(always)]
    pub const fn bootkey0_2(&self) -> u32 {
        (self.bits & Bootkey02::BOOTKEY0_2_MASK) >> Bootkey02::BOOTKEY0_2_OFFSET
    }

}

impl Default for Bootkey02Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Bootkey02 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Bootkey02Val {
        Bootkey02Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Bootkey02Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Bootkey02Val) -> Bootkey02Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// BOOTKEY0_3 Register
///
/// Bits 63:48 of SHA-256 hash of boot key 0 (ECC)
///
/// Address offset: 0x0106
///
/// ## Fields
///
/// - **BOOTKEY0_3** `[15:0]` (RO)
#[repr(transparent)]
pub struct Bootkey03 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Bootkey03 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Bootkey03 {
    // ======== Field Constants ========

    /// **BOOTKEY0_3**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const BOOTKEY0_3_OFFSET: u32 = 0;
    pub const BOOTKEY0_3_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **BOOTKEY0_3** `[15:0]` (RO)
    #[inline(always)]
    pub fn bootkey0_3(&self) -> u32 {
        (self.value.read_bits() & Self::BOOTKEY0_3_MASK) >> Self::BOOTKEY0_3_OFFSET
    }

}

/// BOOTKEY0_3 value type for batch read/modify/write operations
///
/// Use `bootkey03.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootkey03.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bootkey03Val {
    bits: u32,
}

impl Bootkey03Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **BOOTKEY0_3** `[15:0]`
    #[inline(always)]
    pub const fn bootkey0_3(&self) -> u32 {
        (self.bits & Bootkey03::BOOTKEY0_3_MASK) >> Bootkey03::BOOTKEY0_3_OFFSET
    }

}

impl Default for Bootkey03Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Bootkey03 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Bootkey03Val {
        Bootkey03Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Bootkey03Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Bootkey03Val) -> Bootkey03Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// BOOTKEY0_4 Register
///
/// Bits 79:64 of SHA-256 hash of boot key 0 (ECC)
///
/// Address offset: 0x0108
///
/// ## Fields
///
/// - **BOOTKEY0_4** `[15:0]` (RO)
#[repr(transparent)]
pub struct Bootkey04 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Bootkey04 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Bootkey04 {
    // ======== Field Constants ========

    /// **BOOTKEY0_4**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const BOOTKEY0_4_OFFSET: u32 = 0;
    pub const BOOTKEY0_4_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **BOOTKEY0_4** `[15:0]` (RO)
    #[inline(always)]
    pub fn bootkey0_4(&self) -> u32 {
        (self.value.read_bits() & Self::BOOTKEY0_4_MASK) >> Self::BOOTKEY0_4_OFFSET
    }

}

/// BOOTKEY0_4 value type for batch read/modify/write operations
///
/// Use `bootkey04.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootkey04.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bootkey04Val {
    bits: u32,
}

impl Bootkey04Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **BOOTKEY0_4** `[15:0]`
    #[inline(always)]
    pub const fn bootkey0_4(&self) -> u32 {
        (self.bits & Bootkey04::BOOTKEY0_4_MASK) >> Bootkey04::BOOTKEY0_4_OFFSET
    }

}

impl Default for Bootkey04Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Bootkey04 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Bootkey04Val {
        Bootkey04Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Bootkey04Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Bootkey04Val) -> Bootkey04Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// BOOTKEY0_5 Register
///
/// Bits 95:80 of SHA-256 hash of boot key 0 (ECC)
///
/// Address offset: 0x010a
///
/// ## Fields
///
/// - **BOOTKEY0_5** `[15:0]` (RO)
#[repr(transparent)]
pub struct Bootkey05 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Bootkey05 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Bootkey05 {
    // ======== Field Constants ========

    /// **BOOTKEY0_5**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const BOOTKEY0_5_OFFSET: u32 = 0;
    pub const BOOTKEY0_5_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **BOOTKEY0_5** `[15:0]` (RO)
    #[inline(always)]
    pub fn bootkey0_5(&self) -> u32 {
        (self.value.read_bits() & Self::BOOTKEY0_5_MASK) >> Self::BOOTKEY0_5_OFFSET
    }

}

/// BOOTKEY0_5 value type for batch read/modify/write operations
///
/// Use `bootkey05.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootkey05.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bootkey05Val {
    bits: u32,
}

impl Bootkey05Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **BOOTKEY0_5** `[15:0]`
    #[inline(always)]
    pub const fn bootkey0_5(&self) -> u32 {
        (self.bits & Bootkey05::BOOTKEY0_5_MASK) >> Bootkey05::BOOTKEY0_5_OFFSET
    }

}

impl Default for Bootkey05Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Bootkey05 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Bootkey05Val {
        Bootkey05Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Bootkey05Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Bootkey05Val) -> Bootkey05Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// BOOTKEY0_6 Register
///
/// Bits 111:96 of SHA-256 hash of boot key 0 (ECC)
///
/// Address offset: 0x010c
///
/// ## Fields
///
/// - **BOOTKEY0_6** `[15:0]` (RO)
#[repr(transparent)]
pub struct Bootkey06 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Bootkey06 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Bootkey06 {
    // ======== Field Constants ========

    /// **BOOTKEY0_6**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const BOOTKEY0_6_OFFSET: u32 = 0;
    pub const BOOTKEY0_6_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **BOOTKEY0_6** `[15:0]` (RO)
    #[inline(always)]
    pub fn bootkey0_6(&self) -> u32 {
        (self.value.read_bits() & Self::BOOTKEY0_6_MASK) >> Self::BOOTKEY0_6_OFFSET
    }

}

/// BOOTKEY0_6 value type for batch read/modify/write operations
///
/// Use `bootkey06.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootkey06.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bootkey06Val {
    bits: u32,
}

impl Bootkey06Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **BOOTKEY0_6** `[15:0]`
    #[inline(always)]
    pub const fn bootkey0_6(&self) -> u32 {
        (self.bits & Bootkey06::BOOTKEY0_6_MASK) >> Bootkey06::BOOTKEY0_6_OFFSET
    }

}

impl Default for Bootkey06Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Bootkey06 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Bootkey06Val {
        Bootkey06Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Bootkey06Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Bootkey06Val) -> Bootkey06Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// BOOTKEY0_7 Register
///
/// Bits 127:112 of SHA-256 hash of boot key 0 (ECC)
///
/// Address offset: 0x010e
///
/// ## Fields
///
/// - **BOOTKEY0_7** `[15:0]` (RO)
#[repr(transparent)]
pub struct Bootkey07 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Bootkey07 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Bootkey07 {
    // ======== Field Constants ========

    /// **BOOTKEY0_7**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const BOOTKEY0_7_OFFSET: u32 = 0;
    pub const BOOTKEY0_7_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **BOOTKEY0_7** `[15:0]` (RO)
    #[inline(always)]
    pub fn bootkey0_7(&self) -> u32 {
        (self.value.read_bits() & Self::BOOTKEY0_7_MASK) >> Self::BOOTKEY0_7_OFFSET
    }

}

/// BOOTKEY0_7 value type for batch read/modify/write operations
///
/// Use `bootkey07.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootkey07.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bootkey07Val {
    bits: u32,
}

impl Bootkey07Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **BOOTKEY0_7** `[15:0]`
    #[inline(always)]
    pub const fn bootkey0_7(&self) -> u32 {
        (self.bits & Bootkey07::BOOTKEY0_7_MASK) >> Bootkey07::BOOTKEY0_7_OFFSET
    }

}

impl Default for Bootkey07Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Bootkey07 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Bootkey07Val {
        Bootkey07Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Bootkey07Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Bootkey07Val) -> Bootkey07Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// BOOTKEY0_8 Register
///
/// Bits 143:128 of SHA-256 hash of boot key 0 (ECC)
///
/// Address offset: 0x0110
///
/// ## Fields
///
/// - **BOOTKEY0_8** `[15:0]` (RO)
#[repr(transparent)]
pub struct Bootkey08 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Bootkey08 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Bootkey08 {
    // ======== Field Constants ========

    /// **BOOTKEY0_8**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const BOOTKEY0_8_OFFSET: u32 = 0;
    pub const BOOTKEY0_8_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **BOOTKEY0_8** `[15:0]` (RO)
    #[inline(always)]
    pub fn bootkey0_8(&self) -> u32 {
        (self.value.read_bits() & Self::BOOTKEY0_8_MASK) >> Self::BOOTKEY0_8_OFFSET
    }

}

/// BOOTKEY0_8 value type for batch read/modify/write operations
///
/// Use `bootkey08.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootkey08.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bootkey08Val {
    bits: u32,
}

impl Bootkey08Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **BOOTKEY0_8** `[15:0]`
    #[inline(always)]
    pub const fn bootkey0_8(&self) -> u32 {
        (self.bits & Bootkey08::BOOTKEY0_8_MASK) >> Bootkey08::BOOTKEY0_8_OFFSET
    }

}

impl Default for Bootkey08Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Bootkey08 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Bootkey08Val {
        Bootkey08Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Bootkey08Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Bootkey08Val) -> Bootkey08Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// BOOTKEY0_9 Register
///
/// Bits 159:144 of SHA-256 hash of boot key 0 (ECC)
///
/// Address offset: 0x0112
///
/// ## Fields
///
/// - **BOOTKEY0_9** `[15:0]` (RO)
#[repr(transparent)]
pub struct Bootkey09 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Bootkey09 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Bootkey09 {
    // ======== Field Constants ========

    /// **BOOTKEY0_9**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const BOOTKEY0_9_OFFSET: u32 = 0;
    pub const BOOTKEY0_9_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **BOOTKEY0_9** `[15:0]` (RO)
    #[inline(always)]
    pub fn bootkey0_9(&self) -> u32 {
        (self.value.read_bits() & Self::BOOTKEY0_9_MASK) >> Self::BOOTKEY0_9_OFFSET
    }

}

/// BOOTKEY0_9 value type for batch read/modify/write operations
///
/// Use `bootkey09.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootkey09.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bootkey09Val {
    bits: u32,
}

impl Bootkey09Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **BOOTKEY0_9** `[15:0]`
    #[inline(always)]
    pub const fn bootkey0_9(&self) -> u32 {
        (self.bits & Bootkey09::BOOTKEY0_9_MASK) >> Bootkey09::BOOTKEY0_9_OFFSET
    }

}

impl Default for Bootkey09Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Bootkey09 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Bootkey09Val {
        Bootkey09Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Bootkey09Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Bootkey09Val) -> Bootkey09Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// BOOTKEY0_10 Register
///
/// Bits 175:160 of SHA-256 hash of boot key 0 (ECC)
///
/// Address offset: 0x0114
///
/// ## Fields
///
/// - **BOOTKEY0_10** `[15:0]` (RO)
#[repr(transparent)]
pub struct Bootkey010 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Bootkey010 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Bootkey010 {
    // ======== Field Constants ========

    /// **BOOTKEY0_10**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const BOOTKEY0_10_OFFSET: u32 = 0;
    pub const BOOTKEY0_10_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **BOOTKEY0_10** `[15:0]` (RO)
    #[inline(always)]
    pub fn bootkey0_10(&self) -> u32 {
        (self.value.read_bits() & Self::BOOTKEY0_10_MASK) >> Self::BOOTKEY0_10_OFFSET
    }

}

/// BOOTKEY0_10 value type for batch read/modify/write operations
///
/// Use `bootkey010.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootkey010.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bootkey010Val {
    bits: u32,
}

impl Bootkey010Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **BOOTKEY0_10** `[15:0]`
    #[inline(always)]
    pub const fn bootkey0_10(&self) -> u32 {
        (self.bits & Bootkey010::BOOTKEY0_10_MASK) >> Bootkey010::BOOTKEY0_10_OFFSET
    }

}

impl Default for Bootkey010Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Bootkey010 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Bootkey010Val {
        Bootkey010Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Bootkey010Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Bootkey010Val) -> Bootkey010Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// BOOTKEY0_11 Register
///
/// Bits 191:176 of SHA-256 hash of boot key 0 (ECC)
///
/// Address offset: 0x0116
///
/// ## Fields
///
/// - **BOOTKEY0_11** `[15:0]` (RO)
#[repr(transparent)]
pub struct Bootkey011 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Bootkey011 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Bootkey011 {
    // ======== Field Constants ========

    /// **BOOTKEY0_11**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const BOOTKEY0_11_OFFSET: u32 = 0;
    pub const BOOTKEY0_11_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **BOOTKEY0_11** `[15:0]` (RO)
    #[inline(always)]
    pub fn bootkey0_11(&self) -> u32 {
        (self.value.read_bits() & Self::BOOTKEY0_11_MASK) >> Self::BOOTKEY0_11_OFFSET
    }

}

/// BOOTKEY0_11 value type for batch read/modify/write operations
///
/// Use `bootkey011.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootkey011.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bootkey011Val {
    bits: u32,
}

impl Bootkey011Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **BOOTKEY0_11** `[15:0]`
    #[inline(always)]
    pub const fn bootkey0_11(&self) -> u32 {
        (self.bits & Bootkey011::BOOTKEY0_11_MASK) >> Bootkey011::BOOTKEY0_11_OFFSET
    }

}

impl Default for Bootkey011Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Bootkey011 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Bootkey011Val {
        Bootkey011Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Bootkey011Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Bootkey011Val) -> Bootkey011Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// BOOTKEY0_12 Register
///
/// Bits 207:192 of SHA-256 hash of boot key 0 (ECC)
///
/// Address offset: 0x0118
///
/// ## Fields
///
/// - **BOOTKEY0_12** `[15:0]` (RO)
#[repr(transparent)]
pub struct Bootkey012 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Bootkey012 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Bootkey012 {
    // ======== Field Constants ========

    /// **BOOTKEY0_12**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const BOOTKEY0_12_OFFSET: u32 = 0;
    pub const BOOTKEY0_12_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **BOOTKEY0_12** `[15:0]` (RO)
    #[inline(always)]
    pub fn bootkey0_12(&self) -> u32 {
        (self.value.read_bits() & Self::BOOTKEY0_12_MASK) >> Self::BOOTKEY0_12_OFFSET
    }

}

/// BOOTKEY0_12 value type for batch read/modify/write operations
///
/// Use `bootkey012.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootkey012.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bootkey012Val {
    bits: u32,
}

impl Bootkey012Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **BOOTKEY0_12** `[15:0]`
    #[inline(always)]
    pub const fn bootkey0_12(&self) -> u32 {
        (self.bits & Bootkey012::BOOTKEY0_12_MASK) >> Bootkey012::BOOTKEY0_12_OFFSET
    }

}

impl Default for Bootkey012Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Bootkey012 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Bootkey012Val {
        Bootkey012Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Bootkey012Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Bootkey012Val) -> Bootkey012Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// BOOTKEY0_13 Register
///
/// Bits 223:208 of SHA-256 hash of boot key 0 (ECC)
///
/// Address offset: 0x011a
///
/// ## Fields
///
/// - **BOOTKEY0_13** `[15:0]` (RO)
#[repr(transparent)]
pub struct Bootkey013 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Bootkey013 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Bootkey013 {
    // ======== Field Constants ========

    /// **BOOTKEY0_13**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const BOOTKEY0_13_OFFSET: u32 = 0;
    pub const BOOTKEY0_13_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **BOOTKEY0_13** `[15:0]` (RO)
    #[inline(always)]
    pub fn bootkey0_13(&self) -> u32 {
        (self.value.read_bits() & Self::BOOTKEY0_13_MASK) >> Self::BOOTKEY0_13_OFFSET
    }

}

/// BOOTKEY0_13 value type for batch read/modify/write operations
///
/// Use `bootkey013.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootkey013.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bootkey013Val {
    bits: u32,
}

impl Bootkey013Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **BOOTKEY0_13** `[15:0]`
    #[inline(always)]
    pub const fn bootkey0_13(&self) -> u32 {
        (self.bits & Bootkey013::BOOTKEY0_13_MASK) >> Bootkey013::BOOTKEY0_13_OFFSET
    }

}

impl Default for Bootkey013Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Bootkey013 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Bootkey013Val {
        Bootkey013Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Bootkey013Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Bootkey013Val) -> Bootkey013Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// BOOTKEY0_14 Register
///
/// Bits 239:224 of SHA-256 hash of boot key 0 (ECC)
///
/// Address offset: 0x011c
///
/// ## Fields
///
/// - **BOOTKEY0_14** `[15:0]` (RO)
#[repr(transparent)]
pub struct Bootkey014 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Bootkey014 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Bootkey014 {
    // ======== Field Constants ========

    /// **BOOTKEY0_14**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const BOOTKEY0_14_OFFSET: u32 = 0;
    pub const BOOTKEY0_14_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **BOOTKEY0_14** `[15:0]` (RO)
    #[inline(always)]
    pub fn bootkey0_14(&self) -> u32 {
        (self.value.read_bits() & Self::BOOTKEY0_14_MASK) >> Self::BOOTKEY0_14_OFFSET
    }

}

/// BOOTKEY0_14 value type for batch read/modify/write operations
///
/// Use `bootkey014.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootkey014.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bootkey014Val {
    bits: u32,
}

impl Bootkey014Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **BOOTKEY0_14** `[15:0]`
    #[inline(always)]
    pub const fn bootkey0_14(&self) -> u32 {
        (self.bits & Bootkey014::BOOTKEY0_14_MASK) >> Bootkey014::BOOTKEY0_14_OFFSET
    }

}

impl Default for Bootkey014Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Bootkey014 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Bootkey014Val {
        Bootkey014Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Bootkey014Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Bootkey014Val) -> Bootkey014Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// BOOTKEY0_15 Register
///
/// Bits 255:240 of SHA-256 hash of boot key 0 (ECC)
///
/// Address offset: 0x011e
///
/// ## Fields
///
/// - **BOOTKEY0_15** `[15:0]` (RO)
#[repr(transparent)]
pub struct Bootkey015 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Bootkey015 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Bootkey015 {
    // ======== Field Constants ========

    /// **BOOTKEY0_15**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const BOOTKEY0_15_OFFSET: u32 = 0;
    pub const BOOTKEY0_15_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **BOOTKEY0_15** `[15:0]` (RO)
    #[inline(always)]
    pub fn bootkey0_15(&self) -> u32 {
        (self.value.read_bits() & Self::BOOTKEY0_15_MASK) >> Self::BOOTKEY0_15_OFFSET
    }

}

/// BOOTKEY0_15 value type for batch read/modify/write operations
///
/// Use `bootkey015.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootkey015.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bootkey015Val {
    bits: u32,
}

impl Bootkey015Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **BOOTKEY0_15** `[15:0]`
    #[inline(always)]
    pub const fn bootkey0_15(&self) -> u32 {
        (self.bits & Bootkey015::BOOTKEY0_15_MASK) >> Bootkey015::BOOTKEY0_15_OFFSET
    }

}

impl Default for Bootkey015Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Bootkey015 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Bootkey015Val {
        Bootkey015Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Bootkey015Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Bootkey015Val) -> Bootkey015Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// BOOTKEY1_0 Register
///
/// Bits 15:0 of SHA-256 hash of boot key 1 (ECC)
///
/// Address offset: 0x0120
///
/// ## Fields
///
/// - **BOOTKEY1_0** `[15:0]` (RO)
#[repr(transparent)]
pub struct Bootkey10 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Bootkey10 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Bootkey10 {
    // ======== Field Constants ========

    /// **BOOTKEY1_0**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const BOOTKEY1_0_OFFSET: u32 = 0;
    pub const BOOTKEY1_0_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **BOOTKEY1_0** `[15:0]` (RO)
    #[inline(always)]
    pub fn bootkey1_0(&self) -> u32 {
        (self.value.read_bits() & Self::BOOTKEY1_0_MASK) >> Self::BOOTKEY1_0_OFFSET
    }

}

/// BOOTKEY1_0 value type for batch read/modify/write operations
///
/// Use `bootkey10.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootkey10.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bootkey10Val {
    bits: u32,
}

impl Bootkey10Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **BOOTKEY1_0** `[15:0]`
    #[inline(always)]
    pub const fn bootkey1_0(&self) -> u32 {
        (self.bits & Bootkey10::BOOTKEY1_0_MASK) >> Bootkey10::BOOTKEY1_0_OFFSET
    }

}

impl Default for Bootkey10Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Bootkey10 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Bootkey10Val {
        Bootkey10Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Bootkey10Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Bootkey10Val) -> Bootkey10Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// BOOTKEY1_1 Register
///
/// Bits 31:16 of SHA-256 hash of boot key 1 (ECC)
///
/// Address offset: 0x0122
///
/// ## Fields
///
/// - **BOOTKEY1_1** `[15:0]` (RO)
#[repr(transparent)]
pub struct Bootkey11 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Bootkey11 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Bootkey11 {
    // ======== Field Constants ========

    /// **BOOTKEY1_1**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const BOOTKEY1_1_OFFSET: u32 = 0;
    pub const BOOTKEY1_1_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **BOOTKEY1_1** `[15:0]` (RO)
    #[inline(always)]
    pub fn bootkey1_1(&self) -> u32 {
        (self.value.read_bits() & Self::BOOTKEY1_1_MASK) >> Self::BOOTKEY1_1_OFFSET
    }

}

/// BOOTKEY1_1 value type for batch read/modify/write operations
///
/// Use `bootkey11.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootkey11.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bootkey11Val {
    bits: u32,
}

impl Bootkey11Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **BOOTKEY1_1** `[15:0]`
    #[inline(always)]
    pub const fn bootkey1_1(&self) -> u32 {
        (self.bits & Bootkey11::BOOTKEY1_1_MASK) >> Bootkey11::BOOTKEY1_1_OFFSET
    }

}

impl Default for Bootkey11Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Bootkey11 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Bootkey11Val {
        Bootkey11Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Bootkey11Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Bootkey11Val) -> Bootkey11Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// BOOTKEY1_2 Register
///
/// Bits 47:32 of SHA-256 hash of boot key 1 (ECC)
///
/// Address offset: 0x0124
///
/// ## Fields
///
/// - **BOOTKEY1_2** `[15:0]` (RO)
#[repr(transparent)]
pub struct Bootkey12 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Bootkey12 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Bootkey12 {
    // ======== Field Constants ========

    /// **BOOTKEY1_2**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const BOOTKEY1_2_OFFSET: u32 = 0;
    pub const BOOTKEY1_2_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **BOOTKEY1_2** `[15:0]` (RO)
    #[inline(always)]
    pub fn bootkey1_2(&self) -> u32 {
        (self.value.read_bits() & Self::BOOTKEY1_2_MASK) >> Self::BOOTKEY1_2_OFFSET
    }

}

/// BOOTKEY1_2 value type for batch read/modify/write operations
///
/// Use `bootkey12.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootkey12.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bootkey12Val {
    bits: u32,
}

impl Bootkey12Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **BOOTKEY1_2** `[15:0]`
    #[inline(always)]
    pub const fn bootkey1_2(&self) -> u32 {
        (self.bits & Bootkey12::BOOTKEY1_2_MASK) >> Bootkey12::BOOTKEY1_2_OFFSET
    }

}

impl Default for Bootkey12Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Bootkey12 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Bootkey12Val {
        Bootkey12Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Bootkey12Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Bootkey12Val) -> Bootkey12Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// BOOTKEY1_3 Register
///
/// Bits 63:48 of SHA-256 hash of boot key 1 (ECC)
///
/// Address offset: 0x0126
///
/// ## Fields
///
/// - **BOOTKEY1_3** `[15:0]` (RO)
#[repr(transparent)]
pub struct Bootkey13 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Bootkey13 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Bootkey13 {
    // ======== Field Constants ========

    /// **BOOTKEY1_3**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const BOOTKEY1_3_OFFSET: u32 = 0;
    pub const BOOTKEY1_3_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **BOOTKEY1_3** `[15:0]` (RO)
    #[inline(always)]
    pub fn bootkey1_3(&self) -> u32 {
        (self.value.read_bits() & Self::BOOTKEY1_3_MASK) >> Self::BOOTKEY1_3_OFFSET
    }

}

/// BOOTKEY1_3 value type for batch read/modify/write operations
///
/// Use `bootkey13.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootkey13.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bootkey13Val {
    bits: u32,
}

impl Bootkey13Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **BOOTKEY1_3** `[15:0]`
    #[inline(always)]
    pub const fn bootkey1_3(&self) -> u32 {
        (self.bits & Bootkey13::BOOTKEY1_3_MASK) >> Bootkey13::BOOTKEY1_3_OFFSET
    }

}

impl Default for Bootkey13Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Bootkey13 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Bootkey13Val {
        Bootkey13Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Bootkey13Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Bootkey13Val) -> Bootkey13Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// BOOTKEY1_4 Register
///
/// Bits 79:64 of SHA-256 hash of boot key 1 (ECC)
///
/// Address offset: 0x0128
///
/// ## Fields
///
/// - **BOOTKEY1_4** `[15:0]` (RO)
#[repr(transparent)]
pub struct Bootkey14 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Bootkey14 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Bootkey14 {
    // ======== Field Constants ========

    /// **BOOTKEY1_4**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const BOOTKEY1_4_OFFSET: u32 = 0;
    pub const BOOTKEY1_4_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **BOOTKEY1_4** `[15:0]` (RO)
    #[inline(always)]
    pub fn bootkey1_4(&self) -> u32 {
        (self.value.read_bits() & Self::BOOTKEY1_4_MASK) >> Self::BOOTKEY1_4_OFFSET
    }

}

/// BOOTKEY1_4 value type for batch read/modify/write operations
///
/// Use `bootkey14.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootkey14.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bootkey14Val {
    bits: u32,
}

impl Bootkey14Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **BOOTKEY1_4** `[15:0]`
    #[inline(always)]
    pub const fn bootkey1_4(&self) -> u32 {
        (self.bits & Bootkey14::BOOTKEY1_4_MASK) >> Bootkey14::BOOTKEY1_4_OFFSET
    }

}

impl Default for Bootkey14Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Bootkey14 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Bootkey14Val {
        Bootkey14Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Bootkey14Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Bootkey14Val) -> Bootkey14Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// BOOTKEY1_5 Register
///
/// Bits 95:80 of SHA-256 hash of boot key 1 (ECC)
///
/// Address offset: 0x012a
///
/// ## Fields
///
/// - **BOOTKEY1_5** `[15:0]` (RO)
#[repr(transparent)]
pub struct Bootkey15 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Bootkey15 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Bootkey15 {
    // ======== Field Constants ========

    /// **BOOTKEY1_5**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const BOOTKEY1_5_OFFSET: u32 = 0;
    pub const BOOTKEY1_5_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **BOOTKEY1_5** `[15:0]` (RO)
    #[inline(always)]
    pub fn bootkey1_5(&self) -> u32 {
        (self.value.read_bits() & Self::BOOTKEY1_5_MASK) >> Self::BOOTKEY1_5_OFFSET
    }

}

/// BOOTKEY1_5 value type for batch read/modify/write operations
///
/// Use `bootkey15.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootkey15.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bootkey15Val {
    bits: u32,
}

impl Bootkey15Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **BOOTKEY1_5** `[15:0]`
    #[inline(always)]
    pub const fn bootkey1_5(&self) -> u32 {
        (self.bits & Bootkey15::BOOTKEY1_5_MASK) >> Bootkey15::BOOTKEY1_5_OFFSET
    }

}

impl Default for Bootkey15Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Bootkey15 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Bootkey15Val {
        Bootkey15Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Bootkey15Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Bootkey15Val) -> Bootkey15Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// BOOTKEY1_6 Register
///
/// Bits 111:96 of SHA-256 hash of boot key 1 (ECC)
///
/// Address offset: 0x012c
///
/// ## Fields
///
/// - **BOOTKEY1_6** `[15:0]` (RO)
#[repr(transparent)]
pub struct Bootkey16 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Bootkey16 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Bootkey16 {
    // ======== Field Constants ========

    /// **BOOTKEY1_6**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const BOOTKEY1_6_OFFSET: u32 = 0;
    pub const BOOTKEY1_6_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **BOOTKEY1_6** `[15:0]` (RO)
    #[inline(always)]
    pub fn bootkey1_6(&self) -> u32 {
        (self.value.read_bits() & Self::BOOTKEY1_6_MASK) >> Self::BOOTKEY1_6_OFFSET
    }

}

/// BOOTKEY1_6 value type for batch read/modify/write operations
///
/// Use `bootkey16.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootkey16.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bootkey16Val {
    bits: u32,
}

impl Bootkey16Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **BOOTKEY1_6** `[15:0]`
    #[inline(always)]
    pub const fn bootkey1_6(&self) -> u32 {
        (self.bits & Bootkey16::BOOTKEY1_6_MASK) >> Bootkey16::BOOTKEY1_6_OFFSET
    }

}

impl Default for Bootkey16Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Bootkey16 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Bootkey16Val {
        Bootkey16Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Bootkey16Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Bootkey16Val) -> Bootkey16Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// BOOTKEY1_7 Register
///
/// Bits 127:112 of SHA-256 hash of boot key 1 (ECC)
///
/// Address offset: 0x012e
///
/// ## Fields
///
/// - **BOOTKEY1_7** `[15:0]` (RO)
#[repr(transparent)]
pub struct Bootkey17 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Bootkey17 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Bootkey17 {
    // ======== Field Constants ========

    /// **BOOTKEY1_7**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const BOOTKEY1_7_OFFSET: u32 = 0;
    pub const BOOTKEY1_7_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **BOOTKEY1_7** `[15:0]` (RO)
    #[inline(always)]
    pub fn bootkey1_7(&self) -> u32 {
        (self.value.read_bits() & Self::BOOTKEY1_7_MASK) >> Self::BOOTKEY1_7_OFFSET
    }

}

/// BOOTKEY1_7 value type for batch read/modify/write operations
///
/// Use `bootkey17.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootkey17.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bootkey17Val {
    bits: u32,
}

impl Bootkey17Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **BOOTKEY1_7** `[15:0]`
    #[inline(always)]
    pub const fn bootkey1_7(&self) -> u32 {
        (self.bits & Bootkey17::BOOTKEY1_7_MASK) >> Bootkey17::BOOTKEY1_7_OFFSET
    }

}

impl Default for Bootkey17Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Bootkey17 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Bootkey17Val {
        Bootkey17Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Bootkey17Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Bootkey17Val) -> Bootkey17Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// BOOTKEY1_8 Register
///
/// Bits 143:128 of SHA-256 hash of boot key 1 (ECC)
///
/// Address offset: 0x0130
///
/// ## Fields
///
/// - **BOOTKEY1_8** `[15:0]` (RO)
#[repr(transparent)]
pub struct Bootkey18 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Bootkey18 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Bootkey18 {
    // ======== Field Constants ========

    /// **BOOTKEY1_8**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const BOOTKEY1_8_OFFSET: u32 = 0;
    pub const BOOTKEY1_8_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **BOOTKEY1_8** `[15:0]` (RO)
    #[inline(always)]
    pub fn bootkey1_8(&self) -> u32 {
        (self.value.read_bits() & Self::BOOTKEY1_8_MASK) >> Self::BOOTKEY1_8_OFFSET
    }

}

/// BOOTKEY1_8 value type for batch read/modify/write operations
///
/// Use `bootkey18.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootkey18.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bootkey18Val {
    bits: u32,
}

impl Bootkey18Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **BOOTKEY1_8** `[15:0]`
    #[inline(always)]
    pub const fn bootkey1_8(&self) -> u32 {
        (self.bits & Bootkey18::BOOTKEY1_8_MASK) >> Bootkey18::BOOTKEY1_8_OFFSET
    }

}

impl Default for Bootkey18Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Bootkey18 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Bootkey18Val {
        Bootkey18Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Bootkey18Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Bootkey18Val) -> Bootkey18Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// BOOTKEY1_9 Register
///
/// Bits 159:144 of SHA-256 hash of boot key 1 (ECC)
///
/// Address offset: 0x0132
///
/// ## Fields
///
/// - **BOOTKEY1_9** `[15:0]` (RO)
#[repr(transparent)]
pub struct Bootkey19 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Bootkey19 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Bootkey19 {
    // ======== Field Constants ========

    /// **BOOTKEY1_9**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const BOOTKEY1_9_OFFSET: u32 = 0;
    pub const BOOTKEY1_9_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **BOOTKEY1_9** `[15:0]` (RO)
    #[inline(always)]
    pub fn bootkey1_9(&self) -> u32 {
        (self.value.read_bits() & Self::BOOTKEY1_9_MASK) >> Self::BOOTKEY1_9_OFFSET
    }

}

/// BOOTKEY1_9 value type for batch read/modify/write operations
///
/// Use `bootkey19.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootkey19.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bootkey19Val {
    bits: u32,
}

impl Bootkey19Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **BOOTKEY1_9** `[15:0]`
    #[inline(always)]
    pub const fn bootkey1_9(&self) -> u32 {
        (self.bits & Bootkey19::BOOTKEY1_9_MASK) >> Bootkey19::BOOTKEY1_9_OFFSET
    }

}

impl Default for Bootkey19Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Bootkey19 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Bootkey19Val {
        Bootkey19Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Bootkey19Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Bootkey19Val) -> Bootkey19Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// BOOTKEY1_10 Register
///
/// Bits 175:160 of SHA-256 hash of boot key 1 (ECC)
///
/// Address offset: 0x0134
///
/// ## Fields
///
/// - **BOOTKEY1_10** `[15:0]` (RO)
#[repr(transparent)]
pub struct Bootkey110 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Bootkey110 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Bootkey110 {
    // ======== Field Constants ========

    /// **BOOTKEY1_10**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const BOOTKEY1_10_OFFSET: u32 = 0;
    pub const BOOTKEY1_10_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **BOOTKEY1_10** `[15:0]` (RO)
    #[inline(always)]
    pub fn bootkey1_10(&self) -> u32 {
        (self.value.read_bits() & Self::BOOTKEY1_10_MASK) >> Self::BOOTKEY1_10_OFFSET
    }

}

/// BOOTKEY1_10 value type for batch read/modify/write operations
///
/// Use `bootkey110.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootkey110.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bootkey110Val {
    bits: u32,
}

impl Bootkey110Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **BOOTKEY1_10** `[15:0]`
    #[inline(always)]
    pub const fn bootkey1_10(&self) -> u32 {
        (self.bits & Bootkey110::BOOTKEY1_10_MASK) >> Bootkey110::BOOTKEY1_10_OFFSET
    }

}

impl Default for Bootkey110Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Bootkey110 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Bootkey110Val {
        Bootkey110Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Bootkey110Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Bootkey110Val) -> Bootkey110Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// BOOTKEY1_11 Register
///
/// Bits 191:176 of SHA-256 hash of boot key 1 (ECC)
///
/// Address offset: 0x0136
///
/// ## Fields
///
/// - **BOOTKEY1_11** `[15:0]` (RO)
#[repr(transparent)]
pub struct Bootkey111 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Bootkey111 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Bootkey111 {
    // ======== Field Constants ========

    /// **BOOTKEY1_11**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const BOOTKEY1_11_OFFSET: u32 = 0;
    pub const BOOTKEY1_11_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **BOOTKEY1_11** `[15:0]` (RO)
    #[inline(always)]
    pub fn bootkey1_11(&self) -> u32 {
        (self.value.read_bits() & Self::BOOTKEY1_11_MASK) >> Self::BOOTKEY1_11_OFFSET
    }

}

/// BOOTKEY1_11 value type for batch read/modify/write operations
///
/// Use `bootkey111.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootkey111.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bootkey111Val {
    bits: u32,
}

impl Bootkey111Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **BOOTKEY1_11** `[15:0]`
    #[inline(always)]
    pub const fn bootkey1_11(&self) -> u32 {
        (self.bits & Bootkey111::BOOTKEY1_11_MASK) >> Bootkey111::BOOTKEY1_11_OFFSET
    }

}

impl Default for Bootkey111Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Bootkey111 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Bootkey111Val {
        Bootkey111Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Bootkey111Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Bootkey111Val) -> Bootkey111Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// BOOTKEY1_12 Register
///
/// Bits 207:192 of SHA-256 hash of boot key 1 (ECC)
///
/// Address offset: 0x0138
///
/// ## Fields
///
/// - **BOOTKEY1_12** `[15:0]` (RO)
#[repr(transparent)]
pub struct Bootkey112 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Bootkey112 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Bootkey112 {
    // ======== Field Constants ========

    /// **BOOTKEY1_12**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const BOOTKEY1_12_OFFSET: u32 = 0;
    pub const BOOTKEY1_12_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **BOOTKEY1_12** `[15:0]` (RO)
    #[inline(always)]
    pub fn bootkey1_12(&self) -> u32 {
        (self.value.read_bits() & Self::BOOTKEY1_12_MASK) >> Self::BOOTKEY1_12_OFFSET
    }

}

/// BOOTKEY1_12 value type for batch read/modify/write operations
///
/// Use `bootkey112.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootkey112.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bootkey112Val {
    bits: u32,
}

impl Bootkey112Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **BOOTKEY1_12** `[15:0]`
    #[inline(always)]
    pub const fn bootkey1_12(&self) -> u32 {
        (self.bits & Bootkey112::BOOTKEY1_12_MASK) >> Bootkey112::BOOTKEY1_12_OFFSET
    }

}

impl Default for Bootkey112Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Bootkey112 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Bootkey112Val {
        Bootkey112Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Bootkey112Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Bootkey112Val) -> Bootkey112Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// BOOTKEY1_13 Register
///
/// Bits 223:208 of SHA-256 hash of boot key 1 (ECC)
///
/// Address offset: 0x013a
///
/// ## Fields
///
/// - **BOOTKEY1_13** `[15:0]` (RO)
#[repr(transparent)]
pub struct Bootkey113 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Bootkey113 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Bootkey113 {
    // ======== Field Constants ========

    /// **BOOTKEY1_13**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const BOOTKEY1_13_OFFSET: u32 = 0;
    pub const BOOTKEY1_13_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **BOOTKEY1_13** `[15:0]` (RO)
    #[inline(always)]
    pub fn bootkey1_13(&self) -> u32 {
        (self.value.read_bits() & Self::BOOTKEY1_13_MASK) >> Self::BOOTKEY1_13_OFFSET
    }

}

/// BOOTKEY1_13 value type for batch read/modify/write operations
///
/// Use `bootkey113.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootkey113.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bootkey113Val {
    bits: u32,
}

impl Bootkey113Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **BOOTKEY1_13** `[15:0]`
    #[inline(always)]
    pub const fn bootkey1_13(&self) -> u32 {
        (self.bits & Bootkey113::BOOTKEY1_13_MASK) >> Bootkey113::BOOTKEY1_13_OFFSET
    }

}

impl Default for Bootkey113Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Bootkey113 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Bootkey113Val {
        Bootkey113Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Bootkey113Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Bootkey113Val) -> Bootkey113Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// BOOTKEY1_14 Register
///
/// Bits 239:224 of SHA-256 hash of boot key 1 (ECC)
///
/// Address offset: 0x013c
///
/// ## Fields
///
/// - **BOOTKEY1_14** `[15:0]` (RO)
#[repr(transparent)]
pub struct Bootkey114 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Bootkey114 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Bootkey114 {
    // ======== Field Constants ========

    /// **BOOTKEY1_14**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const BOOTKEY1_14_OFFSET: u32 = 0;
    pub const BOOTKEY1_14_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **BOOTKEY1_14** `[15:0]` (RO)
    #[inline(always)]
    pub fn bootkey1_14(&self) -> u32 {
        (self.value.read_bits() & Self::BOOTKEY1_14_MASK) >> Self::BOOTKEY1_14_OFFSET
    }

}

/// BOOTKEY1_14 value type for batch read/modify/write operations
///
/// Use `bootkey114.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootkey114.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bootkey114Val {
    bits: u32,
}

impl Bootkey114Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **BOOTKEY1_14** `[15:0]`
    #[inline(always)]
    pub const fn bootkey1_14(&self) -> u32 {
        (self.bits & Bootkey114::BOOTKEY1_14_MASK) >> Bootkey114::BOOTKEY1_14_OFFSET
    }

}

impl Default for Bootkey114Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Bootkey114 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Bootkey114Val {
        Bootkey114Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Bootkey114Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Bootkey114Val) -> Bootkey114Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// BOOTKEY1_15 Register
///
/// Bits 255:240 of SHA-256 hash of boot key 1 (ECC)
///
/// Address offset: 0x013e
///
/// ## Fields
///
/// - **BOOTKEY1_15** `[15:0]` (RO)
#[repr(transparent)]
pub struct Bootkey115 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Bootkey115 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Bootkey115 {
    // ======== Field Constants ========

    /// **BOOTKEY1_15**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const BOOTKEY1_15_OFFSET: u32 = 0;
    pub const BOOTKEY1_15_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **BOOTKEY1_15** `[15:0]` (RO)
    #[inline(always)]
    pub fn bootkey1_15(&self) -> u32 {
        (self.value.read_bits() & Self::BOOTKEY1_15_MASK) >> Self::BOOTKEY1_15_OFFSET
    }

}

/// BOOTKEY1_15 value type for batch read/modify/write operations
///
/// Use `bootkey115.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootkey115.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bootkey115Val {
    bits: u32,
}

impl Bootkey115Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **BOOTKEY1_15** `[15:0]`
    #[inline(always)]
    pub const fn bootkey1_15(&self) -> u32 {
        (self.bits & Bootkey115::BOOTKEY1_15_MASK) >> Bootkey115::BOOTKEY1_15_OFFSET
    }

}

impl Default for Bootkey115Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Bootkey115 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Bootkey115Val {
        Bootkey115Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Bootkey115Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Bootkey115Val) -> Bootkey115Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// BOOTKEY2_0 Register
///
/// Bits 15:0 of SHA-256 hash of boot key 2 (ECC)
///
/// Address offset: 0x0140
///
/// ## Fields
///
/// - **BOOTKEY2_0** `[15:0]` (RO)
#[repr(transparent)]
pub struct Bootkey20 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Bootkey20 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Bootkey20 {
    // ======== Field Constants ========

    /// **BOOTKEY2_0**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const BOOTKEY2_0_OFFSET: u32 = 0;
    pub const BOOTKEY2_0_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **BOOTKEY2_0** `[15:0]` (RO)
    #[inline(always)]
    pub fn bootkey2_0(&self) -> u32 {
        (self.value.read_bits() & Self::BOOTKEY2_0_MASK) >> Self::BOOTKEY2_0_OFFSET
    }

}

/// BOOTKEY2_0 value type for batch read/modify/write operations
///
/// Use `bootkey20.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootkey20.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bootkey20Val {
    bits: u32,
}

impl Bootkey20Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **BOOTKEY2_0** `[15:0]`
    #[inline(always)]
    pub const fn bootkey2_0(&self) -> u32 {
        (self.bits & Bootkey20::BOOTKEY2_0_MASK) >> Bootkey20::BOOTKEY2_0_OFFSET
    }

}

impl Default for Bootkey20Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Bootkey20 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Bootkey20Val {
        Bootkey20Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Bootkey20Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Bootkey20Val) -> Bootkey20Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// BOOTKEY2_1 Register
///
/// Bits 31:16 of SHA-256 hash of boot key 2 (ECC)
///
/// Address offset: 0x0142
///
/// ## Fields
///
/// - **BOOTKEY2_1** `[15:0]` (RO)
#[repr(transparent)]
pub struct Bootkey21 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Bootkey21 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Bootkey21 {
    // ======== Field Constants ========

    /// **BOOTKEY2_1**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const BOOTKEY2_1_OFFSET: u32 = 0;
    pub const BOOTKEY2_1_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **BOOTKEY2_1** `[15:0]` (RO)
    #[inline(always)]
    pub fn bootkey2_1(&self) -> u32 {
        (self.value.read_bits() & Self::BOOTKEY2_1_MASK) >> Self::BOOTKEY2_1_OFFSET
    }

}

/// BOOTKEY2_1 value type for batch read/modify/write operations
///
/// Use `bootkey21.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootkey21.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bootkey21Val {
    bits: u32,
}

impl Bootkey21Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **BOOTKEY2_1** `[15:0]`
    #[inline(always)]
    pub const fn bootkey2_1(&self) -> u32 {
        (self.bits & Bootkey21::BOOTKEY2_1_MASK) >> Bootkey21::BOOTKEY2_1_OFFSET
    }

}

impl Default for Bootkey21Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Bootkey21 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Bootkey21Val {
        Bootkey21Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Bootkey21Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Bootkey21Val) -> Bootkey21Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// BOOTKEY2_2 Register
///
/// Bits 47:32 of SHA-256 hash of boot key 2 (ECC)
///
/// Address offset: 0x0144
///
/// ## Fields
///
/// - **BOOTKEY2_2** `[15:0]` (RO)
#[repr(transparent)]
pub struct Bootkey22 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Bootkey22 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Bootkey22 {
    // ======== Field Constants ========

    /// **BOOTKEY2_2**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const BOOTKEY2_2_OFFSET: u32 = 0;
    pub const BOOTKEY2_2_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **BOOTKEY2_2** `[15:0]` (RO)
    #[inline(always)]
    pub fn bootkey2_2(&self) -> u32 {
        (self.value.read_bits() & Self::BOOTKEY2_2_MASK) >> Self::BOOTKEY2_2_OFFSET
    }

}

/// BOOTKEY2_2 value type for batch read/modify/write operations
///
/// Use `bootkey22.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootkey22.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bootkey22Val {
    bits: u32,
}

impl Bootkey22Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **BOOTKEY2_2** `[15:0]`
    #[inline(always)]
    pub const fn bootkey2_2(&self) -> u32 {
        (self.bits & Bootkey22::BOOTKEY2_2_MASK) >> Bootkey22::BOOTKEY2_2_OFFSET
    }

}

impl Default for Bootkey22Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Bootkey22 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Bootkey22Val {
        Bootkey22Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Bootkey22Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Bootkey22Val) -> Bootkey22Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// BOOTKEY2_3 Register
///
/// Bits 63:48 of SHA-256 hash of boot key 2 (ECC)
///
/// Address offset: 0x0146
///
/// ## Fields
///
/// - **BOOTKEY2_3** `[15:0]` (RO)
#[repr(transparent)]
pub struct Bootkey23 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Bootkey23 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Bootkey23 {
    // ======== Field Constants ========

    /// **BOOTKEY2_3**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const BOOTKEY2_3_OFFSET: u32 = 0;
    pub const BOOTKEY2_3_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **BOOTKEY2_3** `[15:0]` (RO)
    #[inline(always)]
    pub fn bootkey2_3(&self) -> u32 {
        (self.value.read_bits() & Self::BOOTKEY2_3_MASK) >> Self::BOOTKEY2_3_OFFSET
    }

}

/// BOOTKEY2_3 value type for batch read/modify/write operations
///
/// Use `bootkey23.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootkey23.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bootkey23Val {
    bits: u32,
}

impl Bootkey23Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **BOOTKEY2_3** `[15:0]`
    #[inline(always)]
    pub const fn bootkey2_3(&self) -> u32 {
        (self.bits & Bootkey23::BOOTKEY2_3_MASK) >> Bootkey23::BOOTKEY2_3_OFFSET
    }

}

impl Default for Bootkey23Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Bootkey23 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Bootkey23Val {
        Bootkey23Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Bootkey23Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Bootkey23Val) -> Bootkey23Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// BOOTKEY2_4 Register
///
/// Bits 79:64 of SHA-256 hash of boot key 2 (ECC)
///
/// Address offset: 0x0148
///
/// ## Fields
///
/// - **BOOTKEY2_4** `[15:0]` (RO)
#[repr(transparent)]
pub struct Bootkey24 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Bootkey24 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Bootkey24 {
    // ======== Field Constants ========

    /// **BOOTKEY2_4**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const BOOTKEY2_4_OFFSET: u32 = 0;
    pub const BOOTKEY2_4_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **BOOTKEY2_4** `[15:0]` (RO)
    #[inline(always)]
    pub fn bootkey2_4(&self) -> u32 {
        (self.value.read_bits() & Self::BOOTKEY2_4_MASK) >> Self::BOOTKEY2_4_OFFSET
    }

}

/// BOOTKEY2_4 value type for batch read/modify/write operations
///
/// Use `bootkey24.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootkey24.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bootkey24Val {
    bits: u32,
}

impl Bootkey24Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **BOOTKEY2_4** `[15:0]`
    #[inline(always)]
    pub const fn bootkey2_4(&self) -> u32 {
        (self.bits & Bootkey24::BOOTKEY2_4_MASK) >> Bootkey24::BOOTKEY2_4_OFFSET
    }

}

impl Default for Bootkey24Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Bootkey24 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Bootkey24Val {
        Bootkey24Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Bootkey24Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Bootkey24Val) -> Bootkey24Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// BOOTKEY2_5 Register
///
/// Bits 95:80 of SHA-256 hash of boot key 2 (ECC)
///
/// Address offset: 0x014a
///
/// ## Fields
///
/// - **BOOTKEY2_5** `[15:0]` (RO)
#[repr(transparent)]
pub struct Bootkey25 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Bootkey25 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Bootkey25 {
    // ======== Field Constants ========

    /// **BOOTKEY2_5**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const BOOTKEY2_5_OFFSET: u32 = 0;
    pub const BOOTKEY2_5_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **BOOTKEY2_5** `[15:0]` (RO)
    #[inline(always)]
    pub fn bootkey2_5(&self) -> u32 {
        (self.value.read_bits() & Self::BOOTKEY2_5_MASK) >> Self::BOOTKEY2_5_OFFSET
    }

}

/// BOOTKEY2_5 value type for batch read/modify/write operations
///
/// Use `bootkey25.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootkey25.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bootkey25Val {
    bits: u32,
}

impl Bootkey25Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **BOOTKEY2_5** `[15:0]`
    #[inline(always)]
    pub const fn bootkey2_5(&self) -> u32 {
        (self.bits & Bootkey25::BOOTKEY2_5_MASK) >> Bootkey25::BOOTKEY2_5_OFFSET
    }

}

impl Default for Bootkey25Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Bootkey25 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Bootkey25Val {
        Bootkey25Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Bootkey25Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Bootkey25Val) -> Bootkey25Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// BOOTKEY2_6 Register
///
/// Bits 111:96 of SHA-256 hash of boot key 2 (ECC)
///
/// Address offset: 0x014c
///
/// ## Fields
///
/// - **BOOTKEY2_6** `[15:0]` (RO)
#[repr(transparent)]
pub struct Bootkey26 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Bootkey26 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Bootkey26 {
    // ======== Field Constants ========

    /// **BOOTKEY2_6**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const BOOTKEY2_6_OFFSET: u32 = 0;
    pub const BOOTKEY2_6_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **BOOTKEY2_6** `[15:0]` (RO)
    #[inline(always)]
    pub fn bootkey2_6(&self) -> u32 {
        (self.value.read_bits() & Self::BOOTKEY2_6_MASK) >> Self::BOOTKEY2_6_OFFSET
    }

}

/// BOOTKEY2_6 value type for batch read/modify/write operations
///
/// Use `bootkey26.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootkey26.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bootkey26Val {
    bits: u32,
}

impl Bootkey26Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **BOOTKEY2_6** `[15:0]`
    #[inline(always)]
    pub const fn bootkey2_6(&self) -> u32 {
        (self.bits & Bootkey26::BOOTKEY2_6_MASK) >> Bootkey26::BOOTKEY2_6_OFFSET
    }

}

impl Default for Bootkey26Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Bootkey26 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Bootkey26Val {
        Bootkey26Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Bootkey26Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Bootkey26Val) -> Bootkey26Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// BOOTKEY2_7 Register
///
/// Bits 127:112 of SHA-256 hash of boot key 2 (ECC)
///
/// Address offset: 0x014e
///
/// ## Fields
///
/// - **BOOTKEY2_7** `[15:0]` (RO)
#[repr(transparent)]
pub struct Bootkey27 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Bootkey27 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Bootkey27 {
    // ======== Field Constants ========

    /// **BOOTKEY2_7**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const BOOTKEY2_7_OFFSET: u32 = 0;
    pub const BOOTKEY2_7_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **BOOTKEY2_7** `[15:0]` (RO)
    #[inline(always)]
    pub fn bootkey2_7(&self) -> u32 {
        (self.value.read_bits() & Self::BOOTKEY2_7_MASK) >> Self::BOOTKEY2_7_OFFSET
    }

}

/// BOOTKEY2_7 value type for batch read/modify/write operations
///
/// Use `bootkey27.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootkey27.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bootkey27Val {
    bits: u32,
}

impl Bootkey27Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **BOOTKEY2_7** `[15:0]`
    #[inline(always)]
    pub const fn bootkey2_7(&self) -> u32 {
        (self.bits & Bootkey27::BOOTKEY2_7_MASK) >> Bootkey27::BOOTKEY2_7_OFFSET
    }

}

impl Default for Bootkey27Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Bootkey27 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Bootkey27Val {
        Bootkey27Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Bootkey27Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Bootkey27Val) -> Bootkey27Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// BOOTKEY2_8 Register
///
/// Bits 143:128 of SHA-256 hash of boot key 2 (ECC)
///
/// Address offset: 0x0150
///
/// ## Fields
///
/// - **BOOTKEY2_8** `[15:0]` (RO)
#[repr(transparent)]
pub struct Bootkey28 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Bootkey28 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Bootkey28 {
    // ======== Field Constants ========

    /// **BOOTKEY2_8**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const BOOTKEY2_8_OFFSET: u32 = 0;
    pub const BOOTKEY2_8_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **BOOTKEY2_8** `[15:0]` (RO)
    #[inline(always)]
    pub fn bootkey2_8(&self) -> u32 {
        (self.value.read_bits() & Self::BOOTKEY2_8_MASK) >> Self::BOOTKEY2_8_OFFSET
    }

}

/// BOOTKEY2_8 value type for batch read/modify/write operations
///
/// Use `bootkey28.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootkey28.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bootkey28Val {
    bits: u32,
}

impl Bootkey28Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **BOOTKEY2_8** `[15:0]`
    #[inline(always)]
    pub const fn bootkey2_8(&self) -> u32 {
        (self.bits & Bootkey28::BOOTKEY2_8_MASK) >> Bootkey28::BOOTKEY2_8_OFFSET
    }

}

impl Default for Bootkey28Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Bootkey28 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Bootkey28Val {
        Bootkey28Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Bootkey28Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Bootkey28Val) -> Bootkey28Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// BOOTKEY2_9 Register
///
/// Bits 159:144 of SHA-256 hash of boot key 2 (ECC)
///
/// Address offset: 0x0152
///
/// ## Fields
///
/// - **BOOTKEY2_9** `[15:0]` (RO)
#[repr(transparent)]
pub struct Bootkey29 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Bootkey29 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Bootkey29 {
    // ======== Field Constants ========

    /// **BOOTKEY2_9**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const BOOTKEY2_9_OFFSET: u32 = 0;
    pub const BOOTKEY2_9_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **BOOTKEY2_9** `[15:0]` (RO)
    #[inline(always)]
    pub fn bootkey2_9(&self) -> u32 {
        (self.value.read_bits() & Self::BOOTKEY2_9_MASK) >> Self::BOOTKEY2_9_OFFSET
    }

}

/// BOOTKEY2_9 value type for batch read/modify/write operations
///
/// Use `bootkey29.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootkey29.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bootkey29Val {
    bits: u32,
}

impl Bootkey29Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **BOOTKEY2_9** `[15:0]`
    #[inline(always)]
    pub const fn bootkey2_9(&self) -> u32 {
        (self.bits & Bootkey29::BOOTKEY2_9_MASK) >> Bootkey29::BOOTKEY2_9_OFFSET
    }

}

impl Default for Bootkey29Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Bootkey29 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Bootkey29Val {
        Bootkey29Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Bootkey29Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Bootkey29Val) -> Bootkey29Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// BOOTKEY2_10 Register
///
/// Bits 175:160 of SHA-256 hash of boot key 2 (ECC)
///
/// Address offset: 0x0154
///
/// ## Fields
///
/// - **BOOTKEY2_10** `[15:0]` (RO)
#[repr(transparent)]
pub struct Bootkey210 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Bootkey210 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Bootkey210 {
    // ======== Field Constants ========

    /// **BOOTKEY2_10**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const BOOTKEY2_10_OFFSET: u32 = 0;
    pub const BOOTKEY2_10_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **BOOTKEY2_10** `[15:0]` (RO)
    #[inline(always)]
    pub fn bootkey2_10(&self) -> u32 {
        (self.value.read_bits() & Self::BOOTKEY2_10_MASK) >> Self::BOOTKEY2_10_OFFSET
    }

}

/// BOOTKEY2_10 value type for batch read/modify/write operations
///
/// Use `bootkey210.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootkey210.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bootkey210Val {
    bits: u32,
}

impl Bootkey210Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **BOOTKEY2_10** `[15:0]`
    #[inline(always)]
    pub const fn bootkey2_10(&self) -> u32 {
        (self.bits & Bootkey210::BOOTKEY2_10_MASK) >> Bootkey210::BOOTKEY2_10_OFFSET
    }

}

impl Default for Bootkey210Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Bootkey210 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Bootkey210Val {
        Bootkey210Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Bootkey210Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Bootkey210Val) -> Bootkey210Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// BOOTKEY2_11 Register
///
/// Bits 191:176 of SHA-256 hash of boot key 2 (ECC)
///
/// Address offset: 0x0156
///
/// ## Fields
///
/// - **BOOTKEY2_11** `[15:0]` (RO)
#[repr(transparent)]
pub struct Bootkey211 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Bootkey211 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Bootkey211 {
    // ======== Field Constants ========

    /// **BOOTKEY2_11**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const BOOTKEY2_11_OFFSET: u32 = 0;
    pub const BOOTKEY2_11_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **BOOTKEY2_11** `[15:0]` (RO)
    #[inline(always)]
    pub fn bootkey2_11(&self) -> u32 {
        (self.value.read_bits() & Self::BOOTKEY2_11_MASK) >> Self::BOOTKEY2_11_OFFSET
    }

}

/// BOOTKEY2_11 value type for batch read/modify/write operations
///
/// Use `bootkey211.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootkey211.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bootkey211Val {
    bits: u32,
}

impl Bootkey211Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **BOOTKEY2_11** `[15:0]`
    #[inline(always)]
    pub const fn bootkey2_11(&self) -> u32 {
        (self.bits & Bootkey211::BOOTKEY2_11_MASK) >> Bootkey211::BOOTKEY2_11_OFFSET
    }

}

impl Default for Bootkey211Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Bootkey211 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Bootkey211Val {
        Bootkey211Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Bootkey211Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Bootkey211Val) -> Bootkey211Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// BOOTKEY2_12 Register
///
/// Bits 207:192 of SHA-256 hash of boot key 2 (ECC)
///
/// Address offset: 0x0158
///
/// ## Fields
///
/// - **BOOTKEY2_12** `[15:0]` (RO)
#[repr(transparent)]
pub struct Bootkey212 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Bootkey212 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Bootkey212 {
    // ======== Field Constants ========

    /// **BOOTKEY2_12**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const BOOTKEY2_12_OFFSET: u32 = 0;
    pub const BOOTKEY2_12_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **BOOTKEY2_12** `[15:0]` (RO)
    #[inline(always)]
    pub fn bootkey2_12(&self) -> u32 {
        (self.value.read_bits() & Self::BOOTKEY2_12_MASK) >> Self::BOOTKEY2_12_OFFSET
    }

}

/// BOOTKEY2_12 value type for batch read/modify/write operations
///
/// Use `bootkey212.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootkey212.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bootkey212Val {
    bits: u32,
}

impl Bootkey212Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **BOOTKEY2_12** `[15:0]`
    #[inline(always)]
    pub const fn bootkey2_12(&self) -> u32 {
        (self.bits & Bootkey212::BOOTKEY2_12_MASK) >> Bootkey212::BOOTKEY2_12_OFFSET
    }

}

impl Default for Bootkey212Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Bootkey212 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Bootkey212Val {
        Bootkey212Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Bootkey212Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Bootkey212Val) -> Bootkey212Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// BOOTKEY2_13 Register
///
/// Bits 223:208 of SHA-256 hash of boot key 2 (ECC)
///
/// Address offset: 0x015a
///
/// ## Fields
///
/// - **BOOTKEY2_13** `[15:0]` (RO)
#[repr(transparent)]
pub struct Bootkey213 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Bootkey213 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Bootkey213 {
    // ======== Field Constants ========

    /// **BOOTKEY2_13**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const BOOTKEY2_13_OFFSET: u32 = 0;
    pub const BOOTKEY2_13_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **BOOTKEY2_13** `[15:0]` (RO)
    #[inline(always)]
    pub fn bootkey2_13(&self) -> u32 {
        (self.value.read_bits() & Self::BOOTKEY2_13_MASK) >> Self::BOOTKEY2_13_OFFSET
    }

}

/// BOOTKEY2_13 value type for batch read/modify/write operations
///
/// Use `bootkey213.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootkey213.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bootkey213Val {
    bits: u32,
}

impl Bootkey213Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **BOOTKEY2_13** `[15:0]`
    #[inline(always)]
    pub const fn bootkey2_13(&self) -> u32 {
        (self.bits & Bootkey213::BOOTKEY2_13_MASK) >> Bootkey213::BOOTKEY2_13_OFFSET
    }

}

impl Default for Bootkey213Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Bootkey213 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Bootkey213Val {
        Bootkey213Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Bootkey213Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Bootkey213Val) -> Bootkey213Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// BOOTKEY2_14 Register
///
/// Bits 239:224 of SHA-256 hash of boot key 2 (ECC)
///
/// Address offset: 0x015c
///
/// ## Fields
///
/// - **BOOTKEY2_14** `[15:0]` (RO)
#[repr(transparent)]
pub struct Bootkey214 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Bootkey214 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Bootkey214 {
    // ======== Field Constants ========

    /// **BOOTKEY2_14**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const BOOTKEY2_14_OFFSET: u32 = 0;
    pub const BOOTKEY2_14_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **BOOTKEY2_14** `[15:0]` (RO)
    #[inline(always)]
    pub fn bootkey2_14(&self) -> u32 {
        (self.value.read_bits() & Self::BOOTKEY2_14_MASK) >> Self::BOOTKEY2_14_OFFSET
    }

}

/// BOOTKEY2_14 value type for batch read/modify/write operations
///
/// Use `bootkey214.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootkey214.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bootkey214Val {
    bits: u32,
}

impl Bootkey214Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **BOOTKEY2_14** `[15:0]`
    #[inline(always)]
    pub const fn bootkey2_14(&self) -> u32 {
        (self.bits & Bootkey214::BOOTKEY2_14_MASK) >> Bootkey214::BOOTKEY2_14_OFFSET
    }

}

impl Default for Bootkey214Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Bootkey214 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Bootkey214Val {
        Bootkey214Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Bootkey214Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Bootkey214Val) -> Bootkey214Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// BOOTKEY2_15 Register
///
/// Bits 255:240 of SHA-256 hash of boot key 2 (ECC)
///
/// Address offset: 0x015e
///
/// ## Fields
///
/// - **BOOTKEY2_15** `[15:0]` (RO)
#[repr(transparent)]
pub struct Bootkey215 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Bootkey215 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Bootkey215 {
    // ======== Field Constants ========

    /// **BOOTKEY2_15**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const BOOTKEY2_15_OFFSET: u32 = 0;
    pub const BOOTKEY2_15_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **BOOTKEY2_15** `[15:0]` (RO)
    #[inline(always)]
    pub fn bootkey2_15(&self) -> u32 {
        (self.value.read_bits() & Self::BOOTKEY2_15_MASK) >> Self::BOOTKEY2_15_OFFSET
    }

}

/// BOOTKEY2_15 value type for batch read/modify/write operations
///
/// Use `bootkey215.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootkey215.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bootkey215Val {
    bits: u32,
}

impl Bootkey215Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **BOOTKEY2_15** `[15:0]`
    #[inline(always)]
    pub const fn bootkey2_15(&self) -> u32 {
        (self.bits & Bootkey215::BOOTKEY2_15_MASK) >> Bootkey215::BOOTKEY2_15_OFFSET
    }

}

impl Default for Bootkey215Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Bootkey215 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Bootkey215Val {
        Bootkey215Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Bootkey215Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Bootkey215Val) -> Bootkey215Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// BOOTKEY3_0 Register
///
/// Bits 15:0 of SHA-256 hash of boot key 3 (ECC)
///
/// Address offset: 0x0160
///
/// ## Fields
///
/// - **BOOTKEY3_0** `[15:0]` (RO)
#[repr(transparent)]
pub struct Bootkey30 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Bootkey30 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Bootkey30 {
    // ======== Field Constants ========

    /// **BOOTKEY3_0**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const BOOTKEY3_0_OFFSET: u32 = 0;
    pub const BOOTKEY3_0_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **BOOTKEY3_0** `[15:0]` (RO)
    #[inline(always)]
    pub fn bootkey3_0(&self) -> u32 {
        (self.value.read_bits() & Self::BOOTKEY3_0_MASK) >> Self::BOOTKEY3_0_OFFSET
    }

}

/// BOOTKEY3_0 value type for batch read/modify/write operations
///
/// Use `bootkey30.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootkey30.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bootkey30Val {
    bits: u32,
}

impl Bootkey30Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **BOOTKEY3_0** `[15:0]`
    #[inline(always)]
    pub const fn bootkey3_0(&self) -> u32 {
        (self.bits & Bootkey30::BOOTKEY3_0_MASK) >> Bootkey30::BOOTKEY3_0_OFFSET
    }

}

impl Default for Bootkey30Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Bootkey30 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Bootkey30Val {
        Bootkey30Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Bootkey30Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Bootkey30Val) -> Bootkey30Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// BOOTKEY3_1 Register
///
/// Bits 31:16 of SHA-256 hash of boot key 3 (ECC)
///
/// Address offset: 0x0162
///
/// ## Fields
///
/// - **BOOTKEY3_1** `[15:0]` (RO)
#[repr(transparent)]
pub struct Bootkey31 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Bootkey31 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Bootkey31 {
    // ======== Field Constants ========

    /// **BOOTKEY3_1**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const BOOTKEY3_1_OFFSET: u32 = 0;
    pub const BOOTKEY3_1_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **BOOTKEY3_1** `[15:0]` (RO)
    #[inline(always)]
    pub fn bootkey3_1(&self) -> u32 {
        (self.value.read_bits() & Self::BOOTKEY3_1_MASK) >> Self::BOOTKEY3_1_OFFSET
    }

}

/// BOOTKEY3_1 value type for batch read/modify/write operations
///
/// Use `bootkey31.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootkey31.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bootkey31Val {
    bits: u32,
}

impl Bootkey31Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **BOOTKEY3_1** `[15:0]`
    #[inline(always)]
    pub const fn bootkey3_1(&self) -> u32 {
        (self.bits & Bootkey31::BOOTKEY3_1_MASK) >> Bootkey31::BOOTKEY3_1_OFFSET
    }

}

impl Default for Bootkey31Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Bootkey31 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Bootkey31Val {
        Bootkey31Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Bootkey31Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Bootkey31Val) -> Bootkey31Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// BOOTKEY3_2 Register
///
/// Bits 47:32 of SHA-256 hash of boot key 3 (ECC)
///
/// Address offset: 0x0164
///
/// ## Fields
///
/// - **BOOTKEY3_2** `[15:0]` (RO)
#[repr(transparent)]
pub struct Bootkey32 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Bootkey32 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Bootkey32 {
    // ======== Field Constants ========

    /// **BOOTKEY3_2**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const BOOTKEY3_2_OFFSET: u32 = 0;
    pub const BOOTKEY3_2_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **BOOTKEY3_2** `[15:0]` (RO)
    #[inline(always)]
    pub fn bootkey3_2(&self) -> u32 {
        (self.value.read_bits() & Self::BOOTKEY3_2_MASK) >> Self::BOOTKEY3_2_OFFSET
    }

}

/// BOOTKEY3_2 value type for batch read/modify/write operations
///
/// Use `bootkey32.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootkey32.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bootkey32Val {
    bits: u32,
}

impl Bootkey32Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **BOOTKEY3_2** `[15:0]`
    #[inline(always)]
    pub const fn bootkey3_2(&self) -> u32 {
        (self.bits & Bootkey32::BOOTKEY3_2_MASK) >> Bootkey32::BOOTKEY3_2_OFFSET
    }

}

impl Default for Bootkey32Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Bootkey32 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Bootkey32Val {
        Bootkey32Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Bootkey32Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Bootkey32Val) -> Bootkey32Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// BOOTKEY3_3 Register
///
/// Bits 63:48 of SHA-256 hash of boot key 3 (ECC)
///
/// Address offset: 0x0166
///
/// ## Fields
///
/// - **BOOTKEY3_3** `[15:0]` (RO)
#[repr(transparent)]
pub struct Bootkey33 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Bootkey33 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Bootkey33 {
    // ======== Field Constants ========

    /// **BOOTKEY3_3**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const BOOTKEY3_3_OFFSET: u32 = 0;
    pub const BOOTKEY3_3_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **BOOTKEY3_3** `[15:0]` (RO)
    #[inline(always)]
    pub fn bootkey3_3(&self) -> u32 {
        (self.value.read_bits() & Self::BOOTKEY3_3_MASK) >> Self::BOOTKEY3_3_OFFSET
    }

}

/// BOOTKEY3_3 value type for batch read/modify/write operations
///
/// Use `bootkey33.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootkey33.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bootkey33Val {
    bits: u32,
}

impl Bootkey33Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **BOOTKEY3_3** `[15:0]`
    #[inline(always)]
    pub const fn bootkey3_3(&self) -> u32 {
        (self.bits & Bootkey33::BOOTKEY3_3_MASK) >> Bootkey33::BOOTKEY3_3_OFFSET
    }

}

impl Default for Bootkey33Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Bootkey33 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Bootkey33Val {
        Bootkey33Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Bootkey33Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Bootkey33Val) -> Bootkey33Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// BOOTKEY3_4 Register
///
/// Bits 79:64 of SHA-256 hash of boot key 3 (ECC)
///
/// Address offset: 0x0168
///
/// ## Fields
///
/// - **BOOTKEY3_4** `[15:0]` (RO)
#[repr(transparent)]
pub struct Bootkey34 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Bootkey34 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Bootkey34 {
    // ======== Field Constants ========

    /// **BOOTKEY3_4**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const BOOTKEY3_4_OFFSET: u32 = 0;
    pub const BOOTKEY3_4_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **BOOTKEY3_4** `[15:0]` (RO)
    #[inline(always)]
    pub fn bootkey3_4(&self) -> u32 {
        (self.value.read_bits() & Self::BOOTKEY3_4_MASK) >> Self::BOOTKEY3_4_OFFSET
    }

}

/// BOOTKEY3_4 value type for batch read/modify/write operations
///
/// Use `bootkey34.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootkey34.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bootkey34Val {
    bits: u32,
}

impl Bootkey34Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **BOOTKEY3_4** `[15:0]`
    #[inline(always)]
    pub const fn bootkey3_4(&self) -> u32 {
        (self.bits & Bootkey34::BOOTKEY3_4_MASK) >> Bootkey34::BOOTKEY3_4_OFFSET
    }

}

impl Default for Bootkey34Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Bootkey34 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Bootkey34Val {
        Bootkey34Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Bootkey34Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Bootkey34Val) -> Bootkey34Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// BOOTKEY3_5 Register
///
/// Bits 95:80 of SHA-256 hash of boot key 3 (ECC)
///
/// Address offset: 0x016a
///
/// ## Fields
///
/// - **BOOTKEY3_5** `[15:0]` (RO)
#[repr(transparent)]
pub struct Bootkey35 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Bootkey35 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Bootkey35 {
    // ======== Field Constants ========

    /// **BOOTKEY3_5**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const BOOTKEY3_5_OFFSET: u32 = 0;
    pub const BOOTKEY3_5_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **BOOTKEY3_5** `[15:0]` (RO)
    #[inline(always)]
    pub fn bootkey3_5(&self) -> u32 {
        (self.value.read_bits() & Self::BOOTKEY3_5_MASK) >> Self::BOOTKEY3_5_OFFSET
    }

}

/// BOOTKEY3_5 value type for batch read/modify/write operations
///
/// Use `bootkey35.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootkey35.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bootkey35Val {
    bits: u32,
}

impl Bootkey35Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **BOOTKEY3_5** `[15:0]`
    #[inline(always)]
    pub const fn bootkey3_5(&self) -> u32 {
        (self.bits & Bootkey35::BOOTKEY3_5_MASK) >> Bootkey35::BOOTKEY3_5_OFFSET
    }

}

impl Default for Bootkey35Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Bootkey35 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Bootkey35Val {
        Bootkey35Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Bootkey35Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Bootkey35Val) -> Bootkey35Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// BOOTKEY3_6 Register
///
/// Bits 111:96 of SHA-256 hash of boot key 3 (ECC)
///
/// Address offset: 0x016c
///
/// ## Fields
///
/// - **BOOTKEY3_6** `[15:0]` (RO)
#[repr(transparent)]
pub struct Bootkey36 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Bootkey36 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Bootkey36 {
    // ======== Field Constants ========

    /// **BOOTKEY3_6**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const BOOTKEY3_6_OFFSET: u32 = 0;
    pub const BOOTKEY3_6_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **BOOTKEY3_6** `[15:0]` (RO)
    #[inline(always)]
    pub fn bootkey3_6(&self) -> u32 {
        (self.value.read_bits() & Self::BOOTKEY3_6_MASK) >> Self::BOOTKEY3_6_OFFSET
    }

}

/// BOOTKEY3_6 value type for batch read/modify/write operations
///
/// Use `bootkey36.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootkey36.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bootkey36Val {
    bits: u32,
}

impl Bootkey36Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **BOOTKEY3_6** `[15:0]`
    #[inline(always)]
    pub const fn bootkey3_6(&self) -> u32 {
        (self.bits & Bootkey36::BOOTKEY3_6_MASK) >> Bootkey36::BOOTKEY3_6_OFFSET
    }

}

impl Default for Bootkey36Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Bootkey36 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Bootkey36Val {
        Bootkey36Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Bootkey36Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Bootkey36Val) -> Bootkey36Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// BOOTKEY3_7 Register
///
/// Bits 127:112 of SHA-256 hash of boot key 3 (ECC)
///
/// Address offset: 0x016e
///
/// ## Fields
///
/// - **BOOTKEY3_7** `[15:0]` (RO)
#[repr(transparent)]
pub struct Bootkey37 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Bootkey37 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Bootkey37 {
    // ======== Field Constants ========

    /// **BOOTKEY3_7**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const BOOTKEY3_7_OFFSET: u32 = 0;
    pub const BOOTKEY3_7_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **BOOTKEY3_7** `[15:0]` (RO)
    #[inline(always)]
    pub fn bootkey3_7(&self) -> u32 {
        (self.value.read_bits() & Self::BOOTKEY3_7_MASK) >> Self::BOOTKEY3_7_OFFSET
    }

}

/// BOOTKEY3_7 value type for batch read/modify/write operations
///
/// Use `bootkey37.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootkey37.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bootkey37Val {
    bits: u32,
}

impl Bootkey37Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **BOOTKEY3_7** `[15:0]`
    #[inline(always)]
    pub const fn bootkey3_7(&self) -> u32 {
        (self.bits & Bootkey37::BOOTKEY3_7_MASK) >> Bootkey37::BOOTKEY3_7_OFFSET
    }

}

impl Default for Bootkey37Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Bootkey37 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Bootkey37Val {
        Bootkey37Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Bootkey37Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Bootkey37Val) -> Bootkey37Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// BOOTKEY3_8 Register
///
/// Bits 143:128 of SHA-256 hash of boot key 3 (ECC)
///
/// Address offset: 0x0170
///
/// ## Fields
///
/// - **BOOTKEY3_8** `[15:0]` (RO)
#[repr(transparent)]
pub struct Bootkey38 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Bootkey38 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Bootkey38 {
    // ======== Field Constants ========

    /// **BOOTKEY3_8**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const BOOTKEY3_8_OFFSET: u32 = 0;
    pub const BOOTKEY3_8_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **BOOTKEY3_8** `[15:0]` (RO)
    #[inline(always)]
    pub fn bootkey3_8(&self) -> u32 {
        (self.value.read_bits() & Self::BOOTKEY3_8_MASK) >> Self::BOOTKEY3_8_OFFSET
    }

}

/// BOOTKEY3_8 value type for batch read/modify/write operations
///
/// Use `bootkey38.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootkey38.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bootkey38Val {
    bits: u32,
}

impl Bootkey38Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **BOOTKEY3_8** `[15:0]`
    #[inline(always)]
    pub const fn bootkey3_8(&self) -> u32 {
        (self.bits & Bootkey38::BOOTKEY3_8_MASK) >> Bootkey38::BOOTKEY3_8_OFFSET
    }

}

impl Default for Bootkey38Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Bootkey38 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Bootkey38Val {
        Bootkey38Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Bootkey38Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Bootkey38Val) -> Bootkey38Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// BOOTKEY3_9 Register
///
/// Bits 159:144 of SHA-256 hash of boot key 3 (ECC)
///
/// Address offset: 0x0172
///
/// ## Fields
///
/// - **BOOTKEY3_9** `[15:0]` (RO)
#[repr(transparent)]
pub struct Bootkey39 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Bootkey39 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Bootkey39 {
    // ======== Field Constants ========

    /// **BOOTKEY3_9**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const BOOTKEY3_9_OFFSET: u32 = 0;
    pub const BOOTKEY3_9_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **BOOTKEY3_9** `[15:0]` (RO)
    #[inline(always)]
    pub fn bootkey3_9(&self) -> u32 {
        (self.value.read_bits() & Self::BOOTKEY3_9_MASK) >> Self::BOOTKEY3_9_OFFSET
    }

}

/// BOOTKEY3_9 value type for batch read/modify/write operations
///
/// Use `bootkey39.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootkey39.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bootkey39Val {
    bits: u32,
}

impl Bootkey39Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **BOOTKEY3_9** `[15:0]`
    #[inline(always)]
    pub const fn bootkey3_9(&self) -> u32 {
        (self.bits & Bootkey39::BOOTKEY3_9_MASK) >> Bootkey39::BOOTKEY3_9_OFFSET
    }

}

impl Default for Bootkey39Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Bootkey39 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Bootkey39Val {
        Bootkey39Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Bootkey39Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Bootkey39Val) -> Bootkey39Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// BOOTKEY3_10 Register
///
/// Bits 175:160 of SHA-256 hash of boot key 3 (ECC)
///
/// Address offset: 0x0174
///
/// ## Fields
///
/// - **BOOTKEY3_10** `[15:0]` (RO)
#[repr(transparent)]
pub struct Bootkey310 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Bootkey310 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Bootkey310 {
    // ======== Field Constants ========

    /// **BOOTKEY3_10**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const BOOTKEY3_10_OFFSET: u32 = 0;
    pub const BOOTKEY3_10_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **BOOTKEY3_10** `[15:0]` (RO)
    #[inline(always)]
    pub fn bootkey3_10(&self) -> u32 {
        (self.value.read_bits() & Self::BOOTKEY3_10_MASK) >> Self::BOOTKEY3_10_OFFSET
    }

}

/// BOOTKEY3_10 value type for batch read/modify/write operations
///
/// Use `bootkey310.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootkey310.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bootkey310Val {
    bits: u32,
}

impl Bootkey310Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **BOOTKEY3_10** `[15:0]`
    #[inline(always)]
    pub const fn bootkey3_10(&self) -> u32 {
        (self.bits & Bootkey310::BOOTKEY3_10_MASK) >> Bootkey310::BOOTKEY3_10_OFFSET
    }

}

impl Default for Bootkey310Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Bootkey310 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Bootkey310Val {
        Bootkey310Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Bootkey310Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Bootkey310Val) -> Bootkey310Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// BOOTKEY3_11 Register
///
/// Bits 191:176 of SHA-256 hash of boot key 3 (ECC)
///
/// Address offset: 0x0176
///
/// ## Fields
///
/// - **BOOTKEY3_11** `[15:0]` (RO)
#[repr(transparent)]
pub struct Bootkey311 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Bootkey311 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Bootkey311 {
    // ======== Field Constants ========

    /// **BOOTKEY3_11**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const BOOTKEY3_11_OFFSET: u32 = 0;
    pub const BOOTKEY3_11_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **BOOTKEY3_11** `[15:0]` (RO)
    #[inline(always)]
    pub fn bootkey3_11(&self) -> u32 {
        (self.value.read_bits() & Self::BOOTKEY3_11_MASK) >> Self::BOOTKEY3_11_OFFSET
    }

}

/// BOOTKEY3_11 value type for batch read/modify/write operations
///
/// Use `bootkey311.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootkey311.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bootkey311Val {
    bits: u32,
}

impl Bootkey311Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **BOOTKEY3_11** `[15:0]`
    #[inline(always)]
    pub const fn bootkey3_11(&self) -> u32 {
        (self.bits & Bootkey311::BOOTKEY3_11_MASK) >> Bootkey311::BOOTKEY3_11_OFFSET
    }

}

impl Default for Bootkey311Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Bootkey311 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Bootkey311Val {
        Bootkey311Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Bootkey311Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Bootkey311Val) -> Bootkey311Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// BOOTKEY3_12 Register
///
/// Bits 207:192 of SHA-256 hash of boot key 3 (ECC)
///
/// Address offset: 0x0178
///
/// ## Fields
///
/// - **BOOTKEY3_12** `[15:0]` (RO)
#[repr(transparent)]
pub struct Bootkey312 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Bootkey312 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Bootkey312 {
    // ======== Field Constants ========

    /// **BOOTKEY3_12**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const BOOTKEY3_12_OFFSET: u32 = 0;
    pub const BOOTKEY3_12_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **BOOTKEY3_12** `[15:0]` (RO)
    #[inline(always)]
    pub fn bootkey3_12(&self) -> u32 {
        (self.value.read_bits() & Self::BOOTKEY3_12_MASK) >> Self::BOOTKEY3_12_OFFSET
    }

}

/// BOOTKEY3_12 value type for batch read/modify/write operations
///
/// Use `bootkey312.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootkey312.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bootkey312Val {
    bits: u32,
}

impl Bootkey312Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **BOOTKEY3_12** `[15:0]`
    #[inline(always)]
    pub const fn bootkey3_12(&self) -> u32 {
        (self.bits & Bootkey312::BOOTKEY3_12_MASK) >> Bootkey312::BOOTKEY3_12_OFFSET
    }

}

impl Default for Bootkey312Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Bootkey312 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Bootkey312Val {
        Bootkey312Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Bootkey312Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Bootkey312Val) -> Bootkey312Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// BOOTKEY3_13 Register
///
/// Bits 223:208 of SHA-256 hash of boot key 3 (ECC)
///
/// Address offset: 0x017a
///
/// ## Fields
///
/// - **BOOTKEY3_13** `[15:0]` (RO)
#[repr(transparent)]
pub struct Bootkey313 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Bootkey313 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Bootkey313 {
    // ======== Field Constants ========

    /// **BOOTKEY3_13**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const BOOTKEY3_13_OFFSET: u32 = 0;
    pub const BOOTKEY3_13_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **BOOTKEY3_13** `[15:0]` (RO)
    #[inline(always)]
    pub fn bootkey3_13(&self) -> u32 {
        (self.value.read_bits() & Self::BOOTKEY3_13_MASK) >> Self::BOOTKEY3_13_OFFSET
    }

}

/// BOOTKEY3_13 value type for batch read/modify/write operations
///
/// Use `bootkey313.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootkey313.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bootkey313Val {
    bits: u32,
}

impl Bootkey313Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **BOOTKEY3_13** `[15:0]`
    #[inline(always)]
    pub const fn bootkey3_13(&self) -> u32 {
        (self.bits & Bootkey313::BOOTKEY3_13_MASK) >> Bootkey313::BOOTKEY3_13_OFFSET
    }

}

impl Default for Bootkey313Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Bootkey313 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Bootkey313Val {
        Bootkey313Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Bootkey313Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Bootkey313Val) -> Bootkey313Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// BOOTKEY3_14 Register
///
/// Bits 239:224 of SHA-256 hash of boot key 3 (ECC)
///
/// Address offset: 0x017c
///
/// ## Fields
///
/// - **BOOTKEY3_14** `[15:0]` (RO)
#[repr(transparent)]
pub struct Bootkey314 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Bootkey314 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Bootkey314 {
    // ======== Field Constants ========

    /// **BOOTKEY3_14**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const BOOTKEY3_14_OFFSET: u32 = 0;
    pub const BOOTKEY3_14_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **BOOTKEY3_14** `[15:0]` (RO)
    #[inline(always)]
    pub fn bootkey3_14(&self) -> u32 {
        (self.value.read_bits() & Self::BOOTKEY3_14_MASK) >> Self::BOOTKEY3_14_OFFSET
    }

}

/// BOOTKEY3_14 value type for batch read/modify/write operations
///
/// Use `bootkey314.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootkey314.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bootkey314Val {
    bits: u32,
}

impl Bootkey314Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **BOOTKEY3_14** `[15:0]`
    #[inline(always)]
    pub const fn bootkey3_14(&self) -> u32 {
        (self.bits & Bootkey314::BOOTKEY3_14_MASK) >> Bootkey314::BOOTKEY3_14_OFFSET
    }

}

impl Default for Bootkey314Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Bootkey314 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Bootkey314Val {
        Bootkey314Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Bootkey314Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Bootkey314Val) -> Bootkey314Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// BOOTKEY3_15 Register
///
/// Bits 255:240 of SHA-256 hash of boot key 3 (ECC)
///
/// Address offset: 0x017e
///
/// ## Fields
///
/// - **BOOTKEY3_15** `[15:0]` (RO)
#[repr(transparent)]
pub struct Bootkey315 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Bootkey315 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Bootkey315 {
    // ======== Field Constants ========

    /// **BOOTKEY3_15**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const BOOTKEY3_15_OFFSET: u32 = 0;
    pub const BOOTKEY3_15_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **BOOTKEY3_15** `[15:0]` (RO)
    #[inline(always)]
    pub fn bootkey3_15(&self) -> u32 {
        (self.value.read_bits() & Self::BOOTKEY3_15_MASK) >> Self::BOOTKEY3_15_OFFSET
    }

}

/// BOOTKEY3_15 value type for batch read/modify/write operations
///
/// Use `bootkey315.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootkey315.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Bootkey315Val {
    bits: u32,
}

impl Bootkey315Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **BOOTKEY3_15** `[15:0]`
    #[inline(always)]
    pub const fn bootkey3_15(&self) -> u32 {
        (self.bits & Bootkey315::BOOTKEY3_15_MASK) >> Bootkey315::BOOTKEY3_15_OFFSET
    }

}

impl Default for Bootkey315Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Bootkey315 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Bootkey315Val {
        Bootkey315Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Bootkey315Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Bootkey315Val) -> Bootkey315Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// KEY1_0 Register
///
/// Bits 15:0 of OTP access key 1 (ECC)
///
/// Address offset: 0x1e90
///
/// ## Fields
///
/// - **KEY1_0** `[15:0]` (RO)
#[repr(transparent)]
pub struct Key10 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Key10 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Key10 {
    // ======== Field Constants ========

    /// **KEY1_0**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const KEY1_0_OFFSET: u32 = 0;
    pub const KEY1_0_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **KEY1_0** `[15:0]` (RO)
    #[inline(always)]
    pub fn key1_0(&self) -> u32 {
        (self.value.read_bits() & Self::KEY1_0_MASK) >> Self::KEY1_0_OFFSET
    }

}

/// KEY1_0 value type for batch read/modify/write operations
///
/// Use `key10.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `key10.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Key10Val {
    bits: u32,
}

impl Key10Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY1_0** `[15:0]`
    #[inline(always)]
    pub const fn key1_0(&self) -> u32 {
        (self.bits & Key10::KEY1_0_MASK) >> Key10::KEY1_0_OFFSET
    }

}

impl Default for Key10Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Key10 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Key10Val {
        Key10Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Key10Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Key10Val) -> Key10Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// KEY1_1 Register
///
/// Bits 31:16 of OTP access key 1 (ECC)
///
/// Address offset: 0x1e92
///
/// ## Fields
///
/// - **KEY1_1** `[15:0]` (RO)
#[repr(transparent)]
pub struct Key11 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Key11 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Key11 {
    // ======== Field Constants ========

    /// **KEY1_1**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const KEY1_1_OFFSET: u32 = 0;
    pub const KEY1_1_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **KEY1_1** `[15:0]` (RO)
    #[inline(always)]
    pub fn key1_1(&self) -> u32 {
        (self.value.read_bits() & Self::KEY1_1_MASK) >> Self::KEY1_1_OFFSET
    }

}

/// KEY1_1 value type for batch read/modify/write operations
///
/// Use `key11.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `key11.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Key11Val {
    bits: u32,
}

impl Key11Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY1_1** `[15:0]`
    #[inline(always)]
    pub const fn key1_1(&self) -> u32 {
        (self.bits & Key11::KEY1_1_MASK) >> Key11::KEY1_1_OFFSET
    }

}

impl Default for Key11Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Key11 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Key11Val {
        Key11Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Key11Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Key11Val) -> Key11Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// KEY1_2 Register
///
/// Bits 47:32 of OTP access key 1 (ECC)
///
/// Address offset: 0x1e94
///
/// ## Fields
///
/// - **KEY1_2** `[15:0]` (RO)
#[repr(transparent)]
pub struct Key12 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Key12 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Key12 {
    // ======== Field Constants ========

    /// **KEY1_2**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const KEY1_2_OFFSET: u32 = 0;
    pub const KEY1_2_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **KEY1_2** `[15:0]` (RO)
    #[inline(always)]
    pub fn key1_2(&self) -> u32 {
        (self.value.read_bits() & Self::KEY1_2_MASK) >> Self::KEY1_2_OFFSET
    }

}

/// KEY1_2 value type for batch read/modify/write operations
///
/// Use `key12.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `key12.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Key12Val {
    bits: u32,
}

impl Key12Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY1_2** `[15:0]`
    #[inline(always)]
    pub const fn key1_2(&self) -> u32 {
        (self.bits & Key12::KEY1_2_MASK) >> Key12::KEY1_2_OFFSET
    }

}

impl Default for Key12Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Key12 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Key12Val {
        Key12Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Key12Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Key12Val) -> Key12Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// KEY1_3 Register
///
/// Bits 63:48 of OTP access key 1 (ECC)
///
/// Address offset: 0x1e96
///
/// ## Fields
///
/// - **KEY1_3** `[15:0]` (RO)
#[repr(transparent)]
pub struct Key13 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Key13 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Key13 {
    // ======== Field Constants ========

    /// **KEY1_3**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const KEY1_3_OFFSET: u32 = 0;
    pub const KEY1_3_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **KEY1_3** `[15:0]` (RO)
    #[inline(always)]
    pub fn key1_3(&self) -> u32 {
        (self.value.read_bits() & Self::KEY1_3_MASK) >> Self::KEY1_3_OFFSET
    }

}

/// KEY1_3 value type for batch read/modify/write operations
///
/// Use `key13.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `key13.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Key13Val {
    bits: u32,
}

impl Key13Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY1_3** `[15:0]`
    #[inline(always)]
    pub const fn key1_3(&self) -> u32 {
        (self.bits & Key13::KEY1_3_MASK) >> Key13::KEY1_3_OFFSET
    }

}

impl Default for Key13Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Key13 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Key13Val {
        Key13Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Key13Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Key13Val) -> Key13Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// KEY1_4 Register
///
/// Bits 79:64 of OTP access key 1 (ECC)
///
/// Address offset: 0x1e98
///
/// ## Fields
///
/// - **KEY1_4** `[15:0]` (RO)
#[repr(transparent)]
pub struct Key14 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Key14 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Key14 {
    // ======== Field Constants ========

    /// **KEY1_4**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const KEY1_4_OFFSET: u32 = 0;
    pub const KEY1_4_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **KEY1_4** `[15:0]` (RO)
    #[inline(always)]
    pub fn key1_4(&self) -> u32 {
        (self.value.read_bits() & Self::KEY1_4_MASK) >> Self::KEY1_4_OFFSET
    }

}

/// KEY1_4 value type for batch read/modify/write operations
///
/// Use `key14.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `key14.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Key14Val {
    bits: u32,
}

impl Key14Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY1_4** `[15:0]`
    #[inline(always)]
    pub const fn key1_4(&self) -> u32 {
        (self.bits & Key14::KEY1_4_MASK) >> Key14::KEY1_4_OFFSET
    }

}

impl Default for Key14Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Key14 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Key14Val {
        Key14Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Key14Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Key14Val) -> Key14Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// KEY1_5 Register
///
/// Bits 95:80 of OTP access key 1 (ECC)
///
/// Address offset: 0x1e9a
///
/// ## Fields
///
/// - **KEY1_5** `[15:0]` (RO)
#[repr(transparent)]
pub struct Key15 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Key15 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Key15 {
    // ======== Field Constants ========

    /// **KEY1_5**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const KEY1_5_OFFSET: u32 = 0;
    pub const KEY1_5_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **KEY1_5** `[15:0]` (RO)
    #[inline(always)]
    pub fn key1_5(&self) -> u32 {
        (self.value.read_bits() & Self::KEY1_5_MASK) >> Self::KEY1_5_OFFSET
    }

}

/// KEY1_5 value type for batch read/modify/write operations
///
/// Use `key15.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `key15.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Key15Val {
    bits: u32,
}

impl Key15Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY1_5** `[15:0]`
    #[inline(always)]
    pub const fn key1_5(&self) -> u32 {
        (self.bits & Key15::KEY1_5_MASK) >> Key15::KEY1_5_OFFSET
    }

}

impl Default for Key15Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Key15 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Key15Val {
        Key15Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Key15Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Key15Val) -> Key15Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// KEY1_6 Register
///
/// Bits 111:96 of OTP access key 1 (ECC)
///
/// Address offset: 0x1e9c
///
/// ## Fields
///
/// - **KEY1_6** `[15:0]` (RO)
#[repr(transparent)]
pub struct Key16 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Key16 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Key16 {
    // ======== Field Constants ========

    /// **KEY1_6**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const KEY1_6_OFFSET: u32 = 0;
    pub const KEY1_6_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **KEY1_6** `[15:0]` (RO)
    #[inline(always)]
    pub fn key1_6(&self) -> u32 {
        (self.value.read_bits() & Self::KEY1_6_MASK) >> Self::KEY1_6_OFFSET
    }

}

/// KEY1_6 value type for batch read/modify/write operations
///
/// Use `key16.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `key16.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Key16Val {
    bits: u32,
}

impl Key16Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY1_6** `[15:0]`
    #[inline(always)]
    pub const fn key1_6(&self) -> u32 {
        (self.bits & Key16::KEY1_6_MASK) >> Key16::KEY1_6_OFFSET
    }

}

impl Default for Key16Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Key16 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Key16Val {
        Key16Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Key16Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Key16Val) -> Key16Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// KEY1_7 Register
///
/// Bits 127:112 of OTP access key 1 (ECC)
///
/// Address offset: 0x1e9e
///
/// ## Fields
///
/// - **KEY1_7** `[15:0]` (RO)
#[repr(transparent)]
pub struct Key17 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Key17 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Key17 {
    // ======== Field Constants ========

    /// **KEY1_7**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const KEY1_7_OFFSET: u32 = 0;
    pub const KEY1_7_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **KEY1_7** `[15:0]` (RO)
    #[inline(always)]
    pub fn key1_7(&self) -> u32 {
        (self.value.read_bits() & Self::KEY1_7_MASK) >> Self::KEY1_7_OFFSET
    }

}

/// KEY1_7 value type for batch read/modify/write operations
///
/// Use `key17.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `key17.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Key17Val {
    bits: u32,
}

impl Key17Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY1_7** `[15:0]`
    #[inline(always)]
    pub const fn key1_7(&self) -> u32 {
        (self.bits & Key17::KEY1_7_MASK) >> Key17::KEY1_7_OFFSET
    }

}

impl Default for Key17Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Key17 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Key17Val {
        Key17Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Key17Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Key17Val) -> Key17Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// KEY2_0 Register
///
/// Bits 15:0 of OTP access key 2 (ECC)
///
/// Address offset: 0x1ea0
///
/// ## Fields
///
/// - **KEY2_0** `[15:0]` (RO)
#[repr(transparent)]
pub struct Key20 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Key20 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Key20 {
    // ======== Field Constants ========

    /// **KEY2_0**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const KEY2_0_OFFSET: u32 = 0;
    pub const KEY2_0_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **KEY2_0** `[15:0]` (RO)
    #[inline(always)]
    pub fn key2_0(&self) -> u32 {
        (self.value.read_bits() & Self::KEY2_0_MASK) >> Self::KEY2_0_OFFSET
    }

}

/// KEY2_0 value type for batch read/modify/write operations
///
/// Use `key20.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `key20.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Key20Val {
    bits: u32,
}

impl Key20Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY2_0** `[15:0]`
    #[inline(always)]
    pub const fn key2_0(&self) -> u32 {
        (self.bits & Key20::KEY2_0_MASK) >> Key20::KEY2_0_OFFSET
    }

}

impl Default for Key20Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Key20 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Key20Val {
        Key20Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Key20Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Key20Val) -> Key20Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// KEY2_1 Register
///
/// Bits 31:16 of OTP access key 2 (ECC)
///
/// Address offset: 0x1ea2
///
/// ## Fields
///
/// - **KEY2_1** `[15:0]` (RO)
#[repr(transparent)]
pub struct Key21 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Key21 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Key21 {
    // ======== Field Constants ========

    /// **KEY2_1**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const KEY2_1_OFFSET: u32 = 0;
    pub const KEY2_1_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **KEY2_1** `[15:0]` (RO)
    #[inline(always)]
    pub fn key2_1(&self) -> u32 {
        (self.value.read_bits() & Self::KEY2_1_MASK) >> Self::KEY2_1_OFFSET
    }

}

/// KEY2_1 value type for batch read/modify/write operations
///
/// Use `key21.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `key21.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Key21Val {
    bits: u32,
}

impl Key21Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY2_1** `[15:0]`
    #[inline(always)]
    pub const fn key2_1(&self) -> u32 {
        (self.bits & Key21::KEY2_1_MASK) >> Key21::KEY2_1_OFFSET
    }

}

impl Default for Key21Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Key21 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Key21Val {
        Key21Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Key21Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Key21Val) -> Key21Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// KEY2_2 Register
///
/// Bits 47:32 of OTP access key 2 (ECC)
///
/// Address offset: 0x1ea4
///
/// ## Fields
///
/// - **KEY2_2** `[15:0]` (RO)
#[repr(transparent)]
pub struct Key22 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Key22 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Key22 {
    // ======== Field Constants ========

    /// **KEY2_2**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const KEY2_2_OFFSET: u32 = 0;
    pub const KEY2_2_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **KEY2_2** `[15:0]` (RO)
    #[inline(always)]
    pub fn key2_2(&self) -> u32 {
        (self.value.read_bits() & Self::KEY2_2_MASK) >> Self::KEY2_2_OFFSET
    }

}

/// KEY2_2 value type for batch read/modify/write operations
///
/// Use `key22.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `key22.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Key22Val {
    bits: u32,
}

impl Key22Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY2_2** `[15:0]`
    #[inline(always)]
    pub const fn key2_2(&self) -> u32 {
        (self.bits & Key22::KEY2_2_MASK) >> Key22::KEY2_2_OFFSET
    }

}

impl Default for Key22Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Key22 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Key22Val {
        Key22Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Key22Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Key22Val) -> Key22Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// KEY2_3 Register
///
/// Bits 63:48 of OTP access key 2 (ECC)
///
/// Address offset: 0x1ea6
///
/// ## Fields
///
/// - **KEY2_3** `[15:0]` (RO)
#[repr(transparent)]
pub struct Key23 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Key23 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Key23 {
    // ======== Field Constants ========

    /// **KEY2_3**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const KEY2_3_OFFSET: u32 = 0;
    pub const KEY2_3_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **KEY2_3** `[15:0]` (RO)
    #[inline(always)]
    pub fn key2_3(&self) -> u32 {
        (self.value.read_bits() & Self::KEY2_3_MASK) >> Self::KEY2_3_OFFSET
    }

}

/// KEY2_3 value type for batch read/modify/write operations
///
/// Use `key23.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `key23.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Key23Val {
    bits: u32,
}

impl Key23Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY2_3** `[15:0]`
    #[inline(always)]
    pub const fn key2_3(&self) -> u32 {
        (self.bits & Key23::KEY2_3_MASK) >> Key23::KEY2_3_OFFSET
    }

}

impl Default for Key23Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Key23 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Key23Val {
        Key23Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Key23Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Key23Val) -> Key23Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// KEY2_4 Register
///
/// Bits 79:64 of OTP access key 2 (ECC)
///
/// Address offset: 0x1ea8
///
/// ## Fields
///
/// - **KEY2_4** `[15:0]` (RO)
#[repr(transparent)]
pub struct Key24 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Key24 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Key24 {
    // ======== Field Constants ========

    /// **KEY2_4**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const KEY2_4_OFFSET: u32 = 0;
    pub const KEY2_4_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **KEY2_4** `[15:0]` (RO)
    #[inline(always)]
    pub fn key2_4(&self) -> u32 {
        (self.value.read_bits() & Self::KEY2_4_MASK) >> Self::KEY2_4_OFFSET
    }

}

/// KEY2_4 value type for batch read/modify/write operations
///
/// Use `key24.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `key24.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Key24Val {
    bits: u32,
}

impl Key24Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY2_4** `[15:0]`
    #[inline(always)]
    pub const fn key2_4(&self) -> u32 {
        (self.bits & Key24::KEY2_4_MASK) >> Key24::KEY2_4_OFFSET
    }

}

impl Default for Key24Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Key24 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Key24Val {
        Key24Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Key24Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Key24Val) -> Key24Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// KEY2_5 Register
///
/// Bits 95:80 of OTP access key 2 (ECC)
///
/// Address offset: 0x1eaa
///
/// ## Fields
///
/// - **KEY2_5** `[15:0]` (RO)
#[repr(transparent)]
pub struct Key25 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Key25 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Key25 {
    // ======== Field Constants ========

    /// **KEY2_5**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const KEY2_5_OFFSET: u32 = 0;
    pub const KEY2_5_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **KEY2_5** `[15:0]` (RO)
    #[inline(always)]
    pub fn key2_5(&self) -> u32 {
        (self.value.read_bits() & Self::KEY2_5_MASK) >> Self::KEY2_5_OFFSET
    }

}

/// KEY2_5 value type for batch read/modify/write operations
///
/// Use `key25.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `key25.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Key25Val {
    bits: u32,
}

impl Key25Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY2_5** `[15:0]`
    #[inline(always)]
    pub const fn key2_5(&self) -> u32 {
        (self.bits & Key25::KEY2_5_MASK) >> Key25::KEY2_5_OFFSET
    }

}

impl Default for Key25Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Key25 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Key25Val {
        Key25Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Key25Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Key25Val) -> Key25Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// KEY2_6 Register
///
/// Bits 111:96 of OTP access key 2 (ECC)
///
/// Address offset: 0x1eac
///
/// ## Fields
///
/// - **KEY2_6** `[15:0]` (RO)
#[repr(transparent)]
pub struct Key26 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Key26 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Key26 {
    // ======== Field Constants ========

    /// **KEY2_6**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const KEY2_6_OFFSET: u32 = 0;
    pub const KEY2_6_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **KEY2_6** `[15:0]` (RO)
    #[inline(always)]
    pub fn key2_6(&self) -> u32 {
        (self.value.read_bits() & Self::KEY2_6_MASK) >> Self::KEY2_6_OFFSET
    }

}

/// KEY2_6 value type for batch read/modify/write operations
///
/// Use `key26.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `key26.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Key26Val {
    bits: u32,
}

impl Key26Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY2_6** `[15:0]`
    #[inline(always)]
    pub const fn key2_6(&self) -> u32 {
        (self.bits & Key26::KEY2_6_MASK) >> Key26::KEY2_6_OFFSET
    }

}

impl Default for Key26Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Key26 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Key26Val {
        Key26Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Key26Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Key26Val) -> Key26Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// KEY2_7 Register
///
/// Bits 127:112 of OTP access key 2 (ECC)
///
/// Address offset: 0x1eae
///
/// ## Fields
///
/// - **KEY2_7** `[15:0]` (RO)
#[repr(transparent)]
pub struct Key27 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Key27 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Key27 {
    // ======== Field Constants ========

    /// **KEY2_7**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const KEY2_7_OFFSET: u32 = 0;
    pub const KEY2_7_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **KEY2_7** `[15:0]` (RO)
    #[inline(always)]
    pub fn key2_7(&self) -> u32 {
        (self.value.read_bits() & Self::KEY2_7_MASK) >> Self::KEY2_7_OFFSET
    }

}

/// KEY2_7 value type for batch read/modify/write operations
///
/// Use `key27.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `key27.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Key27Val {
    bits: u32,
}

impl Key27Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY2_7** `[15:0]`
    #[inline(always)]
    pub const fn key2_7(&self) -> u32 {
        (self.bits & Key27::KEY2_7_MASK) >> Key27::KEY2_7_OFFSET
    }

}

impl Default for Key27Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Key27 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Key27Val {
        Key27Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Key27Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Key27Val) -> Key27Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// KEY3_0 Register
///
/// Bits 15:0 of OTP access key 3 (ECC)
///
/// Address offset: 0x1eb0
///
/// ## Fields
///
/// - **KEY3_0** `[15:0]` (RO)
#[repr(transparent)]
pub struct Key30 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Key30 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Key30 {
    // ======== Field Constants ========

    /// **KEY3_0**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const KEY3_0_OFFSET: u32 = 0;
    pub const KEY3_0_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **KEY3_0** `[15:0]` (RO)
    #[inline(always)]
    pub fn key3_0(&self) -> u32 {
        (self.value.read_bits() & Self::KEY3_0_MASK) >> Self::KEY3_0_OFFSET
    }

}

/// KEY3_0 value type for batch read/modify/write operations
///
/// Use `key30.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `key30.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Key30Val {
    bits: u32,
}

impl Key30Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY3_0** `[15:0]`
    #[inline(always)]
    pub const fn key3_0(&self) -> u32 {
        (self.bits & Key30::KEY3_0_MASK) >> Key30::KEY3_0_OFFSET
    }

}

impl Default for Key30Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Key30 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Key30Val {
        Key30Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Key30Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Key30Val) -> Key30Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// KEY3_1 Register
///
/// Bits 31:16 of OTP access key 3 (ECC)
///
/// Address offset: 0x1eb2
///
/// ## Fields
///
/// - **KEY3_1** `[15:0]` (RO)
#[repr(transparent)]
pub struct Key31 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Key31 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Key31 {
    // ======== Field Constants ========

    /// **KEY3_1**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const KEY3_1_OFFSET: u32 = 0;
    pub const KEY3_1_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **KEY3_1** `[15:0]` (RO)
    #[inline(always)]
    pub fn key3_1(&self) -> u32 {
        (self.value.read_bits() & Self::KEY3_1_MASK) >> Self::KEY3_1_OFFSET
    }

}

/// KEY3_1 value type for batch read/modify/write operations
///
/// Use `key31.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `key31.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Key31Val {
    bits: u32,
}

impl Key31Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY3_1** `[15:0]`
    #[inline(always)]
    pub const fn key3_1(&self) -> u32 {
        (self.bits & Key31::KEY3_1_MASK) >> Key31::KEY3_1_OFFSET
    }

}

impl Default for Key31Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Key31 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Key31Val {
        Key31Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Key31Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Key31Val) -> Key31Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// KEY3_2 Register
///
/// Bits 47:32 of OTP access key 3 (ECC)
///
/// Address offset: 0x1eb4
///
/// ## Fields
///
/// - **KEY3_2** `[15:0]` (RO)
#[repr(transparent)]
pub struct Key32 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Key32 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Key32 {
    // ======== Field Constants ========

    /// **KEY3_2**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const KEY3_2_OFFSET: u32 = 0;
    pub const KEY3_2_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **KEY3_2** `[15:0]` (RO)
    #[inline(always)]
    pub fn key3_2(&self) -> u32 {
        (self.value.read_bits() & Self::KEY3_2_MASK) >> Self::KEY3_2_OFFSET
    }

}

/// KEY3_2 value type for batch read/modify/write operations
///
/// Use `key32.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `key32.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Key32Val {
    bits: u32,
}

impl Key32Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY3_2** `[15:0]`
    #[inline(always)]
    pub const fn key3_2(&self) -> u32 {
        (self.bits & Key32::KEY3_2_MASK) >> Key32::KEY3_2_OFFSET
    }

}

impl Default for Key32Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Key32 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Key32Val {
        Key32Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Key32Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Key32Val) -> Key32Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// KEY3_3 Register
///
/// Bits 63:48 of OTP access key 3 (ECC)
///
/// Address offset: 0x1eb6
///
/// ## Fields
///
/// - **KEY3_3** `[15:0]` (RO)
#[repr(transparent)]
pub struct Key33 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Key33 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Key33 {
    // ======== Field Constants ========

    /// **KEY3_3**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const KEY3_3_OFFSET: u32 = 0;
    pub const KEY3_3_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **KEY3_3** `[15:0]` (RO)
    #[inline(always)]
    pub fn key3_3(&self) -> u32 {
        (self.value.read_bits() & Self::KEY3_3_MASK) >> Self::KEY3_3_OFFSET
    }

}

/// KEY3_3 value type for batch read/modify/write operations
///
/// Use `key33.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `key33.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Key33Val {
    bits: u32,
}

impl Key33Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY3_3** `[15:0]`
    #[inline(always)]
    pub const fn key3_3(&self) -> u32 {
        (self.bits & Key33::KEY3_3_MASK) >> Key33::KEY3_3_OFFSET
    }

}

impl Default for Key33Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Key33 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Key33Val {
        Key33Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Key33Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Key33Val) -> Key33Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// KEY3_4 Register
///
/// Bits 79:64 of OTP access key 3 (ECC)
///
/// Address offset: 0x1eb8
///
/// ## Fields
///
/// - **KEY3_4** `[15:0]` (RO)
#[repr(transparent)]
pub struct Key34 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Key34 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Key34 {
    // ======== Field Constants ========

    /// **KEY3_4**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const KEY3_4_OFFSET: u32 = 0;
    pub const KEY3_4_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **KEY3_4** `[15:0]` (RO)
    #[inline(always)]
    pub fn key3_4(&self) -> u32 {
        (self.value.read_bits() & Self::KEY3_4_MASK) >> Self::KEY3_4_OFFSET
    }

}

/// KEY3_4 value type for batch read/modify/write operations
///
/// Use `key34.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `key34.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Key34Val {
    bits: u32,
}

impl Key34Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY3_4** `[15:0]`
    #[inline(always)]
    pub const fn key3_4(&self) -> u32 {
        (self.bits & Key34::KEY3_4_MASK) >> Key34::KEY3_4_OFFSET
    }

}

impl Default for Key34Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Key34 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Key34Val {
        Key34Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Key34Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Key34Val) -> Key34Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// KEY3_5 Register
///
/// Bits 95:80 of OTP access key 3 (ECC)
///
/// Address offset: 0x1eba
///
/// ## Fields
///
/// - **KEY3_5** `[15:0]` (RO)
#[repr(transparent)]
pub struct Key35 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Key35 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Key35 {
    // ======== Field Constants ========

    /// **KEY3_5**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const KEY3_5_OFFSET: u32 = 0;
    pub const KEY3_5_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **KEY3_5** `[15:0]` (RO)
    #[inline(always)]
    pub fn key3_5(&self) -> u32 {
        (self.value.read_bits() & Self::KEY3_5_MASK) >> Self::KEY3_5_OFFSET
    }

}

/// KEY3_5 value type for batch read/modify/write operations
///
/// Use `key35.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `key35.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Key35Val {
    bits: u32,
}

impl Key35Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY3_5** `[15:0]`
    #[inline(always)]
    pub const fn key3_5(&self) -> u32 {
        (self.bits & Key35::KEY3_5_MASK) >> Key35::KEY3_5_OFFSET
    }

}

impl Default for Key35Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Key35 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Key35Val {
        Key35Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Key35Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Key35Val) -> Key35Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// KEY3_6 Register
///
/// Bits 111:96 of OTP access key 3 (ECC)
///
/// Address offset: 0x1ebc
///
/// ## Fields
///
/// - **KEY3_6** `[15:0]` (RO)
#[repr(transparent)]
pub struct Key36 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Key36 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Key36 {
    // ======== Field Constants ========

    /// **KEY3_6**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const KEY3_6_OFFSET: u32 = 0;
    pub const KEY3_6_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **KEY3_6** `[15:0]` (RO)
    #[inline(always)]
    pub fn key3_6(&self) -> u32 {
        (self.value.read_bits() & Self::KEY3_6_MASK) >> Self::KEY3_6_OFFSET
    }

}

/// KEY3_6 value type for batch read/modify/write operations
///
/// Use `key36.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `key36.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Key36Val {
    bits: u32,
}

impl Key36Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY3_6** `[15:0]`
    #[inline(always)]
    pub const fn key3_6(&self) -> u32 {
        (self.bits & Key36::KEY3_6_MASK) >> Key36::KEY3_6_OFFSET
    }

}

impl Default for Key36Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Key36 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Key36Val {
        Key36Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Key36Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Key36Val) -> Key36Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// KEY3_7 Register
///
/// Bits 127:112 of OTP access key 3 (ECC)
///
/// Address offset: 0x1ebe
///
/// ## Fields
///
/// - **KEY3_7** `[15:0]` (RO)
#[repr(transparent)]
pub struct Key37 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Key37 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Key37 {
    // ======== Field Constants ========

    /// **KEY3_7**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const KEY3_7_OFFSET: u32 = 0;
    pub const KEY3_7_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **KEY3_7** `[15:0]` (RO)
    #[inline(always)]
    pub fn key3_7(&self) -> u32 {
        (self.value.read_bits() & Self::KEY3_7_MASK) >> Self::KEY3_7_OFFSET
    }

}

/// KEY3_7 value type for batch read/modify/write operations
///
/// Use `key37.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `key37.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Key37Val {
    bits: u32,
}

impl Key37Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY3_7** `[15:0]`
    #[inline(always)]
    pub const fn key3_7(&self) -> u32 {
        (self.bits & Key37::KEY3_7_MASK) >> Key37::KEY3_7_OFFSET
    }

}

impl Default for Key37Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Key37 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Key37Val {
        Key37Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Key37Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Key37Val) -> Key37Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// KEY4_0 Register
///
/// Bits 15:0 of OTP access key 4 (ECC)
///
/// Address offset: 0x1ec0
///
/// ## Fields
///
/// - **KEY4_0** `[15:0]` (RO)
#[repr(transparent)]
pub struct Key40 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Key40 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Key40 {
    // ======== Field Constants ========

    /// **KEY4_0**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const KEY4_0_OFFSET: u32 = 0;
    pub const KEY4_0_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **KEY4_0** `[15:0]` (RO)
    #[inline(always)]
    pub fn key4_0(&self) -> u32 {
        (self.value.read_bits() & Self::KEY4_0_MASK) >> Self::KEY4_0_OFFSET
    }

}

/// KEY4_0 value type for batch read/modify/write operations
///
/// Use `key40.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `key40.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Key40Val {
    bits: u32,
}

impl Key40Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY4_0** `[15:0]`
    #[inline(always)]
    pub const fn key4_0(&self) -> u32 {
        (self.bits & Key40::KEY4_0_MASK) >> Key40::KEY4_0_OFFSET
    }

}

impl Default for Key40Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Key40 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Key40Val {
        Key40Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Key40Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Key40Val) -> Key40Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// KEY4_1 Register
///
/// Bits 31:16 of OTP access key 4 (ECC)
///
/// Address offset: 0x1ec2
///
/// ## Fields
///
/// - **KEY4_1** `[15:0]` (RO)
#[repr(transparent)]
pub struct Key41 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Key41 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Key41 {
    // ======== Field Constants ========

    /// **KEY4_1**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const KEY4_1_OFFSET: u32 = 0;
    pub const KEY4_1_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **KEY4_1** `[15:0]` (RO)
    #[inline(always)]
    pub fn key4_1(&self) -> u32 {
        (self.value.read_bits() & Self::KEY4_1_MASK) >> Self::KEY4_1_OFFSET
    }

}

/// KEY4_1 value type for batch read/modify/write operations
///
/// Use `key41.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `key41.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Key41Val {
    bits: u32,
}

impl Key41Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY4_1** `[15:0]`
    #[inline(always)]
    pub const fn key4_1(&self) -> u32 {
        (self.bits & Key41::KEY4_1_MASK) >> Key41::KEY4_1_OFFSET
    }

}

impl Default for Key41Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Key41 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Key41Val {
        Key41Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Key41Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Key41Val) -> Key41Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// KEY4_2 Register
///
/// Bits 47:32 of OTP access key 4 (ECC)
///
/// Address offset: 0x1ec4
///
/// ## Fields
///
/// - **KEY4_2** `[15:0]` (RO)
#[repr(transparent)]
pub struct Key42 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Key42 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Key42 {
    // ======== Field Constants ========

    /// **KEY4_2**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const KEY4_2_OFFSET: u32 = 0;
    pub const KEY4_2_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **KEY4_2** `[15:0]` (RO)
    #[inline(always)]
    pub fn key4_2(&self) -> u32 {
        (self.value.read_bits() & Self::KEY4_2_MASK) >> Self::KEY4_2_OFFSET
    }

}

/// KEY4_2 value type for batch read/modify/write operations
///
/// Use `key42.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `key42.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Key42Val {
    bits: u32,
}

impl Key42Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY4_2** `[15:0]`
    #[inline(always)]
    pub const fn key4_2(&self) -> u32 {
        (self.bits & Key42::KEY4_2_MASK) >> Key42::KEY4_2_OFFSET
    }

}

impl Default for Key42Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Key42 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Key42Val {
        Key42Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Key42Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Key42Val) -> Key42Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// KEY4_3 Register
///
/// Bits 63:48 of OTP access key 4 (ECC)
///
/// Address offset: 0x1ec6
///
/// ## Fields
///
/// - **KEY4_3** `[15:0]` (RO)
#[repr(transparent)]
pub struct Key43 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Key43 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Key43 {
    // ======== Field Constants ========

    /// **KEY4_3**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const KEY4_3_OFFSET: u32 = 0;
    pub const KEY4_3_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **KEY4_3** `[15:0]` (RO)
    #[inline(always)]
    pub fn key4_3(&self) -> u32 {
        (self.value.read_bits() & Self::KEY4_3_MASK) >> Self::KEY4_3_OFFSET
    }

}

/// KEY4_3 value type for batch read/modify/write operations
///
/// Use `key43.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `key43.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Key43Val {
    bits: u32,
}

impl Key43Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY4_3** `[15:0]`
    #[inline(always)]
    pub const fn key4_3(&self) -> u32 {
        (self.bits & Key43::KEY4_3_MASK) >> Key43::KEY4_3_OFFSET
    }

}

impl Default for Key43Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Key43 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Key43Val {
        Key43Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Key43Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Key43Val) -> Key43Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// KEY4_4 Register
///
/// Bits 79:64 of OTP access key 4 (ECC)
///
/// Address offset: 0x1ec8
///
/// ## Fields
///
/// - **KEY4_4** `[15:0]` (RO)
#[repr(transparent)]
pub struct Key44 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Key44 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Key44 {
    // ======== Field Constants ========

    /// **KEY4_4**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const KEY4_4_OFFSET: u32 = 0;
    pub const KEY4_4_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **KEY4_4** `[15:0]` (RO)
    #[inline(always)]
    pub fn key4_4(&self) -> u32 {
        (self.value.read_bits() & Self::KEY4_4_MASK) >> Self::KEY4_4_OFFSET
    }

}

/// KEY4_4 value type for batch read/modify/write operations
///
/// Use `key44.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `key44.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Key44Val {
    bits: u32,
}

impl Key44Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY4_4** `[15:0]`
    #[inline(always)]
    pub const fn key4_4(&self) -> u32 {
        (self.bits & Key44::KEY4_4_MASK) >> Key44::KEY4_4_OFFSET
    }

}

impl Default for Key44Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Key44 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Key44Val {
        Key44Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Key44Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Key44Val) -> Key44Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// KEY4_5 Register
///
/// Bits 95:80 of OTP access key 4 (ECC)
///
/// Address offset: 0x1eca
///
/// ## Fields
///
/// - **KEY4_5** `[15:0]` (RO)
#[repr(transparent)]
pub struct Key45 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Key45 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Key45 {
    // ======== Field Constants ========

    /// **KEY4_5**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const KEY4_5_OFFSET: u32 = 0;
    pub const KEY4_5_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **KEY4_5** `[15:0]` (RO)
    #[inline(always)]
    pub fn key4_5(&self) -> u32 {
        (self.value.read_bits() & Self::KEY4_5_MASK) >> Self::KEY4_5_OFFSET
    }

}

/// KEY4_5 value type for batch read/modify/write operations
///
/// Use `key45.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `key45.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Key45Val {
    bits: u32,
}

impl Key45Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY4_5** `[15:0]`
    #[inline(always)]
    pub const fn key4_5(&self) -> u32 {
        (self.bits & Key45::KEY4_5_MASK) >> Key45::KEY4_5_OFFSET
    }

}

impl Default for Key45Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Key45 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Key45Val {
        Key45Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Key45Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Key45Val) -> Key45Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// KEY4_6 Register
///
/// Bits 111:96 of OTP access key 4 (ECC)
///
/// Address offset: 0x1ecc
///
/// ## Fields
///
/// - **KEY4_6** `[15:0]` (RO)
#[repr(transparent)]
pub struct Key46 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Key46 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Key46 {
    // ======== Field Constants ========

    /// **KEY4_6**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const KEY4_6_OFFSET: u32 = 0;
    pub const KEY4_6_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **KEY4_6** `[15:0]` (RO)
    #[inline(always)]
    pub fn key4_6(&self) -> u32 {
        (self.value.read_bits() & Self::KEY4_6_MASK) >> Self::KEY4_6_OFFSET
    }

}

/// KEY4_6 value type for batch read/modify/write operations
///
/// Use `key46.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `key46.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Key46Val {
    bits: u32,
}

impl Key46Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY4_6** `[15:0]`
    #[inline(always)]
    pub const fn key4_6(&self) -> u32 {
        (self.bits & Key46::KEY4_6_MASK) >> Key46::KEY4_6_OFFSET
    }

}

impl Default for Key46Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Key46 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Key46Val {
        Key46Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Key46Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Key46Val) -> Key46Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// KEY4_7 Register
///
/// Bits 127:112 of OTP access key 4 (ECC)
///
/// Address offset: 0x1ece
///
/// ## Fields
///
/// - **KEY4_7** `[15:0]` (RO)
#[repr(transparent)]
pub struct Key47 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Key47 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Key47 {
    // ======== Field Constants ========

    /// **KEY4_7**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const KEY4_7_OFFSET: u32 = 0;
    pub const KEY4_7_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **KEY4_7** `[15:0]` (RO)
    #[inline(always)]
    pub fn key4_7(&self) -> u32 {
        (self.value.read_bits() & Self::KEY4_7_MASK) >> Self::KEY4_7_OFFSET
    }

}

/// KEY4_7 value type for batch read/modify/write operations
///
/// Use `key47.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `key47.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Key47Val {
    bits: u32,
}

impl Key47Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY4_7** `[15:0]`
    #[inline(always)]
    pub const fn key4_7(&self) -> u32 {
        (self.bits & Key47::KEY4_7_MASK) >> Key47::KEY4_7_OFFSET
    }

}

impl Default for Key47Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Key47 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Key47Val {
        Key47Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Key47Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Key47Val) -> Key47Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// KEY5_0 Register
///
/// Bits 15:0 of OTP access key 5 (ECC)
///
/// Address offset: 0x1ed0
///
/// ## Fields
///
/// - **KEY5_0** `[15:0]` (RO)
#[repr(transparent)]
pub struct Key50 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Key50 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Key50 {
    // ======== Field Constants ========

    /// **KEY5_0**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const KEY5_0_OFFSET: u32 = 0;
    pub const KEY5_0_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **KEY5_0** `[15:0]` (RO)
    #[inline(always)]
    pub fn key5_0(&self) -> u32 {
        (self.value.read_bits() & Self::KEY5_0_MASK) >> Self::KEY5_0_OFFSET
    }

}

/// KEY5_0 value type for batch read/modify/write operations
///
/// Use `key50.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `key50.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Key50Val {
    bits: u32,
}

impl Key50Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY5_0** `[15:0]`
    #[inline(always)]
    pub const fn key5_0(&self) -> u32 {
        (self.bits & Key50::KEY5_0_MASK) >> Key50::KEY5_0_OFFSET
    }

}

impl Default for Key50Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Key50 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Key50Val {
        Key50Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Key50Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Key50Val) -> Key50Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// KEY5_1 Register
///
/// Bits 31:16 of OTP access key 5 (ECC)
///
/// Address offset: 0x1ed2
///
/// ## Fields
///
/// - **KEY5_1** `[15:0]` (RO)
#[repr(transparent)]
pub struct Key51 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Key51 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Key51 {
    // ======== Field Constants ========

    /// **KEY5_1**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const KEY5_1_OFFSET: u32 = 0;
    pub const KEY5_1_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **KEY5_1** `[15:0]` (RO)
    #[inline(always)]
    pub fn key5_1(&self) -> u32 {
        (self.value.read_bits() & Self::KEY5_1_MASK) >> Self::KEY5_1_OFFSET
    }

}

/// KEY5_1 value type for batch read/modify/write operations
///
/// Use `key51.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `key51.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Key51Val {
    bits: u32,
}

impl Key51Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY5_1** `[15:0]`
    #[inline(always)]
    pub const fn key5_1(&self) -> u32 {
        (self.bits & Key51::KEY5_1_MASK) >> Key51::KEY5_1_OFFSET
    }

}

impl Default for Key51Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Key51 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Key51Val {
        Key51Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Key51Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Key51Val) -> Key51Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// KEY5_2 Register
///
/// Bits 47:32 of OTP access key 5 (ECC)
///
/// Address offset: 0x1ed4
///
/// ## Fields
///
/// - **KEY5_2** `[15:0]` (RO)
#[repr(transparent)]
pub struct Key52 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Key52 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Key52 {
    // ======== Field Constants ========

    /// **KEY5_2**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const KEY5_2_OFFSET: u32 = 0;
    pub const KEY5_2_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **KEY5_2** `[15:0]` (RO)
    #[inline(always)]
    pub fn key5_2(&self) -> u32 {
        (self.value.read_bits() & Self::KEY5_2_MASK) >> Self::KEY5_2_OFFSET
    }

}

/// KEY5_2 value type for batch read/modify/write operations
///
/// Use `key52.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `key52.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Key52Val {
    bits: u32,
}

impl Key52Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY5_2** `[15:0]`
    #[inline(always)]
    pub const fn key5_2(&self) -> u32 {
        (self.bits & Key52::KEY5_2_MASK) >> Key52::KEY5_2_OFFSET
    }

}

impl Default for Key52Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Key52 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Key52Val {
        Key52Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Key52Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Key52Val) -> Key52Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// KEY5_3 Register
///
/// Bits 63:48 of OTP access key 5 (ECC)
///
/// Address offset: 0x1ed6
///
/// ## Fields
///
/// - **KEY5_3** `[15:0]` (RO)
#[repr(transparent)]
pub struct Key53 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Key53 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Key53 {
    // ======== Field Constants ========

    /// **KEY5_3**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const KEY5_3_OFFSET: u32 = 0;
    pub const KEY5_3_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **KEY5_3** `[15:0]` (RO)
    #[inline(always)]
    pub fn key5_3(&self) -> u32 {
        (self.value.read_bits() & Self::KEY5_3_MASK) >> Self::KEY5_3_OFFSET
    }

}

/// KEY5_3 value type for batch read/modify/write operations
///
/// Use `key53.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `key53.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Key53Val {
    bits: u32,
}

impl Key53Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY5_3** `[15:0]`
    #[inline(always)]
    pub const fn key5_3(&self) -> u32 {
        (self.bits & Key53::KEY5_3_MASK) >> Key53::KEY5_3_OFFSET
    }

}

impl Default for Key53Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Key53 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Key53Val {
        Key53Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Key53Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Key53Val) -> Key53Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// KEY5_4 Register
///
/// Bits 79:64 of OTP access key 5 (ECC)
///
/// Address offset: 0x1ed8
///
/// ## Fields
///
/// - **KEY5_4** `[15:0]` (RO)
#[repr(transparent)]
pub struct Key54 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Key54 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Key54 {
    // ======== Field Constants ========

    /// **KEY5_4**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const KEY5_4_OFFSET: u32 = 0;
    pub const KEY5_4_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **KEY5_4** `[15:0]` (RO)
    #[inline(always)]
    pub fn key5_4(&self) -> u32 {
        (self.value.read_bits() & Self::KEY5_4_MASK) >> Self::KEY5_4_OFFSET
    }

}

/// KEY5_4 value type for batch read/modify/write operations
///
/// Use `key54.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `key54.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Key54Val {
    bits: u32,
}

impl Key54Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY5_4** `[15:0]`
    #[inline(always)]
    pub const fn key5_4(&self) -> u32 {
        (self.bits & Key54::KEY5_4_MASK) >> Key54::KEY5_4_OFFSET
    }

}

impl Default for Key54Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Key54 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Key54Val {
        Key54Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Key54Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Key54Val) -> Key54Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// KEY5_5 Register
///
/// Bits 95:80 of OTP access key 5 (ECC)
///
/// Address offset: 0x1eda
///
/// ## Fields
///
/// - **KEY5_5** `[15:0]` (RO)
#[repr(transparent)]
pub struct Key55 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Key55 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Key55 {
    // ======== Field Constants ========

    /// **KEY5_5**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const KEY5_5_OFFSET: u32 = 0;
    pub const KEY5_5_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **KEY5_5** `[15:0]` (RO)
    #[inline(always)]
    pub fn key5_5(&self) -> u32 {
        (self.value.read_bits() & Self::KEY5_5_MASK) >> Self::KEY5_5_OFFSET
    }

}

/// KEY5_5 value type for batch read/modify/write operations
///
/// Use `key55.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `key55.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Key55Val {
    bits: u32,
}

impl Key55Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY5_5** `[15:0]`
    #[inline(always)]
    pub const fn key5_5(&self) -> u32 {
        (self.bits & Key55::KEY5_5_MASK) >> Key55::KEY5_5_OFFSET
    }

}

impl Default for Key55Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Key55 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Key55Val {
        Key55Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Key55Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Key55Val) -> Key55Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// KEY5_6 Register
///
/// Bits 111:96 of OTP access key 5 (ECC)
///
/// Address offset: 0x1edc
///
/// ## Fields
///
/// - **KEY5_6** `[15:0]` (RO)
#[repr(transparent)]
pub struct Key56 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Key56 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Key56 {
    // ======== Field Constants ========

    /// **KEY5_6**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const KEY5_6_OFFSET: u32 = 0;
    pub const KEY5_6_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **KEY5_6** `[15:0]` (RO)
    #[inline(always)]
    pub fn key5_6(&self) -> u32 {
        (self.value.read_bits() & Self::KEY5_6_MASK) >> Self::KEY5_6_OFFSET
    }

}

/// KEY5_6 value type for batch read/modify/write operations
///
/// Use `key56.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `key56.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Key56Val {
    bits: u32,
}

impl Key56Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY5_6** `[15:0]`
    #[inline(always)]
    pub const fn key5_6(&self) -> u32 {
        (self.bits & Key56::KEY5_6_MASK) >> Key56::KEY5_6_OFFSET
    }

}

impl Default for Key56Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Key56 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Key56Val {
        Key56Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Key56Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Key56Val) -> Key56Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// KEY5_7 Register
///
/// Bits 127:112 of OTP access key 5 (ECC)
///
/// Address offset: 0x1ede
///
/// ## Fields
///
/// - **KEY5_7** `[15:0]` (RO)
#[repr(transparent)]
pub struct Key57 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Key57 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Key57 {
    // ======== Field Constants ========

    /// **KEY5_7**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const KEY5_7_OFFSET: u32 = 0;
    pub const KEY5_7_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **KEY5_7** `[15:0]` (RO)
    #[inline(always)]
    pub fn key5_7(&self) -> u32 {
        (self.value.read_bits() & Self::KEY5_7_MASK) >> Self::KEY5_7_OFFSET
    }

}

/// KEY5_7 value type for batch read/modify/write operations
///
/// Use `key57.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `key57.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Key57Val {
    bits: u32,
}

impl Key57Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY5_7** `[15:0]`
    #[inline(always)]
    pub const fn key5_7(&self) -> u32 {
        (self.bits & Key57::KEY5_7_MASK) >> Key57::KEY5_7_OFFSET
    }

}

impl Default for Key57Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Key57 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Key57Val {
        Key57Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Key57Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Key57Val) -> Key57Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// KEY6_0 Register
///
/// Bits 15:0 of OTP access key 6 (ECC)
///
/// Address offset: 0x1ee0
///
/// ## Fields
///
/// - **KEY6_0** `[15:0]` (RO)
#[repr(transparent)]
pub struct Key60 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Key60 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Key60 {
    // ======== Field Constants ========

    /// **KEY6_0**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const KEY6_0_OFFSET: u32 = 0;
    pub const KEY6_0_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **KEY6_0** `[15:0]` (RO)
    #[inline(always)]
    pub fn key6_0(&self) -> u32 {
        (self.value.read_bits() & Self::KEY6_0_MASK) >> Self::KEY6_0_OFFSET
    }

}

/// KEY6_0 value type for batch read/modify/write operations
///
/// Use `key60.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `key60.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Key60Val {
    bits: u32,
}

impl Key60Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY6_0** `[15:0]`
    #[inline(always)]
    pub const fn key6_0(&self) -> u32 {
        (self.bits & Key60::KEY6_0_MASK) >> Key60::KEY6_0_OFFSET
    }

}

impl Default for Key60Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Key60 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Key60Val {
        Key60Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Key60Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Key60Val) -> Key60Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// KEY6_1 Register
///
/// Bits 31:16 of OTP access key 6 (ECC)
///
/// Address offset: 0x1ee2
///
/// ## Fields
///
/// - **KEY6_1** `[15:0]` (RO)
#[repr(transparent)]
pub struct Key61 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Key61 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Key61 {
    // ======== Field Constants ========

    /// **KEY6_1**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const KEY6_1_OFFSET: u32 = 0;
    pub const KEY6_1_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **KEY6_1** `[15:0]` (RO)
    #[inline(always)]
    pub fn key6_1(&self) -> u32 {
        (self.value.read_bits() & Self::KEY6_1_MASK) >> Self::KEY6_1_OFFSET
    }

}

/// KEY6_1 value type for batch read/modify/write operations
///
/// Use `key61.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `key61.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Key61Val {
    bits: u32,
}

impl Key61Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY6_1** `[15:0]`
    #[inline(always)]
    pub const fn key6_1(&self) -> u32 {
        (self.bits & Key61::KEY6_1_MASK) >> Key61::KEY6_1_OFFSET
    }

}

impl Default for Key61Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Key61 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Key61Val {
        Key61Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Key61Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Key61Val) -> Key61Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// KEY6_2 Register
///
/// Bits 47:32 of OTP access key 6 (ECC)
///
/// Address offset: 0x1ee4
///
/// ## Fields
///
/// - **KEY6_2** `[15:0]` (RO)
#[repr(transparent)]
pub struct Key62 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Key62 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Key62 {
    // ======== Field Constants ========

    /// **KEY6_2**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const KEY6_2_OFFSET: u32 = 0;
    pub const KEY6_2_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **KEY6_2** `[15:0]` (RO)
    #[inline(always)]
    pub fn key6_2(&self) -> u32 {
        (self.value.read_bits() & Self::KEY6_2_MASK) >> Self::KEY6_2_OFFSET
    }

}

/// KEY6_2 value type for batch read/modify/write operations
///
/// Use `key62.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `key62.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Key62Val {
    bits: u32,
}

impl Key62Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY6_2** `[15:0]`
    #[inline(always)]
    pub const fn key6_2(&self) -> u32 {
        (self.bits & Key62::KEY6_2_MASK) >> Key62::KEY6_2_OFFSET
    }

}

impl Default for Key62Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Key62 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Key62Val {
        Key62Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Key62Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Key62Val) -> Key62Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// KEY6_3 Register
///
/// Bits 63:48 of OTP access key 6 (ECC)
///
/// Address offset: 0x1ee6
///
/// ## Fields
///
/// - **KEY6_3** `[15:0]` (RO)
#[repr(transparent)]
pub struct Key63 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Key63 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Key63 {
    // ======== Field Constants ========

    /// **KEY6_3**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const KEY6_3_OFFSET: u32 = 0;
    pub const KEY6_3_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **KEY6_3** `[15:0]` (RO)
    #[inline(always)]
    pub fn key6_3(&self) -> u32 {
        (self.value.read_bits() & Self::KEY6_3_MASK) >> Self::KEY6_3_OFFSET
    }

}

/// KEY6_3 value type for batch read/modify/write operations
///
/// Use `key63.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `key63.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Key63Val {
    bits: u32,
}

impl Key63Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY6_3** `[15:0]`
    #[inline(always)]
    pub const fn key6_3(&self) -> u32 {
        (self.bits & Key63::KEY6_3_MASK) >> Key63::KEY6_3_OFFSET
    }

}

impl Default for Key63Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Key63 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Key63Val {
        Key63Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Key63Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Key63Val) -> Key63Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// KEY6_4 Register
///
/// Bits 79:64 of OTP access key 6 (ECC)
///
/// Address offset: 0x1ee8
///
/// ## Fields
///
/// - **KEY6_4** `[15:0]` (RO)
#[repr(transparent)]
pub struct Key64 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Key64 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Key64 {
    // ======== Field Constants ========

    /// **KEY6_4**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const KEY6_4_OFFSET: u32 = 0;
    pub const KEY6_4_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **KEY6_4** `[15:0]` (RO)
    #[inline(always)]
    pub fn key6_4(&self) -> u32 {
        (self.value.read_bits() & Self::KEY6_4_MASK) >> Self::KEY6_4_OFFSET
    }

}

/// KEY6_4 value type for batch read/modify/write operations
///
/// Use `key64.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `key64.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Key64Val {
    bits: u32,
}

impl Key64Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY6_4** `[15:0]`
    #[inline(always)]
    pub const fn key6_4(&self) -> u32 {
        (self.bits & Key64::KEY6_4_MASK) >> Key64::KEY6_4_OFFSET
    }

}

impl Default for Key64Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Key64 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Key64Val {
        Key64Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Key64Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Key64Val) -> Key64Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// KEY6_5 Register
///
/// Bits 95:80 of OTP access key 6 (ECC)
///
/// Address offset: 0x1eea
///
/// ## Fields
///
/// - **KEY6_5** `[15:0]` (RO)
#[repr(transparent)]
pub struct Key65 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Key65 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Key65 {
    // ======== Field Constants ========

    /// **KEY6_5**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const KEY6_5_OFFSET: u32 = 0;
    pub const KEY6_5_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **KEY6_5** `[15:0]` (RO)
    #[inline(always)]
    pub fn key6_5(&self) -> u32 {
        (self.value.read_bits() & Self::KEY6_5_MASK) >> Self::KEY6_5_OFFSET
    }

}

/// KEY6_5 value type for batch read/modify/write operations
///
/// Use `key65.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `key65.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Key65Val {
    bits: u32,
}

impl Key65Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY6_5** `[15:0]`
    #[inline(always)]
    pub const fn key6_5(&self) -> u32 {
        (self.bits & Key65::KEY6_5_MASK) >> Key65::KEY6_5_OFFSET
    }

}

impl Default for Key65Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Key65 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Key65Val {
        Key65Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Key65Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Key65Val) -> Key65Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// KEY6_6 Register
///
/// Bits 111:96 of OTP access key 6 (ECC)
///
/// Address offset: 0x1eec
///
/// ## Fields
///
/// - **KEY6_6** `[15:0]` (RO)
#[repr(transparent)]
pub struct Key66 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Key66 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Key66 {
    // ======== Field Constants ========

    /// **KEY6_6**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const KEY6_6_OFFSET: u32 = 0;
    pub const KEY6_6_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **KEY6_6** `[15:0]` (RO)
    #[inline(always)]
    pub fn key6_6(&self) -> u32 {
        (self.value.read_bits() & Self::KEY6_6_MASK) >> Self::KEY6_6_OFFSET
    }

}

/// KEY6_6 value type for batch read/modify/write operations
///
/// Use `key66.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `key66.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Key66Val {
    bits: u32,
}

impl Key66Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY6_6** `[15:0]`
    #[inline(always)]
    pub const fn key6_6(&self) -> u32 {
        (self.bits & Key66::KEY6_6_MASK) >> Key66::KEY6_6_OFFSET
    }

}

impl Default for Key66Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Key66 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Key66Val {
        Key66Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Key66Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Key66Val) -> Key66Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// KEY6_7 Register
///
/// Bits 127:112 of OTP access key 6 (ECC)
///
/// Address offset: 0x1eee
///
/// ## Fields
///
/// - **KEY6_7** `[15:0]` (RO)
#[repr(transparent)]
pub struct Key67 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for Key67 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Key67 {
    // ======== Field Constants ========

    /// **KEY6_7**
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const KEY6_7_OFFSET: u32 = 0;
    pub const KEY6_7_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **KEY6_7** `[15:0]` (RO)
    #[inline(always)]
    pub fn key6_7(&self) -> u32 {
        (self.value.read_bits() & Self::KEY6_7_MASK) >> Self::KEY6_7_OFFSET
    }

}

/// KEY6_7 value type for batch read/modify/write operations
///
/// Use `key67.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `key67.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Key67Val {
    bits: u32,
}

impl Key67Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY6_7** `[15:0]`
    #[inline(always)]
    pub const fn key6_7(&self) -> u32 {
        (self.bits & Key67::KEY6_7_MASK) >> Key67::KEY6_7_OFFSET
    }

}

impl Default for Key67Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Key67 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Key67Val {
        Key67Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Key67Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Key67Val) -> Key67Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }

}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// Bits 15:0 of public device ID. (ECC)
    /// The CHIPID0..3 rows contain a 64-bit random identifier for this chip, which can be read from the USB bootloader PICOBOOT interface or from the get_sys_info ROM API.
    /// The number of random bits makes the occurrence of twins exceedingly unlikely: for example, a fleet of a hundred million devices has a 99.97% probability of no twinned IDs. This is estimated to be lower than the occurrence of process errors in the assignment of sequential random IDs, and for practical purposes CHIPID may be treated as unique.
    pub chipid0: Chipid0,
    /// 0x0002
    /// Bits 31:16 of public device ID (ECC)
    pub chipid1: Chipid1,
    /// 0x0004
    /// Bits 47:32 of public device ID (ECC)
    pub chipid2: Chipid2,
    /// 0x0006
    /// Bits 63:48 of public device ID (ECC)
    pub chipid3: Chipid3,
    /// 0x0008
    /// Bits 15:0 of private per-device random number (ECC)
    /// The RANDID0..7 rows form a 128-bit random number generated during device test.
    /// This ID is not exposed through the USB PICOBOOT GET_INFO command or the ROM `get_sys_info()` API. However note that the USB PICOBOOT OTP access point can read the entirety of page 0, so this value is not meaningfully private unless the USB PICOBOOT interface is disabled via the DISABLE_BOOTSEL_USB_PICOBOOT_IFC flag in BOOT_FLAGS0.
    pub randid0: Randid0,
    /// 0x000a
    /// Bits 31:16 of private per-device random number (ECC)
    pub randid1: Randid1,
    /// 0x000c
    /// Bits 47:32 of private per-device random number (ECC)
    pub randid2: Randid2,
    /// 0x000e
    /// Bits 63:48 of private per-device random number (ECC)
    pub randid3: Randid3,
    /// 0x0010
    /// Bits 79:64 of private per-device random number (ECC)
    pub randid4: Randid4,
    /// 0x0012
    /// Bits 95:80 of private per-device random number (ECC)
    pub randid5: Randid5,
    /// 0x0014
    /// Bits 111:96 of private per-device random number (ECC)
    pub randid6: Randid6,
    /// 0x0016
    /// Bits 127:112 of private per-device random number (ECC)
    pub randid7: Randid7,
    _reserved0: [u8; 6],
    /// 0x0020
    /// Ring oscillator frequency in kHz, measured during manufacturing (ECC)
    /// This is measured at 1.1 V, at room temperature, with the ROSC configuration registers in their reset state.
    pub rosc_calib: RoscCalib,
    /// 0x0022
    /// Low-power oscillator frequency in Hz, measured during manufacturing (ECC)
    /// This is measured at 1.1V, at room temperature, with the LPOSC trim register in its reset state.
    pub lposc_calib: LposcCalib,
    _reserved1: [u8; 10],
    /// 0x0030
    /// The number of main user GPIOs (bank 0). Should read 48 in the QFN80 package, and 30 in the QFN60 package. (ECC)
    pub num_gpios: NumGpios,
    _reserved2: [u8; 56],
    /// 0x006c
    /// Lower 16 bits of CRC32 of OTP addresses 0x00 through 0x6b (polynomial 0x4c11db7, input reflected, output reflected, seed all-ones, final XOR all-ones) (ECC)
    pub info_crc0: InfoCrc0,
    /// 0x006e
    /// Upper 16 bits of CRC32 of OTP addresses 0x00 through 0x6b (ECC)
    pub info_crc1: InfoCrc1,
    _reserved3: [u8; 54],
    /// 0x00a8
    /// Stores information about external flash device(s). (ECC)
    /// Assumed to be valid if BOOT_FLAGS0_FLASH_DEVINFO_ENABLE is set.
    pub flash_devinfo: FlashDevinfo,
    /// 0x00aa
    /// Gap between partition table slot 0 and slot 1 at the start of flash (the default size is 4096 bytes) (ECC) Enabled by the OVERRIDE_FLASH_PARTITION_SLOT_SIZE bit in BOOT_FLAGS, the size is 4096 * (value + 1)
    pub flash_partition_slot_size: FlashPartitionSlotSize,
    /// 0x00ac
    /// Pin configuration for LED status, used by USB bootloader. (ECC)
    /// Must be valid if BOOT_FLAGS0_ENABLE_BOOTSEL_LED is set.
    pub bootsel_led_cfg: BootselLedCfg,
    /// 0x00ae
    /// Optional PLL configuration for BOOTSEL mode. (ECC)
    /// This should be configured to produce an exact 48 MHz based on the crystal oscillator frequency. User mode software may also use this value to calculate the expected crystal frequency based on an assumed 48 MHz PLL output.
    /// If no configuration is given, the crystal is assumed to be 12 MHz.
    /// The PLL frequency can be calculated as:
    /// PLL out = (XOSC frequency / (REFDIV+1)) x FBDIV / (POSTDIV1 x POSTDIV2)
    /// Conversely the crystal frequency can be calculated as:
    /// XOSC frequency = 48 MHz x (REFDIV+1) x (POSTDIV1 x POSTDIV2) / FBDIV
    /// (Note the +1 on REFDIV is because the value stored in this OTP location is the actual divisor value minus one.)
    /// Used if and only if ENABLE_BOOTSEL_NON_DEFAULT_PLL_XOSC_CFG is set in BOOT_FLAGS0. That bit should be set only after this row and BOOTSEL_XOSC_CFG are both correctly programmed.
    pub bootsel_pll_cfg: BootselPllCfg,
    /// 0x00b0
    /// Non-default crystal oscillator configuration for the USB bootloader. (ECC)
    /// These values may also be used by user code configuring the crystal oscillator.
    /// Used if and only if ENABLE_BOOTSEL_NON_DEFAULT_PLL_XOSC_CFG is set in BOOT_FLAGS0. That bit should be set only after this row and BOOTSEL_PLL_CFG are both correctly programmed.
    pub bootsel_xosc_cfg: BootselXoscCfg,
    _reserved4: [u8; 4],
    /// 0x00b8
    /// Row index of the USB_WHITE_LABEL structure within OTP (ECC)
    /// The table has 16 rows, each of which are also ECC and marked valid by the corresponding valid bit in USB_BOOT_FLAGS (ECC).
    /// The entries are either _VALUEs where the 16 bit value is used as is, or _STRDEFs which acts as a pointers to a string value.
    /// The value stored in a _STRDEF is two separate bytes: The low seven bits of the first (LSB) byte indicates the number of characters in the string, and the top bit of the first (LSB) byte if set to indicate that each character in the string is two bytes (Unicode) versus one byte if unset. The second (MSB) byte represents the location of the string data, and is encoded as the number of rows from this USB_WHITE_LABEL_ADDR; i.e. the row of the start of the string is USB_WHITE_LABEL_ADDR value + msb_byte.
    /// In each case, the corresponding valid bit enables replacing the default value for the corresponding item provided by the boot rom.
    /// Note that Unicode _STRDEFs are only supported for USB_DEVICE_PRODUCT_STRDEF, USB_DEVICE_SERIAL_NUMBER_STRDEF and USB_DEVICE_MANUFACTURER_STRDEF. Unicode values will be ignored if specified for other fields, and non-unicode values for these three items will be converted to Unicode characters by setting the upper 8 bits to zero.
    /// Note that if the USB_WHITE_LABEL structure or the corresponding strings are not readable by BOOTSEL mode based on OTP permissions, or if alignment requirements are not met, then the corresponding default values are used.
    /// The index values indicate where each field is located (row USB_WHITE_LABEL_ADDR value + index):
    pub usb_white_label_addr: UsbWhiteLabelAddr,
    /// 0x00bc
    /// OTP start row for the OTP boot image. (ECC)
    /// If OTP boot is enabled, the bootrom will load from this location into SRAM and then directly enter the loaded image. Note that the image must be signed if SECURE_BOOT_ENABLE is set. The image itself is assumed to be ECC-protected.
    /// This must be an even number. Equivalently, the OTP boot image must start at a word-aligned location in the ECC read data address window.
    pub otpboot_src: OtpbootSrc,
    /// 0x00be
    /// Length in rows of the OTP boot image. (ECC)
    /// OTPBOOT_LEN must be even. The total image size must be a multiple of 4 bytes (32 bits).
    pub otpboot_len: OtpbootLen,
    /// 0x00c0
    /// Bits 15:0 of the OTP boot image load destination (and entry point). (ECC)
    /// This must be a location in main SRAM (main SRAM is addresses 0x20000000 through 0x20082000) and must be word-aligned.
    pub otpboot_dst0: OtpbootDst0,
    /// 0x00c2
    /// Bits 31:16 of the OTP boot image load destination (and entry point). (ECC)
    /// This must be a location in main SRAM (main SRAM is addresses 0x20000000 through 0x20082000) and must be word-aligned.
    pub otpboot_dst1: OtpbootDst1,
    _reserved5: [u8; 58],
    /// 0x0100
    /// Bits 15:0 of SHA-256 hash of boot key 0 (ECC)
    pub bootkey0_0: Bootkey00,
    /// 0x0102
    /// Bits 31:16 of SHA-256 hash of boot key 0 (ECC)
    pub bootkey0_1: Bootkey01,
    /// 0x0104
    /// Bits 47:32 of SHA-256 hash of boot key 0 (ECC)
    pub bootkey0_2: Bootkey02,
    /// 0x0106
    /// Bits 63:48 of SHA-256 hash of boot key 0 (ECC)
    pub bootkey0_3: Bootkey03,
    /// 0x0108
    /// Bits 79:64 of SHA-256 hash of boot key 0 (ECC)
    pub bootkey0_4: Bootkey04,
    /// 0x010a
    /// Bits 95:80 of SHA-256 hash of boot key 0 (ECC)
    pub bootkey0_5: Bootkey05,
    /// 0x010c
    /// Bits 111:96 of SHA-256 hash of boot key 0 (ECC)
    pub bootkey0_6: Bootkey06,
    /// 0x010e
    /// Bits 127:112 of SHA-256 hash of boot key 0 (ECC)
    pub bootkey0_7: Bootkey07,
    /// 0x0110
    /// Bits 143:128 of SHA-256 hash of boot key 0 (ECC)
    pub bootkey0_8: Bootkey08,
    /// 0x0112
    /// Bits 159:144 of SHA-256 hash of boot key 0 (ECC)
    pub bootkey0_9: Bootkey09,
    /// 0x0114
    /// Bits 175:160 of SHA-256 hash of boot key 0 (ECC)
    pub bootkey0_10: Bootkey010,
    /// 0x0116
    /// Bits 191:176 of SHA-256 hash of boot key 0 (ECC)
    pub bootkey0_11: Bootkey011,
    /// 0x0118
    /// Bits 207:192 of SHA-256 hash of boot key 0 (ECC)
    pub bootkey0_12: Bootkey012,
    /// 0x011a
    /// Bits 223:208 of SHA-256 hash of boot key 0 (ECC)
    pub bootkey0_13: Bootkey013,
    /// 0x011c
    /// Bits 239:224 of SHA-256 hash of boot key 0 (ECC)
    pub bootkey0_14: Bootkey014,
    /// 0x011e
    /// Bits 255:240 of SHA-256 hash of boot key 0 (ECC)
    pub bootkey0_15: Bootkey015,
    /// 0x0120
    /// Bits 15:0 of SHA-256 hash of boot key 1 (ECC)
    pub bootkey1_0: Bootkey10,
    /// 0x0122
    /// Bits 31:16 of SHA-256 hash of boot key 1 (ECC)
    pub bootkey1_1: Bootkey11,
    /// 0x0124
    /// Bits 47:32 of SHA-256 hash of boot key 1 (ECC)
    pub bootkey1_2: Bootkey12,
    /// 0x0126
    /// Bits 63:48 of SHA-256 hash of boot key 1 (ECC)
    pub bootkey1_3: Bootkey13,
    /// 0x0128
    /// Bits 79:64 of SHA-256 hash of boot key 1 (ECC)
    pub bootkey1_4: Bootkey14,
    /// 0x012a
    /// Bits 95:80 of SHA-256 hash of boot key 1 (ECC)
    pub bootkey1_5: Bootkey15,
    /// 0x012c
    /// Bits 111:96 of SHA-256 hash of boot key 1 (ECC)
    pub bootkey1_6: Bootkey16,
    /// 0x012e
    /// Bits 127:112 of SHA-256 hash of boot key 1 (ECC)
    pub bootkey1_7: Bootkey17,
    /// 0x0130
    /// Bits 143:128 of SHA-256 hash of boot key 1 (ECC)
    pub bootkey1_8: Bootkey18,
    /// 0x0132
    /// Bits 159:144 of SHA-256 hash of boot key 1 (ECC)
    pub bootkey1_9: Bootkey19,
    /// 0x0134
    /// Bits 175:160 of SHA-256 hash of boot key 1 (ECC)
    pub bootkey1_10: Bootkey110,
    /// 0x0136
    /// Bits 191:176 of SHA-256 hash of boot key 1 (ECC)
    pub bootkey1_11: Bootkey111,
    /// 0x0138
    /// Bits 207:192 of SHA-256 hash of boot key 1 (ECC)
    pub bootkey1_12: Bootkey112,
    /// 0x013a
    /// Bits 223:208 of SHA-256 hash of boot key 1 (ECC)
    pub bootkey1_13: Bootkey113,
    /// 0x013c
    /// Bits 239:224 of SHA-256 hash of boot key 1 (ECC)
    pub bootkey1_14: Bootkey114,
    /// 0x013e
    /// Bits 255:240 of SHA-256 hash of boot key 1 (ECC)
    pub bootkey1_15: Bootkey115,
    /// 0x0140
    /// Bits 15:0 of SHA-256 hash of boot key 2 (ECC)
    pub bootkey2_0: Bootkey20,
    /// 0x0142
    /// Bits 31:16 of SHA-256 hash of boot key 2 (ECC)
    pub bootkey2_1: Bootkey21,
    /// 0x0144
    /// Bits 47:32 of SHA-256 hash of boot key 2 (ECC)
    pub bootkey2_2: Bootkey22,
    /// 0x0146
    /// Bits 63:48 of SHA-256 hash of boot key 2 (ECC)
    pub bootkey2_3: Bootkey23,
    /// 0x0148
    /// Bits 79:64 of SHA-256 hash of boot key 2 (ECC)
    pub bootkey2_4: Bootkey24,
    /// 0x014a
    /// Bits 95:80 of SHA-256 hash of boot key 2 (ECC)
    pub bootkey2_5: Bootkey25,
    /// 0x014c
    /// Bits 111:96 of SHA-256 hash of boot key 2 (ECC)
    pub bootkey2_6: Bootkey26,
    /// 0x014e
    /// Bits 127:112 of SHA-256 hash of boot key 2 (ECC)
    pub bootkey2_7: Bootkey27,
    /// 0x0150
    /// Bits 143:128 of SHA-256 hash of boot key 2 (ECC)
    pub bootkey2_8: Bootkey28,
    /// 0x0152
    /// Bits 159:144 of SHA-256 hash of boot key 2 (ECC)
    pub bootkey2_9: Bootkey29,
    /// 0x0154
    /// Bits 175:160 of SHA-256 hash of boot key 2 (ECC)
    pub bootkey2_10: Bootkey210,
    /// 0x0156
    /// Bits 191:176 of SHA-256 hash of boot key 2 (ECC)
    pub bootkey2_11: Bootkey211,
    /// 0x0158
    /// Bits 207:192 of SHA-256 hash of boot key 2 (ECC)
    pub bootkey2_12: Bootkey212,
    /// 0x015a
    /// Bits 223:208 of SHA-256 hash of boot key 2 (ECC)
    pub bootkey2_13: Bootkey213,
    /// 0x015c
    /// Bits 239:224 of SHA-256 hash of boot key 2 (ECC)
    pub bootkey2_14: Bootkey214,
    /// 0x015e
    /// Bits 255:240 of SHA-256 hash of boot key 2 (ECC)
    pub bootkey2_15: Bootkey215,
    /// 0x0160
    /// Bits 15:0 of SHA-256 hash of boot key 3 (ECC)
    pub bootkey3_0: Bootkey30,
    /// 0x0162
    /// Bits 31:16 of SHA-256 hash of boot key 3 (ECC)
    pub bootkey3_1: Bootkey31,
    /// 0x0164
    /// Bits 47:32 of SHA-256 hash of boot key 3 (ECC)
    pub bootkey3_2: Bootkey32,
    /// 0x0166
    /// Bits 63:48 of SHA-256 hash of boot key 3 (ECC)
    pub bootkey3_3: Bootkey33,
    /// 0x0168
    /// Bits 79:64 of SHA-256 hash of boot key 3 (ECC)
    pub bootkey3_4: Bootkey34,
    /// 0x016a
    /// Bits 95:80 of SHA-256 hash of boot key 3 (ECC)
    pub bootkey3_5: Bootkey35,
    /// 0x016c
    /// Bits 111:96 of SHA-256 hash of boot key 3 (ECC)
    pub bootkey3_6: Bootkey36,
    /// 0x016e
    /// Bits 127:112 of SHA-256 hash of boot key 3 (ECC)
    pub bootkey3_7: Bootkey37,
    /// 0x0170
    /// Bits 143:128 of SHA-256 hash of boot key 3 (ECC)
    pub bootkey3_8: Bootkey38,
    /// 0x0172
    /// Bits 159:144 of SHA-256 hash of boot key 3 (ECC)
    pub bootkey3_9: Bootkey39,
    /// 0x0174
    /// Bits 175:160 of SHA-256 hash of boot key 3 (ECC)
    pub bootkey3_10: Bootkey310,
    /// 0x0176
    /// Bits 191:176 of SHA-256 hash of boot key 3 (ECC)
    pub bootkey3_11: Bootkey311,
    /// 0x0178
    /// Bits 207:192 of SHA-256 hash of boot key 3 (ECC)
    pub bootkey3_12: Bootkey312,
    /// 0x017a
    /// Bits 223:208 of SHA-256 hash of boot key 3 (ECC)
    pub bootkey3_13: Bootkey313,
    /// 0x017c
    /// Bits 239:224 of SHA-256 hash of boot key 3 (ECC)
    pub bootkey3_14: Bootkey314,
    /// 0x017e
    /// Bits 255:240 of SHA-256 hash of boot key 3 (ECC)
    pub bootkey3_15: Bootkey315,
    _reserved6: [u8; 7438],
    /// 0x1e90
    /// Bits 15:0 of OTP access key 1 (ECC)
    pub key1_0: Key10,
    /// 0x1e92
    /// Bits 31:16 of OTP access key 1 (ECC)
    pub key1_1: Key11,
    /// 0x1e94
    /// Bits 47:32 of OTP access key 1 (ECC)
    pub key1_2: Key12,
    /// 0x1e96
    /// Bits 63:48 of OTP access key 1 (ECC)
    pub key1_3: Key13,
    /// 0x1e98
    /// Bits 79:64 of OTP access key 1 (ECC)
    pub key1_4: Key14,
    /// 0x1e9a
    /// Bits 95:80 of OTP access key 1 (ECC)
    pub key1_5: Key15,
    /// 0x1e9c
    /// Bits 111:96 of OTP access key 1 (ECC)
    pub key1_6: Key16,
    /// 0x1e9e
    /// Bits 127:112 of OTP access key 1 (ECC)
    pub key1_7: Key17,
    /// 0x1ea0
    /// Bits 15:0 of OTP access key 2 (ECC)
    pub key2_0: Key20,
    /// 0x1ea2
    /// Bits 31:16 of OTP access key 2 (ECC)
    pub key2_1: Key21,
    /// 0x1ea4
    /// Bits 47:32 of OTP access key 2 (ECC)
    pub key2_2: Key22,
    /// 0x1ea6
    /// Bits 63:48 of OTP access key 2 (ECC)
    pub key2_3: Key23,
    /// 0x1ea8
    /// Bits 79:64 of OTP access key 2 (ECC)
    pub key2_4: Key24,
    /// 0x1eaa
    /// Bits 95:80 of OTP access key 2 (ECC)
    pub key2_5: Key25,
    /// 0x1eac
    /// Bits 111:96 of OTP access key 2 (ECC)
    pub key2_6: Key26,
    /// 0x1eae
    /// Bits 127:112 of OTP access key 2 (ECC)
    pub key2_7: Key27,
    /// 0x1eb0
    /// Bits 15:0 of OTP access key 3 (ECC)
    pub key3_0: Key30,
    /// 0x1eb2
    /// Bits 31:16 of OTP access key 3 (ECC)
    pub key3_1: Key31,
    /// 0x1eb4
    /// Bits 47:32 of OTP access key 3 (ECC)
    pub key3_2: Key32,
    /// 0x1eb6
    /// Bits 63:48 of OTP access key 3 (ECC)
    pub key3_3: Key33,
    /// 0x1eb8
    /// Bits 79:64 of OTP access key 3 (ECC)
    pub key3_4: Key34,
    /// 0x1eba
    /// Bits 95:80 of OTP access key 3 (ECC)
    pub key3_5: Key35,
    /// 0x1ebc
    /// Bits 111:96 of OTP access key 3 (ECC)
    pub key3_6: Key36,
    /// 0x1ebe
    /// Bits 127:112 of OTP access key 3 (ECC)
    pub key3_7: Key37,
    /// 0x1ec0
    /// Bits 15:0 of OTP access key 4 (ECC)
    pub key4_0: Key40,
    /// 0x1ec2
    /// Bits 31:16 of OTP access key 4 (ECC)
    pub key4_1: Key41,
    /// 0x1ec4
    /// Bits 47:32 of OTP access key 4 (ECC)
    pub key4_2: Key42,
    /// 0x1ec6
    /// Bits 63:48 of OTP access key 4 (ECC)
    pub key4_3: Key43,
    /// 0x1ec8
    /// Bits 79:64 of OTP access key 4 (ECC)
    pub key4_4: Key44,
    /// 0x1eca
    /// Bits 95:80 of OTP access key 4 (ECC)
    pub key4_5: Key45,
    /// 0x1ecc
    /// Bits 111:96 of OTP access key 4 (ECC)
    pub key4_6: Key46,
    /// 0x1ece
    /// Bits 127:112 of OTP access key 4 (ECC)
    pub key4_7: Key47,
    /// 0x1ed0
    /// Bits 15:0 of OTP access key 5 (ECC)
    pub key5_0: Key50,
    /// 0x1ed2
    /// Bits 31:16 of OTP access key 5 (ECC)
    pub key5_1: Key51,
    /// 0x1ed4
    /// Bits 47:32 of OTP access key 5 (ECC)
    pub key5_2: Key52,
    /// 0x1ed6
    /// Bits 63:48 of OTP access key 5 (ECC)
    pub key5_3: Key53,
    /// 0x1ed8
    /// Bits 79:64 of OTP access key 5 (ECC)
    pub key5_4: Key54,
    /// 0x1eda
    /// Bits 95:80 of OTP access key 5 (ECC)
    pub key5_5: Key55,
    /// 0x1edc
    /// Bits 111:96 of OTP access key 5 (ECC)
    pub key5_6: Key56,
    /// 0x1ede
    /// Bits 127:112 of OTP access key 5 (ECC)
    pub key5_7: Key57,
    /// 0x1ee0
    /// Bits 15:0 of OTP access key 6 (ECC)
    pub key6_0: Key60,
    /// 0x1ee2
    /// Bits 31:16 of OTP access key 6 (ECC)
    pub key6_1: Key61,
    /// 0x1ee4
    /// Bits 47:32 of OTP access key 6 (ECC)
    pub key6_2: Key62,
    /// 0x1ee6
    /// Bits 63:48 of OTP access key 6 (ECC)
    pub key6_3: Key63,
    /// 0x1ee8
    /// Bits 79:64 of OTP access key 6 (ECC)
    pub key6_4: Key64,
    /// 0x1eea
    /// Bits 95:80 of OTP access key 6 (ECC)
    pub key6_5: Key65,
    /// 0x1eec
    /// Bits 111:96 of OTP access key 6 (ECC)
    pub key6_6: Key66,
    /// 0x1eee
    /// Bits 127:112 of OTP access key 6 (ECC)
    pub key6_7: Key67,
}

