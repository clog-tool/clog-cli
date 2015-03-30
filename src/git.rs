use std::process::Command;
use std::io::Read;
use regex::Regex;
use common:: { LogEntry };
use common::CommitType;
use std::borrow::ToOwned;

#[derive(Debug)]
pub struct LogReaderConfig {
    pub grep: String,
    pub format: String,
    pub from: Option<String>,
    pub to: String
}

pub fn get_latest_tag () -> String {
    let mut buf = String::new();
    Command::new("git")
            .arg("rev-list")
            .arg("--tags")
            .arg("--max-count=1")
            .spawn()
            .ok().expect("failed to invoke ref-list")
            .stdout.as_mut().unwrap().read_to_string(&mut buf)
            .ok().expect("failed to get latest git log");

            buf
            .trim_matches('\n')
            .to_owned()
}

pub fn get_last_commit () -> String {
    let mut buf = String::new();
    Command::new("git")
            .arg("rev-parse")
            .arg("HEAD")
            .spawn()
            .ok().expect("failed to invoke rev-parse")
            .stdout.as_mut().unwrap().read_to_string(&mut buf)
            .ok().expect("failed to get last commit");
    buf
}

pub fn get_log_entries (config:LogReaderConfig) -> Vec<LogEntry>{

    let range = match config.from {
        Some(ref from) => format!("{}..{}", from, config.to),
        None => "HEAD".to_owned()
    };

    let mut buf = String::new();

    Command::new("git")
            .arg("log")
            .arg("-E")
            .arg(&format!("--grep={}",config.grep))
            .arg(&format!("--format={}", "%H%n%s%n%b%n==END=="))
            .arg(&range)
            .spawn()
            .ok().expect("failed to invoke `git log`")
            .stdout.as_mut().unwrap().read_to_string(&mut buf)
            .ok().expect("failed to read git log");

            buf
            .split("\n==END==\n")
            .map(|commit_str| { parse_raw_commit(commit_str) })
            .filter(| entry| entry.commit_type != CommitType::Unknown)
            .collect()
}

static COMMIT_PATTERN: Regex = regex!(r"^(.*)\((.*)\):(.*)");
static CLOSES_PATTERN: Regex = regex!(r"(?:Closes|Fixes|Resolves)\s((?:#(\d+)(?:,\s)?)+)");

fn parse_raw_commit(commit_str:&str) -> LogEntry {
    let mut lines = commit_str.split('\n');

    let hash = lines.next().unwrap_or("").to_owned();

    let (subject, component, commit_type) =
        match lines.next().and_then(|s| COMMIT_PATTERN.captures(s)) {
            Some(caps) => {
                let commit_type = match caps.at(1) {
                    Some("feat") => CommitType::Feature,
                    Some("fix")  => CommitType::Fix,
                    _            => CommitType::Unknown
                };
                let component = caps.at(2);
                let subject = caps.at(3);
                (subject, component, commit_type)
           },
           None => (Some(""), Some(""), CommitType::Unknown)
        };
    let closes = lines.filter_map(|line| CLOSES_PATTERN.captures(line))
                      .map(|caps| caps.at(2).unwrap().to_owned())
                      .collect();

    LogEntry {
        hash: hash,
        subject: subject.unwrap().to_owned(),
        component: component.unwrap().to_owned(),
        closes: closes,
        breaks: vec![],
        commit_type: commit_type
    }
}
