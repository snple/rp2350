//! USB
//! USB FS/LS controller device registers
//!
//! This peripheral supports atomic SET/CLR/XOR register aliases.

#![allow(dead_code)]

pub const BASE: u32 = 0x50110000;

/// ADDR_ENDP Register
///
/// Device address and endpoint control
///
/// Address offset: 0x0000
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **ENDPOINT** `[19:16]` (RW)
///
///   Device endpoint to send data to. Only valid for HOST mode.
///
/// - **ADDRESS** `[6:0]` (RW)
///
///   In device mode, the address that the device should respond to. Set in response to a SET_ADDR setup packet from the host. In host mode set to the address of the device to communicate with.
#[repr(transparent)]
pub struct AddrEndp {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for AddrEndp {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl AddrEndp {
    // ======== Field Constants ========

    /// **ENDPOINT**
    ///
    /// Device endpoint to send data to. Only valid for HOST mode.
    ///
    /// Bits `[19:16]`, mask `0x000f0000`
    pub const ENDPOINT_OFFSET: u32 = 16;
    pub const ENDPOINT_MASK: u32 = 0x000f0000;

    /// **ADDRESS**
    ///
    /// In device mode, the address that the device should respond to. Set in response to a SET_ADDR setup packet from the host. In host mode set to the address of the device to communicate with.
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const ADDRESS_OFFSET: u32 = 0;
    pub const ADDRESS_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **ENDPOINT** `[19:16]` (RW)
    ///
    /// Device endpoint to send data to. Only valid for HOST mode.
    #[inline(always)]
    pub fn endpoint(&self) -> u32 {
        (self.value.read_bits() & Self::ENDPOINT_MASK) >> Self::ENDPOINT_OFFSET
    }

    /// Write **ENDPOINT** `[19:16]` (RW)
    ///
    /// Device endpoint to send data to. Only valid for HOST mode.
    #[inline(always)]
    pub fn write_endpoint(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ENDPOINT_MASK) | ((value << Self::ENDPOINT_OFFSET) & Self::ENDPOINT_MASK)
        });
    }

    /// **ADDRESS** `[6:0]` (RW)
    ///
    /// In device mode, the address that the device should respond to. Set in response to a SET_ADDR setup packet from the host. In host mode set to the address of the device to communicate with.
    #[inline(always)]
    pub fn address(&self) -> u32 {
        (self.value.read_bits() & Self::ADDRESS_MASK) >> Self::ADDRESS_OFFSET
    }

    /// Write **ADDRESS** `[6:0]` (RW)
    ///
    /// In device mode, the address that the device should respond to. Set in response to a SET_ADDR setup packet from the host. In host mode set to the address of the device to communicate with.
    #[inline(always)]
    pub fn write_address(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ADDRESS_MASK) | ((value << Self::ADDRESS_OFFSET) & Self::ADDRESS_MASK)
        });
    }
}

/// ADDR_ENDP value type for batch read/modify/write operations
///
/// Use `addrendp.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `addrendp.write_val(val)`.
#[derive(Clone, Copy)]
pub struct AddrEndpVal {
    bits: u32,
}

impl AddrEndpVal {
    /// Create from raw bits
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

    /// Read **ENDPOINT** `[19:16]`
    ///
    /// Device endpoint to send data to. Only valid for HOST mode.
    #[inline(always)]
    pub const fn endpoint(&self) -> u32 {
        (self.bits & AddrEndp::ENDPOINT_MASK) >> AddrEndp::ENDPOINT_OFFSET
    }

    /// Read **ADDRESS** `[6:0]`
    ///
    /// In device mode, the address that the device should respond to. Set in response to a SET_ADDR setup packet from the host. In host mode set to the address of the device to communicate with.
    #[inline(always)]
    pub const fn address(&self) -> u32 {
        (self.bits & AddrEndp::ADDRESS_MASK) >> AddrEndp::ADDRESS_OFFSET
    }

    /// Set **ENDPOINT** `[19:16]`
    ///
    /// Device endpoint to send data to. Only valid for HOST mode.
    #[inline(always)]
    pub const fn with_endpoint(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AddrEndp::ENDPOINT_MASK)
                | ((value << AddrEndp::ENDPOINT_OFFSET) & AddrEndp::ENDPOINT_MASK),
        }
    }

    /// Set **ADDRESS** `[6:0]`
    ///
    /// In device mode, the address that the device should respond to. Set in response to a SET_ADDR setup packet from the host. In host mode set to the address of the device to communicate with.
    #[inline(always)]
    pub const fn with_address(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AddrEndp::ADDRESS_MASK)
                | ((value << AddrEndp::ADDRESS_OFFSET) & AddrEndp::ADDRESS_MASK),
        }
    }
}

impl Default for AddrEndpVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl AddrEndp {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> AddrEndpVal {
        AddrEndpVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: AddrEndpVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(AddrEndpVal) -> AddrEndpVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ADDR_ENDP1 Register
///
/// Interrupt endpoint 1. Only valid for HOST mode.
///
/// Address offset: 0x0004
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **INTEP_PREAMBLE** `[26]` (RW)
///
///   Interrupt EP requires preamble (is a low speed device on a full speed hub)
///
/// - **INTEP_DIR** `[25]` (RW)
///
///   Direction of the interrupt endpoint. In=0, Out=1
///
/// - **ENDPOINT** `[19:16]` (RW)
///
///   Endpoint number of the interrupt endpoint
///
/// - **ADDRESS** `[6:0]` (RW)
///
///   Device address
#[repr(transparent)]
pub struct AddrEndp1 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for AddrEndp1 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl AddrEndp1 {
    // ======== Field Constants ========

    /// **INTEP_PREAMBLE**
    ///
    /// Interrupt EP requires preamble (is a low speed device on a full speed hub)
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const INTEP_PREAMBLE_OFFSET: u32 = 26;
    pub const INTEP_PREAMBLE_MASK: u32 = 0x04000000;
    pub const INTEP_PREAMBLE: u32 = 1 << 26;

    /// **INTEP_DIR**
    ///
    /// Direction of the interrupt endpoint. In=0, Out=1
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const INTEP_DIR_OFFSET: u32 = 25;
    pub const INTEP_DIR_MASK: u32 = 0x02000000;
    pub const INTEP_DIR: u32 = 1 << 25;

    /// **ENDPOINT**
    ///
    /// Endpoint number of the interrupt endpoint
    ///
    /// Bits `[19:16]`, mask `0x000f0000`
    pub const ENDPOINT_OFFSET: u32 = 16;
    pub const ENDPOINT_MASK: u32 = 0x000f0000;

    /// **ADDRESS**
    ///
    /// Device address
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const ADDRESS_OFFSET: u32 = 0;
    pub const ADDRESS_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **INTEP_PREAMBLE** `[26]` (RW)
    ///
    /// Interrupt EP requires preamble (is a low speed device on a full speed hub)
    #[inline(always)]
    pub fn intep_preamble(&self) -> bool {
        (self.value.read_bits() & Self::INTEP_PREAMBLE_MASK) != 0
    }

    /// Set INTEP_PREAMBLE bit atomically (RW)
    #[inline(always)]
    pub fn set_intep_preamble(&self) {
        self.set_bits(Self::INTEP_PREAMBLE_MASK);
    }

    /// Clear INTEP_PREAMBLE bit atomically (RW)
    #[inline(always)]
    pub fn clr_intep_preamble(&self) {
        self.clr_bits(Self::INTEP_PREAMBLE_MASK);
    }

    /// **INTEP_DIR** `[25]` (RW)
    ///
    /// Direction of the interrupt endpoint. In=0, Out=1
    #[inline(always)]
    pub fn intep_dir(&self) -> bool {
        (self.value.read_bits() & Self::INTEP_DIR_MASK) != 0
    }

    /// Set INTEP_DIR bit atomically (RW)
    #[inline(always)]
    pub fn set_intep_dir(&self) {
        self.set_bits(Self::INTEP_DIR_MASK);
    }

    /// Clear INTEP_DIR bit atomically (RW)
    #[inline(always)]
    pub fn clr_intep_dir(&self) {
        self.clr_bits(Self::INTEP_DIR_MASK);
    }

    /// **ENDPOINT** `[19:16]` (RW)
    ///
    /// Endpoint number of the interrupt endpoint
    #[inline(always)]
    pub fn endpoint(&self) -> u32 {
        (self.value.read_bits() & Self::ENDPOINT_MASK) >> Self::ENDPOINT_OFFSET
    }

    /// Write **ENDPOINT** `[19:16]` (RW)
    ///
    /// Endpoint number of the interrupt endpoint
    #[inline(always)]
    pub fn write_endpoint(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ENDPOINT_MASK) | ((value << Self::ENDPOINT_OFFSET) & Self::ENDPOINT_MASK)
        });
    }

    /// **ADDRESS** `[6:0]` (RW)
    ///
    /// Device address
    #[inline(always)]
    pub fn address(&self) -> u32 {
        (self.value.read_bits() & Self::ADDRESS_MASK) >> Self::ADDRESS_OFFSET
    }

    /// Write **ADDRESS** `[6:0]` (RW)
    ///
    /// Device address
    #[inline(always)]
    pub fn write_address(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ADDRESS_MASK) | ((value << Self::ADDRESS_OFFSET) & Self::ADDRESS_MASK)
        });
    }
}

/// ADDR_ENDP1 value type for batch read/modify/write operations
///
/// Use `addrendp1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `addrendp1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct AddrEndp1Val {
    bits: u32,
}

impl AddrEndp1Val {
    /// Create from raw bits
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

    /// Read **INTEP_PREAMBLE** `[26:26]`
    ///
    /// Interrupt EP requires preamble (is a low speed device on a full speed hub)
    #[inline(always)]
    pub const fn intep_preamble(&self) -> bool {
        (self.bits & AddrEndp1::INTEP_PREAMBLE_MASK) != 0
    }

    /// Read **INTEP_DIR** `[25:25]`
    ///
    /// Direction of the interrupt endpoint. In=0, Out=1
    #[inline(always)]
    pub const fn intep_dir(&self) -> bool {
        (self.bits & AddrEndp1::INTEP_DIR_MASK) != 0
    }

    /// Read **ENDPOINT** `[19:16]`
    ///
    /// Endpoint number of the interrupt endpoint
    #[inline(always)]
    pub const fn endpoint(&self) -> u32 {
        (self.bits & AddrEndp1::ENDPOINT_MASK) >> AddrEndp1::ENDPOINT_OFFSET
    }

    /// Read **ADDRESS** `[6:0]`
    ///
    /// Device address
    #[inline(always)]
    pub const fn address(&self) -> u32 {
        (self.bits & AddrEndp1::ADDRESS_MASK) >> AddrEndp1::ADDRESS_OFFSET
    }

    /// Set **INTEP_PREAMBLE** `[26:26]`
    ///
    /// Interrupt EP requires preamble (is a low speed device on a full speed hub)
    #[inline(always)]
    pub const fn with_intep_preamble(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | AddrEndp1::INTEP_PREAMBLE_MASK
            } else {
                self.bits & !AddrEndp1::INTEP_PREAMBLE_MASK
            },
        }
    }

    /// Set **INTEP_DIR** `[25:25]`
    ///
    /// Direction of the interrupt endpoint. In=0, Out=1
    #[inline(always)]
    pub const fn with_intep_dir(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | AddrEndp1::INTEP_DIR_MASK
            } else {
                self.bits & !AddrEndp1::INTEP_DIR_MASK
            },
        }
    }

    /// Set **ENDPOINT** `[19:16]`
    ///
    /// Endpoint number of the interrupt endpoint
    #[inline(always)]
    pub const fn with_endpoint(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AddrEndp1::ENDPOINT_MASK)
                | ((value << AddrEndp1::ENDPOINT_OFFSET) & AddrEndp1::ENDPOINT_MASK),
        }
    }

    /// Set **ADDRESS** `[6:0]`
    ///
    /// Device address
    #[inline(always)]
    pub const fn with_address(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AddrEndp1::ADDRESS_MASK)
                | ((value << AddrEndp1::ADDRESS_OFFSET) & AddrEndp1::ADDRESS_MASK),
        }
    }
}

impl Default for AddrEndp1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl AddrEndp1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> AddrEndp1Val {
        AddrEndp1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: AddrEndp1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(AddrEndp1Val) -> AddrEndp1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ADDR_ENDP2 Register
///
/// Interrupt endpoint 2. Only valid for HOST mode.
///
/// Address offset: 0x0008
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **INTEP_PREAMBLE** `[26]` (RW)
///
///   Interrupt EP requires preamble (is a low speed device on a full speed hub)
///
/// - **INTEP_DIR** `[25]` (RW)
///
///   Direction of the interrupt endpoint. In=0, Out=1
///
/// - **ENDPOINT** `[19:16]` (RW)
///
///   Endpoint number of the interrupt endpoint
///
/// - **ADDRESS** `[6:0]` (RW)
///
///   Device address
#[repr(transparent)]
pub struct AddrEndp2 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for AddrEndp2 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl AddrEndp2 {
    // ======== Field Constants ========

    /// **INTEP_PREAMBLE**
    ///
    /// Interrupt EP requires preamble (is a low speed device on a full speed hub)
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const INTEP_PREAMBLE_OFFSET: u32 = 26;
    pub const INTEP_PREAMBLE_MASK: u32 = 0x04000000;
    pub const INTEP_PREAMBLE: u32 = 1 << 26;

    /// **INTEP_DIR**
    ///
    /// Direction of the interrupt endpoint. In=0, Out=1
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const INTEP_DIR_OFFSET: u32 = 25;
    pub const INTEP_DIR_MASK: u32 = 0x02000000;
    pub const INTEP_DIR: u32 = 1 << 25;

    /// **ENDPOINT**
    ///
    /// Endpoint number of the interrupt endpoint
    ///
    /// Bits `[19:16]`, mask `0x000f0000`
    pub const ENDPOINT_OFFSET: u32 = 16;
    pub const ENDPOINT_MASK: u32 = 0x000f0000;

    /// **ADDRESS**
    ///
    /// Device address
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const ADDRESS_OFFSET: u32 = 0;
    pub const ADDRESS_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **INTEP_PREAMBLE** `[26]` (RW)
    ///
    /// Interrupt EP requires preamble (is a low speed device on a full speed hub)
    #[inline(always)]
    pub fn intep_preamble(&self) -> bool {
        (self.value.read_bits() & Self::INTEP_PREAMBLE_MASK) != 0
    }

    /// Set INTEP_PREAMBLE bit atomically (RW)
    #[inline(always)]
    pub fn set_intep_preamble(&self) {
        self.set_bits(Self::INTEP_PREAMBLE_MASK);
    }

    /// Clear INTEP_PREAMBLE bit atomically (RW)
    #[inline(always)]
    pub fn clr_intep_preamble(&self) {
        self.clr_bits(Self::INTEP_PREAMBLE_MASK);
    }

    /// **INTEP_DIR** `[25]` (RW)
    ///
    /// Direction of the interrupt endpoint. In=0, Out=1
    #[inline(always)]
    pub fn intep_dir(&self) -> bool {
        (self.value.read_bits() & Self::INTEP_DIR_MASK) != 0
    }

    /// Set INTEP_DIR bit atomically (RW)
    #[inline(always)]
    pub fn set_intep_dir(&self) {
        self.set_bits(Self::INTEP_DIR_MASK);
    }

    /// Clear INTEP_DIR bit atomically (RW)
    #[inline(always)]
    pub fn clr_intep_dir(&self) {
        self.clr_bits(Self::INTEP_DIR_MASK);
    }

    /// **ENDPOINT** `[19:16]` (RW)
    ///
    /// Endpoint number of the interrupt endpoint
    #[inline(always)]
    pub fn endpoint(&self) -> u32 {
        (self.value.read_bits() & Self::ENDPOINT_MASK) >> Self::ENDPOINT_OFFSET
    }

    /// Write **ENDPOINT** `[19:16]` (RW)
    ///
    /// Endpoint number of the interrupt endpoint
    #[inline(always)]
    pub fn write_endpoint(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ENDPOINT_MASK) | ((value << Self::ENDPOINT_OFFSET) & Self::ENDPOINT_MASK)
        });
    }

    /// **ADDRESS** `[6:0]` (RW)
    ///
    /// Device address
    #[inline(always)]
    pub fn address(&self) -> u32 {
        (self.value.read_bits() & Self::ADDRESS_MASK) >> Self::ADDRESS_OFFSET
    }

    /// Write **ADDRESS** `[6:0]` (RW)
    ///
    /// Device address
    #[inline(always)]
    pub fn write_address(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ADDRESS_MASK) | ((value << Self::ADDRESS_OFFSET) & Self::ADDRESS_MASK)
        });
    }
}

/// ADDR_ENDP2 value type for batch read/modify/write operations
///
/// Use `addrendp2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `addrendp2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct AddrEndp2Val {
    bits: u32,
}

impl AddrEndp2Val {
    /// Create from raw bits
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

    /// Read **INTEP_PREAMBLE** `[26:26]`
    ///
    /// Interrupt EP requires preamble (is a low speed device on a full speed hub)
    #[inline(always)]
    pub const fn intep_preamble(&self) -> bool {
        (self.bits & AddrEndp2::INTEP_PREAMBLE_MASK) != 0
    }

    /// Read **INTEP_DIR** `[25:25]`
    ///
    /// Direction of the interrupt endpoint. In=0, Out=1
    #[inline(always)]
    pub const fn intep_dir(&self) -> bool {
        (self.bits & AddrEndp2::INTEP_DIR_MASK) != 0
    }

    /// Read **ENDPOINT** `[19:16]`
    ///
    /// Endpoint number of the interrupt endpoint
    #[inline(always)]
    pub const fn endpoint(&self) -> u32 {
        (self.bits & AddrEndp2::ENDPOINT_MASK) >> AddrEndp2::ENDPOINT_OFFSET
    }

    /// Read **ADDRESS** `[6:0]`
    ///
    /// Device address
    #[inline(always)]
    pub const fn address(&self) -> u32 {
        (self.bits & AddrEndp2::ADDRESS_MASK) >> AddrEndp2::ADDRESS_OFFSET
    }

    /// Set **INTEP_PREAMBLE** `[26:26]`
    ///
    /// Interrupt EP requires preamble (is a low speed device on a full speed hub)
    #[inline(always)]
    pub const fn with_intep_preamble(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | AddrEndp2::INTEP_PREAMBLE_MASK
            } else {
                self.bits & !AddrEndp2::INTEP_PREAMBLE_MASK
            },
        }
    }

    /// Set **INTEP_DIR** `[25:25]`
    ///
    /// Direction of the interrupt endpoint. In=0, Out=1
    #[inline(always)]
    pub const fn with_intep_dir(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | AddrEndp2::INTEP_DIR_MASK
            } else {
                self.bits & !AddrEndp2::INTEP_DIR_MASK
            },
        }
    }

    /// Set **ENDPOINT** `[19:16]`
    ///
    /// Endpoint number of the interrupt endpoint
    #[inline(always)]
    pub const fn with_endpoint(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AddrEndp2::ENDPOINT_MASK)
                | ((value << AddrEndp2::ENDPOINT_OFFSET) & AddrEndp2::ENDPOINT_MASK),
        }
    }

    /// Set **ADDRESS** `[6:0]`
    ///
    /// Device address
    #[inline(always)]
    pub const fn with_address(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AddrEndp2::ADDRESS_MASK)
                | ((value << AddrEndp2::ADDRESS_OFFSET) & AddrEndp2::ADDRESS_MASK),
        }
    }
}

impl Default for AddrEndp2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl AddrEndp2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> AddrEndp2Val {
        AddrEndp2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: AddrEndp2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(AddrEndp2Val) -> AddrEndp2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ADDR_ENDP3 Register
///
/// Interrupt endpoint 3. Only valid for HOST mode.
///
/// Address offset: 0x000c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **INTEP_PREAMBLE** `[26]` (RW)
///
///   Interrupt EP requires preamble (is a low speed device on a full speed hub)
///
/// - **INTEP_DIR** `[25]` (RW)
///
///   Direction of the interrupt endpoint. In=0, Out=1
///
/// - **ENDPOINT** `[19:16]` (RW)
///
///   Endpoint number of the interrupt endpoint
///
/// - **ADDRESS** `[6:0]` (RW)
///
///   Device address
#[repr(transparent)]
pub struct AddrEndp3 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for AddrEndp3 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl AddrEndp3 {
    // ======== Field Constants ========

    /// **INTEP_PREAMBLE**
    ///
    /// Interrupt EP requires preamble (is a low speed device on a full speed hub)
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const INTEP_PREAMBLE_OFFSET: u32 = 26;
    pub const INTEP_PREAMBLE_MASK: u32 = 0x04000000;
    pub const INTEP_PREAMBLE: u32 = 1 << 26;

    /// **INTEP_DIR**
    ///
    /// Direction of the interrupt endpoint. In=0, Out=1
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const INTEP_DIR_OFFSET: u32 = 25;
    pub const INTEP_DIR_MASK: u32 = 0x02000000;
    pub const INTEP_DIR: u32 = 1 << 25;

    /// **ENDPOINT**
    ///
    /// Endpoint number of the interrupt endpoint
    ///
    /// Bits `[19:16]`, mask `0x000f0000`
    pub const ENDPOINT_OFFSET: u32 = 16;
    pub const ENDPOINT_MASK: u32 = 0x000f0000;

    /// **ADDRESS**
    ///
    /// Device address
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const ADDRESS_OFFSET: u32 = 0;
    pub const ADDRESS_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **INTEP_PREAMBLE** `[26]` (RW)
    ///
    /// Interrupt EP requires preamble (is a low speed device on a full speed hub)
    #[inline(always)]
    pub fn intep_preamble(&self) -> bool {
        (self.value.read_bits() & Self::INTEP_PREAMBLE_MASK) != 0
    }

    /// Set INTEP_PREAMBLE bit atomically (RW)
    #[inline(always)]
    pub fn set_intep_preamble(&self) {
        self.set_bits(Self::INTEP_PREAMBLE_MASK);
    }

    /// Clear INTEP_PREAMBLE bit atomically (RW)
    #[inline(always)]
    pub fn clr_intep_preamble(&self) {
        self.clr_bits(Self::INTEP_PREAMBLE_MASK);
    }

    /// **INTEP_DIR** `[25]` (RW)
    ///
    /// Direction of the interrupt endpoint. In=0, Out=1
    #[inline(always)]
    pub fn intep_dir(&self) -> bool {
        (self.value.read_bits() & Self::INTEP_DIR_MASK) != 0
    }

    /// Set INTEP_DIR bit atomically (RW)
    #[inline(always)]
    pub fn set_intep_dir(&self) {
        self.set_bits(Self::INTEP_DIR_MASK);
    }

    /// Clear INTEP_DIR bit atomically (RW)
    #[inline(always)]
    pub fn clr_intep_dir(&self) {
        self.clr_bits(Self::INTEP_DIR_MASK);
    }

    /// **ENDPOINT** `[19:16]` (RW)
    ///
    /// Endpoint number of the interrupt endpoint
    #[inline(always)]
    pub fn endpoint(&self) -> u32 {
        (self.value.read_bits() & Self::ENDPOINT_MASK) >> Self::ENDPOINT_OFFSET
    }

    /// Write **ENDPOINT** `[19:16]` (RW)
    ///
    /// Endpoint number of the interrupt endpoint
    #[inline(always)]
    pub fn write_endpoint(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ENDPOINT_MASK) | ((value << Self::ENDPOINT_OFFSET) & Self::ENDPOINT_MASK)
        });
    }

    /// **ADDRESS** `[6:0]` (RW)
    ///
    /// Device address
    #[inline(always)]
    pub fn address(&self) -> u32 {
        (self.value.read_bits() & Self::ADDRESS_MASK) >> Self::ADDRESS_OFFSET
    }

    /// Write **ADDRESS** `[6:0]` (RW)
    ///
    /// Device address
    #[inline(always)]
    pub fn write_address(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ADDRESS_MASK) | ((value << Self::ADDRESS_OFFSET) & Self::ADDRESS_MASK)
        });
    }
}

/// ADDR_ENDP3 value type for batch read/modify/write operations
///
/// Use `addrendp3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `addrendp3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct AddrEndp3Val {
    bits: u32,
}

impl AddrEndp3Val {
    /// Create from raw bits
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

    /// Read **INTEP_PREAMBLE** `[26:26]`
    ///
    /// Interrupt EP requires preamble (is a low speed device on a full speed hub)
    #[inline(always)]
    pub const fn intep_preamble(&self) -> bool {
        (self.bits & AddrEndp3::INTEP_PREAMBLE_MASK) != 0
    }

    /// Read **INTEP_DIR** `[25:25]`
    ///
    /// Direction of the interrupt endpoint. In=0, Out=1
    #[inline(always)]
    pub const fn intep_dir(&self) -> bool {
        (self.bits & AddrEndp3::INTEP_DIR_MASK) != 0
    }

    /// Read **ENDPOINT** `[19:16]`
    ///
    /// Endpoint number of the interrupt endpoint
    #[inline(always)]
    pub const fn endpoint(&self) -> u32 {
        (self.bits & AddrEndp3::ENDPOINT_MASK) >> AddrEndp3::ENDPOINT_OFFSET
    }

    /// Read **ADDRESS** `[6:0]`
    ///
    /// Device address
    #[inline(always)]
    pub const fn address(&self) -> u32 {
        (self.bits & AddrEndp3::ADDRESS_MASK) >> AddrEndp3::ADDRESS_OFFSET
    }

    /// Set **INTEP_PREAMBLE** `[26:26]`
    ///
    /// Interrupt EP requires preamble (is a low speed device on a full speed hub)
    #[inline(always)]
    pub const fn with_intep_preamble(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | AddrEndp3::INTEP_PREAMBLE_MASK
            } else {
                self.bits & !AddrEndp3::INTEP_PREAMBLE_MASK
            },
        }
    }

    /// Set **INTEP_DIR** `[25:25]`
    ///
    /// Direction of the interrupt endpoint. In=0, Out=1
    #[inline(always)]
    pub const fn with_intep_dir(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | AddrEndp3::INTEP_DIR_MASK
            } else {
                self.bits & !AddrEndp3::INTEP_DIR_MASK
            },
        }
    }

    /// Set **ENDPOINT** `[19:16]`
    ///
    /// Endpoint number of the interrupt endpoint
    #[inline(always)]
    pub const fn with_endpoint(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AddrEndp3::ENDPOINT_MASK)
                | ((value << AddrEndp3::ENDPOINT_OFFSET) & AddrEndp3::ENDPOINT_MASK),
        }
    }

    /// Set **ADDRESS** `[6:0]`
    ///
    /// Device address
    #[inline(always)]
    pub const fn with_address(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AddrEndp3::ADDRESS_MASK)
                | ((value << AddrEndp3::ADDRESS_OFFSET) & AddrEndp3::ADDRESS_MASK),
        }
    }
}

impl Default for AddrEndp3Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl AddrEndp3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> AddrEndp3Val {
        AddrEndp3Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: AddrEndp3Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(AddrEndp3Val) -> AddrEndp3Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ADDR_ENDP4 Register
///
/// Interrupt endpoint 4. Only valid for HOST mode.
///
/// Address offset: 0x0010
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **INTEP_PREAMBLE** `[26]` (RW)
///
///   Interrupt EP requires preamble (is a low speed device on a full speed hub)
///
/// - **INTEP_DIR** `[25]` (RW)
///
///   Direction of the interrupt endpoint. In=0, Out=1
///
/// - **ENDPOINT** `[19:16]` (RW)
///
///   Endpoint number of the interrupt endpoint
///
/// - **ADDRESS** `[6:0]` (RW)
///
///   Device address
#[repr(transparent)]
pub struct AddrEndp4 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for AddrEndp4 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl AddrEndp4 {
    // ======== Field Constants ========

    /// **INTEP_PREAMBLE**
    ///
    /// Interrupt EP requires preamble (is a low speed device on a full speed hub)
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const INTEP_PREAMBLE_OFFSET: u32 = 26;
    pub const INTEP_PREAMBLE_MASK: u32 = 0x04000000;
    pub const INTEP_PREAMBLE: u32 = 1 << 26;

    /// **INTEP_DIR**
    ///
    /// Direction of the interrupt endpoint. In=0, Out=1
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const INTEP_DIR_OFFSET: u32 = 25;
    pub const INTEP_DIR_MASK: u32 = 0x02000000;
    pub const INTEP_DIR: u32 = 1 << 25;

    /// **ENDPOINT**
    ///
    /// Endpoint number of the interrupt endpoint
    ///
    /// Bits `[19:16]`, mask `0x000f0000`
    pub const ENDPOINT_OFFSET: u32 = 16;
    pub const ENDPOINT_MASK: u32 = 0x000f0000;

    /// **ADDRESS**
    ///
    /// Device address
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const ADDRESS_OFFSET: u32 = 0;
    pub const ADDRESS_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **INTEP_PREAMBLE** `[26]` (RW)
    ///
    /// Interrupt EP requires preamble (is a low speed device on a full speed hub)
    #[inline(always)]
    pub fn intep_preamble(&self) -> bool {
        (self.value.read_bits() & Self::INTEP_PREAMBLE_MASK) != 0
    }

    /// Set INTEP_PREAMBLE bit atomically (RW)
    #[inline(always)]
    pub fn set_intep_preamble(&self) {
        self.set_bits(Self::INTEP_PREAMBLE_MASK);
    }

    /// Clear INTEP_PREAMBLE bit atomically (RW)
    #[inline(always)]
    pub fn clr_intep_preamble(&self) {
        self.clr_bits(Self::INTEP_PREAMBLE_MASK);
    }

    /// **INTEP_DIR** `[25]` (RW)
    ///
    /// Direction of the interrupt endpoint. In=0, Out=1
    #[inline(always)]
    pub fn intep_dir(&self) -> bool {
        (self.value.read_bits() & Self::INTEP_DIR_MASK) != 0
    }

    /// Set INTEP_DIR bit atomically (RW)
    #[inline(always)]
    pub fn set_intep_dir(&self) {
        self.set_bits(Self::INTEP_DIR_MASK);
    }

    /// Clear INTEP_DIR bit atomically (RW)
    #[inline(always)]
    pub fn clr_intep_dir(&self) {
        self.clr_bits(Self::INTEP_DIR_MASK);
    }

    /// **ENDPOINT** `[19:16]` (RW)
    ///
    /// Endpoint number of the interrupt endpoint
    #[inline(always)]
    pub fn endpoint(&self) -> u32 {
        (self.value.read_bits() & Self::ENDPOINT_MASK) >> Self::ENDPOINT_OFFSET
    }

    /// Write **ENDPOINT** `[19:16]` (RW)
    ///
    /// Endpoint number of the interrupt endpoint
    #[inline(always)]
    pub fn write_endpoint(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ENDPOINT_MASK) | ((value << Self::ENDPOINT_OFFSET) & Self::ENDPOINT_MASK)
        });
    }

    /// **ADDRESS** `[6:0]` (RW)
    ///
    /// Device address
    #[inline(always)]
    pub fn address(&self) -> u32 {
        (self.value.read_bits() & Self::ADDRESS_MASK) >> Self::ADDRESS_OFFSET
    }

    /// Write **ADDRESS** `[6:0]` (RW)
    ///
    /// Device address
    #[inline(always)]
    pub fn write_address(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ADDRESS_MASK) | ((value << Self::ADDRESS_OFFSET) & Self::ADDRESS_MASK)
        });
    }
}

/// ADDR_ENDP4 value type for batch read/modify/write operations
///
/// Use `addrendp4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `addrendp4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct AddrEndp4Val {
    bits: u32,
}

impl AddrEndp4Val {
    /// Create from raw bits
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

    /// Read **INTEP_PREAMBLE** `[26:26]`
    ///
    /// Interrupt EP requires preamble (is a low speed device on a full speed hub)
    #[inline(always)]
    pub const fn intep_preamble(&self) -> bool {
        (self.bits & AddrEndp4::INTEP_PREAMBLE_MASK) != 0
    }

    /// Read **INTEP_DIR** `[25:25]`
    ///
    /// Direction of the interrupt endpoint. In=0, Out=1
    #[inline(always)]
    pub const fn intep_dir(&self) -> bool {
        (self.bits & AddrEndp4::INTEP_DIR_MASK) != 0
    }

    /// Read **ENDPOINT** `[19:16]`
    ///
    /// Endpoint number of the interrupt endpoint
    #[inline(always)]
    pub const fn endpoint(&self) -> u32 {
        (self.bits & AddrEndp4::ENDPOINT_MASK) >> AddrEndp4::ENDPOINT_OFFSET
    }

    /// Read **ADDRESS** `[6:0]`
    ///
    /// Device address
    #[inline(always)]
    pub const fn address(&self) -> u32 {
        (self.bits & AddrEndp4::ADDRESS_MASK) >> AddrEndp4::ADDRESS_OFFSET
    }

    /// Set **INTEP_PREAMBLE** `[26:26]`
    ///
    /// Interrupt EP requires preamble (is a low speed device on a full speed hub)
    #[inline(always)]
    pub const fn with_intep_preamble(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | AddrEndp4::INTEP_PREAMBLE_MASK
            } else {
                self.bits & !AddrEndp4::INTEP_PREAMBLE_MASK
            },
        }
    }

    /// Set **INTEP_DIR** `[25:25]`
    ///
    /// Direction of the interrupt endpoint. In=0, Out=1
    #[inline(always)]
    pub const fn with_intep_dir(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | AddrEndp4::INTEP_DIR_MASK
            } else {
                self.bits & !AddrEndp4::INTEP_DIR_MASK
            },
        }
    }

    /// Set **ENDPOINT** `[19:16]`
    ///
    /// Endpoint number of the interrupt endpoint
    #[inline(always)]
    pub const fn with_endpoint(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AddrEndp4::ENDPOINT_MASK)
                | ((value << AddrEndp4::ENDPOINT_OFFSET) & AddrEndp4::ENDPOINT_MASK),
        }
    }

    /// Set **ADDRESS** `[6:0]`
    ///
    /// Device address
    #[inline(always)]
    pub const fn with_address(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AddrEndp4::ADDRESS_MASK)
                | ((value << AddrEndp4::ADDRESS_OFFSET) & AddrEndp4::ADDRESS_MASK),
        }
    }
}

impl Default for AddrEndp4Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl AddrEndp4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> AddrEndp4Val {
        AddrEndp4Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: AddrEndp4Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(AddrEndp4Val) -> AddrEndp4Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ADDR_ENDP5 Register
///
/// Interrupt endpoint 5. Only valid for HOST mode.
///
/// Address offset: 0x0014
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **INTEP_PREAMBLE** `[26]` (RW)
///
///   Interrupt EP requires preamble (is a low speed device on a full speed hub)
///
/// - **INTEP_DIR** `[25]` (RW)
///
///   Direction of the interrupt endpoint. In=0, Out=1
///
/// - **ENDPOINT** `[19:16]` (RW)
///
///   Endpoint number of the interrupt endpoint
///
/// - **ADDRESS** `[6:0]` (RW)
///
///   Device address
#[repr(transparent)]
pub struct AddrEndp5 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for AddrEndp5 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl AddrEndp5 {
    // ======== Field Constants ========

    /// **INTEP_PREAMBLE**
    ///
    /// Interrupt EP requires preamble (is a low speed device on a full speed hub)
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const INTEP_PREAMBLE_OFFSET: u32 = 26;
    pub const INTEP_PREAMBLE_MASK: u32 = 0x04000000;
    pub const INTEP_PREAMBLE: u32 = 1 << 26;

    /// **INTEP_DIR**
    ///
    /// Direction of the interrupt endpoint. In=0, Out=1
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const INTEP_DIR_OFFSET: u32 = 25;
    pub const INTEP_DIR_MASK: u32 = 0x02000000;
    pub const INTEP_DIR: u32 = 1 << 25;

    /// **ENDPOINT**
    ///
    /// Endpoint number of the interrupt endpoint
    ///
    /// Bits `[19:16]`, mask `0x000f0000`
    pub const ENDPOINT_OFFSET: u32 = 16;
    pub const ENDPOINT_MASK: u32 = 0x000f0000;

    /// **ADDRESS**
    ///
    /// Device address
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const ADDRESS_OFFSET: u32 = 0;
    pub const ADDRESS_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **INTEP_PREAMBLE** `[26]` (RW)
    ///
    /// Interrupt EP requires preamble (is a low speed device on a full speed hub)
    #[inline(always)]
    pub fn intep_preamble(&self) -> bool {
        (self.value.read_bits() & Self::INTEP_PREAMBLE_MASK) != 0
    }

    /// Set INTEP_PREAMBLE bit atomically (RW)
    #[inline(always)]
    pub fn set_intep_preamble(&self) {
        self.set_bits(Self::INTEP_PREAMBLE_MASK);
    }

    /// Clear INTEP_PREAMBLE bit atomically (RW)
    #[inline(always)]
    pub fn clr_intep_preamble(&self) {
        self.clr_bits(Self::INTEP_PREAMBLE_MASK);
    }

    /// **INTEP_DIR** `[25]` (RW)
    ///
    /// Direction of the interrupt endpoint. In=0, Out=1
    #[inline(always)]
    pub fn intep_dir(&self) -> bool {
        (self.value.read_bits() & Self::INTEP_DIR_MASK) != 0
    }

    /// Set INTEP_DIR bit atomically (RW)
    #[inline(always)]
    pub fn set_intep_dir(&self) {
        self.set_bits(Self::INTEP_DIR_MASK);
    }

    /// Clear INTEP_DIR bit atomically (RW)
    #[inline(always)]
    pub fn clr_intep_dir(&self) {
        self.clr_bits(Self::INTEP_DIR_MASK);
    }

    /// **ENDPOINT** `[19:16]` (RW)
    ///
    /// Endpoint number of the interrupt endpoint
    #[inline(always)]
    pub fn endpoint(&self) -> u32 {
        (self.value.read_bits() & Self::ENDPOINT_MASK) >> Self::ENDPOINT_OFFSET
    }

    /// Write **ENDPOINT** `[19:16]` (RW)
    ///
    /// Endpoint number of the interrupt endpoint
    #[inline(always)]
    pub fn write_endpoint(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ENDPOINT_MASK) | ((value << Self::ENDPOINT_OFFSET) & Self::ENDPOINT_MASK)
        });
    }

    /// **ADDRESS** `[6:0]` (RW)
    ///
    /// Device address
    #[inline(always)]
    pub fn address(&self) -> u32 {
        (self.value.read_bits() & Self::ADDRESS_MASK) >> Self::ADDRESS_OFFSET
    }

    /// Write **ADDRESS** `[6:0]` (RW)
    ///
    /// Device address
    #[inline(always)]
    pub fn write_address(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ADDRESS_MASK) | ((value << Self::ADDRESS_OFFSET) & Self::ADDRESS_MASK)
        });
    }
}

/// ADDR_ENDP5 value type for batch read/modify/write operations
///
/// Use `addrendp5.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `addrendp5.write_val(val)`.
#[derive(Clone, Copy)]
pub struct AddrEndp5Val {
    bits: u32,
}

impl AddrEndp5Val {
    /// Create from raw bits
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

    /// Read **INTEP_PREAMBLE** `[26:26]`
    ///
    /// Interrupt EP requires preamble (is a low speed device on a full speed hub)
    #[inline(always)]
    pub const fn intep_preamble(&self) -> bool {
        (self.bits & AddrEndp5::INTEP_PREAMBLE_MASK) != 0
    }

    /// Read **INTEP_DIR** `[25:25]`
    ///
    /// Direction of the interrupt endpoint. In=0, Out=1
    #[inline(always)]
    pub const fn intep_dir(&self) -> bool {
        (self.bits & AddrEndp5::INTEP_DIR_MASK) != 0
    }

    /// Read **ENDPOINT** `[19:16]`
    ///
    /// Endpoint number of the interrupt endpoint
    #[inline(always)]
    pub const fn endpoint(&self) -> u32 {
        (self.bits & AddrEndp5::ENDPOINT_MASK) >> AddrEndp5::ENDPOINT_OFFSET
    }

    /// Read **ADDRESS** `[6:0]`
    ///
    /// Device address
    #[inline(always)]
    pub const fn address(&self) -> u32 {
        (self.bits & AddrEndp5::ADDRESS_MASK) >> AddrEndp5::ADDRESS_OFFSET
    }

    /// Set **INTEP_PREAMBLE** `[26:26]`
    ///
    /// Interrupt EP requires preamble (is a low speed device on a full speed hub)
    #[inline(always)]
    pub const fn with_intep_preamble(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | AddrEndp5::INTEP_PREAMBLE_MASK
            } else {
                self.bits & !AddrEndp5::INTEP_PREAMBLE_MASK
            },
        }
    }

    /// Set **INTEP_DIR** `[25:25]`
    ///
    /// Direction of the interrupt endpoint. In=0, Out=1
    #[inline(always)]
    pub const fn with_intep_dir(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | AddrEndp5::INTEP_DIR_MASK
            } else {
                self.bits & !AddrEndp5::INTEP_DIR_MASK
            },
        }
    }

    /// Set **ENDPOINT** `[19:16]`
    ///
    /// Endpoint number of the interrupt endpoint
    #[inline(always)]
    pub const fn with_endpoint(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AddrEndp5::ENDPOINT_MASK)
                | ((value << AddrEndp5::ENDPOINT_OFFSET) & AddrEndp5::ENDPOINT_MASK),
        }
    }

    /// Set **ADDRESS** `[6:0]`
    ///
    /// Device address
    #[inline(always)]
    pub const fn with_address(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AddrEndp5::ADDRESS_MASK)
                | ((value << AddrEndp5::ADDRESS_OFFSET) & AddrEndp5::ADDRESS_MASK),
        }
    }
}

impl Default for AddrEndp5Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl AddrEndp5 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> AddrEndp5Val {
        AddrEndp5Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: AddrEndp5Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(AddrEndp5Val) -> AddrEndp5Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ADDR_ENDP6 Register
///
/// Interrupt endpoint 6. Only valid for HOST mode.
///
/// Address offset: 0x0018
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **INTEP_PREAMBLE** `[26]` (RW)
///
///   Interrupt EP requires preamble (is a low speed device on a full speed hub)
///
/// - **INTEP_DIR** `[25]` (RW)
///
///   Direction of the interrupt endpoint. In=0, Out=1
///
/// - **ENDPOINT** `[19:16]` (RW)
///
///   Endpoint number of the interrupt endpoint
///
/// - **ADDRESS** `[6:0]` (RW)
///
///   Device address
#[repr(transparent)]
pub struct AddrEndp6 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for AddrEndp6 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl AddrEndp6 {
    // ======== Field Constants ========

    /// **INTEP_PREAMBLE**
    ///
    /// Interrupt EP requires preamble (is a low speed device on a full speed hub)
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const INTEP_PREAMBLE_OFFSET: u32 = 26;
    pub const INTEP_PREAMBLE_MASK: u32 = 0x04000000;
    pub const INTEP_PREAMBLE: u32 = 1 << 26;

    /// **INTEP_DIR**
    ///
    /// Direction of the interrupt endpoint. In=0, Out=1
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const INTEP_DIR_OFFSET: u32 = 25;
    pub const INTEP_DIR_MASK: u32 = 0x02000000;
    pub const INTEP_DIR: u32 = 1 << 25;

    /// **ENDPOINT**
    ///
    /// Endpoint number of the interrupt endpoint
    ///
    /// Bits `[19:16]`, mask `0x000f0000`
    pub const ENDPOINT_OFFSET: u32 = 16;
    pub const ENDPOINT_MASK: u32 = 0x000f0000;

    /// **ADDRESS**
    ///
    /// Device address
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const ADDRESS_OFFSET: u32 = 0;
    pub const ADDRESS_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **INTEP_PREAMBLE** `[26]` (RW)
    ///
    /// Interrupt EP requires preamble (is a low speed device on a full speed hub)
    #[inline(always)]
    pub fn intep_preamble(&self) -> bool {
        (self.value.read_bits() & Self::INTEP_PREAMBLE_MASK) != 0
    }

    /// Set INTEP_PREAMBLE bit atomically (RW)
    #[inline(always)]
    pub fn set_intep_preamble(&self) {
        self.set_bits(Self::INTEP_PREAMBLE_MASK);
    }

    /// Clear INTEP_PREAMBLE bit atomically (RW)
    #[inline(always)]
    pub fn clr_intep_preamble(&self) {
        self.clr_bits(Self::INTEP_PREAMBLE_MASK);
    }

    /// **INTEP_DIR** `[25]` (RW)
    ///
    /// Direction of the interrupt endpoint. In=0, Out=1
    #[inline(always)]
    pub fn intep_dir(&self) -> bool {
        (self.value.read_bits() & Self::INTEP_DIR_MASK) != 0
    }

    /// Set INTEP_DIR bit atomically (RW)
    #[inline(always)]
    pub fn set_intep_dir(&self) {
        self.set_bits(Self::INTEP_DIR_MASK);
    }

    /// Clear INTEP_DIR bit atomically (RW)
    #[inline(always)]
    pub fn clr_intep_dir(&self) {
        self.clr_bits(Self::INTEP_DIR_MASK);
    }

    /// **ENDPOINT** `[19:16]` (RW)
    ///
    /// Endpoint number of the interrupt endpoint
    #[inline(always)]
    pub fn endpoint(&self) -> u32 {
        (self.value.read_bits() & Self::ENDPOINT_MASK) >> Self::ENDPOINT_OFFSET
    }

    /// Write **ENDPOINT** `[19:16]` (RW)
    ///
    /// Endpoint number of the interrupt endpoint
    #[inline(always)]
    pub fn write_endpoint(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ENDPOINT_MASK) | ((value << Self::ENDPOINT_OFFSET) & Self::ENDPOINT_MASK)
        });
    }

    /// **ADDRESS** `[6:0]` (RW)
    ///
    /// Device address
    #[inline(always)]
    pub fn address(&self) -> u32 {
        (self.value.read_bits() & Self::ADDRESS_MASK) >> Self::ADDRESS_OFFSET
    }

    /// Write **ADDRESS** `[6:0]` (RW)
    ///
    /// Device address
    #[inline(always)]
    pub fn write_address(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ADDRESS_MASK) | ((value << Self::ADDRESS_OFFSET) & Self::ADDRESS_MASK)
        });
    }
}

/// ADDR_ENDP6 value type for batch read/modify/write operations
///
/// Use `addrendp6.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `addrendp6.write_val(val)`.
#[derive(Clone, Copy)]
pub struct AddrEndp6Val {
    bits: u32,
}

impl AddrEndp6Val {
    /// Create from raw bits
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

    /// Read **INTEP_PREAMBLE** `[26:26]`
    ///
    /// Interrupt EP requires preamble (is a low speed device on a full speed hub)
    #[inline(always)]
    pub const fn intep_preamble(&self) -> bool {
        (self.bits & AddrEndp6::INTEP_PREAMBLE_MASK) != 0
    }

    /// Read **INTEP_DIR** `[25:25]`
    ///
    /// Direction of the interrupt endpoint. In=0, Out=1
    #[inline(always)]
    pub const fn intep_dir(&self) -> bool {
        (self.bits & AddrEndp6::INTEP_DIR_MASK) != 0
    }

    /// Read **ENDPOINT** `[19:16]`
    ///
    /// Endpoint number of the interrupt endpoint
    #[inline(always)]
    pub const fn endpoint(&self) -> u32 {
        (self.bits & AddrEndp6::ENDPOINT_MASK) >> AddrEndp6::ENDPOINT_OFFSET
    }

    /// Read **ADDRESS** `[6:0]`
    ///
    /// Device address
    #[inline(always)]
    pub const fn address(&self) -> u32 {
        (self.bits & AddrEndp6::ADDRESS_MASK) >> AddrEndp6::ADDRESS_OFFSET
    }

    /// Set **INTEP_PREAMBLE** `[26:26]`
    ///
    /// Interrupt EP requires preamble (is a low speed device on a full speed hub)
    #[inline(always)]
    pub const fn with_intep_preamble(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | AddrEndp6::INTEP_PREAMBLE_MASK
            } else {
                self.bits & !AddrEndp6::INTEP_PREAMBLE_MASK
            },
        }
    }

    /// Set **INTEP_DIR** `[25:25]`
    ///
    /// Direction of the interrupt endpoint. In=0, Out=1
    #[inline(always)]
    pub const fn with_intep_dir(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | AddrEndp6::INTEP_DIR_MASK
            } else {
                self.bits & !AddrEndp6::INTEP_DIR_MASK
            },
        }
    }

    /// Set **ENDPOINT** `[19:16]`
    ///
    /// Endpoint number of the interrupt endpoint
    #[inline(always)]
    pub const fn with_endpoint(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AddrEndp6::ENDPOINT_MASK)
                | ((value << AddrEndp6::ENDPOINT_OFFSET) & AddrEndp6::ENDPOINT_MASK),
        }
    }

    /// Set **ADDRESS** `[6:0]`
    ///
    /// Device address
    #[inline(always)]
    pub const fn with_address(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AddrEndp6::ADDRESS_MASK)
                | ((value << AddrEndp6::ADDRESS_OFFSET) & AddrEndp6::ADDRESS_MASK),
        }
    }
}

impl Default for AddrEndp6Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl AddrEndp6 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> AddrEndp6Val {
        AddrEndp6Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: AddrEndp6Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(AddrEndp6Val) -> AddrEndp6Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ADDR_ENDP7 Register
///
/// Interrupt endpoint 7. Only valid for HOST mode.
///
/// Address offset: 0x001c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **INTEP_PREAMBLE** `[26]` (RW)
///
///   Interrupt EP requires preamble (is a low speed device on a full speed hub)
///
/// - **INTEP_DIR** `[25]` (RW)
///
///   Direction of the interrupt endpoint. In=0, Out=1
///
/// - **ENDPOINT** `[19:16]` (RW)
///
///   Endpoint number of the interrupt endpoint
///
/// - **ADDRESS** `[6:0]` (RW)
///
///   Device address
#[repr(transparent)]
pub struct AddrEndp7 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for AddrEndp7 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl AddrEndp7 {
    // ======== Field Constants ========

    /// **INTEP_PREAMBLE**
    ///
    /// Interrupt EP requires preamble (is a low speed device on a full speed hub)
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const INTEP_PREAMBLE_OFFSET: u32 = 26;
    pub const INTEP_PREAMBLE_MASK: u32 = 0x04000000;
    pub const INTEP_PREAMBLE: u32 = 1 << 26;

    /// **INTEP_DIR**
    ///
    /// Direction of the interrupt endpoint. In=0, Out=1
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const INTEP_DIR_OFFSET: u32 = 25;
    pub const INTEP_DIR_MASK: u32 = 0x02000000;
    pub const INTEP_DIR: u32 = 1 << 25;

    /// **ENDPOINT**
    ///
    /// Endpoint number of the interrupt endpoint
    ///
    /// Bits `[19:16]`, mask `0x000f0000`
    pub const ENDPOINT_OFFSET: u32 = 16;
    pub const ENDPOINT_MASK: u32 = 0x000f0000;

    /// **ADDRESS**
    ///
    /// Device address
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const ADDRESS_OFFSET: u32 = 0;
    pub const ADDRESS_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **INTEP_PREAMBLE** `[26]` (RW)
    ///
    /// Interrupt EP requires preamble (is a low speed device on a full speed hub)
    #[inline(always)]
    pub fn intep_preamble(&self) -> bool {
        (self.value.read_bits() & Self::INTEP_PREAMBLE_MASK) != 0
    }

    /// Set INTEP_PREAMBLE bit atomically (RW)
    #[inline(always)]
    pub fn set_intep_preamble(&self) {
        self.set_bits(Self::INTEP_PREAMBLE_MASK);
    }

    /// Clear INTEP_PREAMBLE bit atomically (RW)
    #[inline(always)]
    pub fn clr_intep_preamble(&self) {
        self.clr_bits(Self::INTEP_PREAMBLE_MASK);
    }

    /// **INTEP_DIR** `[25]` (RW)
    ///
    /// Direction of the interrupt endpoint. In=0, Out=1
    #[inline(always)]
    pub fn intep_dir(&self) -> bool {
        (self.value.read_bits() & Self::INTEP_DIR_MASK) != 0
    }

    /// Set INTEP_DIR bit atomically (RW)
    #[inline(always)]
    pub fn set_intep_dir(&self) {
        self.set_bits(Self::INTEP_DIR_MASK);
    }

    /// Clear INTEP_DIR bit atomically (RW)
    #[inline(always)]
    pub fn clr_intep_dir(&self) {
        self.clr_bits(Self::INTEP_DIR_MASK);
    }

    /// **ENDPOINT** `[19:16]` (RW)
    ///
    /// Endpoint number of the interrupt endpoint
    #[inline(always)]
    pub fn endpoint(&self) -> u32 {
        (self.value.read_bits() & Self::ENDPOINT_MASK) >> Self::ENDPOINT_OFFSET
    }

    /// Write **ENDPOINT** `[19:16]` (RW)
    ///
    /// Endpoint number of the interrupt endpoint
    #[inline(always)]
    pub fn write_endpoint(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ENDPOINT_MASK) | ((value << Self::ENDPOINT_OFFSET) & Self::ENDPOINT_MASK)
        });
    }

    /// **ADDRESS** `[6:0]` (RW)
    ///
    /// Device address
    #[inline(always)]
    pub fn address(&self) -> u32 {
        (self.value.read_bits() & Self::ADDRESS_MASK) >> Self::ADDRESS_OFFSET
    }

    /// Write **ADDRESS** `[6:0]` (RW)
    ///
    /// Device address
    #[inline(always)]
    pub fn write_address(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ADDRESS_MASK) | ((value << Self::ADDRESS_OFFSET) & Self::ADDRESS_MASK)
        });
    }
}

/// ADDR_ENDP7 value type for batch read/modify/write operations
///
/// Use `addrendp7.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `addrendp7.write_val(val)`.
#[derive(Clone, Copy)]
pub struct AddrEndp7Val {
    bits: u32,
}

impl AddrEndp7Val {
    /// Create from raw bits
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

    /// Read **INTEP_PREAMBLE** `[26:26]`
    ///
    /// Interrupt EP requires preamble (is a low speed device on a full speed hub)
    #[inline(always)]
    pub const fn intep_preamble(&self) -> bool {
        (self.bits & AddrEndp7::INTEP_PREAMBLE_MASK) != 0
    }

    /// Read **INTEP_DIR** `[25:25]`
    ///
    /// Direction of the interrupt endpoint. In=0, Out=1
    #[inline(always)]
    pub const fn intep_dir(&self) -> bool {
        (self.bits & AddrEndp7::INTEP_DIR_MASK) != 0
    }

    /// Read **ENDPOINT** `[19:16]`
    ///
    /// Endpoint number of the interrupt endpoint
    #[inline(always)]
    pub const fn endpoint(&self) -> u32 {
        (self.bits & AddrEndp7::ENDPOINT_MASK) >> AddrEndp7::ENDPOINT_OFFSET
    }

    /// Read **ADDRESS** `[6:0]`
    ///
    /// Device address
    #[inline(always)]
    pub const fn address(&self) -> u32 {
        (self.bits & AddrEndp7::ADDRESS_MASK) >> AddrEndp7::ADDRESS_OFFSET
    }

    /// Set **INTEP_PREAMBLE** `[26:26]`
    ///
    /// Interrupt EP requires preamble (is a low speed device on a full speed hub)
    #[inline(always)]
    pub const fn with_intep_preamble(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | AddrEndp7::INTEP_PREAMBLE_MASK
            } else {
                self.bits & !AddrEndp7::INTEP_PREAMBLE_MASK
            },
        }
    }

    /// Set **INTEP_DIR** `[25:25]`
    ///
    /// Direction of the interrupt endpoint. In=0, Out=1
    #[inline(always)]
    pub const fn with_intep_dir(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | AddrEndp7::INTEP_DIR_MASK
            } else {
                self.bits & !AddrEndp7::INTEP_DIR_MASK
            },
        }
    }

    /// Set **ENDPOINT** `[19:16]`
    ///
    /// Endpoint number of the interrupt endpoint
    #[inline(always)]
    pub const fn with_endpoint(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AddrEndp7::ENDPOINT_MASK)
                | ((value << AddrEndp7::ENDPOINT_OFFSET) & AddrEndp7::ENDPOINT_MASK),
        }
    }

    /// Set **ADDRESS** `[6:0]`
    ///
    /// Device address
    #[inline(always)]
    pub const fn with_address(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AddrEndp7::ADDRESS_MASK)
                | ((value << AddrEndp7::ADDRESS_OFFSET) & AddrEndp7::ADDRESS_MASK),
        }
    }
}

impl Default for AddrEndp7Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl AddrEndp7 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> AddrEndp7Val {
        AddrEndp7Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: AddrEndp7Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(AddrEndp7Val) -> AddrEndp7Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ADDR_ENDP8 Register
///
/// Interrupt endpoint 8. Only valid for HOST mode.
///
/// Address offset: 0x0020
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **INTEP_PREAMBLE** `[26]` (RW)
///
///   Interrupt EP requires preamble (is a low speed device on a full speed hub)
///
/// - **INTEP_DIR** `[25]` (RW)
///
///   Direction of the interrupt endpoint. In=0, Out=1
///
/// - **ENDPOINT** `[19:16]` (RW)
///
///   Endpoint number of the interrupt endpoint
///
/// - **ADDRESS** `[6:0]` (RW)
///
///   Device address
#[repr(transparent)]
pub struct AddrEndp8 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for AddrEndp8 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl AddrEndp8 {
    // ======== Field Constants ========

    /// **INTEP_PREAMBLE**
    ///
    /// Interrupt EP requires preamble (is a low speed device on a full speed hub)
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const INTEP_PREAMBLE_OFFSET: u32 = 26;
    pub const INTEP_PREAMBLE_MASK: u32 = 0x04000000;
    pub const INTEP_PREAMBLE: u32 = 1 << 26;

    /// **INTEP_DIR**
    ///
    /// Direction of the interrupt endpoint. In=0, Out=1
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const INTEP_DIR_OFFSET: u32 = 25;
    pub const INTEP_DIR_MASK: u32 = 0x02000000;
    pub const INTEP_DIR: u32 = 1 << 25;

    /// **ENDPOINT**
    ///
    /// Endpoint number of the interrupt endpoint
    ///
    /// Bits `[19:16]`, mask `0x000f0000`
    pub const ENDPOINT_OFFSET: u32 = 16;
    pub const ENDPOINT_MASK: u32 = 0x000f0000;

    /// **ADDRESS**
    ///
    /// Device address
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const ADDRESS_OFFSET: u32 = 0;
    pub const ADDRESS_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **INTEP_PREAMBLE** `[26]` (RW)
    ///
    /// Interrupt EP requires preamble (is a low speed device on a full speed hub)
    #[inline(always)]
    pub fn intep_preamble(&self) -> bool {
        (self.value.read_bits() & Self::INTEP_PREAMBLE_MASK) != 0
    }

    /// Set INTEP_PREAMBLE bit atomically (RW)
    #[inline(always)]
    pub fn set_intep_preamble(&self) {
        self.set_bits(Self::INTEP_PREAMBLE_MASK);
    }

    /// Clear INTEP_PREAMBLE bit atomically (RW)
    #[inline(always)]
    pub fn clr_intep_preamble(&self) {
        self.clr_bits(Self::INTEP_PREAMBLE_MASK);
    }

    /// **INTEP_DIR** `[25]` (RW)
    ///
    /// Direction of the interrupt endpoint. In=0, Out=1
    #[inline(always)]
    pub fn intep_dir(&self) -> bool {
        (self.value.read_bits() & Self::INTEP_DIR_MASK) != 0
    }

    /// Set INTEP_DIR bit atomically (RW)
    #[inline(always)]
    pub fn set_intep_dir(&self) {
        self.set_bits(Self::INTEP_DIR_MASK);
    }

    /// Clear INTEP_DIR bit atomically (RW)
    #[inline(always)]
    pub fn clr_intep_dir(&self) {
        self.clr_bits(Self::INTEP_DIR_MASK);
    }

    /// **ENDPOINT** `[19:16]` (RW)
    ///
    /// Endpoint number of the interrupt endpoint
    #[inline(always)]
    pub fn endpoint(&self) -> u32 {
        (self.value.read_bits() & Self::ENDPOINT_MASK) >> Self::ENDPOINT_OFFSET
    }

    /// Write **ENDPOINT** `[19:16]` (RW)
    ///
    /// Endpoint number of the interrupt endpoint
    #[inline(always)]
    pub fn write_endpoint(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ENDPOINT_MASK) | ((value << Self::ENDPOINT_OFFSET) & Self::ENDPOINT_MASK)
        });
    }

    /// **ADDRESS** `[6:0]` (RW)
    ///
    /// Device address
    #[inline(always)]
    pub fn address(&self) -> u32 {
        (self.value.read_bits() & Self::ADDRESS_MASK) >> Self::ADDRESS_OFFSET
    }

    /// Write **ADDRESS** `[6:0]` (RW)
    ///
    /// Device address
    #[inline(always)]
    pub fn write_address(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ADDRESS_MASK) | ((value << Self::ADDRESS_OFFSET) & Self::ADDRESS_MASK)
        });
    }
}

/// ADDR_ENDP8 value type for batch read/modify/write operations
///
/// Use `addrendp8.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `addrendp8.write_val(val)`.
#[derive(Clone, Copy)]
pub struct AddrEndp8Val {
    bits: u32,
}

impl AddrEndp8Val {
    /// Create from raw bits
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

    /// Read **INTEP_PREAMBLE** `[26:26]`
    ///
    /// Interrupt EP requires preamble (is a low speed device on a full speed hub)
    #[inline(always)]
    pub const fn intep_preamble(&self) -> bool {
        (self.bits & AddrEndp8::INTEP_PREAMBLE_MASK) != 0
    }

    /// Read **INTEP_DIR** `[25:25]`
    ///
    /// Direction of the interrupt endpoint. In=0, Out=1
    #[inline(always)]
    pub const fn intep_dir(&self) -> bool {
        (self.bits & AddrEndp8::INTEP_DIR_MASK) != 0
    }

    /// Read **ENDPOINT** `[19:16]`
    ///
    /// Endpoint number of the interrupt endpoint
    #[inline(always)]
    pub const fn endpoint(&self) -> u32 {
        (self.bits & AddrEndp8::ENDPOINT_MASK) >> AddrEndp8::ENDPOINT_OFFSET
    }

    /// Read **ADDRESS** `[6:0]`
    ///
    /// Device address
    #[inline(always)]
    pub const fn address(&self) -> u32 {
        (self.bits & AddrEndp8::ADDRESS_MASK) >> AddrEndp8::ADDRESS_OFFSET
    }

    /// Set **INTEP_PREAMBLE** `[26:26]`
    ///
    /// Interrupt EP requires preamble (is a low speed device on a full speed hub)
    #[inline(always)]
    pub const fn with_intep_preamble(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | AddrEndp8::INTEP_PREAMBLE_MASK
            } else {
                self.bits & !AddrEndp8::INTEP_PREAMBLE_MASK
            },
        }
    }

    /// Set **INTEP_DIR** `[25:25]`
    ///
    /// Direction of the interrupt endpoint. In=0, Out=1
    #[inline(always)]
    pub const fn with_intep_dir(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | AddrEndp8::INTEP_DIR_MASK
            } else {
                self.bits & !AddrEndp8::INTEP_DIR_MASK
            },
        }
    }

    /// Set **ENDPOINT** `[19:16]`
    ///
    /// Endpoint number of the interrupt endpoint
    #[inline(always)]
    pub const fn with_endpoint(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AddrEndp8::ENDPOINT_MASK)
                | ((value << AddrEndp8::ENDPOINT_OFFSET) & AddrEndp8::ENDPOINT_MASK),
        }
    }

    /// Set **ADDRESS** `[6:0]`
    ///
    /// Device address
    #[inline(always)]
    pub const fn with_address(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AddrEndp8::ADDRESS_MASK)
                | ((value << AddrEndp8::ADDRESS_OFFSET) & AddrEndp8::ADDRESS_MASK),
        }
    }
}

impl Default for AddrEndp8Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl AddrEndp8 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> AddrEndp8Val {
        AddrEndp8Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: AddrEndp8Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(AddrEndp8Val) -> AddrEndp8Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ADDR_ENDP9 Register
///
/// Interrupt endpoint 9. Only valid for HOST mode.
///
/// Address offset: 0x0024
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **INTEP_PREAMBLE** `[26]` (RW)
///
///   Interrupt EP requires preamble (is a low speed device on a full speed hub)
///
/// - **INTEP_DIR** `[25]` (RW)
///
///   Direction of the interrupt endpoint. In=0, Out=1
///
/// - **ENDPOINT** `[19:16]` (RW)
///
///   Endpoint number of the interrupt endpoint
///
/// - **ADDRESS** `[6:0]` (RW)
///
///   Device address
#[repr(transparent)]
pub struct AddrEndp9 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for AddrEndp9 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl AddrEndp9 {
    // ======== Field Constants ========

    /// **INTEP_PREAMBLE**
    ///
    /// Interrupt EP requires preamble (is a low speed device on a full speed hub)
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const INTEP_PREAMBLE_OFFSET: u32 = 26;
    pub const INTEP_PREAMBLE_MASK: u32 = 0x04000000;
    pub const INTEP_PREAMBLE: u32 = 1 << 26;

    /// **INTEP_DIR**
    ///
    /// Direction of the interrupt endpoint. In=0, Out=1
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const INTEP_DIR_OFFSET: u32 = 25;
    pub const INTEP_DIR_MASK: u32 = 0x02000000;
    pub const INTEP_DIR: u32 = 1 << 25;

    /// **ENDPOINT**
    ///
    /// Endpoint number of the interrupt endpoint
    ///
    /// Bits `[19:16]`, mask `0x000f0000`
    pub const ENDPOINT_OFFSET: u32 = 16;
    pub const ENDPOINT_MASK: u32 = 0x000f0000;

    /// **ADDRESS**
    ///
    /// Device address
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const ADDRESS_OFFSET: u32 = 0;
    pub const ADDRESS_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **INTEP_PREAMBLE** `[26]` (RW)
    ///
    /// Interrupt EP requires preamble (is a low speed device on a full speed hub)
    #[inline(always)]
    pub fn intep_preamble(&self) -> bool {
        (self.value.read_bits() & Self::INTEP_PREAMBLE_MASK) != 0
    }

    /// Set INTEP_PREAMBLE bit atomically (RW)
    #[inline(always)]
    pub fn set_intep_preamble(&self) {
        self.set_bits(Self::INTEP_PREAMBLE_MASK);
    }

    /// Clear INTEP_PREAMBLE bit atomically (RW)
    #[inline(always)]
    pub fn clr_intep_preamble(&self) {
        self.clr_bits(Self::INTEP_PREAMBLE_MASK);
    }

    /// **INTEP_DIR** `[25]` (RW)
    ///
    /// Direction of the interrupt endpoint. In=0, Out=1
    #[inline(always)]
    pub fn intep_dir(&self) -> bool {
        (self.value.read_bits() & Self::INTEP_DIR_MASK) != 0
    }

    /// Set INTEP_DIR bit atomically (RW)
    #[inline(always)]
    pub fn set_intep_dir(&self) {
        self.set_bits(Self::INTEP_DIR_MASK);
    }

    /// Clear INTEP_DIR bit atomically (RW)
    #[inline(always)]
    pub fn clr_intep_dir(&self) {
        self.clr_bits(Self::INTEP_DIR_MASK);
    }

    /// **ENDPOINT** `[19:16]` (RW)
    ///
    /// Endpoint number of the interrupt endpoint
    #[inline(always)]
    pub fn endpoint(&self) -> u32 {
        (self.value.read_bits() & Self::ENDPOINT_MASK) >> Self::ENDPOINT_OFFSET
    }

    /// Write **ENDPOINT** `[19:16]` (RW)
    ///
    /// Endpoint number of the interrupt endpoint
    #[inline(always)]
    pub fn write_endpoint(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ENDPOINT_MASK) | ((value << Self::ENDPOINT_OFFSET) & Self::ENDPOINT_MASK)
        });
    }

    /// **ADDRESS** `[6:0]` (RW)
    ///
    /// Device address
    #[inline(always)]
    pub fn address(&self) -> u32 {
        (self.value.read_bits() & Self::ADDRESS_MASK) >> Self::ADDRESS_OFFSET
    }

    /// Write **ADDRESS** `[6:0]` (RW)
    ///
    /// Device address
    #[inline(always)]
    pub fn write_address(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ADDRESS_MASK) | ((value << Self::ADDRESS_OFFSET) & Self::ADDRESS_MASK)
        });
    }
}

/// ADDR_ENDP9 value type for batch read/modify/write operations
///
/// Use `addrendp9.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `addrendp9.write_val(val)`.
#[derive(Clone, Copy)]
pub struct AddrEndp9Val {
    bits: u32,
}

impl AddrEndp9Val {
    /// Create from raw bits
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

    /// Read **INTEP_PREAMBLE** `[26:26]`
    ///
    /// Interrupt EP requires preamble (is a low speed device on a full speed hub)
    #[inline(always)]
    pub const fn intep_preamble(&self) -> bool {
        (self.bits & AddrEndp9::INTEP_PREAMBLE_MASK) != 0
    }

    /// Read **INTEP_DIR** `[25:25]`
    ///
    /// Direction of the interrupt endpoint. In=0, Out=1
    #[inline(always)]
    pub const fn intep_dir(&self) -> bool {
        (self.bits & AddrEndp9::INTEP_DIR_MASK) != 0
    }

    /// Read **ENDPOINT** `[19:16]`
    ///
    /// Endpoint number of the interrupt endpoint
    #[inline(always)]
    pub const fn endpoint(&self) -> u32 {
        (self.bits & AddrEndp9::ENDPOINT_MASK) >> AddrEndp9::ENDPOINT_OFFSET
    }

    /// Read **ADDRESS** `[6:0]`
    ///
    /// Device address
    #[inline(always)]
    pub const fn address(&self) -> u32 {
        (self.bits & AddrEndp9::ADDRESS_MASK) >> AddrEndp9::ADDRESS_OFFSET
    }

    /// Set **INTEP_PREAMBLE** `[26:26]`
    ///
    /// Interrupt EP requires preamble (is a low speed device on a full speed hub)
    #[inline(always)]
    pub const fn with_intep_preamble(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | AddrEndp9::INTEP_PREAMBLE_MASK
            } else {
                self.bits & !AddrEndp9::INTEP_PREAMBLE_MASK
            },
        }
    }

    /// Set **INTEP_DIR** `[25:25]`
    ///
    /// Direction of the interrupt endpoint. In=0, Out=1
    #[inline(always)]
    pub const fn with_intep_dir(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | AddrEndp9::INTEP_DIR_MASK
            } else {
                self.bits & !AddrEndp9::INTEP_DIR_MASK
            },
        }
    }

    /// Set **ENDPOINT** `[19:16]`
    ///
    /// Endpoint number of the interrupt endpoint
    #[inline(always)]
    pub const fn with_endpoint(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AddrEndp9::ENDPOINT_MASK)
                | ((value << AddrEndp9::ENDPOINT_OFFSET) & AddrEndp9::ENDPOINT_MASK),
        }
    }

    /// Set **ADDRESS** `[6:0]`
    ///
    /// Device address
    #[inline(always)]
    pub const fn with_address(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AddrEndp9::ADDRESS_MASK)
                | ((value << AddrEndp9::ADDRESS_OFFSET) & AddrEndp9::ADDRESS_MASK),
        }
    }
}

impl Default for AddrEndp9Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl AddrEndp9 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> AddrEndp9Val {
        AddrEndp9Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: AddrEndp9Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(AddrEndp9Val) -> AddrEndp9Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ADDR_ENDP10 Register
///
/// Interrupt endpoint 10. Only valid for HOST mode.
///
/// Address offset: 0x0028
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **INTEP_PREAMBLE** `[26]` (RW)
///
///   Interrupt EP requires preamble (is a low speed device on a full speed hub)
///
/// - **INTEP_DIR** `[25]` (RW)
///
///   Direction of the interrupt endpoint. In=0, Out=1
///
/// - **ENDPOINT** `[19:16]` (RW)
///
///   Endpoint number of the interrupt endpoint
///
/// - **ADDRESS** `[6:0]` (RW)
///
///   Device address
#[repr(transparent)]
pub struct AddrEndp10 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for AddrEndp10 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl AddrEndp10 {
    // ======== Field Constants ========

    /// **INTEP_PREAMBLE**
    ///
    /// Interrupt EP requires preamble (is a low speed device on a full speed hub)
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const INTEP_PREAMBLE_OFFSET: u32 = 26;
    pub const INTEP_PREAMBLE_MASK: u32 = 0x04000000;
    pub const INTEP_PREAMBLE: u32 = 1 << 26;

    /// **INTEP_DIR**
    ///
    /// Direction of the interrupt endpoint. In=0, Out=1
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const INTEP_DIR_OFFSET: u32 = 25;
    pub const INTEP_DIR_MASK: u32 = 0x02000000;
    pub const INTEP_DIR: u32 = 1 << 25;

    /// **ENDPOINT**
    ///
    /// Endpoint number of the interrupt endpoint
    ///
    /// Bits `[19:16]`, mask `0x000f0000`
    pub const ENDPOINT_OFFSET: u32 = 16;
    pub const ENDPOINT_MASK: u32 = 0x000f0000;

    /// **ADDRESS**
    ///
    /// Device address
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const ADDRESS_OFFSET: u32 = 0;
    pub const ADDRESS_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **INTEP_PREAMBLE** `[26]` (RW)
    ///
    /// Interrupt EP requires preamble (is a low speed device on a full speed hub)
    #[inline(always)]
    pub fn intep_preamble(&self) -> bool {
        (self.value.read_bits() & Self::INTEP_PREAMBLE_MASK) != 0
    }

    /// Set INTEP_PREAMBLE bit atomically (RW)
    #[inline(always)]
    pub fn set_intep_preamble(&self) {
        self.set_bits(Self::INTEP_PREAMBLE_MASK);
    }

    /// Clear INTEP_PREAMBLE bit atomically (RW)
    #[inline(always)]
    pub fn clr_intep_preamble(&self) {
        self.clr_bits(Self::INTEP_PREAMBLE_MASK);
    }

    /// **INTEP_DIR** `[25]` (RW)
    ///
    /// Direction of the interrupt endpoint. In=0, Out=1
    #[inline(always)]
    pub fn intep_dir(&self) -> bool {
        (self.value.read_bits() & Self::INTEP_DIR_MASK) != 0
    }

    /// Set INTEP_DIR bit atomically (RW)
    #[inline(always)]
    pub fn set_intep_dir(&self) {
        self.set_bits(Self::INTEP_DIR_MASK);
    }

    /// Clear INTEP_DIR bit atomically (RW)
    #[inline(always)]
    pub fn clr_intep_dir(&self) {
        self.clr_bits(Self::INTEP_DIR_MASK);
    }

    /// **ENDPOINT** `[19:16]` (RW)
    ///
    /// Endpoint number of the interrupt endpoint
    #[inline(always)]
    pub fn endpoint(&self) -> u32 {
        (self.value.read_bits() & Self::ENDPOINT_MASK) >> Self::ENDPOINT_OFFSET
    }

    /// Write **ENDPOINT** `[19:16]` (RW)
    ///
    /// Endpoint number of the interrupt endpoint
    #[inline(always)]
    pub fn write_endpoint(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ENDPOINT_MASK) | ((value << Self::ENDPOINT_OFFSET) & Self::ENDPOINT_MASK)
        });
    }

    /// **ADDRESS** `[6:0]` (RW)
    ///
    /// Device address
    #[inline(always)]
    pub fn address(&self) -> u32 {
        (self.value.read_bits() & Self::ADDRESS_MASK) >> Self::ADDRESS_OFFSET
    }

    /// Write **ADDRESS** `[6:0]` (RW)
    ///
    /// Device address
    #[inline(always)]
    pub fn write_address(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ADDRESS_MASK) | ((value << Self::ADDRESS_OFFSET) & Self::ADDRESS_MASK)
        });
    }
}

/// ADDR_ENDP10 value type for batch read/modify/write operations
///
/// Use `addrendp10.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `addrendp10.write_val(val)`.
#[derive(Clone, Copy)]
pub struct AddrEndp10Val {
    bits: u32,
}

impl AddrEndp10Val {
    /// Create from raw bits
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

    /// Read **INTEP_PREAMBLE** `[26:26]`
    ///
    /// Interrupt EP requires preamble (is a low speed device on a full speed hub)
    #[inline(always)]
    pub const fn intep_preamble(&self) -> bool {
        (self.bits & AddrEndp10::INTEP_PREAMBLE_MASK) != 0
    }

    /// Read **INTEP_DIR** `[25:25]`
    ///
    /// Direction of the interrupt endpoint. In=0, Out=1
    #[inline(always)]
    pub const fn intep_dir(&self) -> bool {
        (self.bits & AddrEndp10::INTEP_DIR_MASK) != 0
    }

    /// Read **ENDPOINT** `[19:16]`
    ///
    /// Endpoint number of the interrupt endpoint
    #[inline(always)]
    pub const fn endpoint(&self) -> u32 {
        (self.bits & AddrEndp10::ENDPOINT_MASK) >> AddrEndp10::ENDPOINT_OFFSET
    }

    /// Read **ADDRESS** `[6:0]`
    ///
    /// Device address
    #[inline(always)]
    pub const fn address(&self) -> u32 {
        (self.bits & AddrEndp10::ADDRESS_MASK) >> AddrEndp10::ADDRESS_OFFSET
    }

    /// Set **INTEP_PREAMBLE** `[26:26]`
    ///
    /// Interrupt EP requires preamble (is a low speed device on a full speed hub)
    #[inline(always)]
    pub const fn with_intep_preamble(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | AddrEndp10::INTEP_PREAMBLE_MASK
            } else {
                self.bits & !AddrEndp10::INTEP_PREAMBLE_MASK
            },
        }
    }

    /// Set **INTEP_DIR** `[25:25]`
    ///
    /// Direction of the interrupt endpoint. In=0, Out=1
    #[inline(always)]
    pub const fn with_intep_dir(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | AddrEndp10::INTEP_DIR_MASK
            } else {
                self.bits & !AddrEndp10::INTEP_DIR_MASK
            },
        }
    }

    /// Set **ENDPOINT** `[19:16]`
    ///
    /// Endpoint number of the interrupt endpoint
    #[inline(always)]
    pub const fn with_endpoint(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AddrEndp10::ENDPOINT_MASK)
                | ((value << AddrEndp10::ENDPOINT_OFFSET) & AddrEndp10::ENDPOINT_MASK),
        }
    }

    /// Set **ADDRESS** `[6:0]`
    ///
    /// Device address
    #[inline(always)]
    pub const fn with_address(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AddrEndp10::ADDRESS_MASK)
                | ((value << AddrEndp10::ADDRESS_OFFSET) & AddrEndp10::ADDRESS_MASK),
        }
    }
}

impl Default for AddrEndp10Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl AddrEndp10 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> AddrEndp10Val {
        AddrEndp10Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: AddrEndp10Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(AddrEndp10Val) -> AddrEndp10Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ADDR_ENDP11 Register
///
/// Interrupt endpoint 11. Only valid for HOST mode.
///
/// Address offset: 0x002c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **INTEP_PREAMBLE** `[26]` (RW)
///
///   Interrupt EP requires preamble (is a low speed device on a full speed hub)
///
/// - **INTEP_DIR** `[25]` (RW)
///
///   Direction of the interrupt endpoint. In=0, Out=1
///
/// - **ENDPOINT** `[19:16]` (RW)
///
///   Endpoint number of the interrupt endpoint
///
/// - **ADDRESS** `[6:0]` (RW)
///
///   Device address
#[repr(transparent)]
pub struct AddrEndp11 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for AddrEndp11 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl AddrEndp11 {
    // ======== Field Constants ========

    /// **INTEP_PREAMBLE**
    ///
    /// Interrupt EP requires preamble (is a low speed device on a full speed hub)
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const INTEP_PREAMBLE_OFFSET: u32 = 26;
    pub const INTEP_PREAMBLE_MASK: u32 = 0x04000000;
    pub const INTEP_PREAMBLE: u32 = 1 << 26;

    /// **INTEP_DIR**
    ///
    /// Direction of the interrupt endpoint. In=0, Out=1
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const INTEP_DIR_OFFSET: u32 = 25;
    pub const INTEP_DIR_MASK: u32 = 0x02000000;
    pub const INTEP_DIR: u32 = 1 << 25;

    /// **ENDPOINT**
    ///
    /// Endpoint number of the interrupt endpoint
    ///
    /// Bits `[19:16]`, mask `0x000f0000`
    pub const ENDPOINT_OFFSET: u32 = 16;
    pub const ENDPOINT_MASK: u32 = 0x000f0000;

    /// **ADDRESS**
    ///
    /// Device address
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const ADDRESS_OFFSET: u32 = 0;
    pub const ADDRESS_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **INTEP_PREAMBLE** `[26]` (RW)
    ///
    /// Interrupt EP requires preamble (is a low speed device on a full speed hub)
    #[inline(always)]
    pub fn intep_preamble(&self) -> bool {
        (self.value.read_bits() & Self::INTEP_PREAMBLE_MASK) != 0
    }

    /// Set INTEP_PREAMBLE bit atomically (RW)
    #[inline(always)]
    pub fn set_intep_preamble(&self) {
        self.set_bits(Self::INTEP_PREAMBLE_MASK);
    }

    /// Clear INTEP_PREAMBLE bit atomically (RW)
    #[inline(always)]
    pub fn clr_intep_preamble(&self) {
        self.clr_bits(Self::INTEP_PREAMBLE_MASK);
    }

    /// **INTEP_DIR** `[25]` (RW)
    ///
    /// Direction of the interrupt endpoint. In=0, Out=1
    #[inline(always)]
    pub fn intep_dir(&self) -> bool {
        (self.value.read_bits() & Self::INTEP_DIR_MASK) != 0
    }

    /// Set INTEP_DIR bit atomically (RW)
    #[inline(always)]
    pub fn set_intep_dir(&self) {
        self.set_bits(Self::INTEP_DIR_MASK);
    }

    /// Clear INTEP_DIR bit atomically (RW)
    #[inline(always)]
    pub fn clr_intep_dir(&self) {
        self.clr_bits(Self::INTEP_DIR_MASK);
    }

    /// **ENDPOINT** `[19:16]` (RW)
    ///
    /// Endpoint number of the interrupt endpoint
    #[inline(always)]
    pub fn endpoint(&self) -> u32 {
        (self.value.read_bits() & Self::ENDPOINT_MASK) >> Self::ENDPOINT_OFFSET
    }

    /// Write **ENDPOINT** `[19:16]` (RW)
    ///
    /// Endpoint number of the interrupt endpoint
    #[inline(always)]
    pub fn write_endpoint(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ENDPOINT_MASK) | ((value << Self::ENDPOINT_OFFSET) & Self::ENDPOINT_MASK)
        });
    }

    /// **ADDRESS** `[6:0]` (RW)
    ///
    /// Device address
    #[inline(always)]
    pub fn address(&self) -> u32 {
        (self.value.read_bits() & Self::ADDRESS_MASK) >> Self::ADDRESS_OFFSET
    }

    /// Write **ADDRESS** `[6:0]` (RW)
    ///
    /// Device address
    #[inline(always)]
    pub fn write_address(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ADDRESS_MASK) | ((value << Self::ADDRESS_OFFSET) & Self::ADDRESS_MASK)
        });
    }
}

/// ADDR_ENDP11 value type for batch read/modify/write operations
///
/// Use `addrendp11.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `addrendp11.write_val(val)`.
#[derive(Clone, Copy)]
pub struct AddrEndp11Val {
    bits: u32,
}

impl AddrEndp11Val {
    /// Create from raw bits
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

    /// Read **INTEP_PREAMBLE** `[26:26]`
    ///
    /// Interrupt EP requires preamble (is a low speed device on a full speed hub)
    #[inline(always)]
    pub const fn intep_preamble(&self) -> bool {
        (self.bits & AddrEndp11::INTEP_PREAMBLE_MASK) != 0
    }

    /// Read **INTEP_DIR** `[25:25]`
    ///
    /// Direction of the interrupt endpoint. In=0, Out=1
    #[inline(always)]
    pub const fn intep_dir(&self) -> bool {
        (self.bits & AddrEndp11::INTEP_DIR_MASK) != 0
    }

    /// Read **ENDPOINT** `[19:16]`
    ///
    /// Endpoint number of the interrupt endpoint
    #[inline(always)]
    pub const fn endpoint(&self) -> u32 {
        (self.bits & AddrEndp11::ENDPOINT_MASK) >> AddrEndp11::ENDPOINT_OFFSET
    }

    /// Read **ADDRESS** `[6:0]`
    ///
    /// Device address
    #[inline(always)]
    pub const fn address(&self) -> u32 {
        (self.bits & AddrEndp11::ADDRESS_MASK) >> AddrEndp11::ADDRESS_OFFSET
    }

    /// Set **INTEP_PREAMBLE** `[26:26]`
    ///
    /// Interrupt EP requires preamble (is a low speed device on a full speed hub)
    #[inline(always)]
    pub const fn with_intep_preamble(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | AddrEndp11::INTEP_PREAMBLE_MASK
            } else {
                self.bits & !AddrEndp11::INTEP_PREAMBLE_MASK
            },
        }
    }

    /// Set **INTEP_DIR** `[25:25]`
    ///
    /// Direction of the interrupt endpoint. In=0, Out=1
    #[inline(always)]
    pub const fn with_intep_dir(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | AddrEndp11::INTEP_DIR_MASK
            } else {
                self.bits & !AddrEndp11::INTEP_DIR_MASK
            },
        }
    }

    /// Set **ENDPOINT** `[19:16]`
    ///
    /// Endpoint number of the interrupt endpoint
    #[inline(always)]
    pub const fn with_endpoint(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AddrEndp11::ENDPOINT_MASK)
                | ((value << AddrEndp11::ENDPOINT_OFFSET) & AddrEndp11::ENDPOINT_MASK),
        }
    }

    /// Set **ADDRESS** `[6:0]`
    ///
    /// Device address
    #[inline(always)]
    pub const fn with_address(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AddrEndp11::ADDRESS_MASK)
                | ((value << AddrEndp11::ADDRESS_OFFSET) & AddrEndp11::ADDRESS_MASK),
        }
    }
}

impl Default for AddrEndp11Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl AddrEndp11 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> AddrEndp11Val {
        AddrEndp11Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: AddrEndp11Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(AddrEndp11Val) -> AddrEndp11Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ADDR_ENDP12 Register
///
/// Interrupt endpoint 12. Only valid for HOST mode.
///
/// Address offset: 0x0030
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **INTEP_PREAMBLE** `[26]` (RW)
///
///   Interrupt EP requires preamble (is a low speed device on a full speed hub)
///
/// - **INTEP_DIR** `[25]` (RW)
///
///   Direction of the interrupt endpoint. In=0, Out=1
///
/// - **ENDPOINT** `[19:16]` (RW)
///
///   Endpoint number of the interrupt endpoint
///
/// - **ADDRESS** `[6:0]` (RW)
///
///   Device address
#[repr(transparent)]
pub struct AddrEndp12 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for AddrEndp12 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl AddrEndp12 {
    // ======== Field Constants ========

    /// **INTEP_PREAMBLE**
    ///
    /// Interrupt EP requires preamble (is a low speed device on a full speed hub)
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const INTEP_PREAMBLE_OFFSET: u32 = 26;
    pub const INTEP_PREAMBLE_MASK: u32 = 0x04000000;
    pub const INTEP_PREAMBLE: u32 = 1 << 26;

    /// **INTEP_DIR**
    ///
    /// Direction of the interrupt endpoint. In=0, Out=1
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const INTEP_DIR_OFFSET: u32 = 25;
    pub const INTEP_DIR_MASK: u32 = 0x02000000;
    pub const INTEP_DIR: u32 = 1 << 25;

    /// **ENDPOINT**
    ///
    /// Endpoint number of the interrupt endpoint
    ///
    /// Bits `[19:16]`, mask `0x000f0000`
    pub const ENDPOINT_OFFSET: u32 = 16;
    pub const ENDPOINT_MASK: u32 = 0x000f0000;

    /// **ADDRESS**
    ///
    /// Device address
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const ADDRESS_OFFSET: u32 = 0;
    pub const ADDRESS_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **INTEP_PREAMBLE** `[26]` (RW)
    ///
    /// Interrupt EP requires preamble (is a low speed device on a full speed hub)
    #[inline(always)]
    pub fn intep_preamble(&self) -> bool {
        (self.value.read_bits() & Self::INTEP_PREAMBLE_MASK) != 0
    }

    /// Set INTEP_PREAMBLE bit atomically (RW)
    #[inline(always)]
    pub fn set_intep_preamble(&self) {
        self.set_bits(Self::INTEP_PREAMBLE_MASK);
    }

    /// Clear INTEP_PREAMBLE bit atomically (RW)
    #[inline(always)]
    pub fn clr_intep_preamble(&self) {
        self.clr_bits(Self::INTEP_PREAMBLE_MASK);
    }

    /// **INTEP_DIR** `[25]` (RW)
    ///
    /// Direction of the interrupt endpoint. In=0, Out=1
    #[inline(always)]
    pub fn intep_dir(&self) -> bool {
        (self.value.read_bits() & Self::INTEP_DIR_MASK) != 0
    }

    /// Set INTEP_DIR bit atomically (RW)
    #[inline(always)]
    pub fn set_intep_dir(&self) {
        self.set_bits(Self::INTEP_DIR_MASK);
    }

    /// Clear INTEP_DIR bit atomically (RW)
    #[inline(always)]
    pub fn clr_intep_dir(&self) {
        self.clr_bits(Self::INTEP_DIR_MASK);
    }

    /// **ENDPOINT** `[19:16]` (RW)
    ///
    /// Endpoint number of the interrupt endpoint
    #[inline(always)]
    pub fn endpoint(&self) -> u32 {
        (self.value.read_bits() & Self::ENDPOINT_MASK) >> Self::ENDPOINT_OFFSET
    }

    /// Write **ENDPOINT** `[19:16]` (RW)
    ///
    /// Endpoint number of the interrupt endpoint
    #[inline(always)]
    pub fn write_endpoint(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ENDPOINT_MASK) | ((value << Self::ENDPOINT_OFFSET) & Self::ENDPOINT_MASK)
        });
    }

    /// **ADDRESS** `[6:0]` (RW)
    ///
    /// Device address
    #[inline(always)]
    pub fn address(&self) -> u32 {
        (self.value.read_bits() & Self::ADDRESS_MASK) >> Self::ADDRESS_OFFSET
    }

    /// Write **ADDRESS** `[6:0]` (RW)
    ///
    /// Device address
    #[inline(always)]
    pub fn write_address(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ADDRESS_MASK) | ((value << Self::ADDRESS_OFFSET) & Self::ADDRESS_MASK)
        });
    }
}

/// ADDR_ENDP12 value type for batch read/modify/write operations
///
/// Use `addrendp12.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `addrendp12.write_val(val)`.
#[derive(Clone, Copy)]
pub struct AddrEndp12Val {
    bits: u32,
}

impl AddrEndp12Val {
    /// Create from raw bits
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

    /// Read **INTEP_PREAMBLE** `[26:26]`
    ///
    /// Interrupt EP requires preamble (is a low speed device on a full speed hub)
    #[inline(always)]
    pub const fn intep_preamble(&self) -> bool {
        (self.bits & AddrEndp12::INTEP_PREAMBLE_MASK) != 0
    }

    /// Read **INTEP_DIR** `[25:25]`
    ///
    /// Direction of the interrupt endpoint. In=0, Out=1
    #[inline(always)]
    pub const fn intep_dir(&self) -> bool {
        (self.bits & AddrEndp12::INTEP_DIR_MASK) != 0
    }

    /// Read **ENDPOINT** `[19:16]`
    ///
    /// Endpoint number of the interrupt endpoint
    #[inline(always)]
    pub const fn endpoint(&self) -> u32 {
        (self.bits & AddrEndp12::ENDPOINT_MASK) >> AddrEndp12::ENDPOINT_OFFSET
    }

    /// Read **ADDRESS** `[6:0]`
    ///
    /// Device address
    #[inline(always)]
    pub const fn address(&self) -> u32 {
        (self.bits & AddrEndp12::ADDRESS_MASK) >> AddrEndp12::ADDRESS_OFFSET
    }

    /// Set **INTEP_PREAMBLE** `[26:26]`
    ///
    /// Interrupt EP requires preamble (is a low speed device on a full speed hub)
    #[inline(always)]
    pub const fn with_intep_preamble(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | AddrEndp12::INTEP_PREAMBLE_MASK
            } else {
                self.bits & !AddrEndp12::INTEP_PREAMBLE_MASK
            },
        }
    }

    /// Set **INTEP_DIR** `[25:25]`
    ///
    /// Direction of the interrupt endpoint. In=0, Out=1
    #[inline(always)]
    pub const fn with_intep_dir(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | AddrEndp12::INTEP_DIR_MASK
            } else {
                self.bits & !AddrEndp12::INTEP_DIR_MASK
            },
        }
    }

    /// Set **ENDPOINT** `[19:16]`
    ///
    /// Endpoint number of the interrupt endpoint
    #[inline(always)]
    pub const fn with_endpoint(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AddrEndp12::ENDPOINT_MASK)
                | ((value << AddrEndp12::ENDPOINT_OFFSET) & AddrEndp12::ENDPOINT_MASK),
        }
    }

    /// Set **ADDRESS** `[6:0]`
    ///
    /// Device address
    #[inline(always)]
    pub const fn with_address(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AddrEndp12::ADDRESS_MASK)
                | ((value << AddrEndp12::ADDRESS_OFFSET) & AddrEndp12::ADDRESS_MASK),
        }
    }
}

impl Default for AddrEndp12Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl AddrEndp12 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> AddrEndp12Val {
        AddrEndp12Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: AddrEndp12Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(AddrEndp12Val) -> AddrEndp12Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ADDR_ENDP13 Register
///
/// Interrupt endpoint 13. Only valid for HOST mode.
///
/// Address offset: 0x0034
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **INTEP_PREAMBLE** `[26]` (RW)
///
///   Interrupt EP requires preamble (is a low speed device on a full speed hub)
///
/// - **INTEP_DIR** `[25]` (RW)
///
///   Direction of the interrupt endpoint. In=0, Out=1
///
/// - **ENDPOINT** `[19:16]` (RW)
///
///   Endpoint number of the interrupt endpoint
///
/// - **ADDRESS** `[6:0]` (RW)
///
///   Device address
#[repr(transparent)]
pub struct AddrEndp13 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for AddrEndp13 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl AddrEndp13 {
    // ======== Field Constants ========

    /// **INTEP_PREAMBLE**
    ///
    /// Interrupt EP requires preamble (is a low speed device on a full speed hub)
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const INTEP_PREAMBLE_OFFSET: u32 = 26;
    pub const INTEP_PREAMBLE_MASK: u32 = 0x04000000;
    pub const INTEP_PREAMBLE: u32 = 1 << 26;

    /// **INTEP_DIR**
    ///
    /// Direction of the interrupt endpoint. In=0, Out=1
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const INTEP_DIR_OFFSET: u32 = 25;
    pub const INTEP_DIR_MASK: u32 = 0x02000000;
    pub const INTEP_DIR: u32 = 1 << 25;

    /// **ENDPOINT**
    ///
    /// Endpoint number of the interrupt endpoint
    ///
    /// Bits `[19:16]`, mask `0x000f0000`
    pub const ENDPOINT_OFFSET: u32 = 16;
    pub const ENDPOINT_MASK: u32 = 0x000f0000;

    /// **ADDRESS**
    ///
    /// Device address
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const ADDRESS_OFFSET: u32 = 0;
    pub const ADDRESS_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **INTEP_PREAMBLE** `[26]` (RW)
    ///
    /// Interrupt EP requires preamble (is a low speed device on a full speed hub)
    #[inline(always)]
    pub fn intep_preamble(&self) -> bool {
        (self.value.read_bits() & Self::INTEP_PREAMBLE_MASK) != 0
    }

    /// Set INTEP_PREAMBLE bit atomically (RW)
    #[inline(always)]
    pub fn set_intep_preamble(&self) {
        self.set_bits(Self::INTEP_PREAMBLE_MASK);
    }

    /// Clear INTEP_PREAMBLE bit atomically (RW)
    #[inline(always)]
    pub fn clr_intep_preamble(&self) {
        self.clr_bits(Self::INTEP_PREAMBLE_MASK);
    }

    /// **INTEP_DIR** `[25]` (RW)
    ///
    /// Direction of the interrupt endpoint. In=0, Out=1
    #[inline(always)]
    pub fn intep_dir(&self) -> bool {
        (self.value.read_bits() & Self::INTEP_DIR_MASK) != 0
    }

    /// Set INTEP_DIR bit atomically (RW)
    #[inline(always)]
    pub fn set_intep_dir(&self) {
        self.set_bits(Self::INTEP_DIR_MASK);
    }

    /// Clear INTEP_DIR bit atomically (RW)
    #[inline(always)]
    pub fn clr_intep_dir(&self) {
        self.clr_bits(Self::INTEP_DIR_MASK);
    }

    /// **ENDPOINT** `[19:16]` (RW)
    ///
    /// Endpoint number of the interrupt endpoint
    #[inline(always)]
    pub fn endpoint(&self) -> u32 {
        (self.value.read_bits() & Self::ENDPOINT_MASK) >> Self::ENDPOINT_OFFSET
    }

    /// Write **ENDPOINT** `[19:16]` (RW)
    ///
    /// Endpoint number of the interrupt endpoint
    #[inline(always)]
    pub fn write_endpoint(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ENDPOINT_MASK) | ((value << Self::ENDPOINT_OFFSET) & Self::ENDPOINT_MASK)
        });
    }

    /// **ADDRESS** `[6:0]` (RW)
    ///
    /// Device address
    #[inline(always)]
    pub fn address(&self) -> u32 {
        (self.value.read_bits() & Self::ADDRESS_MASK) >> Self::ADDRESS_OFFSET
    }

    /// Write **ADDRESS** `[6:0]` (RW)
    ///
    /// Device address
    #[inline(always)]
    pub fn write_address(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ADDRESS_MASK) | ((value << Self::ADDRESS_OFFSET) & Self::ADDRESS_MASK)
        });
    }
}

/// ADDR_ENDP13 value type for batch read/modify/write operations
///
/// Use `addrendp13.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `addrendp13.write_val(val)`.
#[derive(Clone, Copy)]
pub struct AddrEndp13Val {
    bits: u32,
}

impl AddrEndp13Val {
    /// Create from raw bits
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

    /// Read **INTEP_PREAMBLE** `[26:26]`
    ///
    /// Interrupt EP requires preamble (is a low speed device on a full speed hub)
    #[inline(always)]
    pub const fn intep_preamble(&self) -> bool {
        (self.bits & AddrEndp13::INTEP_PREAMBLE_MASK) != 0
    }

    /// Read **INTEP_DIR** `[25:25]`
    ///
    /// Direction of the interrupt endpoint. In=0, Out=1
    #[inline(always)]
    pub const fn intep_dir(&self) -> bool {
        (self.bits & AddrEndp13::INTEP_DIR_MASK) != 0
    }

    /// Read **ENDPOINT** `[19:16]`
    ///
    /// Endpoint number of the interrupt endpoint
    #[inline(always)]
    pub const fn endpoint(&self) -> u32 {
        (self.bits & AddrEndp13::ENDPOINT_MASK) >> AddrEndp13::ENDPOINT_OFFSET
    }

    /// Read **ADDRESS** `[6:0]`
    ///
    /// Device address
    #[inline(always)]
    pub const fn address(&self) -> u32 {
        (self.bits & AddrEndp13::ADDRESS_MASK) >> AddrEndp13::ADDRESS_OFFSET
    }

    /// Set **INTEP_PREAMBLE** `[26:26]`
    ///
    /// Interrupt EP requires preamble (is a low speed device on a full speed hub)
    #[inline(always)]
    pub const fn with_intep_preamble(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | AddrEndp13::INTEP_PREAMBLE_MASK
            } else {
                self.bits & !AddrEndp13::INTEP_PREAMBLE_MASK
            },
        }
    }

    /// Set **INTEP_DIR** `[25:25]`
    ///
    /// Direction of the interrupt endpoint. In=0, Out=1
    #[inline(always)]
    pub const fn with_intep_dir(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | AddrEndp13::INTEP_DIR_MASK
            } else {
                self.bits & !AddrEndp13::INTEP_DIR_MASK
            },
        }
    }

    /// Set **ENDPOINT** `[19:16]`
    ///
    /// Endpoint number of the interrupt endpoint
    #[inline(always)]
    pub const fn with_endpoint(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AddrEndp13::ENDPOINT_MASK)
                | ((value << AddrEndp13::ENDPOINT_OFFSET) & AddrEndp13::ENDPOINT_MASK),
        }
    }

    /// Set **ADDRESS** `[6:0]`
    ///
    /// Device address
    #[inline(always)]
    pub const fn with_address(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AddrEndp13::ADDRESS_MASK)
                | ((value << AddrEndp13::ADDRESS_OFFSET) & AddrEndp13::ADDRESS_MASK),
        }
    }
}

impl Default for AddrEndp13Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl AddrEndp13 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> AddrEndp13Val {
        AddrEndp13Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: AddrEndp13Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(AddrEndp13Val) -> AddrEndp13Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ADDR_ENDP14 Register
///
/// Interrupt endpoint 14. Only valid for HOST mode.
///
/// Address offset: 0x0038
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **INTEP_PREAMBLE** `[26]` (RW)
///
///   Interrupt EP requires preamble (is a low speed device on a full speed hub)
///
/// - **INTEP_DIR** `[25]` (RW)
///
///   Direction of the interrupt endpoint. In=0, Out=1
///
/// - **ENDPOINT** `[19:16]` (RW)
///
///   Endpoint number of the interrupt endpoint
///
/// - **ADDRESS** `[6:0]` (RW)
///
///   Device address
#[repr(transparent)]
pub struct AddrEndp14 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for AddrEndp14 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl AddrEndp14 {
    // ======== Field Constants ========

    /// **INTEP_PREAMBLE**
    ///
    /// Interrupt EP requires preamble (is a low speed device on a full speed hub)
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const INTEP_PREAMBLE_OFFSET: u32 = 26;
    pub const INTEP_PREAMBLE_MASK: u32 = 0x04000000;
    pub const INTEP_PREAMBLE: u32 = 1 << 26;

    /// **INTEP_DIR**
    ///
    /// Direction of the interrupt endpoint. In=0, Out=1
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const INTEP_DIR_OFFSET: u32 = 25;
    pub const INTEP_DIR_MASK: u32 = 0x02000000;
    pub const INTEP_DIR: u32 = 1 << 25;

    /// **ENDPOINT**
    ///
    /// Endpoint number of the interrupt endpoint
    ///
    /// Bits `[19:16]`, mask `0x000f0000`
    pub const ENDPOINT_OFFSET: u32 = 16;
    pub const ENDPOINT_MASK: u32 = 0x000f0000;

    /// **ADDRESS**
    ///
    /// Device address
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const ADDRESS_OFFSET: u32 = 0;
    pub const ADDRESS_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **INTEP_PREAMBLE** `[26]` (RW)
    ///
    /// Interrupt EP requires preamble (is a low speed device on a full speed hub)
    #[inline(always)]
    pub fn intep_preamble(&self) -> bool {
        (self.value.read_bits() & Self::INTEP_PREAMBLE_MASK) != 0
    }

    /// Set INTEP_PREAMBLE bit atomically (RW)
    #[inline(always)]
    pub fn set_intep_preamble(&self) {
        self.set_bits(Self::INTEP_PREAMBLE_MASK);
    }

    /// Clear INTEP_PREAMBLE bit atomically (RW)
    #[inline(always)]
    pub fn clr_intep_preamble(&self) {
        self.clr_bits(Self::INTEP_PREAMBLE_MASK);
    }

    /// **INTEP_DIR** `[25]` (RW)
    ///
    /// Direction of the interrupt endpoint. In=0, Out=1
    #[inline(always)]
    pub fn intep_dir(&self) -> bool {
        (self.value.read_bits() & Self::INTEP_DIR_MASK) != 0
    }

    /// Set INTEP_DIR bit atomically (RW)
    #[inline(always)]
    pub fn set_intep_dir(&self) {
        self.set_bits(Self::INTEP_DIR_MASK);
    }

    /// Clear INTEP_DIR bit atomically (RW)
    #[inline(always)]
    pub fn clr_intep_dir(&self) {
        self.clr_bits(Self::INTEP_DIR_MASK);
    }

    /// **ENDPOINT** `[19:16]` (RW)
    ///
    /// Endpoint number of the interrupt endpoint
    #[inline(always)]
    pub fn endpoint(&self) -> u32 {
        (self.value.read_bits() & Self::ENDPOINT_MASK) >> Self::ENDPOINT_OFFSET
    }

    /// Write **ENDPOINT** `[19:16]` (RW)
    ///
    /// Endpoint number of the interrupt endpoint
    #[inline(always)]
    pub fn write_endpoint(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ENDPOINT_MASK) | ((value << Self::ENDPOINT_OFFSET) & Self::ENDPOINT_MASK)
        });
    }

    /// **ADDRESS** `[6:0]` (RW)
    ///
    /// Device address
    #[inline(always)]
    pub fn address(&self) -> u32 {
        (self.value.read_bits() & Self::ADDRESS_MASK) >> Self::ADDRESS_OFFSET
    }

    /// Write **ADDRESS** `[6:0]` (RW)
    ///
    /// Device address
    #[inline(always)]
    pub fn write_address(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ADDRESS_MASK) | ((value << Self::ADDRESS_OFFSET) & Self::ADDRESS_MASK)
        });
    }
}

/// ADDR_ENDP14 value type for batch read/modify/write operations
///
/// Use `addrendp14.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `addrendp14.write_val(val)`.
#[derive(Clone, Copy)]
pub struct AddrEndp14Val {
    bits: u32,
}

impl AddrEndp14Val {
    /// Create from raw bits
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

    /// Read **INTEP_PREAMBLE** `[26:26]`
    ///
    /// Interrupt EP requires preamble (is a low speed device on a full speed hub)
    #[inline(always)]
    pub const fn intep_preamble(&self) -> bool {
        (self.bits & AddrEndp14::INTEP_PREAMBLE_MASK) != 0
    }

    /// Read **INTEP_DIR** `[25:25]`
    ///
    /// Direction of the interrupt endpoint. In=0, Out=1
    #[inline(always)]
    pub const fn intep_dir(&self) -> bool {
        (self.bits & AddrEndp14::INTEP_DIR_MASK) != 0
    }

    /// Read **ENDPOINT** `[19:16]`
    ///
    /// Endpoint number of the interrupt endpoint
    #[inline(always)]
    pub const fn endpoint(&self) -> u32 {
        (self.bits & AddrEndp14::ENDPOINT_MASK) >> AddrEndp14::ENDPOINT_OFFSET
    }

    /// Read **ADDRESS** `[6:0]`
    ///
    /// Device address
    #[inline(always)]
    pub const fn address(&self) -> u32 {
        (self.bits & AddrEndp14::ADDRESS_MASK) >> AddrEndp14::ADDRESS_OFFSET
    }

    /// Set **INTEP_PREAMBLE** `[26:26]`
    ///
    /// Interrupt EP requires preamble (is a low speed device on a full speed hub)
    #[inline(always)]
    pub const fn with_intep_preamble(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | AddrEndp14::INTEP_PREAMBLE_MASK
            } else {
                self.bits & !AddrEndp14::INTEP_PREAMBLE_MASK
            },
        }
    }

    /// Set **INTEP_DIR** `[25:25]`
    ///
    /// Direction of the interrupt endpoint. In=0, Out=1
    #[inline(always)]
    pub const fn with_intep_dir(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | AddrEndp14::INTEP_DIR_MASK
            } else {
                self.bits & !AddrEndp14::INTEP_DIR_MASK
            },
        }
    }

    /// Set **ENDPOINT** `[19:16]`
    ///
    /// Endpoint number of the interrupt endpoint
    #[inline(always)]
    pub const fn with_endpoint(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AddrEndp14::ENDPOINT_MASK)
                | ((value << AddrEndp14::ENDPOINT_OFFSET) & AddrEndp14::ENDPOINT_MASK),
        }
    }

    /// Set **ADDRESS** `[6:0]`
    ///
    /// Device address
    #[inline(always)]
    pub const fn with_address(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AddrEndp14::ADDRESS_MASK)
                | ((value << AddrEndp14::ADDRESS_OFFSET) & AddrEndp14::ADDRESS_MASK),
        }
    }
}

impl Default for AddrEndp14Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl AddrEndp14 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> AddrEndp14Val {
        AddrEndp14Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: AddrEndp14Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(AddrEndp14Val) -> AddrEndp14Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ADDR_ENDP15 Register
///
/// Interrupt endpoint 15. Only valid for HOST mode.
///
/// Address offset: 0x003c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **INTEP_PREAMBLE** `[26]` (RW)
///
///   Interrupt EP requires preamble (is a low speed device on a full speed hub)
///
/// - **INTEP_DIR** `[25]` (RW)
///
///   Direction of the interrupt endpoint. In=0, Out=1
///
/// - **ENDPOINT** `[19:16]` (RW)
///
///   Endpoint number of the interrupt endpoint
///
/// - **ADDRESS** `[6:0]` (RW)
///
///   Device address
#[repr(transparent)]
pub struct AddrEndp15 {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for AddrEndp15 {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl AddrEndp15 {
    // ======== Field Constants ========

    /// **INTEP_PREAMBLE**
    ///
    /// Interrupt EP requires preamble (is a low speed device on a full speed hub)
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const INTEP_PREAMBLE_OFFSET: u32 = 26;
    pub const INTEP_PREAMBLE_MASK: u32 = 0x04000000;
    pub const INTEP_PREAMBLE: u32 = 1 << 26;

    /// **INTEP_DIR**
    ///
    /// Direction of the interrupt endpoint. In=0, Out=1
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const INTEP_DIR_OFFSET: u32 = 25;
    pub const INTEP_DIR_MASK: u32 = 0x02000000;
    pub const INTEP_DIR: u32 = 1 << 25;

    /// **ENDPOINT**
    ///
    /// Endpoint number of the interrupt endpoint
    ///
    /// Bits `[19:16]`, mask `0x000f0000`
    pub const ENDPOINT_OFFSET: u32 = 16;
    pub const ENDPOINT_MASK: u32 = 0x000f0000;

    /// **ADDRESS**
    ///
    /// Device address
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const ADDRESS_OFFSET: u32 = 0;
    pub const ADDRESS_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **INTEP_PREAMBLE** `[26]` (RW)
    ///
    /// Interrupt EP requires preamble (is a low speed device on a full speed hub)
    #[inline(always)]
    pub fn intep_preamble(&self) -> bool {
        (self.value.read_bits() & Self::INTEP_PREAMBLE_MASK) != 0
    }

    /// Set INTEP_PREAMBLE bit atomically (RW)
    #[inline(always)]
    pub fn set_intep_preamble(&self) {
        self.set_bits(Self::INTEP_PREAMBLE_MASK);
    }

    /// Clear INTEP_PREAMBLE bit atomically (RW)
    #[inline(always)]
    pub fn clr_intep_preamble(&self) {
        self.clr_bits(Self::INTEP_PREAMBLE_MASK);
    }

    /// **INTEP_DIR** `[25]` (RW)
    ///
    /// Direction of the interrupt endpoint. In=0, Out=1
    #[inline(always)]
    pub fn intep_dir(&self) -> bool {
        (self.value.read_bits() & Self::INTEP_DIR_MASK) != 0
    }

    /// Set INTEP_DIR bit atomically (RW)
    #[inline(always)]
    pub fn set_intep_dir(&self) {
        self.set_bits(Self::INTEP_DIR_MASK);
    }

    /// Clear INTEP_DIR bit atomically (RW)
    #[inline(always)]
    pub fn clr_intep_dir(&self) {
        self.clr_bits(Self::INTEP_DIR_MASK);
    }

    /// **ENDPOINT** `[19:16]` (RW)
    ///
    /// Endpoint number of the interrupt endpoint
    #[inline(always)]
    pub fn endpoint(&self) -> u32 {
        (self.value.read_bits() & Self::ENDPOINT_MASK) >> Self::ENDPOINT_OFFSET
    }

    /// Write **ENDPOINT** `[19:16]` (RW)
    ///
    /// Endpoint number of the interrupt endpoint
    #[inline(always)]
    pub fn write_endpoint(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ENDPOINT_MASK) | ((value << Self::ENDPOINT_OFFSET) & Self::ENDPOINT_MASK)
        });
    }

    /// **ADDRESS** `[6:0]` (RW)
    ///
    /// Device address
    #[inline(always)]
    pub fn address(&self) -> u32 {
        (self.value.read_bits() & Self::ADDRESS_MASK) >> Self::ADDRESS_OFFSET
    }

    /// Write **ADDRESS** `[6:0]` (RW)
    ///
    /// Device address
    #[inline(always)]
    pub fn write_address(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ADDRESS_MASK) | ((value << Self::ADDRESS_OFFSET) & Self::ADDRESS_MASK)
        });
    }
}

/// ADDR_ENDP15 value type for batch read/modify/write operations
///
/// Use `addrendp15.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `addrendp15.write_val(val)`.
#[derive(Clone, Copy)]
pub struct AddrEndp15Val {
    bits: u32,
}

impl AddrEndp15Val {
    /// Create from raw bits
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

    /// Read **INTEP_PREAMBLE** `[26:26]`
    ///
    /// Interrupt EP requires preamble (is a low speed device on a full speed hub)
    #[inline(always)]
    pub const fn intep_preamble(&self) -> bool {
        (self.bits & AddrEndp15::INTEP_PREAMBLE_MASK) != 0
    }

    /// Read **INTEP_DIR** `[25:25]`
    ///
    /// Direction of the interrupt endpoint. In=0, Out=1
    #[inline(always)]
    pub const fn intep_dir(&self) -> bool {
        (self.bits & AddrEndp15::INTEP_DIR_MASK) != 0
    }

    /// Read **ENDPOINT** `[19:16]`
    ///
    /// Endpoint number of the interrupt endpoint
    #[inline(always)]
    pub const fn endpoint(&self) -> u32 {
        (self.bits & AddrEndp15::ENDPOINT_MASK) >> AddrEndp15::ENDPOINT_OFFSET
    }

    /// Read **ADDRESS** `[6:0]`
    ///
    /// Device address
    #[inline(always)]
    pub const fn address(&self) -> u32 {
        (self.bits & AddrEndp15::ADDRESS_MASK) >> AddrEndp15::ADDRESS_OFFSET
    }

    /// Set **INTEP_PREAMBLE** `[26:26]`
    ///
    /// Interrupt EP requires preamble (is a low speed device on a full speed hub)
    #[inline(always)]
    pub const fn with_intep_preamble(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | AddrEndp15::INTEP_PREAMBLE_MASK
            } else {
                self.bits & !AddrEndp15::INTEP_PREAMBLE_MASK
            },
        }
    }

    /// Set **INTEP_DIR** `[25:25]`
    ///
    /// Direction of the interrupt endpoint. In=0, Out=1
    #[inline(always)]
    pub const fn with_intep_dir(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | AddrEndp15::INTEP_DIR_MASK
            } else {
                self.bits & !AddrEndp15::INTEP_DIR_MASK
            },
        }
    }

    /// Set **ENDPOINT** `[19:16]`
    ///
    /// Endpoint number of the interrupt endpoint
    #[inline(always)]
    pub const fn with_endpoint(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AddrEndp15::ENDPOINT_MASK)
                | ((value << AddrEndp15::ENDPOINT_OFFSET) & AddrEndp15::ENDPOINT_MASK),
        }
    }

    /// Set **ADDRESS** `[6:0]`
    ///
    /// Device address
    #[inline(always)]
    pub const fn with_address(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AddrEndp15::ADDRESS_MASK)
                | ((value << AddrEndp15::ADDRESS_OFFSET) & AddrEndp15::ADDRESS_MASK),
        }
    }
}

impl Default for AddrEndp15Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl AddrEndp15 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> AddrEndp15Val {
        AddrEndp15Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: AddrEndp15Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(AddrEndp15Val) -> AddrEndp15Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// MAIN_CTRL Register
///
/// Main control register
///
/// Address offset: 0x0040
/// Reset value: 0x00000004
///
/// ## Fields
///
/// - **SIM_TIMING** `[31]` (RW)
///
///   Reduced timings for simulation
///
/// - **PHY_ISO** `[2]` (RW)
///
///   Isolates USB phy after controller power-up
///   Remove isolation once software has configured the controller
///   Not isolated = 0, Isolated = 1
///
/// - **HOST_NDEVICE** `[1]` (RW)
///
///   Device mode = 0, Host mode = 1
///
/// - **CONTROLLER_EN** `[0]` (RW)
///
///   Enable controller
#[repr(transparent)]
pub struct MainCtrl {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for MainCtrl {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MainCtrl {
    // ======== Field Constants ========

    /// **SIM_TIMING**
    ///
    /// Reduced timings for simulation
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const SIM_TIMING_OFFSET: u32 = 31;
    pub const SIM_TIMING_MASK: u32 = 0x80000000;
    pub const SIM_TIMING: u32 = 1 << 31;

    /// **PHY_ISO**
    ///
    /// Isolates USB phy after controller power-up
    /// Remove isolation once software has configured the controller
    /// Not isolated = 0, Isolated = 1
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const PHY_ISO_OFFSET: u32 = 2;
    pub const PHY_ISO_MASK: u32 = 0x00000004;
    pub const PHY_ISO: u32 = 1 << 2;

    /// **HOST_NDEVICE**
    ///
    /// Device mode = 0, Host mode = 1
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const HOST_NDEVICE_OFFSET: u32 = 1;
    pub const HOST_NDEVICE_MASK: u32 = 0x00000002;
    pub const HOST_NDEVICE: u32 = 1 << 1;

    /// **CONTROLLER_EN**
    ///
    /// Enable controller
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CONTROLLER_EN_OFFSET: u32 = 0;
    pub const CONTROLLER_EN_MASK: u32 = 0x00000001;
    pub const CONTROLLER_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **SIM_TIMING** `[31]` (RW)
    ///
    /// Reduced timings for simulation
    #[inline(always)]
    pub fn sim_timing(&self) -> bool {
        (self.value.read_bits() & Self::SIM_TIMING_MASK) != 0
    }

    /// Set SIM_TIMING bit atomically (RW)
    #[inline(always)]
    pub fn set_sim_timing(&self) {
        self.set_bits(Self::SIM_TIMING_MASK);
    }

    /// Clear SIM_TIMING bit atomically (RW)
    #[inline(always)]
    pub fn clr_sim_timing(&self) {
        self.clr_bits(Self::SIM_TIMING_MASK);
    }

    /// **PHY_ISO** `[2]` (RW)
    ///
    /// Isolates USB phy after controller power-up
    /// Remove isolation once software has configured the controller
    /// Not isolated = 0, Isolated = 1
    #[inline(always)]
    pub fn phy_iso(&self) -> bool {
        (self.value.read_bits() & Self::PHY_ISO_MASK) != 0
    }

    /// Set PHY_ISO bit atomically (RW)
    #[inline(always)]
    pub fn set_phy_iso(&self) {
        self.set_bits(Self::PHY_ISO_MASK);
    }

    /// Clear PHY_ISO bit atomically (RW)
    #[inline(always)]
    pub fn clr_phy_iso(&self) {
        self.clr_bits(Self::PHY_ISO_MASK);
    }

    /// **HOST_NDEVICE** `[1]` (RW)
    ///
    /// Device mode = 0, Host mode = 1
    #[inline(always)]
    pub fn host_ndevice(&self) -> bool {
        (self.value.read_bits() & Self::HOST_NDEVICE_MASK) != 0
    }

    /// Set HOST_NDEVICE bit atomically (RW)
    #[inline(always)]
    pub fn set_host_ndevice(&self) {
        self.set_bits(Self::HOST_NDEVICE_MASK);
    }

    /// Clear HOST_NDEVICE bit atomically (RW)
    #[inline(always)]
    pub fn clr_host_ndevice(&self) {
        self.clr_bits(Self::HOST_NDEVICE_MASK);
    }

    /// **CONTROLLER_EN** `[0]` (RW)
    ///
    /// Enable controller
    #[inline(always)]
    pub fn controller_en(&self) -> bool {
        (self.value.read_bits() & Self::CONTROLLER_EN_MASK) != 0
    }

    /// Set CONTROLLER_EN bit atomically (RW)
    #[inline(always)]
    pub fn set_controller_en(&self) {
        self.set_bits(Self::CONTROLLER_EN_MASK);
    }

    /// Clear CONTROLLER_EN bit atomically (RW)
    #[inline(always)]
    pub fn clr_controller_en(&self) {
        self.clr_bits(Self::CONTROLLER_EN_MASK);
    }
}

/// MAIN_CTRL value type for batch read/modify/write operations
///
/// Use `mainctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `mainctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct MainCtrlVal {
    bits: u32,
}

impl MainCtrlVal {
    /// Create from raw bits
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

    /// Read **SIM_TIMING** `[31:31]`
    ///
    /// Reduced timings for simulation
    #[inline(always)]
    pub const fn sim_timing(&self) -> bool {
        (self.bits & MainCtrl::SIM_TIMING_MASK) != 0
    }

    /// Read **PHY_ISO** `[2:2]`
    ///
    /// Isolates USB phy after controller power-up
    /// Remove isolation once software has configured the controller
    /// Not isolated = 0, Isolated = 1
    #[inline(always)]
    pub const fn phy_iso(&self) -> bool {
        (self.bits & MainCtrl::PHY_ISO_MASK) != 0
    }

    /// Read **HOST_NDEVICE** `[1:1]`
    ///
    /// Device mode = 0, Host mode = 1
    #[inline(always)]
    pub const fn host_ndevice(&self) -> bool {
        (self.bits & MainCtrl::HOST_NDEVICE_MASK) != 0
    }

    /// Read **CONTROLLER_EN** `[0:0]`
    ///
    /// Enable controller
    #[inline(always)]
    pub const fn controller_en(&self) -> bool {
        (self.bits & MainCtrl::CONTROLLER_EN_MASK) != 0
    }

    /// Set **SIM_TIMING** `[31:31]`
    ///
    /// Reduced timings for simulation
    #[inline(always)]
    pub const fn with_sim_timing(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | MainCtrl::SIM_TIMING_MASK
            } else {
                self.bits & !MainCtrl::SIM_TIMING_MASK
            },
        }
    }

    /// Set **PHY_ISO** `[2:2]`
    ///
    /// Isolates USB phy after controller power-up
    /// Remove isolation once software has configured the controller
    /// Not isolated = 0, Isolated = 1
    #[inline(always)]
    pub const fn with_phy_iso(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | MainCtrl::PHY_ISO_MASK
            } else {
                self.bits & !MainCtrl::PHY_ISO_MASK
            },
        }
    }

    /// Set **HOST_NDEVICE** `[1:1]`
    ///
    /// Device mode = 0, Host mode = 1
    #[inline(always)]
    pub const fn with_host_ndevice(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | MainCtrl::HOST_NDEVICE_MASK
            } else {
                self.bits & !MainCtrl::HOST_NDEVICE_MASK
            },
        }
    }

    /// Set **CONTROLLER_EN** `[0:0]`
    ///
    /// Enable controller
    #[inline(always)]
    pub const fn with_controller_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | MainCtrl::CONTROLLER_EN_MASK
            } else {
                self.bits & !MainCtrl::CONTROLLER_EN_MASK
            },
        }
    }
}

impl Default for MainCtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl MainCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> MainCtrlVal {
        MainCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: MainCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(MainCtrlVal) -> MainCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SOF_WR Register
///
/// Set the SOF (Start of Frame) frame number in the host controller. The SOF packet is sent every 1ms and the host will increment the frame number by 1 each time.
///
/// Address offset: 0x0044
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **COUNT** `[10:0]` (WO)
#[repr(transparent)]
pub struct SofWr {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for SofWr {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SofWr {
    // ======== Field Constants ========

    /// **COUNT**
    /// Bits `[10:0]`, mask `0x000007ff`
    pub const COUNT_OFFSET: u32 = 0;
    pub const COUNT_MASK: u32 = 0x000007ff;

    // ======== Field Accessors ========

    /// Write **COUNT** `[10:0]` (WO)
    #[inline(always)]
    pub fn write_count(&self, value: u32) {
        self.value
            .write_bits((value << Self::COUNT_OFFSET) & Self::COUNT_MASK);
    }
}

/// SOF_WR value type for batch read/modify/write operations
///
/// Use `sofwr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sofwr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SofWrVal {
    bits: u32,
}

impl SofWrVal {
    /// Create from raw bits
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

    /// Set **COUNT** `[10:0]`
    #[inline(always)]
    pub const fn with_count(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SofWr::COUNT_MASK)
                | ((value << SofWr::COUNT_OFFSET) & SofWr::COUNT_MASK),
        }
    }
}

impl Default for SofWrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SofWr {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SofWrVal) {
        self.value.write_bits(val.bits())
    }
}

/// SOF_RD Register
///
/// Read the last SOF (Start of Frame) frame number seen. In device mode the last SOF received from the host. In host mode the last SOF sent by the host.
///
/// Address offset: 0x0048
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **COUNT** `[10:0]` (RO)
#[repr(transparent)]
pub struct SofRd {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SofRd {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SofRd {
    // ======== Field Constants ========

    /// **COUNT**
    /// Bits `[10:0]`, mask `0x000007ff`
    pub const COUNT_OFFSET: u32 = 0;
    pub const COUNT_MASK: u32 = 0x000007ff;

    // ======== Field Accessors ========

    /// **COUNT** `[10:0]` (RO)
    #[inline(always)]
    pub fn count(&self) -> u32 {
        (self.value.read_bits() & Self::COUNT_MASK) >> Self::COUNT_OFFSET
    }
}

/// SOF_RD value type for batch read/modify/write operations
///
/// Use `sofrd.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sofrd.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SofRdVal {
    bits: u32,
}

impl SofRdVal {
    /// Create from raw bits
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

    /// Read **COUNT** `[10:0]`
    #[inline(always)]
    pub const fn count(&self) -> u32 {
        (self.bits & SofRd::COUNT_MASK) >> SofRd::COUNT_OFFSET
    }
}

impl Default for SofRdVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SofRd {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SofRdVal {
        SofRdVal::from_bits(self.value.read_bits())
    }
}

/// SIE_CTRL Register
///
/// SIE control register
///
/// Address offset: 0x004c
/// Reset value: 0x00008000
///
/// ## Fields
///
/// - **EP0_INT_STALL** `[31]` (RW)
///
///   Device: Set bit in EP_STATUS_STALL_NAK when EP0 sends a STALL
///
/// - **EP0_DOUBLE_BUF** `[30]` (RW)
///
///   Device: EP0 single buffered = 0, double buffered = 1
///
/// - **EP0_INT_1BUF** `[29]` (RW)
///
///   Device: Set bit in BUFF_STATUS for every buffer completed on EP0
///
/// - **EP0_INT_2BUF** `[28]` (RW)
///
///   Device: Set bit in BUFF_STATUS for every 2 buffers completed on EP0
///
/// - **EP0_INT_NAK** `[27]` (RW)
///
///   Device: Set bit in EP_STATUS_STALL_NAK when EP0 sends a NAK
///
/// - **DIRECT_EN** `[26]` (RW)
///
///   Direct bus drive enable
///
/// - **DIRECT_DP** `[25]` (RW)
///
///   Direct control of DP
///
/// - **DIRECT_DM** `[24]` (RW)
///
///   Direct control of DM
///
/// - **EP0_STOP_ON_SHORT_PACKET** `[19]` (RW)
///
///   Device: Stop EP0 on a short packet.
///
/// - **TRANSCEIVER_PD** `[18]` (RW)
///
///   Power down bus transceiver
///
/// - **RPU_OPT** `[17]` (RW)
///
///   Device: Pull-up strength (0=1K2, 1=2k3)
///
/// - **PULLUP_EN** `[16]` (RW)
///
///   Device: Enable pull up resistor
///
/// - **PULLDOWN_EN** `[15]` (RW)
///
///   Host: Enable pull down resistors
///
/// - **RESET_BUS** `[13]` (WO)
///
///   Host: Reset bus
///
/// - **RESUME** `[12]` (WO)
///
///   Device: Remote wakeup. Device can initiate its own resume after suspend.
///
/// - **VBUS_EN** `[11]` (RW)
///
///   Host: Enable VBUS
///
/// - **KEEP_ALIVE_EN** `[10]` (RW)
///
///   Host: Enable keep alive packet (for low speed bus)
///
/// - **SOF_EN** `[9]` (RW)
///
///   Host: Enable SOF generation (for full speed bus)
///
/// - **SOF_SYNC** `[8]` (RW)
///
///   Host: Delay packet(s) until after SOF
///
/// - **PREAMBLE_EN** `[6]` (RW)
///
///   Host: Preable enable for LS device on FS hub
///
/// - **STOP_TRANS** `[4]` (WO)
///
///   Host: Stop transaction
///
/// - **RECEIVE_DATA** `[3]` (RW)
///
///   Host: Receive transaction (IN to host)
///
/// - **SEND_DATA** `[2]` (RW)
///
///   Host: Send transaction (OUT from host)
///
/// - **SEND_SETUP** `[1]` (RW)
///
///   Host: Send Setup packet
///
/// - **START_TRANS** `[0]` (WO)
///
///   Host: Start transaction
#[repr(transparent)]
pub struct SieCtrl {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for SieCtrl {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SieCtrl {
    // ======== Field Constants ========

    /// **EP0_INT_STALL**
    ///
    /// Device: Set bit in EP_STATUS_STALL_NAK when EP0 sends a STALL
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const EP0_INT_STALL_OFFSET: u32 = 31;
    pub const EP0_INT_STALL_MASK: u32 = 0x80000000;
    pub const EP0_INT_STALL: u32 = 1 << 31;

    /// **EP0_DOUBLE_BUF**
    ///
    /// Device: EP0 single buffered = 0, double buffered = 1
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const EP0_DOUBLE_BUF_OFFSET: u32 = 30;
    pub const EP0_DOUBLE_BUF_MASK: u32 = 0x40000000;
    pub const EP0_DOUBLE_BUF: u32 = 1 << 30;

    /// **EP0_INT_1BUF**
    ///
    /// Device: Set bit in BUFF_STATUS for every buffer completed on EP0
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const EP0_INT_1BUF_OFFSET: u32 = 29;
    pub const EP0_INT_1BUF_MASK: u32 = 0x20000000;
    pub const EP0_INT_1BUF: u32 = 1 << 29;

    /// **EP0_INT_2BUF**
    ///
    /// Device: Set bit in BUFF_STATUS for every 2 buffers completed on EP0
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const EP0_INT_2BUF_OFFSET: u32 = 28;
    pub const EP0_INT_2BUF_MASK: u32 = 0x10000000;
    pub const EP0_INT_2BUF: u32 = 1 << 28;

    /// **EP0_INT_NAK**
    ///
    /// Device: Set bit in EP_STATUS_STALL_NAK when EP0 sends a NAK
    ///
    /// Bits `[27:27]`, mask `0x08000000`
    pub const EP0_INT_NAK_OFFSET: u32 = 27;
    pub const EP0_INT_NAK_MASK: u32 = 0x08000000;
    pub const EP0_INT_NAK: u32 = 1 << 27;

    /// **DIRECT_EN**
    ///
    /// Direct bus drive enable
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const DIRECT_EN_OFFSET: u32 = 26;
    pub const DIRECT_EN_MASK: u32 = 0x04000000;
    pub const DIRECT_EN: u32 = 1 << 26;

    /// **DIRECT_DP**
    ///
    /// Direct control of DP
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const DIRECT_DP_OFFSET: u32 = 25;
    pub const DIRECT_DP_MASK: u32 = 0x02000000;
    pub const DIRECT_DP: u32 = 1 << 25;

    /// **DIRECT_DM**
    ///
    /// Direct control of DM
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const DIRECT_DM_OFFSET: u32 = 24;
    pub const DIRECT_DM_MASK: u32 = 0x01000000;
    pub const DIRECT_DM: u32 = 1 << 24;

    /// **EP0_STOP_ON_SHORT_PACKET**
    ///
    /// Device: Stop EP0 on a short packet.
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const EP0_STOP_ON_SHORT_PACKET_OFFSET: u32 = 19;
    pub const EP0_STOP_ON_SHORT_PACKET_MASK: u32 = 0x00080000;
    pub const EP0_STOP_ON_SHORT_PACKET: u32 = 1 << 19;

    /// **TRANSCEIVER_PD**
    ///
    /// Power down bus transceiver
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const TRANSCEIVER_PD_OFFSET: u32 = 18;
    pub const TRANSCEIVER_PD_MASK: u32 = 0x00040000;
    pub const TRANSCEIVER_PD: u32 = 1 << 18;

    /// **RPU_OPT**
    ///
    /// Device: Pull-up strength (0=1K2, 1=2k3)
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const RPU_OPT_OFFSET: u32 = 17;
    pub const RPU_OPT_MASK: u32 = 0x00020000;
    pub const RPU_OPT: u32 = 1 << 17;

    /// **PULLUP_EN**
    ///
    /// Device: Enable pull up resistor
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const PULLUP_EN_OFFSET: u32 = 16;
    pub const PULLUP_EN_MASK: u32 = 0x00010000;
    pub const PULLUP_EN: u32 = 1 << 16;

    /// **PULLDOWN_EN**
    ///
    /// Host: Enable pull down resistors
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const PULLDOWN_EN_OFFSET: u32 = 15;
    pub const PULLDOWN_EN_MASK: u32 = 0x00008000;
    pub const PULLDOWN_EN: u32 = 1 << 15;

    /// **RESET_BUS**
    ///
    /// Host: Reset bus
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const RESET_BUS_OFFSET: u32 = 13;
    pub const RESET_BUS_MASK: u32 = 0x00002000;
    pub const RESET_BUS: u32 = 1 << 13;

    /// **RESUME**
    ///
    /// Device: Remote wakeup. Device can initiate its own resume after suspend.
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const RESUME_OFFSET: u32 = 12;
    pub const RESUME_MASK: u32 = 0x00001000;
    pub const RESUME: u32 = 1 << 12;

    /// **VBUS_EN**
    ///
    /// Host: Enable VBUS
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const VBUS_EN_OFFSET: u32 = 11;
    pub const VBUS_EN_MASK: u32 = 0x00000800;
    pub const VBUS_EN: u32 = 1 << 11;

    /// **KEEP_ALIVE_EN**
    ///
    /// Host: Enable keep alive packet (for low speed bus)
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const KEEP_ALIVE_EN_OFFSET: u32 = 10;
    pub const KEEP_ALIVE_EN_MASK: u32 = 0x00000400;
    pub const KEEP_ALIVE_EN: u32 = 1 << 10;

    /// **SOF_EN**
    ///
    /// Host: Enable SOF generation (for full speed bus)
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const SOF_EN_OFFSET: u32 = 9;
    pub const SOF_EN_MASK: u32 = 0x00000200;
    pub const SOF_EN: u32 = 1 << 9;

    /// **SOF_SYNC**
    ///
    /// Host: Delay packet(s) until after SOF
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const SOF_SYNC_OFFSET: u32 = 8;
    pub const SOF_SYNC_MASK: u32 = 0x00000100;
    pub const SOF_SYNC: u32 = 1 << 8;

    /// **PREAMBLE_EN**
    ///
    /// Host: Preable enable for LS device on FS hub
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const PREAMBLE_EN_OFFSET: u32 = 6;
    pub const PREAMBLE_EN_MASK: u32 = 0x00000040;
    pub const PREAMBLE_EN: u32 = 1 << 6;

    /// **STOP_TRANS**
    ///
    /// Host: Stop transaction
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const STOP_TRANS_OFFSET: u32 = 4;
    pub const STOP_TRANS_MASK: u32 = 0x00000010;
    pub const STOP_TRANS: u32 = 1 << 4;

    /// **RECEIVE_DATA**
    ///
    /// Host: Receive transaction (IN to host)
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const RECEIVE_DATA_OFFSET: u32 = 3;
    pub const RECEIVE_DATA_MASK: u32 = 0x00000008;
    pub const RECEIVE_DATA: u32 = 1 << 3;

    /// **SEND_DATA**
    ///
    /// Host: Send transaction (OUT from host)
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SEND_DATA_OFFSET: u32 = 2;
    pub const SEND_DATA_MASK: u32 = 0x00000004;
    pub const SEND_DATA: u32 = 1 << 2;

    /// **SEND_SETUP**
    ///
    /// Host: Send Setup packet
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SEND_SETUP_OFFSET: u32 = 1;
    pub const SEND_SETUP_MASK: u32 = 0x00000002;
    pub const SEND_SETUP: u32 = 1 << 1;

    /// **START_TRANS**
    ///
    /// Host: Start transaction
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const START_TRANS_OFFSET: u32 = 0;
    pub const START_TRANS_MASK: u32 = 0x00000001;
    pub const START_TRANS: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **EP0_INT_STALL** `[31]` (RW)
    ///
    /// Device: Set bit in EP_STATUS_STALL_NAK when EP0 sends a STALL
    #[inline(always)]
    pub fn ep0_int_stall(&self) -> bool {
        (self.value.read_bits() & Self::EP0_INT_STALL_MASK) != 0
    }

    /// Set EP0_INT_STALL bit atomically (RW)
    #[inline(always)]
    pub fn set_ep0_int_stall(&self) {
        self.set_bits(Self::EP0_INT_STALL_MASK);
    }

    /// Clear EP0_INT_STALL bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep0_int_stall(&self) {
        self.clr_bits(Self::EP0_INT_STALL_MASK);
    }

    /// **EP0_DOUBLE_BUF** `[30]` (RW)
    ///
    /// Device: EP0 single buffered = 0, double buffered = 1
    #[inline(always)]
    pub fn ep0_double_buf(&self) -> bool {
        (self.value.read_bits() & Self::EP0_DOUBLE_BUF_MASK) != 0
    }

    /// Set EP0_DOUBLE_BUF bit atomically (RW)
    #[inline(always)]
    pub fn set_ep0_double_buf(&self) {
        self.set_bits(Self::EP0_DOUBLE_BUF_MASK);
    }

    /// Clear EP0_DOUBLE_BUF bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep0_double_buf(&self) {
        self.clr_bits(Self::EP0_DOUBLE_BUF_MASK);
    }

    /// **EP0_INT_1BUF** `[29]` (RW)
    ///
    /// Device: Set bit in BUFF_STATUS for every buffer completed on EP0
    #[inline(always)]
    pub fn ep0_int_1buf(&self) -> bool {
        (self.value.read_bits() & Self::EP0_INT_1BUF_MASK) != 0
    }

    /// Set EP0_INT_1BUF bit atomically (RW)
    #[inline(always)]
    pub fn set_ep0_int_1buf(&self) {
        self.set_bits(Self::EP0_INT_1BUF_MASK);
    }

    /// Clear EP0_INT_1BUF bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep0_int_1buf(&self) {
        self.clr_bits(Self::EP0_INT_1BUF_MASK);
    }

    /// **EP0_INT_2BUF** `[28]` (RW)
    ///
    /// Device: Set bit in BUFF_STATUS for every 2 buffers completed on EP0
    #[inline(always)]
    pub fn ep0_int_2buf(&self) -> bool {
        (self.value.read_bits() & Self::EP0_INT_2BUF_MASK) != 0
    }

    /// Set EP0_INT_2BUF bit atomically (RW)
    #[inline(always)]
    pub fn set_ep0_int_2buf(&self) {
        self.set_bits(Self::EP0_INT_2BUF_MASK);
    }

    /// Clear EP0_INT_2BUF bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep0_int_2buf(&self) {
        self.clr_bits(Self::EP0_INT_2BUF_MASK);
    }

    /// **EP0_INT_NAK** `[27]` (RW)
    ///
    /// Device: Set bit in EP_STATUS_STALL_NAK when EP0 sends a NAK
    #[inline(always)]
    pub fn ep0_int_nak(&self) -> bool {
        (self.value.read_bits() & Self::EP0_INT_NAK_MASK) != 0
    }

    /// Set EP0_INT_NAK bit atomically (RW)
    #[inline(always)]
    pub fn set_ep0_int_nak(&self) {
        self.set_bits(Self::EP0_INT_NAK_MASK);
    }

    /// Clear EP0_INT_NAK bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep0_int_nak(&self) {
        self.clr_bits(Self::EP0_INT_NAK_MASK);
    }

    /// **DIRECT_EN** `[26]` (RW)
    ///
    /// Direct bus drive enable
    #[inline(always)]
    pub fn direct_en(&self) -> bool {
        (self.value.read_bits() & Self::DIRECT_EN_MASK) != 0
    }

    /// Set DIRECT_EN bit atomically (RW)
    #[inline(always)]
    pub fn set_direct_en(&self) {
        self.set_bits(Self::DIRECT_EN_MASK);
    }

    /// Clear DIRECT_EN bit atomically (RW)
    #[inline(always)]
    pub fn clr_direct_en(&self) {
        self.clr_bits(Self::DIRECT_EN_MASK);
    }

    /// **DIRECT_DP** `[25]` (RW)
    ///
    /// Direct control of DP
    #[inline(always)]
    pub fn direct_dp(&self) -> bool {
        (self.value.read_bits() & Self::DIRECT_DP_MASK) != 0
    }

    /// Set DIRECT_DP bit atomically (RW)
    #[inline(always)]
    pub fn set_direct_dp(&self) {
        self.set_bits(Self::DIRECT_DP_MASK);
    }

    /// Clear DIRECT_DP bit atomically (RW)
    #[inline(always)]
    pub fn clr_direct_dp(&self) {
        self.clr_bits(Self::DIRECT_DP_MASK);
    }

    /// **DIRECT_DM** `[24]` (RW)
    ///
    /// Direct control of DM
    #[inline(always)]
    pub fn direct_dm(&self) -> bool {
        (self.value.read_bits() & Self::DIRECT_DM_MASK) != 0
    }

    /// Set DIRECT_DM bit atomically (RW)
    #[inline(always)]
    pub fn set_direct_dm(&self) {
        self.set_bits(Self::DIRECT_DM_MASK);
    }

    /// Clear DIRECT_DM bit atomically (RW)
    #[inline(always)]
    pub fn clr_direct_dm(&self) {
        self.clr_bits(Self::DIRECT_DM_MASK);
    }

    /// **EP0_STOP_ON_SHORT_PACKET** `[19]` (RW)
    ///
    /// Device: Stop EP0 on a short packet.
    #[inline(always)]
    pub fn ep0_stop_on_short_packet(&self) -> bool {
        (self.value.read_bits() & Self::EP0_STOP_ON_SHORT_PACKET_MASK) != 0
    }

    /// Set EP0_STOP_ON_SHORT_PACKET bit atomically (RW)
    #[inline(always)]
    pub fn set_ep0_stop_on_short_packet(&self) {
        self.set_bits(Self::EP0_STOP_ON_SHORT_PACKET_MASK);
    }

    /// Clear EP0_STOP_ON_SHORT_PACKET bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep0_stop_on_short_packet(&self) {
        self.clr_bits(Self::EP0_STOP_ON_SHORT_PACKET_MASK);
    }

    /// **TRANSCEIVER_PD** `[18]` (RW)
    ///
    /// Power down bus transceiver
    #[inline(always)]
    pub fn transceiver_pd(&self) -> bool {
        (self.value.read_bits() & Self::TRANSCEIVER_PD_MASK) != 0
    }

    /// Set TRANSCEIVER_PD bit atomically (RW)
    #[inline(always)]
    pub fn set_transceiver_pd(&self) {
        self.set_bits(Self::TRANSCEIVER_PD_MASK);
    }

    /// Clear TRANSCEIVER_PD bit atomically (RW)
    #[inline(always)]
    pub fn clr_transceiver_pd(&self) {
        self.clr_bits(Self::TRANSCEIVER_PD_MASK);
    }

    /// **RPU_OPT** `[17]` (RW)
    ///
    /// Device: Pull-up strength (0=1K2, 1=2k3)
    #[inline(always)]
    pub fn rpu_opt(&self) -> bool {
        (self.value.read_bits() & Self::RPU_OPT_MASK) != 0
    }

    /// Set RPU_OPT bit atomically (RW)
    #[inline(always)]
    pub fn set_rpu_opt(&self) {
        self.set_bits(Self::RPU_OPT_MASK);
    }

    /// Clear RPU_OPT bit atomically (RW)
    #[inline(always)]
    pub fn clr_rpu_opt(&self) {
        self.clr_bits(Self::RPU_OPT_MASK);
    }

    /// **PULLUP_EN** `[16]` (RW)
    ///
    /// Device: Enable pull up resistor
    #[inline(always)]
    pub fn pullup_en(&self) -> bool {
        (self.value.read_bits() & Self::PULLUP_EN_MASK) != 0
    }

    /// Set PULLUP_EN bit atomically (RW)
    #[inline(always)]
    pub fn set_pullup_en(&self) {
        self.set_bits(Self::PULLUP_EN_MASK);
    }

    /// Clear PULLUP_EN bit atomically (RW)
    #[inline(always)]
    pub fn clr_pullup_en(&self) {
        self.clr_bits(Self::PULLUP_EN_MASK);
    }

    /// **PULLDOWN_EN** `[15]` (RW)
    ///
    /// Host: Enable pull down resistors
    #[inline(always)]
    pub fn pulldown_en(&self) -> bool {
        (self.value.read_bits() & Self::PULLDOWN_EN_MASK) != 0
    }

    /// Set PULLDOWN_EN bit atomically (RW)
    #[inline(always)]
    pub fn set_pulldown_en(&self) {
        self.set_bits(Self::PULLDOWN_EN_MASK);
    }

    /// Clear PULLDOWN_EN bit atomically (RW)
    #[inline(always)]
    pub fn clr_pulldown_en(&self) {
        self.clr_bits(Self::PULLDOWN_EN_MASK);
    }

    /// Trigger **RESET_BUS** (WO)
    ///
    /// Host: Reset bus
    #[inline(always)]
    pub fn trigger_reset_bus(&self) {
        self.set_bits(Self::RESET_BUS_MASK);
    }

    /// Trigger **RESUME** (WO)
    ///
    /// Device: Remote wakeup. Device can initiate its own resume after suspend.
    #[inline(always)]
    pub fn trigger_resume(&self) {
        self.set_bits(Self::RESUME_MASK);
    }

    /// **VBUS_EN** `[11]` (RW)
    ///
    /// Host: Enable VBUS
    #[inline(always)]
    pub fn vbus_en(&self) -> bool {
        (self.value.read_bits() & Self::VBUS_EN_MASK) != 0
    }

    /// Set VBUS_EN bit atomically (RW)
    #[inline(always)]
    pub fn set_vbus_en(&self) {
        self.set_bits(Self::VBUS_EN_MASK);
    }

    /// Clear VBUS_EN bit atomically (RW)
    #[inline(always)]
    pub fn clr_vbus_en(&self) {
        self.clr_bits(Self::VBUS_EN_MASK);
    }

    /// **KEEP_ALIVE_EN** `[10]` (RW)
    ///
    /// Host: Enable keep alive packet (for low speed bus)
    #[inline(always)]
    pub fn keep_alive_en(&self) -> bool {
        (self.value.read_bits() & Self::KEEP_ALIVE_EN_MASK) != 0
    }

    /// Set KEEP_ALIVE_EN bit atomically (RW)
    #[inline(always)]
    pub fn set_keep_alive_en(&self) {
        self.set_bits(Self::KEEP_ALIVE_EN_MASK);
    }

    /// Clear KEEP_ALIVE_EN bit atomically (RW)
    #[inline(always)]
    pub fn clr_keep_alive_en(&self) {
        self.clr_bits(Self::KEEP_ALIVE_EN_MASK);
    }

    /// **SOF_EN** `[9]` (RW)
    ///
    /// Host: Enable SOF generation (for full speed bus)
    #[inline(always)]
    pub fn sof_en(&self) -> bool {
        (self.value.read_bits() & Self::SOF_EN_MASK) != 0
    }

    /// Set SOF_EN bit atomically (RW)
    #[inline(always)]
    pub fn set_sof_en(&self) {
        self.set_bits(Self::SOF_EN_MASK);
    }

    /// Clear SOF_EN bit atomically (RW)
    #[inline(always)]
    pub fn clr_sof_en(&self) {
        self.clr_bits(Self::SOF_EN_MASK);
    }

    /// **SOF_SYNC** `[8]` (RW)
    ///
    /// Host: Delay packet(s) until after SOF
    #[inline(always)]
    pub fn sof_sync(&self) -> bool {
        (self.value.read_bits() & Self::SOF_SYNC_MASK) != 0
    }

    /// Set SOF_SYNC bit atomically (RW)
    #[inline(always)]
    pub fn set_sof_sync(&self) {
        self.set_bits(Self::SOF_SYNC_MASK);
    }

    /// Clear SOF_SYNC bit atomically (RW)
    #[inline(always)]
    pub fn clr_sof_sync(&self) {
        self.clr_bits(Self::SOF_SYNC_MASK);
    }

    /// **PREAMBLE_EN** `[6]` (RW)
    ///
    /// Host: Preable enable for LS device on FS hub
    #[inline(always)]
    pub fn preamble_en(&self) -> bool {
        (self.value.read_bits() & Self::PREAMBLE_EN_MASK) != 0
    }

    /// Set PREAMBLE_EN bit atomically (RW)
    #[inline(always)]
    pub fn set_preamble_en(&self) {
        self.set_bits(Self::PREAMBLE_EN_MASK);
    }

    /// Clear PREAMBLE_EN bit atomically (RW)
    #[inline(always)]
    pub fn clr_preamble_en(&self) {
        self.clr_bits(Self::PREAMBLE_EN_MASK);
    }

    /// Trigger **STOP_TRANS** (WO)
    ///
    /// Host: Stop transaction
    #[inline(always)]
    pub fn trigger_stop_trans(&self) {
        self.set_bits(Self::STOP_TRANS_MASK);
    }

    /// **RECEIVE_DATA** `[3]` (RW)
    ///
    /// Host: Receive transaction (IN to host)
    #[inline(always)]
    pub fn receive_data(&self) -> bool {
        (self.value.read_bits() & Self::RECEIVE_DATA_MASK) != 0
    }

    /// Set RECEIVE_DATA bit atomically (RW)
    #[inline(always)]
    pub fn set_receive_data(&self) {
        self.set_bits(Self::RECEIVE_DATA_MASK);
    }

    /// Clear RECEIVE_DATA bit atomically (RW)
    #[inline(always)]
    pub fn clr_receive_data(&self) {
        self.clr_bits(Self::RECEIVE_DATA_MASK);
    }

    /// **SEND_DATA** `[2]` (RW)
    ///
    /// Host: Send transaction (OUT from host)
    #[inline(always)]
    pub fn send_data(&self) -> bool {
        (self.value.read_bits() & Self::SEND_DATA_MASK) != 0
    }

    /// Set SEND_DATA bit atomically (RW)
    #[inline(always)]
    pub fn set_send_data(&self) {
        self.set_bits(Self::SEND_DATA_MASK);
    }

    /// Clear SEND_DATA bit atomically (RW)
    #[inline(always)]
    pub fn clr_send_data(&self) {
        self.clr_bits(Self::SEND_DATA_MASK);
    }

    /// **SEND_SETUP** `[1]` (RW)
    ///
    /// Host: Send Setup packet
    #[inline(always)]
    pub fn send_setup(&self) -> bool {
        (self.value.read_bits() & Self::SEND_SETUP_MASK) != 0
    }

    /// Set SEND_SETUP bit atomically (RW)
    #[inline(always)]
    pub fn set_send_setup(&self) {
        self.set_bits(Self::SEND_SETUP_MASK);
    }

    /// Clear SEND_SETUP bit atomically (RW)
    #[inline(always)]
    pub fn clr_send_setup(&self) {
        self.clr_bits(Self::SEND_SETUP_MASK);
    }

    /// Trigger **START_TRANS** (WO)
    ///
    /// Host: Start transaction
    #[inline(always)]
    pub fn trigger_start_trans(&self) {
        self.set_bits(Self::START_TRANS_MASK);
    }
}

/// SIE_CTRL value type for batch read/modify/write operations
///
/// Use `siectrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `siectrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SieCtrlVal {
    bits: u32,
}

impl SieCtrlVal {
    /// Create from raw bits
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
        Self { bits: 0x00008000 }
    }

    /// Read **EP0_INT_STALL** `[31:31]`
    ///
    /// Device: Set bit in EP_STATUS_STALL_NAK when EP0 sends a STALL
    #[inline(always)]
    pub const fn ep0_int_stall(&self) -> bool {
        (self.bits & SieCtrl::EP0_INT_STALL_MASK) != 0
    }

    /// Read **EP0_DOUBLE_BUF** `[30:30]`
    ///
    /// Device: EP0 single buffered = 0, double buffered = 1
    #[inline(always)]
    pub const fn ep0_double_buf(&self) -> bool {
        (self.bits & SieCtrl::EP0_DOUBLE_BUF_MASK) != 0
    }

    /// Read **EP0_INT_1BUF** `[29:29]`
    ///
    /// Device: Set bit in BUFF_STATUS for every buffer completed on EP0
    #[inline(always)]
    pub const fn ep0_int_1buf(&self) -> bool {
        (self.bits & SieCtrl::EP0_INT_1BUF_MASK) != 0
    }

    /// Read **EP0_INT_2BUF** `[28:28]`
    ///
    /// Device: Set bit in BUFF_STATUS for every 2 buffers completed on EP0
    #[inline(always)]
    pub const fn ep0_int_2buf(&self) -> bool {
        (self.bits & SieCtrl::EP0_INT_2BUF_MASK) != 0
    }

    /// Read **EP0_INT_NAK** `[27:27]`
    ///
    /// Device: Set bit in EP_STATUS_STALL_NAK when EP0 sends a NAK
    #[inline(always)]
    pub const fn ep0_int_nak(&self) -> bool {
        (self.bits & SieCtrl::EP0_INT_NAK_MASK) != 0
    }

    /// Read **DIRECT_EN** `[26:26]`
    ///
    /// Direct bus drive enable
    #[inline(always)]
    pub const fn direct_en(&self) -> bool {
        (self.bits & SieCtrl::DIRECT_EN_MASK) != 0
    }

    /// Read **DIRECT_DP** `[25:25]`
    ///
    /// Direct control of DP
    #[inline(always)]
    pub const fn direct_dp(&self) -> bool {
        (self.bits & SieCtrl::DIRECT_DP_MASK) != 0
    }

    /// Read **DIRECT_DM** `[24:24]`
    ///
    /// Direct control of DM
    #[inline(always)]
    pub const fn direct_dm(&self) -> bool {
        (self.bits & SieCtrl::DIRECT_DM_MASK) != 0
    }

    /// Read **EP0_STOP_ON_SHORT_PACKET** `[19:19]`
    ///
    /// Device: Stop EP0 on a short packet.
    #[inline(always)]
    pub const fn ep0_stop_on_short_packet(&self) -> bool {
        (self.bits & SieCtrl::EP0_STOP_ON_SHORT_PACKET_MASK) != 0
    }

    /// Read **TRANSCEIVER_PD** `[18:18]`
    ///
    /// Power down bus transceiver
    #[inline(always)]
    pub const fn transceiver_pd(&self) -> bool {
        (self.bits & SieCtrl::TRANSCEIVER_PD_MASK) != 0
    }

    /// Read **RPU_OPT** `[17:17]`
    ///
    /// Device: Pull-up strength (0=1K2, 1=2k3)
    #[inline(always)]
    pub const fn rpu_opt(&self) -> bool {
        (self.bits & SieCtrl::RPU_OPT_MASK) != 0
    }

    /// Read **PULLUP_EN** `[16:16]`
    ///
    /// Device: Enable pull up resistor
    #[inline(always)]
    pub const fn pullup_en(&self) -> bool {
        (self.bits & SieCtrl::PULLUP_EN_MASK) != 0
    }

    /// Read **PULLDOWN_EN** `[15:15]`
    ///
    /// Host: Enable pull down resistors
    #[inline(always)]
    pub const fn pulldown_en(&self) -> bool {
        (self.bits & SieCtrl::PULLDOWN_EN_MASK) != 0
    }

    /// Read **VBUS_EN** `[11:11]`
    ///
    /// Host: Enable VBUS
    #[inline(always)]
    pub const fn vbus_en(&self) -> bool {
        (self.bits & SieCtrl::VBUS_EN_MASK) != 0
    }

    /// Read **KEEP_ALIVE_EN** `[10:10]`
    ///
    /// Host: Enable keep alive packet (for low speed bus)
    #[inline(always)]
    pub const fn keep_alive_en(&self) -> bool {
        (self.bits & SieCtrl::KEEP_ALIVE_EN_MASK) != 0
    }

    /// Read **SOF_EN** `[9:9]`
    ///
    /// Host: Enable SOF generation (for full speed bus)
    #[inline(always)]
    pub const fn sof_en(&self) -> bool {
        (self.bits & SieCtrl::SOF_EN_MASK) != 0
    }

    /// Read **SOF_SYNC** `[8:8]`
    ///
    /// Host: Delay packet(s) until after SOF
    #[inline(always)]
    pub const fn sof_sync(&self) -> bool {
        (self.bits & SieCtrl::SOF_SYNC_MASK) != 0
    }

    /// Read **PREAMBLE_EN** `[6:6]`
    ///
    /// Host: Preable enable for LS device on FS hub
    #[inline(always)]
    pub const fn preamble_en(&self) -> bool {
        (self.bits & SieCtrl::PREAMBLE_EN_MASK) != 0
    }

    /// Read **RECEIVE_DATA** `[3:3]`
    ///
    /// Host: Receive transaction (IN to host)
    #[inline(always)]
    pub const fn receive_data(&self) -> bool {
        (self.bits & SieCtrl::RECEIVE_DATA_MASK) != 0
    }

    /// Read **SEND_DATA** `[2:2]`
    ///
    /// Host: Send transaction (OUT from host)
    #[inline(always)]
    pub const fn send_data(&self) -> bool {
        (self.bits & SieCtrl::SEND_DATA_MASK) != 0
    }

    /// Read **SEND_SETUP** `[1:1]`
    ///
    /// Host: Send Setup packet
    #[inline(always)]
    pub const fn send_setup(&self) -> bool {
        (self.bits & SieCtrl::SEND_SETUP_MASK) != 0
    }

    /// Set **EP0_INT_STALL** `[31:31]`
    ///
    /// Device: Set bit in EP_STATUS_STALL_NAK when EP0 sends a STALL
    #[inline(always)]
    pub const fn with_ep0_int_stall(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SieCtrl::EP0_INT_STALL_MASK
            } else {
                self.bits & !SieCtrl::EP0_INT_STALL_MASK
            },
        }
    }

    /// Set **EP0_DOUBLE_BUF** `[30:30]`
    ///
    /// Device: EP0 single buffered = 0, double buffered = 1
    #[inline(always)]
    pub const fn with_ep0_double_buf(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SieCtrl::EP0_DOUBLE_BUF_MASK
            } else {
                self.bits & !SieCtrl::EP0_DOUBLE_BUF_MASK
            },
        }
    }

    /// Set **EP0_INT_1BUF** `[29:29]`
    ///
    /// Device: Set bit in BUFF_STATUS for every buffer completed on EP0
    #[inline(always)]
    pub const fn with_ep0_int_1buf(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SieCtrl::EP0_INT_1BUF_MASK
            } else {
                self.bits & !SieCtrl::EP0_INT_1BUF_MASK
            },
        }
    }

    /// Set **EP0_INT_2BUF** `[28:28]`
    ///
    /// Device: Set bit in BUFF_STATUS for every 2 buffers completed on EP0
    #[inline(always)]
    pub const fn with_ep0_int_2buf(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SieCtrl::EP0_INT_2BUF_MASK
            } else {
                self.bits & !SieCtrl::EP0_INT_2BUF_MASK
            },
        }
    }

    /// Set **EP0_INT_NAK** `[27:27]`
    ///
    /// Device: Set bit in EP_STATUS_STALL_NAK when EP0 sends a NAK
    #[inline(always)]
    pub const fn with_ep0_int_nak(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SieCtrl::EP0_INT_NAK_MASK
            } else {
                self.bits & !SieCtrl::EP0_INT_NAK_MASK
            },
        }
    }

    /// Set **DIRECT_EN** `[26:26]`
    ///
    /// Direct bus drive enable
    #[inline(always)]
    pub const fn with_direct_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SieCtrl::DIRECT_EN_MASK
            } else {
                self.bits & !SieCtrl::DIRECT_EN_MASK
            },
        }
    }

    /// Set **DIRECT_DP** `[25:25]`
    ///
    /// Direct control of DP
    #[inline(always)]
    pub const fn with_direct_dp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SieCtrl::DIRECT_DP_MASK
            } else {
                self.bits & !SieCtrl::DIRECT_DP_MASK
            },
        }
    }

    /// Set **DIRECT_DM** `[24:24]`
    ///
    /// Direct control of DM
    #[inline(always)]
    pub const fn with_direct_dm(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SieCtrl::DIRECT_DM_MASK
            } else {
                self.bits & !SieCtrl::DIRECT_DM_MASK
            },
        }
    }

    /// Set **EP0_STOP_ON_SHORT_PACKET** `[19:19]`
    ///
    /// Device: Stop EP0 on a short packet.
    #[inline(always)]
    pub const fn with_ep0_stop_on_short_packet(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SieCtrl::EP0_STOP_ON_SHORT_PACKET_MASK
            } else {
                self.bits & !SieCtrl::EP0_STOP_ON_SHORT_PACKET_MASK
            },
        }
    }

    /// Set **TRANSCEIVER_PD** `[18:18]`
    ///
    /// Power down bus transceiver
    #[inline(always)]
    pub const fn with_transceiver_pd(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SieCtrl::TRANSCEIVER_PD_MASK
            } else {
                self.bits & !SieCtrl::TRANSCEIVER_PD_MASK
            },
        }
    }

    /// Set **RPU_OPT** `[17:17]`
    ///
    /// Device: Pull-up strength (0=1K2, 1=2k3)
    #[inline(always)]
    pub const fn with_rpu_opt(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SieCtrl::RPU_OPT_MASK
            } else {
                self.bits & !SieCtrl::RPU_OPT_MASK
            },
        }
    }

    /// Set **PULLUP_EN** `[16:16]`
    ///
    /// Device: Enable pull up resistor
    #[inline(always)]
    pub const fn with_pullup_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SieCtrl::PULLUP_EN_MASK
            } else {
                self.bits & !SieCtrl::PULLUP_EN_MASK
            },
        }
    }

    /// Set **PULLDOWN_EN** `[15:15]`
    ///
    /// Host: Enable pull down resistors
    #[inline(always)]
    pub const fn with_pulldown_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SieCtrl::PULLDOWN_EN_MASK
            } else {
                self.bits & !SieCtrl::PULLDOWN_EN_MASK
            },
        }
    }

    /// Set **RESET_BUS** `[13:13]`
    ///
    /// Host: Reset bus
    #[inline(always)]
    pub const fn with_reset_bus(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SieCtrl::RESET_BUS_MASK
            } else {
                self.bits & !SieCtrl::RESET_BUS_MASK
            },
        }
    }

    /// Set **RESUME** `[12:12]`
    ///
    /// Device: Remote wakeup. Device can initiate its own resume after suspend.
    #[inline(always)]
    pub const fn with_resume(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SieCtrl::RESUME_MASK
            } else {
                self.bits & !SieCtrl::RESUME_MASK
            },
        }
    }

    /// Set **VBUS_EN** `[11:11]`
    ///
    /// Host: Enable VBUS
    #[inline(always)]
    pub const fn with_vbus_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SieCtrl::VBUS_EN_MASK
            } else {
                self.bits & !SieCtrl::VBUS_EN_MASK
            },
        }
    }

    /// Set **KEEP_ALIVE_EN** `[10:10]`
    ///
    /// Host: Enable keep alive packet (for low speed bus)
    #[inline(always)]
    pub const fn with_keep_alive_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SieCtrl::KEEP_ALIVE_EN_MASK
            } else {
                self.bits & !SieCtrl::KEEP_ALIVE_EN_MASK
            },
        }
    }

    /// Set **SOF_EN** `[9:9]`
    ///
    /// Host: Enable SOF generation (for full speed bus)
    #[inline(always)]
    pub const fn with_sof_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SieCtrl::SOF_EN_MASK
            } else {
                self.bits & !SieCtrl::SOF_EN_MASK
            },
        }
    }

    /// Set **SOF_SYNC** `[8:8]`
    ///
    /// Host: Delay packet(s) until after SOF
    #[inline(always)]
    pub const fn with_sof_sync(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SieCtrl::SOF_SYNC_MASK
            } else {
                self.bits & !SieCtrl::SOF_SYNC_MASK
            },
        }
    }

    /// Set **PREAMBLE_EN** `[6:6]`
    ///
    /// Host: Preable enable for LS device on FS hub
    #[inline(always)]
    pub const fn with_preamble_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SieCtrl::PREAMBLE_EN_MASK
            } else {
                self.bits & !SieCtrl::PREAMBLE_EN_MASK
            },
        }
    }

    /// Set **STOP_TRANS** `[4:4]`
    ///
    /// Host: Stop transaction
    #[inline(always)]
    pub const fn with_stop_trans(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SieCtrl::STOP_TRANS_MASK
            } else {
                self.bits & !SieCtrl::STOP_TRANS_MASK
            },
        }
    }

    /// Set **RECEIVE_DATA** `[3:3]`
    ///
    /// Host: Receive transaction (IN to host)
    #[inline(always)]
    pub const fn with_receive_data(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SieCtrl::RECEIVE_DATA_MASK
            } else {
                self.bits & !SieCtrl::RECEIVE_DATA_MASK
            },
        }
    }

    /// Set **SEND_DATA** `[2:2]`
    ///
    /// Host: Send transaction (OUT from host)
    #[inline(always)]
    pub const fn with_send_data(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SieCtrl::SEND_DATA_MASK
            } else {
                self.bits & !SieCtrl::SEND_DATA_MASK
            },
        }
    }

    /// Set **SEND_SETUP** `[1:1]`
    ///
    /// Host: Send Setup packet
    #[inline(always)]
    pub const fn with_send_setup(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SieCtrl::SEND_SETUP_MASK
            } else {
                self.bits & !SieCtrl::SEND_SETUP_MASK
            },
        }
    }

    /// Set **START_TRANS** `[0:0]`
    ///
    /// Host: Start transaction
    #[inline(always)]
    pub const fn with_start_trans(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SieCtrl::START_TRANS_MASK
            } else {
                self.bits & !SieCtrl::START_TRANS_MASK
            },
        }
    }
}

impl Default for SieCtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SieCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SieCtrlVal {
        SieCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SieCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SieCtrlVal) -> SieCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SIE_STATUS Register
///
/// SIE status register
///
/// Address offset: 0x0050
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **DATA_SEQ_ERROR** `[31]` (RW)
///
///   Data Sequence Error.
///   The device can raise a sequence error in the following conditions:
///   * A SETUP packet is received followed by a DATA1 packet (data phase should always be DATA0) * An OUT packet is received from the host but doesn't match the data pid in the buffer control register read from DPSRAM
///   The host can raise a data sequence error in the following conditions:
///   * An IN packet from the device has the wrong data PID
///
/// - **ACK_REC** `[30]` (RW)
///
///   ACK received. Raised by both host and device.
///
/// - **STALL_REC** `[29]` (RW)
///
///   Host: STALL received
///
/// - **NAK_REC** `[28]` (RW)
///
///   Host: NAK received
///
/// - **RX_TIMEOUT** `[27]` (RW)
///
///   RX timeout is raised by both the host and device if an ACK is not received in the maximum time specified by the USB spec.
///
/// - **RX_OVERFLOW** `[26]` (RW)
///
///   RX overflow is raised by the Serial RX engine if the incoming data is too fast.
///
/// - **BIT_STUFF_ERROR** `[25]` (RW)
///
///   Bit Stuff Error. Raised by the Serial RX engine.
///
/// - **CRC_ERROR** `[24]` (RW)
///
///   CRC Error. Raised by the Serial RX engine.
///
/// - **ENDPOINT_ERROR** `[23]` (RW)
///
///   An endpoint has encountered an error. Read the ep_rx_error and ep_tx_error registers to find out which endpoint had an error.
///
/// - **BUS_RESET** `[19]` (RW)
///
///   Device: bus reset received
///
/// - **TRANS_COMPLETE** `[18]` (RW)
///
///   Transaction complete.
///   Raised by device if:
///   * An IN or OUT packet is sent with the `LAST_BUFF` bit set in the buffer control register
///   Raised by host if:
///   * A setup packet is sent when no data in or data out transaction follows * An IN packet is received and the `LAST_BUFF` bit is set in the buffer control register * An IN packet is received with zero length * An OUT packet is sent and the `LAST_BUFF` bit is set
///
/// - **SETUP_REC** `[17]` (RW)
///
///   Device: Setup packet received
///
/// - **CONNECTED** `[16]` (RO)
///
///   Device: connected
///
/// - **RX_SHORT_PACKET** `[12]` (RW)
///
///   Device or Host has received a short packet. This is when the data received is less than configured in the buffer control register. Device: If using double buffered mode on device the buffer select will not be toggled after writing status back to the buffer control register. This is to prevent any further transactions on that endpoint until the user has reset the buffer control registers. Host: the current transfer will be stopped early.
///
/// - **RESUME** `[11]` (RW)
///
///   Host: Device has initiated a remote resume. Device: host has initiated a resume.
///
/// - **VBUS_OVER_CURR** `[10]` (RO)
///
///   VBUS over current detected
///
/// - **SPEED** `[9:8]` (RO)
///
///   Host: device speed. Disconnected = 00, LS = 01, FS = 10
///
/// - **SUSPENDED** `[4]` (RW)
///
///   Bus in suspended state. Valid for device and host. Host and device will go into suspend if neither Keep Alive / SOF frames are enabled.
///
/// - **LINE_STATE** `[3:2]` (RO)
///
///   USB bus line state
///
/// - **VBUS_DETECTED** `[0]` (RO)
///
///   Device: VBUS Detected
#[repr(transparent)]
pub struct SieStatus {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for SieStatus {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SieStatus {
    // ======== Field Constants ========

    /// **DATA_SEQ_ERROR**
    ///
    /// Data Sequence Error.
    /// The device can raise a sequence error in the following conditions:
    /// * A SETUP packet is received followed by a DATA1 packet (data phase should always be DATA0) * An OUT packet is received from the host but doesn't match the data pid in the buffer control register read from DPSRAM
    /// The host can raise a data sequence error in the following conditions:
    /// * An IN packet from the device has the wrong data PID
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const DATA_SEQ_ERROR_OFFSET: u32 = 31;
    pub const DATA_SEQ_ERROR_MASK: u32 = 0x80000000;
    pub const DATA_SEQ_ERROR: u32 = 1 << 31;

    /// **ACK_REC**
    ///
    /// ACK received. Raised by both host and device.
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const ACK_REC_OFFSET: u32 = 30;
    pub const ACK_REC_MASK: u32 = 0x40000000;
    pub const ACK_REC: u32 = 1 << 30;

    /// **STALL_REC**
    ///
    /// Host: STALL received
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const STALL_REC_OFFSET: u32 = 29;
    pub const STALL_REC_MASK: u32 = 0x20000000;
    pub const STALL_REC: u32 = 1 << 29;

    /// **NAK_REC**
    ///
    /// Host: NAK received
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const NAK_REC_OFFSET: u32 = 28;
    pub const NAK_REC_MASK: u32 = 0x10000000;
    pub const NAK_REC: u32 = 1 << 28;

    /// **RX_TIMEOUT**
    ///
    /// RX timeout is raised by both the host and device if an ACK is not received in the maximum time specified by the USB spec.
    ///
    /// Bits `[27:27]`, mask `0x08000000`
    pub const RX_TIMEOUT_OFFSET: u32 = 27;
    pub const RX_TIMEOUT_MASK: u32 = 0x08000000;
    pub const RX_TIMEOUT: u32 = 1 << 27;

    /// **RX_OVERFLOW**
    ///
    /// RX overflow is raised by the Serial RX engine if the incoming data is too fast.
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const RX_OVERFLOW_OFFSET: u32 = 26;
    pub const RX_OVERFLOW_MASK: u32 = 0x04000000;
    pub const RX_OVERFLOW: u32 = 1 << 26;

    /// **BIT_STUFF_ERROR**
    ///
    /// Bit Stuff Error. Raised by the Serial RX engine.
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const BIT_STUFF_ERROR_OFFSET: u32 = 25;
    pub const BIT_STUFF_ERROR_MASK: u32 = 0x02000000;
    pub const BIT_STUFF_ERROR: u32 = 1 << 25;

    /// **CRC_ERROR**
    ///
    /// CRC Error. Raised by the Serial RX engine.
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const CRC_ERROR_OFFSET: u32 = 24;
    pub const CRC_ERROR_MASK: u32 = 0x01000000;
    pub const CRC_ERROR: u32 = 1 << 24;

    /// **ENDPOINT_ERROR**
    ///
    /// An endpoint has encountered an error. Read the ep_rx_error and ep_tx_error registers to find out which endpoint had an error.
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const ENDPOINT_ERROR_OFFSET: u32 = 23;
    pub const ENDPOINT_ERROR_MASK: u32 = 0x00800000;
    pub const ENDPOINT_ERROR: u32 = 1 << 23;

    /// **BUS_RESET**
    ///
    /// Device: bus reset received
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const BUS_RESET_OFFSET: u32 = 19;
    pub const BUS_RESET_MASK: u32 = 0x00080000;
    pub const BUS_RESET: u32 = 1 << 19;

    /// **TRANS_COMPLETE**
    ///
    /// Transaction complete.
    /// Raised by device if:
    /// * An IN or OUT packet is sent with the `LAST_BUFF` bit set in the buffer control register
    /// Raised by host if:
    /// * A setup packet is sent when no data in or data out transaction follows * An IN packet is received and the `LAST_BUFF` bit is set in the buffer control register * An IN packet is received with zero length * An OUT packet is sent and the `LAST_BUFF` bit is set
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const TRANS_COMPLETE_OFFSET: u32 = 18;
    pub const TRANS_COMPLETE_MASK: u32 = 0x00040000;
    pub const TRANS_COMPLETE: u32 = 1 << 18;

    /// **SETUP_REC**
    ///
    /// Device: Setup packet received
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const SETUP_REC_OFFSET: u32 = 17;
    pub const SETUP_REC_MASK: u32 = 0x00020000;
    pub const SETUP_REC: u32 = 1 << 17;

    /// **CONNECTED**
    ///
    /// Device: connected
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const CONNECTED_OFFSET: u32 = 16;
    pub const CONNECTED_MASK: u32 = 0x00010000;
    pub const CONNECTED: u32 = 1 << 16;

    /// **RX_SHORT_PACKET**
    ///
    /// Device or Host has received a short packet. This is when the data received is less than configured in the buffer control register. Device: If using double buffered mode on device the buffer select will not be toggled after writing status back to the buffer control register. This is to prevent any further transactions on that endpoint until the user has reset the buffer control registers. Host: the current transfer will be stopped early.
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const RX_SHORT_PACKET_OFFSET: u32 = 12;
    pub const RX_SHORT_PACKET_MASK: u32 = 0x00001000;
    pub const RX_SHORT_PACKET: u32 = 1 << 12;

    /// **RESUME**
    ///
    /// Host: Device has initiated a remote resume. Device: host has initiated a resume.
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const RESUME_OFFSET: u32 = 11;
    pub const RESUME_MASK: u32 = 0x00000800;
    pub const RESUME: u32 = 1 << 11;

    /// **VBUS_OVER_CURR**
    ///
    /// VBUS over current detected
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const VBUS_OVER_CURR_OFFSET: u32 = 10;
    pub const VBUS_OVER_CURR_MASK: u32 = 0x00000400;
    pub const VBUS_OVER_CURR: u32 = 1 << 10;

    /// **SPEED**
    ///
    /// Host: device speed. Disconnected = 00, LS = 01, FS = 10
    ///
    /// Bits `[9:8]`, mask `0x00000300`
    pub const SPEED_OFFSET: u32 = 8;
    pub const SPEED_MASK: u32 = 0x00000300;

    /// **SUSPENDED**
    ///
    /// Bus in suspended state. Valid for device and host. Host and device will go into suspend if neither Keep Alive / SOF frames are enabled.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const SUSPENDED_OFFSET: u32 = 4;
    pub const SUSPENDED_MASK: u32 = 0x00000010;
    pub const SUSPENDED: u32 = 1 << 4;

    /// **LINE_STATE**
    ///
    /// USB bus line state
    ///
    /// Bits `[3:2]`, mask `0x0000000c`
    pub const LINE_STATE_OFFSET: u32 = 2;
    pub const LINE_STATE_MASK: u32 = 0x0000000c;

    /// **VBUS_DETECTED**
    ///
    /// Device: VBUS Detected
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const VBUS_DETECTED_OFFSET: u32 = 0;
    pub const VBUS_DETECTED_MASK: u32 = 0x00000001;
    pub const VBUS_DETECTED: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DATA_SEQ_ERROR** `[31]` (RW)
    ///
    /// Data Sequence Error.
    /// The device can raise a sequence error in the following conditions:
    /// * A SETUP packet is received followed by a DATA1 packet (data phase should always be DATA0) * An OUT packet is received from the host but doesn't match the data pid in the buffer control register read from DPSRAM
    /// The host can raise a data sequence error in the following conditions:
    /// * An IN packet from the device has the wrong data PID
    #[inline(always)]
    pub fn data_seq_error(&self) -> bool {
        (self.value.read_bits() & Self::DATA_SEQ_ERROR_MASK) != 0
    }

    /// Set DATA_SEQ_ERROR bit atomically (RW)
    #[inline(always)]
    pub fn set_data_seq_error(&self) {
        self.set_bits(Self::DATA_SEQ_ERROR_MASK);
    }

    /// Clear DATA_SEQ_ERROR bit atomically (RW)
    #[inline(always)]
    pub fn clr_data_seq_error(&self) {
        self.clr_bits(Self::DATA_SEQ_ERROR_MASK);
    }

    /// **ACK_REC** `[30]` (RW)
    ///
    /// ACK received. Raised by both host and device.
    #[inline(always)]
    pub fn ack_rec(&self) -> bool {
        (self.value.read_bits() & Self::ACK_REC_MASK) != 0
    }

    /// Set ACK_REC bit atomically (RW)
    #[inline(always)]
    pub fn set_ack_rec(&self) {
        self.set_bits(Self::ACK_REC_MASK);
    }

    /// Clear ACK_REC bit atomically (RW)
    #[inline(always)]
    pub fn clr_ack_rec(&self) {
        self.clr_bits(Self::ACK_REC_MASK);
    }

    /// **STALL_REC** `[29]` (RW)
    ///
    /// Host: STALL received
    #[inline(always)]
    pub fn stall_rec(&self) -> bool {
        (self.value.read_bits() & Self::STALL_REC_MASK) != 0
    }

    /// Set STALL_REC bit atomically (RW)
    #[inline(always)]
    pub fn set_stall_rec(&self) {
        self.set_bits(Self::STALL_REC_MASK);
    }

    /// Clear STALL_REC bit atomically (RW)
    #[inline(always)]
    pub fn clr_stall_rec(&self) {
        self.clr_bits(Self::STALL_REC_MASK);
    }

    /// **NAK_REC** `[28]` (RW)
    ///
    /// Host: NAK received
    #[inline(always)]
    pub fn nak_rec(&self) -> bool {
        (self.value.read_bits() & Self::NAK_REC_MASK) != 0
    }

    /// Set NAK_REC bit atomically (RW)
    #[inline(always)]
    pub fn set_nak_rec(&self) {
        self.set_bits(Self::NAK_REC_MASK);
    }

    /// Clear NAK_REC bit atomically (RW)
    #[inline(always)]
    pub fn clr_nak_rec(&self) {
        self.clr_bits(Self::NAK_REC_MASK);
    }

    /// **RX_TIMEOUT** `[27]` (RW)
    ///
    /// RX timeout is raised by both the host and device if an ACK is not received in the maximum time specified by the USB spec.
    #[inline(always)]
    pub fn rx_timeout(&self) -> bool {
        (self.value.read_bits() & Self::RX_TIMEOUT_MASK) != 0
    }

    /// Set RX_TIMEOUT bit atomically (RW)
    #[inline(always)]
    pub fn set_rx_timeout(&self) {
        self.set_bits(Self::RX_TIMEOUT_MASK);
    }

    /// Clear RX_TIMEOUT bit atomically (RW)
    #[inline(always)]
    pub fn clr_rx_timeout(&self) {
        self.clr_bits(Self::RX_TIMEOUT_MASK);
    }

    /// **RX_OVERFLOW** `[26]` (RW)
    ///
    /// RX overflow is raised by the Serial RX engine if the incoming data is too fast.
    #[inline(always)]
    pub fn rx_overflow(&self) -> bool {
        (self.value.read_bits() & Self::RX_OVERFLOW_MASK) != 0
    }

    /// Set RX_OVERFLOW bit atomically (RW)
    #[inline(always)]
    pub fn set_rx_overflow(&self) {
        self.set_bits(Self::RX_OVERFLOW_MASK);
    }

    /// Clear RX_OVERFLOW bit atomically (RW)
    #[inline(always)]
    pub fn clr_rx_overflow(&self) {
        self.clr_bits(Self::RX_OVERFLOW_MASK);
    }

    /// **BIT_STUFF_ERROR** `[25]` (RW)
    ///
    /// Bit Stuff Error. Raised by the Serial RX engine.
    #[inline(always)]
    pub fn bit_stuff_error(&self) -> bool {
        (self.value.read_bits() & Self::BIT_STUFF_ERROR_MASK) != 0
    }

    /// Set BIT_STUFF_ERROR bit atomically (RW)
    #[inline(always)]
    pub fn set_bit_stuff_error(&self) {
        self.set_bits(Self::BIT_STUFF_ERROR_MASK);
    }

    /// Clear BIT_STUFF_ERROR bit atomically (RW)
    #[inline(always)]
    pub fn clr_bit_stuff_error(&self) {
        self.clr_bits(Self::BIT_STUFF_ERROR_MASK);
    }

    /// **CRC_ERROR** `[24]` (RW)
    ///
    /// CRC Error. Raised by the Serial RX engine.
    #[inline(always)]
    pub fn crc_error(&self) -> bool {
        (self.value.read_bits() & Self::CRC_ERROR_MASK) != 0
    }

    /// Set CRC_ERROR bit atomically (RW)
    #[inline(always)]
    pub fn set_crc_error(&self) {
        self.set_bits(Self::CRC_ERROR_MASK);
    }

    /// Clear CRC_ERROR bit atomically (RW)
    #[inline(always)]
    pub fn clr_crc_error(&self) {
        self.clr_bits(Self::CRC_ERROR_MASK);
    }

    /// **ENDPOINT_ERROR** `[23]` (RW)
    ///
    /// An endpoint has encountered an error. Read the ep_rx_error and ep_tx_error registers to find out which endpoint had an error.
    #[inline(always)]
    pub fn endpoint_error(&self) -> bool {
        (self.value.read_bits() & Self::ENDPOINT_ERROR_MASK) != 0
    }

    /// Set ENDPOINT_ERROR bit atomically (RW)
    #[inline(always)]
    pub fn set_endpoint_error(&self) {
        self.set_bits(Self::ENDPOINT_ERROR_MASK);
    }

    /// Clear ENDPOINT_ERROR bit atomically (RW)
    #[inline(always)]
    pub fn clr_endpoint_error(&self) {
        self.clr_bits(Self::ENDPOINT_ERROR_MASK);
    }

    /// **BUS_RESET** `[19]` (RW)
    ///
    /// Device: bus reset received
    #[inline(always)]
    pub fn bus_reset(&self) -> bool {
        (self.value.read_bits() & Self::BUS_RESET_MASK) != 0
    }

    /// Set BUS_RESET bit atomically (RW)
    #[inline(always)]
    pub fn set_bus_reset(&self) {
        self.set_bits(Self::BUS_RESET_MASK);
    }

    /// Clear BUS_RESET bit atomically (RW)
    #[inline(always)]
    pub fn clr_bus_reset(&self) {
        self.clr_bits(Self::BUS_RESET_MASK);
    }

    /// **TRANS_COMPLETE** `[18]` (RW)
    ///
    /// Transaction complete.
    /// Raised by device if:
    /// * An IN or OUT packet is sent with the `LAST_BUFF` bit set in the buffer control register
    /// Raised by host if:
    /// * A setup packet is sent when no data in or data out transaction follows * An IN packet is received and the `LAST_BUFF` bit is set in the buffer control register * An IN packet is received with zero length * An OUT packet is sent and the `LAST_BUFF` bit is set
    #[inline(always)]
    pub fn trans_complete(&self) -> bool {
        (self.value.read_bits() & Self::TRANS_COMPLETE_MASK) != 0
    }

    /// Set TRANS_COMPLETE bit atomically (RW)
    #[inline(always)]
    pub fn set_trans_complete(&self) {
        self.set_bits(Self::TRANS_COMPLETE_MASK);
    }

    /// Clear TRANS_COMPLETE bit atomically (RW)
    #[inline(always)]
    pub fn clr_trans_complete(&self) {
        self.clr_bits(Self::TRANS_COMPLETE_MASK);
    }

    /// **SETUP_REC** `[17]` (RW)
    ///
    /// Device: Setup packet received
    #[inline(always)]
    pub fn setup_rec(&self) -> bool {
        (self.value.read_bits() & Self::SETUP_REC_MASK) != 0
    }

    /// Set SETUP_REC bit atomically (RW)
    #[inline(always)]
    pub fn set_setup_rec(&self) {
        self.set_bits(Self::SETUP_REC_MASK);
    }

    /// Clear SETUP_REC bit atomically (RW)
    #[inline(always)]
    pub fn clr_setup_rec(&self) {
        self.clr_bits(Self::SETUP_REC_MASK);
    }

    /// **CONNECTED** `[16]` (RO)
    ///
    /// Device: connected
    #[inline(always)]
    pub fn connected(&self) -> bool {
        (self.value.read_bits() & Self::CONNECTED_MASK) != 0
    }

    /// **RX_SHORT_PACKET** `[12]` (RW)
    ///
    /// Device or Host has received a short packet. This is when the data received is less than configured in the buffer control register. Device: If using double buffered mode on device the buffer select will not be toggled after writing status back to the buffer control register. This is to prevent any further transactions on that endpoint until the user has reset the buffer control registers. Host: the current transfer will be stopped early.
    #[inline(always)]
    pub fn rx_short_packet(&self) -> bool {
        (self.value.read_bits() & Self::RX_SHORT_PACKET_MASK) != 0
    }

    /// Set RX_SHORT_PACKET bit atomically (RW)
    #[inline(always)]
    pub fn set_rx_short_packet(&self) {
        self.set_bits(Self::RX_SHORT_PACKET_MASK);
    }

    /// Clear RX_SHORT_PACKET bit atomically (RW)
    #[inline(always)]
    pub fn clr_rx_short_packet(&self) {
        self.clr_bits(Self::RX_SHORT_PACKET_MASK);
    }

    /// **RESUME** `[11]` (RW)
    ///
    /// Host: Device has initiated a remote resume. Device: host has initiated a resume.
    #[inline(always)]
    pub fn resume(&self) -> bool {
        (self.value.read_bits() & Self::RESUME_MASK) != 0
    }

    /// Set RESUME bit atomically (RW)
    #[inline(always)]
    pub fn set_resume(&self) {
        self.set_bits(Self::RESUME_MASK);
    }

    /// Clear RESUME bit atomically (RW)
    #[inline(always)]
    pub fn clr_resume(&self) {
        self.clr_bits(Self::RESUME_MASK);
    }

    /// **VBUS_OVER_CURR** `[10]` (RO)
    ///
    /// VBUS over current detected
    #[inline(always)]
    pub fn vbus_over_curr(&self) -> bool {
        (self.value.read_bits() & Self::VBUS_OVER_CURR_MASK) != 0
    }

    /// **SPEED** `[9:8]` (RO)
    ///
    /// Host: device speed. Disconnected = 00, LS = 01, FS = 10
    #[inline(always)]
    pub fn speed(&self) -> u32 {
        (self.value.read_bits() & Self::SPEED_MASK) >> Self::SPEED_OFFSET
    }

    /// **SUSPENDED** `[4]` (RW)
    ///
    /// Bus in suspended state. Valid for device and host. Host and device will go into suspend if neither Keep Alive / SOF frames are enabled.
    #[inline(always)]
    pub fn suspended(&self) -> bool {
        (self.value.read_bits() & Self::SUSPENDED_MASK) != 0
    }

    /// Set SUSPENDED bit atomically (RW)
    #[inline(always)]
    pub fn set_suspended(&self) {
        self.set_bits(Self::SUSPENDED_MASK);
    }

    /// Clear SUSPENDED bit atomically (RW)
    #[inline(always)]
    pub fn clr_suspended(&self) {
        self.clr_bits(Self::SUSPENDED_MASK);
    }

    /// **LINE_STATE** `[3:2]` (RO)
    ///
    /// USB bus line state
    #[inline(always)]
    pub fn line_state(&self) -> u32 {
        (self.value.read_bits() & Self::LINE_STATE_MASK) >> Self::LINE_STATE_OFFSET
    }

    /// **VBUS_DETECTED** `[0]` (RO)
    ///
    /// Device: VBUS Detected
    #[inline(always)]
    pub fn vbus_detected(&self) -> bool {
        (self.value.read_bits() & Self::VBUS_DETECTED_MASK) != 0
    }
}

/// SIE_STATUS value type for batch read/modify/write operations
///
/// Use `siestatus.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `siestatus.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SieStatusVal {
    bits: u32,
}

impl SieStatusVal {
    /// Create from raw bits
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

    /// Read **DATA_SEQ_ERROR** `[31:31]`
    ///
    /// Data Sequence Error.
    /// The device can raise a sequence error in the following conditions:
    /// * A SETUP packet is received followed by a DATA1 packet (data phase should always be DATA0) * An OUT packet is received from the host but doesn't match the data pid in the buffer control register read from DPSRAM
    /// The host can raise a data sequence error in the following conditions:
    /// * An IN packet from the device has the wrong data PID
    #[inline(always)]
    pub const fn data_seq_error(&self) -> bool {
        (self.bits & SieStatus::DATA_SEQ_ERROR_MASK) != 0
    }

    /// Read **ACK_REC** `[30:30]`
    ///
    /// ACK received. Raised by both host and device.
    #[inline(always)]
    pub const fn ack_rec(&self) -> bool {
        (self.bits & SieStatus::ACK_REC_MASK) != 0
    }

    /// Read **STALL_REC** `[29:29]`
    ///
    /// Host: STALL received
    #[inline(always)]
    pub const fn stall_rec(&self) -> bool {
        (self.bits & SieStatus::STALL_REC_MASK) != 0
    }

    /// Read **NAK_REC** `[28:28]`
    ///
    /// Host: NAK received
    #[inline(always)]
    pub const fn nak_rec(&self) -> bool {
        (self.bits & SieStatus::NAK_REC_MASK) != 0
    }

    /// Read **RX_TIMEOUT** `[27:27]`
    ///
    /// RX timeout is raised by both the host and device if an ACK is not received in the maximum time specified by the USB spec.
    #[inline(always)]
    pub const fn rx_timeout(&self) -> bool {
        (self.bits & SieStatus::RX_TIMEOUT_MASK) != 0
    }

    /// Read **RX_OVERFLOW** `[26:26]`
    ///
    /// RX overflow is raised by the Serial RX engine if the incoming data is too fast.
    #[inline(always)]
    pub const fn rx_overflow(&self) -> bool {
        (self.bits & SieStatus::RX_OVERFLOW_MASK) != 0
    }

    /// Read **BIT_STUFF_ERROR** `[25:25]`
    ///
    /// Bit Stuff Error. Raised by the Serial RX engine.
    #[inline(always)]
    pub const fn bit_stuff_error(&self) -> bool {
        (self.bits & SieStatus::BIT_STUFF_ERROR_MASK) != 0
    }

    /// Read **CRC_ERROR** `[24:24]`
    ///
    /// CRC Error. Raised by the Serial RX engine.
    #[inline(always)]
    pub const fn crc_error(&self) -> bool {
        (self.bits & SieStatus::CRC_ERROR_MASK) != 0
    }

    /// Read **ENDPOINT_ERROR** `[23:23]`
    ///
    /// An endpoint has encountered an error. Read the ep_rx_error and ep_tx_error registers to find out which endpoint had an error.
    #[inline(always)]
    pub const fn endpoint_error(&self) -> bool {
        (self.bits & SieStatus::ENDPOINT_ERROR_MASK) != 0
    }

    /// Read **BUS_RESET** `[19:19]`
    ///
    /// Device: bus reset received
    #[inline(always)]
    pub const fn bus_reset(&self) -> bool {
        (self.bits & SieStatus::BUS_RESET_MASK) != 0
    }

    /// Read **TRANS_COMPLETE** `[18:18]`
    ///
    /// Transaction complete.
    /// Raised by device if:
    /// * An IN or OUT packet is sent with the `LAST_BUFF` bit set in the buffer control register
    /// Raised by host if:
    /// * A setup packet is sent when no data in or data out transaction follows * An IN packet is received and the `LAST_BUFF` bit is set in the buffer control register * An IN packet is received with zero length * An OUT packet is sent and the `LAST_BUFF` bit is set
    #[inline(always)]
    pub const fn trans_complete(&self) -> bool {
        (self.bits & SieStatus::TRANS_COMPLETE_MASK) != 0
    }

    /// Read **SETUP_REC** `[17:17]`
    ///
    /// Device: Setup packet received
    #[inline(always)]
    pub const fn setup_rec(&self) -> bool {
        (self.bits & SieStatus::SETUP_REC_MASK) != 0
    }

    /// Read **CONNECTED** `[16:16]`
    ///
    /// Device: connected
    #[inline(always)]
    pub const fn connected(&self) -> bool {
        (self.bits & SieStatus::CONNECTED_MASK) != 0
    }

    /// Read **RX_SHORT_PACKET** `[12:12]`
    ///
    /// Device or Host has received a short packet. This is when the data received is less than configured in the buffer control register. Device: If using double buffered mode on device the buffer select will not be toggled after writing status back to the buffer control register. This is to prevent any further transactions on that endpoint until the user has reset the buffer control registers. Host: the current transfer will be stopped early.
    #[inline(always)]
    pub const fn rx_short_packet(&self) -> bool {
        (self.bits & SieStatus::RX_SHORT_PACKET_MASK) != 0
    }

    /// Read **RESUME** `[11:11]`
    ///
    /// Host: Device has initiated a remote resume. Device: host has initiated a resume.
    #[inline(always)]
    pub const fn resume(&self) -> bool {
        (self.bits & SieStatus::RESUME_MASK) != 0
    }

    /// Read **VBUS_OVER_CURR** `[10:10]`
    ///
    /// VBUS over current detected
    #[inline(always)]
    pub const fn vbus_over_curr(&self) -> bool {
        (self.bits & SieStatus::VBUS_OVER_CURR_MASK) != 0
    }

    /// Read **SPEED** `[9:8]`
    ///
    /// Host: device speed. Disconnected = 00, LS = 01, FS = 10
    #[inline(always)]
    pub const fn speed(&self) -> u32 {
        (self.bits & SieStatus::SPEED_MASK) >> SieStatus::SPEED_OFFSET
    }

    /// Read **SUSPENDED** `[4:4]`
    ///
    /// Bus in suspended state. Valid for device and host. Host and device will go into suspend if neither Keep Alive / SOF frames are enabled.
    #[inline(always)]
    pub const fn suspended(&self) -> bool {
        (self.bits & SieStatus::SUSPENDED_MASK) != 0
    }

    /// Read **LINE_STATE** `[3:2]`
    ///
    /// USB bus line state
    #[inline(always)]
    pub const fn line_state(&self) -> u32 {
        (self.bits & SieStatus::LINE_STATE_MASK) >> SieStatus::LINE_STATE_OFFSET
    }

    /// Read **VBUS_DETECTED** `[0:0]`
    ///
    /// Device: VBUS Detected
    #[inline(always)]
    pub const fn vbus_detected(&self) -> bool {
        (self.bits & SieStatus::VBUS_DETECTED_MASK) != 0
    }

    /// Set **DATA_SEQ_ERROR** `[31:31]`
    ///
    /// Data Sequence Error.
    /// The device can raise a sequence error in the following conditions:
    /// * A SETUP packet is received followed by a DATA1 packet (data phase should always be DATA0) * An OUT packet is received from the host but doesn't match the data pid in the buffer control register read from DPSRAM
    /// The host can raise a data sequence error in the following conditions:
    /// * An IN packet from the device has the wrong data PID
    #[inline(always)]
    pub const fn with_data_seq_error(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SieStatus::DATA_SEQ_ERROR_MASK
            } else {
                self.bits & !SieStatus::DATA_SEQ_ERROR_MASK
            },
        }
    }

    /// Set **ACK_REC** `[30:30]`
    ///
    /// ACK received. Raised by both host and device.
    #[inline(always)]
    pub const fn with_ack_rec(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SieStatus::ACK_REC_MASK
            } else {
                self.bits & !SieStatus::ACK_REC_MASK
            },
        }
    }

    /// Set **STALL_REC** `[29:29]`
    ///
    /// Host: STALL received
    #[inline(always)]
    pub const fn with_stall_rec(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SieStatus::STALL_REC_MASK
            } else {
                self.bits & !SieStatus::STALL_REC_MASK
            },
        }
    }

    /// Set **NAK_REC** `[28:28]`
    ///
    /// Host: NAK received
    #[inline(always)]
    pub const fn with_nak_rec(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SieStatus::NAK_REC_MASK
            } else {
                self.bits & !SieStatus::NAK_REC_MASK
            },
        }
    }

    /// Set **RX_TIMEOUT** `[27:27]`
    ///
    /// RX timeout is raised by both the host and device if an ACK is not received in the maximum time specified by the USB spec.
    #[inline(always)]
    pub const fn with_rx_timeout(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SieStatus::RX_TIMEOUT_MASK
            } else {
                self.bits & !SieStatus::RX_TIMEOUT_MASK
            },
        }
    }

    /// Set **RX_OVERFLOW** `[26:26]`
    ///
    /// RX overflow is raised by the Serial RX engine if the incoming data is too fast.
    #[inline(always)]
    pub const fn with_rx_overflow(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SieStatus::RX_OVERFLOW_MASK
            } else {
                self.bits & !SieStatus::RX_OVERFLOW_MASK
            },
        }
    }

    /// Set **BIT_STUFF_ERROR** `[25:25]`
    ///
    /// Bit Stuff Error. Raised by the Serial RX engine.
    #[inline(always)]
    pub const fn with_bit_stuff_error(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SieStatus::BIT_STUFF_ERROR_MASK
            } else {
                self.bits & !SieStatus::BIT_STUFF_ERROR_MASK
            },
        }
    }

    /// Set **CRC_ERROR** `[24:24]`
    ///
    /// CRC Error. Raised by the Serial RX engine.
    #[inline(always)]
    pub const fn with_crc_error(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SieStatus::CRC_ERROR_MASK
            } else {
                self.bits & !SieStatus::CRC_ERROR_MASK
            },
        }
    }

    /// Set **ENDPOINT_ERROR** `[23:23]`
    ///
    /// An endpoint has encountered an error. Read the ep_rx_error and ep_tx_error registers to find out which endpoint had an error.
    #[inline(always)]
    pub const fn with_endpoint_error(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SieStatus::ENDPOINT_ERROR_MASK
            } else {
                self.bits & !SieStatus::ENDPOINT_ERROR_MASK
            },
        }
    }

    /// Set **BUS_RESET** `[19:19]`
    ///
    /// Device: bus reset received
    #[inline(always)]
    pub const fn with_bus_reset(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SieStatus::BUS_RESET_MASK
            } else {
                self.bits & !SieStatus::BUS_RESET_MASK
            },
        }
    }

    /// Set **TRANS_COMPLETE** `[18:18]`
    ///
    /// Transaction complete.
    /// Raised by device if:
    /// * An IN or OUT packet is sent with the `LAST_BUFF` bit set in the buffer control register
    /// Raised by host if:
    /// * A setup packet is sent when no data in or data out transaction follows * An IN packet is received and the `LAST_BUFF` bit is set in the buffer control register * An IN packet is received with zero length * An OUT packet is sent and the `LAST_BUFF` bit is set
    #[inline(always)]
    pub const fn with_trans_complete(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SieStatus::TRANS_COMPLETE_MASK
            } else {
                self.bits & !SieStatus::TRANS_COMPLETE_MASK
            },
        }
    }

    /// Set **SETUP_REC** `[17:17]`
    ///
    /// Device: Setup packet received
    #[inline(always)]
    pub const fn with_setup_rec(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SieStatus::SETUP_REC_MASK
            } else {
                self.bits & !SieStatus::SETUP_REC_MASK
            },
        }
    }

    /// Set **RX_SHORT_PACKET** `[12:12]`
    ///
    /// Device or Host has received a short packet. This is when the data received is less than configured in the buffer control register. Device: If using double buffered mode on device the buffer select will not be toggled after writing status back to the buffer control register. This is to prevent any further transactions on that endpoint until the user has reset the buffer control registers. Host: the current transfer will be stopped early.
    #[inline(always)]
    pub const fn with_rx_short_packet(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SieStatus::RX_SHORT_PACKET_MASK
            } else {
                self.bits & !SieStatus::RX_SHORT_PACKET_MASK
            },
        }
    }

    /// Set **RESUME** `[11:11]`
    ///
    /// Host: Device has initiated a remote resume. Device: host has initiated a resume.
    #[inline(always)]
    pub const fn with_resume(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SieStatus::RESUME_MASK
            } else {
                self.bits & !SieStatus::RESUME_MASK
            },
        }
    }

    /// Set **SUSPENDED** `[4:4]`
    ///
    /// Bus in suspended state. Valid for device and host. Host and device will go into suspend if neither Keep Alive / SOF frames are enabled.
    #[inline(always)]
    pub const fn with_suspended(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SieStatus::SUSPENDED_MASK
            } else {
                self.bits & !SieStatus::SUSPENDED_MASK
            },
        }
    }
}

impl Default for SieStatusVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SieStatus {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SieStatusVal {
        SieStatusVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SieStatusVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SieStatusVal) -> SieStatusVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// INT_EP_CTRL Register
///
/// interrupt endpoint control register
///
/// Address offset: 0x0054
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **INT_EP_ACTIVE** `[15:1]` (RW)
///
///   Host: Enable interrupt endpoint 1 -> 15
#[repr(transparent)]
pub struct IntEpCtrl {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for IntEpCtrl {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IntEpCtrl {
    // ======== Field Constants ========

    /// **INT_EP_ACTIVE**
    ///
    /// Host: Enable interrupt endpoint 1 -> 15
    ///
    /// Bits `[15:1]`, mask `0x0000fffe`
    pub const INT_EP_ACTIVE_OFFSET: u32 = 1;
    pub const INT_EP_ACTIVE_MASK: u32 = 0x0000fffe;

    // ======== Field Accessors ========

    /// **INT_EP_ACTIVE** `[15:1]` (RW)
    ///
    /// Host: Enable interrupt endpoint 1 -> 15
    #[inline(always)]
    pub fn int_ep_active(&self) -> u32 {
        (self.value.read_bits() & Self::INT_EP_ACTIVE_MASK) >> Self::INT_EP_ACTIVE_OFFSET
    }

    /// Write **INT_EP_ACTIVE** `[15:1]` (RW)
    ///
    /// Host: Enable interrupt endpoint 1 -> 15
    #[inline(always)]
    pub fn write_int_ep_active(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::INT_EP_ACTIVE_MASK)
                | ((value << Self::INT_EP_ACTIVE_OFFSET) & Self::INT_EP_ACTIVE_MASK)
        });
    }
}

/// INT_EP_CTRL value type for batch read/modify/write operations
///
/// Use `intepctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `intepctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntEpCtrlVal {
    bits: u32,
}

impl IntEpCtrlVal {
    /// Create from raw bits
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

    /// Read **INT_EP_ACTIVE** `[15:1]`
    ///
    /// Host: Enable interrupt endpoint 1 -> 15
    #[inline(always)]
    pub const fn int_ep_active(&self) -> u32 {
        (self.bits & IntEpCtrl::INT_EP_ACTIVE_MASK) >> IntEpCtrl::INT_EP_ACTIVE_OFFSET
    }

    /// Set **INT_EP_ACTIVE** `[15:1]`
    ///
    /// Host: Enable interrupt endpoint 1 -> 15
    #[inline(always)]
    pub const fn with_int_ep_active(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !IntEpCtrl::INT_EP_ACTIVE_MASK)
                | ((value << IntEpCtrl::INT_EP_ACTIVE_OFFSET) & IntEpCtrl::INT_EP_ACTIVE_MASK),
        }
    }
}

impl Default for IntEpCtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl IntEpCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IntEpCtrlVal {
        IntEpCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IntEpCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IntEpCtrlVal) -> IntEpCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// BUFF_STATUS Register
///
/// Buffer status register. A bit set here indicates that a buffer has completed on the endpoint (if the buffer interrupt is enabled). It is possible for 2 buffers to be completed, so clearing the buffer status bit may instantly re set it on the next clock cycle.
///
/// Address offset: 0x0058
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **EP15_OUT** `[31]` (RW)
///
/// - **EP15_IN** `[30]` (RW)
///
/// - **EP14_OUT** `[29]` (RW)
///
/// - **EP14_IN** `[28]` (RW)
///
/// - **EP13_OUT** `[27]` (RW)
///
/// - **EP13_IN** `[26]` (RW)
///
/// - **EP12_OUT** `[25]` (RW)
///
/// - **EP12_IN** `[24]` (RW)
///
/// - **EP11_OUT** `[23]` (RW)
///
/// - **EP11_IN** `[22]` (RW)
///
/// - **EP10_OUT** `[21]` (RW)
///
/// - **EP10_IN** `[20]` (RW)
///
/// - **EP9_OUT** `[19]` (RW)
///
/// - **EP9_IN** `[18]` (RW)
///
/// - **EP8_OUT** `[17]` (RW)
///
/// - **EP8_IN** `[16]` (RW)
///
/// - **EP7_OUT** `[15]` (RW)
///
/// - **EP7_IN** `[14]` (RW)
///
/// - **EP6_OUT** `[13]` (RW)
///
/// - **EP6_IN** `[12]` (RW)
///
/// - **EP5_OUT** `[11]` (RW)
///
/// - **EP5_IN** `[10]` (RW)
///
/// - **EP4_OUT** `[9]` (RW)
///
/// - **EP4_IN** `[8]` (RW)
///
/// - **EP3_OUT** `[7]` (RW)
///
/// - **EP3_IN** `[6]` (RW)
///
/// - **EP2_OUT** `[5]` (RW)
///
/// - **EP2_IN** `[4]` (RW)
///
/// - **EP1_OUT** `[3]` (RW)
///
/// - **EP1_IN** `[2]` (RW)
///
/// - **EP0_OUT** `[1]` (RW)
///
/// - **EP0_IN** `[0]` (RW)
#[repr(transparent)]
pub struct BuffStatus {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for BuffStatus {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl BuffStatus {
    // ======== Field Constants ========

    /// **EP15_OUT**
    /// Bits `[31:31]`, mask `0x80000000`
    pub const EP15_OUT_OFFSET: u32 = 31;
    pub const EP15_OUT_MASK: u32 = 0x80000000;
    pub const EP15_OUT: u32 = 1 << 31;

    /// **EP15_IN**
    /// Bits `[30:30]`, mask `0x40000000`
    pub const EP15_IN_OFFSET: u32 = 30;
    pub const EP15_IN_MASK: u32 = 0x40000000;
    pub const EP15_IN: u32 = 1 << 30;

    /// **EP14_OUT**
    /// Bits `[29:29]`, mask `0x20000000`
    pub const EP14_OUT_OFFSET: u32 = 29;
    pub const EP14_OUT_MASK: u32 = 0x20000000;
    pub const EP14_OUT: u32 = 1 << 29;

    /// **EP14_IN**
    /// Bits `[28:28]`, mask `0x10000000`
    pub const EP14_IN_OFFSET: u32 = 28;
    pub const EP14_IN_MASK: u32 = 0x10000000;
    pub const EP14_IN: u32 = 1 << 28;

    /// **EP13_OUT**
    /// Bits `[27:27]`, mask `0x08000000`
    pub const EP13_OUT_OFFSET: u32 = 27;
    pub const EP13_OUT_MASK: u32 = 0x08000000;
    pub const EP13_OUT: u32 = 1 << 27;

    /// **EP13_IN**
    /// Bits `[26:26]`, mask `0x04000000`
    pub const EP13_IN_OFFSET: u32 = 26;
    pub const EP13_IN_MASK: u32 = 0x04000000;
    pub const EP13_IN: u32 = 1 << 26;

    /// **EP12_OUT**
    /// Bits `[25:25]`, mask `0x02000000`
    pub const EP12_OUT_OFFSET: u32 = 25;
    pub const EP12_OUT_MASK: u32 = 0x02000000;
    pub const EP12_OUT: u32 = 1 << 25;

    /// **EP12_IN**
    /// Bits `[24:24]`, mask `0x01000000`
    pub const EP12_IN_OFFSET: u32 = 24;
    pub const EP12_IN_MASK: u32 = 0x01000000;
    pub const EP12_IN: u32 = 1 << 24;

    /// **EP11_OUT**
    /// Bits `[23:23]`, mask `0x00800000`
    pub const EP11_OUT_OFFSET: u32 = 23;
    pub const EP11_OUT_MASK: u32 = 0x00800000;
    pub const EP11_OUT: u32 = 1 << 23;

    /// **EP11_IN**
    /// Bits `[22:22]`, mask `0x00400000`
    pub const EP11_IN_OFFSET: u32 = 22;
    pub const EP11_IN_MASK: u32 = 0x00400000;
    pub const EP11_IN: u32 = 1 << 22;

    /// **EP10_OUT**
    /// Bits `[21:21]`, mask `0x00200000`
    pub const EP10_OUT_OFFSET: u32 = 21;
    pub const EP10_OUT_MASK: u32 = 0x00200000;
    pub const EP10_OUT: u32 = 1 << 21;

    /// **EP10_IN**
    /// Bits `[20:20]`, mask `0x00100000`
    pub const EP10_IN_OFFSET: u32 = 20;
    pub const EP10_IN_MASK: u32 = 0x00100000;
    pub const EP10_IN: u32 = 1 << 20;

    /// **EP9_OUT**
    /// Bits `[19:19]`, mask `0x00080000`
    pub const EP9_OUT_OFFSET: u32 = 19;
    pub const EP9_OUT_MASK: u32 = 0x00080000;
    pub const EP9_OUT: u32 = 1 << 19;

    /// **EP9_IN**
    /// Bits `[18:18]`, mask `0x00040000`
    pub const EP9_IN_OFFSET: u32 = 18;
    pub const EP9_IN_MASK: u32 = 0x00040000;
    pub const EP9_IN: u32 = 1 << 18;

    /// **EP8_OUT**
    /// Bits `[17:17]`, mask `0x00020000`
    pub const EP8_OUT_OFFSET: u32 = 17;
    pub const EP8_OUT_MASK: u32 = 0x00020000;
    pub const EP8_OUT: u32 = 1 << 17;

    /// **EP8_IN**
    /// Bits `[16:16]`, mask `0x00010000`
    pub const EP8_IN_OFFSET: u32 = 16;
    pub const EP8_IN_MASK: u32 = 0x00010000;
    pub const EP8_IN: u32 = 1 << 16;

    /// **EP7_OUT**
    /// Bits `[15:15]`, mask `0x00008000`
    pub const EP7_OUT_OFFSET: u32 = 15;
    pub const EP7_OUT_MASK: u32 = 0x00008000;
    pub const EP7_OUT: u32 = 1 << 15;

    /// **EP7_IN**
    /// Bits `[14:14]`, mask `0x00004000`
    pub const EP7_IN_OFFSET: u32 = 14;
    pub const EP7_IN_MASK: u32 = 0x00004000;
    pub const EP7_IN: u32 = 1 << 14;

    /// **EP6_OUT**
    /// Bits `[13:13]`, mask `0x00002000`
    pub const EP6_OUT_OFFSET: u32 = 13;
    pub const EP6_OUT_MASK: u32 = 0x00002000;
    pub const EP6_OUT: u32 = 1 << 13;

    /// **EP6_IN**
    /// Bits `[12:12]`, mask `0x00001000`
    pub const EP6_IN_OFFSET: u32 = 12;
    pub const EP6_IN_MASK: u32 = 0x00001000;
    pub const EP6_IN: u32 = 1 << 12;

    /// **EP5_OUT**
    /// Bits `[11:11]`, mask `0x00000800`
    pub const EP5_OUT_OFFSET: u32 = 11;
    pub const EP5_OUT_MASK: u32 = 0x00000800;
    pub const EP5_OUT: u32 = 1 << 11;

    /// **EP5_IN**
    /// Bits `[10:10]`, mask `0x00000400`
    pub const EP5_IN_OFFSET: u32 = 10;
    pub const EP5_IN_MASK: u32 = 0x00000400;
    pub const EP5_IN: u32 = 1 << 10;

    /// **EP4_OUT**
    /// Bits `[9:9]`, mask `0x00000200`
    pub const EP4_OUT_OFFSET: u32 = 9;
    pub const EP4_OUT_MASK: u32 = 0x00000200;
    pub const EP4_OUT: u32 = 1 << 9;

    /// **EP4_IN**
    /// Bits `[8:8]`, mask `0x00000100`
    pub const EP4_IN_OFFSET: u32 = 8;
    pub const EP4_IN_MASK: u32 = 0x00000100;
    pub const EP4_IN: u32 = 1 << 8;

    /// **EP3_OUT**
    /// Bits `[7:7]`, mask `0x00000080`
    pub const EP3_OUT_OFFSET: u32 = 7;
    pub const EP3_OUT_MASK: u32 = 0x00000080;
    pub const EP3_OUT: u32 = 1 << 7;

    /// **EP3_IN**
    /// Bits `[6:6]`, mask `0x00000040`
    pub const EP3_IN_OFFSET: u32 = 6;
    pub const EP3_IN_MASK: u32 = 0x00000040;
    pub const EP3_IN: u32 = 1 << 6;

    /// **EP2_OUT**
    /// Bits `[5:5]`, mask `0x00000020`
    pub const EP2_OUT_OFFSET: u32 = 5;
    pub const EP2_OUT_MASK: u32 = 0x00000020;
    pub const EP2_OUT: u32 = 1 << 5;

    /// **EP2_IN**
    /// Bits `[4:4]`, mask `0x00000010`
    pub const EP2_IN_OFFSET: u32 = 4;
    pub const EP2_IN_MASK: u32 = 0x00000010;
    pub const EP2_IN: u32 = 1 << 4;

    /// **EP1_OUT**
    /// Bits `[3:3]`, mask `0x00000008`
    pub const EP1_OUT_OFFSET: u32 = 3;
    pub const EP1_OUT_MASK: u32 = 0x00000008;
    pub const EP1_OUT: u32 = 1 << 3;

    /// **EP1_IN**
    /// Bits `[2:2]`, mask `0x00000004`
    pub const EP1_IN_OFFSET: u32 = 2;
    pub const EP1_IN_MASK: u32 = 0x00000004;
    pub const EP1_IN: u32 = 1 << 2;

    /// **EP0_OUT**
    /// Bits `[1:1]`, mask `0x00000002`
    pub const EP0_OUT_OFFSET: u32 = 1;
    pub const EP0_OUT_MASK: u32 = 0x00000002;
    pub const EP0_OUT: u32 = 1 << 1;

    /// **EP0_IN**
    /// Bits `[0:0]`, mask `0x00000001`
    pub const EP0_IN_OFFSET: u32 = 0;
    pub const EP0_IN_MASK: u32 = 0x00000001;
    pub const EP0_IN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **EP15_OUT** `[31]` (RW)
    #[inline(always)]
    pub fn ep15_out(&self) -> bool {
        (self.value.read_bits() & Self::EP15_OUT_MASK) != 0
    }

    /// Set EP15_OUT bit atomically (RW)
    #[inline(always)]
    pub fn set_ep15_out(&self) {
        self.set_bits(Self::EP15_OUT_MASK);
    }

    /// Clear EP15_OUT bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep15_out(&self) {
        self.clr_bits(Self::EP15_OUT_MASK);
    }

    /// **EP15_IN** `[30]` (RW)
    #[inline(always)]
    pub fn ep15_in(&self) -> bool {
        (self.value.read_bits() & Self::EP15_IN_MASK) != 0
    }

    /// Set EP15_IN bit atomically (RW)
    #[inline(always)]
    pub fn set_ep15_in(&self) {
        self.set_bits(Self::EP15_IN_MASK);
    }

    /// Clear EP15_IN bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep15_in(&self) {
        self.clr_bits(Self::EP15_IN_MASK);
    }

    /// **EP14_OUT** `[29]` (RW)
    #[inline(always)]
    pub fn ep14_out(&self) -> bool {
        (self.value.read_bits() & Self::EP14_OUT_MASK) != 0
    }

    /// Set EP14_OUT bit atomically (RW)
    #[inline(always)]
    pub fn set_ep14_out(&self) {
        self.set_bits(Self::EP14_OUT_MASK);
    }

    /// Clear EP14_OUT bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep14_out(&self) {
        self.clr_bits(Self::EP14_OUT_MASK);
    }

    /// **EP14_IN** `[28]` (RW)
    #[inline(always)]
    pub fn ep14_in(&self) -> bool {
        (self.value.read_bits() & Self::EP14_IN_MASK) != 0
    }

    /// Set EP14_IN bit atomically (RW)
    #[inline(always)]
    pub fn set_ep14_in(&self) {
        self.set_bits(Self::EP14_IN_MASK);
    }

    /// Clear EP14_IN bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep14_in(&self) {
        self.clr_bits(Self::EP14_IN_MASK);
    }

    /// **EP13_OUT** `[27]` (RW)
    #[inline(always)]
    pub fn ep13_out(&self) -> bool {
        (self.value.read_bits() & Self::EP13_OUT_MASK) != 0
    }

    /// Set EP13_OUT bit atomically (RW)
    #[inline(always)]
    pub fn set_ep13_out(&self) {
        self.set_bits(Self::EP13_OUT_MASK);
    }

    /// Clear EP13_OUT bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep13_out(&self) {
        self.clr_bits(Self::EP13_OUT_MASK);
    }

    /// **EP13_IN** `[26]` (RW)
    #[inline(always)]
    pub fn ep13_in(&self) -> bool {
        (self.value.read_bits() & Self::EP13_IN_MASK) != 0
    }

    /// Set EP13_IN bit atomically (RW)
    #[inline(always)]
    pub fn set_ep13_in(&self) {
        self.set_bits(Self::EP13_IN_MASK);
    }

    /// Clear EP13_IN bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep13_in(&self) {
        self.clr_bits(Self::EP13_IN_MASK);
    }

    /// **EP12_OUT** `[25]` (RW)
    #[inline(always)]
    pub fn ep12_out(&self) -> bool {
        (self.value.read_bits() & Self::EP12_OUT_MASK) != 0
    }

    /// Set EP12_OUT bit atomically (RW)
    #[inline(always)]
    pub fn set_ep12_out(&self) {
        self.set_bits(Self::EP12_OUT_MASK);
    }

    /// Clear EP12_OUT bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep12_out(&self) {
        self.clr_bits(Self::EP12_OUT_MASK);
    }

    /// **EP12_IN** `[24]` (RW)
    #[inline(always)]
    pub fn ep12_in(&self) -> bool {
        (self.value.read_bits() & Self::EP12_IN_MASK) != 0
    }

    /// Set EP12_IN bit atomically (RW)
    #[inline(always)]
    pub fn set_ep12_in(&self) {
        self.set_bits(Self::EP12_IN_MASK);
    }

    /// Clear EP12_IN bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep12_in(&self) {
        self.clr_bits(Self::EP12_IN_MASK);
    }

    /// **EP11_OUT** `[23]` (RW)
    #[inline(always)]
    pub fn ep11_out(&self) -> bool {
        (self.value.read_bits() & Self::EP11_OUT_MASK) != 0
    }

    /// Set EP11_OUT bit atomically (RW)
    #[inline(always)]
    pub fn set_ep11_out(&self) {
        self.set_bits(Self::EP11_OUT_MASK);
    }

    /// Clear EP11_OUT bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep11_out(&self) {
        self.clr_bits(Self::EP11_OUT_MASK);
    }

    /// **EP11_IN** `[22]` (RW)
    #[inline(always)]
    pub fn ep11_in(&self) -> bool {
        (self.value.read_bits() & Self::EP11_IN_MASK) != 0
    }

    /// Set EP11_IN bit atomically (RW)
    #[inline(always)]
    pub fn set_ep11_in(&self) {
        self.set_bits(Self::EP11_IN_MASK);
    }

    /// Clear EP11_IN bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep11_in(&self) {
        self.clr_bits(Self::EP11_IN_MASK);
    }

    /// **EP10_OUT** `[21]` (RW)
    #[inline(always)]
    pub fn ep10_out(&self) -> bool {
        (self.value.read_bits() & Self::EP10_OUT_MASK) != 0
    }

    /// Set EP10_OUT bit atomically (RW)
    #[inline(always)]
    pub fn set_ep10_out(&self) {
        self.set_bits(Self::EP10_OUT_MASK);
    }

    /// Clear EP10_OUT bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep10_out(&self) {
        self.clr_bits(Self::EP10_OUT_MASK);
    }

    /// **EP10_IN** `[20]` (RW)
    #[inline(always)]
    pub fn ep10_in(&self) -> bool {
        (self.value.read_bits() & Self::EP10_IN_MASK) != 0
    }

    /// Set EP10_IN bit atomically (RW)
    #[inline(always)]
    pub fn set_ep10_in(&self) {
        self.set_bits(Self::EP10_IN_MASK);
    }

    /// Clear EP10_IN bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep10_in(&self) {
        self.clr_bits(Self::EP10_IN_MASK);
    }

    /// **EP9_OUT** `[19]` (RW)
    #[inline(always)]
    pub fn ep9_out(&self) -> bool {
        (self.value.read_bits() & Self::EP9_OUT_MASK) != 0
    }

    /// Set EP9_OUT bit atomically (RW)
    #[inline(always)]
    pub fn set_ep9_out(&self) {
        self.set_bits(Self::EP9_OUT_MASK);
    }

    /// Clear EP9_OUT bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep9_out(&self) {
        self.clr_bits(Self::EP9_OUT_MASK);
    }

    /// **EP9_IN** `[18]` (RW)
    #[inline(always)]
    pub fn ep9_in(&self) -> bool {
        (self.value.read_bits() & Self::EP9_IN_MASK) != 0
    }

    /// Set EP9_IN bit atomically (RW)
    #[inline(always)]
    pub fn set_ep9_in(&self) {
        self.set_bits(Self::EP9_IN_MASK);
    }

    /// Clear EP9_IN bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep9_in(&self) {
        self.clr_bits(Self::EP9_IN_MASK);
    }

    /// **EP8_OUT** `[17]` (RW)
    #[inline(always)]
    pub fn ep8_out(&self) -> bool {
        (self.value.read_bits() & Self::EP8_OUT_MASK) != 0
    }

    /// Set EP8_OUT bit atomically (RW)
    #[inline(always)]
    pub fn set_ep8_out(&self) {
        self.set_bits(Self::EP8_OUT_MASK);
    }

    /// Clear EP8_OUT bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep8_out(&self) {
        self.clr_bits(Self::EP8_OUT_MASK);
    }

    /// **EP8_IN** `[16]` (RW)
    #[inline(always)]
    pub fn ep8_in(&self) -> bool {
        (self.value.read_bits() & Self::EP8_IN_MASK) != 0
    }

    /// Set EP8_IN bit atomically (RW)
    #[inline(always)]
    pub fn set_ep8_in(&self) {
        self.set_bits(Self::EP8_IN_MASK);
    }

    /// Clear EP8_IN bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep8_in(&self) {
        self.clr_bits(Self::EP8_IN_MASK);
    }

    /// **EP7_OUT** `[15]` (RW)
    #[inline(always)]
    pub fn ep7_out(&self) -> bool {
        (self.value.read_bits() & Self::EP7_OUT_MASK) != 0
    }

    /// Set EP7_OUT bit atomically (RW)
    #[inline(always)]
    pub fn set_ep7_out(&self) {
        self.set_bits(Self::EP7_OUT_MASK);
    }

    /// Clear EP7_OUT bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep7_out(&self) {
        self.clr_bits(Self::EP7_OUT_MASK);
    }

    /// **EP7_IN** `[14]` (RW)
    #[inline(always)]
    pub fn ep7_in(&self) -> bool {
        (self.value.read_bits() & Self::EP7_IN_MASK) != 0
    }

    /// Set EP7_IN bit atomically (RW)
    #[inline(always)]
    pub fn set_ep7_in(&self) {
        self.set_bits(Self::EP7_IN_MASK);
    }

    /// Clear EP7_IN bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep7_in(&self) {
        self.clr_bits(Self::EP7_IN_MASK);
    }

    /// **EP6_OUT** `[13]` (RW)
    #[inline(always)]
    pub fn ep6_out(&self) -> bool {
        (self.value.read_bits() & Self::EP6_OUT_MASK) != 0
    }

    /// Set EP6_OUT bit atomically (RW)
    #[inline(always)]
    pub fn set_ep6_out(&self) {
        self.set_bits(Self::EP6_OUT_MASK);
    }

    /// Clear EP6_OUT bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep6_out(&self) {
        self.clr_bits(Self::EP6_OUT_MASK);
    }

    /// **EP6_IN** `[12]` (RW)
    #[inline(always)]
    pub fn ep6_in(&self) -> bool {
        (self.value.read_bits() & Self::EP6_IN_MASK) != 0
    }

    /// Set EP6_IN bit atomically (RW)
    #[inline(always)]
    pub fn set_ep6_in(&self) {
        self.set_bits(Self::EP6_IN_MASK);
    }

    /// Clear EP6_IN bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep6_in(&self) {
        self.clr_bits(Self::EP6_IN_MASK);
    }

    /// **EP5_OUT** `[11]` (RW)
    #[inline(always)]
    pub fn ep5_out(&self) -> bool {
        (self.value.read_bits() & Self::EP5_OUT_MASK) != 0
    }

    /// Set EP5_OUT bit atomically (RW)
    #[inline(always)]
    pub fn set_ep5_out(&self) {
        self.set_bits(Self::EP5_OUT_MASK);
    }

    /// Clear EP5_OUT bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep5_out(&self) {
        self.clr_bits(Self::EP5_OUT_MASK);
    }

    /// **EP5_IN** `[10]` (RW)
    #[inline(always)]
    pub fn ep5_in(&self) -> bool {
        (self.value.read_bits() & Self::EP5_IN_MASK) != 0
    }

    /// Set EP5_IN bit atomically (RW)
    #[inline(always)]
    pub fn set_ep5_in(&self) {
        self.set_bits(Self::EP5_IN_MASK);
    }

    /// Clear EP5_IN bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep5_in(&self) {
        self.clr_bits(Self::EP5_IN_MASK);
    }

    /// **EP4_OUT** `[9]` (RW)
    #[inline(always)]
    pub fn ep4_out(&self) -> bool {
        (self.value.read_bits() & Self::EP4_OUT_MASK) != 0
    }

    /// Set EP4_OUT bit atomically (RW)
    #[inline(always)]
    pub fn set_ep4_out(&self) {
        self.set_bits(Self::EP4_OUT_MASK);
    }

    /// Clear EP4_OUT bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep4_out(&self) {
        self.clr_bits(Self::EP4_OUT_MASK);
    }

    /// **EP4_IN** `[8]` (RW)
    #[inline(always)]
    pub fn ep4_in(&self) -> bool {
        (self.value.read_bits() & Self::EP4_IN_MASK) != 0
    }

    /// Set EP4_IN bit atomically (RW)
    #[inline(always)]
    pub fn set_ep4_in(&self) {
        self.set_bits(Self::EP4_IN_MASK);
    }

    /// Clear EP4_IN bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep4_in(&self) {
        self.clr_bits(Self::EP4_IN_MASK);
    }

    /// **EP3_OUT** `[7]` (RW)
    #[inline(always)]
    pub fn ep3_out(&self) -> bool {
        (self.value.read_bits() & Self::EP3_OUT_MASK) != 0
    }

    /// Set EP3_OUT bit atomically (RW)
    #[inline(always)]
    pub fn set_ep3_out(&self) {
        self.set_bits(Self::EP3_OUT_MASK);
    }

    /// Clear EP3_OUT bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep3_out(&self) {
        self.clr_bits(Self::EP3_OUT_MASK);
    }

    /// **EP3_IN** `[6]` (RW)
    #[inline(always)]
    pub fn ep3_in(&self) -> bool {
        (self.value.read_bits() & Self::EP3_IN_MASK) != 0
    }

    /// Set EP3_IN bit atomically (RW)
    #[inline(always)]
    pub fn set_ep3_in(&self) {
        self.set_bits(Self::EP3_IN_MASK);
    }

    /// Clear EP3_IN bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep3_in(&self) {
        self.clr_bits(Self::EP3_IN_MASK);
    }

    /// **EP2_OUT** `[5]` (RW)
    #[inline(always)]
    pub fn ep2_out(&self) -> bool {
        (self.value.read_bits() & Self::EP2_OUT_MASK) != 0
    }

    /// Set EP2_OUT bit atomically (RW)
    #[inline(always)]
    pub fn set_ep2_out(&self) {
        self.set_bits(Self::EP2_OUT_MASK);
    }

    /// Clear EP2_OUT bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep2_out(&self) {
        self.clr_bits(Self::EP2_OUT_MASK);
    }

    /// **EP2_IN** `[4]` (RW)
    #[inline(always)]
    pub fn ep2_in(&self) -> bool {
        (self.value.read_bits() & Self::EP2_IN_MASK) != 0
    }

    /// Set EP2_IN bit atomically (RW)
    #[inline(always)]
    pub fn set_ep2_in(&self) {
        self.set_bits(Self::EP2_IN_MASK);
    }

    /// Clear EP2_IN bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep2_in(&self) {
        self.clr_bits(Self::EP2_IN_MASK);
    }

    /// **EP1_OUT** `[3]` (RW)
    #[inline(always)]
    pub fn ep1_out(&self) -> bool {
        (self.value.read_bits() & Self::EP1_OUT_MASK) != 0
    }

    /// Set EP1_OUT bit atomically (RW)
    #[inline(always)]
    pub fn set_ep1_out(&self) {
        self.set_bits(Self::EP1_OUT_MASK);
    }

    /// Clear EP1_OUT bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep1_out(&self) {
        self.clr_bits(Self::EP1_OUT_MASK);
    }

    /// **EP1_IN** `[2]` (RW)
    #[inline(always)]
    pub fn ep1_in(&self) -> bool {
        (self.value.read_bits() & Self::EP1_IN_MASK) != 0
    }

    /// Set EP1_IN bit atomically (RW)
    #[inline(always)]
    pub fn set_ep1_in(&self) {
        self.set_bits(Self::EP1_IN_MASK);
    }

    /// Clear EP1_IN bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep1_in(&self) {
        self.clr_bits(Self::EP1_IN_MASK);
    }

    /// **EP0_OUT** `[1]` (RW)
    #[inline(always)]
    pub fn ep0_out(&self) -> bool {
        (self.value.read_bits() & Self::EP0_OUT_MASK) != 0
    }

    /// Set EP0_OUT bit atomically (RW)
    #[inline(always)]
    pub fn set_ep0_out(&self) {
        self.set_bits(Self::EP0_OUT_MASK);
    }

    /// Clear EP0_OUT bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep0_out(&self) {
        self.clr_bits(Self::EP0_OUT_MASK);
    }

    /// **EP0_IN** `[0]` (RW)
    #[inline(always)]
    pub fn ep0_in(&self) -> bool {
        (self.value.read_bits() & Self::EP0_IN_MASK) != 0
    }

    /// Set EP0_IN bit atomically (RW)
    #[inline(always)]
    pub fn set_ep0_in(&self) {
        self.set_bits(Self::EP0_IN_MASK);
    }

    /// Clear EP0_IN bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep0_in(&self) {
        self.clr_bits(Self::EP0_IN_MASK);
    }
}

/// BUFF_STATUS value type for batch read/modify/write operations
///
/// Use `buffstatus.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `buffstatus.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BuffStatusVal {
    bits: u32,
}

impl BuffStatusVal {
    /// Create from raw bits
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

    /// Read **EP15_OUT** `[31:31]`
    #[inline(always)]
    pub const fn ep15_out(&self) -> bool {
        (self.bits & BuffStatus::EP15_OUT_MASK) != 0
    }

    /// Read **EP15_IN** `[30:30]`
    #[inline(always)]
    pub const fn ep15_in(&self) -> bool {
        (self.bits & BuffStatus::EP15_IN_MASK) != 0
    }

    /// Read **EP14_OUT** `[29:29]`
    #[inline(always)]
    pub const fn ep14_out(&self) -> bool {
        (self.bits & BuffStatus::EP14_OUT_MASK) != 0
    }

    /// Read **EP14_IN** `[28:28]`
    #[inline(always)]
    pub const fn ep14_in(&self) -> bool {
        (self.bits & BuffStatus::EP14_IN_MASK) != 0
    }

    /// Read **EP13_OUT** `[27:27]`
    #[inline(always)]
    pub const fn ep13_out(&self) -> bool {
        (self.bits & BuffStatus::EP13_OUT_MASK) != 0
    }

    /// Read **EP13_IN** `[26:26]`
    #[inline(always)]
    pub const fn ep13_in(&self) -> bool {
        (self.bits & BuffStatus::EP13_IN_MASK) != 0
    }

    /// Read **EP12_OUT** `[25:25]`
    #[inline(always)]
    pub const fn ep12_out(&self) -> bool {
        (self.bits & BuffStatus::EP12_OUT_MASK) != 0
    }

    /// Read **EP12_IN** `[24:24]`
    #[inline(always)]
    pub const fn ep12_in(&self) -> bool {
        (self.bits & BuffStatus::EP12_IN_MASK) != 0
    }

    /// Read **EP11_OUT** `[23:23]`
    #[inline(always)]
    pub const fn ep11_out(&self) -> bool {
        (self.bits & BuffStatus::EP11_OUT_MASK) != 0
    }

    /// Read **EP11_IN** `[22:22]`
    #[inline(always)]
    pub const fn ep11_in(&self) -> bool {
        (self.bits & BuffStatus::EP11_IN_MASK) != 0
    }

    /// Read **EP10_OUT** `[21:21]`
    #[inline(always)]
    pub const fn ep10_out(&self) -> bool {
        (self.bits & BuffStatus::EP10_OUT_MASK) != 0
    }

    /// Read **EP10_IN** `[20:20]`
    #[inline(always)]
    pub const fn ep10_in(&self) -> bool {
        (self.bits & BuffStatus::EP10_IN_MASK) != 0
    }

    /// Read **EP9_OUT** `[19:19]`
    #[inline(always)]
    pub const fn ep9_out(&self) -> bool {
        (self.bits & BuffStatus::EP9_OUT_MASK) != 0
    }

    /// Read **EP9_IN** `[18:18]`
    #[inline(always)]
    pub const fn ep9_in(&self) -> bool {
        (self.bits & BuffStatus::EP9_IN_MASK) != 0
    }

    /// Read **EP8_OUT** `[17:17]`
    #[inline(always)]
    pub const fn ep8_out(&self) -> bool {
        (self.bits & BuffStatus::EP8_OUT_MASK) != 0
    }

    /// Read **EP8_IN** `[16:16]`
    #[inline(always)]
    pub const fn ep8_in(&self) -> bool {
        (self.bits & BuffStatus::EP8_IN_MASK) != 0
    }

    /// Read **EP7_OUT** `[15:15]`
    #[inline(always)]
    pub const fn ep7_out(&self) -> bool {
        (self.bits & BuffStatus::EP7_OUT_MASK) != 0
    }

    /// Read **EP7_IN** `[14:14]`
    #[inline(always)]
    pub const fn ep7_in(&self) -> bool {
        (self.bits & BuffStatus::EP7_IN_MASK) != 0
    }

    /// Read **EP6_OUT** `[13:13]`
    #[inline(always)]
    pub const fn ep6_out(&self) -> bool {
        (self.bits & BuffStatus::EP6_OUT_MASK) != 0
    }

    /// Read **EP6_IN** `[12:12]`
    #[inline(always)]
    pub const fn ep6_in(&self) -> bool {
        (self.bits & BuffStatus::EP6_IN_MASK) != 0
    }

    /// Read **EP5_OUT** `[11:11]`
    #[inline(always)]
    pub const fn ep5_out(&self) -> bool {
        (self.bits & BuffStatus::EP5_OUT_MASK) != 0
    }

    /// Read **EP5_IN** `[10:10]`
    #[inline(always)]
    pub const fn ep5_in(&self) -> bool {
        (self.bits & BuffStatus::EP5_IN_MASK) != 0
    }

    /// Read **EP4_OUT** `[9:9]`
    #[inline(always)]
    pub const fn ep4_out(&self) -> bool {
        (self.bits & BuffStatus::EP4_OUT_MASK) != 0
    }

    /// Read **EP4_IN** `[8:8]`
    #[inline(always)]
    pub const fn ep4_in(&self) -> bool {
        (self.bits & BuffStatus::EP4_IN_MASK) != 0
    }

    /// Read **EP3_OUT** `[7:7]`
    #[inline(always)]
    pub const fn ep3_out(&self) -> bool {
        (self.bits & BuffStatus::EP3_OUT_MASK) != 0
    }

    /// Read **EP3_IN** `[6:6]`
    #[inline(always)]
    pub const fn ep3_in(&self) -> bool {
        (self.bits & BuffStatus::EP3_IN_MASK) != 0
    }

    /// Read **EP2_OUT** `[5:5]`
    #[inline(always)]
    pub const fn ep2_out(&self) -> bool {
        (self.bits & BuffStatus::EP2_OUT_MASK) != 0
    }

    /// Read **EP2_IN** `[4:4]`
    #[inline(always)]
    pub const fn ep2_in(&self) -> bool {
        (self.bits & BuffStatus::EP2_IN_MASK) != 0
    }

    /// Read **EP1_OUT** `[3:3]`
    #[inline(always)]
    pub const fn ep1_out(&self) -> bool {
        (self.bits & BuffStatus::EP1_OUT_MASK) != 0
    }

    /// Read **EP1_IN** `[2:2]`
    #[inline(always)]
    pub const fn ep1_in(&self) -> bool {
        (self.bits & BuffStatus::EP1_IN_MASK) != 0
    }

    /// Read **EP0_OUT** `[1:1]`
    #[inline(always)]
    pub const fn ep0_out(&self) -> bool {
        (self.bits & BuffStatus::EP0_OUT_MASK) != 0
    }

    /// Read **EP0_IN** `[0:0]`
    #[inline(always)]
    pub const fn ep0_in(&self) -> bool {
        (self.bits & BuffStatus::EP0_IN_MASK) != 0
    }

    /// Set **EP15_OUT** `[31:31]`
    #[inline(always)]
    pub const fn with_ep15_out(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | BuffStatus::EP15_OUT_MASK
            } else {
                self.bits & !BuffStatus::EP15_OUT_MASK
            },
        }
    }

    /// Set **EP15_IN** `[30:30]`
    #[inline(always)]
    pub const fn with_ep15_in(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | BuffStatus::EP15_IN_MASK
            } else {
                self.bits & !BuffStatus::EP15_IN_MASK
            },
        }
    }

    /// Set **EP14_OUT** `[29:29]`
    #[inline(always)]
    pub const fn with_ep14_out(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | BuffStatus::EP14_OUT_MASK
            } else {
                self.bits & !BuffStatus::EP14_OUT_MASK
            },
        }
    }

    /// Set **EP14_IN** `[28:28]`
    #[inline(always)]
    pub const fn with_ep14_in(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | BuffStatus::EP14_IN_MASK
            } else {
                self.bits & !BuffStatus::EP14_IN_MASK
            },
        }
    }

    /// Set **EP13_OUT** `[27:27]`
    #[inline(always)]
    pub const fn with_ep13_out(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | BuffStatus::EP13_OUT_MASK
            } else {
                self.bits & !BuffStatus::EP13_OUT_MASK
            },
        }
    }

    /// Set **EP13_IN** `[26:26]`
    #[inline(always)]
    pub const fn with_ep13_in(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | BuffStatus::EP13_IN_MASK
            } else {
                self.bits & !BuffStatus::EP13_IN_MASK
            },
        }
    }

    /// Set **EP12_OUT** `[25:25]`
    #[inline(always)]
    pub const fn with_ep12_out(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | BuffStatus::EP12_OUT_MASK
            } else {
                self.bits & !BuffStatus::EP12_OUT_MASK
            },
        }
    }

    /// Set **EP12_IN** `[24:24]`
    #[inline(always)]
    pub const fn with_ep12_in(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | BuffStatus::EP12_IN_MASK
            } else {
                self.bits & !BuffStatus::EP12_IN_MASK
            },
        }
    }

    /// Set **EP11_OUT** `[23:23]`
    #[inline(always)]
    pub const fn with_ep11_out(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | BuffStatus::EP11_OUT_MASK
            } else {
                self.bits & !BuffStatus::EP11_OUT_MASK
            },
        }
    }

    /// Set **EP11_IN** `[22:22]`
    #[inline(always)]
    pub const fn with_ep11_in(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | BuffStatus::EP11_IN_MASK
            } else {
                self.bits & !BuffStatus::EP11_IN_MASK
            },
        }
    }

    /// Set **EP10_OUT** `[21:21]`
    #[inline(always)]
    pub const fn with_ep10_out(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | BuffStatus::EP10_OUT_MASK
            } else {
                self.bits & !BuffStatus::EP10_OUT_MASK
            },
        }
    }

    /// Set **EP10_IN** `[20:20]`
    #[inline(always)]
    pub const fn with_ep10_in(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | BuffStatus::EP10_IN_MASK
            } else {
                self.bits & !BuffStatus::EP10_IN_MASK
            },
        }
    }

    /// Set **EP9_OUT** `[19:19]`
    #[inline(always)]
    pub const fn with_ep9_out(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | BuffStatus::EP9_OUT_MASK
            } else {
                self.bits & !BuffStatus::EP9_OUT_MASK
            },
        }
    }

    /// Set **EP9_IN** `[18:18]`
    #[inline(always)]
    pub const fn with_ep9_in(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | BuffStatus::EP9_IN_MASK
            } else {
                self.bits & !BuffStatus::EP9_IN_MASK
            },
        }
    }

    /// Set **EP8_OUT** `[17:17]`
    #[inline(always)]
    pub const fn with_ep8_out(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | BuffStatus::EP8_OUT_MASK
            } else {
                self.bits & !BuffStatus::EP8_OUT_MASK
            },
        }
    }

    /// Set **EP8_IN** `[16:16]`
    #[inline(always)]
    pub const fn with_ep8_in(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | BuffStatus::EP8_IN_MASK
            } else {
                self.bits & !BuffStatus::EP8_IN_MASK
            },
        }
    }

    /// Set **EP7_OUT** `[15:15]`
    #[inline(always)]
    pub const fn with_ep7_out(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | BuffStatus::EP7_OUT_MASK
            } else {
                self.bits & !BuffStatus::EP7_OUT_MASK
            },
        }
    }

    /// Set **EP7_IN** `[14:14]`
    #[inline(always)]
    pub const fn with_ep7_in(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | BuffStatus::EP7_IN_MASK
            } else {
                self.bits & !BuffStatus::EP7_IN_MASK
            },
        }
    }

    /// Set **EP6_OUT** `[13:13]`
    #[inline(always)]
    pub const fn with_ep6_out(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | BuffStatus::EP6_OUT_MASK
            } else {
                self.bits & !BuffStatus::EP6_OUT_MASK
            },
        }
    }

    /// Set **EP6_IN** `[12:12]`
    #[inline(always)]
    pub const fn with_ep6_in(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | BuffStatus::EP6_IN_MASK
            } else {
                self.bits & !BuffStatus::EP6_IN_MASK
            },
        }
    }

    /// Set **EP5_OUT** `[11:11]`
    #[inline(always)]
    pub const fn with_ep5_out(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | BuffStatus::EP5_OUT_MASK
            } else {
                self.bits & !BuffStatus::EP5_OUT_MASK
            },
        }
    }

    /// Set **EP5_IN** `[10:10]`
    #[inline(always)]
    pub const fn with_ep5_in(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | BuffStatus::EP5_IN_MASK
            } else {
                self.bits & !BuffStatus::EP5_IN_MASK
            },
        }
    }

    /// Set **EP4_OUT** `[9:9]`
    #[inline(always)]
    pub const fn with_ep4_out(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | BuffStatus::EP4_OUT_MASK
            } else {
                self.bits & !BuffStatus::EP4_OUT_MASK
            },
        }
    }

    /// Set **EP4_IN** `[8:8]`
    #[inline(always)]
    pub const fn with_ep4_in(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | BuffStatus::EP4_IN_MASK
            } else {
                self.bits & !BuffStatus::EP4_IN_MASK
            },
        }
    }

    /// Set **EP3_OUT** `[7:7]`
    #[inline(always)]
    pub const fn with_ep3_out(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | BuffStatus::EP3_OUT_MASK
            } else {
                self.bits & !BuffStatus::EP3_OUT_MASK
            },
        }
    }

    /// Set **EP3_IN** `[6:6]`
    #[inline(always)]
    pub const fn with_ep3_in(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | BuffStatus::EP3_IN_MASK
            } else {
                self.bits & !BuffStatus::EP3_IN_MASK
            },
        }
    }

    /// Set **EP2_OUT** `[5:5]`
    #[inline(always)]
    pub const fn with_ep2_out(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | BuffStatus::EP2_OUT_MASK
            } else {
                self.bits & !BuffStatus::EP2_OUT_MASK
            },
        }
    }

    /// Set **EP2_IN** `[4:4]`
    #[inline(always)]
    pub const fn with_ep2_in(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | BuffStatus::EP2_IN_MASK
            } else {
                self.bits & !BuffStatus::EP2_IN_MASK
            },
        }
    }

    /// Set **EP1_OUT** `[3:3]`
    #[inline(always)]
    pub const fn with_ep1_out(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | BuffStatus::EP1_OUT_MASK
            } else {
                self.bits & !BuffStatus::EP1_OUT_MASK
            },
        }
    }

    /// Set **EP1_IN** `[2:2]`
    #[inline(always)]
    pub const fn with_ep1_in(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | BuffStatus::EP1_IN_MASK
            } else {
                self.bits & !BuffStatus::EP1_IN_MASK
            },
        }
    }

    /// Set **EP0_OUT** `[1:1]`
    #[inline(always)]
    pub const fn with_ep0_out(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | BuffStatus::EP0_OUT_MASK
            } else {
                self.bits & !BuffStatus::EP0_OUT_MASK
            },
        }
    }

    /// Set **EP0_IN** `[0:0]`
    #[inline(always)]
    pub const fn with_ep0_in(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | BuffStatus::EP0_IN_MASK
            } else {
                self.bits & !BuffStatus::EP0_IN_MASK
            },
        }
    }
}

impl Default for BuffStatusVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl BuffStatus {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> BuffStatusVal {
        BuffStatusVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: BuffStatusVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(BuffStatusVal) -> BuffStatusVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// BUFF_CPU_SHOULD_HANDLE Register
///
/// Which of the double buffers should be handled. Only valid if using an interrupt per buffer (i.e. not per 2 buffers). Not valid for host interrupt endpoint polling because they are only single buffered.
///
/// Address offset: 0x005c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **EP15_OUT** `[31]` (RO)
///
/// - **EP15_IN** `[30]` (RO)
///
/// - **EP14_OUT** `[29]` (RO)
///
/// - **EP14_IN** `[28]` (RO)
///
/// - **EP13_OUT** `[27]` (RO)
///
/// - **EP13_IN** `[26]` (RO)
///
/// - **EP12_OUT** `[25]` (RO)
///
/// - **EP12_IN** `[24]` (RO)
///
/// - **EP11_OUT** `[23]` (RO)
///
/// - **EP11_IN** `[22]` (RO)
///
/// - **EP10_OUT** `[21]` (RO)
///
/// - **EP10_IN** `[20]` (RO)
///
/// - **EP9_OUT** `[19]` (RO)
///
/// - **EP9_IN** `[18]` (RO)
///
/// - **EP8_OUT** `[17]` (RO)
///
/// - **EP8_IN** `[16]` (RO)
///
/// - **EP7_OUT** `[15]` (RO)
///
/// - **EP7_IN** `[14]` (RO)
///
/// - **EP6_OUT** `[13]` (RO)
///
/// - **EP6_IN** `[12]` (RO)
///
/// - **EP5_OUT** `[11]` (RO)
///
/// - **EP5_IN** `[10]` (RO)
///
/// - **EP4_OUT** `[9]` (RO)
///
/// - **EP4_IN** `[8]` (RO)
///
/// - **EP3_OUT** `[7]` (RO)
///
/// - **EP3_IN** `[6]` (RO)
///
/// - **EP2_OUT** `[5]` (RO)
///
/// - **EP2_IN** `[4]` (RO)
///
/// - **EP1_OUT** `[3]` (RO)
///
/// - **EP1_IN** `[2]` (RO)
///
/// - **EP0_OUT** `[1]` (RO)
///
/// - **EP0_IN** `[0]` (RO)
#[repr(transparent)]
pub struct BuffCpuShouldHandle {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for BuffCpuShouldHandle {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl BuffCpuShouldHandle {
    // ======== Field Constants ========

    /// **EP15_OUT**
    /// Bits `[31:31]`, mask `0x80000000`
    pub const EP15_OUT_OFFSET: u32 = 31;
    pub const EP15_OUT_MASK: u32 = 0x80000000;
    pub const EP15_OUT: u32 = 1 << 31;

    /// **EP15_IN**
    /// Bits `[30:30]`, mask `0x40000000`
    pub const EP15_IN_OFFSET: u32 = 30;
    pub const EP15_IN_MASK: u32 = 0x40000000;
    pub const EP15_IN: u32 = 1 << 30;

    /// **EP14_OUT**
    /// Bits `[29:29]`, mask `0x20000000`
    pub const EP14_OUT_OFFSET: u32 = 29;
    pub const EP14_OUT_MASK: u32 = 0x20000000;
    pub const EP14_OUT: u32 = 1 << 29;

    /// **EP14_IN**
    /// Bits `[28:28]`, mask `0x10000000`
    pub const EP14_IN_OFFSET: u32 = 28;
    pub const EP14_IN_MASK: u32 = 0x10000000;
    pub const EP14_IN: u32 = 1 << 28;

    /// **EP13_OUT**
    /// Bits `[27:27]`, mask `0x08000000`
    pub const EP13_OUT_OFFSET: u32 = 27;
    pub const EP13_OUT_MASK: u32 = 0x08000000;
    pub const EP13_OUT: u32 = 1 << 27;

    /// **EP13_IN**
    /// Bits `[26:26]`, mask `0x04000000`
    pub const EP13_IN_OFFSET: u32 = 26;
    pub const EP13_IN_MASK: u32 = 0x04000000;
    pub const EP13_IN: u32 = 1 << 26;

    /// **EP12_OUT**
    /// Bits `[25:25]`, mask `0x02000000`
    pub const EP12_OUT_OFFSET: u32 = 25;
    pub const EP12_OUT_MASK: u32 = 0x02000000;
    pub const EP12_OUT: u32 = 1 << 25;

    /// **EP12_IN**
    /// Bits `[24:24]`, mask `0x01000000`
    pub const EP12_IN_OFFSET: u32 = 24;
    pub const EP12_IN_MASK: u32 = 0x01000000;
    pub const EP12_IN: u32 = 1 << 24;

    /// **EP11_OUT**
    /// Bits `[23:23]`, mask `0x00800000`
    pub const EP11_OUT_OFFSET: u32 = 23;
    pub const EP11_OUT_MASK: u32 = 0x00800000;
    pub const EP11_OUT: u32 = 1 << 23;

    /// **EP11_IN**
    /// Bits `[22:22]`, mask `0x00400000`
    pub const EP11_IN_OFFSET: u32 = 22;
    pub const EP11_IN_MASK: u32 = 0x00400000;
    pub const EP11_IN: u32 = 1 << 22;

    /// **EP10_OUT**
    /// Bits `[21:21]`, mask `0x00200000`
    pub const EP10_OUT_OFFSET: u32 = 21;
    pub const EP10_OUT_MASK: u32 = 0x00200000;
    pub const EP10_OUT: u32 = 1 << 21;

    /// **EP10_IN**
    /// Bits `[20:20]`, mask `0x00100000`
    pub const EP10_IN_OFFSET: u32 = 20;
    pub const EP10_IN_MASK: u32 = 0x00100000;
    pub const EP10_IN: u32 = 1 << 20;

    /// **EP9_OUT**
    /// Bits `[19:19]`, mask `0x00080000`
    pub const EP9_OUT_OFFSET: u32 = 19;
    pub const EP9_OUT_MASK: u32 = 0x00080000;
    pub const EP9_OUT: u32 = 1 << 19;

    /// **EP9_IN**
    /// Bits `[18:18]`, mask `0x00040000`
    pub const EP9_IN_OFFSET: u32 = 18;
    pub const EP9_IN_MASK: u32 = 0x00040000;
    pub const EP9_IN: u32 = 1 << 18;

    /// **EP8_OUT**
    /// Bits `[17:17]`, mask `0x00020000`
    pub const EP8_OUT_OFFSET: u32 = 17;
    pub const EP8_OUT_MASK: u32 = 0x00020000;
    pub const EP8_OUT: u32 = 1 << 17;

    /// **EP8_IN**
    /// Bits `[16:16]`, mask `0x00010000`
    pub const EP8_IN_OFFSET: u32 = 16;
    pub const EP8_IN_MASK: u32 = 0x00010000;
    pub const EP8_IN: u32 = 1 << 16;

    /// **EP7_OUT**
    /// Bits `[15:15]`, mask `0x00008000`
    pub const EP7_OUT_OFFSET: u32 = 15;
    pub const EP7_OUT_MASK: u32 = 0x00008000;
    pub const EP7_OUT: u32 = 1 << 15;

    /// **EP7_IN**
    /// Bits `[14:14]`, mask `0x00004000`
    pub const EP7_IN_OFFSET: u32 = 14;
    pub const EP7_IN_MASK: u32 = 0x00004000;
    pub const EP7_IN: u32 = 1 << 14;

    /// **EP6_OUT**
    /// Bits `[13:13]`, mask `0x00002000`
    pub const EP6_OUT_OFFSET: u32 = 13;
    pub const EP6_OUT_MASK: u32 = 0x00002000;
    pub const EP6_OUT: u32 = 1 << 13;

    /// **EP6_IN**
    /// Bits `[12:12]`, mask `0x00001000`
    pub const EP6_IN_OFFSET: u32 = 12;
    pub const EP6_IN_MASK: u32 = 0x00001000;
    pub const EP6_IN: u32 = 1 << 12;

    /// **EP5_OUT**
    /// Bits `[11:11]`, mask `0x00000800`
    pub const EP5_OUT_OFFSET: u32 = 11;
    pub const EP5_OUT_MASK: u32 = 0x00000800;
    pub const EP5_OUT: u32 = 1 << 11;

    /// **EP5_IN**
    /// Bits `[10:10]`, mask `0x00000400`
    pub const EP5_IN_OFFSET: u32 = 10;
    pub const EP5_IN_MASK: u32 = 0x00000400;
    pub const EP5_IN: u32 = 1 << 10;

    /// **EP4_OUT**
    /// Bits `[9:9]`, mask `0x00000200`
    pub const EP4_OUT_OFFSET: u32 = 9;
    pub const EP4_OUT_MASK: u32 = 0x00000200;
    pub const EP4_OUT: u32 = 1 << 9;

    /// **EP4_IN**
    /// Bits `[8:8]`, mask `0x00000100`
    pub const EP4_IN_OFFSET: u32 = 8;
    pub const EP4_IN_MASK: u32 = 0x00000100;
    pub const EP4_IN: u32 = 1 << 8;

    /// **EP3_OUT**
    /// Bits `[7:7]`, mask `0x00000080`
    pub const EP3_OUT_OFFSET: u32 = 7;
    pub const EP3_OUT_MASK: u32 = 0x00000080;
    pub const EP3_OUT: u32 = 1 << 7;

    /// **EP3_IN**
    /// Bits `[6:6]`, mask `0x00000040`
    pub const EP3_IN_OFFSET: u32 = 6;
    pub const EP3_IN_MASK: u32 = 0x00000040;
    pub const EP3_IN: u32 = 1 << 6;

    /// **EP2_OUT**
    /// Bits `[5:5]`, mask `0x00000020`
    pub const EP2_OUT_OFFSET: u32 = 5;
    pub const EP2_OUT_MASK: u32 = 0x00000020;
    pub const EP2_OUT: u32 = 1 << 5;

    /// **EP2_IN**
    /// Bits `[4:4]`, mask `0x00000010`
    pub const EP2_IN_OFFSET: u32 = 4;
    pub const EP2_IN_MASK: u32 = 0x00000010;
    pub const EP2_IN: u32 = 1 << 4;

    /// **EP1_OUT**
    /// Bits `[3:3]`, mask `0x00000008`
    pub const EP1_OUT_OFFSET: u32 = 3;
    pub const EP1_OUT_MASK: u32 = 0x00000008;
    pub const EP1_OUT: u32 = 1 << 3;

    /// **EP1_IN**
    /// Bits `[2:2]`, mask `0x00000004`
    pub const EP1_IN_OFFSET: u32 = 2;
    pub const EP1_IN_MASK: u32 = 0x00000004;
    pub const EP1_IN: u32 = 1 << 2;

    /// **EP0_OUT**
    /// Bits `[1:1]`, mask `0x00000002`
    pub const EP0_OUT_OFFSET: u32 = 1;
    pub const EP0_OUT_MASK: u32 = 0x00000002;
    pub const EP0_OUT: u32 = 1 << 1;

    /// **EP0_IN**
    /// Bits `[0:0]`, mask `0x00000001`
    pub const EP0_IN_OFFSET: u32 = 0;
    pub const EP0_IN_MASK: u32 = 0x00000001;
    pub const EP0_IN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **EP15_OUT** `[31]` (RO)
    #[inline(always)]
    pub fn ep15_out(&self) -> bool {
        (self.value.read_bits() & Self::EP15_OUT_MASK) != 0
    }

    /// **EP15_IN** `[30]` (RO)
    #[inline(always)]
    pub fn ep15_in(&self) -> bool {
        (self.value.read_bits() & Self::EP15_IN_MASK) != 0
    }

    /// **EP14_OUT** `[29]` (RO)
    #[inline(always)]
    pub fn ep14_out(&self) -> bool {
        (self.value.read_bits() & Self::EP14_OUT_MASK) != 0
    }

    /// **EP14_IN** `[28]` (RO)
    #[inline(always)]
    pub fn ep14_in(&self) -> bool {
        (self.value.read_bits() & Self::EP14_IN_MASK) != 0
    }

    /// **EP13_OUT** `[27]` (RO)
    #[inline(always)]
    pub fn ep13_out(&self) -> bool {
        (self.value.read_bits() & Self::EP13_OUT_MASK) != 0
    }

    /// **EP13_IN** `[26]` (RO)
    #[inline(always)]
    pub fn ep13_in(&self) -> bool {
        (self.value.read_bits() & Self::EP13_IN_MASK) != 0
    }

    /// **EP12_OUT** `[25]` (RO)
    #[inline(always)]
    pub fn ep12_out(&self) -> bool {
        (self.value.read_bits() & Self::EP12_OUT_MASK) != 0
    }

    /// **EP12_IN** `[24]` (RO)
    #[inline(always)]
    pub fn ep12_in(&self) -> bool {
        (self.value.read_bits() & Self::EP12_IN_MASK) != 0
    }

    /// **EP11_OUT** `[23]` (RO)
    #[inline(always)]
    pub fn ep11_out(&self) -> bool {
        (self.value.read_bits() & Self::EP11_OUT_MASK) != 0
    }

    /// **EP11_IN** `[22]` (RO)
    #[inline(always)]
    pub fn ep11_in(&self) -> bool {
        (self.value.read_bits() & Self::EP11_IN_MASK) != 0
    }

    /// **EP10_OUT** `[21]` (RO)
    #[inline(always)]
    pub fn ep10_out(&self) -> bool {
        (self.value.read_bits() & Self::EP10_OUT_MASK) != 0
    }

    /// **EP10_IN** `[20]` (RO)
    #[inline(always)]
    pub fn ep10_in(&self) -> bool {
        (self.value.read_bits() & Self::EP10_IN_MASK) != 0
    }

    /// **EP9_OUT** `[19]` (RO)
    #[inline(always)]
    pub fn ep9_out(&self) -> bool {
        (self.value.read_bits() & Self::EP9_OUT_MASK) != 0
    }

    /// **EP9_IN** `[18]` (RO)
    #[inline(always)]
    pub fn ep9_in(&self) -> bool {
        (self.value.read_bits() & Self::EP9_IN_MASK) != 0
    }

    /// **EP8_OUT** `[17]` (RO)
    #[inline(always)]
    pub fn ep8_out(&self) -> bool {
        (self.value.read_bits() & Self::EP8_OUT_MASK) != 0
    }

    /// **EP8_IN** `[16]` (RO)
    #[inline(always)]
    pub fn ep8_in(&self) -> bool {
        (self.value.read_bits() & Self::EP8_IN_MASK) != 0
    }

    /// **EP7_OUT** `[15]` (RO)
    #[inline(always)]
    pub fn ep7_out(&self) -> bool {
        (self.value.read_bits() & Self::EP7_OUT_MASK) != 0
    }

    /// **EP7_IN** `[14]` (RO)
    #[inline(always)]
    pub fn ep7_in(&self) -> bool {
        (self.value.read_bits() & Self::EP7_IN_MASK) != 0
    }

    /// **EP6_OUT** `[13]` (RO)
    #[inline(always)]
    pub fn ep6_out(&self) -> bool {
        (self.value.read_bits() & Self::EP6_OUT_MASK) != 0
    }

    /// **EP6_IN** `[12]` (RO)
    #[inline(always)]
    pub fn ep6_in(&self) -> bool {
        (self.value.read_bits() & Self::EP6_IN_MASK) != 0
    }

    /// **EP5_OUT** `[11]` (RO)
    #[inline(always)]
    pub fn ep5_out(&self) -> bool {
        (self.value.read_bits() & Self::EP5_OUT_MASK) != 0
    }

    /// **EP5_IN** `[10]` (RO)
    #[inline(always)]
    pub fn ep5_in(&self) -> bool {
        (self.value.read_bits() & Self::EP5_IN_MASK) != 0
    }

    /// **EP4_OUT** `[9]` (RO)
    #[inline(always)]
    pub fn ep4_out(&self) -> bool {
        (self.value.read_bits() & Self::EP4_OUT_MASK) != 0
    }

    /// **EP4_IN** `[8]` (RO)
    #[inline(always)]
    pub fn ep4_in(&self) -> bool {
        (self.value.read_bits() & Self::EP4_IN_MASK) != 0
    }

    /// **EP3_OUT** `[7]` (RO)
    #[inline(always)]
    pub fn ep3_out(&self) -> bool {
        (self.value.read_bits() & Self::EP3_OUT_MASK) != 0
    }

    /// **EP3_IN** `[6]` (RO)
    #[inline(always)]
    pub fn ep3_in(&self) -> bool {
        (self.value.read_bits() & Self::EP3_IN_MASK) != 0
    }

    /// **EP2_OUT** `[5]` (RO)
    #[inline(always)]
    pub fn ep2_out(&self) -> bool {
        (self.value.read_bits() & Self::EP2_OUT_MASK) != 0
    }

    /// **EP2_IN** `[4]` (RO)
    #[inline(always)]
    pub fn ep2_in(&self) -> bool {
        (self.value.read_bits() & Self::EP2_IN_MASK) != 0
    }

    /// **EP1_OUT** `[3]` (RO)
    #[inline(always)]
    pub fn ep1_out(&self) -> bool {
        (self.value.read_bits() & Self::EP1_OUT_MASK) != 0
    }

    /// **EP1_IN** `[2]` (RO)
    #[inline(always)]
    pub fn ep1_in(&self) -> bool {
        (self.value.read_bits() & Self::EP1_IN_MASK) != 0
    }

    /// **EP0_OUT** `[1]` (RO)
    #[inline(always)]
    pub fn ep0_out(&self) -> bool {
        (self.value.read_bits() & Self::EP0_OUT_MASK) != 0
    }

    /// **EP0_IN** `[0]` (RO)
    #[inline(always)]
    pub fn ep0_in(&self) -> bool {
        (self.value.read_bits() & Self::EP0_IN_MASK) != 0
    }
}

/// BUFF_CPU_SHOULD_HANDLE value type for batch read/modify/write operations
///
/// Use `buffcpushouldhandle.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `buffcpushouldhandle.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BuffCpuShouldHandleVal {
    bits: u32,
}

impl BuffCpuShouldHandleVal {
    /// Create from raw bits
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

    /// Read **EP15_OUT** `[31:31]`
    #[inline(always)]
    pub const fn ep15_out(&self) -> bool {
        (self.bits & BuffCpuShouldHandle::EP15_OUT_MASK) != 0
    }

    /// Read **EP15_IN** `[30:30]`
    #[inline(always)]
    pub const fn ep15_in(&self) -> bool {
        (self.bits & BuffCpuShouldHandle::EP15_IN_MASK) != 0
    }

    /// Read **EP14_OUT** `[29:29]`
    #[inline(always)]
    pub const fn ep14_out(&self) -> bool {
        (self.bits & BuffCpuShouldHandle::EP14_OUT_MASK) != 0
    }

    /// Read **EP14_IN** `[28:28]`
    #[inline(always)]
    pub const fn ep14_in(&self) -> bool {
        (self.bits & BuffCpuShouldHandle::EP14_IN_MASK) != 0
    }

    /// Read **EP13_OUT** `[27:27]`
    #[inline(always)]
    pub const fn ep13_out(&self) -> bool {
        (self.bits & BuffCpuShouldHandle::EP13_OUT_MASK) != 0
    }

    /// Read **EP13_IN** `[26:26]`
    #[inline(always)]
    pub const fn ep13_in(&self) -> bool {
        (self.bits & BuffCpuShouldHandle::EP13_IN_MASK) != 0
    }

    /// Read **EP12_OUT** `[25:25]`
    #[inline(always)]
    pub const fn ep12_out(&self) -> bool {
        (self.bits & BuffCpuShouldHandle::EP12_OUT_MASK) != 0
    }

    /// Read **EP12_IN** `[24:24]`
    #[inline(always)]
    pub const fn ep12_in(&self) -> bool {
        (self.bits & BuffCpuShouldHandle::EP12_IN_MASK) != 0
    }

    /// Read **EP11_OUT** `[23:23]`
    #[inline(always)]
    pub const fn ep11_out(&self) -> bool {
        (self.bits & BuffCpuShouldHandle::EP11_OUT_MASK) != 0
    }

    /// Read **EP11_IN** `[22:22]`
    #[inline(always)]
    pub const fn ep11_in(&self) -> bool {
        (self.bits & BuffCpuShouldHandle::EP11_IN_MASK) != 0
    }

    /// Read **EP10_OUT** `[21:21]`
    #[inline(always)]
    pub const fn ep10_out(&self) -> bool {
        (self.bits & BuffCpuShouldHandle::EP10_OUT_MASK) != 0
    }

    /// Read **EP10_IN** `[20:20]`
    #[inline(always)]
    pub const fn ep10_in(&self) -> bool {
        (self.bits & BuffCpuShouldHandle::EP10_IN_MASK) != 0
    }

    /// Read **EP9_OUT** `[19:19]`
    #[inline(always)]
    pub const fn ep9_out(&self) -> bool {
        (self.bits & BuffCpuShouldHandle::EP9_OUT_MASK) != 0
    }

    /// Read **EP9_IN** `[18:18]`
    #[inline(always)]
    pub const fn ep9_in(&self) -> bool {
        (self.bits & BuffCpuShouldHandle::EP9_IN_MASK) != 0
    }

    /// Read **EP8_OUT** `[17:17]`
    #[inline(always)]
    pub const fn ep8_out(&self) -> bool {
        (self.bits & BuffCpuShouldHandle::EP8_OUT_MASK) != 0
    }

    /// Read **EP8_IN** `[16:16]`
    #[inline(always)]
    pub const fn ep8_in(&self) -> bool {
        (self.bits & BuffCpuShouldHandle::EP8_IN_MASK) != 0
    }

    /// Read **EP7_OUT** `[15:15]`
    #[inline(always)]
    pub const fn ep7_out(&self) -> bool {
        (self.bits & BuffCpuShouldHandle::EP7_OUT_MASK) != 0
    }

    /// Read **EP7_IN** `[14:14]`
    #[inline(always)]
    pub const fn ep7_in(&self) -> bool {
        (self.bits & BuffCpuShouldHandle::EP7_IN_MASK) != 0
    }

    /// Read **EP6_OUT** `[13:13]`
    #[inline(always)]
    pub const fn ep6_out(&self) -> bool {
        (self.bits & BuffCpuShouldHandle::EP6_OUT_MASK) != 0
    }

    /// Read **EP6_IN** `[12:12]`
    #[inline(always)]
    pub const fn ep6_in(&self) -> bool {
        (self.bits & BuffCpuShouldHandle::EP6_IN_MASK) != 0
    }

    /// Read **EP5_OUT** `[11:11]`
    #[inline(always)]
    pub const fn ep5_out(&self) -> bool {
        (self.bits & BuffCpuShouldHandle::EP5_OUT_MASK) != 0
    }

    /// Read **EP5_IN** `[10:10]`
    #[inline(always)]
    pub const fn ep5_in(&self) -> bool {
        (self.bits & BuffCpuShouldHandle::EP5_IN_MASK) != 0
    }

    /// Read **EP4_OUT** `[9:9]`
    #[inline(always)]
    pub const fn ep4_out(&self) -> bool {
        (self.bits & BuffCpuShouldHandle::EP4_OUT_MASK) != 0
    }

    /// Read **EP4_IN** `[8:8]`
    #[inline(always)]
    pub const fn ep4_in(&self) -> bool {
        (self.bits & BuffCpuShouldHandle::EP4_IN_MASK) != 0
    }

    /// Read **EP3_OUT** `[7:7]`
    #[inline(always)]
    pub const fn ep3_out(&self) -> bool {
        (self.bits & BuffCpuShouldHandle::EP3_OUT_MASK) != 0
    }

    /// Read **EP3_IN** `[6:6]`
    #[inline(always)]
    pub const fn ep3_in(&self) -> bool {
        (self.bits & BuffCpuShouldHandle::EP3_IN_MASK) != 0
    }

    /// Read **EP2_OUT** `[5:5]`
    #[inline(always)]
    pub const fn ep2_out(&self) -> bool {
        (self.bits & BuffCpuShouldHandle::EP2_OUT_MASK) != 0
    }

    /// Read **EP2_IN** `[4:4]`
    #[inline(always)]
    pub const fn ep2_in(&self) -> bool {
        (self.bits & BuffCpuShouldHandle::EP2_IN_MASK) != 0
    }

    /// Read **EP1_OUT** `[3:3]`
    #[inline(always)]
    pub const fn ep1_out(&self) -> bool {
        (self.bits & BuffCpuShouldHandle::EP1_OUT_MASK) != 0
    }

    /// Read **EP1_IN** `[2:2]`
    #[inline(always)]
    pub const fn ep1_in(&self) -> bool {
        (self.bits & BuffCpuShouldHandle::EP1_IN_MASK) != 0
    }

    /// Read **EP0_OUT** `[1:1]`
    #[inline(always)]
    pub const fn ep0_out(&self) -> bool {
        (self.bits & BuffCpuShouldHandle::EP0_OUT_MASK) != 0
    }

    /// Read **EP0_IN** `[0:0]`
    #[inline(always)]
    pub const fn ep0_in(&self) -> bool {
        (self.bits & BuffCpuShouldHandle::EP0_IN_MASK) != 0
    }
}

impl Default for BuffCpuShouldHandleVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl BuffCpuShouldHandle {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> BuffCpuShouldHandleVal {
        BuffCpuShouldHandleVal::from_bits(self.value.read_bits())
    }
}

/// EP_ABORT Register
///
/// Device only: Can be set to ignore the buffer control register for this endpoint in case you would like to revoke a buffer. A NAK will be sent for every access to the endpoint until this bit is cleared. A corresponding bit in `EP_ABORT_DONE` is set when it is safe to modify the buffer control register.
///
/// Address offset: 0x0060
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **EP15_OUT** `[31]` (RW)
///
/// - **EP15_IN** `[30]` (RW)
///
/// - **EP14_OUT** `[29]` (RW)
///
/// - **EP14_IN** `[28]` (RW)
///
/// - **EP13_OUT** `[27]` (RW)
///
/// - **EP13_IN** `[26]` (RW)
///
/// - **EP12_OUT** `[25]` (RW)
///
/// - **EP12_IN** `[24]` (RW)
///
/// - **EP11_OUT** `[23]` (RW)
///
/// - **EP11_IN** `[22]` (RW)
///
/// - **EP10_OUT** `[21]` (RW)
///
/// - **EP10_IN** `[20]` (RW)
///
/// - **EP9_OUT** `[19]` (RW)
///
/// - **EP9_IN** `[18]` (RW)
///
/// - **EP8_OUT** `[17]` (RW)
///
/// - **EP8_IN** `[16]` (RW)
///
/// - **EP7_OUT** `[15]` (RW)
///
/// - **EP7_IN** `[14]` (RW)
///
/// - **EP6_OUT** `[13]` (RW)
///
/// - **EP6_IN** `[12]` (RW)
///
/// - **EP5_OUT** `[11]` (RW)
///
/// - **EP5_IN** `[10]` (RW)
///
/// - **EP4_OUT** `[9]` (RW)
///
/// - **EP4_IN** `[8]` (RW)
///
/// - **EP3_OUT** `[7]` (RW)
///
/// - **EP3_IN** `[6]` (RW)
///
/// - **EP2_OUT** `[5]` (RW)
///
/// - **EP2_IN** `[4]` (RW)
///
/// - **EP1_OUT** `[3]` (RW)
///
/// - **EP1_IN** `[2]` (RW)
///
/// - **EP0_OUT** `[1]` (RW)
///
/// - **EP0_IN** `[0]` (RW)
#[repr(transparent)]
pub struct EpAbort {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for EpAbort {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl EpAbort {
    // ======== Field Constants ========

    /// **EP15_OUT**
    /// Bits `[31:31]`, mask `0x80000000`
    pub const EP15_OUT_OFFSET: u32 = 31;
    pub const EP15_OUT_MASK: u32 = 0x80000000;
    pub const EP15_OUT: u32 = 1 << 31;

    /// **EP15_IN**
    /// Bits `[30:30]`, mask `0x40000000`
    pub const EP15_IN_OFFSET: u32 = 30;
    pub const EP15_IN_MASK: u32 = 0x40000000;
    pub const EP15_IN: u32 = 1 << 30;

    /// **EP14_OUT**
    /// Bits `[29:29]`, mask `0x20000000`
    pub const EP14_OUT_OFFSET: u32 = 29;
    pub const EP14_OUT_MASK: u32 = 0x20000000;
    pub const EP14_OUT: u32 = 1 << 29;

    /// **EP14_IN**
    /// Bits `[28:28]`, mask `0x10000000`
    pub const EP14_IN_OFFSET: u32 = 28;
    pub const EP14_IN_MASK: u32 = 0x10000000;
    pub const EP14_IN: u32 = 1 << 28;

    /// **EP13_OUT**
    /// Bits `[27:27]`, mask `0x08000000`
    pub const EP13_OUT_OFFSET: u32 = 27;
    pub const EP13_OUT_MASK: u32 = 0x08000000;
    pub const EP13_OUT: u32 = 1 << 27;

    /// **EP13_IN**
    /// Bits `[26:26]`, mask `0x04000000`
    pub const EP13_IN_OFFSET: u32 = 26;
    pub const EP13_IN_MASK: u32 = 0x04000000;
    pub const EP13_IN: u32 = 1 << 26;

    /// **EP12_OUT**
    /// Bits `[25:25]`, mask `0x02000000`
    pub const EP12_OUT_OFFSET: u32 = 25;
    pub const EP12_OUT_MASK: u32 = 0x02000000;
    pub const EP12_OUT: u32 = 1 << 25;

    /// **EP12_IN**
    /// Bits `[24:24]`, mask `0x01000000`
    pub const EP12_IN_OFFSET: u32 = 24;
    pub const EP12_IN_MASK: u32 = 0x01000000;
    pub const EP12_IN: u32 = 1 << 24;

    /// **EP11_OUT**
    /// Bits `[23:23]`, mask `0x00800000`
    pub const EP11_OUT_OFFSET: u32 = 23;
    pub const EP11_OUT_MASK: u32 = 0x00800000;
    pub const EP11_OUT: u32 = 1 << 23;

    /// **EP11_IN**
    /// Bits `[22:22]`, mask `0x00400000`
    pub const EP11_IN_OFFSET: u32 = 22;
    pub const EP11_IN_MASK: u32 = 0x00400000;
    pub const EP11_IN: u32 = 1 << 22;

    /// **EP10_OUT**
    /// Bits `[21:21]`, mask `0x00200000`
    pub const EP10_OUT_OFFSET: u32 = 21;
    pub const EP10_OUT_MASK: u32 = 0x00200000;
    pub const EP10_OUT: u32 = 1 << 21;

    /// **EP10_IN**
    /// Bits `[20:20]`, mask `0x00100000`
    pub const EP10_IN_OFFSET: u32 = 20;
    pub const EP10_IN_MASK: u32 = 0x00100000;
    pub const EP10_IN: u32 = 1 << 20;

    /// **EP9_OUT**
    /// Bits `[19:19]`, mask `0x00080000`
    pub const EP9_OUT_OFFSET: u32 = 19;
    pub const EP9_OUT_MASK: u32 = 0x00080000;
    pub const EP9_OUT: u32 = 1 << 19;

    /// **EP9_IN**
    /// Bits `[18:18]`, mask `0x00040000`
    pub const EP9_IN_OFFSET: u32 = 18;
    pub const EP9_IN_MASK: u32 = 0x00040000;
    pub const EP9_IN: u32 = 1 << 18;

    /// **EP8_OUT**
    /// Bits `[17:17]`, mask `0x00020000`
    pub const EP8_OUT_OFFSET: u32 = 17;
    pub const EP8_OUT_MASK: u32 = 0x00020000;
    pub const EP8_OUT: u32 = 1 << 17;

    /// **EP8_IN**
    /// Bits `[16:16]`, mask `0x00010000`
    pub const EP8_IN_OFFSET: u32 = 16;
    pub const EP8_IN_MASK: u32 = 0x00010000;
    pub const EP8_IN: u32 = 1 << 16;

    /// **EP7_OUT**
    /// Bits `[15:15]`, mask `0x00008000`
    pub const EP7_OUT_OFFSET: u32 = 15;
    pub const EP7_OUT_MASK: u32 = 0x00008000;
    pub const EP7_OUT: u32 = 1 << 15;

    /// **EP7_IN**
    /// Bits `[14:14]`, mask `0x00004000`
    pub const EP7_IN_OFFSET: u32 = 14;
    pub const EP7_IN_MASK: u32 = 0x00004000;
    pub const EP7_IN: u32 = 1 << 14;

    /// **EP6_OUT**
    /// Bits `[13:13]`, mask `0x00002000`
    pub const EP6_OUT_OFFSET: u32 = 13;
    pub const EP6_OUT_MASK: u32 = 0x00002000;
    pub const EP6_OUT: u32 = 1 << 13;

    /// **EP6_IN**
    /// Bits `[12:12]`, mask `0x00001000`
    pub const EP6_IN_OFFSET: u32 = 12;
    pub const EP6_IN_MASK: u32 = 0x00001000;
    pub const EP6_IN: u32 = 1 << 12;

    /// **EP5_OUT**
    /// Bits `[11:11]`, mask `0x00000800`
    pub const EP5_OUT_OFFSET: u32 = 11;
    pub const EP5_OUT_MASK: u32 = 0x00000800;
    pub const EP5_OUT: u32 = 1 << 11;

    /// **EP5_IN**
    /// Bits `[10:10]`, mask `0x00000400`
    pub const EP5_IN_OFFSET: u32 = 10;
    pub const EP5_IN_MASK: u32 = 0x00000400;
    pub const EP5_IN: u32 = 1 << 10;

    /// **EP4_OUT**
    /// Bits `[9:9]`, mask `0x00000200`
    pub const EP4_OUT_OFFSET: u32 = 9;
    pub const EP4_OUT_MASK: u32 = 0x00000200;
    pub const EP4_OUT: u32 = 1 << 9;

    /// **EP4_IN**
    /// Bits `[8:8]`, mask `0x00000100`
    pub const EP4_IN_OFFSET: u32 = 8;
    pub const EP4_IN_MASK: u32 = 0x00000100;
    pub const EP4_IN: u32 = 1 << 8;

    /// **EP3_OUT**
    /// Bits `[7:7]`, mask `0x00000080`
    pub const EP3_OUT_OFFSET: u32 = 7;
    pub const EP3_OUT_MASK: u32 = 0x00000080;
    pub const EP3_OUT: u32 = 1 << 7;

    /// **EP3_IN**
    /// Bits `[6:6]`, mask `0x00000040`
    pub const EP3_IN_OFFSET: u32 = 6;
    pub const EP3_IN_MASK: u32 = 0x00000040;
    pub const EP3_IN: u32 = 1 << 6;

    /// **EP2_OUT**
    /// Bits `[5:5]`, mask `0x00000020`
    pub const EP2_OUT_OFFSET: u32 = 5;
    pub const EP2_OUT_MASK: u32 = 0x00000020;
    pub const EP2_OUT: u32 = 1 << 5;

    /// **EP2_IN**
    /// Bits `[4:4]`, mask `0x00000010`
    pub const EP2_IN_OFFSET: u32 = 4;
    pub const EP2_IN_MASK: u32 = 0x00000010;
    pub const EP2_IN: u32 = 1 << 4;

    /// **EP1_OUT**
    /// Bits `[3:3]`, mask `0x00000008`
    pub const EP1_OUT_OFFSET: u32 = 3;
    pub const EP1_OUT_MASK: u32 = 0x00000008;
    pub const EP1_OUT: u32 = 1 << 3;

    /// **EP1_IN**
    /// Bits `[2:2]`, mask `0x00000004`
    pub const EP1_IN_OFFSET: u32 = 2;
    pub const EP1_IN_MASK: u32 = 0x00000004;
    pub const EP1_IN: u32 = 1 << 2;

    /// **EP0_OUT**
    /// Bits `[1:1]`, mask `0x00000002`
    pub const EP0_OUT_OFFSET: u32 = 1;
    pub const EP0_OUT_MASK: u32 = 0x00000002;
    pub const EP0_OUT: u32 = 1 << 1;

    /// **EP0_IN**
    /// Bits `[0:0]`, mask `0x00000001`
    pub const EP0_IN_OFFSET: u32 = 0;
    pub const EP0_IN_MASK: u32 = 0x00000001;
    pub const EP0_IN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **EP15_OUT** `[31]` (RW)
    #[inline(always)]
    pub fn ep15_out(&self) -> bool {
        (self.value.read_bits() & Self::EP15_OUT_MASK) != 0
    }

    /// Set EP15_OUT bit atomically (RW)
    #[inline(always)]
    pub fn set_ep15_out(&self) {
        self.set_bits(Self::EP15_OUT_MASK);
    }

    /// Clear EP15_OUT bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep15_out(&self) {
        self.clr_bits(Self::EP15_OUT_MASK);
    }

    /// **EP15_IN** `[30]` (RW)
    #[inline(always)]
    pub fn ep15_in(&self) -> bool {
        (self.value.read_bits() & Self::EP15_IN_MASK) != 0
    }

    /// Set EP15_IN bit atomically (RW)
    #[inline(always)]
    pub fn set_ep15_in(&self) {
        self.set_bits(Self::EP15_IN_MASK);
    }

    /// Clear EP15_IN bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep15_in(&self) {
        self.clr_bits(Self::EP15_IN_MASK);
    }

    /// **EP14_OUT** `[29]` (RW)
    #[inline(always)]
    pub fn ep14_out(&self) -> bool {
        (self.value.read_bits() & Self::EP14_OUT_MASK) != 0
    }

    /// Set EP14_OUT bit atomically (RW)
    #[inline(always)]
    pub fn set_ep14_out(&self) {
        self.set_bits(Self::EP14_OUT_MASK);
    }

    /// Clear EP14_OUT bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep14_out(&self) {
        self.clr_bits(Self::EP14_OUT_MASK);
    }

    /// **EP14_IN** `[28]` (RW)
    #[inline(always)]
    pub fn ep14_in(&self) -> bool {
        (self.value.read_bits() & Self::EP14_IN_MASK) != 0
    }

    /// Set EP14_IN bit atomically (RW)
    #[inline(always)]
    pub fn set_ep14_in(&self) {
        self.set_bits(Self::EP14_IN_MASK);
    }

    /// Clear EP14_IN bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep14_in(&self) {
        self.clr_bits(Self::EP14_IN_MASK);
    }

    /// **EP13_OUT** `[27]` (RW)
    #[inline(always)]
    pub fn ep13_out(&self) -> bool {
        (self.value.read_bits() & Self::EP13_OUT_MASK) != 0
    }

    /// Set EP13_OUT bit atomically (RW)
    #[inline(always)]
    pub fn set_ep13_out(&self) {
        self.set_bits(Self::EP13_OUT_MASK);
    }

    /// Clear EP13_OUT bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep13_out(&self) {
        self.clr_bits(Self::EP13_OUT_MASK);
    }

    /// **EP13_IN** `[26]` (RW)
    #[inline(always)]
    pub fn ep13_in(&self) -> bool {
        (self.value.read_bits() & Self::EP13_IN_MASK) != 0
    }

    /// Set EP13_IN bit atomically (RW)
    #[inline(always)]
    pub fn set_ep13_in(&self) {
        self.set_bits(Self::EP13_IN_MASK);
    }

    /// Clear EP13_IN bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep13_in(&self) {
        self.clr_bits(Self::EP13_IN_MASK);
    }

    /// **EP12_OUT** `[25]` (RW)
    #[inline(always)]
    pub fn ep12_out(&self) -> bool {
        (self.value.read_bits() & Self::EP12_OUT_MASK) != 0
    }

    /// Set EP12_OUT bit atomically (RW)
    #[inline(always)]
    pub fn set_ep12_out(&self) {
        self.set_bits(Self::EP12_OUT_MASK);
    }

    /// Clear EP12_OUT bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep12_out(&self) {
        self.clr_bits(Self::EP12_OUT_MASK);
    }

    /// **EP12_IN** `[24]` (RW)
    #[inline(always)]
    pub fn ep12_in(&self) -> bool {
        (self.value.read_bits() & Self::EP12_IN_MASK) != 0
    }

    /// Set EP12_IN bit atomically (RW)
    #[inline(always)]
    pub fn set_ep12_in(&self) {
        self.set_bits(Self::EP12_IN_MASK);
    }

    /// Clear EP12_IN bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep12_in(&self) {
        self.clr_bits(Self::EP12_IN_MASK);
    }

    /// **EP11_OUT** `[23]` (RW)
    #[inline(always)]
    pub fn ep11_out(&self) -> bool {
        (self.value.read_bits() & Self::EP11_OUT_MASK) != 0
    }

    /// Set EP11_OUT bit atomically (RW)
    #[inline(always)]
    pub fn set_ep11_out(&self) {
        self.set_bits(Self::EP11_OUT_MASK);
    }

    /// Clear EP11_OUT bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep11_out(&self) {
        self.clr_bits(Self::EP11_OUT_MASK);
    }

    /// **EP11_IN** `[22]` (RW)
    #[inline(always)]
    pub fn ep11_in(&self) -> bool {
        (self.value.read_bits() & Self::EP11_IN_MASK) != 0
    }

    /// Set EP11_IN bit atomically (RW)
    #[inline(always)]
    pub fn set_ep11_in(&self) {
        self.set_bits(Self::EP11_IN_MASK);
    }

    /// Clear EP11_IN bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep11_in(&self) {
        self.clr_bits(Self::EP11_IN_MASK);
    }

    /// **EP10_OUT** `[21]` (RW)
    #[inline(always)]
    pub fn ep10_out(&self) -> bool {
        (self.value.read_bits() & Self::EP10_OUT_MASK) != 0
    }

    /// Set EP10_OUT bit atomically (RW)
    #[inline(always)]
    pub fn set_ep10_out(&self) {
        self.set_bits(Self::EP10_OUT_MASK);
    }

    /// Clear EP10_OUT bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep10_out(&self) {
        self.clr_bits(Self::EP10_OUT_MASK);
    }

    /// **EP10_IN** `[20]` (RW)
    #[inline(always)]
    pub fn ep10_in(&self) -> bool {
        (self.value.read_bits() & Self::EP10_IN_MASK) != 0
    }

    /// Set EP10_IN bit atomically (RW)
    #[inline(always)]
    pub fn set_ep10_in(&self) {
        self.set_bits(Self::EP10_IN_MASK);
    }

    /// Clear EP10_IN bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep10_in(&self) {
        self.clr_bits(Self::EP10_IN_MASK);
    }

    /// **EP9_OUT** `[19]` (RW)
    #[inline(always)]
    pub fn ep9_out(&self) -> bool {
        (self.value.read_bits() & Self::EP9_OUT_MASK) != 0
    }

    /// Set EP9_OUT bit atomically (RW)
    #[inline(always)]
    pub fn set_ep9_out(&self) {
        self.set_bits(Self::EP9_OUT_MASK);
    }

    /// Clear EP9_OUT bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep9_out(&self) {
        self.clr_bits(Self::EP9_OUT_MASK);
    }

    /// **EP9_IN** `[18]` (RW)
    #[inline(always)]
    pub fn ep9_in(&self) -> bool {
        (self.value.read_bits() & Self::EP9_IN_MASK) != 0
    }

    /// Set EP9_IN bit atomically (RW)
    #[inline(always)]
    pub fn set_ep9_in(&self) {
        self.set_bits(Self::EP9_IN_MASK);
    }

    /// Clear EP9_IN bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep9_in(&self) {
        self.clr_bits(Self::EP9_IN_MASK);
    }

    /// **EP8_OUT** `[17]` (RW)
    #[inline(always)]
    pub fn ep8_out(&self) -> bool {
        (self.value.read_bits() & Self::EP8_OUT_MASK) != 0
    }

    /// Set EP8_OUT bit atomically (RW)
    #[inline(always)]
    pub fn set_ep8_out(&self) {
        self.set_bits(Self::EP8_OUT_MASK);
    }

    /// Clear EP8_OUT bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep8_out(&self) {
        self.clr_bits(Self::EP8_OUT_MASK);
    }

    /// **EP8_IN** `[16]` (RW)
    #[inline(always)]
    pub fn ep8_in(&self) -> bool {
        (self.value.read_bits() & Self::EP8_IN_MASK) != 0
    }

    /// Set EP8_IN bit atomically (RW)
    #[inline(always)]
    pub fn set_ep8_in(&self) {
        self.set_bits(Self::EP8_IN_MASK);
    }

    /// Clear EP8_IN bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep8_in(&self) {
        self.clr_bits(Self::EP8_IN_MASK);
    }

    /// **EP7_OUT** `[15]` (RW)
    #[inline(always)]
    pub fn ep7_out(&self) -> bool {
        (self.value.read_bits() & Self::EP7_OUT_MASK) != 0
    }

    /// Set EP7_OUT bit atomically (RW)
    #[inline(always)]
    pub fn set_ep7_out(&self) {
        self.set_bits(Self::EP7_OUT_MASK);
    }

    /// Clear EP7_OUT bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep7_out(&self) {
        self.clr_bits(Self::EP7_OUT_MASK);
    }

    /// **EP7_IN** `[14]` (RW)
    #[inline(always)]
    pub fn ep7_in(&self) -> bool {
        (self.value.read_bits() & Self::EP7_IN_MASK) != 0
    }

    /// Set EP7_IN bit atomically (RW)
    #[inline(always)]
    pub fn set_ep7_in(&self) {
        self.set_bits(Self::EP7_IN_MASK);
    }

    /// Clear EP7_IN bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep7_in(&self) {
        self.clr_bits(Self::EP7_IN_MASK);
    }

    /// **EP6_OUT** `[13]` (RW)
    #[inline(always)]
    pub fn ep6_out(&self) -> bool {
        (self.value.read_bits() & Self::EP6_OUT_MASK) != 0
    }

    /// Set EP6_OUT bit atomically (RW)
    #[inline(always)]
    pub fn set_ep6_out(&self) {
        self.set_bits(Self::EP6_OUT_MASK);
    }

    /// Clear EP6_OUT bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep6_out(&self) {
        self.clr_bits(Self::EP6_OUT_MASK);
    }

    /// **EP6_IN** `[12]` (RW)
    #[inline(always)]
    pub fn ep6_in(&self) -> bool {
        (self.value.read_bits() & Self::EP6_IN_MASK) != 0
    }

    /// Set EP6_IN bit atomically (RW)
    #[inline(always)]
    pub fn set_ep6_in(&self) {
        self.set_bits(Self::EP6_IN_MASK);
    }

    /// Clear EP6_IN bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep6_in(&self) {
        self.clr_bits(Self::EP6_IN_MASK);
    }

    /// **EP5_OUT** `[11]` (RW)
    #[inline(always)]
    pub fn ep5_out(&self) -> bool {
        (self.value.read_bits() & Self::EP5_OUT_MASK) != 0
    }

    /// Set EP5_OUT bit atomically (RW)
    #[inline(always)]
    pub fn set_ep5_out(&self) {
        self.set_bits(Self::EP5_OUT_MASK);
    }

    /// Clear EP5_OUT bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep5_out(&self) {
        self.clr_bits(Self::EP5_OUT_MASK);
    }

    /// **EP5_IN** `[10]` (RW)
    #[inline(always)]
    pub fn ep5_in(&self) -> bool {
        (self.value.read_bits() & Self::EP5_IN_MASK) != 0
    }

    /// Set EP5_IN bit atomically (RW)
    #[inline(always)]
    pub fn set_ep5_in(&self) {
        self.set_bits(Self::EP5_IN_MASK);
    }

    /// Clear EP5_IN bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep5_in(&self) {
        self.clr_bits(Self::EP5_IN_MASK);
    }

    /// **EP4_OUT** `[9]` (RW)
    #[inline(always)]
    pub fn ep4_out(&self) -> bool {
        (self.value.read_bits() & Self::EP4_OUT_MASK) != 0
    }

    /// Set EP4_OUT bit atomically (RW)
    #[inline(always)]
    pub fn set_ep4_out(&self) {
        self.set_bits(Self::EP4_OUT_MASK);
    }

    /// Clear EP4_OUT bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep4_out(&self) {
        self.clr_bits(Self::EP4_OUT_MASK);
    }

    /// **EP4_IN** `[8]` (RW)
    #[inline(always)]
    pub fn ep4_in(&self) -> bool {
        (self.value.read_bits() & Self::EP4_IN_MASK) != 0
    }

    /// Set EP4_IN bit atomically (RW)
    #[inline(always)]
    pub fn set_ep4_in(&self) {
        self.set_bits(Self::EP4_IN_MASK);
    }

    /// Clear EP4_IN bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep4_in(&self) {
        self.clr_bits(Self::EP4_IN_MASK);
    }

    /// **EP3_OUT** `[7]` (RW)
    #[inline(always)]
    pub fn ep3_out(&self) -> bool {
        (self.value.read_bits() & Self::EP3_OUT_MASK) != 0
    }

    /// Set EP3_OUT bit atomically (RW)
    #[inline(always)]
    pub fn set_ep3_out(&self) {
        self.set_bits(Self::EP3_OUT_MASK);
    }

    /// Clear EP3_OUT bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep3_out(&self) {
        self.clr_bits(Self::EP3_OUT_MASK);
    }

    /// **EP3_IN** `[6]` (RW)
    #[inline(always)]
    pub fn ep3_in(&self) -> bool {
        (self.value.read_bits() & Self::EP3_IN_MASK) != 0
    }

    /// Set EP3_IN bit atomically (RW)
    #[inline(always)]
    pub fn set_ep3_in(&self) {
        self.set_bits(Self::EP3_IN_MASK);
    }

    /// Clear EP3_IN bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep3_in(&self) {
        self.clr_bits(Self::EP3_IN_MASK);
    }

    /// **EP2_OUT** `[5]` (RW)
    #[inline(always)]
    pub fn ep2_out(&self) -> bool {
        (self.value.read_bits() & Self::EP2_OUT_MASK) != 0
    }

    /// Set EP2_OUT bit atomically (RW)
    #[inline(always)]
    pub fn set_ep2_out(&self) {
        self.set_bits(Self::EP2_OUT_MASK);
    }

    /// Clear EP2_OUT bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep2_out(&self) {
        self.clr_bits(Self::EP2_OUT_MASK);
    }

    /// **EP2_IN** `[4]` (RW)
    #[inline(always)]
    pub fn ep2_in(&self) -> bool {
        (self.value.read_bits() & Self::EP2_IN_MASK) != 0
    }

    /// Set EP2_IN bit atomically (RW)
    #[inline(always)]
    pub fn set_ep2_in(&self) {
        self.set_bits(Self::EP2_IN_MASK);
    }

    /// Clear EP2_IN bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep2_in(&self) {
        self.clr_bits(Self::EP2_IN_MASK);
    }

    /// **EP1_OUT** `[3]` (RW)
    #[inline(always)]
    pub fn ep1_out(&self) -> bool {
        (self.value.read_bits() & Self::EP1_OUT_MASK) != 0
    }

    /// Set EP1_OUT bit atomically (RW)
    #[inline(always)]
    pub fn set_ep1_out(&self) {
        self.set_bits(Self::EP1_OUT_MASK);
    }

    /// Clear EP1_OUT bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep1_out(&self) {
        self.clr_bits(Self::EP1_OUT_MASK);
    }

    /// **EP1_IN** `[2]` (RW)
    #[inline(always)]
    pub fn ep1_in(&self) -> bool {
        (self.value.read_bits() & Self::EP1_IN_MASK) != 0
    }

    /// Set EP1_IN bit atomically (RW)
    #[inline(always)]
    pub fn set_ep1_in(&self) {
        self.set_bits(Self::EP1_IN_MASK);
    }

    /// Clear EP1_IN bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep1_in(&self) {
        self.clr_bits(Self::EP1_IN_MASK);
    }

    /// **EP0_OUT** `[1]` (RW)
    #[inline(always)]
    pub fn ep0_out(&self) -> bool {
        (self.value.read_bits() & Self::EP0_OUT_MASK) != 0
    }

    /// Set EP0_OUT bit atomically (RW)
    #[inline(always)]
    pub fn set_ep0_out(&self) {
        self.set_bits(Self::EP0_OUT_MASK);
    }

    /// Clear EP0_OUT bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep0_out(&self) {
        self.clr_bits(Self::EP0_OUT_MASK);
    }

    /// **EP0_IN** `[0]` (RW)
    #[inline(always)]
    pub fn ep0_in(&self) -> bool {
        (self.value.read_bits() & Self::EP0_IN_MASK) != 0
    }

    /// Set EP0_IN bit atomically (RW)
    #[inline(always)]
    pub fn set_ep0_in(&self) {
        self.set_bits(Self::EP0_IN_MASK);
    }

    /// Clear EP0_IN bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep0_in(&self) {
        self.clr_bits(Self::EP0_IN_MASK);
    }
}

/// EP_ABORT value type for batch read/modify/write operations
///
/// Use `epabort.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `epabort.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EpAbortVal {
    bits: u32,
}

impl EpAbortVal {
    /// Create from raw bits
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

    /// Read **EP15_OUT** `[31:31]`
    #[inline(always)]
    pub const fn ep15_out(&self) -> bool {
        (self.bits & EpAbort::EP15_OUT_MASK) != 0
    }

    /// Read **EP15_IN** `[30:30]`
    #[inline(always)]
    pub const fn ep15_in(&self) -> bool {
        (self.bits & EpAbort::EP15_IN_MASK) != 0
    }

    /// Read **EP14_OUT** `[29:29]`
    #[inline(always)]
    pub const fn ep14_out(&self) -> bool {
        (self.bits & EpAbort::EP14_OUT_MASK) != 0
    }

    /// Read **EP14_IN** `[28:28]`
    #[inline(always)]
    pub const fn ep14_in(&self) -> bool {
        (self.bits & EpAbort::EP14_IN_MASK) != 0
    }

    /// Read **EP13_OUT** `[27:27]`
    #[inline(always)]
    pub const fn ep13_out(&self) -> bool {
        (self.bits & EpAbort::EP13_OUT_MASK) != 0
    }

    /// Read **EP13_IN** `[26:26]`
    #[inline(always)]
    pub const fn ep13_in(&self) -> bool {
        (self.bits & EpAbort::EP13_IN_MASK) != 0
    }

    /// Read **EP12_OUT** `[25:25]`
    #[inline(always)]
    pub const fn ep12_out(&self) -> bool {
        (self.bits & EpAbort::EP12_OUT_MASK) != 0
    }

    /// Read **EP12_IN** `[24:24]`
    #[inline(always)]
    pub const fn ep12_in(&self) -> bool {
        (self.bits & EpAbort::EP12_IN_MASK) != 0
    }

    /// Read **EP11_OUT** `[23:23]`
    #[inline(always)]
    pub const fn ep11_out(&self) -> bool {
        (self.bits & EpAbort::EP11_OUT_MASK) != 0
    }

    /// Read **EP11_IN** `[22:22]`
    #[inline(always)]
    pub const fn ep11_in(&self) -> bool {
        (self.bits & EpAbort::EP11_IN_MASK) != 0
    }

    /// Read **EP10_OUT** `[21:21]`
    #[inline(always)]
    pub const fn ep10_out(&self) -> bool {
        (self.bits & EpAbort::EP10_OUT_MASK) != 0
    }

    /// Read **EP10_IN** `[20:20]`
    #[inline(always)]
    pub const fn ep10_in(&self) -> bool {
        (self.bits & EpAbort::EP10_IN_MASK) != 0
    }

    /// Read **EP9_OUT** `[19:19]`
    #[inline(always)]
    pub const fn ep9_out(&self) -> bool {
        (self.bits & EpAbort::EP9_OUT_MASK) != 0
    }

    /// Read **EP9_IN** `[18:18]`
    #[inline(always)]
    pub const fn ep9_in(&self) -> bool {
        (self.bits & EpAbort::EP9_IN_MASK) != 0
    }

    /// Read **EP8_OUT** `[17:17]`
    #[inline(always)]
    pub const fn ep8_out(&self) -> bool {
        (self.bits & EpAbort::EP8_OUT_MASK) != 0
    }

    /// Read **EP8_IN** `[16:16]`
    #[inline(always)]
    pub const fn ep8_in(&self) -> bool {
        (self.bits & EpAbort::EP8_IN_MASK) != 0
    }

    /// Read **EP7_OUT** `[15:15]`
    #[inline(always)]
    pub const fn ep7_out(&self) -> bool {
        (self.bits & EpAbort::EP7_OUT_MASK) != 0
    }

    /// Read **EP7_IN** `[14:14]`
    #[inline(always)]
    pub const fn ep7_in(&self) -> bool {
        (self.bits & EpAbort::EP7_IN_MASK) != 0
    }

    /// Read **EP6_OUT** `[13:13]`
    #[inline(always)]
    pub const fn ep6_out(&self) -> bool {
        (self.bits & EpAbort::EP6_OUT_MASK) != 0
    }

    /// Read **EP6_IN** `[12:12]`
    #[inline(always)]
    pub const fn ep6_in(&self) -> bool {
        (self.bits & EpAbort::EP6_IN_MASK) != 0
    }

    /// Read **EP5_OUT** `[11:11]`
    #[inline(always)]
    pub const fn ep5_out(&self) -> bool {
        (self.bits & EpAbort::EP5_OUT_MASK) != 0
    }

    /// Read **EP5_IN** `[10:10]`
    #[inline(always)]
    pub const fn ep5_in(&self) -> bool {
        (self.bits & EpAbort::EP5_IN_MASK) != 0
    }

    /// Read **EP4_OUT** `[9:9]`
    #[inline(always)]
    pub const fn ep4_out(&self) -> bool {
        (self.bits & EpAbort::EP4_OUT_MASK) != 0
    }

    /// Read **EP4_IN** `[8:8]`
    #[inline(always)]
    pub const fn ep4_in(&self) -> bool {
        (self.bits & EpAbort::EP4_IN_MASK) != 0
    }

    /// Read **EP3_OUT** `[7:7]`
    #[inline(always)]
    pub const fn ep3_out(&self) -> bool {
        (self.bits & EpAbort::EP3_OUT_MASK) != 0
    }

    /// Read **EP3_IN** `[6:6]`
    #[inline(always)]
    pub const fn ep3_in(&self) -> bool {
        (self.bits & EpAbort::EP3_IN_MASK) != 0
    }

    /// Read **EP2_OUT** `[5:5]`
    #[inline(always)]
    pub const fn ep2_out(&self) -> bool {
        (self.bits & EpAbort::EP2_OUT_MASK) != 0
    }

    /// Read **EP2_IN** `[4:4]`
    #[inline(always)]
    pub const fn ep2_in(&self) -> bool {
        (self.bits & EpAbort::EP2_IN_MASK) != 0
    }

    /// Read **EP1_OUT** `[3:3]`
    #[inline(always)]
    pub const fn ep1_out(&self) -> bool {
        (self.bits & EpAbort::EP1_OUT_MASK) != 0
    }

    /// Read **EP1_IN** `[2:2]`
    #[inline(always)]
    pub const fn ep1_in(&self) -> bool {
        (self.bits & EpAbort::EP1_IN_MASK) != 0
    }

    /// Read **EP0_OUT** `[1:1]`
    #[inline(always)]
    pub const fn ep0_out(&self) -> bool {
        (self.bits & EpAbort::EP0_OUT_MASK) != 0
    }

    /// Read **EP0_IN** `[0:0]`
    #[inline(always)]
    pub const fn ep0_in(&self) -> bool {
        (self.bits & EpAbort::EP0_IN_MASK) != 0
    }

    /// Set **EP15_OUT** `[31:31]`
    #[inline(always)]
    pub const fn with_ep15_out(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpAbort::EP15_OUT_MASK
            } else {
                self.bits & !EpAbort::EP15_OUT_MASK
            },
        }
    }

    /// Set **EP15_IN** `[30:30]`
    #[inline(always)]
    pub const fn with_ep15_in(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpAbort::EP15_IN_MASK
            } else {
                self.bits & !EpAbort::EP15_IN_MASK
            },
        }
    }

    /// Set **EP14_OUT** `[29:29]`
    #[inline(always)]
    pub const fn with_ep14_out(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpAbort::EP14_OUT_MASK
            } else {
                self.bits & !EpAbort::EP14_OUT_MASK
            },
        }
    }

    /// Set **EP14_IN** `[28:28]`
    #[inline(always)]
    pub const fn with_ep14_in(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpAbort::EP14_IN_MASK
            } else {
                self.bits & !EpAbort::EP14_IN_MASK
            },
        }
    }

    /// Set **EP13_OUT** `[27:27]`
    #[inline(always)]
    pub const fn with_ep13_out(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpAbort::EP13_OUT_MASK
            } else {
                self.bits & !EpAbort::EP13_OUT_MASK
            },
        }
    }

    /// Set **EP13_IN** `[26:26]`
    #[inline(always)]
    pub const fn with_ep13_in(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpAbort::EP13_IN_MASK
            } else {
                self.bits & !EpAbort::EP13_IN_MASK
            },
        }
    }

    /// Set **EP12_OUT** `[25:25]`
    #[inline(always)]
    pub const fn with_ep12_out(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpAbort::EP12_OUT_MASK
            } else {
                self.bits & !EpAbort::EP12_OUT_MASK
            },
        }
    }

    /// Set **EP12_IN** `[24:24]`
    #[inline(always)]
    pub const fn with_ep12_in(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpAbort::EP12_IN_MASK
            } else {
                self.bits & !EpAbort::EP12_IN_MASK
            },
        }
    }

    /// Set **EP11_OUT** `[23:23]`
    #[inline(always)]
    pub const fn with_ep11_out(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpAbort::EP11_OUT_MASK
            } else {
                self.bits & !EpAbort::EP11_OUT_MASK
            },
        }
    }

    /// Set **EP11_IN** `[22:22]`
    #[inline(always)]
    pub const fn with_ep11_in(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpAbort::EP11_IN_MASK
            } else {
                self.bits & !EpAbort::EP11_IN_MASK
            },
        }
    }

    /// Set **EP10_OUT** `[21:21]`
    #[inline(always)]
    pub const fn with_ep10_out(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpAbort::EP10_OUT_MASK
            } else {
                self.bits & !EpAbort::EP10_OUT_MASK
            },
        }
    }

    /// Set **EP10_IN** `[20:20]`
    #[inline(always)]
    pub const fn with_ep10_in(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpAbort::EP10_IN_MASK
            } else {
                self.bits & !EpAbort::EP10_IN_MASK
            },
        }
    }

    /// Set **EP9_OUT** `[19:19]`
    #[inline(always)]
    pub const fn with_ep9_out(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpAbort::EP9_OUT_MASK
            } else {
                self.bits & !EpAbort::EP9_OUT_MASK
            },
        }
    }

    /// Set **EP9_IN** `[18:18]`
    #[inline(always)]
    pub const fn with_ep9_in(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpAbort::EP9_IN_MASK
            } else {
                self.bits & !EpAbort::EP9_IN_MASK
            },
        }
    }

    /// Set **EP8_OUT** `[17:17]`
    #[inline(always)]
    pub const fn with_ep8_out(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpAbort::EP8_OUT_MASK
            } else {
                self.bits & !EpAbort::EP8_OUT_MASK
            },
        }
    }

    /// Set **EP8_IN** `[16:16]`
    #[inline(always)]
    pub const fn with_ep8_in(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpAbort::EP8_IN_MASK
            } else {
                self.bits & !EpAbort::EP8_IN_MASK
            },
        }
    }

    /// Set **EP7_OUT** `[15:15]`
    #[inline(always)]
    pub const fn with_ep7_out(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpAbort::EP7_OUT_MASK
            } else {
                self.bits & !EpAbort::EP7_OUT_MASK
            },
        }
    }

    /// Set **EP7_IN** `[14:14]`
    #[inline(always)]
    pub const fn with_ep7_in(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpAbort::EP7_IN_MASK
            } else {
                self.bits & !EpAbort::EP7_IN_MASK
            },
        }
    }

    /// Set **EP6_OUT** `[13:13]`
    #[inline(always)]
    pub const fn with_ep6_out(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpAbort::EP6_OUT_MASK
            } else {
                self.bits & !EpAbort::EP6_OUT_MASK
            },
        }
    }

    /// Set **EP6_IN** `[12:12]`
    #[inline(always)]
    pub const fn with_ep6_in(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpAbort::EP6_IN_MASK
            } else {
                self.bits & !EpAbort::EP6_IN_MASK
            },
        }
    }

    /// Set **EP5_OUT** `[11:11]`
    #[inline(always)]
    pub const fn with_ep5_out(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpAbort::EP5_OUT_MASK
            } else {
                self.bits & !EpAbort::EP5_OUT_MASK
            },
        }
    }

    /// Set **EP5_IN** `[10:10]`
    #[inline(always)]
    pub const fn with_ep5_in(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpAbort::EP5_IN_MASK
            } else {
                self.bits & !EpAbort::EP5_IN_MASK
            },
        }
    }

    /// Set **EP4_OUT** `[9:9]`
    #[inline(always)]
    pub const fn with_ep4_out(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpAbort::EP4_OUT_MASK
            } else {
                self.bits & !EpAbort::EP4_OUT_MASK
            },
        }
    }

    /// Set **EP4_IN** `[8:8]`
    #[inline(always)]
    pub const fn with_ep4_in(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpAbort::EP4_IN_MASK
            } else {
                self.bits & !EpAbort::EP4_IN_MASK
            },
        }
    }

    /// Set **EP3_OUT** `[7:7]`
    #[inline(always)]
    pub const fn with_ep3_out(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpAbort::EP3_OUT_MASK
            } else {
                self.bits & !EpAbort::EP3_OUT_MASK
            },
        }
    }

    /// Set **EP3_IN** `[6:6]`
    #[inline(always)]
    pub const fn with_ep3_in(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpAbort::EP3_IN_MASK
            } else {
                self.bits & !EpAbort::EP3_IN_MASK
            },
        }
    }

    /// Set **EP2_OUT** `[5:5]`
    #[inline(always)]
    pub const fn with_ep2_out(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpAbort::EP2_OUT_MASK
            } else {
                self.bits & !EpAbort::EP2_OUT_MASK
            },
        }
    }

    /// Set **EP2_IN** `[4:4]`
    #[inline(always)]
    pub const fn with_ep2_in(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpAbort::EP2_IN_MASK
            } else {
                self.bits & !EpAbort::EP2_IN_MASK
            },
        }
    }

    /// Set **EP1_OUT** `[3:3]`
    #[inline(always)]
    pub const fn with_ep1_out(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpAbort::EP1_OUT_MASK
            } else {
                self.bits & !EpAbort::EP1_OUT_MASK
            },
        }
    }

    /// Set **EP1_IN** `[2:2]`
    #[inline(always)]
    pub const fn with_ep1_in(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpAbort::EP1_IN_MASK
            } else {
                self.bits & !EpAbort::EP1_IN_MASK
            },
        }
    }

    /// Set **EP0_OUT** `[1:1]`
    #[inline(always)]
    pub const fn with_ep0_out(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpAbort::EP0_OUT_MASK
            } else {
                self.bits & !EpAbort::EP0_OUT_MASK
            },
        }
    }

    /// Set **EP0_IN** `[0:0]`
    #[inline(always)]
    pub const fn with_ep0_in(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpAbort::EP0_IN_MASK
            } else {
                self.bits & !EpAbort::EP0_IN_MASK
            },
        }
    }
}

impl Default for EpAbortVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl EpAbort {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EpAbortVal {
        EpAbortVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EpAbortVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EpAbortVal) -> EpAbortVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// EP_ABORT_DONE Register
///
/// Device only: Used in conjunction with `EP_ABORT`. Set once an endpoint is idle so the programmer knows it is safe to modify the buffer control register.
///
/// Address offset: 0x0064
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **EP15_OUT** `[31]` (RW)
///
/// - **EP15_IN** `[30]` (RW)
///
/// - **EP14_OUT** `[29]` (RW)
///
/// - **EP14_IN** `[28]` (RW)
///
/// - **EP13_OUT** `[27]` (RW)
///
/// - **EP13_IN** `[26]` (RW)
///
/// - **EP12_OUT** `[25]` (RW)
///
/// - **EP12_IN** `[24]` (RW)
///
/// - **EP11_OUT** `[23]` (RW)
///
/// - **EP11_IN** `[22]` (RW)
///
/// - **EP10_OUT** `[21]` (RW)
///
/// - **EP10_IN** `[20]` (RW)
///
/// - **EP9_OUT** `[19]` (RW)
///
/// - **EP9_IN** `[18]` (RW)
///
/// - **EP8_OUT** `[17]` (RW)
///
/// - **EP8_IN** `[16]` (RW)
///
/// - **EP7_OUT** `[15]` (RW)
///
/// - **EP7_IN** `[14]` (RW)
///
/// - **EP6_OUT** `[13]` (RW)
///
/// - **EP6_IN** `[12]` (RW)
///
/// - **EP5_OUT** `[11]` (RW)
///
/// - **EP5_IN** `[10]` (RW)
///
/// - **EP4_OUT** `[9]` (RW)
///
/// - **EP4_IN** `[8]` (RW)
///
/// - **EP3_OUT** `[7]` (RW)
///
/// - **EP3_IN** `[6]` (RW)
///
/// - **EP2_OUT** `[5]` (RW)
///
/// - **EP2_IN** `[4]` (RW)
///
/// - **EP1_OUT** `[3]` (RW)
///
/// - **EP1_IN** `[2]` (RW)
///
/// - **EP0_OUT** `[1]` (RW)
///
/// - **EP0_IN** `[0]` (RW)
#[repr(transparent)]
pub struct EpAbortDone {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for EpAbortDone {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl EpAbortDone {
    // ======== Field Constants ========

    /// **EP15_OUT**
    /// Bits `[31:31]`, mask `0x80000000`
    pub const EP15_OUT_OFFSET: u32 = 31;
    pub const EP15_OUT_MASK: u32 = 0x80000000;
    pub const EP15_OUT: u32 = 1 << 31;

    /// **EP15_IN**
    /// Bits `[30:30]`, mask `0x40000000`
    pub const EP15_IN_OFFSET: u32 = 30;
    pub const EP15_IN_MASK: u32 = 0x40000000;
    pub const EP15_IN: u32 = 1 << 30;

    /// **EP14_OUT**
    /// Bits `[29:29]`, mask `0x20000000`
    pub const EP14_OUT_OFFSET: u32 = 29;
    pub const EP14_OUT_MASK: u32 = 0x20000000;
    pub const EP14_OUT: u32 = 1 << 29;

    /// **EP14_IN**
    /// Bits `[28:28]`, mask `0x10000000`
    pub const EP14_IN_OFFSET: u32 = 28;
    pub const EP14_IN_MASK: u32 = 0x10000000;
    pub const EP14_IN: u32 = 1 << 28;

    /// **EP13_OUT**
    /// Bits `[27:27]`, mask `0x08000000`
    pub const EP13_OUT_OFFSET: u32 = 27;
    pub const EP13_OUT_MASK: u32 = 0x08000000;
    pub const EP13_OUT: u32 = 1 << 27;

    /// **EP13_IN**
    /// Bits `[26:26]`, mask `0x04000000`
    pub const EP13_IN_OFFSET: u32 = 26;
    pub const EP13_IN_MASK: u32 = 0x04000000;
    pub const EP13_IN: u32 = 1 << 26;

    /// **EP12_OUT**
    /// Bits `[25:25]`, mask `0x02000000`
    pub const EP12_OUT_OFFSET: u32 = 25;
    pub const EP12_OUT_MASK: u32 = 0x02000000;
    pub const EP12_OUT: u32 = 1 << 25;

    /// **EP12_IN**
    /// Bits `[24:24]`, mask `0x01000000`
    pub const EP12_IN_OFFSET: u32 = 24;
    pub const EP12_IN_MASK: u32 = 0x01000000;
    pub const EP12_IN: u32 = 1 << 24;

    /// **EP11_OUT**
    /// Bits `[23:23]`, mask `0x00800000`
    pub const EP11_OUT_OFFSET: u32 = 23;
    pub const EP11_OUT_MASK: u32 = 0x00800000;
    pub const EP11_OUT: u32 = 1 << 23;

    /// **EP11_IN**
    /// Bits `[22:22]`, mask `0x00400000`
    pub const EP11_IN_OFFSET: u32 = 22;
    pub const EP11_IN_MASK: u32 = 0x00400000;
    pub const EP11_IN: u32 = 1 << 22;

    /// **EP10_OUT**
    /// Bits `[21:21]`, mask `0x00200000`
    pub const EP10_OUT_OFFSET: u32 = 21;
    pub const EP10_OUT_MASK: u32 = 0x00200000;
    pub const EP10_OUT: u32 = 1 << 21;

    /// **EP10_IN**
    /// Bits `[20:20]`, mask `0x00100000`
    pub const EP10_IN_OFFSET: u32 = 20;
    pub const EP10_IN_MASK: u32 = 0x00100000;
    pub const EP10_IN: u32 = 1 << 20;

    /// **EP9_OUT**
    /// Bits `[19:19]`, mask `0x00080000`
    pub const EP9_OUT_OFFSET: u32 = 19;
    pub const EP9_OUT_MASK: u32 = 0x00080000;
    pub const EP9_OUT: u32 = 1 << 19;

    /// **EP9_IN**
    /// Bits `[18:18]`, mask `0x00040000`
    pub const EP9_IN_OFFSET: u32 = 18;
    pub const EP9_IN_MASK: u32 = 0x00040000;
    pub const EP9_IN: u32 = 1 << 18;

    /// **EP8_OUT**
    /// Bits `[17:17]`, mask `0x00020000`
    pub const EP8_OUT_OFFSET: u32 = 17;
    pub const EP8_OUT_MASK: u32 = 0x00020000;
    pub const EP8_OUT: u32 = 1 << 17;

    /// **EP8_IN**
    /// Bits `[16:16]`, mask `0x00010000`
    pub const EP8_IN_OFFSET: u32 = 16;
    pub const EP8_IN_MASK: u32 = 0x00010000;
    pub const EP8_IN: u32 = 1 << 16;

    /// **EP7_OUT**
    /// Bits `[15:15]`, mask `0x00008000`
    pub const EP7_OUT_OFFSET: u32 = 15;
    pub const EP7_OUT_MASK: u32 = 0x00008000;
    pub const EP7_OUT: u32 = 1 << 15;

    /// **EP7_IN**
    /// Bits `[14:14]`, mask `0x00004000`
    pub const EP7_IN_OFFSET: u32 = 14;
    pub const EP7_IN_MASK: u32 = 0x00004000;
    pub const EP7_IN: u32 = 1 << 14;

    /// **EP6_OUT**
    /// Bits `[13:13]`, mask `0x00002000`
    pub const EP6_OUT_OFFSET: u32 = 13;
    pub const EP6_OUT_MASK: u32 = 0x00002000;
    pub const EP6_OUT: u32 = 1 << 13;

    /// **EP6_IN**
    /// Bits `[12:12]`, mask `0x00001000`
    pub const EP6_IN_OFFSET: u32 = 12;
    pub const EP6_IN_MASK: u32 = 0x00001000;
    pub const EP6_IN: u32 = 1 << 12;

    /// **EP5_OUT**
    /// Bits `[11:11]`, mask `0x00000800`
    pub const EP5_OUT_OFFSET: u32 = 11;
    pub const EP5_OUT_MASK: u32 = 0x00000800;
    pub const EP5_OUT: u32 = 1 << 11;

    /// **EP5_IN**
    /// Bits `[10:10]`, mask `0x00000400`
    pub const EP5_IN_OFFSET: u32 = 10;
    pub const EP5_IN_MASK: u32 = 0x00000400;
    pub const EP5_IN: u32 = 1 << 10;

    /// **EP4_OUT**
    /// Bits `[9:9]`, mask `0x00000200`
    pub const EP4_OUT_OFFSET: u32 = 9;
    pub const EP4_OUT_MASK: u32 = 0x00000200;
    pub const EP4_OUT: u32 = 1 << 9;

    /// **EP4_IN**
    /// Bits `[8:8]`, mask `0x00000100`
    pub const EP4_IN_OFFSET: u32 = 8;
    pub const EP4_IN_MASK: u32 = 0x00000100;
    pub const EP4_IN: u32 = 1 << 8;

    /// **EP3_OUT**
    /// Bits `[7:7]`, mask `0x00000080`
    pub const EP3_OUT_OFFSET: u32 = 7;
    pub const EP3_OUT_MASK: u32 = 0x00000080;
    pub const EP3_OUT: u32 = 1 << 7;

    /// **EP3_IN**
    /// Bits `[6:6]`, mask `0x00000040`
    pub const EP3_IN_OFFSET: u32 = 6;
    pub const EP3_IN_MASK: u32 = 0x00000040;
    pub const EP3_IN: u32 = 1 << 6;

    /// **EP2_OUT**
    /// Bits `[5:5]`, mask `0x00000020`
    pub const EP2_OUT_OFFSET: u32 = 5;
    pub const EP2_OUT_MASK: u32 = 0x00000020;
    pub const EP2_OUT: u32 = 1 << 5;

    /// **EP2_IN**
    /// Bits `[4:4]`, mask `0x00000010`
    pub const EP2_IN_OFFSET: u32 = 4;
    pub const EP2_IN_MASK: u32 = 0x00000010;
    pub const EP2_IN: u32 = 1 << 4;

    /// **EP1_OUT**
    /// Bits `[3:3]`, mask `0x00000008`
    pub const EP1_OUT_OFFSET: u32 = 3;
    pub const EP1_OUT_MASK: u32 = 0x00000008;
    pub const EP1_OUT: u32 = 1 << 3;

    /// **EP1_IN**
    /// Bits `[2:2]`, mask `0x00000004`
    pub const EP1_IN_OFFSET: u32 = 2;
    pub const EP1_IN_MASK: u32 = 0x00000004;
    pub const EP1_IN: u32 = 1 << 2;

    /// **EP0_OUT**
    /// Bits `[1:1]`, mask `0x00000002`
    pub const EP0_OUT_OFFSET: u32 = 1;
    pub const EP0_OUT_MASK: u32 = 0x00000002;
    pub const EP0_OUT: u32 = 1 << 1;

    /// **EP0_IN**
    /// Bits `[0:0]`, mask `0x00000001`
    pub const EP0_IN_OFFSET: u32 = 0;
    pub const EP0_IN_MASK: u32 = 0x00000001;
    pub const EP0_IN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **EP15_OUT** `[31]` (RW)
    #[inline(always)]
    pub fn ep15_out(&self) -> bool {
        (self.value.read_bits() & Self::EP15_OUT_MASK) != 0
    }

    /// Set EP15_OUT bit atomically (RW)
    #[inline(always)]
    pub fn set_ep15_out(&self) {
        self.set_bits(Self::EP15_OUT_MASK);
    }

    /// Clear EP15_OUT bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep15_out(&self) {
        self.clr_bits(Self::EP15_OUT_MASK);
    }

    /// **EP15_IN** `[30]` (RW)
    #[inline(always)]
    pub fn ep15_in(&self) -> bool {
        (self.value.read_bits() & Self::EP15_IN_MASK) != 0
    }

    /// Set EP15_IN bit atomically (RW)
    #[inline(always)]
    pub fn set_ep15_in(&self) {
        self.set_bits(Self::EP15_IN_MASK);
    }

    /// Clear EP15_IN bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep15_in(&self) {
        self.clr_bits(Self::EP15_IN_MASK);
    }

    /// **EP14_OUT** `[29]` (RW)
    #[inline(always)]
    pub fn ep14_out(&self) -> bool {
        (self.value.read_bits() & Self::EP14_OUT_MASK) != 0
    }

    /// Set EP14_OUT bit atomically (RW)
    #[inline(always)]
    pub fn set_ep14_out(&self) {
        self.set_bits(Self::EP14_OUT_MASK);
    }

    /// Clear EP14_OUT bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep14_out(&self) {
        self.clr_bits(Self::EP14_OUT_MASK);
    }

    /// **EP14_IN** `[28]` (RW)
    #[inline(always)]
    pub fn ep14_in(&self) -> bool {
        (self.value.read_bits() & Self::EP14_IN_MASK) != 0
    }

    /// Set EP14_IN bit atomically (RW)
    #[inline(always)]
    pub fn set_ep14_in(&self) {
        self.set_bits(Self::EP14_IN_MASK);
    }

    /// Clear EP14_IN bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep14_in(&self) {
        self.clr_bits(Self::EP14_IN_MASK);
    }

    /// **EP13_OUT** `[27]` (RW)
    #[inline(always)]
    pub fn ep13_out(&self) -> bool {
        (self.value.read_bits() & Self::EP13_OUT_MASK) != 0
    }

    /// Set EP13_OUT bit atomically (RW)
    #[inline(always)]
    pub fn set_ep13_out(&self) {
        self.set_bits(Self::EP13_OUT_MASK);
    }

    /// Clear EP13_OUT bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep13_out(&self) {
        self.clr_bits(Self::EP13_OUT_MASK);
    }

    /// **EP13_IN** `[26]` (RW)
    #[inline(always)]
    pub fn ep13_in(&self) -> bool {
        (self.value.read_bits() & Self::EP13_IN_MASK) != 0
    }

    /// Set EP13_IN bit atomically (RW)
    #[inline(always)]
    pub fn set_ep13_in(&self) {
        self.set_bits(Self::EP13_IN_MASK);
    }

    /// Clear EP13_IN bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep13_in(&self) {
        self.clr_bits(Self::EP13_IN_MASK);
    }

    /// **EP12_OUT** `[25]` (RW)
    #[inline(always)]
    pub fn ep12_out(&self) -> bool {
        (self.value.read_bits() & Self::EP12_OUT_MASK) != 0
    }

    /// Set EP12_OUT bit atomically (RW)
    #[inline(always)]
    pub fn set_ep12_out(&self) {
        self.set_bits(Self::EP12_OUT_MASK);
    }

    /// Clear EP12_OUT bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep12_out(&self) {
        self.clr_bits(Self::EP12_OUT_MASK);
    }

    /// **EP12_IN** `[24]` (RW)
    #[inline(always)]
    pub fn ep12_in(&self) -> bool {
        (self.value.read_bits() & Self::EP12_IN_MASK) != 0
    }

    /// Set EP12_IN bit atomically (RW)
    #[inline(always)]
    pub fn set_ep12_in(&self) {
        self.set_bits(Self::EP12_IN_MASK);
    }

    /// Clear EP12_IN bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep12_in(&self) {
        self.clr_bits(Self::EP12_IN_MASK);
    }

    /// **EP11_OUT** `[23]` (RW)
    #[inline(always)]
    pub fn ep11_out(&self) -> bool {
        (self.value.read_bits() & Self::EP11_OUT_MASK) != 0
    }

    /// Set EP11_OUT bit atomically (RW)
    #[inline(always)]
    pub fn set_ep11_out(&self) {
        self.set_bits(Self::EP11_OUT_MASK);
    }

    /// Clear EP11_OUT bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep11_out(&self) {
        self.clr_bits(Self::EP11_OUT_MASK);
    }

    /// **EP11_IN** `[22]` (RW)
    #[inline(always)]
    pub fn ep11_in(&self) -> bool {
        (self.value.read_bits() & Self::EP11_IN_MASK) != 0
    }

    /// Set EP11_IN bit atomically (RW)
    #[inline(always)]
    pub fn set_ep11_in(&self) {
        self.set_bits(Self::EP11_IN_MASK);
    }

    /// Clear EP11_IN bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep11_in(&self) {
        self.clr_bits(Self::EP11_IN_MASK);
    }

    /// **EP10_OUT** `[21]` (RW)
    #[inline(always)]
    pub fn ep10_out(&self) -> bool {
        (self.value.read_bits() & Self::EP10_OUT_MASK) != 0
    }

    /// Set EP10_OUT bit atomically (RW)
    #[inline(always)]
    pub fn set_ep10_out(&self) {
        self.set_bits(Self::EP10_OUT_MASK);
    }

    /// Clear EP10_OUT bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep10_out(&self) {
        self.clr_bits(Self::EP10_OUT_MASK);
    }

    /// **EP10_IN** `[20]` (RW)
    #[inline(always)]
    pub fn ep10_in(&self) -> bool {
        (self.value.read_bits() & Self::EP10_IN_MASK) != 0
    }

    /// Set EP10_IN bit atomically (RW)
    #[inline(always)]
    pub fn set_ep10_in(&self) {
        self.set_bits(Self::EP10_IN_MASK);
    }

    /// Clear EP10_IN bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep10_in(&self) {
        self.clr_bits(Self::EP10_IN_MASK);
    }

    /// **EP9_OUT** `[19]` (RW)
    #[inline(always)]
    pub fn ep9_out(&self) -> bool {
        (self.value.read_bits() & Self::EP9_OUT_MASK) != 0
    }

    /// Set EP9_OUT bit atomically (RW)
    #[inline(always)]
    pub fn set_ep9_out(&self) {
        self.set_bits(Self::EP9_OUT_MASK);
    }

    /// Clear EP9_OUT bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep9_out(&self) {
        self.clr_bits(Self::EP9_OUT_MASK);
    }

    /// **EP9_IN** `[18]` (RW)
    #[inline(always)]
    pub fn ep9_in(&self) -> bool {
        (self.value.read_bits() & Self::EP9_IN_MASK) != 0
    }

    /// Set EP9_IN bit atomically (RW)
    #[inline(always)]
    pub fn set_ep9_in(&self) {
        self.set_bits(Self::EP9_IN_MASK);
    }

    /// Clear EP9_IN bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep9_in(&self) {
        self.clr_bits(Self::EP9_IN_MASK);
    }

    /// **EP8_OUT** `[17]` (RW)
    #[inline(always)]
    pub fn ep8_out(&self) -> bool {
        (self.value.read_bits() & Self::EP8_OUT_MASK) != 0
    }

    /// Set EP8_OUT bit atomically (RW)
    #[inline(always)]
    pub fn set_ep8_out(&self) {
        self.set_bits(Self::EP8_OUT_MASK);
    }

    /// Clear EP8_OUT bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep8_out(&self) {
        self.clr_bits(Self::EP8_OUT_MASK);
    }

    /// **EP8_IN** `[16]` (RW)
    #[inline(always)]
    pub fn ep8_in(&self) -> bool {
        (self.value.read_bits() & Self::EP8_IN_MASK) != 0
    }

    /// Set EP8_IN bit atomically (RW)
    #[inline(always)]
    pub fn set_ep8_in(&self) {
        self.set_bits(Self::EP8_IN_MASK);
    }

    /// Clear EP8_IN bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep8_in(&self) {
        self.clr_bits(Self::EP8_IN_MASK);
    }

    /// **EP7_OUT** `[15]` (RW)
    #[inline(always)]
    pub fn ep7_out(&self) -> bool {
        (self.value.read_bits() & Self::EP7_OUT_MASK) != 0
    }

    /// Set EP7_OUT bit atomically (RW)
    #[inline(always)]
    pub fn set_ep7_out(&self) {
        self.set_bits(Self::EP7_OUT_MASK);
    }

    /// Clear EP7_OUT bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep7_out(&self) {
        self.clr_bits(Self::EP7_OUT_MASK);
    }

    /// **EP7_IN** `[14]` (RW)
    #[inline(always)]
    pub fn ep7_in(&self) -> bool {
        (self.value.read_bits() & Self::EP7_IN_MASK) != 0
    }

    /// Set EP7_IN bit atomically (RW)
    #[inline(always)]
    pub fn set_ep7_in(&self) {
        self.set_bits(Self::EP7_IN_MASK);
    }

    /// Clear EP7_IN bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep7_in(&self) {
        self.clr_bits(Self::EP7_IN_MASK);
    }

    /// **EP6_OUT** `[13]` (RW)
    #[inline(always)]
    pub fn ep6_out(&self) -> bool {
        (self.value.read_bits() & Self::EP6_OUT_MASK) != 0
    }

    /// Set EP6_OUT bit atomically (RW)
    #[inline(always)]
    pub fn set_ep6_out(&self) {
        self.set_bits(Self::EP6_OUT_MASK);
    }

    /// Clear EP6_OUT bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep6_out(&self) {
        self.clr_bits(Self::EP6_OUT_MASK);
    }

    /// **EP6_IN** `[12]` (RW)
    #[inline(always)]
    pub fn ep6_in(&self) -> bool {
        (self.value.read_bits() & Self::EP6_IN_MASK) != 0
    }

    /// Set EP6_IN bit atomically (RW)
    #[inline(always)]
    pub fn set_ep6_in(&self) {
        self.set_bits(Self::EP6_IN_MASK);
    }

    /// Clear EP6_IN bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep6_in(&self) {
        self.clr_bits(Self::EP6_IN_MASK);
    }

    /// **EP5_OUT** `[11]` (RW)
    #[inline(always)]
    pub fn ep5_out(&self) -> bool {
        (self.value.read_bits() & Self::EP5_OUT_MASK) != 0
    }

    /// Set EP5_OUT bit atomically (RW)
    #[inline(always)]
    pub fn set_ep5_out(&self) {
        self.set_bits(Self::EP5_OUT_MASK);
    }

    /// Clear EP5_OUT bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep5_out(&self) {
        self.clr_bits(Self::EP5_OUT_MASK);
    }

    /// **EP5_IN** `[10]` (RW)
    #[inline(always)]
    pub fn ep5_in(&self) -> bool {
        (self.value.read_bits() & Self::EP5_IN_MASK) != 0
    }

    /// Set EP5_IN bit atomically (RW)
    #[inline(always)]
    pub fn set_ep5_in(&self) {
        self.set_bits(Self::EP5_IN_MASK);
    }

    /// Clear EP5_IN bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep5_in(&self) {
        self.clr_bits(Self::EP5_IN_MASK);
    }

    /// **EP4_OUT** `[9]` (RW)
    #[inline(always)]
    pub fn ep4_out(&self) -> bool {
        (self.value.read_bits() & Self::EP4_OUT_MASK) != 0
    }

    /// Set EP4_OUT bit atomically (RW)
    #[inline(always)]
    pub fn set_ep4_out(&self) {
        self.set_bits(Self::EP4_OUT_MASK);
    }

    /// Clear EP4_OUT bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep4_out(&self) {
        self.clr_bits(Self::EP4_OUT_MASK);
    }

    /// **EP4_IN** `[8]` (RW)
    #[inline(always)]
    pub fn ep4_in(&self) -> bool {
        (self.value.read_bits() & Self::EP4_IN_MASK) != 0
    }

    /// Set EP4_IN bit atomically (RW)
    #[inline(always)]
    pub fn set_ep4_in(&self) {
        self.set_bits(Self::EP4_IN_MASK);
    }

    /// Clear EP4_IN bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep4_in(&self) {
        self.clr_bits(Self::EP4_IN_MASK);
    }

    /// **EP3_OUT** `[7]` (RW)
    #[inline(always)]
    pub fn ep3_out(&self) -> bool {
        (self.value.read_bits() & Self::EP3_OUT_MASK) != 0
    }

    /// Set EP3_OUT bit atomically (RW)
    #[inline(always)]
    pub fn set_ep3_out(&self) {
        self.set_bits(Self::EP3_OUT_MASK);
    }

    /// Clear EP3_OUT bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep3_out(&self) {
        self.clr_bits(Self::EP3_OUT_MASK);
    }

    /// **EP3_IN** `[6]` (RW)
    #[inline(always)]
    pub fn ep3_in(&self) -> bool {
        (self.value.read_bits() & Self::EP3_IN_MASK) != 0
    }

    /// Set EP3_IN bit atomically (RW)
    #[inline(always)]
    pub fn set_ep3_in(&self) {
        self.set_bits(Self::EP3_IN_MASK);
    }

    /// Clear EP3_IN bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep3_in(&self) {
        self.clr_bits(Self::EP3_IN_MASK);
    }

    /// **EP2_OUT** `[5]` (RW)
    #[inline(always)]
    pub fn ep2_out(&self) -> bool {
        (self.value.read_bits() & Self::EP2_OUT_MASK) != 0
    }

    /// Set EP2_OUT bit atomically (RW)
    #[inline(always)]
    pub fn set_ep2_out(&self) {
        self.set_bits(Self::EP2_OUT_MASK);
    }

    /// Clear EP2_OUT bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep2_out(&self) {
        self.clr_bits(Self::EP2_OUT_MASK);
    }

    /// **EP2_IN** `[4]` (RW)
    #[inline(always)]
    pub fn ep2_in(&self) -> bool {
        (self.value.read_bits() & Self::EP2_IN_MASK) != 0
    }

    /// Set EP2_IN bit atomically (RW)
    #[inline(always)]
    pub fn set_ep2_in(&self) {
        self.set_bits(Self::EP2_IN_MASK);
    }

    /// Clear EP2_IN bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep2_in(&self) {
        self.clr_bits(Self::EP2_IN_MASK);
    }

    /// **EP1_OUT** `[3]` (RW)
    #[inline(always)]
    pub fn ep1_out(&self) -> bool {
        (self.value.read_bits() & Self::EP1_OUT_MASK) != 0
    }

    /// Set EP1_OUT bit atomically (RW)
    #[inline(always)]
    pub fn set_ep1_out(&self) {
        self.set_bits(Self::EP1_OUT_MASK);
    }

    /// Clear EP1_OUT bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep1_out(&self) {
        self.clr_bits(Self::EP1_OUT_MASK);
    }

    /// **EP1_IN** `[2]` (RW)
    #[inline(always)]
    pub fn ep1_in(&self) -> bool {
        (self.value.read_bits() & Self::EP1_IN_MASK) != 0
    }

    /// Set EP1_IN bit atomically (RW)
    #[inline(always)]
    pub fn set_ep1_in(&self) {
        self.set_bits(Self::EP1_IN_MASK);
    }

    /// Clear EP1_IN bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep1_in(&self) {
        self.clr_bits(Self::EP1_IN_MASK);
    }

    /// **EP0_OUT** `[1]` (RW)
    #[inline(always)]
    pub fn ep0_out(&self) -> bool {
        (self.value.read_bits() & Self::EP0_OUT_MASK) != 0
    }

    /// Set EP0_OUT bit atomically (RW)
    #[inline(always)]
    pub fn set_ep0_out(&self) {
        self.set_bits(Self::EP0_OUT_MASK);
    }

    /// Clear EP0_OUT bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep0_out(&self) {
        self.clr_bits(Self::EP0_OUT_MASK);
    }

    /// **EP0_IN** `[0]` (RW)
    #[inline(always)]
    pub fn ep0_in(&self) -> bool {
        (self.value.read_bits() & Self::EP0_IN_MASK) != 0
    }

    /// Set EP0_IN bit atomically (RW)
    #[inline(always)]
    pub fn set_ep0_in(&self) {
        self.set_bits(Self::EP0_IN_MASK);
    }

    /// Clear EP0_IN bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep0_in(&self) {
        self.clr_bits(Self::EP0_IN_MASK);
    }
}

/// EP_ABORT_DONE value type for batch read/modify/write operations
///
/// Use `epabortdone.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `epabortdone.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EpAbortDoneVal {
    bits: u32,
}

impl EpAbortDoneVal {
    /// Create from raw bits
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

    /// Read **EP15_OUT** `[31:31]`
    #[inline(always)]
    pub const fn ep15_out(&self) -> bool {
        (self.bits & EpAbortDone::EP15_OUT_MASK) != 0
    }

    /// Read **EP15_IN** `[30:30]`
    #[inline(always)]
    pub const fn ep15_in(&self) -> bool {
        (self.bits & EpAbortDone::EP15_IN_MASK) != 0
    }

    /// Read **EP14_OUT** `[29:29]`
    #[inline(always)]
    pub const fn ep14_out(&self) -> bool {
        (self.bits & EpAbortDone::EP14_OUT_MASK) != 0
    }

    /// Read **EP14_IN** `[28:28]`
    #[inline(always)]
    pub const fn ep14_in(&self) -> bool {
        (self.bits & EpAbortDone::EP14_IN_MASK) != 0
    }

    /// Read **EP13_OUT** `[27:27]`
    #[inline(always)]
    pub const fn ep13_out(&self) -> bool {
        (self.bits & EpAbortDone::EP13_OUT_MASK) != 0
    }

    /// Read **EP13_IN** `[26:26]`
    #[inline(always)]
    pub const fn ep13_in(&self) -> bool {
        (self.bits & EpAbortDone::EP13_IN_MASK) != 0
    }

    /// Read **EP12_OUT** `[25:25]`
    #[inline(always)]
    pub const fn ep12_out(&self) -> bool {
        (self.bits & EpAbortDone::EP12_OUT_MASK) != 0
    }

    /// Read **EP12_IN** `[24:24]`
    #[inline(always)]
    pub const fn ep12_in(&self) -> bool {
        (self.bits & EpAbortDone::EP12_IN_MASK) != 0
    }

    /// Read **EP11_OUT** `[23:23]`
    #[inline(always)]
    pub const fn ep11_out(&self) -> bool {
        (self.bits & EpAbortDone::EP11_OUT_MASK) != 0
    }

    /// Read **EP11_IN** `[22:22]`
    #[inline(always)]
    pub const fn ep11_in(&self) -> bool {
        (self.bits & EpAbortDone::EP11_IN_MASK) != 0
    }

    /// Read **EP10_OUT** `[21:21]`
    #[inline(always)]
    pub const fn ep10_out(&self) -> bool {
        (self.bits & EpAbortDone::EP10_OUT_MASK) != 0
    }

    /// Read **EP10_IN** `[20:20]`
    #[inline(always)]
    pub const fn ep10_in(&self) -> bool {
        (self.bits & EpAbortDone::EP10_IN_MASK) != 0
    }

    /// Read **EP9_OUT** `[19:19]`
    #[inline(always)]
    pub const fn ep9_out(&self) -> bool {
        (self.bits & EpAbortDone::EP9_OUT_MASK) != 0
    }

    /// Read **EP9_IN** `[18:18]`
    #[inline(always)]
    pub const fn ep9_in(&self) -> bool {
        (self.bits & EpAbortDone::EP9_IN_MASK) != 0
    }

    /// Read **EP8_OUT** `[17:17]`
    #[inline(always)]
    pub const fn ep8_out(&self) -> bool {
        (self.bits & EpAbortDone::EP8_OUT_MASK) != 0
    }

    /// Read **EP8_IN** `[16:16]`
    #[inline(always)]
    pub const fn ep8_in(&self) -> bool {
        (self.bits & EpAbortDone::EP8_IN_MASK) != 0
    }

    /// Read **EP7_OUT** `[15:15]`
    #[inline(always)]
    pub const fn ep7_out(&self) -> bool {
        (self.bits & EpAbortDone::EP7_OUT_MASK) != 0
    }

    /// Read **EP7_IN** `[14:14]`
    #[inline(always)]
    pub const fn ep7_in(&self) -> bool {
        (self.bits & EpAbortDone::EP7_IN_MASK) != 0
    }

    /// Read **EP6_OUT** `[13:13]`
    #[inline(always)]
    pub const fn ep6_out(&self) -> bool {
        (self.bits & EpAbortDone::EP6_OUT_MASK) != 0
    }

    /// Read **EP6_IN** `[12:12]`
    #[inline(always)]
    pub const fn ep6_in(&self) -> bool {
        (self.bits & EpAbortDone::EP6_IN_MASK) != 0
    }

    /// Read **EP5_OUT** `[11:11]`
    #[inline(always)]
    pub const fn ep5_out(&self) -> bool {
        (self.bits & EpAbortDone::EP5_OUT_MASK) != 0
    }

    /// Read **EP5_IN** `[10:10]`
    #[inline(always)]
    pub const fn ep5_in(&self) -> bool {
        (self.bits & EpAbortDone::EP5_IN_MASK) != 0
    }

    /// Read **EP4_OUT** `[9:9]`
    #[inline(always)]
    pub const fn ep4_out(&self) -> bool {
        (self.bits & EpAbortDone::EP4_OUT_MASK) != 0
    }

    /// Read **EP4_IN** `[8:8]`
    #[inline(always)]
    pub const fn ep4_in(&self) -> bool {
        (self.bits & EpAbortDone::EP4_IN_MASK) != 0
    }

    /// Read **EP3_OUT** `[7:7]`
    #[inline(always)]
    pub const fn ep3_out(&self) -> bool {
        (self.bits & EpAbortDone::EP3_OUT_MASK) != 0
    }

    /// Read **EP3_IN** `[6:6]`
    #[inline(always)]
    pub const fn ep3_in(&self) -> bool {
        (self.bits & EpAbortDone::EP3_IN_MASK) != 0
    }

    /// Read **EP2_OUT** `[5:5]`
    #[inline(always)]
    pub const fn ep2_out(&self) -> bool {
        (self.bits & EpAbortDone::EP2_OUT_MASK) != 0
    }

    /// Read **EP2_IN** `[4:4]`
    #[inline(always)]
    pub const fn ep2_in(&self) -> bool {
        (self.bits & EpAbortDone::EP2_IN_MASK) != 0
    }

    /// Read **EP1_OUT** `[3:3]`
    #[inline(always)]
    pub const fn ep1_out(&self) -> bool {
        (self.bits & EpAbortDone::EP1_OUT_MASK) != 0
    }

    /// Read **EP1_IN** `[2:2]`
    #[inline(always)]
    pub const fn ep1_in(&self) -> bool {
        (self.bits & EpAbortDone::EP1_IN_MASK) != 0
    }

    /// Read **EP0_OUT** `[1:1]`
    #[inline(always)]
    pub const fn ep0_out(&self) -> bool {
        (self.bits & EpAbortDone::EP0_OUT_MASK) != 0
    }

    /// Read **EP0_IN** `[0:0]`
    #[inline(always)]
    pub const fn ep0_in(&self) -> bool {
        (self.bits & EpAbortDone::EP0_IN_MASK) != 0
    }

    /// Set **EP15_OUT** `[31:31]`
    #[inline(always)]
    pub const fn with_ep15_out(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpAbortDone::EP15_OUT_MASK
            } else {
                self.bits & !EpAbortDone::EP15_OUT_MASK
            },
        }
    }

    /// Set **EP15_IN** `[30:30]`
    #[inline(always)]
    pub const fn with_ep15_in(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpAbortDone::EP15_IN_MASK
            } else {
                self.bits & !EpAbortDone::EP15_IN_MASK
            },
        }
    }

    /// Set **EP14_OUT** `[29:29]`
    #[inline(always)]
    pub const fn with_ep14_out(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpAbortDone::EP14_OUT_MASK
            } else {
                self.bits & !EpAbortDone::EP14_OUT_MASK
            },
        }
    }

    /// Set **EP14_IN** `[28:28]`
    #[inline(always)]
    pub const fn with_ep14_in(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpAbortDone::EP14_IN_MASK
            } else {
                self.bits & !EpAbortDone::EP14_IN_MASK
            },
        }
    }

    /// Set **EP13_OUT** `[27:27]`
    #[inline(always)]
    pub const fn with_ep13_out(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpAbortDone::EP13_OUT_MASK
            } else {
                self.bits & !EpAbortDone::EP13_OUT_MASK
            },
        }
    }

    /// Set **EP13_IN** `[26:26]`
    #[inline(always)]
    pub const fn with_ep13_in(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpAbortDone::EP13_IN_MASK
            } else {
                self.bits & !EpAbortDone::EP13_IN_MASK
            },
        }
    }

    /// Set **EP12_OUT** `[25:25]`
    #[inline(always)]
    pub const fn with_ep12_out(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpAbortDone::EP12_OUT_MASK
            } else {
                self.bits & !EpAbortDone::EP12_OUT_MASK
            },
        }
    }

    /// Set **EP12_IN** `[24:24]`
    #[inline(always)]
    pub const fn with_ep12_in(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpAbortDone::EP12_IN_MASK
            } else {
                self.bits & !EpAbortDone::EP12_IN_MASK
            },
        }
    }

    /// Set **EP11_OUT** `[23:23]`
    #[inline(always)]
    pub const fn with_ep11_out(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpAbortDone::EP11_OUT_MASK
            } else {
                self.bits & !EpAbortDone::EP11_OUT_MASK
            },
        }
    }

    /// Set **EP11_IN** `[22:22]`
    #[inline(always)]
    pub const fn with_ep11_in(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpAbortDone::EP11_IN_MASK
            } else {
                self.bits & !EpAbortDone::EP11_IN_MASK
            },
        }
    }

    /// Set **EP10_OUT** `[21:21]`
    #[inline(always)]
    pub const fn with_ep10_out(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpAbortDone::EP10_OUT_MASK
            } else {
                self.bits & !EpAbortDone::EP10_OUT_MASK
            },
        }
    }

    /// Set **EP10_IN** `[20:20]`
    #[inline(always)]
    pub const fn with_ep10_in(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpAbortDone::EP10_IN_MASK
            } else {
                self.bits & !EpAbortDone::EP10_IN_MASK
            },
        }
    }

    /// Set **EP9_OUT** `[19:19]`
    #[inline(always)]
    pub const fn with_ep9_out(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpAbortDone::EP9_OUT_MASK
            } else {
                self.bits & !EpAbortDone::EP9_OUT_MASK
            },
        }
    }

    /// Set **EP9_IN** `[18:18]`
    #[inline(always)]
    pub const fn with_ep9_in(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpAbortDone::EP9_IN_MASK
            } else {
                self.bits & !EpAbortDone::EP9_IN_MASK
            },
        }
    }

    /// Set **EP8_OUT** `[17:17]`
    #[inline(always)]
    pub const fn with_ep8_out(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpAbortDone::EP8_OUT_MASK
            } else {
                self.bits & !EpAbortDone::EP8_OUT_MASK
            },
        }
    }

    /// Set **EP8_IN** `[16:16]`
    #[inline(always)]
    pub const fn with_ep8_in(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpAbortDone::EP8_IN_MASK
            } else {
                self.bits & !EpAbortDone::EP8_IN_MASK
            },
        }
    }

    /// Set **EP7_OUT** `[15:15]`
    #[inline(always)]
    pub const fn with_ep7_out(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpAbortDone::EP7_OUT_MASK
            } else {
                self.bits & !EpAbortDone::EP7_OUT_MASK
            },
        }
    }

    /// Set **EP7_IN** `[14:14]`
    #[inline(always)]
    pub const fn with_ep7_in(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpAbortDone::EP7_IN_MASK
            } else {
                self.bits & !EpAbortDone::EP7_IN_MASK
            },
        }
    }

    /// Set **EP6_OUT** `[13:13]`
    #[inline(always)]
    pub const fn with_ep6_out(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpAbortDone::EP6_OUT_MASK
            } else {
                self.bits & !EpAbortDone::EP6_OUT_MASK
            },
        }
    }

    /// Set **EP6_IN** `[12:12]`
    #[inline(always)]
    pub const fn with_ep6_in(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpAbortDone::EP6_IN_MASK
            } else {
                self.bits & !EpAbortDone::EP6_IN_MASK
            },
        }
    }

    /// Set **EP5_OUT** `[11:11]`
    #[inline(always)]
    pub const fn with_ep5_out(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpAbortDone::EP5_OUT_MASK
            } else {
                self.bits & !EpAbortDone::EP5_OUT_MASK
            },
        }
    }

    /// Set **EP5_IN** `[10:10]`
    #[inline(always)]
    pub const fn with_ep5_in(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpAbortDone::EP5_IN_MASK
            } else {
                self.bits & !EpAbortDone::EP5_IN_MASK
            },
        }
    }

    /// Set **EP4_OUT** `[9:9]`
    #[inline(always)]
    pub const fn with_ep4_out(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpAbortDone::EP4_OUT_MASK
            } else {
                self.bits & !EpAbortDone::EP4_OUT_MASK
            },
        }
    }

    /// Set **EP4_IN** `[8:8]`
    #[inline(always)]
    pub const fn with_ep4_in(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpAbortDone::EP4_IN_MASK
            } else {
                self.bits & !EpAbortDone::EP4_IN_MASK
            },
        }
    }

    /// Set **EP3_OUT** `[7:7]`
    #[inline(always)]
    pub const fn with_ep3_out(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpAbortDone::EP3_OUT_MASK
            } else {
                self.bits & !EpAbortDone::EP3_OUT_MASK
            },
        }
    }

    /// Set **EP3_IN** `[6:6]`
    #[inline(always)]
    pub const fn with_ep3_in(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpAbortDone::EP3_IN_MASK
            } else {
                self.bits & !EpAbortDone::EP3_IN_MASK
            },
        }
    }

    /// Set **EP2_OUT** `[5:5]`
    #[inline(always)]
    pub const fn with_ep2_out(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpAbortDone::EP2_OUT_MASK
            } else {
                self.bits & !EpAbortDone::EP2_OUT_MASK
            },
        }
    }

    /// Set **EP2_IN** `[4:4]`
    #[inline(always)]
    pub const fn with_ep2_in(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpAbortDone::EP2_IN_MASK
            } else {
                self.bits & !EpAbortDone::EP2_IN_MASK
            },
        }
    }

    /// Set **EP1_OUT** `[3:3]`
    #[inline(always)]
    pub const fn with_ep1_out(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpAbortDone::EP1_OUT_MASK
            } else {
                self.bits & !EpAbortDone::EP1_OUT_MASK
            },
        }
    }

    /// Set **EP1_IN** `[2:2]`
    #[inline(always)]
    pub const fn with_ep1_in(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpAbortDone::EP1_IN_MASK
            } else {
                self.bits & !EpAbortDone::EP1_IN_MASK
            },
        }
    }

    /// Set **EP0_OUT** `[1:1]`
    #[inline(always)]
    pub const fn with_ep0_out(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpAbortDone::EP0_OUT_MASK
            } else {
                self.bits & !EpAbortDone::EP0_OUT_MASK
            },
        }
    }

    /// Set **EP0_IN** `[0:0]`
    #[inline(always)]
    pub const fn with_ep0_in(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpAbortDone::EP0_IN_MASK
            } else {
                self.bits & !EpAbortDone::EP0_IN_MASK
            },
        }
    }
}

impl Default for EpAbortDoneVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl EpAbortDone {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EpAbortDoneVal {
        EpAbortDoneVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EpAbortDoneVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EpAbortDoneVal) -> EpAbortDoneVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// EP_STALL_ARM Register
///
/// Device: this bit must be set in conjunction with the `STALL` bit in the buffer control register to send a STALL on EP0. The device controller clears these bits when a SETUP packet is received because the USB spec requires that a STALL condition is cleared when a SETUP packet is received.
///
/// Address offset: 0x0068
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **EP0_OUT** `[1]` (RW)
///
/// - **EP0_IN** `[0]` (RW)
#[repr(transparent)]
pub struct EpStallArm {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for EpStallArm {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl EpStallArm {
    // ======== Field Constants ========

    /// **EP0_OUT**
    /// Bits `[1:1]`, mask `0x00000002`
    pub const EP0_OUT_OFFSET: u32 = 1;
    pub const EP0_OUT_MASK: u32 = 0x00000002;
    pub const EP0_OUT: u32 = 1 << 1;

    /// **EP0_IN**
    /// Bits `[0:0]`, mask `0x00000001`
    pub const EP0_IN_OFFSET: u32 = 0;
    pub const EP0_IN_MASK: u32 = 0x00000001;
    pub const EP0_IN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **EP0_OUT** `[1]` (RW)
    #[inline(always)]
    pub fn ep0_out(&self) -> bool {
        (self.value.read_bits() & Self::EP0_OUT_MASK) != 0
    }

    /// Set EP0_OUT bit atomically (RW)
    #[inline(always)]
    pub fn set_ep0_out(&self) {
        self.set_bits(Self::EP0_OUT_MASK);
    }

    /// Clear EP0_OUT bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep0_out(&self) {
        self.clr_bits(Self::EP0_OUT_MASK);
    }

    /// **EP0_IN** `[0]` (RW)
    #[inline(always)]
    pub fn ep0_in(&self) -> bool {
        (self.value.read_bits() & Self::EP0_IN_MASK) != 0
    }

    /// Set EP0_IN bit atomically (RW)
    #[inline(always)]
    pub fn set_ep0_in(&self) {
        self.set_bits(Self::EP0_IN_MASK);
    }

    /// Clear EP0_IN bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep0_in(&self) {
        self.clr_bits(Self::EP0_IN_MASK);
    }
}

/// EP_STALL_ARM value type for batch read/modify/write operations
///
/// Use `epstallarm.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `epstallarm.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EpStallArmVal {
    bits: u32,
}

impl EpStallArmVal {
    /// Create from raw bits
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

    /// Read **EP0_OUT** `[1:1]`
    #[inline(always)]
    pub const fn ep0_out(&self) -> bool {
        (self.bits & EpStallArm::EP0_OUT_MASK) != 0
    }

    /// Read **EP0_IN** `[0:0]`
    #[inline(always)]
    pub const fn ep0_in(&self) -> bool {
        (self.bits & EpStallArm::EP0_IN_MASK) != 0
    }

    /// Set **EP0_OUT** `[1:1]`
    #[inline(always)]
    pub const fn with_ep0_out(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpStallArm::EP0_OUT_MASK
            } else {
                self.bits & !EpStallArm::EP0_OUT_MASK
            },
        }
    }

    /// Set **EP0_IN** `[0:0]`
    #[inline(always)]
    pub const fn with_ep0_in(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpStallArm::EP0_IN_MASK
            } else {
                self.bits & !EpStallArm::EP0_IN_MASK
            },
        }
    }
}

impl Default for EpStallArmVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl EpStallArm {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EpStallArmVal {
        EpStallArmVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EpStallArmVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EpStallArmVal) -> EpStallArmVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// NAK_POLL Register
///
/// Used by the host controller. Sets the wait time in microseconds before trying again if the device replies with a NAK.
///
/// Address offset: 0x006c
/// Reset value: 0x00100010
///
/// ## Fields
///
/// - **RETRY_COUNT_HI** `[31:28]` (RO)
///
///   Bits 9:6 of nak_retry count
///
/// - **EPX_STOPPED_ON_NAK** `[27]` (RW)
///
///   EPX polling has stopped because a nak was received
///
/// - **STOP_EPX_ON_NAK** `[26]` (RW)
///
///   Stop polling epx when a nak is received
///
/// - **DELAY_FS** `[25:16]` (RW)
///
///   NAK polling interval for a full speed device
///
/// - **RETRY_COUNT_LO** `[15:10]` (RO)
///
///   Bits 5:0 of nak_retry_count
///
/// - **DELAY_LS** `[9:0]` (RW)
///
///   NAK polling interval for a low speed device
#[repr(transparent)]
pub struct NakPoll {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for NakPoll {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl NakPoll {
    // ======== Field Constants ========

    /// **RETRY_COUNT_HI**
    ///
    /// Bits 9:6 of nak_retry count
    ///
    /// Bits `[31:28]`, mask `0xf0000000`
    pub const RETRY_COUNT_HI_OFFSET: u32 = 28;
    pub const RETRY_COUNT_HI_MASK: u32 = 0xf0000000;

    /// **EPX_STOPPED_ON_NAK**
    ///
    /// EPX polling has stopped because a nak was received
    ///
    /// Bits `[27:27]`, mask `0x08000000`
    pub const EPX_STOPPED_ON_NAK_OFFSET: u32 = 27;
    pub const EPX_STOPPED_ON_NAK_MASK: u32 = 0x08000000;
    pub const EPX_STOPPED_ON_NAK: u32 = 1 << 27;

    /// **STOP_EPX_ON_NAK**
    ///
    /// Stop polling epx when a nak is received
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const STOP_EPX_ON_NAK_OFFSET: u32 = 26;
    pub const STOP_EPX_ON_NAK_MASK: u32 = 0x04000000;
    pub const STOP_EPX_ON_NAK: u32 = 1 << 26;

    /// **DELAY_FS**
    ///
    /// NAK polling interval for a full speed device
    ///
    /// Bits `[25:16]`, mask `0x03ff0000`
    pub const DELAY_FS_OFFSET: u32 = 16;
    pub const DELAY_FS_MASK: u32 = 0x03ff0000;

    /// **RETRY_COUNT_LO**
    ///
    /// Bits 5:0 of nak_retry_count
    ///
    /// Bits `[15:10]`, mask `0x0000fc00`
    pub const RETRY_COUNT_LO_OFFSET: u32 = 10;
    pub const RETRY_COUNT_LO_MASK: u32 = 0x0000fc00;

    /// **DELAY_LS**
    ///
    /// NAK polling interval for a low speed device
    ///
    /// Bits `[9:0]`, mask `0x000003ff`
    pub const DELAY_LS_OFFSET: u32 = 0;
    pub const DELAY_LS_MASK: u32 = 0x000003ff;

    // ======== Field Accessors ========

    /// **RETRY_COUNT_HI** `[31:28]` (RO)
    ///
    /// Bits 9:6 of nak_retry count
    #[inline(always)]
    pub fn retry_count_hi(&self) -> u32 {
        (self.value.read_bits() & Self::RETRY_COUNT_HI_MASK) >> Self::RETRY_COUNT_HI_OFFSET
    }

    /// **EPX_STOPPED_ON_NAK** `[27]` (RW)
    ///
    /// EPX polling has stopped because a nak was received
    #[inline(always)]
    pub fn epx_stopped_on_nak(&self) -> bool {
        (self.value.read_bits() & Self::EPX_STOPPED_ON_NAK_MASK) != 0
    }

    /// Set EPX_STOPPED_ON_NAK bit atomically (RW)
    #[inline(always)]
    pub fn set_epx_stopped_on_nak(&self) {
        self.set_bits(Self::EPX_STOPPED_ON_NAK_MASK);
    }

    /// Clear EPX_STOPPED_ON_NAK bit atomically (RW)
    #[inline(always)]
    pub fn clr_epx_stopped_on_nak(&self) {
        self.clr_bits(Self::EPX_STOPPED_ON_NAK_MASK);
    }

    /// **STOP_EPX_ON_NAK** `[26]` (RW)
    ///
    /// Stop polling epx when a nak is received
    #[inline(always)]
    pub fn stop_epx_on_nak(&self) -> bool {
        (self.value.read_bits() & Self::STOP_EPX_ON_NAK_MASK) != 0
    }

    /// Set STOP_EPX_ON_NAK bit atomically (RW)
    #[inline(always)]
    pub fn set_stop_epx_on_nak(&self) {
        self.set_bits(Self::STOP_EPX_ON_NAK_MASK);
    }

    /// Clear STOP_EPX_ON_NAK bit atomically (RW)
    #[inline(always)]
    pub fn clr_stop_epx_on_nak(&self) {
        self.clr_bits(Self::STOP_EPX_ON_NAK_MASK);
    }

    /// **DELAY_FS** `[25:16]` (RW)
    ///
    /// NAK polling interval for a full speed device
    #[inline(always)]
    pub fn delay_fs(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_FS_MASK) >> Self::DELAY_FS_OFFSET
    }

    /// Write **DELAY_FS** `[25:16]` (RW)
    ///
    /// NAK polling interval for a full speed device
    #[inline(always)]
    pub fn write_delay_fs(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DELAY_FS_MASK) | ((value << Self::DELAY_FS_OFFSET) & Self::DELAY_FS_MASK)
        });
    }

    /// **RETRY_COUNT_LO** `[15:10]` (RO)
    ///
    /// Bits 5:0 of nak_retry_count
    #[inline(always)]
    pub fn retry_count_lo(&self) -> u32 {
        (self.value.read_bits() & Self::RETRY_COUNT_LO_MASK) >> Self::RETRY_COUNT_LO_OFFSET
    }

    /// **DELAY_LS** `[9:0]` (RW)
    ///
    /// NAK polling interval for a low speed device
    #[inline(always)]
    pub fn delay_ls(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_LS_MASK) >> Self::DELAY_LS_OFFSET
    }

    /// Write **DELAY_LS** `[9:0]` (RW)
    ///
    /// NAK polling interval for a low speed device
    #[inline(always)]
    pub fn write_delay_ls(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DELAY_LS_MASK) | ((value << Self::DELAY_LS_OFFSET) & Self::DELAY_LS_MASK)
        });
    }
}

/// NAK_POLL value type for batch read/modify/write operations
///
/// Use `nakpoll.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `nakpoll.write_val(val)`.
#[derive(Clone, Copy)]
pub struct NakPollVal {
    bits: u32,
}

impl NakPollVal {
    /// Create from raw bits
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
        Self { bits: 0x00100010 }
    }

    /// Read **RETRY_COUNT_HI** `[31:28]`
    ///
    /// Bits 9:6 of nak_retry count
    #[inline(always)]
    pub const fn retry_count_hi(&self) -> u32 {
        (self.bits & NakPoll::RETRY_COUNT_HI_MASK) >> NakPoll::RETRY_COUNT_HI_OFFSET
    }

    /// Read **EPX_STOPPED_ON_NAK** `[27:27]`
    ///
    /// EPX polling has stopped because a nak was received
    #[inline(always)]
    pub const fn epx_stopped_on_nak(&self) -> bool {
        (self.bits & NakPoll::EPX_STOPPED_ON_NAK_MASK) != 0
    }

    /// Read **STOP_EPX_ON_NAK** `[26:26]`
    ///
    /// Stop polling epx when a nak is received
    #[inline(always)]
    pub const fn stop_epx_on_nak(&self) -> bool {
        (self.bits & NakPoll::STOP_EPX_ON_NAK_MASK) != 0
    }

    /// Read **DELAY_FS** `[25:16]`
    ///
    /// NAK polling interval for a full speed device
    #[inline(always)]
    pub const fn delay_fs(&self) -> u32 {
        (self.bits & NakPoll::DELAY_FS_MASK) >> NakPoll::DELAY_FS_OFFSET
    }

    /// Read **RETRY_COUNT_LO** `[15:10]`
    ///
    /// Bits 5:0 of nak_retry_count
    #[inline(always)]
    pub const fn retry_count_lo(&self) -> u32 {
        (self.bits & NakPoll::RETRY_COUNT_LO_MASK) >> NakPoll::RETRY_COUNT_LO_OFFSET
    }

    /// Read **DELAY_LS** `[9:0]`
    ///
    /// NAK polling interval for a low speed device
    #[inline(always)]
    pub const fn delay_ls(&self) -> u32 {
        (self.bits & NakPoll::DELAY_LS_MASK) >> NakPoll::DELAY_LS_OFFSET
    }

    /// Set **EPX_STOPPED_ON_NAK** `[27:27]`
    ///
    /// EPX polling has stopped because a nak was received
    #[inline(always)]
    pub const fn with_epx_stopped_on_nak(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | NakPoll::EPX_STOPPED_ON_NAK_MASK
            } else {
                self.bits & !NakPoll::EPX_STOPPED_ON_NAK_MASK
            },
        }
    }

    /// Set **STOP_EPX_ON_NAK** `[26:26]`
    ///
    /// Stop polling epx when a nak is received
    #[inline(always)]
    pub const fn with_stop_epx_on_nak(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | NakPoll::STOP_EPX_ON_NAK_MASK
            } else {
                self.bits & !NakPoll::STOP_EPX_ON_NAK_MASK
            },
        }
    }

    /// Set **DELAY_FS** `[25:16]`
    ///
    /// NAK polling interval for a full speed device
    #[inline(always)]
    pub const fn with_delay_fs(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !NakPoll::DELAY_FS_MASK)
                | ((value << NakPoll::DELAY_FS_OFFSET) & NakPoll::DELAY_FS_MASK),
        }
    }

    /// Set **DELAY_LS** `[9:0]`
    ///
    /// NAK polling interval for a low speed device
    #[inline(always)]
    pub const fn with_delay_ls(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !NakPoll::DELAY_LS_MASK)
                | ((value << NakPoll::DELAY_LS_OFFSET) & NakPoll::DELAY_LS_MASK),
        }
    }
}

impl Default for NakPollVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl NakPoll {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> NakPollVal {
        NakPollVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: NakPollVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(NakPollVal) -> NakPollVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// EP_STATUS_STALL_NAK Register
///
/// Device: bits are set when the `IRQ_ON_NAK` or `IRQ_ON_STALL` bits are set. For EP0 this comes from `SIE_CTRL`. For all other endpoints it comes from the endpoint control register.
///
/// Address offset: 0x0070
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **EP15_OUT** `[31]` (RW)
///
/// - **EP15_IN** `[30]` (RW)
///
/// - **EP14_OUT** `[29]` (RW)
///
/// - **EP14_IN** `[28]` (RW)
///
/// - **EP13_OUT** `[27]` (RW)
///
/// - **EP13_IN** `[26]` (RW)
///
/// - **EP12_OUT** `[25]` (RW)
///
/// - **EP12_IN** `[24]` (RW)
///
/// - **EP11_OUT** `[23]` (RW)
///
/// - **EP11_IN** `[22]` (RW)
///
/// - **EP10_OUT** `[21]` (RW)
///
/// - **EP10_IN** `[20]` (RW)
///
/// - **EP9_OUT** `[19]` (RW)
///
/// - **EP9_IN** `[18]` (RW)
///
/// - **EP8_OUT** `[17]` (RW)
///
/// - **EP8_IN** `[16]` (RW)
///
/// - **EP7_OUT** `[15]` (RW)
///
/// - **EP7_IN** `[14]` (RW)
///
/// - **EP6_OUT** `[13]` (RW)
///
/// - **EP6_IN** `[12]` (RW)
///
/// - **EP5_OUT** `[11]` (RW)
///
/// - **EP5_IN** `[10]` (RW)
///
/// - **EP4_OUT** `[9]` (RW)
///
/// - **EP4_IN** `[8]` (RW)
///
/// - **EP3_OUT** `[7]` (RW)
///
/// - **EP3_IN** `[6]` (RW)
///
/// - **EP2_OUT** `[5]` (RW)
///
/// - **EP2_IN** `[4]` (RW)
///
/// - **EP1_OUT** `[3]` (RW)
///
/// - **EP1_IN** `[2]` (RW)
///
/// - **EP0_OUT** `[1]` (RW)
///
/// - **EP0_IN** `[0]` (RW)
#[repr(transparent)]
pub struct EpStatusStallNak {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for EpStatusStallNak {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl EpStatusStallNak {
    // ======== Field Constants ========

    /// **EP15_OUT**
    /// Bits `[31:31]`, mask `0x80000000`
    pub const EP15_OUT_OFFSET: u32 = 31;
    pub const EP15_OUT_MASK: u32 = 0x80000000;
    pub const EP15_OUT: u32 = 1 << 31;

    /// **EP15_IN**
    /// Bits `[30:30]`, mask `0x40000000`
    pub const EP15_IN_OFFSET: u32 = 30;
    pub const EP15_IN_MASK: u32 = 0x40000000;
    pub const EP15_IN: u32 = 1 << 30;

    /// **EP14_OUT**
    /// Bits `[29:29]`, mask `0x20000000`
    pub const EP14_OUT_OFFSET: u32 = 29;
    pub const EP14_OUT_MASK: u32 = 0x20000000;
    pub const EP14_OUT: u32 = 1 << 29;

    /// **EP14_IN**
    /// Bits `[28:28]`, mask `0x10000000`
    pub const EP14_IN_OFFSET: u32 = 28;
    pub const EP14_IN_MASK: u32 = 0x10000000;
    pub const EP14_IN: u32 = 1 << 28;

    /// **EP13_OUT**
    /// Bits `[27:27]`, mask `0x08000000`
    pub const EP13_OUT_OFFSET: u32 = 27;
    pub const EP13_OUT_MASK: u32 = 0x08000000;
    pub const EP13_OUT: u32 = 1 << 27;

    /// **EP13_IN**
    /// Bits `[26:26]`, mask `0x04000000`
    pub const EP13_IN_OFFSET: u32 = 26;
    pub const EP13_IN_MASK: u32 = 0x04000000;
    pub const EP13_IN: u32 = 1 << 26;

    /// **EP12_OUT**
    /// Bits `[25:25]`, mask `0x02000000`
    pub const EP12_OUT_OFFSET: u32 = 25;
    pub const EP12_OUT_MASK: u32 = 0x02000000;
    pub const EP12_OUT: u32 = 1 << 25;

    /// **EP12_IN**
    /// Bits `[24:24]`, mask `0x01000000`
    pub const EP12_IN_OFFSET: u32 = 24;
    pub const EP12_IN_MASK: u32 = 0x01000000;
    pub const EP12_IN: u32 = 1 << 24;

    /// **EP11_OUT**
    /// Bits `[23:23]`, mask `0x00800000`
    pub const EP11_OUT_OFFSET: u32 = 23;
    pub const EP11_OUT_MASK: u32 = 0x00800000;
    pub const EP11_OUT: u32 = 1 << 23;

    /// **EP11_IN**
    /// Bits `[22:22]`, mask `0x00400000`
    pub const EP11_IN_OFFSET: u32 = 22;
    pub const EP11_IN_MASK: u32 = 0x00400000;
    pub const EP11_IN: u32 = 1 << 22;

    /// **EP10_OUT**
    /// Bits `[21:21]`, mask `0x00200000`
    pub const EP10_OUT_OFFSET: u32 = 21;
    pub const EP10_OUT_MASK: u32 = 0x00200000;
    pub const EP10_OUT: u32 = 1 << 21;

    /// **EP10_IN**
    /// Bits `[20:20]`, mask `0x00100000`
    pub const EP10_IN_OFFSET: u32 = 20;
    pub const EP10_IN_MASK: u32 = 0x00100000;
    pub const EP10_IN: u32 = 1 << 20;

    /// **EP9_OUT**
    /// Bits `[19:19]`, mask `0x00080000`
    pub const EP9_OUT_OFFSET: u32 = 19;
    pub const EP9_OUT_MASK: u32 = 0x00080000;
    pub const EP9_OUT: u32 = 1 << 19;

    /// **EP9_IN**
    /// Bits `[18:18]`, mask `0x00040000`
    pub const EP9_IN_OFFSET: u32 = 18;
    pub const EP9_IN_MASK: u32 = 0x00040000;
    pub const EP9_IN: u32 = 1 << 18;

    /// **EP8_OUT**
    /// Bits `[17:17]`, mask `0x00020000`
    pub const EP8_OUT_OFFSET: u32 = 17;
    pub const EP8_OUT_MASK: u32 = 0x00020000;
    pub const EP8_OUT: u32 = 1 << 17;

    /// **EP8_IN**
    /// Bits `[16:16]`, mask `0x00010000`
    pub const EP8_IN_OFFSET: u32 = 16;
    pub const EP8_IN_MASK: u32 = 0x00010000;
    pub const EP8_IN: u32 = 1 << 16;

    /// **EP7_OUT**
    /// Bits `[15:15]`, mask `0x00008000`
    pub const EP7_OUT_OFFSET: u32 = 15;
    pub const EP7_OUT_MASK: u32 = 0x00008000;
    pub const EP7_OUT: u32 = 1 << 15;

    /// **EP7_IN**
    /// Bits `[14:14]`, mask `0x00004000`
    pub const EP7_IN_OFFSET: u32 = 14;
    pub const EP7_IN_MASK: u32 = 0x00004000;
    pub const EP7_IN: u32 = 1 << 14;

    /// **EP6_OUT**
    /// Bits `[13:13]`, mask `0x00002000`
    pub const EP6_OUT_OFFSET: u32 = 13;
    pub const EP6_OUT_MASK: u32 = 0x00002000;
    pub const EP6_OUT: u32 = 1 << 13;

    /// **EP6_IN**
    /// Bits `[12:12]`, mask `0x00001000`
    pub const EP6_IN_OFFSET: u32 = 12;
    pub const EP6_IN_MASK: u32 = 0x00001000;
    pub const EP6_IN: u32 = 1 << 12;

    /// **EP5_OUT**
    /// Bits `[11:11]`, mask `0x00000800`
    pub const EP5_OUT_OFFSET: u32 = 11;
    pub const EP5_OUT_MASK: u32 = 0x00000800;
    pub const EP5_OUT: u32 = 1 << 11;

    /// **EP5_IN**
    /// Bits `[10:10]`, mask `0x00000400`
    pub const EP5_IN_OFFSET: u32 = 10;
    pub const EP5_IN_MASK: u32 = 0x00000400;
    pub const EP5_IN: u32 = 1 << 10;

    /// **EP4_OUT**
    /// Bits `[9:9]`, mask `0x00000200`
    pub const EP4_OUT_OFFSET: u32 = 9;
    pub const EP4_OUT_MASK: u32 = 0x00000200;
    pub const EP4_OUT: u32 = 1 << 9;

    /// **EP4_IN**
    /// Bits `[8:8]`, mask `0x00000100`
    pub const EP4_IN_OFFSET: u32 = 8;
    pub const EP4_IN_MASK: u32 = 0x00000100;
    pub const EP4_IN: u32 = 1 << 8;

    /// **EP3_OUT**
    /// Bits `[7:7]`, mask `0x00000080`
    pub const EP3_OUT_OFFSET: u32 = 7;
    pub const EP3_OUT_MASK: u32 = 0x00000080;
    pub const EP3_OUT: u32 = 1 << 7;

    /// **EP3_IN**
    /// Bits `[6:6]`, mask `0x00000040`
    pub const EP3_IN_OFFSET: u32 = 6;
    pub const EP3_IN_MASK: u32 = 0x00000040;
    pub const EP3_IN: u32 = 1 << 6;

    /// **EP2_OUT**
    /// Bits `[5:5]`, mask `0x00000020`
    pub const EP2_OUT_OFFSET: u32 = 5;
    pub const EP2_OUT_MASK: u32 = 0x00000020;
    pub const EP2_OUT: u32 = 1 << 5;

    /// **EP2_IN**
    /// Bits `[4:4]`, mask `0x00000010`
    pub const EP2_IN_OFFSET: u32 = 4;
    pub const EP2_IN_MASK: u32 = 0x00000010;
    pub const EP2_IN: u32 = 1 << 4;

    /// **EP1_OUT**
    /// Bits `[3:3]`, mask `0x00000008`
    pub const EP1_OUT_OFFSET: u32 = 3;
    pub const EP1_OUT_MASK: u32 = 0x00000008;
    pub const EP1_OUT: u32 = 1 << 3;

    /// **EP1_IN**
    /// Bits `[2:2]`, mask `0x00000004`
    pub const EP1_IN_OFFSET: u32 = 2;
    pub const EP1_IN_MASK: u32 = 0x00000004;
    pub const EP1_IN: u32 = 1 << 2;

    /// **EP0_OUT**
    /// Bits `[1:1]`, mask `0x00000002`
    pub const EP0_OUT_OFFSET: u32 = 1;
    pub const EP0_OUT_MASK: u32 = 0x00000002;
    pub const EP0_OUT: u32 = 1 << 1;

    /// **EP0_IN**
    /// Bits `[0:0]`, mask `0x00000001`
    pub const EP0_IN_OFFSET: u32 = 0;
    pub const EP0_IN_MASK: u32 = 0x00000001;
    pub const EP0_IN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **EP15_OUT** `[31]` (RW)
    #[inline(always)]
    pub fn ep15_out(&self) -> bool {
        (self.value.read_bits() & Self::EP15_OUT_MASK) != 0
    }

    /// Set EP15_OUT bit atomically (RW)
    #[inline(always)]
    pub fn set_ep15_out(&self) {
        self.set_bits(Self::EP15_OUT_MASK);
    }

    /// Clear EP15_OUT bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep15_out(&self) {
        self.clr_bits(Self::EP15_OUT_MASK);
    }

    /// **EP15_IN** `[30]` (RW)
    #[inline(always)]
    pub fn ep15_in(&self) -> bool {
        (self.value.read_bits() & Self::EP15_IN_MASK) != 0
    }

    /// Set EP15_IN bit atomically (RW)
    #[inline(always)]
    pub fn set_ep15_in(&self) {
        self.set_bits(Self::EP15_IN_MASK);
    }

    /// Clear EP15_IN bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep15_in(&self) {
        self.clr_bits(Self::EP15_IN_MASK);
    }

    /// **EP14_OUT** `[29]` (RW)
    #[inline(always)]
    pub fn ep14_out(&self) -> bool {
        (self.value.read_bits() & Self::EP14_OUT_MASK) != 0
    }

    /// Set EP14_OUT bit atomically (RW)
    #[inline(always)]
    pub fn set_ep14_out(&self) {
        self.set_bits(Self::EP14_OUT_MASK);
    }

    /// Clear EP14_OUT bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep14_out(&self) {
        self.clr_bits(Self::EP14_OUT_MASK);
    }

    /// **EP14_IN** `[28]` (RW)
    #[inline(always)]
    pub fn ep14_in(&self) -> bool {
        (self.value.read_bits() & Self::EP14_IN_MASK) != 0
    }

    /// Set EP14_IN bit atomically (RW)
    #[inline(always)]
    pub fn set_ep14_in(&self) {
        self.set_bits(Self::EP14_IN_MASK);
    }

    /// Clear EP14_IN bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep14_in(&self) {
        self.clr_bits(Self::EP14_IN_MASK);
    }

    /// **EP13_OUT** `[27]` (RW)
    #[inline(always)]
    pub fn ep13_out(&self) -> bool {
        (self.value.read_bits() & Self::EP13_OUT_MASK) != 0
    }

    /// Set EP13_OUT bit atomically (RW)
    #[inline(always)]
    pub fn set_ep13_out(&self) {
        self.set_bits(Self::EP13_OUT_MASK);
    }

    /// Clear EP13_OUT bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep13_out(&self) {
        self.clr_bits(Self::EP13_OUT_MASK);
    }

    /// **EP13_IN** `[26]` (RW)
    #[inline(always)]
    pub fn ep13_in(&self) -> bool {
        (self.value.read_bits() & Self::EP13_IN_MASK) != 0
    }

    /// Set EP13_IN bit atomically (RW)
    #[inline(always)]
    pub fn set_ep13_in(&self) {
        self.set_bits(Self::EP13_IN_MASK);
    }

    /// Clear EP13_IN bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep13_in(&self) {
        self.clr_bits(Self::EP13_IN_MASK);
    }

    /// **EP12_OUT** `[25]` (RW)
    #[inline(always)]
    pub fn ep12_out(&self) -> bool {
        (self.value.read_bits() & Self::EP12_OUT_MASK) != 0
    }

    /// Set EP12_OUT bit atomically (RW)
    #[inline(always)]
    pub fn set_ep12_out(&self) {
        self.set_bits(Self::EP12_OUT_MASK);
    }

    /// Clear EP12_OUT bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep12_out(&self) {
        self.clr_bits(Self::EP12_OUT_MASK);
    }

    /// **EP12_IN** `[24]` (RW)
    #[inline(always)]
    pub fn ep12_in(&self) -> bool {
        (self.value.read_bits() & Self::EP12_IN_MASK) != 0
    }

    /// Set EP12_IN bit atomically (RW)
    #[inline(always)]
    pub fn set_ep12_in(&self) {
        self.set_bits(Self::EP12_IN_MASK);
    }

    /// Clear EP12_IN bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep12_in(&self) {
        self.clr_bits(Self::EP12_IN_MASK);
    }

    /// **EP11_OUT** `[23]` (RW)
    #[inline(always)]
    pub fn ep11_out(&self) -> bool {
        (self.value.read_bits() & Self::EP11_OUT_MASK) != 0
    }

    /// Set EP11_OUT bit atomically (RW)
    #[inline(always)]
    pub fn set_ep11_out(&self) {
        self.set_bits(Self::EP11_OUT_MASK);
    }

    /// Clear EP11_OUT bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep11_out(&self) {
        self.clr_bits(Self::EP11_OUT_MASK);
    }

    /// **EP11_IN** `[22]` (RW)
    #[inline(always)]
    pub fn ep11_in(&self) -> bool {
        (self.value.read_bits() & Self::EP11_IN_MASK) != 0
    }

    /// Set EP11_IN bit atomically (RW)
    #[inline(always)]
    pub fn set_ep11_in(&self) {
        self.set_bits(Self::EP11_IN_MASK);
    }

    /// Clear EP11_IN bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep11_in(&self) {
        self.clr_bits(Self::EP11_IN_MASK);
    }

    /// **EP10_OUT** `[21]` (RW)
    #[inline(always)]
    pub fn ep10_out(&self) -> bool {
        (self.value.read_bits() & Self::EP10_OUT_MASK) != 0
    }

    /// Set EP10_OUT bit atomically (RW)
    #[inline(always)]
    pub fn set_ep10_out(&self) {
        self.set_bits(Self::EP10_OUT_MASK);
    }

    /// Clear EP10_OUT bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep10_out(&self) {
        self.clr_bits(Self::EP10_OUT_MASK);
    }

    /// **EP10_IN** `[20]` (RW)
    #[inline(always)]
    pub fn ep10_in(&self) -> bool {
        (self.value.read_bits() & Self::EP10_IN_MASK) != 0
    }

    /// Set EP10_IN bit atomically (RW)
    #[inline(always)]
    pub fn set_ep10_in(&self) {
        self.set_bits(Self::EP10_IN_MASK);
    }

    /// Clear EP10_IN bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep10_in(&self) {
        self.clr_bits(Self::EP10_IN_MASK);
    }

    /// **EP9_OUT** `[19]` (RW)
    #[inline(always)]
    pub fn ep9_out(&self) -> bool {
        (self.value.read_bits() & Self::EP9_OUT_MASK) != 0
    }

    /// Set EP9_OUT bit atomically (RW)
    #[inline(always)]
    pub fn set_ep9_out(&self) {
        self.set_bits(Self::EP9_OUT_MASK);
    }

    /// Clear EP9_OUT bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep9_out(&self) {
        self.clr_bits(Self::EP9_OUT_MASK);
    }

    /// **EP9_IN** `[18]` (RW)
    #[inline(always)]
    pub fn ep9_in(&self) -> bool {
        (self.value.read_bits() & Self::EP9_IN_MASK) != 0
    }

    /// Set EP9_IN bit atomically (RW)
    #[inline(always)]
    pub fn set_ep9_in(&self) {
        self.set_bits(Self::EP9_IN_MASK);
    }

    /// Clear EP9_IN bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep9_in(&self) {
        self.clr_bits(Self::EP9_IN_MASK);
    }

    /// **EP8_OUT** `[17]` (RW)
    #[inline(always)]
    pub fn ep8_out(&self) -> bool {
        (self.value.read_bits() & Self::EP8_OUT_MASK) != 0
    }

    /// Set EP8_OUT bit atomically (RW)
    #[inline(always)]
    pub fn set_ep8_out(&self) {
        self.set_bits(Self::EP8_OUT_MASK);
    }

    /// Clear EP8_OUT bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep8_out(&self) {
        self.clr_bits(Self::EP8_OUT_MASK);
    }

    /// **EP8_IN** `[16]` (RW)
    #[inline(always)]
    pub fn ep8_in(&self) -> bool {
        (self.value.read_bits() & Self::EP8_IN_MASK) != 0
    }

    /// Set EP8_IN bit atomically (RW)
    #[inline(always)]
    pub fn set_ep8_in(&self) {
        self.set_bits(Self::EP8_IN_MASK);
    }

    /// Clear EP8_IN bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep8_in(&self) {
        self.clr_bits(Self::EP8_IN_MASK);
    }

    /// **EP7_OUT** `[15]` (RW)
    #[inline(always)]
    pub fn ep7_out(&self) -> bool {
        (self.value.read_bits() & Self::EP7_OUT_MASK) != 0
    }

    /// Set EP7_OUT bit atomically (RW)
    #[inline(always)]
    pub fn set_ep7_out(&self) {
        self.set_bits(Self::EP7_OUT_MASK);
    }

    /// Clear EP7_OUT bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep7_out(&self) {
        self.clr_bits(Self::EP7_OUT_MASK);
    }

    /// **EP7_IN** `[14]` (RW)
    #[inline(always)]
    pub fn ep7_in(&self) -> bool {
        (self.value.read_bits() & Self::EP7_IN_MASK) != 0
    }

    /// Set EP7_IN bit atomically (RW)
    #[inline(always)]
    pub fn set_ep7_in(&self) {
        self.set_bits(Self::EP7_IN_MASK);
    }

    /// Clear EP7_IN bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep7_in(&self) {
        self.clr_bits(Self::EP7_IN_MASK);
    }

    /// **EP6_OUT** `[13]` (RW)
    #[inline(always)]
    pub fn ep6_out(&self) -> bool {
        (self.value.read_bits() & Self::EP6_OUT_MASK) != 0
    }

    /// Set EP6_OUT bit atomically (RW)
    #[inline(always)]
    pub fn set_ep6_out(&self) {
        self.set_bits(Self::EP6_OUT_MASK);
    }

    /// Clear EP6_OUT bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep6_out(&self) {
        self.clr_bits(Self::EP6_OUT_MASK);
    }

    /// **EP6_IN** `[12]` (RW)
    #[inline(always)]
    pub fn ep6_in(&self) -> bool {
        (self.value.read_bits() & Self::EP6_IN_MASK) != 0
    }

    /// Set EP6_IN bit atomically (RW)
    #[inline(always)]
    pub fn set_ep6_in(&self) {
        self.set_bits(Self::EP6_IN_MASK);
    }

    /// Clear EP6_IN bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep6_in(&self) {
        self.clr_bits(Self::EP6_IN_MASK);
    }

    /// **EP5_OUT** `[11]` (RW)
    #[inline(always)]
    pub fn ep5_out(&self) -> bool {
        (self.value.read_bits() & Self::EP5_OUT_MASK) != 0
    }

    /// Set EP5_OUT bit atomically (RW)
    #[inline(always)]
    pub fn set_ep5_out(&self) {
        self.set_bits(Self::EP5_OUT_MASK);
    }

    /// Clear EP5_OUT bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep5_out(&self) {
        self.clr_bits(Self::EP5_OUT_MASK);
    }

    /// **EP5_IN** `[10]` (RW)
    #[inline(always)]
    pub fn ep5_in(&self) -> bool {
        (self.value.read_bits() & Self::EP5_IN_MASK) != 0
    }

    /// Set EP5_IN bit atomically (RW)
    #[inline(always)]
    pub fn set_ep5_in(&self) {
        self.set_bits(Self::EP5_IN_MASK);
    }

    /// Clear EP5_IN bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep5_in(&self) {
        self.clr_bits(Self::EP5_IN_MASK);
    }

    /// **EP4_OUT** `[9]` (RW)
    #[inline(always)]
    pub fn ep4_out(&self) -> bool {
        (self.value.read_bits() & Self::EP4_OUT_MASK) != 0
    }

    /// Set EP4_OUT bit atomically (RW)
    #[inline(always)]
    pub fn set_ep4_out(&self) {
        self.set_bits(Self::EP4_OUT_MASK);
    }

    /// Clear EP4_OUT bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep4_out(&self) {
        self.clr_bits(Self::EP4_OUT_MASK);
    }

    /// **EP4_IN** `[8]` (RW)
    #[inline(always)]
    pub fn ep4_in(&self) -> bool {
        (self.value.read_bits() & Self::EP4_IN_MASK) != 0
    }

    /// Set EP4_IN bit atomically (RW)
    #[inline(always)]
    pub fn set_ep4_in(&self) {
        self.set_bits(Self::EP4_IN_MASK);
    }

    /// Clear EP4_IN bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep4_in(&self) {
        self.clr_bits(Self::EP4_IN_MASK);
    }

    /// **EP3_OUT** `[7]` (RW)
    #[inline(always)]
    pub fn ep3_out(&self) -> bool {
        (self.value.read_bits() & Self::EP3_OUT_MASK) != 0
    }

    /// Set EP3_OUT bit atomically (RW)
    #[inline(always)]
    pub fn set_ep3_out(&self) {
        self.set_bits(Self::EP3_OUT_MASK);
    }

    /// Clear EP3_OUT bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep3_out(&self) {
        self.clr_bits(Self::EP3_OUT_MASK);
    }

    /// **EP3_IN** `[6]` (RW)
    #[inline(always)]
    pub fn ep3_in(&self) -> bool {
        (self.value.read_bits() & Self::EP3_IN_MASK) != 0
    }

    /// Set EP3_IN bit atomically (RW)
    #[inline(always)]
    pub fn set_ep3_in(&self) {
        self.set_bits(Self::EP3_IN_MASK);
    }

    /// Clear EP3_IN bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep3_in(&self) {
        self.clr_bits(Self::EP3_IN_MASK);
    }

    /// **EP2_OUT** `[5]` (RW)
    #[inline(always)]
    pub fn ep2_out(&self) -> bool {
        (self.value.read_bits() & Self::EP2_OUT_MASK) != 0
    }

    /// Set EP2_OUT bit atomically (RW)
    #[inline(always)]
    pub fn set_ep2_out(&self) {
        self.set_bits(Self::EP2_OUT_MASK);
    }

    /// Clear EP2_OUT bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep2_out(&self) {
        self.clr_bits(Self::EP2_OUT_MASK);
    }

    /// **EP2_IN** `[4]` (RW)
    #[inline(always)]
    pub fn ep2_in(&self) -> bool {
        (self.value.read_bits() & Self::EP2_IN_MASK) != 0
    }

    /// Set EP2_IN bit atomically (RW)
    #[inline(always)]
    pub fn set_ep2_in(&self) {
        self.set_bits(Self::EP2_IN_MASK);
    }

    /// Clear EP2_IN bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep2_in(&self) {
        self.clr_bits(Self::EP2_IN_MASK);
    }

    /// **EP1_OUT** `[3]` (RW)
    #[inline(always)]
    pub fn ep1_out(&self) -> bool {
        (self.value.read_bits() & Self::EP1_OUT_MASK) != 0
    }

    /// Set EP1_OUT bit atomically (RW)
    #[inline(always)]
    pub fn set_ep1_out(&self) {
        self.set_bits(Self::EP1_OUT_MASK);
    }

    /// Clear EP1_OUT bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep1_out(&self) {
        self.clr_bits(Self::EP1_OUT_MASK);
    }

    /// **EP1_IN** `[2]` (RW)
    #[inline(always)]
    pub fn ep1_in(&self) -> bool {
        (self.value.read_bits() & Self::EP1_IN_MASK) != 0
    }

    /// Set EP1_IN bit atomically (RW)
    #[inline(always)]
    pub fn set_ep1_in(&self) {
        self.set_bits(Self::EP1_IN_MASK);
    }

    /// Clear EP1_IN bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep1_in(&self) {
        self.clr_bits(Self::EP1_IN_MASK);
    }

    /// **EP0_OUT** `[1]` (RW)
    #[inline(always)]
    pub fn ep0_out(&self) -> bool {
        (self.value.read_bits() & Self::EP0_OUT_MASK) != 0
    }

    /// Set EP0_OUT bit atomically (RW)
    #[inline(always)]
    pub fn set_ep0_out(&self) {
        self.set_bits(Self::EP0_OUT_MASK);
    }

    /// Clear EP0_OUT bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep0_out(&self) {
        self.clr_bits(Self::EP0_OUT_MASK);
    }

    /// **EP0_IN** `[0]` (RW)
    #[inline(always)]
    pub fn ep0_in(&self) -> bool {
        (self.value.read_bits() & Self::EP0_IN_MASK) != 0
    }

    /// Set EP0_IN bit atomically (RW)
    #[inline(always)]
    pub fn set_ep0_in(&self) {
        self.set_bits(Self::EP0_IN_MASK);
    }

    /// Clear EP0_IN bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep0_in(&self) {
        self.clr_bits(Self::EP0_IN_MASK);
    }
}

/// EP_STATUS_STALL_NAK value type for batch read/modify/write operations
///
/// Use `epstatusstallnak.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `epstatusstallnak.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EpStatusStallNakVal {
    bits: u32,
}

impl EpStatusStallNakVal {
    /// Create from raw bits
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

    /// Read **EP15_OUT** `[31:31]`
    #[inline(always)]
    pub const fn ep15_out(&self) -> bool {
        (self.bits & EpStatusStallNak::EP15_OUT_MASK) != 0
    }

    /// Read **EP15_IN** `[30:30]`
    #[inline(always)]
    pub const fn ep15_in(&self) -> bool {
        (self.bits & EpStatusStallNak::EP15_IN_MASK) != 0
    }

    /// Read **EP14_OUT** `[29:29]`
    #[inline(always)]
    pub const fn ep14_out(&self) -> bool {
        (self.bits & EpStatusStallNak::EP14_OUT_MASK) != 0
    }

    /// Read **EP14_IN** `[28:28]`
    #[inline(always)]
    pub const fn ep14_in(&self) -> bool {
        (self.bits & EpStatusStallNak::EP14_IN_MASK) != 0
    }

    /// Read **EP13_OUT** `[27:27]`
    #[inline(always)]
    pub const fn ep13_out(&self) -> bool {
        (self.bits & EpStatusStallNak::EP13_OUT_MASK) != 0
    }

    /// Read **EP13_IN** `[26:26]`
    #[inline(always)]
    pub const fn ep13_in(&self) -> bool {
        (self.bits & EpStatusStallNak::EP13_IN_MASK) != 0
    }

    /// Read **EP12_OUT** `[25:25]`
    #[inline(always)]
    pub const fn ep12_out(&self) -> bool {
        (self.bits & EpStatusStallNak::EP12_OUT_MASK) != 0
    }

    /// Read **EP12_IN** `[24:24]`
    #[inline(always)]
    pub const fn ep12_in(&self) -> bool {
        (self.bits & EpStatusStallNak::EP12_IN_MASK) != 0
    }

    /// Read **EP11_OUT** `[23:23]`
    #[inline(always)]
    pub const fn ep11_out(&self) -> bool {
        (self.bits & EpStatusStallNak::EP11_OUT_MASK) != 0
    }

    /// Read **EP11_IN** `[22:22]`
    #[inline(always)]
    pub const fn ep11_in(&self) -> bool {
        (self.bits & EpStatusStallNak::EP11_IN_MASK) != 0
    }

    /// Read **EP10_OUT** `[21:21]`
    #[inline(always)]
    pub const fn ep10_out(&self) -> bool {
        (self.bits & EpStatusStallNak::EP10_OUT_MASK) != 0
    }

    /// Read **EP10_IN** `[20:20]`
    #[inline(always)]
    pub const fn ep10_in(&self) -> bool {
        (self.bits & EpStatusStallNak::EP10_IN_MASK) != 0
    }

    /// Read **EP9_OUT** `[19:19]`
    #[inline(always)]
    pub const fn ep9_out(&self) -> bool {
        (self.bits & EpStatusStallNak::EP9_OUT_MASK) != 0
    }

    /// Read **EP9_IN** `[18:18]`
    #[inline(always)]
    pub const fn ep9_in(&self) -> bool {
        (self.bits & EpStatusStallNak::EP9_IN_MASK) != 0
    }

    /// Read **EP8_OUT** `[17:17]`
    #[inline(always)]
    pub const fn ep8_out(&self) -> bool {
        (self.bits & EpStatusStallNak::EP8_OUT_MASK) != 0
    }

    /// Read **EP8_IN** `[16:16]`
    #[inline(always)]
    pub const fn ep8_in(&self) -> bool {
        (self.bits & EpStatusStallNak::EP8_IN_MASK) != 0
    }

    /// Read **EP7_OUT** `[15:15]`
    #[inline(always)]
    pub const fn ep7_out(&self) -> bool {
        (self.bits & EpStatusStallNak::EP7_OUT_MASK) != 0
    }

    /// Read **EP7_IN** `[14:14]`
    #[inline(always)]
    pub const fn ep7_in(&self) -> bool {
        (self.bits & EpStatusStallNak::EP7_IN_MASK) != 0
    }

    /// Read **EP6_OUT** `[13:13]`
    #[inline(always)]
    pub const fn ep6_out(&self) -> bool {
        (self.bits & EpStatusStallNak::EP6_OUT_MASK) != 0
    }

    /// Read **EP6_IN** `[12:12]`
    #[inline(always)]
    pub const fn ep6_in(&self) -> bool {
        (self.bits & EpStatusStallNak::EP6_IN_MASK) != 0
    }

    /// Read **EP5_OUT** `[11:11]`
    #[inline(always)]
    pub const fn ep5_out(&self) -> bool {
        (self.bits & EpStatusStallNak::EP5_OUT_MASK) != 0
    }

    /// Read **EP5_IN** `[10:10]`
    #[inline(always)]
    pub const fn ep5_in(&self) -> bool {
        (self.bits & EpStatusStallNak::EP5_IN_MASK) != 0
    }

    /// Read **EP4_OUT** `[9:9]`
    #[inline(always)]
    pub const fn ep4_out(&self) -> bool {
        (self.bits & EpStatusStallNak::EP4_OUT_MASK) != 0
    }

    /// Read **EP4_IN** `[8:8]`
    #[inline(always)]
    pub const fn ep4_in(&self) -> bool {
        (self.bits & EpStatusStallNak::EP4_IN_MASK) != 0
    }

    /// Read **EP3_OUT** `[7:7]`
    #[inline(always)]
    pub const fn ep3_out(&self) -> bool {
        (self.bits & EpStatusStallNak::EP3_OUT_MASK) != 0
    }

    /// Read **EP3_IN** `[6:6]`
    #[inline(always)]
    pub const fn ep3_in(&self) -> bool {
        (self.bits & EpStatusStallNak::EP3_IN_MASK) != 0
    }

    /// Read **EP2_OUT** `[5:5]`
    #[inline(always)]
    pub const fn ep2_out(&self) -> bool {
        (self.bits & EpStatusStallNak::EP2_OUT_MASK) != 0
    }

    /// Read **EP2_IN** `[4:4]`
    #[inline(always)]
    pub const fn ep2_in(&self) -> bool {
        (self.bits & EpStatusStallNak::EP2_IN_MASK) != 0
    }

    /// Read **EP1_OUT** `[3:3]`
    #[inline(always)]
    pub const fn ep1_out(&self) -> bool {
        (self.bits & EpStatusStallNak::EP1_OUT_MASK) != 0
    }

    /// Read **EP1_IN** `[2:2]`
    #[inline(always)]
    pub const fn ep1_in(&self) -> bool {
        (self.bits & EpStatusStallNak::EP1_IN_MASK) != 0
    }

    /// Read **EP0_OUT** `[1:1]`
    #[inline(always)]
    pub const fn ep0_out(&self) -> bool {
        (self.bits & EpStatusStallNak::EP0_OUT_MASK) != 0
    }

    /// Read **EP0_IN** `[0:0]`
    #[inline(always)]
    pub const fn ep0_in(&self) -> bool {
        (self.bits & EpStatusStallNak::EP0_IN_MASK) != 0
    }

    /// Set **EP15_OUT** `[31:31]`
    #[inline(always)]
    pub const fn with_ep15_out(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpStatusStallNak::EP15_OUT_MASK
            } else {
                self.bits & !EpStatusStallNak::EP15_OUT_MASK
            },
        }
    }

    /// Set **EP15_IN** `[30:30]`
    #[inline(always)]
    pub const fn with_ep15_in(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpStatusStallNak::EP15_IN_MASK
            } else {
                self.bits & !EpStatusStallNak::EP15_IN_MASK
            },
        }
    }

    /// Set **EP14_OUT** `[29:29]`
    #[inline(always)]
    pub const fn with_ep14_out(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpStatusStallNak::EP14_OUT_MASK
            } else {
                self.bits & !EpStatusStallNak::EP14_OUT_MASK
            },
        }
    }

    /// Set **EP14_IN** `[28:28]`
    #[inline(always)]
    pub const fn with_ep14_in(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpStatusStallNak::EP14_IN_MASK
            } else {
                self.bits & !EpStatusStallNak::EP14_IN_MASK
            },
        }
    }

    /// Set **EP13_OUT** `[27:27]`
    #[inline(always)]
    pub const fn with_ep13_out(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpStatusStallNak::EP13_OUT_MASK
            } else {
                self.bits & !EpStatusStallNak::EP13_OUT_MASK
            },
        }
    }

    /// Set **EP13_IN** `[26:26]`
    #[inline(always)]
    pub const fn with_ep13_in(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpStatusStallNak::EP13_IN_MASK
            } else {
                self.bits & !EpStatusStallNak::EP13_IN_MASK
            },
        }
    }

    /// Set **EP12_OUT** `[25:25]`
    #[inline(always)]
    pub const fn with_ep12_out(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpStatusStallNak::EP12_OUT_MASK
            } else {
                self.bits & !EpStatusStallNak::EP12_OUT_MASK
            },
        }
    }

    /// Set **EP12_IN** `[24:24]`
    #[inline(always)]
    pub const fn with_ep12_in(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpStatusStallNak::EP12_IN_MASK
            } else {
                self.bits & !EpStatusStallNak::EP12_IN_MASK
            },
        }
    }

    /// Set **EP11_OUT** `[23:23]`
    #[inline(always)]
    pub const fn with_ep11_out(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpStatusStallNak::EP11_OUT_MASK
            } else {
                self.bits & !EpStatusStallNak::EP11_OUT_MASK
            },
        }
    }

    /// Set **EP11_IN** `[22:22]`
    #[inline(always)]
    pub const fn with_ep11_in(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpStatusStallNak::EP11_IN_MASK
            } else {
                self.bits & !EpStatusStallNak::EP11_IN_MASK
            },
        }
    }

    /// Set **EP10_OUT** `[21:21]`
    #[inline(always)]
    pub const fn with_ep10_out(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpStatusStallNak::EP10_OUT_MASK
            } else {
                self.bits & !EpStatusStallNak::EP10_OUT_MASK
            },
        }
    }

    /// Set **EP10_IN** `[20:20]`
    #[inline(always)]
    pub const fn with_ep10_in(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpStatusStallNak::EP10_IN_MASK
            } else {
                self.bits & !EpStatusStallNak::EP10_IN_MASK
            },
        }
    }

    /// Set **EP9_OUT** `[19:19]`
    #[inline(always)]
    pub const fn with_ep9_out(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpStatusStallNak::EP9_OUT_MASK
            } else {
                self.bits & !EpStatusStallNak::EP9_OUT_MASK
            },
        }
    }

    /// Set **EP9_IN** `[18:18]`
    #[inline(always)]
    pub const fn with_ep9_in(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpStatusStallNak::EP9_IN_MASK
            } else {
                self.bits & !EpStatusStallNak::EP9_IN_MASK
            },
        }
    }

    /// Set **EP8_OUT** `[17:17]`
    #[inline(always)]
    pub const fn with_ep8_out(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpStatusStallNak::EP8_OUT_MASK
            } else {
                self.bits & !EpStatusStallNak::EP8_OUT_MASK
            },
        }
    }

    /// Set **EP8_IN** `[16:16]`
    #[inline(always)]
    pub const fn with_ep8_in(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpStatusStallNak::EP8_IN_MASK
            } else {
                self.bits & !EpStatusStallNak::EP8_IN_MASK
            },
        }
    }

    /// Set **EP7_OUT** `[15:15]`
    #[inline(always)]
    pub const fn with_ep7_out(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpStatusStallNak::EP7_OUT_MASK
            } else {
                self.bits & !EpStatusStallNak::EP7_OUT_MASK
            },
        }
    }

    /// Set **EP7_IN** `[14:14]`
    #[inline(always)]
    pub const fn with_ep7_in(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpStatusStallNak::EP7_IN_MASK
            } else {
                self.bits & !EpStatusStallNak::EP7_IN_MASK
            },
        }
    }

    /// Set **EP6_OUT** `[13:13]`
    #[inline(always)]
    pub const fn with_ep6_out(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpStatusStallNak::EP6_OUT_MASK
            } else {
                self.bits & !EpStatusStallNak::EP6_OUT_MASK
            },
        }
    }

    /// Set **EP6_IN** `[12:12]`
    #[inline(always)]
    pub const fn with_ep6_in(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpStatusStallNak::EP6_IN_MASK
            } else {
                self.bits & !EpStatusStallNak::EP6_IN_MASK
            },
        }
    }

    /// Set **EP5_OUT** `[11:11]`
    #[inline(always)]
    pub const fn with_ep5_out(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpStatusStallNak::EP5_OUT_MASK
            } else {
                self.bits & !EpStatusStallNak::EP5_OUT_MASK
            },
        }
    }

    /// Set **EP5_IN** `[10:10]`
    #[inline(always)]
    pub const fn with_ep5_in(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpStatusStallNak::EP5_IN_MASK
            } else {
                self.bits & !EpStatusStallNak::EP5_IN_MASK
            },
        }
    }

    /// Set **EP4_OUT** `[9:9]`
    #[inline(always)]
    pub const fn with_ep4_out(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpStatusStallNak::EP4_OUT_MASK
            } else {
                self.bits & !EpStatusStallNak::EP4_OUT_MASK
            },
        }
    }

    /// Set **EP4_IN** `[8:8]`
    #[inline(always)]
    pub const fn with_ep4_in(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpStatusStallNak::EP4_IN_MASK
            } else {
                self.bits & !EpStatusStallNak::EP4_IN_MASK
            },
        }
    }

    /// Set **EP3_OUT** `[7:7]`
    #[inline(always)]
    pub const fn with_ep3_out(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpStatusStallNak::EP3_OUT_MASK
            } else {
                self.bits & !EpStatusStallNak::EP3_OUT_MASK
            },
        }
    }

    /// Set **EP3_IN** `[6:6]`
    #[inline(always)]
    pub const fn with_ep3_in(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpStatusStallNak::EP3_IN_MASK
            } else {
                self.bits & !EpStatusStallNak::EP3_IN_MASK
            },
        }
    }

    /// Set **EP2_OUT** `[5:5]`
    #[inline(always)]
    pub const fn with_ep2_out(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpStatusStallNak::EP2_OUT_MASK
            } else {
                self.bits & !EpStatusStallNak::EP2_OUT_MASK
            },
        }
    }

    /// Set **EP2_IN** `[4:4]`
    #[inline(always)]
    pub const fn with_ep2_in(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpStatusStallNak::EP2_IN_MASK
            } else {
                self.bits & !EpStatusStallNak::EP2_IN_MASK
            },
        }
    }

    /// Set **EP1_OUT** `[3:3]`
    #[inline(always)]
    pub const fn with_ep1_out(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpStatusStallNak::EP1_OUT_MASK
            } else {
                self.bits & !EpStatusStallNak::EP1_OUT_MASK
            },
        }
    }

    /// Set **EP1_IN** `[2:2]`
    #[inline(always)]
    pub const fn with_ep1_in(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpStatusStallNak::EP1_IN_MASK
            } else {
                self.bits & !EpStatusStallNak::EP1_IN_MASK
            },
        }
    }

    /// Set **EP0_OUT** `[1:1]`
    #[inline(always)]
    pub const fn with_ep0_out(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpStatusStallNak::EP0_OUT_MASK
            } else {
                self.bits & !EpStatusStallNak::EP0_OUT_MASK
            },
        }
    }

    /// Set **EP0_IN** `[0:0]`
    #[inline(always)]
    pub const fn with_ep0_in(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpStatusStallNak::EP0_IN_MASK
            } else {
                self.bits & !EpStatusStallNak::EP0_IN_MASK
            },
        }
    }
}

impl Default for EpStatusStallNakVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl EpStatusStallNak {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EpStatusStallNakVal {
        EpStatusStallNakVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EpStatusStallNakVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EpStatusStallNakVal) -> EpStatusStallNakVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// USB_MUXING Register
///
/// Where to connect the USB controller. Should be to_phy by default.
///
/// Address offset: 0x0074
/// Reset value: 0x00000001
///
/// ## Fields
///
/// - **SWAP_DPDM** `[31]` (RW)
///
///   Swap the USB PHY DP and DM pins and all related controls and flip receive differential data. Can be used to switch USB DP/DP on the PCB.
///   This is done at a low level so overrides all other controls.
///
/// - **USBPHY_AS_GPIO** `[4]` (RW)
///
///   Use the usb DP and DM pins as GPIO pins instead of connecting them to the USB controller.
///
/// - **SOFTCON** `[3]` (RW)
///
/// - **TO_DIGITAL_PAD** `[2]` (RW)
///
/// - **TO_EXTPHY** `[1]` (RW)
///
/// - **TO_PHY** `[0]` (RW)
#[repr(transparent)]
pub struct UsbMuxing {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for UsbMuxing {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl UsbMuxing {
    // ======== Field Constants ========

    /// **SWAP_DPDM**
    ///
    /// Swap the USB PHY DP and DM pins and all related controls and flip receive differential data. Can be used to switch USB DP/DP on the PCB.
    /// This is done at a low level so overrides all other controls.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const SWAP_DPDM_OFFSET: u32 = 31;
    pub const SWAP_DPDM_MASK: u32 = 0x80000000;
    pub const SWAP_DPDM: u32 = 1 << 31;

    /// **USBPHY_AS_GPIO**
    ///
    /// Use the usb DP and DM pins as GPIO pins instead of connecting them to the USB controller.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const USBPHY_AS_GPIO_OFFSET: u32 = 4;
    pub const USBPHY_AS_GPIO_MASK: u32 = 0x00000010;
    pub const USBPHY_AS_GPIO: u32 = 1 << 4;

    /// **SOFTCON**
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SOFTCON_OFFSET: u32 = 3;
    pub const SOFTCON_MASK: u32 = 0x00000008;
    pub const SOFTCON: u32 = 1 << 3;

    /// **TO_DIGITAL_PAD**
    /// Bits `[2:2]`, mask `0x00000004`
    pub const TO_DIGITAL_PAD_OFFSET: u32 = 2;
    pub const TO_DIGITAL_PAD_MASK: u32 = 0x00000004;
    pub const TO_DIGITAL_PAD: u32 = 1 << 2;

    /// **TO_EXTPHY**
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TO_EXTPHY_OFFSET: u32 = 1;
    pub const TO_EXTPHY_MASK: u32 = 0x00000002;
    pub const TO_EXTPHY: u32 = 1 << 1;

    /// **TO_PHY**
    /// Bits `[0:0]`, mask `0x00000001`
    pub const TO_PHY_OFFSET: u32 = 0;
    pub const TO_PHY_MASK: u32 = 0x00000001;
    pub const TO_PHY: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **SWAP_DPDM** `[31]` (RW)
    ///
    /// Swap the USB PHY DP and DM pins and all related controls and flip receive differential data. Can be used to switch USB DP/DP on the PCB.
    /// This is done at a low level so overrides all other controls.
    #[inline(always)]
    pub fn swap_dpdm(&self) -> bool {
        (self.value.read_bits() & Self::SWAP_DPDM_MASK) != 0
    }

    /// Set SWAP_DPDM bit atomically (RW)
    #[inline(always)]
    pub fn set_swap_dpdm(&self) {
        self.set_bits(Self::SWAP_DPDM_MASK);
    }

    /// Clear SWAP_DPDM bit atomically (RW)
    #[inline(always)]
    pub fn clr_swap_dpdm(&self) {
        self.clr_bits(Self::SWAP_DPDM_MASK);
    }

    /// **USBPHY_AS_GPIO** `[4]` (RW)
    ///
    /// Use the usb DP and DM pins as GPIO pins instead of connecting them to the USB controller.
    #[inline(always)]
    pub fn usbphy_as_gpio(&self) -> bool {
        (self.value.read_bits() & Self::USBPHY_AS_GPIO_MASK) != 0
    }

    /// Set USBPHY_AS_GPIO bit atomically (RW)
    #[inline(always)]
    pub fn set_usbphy_as_gpio(&self) {
        self.set_bits(Self::USBPHY_AS_GPIO_MASK);
    }

    /// Clear USBPHY_AS_GPIO bit atomically (RW)
    #[inline(always)]
    pub fn clr_usbphy_as_gpio(&self) {
        self.clr_bits(Self::USBPHY_AS_GPIO_MASK);
    }

    /// **SOFTCON** `[3]` (RW)
    #[inline(always)]
    pub fn softcon(&self) -> bool {
        (self.value.read_bits() & Self::SOFTCON_MASK) != 0
    }

    /// Set SOFTCON bit atomically (RW)
    #[inline(always)]
    pub fn set_softcon(&self) {
        self.set_bits(Self::SOFTCON_MASK);
    }

    /// Clear SOFTCON bit atomically (RW)
    #[inline(always)]
    pub fn clr_softcon(&self) {
        self.clr_bits(Self::SOFTCON_MASK);
    }

    /// **TO_DIGITAL_PAD** `[2]` (RW)
    #[inline(always)]
    pub fn to_digital_pad(&self) -> bool {
        (self.value.read_bits() & Self::TO_DIGITAL_PAD_MASK) != 0
    }

    /// Set TO_DIGITAL_PAD bit atomically (RW)
    #[inline(always)]
    pub fn set_to_digital_pad(&self) {
        self.set_bits(Self::TO_DIGITAL_PAD_MASK);
    }

    /// Clear TO_DIGITAL_PAD bit atomically (RW)
    #[inline(always)]
    pub fn clr_to_digital_pad(&self) {
        self.clr_bits(Self::TO_DIGITAL_PAD_MASK);
    }

    /// **TO_EXTPHY** `[1]` (RW)
    #[inline(always)]
    pub fn to_extphy(&self) -> bool {
        (self.value.read_bits() & Self::TO_EXTPHY_MASK) != 0
    }

    /// Set TO_EXTPHY bit atomically (RW)
    #[inline(always)]
    pub fn set_to_extphy(&self) {
        self.set_bits(Self::TO_EXTPHY_MASK);
    }

    /// Clear TO_EXTPHY bit atomically (RW)
    #[inline(always)]
    pub fn clr_to_extphy(&self) {
        self.clr_bits(Self::TO_EXTPHY_MASK);
    }

    /// **TO_PHY** `[0]` (RW)
    #[inline(always)]
    pub fn to_phy(&self) -> bool {
        (self.value.read_bits() & Self::TO_PHY_MASK) != 0
    }

    /// Set TO_PHY bit atomically (RW)
    #[inline(always)]
    pub fn set_to_phy(&self) {
        self.set_bits(Self::TO_PHY_MASK);
    }

    /// Clear TO_PHY bit atomically (RW)
    #[inline(always)]
    pub fn clr_to_phy(&self) {
        self.clr_bits(Self::TO_PHY_MASK);
    }
}

/// USB_MUXING value type for batch read/modify/write operations
///
/// Use `usbmuxing.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `usbmuxing.write_val(val)`.
#[derive(Clone, Copy)]
pub struct UsbMuxingVal {
    bits: u32,
}

impl UsbMuxingVal {
    /// Create from raw bits
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

    /// Read **SWAP_DPDM** `[31:31]`
    ///
    /// Swap the USB PHY DP and DM pins and all related controls and flip receive differential data. Can be used to switch USB DP/DP on the PCB.
    /// This is done at a low level so overrides all other controls.
    #[inline(always)]
    pub const fn swap_dpdm(&self) -> bool {
        (self.bits & UsbMuxing::SWAP_DPDM_MASK) != 0
    }

    /// Read **USBPHY_AS_GPIO** `[4:4]`
    ///
    /// Use the usb DP and DM pins as GPIO pins instead of connecting them to the USB controller.
    #[inline(always)]
    pub const fn usbphy_as_gpio(&self) -> bool {
        (self.bits & UsbMuxing::USBPHY_AS_GPIO_MASK) != 0
    }

    /// Read **SOFTCON** `[3:3]`
    #[inline(always)]
    pub const fn softcon(&self) -> bool {
        (self.bits & UsbMuxing::SOFTCON_MASK) != 0
    }

    /// Read **TO_DIGITAL_PAD** `[2:2]`
    #[inline(always)]
    pub const fn to_digital_pad(&self) -> bool {
        (self.bits & UsbMuxing::TO_DIGITAL_PAD_MASK) != 0
    }

    /// Read **TO_EXTPHY** `[1:1]`
    #[inline(always)]
    pub const fn to_extphy(&self) -> bool {
        (self.bits & UsbMuxing::TO_EXTPHY_MASK) != 0
    }

    /// Read **TO_PHY** `[0:0]`
    #[inline(always)]
    pub const fn to_phy(&self) -> bool {
        (self.bits & UsbMuxing::TO_PHY_MASK) != 0
    }

    /// Set **SWAP_DPDM** `[31:31]`
    ///
    /// Swap the USB PHY DP and DM pins and all related controls and flip receive differential data. Can be used to switch USB DP/DP on the PCB.
    /// This is done at a low level so overrides all other controls.
    #[inline(always)]
    pub const fn with_swap_dpdm(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | UsbMuxing::SWAP_DPDM_MASK
            } else {
                self.bits & !UsbMuxing::SWAP_DPDM_MASK
            },
        }
    }

    /// Set **USBPHY_AS_GPIO** `[4:4]`
    ///
    /// Use the usb DP and DM pins as GPIO pins instead of connecting them to the USB controller.
    #[inline(always)]
    pub const fn with_usbphy_as_gpio(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | UsbMuxing::USBPHY_AS_GPIO_MASK
            } else {
                self.bits & !UsbMuxing::USBPHY_AS_GPIO_MASK
            },
        }
    }

    /// Set **SOFTCON** `[3:3]`
    #[inline(always)]
    pub const fn with_softcon(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | UsbMuxing::SOFTCON_MASK
            } else {
                self.bits & !UsbMuxing::SOFTCON_MASK
            },
        }
    }

    /// Set **TO_DIGITAL_PAD** `[2:2]`
    #[inline(always)]
    pub const fn with_to_digital_pad(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | UsbMuxing::TO_DIGITAL_PAD_MASK
            } else {
                self.bits & !UsbMuxing::TO_DIGITAL_PAD_MASK
            },
        }
    }

    /// Set **TO_EXTPHY** `[1:1]`
    #[inline(always)]
    pub const fn with_to_extphy(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | UsbMuxing::TO_EXTPHY_MASK
            } else {
                self.bits & !UsbMuxing::TO_EXTPHY_MASK
            },
        }
    }

    /// Set **TO_PHY** `[0:0]`
    #[inline(always)]
    pub const fn with_to_phy(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | UsbMuxing::TO_PHY_MASK
            } else {
                self.bits & !UsbMuxing::TO_PHY_MASK
            },
        }
    }
}

impl Default for UsbMuxingVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl UsbMuxing {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> UsbMuxingVal {
        UsbMuxingVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: UsbMuxingVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(UsbMuxingVal) -> UsbMuxingVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// USB_PWR Register
///
/// Overrides for the power signals in the event that the VBUS signals are not hooked up to GPIO. Set the value of the override and then the override enable to switch over to the override value.
///
/// Address offset: 0x0078
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **OVERCURR_DETECT_EN** `[5]` (RW)
///
/// - **OVERCURR_DETECT** `[4]` (RW)
///
/// - **VBUS_DETECT_OVERRIDE_EN** `[3]` (RW)
///
/// - **VBUS_DETECT** `[2]` (RW)
///
/// - **VBUS_EN_OVERRIDE_EN** `[1]` (RW)
///
/// - **VBUS_EN** `[0]` (RW)
#[repr(transparent)]
pub struct UsbPwr {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for UsbPwr {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl UsbPwr {
    // ======== Field Constants ========

    /// **OVERCURR_DETECT_EN**
    /// Bits `[5:5]`, mask `0x00000020`
    pub const OVERCURR_DETECT_EN_OFFSET: u32 = 5;
    pub const OVERCURR_DETECT_EN_MASK: u32 = 0x00000020;
    pub const OVERCURR_DETECT_EN: u32 = 1 << 5;

    /// **OVERCURR_DETECT**
    /// Bits `[4:4]`, mask `0x00000010`
    pub const OVERCURR_DETECT_OFFSET: u32 = 4;
    pub const OVERCURR_DETECT_MASK: u32 = 0x00000010;
    pub const OVERCURR_DETECT: u32 = 1 << 4;

    /// **VBUS_DETECT_OVERRIDE_EN**
    /// Bits `[3:3]`, mask `0x00000008`
    pub const VBUS_DETECT_OVERRIDE_EN_OFFSET: u32 = 3;
    pub const VBUS_DETECT_OVERRIDE_EN_MASK: u32 = 0x00000008;
    pub const VBUS_DETECT_OVERRIDE_EN: u32 = 1 << 3;

    /// **VBUS_DETECT**
    /// Bits `[2:2]`, mask `0x00000004`
    pub const VBUS_DETECT_OFFSET: u32 = 2;
    pub const VBUS_DETECT_MASK: u32 = 0x00000004;
    pub const VBUS_DETECT: u32 = 1 << 2;

    /// **VBUS_EN_OVERRIDE_EN**
    /// Bits `[1:1]`, mask `0x00000002`
    pub const VBUS_EN_OVERRIDE_EN_OFFSET: u32 = 1;
    pub const VBUS_EN_OVERRIDE_EN_MASK: u32 = 0x00000002;
    pub const VBUS_EN_OVERRIDE_EN: u32 = 1 << 1;

    /// **VBUS_EN**
    /// Bits `[0:0]`, mask `0x00000001`
    pub const VBUS_EN_OFFSET: u32 = 0;
    pub const VBUS_EN_MASK: u32 = 0x00000001;
    pub const VBUS_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **OVERCURR_DETECT_EN** `[5]` (RW)
    #[inline(always)]
    pub fn overcurr_detect_en(&self) -> bool {
        (self.value.read_bits() & Self::OVERCURR_DETECT_EN_MASK) != 0
    }

    /// Set OVERCURR_DETECT_EN bit atomically (RW)
    #[inline(always)]
    pub fn set_overcurr_detect_en(&self) {
        self.set_bits(Self::OVERCURR_DETECT_EN_MASK);
    }

    /// Clear OVERCURR_DETECT_EN bit atomically (RW)
    #[inline(always)]
    pub fn clr_overcurr_detect_en(&self) {
        self.clr_bits(Self::OVERCURR_DETECT_EN_MASK);
    }

    /// **OVERCURR_DETECT** `[4]` (RW)
    #[inline(always)]
    pub fn overcurr_detect(&self) -> bool {
        (self.value.read_bits() & Self::OVERCURR_DETECT_MASK) != 0
    }

    /// Set OVERCURR_DETECT bit atomically (RW)
    #[inline(always)]
    pub fn set_overcurr_detect(&self) {
        self.set_bits(Self::OVERCURR_DETECT_MASK);
    }

    /// Clear OVERCURR_DETECT bit atomically (RW)
    #[inline(always)]
    pub fn clr_overcurr_detect(&self) {
        self.clr_bits(Self::OVERCURR_DETECT_MASK);
    }

    /// **VBUS_DETECT_OVERRIDE_EN** `[3]` (RW)
    #[inline(always)]
    pub fn vbus_detect_override_en(&self) -> bool {
        (self.value.read_bits() & Self::VBUS_DETECT_OVERRIDE_EN_MASK) != 0
    }

    /// Set VBUS_DETECT_OVERRIDE_EN bit atomically (RW)
    #[inline(always)]
    pub fn set_vbus_detect_override_en(&self) {
        self.set_bits(Self::VBUS_DETECT_OVERRIDE_EN_MASK);
    }

    /// Clear VBUS_DETECT_OVERRIDE_EN bit atomically (RW)
    #[inline(always)]
    pub fn clr_vbus_detect_override_en(&self) {
        self.clr_bits(Self::VBUS_DETECT_OVERRIDE_EN_MASK);
    }

    /// **VBUS_DETECT** `[2]` (RW)
    #[inline(always)]
    pub fn vbus_detect(&self) -> bool {
        (self.value.read_bits() & Self::VBUS_DETECT_MASK) != 0
    }

    /// Set VBUS_DETECT bit atomically (RW)
    #[inline(always)]
    pub fn set_vbus_detect(&self) {
        self.set_bits(Self::VBUS_DETECT_MASK);
    }

    /// Clear VBUS_DETECT bit atomically (RW)
    #[inline(always)]
    pub fn clr_vbus_detect(&self) {
        self.clr_bits(Self::VBUS_DETECT_MASK);
    }

    /// **VBUS_EN_OVERRIDE_EN** `[1]` (RW)
    #[inline(always)]
    pub fn vbus_en_override_en(&self) -> bool {
        (self.value.read_bits() & Self::VBUS_EN_OVERRIDE_EN_MASK) != 0
    }

    /// Set VBUS_EN_OVERRIDE_EN bit atomically (RW)
    #[inline(always)]
    pub fn set_vbus_en_override_en(&self) {
        self.set_bits(Self::VBUS_EN_OVERRIDE_EN_MASK);
    }

    /// Clear VBUS_EN_OVERRIDE_EN bit atomically (RW)
    #[inline(always)]
    pub fn clr_vbus_en_override_en(&self) {
        self.clr_bits(Self::VBUS_EN_OVERRIDE_EN_MASK);
    }

    /// **VBUS_EN** `[0]` (RW)
    #[inline(always)]
    pub fn vbus_en(&self) -> bool {
        (self.value.read_bits() & Self::VBUS_EN_MASK) != 0
    }

    /// Set VBUS_EN bit atomically (RW)
    #[inline(always)]
    pub fn set_vbus_en(&self) {
        self.set_bits(Self::VBUS_EN_MASK);
    }

    /// Clear VBUS_EN bit atomically (RW)
    #[inline(always)]
    pub fn clr_vbus_en(&self) {
        self.clr_bits(Self::VBUS_EN_MASK);
    }
}

/// USB_PWR value type for batch read/modify/write operations
///
/// Use `usbpwr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `usbpwr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct UsbPwrVal {
    bits: u32,
}

impl UsbPwrVal {
    /// Create from raw bits
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

    /// Read **OVERCURR_DETECT_EN** `[5:5]`
    #[inline(always)]
    pub const fn overcurr_detect_en(&self) -> bool {
        (self.bits & UsbPwr::OVERCURR_DETECT_EN_MASK) != 0
    }

    /// Read **OVERCURR_DETECT** `[4:4]`
    #[inline(always)]
    pub const fn overcurr_detect(&self) -> bool {
        (self.bits & UsbPwr::OVERCURR_DETECT_MASK) != 0
    }

    /// Read **VBUS_DETECT_OVERRIDE_EN** `[3:3]`
    #[inline(always)]
    pub const fn vbus_detect_override_en(&self) -> bool {
        (self.bits & UsbPwr::VBUS_DETECT_OVERRIDE_EN_MASK) != 0
    }

    /// Read **VBUS_DETECT** `[2:2]`
    #[inline(always)]
    pub const fn vbus_detect(&self) -> bool {
        (self.bits & UsbPwr::VBUS_DETECT_MASK) != 0
    }

    /// Read **VBUS_EN_OVERRIDE_EN** `[1:1]`
    #[inline(always)]
    pub const fn vbus_en_override_en(&self) -> bool {
        (self.bits & UsbPwr::VBUS_EN_OVERRIDE_EN_MASK) != 0
    }

    /// Read **VBUS_EN** `[0:0]`
    #[inline(always)]
    pub const fn vbus_en(&self) -> bool {
        (self.bits & UsbPwr::VBUS_EN_MASK) != 0
    }

    /// Set **OVERCURR_DETECT_EN** `[5:5]`
    #[inline(always)]
    pub const fn with_overcurr_detect_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | UsbPwr::OVERCURR_DETECT_EN_MASK
            } else {
                self.bits & !UsbPwr::OVERCURR_DETECT_EN_MASK
            },
        }
    }

    /// Set **OVERCURR_DETECT** `[4:4]`
    #[inline(always)]
    pub const fn with_overcurr_detect(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | UsbPwr::OVERCURR_DETECT_MASK
            } else {
                self.bits & !UsbPwr::OVERCURR_DETECT_MASK
            },
        }
    }

    /// Set **VBUS_DETECT_OVERRIDE_EN** `[3:3]`
    #[inline(always)]
    pub const fn with_vbus_detect_override_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | UsbPwr::VBUS_DETECT_OVERRIDE_EN_MASK
            } else {
                self.bits & !UsbPwr::VBUS_DETECT_OVERRIDE_EN_MASK
            },
        }
    }

    /// Set **VBUS_DETECT** `[2:2]`
    #[inline(always)]
    pub const fn with_vbus_detect(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | UsbPwr::VBUS_DETECT_MASK
            } else {
                self.bits & !UsbPwr::VBUS_DETECT_MASK
            },
        }
    }

    /// Set **VBUS_EN_OVERRIDE_EN** `[1:1]`
    #[inline(always)]
    pub const fn with_vbus_en_override_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | UsbPwr::VBUS_EN_OVERRIDE_EN_MASK
            } else {
                self.bits & !UsbPwr::VBUS_EN_OVERRIDE_EN_MASK
            },
        }
    }

    /// Set **VBUS_EN** `[0:0]`
    #[inline(always)]
    pub const fn with_vbus_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | UsbPwr::VBUS_EN_MASK
            } else {
                self.bits & !UsbPwr::VBUS_EN_MASK
            },
        }
    }
}

impl Default for UsbPwrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl UsbPwr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> UsbPwrVal {
        UsbPwrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: UsbPwrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(UsbPwrVal) -> UsbPwrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// USBPHY_DIRECT Register
///
/// This register allows for direct control of the USB phy. Use in conjunction with usbphy_direct_override register to enable each override bit.
///
/// Address offset: 0x007c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **RX_DM_OVERRIDE** `[25]` (RW)
///
///   Override rx_dm value into controller
///
/// - **RX_DP_OVERRIDE** `[24]` (RW)
///
///   Override rx_dp value into controller
///
/// - **RX_DD_OVERRIDE** `[23]` (RW)
///
///   Override rx_dd value into controller
///
/// - **DM_OVV** `[22]` (RO)
///
///   DM over voltage
///
/// - **DP_OVV** `[21]` (RO)
///
///   DP over voltage
///
/// - **DM_OVCN** `[20]` (RO)
///
///   DM overcurrent
///
/// - **DP_OVCN** `[19]` (RO)
///
///   DP overcurrent
///
/// - **RX_DM** `[18]` (RO)
///
///   DPM pin state
///
/// - **RX_DP** `[17]` (RO)
///
///   DPP pin state
///
/// - **RX_DD** `[16]` (RO)
///
///   Differential RX
///
/// - **TX_DIFFMODE** `[15]` (RW)
///
///   TX_DIFFMODE=0: Single ended mode
///   TX_DIFFMODE=1: Differential drive mode (TX_DM, TX_DM_OE ignored)
///
/// - **TX_FSSLEW** `[14]` (RW)
///
///   TX_FSSLEW=0: Low speed slew rate
///   TX_FSSLEW=1: Full speed slew rate
///
/// - **TX_PD** `[13]` (RW)
///
///   TX power down override (if override enable is set). 1 = powered down.
///
/// - **RX_PD** `[12]` (RW)
///
///   RX power down override (if override enable is set). 1 = powered down.
///
/// - **TX_DM** `[11]` (RW)
///
///   Output data. TX_DIFFMODE=1, Ignored
///   TX_DIFFMODE=0, Drives DPM only. TX_DM_OE=1 to enable drive. DPM=TX_DM
///
/// - **TX_DP** `[10]` (RW)
///
///   Output data. If TX_DIFFMODE=1, Drives DPP/DPM diff pair. TX_DP_OE=1 to enable drive. DPP=TX_DP, DPM=~TX_DP
///   If TX_DIFFMODE=0, Drives DPP only. TX_DP_OE=1 to enable drive. DPP=TX_DP
///
/// - **TX_DM_OE** `[9]` (RW)
///
///   Output enable. If TX_DIFFMODE=1, Ignored.
///   If TX_DIFFMODE=0, OE for DPM only. 0 - DPM in Hi-Z state; 1 - DPM driving
///
/// - **TX_DP_OE** `[8]` (RW)
///
///   Output enable. If TX_DIFFMODE=1, OE for DPP/DPM diff pair. 0 - DPP/DPM in Hi-Z state; 1 - DPP/DPM driving
///   If TX_DIFFMODE=0, OE for DPP only. 0 - DPP in Hi-Z state; 1 - DPP driving
///
/// - **DM_PULLDN_EN** `[6]` (RW)
///
///   DM pull down enable
///
/// - **DM_PULLUP_EN** `[5]` (RW)
///
///   DM pull up enable
///
/// - **DM_PULLUP_HISEL** `[4]` (RW)
///
///   Enable the second DM pull up resistor. 0 - Pull = Rpu2; 1 - Pull = Rpu1 + Rpu2
///
/// - **DP_PULLDN_EN** `[2]` (RW)
///
///   DP pull down enable
///
/// - **DP_PULLUP_EN** `[1]` (RW)
///
///   DP pull up enable
///
/// - **DP_PULLUP_HISEL** `[0]` (RW)
///
///   Enable the second DP pull up resistor. 0 - Pull = Rpu2; 1 - Pull = Rpu1 + Rpu2
#[repr(transparent)]
pub struct UsbphyDirect {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for UsbphyDirect {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl UsbphyDirect {
    // ======== Field Constants ========

    /// **RX_DM_OVERRIDE**
    ///
    /// Override rx_dm value into controller
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const RX_DM_OVERRIDE_OFFSET: u32 = 25;
    pub const RX_DM_OVERRIDE_MASK: u32 = 0x02000000;
    pub const RX_DM_OVERRIDE: u32 = 1 << 25;

    /// **RX_DP_OVERRIDE**
    ///
    /// Override rx_dp value into controller
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const RX_DP_OVERRIDE_OFFSET: u32 = 24;
    pub const RX_DP_OVERRIDE_MASK: u32 = 0x01000000;
    pub const RX_DP_OVERRIDE: u32 = 1 << 24;

    /// **RX_DD_OVERRIDE**
    ///
    /// Override rx_dd value into controller
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const RX_DD_OVERRIDE_OFFSET: u32 = 23;
    pub const RX_DD_OVERRIDE_MASK: u32 = 0x00800000;
    pub const RX_DD_OVERRIDE: u32 = 1 << 23;

    /// **DM_OVV**
    ///
    /// DM over voltage
    ///
    /// Bits `[22:22]`, mask `0x00400000`
    pub const DM_OVV_OFFSET: u32 = 22;
    pub const DM_OVV_MASK: u32 = 0x00400000;
    pub const DM_OVV: u32 = 1 << 22;

    /// **DP_OVV**
    ///
    /// DP over voltage
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const DP_OVV_OFFSET: u32 = 21;
    pub const DP_OVV_MASK: u32 = 0x00200000;
    pub const DP_OVV: u32 = 1 << 21;

    /// **DM_OVCN**
    ///
    /// DM overcurrent
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const DM_OVCN_OFFSET: u32 = 20;
    pub const DM_OVCN_MASK: u32 = 0x00100000;
    pub const DM_OVCN: u32 = 1 << 20;

    /// **DP_OVCN**
    ///
    /// DP overcurrent
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const DP_OVCN_OFFSET: u32 = 19;
    pub const DP_OVCN_MASK: u32 = 0x00080000;
    pub const DP_OVCN: u32 = 1 << 19;

    /// **RX_DM**
    ///
    /// DPM pin state
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const RX_DM_OFFSET: u32 = 18;
    pub const RX_DM_MASK: u32 = 0x00040000;
    pub const RX_DM: u32 = 1 << 18;

    /// **RX_DP**
    ///
    /// DPP pin state
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const RX_DP_OFFSET: u32 = 17;
    pub const RX_DP_MASK: u32 = 0x00020000;
    pub const RX_DP: u32 = 1 << 17;

    /// **RX_DD**
    ///
    /// Differential RX
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const RX_DD_OFFSET: u32 = 16;
    pub const RX_DD_MASK: u32 = 0x00010000;
    pub const RX_DD: u32 = 1 << 16;

    /// **TX_DIFFMODE**
    ///
    /// TX_DIFFMODE=0: Single ended mode
    /// TX_DIFFMODE=1: Differential drive mode (TX_DM, TX_DM_OE ignored)
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const TX_DIFFMODE_OFFSET: u32 = 15;
    pub const TX_DIFFMODE_MASK: u32 = 0x00008000;
    pub const TX_DIFFMODE: u32 = 1 << 15;

    /// **TX_FSSLEW**
    ///
    /// TX_FSSLEW=0: Low speed slew rate
    /// TX_FSSLEW=1: Full speed slew rate
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const TX_FSSLEW_OFFSET: u32 = 14;
    pub const TX_FSSLEW_MASK: u32 = 0x00004000;
    pub const TX_FSSLEW: u32 = 1 << 14;

    /// **TX_PD**
    ///
    /// TX power down override (if override enable is set). 1 = powered down.
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const TX_PD_OFFSET: u32 = 13;
    pub const TX_PD_MASK: u32 = 0x00002000;
    pub const TX_PD: u32 = 1 << 13;

    /// **RX_PD**
    ///
    /// RX power down override (if override enable is set). 1 = powered down.
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const RX_PD_OFFSET: u32 = 12;
    pub const RX_PD_MASK: u32 = 0x00001000;
    pub const RX_PD: u32 = 1 << 12;

    /// **TX_DM**
    ///
    /// Output data. TX_DIFFMODE=1, Ignored
    /// TX_DIFFMODE=0, Drives DPM only. TX_DM_OE=1 to enable drive. DPM=TX_DM
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const TX_DM_OFFSET: u32 = 11;
    pub const TX_DM_MASK: u32 = 0x00000800;
    pub const TX_DM: u32 = 1 << 11;

    /// **TX_DP**
    ///
    /// Output data. If TX_DIFFMODE=1, Drives DPP/DPM diff pair. TX_DP_OE=1 to enable drive. DPP=TX_DP, DPM=~TX_DP
    /// If TX_DIFFMODE=0, Drives DPP only. TX_DP_OE=1 to enable drive. DPP=TX_DP
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const TX_DP_OFFSET: u32 = 10;
    pub const TX_DP_MASK: u32 = 0x00000400;
    pub const TX_DP: u32 = 1 << 10;

    /// **TX_DM_OE**
    ///
    /// Output enable. If TX_DIFFMODE=1, Ignored.
    /// If TX_DIFFMODE=0, OE for DPM only. 0 - DPM in Hi-Z state; 1 - DPM driving
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const TX_DM_OE_OFFSET: u32 = 9;
    pub const TX_DM_OE_MASK: u32 = 0x00000200;
    pub const TX_DM_OE: u32 = 1 << 9;

    /// **TX_DP_OE**
    ///
    /// Output enable. If TX_DIFFMODE=1, OE for DPP/DPM diff pair. 0 - DPP/DPM in Hi-Z state; 1 - DPP/DPM driving
    /// If TX_DIFFMODE=0, OE for DPP only. 0 - DPP in Hi-Z state; 1 - DPP driving
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const TX_DP_OE_OFFSET: u32 = 8;
    pub const TX_DP_OE_MASK: u32 = 0x00000100;
    pub const TX_DP_OE: u32 = 1 << 8;

    /// **DM_PULLDN_EN**
    ///
    /// DM pull down enable
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const DM_PULLDN_EN_OFFSET: u32 = 6;
    pub const DM_PULLDN_EN_MASK: u32 = 0x00000040;
    pub const DM_PULLDN_EN: u32 = 1 << 6;

    /// **DM_PULLUP_EN**
    ///
    /// DM pull up enable
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const DM_PULLUP_EN_OFFSET: u32 = 5;
    pub const DM_PULLUP_EN_MASK: u32 = 0x00000020;
    pub const DM_PULLUP_EN: u32 = 1 << 5;

    /// **DM_PULLUP_HISEL**
    ///
    /// Enable the second DM pull up resistor. 0 - Pull = Rpu2; 1 - Pull = Rpu1 + Rpu2
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const DM_PULLUP_HISEL_OFFSET: u32 = 4;
    pub const DM_PULLUP_HISEL_MASK: u32 = 0x00000010;
    pub const DM_PULLUP_HISEL: u32 = 1 << 4;

    /// **DP_PULLDN_EN**
    ///
    /// DP pull down enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const DP_PULLDN_EN_OFFSET: u32 = 2;
    pub const DP_PULLDN_EN_MASK: u32 = 0x00000004;
    pub const DP_PULLDN_EN: u32 = 1 << 2;

    /// **DP_PULLUP_EN**
    ///
    /// DP pull up enable
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const DP_PULLUP_EN_OFFSET: u32 = 1;
    pub const DP_PULLUP_EN_MASK: u32 = 0x00000002;
    pub const DP_PULLUP_EN: u32 = 1 << 1;

    /// **DP_PULLUP_HISEL**
    ///
    /// Enable the second DP pull up resistor. 0 - Pull = Rpu2; 1 - Pull = Rpu1 + Rpu2
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const DP_PULLUP_HISEL_OFFSET: u32 = 0;
    pub const DP_PULLUP_HISEL_MASK: u32 = 0x00000001;
    pub const DP_PULLUP_HISEL: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **RX_DM_OVERRIDE** `[25]` (RW)
    ///
    /// Override rx_dm value into controller
    #[inline(always)]
    pub fn rx_dm_override(&self) -> bool {
        (self.value.read_bits() & Self::RX_DM_OVERRIDE_MASK) != 0
    }

    /// Set RX_DM_OVERRIDE bit atomically (RW)
    #[inline(always)]
    pub fn set_rx_dm_override(&self) {
        self.set_bits(Self::RX_DM_OVERRIDE_MASK);
    }

    /// Clear RX_DM_OVERRIDE bit atomically (RW)
    #[inline(always)]
    pub fn clr_rx_dm_override(&self) {
        self.clr_bits(Self::RX_DM_OVERRIDE_MASK);
    }

    /// **RX_DP_OVERRIDE** `[24]` (RW)
    ///
    /// Override rx_dp value into controller
    #[inline(always)]
    pub fn rx_dp_override(&self) -> bool {
        (self.value.read_bits() & Self::RX_DP_OVERRIDE_MASK) != 0
    }

    /// Set RX_DP_OVERRIDE bit atomically (RW)
    #[inline(always)]
    pub fn set_rx_dp_override(&self) {
        self.set_bits(Self::RX_DP_OVERRIDE_MASK);
    }

    /// Clear RX_DP_OVERRIDE bit atomically (RW)
    #[inline(always)]
    pub fn clr_rx_dp_override(&self) {
        self.clr_bits(Self::RX_DP_OVERRIDE_MASK);
    }

    /// **RX_DD_OVERRIDE** `[23]` (RW)
    ///
    /// Override rx_dd value into controller
    #[inline(always)]
    pub fn rx_dd_override(&self) -> bool {
        (self.value.read_bits() & Self::RX_DD_OVERRIDE_MASK) != 0
    }

    /// Set RX_DD_OVERRIDE bit atomically (RW)
    #[inline(always)]
    pub fn set_rx_dd_override(&self) {
        self.set_bits(Self::RX_DD_OVERRIDE_MASK);
    }

    /// Clear RX_DD_OVERRIDE bit atomically (RW)
    #[inline(always)]
    pub fn clr_rx_dd_override(&self) {
        self.clr_bits(Self::RX_DD_OVERRIDE_MASK);
    }

    /// **DM_OVV** `[22]` (RO)
    ///
    /// DM over voltage
    #[inline(always)]
    pub fn dm_ovv(&self) -> bool {
        (self.value.read_bits() & Self::DM_OVV_MASK) != 0
    }

    /// **DP_OVV** `[21]` (RO)
    ///
    /// DP over voltage
    #[inline(always)]
    pub fn dp_ovv(&self) -> bool {
        (self.value.read_bits() & Self::DP_OVV_MASK) != 0
    }

    /// **DM_OVCN** `[20]` (RO)
    ///
    /// DM overcurrent
    #[inline(always)]
    pub fn dm_ovcn(&self) -> bool {
        (self.value.read_bits() & Self::DM_OVCN_MASK) != 0
    }

    /// **DP_OVCN** `[19]` (RO)
    ///
    /// DP overcurrent
    #[inline(always)]
    pub fn dp_ovcn(&self) -> bool {
        (self.value.read_bits() & Self::DP_OVCN_MASK) != 0
    }

    /// **RX_DM** `[18]` (RO)
    ///
    /// DPM pin state
    #[inline(always)]
    pub fn rx_dm(&self) -> bool {
        (self.value.read_bits() & Self::RX_DM_MASK) != 0
    }

    /// **RX_DP** `[17]` (RO)
    ///
    /// DPP pin state
    #[inline(always)]
    pub fn rx_dp(&self) -> bool {
        (self.value.read_bits() & Self::RX_DP_MASK) != 0
    }

    /// **RX_DD** `[16]` (RO)
    ///
    /// Differential RX
    #[inline(always)]
    pub fn rx_dd(&self) -> bool {
        (self.value.read_bits() & Self::RX_DD_MASK) != 0
    }

    /// **TX_DIFFMODE** `[15]` (RW)
    ///
    /// TX_DIFFMODE=0: Single ended mode
    /// TX_DIFFMODE=1: Differential drive mode (TX_DM, TX_DM_OE ignored)
    #[inline(always)]
    pub fn tx_diffmode(&self) -> bool {
        (self.value.read_bits() & Self::TX_DIFFMODE_MASK) != 0
    }

    /// Set TX_DIFFMODE bit atomically (RW)
    #[inline(always)]
    pub fn set_tx_diffmode(&self) {
        self.set_bits(Self::TX_DIFFMODE_MASK);
    }

    /// Clear TX_DIFFMODE bit atomically (RW)
    #[inline(always)]
    pub fn clr_tx_diffmode(&self) {
        self.clr_bits(Self::TX_DIFFMODE_MASK);
    }

    /// **TX_FSSLEW** `[14]` (RW)
    ///
    /// TX_FSSLEW=0: Low speed slew rate
    /// TX_FSSLEW=1: Full speed slew rate
    #[inline(always)]
    pub fn tx_fsslew(&self) -> bool {
        (self.value.read_bits() & Self::TX_FSSLEW_MASK) != 0
    }

    /// Set TX_FSSLEW bit atomically (RW)
    #[inline(always)]
    pub fn set_tx_fsslew(&self) {
        self.set_bits(Self::TX_FSSLEW_MASK);
    }

    /// Clear TX_FSSLEW bit atomically (RW)
    #[inline(always)]
    pub fn clr_tx_fsslew(&self) {
        self.clr_bits(Self::TX_FSSLEW_MASK);
    }

    /// **TX_PD** `[13]` (RW)
    ///
    /// TX power down override (if override enable is set). 1 = powered down.
    #[inline(always)]
    pub fn tx_pd(&self) -> bool {
        (self.value.read_bits() & Self::TX_PD_MASK) != 0
    }

    /// Set TX_PD bit atomically (RW)
    #[inline(always)]
    pub fn set_tx_pd(&self) {
        self.set_bits(Self::TX_PD_MASK);
    }

    /// Clear TX_PD bit atomically (RW)
    #[inline(always)]
    pub fn clr_tx_pd(&self) {
        self.clr_bits(Self::TX_PD_MASK);
    }

    /// **RX_PD** `[12]` (RW)
    ///
    /// RX power down override (if override enable is set). 1 = powered down.
    #[inline(always)]
    pub fn rx_pd(&self) -> bool {
        (self.value.read_bits() & Self::RX_PD_MASK) != 0
    }

    /// Set RX_PD bit atomically (RW)
    #[inline(always)]
    pub fn set_rx_pd(&self) {
        self.set_bits(Self::RX_PD_MASK);
    }

    /// Clear RX_PD bit atomically (RW)
    #[inline(always)]
    pub fn clr_rx_pd(&self) {
        self.clr_bits(Self::RX_PD_MASK);
    }

    /// **TX_DM** `[11]` (RW)
    ///
    /// Output data. TX_DIFFMODE=1, Ignored
    /// TX_DIFFMODE=0, Drives DPM only. TX_DM_OE=1 to enable drive. DPM=TX_DM
    #[inline(always)]
    pub fn tx_dm(&self) -> bool {
        (self.value.read_bits() & Self::TX_DM_MASK) != 0
    }

    /// Set TX_DM bit atomically (RW)
    #[inline(always)]
    pub fn set_tx_dm(&self) {
        self.set_bits(Self::TX_DM_MASK);
    }

    /// Clear TX_DM bit atomically (RW)
    #[inline(always)]
    pub fn clr_tx_dm(&self) {
        self.clr_bits(Self::TX_DM_MASK);
    }

    /// **TX_DP** `[10]` (RW)
    ///
    /// Output data. If TX_DIFFMODE=1, Drives DPP/DPM diff pair. TX_DP_OE=1 to enable drive. DPP=TX_DP, DPM=~TX_DP
    /// If TX_DIFFMODE=0, Drives DPP only. TX_DP_OE=1 to enable drive. DPP=TX_DP
    #[inline(always)]
    pub fn tx_dp(&self) -> bool {
        (self.value.read_bits() & Self::TX_DP_MASK) != 0
    }

    /// Set TX_DP bit atomically (RW)
    #[inline(always)]
    pub fn set_tx_dp(&self) {
        self.set_bits(Self::TX_DP_MASK);
    }

    /// Clear TX_DP bit atomically (RW)
    #[inline(always)]
    pub fn clr_tx_dp(&self) {
        self.clr_bits(Self::TX_DP_MASK);
    }

    /// **TX_DM_OE** `[9]` (RW)
    ///
    /// Output enable. If TX_DIFFMODE=1, Ignored.
    /// If TX_DIFFMODE=0, OE for DPM only. 0 - DPM in Hi-Z state; 1 - DPM driving
    #[inline(always)]
    pub fn tx_dm_oe(&self) -> bool {
        (self.value.read_bits() & Self::TX_DM_OE_MASK) != 0
    }

    /// Set TX_DM_OE bit atomically (RW)
    #[inline(always)]
    pub fn set_tx_dm_oe(&self) {
        self.set_bits(Self::TX_DM_OE_MASK);
    }

    /// Clear TX_DM_OE bit atomically (RW)
    #[inline(always)]
    pub fn clr_tx_dm_oe(&self) {
        self.clr_bits(Self::TX_DM_OE_MASK);
    }

    /// **TX_DP_OE** `[8]` (RW)
    ///
    /// Output enable. If TX_DIFFMODE=1, OE for DPP/DPM diff pair. 0 - DPP/DPM in Hi-Z state; 1 - DPP/DPM driving
    /// If TX_DIFFMODE=0, OE for DPP only. 0 - DPP in Hi-Z state; 1 - DPP driving
    #[inline(always)]
    pub fn tx_dp_oe(&self) -> bool {
        (self.value.read_bits() & Self::TX_DP_OE_MASK) != 0
    }

    /// Set TX_DP_OE bit atomically (RW)
    #[inline(always)]
    pub fn set_tx_dp_oe(&self) {
        self.set_bits(Self::TX_DP_OE_MASK);
    }

    /// Clear TX_DP_OE bit atomically (RW)
    #[inline(always)]
    pub fn clr_tx_dp_oe(&self) {
        self.clr_bits(Self::TX_DP_OE_MASK);
    }

    /// **DM_PULLDN_EN** `[6]` (RW)
    ///
    /// DM pull down enable
    #[inline(always)]
    pub fn dm_pulldn_en(&self) -> bool {
        (self.value.read_bits() & Self::DM_PULLDN_EN_MASK) != 0
    }

    /// Set DM_PULLDN_EN bit atomically (RW)
    #[inline(always)]
    pub fn set_dm_pulldn_en(&self) {
        self.set_bits(Self::DM_PULLDN_EN_MASK);
    }

    /// Clear DM_PULLDN_EN bit atomically (RW)
    #[inline(always)]
    pub fn clr_dm_pulldn_en(&self) {
        self.clr_bits(Self::DM_PULLDN_EN_MASK);
    }

    /// **DM_PULLUP_EN** `[5]` (RW)
    ///
    /// DM pull up enable
    #[inline(always)]
    pub fn dm_pullup_en(&self) -> bool {
        (self.value.read_bits() & Self::DM_PULLUP_EN_MASK) != 0
    }

    /// Set DM_PULLUP_EN bit atomically (RW)
    #[inline(always)]
    pub fn set_dm_pullup_en(&self) {
        self.set_bits(Self::DM_PULLUP_EN_MASK);
    }

    /// Clear DM_PULLUP_EN bit atomically (RW)
    #[inline(always)]
    pub fn clr_dm_pullup_en(&self) {
        self.clr_bits(Self::DM_PULLUP_EN_MASK);
    }

    /// **DM_PULLUP_HISEL** `[4]` (RW)
    ///
    /// Enable the second DM pull up resistor. 0 - Pull = Rpu2; 1 - Pull = Rpu1 + Rpu2
    #[inline(always)]
    pub fn dm_pullup_hisel(&self) -> bool {
        (self.value.read_bits() & Self::DM_PULLUP_HISEL_MASK) != 0
    }

    /// Set DM_PULLUP_HISEL bit atomically (RW)
    #[inline(always)]
    pub fn set_dm_pullup_hisel(&self) {
        self.set_bits(Self::DM_PULLUP_HISEL_MASK);
    }

    /// Clear DM_PULLUP_HISEL bit atomically (RW)
    #[inline(always)]
    pub fn clr_dm_pullup_hisel(&self) {
        self.clr_bits(Self::DM_PULLUP_HISEL_MASK);
    }

    /// **DP_PULLDN_EN** `[2]` (RW)
    ///
    /// DP pull down enable
    #[inline(always)]
    pub fn dp_pulldn_en(&self) -> bool {
        (self.value.read_bits() & Self::DP_PULLDN_EN_MASK) != 0
    }

    /// Set DP_PULLDN_EN bit atomically (RW)
    #[inline(always)]
    pub fn set_dp_pulldn_en(&self) {
        self.set_bits(Self::DP_PULLDN_EN_MASK);
    }

    /// Clear DP_PULLDN_EN bit atomically (RW)
    #[inline(always)]
    pub fn clr_dp_pulldn_en(&self) {
        self.clr_bits(Self::DP_PULLDN_EN_MASK);
    }

    /// **DP_PULLUP_EN** `[1]` (RW)
    ///
    /// DP pull up enable
    #[inline(always)]
    pub fn dp_pullup_en(&self) -> bool {
        (self.value.read_bits() & Self::DP_PULLUP_EN_MASK) != 0
    }

    /// Set DP_PULLUP_EN bit atomically (RW)
    #[inline(always)]
    pub fn set_dp_pullup_en(&self) {
        self.set_bits(Self::DP_PULLUP_EN_MASK);
    }

    /// Clear DP_PULLUP_EN bit atomically (RW)
    #[inline(always)]
    pub fn clr_dp_pullup_en(&self) {
        self.clr_bits(Self::DP_PULLUP_EN_MASK);
    }

    /// **DP_PULLUP_HISEL** `[0]` (RW)
    ///
    /// Enable the second DP pull up resistor. 0 - Pull = Rpu2; 1 - Pull = Rpu1 + Rpu2
    #[inline(always)]
    pub fn dp_pullup_hisel(&self) -> bool {
        (self.value.read_bits() & Self::DP_PULLUP_HISEL_MASK) != 0
    }

    /// Set DP_PULLUP_HISEL bit atomically (RW)
    #[inline(always)]
    pub fn set_dp_pullup_hisel(&self) {
        self.set_bits(Self::DP_PULLUP_HISEL_MASK);
    }

    /// Clear DP_PULLUP_HISEL bit atomically (RW)
    #[inline(always)]
    pub fn clr_dp_pullup_hisel(&self) {
        self.clr_bits(Self::DP_PULLUP_HISEL_MASK);
    }
}

/// USBPHY_DIRECT value type for batch read/modify/write operations
///
/// Use `usbphydirect.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `usbphydirect.write_val(val)`.
#[derive(Clone, Copy)]
pub struct UsbphyDirectVal {
    bits: u32,
}

impl UsbphyDirectVal {
    /// Create from raw bits
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

    /// Read **RX_DM_OVERRIDE** `[25:25]`
    ///
    /// Override rx_dm value into controller
    #[inline(always)]
    pub const fn rx_dm_override(&self) -> bool {
        (self.bits & UsbphyDirect::RX_DM_OVERRIDE_MASK) != 0
    }

    /// Read **RX_DP_OVERRIDE** `[24:24]`
    ///
    /// Override rx_dp value into controller
    #[inline(always)]
    pub const fn rx_dp_override(&self) -> bool {
        (self.bits & UsbphyDirect::RX_DP_OVERRIDE_MASK) != 0
    }

    /// Read **RX_DD_OVERRIDE** `[23:23]`
    ///
    /// Override rx_dd value into controller
    #[inline(always)]
    pub const fn rx_dd_override(&self) -> bool {
        (self.bits & UsbphyDirect::RX_DD_OVERRIDE_MASK) != 0
    }

    /// Read **DM_OVV** `[22:22]`
    ///
    /// DM over voltage
    #[inline(always)]
    pub const fn dm_ovv(&self) -> bool {
        (self.bits & UsbphyDirect::DM_OVV_MASK) != 0
    }

    /// Read **DP_OVV** `[21:21]`
    ///
    /// DP over voltage
    #[inline(always)]
    pub const fn dp_ovv(&self) -> bool {
        (self.bits & UsbphyDirect::DP_OVV_MASK) != 0
    }

    /// Read **DM_OVCN** `[20:20]`
    ///
    /// DM overcurrent
    #[inline(always)]
    pub const fn dm_ovcn(&self) -> bool {
        (self.bits & UsbphyDirect::DM_OVCN_MASK) != 0
    }

    /// Read **DP_OVCN** `[19:19]`
    ///
    /// DP overcurrent
    #[inline(always)]
    pub const fn dp_ovcn(&self) -> bool {
        (self.bits & UsbphyDirect::DP_OVCN_MASK) != 0
    }

    /// Read **RX_DM** `[18:18]`
    ///
    /// DPM pin state
    #[inline(always)]
    pub const fn rx_dm(&self) -> bool {
        (self.bits & UsbphyDirect::RX_DM_MASK) != 0
    }

    /// Read **RX_DP** `[17:17]`
    ///
    /// DPP pin state
    #[inline(always)]
    pub const fn rx_dp(&self) -> bool {
        (self.bits & UsbphyDirect::RX_DP_MASK) != 0
    }

    /// Read **RX_DD** `[16:16]`
    ///
    /// Differential RX
    #[inline(always)]
    pub const fn rx_dd(&self) -> bool {
        (self.bits & UsbphyDirect::RX_DD_MASK) != 0
    }

    /// Read **TX_DIFFMODE** `[15:15]`
    ///
    /// TX_DIFFMODE=0: Single ended mode
    /// TX_DIFFMODE=1: Differential drive mode (TX_DM, TX_DM_OE ignored)
    #[inline(always)]
    pub const fn tx_diffmode(&self) -> bool {
        (self.bits & UsbphyDirect::TX_DIFFMODE_MASK) != 0
    }

    /// Read **TX_FSSLEW** `[14:14]`
    ///
    /// TX_FSSLEW=0: Low speed slew rate
    /// TX_FSSLEW=1: Full speed slew rate
    #[inline(always)]
    pub const fn tx_fsslew(&self) -> bool {
        (self.bits & UsbphyDirect::TX_FSSLEW_MASK) != 0
    }

    /// Read **TX_PD** `[13:13]`
    ///
    /// TX power down override (if override enable is set). 1 = powered down.
    #[inline(always)]
    pub const fn tx_pd(&self) -> bool {
        (self.bits & UsbphyDirect::TX_PD_MASK) != 0
    }

    /// Read **RX_PD** `[12:12]`
    ///
    /// RX power down override (if override enable is set). 1 = powered down.
    #[inline(always)]
    pub const fn rx_pd(&self) -> bool {
        (self.bits & UsbphyDirect::RX_PD_MASK) != 0
    }

    /// Read **TX_DM** `[11:11]`
    ///
    /// Output data. TX_DIFFMODE=1, Ignored
    /// TX_DIFFMODE=0, Drives DPM only. TX_DM_OE=1 to enable drive. DPM=TX_DM
    #[inline(always)]
    pub const fn tx_dm(&self) -> bool {
        (self.bits & UsbphyDirect::TX_DM_MASK) != 0
    }

    /// Read **TX_DP** `[10:10]`
    ///
    /// Output data. If TX_DIFFMODE=1, Drives DPP/DPM diff pair. TX_DP_OE=1 to enable drive. DPP=TX_DP, DPM=~TX_DP
    /// If TX_DIFFMODE=0, Drives DPP only. TX_DP_OE=1 to enable drive. DPP=TX_DP
    #[inline(always)]
    pub const fn tx_dp(&self) -> bool {
        (self.bits & UsbphyDirect::TX_DP_MASK) != 0
    }

    /// Read **TX_DM_OE** `[9:9]`
    ///
    /// Output enable. If TX_DIFFMODE=1, Ignored.
    /// If TX_DIFFMODE=0, OE for DPM only. 0 - DPM in Hi-Z state; 1 - DPM driving
    #[inline(always)]
    pub const fn tx_dm_oe(&self) -> bool {
        (self.bits & UsbphyDirect::TX_DM_OE_MASK) != 0
    }

    /// Read **TX_DP_OE** `[8:8]`
    ///
    /// Output enable. If TX_DIFFMODE=1, OE for DPP/DPM diff pair. 0 - DPP/DPM in Hi-Z state; 1 - DPP/DPM driving
    /// If TX_DIFFMODE=0, OE for DPP only. 0 - DPP in Hi-Z state; 1 - DPP driving
    #[inline(always)]
    pub const fn tx_dp_oe(&self) -> bool {
        (self.bits & UsbphyDirect::TX_DP_OE_MASK) != 0
    }

    /// Read **DM_PULLDN_EN** `[6:6]`
    ///
    /// DM pull down enable
    #[inline(always)]
    pub const fn dm_pulldn_en(&self) -> bool {
        (self.bits & UsbphyDirect::DM_PULLDN_EN_MASK) != 0
    }

    /// Read **DM_PULLUP_EN** `[5:5]`
    ///
    /// DM pull up enable
    #[inline(always)]
    pub const fn dm_pullup_en(&self) -> bool {
        (self.bits & UsbphyDirect::DM_PULLUP_EN_MASK) != 0
    }

    /// Read **DM_PULLUP_HISEL** `[4:4]`
    ///
    /// Enable the second DM pull up resistor. 0 - Pull = Rpu2; 1 - Pull = Rpu1 + Rpu2
    #[inline(always)]
    pub const fn dm_pullup_hisel(&self) -> bool {
        (self.bits & UsbphyDirect::DM_PULLUP_HISEL_MASK) != 0
    }

    /// Read **DP_PULLDN_EN** `[2:2]`
    ///
    /// DP pull down enable
    #[inline(always)]
    pub const fn dp_pulldn_en(&self) -> bool {
        (self.bits & UsbphyDirect::DP_PULLDN_EN_MASK) != 0
    }

    /// Read **DP_PULLUP_EN** `[1:1]`
    ///
    /// DP pull up enable
    #[inline(always)]
    pub const fn dp_pullup_en(&self) -> bool {
        (self.bits & UsbphyDirect::DP_PULLUP_EN_MASK) != 0
    }

    /// Read **DP_PULLUP_HISEL** `[0:0]`
    ///
    /// Enable the second DP pull up resistor. 0 - Pull = Rpu2; 1 - Pull = Rpu1 + Rpu2
    #[inline(always)]
    pub const fn dp_pullup_hisel(&self) -> bool {
        (self.bits & UsbphyDirect::DP_PULLUP_HISEL_MASK) != 0
    }

    /// Set **RX_DM_OVERRIDE** `[25:25]`
    ///
    /// Override rx_dm value into controller
    #[inline(always)]
    pub const fn with_rx_dm_override(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | UsbphyDirect::RX_DM_OVERRIDE_MASK
            } else {
                self.bits & !UsbphyDirect::RX_DM_OVERRIDE_MASK
            },
        }
    }

    /// Set **RX_DP_OVERRIDE** `[24:24]`
    ///
    /// Override rx_dp value into controller
    #[inline(always)]
    pub const fn with_rx_dp_override(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | UsbphyDirect::RX_DP_OVERRIDE_MASK
            } else {
                self.bits & !UsbphyDirect::RX_DP_OVERRIDE_MASK
            },
        }
    }

    /// Set **RX_DD_OVERRIDE** `[23:23]`
    ///
    /// Override rx_dd value into controller
    #[inline(always)]
    pub const fn with_rx_dd_override(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | UsbphyDirect::RX_DD_OVERRIDE_MASK
            } else {
                self.bits & !UsbphyDirect::RX_DD_OVERRIDE_MASK
            },
        }
    }

    /// Set **TX_DIFFMODE** `[15:15]`
    ///
    /// TX_DIFFMODE=0: Single ended mode
    /// TX_DIFFMODE=1: Differential drive mode (TX_DM, TX_DM_OE ignored)
    #[inline(always)]
    pub const fn with_tx_diffmode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | UsbphyDirect::TX_DIFFMODE_MASK
            } else {
                self.bits & !UsbphyDirect::TX_DIFFMODE_MASK
            },
        }
    }

    /// Set **TX_FSSLEW** `[14:14]`
    ///
    /// TX_FSSLEW=0: Low speed slew rate
    /// TX_FSSLEW=1: Full speed slew rate
    #[inline(always)]
    pub const fn with_tx_fsslew(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | UsbphyDirect::TX_FSSLEW_MASK
            } else {
                self.bits & !UsbphyDirect::TX_FSSLEW_MASK
            },
        }
    }

    /// Set **TX_PD** `[13:13]`
    ///
    /// TX power down override (if override enable is set). 1 = powered down.
    #[inline(always)]
    pub const fn with_tx_pd(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | UsbphyDirect::TX_PD_MASK
            } else {
                self.bits & !UsbphyDirect::TX_PD_MASK
            },
        }
    }

    /// Set **RX_PD** `[12:12]`
    ///
    /// RX power down override (if override enable is set). 1 = powered down.
    #[inline(always)]
    pub const fn with_rx_pd(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | UsbphyDirect::RX_PD_MASK
            } else {
                self.bits & !UsbphyDirect::RX_PD_MASK
            },
        }
    }

    /// Set **TX_DM** `[11:11]`
    ///
    /// Output data. TX_DIFFMODE=1, Ignored
    /// TX_DIFFMODE=0, Drives DPM only. TX_DM_OE=1 to enable drive. DPM=TX_DM
    #[inline(always)]
    pub const fn with_tx_dm(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | UsbphyDirect::TX_DM_MASK
            } else {
                self.bits & !UsbphyDirect::TX_DM_MASK
            },
        }
    }

    /// Set **TX_DP** `[10:10]`
    ///
    /// Output data. If TX_DIFFMODE=1, Drives DPP/DPM diff pair. TX_DP_OE=1 to enable drive. DPP=TX_DP, DPM=~TX_DP
    /// If TX_DIFFMODE=0, Drives DPP only. TX_DP_OE=1 to enable drive. DPP=TX_DP
    #[inline(always)]
    pub const fn with_tx_dp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | UsbphyDirect::TX_DP_MASK
            } else {
                self.bits & !UsbphyDirect::TX_DP_MASK
            },
        }
    }

    /// Set **TX_DM_OE** `[9:9]`
    ///
    /// Output enable. If TX_DIFFMODE=1, Ignored.
    /// If TX_DIFFMODE=0, OE for DPM only. 0 - DPM in Hi-Z state; 1 - DPM driving
    #[inline(always)]
    pub const fn with_tx_dm_oe(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | UsbphyDirect::TX_DM_OE_MASK
            } else {
                self.bits & !UsbphyDirect::TX_DM_OE_MASK
            },
        }
    }

    /// Set **TX_DP_OE** `[8:8]`
    ///
    /// Output enable. If TX_DIFFMODE=1, OE for DPP/DPM diff pair. 0 - DPP/DPM in Hi-Z state; 1 - DPP/DPM driving
    /// If TX_DIFFMODE=0, OE for DPP only. 0 - DPP in Hi-Z state; 1 - DPP driving
    #[inline(always)]
    pub const fn with_tx_dp_oe(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | UsbphyDirect::TX_DP_OE_MASK
            } else {
                self.bits & !UsbphyDirect::TX_DP_OE_MASK
            },
        }
    }

    /// Set **DM_PULLDN_EN** `[6:6]`
    ///
    /// DM pull down enable
    #[inline(always)]
    pub const fn with_dm_pulldn_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | UsbphyDirect::DM_PULLDN_EN_MASK
            } else {
                self.bits & !UsbphyDirect::DM_PULLDN_EN_MASK
            },
        }
    }

    /// Set **DM_PULLUP_EN** `[5:5]`
    ///
    /// DM pull up enable
    #[inline(always)]
    pub const fn with_dm_pullup_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | UsbphyDirect::DM_PULLUP_EN_MASK
            } else {
                self.bits & !UsbphyDirect::DM_PULLUP_EN_MASK
            },
        }
    }

    /// Set **DM_PULLUP_HISEL** `[4:4]`
    ///
    /// Enable the second DM pull up resistor. 0 - Pull = Rpu2; 1 - Pull = Rpu1 + Rpu2
    #[inline(always)]
    pub const fn with_dm_pullup_hisel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | UsbphyDirect::DM_PULLUP_HISEL_MASK
            } else {
                self.bits & !UsbphyDirect::DM_PULLUP_HISEL_MASK
            },
        }
    }

    /// Set **DP_PULLDN_EN** `[2:2]`
    ///
    /// DP pull down enable
    #[inline(always)]
    pub const fn with_dp_pulldn_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | UsbphyDirect::DP_PULLDN_EN_MASK
            } else {
                self.bits & !UsbphyDirect::DP_PULLDN_EN_MASK
            },
        }
    }

    /// Set **DP_PULLUP_EN** `[1:1]`
    ///
    /// DP pull up enable
    #[inline(always)]
    pub const fn with_dp_pullup_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | UsbphyDirect::DP_PULLUP_EN_MASK
            } else {
                self.bits & !UsbphyDirect::DP_PULLUP_EN_MASK
            },
        }
    }

    /// Set **DP_PULLUP_HISEL** `[0:0]`
    ///
    /// Enable the second DP pull up resistor. 0 - Pull = Rpu2; 1 - Pull = Rpu1 + Rpu2
    #[inline(always)]
    pub const fn with_dp_pullup_hisel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | UsbphyDirect::DP_PULLUP_HISEL_MASK
            } else {
                self.bits & !UsbphyDirect::DP_PULLUP_HISEL_MASK
            },
        }
    }
}

impl Default for UsbphyDirectVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl UsbphyDirect {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> UsbphyDirectVal {
        UsbphyDirectVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: UsbphyDirectVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(UsbphyDirectVal) -> UsbphyDirectVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// USBPHY_DIRECT_OVERRIDE Register
///
/// Override enable for each control in usbphy_direct
///
/// Address offset: 0x0080
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **RX_DM_OVERRIDE_EN** `[18]` (RW)
///
/// - **RX_DP_OVERRIDE_EN** `[17]` (RW)
///
/// - **RX_DD_OVERRIDE_EN** `[16]` (RW)
///
/// - **TX_DIFFMODE_OVERRIDE_EN** `[15]` (RW)
///
/// - **DM_PULLUP_OVERRIDE_EN** `[12]` (RW)
///
/// - **TX_FSSLEW_OVERRIDE_EN** `[11]` (RW)
///
/// - **TX_PD_OVERRIDE_EN** `[10]` (RW)
///
/// - **RX_PD_OVERRIDE_EN** `[9]` (RW)
///
/// - **TX_DM_OVERRIDE_EN** `[8]` (RW)
///
/// - **TX_DP_OVERRIDE_EN** `[7]` (RW)
///
/// - **TX_DM_OE_OVERRIDE_EN** `[6]` (RW)
///
/// - **TX_DP_OE_OVERRIDE_EN** `[5]` (RW)
///
/// - **DM_PULLDN_EN_OVERRIDE_EN** `[4]` (RW)
///
/// - **DP_PULLDN_EN_OVERRIDE_EN** `[3]` (RW)
///
/// - **DP_PULLUP_EN_OVERRIDE_EN** `[2]` (RW)
///
/// - **DM_PULLUP_HISEL_OVERRIDE_EN** `[1]` (RW)
///
/// - **DP_PULLUP_HISEL_OVERRIDE_EN** `[0]` (RW)
#[repr(transparent)]
pub struct UsbphyDirectOverride {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for UsbphyDirectOverride {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl UsbphyDirectOverride {
    // ======== Field Constants ========

    /// **RX_DM_OVERRIDE_EN**
    /// Bits `[18:18]`, mask `0x00040000`
    pub const RX_DM_OVERRIDE_EN_OFFSET: u32 = 18;
    pub const RX_DM_OVERRIDE_EN_MASK: u32 = 0x00040000;
    pub const RX_DM_OVERRIDE_EN: u32 = 1 << 18;

    /// **RX_DP_OVERRIDE_EN**
    /// Bits `[17:17]`, mask `0x00020000`
    pub const RX_DP_OVERRIDE_EN_OFFSET: u32 = 17;
    pub const RX_DP_OVERRIDE_EN_MASK: u32 = 0x00020000;
    pub const RX_DP_OVERRIDE_EN: u32 = 1 << 17;

    /// **RX_DD_OVERRIDE_EN**
    /// Bits `[16:16]`, mask `0x00010000`
    pub const RX_DD_OVERRIDE_EN_OFFSET: u32 = 16;
    pub const RX_DD_OVERRIDE_EN_MASK: u32 = 0x00010000;
    pub const RX_DD_OVERRIDE_EN: u32 = 1 << 16;

    /// **TX_DIFFMODE_OVERRIDE_EN**
    /// Bits `[15:15]`, mask `0x00008000`
    pub const TX_DIFFMODE_OVERRIDE_EN_OFFSET: u32 = 15;
    pub const TX_DIFFMODE_OVERRIDE_EN_MASK: u32 = 0x00008000;
    pub const TX_DIFFMODE_OVERRIDE_EN: u32 = 1 << 15;

    /// **DM_PULLUP_OVERRIDE_EN**
    /// Bits `[12:12]`, mask `0x00001000`
    pub const DM_PULLUP_OVERRIDE_EN_OFFSET: u32 = 12;
    pub const DM_PULLUP_OVERRIDE_EN_MASK: u32 = 0x00001000;
    pub const DM_PULLUP_OVERRIDE_EN: u32 = 1 << 12;

    /// **TX_FSSLEW_OVERRIDE_EN**
    /// Bits `[11:11]`, mask `0x00000800`
    pub const TX_FSSLEW_OVERRIDE_EN_OFFSET: u32 = 11;
    pub const TX_FSSLEW_OVERRIDE_EN_MASK: u32 = 0x00000800;
    pub const TX_FSSLEW_OVERRIDE_EN: u32 = 1 << 11;

    /// **TX_PD_OVERRIDE_EN**
    /// Bits `[10:10]`, mask `0x00000400`
    pub const TX_PD_OVERRIDE_EN_OFFSET: u32 = 10;
    pub const TX_PD_OVERRIDE_EN_MASK: u32 = 0x00000400;
    pub const TX_PD_OVERRIDE_EN: u32 = 1 << 10;

    /// **RX_PD_OVERRIDE_EN**
    /// Bits `[9:9]`, mask `0x00000200`
    pub const RX_PD_OVERRIDE_EN_OFFSET: u32 = 9;
    pub const RX_PD_OVERRIDE_EN_MASK: u32 = 0x00000200;
    pub const RX_PD_OVERRIDE_EN: u32 = 1 << 9;

    /// **TX_DM_OVERRIDE_EN**
    /// Bits `[8:8]`, mask `0x00000100`
    pub const TX_DM_OVERRIDE_EN_OFFSET: u32 = 8;
    pub const TX_DM_OVERRIDE_EN_MASK: u32 = 0x00000100;
    pub const TX_DM_OVERRIDE_EN: u32 = 1 << 8;

    /// **TX_DP_OVERRIDE_EN**
    /// Bits `[7:7]`, mask `0x00000080`
    pub const TX_DP_OVERRIDE_EN_OFFSET: u32 = 7;
    pub const TX_DP_OVERRIDE_EN_MASK: u32 = 0x00000080;
    pub const TX_DP_OVERRIDE_EN: u32 = 1 << 7;

    /// **TX_DM_OE_OVERRIDE_EN**
    /// Bits `[6:6]`, mask `0x00000040`
    pub const TX_DM_OE_OVERRIDE_EN_OFFSET: u32 = 6;
    pub const TX_DM_OE_OVERRIDE_EN_MASK: u32 = 0x00000040;
    pub const TX_DM_OE_OVERRIDE_EN: u32 = 1 << 6;

    /// **TX_DP_OE_OVERRIDE_EN**
    /// Bits `[5:5]`, mask `0x00000020`
    pub const TX_DP_OE_OVERRIDE_EN_OFFSET: u32 = 5;
    pub const TX_DP_OE_OVERRIDE_EN_MASK: u32 = 0x00000020;
    pub const TX_DP_OE_OVERRIDE_EN: u32 = 1 << 5;

    /// **DM_PULLDN_EN_OVERRIDE_EN**
    /// Bits `[4:4]`, mask `0x00000010`
    pub const DM_PULLDN_EN_OVERRIDE_EN_OFFSET: u32 = 4;
    pub const DM_PULLDN_EN_OVERRIDE_EN_MASK: u32 = 0x00000010;
    pub const DM_PULLDN_EN_OVERRIDE_EN: u32 = 1 << 4;

    /// **DP_PULLDN_EN_OVERRIDE_EN**
    /// Bits `[3:3]`, mask `0x00000008`
    pub const DP_PULLDN_EN_OVERRIDE_EN_OFFSET: u32 = 3;
    pub const DP_PULLDN_EN_OVERRIDE_EN_MASK: u32 = 0x00000008;
    pub const DP_PULLDN_EN_OVERRIDE_EN: u32 = 1 << 3;

    /// **DP_PULLUP_EN_OVERRIDE_EN**
    /// Bits `[2:2]`, mask `0x00000004`
    pub const DP_PULLUP_EN_OVERRIDE_EN_OFFSET: u32 = 2;
    pub const DP_PULLUP_EN_OVERRIDE_EN_MASK: u32 = 0x00000004;
    pub const DP_PULLUP_EN_OVERRIDE_EN: u32 = 1 << 2;

    /// **DM_PULLUP_HISEL_OVERRIDE_EN**
    /// Bits `[1:1]`, mask `0x00000002`
    pub const DM_PULLUP_HISEL_OVERRIDE_EN_OFFSET: u32 = 1;
    pub const DM_PULLUP_HISEL_OVERRIDE_EN_MASK: u32 = 0x00000002;
    pub const DM_PULLUP_HISEL_OVERRIDE_EN: u32 = 1 << 1;

    /// **DP_PULLUP_HISEL_OVERRIDE_EN**
    /// Bits `[0:0]`, mask `0x00000001`
    pub const DP_PULLUP_HISEL_OVERRIDE_EN_OFFSET: u32 = 0;
    pub const DP_PULLUP_HISEL_OVERRIDE_EN_MASK: u32 = 0x00000001;
    pub const DP_PULLUP_HISEL_OVERRIDE_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **RX_DM_OVERRIDE_EN** `[18]` (RW)
    #[inline(always)]
    pub fn rx_dm_override_en(&self) -> bool {
        (self.value.read_bits() & Self::RX_DM_OVERRIDE_EN_MASK) != 0
    }

    /// Set RX_DM_OVERRIDE_EN bit atomically (RW)
    #[inline(always)]
    pub fn set_rx_dm_override_en(&self) {
        self.set_bits(Self::RX_DM_OVERRIDE_EN_MASK);
    }

    /// Clear RX_DM_OVERRIDE_EN bit atomically (RW)
    #[inline(always)]
    pub fn clr_rx_dm_override_en(&self) {
        self.clr_bits(Self::RX_DM_OVERRIDE_EN_MASK);
    }

    /// **RX_DP_OVERRIDE_EN** `[17]` (RW)
    #[inline(always)]
    pub fn rx_dp_override_en(&self) -> bool {
        (self.value.read_bits() & Self::RX_DP_OVERRIDE_EN_MASK) != 0
    }

    /// Set RX_DP_OVERRIDE_EN bit atomically (RW)
    #[inline(always)]
    pub fn set_rx_dp_override_en(&self) {
        self.set_bits(Self::RX_DP_OVERRIDE_EN_MASK);
    }

    /// Clear RX_DP_OVERRIDE_EN bit atomically (RW)
    #[inline(always)]
    pub fn clr_rx_dp_override_en(&self) {
        self.clr_bits(Self::RX_DP_OVERRIDE_EN_MASK);
    }

    /// **RX_DD_OVERRIDE_EN** `[16]` (RW)
    #[inline(always)]
    pub fn rx_dd_override_en(&self) -> bool {
        (self.value.read_bits() & Self::RX_DD_OVERRIDE_EN_MASK) != 0
    }

    /// Set RX_DD_OVERRIDE_EN bit atomically (RW)
    #[inline(always)]
    pub fn set_rx_dd_override_en(&self) {
        self.set_bits(Self::RX_DD_OVERRIDE_EN_MASK);
    }

    /// Clear RX_DD_OVERRIDE_EN bit atomically (RW)
    #[inline(always)]
    pub fn clr_rx_dd_override_en(&self) {
        self.clr_bits(Self::RX_DD_OVERRIDE_EN_MASK);
    }

    /// **TX_DIFFMODE_OVERRIDE_EN** `[15]` (RW)
    #[inline(always)]
    pub fn tx_diffmode_override_en(&self) -> bool {
        (self.value.read_bits() & Self::TX_DIFFMODE_OVERRIDE_EN_MASK) != 0
    }

    /// Set TX_DIFFMODE_OVERRIDE_EN bit atomically (RW)
    #[inline(always)]
    pub fn set_tx_diffmode_override_en(&self) {
        self.set_bits(Self::TX_DIFFMODE_OVERRIDE_EN_MASK);
    }

    /// Clear TX_DIFFMODE_OVERRIDE_EN bit atomically (RW)
    #[inline(always)]
    pub fn clr_tx_diffmode_override_en(&self) {
        self.clr_bits(Self::TX_DIFFMODE_OVERRIDE_EN_MASK);
    }

    /// **DM_PULLUP_OVERRIDE_EN** `[12]` (RW)
    #[inline(always)]
    pub fn dm_pullup_override_en(&self) -> bool {
        (self.value.read_bits() & Self::DM_PULLUP_OVERRIDE_EN_MASK) != 0
    }

    /// Set DM_PULLUP_OVERRIDE_EN bit atomically (RW)
    #[inline(always)]
    pub fn set_dm_pullup_override_en(&self) {
        self.set_bits(Self::DM_PULLUP_OVERRIDE_EN_MASK);
    }

    /// Clear DM_PULLUP_OVERRIDE_EN bit atomically (RW)
    #[inline(always)]
    pub fn clr_dm_pullup_override_en(&self) {
        self.clr_bits(Self::DM_PULLUP_OVERRIDE_EN_MASK);
    }

    /// **TX_FSSLEW_OVERRIDE_EN** `[11]` (RW)
    #[inline(always)]
    pub fn tx_fsslew_override_en(&self) -> bool {
        (self.value.read_bits() & Self::TX_FSSLEW_OVERRIDE_EN_MASK) != 0
    }

    /// Set TX_FSSLEW_OVERRIDE_EN bit atomically (RW)
    #[inline(always)]
    pub fn set_tx_fsslew_override_en(&self) {
        self.set_bits(Self::TX_FSSLEW_OVERRIDE_EN_MASK);
    }

    /// Clear TX_FSSLEW_OVERRIDE_EN bit atomically (RW)
    #[inline(always)]
    pub fn clr_tx_fsslew_override_en(&self) {
        self.clr_bits(Self::TX_FSSLEW_OVERRIDE_EN_MASK);
    }

    /// **TX_PD_OVERRIDE_EN** `[10]` (RW)
    #[inline(always)]
    pub fn tx_pd_override_en(&self) -> bool {
        (self.value.read_bits() & Self::TX_PD_OVERRIDE_EN_MASK) != 0
    }

    /// Set TX_PD_OVERRIDE_EN bit atomically (RW)
    #[inline(always)]
    pub fn set_tx_pd_override_en(&self) {
        self.set_bits(Self::TX_PD_OVERRIDE_EN_MASK);
    }

    /// Clear TX_PD_OVERRIDE_EN bit atomically (RW)
    #[inline(always)]
    pub fn clr_tx_pd_override_en(&self) {
        self.clr_bits(Self::TX_PD_OVERRIDE_EN_MASK);
    }

    /// **RX_PD_OVERRIDE_EN** `[9]` (RW)
    #[inline(always)]
    pub fn rx_pd_override_en(&self) -> bool {
        (self.value.read_bits() & Self::RX_PD_OVERRIDE_EN_MASK) != 0
    }

    /// Set RX_PD_OVERRIDE_EN bit atomically (RW)
    #[inline(always)]
    pub fn set_rx_pd_override_en(&self) {
        self.set_bits(Self::RX_PD_OVERRIDE_EN_MASK);
    }

    /// Clear RX_PD_OVERRIDE_EN bit atomically (RW)
    #[inline(always)]
    pub fn clr_rx_pd_override_en(&self) {
        self.clr_bits(Self::RX_PD_OVERRIDE_EN_MASK);
    }

    /// **TX_DM_OVERRIDE_EN** `[8]` (RW)
    #[inline(always)]
    pub fn tx_dm_override_en(&self) -> bool {
        (self.value.read_bits() & Self::TX_DM_OVERRIDE_EN_MASK) != 0
    }

    /// Set TX_DM_OVERRIDE_EN bit atomically (RW)
    #[inline(always)]
    pub fn set_tx_dm_override_en(&self) {
        self.set_bits(Self::TX_DM_OVERRIDE_EN_MASK);
    }

    /// Clear TX_DM_OVERRIDE_EN bit atomically (RW)
    #[inline(always)]
    pub fn clr_tx_dm_override_en(&self) {
        self.clr_bits(Self::TX_DM_OVERRIDE_EN_MASK);
    }

    /// **TX_DP_OVERRIDE_EN** `[7]` (RW)
    #[inline(always)]
    pub fn tx_dp_override_en(&self) -> bool {
        (self.value.read_bits() & Self::TX_DP_OVERRIDE_EN_MASK) != 0
    }

    /// Set TX_DP_OVERRIDE_EN bit atomically (RW)
    #[inline(always)]
    pub fn set_tx_dp_override_en(&self) {
        self.set_bits(Self::TX_DP_OVERRIDE_EN_MASK);
    }

    /// Clear TX_DP_OVERRIDE_EN bit atomically (RW)
    #[inline(always)]
    pub fn clr_tx_dp_override_en(&self) {
        self.clr_bits(Self::TX_DP_OVERRIDE_EN_MASK);
    }

    /// **TX_DM_OE_OVERRIDE_EN** `[6]` (RW)
    #[inline(always)]
    pub fn tx_dm_oe_override_en(&self) -> bool {
        (self.value.read_bits() & Self::TX_DM_OE_OVERRIDE_EN_MASK) != 0
    }

    /// Set TX_DM_OE_OVERRIDE_EN bit atomically (RW)
    #[inline(always)]
    pub fn set_tx_dm_oe_override_en(&self) {
        self.set_bits(Self::TX_DM_OE_OVERRIDE_EN_MASK);
    }

    /// Clear TX_DM_OE_OVERRIDE_EN bit atomically (RW)
    #[inline(always)]
    pub fn clr_tx_dm_oe_override_en(&self) {
        self.clr_bits(Self::TX_DM_OE_OVERRIDE_EN_MASK);
    }

    /// **TX_DP_OE_OVERRIDE_EN** `[5]` (RW)
    #[inline(always)]
    pub fn tx_dp_oe_override_en(&self) -> bool {
        (self.value.read_bits() & Self::TX_DP_OE_OVERRIDE_EN_MASK) != 0
    }

    /// Set TX_DP_OE_OVERRIDE_EN bit atomically (RW)
    #[inline(always)]
    pub fn set_tx_dp_oe_override_en(&self) {
        self.set_bits(Self::TX_DP_OE_OVERRIDE_EN_MASK);
    }

    /// Clear TX_DP_OE_OVERRIDE_EN bit atomically (RW)
    #[inline(always)]
    pub fn clr_tx_dp_oe_override_en(&self) {
        self.clr_bits(Self::TX_DP_OE_OVERRIDE_EN_MASK);
    }

    /// **DM_PULLDN_EN_OVERRIDE_EN** `[4]` (RW)
    #[inline(always)]
    pub fn dm_pulldn_en_override_en(&self) -> bool {
        (self.value.read_bits() & Self::DM_PULLDN_EN_OVERRIDE_EN_MASK) != 0
    }

    /// Set DM_PULLDN_EN_OVERRIDE_EN bit atomically (RW)
    #[inline(always)]
    pub fn set_dm_pulldn_en_override_en(&self) {
        self.set_bits(Self::DM_PULLDN_EN_OVERRIDE_EN_MASK);
    }

    /// Clear DM_PULLDN_EN_OVERRIDE_EN bit atomically (RW)
    #[inline(always)]
    pub fn clr_dm_pulldn_en_override_en(&self) {
        self.clr_bits(Self::DM_PULLDN_EN_OVERRIDE_EN_MASK);
    }

    /// **DP_PULLDN_EN_OVERRIDE_EN** `[3]` (RW)
    #[inline(always)]
    pub fn dp_pulldn_en_override_en(&self) -> bool {
        (self.value.read_bits() & Self::DP_PULLDN_EN_OVERRIDE_EN_MASK) != 0
    }

    /// Set DP_PULLDN_EN_OVERRIDE_EN bit atomically (RW)
    #[inline(always)]
    pub fn set_dp_pulldn_en_override_en(&self) {
        self.set_bits(Self::DP_PULLDN_EN_OVERRIDE_EN_MASK);
    }

    /// Clear DP_PULLDN_EN_OVERRIDE_EN bit atomically (RW)
    #[inline(always)]
    pub fn clr_dp_pulldn_en_override_en(&self) {
        self.clr_bits(Self::DP_PULLDN_EN_OVERRIDE_EN_MASK);
    }

    /// **DP_PULLUP_EN_OVERRIDE_EN** `[2]` (RW)
    #[inline(always)]
    pub fn dp_pullup_en_override_en(&self) -> bool {
        (self.value.read_bits() & Self::DP_PULLUP_EN_OVERRIDE_EN_MASK) != 0
    }

    /// Set DP_PULLUP_EN_OVERRIDE_EN bit atomically (RW)
    #[inline(always)]
    pub fn set_dp_pullup_en_override_en(&self) {
        self.set_bits(Self::DP_PULLUP_EN_OVERRIDE_EN_MASK);
    }

    /// Clear DP_PULLUP_EN_OVERRIDE_EN bit atomically (RW)
    #[inline(always)]
    pub fn clr_dp_pullup_en_override_en(&self) {
        self.clr_bits(Self::DP_PULLUP_EN_OVERRIDE_EN_MASK);
    }

    /// **DM_PULLUP_HISEL_OVERRIDE_EN** `[1]` (RW)
    #[inline(always)]
    pub fn dm_pullup_hisel_override_en(&self) -> bool {
        (self.value.read_bits() & Self::DM_PULLUP_HISEL_OVERRIDE_EN_MASK) != 0
    }

    /// Set DM_PULLUP_HISEL_OVERRIDE_EN bit atomically (RW)
    #[inline(always)]
    pub fn set_dm_pullup_hisel_override_en(&self) {
        self.set_bits(Self::DM_PULLUP_HISEL_OVERRIDE_EN_MASK);
    }

    /// Clear DM_PULLUP_HISEL_OVERRIDE_EN bit atomically (RW)
    #[inline(always)]
    pub fn clr_dm_pullup_hisel_override_en(&self) {
        self.clr_bits(Self::DM_PULLUP_HISEL_OVERRIDE_EN_MASK);
    }

    /// **DP_PULLUP_HISEL_OVERRIDE_EN** `[0]` (RW)
    #[inline(always)]
    pub fn dp_pullup_hisel_override_en(&self) -> bool {
        (self.value.read_bits() & Self::DP_PULLUP_HISEL_OVERRIDE_EN_MASK) != 0
    }

    /// Set DP_PULLUP_HISEL_OVERRIDE_EN bit atomically (RW)
    #[inline(always)]
    pub fn set_dp_pullup_hisel_override_en(&self) {
        self.set_bits(Self::DP_PULLUP_HISEL_OVERRIDE_EN_MASK);
    }

    /// Clear DP_PULLUP_HISEL_OVERRIDE_EN bit atomically (RW)
    #[inline(always)]
    pub fn clr_dp_pullup_hisel_override_en(&self) {
        self.clr_bits(Self::DP_PULLUP_HISEL_OVERRIDE_EN_MASK);
    }
}

/// USBPHY_DIRECT_OVERRIDE value type for batch read/modify/write operations
///
/// Use `usbphydirectoverride.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `usbphydirectoverride.write_val(val)`.
#[derive(Clone, Copy)]
pub struct UsbphyDirectOverrideVal {
    bits: u32,
}

impl UsbphyDirectOverrideVal {
    /// Create from raw bits
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

    /// Read **RX_DM_OVERRIDE_EN** `[18:18]`
    #[inline(always)]
    pub const fn rx_dm_override_en(&self) -> bool {
        (self.bits & UsbphyDirectOverride::RX_DM_OVERRIDE_EN_MASK) != 0
    }

    /// Read **RX_DP_OVERRIDE_EN** `[17:17]`
    #[inline(always)]
    pub const fn rx_dp_override_en(&self) -> bool {
        (self.bits & UsbphyDirectOverride::RX_DP_OVERRIDE_EN_MASK) != 0
    }

    /// Read **RX_DD_OVERRIDE_EN** `[16:16]`
    #[inline(always)]
    pub const fn rx_dd_override_en(&self) -> bool {
        (self.bits & UsbphyDirectOverride::RX_DD_OVERRIDE_EN_MASK) != 0
    }

    /// Read **TX_DIFFMODE_OVERRIDE_EN** `[15:15]`
    #[inline(always)]
    pub const fn tx_diffmode_override_en(&self) -> bool {
        (self.bits & UsbphyDirectOverride::TX_DIFFMODE_OVERRIDE_EN_MASK) != 0
    }

    /// Read **DM_PULLUP_OVERRIDE_EN** `[12:12]`
    #[inline(always)]
    pub const fn dm_pullup_override_en(&self) -> bool {
        (self.bits & UsbphyDirectOverride::DM_PULLUP_OVERRIDE_EN_MASK) != 0
    }

    /// Read **TX_FSSLEW_OVERRIDE_EN** `[11:11]`
    #[inline(always)]
    pub const fn tx_fsslew_override_en(&self) -> bool {
        (self.bits & UsbphyDirectOverride::TX_FSSLEW_OVERRIDE_EN_MASK) != 0
    }

    /// Read **TX_PD_OVERRIDE_EN** `[10:10]`
    #[inline(always)]
    pub const fn tx_pd_override_en(&self) -> bool {
        (self.bits & UsbphyDirectOverride::TX_PD_OVERRIDE_EN_MASK) != 0
    }

    /// Read **RX_PD_OVERRIDE_EN** `[9:9]`
    #[inline(always)]
    pub const fn rx_pd_override_en(&self) -> bool {
        (self.bits & UsbphyDirectOverride::RX_PD_OVERRIDE_EN_MASK) != 0
    }

    /// Read **TX_DM_OVERRIDE_EN** `[8:8]`
    #[inline(always)]
    pub const fn tx_dm_override_en(&self) -> bool {
        (self.bits & UsbphyDirectOverride::TX_DM_OVERRIDE_EN_MASK) != 0
    }

    /// Read **TX_DP_OVERRIDE_EN** `[7:7]`
    #[inline(always)]
    pub const fn tx_dp_override_en(&self) -> bool {
        (self.bits & UsbphyDirectOverride::TX_DP_OVERRIDE_EN_MASK) != 0
    }

    /// Read **TX_DM_OE_OVERRIDE_EN** `[6:6]`
    #[inline(always)]
    pub const fn tx_dm_oe_override_en(&self) -> bool {
        (self.bits & UsbphyDirectOverride::TX_DM_OE_OVERRIDE_EN_MASK) != 0
    }

    /// Read **TX_DP_OE_OVERRIDE_EN** `[5:5]`
    #[inline(always)]
    pub const fn tx_dp_oe_override_en(&self) -> bool {
        (self.bits & UsbphyDirectOverride::TX_DP_OE_OVERRIDE_EN_MASK) != 0
    }

    /// Read **DM_PULLDN_EN_OVERRIDE_EN** `[4:4]`
    #[inline(always)]
    pub const fn dm_pulldn_en_override_en(&self) -> bool {
        (self.bits & UsbphyDirectOverride::DM_PULLDN_EN_OVERRIDE_EN_MASK) != 0
    }

    /// Read **DP_PULLDN_EN_OVERRIDE_EN** `[3:3]`
    #[inline(always)]
    pub const fn dp_pulldn_en_override_en(&self) -> bool {
        (self.bits & UsbphyDirectOverride::DP_PULLDN_EN_OVERRIDE_EN_MASK) != 0
    }

    /// Read **DP_PULLUP_EN_OVERRIDE_EN** `[2:2]`
    #[inline(always)]
    pub const fn dp_pullup_en_override_en(&self) -> bool {
        (self.bits & UsbphyDirectOverride::DP_PULLUP_EN_OVERRIDE_EN_MASK) != 0
    }

    /// Read **DM_PULLUP_HISEL_OVERRIDE_EN** `[1:1]`
    #[inline(always)]
    pub const fn dm_pullup_hisel_override_en(&self) -> bool {
        (self.bits & UsbphyDirectOverride::DM_PULLUP_HISEL_OVERRIDE_EN_MASK) != 0
    }

    /// Read **DP_PULLUP_HISEL_OVERRIDE_EN** `[0:0]`
    #[inline(always)]
    pub const fn dp_pullup_hisel_override_en(&self) -> bool {
        (self.bits & UsbphyDirectOverride::DP_PULLUP_HISEL_OVERRIDE_EN_MASK) != 0
    }

    /// Set **RX_DM_OVERRIDE_EN** `[18:18]`
    #[inline(always)]
    pub const fn with_rx_dm_override_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | UsbphyDirectOverride::RX_DM_OVERRIDE_EN_MASK
            } else {
                self.bits & !UsbphyDirectOverride::RX_DM_OVERRIDE_EN_MASK
            },
        }
    }

    /// Set **RX_DP_OVERRIDE_EN** `[17:17]`
    #[inline(always)]
    pub const fn with_rx_dp_override_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | UsbphyDirectOverride::RX_DP_OVERRIDE_EN_MASK
            } else {
                self.bits & !UsbphyDirectOverride::RX_DP_OVERRIDE_EN_MASK
            },
        }
    }

    /// Set **RX_DD_OVERRIDE_EN** `[16:16]`
    #[inline(always)]
    pub const fn with_rx_dd_override_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | UsbphyDirectOverride::RX_DD_OVERRIDE_EN_MASK
            } else {
                self.bits & !UsbphyDirectOverride::RX_DD_OVERRIDE_EN_MASK
            },
        }
    }

    /// Set **TX_DIFFMODE_OVERRIDE_EN** `[15:15]`
    #[inline(always)]
    pub const fn with_tx_diffmode_override_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | UsbphyDirectOverride::TX_DIFFMODE_OVERRIDE_EN_MASK
            } else {
                self.bits & !UsbphyDirectOverride::TX_DIFFMODE_OVERRIDE_EN_MASK
            },
        }
    }

    /// Set **DM_PULLUP_OVERRIDE_EN** `[12:12]`
    #[inline(always)]
    pub const fn with_dm_pullup_override_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | UsbphyDirectOverride::DM_PULLUP_OVERRIDE_EN_MASK
            } else {
                self.bits & !UsbphyDirectOverride::DM_PULLUP_OVERRIDE_EN_MASK
            },
        }
    }

    /// Set **TX_FSSLEW_OVERRIDE_EN** `[11:11]`
    #[inline(always)]
    pub const fn with_tx_fsslew_override_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | UsbphyDirectOverride::TX_FSSLEW_OVERRIDE_EN_MASK
            } else {
                self.bits & !UsbphyDirectOverride::TX_FSSLEW_OVERRIDE_EN_MASK
            },
        }
    }

    /// Set **TX_PD_OVERRIDE_EN** `[10:10]`
    #[inline(always)]
    pub const fn with_tx_pd_override_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | UsbphyDirectOverride::TX_PD_OVERRIDE_EN_MASK
            } else {
                self.bits & !UsbphyDirectOverride::TX_PD_OVERRIDE_EN_MASK
            },
        }
    }

    /// Set **RX_PD_OVERRIDE_EN** `[9:9]`
    #[inline(always)]
    pub const fn with_rx_pd_override_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | UsbphyDirectOverride::RX_PD_OVERRIDE_EN_MASK
            } else {
                self.bits & !UsbphyDirectOverride::RX_PD_OVERRIDE_EN_MASK
            },
        }
    }

    /// Set **TX_DM_OVERRIDE_EN** `[8:8]`
    #[inline(always)]
    pub const fn with_tx_dm_override_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | UsbphyDirectOverride::TX_DM_OVERRIDE_EN_MASK
            } else {
                self.bits & !UsbphyDirectOverride::TX_DM_OVERRIDE_EN_MASK
            },
        }
    }

    /// Set **TX_DP_OVERRIDE_EN** `[7:7]`
    #[inline(always)]
    pub const fn with_tx_dp_override_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | UsbphyDirectOverride::TX_DP_OVERRIDE_EN_MASK
            } else {
                self.bits & !UsbphyDirectOverride::TX_DP_OVERRIDE_EN_MASK
            },
        }
    }

    /// Set **TX_DM_OE_OVERRIDE_EN** `[6:6]`
    #[inline(always)]
    pub const fn with_tx_dm_oe_override_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | UsbphyDirectOverride::TX_DM_OE_OVERRIDE_EN_MASK
            } else {
                self.bits & !UsbphyDirectOverride::TX_DM_OE_OVERRIDE_EN_MASK
            },
        }
    }

    /// Set **TX_DP_OE_OVERRIDE_EN** `[5:5]`
    #[inline(always)]
    pub const fn with_tx_dp_oe_override_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | UsbphyDirectOverride::TX_DP_OE_OVERRIDE_EN_MASK
            } else {
                self.bits & !UsbphyDirectOverride::TX_DP_OE_OVERRIDE_EN_MASK
            },
        }
    }

    /// Set **DM_PULLDN_EN_OVERRIDE_EN** `[4:4]`
    #[inline(always)]
    pub const fn with_dm_pulldn_en_override_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | UsbphyDirectOverride::DM_PULLDN_EN_OVERRIDE_EN_MASK
            } else {
                self.bits & !UsbphyDirectOverride::DM_PULLDN_EN_OVERRIDE_EN_MASK
            },
        }
    }

    /// Set **DP_PULLDN_EN_OVERRIDE_EN** `[3:3]`
    #[inline(always)]
    pub const fn with_dp_pulldn_en_override_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | UsbphyDirectOverride::DP_PULLDN_EN_OVERRIDE_EN_MASK
            } else {
                self.bits & !UsbphyDirectOverride::DP_PULLDN_EN_OVERRIDE_EN_MASK
            },
        }
    }

    /// Set **DP_PULLUP_EN_OVERRIDE_EN** `[2:2]`
    #[inline(always)]
    pub const fn with_dp_pullup_en_override_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | UsbphyDirectOverride::DP_PULLUP_EN_OVERRIDE_EN_MASK
            } else {
                self.bits & !UsbphyDirectOverride::DP_PULLUP_EN_OVERRIDE_EN_MASK
            },
        }
    }

    /// Set **DM_PULLUP_HISEL_OVERRIDE_EN** `[1:1]`
    #[inline(always)]
    pub const fn with_dm_pullup_hisel_override_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | UsbphyDirectOverride::DM_PULLUP_HISEL_OVERRIDE_EN_MASK
            } else {
                self.bits & !UsbphyDirectOverride::DM_PULLUP_HISEL_OVERRIDE_EN_MASK
            },
        }
    }

    /// Set **DP_PULLUP_HISEL_OVERRIDE_EN** `[0:0]`
    #[inline(always)]
    pub const fn with_dp_pullup_hisel_override_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | UsbphyDirectOverride::DP_PULLUP_HISEL_OVERRIDE_EN_MASK
            } else {
                self.bits & !UsbphyDirectOverride::DP_PULLUP_HISEL_OVERRIDE_EN_MASK
            },
        }
    }
}

impl Default for UsbphyDirectOverrideVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl UsbphyDirectOverride {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> UsbphyDirectOverrideVal {
        UsbphyDirectOverrideVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: UsbphyDirectOverrideVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(UsbphyDirectOverrideVal) -> UsbphyDirectOverrideVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// USBPHY_TRIM Register
///
/// Used to adjust trim values of USB phy pull down resistors.
///
/// Address offset: 0x0084
/// Reset value: 0x00001f1f
///
/// ## Fields
///
/// - **DM_PULLDN_TRIM** `[12:8]` (RW)
///
///   Value to drive to USB PHY
///   DM pulldown resistor trim control
///   Experimental data suggests that the reset value will work, but this register allows adjustment if required
///
/// - **DP_PULLDN_TRIM** `[4:0]` (RW)
///
///   Value to drive to USB PHY
///   DP pulldown resistor trim control
///   Experimental data suggests that the reset value will work, but this register allows adjustment if required
#[repr(transparent)]
pub struct UsbphyTrim {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for UsbphyTrim {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl UsbphyTrim {
    // ======== Field Constants ========

    /// **DM_PULLDN_TRIM**
    ///
    /// Value to drive to USB PHY
    /// DM pulldown resistor trim control
    /// Experimental data suggests that the reset value will work, but this register allows adjustment if required
    ///
    /// Bits `[12:8]`, mask `0x00001f00`
    pub const DM_PULLDN_TRIM_OFFSET: u32 = 8;
    pub const DM_PULLDN_TRIM_MASK: u32 = 0x00001f00;

    /// **DP_PULLDN_TRIM**
    ///
    /// Value to drive to USB PHY
    /// DP pulldown resistor trim control
    /// Experimental data suggests that the reset value will work, but this register allows adjustment if required
    ///
    /// Bits `[4:0]`, mask `0x0000001f`
    pub const DP_PULLDN_TRIM_OFFSET: u32 = 0;
    pub const DP_PULLDN_TRIM_MASK: u32 = 0x0000001f;

    // ======== Field Accessors ========

    /// **DM_PULLDN_TRIM** `[12:8]` (RW)
    ///
    /// Value to drive to USB PHY
    /// DM pulldown resistor trim control
    /// Experimental data suggests that the reset value will work, but this register allows adjustment if required
    #[inline(always)]
    pub fn dm_pulldn_trim(&self) -> u32 {
        (self.value.read_bits() & Self::DM_PULLDN_TRIM_MASK) >> Self::DM_PULLDN_TRIM_OFFSET
    }

    /// Write **DM_PULLDN_TRIM** `[12:8]` (RW)
    ///
    /// Value to drive to USB PHY
    /// DM pulldown resistor trim control
    /// Experimental data suggests that the reset value will work, but this register allows adjustment if required
    #[inline(always)]
    pub fn write_dm_pulldn_trim(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DM_PULLDN_TRIM_MASK)
                | ((value << Self::DM_PULLDN_TRIM_OFFSET) & Self::DM_PULLDN_TRIM_MASK)
        });
    }

    /// **DP_PULLDN_TRIM** `[4:0]` (RW)
    ///
    /// Value to drive to USB PHY
    /// DP pulldown resistor trim control
    /// Experimental data suggests that the reset value will work, but this register allows adjustment if required
    #[inline(always)]
    pub fn dp_pulldn_trim(&self) -> u32 {
        (self.value.read_bits() & Self::DP_PULLDN_TRIM_MASK) >> Self::DP_PULLDN_TRIM_OFFSET
    }

    /// Write **DP_PULLDN_TRIM** `[4:0]` (RW)
    ///
    /// Value to drive to USB PHY
    /// DP pulldown resistor trim control
    /// Experimental data suggests that the reset value will work, but this register allows adjustment if required
    #[inline(always)]
    pub fn write_dp_pulldn_trim(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DP_PULLDN_TRIM_MASK)
                | ((value << Self::DP_PULLDN_TRIM_OFFSET) & Self::DP_PULLDN_TRIM_MASK)
        });
    }
}

/// USBPHY_TRIM value type for batch read/modify/write operations
///
/// Use `usbphytrim.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `usbphytrim.write_val(val)`.
#[derive(Clone, Copy)]
pub struct UsbphyTrimVal {
    bits: u32,
}

impl UsbphyTrimVal {
    /// Create from raw bits
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
        Self { bits: 0x00001f1f }
    }

    /// Read **DM_PULLDN_TRIM** `[12:8]`
    ///
    /// Value to drive to USB PHY
    /// DM pulldown resistor trim control
    /// Experimental data suggests that the reset value will work, but this register allows adjustment if required
    #[inline(always)]
    pub const fn dm_pulldn_trim(&self) -> u32 {
        (self.bits & UsbphyTrim::DM_PULLDN_TRIM_MASK) >> UsbphyTrim::DM_PULLDN_TRIM_OFFSET
    }

    /// Read **DP_PULLDN_TRIM** `[4:0]`
    ///
    /// Value to drive to USB PHY
    /// DP pulldown resistor trim control
    /// Experimental data suggests that the reset value will work, but this register allows adjustment if required
    #[inline(always)]
    pub const fn dp_pulldn_trim(&self) -> u32 {
        (self.bits & UsbphyTrim::DP_PULLDN_TRIM_MASK) >> UsbphyTrim::DP_PULLDN_TRIM_OFFSET
    }

    /// Set **DM_PULLDN_TRIM** `[12:8]`
    ///
    /// Value to drive to USB PHY
    /// DM pulldown resistor trim control
    /// Experimental data suggests that the reset value will work, but this register allows adjustment if required
    #[inline(always)]
    pub const fn with_dm_pulldn_trim(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !UsbphyTrim::DM_PULLDN_TRIM_MASK)
                | ((value << UsbphyTrim::DM_PULLDN_TRIM_OFFSET) & UsbphyTrim::DM_PULLDN_TRIM_MASK),
        }
    }

    /// Set **DP_PULLDN_TRIM** `[4:0]`
    ///
    /// Value to drive to USB PHY
    /// DP pulldown resistor trim control
    /// Experimental data suggests that the reset value will work, but this register allows adjustment if required
    #[inline(always)]
    pub const fn with_dp_pulldn_trim(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !UsbphyTrim::DP_PULLDN_TRIM_MASK)
                | ((value << UsbphyTrim::DP_PULLDN_TRIM_OFFSET) & UsbphyTrim::DP_PULLDN_TRIM_MASK),
        }
    }
}

impl Default for UsbphyTrimVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl UsbphyTrim {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> UsbphyTrimVal {
        UsbphyTrimVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: UsbphyTrimVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(UsbphyTrimVal) -> UsbphyTrimVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LINESTATE_TUNING Register
///
/// Used for debug only.
///
/// Address offset: 0x0088
/// Reset value: 0x000000f8
///
/// ## Fields
///
/// - **SPARE_FIX** `[11:8]` (RW)
///
/// - **DEV_LS_WAKE_FIX** `[7]` (RW)
///
///   Device - exit suspend on any non-idle signalling, not qualified with a 1ms timer
///
/// - **DEV_RX_ERR_QUIESCE** `[6]` (RW)
///
///   Device - suppress repeated errors until the device FSM is next in the process of decoding an inbound packet.
///
/// - **SIE_RX_CHATTER_SE0_FIX** `[5]` (RW)
///
///   RX - when recovering from line chatter or bitstuff errors, treat SE0 as the end of chatter as well as
///   8 consecutive idle bits.
///
/// - **SIE_RX_BITSTUFF_FIX** `[4]` (RW)
///
///   RX - when a bitstuff error is signalled by rx_dasm, unconditionally terminate RX decode to
///   avoid a hang during certain packet phases.
///
/// - **DEV_BUFF_CONTROL_DOUBLE_READ_FIX** `[3]` (RW)
///
///   Device - the controller FSM performs two reads of the buffer status memory address to
///   avoid sampling metastable data. An enabled buffer is only used if both reads match.
///
/// - **MULTI_HUB_FIX** `[2]` (RW)
///
///   Host - increase inter-packet and turnaround timeouts to accommodate worst-case hub delays.
///
/// - **LINESTATE_DELAY** `[1]` (RW)
///
///   Device/Host - add an extra 1-bit debounce of linestate sampling.
///
/// - **RCV_DELAY** `[0]` (RW)
///
///   Device - register the received data to account for hub bit dribble before EOP. Only affects certain hubs.
#[repr(transparent)]
pub struct LinestateTuning {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for LinestateTuning {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LinestateTuning {
    // ======== Field Constants ========

    /// **SPARE_FIX**
    /// Bits `[11:8]`, mask `0x00000f00`
    pub const SPARE_FIX_OFFSET: u32 = 8;
    pub const SPARE_FIX_MASK: u32 = 0x00000f00;

    /// **DEV_LS_WAKE_FIX**
    ///
    /// Device - exit suspend on any non-idle signalling, not qualified with a 1ms timer
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const DEV_LS_WAKE_FIX_OFFSET: u32 = 7;
    pub const DEV_LS_WAKE_FIX_MASK: u32 = 0x00000080;
    pub const DEV_LS_WAKE_FIX: u32 = 1 << 7;

    /// **DEV_RX_ERR_QUIESCE**
    ///
    /// Device - suppress repeated errors until the device FSM is next in the process of decoding an inbound packet.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const DEV_RX_ERR_QUIESCE_OFFSET: u32 = 6;
    pub const DEV_RX_ERR_QUIESCE_MASK: u32 = 0x00000040;
    pub const DEV_RX_ERR_QUIESCE: u32 = 1 << 6;

    /// **SIE_RX_CHATTER_SE0_FIX**
    ///
    /// RX - when recovering from line chatter or bitstuff errors, treat SE0 as the end of chatter as well as
    /// 8 consecutive idle bits.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const SIE_RX_CHATTER_SE0_FIX_OFFSET: u32 = 5;
    pub const SIE_RX_CHATTER_SE0_FIX_MASK: u32 = 0x00000020;
    pub const SIE_RX_CHATTER_SE0_FIX: u32 = 1 << 5;

    /// **SIE_RX_BITSTUFF_FIX**
    ///
    /// RX - when a bitstuff error is signalled by rx_dasm, unconditionally terminate RX decode to
    /// avoid a hang during certain packet phases.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const SIE_RX_BITSTUFF_FIX_OFFSET: u32 = 4;
    pub const SIE_RX_BITSTUFF_FIX_MASK: u32 = 0x00000010;
    pub const SIE_RX_BITSTUFF_FIX: u32 = 1 << 4;

    /// **DEV_BUFF_CONTROL_DOUBLE_READ_FIX**
    ///
    /// Device - the controller FSM performs two reads of the buffer status memory address to
    /// avoid sampling metastable data. An enabled buffer is only used if both reads match.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const DEV_BUFF_CONTROL_DOUBLE_READ_FIX_OFFSET: u32 = 3;
    pub const DEV_BUFF_CONTROL_DOUBLE_READ_FIX_MASK: u32 = 0x00000008;
    pub const DEV_BUFF_CONTROL_DOUBLE_READ_FIX: u32 = 1 << 3;

    /// **MULTI_HUB_FIX**
    ///
    /// Host - increase inter-packet and turnaround timeouts to accommodate worst-case hub delays.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const MULTI_HUB_FIX_OFFSET: u32 = 2;
    pub const MULTI_HUB_FIX_MASK: u32 = 0x00000004;
    pub const MULTI_HUB_FIX: u32 = 1 << 2;

    /// **LINESTATE_DELAY**
    ///
    /// Device/Host - add an extra 1-bit debounce of linestate sampling.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const LINESTATE_DELAY_OFFSET: u32 = 1;
    pub const LINESTATE_DELAY_MASK: u32 = 0x00000002;
    pub const LINESTATE_DELAY: u32 = 1 << 1;

    /// **RCV_DELAY**
    ///
    /// Device - register the received data to account for hub bit dribble before EOP. Only affects certain hubs.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const RCV_DELAY_OFFSET: u32 = 0;
    pub const RCV_DELAY_MASK: u32 = 0x00000001;
    pub const RCV_DELAY: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **SPARE_FIX** `[11:8]` (RW)
    #[inline(always)]
    pub fn spare_fix(&self) -> u32 {
        (self.value.read_bits() & Self::SPARE_FIX_MASK) >> Self::SPARE_FIX_OFFSET
    }

    /// Write **SPARE_FIX** `[11:8]` (RW)
    #[inline(always)]
    pub fn write_spare_fix(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SPARE_FIX_MASK) | ((value << Self::SPARE_FIX_OFFSET) & Self::SPARE_FIX_MASK)
        });
    }

    /// **DEV_LS_WAKE_FIX** `[7]` (RW)
    ///
    /// Device - exit suspend on any non-idle signalling, not qualified with a 1ms timer
    #[inline(always)]
    pub fn dev_ls_wake_fix(&self) -> bool {
        (self.value.read_bits() & Self::DEV_LS_WAKE_FIX_MASK) != 0
    }

    /// Set DEV_LS_WAKE_FIX bit atomically (RW)
    #[inline(always)]
    pub fn set_dev_ls_wake_fix(&self) {
        self.set_bits(Self::DEV_LS_WAKE_FIX_MASK);
    }

    /// Clear DEV_LS_WAKE_FIX bit atomically (RW)
    #[inline(always)]
    pub fn clr_dev_ls_wake_fix(&self) {
        self.clr_bits(Self::DEV_LS_WAKE_FIX_MASK);
    }

    /// **DEV_RX_ERR_QUIESCE** `[6]` (RW)
    ///
    /// Device - suppress repeated errors until the device FSM is next in the process of decoding an inbound packet.
    #[inline(always)]
    pub fn dev_rx_err_quiesce(&self) -> bool {
        (self.value.read_bits() & Self::DEV_RX_ERR_QUIESCE_MASK) != 0
    }

    /// Set DEV_RX_ERR_QUIESCE bit atomically (RW)
    #[inline(always)]
    pub fn set_dev_rx_err_quiesce(&self) {
        self.set_bits(Self::DEV_RX_ERR_QUIESCE_MASK);
    }

    /// Clear DEV_RX_ERR_QUIESCE bit atomically (RW)
    #[inline(always)]
    pub fn clr_dev_rx_err_quiesce(&self) {
        self.clr_bits(Self::DEV_RX_ERR_QUIESCE_MASK);
    }

    /// **SIE_RX_CHATTER_SE0_FIX** `[5]` (RW)
    ///
    /// RX - when recovering from line chatter or bitstuff errors, treat SE0 as the end of chatter as well as
    /// 8 consecutive idle bits.
    #[inline(always)]
    pub fn sie_rx_chatter_se0_fix(&self) -> bool {
        (self.value.read_bits() & Self::SIE_RX_CHATTER_SE0_FIX_MASK) != 0
    }

    /// Set SIE_RX_CHATTER_SE0_FIX bit atomically (RW)
    #[inline(always)]
    pub fn set_sie_rx_chatter_se0_fix(&self) {
        self.set_bits(Self::SIE_RX_CHATTER_SE0_FIX_MASK);
    }

    /// Clear SIE_RX_CHATTER_SE0_FIX bit atomically (RW)
    #[inline(always)]
    pub fn clr_sie_rx_chatter_se0_fix(&self) {
        self.clr_bits(Self::SIE_RX_CHATTER_SE0_FIX_MASK);
    }

    /// **SIE_RX_BITSTUFF_FIX** `[4]` (RW)
    ///
    /// RX - when a bitstuff error is signalled by rx_dasm, unconditionally terminate RX decode to
    /// avoid a hang during certain packet phases.
    #[inline(always)]
    pub fn sie_rx_bitstuff_fix(&self) -> bool {
        (self.value.read_bits() & Self::SIE_RX_BITSTUFF_FIX_MASK) != 0
    }

    /// Set SIE_RX_BITSTUFF_FIX bit atomically (RW)
    #[inline(always)]
    pub fn set_sie_rx_bitstuff_fix(&self) {
        self.set_bits(Self::SIE_RX_BITSTUFF_FIX_MASK);
    }

    /// Clear SIE_RX_BITSTUFF_FIX bit atomically (RW)
    #[inline(always)]
    pub fn clr_sie_rx_bitstuff_fix(&self) {
        self.clr_bits(Self::SIE_RX_BITSTUFF_FIX_MASK);
    }

    /// **DEV_BUFF_CONTROL_DOUBLE_READ_FIX** `[3]` (RW)
    ///
    /// Device - the controller FSM performs two reads of the buffer status memory address to
    /// avoid sampling metastable data. An enabled buffer is only used if both reads match.
    #[inline(always)]
    pub fn dev_buff_control_double_read_fix(&self) -> bool {
        (self.value.read_bits() & Self::DEV_BUFF_CONTROL_DOUBLE_READ_FIX_MASK) != 0
    }

    /// Set DEV_BUFF_CONTROL_DOUBLE_READ_FIX bit atomically (RW)
    #[inline(always)]
    pub fn set_dev_buff_control_double_read_fix(&self) {
        self.set_bits(Self::DEV_BUFF_CONTROL_DOUBLE_READ_FIX_MASK);
    }

    /// Clear DEV_BUFF_CONTROL_DOUBLE_READ_FIX bit atomically (RW)
    #[inline(always)]
    pub fn clr_dev_buff_control_double_read_fix(&self) {
        self.clr_bits(Self::DEV_BUFF_CONTROL_DOUBLE_READ_FIX_MASK);
    }

    /// **MULTI_HUB_FIX** `[2]` (RW)
    ///
    /// Host - increase inter-packet and turnaround timeouts to accommodate worst-case hub delays.
    #[inline(always)]
    pub fn multi_hub_fix(&self) -> bool {
        (self.value.read_bits() & Self::MULTI_HUB_FIX_MASK) != 0
    }

    /// Set MULTI_HUB_FIX bit atomically (RW)
    #[inline(always)]
    pub fn set_multi_hub_fix(&self) {
        self.set_bits(Self::MULTI_HUB_FIX_MASK);
    }

    /// Clear MULTI_HUB_FIX bit atomically (RW)
    #[inline(always)]
    pub fn clr_multi_hub_fix(&self) {
        self.clr_bits(Self::MULTI_HUB_FIX_MASK);
    }

    /// **LINESTATE_DELAY** `[1]` (RW)
    ///
    /// Device/Host - add an extra 1-bit debounce of linestate sampling.
    #[inline(always)]
    pub fn linestate_delay(&self) -> bool {
        (self.value.read_bits() & Self::LINESTATE_DELAY_MASK) != 0
    }

    /// Set LINESTATE_DELAY bit atomically (RW)
    #[inline(always)]
    pub fn set_linestate_delay(&self) {
        self.set_bits(Self::LINESTATE_DELAY_MASK);
    }

    /// Clear LINESTATE_DELAY bit atomically (RW)
    #[inline(always)]
    pub fn clr_linestate_delay(&self) {
        self.clr_bits(Self::LINESTATE_DELAY_MASK);
    }

    /// **RCV_DELAY** `[0]` (RW)
    ///
    /// Device - register the received data to account for hub bit dribble before EOP. Only affects certain hubs.
    #[inline(always)]
    pub fn rcv_delay(&self) -> bool {
        (self.value.read_bits() & Self::RCV_DELAY_MASK) != 0
    }

    /// Set RCV_DELAY bit atomically (RW)
    #[inline(always)]
    pub fn set_rcv_delay(&self) {
        self.set_bits(Self::RCV_DELAY_MASK);
    }

    /// Clear RCV_DELAY bit atomically (RW)
    #[inline(always)]
    pub fn clr_rcv_delay(&self) {
        self.clr_bits(Self::RCV_DELAY_MASK);
    }
}

/// LINESTATE_TUNING value type for batch read/modify/write operations
///
/// Use `linestatetuning.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `linestatetuning.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LinestateTuningVal {
    bits: u32,
}

impl LinestateTuningVal {
    /// Create from raw bits
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

    /// Read **SPARE_FIX** `[11:8]`
    #[inline(always)]
    pub const fn spare_fix(&self) -> u32 {
        (self.bits & LinestateTuning::SPARE_FIX_MASK) >> LinestateTuning::SPARE_FIX_OFFSET
    }

    /// Read **DEV_LS_WAKE_FIX** `[7:7]`
    ///
    /// Device - exit suspend on any non-idle signalling, not qualified with a 1ms timer
    #[inline(always)]
    pub const fn dev_ls_wake_fix(&self) -> bool {
        (self.bits & LinestateTuning::DEV_LS_WAKE_FIX_MASK) != 0
    }

    /// Read **DEV_RX_ERR_QUIESCE** `[6:6]`
    ///
    /// Device - suppress repeated errors until the device FSM is next in the process of decoding an inbound packet.
    #[inline(always)]
    pub const fn dev_rx_err_quiesce(&self) -> bool {
        (self.bits & LinestateTuning::DEV_RX_ERR_QUIESCE_MASK) != 0
    }

    /// Read **SIE_RX_CHATTER_SE0_FIX** `[5:5]`
    ///
    /// RX - when recovering from line chatter or bitstuff errors, treat SE0 as the end of chatter as well as
    /// 8 consecutive idle bits.
    #[inline(always)]
    pub const fn sie_rx_chatter_se0_fix(&self) -> bool {
        (self.bits & LinestateTuning::SIE_RX_CHATTER_SE0_FIX_MASK) != 0
    }

    /// Read **SIE_RX_BITSTUFF_FIX** `[4:4]`
    ///
    /// RX - when a bitstuff error is signalled by rx_dasm, unconditionally terminate RX decode to
    /// avoid a hang during certain packet phases.
    #[inline(always)]
    pub const fn sie_rx_bitstuff_fix(&self) -> bool {
        (self.bits & LinestateTuning::SIE_RX_BITSTUFF_FIX_MASK) != 0
    }

    /// Read **DEV_BUFF_CONTROL_DOUBLE_READ_FIX** `[3:3]`
    ///
    /// Device - the controller FSM performs two reads of the buffer status memory address to
    /// avoid sampling metastable data. An enabled buffer is only used if both reads match.
    #[inline(always)]
    pub const fn dev_buff_control_double_read_fix(&self) -> bool {
        (self.bits & LinestateTuning::DEV_BUFF_CONTROL_DOUBLE_READ_FIX_MASK) != 0
    }

    /// Read **MULTI_HUB_FIX** `[2:2]`
    ///
    /// Host - increase inter-packet and turnaround timeouts to accommodate worst-case hub delays.
    #[inline(always)]
    pub const fn multi_hub_fix(&self) -> bool {
        (self.bits & LinestateTuning::MULTI_HUB_FIX_MASK) != 0
    }

    /// Read **LINESTATE_DELAY** `[1:1]`
    ///
    /// Device/Host - add an extra 1-bit debounce of linestate sampling.
    #[inline(always)]
    pub const fn linestate_delay(&self) -> bool {
        (self.bits & LinestateTuning::LINESTATE_DELAY_MASK) != 0
    }

    /// Read **RCV_DELAY** `[0:0]`
    ///
    /// Device - register the received data to account for hub bit dribble before EOP. Only affects certain hubs.
    #[inline(always)]
    pub const fn rcv_delay(&self) -> bool {
        (self.bits & LinestateTuning::RCV_DELAY_MASK) != 0
    }

    /// Set **SPARE_FIX** `[11:8]`
    #[inline(always)]
    pub const fn with_spare_fix(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LinestateTuning::SPARE_FIX_MASK)
                | ((value << LinestateTuning::SPARE_FIX_OFFSET) & LinestateTuning::SPARE_FIX_MASK),
        }
    }

    /// Set **DEV_LS_WAKE_FIX** `[7:7]`
    ///
    /// Device - exit suspend on any non-idle signalling, not qualified with a 1ms timer
    #[inline(always)]
    pub const fn with_dev_ls_wake_fix(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LinestateTuning::DEV_LS_WAKE_FIX_MASK
            } else {
                self.bits & !LinestateTuning::DEV_LS_WAKE_FIX_MASK
            },
        }
    }

    /// Set **DEV_RX_ERR_QUIESCE** `[6:6]`
    ///
    /// Device - suppress repeated errors until the device FSM is next in the process of decoding an inbound packet.
    #[inline(always)]
    pub const fn with_dev_rx_err_quiesce(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LinestateTuning::DEV_RX_ERR_QUIESCE_MASK
            } else {
                self.bits & !LinestateTuning::DEV_RX_ERR_QUIESCE_MASK
            },
        }
    }

    /// Set **SIE_RX_CHATTER_SE0_FIX** `[5:5]`
    ///
    /// RX - when recovering from line chatter or bitstuff errors, treat SE0 as the end of chatter as well as
    /// 8 consecutive idle bits.
    #[inline(always)]
    pub const fn with_sie_rx_chatter_se0_fix(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LinestateTuning::SIE_RX_CHATTER_SE0_FIX_MASK
            } else {
                self.bits & !LinestateTuning::SIE_RX_CHATTER_SE0_FIX_MASK
            },
        }
    }

    /// Set **SIE_RX_BITSTUFF_FIX** `[4:4]`
    ///
    /// RX - when a bitstuff error is signalled by rx_dasm, unconditionally terminate RX decode to
    /// avoid a hang during certain packet phases.
    #[inline(always)]
    pub const fn with_sie_rx_bitstuff_fix(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LinestateTuning::SIE_RX_BITSTUFF_FIX_MASK
            } else {
                self.bits & !LinestateTuning::SIE_RX_BITSTUFF_FIX_MASK
            },
        }
    }

    /// Set **DEV_BUFF_CONTROL_DOUBLE_READ_FIX** `[3:3]`
    ///
    /// Device - the controller FSM performs two reads of the buffer status memory address to
    /// avoid sampling metastable data. An enabled buffer is only used if both reads match.
    #[inline(always)]
    pub const fn with_dev_buff_control_double_read_fix(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LinestateTuning::DEV_BUFF_CONTROL_DOUBLE_READ_FIX_MASK
            } else {
                self.bits & !LinestateTuning::DEV_BUFF_CONTROL_DOUBLE_READ_FIX_MASK
            },
        }
    }

    /// Set **MULTI_HUB_FIX** `[2:2]`
    ///
    /// Host - increase inter-packet and turnaround timeouts to accommodate worst-case hub delays.
    #[inline(always)]
    pub const fn with_multi_hub_fix(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LinestateTuning::MULTI_HUB_FIX_MASK
            } else {
                self.bits & !LinestateTuning::MULTI_HUB_FIX_MASK
            },
        }
    }

    /// Set **LINESTATE_DELAY** `[1:1]`
    ///
    /// Device/Host - add an extra 1-bit debounce of linestate sampling.
    #[inline(always)]
    pub const fn with_linestate_delay(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LinestateTuning::LINESTATE_DELAY_MASK
            } else {
                self.bits & !LinestateTuning::LINESTATE_DELAY_MASK
            },
        }
    }

    /// Set **RCV_DELAY** `[0:0]`
    ///
    /// Device - register the received data to account for hub bit dribble before EOP. Only affects certain hubs.
    #[inline(always)]
    pub const fn with_rcv_delay(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LinestateTuning::RCV_DELAY_MASK
            } else {
                self.bits & !LinestateTuning::RCV_DELAY_MASK
            },
        }
    }
}

impl Default for LinestateTuningVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl LinestateTuning {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LinestateTuningVal {
        LinestateTuningVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LinestateTuningVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LinestateTuningVal) -> LinestateTuningVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// INTR Register
///
/// Raw Interrupts
///
/// Address offset: 0x008c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **EPX_STOPPED_ON_NAK** `[23]` (RO)
///
///   Source: NAK_POLL.EPX_STOPPED_ON_NAK
///
/// - **DEV_SM_WATCHDOG_FIRED** `[22]` (RO)
///
///   Source: DEV_SM_WATCHDOG.FIRED
///
/// - **ENDPOINT_ERROR** `[21]` (RO)
///
///   Source: SIE_STATUS.ENDPOINT_ERROR
///
/// - **RX_SHORT_PACKET** `[20]` (RO)
///
///   Source: SIE_STATUS.RX_SHORT_PACKET
///
/// - **EP_STALL_NAK** `[19]` (RO)
///
///   Raised when any bit in EP_STATUS_STALL_NAK is set. Clear by clearing all bits in EP_STATUS_STALL_NAK.
///
/// - **ABORT_DONE** `[18]` (RO)
///
///   Raised when any bit in ABORT_DONE is set. Clear by clearing all bits in ABORT_DONE.
///
/// - **DEV_SOF** `[17]` (RO)
///
///   Set every time the device receives a SOF (Start of Frame) packet. Cleared by reading SOF_RD
///
/// - **SETUP_REQ** `[16]` (RO)
///
///   Device. Source: SIE_STATUS.SETUP_REC
///
/// - **DEV_RESUME_FROM_HOST** `[15]` (RO)
///
///   Set when the device receives a resume from the host. Cleared by writing to SIE_STATUS.RESUME
///
/// - **DEV_SUSPEND** `[14]` (RO)
///
///   Set when the device suspend state changes. Cleared by writing to SIE_STATUS.SUSPENDED
///
/// - **DEV_CONN_DIS** `[13]` (RO)
///
///   Set when the device connection state changes. Cleared by writing to SIE_STATUS.CONNECTED
///
/// - **BUS_RESET** `[12]` (RO)
///
///   Source: SIE_STATUS.BUS_RESET
///
/// - **VBUS_DETECT** `[11]` (RO)
///
///   Source: SIE_STATUS.VBUS_DETECTED
///
/// - **STALL** `[10]` (RO)
///
///   Source: SIE_STATUS.STALL_REC
///
/// - **ERROR_CRC** `[9]` (RO)
///
///   Source: SIE_STATUS.CRC_ERROR
///
/// - **ERROR_BIT_STUFF** `[8]` (RO)
///
///   Source: SIE_STATUS.BIT_STUFF_ERROR
///
/// - **ERROR_RX_OVERFLOW** `[7]` (RO)
///
///   Source: SIE_STATUS.RX_OVERFLOW
///
/// - **ERROR_RX_TIMEOUT** `[6]` (RO)
///
///   Source: SIE_STATUS.RX_TIMEOUT
///
/// - **ERROR_DATA_SEQ** `[5]` (RO)
///
///   Source: SIE_STATUS.DATA_SEQ_ERROR
///
/// - **BUFF_STATUS** `[4]` (RO)
///
///   Raised when any bit in BUFF_STATUS is set. Clear by clearing all bits in BUFF_STATUS.
///
/// - **TRANS_COMPLETE** `[3]` (RO)
///
///   Raised every time SIE_STATUS.TRANS_COMPLETE is set. Clear by writing to this bit.
///
/// - **HOST_SOF** `[2]` (RO)
///
///   Host: raised every time the host sends a SOF (Start of Frame). Cleared by reading SOF_RD
///
/// - **HOST_RESUME** `[1]` (RO)
///
///   Host: raised when a device wakes up the host. Cleared by writing to SIE_STATUS.RESUME
///
/// - **HOST_CONN_DIS** `[0]` (RO)
///
///   Host: raised when a device is connected or disconnected (i.e. when SIE_STATUS.SPEED changes). Cleared by writing to SIE_STATUS.SPEED
#[repr(transparent)]
pub struct Intr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Intr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Intr {
    // ======== Field Constants ========

    /// **EPX_STOPPED_ON_NAK**
    ///
    /// Source: NAK_POLL.EPX_STOPPED_ON_NAK
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const EPX_STOPPED_ON_NAK_OFFSET: u32 = 23;
    pub const EPX_STOPPED_ON_NAK_MASK: u32 = 0x00800000;
    pub const EPX_STOPPED_ON_NAK: u32 = 1 << 23;

    /// **DEV_SM_WATCHDOG_FIRED**
    ///
    /// Source: DEV_SM_WATCHDOG.FIRED
    ///
    /// Bits `[22:22]`, mask `0x00400000`
    pub const DEV_SM_WATCHDOG_FIRED_OFFSET: u32 = 22;
    pub const DEV_SM_WATCHDOG_FIRED_MASK: u32 = 0x00400000;
    pub const DEV_SM_WATCHDOG_FIRED: u32 = 1 << 22;

    /// **ENDPOINT_ERROR**
    ///
    /// Source: SIE_STATUS.ENDPOINT_ERROR
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const ENDPOINT_ERROR_OFFSET: u32 = 21;
    pub const ENDPOINT_ERROR_MASK: u32 = 0x00200000;
    pub const ENDPOINT_ERROR: u32 = 1 << 21;

    /// **RX_SHORT_PACKET**
    ///
    /// Source: SIE_STATUS.RX_SHORT_PACKET
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const RX_SHORT_PACKET_OFFSET: u32 = 20;
    pub const RX_SHORT_PACKET_MASK: u32 = 0x00100000;
    pub const RX_SHORT_PACKET: u32 = 1 << 20;

    /// **EP_STALL_NAK**
    ///
    /// Raised when any bit in EP_STATUS_STALL_NAK is set. Clear by clearing all bits in EP_STATUS_STALL_NAK.
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const EP_STALL_NAK_OFFSET: u32 = 19;
    pub const EP_STALL_NAK_MASK: u32 = 0x00080000;
    pub const EP_STALL_NAK: u32 = 1 << 19;

    /// **ABORT_DONE**
    ///
    /// Raised when any bit in ABORT_DONE is set. Clear by clearing all bits in ABORT_DONE.
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const ABORT_DONE_OFFSET: u32 = 18;
    pub const ABORT_DONE_MASK: u32 = 0x00040000;
    pub const ABORT_DONE: u32 = 1 << 18;

    /// **DEV_SOF**
    ///
    /// Set every time the device receives a SOF (Start of Frame) packet. Cleared by reading SOF_RD
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const DEV_SOF_OFFSET: u32 = 17;
    pub const DEV_SOF_MASK: u32 = 0x00020000;
    pub const DEV_SOF: u32 = 1 << 17;

    /// **SETUP_REQ**
    ///
    /// Device. Source: SIE_STATUS.SETUP_REC
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const SETUP_REQ_OFFSET: u32 = 16;
    pub const SETUP_REQ_MASK: u32 = 0x00010000;
    pub const SETUP_REQ: u32 = 1 << 16;

    /// **DEV_RESUME_FROM_HOST**
    ///
    /// Set when the device receives a resume from the host. Cleared by writing to SIE_STATUS.RESUME
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const DEV_RESUME_FROM_HOST_OFFSET: u32 = 15;
    pub const DEV_RESUME_FROM_HOST_MASK: u32 = 0x00008000;
    pub const DEV_RESUME_FROM_HOST: u32 = 1 << 15;

    /// **DEV_SUSPEND**
    ///
    /// Set when the device suspend state changes. Cleared by writing to SIE_STATUS.SUSPENDED
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const DEV_SUSPEND_OFFSET: u32 = 14;
    pub const DEV_SUSPEND_MASK: u32 = 0x00004000;
    pub const DEV_SUSPEND: u32 = 1 << 14;

    /// **DEV_CONN_DIS**
    ///
    /// Set when the device connection state changes. Cleared by writing to SIE_STATUS.CONNECTED
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const DEV_CONN_DIS_OFFSET: u32 = 13;
    pub const DEV_CONN_DIS_MASK: u32 = 0x00002000;
    pub const DEV_CONN_DIS: u32 = 1 << 13;

    /// **BUS_RESET**
    ///
    /// Source: SIE_STATUS.BUS_RESET
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const BUS_RESET_OFFSET: u32 = 12;
    pub const BUS_RESET_MASK: u32 = 0x00001000;
    pub const BUS_RESET: u32 = 1 << 12;

    /// **VBUS_DETECT**
    ///
    /// Source: SIE_STATUS.VBUS_DETECTED
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const VBUS_DETECT_OFFSET: u32 = 11;
    pub const VBUS_DETECT_MASK: u32 = 0x00000800;
    pub const VBUS_DETECT: u32 = 1 << 11;

    /// **STALL**
    ///
    /// Source: SIE_STATUS.STALL_REC
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const STALL_OFFSET: u32 = 10;
    pub const STALL_MASK: u32 = 0x00000400;
    pub const STALL: u32 = 1 << 10;

    /// **ERROR_CRC**
    ///
    /// Source: SIE_STATUS.CRC_ERROR
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const ERROR_CRC_OFFSET: u32 = 9;
    pub const ERROR_CRC_MASK: u32 = 0x00000200;
    pub const ERROR_CRC: u32 = 1 << 9;

    /// **ERROR_BIT_STUFF**
    ///
    /// Source: SIE_STATUS.BIT_STUFF_ERROR
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ERROR_BIT_STUFF_OFFSET: u32 = 8;
    pub const ERROR_BIT_STUFF_MASK: u32 = 0x00000100;
    pub const ERROR_BIT_STUFF: u32 = 1 << 8;

    /// **ERROR_RX_OVERFLOW**
    ///
    /// Source: SIE_STATUS.RX_OVERFLOW
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const ERROR_RX_OVERFLOW_OFFSET: u32 = 7;
    pub const ERROR_RX_OVERFLOW_MASK: u32 = 0x00000080;
    pub const ERROR_RX_OVERFLOW: u32 = 1 << 7;

    /// **ERROR_RX_TIMEOUT**
    ///
    /// Source: SIE_STATUS.RX_TIMEOUT
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const ERROR_RX_TIMEOUT_OFFSET: u32 = 6;
    pub const ERROR_RX_TIMEOUT_MASK: u32 = 0x00000040;
    pub const ERROR_RX_TIMEOUT: u32 = 1 << 6;

    /// **ERROR_DATA_SEQ**
    ///
    /// Source: SIE_STATUS.DATA_SEQ_ERROR
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const ERROR_DATA_SEQ_OFFSET: u32 = 5;
    pub const ERROR_DATA_SEQ_MASK: u32 = 0x00000020;
    pub const ERROR_DATA_SEQ: u32 = 1 << 5;

    /// **BUFF_STATUS**
    ///
    /// Raised when any bit in BUFF_STATUS is set. Clear by clearing all bits in BUFF_STATUS.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const BUFF_STATUS_OFFSET: u32 = 4;
    pub const BUFF_STATUS_MASK: u32 = 0x00000010;
    pub const BUFF_STATUS: u32 = 1 << 4;

    /// **TRANS_COMPLETE**
    ///
    /// Raised every time SIE_STATUS.TRANS_COMPLETE is set. Clear by writing to this bit.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const TRANS_COMPLETE_OFFSET: u32 = 3;
    pub const TRANS_COMPLETE_MASK: u32 = 0x00000008;
    pub const TRANS_COMPLETE: u32 = 1 << 3;

    /// **HOST_SOF**
    ///
    /// Host: raised every time the host sends a SOF (Start of Frame). Cleared by reading SOF_RD
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const HOST_SOF_OFFSET: u32 = 2;
    pub const HOST_SOF_MASK: u32 = 0x00000004;
    pub const HOST_SOF: u32 = 1 << 2;

    /// **HOST_RESUME**
    ///
    /// Host: raised when a device wakes up the host. Cleared by writing to SIE_STATUS.RESUME
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const HOST_RESUME_OFFSET: u32 = 1;
    pub const HOST_RESUME_MASK: u32 = 0x00000002;
    pub const HOST_RESUME: u32 = 1 << 1;

    /// **HOST_CONN_DIS**
    ///
    /// Host: raised when a device is connected or disconnected (i.e. when SIE_STATUS.SPEED changes). Cleared by writing to SIE_STATUS.SPEED
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const HOST_CONN_DIS_OFFSET: u32 = 0;
    pub const HOST_CONN_DIS_MASK: u32 = 0x00000001;
    pub const HOST_CONN_DIS: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **EPX_STOPPED_ON_NAK** `[23]` (RO)
    ///
    /// Source: NAK_POLL.EPX_STOPPED_ON_NAK
    #[inline(always)]
    pub fn epx_stopped_on_nak(&self) -> bool {
        (self.value.read_bits() & Self::EPX_STOPPED_ON_NAK_MASK) != 0
    }

    /// **DEV_SM_WATCHDOG_FIRED** `[22]` (RO)
    ///
    /// Source: DEV_SM_WATCHDOG.FIRED
    #[inline(always)]
    pub fn dev_sm_watchdog_fired(&self) -> bool {
        (self.value.read_bits() & Self::DEV_SM_WATCHDOG_FIRED_MASK) != 0
    }

    /// **ENDPOINT_ERROR** `[21]` (RO)
    ///
    /// Source: SIE_STATUS.ENDPOINT_ERROR
    #[inline(always)]
    pub fn endpoint_error(&self) -> bool {
        (self.value.read_bits() & Self::ENDPOINT_ERROR_MASK) != 0
    }

    /// **RX_SHORT_PACKET** `[20]` (RO)
    ///
    /// Source: SIE_STATUS.RX_SHORT_PACKET
    #[inline(always)]
    pub fn rx_short_packet(&self) -> bool {
        (self.value.read_bits() & Self::RX_SHORT_PACKET_MASK) != 0
    }

    /// **EP_STALL_NAK** `[19]` (RO)
    ///
    /// Raised when any bit in EP_STATUS_STALL_NAK is set. Clear by clearing all bits in EP_STATUS_STALL_NAK.
    #[inline(always)]
    pub fn ep_stall_nak(&self) -> bool {
        (self.value.read_bits() & Self::EP_STALL_NAK_MASK) != 0
    }

    /// **ABORT_DONE** `[18]` (RO)
    ///
    /// Raised when any bit in ABORT_DONE is set. Clear by clearing all bits in ABORT_DONE.
    #[inline(always)]
    pub fn abort_done(&self) -> bool {
        (self.value.read_bits() & Self::ABORT_DONE_MASK) != 0
    }

    /// **DEV_SOF** `[17]` (RO)
    ///
    /// Set every time the device receives a SOF (Start of Frame) packet. Cleared by reading SOF_RD
    #[inline(always)]
    pub fn dev_sof(&self) -> bool {
        (self.value.read_bits() & Self::DEV_SOF_MASK) != 0
    }

    /// **SETUP_REQ** `[16]` (RO)
    ///
    /// Device. Source: SIE_STATUS.SETUP_REC
    #[inline(always)]
    pub fn setup_req(&self) -> bool {
        (self.value.read_bits() & Self::SETUP_REQ_MASK) != 0
    }

    /// **DEV_RESUME_FROM_HOST** `[15]` (RO)
    ///
    /// Set when the device receives a resume from the host. Cleared by writing to SIE_STATUS.RESUME
    #[inline(always)]
    pub fn dev_resume_from_host(&self) -> bool {
        (self.value.read_bits() & Self::DEV_RESUME_FROM_HOST_MASK) != 0
    }

    /// **DEV_SUSPEND** `[14]` (RO)
    ///
    /// Set when the device suspend state changes. Cleared by writing to SIE_STATUS.SUSPENDED
    #[inline(always)]
    pub fn dev_suspend(&self) -> bool {
        (self.value.read_bits() & Self::DEV_SUSPEND_MASK) != 0
    }

    /// **DEV_CONN_DIS** `[13]` (RO)
    ///
    /// Set when the device connection state changes. Cleared by writing to SIE_STATUS.CONNECTED
    #[inline(always)]
    pub fn dev_conn_dis(&self) -> bool {
        (self.value.read_bits() & Self::DEV_CONN_DIS_MASK) != 0
    }

    /// **BUS_RESET** `[12]` (RO)
    ///
    /// Source: SIE_STATUS.BUS_RESET
    #[inline(always)]
    pub fn bus_reset(&self) -> bool {
        (self.value.read_bits() & Self::BUS_RESET_MASK) != 0
    }

    /// **VBUS_DETECT** `[11]` (RO)
    ///
    /// Source: SIE_STATUS.VBUS_DETECTED
    #[inline(always)]
    pub fn vbus_detect(&self) -> bool {
        (self.value.read_bits() & Self::VBUS_DETECT_MASK) != 0
    }

    /// **STALL** `[10]` (RO)
    ///
    /// Source: SIE_STATUS.STALL_REC
    #[inline(always)]
    pub fn stall(&self) -> bool {
        (self.value.read_bits() & Self::STALL_MASK) != 0
    }

    /// **ERROR_CRC** `[9]` (RO)
    ///
    /// Source: SIE_STATUS.CRC_ERROR
    #[inline(always)]
    pub fn error_crc(&self) -> bool {
        (self.value.read_bits() & Self::ERROR_CRC_MASK) != 0
    }

    /// **ERROR_BIT_STUFF** `[8]` (RO)
    ///
    /// Source: SIE_STATUS.BIT_STUFF_ERROR
    #[inline(always)]
    pub fn error_bit_stuff(&self) -> bool {
        (self.value.read_bits() & Self::ERROR_BIT_STUFF_MASK) != 0
    }

    /// **ERROR_RX_OVERFLOW** `[7]` (RO)
    ///
    /// Source: SIE_STATUS.RX_OVERFLOW
    #[inline(always)]
    pub fn error_rx_overflow(&self) -> bool {
        (self.value.read_bits() & Self::ERROR_RX_OVERFLOW_MASK) != 0
    }

    /// **ERROR_RX_TIMEOUT** `[6]` (RO)
    ///
    /// Source: SIE_STATUS.RX_TIMEOUT
    #[inline(always)]
    pub fn error_rx_timeout(&self) -> bool {
        (self.value.read_bits() & Self::ERROR_RX_TIMEOUT_MASK) != 0
    }

    /// **ERROR_DATA_SEQ** `[5]` (RO)
    ///
    /// Source: SIE_STATUS.DATA_SEQ_ERROR
    #[inline(always)]
    pub fn error_data_seq(&self) -> bool {
        (self.value.read_bits() & Self::ERROR_DATA_SEQ_MASK) != 0
    }

    /// **BUFF_STATUS** `[4]` (RO)
    ///
    /// Raised when any bit in BUFF_STATUS is set. Clear by clearing all bits in BUFF_STATUS.
    #[inline(always)]
    pub fn buff_status(&self) -> bool {
        (self.value.read_bits() & Self::BUFF_STATUS_MASK) != 0
    }

    /// **TRANS_COMPLETE** `[3]` (RO)
    ///
    /// Raised every time SIE_STATUS.TRANS_COMPLETE is set. Clear by writing to this bit.
    #[inline(always)]
    pub fn trans_complete(&self) -> bool {
        (self.value.read_bits() & Self::TRANS_COMPLETE_MASK) != 0
    }

    /// **HOST_SOF** `[2]` (RO)
    ///
    /// Host: raised every time the host sends a SOF (Start of Frame). Cleared by reading SOF_RD
    #[inline(always)]
    pub fn host_sof(&self) -> bool {
        (self.value.read_bits() & Self::HOST_SOF_MASK) != 0
    }

    /// **HOST_RESUME** `[1]` (RO)
    ///
    /// Host: raised when a device wakes up the host. Cleared by writing to SIE_STATUS.RESUME
    #[inline(always)]
    pub fn host_resume(&self) -> bool {
        (self.value.read_bits() & Self::HOST_RESUME_MASK) != 0
    }

    /// **HOST_CONN_DIS** `[0]` (RO)
    ///
    /// Host: raised when a device is connected or disconnected (i.e. when SIE_STATUS.SPEED changes). Cleared by writing to SIE_STATUS.SPEED
    #[inline(always)]
    pub fn host_conn_dis(&self) -> bool {
        (self.value.read_bits() & Self::HOST_CONN_DIS_MASK) != 0
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

    /// Read **EPX_STOPPED_ON_NAK** `[23:23]`
    ///
    /// Source: NAK_POLL.EPX_STOPPED_ON_NAK
    #[inline(always)]
    pub const fn epx_stopped_on_nak(&self) -> bool {
        (self.bits & Intr::EPX_STOPPED_ON_NAK_MASK) != 0
    }

    /// Read **DEV_SM_WATCHDOG_FIRED** `[22:22]`
    ///
    /// Source: DEV_SM_WATCHDOG.FIRED
    #[inline(always)]
    pub const fn dev_sm_watchdog_fired(&self) -> bool {
        (self.bits & Intr::DEV_SM_WATCHDOG_FIRED_MASK) != 0
    }

    /// Read **ENDPOINT_ERROR** `[21:21]`
    ///
    /// Source: SIE_STATUS.ENDPOINT_ERROR
    #[inline(always)]
    pub const fn endpoint_error(&self) -> bool {
        (self.bits & Intr::ENDPOINT_ERROR_MASK) != 0
    }

    /// Read **RX_SHORT_PACKET** `[20:20]`
    ///
    /// Source: SIE_STATUS.RX_SHORT_PACKET
    #[inline(always)]
    pub const fn rx_short_packet(&self) -> bool {
        (self.bits & Intr::RX_SHORT_PACKET_MASK) != 0
    }

    /// Read **EP_STALL_NAK** `[19:19]`
    ///
    /// Raised when any bit in EP_STATUS_STALL_NAK is set. Clear by clearing all bits in EP_STATUS_STALL_NAK.
    #[inline(always)]
    pub const fn ep_stall_nak(&self) -> bool {
        (self.bits & Intr::EP_STALL_NAK_MASK) != 0
    }

    /// Read **ABORT_DONE** `[18:18]`
    ///
    /// Raised when any bit in ABORT_DONE is set. Clear by clearing all bits in ABORT_DONE.
    #[inline(always)]
    pub const fn abort_done(&self) -> bool {
        (self.bits & Intr::ABORT_DONE_MASK) != 0
    }

    /// Read **DEV_SOF** `[17:17]`
    ///
    /// Set every time the device receives a SOF (Start of Frame) packet. Cleared by reading SOF_RD
    #[inline(always)]
    pub const fn dev_sof(&self) -> bool {
        (self.bits & Intr::DEV_SOF_MASK) != 0
    }

    /// Read **SETUP_REQ** `[16:16]`
    ///
    /// Device. Source: SIE_STATUS.SETUP_REC
    #[inline(always)]
    pub const fn setup_req(&self) -> bool {
        (self.bits & Intr::SETUP_REQ_MASK) != 0
    }

    /// Read **DEV_RESUME_FROM_HOST** `[15:15]`
    ///
    /// Set when the device receives a resume from the host. Cleared by writing to SIE_STATUS.RESUME
    #[inline(always)]
    pub const fn dev_resume_from_host(&self) -> bool {
        (self.bits & Intr::DEV_RESUME_FROM_HOST_MASK) != 0
    }

    /// Read **DEV_SUSPEND** `[14:14]`
    ///
    /// Set when the device suspend state changes. Cleared by writing to SIE_STATUS.SUSPENDED
    #[inline(always)]
    pub const fn dev_suspend(&self) -> bool {
        (self.bits & Intr::DEV_SUSPEND_MASK) != 0
    }

    /// Read **DEV_CONN_DIS** `[13:13]`
    ///
    /// Set when the device connection state changes. Cleared by writing to SIE_STATUS.CONNECTED
    #[inline(always)]
    pub const fn dev_conn_dis(&self) -> bool {
        (self.bits & Intr::DEV_CONN_DIS_MASK) != 0
    }

    /// Read **BUS_RESET** `[12:12]`
    ///
    /// Source: SIE_STATUS.BUS_RESET
    #[inline(always)]
    pub const fn bus_reset(&self) -> bool {
        (self.bits & Intr::BUS_RESET_MASK) != 0
    }

    /// Read **VBUS_DETECT** `[11:11]`
    ///
    /// Source: SIE_STATUS.VBUS_DETECTED
    #[inline(always)]
    pub const fn vbus_detect(&self) -> bool {
        (self.bits & Intr::VBUS_DETECT_MASK) != 0
    }

    /// Read **STALL** `[10:10]`
    ///
    /// Source: SIE_STATUS.STALL_REC
    #[inline(always)]
    pub const fn stall(&self) -> bool {
        (self.bits & Intr::STALL_MASK) != 0
    }

    /// Read **ERROR_CRC** `[9:9]`
    ///
    /// Source: SIE_STATUS.CRC_ERROR
    #[inline(always)]
    pub const fn error_crc(&self) -> bool {
        (self.bits & Intr::ERROR_CRC_MASK) != 0
    }

    /// Read **ERROR_BIT_STUFF** `[8:8]`
    ///
    /// Source: SIE_STATUS.BIT_STUFF_ERROR
    #[inline(always)]
    pub const fn error_bit_stuff(&self) -> bool {
        (self.bits & Intr::ERROR_BIT_STUFF_MASK) != 0
    }

    /// Read **ERROR_RX_OVERFLOW** `[7:7]`
    ///
    /// Source: SIE_STATUS.RX_OVERFLOW
    #[inline(always)]
    pub const fn error_rx_overflow(&self) -> bool {
        (self.bits & Intr::ERROR_RX_OVERFLOW_MASK) != 0
    }

    /// Read **ERROR_RX_TIMEOUT** `[6:6]`
    ///
    /// Source: SIE_STATUS.RX_TIMEOUT
    #[inline(always)]
    pub const fn error_rx_timeout(&self) -> bool {
        (self.bits & Intr::ERROR_RX_TIMEOUT_MASK) != 0
    }

    /// Read **ERROR_DATA_SEQ** `[5:5]`
    ///
    /// Source: SIE_STATUS.DATA_SEQ_ERROR
    #[inline(always)]
    pub const fn error_data_seq(&self) -> bool {
        (self.bits & Intr::ERROR_DATA_SEQ_MASK) != 0
    }

    /// Read **BUFF_STATUS** `[4:4]`
    ///
    /// Raised when any bit in BUFF_STATUS is set. Clear by clearing all bits in BUFF_STATUS.
    #[inline(always)]
    pub const fn buff_status(&self) -> bool {
        (self.bits & Intr::BUFF_STATUS_MASK) != 0
    }

    /// Read **TRANS_COMPLETE** `[3:3]`
    ///
    /// Raised every time SIE_STATUS.TRANS_COMPLETE is set. Clear by writing to this bit.
    #[inline(always)]
    pub const fn trans_complete(&self) -> bool {
        (self.bits & Intr::TRANS_COMPLETE_MASK) != 0
    }

    /// Read **HOST_SOF** `[2:2]`
    ///
    /// Host: raised every time the host sends a SOF (Start of Frame). Cleared by reading SOF_RD
    #[inline(always)]
    pub const fn host_sof(&self) -> bool {
        (self.bits & Intr::HOST_SOF_MASK) != 0
    }

    /// Read **HOST_RESUME** `[1:1]`
    ///
    /// Host: raised when a device wakes up the host. Cleared by writing to SIE_STATUS.RESUME
    #[inline(always)]
    pub const fn host_resume(&self) -> bool {
        (self.bits & Intr::HOST_RESUME_MASK) != 0
    }

    /// Read **HOST_CONN_DIS** `[0:0]`
    ///
    /// Host: raised when a device is connected or disconnected (i.e. when SIE_STATUS.SPEED changes). Cleared by writing to SIE_STATUS.SPEED
    #[inline(always)]
    pub const fn host_conn_dis(&self) -> bool {
        (self.bits & Intr::HOST_CONN_DIS_MASK) != 0
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
/// Address offset: 0x0090
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **EPX_STOPPED_ON_NAK** `[23]` (RW)
///
///   Source: NAK_POLL.EPX_STOPPED_ON_NAK
///
/// - **DEV_SM_WATCHDOG_FIRED** `[22]` (RW)
///
///   Source: DEV_SM_WATCHDOG.FIRED
///
/// - **ENDPOINT_ERROR** `[21]` (RW)
///
///   Source: SIE_STATUS.ENDPOINT_ERROR
///
/// - **RX_SHORT_PACKET** `[20]` (RW)
///
///   Source: SIE_STATUS.RX_SHORT_PACKET
///
/// - **EP_STALL_NAK** `[19]` (RW)
///
///   Raised when any bit in EP_STATUS_STALL_NAK is set. Clear by clearing all bits in EP_STATUS_STALL_NAK.
///
/// - **ABORT_DONE** `[18]` (RW)
///
///   Raised when any bit in ABORT_DONE is set. Clear by clearing all bits in ABORT_DONE.
///
/// - **DEV_SOF** `[17]` (RW)
///
///   Set every time the device receives a SOF (Start of Frame) packet. Cleared by reading SOF_RD
///
/// - **SETUP_REQ** `[16]` (RW)
///
///   Device. Source: SIE_STATUS.SETUP_REC
///
/// - **DEV_RESUME_FROM_HOST** `[15]` (RW)
///
///   Set when the device receives a resume from the host. Cleared by writing to SIE_STATUS.RESUME
///
/// - **DEV_SUSPEND** `[14]` (RW)
///
///   Set when the device suspend state changes. Cleared by writing to SIE_STATUS.SUSPENDED
///
/// - **DEV_CONN_DIS** `[13]` (RW)
///
///   Set when the device connection state changes. Cleared by writing to SIE_STATUS.CONNECTED
///
/// - **BUS_RESET** `[12]` (RW)
///
///   Source: SIE_STATUS.BUS_RESET
///
/// - **VBUS_DETECT** `[11]` (RW)
///
///   Source: SIE_STATUS.VBUS_DETECTED
///
/// - **STALL** `[10]` (RW)
///
///   Source: SIE_STATUS.STALL_REC
///
/// - **ERROR_CRC** `[9]` (RW)
///
///   Source: SIE_STATUS.CRC_ERROR
///
/// - **ERROR_BIT_STUFF** `[8]` (RW)
///
///   Source: SIE_STATUS.BIT_STUFF_ERROR
///
/// - **ERROR_RX_OVERFLOW** `[7]` (RW)
///
///   Source: SIE_STATUS.RX_OVERFLOW
///
/// - **ERROR_RX_TIMEOUT** `[6]` (RW)
///
///   Source: SIE_STATUS.RX_TIMEOUT
///
/// - **ERROR_DATA_SEQ** `[5]` (RW)
///
///   Source: SIE_STATUS.DATA_SEQ_ERROR
///
/// - **BUFF_STATUS** `[4]` (RW)
///
///   Raised when any bit in BUFF_STATUS is set. Clear by clearing all bits in BUFF_STATUS.
///
/// - **TRANS_COMPLETE** `[3]` (RW)
///
///   Raised every time SIE_STATUS.TRANS_COMPLETE is set. Clear by writing to this bit.
///
/// - **HOST_SOF** `[2]` (RW)
///
///   Host: raised every time the host sends a SOF (Start of Frame). Cleared by reading SOF_RD
///
/// - **HOST_RESUME** `[1]` (RW)
///
///   Host: raised when a device wakes up the host. Cleared by writing to SIE_STATUS.RESUME
///
/// - **HOST_CONN_DIS** `[0]` (RW)
///
///   Host: raised when a device is connected or disconnected (i.e. when SIE_STATUS.SPEED changes). Cleared by writing to SIE_STATUS.SPEED
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

    /// **EPX_STOPPED_ON_NAK**
    ///
    /// Source: NAK_POLL.EPX_STOPPED_ON_NAK
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const EPX_STOPPED_ON_NAK_OFFSET: u32 = 23;
    pub const EPX_STOPPED_ON_NAK_MASK: u32 = 0x00800000;
    pub const EPX_STOPPED_ON_NAK: u32 = 1 << 23;

    /// **DEV_SM_WATCHDOG_FIRED**
    ///
    /// Source: DEV_SM_WATCHDOG.FIRED
    ///
    /// Bits `[22:22]`, mask `0x00400000`
    pub const DEV_SM_WATCHDOG_FIRED_OFFSET: u32 = 22;
    pub const DEV_SM_WATCHDOG_FIRED_MASK: u32 = 0x00400000;
    pub const DEV_SM_WATCHDOG_FIRED: u32 = 1 << 22;

    /// **ENDPOINT_ERROR**
    ///
    /// Source: SIE_STATUS.ENDPOINT_ERROR
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const ENDPOINT_ERROR_OFFSET: u32 = 21;
    pub const ENDPOINT_ERROR_MASK: u32 = 0x00200000;
    pub const ENDPOINT_ERROR: u32 = 1 << 21;

    /// **RX_SHORT_PACKET**
    ///
    /// Source: SIE_STATUS.RX_SHORT_PACKET
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const RX_SHORT_PACKET_OFFSET: u32 = 20;
    pub const RX_SHORT_PACKET_MASK: u32 = 0x00100000;
    pub const RX_SHORT_PACKET: u32 = 1 << 20;

    /// **EP_STALL_NAK**
    ///
    /// Raised when any bit in EP_STATUS_STALL_NAK is set. Clear by clearing all bits in EP_STATUS_STALL_NAK.
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const EP_STALL_NAK_OFFSET: u32 = 19;
    pub const EP_STALL_NAK_MASK: u32 = 0x00080000;
    pub const EP_STALL_NAK: u32 = 1 << 19;

    /// **ABORT_DONE**
    ///
    /// Raised when any bit in ABORT_DONE is set. Clear by clearing all bits in ABORT_DONE.
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const ABORT_DONE_OFFSET: u32 = 18;
    pub const ABORT_DONE_MASK: u32 = 0x00040000;
    pub const ABORT_DONE: u32 = 1 << 18;

    /// **DEV_SOF**
    ///
    /// Set every time the device receives a SOF (Start of Frame) packet. Cleared by reading SOF_RD
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const DEV_SOF_OFFSET: u32 = 17;
    pub const DEV_SOF_MASK: u32 = 0x00020000;
    pub const DEV_SOF: u32 = 1 << 17;

    /// **SETUP_REQ**
    ///
    /// Device. Source: SIE_STATUS.SETUP_REC
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const SETUP_REQ_OFFSET: u32 = 16;
    pub const SETUP_REQ_MASK: u32 = 0x00010000;
    pub const SETUP_REQ: u32 = 1 << 16;

    /// **DEV_RESUME_FROM_HOST**
    ///
    /// Set when the device receives a resume from the host. Cleared by writing to SIE_STATUS.RESUME
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const DEV_RESUME_FROM_HOST_OFFSET: u32 = 15;
    pub const DEV_RESUME_FROM_HOST_MASK: u32 = 0x00008000;
    pub const DEV_RESUME_FROM_HOST: u32 = 1 << 15;

    /// **DEV_SUSPEND**
    ///
    /// Set when the device suspend state changes. Cleared by writing to SIE_STATUS.SUSPENDED
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const DEV_SUSPEND_OFFSET: u32 = 14;
    pub const DEV_SUSPEND_MASK: u32 = 0x00004000;
    pub const DEV_SUSPEND: u32 = 1 << 14;

    /// **DEV_CONN_DIS**
    ///
    /// Set when the device connection state changes. Cleared by writing to SIE_STATUS.CONNECTED
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const DEV_CONN_DIS_OFFSET: u32 = 13;
    pub const DEV_CONN_DIS_MASK: u32 = 0x00002000;
    pub const DEV_CONN_DIS: u32 = 1 << 13;

    /// **BUS_RESET**
    ///
    /// Source: SIE_STATUS.BUS_RESET
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const BUS_RESET_OFFSET: u32 = 12;
    pub const BUS_RESET_MASK: u32 = 0x00001000;
    pub const BUS_RESET: u32 = 1 << 12;

    /// **VBUS_DETECT**
    ///
    /// Source: SIE_STATUS.VBUS_DETECTED
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const VBUS_DETECT_OFFSET: u32 = 11;
    pub const VBUS_DETECT_MASK: u32 = 0x00000800;
    pub const VBUS_DETECT: u32 = 1 << 11;

    /// **STALL**
    ///
    /// Source: SIE_STATUS.STALL_REC
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const STALL_OFFSET: u32 = 10;
    pub const STALL_MASK: u32 = 0x00000400;
    pub const STALL: u32 = 1 << 10;

    /// **ERROR_CRC**
    ///
    /// Source: SIE_STATUS.CRC_ERROR
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const ERROR_CRC_OFFSET: u32 = 9;
    pub const ERROR_CRC_MASK: u32 = 0x00000200;
    pub const ERROR_CRC: u32 = 1 << 9;

    /// **ERROR_BIT_STUFF**
    ///
    /// Source: SIE_STATUS.BIT_STUFF_ERROR
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ERROR_BIT_STUFF_OFFSET: u32 = 8;
    pub const ERROR_BIT_STUFF_MASK: u32 = 0x00000100;
    pub const ERROR_BIT_STUFF: u32 = 1 << 8;

    /// **ERROR_RX_OVERFLOW**
    ///
    /// Source: SIE_STATUS.RX_OVERFLOW
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const ERROR_RX_OVERFLOW_OFFSET: u32 = 7;
    pub const ERROR_RX_OVERFLOW_MASK: u32 = 0x00000080;
    pub const ERROR_RX_OVERFLOW: u32 = 1 << 7;

    /// **ERROR_RX_TIMEOUT**
    ///
    /// Source: SIE_STATUS.RX_TIMEOUT
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const ERROR_RX_TIMEOUT_OFFSET: u32 = 6;
    pub const ERROR_RX_TIMEOUT_MASK: u32 = 0x00000040;
    pub const ERROR_RX_TIMEOUT: u32 = 1 << 6;

    /// **ERROR_DATA_SEQ**
    ///
    /// Source: SIE_STATUS.DATA_SEQ_ERROR
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const ERROR_DATA_SEQ_OFFSET: u32 = 5;
    pub const ERROR_DATA_SEQ_MASK: u32 = 0x00000020;
    pub const ERROR_DATA_SEQ: u32 = 1 << 5;

    /// **BUFF_STATUS**
    ///
    /// Raised when any bit in BUFF_STATUS is set. Clear by clearing all bits in BUFF_STATUS.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const BUFF_STATUS_OFFSET: u32 = 4;
    pub const BUFF_STATUS_MASK: u32 = 0x00000010;
    pub const BUFF_STATUS: u32 = 1 << 4;

    /// **TRANS_COMPLETE**
    ///
    /// Raised every time SIE_STATUS.TRANS_COMPLETE is set. Clear by writing to this bit.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const TRANS_COMPLETE_OFFSET: u32 = 3;
    pub const TRANS_COMPLETE_MASK: u32 = 0x00000008;
    pub const TRANS_COMPLETE: u32 = 1 << 3;

    /// **HOST_SOF**
    ///
    /// Host: raised every time the host sends a SOF (Start of Frame). Cleared by reading SOF_RD
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const HOST_SOF_OFFSET: u32 = 2;
    pub const HOST_SOF_MASK: u32 = 0x00000004;
    pub const HOST_SOF: u32 = 1 << 2;

    /// **HOST_RESUME**
    ///
    /// Host: raised when a device wakes up the host. Cleared by writing to SIE_STATUS.RESUME
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const HOST_RESUME_OFFSET: u32 = 1;
    pub const HOST_RESUME_MASK: u32 = 0x00000002;
    pub const HOST_RESUME: u32 = 1 << 1;

    /// **HOST_CONN_DIS**
    ///
    /// Host: raised when a device is connected or disconnected (i.e. when SIE_STATUS.SPEED changes). Cleared by writing to SIE_STATUS.SPEED
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const HOST_CONN_DIS_OFFSET: u32 = 0;
    pub const HOST_CONN_DIS_MASK: u32 = 0x00000001;
    pub const HOST_CONN_DIS: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **EPX_STOPPED_ON_NAK** `[23]` (RW)
    ///
    /// Source: NAK_POLL.EPX_STOPPED_ON_NAK
    #[inline(always)]
    pub fn epx_stopped_on_nak(&self) -> bool {
        (self.value.read_bits() & Self::EPX_STOPPED_ON_NAK_MASK) != 0
    }

    /// Set EPX_STOPPED_ON_NAK bit atomically (RW)
    #[inline(always)]
    pub fn set_epx_stopped_on_nak(&self) {
        self.set_bits(Self::EPX_STOPPED_ON_NAK_MASK);
    }

    /// Clear EPX_STOPPED_ON_NAK bit atomically (RW)
    #[inline(always)]
    pub fn clr_epx_stopped_on_nak(&self) {
        self.clr_bits(Self::EPX_STOPPED_ON_NAK_MASK);
    }

    /// **DEV_SM_WATCHDOG_FIRED** `[22]` (RW)
    ///
    /// Source: DEV_SM_WATCHDOG.FIRED
    #[inline(always)]
    pub fn dev_sm_watchdog_fired(&self) -> bool {
        (self.value.read_bits() & Self::DEV_SM_WATCHDOG_FIRED_MASK) != 0
    }

    /// Set DEV_SM_WATCHDOG_FIRED bit atomically (RW)
    #[inline(always)]
    pub fn set_dev_sm_watchdog_fired(&self) {
        self.set_bits(Self::DEV_SM_WATCHDOG_FIRED_MASK);
    }

    /// Clear DEV_SM_WATCHDOG_FIRED bit atomically (RW)
    #[inline(always)]
    pub fn clr_dev_sm_watchdog_fired(&self) {
        self.clr_bits(Self::DEV_SM_WATCHDOG_FIRED_MASK);
    }

    /// **ENDPOINT_ERROR** `[21]` (RW)
    ///
    /// Source: SIE_STATUS.ENDPOINT_ERROR
    #[inline(always)]
    pub fn endpoint_error(&self) -> bool {
        (self.value.read_bits() & Self::ENDPOINT_ERROR_MASK) != 0
    }

    /// Set ENDPOINT_ERROR bit atomically (RW)
    #[inline(always)]
    pub fn set_endpoint_error(&self) {
        self.set_bits(Self::ENDPOINT_ERROR_MASK);
    }

    /// Clear ENDPOINT_ERROR bit atomically (RW)
    #[inline(always)]
    pub fn clr_endpoint_error(&self) {
        self.clr_bits(Self::ENDPOINT_ERROR_MASK);
    }

    /// **RX_SHORT_PACKET** `[20]` (RW)
    ///
    /// Source: SIE_STATUS.RX_SHORT_PACKET
    #[inline(always)]
    pub fn rx_short_packet(&self) -> bool {
        (self.value.read_bits() & Self::RX_SHORT_PACKET_MASK) != 0
    }

    /// Set RX_SHORT_PACKET bit atomically (RW)
    #[inline(always)]
    pub fn set_rx_short_packet(&self) {
        self.set_bits(Self::RX_SHORT_PACKET_MASK);
    }

    /// Clear RX_SHORT_PACKET bit atomically (RW)
    #[inline(always)]
    pub fn clr_rx_short_packet(&self) {
        self.clr_bits(Self::RX_SHORT_PACKET_MASK);
    }

    /// **EP_STALL_NAK** `[19]` (RW)
    ///
    /// Raised when any bit in EP_STATUS_STALL_NAK is set. Clear by clearing all bits in EP_STATUS_STALL_NAK.
    #[inline(always)]
    pub fn ep_stall_nak(&self) -> bool {
        (self.value.read_bits() & Self::EP_STALL_NAK_MASK) != 0
    }

    /// Set EP_STALL_NAK bit atomically (RW)
    #[inline(always)]
    pub fn set_ep_stall_nak(&self) {
        self.set_bits(Self::EP_STALL_NAK_MASK);
    }

    /// Clear EP_STALL_NAK bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep_stall_nak(&self) {
        self.clr_bits(Self::EP_STALL_NAK_MASK);
    }

    /// **ABORT_DONE** `[18]` (RW)
    ///
    /// Raised when any bit in ABORT_DONE is set. Clear by clearing all bits in ABORT_DONE.
    #[inline(always)]
    pub fn abort_done(&self) -> bool {
        (self.value.read_bits() & Self::ABORT_DONE_MASK) != 0
    }

    /// Set ABORT_DONE bit atomically (RW)
    #[inline(always)]
    pub fn set_abort_done(&self) {
        self.set_bits(Self::ABORT_DONE_MASK);
    }

    /// Clear ABORT_DONE bit atomically (RW)
    #[inline(always)]
    pub fn clr_abort_done(&self) {
        self.clr_bits(Self::ABORT_DONE_MASK);
    }

    /// **DEV_SOF** `[17]` (RW)
    ///
    /// Set every time the device receives a SOF (Start of Frame) packet. Cleared by reading SOF_RD
    #[inline(always)]
    pub fn dev_sof(&self) -> bool {
        (self.value.read_bits() & Self::DEV_SOF_MASK) != 0
    }

    /// Set DEV_SOF bit atomically (RW)
    #[inline(always)]
    pub fn set_dev_sof(&self) {
        self.set_bits(Self::DEV_SOF_MASK);
    }

    /// Clear DEV_SOF bit atomically (RW)
    #[inline(always)]
    pub fn clr_dev_sof(&self) {
        self.clr_bits(Self::DEV_SOF_MASK);
    }

    /// **SETUP_REQ** `[16]` (RW)
    ///
    /// Device. Source: SIE_STATUS.SETUP_REC
    #[inline(always)]
    pub fn setup_req(&self) -> bool {
        (self.value.read_bits() & Self::SETUP_REQ_MASK) != 0
    }

    /// Set SETUP_REQ bit atomically (RW)
    #[inline(always)]
    pub fn set_setup_req(&self) {
        self.set_bits(Self::SETUP_REQ_MASK);
    }

    /// Clear SETUP_REQ bit atomically (RW)
    #[inline(always)]
    pub fn clr_setup_req(&self) {
        self.clr_bits(Self::SETUP_REQ_MASK);
    }

    /// **DEV_RESUME_FROM_HOST** `[15]` (RW)
    ///
    /// Set when the device receives a resume from the host. Cleared by writing to SIE_STATUS.RESUME
    #[inline(always)]
    pub fn dev_resume_from_host(&self) -> bool {
        (self.value.read_bits() & Self::DEV_RESUME_FROM_HOST_MASK) != 0
    }

    /// Set DEV_RESUME_FROM_HOST bit atomically (RW)
    #[inline(always)]
    pub fn set_dev_resume_from_host(&self) {
        self.set_bits(Self::DEV_RESUME_FROM_HOST_MASK);
    }

    /// Clear DEV_RESUME_FROM_HOST bit atomically (RW)
    #[inline(always)]
    pub fn clr_dev_resume_from_host(&self) {
        self.clr_bits(Self::DEV_RESUME_FROM_HOST_MASK);
    }

    /// **DEV_SUSPEND** `[14]` (RW)
    ///
    /// Set when the device suspend state changes. Cleared by writing to SIE_STATUS.SUSPENDED
    #[inline(always)]
    pub fn dev_suspend(&self) -> bool {
        (self.value.read_bits() & Self::DEV_SUSPEND_MASK) != 0
    }

    /// Set DEV_SUSPEND bit atomically (RW)
    #[inline(always)]
    pub fn set_dev_suspend(&self) {
        self.set_bits(Self::DEV_SUSPEND_MASK);
    }

    /// Clear DEV_SUSPEND bit atomically (RW)
    #[inline(always)]
    pub fn clr_dev_suspend(&self) {
        self.clr_bits(Self::DEV_SUSPEND_MASK);
    }

    /// **DEV_CONN_DIS** `[13]` (RW)
    ///
    /// Set when the device connection state changes. Cleared by writing to SIE_STATUS.CONNECTED
    #[inline(always)]
    pub fn dev_conn_dis(&self) -> bool {
        (self.value.read_bits() & Self::DEV_CONN_DIS_MASK) != 0
    }

    /// Set DEV_CONN_DIS bit atomically (RW)
    #[inline(always)]
    pub fn set_dev_conn_dis(&self) {
        self.set_bits(Self::DEV_CONN_DIS_MASK);
    }

    /// Clear DEV_CONN_DIS bit atomically (RW)
    #[inline(always)]
    pub fn clr_dev_conn_dis(&self) {
        self.clr_bits(Self::DEV_CONN_DIS_MASK);
    }

    /// **BUS_RESET** `[12]` (RW)
    ///
    /// Source: SIE_STATUS.BUS_RESET
    #[inline(always)]
    pub fn bus_reset(&self) -> bool {
        (self.value.read_bits() & Self::BUS_RESET_MASK) != 0
    }

    /// Set BUS_RESET bit atomically (RW)
    #[inline(always)]
    pub fn set_bus_reset(&self) {
        self.set_bits(Self::BUS_RESET_MASK);
    }

    /// Clear BUS_RESET bit atomically (RW)
    #[inline(always)]
    pub fn clr_bus_reset(&self) {
        self.clr_bits(Self::BUS_RESET_MASK);
    }

    /// **VBUS_DETECT** `[11]` (RW)
    ///
    /// Source: SIE_STATUS.VBUS_DETECTED
    #[inline(always)]
    pub fn vbus_detect(&self) -> bool {
        (self.value.read_bits() & Self::VBUS_DETECT_MASK) != 0
    }

    /// Set VBUS_DETECT bit atomically (RW)
    #[inline(always)]
    pub fn set_vbus_detect(&self) {
        self.set_bits(Self::VBUS_DETECT_MASK);
    }

    /// Clear VBUS_DETECT bit atomically (RW)
    #[inline(always)]
    pub fn clr_vbus_detect(&self) {
        self.clr_bits(Self::VBUS_DETECT_MASK);
    }

    /// **STALL** `[10]` (RW)
    ///
    /// Source: SIE_STATUS.STALL_REC
    #[inline(always)]
    pub fn stall(&self) -> bool {
        (self.value.read_bits() & Self::STALL_MASK) != 0
    }

    /// Set STALL bit atomically (RW)
    #[inline(always)]
    pub fn set_stall(&self) {
        self.set_bits(Self::STALL_MASK);
    }

    /// Clear STALL bit atomically (RW)
    #[inline(always)]
    pub fn clr_stall(&self) {
        self.clr_bits(Self::STALL_MASK);
    }

    /// **ERROR_CRC** `[9]` (RW)
    ///
    /// Source: SIE_STATUS.CRC_ERROR
    #[inline(always)]
    pub fn error_crc(&self) -> bool {
        (self.value.read_bits() & Self::ERROR_CRC_MASK) != 0
    }

    /// Set ERROR_CRC bit atomically (RW)
    #[inline(always)]
    pub fn set_error_crc(&self) {
        self.set_bits(Self::ERROR_CRC_MASK);
    }

    /// Clear ERROR_CRC bit atomically (RW)
    #[inline(always)]
    pub fn clr_error_crc(&self) {
        self.clr_bits(Self::ERROR_CRC_MASK);
    }

    /// **ERROR_BIT_STUFF** `[8]` (RW)
    ///
    /// Source: SIE_STATUS.BIT_STUFF_ERROR
    #[inline(always)]
    pub fn error_bit_stuff(&self) -> bool {
        (self.value.read_bits() & Self::ERROR_BIT_STUFF_MASK) != 0
    }

    /// Set ERROR_BIT_STUFF bit atomically (RW)
    #[inline(always)]
    pub fn set_error_bit_stuff(&self) {
        self.set_bits(Self::ERROR_BIT_STUFF_MASK);
    }

    /// Clear ERROR_BIT_STUFF bit atomically (RW)
    #[inline(always)]
    pub fn clr_error_bit_stuff(&self) {
        self.clr_bits(Self::ERROR_BIT_STUFF_MASK);
    }

    /// **ERROR_RX_OVERFLOW** `[7]` (RW)
    ///
    /// Source: SIE_STATUS.RX_OVERFLOW
    #[inline(always)]
    pub fn error_rx_overflow(&self) -> bool {
        (self.value.read_bits() & Self::ERROR_RX_OVERFLOW_MASK) != 0
    }

    /// Set ERROR_RX_OVERFLOW bit atomically (RW)
    #[inline(always)]
    pub fn set_error_rx_overflow(&self) {
        self.set_bits(Self::ERROR_RX_OVERFLOW_MASK);
    }

    /// Clear ERROR_RX_OVERFLOW bit atomically (RW)
    #[inline(always)]
    pub fn clr_error_rx_overflow(&self) {
        self.clr_bits(Self::ERROR_RX_OVERFLOW_MASK);
    }

    /// **ERROR_RX_TIMEOUT** `[6]` (RW)
    ///
    /// Source: SIE_STATUS.RX_TIMEOUT
    #[inline(always)]
    pub fn error_rx_timeout(&self) -> bool {
        (self.value.read_bits() & Self::ERROR_RX_TIMEOUT_MASK) != 0
    }

    /// Set ERROR_RX_TIMEOUT bit atomically (RW)
    #[inline(always)]
    pub fn set_error_rx_timeout(&self) {
        self.set_bits(Self::ERROR_RX_TIMEOUT_MASK);
    }

    /// Clear ERROR_RX_TIMEOUT bit atomically (RW)
    #[inline(always)]
    pub fn clr_error_rx_timeout(&self) {
        self.clr_bits(Self::ERROR_RX_TIMEOUT_MASK);
    }

    /// **ERROR_DATA_SEQ** `[5]` (RW)
    ///
    /// Source: SIE_STATUS.DATA_SEQ_ERROR
    #[inline(always)]
    pub fn error_data_seq(&self) -> bool {
        (self.value.read_bits() & Self::ERROR_DATA_SEQ_MASK) != 0
    }

    /// Set ERROR_DATA_SEQ bit atomically (RW)
    #[inline(always)]
    pub fn set_error_data_seq(&self) {
        self.set_bits(Self::ERROR_DATA_SEQ_MASK);
    }

    /// Clear ERROR_DATA_SEQ bit atomically (RW)
    #[inline(always)]
    pub fn clr_error_data_seq(&self) {
        self.clr_bits(Self::ERROR_DATA_SEQ_MASK);
    }

    /// **BUFF_STATUS** `[4]` (RW)
    ///
    /// Raised when any bit in BUFF_STATUS is set. Clear by clearing all bits in BUFF_STATUS.
    #[inline(always)]
    pub fn buff_status(&self) -> bool {
        (self.value.read_bits() & Self::BUFF_STATUS_MASK) != 0
    }

    /// Set BUFF_STATUS bit atomically (RW)
    #[inline(always)]
    pub fn set_buff_status(&self) {
        self.set_bits(Self::BUFF_STATUS_MASK);
    }

    /// Clear BUFF_STATUS bit atomically (RW)
    #[inline(always)]
    pub fn clr_buff_status(&self) {
        self.clr_bits(Self::BUFF_STATUS_MASK);
    }

    /// **TRANS_COMPLETE** `[3]` (RW)
    ///
    /// Raised every time SIE_STATUS.TRANS_COMPLETE is set. Clear by writing to this bit.
    #[inline(always)]
    pub fn trans_complete(&self) -> bool {
        (self.value.read_bits() & Self::TRANS_COMPLETE_MASK) != 0
    }

    /// Set TRANS_COMPLETE bit atomically (RW)
    #[inline(always)]
    pub fn set_trans_complete(&self) {
        self.set_bits(Self::TRANS_COMPLETE_MASK);
    }

    /// Clear TRANS_COMPLETE bit atomically (RW)
    #[inline(always)]
    pub fn clr_trans_complete(&self) {
        self.clr_bits(Self::TRANS_COMPLETE_MASK);
    }

    /// **HOST_SOF** `[2]` (RW)
    ///
    /// Host: raised every time the host sends a SOF (Start of Frame). Cleared by reading SOF_RD
    #[inline(always)]
    pub fn host_sof(&self) -> bool {
        (self.value.read_bits() & Self::HOST_SOF_MASK) != 0
    }

    /// Set HOST_SOF bit atomically (RW)
    #[inline(always)]
    pub fn set_host_sof(&self) {
        self.set_bits(Self::HOST_SOF_MASK);
    }

    /// Clear HOST_SOF bit atomically (RW)
    #[inline(always)]
    pub fn clr_host_sof(&self) {
        self.clr_bits(Self::HOST_SOF_MASK);
    }

    /// **HOST_RESUME** `[1]` (RW)
    ///
    /// Host: raised when a device wakes up the host. Cleared by writing to SIE_STATUS.RESUME
    #[inline(always)]
    pub fn host_resume(&self) -> bool {
        (self.value.read_bits() & Self::HOST_RESUME_MASK) != 0
    }

    /// Set HOST_RESUME bit atomically (RW)
    #[inline(always)]
    pub fn set_host_resume(&self) {
        self.set_bits(Self::HOST_RESUME_MASK);
    }

    /// Clear HOST_RESUME bit atomically (RW)
    #[inline(always)]
    pub fn clr_host_resume(&self) {
        self.clr_bits(Self::HOST_RESUME_MASK);
    }

    /// **HOST_CONN_DIS** `[0]` (RW)
    ///
    /// Host: raised when a device is connected or disconnected (i.e. when SIE_STATUS.SPEED changes). Cleared by writing to SIE_STATUS.SPEED
    #[inline(always)]
    pub fn host_conn_dis(&self) -> bool {
        (self.value.read_bits() & Self::HOST_CONN_DIS_MASK) != 0
    }

    /// Set HOST_CONN_DIS bit atomically (RW)
    #[inline(always)]
    pub fn set_host_conn_dis(&self) {
        self.set_bits(Self::HOST_CONN_DIS_MASK);
    }

    /// Clear HOST_CONN_DIS bit atomically (RW)
    #[inline(always)]
    pub fn clr_host_conn_dis(&self) {
        self.clr_bits(Self::HOST_CONN_DIS_MASK);
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

    /// Read **EPX_STOPPED_ON_NAK** `[23:23]`
    ///
    /// Source: NAK_POLL.EPX_STOPPED_ON_NAK
    #[inline(always)]
    pub const fn epx_stopped_on_nak(&self) -> bool {
        (self.bits & Inte::EPX_STOPPED_ON_NAK_MASK) != 0
    }

    /// Read **DEV_SM_WATCHDOG_FIRED** `[22:22]`
    ///
    /// Source: DEV_SM_WATCHDOG.FIRED
    #[inline(always)]
    pub const fn dev_sm_watchdog_fired(&self) -> bool {
        (self.bits & Inte::DEV_SM_WATCHDOG_FIRED_MASK) != 0
    }

    /// Read **ENDPOINT_ERROR** `[21:21]`
    ///
    /// Source: SIE_STATUS.ENDPOINT_ERROR
    #[inline(always)]
    pub const fn endpoint_error(&self) -> bool {
        (self.bits & Inte::ENDPOINT_ERROR_MASK) != 0
    }

    /// Read **RX_SHORT_PACKET** `[20:20]`
    ///
    /// Source: SIE_STATUS.RX_SHORT_PACKET
    #[inline(always)]
    pub const fn rx_short_packet(&self) -> bool {
        (self.bits & Inte::RX_SHORT_PACKET_MASK) != 0
    }

    /// Read **EP_STALL_NAK** `[19:19]`
    ///
    /// Raised when any bit in EP_STATUS_STALL_NAK is set. Clear by clearing all bits in EP_STATUS_STALL_NAK.
    #[inline(always)]
    pub const fn ep_stall_nak(&self) -> bool {
        (self.bits & Inte::EP_STALL_NAK_MASK) != 0
    }

    /// Read **ABORT_DONE** `[18:18]`
    ///
    /// Raised when any bit in ABORT_DONE is set. Clear by clearing all bits in ABORT_DONE.
    #[inline(always)]
    pub const fn abort_done(&self) -> bool {
        (self.bits & Inte::ABORT_DONE_MASK) != 0
    }

    /// Read **DEV_SOF** `[17:17]`
    ///
    /// Set every time the device receives a SOF (Start of Frame) packet. Cleared by reading SOF_RD
    #[inline(always)]
    pub const fn dev_sof(&self) -> bool {
        (self.bits & Inte::DEV_SOF_MASK) != 0
    }

    /// Read **SETUP_REQ** `[16:16]`
    ///
    /// Device. Source: SIE_STATUS.SETUP_REC
    #[inline(always)]
    pub const fn setup_req(&self) -> bool {
        (self.bits & Inte::SETUP_REQ_MASK) != 0
    }

    /// Read **DEV_RESUME_FROM_HOST** `[15:15]`
    ///
    /// Set when the device receives a resume from the host. Cleared by writing to SIE_STATUS.RESUME
    #[inline(always)]
    pub const fn dev_resume_from_host(&self) -> bool {
        (self.bits & Inte::DEV_RESUME_FROM_HOST_MASK) != 0
    }

    /// Read **DEV_SUSPEND** `[14:14]`
    ///
    /// Set when the device suspend state changes. Cleared by writing to SIE_STATUS.SUSPENDED
    #[inline(always)]
    pub const fn dev_suspend(&self) -> bool {
        (self.bits & Inte::DEV_SUSPEND_MASK) != 0
    }

    /// Read **DEV_CONN_DIS** `[13:13]`
    ///
    /// Set when the device connection state changes. Cleared by writing to SIE_STATUS.CONNECTED
    #[inline(always)]
    pub const fn dev_conn_dis(&self) -> bool {
        (self.bits & Inte::DEV_CONN_DIS_MASK) != 0
    }

    /// Read **BUS_RESET** `[12:12]`
    ///
    /// Source: SIE_STATUS.BUS_RESET
    #[inline(always)]
    pub const fn bus_reset(&self) -> bool {
        (self.bits & Inte::BUS_RESET_MASK) != 0
    }

    /// Read **VBUS_DETECT** `[11:11]`
    ///
    /// Source: SIE_STATUS.VBUS_DETECTED
    #[inline(always)]
    pub const fn vbus_detect(&self) -> bool {
        (self.bits & Inte::VBUS_DETECT_MASK) != 0
    }

    /// Read **STALL** `[10:10]`
    ///
    /// Source: SIE_STATUS.STALL_REC
    #[inline(always)]
    pub const fn stall(&self) -> bool {
        (self.bits & Inte::STALL_MASK) != 0
    }

    /// Read **ERROR_CRC** `[9:9]`
    ///
    /// Source: SIE_STATUS.CRC_ERROR
    #[inline(always)]
    pub const fn error_crc(&self) -> bool {
        (self.bits & Inte::ERROR_CRC_MASK) != 0
    }

    /// Read **ERROR_BIT_STUFF** `[8:8]`
    ///
    /// Source: SIE_STATUS.BIT_STUFF_ERROR
    #[inline(always)]
    pub const fn error_bit_stuff(&self) -> bool {
        (self.bits & Inte::ERROR_BIT_STUFF_MASK) != 0
    }

    /// Read **ERROR_RX_OVERFLOW** `[7:7]`
    ///
    /// Source: SIE_STATUS.RX_OVERFLOW
    #[inline(always)]
    pub const fn error_rx_overflow(&self) -> bool {
        (self.bits & Inte::ERROR_RX_OVERFLOW_MASK) != 0
    }

    /// Read **ERROR_RX_TIMEOUT** `[6:6]`
    ///
    /// Source: SIE_STATUS.RX_TIMEOUT
    #[inline(always)]
    pub const fn error_rx_timeout(&self) -> bool {
        (self.bits & Inte::ERROR_RX_TIMEOUT_MASK) != 0
    }

    /// Read **ERROR_DATA_SEQ** `[5:5]`
    ///
    /// Source: SIE_STATUS.DATA_SEQ_ERROR
    #[inline(always)]
    pub const fn error_data_seq(&self) -> bool {
        (self.bits & Inte::ERROR_DATA_SEQ_MASK) != 0
    }

    /// Read **BUFF_STATUS** `[4:4]`
    ///
    /// Raised when any bit in BUFF_STATUS is set. Clear by clearing all bits in BUFF_STATUS.
    #[inline(always)]
    pub const fn buff_status(&self) -> bool {
        (self.bits & Inte::BUFF_STATUS_MASK) != 0
    }

    /// Read **TRANS_COMPLETE** `[3:3]`
    ///
    /// Raised every time SIE_STATUS.TRANS_COMPLETE is set. Clear by writing to this bit.
    #[inline(always)]
    pub const fn trans_complete(&self) -> bool {
        (self.bits & Inte::TRANS_COMPLETE_MASK) != 0
    }

    /// Read **HOST_SOF** `[2:2]`
    ///
    /// Host: raised every time the host sends a SOF (Start of Frame). Cleared by reading SOF_RD
    #[inline(always)]
    pub const fn host_sof(&self) -> bool {
        (self.bits & Inte::HOST_SOF_MASK) != 0
    }

    /// Read **HOST_RESUME** `[1:1]`
    ///
    /// Host: raised when a device wakes up the host. Cleared by writing to SIE_STATUS.RESUME
    #[inline(always)]
    pub const fn host_resume(&self) -> bool {
        (self.bits & Inte::HOST_RESUME_MASK) != 0
    }

    /// Read **HOST_CONN_DIS** `[0:0]`
    ///
    /// Host: raised when a device is connected or disconnected (i.e. when SIE_STATUS.SPEED changes). Cleared by writing to SIE_STATUS.SPEED
    #[inline(always)]
    pub const fn host_conn_dis(&self) -> bool {
        (self.bits & Inte::HOST_CONN_DIS_MASK) != 0
    }

    /// Set **EPX_STOPPED_ON_NAK** `[23:23]`
    ///
    /// Source: NAK_POLL.EPX_STOPPED_ON_NAK
    #[inline(always)]
    pub const fn with_epx_stopped_on_nak(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Inte::EPX_STOPPED_ON_NAK_MASK
            } else {
                self.bits & !Inte::EPX_STOPPED_ON_NAK_MASK
            },
        }
    }

    /// Set **DEV_SM_WATCHDOG_FIRED** `[22:22]`
    ///
    /// Source: DEV_SM_WATCHDOG.FIRED
    #[inline(always)]
    pub const fn with_dev_sm_watchdog_fired(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Inte::DEV_SM_WATCHDOG_FIRED_MASK
            } else {
                self.bits & !Inte::DEV_SM_WATCHDOG_FIRED_MASK
            },
        }
    }

    /// Set **ENDPOINT_ERROR** `[21:21]`
    ///
    /// Source: SIE_STATUS.ENDPOINT_ERROR
    #[inline(always)]
    pub const fn with_endpoint_error(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Inte::ENDPOINT_ERROR_MASK
            } else {
                self.bits & !Inte::ENDPOINT_ERROR_MASK
            },
        }
    }

    /// Set **RX_SHORT_PACKET** `[20:20]`
    ///
    /// Source: SIE_STATUS.RX_SHORT_PACKET
    #[inline(always)]
    pub const fn with_rx_short_packet(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Inte::RX_SHORT_PACKET_MASK
            } else {
                self.bits & !Inte::RX_SHORT_PACKET_MASK
            },
        }
    }

    /// Set **EP_STALL_NAK** `[19:19]`
    ///
    /// Raised when any bit in EP_STATUS_STALL_NAK is set. Clear by clearing all bits in EP_STATUS_STALL_NAK.
    #[inline(always)]
    pub const fn with_ep_stall_nak(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Inte::EP_STALL_NAK_MASK
            } else {
                self.bits & !Inte::EP_STALL_NAK_MASK
            },
        }
    }

    /// Set **ABORT_DONE** `[18:18]`
    ///
    /// Raised when any bit in ABORT_DONE is set. Clear by clearing all bits in ABORT_DONE.
    #[inline(always)]
    pub const fn with_abort_done(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Inte::ABORT_DONE_MASK
            } else {
                self.bits & !Inte::ABORT_DONE_MASK
            },
        }
    }

    /// Set **DEV_SOF** `[17:17]`
    ///
    /// Set every time the device receives a SOF (Start of Frame) packet. Cleared by reading SOF_RD
    #[inline(always)]
    pub const fn with_dev_sof(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Inte::DEV_SOF_MASK
            } else {
                self.bits & !Inte::DEV_SOF_MASK
            },
        }
    }

    /// Set **SETUP_REQ** `[16:16]`
    ///
    /// Device. Source: SIE_STATUS.SETUP_REC
    #[inline(always)]
    pub const fn with_setup_req(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Inte::SETUP_REQ_MASK
            } else {
                self.bits & !Inte::SETUP_REQ_MASK
            },
        }
    }

    /// Set **DEV_RESUME_FROM_HOST** `[15:15]`
    ///
    /// Set when the device receives a resume from the host. Cleared by writing to SIE_STATUS.RESUME
    #[inline(always)]
    pub const fn with_dev_resume_from_host(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Inte::DEV_RESUME_FROM_HOST_MASK
            } else {
                self.bits & !Inte::DEV_RESUME_FROM_HOST_MASK
            },
        }
    }

    /// Set **DEV_SUSPEND** `[14:14]`
    ///
    /// Set when the device suspend state changes. Cleared by writing to SIE_STATUS.SUSPENDED
    #[inline(always)]
    pub const fn with_dev_suspend(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Inte::DEV_SUSPEND_MASK
            } else {
                self.bits & !Inte::DEV_SUSPEND_MASK
            },
        }
    }

    /// Set **DEV_CONN_DIS** `[13:13]`
    ///
    /// Set when the device connection state changes. Cleared by writing to SIE_STATUS.CONNECTED
    #[inline(always)]
    pub const fn with_dev_conn_dis(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Inte::DEV_CONN_DIS_MASK
            } else {
                self.bits & !Inte::DEV_CONN_DIS_MASK
            },
        }
    }

    /// Set **BUS_RESET** `[12:12]`
    ///
    /// Source: SIE_STATUS.BUS_RESET
    #[inline(always)]
    pub const fn with_bus_reset(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Inte::BUS_RESET_MASK
            } else {
                self.bits & !Inte::BUS_RESET_MASK
            },
        }
    }

    /// Set **VBUS_DETECT** `[11:11]`
    ///
    /// Source: SIE_STATUS.VBUS_DETECTED
    #[inline(always)]
    pub const fn with_vbus_detect(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Inte::VBUS_DETECT_MASK
            } else {
                self.bits & !Inte::VBUS_DETECT_MASK
            },
        }
    }

    /// Set **STALL** `[10:10]`
    ///
    /// Source: SIE_STATUS.STALL_REC
    #[inline(always)]
    pub const fn with_stall(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Inte::STALL_MASK
            } else {
                self.bits & !Inte::STALL_MASK
            },
        }
    }

    /// Set **ERROR_CRC** `[9:9]`
    ///
    /// Source: SIE_STATUS.CRC_ERROR
    #[inline(always)]
    pub const fn with_error_crc(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Inte::ERROR_CRC_MASK
            } else {
                self.bits & !Inte::ERROR_CRC_MASK
            },
        }
    }

    /// Set **ERROR_BIT_STUFF** `[8:8]`
    ///
    /// Source: SIE_STATUS.BIT_STUFF_ERROR
    #[inline(always)]
    pub const fn with_error_bit_stuff(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Inte::ERROR_BIT_STUFF_MASK
            } else {
                self.bits & !Inte::ERROR_BIT_STUFF_MASK
            },
        }
    }

    /// Set **ERROR_RX_OVERFLOW** `[7:7]`
    ///
    /// Source: SIE_STATUS.RX_OVERFLOW
    #[inline(always)]
    pub const fn with_error_rx_overflow(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Inte::ERROR_RX_OVERFLOW_MASK
            } else {
                self.bits & !Inte::ERROR_RX_OVERFLOW_MASK
            },
        }
    }

    /// Set **ERROR_RX_TIMEOUT** `[6:6]`
    ///
    /// Source: SIE_STATUS.RX_TIMEOUT
    #[inline(always)]
    pub const fn with_error_rx_timeout(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Inte::ERROR_RX_TIMEOUT_MASK
            } else {
                self.bits & !Inte::ERROR_RX_TIMEOUT_MASK
            },
        }
    }

    /// Set **ERROR_DATA_SEQ** `[5:5]`
    ///
    /// Source: SIE_STATUS.DATA_SEQ_ERROR
    #[inline(always)]
    pub const fn with_error_data_seq(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Inte::ERROR_DATA_SEQ_MASK
            } else {
                self.bits & !Inte::ERROR_DATA_SEQ_MASK
            },
        }
    }

    /// Set **BUFF_STATUS** `[4:4]`
    ///
    /// Raised when any bit in BUFF_STATUS is set. Clear by clearing all bits in BUFF_STATUS.
    #[inline(always)]
    pub const fn with_buff_status(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Inte::BUFF_STATUS_MASK
            } else {
                self.bits & !Inte::BUFF_STATUS_MASK
            },
        }
    }

    /// Set **TRANS_COMPLETE** `[3:3]`
    ///
    /// Raised every time SIE_STATUS.TRANS_COMPLETE is set. Clear by writing to this bit.
    #[inline(always)]
    pub const fn with_trans_complete(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Inte::TRANS_COMPLETE_MASK
            } else {
                self.bits & !Inte::TRANS_COMPLETE_MASK
            },
        }
    }

    /// Set **HOST_SOF** `[2:2]`
    ///
    /// Host: raised every time the host sends a SOF (Start of Frame). Cleared by reading SOF_RD
    #[inline(always)]
    pub const fn with_host_sof(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Inte::HOST_SOF_MASK
            } else {
                self.bits & !Inte::HOST_SOF_MASK
            },
        }
    }

    /// Set **HOST_RESUME** `[1:1]`
    ///
    /// Host: raised when a device wakes up the host. Cleared by writing to SIE_STATUS.RESUME
    #[inline(always)]
    pub const fn with_host_resume(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Inte::HOST_RESUME_MASK
            } else {
                self.bits & !Inte::HOST_RESUME_MASK
            },
        }
    }

    /// Set **HOST_CONN_DIS** `[0:0]`
    ///
    /// Host: raised when a device is connected or disconnected (i.e. when SIE_STATUS.SPEED changes). Cleared by writing to SIE_STATUS.SPEED
    #[inline(always)]
    pub const fn with_host_conn_dis(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Inte::HOST_CONN_DIS_MASK
            } else {
                self.bits & !Inte::HOST_CONN_DIS_MASK
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
/// Address offset: 0x0094
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **EPX_STOPPED_ON_NAK** `[23]` (RW)
///
///   Source: NAK_POLL.EPX_STOPPED_ON_NAK
///
/// - **DEV_SM_WATCHDOG_FIRED** `[22]` (RW)
///
///   Source: DEV_SM_WATCHDOG.FIRED
///
/// - **ENDPOINT_ERROR** `[21]` (RW)
///
///   Source: SIE_STATUS.ENDPOINT_ERROR
///
/// - **RX_SHORT_PACKET** `[20]` (RW)
///
///   Source: SIE_STATUS.RX_SHORT_PACKET
///
/// - **EP_STALL_NAK** `[19]` (RW)
///
///   Raised when any bit in EP_STATUS_STALL_NAK is set. Clear by clearing all bits in EP_STATUS_STALL_NAK.
///
/// - **ABORT_DONE** `[18]` (RW)
///
///   Raised when any bit in ABORT_DONE is set. Clear by clearing all bits in ABORT_DONE.
///
/// - **DEV_SOF** `[17]` (RW)
///
///   Set every time the device receives a SOF (Start of Frame) packet. Cleared by reading SOF_RD
///
/// - **SETUP_REQ** `[16]` (RW)
///
///   Device. Source: SIE_STATUS.SETUP_REC
///
/// - **DEV_RESUME_FROM_HOST** `[15]` (RW)
///
///   Set when the device receives a resume from the host. Cleared by writing to SIE_STATUS.RESUME
///
/// - **DEV_SUSPEND** `[14]` (RW)
///
///   Set when the device suspend state changes. Cleared by writing to SIE_STATUS.SUSPENDED
///
/// - **DEV_CONN_DIS** `[13]` (RW)
///
///   Set when the device connection state changes. Cleared by writing to SIE_STATUS.CONNECTED
///
/// - **BUS_RESET** `[12]` (RW)
///
///   Source: SIE_STATUS.BUS_RESET
///
/// - **VBUS_DETECT** `[11]` (RW)
///
///   Source: SIE_STATUS.VBUS_DETECTED
///
/// - **STALL** `[10]` (RW)
///
///   Source: SIE_STATUS.STALL_REC
///
/// - **ERROR_CRC** `[9]` (RW)
///
///   Source: SIE_STATUS.CRC_ERROR
///
/// - **ERROR_BIT_STUFF** `[8]` (RW)
///
///   Source: SIE_STATUS.BIT_STUFF_ERROR
///
/// - **ERROR_RX_OVERFLOW** `[7]` (RW)
///
///   Source: SIE_STATUS.RX_OVERFLOW
///
/// - **ERROR_RX_TIMEOUT** `[6]` (RW)
///
///   Source: SIE_STATUS.RX_TIMEOUT
///
/// - **ERROR_DATA_SEQ** `[5]` (RW)
///
///   Source: SIE_STATUS.DATA_SEQ_ERROR
///
/// - **BUFF_STATUS** `[4]` (RW)
///
///   Raised when any bit in BUFF_STATUS is set. Clear by clearing all bits in BUFF_STATUS.
///
/// - **TRANS_COMPLETE** `[3]` (RW)
///
///   Raised every time SIE_STATUS.TRANS_COMPLETE is set. Clear by writing to this bit.
///
/// - **HOST_SOF** `[2]` (RW)
///
///   Host: raised every time the host sends a SOF (Start of Frame). Cleared by reading SOF_RD
///
/// - **HOST_RESUME** `[1]` (RW)
///
///   Host: raised when a device wakes up the host. Cleared by writing to SIE_STATUS.RESUME
///
/// - **HOST_CONN_DIS** `[0]` (RW)
///
///   Host: raised when a device is connected or disconnected (i.e. when SIE_STATUS.SPEED changes). Cleared by writing to SIE_STATUS.SPEED
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

    /// **EPX_STOPPED_ON_NAK**
    ///
    /// Source: NAK_POLL.EPX_STOPPED_ON_NAK
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const EPX_STOPPED_ON_NAK_OFFSET: u32 = 23;
    pub const EPX_STOPPED_ON_NAK_MASK: u32 = 0x00800000;
    pub const EPX_STOPPED_ON_NAK: u32 = 1 << 23;

    /// **DEV_SM_WATCHDOG_FIRED**
    ///
    /// Source: DEV_SM_WATCHDOG.FIRED
    ///
    /// Bits `[22:22]`, mask `0x00400000`
    pub const DEV_SM_WATCHDOG_FIRED_OFFSET: u32 = 22;
    pub const DEV_SM_WATCHDOG_FIRED_MASK: u32 = 0x00400000;
    pub const DEV_SM_WATCHDOG_FIRED: u32 = 1 << 22;

    /// **ENDPOINT_ERROR**
    ///
    /// Source: SIE_STATUS.ENDPOINT_ERROR
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const ENDPOINT_ERROR_OFFSET: u32 = 21;
    pub const ENDPOINT_ERROR_MASK: u32 = 0x00200000;
    pub const ENDPOINT_ERROR: u32 = 1 << 21;

    /// **RX_SHORT_PACKET**
    ///
    /// Source: SIE_STATUS.RX_SHORT_PACKET
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const RX_SHORT_PACKET_OFFSET: u32 = 20;
    pub const RX_SHORT_PACKET_MASK: u32 = 0x00100000;
    pub const RX_SHORT_PACKET: u32 = 1 << 20;

    /// **EP_STALL_NAK**
    ///
    /// Raised when any bit in EP_STATUS_STALL_NAK is set. Clear by clearing all bits in EP_STATUS_STALL_NAK.
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const EP_STALL_NAK_OFFSET: u32 = 19;
    pub const EP_STALL_NAK_MASK: u32 = 0x00080000;
    pub const EP_STALL_NAK: u32 = 1 << 19;

    /// **ABORT_DONE**
    ///
    /// Raised when any bit in ABORT_DONE is set. Clear by clearing all bits in ABORT_DONE.
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const ABORT_DONE_OFFSET: u32 = 18;
    pub const ABORT_DONE_MASK: u32 = 0x00040000;
    pub const ABORT_DONE: u32 = 1 << 18;

    /// **DEV_SOF**
    ///
    /// Set every time the device receives a SOF (Start of Frame) packet. Cleared by reading SOF_RD
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const DEV_SOF_OFFSET: u32 = 17;
    pub const DEV_SOF_MASK: u32 = 0x00020000;
    pub const DEV_SOF: u32 = 1 << 17;

    /// **SETUP_REQ**
    ///
    /// Device. Source: SIE_STATUS.SETUP_REC
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const SETUP_REQ_OFFSET: u32 = 16;
    pub const SETUP_REQ_MASK: u32 = 0x00010000;
    pub const SETUP_REQ: u32 = 1 << 16;

    /// **DEV_RESUME_FROM_HOST**
    ///
    /// Set when the device receives a resume from the host. Cleared by writing to SIE_STATUS.RESUME
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const DEV_RESUME_FROM_HOST_OFFSET: u32 = 15;
    pub const DEV_RESUME_FROM_HOST_MASK: u32 = 0x00008000;
    pub const DEV_RESUME_FROM_HOST: u32 = 1 << 15;

    /// **DEV_SUSPEND**
    ///
    /// Set when the device suspend state changes. Cleared by writing to SIE_STATUS.SUSPENDED
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const DEV_SUSPEND_OFFSET: u32 = 14;
    pub const DEV_SUSPEND_MASK: u32 = 0x00004000;
    pub const DEV_SUSPEND: u32 = 1 << 14;

    /// **DEV_CONN_DIS**
    ///
    /// Set when the device connection state changes. Cleared by writing to SIE_STATUS.CONNECTED
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const DEV_CONN_DIS_OFFSET: u32 = 13;
    pub const DEV_CONN_DIS_MASK: u32 = 0x00002000;
    pub const DEV_CONN_DIS: u32 = 1 << 13;

    /// **BUS_RESET**
    ///
    /// Source: SIE_STATUS.BUS_RESET
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const BUS_RESET_OFFSET: u32 = 12;
    pub const BUS_RESET_MASK: u32 = 0x00001000;
    pub const BUS_RESET: u32 = 1 << 12;

    /// **VBUS_DETECT**
    ///
    /// Source: SIE_STATUS.VBUS_DETECTED
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const VBUS_DETECT_OFFSET: u32 = 11;
    pub const VBUS_DETECT_MASK: u32 = 0x00000800;
    pub const VBUS_DETECT: u32 = 1 << 11;

    /// **STALL**
    ///
    /// Source: SIE_STATUS.STALL_REC
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const STALL_OFFSET: u32 = 10;
    pub const STALL_MASK: u32 = 0x00000400;
    pub const STALL: u32 = 1 << 10;

    /// **ERROR_CRC**
    ///
    /// Source: SIE_STATUS.CRC_ERROR
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const ERROR_CRC_OFFSET: u32 = 9;
    pub const ERROR_CRC_MASK: u32 = 0x00000200;
    pub const ERROR_CRC: u32 = 1 << 9;

    /// **ERROR_BIT_STUFF**
    ///
    /// Source: SIE_STATUS.BIT_STUFF_ERROR
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ERROR_BIT_STUFF_OFFSET: u32 = 8;
    pub const ERROR_BIT_STUFF_MASK: u32 = 0x00000100;
    pub const ERROR_BIT_STUFF: u32 = 1 << 8;

    /// **ERROR_RX_OVERFLOW**
    ///
    /// Source: SIE_STATUS.RX_OVERFLOW
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const ERROR_RX_OVERFLOW_OFFSET: u32 = 7;
    pub const ERROR_RX_OVERFLOW_MASK: u32 = 0x00000080;
    pub const ERROR_RX_OVERFLOW: u32 = 1 << 7;

    /// **ERROR_RX_TIMEOUT**
    ///
    /// Source: SIE_STATUS.RX_TIMEOUT
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const ERROR_RX_TIMEOUT_OFFSET: u32 = 6;
    pub const ERROR_RX_TIMEOUT_MASK: u32 = 0x00000040;
    pub const ERROR_RX_TIMEOUT: u32 = 1 << 6;

    /// **ERROR_DATA_SEQ**
    ///
    /// Source: SIE_STATUS.DATA_SEQ_ERROR
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const ERROR_DATA_SEQ_OFFSET: u32 = 5;
    pub const ERROR_DATA_SEQ_MASK: u32 = 0x00000020;
    pub const ERROR_DATA_SEQ: u32 = 1 << 5;

    /// **BUFF_STATUS**
    ///
    /// Raised when any bit in BUFF_STATUS is set. Clear by clearing all bits in BUFF_STATUS.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const BUFF_STATUS_OFFSET: u32 = 4;
    pub const BUFF_STATUS_MASK: u32 = 0x00000010;
    pub const BUFF_STATUS: u32 = 1 << 4;

    /// **TRANS_COMPLETE**
    ///
    /// Raised every time SIE_STATUS.TRANS_COMPLETE is set. Clear by writing to this bit.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const TRANS_COMPLETE_OFFSET: u32 = 3;
    pub const TRANS_COMPLETE_MASK: u32 = 0x00000008;
    pub const TRANS_COMPLETE: u32 = 1 << 3;

    /// **HOST_SOF**
    ///
    /// Host: raised every time the host sends a SOF (Start of Frame). Cleared by reading SOF_RD
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const HOST_SOF_OFFSET: u32 = 2;
    pub const HOST_SOF_MASK: u32 = 0x00000004;
    pub const HOST_SOF: u32 = 1 << 2;

    /// **HOST_RESUME**
    ///
    /// Host: raised when a device wakes up the host. Cleared by writing to SIE_STATUS.RESUME
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const HOST_RESUME_OFFSET: u32 = 1;
    pub const HOST_RESUME_MASK: u32 = 0x00000002;
    pub const HOST_RESUME: u32 = 1 << 1;

    /// **HOST_CONN_DIS**
    ///
    /// Host: raised when a device is connected or disconnected (i.e. when SIE_STATUS.SPEED changes). Cleared by writing to SIE_STATUS.SPEED
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const HOST_CONN_DIS_OFFSET: u32 = 0;
    pub const HOST_CONN_DIS_MASK: u32 = 0x00000001;
    pub const HOST_CONN_DIS: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **EPX_STOPPED_ON_NAK** `[23]` (RW)
    ///
    /// Source: NAK_POLL.EPX_STOPPED_ON_NAK
    #[inline(always)]
    pub fn epx_stopped_on_nak(&self) -> bool {
        (self.value.read_bits() & Self::EPX_STOPPED_ON_NAK_MASK) != 0
    }

    /// Set EPX_STOPPED_ON_NAK bit atomically (RW)
    #[inline(always)]
    pub fn set_epx_stopped_on_nak(&self) {
        self.set_bits(Self::EPX_STOPPED_ON_NAK_MASK);
    }

    /// Clear EPX_STOPPED_ON_NAK bit atomically (RW)
    #[inline(always)]
    pub fn clr_epx_stopped_on_nak(&self) {
        self.clr_bits(Self::EPX_STOPPED_ON_NAK_MASK);
    }

    /// **DEV_SM_WATCHDOG_FIRED** `[22]` (RW)
    ///
    /// Source: DEV_SM_WATCHDOG.FIRED
    #[inline(always)]
    pub fn dev_sm_watchdog_fired(&self) -> bool {
        (self.value.read_bits() & Self::DEV_SM_WATCHDOG_FIRED_MASK) != 0
    }

    /// Set DEV_SM_WATCHDOG_FIRED bit atomically (RW)
    #[inline(always)]
    pub fn set_dev_sm_watchdog_fired(&self) {
        self.set_bits(Self::DEV_SM_WATCHDOG_FIRED_MASK);
    }

    /// Clear DEV_SM_WATCHDOG_FIRED bit atomically (RW)
    #[inline(always)]
    pub fn clr_dev_sm_watchdog_fired(&self) {
        self.clr_bits(Self::DEV_SM_WATCHDOG_FIRED_MASK);
    }

    /// **ENDPOINT_ERROR** `[21]` (RW)
    ///
    /// Source: SIE_STATUS.ENDPOINT_ERROR
    #[inline(always)]
    pub fn endpoint_error(&self) -> bool {
        (self.value.read_bits() & Self::ENDPOINT_ERROR_MASK) != 0
    }

    /// Set ENDPOINT_ERROR bit atomically (RW)
    #[inline(always)]
    pub fn set_endpoint_error(&self) {
        self.set_bits(Self::ENDPOINT_ERROR_MASK);
    }

    /// Clear ENDPOINT_ERROR bit atomically (RW)
    #[inline(always)]
    pub fn clr_endpoint_error(&self) {
        self.clr_bits(Self::ENDPOINT_ERROR_MASK);
    }

    /// **RX_SHORT_PACKET** `[20]` (RW)
    ///
    /// Source: SIE_STATUS.RX_SHORT_PACKET
    #[inline(always)]
    pub fn rx_short_packet(&self) -> bool {
        (self.value.read_bits() & Self::RX_SHORT_PACKET_MASK) != 0
    }

    /// Set RX_SHORT_PACKET bit atomically (RW)
    #[inline(always)]
    pub fn set_rx_short_packet(&self) {
        self.set_bits(Self::RX_SHORT_PACKET_MASK);
    }

    /// Clear RX_SHORT_PACKET bit atomically (RW)
    #[inline(always)]
    pub fn clr_rx_short_packet(&self) {
        self.clr_bits(Self::RX_SHORT_PACKET_MASK);
    }

    /// **EP_STALL_NAK** `[19]` (RW)
    ///
    /// Raised when any bit in EP_STATUS_STALL_NAK is set. Clear by clearing all bits in EP_STATUS_STALL_NAK.
    #[inline(always)]
    pub fn ep_stall_nak(&self) -> bool {
        (self.value.read_bits() & Self::EP_STALL_NAK_MASK) != 0
    }

    /// Set EP_STALL_NAK bit atomically (RW)
    #[inline(always)]
    pub fn set_ep_stall_nak(&self) {
        self.set_bits(Self::EP_STALL_NAK_MASK);
    }

    /// Clear EP_STALL_NAK bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep_stall_nak(&self) {
        self.clr_bits(Self::EP_STALL_NAK_MASK);
    }

    /// **ABORT_DONE** `[18]` (RW)
    ///
    /// Raised when any bit in ABORT_DONE is set. Clear by clearing all bits in ABORT_DONE.
    #[inline(always)]
    pub fn abort_done(&self) -> bool {
        (self.value.read_bits() & Self::ABORT_DONE_MASK) != 0
    }

    /// Set ABORT_DONE bit atomically (RW)
    #[inline(always)]
    pub fn set_abort_done(&self) {
        self.set_bits(Self::ABORT_DONE_MASK);
    }

    /// Clear ABORT_DONE bit atomically (RW)
    #[inline(always)]
    pub fn clr_abort_done(&self) {
        self.clr_bits(Self::ABORT_DONE_MASK);
    }

    /// **DEV_SOF** `[17]` (RW)
    ///
    /// Set every time the device receives a SOF (Start of Frame) packet. Cleared by reading SOF_RD
    #[inline(always)]
    pub fn dev_sof(&self) -> bool {
        (self.value.read_bits() & Self::DEV_SOF_MASK) != 0
    }

    /// Set DEV_SOF bit atomically (RW)
    #[inline(always)]
    pub fn set_dev_sof(&self) {
        self.set_bits(Self::DEV_SOF_MASK);
    }

    /// Clear DEV_SOF bit atomically (RW)
    #[inline(always)]
    pub fn clr_dev_sof(&self) {
        self.clr_bits(Self::DEV_SOF_MASK);
    }

    /// **SETUP_REQ** `[16]` (RW)
    ///
    /// Device. Source: SIE_STATUS.SETUP_REC
    #[inline(always)]
    pub fn setup_req(&self) -> bool {
        (self.value.read_bits() & Self::SETUP_REQ_MASK) != 0
    }

    /// Set SETUP_REQ bit atomically (RW)
    #[inline(always)]
    pub fn set_setup_req(&self) {
        self.set_bits(Self::SETUP_REQ_MASK);
    }

    /// Clear SETUP_REQ bit atomically (RW)
    #[inline(always)]
    pub fn clr_setup_req(&self) {
        self.clr_bits(Self::SETUP_REQ_MASK);
    }

    /// **DEV_RESUME_FROM_HOST** `[15]` (RW)
    ///
    /// Set when the device receives a resume from the host. Cleared by writing to SIE_STATUS.RESUME
    #[inline(always)]
    pub fn dev_resume_from_host(&self) -> bool {
        (self.value.read_bits() & Self::DEV_RESUME_FROM_HOST_MASK) != 0
    }

    /// Set DEV_RESUME_FROM_HOST bit atomically (RW)
    #[inline(always)]
    pub fn set_dev_resume_from_host(&self) {
        self.set_bits(Self::DEV_RESUME_FROM_HOST_MASK);
    }

    /// Clear DEV_RESUME_FROM_HOST bit atomically (RW)
    #[inline(always)]
    pub fn clr_dev_resume_from_host(&self) {
        self.clr_bits(Self::DEV_RESUME_FROM_HOST_MASK);
    }

    /// **DEV_SUSPEND** `[14]` (RW)
    ///
    /// Set when the device suspend state changes. Cleared by writing to SIE_STATUS.SUSPENDED
    #[inline(always)]
    pub fn dev_suspend(&self) -> bool {
        (self.value.read_bits() & Self::DEV_SUSPEND_MASK) != 0
    }

    /// Set DEV_SUSPEND bit atomically (RW)
    #[inline(always)]
    pub fn set_dev_suspend(&self) {
        self.set_bits(Self::DEV_SUSPEND_MASK);
    }

    /// Clear DEV_SUSPEND bit atomically (RW)
    #[inline(always)]
    pub fn clr_dev_suspend(&self) {
        self.clr_bits(Self::DEV_SUSPEND_MASK);
    }

    /// **DEV_CONN_DIS** `[13]` (RW)
    ///
    /// Set when the device connection state changes. Cleared by writing to SIE_STATUS.CONNECTED
    #[inline(always)]
    pub fn dev_conn_dis(&self) -> bool {
        (self.value.read_bits() & Self::DEV_CONN_DIS_MASK) != 0
    }

    /// Set DEV_CONN_DIS bit atomically (RW)
    #[inline(always)]
    pub fn set_dev_conn_dis(&self) {
        self.set_bits(Self::DEV_CONN_DIS_MASK);
    }

    /// Clear DEV_CONN_DIS bit atomically (RW)
    #[inline(always)]
    pub fn clr_dev_conn_dis(&self) {
        self.clr_bits(Self::DEV_CONN_DIS_MASK);
    }

    /// **BUS_RESET** `[12]` (RW)
    ///
    /// Source: SIE_STATUS.BUS_RESET
    #[inline(always)]
    pub fn bus_reset(&self) -> bool {
        (self.value.read_bits() & Self::BUS_RESET_MASK) != 0
    }

    /// Set BUS_RESET bit atomically (RW)
    #[inline(always)]
    pub fn set_bus_reset(&self) {
        self.set_bits(Self::BUS_RESET_MASK);
    }

    /// Clear BUS_RESET bit atomically (RW)
    #[inline(always)]
    pub fn clr_bus_reset(&self) {
        self.clr_bits(Self::BUS_RESET_MASK);
    }

    /// **VBUS_DETECT** `[11]` (RW)
    ///
    /// Source: SIE_STATUS.VBUS_DETECTED
    #[inline(always)]
    pub fn vbus_detect(&self) -> bool {
        (self.value.read_bits() & Self::VBUS_DETECT_MASK) != 0
    }

    /// Set VBUS_DETECT bit atomically (RW)
    #[inline(always)]
    pub fn set_vbus_detect(&self) {
        self.set_bits(Self::VBUS_DETECT_MASK);
    }

    /// Clear VBUS_DETECT bit atomically (RW)
    #[inline(always)]
    pub fn clr_vbus_detect(&self) {
        self.clr_bits(Self::VBUS_DETECT_MASK);
    }

    /// **STALL** `[10]` (RW)
    ///
    /// Source: SIE_STATUS.STALL_REC
    #[inline(always)]
    pub fn stall(&self) -> bool {
        (self.value.read_bits() & Self::STALL_MASK) != 0
    }

    /// Set STALL bit atomically (RW)
    #[inline(always)]
    pub fn set_stall(&self) {
        self.set_bits(Self::STALL_MASK);
    }

    /// Clear STALL bit atomically (RW)
    #[inline(always)]
    pub fn clr_stall(&self) {
        self.clr_bits(Self::STALL_MASK);
    }

    /// **ERROR_CRC** `[9]` (RW)
    ///
    /// Source: SIE_STATUS.CRC_ERROR
    #[inline(always)]
    pub fn error_crc(&self) -> bool {
        (self.value.read_bits() & Self::ERROR_CRC_MASK) != 0
    }

    /// Set ERROR_CRC bit atomically (RW)
    #[inline(always)]
    pub fn set_error_crc(&self) {
        self.set_bits(Self::ERROR_CRC_MASK);
    }

    /// Clear ERROR_CRC bit atomically (RW)
    #[inline(always)]
    pub fn clr_error_crc(&self) {
        self.clr_bits(Self::ERROR_CRC_MASK);
    }

    /// **ERROR_BIT_STUFF** `[8]` (RW)
    ///
    /// Source: SIE_STATUS.BIT_STUFF_ERROR
    #[inline(always)]
    pub fn error_bit_stuff(&self) -> bool {
        (self.value.read_bits() & Self::ERROR_BIT_STUFF_MASK) != 0
    }

    /// Set ERROR_BIT_STUFF bit atomically (RW)
    #[inline(always)]
    pub fn set_error_bit_stuff(&self) {
        self.set_bits(Self::ERROR_BIT_STUFF_MASK);
    }

    /// Clear ERROR_BIT_STUFF bit atomically (RW)
    #[inline(always)]
    pub fn clr_error_bit_stuff(&self) {
        self.clr_bits(Self::ERROR_BIT_STUFF_MASK);
    }

    /// **ERROR_RX_OVERFLOW** `[7]` (RW)
    ///
    /// Source: SIE_STATUS.RX_OVERFLOW
    #[inline(always)]
    pub fn error_rx_overflow(&self) -> bool {
        (self.value.read_bits() & Self::ERROR_RX_OVERFLOW_MASK) != 0
    }

    /// Set ERROR_RX_OVERFLOW bit atomically (RW)
    #[inline(always)]
    pub fn set_error_rx_overflow(&self) {
        self.set_bits(Self::ERROR_RX_OVERFLOW_MASK);
    }

    /// Clear ERROR_RX_OVERFLOW bit atomically (RW)
    #[inline(always)]
    pub fn clr_error_rx_overflow(&self) {
        self.clr_bits(Self::ERROR_RX_OVERFLOW_MASK);
    }

    /// **ERROR_RX_TIMEOUT** `[6]` (RW)
    ///
    /// Source: SIE_STATUS.RX_TIMEOUT
    #[inline(always)]
    pub fn error_rx_timeout(&self) -> bool {
        (self.value.read_bits() & Self::ERROR_RX_TIMEOUT_MASK) != 0
    }

    /// Set ERROR_RX_TIMEOUT bit atomically (RW)
    #[inline(always)]
    pub fn set_error_rx_timeout(&self) {
        self.set_bits(Self::ERROR_RX_TIMEOUT_MASK);
    }

    /// Clear ERROR_RX_TIMEOUT bit atomically (RW)
    #[inline(always)]
    pub fn clr_error_rx_timeout(&self) {
        self.clr_bits(Self::ERROR_RX_TIMEOUT_MASK);
    }

    /// **ERROR_DATA_SEQ** `[5]` (RW)
    ///
    /// Source: SIE_STATUS.DATA_SEQ_ERROR
    #[inline(always)]
    pub fn error_data_seq(&self) -> bool {
        (self.value.read_bits() & Self::ERROR_DATA_SEQ_MASK) != 0
    }

    /// Set ERROR_DATA_SEQ bit atomically (RW)
    #[inline(always)]
    pub fn set_error_data_seq(&self) {
        self.set_bits(Self::ERROR_DATA_SEQ_MASK);
    }

    /// Clear ERROR_DATA_SEQ bit atomically (RW)
    #[inline(always)]
    pub fn clr_error_data_seq(&self) {
        self.clr_bits(Self::ERROR_DATA_SEQ_MASK);
    }

    /// **BUFF_STATUS** `[4]` (RW)
    ///
    /// Raised when any bit in BUFF_STATUS is set. Clear by clearing all bits in BUFF_STATUS.
    #[inline(always)]
    pub fn buff_status(&self) -> bool {
        (self.value.read_bits() & Self::BUFF_STATUS_MASK) != 0
    }

    /// Set BUFF_STATUS bit atomically (RW)
    #[inline(always)]
    pub fn set_buff_status(&self) {
        self.set_bits(Self::BUFF_STATUS_MASK);
    }

    /// Clear BUFF_STATUS bit atomically (RW)
    #[inline(always)]
    pub fn clr_buff_status(&self) {
        self.clr_bits(Self::BUFF_STATUS_MASK);
    }

    /// **TRANS_COMPLETE** `[3]` (RW)
    ///
    /// Raised every time SIE_STATUS.TRANS_COMPLETE is set. Clear by writing to this bit.
    #[inline(always)]
    pub fn trans_complete(&self) -> bool {
        (self.value.read_bits() & Self::TRANS_COMPLETE_MASK) != 0
    }

    /// Set TRANS_COMPLETE bit atomically (RW)
    #[inline(always)]
    pub fn set_trans_complete(&self) {
        self.set_bits(Self::TRANS_COMPLETE_MASK);
    }

    /// Clear TRANS_COMPLETE bit atomically (RW)
    #[inline(always)]
    pub fn clr_trans_complete(&self) {
        self.clr_bits(Self::TRANS_COMPLETE_MASK);
    }

    /// **HOST_SOF** `[2]` (RW)
    ///
    /// Host: raised every time the host sends a SOF (Start of Frame). Cleared by reading SOF_RD
    #[inline(always)]
    pub fn host_sof(&self) -> bool {
        (self.value.read_bits() & Self::HOST_SOF_MASK) != 0
    }

    /// Set HOST_SOF bit atomically (RW)
    #[inline(always)]
    pub fn set_host_sof(&self) {
        self.set_bits(Self::HOST_SOF_MASK);
    }

    /// Clear HOST_SOF bit atomically (RW)
    #[inline(always)]
    pub fn clr_host_sof(&self) {
        self.clr_bits(Self::HOST_SOF_MASK);
    }

    /// **HOST_RESUME** `[1]` (RW)
    ///
    /// Host: raised when a device wakes up the host. Cleared by writing to SIE_STATUS.RESUME
    #[inline(always)]
    pub fn host_resume(&self) -> bool {
        (self.value.read_bits() & Self::HOST_RESUME_MASK) != 0
    }

    /// Set HOST_RESUME bit atomically (RW)
    #[inline(always)]
    pub fn set_host_resume(&self) {
        self.set_bits(Self::HOST_RESUME_MASK);
    }

    /// Clear HOST_RESUME bit atomically (RW)
    #[inline(always)]
    pub fn clr_host_resume(&self) {
        self.clr_bits(Self::HOST_RESUME_MASK);
    }

    /// **HOST_CONN_DIS** `[0]` (RW)
    ///
    /// Host: raised when a device is connected or disconnected (i.e. when SIE_STATUS.SPEED changes). Cleared by writing to SIE_STATUS.SPEED
    #[inline(always)]
    pub fn host_conn_dis(&self) -> bool {
        (self.value.read_bits() & Self::HOST_CONN_DIS_MASK) != 0
    }

    /// Set HOST_CONN_DIS bit atomically (RW)
    #[inline(always)]
    pub fn set_host_conn_dis(&self) {
        self.set_bits(Self::HOST_CONN_DIS_MASK);
    }

    /// Clear HOST_CONN_DIS bit atomically (RW)
    #[inline(always)]
    pub fn clr_host_conn_dis(&self) {
        self.clr_bits(Self::HOST_CONN_DIS_MASK);
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

    /// Read **EPX_STOPPED_ON_NAK** `[23:23]`
    ///
    /// Source: NAK_POLL.EPX_STOPPED_ON_NAK
    #[inline(always)]
    pub const fn epx_stopped_on_nak(&self) -> bool {
        (self.bits & Intf::EPX_STOPPED_ON_NAK_MASK) != 0
    }

    /// Read **DEV_SM_WATCHDOG_FIRED** `[22:22]`
    ///
    /// Source: DEV_SM_WATCHDOG.FIRED
    #[inline(always)]
    pub const fn dev_sm_watchdog_fired(&self) -> bool {
        (self.bits & Intf::DEV_SM_WATCHDOG_FIRED_MASK) != 0
    }

    /// Read **ENDPOINT_ERROR** `[21:21]`
    ///
    /// Source: SIE_STATUS.ENDPOINT_ERROR
    #[inline(always)]
    pub const fn endpoint_error(&self) -> bool {
        (self.bits & Intf::ENDPOINT_ERROR_MASK) != 0
    }

    /// Read **RX_SHORT_PACKET** `[20:20]`
    ///
    /// Source: SIE_STATUS.RX_SHORT_PACKET
    #[inline(always)]
    pub const fn rx_short_packet(&self) -> bool {
        (self.bits & Intf::RX_SHORT_PACKET_MASK) != 0
    }

    /// Read **EP_STALL_NAK** `[19:19]`
    ///
    /// Raised when any bit in EP_STATUS_STALL_NAK is set. Clear by clearing all bits in EP_STATUS_STALL_NAK.
    #[inline(always)]
    pub const fn ep_stall_nak(&self) -> bool {
        (self.bits & Intf::EP_STALL_NAK_MASK) != 0
    }

    /// Read **ABORT_DONE** `[18:18]`
    ///
    /// Raised when any bit in ABORT_DONE is set. Clear by clearing all bits in ABORT_DONE.
    #[inline(always)]
    pub const fn abort_done(&self) -> bool {
        (self.bits & Intf::ABORT_DONE_MASK) != 0
    }

    /// Read **DEV_SOF** `[17:17]`
    ///
    /// Set every time the device receives a SOF (Start of Frame) packet. Cleared by reading SOF_RD
    #[inline(always)]
    pub const fn dev_sof(&self) -> bool {
        (self.bits & Intf::DEV_SOF_MASK) != 0
    }

    /// Read **SETUP_REQ** `[16:16]`
    ///
    /// Device. Source: SIE_STATUS.SETUP_REC
    #[inline(always)]
    pub const fn setup_req(&self) -> bool {
        (self.bits & Intf::SETUP_REQ_MASK) != 0
    }

    /// Read **DEV_RESUME_FROM_HOST** `[15:15]`
    ///
    /// Set when the device receives a resume from the host. Cleared by writing to SIE_STATUS.RESUME
    #[inline(always)]
    pub const fn dev_resume_from_host(&self) -> bool {
        (self.bits & Intf::DEV_RESUME_FROM_HOST_MASK) != 0
    }

    /// Read **DEV_SUSPEND** `[14:14]`
    ///
    /// Set when the device suspend state changes. Cleared by writing to SIE_STATUS.SUSPENDED
    #[inline(always)]
    pub const fn dev_suspend(&self) -> bool {
        (self.bits & Intf::DEV_SUSPEND_MASK) != 0
    }

    /// Read **DEV_CONN_DIS** `[13:13]`
    ///
    /// Set when the device connection state changes. Cleared by writing to SIE_STATUS.CONNECTED
    #[inline(always)]
    pub const fn dev_conn_dis(&self) -> bool {
        (self.bits & Intf::DEV_CONN_DIS_MASK) != 0
    }

    /// Read **BUS_RESET** `[12:12]`
    ///
    /// Source: SIE_STATUS.BUS_RESET
    #[inline(always)]
    pub const fn bus_reset(&self) -> bool {
        (self.bits & Intf::BUS_RESET_MASK) != 0
    }

    /// Read **VBUS_DETECT** `[11:11]`
    ///
    /// Source: SIE_STATUS.VBUS_DETECTED
    #[inline(always)]
    pub const fn vbus_detect(&self) -> bool {
        (self.bits & Intf::VBUS_DETECT_MASK) != 0
    }

    /// Read **STALL** `[10:10]`
    ///
    /// Source: SIE_STATUS.STALL_REC
    #[inline(always)]
    pub const fn stall(&self) -> bool {
        (self.bits & Intf::STALL_MASK) != 0
    }

    /// Read **ERROR_CRC** `[9:9]`
    ///
    /// Source: SIE_STATUS.CRC_ERROR
    #[inline(always)]
    pub const fn error_crc(&self) -> bool {
        (self.bits & Intf::ERROR_CRC_MASK) != 0
    }

    /// Read **ERROR_BIT_STUFF** `[8:8]`
    ///
    /// Source: SIE_STATUS.BIT_STUFF_ERROR
    #[inline(always)]
    pub const fn error_bit_stuff(&self) -> bool {
        (self.bits & Intf::ERROR_BIT_STUFF_MASK) != 0
    }

    /// Read **ERROR_RX_OVERFLOW** `[7:7]`
    ///
    /// Source: SIE_STATUS.RX_OVERFLOW
    #[inline(always)]
    pub const fn error_rx_overflow(&self) -> bool {
        (self.bits & Intf::ERROR_RX_OVERFLOW_MASK) != 0
    }

    /// Read **ERROR_RX_TIMEOUT** `[6:6]`
    ///
    /// Source: SIE_STATUS.RX_TIMEOUT
    #[inline(always)]
    pub const fn error_rx_timeout(&self) -> bool {
        (self.bits & Intf::ERROR_RX_TIMEOUT_MASK) != 0
    }

    /// Read **ERROR_DATA_SEQ** `[5:5]`
    ///
    /// Source: SIE_STATUS.DATA_SEQ_ERROR
    #[inline(always)]
    pub const fn error_data_seq(&self) -> bool {
        (self.bits & Intf::ERROR_DATA_SEQ_MASK) != 0
    }

    /// Read **BUFF_STATUS** `[4:4]`
    ///
    /// Raised when any bit in BUFF_STATUS is set. Clear by clearing all bits in BUFF_STATUS.
    #[inline(always)]
    pub const fn buff_status(&self) -> bool {
        (self.bits & Intf::BUFF_STATUS_MASK) != 0
    }

    /// Read **TRANS_COMPLETE** `[3:3]`
    ///
    /// Raised every time SIE_STATUS.TRANS_COMPLETE is set. Clear by writing to this bit.
    #[inline(always)]
    pub const fn trans_complete(&self) -> bool {
        (self.bits & Intf::TRANS_COMPLETE_MASK) != 0
    }

    /// Read **HOST_SOF** `[2:2]`
    ///
    /// Host: raised every time the host sends a SOF (Start of Frame). Cleared by reading SOF_RD
    #[inline(always)]
    pub const fn host_sof(&self) -> bool {
        (self.bits & Intf::HOST_SOF_MASK) != 0
    }

    /// Read **HOST_RESUME** `[1:1]`
    ///
    /// Host: raised when a device wakes up the host. Cleared by writing to SIE_STATUS.RESUME
    #[inline(always)]
    pub const fn host_resume(&self) -> bool {
        (self.bits & Intf::HOST_RESUME_MASK) != 0
    }

    /// Read **HOST_CONN_DIS** `[0:0]`
    ///
    /// Host: raised when a device is connected or disconnected (i.e. when SIE_STATUS.SPEED changes). Cleared by writing to SIE_STATUS.SPEED
    #[inline(always)]
    pub const fn host_conn_dis(&self) -> bool {
        (self.bits & Intf::HOST_CONN_DIS_MASK) != 0
    }

    /// Set **EPX_STOPPED_ON_NAK** `[23:23]`
    ///
    /// Source: NAK_POLL.EPX_STOPPED_ON_NAK
    #[inline(always)]
    pub const fn with_epx_stopped_on_nak(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intf::EPX_STOPPED_ON_NAK_MASK
            } else {
                self.bits & !Intf::EPX_STOPPED_ON_NAK_MASK
            },
        }
    }

    /// Set **DEV_SM_WATCHDOG_FIRED** `[22:22]`
    ///
    /// Source: DEV_SM_WATCHDOG.FIRED
    #[inline(always)]
    pub const fn with_dev_sm_watchdog_fired(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intf::DEV_SM_WATCHDOG_FIRED_MASK
            } else {
                self.bits & !Intf::DEV_SM_WATCHDOG_FIRED_MASK
            },
        }
    }

    /// Set **ENDPOINT_ERROR** `[21:21]`
    ///
    /// Source: SIE_STATUS.ENDPOINT_ERROR
    #[inline(always)]
    pub const fn with_endpoint_error(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intf::ENDPOINT_ERROR_MASK
            } else {
                self.bits & !Intf::ENDPOINT_ERROR_MASK
            },
        }
    }

    /// Set **RX_SHORT_PACKET** `[20:20]`
    ///
    /// Source: SIE_STATUS.RX_SHORT_PACKET
    #[inline(always)]
    pub const fn with_rx_short_packet(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intf::RX_SHORT_PACKET_MASK
            } else {
                self.bits & !Intf::RX_SHORT_PACKET_MASK
            },
        }
    }

    /// Set **EP_STALL_NAK** `[19:19]`
    ///
    /// Raised when any bit in EP_STATUS_STALL_NAK is set. Clear by clearing all bits in EP_STATUS_STALL_NAK.
    #[inline(always)]
    pub const fn with_ep_stall_nak(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intf::EP_STALL_NAK_MASK
            } else {
                self.bits & !Intf::EP_STALL_NAK_MASK
            },
        }
    }

    /// Set **ABORT_DONE** `[18:18]`
    ///
    /// Raised when any bit in ABORT_DONE is set. Clear by clearing all bits in ABORT_DONE.
    #[inline(always)]
    pub const fn with_abort_done(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intf::ABORT_DONE_MASK
            } else {
                self.bits & !Intf::ABORT_DONE_MASK
            },
        }
    }

    /// Set **DEV_SOF** `[17:17]`
    ///
    /// Set every time the device receives a SOF (Start of Frame) packet. Cleared by reading SOF_RD
    #[inline(always)]
    pub const fn with_dev_sof(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intf::DEV_SOF_MASK
            } else {
                self.bits & !Intf::DEV_SOF_MASK
            },
        }
    }

    /// Set **SETUP_REQ** `[16:16]`
    ///
    /// Device. Source: SIE_STATUS.SETUP_REC
    #[inline(always)]
    pub const fn with_setup_req(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intf::SETUP_REQ_MASK
            } else {
                self.bits & !Intf::SETUP_REQ_MASK
            },
        }
    }

    /// Set **DEV_RESUME_FROM_HOST** `[15:15]`
    ///
    /// Set when the device receives a resume from the host. Cleared by writing to SIE_STATUS.RESUME
    #[inline(always)]
    pub const fn with_dev_resume_from_host(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intf::DEV_RESUME_FROM_HOST_MASK
            } else {
                self.bits & !Intf::DEV_RESUME_FROM_HOST_MASK
            },
        }
    }

    /// Set **DEV_SUSPEND** `[14:14]`
    ///
    /// Set when the device suspend state changes. Cleared by writing to SIE_STATUS.SUSPENDED
    #[inline(always)]
    pub const fn with_dev_suspend(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intf::DEV_SUSPEND_MASK
            } else {
                self.bits & !Intf::DEV_SUSPEND_MASK
            },
        }
    }

    /// Set **DEV_CONN_DIS** `[13:13]`
    ///
    /// Set when the device connection state changes. Cleared by writing to SIE_STATUS.CONNECTED
    #[inline(always)]
    pub const fn with_dev_conn_dis(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intf::DEV_CONN_DIS_MASK
            } else {
                self.bits & !Intf::DEV_CONN_DIS_MASK
            },
        }
    }

    /// Set **BUS_RESET** `[12:12]`
    ///
    /// Source: SIE_STATUS.BUS_RESET
    #[inline(always)]
    pub const fn with_bus_reset(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intf::BUS_RESET_MASK
            } else {
                self.bits & !Intf::BUS_RESET_MASK
            },
        }
    }

    /// Set **VBUS_DETECT** `[11:11]`
    ///
    /// Source: SIE_STATUS.VBUS_DETECTED
    #[inline(always)]
    pub const fn with_vbus_detect(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intf::VBUS_DETECT_MASK
            } else {
                self.bits & !Intf::VBUS_DETECT_MASK
            },
        }
    }

    /// Set **STALL** `[10:10]`
    ///
    /// Source: SIE_STATUS.STALL_REC
    #[inline(always)]
    pub const fn with_stall(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intf::STALL_MASK
            } else {
                self.bits & !Intf::STALL_MASK
            },
        }
    }

    /// Set **ERROR_CRC** `[9:9]`
    ///
    /// Source: SIE_STATUS.CRC_ERROR
    #[inline(always)]
    pub const fn with_error_crc(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intf::ERROR_CRC_MASK
            } else {
                self.bits & !Intf::ERROR_CRC_MASK
            },
        }
    }

    /// Set **ERROR_BIT_STUFF** `[8:8]`
    ///
    /// Source: SIE_STATUS.BIT_STUFF_ERROR
    #[inline(always)]
    pub const fn with_error_bit_stuff(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intf::ERROR_BIT_STUFF_MASK
            } else {
                self.bits & !Intf::ERROR_BIT_STUFF_MASK
            },
        }
    }

    /// Set **ERROR_RX_OVERFLOW** `[7:7]`
    ///
    /// Source: SIE_STATUS.RX_OVERFLOW
    #[inline(always)]
    pub const fn with_error_rx_overflow(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intf::ERROR_RX_OVERFLOW_MASK
            } else {
                self.bits & !Intf::ERROR_RX_OVERFLOW_MASK
            },
        }
    }

    /// Set **ERROR_RX_TIMEOUT** `[6:6]`
    ///
    /// Source: SIE_STATUS.RX_TIMEOUT
    #[inline(always)]
    pub const fn with_error_rx_timeout(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intf::ERROR_RX_TIMEOUT_MASK
            } else {
                self.bits & !Intf::ERROR_RX_TIMEOUT_MASK
            },
        }
    }

    /// Set **ERROR_DATA_SEQ** `[5:5]`
    ///
    /// Source: SIE_STATUS.DATA_SEQ_ERROR
    #[inline(always)]
    pub const fn with_error_data_seq(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intf::ERROR_DATA_SEQ_MASK
            } else {
                self.bits & !Intf::ERROR_DATA_SEQ_MASK
            },
        }
    }

    /// Set **BUFF_STATUS** `[4:4]`
    ///
    /// Raised when any bit in BUFF_STATUS is set. Clear by clearing all bits in BUFF_STATUS.
    #[inline(always)]
    pub const fn with_buff_status(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intf::BUFF_STATUS_MASK
            } else {
                self.bits & !Intf::BUFF_STATUS_MASK
            },
        }
    }

    /// Set **TRANS_COMPLETE** `[3:3]`
    ///
    /// Raised every time SIE_STATUS.TRANS_COMPLETE is set. Clear by writing to this bit.
    #[inline(always)]
    pub const fn with_trans_complete(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intf::TRANS_COMPLETE_MASK
            } else {
                self.bits & !Intf::TRANS_COMPLETE_MASK
            },
        }
    }

    /// Set **HOST_SOF** `[2:2]`
    ///
    /// Host: raised every time the host sends a SOF (Start of Frame). Cleared by reading SOF_RD
    #[inline(always)]
    pub const fn with_host_sof(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intf::HOST_SOF_MASK
            } else {
                self.bits & !Intf::HOST_SOF_MASK
            },
        }
    }

    /// Set **HOST_RESUME** `[1:1]`
    ///
    /// Host: raised when a device wakes up the host. Cleared by writing to SIE_STATUS.RESUME
    #[inline(always)]
    pub const fn with_host_resume(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intf::HOST_RESUME_MASK
            } else {
                self.bits & !Intf::HOST_RESUME_MASK
            },
        }
    }

    /// Set **HOST_CONN_DIS** `[0:0]`
    ///
    /// Host: raised when a device is connected or disconnected (i.e. when SIE_STATUS.SPEED changes). Cleared by writing to SIE_STATUS.SPEED
    #[inline(always)]
    pub const fn with_host_conn_dis(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Intf::HOST_CONN_DIS_MASK
            } else {
                self.bits & !Intf::HOST_CONN_DIS_MASK
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
/// Address offset: 0x0098
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **EPX_STOPPED_ON_NAK** `[23]` (RO)
///
///   Source: NAK_POLL.EPX_STOPPED_ON_NAK
///
/// - **DEV_SM_WATCHDOG_FIRED** `[22]` (RO)
///
///   Source: DEV_SM_WATCHDOG.FIRED
///
/// - **ENDPOINT_ERROR** `[21]` (RO)
///
///   Source: SIE_STATUS.ENDPOINT_ERROR
///
/// - **RX_SHORT_PACKET** `[20]` (RO)
///
///   Source: SIE_STATUS.RX_SHORT_PACKET
///
/// - **EP_STALL_NAK** `[19]` (RO)
///
///   Raised when any bit in EP_STATUS_STALL_NAK is set. Clear by clearing all bits in EP_STATUS_STALL_NAK.
///
/// - **ABORT_DONE** `[18]` (RO)
///
///   Raised when any bit in ABORT_DONE is set. Clear by clearing all bits in ABORT_DONE.
///
/// - **DEV_SOF** `[17]` (RO)
///
///   Set every time the device receives a SOF (Start of Frame) packet. Cleared by reading SOF_RD
///
/// - **SETUP_REQ** `[16]` (RO)
///
///   Device. Source: SIE_STATUS.SETUP_REC
///
/// - **DEV_RESUME_FROM_HOST** `[15]` (RO)
///
///   Set when the device receives a resume from the host. Cleared by writing to SIE_STATUS.RESUME
///
/// - **DEV_SUSPEND** `[14]` (RO)
///
///   Set when the device suspend state changes. Cleared by writing to SIE_STATUS.SUSPENDED
///
/// - **DEV_CONN_DIS** `[13]` (RO)
///
///   Set when the device connection state changes. Cleared by writing to SIE_STATUS.CONNECTED
///
/// - **BUS_RESET** `[12]` (RO)
///
///   Source: SIE_STATUS.BUS_RESET
///
/// - **VBUS_DETECT** `[11]` (RO)
///
///   Source: SIE_STATUS.VBUS_DETECTED
///
/// - **STALL** `[10]` (RO)
///
///   Source: SIE_STATUS.STALL_REC
///
/// - **ERROR_CRC** `[9]` (RO)
///
///   Source: SIE_STATUS.CRC_ERROR
///
/// - **ERROR_BIT_STUFF** `[8]` (RO)
///
///   Source: SIE_STATUS.BIT_STUFF_ERROR
///
/// - **ERROR_RX_OVERFLOW** `[7]` (RO)
///
///   Source: SIE_STATUS.RX_OVERFLOW
///
/// - **ERROR_RX_TIMEOUT** `[6]` (RO)
///
///   Source: SIE_STATUS.RX_TIMEOUT
///
/// - **ERROR_DATA_SEQ** `[5]` (RO)
///
///   Source: SIE_STATUS.DATA_SEQ_ERROR
///
/// - **BUFF_STATUS** `[4]` (RO)
///
///   Raised when any bit in BUFF_STATUS is set. Clear by clearing all bits in BUFF_STATUS.
///
/// - **TRANS_COMPLETE** `[3]` (RO)
///
///   Raised every time SIE_STATUS.TRANS_COMPLETE is set. Clear by writing to this bit.
///
/// - **HOST_SOF** `[2]` (RO)
///
///   Host: raised every time the host sends a SOF (Start of Frame). Cleared by reading SOF_RD
///
/// - **HOST_RESUME** `[1]` (RO)
///
///   Host: raised when a device wakes up the host. Cleared by writing to SIE_STATUS.RESUME
///
/// - **HOST_CONN_DIS** `[0]` (RO)
///
///   Host: raised when a device is connected or disconnected (i.e. when SIE_STATUS.SPEED changes). Cleared by writing to SIE_STATUS.SPEED
#[repr(transparent)]
pub struct Ints {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ints {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ints {
    // ======== Field Constants ========

    /// **EPX_STOPPED_ON_NAK**
    ///
    /// Source: NAK_POLL.EPX_STOPPED_ON_NAK
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const EPX_STOPPED_ON_NAK_OFFSET: u32 = 23;
    pub const EPX_STOPPED_ON_NAK_MASK: u32 = 0x00800000;
    pub const EPX_STOPPED_ON_NAK: u32 = 1 << 23;

    /// **DEV_SM_WATCHDOG_FIRED**
    ///
    /// Source: DEV_SM_WATCHDOG.FIRED
    ///
    /// Bits `[22:22]`, mask `0x00400000`
    pub const DEV_SM_WATCHDOG_FIRED_OFFSET: u32 = 22;
    pub const DEV_SM_WATCHDOG_FIRED_MASK: u32 = 0x00400000;
    pub const DEV_SM_WATCHDOG_FIRED: u32 = 1 << 22;

    /// **ENDPOINT_ERROR**
    ///
    /// Source: SIE_STATUS.ENDPOINT_ERROR
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const ENDPOINT_ERROR_OFFSET: u32 = 21;
    pub const ENDPOINT_ERROR_MASK: u32 = 0x00200000;
    pub const ENDPOINT_ERROR: u32 = 1 << 21;

    /// **RX_SHORT_PACKET**
    ///
    /// Source: SIE_STATUS.RX_SHORT_PACKET
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const RX_SHORT_PACKET_OFFSET: u32 = 20;
    pub const RX_SHORT_PACKET_MASK: u32 = 0x00100000;
    pub const RX_SHORT_PACKET: u32 = 1 << 20;

    /// **EP_STALL_NAK**
    ///
    /// Raised when any bit in EP_STATUS_STALL_NAK is set. Clear by clearing all bits in EP_STATUS_STALL_NAK.
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const EP_STALL_NAK_OFFSET: u32 = 19;
    pub const EP_STALL_NAK_MASK: u32 = 0x00080000;
    pub const EP_STALL_NAK: u32 = 1 << 19;

    /// **ABORT_DONE**
    ///
    /// Raised when any bit in ABORT_DONE is set. Clear by clearing all bits in ABORT_DONE.
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const ABORT_DONE_OFFSET: u32 = 18;
    pub const ABORT_DONE_MASK: u32 = 0x00040000;
    pub const ABORT_DONE: u32 = 1 << 18;

    /// **DEV_SOF**
    ///
    /// Set every time the device receives a SOF (Start of Frame) packet. Cleared by reading SOF_RD
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const DEV_SOF_OFFSET: u32 = 17;
    pub const DEV_SOF_MASK: u32 = 0x00020000;
    pub const DEV_SOF: u32 = 1 << 17;

    /// **SETUP_REQ**
    ///
    /// Device. Source: SIE_STATUS.SETUP_REC
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const SETUP_REQ_OFFSET: u32 = 16;
    pub const SETUP_REQ_MASK: u32 = 0x00010000;
    pub const SETUP_REQ: u32 = 1 << 16;

    /// **DEV_RESUME_FROM_HOST**
    ///
    /// Set when the device receives a resume from the host. Cleared by writing to SIE_STATUS.RESUME
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const DEV_RESUME_FROM_HOST_OFFSET: u32 = 15;
    pub const DEV_RESUME_FROM_HOST_MASK: u32 = 0x00008000;
    pub const DEV_RESUME_FROM_HOST: u32 = 1 << 15;

    /// **DEV_SUSPEND**
    ///
    /// Set when the device suspend state changes. Cleared by writing to SIE_STATUS.SUSPENDED
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const DEV_SUSPEND_OFFSET: u32 = 14;
    pub const DEV_SUSPEND_MASK: u32 = 0x00004000;
    pub const DEV_SUSPEND: u32 = 1 << 14;

    /// **DEV_CONN_DIS**
    ///
    /// Set when the device connection state changes. Cleared by writing to SIE_STATUS.CONNECTED
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const DEV_CONN_DIS_OFFSET: u32 = 13;
    pub const DEV_CONN_DIS_MASK: u32 = 0x00002000;
    pub const DEV_CONN_DIS: u32 = 1 << 13;

    /// **BUS_RESET**
    ///
    /// Source: SIE_STATUS.BUS_RESET
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const BUS_RESET_OFFSET: u32 = 12;
    pub const BUS_RESET_MASK: u32 = 0x00001000;
    pub const BUS_RESET: u32 = 1 << 12;

    /// **VBUS_DETECT**
    ///
    /// Source: SIE_STATUS.VBUS_DETECTED
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const VBUS_DETECT_OFFSET: u32 = 11;
    pub const VBUS_DETECT_MASK: u32 = 0x00000800;
    pub const VBUS_DETECT: u32 = 1 << 11;

    /// **STALL**
    ///
    /// Source: SIE_STATUS.STALL_REC
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const STALL_OFFSET: u32 = 10;
    pub const STALL_MASK: u32 = 0x00000400;
    pub const STALL: u32 = 1 << 10;

    /// **ERROR_CRC**
    ///
    /// Source: SIE_STATUS.CRC_ERROR
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const ERROR_CRC_OFFSET: u32 = 9;
    pub const ERROR_CRC_MASK: u32 = 0x00000200;
    pub const ERROR_CRC: u32 = 1 << 9;

    /// **ERROR_BIT_STUFF**
    ///
    /// Source: SIE_STATUS.BIT_STUFF_ERROR
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ERROR_BIT_STUFF_OFFSET: u32 = 8;
    pub const ERROR_BIT_STUFF_MASK: u32 = 0x00000100;
    pub const ERROR_BIT_STUFF: u32 = 1 << 8;

    /// **ERROR_RX_OVERFLOW**
    ///
    /// Source: SIE_STATUS.RX_OVERFLOW
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const ERROR_RX_OVERFLOW_OFFSET: u32 = 7;
    pub const ERROR_RX_OVERFLOW_MASK: u32 = 0x00000080;
    pub const ERROR_RX_OVERFLOW: u32 = 1 << 7;

    /// **ERROR_RX_TIMEOUT**
    ///
    /// Source: SIE_STATUS.RX_TIMEOUT
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const ERROR_RX_TIMEOUT_OFFSET: u32 = 6;
    pub const ERROR_RX_TIMEOUT_MASK: u32 = 0x00000040;
    pub const ERROR_RX_TIMEOUT: u32 = 1 << 6;

    /// **ERROR_DATA_SEQ**
    ///
    /// Source: SIE_STATUS.DATA_SEQ_ERROR
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const ERROR_DATA_SEQ_OFFSET: u32 = 5;
    pub const ERROR_DATA_SEQ_MASK: u32 = 0x00000020;
    pub const ERROR_DATA_SEQ: u32 = 1 << 5;

    /// **BUFF_STATUS**
    ///
    /// Raised when any bit in BUFF_STATUS is set. Clear by clearing all bits in BUFF_STATUS.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const BUFF_STATUS_OFFSET: u32 = 4;
    pub const BUFF_STATUS_MASK: u32 = 0x00000010;
    pub const BUFF_STATUS: u32 = 1 << 4;

    /// **TRANS_COMPLETE**
    ///
    /// Raised every time SIE_STATUS.TRANS_COMPLETE is set. Clear by writing to this bit.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const TRANS_COMPLETE_OFFSET: u32 = 3;
    pub const TRANS_COMPLETE_MASK: u32 = 0x00000008;
    pub const TRANS_COMPLETE: u32 = 1 << 3;

    /// **HOST_SOF**
    ///
    /// Host: raised every time the host sends a SOF (Start of Frame). Cleared by reading SOF_RD
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const HOST_SOF_OFFSET: u32 = 2;
    pub const HOST_SOF_MASK: u32 = 0x00000004;
    pub const HOST_SOF: u32 = 1 << 2;

    /// **HOST_RESUME**
    ///
    /// Host: raised when a device wakes up the host. Cleared by writing to SIE_STATUS.RESUME
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const HOST_RESUME_OFFSET: u32 = 1;
    pub const HOST_RESUME_MASK: u32 = 0x00000002;
    pub const HOST_RESUME: u32 = 1 << 1;

    /// **HOST_CONN_DIS**
    ///
    /// Host: raised when a device is connected or disconnected (i.e. when SIE_STATUS.SPEED changes). Cleared by writing to SIE_STATUS.SPEED
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const HOST_CONN_DIS_OFFSET: u32 = 0;
    pub const HOST_CONN_DIS_MASK: u32 = 0x00000001;
    pub const HOST_CONN_DIS: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **EPX_STOPPED_ON_NAK** `[23]` (RO)
    ///
    /// Source: NAK_POLL.EPX_STOPPED_ON_NAK
    #[inline(always)]
    pub fn epx_stopped_on_nak(&self) -> bool {
        (self.value.read_bits() & Self::EPX_STOPPED_ON_NAK_MASK) != 0
    }

    /// **DEV_SM_WATCHDOG_FIRED** `[22]` (RO)
    ///
    /// Source: DEV_SM_WATCHDOG.FIRED
    #[inline(always)]
    pub fn dev_sm_watchdog_fired(&self) -> bool {
        (self.value.read_bits() & Self::DEV_SM_WATCHDOG_FIRED_MASK) != 0
    }

    /// **ENDPOINT_ERROR** `[21]` (RO)
    ///
    /// Source: SIE_STATUS.ENDPOINT_ERROR
    #[inline(always)]
    pub fn endpoint_error(&self) -> bool {
        (self.value.read_bits() & Self::ENDPOINT_ERROR_MASK) != 0
    }

    /// **RX_SHORT_PACKET** `[20]` (RO)
    ///
    /// Source: SIE_STATUS.RX_SHORT_PACKET
    #[inline(always)]
    pub fn rx_short_packet(&self) -> bool {
        (self.value.read_bits() & Self::RX_SHORT_PACKET_MASK) != 0
    }

    /// **EP_STALL_NAK** `[19]` (RO)
    ///
    /// Raised when any bit in EP_STATUS_STALL_NAK is set. Clear by clearing all bits in EP_STATUS_STALL_NAK.
    #[inline(always)]
    pub fn ep_stall_nak(&self) -> bool {
        (self.value.read_bits() & Self::EP_STALL_NAK_MASK) != 0
    }

    /// **ABORT_DONE** `[18]` (RO)
    ///
    /// Raised when any bit in ABORT_DONE is set. Clear by clearing all bits in ABORT_DONE.
    #[inline(always)]
    pub fn abort_done(&self) -> bool {
        (self.value.read_bits() & Self::ABORT_DONE_MASK) != 0
    }

    /// **DEV_SOF** `[17]` (RO)
    ///
    /// Set every time the device receives a SOF (Start of Frame) packet. Cleared by reading SOF_RD
    #[inline(always)]
    pub fn dev_sof(&self) -> bool {
        (self.value.read_bits() & Self::DEV_SOF_MASK) != 0
    }

    /// **SETUP_REQ** `[16]` (RO)
    ///
    /// Device. Source: SIE_STATUS.SETUP_REC
    #[inline(always)]
    pub fn setup_req(&self) -> bool {
        (self.value.read_bits() & Self::SETUP_REQ_MASK) != 0
    }

    /// **DEV_RESUME_FROM_HOST** `[15]` (RO)
    ///
    /// Set when the device receives a resume from the host. Cleared by writing to SIE_STATUS.RESUME
    #[inline(always)]
    pub fn dev_resume_from_host(&self) -> bool {
        (self.value.read_bits() & Self::DEV_RESUME_FROM_HOST_MASK) != 0
    }

    /// **DEV_SUSPEND** `[14]` (RO)
    ///
    /// Set when the device suspend state changes. Cleared by writing to SIE_STATUS.SUSPENDED
    #[inline(always)]
    pub fn dev_suspend(&self) -> bool {
        (self.value.read_bits() & Self::DEV_SUSPEND_MASK) != 0
    }

    /// **DEV_CONN_DIS** `[13]` (RO)
    ///
    /// Set when the device connection state changes. Cleared by writing to SIE_STATUS.CONNECTED
    #[inline(always)]
    pub fn dev_conn_dis(&self) -> bool {
        (self.value.read_bits() & Self::DEV_CONN_DIS_MASK) != 0
    }

    /// **BUS_RESET** `[12]` (RO)
    ///
    /// Source: SIE_STATUS.BUS_RESET
    #[inline(always)]
    pub fn bus_reset(&self) -> bool {
        (self.value.read_bits() & Self::BUS_RESET_MASK) != 0
    }

    /// **VBUS_DETECT** `[11]` (RO)
    ///
    /// Source: SIE_STATUS.VBUS_DETECTED
    #[inline(always)]
    pub fn vbus_detect(&self) -> bool {
        (self.value.read_bits() & Self::VBUS_DETECT_MASK) != 0
    }

    /// **STALL** `[10]` (RO)
    ///
    /// Source: SIE_STATUS.STALL_REC
    #[inline(always)]
    pub fn stall(&self) -> bool {
        (self.value.read_bits() & Self::STALL_MASK) != 0
    }

    /// **ERROR_CRC** `[9]` (RO)
    ///
    /// Source: SIE_STATUS.CRC_ERROR
    #[inline(always)]
    pub fn error_crc(&self) -> bool {
        (self.value.read_bits() & Self::ERROR_CRC_MASK) != 0
    }

    /// **ERROR_BIT_STUFF** `[8]` (RO)
    ///
    /// Source: SIE_STATUS.BIT_STUFF_ERROR
    #[inline(always)]
    pub fn error_bit_stuff(&self) -> bool {
        (self.value.read_bits() & Self::ERROR_BIT_STUFF_MASK) != 0
    }

    /// **ERROR_RX_OVERFLOW** `[7]` (RO)
    ///
    /// Source: SIE_STATUS.RX_OVERFLOW
    #[inline(always)]
    pub fn error_rx_overflow(&self) -> bool {
        (self.value.read_bits() & Self::ERROR_RX_OVERFLOW_MASK) != 0
    }

    /// **ERROR_RX_TIMEOUT** `[6]` (RO)
    ///
    /// Source: SIE_STATUS.RX_TIMEOUT
    #[inline(always)]
    pub fn error_rx_timeout(&self) -> bool {
        (self.value.read_bits() & Self::ERROR_RX_TIMEOUT_MASK) != 0
    }

    /// **ERROR_DATA_SEQ** `[5]` (RO)
    ///
    /// Source: SIE_STATUS.DATA_SEQ_ERROR
    #[inline(always)]
    pub fn error_data_seq(&self) -> bool {
        (self.value.read_bits() & Self::ERROR_DATA_SEQ_MASK) != 0
    }

    /// **BUFF_STATUS** `[4]` (RO)
    ///
    /// Raised when any bit in BUFF_STATUS is set. Clear by clearing all bits in BUFF_STATUS.
    #[inline(always)]
    pub fn buff_status(&self) -> bool {
        (self.value.read_bits() & Self::BUFF_STATUS_MASK) != 0
    }

    /// **TRANS_COMPLETE** `[3]` (RO)
    ///
    /// Raised every time SIE_STATUS.TRANS_COMPLETE is set. Clear by writing to this bit.
    #[inline(always)]
    pub fn trans_complete(&self) -> bool {
        (self.value.read_bits() & Self::TRANS_COMPLETE_MASK) != 0
    }

    /// **HOST_SOF** `[2]` (RO)
    ///
    /// Host: raised every time the host sends a SOF (Start of Frame). Cleared by reading SOF_RD
    #[inline(always)]
    pub fn host_sof(&self) -> bool {
        (self.value.read_bits() & Self::HOST_SOF_MASK) != 0
    }

    /// **HOST_RESUME** `[1]` (RO)
    ///
    /// Host: raised when a device wakes up the host. Cleared by writing to SIE_STATUS.RESUME
    #[inline(always)]
    pub fn host_resume(&self) -> bool {
        (self.value.read_bits() & Self::HOST_RESUME_MASK) != 0
    }

    /// **HOST_CONN_DIS** `[0]` (RO)
    ///
    /// Host: raised when a device is connected or disconnected (i.e. when SIE_STATUS.SPEED changes). Cleared by writing to SIE_STATUS.SPEED
    #[inline(always)]
    pub fn host_conn_dis(&self) -> bool {
        (self.value.read_bits() & Self::HOST_CONN_DIS_MASK) != 0
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

    /// Read **EPX_STOPPED_ON_NAK** `[23:23]`
    ///
    /// Source: NAK_POLL.EPX_STOPPED_ON_NAK
    #[inline(always)]
    pub const fn epx_stopped_on_nak(&self) -> bool {
        (self.bits & Ints::EPX_STOPPED_ON_NAK_MASK) != 0
    }

    /// Read **DEV_SM_WATCHDOG_FIRED** `[22:22]`
    ///
    /// Source: DEV_SM_WATCHDOG.FIRED
    #[inline(always)]
    pub const fn dev_sm_watchdog_fired(&self) -> bool {
        (self.bits & Ints::DEV_SM_WATCHDOG_FIRED_MASK) != 0
    }

    /// Read **ENDPOINT_ERROR** `[21:21]`
    ///
    /// Source: SIE_STATUS.ENDPOINT_ERROR
    #[inline(always)]
    pub const fn endpoint_error(&self) -> bool {
        (self.bits & Ints::ENDPOINT_ERROR_MASK) != 0
    }

    /// Read **RX_SHORT_PACKET** `[20:20]`
    ///
    /// Source: SIE_STATUS.RX_SHORT_PACKET
    #[inline(always)]
    pub const fn rx_short_packet(&self) -> bool {
        (self.bits & Ints::RX_SHORT_PACKET_MASK) != 0
    }

    /// Read **EP_STALL_NAK** `[19:19]`
    ///
    /// Raised when any bit in EP_STATUS_STALL_NAK is set. Clear by clearing all bits in EP_STATUS_STALL_NAK.
    #[inline(always)]
    pub const fn ep_stall_nak(&self) -> bool {
        (self.bits & Ints::EP_STALL_NAK_MASK) != 0
    }

    /// Read **ABORT_DONE** `[18:18]`
    ///
    /// Raised when any bit in ABORT_DONE is set. Clear by clearing all bits in ABORT_DONE.
    #[inline(always)]
    pub const fn abort_done(&self) -> bool {
        (self.bits & Ints::ABORT_DONE_MASK) != 0
    }

    /// Read **DEV_SOF** `[17:17]`
    ///
    /// Set every time the device receives a SOF (Start of Frame) packet. Cleared by reading SOF_RD
    #[inline(always)]
    pub const fn dev_sof(&self) -> bool {
        (self.bits & Ints::DEV_SOF_MASK) != 0
    }

    /// Read **SETUP_REQ** `[16:16]`
    ///
    /// Device. Source: SIE_STATUS.SETUP_REC
    #[inline(always)]
    pub const fn setup_req(&self) -> bool {
        (self.bits & Ints::SETUP_REQ_MASK) != 0
    }

    /// Read **DEV_RESUME_FROM_HOST** `[15:15]`
    ///
    /// Set when the device receives a resume from the host. Cleared by writing to SIE_STATUS.RESUME
    #[inline(always)]
    pub const fn dev_resume_from_host(&self) -> bool {
        (self.bits & Ints::DEV_RESUME_FROM_HOST_MASK) != 0
    }

    /// Read **DEV_SUSPEND** `[14:14]`
    ///
    /// Set when the device suspend state changes. Cleared by writing to SIE_STATUS.SUSPENDED
    #[inline(always)]
    pub const fn dev_suspend(&self) -> bool {
        (self.bits & Ints::DEV_SUSPEND_MASK) != 0
    }

    /// Read **DEV_CONN_DIS** `[13:13]`
    ///
    /// Set when the device connection state changes. Cleared by writing to SIE_STATUS.CONNECTED
    #[inline(always)]
    pub const fn dev_conn_dis(&self) -> bool {
        (self.bits & Ints::DEV_CONN_DIS_MASK) != 0
    }

    /// Read **BUS_RESET** `[12:12]`
    ///
    /// Source: SIE_STATUS.BUS_RESET
    #[inline(always)]
    pub const fn bus_reset(&self) -> bool {
        (self.bits & Ints::BUS_RESET_MASK) != 0
    }

    /// Read **VBUS_DETECT** `[11:11]`
    ///
    /// Source: SIE_STATUS.VBUS_DETECTED
    #[inline(always)]
    pub const fn vbus_detect(&self) -> bool {
        (self.bits & Ints::VBUS_DETECT_MASK) != 0
    }

    /// Read **STALL** `[10:10]`
    ///
    /// Source: SIE_STATUS.STALL_REC
    #[inline(always)]
    pub const fn stall(&self) -> bool {
        (self.bits & Ints::STALL_MASK) != 0
    }

    /// Read **ERROR_CRC** `[9:9]`
    ///
    /// Source: SIE_STATUS.CRC_ERROR
    #[inline(always)]
    pub const fn error_crc(&self) -> bool {
        (self.bits & Ints::ERROR_CRC_MASK) != 0
    }

    /// Read **ERROR_BIT_STUFF** `[8:8]`
    ///
    /// Source: SIE_STATUS.BIT_STUFF_ERROR
    #[inline(always)]
    pub const fn error_bit_stuff(&self) -> bool {
        (self.bits & Ints::ERROR_BIT_STUFF_MASK) != 0
    }

    /// Read **ERROR_RX_OVERFLOW** `[7:7]`
    ///
    /// Source: SIE_STATUS.RX_OVERFLOW
    #[inline(always)]
    pub const fn error_rx_overflow(&self) -> bool {
        (self.bits & Ints::ERROR_RX_OVERFLOW_MASK) != 0
    }

    /// Read **ERROR_RX_TIMEOUT** `[6:6]`
    ///
    /// Source: SIE_STATUS.RX_TIMEOUT
    #[inline(always)]
    pub const fn error_rx_timeout(&self) -> bool {
        (self.bits & Ints::ERROR_RX_TIMEOUT_MASK) != 0
    }

    /// Read **ERROR_DATA_SEQ** `[5:5]`
    ///
    /// Source: SIE_STATUS.DATA_SEQ_ERROR
    #[inline(always)]
    pub const fn error_data_seq(&self) -> bool {
        (self.bits & Ints::ERROR_DATA_SEQ_MASK) != 0
    }

    /// Read **BUFF_STATUS** `[4:4]`
    ///
    /// Raised when any bit in BUFF_STATUS is set. Clear by clearing all bits in BUFF_STATUS.
    #[inline(always)]
    pub const fn buff_status(&self) -> bool {
        (self.bits & Ints::BUFF_STATUS_MASK) != 0
    }

    /// Read **TRANS_COMPLETE** `[3:3]`
    ///
    /// Raised every time SIE_STATUS.TRANS_COMPLETE is set. Clear by writing to this bit.
    #[inline(always)]
    pub const fn trans_complete(&self) -> bool {
        (self.bits & Ints::TRANS_COMPLETE_MASK) != 0
    }

    /// Read **HOST_SOF** `[2:2]`
    ///
    /// Host: raised every time the host sends a SOF (Start of Frame). Cleared by reading SOF_RD
    #[inline(always)]
    pub const fn host_sof(&self) -> bool {
        (self.bits & Ints::HOST_SOF_MASK) != 0
    }

    /// Read **HOST_RESUME** `[1:1]`
    ///
    /// Host: raised when a device wakes up the host. Cleared by writing to SIE_STATUS.RESUME
    #[inline(always)]
    pub const fn host_resume(&self) -> bool {
        (self.bits & Ints::HOST_RESUME_MASK) != 0
    }

    /// Read **HOST_CONN_DIS** `[0:0]`
    ///
    /// Host: raised when a device is connected or disconnected (i.e. when SIE_STATUS.SPEED changes). Cleared by writing to SIE_STATUS.SPEED
    #[inline(always)]
    pub const fn host_conn_dis(&self) -> bool {
        (self.bits & Ints::HOST_CONN_DIS_MASK) != 0
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

/// SOF_TIMESTAMP_RAW Register
///
/// Device only. Raw value of free-running PHY clock counter @48MHz. Used to calculate time between SOF events.
///
/// Address offset: 0x0100
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SOF_TIMESTAMP_RAW** `[20:0]` (RO)
#[repr(transparent)]
pub struct SofTimestampRaw {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SofTimestampRaw {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SofTimestampRaw {
    // ======== Field Constants ========

    /// **SOF_TIMESTAMP_RAW**
    /// Bits `[20:0]`, mask `0x001fffff`
    pub const SOF_TIMESTAMP_RAW_OFFSET: u32 = 0;
    pub const SOF_TIMESTAMP_RAW_MASK: u32 = 0x001fffff;

    // ======== Field Accessors ========

    /// **SOF_TIMESTAMP_RAW** `[20:0]` (RO)
    #[inline(always)]
    pub fn sof_timestamp_raw(&self) -> u32 {
        (self.value.read_bits() & Self::SOF_TIMESTAMP_RAW_MASK) >> Self::SOF_TIMESTAMP_RAW_OFFSET
    }
}

/// SOF_TIMESTAMP_RAW value type for batch read/modify/write operations
///
/// Use `softimestampraw.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `softimestampraw.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SofTimestampRawVal {
    bits: u32,
}

impl SofTimestampRawVal {
    /// Create from raw bits
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

    /// Read **SOF_TIMESTAMP_RAW** `[20:0]`
    #[inline(always)]
    pub const fn sof_timestamp_raw(&self) -> u32 {
        (self.bits & SofTimestampRaw::SOF_TIMESTAMP_RAW_MASK)
            >> SofTimestampRaw::SOF_TIMESTAMP_RAW_OFFSET
    }
}

impl Default for SofTimestampRawVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SofTimestampRaw {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SofTimestampRawVal {
        SofTimestampRawVal::from_bits(self.value.read_bits())
    }
}

/// SOF_TIMESTAMP_LAST Register
///
/// Device only. Value of free-running PHY clock counter @48MHz when last SOF event occurred.
///
/// Address offset: 0x0104
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SOF_TIMESTAMP_LAST** `[20:0]` (RO)
#[repr(transparent)]
pub struct SofTimestampLast {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SofTimestampLast {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SofTimestampLast {
    // ======== Field Constants ========

    /// **SOF_TIMESTAMP_LAST**
    /// Bits `[20:0]`, mask `0x001fffff`
    pub const SOF_TIMESTAMP_LAST_OFFSET: u32 = 0;
    pub const SOF_TIMESTAMP_LAST_MASK: u32 = 0x001fffff;

    // ======== Field Accessors ========

    /// **SOF_TIMESTAMP_LAST** `[20:0]` (RO)
    #[inline(always)]
    pub fn sof_timestamp_last(&self) -> u32 {
        (self.value.read_bits() & Self::SOF_TIMESTAMP_LAST_MASK) >> Self::SOF_TIMESTAMP_LAST_OFFSET
    }
}

/// SOF_TIMESTAMP_LAST value type for batch read/modify/write operations
///
/// Use `softimestamplast.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `softimestamplast.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SofTimestampLastVal {
    bits: u32,
}

impl SofTimestampLastVal {
    /// Create from raw bits
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

    /// Read **SOF_TIMESTAMP_LAST** `[20:0]`
    #[inline(always)]
    pub const fn sof_timestamp_last(&self) -> u32 {
        (self.bits & SofTimestampLast::SOF_TIMESTAMP_LAST_MASK)
            >> SofTimestampLast::SOF_TIMESTAMP_LAST_OFFSET
    }
}

impl Default for SofTimestampLastVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SofTimestampLast {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SofTimestampLastVal {
        SofTimestampLastVal::from_bits(self.value.read_bits())
    }
}

/// SM_STATE Register
///
/// Address offset: 0x0108
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **RX_DASM** `[11:8]` (RO)
///
/// - **BC_STATE** `[7:5]` (RO)
///
/// - **STATE** `[4:0]` (RO)
#[repr(transparent)]
pub struct SmState {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SmState {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SmState {
    // ======== Field Constants ========

    /// **RX_DASM**
    /// Bits `[11:8]`, mask `0x00000f00`
    pub const RX_DASM_OFFSET: u32 = 8;
    pub const RX_DASM_MASK: u32 = 0x00000f00;

    /// **BC_STATE**
    /// Bits `[7:5]`, mask `0x000000e0`
    pub const BC_STATE_OFFSET: u32 = 5;
    pub const BC_STATE_MASK: u32 = 0x000000e0;

    /// **STATE**
    /// Bits `[4:0]`, mask `0x0000001f`
    pub const STATE_OFFSET: u32 = 0;
    pub const STATE_MASK: u32 = 0x0000001f;

    // ======== Field Accessors ========

    /// **RX_DASM** `[11:8]` (RO)
    #[inline(always)]
    pub fn rx_dasm(&self) -> u32 {
        (self.value.read_bits() & Self::RX_DASM_MASK) >> Self::RX_DASM_OFFSET
    }

    /// **BC_STATE** `[7:5]` (RO)
    #[inline(always)]
    pub fn bc_state(&self) -> u32 {
        (self.value.read_bits() & Self::BC_STATE_MASK) >> Self::BC_STATE_OFFSET
    }

    /// **STATE** `[4:0]` (RO)
    #[inline(always)]
    pub fn state(&self) -> u32 {
        (self.value.read_bits() & Self::STATE_MASK) >> Self::STATE_OFFSET
    }
}

/// SM_STATE value type for batch read/modify/write operations
///
/// Use `smstate.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `smstate.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SmStateVal {
    bits: u32,
}

impl SmStateVal {
    /// Create from raw bits
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

    /// Read **RX_DASM** `[11:8]`
    #[inline(always)]
    pub const fn rx_dasm(&self) -> u32 {
        (self.bits & SmState::RX_DASM_MASK) >> SmState::RX_DASM_OFFSET
    }

    /// Read **BC_STATE** `[7:5]`
    #[inline(always)]
    pub const fn bc_state(&self) -> u32 {
        (self.bits & SmState::BC_STATE_MASK) >> SmState::BC_STATE_OFFSET
    }

    /// Read **STATE** `[4:0]`
    #[inline(always)]
    pub const fn state(&self) -> u32 {
        (self.bits & SmState::STATE_MASK) >> SmState::STATE_OFFSET
    }
}

impl Default for SmStateVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SmState {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SmStateVal {
        SmStateVal::from_bits(self.value.read_bits())
    }
}

/// EP_TX_ERROR Register
///
/// TX error count for each endpoint. Write to each field to reset the counter to 0.
///
/// Address offset: 0x010c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **EP15** `[31:30]` (RW)
///
/// - **EP14** `[29:28]` (RW)
///
/// - **EP13** `[27:26]` (RW)
///
/// - **EP12** `[25:24]` (RW)
///
/// - **EP11** `[23:22]` (RW)
///
/// - **EP10** `[21:20]` (RW)
///
/// - **EP9** `[19:18]` (RW)
///
/// - **EP8** `[17:16]` (RW)
///
/// - **EP7** `[15:14]` (RW)
///
/// - **EP6** `[13:12]` (RW)
///
/// - **EP5** `[11:10]` (RW)
///
/// - **EP4** `[9:8]` (RW)
///
/// - **EP3** `[7:6]` (RW)
///
/// - **EP2** `[5:4]` (RW)
///
/// - **EP1** `[3:2]` (RW)
///
/// - **EP0** `[1:0]` (RW)
#[repr(transparent)]
pub struct EpTxError {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for EpTxError {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl EpTxError {
    // ======== Field Constants ========

    /// **EP15**
    /// Bits `[31:30]`, mask `0xc0000000`
    pub const EP15_OFFSET: u32 = 30;
    pub const EP15_MASK: u32 = 0xc0000000;

    /// **EP14**
    /// Bits `[29:28]`, mask `0x30000000`
    pub const EP14_OFFSET: u32 = 28;
    pub const EP14_MASK: u32 = 0x30000000;

    /// **EP13**
    /// Bits `[27:26]`, mask `0x0c000000`
    pub const EP13_OFFSET: u32 = 26;
    pub const EP13_MASK: u32 = 0x0c000000;

    /// **EP12**
    /// Bits `[25:24]`, mask `0x03000000`
    pub const EP12_OFFSET: u32 = 24;
    pub const EP12_MASK: u32 = 0x03000000;

    /// **EP11**
    /// Bits `[23:22]`, mask `0x00c00000`
    pub const EP11_OFFSET: u32 = 22;
    pub const EP11_MASK: u32 = 0x00c00000;

    /// **EP10**
    /// Bits `[21:20]`, mask `0x00300000`
    pub const EP10_OFFSET: u32 = 20;
    pub const EP10_MASK: u32 = 0x00300000;

    /// **EP9**
    /// Bits `[19:18]`, mask `0x000c0000`
    pub const EP9_OFFSET: u32 = 18;
    pub const EP9_MASK: u32 = 0x000c0000;

    /// **EP8**
    /// Bits `[17:16]`, mask `0x00030000`
    pub const EP8_OFFSET: u32 = 16;
    pub const EP8_MASK: u32 = 0x00030000;

    /// **EP7**
    /// Bits `[15:14]`, mask `0x0000c000`
    pub const EP7_OFFSET: u32 = 14;
    pub const EP7_MASK: u32 = 0x0000c000;

    /// **EP6**
    /// Bits `[13:12]`, mask `0x00003000`
    pub const EP6_OFFSET: u32 = 12;
    pub const EP6_MASK: u32 = 0x00003000;

    /// **EP5**
    /// Bits `[11:10]`, mask `0x00000c00`
    pub const EP5_OFFSET: u32 = 10;
    pub const EP5_MASK: u32 = 0x00000c00;

    /// **EP4**
    /// Bits `[9:8]`, mask `0x00000300`
    pub const EP4_OFFSET: u32 = 8;
    pub const EP4_MASK: u32 = 0x00000300;

    /// **EP3**
    /// Bits `[7:6]`, mask `0x000000c0`
    pub const EP3_OFFSET: u32 = 6;
    pub const EP3_MASK: u32 = 0x000000c0;

    /// **EP2**
    /// Bits `[5:4]`, mask `0x00000030`
    pub const EP2_OFFSET: u32 = 4;
    pub const EP2_MASK: u32 = 0x00000030;

    /// **EP1**
    /// Bits `[3:2]`, mask `0x0000000c`
    pub const EP1_OFFSET: u32 = 2;
    pub const EP1_MASK: u32 = 0x0000000c;

    /// **EP0**
    /// Bits `[1:0]`, mask `0x00000003`
    pub const EP0_OFFSET: u32 = 0;
    pub const EP0_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **EP15** `[31:30]` (RW)
    #[inline(always)]
    pub fn ep15(&self) -> u32 {
        (self.value.read_bits() & Self::EP15_MASK) >> Self::EP15_OFFSET
    }

    /// Write **EP15** `[31:30]` (RW)
    #[inline(always)]
    pub fn write_ep15(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::EP15_MASK) | ((value << Self::EP15_OFFSET) & Self::EP15_MASK)
        });
    }

    /// **EP14** `[29:28]` (RW)
    #[inline(always)]
    pub fn ep14(&self) -> u32 {
        (self.value.read_bits() & Self::EP14_MASK) >> Self::EP14_OFFSET
    }

    /// Write **EP14** `[29:28]` (RW)
    #[inline(always)]
    pub fn write_ep14(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::EP14_MASK) | ((value << Self::EP14_OFFSET) & Self::EP14_MASK)
        });
    }

    /// **EP13** `[27:26]` (RW)
    #[inline(always)]
    pub fn ep13(&self) -> u32 {
        (self.value.read_bits() & Self::EP13_MASK) >> Self::EP13_OFFSET
    }

    /// Write **EP13** `[27:26]` (RW)
    #[inline(always)]
    pub fn write_ep13(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::EP13_MASK) | ((value << Self::EP13_OFFSET) & Self::EP13_MASK)
        });
    }

    /// **EP12** `[25:24]` (RW)
    #[inline(always)]
    pub fn ep12(&self) -> u32 {
        (self.value.read_bits() & Self::EP12_MASK) >> Self::EP12_OFFSET
    }

    /// Write **EP12** `[25:24]` (RW)
    #[inline(always)]
    pub fn write_ep12(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::EP12_MASK) | ((value << Self::EP12_OFFSET) & Self::EP12_MASK)
        });
    }

    /// **EP11** `[23:22]` (RW)
    #[inline(always)]
    pub fn ep11(&self) -> u32 {
        (self.value.read_bits() & Self::EP11_MASK) >> Self::EP11_OFFSET
    }

    /// Write **EP11** `[23:22]` (RW)
    #[inline(always)]
    pub fn write_ep11(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::EP11_MASK) | ((value << Self::EP11_OFFSET) & Self::EP11_MASK)
        });
    }

    /// **EP10** `[21:20]` (RW)
    #[inline(always)]
    pub fn ep10(&self) -> u32 {
        (self.value.read_bits() & Self::EP10_MASK) >> Self::EP10_OFFSET
    }

    /// Write **EP10** `[21:20]` (RW)
    #[inline(always)]
    pub fn write_ep10(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::EP10_MASK) | ((value << Self::EP10_OFFSET) & Self::EP10_MASK)
        });
    }

    /// **EP9** `[19:18]` (RW)
    #[inline(always)]
    pub fn ep9(&self) -> u32 {
        (self.value.read_bits() & Self::EP9_MASK) >> Self::EP9_OFFSET
    }

    /// Write **EP9** `[19:18]` (RW)
    #[inline(always)]
    pub fn write_ep9(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::EP9_MASK) | ((value << Self::EP9_OFFSET) & Self::EP9_MASK)
        });
    }

    /// **EP8** `[17:16]` (RW)
    #[inline(always)]
    pub fn ep8(&self) -> u32 {
        (self.value.read_bits() & Self::EP8_MASK) >> Self::EP8_OFFSET
    }

    /// Write **EP8** `[17:16]` (RW)
    #[inline(always)]
    pub fn write_ep8(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::EP8_MASK) | ((value << Self::EP8_OFFSET) & Self::EP8_MASK)
        });
    }

    /// **EP7** `[15:14]` (RW)
    #[inline(always)]
    pub fn ep7(&self) -> u32 {
        (self.value.read_bits() & Self::EP7_MASK) >> Self::EP7_OFFSET
    }

    /// Write **EP7** `[15:14]` (RW)
    #[inline(always)]
    pub fn write_ep7(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::EP7_MASK) | ((value << Self::EP7_OFFSET) & Self::EP7_MASK)
        });
    }

    /// **EP6** `[13:12]` (RW)
    #[inline(always)]
    pub fn ep6(&self) -> u32 {
        (self.value.read_bits() & Self::EP6_MASK) >> Self::EP6_OFFSET
    }

    /// Write **EP6** `[13:12]` (RW)
    #[inline(always)]
    pub fn write_ep6(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::EP6_MASK) | ((value << Self::EP6_OFFSET) & Self::EP6_MASK)
        });
    }

    /// **EP5** `[11:10]` (RW)
    #[inline(always)]
    pub fn ep5(&self) -> u32 {
        (self.value.read_bits() & Self::EP5_MASK) >> Self::EP5_OFFSET
    }

    /// Write **EP5** `[11:10]` (RW)
    #[inline(always)]
    pub fn write_ep5(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::EP5_MASK) | ((value << Self::EP5_OFFSET) & Self::EP5_MASK)
        });
    }

    /// **EP4** `[9:8]` (RW)
    #[inline(always)]
    pub fn ep4(&self) -> u32 {
        (self.value.read_bits() & Self::EP4_MASK) >> Self::EP4_OFFSET
    }

    /// Write **EP4** `[9:8]` (RW)
    #[inline(always)]
    pub fn write_ep4(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::EP4_MASK) | ((value << Self::EP4_OFFSET) & Self::EP4_MASK)
        });
    }

    /// **EP3** `[7:6]` (RW)
    #[inline(always)]
    pub fn ep3(&self) -> u32 {
        (self.value.read_bits() & Self::EP3_MASK) >> Self::EP3_OFFSET
    }

    /// Write **EP3** `[7:6]` (RW)
    #[inline(always)]
    pub fn write_ep3(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::EP3_MASK) | ((value << Self::EP3_OFFSET) & Self::EP3_MASK)
        });
    }

    /// **EP2** `[5:4]` (RW)
    #[inline(always)]
    pub fn ep2(&self) -> u32 {
        (self.value.read_bits() & Self::EP2_MASK) >> Self::EP2_OFFSET
    }

    /// Write **EP2** `[5:4]` (RW)
    #[inline(always)]
    pub fn write_ep2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::EP2_MASK) | ((value << Self::EP2_OFFSET) & Self::EP2_MASK)
        });
    }

    /// **EP1** `[3:2]` (RW)
    #[inline(always)]
    pub fn ep1(&self) -> u32 {
        (self.value.read_bits() & Self::EP1_MASK) >> Self::EP1_OFFSET
    }

    /// Write **EP1** `[3:2]` (RW)
    #[inline(always)]
    pub fn write_ep1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::EP1_MASK) | ((value << Self::EP1_OFFSET) & Self::EP1_MASK)
        });
    }

    /// **EP0** `[1:0]` (RW)
    #[inline(always)]
    pub fn ep0(&self) -> u32 {
        (self.value.read_bits() & Self::EP0_MASK) >> Self::EP0_OFFSET
    }

    /// Write **EP0** `[1:0]` (RW)
    #[inline(always)]
    pub fn write_ep0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::EP0_MASK) | ((value << Self::EP0_OFFSET) & Self::EP0_MASK)
        });
    }
}

/// EP_TX_ERROR value type for batch read/modify/write operations
///
/// Use `eptxerror.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `eptxerror.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EpTxErrorVal {
    bits: u32,
}

impl EpTxErrorVal {
    /// Create from raw bits
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

    /// Read **EP15** `[31:30]`
    #[inline(always)]
    pub const fn ep15(&self) -> u32 {
        (self.bits & EpTxError::EP15_MASK) >> EpTxError::EP15_OFFSET
    }

    /// Read **EP14** `[29:28]`
    #[inline(always)]
    pub const fn ep14(&self) -> u32 {
        (self.bits & EpTxError::EP14_MASK) >> EpTxError::EP14_OFFSET
    }

    /// Read **EP13** `[27:26]`
    #[inline(always)]
    pub const fn ep13(&self) -> u32 {
        (self.bits & EpTxError::EP13_MASK) >> EpTxError::EP13_OFFSET
    }

    /// Read **EP12** `[25:24]`
    #[inline(always)]
    pub const fn ep12(&self) -> u32 {
        (self.bits & EpTxError::EP12_MASK) >> EpTxError::EP12_OFFSET
    }

    /// Read **EP11** `[23:22]`
    #[inline(always)]
    pub const fn ep11(&self) -> u32 {
        (self.bits & EpTxError::EP11_MASK) >> EpTxError::EP11_OFFSET
    }

    /// Read **EP10** `[21:20]`
    #[inline(always)]
    pub const fn ep10(&self) -> u32 {
        (self.bits & EpTxError::EP10_MASK) >> EpTxError::EP10_OFFSET
    }

    /// Read **EP9** `[19:18]`
    #[inline(always)]
    pub const fn ep9(&self) -> u32 {
        (self.bits & EpTxError::EP9_MASK) >> EpTxError::EP9_OFFSET
    }

    /// Read **EP8** `[17:16]`
    #[inline(always)]
    pub const fn ep8(&self) -> u32 {
        (self.bits & EpTxError::EP8_MASK) >> EpTxError::EP8_OFFSET
    }

    /// Read **EP7** `[15:14]`
    #[inline(always)]
    pub const fn ep7(&self) -> u32 {
        (self.bits & EpTxError::EP7_MASK) >> EpTxError::EP7_OFFSET
    }

    /// Read **EP6** `[13:12]`
    #[inline(always)]
    pub const fn ep6(&self) -> u32 {
        (self.bits & EpTxError::EP6_MASK) >> EpTxError::EP6_OFFSET
    }

    /// Read **EP5** `[11:10]`
    #[inline(always)]
    pub const fn ep5(&self) -> u32 {
        (self.bits & EpTxError::EP5_MASK) >> EpTxError::EP5_OFFSET
    }

    /// Read **EP4** `[9:8]`
    #[inline(always)]
    pub const fn ep4(&self) -> u32 {
        (self.bits & EpTxError::EP4_MASK) >> EpTxError::EP4_OFFSET
    }

    /// Read **EP3** `[7:6]`
    #[inline(always)]
    pub const fn ep3(&self) -> u32 {
        (self.bits & EpTxError::EP3_MASK) >> EpTxError::EP3_OFFSET
    }

    /// Read **EP2** `[5:4]`
    #[inline(always)]
    pub const fn ep2(&self) -> u32 {
        (self.bits & EpTxError::EP2_MASK) >> EpTxError::EP2_OFFSET
    }

    /// Read **EP1** `[3:2]`
    #[inline(always)]
    pub const fn ep1(&self) -> u32 {
        (self.bits & EpTxError::EP1_MASK) >> EpTxError::EP1_OFFSET
    }

    /// Read **EP0** `[1:0]`
    #[inline(always)]
    pub const fn ep0(&self) -> u32 {
        (self.bits & EpTxError::EP0_MASK) >> EpTxError::EP0_OFFSET
    }

    /// Set **EP15** `[31:30]`
    #[inline(always)]
    pub const fn with_ep15(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EpTxError::EP15_MASK)
                | ((value << EpTxError::EP15_OFFSET) & EpTxError::EP15_MASK),
        }
    }

    /// Set **EP14** `[29:28]`
    #[inline(always)]
    pub const fn with_ep14(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EpTxError::EP14_MASK)
                | ((value << EpTxError::EP14_OFFSET) & EpTxError::EP14_MASK),
        }
    }

    /// Set **EP13** `[27:26]`
    #[inline(always)]
    pub const fn with_ep13(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EpTxError::EP13_MASK)
                | ((value << EpTxError::EP13_OFFSET) & EpTxError::EP13_MASK),
        }
    }

    /// Set **EP12** `[25:24]`
    #[inline(always)]
    pub const fn with_ep12(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EpTxError::EP12_MASK)
                | ((value << EpTxError::EP12_OFFSET) & EpTxError::EP12_MASK),
        }
    }

    /// Set **EP11** `[23:22]`
    #[inline(always)]
    pub const fn with_ep11(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EpTxError::EP11_MASK)
                | ((value << EpTxError::EP11_OFFSET) & EpTxError::EP11_MASK),
        }
    }

    /// Set **EP10** `[21:20]`
    #[inline(always)]
    pub const fn with_ep10(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EpTxError::EP10_MASK)
                | ((value << EpTxError::EP10_OFFSET) & EpTxError::EP10_MASK),
        }
    }

    /// Set **EP9** `[19:18]`
    #[inline(always)]
    pub const fn with_ep9(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EpTxError::EP9_MASK)
                | ((value << EpTxError::EP9_OFFSET) & EpTxError::EP9_MASK),
        }
    }

    /// Set **EP8** `[17:16]`
    #[inline(always)]
    pub const fn with_ep8(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EpTxError::EP8_MASK)
                | ((value << EpTxError::EP8_OFFSET) & EpTxError::EP8_MASK),
        }
    }

    /// Set **EP7** `[15:14]`
    #[inline(always)]
    pub const fn with_ep7(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EpTxError::EP7_MASK)
                | ((value << EpTxError::EP7_OFFSET) & EpTxError::EP7_MASK),
        }
    }

    /// Set **EP6** `[13:12]`
    #[inline(always)]
    pub const fn with_ep6(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EpTxError::EP6_MASK)
                | ((value << EpTxError::EP6_OFFSET) & EpTxError::EP6_MASK),
        }
    }

    /// Set **EP5** `[11:10]`
    #[inline(always)]
    pub const fn with_ep5(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EpTxError::EP5_MASK)
                | ((value << EpTxError::EP5_OFFSET) & EpTxError::EP5_MASK),
        }
    }

    /// Set **EP4** `[9:8]`
    #[inline(always)]
    pub const fn with_ep4(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EpTxError::EP4_MASK)
                | ((value << EpTxError::EP4_OFFSET) & EpTxError::EP4_MASK),
        }
    }

    /// Set **EP3** `[7:6]`
    #[inline(always)]
    pub const fn with_ep3(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EpTxError::EP3_MASK)
                | ((value << EpTxError::EP3_OFFSET) & EpTxError::EP3_MASK),
        }
    }

    /// Set **EP2** `[5:4]`
    #[inline(always)]
    pub const fn with_ep2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EpTxError::EP2_MASK)
                | ((value << EpTxError::EP2_OFFSET) & EpTxError::EP2_MASK),
        }
    }

    /// Set **EP1** `[3:2]`
    #[inline(always)]
    pub const fn with_ep1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EpTxError::EP1_MASK)
                | ((value << EpTxError::EP1_OFFSET) & EpTxError::EP1_MASK),
        }
    }

    /// Set **EP0** `[1:0]`
    #[inline(always)]
    pub const fn with_ep0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EpTxError::EP0_MASK)
                | ((value << EpTxError::EP0_OFFSET) & EpTxError::EP0_MASK),
        }
    }
}

impl Default for EpTxErrorVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl EpTxError {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EpTxErrorVal {
        EpTxErrorVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EpTxErrorVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EpTxErrorVal) -> EpTxErrorVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// EP_RX_ERROR Register
///
/// RX error count for each endpoint. Write to each field to reset the counter to 0.
///
/// Address offset: 0x0110
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **EP15_SEQ** `[31]` (RW)
///
/// - **EP15_TRANSACTION** `[30]` (RW)
///
/// - **EP14_SEQ** `[29]` (RW)
///
/// - **EP14_TRANSACTION** `[28]` (RW)
///
/// - **EP13_SEQ** `[27]` (RW)
///
/// - **EP13_TRANSACTION** `[26]` (RW)
///
/// - **EP12_SEQ** `[25]` (RW)
///
/// - **EP12_TRANSACTION** `[24]` (RW)
///
/// - **EP11_SEQ** `[23]` (RW)
///
/// - **EP11_TRANSACTION** `[22]` (RW)
///
/// - **EP10_SEQ** `[21]` (RW)
///
/// - **EP10_TRANSACTION** `[20]` (RW)
///
/// - **EP9_SEQ** `[19]` (RW)
///
/// - **EP9_TRANSACTION** `[18]` (RW)
///
/// - **EP8_SEQ** `[17]` (RW)
///
/// - **EP8_TRANSACTION** `[16]` (RW)
///
/// - **EP7_SEQ** `[15]` (RW)
///
/// - **EP7_TRANSACTION** `[14]` (RW)
///
/// - **EP6_SEQ** `[13]` (RW)
///
/// - **EP6_TRANSACTION** `[12]` (RW)
///
/// - **EP5_SEQ** `[11]` (RW)
///
/// - **EP5_TRANSACTION** `[10]` (RW)
///
/// - **EP4_SEQ** `[9]` (RW)
///
/// - **EP4_TRANSACTION** `[8]` (RW)
///
/// - **EP3_SEQ** `[7]` (RW)
///
/// - **EP3_TRANSACTION** `[6]` (RW)
///
/// - **EP2_SEQ** `[5]` (RW)
///
/// - **EP2_TRANSACTION** `[4]` (RW)
///
/// - **EP1_SEQ** `[3]` (RW)
///
/// - **EP1_TRANSACTION** `[2]` (RW)
///
/// - **EP0_SEQ** `[1]` (RW)
///
/// - **EP0_TRANSACTION** `[0]` (RW)
#[repr(transparent)]
pub struct EpRxError {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for EpRxError {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl EpRxError {
    // ======== Field Constants ========

    /// **EP15_SEQ**
    /// Bits `[31:31]`, mask `0x80000000`
    pub const EP15_SEQ_OFFSET: u32 = 31;
    pub const EP15_SEQ_MASK: u32 = 0x80000000;
    pub const EP15_SEQ: u32 = 1 << 31;

    /// **EP15_TRANSACTION**
    /// Bits `[30:30]`, mask `0x40000000`
    pub const EP15_TRANSACTION_OFFSET: u32 = 30;
    pub const EP15_TRANSACTION_MASK: u32 = 0x40000000;
    pub const EP15_TRANSACTION: u32 = 1 << 30;

    /// **EP14_SEQ**
    /// Bits `[29:29]`, mask `0x20000000`
    pub const EP14_SEQ_OFFSET: u32 = 29;
    pub const EP14_SEQ_MASK: u32 = 0x20000000;
    pub const EP14_SEQ: u32 = 1 << 29;

    /// **EP14_TRANSACTION**
    /// Bits `[28:28]`, mask `0x10000000`
    pub const EP14_TRANSACTION_OFFSET: u32 = 28;
    pub const EP14_TRANSACTION_MASK: u32 = 0x10000000;
    pub const EP14_TRANSACTION: u32 = 1 << 28;

    /// **EP13_SEQ**
    /// Bits `[27:27]`, mask `0x08000000`
    pub const EP13_SEQ_OFFSET: u32 = 27;
    pub const EP13_SEQ_MASK: u32 = 0x08000000;
    pub const EP13_SEQ: u32 = 1 << 27;

    /// **EP13_TRANSACTION**
    /// Bits `[26:26]`, mask `0x04000000`
    pub const EP13_TRANSACTION_OFFSET: u32 = 26;
    pub const EP13_TRANSACTION_MASK: u32 = 0x04000000;
    pub const EP13_TRANSACTION: u32 = 1 << 26;

    /// **EP12_SEQ**
    /// Bits `[25:25]`, mask `0x02000000`
    pub const EP12_SEQ_OFFSET: u32 = 25;
    pub const EP12_SEQ_MASK: u32 = 0x02000000;
    pub const EP12_SEQ: u32 = 1 << 25;

    /// **EP12_TRANSACTION**
    /// Bits `[24:24]`, mask `0x01000000`
    pub const EP12_TRANSACTION_OFFSET: u32 = 24;
    pub const EP12_TRANSACTION_MASK: u32 = 0x01000000;
    pub const EP12_TRANSACTION: u32 = 1 << 24;

    /// **EP11_SEQ**
    /// Bits `[23:23]`, mask `0x00800000`
    pub const EP11_SEQ_OFFSET: u32 = 23;
    pub const EP11_SEQ_MASK: u32 = 0x00800000;
    pub const EP11_SEQ: u32 = 1 << 23;

    /// **EP11_TRANSACTION**
    /// Bits `[22:22]`, mask `0x00400000`
    pub const EP11_TRANSACTION_OFFSET: u32 = 22;
    pub const EP11_TRANSACTION_MASK: u32 = 0x00400000;
    pub const EP11_TRANSACTION: u32 = 1 << 22;

    /// **EP10_SEQ**
    /// Bits `[21:21]`, mask `0x00200000`
    pub const EP10_SEQ_OFFSET: u32 = 21;
    pub const EP10_SEQ_MASK: u32 = 0x00200000;
    pub const EP10_SEQ: u32 = 1 << 21;

    /// **EP10_TRANSACTION**
    /// Bits `[20:20]`, mask `0x00100000`
    pub const EP10_TRANSACTION_OFFSET: u32 = 20;
    pub const EP10_TRANSACTION_MASK: u32 = 0x00100000;
    pub const EP10_TRANSACTION: u32 = 1 << 20;

    /// **EP9_SEQ**
    /// Bits `[19:19]`, mask `0x00080000`
    pub const EP9_SEQ_OFFSET: u32 = 19;
    pub const EP9_SEQ_MASK: u32 = 0x00080000;
    pub const EP9_SEQ: u32 = 1 << 19;

    /// **EP9_TRANSACTION**
    /// Bits `[18:18]`, mask `0x00040000`
    pub const EP9_TRANSACTION_OFFSET: u32 = 18;
    pub const EP9_TRANSACTION_MASK: u32 = 0x00040000;
    pub const EP9_TRANSACTION: u32 = 1 << 18;

    /// **EP8_SEQ**
    /// Bits `[17:17]`, mask `0x00020000`
    pub const EP8_SEQ_OFFSET: u32 = 17;
    pub const EP8_SEQ_MASK: u32 = 0x00020000;
    pub const EP8_SEQ: u32 = 1 << 17;

    /// **EP8_TRANSACTION**
    /// Bits `[16:16]`, mask `0x00010000`
    pub const EP8_TRANSACTION_OFFSET: u32 = 16;
    pub const EP8_TRANSACTION_MASK: u32 = 0x00010000;
    pub const EP8_TRANSACTION: u32 = 1 << 16;

    /// **EP7_SEQ**
    /// Bits `[15:15]`, mask `0x00008000`
    pub const EP7_SEQ_OFFSET: u32 = 15;
    pub const EP7_SEQ_MASK: u32 = 0x00008000;
    pub const EP7_SEQ: u32 = 1 << 15;

    /// **EP7_TRANSACTION**
    /// Bits `[14:14]`, mask `0x00004000`
    pub const EP7_TRANSACTION_OFFSET: u32 = 14;
    pub const EP7_TRANSACTION_MASK: u32 = 0x00004000;
    pub const EP7_TRANSACTION: u32 = 1 << 14;

    /// **EP6_SEQ**
    /// Bits `[13:13]`, mask `0x00002000`
    pub const EP6_SEQ_OFFSET: u32 = 13;
    pub const EP6_SEQ_MASK: u32 = 0x00002000;
    pub const EP6_SEQ: u32 = 1 << 13;

    /// **EP6_TRANSACTION**
    /// Bits `[12:12]`, mask `0x00001000`
    pub const EP6_TRANSACTION_OFFSET: u32 = 12;
    pub const EP6_TRANSACTION_MASK: u32 = 0x00001000;
    pub const EP6_TRANSACTION: u32 = 1 << 12;

    /// **EP5_SEQ**
    /// Bits `[11:11]`, mask `0x00000800`
    pub const EP5_SEQ_OFFSET: u32 = 11;
    pub const EP5_SEQ_MASK: u32 = 0x00000800;
    pub const EP5_SEQ: u32 = 1 << 11;

    /// **EP5_TRANSACTION**
    /// Bits `[10:10]`, mask `0x00000400`
    pub const EP5_TRANSACTION_OFFSET: u32 = 10;
    pub const EP5_TRANSACTION_MASK: u32 = 0x00000400;
    pub const EP5_TRANSACTION: u32 = 1 << 10;

    /// **EP4_SEQ**
    /// Bits `[9:9]`, mask `0x00000200`
    pub const EP4_SEQ_OFFSET: u32 = 9;
    pub const EP4_SEQ_MASK: u32 = 0x00000200;
    pub const EP4_SEQ: u32 = 1 << 9;

    /// **EP4_TRANSACTION**
    /// Bits `[8:8]`, mask `0x00000100`
    pub const EP4_TRANSACTION_OFFSET: u32 = 8;
    pub const EP4_TRANSACTION_MASK: u32 = 0x00000100;
    pub const EP4_TRANSACTION: u32 = 1 << 8;

    /// **EP3_SEQ**
    /// Bits `[7:7]`, mask `0x00000080`
    pub const EP3_SEQ_OFFSET: u32 = 7;
    pub const EP3_SEQ_MASK: u32 = 0x00000080;
    pub const EP3_SEQ: u32 = 1 << 7;

    /// **EP3_TRANSACTION**
    /// Bits `[6:6]`, mask `0x00000040`
    pub const EP3_TRANSACTION_OFFSET: u32 = 6;
    pub const EP3_TRANSACTION_MASK: u32 = 0x00000040;
    pub const EP3_TRANSACTION: u32 = 1 << 6;

    /// **EP2_SEQ**
    /// Bits `[5:5]`, mask `0x00000020`
    pub const EP2_SEQ_OFFSET: u32 = 5;
    pub const EP2_SEQ_MASK: u32 = 0x00000020;
    pub const EP2_SEQ: u32 = 1 << 5;

    /// **EP2_TRANSACTION**
    /// Bits `[4:4]`, mask `0x00000010`
    pub const EP2_TRANSACTION_OFFSET: u32 = 4;
    pub const EP2_TRANSACTION_MASK: u32 = 0x00000010;
    pub const EP2_TRANSACTION: u32 = 1 << 4;

    /// **EP1_SEQ**
    /// Bits `[3:3]`, mask `0x00000008`
    pub const EP1_SEQ_OFFSET: u32 = 3;
    pub const EP1_SEQ_MASK: u32 = 0x00000008;
    pub const EP1_SEQ: u32 = 1 << 3;

    /// **EP1_TRANSACTION**
    /// Bits `[2:2]`, mask `0x00000004`
    pub const EP1_TRANSACTION_OFFSET: u32 = 2;
    pub const EP1_TRANSACTION_MASK: u32 = 0x00000004;
    pub const EP1_TRANSACTION: u32 = 1 << 2;

    /// **EP0_SEQ**
    /// Bits `[1:1]`, mask `0x00000002`
    pub const EP0_SEQ_OFFSET: u32 = 1;
    pub const EP0_SEQ_MASK: u32 = 0x00000002;
    pub const EP0_SEQ: u32 = 1 << 1;

    /// **EP0_TRANSACTION**
    /// Bits `[0:0]`, mask `0x00000001`
    pub const EP0_TRANSACTION_OFFSET: u32 = 0;
    pub const EP0_TRANSACTION_MASK: u32 = 0x00000001;
    pub const EP0_TRANSACTION: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **EP15_SEQ** `[31]` (RW)
    #[inline(always)]
    pub fn ep15_seq(&self) -> bool {
        (self.value.read_bits() & Self::EP15_SEQ_MASK) != 0
    }

    /// Set EP15_SEQ bit atomically (RW)
    #[inline(always)]
    pub fn set_ep15_seq(&self) {
        self.set_bits(Self::EP15_SEQ_MASK);
    }

    /// Clear EP15_SEQ bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep15_seq(&self) {
        self.clr_bits(Self::EP15_SEQ_MASK);
    }

    /// **EP15_TRANSACTION** `[30]` (RW)
    #[inline(always)]
    pub fn ep15_transaction(&self) -> bool {
        (self.value.read_bits() & Self::EP15_TRANSACTION_MASK) != 0
    }

    /// Set EP15_TRANSACTION bit atomically (RW)
    #[inline(always)]
    pub fn set_ep15_transaction(&self) {
        self.set_bits(Self::EP15_TRANSACTION_MASK);
    }

    /// Clear EP15_TRANSACTION bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep15_transaction(&self) {
        self.clr_bits(Self::EP15_TRANSACTION_MASK);
    }

    /// **EP14_SEQ** `[29]` (RW)
    #[inline(always)]
    pub fn ep14_seq(&self) -> bool {
        (self.value.read_bits() & Self::EP14_SEQ_MASK) != 0
    }

    /// Set EP14_SEQ bit atomically (RW)
    #[inline(always)]
    pub fn set_ep14_seq(&self) {
        self.set_bits(Self::EP14_SEQ_MASK);
    }

    /// Clear EP14_SEQ bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep14_seq(&self) {
        self.clr_bits(Self::EP14_SEQ_MASK);
    }

    /// **EP14_TRANSACTION** `[28]` (RW)
    #[inline(always)]
    pub fn ep14_transaction(&self) -> bool {
        (self.value.read_bits() & Self::EP14_TRANSACTION_MASK) != 0
    }

    /// Set EP14_TRANSACTION bit atomically (RW)
    #[inline(always)]
    pub fn set_ep14_transaction(&self) {
        self.set_bits(Self::EP14_TRANSACTION_MASK);
    }

    /// Clear EP14_TRANSACTION bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep14_transaction(&self) {
        self.clr_bits(Self::EP14_TRANSACTION_MASK);
    }

    /// **EP13_SEQ** `[27]` (RW)
    #[inline(always)]
    pub fn ep13_seq(&self) -> bool {
        (self.value.read_bits() & Self::EP13_SEQ_MASK) != 0
    }

    /// Set EP13_SEQ bit atomically (RW)
    #[inline(always)]
    pub fn set_ep13_seq(&self) {
        self.set_bits(Self::EP13_SEQ_MASK);
    }

    /// Clear EP13_SEQ bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep13_seq(&self) {
        self.clr_bits(Self::EP13_SEQ_MASK);
    }

    /// **EP13_TRANSACTION** `[26]` (RW)
    #[inline(always)]
    pub fn ep13_transaction(&self) -> bool {
        (self.value.read_bits() & Self::EP13_TRANSACTION_MASK) != 0
    }

    /// Set EP13_TRANSACTION bit atomically (RW)
    #[inline(always)]
    pub fn set_ep13_transaction(&self) {
        self.set_bits(Self::EP13_TRANSACTION_MASK);
    }

    /// Clear EP13_TRANSACTION bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep13_transaction(&self) {
        self.clr_bits(Self::EP13_TRANSACTION_MASK);
    }

    /// **EP12_SEQ** `[25]` (RW)
    #[inline(always)]
    pub fn ep12_seq(&self) -> bool {
        (self.value.read_bits() & Self::EP12_SEQ_MASK) != 0
    }

    /// Set EP12_SEQ bit atomically (RW)
    #[inline(always)]
    pub fn set_ep12_seq(&self) {
        self.set_bits(Self::EP12_SEQ_MASK);
    }

    /// Clear EP12_SEQ bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep12_seq(&self) {
        self.clr_bits(Self::EP12_SEQ_MASK);
    }

    /// **EP12_TRANSACTION** `[24]` (RW)
    #[inline(always)]
    pub fn ep12_transaction(&self) -> bool {
        (self.value.read_bits() & Self::EP12_TRANSACTION_MASK) != 0
    }

    /// Set EP12_TRANSACTION bit atomically (RW)
    #[inline(always)]
    pub fn set_ep12_transaction(&self) {
        self.set_bits(Self::EP12_TRANSACTION_MASK);
    }

    /// Clear EP12_TRANSACTION bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep12_transaction(&self) {
        self.clr_bits(Self::EP12_TRANSACTION_MASK);
    }

    /// **EP11_SEQ** `[23]` (RW)
    #[inline(always)]
    pub fn ep11_seq(&self) -> bool {
        (self.value.read_bits() & Self::EP11_SEQ_MASK) != 0
    }

    /// Set EP11_SEQ bit atomically (RW)
    #[inline(always)]
    pub fn set_ep11_seq(&self) {
        self.set_bits(Self::EP11_SEQ_MASK);
    }

    /// Clear EP11_SEQ bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep11_seq(&self) {
        self.clr_bits(Self::EP11_SEQ_MASK);
    }

    /// **EP11_TRANSACTION** `[22]` (RW)
    #[inline(always)]
    pub fn ep11_transaction(&self) -> bool {
        (self.value.read_bits() & Self::EP11_TRANSACTION_MASK) != 0
    }

    /// Set EP11_TRANSACTION bit atomically (RW)
    #[inline(always)]
    pub fn set_ep11_transaction(&self) {
        self.set_bits(Self::EP11_TRANSACTION_MASK);
    }

    /// Clear EP11_TRANSACTION bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep11_transaction(&self) {
        self.clr_bits(Self::EP11_TRANSACTION_MASK);
    }

    /// **EP10_SEQ** `[21]` (RW)
    #[inline(always)]
    pub fn ep10_seq(&self) -> bool {
        (self.value.read_bits() & Self::EP10_SEQ_MASK) != 0
    }

    /// Set EP10_SEQ bit atomically (RW)
    #[inline(always)]
    pub fn set_ep10_seq(&self) {
        self.set_bits(Self::EP10_SEQ_MASK);
    }

    /// Clear EP10_SEQ bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep10_seq(&self) {
        self.clr_bits(Self::EP10_SEQ_MASK);
    }

    /// **EP10_TRANSACTION** `[20]` (RW)
    #[inline(always)]
    pub fn ep10_transaction(&self) -> bool {
        (self.value.read_bits() & Self::EP10_TRANSACTION_MASK) != 0
    }

    /// Set EP10_TRANSACTION bit atomically (RW)
    #[inline(always)]
    pub fn set_ep10_transaction(&self) {
        self.set_bits(Self::EP10_TRANSACTION_MASK);
    }

    /// Clear EP10_TRANSACTION bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep10_transaction(&self) {
        self.clr_bits(Self::EP10_TRANSACTION_MASK);
    }

    /// **EP9_SEQ** `[19]` (RW)
    #[inline(always)]
    pub fn ep9_seq(&self) -> bool {
        (self.value.read_bits() & Self::EP9_SEQ_MASK) != 0
    }

    /// Set EP9_SEQ bit atomically (RW)
    #[inline(always)]
    pub fn set_ep9_seq(&self) {
        self.set_bits(Self::EP9_SEQ_MASK);
    }

    /// Clear EP9_SEQ bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep9_seq(&self) {
        self.clr_bits(Self::EP9_SEQ_MASK);
    }

    /// **EP9_TRANSACTION** `[18]` (RW)
    #[inline(always)]
    pub fn ep9_transaction(&self) -> bool {
        (self.value.read_bits() & Self::EP9_TRANSACTION_MASK) != 0
    }

    /// Set EP9_TRANSACTION bit atomically (RW)
    #[inline(always)]
    pub fn set_ep9_transaction(&self) {
        self.set_bits(Self::EP9_TRANSACTION_MASK);
    }

    /// Clear EP9_TRANSACTION bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep9_transaction(&self) {
        self.clr_bits(Self::EP9_TRANSACTION_MASK);
    }

    /// **EP8_SEQ** `[17]` (RW)
    #[inline(always)]
    pub fn ep8_seq(&self) -> bool {
        (self.value.read_bits() & Self::EP8_SEQ_MASK) != 0
    }

    /// Set EP8_SEQ bit atomically (RW)
    #[inline(always)]
    pub fn set_ep8_seq(&self) {
        self.set_bits(Self::EP8_SEQ_MASK);
    }

    /// Clear EP8_SEQ bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep8_seq(&self) {
        self.clr_bits(Self::EP8_SEQ_MASK);
    }

    /// **EP8_TRANSACTION** `[16]` (RW)
    #[inline(always)]
    pub fn ep8_transaction(&self) -> bool {
        (self.value.read_bits() & Self::EP8_TRANSACTION_MASK) != 0
    }

    /// Set EP8_TRANSACTION bit atomically (RW)
    #[inline(always)]
    pub fn set_ep8_transaction(&self) {
        self.set_bits(Self::EP8_TRANSACTION_MASK);
    }

    /// Clear EP8_TRANSACTION bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep8_transaction(&self) {
        self.clr_bits(Self::EP8_TRANSACTION_MASK);
    }

    /// **EP7_SEQ** `[15]` (RW)
    #[inline(always)]
    pub fn ep7_seq(&self) -> bool {
        (self.value.read_bits() & Self::EP7_SEQ_MASK) != 0
    }

    /// Set EP7_SEQ bit atomically (RW)
    #[inline(always)]
    pub fn set_ep7_seq(&self) {
        self.set_bits(Self::EP7_SEQ_MASK);
    }

    /// Clear EP7_SEQ bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep7_seq(&self) {
        self.clr_bits(Self::EP7_SEQ_MASK);
    }

    /// **EP7_TRANSACTION** `[14]` (RW)
    #[inline(always)]
    pub fn ep7_transaction(&self) -> bool {
        (self.value.read_bits() & Self::EP7_TRANSACTION_MASK) != 0
    }

    /// Set EP7_TRANSACTION bit atomically (RW)
    #[inline(always)]
    pub fn set_ep7_transaction(&self) {
        self.set_bits(Self::EP7_TRANSACTION_MASK);
    }

    /// Clear EP7_TRANSACTION bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep7_transaction(&self) {
        self.clr_bits(Self::EP7_TRANSACTION_MASK);
    }

    /// **EP6_SEQ** `[13]` (RW)
    #[inline(always)]
    pub fn ep6_seq(&self) -> bool {
        (self.value.read_bits() & Self::EP6_SEQ_MASK) != 0
    }

    /// Set EP6_SEQ bit atomically (RW)
    #[inline(always)]
    pub fn set_ep6_seq(&self) {
        self.set_bits(Self::EP6_SEQ_MASK);
    }

    /// Clear EP6_SEQ bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep6_seq(&self) {
        self.clr_bits(Self::EP6_SEQ_MASK);
    }

    /// **EP6_TRANSACTION** `[12]` (RW)
    #[inline(always)]
    pub fn ep6_transaction(&self) -> bool {
        (self.value.read_bits() & Self::EP6_TRANSACTION_MASK) != 0
    }

    /// Set EP6_TRANSACTION bit atomically (RW)
    #[inline(always)]
    pub fn set_ep6_transaction(&self) {
        self.set_bits(Self::EP6_TRANSACTION_MASK);
    }

    /// Clear EP6_TRANSACTION bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep6_transaction(&self) {
        self.clr_bits(Self::EP6_TRANSACTION_MASK);
    }

    /// **EP5_SEQ** `[11]` (RW)
    #[inline(always)]
    pub fn ep5_seq(&self) -> bool {
        (self.value.read_bits() & Self::EP5_SEQ_MASK) != 0
    }

    /// Set EP5_SEQ bit atomically (RW)
    #[inline(always)]
    pub fn set_ep5_seq(&self) {
        self.set_bits(Self::EP5_SEQ_MASK);
    }

    /// Clear EP5_SEQ bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep5_seq(&self) {
        self.clr_bits(Self::EP5_SEQ_MASK);
    }

    /// **EP5_TRANSACTION** `[10]` (RW)
    #[inline(always)]
    pub fn ep5_transaction(&self) -> bool {
        (self.value.read_bits() & Self::EP5_TRANSACTION_MASK) != 0
    }

    /// Set EP5_TRANSACTION bit atomically (RW)
    #[inline(always)]
    pub fn set_ep5_transaction(&self) {
        self.set_bits(Self::EP5_TRANSACTION_MASK);
    }

    /// Clear EP5_TRANSACTION bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep5_transaction(&self) {
        self.clr_bits(Self::EP5_TRANSACTION_MASK);
    }

    /// **EP4_SEQ** `[9]` (RW)
    #[inline(always)]
    pub fn ep4_seq(&self) -> bool {
        (self.value.read_bits() & Self::EP4_SEQ_MASK) != 0
    }

    /// Set EP4_SEQ bit atomically (RW)
    #[inline(always)]
    pub fn set_ep4_seq(&self) {
        self.set_bits(Self::EP4_SEQ_MASK);
    }

    /// Clear EP4_SEQ bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep4_seq(&self) {
        self.clr_bits(Self::EP4_SEQ_MASK);
    }

    /// **EP4_TRANSACTION** `[8]` (RW)
    #[inline(always)]
    pub fn ep4_transaction(&self) -> bool {
        (self.value.read_bits() & Self::EP4_TRANSACTION_MASK) != 0
    }

    /// Set EP4_TRANSACTION bit atomically (RW)
    #[inline(always)]
    pub fn set_ep4_transaction(&self) {
        self.set_bits(Self::EP4_TRANSACTION_MASK);
    }

    /// Clear EP4_TRANSACTION bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep4_transaction(&self) {
        self.clr_bits(Self::EP4_TRANSACTION_MASK);
    }

    /// **EP3_SEQ** `[7]` (RW)
    #[inline(always)]
    pub fn ep3_seq(&self) -> bool {
        (self.value.read_bits() & Self::EP3_SEQ_MASK) != 0
    }

    /// Set EP3_SEQ bit atomically (RW)
    #[inline(always)]
    pub fn set_ep3_seq(&self) {
        self.set_bits(Self::EP3_SEQ_MASK);
    }

    /// Clear EP3_SEQ bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep3_seq(&self) {
        self.clr_bits(Self::EP3_SEQ_MASK);
    }

    /// **EP3_TRANSACTION** `[6]` (RW)
    #[inline(always)]
    pub fn ep3_transaction(&self) -> bool {
        (self.value.read_bits() & Self::EP3_TRANSACTION_MASK) != 0
    }

    /// Set EP3_TRANSACTION bit atomically (RW)
    #[inline(always)]
    pub fn set_ep3_transaction(&self) {
        self.set_bits(Self::EP3_TRANSACTION_MASK);
    }

    /// Clear EP3_TRANSACTION bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep3_transaction(&self) {
        self.clr_bits(Self::EP3_TRANSACTION_MASK);
    }

    /// **EP2_SEQ** `[5]` (RW)
    #[inline(always)]
    pub fn ep2_seq(&self) -> bool {
        (self.value.read_bits() & Self::EP2_SEQ_MASK) != 0
    }

    /// Set EP2_SEQ bit atomically (RW)
    #[inline(always)]
    pub fn set_ep2_seq(&self) {
        self.set_bits(Self::EP2_SEQ_MASK);
    }

    /// Clear EP2_SEQ bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep2_seq(&self) {
        self.clr_bits(Self::EP2_SEQ_MASK);
    }

    /// **EP2_TRANSACTION** `[4]` (RW)
    #[inline(always)]
    pub fn ep2_transaction(&self) -> bool {
        (self.value.read_bits() & Self::EP2_TRANSACTION_MASK) != 0
    }

    /// Set EP2_TRANSACTION bit atomically (RW)
    #[inline(always)]
    pub fn set_ep2_transaction(&self) {
        self.set_bits(Self::EP2_TRANSACTION_MASK);
    }

    /// Clear EP2_TRANSACTION bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep2_transaction(&self) {
        self.clr_bits(Self::EP2_TRANSACTION_MASK);
    }

    /// **EP1_SEQ** `[3]` (RW)
    #[inline(always)]
    pub fn ep1_seq(&self) -> bool {
        (self.value.read_bits() & Self::EP1_SEQ_MASK) != 0
    }

    /// Set EP1_SEQ bit atomically (RW)
    #[inline(always)]
    pub fn set_ep1_seq(&self) {
        self.set_bits(Self::EP1_SEQ_MASK);
    }

    /// Clear EP1_SEQ bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep1_seq(&self) {
        self.clr_bits(Self::EP1_SEQ_MASK);
    }

    /// **EP1_TRANSACTION** `[2]` (RW)
    #[inline(always)]
    pub fn ep1_transaction(&self) -> bool {
        (self.value.read_bits() & Self::EP1_TRANSACTION_MASK) != 0
    }

    /// Set EP1_TRANSACTION bit atomically (RW)
    #[inline(always)]
    pub fn set_ep1_transaction(&self) {
        self.set_bits(Self::EP1_TRANSACTION_MASK);
    }

    /// Clear EP1_TRANSACTION bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep1_transaction(&self) {
        self.clr_bits(Self::EP1_TRANSACTION_MASK);
    }

    /// **EP0_SEQ** `[1]` (RW)
    #[inline(always)]
    pub fn ep0_seq(&self) -> bool {
        (self.value.read_bits() & Self::EP0_SEQ_MASK) != 0
    }

    /// Set EP0_SEQ bit atomically (RW)
    #[inline(always)]
    pub fn set_ep0_seq(&self) {
        self.set_bits(Self::EP0_SEQ_MASK);
    }

    /// Clear EP0_SEQ bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep0_seq(&self) {
        self.clr_bits(Self::EP0_SEQ_MASK);
    }

    /// **EP0_TRANSACTION** `[0]` (RW)
    #[inline(always)]
    pub fn ep0_transaction(&self) -> bool {
        (self.value.read_bits() & Self::EP0_TRANSACTION_MASK) != 0
    }

    /// Set EP0_TRANSACTION bit atomically (RW)
    #[inline(always)]
    pub fn set_ep0_transaction(&self) {
        self.set_bits(Self::EP0_TRANSACTION_MASK);
    }

    /// Clear EP0_TRANSACTION bit atomically (RW)
    #[inline(always)]
    pub fn clr_ep0_transaction(&self) {
        self.clr_bits(Self::EP0_TRANSACTION_MASK);
    }
}

/// EP_RX_ERROR value type for batch read/modify/write operations
///
/// Use `eprxerror.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `eprxerror.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EpRxErrorVal {
    bits: u32,
}

impl EpRxErrorVal {
    /// Create from raw bits
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

    /// Read **EP15_SEQ** `[31:31]`
    #[inline(always)]
    pub const fn ep15_seq(&self) -> bool {
        (self.bits & EpRxError::EP15_SEQ_MASK) != 0
    }

    /// Read **EP15_TRANSACTION** `[30:30]`
    #[inline(always)]
    pub const fn ep15_transaction(&self) -> bool {
        (self.bits & EpRxError::EP15_TRANSACTION_MASK) != 0
    }

    /// Read **EP14_SEQ** `[29:29]`
    #[inline(always)]
    pub const fn ep14_seq(&self) -> bool {
        (self.bits & EpRxError::EP14_SEQ_MASK) != 0
    }

    /// Read **EP14_TRANSACTION** `[28:28]`
    #[inline(always)]
    pub const fn ep14_transaction(&self) -> bool {
        (self.bits & EpRxError::EP14_TRANSACTION_MASK) != 0
    }

    /// Read **EP13_SEQ** `[27:27]`
    #[inline(always)]
    pub const fn ep13_seq(&self) -> bool {
        (self.bits & EpRxError::EP13_SEQ_MASK) != 0
    }

    /// Read **EP13_TRANSACTION** `[26:26]`
    #[inline(always)]
    pub const fn ep13_transaction(&self) -> bool {
        (self.bits & EpRxError::EP13_TRANSACTION_MASK) != 0
    }

    /// Read **EP12_SEQ** `[25:25]`
    #[inline(always)]
    pub const fn ep12_seq(&self) -> bool {
        (self.bits & EpRxError::EP12_SEQ_MASK) != 0
    }

    /// Read **EP12_TRANSACTION** `[24:24]`
    #[inline(always)]
    pub const fn ep12_transaction(&self) -> bool {
        (self.bits & EpRxError::EP12_TRANSACTION_MASK) != 0
    }

    /// Read **EP11_SEQ** `[23:23]`
    #[inline(always)]
    pub const fn ep11_seq(&self) -> bool {
        (self.bits & EpRxError::EP11_SEQ_MASK) != 0
    }

    /// Read **EP11_TRANSACTION** `[22:22]`
    #[inline(always)]
    pub const fn ep11_transaction(&self) -> bool {
        (self.bits & EpRxError::EP11_TRANSACTION_MASK) != 0
    }

    /// Read **EP10_SEQ** `[21:21]`
    #[inline(always)]
    pub const fn ep10_seq(&self) -> bool {
        (self.bits & EpRxError::EP10_SEQ_MASK) != 0
    }

    /// Read **EP10_TRANSACTION** `[20:20]`
    #[inline(always)]
    pub const fn ep10_transaction(&self) -> bool {
        (self.bits & EpRxError::EP10_TRANSACTION_MASK) != 0
    }

    /// Read **EP9_SEQ** `[19:19]`
    #[inline(always)]
    pub const fn ep9_seq(&self) -> bool {
        (self.bits & EpRxError::EP9_SEQ_MASK) != 0
    }

    /// Read **EP9_TRANSACTION** `[18:18]`
    #[inline(always)]
    pub const fn ep9_transaction(&self) -> bool {
        (self.bits & EpRxError::EP9_TRANSACTION_MASK) != 0
    }

    /// Read **EP8_SEQ** `[17:17]`
    #[inline(always)]
    pub const fn ep8_seq(&self) -> bool {
        (self.bits & EpRxError::EP8_SEQ_MASK) != 0
    }

    /// Read **EP8_TRANSACTION** `[16:16]`
    #[inline(always)]
    pub const fn ep8_transaction(&self) -> bool {
        (self.bits & EpRxError::EP8_TRANSACTION_MASK) != 0
    }

    /// Read **EP7_SEQ** `[15:15]`
    #[inline(always)]
    pub const fn ep7_seq(&self) -> bool {
        (self.bits & EpRxError::EP7_SEQ_MASK) != 0
    }

    /// Read **EP7_TRANSACTION** `[14:14]`
    #[inline(always)]
    pub const fn ep7_transaction(&self) -> bool {
        (self.bits & EpRxError::EP7_TRANSACTION_MASK) != 0
    }

    /// Read **EP6_SEQ** `[13:13]`
    #[inline(always)]
    pub const fn ep6_seq(&self) -> bool {
        (self.bits & EpRxError::EP6_SEQ_MASK) != 0
    }

    /// Read **EP6_TRANSACTION** `[12:12]`
    #[inline(always)]
    pub const fn ep6_transaction(&self) -> bool {
        (self.bits & EpRxError::EP6_TRANSACTION_MASK) != 0
    }

    /// Read **EP5_SEQ** `[11:11]`
    #[inline(always)]
    pub const fn ep5_seq(&self) -> bool {
        (self.bits & EpRxError::EP5_SEQ_MASK) != 0
    }

    /// Read **EP5_TRANSACTION** `[10:10]`
    #[inline(always)]
    pub const fn ep5_transaction(&self) -> bool {
        (self.bits & EpRxError::EP5_TRANSACTION_MASK) != 0
    }

    /// Read **EP4_SEQ** `[9:9]`
    #[inline(always)]
    pub const fn ep4_seq(&self) -> bool {
        (self.bits & EpRxError::EP4_SEQ_MASK) != 0
    }

    /// Read **EP4_TRANSACTION** `[8:8]`
    #[inline(always)]
    pub const fn ep4_transaction(&self) -> bool {
        (self.bits & EpRxError::EP4_TRANSACTION_MASK) != 0
    }

    /// Read **EP3_SEQ** `[7:7]`
    #[inline(always)]
    pub const fn ep3_seq(&self) -> bool {
        (self.bits & EpRxError::EP3_SEQ_MASK) != 0
    }

    /// Read **EP3_TRANSACTION** `[6:6]`
    #[inline(always)]
    pub const fn ep3_transaction(&self) -> bool {
        (self.bits & EpRxError::EP3_TRANSACTION_MASK) != 0
    }

    /// Read **EP2_SEQ** `[5:5]`
    #[inline(always)]
    pub const fn ep2_seq(&self) -> bool {
        (self.bits & EpRxError::EP2_SEQ_MASK) != 0
    }

    /// Read **EP2_TRANSACTION** `[4:4]`
    #[inline(always)]
    pub const fn ep2_transaction(&self) -> bool {
        (self.bits & EpRxError::EP2_TRANSACTION_MASK) != 0
    }

    /// Read **EP1_SEQ** `[3:3]`
    #[inline(always)]
    pub const fn ep1_seq(&self) -> bool {
        (self.bits & EpRxError::EP1_SEQ_MASK) != 0
    }

    /// Read **EP1_TRANSACTION** `[2:2]`
    #[inline(always)]
    pub const fn ep1_transaction(&self) -> bool {
        (self.bits & EpRxError::EP1_TRANSACTION_MASK) != 0
    }

    /// Read **EP0_SEQ** `[1:1]`
    #[inline(always)]
    pub const fn ep0_seq(&self) -> bool {
        (self.bits & EpRxError::EP0_SEQ_MASK) != 0
    }

    /// Read **EP0_TRANSACTION** `[0:0]`
    #[inline(always)]
    pub const fn ep0_transaction(&self) -> bool {
        (self.bits & EpRxError::EP0_TRANSACTION_MASK) != 0
    }

    /// Set **EP15_SEQ** `[31:31]`
    #[inline(always)]
    pub const fn with_ep15_seq(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpRxError::EP15_SEQ_MASK
            } else {
                self.bits & !EpRxError::EP15_SEQ_MASK
            },
        }
    }

    /// Set **EP15_TRANSACTION** `[30:30]`
    #[inline(always)]
    pub const fn with_ep15_transaction(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpRxError::EP15_TRANSACTION_MASK
            } else {
                self.bits & !EpRxError::EP15_TRANSACTION_MASK
            },
        }
    }

    /// Set **EP14_SEQ** `[29:29]`
    #[inline(always)]
    pub const fn with_ep14_seq(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpRxError::EP14_SEQ_MASK
            } else {
                self.bits & !EpRxError::EP14_SEQ_MASK
            },
        }
    }

    /// Set **EP14_TRANSACTION** `[28:28]`
    #[inline(always)]
    pub const fn with_ep14_transaction(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpRxError::EP14_TRANSACTION_MASK
            } else {
                self.bits & !EpRxError::EP14_TRANSACTION_MASK
            },
        }
    }

    /// Set **EP13_SEQ** `[27:27]`
    #[inline(always)]
    pub const fn with_ep13_seq(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpRxError::EP13_SEQ_MASK
            } else {
                self.bits & !EpRxError::EP13_SEQ_MASK
            },
        }
    }

    /// Set **EP13_TRANSACTION** `[26:26]`
    #[inline(always)]
    pub const fn with_ep13_transaction(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpRxError::EP13_TRANSACTION_MASK
            } else {
                self.bits & !EpRxError::EP13_TRANSACTION_MASK
            },
        }
    }

    /// Set **EP12_SEQ** `[25:25]`
    #[inline(always)]
    pub const fn with_ep12_seq(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpRxError::EP12_SEQ_MASK
            } else {
                self.bits & !EpRxError::EP12_SEQ_MASK
            },
        }
    }

    /// Set **EP12_TRANSACTION** `[24:24]`
    #[inline(always)]
    pub const fn with_ep12_transaction(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpRxError::EP12_TRANSACTION_MASK
            } else {
                self.bits & !EpRxError::EP12_TRANSACTION_MASK
            },
        }
    }

    /// Set **EP11_SEQ** `[23:23]`
    #[inline(always)]
    pub const fn with_ep11_seq(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpRxError::EP11_SEQ_MASK
            } else {
                self.bits & !EpRxError::EP11_SEQ_MASK
            },
        }
    }

    /// Set **EP11_TRANSACTION** `[22:22]`
    #[inline(always)]
    pub const fn with_ep11_transaction(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpRxError::EP11_TRANSACTION_MASK
            } else {
                self.bits & !EpRxError::EP11_TRANSACTION_MASK
            },
        }
    }

    /// Set **EP10_SEQ** `[21:21]`
    #[inline(always)]
    pub const fn with_ep10_seq(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpRxError::EP10_SEQ_MASK
            } else {
                self.bits & !EpRxError::EP10_SEQ_MASK
            },
        }
    }

    /// Set **EP10_TRANSACTION** `[20:20]`
    #[inline(always)]
    pub const fn with_ep10_transaction(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpRxError::EP10_TRANSACTION_MASK
            } else {
                self.bits & !EpRxError::EP10_TRANSACTION_MASK
            },
        }
    }

    /// Set **EP9_SEQ** `[19:19]`
    #[inline(always)]
    pub const fn with_ep9_seq(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpRxError::EP9_SEQ_MASK
            } else {
                self.bits & !EpRxError::EP9_SEQ_MASK
            },
        }
    }

    /// Set **EP9_TRANSACTION** `[18:18]`
    #[inline(always)]
    pub const fn with_ep9_transaction(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpRxError::EP9_TRANSACTION_MASK
            } else {
                self.bits & !EpRxError::EP9_TRANSACTION_MASK
            },
        }
    }

    /// Set **EP8_SEQ** `[17:17]`
    #[inline(always)]
    pub const fn with_ep8_seq(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpRxError::EP8_SEQ_MASK
            } else {
                self.bits & !EpRxError::EP8_SEQ_MASK
            },
        }
    }

    /// Set **EP8_TRANSACTION** `[16:16]`
    #[inline(always)]
    pub const fn with_ep8_transaction(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpRxError::EP8_TRANSACTION_MASK
            } else {
                self.bits & !EpRxError::EP8_TRANSACTION_MASK
            },
        }
    }

    /// Set **EP7_SEQ** `[15:15]`
    #[inline(always)]
    pub const fn with_ep7_seq(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpRxError::EP7_SEQ_MASK
            } else {
                self.bits & !EpRxError::EP7_SEQ_MASK
            },
        }
    }

    /// Set **EP7_TRANSACTION** `[14:14]`
    #[inline(always)]
    pub const fn with_ep7_transaction(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpRxError::EP7_TRANSACTION_MASK
            } else {
                self.bits & !EpRxError::EP7_TRANSACTION_MASK
            },
        }
    }

    /// Set **EP6_SEQ** `[13:13]`
    #[inline(always)]
    pub const fn with_ep6_seq(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpRxError::EP6_SEQ_MASK
            } else {
                self.bits & !EpRxError::EP6_SEQ_MASK
            },
        }
    }

    /// Set **EP6_TRANSACTION** `[12:12]`
    #[inline(always)]
    pub const fn with_ep6_transaction(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpRxError::EP6_TRANSACTION_MASK
            } else {
                self.bits & !EpRxError::EP6_TRANSACTION_MASK
            },
        }
    }

    /// Set **EP5_SEQ** `[11:11]`
    #[inline(always)]
    pub const fn with_ep5_seq(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpRxError::EP5_SEQ_MASK
            } else {
                self.bits & !EpRxError::EP5_SEQ_MASK
            },
        }
    }

    /// Set **EP5_TRANSACTION** `[10:10]`
    #[inline(always)]
    pub const fn with_ep5_transaction(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpRxError::EP5_TRANSACTION_MASK
            } else {
                self.bits & !EpRxError::EP5_TRANSACTION_MASK
            },
        }
    }

    /// Set **EP4_SEQ** `[9:9]`
    #[inline(always)]
    pub const fn with_ep4_seq(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpRxError::EP4_SEQ_MASK
            } else {
                self.bits & !EpRxError::EP4_SEQ_MASK
            },
        }
    }

    /// Set **EP4_TRANSACTION** `[8:8]`
    #[inline(always)]
    pub const fn with_ep4_transaction(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpRxError::EP4_TRANSACTION_MASK
            } else {
                self.bits & !EpRxError::EP4_TRANSACTION_MASK
            },
        }
    }

    /// Set **EP3_SEQ** `[7:7]`
    #[inline(always)]
    pub const fn with_ep3_seq(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpRxError::EP3_SEQ_MASK
            } else {
                self.bits & !EpRxError::EP3_SEQ_MASK
            },
        }
    }

    /// Set **EP3_TRANSACTION** `[6:6]`
    #[inline(always)]
    pub const fn with_ep3_transaction(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpRxError::EP3_TRANSACTION_MASK
            } else {
                self.bits & !EpRxError::EP3_TRANSACTION_MASK
            },
        }
    }

    /// Set **EP2_SEQ** `[5:5]`
    #[inline(always)]
    pub const fn with_ep2_seq(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpRxError::EP2_SEQ_MASK
            } else {
                self.bits & !EpRxError::EP2_SEQ_MASK
            },
        }
    }

    /// Set **EP2_TRANSACTION** `[4:4]`
    #[inline(always)]
    pub const fn with_ep2_transaction(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpRxError::EP2_TRANSACTION_MASK
            } else {
                self.bits & !EpRxError::EP2_TRANSACTION_MASK
            },
        }
    }

    /// Set **EP1_SEQ** `[3:3]`
    #[inline(always)]
    pub const fn with_ep1_seq(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpRxError::EP1_SEQ_MASK
            } else {
                self.bits & !EpRxError::EP1_SEQ_MASK
            },
        }
    }

    /// Set **EP1_TRANSACTION** `[2:2]`
    #[inline(always)]
    pub const fn with_ep1_transaction(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpRxError::EP1_TRANSACTION_MASK
            } else {
                self.bits & !EpRxError::EP1_TRANSACTION_MASK
            },
        }
    }

    /// Set **EP0_SEQ** `[1:1]`
    #[inline(always)]
    pub const fn with_ep0_seq(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpRxError::EP0_SEQ_MASK
            } else {
                self.bits & !EpRxError::EP0_SEQ_MASK
            },
        }
    }

    /// Set **EP0_TRANSACTION** `[0:0]`
    #[inline(always)]
    pub const fn with_ep0_transaction(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EpRxError::EP0_TRANSACTION_MASK
            } else {
                self.bits & !EpRxError::EP0_TRANSACTION_MASK
            },
        }
    }
}

impl Default for EpRxErrorVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl EpRxError {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EpRxErrorVal {
        EpRxErrorVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EpRxErrorVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EpRxErrorVal) -> EpRxErrorVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DEV_SM_WATCHDOG Register
///
/// Watchdog that forces the device state machine to idle and raises an interrupt if the device stays in a state that isn't idle for the configured limit. The counter is reset on every state transition.
/// Set limit while enable is low and then set the enable.
///
/// Address offset: 0x0114
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **FIRED** `[20]` (RW)
///
/// - **RESET** `[19]` (RW)
///
///   Set to 1 to forcibly reset the device state machine on watchdog expiry
///
/// - **ENABLE** `[18]` (RW)
///
/// - **LIMIT** `[17:0]` (RW)
#[repr(transparent)]
pub struct DevSmWatchdog {
    value: crate::common::AtomicCell<u32>,
}

impl core::ops::Deref for DevSmWatchdog {
    type Target = crate::common::AtomicCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DevSmWatchdog {
    // ======== Field Constants ========

    /// **FIRED**
    /// Bits `[20:20]`, mask `0x00100000`
    pub const FIRED_OFFSET: u32 = 20;
    pub const FIRED_MASK: u32 = 0x00100000;
    pub const FIRED: u32 = 1 << 20;

    /// **RESET**
    ///
    /// Set to 1 to forcibly reset the device state machine on watchdog expiry
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const RESET_OFFSET: u32 = 19;
    pub const RESET_MASK: u32 = 0x00080000;
    pub const RESET: u32 = 1 << 19;

    /// **ENABLE**
    /// Bits `[18:18]`, mask `0x00040000`
    pub const ENABLE_OFFSET: u32 = 18;
    pub const ENABLE_MASK: u32 = 0x00040000;
    pub const ENABLE: u32 = 1 << 18;

    /// **LIMIT**
    /// Bits `[17:0]`, mask `0x0003ffff`
    pub const LIMIT_OFFSET: u32 = 0;
    pub const LIMIT_MASK: u32 = 0x0003ffff;

    // ======== Field Accessors ========

    /// **FIRED** `[20]` (RW)
    #[inline(always)]
    pub fn fired(&self) -> bool {
        (self.value.read_bits() & Self::FIRED_MASK) != 0
    }

    /// Set FIRED bit atomically (RW)
    #[inline(always)]
    pub fn set_fired(&self) {
        self.set_bits(Self::FIRED_MASK);
    }

    /// Clear FIRED bit atomically (RW)
    #[inline(always)]
    pub fn clr_fired(&self) {
        self.clr_bits(Self::FIRED_MASK);
    }

    /// **RESET** `[19]` (RW)
    ///
    /// Set to 1 to forcibly reset the device state machine on watchdog expiry
    #[inline(always)]
    pub fn reset(&self) -> bool {
        (self.value.read_bits() & Self::RESET_MASK) != 0
    }

    /// Set RESET bit atomically (RW)
    #[inline(always)]
    pub fn set_reset(&self) {
        self.set_bits(Self::RESET_MASK);
    }

    /// Clear RESET bit atomically (RW)
    #[inline(always)]
    pub fn clr_reset(&self) {
        self.clr_bits(Self::RESET_MASK);
    }

    /// **ENABLE** `[18]` (RW)
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

    /// **LIMIT** `[17:0]` (RW)
    #[inline(always)]
    pub fn limit(&self) -> u32 {
        (self.value.read_bits() & Self::LIMIT_MASK) >> Self::LIMIT_OFFSET
    }

    /// Write **LIMIT** `[17:0]` (RW)
    #[inline(always)]
    pub fn write_limit(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LIMIT_MASK) | ((value << Self::LIMIT_OFFSET) & Self::LIMIT_MASK)
        });
    }
}

/// DEV_SM_WATCHDOG value type for batch read/modify/write operations
///
/// Use `devsmwatchdog.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `devsmwatchdog.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DevSmWatchdogVal {
    bits: u32,
}

impl DevSmWatchdogVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **FIRED** `[20:20]`
    #[inline(always)]
    pub const fn fired(&self) -> bool {
        (self.bits & DevSmWatchdog::FIRED_MASK) != 0
    }

    /// Read **RESET** `[19:19]`
    ///
    /// Set to 1 to forcibly reset the device state machine on watchdog expiry
    #[inline(always)]
    pub const fn reset(&self) -> bool {
        (self.bits & DevSmWatchdog::RESET_MASK) != 0
    }

    /// Read **ENABLE** `[18:18]`
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        (self.bits & DevSmWatchdog::ENABLE_MASK) != 0
    }

    /// Read **LIMIT** `[17:0]`
    #[inline(always)]
    pub const fn limit(&self) -> u32 {
        (self.bits & DevSmWatchdog::LIMIT_MASK) >> DevSmWatchdog::LIMIT_OFFSET
    }

    /// Set **FIRED** `[20:20]`
    #[inline(always)]
    pub const fn with_fired(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DevSmWatchdog::FIRED_MASK
            } else {
                self.bits & !DevSmWatchdog::FIRED_MASK
            },
        }
    }

    /// Set **RESET** `[19:19]`
    ///
    /// Set to 1 to forcibly reset the device state machine on watchdog expiry
    #[inline(always)]
    pub const fn with_reset(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DevSmWatchdog::RESET_MASK
            } else {
                self.bits & !DevSmWatchdog::RESET_MASK
            },
        }
    }

    /// Set **ENABLE** `[18:18]`
    #[inline(always)]
    pub const fn with_enable(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DevSmWatchdog::ENABLE_MASK
            } else {
                self.bits & !DevSmWatchdog::ENABLE_MASK
            },
        }
    }

    /// Set **LIMIT** `[17:0]`
    #[inline(always)]
    pub const fn with_limit(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DevSmWatchdog::LIMIT_MASK)
                | ((value << DevSmWatchdog::LIMIT_OFFSET) & DevSmWatchdog::LIMIT_MASK),
        }
    }
}

impl Default for DevSmWatchdogVal {
    fn default() -> Self {
        Self { bits: 0x00000000 }
    }
}

impl DevSmWatchdog {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DevSmWatchdogVal {
        DevSmWatchdogVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DevSmWatchdogVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DevSmWatchdogVal) -> DevSmWatchdogVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// Device address and endpoint control
    pub addr_endp: AddrEndp,
    /// 0x0004
    /// Interrupt endpoint 1. Only valid for HOST mode.
    pub addr_endp1: AddrEndp1,
    /// 0x0008
    /// Interrupt endpoint 2. Only valid for HOST mode.
    pub addr_endp2: AddrEndp2,
    /// 0x000c
    /// Interrupt endpoint 3. Only valid for HOST mode.
    pub addr_endp3: AddrEndp3,
    /// 0x0010
    /// Interrupt endpoint 4. Only valid for HOST mode.
    pub addr_endp4: AddrEndp4,
    /// 0x0014
    /// Interrupt endpoint 5. Only valid for HOST mode.
    pub addr_endp5: AddrEndp5,
    /// 0x0018
    /// Interrupt endpoint 6. Only valid for HOST mode.
    pub addr_endp6: AddrEndp6,
    /// 0x001c
    /// Interrupt endpoint 7. Only valid for HOST mode.
    pub addr_endp7: AddrEndp7,
    /// 0x0020
    /// Interrupt endpoint 8. Only valid for HOST mode.
    pub addr_endp8: AddrEndp8,
    /// 0x0024
    /// Interrupt endpoint 9. Only valid for HOST mode.
    pub addr_endp9: AddrEndp9,
    /// 0x0028
    /// Interrupt endpoint 10. Only valid for HOST mode.
    pub addr_endp10: AddrEndp10,
    /// 0x002c
    /// Interrupt endpoint 11. Only valid for HOST mode.
    pub addr_endp11: AddrEndp11,
    /// 0x0030
    /// Interrupt endpoint 12. Only valid for HOST mode.
    pub addr_endp12: AddrEndp12,
    /// 0x0034
    /// Interrupt endpoint 13. Only valid for HOST mode.
    pub addr_endp13: AddrEndp13,
    /// 0x0038
    /// Interrupt endpoint 14. Only valid for HOST mode.
    pub addr_endp14: AddrEndp14,
    /// 0x003c
    /// Interrupt endpoint 15. Only valid for HOST mode.
    pub addr_endp15: AddrEndp15,
    /// 0x0040
    /// Main control register
    pub main_ctrl: MainCtrl,
    /// 0x0044
    /// Set the SOF (Start of Frame) frame number in the host controller. The SOF packet is sent every 1ms and the host will increment the frame number by 1 each time.
    pub sof_wr: SofWr,
    /// 0x0048
    /// Read the last SOF (Start of Frame) frame number seen. In device mode the last SOF received from the host. In host mode the last SOF sent by the host.
    pub sof_rd: SofRd,
    /// 0x004c
    /// SIE control register
    pub sie_ctrl: SieCtrl,
    /// 0x0050
    /// SIE status register
    pub sie_status: SieStatus,
    /// 0x0054
    /// interrupt endpoint control register
    pub int_ep_ctrl: IntEpCtrl,
    /// 0x0058
    /// Buffer status register. A bit set here indicates that a buffer has completed on the endpoint (if the buffer interrupt is enabled). It is possible for 2 buffers to be completed, so clearing the buffer status bit may instantly re set it on the next clock cycle.
    pub buff_status: BuffStatus,
    /// 0x005c
    /// Which of the double buffers should be handled. Only valid if using an interrupt per buffer (i.e. not per 2 buffers). Not valid for host interrupt endpoint polling because they are only single buffered.
    pub buff_cpu_should_handle: BuffCpuShouldHandle,
    /// 0x0060
    /// Device only: Can be set to ignore the buffer control register for this endpoint in case you would like to revoke a buffer. A NAK will be sent for every access to the endpoint until this bit is cleared. A corresponding bit in `EP_ABORT_DONE` is set when it is safe to modify the buffer control register.
    pub ep_abort: EpAbort,
    /// 0x0064
    /// Device only: Used in conjunction with `EP_ABORT`. Set once an endpoint is idle so the programmer knows it is safe to modify the buffer control register.
    pub ep_abort_done: EpAbortDone,
    /// 0x0068
    /// Device: this bit must be set in conjunction with the `STALL` bit in the buffer control register to send a STALL on EP0. The device controller clears these bits when a SETUP packet is received because the USB spec requires that a STALL condition is cleared when a SETUP packet is received.
    pub ep_stall_arm: EpStallArm,
    /// 0x006c
    /// Used by the host controller. Sets the wait time in microseconds before trying again if the device replies with a NAK.
    pub nak_poll: NakPoll,
    /// 0x0070
    /// Device: bits are set when the `IRQ_ON_NAK` or `IRQ_ON_STALL` bits are set. For EP0 this comes from `SIE_CTRL`. For all other endpoints it comes from the endpoint control register.
    pub ep_status_stall_nak: EpStatusStallNak,
    /// 0x0074
    /// Where to connect the USB controller. Should be to_phy by default.
    pub usb_muxing: UsbMuxing,
    /// 0x0078
    /// Overrides for the power signals in the event that the VBUS signals are not hooked up to GPIO. Set the value of the override and then the override enable to switch over to the override value.
    pub usb_pwr: UsbPwr,
    /// 0x007c
    /// This register allows for direct control of the USB phy. Use in conjunction with usbphy_direct_override register to enable each override bit.
    pub usbphy_direct: UsbphyDirect,
    /// 0x0080
    /// Override enable for each control in usbphy_direct
    pub usbphy_direct_override: UsbphyDirectOverride,
    /// 0x0084
    /// Used to adjust trim values of USB phy pull down resistors.
    pub usbphy_trim: UsbphyTrim,
    /// 0x0088
    /// Used for debug only.
    pub linestate_tuning: LinestateTuning,
    /// 0x008c
    /// Raw Interrupts
    pub intr: Intr,
    /// 0x0090
    /// Interrupt Enable
    pub inte: Inte,
    /// 0x0094
    /// Interrupt Force
    pub intf: Intf,
    /// 0x0098
    /// Interrupt status after masking & forcing
    pub ints: Ints,
    _reserved0: [u8; 100],
    /// 0x0100
    /// Device only. Raw value of free-running PHY clock counter @48MHz. Used to calculate time between SOF events.
    pub sof_timestamp_raw: SofTimestampRaw,
    /// 0x0104
    /// Device only. Value of free-running PHY clock counter @48MHz when last SOF event occurred.
    pub sof_timestamp_last: SofTimestampLast,
    /// 0x0108
    pub sm_state: SmState,
    /// 0x010c
    /// TX error count for each endpoint. Write to each field to reset the counter to 0.
    pub ep_tx_error: EpTxError,
    /// 0x0110
    /// RX error count for each endpoint. Write to each field to reset the counter to 0.
    pub ep_rx_error: EpRxError,
    /// 0x0114
    /// Watchdog that forces the device state machine to idle and raises an interrupt if the device stays in a state that isn't idle for the configured limit. The counter is reset on every state transition.
    /// Set limit while enable is low and then set the enable.
    pub dev_sm_watchdog: DevSmWatchdog,
}
