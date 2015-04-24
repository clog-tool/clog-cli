#![crate_name = "clog"]

extern crate regex;
extern crate time;
extern crate semver;

#[macro_use]
extern crate clap;

use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::borrow::ToOwned;

use common::CommitType;
use git::LogReaderConfig;
use log_writer::{ LogWriter, LogWriterOptions };

use clap::{App, Arg};

#[macro_use]
mod macros;
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
                          --from=[from]                 'e.g. 12a8546'
                          --major                       'Increment major version by one (Sets minor and patch to 0)'
                          --minor                       'Increment minor version by one (Sets patch to 0)'
                          --patch                       'Increment patch version by one'
                          --subtitle=[subtitle]         'e.g. crazy-release-title'
                          --to=[to]                     'e.g. 8057684 (Defaults to HEAD when omitted)'")
        // Because --from-latest-tag can't be used with --from, we add it seperately so we can
        // specify a .mutually_excludes()
        .arg(Arg::from_usage("--from-latest-tag 'use latest tag as start (instead of --from)'")
                .mutually_excludes("from"))
        .arg(Arg::from_usage("--setversion=[setversion]     'e.g. 1.0.1'")
                .mutually_excludes_all(vec!["major", "minor", "patch"]))
        .get_matches();

    let start_nsec = time::get_time().nsec;

    let log_reader_config = LogReaderConfig {
        grep: format!("{}BREAKING'", CommitType::all_aliases().iter().fold(String::new(),|acc, al| acc + &format!("^{}|", al)[..])),
        format: "%H%n%s%n%b%n==END==".to_owned(),
        from: if matches.is_present("from-latest-tag") { Some(git::get_latest_tag()) } else { matches.value_of("from").map(|v| v.to_owned()) },
        to: matches.value_of("to").unwrap_or("").to_owned()
    };

    // compute version early, so we can exit on error
    let version =  {
        // less typing later...
        let (major, minor, patch) = (matches.is_present("major"), matches.is_present("minor"), matches.is_present("patch"));
        if matches.is_present("setversion") {
            matches.value_of("setversion").unwrap().to_owned()
        } else if major || minor || patch {
            match git::get_latest_tag_ver() {
                Ok(ref mut v) => {
                    // if-else may be quicker, but it's longer mentally, and this isn't slow
                    match (major, minor, patch) {
                        (true,_,_) => { v.major += 1; v.minor = 0; v.patch = 0; },
                        (_,true,_) => { v.minor += 1; v.patch = 0; },
                        (_,_,true) => { v.patch += 1; },
                        _          => unreachable!()
                    }
                    format!("{}", v)
                },
                Err(e) => {
                    println!("Error parsing latest version: {}\nTry setting the version manually with --setversion=[version]", e );
                    std::process::exit(1);
                }
            }
        } else {           
            format_util::get_short_hash(&git::get_last_commit()[..]).to_owned()
        }
    };

    let commits = git::get_log_entries(log_reader_config);

    let sections = section_builder::build_sections(commits.clone());

    let mut contents = String::new();

    File::open(&Path::new("changelog.md")).map(|mut f| f.read_to_string(&mut contents).ok()).ok();

    let mut file = File::create(&Path::new("changelog.md")).ok().unwrap();
    let mut writer = LogWriter::new(&mut file, LogWriterOptions {
        repository_link: matches.value_of("repository").unwrap_or(""),
        version: version,
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
