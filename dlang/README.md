# D言語 for AtCoder

AtCoderの問題をD言語で解くためのディレクトリ。

## 環境

- DMD v2.112.0
- LDC 1.42.0 (AtCoder本番のジャッジは LDC を使用)
- online-judge-tools (`oj`)

## 使い方

### 1. 問題のセットアップ

`dlang/` ディレクトリで以下を実行:

```sh
./setup.sh abc199_a
```

これで:
- `abc199_a/` ディレクトリが作成される
- サンプル入出力が `abc199_a/test/` にダウンロードされる
- `template/` から `main.d`, `run.sh`, `test.sh` が配置される

### 2. 解答を書く

```sh
cd abc199_a
$EDITOR main.d
```

### 3. サンプルでテスト

問題ディレクトリ内で:

```sh
bash run.sh
```

ビルド (`dmd`) → `oj test` でサンプル全件チェック → 生成物の自動削除、が一括実行される。

単発で動かす場合:

```sh
dmd -run main.d < test/sample-1.in
```

### 4. ユニットテスト

D言語の `unittest` ブロックを実行する:

```sh
bash test.sh
```

### 5. 提出

問題ディレクトリ内で:

```sh
oj s main.d
```

URLを聞かれる/拒否される場合は明示指定:

```sh
oj s https://atcoder.jp/contests/abc199/tasks/abc199_a main.d
```

### 6. 本番風タイマー

コンテスト相当の制限時間でカウントダウンしたいとき:

```sh
./timer.sh 100m      # ABC本番 (100分)
./timer.sh 1h40m     # 同上 (時間指定でも可)
./timer.sh 300       # 300秒
./timer.sh 5m30s     # 5分30秒
```

- 残り時間がターミナルに表示される
- 終了時に `notify-send` でデスクトップ通知
- `Ctrl+C` で中断可

## sandbox

`sandbox/` は自由に試し書きするための場所。`build_run.fish` でビルド＆実行できる:

```sh
cd sandbox
fish build_run.fish main.d           # 通常実行
fish build_run.fish --test main.d    # unittest付き実行
```

## ディレクトリ構成

```
dlang/
├── README.md
├── dub.sdl               # serve-d用 (VSCode補完)
├── setup.sh              # 問題ディレクトリの初期化
├── timer.sh              # 本番風カウントダウンタイマー
├── template/
│   ├── main.d            # 解答テンプレート
│   ├── run.sh            # oj test 起動
│   └── test.sh           # unittest 実行
├── sandbox/
│   ├── main.d            # 試し書き用
│   └── build_run.fish    # ビルド＆実行スクリプト
└── abc199_a/             # 各問題ディレクトリ (setup.shで生成)
    ├── main.d
    ├── run.sh
    ├── test.sh
    └── test/
        ├── sample-1.in
        └── sample-1.out
```
