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
  <img src="https://img.shields.io/badge/Version-0.0.3-0f6cbd.svg" alt="Version">
  <img src="https://img.shields.io/badge/Maturity-scaffolding-FFB020.svg" alt="Maturity">
  <img src="https://img.shields.io/badge/Tests-12-0f6cbd.svg" alt="Tests">
</p>

---

**正直さの確認 - 現在実際に動くもの：**ウィンドウを持つ Rust プログラム（`motioncnc`）で、**模擬**マシンを制御します。マシンの位置（ワークとマシン）を表示し、ジョグパッドで手動移動し、ゼロを設定し、原点復帰を行い、オペレーターが入力したコマンドを受け付け、G コードのプログラムを 1 行ずつ実行（一時停止、停止、非常停止付き）し、テーブル上にプログラムの経路を描いて実行済みの行に印を付け、コントローラーが拒否したところで止まります。ウィンドウは 8 言語と 5 つのテーマに対応します。コントローラーのインターフェース（EH3D-COMMON の `eh3d-controller`）には、スクリプト化した回線に対する GRBL が書かれており、他のファミリーは未対応として一覧にあります。12 件のテストがコンピューター上で動きます。**ここにあるものは実機で実行されたことがなく**、シリアルポートもネットワーク接続も開かず、円弧やその他のコードはシミュレーターが描画も実行もしません。

---

> **安全状態：**このプログラムは模擬マシンのみを制御し、シリアルポートもネットワーク接続も開きません。実際のコントローラーを接続する際は、動作すると言う前にまずシミュレーターで、次に実機で実証する必要があります。リミットスイッチと非常停止はマシン自身の制御ユニットの役目のままで、ウィンドウの非常停止は安全回路ではありません。

## 📋 ひと目で

| | |
|---|---|
| バージョン | `0.0.3` |
| 成熟度 | 足場 |
| テスト | 12 |
| 技術スタック | Rust |
| 実機での動作 | いいえ：模擬マシンのみを制御します |

**移動先：** [概要](#-概要) · [アーキテクチャ](#-アーキテクチャ) · [クイックスタート](#-クイックスタート) · [テスト](#-テスト) · [構成](#-リポジトリの構成) · [関連](#-関連プロジェクト)

## 🚦 プロジェクトの状況

各行はこのリポジトリの一部分で、信号は今日どこまで進んでいるかを示します。計画ではなく、リポジトリが示せるもの（テスト、ビルド、実行）に基づいて書かれています。

🟢 完了・検証済み · 🟡 動作する（既知の制限あり） · 🟠 一部のみ、または未検証 · 🔴 未実装または動作しない

| 領域 | 状態 | ここでの意味 |
|---|---|---|
| 基盤ライブラリ | 🟡 | `eh3d-view` の `scene`、`texts`、`theme` をこのプログラム自身の名前で再エクスポートする Rust クレート。隣に EH3D-COMMON のフォルダーが必要です。 |
| シーンと表示 | 🟡 | EH3D-COMMON で作成・テスト済み。このプログラムのウィンドウがテーブル上にプログラムの経路とツールの位置を描きます（コンピューター上で確認済み）。 |
| テーマと文言 | 🟡 | 5 つのテーマと 8 言語のインターフェースの言葉を、このプログラムのウィンドウで確認済み。 |
| テスト | 🟡 | コンピューター上で 12 件のテストが通ります（`cargo test --features gui`）：セッション（ジョグ、ゼロ、原点復帰、プログラム、一時停止、アラーム、非常停止）、プログラムの経路、コマンドライン。模擬コントローラーは EH3D-COMMON でテストします。 |
| ウィンドウとジョグ | 🟡 | コンピューター上で模擬コントローラーと動作します：位置、ジョグパッド、ステップ、送り、原点復帰、ゼロ、ロック解除、コマンド行、ログ。実機はありません。 |
| マシン制御 | 🟠 | シミュレーターのみ：プログラムは 1 行ずつ実行され、一時停止、停止、非常停止があります。ここからはまだ実際のコントローラーに接続していません。 |
| ドキュメント | 🟡 | 8 言語の README、2 つの図、3 つのサンプルプログラム、変更履歴。 |

## 🎯 概要

* **ウィンドウ**（`src/gui.rs`、`motioncnc`）：マシンの位置（ワークとマシン）、ステップと送り付きのジョグパッド、原点復帰、ゼロ、ロック解除、非常停止、ログ付きのコマンド行、テーブル上に経路を表示するプログラム。
* **セッション**（`src/session.rs`）：`eh3d-controller` の任意のコントローラーに対してプログラムが行うこと：ジョグ、ゼロ、原点復帰、1 行の送信、プログラムの 1 行ずつの実行、一時停止、停止、アラーム。模擬コントローラーでテストします。
* **プログラムの経路**（`src/gcode.rs`）：直線移動（`G0`、`G1`、絶対または相対、mm またはインチ）を実行前に描画します。円弧は描画しません。
* **模擬コントローラー**（EH3D-COMMON の `eh3d_controller::sim`）：移動範囲を超えるとアラームを出す、メモリ上のマシン。市場のコントローラーファミリーは `eh3d-controller` に一覧があり、GRBL が最初に書かれたもので、他は未対応として表示されます。
* `eh3d-view` の**テーマと言葉**：Forge（既定）、Neon Arcade、Glass Pro、クラシック Windows グレー、Windows に追従するテーマ、そして英語、スペイン語、イタリア語、ドイツ語、フランス語、ポルトガル語、中国語、日本語のインターフェース。将来は EH3D-DRILLPRO と EH3D-ROUTEPRO のエンジンの上で、穴あけ、ルーティング、2D・3D の CNC 加工を行う 1 つのプログラムになります。その部分はまだありません。

## 🧩 EH3D での役割

EH3D-MOTIONCNC は EH3D のマシン制御プログラムで、現在は模擬マシン上で動作します。パスで EH3D-COMMON に依存し（フォルダー `EH3D-COMMON` をこのフォルダーの隣に置く必要があります：ビュー、テーマ、言葉、コントローラー）、現時点で他の EH3D リポジトリには依存しません。

## 🏛️ アーキテクチャ

<p align="center">
  <img src="images/architecture.svg" alt="EH3D-MOTIONCNC のアーキテクチャ：誰が使うか、何があるか、何に依存するか" width="100%">
</p>

各部分の組み合わせ：誰が使うのか、このリポジトリに何があるのか、何に依存するのか。図は `docs/architecture.json` から描かれ、コードと照合され、将来のものは将来として示されます。

## 🖼️ スクリーンショット

<p align="center">
  <img src="images/screenshots/program-running.png" alt="a program running" width="100%">
</p>

<p align="center"><em>模擬マシンで実行中のプログラム：実行済みの行は緑、現在の行は明るく表示され、経路がテーブル上に描かれ、ツールはマシンのある位置にあります。</em></p>

<p align="center">
  <img src="images/screenshots/spiral-neon.png" alt="Neon Arcade theme" width="100%">
</p>

<p align="center"><em>Neon Arcade テーマと、実行が終わった長いプログラム。</em></p>

<p align="center">
  <img src="images/screenshots/square-german.png" alt="German interface" width="100%">
</p>

<p align="center"><em>Glass テーマのドイツ語インターフェース：ウィンドウのすべての言葉が言語に従います。</em></p>

## 🔒 セキュリティと安全

このプログラムはシリアルポートもネットワーク接続も開かず、オペレーターが選ぶ G コードファイル（テキストとして読み込み、自動では実行しない）以外の他人のファイルも読みません。模擬マシンのみを制御し、安全システムではありません。[SECURITY.md](SECURITY.md) を参照してください。

## 🚀 クイックスタート

安定版の Rust ツールチェーンと、このフォルダーの隣の `EH3D-COMMON` フォルダーが必要です。

```bash
cargo build --release --features gui
cargo test --features gui
cargo clippy --all-targets --features gui -- -D warnings
cargo fmt --all --check
python -I tools/ci_validate.py
cargo run --release --features gui -- examples/02_frame_and_pocket.nc
```

`build.bat` / `build.sh` はプログラムをビルドし、`test.bat` / `test.sh` はテストを実行し、`run.bat` / `run.sh` はサンプルプログラムでウィンドウを開きます。`NOPAUSE` を設定しない限り、スクリプトは最後にキー入力を待ちます。

## 🧪 テスト

`cargo test --features gui` は 12 件のテストを実行します：セッション（ジョグとゼロ、プログラムの 1 行ずつの実行、一時停止と再開、拒否された行、非常停止とアラーム、コマンドのログ、原点復帰）、プログラムの経路、コマンドライン。ビューと模擬コントローラーは EH3D-COMMON でテストします。CI は `cargo fmt --check` と警告をエラーとする `cargo clippy` も実行し、`python -I tools/ci_validate.py` がマニフェスト、バージョン、変更履歴、8 つの README を確認します。

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
│   ├── lib.rs                                   モジュールと、このプログラム名で再エクスポートした eh3d-view の表示
│   ├── gui.rs                                   ウィンドウ：位置、ジョグ、プログラム、経路、コマンド、ログ
│   ├── session.rs                               コントローラーで行うこと：ジョグ、ゼロ、原点復帰、プログラムの 1 行ずつの実行、一時停止、停止、アラーム
│   ├── gcode.rs                                 G コードプログラムの直線移動。経路の描画用
│   ├── cli.rs                                   コマンドライン：ファイル、言語、テーマ、画像のオプション
│   └── bin/motioncnc.rs                         ウィンドウを開くプログラム
├── examples/                                    3 つのサンプル G コード：正方形から長い渦巻きまで
├── tools/                                       CI チェック、README チェック、図の生成ツール
├── docs/                                        アーキテクチャ図とリポジトリ構成図の元データ
├── .github/                                     CI ワークフロー、Issue と PR のテンプレート
├── images/                                      バナー、2 つの図、スクリーンショット
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

* ここにあるものは実機で実行されたことがありません。コントローラーはシミュレーターで、シリアルポートもネットワーク接続も開きません。
* シミュレーターは基本的な G コード（`G0`、`G1`、`G90`、`G91`、`G20`、`G21`、`G92`、`M3` から `M9`）のみ受け付けます。円弧、固定サイクル、プロービングは拒否され、経路は円弧を描きません。
* パスで依存しているため、隣に `EH3D-COMMON` フォルダーが必要です。

**次のステップ**

* 実際のコントローラー：まずシリアルポート経由の GRBL、次に他のファミリー、その後 HYDRA-UMC のコントローラー。
* シミュレーターと経路での円弧とその他の G コード。
* 切削の高さを含む経路の 3D 表示。
* プロービング、ツールチェンジャー、マシンの材料。
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
