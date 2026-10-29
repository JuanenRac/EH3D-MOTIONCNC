<p align="center">
  <img src="images/EH3D_BANNER.svg" alt="EH3D-MOTIONCNC banner" width="100%">
</p>

# 🧭 EH3D-MOTIONCNC

<p align="center"><a href="README.md">🇺🇸 English</a> | <a href="README_spa.md">🇪🇸 Español</a> | <a href="README_fra.md">🇫🇷 Français</a> | 🇮🇹 <b>Italiano</b> | <a href="README_deu.md">🇩🇪 Deutsch</a> | <a href="README_zho.md">🇨🇳 中文</a> | <a href="README_jpn.md">🇯🇵 日本語</a> | <a href="README_por.md">🇵🇹 Português</a></p>

### La base comune dei programmi di controllo macchina EH3D: la vista di un lavoro, i temi e i testi dell'interfaccia in otto lingue

<p align="center">
  <img src="https://img.shields.io/badge/License-GPL%203.0--or--later-blue.svg" alt="GPL 3.0 or later">
  <img src="https://img.shields.io/badge/Stack-Rust-DEA584.svg" alt="Rust">
  <img src="https://img.shields.io/badge/Languages-8-00E5FF.svg" alt="Languages">
  <img src="https://img.shields.io/badge/Version-0.0.1-0f6cbd.svg" alt="Version">
  <img src="https://img.shields.io/badge/Maturity-scaffolding-FFB020.svg" alt="Maturity">
  <img src="https://img.shields.io/badge/Tests-0-lightgrey.svg" alt="Tests">
</p>

---

**Controllo di onestà - cosa funziona davvero oggi:** un crate Rust che usa la vista di un lavoro di `eh3d-view` di EH3D-COMMON, dove è condivisa con gli altri programmi della suite e ha i suoi test. Quella vista contiene la scena di un lavoro sul piano di lavoro (ogni foro con la dimensione della sua punta, i percorsi di fresatura, a che punto è il lavoro, un lavoro di foratura e uno di fresatura sovrapposti), la vista dall'alto e la vista isometrica della macchina adattate alla dimensione del piano, un controllo che dice cosa non entra nel piano prima di iniziare un lavoro, cinque temi (Forge, Neon Arcade, Glass Pro, il grigio classico di Windows e uno che segue i colori di Windows) e i testi dell'interfaccia in otto lingue. **Non c'è ancora una finestra né il controllo della macchina**, niente di tutto ciò è stato eseguito su una macchina e questo repository non ha test propri: riesporta soltanto la vista.

---

> **Stato di sicurezza funzionale e informatica:** questo repository non è un sistema di sicurezza e non comanda nessuna macchina. Non apre porte seriali né connessioni di rete e non legge file di altre persone. Quando verrà aggiunto il controllo della macchina dovrà dimostrarsi prima su un simulatore e poi su una macchina reale prima di dirsi funzionante, e i finecorsa e un arresto di emergenza restano compito dell'unità di controllo della macchina stessa.

## 📋 In breve

| | |
|---|---|
| Versione | `0.0.1` |
| Maturità | Impalcatura |
| Test | 0 |
| Stack tecnologico | Rust |
| Su hardware reale | No: niente qui si collega a una macchina |

**Vai a:** [Panoramica](#-panoramica) · [Architettura](#-architettura) · [Avvio rapido](#-avvio-rapido) · [Test](#-test) · [Struttura](#-struttura-del-repository) · [Correlati](#-progetti-correlati)

## 🚦 Stato del progetto

Ogni riga è una parte di questo repository e la luce dice a che punto è oggi. È scritta in base a ciò che il repository può mostrare (test, compilazioni, esecuzioni), non a ciò che è previsto.

🟢 Fatto e verificato · 🟡 Funziona, con limiti noti · 🟠 Fatto a metà o non ancora verificato · 🔴 Non fatto o non funzionante

| Area | Stato | Cosa significa qui |
|---|---|---|
| Libreria di base | 🟡 | Un crate Rust che riesporta `scene`, `texts` e `theme` di `eh3d-view` con il nome di questo programma; richiede la cartella di EH3D-COMMON accanto. |
| Scena e viste | 🟡 | Scritte e provate su un computer in EH3D-COMMON: il lavoro sul piano, le viste dall'alto e isometrica, il controllo di ciò che non entra; nessuna finestra le disegna ancora. |
| Temi e testi | 🟡 | Cinque temi con controllo del contrasto e i testi dell'interfaccia in otto lingue, provati in EH3D-COMMON; non ancora visti in una finestra. |
| Test | 🟠 | Questo repository ha 0 test propri (`cargo test`): la vista è testata dove vive, in EH3D-COMMON. |
| Finestra e jog | 🔴 | Non iniziato. |
| Controllo della macchina | 🔴 | Non iniziato: nessuna connessione a una macchina, niente G-code, nessun lavoro di foratura o fresatura parte da qui. |
| Documentazione | 🟡 | README in otto lingue, le due immagini e il registro delle modifiche; c'è poco da documentare finché il programma non esiste. |

## 🎯 Panoramica

* La **libreria di base** (`src/lib.rs`): riesporta `scene`, `texts` e `theme` di `eh3d-view` di EH3D-COMMON, in modo che il futuro programma li usi con il proprio nome e ogni programma della suite mostri un lavoro allo stesso modo.
* La **scena** di un lavoro sul piano: fori per dimensione della punta, percorsi di fresatura, l'avanzamento del lavoro, un lavoro di foratura e uno di fresatura sovrapposti, la vista dall'alto, la vista isometrica della macchina adattata alla dimensione del piano e un controllo di ciò che non entra nel piano.
* I **temi**: Forge (predefinito), Neon Arcade, Glass Pro, il grigio classico di Windows e uno che segue i colori di Windows; ognuno può scrivere il proprio in un file di testo.
* I **testi dell'interfaccia** in inglese, spagnolo, italiano, tedesco, francese, portoghese, cinese e giapponese.
* Cosa sarà: un programma per forare, fresare e fare CNC 2D e 3D, costruito su questa base, sullo ZIP dei lavori di EH3D-COMMON (`eh3d-archive`) e sui motori di EH3D-DRILLPRO ed EH3D-ROUTEPRO. **Niente di tutto ciò esiste ancora.**

## 🧩 Ruolo in EH3D

EH3D-MOTIONCNC sarà il futuro programma di controllo macchina di EH3D e oggi ne è solo la base. Dipende per percorso da EH3D-COMMON (la cartella `EH3D-COMMON` deve stare accanto a questa) e da nessun altro repository EH3D.

## 🏛️ Architettura

<p align="center">
  <img src="images/architecture.svg" alt="Architettura di EH3D-MOTIONCNC: chi lo userà, cosa contiene e su cosa si appoggia" width="100%">
</p>

Come si incastrano le parti: chi lo userà, cosa contiene questo repository e su cosa si appoggia. L'immagine è disegnata da `docs/architecture.json`, verificata rispetto al codice e segna come futuro ciò che è futuro.

## 🔒 Sicurezza e protezione

Questo repository non apre porte seriali, connessioni di rete né file di altre persone; l'unico testo esterno che le sue parti leggono è un file di tema scritto dall'utente, che viene verificato anziché creduto. Non comanda una macchina e non è un sistema di sicurezza. Vedi [SECURITY.md](SECURITY.md).

## 🚀 Avvio rapido

Serve una toolchain Rust stabile e la cartella `EH3D-COMMON` accanto a questa.

```bash
cargo build --release
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --all --check
python -I tools/ci_validate.py
```

`build.bat` / `build.sh` compilano la libreria, `test.bat` / `test.sh` eseguono i test e `run.bat` / `run.sh` avvisano che non c'è ancora un programma ed eseguono i test. Gli script aspettano un tasto alla fine, salvo che `NOPAUSE` sia impostato.

## 🧪 Test

`cargo test` riporta 0 test qui: questo repository riesporta soltanto la vista, i cui test sono in EH3D-COMMON. La CI esegue anche `cargo fmt --check` e `cargo clippy` con gli avvisi come errori, e `python -I tools/ci_validate.py` controlla il manifesto, le versioni, il registro delle modifiche e gli otto README.

## 🏗️ Compilazione e rilascio

La versione sta in `Cargo.toml` e in `eh3d.project.json`, e ogni versione ha la sua voce in [CHANGELOG.md](CHANGELOG.md). Non c'è ancora un programma da impacchettare, quindi niente binari pronti.

## 📂 Struttura del repository

<p align="center">
  <img src="images/repository-structure.svg" alt="Struttura del repository di EH3D-MOTIONCNC" width="100%">
</p>

<details>
<summary>Versione testuale dell'albero</summary>

```text
EH3D-MOTIONCNC/
├── src/
│   └── lib.rs                                   riesporta scena, testi e tema da eh3d-view di EH3D-COMMON
├── tools/                                       controllo di CI, del README e strumenti per le immagini
├── docs/                                        le sorgenti delle immagini di Architettura e Struttura del repository
├── .github/                                     workflow di CI, modelli di issue e pull request
├── images/                                      banner e le immagini di Architettura e Struttura del repository
├── build.sh build.bat                           compilano la libreria
├── run.sh run.bat                               avvisano che non c'è ancora un programma ed eseguono i test
├── eh3d.project.json                            manifesto: versione, ruolo, maturità e comando di compilazione
├── Cargo.toml Cargo.lock                        il pacchetto Rust e le sue dipendenze bloccate
├── CHANGELOG.md                                 cosa è cambiato in ogni versione
├── SECURITY.md SUPPORT.md                       come segnalare una vulnerabilità e dove chiedere
└── CONTRIBUTING.md CODE_OF_CONDUCT.md LICENSE   come contribuire, codice di condotta e licenza
```

</details>

## 🗺️ Roadmap e limiti noti

**Limiti noti oggi**

* Non ci sono finestra né controllo della macchina: niente qui è stato eseguito su una macchina.
* Non ci sono test in questo repository; la vista è testata in EH3D-COMMON.
* Richiede la cartella `EH3D-COMMON` accanto, perché ne dipende per percorso.

**Prossimi passi**

* Studiare i programmi di controllo macchina che esistono oggi (uno nativo e uno .NET) per decidere di cosa ha bisogno l'operatore.
* Una finestra con la scena, i temi e i testi dell'interfaccia.
* Jog e G-code: connessione a una macchina, prima su un simulatore.
* Viste 2D e 3D di un lavoro in corso.
* Unire i motori di EH3D-DRILLPRO ed EH3D-ROUTEPRO perché un solo programma fori e fresi.

## 🌍 Lingue

README e testi esistono in inglese, spagnolo, francese, italiano, tedesco, cinese, giapponese e portoghese europeo.

## 🔗 Progetti correlati

* [EH3D-COMMON](https://github.com/JuanenRac/EH3D-COMMON): dove vivono la vista (`eh3d-view`) e lo ZIP dei lavori (`eh3d-archive`)
* [EH3D-DRILLPRO](https://github.com/JuanenRac/EH3D-DRILLPRO): il programma di foratura il cui motore sarà unito qui
* [EH3D-ROUTEPRO](https://github.com/JuanenRac/EH3D-ROUTEPRO): il programma di fresatura il cui motore sarà unito qui

## 📚 Documentazione

* [Registro delle modifiche](CHANGELOG.md)
* [Politica di sicurezza](SECURITY.md) e [supporto](SUPPORT.md)
* La vista di un lavoro è documentata in EH3D-COMMON, dove vive.

## 🤝 Contribuire e comunità

* Prima di una pull request esegui `cargo fmt --all --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test` e `python -I tools/ci_validate.py`: tutto deve passare. Vedi [CONTRIBUTING.md](CONTRIBUTING.md) e il [codice di condotta](CODE_OF_CONDUCT.md).
* Il README esiste in otto lingue: una modifica al testo si fa in tutti e otto i file, e le due immagini sono disegnate da `docs/architecture.json` e `docs/structure.json` con `tools/make_architecture.py` e `tools/make_structure.py`.
* Non descrivere come funzionante ciò che non è stato eseguito su una macchina.
* Segnala una vulnerabilità in privato: [SECURITY.md](SECURITY.md). Domande, idee e segnalazioni: electrohobby3d@gmail.com ([SUPPORT.md](SUPPORT.md)).

## 👤 AUTORE

**JuanenRac (Electro Hobby 3D)** · electrohobby3d@gmail.com

## 📜 LICENZA

Software: GPL-3.0-or-later. Vedi [LICENSE](LICENSE). I progetti hardware della suite usano CERN OHL-S.
