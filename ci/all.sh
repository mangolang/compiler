#!/usr/bin/env bash

set -eEu

function STEP() {
    if [ $# -lt 2 ]; then
        printf "STEP expects 2 arguments: name and script\n" 1>&2
        return 1
    fi
    pth="${BASH_SOURCE%/*}/pipeline/$1"
    if [ ! -f "$pth" ]; then
        printf "STEP script '%s' does not exist at '%s'\n" "$1" "$pth" 1>&2
        return 1
    fi
    printf '\n=== step: %s (%s) ===\n' "$2" "$pth" 1>&2
    source "$pth"
}

# Note: this must be the first step
STEP 'make/base.sh' 'build - dependencies image'

STEP 'make/debug.sh' 'build - ci image'

STEP 'test/test.sh' 'test'

STEP 'test/lint.sh' 'lint'
STEP 'test/lint.sh' 'lint'

STEP 'test/style.sh' 'style'

#TODO @mark: turn on
#STEP 'test/test_miri.sh' 'test (miri)'

STEP 'test/cov.sh' 'coverage'

STEP 'deps/versions_direct.sh' 'dependencies - versions'
#TODO: do something with indirect dependencies, version_deep

STEP 'deps/audit.sh' 'dependencies - audit'

#TODO @mark: turn on
#STEP 'deps/license.sh' 'dependencies - license'

#TODO @mark: turn on
#STEP 'deps/usage.sh' 'dependencies - unused'

STEP 'make/docs.sh' 'documentation'

printf '== cleanup ==\n'
# Untag the docker images so next run cannot accidentally rely on old versions.
docker rmi 'mangocode/mango_daily_base:stable'
docker rmi 'mangocode/mango_daily_base:nightly'
docker rmi 'mango_ci:stable'
docker rmi 'mango_ci:nightly'
#docker rmi 'mangocode/mango:latest'

printf '== done ==\n'
