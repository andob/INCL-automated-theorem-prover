use std::rc::Rc;
use crate::formula::Formula;
use crate::graph::Graph;
use crate::logic::common_modal_logic::NecessityReapplicationData;
use crate::logic::Logic;
use crate::tree::node::ProofTreeNode;
use crate::tree::node_factory::{ProofTreeNodeFactory, ProofTreeNodeID};
use crate::tree::ProofTree;

pub struct RuleApplyFactory<'a>
{
    pub tree : &'a ProofTree,
    pub tree_node_factory : &'a mut ProofTreeNodeFactory,
    pub modality_graph : &'a mut Graph,
}

impl <'a> RuleApplyFactory<'a>
{
    pub fn get_logic(&self) -> &Rc<dyn Logic>
    {
        return &self.tree.problem.logic;
    }

    pub fn new_node(&mut self, formula : Formula) -> ProofTreeNode
    {
        return self.tree_node_factory.new_node(formula);
    }

    pub fn new_node_with_subnode(&mut self, formula : Formula, child : ProofTreeNode) -> ProofTreeNode
    {
        return self.tree_node_factory.new_node_with_subnode(formula, child);
    }

    pub fn new_predicate_argument_instance_name(&mut self) -> String
    {
        return self.tree_node_factory.new_predicate_argument_instance_name();
    }

    pub fn pop_next_necessity_reapplication(&mut self) -> Option<NecessityReapplicationData>
    {
        return if let Some(reapplication) = self.modality_graph.necessity_reapplications.pop()
        { Some(reapplication) } else { None };
    }

    pub fn set_spawner_node_id(&mut self, spawner_node_id : ProofTreeNodeID)
    {
        if let Some(spawner_node) = self.tree.get_node_with_id(spawner_node_id)
        {
            if spawner_node.formula.is_hidden()
            {
                if let Some(spawner_spawner_node_id) = spawner_node.spawner_node_id
                {
                    self.tree_node_factory.set_spawner_node_id(spawner_spawner_node_id);
                    return;
                }
            }
        }

        self.tree_node_factory.set_spawner_node_id(spawner_node_id);
    }

    pub fn push_necessity_reapplication(&mut self, data : NecessityReapplicationData)
    {
        self.modality_graph.necessity_reapplications.push(data);
    }

    pub fn push_necessity_reapplications(&mut self, data : Vec<NecessityReapplicationData>)
    {
        for data_item in data
        {
            self.modality_graph.necessity_reapplications.push(data_item);
        }
    }
}
