/*todo this project uses "box patterns" features available in rust nightly
 * migrate to "deref patterns" feature when it will become stable
 * then migrate to a non-nightly version of rust */
#![feature(box_patterns)]

use std::fmt::Display;
use anyhow::{Context, Result};
use itertools::Itertools;
use serde_json::to_string;
use substring::Substring;

use crate::formula::Formula;
use crate::parser::algorithm::LogicalExpressionParser;
use crate::problem::catalog::get_demo_problem_catalog;
use crate::problem::Problem;
use crate::tree::node_factory::ProofTreeNodeFactory;
use crate::tree::ProofTree;
use crate::tree::subtree::ProofSubtree;

mod parser;
mod tree;
mod formula;
mod problem;
mod logic;
mod proof;

#[macro_export]
macro_rules! codeloc
{
    () => { format!("{}:{}", file!(), line!()) }
}

pub fn test() -> Result<()>
{
    let book_chapters = get_demo_problem_catalog().context(codeloc!())?;
    for book_chapter in &book_chapters
    {
        for problem_json in &book_chapter.problems
        {
            let problem = problem_json.to_problem().context(codeloc!())?;

            let proof_tree = problem.prove();
            println!("{}", proof_tree);
        }
    }

    return Ok(());
}
