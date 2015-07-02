// Until regex_macros compiles with nightly, these should be commented out
//
// #![cfg_attr(feature = "unstable", feature(plugin))]
// #![cfg_attr(feature = "unstable", plugin(regex_macros))]

// DOCS

extern crate regex;
extern crate semver;
extern crate toml;
#[macro_use]
extern crate clap;
extern crate time;

#[macro_use]
mod macros;
pub mod git;
mod log_writer;
mod sectionmap;
mod clog;

pub use clog::{Clog, LinkStyle};
pub use log_writer::LogWriter;
pub use sectionmap::SectionMap;

// The default config file
const CLOG_CONFIG_FILE: &'static str = ".clog.toml";