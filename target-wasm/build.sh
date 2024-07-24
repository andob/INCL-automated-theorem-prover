#!/bin/bash
rm -rf ./pkg
mkdir ./pkg
cp ./src/index.html ./pkg/index.html
wasm-pack build --release --target web