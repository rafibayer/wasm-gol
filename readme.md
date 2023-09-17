# Wasm-gol
Game Of Life Implemented in Rust, targeting WASM.

# Dependencies
- Rust
  - WASM target (`rustup target add wasm32-unknown-unknown`)
- basic-http-server (`cargo install basic-http-server`)

# Try It!
https://rafibayer.github.io/2023/09/15/GameOfLife


# Run It Locally
Install dependencies, run `run.sh` in bash. This will compile to WASM and copy the `.wasm` file to the `/www` directory, then serve the static content via `basic-http-server`. Alternatively, compile your WASM and copy it to `/www` manually, and serve the files in this directory using an alternative static file server of your choice.
