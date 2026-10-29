<p align="center">
  <img src="images/EH3D_BANNER.svg" alt="EH3D-MOTIONCNC banner" width="100%">
</p>

# 🧭 EH3D-MOTIONCNC

<p align="center"><a href="README.md">🇺🇸 English</a> | <a href="README_spa.md">🇪🇸 Español</a> | <a href="README_fra.md">🇫🇷 Français</a> | <a href="README_ita.md">🇮🇹 Italiano</a> | <a href="README_deu.md">🇩🇪 Deutsch</a> | <a href="README_zho.md">🇨🇳 中文</a> | <a href="README_jpn.md">🇯🇵 日本語</a> | 🇵🇹 <b>Português</b></p>

### A base partilhada dos programas de controlo de máquina do EH3D: a vista de um trabalho, os temas e os textos da interface em oito idiomas

<p align="center">
  <img src="https://img.shields.io/badge/License-GPL%203.0--or--later-blue.svg" alt="GPL 3.0 or later">
  <img src="https://img.shields.io/badge/Stack-Rust-DEA584.svg" alt="Rust">
  <img src="https://img.shields.io/badge/Languages-8-00E5FF.svg" alt="Languages">
  <img src="https://img.shields.io/badge/Version-0.0.1-0f6cbd.svg" alt="Version">
  <img src="https://img.shields.io/badge/Maturity-scaffolding-FFB020.svg" alt="Maturity">
  <img src="https://img.shields.io/badge/Tests-0-lightgrey.svg" alt="Tests">
</p>

---

**Verificação de honestidade - o que realmente corre hoje:** um crate de Rust que usa a vista de um trabalho do `eh3d-view` do EH3D-COMMON, onde é partilhada com os outros programas da suite e tem os seus testes. Essa vista tem a cena de um trabalho na mesa de trabalho (cada furo com o tamanho da sua broca, os trajetos de fresagem, quanto do trabalho já está feito, um trabalho de furação e um de fresagem sobrepostos), a vista de cima e a vista isométrica da máquina ajustadas ao tamanho da mesa, uma verificação que diz o que não cabe na mesa antes de começar um trabalho, cinco temas (Forge, Neon Arcade, Glass Pro, o cinzento clássico do Windows e um que segue as cores do Windows) e os textos da interface em oito idiomas. **Ainda não há ecrã nem controlo de máquina**, nada disto correu numa máquina e este repositório não tem testes próprios: só reexporta a vista.

---

> **Estado de segurança (safety) e de segurança informática:** este repositório não é um sistema de segurança e não comanda nenhuma máquina. Não abre nenhuma porta série nem ligação de rede e não lê ficheiros de outras pessoas. Quando o controlo de máquina for acrescentado, terá de provar-se primeiro num simulador e depois numa máquina real antes de se dizer que funciona, e os fins de curso e uma paragem de emergência continuam a ser tarefa da própria unidade de controlo da máquina.

## 📋 Num relance

| | |
|---|---|
| Versão | `0.0.1` |
| Maturidade | Estrutura inicial |
| Testes | 0 |
| Tecnologias | Rust |
| Em hardware real | Não: nada aqui se liga a uma máquina |

**Ir para:** [Visão geral](#-visão-geral) · [Arquitetura](#-arquitetura) · [Início rápido](#-início-rápido) · [Testes](#-testes) · [Estrutura](#-estrutura-do-repositório) · [Relacionados](#-projetos-relacionados)

## 🚦 Estado do projeto

Cada linha é uma parte deste repositório e a luz diz até onde chega hoje. Escreve-se com o que o repositório consegue mostrar (testes, compilações, execuções), não com o que está planeado.

🟢 Feito e verificado · 🟡 Funciona, com limites conhecidos · 🟠 Feito a meio ou ainda por verificar · 🔴 Por fazer ou a não funcionar

| Área | Estado | O que significa aqui |
|---|---|---|
| Biblioteca base | 🟡 | Um crate de Rust que reexporta `scene`, `texts` e `theme` do `eh3d-view` com o nome deste programa; precisa da pasta do EH3D-COMMON ao lado. |
| Cena e vistas | 🟡 | Escritas e testadas num computador no EH3D-COMMON: o trabalho na mesa, as vistas de cima e isométrica, a verificação do que não cabe; ainda nenhum ecrã as desenha. |
| Temas e textos | 🟡 | Cinco temas com verificação de contraste e os textos da interface em oito idiomas, testados no EH3D-COMMON; ainda não vistos numa janela. |
| Testes | 🟠 | Este repositório tem 0 testes próprios (`cargo test`): a vista é testada onde vive, no EH3D-COMMON. |
| Janela e jog | 🔴 | Por começar. |
| Controlo de máquina | 🔴 | Por começar: nenhuma ligação a uma máquina, nada de G-code, nenhum trabalho de furação ou fresagem corre a partir daqui. |
| Documentação | 🟡 | README em oito idiomas, as duas imagens e o registo de alterações; há pouco para documentar até o programa existir. |

## 🎯 Visão geral

* A **biblioteca base** (`src/lib.rs`): reexporta `scene`, `texts` e `theme` do `eh3d-view` do EH3D-COMMON, para que o futuro programa os use com o seu próprio nome e todos os programas da suite mostrem um trabalho da mesma maneira.
* A **cena** de um trabalho na mesa: furos por tamanho de broca, trajetos de fresagem, o avanço do trabalho, um trabalho de furação e outro de fresagem sobrepostos, a vista de cima, a vista isométrica da máquina ajustada ao tamanho da mesa e uma verificação do que não cabe na mesa.
* Os **temas**: Forge (o predefinido), Neon Arcade, Glass Pro, o cinzento clássico do Windows e um que segue as cores do Windows; cada utilizador pode escrever o seu num ficheiro de texto.
* Os **textos da interface** em inglês, espanhol, italiano, alemão, francês, português, chinês e japonês.
* O que será: um programa para furar, fresar e fazer CNC 2D e 3D, construído sobre esta base, sobre o ZIP de trabalhos do EH3D-COMMON (`eh3d-archive`) e sobre os motores do EH3D-DRILLPRO e do EH3D-ROUTEPRO. **Nada disso existe ainda.**

## 🧩 Papel no EH3D

O EH3D-MOTIONCNC será o futuro programa de controlo de máquina do EH3D e, hoje, é apenas a sua base. Depende por caminho do EH3D-COMMON (a pasta `EH3D-COMMON` tem de estar ao lado desta) e de nenhum outro repositório do EH3D.

## 🏛️ Arquitetura

<p align="center">
  <img src="images/architecture.svg" alt="Arquitetura do EH3D-MOTIONCNC: quem o vai usar, o que contém e em que se apoia" width="100%">
</p>

Como as partes se encaixam: quem o vai usar, o que este repositório contém e em que se apoia. A imagem é desenhada a partir de `docs/architecture.json`, verificada contra o código, e marca como futuro o que é futuro.

## 🔒 Segurança e proteção

Este repositório não abre nenhuma porta série, nenhuma ligação de rede nem ficheiros de outras pessoas; o único texto externo que as suas partes leem é um ficheiro de tema escrito pelo utilizador, que é verificado em vez de aceite sem mais. Não comanda uma máquina e não é um sistema de segurança. Ver [SECURITY.md](SECURITY.md).

## 🚀 Início rápido

Precisa de uma cadeia de ferramentas Rust estável e da pasta `EH3D-COMMON` ao lado desta.

```bash
cargo build --release
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --all --check
python -I tools/ci_validate.py
```

`build.bat` / `build.sh` compilam a biblioteca, `test.bat` / `test.sh` executam os testes e `run.bat` / `run.sh` avisam de que ainda não há programa e executam os testes. Os scripts esperam por uma tecla no fim, a não ser que `NOPAUSE` esteja definido.

## 🧪 Testes

`cargo test` indica 0 testes aqui: este repositório só reexporta a vista, cujos testes estão no EH3D-COMMON. A CI executa também `cargo fmt --check` e `cargo clippy` com os avisos como erros, e `python -I tools/ci_validate.py` verifica o manifesto, as versões, o registo de alterações e os oito README.

## 🏗️ Compilação e publicação

A versão está em `Cargo.toml` e em `eh3d.project.json`, e cada versão tem a sua entrada em [CHANGELOG.md](CHANGELOG.md). Ainda não há programa para empacotar, por isso não há binários prontos.

## 📂 Estrutura do repositório

<p align="center">
  <img src="images/repository-structure.svg" alt="Estrutura do repositório do EH3D-MOTIONCNC" width="100%">
</p>

<details>
<summary>Versão em texto da árvore</summary>

```text
EH3D-MOTIONCNC/
├── src/
│   └── lib.rs                                   reexporta cena, textos e tema do eh3d-view do EH3D-COMMON
├── tools/                                       verificação de CI, do README e ferramentas de imagens
├── docs/                                        as fontes das imagens de Arquitetura e Estrutura do repositório
├── .github/                                     fluxo de CI, modelos de issues e pull requests
├── images/                                      banner e as imagens de Arquitetura e Estrutura do repositório
├── build.sh build.bat                           compilam a biblioteca
├── run.sh run.bat                               avisam de que ainda não há programa e executam os testes
├── eh3d.project.json                            manifesto: versão, papel, maturidade e comando de compilação
├── Cargo.toml Cargo.lock                        o pacote Rust e as suas dependências fixadas
├── CHANGELOG.md                                 o que mudou em cada versão
├── SECURITY.md SUPPORT.md                       como comunicar uma vulnerabilidade e onde perguntar
└── CONTRIBUTING.md CODE_OF_CONDUCT.md LICENSE   como contribuir, o código de conduta e a licença
```

</details>

## 🗺️ Roteiro e limites conhecidos

**Limites conhecidos hoje**

* Não há ecrã nem controlo de máquina: nada aqui correu numa máquina.
* Não há testes neste repositório; a vista é testada no EH3D-COMMON.
* Precisa da pasta `EH3D-COMMON` ao lado, porque depende dela por caminho.

**A seguir**

* Estudar os programas de controlo de máquina que existem hoje (um nativo e um .NET) para decidir do que o operador precisa.
* Uma janela com a cena, os temas e os textos da interface.
* Jog e G-code: ligação a uma máquina, primeiro num simulador.
* Vistas 2D e 3D de um trabalho em curso.
* Juntar os motores do EH3D-DRILLPRO e do EH3D-ROUTEPRO para que um só programa fure e frese.

## 🌍 Idiomas

O README e os textos existem em inglês, espanhol, francês, italiano, alemão, chinês, japonês e português europeu.

## 🔗 Projetos relacionados

* [EH3D-COMMON](https://github.com/JuanenRac/EH3D-COMMON): onde vivem a vista (`eh3d-view`) e o ZIP de trabalhos (`eh3d-archive`)
* [EH3D-DRILLPRO](https://github.com/JuanenRac/EH3D-DRILLPRO): o programa de furação cujo motor será juntado aqui
* [EH3D-ROUTEPRO](https://github.com/JuanenRac/EH3D-ROUTEPRO): o programa de fresagem cujo motor será juntado aqui

## 📚 Documentação

* [Registo de alterações](CHANGELOG.md)
* [Política de segurança](SECURITY.md) e [suporte](SUPPORT.md)
* A vista de um trabalho está documentada no EH3D-COMMON, onde vive.

## 🤝 Contribuir e comunidade

* Antes de um pull request execute `cargo fmt --all --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test` e `python -I tools/ci_validate.py`: tudo tem de passar. Ver [CONTRIBUTING.md](CONTRIBUTING.md) e o [código de conduta](CODE_OF_CONDUCT.md).
* O README existe em oito idiomas: uma alteração ao texto faz-se nos oito ficheiros, e as duas imagens são desenhadas a partir de `docs/architecture.json` e `docs/structure.json` com `tools/make_architecture.py` e `tools/make_structure.py`.
* Não descreva como a funcionar o que não correu numa máquina.
* Comunique uma vulnerabilidade em privado: [SECURITY.md](SECURITY.md). Perguntas, ideias e relatos: electrohobby3d@gmail.com ([SUPPORT.md](SUPPORT.md)).

## 👤 AUTOR

**JuanenRac (Electro Hobby 3D)** · electrohobby3d@gmail.com

## 📜 LICENÇA

Software: GPL-3.0-or-later. Ver [LICENSE](LICENSE). Os desenhos de hardware da suite usam CERN OHL-S.
