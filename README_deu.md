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
  <img src="https://img.shields.io/badge/Version-0.0.2-0f6cbd.svg" alt="Version">
  <img src="https://img.shields.io/badge/Maturity-scaffolding-FFB020.svg" alt="Maturity">
  <img src="https://img.shields.io/badge/Tests-0-lightgrey.svg" alt="Tests">
</p>

---

**Ehrlichkeitscheck - was heute wirklich läuft:** ein Rust-Crate, das die Ansicht eines Auftrags aus `eh3d-view` von EH3D-COMMON verwendet, wo sie mit den anderen Programmen der Suite geteilt wird und ihre Tests hat. Diese Ansicht enthält die Szene eines Auftrags auf dem Arbeitstisch (jedes Loch mit seiner Bohrergröße, die Frässpuren, wie weit die Arbeit ist, ein Bohr- und ein Fräsauftrag übereinander), die Draufsicht und die isometrische Ansicht der Maschine, angepasst an die Tischgröße, eine Prüfung, die vor dem Start sagt, was nicht auf den Tisch passt, fünf Themen (Forge, Neon Arcade, Glass Pro, das klassische Windows-Grau und eines, das den Windows-Farben folgt) und die Texte der Oberfläche in acht Sprachen. **Es gibt noch kein Fenster und keine Maschinensteuerung**, nichts davon lief auf einer Maschine und dieses Repository hat keine eigenen Tests: es exportiert die Ansicht nur erneut.

---

> **Stand von Sicherheit und IT-Sicherheit:** dieses Repository ist kein Sicherheitssystem und steuert keine Maschine. Es öffnet keine serielle Schnittstelle und keine Netzwerkverbindung und liest keine Dateien anderer Personen. Wenn die Maschinensteuerung hinzukommt, muss sie sich erst im Simulator und dann an einer echten Maschine bewähren, bevor sie als funktionierend gilt; Endschalter und Not-Halt bleiben Aufgabe der Steuerung der Maschine selbst.

## 📋 Auf einen Blick

| | |
|---|---|
| Version | `0.0.2` |
| Reifegrad | Gerüst |
| Tests | 0 |
| Technologie-Stack | Rust |
| Auf echter Hardware | Nein: nichts hier verbindet sich mit einer Maschine |

**Springe zu:** [Überblick](#-überblick) · [Architektur](#-architektur) · [Schnellstart](#-schnellstart) · [Tests](#-tests) · [Struktur](#-struktur-des-repositorys) · [Verwandte](#-verwandte-projekte)

## 🚦 Projektstatus

Jede Zeile ist ein Teil dieses Repositorys, und die Ampel zeigt, wie weit er heute ist. Sie beruht auf dem, was das Repository zeigen kann (Tests, Builds, Läufe), nicht auf Plänen.

🟢 Fertig und überprüft · 🟡 Funktioniert, mit bekannten Grenzen · 🟠 Teilweise fertig oder noch nicht überprüft · 🔴 Nicht gemacht oder funktioniert nicht

| Bereich | Stand | Was das hier bedeutet |
|---|---|---|
| Basis-Bibliothek | 🟡 | Ein Rust-Crate, das `scene`, `texts` und `theme` aus `eh3d-view` unter dem Namen dieses Programms erneut exportiert; es braucht den Ordner von EH3D-COMMON daneben. |
| Szene und Ansichten | 🟡 | In EH3D-COMMON geschrieben und auf einem Computer getestet: der Auftrag auf dem Tisch, Drauf- und isometrische Ansicht, die Prüfung dessen, was nicht passt; noch zeichnet kein Fenster sie. |
| Themen und Texte | 🟡 | Fünf Themen mit Kontrastprüfung und die Texte der Oberfläche in acht Sprachen, in EH3D-COMMON getestet; noch nicht in einem Fenster gesehen. |
| Tests | 🟠 | Dieses Repository hat 0 eigene Tests (`cargo test`): die Ansicht wird dort getestet, wo sie liegt, in EH3D-COMMON. |
| Fenster und Jog | 🔴 | Nicht begonnen. |
| Maschinensteuerung | 🔴 | Nicht begonnen: keine Verbindung zu einer Maschine, kein G-Code, kein Bohr- oder Fräsauftrag läuft von hier aus. |
| Dokumentation | 🟡 | README in acht Sprachen, die beiden Bilder und das Änderungsprotokoll; bis das Programm existiert, gibt es wenig zu dokumentieren. |

## 🎯 Überblick

* Die **Basis-Bibliothek** (`src/lib.rs`): exportiert `scene`, `texts` und `theme` aus `eh3d-view` von EH3D-COMMON erneut, damit das künftige Programm sie unter eigenem Namen nutzt und jedes Programm der Suite einen Auftrag gleich zeigt.
* Die **Szene** eines Auftrags auf dem Arbeitstisch: Löcher nach Bohrergröße, Frässpuren, der Fortschritt der Arbeit, ein Bohr- und ein Fräsauftrag übereinander, die Draufsicht, die isometrische Ansicht der Maschine passend zur Tischgröße und eine Prüfung dessen, was nicht auf den Tisch passt.
* Die **Themen**: Forge (Standard), Neon Arcade, Glass Pro, das klassische Windows-Grau und eines, das den Windows-Farben folgt; eigene Themen schreibt man in einer Textdatei.
* Die **Texte der Oberfläche** auf Englisch, Spanisch, Italienisch, Deutsch, Französisch, Portugiesisch, Chinesisch und Japanisch.
* Was es werden soll: ein Programm zum Bohren, Fräsen und für 2D- und 3D-CNC-Arbeit, gebaut auf dieser Basis, auf dem ZIP der Aufträge von EH3D-COMMON (`eh3d-archive`) und auf den Motoren von EH3D-DRILLPRO und EH3D-ROUTEPRO. **Nichts davon gibt es bisher.**

## 🧩 Rolle in EH3D

EH3D-MOTIONCNC ist das künftige Maschinensteuerprogramm von EH3D und heute nur dessen Basis. Es hängt per Pfad von EH3D-COMMON ab (der Ordner `EH3D-COMMON` muss neben diesem liegen) und von keinem anderen EH3D-Repository.

## 🏛️ Architektur

<p align="center">
  <img src="images/architecture.svg" alt="Architektur von EH3D-MOTIONCNC: wer es nutzen wird, was es enthält und worauf es sich stützt" width="100%">
</p>

Wie die Teile zusammenpassen: wer es nutzen wird, was dieses Repository enthält und worauf es sich stützt. Das Bild wird aus `docs/architecture.json` gezeichnet, gegen den Code geprüft und kennzeichnet Künftiges als künftig.

## 🔒 Sicherheit und Schutz

Dieses Repository öffnet keine serielle Schnittstelle, keine Netzwerkverbindung und keine Dateien anderer Personen; der einzige externe Text, den seine Teile lesen, ist eine vom Benutzer geschriebene Themendatei, die geprüft statt vertraut wird. Es steuert keine Maschine und ist kein Sicherheitssystem. Siehe [SECURITY.md](SECURITY.md).

## 🚀 Schnellstart

Du brauchst eine stabile Rust-Toolchain und den Ordner `EH3D-COMMON` neben diesem.

```bash
cargo build --release
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --all --check
python -I tools/ci_validate.py
```

`build.bat` / `build.sh` bauen die Bibliothek, `test.bat` / `test.sh` führen die Tests aus und `run.bat` / `run.sh` melden, dass es noch kein Programm gibt, und führen die Tests aus. Die Skripte warten am Ende auf eine Taste, außer `NOPAUSE` ist gesetzt.

## 🧪 Tests

`cargo test` meldet hier 0 Tests: dieses Repository exportiert die Ansicht nur erneut, deren Tests in EH3D-COMMON liegen. Die CI führt außerdem `cargo fmt --check` und `cargo clippy` mit Warnungen als Fehlern aus, und `python -I tools/ci_validate.py` prüft Manifest, Versionen, Änderungsprotokoll und die acht READMEs.

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
│   └── lib.rs                                   exportiert Szene, Texte und Thema aus eh3d-view von EH3D-COMMON erneut
├── tools/                                       CI-Prüfung, README-Prüfung und die Bild-Werkzeuge
├── docs/                                        die Quellen der Bilder von Architektur und Repository-Struktur
├── .github/                                     CI-Workflow, Vorlagen für Issues und Pull Requests
├── images/                                      Banner und die Bilder von Architektur und Repository-Struktur
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

* Es gibt kein Fenster und keine Maschinensteuerung: nichts hier lief auf einer Maschine.
* In diesem Repository gibt es keine Tests; die Ansicht wird in EH3D-COMMON getestet.
* Es braucht den Ordner `EH3D-COMMON` daneben, weil es per Pfad davon abhängt.

**Als Nächstes**

* Die heute existierenden Maschinensteuerprogramme (ein natives und ein .NET-Programm) untersuchen, um zu entscheiden, was der Bediener braucht.
* Ein Fenster mit der Szene, den Themen und den Texten der Oberfläche.
* Jog und G-Code: Verbindung zu einer Maschine, zuerst in einem Simulator.
* 2D- und 3D-Ansichten eines laufenden Auftrags.
* Die Motoren von EH3D-DRILLPRO und EH3D-ROUTEPRO verbinden, damit ein Programm bohrt und fräst.

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
