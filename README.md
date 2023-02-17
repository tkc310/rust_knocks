# Rustの素振り

## 環境構築

asdf経由だとcargoのパスが上手く通らなかったので直接インストールする。

```
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
$ echo 'export PATH="$PATH:$HOME/.cargo/bin"' >> ~/.zprofile
$ echo 'export PATH="$PATH:$HOME/.cargo/env"' >> ~/.zprofile
$ rustup install stable
$ rustup target add wasm32-wasi wasm32-unknown-unknown
```

vscodeにランゲージサーバの拡張機能 `rust-analyzer` をインストール。  
(これを入れないと静的解析されない、コードジャンプできないので必須)  
`Cargo.toml` が存在するディレクトリで有効になる

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
$ cd wasm-dev-book-hello-wasm
$ cargo new --lib crate

# wasm-packインストール
$ cargo install wasm-pack

# wasmバイナリーのビルド
$ npm run build
$ npx serve .
$ open http://localhost:3000
```

### ログ

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

AST周りのインターフェースをあまり触らなくてよさそうなサンプルで雰囲気だけ学習したい。  

@refs:
- [公式](https://swc.rs/docs/plugin/ecmascript/getting-started)
- https://zenn.dev/nissy_dev/articles/create-swc-plugin
- https://zenn.dev/sakamuuy/articles/implement-swc-plugin
- https://www.wantedly.com/companies/wantedly/post_articles/386347

### Usage

```
$ cargo install swc_cli
$ swc plugin new swc --target-type wasm32-wasi
$ npm run prepublishOnly
```

### ログ

- playgroundでASTを確認できる  
https://swc.rs/playground  
- ライブラリは軒並みメジャーリリースされておらず、参考記事のインポート元( `use` )もまばらだった  
- next.jsの[swcカスタムプラグイン機能](https://nextjs.org/docs/advanced-features/compiler#swc-plugins-experimental)もexperimentalなので、自作してメンテナンスするリスクは高そうだった (vercelが提供するプラグインのみ利用するのが無難そう)
