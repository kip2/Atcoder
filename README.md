# Atcoder

主にRust用のAtcoder置き場とする

他の言語の管理については、思いついた段階でおいおい追加していくことにすること

---

# 実行方法

## テストケースの実行


```shell
# 作成したコンテストのディレクトリに移動してから行う
# 以下は一例
cd tessoku-book

cargo compete test {問題名}
```

## 提出

```shell
# 作成したコンテストのディレクトリに移動してから行う
# 以下は一例
cd tessoku-book

cargo compete submit {問題名}
```


## rustupのバージョン管理について

Atcoderのコンパイラのバージョンに合わせるため、以下の方法をとっている。

1. rust-toolchain.tomlを設置
2. 以下のように設定を記載する

```toml
[toolchain]
channel = "1.70.0"
```

## proconio

atcoderではproconioという、競プロ用の入力をサポートするクレートが使えるらしい

Atcoderのバージョンに合わせるために、以下の記事を参考にした

https://zenn.dev/toga/books/rust-atcoder/viewer/input

2024/06現在では、0.4.5らしいので、以下のようにして揃えた

```toml
[dependencies]
proconio = "0.4.5"
```
