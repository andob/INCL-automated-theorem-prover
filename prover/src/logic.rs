use std::fmt::Display;

use crate::parser::token_types::TokenTypeID;
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
    fn get_name(&self) -> &str;

    fn get_parser_syntax(&self) -> Vec<TokenTypeID>;

    fn get_rules(&self) -> Vec<Box<dyn LogicRule>>;
}
