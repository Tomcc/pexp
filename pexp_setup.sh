#!/bin/bash
# only for syntax highlighting

# pexp_bin wrapper that sets the variable immediately on this session
pexp() {
    # if it doesn't have exactly 2 parameters, print usage and return 1
    if [ $# -ne 2 ]; then
        echo "Usage: pexp <name> <value>"
        return 1
    fi

    export "$1"="$2"

    # set for later and for other sessions
    pexp_bin set "$1" "$2"
}

# pexp_bin wrapper that unsets a variable
pexp_unset() {
    if [ $# -ne 1 ]; then
        echo "Usage: pexp_unset <name>"
        return 1
    fi

    unset "$1"

    # unset for later and for other sessions
    pexp_bin unset "$1"
}

# source the .pexprc file right away, make sure it exists
PEXP_RC="$HOME/.pexprc"
touch "$PEXP_RC"
source "$PEXP_RC"

# set up a trap to source the .pexprc file when it changes
on_change() {
    source "$PEXP_RC"
}

trap on_change SIGUSR2

# Wait in the background until .pexprc is touched, then wake up and source it again
PARENT_PID=$$

# spawn the watcher in the background without printint the PID
(pexp_bin watch $PARENT_PID &)