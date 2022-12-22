#!/bin/bash
# only for syntax highlighting

# syncenv_bin wrapper that sets the variable immediately on this session
syncenv() {
    # if it doesn't have exactly 2 parameters, print usage and return 1
    if [ $# -ne 2 ]; then
        echo "Usage: syncenv <name> <value>"
        return 1
    fi

    export "$1"="$2"

    # set for later and for other sessions
    syncenv_bin set "$1" "$2"
}

# syncenv_bin wrapper that unsets a variable
syncenv_unset() {
    if [ $# -ne 1 ]; then
        echo "Usage: syncenv_unset <name>"
        return 1
    fi

    unset "$1"

    # unset for later and for other sessions
    syncenv_bin unset "$1"
}

# source the .syncenvrc file right away
source "$HOME/.syncenvrc"

# set up a trap to source the .syncenvrc file when it changes
on_change() {
    source "$HOME/.syncenvrc"
}

trap on_change SIGUSR2

# Wait in the background until .syncenvrc is touched, then wake up and source it again
PARENT_PID=$$

# spawn the watcher in the background without printint the PID
(syncenv_bin watch $PARENT_PID &)