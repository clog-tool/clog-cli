clog
====

A conventional changelog for the rest of us

### Usage

```
Usage:
  clog [--repository=<link> --setversion=<version> --subtitle=<subtitle> --from=<from> --to=<to>]

Options:
  -h --help               Show this screen.
  --version               Show version
  --repository=<link>     e.g https://github.com/thoughtram/clog
  --setversion=<version>  e.g. 0.1.0
  --subtitle=<subtitle>   e.g. crazy-release-name
  --from=<from>           e.g. 12a8546
  --to=<to>               e.g. 8057684
``

Try it!

1. Run `cargo build`

2. Run `rm -rf changelog.md && ./target/clog --repository=https://github.com/thoughtram/clog --setversion=0.1.0 --subtitle=crazy-dog`