#!/usr/bin/env bash

source "${BASH_SOURCE%/*}/../shared.sh"

CHECK_NIGHTLY bash -c "\
cargo --offline test --all-targets --all-features --quiet;\
grcov target/debug/ -s . -t html --llvm --branch -o ./target/debug/coverage/;\
cp target/debug/coverage/index.html /release/coverage.html"
if [ ! -f "$RELEASE_PATH/coverage.html" ]; then
    printf 'failed to generate coverage\n' 1>&2
    exit 1
fi
