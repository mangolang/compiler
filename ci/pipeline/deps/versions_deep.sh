#!/usr/bin/env bash

source "${BASH_SOURCE%/*}/../shared.sh"

CHECK cargo --offline outdated --exit-code 1
