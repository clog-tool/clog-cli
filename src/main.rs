#[macro_use]
mod macros;
mod cli;
mod error;
mod fmt;

use std::time::Instant;

use clap::Parser;

use crate::error::CliResult;

const DEFAULT_CONFIG_FILE: &str = ".clog.toml";

fn try_main() -> CliResult<()> {
    let args = cli::Args::parse();
    let clog = args.into_clog().unwrap_or_else(|e| e.exit());

    if let Some(file) = &clog.outfile {
        clog.write_changelog_to(file)?;
    } else {
        clog.write_changelog()?;
    }
    Ok(())
}

fn main() {
    let start = Instant::now();
    if let Err(e) = try_main() {
        e.exit();
    }
    let elapsed = start.elapsed();
    println!("changelog written. (took {} ms)", elapsed.as_millis());
}
