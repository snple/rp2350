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
use pico_rs::clocks::{ClkPeriAuxSrc, ClkRefSrc, ClkSysAuxSrc, ClkUsbAuxSrc, Clocks};
use pico_rs::entry;
use pico_rs::pac;
use pico_rs::pll::{PllConfig, PllSys, PllUsb};
use pico_rs::reset::{self, Peripheral};
use pico_rs::systick::Delay;
use pico_rs::xosc::Xosc;

use pico_rs_usb::bus::UsbBus;
use pico_rs_usb::class::cdc::CdcAcm;
use pico_rs_usb::device::UsbDeviceBuilder;

/// RP2350 Boot Block
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::secure_exe();

/// XOSC frequency (Pico 2 uses 12 MHz crystal)
const XOSC_HZ: u32 = 12_000_000;

/// Target system clock frequency (150 MHz)
const SYS_CLK_HZ: u32 = 150_000_000;

/// USB clock frequency (must be 48 MHz)
const USB_CLK_HZ: u32 = 48_000_000;

/// Initialize clock system: XOSC -> PLL_SYS (150MHz) + PLL_USB (48MHz)
fn init_clocks() -> Clocks {
    info!("Initializing XOSC...");

    // Check XOSC status before init
    let xosc_reg = unsafe { &*pac::XOSC::PTR };
    info!("XOSC status before init: {:#010x}", xosc_reg.status.read_bits());
    info!("XOSC ctrl before init: {:#010x}", xosc_reg.ctrl.read_bits());

    let xosc = Xosc::init(XOSC_HZ);
    info!("XOSC init returned, waiting stable...");
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

#[entry]
fn main() -> ! {
    info!("USB CDC Serial Example starting...");

    // Initialize clock system first (USB requires 48MHz clock!)
    let _clocks = init_clocks();
    info!("Clock system initialized");

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

    let mut delay = Delay::new(SYS_CLK_HZ);
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
