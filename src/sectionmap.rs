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
    /// # use std::io::Read;
    /// # use std::path::Path;
    /// # use std::collections::BTreeMap;
    /// # use clog::{Clog, Writer, Markdown, SectionMap};
    /// let clog = Clog::new().unwrap_or_else(|e| { 
    ///     e.exit();
    /// });
    ///
    /// // Get the commits we're interested in...
    /// let sm = SectionMap::from_commits(clog.get_commits());
    ///
    /// // Open and prepend, or create the changelog file...
    /// let mut contents = String::new();
    /// if let Some(ref file) = clog.changelog {
    ///     File::open(file).map(|mut f| f.read_to_string(&mut contents).ok()).ok();
    ///     let mut file = File::create(file).ok().unwrap();
    ///
    ///     // Write the header...
    ///     let mut writer = Markdown::new(&mut file, &clog);
    ///     writer.write_header().ok().expect("failed to write header");
    ///
    ///     // Write the sections
    ///     for (sec, secmap) in sm.sections {
    ///         writer.write_section(&sec[..], &secmap.iter().collect::<BTreeMap<_,_>>()).ok().expect(&format!("failed to write {}", sec)[..]);
    ///     }
    ///
    ///     // Write old changelog data last
    ///     writer.write(&contents[..]).ok().expect("failed to write contents");
    /// }
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
