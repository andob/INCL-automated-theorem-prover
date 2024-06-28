use std::fmt::Display;
use anyhow::{Context, Result};
use crate::logic::propositional_logic::PropositionalLogic;
use crate::parser::token_types::TokenTypeID;
use crate::semantics::Semantics;
use crate::tree::node::ProofTreeNode;
use crate::tree::node_factory::ProofTreeNodeFactory;
use crate::tree::subtree::ProofSubtree;

pub mod propositional_logic;

pub trait LogicRule
{
    fn apply(&self, factory : &mut ProofTreeNodeFactory, node : &ProofTreeNode) -> Option<ProofSubtree>;
}

pub trait Logic
{
    //the logic name, eg: PropositionalLogic
    fn get_name(&self) -> &str;

    //logic semantics, eg: true/false, true/false/unknown
    fn get_semantics(&self) -> Box<dyn Semantics>;

    //list of syntactic symbols, eg: not, and, or
    fn get_parser_syntax(&self) -> Vec<TokenTypeID>;

    //tree decomposition rules
    fn get_rules(&self) -> Vec<Box<dyn LogicRule>>;
}

pub struct LogicFactory {}
impl LogicFactory
{
    pub fn get_logic_by_name(name : &String) -> Result<Box<dyn Logic>>
    {
        return Self::get_logic_theories().into_iter()
            .find(|logic| logic.get_name() == name.as_str())
            .context(format!("Invalid logic with name {}!", name));
    }

    pub fn get_logic_theories() -> Vec<Box<dyn Logic>>
    {
        return vec!
        [
            Box::new(PropositionalLogic {}),
        ];
    }
}
