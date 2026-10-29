# Changelog

All notable changes to this project are documented here.

## [Unreleased]

- The view of a job (the scene, the themes and the words of the interface) moved to `eh3d-view` of EH3D-COMMON, where every program of the suite can use it; this crate uses it from there.
- `run` scripts, and `build` and `test` scripts that wait for a key unless `NOPAUSE` is set.

## [0.0.1] - The scene, the themes and the words of the interface

- **The scene of a job on the table:** holes with their drill sizes, routing paths, the progress of the work, a drilling job and a routing job in one scene, the plan view and the isometric view of the machine fitted to the size of the table, and a check that tells what does not fit on the table before a job starts.
- **Themes:** Forge (the default), Neon Arcade, Glass Pro, the classic Windows grey and a theme that follows the colours of Windows (light or dark and the accent colour); a user can write their own in a text file. Every theme is checked for readable contrast.
- **The words of the interface** in English, Spanish, Italian, German, French, Portuguese, Chinese and Japanese.
