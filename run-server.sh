#!/usr/bin/env sh

make debug && RUST_BACKTRACE=1 RUST_LOG=server=debug,shared=debug,warn ./minecraft-rust-server/bin/minecraft-rust-server
