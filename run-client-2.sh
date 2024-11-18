#!/usr/bin/env sh

LD_LIBRARY_PATH=$LD_LIBRARY_PATH:$(rustc --print sysroot)/lib:$PWD/target/debug/deps RUST_BACKTRACE=1 RUST_LOG=client=debug,server=debug,shared=debug,warn ./minecraft-rust-client-2/bin/minecraft-rust
