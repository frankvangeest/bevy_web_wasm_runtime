# Bevy Web WASM Runtime

A Bevy game engine project configured for WebAssembly.

## Prerequisites

- Rust (with `wasm32-unknown-unknown` target)
- wasm-bindgen-cli

Install the WASM target:
```bash
rustup target add wasm32-unknown-unknown
```

Install wasm-bindgen-cli:
```bash
cargo install wasm-bindgen-cli
```

## Building for WASM

1. Build the project:
```bash
cargo build --release --target wasm32-unknown-unknown
```

2. Generate WASM bindings:
```bash
wasm-bindgen --out-dir ./../web_server/public/lib --target web --no-typescript ./target/wasm32-unknown-unknown/release/app.wasm
```

3. Serve the files with a local web server:
```bash
# Run from the web_server folder
cargo run --release
```

4. Open your browser to `http://localhost:8080` (or the port shown)

## Development

Run locally (native):
```bash
cargo run
```
