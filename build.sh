#!/usr/bin/env sh
# Builds the library.
set -e
cd "$(dirname "$0")"
cargo build --release
printf 'Press Enter to close... '
read -r _
