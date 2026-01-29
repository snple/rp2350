//! TBMAN - For managing simulation testbenches

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
/// - **HDLSIM** [2] (RO): Indicates the platform is a simulation
/// - **FPGA** [1] (RO): Indicates the platform is an FPGA
/// - **ASIC** [0] (RO): Indicates the platform is an ASIC
#[repr(transparent)]
pub struct Platform {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for Platform {}
unsafe impl Sync for Platform {}

impl Platform {
    // ======== Field Constants ========

    /// HDLSIM - Indicates the platform is a simulation
    /// Bits [2:2], mask 0x00000004
    pub const HDLSIM_OFFSET: u32 = 2;
    pub const HDLSIM_MASK: u32 = 0x00000004;
    pub const HDLSIM: u32 = 1 << 2;

    /// FPGA - Indicates the platform is an FPGA
    /// Bits [1:1], mask 0x00000002
    pub const FPGA_OFFSET: u32 = 1;
    pub const FPGA_MASK: u32 = 0x00000002;
    pub const FPGA: u32 = 1 << 1;

    /// ASIC - Indicates the platform is an ASIC
    /// Bits [0:0], mask 0x00000001
    pub const ASIC_OFFSET: u32 = 0;
    pub const ASIC_MASK: u32 = 0x00000001;
    pub const ASIC: u32 = 1 << 0;

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

    /// **HDLSIM** [2:2]: Indicates the platform is a simulation
    #[inline(always)]
    pub fn hdlsim(&self) -> bool {
        (self.read_bits() & Self::HDLSIM_MASK) != 0
    }

    /// **FPGA** [1:1]: Indicates the platform is an FPGA
    #[inline(always)]
    pub fn fpga(&self) -> bool {
        (self.read_bits() & Self::FPGA_MASK) != 0
    }

    /// **ASIC** [0:0]: Indicates the platform is an ASIC
    #[inline(always)]
    pub fn asic(&self) -> bool {
        (self.read_bits() & Self::ASIC_MASK) != 0
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

    /// Read HDLSIM [2:2]: Indicates the platform is a simulation
    #[inline(always)]
    pub const fn hdlsim(&self) -> bool {
        (self.bits & Platform::HDLSIM_MASK) != 0
    }

    /// Read FPGA [1:1]: Indicates the platform is an FPGA
    #[inline(always)]
    pub const fn fpga(&self) -> bool {
        (self.bits & Platform::FPGA_MASK) != 0
    }

    /// Read ASIC [0:0]: Indicates the platform is an ASIC
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
    pub fn read(&self) -> PlatformVal {
        PlatformVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: PlatformVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(PlatformVal) -> PlatformVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000 - Indicates the type of platform in use
    pub platform: Platform,
}

