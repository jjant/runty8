#!/bin/bash

set -e

if [[ -z $1 ]]; then
  package="standalone-game"
else
  package="$1"
fi

echo "Building: $package"

rm -rf generated/*
cargo build --target wasm32-unknown-unknown -p "examples" --bin "$package" --release
wasm-bindgen target/wasm32-unknown-unknown/release/$package.wasm --out-dir generated --target web

cp index.html generated/index.html
placeholder="__PACKAGE_NAME__"
echo "Replacing placeholder: $placeholder -> $package"
sed -i.bkp "s/$placeholder/$package/g" generated/index.html

cd generated
serve
