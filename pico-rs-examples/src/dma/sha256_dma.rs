//! # SHA-256 + DMA Example
//!
//! Demonstrates using DMA to feed data into the RP2350's hardware SHA-256
//! accelerator for efficient hashing without CPU intervention.
//!
//! ## No external wiring required
//!
//! ## What this example does
//!
//! 1. Initialises the SHA-256 hardware block
//! 2. Uses DMA channel 0 to transfer a message into the SHA-256 engine
//! 3. Reads out the computed hash and prints it

#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;

use pico_rs::dma::{DataSize, Dma};
use pico_rs::main;
use pico_rs::pac::Peripherals;
use pico_rs::sha256::{DmaSize, Sha256};#[main]
fn main() -> ! {
    info!("SHA-256 + DMA hashing example");

    let dp = Peripherals::take().unwrap();

    // ------------------------------------------------------------------
    // SHA-256 setup
    // ------------------------------------------------------------------
    let mut sha = Sha256::steal();

    // Initialise the SHA-256 engine (resets SUM registers to H0..H7)
    sha.start();

    // Configure for 32-bit DMA transfers (most efficient)
    sha.set_dma_size(DmaSize::Bit32);

    // Our padded_message is already in SHA-256 standard big-endian word
    // order, so byte-swap must be DISABLED.  (byte-swap is meant for
    // feeding raw little-endian memory bytes, which is what the
    // high-level `hash()` / `write_block()` path does.)
    sha.disable_byte_swap();

    // The message to hash — must be padded to SHA-256 block boundary.
    // SHA-256 processes 512-bit (64-byte) blocks. For the message "abc"
    // (3 bytes), we need to pad: message + 0x80 + zeros + length(bits).
    //
    // "abc" = [0x61, 0x62, 0x63]
    // Padded block (64 bytes / 16 words), big-endian:
    #[rustfmt::skip]
    let padded_message: [u32; 16] = [
        0x61626380, // 'a','b','c', 0x80
        0x00000000,
        0x00000000,
        0x00000000,
        0x00000000,
        0x00000000,
        0x00000000,
        0x00000000,
        0x00000000,
        0x00000000,
        0x00000000,
        0x00000000,
        0x00000000,
        0x00000000,
        0x00000000,
        0x00000018, // length = 24 bits = 3 bytes
    ];

    // Expected SHA-256("abc") =
    //   ba7816bf 8f01cfea 414140de 5dae2223
    //   b00361a3 96177a9c b410ff61 f20015ad

    let dma = Dma::new(dp.DMA);

    // ------------------------------------------------------------------
    // DMA: memory → SHA-256 WDATA register
    // ------------------------------------------------------------------
    info!("Starting DMA transfer to SHA-256 engine...");

    dma.channel::<0>()
        .mem_to_periph(
            padded_message.as_ptr() as u32,
            sha.wdata_address(),
            padded_message.len() as u32,
            Sha256::DREQ,
            DataSize::Word,
        )
        .trigger()
        .wait();

    info!("DMA transfer complete");

    // ------------------------------------------------------------------
    // Read the hash result
    // ------------------------------------------------------------------
    sha.wait_sum_valid();
    let hash = sha.read_hash_words();

    info!("SHA-256(\"abc\") =");
    info!(
        "  {:08x} {:08x} {:08x} {:08x}",
        hash[0], hash[1], hash[2], hash[3]
    );
    info!(
        "  {:08x} {:08x} {:08x} {:08x}",
        hash[4], hash[5], hash[6], hash[7]
    );

    // Verify against known hash
    let expected: [u32; 8] = [
        0xba7816bf, 0x8f01cfea, 0x414140de, 0x5dae2223, 0xb00361a3, 0x96177a9c, 0xb410ff61,
        0xf20015ad,
    ];

    if hash == expected {
        info!("Hash MATCHES expected value!");
    } else {
        warn!("Hash does NOT match — SHA-256 engine may need byte-swap config");
    }

    info!("Done.");
    loop {
        core::hint::spin_loop();
    }
}
