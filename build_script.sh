#!/bin/bash

set -e

package="standalone-game"

rm -rf generated/*
cargo build --target wasm32-unknown-unknown -p "$package"
wasm-bindgen target/wasm32-unknown-unknown/debug/$package.wasm --out-dir generated --target web
cp index.html generated/index.html
cd generated
serve
