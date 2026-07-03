//! GPIOC (GPIO Coprocessor) comprehensive example for RP2350
//!
//! Demonstrates all GPIOC features using the M33 CP0 coprocessor:
//! - Low/high bank output control (MCR)
//! - Single-bit output operations (MCRR bit-level)
//! - Output enable direction control
//! - Input sampling (MRC/MRRC)
//! - 64-bit atomic operations
//! - Compile-time pin read dispatch

#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;

use pico_rs::Delay;
use pico_rs::block::ImageDef;
use pico_rs::entry;
use pico_rs::gpio::Pins;
use pico_rs::gpioc;
use pico_rs::pac;
use pico_rs::rt::peripheral::Peripherals as CorePeripherals;

#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::exe();

const CLOCK_FREQ_HZ: u32 = 12_000_000;
const LED: u8 = 6;
const TEST_OUT: u8 = 7;
const TEST_IN: u8 = 8;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();

    let pins = Pins::new(dp.IO_BANK0, dp.PADS_BANK0);
    let _led = pins.pin::<LED>().into_output();
    let _out = pins.pin::<TEST_OUT>().into_output();
    let _in = pins.pin::<TEST_IN>().into_input();

    let delay = Delay::new(core.SYST, CLOCK_FREQ_HZ);

    info!("=== GPIOC Comprehensive Example ===");

    // 1. Low-bank output (GPIO 0–31)
    info!("--- 1. Low-bank Output ---");
    gpioc::out_lo_set(1 << LED);
    info!("  set(1<<{}):  out_lo={:#034b}", LED, gpioc::out_lo_get());
    delay.delay_ms(200);
    gpioc::out_lo_clr(1 << LED);
    info!("  clr(1<<{}):  out_lo={:#034b}", LED, gpioc::out_lo_get());
    gpioc::out_lo_xor(1 << LED);
    info!("  xor(1<<{}):  out_lo={:#034b}", LED, gpioc::out_lo_get());
    gpioc::out_lo_put(0);

    // 2. High-bank output (GPIO 32–63)
    info!("--- 2. High-bank Output ---");
    info!("  out_hi_get()={:#034b}", gpioc::out_hi_get());

    // 3. Single-bit output (MCRR bit-level, opc 4–7)
    info!("--- 3. Single-bit Output ---");
    gpioc::bit_out_set(LED as u32);
    info!("  bit_set({}):   bit={}", LED, gpioc::out_pin_lo_get(LED as u32));
    gpioc::bit_out_clr(LED as u32);
    gpioc::bit_out_toggle(LED as u32);
    info!("  bit_toggle({}):bit={}", LED, gpioc::out_pin_lo_get(LED as u32));
    gpioc::bit_out_put(LED as u32, false);

    // 4. 64-bit atomic output
    info!("--- 4. 64-bit Atomic Output ---");
    gpioc::out64_set(1u64 << LED);
    info!("  out64_set: out64={:#018x}", gpioc::out64_get());
    gpioc::out64_clr(1u64 << LED);
    gpioc::out64_xor(1u64 << LED);
    info!("  out64_xor: out64={:#018x}", gpioc::out64_get());
    gpioc::out64_clr(1u64 << LED);

    // 5. Output Enable
    info!("--- 5. Output Enable ---");
    gpioc::oe_lo_set(1 << TEST_OUT);
    info!("  oe_lo_set(1<<{}): oe_pin={}", TEST_OUT, gpioc::oe_pin_lo_get(TEST_OUT as u32));
    gpioc::bit_oe_clr(TEST_OUT as u32);
    info!("  bit_oe_clr({}):   oe_pin={}", TEST_OUT, gpioc::oe_pin_lo_get(TEST_OUT as u32));
    gpioc::oe_lo_set(1 << TEST_OUT);

    // 6. Input sampling
    info!("--- 6. Input Sampling ---");
    gpioc::out_lo_set(1 << TEST_OUT);
    delay.delay_ms(1);
    info!("  in_lo()  ={:#034b}", gpioc::in_lo());
    info!("  in_hi()  ={:#034b}", gpioc::in_hi());
    info!("  in64()   ={:#018x}", gpioc::in64());
    info!("  pin_lo({})={}", TEST_IN, gpioc::in_pin_lo(TEST_IN as u32));

    // 7. Compile-time pin read (zero-overhead)
    info!("--- 7. Compile-time Pin Read ---");
    info!("  const_in_pin::<{}>()      = {}", LED, gpioc::const_in_pin::<{LED as u32}>());
    info!("  const_out_pin_get::<{}>() = {}", LED, gpioc::const_out_pin_get::<{LED as u32}>());
    info!("  const_oe_pin_get::<{}>()  = {}", LED, gpioc::const_oe_pin_get::<{LED as u32}>());

    // 8. Indexed operations
    info!("--- 8. Indexed Operations ---");
    gpioc::index_out_set(0, 1 << TEST_OUT);
    info!("  index_out_set(0,1<<{}): out_lo={:#034b}", TEST_OUT, gpioc::out_lo_get());
    gpioc::index_out_clr(0, 1 << TEST_OUT);
    gpioc::index_oe_set(0, 1 << TEST_OUT);
    info!("  index_oe_set(0,1<<{}):  oe_lo={:#034b}", TEST_OUT, gpioc::oe_lo_get());

    // 9. Pin read API summary
    info!("--- 9. Pin Read API ---");
    gpioc::out_lo_set(1 << TEST_OUT);
    delay.delay_ms(1);
    info!("  in_pin({})         = {}", TEST_IN, gpioc::in_pin(TEST_IN as u32));
    info!("  in_pin_lo({})      = {}", TEST_IN, gpioc::in_pin_lo(TEST_IN as u32));
    info!("  const_in_pin::<{}>()  = {}", TEST_IN, gpioc::const_in_pin::<{TEST_IN as u32}>());
    info!("  out_pin_get({})       = {}", TEST_OUT, gpioc::out_pin_get(TEST_OUT as u32));
    info!("  out_pin_lo_get({})    = {}", TEST_OUT, gpioc::out_pin_lo_get(TEST_OUT as u32));
    info!("  const_out_pin_get::<{}>() = {}", TEST_OUT, gpioc::const_out_pin_get::<{TEST_OUT as u32}>());
    info!("  oe_pin_get({})        = {}", LED, gpioc::oe_pin_get(LED as u32));
    info!("  oe_pin_lo_get({})     = {}", LED, gpioc::oe_pin_lo_get(LED as u32));
    info!("  const_oe_pin_get::<{}>()  = {}", LED, gpioc::const_oe_pin_get::<{LED as u32}>());

    info!("=== Done ===");
    loop {
        gpioc::bit_out_toggle(LED as u32);
        delay.delay_ms(1000);
    }
}
