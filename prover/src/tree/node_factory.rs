use crate::formula::Formula;
use crate::tree::node::ProofTreeNode;

pub type ProofTreeNodeID = usize;

struct ProofTreeNodeIDSequence
{
    current_id : ProofTreeNodeID,
    max_id : ProofTreeNodeID,
}

impl ProofTreeNodeIDSequence
{
    //todo use this
    pub fn new() -> ProofTreeNodeIDSequence
    {
        return ProofTreeNodeIDSequence { current_id: 0, max_id: 2000 };
    }

    //todo use this
    pub fn has_next(&self) -> bool
    {
        return self.current_id < self.max_id;
    }

    //todo use this
    pub fn next(&mut self) -> ProofTreeNodeID
    {
        let id = self.current_id;
        self.current_id += 1;
        return id;
    }
}

pub struct ProofTreeNodeFactory
{
    node_id_sequence : ProofTreeNodeIDSequence
}

impl ProofTreeNodeFactory
{
    //todo use this
    pub fn new() -> ProofTreeNodeFactory
    {
        return ProofTreeNodeFactory
        {
            node_id_sequence: ProofTreeNodeIDSequence::new()
        };
    }

    //todo use this
    pub fn new_node
    (
        &mut self, formula : Formula,
        left : Option<Box<ProofTreeNode>>,
        middle : Option<Box<ProofTreeNode>>,
        right : Option<Box<ProofTreeNode>>,
    ) -> ProofTreeNode
    {
        return ProofTreeNode
        {
            id: self.node_id_sequence.next(),
            formula: formula,
            left: left, middle: middle, right: right,
            is_contradictory: false,
        };
    }
}
