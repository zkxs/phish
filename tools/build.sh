#!/bin/bash
# assumes crinkler is installed next to this script in the 'crinkler30a' directory
TOOLS_DIR="$(readlink -f "$(dirname "$0")")"
#echo "TOOLS_DIR=$TOOLS_DIR"
CRINKLER_PATH="$TOOLS_DIR/crinkler30a/Win64"
#echo "CRINKLER_PATH=$CRINKLER_PATH"
export PATH="$CRINKLER_PATH:$PATH"
cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features="optimize_for_size" --target i686-pc-windows-msvc --release
