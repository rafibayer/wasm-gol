# deps: 
# rustup target add wasm32-unknown-unknown
# cargo install basic-http-server
cargo build --target wasm32-unknown-unknown --release && \
mv ./target/wasm32-unknown-unknown/release/wasm-gol.wasm ./www/ || true && \
basic-http-server www



