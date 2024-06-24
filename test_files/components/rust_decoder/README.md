How to create and build the Rust component:

1. `cargo install cargo-component`
2. `cargo component new rust_decoder --lib`
3. `cd rust_decoder`
4. Update code as needed
5. `cargo +nightly component build --release --artifact-dir . -Z unstable-options`
