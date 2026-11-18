//! # Reboot Example
//!
//! Demonstrates [`rp2350::reboot`], which resets the chip through the bootrom's
//! watchdog facility while choosing the boot path (normal, BOOTSEL, RAM image,
//! flash update or PC/SP).
//!
//! ## Behavior
//!
//! 1. Logs the plan over defmt/RTT
//! 2. Waits 3 seconds
//! 3. Reboots into BOOTSEL mode — the equivalent of the Pico SDK's
//!    `reset_usb_boot(0, 0)`
//!
//! ## Notes
//!
//! The chip leaves the application during the reboot, so `cargo run` (probe-rs)
//! ends with a lost-connection error; that is expected. The board then
//! enumerates as a USB boot device (`0x2e8a:0x0003`) and can be flashed again
//! with `picotool` or by copying a UF2 file to the mass-storage drive.

#![no_std]
#![no_main]

#[cfg(feature = "defmt")]
use defmt_rtt as _;
use rp2350_examples::log::*;

use rp2350::Delay;
use rp2350::main;
use rp2350::reboot::{self, RebootKind};
use rp2350::rt::peripheral::Peripherals as CorePeripherals;

// Binary info declarations
rp2350::bi_program_name!("Reboot Example");
rp2350::bi_program_version!("1.0.0");
rp2350::bi_program_description!("Demonstrates bootrom-based reboots");
rp2350::bi_rp2350_board!("rp23502");
rp2350::bi_program_feature!("Reboot");

const CLOCK_FREQ_HZ: u32 = 12_000_000;

#[main]
fn main() -> ! {
    let cp = CorePeripherals::take().unwrap();
    let delay = Delay::init(cp.SYST, CLOCK_FREQ_HZ);

    info!("Rebooting into BOOTSEL in 3 s...");
    delay.delay_ms(3000);

    // BOOTSEL with both USB interfaces (picoboot + mass storage) enabled.
    info!("Rebooting now");
    reboot::reboot(RebootKind::BootSel {
        picoboot_disabled: false,
        msd_disabled: false,
    });
}
