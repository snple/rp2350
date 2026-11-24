//! # SPI master/slave loopback test (SPI0 ↔ SPI1)
//!
//! Uses SPI0 as master and SPI1 as slave, wired through the four shorted
//! jumper pairs:
//!
//! ```text
//!   SPI0 (master)             SPI1 (slave)
//!   GP18 SCK      ──────────  GP10 SCK
//!   GP19 TX(MOSI) ──────────  GP8  RX(MOSI)
//!   GP16 RX(MISO) ──────────  GP11 TX(MISO)
//!   GP17 CSn      ──────────  GP9  CSn
//! ```
//!
//! Both directions are checked on every transfer: the master receives the
//! bytes preloaded into the slave's TX FIFO while the slave receives the
//! master's MOSI bytes. Modes 0-3 and several clock rates are covered.
//!
//! Run with:
//!
//! ```bash
//! cargo run --bin spi_slave_loopback
//! ```

#![no_std]
#![no_main]

#[cfg(feature = "defmt")]
use defmt_rtt as _;
use rp2350_examples::log::*;

use rp2350::clocks::ClockBuilder;
use rp2350::gpio::{Pin, Pins};
use rp2350::main;
use rp2350::pac::Peripherals;
use rp2350::spi::{FrameFormat, Mode, Spi0WithCs, Spi1Slave};

const TX_PATTERN: [u8; 8] = [0x00, 0xFF, 0xAA, 0x55, 0x12, 0x34, 0x56, 0x78];
const RX_PATTERN: [u8; 8] = [0xDE, 0xAD, 0xBE, 0xEF, 0x5A, 0xA5, 0x3C, 0xC3];

type Master = Spi0WithCs<Pin<18>, Pin<19>, Pin<16>, Pin<17>>;
type Slave = Spi1Slave<Pin<10>, Pin<11>, Pin<8>, Pin<9>>;

fn report(name: &str, mode: Mode, baud: u32, actual_baud: u32, ok: bool) {
    if ok {
        info!(
            "PASS  {} mode={:?} baud={} (actual {})",
            name, mode, baud, actual_baud
        );
    } else {
        error!("FAIL  {} mode={:?} baud={}", name, mode, baud);
    }
}

fn prepare(
    master: &mut Master,
    slave: &mut Slave,
    mode: Mode,
    baud: u32,
    peri_clk: u32,
    response: &[u8],
) -> u32 {
    master.set_enabled(false);
    slave.set_enabled(false);

    master.set_format(8, mode, FrameFormat::MotorolaSpi);
    slave.set_format(8, mode, FrameFormat::MotorolaSpi);

    // Drain stale FIFO contents before the next transfer.
    while master.try_read_word().is_some() {}
    while slave.try_read_word().is_some() {}

    // The slave's MISO bytes must be in its TX FIFO before the first clock.
    for &byte in response {
        slave.write_word(byte as u16);
    }

    let actual_baud = master.set_baudrate(baud, peri_clk);
    slave.set_enabled(true);
    actual_baud
}

#[allow(clippy::too_many_arguments)]
fn run_transfer(
    master: &mut Master,
    slave: &mut Slave,
    name: &str,
    mode: Mode,
    baud: u32,
    peri_clk: u32,
    tx: &[u8],
    response: &[u8],
) -> bool {
    let actual_baud = prepare(master, slave, mode, baud, peri_clk, response);

    let mut rx = [0u8; TX_PATTERN.len()];
    let len = tx.len();
    let result = master.transfer(&mut rx[..len], tx);
    master.flush();

    // Collect everything the slave received; a mismatched count is a failure.
    let mut slave_rx = [0u8; TX_PATTERN.len()];
    let mut slave_len = 0;
    while let Some(word) = slave.try_read_word() {
        if slave_len < slave_rx.len() {
            slave_rx[slave_len] = word as u8;
            slave_len += 1;
        }
    }

    let mut ok = result.is_ok();
    if !ok {
        error!("{}: master transfer error {:?}", name, result);
    }

    if &rx[..len] != response {
        ok = false;
        error!("{}: master RX mismatch", name);
        error!("  expected {:02x}", response);
        error!("  got      {:02x}", &rx[..len]);
    }

    if slave_len != len {
        ok = false;
        error!(
            "{}: slave received {} bytes, expected {}",
            name, slave_len, len
        );
    } else if &slave_rx[..len] != tx {
        ok = false;
        error!("{}: slave RX mismatch", name);
        error!("  expected {:02x}", tx);
        error!("  got      {:02x}", &slave_rx[..len]);
    }

    report(name, mode, baud, actual_baud, ok);
    ok
}

#[allow(clippy::too_many_arguments)]
fn run_master_read(
    master: &mut Master,
    slave: &mut Slave,
    name: &str,
    mode: Mode,
    baud: u32,
    peri_clk: u32,
    response: &[u8],
) -> bool {
    let actual_baud = prepare(master, slave, mode, baud, peri_clk, response);

    let mut rx = [0u8; 8];
    let len = response.len();
    let result = master.read(&mut rx[..len]);
    master.flush();

    let mut slave_rx = [0u8; 8];
    let mut slave_len = 0;
    while let Some(word) = slave.try_read_word() {
        if slave_len < slave_rx.len() {
            slave_rx[slave_len] = word as u8;
            slave_len += 1;
        }
    }

    let mut ok = result.is_ok();
    if !ok {
        error!("{}: master read error {:?}", name, result);
    }

    if &rx[..len] != response {
        ok = false;
        error!("{}: master RX mismatch", name);
        error!("  expected {:02x}", response);
        error!("  got      {:02x}", &rx[..len]);
    }

    // `read()` clocks out zero bytes as dummy data.
    if slave_len != len || slave_rx[..len] != [0u8; 8][..len] {
        ok = false;
        error!("{}: slave did not receive dummy zeros", name);
    }

    report(name, mode, baud, actual_baud, ok);
    ok
}

#[main]
fn main() -> ! {
    info!("==================================================");
    info!("SPI master/slave loopback test");
    info!("SPI0 master: SCK=GP18 TX=GP19 RX=GP16 CSn=GP17");
    info!("SPI1 slave : SCK=GP10 RX=GP8  TX=GP11 CSn=GP9");
    info!("==================================================");

    let dp = Peripherals::take().unwrap();
    let clocks = ClockBuilder::new().build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);
    let peri_clk = clocks.peri_freq();
    let pins = Pins::init(dp.IO_BANK0, dp.PADS_BANK0);

    // Verify the four jumper pairs before driving the SPI peripherals; an
    // open wire otherwise shows up as a confusing slave FIFO timeout.
    macro_rules! check_wire {
        ($a:literal, $b:literal) => {{
            let mut a = pins.pin::<$a>();
            a.into_output();
            let mut b = pins.pin::<$b>();
            b.into_input_pulldown();
            a.set_high();
            rp2350_rt::asm::delay(1500);
            let ok = b.get();
            a.into_input_pulldown();
            info!("wire GP{}-GP{}: {}", $a, $b, if ok { "OK" } else { "OPEN" });
            ok
        }};
    }

    let mut wiring_ok = true;
    wiring_ok &= check_wire!(18, 10); // SCK
    wiring_ok &= check_wire!(19, 8); // MOSI
    wiring_ok &= check_wire!(16, 11); // MISO
    wiring_ok &= check_wire!(17, 9); // CSn

    if !wiring_ok {
        error!("SPI wiring incomplete: connect GP18-GP10, GP19-GP8, GP16-GP11, GP17-GP9");
        info!("DONE");
        loop {
            core::hint::spin_loop();
        }
    }

    let mut master = Master::init(
        dp.SPI0,
        pins.pin::<18>(),
        pins.pin::<19>(),
        pins.pin::<16>(),
        pins.pin::<17>(),
        1_000_000,
        peri_clk,
    );
    let mut slave = Slave::init(
        dp.SPI1,
        pins.pin::<10>(),
        pins.pin::<11>(),
        pins.pin::<8>(),
        pins.pin::<9>(),
        peri_clk,
    );

    let mut passed = 0u32;
    let mut failed = 0u32;

    macro_rules! case {
        ($expr:expr) => {{
            if $expr {
                passed += 1;
            } else {
                failed += 1;
            }
        }};
    }

    // All four SPI modes at 1 MHz.
    for mode in [Mode::Mode0, Mode::Mode1, Mode::Mode2, Mode::Mode3] {
        case!(run_transfer(
            &mut master,
            &mut slave,
            "8B",
            mode,
            1_000_000,
            peri_clk,
            &TX_PATTERN,
            &RX_PATTERN,
        ));
    }

    // Clock-rate sweep in mode 0.
    for &baud in &[100_000u32, 500_000, 4_000_000, 8_000_000] {
        case!(run_transfer(
            &mut master,
            &mut slave,
            "8B",
            Mode::Mode0,
            baud,
            peri_clk,
            &TX_PATTERN,
            &RX_PATTERN,
        ));
    }

    // Single byte, both directions.
    case!(run_transfer(
        &mut master,
        &mut slave,
        "1B",
        Mode::Mode0,
        1_000_000,
        peri_clk,
        &[0xA5],
        &[0x5A],
    ));

    // Master read-only path (dummy 0x00 bytes on MOSI).
    case!(run_master_read(
        &mut master,
        &mut slave,
        "read 4B",
        Mode::Mode0,
        1_000_000,
        peri_clk,
        &RX_PATTERN[..4],
    ));

    info!("==================================================");
    if failed == 0 {
        info!("RESULT: ALL {} SPI CASES PASSED", passed);
    } else {
        error!("RESULT: {} passed, {} FAILED", passed, failed);
    }
    info!("DONE");

    loop {
        core::hint::spin_loop();
    }
}
