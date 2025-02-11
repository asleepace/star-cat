# Start Cat 😺🚀

A simple scrolling game written in [Rust](https://www.rust-lang.org/tools/install)!

| Example Gameplay |
|------------------|
|<video alt="Gameplay" src="https://github.com/user-attachments/assets/e39c3dd3-8532-4943-9956-700fbe3f5ad4" />|

## Gameplay Controls

- ↔️ Use arrow keys to move
- ⭐️ Collect stars to accelorate forward
- ☄️ Surf, grind and dodge asteroids
- 🏆 Reach top to win!

## Quick Start 

```bash
# Step 1: Install Rust
# https://www.rust-lang.org/tools/install

# Step 2: Cone the project
git clone https://github.com/asleepace/star-cat.git

# Step 3: Build, run and play!
cargo run
```

## Build WASM Binary (WiP)

Start a local http server with the following command:

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
# build for web
wasm-pack build --target web

# navigate to pkg directory for web project
cd pkg

# install serve
npm install serve --save-dev
npm start

# run the game
serve .
```

## Resources

- [Rust Canvas](https://rustwasm.github.io/wasm-bindgen/examples/2d-canvas.html)
- [Rust + WASM Hello, world!](https://rustwasm.github.io/docs/book/game-of-life/hello-world.html)
- [Comiling Rust to WASM](https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm)

## Troubleshooting

- [WASM Bindgen Discussions "env"](https://github.com/rustwasm/wasm-bindgen/discussions/3500)