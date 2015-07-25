use std::collections::HashMap;
use std::collections::BTreeMap;

use git::Commit;

/// The second level of the changelog, i.e. the components -> commit information
pub type ComponentMap = BTreeMap<String, Vec<Commit>>;

/// A struct which holds sections to and components->commits map
pub struct SectionMap {
    /// The top level map of the changelog, i.e. sections -> components
    pub sections: HashMap<String, ComponentMap>
}

impl SectionMap {
    /// Creates a section map from a vector of commits, which we can then iterate through and write
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use std::fs::File;
    /// # use clog::{Clog, SectionMap};
    /// # use clog::fmt::{FormatWriter, MarkdownWriter};
    /// let clog = Clog::new().unwrap_or_else(|e| { 
    ///     e.exit();
    /// });
    ///
    /// // Get the commits we're interested in...
    /// let sm = SectionMap::from_commits(clog.get_commits());
    ///
    /// // Create a file to hold our results, which the MardownWriter will wrap (note, .unwrap() is only
    /// // used to keep the example short and concise)
    /// let mut file = File::create("my_changelog.md").ok().unwrap();
    ///
    /// // Create the MarkdownWriter
    /// let mut writer = MarkdownWriter::new(&mut file);
    /// 
    /// // Use the MarkdownWriter to write the changelog
    /// clog.write_changelog_with(&mut writer).unwrap_or_else(|e| { 
    ///     e.exit();
    /// });
    /// ```
    pub fn from_commits(commits: Vec<Commit>) -> SectionMap {
        let mut sm = SectionMap {
            sections: HashMap::new()
        };

        commits.into_iter().map(|entry| {
            let comp_map = sm.sections.entry(entry.commit_type.clone()).or_insert(BTreeMap::new());
            let sec_map = comp_map.entry(entry.component.clone()).or_insert(vec![]);
            sec_map.push(entry);
        }).collect::<Vec<_>>();

        sm
    }
}
