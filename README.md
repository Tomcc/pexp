# pexp
Persistently EXPort environment vars, across sessions.

`pexp` is a lot like `export`, but it also persists the variable to disk and instantly sets it in every other open session.

Useful for system-wide variables that can change often and are annoying to re-export, like keys, `RUST_BACKTRACE` and so on!

## Usage

```sh
pexp FOO BAR
echo $FOO   # prints "BAR"
```

## How

Hacks, mostly. It creates a `~/.pexprc` file to store every export, and automatically sources it again in the background every time it changes it.

## Installation

Clone this repository, then put 

```bash
source "this_repo/pexp_setup.sh"
```
in your `.bashrc` or `.zshrc`.

## Caveats

It may destroy all your data, fail to work outside MacOS, and hijack your usage of `SIGUSR2`. You've been warned.
