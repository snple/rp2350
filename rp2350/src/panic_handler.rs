//! Default panic handling for bare-metal `rp2350` binaries.
//!
//! The panic path uses the SCB definition from `rp2350-rt`, not the device
//! PAC, because `SHCSR` is a Cortex-M core register rather than an RP2350
//! peripheral register.

use core::panic::PanicInfo;
use core::sync::atomic::{AtomicBool, Ordering};

#[cfg(feature = "defmt")]
#[defmt::panic_handler]
fn defmt_panic() -> ! {
    panic_trap()
}

#[cfg(feature = "defmt")]
fn print_panic(info: &PanicInfo) {
    defmt::error!("{}", defmt::Display2Format(info));
}

#[cfg(not(feature = "defmt"))]
fn print_panic(_: &PanicInfo) {}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    static PANICKED: AtomicBool = AtomicBool::new(false);

    crate::rt::interrupt::disable();

    if !PANICKED.swap(true, Ordering::Relaxed) {
        print_panic(info);
    }

    panic_trap()
}

fn panic_trap() -> ! {
    const USGFAULTENA: usize = 18;

    unsafe {
        let shcsr = &(*crate::rt::peripheral::scb::SCB::PTR).shcsr;
        let mut shcsr_bits = shcsr.read_bits();
        shcsr_bits &= !(1 << USGFAULTENA);
        shcsr.write_bits(shcsr_bits);
    }

    crate::rt::asm::udf()
}
