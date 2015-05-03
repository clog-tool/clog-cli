clog
====

[![Join the chat at https://gitter.im/thoughtram/clog](https://badges.gitter.im/Join%20Chat.svg)](https://gitter.im/thoughtram/clog?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

[![Build Status](https://travis-ci.org/thoughtram/clog.png?branch=master)](https://travis-ci.org/thoughtram/clog)

A [conventional](https://github.com/ajoslin/conventional-changelog/blob/master/CONVENTIONS.md) changelog for the rest of us

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
        --from=from                   e.g. 12a8546
    -r, --repository <repository>     e.g. https://github.com/thoughtram/clog
        --setversion <setversion>     e.g. 1.0.1
        --subtitle <subtitle>         e.g. crazy-release-title
        --to <to>                     e.g. 8057684 (Defaults to HEAD when omitted)

```

### Try it!

1. Clone the repo `git clone https://github.com/thoughtram/clog && cd clog`

2. Build clog `cargo build --release`

3. Delete the old changelog file `rm changelog.md`

3. Run clog `./target/release/clog -r https://github.com/thoughtram/clog --setversion 0.1.0 --subtitle crazy-dog --from 88ccacd`

### Default Options

`clog` can also be configured using a default configuration file so that you don't have to specify all the options each time you want to update your changelog. To do this add a `.clog.toml` file to your repository. 

```toml
[clog]
repository = "https://github.com/thoughtram/clog"
subtitle = "my awesome title"
# If you use tags, you can set the following
from-latest-tag = true
```

Now you can update your `changelog.md` with `clog --patch` (assuming you want to update from the latest tag version, and increment your patch version by 1).

*Note:* Any options you specify at the command line will override options set in your `.clog.toml`

#### Custom Sections

When using a `.clog.toml` file you can add your own custom sections to show up in your `changelog.md`. Add a `[sections]` table, along with the sections and aliases you'd like to use:

```toml
[sections]
MySection = ["mysec", "ms"]
```

Now if you make a commit message such as `mysec(Component): some message` or `ms(Component): some message` there will be a new "MySection" section along side the "Features" and "Bug Fixes" areas.

## LICENSE

clog is licensed under the MIT Open Source license. For more information, see the LICENSE file in this repository.
