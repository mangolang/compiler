#!/usr/bin/env bash

source "${BASH_SOURCE%/*}/../shared.sh"

(
    set -e
    cp "${BASH_SOURCE%/*}/../../../README.rst" "$RELEASE_PATH/README.rst"
    cp "${BASH_SOURCE%/*}/../../../LICENSE.txt" "$RELEASE_PATH/LICENSE.txt"
)
