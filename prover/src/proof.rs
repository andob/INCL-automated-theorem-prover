use crate::formula::to_string::FormulaFormatOptions;
use crate::graph::Graph;
use crate::logic::{LogicName, LogicRuleCollection, LogicRuleResult};
use crate::logic::rule_apply_factory::RuleApplyFactory;
use crate::problem::ProblemFlags;
use crate::proof::decomposition_queue::DecompositionPriorityQueue;
use crate::proof::execution_log::{ExecutionLog, ExecutionLogHelperData};
use crate::tree::node::ProofTreeNode;
use crate::tree::node_factory::ProofTreeNodeFactory;
use crate::tree::ProofTree;
use crate::tree::subtree::ProofSubtree;
use crate::utils::measure_total_number_of_allocated_bytes;

pub mod decomposition_queue;
pub mod execution_log;
mod initialize;

const MAX_NUMBER_OF_POSSIBLE_WORLDS_ON_MODAL_LOGIC : usize = 25;
const MAX_NUMBER_OF_TREE_NODES_ON_FIRST_ORDER_LOGIC : usize = 250;
const MAX_NUMBER_OF_TREE_NODES_ON_INTUITIONISTIC_LOGIC : usize = 1000;

pub struct ProofAlgorithm
{
    proof_tree : ProofTree,
    decomposition_queue : DecompositionPriorityQueue,
    logic_name : LogicName,
    logic_rules : LogicRuleCollection,
    node_factory : ProofTreeNodeFactory,
    modality_graph : Graph,
    problem_flags : ProblemFlags,
}

impl ProofAlgorithm
{
    pub fn prove(mut self) -> ProofTree
    {
        if !self.problem_flags.should_skip_contradiction_check
        {
            //check for contradictions right in premises and non-conclusion
            self.proof_tree.check_for_contradictions();
        }

        let formula_format_options = FormulaFormatOptions::recommended_for(&self.proof_tree.problem.logic);

        while !self.decomposition_queue.is_empty() && !self.proof_tree.is_proof_correct && !self.reached_timeout()
        {
            let ram_consumption = measure_total_number_of_allocated_bytes(||
            {
                let (box node, mut result) = self.consume_next_queue_node().unwrap();

                //todo test this
                ExecutionLog::log(format!("Apply: <{}> {}\nResult: {}", node.id,
                    node.formula.to_string_with_options(&formula_format_options),
                    result.to_string_with_options(&formula_format_options)));


                self.proof_tree.append_logic_rule_result(&mut result, node.id);

                if !self.problem_flags.should_skip_contradiction_check
                {
                    self.proof_tree.check_for_contradictions();
                }

                self.decomposition_queue.push_logic_rule_result(result);
            });

            let log_helper_data = ExecutionLogHelperData::flush();
            ExecutionLog::log(format!("New nodes: {:?}\nNew vertices:\n{:?}", log_helper_data.new_graph_nodes, log_helper_data.new_graph_vertices));
            ExecutionLog::log(format!("New contradictions:\n{:?}", log_helper_data.new_contradictions));
            ExecutionLog::log(format!("{}B ({:.4}MB)", ram_consumption, ram_consumption/1024.0/1024.0));
        }

        self.proof_tree.has_timeout = self.reached_timeout();
        self.proof_tree.modality_graph = self.modality_graph;

        return self.proof_tree;
    }

    fn consume_next_queue_node(&mut self) -> Option<(Box<ProofTreeNode>, LogicRuleResult)>
    {
        let mut factory = RuleApplyFactory
        {
            tree: &self.proof_tree,
            tree_node_factory: &mut self.node_factory,
            modality_graph: &mut self.modality_graph,
            problem_flags: &self.problem_flags,
        };

        if let Some(node) = self.decomposition_queue.pop()
        {
            factory.set_spawner_node_id(Some(node.id));

            let result = self.logic_rules.apply(&mut factory, &node);
            if !result.is_empty()
            {
                return Some((node, result));
            }

            return Some((node, LogicRuleResult::Empty));
        }

        return None;
    }

    fn reached_timeout(&self) -> bool
    {
        let proof_tree_is_too_large =
            if self.logic_name.is_intuitionistic_logic()
                { self.proof_tree.get_total_number_of_nodes() >= MAX_NUMBER_OF_TREE_NODES_ON_INTUITIONISTIC_LOGIC }
            else if self.logic_name.is_first_order_logic()
                { self.proof_tree.get_total_number_of_nodes() >= MAX_NUMBER_OF_TREE_NODES_ON_FIRST_ORDER_LOGIC }
            else { false };

        let modality_graph_is_too_large =
            if self.logic_name.is_modal_logic()
                { self.modality_graph.nodes().len() >= MAX_NUMBER_OF_POSSIBLE_WORLDS_ON_MODAL_LOGIC }
            else { false };

        return proof_tree_is_too_large || modality_graph_is_too_large;
    }
}
