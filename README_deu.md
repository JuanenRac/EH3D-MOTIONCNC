<p align="center">
  <img src="images/EH3D_BANNER.svg" alt="EH3D-MOTIONCNC Banner" width="100%">
</p>

# 🧭 EH3D-MOTIONCNC

<p align="center"><a href="README.md">🇺🇸 English</a> | <a href="README_spa.md">🇪🇸 Español</a> | <a href="README_fra.md">🇫🇷 Français</a> | <a href="README_ita.md">🇮🇹 Italiano</a> | 🇩🇪 <b>Deutsch</b> | <a href="README_zho.md">🇨🇳 中文</a> | <a href="README_jpn.md">🇯🇵 日本語</a> | <a href="README_por.md">🇵🇹 Português</a></p>

### Die gemeinsame Basis der EH3D-Maschinensteuerprogramme: die Ansicht eines Auftrags, die Themen und die Texte der Oberfläche in acht Sprachen

<p align="center">
  <img src="https://img.shields.io/badge/License-GPL%203.0--or--later-blue.svg" alt="GPL 3.0 or later">
  <img src="https://img.shields.io/badge/Stack-Rust-DEA584.svg" alt="Rust">
  <img src="https://img.shields.io/badge/Languages-8-00E5FF.svg" alt="Languages">
  <img src="https://img.shields.io/badge/Version-0.0.3-0f6cbd.svg" alt="Version">
  <img src="https://img.shields.io/badge/Maturity-scaffolding-FFB020.svg" alt="Maturity">
  <img src="https://img.shields.io/badge/Tests-22-0f6cbd.svg" alt="Tests">
</p>

---

**Ehrlichkeitscheck - was heute tatsächlich läuft:** ein Rust-Programm mit einem Fenster (`motioncnc`), das eine **simulierte** Maschine steuert: es zeigt die Position der Maschine (Werkstück und Maschine), fährt sie von Hand mit einem Jog-Feld, setzt den Nullpunkt, fährt die Referenz an, nimmt die vom Bediener getippten Befehle an, führt ein G-Code-Programm Zeile für Zeile aus (mit Pause, Stopp und Not-Halt), zeichnet die Bahn des Programms auf den Tisch mit den erledigten Zeilen markiert und hält an, was die Steuerung ablehnt. Das Fenster spricht acht Sprachen und trägt fünf Themen. Die Steuerungsschnittstelle (`eh3d-controller`, in EH3D-COMMON) hat GRBL gegen eine geskriptete Leitung geschrieben und führt die übrigen Familien als noch nicht angesteuert. 22 Tests laufen auf einem Rechner. **Nichts davon lief auf einer echten Maschine**, es wird weder eine serielle Schnittstelle noch eine Netzverbindung geöffnet, und Bögen und die übrigen Codes werden vom Simulator weder gezeichnet noch ausgeführt.

---

> **Sicherheitsstatus:** dieses Programm steuert nur eine simulierte Maschine: es öffnet weder eine serielle Schnittstelle noch eine Netzverbindung. Wenn eine echte Steuerung angeschlossen wird, muss sie sich erst an einem Simulator und dann an einer echten Maschine bewähren, bevor sie als funktionierend gilt; die Endschalter und ein Not-Halt bleiben Aufgabe der eigenen Steuereinheit der Maschine, der Not-Halt des Fensters ist kein Sicherheitskreis.

## 📋 Auf einen Blick

| | |
|---|---|
| Version | `0.0.3` |
| Reifegrad | Gerüst |
| Tests | 22 |
| Technologie-Stack | Rust |
| Auf echter Hardware | Nein: es steuert nur eine simulierte Maschine |

**Springe zu:** [Überblick](#-überblick) · [Architektur](#-architektur) · [Schnellstart](#-schnellstart) · [Tests](#-tests) · [Struktur](#-struktur-des-repositorys) · [Verwandte](#-verwandte-projekte)

## 🚦 Projektstatus

Jede Zeile ist ein Teil dieses Repositorys, und die Ampel zeigt, wie weit er heute ist. Sie beruht auf dem, was das Repository zeigen kann (Tests, Builds, Läufe), nicht auf Plänen.

🟢 Fertig und überprüft · 🟡 Funktioniert, mit bekannten Grenzen · 🟠 Teilweise fertig oder noch nicht überprüft · 🔴 Nicht gemacht oder funktioniert nicht

| Bereich | Stand | Was das hier bedeutet |
|---|---|---|
| Basis-Bibliothek | 🟡 | Ein Rust-Crate, das `scene`, `texts` und `theme` aus `eh3d-view` unter dem Namen dieses Programms erneut exportiert; es braucht den Ordner von EH3D-COMMON daneben. |
| Szene und Ansichten | 🟡 | In EH3D-COMMON geschrieben und getestet; das Fenster dieses Programms zeichnet die Bahn eines Programms auf den Tisch und die Position des Werkzeugs (auf einem Rechner gesehen). |
| Themen und Texte | 🟡 | Fünf Themen und die Wörter der Oberfläche in acht Sprachen, im Fenster dieses Programms gesehen. |
| Tests | 🟡 | 22 Tests bestehen auf einem Rechner (`cargo test --features gui`): die Sitzung (Jog, Nullpunkt, Referenz, ein Programm, Pause, Alarm, Not-Halt), die Bahn eines Programms und die Befehlszeile; die simulierte Steuerung wird in EH3D-COMMON getestet. |
| Fenster und Jog | 🟡 | Läuft auf einem Rechner mit der simulierten Steuerung: Position, Jog-Feld, Schritte, Vorschub, Referenz, Nullpunkt, Entsperren, eine Befehlszeile und ein Protokoll; keine echte Maschine. |
| Maschinensteuerung | 🟠 | Nur der Simulator: ein Programm läuft Zeile für Zeile mit Pause, Stopp und Not-Halt; von hier ist noch keine echte Steuerung angeschlossen. |
| Dokumentation | 🟡 | README in acht Sprachen, die beiden Bilder, drei Beispielprogramme und das Changelog. |

## 🎯 Überblick

* Das **Fenster** (`src/gui.rs`, `motioncnc`): die Position der Maschine (Werkstück und Maschine), ein Jog-Feld mit Schritten und Vorschub, Referenz, Nullpunkt und Entsperren, der Not-Halt, eine Befehlszeile mit Protokoll und das Programm mit seiner Bahn auf dem Tisch.
* Die **Sitzung** (`src/session.rs`): was das Programm mit jeder Steuerung von `eh3d-controller` tut: Jog, Nullpunkt, Referenz, eine Zeile senden, ein Programm Zeile für Zeile ausführen, Pause, Stopp und die Alarme; sie wird an der simulierten Steuerung getestet.
* Die **Bahn eines Programms** (`src/gcode.rs`): die geraden Bewegungen (`G0`, `G1`, absolut oder relativ, in mm oder Zoll), vor dem Lauf gezeichnet; Bögen werden nicht gezeichnet.
* Die **simulierte Steuerung** (`eh3d_controller::sim`, in EH3D-COMMON): eine Maschine im Speicher mit Verfahrgrenzen, die einen Alarm auslösen; die Steuerungsfamilien des Marktes sind in `eh3d-controller` aufgeführt, GRBL ist die erste geschriebene und die anderen erscheinen als noch nicht angesteuert.
* Die **Themen und Wörter** von `eh3d-view`: Forge (Standard), Neon Arcade, Glass Pro, das klassische Windows-Grau und eines, das Windows folgt, und die Oberfläche auf Englisch, Spanisch, Italienisch, Deutsch, Französisch, Portugiesisch, Chinesisch und Japanisch. Was es werden soll: ein Programm zum Bohren, Fräsen und für 2D- und 3D-CNC auf den Engines von EH3D-DRILLPRO und EH3D-ROUTEPRO; dieser Teil existiert noch nicht.
* Die **Menüs, die Einstellungen und die Dateien**: die Menüs Datei, Ansicht, Programm, Maschine und Hilfe mit dem, was PlanetCNC darin hat (öffnen, zuletzt geöffnet, speichern, bearbeiten, die Nullpunkte, die gespeicherten Positionen, die Spindel und das Kühlmittel); ein Einstellungsfenster (Einheiten, Sprache, elf Themen, Verfahrgrenzen, Jog), das zwischen den Läufen gemerkt wird; und die Fenster Öffnen und Speichern der Suite (`eh3d-filedialog`: Orte, Sortierung, Filter, die Endung einer gespeicherten Datei und die Frage vor dem Ersetzen).

## 🧩 Rolle in EH3D

EH3D-MOTIONCNC ist das Maschinensteuerungsprogramm von EH3D, heute auf einer simulierten Maschine. Es hängt per Pfad von EH3D-COMMON ab (der Ordner `EH3D-COMMON` muss neben diesem liegen: die Ansicht, die Themen, die Wörter und die Steuerungen) und noch von keinem anderen EH3D-Repository.

## 🏛️ Architektur

<p align="center">
  <img src="images/architecture.svg" alt="Architektur von EH3D-MOTIONCNC: wer es nutzen wird, was es enthält und worauf es sich stützt" width="100%">
</p>

Wie die Teile zusammenpassen: wer es nutzen wird, was dieses Repository enthält und worauf es sich stützt. Das Bild wird aus `docs/architecture.json` gezeichnet, gegen den Code geprüft und kennzeichnet Künftiges als künftig.

## 🖼️ Screenshots

<p align="center">
  <img src="images/screenshots/program-running.png" alt="a program running" width="100%">
</p>

<p align="center"><em>Ein Programm läuft auf der simulierten Maschine: die erledigten Zeilen sind grün, die aktuelle Zeile hell, die Bahn ist auf den Tisch gezeichnet und das Werkzeug steht, wo die Maschine ist.</em></p>

<p align="center">
  <img src="images/screenshots/spiral-neon.png" alt="Midnight theme" width="100%">
</p>

<p align="center"><em>Das Thema Mitternacht mit einem langen, beendeten Programm.</em></p>

<p align="center">
  <img src="images/screenshots/square-german.png" alt="German interface, light theme" width="100%">
</p>

<p align="center"><em>Die Oberfläche auf Deutsch mit dem hellen Thema Papier: jedes Wort des Fensters folgt der Sprache.</em></p>

<p align="center">
  <img src="images/screenshots/settings-window.png" alt="settings window" width="100%">
</p>

<p align="center"><em>Das Einstellungsfenster, mit den Abschnitten links wie der Baum von PlanetCNC: die Sprache, die elf Themen und was gezeigt wird.</em></p>

<p align="center">
  <img src="images/screenshots/open-window.png" alt="Open window" width="100%">
</p>

<p align="center"><em>Das Fenster Öffnen der Suite: Orte, zuletzt geöffnet, die Liste des Ordners mit Datum und Größe, der Name und der Typ.</em></p>

## 🔒 Sicherheit und Schutz

Dieses Programm öffnet keine serielle Schnittstelle, keine Netzverbindung und keine Dateien anderer Personen, außer der G-Code-Datei, die der Bediener wählt (als Text gelesen, nie von selbst ausgeführt). Es steuert nur eine simulierte Maschine und ist kein Sicherheitssystem. Siehe [SECURITY.md](SECURITY.md).

## 🚀 Schnellstart

Du brauchst eine stabile Rust-Toolchain und den Ordner `EH3D-COMMON` neben diesem.

```bash
cargo build --release --features gui
cargo test --features gui
cargo clippy --all-targets --features gui -- -D warnings
cargo fmt --all --check
python -I tools/ci_validate.py
cargo run --release --features gui -- examples/02_frame_and_pocket.nc
```

`build.bat` / `build.sh` bauen das Programm, `test.bat` / `test.sh` führen die Tests aus und `run.bat` / `run.sh` öffnen das Fenster mit einem Beispielprogramm. Die Skripte warten am Ende auf eine Taste, außer `NOPAUSE` ist gesetzt.

## 🧪 Tests

`cargo test --features gui` führt 12 Tests aus: die Sitzung (Jog und Nullpunkt, ein Programm Zeile für Zeile, Pause und Weiter, eine abgelehnte Zeile, der Not-Halt und der Alarm, das Protokoll der Befehle, die Referenz), die Bahn eines Programms und die Befehlszeile; die Ansicht und die simulierte Steuerung werden in EH3D-COMMON getestet. Die CI führt außerdem `cargo fmt --check` und `cargo clippy` mit Warnungen als Fehlern aus, und `python -I tools/ci_validate.py` prüft Manifest, Versionen, Changelog und die acht READMEs.

## 🏗️ Build und Veröffentlichung

Die Version steht in `Cargo.toml` und in `eh3d.project.json`, und jede Version hat ihren Eintrag in [CHANGELOG.md](CHANGELOG.md). Es gibt noch kein Programm zu verpacken, also keine fertigen Binärdateien.

## 📂 Struktur des Repositorys

<p align="center">
  <img src="images/repository-structure.svg" alt="Repository-Struktur von EH3D-MOTIONCNC" width="100%">
</p>

<details>
<summary>Textversion des Baums</summary>

```text
EH3D-MOTIONCNC/
├── src/
│   ├── lib.rs                                   die Module und die Ansicht von eh3d-view unter dem Namen dieses Programms
│   ├── gui.rs                                   das Fenster: Position, Jog, Programm, Bahn, Befehle und Protokoll
│   ├── session.rs                               was mit einer Steuerung getan wird: Jog, Nullpunkt, Referenz, ein Programm Zeile für Zeile, Pause, Stopp, Alarme
│   ├── gcode.rs                                 die geraden Bewegungen eines G-Code-Programms, um seine Bahn zu zeichnen
│   ├── cli.rs                                   die Befehlszeile: die Datei, die Sprache, das Thema und die Bildoptionen
│   ├── bin/motioncnc.rs                         das Programm, das das Fenster öffnet
│   ├── settings.rs                              was zwischen Läufen gemerkt wird
│   └── gui/dialogs.rs                           Menüs, Einstellungen, Öffnen, Speichern
├── examples/                                    drei Beispiel-G-Code-Programme, vom Quadrat bis zur langen Spirale
├── tools/                                       CI-Prüfung, README-Prüfung und die Bild-Werkzeuge
├── docs/                                        die Quellen der Bilder von Architektur und Repository-Struktur
├── .github/                                     CI-Workflow, Vorlagen für Issues und Pull Requests
├── images/                                      Banner, die beiden Bilder und die Screenshots
├── build.sh build.bat                           bauen die Bibliothek
├── run.sh run.bat                               melden, dass es noch kein Programm gibt, und führen die Tests aus
├── eh3d.project.json                            Manifest: Version, Rolle, Reifegrad und Build-Befehl
├── Cargo.toml Cargo.lock                        das Rust-Paket und seine festgelegten Abhängigkeiten
├── CHANGELOG.md                                 was sich in jeder Version geändert hat
├── SECURITY.md SUPPORT.md                       Schwachstellen melden und Fragen stellen
└── CONTRIBUTING.md CODE_OF_CONDUCT.md LICENSE   Mitwirken, Verhaltenskodex und Lizenz
```

</details>

## 🗺️ Roadmap und bekannte Grenzen

**Bekannte Grenzen heute**

* Nichts davon lief auf einer echten Maschine: die Steuerung ist ein Simulator, und es wird weder eine serielle Schnittstelle noch eine Netzverbindung geöffnet.
* Der Simulator nimmt nur einfachen G-Code (`G0`, `G1`, `G90`, `G91`, `G20`, `G21`, `G92`, `M3` bis `M9`); Bögen, Festzyklen und Tasten werden abgelehnt, und die Bahn zeichnet keine Bögen.
* Es braucht den Ordner `EH3D-COMMON` daneben, weil es per Pfad davon abhängt.

**Als Nächstes**

* Eine echte Steuerung: zuerst GRBL über eine serielle Schnittstelle, dann die anderen Familien, dann die Steuerung von HYDRA-UMC.
* Bögen und der Rest des G-Codes im Simulator und in der Bahn.
* Eine 3D-Ansicht der Bahn mit der Höhe der Schnitte.
* Tasten, Werkzeugwechsler und die Materialien der Maschine.
* Die Engines von EH3D-DRILLPRO und EH3D-ROUTEPRO verbinden, damit ein Programm bohrt und fräst.

## 🌍 Sprachen

README und Texte gibt es auf Englisch, Spanisch, Französisch, Italienisch, Deutsch, Chinesisch, Japanisch und europäischem Portugiesisch.

## 🔗 Verwandte Projekte

* [EH3D-COMMON](https://github.com/JuanenRac/EH3D-COMMON): dort liegen die Ansicht (`eh3d-view`) und das ZIP der Aufträge (`eh3d-archive`)
* [EH3D-DRILLPRO](https://github.com/JuanenRac/EH3D-DRILLPRO): das Bohrprogramm, dessen Motor hier verbunden wird
* [EH3D-ROUTEPRO](https://github.com/JuanenRac/EH3D-ROUTEPRO): das Fräsprogramm, dessen Motor hier verbunden wird

## 📚 Dokumentation

* [Änderungsprotokoll](CHANGELOG.md)
* [Sicherheitsrichtlinie](SECURITY.md) und [Support](SUPPORT.md)
* Die Ansicht eines Auftrags ist in EH3D-COMMON dokumentiert, wo sie liegt.

## 🤝 Mitwirken und Community

* Vor einem Pull Request `cargo fmt --all --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test` und `python -I tools/ci_validate.py` ausführen: alles muss bestehen. Siehe [CONTRIBUTING.md](CONTRIBUTING.md) und den [Verhaltenskodex](CODE_OF_CONDUCT.md).
* Das README gibt es in acht Sprachen: eine Textänderung wird in allen acht Dateien gemacht, und die beiden Bilder werden aus `docs/architecture.json` und `docs/structure.json` mit `tools/make_architecture.py` und `tools/make_structure.py` gezeichnet.
* Beschreibe nichts als funktionierend, was nicht auf einer Maschine gelaufen ist.
* Schwachstellen bitte privat melden: [SECURITY.md](SECURITY.md). Fragen, Ideen und Berichte: electrohobby3d@gmail.com ([SUPPORT.md](SUPPORT.md)).

## 👤 AUTOR

**JuanenRac (Electro Hobby 3D)** · electrohobby3d@gmail.com

## 📜 LIZENZ

Software: GPL-3.0-or-later. Siehe [LICENSE](LICENSE). Hardware-Entwürfe der Suite verwenden CERN OHL-S.
