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

use roschi::SysTick;
use roschi::block::ImageDef;
use roschi::clocks::ClockBuilder;
use roschi::entry;
use roschi::pac::Peripherals;
use roschi::rt::peripheral::Peripherals as CorePeripherals;

use roschi_usb::bus::UsbBusAllocator;
use roschi_usb::class::UsbClass;
use roschi_usb::class::cdc_acm::CdcAcm;
use roschi_usb::device::UsbDeviceBuilder;
use roschi_usb::rp2350::Rp2350Bus;

/// RP2350 Boot Block
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::exe();

/// Target system clock frequency (150 MHz)
const SYS_CLK_HZ: u32 = 150_000_000;

#[entry]
fn main() -> ! {
    info!("USB CDC Serial Example starting...");

    let dp = Peripherals::take().unwrap();
    let cp = CorePeripherals::take().unwrap();

    // Initialize clock system (150 MHz sys, 48 MHz USB)
    let clocks = ClockBuilder::new()
        .enable_usb()
        .build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);
    info!(
        "Clocks: sys={} Hz, usb={} Hz",
        clocks.sys_freq(),
        clocks.usb_freq()
    );

    // ── USB setup ────────────────────────────────────────────────────────────
    //
    // 1. Create a bus allocator wrapping the RP2350 hardware driver.
    // 2. Allocate class endpoints (CDC ACM needs 3 endpoints + 2 interfaces).
    // 3. Build the device — this freezes the allocator and enables the peripheral.
    // 4. Connect the D+ pull-up to signal presence to the host.

    let alloc = UsbBusAllocator::new(unsafe { Rp2350Bus::steal() });

    let mut cdc = CdcAcm::new(&alloc);

    let mut usb_dev = UsbDeviceBuilder::new(&alloc, 0x2E8A, 0x000A) // Raspberry Pi, Pico SDK CDC
        .device_release(0x0100)
        .manufacturer("Raspberry Pi")
        .product("Pico USB Serial")
        .serial_number("123456")
        .composite_with_iads()
        .build();

    // Connect D+ pull-up: signals USB presence to the host.
    usb_dev.bus().connect();
    info!("USB connected, waiting for enumeration...");

    // ── Main loop ─────────────────────────────────────────────────────────────

    let systick = SysTick::new(cp.SYST, SYS_CLK_HZ);
    let mut read_buf = [0u8; 64];
    let mut counter: u32 = 0;
    let mut last_send: u32 = 0;

    loop {
        // Drive the USB state machine.  Must be called at ≤ 10 ms intervals.
        usb_dev.poll(&mut [&mut cdc as &mut dyn UsbClass<Rp2350Bus>]);

        // Only interact when the host has configured the device and opened the
        // port (DTR set by the host terminal application).
        if cdc.port_open() {
            // ── Receive and echo ─────────────────────────────────────────────
            match cdc.read(&mut read_buf) {
                Ok(n) if n > 0 => {
                    info!("RX {} bytes: {:02x}", n, &read_buf[..n]);

                    // Echo back; write() sends up to one full packet at a time.
                    let mut sent = 0;
                    while sent < n {
                        match cdc.write(&read_buf[sent..n]) {
                            Ok(written) => sent += written,
                            Err(_) => break,
                        }
                    }
                }
                _ => {}
            }

            // ── Periodic hello ───────────────────────────────────────────────
            counter = counter.wrapping_add(1);
            if counter.wrapping_sub(last_send) > 100_000 {
                last_send = counter;
                match cdc.write(b"Hello from RP2350 USB CDC!\r\n") {
                    Ok(_) => info!("Sent periodic message"),
                    Err(e) => warn!("Write error: {:?}", e),
                }
            }
        }

        // Yield for ~10 µs to avoid monopolising the CPU.
        systick.delay_us(10);
    }
}
