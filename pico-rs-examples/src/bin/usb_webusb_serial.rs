//! # WebUSB CDC Serial Example
//!
//! This example demonstrates WebUSB with CDC ACM (serial port) functionality.
//! The device can be accessed from a web browser using the WebUSB API.
//!
//! ## Hardware
//! - Raspberry Pi Pico 2 (RP2350)
//! - USB cable connected to the Pico's USB port
//!
//! ## Testing
//! 1. Flash this example to the Pico 2
//! 2. Connect USB cable
//! 3. Open Chrome/Edge and navigate to the landing page URL
//! 4. Click "Connect" and select the device
//! 5. The web page can now communicate with the device
//!
//! ## Web Page Example
//! ```html
//! <button onclick="connect()">Connect</button>
//! <script>
//!   async function connect() {
//!     const device = await navigator.usb.requestDevice({
//!       filters: [{ vendorId: 0x2E8A }]
//!     });
//!     await device.open();
//!     await device.selectConfiguration(1);
//!     await device.claimInterface(0);
//!     // Now you can transfer data...
//!   }
//! </script>
//! ```
//!
//! ## Notes
//! - WebUSB requires HTTPS or localhost
//! - Only Chrome/Edge/Opera support WebUSB
//! - The landing page URL helps users find the correct web interface

#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

use pico_rs::block::ImageDef;
use pico_rs::clocks::ClockBuilder;
use pico_rs::entry;
use pico_rs::systick::Delay;

use pico_rs_usb::bus::UsbBus;
use pico_rs_usb::class::cdc_acm::CdcAcm;
use pico_rs_usb::class::webusb::WebUsb;
use pico_rs_usb::device::UsbDeviceBuilder;

/// RP2350 Boot Block
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::secure_exe();

/// Target system clock frequency (150 MHz)
const SYS_CLK_HZ: u32 = 150_000_000;

/// Landing page URL (without https:// prefix)
/// This is where browsers will direct users when they connect the device
const LANDING_PAGE_URL: &str = "example.com/pico-webusb";

#[entry]
fn main() -> ! {
    info!("WebUSB CDC Serial Example starting...");

    // Initialize clock system (150 MHz sys, 48 MHz USB)
    let clocks = ClockBuilder::new().enable_usb().build();
    info!(
        "Clock system initialized: sys={} Hz, usb={} Hz",
        clocks.sys_freq(),
        clocks.usb_freq()
    );

    // Initialize USB bus
    let usb_bus = UsbBus::new();
    let _ = usb_bus.init();
    info!("USB bus initialized");

    // Create WebUSB class with landing page
    let mut webusb = WebUsb::new(LANDING_PAGE_URL);
    info!(
        "WebUSB class created with URL: https://{}",
        LANDING_PAGE_URL
    );

    // Create CDC ACM class
    let mut cdc = CdcAcm::new(&usb_bus);
    info!("CDC ACM class created");

    // Build USB device with WebUSB support
    // Using USB 2.1 for BOS descriptor support
    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus)
        .vendor_id(0x2E8A) // Raspberry Pi
        .product_id(0x000C) // WebUSB device
        .device_release(0x0100)
        .manufacturer("Raspberry Pi")
        .product("Pico WebUSB Serial")
        .serial_number("WEBUSB001")
        .build();

    info!("USB device built, connecting...");

    // Connect to host
    usb_bus.connect();
    info!("USB connected, waiting for enumeration...");

    let mut delay = Delay::new(SYS_CLK_HZ);
    let mut read_buf = [0u8; 64];
    let mut counter: u32 = 0;
    let mut last_send = 0u32;

    loop {
        // Poll USB - WebUSB must be polled first for BOS descriptor handling
        usb_dev.poll(&mut [&mut webusb]);
        usb_dev.poll(&mut [&mut cdc]);

        // Check if CDC port is open
        if cdc.port_open() {
            // Read any incoming data and echo it back
            if let Ok(count) = cdc.read(&mut read_buf) {
                if count > 0 {
                    info!("Received {} bytes: {:02x}", count, &read_buf[..count]);

                    // Echo back with prefix
                    let _ = cdc.write_all(b"Echo: ");
                    let _ = cdc.write_all(&read_buf[..count]);
                    let _ = cdc.write_all(b"\r\n");
                }
            }

            // Send periodic message
            counter = counter.wrapping_add(1);
            if counter.wrapping_sub(last_send) > 100000 {
                last_send = counter;
                let msg = b"Hello from Pico WebUSB!\r\n";
                match cdc.write_all(msg) {
                    Ok(_) => debug!("Sent message"),
                    Err(e) => warn!("Write error: {:?}", e),
                }
            }
        }

        // Small delay to prevent busy loop
        delay.delay_us(10);
    }
}
