use std::borrow::Cow;
use std::collections::HashMap;
use std::convert::AsRef;
use std::env;
use std::fmt::Display;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process::Command;

use clap::ArgMatches;
use toml::{Value, Parser};
use semver;

use git::{self, Commits, Commit};
use CLOG_CONFIG_FILE;

arg_enum!{
    pub enum LinkStyle {
        Github,
        Gitlab,
        Stash
    }
}

impl LinkStyle {
    pub fn issue_link<S: AsRef<str>>(&self, issue: S, repo: S) -> String {
        match repo.as_ref() {
            "" => format!("(#{})", issue.as_ref()),
            link => {
                match *self {
                    LinkStyle::Github => format!("[#{}]({}/issues/{})", issue.as_ref(), link, issue.as_ref()),
                    LinkStyle::Gitlab => format!("[#{}]({}/issues/{})", issue.as_ref(), link, issue.as_ref()),
                    LinkStyle::Stash  => format!("(#{})", issue.as_ref()) // Stash doesn't support issue links
                }
            }
        }
    }

    pub fn commit_link<S: AsRef<str>>(&self, hash: S, repo: S) -> String {
        let short_hash = &hash.as_ref()[0..8];
        match repo.as_ref() {
            "" => format!("({})", short_hash),
            link => {
                match *self {
                    LinkStyle::Github => format!("[{}]({}/commit/{})", short_hash, link, hash.as_ref()),
                    LinkStyle::Gitlab => format!("[{}]({}/commit/{})", short_hash, link, hash.as_ref()),
                    LinkStyle::Stash  => format!("[{}]({}/commits/{})", short_hash, link, hash.as_ref())
                }
            }
        }
    }
}

pub struct Clog {
    pub grep: String,
    pub format: String,
    pub repo: String,
    pub link_style: LinkStyle,
    pub version: String,
    pub patch_ver: bool,
    pub subtitle: String,
    pub from: String,
    pub to: String,
    pub changelog: String,
    pub section_map: HashMap<String, Vec<String>>
}

pub type ClogResult = Result<Clog, Box<Display>>;

impl Clog {
    pub fn new() -> Clog {
        let mut sections = HashMap::new();
        sections.insert("Features".to_owned(), vec!["ft".to_owned(), "feat".to_owned()]);
        sections.insert("Bug Fixes".to_owned(), vec!["fx".to_owned(), "fix".to_owned()]);
        sections.insert("Unknown".to_owned(), vec!["unk".to_owned()]);
        sections.insert("Breaks".to_owned(), vec![]);

        Clog {
            grep: format!("{}BREAKING'",
                sections.values()
                        .map(|v| v.iter().fold(String::new(), |acc, al| {
                            acc + &format!("^{}|", al)[..]
                        }))
                        .fold(String::new(), |acc, al| {
                            acc + &format!("^{}|", al)[..]
                        })),
            format: "%H%n%s%n%b%n==END==".to_owned(),
            repo: "".to_owned(),
            link_style: LinkStyle::Github,
            version: (&git::get_last_commit()[0..8]).to_owned(),
            patch_ver: false,
            subtitle: "".to_owned(),
            from: "".to_owned(),
            to: "HEAD".to_owned(),
            changelog: "changelog.md".to_owned(),
            section_map: sections
        }
    }

    pub fn from_file<P: AsRef<Path>>(file: P) -> ClogResult {
        let mut clog = Clog::new();
        let cfg_file = if file.as_ref().is_relative() {
            let cwd = match env::current_dir() {
                Ok(d)  => d,
                Err(e) => return Err(Box::new(e)),
            };
            Path::new(&cwd).join(file.as_ref())
        } else {
            file.as_ref().to_path_buf()
        };

        let mut toml_from_latest = None;
        let mut toml_repo = None;
        let mut toml_subtitle = None;
        let mut toml_link_style = None;
        let mut toml_outfile = None;

        if let Ok(ref mut toml_f) = File::open(&cfg_file) {
            let mut toml_s = String::with_capacity(100);

            if let Err(e) = toml_f.read_to_string(&mut toml_s) {
                return Err(Box::new(e))
            }

            toml_s.shrink_to_fit();

            let mut toml = Parser::new(&toml_s[..]);

            let toml_table = match toml.parse() {
                Some(table) => table,
                None        => {
                    return Err(Box::new(format!("Error parsing file: {}\n\nPlease check the format or specify the options manually", cfg_file.into_os_string().into_string().ok().unwrap_or("UNABLE TO DISPLAY".to_owned()))))
                }
            };

            let clog_table = match toml_table.get("clog") {
                Some(table) => table,
                None        => {
                    return Err(Box::new(format!("Error parsing file {}\n\nPlease check the format or specify the options manually", cfg_file.into_os_string().into_string().ok().unwrap_or("UNABLE TO DISPLAY".to_owned()))))
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
            toml_link_style = match clog_table.lookup("link-style") {
                Some(val) => match val.as_str().unwrap_or("github").parse::<LinkStyle>() {
                    Ok(style) => Some(style),
                    Err(err)   => {
                        return Err(Box::new(format!("Error parsing file {}\n\n{}", cfg_file.into_os_string().into_string().ok().unwrap_or("UNABLE TO DISPLAY".to_owned()), err)))
                    }
                },
                None      => Some(LinkStyle::Github)
            };
            toml_outfile = match clog_table.lookup("outfile") {
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
                                    clog.section_map.insert(sec.to_owned(), alias_vec);
                                }
                            }
                        },
                        None        => ()
                    }
                },
                None        => ()
            };
        };

        if toml_from_latest.unwrap_or(false) {
            clog.from = git::get_latest_tag();
        }

        if let Some(repo) = toml_repo {
            clog.repo = repo;
        }

        if let Some(ls) = toml_link_style {
            clog.link_style = ls;
        }

        if let Some(subtitle) = toml_subtitle {
            clog.subtitle = subtitle;
        }

        if let Some(outfile) = toml_outfile {
            clog.changelog = outfile;
        }

        Ok(clog)
    }

    pub fn from_matches(matches: &ArgMatches) -> ClogResult {
        let mut clog = if let Some(cfg_file) = matches.value_of("config") {
            try!(Clog::from_file(Path::new(cfg_file)))
        } else {
            try!(Clog::from_file(Path::new(CLOG_CONFIG_FILE)))
        };

        // compute version early, so we can exit on error
        clog.version = {
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
                            (_,_,true) => { v.patch += 1; clog.patch_ver = true; },
                            _          => unreachable!()
                        }
                        format!("{}{}", if had_v{"v"}else{""}, v)
                    },
                    Err(e) => {
                        return Err(Box::new(format!("Error: {}\n\n\tEnsure the tag format follows Semantic Versioning such as N.N.N\n\tor set the version manually with --setversion <version>" , e )));
                    }
                }
            } else {
                clog.version
            }
        };

        if let Some(from) = matches.value_of("from") {
            clog.from = from.to_owned();
        } else if matches.is_present("from-latest-tag") {
            clog.from = git::get_latest_tag();
        }

        if let Some(repo) = matches.value_of("repo") {
            clog.repo = repo.to_owned();
        }

        if matches.is_present("link-style") {
            clog.link_style = value_t!(matches.value_of("link-style"), LinkStyle).unwrap_or(LinkStyle::Github);
        } 

        if let Some(subtitle) = matches.value_of("subtitle") {
            clog.subtitle = subtitle.to_owned();
        }

        if let Some(file) = matches.value_of("outfile") {
            clog.changelog = file.to_owned();
        }

        Ok(clog)
    }

    pub fn grep<'a>(mut self, g: Cow<'a, str>) -> Clog {
        self.grep = g.into_owned();
        self
    }

    pub fn format<'a>(mut self, f: Cow<'a, str>) -> Clog {
        self.format = f.into_owned();
        self
    }

    pub fn repository<'a>(mut self, r: Cow<'a, str>) -> Clog {
        self.repo = r.into_owned();
        self
    }

    pub fn link_style<'a>(mut self, l: LinkStyle) -> Clog {
        self.link_style = l;
        self
    }

    pub fn version<'a>(mut self, v: Cow<'a, str>) -> Clog {
        self.version = v.into_owned();
        self
    }

    pub fn subtitle<'a>(mut self, s: Cow<'a, str>) -> Clog {
        self.subtitle = s.into_owned();
        self
    }

    pub fn from<'a>(mut self, f: Cow<'a, str>) -> Clog {
        self.from = f.into_owned();
        self
    }

    pub fn to<'a>(mut self, t: Cow<'a, str>) -> Clog {
        self.to = t.into_owned();
        self
    }

    pub fn changelog<'a>(mut self, c: Cow<'a, str>) -> Clog {
        self.changelog = c.into_owned();
        self
    }

    pub fn patch_version<'a>(mut self, p: bool) -> Clog {
        self.patch_ver = p;
        self
    }

    pub fn get_log_entries(&self) -> Commits {
        let range = match &self.from[..] {
            "" => "HEAD".to_owned(),
            _  => format!("{}..{}", self.from, self.to)
        };

        let output = Command::new("git")
                .arg("log")
                .arg("-E")
                .arg(&format!("--grep={}", self.grep))
                .arg(&format!("--format={}", self.format))
                .arg(&range)
                .output().unwrap_or_else(|e| panic!("Failed to run 'git log' with error: {}", e));

        String::from_utf8_lossy(&output.stdout)
                .split("\n==END==\n")
                .map(|commit_str| { self.parse_raw_commit(commit_str) })
                .filter(| entry| entry.commit_type != "Unknown")
                .collect()
    }

    fn parse_raw_commit(&self, commit_str:&str) -> Commit {
        let mut lines = commit_str.split('\n');

        let hash = lines.next().unwrap_or("").to_owned();

        let commit_pattern = regex!(r"^(.*?)(?:\((.*)?\))?:(.*)");
        let (subject, component, commit_type) =
            match lines.next().and_then(|s| commit_pattern.captures(s)) {
                Some(caps) => {
                    let commit_type = self.section_for(caps.at(1).unwrap_or("")).to_owned();
                    let component = caps.at(2);
                    let subject = caps.at(3);
                    (subject, component, commit_type)
               },
               None => (Some(""), Some(""), self.section_for("unk").clone())
            };
        let closes_pattern = regex!(r"(?:Closes|Fixes|Resolves)\s((?:#(\d+)(?:,\s)?)+)");
        let closes = lines.filter_map(|line| closes_pattern.captures(line))
                          .map(|caps| caps.at(2).unwrap_or("").to_owned())
                          .collect();

        Commit {
            hash: hash,
            subject: subject.unwrap().to_owned(),
            component: component.unwrap_or("").to_owned(),
            closes: closes,
            breaks: vec![],
            commit_type: commit_type
        }
    }

    pub fn section_for(&self, alias: &str) -> &String {
        self.section_map.iter().filter(|&(_, v)| v.iter().any(|s| s == alias)).map(|(k, _)| k).next().unwrap_or(self.section_map.keys().filter(|&k| *k == "Unknown".to_owned()).next().unwrap())
    }
}
