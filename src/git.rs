use std::io::Command;
use regex::Regex;
use common:: { LogEntry, Feature, Fix, Unknown };

#[deriving(Show)]
pub struct LogReaderConfig {
    pub grep: String,
    pub format: String,
    pub from: String,
    pub to: String
}

pub fn get_latest_tag () -> String {

    Command::new("git")
            .arg("rev-list")
            .arg("--tags")
            .arg("--max-count=1")
            .spawn()
            .ok().expect("failed to invoke ref-list")
            .stdout.get_mut_ref().read_to_string()
            .ok().expect("failed to get latest git log")
            .as_slice().trim_chars('\n')
            .to_string()
}

pub fn get_log_entries (config:LogReaderConfig) -> Vec<LogEntry>{

    let range = if config.from.len() == 0 {
        "HEAD".to_string()
    }
    else {
        format!("{}..{}", config.from, config.to)
    };

    Command::new("git")
            .arg("log")
            .arg("-E")
            .arg(format!("--grep={}",config.grep))
            .arg(format!("--format={}", "%H%n%s%n%b%n==END=="))
            .arg(range)
            .spawn()
            .ok().expect("failed to invoke `git log`")
            .stdout.get_mut_ref().read_to_string()
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

    let mut lines = commit_str.split('\n').collect::<Vec<&str>>();

    //println!("parsed: {}", lines);

    let hash = lines.remove(0).unwrap_or("").to_string();
    let temp_subject = lines.remove(0).unwrap_or("").to_string();

    let mut entry = LogEntry {
        hash: hash,
        subject: "".to_string(),
        component: "".to_string(),
        closes: vec!(),
        breaks: vec!(),
        commit_type: Unknown
    };

    lines.iter().all(|line| {
        CLOSES_PATTERN.captures(*line)
                      .map(|caps| {
                          entry.closes.push(caps.at(2).to_string());
                      });
        true
    });

    COMMIT_PATTERN.captures(temp_subject.as_slice())
                  .map(|caps| {
                       entry.commit_type = match caps.at(1) {
                           "feat" => Feature,
                           "fix"  => Fix,
                           _      => Unknown
                       };
                       entry.component = caps.at(2).to_string();
                       entry.subject = caps.at(3).to_string();
                  });

    entry
}