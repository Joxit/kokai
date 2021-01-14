# kokai

[![Rust](https://github.com/Joxit/kokai/workflows/Rust/badge.svg)](https://github.com/Joxit/kokai/actions?query=workflow%3ARust)
[![Crates.io version shield](https://img.shields.io/crates/v/kokai.svg)](https://crates.io/crates/kokai)
[![Crates.io license shield](https://img.shields.io/crates/l/kokai.svg)](https://crates.io/crates/kokai)


公開: kokai => Release/publish

Kokai allows you to create changelog for your releases and full changelog for your projects. It is based on Conventional Commits (for now) to have a pleasant reading of the changelog.

## All kokai commands

```
kokai 0.3.1

USAGE:
    kokai <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    changelog     Create a full changelog of your project
    completion    Generate autocompletion file for your shell
    help          Prints this message or the help of the given subcommand(s)
    release       Create a release changelog for a specified tag

```

### Release

```
kokai-release 0.3.1
Jones Magloire @Joxit
Create a release changelog for a specified tag

USAGE:
    kokai release [FLAGS] [OPTIONS] --ref <ref> [repository]

FLAGS:
    -h, --help            Prints help information
        --no-emoji        Remove emojis from headers
        --tag-from-ref    Get the tag of the ref commit and use it as a release name. This is like `git describe --tags
                          --exact-match`
    -V, --version         Prints version information

OPTIONS:
        --add-links <add-links>    Add links to commits/issues/pr with specified url format (github/gitlab...). For
                                   commits only using github url format, use github:commits. For gitlab with commits and
                                   issues use gitlab:commits,issues
        --git-url <git-url>        The git url of the project. Should be a url using http protocol for links
        --name <name>              Explicit name for the release. Useful when tag is a commit or HEAD
        --ref <ref>                Create a release changelog from previous tag until this ref. Can be a tag, commit
                                   hash or branch

ARGS:
    <repository>    Path to the git repository [default: .]
```

### Changelog

```
kokai-changelog 0.3.1
Jones Magloire @Joxit
Create a full changelog of your project

USAGE:
    kokai changelog [OPTIONS] [repository]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --ref <tag>    Create a full changelog from the first commit until this ref. Can be a tag, commit hash or branch

ARGS:
    <repository>    Path to the git repository [default: .]
```