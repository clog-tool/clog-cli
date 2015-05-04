use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::fmt::Display;
use std::env;
use std::collections::HashMap;

use clap::ArgMatches;
use toml::{Value, Parser};
use semver;

use git;
use CLOG_CONFIG_FILE;

pub struct ClogConfig {
    pub grep: String,
    pub format: String,
    pub repo: String,
    pub version: String,
    pub subtitle: String,
    pub from: String,
    pub to: String,
    pub changelog: String,
    pub section_map: HashMap<String, Vec<String>>
}

pub type ConfigResult = Result<ClogConfig, Box<Display>>;

impl ClogConfig {
    pub fn from_matches(matches: &ArgMatches) -> ConfigResult {
        // compute version early, so we can exit on error
        let version =  {
            // less typing later...
            let (major, minor, patch) = (matches.is_present("major"), matches.is_present("minor"), matches.is_present("patch"));
            if matches.is_present("ver") {
                matches.value_of("ver").unwrap().to_owned()
            } else if major || minor || patch {
                let mut had_v = false;
                let v_string = git::get_latest_tag_ver();
                let first_char = v_string.chars().nth(0).unwrap_or(' ');
                let v_slice = if first_char == 'v' || first_char == 'V' {
                    had_v = true;
                    v_string.trim_left_matches(|c| c == 'v' || c == 'V')
                } else {
                    &v_string[..]
                };
                match semver::Version::parse(v_slice) {
                    Ok(ref mut v) => {
                        // if-else may be quicker, but it's longer mentally, and this isn't slow
                        match (major, minor, patch) {
                            (true,_,_) => { v.major += 1; v.minor = 0; v.patch = 0; },
                            (_,true,_) => { v.minor += 1; v.patch = 0; },
                            (_,_,true) => { v.patch += 1; },
                            _          => unreachable!()
                        }
                        format!("{}{}", if had_v{"v"}else{""}, v)
                    },
                    Err(e) => {
                        return Err(Box::new(format!("Error: {}\n\n\tEnsure the tag format follows Semantic Versioning such as N.N.N\n\tor set the version manually with --setversion <version>" , e )));
                    }
                }
            } else {
                // Use short hash
                (&git::get_last_commit()[0..8]).to_owned()
            }
        };

        let cwd = match env::current_dir() {
            Ok(d)  => d,
            Err(e) => return Err(Box::new(e)),
        };

        let mut sections = HashMap::new();
        sections.insert("Features".to_owned(), vec!["ft".to_owned(), "feat".to_owned()]);
        sections.insert("Bug Fixes".to_owned(), vec!["fx".to_owned(), "fix".to_owned()]);
        sections.insert("Unknown".to_owned(), vec!["unk".to_owned()]);
        sections.insert("Breaks".to_owned(), vec![]);

        let cfg_file = Path::new(&cwd).join(CLOG_CONFIG_FILE);
        let mut toml_from_latest = None;
        let mut toml_repo = None;
        let mut toml_subtitle = None;

        let mut outfile = None;

        if let Ok(ref mut toml_f) = File::open(cfg_file){
            let mut toml_s = String::with_capacity(100);

            if let Err(e) = toml_f.read_to_string(&mut toml_s) {
                return Err(Box::new(e))
            }

            toml_s.shrink_to_fit();

            let mut toml = Parser::new(&toml_s[..]);

            let toml_table = match toml.parse() {
                Some(table) => table,
                None        => {
                    return Err(Box::new(format!("Error parsing file {}\n\nPlease check the format or specify the options manually", CLOG_CONFIG_FILE)))
                }
            };

            let clog_table = match toml_table.get("clog") {
                Some(table) => table,
                None        => {
                    return Err(Box::new(format!("Error parsing file {}\n\nPlease check the format or specify the options manually", CLOG_CONFIG_FILE)))
                }
            };

            toml_from_latest = clog_table.lookup("from-latest-tag").unwrap_or(&Value::Boolean(false)).as_bool();
            toml_repo = match clog_table.lookup("repository") {
                Some(val) => Some(val.as_str().unwrap_or("").to_owned()),
                None      => Some("".to_owned())
            };
            toml_subtitle = match clog_table.lookup("subtitle") {
                Some(val) => Some(val.as_str().unwrap_or("").to_owned()),
                None      => Some("".to_owned())
            };
            outfile = match clog_table.lookup("outfile") {
                Some(val) => Some(val.as_str().unwrap_or("changelog.md").to_owned()),
                None      => None
            };
            match toml_table.get("sections") {
                Some(table) => {
                    match table.as_table() {
                        Some(table) => {
                            for (sec, val) in table.iter() {
                                if let Some(vec) = val.as_slice() {
                                    let alias_vec = vec.iter().map(|v| v.as_str().unwrap_or("").to_owned()).collect::<Vec<_>>();
                                    sections.insert(sec.to_owned(), alias_vec);
                                }
                            }
                        },
                        None        => ()
                    }
                },
                None        => ()
            };
        };

        let from = if let Some(from) = matches.value_of("from") {
            from.to_owned()
        } else if matches.is_present("from-latest-tag") || toml_from_latest.unwrap_or(false) {
            git::get_latest_tag()
        } else {
           "".to_owned()
        };

        let repo = match matches.value_of("repository") {
            Some(repo) => repo.to_owned(),
            None       => toml_repo.unwrap_or("".to_owned())
        };

        let subtitle = match matches.value_of("subtitle") {
            Some(title) => title.to_owned(),
            None        => toml_subtitle.unwrap_or("".to_owned())
        };

        if let Some(file) = matches.value_of("outfile") {
            outfile = Some(file.to_owned());
        }

        Ok(ClogConfig{
            grep: format!("{}BREAKING'",
                sections.values()
                        .map(|v| v.iter().fold(String::new(), |acc, al| {
                            acc + &format!("^{}|", al)[..]
                        }))
                        .fold(String::new(), |acc, al| {
                            acc + &format!("^{}|", al)[..]
                        })),
            format: "%H%n%s%n%b%n==END==".to_owned(),
            repo: repo,
            version: version,
            subtitle: subtitle,
            from: from,
            to: matches.value_of("to").unwrap_or("HEAD").to_owned(),
            changelog: outfile.unwrap_or("changelog.md".to_owned()),
            section_map: sections
        })
    }

    pub fn section_for(&self, alias: &str) -> &String {
        self.section_map.iter().filter(|&(_, v)| v.iter().any(|s| s == alias)).map(|(k, _)| k).next().unwrap_or(self.section_map.keys().filter(|&k| *k == "Unknown".to_owned()).next().unwrap())
    }
}
