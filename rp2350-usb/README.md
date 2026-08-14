# rp2350-usb

USB Device Stack for RP2350 microcontrollers (Raspberry Pi Pico 2).

## Features

- Builder-pattern API for device configuration
- USB device classes:
  - **CDC ACM** — virtual serial port
  - **HID** — keyboard / mouse / custom HID
  - **GS_USB** — CAN bus adapter (candleLight protocol)
  - **UAC 1.0** — USB audio (speaker / microphone)
- Zero external dependencies beyond `rp2350-pac` / `rp2350-rt`
- Optional `defmt` logging

## Architecture

- **Bus layer** (`UsbBus` + `UsbBusAllocator`) — hardware abstraction. Classes
  allocate endpoints / interfaces / strings through the allocator before it is
  frozen.
- **Device layer** (`UsbDevice` / `UsbDeviceBuilder`) — device state machine and
  standard control-transfer handling.
- **Class layer** (`UsbClass`) — CDC ACM, HID, GS_USB, UAC.

## Quick start

```rust
use rp2350_usb::{UsbBusAllocator, UsbDeviceBuilder};
use rp2350_usb::class::cdc_acm::CdcAcm;
use rp2350_usb::rp2350::Rp2350Bus;

// 1. Wrap the hardware driver.
let alloc = UsbBusAllocator::new(unsafe { Rp2350Bus::steal() });

// 2. Create classes (allocates endpoints / interfaces / strings).
let mut serial = CdcAcm::new(&alloc);

// 3. Build the device — freezes the allocator and enables the peripheral.
let mut usb_dev = UsbDeviceBuilder::new(&alloc, 0x2E8A, 0x000A)
    .manufacturer("Raspberry Pi")
    .product("Pico USB Serial")
    .serial_number("123456")
    .composite_with_iads()
    .build();

// 4. Connect the D+ pull-up.
usb_dev.bus().connect();

loop {
    // Drive the state machine at least every 10 ms.
    if usb_dev.poll(&mut [&mut serial]) {
        let mut buf = [0u8; 64];
        if let Ok(n) = serial.read(&mut buf) {
            let _ = serial.write(&buf[..n]);
        }
    }
}
```

## USB classes

| Class | Path |
|-------|------|
| CDC ACM | `rp2350_usb::class::cdc_acm::CdcAcm` |
| HID | `rp2350_usb::class::hid::{HidClass, HidConfig, KeyboardReport, …}` |
| GS_USB | `rp2350_usb::class::gs_usb::GsUsbClass` |
| UAC 1.0 | `rp2350_usb::class::uac::AudioClassBuilder` |

## Builder

```rust
let mut usb_dev = UsbDeviceBuilder::new(&alloc, 0x1234, 0x5678) // VID, PID
    .device_release(0x0100)          // bcdDevice
    .manufacturer("Company")
    .product("Product")
    .serial_number("SN001")
    .self_powered(false)
    .supports_remote_wakeup(false)
    .max_power_ma(100)               // stored as mA/2 per USB spec
    .composite_with_iads()           // multi-interface composite devices
    .build();
```

## Device state

```rust
match usb_dev.state() {
    DeviceState::Default => { /* reset, no address */ }
    DeviceState::Addressed => { /* address assigned, not configured */ }
    DeviceState::Configured => { /* ready for data */ }
    DeviceState::Suspend => { /* bus suspended */ }
}

if usb_dev.is_configured() {
    // ready
}
```

## Endpoint types

Control, Bulk, Interrupt and Isochronous endpoints (full-speed).

## Errors

Fallible operations return `UsbResult<T>`:

```rust
match serial.write(data) {
    Ok(n) => { /* wrote n bytes */ }
    Err(UsbError::WouldBlock) => { /* retry later */ }
    Err(UsbError::InvalidEndpoint) => { /* endpoint not allocated */ }
    Err(e) => { /* … */ }
}
```

## VID / PID

- pid.codes: `0x1209:0x0001` (free for open-source projects)
- Your own registered VID/PID for production


**Note**: Using someone else's VID/PID is against USB specifications.

## License

MIT OR Apache-2.0
