<p align="center">
  <img src="images/EH3D_BANNER.svg" alt="EH3D-MOTIONCNC bannière" width="100%">
</p>

# 🧭 EH3D-MOTIONCNC

<p align="center"><a href="README.md">🇺🇸 English</a> | <a href="README_spa.md">🇪🇸 Español</a> | 🇫🇷 <b>Français</b> | <a href="README_ita.md">🇮🇹 Italiano</a> | <a href="README_deu.md">🇩🇪 Deutsch</a> | <a href="README_zho.md">🇨🇳 中文</a> | <a href="README_jpn.md">🇯🇵 日本語</a> | <a href="README_por.md">🇵🇹 Português</a></p>

### La base commune des programmes de commande de machine EH3D : la vue d'un travail, les thèmes et les textes de l'interface en huit langues

<p align="center">
  <img src="https://img.shields.io/badge/License-GPL%203.0--or--later-blue.svg" alt="GPL 3.0 or later">
  <img src="https://img.shields.io/badge/Stack-Rust-DEA584.svg" alt="Rust">
  <img src="https://img.shields.io/badge/Languages-8-00E5FF.svg" alt="Languages">
  <img src="https://img.shields.io/badge/Version-0.0.1-0f6cbd.svg" alt="Version">
  <img src="https://img.shields.io/badge/Maturity-scaffolding-FFB020.svg" alt="Maturity">
  <img src="https://img.shields.io/badge/Tests-0-lightgrey.svg" alt="Tests">
</p>

---

**Contrôle d'honnêteté - ce qui fonctionne réellement aujourd'hui :** un crate Rust qui utilise la vue d'un travail d'`eh3d-view` d'EH3D-COMMON, où elle est partagée avec les autres programmes de la suite et possède ses tests. Cette vue contient la scène d'un travail sur la table de travail (chaque trou avec la taille de son foret, les trajets de fraisage, l'avancement du travail, un travail de perçage et un de fraisage superposés), la vue de dessus et la vue isométrique de la machine adaptées à la taille de la table, un contrôle qui dit ce qui ne tient pas sur la table avant de lancer un travail, cinq thèmes (Forge, Neon Arcade, Glass Pro, le gris classique de Windows et un qui suit les couleurs de Windows) et les textes de l'interface en huit langues. **Il n'y a pas encore de fenêtre ni de commande de machine**, rien de tout cela n'a tourné sur une machine et ce dépôt n'a pas de tests propres : il ne fait que réexporter la vue.

---

> **État de la sûreté et de la sécurité :** ce dépôt n'est pas un système de sécurité et ne pilote aucune machine. Il n'ouvre aucun port série ni connexion réseau et ne lit pas de fichiers d'autres personnes. Quand la commande de machine sera ajoutée, elle devra faire ses preuves sur un simulateur puis sur une machine réelle avant d'être déclarée fonctionnelle, et les fins de course et un arrêt d'urgence restent l'affaire de l'unité de commande de la machine elle-même.

## 📋 En un coup d'œil

| | |
|---|---|
| Version | `0.0.1` |
| Maturité | Échafaudage |
| Tests | 0 |
| Pile technique | Rust |
| Sur du matériel réel | Non : rien ici ne se connecte à une machine |

**Aller à :** [Présentation](#-présentation) · [Architecture](#-architecture) · [Démarrage rapide](#-démarrage-rapide) · [Tests](#-tests) · [Structure](#-structure-du-dépôt) · [Liés](#-projets-liés)

## 🚦 État du projet

Chaque ligne est une partie de ce dépôt et le feu dit où elle en est aujourd'hui. Il est écrit d'après ce que le dépôt peut montrer (tests, compilations, exécutions), pas d'après ce qui est prévu.

🟢 Fait et vérifié · 🟡 Fonctionne, avec des limites connues · 🟠 À moitié fait ou pas encore vérifié · 🔴 Pas fait ou ne fonctionne pas

| Domaine | État | Ce que cela signifie ici |
|---|---|---|
| Bibliothèque de base | 🟡 | Un crate Rust qui réexporte `scene`, `texts` et `theme` d'`eh3d-view` sous le nom de ce programme ; il a besoin du dossier d'EH3D-COMMON à côté. |
| Scène et vues | 🟡 | Écrites et testées sur un ordinateur dans EH3D-COMMON : le travail sur la table, les vues de dessus et isométrique, le contrôle de ce qui ne tient pas ; aucune fenêtre ne les dessine encore. |
| Thèmes et textes | 🟡 | Cinq thèmes avec contrôle du contraste et les textes de l'interface en huit langues, testés dans EH3D-COMMON ; pas encore vus dans une fenêtre. |
| Tests | 🟠 | Ce dépôt a 0 tests propres (`cargo test`) : la vue est testée là où elle vit, dans EH3D-COMMON. |
| Fenêtre et jog | 🔴 | Pas commencé. |
| Commande de machine | 🔴 | Pas commencé : aucune connexion à une machine, pas de G-code, aucun travail de perçage ou de fraisage ne part d'ici. |
| Documentation | 🟡 | README en huit langues, les deux images et le journal des modifications ; il y a peu à documenter tant que le programme n'existe pas. |

## 🎯 Présentation

* La **bibliothèque de base** (`src/lib.rs`) : réexporte `scene`, `texts` et `theme` d'`eh3d-view` d'EH3D-COMMON, pour que le futur programme les utilise sous son propre nom et que tous les programmes de la suite montrent un travail de la même façon.
* La **scène** d'un travail sur la table : trous par taille de foret, trajets de fraisage, l'avancement du travail, un travail de perçage et un de fraisage superposés, la vue de dessus, la vue isométrique de la machine adaptée à la taille de la table et un contrôle de ce qui ne tient pas sur la table.
* Les **thèmes** : Forge (par défaut), Neon Arcade, Glass Pro, le gris classique de Windows et un qui suit les couleurs de Windows ; chacun peut écrire le sien dans un fichier texte.
* Les **textes de l'interface** en anglais, espagnol, italien, allemand, français, portugais, chinois et japonais.
* Ce que ce sera : un programme pour percer, fraiser et faire de la CNC 2D et 3D, construit sur cette base, sur le ZIP des travaux d'EH3D-COMMON (`eh3d-archive`) et sur les moteurs d'EH3D-DRILLPRO et d'EH3D-ROUTEPRO. **Rien de cela n'existe encore.**

## 🧩 Rôle dans EH3D

EH3D-MOTIONCNC sera le futur programme de commande de machine d'EH3D et n'en est aujourd'hui que la base. Il dépend par chemin d'EH3D-COMMON (le dossier `EH3D-COMMON` doit se trouver à côté de celui-ci) et d'aucun autre dépôt EH3D.

## 🏛️ Architecture

<p align="center">
  <img src="images/architecture.svg" alt="Architecture d'EH3D-MOTIONCNC : qui l'utilisera, ce qu'il contient et sur quoi il s'appuie" width="100%">
</p>

Comment les parties s'assemblent : qui l'utilisera, ce que contient ce dépôt et sur quoi il s'appuie. L'image est dessinée à partir de `docs/architecture.json`, vérifiée par rapport au code, et marque comme futur ce qui est futur.

## 🔒 Sécurité et protection

Ce dépôt n'ouvre aucun port série, aucune connexion réseau et aucun fichier d'autres personnes ; le seul texte extérieur que ses parties lisent est un fichier de thème écrit par l'utilisateur, qui est vérifié plutôt que cru. Il ne pilote pas de machine et n'est pas un système de sécurité. Voir [SECURITY.md](SECURITY.md).

## 🚀 Démarrage rapide

Il faut une chaîne d'outils Rust stable et le dossier `EH3D-COMMON` à côté de celui-ci.

```bash
cargo build --release
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --all --check
python -I tools/ci_validate.py
```

`build.bat` / `build.sh` compilent la bibliothèque, `test.bat` / `test.sh` lancent les tests et `run.bat` / `run.sh` indiquent qu'il n'y a pas encore de programme et lancent les tests. Les scripts attendent une touche à la fin sauf si `NOPAUSE` est défini.

## 🧪 Tests

`cargo test` signale 0 tests ici : ce dépôt ne fait que réexporter la vue, dont les tests sont dans EH3D-COMMON. La CI lance aussi `cargo fmt --check` et `cargo clippy` avec les avertissements comme erreurs, et `python -I tools/ci_validate.py` vérifie le manifeste, les versions, le journal des modifications et les huit README.

## 🏗️ Compilation et publication

La version se trouve dans `Cargo.toml` et dans `eh3d.project.json`, et chaque version a son entrée dans [CHANGELOG.md](CHANGELOG.md). Il n'y a pas encore de programme à empaqueter, donc pas de binaires prêts à l'emploi.

## 📂 Structure du dépôt

<p align="center">
  <img src="images/repository-structure.svg" alt="Structure du dépôt EH3D-MOTIONCNC" width="100%">
</p>

<details>
<summary>Version texte de l'arborescence</summary>

```text
EH3D-MOTIONCNC/
├── src/
│   └── lib.rs                                   réexporte scène, textes et thème d'eh3d-view d'EH3D-COMMON
├── tools/                                       contrôle de CI, du README et outils d'images
├── docs/                                        les sources des images d'Architecture et de Structure du dépôt
├── .github/                                     workflow de CI, modèles d'issues et de pull requests
├── images/                                      bannière et images d'Architecture et de Structure du dépôt
├── build.sh build.bat                           compilent la bibliothèque
├── run.sh run.bat                               indiquent qu'il n'y a pas encore de programme et lancent les tests
├── eh3d.project.json                            manifeste : version, rôle, maturité et commande de compilation
├── Cargo.toml Cargo.lock                        le paquet Rust et ses dépendances figées
├── CHANGELOG.md                                 ce qui a changé dans chaque version
├── SECURITY.md SUPPORT.md                       signaler une vulnérabilité et poser des questions
└── CONTRIBUTING.md CODE_OF_CONDUCT.md LICENSE   contribuer, code de conduite et licence
```

</details>

## 🗺️ Feuille de route et limites connues

**Limites connues aujourd'hui**

* Il n'y a ni fenêtre ni commande de machine : rien ici n'a tourné sur une machine.
* Il n'y a pas de tests dans ce dépôt ; la vue est testée dans EH3D-COMMON.
* Il a besoin du dossier `EH3D-COMMON` à côté, car il en dépend par chemin.

**Ensuite**

* Étudier les programmes de commande de machine qui existent aujourd'hui (un natif et un .NET) pour décider de ce dont l'opérateur a besoin.
* Une fenêtre avec la scène, les thèmes et les textes de l'interface.
* Jog et G-code : connexion à une machine, d'abord sur un simulateur.
* Vues 2D et 3D d'un travail en cours.
* Réunir les moteurs d'EH3D-DRILLPRO et d'EH3D-ROUTEPRO pour qu'un seul programme perce et fraise.

## 🌍 Langues

Le README et les textes existent en anglais, espagnol, français, italien, allemand, chinois, japonais et portugais européen.

## 🔗 Projets liés

* [EH3D-COMMON](https://github.com/JuanenRac/EH3D-COMMON) : là où vivent la vue (`eh3d-view`) et le ZIP des travaux (`eh3d-archive`)
* [EH3D-DRILLPRO](https://github.com/JuanenRac/EH3D-DRILLPRO) : le programme de perçage dont le moteur sera réuni ici
* [EH3D-ROUTEPRO](https://github.com/JuanenRac/EH3D-ROUTEPRO) : le programme de fraisage dont le moteur sera réuni ici

## 📚 Documentation

* [Journal des modifications](CHANGELOG.md)
* [Politique de sécurité](SECURITY.md) et [assistance](SUPPORT.md)
* La vue d'un travail est documentée dans EH3D-COMMON, là où elle vit.

## 🤝 Contribuer et communauté

* Avant une pull request, lancez `cargo fmt --all --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test` et `python -I tools/ci_validate.py` : tout doit passer. Voir [CONTRIBUTING.md](CONTRIBUTING.md) et le [code de conduite](CODE_OF_CONDUCT.md).
* Le README existe en huit langues : une modification du texte se fait dans les huit fichiers, et les deux images sont dessinées à partir de `docs/architecture.json` et `docs/structure.json` par `tools/make_architecture.py` et `tools/make_structure.py`.
* Ne décrivez pas comme fonctionnel ce qui n'a pas tourné sur une machine.
* Signalez une vulnérabilité en privé : [SECURITY.md](SECURITY.md). Questions, idées et rapports : electrohobby3d@gmail.com ([SUPPORT.md](SUPPORT.md)).

## 👤 AUTEUR

**JuanenRac (Electro Hobby 3D)** · electrohobby3d@gmail.com

## 📜 LICENCE

Logiciel : GPL-3.0-or-later. Voir [LICENSE](LICENSE). Les conceptions matérielles de la suite utilisent CERN OHL-S.
