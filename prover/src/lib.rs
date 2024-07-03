/*todo this project uses "box patterns" features available in rust nightly
 * migrate to "deref patterns" feature when it will become stable
 * then migrate to a non-nightly version of rust */
#![feature(box_patterns)]
#![feature(iter_intersperse)]

use std::fmt::Display;
use anyhow::{Context};
use itertools::Itertools;
use substring::Substring;

pub mod parser;
mod tree;
pub mod formula;
pub mod logic;
mod proof;
mod semantics;
pub mod problem;
mod graph;

#[macro_export]
macro_rules! codeloc
{
    () => { format!("{}:{}", file!(), line!()) }
}
