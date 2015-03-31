use std::collections::HashMap;
use std::collections::hash_map::Entry:: { Occupied, Vacant };
use common::{ LogEntry, SectionMap };
use common::CommitType::{ Feature, Fix };

pub fn build_sections(log_entries: Vec<LogEntry>) -> SectionMap {
    let mut sections = SectionMap {
        features: HashMap::new(),
        fixes: HashMap::new(),
        breaks: HashMap::new()
    };

    for entry in log_entries.into_iter() {
        match entry.commit_type {
            Feature => {
                let feature = match sections.features.entry(entry.component.clone()) {
                    Vacant(v) => v.insert(Vec::new()),
                    Occupied(o) => o.into_mut()
                };

                feature.push(entry.clone());

                // see https://github.com/rust-lang/rfcs/issues/353
                /* sections.features
                        .find_or_insert(entry.component.clone(), Vec::new())
                        .push(entry.clone());*/
            },
            Fix => {
                let fix = match sections.fixes.entry(entry.component.clone()) {
                    Vacant(v) => v.insert(Vec::new()),
                    Occupied(o) => o.into_mut()
                };

                fix.push(entry.clone());

                /* sections.fixes
                        .find_or_insert(entry.component.clone(), Vec::new())
                        .push(entry.clone());*/
            },
            _   => {}
        }
    }

    sections
}
