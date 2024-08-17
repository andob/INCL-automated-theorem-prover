use crate::graph::Graph;
use crate::logic::{LogicName, LogicRule};
use crate::logic::rule_apply_factory::RuleApplyFactory;
use crate::proof::decomposition_queue::DecompositionPriorityQueue;
use crate::tree::node::ProofTreeNode;
use crate::tree::node_factory::ProofTreeNodeFactory;
use crate::tree::ProofTree;
use crate::tree::subtree::ProofSubtree;

pub mod decomposition_queue;
mod initialize;

const MAX_NUMBER_OF_POSSIBLE_WORLDS_ON_MODAL_LOGIC : usize = 25;
const MAX_NUMBER_OF_TREE_NODES_ON_FIRST_ORDER_LOGIC : usize = 250;

pub struct ProofAlgorithm
{
    proof_tree : ProofTree,
    decomposition_queue : DecompositionPriorityQueue,
    logic_name : LogicName,
    logic_rules : Vec<Box<dyn LogicRule>>,
    node_factory : ProofTreeNodeFactory,
    modality_graph: Graph,
}

impl ProofAlgorithm
{
    pub fn prove(mut self) -> ProofTree
    {
        //check for contradictions right in premises and non-conclusion
        self.proof_tree.check_for_contradictions();

        while !self.decomposition_queue.is_empty() && !self.proof_tree.is_proof_correct && !self.reached_timeout()
        {
            if let Some((node, mut subtree)) = self.consume_next_queue_node()
            {
                self.proof_tree.append_subtree(&mut subtree, node.id);

                self.proof_tree.check_for_contradictions();

                self.decomposition_queue.push_subtree(subtree);
            }
        }

        self.proof_tree.has_timeout = self.reached_timeout();
        self.proof_tree.modality_graph = self.modality_graph;

        return self.proof_tree;
    }

    fn consume_next_queue_node(&mut self) -> Option<(Box<ProofTreeNode>, Box<ProofSubtree>)>
    {
        let mut factory = RuleApplyFactory
        {
            tree: &self.proof_tree,
            tree_node_factory: &mut self.node_factory,
            modality_graph: &mut self.modality_graph,
        };

        if let Some(node) = self.decomposition_queue.pop()
        {
            factory.set_spawner_node_id(Some(node.id));

            for logic_rule in &self.logic_rules
            {
                if let Some(subtree) = logic_rule.apply(&mut factory, &node)
                {
                    return Some((node, Box::new(subtree)));
                }
            }
        }

        return None;
    }

    fn reached_timeout(&self) -> bool
    {
        if !self.logic_name.is_modal_logic() && self.logic_name.is_first_order_logic()
        {
            return self.proof_tree.get_total_number_of_nodes() >= MAX_NUMBER_OF_TREE_NODES_ON_FIRST_ORDER_LOGIC;
        }

        return self.modality_graph.nodes.len() >= MAX_NUMBER_OF_POSSIBLE_WORLDS_ON_MODAL_LOGIC;
    }
}
