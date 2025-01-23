# DWIND webpage

## Running the webpage in local developer mode

You need a recent (>= 1.79.0) version of rust, please refer to https://rustup.rs/ for how to install this.

After installing those, add the `wasm32-unknown-unknown` target:

```shell
rustup target add wasm32-unknown-unknown
```

To run the application

### Trunk

First install the trunk utility: https://trunkrs.dev/guide/getting-started/installation.html

then do

```shell
trunk serve --open
```