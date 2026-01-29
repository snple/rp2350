//! SYSINFO - 

#![allow(dead_code)]

pub const BASE: u32 = 0x40000000;

/// CHIP_ID Register
///
/// JEDEC JEP-106 compliant chip identifier.
///
/// Address offset: 0x0000
/// Reset value: 0x00000001
///
/// ## Fields
///
/// - **REVISION** [31:28] (RO): 
/// - **PART** [27:12] (RO): 
/// - **MANUFACTURER** [11:1] (RO): 
/// - **STOP_BIT** [0] (RO): 
#[repr(transparent)]
pub struct ChipId {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for ChipId {}
unsafe impl Sync for ChipId {}

impl ChipId {
    // ======== Field Constants ========

    /// Bits [31:28], mask 0xf0000000
    pub const REVISION_OFFSET: u32 = 28;
    pub const REVISION_MASK: u32 = 0xf0000000;

    /// Bits [27:12], mask 0x0ffff000
    pub const PART_OFFSET: u32 = 12;
    pub const PART_MASK: u32 = 0x0ffff000;

    /// Bits [11:1], mask 0x00000ffe
    pub const MANUFACTURER_OFFSET: u32 = 1;
    pub const MANUFACTURER_MASK: u32 = 0x00000ffe;

    /// Bits [0:0], mask 0x00000001
    pub const STOP_BIT_OFFSET: u32 = 0;
    pub const STOP_BIT_MASK: u32 = 0x00000001;
    pub const STOP_BIT: u32 = 1 << 0;

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

    /// **REVISION** [31:28]
    #[inline(always)]
    pub fn revision(&self) -> u32 {
        (self.read_bits() & Self::REVISION_MASK) >> Self::REVISION_OFFSET
    }

    /// **PART** [27:12]
    #[inline(always)]
    pub fn part(&self) -> u32 {
        (self.read_bits() & Self::PART_MASK) >> Self::PART_OFFSET
    }

    /// **MANUFACTURER** [11:1]
    #[inline(always)]
    pub fn manufacturer(&self) -> u32 {
        (self.read_bits() & Self::MANUFACTURER_MASK) >> Self::MANUFACTURER_OFFSET
    }

    /// **STOP_BIT** [0:0]
    #[inline(always)]
    pub fn stop_bit(&self) -> bool {
        (self.read_bits() & Self::STOP_BIT_MASK) != 0
    }

}

/// CHIP_ID value type for batch read/modify/write operations
///
/// Use `chipid.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `chipid.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ChipIdVal {
    bits: u32,
}

impl ChipIdVal {
    /// Create from raw bits
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

    /// Read REVISION [31:28]
    #[inline(always)]
    pub const fn revision(&self) -> u32 {
        (self.bits & ChipId::REVISION_MASK) >> ChipId::REVISION_OFFSET
    }

    /// Read PART [27:12]
    #[inline(always)]
    pub const fn part(&self) -> u32 {
        (self.bits & ChipId::PART_MASK) >> ChipId::PART_OFFSET
    }

    /// Read MANUFACTURER [11:1]
    #[inline(always)]
    pub const fn manufacturer(&self) -> u32 {
        (self.bits & ChipId::MANUFACTURER_MASK) >> ChipId::MANUFACTURER_OFFSET
    }

    /// Read STOP_BIT [0:0]
    #[inline(always)]
    pub const fn stop_bit(&self) -> bool {
        (self.bits & ChipId::STOP_BIT_MASK) != 0
    }

}

impl Default for ChipIdVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl ChipId {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> ChipIdVal {
        ChipIdVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: ChipIdVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(ChipIdVal) -> ChipIdVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// PACKAGE_SEL Register
///
/// Address offset: 0x0004
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **PACKAGE_SEL** [0] (RO): 
#[repr(transparent)]
pub struct PackageSel {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for PackageSel {}
unsafe impl Sync for PackageSel {}

impl PackageSel {
    // ======== Field Constants ========

    /// Bits [0:0], mask 0x00000001
    pub const PACKAGE_SEL_OFFSET: u32 = 0;
    pub const PACKAGE_SEL_MASK: u32 = 0x00000001;
    pub const PACKAGE_SEL: u32 = 1 << 0;

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

    /// **PACKAGE_SEL** [0:0]
    #[inline(always)]
    pub fn package_sel(&self) -> bool {
        (self.read_bits() & Self::PACKAGE_SEL_MASK) != 0
    }

}

/// PACKAGE_SEL value type for batch read/modify/write operations
///
/// Use `packagesel.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `packagesel.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PackageSelVal {
    bits: u32,
}

impl PackageSelVal {
    /// Create from raw bits
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

    /// Read PACKAGE_SEL [0:0]
    #[inline(always)]
    pub const fn package_sel(&self) -> bool {
        (self.bits & PackageSel::PACKAGE_SEL_MASK) != 0
    }

}

impl Default for PackageSelVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl PackageSel {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> PackageSelVal {
        PackageSelVal::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: PackageSelVal) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(PackageSelVal) -> PackageSelVal>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// PLATFORM Register
///
/// Platform register. Allows software to know what environment it is running in
/// during pre-production development. Post-production, the PLATFORM is always
/// ASIC, non-SIM.
///
/// Address offset: 0x0008
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **GATESIM** [4] (RO): 
/// - **BATCHSIM** [3] (RO): 
/// - **HDLSIM** [2] (RO): 
/// - **ASIC** [1] (RO): 
/// - **FPGA** [0] (RO): 
#[repr(transparent)]
pub struct Platform {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for Platform {}
unsafe impl Sync for Platform {}

impl Platform {
    // ======== Field Constants ========

    /// Bits [4:4], mask 0x00000010
    pub const GATESIM_OFFSET: u32 = 4;
    pub const GATESIM_MASK: u32 = 0x00000010;
    pub const GATESIM: u32 = 1 << 4;

    /// Bits [3:3], mask 0x00000008
    pub const BATCHSIM_OFFSET: u32 = 3;
    pub const BATCHSIM_MASK: u32 = 0x00000008;
    pub const BATCHSIM: u32 = 1 << 3;

    /// Bits [2:2], mask 0x00000004
    pub const HDLSIM_OFFSET: u32 = 2;
    pub const HDLSIM_MASK: u32 = 0x00000004;
    pub const HDLSIM: u32 = 1 << 2;

    /// Bits [1:1], mask 0x00000002
    pub const ASIC_OFFSET: u32 = 1;
    pub const ASIC_MASK: u32 = 0x00000002;
    pub const ASIC: u32 = 1 << 1;

    /// Bits [0:0], mask 0x00000001
    pub const FPGA_OFFSET: u32 = 0;
    pub const FPGA_MASK: u32 = 0x00000001;
    pub const FPGA: u32 = 1 << 0;

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

    /// **GATESIM** [4:4]
    #[inline(always)]
    pub fn gatesim(&self) -> bool {
        (self.read_bits() & Self::GATESIM_MASK) != 0
    }

    /// **BATCHSIM** [3:3]
    #[inline(always)]
    pub fn batchsim(&self) -> bool {
        (self.read_bits() & Self::BATCHSIM_MASK) != 0
    }

    /// **HDLSIM** [2:2]
    #[inline(always)]
    pub fn hdlsim(&self) -> bool {
        (self.read_bits() & Self::HDLSIM_MASK) != 0
    }

    /// **ASIC** [1:1]
    #[inline(always)]
    pub fn asic(&self) -> bool {
        (self.read_bits() & Self::ASIC_MASK) != 0
    }

    /// **FPGA** [0:0]
    #[inline(always)]
    pub fn fpga(&self) -> bool {
        (self.read_bits() & Self::FPGA_MASK) != 0
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
        Self { bits: 0x00000000 }
    }

    /// Read GATESIM [4:4]
    #[inline(always)]
    pub const fn gatesim(&self) -> bool {
        (self.bits & Platform::GATESIM_MASK) != 0
    }

    /// Read BATCHSIM [3:3]
    #[inline(always)]
    pub const fn batchsim(&self) -> bool {
        (self.bits & Platform::BATCHSIM_MASK) != 0
    }

    /// Read HDLSIM [2:2]
    #[inline(always)]
    pub const fn hdlsim(&self) -> bool {
        (self.bits & Platform::HDLSIM_MASK) != 0
    }

    /// Read ASIC [1:1]
    #[inline(always)]
    pub const fn asic(&self) -> bool {
        (self.bits & Platform::ASIC_MASK) != 0
    }

    /// Read FPGA [0:0]
    #[inline(always)]
    pub const fn fpga(&self) -> bool {
        (self.bits & Platform::FPGA_MASK) != 0
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

/// GITREF_RP2350 Register
///
/// Git hash of the chip source. Used to identify chip version.
///
/// Address offset: 0x0014
///
/// ## Fields
///
/// - **GITREF_RP2350** [31:0] (RO): 
#[repr(transparent)]
pub struct GitrefRp2350 {
    value: core::cell::UnsafeCell<u32>,
}

unsafe impl Send for GitrefRp2350 {}
unsafe impl Sync for GitrefRp2350 {}

impl GitrefRp2350 {
    // ======== Field Constants ========

    /// Bits [31:0], mask 0xffffffff
    pub const GITREF_RP2350_OFFSET: u32 = 0;
    pub const GITREF_RP2350_MASK: u32 = 0xffffffff;

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

    /// **GITREF_RP2350** [31:0]
    #[inline(always)]
    pub fn gitref_rp2350(&self) -> u32 {
        (self.read_bits() & Self::GITREF_RP2350_MASK) >> Self::GITREF_RP2350_OFFSET
    }

}

/// GITREF_RP2350 value type for batch read/modify/write operations
///
/// Use `gitrefrp2350.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gitrefrp2350.write_val(val)`.
#[derive(Clone, Copy)]
pub struct GitrefRp2350Val {
    bits: u32,
}

impl GitrefRp2350Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read GITREF_RP2350 [31:0]
    #[inline(always)]
    pub const fn gitref_rp2350(&self) -> u32 {
        (self.bits & GitrefRp2350::GITREF_RP2350_MASK) >> GitrefRp2350::GITREF_RP2350_OFFSET
    }

}

impl Default for GitrefRp2350Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl GitrefRp2350 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read(&self) -> GitrefRp2350Val {
        GitrefRp2350Val::from_bits(self.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write(&self, val: GitrefRp2350Val) {
        self.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify<F: FnOnce(GitrefRp2350Val) -> GitrefRp2350Val>(&self, f: F) {
        self.write(f(self.read()))
    }

}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000 - JEDEC JEP-106 compliant chip identifier.
    pub chip_id: ChipId,
    /// 0x0004
    pub package_sel: PackageSel,
    /// 0x0008 - Platform register. Allows software to know what environment it is running i...
    pub platform: Platform,
    _reserved0: [u8; 8],
    /// 0x0014 - Git hash of the chip source. Used to identify chip version.
    pub gitref_rp2350: GitrefRp2350,
}

