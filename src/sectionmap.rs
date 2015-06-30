use std::collections::HashMap;
use std::collections::BTreeMap;

use git::Commit;

pub type ComponentMap = BTreeMap<String, Vec<Commit>>;

pub struct SectionMap {
    pub sections: HashMap<String, ComponentMap>
}

impl SectionMap {
    pub fn from_commits(log_entries: Vec<Commit>) -> SectionMap {
        let mut sm = SectionMap {
            sections: HashMap::new()
        };

        log_entries.into_iter().map(|entry| {
            let comp_map = sm.sections.entry(entry.commit_type.clone()).or_insert(BTreeMap::new());
            let sec_map = comp_map.entry(entry.component.clone()).or_insert(vec![]);
            sec_map.push(entry);
        }).collect::<Vec<_>>();

        sm
    }
}
