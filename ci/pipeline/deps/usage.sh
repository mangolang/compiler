#!/usr/bin/env bash

source "${BASH_SOURCE%/*}/../shared.sh"

CHECK_NIGHTLY cargo udeps --all-targets --all-features
