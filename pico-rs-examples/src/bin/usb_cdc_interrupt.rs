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

use cortex_m::interrupt::Mutex;
use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

use cortex_m_rt::exception;
use pico_rs::block::ImageDef;
use pico_rs::clocks::{ClkPeriAuxSrc, ClkRefSrc, ClkSysAuxSrc, ClkUsbAuxSrc, Clocks};
use pico_rs::entry;
use pico_rs::interrupt::{self, Interrupt};
use pico_rs::pll::{PllConfig, PllSys, PllUsb};
use pico_rs::reset::{self, Peripheral};
use pico_rs::task::{Event, EventQueue};
use pico_rs::xosc::Xosc;

use pico_rs_usb::bus::UsbBus;
use pico_rs_usb::class::cdc::CdcAcm;
use pico_rs_usb::device::{UsbDevice, UsbDeviceBuilder};

/// RP2350 Boot Block
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::secure_exe();

// ============================================================================
// Constants
// ============================================================================

/// XOSC frequency (Pico 2 uses 12 MHz crystal)
const XOSC_HZ: u32 = 12_000_000;

/// Target system clock frequency (150 MHz)
const SYS_CLK_HZ: u32 = 150_000_000;

/// USB clock frequency (must be 48 MHz)
const USB_CLK_HZ: u32 = 48_000_000;

// ============================================================================
// Event Definitions
// ============================================================================

/// Event ID: USB data received
const EVT_USB_RX: u16 = 1;
/// Event ID: USB port opened
const EVT_USB_CONNECTED: u16 = 2;
/// Event ID: USB port closed
const EVT_USB_DISCONNECTED: u16 = 3;
/// Event ID: Timer tick (for periodic messages)
const EVT_TIMER_TICK: u16 = 4;

/// Global event queue
static EVENTS: EventQueue<16> = EventQueue::new();

// ============================================================================
// USB Globals (for interrupt access)
// ============================================================================

/// USB Bus - stored as Option because we initialize it later
static USB_BUS: Mutex<RefCell<Option<UsbBus>>> = Mutex::new(RefCell::new(None));

/// USB Device - stored separately for interrupt access
static USB_DEV: Mutex<RefCell<Option<UsbDevice<'static>>>> = Mutex::new(RefCell::new(None));

/// USB CDC Class
static USB_CDC: Mutex<RefCell<Option<CdcAcm<'static>>>> = Mutex::new(RefCell::new(None));

/// RX Buffer - filled by interrupt, read by main loop
static RX_BUFFER: Mutex<RefCell<[u8; 256]>> = Mutex::new(RefCell::new([0u8; 256]));
static RX_LEN: Mutex<RefCell<usize>> = Mutex::new(RefCell::new(0));

/// Track port open state
static PORT_WAS_OPEN: Mutex<RefCell<bool>> = Mutex::new(RefCell::new(false));

// ============================================================================
// USB Interrupt Handler
// ============================================================================

/// USB interrupt handler
///
/// This handles all USB events and posts events to the queue for main loop processing.
#[allow(non_snake_case)]
#[unsafe(no_mangle)]
fn USBCTRL_IRQ() {
    cortex_m::interrupt::free(|cs| {
        // Get mutable references to USB device and class
        let mut dev_ref = USB_DEV.borrow(cs).borrow_mut();
        let mut cdc_ref = USB_CDC.borrow(cs).borrow_mut();

        if let (Some(dev), Some(cdc)) = (dev_ref.as_mut(), cdc_ref.as_mut()) {
            // Poll USB - this handles all USB protocol stuff
            dev.poll(&mut [cdc]);

            let mut buf = [0u8; 64];
            if let Ok(count) = cdc.read(&mut buf) {
                if count > 0 {
                    defmt::debug!("USB RX: {} bytes", count);
                    cdc.write(&buf[..count]).ok();
                }
            }

            // Check for port open/close transitions
            // Note: Some hosts (Linux cdc-acm) may not send SET_CONTROL_LINE_STATE,
            // so we also use is_configured() to detect when the port is usable.
            // let port_open = cdc.port_open() || cdc.is_configured();
            // let was_open = *PORT_WAS_OPEN.borrow(cs).borrow();

            // if port_open && !was_open {
            //     EVENTS.push(Event::new(EVT_USB_CONNECTED, 0));
            //     *PORT_WAS_OPEN.borrow(cs).borrow_mut() = true;
            // } else if !port_open && was_open {
            //     EVENTS.push(Event::new(EVT_USB_DISCONNECTED, 0));
            //     *PORT_WAS_OPEN.borrow(cs).borrow_mut() = false;
            // }

            // Check for received data if device is configured
            // (don't wait for DTR, as some hosts don't send SET_CONTROL_LINE_STATE)
            // if cdc.is_configured() {

            // Store received data in global buffer
            // let mut rx_buf = RX_BUFFER.borrow(cs).borrow_mut();
            // let mut rx_len = RX_LEN.borrow(cs).borrow_mut();

            // // Only read if buffer is empty (main loop consumed previous data)
            // if *rx_len == 0 {
            //     if let Ok(count) = cdc.read(&mut rx_buf[..64]) {
            //         if count > 0 {
            //             *rx_len = count;
            //             EVENTS.push(Event::new(EVT_USB_RX, count as u16));
            //         }
            //     }
            // }
        }
        // }
    });
}

// ============================================================================
// Clock Initialization
// ============================================================================

/// Initialize clock system: XOSC -> PLL_SYS (150MHz) + PLL_USB (48MHz)
fn init_clocks() -> Clocks {
    info!("Initializing XOSC...");

    let xosc = Xosc::init(XOSC_HZ);
    xosc.wait_stable();
    info!("XOSC stable");

    let mut clocks = Clocks::new();

    // Configure ref clock from XOSC
    clocks.configure_ref_src(ClkRefSrc::Xosc, XOSC_HZ);
    info!("Ref clock: {} Hz", clocks.get_ref_freq());

    // Switch sys clock to ref clock temporarily
    clocks.configure_sys_from_ref();

    // Configure PLL_SYS for 150 MHz
    info!("Configuring PLL_SYS for 150 MHz...");
    reset::reset_block(Peripheral::PLL_SYS);
    reset::unreset_block_wait(Peripheral::PLL_SYS);

    let pll_sys = PllSys::new();
    let sys_config = PllConfig::sys_150mhz();
    pll_sys.configure(
        sys_config.refdiv,
        sys_config.fbdiv,
        sys_config.postdiv1,
        sys_config.postdiv2,
    );
    pll_sys.wait_lock();
    info!("PLL_SYS locked");

    // Configure PLL_USB for 48 MHz
    info!("Configuring PLL_USB for 48 MHz...");
    reset::reset_block(Peripheral::PLL_USB);
    reset::unreset_block_wait(Peripheral::PLL_USB);

    let pll_usb = PllUsb::new();
    let usb_config = PllConfig::usb_48mhz();
    pll_usb.configure(
        usb_config.refdiv,
        usb_config.fbdiv,
        usb_config.postdiv1,
        usb_config.postdiv2,
    );
    pll_usb.wait_lock();
    info!("PLL_USB locked");

    // Switch system clock to PLL_SYS
    clocks.configure_sys_aux(ClkSysAuxSrc::PllSys, SYS_CLK_HZ, SYS_CLK_HZ);
    info!("System clock: {} Hz", clocks.get_sys_freq());

    // Configure peripheral clock
    clocks.configure_peri(ClkPeriAuxSrc::ClkSys, SYS_CLK_HZ);

    // Configure USB clock from PLL_USB (48 MHz required for USB!)
    clocks.configure_usb(ClkUsbAuxSrc::PllUsb, USB_CLK_HZ);
    info!("USB clock: {} Hz", clocks.get_usb_freq());

    clocks
}

// ============================================================================
// Timer setup (using SysTick for periodic events)
// ============================================================================

fn setup_systick(reload_value: u32) {
    // Access SysTick using raw pointer
    let syst = unsafe { &*cortex_m::peripheral::SYST::PTR };

    // Configure SysTick
    unsafe {
        // Disable first
        syst.csr.write(0);
        // Set reload value (24-bit max)
        syst.rvr.write(reload_value & 0x00FF_FFFF);
        // Clear current value
        syst.cvr.write(0);
        // Enable with processor clock source and interrupt
        // Bit 0: ENABLE, Bit 1: TICKINT, Bit 2: CLKSOURCE
        syst.csr.write(0b111);
    }
}

/// SysTick exception handler
#[exception]
fn SysTick() {
    defmt::debug!("SysTick");
    EVENTS.push(Event::new(EVT_TIMER_TICK, 0));
}

// ============================================================================
// Main Entry Point
// ============================================================================

#[entry]
fn main() -> ! {
    info!("USB CDC Interrupt Example starting...");

    // Initialize clock system first (USB requires 48MHz clock!)
    let _clocks = init_clocks();
    info!("Clock system initialized");

    // Initialize USB in a critical section to set up globals
    cortex_m::interrupt::free(|cs| {
        // Create USB bus
        let usb_bus = UsbBus::new();
        let _ = usb_bus.init();
        info!("USB bus initialized");

        // Store bus in global (leak it for 'static lifetime)
        *USB_BUS.borrow(cs).borrow_mut() = Some(usb_bus);

        // Get 'static reference to the bus
        // Safety: The bus lives in a static and will never be dropped
        let bus_ref: &'static UsbBus = unsafe {
            let ptr = USB_BUS.borrow(cs).borrow().as_ref().unwrap() as *const UsbBus;
            &*ptr
        };

        // Create CDC ACM class
        let cdc = CdcAcm::new(bus_ref);
        *USB_CDC.borrow(cs).borrow_mut() = Some(cdc);
        info!("CDC ACM class created");

        // Build USB device
        let usb_dev = UsbDeviceBuilder::new(bus_ref)
            .vendor_id(0x2E8A) // Raspberry Pi
            .product_id(0x000A) // Pico SDK CDC
            .device_release(0x0100)
            .manufacturer("Pico-RS")
            .product("USB Serial (Interrupt)")
            .serial_number("INT001")
            .build();

        *USB_DEV.borrow(cs).borrow_mut() = Some(usb_dev);
        info!("USB device built");

        // Connect to host
        bus_ref.connect();
        info!("USB connected");
    });

    // Enable USB interrupt in NVIC
    unsafe {
        interrupt::enable(Interrupt::USBCTRL_IRQ);
    }
    info!("USB interrupt enabled");

    // Setup SysTick for 1 second periodic events
    // SYS_CLK_HZ / 1 = 1 second, but SysTick is 24-bit max (16777215)
    // So we use a shorter interval and count in software
    setup_systick(SYS_CLK_HZ / 10); // 100ms ticks
    info!("SysTick configured for 100ms ticks");

    // Main loop variables
    let mut tick_counter = 0u32;
    let mut message_count = 0u32;
    let mut connected = false;

    info!("Entering main loop...");

    loop {
        // Process events from queue
        while let Some(event) = EVENTS.pop() {
            match event.id {
                EVT_USB_RX => {
                    // Data received from USB
                    let count = event.data as usize;
                    info!("RX event: {} bytes", count);

                    // Read data from shared buffer and echo back
                    cortex_m::interrupt::free(|cs| {
                        let rx_buf = RX_BUFFER.borrow(cs).borrow();
                        let mut rx_len = RX_LEN.borrow(cs).borrow_mut();
                        let len = *rx_len;

                        if len > 0 {
                            // Log received data
                            info!("Data: {:02x}", &rx_buf[..len]);

                            // Echo back via CDC
                            if let Some(cdc) = USB_CDC.borrow(cs).borrow_mut().as_mut() {
                                let _ = cdc.write_all(&rx_buf[..len]);
                            }

                            // Mark buffer as consumed
                            *rx_len = 0;
                        }
                    });
                }

                EVT_USB_CONNECTED => {
                    info!("USB port opened!");
                    connected = true;

                    // Send welcome message
                    cortex_m::interrupt::free(|cs| {
                        if let Some(cdc) = USB_CDC.borrow(cs).borrow_mut().as_mut() {
                            let _ =
                                cdc.write_all(b"\r\n=== Pico-RS USB CDC (Interrupt Mode) ===\r\n");
                            let _ = cdc
                                .write_all(b"Type something and it will be echoed back.\r\n\r\n");
                        }
                    });
                }

                EVT_USB_DISCONNECTED => {
                    info!("USB port closed!");
                    connected = false;
                }

                EVT_TIMER_TICK => {
                    tick_counter += 1;

                    // Every 10 ticks (1 second), send a message if connected
                    if tick_counter >= 10 {
                        tick_counter = 0;

                        if connected {
                            message_count += 1;

                            cortex_m::interrupt::free(|cs| {
                                if let Some(cdc) = USB_CDC.borrow(cs).borrow_mut().as_mut() {
                                    // Format message with counter
                                    let mut buf = [0u8; 64];
                                    let msg = format_message(&mut buf, message_count);
                                    let _ = cdc.write_all(msg);
                                }
                            });

                            info!("Sent periodic message #{}", message_count);
                        }
                    }
                }

                _ => {
                    warn!("Unknown event: id={}, data={}", event.id, event.data);
                }
            }
        }

        // Wait for interrupt (low power)
        cortex_m::asm::wfi();
        // delay.delay_us(10);
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
