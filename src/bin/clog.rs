#[macro_use]
extern crate clap;
extern crate time;
extern crate clog;

use clap::{App, Arg, ArgGroup};

use clog::{LinkStyle, Clog};

fn main () {
    let styles = LinkStyle::variants();
    let matches = App::new("clog")
        // Pull version from Cargo.toml
        .version(&format!("v{}", crate_version!())[..])
        .about("a conventional changelog for the rest of us")
        .args_from_usage("-r, --repository=[repo]   'Repo used for link generation (without the .git, e.g. https://github.com/thoughtram/clog)'
                          -f, --from=[from]         'e.g. 12a8546'
                          -M, --major               'Increment major version by one (Sets minor and patch to 0)'
                          -g, --git-dir=[gitdir]    'Local .git directory (defaults to current dir + \'.git\')*'
                          -w, --work-tree=[workdir] 'Local working tree of the git project (defaults to current dir)*' 
                          -m, --minor               'Increment minor version by one (Sets patch to 0)'
                          -p, --patch               'Increment patch version by one'
                          -s, --subtitle=[subtitle] 'e.g. \"Crazy Release Title\"'
                          -t, --to=[to]             'e.g. 8057684 (Defaults to HEAD when omitted)'
                          -o, --outfile=[outfile]   'Where to write the changelog (Defaults to \'changelog.md\')'
                          -c, --config=[config]     'The Clog Configuration TOML file to use (Defaults to \'.clog.toml\')**'
                          --setversion=[ver]        'e.g. 1.0.1'")
        // Because --from-latest-tag can't be used with --from, we add it seperately so we can
        // specify a .conflicts_with()
        .arg(Arg::from_usage("-F, --from-latest-tag 'use latest tag as start (instead of --from)'")
                .conflicts_with("from"))
        // Because we may want to add more "flavors" at a later date, we can automate the process
        // of enumerating all possible values with clap
        .arg(Arg::from_usage("-l, --link-style=[style]     'The style of repository link to generate (Defaults to github)'")
            .possible_values(&styles))
        // Since --setversion shouldn't be used with any of the --major, --minor, or --match, we
        // set those as exclusions
        .arg_group(ArgGroup::with_name("setver")
                .add_all(vec!["major", "minor", "patch", "ver"]))
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

    let clog = Clog::from_matches(&matches).unwrap_or_else(|e| e.exit());

    if let Some(ref file) = clog.changelog {
        clog.write_changelog_to(file).unwrap_or_else(|e| e.exit());

        let end_nsec = time::get_time().nsec;
        let elapsed_mssec = (end_nsec - start_nsec) / 1000000;
        println!("changelog written. (took {} ms)", elapsed_mssec);
    } else {
        clog.write_changelog().unwrap_or_else(|e| e.exit());
    }
}
