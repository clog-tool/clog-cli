// Until regex_macros compiles on nightly, we comment this out
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
pub mod fmt;
mod sectionmap;
mod clog;
pub mod error;

pub use clog::{Clog, LinkStyle};
pub use sectionmap::SectionMap;

// The default config file
const CLOG_CONFIG_FILE: &'static str = ".clog.toml";
