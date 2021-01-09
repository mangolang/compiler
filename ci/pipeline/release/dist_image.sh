#!/usr/bin/env bash

source "${BASH_SOURCE%/*}/../shared.sh"

(
    set -x
    docker build -t 'mangocode/mango:latest' -t 'tmp_mango_exe' -f  'ci/image/executable.Dockerfile' .
    docker run --rm --read-only --tmpfs /tmp mangocode/mango:latest -h
    docker save 'mangocode/mango:latest' -o "$RELEASE_PATH/mango.docker-image"
)
