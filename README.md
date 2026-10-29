# EH3D-MOTIONCNC

The shared base of the EH3D machine-control programs: one place where the drilling and the routing programs, and later the machine-control program that holds both, get the same view of a job, the same themes and the same words in eight languages.

**Honesty check - what actually runs today:** a Rust crate that uses the view of a job from `eh3d-view` of EH3D-COMMON (shared with the other programs of the suite, with its tests there). That view has the scene of a job on the work table (every hole with its drill size, the routing paths, how far the work has got, a drilling job and a routing job on top of each other), the plan view from above and the isometric view of the machine fitted to the table size of the configuration, a check that says what does not fit on the table before a job starts, five themes (Forge, Neon Arcade, Glass Pro, the classic Windows grey and one that follows the colours of Windows) and the words of the interface in English, Spanish, Italian, German, French, Portuguese, Chinese and Japanese. **There is no window and no machine control yet**; nothing here has run on a machine.

## Project status

| Area | State | What it means here |
|---|---|---|
| Scene and views | 🟡 | Written and tested on a computer; no window draws them yet. |
| Themes | 🟡 | Written and tested on a computer (contrast, the Windows colours read from their output); not seen in a window yet. |
| Machine control | 🔴 | Not started. |

## Build and test

```
cargo test
```

`build.bat` / `build.sh` build the library, `test.bat` / `test.sh` run the tests and `run.bat` / `run.sh` say that there is no program yet and run the tests. It needs the sibling folder `EH3D-COMMON` to build.

## Licence

GPL-3.0-or-later. See `LICENSE`.
