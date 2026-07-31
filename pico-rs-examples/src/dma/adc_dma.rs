//! # ADC + DMA Continuous Sampling Example
//!
//! Demonstrates using DMA to continuously read ADC samples from the
//! internal temperature sensor into a memory buffer.
//!
//! ## No external wiring required
//!
//! This example reads the on-chip temperature sensor (ADC channel 4),
//! which doesn't need any external connections.
//!
//! ## What this example does
//!
//! 1. Initialises ADC with the temperature sensor enabled
//! 2. Configures ADC FIFO with DREQ pacing
//! 3. Uses DMA channel 0 to collect samples into a buffer
//! 4. Converts and prints temperature readings

#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;

use pico_rs::adc::{Adc, TEMPERATURE_CHANNEL};
use pico_rs::clocks::ClockBuilder;
use pico_rs::dma::{DataSize, Dma};
use pico_rs::gpio::Pins;
use pico_rs::main;
use pico_rs::pac::Peripherals;
const NUM_SAMPLES: usize = 16;

#[main]
fn main() -> ! {
    info!("ADC + DMA temperature sampling example");

    let dp = Peripherals::take().unwrap();
    let _clocks = ClockBuilder::new()
        .enable_adc()
        .build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);
    let _pins = Pins::init(dp.IO_BANK0, dp.PADS_BANK0);

    // ------------------------------------------------------------------
    // ADC setup — temperature sensor on channel 4
    // ------------------------------------------------------------------
    let mut adc = Adc::init(dp.ADC);
    adc.enable_temp_sensor();

    // Use round-robin with only the temperature channel enabled,
    // so free-running mode continuously samples channel 4.
    adc.set_round_robin(1 << TEMPERATURE_CHANNEL);

    // Configure ADC FIFO:
    //   enable=true, dreq_enable=true, threshold=1,
    //   error_in_fifo=false, byte_shift=false (full 12-bit samples)
    adc.fifo_setup(true, true, 1, false, false);
    adc.fifo_drain();

    let dma = Dma::init(dp.DMA);

    // ------------------------------------------------------------------
    // DMA: ADC FIFO → memory buffer
    // ------------------------------------------------------------------
    let mut samples = [0u16; NUM_SAMPLES];

    // Use half-word (16-bit) transfers to match ADC sample size
    dma.channel::<0>()
        .periph_to_mem(
            adc.fifo_dma_address(),
            samples.as_mut_ptr() as u32,
            NUM_SAMPLES as u32,
            Adc::DREQ,
            DataSize::HalfWord,
        )
        .trigger();

    // Start free-running ADC conversion
    adc.start_free_running();

    // Wait for all samples to be collected
    dma.channel::<0>().wait();

    // Stop ADC
    adc.stop_free_running();
    adc.fifo_drain();

    // ------------------------------------------------------------------
    // Process and display results
    // ------------------------------------------------------------------
    info!(
        "Collected {} ADC samples from temperature sensor:",
        NUM_SAMPLES
    );

    for (i, &sample) in samples.iter().enumerate() {
        // Temperature formula from RP2350 datasheet:
        // T(°C) = 27 - (ADC_voltage - 0.706) / 0.001721
        // ADC_voltage = sample * 3.3 / 4096
        let voltage = (sample as f32) * 3.3 / 4096.0;
        let temp_c = 27.0 - (voltage - 0.706) / 0.001721;
        info!(
            "  [{}] raw={}, voltage={}.{}V, temp={}°C",
            i,
            sample,
            voltage as u32,
            ((voltage * 1000.0) as u32) % 1000,
            temp_c as i32,
        );
    }

    info!("Done.");
    loop {
        core::hint::spin_loop();
    }
}
