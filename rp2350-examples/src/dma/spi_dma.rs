//! # SPI + DMA Loopback Example
//!
//! Demonstrates using DMA to transfer data through SPI0 in loopback mode.
//!
//! ## Wiring
//!
//! - GPIO 2 → SPI0 SCK
//! - GPIO 3 → SPI0 TX (MOSI)
//! - GPIO 0 → SPI0 RX (MISO)
//!
//! For loopback testing, connect GPIO 3 (MOSI) → GPIO 0 (MISO).
//!
//! ## What this example does
//!
//! 1. Initialises SPI0 at 1 MHz
//! 2. Uses DMA channel 0 to transmit data from memory → SPI TX FIFO
//! 3. Uses DMA channel 1 to receive data from SPI RX FIFO → memory
//! 4. Verifies received data matches sent data (in loopback)

#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;

use rp2350::clocks::ClockBuilder;
use rp2350::dma::{DataSize, Dma, TreqSel};
use rp2350::gpio::Pins;
use rp2350::main;
use rp2350::pac::Peripherals;
use rp2350::spi;
#[main]
fn main() -> ! {
    info!("SPI + DMA loopback example");

    let dp = Peripherals::take().unwrap();
    let clocks = ClockBuilder::new().build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);
    let pins = Pins::init(dp.IO_BANK0, dp.PADS_BANK0);

    // SPI0: 1 MHz, GPIO 2=SCK, GPIO 3=TX, GPIO 0=RX
    let spi = spi::Spi0::init(
        dp.SPI0,
        pins.pin::<2>(),
        pins.pin::<3>(),
        pins.pin::<0>(),
        1_000_000,
        clocks.peri_freq(),
    );

    let dma = Dma::init(dp.DMA);

    // ------------------------------------------------------------------
    // Prepare buffers
    // ------------------------------------------------------------------
    let tx_data: [u8; 8] = [0xDE, 0xAD, 0xBE, 0xEF, 0xCA, 0xFE, 0xBA, 0xBE];
    let mut rx_buf = [0u8; 8];

    // Enable SPI DMA for both TX and RX
    spi.enable_dma_tx();
    spi.enable_dma_rx();

    // ------------------------------------------------------------------
    // DMA RX — set up first so it's ready before TX starts
    // ------------------------------------------------------------------
    dma.channel::<1>()
        .periph_to_mem(
            spi.data_register_address(),
            rx_buf.as_mut_ptr() as u32,
            rx_buf.len() as u32,
            TreqSel::Spi0Rx,
            DataSize::Byte,
        )
        .trigger();

    // ------------------------------------------------------------------
    // DMA TX — start transmitting
    // ------------------------------------------------------------------
    dma.channel::<0>()
        .mem_to_periph(
            tx_data.as_ptr() as u32,
            spi.data_register_address(),
            tx_data.len() as u32,
            TreqSel::Spi0Tx,
            DataSize::Byte,
        )
        .trigger();

    // ------------------------------------------------------------------
    // Wait for both to finish
    // ------------------------------------------------------------------
    dma.channel::<0>().wait();
    dma.channel::<1>().wait();

    // Wait for SPI to finish shifting
    spi.flush();

    info!("TX data: {:02x}", &tx_data[..]);
    info!("RX data: {:02x}", &rx_buf[..]);

    // Verify loopback
    if tx_data == rx_buf {
        info!("Loopback OK — data matches!");
    } else {
        warn!("Loopback MISMATCH — check MOSI→MISO wiring");
    }

    info!("Done.");
    loop {
        core::hint::spin_loop();
    }
}
