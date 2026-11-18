//! # I2C + DMA Example
//!
//! Demonstrates using DMA to perform I2C master writes and reads.
//! This example writes a register address to an I2C device, then
//! reads back data from it, both using DMA.
//!
//! ## Wiring
//!
//! - GPIO 4 → I2C0 SDA (with pull-up)
//! - GPIO 5 → I2C0 SCL (with pull-up)
//! - An I2C device on the bus (e.g. SHT3x at address 0x44)
//!
//! ## What this example does
//!
//! 1. Initialises I2C0 at 400 kHz on GPIO 4 (SDA) / GPIO 5 (SCL)
//! 2. Uses DMA channel 0 to write a command to the I2C device
//! 3. Uses DMA channel 1 to read response data from the I2C device
//!
//! NOTE: I2C DMA writes go to the IC_DATA_CMD register, which requires
//! special formatting (see RP2350 datasheet §4.3). Each 32-bit write
//! contains the data byte in bits [7:0] and control flags in upper bits.

#![no_std]
#![no_main]

#[cfg(feature = "defmt")]
use defmt_rtt as _;
use rp2350_examples::log::*;

use rp2350::clocks::ClockBuilder;
use rp2350::dma::{DataSize, Dma, TreqSel};
use rp2350::gpio::Pins;
use rp2350::i2c::I2c0;
use rp2350::main;
use rp2350::pac::Peripherals;
/// I2C target device address (e.g. SHT3x temperature sensor)
const I2C_ADDR: u8 = 0x44;

#[main]
fn main() -> ! {
    info!("I2C + DMA example");

    let dp = Peripherals::take().unwrap();
    let clocks = ClockBuilder::new().build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);
    let pins = Pins::init(dp.IO_BANK0, dp.PADS_BANK0);

    // ------------------------------------------------------------------
    // I2C0 setup — 400 kHz, GPIO 4=SDA, GPIO 5=SCL
    // ------------------------------------------------------------------
    let i2c = I2c0::init(dp.I2C0, pins.pin::<4>(), pins.pin::<5>(), 400_000, &clocks);

    // Set target address
    i2c.set_target_addr(I2C_ADDR as u16, false);

    let dma = Dma::init(dp.DMA);

    // ------------------------------------------------------------------
    // DMA TX: Write command bytes to I2C device
    // ------------------------------------------------------------------
    // For I2C, the IC_DATA_CMD register format (32-bit):
    //   bit [8]  = CMD: 0=write, 1=read
    //   bit [9]  = STOP: 1=issue STOP after this byte
    //   bit [10] = RESTART: 1=issue RESTART before this byte
    //   bits [7:0] = DATA byte
    //
    // Example: write [0x24, 0x00] (SHT3x measurement command)
    let tx_cmd: [u32; 2] = [
        0x0024, // write byte 0x24 (no STOP, no RESTART)
        0x0200, // write byte 0x00 + STOP (bit 9 = STOP)
    ];

    i2c.enable_dma_tx();

    info!("Starting I2C DMA TX ({} bytes)...", tx_cmd.len());

    dma.channel::<0>()
        .mem_to_periph(
            tx_cmd.as_ptr() as u32,
            i2c.data_register_address(),
            tx_cmd.len() as u32,
            TreqSel::I2c0Tx,
            DataSize::Word, // IC_DATA_CMD is 32-bit
        )
        .trigger()
        .wait();

    info!("I2C DMA TX complete");

    // ------------------------------------------------------------------
    // Brief delay for the device to process the command
    // ------------------------------------------------------------------
    for _ in 0..100_000u32 {
        core::hint::spin_loop();
    }

    // ------------------------------------------------------------------
    // DMA RX: Read response from I2C device
    // ------------------------------------------------------------------
    // To read via DMA, we need to write read-commands to IC_DATA_CMD
    // (CMD bit set = read request), then DMA the incoming data.
    //
    // Step 1: Issue read requests via DMA channel 0
    // Step 2: Collect received bytes via DMA channel 1
    let read_cmds: [u32; 6] = [
        0x0500, // READ + RESTART (bit 8=CMD, bit 10=RESTART)
        0x0100, // READ
        0x0100, // READ
        0x0100, // READ
        0x0100, // READ
        0x0300, // READ + STOP (bit 8=CMD, bit 9=STOP)
    ];
    let mut rx_buf = [0u8; 6]; // 6 bytes response

    i2c.enable_dma_rx();

    // Set up RX channel FIRST so we don't miss any data
    dma.channel::<1>()
        .periph_to_mem(
            i2c.data_register_address(),
            rx_buf.as_mut_ptr() as u32,
            rx_buf.len() as u32,
            TreqSel::I2c0Rx,
            DataSize::Byte, // we only want the data byte
        )
        .trigger();

    // Then send the read commands
    dma.channel::<0>()
        .mem_to_periph(
            read_cmds.as_ptr() as u32,
            i2c.data_register_address(),
            read_cmds.len() as u32,
            TreqSel::I2c0Tx,
            DataSize::Word,
        )
        .trigger();

    // Wait for both
    dma.channel::<0>().wait();

    // Brief timeout for RX
    for _ in 0..1_000_000u32 {
        if !dma.channel::<1>().is_busy() {
            break;
        }
        core::hint::spin_loop();
    }

    if !dma.channel::<1>().is_busy() {
        info!("I2C DMA RX complete — {} bytes received", rx_buf.len());
        info!("RX data: {:02x}", &rx_buf[..]);
    } else {
        let remaining = dma.channel::<1>().trans_count();
        warn!(
            "I2C DMA RX timeout — {} of {} bytes received",
            rx_buf.len() as u32 - remaining,
            rx_buf.len()
        );
        dma.channel::<1>().abort();
    }

    info!("Done.");
    loop {
        core::hint::spin_loop();
    }
}
