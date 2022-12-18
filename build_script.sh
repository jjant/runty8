#!/bin/bash

set -e

if [[ -z $1 ]]; then
  package="standalone-game"
else
  package="$1"
fi

mode="debug"

if [[ $2 == "--release" ]]; then
  mode="release"
fi

echo "Building: $package in $mode mode"

actual_mode="--release"
if [[ $mode == "debug" ]]; then
  actual_mode=""
fi

rm -rf generated/*
cargo build --target wasm32-unknown-unknown -p "examples" --bin "$package" $actual_mode
wasm-bindgen target/wasm32-unknown-unknown/$mode/$package.wasm --out-dir generated --target web

cp index.html generated/index.html
placeholder="__PACKAGE_NAME__"
echo "Replacing placeholder: $placeholder -> $package"
sed -i.bkp "s/$placeholder/$package/g" generated/index.html

cd generated
serve
