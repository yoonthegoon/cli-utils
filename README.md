# cli-utils

Command line interface utilities

This is a collection of some small executables I wrote that I find useful.
They're all relatively small and share a lot of the same dependencies, so I bundled them all together in this
repository.
If you find that you, for some reason, don't want some but want others, simply run `rm ~/.cargo/bin/<bin>`.

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
    - [alias-manager](#alias-manager)

## Installation

```shell
cargo install --git https://github.com/yoonthegoon/cli-utils.git
```

## Usage

### alias-manager

```console
$ alias-manager help
Usage: alias-manager <COMMAND>

Commands:
  add     Add a managed alias
  edit    Edit a managed alias
  list    List all managed aliases
  remove  Remove a managed alias
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help

$ alias-manager list
Name | String
-----+--------------------------
am   | alias-manager
act  | source venv/bin/activate
echo | cowsay

$ am add venv "python3 -m venv .venv"

$ am edit act "source .venv/bin/activate"

$ am remove echo

$ am list
name | string
-----+---------------------------
act  | source .venv/bin/activate
venv | python3 -m venv .venv
```
