use std::fmt;

/// The struct representation of a `Commit`
#[derive(Clone)]
pub struct Commit {
    /// The 40 char hash
    pub hash: String,
    /// The commit subject
    pub subject: String,
    /// The component (if any)
    pub component: String,
    /// Any issues this commit closes
    pub closes: Vec<String>,
    /// Any issues this commit breaks
    pub breaks: Vec<String>,
    /// The commit type (or alias)
    pub commit_type: String 
}

/// A convienience type for multiple commits
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
