//! # USB CDC-ECM Ethernet Example
//!
//! This example demonstrates USB CDC-ECM (Ethernet Control Model) functionality.
//! Connect the Pico 2 via USB and it will appear as a network interface.
//!
//! ## Hardware
//! - Raspberry Pi Pico 2 (RP2350)
//! - USB cable connected to the Pico's USB port
//!
//! ## Testing
//! 1. Flash this example to the Pico 2
//! 2. Connect USB cable
//! 3. The device should appear as a USB Ethernet adapter
//! 4. Configure the host's network interface:
//!    - Linux: `sudo ip addr add 192.168.7.2/24 dev usb0 && sudo ip link set usb0 up`
//!    - macOS: System Preferences -> Network -> Configure manually
//! 5. Ping the device: `ping 192.168.7.1`
//! 6. Send UDP: `echo "Hello" | nc -u 192.168.7.1 5000`
//!
//! ## Notes
//! - MAC address: 02:00:00:00:00:01 (locally administered)
//! - IP address: 192.168.7.1 (device side)
//! - Expected host IP: 192.168.7.2

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
use pico_rs_usb::class::cdc_ecm::CdcEcm;
use pico_rs_usb::device::UsbDeviceBuilder;
use pico_rs_usb::net::{Ipv4Addr, MacAddr, NetInterface};

/// RP2350 Boot Block
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::secure_exe();

/// Target system clock frequency (150 MHz)
const SYS_CLK_HZ: u32 = 150_000_000;

/// Device MAC address (locally administered)
const MAC_ADDRESS: [u8; 6] = [0x02, 0x00, 0x00, 0x00, 0x00, 0x01];

/// Device IP address
const DEVICE_IP: [u8; 4] = [192, 168, 7, 1];

/// UDP echo port
const UDP_ECHO_PORT: u16 = 5000;

#[entry]
fn main() -> ! {
    info!("USB CDC-ECM Ethernet Example starting...");

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

    // Create CDC-ECM class
    let mut ecm = CdcEcm::new(&usb_bus, MAC_ADDRESS);
    info!(
        "CDC-ECM class created with MAC: {:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
        MAC_ADDRESS[0],
        MAC_ADDRESS[1],
        MAC_ADDRESS[2],
        MAC_ADDRESS[3],
        MAC_ADDRESS[4],
        MAC_ADDRESS[5]
    );

    // Create network interface
    let mac = MacAddr::new(MAC_ADDRESS);
    let ip = Ipv4Addr::new(DEVICE_IP[0], DEVICE_IP[1], DEVICE_IP[2], DEVICE_IP[3]);
    let mut net_iface = NetInterface::new(mac, ip);

    // Register UDP port for echo service
    net_iface.register_udp_port(UDP_ECHO_PORT);
    info!("UDP echo service listening on port {}", UDP_ECHO_PORT);

    // Build USB device
    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus)
        .vendor_id(0x2E8A) // Raspberry Pi
        .product_id(0x000B) // ECM device
        .device_release(0x0100)
        .device_class(0x02) // Communications
        .device_subclass(0x00)
        .device_protocol(0x00)
        .manufacturer("Raspberry Pi")
        .product("Pico USB Ethernet")
        .serial_number("ECM001")
        .build();

    info!("USB device built, connecting...");

    // Connect to host
    usb_bus.connect();
    info!("USB connected, waiting for enumeration...");

    let mut delay = Delay::new(SYS_CLK_HZ);
    let mut rx_buf = [0u8; 1514];

    loop {
        // Poll USB
        usb_dev.poll(&mut [&mut ecm]);

        // Check if device is ready (alternate setting 1 selected)
        if ecm.is_ready() {
            // Try to receive Ethernet frame
            match ecm.receive(&mut rx_buf) {
                Ok(len) if len > 0 => {
                    debug!("Received {} bytes", len);

                    // First check for UDP packets (for echo service)
                    if let Some((src_ip, src_port, dst_port, payload)) =
                        net_iface.process_frame_udp(&rx_buf[..len])
                    {
                        info!(
                            "UDP packet from {}:{} to port {}, {} bytes",
                            src_ip,
                            src_port,
                            dst_port,
                            payload.len()
                        );

                        // Echo the data back
                        if dst_port == UDP_ECHO_PORT {
                            if let Ok(Some(response)) =
                                net_iface.send_udp(src_ip, UDP_ECHO_PORT, src_port, payload)
                            {
                                info!("Sending UDP echo response");
                                let _ = ecm.transmit(response);
                            }
                        }
                    }
                    // Process other frames (ARP, ICMP)
                    else if let Some(response) = net_iface.process_frame(&rx_buf[..len]) {
                        debug!("Sending response ({} bytes)", response.len());
                        let _ = ecm.transmit(response);
                    }
                }
                Ok(_) => {}
                Err(pico_rs_usb::error::UsbError::WouldBlock) => {}
                Err(e) => {
                    warn!("Receive error: {:?}", e);
                }
            }
        }

        // Small delay to prevent busy loop
        delay.delay_us(10);
    }
}
