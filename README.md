# 😼 Star Cat 🚀

<img width="798" alt="Screen Shot 2022-07-23 at 2 01 16 PM" src="https://user-images.githubusercontent.com/10716803/180623339-25af6d67-2598-4c78-8ccd-8bec13266720.png">

A simple scrolling game written in Rust!

## Build

from https://github.com/not-fl3/macroquad/issues/5

- Clone https://github.com/not-fl3/macroquad-sampl`e-project
- run `cargo build --target wasm32-unknown-unknown
- copy wasm to www folder: `cp target/wasm32-unknown-unknown/debug/macroquad_sample_project.wasm www`
- serve www folder content with any wasm-mime compatible webserver, like `cd www && basic-http-server .`

To generate the wasm file run the following:

```bash
cargo build --target wasm32-unknown-unknown
cp target/wasm32-unknown-unknown/debug/star-cat.wasm www
```

To generate the javascript file run the following:

```bash
cat miniquad/native/sapp-wasm/js/gl.js >> mq_js_bundle.js
cat macroquad/js/audio.js >> mq_js_bundle.js
cat sapp-jsutils/js/sapp_jsutils.js >> mq_js_bundle.js
cat quad-net/js/quad-net.js >> mq_js_bundle.js
minify mq_js_bundle.js
```

To run in the browser:

```bash
cd www && basic-http-server .
```

## Running

```
cargo run
```

To run on wev start a local http server with the following command:

```bash
wasm-pack build --target web
cd www
npm install
npm run build
npm run start
```

now it should be running at http://localhost:8080!

## Building

Run the following command to create the WASM binary:

```bash
wasm-pack build --target web
cd www && npm run start
```

## Resources

- [Rust Canvas](https://rustwasm.github.io/wasm-bindgen/examples/2d-canvas.html)
- [Rust + WASM Hello, world!](https://rustwasm.github.io/docs/book/game-of-life/hello-world.html)
- [Comiling Rust to WASM](https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm)
