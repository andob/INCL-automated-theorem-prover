use itertools::Itertools;
use crate::proof::Problem;
use crate::tree::node::ProofTreeNode;
use crate::tree::path::ProofTreePath;

mod node;
mod path;
mod to_string;
mod to_json;
pub mod subtree;
pub mod node_factory;

pub struct ProofTree<'a>
{
    pub problem : &'a Problem,
    pub root_node : ProofTreeNode,
    pub is_proof_correct : bool,
    pub has_timeout : bool,
}

impl<'a> ProofTree<'a>
{
    pub fn new(problem : &'a Problem, root_node : ProofTreeNode) -> ProofTree<'a>
    {
        return ProofTree { problem, root_node, is_proof_correct:false, has_timeout: false };
    }

    //todo use this
    pub fn get_all_leafs(&self) -> Vec<ProofTreeNode>
    {
        return self.root_node.get_all_leafs();
    }

    //todo use this
    pub fn get_all_paths(&self) -> Vec<ProofTreePath>
    {
        return self.root_node.get_all_paths();
    }

    //todo use this
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

        return ProofTreePath::new(vec![self.root_node.clone()]);
    }
}
