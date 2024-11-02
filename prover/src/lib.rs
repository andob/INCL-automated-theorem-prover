/* todo this project uses "box patterns" and "let chains" features available in nightly versions of rust
    migrate "box patterns" to "deref patterns" feature when it will become available in stable rust
    migrate to a stable version of rust after "let chain" and "deref patterns" both become stable */
#![feature(box_patterns)]
#![feature(let_chains)]

use std::fmt::Display;
use anyhow::{Context};
use itertools::Itertools;
use substring::Substring;

pub mod parser;
pub mod formula;
pub mod logic;
pub mod problem;
pub mod utils;
mod tree;
mod proof;
mod semantics;
mod graph;
mod countermodel;
