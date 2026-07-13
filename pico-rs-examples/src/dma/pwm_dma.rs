//! # PWM + DMA LED Fade Example
//!
//! Demonstrates using DMA to update PWM duty cycle automatically,
//! creating a smooth LED fade effect without CPU intervention.
//!
//! ## Wiring
//!
//! - GPIO 0 → LED (via resistor)
//!
//! ## What this example does
//!
//! 1. Initialises PWM slice 0, channel A on GPIO 0
//! 2. Prepares a lookup table of duty cycle values (triangle wave)
//! 3. Uses DMA with ring buffer to continuously cycle through the table
//! 4. The LED smoothly fades in and out

#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;

use pico_rs::clocks::{ClockBuilder};
use pico_rs::dma::{DataSize, Dma, TransCountMode, TreqSel};
use pico_rs::main;
use pico_rs::gpio::Pins;
use pico_rs::pac::Peripherals;
use pico_rs::pwm::Pwm;/// 64-entry triangle fade table.  Each entry stores the CC register value
/// for channel-A duty in bits [15:0].
///
/// Duty ramps 0 → 65535 → 0 in a triangular wave.
static FADE_TABLE: [u32; 64] = {
    let mut table = [0u32; 64];
    let mut i = 0;
    while i < 64 {
        // Triangle wave: 0→peak→0
        let duty = if i < 32 {
            (i as u32) * 2048 // 0..65536 in steps of 2048
        } else {
            (64 - i as u32) * 2048
        };
        // Pack into CC register: channel A in lower 16 bits
        table[i] = duty;
        i += 1;
    }
    table
};

#[main]
fn main() -> ! {
    info!("PWM + DMA LED fade example");

    let dp = Peripherals::take().unwrap();
    let _clocks = ClockBuilder::new().build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);
    let pins = Pins::init(dp.IO_BANK0, dp.PADS_BANK0);

    // ------------------------------------------------------------------
    // PWM setup — Slice 0, channel A for GPIO 0
    // ------------------------------------------------------------------
    let pwm = Pwm::init(dp.PWM);
    let slice = pwm.slice::<0>();
    let slice = slice.output_a(pins.pin::<0>());

    // Use 0xFFFE so CC=0xFFFF still represents a true 100% duty cycle.
    slice.set_wrap(0xFFFE);
    // Clock divider: slow down for visible LED fade
    // 150 MHz / 8 / 65535 ≈ 286 Hz PWM frequency
    slice.set_div(8, 0);
    slice.enable();

    // ------------------------------------------------------------------
    // DMA: continuously write fade table → PWM CC register
    // ------------------------------------------------------------------
    let dma = Dma::init(dp.DMA);
    let ch = dma.channel::<0>();

    // FADE_TABLE is 64 × 4 = 256 bytes = 2^8
    // Use ring-buffer on read side so DMA wraps around the table
    ch.disable()
        .set_data_size(DataSize::Word)
        .set_treq_sel(TreqSel::PwmWrap0)
        .set_incr_read(true)
        .set_incr_write(false) // fixed write to CC register
        .set_ring_sel(false) // ring on read side
        .set_ring_size(8) // 2^8 = 256 bytes = 64 words
        .set_chain_to(0) // chain to self = no chaining
        .set_trans_count_mode(TransCountMode::Endless) // infinite transfer
        .set_addrs(FADE_TABLE.as_ptr() as u32, slice.cc_address())
        .set_trans_count(FADE_TABLE.len() as u32)
        .trigger();

    info!("DMA running — LED is fading. CPU is free!");

    loop {
        // The CPU is completely free — DMA handles everything
        core::hint::spin_loop();
    }
}
