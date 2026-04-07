//! # USB CDC Serial with Interrupt + Event Queue Example
//!
//! Demonstrates interrupt-driven USB CDC ACM using an event queue (SPSC ring
//! buffer) to decouple the USB ISR from the application main loop.
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────┐      ┌─────────────────┐      ┌─────────────────┐
//! │  USB Interrupt  │ ──── │   Event Queue   │ ──── │   Main Loop     │
//! │  (poll USB)     │      │  (SPSC ring)    │      │  (process data) │
//! └─────────────────┘      └─────────────────┘      └─────────────────┘
//! ```
//!
//! ## OUT endpoint re-arming
//!
//! The RP2350 `read()` implementation re-arms the OUT endpoint automatically
//! on every successful read, matching the `usb-device` crate convention used
//! by all other HALs (rp-hal, stm32, nrf52, etc.).  No manual re-arm needed.
//!
//! ## Hardware
//! - Raspberry Pi Pico 2 (RP2350)
//! - USB cable connected to the Pico's USB port

#![no_std]
#![no_main]

use core::cell::RefCell;
use core::hint::spin_loop;
use core::str;
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

use pico_rs::block::ImageDef;
use pico_rs::clocks::ClockBuilder;
use pico_rs::interrupt::{self, Interrupt, Mutex};
use pico_rs::pac::Peripherals;
use pico_rs::rt::peripheral::Peripherals as CorePeripherals;
use pico_rs::task::GenericQueue;
use pico_rs::{SysTick, bind, entry, exception};

use pico_rs_usb::bus::UsbBusAllocator;
use pico_rs_usb::class::UsbClass;
use pico_rs_usb::class::cdc_acm::CdcAcm;
use pico_rs_usb::device::{UsbDevice, UsbDeviceBuilder};
use pico_rs_usb::error::UsbError;
use pico_rs_usb::rp2350::Rp2350Bus;

// ──────────────────────────────────────────────────────────────────────────────
// Boot block
// ──────────────────────────────────────────────────────────────────────────────

#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::exe();

// ──────────────────────────────────────────────────────────────────────────────
// Constants
// ──────────────────────────────────────────────────────────────────────────────

/// Maximum bytes per RX event (one USB full-speed bulk packet).
const RX_CHUNK: usize = 64;

/// Event queue capacity.
const QUEUE_SIZE: usize = 16;

/// Maximum command line length.
const MAX_LINE: usize = 96;

// ──────────────────────────────────────────────────────────────────────────────
// Event types
// ──────────────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy)]
enum UsbEvent {
    Connected,
    Disconnected,
    TimerTick,
    RxPacket { len: usize, data: [u8; RX_CHUNK] },
}

// ──────────────────────────────────────────────────────────────────────────────
// Line accumulator
// ──────────────────────────────────────────────────────────────────────────────

enum FeedResult {
    Incomplete,
    Ready,
    Overflow,
}

struct LineAccumulator {
    buf: [u8; MAX_LINE],
    len: usize,
    overflow: bool,
}

impl LineAccumulator {
    const fn new() -> Self {
        Self {
            buf: [0; MAX_LINE],
            len: 0,
            overflow: false,
        }
    }

    fn reset(&mut self) {
        self.len = 0;
        self.overflow = false;
    }

    fn feed(&mut self, byte: u8) -> FeedResult {
        match byte {
            b'\n' | b'\r' => {
                if self.overflow {
                    self.reset();
                    FeedResult::Overflow
                } else if self.len == 0 {
                    FeedResult::Incomplete
                } else {
                    FeedResult::Ready
                }
            }
            _ => {
                if self.len < self.buf.len() {
                    self.buf[self.len] = byte;
                    self.len += 1;
                } else {
                    self.overflow = true;
                }
                FeedResult::Incomplete
            }
        }
    }

    fn line(&self) -> &str {
        str::from_utf8(&self.buf[..self.len])
            .map(|s| s.trim())
            .unwrap_or("")
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// Global state
// ──────────────────────────────────────────────────────────────────────────────

/// Event queue: USB ISR → main loop.
static EVENTS: GenericQueue<UsbEvent, QUEUE_SIZE> = GenericQueue::new();

/// Number of RX packets dropped because the event queue was full.
static DROPPED: AtomicU32 = AtomicU32::new(0);

// The UsbBusAllocator must live for `'static` so that the UsbDevice and CdcAcm
// (which borrow from it) can be stored in statics.  We initialise it once in
// `main` before enabling interrupts, then never touch it again.
//
// Safety invariant: `USB_ALLOC` is written exactly once (in `main`, before any
// interrupt can fire) and then only read through the `'static` reference.
static mut USB_ALLOC: Option<UsbBusAllocator<Rp2350Bus>> = None;

static USB_DEV: Mutex<RefCell<Option<UsbDevice<'static, Rp2350Bus>>>> =
    Mutex::new(RefCell::new(None));

static USB_CDC: Mutex<RefCell<Option<CdcAcm<'static, Rp2350Bus>>>> = Mutex::new(RefCell::new(None));

/// Current DTR state; set atomically in the USB ISR, read anywhere without a lock.
static PORT_OPEN: AtomicBool = AtomicBool::new(false);

// ──────────────────────────────────────────────────────────────────────────────
// USB interrupt handler
// ──────────────────────────────────────────────────────────────────────────────

#[bind(USBCTRL_IRQ)]
fn usbctrl_irq() {
    info!("USB interrupt");
    interrupt::free(|cs| {
        let mut dev = USB_DEV.borrow(cs).borrow_mut();
        let mut cdc = USB_CDC.borrow(cs).borrow_mut();

        if let (Some(dev), Some(cdc)) = (dev.as_mut(), cdc.as_mut()) {
            dev.poll(&mut [cdc as &mut dyn UsbClass<Rp2350Bus>]);

            let port_open = cdc.is_configured() && cdc.dtr();
            let was_open = PORT_OPEN.swap(port_open, Ordering::Relaxed);

            if port_open && !was_open {
                EVENTS.push(UsbEvent::Connected);
            } else if !port_open && was_open {
                EVENTS.push(UsbEvent::Disconnected);
            }

            if port_open {
                let mut buf = [0u8; RX_CHUNK];
                loop {
                    match cdc.read(&mut buf) {
                        Ok(n) if n > 0 => {
                            if !EVENTS.push(UsbEvent::RxPacket { len: n, data: buf }) {
                                DROPPED.fetch_add(1, Ordering::Relaxed);
                                break;
                            }
                        }
                        _ => break,
                    }
                }
            }
        }
    });
}

// ──────────────────────────────────────────────────────────────────────────────
// SysTick — 1 s periodic tick
// ──────────────────────────────────────────────────────────────────────────────

#[exception]
fn SysTick() {
    EVENTS.push(UsbEvent::TimerTick);
}

// ──────────────────────────────────────────────────────────────────────────────
// Main
// ──────────────────────────────────────────────────────────────────────────────

#[entry]
fn main() -> ! {
    info!("USB CDC Interrupt Example starting...");

    let dp = Peripherals::take().unwrap();
    let cp = CorePeripherals::take().unwrap();

    let clocks = ClockBuilder::new()
        .enable_usb()
        .build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);
    info!(
        "Clocks: sys={} Hz, usb={} Hz",
        clocks.sys_freq(),
        clocks.usb_freq()
    );

    // ── USB setup ─────────────────────────────────────────────────────────────
    //
    // Store the allocator in a `static mut` so that its lifetime is `'static`.
    // This lets UsbDevice<'static, B> and CdcAcm<'static, B> borrow from it.
    //
    // Safety: this block runs once in `main` before interrupts are enabled, so
    // no other code can observe the partially initialised state.
    let alloc: &'static UsbBusAllocator<Rp2350Bus> = unsafe {
        // Write through a raw pointer so we never create a `&mut` or `&`
        // reference to the mutable static, satisfying the `static_mut_refs` lint.
        core::ptr::addr_of_mut!(USB_ALLOC).write(Some(UsbBusAllocator::new(Rp2350Bus::new())));
        // Dereference the raw pointer to call `.as_ref()`.  `addr_of!` does not
        // create a reference to the static; it returns a raw `*const` pointer.
        (*core::ptr::addr_of!(USB_ALLOC)).as_ref().unwrap()
    };

    let cdc = CdcAcm::new(alloc);

    let usb_dev = UsbDeviceBuilder::new(alloc, 0x2E8A, 0x000A)
        .device_release(0x0100)
        .manufacturer("Pico-RS")
        .product("USB Serial (Interrupt)")
        .serial_number("INT001")
        .composite_with_iads()
        .build();

    // Connect D+ pull-up before enabling interrupts.
    usb_dev.bus().connect();

    interrupt::free(|cs| {
        *USB_CDC.borrow(cs).borrow_mut() = Some(cdc);
        *USB_DEV.borrow(cs).borrow_mut() = Some(usb_dev);
    });
    info!("USB device configured and connected");

    interrupt::unpend(Interrupt::USBCTRL_IRQ);
    unsafe {
        interrupt::enable(Interrupt::USBCTRL_IRQ);
    }
    info!("USB interrupt enabled");

    // SysTick: 1 s tick for periodic messages.
    let mut systick = SysTick::new(cp.SYST, clocks.sys_freq());
    systick.disable();
    systick.set_reload(clocks.sys_freq()); // 1 s
    systick.enable_interrupt();
    systick.enable();
    info!("SysTick enabled (1 s)");

    // ── Application state ─────────────────────────────────────────────────────

    let mut line = LineAccumulator::new();
    let mut msg_count: u32 = 0;
    let mut periodic = true;
    let mut cmd_count: u32 = 0;
    let mut last_drops: u32 = 0;

    info!("Entering main loop");

    loop {
        while let Some(ev) = EVENTS.pop() {
            match ev {
                // ── Port opened ──────────────────────────────────────────────
                UsbEvent::Connected => {
                    info!("Port opened");
                    line.reset();
                    send(b"\r\n=== Pico-RS USB CDC (Interrupt Mode) ===\r\n");
                    send(b"Commands: HELP, PING, STATS, PERIODIC ON/OFF, ECHO <text>\r\n\r\n");
                }

                // ── Port closed ──────────────────────────────────────────────
                UsbEvent::Disconnected => {
                    info!("Port closed");
                    line.reset();
                }

                // ── Received data ────────────────────────────────────────────
                UsbEvent::RxPacket { len, data } => {
                    for &b in &data[..len] {
                        match line.feed(b) {
                            FeedResult::Incomplete => {}
                            FeedResult::Overflow => {
                                send(b"ERR line too long\r\n");
                            }
                            FeedResult::Ready => {
                                let cmd = line.line();
                                if !cmd.is_empty() {
                                    cmd_count += 1;
                                    handle_command(cmd, &mut periodic, cmd_count, msg_count);
                                }
                                line.reset();
                            }
                        }
                    }
                }

                // ── 100 ms tick ──────────────────────────────────────────────
                UsbEvent::TimerTick => {
                    if PORT_OPEN.load(Ordering::Relaxed) && periodic {
                        msg_count += 1;
                        let mut buf = [0u8; 64];
                        let msg = fmt_u32_msg(&mut buf, msg_count);
                        send(msg);
                    }
                }
            }
        }

        let drops = DROPPED.load(Ordering::Relaxed);
        if drops != last_drops {
            warn!("Dropped {} RX packets", drops - last_drops);
            last_drops = drops;
        }

        pico_rs::rt::asm::wfi();
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// Command handler
// ──────────────────────────────────────────────────────────────────────────────

fn handle_command(line: &str, periodic: &mut bool, cmd_count: u32, msg_count: u32) {
    if line.eq_ignore_ascii_case("HELP") {
        send(b"OK HELP PING STATS PERIODIC ON PERIODIC OFF ECHO <text>\r\n");
    } else if line.eq_ignore_ascii_case("PING") {
        send(b"OK PONG\r\n");
    } else if line.eq_ignore_ascii_case("STATS") {
        let mut buf = [0u8; 128];
        let msg = fmt_stats(&mut buf, *periodic, cmd_count, msg_count);
        send(msg);
    } else if line.eq_ignore_ascii_case("PERIODIC ON") {
        *periodic = true;
        send(b"OK periodic=on\r\n");
    } else if line.eq_ignore_ascii_case("PERIODIC OFF") {
        *periodic = false;
        send(b"OK periodic=off\r\n");
    } else if let Some(rest) = ascii_prefix(line, "ECHO ") {
        let mut buf = [0u8; MAX_LINE + 8];
        let msg = fmt_echo(&mut buf, rest);
        send(msg);
    } else {
        send(b"ERR unknown command\r\n");
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// USB send helper
// ──────────────────────────────────────────────────────────────────────────────

fn send(bytes: &[u8]) {
    let mut offset = 0;
    let mut spins = 0u32;

    while offset < bytes.len() {
        let result = interrupt::free(|cs| {
            if let Some(cdc) = USB_CDC.borrow(cs).borrow_mut().as_mut() {
                cdc.write(&bytes[offset..])
            } else {
                Err(UsbError::InvalidState)
            }
        });

        match result {
            Ok(n) if n > 0 => {
                offset += n;
                spins = 0;
            }
            Err(UsbError::WouldBlock) => {
                spins = spins.saturating_add(1);
                if spins > 1024 {
                    warn!("USB write stalled after {} bytes", offset);
                    break;
                }
                spin_loop();
            }
            _ => break,
        }
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// Formatting helpers (no_std, no alloc)
// ──────────────────────────────────────────────────────────────────────────────

fn ascii_prefix<'a>(s: &'a str, prefix: &str) -> Option<&'a str> {
    if s.len() >= prefix.len() && s[..prefix.len()].eq_ignore_ascii_case(prefix) {
        Some(&s[prefix.len()..])
    } else {
        None
    }
}

fn write_bytes(buf: &mut [u8], pos: usize, src: &[u8]) -> usize {
    let n = core::cmp::min(src.len(), buf.len().saturating_sub(pos));
    buf[pos..pos + n].copy_from_slice(&src[..n]);
    n
}

fn write_u32(buf: &mut [u8], pos: usize, v: u32) -> usize {
    let mut digits = [0u8; 10];
    let mut count = 0;
    let mut n = v;
    if n == 0 {
        digits[0] = b'0';
        count = 1;
    } else {
        while n > 0 && count < digits.len() {
            digits[count] = b'0' + (n % 10) as u8;
            count += 1;
            n /= 10;
        }
    }
    let mut w = 0;
    for i in (0..count).rev() {
        if pos + w >= buf.len() {
            break;
        }
        buf[pos + w] = digits[i];
        w += 1;
    }
    w
}

fn fmt_u32_msg(buf: &mut [u8; 64], count: u32) -> &[u8] {
    let mut p = 0;
    p += write_bytes(buf, p, b"[");
    p += write_u32(buf, p, count);
    p += write_bytes(buf, p, b"] Hello from RP2350 USB CDC!\r\n");
    &buf[..p]
}

fn fmt_stats(buf: &mut [u8; 128], periodic: bool, cmds: u32, msgs: u32) -> &[u8] {
    let connected = PORT_OPEN.load(Ordering::Relaxed);
    let drops = DROPPED.load(Ordering::Relaxed);
    let queue = EVENTS.len() as u32;
    let mut p = 0;
    p += write_bytes(buf, p, b"OK connected=");
    buf[p] = if connected { b'1' } else { b'0' };
    p += 1;
    p += write_bytes(buf, p, b" periodic=");
    buf[p] = if periodic { b'1' } else { b'0' };
    p += 1;
    p += write_bytes(buf, p, b" cmds=");
    p += write_u32(buf, p, cmds);
    p += write_bytes(buf, p, b" msgs=");
    p += write_u32(buf, p, msgs);
    p += write_bytes(buf, p, b" drops=");
    p += write_u32(buf, p, drops);
    p += write_bytes(buf, p, b" queue=");
    p += write_u32(buf, p, queue);
    p += write_bytes(buf, p, b"\r\n");
    &buf[..p]
}

fn fmt_echo<'a>(buf: &'a mut [u8; MAX_LINE + 8], payload: &str) -> &'a [u8] {
    let mut p = 0;
    p += write_bytes(buf, p, b"OK ");
    p += write_bytes(buf, p, payload.as_bytes());
    p += write_bytes(buf, p, b"\r\n");
    &buf[..p]
}
