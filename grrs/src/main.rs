use anyhow::{Context, Result};
use clap::Parser;
use std::{fs, path::PathBuf};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to file to search
    path: PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let content = fs::read_to_string(&args.path)
        .with_context(|| format!("could not open file `{}`", args.path.display()))?;

    grrs::find_matches(&content, &args.pattern, &mut std::io::stdout());

    Ok(())
}
