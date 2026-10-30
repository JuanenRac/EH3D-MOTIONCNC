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
  <img src="https://img.shields.io/badge/Version-0.0.4-0f6cbd.svg" alt="Version">
  <img src="https://img.shields.io/badge/Maturity-scaffolding-FFB020.svg" alt="Maturity">
  <img src="https://img.shields.io/badge/Tests-25-0f6cbd.svg" alt="Tests">
</p>

---

**Verifica di onestà - cosa funziona oggi:** un programma Rust con una finestra (`motioncnc`) che pilota una macchina **simulata**: mostra la posizione della macchina (pezzo e macchina), la sposta a mano con un tastierino di jog, imposta lo zero, esegue l'origine, accetta i comandi digitati dall'operatore, esegue un programma G-code riga per riga (con pausa, stop e arresto di emergenza), disegna il percorso del programma sul piano con le righe eseguite segnate e si ferma su ciò che il controllore rifiuta. La finestra parla otto lingue e porta cinque temi. L'interfaccia dei controllori (`eh3d-controller`, in EH3D-COMMON) ha GRBL scritto contro una linea simulata ed elenca le altre famiglie come non ancora pilotate. 25 test girano su un computer. **Nulla di qui è stato eseguito su una macchina reale**, non si apre alcuna porta seriale né connessione di rete, e gli archi e gli altri codici il simulatore non li disegna né li esegue.

---

> **Stato di sicurezza:** questo programma pilota solo una macchina simulata: non apre alcuna porta seriale né connessione di rete. Quando sarà collegato un controllore vero dovrà dimostrarsi su un simulatore e poi su una macchina reale prima di dirsi funzionante, e i finecorsa e un arresto di emergenza restano compito dell'unità di controllo della macchina; l'arresto di emergenza della finestra non è un circuito di sicurezza.

## 📋 In breve

| | |
|---|---|
| Versione | `0.0.4` |
| Maturità | Impalcatura |
| Test | 25 |
| Stack tecnologico | Rust |
| Su hardware reale | No: pilota solo una macchina simulata |

**Vai a:** [Panoramica](#-panoramica) · [Architettura](#-architettura) · [Avvio rapido](#-avvio-rapido) · [Test](#-test) · [Struttura](#-struttura-del-repository) · [Correlati](#-progetti-correlati)

## 🚦 Stato del progetto

Ogni riga è una parte di questo repository e la luce dice a che punto è oggi. È scritta in base a ciò che il repository può mostrare (test, compilazioni, esecuzioni), non a ciò che è previsto.

🟢 Fatto e verificato · 🟡 Funziona, con limiti noti · 🟠 Fatto a metà o non ancora verificato · 🔴 Non fatto o non funzionante

| Area | Stato | Cosa significa qui |
|---|---|---|
| Libreria di base | 🟡 | Un crate Rust che riesporta `scene`, `texts` e `theme` di `eh3d-view` con il nome di questo programma; richiede la cartella di EH3D-COMMON accanto. |
| Scena e viste | 🟡 | Scritta e provata in EH3D-COMMON; la finestra di questo programma disegna il percorso di un programma sul piano e la posizione dell'utensile (vista su un computer). |
| Temi e testi | 🟡 | Cinque temi e le parole dell'interfaccia in otto lingue, viste nella finestra di questo programma. |
| Test | 🟡 | 25 test passano su un computer (`cargo test --features gui`): la sessione (jog, zero, origine, un programma, pausa, allarme, arresto di emergenza), il percorso di un programma e la riga di comando; il controllore simulato è provato in EH3D-COMMON. |
| Finestra e jog | 🟡 | Funziona su un computer con il controllore simulato: posizione, tastierino di jog, passi, avanzamento, origine, zero, sblocco, una riga di comandi e un registro; nessuna macchina reale. |
| Controllo della macchina | 🟠 | Solo il simulatore: un programma gira riga per riga con pausa, stop e arresto di emergenza; da qui non è ancora collegato alcun controllore reale. |
| Documentazione | 🟡 | README in otto lingue, le due immagini, tre programmi di esempio e il changelog. |

## 🎯 Panoramica

* La **finestra** (`src/gui.rs`, `motioncnc`): la posizione della macchina (pezzo e macchina), un tastierino di jog con passi e avanzamento, origine, zero e sblocco, l'arresto di emergenza, una riga di comandi con il suo registro e il programma con il suo percorso sul piano.
* La **sessione** (`src/session.rs`): ciò che il programma fa con qualsiasi controllore di `eh3d-controller`: jog, zero, origine, inviare una riga, eseguire un programma riga per riga, pausa, stop e gli allarmi; è provata sul controllore simulato.
* Il **percorso di un programma** (`src/gcode.rs`): i movimenti rettilinei (`G0`, `G1`, assoluti o relativi, in mm o pollici) disegnati prima dell'esecuzione; gli archi non sono disegnati.
* Il **controllore simulato** (`eh3d_controller::sim`, in EH3D-COMMON): una macchina in memoria con limiti di corsa che danno allarme; le famiglie di controllori del mercato sono elencate in `eh3d-controller`, GRBL è la prima scritta e le altre risultano non ancora pilotate.
* I **temi e le parole** di `eh3d-view`: Forge (predefinito), Neon Arcade, Glass Pro, il grigio classico di Windows e uno che segue Windows, e l'interfaccia in inglese, spagnolo, italiano, tedesco, francese, portoghese, cinese e giapponese. Cosa sarà: un programma per forare, fresare e fare CNC 2D e 3D sui motori di EH3D-DRILLPRO ed EH3D-ROUTEPRO; quella parte non esiste ancora.
* I **menu, le impostazioni e i file**: i menu File, Visualizza, Programma, Macchina e Aiuto con ciò che ha PlanetCNC (apri, recenti, salva, modifica, gli zeri, le posizioni memorizzate, il mandrino e il refrigerante); una finestra di impostazioni (unità, lingua, undici temi, limiti di corsa, jog) che si ricorda tra le esecuzioni; e le finestre Apri e Salva della suite (`eh3d-filedialog`: luoghi, ordinamento, filtri, l'estensione di un file salvato e la domanda prima di sostituire).
* **GRBL su una porta seriale:** compilato con `--features serial`, `motioncnc --grbl PORTA[@BAUD]` comanda un controller GRBL (115200 baud di default) al posto del simulatore e `motioncnc --ports` elenca le porte seriali; una build senza la funzione, o una porta che non si apre, lo dice ed esce. Provato solo contro un controller scriptato, mai contro uno reale.
* **Una finestra che segue lo schermo:** si apre massimizzata e, da 720p a 1080p e fino a schermi più grandi e larghi, tutto cresce o si riduce con essa.

## 🧩 Ruolo in EH3D

EH3D-MOTIONCNC è il programma di controllo macchina di EH3D, oggi funzionante su una macchina simulata. Dipende per percorso da EH3D-COMMON (la cartella `EH3D-COMMON` deve stare accanto a questa: la vista, i temi, le parole e i controllori) e per ora da nessun altro repository EH3D.

## 🏛️ Architettura

<p align="center">
  <img src="images/architecture.svg" alt="Architettura di EH3D-MOTIONCNC: chi lo userà, cosa contiene e su cosa si appoggia" width="100%">
</p>

Come si incastrano le parti: chi lo userà, cosa contiene questo repository e su cosa si appoggia. L'immagine è disegnata da `docs/architecture.json`, verificata rispetto al codice e segna come futuro ciò che è futuro.

## 🖼️ Schermate

<p align="center">
  <img src="images/screenshots/program-running.png" alt="a program running" width="100%">
</p>

<p align="center"><em>Un programma in corso sulla macchina simulata: le righe eseguite in verde, la riga attuale evidenziata, il percorso disegnato sul piano e l'utensile dove si trova la macchina.</em></p>

<p align="center">
  <img src="images/screenshots/spiral-neon.png" alt="Midnight theme" width="100%">
</p>

<p align="center"><em>Il tema Mezzanotte, con un programma lungo terminato.</em></p>

<p align="center">
  <img src="images/screenshots/square-german.png" alt="German interface, light theme" width="100%">
</p>

<p align="center"><em>L'interfaccia in tedesco con il tema chiaro Carta: ogni parola della finestra segue la lingua.</em></p>

<p align="center">
  <img src="images/screenshots/settings-window.png" alt="settings window" width="100%">
</p>

<p align="center"><em>La finestra delle impostazioni, con le sezioni a sinistra come l'albero di PlanetCNC: la lingua, gli undici temi e ciò che si mostra.</em></p>

<p align="center">
  <img src="images/screenshots/open-window.png" alt="Open window" width="100%">
</p>

<p align="center"><em>La finestra Apri della suite: luoghi, recenti, l'elenco della cartella con date e dimensioni, il nome e il tipo.</em></p>

## 🔒 Sicurezza e protezione

Questo programma non apre porte seriali, connessioni di rete né file di altre persone, salvo il file G-code che sceglie l'operatore (letto come testo, mai eseguito da solo). Pilota solo una macchina simulata e non è un sistema di sicurezza. Vedi [SECURITY.md](SECURITY.md).

## 🚀 Avvio rapido

Serve una toolchain Rust stabile e la cartella `EH3D-COMMON` accanto a questa.

```bash
cargo build --release --features gui
cargo test --features gui
cargo clippy --all-targets --features gui -- -D warnings
cargo fmt --all --check
python -I tools/ci_validate.py
cargo run --release --features gui -- examples/02_frame_and_pocket.nc
```

`build.bat` / `build.sh` compilano il programma, `test.bat` / `test.sh` eseguono i test e `run.bat` / `run.sh` aprono la finestra su un programma di esempio. Gli script aspettano un tasto alla fine salvo che `NOPAUSE` sia impostato.

## 🧪 Test

`cargo test --features gui` esegue 25 test: la sessione (jog e zero, un programma riga per riga, pausa e ripresa, una riga rifiutata, l'arresto di emergenza e l'allarme, il registro dei comandi, l'origine), il percorso di un programma e la riga di comando; la vista e il controllore simulato sono provati in EH3D-COMMON. La CI esegue anche `cargo fmt --check` e `cargo clippy` con gli avvisi come errori, e `python -I tools/ci_validate.py` controlla manifesto, versioni, changelog e gli otto README.

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
│   ├── lib.rs                                   i moduli e la vista di eh3d-view riesportata con il nome di questo programma
│   ├── gui.rs                                   la finestra: posizione, jog, programma, percorso, comandi e registro
│   ├── session.rs                               ciò che si fa con un controllore: jog, zero, origine, un programma riga per riga, pausa, stop, allarmi
│   ├── gcode.rs                                 i movimenti rettilinei di un programma G-code, per disegnarne il percorso
│   ├── cli.rs                                   la riga di comando: il file, la lingua, il tema e le opzioni per l'immagine
│   ├── bin/motioncnc.rs                         il programma che apre la finestra
│   ├── settings.rs                              ciò che si ricorda tra esecuzioni
│   └── gui/dialogs.rs                           menu, impostazioni, Apri, Salva, editor
├── examples/                                    tre programmi G-code di esempio, da un quadrato a una lunga spirale
├── tools/                                       controllo di CI, del README e strumenti per le immagini
├── docs/                                        le sorgenti delle immagini di Architettura e Struttura del repository
├── .github/                                     workflow di CI, modelli di issue e pull request
├── images/                                      banner, le due immagini e le schermate
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

* Nulla di qui è stato eseguito su una macchina reale: il controllore è un simulatore e non si apre alcuna porta seriale né connessione di rete.
* Il simulatore accetta solo G-code di base (`G0`, `G1`, `G90`, `G91`, `G20`, `G21`, `G92`, da `M3` a `M9`); archi, cicli fissi e tastatura sono rifiutati, e il percorso non disegna gli archi.
* Richiede la cartella `EH3D-COMMON` accanto, perché ne dipende per percorso.

**Prossimi passi**

* Un controllore reale: prima GRBL su porta seriale, poi le altre famiglie, poi il controllore di HYDRA-UMC.
* Gli archi e il resto del G-code nel simulatore e nel percorso.
* Una vista 3D del percorso con l'altezza dei tagli.
* Tastatura, cambio utensile e i materiali della macchina.
* Unire i motori di EH3D-DRILLPRO ed EH3D-ROUTEPRO perché un programma fori e fresi.

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
