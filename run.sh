#!/usr/bin/env sh

make debug && RUST_BACKTRACE=1 RUST_LOG=client=debug,server=debug,shared=debug,warn ./minecraft-rust-client/bin/minecraft-rust
