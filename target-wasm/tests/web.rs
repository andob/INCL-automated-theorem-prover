//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;
use prover::codeloc;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen]
extern "C"
{
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[derive(Serialize, Deserialize)]
struct RootJSONNode
{
    chapters : Vec<BookChapterJSON>
}

#[derive(Serialize, Deserialize)]
struct BookChapterJSON
{
    name : String,
    problems : Vec<ProblemJSON>,
}

#[derive(Serialize, Deserialize)]
struct ProblemJSON
{
    id : String,
    logic : String,
    expected : String,
    premises : Vec<String>,
    conclusion : String,
}

#[wasm_bindgen_test]
fn pass()
{
    let raw_json = include_str!("../../problems.json");
    let parsed_json : RootJSONNode = serde_json::from_str(raw_json).context(codeloc!()).unwrap();

    for chapter in &parsed_json.chapters
    {
        for problem in &chapter.problems
        {
            for premise in &problem.premises
            {
                log(format!("{}", premise).as_str());
            }

            log(format!("{}", problem.conclusion).as_str());
        }
    }

    assert_eq!(1 + 1, 2);
}
