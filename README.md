# 😼 Star Cat 🚀

<img width="798" alt="Screen Shot 2022-07-23 at 2 01 16 PM" src="https://user-images.githubusercontent.com/10716803/180623339-25af6d67-2598-4c78-8ccd-8bec13266720.png">

A simple scrolling game written in Rust!

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
