//! # rp2350-boot2
//!
//! RP2350 second stage boot loader ("boot2"), for the ARM Cortex-M33 target.
//!
//! ## What boot2 is on RP2350
//!
//! On RP2040 the boot ROM copies 256 bytes of flash into SRAM, verifies a CRC32 in the
//! last four bytes, and jumps into the copy. That code configures the QSPI and then
//! vectors into the application image, so on RP2040 a boot2 is required to boot at all.
//!
//! RP2350 is different. The boot ROM performs a best-effort XIP setup by itself and
//! enters the image through its `IMAGE_DEF` block (see `rp2350::block`), so no boot2 is
//! needed to boot. A boot2 on RP2350 is an optional, position-independent "XIP setup
//! function" of at most 256 bytes with no CRC that the *image* calls early in startup,
//! to replace the ROM's conservative configuration with an aggressive one: a high
//! interface clock, a tuned RX sampling delay, and — for quad-capable flash —
//! continuous quad-I/O reads instead of single-bit `03h` reads.
//!
//! The function rewrites the QMI timing registers that govern instruction fetch, so it
//! cannot run while being fetched from XIP. [`init`] copies the blob onto the stack and
//! calls it from there, which is what the Pico SDK's `crt0` does as well.
//!
//! ## Usage
//!
//! Enabling the feature on the HAL is enough: `rp2350-rt` then calls [`init`] from its
//! reset handler, before `.data` is initialised.
//!
//! ```toml
//! [dependencies]
//! rp2350 = { path = "../rp2350", features = ["boot2-w25q80"] }
//! ```
//!
//! Calling it manually (for instance from a `__pre_init` hook) works too, and is a
//! no-op when nothing is linked into `.boot2`:
//!
//! ```text
//! rp2350_boot2::init();
//! ```
//!
//! ## Features
//!
//! Exactly one variant must be selected. The crate's default is `generic-03h`, the one
//! that works with any flash part; `rp2350-rt` and the HAL turn default features off and
//! always select explicitly (`boot2-w25q80` / `boot2-generic-03h`).
//!
//! * `w25q80` — W25Q80-family quad-SPI flash, and the parts the Pico SDK's
//!   `boot2_w25q080.S` lists as compatible: W25Q16JV, W25Q32JV, AT25SF081, S25FL132K0.
//!   Uses `EBh` fast quad I/O with continuous-read mode bits.
//! * `generic-03h` — anything that answers a plain `03h` serial read. Roughly three
//!   times slower, but the safest choice and it needs no status-register programming.
//!
//! * `program-status-reg` (`w25q80` only) — check status register 2 and, if needed,
//!   program status register 1 to set the quad-enable bit before switching to quad
//!   reads. Without it a virgin flash part may answer quad reads incorrectly. The cost
//!   is code size and the loss of the flash's block protection bits.
//! * `binary-info` — emit a `BINARY_INFO_ID_RP_BOOT2_NAME` entry into `.bi_entries`
//!   naming the variant, so `picotool` can report it.
//!
//! ## Requirements
//!
//! The `.boot2` output section and the `__boot2_start__` / `__boot2_end__` symbols come
//! from `rp2350-rt`'s linker scripts (`link.x`, `link_ram.x`). The section must fit in
//! 256 bytes, and the linker fails the build if it does not.

#![no_std]
#![deny(missing_docs)]
#![allow(binary_asm_labels)]

#[cfg(all(feature = "w25q80", feature = "generic-03h"))]
compile_error!(
    "features `w25q80` and `generic-03h` are mutually exclusive; select exactly one boot2 \
     variant (use `default-features = false` to turn off the default `generic-03h`)"
);

#[cfg(not(any(feature = "w25q80", feature = "generic-03h")))]
compile_error!(
    "rp2350-boot2 needs a variant feature: enable `w25q80` or `generic-03h`, or drop the \
     dependency"
);

#[cfg(feature = "generic-03h")]
mod blob_generic_03h;
#[cfg(feature = "w25q80")]
mod blob_w25q80;

mod regs;

#[cfg(feature = "binary-info")]
mod binary_info;

#[cfg(feature = "generic-03h")]
pub use blob_generic_03h::{SPI_CLKDIV, SPI_RX_DELAY};
#[cfg(feature = "w25q80")]
pub use blob_w25q80::{SPI_CLKDIV, SPI_RX_DELAY};

/// Maximum size of the boot2 blob.
///
/// The boot ROM reserves 256 bytes for the copy it used to perform on RP2040, and the
/// Pico SDK keeps the same limit on RP2350. The linker asserts it.
pub const MAX_SIZE: usize = 256;

/// Name of the linked boot2 variant, for reporting and for `BINARY_INFO`.
#[cfg(feature = "w25q80")]
pub const NAME: &str = "boot2_w25q80";

/// Name of the linked boot2 variant, for reporting and for `BINARY_INFO`.
#[cfg(feature = "generic-03h")]
pub const NAME: &str = "boot2_generic_03h";

/// [`NAME`] with the trailing NUL that `BINARY_INFO` strings require.
///
/// Kept as a separate literal because `concat!` cannot see through a `const`.
#[cfg(all(feature = "binary-info", feature = "w25q80"))]
pub(crate) const NAME_NUL: &str = "boot2_w25q80\0";

/// [`NAME`] with the trailing NUL that `BINARY_INFO` strings require.
///
/// Kept as a separate literal because `concat!` cannot see through a `const`.
#[cfg(all(feature = "binary-info", feature = "generic-03h"))]
pub(crate) const NAME_NUL: &str = "boot2_generic_03h\0";

/// Copy the boot2 blob into RAM and run it, re-configuring the QMI for fast XIP.
///
/// The copy target is a 256-byte buffer on the stack, which stays valid for the whole
/// call. Nothing here depends on `.data`, `.bss` or any other RAM content, so it is safe
/// to call this before the reset handler has initialised them — which is exactly how
/// `rp2350-rt` uses it.
///
/// Calling this twice is harmless (the setup is idempotent) but pointless. It does not
/// return a value and cannot fail: when nothing is linked into `.boot2` the blob has zero
/// length and the call returns immediately, leaving the boot ROM's XIP setup in place.
///
/// # Panics
///
/// Never. The implementation deliberately uses only raw pointers and inline assembly, so
/// no bounds check or `memcpy` can end up in the early boot path.
#[inline(never)]
pub fn init() {
    // Provided by the rp2350-rt linker scripts.
    unsafe extern "C" {
        static __boot2_start__: u8;
        static __boot2_end__: u8;

        /// Entry point of the blob, as linked into flash. Calling it where it lives
        /// would be wrong — it rewrites the timing of the very bus it is fetched over —
        /// so only its address is ever used.
        fn __boot2_entry_point();
    }

    // SAFETY: both symbols are section-boundary markers in `.boot2`; their addresses are
    // valid to compare and nothing is dereferenced here.
    let start = core::ptr::addr_of!(__boot2_start__) as usize;
    let end = core::ptr::addr_of!(__boot2_end__) as usize;

    let len = end.saturating_sub(start);
    if len == 0 || len > MAX_SIZE {
        // No blob linked, or a section we cannot copy: leave the ROM setup alone.
        return;
    }

    // `MaybeUninit` keeps the 256 bytes off the initialisation path (no `memset`).
    #[repr(align(4))]
    struct Scratch(core::mem::MaybeUninit<[u8; MAX_SIZE]>);

    let mut scratch = Scratch(core::mem::MaybeUninit::uninit());
    let dst = core::ptr::addr_of_mut!(scratch.0).cast::<u8>();

    // Byte-wise copy in inline assembly: `core::ptr::copy_nonoverlapping` would call
    // `memcpy`, and this has to work before `.data` exists. The blob is only a few dozen
    // bytes, so the cost is irrelevant.
    unsafe {
        core::arch::asm!(
            "1:",
            "ldrb {tmp}, [{src}]",
            "strb {tmp}, [{dst}]",
            "adds {src}, {src}, #1",
            "adds {dst}, {dst}, #1",
            "subs {n}, {n}, #1",
            "bne 1b",
            src = inout(reg) start => _,
            dst = inout(reg) dst => _,
            n = inout(reg) len => _,
            tmp = out(reg) _,
            options(nostack),
        );
    }

    // The blob's object file defines `__boot2_entry_point` and nothing else that Rust
    // calls. A linker never pulls an unreferenced member out of an rlib, so this
    // reference is load-bearing: without it `.boot2` silently comes out empty. (The Pico
    // SDK's crt0 holds the same reference for the same reason.)
    let entry_fn: unsafe extern "C" fn() = __boot2_entry_point;
    let entry_sym = entry_fn as usize;

    // Locate the entry point inside the blob instead of assuming it sits at offset 0. Bit
    // 0 of the symbol value is the Thumb bit, not part of the offset.
    let entry_offset = (entry_sym & !1).wrapping_sub(start);
    if entry_offset >= len {
        // Not a blob we can reason about; leave the boot ROM's XIP setup alone.
        return;
    }

    // The assembler emits Thumb code, so the entry point needs the Thumb bit set; a `blx`
    // to an even address would try to switch to ARM state and fault.
    let entry = (dst as usize + entry_offset) | 1;

    // SAFETY: `dst` points at a word-aligned (see `Scratch`) complete copy of a
    // position-independent Thumb routine that only touches core registers, the QMI and the
    // QSPI pads, and returns through `lr`.
    let boot2: extern "C" fn() = unsafe { core::mem::transmute(entry) };
    boot2();
}
