//! Set up linker scripts for pico-rs

use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    #[cfg(all(feature = "rp2350a", feature = "rp2350b"))]
    panic!("Only one of the features 'rp2350a' or 'rp2350b' can be enabled at a time.");

    // Put the linker script somewhere the linker can find it
    let out = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    println!("cargo:rustc-link-search={}", out.display());

    // The file `device.x` is required by cortex-m-rt when using the "device" feature
    let device_x = include_bytes!("device.x");
    let mut f = File::create(out.join("device.x")).unwrap();
    f.write_all(device_x).unwrap();

    println!("cargo:rerun-if-changed=device.x");
    println!("cargo:rerun-if-changed=build.rs");
}
