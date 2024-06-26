use std::fmt::{Display, Formatter};
use itertools::Itertools;
use crate::tree::node::ProofTreeNode;

#[derive(Clone)]
pub struct ProofTreePath
{
    pub nodes : Vec<ProofTreeNode>
}

impl ProofTreePath
{
    pub fn new(nodes : Vec<ProofTreeNode>) -> ProofTreePath
    {
        return ProofTreePath { nodes };
    }

    pub fn contains(&self, node : &ProofTreeNode) -> bool
    {
        return self.nodes.contains(node);
    }

    pub fn plus(&self, node : &ProofTreeNode) -> ProofTreePath
    {
        let mut nodes = self.nodes.clone();
        nodes.push(node.clone());
        return Self::new(nodes);
    }

    pub fn append(&self, nodes : &Vec<ProofTreeNode>) -> ProofTreePath
    {
        let mut out_nodes = self.nodes.clone();
        for node in nodes { out_nodes.push(node.clone()); }
        return Self::new(out_nodes);
    }
}

impl Display for ProofTreePath
{
    fn fmt(&self, f : &mut Formatter<'_>) -> std::fmt::Result
    {
        let nodes_as_string = self.nodes.iter().map(|node| node.formula.to_string())
            .intersperse(String::from(" -> ")).collect::<String>();
        return write!(f, "{}", nodes_as_string);
    }
}
