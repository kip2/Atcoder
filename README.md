# Atcoder

主にRust用のAtcoder置き場

ツールとしては[cargo-compete](https://github.com/qryxip/cargo-compete/tree/master)を使用している

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

## ワークスペースへのコンテストフォルダの追加

まずコンテストフォルダをcompeteを使用して追加する

```sh
# comtest-name examples: ABC199, ABC621, etc...
cargo compete new <contest-name>
```

次に、各コンテストフォルダでrust-analyzerを効かせるために、workspaceへの登録を行う

フォルダパスと、Cargo.tomlのディレクトリを設定する

./Atcoder.code-workspace

```json
{
	"folders": [
		{
			"path": "."
		},
		{
			"path": "abc199"
		},
		{
			"path": "tessoku-book"
		}
	],
	"settings": {
		"rust-analyzer.linkedProjects": [
			"abc199/Cargo.toml",
			"tessoku-book/Cargo.toml"
		]
	}
}
```
