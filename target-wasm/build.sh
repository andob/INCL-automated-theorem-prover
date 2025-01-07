#!/bin/bash
set -e #fail on first error

function build_with_benchmark_mode_enabled
{
  [ -e ./pkg ] && rm -rf ./pkg
  mkdir ./pkg

  cargo clean
  RUSTFLAGS='--cfg is_benchmark_mode_enabled="true"' wasm-pack build --release --target web

  cp ./pkg/target_wasm.js ./src
  cp ./pkg/target_wasm_bg.wasm ./src

  [ -e ./src/benchmark ] && rm -rf ./src/benchmark
  [ -e ./benchmark ] && rm -rf ./benchmark
  cp -r ./src ./benchmark
  mv ./benchmark ./src
}

function build_with_benchmark_mode_disabled
{
  [ -e ./pkg ] && rm -rf ./pkg
  mkdir ./pkg

  cargo clean
  wasm-pack build --release --target web

  cp ./pkg/target_wasm.js ./src
  cp ./pkg/target_wasm_bg.wasm ./src
}

build_with_benchmark_mode_enabled
build_with_benchmark_mode_disabled

[ -e archive.zip ] && rm archive.zip
cd ./src
zip -r archive.zip .
zip -d archive.zip lib.rs
zip -d archive.zip benchmark/lib.rs
mv archive.zip ..
cd ..

[ -e ./src/benchmark ] && rm -rf ./src/benchmark
