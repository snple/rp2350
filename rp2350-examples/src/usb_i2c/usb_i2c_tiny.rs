//! # i2c-tiny-usb example
//!
//! Presents the Pico 2 as an i2c-tiny-usb adapter.  The Linux `i2c-tiny-usb`
//! driver binds to the VID/PID pair the project was donated (0x0403:0xC631) and
//! the I2C bus appears as a regular `/dev/i2c-N` adapter, usable with
//! `i2cdetect`, `i2cget`, `i2cset` and `i2cdump`.
//!
//! ## Hardware
//! - Raspberry Pi Pico 2 (RP2350) and a USB cable
//! - I2C0 on GP4 (SDA) and GP5 (SCL), with 4.7 kΩ pull-ups to 3V3
//! - LED on GPIO 25 — lit while the host is using the adapter
//!
//! ## Testing on Linux
//!
//! ```text
//! $ sudo modprobe i2c-tiny-usb
//! $ sudo dmesg | tail            # "i2c-tiny-usb ... version 1.00 found"
//! $ i2cdetect -l                 # the adapter shows up as i2c-N
//! $ i2cdetect -y N               # scan the bus
//! $ i2cget -y N 0x50 0x00 w      # register read (write + repeated start + read)
//! $ i2cset -y N 0x50 0x00 0x12   # register write
//! ```
//!
//! The adapter reports its activity once a second: `i2cdetect` shows up as a
//! burst of transfers, most of them failing with a NAK because nothing answers
//! at those addresses.
//!
//! ## Watching host traffic
//!
//! Every bus operation is echoed to the console, so a write coming from the host
//! is visible on the firmware side:
//!
//! ```text
//! $ sudo i2cset -y 14 0x50 0x00 0x48 0x45 0x4c 0x4c 0x4f i
//! INFO  i2c write 0x50: [00, 05, 48, 45, 4c, 4c, 4f] | b"\x00\x05HELLO" | Err(NoAcknowledge)
//! ```
//!
//! The payload is printed before the transfer result, so it is logged even when
//! the write fails — with nothing at that address the transfer NAKs, which the
//! `Err(NoAcknowledge)` above shows. Address probes are logged at `debug!`
//! instead of `info!`, because a scan issues one per address.
//!
//! ## Greeting
//!
//! Scanning an empty bus draws a picture. The scan output is a grid of sixteen
//! addresses per row, and every cell that answers shows up as one pixel, so the
//! adapter reports a device on a handful of addresses that are quiet anyway:
//!
//! ```text
//!      0  1  2  3  4  5  6  7  8  9  a  b  c  d  e  f
//! 00:                         -- -- -- -- -- -- -- --
//! 10: -- 11 -- -- -- -- -- -- -- 19 -- 1b -- -- -- --
//! 20: -- 21 -- -- -- -- 26 27 -- 29 -- 2b -- -- 2e 2f
//! 30: -- 31 -- -- -- 35 36 37 -- 39 -- 3b -- 3d -- 3f
//! 40: -- 41 42 -- -- 45 -- -- -- 49 -- 4b -- 4d -- 4f
//! 50: -- 51 -- 53 -- 55 -- -- -- 59 -- 5b -- 5d -- 5f
//! 60: -- 61 -- 63 -- -- 66 67 -- 69 -- 6b -- -- 6e 6f
//! 70: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --
//! ```
//!
//! Only addresses that do not answer for real are faked, so a device sitting on
//! one of them still shows itself and the greeting never covers a real chip. The
//! picture is [`GREETING`]; empty that list to turn it off.
//!
//! ## What the example implements
//!
//! [`HalI2c`] wraps the HAL's `I2c0` for the [`I2cBus`] trait the class expects,
//! and turns the bit delay the host requests into a baud rate: 10 µs is the
//! driver's default, i.e. 100 kHz.

#![no_std]
#![no_main]

#[cfg(feature = "defmt")]
use defmt_rtt as _;
use rp2350_examples::log::*;

use rp2350::clocks::ClockBuilder;
use rp2350::gpio::{Pin, Pins};
use rp2350::i2c::{Error as HalError, I2c0};
use rp2350::main;
use rp2350::pac::Peripherals;

use rp2350_usb::UsbBusAllocator;
use rp2350_usb::class::i2c_tiny_usb::{I2cBus, I2cError, I2cTinyUsb};
use rp2350_usb::device::UsbDeviceBuilder;
use rp2350_usb::rp2350::Rp2350Bus;

/// Bus speed before the host asks for something else (the driver's default).
const I2C_BAUDRATE: u32 = 100_000;

/// A greeting for anyone who scans the bus.
///
/// The scan output is a grid of sixteen addresses per row, one row per high
/// nibble, and every cell that answers shows up as a pixel. So the picture is
/// one line per grid row here and one character per address: line 0 is row
/// `0x10`, character 0 is address `0x11`, and a `#` means "answer here even
/// though there is nothing on the bus". Empty the list to turn it off.
const GREETING: &[&str] = &[
    "...............",
    "#.#.###.#.#..#.",
    "#.#.#...#.#.#.#",
    "###.###.#.#.#.#",
    "#.#.#...#.#.#.#",
    "#.#.###.#.#..#.",
];

/// Grid row the picture starts on.
const GREETING_ROW: usize = 0x1;

/// Whether the greeting draws a pixel at `address`.
fn greeting_pixel(address: u8) -> bool {
    let row = (address >> 4) as usize;
    let col = (address & 0x0f) as usize;

    // Column 0 of the grid is the margin left of the picture.
    if row < GREETING_ROW || col == 0 {
        return false;
    }

    GREETING
        .get(row - GREETING_ROW)
        .and_then(|line| line.as_bytes().get(col - 1))
        == Some(&b'#')
}

/// I2C0 wrapped for the adapter.
struct HalI2c {
    i2c: I2c0<Pin<4>, Pin<5>>,
    sys_freq: u32,
}

impl HalI2c {
    /// Translate a HAL error into the two cases the protocol can report.
    fn map(error: HalError) -> I2cError {
        match error {
            HalError::NoAcknowledge => I2cError::NoAcknowledge,
            _ => I2cError::BusFault,
        }
    }

    /// Whether the greeting answers for `address` because the bus did not.
    ///
    /// A device that does answer always wins, so the greeting only ever fills in
    /// cells a scan would have shown as `--`.
    fn greeted(address: u8, real: Result<(), HalError>) -> bool {
        real.is_err() && greeting_pixel(address)
    }
}

impl I2cBus for HalI2c {
    fn write(&mut self, address: u8, data: &[u8]) -> Result<(), I2cError> {
        let result = self.i2c.write(address, data);

        if data.is_empty() {
            // An empty slice is the address probe bus scanning uses, and
            // `i2cdetect` does one per address, so it stays at debug level to
            // keep real traffic readable. The log still shows what the bus did,
            // with a note when the greeting answers in its place.
            let greeted = Self::greeted(address, result);
            debug!(
                "i2c probe 0x{:02x}: {}{}",
                address,
                result,
                if greeted { " (greeting)" } else { "" }
            );

            let answer = if greeted { Ok(()) } else { result };
            return answer.map_err(Self::map);
        }

        // Echo the payload: a host write of "HELLO" shows up here as the hex
        // bytes and as text.
        info!(
            "i2c write 0x{:02x}: {:02x} | {:a} | {}",
            address, data, data, result
        );

        result.map_err(Self::map)
    }

    fn read(&mut self, address: u8, data: &mut [u8]) -> Result<(), I2cError> {
        let real = self.i2c.read(address, data);
        let greeted = Self::greeted(address, real);

        if greeted {
            // Nothing answered, so hand back zeros rather than whatever the
            // buffer held before.
            data.fill(0);
        }

        match real {
            // The buffer only holds something when the target answered.
            Ok(()) => info!(
                "i2c read 0x{:02x}: {:02x} | {:a}",
                address,
                &data[..],
                &data[..]
            ),
            Err(error) => info!(
                "i2c read 0x{:02x}: {}{}",
                address,
                error,
                if greeted { " (greeting)" } else { "" }
            ),
        }

        let answer = if greeted { Ok(()) } else { real };
        answer.map_err(Self::map)
    }

    fn write_read(&mut self, address: u8, data: &[u8], buffer: &mut [u8]) -> Result<(), I2cError> {
        let result = self.i2c.write_read(address, data, buffer);
        match result {
            Ok(()) => info!(
                "i2c write-read 0x{:02x}: {:02x} -> {:02x} | {:a}",
                address,
                data,
                &buffer[..],
                &buffer[..]
            ),
            Err(error) => info!(
                "i2c write-read 0x{:02x}: {:02x} -> {}",
                address, data, error
            ),
        }
        result.map_err(Self::map)
    }

    fn set_delay(&mut self, microseconds: u16) {
        // The host sends a bit delay; the peripheral wants a baud rate.
        let baudrate = 1_000_000 / u32::from(microseconds.max(1));
        self.i2c.set_baudrate(baudrate, self.sys_freq);
        info!("Bit delay {} us -> {} Hz", microseconds, baudrate);
    }
}

#[main]
fn main() -> ! {
    info!("i2c-tiny-usb example starting...");

    let dp = Peripherals::take().unwrap();

    let clocks = ClockBuilder::new()
        .enable_usb()
        .build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);

    let pins = Pins::init(dp.IO_BANK0, dp.PADS_BANK0);

    let mut led = pins.pin::<25>();
    led.into_output();
    led.set_low();

    let i2c = I2c0::init(
        dp.I2C0,
        pins.pin::<4>(), // SDA
        pins.pin::<5>(), // SCL
        I2C_BAUDRATE,
        &clocks,
    );
    info!("I2C0 on GP4/GP5 at {} Hz", I2C_BAUDRATE);

    let bus = HalI2c {
        i2c,
        sys_freq: clocks.sys_freq(),
    };

    let alloc = UsbBusAllocator::new(unsafe { Rp2350Bus::steal() });
    let mut adapter = I2cTinyUsb::new(&alloc, bus);

    // The VID/PID pair the i2c-tiny-usb project was donated: the Linux driver
    // matches exactly this pair.
    let mut usb_dev = UsbDeviceBuilder::new(&alloc, 0x0403, 0xC631)
        .device_release(0x0100)
        .manufacturer("hello")
        .product("i2c-tiny-usb")
        .serial_number("RP2350")
        .build();

    usb_dev.bus().connect();
    info!("USB connected, waiting for the i2c-tiny-usb driver...");

    let delay = rp2350_examples::common::board::init_delay(&clocks);
    let mut tick: u32 = 0;
    let mut transfers = 0u32;
    let mut idle_ticks: u32 = 0;

    loop {
        usb_dev.poll(&mut [&mut adapter]);
        delay.delay_us(2_000);

        // The host only touches the adapter when it talks to the bus, so the
        // transfer count only moves when something happened.  Report it once a
        // second rather than on every control transfer.
        tick = tick.wrapping_add(1);
        if tick.is_multiple_of(500) {
            let current = adapter.transfers();
            if current != transfers {
                transfers = current;
                idle_ticks = 0;
                led.set_high();
                info!(
                    "{} transfers, {} failures, bit delay {} us",
                    current,
                    adapter.failures(),
                    adapter.delay_us()
                );
            } else {
                idle_ticks += 1;
                if idle_ticks == 2 {
                    // Roughly two seconds without traffic.
                    led.set_low();
                }
            }
        }
    }
}
