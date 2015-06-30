use std::fmt;
use std::process::Command;

#[derive(Clone)]
pub struct Commit {
    pub hash: String,
    pub subject: String,
    pub component: String,
    pub closes: Vec<String>,
    pub breaks: Vec<String>,
    pub commit_type: String 
}

pub type Commits = Vec<Commit>;

impl fmt::Debug for Commit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{
            hash:{:?},
            subject: {:?},
            commit_type: {:?},
            component: {:?},
            closes: {:?},
            breaks: {:?}
        }}", self.hash, self.subject, self.commit_type, self.component, self.closes, self.breaks)
    }
}

pub fn get_latest_tag() -> String {
    let output = Command::new("git")
            .arg("rev-list")
            .arg("--tags")
            .arg("--max-count=1")
            .output().unwrap_or_else(|e| panic!("Failed to run 'git rev-list' with error: {}",e));
    let buf = String::from_utf8_lossy(&output.stdout);

    buf.trim_matches('\n').to_owned()
}

pub fn get_latest_tag_ver() -> String {
    let output = Command::new("git")
            .arg("describe")
            .arg("--tags")
            .arg("--abbrev=0")
            .output().unwrap_or_else(|e| panic!("Failed to run 'git describe' with error: {}",e));

    String::from_utf8_lossy(&output.stdout).into_owned()
}

pub fn get_last_commit() -> String {
    let output = Command::new("git")
            .arg("rev-parse")
            .arg("HEAD")
            .output().unwrap_or_else(|e| panic!("Failed to run 'git rev-parse' with error: {}", e));

    String::from_utf8_lossy(&output.stdout).into_owned()
}
