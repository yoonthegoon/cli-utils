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

Make sure you've added `source ~/.aliases.sh` to either your `~/.bashrc` or `~/.zshrc` file.
It will also be convenient to run `alias-manager add am alias-manager`.

The outputs of `add`, `edit`, and `remove` are the commands you'd likely want to run to perform the desired action in
the current terminal session.

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
alias venv="python3 -m venv .venv"

$ am edit act "source .venv/bin/activate"
alias act="source .venv/bin/activate"

$ am remove echo
unalias echo

$ am list
name | string
-----+---------------------------
act  | source .venv/bin/activate
venv | python3 -m venv .venv
```

If you'd like to automatically run the outputs of `add`, `edit`, and `remove`, I suggest adding the following to your
`~/.bashrc` or `~/.zshrc` file:

```shell
function alias-manager-wrapper() {
  local output
  output=$(alias-manager "$@")
  if [[ "$1" =~ ^(add|edit|remove)$ ]]; then
    if [ $? -eq 0 ]; then
      eval "$output"
      return
    fi
  fi
  echo "$output"
}
```

and run `am edit am alias-manager-wrapper`.

Now `add`, `edit`, and `remove` won't output anything on success, and the results will be immediate.
