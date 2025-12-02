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
wasm-bindgen --out-dir . --target web --no-typescript .\target\wasm32-unknown-unknown\release\bevy_web_wasm_runtime.wasm
```

3. Serve the files with a local web server:
```bash
# Using Python
python -m http.server

# Or using a simple server like 'basic-http-server'
cargo install basic-http-server
basic-http-server .
```

4. Open your browser to `http://localhost:8000` (or the port shown)

## Development

Run locally (native):
```bash
cargo run
```
