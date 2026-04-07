//! # USB CDC Serial with Interrupt + Event Example
//!
//! This example demonstrates USB CDC ACM using interrupt-driven USB handling
//! combined with an event queue for communication between interrupt and main loop.
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
//! ## Benefits
//!
//! - **Responsive**: USB events are handled immediately in interrupt
//! - **Composable**: Can easily add HID, keyboard, etc. alongside CDC
//! - **Simple**: Uses existing Event system, no complex abstractions
//!
//! ## Hardware
//! - Raspberry Pi Pico 2 (RP2350)
//! - USB cable connected to the Pico's USB port

#![no_std]
#![no_main]

use core::cell::RefCell;
use core::hint::spin_loop;
use core::str;
use core::sync::atomic::{AtomicU32, Ordering};

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

use pico_rs_usb::bus::UsbBus;
use pico_rs_usb::class::cdc_acm::CdcAcm;
use pico_rs_usb::device::{UsbDevice, UsbDeviceBuilder};
use pico_rs_usb::error::UsbError;

/// RP2350 Boot Block
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::exe();

// ============================================================================
// Constants
// ============================================================================

/// Target system clock frequency (150 MHz)
const SYS_CLK_HZ: u32 = 150_000_000;

// ============================================================================
// Event Definitions
// ============================================================================

/// Maximum number of bytes handed from ISR to main loop per event.
const RX_CHUNK_SIZE: usize = 64;

/// Maximum number of queued USB/timer events.
const EVENT_QUEUE_SIZE: usize = 16;

/// Maximum supported input line length.
const MAX_LINE_LEN: usize = 96;

#[derive(Clone, Copy)]
enum UsbEvent {
    Connected,
    Disconnected,
    TimerTick,
    RxPacket {
        len: usize,
        data: [u8; RX_CHUNK_SIZE],
    },
}

enum LineFeedResult {
    None,
    Ready,
    Overflow,
}

struct LineAccumulator {
    buf: [u8; MAX_LINE_LEN],
    len: usize,
    overflow: bool,
}

impl LineAccumulator {
    const fn new() -> Self {
        Self {
            buf: [0u8; MAX_LINE_LEN],
            len: 0,
            overflow: false,
        }
    }

    fn reset(&mut self) {
        self.len = 0;
        self.overflow = false;
    }

    fn feed(&mut self, byte: u8) -> LineFeedResult {
        match byte {
            b'\n' | b'\r' => {
                if self.overflow {
                    self.reset();
                    LineFeedResult::Overflow
                } else if self.len == 0 {
                    LineFeedResult::None
                } else {
                    LineFeedResult::Ready
                }
            }
            _ => {
                if self.len < self.buf.len() {
                    self.buf[self.len] = byte;
                    self.len += 1;
                } else {
                    self.overflow = true;
                }
                LineFeedResult::None
            }
        }
    }

    fn line(&self) -> Result<&str, ()> {
        str::from_utf8(&self.buf[..self.len])
            .map(|line| line.trim())
            .map_err(|_| ())
    }
}

struct AppState {
    line_accumulator: LineAccumulator,
    periodic_messages_enabled: bool,
    command_count: u32,
}

impl AppState {
    const fn new() -> Self {
        Self {
            line_accumulator: LineAccumulator::new(),
            periodic_messages_enabled: true,
            command_count: 0,
        }
    }
}

/// Global event queue.
static EVENTS: GenericQueue<UsbEvent, EVENT_QUEUE_SIZE> = GenericQueue::new();

/// Number of RX packets dropped because the event queue was full.
static RX_DROPPED_PACKETS: AtomicU32 = AtomicU32::new(0);

// ============================================================================
// USB Globals (for interrupt access)
// ============================================================================

/// USB Bus storage, initialized at runtime before USB interrupts are enabled.
static USB_BUS: Mutex<RefCell<Option<UsbBus>>> = Mutex::new(RefCell::new(None));

/// USB Device - stored separately for interrupt access
static USB_DEV: Mutex<RefCell<Option<UsbDevice<'static>>>> = Mutex::new(RefCell::new(None));

/// USB CDC Class
static USB_CDC: Mutex<RefCell<Option<CdcAcm<'static>>>> = Mutex::new(RefCell::new(None));

/// Track port open state
static PORT_WAS_OPEN: Mutex<RefCell<bool>> = Mutex::new(RefCell::new(false));

// ============================================================================
// USB Interrupt Handler
// ============================================================================

/// USB interrupt handler
///
/// This handles all USB events and posts events to the queue for main loop processing.
#[bind(USBCTRL_IRQ)]
fn usbctrl_irq() {
    debug!("USB interrupt");
    interrupt::free(|cs| {
        let mut dev_ref = USB_DEV.borrow(cs).borrow_mut();
        let mut cdc_ref = USB_CDC.borrow(cs).borrow_mut();

        if let (Some(dev), Some(cdc)) = (dev_ref.as_mut(), cdc_ref.as_mut()) {
            dev.poll(&mut [cdc as &mut dyn pico_rs_usb::device::UsbClass]);

            let port_open = cdc.port_open() || cdc.is_configured();
            let was_open = *PORT_WAS_OPEN.borrow(cs).borrow();

            if port_open && !was_open {
                *PORT_WAS_OPEN.borrow(cs).borrow_mut() = true;
                let _ = EVENTS.push(UsbEvent::Connected);
            } else if !port_open && was_open {
                *PORT_WAS_OPEN.borrow(cs).borrow_mut() = false;
                let _ = EVENTS.push(UsbEvent::Disconnected);
            }

            if port_open {
                loop {
                    debug!("Polling for RX data...");
                    let mut rx_buf = [0u8; RX_CHUNK_SIZE];
                    match cdc.read(&mut rx_buf) {
                        Ok(count) if count > 0 => {
                            let event = UsbEvent::RxPacket {
                                len: count,
                                data: rx_buf,
                            };
                            if !EVENTS.push(event) {
                                RX_DROPPED_PACKETS.fetch_add(1, Ordering::Relaxed);
                                break;
                            }
                        }
                        Ok(_) => break,
                        Err(_) => break,
                    }
                }
            }
        }
    });

    // Re-arm the OUT endpoint in PendSV (lowest-priority exception) rather
    // than right here in the USB ISR.  This prevents the interrupt storm:
    // PendSV only fires after all higher-priority exceptions are done, so the
    // hardware accepts the next host packet only after reaching PendSV context.
    trigger_pendsv();
}

// ============================================================================
// PendSV — deferred (lowest-priority) USB re-arm
// ============================================================================

/// Set PendSV to the lowest possible exception priority (0xFF).
///
/// PendSV must be lower priority than the USB IRQ so that it only runs after
/// all higher-priority exceptions (including USBCTRL_IRQ) have completed.
fn configure_pendsv_priority() {
    unsafe {
        // SCB.SHPR3 at 0xE000_ED20
        // Bits [23:16] = PendSV priority (exception 14)
        let shpr3 = 0xE000_ED20 as *mut u32;
        let current = shpr3.read_volatile();
        shpr3.write_volatile((current & !0x00FF_0000) | (0xFF_u32 << 16));
    }
}

/// Pend the PendSV exception so it fires after all higher-priority ISRs.
///
/// Writing PENDSVSET (bit 28) to SCB.ICSR is idempotent — if PendSV is
/// already pending it stays pending, so calling this multiple times is safe.
#[inline]
fn trigger_pendsv() {
    unsafe {
        // SCB.ICSR at 0xE000_ED04, bit 28 = PENDSVSET
        (0xE000_ED04 as *mut u32).write_volatile(1 << 28);
    }
}

/// PendSV exception handler — deferred USB RX endpoint re-arm.
///
/// The USB ISR sets PendSV pending instead of re-arming the OUT endpoint
/// directly.  PendSV has the lowest exception priority and only runs when no
/// higher-priority exception is active, preventing an interrupt storm: the
/// hardware cannot accept the next host packet until this handler fires and
/// calls `rearm_rx()`, giving the main loop at least one opportunity to run
/// between consecutive host packets.
#[exception]
fn PendSV() {
    interrupt::free(|cs| {
        if let Some(cdc) = USB_CDC.borrow(cs).borrow_mut().as_mut() {
            cdc.rearm_rx();
        }
    });
}

// ============================================================================
// Timer setup (using SysTick for periodic events)
// ============================================================================

fn setup_systick(systick: &mut SysTick, reload_value: u32) {
    systick.disable();
    systick.set_reload(reload_value);
    systick.enable_interrupt();
    systick.enable();
}

/// SysTick exception handler
#[exception]
fn SysTick() {
    let _ = EVENTS.push(UsbEvent::TimerTick);
}

// ============================================================================
// Main Entry Point
// ============================================================================

#[entry]
fn main() -> ! {
    info!("USB CDC Interrupt Example starting...");

    // Take device peripherals
    let dp = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();

    // Initialize clock system (150 MHz sys, 48 MHz USB)
    let clocks = ClockBuilder::new()
        .enable_usb()
        .build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);
    info!(
        "Clock system initialized: sys={} Hz, usb={} Hz",
        clocks.sys_freq(),
        clocks.usb_freq()
    );

    let bus_ptr = interrupt::free(|cs| {
        *USB_BUS.borrow(cs).borrow_mut() = Some(UsbBus::new());

        let option_ptr = USB_BUS.borrow(cs).as_ptr();
        unsafe { (*option_ptr).as_ref().unwrap() as *const UsbBus }
    });
    let bus_ref: &'static UsbBus = unsafe { &*bus_ptr };

    let cdc = CdcAcm::new(bus_ref);
    info!("CDC ACM class created");

    let mut usb_dev = UsbDeviceBuilder::new(bus_ref)
        .vendor_id(0x2E8A) // Raspberry Pi
        .product_id(0x000A) // Pico SDK CDC
        .device_release(0x0100)
        .manufacturer("Pico-RS")
        .product("USB Serial (Interrupt)")
        .serial_number("INT001")
        .build();

    match usb_dev.init() {
        Ok(()) => info!("USB device initialized"),
        Err(err) => warn!("USB device init failed: {:?}", err),
    }

    interrupt::free(|cs| {
        *USB_CDC.borrow(cs).borrow_mut() = Some(cdc);
        *USB_DEV.borrow(cs).borrow_mut() = Some(usb_dev);
    });
    info!("USB device built and connected");

    // Set PendSV to the lowest exception priority so it only runs after the
    // USB ISR (and any other higher-priority ISRs) have completed.
    configure_pendsv_priority();

    // Enable USB interrupt in NVIC
    interrupt::unpend(Interrupt::USBCTRL_IRQ);
    unsafe {
        interrupt::enable(Interrupt::USBCTRL_IRQ);
    }
    info!("USB interrupt enabled");

    // Setup SysTick for 1 second periodic events
    // SYS_CLK_HZ / 1 = 1 second, but SysTick is 24-bit max (16777215)
    // So we use a shorter interval and count in software
    let mut systick = SysTick::new(core.SYST, clocks.sys_freq());
    setup_systick(&mut systick, clocks.sys_freq() / 10); // 100ms ticks
    info!("SysTick configured for 100ms ticks");

    // Main loop variables
    let mut tick_counter = 0u32;
    let mut message_count = 0u32;
    let mut connected = false;
    let mut last_reported_drops = 0u32;
    let mut app = AppState::new();

    info!("Entering main loop...");

    loop {
        while let Some(event) = EVENTS.pop() {
            match event {
                UsbEvent::RxPacket { len, data } => {
                    info!("RX event: {} bytes", len);
                    info!("Data: {:02x}", &data[..len]);

                    for &byte in &data[..len] {
                        match app.line_accumulator.feed(byte) {
                            LineFeedResult::None => {}
                            LineFeedResult::Overflow => {
                                send_usb_text(b"ERR line too long (max 96 bytes)\r\n");
                            }
                            LineFeedResult::Ready => {
                                match app.line_accumulator.line() {
                                    Ok(line) if !line.is_empty() => {
                                        let mut line_buf = [0u8; MAX_LINE_LEN];
                                        let line_len = core::cmp::min(line.len(), line_buf.len());
                                        line_buf[..line_len]
                                            .copy_from_slice(&line.as_bytes()[..line_len]);
                                        app.command_count = app.command_count.wrapping_add(1);
                                        if let Ok(command_line) =
                                            str::from_utf8(&line_buf[..line_len])
                                        {
                                            handle_command(
                                                command_line,
                                                &mut app,
                                                connected,
                                                message_count,
                                                last_reported_drops,
                                            );
                                        } else {
                                            send_usb_text(b"ERR invalid UTF-8\r\n");
                                        }
                                    }
                                    Ok(_) => {}
                                    Err(()) => {
                                        send_usb_text(b"ERR invalid UTF-8\r\n");
                                    }
                                }
                                app.line_accumulator.reset();
                            }
                        }
                    }
                }

                UsbEvent::Connected => {
                    info!("USB port opened!");
                    connected = true;
                    app.line_accumulator.reset();

                    send_usb_text(b"\r\n=== Pico-RS USB CDC (Interrupt Mode) ===\r\n");
                    send_usb_text(b"Commands: HELP, PING, STATS, PERIODIC ON, PERIODIC OFF, ECHO <text>\r\n\r\n");
                }

                UsbEvent::Disconnected => {
                    info!("USB port closed!");
                    connected = false;
                    app.line_accumulator.reset();
                }

                UsbEvent::TimerTick => {
                    tick_counter += 1;

                    if tick_counter >= 10 {
                        tick_counter = 0;

                        if connected && app.periodic_messages_enabled {
                            message_count += 1;

                            let mut buf = [0u8; 64];
                            let msg = format_message(&mut buf, message_count);
                            send_usb_text(msg);

                            info!("Sent periodic message #{}", message_count);
                        }
                    }
                }
            }
        }

        let dropped = RX_DROPPED_PACKETS.load(Ordering::Relaxed);
        if dropped != last_reported_drops {
            warn!(
                "Dropped {} RX packets because the event queue was full",
                dropped - last_reported_drops
            );
            last_reported_drops = dropped;
        }

        pico_rs::rt::asm::wfi();
    }
}

fn handle_command(
    line: &str,
    app: &mut AppState,
    connected: bool,
    message_count: u32,
    dropped_packets: u32,
) {
    info!("Command: {}", line);

    if line.eq_ignore_ascii_case("HELP") {
        send_usb_text(b"OK HELP PING STATS PERIODIC ON PERIODIC OFF ECHO <text>\r\n");
    } else if line.eq_ignore_ascii_case("PING") {
        send_usb_text(b"OK PONG\r\n");
    } else if line.eq_ignore_ascii_case("STATS") {
        let mut buf = [0u8; 96];
        let msg = format_stats(
            &mut buf,
            connected,
            app.periodic_messages_enabled,
            app.command_count,
            message_count,
            dropped_packets,
            EVENTS.len(),
        );
        send_usb_text(msg);
    } else if line.eq_ignore_ascii_case("PERIODIC ON") {
        app.periodic_messages_enabled = true;
        send_usb_text(b"OK periodic=on\r\n");
    } else if line.eq_ignore_ascii_case("PERIODIC OFF") {
        app.periodic_messages_enabled = false;
        send_usb_text(b"OK periodic=off\r\n");
    } else if let Some(payload) = strip_prefix_ascii_case(line, "ECHO ") {
        let mut buf = [0u8; MAX_LINE_LEN + 16];
        let msg = format_echo(&mut buf, payload);
        send_usb_text(msg);
    } else {
        send_usb_text(b"ERR unknown command\r\n");
    }
}

fn send_usb_text(bytes: &[u8]) {
    let mut offset = 0;
    let mut would_block_spins = 0u32;

    while offset < bytes.len() {
        let write_result = interrupt::free(|cs| {
            if let Some(cdc) = USB_CDC.borrow(cs).borrow_mut().as_mut() {
                cdc.write(&bytes[offset..])
            } else {
                Err(UsbError::NotReady)
            }
        });

        match write_result {
            Ok(written) if written > 0 => {
                offset += written;
                would_block_spins = 0;
            }
            Ok(_) | Err(UsbError::WouldBlock) => {
                would_block_spins = would_block_spins.saturating_add(1);
                if would_block_spins > 1024 {
                    warn!("USB write stalled after {} bytes", offset);
                    break;
                }
                spin_loop();
            }
            Err(UsbError::NotReady) => {
                break;
            }
            Err(err) => {
                warn!("USB write error: {:?}", err);
                break;
            }
        }
    }
}

fn strip_prefix_ascii_case<'a>(line: &'a str, prefix: &str) -> Option<&'a str> {
    if line.len() < prefix.len() {
        return None;
    }

    let (candidate, rest) = line.split_at(prefix.len());
    if candidate.eq_ignore_ascii_case(prefix) {
        Some(rest)
    } else {
        None
    }
}

/// Format a periodic message into buffer
fn format_message(buf: &mut [u8], count: u32) -> &[u8] {
    // Simple formatting without alloc
    let prefix = b"[";
    let suffix = b"] Hello from RP2350 USB CDC (Interrupt mode)!\r\n";

    let mut pos = 0;

    // Copy prefix
    for &b in prefix {
        if pos < buf.len() {
            buf[pos] = b;
            pos += 1;
        }
    }

    // Write number
    let mut num = count;
    let mut digits = [0u8; 10];
    let mut digit_count = 0;

    if num == 0 {
        digits[0] = b'0';
        digit_count = 1;
    } else {
        while num > 0 {
            digits[digit_count] = b'0' + (num % 10) as u8;
            digit_count += 1;
            num /= 10;
        }
    }

    // Write digits in reverse
    for i in (0..digit_count).rev() {
        if pos < buf.len() {
            buf[pos] = digits[i];
            pos += 1;
        }
    }

    // Copy suffix
    for &b in suffix {
        if pos < buf.len() {
            buf[pos] = b;
            pos += 1;
        }
    }

    &buf[..pos]
}

fn format_stats(
    buf: &mut [u8],
    connected: bool,
    periodic_enabled: bool,
    command_count: u32,
    message_count: u32,
    dropped_packets: u32,
    queue_depth: usize,
) -> &[u8] {
    let mut pos = 0;

    pos += write_bytes(buf, pos, b"OK connected=");
    pos += write_bool_digit(buf, pos, connected);
    pos += write_bytes(buf, pos, b" periodic=");
    pos += write_bool_digit(buf, pos, periodic_enabled);
    pos += write_bytes(buf, pos, b" commands=");
    pos += write_u32(buf, pos, command_count);
    pos += write_bytes(buf, pos, b" messages=");
    pos += write_u32(buf, pos, message_count);
    pos += write_bytes(buf, pos, b" dropped=");
    pos += write_u32(buf, pos, dropped_packets);
    pos += write_bytes(buf, pos, b" queue=");
    pos += write_u32(buf, pos, queue_depth as u32);
    pos += write_bytes(buf, pos, b"\r\n");

    &buf[..pos]
}

fn format_echo<'a>(buf: &'a mut [u8], payload: &str) -> &'a [u8] {
    let mut pos = 0;
    pos += write_bytes(buf, pos, b"OK ");
    pos += write_bytes(buf, pos, payload.as_bytes());
    pos += write_bytes(buf, pos, b"\r\n");
    &buf[..pos]
}

fn write_bytes(buf: &mut [u8], pos: usize, bytes: &[u8]) -> usize {
    let len = core::cmp::min(bytes.len(), buf.len().saturating_sub(pos));
    buf[pos..pos + len].copy_from_slice(&bytes[..len]);
    len
}

fn write_bool_digit(buf: &mut [u8], pos: usize, value: bool) -> usize {
    if pos < buf.len() {
        buf[pos] = if value { b'1' } else { b'0' };
        1
    } else {
        0
    }
}

fn write_u32(buf: &mut [u8], pos: usize, value: u32) -> usize {
    let mut digits = [0u8; 10];
    let mut count = 0;
    let mut number = value;

    if number == 0 {
        digits[0] = b'0';
        count = 1;
    } else {
        while number > 0 && count < digits.len() {
            digits[count] = b'0' + (number % 10) as u8;
            count += 1;
            number /= 10;
        }
    }

    let mut written = 0;
    for index in (0..count).rev() {
        if pos + written >= buf.len() {
            break;
        }
        buf[pos + written] = digits[index];
        written += 1;
    }
    written
}
