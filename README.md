clog
====

A [conventional](https://github.com/ajoslin/conventional-changelog/blob/master/CONVENTIONS.md) changelog for the rest of us

### Usage

```
USAGE:
    clog [FLAGS] [OPTIONS] 

FLAGS:
        --from-latest-tag    uses the latest tag as starting point (ignores other --from parameters)
    -v, --version            Prints version information
    -h, --help               Prints this message

OPTIONS:
        --subtitle=subtitle         e.g. crazy-release-title
    -r, --repository=repository     e.g. https://github.com/thoughtram/clog
        --setversion=setversion     e.g. 1.0.1
        --to=to                     e.g. 8057684 (Defaults to HEAD when omitted)
        --from=from                 e.g. 12a8546
```

Try it!

1. Clone the repo `git clone https://github.com/thoughtram/clog && cd clog`

2. Build clog `cargo build --release`

3. Delete the old changelog file `rm changelog.md`

3. Run clog `./target/release/clog --repository=https://github.com/thoughtram/clog --setversion=0.1.0 --subtitle=crazy-dog`

## LICENSE

clog is licensed under the MIT Open Source license. For more information, see the LICENSE file in this repository.
