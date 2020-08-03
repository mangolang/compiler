#!/usr/bin/env bash

# Build the current crate's code on top of pre-compiled dependencies.
#
# Note: must be called after `base.sh` (not necessarily directly after).
#
# Note: this is called as part of `shared.sh` so that all steps have this image,
#   but it could also be called directly as a step to check the build.

(
    set -x
    docker build -t 'mango_ci:stable' -t 'tmp_mango_ci' -f  'ci/image/debug.Dockerfile' .
    docker build -t 'mango_ci:nightly' -t 'tmp_mango_ci_nightly' -f  'ci/image/debug_nightly.Dockerfile' .
)
