use std::fmt;
use std::collections::hashmap::HashMap;

#[deriving(Show, PartialEq, Clone)]
pub enum CommitType {
    Feature,
    Fix,
    Unknown
}

#[deriving(Clone)]
pub struct LogEntry {
    pub hash: String,
    pub subject: String,
    pub component: String,
    pub closes: Vec<String>,
    pub breaks: Vec<String>,
    pub commit_type: CommitType
}

impl fmt::Show for LogEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ 
            hash:{}, 
            subject: {},
            commit_type: {},
            component: {}
        }}", self.hash, self.subject, self.commit_type, self.component)
    }
}

pub struct SectionMap {
    pub features: HashMap<String, Vec<LogEntry>>,
    pub fixes: HashMap<String, Vec<LogEntry>>,
    pub breaks: HashMap<String, Vec<LogEntry>>
}