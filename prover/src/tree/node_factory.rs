use crate::formula::Formula;
use crate::tree::node::ProofTreeNode;

pub type ProofTreeNodeID = usize;

pub struct ProofTreeNodeIDSequence
{
    current_id : ProofTreeNodeID,
    max_id : ProofTreeNodeID,
}

impl ProofTreeNodeIDSequence
{
    pub fn new() -> ProofTreeNodeIDSequence
    {
        return ProofTreeNodeIDSequence { current_id: 0, max_id: 2000 };
    }

    pub fn has_next(&self) -> bool
    {
        return self.current_id < self.max_id;
    }

    pub fn next(&mut self) -> ProofTreeNodeID
    {
        let id = self.current_id;
        self.current_id += 1;
        return id;
    }
}

pub struct ProofTreeNodeFactory
{
    pub node_id_sequence : ProofTreeNodeIDSequence
}

impl ProofTreeNodeFactory
{
    pub fn new() -> ProofTreeNodeFactory
    {
        return ProofTreeNodeFactory
        {
            node_id_sequence: ProofTreeNodeIDSequence::new()
        };
    }

    pub fn new_node(&mut self, formula : Formula) -> ProofTreeNode
    {
        return ProofTreeNode
        {
            id: self.node_id_sequence.next(),
            formula: formula,
            left:None, middle:None, right:None,
            is_contradictory: false,
        };
    }

    pub fn new_node_with_subnode(&mut self, formula : Formula, child : ProofTreeNode) -> ProofTreeNode
    {
        return ProofTreeNode
        {
            id: self.node_id_sequence.next(),
            formula: formula,
            left:None, right:None,
            middle: Some(Box::new(child)),
            is_contradictory: false,
        };
    }
}
