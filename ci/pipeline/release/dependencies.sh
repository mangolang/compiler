#!/usr/bin/env bash

source "${BASH_SOURCE%/*}/../shared.sh"

# Show the dependency tree
CHECK cargo tree --all-features | tee "$RELEASE_PATH/dependency-tree.txt"

# Show the licenses of all the dependencies
CHECK cargo --offline deny list --format=Human --layout=Crate -t 0.95 | tee "$RELEASE_PATH/dependency-licenses.txt"
