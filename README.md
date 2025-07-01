# Atcoder

Atcoderの問題を解くためのリポジトリ

各種言語ごとにディレクトリを切って、各言語ごとの書き方の違いについても学んでいくのも目的

## 対象言語

- Rust
- Haskell
- Scala
- TypeScript
- D言語
- Clojure
- Common Lisp

## 各言語で行うこと

ローカルで実行、実験ができる環境を整えること。

実際のACの確認はAtcoderサイト上で行えば良い。

### 各言語で用意するもの

#### Rust

Rustは`cargo-compete`を用いて実現するので、特に考える必要はない。

#### 他の言語

`oj`を利用する。

各種問題のサンプルダウンロードは以下のように行うこと。

以下は一例。

```sh
# 先にディレクトリを作成しておくと良い
mkdir abc199_a
cd abc199_a

# サンプル入力値などをダウンロード
oj d https://atcoder.jp/contests/abc199/tasks/abc199_a
```

ダウンロードすると、`test/sample-1.in`のように、`test`ディレクトが作成され、その中にサンプルデータが取得される。

テストする場合は、以下のコマンドで実行ができる。

```sh
oj t -c <command>
```

