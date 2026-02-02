//! Memory Protection Unit (MPU)
//!
//! The MPU provides memory protection by dividing memory into regions with
//! individual access permissions. On Cortex-M33 (ARMv8-M), the MPU supports:
//!
//! - Up to 16 memory regions (RP2350 typically has 8)
//! - Configurable region sizes (minimum 32 bytes, must be power of 2)
//! - Per-region access permissions (read/write, read-only, no access)
//! - Per-region attributes (cacheable, bufferable, shareable)
//! - Background region support
//!
//! # ARMv8-M MPU Differences
//!
//! The ARMv8-M MPU differs from earlier versions:
//! - Regions use base/limit addresses instead of size fields
//! - More flexible region sizing (any multiple of 32 bytes)
//! - Separate Secure and Non-Secure MPUs when TrustZone is enabled

use core::marker::PhantomData;

use super::{Reg32, peripheral};

/// MPU base address
const MPU_BASE: u32 = 0xE000_ED90;

/// MPU Register Block (ARMv8-M)
#[repr(C)]
pub struct RegisterBlock {
    /// MPU Type Register (read-only)
    pub type_: Reg32,
    /// MPU Control Register
    pub ctrl: Reg32,
    /// MPU Region Number Register
    pub rnr: Reg32,
    /// MPU Region Base Address Register
    pub rbar: Reg32,
    /// MPU Region Limit Address Register
    pub rlar: Reg32,
    /// MPU Region Base Address Register Alias 1
    pub rbar_a1: Reg32,
    /// MPU Region Limit Address Register Alias 1
    pub rlar_a1: Reg32,
    /// MPU Region Base Address Register Alias 2
    pub rbar_a2: Reg32,
    /// MPU Region Limit Address Register Alias 2
    pub rlar_a2: Reg32,
    /// MPU Region Base Address Register Alias 3
    pub rbar_a3: Reg32,
    /// MPU Region Limit Address Register Alias 3
    pub rlar_a3: Reg32,
    _reserved: u32,
    /// MPU Memory Attribute Indirection Register 0
    pub mair0: Reg32,
    /// MPU Memory Attribute Indirection Register 1
    pub mair1: Reg32,
}

/// MPU Control Register bits
pub mod ctrl {
    /// MPU Enable bit
    pub const ENABLE: u32 = 1 << 0;
    /// Enable MPU during HardFault and NMI handlers
    pub const HFNMIENA: u32 = 1 << 1;
    /// Enable privileged access to default memory map
    pub const PRIVDEFENA: u32 = 1 << 2;
}

/// Access Permission values for RBAR.AP field
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AccessPermission {
    /// Read/Write by privileged code only
    PrivilegedReadWrite = 0b00,
    /// Read/Write by any privilege level
    ReadWrite = 0b01,
    /// Read-only by privileged code only
    PrivilegedReadOnly = 0b10,
    /// Read-only by any privilege level
    ReadOnly = 0b11,
}

/// Memory attribute index (references MAIR registers)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AttributeIndex {
    /// Attribute Index 0
    Attr0 = 0,
    /// Attribute Index 1
    Attr1 = 1,
    /// Attribute Index 2
    Attr2 = 2,
    /// Attribute Index 3
    Attr3 = 3,
    /// Attribute Index 4
    Attr4 = 4,
    /// Attribute Index 5
    Attr5 = 5,
    /// Attribute Index 6
    Attr6 = 6,
    /// Attribute Index 7
    Attr7 = 7,
}

/// Memory attributes for MAIR register
#[derive(Debug, Clone, Copy)]
pub struct MemoryAttribute(pub u8);

impl MemoryAttribute {
    /// Device memory, nGnRnE (strongly-ordered)
    pub const DEVICE_NGNRNE: Self = Self(0b0000_0000);

    /// Device memory, nGnRE
    pub const DEVICE_NGNRE: Self = Self(0b0000_0100);

    /// Device memory, nGRE
    pub const DEVICE_NGRE: Self = Self(0b0000_1000);

    /// Device memory, GRE
    pub const DEVICE_GRE: Self = Self(0b0000_1100);

    /// Normal memory, Non-cacheable
    pub const NORMAL_NON_CACHEABLE: Self = Self(0b0100_0100);

    /// Normal memory, Write-through, No-allocate
    pub const NORMAL_WT_NA: Self = Self(0b1010_1010);

    /// Normal memory, Write-back, No-allocate
    pub const NORMAL_WB_NA: Self = Self(0b1110_1110);

    /// Normal memory, Write-back, Read-allocate, Write-allocate
    pub const NORMAL_WB_RA_WA: Self = Self(0b1111_1111);

    /// Create a custom memory attribute
    pub const fn custom(outer: u8, inner: u8) -> Self {
        Self((outer << 4) | (inner & 0x0F))
    }
}

/// Shareability domain
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Shareable {
    /// Non-shareable
    None = 0b00,
    /// Outer shareable
    Outer = 0b10,
    /// Inner shareable
    Inner = 0b11,
}

/// MPU Region configuration
#[derive(Debug, Clone, Copy)]
pub struct RegionConfig {
    /// Region number (0-15)
    pub region: u8,
    /// Base address (must be 32-byte aligned)
    pub base: u32,
    /// Limit address (must be 32-byte aligned minus 1)
    pub limit: u32,
    /// Access permission
    pub access: AccessPermission,
    /// Memory attribute index
    pub attr_index: AttributeIndex,
    /// Shareability
    pub shareable: Shareable,
    /// Execute Never - when true, code cannot be executed from this region
    pub execute_never: bool,
    /// Enable the region
    pub enable: bool,
}

/// MPU peripheral
pub struct MPU {
    _marker: PhantomData<*const ()>,
}

peripheral!(MPU, RegisterBlock, MPU_BASE);

impl MPU {
    /// Enable the MPU
    ///
    /// # Arguments
    ///
    /// * `enable_in_fault` - If true, MPU stays enabled during HardFault/NMI
    /// * `enable_background` - If true, privileged code can access default memory map
    #[inline]
    pub fn enable(&mut self, enable_in_fault: bool, enable_background: bool) {
        let mut ctrl_val = ctrl::ENABLE;
        if enable_in_fault {
            ctrl_val |= ctrl::HFNMIENA;
        }
        if enable_background {
            ctrl_val |= ctrl::PRIVDEFENA;
        }
        unsafe {
            (*Self::PTR).ctrl.write(ctrl_val);
        }
        crate::asm::dsb();
        crate::asm::isb();
    }

    /// Disable the MPU
    #[inline]
    pub fn disable(&mut self) {
        unsafe {
            (*Self::PTR).ctrl.write(0);
        }
        crate::asm::dsb();
        crate::asm::isb();
    }

    /// Check if MPU is enabled
    #[inline]
    pub fn is_enabled(&self) -> bool {
        unsafe { (*Self::PTR).ctrl.read() & ctrl::ENABLE != 0 }
    }

    /// Get the number of MPU regions supported
    #[inline]
    pub fn num_regions(&self) -> u8 {
        // DREGION field in MPU_TYPE[15:8]
        ((unsafe { (*Self::PTR).type_.read() } >> 8) & 0xFF) as u8
    }

    /// Set a memory attribute in MAIR registers
    ///
    /// # Arguments
    ///
    /// * `index` - Attribute index (0-7)
    /// * `attr` - Memory attribute value
    #[inline]
    pub fn set_memory_attribute(&mut self, index: AttributeIndex, attr: MemoryAttribute) {
        let idx = index as u8;
        let shift = (idx % 4) * 8;

        unsafe {
            if idx < 4 {
                (*Self::PTR)
                    .mair0
                    .modify(|v| (v & !(0xFF << shift)) | ((attr.0 as u32) << shift));
            } else {
                (*Self::PTR)
                    .mair1
                    .modify(|v| (v & !(0xFF << shift)) | ((attr.0 as u32) << shift));
            }
        }
    }

    /// Configure an MPU region
    ///
    /// # Arguments
    ///
    /// * `config` - Region configuration
    ///
    /// # Panics
    ///
    /// Panics if region number is invalid or addresses are not properly aligned.
    #[inline]
    pub fn configure_region(&mut self, config: &RegionConfig) {
        assert!(
            config.region < self.num_regions(),
            "Invalid MPU region number"
        );
        assert!(
            config.base & 0x1F == 0,
            "Base address must be 32-byte aligned"
        );
        assert!(
            (config.limit + 1) & 0x1F == 0,
            "Limit+1 must be 32-byte aligned"
        );

        unsafe {
            // Select region
            (*Self::PTR).rnr.write(config.region as u32);

            // Build RBAR value
            // [31:5] = Base address
            // [4:3] = SH (Shareability)
            // [2:1] = AP (Access Permission)
            // [0] = XN (Execute Never)
            let mut rbar_val = config.base & !0x1F;
            rbar_val |= (config.shareable as u32) << 3;
            rbar_val |= (config.access as u32) << 1;
            if config.execute_never {
                rbar_val |= 1;
            }
            (*Self::PTR).rbar.write(rbar_val);

            // Build RLAR value
            // [31:5] = Limit address
            // [4] = PXN (Privileged Execute Never) - not used here
            // [3:1] = AttrIndx (Attribute Index)
            // [0] = EN (Enable)
            let mut rlar_val = config.limit & !0x1F;
            rlar_val |= (config.attr_index as u32) << 1;
            if config.enable {
                rlar_val |= 1;
            }
            (*Self::PTR).rlar.write(rlar_val);
        }

        crate::asm::dsb();
        crate::asm::isb();
    }

    /// Disable an MPU region
    #[inline]
    pub fn disable_region(&mut self, region: u8) {
        assert!(region < self.num_regions(), "Invalid MPU region number");

        unsafe {
            (*Self::PTR).rnr.write(region as u32);
            (*Self::PTR).rlar.modify(|v| v & !1);
        }

        crate::asm::dsb();
        crate::asm::isb();
    }

    /// Configure multiple regions at once using the alias registers
    ///
    /// This is more efficient than configuring regions one at a time.
    /// Configures up to 4 consecutive regions starting at `start_region`.
    ///
    /// # Arguments
    ///
    /// * `start_region` - First region number
    /// * `configs` - Up to 4 region configurations (must be consecutive)
    #[inline]
    pub fn configure_regions(&mut self, start_region: u8, configs: &[RegionConfig]) {
        assert!(
            configs.len() <= 4,
            "Can only configure up to 4 regions at once"
        );
        assert!(
            start_region + configs.len() as u8 <= self.num_regions(),
            "Region range exceeds available regions"
        );

        unsafe {
            // Select starting region
            (*Self::PTR).rnr.write(start_region as u32);

            // Configure each region using the alias registers
            for (i, config) in configs.iter().enumerate() {
                assert!(
                    config.region == start_region + i as u8,
                    "Regions must be consecutive"
                );

                let mut rbar_val = config.base & !0x1F;
                rbar_val |= (config.shareable as u32) << 3;
                rbar_val |= (config.access as u32) << 1;
                if config.execute_never {
                    rbar_val |= 1;
                }

                let mut rlar_val = config.limit & !0x1F;
                rlar_val |= (config.attr_index as u32) << 1;
                if config.enable {
                    rlar_val |= 1;
                }

                match i {
                    0 => {
                        (*Self::PTR).rbar.write(rbar_val);
                        (*Self::PTR).rlar.write(rlar_val);
                    }
                    1 => {
                        (*Self::PTR).rbar_a1.write(rbar_val);
                        (*Self::PTR).rlar_a1.write(rlar_val);
                    }
                    2 => {
                        (*Self::PTR).rbar_a2.write(rbar_val);
                        (*Self::PTR).rlar_a2.write(rlar_val);
                    }
                    3 => {
                        (*Self::PTR).rbar_a3.write(rbar_val);
                        (*Self::PTR).rlar_a3.write(rlar_val);
                    }
                    _ => unreachable!(),
                }
            }
        }

        crate::asm::dsb();
        crate::asm::isb();
    }
}

// Safety: MPU is a single peripheral that should only be accessed from one place
unsafe impl Send for MPU {}
