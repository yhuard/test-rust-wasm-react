# From Rust to WebAssembly, in a React app

Based on this article: https://www.joshfinnie.com/blog/using-webassembly-created-in-rust-for-fast-react-components/

## Notes

https://www.rust-lang.org/learn/get-started

Install Rustup (a Rust installer and version management tool):

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

This automatically install Cargo as well.

To add dependencies (à la npm), we can use [cargo-edit](https://crates.io/crates/cargo-edit):

```sh
# Install cargo-edit
cargo install cargo-edit

# Add wasm-bindgen dependency
cargo add wasm-bindgen
```

Add `wasm32-unknown-unknown` Rust target:

```sh
rustup target add wasm32-unknown-unknown
```

Build Rust app with target:

```sh
cargo build --target wasm32-unknown-unknown
```

Install `wasm-bindgen-cli`:

```sh
cargo install -f wasm-bindgen-cli
```

Create a wrapping around the WebAssembly code for our React code:

```sh
wasm-bindgen target/wasm32-unknown-unknown/debug/test_rust_wasm_react.wasm --out-dir build
```

All-in-one command:

```sh
npm run build
```

Serve the dist:

```sh
npx serve dist
```

## Impressions

Seems really easy to get started. TypeScript types are also generated, which is pretty cool.

## Extra notes

Compiling secp256k1 on Mac M1:

```sh
brew install llvm
# M1
export CC=/opt/homebrew/opt/llvm/bin/clang
export AR=/opt/homebrew/opt/llvm/bin/llvm-ar
npm run build
npx serve dist
```

The goal is to run https://docs.rs/secp256k1/latest/secp256k1/ in the browser.

Installing the dependencies was REALLY painful. Getting it to work in the browser was not straightforward.

Or maybe I'm just lacking experience in Rust.
