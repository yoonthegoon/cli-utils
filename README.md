# cli-utils

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
Usage:
  alias-manager <command>

Example usage:
  alias-manager add <name> <string>
  alias-manager edit <name> <string>
  alias-manager list
  alias-manager remove <name>

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
