use clap::{Parser, ValueEnum};
use clog::{fmt::ChangelogFormat, Clog, LinkStyle as ClogLinkStyle};
use strum::{Display, EnumString};

use crate::{
    error::{CliError, CliResult},
    DEFAULT_CONFIG_FILE,
};

static VERSION: &str = env!("CARGO_PKG_VERSION");

static AFTER_HELP: &str = "
If your .git directory is a child of your project directory (most common, such \
as /myproject/.git) AND not in the current working directory (i.e you need to \
use --work-tree or --git-dir) you only need to specify either the --work-tree \
(i.e. /myproject) OR --git-dir (i.e. /myproject/.git), you don't need to use \
both.

If using the --config to specify a clog configuration TOML file NOT in the \
current working directory (meaning you need to use --work-tree or --git-dir) \
AND the TOML file is inside your project directory (i.e. \
/myproject/.clog.toml) you do not need to use --work-tree or --git-dir.
";

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, ValueEnum, EnumString, Display)]
#[strum(serialize_all = "lowercase", ascii_case_insensitive)]
pub enum OutFormat {
    #[default]
    Markdown,
    Json,
}

impl From<OutFormat> for ChangelogFormat {
    fn from(of: OutFormat) -> ChangelogFormat {
        match of {
            OutFormat::Markdown => ChangelogFormat::Markdown,
            OutFormat::Json => ChangelogFormat::Json,
        }
    }
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, ValueEnum, EnumString, Display)]
#[strum(serialize_all = "lowercase", ascii_case_insensitive)]
pub enum LinkStyle {
    #[default]
    Github,
    Gitlab,
    Stash,
    Cgit,
}

impl From<LinkStyle> for ClogLinkStyle {
    fn from(ls: LinkStyle) -> ClogLinkStyle {
        match ls {
            LinkStyle::Github => ClogLinkStyle::Github,
            LinkStyle::Gitlab => ClogLinkStyle::Gitlab,
            LinkStyle::Stash => ClogLinkStyle::Stash,
            LinkStyle::Cgit => ClogLinkStyle::Cgit,
        }
    }
}

/// a conventional changelog for the rest of us
#[derive(Debug, Clone, PartialEq, Eq, Parser)]
#[command(name= "clog", version = VERSION, after_help = AFTER_HELP)]
pub struct Args {
    /// Repository used for generating commit and issue links (without the .git,
    /// e.g. https://github.com/thoughtram/clog)
    #[arg(short, long, value_name = "URL")]
    pub repository: Option<String>,

    /// e.g. 12a8546
    #[arg(short, long, value_name = "COMMIT")]
    pub from: Option<String>,

    /// The output format, defaults to markdown
    #[arg(short = 'T', long, value_name = "STR", default_value_t)]
    pub format: OutFormat,

    /// Increment major version by one (Sets minor and patch to 0)
    #[arg(short = 'M', long)]
    pub major: bool,

    /// Local .git directory (defaults to "$(pwd)/.git")
    #[arg(short, long, value_name = "PATH")]
    pub git_dir: Option<String>,

    /// Local working tree of the git project (defaults to "$(pwd)")
    #[arg(short, long, value_name = "PATH")]
    pub work_tree: Option<String>,

    /// Increment minor version by one (Sets patch to 0)
    #[arg(short, long)]
    pub minor: bool,

    /// Increment patch version by one
    #[arg(short, long)]
    pub patch: bool,

    // e.g. "Crazy Release Title"
    #[arg(short, long, value_name = "STR")]
    pub subtitle: Option<String>,

    /// e.g. 8057684
    #[arg(short, long, value_name = "COMMIT", default_value = "HEAD")]
    pub to: String,

    /// Where to write the changelog (Defaults to stdout when omitted)
    #[arg(short, long, value_name = "PATH")]
    pub outfile: Option<String>,

    /// The Clog Configuration TOML file to use
    #[arg(short, long, value_name = "COMMIT", default_value = DEFAULT_CONFIG_FILE)]
    pub config: String,

    /// A changelog to append to, but *NOT* write to (Useful in conjunction with
    /// --outfile)
    #[arg(short, long, value_name = "PATH")]
    pub infile: Option<String>,
    /// e.g. 1.0.1
    #[arg(long, value_name = "VER", group = "setver")]
    pub setversion: Option<String>,

    /// use latest tag as start (instead of --from)
    #[arg(short = 'F', long, conflicts_with = "from")]
    pub from_latest_tag: bool,

    /// The style of repository link to generate
    #[arg(short, long, value_name = "STR", default_value_t)]
    pub link_style: LinkStyle,

    /// A previous changelog to prepend new changes to (this is like using the
    /// same file for both --infile and --outfile and should not be used in
    /// conjunction with either)
    #[arg(short = 'C', long, value_name = "PATH", conflicts_with_all = ["infile", "outfile"])]
    pub changelog: Option<String>,
}

impl Args {
    pub fn into_clog(self) -> CliResult<Clog> {
        debugln!("Creating clog from matches");
        let mut clog = if self.work_tree.is_some() && self.git_dir.is_some() {
            debugln!(
                "User passed in both\n\tworking dir: {:?}\n\tgit dir: {:?}",
                self.work_tree,
                self.git_dir
            );
            // use --config --work-tree --git-dir
            Clog::with_all(
                self.git_dir.as_ref().unwrap(),
                self.work_tree.as_ref().unwrap(),
                &self.config,
            )?
        } else if let Some(dir) = &self.work_tree {
            debugln!("User passed in working dir: {:?}", dir);
            // use --config --work-tree
            Clog::with_dir_and_file(dir, &self.config)?
        } else if let Some(dir) = &self.git_dir {
            debugln!("User passed in git dir: {:?}", dir);
            // use --config --git-dir
            Clog::with_dir_and_file(dir, &self.config)?
        } else {
            debugln!("User only passed config");
            // use --config only
            Clog::from_file(&self.config)?
        };

        // compute version early, so we can exit on error
        clog.version = {
            // less typing later...
            let (major, minor, patch) = (self.major, self.minor, self.patch);
            if self.setversion.is_some() {
                self.setversion.as_ref().unwrap().to_owned()
            } else if major || minor || patch {
                let mut had_v = false;
                let v_string = clog.get_latest_tag_ver();
                let first_char = v_string.chars().next().unwrap_or(' ');
                let v_slice = if first_char == 'v' || first_char == 'V' {
                    had_v = true;
                    v_string.trim_start_matches(['v', 'V'])
                } else {
                    &v_string[..]
                };
                match semver::Version::parse(v_slice) {
                    Ok(ref mut v) => {
                        // if-else may be quicker, but it's longer mentally, and this isn't slow
                        match (major, minor, patch) {
                            (true, _, _) => {
                                v.major += 1;
                                v.minor = 0;
                                v.patch = 0;
                            }
                            (_, true, _) => {
                                v.minor += 1;
                                v.patch = 0;
                            }
                            (_, _, true) => {
                                v.patch += 1;
                                clog.patch_ver = true;
                            }
                            _ => unreachable!(),
                        }
                        format!("{}{}", if had_v { "v" } else { "" }, v)
                    }
                    Err(e) => {
                        return Err(CliError::Semver(
                            Box::new(e),
                            String::from(
                                "Failed to parse version into valid SemVer. \
                                Ensure the version is in the X.Y.Z format.",
                            ),
                        ));
                    }
                }
            } else {
                clog.version
            }
        };

        if let Some(from) = &self.from {
            clog.from = from.to_owned();
        } else if self.from_latest_tag {
            clog.from = clog.get_latest_tag();
        }

        if let Some(repo) = &self.repository {
            clog.repo = repo.to_owned();
        }

        if let Some(subtitle) = &self.subtitle {
            clog.subtitle = subtitle.to_owned();
        }

        if let Some(file) = &self.outfile {
            clog.outfile = Some(file.to_owned());
        }

        if let Some(file) = &self.infile {
            clog.infile = Some(file.to_owned());
        }

        if let Some(file) = &self.changelog {
            clog.infile = Some(file.to_owned());
            clog.outfile = Some(file.to_owned());
        }

        clog.link_style = self.link_style.into();
        clog.to = self.to.to_owned();
        clog.out_format = self.format.into();

        debugln!("Returning clog:\n{:?}", clog);

        Ok(clog)
    }
}
