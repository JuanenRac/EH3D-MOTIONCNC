#!/usr/bin/env bash
# Opens the EH3D machine-control window on a sample program (the simulated controller). It needs the sibling folder EH3D-COMMON.
set -u
cd "$(dirname "$0")"
pause() { [ -n "${NOPAUSE:-}" ] || { read -r -p "Press Enter to close..." _ || true; }; }
trap pause EXIT
cargo run --release --features gui -- "${@:-examples/02_frame_and_pocket.nc}"
