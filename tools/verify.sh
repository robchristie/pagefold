#!/bin/sh
set -eu
export CARGO_HOME="$PWD/.runtime-scratch/cargo"
cargo fmt --check
python3 -m unittest discover -s tests -v
cargo test --locked
cargo clippy --locked --all-targets -- -D warnings
cargo build --locked
cargo build --locked --target wasm32-unknown-unknown --lib
wasm-bindgen --target web --out-dir web/pkg target/wasm32-unknown-unknown/debug/pagefold_probe.wasm
python3 tools/inventory.py evidence/implementation/fixtures-after.json evidence/calibration/fixtures-before.json
