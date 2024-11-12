use itertools::Itertools;
use rand::prelude::IteratorRandom;
use crate::graph::Graph;
use crate::problem::Problem;
use crate::tree::node::ProofTreeNode;
use crate::tree::node_factory::{ProofTreeNodeFactory, ProofTreeNodeID};
use crate::tree::path::ProofTreePath;

pub mod node;
pub mod path;
mod to_string;
mod to_json;
pub mod subtree;
pub mod node_factory;

pub struct ProofTree
{
    pub problem : Problem,
    pub root_node : ProofTreeNode,
    pub node_factory : ProofTreeNodeFactory,
    pub modality_graph : Graph,
    pub is_proof_correct : bool,
    pub has_timeout : bool,
    pub execution_log : String,
}

impl ProofTree
{
    pub fn new(problem : Problem, node_factory : ProofTreeNodeFactory, root_node : ProofTreeNode) -> ProofTree
    {
        return ProofTree
        {
            problem, root_node, node_factory,
            modality_graph: Graph::new(),
            is_proof_correct:false, has_timeout:false,
            execution_log: String::new(),
        }
    }

    pub fn get_all_paths(&self) -> Vec<ProofTreePath>
    {
        return self.root_node.get_all_paths();
    }

    pub fn get_path_that_goes_through_node(&self, node : &ProofTreeNode) -> ProofTreePath
    {
        let paths = self.get_paths_that_goes_through_node(node);
        return paths.into_iter().choose(&mut rand::thread_rng()).unwrap();
    }

    pub fn get_paths_that_goes_through_node(&self, node : &ProofTreeNode) -> Vec<ProofTreePath>
    {
        return self.get_all_paths().into_iter().filter(|path| path.contains(node)).collect();
    }

    pub fn get_node_with_id(&self, node_id : ProofTreeNodeID) -> Option<&ProofTreeNode>
    {
        return self.root_node.get_node_with_id(node_id);
    }

    pub fn get_total_number_of_nodes(&self) -> usize
    {
        return 1 + self.root_node.get_total_number_of_nodes();
    }

    pub fn check_for_contradictions(&mut self)
    {
        let mut number_of_contradictory_paths = 0usize;

        let paths = self.get_all_paths();
        for path in &paths
        {
            let contradictory_node_ids = path.get_contradictory_node_ids(&self.problem.logic);
            for (contradictory_node_id, contrarian_node_id) in &contradictory_node_ids
            {
                self.root_node.mark_child_node_as_contradictory(*contradictory_node_id, *contrarian_node_id);
            }

            if !contradictory_node_ids.is_empty()
            {
                number_of_contradictory_paths += 1;
            }
        }

        if number_of_contradictory_paths > 0
        {
            self.execution_log.push_str(format!("\n\nFound {} contradictions!", number_of_contradictory_paths).as_str());
        }

        if number_of_contradictory_paths == paths.len()
        {
            self.is_proof_correct = true;
        }
    }
}
