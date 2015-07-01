use std::fmt;

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
