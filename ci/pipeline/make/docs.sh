#!/usr/bin/env bash

source "${BASH_SOURCE%/*}/../shared.sh"

CHECK_NIGHTLY bash -c "\
cargo --offline doc --no-deps --all-features --release; \
cp -r target/doc /release/api-doc; \
chmod 777 -R /release/api-doc"
