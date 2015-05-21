use std::collections::HashMap;
use std::collections::BTreeMap;

use logentry::LogEntry;

pub type ComponentMap<'a> = BTreeMap<String, Vec<LogEntry<'a>>>;

pub struct SectionMap<'a> {
    pub sections: HashMap<String, ComponentMap<'a>>
}

impl<'a> SectionMap<'a> {
    pub fn from_entries(log_entries: Vec<LogEntry>) -> SectionMap {
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
