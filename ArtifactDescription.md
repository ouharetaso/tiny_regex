# Artifact Description

## 概要：表現力の向上と高速化

### 改変対象
#### TinyRegex
https://github.com/ouharetaso/tiny_regex  
学習用にRustで書かれた小さい正規表現エンジン．  

### 改変内容
* 表現力の向上
  * 否定文字クラス`[^...]`のサポート
  * 任意の文字をマッチさせる`.`のサポート
  * 一部のエスケープシーケンス(`\n`, `\t`, `\r`, `\0`)のサポート
* 高速化
  * on-the-fly DFAによる高速化

## クイックスタート

Dockerイメージの準備
```
docker pull ouhare/2024-s2210298-tiny_regex
docker run -it --rm --name ae_tiny_regex ouhare/2024-s2210298-tiny_regex
```
コンテナ内での動作チェック
```
echo "Das Alter beginnt in dem Augenblick, wo man nicht mehr ohne die Vergangenheit leben kann."|cargo --quiet run --bin tiny_grep "[a-zA-Z][a-zA-Z]*"
```

## 評価手順
### 表現力の向上
#### 否定文字クラス`[^...]`のサポート
```
echo "エドワード・ノートン"|cargo --quiet run --bin re_place "[^・ー]" "ボ"
```
を実行すると
```
ボボボーボ・ボーボボ
```
と出力される．  
* `cargo --quiet run --bin re_place`は`re_place`というバイナリを実行するコマンド．  
  * `re_place`は，第1引数の正規表現にマッチする文字列を第2引数の文字列に置換する．
* `"[^・ー]"`は否定文字クラスで`・` `ー`以外の1文字を表す．  
* `"ボ"`は否定文字クラスにマッチした文字列を置換する文字． 

#### 任意の文字をマッチさせる`.`のサポート
```
echo "I have a 🦀 that is cute. You have a 🐍 that is cute."|cargo --quiet run --bin tiny_grep "a .* that is cute"
```
を実行すると
```
a 🦀 that is cute
a 🐍 that is cute
```
と出力される．

* `cargo --quiet run --bin tiny_grep`は`tiny_grep`というバイナリを実行するコマンド．  
  * `tiny_grep`は，第1引数の正規表現にマッチする部分文字列を1行ずつ標準出力に出力する．
* `"a .* that is cute"`はかわいいなにかを表現する英語の名詞句を表す．  

#### 一部のエスケープシーケンスのサポート
1. 入力の準備
```
echo -en ".*\t" > unescaped_regex.txt
echo -En ".*\t" > escaped_regex.txt
```
を実行すると`unescaped_regex.txt`と`escaped_regex.txt`が生成される．

* `unescaped_regex.txt`はエスケープシーケンスを用いずにタブ文字をそのまま表現している．
  * `cat unescaped_regex.txt`を実行すると`.*	`が出力される．
* `escaped_regex.txt`はタブ文字をエスケープシーケンスで表現している．
  * `cat escaped_regex.txt`を実行すると`.*\t`が出力される．

2. エスケープシーケンスのサポートの確認
```
echo -en "Hello\tWorld\t"|cargo --quiet run --bin tiny_grep "$(cat unescaped_regex.txt)"
echo -en "Hello\tWorld\t"|cargo --quiet run --bin tiny_grep "$(cat escaped_regex.txt)"
```
を実行するとどちらのコマンドも次のような出力が得られる．
```
Hello	
World	
```

### 高速化
1. 入力の準備
```
echo a{,}{,,}{,,,,}|tr -d " " > input.txt
echo -n "(a*)"{,}{,,}{,,,,} a{,}{,,}{,,,,}|tr -d " " > regex.txt
```
を実行すると，`input.txt`と`regex.txt`が生成される．

* `echo a{,}{,,}{,,,,}|tr -d " " > input.txt`は`a`が30回連続した文字列を`input.txt`に書き出している．
* `echo -n "(a*)"{,}{,,}{,,,,} a{,}{,,}{,,,,}|tr -d " " > regex.txt`は`(a*){30}a{30}`と等価な正規表現を`regex.txt`に書き出している．

2. 元の実装による実行時間の測定
```
time cat input.txt | cargo --quiet run --bin tiny_grep "$(cat regex.txt)"
```
を実行すると以下のような出力が期待される．
```
aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa

real    0m2.524s
user    0m2.511s
sys     0m0.014s
```
ここでの`user`の項が実際にプログラムを実行するのにかかった時間である．

3. 新しく実装した手法を用いた実行時間の測定
```
time cat input.txt | cargo --quiet run --features on_the_fly --bin tiny_grep "$(cat regex.txt)"
```
を実行すると以下のような出力が期待される．
```
aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa

real    0m0.139s
user    0m0.127s
sys     0m0.013s
```
新しく実装したOn-the-fly手法によってマッチング処理が高速化されていることが確認できるはずである．  
上記の結果は筆者の環境(MacbookAir M1, 2020)での結果であり，環境によって異なる可能性があるが，概ね同じような傾向が得られるはずである．

## 制限と展望
### より高度な正規表現のサポート
今の実装では量指定子は`*`(0回以上の繰り返し)しかないのでより表現力を高めるために`?`や`+`や`{n}`などを実装する余地がある．
