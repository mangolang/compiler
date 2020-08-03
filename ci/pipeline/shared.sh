#!/usr/bin/env bash

if [[ ! -v _IS_SHARED_SCRIPT_SOURCED ]]
then
    # This sets the variable for local run. On Github Actions this is not remembered between steps, and
    # neither are the other definitions defined here, so it is correct that most things are repeated.
    _IS_SHARED_SCRIPT_SOURCED='yes'
    # Locally it is always the first run if we get to this point, but on Github Actions it is not.
    # We skip specific steps on subsequent runs, which we detect by existence of the ./artifact directory.
    is_first_run=true
    # shellcheck disable=SC2034
    if [ -d "$(pwd)/artifact" ]; then is_first_run=false; fi

    set -e  # fail if a command fails
    set -E  # technical change so traps work with -E
    set -o pipefail  # also include intermediate commands in -e
    set -u  # undefined variables are errors

    if [[ ! -d '.git' ]] || [[ ! -f 'Cargo.toml' ]]
    then
        printf 'must run from the project root\n' 1>&2
        exit 1
    fi

    # If necessary, build the daily pre-compiled-dependencies image.
    # Ideally this should be downloaded instead of built.
    if ! docker pull 'mangocode/mango_daily_base:stable' || ! docker pull 'mangocode/mango_daily_base:nightly'
    then
        printf '***************************************************************************\n' 1>&2
        printf '* Could not find base Docker image "mangocode/mango_daily_base:stable" !         *\n' 1>&2
        printf '***************************************************************************\n' 1>&2

        exit 2
    fi

    # Make a debug-mode image for further CI steps.
    source "${BASH_SOURCE%/*}/make/debug.sh"

    # Create / clean release directory (this is outside the Docker image)
    CRATE_NAME="$(grep -h -m1 '^name\s*=\s*"[^"]*"' Cargo.toml | sed 's/^name\s*=\s*"\([^"]*\)".*/\1/g')"
    CRATE_VERSION="$(grep -h -m1 '^version\s*=\s*"[^"]*"' Cargo.toml | sed 's/^version\s*=\s*"\([^"]*\)".*/\1/g')"
    GIT_BRANCH="$(git rev-parse --abbrev-ref HEAD | sed 's/_/-/g')"
    if [ "$GIT_BRANCH" = "master" ]; then RELEASE_NAME="${CRATE_NAME}-${CRATE_VERSION}"; else RELEASE_NAME="${CRATE_NAME}-${GIT_BRANCH}-${CRATE_VERSION}-dev"; fi
    printf 'release name: %s\n' "$RELEASE_NAME"
    RELEASE_PATH="$(pwd)/artifact/$RELEASE_NAME"
    # This `if` makes sure cleanup only happens in the first Github Action step.
    if is_first_run; then
        rm -rf "${RELEASE_PATH:?}"
    fi
    # shellcheck disable=SC2174
    mkdir -m770 -p "$(pwd)/artifact"
    # shellcheck disable=SC2174
    mkdir -m770 -p "$RELEASE_PATH"

    # Create a function to run steps inside the image.
    function CHECK() {
        (
            printf "[@mango_ci] $*\n" 1>&2
            docker run --rm -v"$RELEASE_PATH":'/release' 'mango_ci:stable' "$@"
        )
    }
    function CHECK_NIGHTLY() {
        (
            printf "[@mango_ci_nightly] $*\n" 1>&2
            docker run --rm -v"$RELEASE_PATH":'/release' 'mango_ci:nightly' "$@"
        )
    }

    printf 'setup completed\n'
fi
