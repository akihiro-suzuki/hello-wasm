https://developer.mozilla.org/ja/docs/WebAssembly/Rust_to_Wasm
の内容をとりあえず実践してみた

## wasm-pack
Rustにより生成されたWebAssemblyのビルド、テスト、パブリッシュのためのワンストップショップ (訳注: そこだけで全ての必要な買い物ができるような場所のこと)。

## wasm-bindgen
wasmモジュールとJavaScriptの間のやり取りをしやすくしてくれるRustのライブラリ、およびCLIツールのこと。

## wasm-pack buildについて
pkgディレクトリにnpmパッケージが生成される

```
wasm-pack build --target web
```
ここに該当の.jsが出力されるので、index.htmlはそのjsを取り込む
```javascript

import init, { greet, random } from "./hello_wasm.js";
init().then(() => {
    greet("WebAssembly");
    const v = random()
    greet(`${v}`)
});
        
```