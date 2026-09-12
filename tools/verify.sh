#!/bin/sh
set -eu
cd "$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)"
if [ "$(wasm-bindgen --version)" != "wasm-bindgen 0.2.127" ]; then
    echo "Install wasm-bindgen-cli 0.2.127: cargo install wasm-bindgen-cli --version 0.2.127 --locked" >&2
    exit 1
fi
cargo fmt --check
python3 -m unittest discover -s tests -v
cargo test --locked
cargo clippy --locked --all-targets -- -D warnings
cargo build --locked
cargo build --locked --target wasm32-unknown-unknown --lib
# Honour Cargo's configured target directory, including an external build cache.
target_directory=$(cargo metadata --locked --format-version 1 --no-deps | python3 -c 'import json,sys; print(json.load(sys.stdin)["target_directory"])')
wasm-bindgen --target web --out-dir web/pkg "$target_directory/wasm32-unknown-unknown/debug/pagefold_probe.wasm"
