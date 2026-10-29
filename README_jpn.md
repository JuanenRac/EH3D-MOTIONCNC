<p align="center">
  <img src="images/EH3D_BANNER.svg" alt="EH3D-MOTIONCNC バナー" width="100%">
</p>

# 🧭 EH3D-MOTIONCNC

<p align="center"><a href="README.md">🇺🇸 English</a> | <a href="README_spa.md">🇪🇸 Español</a> | <a href="README_fra.md">🇫🇷 Français</a> | <a href="README_ita.md">🇮🇹 Italiano</a> | <a href="README_deu.md">🇩🇪 Deutsch</a> | <a href="README_zho.md">🇨🇳 中文</a> | 🇯🇵 <b>日本語</b> | <a href="README_por.md">🇵🇹 Português</a></p>

### EH3D のマシン制御プログラムの共通基盤：ジョブの表示、テーマ、8 言語のインターフェース文言

<p align="center">
  <img src="https://img.shields.io/badge/License-GPL%203.0--or--later-blue.svg" alt="GPL 3.0 or later">
  <img src="https://img.shields.io/badge/Stack-Rust-DEA584.svg" alt="Rust">
  <img src="https://img.shields.io/badge/Languages-8-00E5FF.svg" alt="Languages">
  <img src="https://img.shields.io/badge/Version-0.0.1-0f6cbd.svg" alt="Version">
  <img src="https://img.shields.io/badge/Maturity-scaffolding-FFB020.svg" alt="Maturity">
  <img src="https://img.shields.io/badge/Tests-0-lightgrey.svg" alt="Tests">
</p>

---

**正直さの確認 - 今日実際に動くもの：** EH3D-COMMON の `eh3d-view` にあるジョブの表示を使う Rust クレートです。この表示はスイートの他のプログラムと共有され、テストもそちらにあります。表示には、作業テーブル上のジョブのシーン（ドリル径ごとの穴、ルーティング経路、作業の進み具合、重ねた穴あけジョブとルーティングジョブ）、テーブルの大きさに合わせた上面図と機械のアイソメトリック図、ジョブ開始前にテーブルに収まらないものを知らせるチェック、5 つのテーマ（Forge、Neon Arcade、Glass Pro、従来の Windows グレー、Windows の色に従うもの）、8 言語のインターフェース文言が含まれます。**ウィンドウもマシン制御もまだありません。** ここにあるものは実機で動かしたことがなく、このリポジトリ自体のテストもありません（表示を再エクスポートするだけです）。

---

> **安全性とセキュリティの現状：** このリポジトリは安全システムではなく、機械を動かしません。シリアルポートもネットワーク接続も開かず、他人のファイルも読みません。マシン制御を追加するときは、動作すると言う前にまずシミュレーターで、次に実機で確かめる必要があり、リミットスイッチと非常停止は引き続き機械自身の制御ユニットの役目です。

## 📋 ひと目で

| | |
|---|---|
| バージョン | `0.0.1` |
| 成熟度 | 足場 |
| テスト | 0 |
| 技術スタック | Rust |
| 実機での動作 | いいえ：ここには機械に接続するものはありません |

**移動先：** [概要](#-概要) · [アーキテクチャ](#-アーキテクチャ) · [クイックスタート](#-クイックスタート) · [テスト](#-テスト) · [構成](#-リポジトリの構成) · [関連](#-関連プロジェクト)

## 🚦 プロジェクトの状況

各行はこのリポジトリの一部分で、信号は今日どこまで進んでいるかを示します。計画ではなく、リポジトリが示せるもの（テスト、ビルド、実行）に基づいて書かれています。

🟢 完了・検証済み · 🟡 動作する（既知の制限あり） · 🟠 一部のみ、または未検証 · 🔴 未実装または動作しない

| 領域 | 状態 | ここでの意味 |
|---|---|---|
| 基盤ライブラリ | 🟡 | `eh3d-view` の `scene`、`texts`、`theme` をこのプログラム自身の名前で再エクスポートする Rust クレート。隣に EH3D-COMMON のフォルダーが必要です。 |
| シーンと表示 | 🟡 | EH3D-COMMON で作成し、コンピューター上でテスト済み：テーブル上のジョブ、上面図とアイソメトリック図、収まらないもののチェック。まだウィンドウには描画されません。 |
| テーマと文言 | 🟡 | コントラストチェック付きの 5 つのテーマと 8 言語のインターフェース文言を EH3D-COMMON でテスト済み。ウィンドウではまだ確認していません。 |
| テスト | 🟠 | このリポジトリ自体のテストは 0 件です（`cargo test`）。表示は、その置き場所である EH3D-COMMON でテストされています。 |
| ウィンドウとジョグ | 🔴 | 未着手。 |
| マシン制御 | 🔴 | 未着手：機械への接続も G コードもなく、ここから穴あけやルーティングのジョブは動きません。 |
| ドキュメント | 🟡 | 8 言語の README、2 つの図、変更履歴。プログラムができるまで書くことは多くありません。 |

## 🎯 概要

* **基盤ライブラリ**（`src/lib.rs`）：EH3D-COMMON の `eh3d-view` から `scene`、`texts`、`theme` を再エクスポートし、将来のプログラムが自身の名前でそれらを使い、スイートのどのプログラムも同じ方法でジョブを表示できるようにします。
* 作業テーブル上のジョブの**シーン**：ドリル径ごとの穴、ルーティング経路、作業の進み具合、重ねた穴あけジョブとルーティングジョブ、上面図、テーブルの大きさに合わせた機械のアイソメトリック図、テーブルに収まらないものの確認。
* **テーマ**：Forge（既定）、Neon Arcade、Glass Pro、従来の Windows グレー、Windows の色に従うテーマ。ユーザーはテキストファイルで自分のテーマを書けます。
* **インターフェース文言**：英語、スペイン語、イタリア語、ドイツ語、フランス語、ポルトガル語、中国語、日本語。
* 将来の姿：穴あけ、ルーティング、2D・3D の CNC 加工を 1 つで行うプログラムで、この基盤、EH3D-COMMON のジョブ ZIP（`eh3d-archive`）、EH3D-DRILLPRO と EH3D-ROUTEPRO のエンジンの上に作られます。**これらはまだ存在しません。**

## 🧩 EH3D での役割

EH3D-MOTIONCNC は EH3D の将来のマシン制御プログラムであり、今日はその基盤だけです。EH3D-COMMON にパスで依存し（`EH3D-COMMON` フォルダーをこのフォルダーの隣に置く必要があります）、他の EH3D リポジトリにはまだ依存しません。

## 🏛️ アーキテクチャ

<p align="center">
  <img src="images/architecture.svg" alt="EH3D-MOTIONCNC のアーキテクチャ：誰が使うか、何があるか、何に依存するか" width="100%">
</p>

各部分の組み合わせ：誰が使うのか、このリポジトリに何があるのか、何に依存するのか。図は `docs/architecture.json` から描かれ、コードと照合され、将来のものは将来として示されます。

## 🔒 セキュリティと安全

このリポジトリはシリアルポートもネットワーク接続も開かず、他人のファイルも読みません。各部分が読む外部テキストはユーザーが書いたテーマファイルだけで、信用せずに検査されます。機械は動かさず、安全システムでもありません。[SECURITY.md](SECURITY.md) を参照してください。

## 🚀 クイックスタート

安定版の Rust ツールチェーンと、このフォルダーの隣の `EH3D-COMMON` フォルダーが必要です。

```bash
cargo build --release
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --all --check
python -I tools/ci_validate.py
```

`build.bat` / `build.sh` はライブラリをビルドし、`test.bat` / `test.sh` はテストを実行し、`run.bat` / `run.sh` はまだプログラムがないことを表示してテストを実行します。`NOPAUSE` が設定されていなければ、スクリプトは最後にキー入力を待ちます。

## 🧪 テスト

ここで `cargo test` が報告するテストは 0 件です。このリポジトリは表示を再エクスポートするだけで、そのテストは EH3D-COMMON にあります。CI は `cargo fmt --check` と警告をエラーとする `cargo clippy` も実行し、`python -I tools/ci_validate.py` がマニフェスト、バージョン、変更履歴、8 つの README を確認します。

## 🏗️ ビルドとリリース

バージョンは `Cargo.toml` と `eh3d.project.json` にあり、各バージョンには [CHANGELOG.md](CHANGELOG.md) に項目があります。まだパッケージ化するプログラムがないため、ビルド済みバイナリはありません。

## 📂 リポジトリの構成

<p align="center">
  <img src="images/repository-structure.svg" alt="EH3D-MOTIONCNC のリポジトリ構成" width="100%">
</p>

<details>
<summary>ツリーのテキスト版</summary>

```text
EH3D-MOTIONCNC/
├── src/
│   └── lib.rs                                   EH3D-COMMON の eh3d-view からシーン、テキスト、テーマを再エクスポート
├── tools/                                       CI チェック、README チェック、図の生成ツール
├── docs/                                        アーキテクチャ図とリポジトリ構成図の元データ
├── .github/                                     CI ワークフロー、Issue と PR のテンプレート
├── images/                                      バナーとアーキテクチャ・リポジトリ構成の図
├── build.sh build.bat                           ライブラリをビルド
├── run.sh run.bat                               まだプログラムがないことを表示し、テストを実行
├── eh3d.project.json                            マニフェスト：バージョン、役割、成熟度、ビルドコマンド
├── Cargo.toml Cargo.lock                        Rust パッケージと固定された依存関係
├── CHANGELOG.md                                 各バージョンの変更点
├── SECURITY.md SUPPORT.md                       脆弱性の報告方法と問い合わせ先
└── CONTRIBUTING.md CODE_OF_CONDUCT.md LICENSE   貢献方法、行動規範、ライセンス
```

</details>

## 🗺️ ロードマップと既知の制限

**現在の既知の制限**

* ウィンドウもマシン制御もありません。ここにあるものは実機で動かしたことがありません。
* このリポジトリにはテストがありません。表示は EH3D-COMMON でテストされています。
* パスで依存しているため、隣に `EH3D-COMMON` フォルダーが必要です。

**次のステップ**

* 現在ある機械制御プログラム（ネイティブのものと .NET のもの）を調べ、オペレーターに必要なものを決める。
* シーン、テーマ、インターフェース文言を表示するウィンドウ。
* ジョグと G コード：機械への接続（まずシミュレーターで）。
* 実行中のジョブの 2D・3D 表示。
* EH3D-DRILLPRO と EH3D-ROUTEPRO のエンジンを統合し、1 つのプログラムで穴あけとルーティングを行う。

## 🌍 言語

README とテキストは、英語、スペイン語、フランス語、イタリア語、ドイツ語、中国語、日本語、ヨーロッパポルトガル語で用意されています。

## 🔗 関連プロジェクト

* [EH3D-COMMON](https://github.com/JuanenRac/EH3D-COMMON)：表示（`eh3d-view`）とジョブ ZIP（`eh3d-archive`）の置き場所
* [EH3D-DRILLPRO](https://github.com/JuanenRac/EH3D-DRILLPRO)：エンジンがここに統合される予定の穴あけプログラム
* [EH3D-ROUTEPRO](https://github.com/JuanenRac/EH3D-ROUTEPRO)：エンジンがここに統合される予定のルーティングプログラム

## 📚 ドキュメント

* [変更履歴](CHANGELOG.md)
* [セキュリティポリシー](SECURITY.md)と[サポート](SUPPORT.md)
* ジョブの表示のドキュメントは、その置き場所である EH3D-COMMON にあります。

## 🤝 コントリビュートとコミュニティ

* プルリクエストの前に `cargo fmt --all --check`、`cargo clippy --all-targets -- -D warnings`、`cargo test`、`python -I tools/ci_validate.py` を実行してください。すべて通る必要があります。[CONTRIBUTING.md](CONTRIBUTING.md) と[行動規範](CODE_OF_CONDUCT.md)を参照してください。
* README は 8 言語あります。文章を変えるときは 8 つのファイルすべてを変更し、2 つの図は `tools/make_architecture.py` と `tools/make_structure.py` が `docs/architecture.json` と `docs/structure.json` から描きます。
* 機械で動かしていないものを、動作するものとして書かないでください。
* 脆弱性は非公開で報告してください：[SECURITY.md](SECURITY.md)。質問、アイデア、報告：electrohobby3d@gmail.com（[SUPPORT.md](SUPPORT.md)）。

## 👤 作者

**JuanenRac (Electro Hobby 3D)** · electrohobby3d@gmail.com

## 📜 ライセンス

ソフトウェア：GPL-3.0-or-later。[LICENSE](LICENSE) を参照。スイートのハードウェア設計は CERN OHL-S を使用します。
