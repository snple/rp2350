//! # UART loopback test (primary and `UART_AUX` mappings)
//!
//! The test probes both UART0 mappings and runs the full suite on whichever
//! one(s) have a loopback:
//!
//! ```text
//!   primary:  GP0 (TX, funcsel 2)  ─────  GP1 (RX, funcsel 2)
//!   AUX:      GP2 (TX, funcsel 11) ─────  GP3 (RX, funcsel 11)
//! ```
//!
//! The board used for the original test wired GP0–GP2 and GP1–GP3; that alone
//! is not a loopback. Add **GP0–GP1** or **GP2–GP3** as well.
//!
//! ## What is tested
//!
//! - Several baud rates, including the extremes of the 16x oversampling
//! - 8N1, 8E1, 8O1, 8N2, 7N1 and 7E1 frame formats
//! - Single-byte, short and 32-byte (two FIFO batches) transfers
//!
//! Run with:
//!
//! ```bash
//! cargo run --bin uart_aux_loopback
//! ```

#![no_std]
#![no_main]

#[cfg(feature = "defmt")]
use defmt_rtt as _;
use rp2350_examples::log::*;

use rp2350::clocks::ClockBuilder;
use rp2350::gpio::{Pins, Uart0RxPin, Uart0TxPin};
use rp2350::main;
use rp2350::pac::Peripherals;
use rp2350::uart::{DataBits, Parity, StopBits, Uart0};

/// Bytes per TX burst. Keeping this below the 32-byte RX FIFO depth means the
/// loopback never overruns the receiver while the test is not draining it.
const CHUNK: usize = 16;

const PATTERN: [u8; 8] = [0x00, 0xFF, 0x55, 0xAA, 0x01, 0x80, 0x7F, 0x3C];

/// 7-bit-safe pattern (all values below 0x80).
const PATTERN7: [u8; 8] = [0x00, 0x7F, 0x55, 0x2A, 0x01, 0x40, 0x3F, 0x15];

fn pattern32() -> [u8; 32] {
    let mut data = [0u8; 32];
    let mut i = 0;
    while i < data.len() {
        data[i] = (i as u8).wrapping_mul(7).wrapping_add(0x31);
        i += 1;
    }
    data
}

#[allow(clippy::too_many_arguments)]
fn run_case<TX, RX>(
    uart: &mut Uart0<TX, RX>,
    name: &str,
    baud: u32,
    bits: DataBits,
    stop: StopBits,
    parity: Parity,
    data: &[u8],
    peri_clk: u32,
) -> bool
where
    TX: Uart0TxPin,
    RX: Uart0RxPin,
{
    let actual_baud = uart.set_baudrate(baud, peri_clk);
    uart.set_format(bits, stop, parity);

    // Drain anything left over from the previous case and start clean.
    uart.flush();
    while uart.read_byte().is_some() {}
    uart.clear_errors();

    let mut ok = true;
    let mut offset = 0;
    while offset < data.len() && ok {
        let end = (offset + CHUNK).min(data.len());
        let chunk = &data[offset..end];

        uart.write(chunk);

        let mut rx = [0u8; CHUNK];
        let rx_slice = &mut rx[..chunk.len()];
        match uart.read(rx_slice) {
            Ok(()) => {
                for (i, (&got, &want)) in rx_slice.iter().zip(chunk).enumerate() {
                    if got != want {
                        error!(
                            "{} @ {} baud (actual {}): byte {} expected 0x{:02x}, got 0x{:02x}",
                            name,
                            baud,
                            actual_baud,
                            offset + i,
                            want,
                            got
                        );
                        ok = false;
                        break;
                    }
                }
            }
            Err(err) => {
                error!("{} @ {} baud: read error {:?}", name, baud, err);
                ok = false;
            }
        }

        offset = end;
    }

    uart.flush();

    if ok {
        info!("PASS  {} @ {} baud (actual {} Hz)", name, baud, actual_baud);
    } else {
        error!("FAIL  {} @ {} baud", name, baud);
    }
    ok
}

/// One-byte 9600 8N1 probe used to find a working mapping.
fn probe<TX, RX>(uart: &mut Uart0<TX, RX>, peri_clk: u32) -> bool
where
    TX: Uart0TxPin,
    RX: Uart0RxPin,
{
    uart.set_baudrate(9_600, peri_clk);
    uart.set_format(DataBits::Eight, StopBits::One, Parity::None);
    uart.flush();
    while uart.read_byte().is_some() {}
    uart.clear_errors();

    uart.write(&[0x55]);

    let mut byte = [0u8; 1];
    matches!(uart.read(&mut byte), Ok(()) if byte[0] == 0x55)
}

fn run_suite<TX, RX>(uart: &mut Uart0<TX, RX>, mapping: &str, peri_clk: u32) -> (u32, u32)
where
    TX: Uart0TxPin,
    RX: Uart0RxPin,
{
    info!("--- {} ---", mapping);

    let mut passed = 0u32;
    let mut failed = 0u32;
    let long = pattern32();

    macro_rules! case {
        ($name:expr, $baud:expr, $bits:expr, $stop:expr, $parity:expr, $data:expr) => {{
            if run_case(uart, $name, $baud, $bits, $stop, $parity, &$data, peri_clk) {
                passed += 1;
            } else {
                failed += 1;
            }
        }};
    }

    // Baud rate sweep, 8N1.
    for &baud in &[9_600u32, 115_200, 921_600, 1_500_000, 3_000_000] {
        case!(
            "8N1",
            baud,
            DataBits::Eight,
            StopBits::One,
            Parity::None,
            PATTERN
        );
    }

    // Frame format sweep at a moderate baud rate.
    case!(
        "8N1",
        115_200,
        DataBits::Eight,
        StopBits::One,
        Parity::None,
        PATTERN
    );
    case!(
        "8E1",
        115_200,
        DataBits::Eight,
        StopBits::One,
        Parity::Even,
        PATTERN
    );
    case!(
        "8O1",
        115_200,
        DataBits::Eight,
        StopBits::One,
        Parity::Odd,
        PATTERN
    );
    case!(
        "8N2",
        115_200,
        DataBits::Eight,
        StopBits::Two,
        Parity::None,
        PATTERN
    );
    case!(
        "7N1",
        115_200,
        DataBits::Seven,
        StopBits::One,
        Parity::None,
        PATTERN7
    );
    case!(
        "7E1",
        115_200,
        DataBits::Seven,
        StopBits::One,
        Parity::Even,
        PATTERN7
    );

    // Length coverage: one byte and two full FIFO batches.
    case!(
        "8N1 1B",
        115_200,
        DataBits::Eight,
        StopBits::One,
        Parity::None,
        [0x5Au8]
    );
    case!(
        "8N1 32B",
        115_200,
        DataBits::Eight,
        StopBits::One,
        Parity::None,
        long
    );

    (passed, failed)
}

#[main]
fn main() -> ! {
    info!("==================================================");
    info!("UART loopback test");
    info!("primary GP0(TX)/GP1(RX), AUX GP2(TX)/GP3(RX)");
    info!("==================================================");

    let dp = Peripherals::take().unwrap();
    let clocks = ClockBuilder::new().build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);
    let peri_clk = clocks.peri_freq();
    let pins = Pins::init(dp.IO_BANK0, dp.PADS_BANK0);

    let mut passed = 0u32;
    let mut failed = 0u32;

    // Primary mapping.
    let mut uart = Uart0::init(dp.UART0, pins.pin::<0>(), pins.pin::<1>(), 9_600, peri_clk);
    let primary_ok = probe(&mut uart, peri_clk);
    info!(
        "primary GP0/GP1: {}",
        if primary_ok {
            "loopback detected"
        } else {
            "no loopback"
        }
    );
    if primary_ok {
        let (p, f) = run_suite(&mut uart, "PRIMARY", peri_clk);
        passed += p;
        failed += f;
    }
    let (uart_periph, _, _) = uart.release();

    // AUX mapping.
    let mut uart = Uart0::init(
        uart_periph,
        pins.pin::<2>(),
        pins.pin::<3>(),
        9_600,
        peri_clk,
    );
    let aux_ok = probe(&mut uart, peri_clk);
    info!(
        "AUX GP2/GP3: {}",
        if aux_ok {
            "loopback detected"
        } else {
            "no loopback"
        }
    );
    if aux_ok {
        let (p, f) = run_suite(&mut uart, "AUX", peri_clk);
        passed += p;
        failed += f;
    }

    info!("==================================================");
    if !primary_ok && !aux_ok {
        error!("No UART loopback found: add GP0-GP1 or GP2-GP3");
    } else if failed == 0 {
        info!("RESULT: ALL {} UART CASES PASSED", passed);
    } else {
        error!("RESULT: {} passed, {} FAILED", passed, failed);
    }
    info!("DONE");

    loop {
        core::hint::spin_loop();
    }
}
