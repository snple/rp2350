//! Build script for pico-rs-rt
//!
//! This script:
//! - Copies the linker script to OUT_DIR
//! - Sets up linker search paths
//! - Selects link.x or link_ram.x based on the `run-from-ram` feature

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    // Select the appropriate linker script
    let run_from_ram = env::var("CARGO_FEATURE_RUN_FROM_RAM").is_ok();
    let link_script: &[u8] = if run_from_ram {
        include_bytes!("link_ram.x")
    } else {
        include_bytes!("link.x")
    };

    File::create(out_dir.join("link.x"))
        .unwrap()
        .write_all(link_script)
        .unwrap();

    // Add linker search path
    println!("cargo:rustc-link-search={}", out_dir.display());

    // Re-run if linker scripts or build.rs change
    println!("cargo:rerun-if-changed=link.x");
    println!("cargo:rerun-if-changed=link_ram.x");
    println!("cargo:rerun-if-changed=build.rs");
}
