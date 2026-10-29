<p align="center">
  <img src="images/EH3D_BANNER.svg" alt="EH3D-MOTIONCNC 横幅" width="100%">
</p>

# 🧭 EH3D-MOTIONCNC

<p align="center"><a href="README.md">🇺🇸 English</a> | <a href="README_spa.md">🇪🇸 Español</a> | <a href="README_fra.md">🇫🇷 Français</a> | <a href="README_ita.md">🇮🇹 Italiano</a> | <a href="README_deu.md">🇩🇪 Deutsch</a> | 🇨🇳 <b>中文</b> | <a href="README_jpn.md">🇯🇵 日本語</a> | <a href="README_por.md">🇵🇹 Português</a></p>

### EH3D 机床控制程序的共享基础：作业视图、主题以及八种语言的界面文本

<p align="center">
  <img src="https://img.shields.io/badge/License-GPL%203.0--or--later-blue.svg" alt="GPL 3.0 or later">
  <img src="https://img.shields.io/badge/Stack-Rust-DEA584.svg" alt="Rust">
  <img src="https://img.shields.io/badge/Languages-8-00E5FF.svg" alt="Languages">
  <img src="https://img.shields.io/badge/Version-0.0.1-0f6cbd.svg" alt="Version">
  <img src="https://img.shields.io/badge/Maturity-scaffolding-FFB020.svg" alt="Maturity">
  <img src="https://img.shields.io/badge/Tests-0-lightgrey.svg" alt="Tests">
</p>

---

**诚实性检查 - 今天实际能运行的内容：** 一个 Rust crate，使用 EH3D-COMMON 中 `eh3d-view` 的作业视图；该视图与套件中的其他程序共享，测试也在那里。视图包含工作台上作业的场景（每个孔及其钻头尺寸、铣削路径、作业进度、叠放的钻孔作业和铣削作业）、按台面尺寸适配的俯视图和机床等轴测图、作业开始前说明哪些内容放不进台面的检查、五个主题（Forge、Neon Arcade、Glass Pro、经典 Windows 灰色，以及一个跟随 Windows 颜色的主题）和八种语言的界面文本。**目前还没有窗口，也没有机床控制**，这里的内容都没有在机床上运行过，本仓库也没有自己的测试：它只是重新导出该视图。

---

> **安全（safety）与信息安全现状：** 本仓库不是安全系统，也不驱动任何机床。它不打开串口或网络连接，也不读取他人的文件。加入机床控制后，必须先在模拟器上、再在真实机床上证明自己，才能称为可用；限位开关和急停仍由机床自身的控制单元负责。

## 📋 速览

| | |
|---|---|
| 版本 | `0.0.1` |
| 成熟度 | 脚手架 |
| 测试 | 0 |
| 技术栈 | Rust |
| 真实硬件 | 否：这里的内容不连接任何机床 |

**跳转到：** [概述](#-概述) · [架构](#-架构) · [快速开始](#-快速开始) · [测试](#-测试) · [结构](#-仓库结构) · [相关项目](#-相关项目)

## 🚦 项目状态

每一行是本仓库的一个部分，灯号表示它今天做到了哪一步。内容依据仓库能够展示的证据（测试、构建、运行），而不是计划。

🟢 已完成并验证 · 🟡 可用，有已知限制 · 🟠 部分完成或尚未验证 · 🔴 未完成或不可用

| 领域 | 状态 | 在这里的含义 |
|---|---|---|
| 基础库 | 🟡 | 一个 Rust crate，以本程序的名称重新导出 `eh3d-view` 的 `scene`、`texts` 和 `theme`；需要旁边有 EH3D-COMMON 文件夹。 |
| 场景与视图 | 🟡 | 已在 EH3D-COMMON 中编写并在计算机上测试：台面上的作业、俯视图与等轴测图、放不下内容的检查；目前还没有窗口绘制它们。 |
| 主题与文本 | 🟡 | 五个带对比度检查的主题和八种语言的界面文本，已在 EH3D-COMMON 中测试；尚未在窗口中看过。 |
| 测试 | 🟠 | 本仓库自己有 0 个测试（`cargo test`）：视图在它所在的 EH3D-COMMON 中测试。 |
| 窗口与点动 | 🔴 | 尚未开始。 |
| 机床控制 | 🔴 | 尚未开始：不连接任何机床，没有 G 代码，这里不会运行任何钻孔或铣削作业。 |
| 文档 | 🟡 | 八种语言的 README、两幅图和变更日志；程序出现之前可写的文档不多。 |

## 🎯 概述

* **基础库**（`src/lib.rs`）：重新导出 EH3D-COMMON 中 `eh3d-view` 的 `scene`、`texts` 和 `theme`，让未来的程序以自己的名称使用它们，并让套件中的每个程序以同样的方式显示作业。
* 工作台上作业的**场景**：按钻头尺寸分类的孔、铣削路径、作业进度、叠放的钻孔与铣削作业、俯视图、按台面尺寸适配的机床等轴测图，以及放不进台面的内容的检查。
* **主题**：Forge（默认）、Neon Arcade、Glass Pro、经典 Windows 灰色和一个跟随 Windows 颜色的主题；用户可以在文本文件中编写自己的主题。
* **界面文本**：英语、西班牙语、意大利语、德语、法语、葡萄牙语、中文和日语。
* 它将成为什么：一个用于钻孔、铣削以及 2D 和 3D CNC 加工的程序，建立在这个基础、EH3D-COMMON 的作业 ZIP（`eh3d-archive`）以及 EH3D-DRILLPRO 和 EH3D-ROUTEPRO 的引擎之上。**这些目前都还不存在。**

## 🧩 在 EH3D 中的角色

EH3D-MOTIONCNC 将是 EH3D 未来的机床控制程序，今天只是它的基础。它通过路径依赖 EH3D-COMMON（`EH3D-COMMON` 文件夹必须放在本文件夹旁边），目前不依赖其他 EH3D 仓库。

## 🏛️ 架构

<p align="center">
  <img src="images/architecture.svg" alt="EH3D-MOTIONCNC 架构：谁将使用它、包含什么、依赖什么" width="100%">
</p>

各部分如何配合：谁将使用它、本仓库包含什么、它依赖什么。图片由 `docs/architecture.json` 绘制，并与代码核对，未来的内容标为未来。

## 🔒 安全与防护

本仓库不打开串口、网络连接，也不读取他人的文件；它各部分读取的唯一外部文本是用户编写的主题文件，该文件会被检查而不是被直接信任。它不驱动机床，也不是安全系统。参见 [SECURITY.md](SECURITY.md)。

## 🚀 快速开始

需要稳定版 Rust 工具链，以及本文件夹旁边的 `EH3D-COMMON` 文件夹。

```bash
cargo build --release
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --all --check
python -I tools/ci_validate.py
```

`build.bat` / `build.sh` 构建库，`test.bat` / `test.sh` 运行测试，`run.bat` / `run.sh` 提示尚无程序并运行测试。除非设置了 `NOPAUSE`，脚本结束时会等待按键。

## 🧪 测试

`cargo test` 在这里报告 0 个测试：本仓库只重新导出视图，视图的测试在 EH3D-COMMON 中。CI 还会运行 `cargo fmt --check` 和把警告当作错误的 `cargo clippy`，`python -I tools/ci_validate.py` 检查清单、版本、变更日志和八个 README。

## 🏗️ 构建与发布

版本号位于 `Cargo.toml` 和 `eh3d.project.json`，每个版本在 [CHANGELOG.md](CHANGELOG.md) 中有对应条目。目前还没有可打包的程序，所以没有现成的二进制文件。

## 📂 仓库结构

<p align="center">
  <img src="images/repository-structure.svg" alt="EH3D-MOTIONCNC 仓库结构" width="100%">
</p>

<details>
<summary>目录树的文本版</summary>

```text
EH3D-MOTIONCNC/
├── src/
│   └── lib.rs                                   重新导出 EH3D-COMMON 中 eh3d-view 的场景、文本和主题
├── tools/                                       CI 检查、README 检查和图片工具
├── docs/                                        架构图和仓库结构图的源文件
├── .github/                                     CI 工作流、Issue 和 PR 模板
├── images/                                      横幅图以及架构和仓库结构图片
├── build.sh build.bat                           构建库
├── run.sh run.bat                               提示尚无程序并运行测试
├── eh3d.project.json                            清单：版本、角色、成熟度和构建命令
├── Cargo.toml Cargo.lock                        Rust 包及其锁定的依赖
├── CHANGELOG.md                                 每个版本的变更
├── SECURITY.md SUPPORT.md                       如何报告漏洞及在哪里提问
└── CONTRIBUTING.md CODE_OF_CONDUCT.md LICENSE   参与贡献、行为准则和许可证
```

</details>

## 🗺️ 路线图与已知限制

**目前已知的限制**

* 没有窗口，也没有机床控制：这里的内容都没有在机床上运行过。
* 本仓库没有测试；视图在 EH3D-COMMON 中测试。
* 需要旁边有 `EH3D-COMMON` 文件夹，因为它通过路径依赖该文件夹。

**下一步**

* 研究现有的机床控制程序（一个原生程序和一个 .NET 程序），确定操作人员需要什么。
* 一个显示场景、主题和界面文本的窗口。
* 点动和 G 代码：连接机床，先在模拟器上进行。
* 运行中作业的 2D 和 3D 视图。
* 合并 EH3D-DRILLPRO 和 EH3D-ROUTEPRO 的引擎，让一个程序同时钻孔和铣削。

## 🌍 语言

README 和文本提供英语、西班牙语、法语、意大利语、德语、中文、日语和欧洲葡萄牙语版本。

## 🔗 相关项目

* [EH3D-COMMON](https://github.com/JuanenRac/EH3D-COMMON)：视图（`eh3d-view`）和作业 ZIP（`eh3d-archive`）所在的仓库
* [EH3D-DRILLPRO](https://github.com/JuanenRac/EH3D-DRILLPRO)：其引擎将合并到这里的钻孔程序
* [EH3D-ROUTEPRO](https://github.com/JuanenRac/EH3D-ROUTEPRO)：其引擎将合并到这里的铣削程序

## 📚 文档

* [变更日志](CHANGELOG.md)
* [安全政策](SECURITY.md)和[支持](SUPPORT.md)
* 作业视图的文档在它所在的 EH3D-COMMON 中。

## 🤝 参与贡献与社区

* 提交 pull request 之前请运行 `cargo fmt --all --check`、`cargo clippy --all-targets -- -D warnings`、`cargo test` 和 `python -I tools/ci_validate.py`：全部必须通过。参见 [CONTRIBUTING.md](CONTRIBUTING.md) 和[行为准则](CODE_OF_CONDUCT.md)。
* README 有八种语言：修改文字时八个文件都要改，两幅图由 `tools/make_architecture.py` 和 `tools/make_structure.py` 根据 `docs/architecture.json` 和 `docs/structure.json` 绘制。
* 不要把没有在机床上运行过的内容描述为可用。
* 请私下报告漏洞：[SECURITY.md](SECURITY.md)。问题、想法和报告：electrohobby3d@gmail.com（[SUPPORT.md](SUPPORT.md)）。

## 👤 作者

**JuanenRac (Electro Hobby 3D)** · electrohobby3d@gmail.com

## 📜 许可证

软件：GPL-3.0-or-later。参见 [LICENSE](LICENSE)。套件的硬件设计使用 CERN OHL-S。
