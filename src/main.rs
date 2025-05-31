use anyhow::{Context, Result};
use clap::Parser;
use grrs::find_matches;
use log::{info};
use std::fs::File;

#[derive(Parser, Debug)]
struct Cli {
    /// The pattern to search for
    pattern: String,
    /// The path to search in
    path: std::path::PathBuf,
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let args = Cli::parse();

    let file = File::open(&args.path)
        .with_context(|| format!("could not read file `{:?}`", &args.path))?;

    for line in find_matches(file, &args.pattern) {
        info!("{}", line);
    }
    Ok(())
}
