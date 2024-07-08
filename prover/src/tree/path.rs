use std::fmt::{Display, Formatter};
use std::rc::Rc;
use crate::formula::Formula;
use crate::logic::Logic;
use crate::semantics::Semantics;
use crate::tree::node::ProofTreeNode;
use crate::tree::node_factory::ProofTreeNodeID;

#[derive(Clone)]
pub struct ProofTreePath
{
    pub nodes : Vec<ProofTreePathNodeData>
}

#[derive(Clone)]
pub struct ProofTreePathNodeData
{
    pub id : ProofTreeNodeID,
    pub is_contradictory : bool,
    pub formula : Formula,
}

impl ProofTreePathNodeData
{
    fn from_node(node : &ProofTreeNode) -> ProofTreePathNodeData
    {
        return ProofTreePathNodeData
        {
            id: node.id,
            is_contradictory: node.is_contradictory,
            formula: node.formula.clone(),
        };
    }
}

impl ProofTreePath
{
    pub fn new(initial_node : &ProofTreeNode) -> ProofTreePath
    {
        let initial_node_data = ProofTreePathNodeData::from_node(initial_node);

        return ProofTreePath { nodes: vec![initial_node_data] }
    }

    pub fn contains(&self, node : &ProofTreeNode) -> bool
    {
        return self.contains_node_with_id(node.id);
    }

    pub fn contains_node_with_id(&self, node_id : ProofTreeNodeID) -> bool
    {
        return self.nodes.iter().any(|path_node| path_node.id == node_id);
    }

    pub fn plus(&self, node : &ProofTreeNode) -> ProofTreePath
    {
        let mut nodes = self.nodes.clone();
        nodes.push(ProofTreePathNodeData::from_node(node));
        return ProofTreePath { nodes };
    }

    pub fn append(&self, nodes : &Vec<ProofTreeNode>) -> ProofTreePath
    {
        let mut out_nodes = self.nodes.clone();
        nodes.iter().for_each(|node| out_nodes
            .push(ProofTreePathNodeData::from_node(node)));
        return ProofTreePath { nodes:out_nodes };
    }

    pub fn get_leaf_node_id(&self) -> ProofTreeNodeID
    {
        return self.nodes.last().unwrap().id;
    }

    pub fn get_contradictory_node_ids(&self, logic : &Rc<dyn Logic>) -> Vec<(ProofTreeNodeID, ProofTreeNodeID)>
    {
        let mut contradictory_ids : Vec<(ProofTreeNodeID, ProofTreeNodeID)> = vec![];

        for i in 0..self.nodes.len()
        {
            for j in 0..i
            {
                let semantics = logic.get_semantics();
                if semantics.are_formulas_contradictory(&self.nodes[i].formula, &self.nodes[j].formula)
                {
                    contradictory_ids.push((self.nodes[i].id, self.nodes[j].id));
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
        let last_index = self.nodes.len()-1;
        for (index, node) in self.nodes.iter().enumerate()
        {
            if index < last_index
            {
                write!(f, "{} -> ", node.formula).unwrap();
            }
            else
            {
                write!(f, "{}", node.formula).unwrap();
            }
        }

        return write!(f, "");
    }
}
