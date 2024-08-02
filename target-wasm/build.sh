#!/bin/bash
rm -rf ./pkg
mkdir ./pkg

wasm-pack build --release --target web

cp ./pkg/target_wasm.js ./src
cp ./pkg/target_wasm_bg.wasm ./src

rm archive.zip
zip -r -j archive.zip ./src
zip -d archive.zip lib.rs