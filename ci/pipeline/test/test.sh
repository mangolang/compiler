#!/usr/bin/env bash

# Run unit tests. There is a separate script to run tests with miri.

source "${BASH_SOURCE%/*}/../shared.sh"

CHECK cargo --offline test --all-targets --all-features

#TODO: maybe this should be split into `unit`, `example`, `doc` and `integration`, but that seems inconvenient to achieve currently
