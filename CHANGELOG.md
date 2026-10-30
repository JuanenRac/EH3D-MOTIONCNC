# Changelog

All notable changes to this project are documented here.

## [Unreleased]

- **The session and the path of a program:** `session` runs a program on any controller of `eh3d-controller` (jog, zero, home, MDI, pause, stop, alarm) and `gcode` draws the straight moves of a program; both are tested on the simulated controller (10 tests), and the command line has its own (2).
- **A window with the simulated controller:** `motioncnc` shows the position of the machine (work and machine), a jog pad with its steps and feed, home, zero and unlock, the emergency stop, a line of commands (MDI) with its log, and a G-code program with its path drawn on the table, the lines done marked and the tool where the machine is; it speaks the eight languages and wears the themes of the suite. The program is run line by line, can be paused and stopped, and a line the controller refuses stops it where it is.
## [0.0.2] - The scene moved to the shared view crate, the repository scaffolding in eight languages and the run, build and test scripts

- **A lighter debug build:** the `dev` profile keeps the file and line of a panic (`debug = "line-tables-only"`) but not the full debug symbols, so the `target` folder of a test run is about a tenth of its size.
- Repository scaffolding: the README in eight languages with the project status table, the Architecture and Repository Structure pictures drawn from `docs/architecture.json` and `docs/structure.json`, the CI workflow, the issue and pull-request templates, the code of conduct, the contributing guide, the security policy, the support page and the repository checks in `tools/`.
- The manifest says what the repository is today: a library with no tests of its own, no window and no machine control.
- The view of a job (the scene, the themes and the words of the interface) moved to `eh3d-view` of EH3D-COMMON, where every program of the suite can use it; this crate uses it from there.
- `run` scripts, and `build` and `test` scripts that wait for a key unless `NOPAUSE` is set.

## [0.0.1] - The scene, the themes and the words of the interface

- **The scene of a job on the table:** holes with their drill sizes, routing paths, the progress of the work, a drilling job and a routing job in one scene, the plan view and the isometric view of the machine fitted to the size of the table, and a check that tells what does not fit on the table before a job starts.
- **Themes:** Forge (the default), Neon Arcade, Glass Pro, the classic Windows grey and a theme that follows the colours of Windows (light or dark and the accent colour); a user can write their own in a text file. Every theme is checked for readable contrast.
- **The words of the interface** in English, Spanish, Italian, German, French, Portuguese, Chinese and Japanese.
