#!/usr/bin/env bash

# Run tests using miri to detect more possible issues.

source "${BASH_SOURCE%/*}/../shared.sh"

CHECK_NIGHTLY cargo --offline miri test --all-targets --all-features
