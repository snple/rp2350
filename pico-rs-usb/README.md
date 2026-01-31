# pico-rs-usb

USB Device Stack for RP2350 microcontrollers (Raspberry Pi Pico 2).

## Features

- 🚀 Simple, builder-pattern API for device configuration
- 📦 Support for USB device classes (CDC ACM for serial communication)
- ⚡ Efficient interrupt-driven operation
- 📝 defmt logging support
- 🔧 Zero external dependencies beyond pico-rs-pac

## Architecture

The USB stack is organized in layers:

- **Bus Layer** (`UsbBus`): Low-level hardware abstraction for the RP2350 USB controller
- **Device Layer** (`UsbDevice`): USB device state machine and control transfers
- **Class Layer**: USB class implementations (CDC, HID, etc.)

## Quick Start

```rust
#![no_std]
#![no_main]

use pico_rs_usb::{UsbDevice, UsbBus};
use pico_rs_usb::class::cdc::CdcAcm;

#[entry]
fn main() -> ! {
    // Create USB bus (static lifetime required)
    static USB_BUS: UsbBus = UsbBus::new();

    // Create CDC ACM class (virtual serial port)
    let mut serial = CdcAcm::new(&USB_BUS);

    // Build USB device with descriptors
    let mut usb_dev = UsbDevice::builder(&USB_BUS)
        .vendor_id(0x1234)
        .product_id(0x5678)
        .manufacturer("My Company")
        .product("My USB Device")
        .serial_number("001")
        .build();

    // Initialize USB
    usb_dev.init().expect("USB init failed");

    let mut buf = [0u8; 64];

    loop {
        // Poll for USB events
        usb_dev.poll(&mut [&mut serial]);

        // Handle serial data when configured
        if serial.is_configured() && serial.port_open() {
            if let Ok(count) = serial.read(&mut buf) {
                // Echo received data
                let _ = serial.write(&buf[..count]);
            }
        }
    }
}
```

## USB Classes

### CDC ACM (Serial Port)

The CDC ACM class provides a virtual serial port over USB:

```rust
use pico_rs_usb::class::cdc::CdcAcm;

let mut serial = CdcAcm::new(&usb_bus);

// Check if host opened the port (DTR signal)
if serial.port_open() {
    // Write data
    serial.write(b"Hello, USB!\r\n")?;

    // Read data
    let mut buf = [0u8; 64];
    if let Ok(n) = serial.read(&mut buf) {
        // Process received data
    }
}

// Get line coding (baud rate, etc.)
let coding = serial.line_coding();
println!("Baud rate: {}", coding.baud_rate());
```

## API Overview

### UsbDevice Builder

```rust
UsbDevice::builder(&usb_bus)
    .vendor_id(0x1234)          // USB Vendor ID
    .product_id(0x5678)         // USB Product ID
    .device_release(0x0100)     // Device version (BCD)
    .manufacturer("Company")     // Manufacturer string
    .product("Product Name")     // Product string
    .serial_number("SN001")      // Serial number string
    .max_power(100)              // Max power in mA
    .self_powered(false)         // Self-powered flag
    .remote_wakeup(false)        // Remote wakeup capability
    .build()
```

### Device State

```rust
// Check device state
match usb_dev.state() {
    DeviceState::Configured => { /* Ready to use */ }
    DeviceState::Addressed => { /* Has address, not configured */ }
    DeviceState::Suspended => { /* Bus suspended */ }
    _ => { /* Not ready */ }
}

// Convenience method
if usb_dev.is_configured() {
    // Device is ready for data transfer
}
```

## Endpoint Types

The stack supports all USB endpoint types:

- **Control**: Used for device configuration (EP0)
- **Bulk**: High-throughput data transfer (CDC data)
- **Interrupt**: Low-latency, small packets (CDC notifications)
- **Isochronous**: Guaranteed bandwidth (audio/video)

## Memory Layout

The RP2350 USB controller uses 4KB of DPRAM for endpoint buffers:

- First 256 bytes: Reserved for endpoint control registers
- Remaining space: Dynamically allocated for endpoint buffers

## Error Handling

All fallible operations return `UsbResult<T>`:

```rust
use pico_rs_usb::error::{UsbError, UsbResult};

match serial.write(data) {
    Ok(n) => println!("Wrote {} bytes", n),
    Err(UsbError::WouldBlock) => { /* Buffer full, retry later */ }
    Err(UsbError::NotReady) => { /* Device not configured */ }
    Err(e) => println!("Error: {:?}", e),
}
```

## VID/PID

For development and testing, you can use:
- pid.codes: `0x1209:0x0001` (free for open source projects)
- Your own registered VID/PID for production

**Note**: Using someone else's VID/PID is against USB specifications.

## License

MIT OR Apache-2.0
