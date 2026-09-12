//! # USB HID Power Device (UPS) example
//!
//! Presents the Pico 2 as a USB HID Power Device — a UPS with a battery. Linux
//! ships no generic driver for the class, so the device shows up as a plain HID
//! interface (`/dev/hidrawN`) and the host side is userspace: a raw read of that
//! node for the input reports, `hidapi` for the feature and output reports, or
//! NUT's `usbhid-ups` to drive it as a real UPS.
//!
//! ## Hardware
//! - Raspberry Pi Pico 2 (RP2350) and a USB cable
//! - Button on GPIO 15 (internal pull-up) — simulates a mains failure
//! - LED on GPIO 25 — lit while the load runs from the battery
//!
//! ## What it reports
//!
//! The on-chip temperature sensor is the only real measurement; voltage,
//! current and load are simulated, because the Pico has no line-voltage or
//! current sensing. The point of the example is the class API, not the
//! numbers.
//!
//! - Input report 1: present/changed status flags (`Good`, `Present`, `Used`, …)
//! - Input report 2: 230 V / 50 Hz output, load, on-chip temperature
//! - Input report 3: battery flags, state of charge, runtime to empty
//! - Feature report 4: nominal ratings, readable and writable by the host
//! - Output report 5: switch on / switch off / toggle / audible alarm
//!
//! Pressing the button switches between mains and battery. While on battery
//! the state of charge drops fast — one simulated minute per 100 ms — so a
//! discharge cycle is visible in seconds. The host can switch the load off by
//! writing output report 5, which shows up in the load and current fields.
//!
//! Note how the application keeps its own simulation state (`on_battery`,
//! `battery_pct`, …) and only feeds the power device through
//! `power.device_mut()`; the device itself queues the input reports and pushes
//! them when the endpoint is free.
//!
//! ## Reading it from Linux
//!
//! `hid-generic` binds and creates a hidraw node, which `dmesg` names:
//!
//! ```text
//! $ lsusb -d 2e8a:000d
//! Bus 001 Device 025: ID 2e8a:000d Raspberry Pi Pico HID UPS
//! $ sudo dmesg | grep -i "Pico HID UPS"
//! hid-generic 0003:2E8A:000D.0001: hiddev5,hidraw11: USB HID v1.11 Device [Raspberry Pi Pico HID UPS] ...
//! ```
//!
//! The node is root-only, so either read it with `sudo` or add a udev rule for
//! the group you are in:
//!
//! ```text
//! # /etc/udev/rules.d/99-pico-ups.rules
//! KERNEL=="hidraw*", ATTRS{idVendor}=="2e8a", ATTRS{idProduct}=="000d", MODE="0660", GROUP="plugdev"
//! ```
//!
//! Reading the node streams the input reports, each prefixed with its report
//! ID, with the application's values shown for reference:
//!
//! ```text
//! 01 <present:3> <changed:3>                          status flags, then the ones that changed
//! 02 <voltage:2> <current:2> <Hz:1> <load%:1> <K:2>    23000 cV, 1.50 A, 50 Hz, 35 %, ~300 K
//! 03 <flags:1> <changed:1> <charge%:1> <runtime:2>    battery flags, charge, minutes to empty
//! ```
//!
//! (The 16-bit fields are little-endian: `23000` cV is `d8 59`, so a full
//! report 2 on mains with the load on is `02 d8 59 96 00 32 23 2c 01`.) Feature
//! reports need the `HIDIOCGFEATURE` / `HIDIOCSFEATURE` ioctls rather than
//! plain read/write, so reach for a library that has them — `hidapi`, from
//! `sudo apt install python3-hid`:
//!
//! ```python
//! import hid
//! d = hid.device()
//! d.open(0x2e8a, 0x000d)
//! print(bytes(d.get_feature_report(4, 14)).hex(" "))   # feature report 4, 13 bytes
//! d.write(b"\x05\x02")                                  # output report 5: switch the load off
//! ```
//!
//! NUT (`sudo apt install nut`, `MODE=standalone` in `/etc/nut/nut.conf`) drives
//! the same reports through its `usbhid-ups` driver, with the device described in
//! `/etc/nut/ups.conf`:
//!
//! ```text
//! [pico]
//!     driver = usbhid-ups
//!     port = auto
//!     vendorid = 2e8a
//!     productid = 000d
//! ```
//!
//! `sudo usbhid-ups -DD -a pico -x explore` shows whether it recognises the
//! device and which usages it maps, and `upsc pico` reports the result as a
//! normal UPS (`battery.charge`, `ups.load`, `ups.status`). NUT claims the
//! interface through libusb, so hidraw stops delivering reports while it runs.

#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;

use rp2350::adc::Adc;
use rp2350::clocks::ClockBuilder;
use rp2350::gpio::Pins;
use rp2350::main;
use rp2350::pac::Peripherals;

use rp2350_usb::UsbBusAllocator;
use rp2350_usb::class::hid::HidClass;
use rp2350_usb::class::hid::power::{
    BatteryState, BatteryStatus, PowerConfig, PowerControls, PowerDevice, PowerMeasures,
    PowerStatus,
};
use rp2350_usb::device::UsbDeviceBuilder;
use rp2350_usb::rp2350::Rp2350Bus;

/// Output voltage while running on mains, in centivolts.
const MAINS_VOLTAGE_CV: u16 = 23_000;
/// Output voltage while running on the inverter, in centivolts.
const INVERTER_VOLTAGE_CV: u16 = 22_800;
/// Output current with the load connected, in centiamps.
const LOAD_CURRENT_CA: u16 = 150;
/// Output current with the load switched off, in centiamps.
const IDLE_CURRENT_CA: u16 = 20;

/// The power device, wrapped in the HID class that owns the USB endpoints.
type Ups<'a> = HidClass<'a, Rp2350Bus, PowerDevice>;

#[main]
fn main() -> ! {
    info!("USB HID Power Device (UPS) example starting...");

    let dp = Peripherals::take().unwrap();

    let clocks = ClockBuilder::new()
        .enable_usb()
        .enable_adc()
        .build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);
    info!(
        "Clocks: sys={} Hz, usb={} Hz",
        clocks.sys_freq(),
        clocks.usb_freq()
    );

    let mut adc = Adc::init(dp.ADC);

    let pins = Pins::init(dp.IO_BANK0, dp.PADS_BANK0);

    let mut led = pins.pin::<25>();
    led.into_output();
    led.set_low();

    let mut button = pins.pin::<15>();
    button.into_input_pullup();

    // The class allocates the interface and the two interrupt endpoints; the
    // device supplies the report descriptor and the request/response sizes.
    let alloc = UsbBusAllocator::new(unsafe { Rp2350Bus::steal() });
    let mut power: Ups = HidClass::new(&alloc, PowerDevice::ups());
    info!(
        "HID power device created (report descriptor {} bytes)",
        power.report_descriptor().len()
    );

    let mut usb_dev = UsbDeviceBuilder::new(&alloc, 0x2E8A, 0x000D)
        .device_release(0x0100)
        .manufacturer("Raspberry Pi")
        .product("Pico HID UPS")
        .serial_number("UPS001")
        .max_power_ma(100)
        .build();

    // Nominal ratings, published through feature report 4.
    power.device_mut().set_config(PowerConfig {
        voltage_cv: MAINS_VOLTAGE_CV,
        current_ca: 650, // 6.5 A ≈ 1500 VA
        frequency_hz: 50,
        low_voltage_transfer_cv: 19_000, // transfer to battery below 190 V
        high_voltage_transfer_cv: 25_000, // transfer back to mains above 250 V
        delay_before_shutdown_s: 30,
        temperature_k: 320,
    });

    usb_dev.bus().connect();
    info!("USB connected, waiting for enumeration...");

    let delay = rp2350_examples::common::board::init_delay(&clocks);

    // Simulation state.
    let mut on_battery = false;
    let mut load_on = true;
    let mut battery_pct: u8 = 100;
    let mut temperature_k: u16 = 300;
    let mut present = PowerStatus::NONE;
    let mut battery_status = BatteryStatus::NONE;
    let mut button_prev = true;
    let mut announced = false;
    let mut tick: u32 = 0;

    loop {
        usb_dev.poll(&mut [&mut power]);

        if !announced && power.is_configured() {
            announced = true;
            info!("Host configured the power device, reports are live");
        }

        // Button: mains failure / mains restored.
        let button_now = button.get();
        if button_prev && !button_now {
            on_battery = !on_battery;
            if on_battery {
                led.set_high();
                info!("Mains failed, running from battery");
            } else {
                led.set_low();
                info!("Mains restored, charging battery");
            }
        }
        button_prev = button_now;

        // Configuration written by the host.
        if let Some(config) = power.device_mut().take_config_write() {
            info!(
                "Host configuration: {} cV, {} cA, {} Hz, transfer {}..{} cV, delay {} s",
                config.voltage_cv,
                config.current_ca,
                config.frequency_hz,
                config.low_voltage_transfer_cv,
                config.high_voltage_transfer_cv,
                config.delay_before_shutdown_s
            );
            // Publish it back so a host read returns what was applied.
            power.device_mut().set_config(config);
        }

        // Switch commands written to output report 5.
        if let Some(controls) = power.device_mut().take_controls() {
            if controls.contains(PowerControls::SWITCH_OFF) {
                load_on = false;
                info!("Host: load switched off");
            }
            if controls.contains(PowerControls::SWITCH_ON) {
                load_on = true;
                info!("Host: load switched on");
            }
            if controls.contains(PowerControls::TOGGLE) {
                load_on = !load_on;
                info!("Host: load toggled, now {}", load_on);
            }
            if controls.contains(PowerControls::AUDIBLE_ALARM) {
                info!("Host: audible alarm");
            }
        }

        // The loop runs every ~2 ms; update the device every ~100 ms so the
        // host sees a fresh input report without flooding the bus.
        tick += 1;
        if tick.is_multiple_of(50) {
            if on_battery {
                battery_pct = battery_pct.saturating_sub(1);
                if battery_pct == 0 {
                    warn!("Battery empty, shutdown imminent");
                }
            } else {
                battery_pct = (battery_pct + 1).min(100);
            }

            // Sample the on-chip temperature once a second.
            if tick.is_multiple_of(500) {
                let celsius = Adc::convert_temp_to_celsius(adc.read_temp());
                temperature_k = if celsius < 0.0 {
                    273
                } else {
                    celsius as u16 + 273
                };
            }

            let next_present = PowerStatus::PRESENT
                | PowerStatus::GOOD
                | PowerStatus::USED
                | PowerStatus::SWITCHABLE
                | PowerStatus::INITIALIZED
                | PowerStatus::TESTED
                | if load_on {
                    PowerStatus::SWITCH_ON_OFF
                } else {
                    PowerStatus::NONE
                }
                | if battery_pct == 0 {
                    PowerStatus::SHUTDOWN_IMMINENT
                } else {
                    PowerStatus::NONE
                };

            let next_battery = if on_battery {
                BatteryStatus::DISCHARGING
                    | BatteryStatus::BATTERY_PRESENT
                    | if battery_pct < 20 {
                        BatteryStatus::BELOW_REMAINING_CAPACITY_LIMIT
                    } else {
                        BatteryStatus::NONE
                    }
            } else if battery_pct >= 100 {
                BatteryStatus::FULLY_CHARGED
                    | BatteryStatus::AC_PRESENT
                    | BatteryStatus::BATTERY_PRESENT
            } else {
                BatteryStatus::CHARGING | BatteryStatus::AC_PRESENT | BatteryStatus::BATTERY_PRESENT
            };

            // `diff` against the previous value reports the change once, and
            // the follow-up report clears the ChangedStatus flags again.
            let ups = power.device_mut();
            ups.set_status(next_present, next_present.diff(present));
            present = next_present;

            ups.set_battery(BatteryState {
                status: next_battery,
                changed: next_battery.diff(battery_status),
                state_of_charge_pct: battery_pct,
                run_time_to_empty_min: u16::from(battery_pct) * 3,
            });
            battery_status = next_battery;

            ups.set_measures(PowerMeasures {
                voltage_cv: if on_battery {
                    INVERTER_VOLTAGE_CV
                } else {
                    MAINS_VOLTAGE_CV
                },
                current_ca: if load_on {
                    LOAD_CURRENT_CA
                } else {
                    IDLE_CURRENT_CA
                },
                frequency_hz: 50,
                percent_load: if load_on { 35 } else { 5 },
                temperature_k,
            });

            // `poll` would push these on the next iteration; this is one loop
            // iteration earlier.
            power.flush();
        }

        delay.delay_us(2_000);
    }
}
