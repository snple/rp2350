//! `BINARY_INFO` entry naming the linked boot2 variant.
//!
//! This mirrors the layout used by `rp2350::binary_info` (and therefore the Pico SDK's
//! `bi_decl(bi_string(BINARY_INFO_TAG_RASPBERRY_PI, BINARY_INFO_ID_RP_BOOT2_NAME, ...))`):
//! a [`BinaryInfoIdAndString`] record placed in `.bi_entries`, which the linker script
//! inserts after `.text`.
//!
//! The struct is duplicated here rather than imported from the HAL because the HAL sits
//! above `rp2350-rt` in the dependency graph, while this crate sits below it. The mapping
//! is checked by eye against `rp2350/src/binary_info.rs`.

/// Tag marking an entry as defined by Raspberry Pi (`'R' | 'P' << 8`).
///
/// `rp2350::binary_info::TAG_RASPBERRY_PI`.
const TAG_RASPBERRY_PI: u16 = (b'P' as u16) << 8 | (b'R' as u16);

/// `rp2350::binary_info::BinaryInfoType::IdAndString`.
const BI_TYPE_ID_AND_STRING: u16 = 6;

/// `rp2350::binary_info::ID_RP_BOOT2_NAME`.
const ID_RP_BOOT2_NAME: u32 = 0x7f88_82e1;

/// Common header of every binary info entry.
#[repr(C, packed)]
struct BinaryInfoCore {
    info_type: u16,
    tag: u16,
}

/// An entry holding a 32-bit id and a pointer to a NUL-terminated string.
#[repr(C, packed)]
struct BinaryInfoIdAndString {
    core: BinaryInfoCore,
    id: u32,
    value: *const u8,
}

// SAFETY: the entry is immutable and only ever points at a static string.
unsafe impl Sync for BinaryInfoIdAndString {}

#[unsafe(link_section = ".bi_entries")]
#[used]
static BOOT2_NAME: BinaryInfoIdAndString = BinaryInfoIdAndString {
    core: BinaryInfoCore {
        info_type: BI_TYPE_ID_AND_STRING,
        tag: TAG_RASPBERRY_PI,
    },
    id: ID_RP_BOOT2_NAME,
    value: crate::NAME_NUL.as_ptr(),
};
