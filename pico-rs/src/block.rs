//! RP2350 Boot Block (PICOBIN) definitions
//!
//! The Boot ROM looks for a valid Block in the first 4KB of flash to determine
//! how to boot the image. The block format is defined by the PICOBIN specification.
//!
//! ## Usage
//!
//! Every RP2350 application needs an Image Definition block in the `.start_block` section:
//!
//! ```ignore
//! use pico_rs::block::ImageDef;
//!
//! #[link_section = ".start_block"]
//! #[used]
//! pub static IMAGE_DEF: ImageDef = ImageDef::secure_exe();
//! ```
//!
//! ## Security Modes
//!
//! - `secure_exe()` - Secure mode (TrustZone-M enabled, default)
//! - `non_secure_exe()` - Non-secure mode
//!
//! ## Architecture Support
//!
//! Currently this module supports ARM Cortex-M33 images. RISC-V support
//! can be added by using `ImageDef::riscv_exe()`.

#![allow(dead_code)]

/// Magic value marking the start of a block (PICOBIN_BLOCK_MARKER_START)
pub const BLOCK_MARKER_START: u32 = 0xffffded3;

/// Magic value marking the end of a block (PICOBIN_BLOCK_MARKER_END)
pub const BLOCK_MARKER_END: u32 = 0xab123579;

// Block item type identifiers
/// 1-byte-size item: IMAGE_TYPE
const ITEM_1BS_IMAGE_TYPE: u8 = 0x42;
/// 1-byte-size item: VECTOR_TABLE
const ITEM_1BS_VECTOR_TABLE: u8 = 0x03;
/// 1-byte-size item: ENTRY_POINT
const ITEM_1BS_ENTRY_POINT: u8 = 0x44;
/// 1-byte-size item: VERSION
const ITEM_1BS_VERSION: u8 = 0x48;
/// 2-byte-size item: LAST (end of block)
const ITEM_2BS_LAST: u8 = 0xff;

// Image type field values
/// Image type: executable
const IMAGE_TYPE_EXE: u16 = 0x0001;
/// Image type: data only
const IMAGE_TYPE_DATA: u16 = 0x0002;

// Security mode (bits 4-5)
/// Security: unspecified
const IMAGE_TYPE_EXE_SECURITY_UNSPECIFIED: u16 = 0x0000;
/// Security: non-secure
const IMAGE_TYPE_EXE_SECURITY_NS: u16 = 0x0010;
/// Security: secure
const IMAGE_TYPE_EXE_SECURITY_S: u16 = 0x0020;

// CPU type (bits 8-10)
/// CPU: ARM Cortex-M33
const IMAGE_TYPE_EXE_CPU_ARM: u16 = 0x0000;
/// CPU: RISC-V
const IMAGE_TYPE_EXE_CPU_RISCV: u16 = 0x0100;
/// CPU: Varmulet (legacy compatibility)
const IMAGE_TYPE_EXE_CPU_VARMULET: u16 = 0x0200;

// Chip type (bits 12-14)
/// Chip: RP2040
const IMAGE_TYPE_EXE_CHIP_RP2040: u16 = 0x0000;
/// Chip: RP2350
const IMAGE_TYPE_EXE_CHIP_RP2350: u16 = 0x1000;

/// Make an item header with 1-byte size (tag | size<<8 | value<<16)
const fn item_1bs(value: u16, size: u8, tag: u8) -> u32 {
    ((value as u32) << 16) | ((size as u32) << 8) | (tag as u32)
}

/// Make an item header with 2-byte size (tag | size<<8)
const fn item_2bs(size: u16, tag: u8) -> u32 {
    ((size as u32) << 8) | (tag as u32)
}

/// Create the IMAGE_TYPE item for an ARM executable
const fn item_image_type_exe_arm(security: u16) -> u32 {
    let value = IMAGE_TYPE_EXE | IMAGE_TYPE_EXE_CHIP_RP2350 | IMAGE_TYPE_EXE_CPU_ARM | security;
    item_1bs(value, 1, ITEM_1BS_IMAGE_TYPE)
}

/// Create the IMAGE_TYPE item for a RISC-V executable
const fn item_image_type_exe_riscv() -> u32 {
    let value = IMAGE_TYPE_EXE
        | IMAGE_TYPE_EXE_CHIP_RP2350
        | IMAGE_TYPE_EXE_CPU_RISCV
        | IMAGE_TYPE_EXE_SECURITY_UNSPECIFIED;
    item_1bs(value, 1, ITEM_1BS_IMAGE_TYPE)
}

/// RP2350 Image Definition Block
///
/// This structure must be placed in the `.start_block` section in the first 4K of flash.
/// It tells the Boot ROM that this is a valid executable image.
///
/// # Example
///
/// ```ignore
/// #[link_section = ".start_block"]
/// #[used]
/// pub static IMAGE_DEF: ImageDef = ImageDef::secure_exe();
/// ```
#[repr(C)]
pub struct ImageDef {
    marker_start: u32,
    image_type: u32,
    length: u32,
    offset: *const u32,
    marker_end: u32,
}

// SAFETY: ImageDef contains only u32 values and a null pointer, making it safe to share
unsafe impl Sync for ImageDef {}

impl ImageDef {
    /// Construct a new Image Definition for a secure ARM executable
    ///
    /// This is the default and recommended mode for most applications.
    /// TrustZone-M security extensions are enabled.
    pub const fn secure_exe() -> Self {
        Self {
            marker_start: BLOCK_MARKER_START,
            image_type: item_image_type_exe_arm(IMAGE_TYPE_EXE_SECURITY_S),
            length: item_2bs(1, ITEM_2BS_LAST),
            offset: core::ptr::null(),
            marker_end: BLOCK_MARKER_END,
        }
    }

    /// Construct a new Image Definition for a non-secure ARM executable
    ///
    /// Use this when running without TrustZone-M security extensions.
    pub const fn non_secure_exe() -> Self {
        Self {
            marker_start: BLOCK_MARKER_START,
            image_type: item_image_type_exe_arm(IMAGE_TYPE_EXE_SECURITY_NS),
            length: item_2bs(1, ITEM_2BS_LAST),
            offset: core::ptr::null(),
            marker_end: BLOCK_MARKER_END,
        }
    }

    /// Construct a new Image Definition for a RISC-V executable
    ///
    /// Use this for RISC-V based applications on RP2350.
    pub const fn riscv_exe() -> Self {
        Self {
            marker_start: BLOCK_MARKER_START,
            image_type: item_image_type_exe_riscv(),
            length: item_2bs(1, ITEM_2BS_LAST),
            offset: core::ptr::null(),
            marker_end: BLOCK_MARKER_END,
        }
    }
}

/// Extended Image Definition Block with version information
///
/// Use this when you want to include version metadata in your binary.
#[repr(C)]
pub struct ImageDefWithVersion {
    marker_start: u32,
    image_type: u32,
    version: u32,
    version_data: [u32; 2],
    length: u32,
    offset: *const u32,
    marker_end: u32,
}

unsafe impl Sync for ImageDefWithVersion {}

impl ImageDefWithVersion {
    /// Construct an Image Definition with version for a secure ARM executable
    ///
    /// # Arguments
    /// * `major` - Major version number (0-65535)
    /// * `minor` - Minor version number (0-65535)
    /// * `rollback` - Rollback version (optional, for OTP anti-rollback)
    pub const fn secure_exe_with_version(major: u16, minor: u16, rollback: u32) -> Self {
        Self {
            marker_start: BLOCK_MARKER_START,
            image_type: item_image_type_exe_arm(IMAGE_TYPE_EXE_SECURITY_S),
            version: item_1bs(0, 3, ITEM_1BS_VERSION),
            version_data: [((minor as u32) << 16) | (major as u32), rollback],
            length: item_2bs(4, ITEM_2BS_LAST),
            offset: core::ptr::null(),
            marker_end: BLOCK_MARKER_END,
        }
    }
}

/// Boot configuration constants
pub mod boot_config {
    /// Default flash sector size (4K)
    pub const FLASH_SECTOR_SIZE: u32 = 4096;

    /// Maximum block size
    pub const MAX_BLOCK_SIZE: u32 = 0x280;

    /// Maximum image definition block size
    pub const MAX_IMAGE_DEF_BLOCK_SIZE: u32 = 0x180;
}
