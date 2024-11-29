#!/usr/bin/env sh

just create-game-folders

# RUST_BACKTRACE=1 \
RUST_LOG=client=debug,server=debug,shared=debug,warn \
cargo run \
--features=bevy/dynamic_linking \
--bin client -- \
    --game-folder-path ../../minecraft-rust-client-1
