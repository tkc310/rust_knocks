# Rustの素振り

## 環境構築

```
$ brew install asdf
$ asdf plugin add rust
$ asdf install rust
```

## 基礎

@refs: https://zenn.dev/mebiusbox/books/22d4c1ed9b0003

### Usage

```
$ cargo new base
$ cd base
$ cargo run
$ cargo test
```

## wasm

@refs  
- https://wasm-dev-book.netlify.app/hello-wasm.html
- https://qiita.com/mizchi/items/dc089c28e4d3afa78207

### Usage

```
# wasm用のコンパイルを有効化
$ rustup target add wasm32-unknown-unknown
$ cargo new --lib wasm-dev-book-hello-wasm
$ cd wasm-dev-book-hello-wasm
# wasmバイナリーのビルド
$ cargo build --target=wasm32-unknown-unknown --release
$ npx serve .
$ open http://localhost:3000
```

### 学び

- 暗黙の型変換
wasmのプリミティブ型はi32, i64, f32, f64の4つしかサポートしないため、rustから変換されたコードには型変換が生じる  
https://wasm-dev-book.netlify.app/hello-wasm.html#%E6%9A%97%E9%BB%99%E3%81%AE%E5%9E%8B%E5%A4%89%E6%8F%9B
- parcelなどのモジュールバンドラーから直接rustファイルを読み込んでHMRでwasmをコンパイルしてくれる仕組みがあるらしい  
(他のツールチェインにもありそう)
- wasm-bindgenを利用すれば文字列など他の型変換にも対応してくれる (ラッパーライブラリ)  
[wasm-bindgen](https://developer.mozilla.org/ja/docs/WebAssembly/Rust_to_wasm#wasm-bindgen_%E3%82%92%E4%BD%BF%E7%94%A8%E3%81%97%E3%81%A6_rust_%E3%81%A8_javascript_%E3%82%92%E5%8D%94%E8%AA%BF%E3%81%95%E3%81%9B%E3%82%8B)
- HMRが掛かってwasmの型も出力されるので思ったより生産性は高そうだった。でもrust&wasmビルド分は重くなる。
- 普通のWebサービス作ってたらwasmって使い所あるのかな..?  (service worker使うよりはデバッグしやすそう)

## swcプラグイン

AST周りのインターフェースをあまり触らなくてよさそうなサンプルで雰囲気だけ学習する。  
(swcにおけるASTのAPIを知ることは重要でない)

https://zenn.dev/nissy_dev/articles/create-swc-plugin
