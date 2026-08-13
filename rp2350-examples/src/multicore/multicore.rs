//! Multicore Example - Demonstrates dual-core operation on RP2350
//!
//! This example demonstrates:
//! - Launching code on Core 1
//! - Inter-core communication via FIFO
//! - Using hardware spinlocks for shared data protection
//! - Doorbell interrupts for signaling between cores
//!
//! Core 0 will launch Core 1 and communicate with it:
//! - Core 0 sends numbers to Core 1 via FIFO
//! - Core 1 doubles the numbers and sends them back
//! - Spinlocks protect access to a shared counter
//!
//! Run with: cargo run --release --bin multicore

#![no_std]
#![no_main]

use core::sync::atomic::{AtomicU32, Ordering};

use defmt::*;
use defmt_rtt as _;
use rp2350::clocks::ClockBuilder;
use rp2350::main;
use rp2350::multicore::{self, Fifo, Multicore, Stack};
use rp2350::pac::Peripherals;
use rp2350::sio::Spinlock;

static mut CORE1_STACK: Stack<4096> = Stack::new();

// Shared counter protected by spinlock
static SHARED_COUNTER: AtomicU32 = AtomicU32::new(0);

// Spinlock number for protecting critical sections
const COUNTER_SPINLOCK: u8 = 0;

/// Entry point for Core 1
///
/// This function runs on Core 1 after being launched by Core 0.
/// It receives values from Core 0, doubles them, and sends them back.
///
/// Note: We avoid using defmt logging in Core 1 because defmt_rtt
/// is not thread-safe and concurrent access from both cores would
/// cause data corruption.
fn core1_main() -> ! {
    let fifo = unsafe { Fifo::steal() };

    // Wait for the initial handshake value
    let handshake = fifo.read();

    // Send handshake response (echo it back)
    fifo.write(handshake);

    // Main loop: receive, process, and respond
    loop {
        // Wait for a value from Core 0
        let value = fifo.read();

        // Check for stop signal
        if value == 0xFFFF_FFFF {
            loop {
                rp2350_rt::asm::wfe();
            }
        }

        // Double the value
        let result = value.wrapping_mul(2);

        // Update shared counter with spinlock protection
        {
            let _guard = Spinlock::<COUNTER_SPINLOCK>::acquire();
            let current = SHARED_COUNTER.load(Ordering::Relaxed);
            SHARED_COUNTER.store(current + 1, Ordering::Relaxed);
        }

        // Send result back to Core 0
        fifo.write(result);
    }
}

#[main]
fn main() -> ! {
    info!("Multicore Example");
    info!("==================");

    // Take device peripherals
    let dp = Peripherals::take().unwrap();

    // Initialize clocks (150 MHz sys by default)
    let clocks = ClockBuilder::new().build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);

    // Initialize delay (matches actual system clock)
    let delay = rp2350_examples::common::board::init_delay(&clocks);

    // Print Core 0 info
    let core_id = multicore::core_id();
    info!("[Core 0] Running on {}", core_id);

    // Get FIFO interface for Core 0
    let fifo = unsafe { Fifo::steal() };

    // Drain any stale data in the FIFOs
    fifo.drain();

    // Get multicore controller
    let mut mc = Multicore::init(dp.PSM, dp.PPB);

    // Reset Core 1 first to ensure it's in a known state
    info!("[Core 0] Launching Core 1...");

    // Launch Core 1 with its own stack
    // SAFETY: CORE1_STACK is only accessed here, and core1_main never returns
    unsafe {
        mc.launch_core1(core1_main, &raw mut CORE1_STACK);
    }

    // Small delay for Core 1 to initialize
    delay.delay_ms(10);

    info!("[Core 0] Performing handshake with Core 1...");

    // Handshake: send magic value and wait for echo
    const HANDSHAKE_MAGIC: u32 = 0xCAFE_BABE;
    fifo.write(HANDSHAKE_MAGIC);

    let response = fifo.read();
    if response == HANDSHAKE_MAGIC {
        info!("[Core 0] Handshake successful!");
    } else {
        error!(
            "[Core 0] Handshake failed! Expected 0x{:08x}, got 0x{:08x}",
            HANDSHAKE_MAGIC, response
        );
        loop {
            rp2350_rt::asm::wfi();
        }
    }

    // Test: Send numbers to Core 1 and receive doubled results
    info!("[Core 0] Starting computation test...");
    info!("");

    for i in 1u32..=10 {
        info!("[Core 0] Sending: {}", i);
        fifo.write(i);

        // Wait for response
        let result = fifo.read();
        info!("[Core 0] Received: {} (expected {})", result, i * 2);

        if result != i * 2 {
            error!("[Core 0] ERROR: Expected {}, got {}", i * 2, result);
        }

        // Check shared counter (protected by spinlock)
        let counter = {
            let _guard = Spinlock::<COUNTER_SPINLOCK>::acquire();
            SHARED_COUNTER.load(Ordering::Relaxed)
        };
        info!("[Core 0] Shared counter: {}", counter);
        info!("");

        delay.delay_ms(500);
    }

    // All tests complete
    info!("==================");
    info!("All tests completed successfully!");

    // Read final counter value
    let final_counter = {
        let _guard = Spinlock::<COUNTER_SPINLOCK>::acquire();
        SHARED_COUNTER.load(Ordering::Relaxed)
    };
    info!(
        "Final shared counter value: {} (expected 10)",
        final_counter
    );

    // Send stop signal to Core 1
    info!("[Core 0] Sending stop signal to Core 1...");
    fifo.write(0xFFFF_FFFF);

    info!("[Core 0] Done! Entering idle loop.");

    loop {
        rp2350_rt::asm::wfe();
    }
}
