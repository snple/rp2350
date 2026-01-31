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
use std::path::PathBuf;
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
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::GenPac {
            svd,
            output,
            no_check,
        } => gen_pac(svd, output, no_check),
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

fn gen_pac(svd: Option<PathBuf>, output: Option<PathBuf>, no_check: bool) -> Result<()> {
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

    generator::generate_all(&device, &output_path).context("Failed to generate PAC code")?;

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
