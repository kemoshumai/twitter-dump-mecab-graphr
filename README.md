Twitter Dump Mecab Graphr
====

Twitterのデータから2023年(注意：UTC+0)にTwitterでつぶやいた固有名詞トップ10を発見するプログラム

## Demo
```
==== RESULT ====
「RT」2201回
「https」1455回
「シュー」307回
「マイ」302回
「亜」104回
「M」62回
「VRChat」60回
「ミーシェ」48回
「C」39回
「Twitter」38回
```
https://twitter.com/kemoshumai/status/1734215398214807615

## Requirement
1. Twitterデータ(全ツイート履歴)
   https://help.twitter.com/ja/managing-your-account/how-to-download-your-x-archive

1. ipadic-mecab-2_7_0
   https://github.com/daac-tools/vibrato


## Usage

ハードコードされているファイルパスを確認し、指定の箇所にTwitterデータとipadic-mecab-2_7_0を置いてから`cargo run`する。

ただし、tweets.jsonは、全ツイート履歴でダウンロードできるzipファイル内のdataフォルダにあるtweets.jsから、はじめの「window.YTD.tweets.part0 = 」を削除したものであり、このファイルは自分で作成する必要がある。

## Licence

MIT License

## Author

[Kemoshumai](https://github.com/kemoshumai)

