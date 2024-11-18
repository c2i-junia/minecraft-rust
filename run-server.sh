#!/usr/bin/env sh

make debug && RUST_LOG=server=debug,shared=debug,warn ./minecraft-rust/bin/minecraft-rust-server
