#!/usr/bin/env bash
# Builds the library. It needs the sibling folder EH3D-COMMON.
set -u
cd "$(dirname "$0")"
pause() { [ -n "${NOPAUSE:-}" ] || { read -r -p "Press Enter to close..." _ || true; }; }
trap pause EXIT
cargo build --release
