#!/usr/bin/env sh

sh make-if-needed.sh

PORT=${1:-8000}

LD_LIBRARY_PATH=$LD_LIBRARY_PATH:$(rustc --print sysroot)/lib:$PWD/target/debug/deps:$(rustc --print sysroot)/lib/rustlib/x86_64-unknown-linux-gnu/lib RUST_BACKTRACE=1 RUST_LOG=server=debug,shared=debug,warn ./minecraft-rust-server/bin/minecraft-rust-server --port $PORT
