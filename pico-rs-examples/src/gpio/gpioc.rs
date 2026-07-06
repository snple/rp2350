//! GPIOC (GPIO Coprocessor) comprehensive example for RP2350
//!
//! Demonstrates all GPIOC features using the M33 CP0 coprocessor:
//! - Pin-level output control (set_high / set_low / toggle / set)
//! - Mask-level batch output (set / clr / xor / write)
//! - Output enable direction control (enable / disable / toggle)
//! - Input sampling (get / mask_input / in64)
//! - 64-bit atomic operations (out64 / output_enable64)

#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;

use pico_rs::Delay;
use pico_rs::main;
use pico_rs::gpio::Pins;
use pico_rs::gpioc;
use pico_rs::pac;
use pico_rs::rt::peripheral::Peripherals as CorePeripherals;const CLOCK_FREQ_HZ: u32 = 12_000_000;
const LED: u32 = 6;
const TEST_OUT: u32 = 7;
const TEST_IN: u32 = 8;

#[main]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();

    let pins = Pins::new(dp.IO_BANK0, dp.PADS_BANK0);
    let _led = pins.pin::<LED>().into_output();
    let _out = pins.pin::<TEST_OUT>().into_output();
    let _in = pins.pin::<TEST_IN>().into_input();

    let delay = Delay::new(core.SYST, CLOCK_FREQ_HZ);

    info!("=== GPIOC Comprehensive Example ===");

    // 1. Pin-level output: set_high / set_low / toggle / set
    info!("--- 1. Pin-level Output ---");
    unsafe {
        gpioc::set_high(LED);
        info!("  set_high({}): output={}", LED, gpioc::output(LED));
        delay.delay_ms(200);
        gpioc::set_low(LED);
        info!("  set_low({}):  output={}", LED, gpioc::output(LED));
        gpioc::toggle(LED);
        info!("  toggle({}):   output={}", LED, gpioc::output(LED));
        gpioc::set(LED, false);
    }

    // 2. Mask-level output: batch read/write/set/clr/xor for GPIO 0–31
    info!("--- 2. Mask-level Output (low bank) ---");
    unsafe {
        gpioc::mask_set_low(1 << LED);
        info!(
            "  mask_set(1<<{}): mask_output={:#034b}",
            LED,
            gpioc::mask_output_low()
        );
        gpioc::mask_clr_lo(1 << LED);
        info!(
            "  mask_clr(1<<{}): mask_output={:#034b}",
            LED,
            gpioc::mask_output_low()
        );
        gpioc::mask_xor_lo(1 << LED);
        info!(
            "  mask_xor(1<<{}): mask_output={:#034b}",
            LED,
            gpioc::mask_output_low()
        );
        gpioc::mask_write_output_low(0);
    }

    // 3. Output enable (direction control)
    info!("--- 3. Output Enable ---");
    unsafe {
        gpioc::enable_output(TEST_OUT);
        info!(
            "  enable_output({}):  is_output={}",
            TEST_OUT,
            gpioc::is_output(TEST_OUT)
        );
        gpioc::disable_output(TEST_OUT);
        info!(
            "  disable_output({}): is_output={}",
            TEST_OUT,
            gpioc::is_output(TEST_OUT)
        );
        gpioc::set_output_enable(TEST_OUT, true);
        gpioc::toggle_output_enable(TEST_OUT);
        info!(
            "  toggle_output_enable({}): is_output={}",
            TEST_OUT,
            gpioc::is_output(TEST_OUT)
        );
        gpioc::enable_output(TEST_OUT);

        // mask-level OE batch operations
        gpioc::mask_set_output_enable_lo(1 << TEST_OUT);
        info!(
            "  mask_set_output_enable(1<<{}):  mask_oe={:#034b}",
            TEST_OUT,
            gpioc::mask_output_enable_lo()
        );
        gpioc::mask_toggle_output_enable_lo(1 << TEST_OUT);
    }

    // 4. Input sampling
    info!("--- 4. Input Sampling ---");
    unsafe {
        gpioc::mask_set_low(1 << TEST_OUT);
        delay.delay_ms(1);
        info!("  get({})          = {}", TEST_IN, gpioc::get(TEST_IN));
        info!("  output({})       = {}", TEST_OUT, gpioc::output(TEST_OUT));
        info!("  mask_input_low() = {:#034b}", gpioc::mask_input_low());
        info!("  mask_input_hi()  = {:#034b}", gpioc::mask_input_hi());
        info!("  in64()           = {:#018x}", gpioc::in64());
    }

    // 5. 64-bit atomic operations
    info!("--- 5. 64-bit Atomic Operations ---");
    unsafe {
        gpioc::out64_set(1u64 << LED);
        info!("  out64_set:      out64={:#018x}", gpioc::out64_get());
        gpioc::out64_clr(1u64 << LED);
        gpioc::out64_xor(1u64 << LED);
        info!("  out64_xor:      out64={:#018x}", gpioc::out64_get());
        gpioc::out64_clr(1u64 << LED);

        gpioc::output_enable64_set(1u64 << LED);
        info!(
            "  output_enable64_set: oe64={:#018x}",
            gpioc::output_enable64_get()
        );
        gpioc::output_enable64_clr(1u64 << LED);
    }

    // 6. High-bank mask-level (GPIO 32–63, RP2350B only; reads 0 on RP2350A)
    info!("--- 6. High-bank (GPIO 32–63) ---");
    unsafe {
        info!("  mask_output_hi       = {:#034b}", gpioc::mask_output_hi());
        info!(
            "  mask_output_enable_hi = {:#034b}",
            gpioc::mask_output_enable_hi()
        );
    }

    info!("=== Done ===");
    loop {
        unsafe {
            gpioc::toggle(LED);
        }
        delay.delay_ms(1000);
    }
}
