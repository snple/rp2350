//! USB CDC Serial Example
//!
//! This example demonstrates how to create a USB serial port using CDC ACM.
//! The device will echo any data received back to the host.
//!
//! ## Usage
//!
//! 1. Flash this program to your Pico 2
//! 2. Connect the Pico 2 to your computer via USB
//! 3. Open a serial terminal (e.g., `picocom /dev/ttyACM0`)
//! 4. Type something and see it echoed back

#![no_std]
#![no_main]

use pico_rs::entry;
use pico_rs_usb::class::cdc::CdcAcm;
use pico_rs_usb::{UsbBus, UsbDevice};

#[cfg(feature = "defmt")]
use defmt_rtt as _;
#[cfg(feature = "defmt")]
use panic_probe as _;

#[cfg(not(feature = "defmt"))]
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

/// USB VID/PID - Use your own for production!
/// These are from pid.codes, intended for testing
const USB_VID: u16 = 0x1209;
const USB_PID: u16 = 0x0001;

#[entry]
fn main() -> ! {
    // Create USB bus
    static USB_BUS: UsbBus = UsbBus::new();

    // Create CDC ACM (serial port) class
    let mut serial = CdcAcm::new(&USB_BUS);

    // Build USB device
    let mut usb_dev = UsbDevice::builder(&USB_BUS)
        .vendor_id(USB_VID)
        .product_id(USB_PID)
        .manufacturer("Pico-RS")
        .product("USB Serial Demo")
        .serial_number("001")
        .max_power(100)
        .build();

    // Initialize USB
    usb_dev.init().expect("Failed to initialize USB");

    #[cfg(feature = "defmt")]
    defmt::info!("USB CDC Serial example started");

    let mut buf = [0u8; 64];

    loop {
        // Poll USB for events
        usb_dev.poll(&mut [&mut serial]);

        // Check if port is open (DTR is set)
        if serial.dtr_changed() {
            if serial.port_open() {
                #[cfg(feature = "defmt")]
                defmt::info!("Serial port opened");

                // Send welcome message
                let _ = serial.write_all(b"Welcome to Pico-RS USB Serial!\r\n");
            } else {
                #[cfg(feature = "defmt")]
                defmt::info!("Serial port closed");
            }
        }

        // Echo received data
        if serial.is_configured() && serial.port_open() {
            if let Ok(count) = serial.read(&mut buf) {
                if count > 0 {
                    // Echo back
                    let _ = serial.write(&buf[..count]);
                }
            }
        }
    }
}
