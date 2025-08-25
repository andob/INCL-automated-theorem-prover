//todo this project uses "box patterns" nightly rust feature; migrate to "deref patterns" feature when it will become available in stable rust
#![feature(box_patterns)]

pub mod parser;
pub mod formula;
pub mod logic;
pub mod problem;
pub mod utils;
pub mod tree;
pub mod proof;
pub mod semantics;
pub mod graph;
pub mod countermodel;

