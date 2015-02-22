use std::io::Command;
use regex::Regex;
use common:: { LogEntry, Feature, Fix, Unknown };

#[derive(Show)]
pub struct LogReaderConfig {
    pub grep: String,
    pub format: String,
    pub from: Option<String>,
    pub to: String
}

pub fn get_latest_tag () -> String {

    Command::new("git")
            .arg("rev-list")
            .arg("--tags")
            .arg("--max-count=1")
            .spawn()
            .ok().expect("failed to invoke ref-list")
            .stdout.as_mut().unwrap().read_to_string()
            .ok().expect("failed to get latest git log")
            .as_slice().trim_chars('\n')
            .to_string()
}

pub fn get_last_commit () -> String {
    Command::new("git")
            .arg("rev-parse")
            .arg("HEAD")
            .spawn()
            .ok().expect("failed to invoke rev-parse")
            .stdout.as_mut().unwrap().read_to_string()
            .ok().expect("failed to get last commit")
}

pub fn get_log_entries (config:LogReaderConfig) -> Vec<LogEntry>{

    let range = match config.from {
        Some(ref from) => format!("{}..{}", from, config.to),
        None => "HEAD".to_string()
    };

    Command::new("git")
            .arg("log")
            .arg("-E")
            .arg(format!("--grep={}",config.grep))
            .arg(format!("--format={}", "%H%n%s%n%b%n==END=="))
            .arg(range)
            .spawn()
            .ok().expect("failed to invoke `git log`")
            .stdout.as_mut().unwrap().read_to_string()
            .ok().expect("failed to read git log")
            .as_slice()
            .split_str("\n==END==\n")
            .map(|commit_str| { parse_raw_commit(commit_str) })
            .filter(| entry| entry.commit_type != Unknown)
            .collect()
}

static COMMIT_PATTERN: Regex = regex!(r"^(.*)\((.*)\):(.*)");
static CLOSES_PATTERN: Regex = regex!(r"(?:Closes|Fixes|Resolves)\s((?:#(\d+)(?:,\s)?)+)");

fn parse_raw_commit(commit_str:&str) -> LogEntry {
    let mut lines = commit_str.split('\n');

    let hash = lines.next().unwrap_or("").to_string();

    let (subject, component, commit_type) =
        match lines.next().and_then(|s| COMMIT_PATTERN.captures(s)) {
            Some(caps) => {
                let commit_type = match caps.at(1) {
                    "feat" => Feature,
                    "fix"  => Fix,
                    _      => Unknown
                };
                let component = caps.at(2).to_string();
                let subject = caps.at(3).to_string();
                (subject, component, commit_type)
           },
           None => ("".to_string(), "".to_string(), Unknown)
        };
    let closes = lines.filter_map(|line| CLOSES_PATTERN.captures(line))
                      .map(|caps| caps.at(2).to_string())
                      .collect();

    LogEntry {
        hash: hash,
        subject: subject,
        component: component,
        closes: closes,
        breaks: vec!(),
        commit_type: commit_type
    }
}
