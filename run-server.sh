#!/usr/bin/env sh

just create-game-folders

# RUST_BACKTRACE=1 \
RUST_LOG=server=debug,server=debug,shared=debug,warn \
cargo run \
--features=bevy/dynamic_linking \
--bin server -- \
    --port 8000 \
    --game-folder-path ../../minecraft-rust-server
