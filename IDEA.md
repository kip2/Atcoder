# Inline-Expansion Build Tool — Design Notes

AtCoder は提出が **1ファイル** に限られる。ローカルでは `require_once` などで共通ライブラリに切り出しておき、提出時に **1ファイルに展開** するビルドツールを作る。

このドキュメントはツールの設計メモ。

---

## 目的

```
編集する main.php                      提出する main.php
──────────────                         ───────────────
<?php                                 <?php
require_once '../lib/io.php';   →     // === BEGIN: lib/io.php ===
                                       function get_input() { ... }
function solve() { ... }              function println() { ... }
                                       // === END: lib/io.php ===
                                       function solve() { ... }
```

- ローカルでは分離 → 編集効率・IDE補完◎
- 提出時に inline 展開 → ジャッジで動く

## 想定インターフェース

```sh
# 標準出力に展開結果
inline main.php > /tmp/submit.php

# 出力ファイル指定
inline main.php -o /tmp/submit.php

# ドライラン (展開先一覧の表示のみ)
inline main.php --dry-run

# パイプで oj submit に直結
inline main.php | oj s --file /dev/stdin <URL>
```

問題ディレクトリの `submit.sh` 例:

```sh
#!/bin/bash
set -euo pipefail
TMP=$(mktemp --suffix=.php)
trap 'rm -f "$TMP"' EXIT
inline main.php -o "$TMP"
oj t -c "php $TMP"        # 展開後コードでテスト
oj s --no-open "$TMP" "$(cat URL)"
```

---

## 実装言語の候補

| 候補 | 長所 | 短所 |
|---|---|---|
| **Rust** ✓推奨 | 単一バイナリ配布、cargoで install、リポジトリで既に使用 | テキスト処理にしてはやや冗長 |
| Go | 単一バイナリ、go run も可 | 同上 |
| TypeScript (Deno/Bun) | 1ファイル即書き、配布も簡単 | ランタイム依存 |
| Python | 即プロトタイプ可能、システム既存 | mise管理外で動かしたい時に環境依存 |

**進め方推奨**:
1. **MVP は Python or Deno** で 1〜2 時間で書く。PHPだけ対応・最低限の置換で OK
2. 使い心地が固まったら **Rust で書き直して `cargo install` 配布**

---

## アーキテクチャ

```
   main.php
      ↓
  ┌─────────┐
  │ Parser  │ ← 言語ルール(include_pattern等)
  └────┬────┘
       ↓
  ┌─────────┐
  │ Resolver│ ← visited set, 循環検出
  └────┬────┘
       ↓
  ┌─────────┐
  │ Emitter │ ← BEGIN/END マーカー, <?php除去
  └────┬────┘
       ↓
  bundled.php
```

### Parser
入力ファイルを行単位で読み、include 行を検出する。

### Resolver
- include 先パスを **入力ファイルのディレクトリ基準** で解決
- `__DIR__` のような特殊定数は当該ディレクトリに置換
- 同一ファイルが2回以上参照されても **1回だけ展開**
- 循環参照を検出したらエラー終了 (どの経路で循環したかを表示)
- 展開後ファイルの中に更に include があれば再帰的に展開

### Emitter
- 展開部分を `// === BEGIN: <path> ===` / `// === END: <path> ===` で囲む(言語のコメント記法に合わせる)
- 展開ファイルの先頭 `<?php` 等の言語ヘッダは剥がす
- 入力ファイル先頭の `<?php` は残す
- 元の include 行は **削除して展開内容に置換**

---

## 多言語ルール

設定ファイル `inline.toml` に言語ごとのパターンを定義:

```toml
[lang.php]
extensions = ["php"]
include_patterns = [
  '^\s*require(_once)?\s+(__DIR__\s*\.\s*)?[''"]([^''"]+)[''"]\s*;',
]
strip_header = '^<\?php\s*$'
comment_prefix = "//"

[lang.ruby]
extensions = ["rb"]
include_patterns = [
  '^\s*require_relative\s+[''"]([^''"]+)[''"]',
]
comment_prefix = "#"

[lang.python]
extensions = ["py"]
include_patterns = [
  '^\s*from\s+\.(\S+)\s+import\s+\*',
]
comment_prefix = "#"
```

設定なしでも拡張子から既定ルールで動かせるようにする(PHPだけ組み込みデフォルト等)。

---

## マッチング方式 (2系統)

1. **自然な include 構文**: `require_once '...'` をマッチして自動展開
   - 長所: 既存コードを書き換えなくていい、IDEのジャンプも効く
   - 短所: パスの形式に揺れがあると対応が増える

2. **明示マーカー**: `// @inline path/to/lib.php` のコメントで指示
   - 長所: 言語非依存・確実
   - 短所: 手書きが必要、IDEはこのコメントを理解しない

**初版は (1) のみ**。後で (2) を追加。

---

## エラーハンドリング

- include 先が見つからない → どの行/どのファイルかを明示してexit 1
- 循環参照 → サイクル経路をフルパスで表示
- 構文エラーはツール側でチェックしない (展開後 `php -l` で別途検査可)

---

## ワークフロー統合

| シーン | コマンド |
|---|---|
| 編集 | `$EDITOR main.php` (require_once のまま) |
| ローカルテスト | (オプションA) `php main.php < in` で `require_once` を実行時解決<br>(オプションB) `inline main.php \| php` で展開後を実行 |
| サンプル一括 | `inline main.php -o /tmp/x.php && oj test -c "php /tmp/x.php"` |
| 提出 | `submit.sh` がinline+oj sを実行 |
| CI (任意) | `find . -name main.php -exec inline {} -o /dev/null \;` で全問題が展開可能か検証 |

---

## 出力ファイルの管理

- 既定では **一時ファイル** (`/tmp/inline-<task>.php`)
- リポジトリに展開済みコピーを残したい場合は `dist/` 配下、`.gitignore` 推奨
- 提出履歴は AtCoder マイページ + git の元 main.php で十分

---

## 開発ステップ

1. **MVP** (1〜2 時間 / Python or Deno)
   - PHP の `require_once` をマッチして単一ファイル展開
   - 循環検出なし、visited set のみ
   - 出力は stdout のみ
2. **v0.2**
   - 多言語ルール(Ruby, Python)
   - 循環検出、エラー表示の整形
   - `-o`, `--dry-run` 追加
3. **v0.3 / Rust 移植**
   - `cargo install` 可能なバイナリ
   - `inline.toml` で言語ルールを外部設定化
4. **v0.4 (任意)**
   - 明示マーカー方式
   - `--check` モード(全問題スキャン)

---

## オープン課題

- **Tree shaking (未使用関数の削除)**: PHP は動的呼び出し(`$f()`, `call_user_func`)があり静的解析が難しい。AtCoderのコードサイズ上限は十分大きい(数百KB)ので、初版は **全部入り** で問題ない。
- **namespace / use 宣言**: 競プロ用途では namespace は使わない方針。仮に使う場合は use 宣言の重複排除が必要。
- **`__DIR__` 以外の動的パス**: `require $base . '/foo.php'` のような動的解決はサポートしない。事前に怒る。
- **複数ファイルが同じ関数を定義**: PHPでは fatal error なので、ライブラリ側で重複定義しないことを利用者責任とする(検出はオプション)。

---

## 命名候補

- `inline` (短いが一般語すぎる)
- `bundlephp` (PHP特化感)
- `atcb` (atcoder-bundle)
- `oj-inline` (oj系の命名規則に乗る)

`oj-inline` あたりが分かりやすい?
