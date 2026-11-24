//! # I2C hardware-to-hardware loopback test (I2C0 ↔ I2C1)
//!
//! Uses the two on-chip I2C controllers: I2C0 as master and I2C1 as target.
//!
//! ```text
//!   I2C0 (master)             I2C1 (target)
//!   GP4  SDA      ──────────  GP6 SDA
//!   GP5  SCL      ──────────  GP7 SCL
//! ```
//!
//! The target's RX FIFO is drained from the RX_FULL interrupt (needed for
//! writes longer than the 16-entry FIFO) and its TX FIFO is filled on RD_REQ,
//! in 8-byte chunks so a long read cannot deadlock on a full TX FIFO.
//!
//! Covered at 100 kHz and 400 kHz: write/read 4, 8 and 20 bytes,
//! write+repeated-start+read, address probing, and NACK on an absent address.
//!
//! Run with:
//!
//! ```bash
//! cargo run --release --bin i2c_hw_loopback
//! ```

#![no_std]
#![no_main]

use core::sync::atomic::{AtomicU8, AtomicU32, Ordering, compiler_fence};

#[cfg(feature = "defmt")]
use defmt_rtt as _;
use rp2350_examples::log::*;

use rp2350::bind;
use rp2350::clocks::ClockBuilder;
use rp2350::gpio::{ErasedPin, Pin, Pins};
use rp2350::i2c::{Error as I2cError, I2c0, I2c1Slave};
use rp2350::interrupt;
use rp2350::main;
use rp2350::pac::{Interrupt, Peripherals};

const ADDR: u8 = 0x2A;
/// RD_REQ chunk size: must be well below the 16-entry TX FIFO so the ISR
/// never blocks waiting for space while the target holds SCL low.
const CHUNK: usize = 8;

type Target = I2c1Slave<Pin<6>, Pin<7>>;

const RESP_WORDS: usize = 8; // 32 bytes
const RX_BYTES: usize = 64;

static RESP: [AtomicU32; RESP_WORDS] = [const { AtomicU32::new(0) }; RESP_WORDS];
static RESP_LEN: AtomicU32 = AtomicU32::new(0);
static RESP_OFFSET: AtomicU32 = AtomicU32::new(0);

static RX_BUF: [AtomicU8; RX_BYTES] = [const { AtomicU8::new(0) }; RX_BYTES];
static RX_LEN: AtomicU32 = AtomicU32::new(0);

static RD_REQ_COUNT: AtomicU32 = AtomicU32::new(0);
static EXTRA_READS: AtomicU32 = AtomicU32::new(0);

fn set_response(data: &[u8]) {
    let len = data.len().min(RESP_WORDS * 4);
    for (i, slot) in RESP.iter().enumerate() {
        let mut word = 0u32;
        for j in 0..4 {
            let idx = i * 4 + j;
            if idx < len {
                word |= (data[idx] as u32) << (8 * j);
            }
        }
        slot.store(word, Ordering::Relaxed);
    }
    compiler_fence(Ordering::SeqCst);
    RESP_OFFSET.store(0, Ordering::SeqCst);
    RESP_LEN.store(len as u32, Ordering::SeqCst);
}

fn response_byte(index: usize) -> u8 {
    let word = RESP[index / 4].load(Ordering::Relaxed);
    (word >> (8 * (index % 4))) as u8
}

fn reset_rx() {
    RX_LEN.store(0, Ordering::Release);
}

fn target_rx(buffer: &mut [u8]) -> usize {
    compiler_fence(Ordering::SeqCst);
    let len = (RX_LEN.load(Ordering::Acquire) as usize).min(buffer.len());
    for (i, slot) in buffer.iter_mut().take(len).enumerate() {
        *slot = RX_BUF[i].load(Ordering::Relaxed);
    }
    len
}

#[bind(I2C1_IRQ)]
fn i2c1_isr() {
    let i2c = unsafe { I2c1Slave::<ErasedPin, ErasedPin>::steal() };

    // Drain received bytes into the shared buffer.
    let mut tmp = [0u8; 16];
    loop {
        let n = i2c.read_available(&mut tmp);
        if n == 0 {
            break;
        }
        for &byte in &tmp[..n] {
            let idx = RX_LEN.fetch_add(1, Ordering::AcqRel) as usize;
            if idx < RX_BUF.len() {
                RX_BUF[idx].store(byte, Ordering::Relaxed);
            }
        }
    }

    if i2c.read_requested() {
        RD_REQ_COUNT.fetch_add(1, Ordering::Relaxed);

        let len = RESP_LEN.load(Ordering::SeqCst) as usize;
        let offset = RESP_OFFSET.load(Ordering::SeqCst) as usize;

        if offset < len {
            let chunk = (len - offset).min(CHUNK);
            let mut buf = [0u8; CHUNK];
            for (i, slot) in buf.iter_mut().take(chunk).enumerate() {
                *slot = response_byte(offset + i);
            }
            let _ = i2c.write(&buf[..chunk]);
            RESP_OFFSET.store((offset + chunk) as u32, Ordering::SeqCst);
        } else {
            EXTRA_READS.fetch_add(1, Ordering::Relaxed);
            let _ = i2c.write(&[0xFF]);
        }
    }
}

fn wait_stop(i2c: &Target) -> bool {
    for _ in 0..1_000_000u32 {
        if i2c.stop_detected() {
            return true;
        }
        core::hint::spin_loop();
    }
    false
}

fn finish(label: &str, ok: bool) -> bool {
    if ok {
        info!("PASS  {}", label);
    } else {
        error!("FAIL  {}", label);
    }
    ok
}

fn expect_target_rx(label: &str, expected: &[u8]) -> bool {
    let mut rx = [0u8; RX_BYTES];
    let len = target_rx(&mut rx);
    if &rx[..len] != expected {
        error!("{}: target RX mismatch", label);
        error!("  expected {:02x}", expected);
        error!("  got      {:02x}", &rx[..len]);
        false
    } else {
        true
    }
}

fn run_write(master: &I2c0<Pin<4>, Pin<5>>, target: &Target, label: &str, data: &[u8]) -> bool {
    let mut ok = true;
    reset_rx();
    set_response(&[]);

    if let Err(err) = master.write(ADDR, data) {
        error!("{}: master error {:?}", label, err);
        ok = false;
    }

    if !wait_stop(target) {
        error!("{}: target did not detect STOP", label);
        return false;
    }
    target.clear_stop();

    ok &= expect_target_rx(label, data);
    finish(label, ok)
}

fn run_probe(master: &I2c0<Pin<4>, Pin<5>>, target: &Target, label: &str) -> bool {
    let mut ok = true;
    reset_rx();
    set_response(&[]);

    if let Err(err) = master.write(ADDR, &[]) {
        error!("{}: master error {:?}", label, err);
        ok = false;
    }

    if !wait_stop(target) {
        error!("{}: target did not detect STOP", label);
        return false;
    }
    target.clear_stop();

    // The HAL's empty-write probe sends one zero data byte before STOP.
    ok &= expect_target_rx(label, &[0x00]);
    finish(label, ok)
}

fn run_read(master: &I2c0<Pin<4>, Pin<5>>, target: &Target, label: &str, response: &[u8]) -> bool {
    let mut ok = true;
    reset_rx();
    set_response(response);

    let rd_before = RD_REQ_COUNT.load(Ordering::Relaxed);
    let expected_reqs = response.len().div_ceil(CHUNK) as u32;

    let mut rx = [0u8; 32];
    if let Err(err) = master.read(ADDR, &mut rx[..response.len()]) {
        error!("{}: master error {:?}", label, err);
        ok = false;
    }

    if &rx[..response.len()] != response {
        ok = false;
        error!("{}: master RX mismatch", label);
        error!("  expected {:02x}", response);
        error!("  got      {:02x}", &rx[..response.len()]);
    }

    let reqs = RD_REQ_COUNT.load(Ordering::Relaxed) - rd_before;
    if reqs != expected_reqs {
        ok = false;
        error!(
            "{}: expected {} read requests, got {}",
            label, expected_reqs, reqs
        );
    }

    if !wait_stop(target) {
        ok = false;
        error!("{}: target did not detect STOP", label);
    }
    target.clear_stop();

    let mut stray = [0u8; RX_BYTES];
    let n = target_rx(&mut stray);
    if n != 0 {
        ok = false;
        error!("{}: target saw {} unexpected write bytes", label, n);
    }

    finish(label, ok)
}

fn run_write_read(
    master: &I2c0<Pin<4>, Pin<5>>,
    target: &Target,
    label: &str,
    write_data: &[u8],
    response: &[u8],
) -> bool {
    let mut ok = true;
    reset_rx();
    set_response(response);

    let rd_before = RD_REQ_COUNT.load(Ordering::Relaxed);
    let mut rx = [0u8; 32];
    if let Err(err) = master.write_read(ADDR, write_data, &mut rx[..response.len()]) {
        error!("{}: master error {:?}", label, err);
        ok = false;
    }

    if &rx[..response.len()] != response {
        ok = false;
        error!("{}: master RX mismatch", label);
        error!("  expected {:02x}", response);
        error!("  got      {:02x}", &rx[..response.len()]);
    }

    if RD_REQ_COUNT.load(Ordering::Relaxed) == rd_before {
        ok = false;
        error!("{}: no read request observed", label);
    }

    if !wait_stop(target) {
        error!("{}: target did not detect STOP", label);
        return false;
    }
    target.clear_stop();

    ok &= expect_target_rx(label, write_data);
    finish(label, ok)
}

fn run_at(
    master: &mut I2c0<Pin<4>, Pin<5>>,
    target: &Target,
    baud: u32,
    sys_clk: u32,
) -> (u32, u32) {
    master.set_baudrate(baud, sys_clk);

    let mut passed = 0;
    let mut failed = 0;
    let mut case = |ok: bool| {
        if ok {
            passed += 1;
        } else {
            failed += 1;
        }
    };

    info!("--- {} Hz ---", master.baudrate(sys_clk));

    case(run_write(
        master,
        target,
        "write 4B",
        &[0x11, 0x22, 0x33, 0x44],
    ));
    case(run_probe(master, target, "probe"));
    case(run_read(
        master,
        target,
        "read 4B",
        &[0xA1, 0xB2, 0xC3, 0xD4],
    ));
    case(run_write_read(
        master,
        target,
        "write_read 1+4B",
        &[0x10],
        &[0xDE, 0xAD, 0xBE, 0xEF],
    ));
    case(run_write(
        master,
        target,
        "write 8B",
        &[0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77],
    ));
    case(run_read(
        master,
        target,
        "read 8B",
        &[0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87],
    ));

    let long: [u8; 20] = [
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
        0x0F, 0x10, 0x11, 0x12, 0x13,
    ];
    case(run_write(master, target, "write 20B", &long));
    case(run_read(master, target, "read 20B", &long));

    // Absent address.
    {
        let result = master.write(0x55, &[0x00]);
        let ok = matches!(result, Err(I2cError::NoAcknowledge));
        if ok {
            info!("PASS  NACK on absent address");
            passed += 1;
        } else {
            error!("FAIL  NACK on absent address: {:?}", result);
            failed += 1;
        }
    }

    (passed, failed)
}

#[main]
fn main() -> ! {
    info!("==================================================");
    info!("I2C hardware-to-hardware loopback test");
    info!("I2C0 master: SDA=GP4, SCL=GP5");
    info!("I2C1 target: SDA=GP6, SCL=GP7, addr=0x{:02x}", ADDR);
    info!("==================================================");

    let dp = Peripherals::take().unwrap();
    let clocks = ClockBuilder::new().build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);
    let sys_clk = clocks.sys_freq();
    let pins = Pins::init(dp.IO_BANK0, dp.PADS_BANK0);

    let target = Target::init(dp.I2C1, pins.pin::<6>(), pins.pin::<7>(), ADDR, sys_clk);
    target.enable_rd_req_interrupt();
    target.enable_rx_full_interrupt();
    unsafe {
        interrupt::enable(Interrupt::I2C1_IRQ);
    }

    let mut master = I2c0::init(dp.I2C0, pins.pin::<4>(), pins.pin::<5>(), 100_000, &clocks);
    master.set_target_addr(ADDR as u16, false);

    let (mut passed, mut failed) = (0u32, 0u32);

    for &baud in &[100_000u32, 400_000] {
        let (p, f) = run_at(&mut master, &target, baud, sys_clk);
        passed += p;
        failed += f;
    }

    let extra = EXTRA_READS.load(Ordering::Relaxed);
    if extra != 0 {
        error!("{} unexpected read requests were serviced", extra);
    }

    info!("==================================================");
    if failed == 0 && extra == 0 {
        info!("RESULT: ALL {} I2C HW LOOPBACK CASES PASSED", passed);
    } else {
        error!("RESULT: {} passed, {} FAILED", passed, failed);
    }
    info!("DONE");

    loop {
        core::hint::spin_loop();
    }
}
