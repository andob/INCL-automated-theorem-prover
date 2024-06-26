use std::fmt::Display;

use anyhow::{Context, Result};
use itertools::Itertools;
use serde_json::to_string;
use substring::Substring;

use crate::formula::Formula;
use crate::parser::algorithm::LogicalExpressionParser;
use crate::proof::problem_catalog::get_demo_problem_catalog;
use crate::proof::Problem;
use crate::tree::node_factory::ProofTreeNodeFactory;
use crate::tree::ProofTree;
use crate::tree::subtree::ProofSubtree;

mod parser;
mod tree;
mod formula;
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
        for problem in &book_chapter.problems
        {
            for premise in &problem.premises
            {
                println!("{}", premise);
            }

            println!("{}", problem.conclusion);
        }
    }

    return Ok(());
}

pub fn test2() -> Result<()>
{
    let problem = Problem
    {
        premises: vec![],
        conclusion: Formula::Atomic("P".to_string(), vec![]),
    };

    let mut node_factory = ProofTreeNodeFactory::new();
    let n6 = Box::new(node_factory.new_node(Formula::Atomic("N6".to_string(), vec![]), None, None, None));
    let n5 = Box::new(node_factory.new_node(Formula::Atomic("N5".to_string(), vec![]), None, Some(n6), None));
    let n4 = Box::new(node_factory.new_node(Formula::Atomic("N4".to_string(), vec![]), None, None, None));
    let n3 = Box::new(node_factory.new_node(Formula::Atomic("N3".to_string(), vec![]), Some(n4), None, Some(n5)));
    let n2 = Box::new(node_factory.new_node(Formula::Atomic("N2".to_string(), vec![]), None, None, None));
    let n1 = Box::new(node_factory.new_node(Formula::Atomic("N1".to_string(), vec![]), Some(n2), None, Some(n3)));
    let mut tree = ProofTree::new(&problem, *n1);
    println!("\n\n{}", tree);

    println!("{}\n\n", tree.to_json()?);

    let n10 = Box::new(node_factory.new_node(Formula::Atomic("N10".to_string(), vec![]), None, None, None));
    let n20 = Box::new(node_factory.new_node(Formula::Atomic("N20".to_string(), vec![]), None, None, None));
    let subtree = ProofSubtree { left:Some(n10), middle:None, right:Some(n20) };
    tree.append_subtree(subtree, 5);

    println!("{}", tree);

    println!("{}\n\n", tree.to_json()?);

    return Ok(());
}
