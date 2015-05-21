clog
====

[![Join the chat at https://gitter.im/thoughtram/clog](https://badges.gitter.im/Join%20Chat.svg)](https://gitter.im/thoughtram/clog?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

[![Build Status](https://travis-ci.org/thoughtram/clog.png?branch=master)](https://travis-ci.org/thoughtram/clog)

A [conventional](https://github.com/ajoslin/conventional-changelog/blob/master/CONVENTIONS.md) changelog for the rest of us

### About

`clog` creates a changelog automatically from your local git metadata. See the `clog`s [changelog.md](https://github.com/thoughtram/clog/blob/master/changelog.md) for an example.

The way this works, is every time you make a commit, you ensure your commit subject line follows the [conventional](https://github.com/thoughtram/clog/blob/master/changelog.md) format. Then when you wish to update your changelog, you simply run `clog` inside your local repository with any options you'd like to specify.

*NOTE:* `clog` also supports empty components by making commit messages such as `alias: message` or `alias(): message` (i.e. without the component)


### Usage

```
USAGE:
	clog [FLAGS] [OPTIONS]

FLAGS:
        --from-latest-tag    use latest tag as start (instead of --from)
    -h, --help               Prints help information
        --major              Increment major version by one (Sets minor and patch to 0)
        --minor              Increment minor version by one (Sets patch to 0)
        --patch              Increment patch version by one
    -v, --version            Prints version information

OPTIONS:
        --from <from>                e.g. 12a8546
    -o, --outfile <outfile>          Where to write the changelog (Defaults to 'changelog.md')
    -r, --repository <repository>    e.g. https://github.com/thoughtram/clog
        --subtitle <subtitle>        e.g. crazy-release-title
        --to <to>                    e.g. 8057684 (Defaults to HEAD when omitted)
        --setversion <ver>           e.g. 1.0.1
```

### Try it!

1. Clone the repo `git clone https://github.com/thoughtram/clog && cd clog`

2. Build clog `cargo build --release`

3. Delete the old changelog file `rm changelog.md`

3. Run clog `./target/release/clog -r https://github.com/thoughtram/clog --setversion 0.1.0 --subtitle crazy-dog --from 6d8183f`

### Default Options

`clog` can also be configured using a default configuration file so that you don't have to specify all the options each time you want to update your changelog. To do this add a `.clog.toml` file to your repository.

```toml
[clog]
repository = "https://github.com/thoughtram/clog"
subtitle = "my awesome title"

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
