//! # Clock Configuration Example
//!
//! Demonstrates how to configure the RP2350 clock system to run at 150 MHz
//! using the external crystal oscillator (XOSC) and PLL.
//!
//! ## Clock Tree:
//!
//! ```text
//! XOSC (12 MHz) -> PLL_SYS -> CLK_SYS (150 MHz)
//!              \-> PLL_USB -> CLK_USB (48 MHz)
//!                         \-> CLK_ADC (48 MHz)
//! ```

#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

use pico_rs::block::ImageDef;
use pico_rs::clocks::{
    ClkAdcAuxSrc, ClkPeriAuxSrc, ClkRefSrc, ClkSysAuxSrc, ClkUsbAuxSrc, Clocks, PllConfig, PllSys,
    PllUsb, Xosc,
};
use pico_rs::entry;
use pico_rs::gpio::Pins;
use pico_rs::pac::Peripherals;
use pico_rs::reset::{self, Peripheral};
use pico_rs::rt::peripheral::Peripherals as CorePeripherals;
use pico_rs::systick::Delay;

/// RP2350 Boot Block
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::exe();

/// Target system clock frequency
const SYS_CLK_HZ: u32 = 150_000_000;

/// XOSC frequency (Pico 2 uses 12 MHz crystal)
const XOSC_HZ: u32 = 12_000_000;

/// USB/ADC clock frequency (must be 48 MHz)
const USB_CLK_HZ: u32 = 48_000_000;

#[entry]
fn main() -> ! {
    info!("=== RP2350 Clock Configuration Example ===");
    info!("Target: {} MHz system clock", SYS_CLK_HZ / 1_000_000);

    // Take device peripherals
    let dp = Peripherals::take().unwrap();

    // ========================================================================
    // Step 1: Initialize XOSC (Crystal Oscillator)
    // ========================================================================
    info!("Initializing XOSC ({} MHz crystal)...", XOSC_HZ / 1_000_000);

    let xosc = Xosc::init(dp.XOSC, XOSC_HZ);
    xosc.wait_stable();

    info!("XOSC stable and running");

    // ========================================================================
    // Step 2: Configure Reference Clock from XOSC
    // ========================================================================
    info!("Configuring reference clock from XOSC...");

    let mut clocks = Clocks::new(dp.CLOCKS, xosc);

    // Configure ref clock from XOSC first
    clocks.configure_ref_src(ClkRefSrc::Xosc, XOSC_HZ);

    info!("Reference clock: {} Hz", clocks.ref_freq());

    // Switch sys clock from ROSC to ref clock for stable operation
    info!("Switching sys clock to ref...");
    clocks.configure_sys_from_ref();
    info!("System clock now: {} Hz", clocks.sys_freq());

    // ========================================================================
    // Step 3: Configure PLL_SYS for 150 MHz
    // ========================================================================
    //
    // PLL calculation for 150 MHz from 12 MHz XOSC:
    //   VCO = (XOSC / REFDIV) * FBDIV
    //   OUT = VCO / (POSTDIV1 * POSTDIV2)
    //
    // Using: REFDIV=1, FBDIV=125, POSTDIV1=5, POSTDIV2=2
    //   VCO = (12 MHz / 1) * 125 = 1500 MHz (within 750-1600 MHz range)
    //   OUT = 1500 MHz / (5 * 2) = 150 MHz ✓
    //
    info!("Configuring PLL_SYS for {} MHz...", SYS_CLK_HZ / 1_000_000);

    // Reset and then unreset PLL_SYS to ensure clean state
    reset::reset_block(Peripheral::PLL_SYS);
    reset::unreset_block_wait(Peripheral::PLL_SYS);

    let pll_sys = PllSys::new(dp.PLL_SYS);

    // Use predefined configuration for 150 MHz
    let config = PllConfig::sys_150mhz();
    info!(
        "  REFDIV={}, FBDIV={}, POSTDIV1={}, POSTDIV2={}",
        config.refdiv, config.fbdiv, config.postdiv1, config.postdiv2
    );

    let vco_freq = config.vco_freq_khz(XOSC_HZ / 1000);
    let out_freq = config.output_freq_khz(XOSC_HZ / 1000);
    info!(
        "  VCO: {} MHz, Output: {} MHz",
        vco_freq / 1000,
        out_freq / 1000
    );

    pll_sys.configure(
        config.refdiv,
        config.fbdiv,
        config.postdiv1,
        config.postdiv2,
    );
    pll_sys.wait_lock();

    info!("PLL_SYS locked");

    // ========================================================================
    // Step 4: Configure PLL_USB for 48 MHz
    // ========================================================================
    //
    // Using: REFDIV=1, FBDIV=100, POSTDIV1=5, POSTDIV2=5
    //   VCO = (12 MHz / 1) * 100 = 1200 MHz
    //   OUT = 1200 MHz / (5 * 5) = 48 MHz ✓
    //
    info!("Configuring PLL_USB for 48 MHz...");

    // Reset and then unreset PLL_USB to ensure clean state
    reset::reset_block(Peripheral::PLL_USB);
    reset::unreset_block_wait(Peripheral::PLL_USB);

    let pll_usb = PllUsb::new(dp.PLL_USB);
    let usb_config = PllConfig::usb_48mhz();
    pll_usb.configure(
        usb_config.refdiv,
        usb_config.fbdiv,
        usb_config.postdiv1,
        usb_config.postdiv2,
    );
    pll_usb.wait_lock();

    info!("PLL_USB locked");

    // ========================================================================
    // Step 5: Configure System Clock from PLL_SYS
    // ========================================================================
    info!("Switching system clock to PLL_SYS...");

    clocks.configure_sys_aux(ClkSysAuxSrc::PllSys, SYS_CLK_HZ, SYS_CLK_HZ);

    let actual_sys_freq = clocks.sys_freq();
    info!("System clock: {} Hz", actual_sys_freq);
    if actual_sys_freq != SYS_CLK_HZ {
        warn!(
            "WARNING: System clock {} Hz != target {} Hz",
            actual_sys_freq, SYS_CLK_HZ
        );
    }

    // ========================================================================
    // Step 6: Configure Peripheral Clocks
    // ========================================================================
    info!("Configuring peripheral clocks...");

    // Peripheral clock from system clock
    clocks.configure_peri(ClkPeriAuxSrc::ClkSys, SYS_CLK_HZ);
    info!("  CLK_PERI: {} Hz", clocks.peri_freq());

    // USB clock from PLL_USB (48 MHz required)
    clocks.configure_usb(ClkUsbAuxSrc::PllUsb, USB_CLK_HZ);
    info!("  CLK_USB: {} Hz", clocks.usb_freq());

    // ADC clock from PLL_USB (48 MHz recommended)
    clocks.configure_adc(ClkAdcAuxSrc::PllUsb, USB_CLK_HZ);
    info!("  CLK_ADC: {} Hz", clocks.adc_freq());

    info!("=== Clock configuration complete! ===");

    // ========================================================================
    // Verification: Blink LED at precise rate
    // ========================================================================
    info!("Verifying clock accuracy with LED blink...");
    info!("LED should blink at exactly 1 Hz (500ms on, 500ms off)");

    // Initialize GPIO
    let pins = Pins::new(dp.IO_BANK0, dp.PADS_BANK0);
    let led = pins.pin::<25>();
    led.into_output();

    // Take core peripherals for Delay
    let core_peripherals = CorePeripherals::take().unwrap();

    // Create delay using the new system clock frequency
    let delay = Delay::new(core_peripherals.SYST, SYS_CLK_HZ);
    info!("Delay initialized with {} Hz", SYS_CLK_HZ);
    info!(
        "Delay ticks_per_us: {} (should be {})",
        delay.systick().ticks_per_us(),
        SYS_CLK_HZ / 1_000_000
    );

    let mut blink_count = 0u32;
    loop {
        led.set_high();
        delay.delay_ms(500);
        led.set_low();
        delay.delay_ms(500);

        blink_count += 1;
        if blink_count % 5 == 0 {
            info!("Blink #{}", blink_count);
        }
    }
}
