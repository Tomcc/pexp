

# function that takes a name and a var and prints them
syncenv() {
    if [ -z "$1" ] || [ -z "$2" ]; then
        echo "Usage: syncenv <name> <value>"
        return 1
    fi

    syncenv_bin set "$1" "$2"
}

source "$HOME/.syncenvrc"

# Wait in the background until .syncenvrc is touched, then wake up and source it again
watcher() {
    while true; do
        syncenv_bin watch

        echo "Sourcing .syncenvrc..."

        source "$HOME/.syncenvrc"
        cat "$HOME/.syncenvrc"
        echo $TEST10
    done
}

watcher &