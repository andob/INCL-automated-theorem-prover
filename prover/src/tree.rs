use itertools::Itertools;
use crate::problem::Problem;
use crate::tree::node::ProofTreeNode;
use crate::tree::node_factory::ProofTreeNodeFactory;
use crate::tree::path::ProofTreePath;

pub mod node;
mod path;
mod to_string;
mod to_json;
pub mod subtree;
pub mod node_factory;

pub struct ProofTree
{
    pub problem : Problem,
    pub root_node : ProofTreeNode,
    pub node_factory : ProofTreeNodeFactory,
    pub is_proof_correct : bool,
    pub has_timeout : bool,
}

impl ProofTree
{
    pub fn new(problem : Problem, node_factory : ProofTreeNodeFactory, root_node : ProofTreeNode) -> ProofTree
    {
        return ProofTree { problem, root_node, node_factory, is_proof_correct:false, has_timeout: false };
    }

    pub fn get_all_leafs(&self) -> Vec<ProofTreeNode>
    {
        return self.root_node.get_all_leafs();
    }

    pub fn get_all_paths(&self) -> Vec<ProofTreePath>
    {
        return self.root_node.get_all_paths();
    }

    pub fn get_all_leafs_with_paths(&self) -> (Vec<ProofTreeNode>, Vec<ProofTreePath>)
    {
        return self.root_node.get_all_leafs_with_paths();
    }

    //todo use this
    pub fn get_path_from_root_to_leafs_through_node(&self, node : &ProofTreeNode) -> ProofTreePath
    {
        if let Some(found_path) = self.get_all_paths().iter().find(|path| path.contains(node))
        {
            return found_path.clone();
        }

        return ProofTreePath::new(&self.root_node);
    }

    pub fn check_for_contradictions(&mut self)
    {
        let mut number_of_contradictory_paths = 0usize;

        let paths = self.get_all_paths();
        for path in &paths
        {
            let contradictory_node_ids = path.get_contradictory_node_ids(&self.problem.logic);
            if !contradictory_node_ids.is_empty()
            {
                for contradictory_node_id in contradictory_node_ids
                {
                    self.root_node.mark_child_node_as_contradictory(contradictory_node_id);
                }

                number_of_contradictory_paths += 1;
            }
        }

        if number_of_contradictory_paths == paths.len()
        {
            self.is_proof_correct = true;
        }
    }
}
