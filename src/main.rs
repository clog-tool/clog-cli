#[macro_use]
extern crate clap;
extern crate time;
extern crate clog;
extern crate semver;
#[cfg(feature = "color")]
extern crate ansi_term;

use clap::{App, Arg, ArgGroup, ArgMatches};

use clog::{LinkStyle, Clog};
use clog::fmt::ChangelogFormat;

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
        .version(&format!("v{}", crate_version!())[..])
        .about("a conventional changelog for the rest of us")
        .args_from_usage("-r, --repository=[repo]     'Repository used for generating commit and issue links{n}\
                                                       (without the .git, e.g. https://github.com/thoughtram/clog)'
                          -f, --from=[from]           'e.g. 12a8546'
                          -T, --format=[format]       'The output format, defaults to markdown{n}\
                                                       (valid values: markdown, json)'
                          -M, --major                 'Increment major version by one (Sets minor and patch to 0)'
                          -g, --git-dir=[gitdir]      'Local .git directory (defaults to current dir + \'.git\')*'
                          -w, --work-tree=[workdir]   'Local working tree of the git project{n}\
                                                       (defaults to current dir)*'
                          -m, --minor                 'Increment minor version by one (Sets patch to 0)'
                          -p, --patch                 'Increment patch version by one'
                          -s, --subtitle=[subtitle]   'e.g. \"Crazy Release Title\"'
                          -t, --to=[to]               'e.g. 8057684 (Defaults to HEAD when omitted)'
                          -o, --outfile=[outfile]     'Where to write the changelog (Defaults to stdout when omitted)'
                          -c, --config=[config]       'The Clog Configuration TOML file to use (Defaults to{n}\
                                                       \'.clog.toml\')**'
                          -i, --infile=[infile]       'A changelog to append to, but *NOT* write to (Useful in{n}\
                                                       conjunction with --outfile)'
                          --setversion=[ver]          'e.g. 1.0.1'")
        // Because --from-latest-tag can't be used with --from, we add it seperately so we can
        // specify a .conflicts_with()
        .arg(Arg::from_usage("-F, --from-latest-tag 'use latest tag as start (instead of --from)'")
                .conflicts_with("from"))
        // Because we may want to add more "flavors" at a later date, we can automate the process
        // of enumerating all possible values with clap
        .arg(Arg::from_usage("-l, --link-style=[style]     'The style of repository link to generate{n}(Defaults to github)'")
            .possible_values(&styles))
        // Because no one should use --changelog and either an --infile or --outfile, we add those
        // to conflicting lists
        .arg(Arg::from_usage("-C, --changelog=[changelog] 'A previous changelog to prepend new changes to (this is like{n}\
                                                           using the same file for both --infile and --outfile and{n}\
                                                           should not be used in conjuction with either)'")
            .conflicts_with("infile")
            .conflicts_with("outfile"))
        // Since --setversion shouldn't be used with any of the --major, --minor, or --match, we
        // set those as exclusions
        .arg_group(ArgGroup::with_name("setver")
                .add_all(&["major", "minor", "patch", "ver"]))
        .after_help("\
* If your .git directory is a child of your project directory (most common, such as\n\
/myproject/.git) AND not in the current working directory (i.e you need to use --work-tree or\n\
--git-dir) you only need to specify either the --work-tree (i.e. /myproject) OR --git-dir (i.e. \n\
/myproject/.git), you don't need to use both.\n\n\

** If using the --config to specify a clog configuration TOML file NOT in the current working\n\
directory (meaning you need to use --work-tree or --git-dir) AND the TOML file is inside your\n\
project directory (i.e. /myproject/.clog.toml) you do not need to use --work-tree or --git-dir.")
        .get_matches();

    let start_nsec = time::get_time().nsec;

    let clog = from_matches(&matches).unwrap_or_else(|e| e.exit());

    if let Some(ref file) = clog.outfile {
        clog.write_changelog_to(file).unwrap_or_else(|e| e.exit());

        let end_nsec = time::get_time().nsec;
        let elapsed_mssec = (end_nsec - start_nsec) / 1000000;
        println!("changelog written. (took {} ms)", elapsed_mssec);
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
        if matches.is_present("workdir") && matches.is_present("gitdir") {
            debugln!("User passed in both\n\tworking dir: {:?}\n\tgit dir: {:?}",
                     matches.value_of("workdir"),
                     matches.value_of("gitdir"));
           // use --config --work-tree --git-dir
            try!(Clog::with_all(matches.value_of("gitdir").unwrap(),
                                matches.value_of("workdir").unwrap(),
                                cfg))
        } else if let Some(dir) = matches.value_of("workdir") {
            debugln!("User passed in working dir: {:?}", dir);
           // use --config --work-tree
            try!(Clog::with_dir_and_file(dir, cfg))
        } else if let Some(dir) = matches.value_of("gitdir") {
            debugln!("User passed in git dir: {:?}", dir);
           // use --config --git-dir
            try!(Clog::with_dir_and_file(dir, cfg))
        } else {
            debugln!("User only passed config");
           // use --config only
            try!(Clog::from_file(cfg))
        }
    } else {
        debugln!("User didn't pass in a config");
        if matches.is_present("gitdir") && matches.is_present("workdir") {
            let wdir = matches.value_of("workdir").unwrap();
            let gdir = matches.value_of("gitdir").unwrap();
            debugln!("User passed in both\n\tworking dir: {:?}\n\tgit dir: {:?}",
                     wdir,
                     gdir);
            try!(Clog::with_dirs(gdir, wdir))
        } else if let Some(dir) = matches.value_of("gitdir") {
            debugln!("User passed in git dir: {:?}", dir);
            try!(Clog::with_dir(dir))
        } else if let Some(dir) = matches.value_of("workdir") {
            debugln!("User passed in working dir: {:?}", dir);
            try!(Clog::with_dir(dir))
        } else {
            debugln!("Trying the default config file");
            try!(Clog::from_file(CLOG_CONFIG_FILE))
        }
    };

    // compute version early, so we can exit on error
    clog.version = {
        // less typing later...
        let (major, minor, patch) = (matches.is_present("major"),
                                     matches.is_present("minor"),
                                     matches.is_present("patch"));
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
                        (true,_,_) => {
                            v.major += 1;
                            v.minor = 0;
                            v.patch = 0;
                        }
                        (_,true,_) => {
                            v.minor += 1;
                            v.patch = 0;
                        }
                        (_,_,true) => {
                            v.patch += 1;
                            clog.patch_ver = true;
                        }
                        _ => unreachable!(),
                    }
                    format!("{}{}",
                            if had_v {
                                "v"
                            } else {
                                ""
                            },
                            v)
                }
                Err(e) => {
                    return Err(CliError::Semver(Box::new(e),
                                                String::from("Failed to parse version into \
                                                              valid SemVer. Ensure the version \
                                                              is in the X.Y.Z format.")));
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
        clog.link_style = value_t!(matches.value_of("link-style"), LinkStyle)
                              .unwrap_or(LinkStyle::Github);
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
