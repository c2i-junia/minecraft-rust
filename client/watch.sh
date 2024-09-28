#!/usr/bin/env sh

# Check if cargo-watch is installed
if ! cargo watch --version &> /dev/null
then
    echo "cargo-watch is not installed. Installing..."
    cargo install cargo-watch
else
    echo "cargo-watch is already installed."
fi

cargo watch --watch src -x "run --features bevy/dynamic_linking"
