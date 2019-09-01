# Rusthon

Rust でマラソンに参加する際のテンプレート

## 使い方
問題名を `a` とします。

これをクローンして、問題用のディレクトリに移動します。
```
git clone git@github.com:koba-e964/rusthon.git a
cd a
```

提出用のファイル (`src/solver.rs`) と、テスト用のファイル (`src/main.rs`) を編集します。

編集が終わったら、
```
cargo run --release
```
を実行するとテストが行われます。