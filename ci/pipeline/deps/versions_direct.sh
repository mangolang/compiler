#!/usr/bin/env bash

source "${BASH_SOURCE%/*}/../shared.sh"

CHECK cargo --offline outdated --depth 1 --exit-code 1
CHECK cargo --offline deny check bans
