#!/usr/bin/env bash

source "${BASH_SOURCE%/*}/../shared.sh"

CHECK cargo --offline deny check licenses
