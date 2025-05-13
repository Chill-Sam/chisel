// SPDX-License-Identifier: Apache-2.0

use anyhow::{Context, Result};
use clap::Parser;
use editor_core::Buffer;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(name = "chisel", version)]
struct Cli {
    /// File to open
    #[clap(value_parser)]
    file: PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    // Attempt to load the file; on error, show context and exit.
    let buffer = Buffer::from_file(&args.file)
        .with_context(|| format!("failed to open '{}'", args.file.display()))?;

    // For now: just print the first few lines to verify.
    for line in buffer.lines.iter().take(10) {
        println!("{}", line);
    }

    Ok(())
}
