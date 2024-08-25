use std::cell::RefCell;
use std::rc::Rc;
use crate::formula::Formula;
use crate::logic::first_order_logic::{FirstOrderLogic, FirstOrderLogicDomainType};
use crate::logic::Logic;
use crate::tree::node::ProofTreeNode;

pub type ProofTreeNodeID = usize;

pub struct ProofTreeNodeIDSequence
{
    current_id : ProofTreeNodeID,
}

impl ProofTreeNodeIDSequence
{
    pub fn new() -> ProofTreeNodeIDSequence
    {
        return ProofTreeNodeIDSequence { current_id: 0 };
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
    pointer : Rc<RefCell<ProofTreeNodeFactoryImpl>>
}

pub struct ProofTreeNodeFactoryImpl
{
    pub domain_type : FirstOrderLogicDomainType,
    pub node_id_sequence : ProofTreeNodeIDSequence,
    pub spawner_node_id : Option<ProofTreeNodeID>,
}

impl ProofTreeNodeFactory
{
    pub fn new(logic : &Rc<dyn Logic>) -> ProofTreeNodeFactory
    {
        let domain_type = logic.cast_to::<FirstOrderLogic>()
            .map(|first_order_logic| first_order_logic.domain_type)
            .unwrap_or(FirstOrderLogicDomainType::ConstantDomain);

        return ProofTreeNodeFactory
        {
            pointer: Rc::new(RefCell::new(ProofTreeNodeFactoryImpl
            {
                domain_type: domain_type,
                node_id_sequence: ProofTreeNodeIDSequence::new(),
                spawner_node_id: None,
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

    pub fn set_spawner_node_id(&mut self, spawner_node_id_option : Option<ProofTreeNodeID>)
    {
        self.pointer.borrow_mut().spawner_node_id = spawner_node_id_option;
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
            left: None, middle: None, right: None,
            domain_type: self.domain_type,
            spawner_node_id: self.spawner_node_id,
            contrarian_node_id: None,
            is_contradictory: false,
        };
    }

    pub fn new_node_with_subnode(&mut self, formula : Formula, child : ProofTreeNode) -> ProofTreeNode
    {
        return ProofTreeNode
        {
            id: self.node_id_sequence.next(),
            formula: formula,
            left: None, right: None,
            middle: Some(Box::new(child)),
            domain_type: self.domain_type,
            spawner_node_id: self.spawner_node_id,
            contrarian_node_id: None,
            is_contradictory: false,
        };
    }
}
