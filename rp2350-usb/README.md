# rp2350-usb

USB device stack for RP2350 microcontrollers (Raspberry Pi Pico 2).  It contains
a full-speed USB 2.0 device driver for the on-chip USB peripheral, the device
state machine including the standard control transfers, and a class layer with
CDC ACM, HID (keyboard, mouse, power device), GS_USB, i2c-tiny-usb and UAC 1.0.

`no_std`, no allocator, no dependencies beyond `rp2350-pac` / `rp2350-rt`.  The
`defmt` feature (on by default) adds logging.

## Layout

The MCU-independent stack lives in the shared
[`usb-stack`](../../usb-stack) crate (also used by `ch32x035-usb`) and is
re-exported here, so `rp2350_usb::bus::…`, `rp2350_usb::class::…`, … keep
working. This crate adds the RP2350 peripheral driver and the RP2350-specific
classes.

| Module | Contents |
|--------|----------|
| `bus` | `UsbBus` (hardware abstraction) and `UsbBusAllocator`, which hands out interfaces, endpoints and string indices *(shared)* |
| `rp2350` | `Rp2350Bus`, the RP2350 implementation of `UsbBus` |
| `device` | `UsbDeviceBuilder` / `UsbDevice` — device state machine, descriptor serving, control-transfer dispatch *(shared)* |
| `control` | `ControlPipe` (EP0 state machine) and the `ControlIn` / `ControlOut` handles classes answer requests with *(shared)* |
| `descriptor` | `DescriptorWriter`, used by the device and by classes to emit descriptors *(shared)* |
| `endpoint` | `Endpoint`, `EndpointAddress`, `EndpointType`, direction markers *(shared)* |
| `class` | `UsbClass` trait, CDC-ACM, HID (incl. power device), GS_USB, i2c-tiny-usb and UAC *(shared)* plus the RP2350 `reset` interface *(this crate)* |

A class allocates everything it needs from the `UsbBusAllocator` in its
constructor.  Building the device freezes the allocator and enables the
peripheral, so all classes must be created first.

## Quick start

```rust
use rp2350_usb::{UsbBusAllocator, UsbDeviceBuilder};
use rp2350_usb::class::cdc_acm::CdcAcm;
use rp2350_usb::rp2350::Rp2350Bus;

// 1. Wrap the hardware driver.
let alloc = UsbBusAllocator::new(unsafe { Rp2350Bus::steal() });

// 2. Create classes — this allocates their endpoints, interfaces and strings.
let mut serial = CdcAcm::new(&alloc);

// 3. Build the device: freezes the allocator and enables the peripheral.
let mut usb_dev = UsbDeviceBuilder::new(&alloc, 0x2E8A, 0x000A)
    .manufacturer("Raspberry Pi")
    .product("Pico USB Serial")
    .serial_number("123456")
    .composite_with_iads()
    .build();

// 4. Connect the D+ pull-up to signal presence to the host.
usb_dev.bus().connect();

loop {
    // Drives the state machine.  Call at least every 10 ms.
    usb_dev.poll(&mut [&mut serial]);

    let mut buf = [0u8; 64];
    if let Ok(n) = serial.read(&mut buf) {
        let _ = serial.write(&buf[..n]);
    }
}
```

`poll` returns `true` when a class may have data to read or write, so the body of
the loop can also be guarded with `if usb_dev.poll(&mut [&mut serial]) { … }`.

## CDC ACM

`CdcAcm::new(&alloc)` allocates two interfaces (control and data), an interrupt
IN endpoint for notifications and two bulk endpoints, and ties the interfaces
together with an IAD — build the device with `.composite_with_iads()`.

```rust
use rp2350_usb::UsbError;
use rp2350_usb::class::cdc_acm::CdcAcm;

let mut serial = CdcAcm::new(&alloc);

// Read: WouldBlock when nothing arrived, InvalidState before configuration.
let mut buf = [0u8; 64];
match serial.read(&mut buf) {
    Ok(n) => { /* n bytes received */ }
    Err(UsbError::WouldBlock) => {}
    Err(e) => { /* … */ }
}

// Write: sends up to one bulk packet, WouldBlock until the host collected it.
let _ = serial.write(b"hello\r\n");
```

Line coding (`Set Line Coding`) is decoded and stored; the host's control line
state is exposed as well:

| Method | Meaning |
|--------|---------|
| `port_open()` | configured by the host and DTR asserted |
| `dtr()`, `rts()` | raw control line state |
| `configured()` | host sent `Set Configuration` |
| `line_coding()` | `LineCoding { baud_rate, stop_bits, parity, data_bits }` as requested by the host |

A host sends `Set Line Coding` to configure an abstract UART; the values are not
applied to any hardware here, so the application has to interpret them.

## HID

`hid` is split in two halves:

- `HidClass` is the USB side.  It owns the interface, an interrupt IN endpoint
  for input reports and — when the function asks for one — an interrupt OUT
  endpoint, writes the interface/HID/endpoint descriptors, serves the report
  descriptor for `Get Descriptor (HID Report)` and answers `Get/Set Report`,
  `Get/Set Idle` and `Get/Set Protocol`.
- `HidDevice` is the HID side: the report descriptor, endpoint geometry and the
  reports the function exchanges with the host.

The class owns the function, so its state is reached through `device_mut()`.

```rust
use rp2350_usb::class::hid::HidClass;
use rp2350_usb::class::hid::keyboard::{Keyboard, KeyboardReport};

let mut kbd = HidClass::new(&alloc, Keyboard::new());

loop {
    usb_dev.poll(&mut [&mut kbd]);

    let report = KeyboardReport { keys: [0x04, 0, 0, 0, 0, 0], ..Default::default() };
    let _ = kbd.send_input(&report.to_bytes());   // WouldBlock if the host is slow

    let leds = kbd.device().leds();               // LED report the host sent
}
```

`send_input` takes the raw report — the report ID first when the descriptor uses
report IDs.  Reports that a device queues internally (see `power::PowerDevice`)
are pushed by `HidClass::flush`, which `poll` also calls.

Ready-made functions:

| Function | Reports |
|----------|---------|
| `keyboard::Keyboard` | boot keyboard: 8-byte input, 1-byte LED output |
| `mouse::Mouse` | boot mouse: 4-byte input |
| `custom::CustomHid` | caller-supplied report descriptor, host reports buffered for reading |
| `power::PowerDevice` | HID Power Device, see below |

`custom::CustomHid` covers the "my own descriptor" case:

```rust
use rp2350_usb::class::hid::{HidClass, HidSpec, custom::CustomHid};

let spec = HidSpec::new(MY_REPORT_DESCRIPTOR, 8)   // descriptor, IN endpoint size
    .out_report_size(8)                            // allocate an interrupt OUT endpoint
    .report_ids(true)                              // descriptor declares report IDs
    .poll_interval_ms(8);

let mut hid = HidClass::new(&alloc, CustomHid::new(spec));
hid.send_input(&[0x01, 0x00]);

let mut buf = [0u8; 8];
if let Some(len) = hid.device_mut().read_report(&mut buf) {
    // a report the host wrote, report ID first
}
```

Writing a new function means implementing `HidDevice`: `spec()` plus whichever
report hooks are needed (`get_report`, `set_report`, `output_report`) and
lifecycle hooks (`configure`, `deconfigure`, `reset`, `poll`).  Everything the
function does not override is a no-op, and unanswered requests are stalled by the
device layer.  `HidSpec` carries the report descriptor, the endpoint sizes and
the values that cannot be derived from the descriptor itself: subclass, protocol
and whether reports carry an ID byte.  Note that `Set Report` passes the report
ID in `wValue` while an interrupt OUT packet carries it as its first byte; the
class normalises both into `(report_id, data)`.

## HID Power Device

`power::PowerDevice` implements the *USB Usage Tables for HID Power Devices*
(Power Device page `0x84`, Battery System page `0x85`).  It is a UPS or a power
supply that reports status, measurements, battery state, configuration and
switch controls:

| Report ID | Type | Direction | Payload |
|-----------|------|-----------|---------|
| 1 | Input | device → host | `PowerStatus` present + changed flags |
| 2 | Input | device → host | `PowerMeasures` voltage, current, frequency, load, temperature |
| 3 | Input | device → host | `BatteryState` battery flags, state of charge, runtime |
| 4 | Feature | both | `PowerConfig` nominal values and transfer thresholds |
| 5 | Output | host → device | `PowerControls` switch on / off / toggle / audible alarm |

The application owns the measurements and pushes them; the device re-sends an
input report only when its contents changed, and retries reports the endpoint
could not take yet.

```rust
use rp2350_usb::class::hid::HidClass;
use rp2350_usb::class::hid::power::{
    BatteryState, BatteryStatus, PowerConfig, PowerControls, PowerDevice, PowerMeasures,
    PowerStatus,
};

let mut power = HidClass::new(&alloc, PowerDevice::ups());   // or ::power_supply()

power.device_mut().set_config(PowerConfig {
    voltage_cv: 23_000,
    current_ca: 650,
    frequency_hz: 50,
    low_voltage_transfer_cv: 19_000,
    high_voltage_transfer_cv: 25_000,
    delay_before_shutdown_s: 30,
    temperature_k: 320,
});

loop {
    usb_dev.poll(&mut [&mut power]);

    power.device_mut().set_status(
        PowerStatus::PRESENT | PowerStatus::GOOD | PowerStatus::USED,
        PowerStatus::NONE,
    );
    power.device_mut().set_measures(PowerMeasures {
        voltage_cv: 23_000,
        current_ca: 150,
        frequency_hz: 50,
        percent_load: 35,
        temperature_k: 300,
    });
    power.device_mut().set_battery(BatteryState {
        status: BatteryStatus::AC_PRESENT | BatteryStatus::BATTERY_PRESENT,
        ..Default::default()
    });

    if let Some(controls) = power.device_mut().take_controls() {
        // host wrote output report 5
    }
    if let Some(config) = power.device_mut().take_config_write() {
        // host wrote feature report 4; apply it, then publish it with set_config
    }
}
```

`set_status` takes the changed flags as well — `PowerStatus::diff` computes them
against the previous value, which reports each change once and clears the flags
again on the following report.  Values follow the specification's unit table
(centivolts, centiamps, hertz, percent, kelvin, minutes for runtime) and are
clamped to the logical range declared in the descriptor.  A feature read returns
what `set_config` published, i.e. what the device actually applied, not what the
host asked for.

## GS_USB

`GsUsbClass::new(&alloc)` is a vendor-specific (class `0xFF`) CAN adapter
speaking the candleLight protocol, so the Linux `gs_usb` driver binds to it
directly.

```rust
use rp2350_usb::class::gs_usb::{GsDeviceConfig, GsHostFrame, GsUsbClass};

let mut can = GsUsbClass::new(&alloc);
can.set_device_config(GsDeviceConfig { icount: 1, ..Default::default() });

loop {
    usb_dev.poll(&mut [&mut can]);

    match can.try_recv() {
        Ok(Some(frame)) => { /* host → CAN: transmit frame */ }
        Ok(None) => {}
        Err(_) => {}
    }

    let frame = GsHostFrame { can_id: 0x123, can_dlc: 8, ..Default::default() };
    let _ = can.try_send(&frame);          // CAN → host
}
```

The host's CAN setup arrives through control requests and is readable as
`can.mode()`, `can.mode_flags()`, `can.bit_timing()`; `set_bt_const` advertises
what the firmware supports.  The Linux driver binds to `0x1D50:0x606F`
(candleLight); while bringing firmware up, use another VID/PID so the driver does
not probe the device before it handles every request.

## i2c-tiny-usb

`i2c_tiny_usb::I2cTinyUsb` bridges a plain I2C master to USB, speaking the
protocol of Till Harbaum's i2c-tiny-usb adapter: the Linux `i2c-tiny-usb` driver
binds to the donated `0x0403:0xC631` pair and the bus turns up as an ordinary
`/dev/i2c-N` adapter.  Everything travels over control transfers, so the
interface declares no endpoints at all.

The bus is supplied by implementing `I2cBus`, which the class maps the host's
`i2c_msg` flags onto: one message becomes `write` or `read`, a write followed by
a read becomes `write_read` (repeated start), which is what SMBus emulation —
and therefore `i2cdetect`, `i2cget` and `i2cdump` — needs.  `Set Delay` is
forwarded as a bit delay, 10 µs being the driver's default of 100 kHz.

```rust
use rp2350_usb::class::i2c_tiny_usb::{I2cBus, I2cError, I2cTinyUsb};

impl I2cBus for HalI2c {
    fn write(&mut self, address: u8, data: &[u8]) -> Result<(), I2cError> {
        self.i2c.write(address, data).map_err(|_| I2cError::NoAcknowledge)
    }
    fn read(&mut self, address: u8, data: &mut [u8]) -> Result<(), I2cError> {
        self.i2c.read(address, data).map_err(|_| I2cError::NoAcknowledge)
    }
    fn write_read(&mut self, address: u8, data: &[u8], buffer: &mut [u8]) -> Result<(), I2cError> {
        self.i2c.write_read(address, data, buffer).map_err(|_| I2cError::NoAcknowledge)
    }
}

let mut adapter = I2cTinyUsb::new(&alloc, hal_i2c);

let mut dev = UsbDeviceBuilder::new(&alloc, 0x0403, 0xC631)
    .device_release(0x0100)
    .product("i2c-tiny-usb")
    .build();

loop {
    dev.poll(&mut [&mut adapter]);

    // Optional: the class counts what the host did.
    if adapter.failures() > 0 {
        // a device did not answer, which the host saw as -EREMOTEIO
    }
}
```

A hardware peripheral cannot pause in the middle of a transaction, so a message
without `I2C_IO_END` is buffered (up to 16 bytes) and replayed with the message
that ends it.  Reads must be the last message of a transaction, 10-bit addresses
and SMBus block reads are not supported, and those requests are stalled rather
than mistranslated — `func::DEFAULT` advertises only what works.  The class
answers vendor requests for any interface index, because the protocol carries the
I2C address in `wIndex`, so it is meant to be the only vendor-specific function in
the device.

## UAC 1.0

`AudioClassBuilder` builds a UAC 1.0 function with an input and/or an output
stream: S16le or S24le samples, discrete or continuous rates, mono or
multi-channel.  Audio uses isochronous endpoints, so the device has to be built
with `.composite_with_iads()`.

```rust
use rp2350_usb::class::uac::{AudioClassBuilder, Format, StreamConfig, TerminalType};

static RATES_48K: [u32; 1] = [48_000];

let mut audio = AudioClassBuilder::new()
    .output(
        StreamConfig::new_discrete(Format::S16le, 2, &RATES_48K, TerminalType::OutSpeaker).unwrap(),
    )
    .input(
        StreamConfig::new_discrete(Format::S16le, 1, &RATES_48K, TerminalType::InMicrophone).unwrap(),
    )
    .build(&alloc);

let mut dev = UsbDeviceBuilder::new(&alloc, 0x1209, 0x1561)
    .product("USB Audio")
    .composite_with_iads()
    .build();

loop {
    dev.poll(&mut [&mut audio]);

    // Playback: PCM from the host, one packed frame (1 ms) per call.
    let mut buf = [0u8; 192];
    if let Ok(n) = audio.read(&mut buf) {
        // write the samples to a DAC
    }

    // Capture: PCM to the host; WouldBlock until the previous packet is taken.
    let pcm = [0u8; 192];
    let _ = audio.write(&pcm);
}
```

The host only streams while the corresponding alternate setting is selected:
`input_active()` / `output_active()` report that, `input_alt_setting()` /
`output_alt_setting()` return the current setting, and `is_configured()` reports
whether the device is configured at all.

## Device builder

```rust
let mut usb_dev = UsbDeviceBuilder::new(&alloc, 0x1234, 0x5678)   // VID, PID
    .device_release(0x0100)          // bcdDevice
    .usb_rev(0x0200)                 // bcdUSB, default USB 2.0
    .device_class(0)                 // 0 = class defined at interface level
    .max_packet_size_0(64)           // EP0 packet size: 8, 16, 32 or 64
    .manufacturer("Company")
    .product("Product")
    .serial_number("SN001")
    .self_powered(false)
    .supports_remote_wakeup(false)
    .max_power_ma(100)               // stored as mA/2 per USB spec, capped at 500 mA
    .composite_with_iads()           // device class 0xEF, subclass 0x02, protocol 0x01
    .build();
```

String descriptors are only served when the corresponding string was set;
`composite_with_iads()` is needed for classes that span several interfaces
(CDC ACM, UAC) and harmless otherwise.

## Device state

```rust
match usb_dev.state() {
    DeviceState::Default   => { /* reset, no address */ }
    DeviceState::Addressed => { /* address assigned, not configured */ }
    DeviceState::Configured => { /* ready for data */ }
    DeviceState::Suspend   => { /* bus suspended */ }
}
```

`is_configured()` is the shorthand for the `Configured` state,
`remote_wakeup_enabled()` reports whether the host armed remote wake-up, and
`force_reset()` disconnects and re-enumerates on buses that support it.
Suspend and resume are handled by `poll`; classes see configure/deconfigure and
reset through their `UsbClass` hooks.

## Endpoints

```rust
use rp2350_usb::{In, Out};

let iface = alloc.interface();                    // allocate an interface number
let str_idx = alloc.string();                     // allocate a string index
let ep_in = alloc.interrupt::<In>(16, 10);        // 16-byte interrupt IN, 10 ms poll
let ep_out = alloc.bulk::<Out>(64);               // 64-byte bulk OUT
```

`control`, `bulk`, `interrupt` and `isochronous` are generic over the direction
(`In` / `Out`); `interrupt` and `isochronous` also take the polling interval and
the isochronous synchronisation/usage attributes (`EndpointType`,
`IsoSynchronization`, `IsoUsage`).  Limits: 16 endpoints, 64-byte packets for
control and bulk/interrupt, 4 KiB of DPRAM, which the driver allocates in
64-byte chunks.  Filling DPRAM or running out of endpoints fails the allocation
with `EndpointMemoryOverflow` / `EndpointOverflow`.

## Errors

Fallible operations return `UsbResult<T>`:

| Variant | Meaning |
|---------|---------|
| `WouldBlock` | No data available, or the previous packet has not been collected yet — retry later |
| `InvalidState` | Called before the host configured the device, or in another invalid state |
| `InvalidEndpoint` | The endpoint address was not allocated |
| `EndpointOverflow` | No free endpoint of the requested type |
| `EndpointMemoryOverflow` | DPRAM exhausted |
| `BufferOverflow` | Buffer too small for the data |
| `ParseError` | A SETUP packet or descriptor could not be parsed |
| `Unsupported` | The bus does not implement the operation |

## Features

| Feature | Effect |
|---------|--------|
| `defmt` (default) | `defmt::Format` impls and debug logging through `rp2350`'s `defmt` |

The chip variant (`rp2350a` / `rp2350b`) is selected on the `rp2350` crate; see
the workspace root.

## Examples

The `rp2350-examples` package has one binary per example.  Each has a `.cargo`
configuration with the `probe-rs` runner, so from `rp2350-examples/`:

```console
$ cargo run --release --bin usb_hid_power
```

| Binary | Shows |
|--------|-------|
| `usb_cdc_acm` | CDC serial port that echoes received bytes and prints a line every second |
| `usb_cdc_acm_interrupt` | The same, driven from `USBCTRL_IRQ` with the main loop parked in `wfi` |
| `usb_hid_keyboard` | HID boot keyboard that types "Hello!" on a button press |
| `usb_hid_power` | HID Power Device (UPS) with simulated mains failure and the on-chip temperature sensor |
| `usb_gs_can` | GS_USB CAN adapter that echoes received frames back, with stubs where a CAN driver goes |
| `usb_i2c_tiny` | i2c-tiny-usb adapter on I2C0 (GP4/GP5), usable with `i2cdetect` and friends |
| `usb_audio_speaker` | UAC 1.0 speaker receiving 48 kHz stereo PCM |

## VID / PID

- pid.codes: `0x1209:0x0001` (free for open-source projects)
- Your own registered VID/PID for production

Using someone else's VID/PID is against USB specifications.

## License

MIT OR Apache-2.0
