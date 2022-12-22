#!/bin/bash
# only for syntax highlighting

# syncenv_bin wrapper that sets the variable immediately on this session
syncenv() {
    if [ -z "$1" ] || [ -z "$2" ]; then
        echo "Usage: syncenv <name> <value>"
        return 1
    fi

    export "$1"="$2"

    # set for later and for other sessions
    syncenv_bin set "$1" "$2"
}

# syncenv_bin wrapper that unsets a variable
syncenv_unset() {
    if [ -z "$1" ]; then
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

trap on_change SIGUSR1

# Wait in the background until .syncenvrc is touched, then wake up and source it again
PARENT_PID=$$
syncenv_watcher() {
    while true; do
        syncenv_bin watch

        kill -s SIGUSR1 $PARENT_PID
    done
}

# spawn the watcher in the background without printint the PID
(syncenv_watcher &)