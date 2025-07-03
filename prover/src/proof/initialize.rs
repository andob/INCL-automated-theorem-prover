use crate::graph::Graph;
use crate::problem::Problem;
use crate::proof::decomposition_queue::DecompositionPriorityQueue;
use crate::proof::ProofAlgorithm;
use crate::tree::node::ProofTreeNode;
use crate::tree::node_factory::ProofTreeNodeFactory;
use crate::tree::ProofTree;
use crate::tree::subtree::ProofSubtree;

impl ProofAlgorithm
{
    pub fn initialize(problem : Problem) -> ProofAlgorithm
    {
        let logic = problem.logic.clone();
        let mut problem_flags = problem.flags.clone();
        problem_flags.non_rigid_designators = problem.find_all_non_rigid_designators();

        let mut node_factory = ProofTreeNodeFactory::new(&logic);

        let non_conclusion = logic.get_semantics().reductio_ad_absurdum(&problem.conclusion);
        let non_conclusion_node = node_factory.new_node(non_conclusion);

        let mut decomposition_queue = DecompositionPriorityQueue::new(logic.clone());
        decomposition_queue.push_tree_node(Box::new(non_conclusion_node.clone()));

        if problem.premises.is_empty()
        {
            let proof_tree = ProofTree::new(problem, node_factory.clone(), non_conclusion_node);

            return ProofAlgorithm
            {
                proof_tree: proof_tree, decomposition_queue: decomposition_queue,
                logic_name: logic.get_name(), logic_rules: logic.get_rules(),
                node_factory: node_factory, modality_graph: Graph::new(),
                problem_flags: problem_flags,
            };
        }

        let first_premise_node = node_factory.new_node(problem.premises[0].clone());
        let first_premise_node_id = first_premise_node.id;

        let other_premises_nodes = problem.premises.iter().enumerate()
            .filter(|(index, _premise)| *index>0)
            .map(|(_index, premise)| node_factory.new_node(premise.clone()))
            .collect::<Vec<ProofTreeNode>>();

        let mut other_premise_subtree = ProofSubtree::with_middle_vertical_nodes(other_premises_nodes);

        let mut proof_tree = ProofTree::new(problem, node_factory.clone(), first_premise_node.clone());
        proof_tree.append_subtree(&mut other_premise_subtree, first_premise_node_id);

        decomposition_queue.push_subtree(Box::new(other_premise_subtree));
        decomposition_queue.push_tree_node(Box::new(first_premise_node));

        let mut non_conclusion_subtree = ProofSubtree::with_middle_node(non_conclusion_node);
        proof_tree.append_subtree(&mut non_conclusion_subtree, first_premise_node_id);

        return ProofAlgorithm
        {
            proof_tree: proof_tree, decomposition_queue: decomposition_queue,
            logic_name: logic.get_name(), logic_rules: logic.get_rules(),
            node_factory: node_factory, modality_graph: Graph::new(),
            problem_flags: problem_flags,
        };
    }
}
