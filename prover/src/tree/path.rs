use std::fmt::{Display, Formatter};
use itertools::Itertools;
use crate::logic::Logic;
use crate::semantics::Semantics;
use crate::tree::node::ProofTreeNode;
use crate::tree::node_factory::ProofTreeNodeID;

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

    pub fn get_contradictory_node_ids(&self, logic : &Box<dyn Logic>) -> Vec<ProofTreeNodeID>
    {
        let mut contradictory_ids: Vec<ProofTreeNodeID> = vec![];
        for i in 0..self.nodes.len()
        {
            for j in 0..i
            {
                let semantics = logic.get_semantics();
                if semantics.are_formulas_contradictory(&self.nodes[i].formula, &self.nodes[j].formula)
                {
                    contradictory_ids.push(self.nodes[i].id);
                }
            }
        }

        return contradictory_ids;
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
