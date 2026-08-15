//! Set up linker scripts for RP2350 examples

use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    // Put the linker script somewhere the linker can find it
    let out = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    println!("cargo:rustc-link-search={}", out.display());

    // Link the defmt linker script (`defmt.x`) provided by the `defmt` crate.
    // The examples always use defmt; keeping this here instead of the
    // workspace-wide `.cargo/config.toml` leaves the HAL defmt-agnostic.
    println!("cargo:rustc-link-arg=-Tdefmt.x");

    // The file `memory.x` is loaded by cortex-m-rt's `link.x` script
    let memory_x = include_bytes!("memory.x");
    let mut f = File::create(out.join("memory.x")).unwrap();
    f.write_all(memory_x).unwrap();

    println!("cargo:rerun-if-changed=memory.x");
    println!("cargo:rerun-if-changed=build.rs");
}
