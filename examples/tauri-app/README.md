# DWIND + Tauri Example

A minimal Tauri desktop application using DWIND and Dominator for the frontend.

## Prerequisites

```bash
# Tauri CLI
cargo install tauri-cli --version "^2.0" --locked

# Trunk WASM bundler
cargo install --locked trunk

# WASM target
rustup target add wasm32-unknown-unknown
```

## Running

```bash
# Development mode with hot-reload
cargo tauri dev

# Production build
cargo tauri build
```

## Structure

```
tauri-app/
├── src/lib.rs              # WASM frontend (dominator/dwind)
├── public/index.html       # Entry HTML
├── Cargo.toml              # Frontend dependencies
├── Trunk.toml              # Trunk bundler config
└── src-tauri/              # Tauri backend
    ├── Cargo.toml
    ├── tauri.conf.json
    └── src/main.rs
```

## Documentation

See [docs/dwind-tauri.md](../../docs/dwind-tauri.md) for complete setup instructions.
