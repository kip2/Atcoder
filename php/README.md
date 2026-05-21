# PHP for AtCoder

AtCoderの問題をPHPで解くためのディレクトリ。

## 環境

- PHP 8.2.31 (mise でインストール、`mise.toml` で固定)
- このディレクトリに `cd` すると自動で 8.2 系に切り替わる

AtCoder本番は PHP 8.2.8 (パッチバージョン差のみ、言語仕様は同一)。

## 使い方

### 1. 問題のセットアップ

`php/` ディレクトリで以下を実行:

```sh
./setup.sh abc199_a
```

これで:
- `abc199_a/` ディレクトリが作成される
- サンプル入出力が `abc199_a/test/` にダウンロードされる
- `template/main.php` と `template/run.sh` が配置される

### 2. 解答を書く

```sh
cd abc199_a
$EDITOR main.php
```

### 3. サンプルでテスト

問題ディレクトリ内で:

```sh
bash run.sh
```

中身は `oj test -c "php main.php"`。`test/` 配下の全サンプルが自動チェックされる。

単発で動かす場合:

```sh
php main.php < test/sample-1.in
```

### 4. 提出

問題ディレクトリ内で:

```sh
oj s main.php
```

URLを聞かれる/拒否される場合は明示指定:

```sh
oj s https://atcoder.jp/contests/abc199/tasks/abc199_a main.php
```

ログインが必要な場合は親ディレクトリの README にあるトラブルシューティング参照。

### 5. 本番風タイマー

コンテスト相当の制限時間でカウントダウンしたいとき:

```sh
./timer.sh 100m      # ABC本番 (100分)
./timer.sh 1h40m     # 同上 (時間指定でも可)
./timer.sh 300       # 300秒
./timer.sh 5m30s     # 5分30秒
```

- 残り時間がターミナルに表示される
- 終了時に `notify-send` でデスクトップ通知 (mako等が必要)
- `Ctrl+C` で中断可

## ディレクトリ構成

```
php/
├── README.md
├── mise.toml             # PHPバージョン固定
├── setup.sh              # 問題ディレクトリの初期化
├── timer.sh              # 本番風カウントダウンタイマー
├── template/
│   ├── main.php          # 解答テンプレート
│   └── run.sh            # oj test 起動スクリプト
└── abc199_a/             # 各問題ディレクトリ
    ├── main.php
    ├── run.sh
    └── test/
        ├── sample-1.in
        └── sample-1.out
```
