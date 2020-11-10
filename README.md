# kokai

[![Rust](https://github.com/Joxit/kokai/workflows/Rust/badge.svg)](https://github.com/Joxit/kokai/actions?query=workflow%3ARust)
[![Crates.io version shield](https://img.shields.io/crates/v/kokai.svg)](https://crates.io/crates/kokai)
[![Crates.io license shield](https://img.shields.io/crates/l/kokai.svg)](https://crates.io/crates/kokai)


公開: kokai => Release/publish

Kokai allows you to create changelog for your releases and full changelog for your projects. It is based on Conventional Commits (for now) to have a pleasant reading of the changelog.

## All kokai commands

```
kokai 0.1.0

USAGE:
    kokai <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    changelog    Create a full changelog of your project
    help         Prints this message or the help of the given subcommand(s)
    release      Create a release changelog for a specified tag
```

### Release

```
kokai-release 0.1.0
Jones Magloire @Joxit
Create a release changelog for a specified tag

USAGE:
    kokai release [OPTIONS] --tag <tag> [repository]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --name <name>    Explicit name for the release. Useful when tag is a commit or HEAD
        --tag <tag>      Create a release changelog from previous tag until this one. Can be a tag, commit hash or branch

ARGS:
    <repository>    Path to the git repository [default: .]
```

### Changelog

```
kokai-changelog 0.1.0
Jones Magloire @Joxit
Create a full changelog for the full history

USAGE:
    kokai changelog [OPTIONS] [repository]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --tag <tag>    Create a release changelog from the first commit until this tag. Can be a tag, commit hash or branch

ARGS:
    <repository>    Path to the git repository [default: .]
```