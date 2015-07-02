clog
====

[![Join the chat at https://gitter.im/thoughtram/clog](https://badges.gitter.im/Join%20Chat.svg)](https://gitter.im/thoughtram/clog?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

[![Build Status](https://travis-ci.org/thoughtram/clog.png?branch=master)](https://travis-ci.org/thoughtram/clog)

A [conventional][convention] changelog for the rest of us

[convention]: https://github.com/ajoslin/conventional-changelog/blob/a5505865ff3dd710cf757f50530e73ef0ca641da/conventions/angular.md

### About

`clog` creates a changelog automatically from your local git metadata. See the `clog`s [changelog.md](https://github.com/thoughtram/clog/blob/master/changelog.md) for an example.

The way this works, is every time you make a commit, you ensure your commit subject line follows the [conventional](https://github.com/thoughtram/clog/blob/master/changelog.md) format. Then when you wish to update your changelog, you simply run `clog` inside your local repository with any options you'd like to specify.

*NOTE:* `clog` also supports empty components by making commit messages such as `alias: message` or `alias(): message` (i.e. without the component)


### Usage

There are two ways to use `clog`, via the command line or a library in your applicaitons.

#### Command Line

```
USAGE:
    clog [FLAGS] [OPTIONS]

FLAGS:
    -c, --config             The Clog Configuration TOML file to use (Defaults to '.clog.toml')**
    -F, --from-latest-tag    use latest tag as start (instead of --from)
    -h, --help               Prints help information
    -M, --major              Increment major version by one (Sets minor and patch to 0)
    -m, --minor              Increment minor version by one (Sets patch to 0)
    -p, --patch              Increment patch version by one
    -V, --version            Prints version information

OPTIONS:
    -f, --from <from>                e.g. 12a8546
    -g, --git-dir <gitdir>           Local .git directory (defaults to current dir + '.git')*
    -o, --outfile <outfile>          Where to write the changelog (Defaults to 'changelog.md')
    -r, --repository <repo>          Repo used for link generation (without the .git, e.g. https://github.com/thoughtram/clog)
    -l, --link-style <style>         The style of repository link to generate (Defaults to github) [values: Github, Gitlab, Stash]
    -s, --subtitle <subtitle>        e.g. "Crazy Release Title"
    -t, --to <to>                    e.g. 8057684 (Defaults to HEAD when omitted)
        --setversion <ver>           e.g. 1.0.1
    -w, --work-tree <workdir>        Local working tree of the git project (defaults to current dir)*

* If your .git directory is a child of your project directory (most common, such as
/myproject/.git) AND not in the current working directory (i.e you need to use --work-tree or
--git-dir) you only need to specify either the --work-tree (i.e. /myproject) OR --git-dir (i.e. 
/myproject/.git), you don't need to use both.

** If using the --config to specify a clog configuration TOML file NOT in the current working
directory (meaning you need to use --work-tree or --git-dir) AND the TOML file is inside your
project directory (i.e. /myproject/.clog.toml) you do not need to use --work-tree or --git-dir.
```

##### Try it!

1. Clone the repo `git clone https://github.com/thoughtram/clog && cd clog`

2. Build clog `cargo build --release`

3. Delete the old changelog file `rm changelog.md`

3. Run clog `./target/release/clog -r https://github.com/thoughtram/clog --setversion 0.1.0 --subtitle crazy-dog --from 6d8183f`

#### As a Library

See the documentation for information on using `clog` in your applications.

##### Try it!

 1. Clone the `clog` repo so that you have something to search through (Because `clog` uses 
    specially formatted commit messages)
```
$ git clone https://github.com/thoughtram/clog ~/clog
```

 2. Add `clog` as a dependency in your `Cargo.toml` 

```toml
[dependencies]
clog = "*"
```

 3. Use the following in your `src/main.rs`

```rust
extern crate clog;

use clog::Clog;

fn main() {
    // Create the struct
    let mut clog = Clog::with_dir("~/clog").unwrap_or_else(|e| { 
        println!("{}",e); 
        std::process::exit(1); 
    });

    // Set some options
    clog.repository("https://github.com/thoughtram/clog")
        .subtitle("Crazy Dog")
        .from("6d8183f")
        .version("0.1.0");

    // Write the changelog to the current working directory
    //
    // Alternatively we could have used .write_changelog_to("/somedir/some_file.md")
    clog.write_changelog();
}
```

 4. Compile and run `$ cargo build --release && ./target/release/bin_name
 5. View the output in your favorite markdown viewer! `$ vim changelog.md`

### Default Options

`clog` can also be configured using a default configuration file so that you don't have to specify all the options each time you want to update your changelog. To do this add a `.clog.toml` file to your repository.

```toml
[clog]
repository = "https://github.com/thoughtram/clog"
subtitle = "my awesome title"

# specify the style of commit links to generate, defaults to "github" if omitted
link-style = "github"

# sets the changelog output file, defaults to "changelog.md" if omitted
outfile = "MyChangelog.md"

# If you use tags, you can set the following if you wish to only pick
# up changes since your latest tag
from-latest-tag = true
```

Now you can update your `MyChangelog.md` with `clog --patch` (assuming you want to update from the latest tag version, and increment your patch version by 1).

*Note:* Any options you specify at the command line will override options set in your `.clog.toml`

#### Custom Sections

By default, `clog` will display two sections in your changelog, `Features` and `Bug Fixes`. You can add additional sections by using a `.clog.toml` file. To add more sections, simply add a `[sections]` table, along with the section name and aliases you'd like to use in your commit messages:

```toml
[sections]
MySection = ["mysec", "ms"]
```

Now if you make a commit message such as `mysec(Component): some message` or `ms(Component): some message` there will be a new "MySection" section along side the "Features" and "Bug Fixes" areas.

*NOTE:* Sections with spaces are suppported, such as `"My Special Section" = ["ms", "mysec"]`

## LICENSE

clog is licensed under the MIT Open Source license. For more information, see the LICENSE file in this repository.
