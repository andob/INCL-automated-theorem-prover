use std::cell::RefCell;
use std::rc::Rc;
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

#[derive(Clone)]
pub struct ProofTreeNodeFactory
{
    pointer: Rc<RefCell<ProofTreeNodeFactoryImpl>>
}

pub struct ProofTreeNodeFactoryImpl
{
    pub node_id_sequence : ProofTreeNodeIDSequence
}

impl ProofTreeNodeFactory
{
    pub fn new() -> ProofTreeNodeFactory
    {
        return ProofTreeNodeFactory
        {
            pointer: Rc::new(RefCell::new(ProofTreeNodeFactoryImpl
            {
                node_id_sequence: ProofTreeNodeIDSequence::new()
            }))
        };
    }
}

impl ProofTreeNodeFactory
{
    pub fn new_node_id(&mut self) -> ProofTreeNodeID
    {
        return self.pointer.borrow_mut().new_node_id();
    }

    pub fn new_node(&mut self, formula : Formula) -> ProofTreeNode
    {
        return self.pointer.borrow_mut().new_node(formula);
    }

    pub fn new_node_with_subnode(&mut self, formula : Formula, child : ProofTreeNode) -> ProofTreeNode
    {
        return self.pointer.borrow_mut().new_node_with_subnode(formula, child);
    }

    pub fn can_create_new_node(&self) -> bool
    {
        return self.pointer.borrow().node_id_sequence.has_next();
    }
}

impl ProofTreeNodeFactoryImpl
{
    pub fn new_node_id(&mut self) -> ProofTreeNodeID
    {
        return self.node_id_sequence.next();
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
