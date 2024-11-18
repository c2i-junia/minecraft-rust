#!/usr/bin/env sh

LD_LIBRARY_PATH=$LD_LIBRARY_PATH:$(rustc --print sysroot)/lib:$PWD/target/debug/deps RUST_BACKTRACE=1 RUST_LOG=server=debug,shared=debug,warn ./minecraft-rust-server/bin/minecraft-rust-server --port 8000
