//! TBMAN
//! For managing simulation testbenches
//!
//! This peripheral supports atomic SET/CLR/XOR register aliases.

#![allow(dead_code)]

pub const BASE: u32 = 0x40160000;

/// PLATFORM Register
///
/// Indicates the type of platform in use
///
/// Address offset: 0x0000
/// Reset value: 0x00000001
///
/// ## Fields
///
/// - **HDLSIM** `[2]` (RO)
///
///   Indicates the platform is a simulation
///
/// - **FPGA** `[1]` (RO)
///
///   Indicates the platform is an FPGA
///
/// - **ASIC** `[0]` (RO)
///
///   Indicates the platform is an ASIC
#[repr(transparent)]
pub struct Platform {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Platform {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Platform {
    // ======== Field Constants ========

    /// **HDLSIM**
    ///
    /// Indicates the platform is a simulation
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const HDLSIM_OFFSET: u32 = 2;
    pub const HDLSIM_MASK: u32 = 0x00000004;
    pub const HDLSIM: u32 = 1 << 2;

    /// **FPGA**
    ///
    /// Indicates the platform is an FPGA
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const FPGA_OFFSET: u32 = 1;
    pub const FPGA_MASK: u32 = 0x00000002;
    pub const FPGA: u32 = 1 << 1;

    /// **ASIC**
    ///
    /// Indicates the platform is an ASIC
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const ASIC_OFFSET: u32 = 0;
    pub const ASIC_MASK: u32 = 0x00000001;
    pub const ASIC: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **HDLSIM** `[2]` (RO)
    ///
    /// Indicates the platform is a simulation
    #[inline(always)]
    pub fn hdlsim(&self) -> bool {
        (self.value.read_bits() & Self::HDLSIM_MASK) != 0
    }

    /// **FPGA** `[1]` (RO)
    ///
    /// Indicates the platform is an FPGA
    #[inline(always)]
    pub fn fpga(&self) -> bool {
        (self.value.read_bits() & Self::FPGA_MASK) != 0
    }

    /// **ASIC** `[0]` (RO)
    ///
    /// Indicates the platform is an ASIC
    #[inline(always)]
    pub fn asic(&self) -> bool {
        (self.value.read_bits() & Self::ASIC_MASK) != 0
    }
}

/// PLATFORM value type for batch read/modify/write operations
///
/// Use `platform.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `platform.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PlatformVal {
    bits: u32,
}

impl PlatformVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
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

    /// Read **HDLSIM** `[2:2]`
    ///
    /// Indicates the platform is a simulation
    #[inline(always)]
    pub const fn hdlsim(&self) -> bool {
        (self.bits & Platform::HDLSIM_MASK) != 0
    }

    /// Read **FPGA** `[1:1]`
    ///
    /// Indicates the platform is an FPGA
    #[inline(always)]
    pub const fn fpga(&self) -> bool {
        (self.bits & Platform::FPGA_MASK) != 0
    }

    /// Read **ASIC** `[0:0]`
    ///
    /// Indicates the platform is an ASIC
    #[inline(always)]
    pub const fn asic(&self) -> bool {
        (self.bits & Platform::ASIC_MASK) != 0
    }
}

impl Default for PlatformVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Platform {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PlatformVal {
        PlatformVal::from_bits(self.value.read_bits())
    }
}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// Indicates the type of platform in use
    pub platform: Platform,
}
