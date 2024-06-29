//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
use anyhow::{Context, Result};
use wasm_bindgen::prelude::wasm_bindgen;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen]
extern "C"
{
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[wasm_bindgen_test]
fn pass()
{
    assert_eq!(1 + 1, 2);
}
