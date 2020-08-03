#!/usr/bin/env bash

source "${BASH_SOURCE%/*}/../shared.sh"

INFO_PTH="${RELEASE_PATH}/executable-info.txt"

printf '\n== size ==\n' > "$INFO_PTH"
CHECK bash -c 'du -h "$(find . -wholename "*/debug/*" -name "mango" -type f -executable)"' | tee -a "$INFO_PTH"
CHECK bash -c 'du -h "$(find . -wholename "*/release/*" -name "mango" -type f -executable)"' | tee -a "$INFO_PTH"

printf '\n== crates ==\n' >> "$INFO_PTH"
CHECK cargo --offline bloat --release --crates --all-features --wide -n 50 | tee -a "$INFO_PTH"

printf '\n== functions ==\n' >> "$INFO_PTH"
CHECK cargo --offline bloat --release --all-features --wide -n 30 | tee -a "$INFO_PTH"

printf '\n== linking ==\n' >> "$INFO_PTH"
CHECK bash -c 'ldd "$(find . -wholename "*/release/*" -name "mango" -type f -executable)" || printf "ldd failed; perhaps no dynamic linking\\n"' | tee -a "$INFO_PTH"
