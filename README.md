# これは何か？
ハフマン符号化を行うプログラムです。優先度付きキューと辞書（連想配列）を用いて実装しています。
Java, Python, Ruby, Rust の 4 言語に対応しています。
Ruby と Rust に対してはテスト駆動開発で開発したためテストコードがあります。

|言語|
|:-:|
|[Java](https://github.com/NAVYSHUNTA/huffman_code/blob/main/java/Main.java)|
|[Python](https://github.com/NAVYSHUNTA/huffman_code/tree/main/python)|
|[Ruby](https://github.com/NAVYSHUNTA/huffman_code/tree/main/ruby/src)|
|[Rust](https://github.com/NAVYSHUNTA/huffman_code/tree/main/rust/src)|

# 使い方（Python の場合）
プログラムを実行するとハフマン符号化を行いたい文字列の入力を求められるので好きな文字列を与えます。
例えば `すもももももももものうち` を入力として与えると実行結果は下記の通りになります。
```
ハフマン符号化したい文字列を英小文字か平仮名で入力してください: すもももももももものうち
---ハフマン符号の対応表---
す: 001
も: 1
の: 011
う: 000
ち: 010
--------------------------
ハフマン符号化する前の文字列: すもももももももものうち
ハフマン符号化した後の文字列: 00111111111011000010
```

# 言語ごとの実行方法と補足説明
## Java の場合
```
$ javac Main.java && java Main
```

## Python の場合
```
$ python main.py
```

## Ruby の場合
```
$ ruby main.rb
```
なお、Ruby の標準ライブラリには優先度付きキューがないため外部ライブラリを利用しています。そのため、外部ライブラリをインストールしていない場合にエラーが出る場合があります。下記のコマンドで今回のプログラムに必要な外部ライブラリのみのインストールを行うことができます。
```
$ gem install ac-library-rb
```

## Rust の場合（Cargo）
```
$ cargo run
```
