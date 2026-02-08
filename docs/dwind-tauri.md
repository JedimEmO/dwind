# DWIND + Tauri Desktop Applications

Build native desktop applications with DWIND/Dominator frontend using Tauri. This guide covers setting up a Tauri 2.0 project with a Rust/WASM frontend.

## Prerequisites

```bash
# Install Tauri CLI
cargo install tauri-cli --version "^2.0" --locked

# Install Trunk WASM bundler
cargo install --locked trunk

# Add WASM target
rustup target add wasm32-unknown-unknown
```

## Project Structure

```
my-tauri-app/
├── src/                    # WASM frontend (dominator/dwind)
│   └── lib.rs
├── public/
│   └── index.html
├── src-tauri/              # Tauri backend
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   ├── build.rs
│   ├── icons/
│   └── src/
│       └── main.rs
├── Cargo.toml              # Frontend crate
├── Trunk.toml
└── README.md
```

## Configuration Files

### Frontend Cargo.toml

```toml
[package]
name = "my-tauri-frontend"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
dominator = "0.5"
dwind = "0.7"
dwind-macros = "0.4"
futures-signals = "0.3"
log = "0.4"
once_cell = "1.19"
wasm-bindgen = "0.2"
wasm-bindgen-futures = "0.4"
wasm-log = "0.3"
web-sys = "0.3"
```

### Trunk.toml

```toml
[build]
target = "public/index.html"

[watch]
ignore = ["./src-tauri"]

[serve]
port = 1420
ws_protocol = "ws"
```

### public/index.html

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <meta charset="utf-8">
    <link data-trunk rel="rust" href="../Cargo.toml">
    <title>My Tauri App</title>
</head>
<body></body>
</html>
```

### Backend Cargo.toml (src-tauri/Cargo.toml)

```toml
[package]
name = "my-tauri-backend"
version = "0.1.0"
edition = "2021"

[build-dependencies]
tauri-build = { version = "2", features = [] }

[dependencies]
tauri = { version = "2", features = [] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

### tauri.conf.json (src-tauri/tauri.conf.json)

```json
{
  "$schema": "https://schema.tauri.app/config/2",
  "productName": "my-app",
  "version": "0.1.0",
  "identifier": "com.example.my-app",
  "build": {
    "beforeDevCommand": "trunk serve --port 1420",
    "devUrl": "http://localhost:1420",
    "beforeBuildCommand": "trunk build",
    "frontendDist": "../dist"
  },
  "app": {
    "withGlobalTauri": true,
    "windows": [
      {
        "title": "My App",
        "width": 800,
        "height": 600,
        "resizable": true
      }
    ],
    "security": {
      "csp": null
    }
  },
  "bundle": {
    "active": true,
    "targets": "all",
    "icon": [
      "icons/32x32.png",
      "icons/128x128.png",
      "icons/icon.png"
    ]
  }
}
```

### build.rs (src-tauri/build.rs)

```rust
fn main() {
    tauri_build::build()
}
```

### main.rs (src-tauri/src/main.rs)

```rust
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

## Frontend Code

### src/lib.rs

```rust
#[macro_use]
extern crate dominator;

use dominator::{body, Dom};
use dwind::prelude::*;
use dwind_macros::dwclass;

#[wasm_bindgen::prelude::wasm_bindgen(start)]
async fn main() {
    wasm_log::init(Default::default());
    dominator::replace_dom(&body().parent_node().unwrap(), &body(), main_view());
}

fn main_view() -> Dom {
    dwind::stylesheet();

    html!("div", {
        .dwclass!("font-sans text-woodsmoke-50 bg-woodsmoke-950")
        .dwclass!("h-screen w-screen flex flex-col justify-center items-center")
        .child(html!("div", {
            .dwclass!("bg-woodsmoke-900 rounded-lg p-8 max-w-md")
            .dwclass!("border border-woodsmoke-700 border-solid shadow-lg")
            .child(html!("h1", {
                .dwclass!("text-2xl font-bold text-picton-blue-400 m-b-4")
                .text("Hello from DWIND + Tauri!")
            }))
            .child(html!("p", {
                .dwclass!("text-woodsmoke-300")
                .text("Your desktop app is running.")
            }))
        }))
    })
}
```

## Running the App

### Development

```bash
cd my-tauri-app
cargo tauri dev
```

This will:
1. Start Trunk dev server on port 1420
2. Build and launch the Tauri app
3. Enable hot-reload for frontend changes

### Production Build

```bash
cargo tauri build
```

Output binaries are placed in `src-tauri/target/release/bundle/`.

## Icons

Tauri requires RGBA PNG icons. Place them in `src-tauri/icons/`:
- `32x32.png` - Small icon
- `128x128.png` - Standard icon
- `128x128@2x.png` - Retina icon (256x256)
- `icon.png` - Base icon (512x512 recommended)

## Backend-Frontend Communication (Optional)

### Define Tauri Commands

```rust
// src-tauri/src/main.rs
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### Call from WASM Frontend

Use the `tauri-wasm` crate to call Tauri commands from Rust/WASM:

```toml
# Add to frontend Cargo.toml
tauri-wasm = { git = "https://github.com/p1mo/tauri-wasm" }
```

```rust
use tauri_wasm::api::core::invoke;
use serde_json::json;

async fn call_greet() {
    let result: String = invoke("greet", json!({"name": "World"})).await.unwrap();
    log::info!("Got: {}", result);
}
```

## Troubleshooting

### "icon is not RGBA"
Icons must be RGBA PNG format, not RGB. Regenerate with proper format.

### Trunk watch ignoring src-tauri
Add to `Trunk.toml`:
```toml
[watch]
ignore = ["./src-tauri"]
```

### Hot reload not working
Ensure `ws_protocol = "ws"` in `Trunk.toml` serve section.

### Missing WASM target
```bash
rustup target add wasm32-unknown-unknown
```

## Example Project

See `examples/tauri-app/` in the dwind repository for a complete working example.
