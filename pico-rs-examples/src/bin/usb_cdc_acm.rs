//! # USB CDC Serial Example
//!
//! This example demonstrates USB CDC ACM (virtual serial port) functionality.
//! Connect the Pico 2 via USB and it will appear as a serial device.
//!
//! ## Hardware
//! - Raspberry Pi Pico 2 (RP2350)
//! - USB cable connected to the Pico's USB port
//!
//! ## Testing
//! 1. Flash this example to the Pico 2
//! 2. Connect USB cable
//! 3. Open a serial terminal (e.g., `minicom`, `screen`, or `picocom`)
//! 4. You should see "Hello from RP2350 USB CDC!" printed every second
//! 5. Any characters you type will be echoed back

#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

use pico_rs::block::ImageDef;
use pico_rs::clocks::ClockBuilder;
use pico_rs::entry;
use pico_rs::pac::Peripherals;
use pico_rs::rt::peripheral::Peripherals as CorePeripherals;
use pico_rs::systick::Delay;

use pico_rs_usb::bus::UsbBus;
use pico_rs_usb::class::cdc_acm::CdcAcm;
use pico_rs_usb::device::UsbDeviceBuilder;

/// RP2350 Boot Block
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::exe();

/// Target system clock frequency (150 MHz)
const SYS_CLK_HZ: u32 = 150_000_000;

#[entry]
fn main() -> ! {
    info!("USB CDC Serial Example starting...");

    // Take device peripherals
    let dp = Peripherals::take().unwrap();

    // Initialize clock system (150 MHz sys, 48 MHz USB)
    let clocks = ClockBuilder::new()
        .enable_usb()
        .build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);
    info!(
        "Clock system initialized: sys={} Hz, usb={} Hz",
        clocks.sys_freq(),
        clocks.usb_freq()
    );

    // Initialize USB bus
    let usb_bus = UsbBus::new();
    let _ = usb_bus.init();
    info!("USB bus initialized");

    // Create CDC ACM class
    let mut cdc = CdcAcm::new(&usb_bus);
    info!("CDC ACM class created");

    // Build USB device
    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus)
        .vendor_id(0x2E8A) // Raspberry Pi
        .product_id(0x000A) // Pico SDK CDC
        .device_release(0x0100)
        .manufacturer("Raspberry Pi")
        .product("Pico USB Serial")
        .serial_number("123456")
        .build();

    info!("USB device built, connecting...");

    // Connect to host
    usb_bus.connect();
    info!("USB connected, waiting for enumeration...");

    // Take core peripherals for Delay
    let core_peripherals = CorePeripherals::take().unwrap();

    let mut delay = Delay::new(core_peripherals.SYST, SYS_CLK_HZ);
    let mut read_buf = [0u8; 64];
    let mut counter: u32 = 0;
    let mut last_send = 0u32;

    loop {
        // Poll USB
        usb_dev.poll(&mut [&mut cdc]);

        // Check if port is open
        if cdc.port_open() {
            // Read any incoming data and echo it back
            if let Ok(count) = cdc.read(&mut read_buf) {
                if count > 0 {
                    info!("Received {} bytes: {:02x}", count, &read_buf[..count]);

                    // Echo back
                    let _ = cdc.write_all(&read_buf[..count]);
                }
            }

            // Send periodic message
            counter = counter.wrapping_add(1);
            if counter.wrapping_sub(last_send) > 100000 {
                last_send = counter;
                let msg = b"Hello from RP2350 USB CDC!\r\n";
                match cdc.write_all(msg) {
                    Ok(_) => info!("Sent message"),
                    Err(e) => warn!("Write error: {:?}", e),
                }
            }
        }

        // Small delay to prevent busy loop
        delay.delay_us(10);
    }
}
