RUST_LOG=info cargo build --target wasm32-unknown-unknown --release --no-default-features
wasm-bindgen --out-name bevy-robbo-v3 --out-dir wasm/target --target web ${CARGO_TARGET_DIR:-target}/wasm32-unknown-unknown/release/bevy-robbo.wasm
# basic-http-server wasm
