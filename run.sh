#!/usr/bin/env bash
# EH3D-MOTIONCNC has no program to run yet: so far it is the base (the view of a job, the themes and the words of the interface, shared
# through EH3D-COMMON). This script says so and runs the tests, which are what can be run today.
set -u
cd "$(dirname "$0")"
pause() { [ -n "${NOPAUSE:-}" ] || { read -r -p "Press Enter to close..." _ || true; }; }
trap pause EXIT
echo "EH3D-MOTIONCNC has no program yet (it is the base of the future machine-control program)."
echo "For the drilling and routing windows run EH3D-BUNGARD-GUI. Running the tests here instead:"
echo
cargo test
