#!/usr/bin/env bash

# Usage: ./cargo_for_coverage.sh test [args]
# This uses grcov and is described in
# https://github.com/mozilla/grcov#example-how-to-generate-gcda-files-for-a-rust-project

if [[ $# -lt 1 || ( "$1" != 'build' && "$1" != 'run' ) ]]; then
    printf "provide argument 'build' or 'run'\n" 1>&2
    exit 1
fi

# shellcheck disable=SC2034
(
    set -eEu -x
    RUSTC_WRAPPER=""
    CARGO_TARGET_DIR="target/coverage"
    CARGO_INCREMENTAL=0
    RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort -Zmacro-backtrace"
    cargo install grcov
    cargo --offline build --verbose --tests --all-targets --all-features
    if [ "$1" = 'run' ]; then
        cargo --offline test --all-targets --all-features
        mkdir -p '/coverage'
        grcov 'target/debug/' -s . -t html --llvm --branch --ignore-not-existing -o '/coverage'
        #cp -r 'target/debug/deps' '/coverage'
    fi
)
