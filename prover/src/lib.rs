/* todo this project uses "box patterns" and "let chains" features available in nightly versions of rust
    migrate "box patterns" to "deref patterns" feature when it will become available in stable rust
    migrate to a stable version of rust after "let chain" and "deref patterns" both become stable */
#![feature(box_patterns)]
#![feature(let_chains)]

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
