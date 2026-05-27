//! Build script for roschi-rt
//!
//! This script:
//! - Copies the linker script to OUT_DIR
//! - Sets up linker search paths

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    // Copy link.x to OUT_DIR
    File::create(out_dir.join("link.x"))
        .unwrap()
        .write_all(include_bytes!("link.x"))
        .unwrap();

    // Add linker search path
    println!("cargo:rustc-link-search={}", out_dir.display());

    // Re-run if link.x changes
    println!("cargo:rerun-if-changed=link.x");
    println!("cargo:rerun-if-changed=build.rs");
}
