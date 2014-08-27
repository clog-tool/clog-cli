#![crate_name = "clog"]
#![comment = "A conventional changelog generator"]
#![license = "MIT"]
#![feature(macro_rules, phase)]

extern crate regex;
#[phase(plugin)]
extern crate regex_macros;

use git::{ LogReaderConfig, get_commits };
use log_writer::{ LogWriter, LogWriterOptions };
use section_builder::build_sections;
use std::io::{File, Open, Write};

mod common;
mod git;
mod log_writer;
mod section_builder;

fn main () {

    let log_reader_config = LogReaderConfig {
        grep: "^feat|^fix|BREAKING'".to_string(),
        format: "%H%n%s%n%b%n==END==".to_string(),
        from: "".to_string(),
        to: "HEAD".to_string()
    };
    let commits = get_commits(log_reader_config);

    let sections = build_sections(commits.clone());
    let mut file = File::open_mode(&Path::new("changelog.md"), Open, Write).ok().unwrap();
    let mut writer = LogWriter::new(&mut file, LogWriterOptions { 
        repository_link: "https://github.com/ajoslin/conventional-changelog".to_string() 
    });

    writer.write_section("Bug Fixes", &sections.fixes);
    writer.write_section("Features", &sections.features);
    //println!("{}", commits);
}
