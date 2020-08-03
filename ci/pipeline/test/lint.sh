#!/usr/bin/env bash

source "${BASH_SOURCE%/*}/../shared.sh"

CHECK cargo --offline clippy --release --all-targets --all-features -- -D warnings
