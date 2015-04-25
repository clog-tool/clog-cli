use std::collections::HashMap;
use common::{ LogEntry, SectionMap };
use common::CommitType::{ Feature, Fix };

pub fn build_sections(log_entries: Vec<LogEntry>) -> SectionMap {
    let mut sections = SectionMap {
        features: HashMap::new(),
        fixes: HashMap::new(),
        breaks: HashMap::new()
    };

    // see https://github.com/rust-lang/rfcs/issues/353
    //     sections.features
    //             .find_or_insert(entry.component.clone(), Vec::new())
    //             .push(entry.clone());
    log_entries.into_iter().map(|entry| {
        match entry.commit_type {
            Feature => {
                let feature = sections.features.entry(entry.component.clone()).or_insert(vec![]);
                feature.push(entry);
            },
            Fix     => {
                let fix = sections.fixes.entry(entry.component.clone()).or_insert(vec![]);
                fix.push(entry);
            },
            _ => (),
        }
    }).collect::<Vec<_>>();

    sections
}
