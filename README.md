clog-cli
====

[![Join the chat at https://gitter.im/thoughtram/clog](https://badges.gitter.im/Join%20Chat.svg)](https://gitter.im/thoughtram/clog?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

[![Build Status](https://travis-ci.org/clog-tool/clog-cli.png?branch=master)](https://travis-ci.org/clog-tool/clog-cli)

A [conventional][convention] changelog for the rest of us

[convention]: https://github.com/ajoslin/conventional-changelog/blob/a5505865ff3dd710cf757f50530e73ef0ca641da/conventions/angular.md

## About

`clog` creates a changelog automatically from your local git metadata. See the `clog`s [changelog.md](https://github.com/clog-tool/clog-cli/blob/master/changelog.md) for an example.

The way this works, is every time you make a commit, you ensure your commit subject line follows the [conventional](https://github.com/ajoslin/conventional-changelog/blob/a5505865ff3dd710cf757f50530e73ef0ca641da/conventions/angular.md) format. Then when you wish to update your changelog, you simply run `clog` inside your local repository with any options you'd like to specify.

*NOTE:* `clog` also supports empty components by making commit messages such as `alias: message` or `alias(): message` (i.e. without the component)

## Usage

There are two ways to use `clog`, as a binary via the command line or as a library in your applications via [clog-lib](https://github.com/clog-tool/clog-lib).

### Binary (Command Line)

In order to use `clog` via the command line you must first obtain a binary by either compiling it yourself, or downlading and installing one of the precompiled binaries.

#### `cargo install`

If you want to both compile and install `clog` using *cargo* you can simply run

```bash
cargo install clog-cli
```

#### Compiling

Follow these instructions to compile `clog`, then skip down to Installation.

 1. Ensure you have current version of `cargo` and [Rust](https://www.rust-lang.org) installed
 2. Clone the project `$ git clone https://github.com/clog-tool/clog-cli && cd clog-cli`
 3. Build the project `$ cargo build --release`
 4. Once complete, the binary will be located at `target/release/clog`

#### Using a Precompiled Binary

There are several precompiled binaries readily availbe. Browse to http://wod.twentyfives.net/bin/clog/ and download the latest binary for your particular OS. Once you download and extract the tar file (or zip for Windows), the binary will be located at `bin/clog`

**Note**: The Mac distribution is available on npm via [clog-cli](http://npm.im/clog-cli).

#### Installation

Once you have downloaded, or compiled, `clog` you simply need to place the binary somewhere in your `$PATH`. If you are not familiar with `$PATH` read-on; otherwise skip down to Using clog.

##### Arch Linux

You can use `clog-bin` from the AUR, or follow the instructions for Linux / OS X

##### Linux / OS X

You have two options, place `clog` into a directory that is already located in your `$PATH` variable (To see which directories those are, open a terminal and type `echo "${PATH//:/\n}"`, the quotation marks are important), or you can add a custom directory to your `$PATH`

**Option 1**
If you have write permission to a directory listed in your `$PATH` or you have root permission (or via `sudo`), simply copy the `clog` to that directory `# sudo cp clog /usr/local/bin`

**Option 2**
If you do not have root, `sudo`, or write permission to any directory already in `$PATH` you can create a directory inside your home directory, and add that. Many people use `$HOME/.bin` to keep it hidden (and not clutter your home directory), or `$HOME/bin` if you want it to be always visible. Here is an example to make the directory, add it to `$PATH`, and copy `clog` there.

Simply change `bin` to whatever you'd like to name the directory, and `.bashrc` to whatever your shell startup file is (usually `.bashrc`, `.bash_profile`, or `.zshrc`)

```sh
$ mkdir ~/bin
$ echo "export PATH=$PATH:$HOME/bin" >> ~/.bashrc
$ cp clog ~/bin
$ source ~/.bashrc
```

##### Windows

On Windows 7/8 you can add directory to the `PATH` variable by opening a command line as an administrator and running

```sh
C:\> setx path "%path%;C:\path\to\clog\binary"
```

Otherwise, ensure you have the `clog` binary in the directory which you operating in the command line from, because Windows automatically adds your current directory to PATH (i.e. if you open a command line to `C:\my_project\` to use `clog` ensure `clog.exe` is inside that directory as well).

#### Using clog from the Command Line

`clog` works by reading your `git` metadata and specially crafted commit messages and subjects to create a changelog. `clog` has the following options availble.

```sh
USAGE:
    clog [FLAGS] [OPTIONS]

FLAGS:
    -F, --from-latest-tag    use latest tag as start (instead of --from)
    -h, --help               Prints help information
    -M, --major              Increment major version by one (Sets minor and patch to 0)
    -m, --minor              Increment minor version by one (Sets patch to 0)
    -p, --patch              Increment patch version by one
    -V, --version            Prints version information

OPTIONS:
    -C, --changelog <changelog>    A previous changelog to prepend new changes to (this is like
                                   using the same file for both --infile and --outfile and
                                   should not be used in conjuction with either)
    -c, --config <config>          The Clog Configuration TOML file to use (Defaults to
                                   '.clog.toml')**
    -T, --format <format>          The output format, defaults to markdown
                                   (valid values: markdown, json)
    -f, --from <from>              e.g. 12a8546
    -g, --git-dir <gitdir>         Local .git directory (defaults to current dir + '.git')*
    -i, --infile <infile>          A changelog to append to, but *NOT* write to (Useful in
                                   conjunction with --outfile)
    -o, --outfile <outfile>        Where to write the changelog (Defaults to stdout when omitted)
    -r, --repository <repo>        Repository used for generating commit and issue links
                                   (without the .git, e.g. https://github.com/clog-tool/clog-cli)
    -l, --link-style <style>       The style of repository link to generate
                                   (Defaults to github) [values: Github Gitlab Stash]
    -s, --subtitle <subtitle>      e.g. "Crazy Release Title"
    -t, --to <to>                  e.g. 8057684 (Defaults to HEAD when omitted)
        --setversion <ver>         e.g. 1.0.1
    -w, --work-tree <workdir>      Local working tree of the git project
                                   (defaults to current dir)*

* If your .git directory is a child of your project directory (most common, such as
/myproject/.git) AND not in the current working directory (i.e you need to use --work-tree or
--git-dir) you only need to specify either the --work-tree (i.e. /myproject) OR --git-dir (i.e.
/myproject/.git), you don't need to use both.

** If using the --config to specify a clog configuration TOML file NOT in the current working
directory (meaning you need to use --work-tree or --git-dir) AND the TOML file is inside your
project directory (i.e. /myproject/.clog.toml) you do not need to use --work-tree or --git-dir.
```

#### Try it!

In order to see it in action, you'll need a repository that already has some of those specially crafted commit messages in it's history. For this, we'll use the `clog` repository itself.

1. Clone the repo `git clone https://github.com/clog-tool/clog-cli && cd clog-cli`

2. Ensure you already `clog` binary from any of the steps above

4. There are many, many ways to run `clog`. Note, in these examples we will be typing the same options over and over again, in times like that we could a [clog TOML configuration file](https://github.com/clog-tool/clog-cli#default-options) to specify those options that don't normally change. Also note, all these CLI options have short versions as well, we're using the long version because they're easier to understand.

  1. Let's start by picking up only new commits since our last release (this may not be a lot...or none)
  2. Run `clog -r https://github.com/clog-tool/clog-cli --outfile only_new.md`
  3. By default, `clog` outputs to `stdout` unless you have a file set inside a TOML configuration file. (Note, we could have used the shell `>` operator instead of `--outfile`)
  4. Anything options you set via the CLI will override anything you set the configuration file.
  5. Let's now tell `clog` where it can find our old changelog, and prepend any new commits to that old data
  6. Run `clog -r https://github.com/clog-tool/clog-cli --infile changelog.md --outfile new_combined.md`
  7. Finally, let's assume like most projects we just want to use one file, and prepend all new data to our old changelog (most useful)
  8. First make a backup of the `changelog.md` so you can compare it later `cp changelog.md changelog.md.bak`
  9. Run `clog -r https://github.com/clog-tool/clog-cli --changelog changelog.md`
  10. Try viewing any of the `only_new.md`, `new_combined.md`, `changelog.md.bak`, or `changelog.md` in your favorite markdown viewer to compare them.

### As a Library

See the [documentation](http://clog-tool.github.io/clog-lib/) or [clog-lib](https://github.com/clog-tool/clog-lib) for information on using `clog` in your applications. You can also see the [clog crates.io page](https://crates.io/crates/clog).

### Default Options

`clog` can also be configured using a default configuration file so that you don't have to specify all the options each time you want to update your changelog. To do this add a `.clog.toml` file to your repository.

```toml
[clog]
# A repository link with the trailing '.git' which will be used to generate
# all commit and issue links
repository = "https://github.com/clog-tool/clog-cli"
# A constant release title
subtitle = "my awesome title"

# specify the style of commit links to generate, defaults to "github" if omitted
link-style = "github"

# The preferred way to set a constant changelog. This file will be read for old changelog
# data, then prepended to for new changelog data. It's the equivilant to setting
# both infile and outfile to the same file.
#
# Do not use with outfile or infile fields!
#
# Defaults to stdout when omitted
changelog = "mychangelog.md"

# This sets an output file only! If it exists already, new changelog data will be
# prepended, if not it will be created.
#
# This is useful in conjunction with the infile field if you have a separate file
# that you would like to append after newly created clog data
#
# Defaults to stdout when omitted
outfile = "MyChangelog.md"

# This sets the input file old! Any data inside this file will be appended to any
# new data that clog picks up
#
# This is useful in conjunction with the outfile field where you may wish to read
# from one file and append that data to the clog output in another
infile = "My_old_changelog.md"

# This sets the output format. There are two options "json" or "markdown" and
# defaults to "markdown" when omitted
output-format = "json"

# If you use tags, you can set the following if you wish to only pick
# up changes since your latest tag
from-latest-tag = true
```

Now you can update your `MyChangelog.md` with `clog --patch` (assuming you want to update from the latest tag version, and increment your patch version by 1).

*Note:* Any options you specify at the command line will override options set in your `.clog.toml`

### Custom Sections

By default, `clog` will display three sections in your changelog, `Features`, `Performance`, and `Bug Fixes`. You can add additional sections by using a `.clog.toml` file. To add more sections, simply add a `[sections]` table, along with the section name and aliases you'd like to use in your commit messages:

```toml
[sections]
MySection = ["mysec", "ms"]
```

Now if you make a commit message such as `mysec(Component): some message` or `ms(Component): some message` there will be a new "MySection" section along side the "Features" and "Bug Fixes" areas.

*NOTE:* Sections with spaces are suppported, such as `"My Special Section" = ["ms", "mysec"]`

## Companion Projects

- [Commitizen](http://commitizen.github.io/cz-cli/) - A command line tool that helps you writing better commit messages.

## LICENSE

`clog` is licensed under the MIT Open Source license. For more information, see the LICENSE file in this repository.
