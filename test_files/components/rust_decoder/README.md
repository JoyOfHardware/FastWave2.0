How to create and build the Rust component:

1. `cargo install cargo-component`
2. `cargo component new rust_decoder --lib`
3. `cd rust_decoder`
4. Update code as needed
5. `cargo component build --release --target wasm32-unknown-unknown && cp ../../../target/wasm32-unknown-unknown/release/rust_decoder.wasm .`
