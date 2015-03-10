use std::fmt;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum CommitType {
    Feature,
    Fix,
    Unknown
}

#[derive(Clone)]
pub struct LogEntry {
    pub hash: String,
    pub subject: String,
    pub component: String,
    pub closes: Vec<String>,
    pub breaks: Vec<String>,
    pub commit_type: CommitType
}

impl fmt::Debug for LogEntry {
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

pub struct SectionMap {
    pub features: HashMap<String, Vec<LogEntry>>,
    pub fixes: HashMap<String, Vec<LogEntry>>,
    pub breaks: HashMap<String, Vec<LogEntry>>
}
