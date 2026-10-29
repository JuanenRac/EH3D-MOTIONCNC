@echo off
rem Builds the library. It needs the sibling folder EH3D-COMMON.
cd /d "%~dp0"
cargo build --release
if not defined NOPAUSE pause
