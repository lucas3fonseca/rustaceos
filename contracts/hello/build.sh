cargo +nightly build --release --target=wasm32-unknown-unknown
wasm-gc ./target/wasm32-unknown-unknown/release/hello.wasm hello_gc.wasm
wasm-opt hello_gc.wasm --output hello_gc_opt.wasm -Oz
