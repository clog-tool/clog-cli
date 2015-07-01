use std::collections::HashMap;
use std::convert::AsRef;
use std::env;
use std::fmt::Display;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::Command;

use clap::ArgMatches;
use toml::{Value, Parser};
use semver;

use git::{Commits, Commit};
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
    pub section_map: HashMap<String, Vec<String>>,
    pub git_dir: Option<PathBuf>,
    pub git_work_tree: Option<PathBuf>, 
}

pub type ClogResult = Result<Clog, Box<Display>>;

impl Clog {
    fn _new() -> Clog {
        debugln!("Creating private default clog");
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
            version: "".to_owned(),
            patch_ver: false,
            subtitle: "".to_owned(),
            from: "".to_owned(),
            to: "HEAD".to_owned(),
            changelog: "changelog.md".to_owned(),
            section_map: sections,
            git_dir: None,
            git_work_tree: None,
        }
    }

    pub fn new() -> ClogResult {
        debugln!("Creating public default clog");
        Clog::from_file(CLOG_CONFIG_FILE)
    }

    pub fn with_all<P: AsRef<Path>>(git_dir: P, work_tree: P, cfg_file: P) -> ClogResult {
        debugln!("Creating clog with \n\tgit_dir: {:?}\n\twork_tree: {:?}\n\tcfg_file: {:?}", 
            git_dir.as_ref(), 
            work_tree.as_ref(), 
            cfg_file.as_ref());
        let clog = try!(Clog::with_dirs(git_dir, 
                                            work_tree));
        clog.try_config_file(cfg_file.as_ref())   
    }

    pub fn with_dir_and_file<P: AsRef<Path>>(dir: P, cfg_file: P) -> ClogResult {
        debugln!("Creating clog with \n\tdir: {:?}\n\tcfg_file: {:?}", 
            dir.as_ref(), 
            cfg_file.as_ref());
        let clog = try!(Clog::_with_dir(dir));
        clog.try_config_file(cfg_file.as_ref())   
    }

    fn _with_dir<P: AsRef<Path>>(dir: P) -> ClogResult {
        debugln!("Creating private clog with \n\tdir: {:?}", dir.as_ref());
        let mut clog = Clog::_new();
        if dir.as_ref().ends_with(".git") {
            debugln!("dir ends with .git");
            let mut wd = dir.as_ref().to_path_buf();
            clog.git_dir = Some(wd.clone());
            wd.pop();
            clog.git_work_tree = Some(wd);
        } else {
            debugln!("dir doesn't end with .git");
            let mut gd = dir.as_ref().to_path_buf();
            clog.git_work_tree = Some(gd.clone());
            gd.push(".git");
            clog.git_dir = Some(gd);
        }

        Ok(clog)
    }

    pub fn with_dir<P: AsRef<Path>>(dir: P) -> ClogResult {
        let clog = try!(Clog::_with_dir(dir));
        clog.try_config_file(Path::new(CLOG_CONFIG_FILE))
    }

    pub fn with_dirs<P: AsRef<Path>>(git_dir: P, work_tree: P) -> ClogResult {
        debugln!("Creating clog with \n\tgit_dir: {:?}\n\twork_tree: {:?}", 
            git_dir.as_ref(), 
            work_tree.as_ref());
        let mut clog = Clog::_new();
        clog.git_dir = Some(git_dir.as_ref().to_path_buf());
        clog.git_work_tree = Some(work_tree.as_ref().to_path_buf());
        clog.try_config_file(Path::new(CLOG_CONFIG_FILE))
    }

    pub fn from_file<P: AsRef<Path>>(file: P) -> ClogResult {
        debugln!("Creating clog with \n\tfile: {:?}", file.as_ref());
        // Determine if the cfg_file was relative or not
        let cfg_file = if file.as_ref().is_relative() {
            debugln!("file is relative");
            let cwd = match env::current_dir() {
                Ok(d)  => d,
                Err(e) => return Err(Box::new(e)),
            };
            Path::new(&cwd).join(file.as_ref())
        } else {
            debugln!("file is absolute");
            file.as_ref().to_path_buf()
        };

        // We assume whatever dir the .clog.toml file is also contains the git metadata
        let mut dir = cfg_file.clone();
        dir.pop();
        Clog::with_dir_and_file(dir, cfg_file)
    }

    fn try_config_file(mut self, cfg_file: &Path) -> ClogResult {
        debugln!("Trying to use config file: {:?}", cfg_file);
        let mut toml_from_latest = None;
        let mut toml_repo = None;
        let mut toml_subtitle = None;
        let mut toml_link_style = None;
        let mut toml_outfile = None;

        if let Ok(ref mut toml_f) = File::open(cfg_file) {
            let mut toml_s = String::with_capacity(100);

            if let Err(e) = toml_f.read_to_string(&mut toml_s) {
                return Err(Box::new(e))
            }

            toml_s.shrink_to_fit();

            let mut toml = Parser::new(&toml_s[..]);

            let toml_table = match toml.parse() {
                Some(table) => table,
                None        => {
                    return Err(Box::new(format!("Error parsing file: {}\n\nPlease check the format or specify the options manually", cfg_file.to_str().unwrap_or("UNABLE TO DISPLAY"))))
                }
            };

            let clog_table = match toml_table.get("clog") {
                Some(table) => table,
                None        => {
                    return Err(Box::new(format!("Error parsing file {}\n\nPlease check the format or specify the options manually", cfg_file.to_str().unwrap_or("UNABLE TO DISPLAY"))))
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
                        return Err(Box::new(format!("Error parsing file {}\n\n{}", cfg_file.to_str().unwrap_or("UNABLE TO DISPLAY"), err)))
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
                                    self.section_map.insert(sec.to_owned(), alias_vec);
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
            self.from = self.get_latest_tag();
        }

        if let Some(repo) = toml_repo {
            self.repo = repo;
        }

        if let Some(ls) = toml_link_style {
            self.link_style = ls;
        }

        if let Some(subtitle) = toml_subtitle {
            self.subtitle = subtitle;
        }

        if let Some(outfile) = toml_outfile {
            self.changelog = outfile;
        }

        Ok(self)
    }

    pub fn from_matches(matches: &ArgMatches) -> ClogResult {
        let mut clog = if let Some(cfg) = matches.value_of("config") {
            if matches.is_present("workdir") && matches.is_present("gitdir") {
               // use --config --work-tree --git-dir
               try!(Clog::with_all(matches.value_of("gitdir").unwrap(),
                              matches.value_of("workdir").unwrap(),
                              cfg))
            } else if let Some(dir) = matches.value_of("workdir") {
               // use --config --work-tree
               try!(Clog::with_dir_and_file(dir, cfg))
            } else if let Some(dir) = matches.value_of("gitdir") {
               // use --config --git-dir
               try!(Clog::with_dir_and_file(dir, cfg))
            } else {
               // use --config only
               try!(Clog::from_file(cfg))
            }
        } else {
            if matches.is_present("gitdir") && matches.is_present("workdir") {
                let wdir = matches.value_of("workdir").unwrap();
                let gdir = matches.value_of("gitdir").unwrap();
                try!(Clog::with_dirs(gdir, wdir))
            } else if let Some(dir) = matches.value_of("gitdir") {
                try!(Clog::with_dir(dir))
            } else if let Some(dir) = matches.value_of("workdir") {
                try!(Clog::with_dir(dir))
            } else {
                try!(Clog::from_file(CLOG_CONFIG_FILE))
            }
        };

        // compute version early, so we can exit on error
        clog.version = {
            // less typing later...
            let (major, minor, patch) = (matches.is_present("major"), matches.is_present("minor"), matches.is_present("patch"));
            if matches.is_present("ver") {
                matches.value_of("ver").unwrap().to_owned()
            } else if major || minor || patch {
                let mut had_v = false;
                let v_string = clog.get_latest_tag_ver();
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
            clog.from = clog.get_latest_tag();
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

    pub fn grep<S: Into<String>>(&mut self, g: S) -> &mut Clog {
        self.grep = g.into();
        self
    }

    pub fn format<S: Into<String>>(&mut self, f: S) -> &mut Clog {
        self.format = f.into();
        self
    }

    pub fn repository<S: Into<String>>(&mut self, r: S) -> &mut Clog {
        self.repo = r.into();
        self
    }

    pub fn link_style(&mut self, l: LinkStyle) -> &mut Clog {
        self.link_style = l;
        self
    }

    pub fn version<S: Into<String>>(&mut self, v: S) -> &mut Clog {
        self.version = v.into();
        self
    }

    pub fn subtitle<S: Into<String>>(&mut self, s: S) -> &mut Clog {
        self.subtitle = s.into();
        self
    }

    pub fn from<S: Into<String>>(&mut self, f: S) -> &mut Clog {
        self.from = f.into();
        self
    }

    pub fn to<S: Into<String>>(&mut self, t: S) -> &mut Clog {
        self.to = t.into();
        self
    }

    pub fn changelog<S: Into<String>>(&mut self, c: S) -> &mut Clog {
        self.changelog = c.into();
        self
    }

    pub fn git_dir<P: AsRef<Path>>(&mut self, d: P) -> &mut Clog {
        self.git_dir = Some(d.as_ref().to_path_buf());
        self
    }
    pub fn git_work_tree<P: AsRef<Path>>(&mut self, d: P) -> &mut Clog {
        self.git_work_tree = Some(d.as_ref().to_path_buf());
        self
    }

    pub fn patch_version(&mut self, p: bool) -> &mut Clog {
        self.patch_ver = p;
        self
    }

    pub fn get_commits(&self) -> Commits {
        let range = match &self.from[..] {
            "" => "HEAD".to_owned(),
            _  => format!("{}..{}", self.from, self.to)
        };

        let output = Command::new("git")
                .arg(&self.get_git_dir()[..])
                .arg(&self.get_git_work_tree()[..])
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
    pub fn get_latest_tag(&self) -> String {
        let output = Command::new("git")
                .arg(&self.get_git_dir()[..])
                .arg(&self.get_git_work_tree()[..])
                .arg("rev-list")
                .arg("--tags")
                .arg("--max-count=1")
                .output().unwrap_or_else(|e| panic!("Failed to run 'git rev-list' with error: {}",e));
        let buf = String::from_utf8_lossy(&output.stdout);

        buf.trim_matches('\n').to_owned()
    }

    pub fn get_latest_tag_ver(&self) -> String {
        let output = Command::new("git")
                .arg(&self.get_git_dir()[..])
                .arg(&self.get_git_work_tree()[..])
                .arg("describe")
                .arg("--tags")
                .arg("--abbrev=0")
                .output().unwrap_or_else(|e| panic!("Failed to run 'git describe' with error: {}",e));

        String::from_utf8_lossy(&output.stdout).into_owned()
    }

    pub fn get_last_commit(&self) -> String {
        let output = Command::new("git")
                .arg(&self.get_git_dir()[..])
                .arg(&self.get_git_work_tree()[..])
                .arg("rev-parse")
                .arg("HEAD")
                .output().unwrap_or_else(|e| panic!("Failed to run 'git rev-parse' with error: {}", e));

        String::from_utf8_lossy(&output.stdout).into_owned()
    }

    fn get_git_work_tree(&self) -> String {
        // Check if user supplied a local git dir and working tree
        if self.git_work_tree.is_none() && self.git_dir.is_none() {
            // None was provided
            "".to_owned()
        } else if self.git_dir.is_some() {
            // user supplied both
            format!("--work-tree={}", self.git_work_tree.clone().unwrap().to_str().unwrap())
        } else {
            // user only supplied a working tree i.e. /home/user/mycode
            let mut w = self.git_work_tree.clone().unwrap();
            w.pop();
            format!("--work-tree={}", w.to_str().unwrap())
        }

    }

    fn get_git_dir(&self) -> String {
        // Check if user supplied a local git dir and working tree
        if self.git_dir.is_none() && self.git_work_tree.is_none() {
            // None was provided
            "".to_owned()
        } else if self.git_work_tree.is_some() {
            // user supplied both
            format!("--git-dir={}", self.git_dir.clone().unwrap().to_str().unwrap())
        } else {
            // user only supplied a git dir i.e. /home/user/mycode/.git
            let mut g =  self.git_dir.clone().unwrap();
            g.push(".git");
            format!("--git-dir={}", g.to_str().unwrap())
        }
    }

    pub fn section_for(&self, alias: &str) -> &String {
        self.section_map.iter().filter(|&(_, v)| v.iter().any(|s| s == alias)).map(|(k, _)| k).next().unwrap_or(self.section_map.keys().filter(|&k| *k == "Unknown".to_owned()).next().unwrap())
    }
}
