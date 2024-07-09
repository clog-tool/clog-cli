use std::time::Instant;

use clap::Parser;

#[macro_use]
mod macros;
mod cli;
mod error;
mod fmt;

const DEFAULT_CONFIG_FILE: &str = ".clog.toml";

fn main() {
    let start = Instant::now();
    let args = cli::Args::parse();

    let clog = args.into_clog().unwrap_or_else(|e| e.exit());

    if let Some(ref file) = clog.outfile {
        clog.write_changelog_to(file).unwrap_or_else(|e| e.exit());

        let elapsed = start.elapsed();
        println!("changelog written. (took {} ms)", elapsed.as_millis());
    } else {
        clog.write_changelog().unwrap_or_else(|e| e.exit());
    }
}
