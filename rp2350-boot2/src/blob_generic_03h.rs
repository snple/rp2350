//! Boot2 blob for flash that answers the plain `03h` serial read command.
//!
//! Ported from the Pico SDK's `src/rp2350/boot_stage2/boot2_generic_03h.S`, with the
//! RISC-V half dropped and the register values interpolated as `const` operands.
//!
//! This variant keeps the interface in single-bit transfers, so it is roughly three
//! times slower than quad I/O XIP, but it works with essentially any SPI flash part
//! and needs no quad-enable programming.

use crate::regs::*;

/// Flash interface clock divisor: the interface runs at `clk_sys / SPI_CLKDIV`.
pub const SPI_CLKDIV: u32 = 4;

/// RX sampling delay, in units of half a `clk_sys` cycle.
pub const SPI_RX_DELAY: u32 = 1;

const _: () = assert!(SPI_CLKDIV <= M0_TIMING_CLKDIV_MASK >> M0_TIMING_CLKDIV_LSB);
const _: () = assert!(SPI_RX_DELAY <= M0_TIMING_RXDELAY_MASK >> M0_TIMING_RXDELAY_LSB);

const INIT_M0_TIMING: u32 = (1 << M0_TIMING_COOLDOWN_LSB)
    | (SPI_RX_DELAY << M0_TIMING_RXDELAY_LSB)
    | (SPI_CLKDIV << M0_TIMING_CLKDIV_LSB);

const INIT_M0_RCMD: u32 = CMD_READ << M0_RCMD_PREFIX_LSB;

/// All-serial read format with an 8-bit command prefix and no suffix or dummy phase.
const INIT_M0_RFMT: u32 = (WIDTH_S << M0_RFMT_PREFIX_WIDTH_LSB)
    | (WIDTH_S << M0_RFMT_ADDR_WIDTH_LSB)
    | (WIDTH_S << M0_RFMT_SUFFIX_WIDTH_LSB)
    | (WIDTH_S << M0_RFMT_DUMMY_WIDTH_LSB)
    | (WIDTH_S << M0_RFMT_DATA_WIDTH_LSB)
    | (PREFIX_LEN_8 << M0_RFMT_PREFIX_LEN_LSB);

core::arch::global_asm!(
    r#"
    .section .boot2, "ax"
    .syntax unified
    .thumb
    .p2align 2

    .global __boot2_entry_point
    .thumb_func
    .type __boot2_entry_point, %function
__boot2_entry_point:
    push {{lr}}

    // The QMI is configured for 03h reads straight out of reset, but this code
    // cannot assume it is still in that state, so write the window 0 registers
    // unconditionally.
    ldr r3, ={qmi_base}
    ldr r0, ={init_m0_timing}
    str r0, [r3, #{off_m0_timing}]
    ldr r0, ={init_m0_rcmd}
    str r0, [r3, #{off_m0_rcmd}]
    ldr r0, ={init_m0_rfmt}
    str r0, [r3, #{off_m0_rfmt}]

    pop {{pc}}

    // Literal pool: must stay inside .boot2, as it is copied along with the code.
    .p2align 2
    .ltorg
"#,
    qmi_base = const QMI_BASE,
    off_m0_timing = const QMI_M0_TIMING,
    off_m0_rcmd = const QMI_M0_RCMD,
    off_m0_rfmt = const QMI_M0_RFMT,
    init_m0_timing = const INIT_M0_TIMING,
    init_m0_rcmd = const INIT_M0_RCMD,
    init_m0_rfmt = const INIT_M0_RFMT,
);
