//! # RAM Function Example
//!
//! Demonstrates `#[ramfunc]` — functions that execute from RAM instead of flash.
//! The startup code copies all `.ramfunc` code from flash to RAM before
//! `__pre_init()` and `main()`.
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
//! the RAM (VMA) vs Flash (LMA) addresses, and checks the runtime addresses of
//! the functions to prove the code lives in RAM.  That part is demo scaffolding:
//! the `#[ramfunc]` functions work exactly the same without it, since the copy
//! is done by `rp2350-rt`'s startup code.

#![no_std]
#![no_main]

use core::sync::atomic::{AtomicU32, Ordering};

use defmt::*;
use defmt_rtt as _;

use rp2350::CorePeripherals;
use rp2350::Delay;
use rp2350::Peripherals;
use rp2350::bind;
use rp2350::interrupt;
use rp2350::main;
use rp2350::ramfunc;

const CLOCK_FREQ_HZ: u32 = 12_000_000;

// ============================================================================
// Functions marked with #[ramfunc] are copied from flash to RAM at startup.
// ============================================================================

/// Compute a Fibonacci number — runs from RAM.
///
/// This is a typical "code that must run from RAM" scenario: if the flash
/// controller is busy (e.g. during an erase cycle), XIP reads would stall
/// or fault.  Running from RAM avoids that problem entirely.
///
/// Note that `main` still has to reach this function through a linker veneer:
/// flash (`0x10xxxxxx`) and RAM (`0x20xxxxxx`) are further apart than a `BL`
/// can jump.
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

/// Fast RAM helper.
///
/// `#[ramfunc]` implies `#[inline(never)]`: an inlined copy would be emitted
/// into the caller's section (`main`, i.e. flash), so the macro refuses to
/// combine with `#[inline]`.
#[ramfunc]
fn square_ram(x: u32) -> u32 {
    x * x
}

// ============================================================================
// #[ramfunc] + #[bind] — interrupt handler fully in RAM
// ============================================================================
//
// `#[bind]` renames the function to the interrupt's link name via
// `#[export_name]` and keeps every other attribute, so combining the two in
// either order puts the interrupt entry point itself in `.ramfunc`:
// vector → handler, with no trampoline in flash.

static ISR_COUNT: AtomicU32 = AtomicU32::new(0);

/// Interrupt handler in RAM.  The handler is the entry point, so the whole
/// interrupt path runs from `.ramfunc` and is safe even during flash operations.
#[bind(SPARE_IRQ_0)]
#[ramfunc]
fn ram_isr() {
    ISR_COUNT.fetch_add(1, Ordering::Relaxed);
}

// ============================================================================
// Linker symbols — verification only, not part of using #[ramfunc]
// ============================================================================
//
// `link.x` defines these three symbols, and `rp2350-rt`'s `Reset` declares its
// own copies of them and performs the flash → RAM copy before `__pre_init()`
// runs.  They are re-declared here purely so this example can show *where* the
// code ended up: the section's VMA (RAM, 0x20xxxxxx) vs LMA (flash, 0x10xxxxxx),
// and whether a function's address falls inside `[__sramfunc, __eramfunc)`.
//
// An application using `#[ramfunc]` needs none of this — the block and the code
// that reads it can be deleted without changing the macro or the linker script.
// Taking only their addresses costs no RAM and no flash.
//
// The names must match `link.x`; a typo fails at link time with an undefined
// symbol.

unsafe extern "C" {
    /// Start of `.ramfunc` in RAM (VMA — Virtual Memory Address).
    static __sramfunc: u32;

    /// End of `.ramfunc` in RAM.
    static __eramfunc: u32;

    /// Start of `.ramfunc` in flash (LMA — Load Memory Address).
    static __siramfunc: u32;
}

/// Reports where `addr` ended up and flags a mismatch with `expect_ram`.
///
/// Part of the example's verification scaffolding — see the note above.
fn report_placement(name: &str, addr: u32, ram_start: u32, ram_end: u32, expect_ram: bool) {
    let in_ram = addr >= ram_start && addr < ram_end;
    match (in_ram, expect_ram) {
        (true, true) => info!("✓ {} @ 0x{:08X} — running from RAM", name, addr),
        (false, false) => info!("✓ {} @ 0x{:08X} — running from flash", name, addr),
        (false, true) => warn!(
            "⚠ {} @ 0x{:08X} is outside .ramfunc [0x{:08X}..0x{:08X}] — it runs from flash",
            name, addr, ram_start, ram_end
        ),
        (true, false) => warn!(
            "⚠ {} @ 0x{:08X} unexpectedly landed in .ramfunc",
            name, addr
        ),
    }
}

#[main]
fn main() -> ! {
    info!("=== RAM Function Example ===");

    // --- Verification scaffolding: read the linker symbols back ---
    // (Nothing below this point is required to use `#[ramfunc]`.)
    let ram_start = &raw const __sramfunc as u32;
    let ram_end = &raw const __eramfunc as u32;
    let flash_lma = &raw const __siramfunc as u32;
    let ramfunc_size = ram_end - ram_start;

    info!(
        ".ramfunc section:  VMA=0x{:08X}  LMA=0x{:08X}  size={} bytes",
        ram_start, flash_lma, ramfunc_size
    );

    if ramfunc_size == 0 {
        warn!("⚠ .ramfunc section is empty — check that #[ramfunc] is applied");
    }

    // --- Demonstrate #[ramfunc] and check where the code actually lives ---
    report_placement(
        "fib_ram",
        fib_ram as *const () as u32,
        ram_start,
        ram_end,
        true,
    );
    report_placement(
        "square_ram",
        square_ram as *const () as u32,
        ram_start,
        ram_end,
        true,
    );
    report_placement(
        "ram_isr",
        ram_isr as *const () as u32,
        ram_start,
        ram_end,
        true,
    );
    report_placement(
        "fib_flash",
        fib_flash as *const () as u32,
        ram_start,
        ram_end,
        false,
    );

    let n = 10;
    let result = fib_ram(n);
    let result_flash = fib_flash(n);
    info!("fib_ram({}) = {}  (computed in RAM)", n, result);
    info!("fib_flash({}) = {}  (computed in flash)", n, result_flash);
    info!("square_ram(7) = {}", square_ram(7));

    // --- Exercise the RAM ISR ---
    unsafe {
        interrupt::set_priority(interrupt::spare::SPARE_IRQ_0, 0xC0);
        interrupt::enable(interrupt::spare::SPARE_IRQ_0);
    }
    interrupt::pend(interrupt::spare::SPARE_IRQ_0);
    info!("SPARE_IRQ_0 enabled & pended — ram_isr should fire now");

    // The handler runs asynchronously, so wait for it instead of reading the
    // counter straight away (which would race and usually print 0).
    let mut spins = 0u32;
    while ISR_COUNT.load(Ordering::Relaxed) == 0 && spins < 100_000 {
        spins = spins.wrapping_add(1);
    }
    info!(
        "ISR_COUNT = {} (expected: 1)",
        ISR_COUNT.load(Ordering::Relaxed)
    );

    // --- Periodically exercise ramfunc ---
    let dp = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let delay = Delay::init(core.SYST, CLOCK_FREQ_HZ);

    // Also init GPIO to blink an LED via ramfunc
    let pins = rp2350::gpio::Pins::init(dp.IO_BANK0, dp.PADS_BANK0);
    let mut led = pins.pin::<6u32>();
    led.into_output();

    let mut counter: u32 = 0;
    loop {
        counter = counter.wrapping_add(1);

        // Call fib_ram regularly to keep exercising RAM code
        let fib = fib_ram(counter % 12);
        led.toggle();

        // Every 10 iterations, print a status line and pend the RAM ISR
        if counter.is_multiple_of(10) {
            interrupt::pend(interrupt::spare::SPARE_IRQ_0);
            info!(
                "iter={}  fib={}  sq={}  isr_cnt={}",
                counter,
                fib,
                square_ram(counter % 20),
                ISR_COUNT.load(Ordering::Relaxed)
            );
        }

        delay.delay_ms(200);
    }
}
