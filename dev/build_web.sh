#!/usr/bin/env bash

pth="$(dirname $(dirname $(realpath -s $0)))"
echo $pth

set -e  # stop on error
set -o xtrace  # print commands

# Compile rust to wasm
# TODO: need release as workaround for a compile error in dependency; remove when that is resolved
cargo +nightly build --target wasm32-unknown-unknown --release

# Copy the static files
outdir="$pth/target/deploy"
mkdir -p "$outdir"
cp -rf "$pth/static"/* "$outdir"

# Prepare the wasm for use from js
wasm-bindgen --debug --no-modules --no-typescript "$pth/target/wasm32-unknown-unknown/release/mango.wasm" --out-dir "$outdir"

# Start a simple webserver
(
    cd "$outdir"
    python -m SimpleHTTPServer
)
