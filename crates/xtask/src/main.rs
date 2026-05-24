//! Fe Reader xtask Wave 0 harness.

use anyhow::{bail, Result};
use clap::{Parser, Subcommand};
use std::process::Command;

#[derive(Debug, Parser)]
#[command(name = "xtask", version, about = "Fe Reader repo automation")]
struct Cli {
    #[command(subcommand)]
    command: Task,
}

#[derive(Debug, Subcommand)]
enum Task {
    /// Run the Wave 0 bootstrap checks.
    Wave0Check,
    /// Run formatting, clippy, tests and schema smoke checks where available.
    Review,
    /// Validate JSON schemas.
    ValidateSchemas,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Task::Wave0Check => run_script("scripts/wave0_bootstrap_check.sh"),
        Task::Review => run_script("scripts/conductor_phase_gate.sh"),
        Task::ValidateSchemas => run_script("scripts/validate_schemas.py"),
    }
}

fn run_script(path: &str) -> Result<()> {
    let status = if path.ends_with(".py") {
        Command::new("python3").arg(path).status()?
    } else {
        Command::new("bash").arg(path).status()?
    };
    if !status.success() {
        bail!("{path} failed with status {status}");
    }
    Ok(())
}
