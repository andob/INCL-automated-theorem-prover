use std::rc::Rc;
use crate::formula::Formula;
use crate::graph::Graph;
use crate::logic::Logic;
use crate::tree::node::ProofTreeNode;
use crate::tree::node_factory::ProofTreeNodeFactory;
use crate::tree::ProofTree;

pub struct RuleApplyFactory<'a>
{
    pub tree : &'a ProofTree,
    pub tree_node_factory : &'a mut ProofTreeNodeFactory,
    pub modality_graph : &'a mut Graph,
}

impl <'a> RuleApplyFactory<'a>
{
    pub fn get_tree(&self) -> &'a ProofTree
    {
        return self.tree;
    }

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
}
