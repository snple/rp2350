//! # UART + DMA Example (Builder Pattern)
//!
//! Demonstrates using DMA to send and receive data over UART0 on RP2350,
//! using the builder-pattern DMA API for clean, chainable configuration.
//!
//! ## Wiring
//!
//! - GPIO 0 → UART0 TX
//! - GPIO 1 → UART0 RX
//!
//! Connect TX to RX with a jumper wire for loopback testing, or connect
//! to a USB-to-serial adapter to see the output.
//!
//! ## What this example does
//!
//! 1. Initialises UART0 at 115200 baud on GPIO 0 (TX) / GPIO 1 (RX)
//! 2. Uses DMA channel 0 to transmit a message from memory to the UART TX FIFO
//! 3. Uses DMA channel 1 to receive bytes from the UART RX FIFO into memory
//! 4. Waits for the DMA transfers to complete and prints results

#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;

use pico_rs::dma::{DataSize, Dma};
use pico_rs::main;
use pico_rs::gpio::{Pin, Pins};
use pico_rs::pac::Peripherals;
use pico_rs::uart::Uart0;

const CLOCK_FREQ_HZ: u32 = 12_000_000;

#[main]
fn main() -> ! {
    info!("UART + DMA example (builder pattern)");

    // ------------------------------------------------------------------
    // 1. Peripheral setup
    // ------------------------------------------------------------------
    let dp = Peripherals::take().unwrap();
    let pins = Pins::new(dp.IO_BANK0, dp.PADS_BANK0);

    // UART0: 115200 baud, 8N1, GPIO 0 TX / GPIO 1 RX
    let uart = Uart0::new(
        dp.UART0,
        pins.pin::<0>(),
        pins.pin::<1>(),
        115200,
        CLOCK_FREQ_HZ,
    );

    let dma = Dma::new(dp.DMA);

    // ------------------------------------------------------------------
    // 2. DMA TX — memory → UART0 TX FIFO
    // ------------------------------------------------------------------
    let tx_data: &[u8] = b"Hello from RP2350 DMA!\r\n";
    uart.enable_dma_tx();

    info!("Starting DMA TX ({} bytes)...", tx_data.len());

    // Builder-pattern: configure → trigger → wait, all chained
    dma.channel::<0>()
        .mem_to_periph(
            tx_data.as_ptr() as u32,
            uart.data_register_address(),
            tx_data.len() as u32,
            Uart0::<Pin<0>, Pin<1>>::TX_DREQ,
            DataSize::Byte,
        )
        .trigger()
        .wait();

    info!("DMA TX complete");
    uart.flush();

    // ------------------------------------------------------------------
    // 3. DMA RX — UART0 RX FIFO → memory (loopback)
    // ------------------------------------------------------------------
    let mut rx_buf = [0u8; 24];
    uart.enable_dma_rx();

    let rx_ch = dma.channel::<1>();
    rx_ch
        .periph_to_mem(
            uart.data_register_address(),
            rx_buf.as_mut_ptr() as u32,
            rx_buf.len() as u32,
            Uart0::<Pin<0>, Pin<1>>::RX_DREQ,
            DataSize::Byte,
        )
        .trigger();

    // ------------------------------------------------------------------
    // 4. Check RX results (only meaningful with loopback wiring)
    // ------------------------------------------------------------------
    // Brief poll — in loopback mode data is already present
    for _ in 0..100_000u32 {
        if !rx_ch.is_busy() {
            break;
        }
        core::hint::spin_loop();
    }

    if !rx_ch.is_busy() {
        info!("DMA RX complete — received {} bytes", rx_buf.len());
        info!("RX data: {:a}", &rx_buf[..]);
    } else {
        let remaining = rx_ch.trans_count();
        info!(
            "DMA RX still waiting — {} of {} bytes received",
            rx_buf.len() as u32 - remaining,
            rx_buf.len()
        );
        rx_ch.abort();
        info!("RX DMA aborted (no loopback wiring)");
    }

    // ------------------------------------------------------------------
    // 5. Idle loop
    // ------------------------------------------------------------------
    info!("Done. Entering idle loop.");
    loop {
        core::hint::spin_loop();
    }
}
