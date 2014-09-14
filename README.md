clog
====

A conventional changelog for the rest of us

Try it!

1. Run `cargo build`

2. Run `rm -rf changelog.md && ./target/clog --repository=https://github.com/thoughtram/clog --setversion=0.1.0 --subtitle=crazy-dog`


`--repository` -> sets the repository URL that will be used to create links to commits etc.

`--setversion` -> the version to be used for the header of the changelog

`--subtitle` -> a subtitle to be added as a version suffix