#!/usr/bin/env bash

source "${BASH_SOURCE%/*}/../shared.sh"

CHECK cargo --offline audit --deny-warnings
CHECK cargo --offline deny check advisories
