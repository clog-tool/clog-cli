use std::fmt;
use std::collections::HashMap;

// Creates an enum where the poritions inside the '(' and ')' act as aliases for that
// commit type. This macro auto-generates an "Unknown" variant for failures, no  need to specify
//
// Later you can call CommitType::Fix.aliases() to get all the aliases as a Vec<'static str>
// or CommitType::all_aliases() to get a Vec<'static str> of all aliases
// This macro also implements std::str::FromStr to allow things like "feat".parse<CommitType>();
commit_type_enum!{
    #[derive(Debug, PartialEq, Clone)]
    pub enum CommitType {
        Feature ( feat, ft ),
        Fix ( fix, fx )
    }
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
