//! RP2350 Boot Block definitions
//!
//! The Boot ROM looks for a valid Block in the first 4KB of flash to determine
//! how to boot the image.

/// Magic value marking the start of a block
pub const BLOCK_MARKER_START: u32 = 0xffffded3;

/// Magic value marking the end of a block
pub const BLOCK_MARKER_END: u32 = 0xab123579;

// Image type item constants
const ITEM_1BS_IMAGE_TYPE: u8 = 0x42;
const IMAGE_TYPE_EXE: u16 = 0x0001;
const IMAGE_TYPE_EXE_CHIP_RP2350: u16 = 0x1000;
const IMAGE_TYPE_EXE_CPU_ARM: u16 = 0x0000;
const IMAGE_TYPE_EXE_TYPE_SECURITY_NS: u16 = 0x0010;
const IMAGE_TYPE_EXE_TYPE_SECURITY_S: u16 = 0x0020;

/// Make an item containing a tag, 1 byte length and two extra bytes
const fn item_generic_1bs(value: u16, length: u8, command: u8) -> u32 {
    ((value as u32) << 16) | ((length as u32) << 8) | (command as u32)
}

/// Make an item for LAST with 2-byte size
const fn item_last(length: u16) -> u32 {
    ((length as u32) << 8) | 0xff
}

/// Create the IMAGE_TYPE item for a secure ARM executable
const fn item_image_type_exe_arm(secure: bool) -> u32 {
    let security = if secure {
        IMAGE_TYPE_EXE_TYPE_SECURITY_S
    } else {
        IMAGE_TYPE_EXE_TYPE_SECURITY_NS
    };
    let value = IMAGE_TYPE_EXE | IMAGE_TYPE_EXE_CHIP_RP2350 | IMAGE_TYPE_EXE_CPU_ARM | security;
    item_generic_1bs(value, 1, ITEM_1BS_IMAGE_TYPE)
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
    pub const fn secure_exe() -> Self {
        Self {
            marker_start: BLOCK_MARKER_START,
            image_type: item_image_type_exe_arm(true),
            length: item_last(1),
            offset: core::ptr::null(),
            marker_end: BLOCK_MARKER_END,
        }
    }

    /// Construct a new Image Definition for a non-secure ARM executable
    pub const fn non_secure_exe() -> Self {
        Self {
            marker_start: BLOCK_MARKER_START,
            image_type: item_image_type_exe_arm(false),
            length: item_last(1),
            offset: core::ptr::null(),
            marker_end: BLOCK_MARKER_END,
        }
    }
}
