#!/usr/bin/env bash

# Builds an image with pre-compiled dependencies.
#
# This can be built once a day, so that all the CI tasks during the day will be much faster.

(
    set -x
    docker build -t 'mangocode/mango_daily_base:stable' -t 'tmp_mango_daily' -f  'ci/image/base.Dockerfile' .
    docker build -t 'mangocode/mango_daily_base:nightly' -t 'tmp_mango_nightly' -f  'ci/image/base_nightly.Dockerfile' .
)
