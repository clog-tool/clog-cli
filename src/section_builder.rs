use std::collections::hashmap::HashMap;
use common::{ LogEntry, SectionMap, Feature, Fix };

pub fn build_sections(log_entries: Vec<LogEntry>) -> SectionMap {

    let mut sections = SectionMap {
        features: HashMap::new(),
        fixes: HashMap::new(),
        breaks: HashMap::new()
    };

    log_entries.iter().all(|entry| {

        match entry.commit_type {
            Feature => {
                sections.features
                        .find_or_insert(entry.component.clone(), Vec::new())
                        .push(entry.clone());
            },
            Fix => {
                sections.fixes
                        .find_or_insert(entry.component.clone(), Vec::new())
                        .push(entry.clone());
            },
            _   => {}
        }

        true
    });

    sections
}