# Scala

ScalaによるAtcoderの練習用ディレクトリ

## 使い方

`oj`を用いてローカル環境の構築を行っている。

### サンプルのダウンロード

該当するAtcoderの問題ページのURLを確認する。
`https://atcoder.jp/<ID>/tasks/<ID>_a`の形式になる。

以下のコマンドによって、対象のページの入出力データを取得する。

```sh
# AやBなどの、問題のランクを含めて指定すること
./setup.sh "abc123_a"
```

### テスト実行

以下のシェルスクリプトにより実行する。

```sh
./test.sh
```

あるいは、`oj`の以下のコマンドにより実行する。

```sh
oj t -c "scala-cli Main.scala"
```

