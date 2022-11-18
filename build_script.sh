#!/bin/bash

set -e

cargo build --target wasm32-unknown-unknown -p standalone-game
wasm-bindgen target/wasm32-unknown-unknown/debug/standalone-game.wasm --out-dir generated --target web
cp index.html generated/index.html
cd generated
serve
