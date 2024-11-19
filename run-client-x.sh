#!/usr/bin/env sh

sh make-if-needed.sh

if [[ -z "$1" ]]; then
  echo "Error: First argument is required!" >&2
  exit 1
fi

LD_LIBRARY_PATH=$LD_LIBRARY_PATH:$(rustc --print sysroot)/lib:$PWD/target/debug/deps RUST_BACKTRACE=1 RUST_LOG=client=debug,server=debug,shared=debug,warn ./minecraft-rust-client-$1/bin/minecraft-rust
