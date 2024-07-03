use crate::formula::FormulaExtras;
use crate::graph::Graph;
use crate::logic::LogicRule;
use crate::logic::rule_apply_factory::RuleApplyFactory;
use crate::problem::Problem;
use crate::proof::decomposition_queue::DecompositionPriorityQueue;
use crate::tree::node::ProofTreeNode;
use crate::tree::node_factory::ProofTreeNodeFactory;
use crate::tree::ProofTree;
use crate::tree::subtree::ProofSubtree;

pub mod decomposition_queue;

pub struct ProofAlgorithm
{
    proof_tree : ProofTree,
    decomposition_queue : DecompositionPriorityQueue,
    logic_rules : Vec<Box<dyn LogicRule>>,
    node_factory : ProofTreeNodeFactory,
    modality_graph: Graph,
}

impl ProofAlgorithm
{
    pub fn initialize(problem : Problem) -> ProofAlgorithm
    {
        let mut node_factory = ProofTreeNodeFactory::new();

        let non_conclusion = problem.logic.get_semantics().negate(&problem.conclusion, &FormulaExtras::empty());
        let non_conclusion_node = node_factory.new_node(non_conclusion);

        let mut decomposition_queue = DecompositionPriorityQueue::new();
        decomposition_queue.push_tree_node(Box::new(non_conclusion_node.clone()));

        if problem.premises.is_empty()
        {
            let logic_rules = problem.logic.get_rules();
            let proof_tree = ProofTree::new(problem, node_factory.clone(), non_conclusion_node);

            return ProofAlgorithm { proof_tree, decomposition_queue, logic_rules, node_factory, modality_graph:Graph::new() };
        }

        let logic_rules = problem.logic.get_rules();

        let first_premise_node = node_factory.new_node(problem.premises[0].clone());
        let first_premise_node_id = first_premise_node.id;

        let other_premise_nodes = problem.premises.iter().enumerate()
            .filter(|(index, _premise)| *index>0)
            .map(|(index, premise)| node_factory.new_node(premise.clone()))
            .collect::<Vec<ProofTreeNode>>();

        let mut other_premise_subtree = ProofSubtree::with_middle_vertical_nodes(other_premise_nodes);

        let mut proof_tree = ProofTree::new(problem, node_factory.clone(), first_premise_node.clone());
        proof_tree.append_subtree(&mut other_premise_subtree, first_premise_node_id);

        decomposition_queue.push_subtree(Box::new(other_premise_subtree));
        decomposition_queue.push_tree_node(Box::new(first_premise_node));

        let mut non_conclusion_subtree = ProofSubtree::with_middle_node(non_conclusion_node);
        proof_tree.append_subtree(&mut non_conclusion_subtree, first_premise_node_id);

        return ProofAlgorithm { proof_tree, decomposition_queue, logic_rules, node_factory, modality_graph:Graph::new() };
    }

    pub fn prove(mut self) -> ProofTree
    {
        while !self.decomposition_queue.is_empty() && self.node_factory.can_create_new_node()
        {
            if let Some((node, mut subtree)) = self.consume_next_queue_node()
            {
                self.proof_tree.append_subtree(&mut subtree, node.id);

                self.proof_tree.check_for_contradictions();

                self.decomposition_queue.push_subtree(subtree);
            }
        }

        self.proof_tree.has_timeout = !self.node_factory.can_create_new_node();
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
            for logic_rule in &self.logic_rules
            {
                factory.tree_node_factory.set_spawner_node_id(node.id);

                if let Some(subtree) = logic_rule.apply(&mut factory, &node)
                {
                    return Some((node, Box::new(subtree)));
                }
            }
        }

        return None;
    }
}
