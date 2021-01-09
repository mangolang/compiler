#!/usr/bin/env bash

source "${BASH_SOURCE%/*}/../shared.sh"

function subcmd_help() {
    # arg1: subcommands
    if [[ "$1" != "" ]]; then
        printf 'help for subcommand%s\n' "$1"
        printf '\n\n== subcommand:%s ==\n\n' "$1" >> "$RELEASE_PATH/cli-help.txt"
    fi
    CHECK_NIGHTLY bash -c "cargo run -q -- $1 --help >> /release/cli-help.txt"
    sub_commands="$(CHECK_NIGHTLY bash -c "cargo run -q -- $1 --help" | python -c "
from sys import stdin
lines = stdin.readlines()
# Find the start of 'subcommands' section
subcommand_start = 0
for i, line in enumerate(lines):
    if 'SUBCOMMANDS:' in line:
        subcommand_start = i + 1
        break
# Iterate over the subcommands, until empty line
if subcommand_start:
    for i in range(subcommand_start, len(lines)):
        line = lines[i]
        if len(line) >= 5 and line[5] != ' ':
            cmd = line.split()[0].strip()
            if cmd != 'help':
                print('{}'.format(cmd))
        if not line.strip():
            break
")"
    for sub_cmd in $sub_commands
    do
        subcmd_help "$1 $sub_cmd"
    done
}

printf '== CLI API HELP ==\n\n' > "$RELEASE_PATH/cli-help.txt"
subcmd_help ""
