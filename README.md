clog
====

A [conventional](https://github.com/ajoslin/conventional-changelog/blob/master/CONVENTIONS.md) changelog for the rest of us

### Usage

```
Usage:
  clog [--repository=<link> --setversion=<version> --subtitle=<subtitle>
        --from=<from> --to=<to> --from-latest-tag]

Options:
  -h --help               Show this screen.
  --version               Show version
  -r --repository=<link>  e.g https://github.com/thoughtram/clog
  --setversion=<version>  e.g. 0.1.0
  --subtitle=<subtitle>   e.g. crazy-release-name
  --from=<from>           e.g. 12a8546
  --to=<to>               e.g. 8057684
  --from-latest-tag       uses the latest tag as starting point. Ignores other --from parameter
```

Try it!

1. Build clog `cargo build`

2. Delete the old log file `rm changelog.md`

3. Run clog `./target/clog --repository=https://github.com/thoughtram/clog --setversion=0.1.0 --subtitle=crazy-dog`

## LICENSE

clog is licensed under the MIT Open Source license. For more information, see the LICENSE file in this repository.
