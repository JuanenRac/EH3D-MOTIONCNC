@echo off
rem Opens the EH3D machine-control window on a sample program (the simulated controller). It needs the sibling folder EH3D-COMMON.
cd /d "%~dp0"
if "%~1"=="" (
  cargo run --release --features gui -- examples\02_frame_and_pocket.nc
) else (
  cargo run --release --features gui -- %*
)
if not defined NOPAUSE pause
