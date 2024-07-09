#[macro_use]
extern crate clap;
#[cfg(feature = "color")]
extern crate ansi_term;
extern crate clog;
extern crate semver;

use std::time::Instant;

use clap::{App, Arg, ArgGroup, ArgMatches};

use clog::fmt::ChangelogFormat;
use clog::{Clog, LinkStyle};

#[macro_use]
mod macros;
mod error;
mod fmt;

use error::CliError;

pub type CliResult<T> = Result<T, CliError>;
const CLOG_CONFIG_FILE: &'static str = ".clog.toml";

fn main() {
    let styles = LinkStyle::variants();
    let matches = App::new("clog")
        // Pull version from Cargo.toml
        .version(crate_version!())
        .about("a conventional changelog for the rest of us")
        .args_from_usage("-r, --repository [URL]      'Repository used for generating commit and issue links \
                                                       (without the .git, e.g. https://github.com/thoughtram/clog)'
                          -f, --from [HASH]           'e.g. 12a8546'
                          -T, --format [FORMAT]       'The output format, defaults to markdown \
                                                       (valid values: markdown, json)'
                          -M, --major                 'Increment major version by one (Sets minor and patch to 0)'
                          -g, --git-dir [PATH]        'Local .git directory (defaults to current dir + \'.git\')*'
                          -w, --work-tree [PATH]      'Local working tree of the git project \
                                                       (defaults to current dir)*'
                          -m, --minor                 'Increment minor version by one (Sets patch to 0)'
                          -p, --patch                 'Increment patch version by one'
                          -s, --subtitle [TITLE]      'e.g. \"Crazy Release Title\"'
                          -t, --to [HASH]             'e.g. 8057684 (Defaults to HEAD when omitted)'
                          -o, --outfile [FILE]        'Where to write the changelog (Defaults to stdout when omitted)'
                          -c, --config [FILE]         'The Clog Configuration TOML file to use (Defaults to \
                                                       \'.clog.toml\')**'
                          -i, --infile [FILE]         'A changelog to append to, but *NOT* write to (Useful in \
                                                       conjunction with --outfile)'
                          --setversion [VER]          'e.g. 1.0.1'")
        // Because --from-latest-tag can't be used with --from, we add it separately so we can
        // specify a .conflicts_with()
        .arg(Arg::from_usage("-F, --from-latest-tag 'use latest tag as start (instead of --from)'")
                .conflicts_with("from"))
        // Because we may want to add more "flavors" at a later date, we can automate the process
        // of enumerating all possible values with clap
        .arg(Arg::from_usage("-l, --link-style [STYLE]     'The style of repository link to generate (Defaults to github)'")
            .possible_values(&styles))
        // Because no one should use --changelog and either an --infile or --outfile, we add those
        // to conflicting lists
        .arg(Arg::from_usage("-C, --changelog [FILE]       'A previous changelog to prepend new changes to (this is like \
                                                           using the same file for both --infile and --outfile and \
                                                           should not be used in conjunction with either)'")
            .conflicts_with("infile")
            .conflicts_with("outfile"))
        // Since --setversion shouldn't be used with any of the --major, --minor, or --match, we
        // set those as exclusions
        .group(ArgGroup::with_name("setver")
                .args(&["major", "minor", "patch", "setversion"]))
        .after_help("\
* If your .git directory is a child of your project directory (most common, such as \
/myproject/.git) AND not in the current working directory (i.e you need to use --work-tree or \
--git-dir) you only need to specify either the --work-tree (i.e. /myproject) OR --git-dir (i.e. \
/myproject/.git), you don't need to use both. \
 \
 \
 \
** If using the --config to specify a clog configuration TOML file NOT in the current working \
directory (meaning you need to use --work-tree or --git-dir) AND the TOML file is inside your \
project directory (i.e. /myproject/.clog.toml) you do not need to use --work-tree or --git-dir.")
        .get_matches();

    let start = Instant::now();

    let clog = from_matches(&matches).unwrap_or_else(|e| e.exit());

    if let Some(ref file) = clog.outfile {
        clog.write_changelog_to(file).unwrap_or_else(|e| e.exit());

        let elapsed = start.elapsed();
        println!("changelog written. (took {} ms)", elapsed.as_millis());
    } else {
        clog.write_changelog().unwrap_or_else(|e| e.exit());
    }
}

/// Creates a `Clog` struct from command line `clap::ArgMatches`
///
/// # Example
///
/// ```ignore
/// # use clog::Clog;
///
/// let matches = // clap settings...
///
/// let clog = Clog::from_matches(matches).unwrap_or_else(|e| {
///     e.exit();
/// });
/// ```
pub fn from_matches(matches: &ArgMatches) -> CliResult<Clog> {
    debugln!("Creating clog from matches");
    let mut clog = if let Some(cfg) = matches.value_of("config") {
        debugln!("User passed in config file: {:?}", cfg);
        if matches.is_present("work-dir") && matches.is_present("gitdir") {
            debugln!(
                "User passed in both\n\tworking dir: {:?}\n\tgit dir: {:?}",
                matches.value_of("work-dir"),
                matches.value_of("git-dir")
            );
            // use --config --work-tree --git-dir
            Clog::with_all(
                matches.value_of("git-dir").unwrap(),
                matches.value_of("work-dir").unwrap(),
                cfg,
            )?
        } else if let Some(dir) = matches.value_of("work-dir") {
            debugln!("User passed in working dir: {:?}", dir);
            // use --config --work-tree
            Clog::with_dir_and_file(dir, cfg)?
        } else if let Some(dir) = matches.value_of("git-dir") {
            debugln!("User passed in git dir: {:?}", dir);
            // use --config --git-dir
            Clog::with_dir_and_file(dir, cfg)?
        } else {
            debugln!("User only passed config");
            // use --config only
            Clog::from_file(cfg)?
        }
    } else {
        debugln!("User didn't pass in a config");
        if matches.is_present("git-dir") && matches.is_present("work-dir") {
            let wdir = matches.value_of("work-dir").unwrap();
            let gdir = matches.value_of("git-dir").unwrap();
            debugln!(
                "User passed in both\n\tworking dir: {:?}\n\tgit dir: {:?}",
                wdir,
                gdir
            );
            Clog::with_dirs(gdir, wdir)?
        } else if let Some(dir) = matches.value_of("git-dir") {
            debugln!("User passed in git dir: {:?}", dir);
            Clog::with_dir(dir)?
        } else if let Some(dir) = matches.value_of("work-dir") {
            debugln!("User passed in working dir: {:?}", dir);
            Clog::with_dir(dir)?
        } else {
            debugln!("Trying the default config file");
            Clog::from_file(CLOG_CONFIG_FILE)?
        }
    };

    // compute version early, so we can exit on error
    clog.version = {
        // less typing later...
        let (major, minor, patch) = (
            matches.is_present("major"),
            matches.is_present("minor"),
            matches.is_present("patch"),
        );
        if matches.is_present("setversion") {
            matches.value_of("setversion").unwrap().to_owned()
        } else if major || minor || patch {
            let mut had_v = false;
            let v_string = clog.get_latest_tag_ver();
            let first_char = v_string.chars().nth(0).unwrap_or(' ');
            let v_slice = if first_char == 'v' || first_char == 'V' {
                had_v = true;
                v_string.trim_start_matches(|c| c == 'v' || c == 'V')
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
                            "Failed to parse version into \
                                                              valid SemVer. Ensure the version \
                                                              is in the X.Y.Z format.",
                        ),
                    ));
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

    if let Some(to) = matches.value_of("to") {
        clog.to = to.to_owned();
    }

    if let Some(repo) = matches.value_of("repository") {
        clog.repo = repo.to_owned();
    }

    if matches.is_present("link-style") {
        clog.link_style =
            value_t!(matches.value_of("link-style"), LinkStyle).unwrap_or(LinkStyle::Github);
    }

    if let Some(subtitle) = matches.value_of("subtitle") {
        clog.subtitle = subtitle.to_owned();
    }

    if let Some(file) = matches.value_of("outfile") {
        clog.outfile = Some(file.to_owned());
    }

    if let Some(file) = matches.value_of("infile") {
        clog.infile = Some(file.to_owned());
    }

    if let Some(file) = matches.value_of("changelog") {
        clog.infile = Some(file.to_owned());
        clog.outfile = Some(file.to_owned());
    }

    if matches.is_present("format") {
        clog.out_format = value_t_or_exit!(matches.value_of("format"), ChangelogFormat);
    }

    debugln!("Returning clog:\n{:?}", clog);

    Ok(clog)
}
