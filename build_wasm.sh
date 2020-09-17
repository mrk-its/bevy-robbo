RUST_LOG=info cargo build --target wasm32-unknown-unknown --release --no-default-features --features wasm
wasm-bindgen --out-dir wasm/target --target web ${CARGO_TARGET_DIR:-target}/wasm32-unknown-unknown/release/bevy-robbo.wasm
# basic-http-server wasm
