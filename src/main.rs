#![crate_name = "clog"]

extern crate regex;
extern crate time;

#[macro_use]
extern crate clap;

use git::LogReaderConfig;
use log_writer::{ LogWriter, LogWriterOptions };
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::borrow::ToOwned;

use clap::{App, Arg};

// regex cheat thanks to https://github.com/BurntSushi
macro_rules! regex(
    ($s:expr) => (::regex::Regex::new($s).unwrap());
);

mod common;
mod git;
mod log_writer;
mod section_builder;
mod format_util;

fn main () {
    let matches = App::new("clog")
        // Pull version from Cargo.toml
        .version(&crate_version!()[..])
        .about("a conventional changelog for the rest of us")
        .args_from_usage("-r --repository=[repository]  'e.g. https://github.com/thoughtram/clog'
                          --setversion=[setversion]     'e.g. 1.0.1'
                          --from=[from]                 'e.g. 12a8546'
                          --subtitle=[subtitle]         'e.g. crazy-release-title'
                          --to=[to]                     'e.g. 8057684 (Defaults to HEAD when omitted)'")
        // Because --from-latest-tag can't be used with --from, we add it seperately so we can
        // specify a .mutually_excludes()
        .arg(Arg::from_usage("--from-latest-tag 'use latest tag as start (instead of --from)'")
                .mutually_excludes("from"))
        .get_matches();

    let start_nsec = time::get_time().nsec;

    let log_reader_config = LogReaderConfig {
        grep: "^feat|^fix|BREAKING'".to_owned(),
        format: "%H%n%s%n%b%n==END==".to_owned(),
        from: if matches.is_present("from-latest-tag") { Some(git::get_latest_tag()) } else { matches.value_of("from").map(|v| v.to_owned()) },
        to: matches.value_of("to").unwrap_or("").to_owned()
    };

    let commits = git::get_log_entries(log_reader_config);

    let sections = section_builder::build_sections(commits.clone());

    let mut contents = String::new();

    File::open(&Path::new("changelog.md")).map(|mut f| f.read_to_string(&mut contents).ok()).ok();

    let mut file = File::create(&Path::new("changelog.md")).ok().unwrap();
    let mut writer = LogWriter::new(&mut file, LogWriterOptions {
        repository_link: matches.value_of("repository").unwrap_or(""),
        version: if matches.is_present("setversion") {
                    matches.value_of("setversion").unwrap().to_owned()
                } else {
                    format_util::get_short_hash(&git::get_last_commit()[..]).to_owned()
                },
        subtitle: matches.value_of("subtitle").unwrap_or("").to_owned()
    });

    writer.write_header().ok().expect("failed to write header");
    writer.write_section("Bug Fixes", &sections.fixes).ok().expect("failed to write bugfixes");;
    writer.write_section("Features", &sections.features).ok().expect("failed to write features");;
    writer.write(&contents[..]).ok().expect("failed to write contents");;

    let end_nsec = time::get_time().nsec;
    let elapsed_mssec = (end_nsec - start_nsec) / 1000000;
    println!("changelog updated. (took {} ms)", elapsed_mssec);
}
