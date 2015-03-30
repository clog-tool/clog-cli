#![crate_name = "clog"]
#![feature(macro_rules)]
#![feature(plugin)]
#![plugin(docopt_macros)]
#![plugin(regex_macros)]

extern crate "rustc-serialize" as rustc_serialize;
extern crate regex;
extern crate regex_macros;
extern crate serialize;
extern crate docopt_macros;
extern crate docopt;
extern crate time;

use git::LogReaderConfig;
use log_writer::{ LogWriter, LogWriterOptions };
use std::fs::File;

mod common;
mod git;
mod log_writer;
mod section_builder;
mod format_util;

docopt!(Args, "clog

Usage:
  clog [--repository=<link> --setversion=<version> --subtitle=<subtitle>]
       [--from=<from> --to=<to> --from-latest-tag]

Options:
  -h --help               Show this screen.
  --version               Show version
  -r --repository=<link>  e.g https://github.com/thoughtram/clog
  --setversion=<version>  e.g. 0.1.0
  --subtitle=<subtitle>   e.g. crazy-release-name
  --from=<from>           e.g. 12a8546
  --to=<to>               e.g. 8057684
  --from-latest-tag       uses the latest tag as starting point. Ignores other --from parameter",
  flag_from: Option<String>,
  flag_setversion: Option<String>);

fn main () {

    let start_nsec = time::get_time().nsec;
    let args: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit());

    let log_reader_config = LogReaderConfig {
        grep: "^feat|^fix|BREAKING'".to_string(),
        format: "%H%n%s%n%b%n==END==".to_string(),
        from: if args.flag_from_latest_tag { Some(git::get_latest_tag()) } else { args.flag_from },
        to: args.flag_to
    };

    let commits = git::get_log_entries(log_reader_config);

    let sections = section_builder::build_sections(commits.clone());

    let contents = match File::open(&Path::new("changelog.md")).read_to_string() {
      Ok(content) => content,
      Err(_)      => "".to_string()
    };

    let mut file = File::create(&Path::new("changelog.md")).ok().unwrap();
    let mut writer = LogWriter::new(&mut file, LogWriterOptions {
        repository_link: args.flag_repository,
        version: args.flag_setversion
                     .unwrap_or_else(|| format_util::get_short_hash(git::get_last_commit().as_slice()).to_string()),
        subtitle: args.flag_subtitle
    });

    writer.write_header().ok().expect("failed to write header");
    writer.write_section("Bug Fixes", &sections.fixes).ok().expect("failed to write bugfixes");;
    writer.write_section("Features", &sections.features).ok().expect("failed to write features");;
    writer.write(contents.as_slice()).ok().expect("failed to write contents");;

    let end_nsec = time::get_time().nsec;
    let elapsed_mssec = (end_nsec - start_nsec) / 1000000;
    println!("changelog updated. (took {} ms)", elapsed_mssec);
}
