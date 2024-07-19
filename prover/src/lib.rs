/*todo this project uses "box patterns" features available in nightly version of rust
 * migrate to "deref patterns" feature when it will become stable then migrate to a stable version of rust */
#![feature(box_patterns)]

use std::any;
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

pub fn get_type_name<T>(any : &T) -> String
{
    return any::type_name::<T>().to_string();
}
