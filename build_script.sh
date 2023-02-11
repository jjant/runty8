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

output_dir="generated"
rm -rf $output_dir/*
cargo build --target wasm32-unknown-unknown -p "examples" --bin "$package" $actual_mode
wasm-bindgen target/wasm32-unknown-unknown/$mode/$package.wasm --out-dir $output_dir --target web

cp index.html $output_dir/index.html
placeholder="__PACKAGE_NAME__"
echo "Replacing placeholder: $placeholder -> $package"
sed -i.bkp "s/$placeholder/$package/g" $output_dir/index.html

cd $output_dir
echo "Assets placed in ./$output_dir"
serve
