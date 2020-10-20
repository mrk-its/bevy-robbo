RUST_LOG=info cargo build --release --target wasm32-unknown-unknown --no-default-features --features bevy/bevy_webgl2
wasm-bindgen --no-typescript --out-name bevy-robbo --out-dir assets/wasm/target --target web ${CARGO_TARGET_DIR:-target}/wasm32-unknown-unknown/release/bevy-robbo.wasm
basic-http-server
