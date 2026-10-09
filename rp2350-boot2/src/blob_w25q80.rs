//! Boot2 blob for Winbond W25Q80-family quad-SPI flash.
//!
//! Ported from the Pico SDK's `src/rp2350/boot_stage2/boot2_w25q080.S` (which,
//! despite the `w25q080` in its name, is the generic quad-I/O variant shipped for
//! RP2350 boards; its own header lists W25Q16JV, AT25SF081 and S25FL132K0 as
//! supported too). The RISC-V half of the SDK file is dropped, the `#define`s
//! become the Rust constants below, and the register values are interpolated into
//! the assembly as `const` operands.
//!
//! What it does, in order:
//!
//! 1. Configure the QSPI pads: 8 mA drive and fast slew on SCLK, and disable the
//!    Schmitt trigger on the four data pads to reduce the input delay.
//! 2. Optionally check status register 2 and, if the quad-enable bit is clear,
//!    program status register 1 so that the flash turns WPn/HOLDn into IO2/IO3.
//! 3. Point memory window 0 at `EBh` + continuous-read mode bits with quad
//!    address/suffix/dummy/data phases.
//! 4. Perform one dummy read through the no-allocate XIP alias, which puts the
//!    flash into continuous read mode, then clear the prefix length so that
//!    later reads no longer need the `EBh` instruction byte.

use crate::regs::*;

/// Flash interface clock divisor: the interface runs at `clk_sys / SPI_CLKDIV`.
pub const SPI_CLKDIV: u32 = 2;

/// RX sampling delay, in units of half a `clk_sys` cycle.
pub const SPI_RX_DELAY: u32 = 2;

// The SDK enforces these limits with `#error` preprocessor checks; here they are
// compile-time assertions on the field widths.
const _: () = assert!(SPI_CLKDIV <= M0_TIMING_CLKDIV_MASK >> M0_TIMING_CLKDIV_LSB);
const _: () = assert!(SPI_RX_DELAY <= M0_TIMING_RXDELAY_MASK >> M0_TIMING_RXDELAY_LSB);

/// Direct-mode clock divisor. Chosen conservatively (5 MHz at a 150 MHz `clk_sys`)
/// because in direct mode there is no RX delay to compensate for round-trip delay.
const DIRECT_CLKDIV: u32 = 30;
const _: () = assert!(DIRECT_CLKDIV <= DIRECT_CSR_CLKDIV_MASK >> DIRECT_CSR_CLKDIV_LSB);

/// SCLK pad: 8 mA drive, fast slew, input buffer left disabled.
const INIT_PAD_SCLK: u32 = (PAD_DRIVE_8MA << PAD_DRIVE_LSB) | PAD_SLEWFAST;

/// Schmitt trigger bit of the data pad registers (cleared via the CLR alias).
const PAD_SCHMITT_BITS: u32 = PAD_SCHMITT;

const INIT_DIRECT_CSR: u32 =
    (DIRECT_CLKDIV << DIRECT_CSR_CLKDIV_LSB) | DIRECT_CSR_EN | DIRECT_CSR_AUTO_CS0N;

const INIT_M0_TIMING: u32 = (1 << M0_TIMING_COOLDOWN_LSB)
    | (SPI_RX_DELAY << M0_TIMING_RXDELAY_LSB)
    | (SPI_CLKDIV << M0_TIMING_CLKDIV_LSB);

const INIT_M0_RCMD: u32 =
    (CMD_READ_FAST_QUAD_IO << M0_RCMD_PREFIX_LSB) | (MODE_CONTINUOUS_READ << M0_RCMD_SUFFIX_LSB);

const INIT_M0_RFMT: u32 = (WIDTH_S << M0_RFMT_PREFIX_WIDTH_LSB)
    | (WIDTH_Q << M0_RFMT_ADDR_WIDTH_LSB)
    | (WIDTH_Q << M0_RFMT_SUFFIX_WIDTH_LSB)
    | (WIDTH_Q << M0_RFMT_DUMMY_WIDTH_LSB)
    | (WIDTH_Q << M0_RFMT_DATA_WIDTH_LSB)
    | (PREFIX_LEN_8 << M0_RFMT_PREFIX_LEN_LSB)
    | (SUFFIX_LEN_8 << M0_RFMT_SUFFIX_LEN_LSB)
    | (WAIT_CYCLES << M0_RFMT_DUMMY_LEN_LSB);

/// `.if` operand for the status-register programming path.
const PROGRAM_STATUS_REG: u32 = if cfg!(feature = "program-status-reg") {
    1
} else {
    0
};

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

    // ---- QSPI pad configuration ----
    ldr r3, ={pads_qspi_base}
    ldr r0, ={init_pad_sclk}
    str r0, [r3, #{off_sclk}]
    // SD0..SD3: clear Schmitt trigger to reduce input delay (CLR alias).
    add r3, r3, #{alias_clr}
    ldr r0, ={schmitt_bits}
    str r0, [r3, #{off_sd0}]
    str r0, [r3, #{off_sd1}]
    str r0, [r3, #{off_sd2}]
    str r0, [r3, #{off_sd3}]

    ldr r3, ={qmi_base}

.if {program_status_reg}
    // ---- quad enable via the flash status registers ----
    ldr r1, ={init_direct_csr}
    str r1, [r3, #{off_direct_csr}]
    // Direct mode may not start until the cooldown of the last XIP transfer
    // has expired; BUSY reports that.
.Lqmi_busy:
    ldr r0, [r3, #{off_direct_csr}]
    tst r0, #{direct_csr_busy}
    bne .Lqmi_busy

    // Nothing to do if SR2 already reports quad mode.
    movs r0, #{cmd_read_status2}
    bl .Lread_flash_sreg
    cmp r0, #{sreg_data}
    beq .Lskip_sreg

    // 06h write enable, RX byte discarded.
    movs r0, #{cmd_write_enable}
    str r0, [r3, #{off_direct_tx}]
    bl .Lwait_qmi_ready
    ldr r0, [r3, #{off_direct_rx}]

    // 01h write status register 1 with QE set. This clears the block
    // protection bits, which is the price of not relying on a volatile SR
    // write (not supported by all parts).
    movs r0, #{cmd_write_status}
    str r0, [r3, #{off_direct_tx}]
    movs r0, #0
    str r0, [r3, #{off_direct_tx}]
    movs r0, #{sreg_data}
    str r0, [r3, #{off_direct_tx}]
    bl .Lwait_qmi_ready
    ldr r0, [r3, #{off_direct_rx}]
    ldr r0, [r3, #{off_direct_rx}]
    ldr r0, [r3, #{off_direct_rx}]

    // Poll SR1 until the write in progress bit clears.
.Lwip:
    movs r0, #{cmd_read_status}
    bl .Lread_flash_sreg
    lsrs r0, r0, #1
    bcs .Lwip

.Lskip_sreg:
    // Leave direct mode.
    bic r1, r1, #{direct_csr_en}
    str r1, [r3, #{off_direct_csr}]
.endif

    // ---- memory window 0: EBh + mode bits, then all-quad ----
    ldr r0, ={init_m0_timing}
    str r0, [r3, #{off_m0_timing}]
    ldr r0, ={init_m0_rcmd}
    str r0, [r3, #{off_m0_rcmd}]
    ldr r0, ={init_m0_rfmt}
    str r0, [r3, #{off_m0_rfmt}]

    // Dummy transfer: clocks out EBh + mode bits once. The flash stays in
    // continuous read mode afterwards, so later transfers can drop the prefix.
    ldr r1, ={xip_nocache_noalloc}
    ldrb r1, [r1]

    // Prefix length 0: the flash no longer expects the instruction byte.
    bic r0, r0, #{m0_rfmt_prefix_len}
    str r0, [r3, #{off_m0_rfmt}]

    pop {{pc}}

    // Wait for direct mode to go idle. Clobbers r0.
.Lwait_qmi_ready:
    ldr r0, [r3, #{off_direct_csr}]
    lsrs r0, r0, #{direct_csr_busy_shift}
    bcs .Lwait_qmi_ready
    bx lr

    // Read a status register: command in r0, value returned in r0.
.Lread_flash_sreg:
    push {{lr}}
    str r0, [r3, #{off_direct_tx}]
    // Dummy byte.
    str r0, [r3, #{off_direct_tx}]
    bl .Lwait_qmi_ready
    // Discard the first byte, return the second.
    ldr r0, [r3, #{off_direct_rx}]
    ldr r0, [r3, #{off_direct_rx}]
    pop {{pc}}

    // Literal pool: must stay inside .boot2, as it is copied along with the code.
    .p2align 2
    .ltorg
"#,
    pads_qspi_base = const PADS_QSPI_BASE,
    off_sclk = const PADS_QSPI_GPIO_QSPI_SCLK,
    off_sd0 = const PADS_QSPI_GPIO_QSPI_SD0,
    off_sd1 = const PADS_QSPI_GPIO_QSPI_SD1,
    off_sd2 = const PADS_QSPI_GPIO_QSPI_SD2,
    off_sd3 = const PADS_QSPI_GPIO_QSPI_SD3,
    alias_clr = const REG_ALIAS_CLR,
    init_pad_sclk = const INIT_PAD_SCLK,
    schmitt_bits = const PAD_SCHMITT_BITS,
    qmi_base = const QMI_BASE,
    program_status_reg = const PROGRAM_STATUS_REG,
    off_direct_csr = const QMI_DIRECT_CSR,
    off_direct_tx = const QMI_DIRECT_TX,
    off_direct_rx = const QMI_DIRECT_RX,
    direct_csr_busy = const DIRECT_CSR_BUSY_MASK,
    direct_csr_busy_shift = const (DIRECT_CSR_BUSY_LSB + 1),
    direct_csr_en = const DIRECT_CSR_EN,
    init_direct_csr = const INIT_DIRECT_CSR,
    cmd_read_status2 = const CMD_READ_STATUS2,
    cmd_read_status = const CMD_READ_STATUS,
    cmd_write_enable = const CMD_WRITE_ENABLE,
    cmd_write_status = const CMD_WRITE_STATUS,
    sreg_data = const SREG_DATA,
    off_m0_timing = const QMI_M0_TIMING,
    off_m0_rcmd = const QMI_M0_RCMD,
    off_m0_rfmt = const QMI_M0_RFMT,
    m0_rfmt_prefix_len = const M0_RFMT_PREFIX_LEN_MASK,
    init_m0_timing = const INIT_M0_TIMING,
    init_m0_rcmd = const INIT_M0_RCMD,
    init_m0_rfmt = const INIT_M0_RFMT,
    xip_nocache_noalloc = const XIP_NOCACHE_NOALLOC_BASE,
);
