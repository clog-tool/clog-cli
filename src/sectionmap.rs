use std::collections::HashMap;
use std::collections::BTreeMap;

use git::Commit;

/// A convenience type for a map of components to commits 
pub type ComponentMap = BTreeMap<String, Vec<Commit>>;

/// A struct which holds sections to and components->commits map
pub struct SectionMap {
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
    /// # use clog::{Clog, LogWriter, SectionMap};
    /// let clog = Clog::new().unwrap_or_else(|e| { 
    ///     println!("Error initializing: {}", e);
    ///     std::process::exit(1);
    /// });
    ///
    /// // Get the commits we're interested in and create the section map...
    /// let sm = SectionMap::from_commits(clog.get_commits());
    ///
    /// // Open and prepend, or create the changelog file...
    /// let mut contents = String::new();
    /// File::open(&Path::new(&clog.changelog[..])).map(|mut f| f.read_to_string(&mut contents).ok()).ok();
    /// let mut file = File::create(&Path::new(&clog.changelog[..])).ok().unwrap();
    ///
    /// // Write the header...
    /// let mut writer = LogWriter::new(&mut file, &clog);
    /// writer.write_header().ok().expect("failed to write header");
    ///
    /// // Write the sections
    /// for (sec, secmap) in sm.sections {
    ///    writer.write_section(&sec[..], &secmap.iter()
    ///                                          .collect::<BTreeMap<_,_>>())
    ///          .ok()
    ///          .expect(&format!("failed to write {}", sec)[..]);
    /// }
    /// writer.write(&contents[..]).ok().expect("failed to write contents");
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
