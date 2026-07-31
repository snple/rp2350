//! Build automation tasks for pico-rs
//!
//! Run with `cargo xtask <command>`
//!
//! Available commands:
//! - `gen-pac`: Generate PAC files from SVD

mod generator;
mod svd_parser;
mod types;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Parser)]
#[command(name = "xtask")]
#[command(about = "Build automation tasks for pico-rs")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate PAC files from SVD
    GenPac {
        /// Path to the SVD file (default: RP2350.svd in workspace root)
        #[arg(short, long)]
        svd: Option<PathBuf>,

        /// Output directory (default: pico-rs-pac/src/generated)
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Skip compilation check after generation
        #[arg(long)]
        no_check: bool,

        /// Only verify that the checked-in generated code is up to date with
        /// the SVD. Generates to a temporary directory and compares; writes
        /// nothing.
        #[arg(long)]
        check: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::GenPac {
            svd,
            output,
            no_check,
            check,
        } => gen_pac(svd, output, no_check, check),
    }
}

fn workspace_root() -> PathBuf {
    let output = Command::new(env!("CARGO"))
        .args(["locate-project", "--workspace", "--message-format=plain"])
        .output()
        .expect("Failed to run cargo locate-project");

    let path = String::from_utf8(output.stdout).expect("Invalid UTF-8");
    PathBuf::from(path.trim())
        .parent()
        .expect("Cargo.toml has no parent")
        .to_path_buf()
}

fn gen_pac(
    svd: Option<PathBuf>,
    output: Option<PathBuf>,
    no_check: bool,
    check: bool,
) -> Result<()> {
    let root = workspace_root();

    let svd_path = svd.unwrap_or_else(|| root.join("RP2350.svd"));
    let output_path = output.unwrap_or_else(|| root.join("pico-rs-pac/src/generated"));

    println!("Generating PAC from: {}", svd_path.display());
    println!("Output directory: {}", output_path.display());

    // Check SVD file exists
    if !svd_path.exists() {
        anyhow::bail!("SVD file not found: {}", svd_path.display());
    }

    // Parse SVD and generate code
    let device = svd_parser::parse_svd(&svd_path).context("Failed to parse SVD file")?;

    if check {
        if no_check {
            anyhow::bail!("--check and --no-check are mutually exclusive");
        }
        return check_generated(&device, &output_path);
    }

    generator::generate_all(&device, &output_path).context("Failed to generate PAC code")?;
    format_generated(&output_path)?;

    println!("\n✓ PAC generated successfully!");

    if no_check {
        return Ok(());
    }

    // Check if it compiles
    println!("\nVerifying PAC compiles...");
    let status = Command::new(env!("CARGO"))
        .args([
            "build",
            "-p",
            "pico-rs-pac",
            "--target",
            "thumbv8m.main-none-eabihf",
        ])
        .current_dir(&root)
        .status()
        .context("Failed to build pico-rs-pac")?;

    if !status.success() {
        anyhow::bail!("PAC compilation failed");
    }

    println!("✓ PAC compiles successfully!");

    Ok(())
}

/// Generate the PAC into a temporary directory and compare it against the
/// checked-in `output_path`, without writing anything. Fails if they differ.
fn check_generated(device: &types::SvdDevice, output_path: &Path) -> Result<()> {
    let tmp = std::env::temp_dir().join(format!("pico-rs-pac-check-{}", std::process::id()));

    // Clean up any stale temp dir from a previous run.
    let _ = std::fs::remove_dir_all(&tmp);
    std::fs::create_dir_all(&tmp).context("Failed to create temp dir")?;

    generator::generate_all(device, &tmp).context("Failed to generate PAC code")?;
    format_generated(&tmp)?;

    let result = compare_dirs(&tmp, output_path);
    let _ = std::fs::remove_dir_all(&tmp);
    result?;

    println!("✓ Generated PAC is up to date with RP2350.svd");
    Ok(())
}

/// Format all generated `.rs` files under `dir` with `rustfmt`.
///
/// This keeps the checked-in generated code identical to `cargo fmt --all`
/// output, so the formatting CI check stays green without relying on the
/// nightly-only `ignore` config option. Both `gen-pac` (writes) and
/// `gen-pac --check` (temporary directory) apply this so their outputs always
/// compare equal.
fn format_generated(dir: &Path) -> Result<()> {
    fn collect_rs(dir: &Path, files: &mut Vec<PathBuf>) {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    collect_rs(&path, files);
                } else if path.extension().is_some_and(|e| e == "rs") {
                    files.push(path);
                }
            }
        }
    }

    let mut files = Vec::new();
    collect_rs(dir, &mut files);
    if files.is_empty() {
        return Ok(());
    }

    let status = Command::new("rustfmt")
        .arg("--edition")
        .arg("2024")
        .args(&files)
        .status()
        .context("Failed to run rustfmt on generated code")?;
    if !status.success() {
        anyhow::bail!("rustfmt failed on generated code");
    }
    Ok(())
}

/// Recursively compare the contents of two directories.
fn compare_dirs(a: &Path, b: &Path) -> Result<()> {
    fn collect(dir: &Path) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();
        for entry in std::fs::read_dir(dir).context("Failed to read directory")? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                files.extend(collect(&path)?);
            } else {
                files.push(path);
            }
        }
        Ok(files)
    }

    let a_files = collect(a)?;
    let b_files = collect(b)?;

    if a_files.len() != b_files.len() {
        anyhow::bail!(
            "Generated file count differs: regenerated has {}, checked-in has {}\n  run `cargo xtask gen-pac` to update",
            a_files.len(),
            b_files.len()
        );
    }

    for fa in &a_files {
        let rel_path = fa.strip_prefix(a).expect("temp file inside temp dir");
        let fb = b.join(rel_path);

        if !fb.exists() {
            anyhow::bail!(
                "Checked-in generated file is missing: {}\n  run `cargo xtask gen-pac` to update",
                rel_path.display()
            );
        }

        let fa_content = std::fs::read(fa)?;
        let fb_content = std::fs::read(&fb)?;
        if fa_content != fb_content {
            anyhow::bail!(
                "Generated PAC is out of date: {} differs\n  run `cargo xtask gen-pac` to update",
                rel_path.display()
            );
        }
    }

    Ok(())
}
