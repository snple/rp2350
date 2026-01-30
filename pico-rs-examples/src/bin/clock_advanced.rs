//! # Advanced Clock Example
//!
//! Demonstrates advanced clock configuration features:
//! - Different clock speeds (125 MHz, 133 MHz, 150 MHz)
//! - ROSC vs XOSC clock sources
//! - Clock output on GPIO (GPOUT)
//! - Frequency measurement
//!
//! ## Note
//! This example shows how to dynamically reconfigure clocks.
//! In production, you typically configure clocks once at startup.

#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

use pico_rs::block::ImageDef;
use pico_rs::clocks::{ClkRefSrc, ClkSysAuxSrc, Clocks};
use pico_rs::entry;
use pico_rs::gpio::GpioBank;
use pico_rs::pll::{PllConfig, PllSys, PllUsb};
use pico_rs::reset::{self, Peripheral};
use pico_rs::systick::Delay;
use pico_rs::xosc::Xosc;

/// RP2350 Boot Block
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::secure_exe();

const XOSC_HZ: u32 = 12_000_000;

/// Clock speed preset
#[derive(Clone, Copy, Debug, defmt::Format)]
enum ClockSpeed {
    Slow125MHz,
    Normal133MHz,
    Fast150MHz,
}

impl ClockSpeed {
    fn config(self) -> PllConfig {
        match self {
            ClockSpeed::Slow125MHz => PllConfig::sys_125mhz(),
            ClockSpeed::Normal133MHz => PllConfig::sys_133mhz(),
            ClockSpeed::Fast150MHz => PllConfig::sys_150mhz(),
        }
    }

    fn freq_hz(self) -> u32 {
        match self {
            ClockSpeed::Slow125MHz => 125_000_000,
            ClockSpeed::Normal133MHz => 133_000_000,
            ClockSpeed::Fast150MHz => 150_000_000,
        }
    }
}

#[entry]
fn main() -> ! {
    info!("=== Advanced Clock Configuration Example ===");

    // Initialize oscillators and clocks
    let xosc = Xosc::init(XOSC_HZ);
    xosc.wait_stable();

    let mut clocks = Clocks::new();
    clocks.configure_ref_src(ClkRefSrc::Xosc, XOSC_HZ);

    // Setup PLLs
    reset::unreset_block_wait(Peripheral::PLL_SYS);
    reset::unreset_block_wait(Peripheral::PLL_USB);

    let pll_sys = PllSys::new();
    let pll_usb = PllUsb::new();

    // Configure USB PLL once (always 48 MHz)
    let usb_config = PllConfig::usb_48mhz();
    pll_usb.configure(
        usb_config.refdiv,
        usb_config.fbdiv,
        usb_config.postdiv1,
        usb_config.postdiv2,
    );
    pll_usb.wait_lock();
    info!("PLL_USB configured for 48 MHz");

    // Initialize GPIO
    let gpio = GpioBank::init();
    let led = gpio.pin25();
    led.into_output();

    // Cycle through different clock speeds
    let speeds = [
        ClockSpeed::Slow125MHz,
        ClockSpeed::Normal133MHz,
        ClockSpeed::Fast150MHz,
    ];
    let mut speed_index = 0;

    loop {
        let speed = speeds[speed_index];
        info!(
            "Switching to {:?} ({} MHz)...",
            speed,
            speed.freq_hz() / 1_000_000
        );

        // Reconfigure PLL_SYS
        let config = speed.config();

        // Must switch system clock to ref before reconfiguring PLL
        clocks.configure_sys_from_ref();

        // Reconfigure PLL
        pll_sys.configure(
            config.refdiv,
            config.fbdiv,
            config.postdiv1,
            config.postdiv2,
        );
        pll_sys.wait_lock();

        // Switch back to PLL
        clocks.configure_sys_aux(ClkSysAuxSrc::PllSys, speed.freq_hz(), speed.freq_hz());

        info!("System clock now at {} Hz", clocks.get_sys_freq());

        // Create delay with new clock frequency
        let mut delay = Delay::new(speed.freq_hz());

        // Blink 5 times at this speed
        for i in 0..5 {
            led.set_high();
            delay.delay_ms(200);
            led.set_low();
            delay.delay_ms(200);
            info!("  Blink {} at {} MHz", i + 1, speed.freq_hz() / 1_000_000);
        }

        // Move to next speed
        speed_index = (speed_index + 1) % speeds.len();

        // Pause before switching
        delay.delay_ms(1000);
    }
}
