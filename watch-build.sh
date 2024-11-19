#!/usr/bin/env sh

./install-cargo-watch-if-needed.sh

cargo watch \
    --watch client/src \
    --watch server/src \
    --watch shared/src \
    -- bash -c "./make-if-needed.sh"
