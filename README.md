# 概要
Chrome+JSとQuinnを使ってQuicTransportを動かす最低限のサンプル。

ChromeとRustでUDP的なパケットをやり取りしたいけどまず動くところまで持っていくのが大変という人向けのものです。

# 前準備
- [Chrome Origin Trials](https://developers.chrome.com/origintrials/#/trials/active)でQuicTransportを有効にする
  - 試験したいドメイン名で有効にすること
  - Active Tokensの値をHTML側に貼らないとQUIC Transportが有効にならないので注意
　- サンプルのものも自分で取得したTokenに差し替えてください
- index.htmlをどこか(試験したいドメイン名でアクセスできるところ)に置く
  - index.html単体で動くのでそれだけ持っていく
    - *.pemは外から見えるところに置かないようにしてください
- index.htmlのquic接続先を試験したいドメインに書き換える
- 証明書の用意
  - fullchain.pemとprivkey.pemを用意し、cargo run実行時のフォルダに配置する
- cargo buildでビルド

# 実行  
- cargo runで実行
- index.htmlにブラウザでアクセスする
- connectボタンを押すと、サーバーとクライアントでdatagramパケットのキャッチボールを行う
  - 10から始まってデクリメントされていき0になったらコネクションclose
  - 1秒sleep

# 参考
- [WebTransportとWebCodecsを組み合わせてビデオチャットを実装してみる](https://qiita.com/yuki_uchida/items/b177ec07ac0379950e58)
  - ほぼまるごと参考にさせて頂きました
- [Quinnのexamples](https://github.com/quinn-rs/quinn/blob/main/quinn/examples/connection.rs)
  - 最低限の処理なので、ここから動くところまで行くのはなかなか苦労した

# 言い訳
- Rustの正しい書き方が分かってません