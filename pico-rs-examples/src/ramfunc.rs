//! # RAM Function Example
//!
//! Demonstrates `#[ramfunc]` — functions that execute from RAM instead of flash.
//! The startup code copies all `.ramfunc` code from flash to RAM before `main()`.
//!
//! ## Use Cases
//!
//! - Flash erase/program routines that must run while the flash controller is busy
//! - Performance-critical interrupt handlers
//! - Code that runs after XIP caching is disabled
//!
//! ## Verification
//!
//! This example reads the linker symbols `__sramfunc` and `__siramfunc` to show
//! the RAM (VMA) vs Flash (LMA) addresses, proving the code lives in RAM at runtime.

#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;

use pico_rs::Delay;
use pico_rs::bind;
use pico_rs::interrupt;
use pico_rs::main;
use pico_rs::pac::Interrupt;
use pico_rs::pac::Peripherals;
use pico_rs::ramfunc;
use pico_rs::rt::peripheral::Peripherals as CorePeripherals;

const CLOCK_FREQ_HZ: u32 = 12_000_000;

// ============================================================================
// Functions marked with #[ramfunc] are copied from flash to RAM at startup.
// ============================================================================

/// Compute a Fibonacci number — runs from RAM.
///
/// This is a typical "code that must run from RAM" scenario: if the flash
/// controller is busy (e.g. during an erase cycle), XIP reads would stall
/// or fault.  Running from RAM avoids that problem entirely.
#[ramfunc]
fn fib_ram(n: u32) -> u32 {
    if n <= 1 {
        n
    } else {
        fib_ram(n - 1) + fib_ram(n - 2)
    }
}

/// The same algorithm NOT in RAM — lives in `.text` (flash).
fn fib_flash(n: u32) -> u32 {
    if n <= 1 {
        n
    } else {
        fib_flash(n - 1) + fib_flash(n - 2)
    }
}

/// Fast inlined RAM helper.
#[ramfunc]
#[inline]
fn square_ram(x: u32) -> u32 {
    x * x
}

// ============================================================================
// #[ramfunc] + #[bind] — interrupt handler fully in RAM
// ============================================================================
//
// When both macros are applied, #[bind] detects the `link_section` attribute
// added by #[ramfunc] and also places the trampoline in `.ramfunc`.  This
// ensures the entire interrupt path runs from RAM: vector → trampoline → handler.

static mut ISR_COUNT: u32 = 0;

/// Interrupt handler in RAM.  Both the trampoline and this function run from
/// `.ramfunc`, so the ISR is safe even during flash operations.
#[bind(SPARE_IRQ_0)]
#[ramfunc]
fn ram_isr() {
    unsafe {
        ISR_COUNT = ISR_COUNT.wrapping_add(1);
    }
}

// ============================================================================
// Linker symbols: extern "C" references to symbols defined in link.x
// ============================================================================

unsafe extern "C" {
    /// Start of `.ramfunc` in RAM (VMA — Virtual Memory Address).
    static __sramfunc: u32;

    /// End of `.ramfunc` in RAM.
    static __eramfunc: u32;

    /// Start of `.ramfunc` in flash (LMA — Load Memory Address).
    static __siramfunc: u32;
}

#[main]
fn main() -> ! {
    info!("=== RAM Function Example ===");

    // --- Demonstrate #[ramfunc] ---
    let n = 10;
    let result = fib_ram(n);
    info!("fib_ram({}) = {}  (computed in RAM)", n, result);

    let result_flash = fib_flash(n);
    info!("fib_flash({}) = {}  (computed in flash)", n, result_flash);

    // --- Inline ramfunc ---
    let sq = square_ram(7);
    info!("square_ram(7) = {}", sq);

    // --- Read linker symbols to verify placement ---
    let ram_start = &raw const __sramfunc as u32;
    let ram_end = &raw const __eramfunc as u32;
    let flash_lma = &raw const __siramfunc as u32;

    let ramfunc_size = ram_end - ram_start;

    info!(
        ".ramfunc section:  VMA=0x{:08X}  LMA=0x{:08X}  size={} bytes",
        ram_start, flash_lma, ramfunc_size
    );

    // Get the actual runtime address of fib_ram (should be in the RAM range)
    let fib_addr = fib_ram as *const () as u32;
    info!("fib_ram runtime address: 0x{:08X}", fib_addr);

    if fib_addr >= ram_start && fib_addr < ram_end && ramfunc_size > 0 {
        info!("✓ fib_ram IS running from RAM (in .ramfunc range)");
    } else if ramfunc_size == 0 {
        warn!("⚠ .ramfunc section is empty — check that #[ramfunc] is applied");
    } else {
        warn!(
            "⚠ fib_ram address 0x{:08X} is outside .ramfunc [0x{:08X}..0x{:08X}]",
            fib_addr, ram_start, ram_end
        );
    }

    // fib_flash should be in flash range (0x10xxxxxx)
    let fib_flash_addr = fib_flash as *const () as u32;
    info!("fib_flash runtime address: 0x{:08X}", fib_flash_addr);

    // --- Verify #[ramfunc] + #[bind] combo ---
    let ram_isr_addr = ram_isr as *const () as u32;
    info!(
        "ram_isr [#ramfunc + #[bind]] address: 0x{:08X}",
        ram_isr_addr
    );
    if ram_isr_addr >= ram_start && ram_isr_addr < ram_end {
        info!("✓ ram_isr IS running from RAM (in .ramfunc range)");
    }

    // Enable SPARE_IRQ_0 and manually pend it to demonstrate the RAM ISR
    unsafe {
        interrupt::set_priority(Interrupt::SPARE_IRQ_0, 0xC0);
    }
    unsafe {
        interrupt::enable(Interrupt::SPARE_IRQ_0);
    }
    interrupt::pend(Interrupt::SPARE_IRQ_0);
    info!("SPARE_IRQ_0 enabled & pended — ram_isr should fire now");
    info!("ISR_COUNT = {} (expected: 1)", unsafe { ISR_COUNT });

    // --- Periodically exercise ramfunc ---
    let dp = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let delay = Delay::init(core.SYST, CLOCK_FREQ_HZ);

    // Also init GPIO to blink an LED via ramfunc
    let pins = pico_rs::gpio::Pins::init(dp.IO_BANK0, dp.PADS_BANK0);
    let mut led = pins.pin::<6u32>();
    led.into_output();

    let mut counter: u32 = 0;
    loop {
        counter = counter.wrapping_add(1);

        // Call fib_ram regularly to keep exercising RAM code
        let fib = fib_ram((counter % 12) as u32);
        led.toggle();

        // Every 10 iterations, print a status line and pend the RAM ISR
        if counter % 10 == 0 {
            interrupt::pend(Interrupt::SPARE_IRQ_0);
            info!(
                "iter={}  fib={}  sq={}  isr_cnt={}",
                counter,
                fib,
                square_ram(counter % 20),
                unsafe { ISR_COUNT }
            );
            info!("ISR_COUNT = {}", unsafe { ISR_COUNT });
        }

        delay.delay_ms(200);
    }
}
